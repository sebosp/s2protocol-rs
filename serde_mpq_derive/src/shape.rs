//! What a field type looks like: the wrappers and the leaf.

use syn::{GenericArgument, Ident, PathArguments, Type};

pub enum Shape<'a> {
    Option(&'a Type),
    Vec(&'a Type),
    /// `()`, the NullType of the protocol.
    Unit,
    Bool,
    /// One of the integer primitives.
    Int(Ident),
    /// `serde_mpq::BitArray`.
    BitArray,
    /// Another type with the derives.
    Nested(&'a Type),
}

pub const INTS: [&str; 8] = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"];

/// The single generic argument of a `Name<T>` path.
fn generic_arg<'a>(ty: &'a Type, name: &str) -> Option<&'a Type> {
    let Type::Path(path) = ty else { return None };
    let segment = path.path.segments.last()?;
    if segment.ident != name {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    if args.args.len() != 1 {
        return None;
    }
    match &args.args[0] {
        GenericArgument::Type(inner) => Some(inner),
        _ => None,
    }
}

pub fn shape(ty: &Type) -> Shape<'_> {
    if let Some(inner) = generic_arg(ty, "Option") {
        return Shape::Option(inner);
    }
    if let Some(inner) = generic_arg(ty, "Vec") {
        return Shape::Vec(inner);
    }
    match ty {
        Type::Tuple(tuple) if tuple.elems.is_empty() => Shape::Unit,
        Type::Path(path) => {
            let Some(segment) = path.path.segments.last() else {
                return Shape::Nested(ty);
            };
            if segment.arguments.is_empty() {
                if segment.ident == "bool" {
                    return Shape::Bool;
                }
                if segment.ident == "BitArray" {
                    return Shape::BitArray;
                }
                if INTS.iter().any(|int| segment.ident == int) {
                    return Shape::Int(segment.ident.clone());
                }
            }
            Shape::Nested(ty)
        }
        _ => Shape::Nested(ty),
    }
}

pub fn is_u8(ty: &Type) -> bool {
    matches!(shape(ty), Shape::Int(ident) if ident == "u8")
}
