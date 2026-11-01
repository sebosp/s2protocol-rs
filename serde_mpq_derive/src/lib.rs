//! The derive macros of `serde_mpq`, see its documentation for the attributes.
//!
//! The derives generate, besides the trait impl, a `parse` method and a `parse_<field>` method
//! per field (`Deserialize`) with `tracing` spans, so that the reading of a replay can be
//! followed field by field at the `trace` level.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Fields, Ident, Result, Type, Visibility};

mod attrs;
mod bit_packed;
mod byte_aligned;
mod shape;

use attrs::Attrs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    BitPacked,
    ByteAligned,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Deserialize,
    Serialize,
}

/// Reads a type from the bit packed encoding, see `serde_mpq`.
#[proc_macro_derive(MPQBitPackedDeserialize, attributes(mpq))]
pub fn derive_bit_packed_deserialize(input: TokenStream) -> TokenStream {
    expand(input, Mode::BitPacked, Dir::Deserialize)
}

/// Writes a type in the bit packed encoding, see `serde_mpq`.
#[proc_macro_derive(MPQBitPackedSerialize, attributes(mpq))]
pub fn derive_bit_packed_serialize(input: TokenStream) -> TokenStream {
    expand(input, Mode::BitPacked, Dir::Serialize)
}

/// Reads a type from the byte aligned encoding, see `serde_mpq`.
#[proc_macro_derive(MPQByteAlignedDeserialize, attributes(mpq))]
pub fn derive_byte_aligned_deserialize(input: TokenStream) -> TokenStream {
    expand(input, Mode::ByteAligned, Dir::Deserialize)
}

/// Writes a type in the byte aligned encoding, see `serde_mpq`.
#[proc_macro_derive(MPQByteAlignedSerialize, attributes(mpq))]
pub fn derive_byte_aligned_serialize(input: TokenStream) -> TokenStream {
    expand(input, Mode::ByteAligned, Dir::Serialize)
}

fn expand(input: TokenStream, mode: Mode, dir: Dir) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    match Container::parse(&input).and_then(|container| container.expand(mode, dir)) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// A field of a struct, or the payload of a variant.
struct Field {
    ident: Ident,
    ty: Type,
    attrs: Attrs,
}

struct Variant {
    ident: Ident,
    tag: i64,
    /// The payload of a tuple variant with one field.
    payload: Option<Type>,
}

#[allow(clippy::large_enum_variant)]
enum Body {
    Struct(Vec<Field>),
    /// A struct with a single `value` field read without the struct framing.
    Transparent(Field),
    Enum(Vec<Variant>),
}

struct Container {
    ident: Ident,
    vis: Visibility,
    attrs: Attrs,
    body: Body,
}

impl Container {
    fn parse(input: &DeriveInput) -> Result<Self> {
        if !input.generics.params.is_empty() {
            return Err(Error::new(
                input.generics.span(),
                "the mpq derives do not support generic types",
            ));
        }
        let attrs = Attrs::parse(&input.attrs)?;
        let body = match &input.data {
            Data::Struct(data) => {
                let fields = match &data.fields {
                    Fields::Named(named) => named
                        .named
                        .iter()
                        .map(|field| {
                            Ok(Field {
                                ident: field.ident.clone().expect("named"),
                                ty: field.ty.clone(),
                                attrs: Attrs::parse(&field.attrs)?,
                            })
                        })
                        .collect::<Result<Vec<_>>>()?,
                    Fields::Unit => vec![],
                    Fields::Unnamed(unnamed) => {
                        return Err(Error::new(
                            unnamed.span(),
                            "the mpq derives need named fields, i.e. `pub value: i64`",
                        ));
                    }
                };
                if attrs.transparent {
                    let mut fields = fields;
                    if fields.len() != 1 {
                        return Err(Error::new(
                            input.ident.span(),
                            "a transparent struct has exactly one field",
                        ));
                    }
                    Body::Transparent(fields.remove(0))
                } else {
                    Body::Struct(fields)
                }
            }
            Data::Enum(data) => {
                if !attrs.choice && !attrs.int_enum {
                    return Err(Error::new(
                        input.ident.span(),
                        "an enum needs #[mpq(choice, ...)] or #[mpq(int_enum, ...)]",
                    ));
                }
                let variants = data
                    .variants
                    .iter()
                    .map(|variant| {
                        let variant_attrs = Attrs::parse(&variant.attrs)?;
                        let tag = variant_attrs.tag.ok_or_else(|| {
                            Error::new(variant.span(), "every variant needs #[mpq(tag = n)]")
                        })?;
                        let payload = match &variant.fields {
                            Fields::Unit => None,
                            Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => {
                                Some(unnamed.unnamed[0].ty.clone())
                            }
                            other => {
                                return Err(Error::new(
                                    other.span(),
                                    "a variant is a unit or has one unnamed field",
                                ));
                            }
                        };
                        Ok(Variant {
                            ident: variant.ident.clone(),
                            tag,
                            payload,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                Body::Enum(variants)
            }
            Data::Union(data) => {
                return Err(Error::new(
                    data.union_token.span(),
                    "unions are not supported",
                ));
            }
        };
        Ok(Container {
            ident: input.ident.clone(),
            vis: input.vis.clone(),
            attrs,
            body,
        })
    }

    fn expand(&self, mode: Mode, dir: Dir) -> Result<TokenStream2> {
        match (mode, dir) {
            (Mode::BitPacked, Dir::Deserialize) => bit_packed::deserialize(self),
            (Mode::BitPacked, Dir::Serialize) => bit_packed::serialize(self),
            (Mode::ByteAligned, Dir::Deserialize) => byte_aligned::deserialize(self),
            (Mode::ByteAligned, Dir::Serialize) => byte_aligned::serialize(self),
        }
    }
}

/// The `#[tracing::instrument]` of a parse method.
fn instrument(name: Option<String>, peek: TokenStream2) -> TokenStream2 {
    match name {
        Some(name) => quote! {
            #[::tracing::instrument(name = #name, level = "trace", skip(input), fields(peek = #peek(input)))]
        },
        None => quote! {
            #[::tracing::instrument(level = "trace", skip(input), fields(peek = #peek(input)))]
        },
    }
}

fn parse_field_ident(field: &Ident) -> Ident {
    format_ident!("parse_{}", field)
}
