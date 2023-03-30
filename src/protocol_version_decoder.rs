//! Nom Parsing The S2 Protocol Version
//! This is stored in the MPQ UserData Section
//! and can be decoded with any Protocol Version
//! Decoder.
//! In the original code, in s2protocol from Blizzard, each protocol version
//! has its own version decoder. and whatever protocol is latest would decoded
//! the target file. In this verison we are not yet doing that but we should.

use super::*;
// TODO: Find a way to parse the version with the latest available protocol.
use crate::versions::protocol87702::byte_aligned::{ReplaySHeader, SVersion};
use nom::bytes::complete::*;
use nom::error::dbg_dmp;
use nom::*;
use nom_mpq::parser::peek_hex;
use nom_mpq::MPQ;

/// The S2 Protocol Header
#[derive(Debug, PartialEq, Clone)]
pub struct ProtocolHeader {
    pub m_signature: Vec<u8>,
    pub m_version: SVersion,
}

impl ProtocolHeader {
    #[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (tail, _) = dbg_dmp(tag(b"\x12"), "Protocol Header struct size")(tail)?;
        // This 0x12 translates to a 9 decimal, which is the number of fields
        // expected in the Protocol Version
        let (tail, m_signature) = Self::parse_m_signature(tail)?;
        let (tail, _) = dbg_dmp(tag(b"\x02"), "Protocol Header.Version struct Tag")(tail)?;
        let (tail, m_version) = SVersion::parse(tail)?;
        Ok((
            tail,
            Self {
                m_signature,
                m_version,
            },
        ))
    }

    #[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
    pub fn parse_m_signature(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (tail, _) = dbg_dmp(tag(b"\x00"), "m_signature field tag")(input)?;
        let (tail, m_signature) = tagged_blob(tail)?;
        Ok((tail, m_signature))
    }
}

/// Read the protocol header, this can be read with any protocol
pub fn read_protocol_header<'a>(mpq: &'a MPQ) -> IResult<&'a [u8], ReplaySHeader> {
    let user_data = mpq
        .user_data
        .as_ref()
        .expect("Unable to locate protocol version, missing user data in MPQ");
    ReplaySHeader::parse(&user_data.content)
}
