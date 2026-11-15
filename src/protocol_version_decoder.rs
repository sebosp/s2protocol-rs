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
use nom_mpq::MPQ;
use nom_mpq::parser::peek_hex;

/// The S2 Protocol Header
#[derive(Debug, PartialEq, Clone)]
pub struct ProtocolHeader {
    pub m_signature: Vec<u8>,
    pub m_version: SVersion,
}

impl ProtocolHeader {
    #[tracing::instrument(level = "debug", skip(input), fields(input = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (tail, _) = dbg_dmp(tag(&b"\x12"[..]), "Protocol Header struct size")(tail)?;
        // This 0x12 translates to a 9 decimal, which is the number of fields
        // expected in the Protocol Version
        let (tail, m_signature) = Self::parse_m_signature(tail)?;
        let (tail, _) = dbg_dmp(tag(&b"\x02"[..]), "Protocol Header.Version struct Tag")(tail)?;
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
    pub fn parse_m_signature(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
        let (tail, _) = dbg_dmp(tag(&b"\x00"[..]), "m_signature field tag")(input)?;
        let (tail, m_signature) = tagged_blob(tail)?;
        Ok((tail, m_signature))
    }
}

/// The signature of the protocol header of every replay.
pub const REPLAY_SIGNATURE: &[u8] = b"StarCraft II replay\x1b11";

/// Read the protocol header, this can be read with any protocol.
/// An MPQ without user data or with another signature is
/// [`S2ProtocolError::FileHeader`], so that one damaged file does not stop a directory scan.
pub fn read_protocol_header(mpq: &MPQ) -> S2ProtoResult<&[u8], ReplaySHeader> {
    let user_data = mpq.user_data.as_ref().ok_or_else(|| {
        S2ProtocolError::FileHeader("no user data in the MPQ, no protocol header".to_string())
    })?;
    let (tail, header) = ReplaySHeader::parse(&user_data.content)?;
    if header.m_signature != REPLAY_SIGNATURE {
        return Err(S2ProtocolError::FileHeader(format!(
            "signature {:?}",
            String::from_utf8_lossy(&header.m_signature)
        )));
    }
    Ok((tail, header))
}
