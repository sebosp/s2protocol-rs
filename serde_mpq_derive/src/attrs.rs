//! The `#[mpq(...)]` attributes.
//!
//! On a struct or enum:
//! - `transparent`: a struct with one `value` field read without the struct framing, i.e. the
//!   integer, string, array and bit array newtypes of the protocol.
//! - `choice, bits = n`: an enum whose variant is a `n` bits integer (bit packed) or a choice tag
//!   and a VLQ integer (byte aligned), followed by the payload of the variant.
//! - `int_enum, bits = n, offset = k`: an enum whose variant is an integer (bit packed `n` bits
//!   from `k`, byte aligned an int tag and a VLQ integer), with or without payloads.
//!
//! On a field or a variant: `tag = n`, the number of the field in a byte aligned struct, or of the
//! variant. On a field of the bit packed encoding: `bits = n, offset = k` for an `i64`,
//! `len_bits = n` for a `Vec<T>`, `string, len_bits = n` and `blob, bytes = n` for a `Vec<u8>`,
//! `bitarray, len_bits = n` for a `BitArray`, `fourcc` for the 4 bytes of a `Vec<u8>`. On a field
//! of the byte aligned encoding: `blob` for a `Vec<u8>`, `fourcc` for a `u32`, `default` for a
//! field that may be missing and takes its `Default` then.

use syn::spanned::Spanned;
use syn::{Attribute, Error, Expr, ExprLit, ExprUnary, Lit, Result, UnOp};

#[derive(Debug, Default, Clone)]
pub struct Attrs {
    pub transparent: bool,
    pub choice: bool,
    pub int_enum: bool,
    pub blob: bool,
    pub string: bool,
    pub bitarray: bool,
    pub fourcc: bool,
    pub default: bool,
    pub tag: Option<i64>,
    pub bits: Option<usize>,
    pub offset: Option<i64>,
    pub len_bits: Option<usize>,
    pub bytes: Option<usize>,
}

fn int_value(expr: &Expr) -> Result<i64> {
    // i64::MIN is written as the negation of a literal that does not fit an i64.
    fn wide(expr: &Expr) -> Result<i128> {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(lit), ..
            }) => lit.base10_parse(),
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_),
                expr,
                ..
            }) => Ok(-wide(expr)?),
            other => Err(Error::new(other.span(), "an integer literal is expected")),
        }
    }
    let value = wide(expr)?;
    i64::try_from(value).map_err(|_| Error::new(expr.span(), "the integer does not fit an i64"))
}

impl Attrs {
    pub fn parse(attrs: &[Attribute]) -> Result<Self> {
        let mut out = Attrs::default();
        for attr in attrs.iter().filter(|attr| attr.path().is_ident("mpq")) {
            attr.parse_nested_meta(|meta| {
                let flag = |flag: &mut bool| -> Result<()> {
                    *flag = true;
                    Ok(())
                };
                let usize_of = |meta: &syn::meta::ParseNestedMeta| -> Result<usize> {
                    let value = int_value(&meta.value()?.parse::<Expr>()?)?;
                    usize::try_from(value)
                        .map_err(|_| Error::new(meta.path.span(), "a positive integer is expected"))
                };
                if meta.path.is_ident("transparent") {
                    flag(&mut out.transparent)
                } else if meta.path.is_ident("choice") {
                    flag(&mut out.choice)
                } else if meta.path.is_ident("int_enum") {
                    flag(&mut out.int_enum)
                } else if meta.path.is_ident("blob") {
                    flag(&mut out.blob)
                } else if meta.path.is_ident("string") {
                    flag(&mut out.string)
                } else if meta.path.is_ident("bitarray") {
                    flag(&mut out.bitarray)
                } else if meta.path.is_ident("fourcc") {
                    flag(&mut out.fourcc)
                } else if meta.path.is_ident("default") {
                    flag(&mut out.default)
                } else if meta.path.is_ident("tag") {
                    out.tag = Some(int_value(&meta.value()?.parse::<Expr>()?)?);
                    Ok(())
                } else if meta.path.is_ident("offset") {
                    out.offset = Some(int_value(&meta.value()?.parse::<Expr>()?)?);
                    Ok(())
                } else if meta.path.is_ident("bits") {
                    out.bits = Some(usize_of(&meta)?);
                    Ok(())
                } else if meta.path.is_ident("len_bits") {
                    out.len_bits = Some(usize_of(&meta)?);
                    Ok(())
                } else if meta.path.is_ident("bytes") {
                    out.bytes = Some(usize_of(&meta)?);
                    Ok(())
                } else {
                    Err(meta.error(
                        "unknown mpq attribute, the known ones are transparent, choice, int_enum, \
                         blob, string, bitarray, fourcc, default, tag, bits, offset, len_bits and bytes",
                    ))
                }
            })?;
        }
        Ok(out)
    }

    /// The `len_bits` of an array, string or bit array.
    pub fn len_bits(&self, what: &str, span: proc_macro2::Span) -> Result<usize> {
        self.len_bits
            .ok_or_else(|| Error::new(span, format!("{what} needs #[mpq(len_bits = n)]")))
    }
}
