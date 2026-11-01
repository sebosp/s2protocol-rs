//! The code of the byte aligned (versioned) encoding.

use crate::attrs::Attrs;
use crate::shape::{Shape, is_u8, shape};
use crate::{Body, Container, Field, Variant, instrument, parse_field_ident};
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, Result, Type};

/// A block that reads `ty` from `src` and evaluates to `(tail, value)`.
fn read(ty: &Type, attrs: &Attrs, src: TokenStream) -> Result<TokenStream> {
    Ok(match shape(ty) {
        Shape::Option(inner) => {
            let inner = read(inner, attrs, quote!(tail))?;
            quote! {{
                let (tail, is_provided) = ::serde_mpq::tagged_opt_presence(#src)?;
                if is_provided {
                    let (tail, value) = #inner;
                    (tail, Some(value))
                } else {
                    (tail, None)
                }
            }}
        }
        Shape::Vec(inner) if attrs.blob && is_u8(inner) => {
            quote! { ::serde_mpq::tagged_blob(#src)? }
        }
        Shape::Vec(inner) => {
            let element = read(inner, attrs, quote!(tail))?;
            quote! {{
                let (tail, _) = ::serde_mpq::validate_array_tag(#src)?;
                let (mut tail, array_length) = ::serde_mpq::parse_vlq_int(tail)?;
                let array_length = array_length as usize;
                ::tracing::debug!("Reading array length: {array_length}");
                let max_initial_capacity =
                    ::serde_mpq::MAX_INITIAL_CAPACITY_BYTES / ::std::mem::size_of::<#inner>().max(1);
                let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
                for _ in 0..array_length {
                    let (new_tail, data) = #element;
                    tail = new_tail;
                    array.push(data);
                }
                (tail, array)
            }}
        }
        Shape::Int(ident) if attrs.fourcc => {
            if ident != "u32" {
                return Err(Error::new(ty.span(), "a fourcc is a u32"));
            }
            quote! { ::serde_mpq::tagged_fourcc(#src)? }
        }
        Shape::Int(ident) => quote! {{
            let (tail, value) = ::serde_mpq::tagged_vlq_int(#src)?;
            (tail, #ident::try_from(value)?)
        }},
        Shape::Bool => quote! { ::serde_mpq::tagged_bool(#src)? },
        Shape::Unit | Shape::BitArray => {
            return Err(Error::new(ty.span(), "not a byte aligned type"));
        }
        Shape::Nested(nested) => {
            quote! { <#nested as ::serde_mpq::ByteAlignedDeserialize>::deserialize_bytes(#src)? }
        }
    })
}

/// Statements that write `value`, a reference to a `ty`, into `out`.
fn write(ty: &Type, attrs: &Attrs, value: TokenStream) -> Result<TokenStream> {
    Ok(match shape(ty) {
        Shape::Option(inner) => {
            let inner = write(inner, attrs, quote!(value))?;
            quote! {
                match #value {
                    Some(value) => {
                        out.write_opt_presence(true);
                        #inner
                    }
                    None => out.write_opt_presence(false),
                }
            }
        }
        Shape::Vec(inner) if attrs.blob && is_u8(inner) => {
            quote! { out.write_tagged_blob(#value); }
        }
        Shape::Vec(inner) => {
            let element = write(inner, attrs, quote!(value))?;
            quote! {
                out.write_array_header(#value.len());
                for value in #value.iter() {
                    #element
                }
            }
        }
        Shape::Int(_) if attrs.fourcc => quote! { out.write_tagged_fourcc(*#value); },
        Shape::Int(_) => quote! { out.write_tagged_int(*#value as i64); },
        Shape::Bool => quote! { out.write_tagged_bool(*#value); },
        Shape::Unit | Shape::BitArray => {
            return Err(Error::new(ty.span(), "not a byte aligned type"));
        }
        Shape::Nested(nested) => {
            quote! { <#nested as ::serde_mpq::ByteAlignedSerialize>::serialize_bytes(#value, out)?; }
        }
    })
}

pub fn deserialize(container: &Container) -> Result<TokenStream> {
    let name = &container.ident;
    let vis = &container.vis;
    let peek = quote!(::serde_mpq::peek_hex);
    let main_instrument = instrument(Some(format!("{name}::parse")), peek.clone());
    let field_instrument = instrument(None, peek);
    let (methods, parse_body) = match &container.body {
        Body::Struct(fields) => {
            let mut methods = vec![];
            let mut decls = vec![];
            let mut arms = vec![];
            let mut inits = vec![];
            for Field { ident, ty, attrs } in fields {
                let method = parse_field_ident(ident);
                let field_name = ident.to_string();
                let tag = attrs.tag.ok_or_else(|| {
                    Error::new(ident.span(), "a byte aligned field needs #[mpq(tag = n)]")
                })?;
                let body = read(ty, attrs, quote!(input))?;
                methods.push(quote! {
                    #field_instrument
                    #vis fn #method(input: &[u8]) -> ::serde_mpq::ByteResult<'_, #ty> {
                        let (tail, value) = #body;
                        ::tracing::debug!("{}: {:?}", #field_name, value);
                        Ok((tail, value))
                    }
                });
                decls.push(quote! { let mut #ident: Option<#ty> = None; });
                arms.push(quote! {
                    #tag => {
                        ::tracing::debug!("Field [{i}]: tagged '{}' for field {}", #tag, #field_name);
                        if #ident.is_some() {
                            ::tracing::error!("Field {} with tag {} was already provided", #field_name, #tag);
                            return Err(::serde_mpq::Error::DuplicateTag(String::from(#field_name), #tag));
                        }
                        let (new_tail, value) = Self::#method(tail)?;
                        tail = new_tail;
                        #ident = Some(value);
                    }
                });
                let missing = if attrs.default {
                    quote! { ::std::default::Default::default() }
                } else if matches!(shape(ty), Shape::Option(_)) {
                    quote! { None }
                } else {
                    quote! { return Err(::serde_mpq::Error::MissingField(String::from(#field_name))) }
                };
                inits.push(quote! {
                    #ident: match #ident {
                        Some(value) => value,
                        None => #missing,
                    },
                });
            }
            let parse_body = quote! {
                let (tail, _) = ::serde_mpq::validate_struct_tag(input)?;
                let (mut tail, struct_field_count) = ::serde_mpq::parse_vlq_int(tail)?;
                #( #decls )*
                for i in 0..(struct_field_count as usize) {
                    let (new_tail, field_tag) = ::serde_mpq::parse_vlq_int(tail)?;
                    tail = new_tail;
                    match field_tag {
                        #( #arms )*
                        _ => {
                            ::tracing::error!("Unknown tag {field_tag}");
                            return Err(::serde_mpq::Error::UnknownTag(field_tag));
                        }
                    }
                }
                Ok((tail, Self { #( #inits )* }))
            };
            (methods, parse_body)
        }
        Body::Transparent(Field { ident, ty, attrs }) => {
            let body = read(ty, attrs, quote!(input))?;
            (
                vec![],
                quote! {
                    let (tail, value) = #body;
                    Ok((tail, Self { #ident: value }))
                },
            )
        }
        Body::Enum(variants) => {
            let header = if container.attrs.int_enum {
                quote! { let (tail, _) = ::serde_mpq::validate_int_tag(input)?; }
            } else {
                quote! { let (tail, _) = ::serde_mpq::validate_choice_tag(input)?; }
            };
            let arms = variants.iter().map(|Variant { ident, tag, payload }| {
                let variant_name = ident.to_string();
                Ok(match payload {
                    Some(ty) => {
                        let body = read(ty, &Attrs::default(), quote!(tail))?;
                        quote! {
                            #tag => {
                                ::tracing::debug!("Variant {} for value '{}'", #variant_name, #tag);
                                let (tail, value) = #body;
                                Ok((tail, Self::#ident(value)))
                            }
                        }
                    }
                    None => quote! {
                        #tag => {
                            ::tracing::debug!("Variant {} for value '{}'", #variant_name, #tag);
                            Ok((tail, Self::#ident))
                        }
                    },
                })
            });
            let arms = arms.collect::<Result<Vec<_>>>()?;
            (
                vec![],
                quote! {
                    #header
                    let (tail, variant_tag) = ::serde_mpq::parse_vlq_int(tail)?;
                    match variant_tag {
                        #( #arms )*
                        _ => {
                            ::tracing::debug!("Unknown variant for tag {variant_tag}");
                            Err(::serde_mpq::Error::UnknownTag(variant_tag))
                        }
                    }
                },
            )
        }
    };
    Ok(quote! {
        impl #name {
            #( #methods )*

            #main_instrument
            #vis fn parse(input: &[u8]) -> ::serde_mpq::ByteResult<'_, Self> {
                #parse_body
            }
        }

        impl ::serde_mpq::ByteAlignedDeserialize for #name {
            fn deserialize_bytes(input: &[u8]) -> ::serde_mpq::ByteResult<'_, Self> {
                Self::parse(input)
            }
        }
    })
}

pub fn serialize(container: &Container) -> Result<TokenStream> {
    let name = &container.ident;
    let body = match &container.body {
        Body::Struct(fields) => {
            let count = fields.len();
            let writes = fields.iter().map(|Field { ident, ty, attrs }| {
                let tag = attrs.tag.ok_or_else(|| {
                    Error::new(ident.span(), "a byte aligned field needs #[mpq(tag = n)]")
                })?;
                let stmts = write(ty, attrs, quote!(value))?;
                Ok(quote! {{
                    out.write_field_tag(#tag);
                    let value = &self.#ident;
                    #stmts
                }})
            });
            let writes = writes.collect::<Result<Vec<_>>>()?;
            quote! {
                out.write_struct_header(#count);
                #( #writes )*
            }
        }
        Body::Transparent(Field { ident, ty, attrs }) => {
            let stmts = write(ty, attrs, quote!(value))?;
            quote! {
                let value = &self.#ident;
                #stmts
            }
        }
        Body::Enum(variants) => {
            let header = if container.attrs.int_enum {
                quote! { out.write_tagged_int(tag); }
            } else {
                quote! { out.write_choice_header(tag); }
            };
            let arms = variants.iter().map(
                |Variant {
                     ident,
                     tag,
                     payload,
                 }| {
                    Ok(match payload {
                        Some(ty) => {
                            let stmts = write(ty, &Attrs::default(), quote!(value))?;
                            quote! {
                                Self::#ident(value) => {
                                    let tag: i64 = #tag;
                                    #header
                                    #stmts
                                }
                            }
                        }
                        None => quote! {
                            Self::#ident => {
                                let tag: i64 = #tag;
                                #header
                            }
                        },
                    })
                },
            );
            let arms = arms.collect::<Result<Vec<_>>>()?;
            quote! {
                match self {
                    #( #arms )*
                }
            }
        }
    };
    Ok(quote! {
        impl ::serde_mpq::ByteAlignedSerialize for #name {
            fn serialize_bytes(&self, out: &mut ::serde_mpq::ByteWriter) -> ::serde_mpq::Result<()> {
                #body
                Ok(())
            }
        }
    })
}
