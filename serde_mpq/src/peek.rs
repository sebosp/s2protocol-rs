//! The input around the current position, for the trace spans and the errors.

use colored::Colorize;
use nom::IResult;

/// The bytes shown by [`peek_hex`].
pub const PEEK_BYTES: usize = 8;

/// The next bytes of the input as hex, at most [`PEEK_BYTES`].
pub fn peek_hex(data: &[u8]) -> String {
    let chunk = &data[..PEEK_BYTES.min(data.len())];
    let hex: Vec<String> = chunk.iter().map(|byte| format!("{byte:02x}")).collect();
    format!("[{}]", hex.join(" "))
}

/// A colored binary representation of the current byte: the bits already read in blue, the
/// current position in green and the bits still to read in yellow, followed by the next bytes.
pub fn peek_bits(input: (&[u8], usize)) -> String {
    if input.0.is_empty() {
        return String::from("[]");
    }
    let input_str = format!("{:08b}", input.0[0]);
    let mut res = String::from("[0b");
    for (idx, bit_str) in input_str.chars().enumerate() {
        match idx.cmp(&input.1) {
            std::cmp::Ordering::Less => res.push_str(&format!("{bit_str}").blue()),
            std::cmp::Ordering::Equal => res.push_str(&format!(">{bit_str}<").green()),
            std::cmp::Ordering::Greater => res.push_str(&format!("{bit_str}").yellow()),
        };
    }
    res.push(']');
    res.push_str(&peek_hex(input.0));
    res
}

/// Logs the bits at a parser error, with the context given.
#[allow(clippy::type_complexity)]
pub fn dbg_peek_bits<'a, F, O, E: std::fmt::Debug>(
    mut f: F,
    context: &'static str,
) -> impl FnMut((&'a [u8], usize)) -> IResult<(&'a [u8], usize), O, E>
where
    F: FnMut((&'a [u8], usize)) -> IResult<(&'a [u8], usize), O, E>,
{
    move |i: (&'a [u8], usize)| match f(i) {
        Err(e) => {
            tracing::error!("{}: Error({:?}) at: {}", context, e, peek_bits(i));
            Err(e)
        }
        a => a,
    }
}

/// Logs the bytes at a parser error, with the context given.
pub fn dbg_peek_hex<'a, F, O, E: std::fmt::Debug>(
    mut f: F,
    context: &'static str,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>
where
    F: FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>,
{
    move |i: &'a [u8]| match f(i) {
        Err(e) => {
            tracing::error!("{}: Error({:?}) at: {}", context, e, peek_hex(i));
            Err(e)
        }
        a => a,
    }
}
