//! The code of the bit packed encoding.

use crate::attrs::Attrs;
use crate::shape::{Shape, is_u8, shape};
use crate::{Body, Container, Field, Variant, instrument, parse_field_ident};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Error, Result, Type};

/// A block that reads `ty` from `src` and evaluates to `(tail, value)`.
fn read(ty: &Type, attrs: &Attrs, src: TokenStream) -> Result<TokenStream> {
    Ok(match shape(ty) {
        Shape::Option(inner) => {
            let inner = read(inner, attrs, quote!(tail))?;
            quote! {{
                let (tail, is_provided) = ::serde_mpq::parse_bool(#src)?;
                if is_provided {
                    let (tail, value) = #inner;
                    (tail, Some(value))
                } else {
                    (tail, None)
                }
            }}
        }
        Shape::Vec(inner) if attrs.string => {
            if !is_u8(inner) {
                return Err(Error::new(ty.span(), "a string is a Vec<u8>"));
            }
            let len_bits = attrs.len_bits("a string", ty.span())?;
            quote! {{
                let (tail, str_size) = ::serde_mpq::parse_packed_int(#src, 0, #len_bits)?;
                let (tail, _) = ::serde_mpq::byte_align(tail)?;
                ::serde_mpq::take_bit_array(tail, str_size as usize * 8usize)?
            }}
        }
        Shape::Vec(inner) if attrs.blob => {
            if !is_u8(inner) {
                return Err(Error::new(ty.span(), "a blob is a Vec<u8>"));
            }
            let bytes = attrs
                .bytes
                .ok_or_else(|| Error::new(ty.span(), "a blob needs #[mpq(bytes = n)]"))?;
            quote! {{
                let (tail, _) = ::serde_mpq::byte_align(#src)?;
                ::serde_mpq::take_bit_array(tail, #bytes * 8usize)?
            }}
        }
        Shape::Vec(inner) if attrs.fourcc => {
            if !is_u8(inner) {
                return Err(Error::new(ty.span(), "a fourcc is a Vec<u8>"));
            }
            quote! { ::serde_mpq::take_fourcc(#src)? }
        }
        Shape::Vec(inner) => {
            let len_bits = attrs.len_bits("an array", ty.span())?;
            let element = if is_u8(inner) {
                quote! { ::serde_mpq::take_unaligned_byte(tail)? }
            } else {
                read(inner, &Attrs::default(), quote!(tail))?
            };
            quote! {{
                let (mut tail, array_length) = ::serde_mpq::parse_packed_int(#src, 0, #len_bits)?;
                let array_length = array_length as usize;
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
        Shape::BitArray => {
            if !attrs.bitarray {
                return Err(Error::new(
                    ty.span(),
                    "a BitArray needs #[mpq(bitarray, len_bits = n)]",
                ));
            }
            let len_bits = attrs.len_bits("a bit array", ty.span())?;
            quote! {{
                let (tail, bitarray_length) = ::serde_mpq::take_n_bits_into_i64(#src, #len_bits)?;
                let (tail, bits) = ::serde_mpq::take_bits_be(tail, bitarray_length as usize)?;
                (tail, ::serde_mpq::BitArray::new(bitarray_length as usize, bits))
            }}
        }
        Shape::Int(ident) => {
            if ident != "i64" {
                return Err(Error::new(
                    ty.span(),
                    "a bit packed integer is an i64 with #[mpq(bits = n, offset = k)]",
                ));
            }
            let bits = attrs.bits.ok_or_else(|| {
                Error::new(ty.span(), "an integer needs #[mpq(bits = n, offset = k)]")
            })?;
            let offset = attrs.offset.unwrap_or(0);
            quote! { ::serde_mpq::parse_packed_int(#src, #offset, #bits)? }
        }
        Shape::Bool => quote! { ::serde_mpq::parse_bool(#src)? },
        Shape::Unit => quote! { ::serde_mpq::take_null(#src)? },
        Shape::Nested(nested) => {
            quote! { <#nested as ::serde_mpq::BitPackedDeserialize>::deserialize_bits(#src)? }
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
                        out.write_bool(true)?;
                        #inner
                    }
                    None => out.write_bool(false)?,
                }
            }
        }
        Shape::Vec(_) if attrs.string => {
            let len_bits = attrs.len_bits("a string", ty.span())?;
            quote! {
                out.write_packed_int(#value.len() as i64, 0, #len_bits)?;
                out.byte_align();
                out.write_bit_array(#value, #value.len() * 8usize)?;
            }
        }
        Shape::Vec(_) if attrs.blob => {
            let bytes = attrs
                .bytes
                .ok_or_else(|| Error::new(ty.span(), "a blob needs #[mpq(bytes = n)]"))?;
            quote! {
                if #value.len() != #bytes {
                    return Err(::serde_mpq::Error::LengthMismatch {
                        field: "blob",
                        len: #value.len(),
                        expected: #bytes,
                    });
                }
                out.byte_align();
                out.write_bit_array(#value, #bytes * 8usize)?;
            }
        }
        Shape::Vec(_) if attrs.fourcc => quote! { out.write_fourcc(#value)?; },
        Shape::Vec(inner) => {
            let len_bits = attrs.len_bits("an array", ty.span())?;
            let element = if is_u8(inner) {
                quote! { out.write_unaligned_byte(*value)?; }
            } else {
                write(inner, &Attrs::default(), quote!(value))?
            };
            quote! {
                out.write_packed_int(#value.len() as i64, 0, #len_bits)?;
                for value in #value.iter() {
                    #element
                }
            }
        }
        Shape::BitArray => {
            let len_bits = attrs.len_bits("a bit array", ty.span())?;
            quote! {
                out.write_packed_int(#value.bits as i64, 0, #len_bits)?;
                out.write_bits_be(&#value.value, #value.bits)?;
            }
        }
        Shape::Int(_) => {
            let bits = attrs.bits.ok_or_else(|| {
                Error::new(ty.span(), "an integer needs #[mpq(bits = n, offset = k)]")
            })?;
            let offset = attrs.offset.unwrap_or(0);
            quote! { out.write_packed_int(*#value, #offset, #bits)?; }
        }
        Shape::Bool => quote! { out.write_bool(*#value)?; },
        Shape::Unit => quote! {},
        Shape::Nested(nested) => {
            quote! { <#nested as ::serde_mpq::BitPackedSerialize>::serialize_bits(#value, out)?; }
        }
    })
}

fn variant_tag(container: &Container) -> Result<(usize, i64)> {
    let bits = container.attrs.bits.ok_or_else(|| {
        Error::new(
            container.ident.span(),
            "a bit packed enum needs #[mpq(choice, bits = n)]",
        )
    })?;
    Ok((bits, container.attrs.offset.unwrap_or(0)))
}

pub fn deserialize(container: &Container) -> Result<TokenStream> {
    let name = &container.ident;
    let vis = &container.vis;
    let peek = quote!(::serde_mpq::peek_bits);
    let main_instrument = instrument(Some(format!("{name}::parse")), peek.clone());
    let field_instrument = instrument(None, peek);
    let (methods, parse_body) = match &container.body {
        Body::Struct(fields) => {
            let methods = fields.iter().map(|Field { ident, ty, attrs }| {
                let method = parse_field_ident(ident);
                let field_name = ident.to_string();
                let body = read(ty, attrs, quote!(input))?;
                Ok(quote! {
                    #field_instrument
                    #vis fn #method(input: ::serde_mpq::BitInput<'_>) -> ::serde_mpq::BitResult<'_, #ty> {
                        let (tail, value) = #body;
                        ::tracing::debug!("{}: {:?}", #field_name, value);
                        Ok((tail, value))
                    }
                })
            });
            let methods = methods.collect::<Result<Vec<_>>>()?;
            let idents: Vec<_> = fields.iter().map(|field| &field.ident).collect();
            let method_idents: Vec<_> = idents
                .iter()
                .map(|ident| parse_field_ident(ident))
                .collect();
            let parse_body = quote! {
                let tail = input;
                #( let (tail, #idents) = Self::#method_idents(tail)?; )*
                Ok((tail, Self { #( #idents ),* }))
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
            let (bits, offset) = variant_tag(container)?;
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
                    let (tail, variant_tag) = ::serde_mpq::parse_packed_int(input, #offset, #bits)?;
                    match variant_tag {
                        #( #arms )*
                        _ => {
                            ::tracing::debug!("Unknown variant value {variant_tag}");
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
            #vis fn parse(input: ::serde_mpq::BitInput<'_>) -> ::serde_mpq::BitResult<'_, Self> {
                #parse_body
            }
        }

        impl ::serde_mpq::BitPackedDeserialize for #name {
            fn deserialize_bits(input: ::serde_mpq::BitInput<'_>) -> ::serde_mpq::BitResult<'_, Self> {
                Self::parse(input)
            }
        }
    })
}

pub fn serialize(container: &Container) -> Result<TokenStream> {
    let name = &container.ident;
    let body = match &container.body {
        Body::Struct(fields) => {
            let writes = fields.iter().map(|Field { ident, ty, attrs }| {
                let value = quote!(&self.#ident);
                let stmts = write(ty, attrs, quote!(value))?;
                Ok(quote! {{
                    let value = #value;
                    #stmts
                }})
            });
            let writes = writes.collect::<Result<Vec<_>>>()?;
            quote! { #( #writes )* }
        }
        Body::Transparent(Field { ident, ty, attrs }) => {
            let stmts = write(ty, attrs, quote!(value))?;
            quote! {
                let value = &self.#ident;
                #stmts
            }
        }
        Body::Enum(variants) => {
            let (bits, offset) = variant_tag(container)?;
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
                                    out.write_packed_int(#tag, #offset, #bits)?;
                                    #stmts
                                }
                            }
                        }
                        None => quote! {
                            Self::#ident => {
                                out.write_packed_int(#tag, #offset, #bits)?;
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
    let _ = format_ident!("unused");
    Ok(quote! {
        impl ::serde_mpq::BitPackedSerialize for #name {
            fn serialize_bits(&self, out: &mut ::serde_mpq::BitWriter) -> ::serde_mpq::Result<()> {
                #body
                Ok(())
            }
        }
    })
}
