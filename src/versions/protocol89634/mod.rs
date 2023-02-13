//! Generated code from source: ../s2protocol/json/protocol89634.json
use crate::*;
use nom::*;
use nom_mpq::parser::peek_hex;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
pub enum SVarUint32 {
    Uint6(u8),
    Uint14(u32),
    Uint22(u32),
    Uint32(u32),
}
impl SVarUint32 {
    #[tracing::instrument(name="SVarUint32::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_choice_tag(input)?;
        let (tail, variant_tag) = parse_vlq_int(tail)?;
        match variant_tag {
            0 => {
                tracing::debug!("Variant tagged '0' for Uint6");
                let (tail, res) = tagged_vlq_int(tail)?;
                tracing::debug!("res: {:?}", res);
                Ok((tail, Self::Uint6(u8::try_from(res).unwrap())))
            }
            1 => {
                tracing::debug!("Variant tagged '1' for Uint14");
                let (tail, res) = tagged_vlq_int(tail)?;
                tracing::debug!("res: {:?}", res);
                Ok((tail, Self::Uint14(u32::try_from(res).unwrap())))
            }
            2 => {
                tracing::debug!("Variant tagged '2' for Uint22");
                let (tail, res) = tagged_vlq_int(tail)?;
                tracing::debug!("res: {:?}", res);
                Ok((tail, Self::Uint22(u32::try_from(res).unwrap())))
            }
            3 => {
                tracing::debug!("Variant tagged '3' for Uint32");
                let (tail, res) = tagged_vlq_int(tail)?;
                tracing::debug!("res: {:?}", res);
                Ok((tail, Self::Uint32(u32::try_from(res).unwrap())))
            }

            _ => {
                tracing::error!("Unknown variant for tag {variant_tag}");
                panic!("Unknown variant tag {variant_tag}");
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SVersion {
    pub m_flags: u8,
    pub m_major: u8,
    pub m_minor: u8,
    pub m_revision: u8,
    pub m_build: u32,
    pub m_base_build: u32,
}
impl SVersion {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_flags(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_flags) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_flags);
        Ok((tail, u8::try_from(m_flags).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_major(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_major) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_major);
        Ok((tail, u8::try_from(m_major).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_minor(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_minor) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_minor);
        Ok((tail, u8::try_from(m_minor).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_revision(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_revision) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_revision);
        Ok((tail, u8::try_from(m_revision).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_build(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_build) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_build);
        Ok((tail, u32::try_from(m_build).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_base_build(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_base_build) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_base_build);
        Ok((tail, u32::try_from(m_base_build).unwrap()))
    }
    #[tracing::instrument(name="SVersion::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_flags: Option<u8> = None;
        let mut m_major: Option<u8> = None;
        let mut m_minor: Option<u8> = None;
        let mut m_revision: Option<u8> = None;
        let mut m_build: Option<u32> = None;
        let mut m_base_build: Option<u32> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_flags");
                    if m_flags.is_none() {
                        let (new_tail, parsed_m_flags) = Self::parse_m_flags(tail)?;
                        tail = new_tail;
                        m_flags = Some(parsed_m_flags);
                        continue;
                    } else {
                        tracing::error!("Field m_flags with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_major");
                    if m_major.is_none() {
                        let (new_tail, parsed_m_major) = Self::parse_m_major(tail)?;
                        tail = new_tail;
                        m_major = Some(parsed_m_major);
                        continue;
                    } else {
                        tracing::error!("Field m_major with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!("Field [{i}]: tagged '2' for field m_minor");
                    if m_minor.is_none() {
                        let (new_tail, parsed_m_minor) = Self::parse_m_minor(tail)?;
                        tail = new_tail;
                        m_minor = Some(parsed_m_minor);
                        continue;
                    } else {
                        tracing::error!("Field m_minor with tag 2 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                3 => {
                    tracing::debug!("Field [{i}]: tagged '3' for field m_revision");
                    if m_revision.is_none() {
                        let (new_tail, parsed_m_revision) = Self::parse_m_revision(tail)?;
                        tail = new_tail;
                        m_revision = Some(parsed_m_revision);
                        continue;
                    } else {
                        tracing::error!("Field m_revision with tag 3 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                4 => {
                    tracing::debug!("Field [{i}]: tagged '4' for field m_build");
                    if m_build.is_none() {
                        let (new_tail, parsed_m_build) = Self::parse_m_build(tail)?;
                        tail = new_tail;
                        m_build = Some(parsed_m_build);
                        continue;
                    } else {
                        tracing::error!("Field m_build with tag 4 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                5 => {
                    tracing::debug!("Field [{i}]: tagged '5' for field m_base_build");
                    if m_base_build.is_none() {
                        let (new_tail, parsed_m_base_build) = Self::parse_m_base_build(tail)?;
                        tail = new_tail;
                        m_base_build = Some(parsed_m_base_build);
                        continue;
                    } else {
                        tracing::error!("Field m_base_build with tag 5 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_flags: m_flags.expect("Missing m_flags from struct"),
                m_major: m_major.expect("Missing m_major from struct"),
                m_minor: m_minor.expect("Missing m_minor from struct"),
                m_revision: m_revision.expect("Missing m_revision from struct"),
                m_build: m_build.expect("Missing m_build from struct"),
                m_base_build: m_base_build.expect("Missing m_base_build from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Smd5 {
    pub m_data_deprecated: Option<Vec<u8>>,
    pub m_data: Vec<u8>,
}
impl Smd5 {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_data_deprecated(input: &[u8]) -> IResult<&[u8], Option<Vec<u8>>> {
        let (tail, _) = validate_opt_tag(input)?;
        let (tail, is_provided) = nom::number::complete::u8(tail)?;
        let (tail, m_data_deprecated) = if is_provided != 0 {
            let (tail, _) = validate_array_tag(tail)?;
            let (tail, array_length) = parse_vlq_int(tail)?;
            tracing::debug!("Reading array length: {array_length}");
            let (tail, array) = nom::multi::count(tagged_vlq_int, array_length as usize)(tail)?;
            let array = array
                .iter()
                .map(|val| <_>::try_from(*val).unwrap())
                .collect();
            (tail, Some(array))
        } else {
            (tail, None)
        };
        tracing::debug!("res: {:?}", m_data_deprecated);
        Ok((tail, m_data_deprecated))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_data(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (tail, m_data) = tagged_blob(input)?;
        tracing::debug!("res: {:?}", m_data);
        Ok((tail, m_data))
    }
    #[tracing::instrument(name="Smd5::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_data_deprecated: Option<Option<Vec<u8>>> = Some(None);
        let mut m_data: Option<Vec<u8>> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_data_deprecated");
                    if let Some(None) = m_data_deprecated {
                        let (new_tail, parsed_m_data_deprecated) =
                            Self::parse_m_data_deprecated(tail)?;
                        tail = new_tail;
                        m_data_deprecated = Some(parsed_m_data_deprecated);
                        continue;
                    } else {
                        tracing::error!("Field m_data_deprecated with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_data");
                    if m_data.is_none() {
                        let (new_tail, parsed_m_data) = Self::parse_m_data(tail)?;
                        tail = new_tail;
                        m_data = Some(parsed_m_data);
                        continue;
                    } else {
                        tracing::error!("Field m_data with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_data_deprecated: m_data_deprecated
                    .expect("Missing m_data_deprecated from struct"),
                m_data: m_data.expect("Missing m_data from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplaySHeader {
    pub m_signature: Vec<u8>,
    pub m_version: SVersion,
    pub m_type: u8,
    pub m_elapsed_game_loops: u32,
    pub m_use_scaled_time: bool,
    pub m_ngdp_root_key: Smd5,
    pub m_data_build_num: u32,
    pub m_replay_compatibility_hash: Smd5,
    pub m_ngdp_root_key_is_dev_data: bool,
}
impl ReplaySHeader {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_signature(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (tail, m_signature) = tagged_blob(input)?;
        tracing::debug!("res: {:?}", m_signature);
        Ok((tail, m_signature))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_version(input: &[u8]) -> IResult<&[u8], SVersion> {
        let (tail, m_version) = SVersion::parse(input)?;
        tracing::debug!("res: {:?}", m_version);
        Ok((tail, m_version))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_type(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_type) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_type);
        Ok((tail, u8::try_from(m_type).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_elapsed_game_loops(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_elapsed_game_loops) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_elapsed_game_loops);
        Ok((tail, u32::try_from(m_elapsed_game_loops).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_use_scaled_time(input: &[u8]) -> IResult<&[u8], bool> {
        let (tail, m_use_scaled_time) = tagged_bool(input)?;
        tracing::debug!("res: {:?}", m_use_scaled_time);
        Ok((tail, m_use_scaled_time))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_ngdp_root_key(input: &[u8]) -> IResult<&[u8], Smd5> {
        let (tail, m_ngdp_root_key) = Smd5::parse(input)?;
        tracing::debug!("res: {:?}", m_ngdp_root_key);
        Ok((tail, m_ngdp_root_key))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_data_build_num(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_data_build_num) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_data_build_num);
        Ok((tail, u32::try_from(m_data_build_num).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_replay_compatibility_hash(input: &[u8]) -> IResult<&[u8], Smd5> {
        let (tail, m_replay_compatibility_hash) = Smd5::parse(input)?;
        tracing::debug!("res: {:?}", m_replay_compatibility_hash);
        Ok((tail, m_replay_compatibility_hash))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_ngdp_root_key_is_dev_data(input: &[u8]) -> IResult<&[u8], bool> {
        let (tail, m_ngdp_root_key_is_dev_data) = tagged_bool(input)?;
        tracing::debug!("res: {:?}", m_ngdp_root_key_is_dev_data);
        Ok((tail, m_ngdp_root_key_is_dev_data))
    }
    #[tracing::instrument(name="ReplaySHeader::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_signature: Option<Vec<u8>> = None;
        let mut m_version: Option<SVersion> = None;
        let mut m_type: Option<u8> = None;
        let mut m_elapsed_game_loops: Option<u32> = None;
        let mut m_use_scaled_time: Option<bool> = None;
        let mut m_ngdp_root_key: Option<Smd5> = None;
        let mut m_data_build_num: Option<u32> = None;
        let mut m_replay_compatibility_hash: Option<Smd5> = None;
        let mut m_ngdp_root_key_is_dev_data: Option<bool> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_signature");
                    if m_signature.is_none() {
                        let (new_tail, parsed_m_signature) = Self::parse_m_signature(tail)?;
                        tail = new_tail;
                        m_signature = Some(parsed_m_signature);
                        continue;
                    } else {
                        tracing::error!("Field m_signature with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_version");
                    if m_version.is_none() {
                        let (new_tail, parsed_m_version) = Self::parse_m_version(tail)?;
                        tail = new_tail;
                        m_version = Some(parsed_m_version);
                        continue;
                    } else {
                        tracing::error!("Field m_version with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!("Field [{i}]: tagged '2' for field m_type");
                    if m_type.is_none() {
                        let (new_tail, parsed_m_type) = Self::parse_m_type(tail)?;
                        tail = new_tail;
                        m_type = Some(parsed_m_type);
                        continue;
                    } else {
                        tracing::error!("Field m_type with tag 2 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                3 => {
                    tracing::debug!("Field [{i}]: tagged '3' for field m_elapsed_game_loops");
                    if m_elapsed_game_loops.is_none() {
                        let (new_tail, parsed_m_elapsed_game_loops) =
                            Self::parse_m_elapsed_game_loops(tail)?;
                        tail = new_tail;
                        m_elapsed_game_loops = Some(parsed_m_elapsed_game_loops);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_elapsed_game_loops with tag 3 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                4 => {
                    tracing::debug!("Field [{i}]: tagged '4' for field m_use_scaled_time");
                    if m_use_scaled_time.is_none() {
                        let (new_tail, parsed_m_use_scaled_time) =
                            Self::parse_m_use_scaled_time(tail)?;
                        tail = new_tail;
                        m_use_scaled_time = Some(parsed_m_use_scaled_time);
                        continue;
                    } else {
                        tracing::error!("Field m_use_scaled_time with tag 4 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                5 => {
                    tracing::debug!("Field [{i}]: tagged '5' for field m_ngdp_root_key");
                    if m_ngdp_root_key.is_none() {
                        let (new_tail, parsed_m_ngdp_root_key) = Self::parse_m_ngdp_root_key(tail)?;
                        tail = new_tail;
                        m_ngdp_root_key = Some(parsed_m_ngdp_root_key);
                        continue;
                    } else {
                        tracing::error!("Field m_ngdp_root_key with tag 5 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                6 => {
                    tracing::debug!("Field [{i}]: tagged '6' for field m_data_build_num");
                    if m_data_build_num.is_none() {
                        let (new_tail, parsed_m_data_build_num) =
                            Self::parse_m_data_build_num(tail)?;
                        tail = new_tail;
                        m_data_build_num = Some(parsed_m_data_build_num);
                        continue;
                    } else {
                        tracing::error!("Field m_data_build_num with tag 6 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                7 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '7' for field m_replay_compatibility_hash"
                    );
                    if m_replay_compatibility_hash.is_none() {
                        let (new_tail, parsed_m_replay_compatibility_hash) =
                            Self::parse_m_replay_compatibility_hash(tail)?;
                        tail = new_tail;
                        m_replay_compatibility_hash = Some(parsed_m_replay_compatibility_hash);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_replay_compatibility_hash with tag 7 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                8 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '8' for field m_ngdp_root_key_is_dev_data"
                    );
                    if m_ngdp_root_key_is_dev_data.is_none() {
                        let (new_tail, parsed_m_ngdp_root_key_is_dev_data) =
                            Self::parse_m_ngdp_root_key_is_dev_data(tail)?;
                        tail = new_tail;
                        m_ngdp_root_key_is_dev_data = Some(parsed_m_ngdp_root_key_is_dev_data);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_ngdp_root_key_is_dev_data with tag 8 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_signature: m_signature.expect("Missing m_signature from struct"),
                m_version: m_version.expect("Missing m_version from struct"),
                m_type: m_type.expect("Missing m_type from struct"),
                m_elapsed_game_loops: m_elapsed_game_loops
                    .expect("Missing m_elapsed_game_loops from struct"),
                m_use_scaled_time: m_use_scaled_time
                    .expect("Missing m_use_scaled_time from struct"),
                m_ngdp_root_key: m_ngdp_root_key.expect("Missing m_ngdp_root_key from struct"),
                m_data_build_num: m_data_build_num.expect("Missing m_data_build_num from struct"),
                m_replay_compatibility_hash: m_replay_compatibility_hash
                    .expect("Missing m_replay_compatibility_hash from struct"),
                m_ngdp_root_key_is_dev_data: m_ngdp_root_key_is_dev_data
                    .expect("Missing m_ngdp_root_key_is_dev_data from struct"),
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReplayTrackerEEventId {
    EPlayerStats(ReplayTrackerSPlayerStatsEvent),
    EUnitBorn(ReplayTrackerSUnitBornEvent),
    EUnitDied(ReplayTrackerSUnitDiedEvent),
    EUnitOwnerChange(ReplayTrackerSUnitOwnerChangeEvent),
    EUnitTypeChange(ReplayTrackerSUnitTypeChangeEvent),
    EUpgrade(ReplayTrackerSUpgradeEvent),
    EUnitInit(ReplayTrackerSUnitInitEvent),
    EUnitDone(ReplayTrackerSUnitDoneEvent),
    EUnitPosition(ReplayTrackerSUnitPositionsEvent),
    EPlayerSetup(ReplayTrackerSPlayerSetupEvent),
}
impl ReplayTrackerEEventId {
    #[tracing::instrument(name="ReplayTrackerEEventId::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_int_tag(input)?;
        let (tail, variant_tag) = parse_vlq_int(tail)?;
        match variant_tag {
            0 => {
                tracing::debug!("Variant EPlayerStats for value '0'");

                let (tail, res) = ReplayTrackerSPlayerStatsEvent::parse(tail)?;
                Ok((tail, Self::EPlayerStats(res)))
            }
            1 => {
                tracing::debug!("Variant EUnitBorn for value '1'");

                let (tail, res) = ReplayTrackerSUnitBornEvent::parse(tail)?;
                Ok((tail, Self::EUnitBorn(res)))
            }
            2 => {
                tracing::debug!("Variant EUnitDied for value '2'");

                let (tail, res) = ReplayTrackerSUnitDiedEvent::parse(tail)?;
                Ok((tail, Self::EUnitDied(res)))
            }
            3 => {
                tracing::debug!("Variant EUnitOwnerChange for value '3'");

                let (tail, res) = ReplayTrackerSUnitOwnerChangeEvent::parse(tail)?;
                Ok((tail, Self::EUnitOwnerChange(res)))
            }
            4 => {
                tracing::debug!("Variant EUnitTypeChange for value '4'");

                let (tail, res) = ReplayTrackerSUnitTypeChangeEvent::parse(tail)?;
                Ok((tail, Self::EUnitTypeChange(res)))
            }
            5 => {
                tracing::debug!("Variant EUpgrade for value '5'");

                let (tail, res) = ReplayTrackerSUpgradeEvent::parse(tail)?;
                Ok((tail, Self::EUpgrade(res)))
            }
            6 => {
                tracing::debug!("Variant EUnitInit for value '6'");

                let (tail, res) = ReplayTrackerSUnitInitEvent::parse(tail)?;
                Ok((tail, Self::EUnitInit(res)))
            }
            7 => {
                tracing::debug!("Variant EUnitDone for value '7'");

                let (tail, res) = ReplayTrackerSUnitDoneEvent::parse(tail)?;
                Ok((tail, Self::EUnitDone(res)))
            }
            8 => {
                tracing::debug!("Variant EUnitPosition for value '8'");

                let (tail, res) = ReplayTrackerSUnitPositionsEvent::parse(tail)?;
                Ok((tail, Self::EUnitPosition(res)))
            }
            9 => {
                tracing::debug!("Variant EPlayerSetup for value '9'");

                let (tail, res) = ReplayTrackerSPlayerSetupEvent::parse(tail)?;
                Ok((tail, Self::EPlayerSetup(res)))
            }

            _ => {
                tracing::error!("Unknown variant value {variant_tag}");
                panic!("Unknown variant value {variant_tag}");
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSPlayerStats {
    pub m_score_value_minerals_current: i32,
    pub m_score_value_vespene_current: i32,
    pub m_score_value_minerals_collection_rate: i32,
    pub m_score_value_vespene_collection_rate: i32,
    pub m_score_value_workers_active_count: i32,
    pub m_score_value_minerals_used_in_progress_army: i32,
    pub m_score_value_minerals_used_in_progress_economy: i32,
    pub m_score_value_minerals_used_in_progress_technology: i32,
    pub m_score_value_vespene_used_in_progress_army: i32,
    pub m_score_value_vespene_used_in_progress_economy: i32,
    pub m_score_value_vespene_used_in_progress_technology: i32,
    pub m_score_value_minerals_used_current_army: i32,
    pub m_score_value_minerals_used_current_economy: i32,
    pub m_score_value_minerals_used_current_technology: i32,
    pub m_score_value_vespene_used_current_army: i32,
    pub m_score_value_vespene_used_current_economy: i32,
    pub m_score_value_vespene_used_current_technology: i32,
    pub m_score_value_minerals_lost_army: i32,
    pub m_score_value_minerals_lost_economy: i32,
    pub m_score_value_minerals_lost_technology: i32,
    pub m_score_value_vespene_lost_army: i32,
    pub m_score_value_vespene_lost_economy: i32,
    pub m_score_value_vespene_lost_technology: i32,
    pub m_score_value_minerals_killed_army: i32,
    pub m_score_value_minerals_killed_economy: i32,
    pub m_score_value_minerals_killed_technology: i32,
    pub m_score_value_vespene_killed_army: i32,
    pub m_score_value_vespene_killed_economy: i32,
    pub m_score_value_vespene_killed_technology: i32,
    pub m_score_value_food_used: i32,
    pub m_score_value_food_made: i32,
    pub m_score_value_minerals_used_active_forces: i32,
    pub m_score_value_vespene_used_active_forces: i32,
    pub m_score_value_minerals_friendly_fire_army: i32,
    pub m_score_value_minerals_friendly_fire_economy: i32,
    pub m_score_value_minerals_friendly_fire_technology: i32,
    pub m_score_value_vespene_friendly_fire_army: i32,
    pub m_score_value_vespene_friendly_fire_economy: i32,
    pub m_score_value_vespene_friendly_fire_technology: i32,
}
impl ReplayTrackerSPlayerStats {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_current(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_current) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_current);
        Ok((tail, i32::try_from(m_score_value_minerals_current).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_current(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_current) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_current);
        Ok((tail, i32::try_from(m_score_value_vespene_current).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_collection_rate(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_collection_rate) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_collection_rate);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_collection_rate).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_collection_rate(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_collection_rate) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_collection_rate);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_collection_rate).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_workers_active_count(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_workers_active_count) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_workers_active_count);
        Ok((
            tail,
            i32::try_from(m_score_value_workers_active_count).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_used_in_progress_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_used_in_progress_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_used_in_progress_army);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_used_in_progress_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_used_in_progress_economy(
        input: &[u8],
    ) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_used_in_progress_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_used_in_progress_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_used_in_progress_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_used_in_progress_technology(
        input: &[u8],
    ) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_used_in_progress_technology) = tagged_vlq_int(input)?;
        tracing::debug!(
            "res: {:?}",
            m_score_value_minerals_used_in_progress_technology
        );
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_used_in_progress_technology).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_used_in_progress_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_used_in_progress_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_used_in_progress_army);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_used_in_progress_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_used_in_progress_economy(
        input: &[u8],
    ) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_used_in_progress_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_used_in_progress_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_used_in_progress_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_used_in_progress_technology(
        input: &[u8],
    ) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_used_in_progress_technology) = tagged_vlq_int(input)?;
        tracing::debug!(
            "res: {:?}",
            m_score_value_vespene_used_in_progress_technology
        );
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_used_in_progress_technology).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_used_current_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_used_current_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_used_current_army);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_used_current_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_used_current_economy(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_used_current_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_used_current_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_used_current_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_used_current_technology(
        input: &[u8],
    ) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_used_current_technology) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_used_current_technology);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_used_current_technology).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_used_current_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_used_current_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_used_current_army);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_used_current_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_used_current_economy(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_used_current_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_used_current_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_used_current_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_used_current_technology(
        input: &[u8],
    ) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_used_current_technology) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_used_current_technology);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_used_current_technology).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_lost_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_lost_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_lost_army);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_lost_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_lost_economy(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_lost_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_lost_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_lost_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_lost_technology(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_lost_technology) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_lost_technology);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_lost_technology).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_lost_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_lost_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_lost_army);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_lost_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_lost_economy(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_lost_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_lost_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_lost_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_lost_technology(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_lost_technology) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_lost_technology);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_lost_technology).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_killed_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_killed_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_killed_army);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_killed_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_killed_economy(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_killed_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_killed_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_killed_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_killed_technology(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_killed_technology) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_killed_technology);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_killed_technology).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_killed_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_killed_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_killed_army);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_killed_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_killed_economy(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_killed_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_killed_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_killed_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_killed_technology(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_killed_technology) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_killed_technology);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_killed_technology).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_food_used(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_food_used) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_food_used);
        Ok((tail, i32::try_from(m_score_value_food_used).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_food_made(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_food_made) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_food_made);
        Ok((tail, i32::try_from(m_score_value_food_made).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_used_active_forces(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_used_active_forces) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_used_active_forces);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_used_active_forces).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_used_active_forces(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_used_active_forces) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_used_active_forces);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_used_active_forces).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_friendly_fire_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_friendly_fire_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_friendly_fire_army);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_friendly_fire_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_friendly_fire_economy(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_friendly_fire_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_friendly_fire_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_friendly_fire_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_minerals_friendly_fire_technology(
        input: &[u8],
    ) -> IResult<&[u8], i32> {
        let (tail, m_score_value_minerals_friendly_fire_technology) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_minerals_friendly_fire_technology);
        Ok((
            tail,
            i32::try_from(m_score_value_minerals_friendly_fire_technology).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_friendly_fire_army(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_friendly_fire_army) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_friendly_fire_army);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_friendly_fire_army).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_friendly_fire_economy(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_friendly_fire_economy) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_friendly_fire_economy);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_friendly_fire_economy).unwrap(),
        ))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_score_value_vespene_friendly_fire_technology(
        input: &[u8],
    ) -> IResult<&[u8], i32> {
        let (tail, m_score_value_vespene_friendly_fire_technology) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_score_value_vespene_friendly_fire_technology);
        Ok((
            tail,
            i32::try_from(m_score_value_vespene_friendly_fire_technology).unwrap(),
        ))
    }
    #[tracing::instrument(name="ReplayTrackerSPlayerStats::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_score_value_minerals_current: Option<i32> = None;
        let mut m_score_value_vespene_current: Option<i32> = None;
        let mut m_score_value_minerals_collection_rate: Option<i32> = None;
        let mut m_score_value_vespene_collection_rate: Option<i32> = None;
        let mut m_score_value_workers_active_count: Option<i32> = None;
        let mut m_score_value_minerals_used_in_progress_army: Option<i32> = None;
        let mut m_score_value_minerals_used_in_progress_economy: Option<i32> = None;
        let mut m_score_value_minerals_used_in_progress_technology: Option<i32> = None;
        let mut m_score_value_vespene_used_in_progress_army: Option<i32> = None;
        let mut m_score_value_vespene_used_in_progress_economy: Option<i32> = None;
        let mut m_score_value_vespene_used_in_progress_technology: Option<i32> = None;
        let mut m_score_value_minerals_used_current_army: Option<i32> = None;
        let mut m_score_value_minerals_used_current_economy: Option<i32> = None;
        let mut m_score_value_minerals_used_current_technology: Option<i32> = None;
        let mut m_score_value_vespene_used_current_army: Option<i32> = None;
        let mut m_score_value_vespene_used_current_economy: Option<i32> = None;
        let mut m_score_value_vespene_used_current_technology: Option<i32> = None;
        let mut m_score_value_minerals_lost_army: Option<i32> = None;
        let mut m_score_value_minerals_lost_economy: Option<i32> = None;
        let mut m_score_value_minerals_lost_technology: Option<i32> = None;
        let mut m_score_value_vespene_lost_army: Option<i32> = None;
        let mut m_score_value_vespene_lost_economy: Option<i32> = None;
        let mut m_score_value_vespene_lost_technology: Option<i32> = None;
        let mut m_score_value_minerals_killed_army: Option<i32> = None;
        let mut m_score_value_minerals_killed_economy: Option<i32> = None;
        let mut m_score_value_minerals_killed_technology: Option<i32> = None;
        let mut m_score_value_vespene_killed_army: Option<i32> = None;
        let mut m_score_value_vespene_killed_economy: Option<i32> = None;
        let mut m_score_value_vespene_killed_technology: Option<i32> = None;
        let mut m_score_value_food_used: Option<i32> = None;
        let mut m_score_value_food_made: Option<i32> = None;
        let mut m_score_value_minerals_used_active_forces: Option<i32> = None;
        let mut m_score_value_vespene_used_active_forces: Option<i32> = None;
        let mut m_score_value_minerals_friendly_fire_army: Option<i32> = None;
        let mut m_score_value_minerals_friendly_fire_economy: Option<i32> = None;
        let mut m_score_value_minerals_friendly_fire_technology: Option<i32> = None;
        let mut m_score_value_vespene_friendly_fire_army: Option<i32> = None;
        let mut m_score_value_vespene_friendly_fire_economy: Option<i32> = None;
        let mut m_score_value_vespene_friendly_fire_technology: Option<i32> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '0' for field m_score_value_minerals_current"
                    );
                    if m_score_value_minerals_current.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_current) =
                            Self::parse_m_score_value_minerals_current(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_current =
                            Some(parsed_m_score_value_minerals_current);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_score_value_minerals_current with tag 0 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '1' for field m_score_value_vespene_current"
                    );
                    if m_score_value_vespene_current.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_current) =
                            Self::parse_m_score_value_vespene_current(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_current = Some(parsed_m_score_value_vespene_current);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_score_value_vespene_current with tag 1 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '2' for field m_score_value_minerals_collection_rate"
                    );
                    if m_score_value_minerals_collection_rate.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_collection_rate) =
                            Self::parse_m_score_value_minerals_collection_rate(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_collection_rate =
                            Some(parsed_m_score_value_minerals_collection_rate);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_collection_rate with tag 2 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                3 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '3' for field m_score_value_vespene_collection_rate"
                    );
                    if m_score_value_vespene_collection_rate.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_collection_rate) =
                            Self::parse_m_score_value_vespene_collection_rate(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_collection_rate =
                            Some(parsed_m_score_value_vespene_collection_rate);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_collection_rate with tag 3 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                4 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '4' for field m_score_value_workers_active_count"
                    );
                    if m_score_value_workers_active_count.is_none() {
                        let (new_tail, parsed_m_score_value_workers_active_count) =
                            Self::parse_m_score_value_workers_active_count(tail)?;
                        tail = new_tail;
                        m_score_value_workers_active_count =
                            Some(parsed_m_score_value_workers_active_count);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_workers_active_count with tag 4 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                5 => {
                    tracing::debug!("Field [{i}]: tagged '5' for field m_score_value_minerals_used_in_progress_army");
                    if m_score_value_minerals_used_in_progress_army.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_used_in_progress_army) =
                            Self::parse_m_score_value_minerals_used_in_progress_army(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_used_in_progress_army =
                            Some(parsed_m_score_value_minerals_used_in_progress_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_used_in_progress_army with tag 5 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                6 => {
                    tracing::debug!("Field [{i}]: tagged '6' for field m_score_value_minerals_used_in_progress_economy");
                    if m_score_value_minerals_used_in_progress_economy.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_used_in_progress_economy) =
                            Self::parse_m_score_value_minerals_used_in_progress_economy(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_used_in_progress_economy =
                            Some(parsed_m_score_value_minerals_used_in_progress_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_used_in_progress_economy with tag 6 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                7 => {
                    tracing::debug!("Field [{i}]: tagged '7' for field m_score_value_minerals_used_in_progress_technology");
                    if m_score_value_minerals_used_in_progress_technology.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_used_in_progress_technology) =
                            Self::parse_m_score_value_minerals_used_in_progress_technology(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_used_in_progress_technology =
                            Some(parsed_m_score_value_minerals_used_in_progress_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_used_in_progress_technology with tag 7 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                8 => {
                    tracing::debug!("Field [{i}]: tagged '8' for field m_score_value_vespene_used_in_progress_army");
                    if m_score_value_vespene_used_in_progress_army.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_used_in_progress_army) =
                            Self::parse_m_score_value_vespene_used_in_progress_army(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_used_in_progress_army =
                            Some(parsed_m_score_value_vespene_used_in_progress_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_used_in_progress_army with tag 8 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                9 => {
                    tracing::debug!("Field [{i}]: tagged '9' for field m_score_value_vespene_used_in_progress_economy");
                    if m_score_value_vespene_used_in_progress_economy.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_used_in_progress_economy) =
                            Self::parse_m_score_value_vespene_used_in_progress_economy(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_used_in_progress_economy =
                            Some(parsed_m_score_value_vespene_used_in_progress_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_used_in_progress_economy with tag 9 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                10 => {
                    tracing::debug!("Field [{i}]: tagged '10' for field m_score_value_vespene_used_in_progress_technology");
                    if m_score_value_vespene_used_in_progress_technology.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_used_in_progress_technology) =
                            Self::parse_m_score_value_vespene_used_in_progress_technology(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_used_in_progress_technology =
                            Some(parsed_m_score_value_vespene_used_in_progress_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_used_in_progress_technology with tag 10 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                11 => {
                    tracing::debug!("Field [{i}]: tagged '11' for field m_score_value_minerals_used_current_army");
                    if m_score_value_minerals_used_current_army.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_used_current_army) =
                            Self::parse_m_score_value_minerals_used_current_army(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_used_current_army =
                            Some(parsed_m_score_value_minerals_used_current_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_used_current_army with tag 11 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                12 => {
                    tracing::debug!("Field [{i}]: tagged '12' for field m_score_value_minerals_used_current_economy");
                    if m_score_value_minerals_used_current_economy.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_used_current_economy) =
                            Self::parse_m_score_value_minerals_used_current_economy(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_used_current_economy =
                            Some(parsed_m_score_value_minerals_used_current_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_used_current_economy with tag 12 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                13 => {
                    tracing::debug!("Field [{i}]: tagged '13' for field m_score_value_minerals_used_current_technology");
                    if m_score_value_minerals_used_current_technology.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_used_current_technology) =
                            Self::parse_m_score_value_minerals_used_current_technology(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_used_current_technology =
                            Some(parsed_m_score_value_minerals_used_current_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_used_current_technology with tag 13 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                14 => {
                    tracing::debug!("Field [{i}]: tagged '14' for field m_score_value_vespene_used_current_army");
                    if m_score_value_vespene_used_current_army.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_used_current_army) =
                            Self::parse_m_score_value_vespene_used_current_army(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_used_current_army =
                            Some(parsed_m_score_value_vespene_used_current_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_used_current_army with tag 14 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                15 => {
                    tracing::debug!("Field [{i}]: tagged '15' for field m_score_value_vespene_used_current_economy");
                    if m_score_value_vespene_used_current_economy.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_used_current_economy) =
                            Self::parse_m_score_value_vespene_used_current_economy(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_used_current_economy =
                            Some(parsed_m_score_value_vespene_used_current_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_used_current_economy with tag 15 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                16 => {
                    tracing::debug!("Field [{i}]: tagged '16' for field m_score_value_vespene_used_current_technology");
                    if m_score_value_vespene_used_current_technology.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_used_current_technology) =
                            Self::parse_m_score_value_vespene_used_current_technology(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_used_current_technology =
                            Some(parsed_m_score_value_vespene_used_current_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_used_current_technology with tag 16 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                17 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '17' for field m_score_value_minerals_lost_army"
                    );
                    if m_score_value_minerals_lost_army.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_lost_army) =
                            Self::parse_m_score_value_minerals_lost_army(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_lost_army =
                            Some(parsed_m_score_value_minerals_lost_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_lost_army with tag 17 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                18 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '18' for field m_score_value_minerals_lost_economy"
                    );
                    if m_score_value_minerals_lost_economy.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_lost_economy) =
                            Self::parse_m_score_value_minerals_lost_economy(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_lost_economy =
                            Some(parsed_m_score_value_minerals_lost_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_lost_economy with tag 18 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                19 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '19' for field m_score_value_minerals_lost_technology"
                    );
                    if m_score_value_minerals_lost_technology.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_lost_technology) =
                            Self::parse_m_score_value_minerals_lost_technology(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_lost_technology =
                            Some(parsed_m_score_value_minerals_lost_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_lost_technology with tag 19 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                20 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '20' for field m_score_value_vespene_lost_army"
                    );
                    if m_score_value_vespene_lost_army.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_lost_army) =
                            Self::parse_m_score_value_vespene_lost_army(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_lost_army =
                            Some(parsed_m_score_value_vespene_lost_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_lost_army with tag 20 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                21 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '21' for field m_score_value_vespene_lost_economy"
                    );
                    if m_score_value_vespene_lost_economy.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_lost_economy) =
                            Self::parse_m_score_value_vespene_lost_economy(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_lost_economy =
                            Some(parsed_m_score_value_vespene_lost_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_lost_economy with tag 21 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                22 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '22' for field m_score_value_vespene_lost_technology"
                    );
                    if m_score_value_vespene_lost_technology.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_lost_technology) =
                            Self::parse_m_score_value_vespene_lost_technology(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_lost_technology =
                            Some(parsed_m_score_value_vespene_lost_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_lost_technology with tag 22 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                23 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '23' for field m_score_value_minerals_killed_army"
                    );
                    if m_score_value_minerals_killed_army.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_killed_army) =
                            Self::parse_m_score_value_minerals_killed_army(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_killed_army =
                            Some(parsed_m_score_value_minerals_killed_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_killed_army with tag 23 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                24 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '24' for field m_score_value_minerals_killed_economy"
                    );
                    if m_score_value_minerals_killed_economy.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_killed_economy) =
                            Self::parse_m_score_value_minerals_killed_economy(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_killed_economy =
                            Some(parsed_m_score_value_minerals_killed_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_killed_economy with tag 24 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                25 => {
                    tracing::debug!("Field [{i}]: tagged '25' for field m_score_value_minerals_killed_technology");
                    if m_score_value_minerals_killed_technology.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_killed_technology) =
                            Self::parse_m_score_value_minerals_killed_technology(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_killed_technology =
                            Some(parsed_m_score_value_minerals_killed_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_killed_technology with tag 25 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                26 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '26' for field m_score_value_vespene_killed_army"
                    );
                    if m_score_value_vespene_killed_army.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_killed_army) =
                            Self::parse_m_score_value_vespene_killed_army(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_killed_army =
                            Some(parsed_m_score_value_vespene_killed_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_killed_army with tag 26 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                27 => {
                    tracing::debug!(
                        "Field [{i}]: tagged '27' for field m_score_value_vespene_killed_economy"
                    );
                    if m_score_value_vespene_killed_economy.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_killed_economy) =
                            Self::parse_m_score_value_vespene_killed_economy(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_killed_economy =
                            Some(parsed_m_score_value_vespene_killed_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_killed_economy with tag 27 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                28 => {
                    tracing::debug!("Field [{i}]: tagged '28' for field m_score_value_vespene_killed_technology");
                    if m_score_value_vespene_killed_technology.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_killed_technology) =
                            Self::parse_m_score_value_vespene_killed_technology(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_killed_technology =
                            Some(parsed_m_score_value_vespene_killed_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_killed_technology with tag 28 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                29 => {
                    tracing::debug!("Field [{i}]: tagged '29' for field m_score_value_food_used");
                    if m_score_value_food_used.is_none() {
                        let (new_tail, parsed_m_score_value_food_used) =
                            Self::parse_m_score_value_food_used(tail)?;
                        tail = new_tail;
                        m_score_value_food_used = Some(parsed_m_score_value_food_used);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_score_value_food_used with tag 29 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                30 => {
                    tracing::debug!("Field [{i}]: tagged '30' for field m_score_value_food_made");
                    if m_score_value_food_made.is_none() {
                        let (new_tail, parsed_m_score_value_food_made) =
                            Self::parse_m_score_value_food_made(tail)?;
                        tail = new_tail;
                        m_score_value_food_made = Some(parsed_m_score_value_food_made);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_score_value_food_made with tag 30 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                31 => {
                    tracing::debug!("Field [{i}]: tagged '31' for field m_score_value_minerals_used_active_forces");
                    if m_score_value_minerals_used_active_forces.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_used_active_forces) =
                            Self::parse_m_score_value_minerals_used_active_forces(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_used_active_forces =
                            Some(parsed_m_score_value_minerals_used_active_forces);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_used_active_forces with tag 31 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                32 => {
                    tracing::debug!("Field [{i}]: tagged '32' for field m_score_value_vespene_used_active_forces");
                    if m_score_value_vespene_used_active_forces.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_used_active_forces) =
                            Self::parse_m_score_value_vespene_used_active_forces(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_used_active_forces =
                            Some(parsed_m_score_value_vespene_used_active_forces);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_used_active_forces with tag 32 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                33 => {
                    tracing::debug!("Field [{i}]: tagged '33' for field m_score_value_minerals_friendly_fire_army");
                    if m_score_value_minerals_friendly_fire_army.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_friendly_fire_army) =
                            Self::parse_m_score_value_minerals_friendly_fire_army(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_friendly_fire_army =
                            Some(parsed_m_score_value_minerals_friendly_fire_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_friendly_fire_army with tag 33 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                34 => {
                    tracing::debug!("Field [{i}]: tagged '34' for field m_score_value_minerals_friendly_fire_economy");
                    if m_score_value_minerals_friendly_fire_economy.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_friendly_fire_economy) =
                            Self::parse_m_score_value_minerals_friendly_fire_economy(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_friendly_fire_economy =
                            Some(parsed_m_score_value_minerals_friendly_fire_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_friendly_fire_economy with tag 34 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                35 => {
                    tracing::debug!("Field [{i}]: tagged '35' for field m_score_value_minerals_friendly_fire_technology");
                    if m_score_value_minerals_friendly_fire_technology.is_none() {
                        let (new_tail, parsed_m_score_value_minerals_friendly_fire_technology) =
                            Self::parse_m_score_value_minerals_friendly_fire_technology(tail)?;
                        tail = new_tail;
                        m_score_value_minerals_friendly_fire_technology =
                            Some(parsed_m_score_value_minerals_friendly_fire_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_minerals_friendly_fire_technology with tag 35 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                36 => {
                    tracing::debug!("Field [{i}]: tagged '36' for field m_score_value_vespene_friendly_fire_army");
                    if m_score_value_vespene_friendly_fire_army.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_friendly_fire_army) =
                            Self::parse_m_score_value_vespene_friendly_fire_army(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_friendly_fire_army =
                            Some(parsed_m_score_value_vespene_friendly_fire_army);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_friendly_fire_army with tag 36 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                37 => {
                    tracing::debug!("Field [{i}]: tagged '37' for field m_score_value_vespene_friendly_fire_economy");
                    if m_score_value_vespene_friendly_fire_economy.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_friendly_fire_economy) =
                            Self::parse_m_score_value_vespene_friendly_fire_economy(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_friendly_fire_economy =
                            Some(parsed_m_score_value_vespene_friendly_fire_economy);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_friendly_fire_economy with tag 37 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                38 => {
                    tracing::debug!("Field [{i}]: tagged '38' for field m_score_value_vespene_friendly_fire_technology");
                    if m_score_value_vespene_friendly_fire_technology.is_none() {
                        let (new_tail, parsed_m_score_value_vespene_friendly_fire_technology) =
                            Self::parse_m_score_value_vespene_friendly_fire_technology(tail)?;
                        tail = new_tail;
                        m_score_value_vespene_friendly_fire_technology =
                            Some(parsed_m_score_value_vespene_friendly_fire_technology);
                        continue;
                    } else {
                        tracing::error!("Field m_score_value_vespene_friendly_fire_technology with tag 38 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_score_value_minerals_current: m_score_value_minerals_current
                    .expect("Missing m_score_value_minerals_current from struct"),
                m_score_value_vespene_current: m_score_value_vespene_current
                    .expect("Missing m_score_value_vespene_current from struct"),
                m_score_value_minerals_collection_rate: m_score_value_minerals_collection_rate
                    .expect("Missing m_score_value_minerals_collection_rate from struct"),
                m_score_value_vespene_collection_rate: m_score_value_vespene_collection_rate
                    .expect("Missing m_score_value_vespene_collection_rate from struct"),
                m_score_value_workers_active_count: m_score_value_workers_active_count
                    .expect("Missing m_score_value_workers_active_count from struct"),
                m_score_value_minerals_used_in_progress_army:
                    m_score_value_minerals_used_in_progress_army
                        .expect("Missing m_score_value_minerals_used_in_progress_army from struct"),
                m_score_value_minerals_used_in_progress_economy:
                    m_score_value_minerals_used_in_progress_economy.expect(
                        "Missing m_score_value_minerals_used_in_progress_economy from struct",
                    ),
                m_score_value_minerals_used_in_progress_technology:
                    m_score_value_minerals_used_in_progress_technology.expect(
                        "Missing m_score_value_minerals_used_in_progress_technology from struct",
                    ),
                m_score_value_vespene_used_in_progress_army:
                    m_score_value_vespene_used_in_progress_army
                        .expect("Missing m_score_value_vespene_used_in_progress_army from struct"),
                m_score_value_vespene_used_in_progress_economy:
                    m_score_value_vespene_used_in_progress_economy.expect(
                        "Missing m_score_value_vespene_used_in_progress_economy from struct",
                    ),
                m_score_value_vespene_used_in_progress_technology:
                    m_score_value_vespene_used_in_progress_technology.expect(
                        "Missing m_score_value_vespene_used_in_progress_technology from struct",
                    ),
                m_score_value_minerals_used_current_army: m_score_value_minerals_used_current_army
                    .expect("Missing m_score_value_minerals_used_current_army from struct"),
                m_score_value_minerals_used_current_economy:
                    m_score_value_minerals_used_current_economy
                        .expect("Missing m_score_value_minerals_used_current_economy from struct"),
                m_score_value_minerals_used_current_technology:
                    m_score_value_minerals_used_current_technology.expect(
                        "Missing m_score_value_minerals_used_current_technology from struct",
                    ),
                m_score_value_vespene_used_current_army: m_score_value_vespene_used_current_army
                    .expect("Missing m_score_value_vespene_used_current_army from struct"),
                m_score_value_vespene_used_current_economy:
                    m_score_value_vespene_used_current_economy
                        .expect("Missing m_score_value_vespene_used_current_economy from struct"),
                m_score_value_vespene_used_current_technology:
                    m_score_value_vespene_used_current_technology
                        .expect("Missing m_score_value_vespene_used_current_technology from struct"),
                m_score_value_minerals_lost_army: m_score_value_minerals_lost_army
                    .expect("Missing m_score_value_minerals_lost_army from struct"),
                m_score_value_minerals_lost_economy: m_score_value_minerals_lost_economy
                    .expect("Missing m_score_value_minerals_lost_economy from struct"),
                m_score_value_minerals_lost_technology: m_score_value_minerals_lost_technology
                    .expect("Missing m_score_value_minerals_lost_technology from struct"),
                m_score_value_vespene_lost_army: m_score_value_vespene_lost_army
                    .expect("Missing m_score_value_vespene_lost_army from struct"),
                m_score_value_vespene_lost_economy: m_score_value_vespene_lost_economy
                    .expect("Missing m_score_value_vespene_lost_economy from struct"),
                m_score_value_vespene_lost_technology: m_score_value_vespene_lost_technology
                    .expect("Missing m_score_value_vespene_lost_technology from struct"),
                m_score_value_minerals_killed_army: m_score_value_minerals_killed_army
                    .expect("Missing m_score_value_minerals_killed_army from struct"),
                m_score_value_minerals_killed_economy: m_score_value_minerals_killed_economy
                    .expect("Missing m_score_value_minerals_killed_economy from struct"),
                m_score_value_minerals_killed_technology: m_score_value_minerals_killed_technology
                    .expect("Missing m_score_value_minerals_killed_technology from struct"),
                m_score_value_vespene_killed_army: m_score_value_vespene_killed_army
                    .expect("Missing m_score_value_vespene_killed_army from struct"),
                m_score_value_vespene_killed_economy: m_score_value_vespene_killed_economy
                    .expect("Missing m_score_value_vespene_killed_economy from struct"),
                m_score_value_vespene_killed_technology: m_score_value_vespene_killed_technology
                    .expect("Missing m_score_value_vespene_killed_technology from struct"),
                m_score_value_food_used: m_score_value_food_used
                    .expect("Missing m_score_value_food_used from struct"),
                m_score_value_food_made: m_score_value_food_made
                    .expect("Missing m_score_value_food_made from struct"),
                m_score_value_minerals_used_active_forces:
                    m_score_value_minerals_used_active_forces
                        .expect("Missing m_score_value_minerals_used_active_forces from struct"),
                m_score_value_vespene_used_active_forces: m_score_value_vespene_used_active_forces
                    .expect("Missing m_score_value_vespene_used_active_forces from struct"),
                m_score_value_minerals_friendly_fire_army:
                    m_score_value_minerals_friendly_fire_army
                        .expect("Missing m_score_value_minerals_friendly_fire_army from struct"),
                m_score_value_minerals_friendly_fire_economy:
                    m_score_value_minerals_friendly_fire_economy
                        .expect("Missing m_score_value_minerals_friendly_fire_economy from struct"),
                m_score_value_minerals_friendly_fire_technology:
                    m_score_value_minerals_friendly_fire_technology.expect(
                        "Missing m_score_value_minerals_friendly_fire_technology from struct",
                    ),
                m_score_value_vespene_friendly_fire_army: m_score_value_vespene_friendly_fire_army
                    .expect("Missing m_score_value_vespene_friendly_fire_army from struct"),
                m_score_value_vespene_friendly_fire_economy:
                    m_score_value_vespene_friendly_fire_economy
                        .expect("Missing m_score_value_vespene_friendly_fire_economy from struct"),
                m_score_value_vespene_friendly_fire_technology:
                    m_score_value_vespene_friendly_fire_technology.expect(
                        "Missing m_score_value_vespene_friendly_fire_technology from struct",
                    ),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSPlayerStatsEvent {
    pub m_player_id: u8,
    pub m_stats: ReplayTrackerSPlayerStats,
}
impl ReplayTrackerSPlayerStatsEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_player_id(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_player_id) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_player_id);
        Ok((tail, u8::try_from(m_player_id).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_stats(input: &[u8]) -> IResult<&[u8], ReplayTrackerSPlayerStats> {
        let (tail, m_stats) = ReplayTrackerSPlayerStats::parse(input)?;
        tracing::debug!("res: {:?}", m_stats);
        Ok((tail, m_stats))
    }
    #[tracing::instrument(name="ReplayTrackerSPlayerStatsEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_player_id: Option<u8> = None;
        let mut m_stats: Option<ReplayTrackerSPlayerStats> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_player_id");
                    if m_player_id.is_none() {
                        let (new_tail, parsed_m_player_id) = Self::parse_m_player_id(tail)?;
                        tail = new_tail;
                        m_player_id = Some(parsed_m_player_id);
                        continue;
                    } else {
                        tracing::error!("Field m_player_id with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_stats");
                    if m_stats.is_none() {
                        let (new_tail, parsed_m_stats) = Self::parse_m_stats(tail)?;
                        tail = new_tail;
                        m_stats = Some(parsed_m_stats);
                        continue;
                    } else {
                        tracing::error!("Field m_stats with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_player_id: m_player_id.expect("Missing m_player_id from struct"),
                m_stats: m_stats.expect("Missing m_stats from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSUnitBornEvent {
    pub m_unit_tag_index: u32,
    pub m_unit_tag_recycle: u32,
    pub m_unit_type_name: Vec<u8>,
    pub m_control_player_id: u8,
    pub m_upkeep_player_id: u8,
    pub m_x: u8,
    pub m_y: u8,
    pub m_creator_unit_tag_index: Option<u32>,
    pub m_creator_unit_tag_recycle: Option<u32>,
    pub m_creator_ability_name: Option<Vec<u8>>,
}
impl ReplayTrackerSUnitBornEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_index(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_index);
        Ok((tail, u32::try_from(m_unit_tag_index).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_recycle(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_recycle);
        Ok((tail, u32::try_from(m_unit_tag_recycle).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_type_name(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (tail, m_unit_type_name) = tagged_blob(input)?;
        tracing::debug!("res: {:?}", m_unit_type_name);
        Ok((tail, m_unit_type_name))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_control_player_id(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_control_player_id) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_control_player_id);
        Ok((tail, u8::try_from(m_control_player_id).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_upkeep_player_id(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_upkeep_player_id) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_upkeep_player_id);
        Ok((tail, u8::try_from(m_upkeep_player_id).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_x(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_x) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_x);
        Ok((tail, u8::try_from(m_x).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_y(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_y) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_y);
        Ok((tail, u8::try_from(m_y).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_creator_unit_tag_index(input: &[u8]) -> IResult<&[u8], Option<u32>> {
        let (tail, _) = validate_opt_tag(input)?;
        let (tail, is_provided) = nom::number::complete::u8(tail)?;
        let (tail, m_creator_unit_tag_index) = if is_provided != 0 {
            let (tail, res) = tagged_vlq_int(tail)?;
            (tail, Some(<_>::try_from(res).unwrap()))
        } else {
            (tail, None)
        };
        tracing::debug!("res: {:?}", m_creator_unit_tag_index);
        Ok((tail, m_creator_unit_tag_index))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_creator_unit_tag_recycle(input: &[u8]) -> IResult<&[u8], Option<u32>> {
        let (tail, _) = validate_opt_tag(input)?;
        let (tail, is_provided) = nom::number::complete::u8(tail)?;
        let (tail, m_creator_unit_tag_recycle) = if is_provided != 0 {
            let (tail, res) = tagged_vlq_int(tail)?;
            (tail, Some(<_>::try_from(res).unwrap()))
        } else {
            (tail, None)
        };
        tracing::debug!("res: {:?}", m_creator_unit_tag_recycle);
        Ok((tail, m_creator_unit_tag_recycle))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_creator_ability_name(input: &[u8]) -> IResult<&[u8], Option<Vec<u8>>> {
        let (tail, _) = validate_opt_tag(input)?;
        let (tail, is_provided) = nom::number::complete::u8(tail)?;
        let (tail, m_creator_ability_name) = if is_provided != 0 {
            let (tail, res) = tagged_blob(tail)?;
            (tail, Some(res))
        } else {
            (tail, None)
        };
        tracing::debug!("res: {:?}", m_creator_ability_name);
        Ok((tail, m_creator_ability_name))
    }
    #[tracing::instrument(name="ReplayTrackerSUnitBornEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_unit_tag_index: Option<u32> = None;
        let mut m_unit_tag_recycle: Option<u32> = None;
        let mut m_unit_type_name: Option<Vec<u8>> = None;
        let mut m_control_player_id: Option<u8> = None;
        let mut m_upkeep_player_id: Option<u8> = None;
        let mut m_x: Option<u8> = None;
        let mut m_y: Option<u8> = None;
        let mut m_creator_unit_tag_index: Option<Option<u32>> = Some(None);
        let mut m_creator_unit_tag_recycle: Option<Option<u32>> = Some(None);
        let mut m_creator_ability_name: Option<Option<Vec<u8>>> = Some(None);
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_unit_tag_index");
                    if m_unit_tag_index.is_none() {
                        let (new_tail, parsed_m_unit_tag_index) =
                            Self::parse_m_unit_tag_index(tail)?;
                        tail = new_tail;
                        m_unit_tag_index = Some(parsed_m_unit_tag_index);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_index with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_unit_tag_recycle");
                    if m_unit_tag_recycle.is_none() {
                        let (new_tail, parsed_m_unit_tag_recycle) =
                            Self::parse_m_unit_tag_recycle(tail)?;
                        tail = new_tail;
                        m_unit_tag_recycle = Some(parsed_m_unit_tag_recycle);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_recycle with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!("Field [{i}]: tagged '2' for field m_unit_type_name");
                    if m_unit_type_name.is_none() {
                        let (new_tail, parsed_m_unit_type_name) =
                            Self::parse_m_unit_type_name(tail)?;
                        tail = new_tail;
                        m_unit_type_name = Some(parsed_m_unit_type_name);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_type_name with tag 2 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                3 => {
                    tracing::debug!("Field [{i}]: tagged '3' for field m_control_player_id");
                    if m_control_player_id.is_none() {
                        let (new_tail, parsed_m_control_player_id) =
                            Self::parse_m_control_player_id(tail)?;
                        tail = new_tail;
                        m_control_player_id = Some(parsed_m_control_player_id);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_control_player_id with tag 3 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                4 => {
                    tracing::debug!("Field [{i}]: tagged '4' for field m_upkeep_player_id");
                    if m_upkeep_player_id.is_none() {
                        let (new_tail, parsed_m_upkeep_player_id) =
                            Self::parse_m_upkeep_player_id(tail)?;
                        tail = new_tail;
                        m_upkeep_player_id = Some(parsed_m_upkeep_player_id);
                        continue;
                    } else {
                        tracing::error!("Field m_upkeep_player_id with tag 4 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                5 => {
                    tracing::debug!("Field [{i}]: tagged '5' for field m_x");
                    if m_x.is_none() {
                        let (new_tail, parsed_m_x) = Self::parse_m_x(tail)?;
                        tail = new_tail;
                        m_x = Some(parsed_m_x);
                        continue;
                    } else {
                        tracing::error!("Field m_x with tag 5 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                6 => {
                    tracing::debug!("Field [{i}]: tagged '6' for field m_y");
                    if m_y.is_none() {
                        let (new_tail, parsed_m_y) = Self::parse_m_y(tail)?;
                        tail = new_tail;
                        m_y = Some(parsed_m_y);
                        continue;
                    } else {
                        tracing::error!("Field m_y with tag 6 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                7 => {
                    tracing::debug!("Field [{i}]: tagged '7' for field m_creator_unit_tag_index");
                    if let Some(None) = m_creator_unit_tag_index {
                        let (new_tail, parsed_m_creator_unit_tag_index) =
                            Self::parse_m_creator_unit_tag_index(tail)?;
                        tail = new_tail;
                        m_creator_unit_tag_index = Some(parsed_m_creator_unit_tag_index);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_creator_unit_tag_index with tag 7 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                8 => {
                    tracing::debug!("Field [{i}]: tagged '8' for field m_creator_unit_tag_recycle");
                    if let Some(None) = m_creator_unit_tag_recycle {
                        let (new_tail, parsed_m_creator_unit_tag_recycle) =
                            Self::parse_m_creator_unit_tag_recycle(tail)?;
                        tail = new_tail;
                        m_creator_unit_tag_recycle = Some(parsed_m_creator_unit_tag_recycle);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_creator_unit_tag_recycle with tag 8 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                9 => {
                    tracing::debug!("Field [{i}]: tagged '9' for field m_creator_ability_name");
                    if let Some(None) = m_creator_ability_name {
                        let (new_tail, parsed_m_creator_ability_name) =
                            Self::parse_m_creator_ability_name(tail)?;
                        tail = new_tail;
                        m_creator_ability_name = Some(parsed_m_creator_ability_name);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_creator_ability_name with tag 9 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_unit_tag_index: m_unit_tag_index.expect("Missing m_unit_tag_index from struct"),
                m_unit_tag_recycle: m_unit_tag_recycle
                    .expect("Missing m_unit_tag_recycle from struct"),
                m_unit_type_name: m_unit_type_name.expect("Missing m_unit_type_name from struct"),
                m_control_player_id: m_control_player_id
                    .expect("Missing m_control_player_id from struct"),
                m_upkeep_player_id: m_upkeep_player_id
                    .expect("Missing m_upkeep_player_id from struct"),
                m_x: m_x.expect("Missing m_x from struct"),
                m_y: m_y.expect("Missing m_y from struct"),
                m_creator_unit_tag_index: m_creator_unit_tag_index
                    .expect("Missing m_creator_unit_tag_index from struct"),
                m_creator_unit_tag_recycle: m_creator_unit_tag_recycle
                    .expect("Missing m_creator_unit_tag_recycle from struct"),
                m_creator_ability_name: m_creator_ability_name
                    .expect("Missing m_creator_ability_name from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSUnitDiedEvent {
    pub m_unit_tag_index: u32,
    pub m_unit_tag_recycle: u32,
    pub m_killer_player_id: Option<u8>,
    pub m_x: u8,
    pub m_y: u8,
    pub m_killer_unit_tag_index: Option<u32>,
    pub m_killer_unit_tag_recycle: Option<u32>,
}
impl ReplayTrackerSUnitDiedEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_index(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_index);
        Ok((tail, u32::try_from(m_unit_tag_index).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_recycle(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_recycle);
        Ok((tail, u32::try_from(m_unit_tag_recycle).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_killer_player_id(input: &[u8]) -> IResult<&[u8], Option<u8>> {
        let (tail, _) = validate_opt_tag(input)?;
        let (tail, is_provided) = nom::number::complete::u8(tail)?;
        let (tail, m_killer_player_id) = if is_provided != 0 {
            let (tail, res) = tagged_vlq_int(tail)?;
            (tail, Some(<_>::try_from(res).unwrap()))
        } else {
            (tail, None)
        };
        tracing::debug!("res: {:?}", m_killer_player_id);
        Ok((tail, m_killer_player_id))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_x(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_x) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_x);
        Ok((tail, u8::try_from(m_x).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_y(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_y) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_y);
        Ok((tail, u8::try_from(m_y).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_killer_unit_tag_index(input: &[u8]) -> IResult<&[u8], Option<u32>> {
        let (tail, _) = validate_opt_tag(input)?;
        let (tail, is_provided) = nom::number::complete::u8(tail)?;
        let (tail, m_killer_unit_tag_index) = if is_provided != 0 {
            let (tail, res) = tagged_vlq_int(tail)?;
            (tail, Some(<_>::try_from(res).unwrap()))
        } else {
            (tail, None)
        };
        tracing::debug!("res: {:?}", m_killer_unit_tag_index);
        Ok((tail, m_killer_unit_tag_index))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_killer_unit_tag_recycle(input: &[u8]) -> IResult<&[u8], Option<u32>> {
        let (tail, _) = validate_opt_tag(input)?;
        let (tail, is_provided) = nom::number::complete::u8(tail)?;
        let (tail, m_killer_unit_tag_recycle) = if is_provided != 0 {
            let (tail, res) = tagged_vlq_int(tail)?;
            (tail, Some(<_>::try_from(res).unwrap()))
        } else {
            (tail, None)
        };
        tracing::debug!("res: {:?}", m_killer_unit_tag_recycle);
        Ok((tail, m_killer_unit_tag_recycle))
    }
    #[tracing::instrument(name="ReplayTrackerSUnitDiedEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_unit_tag_index: Option<u32> = None;
        let mut m_unit_tag_recycle: Option<u32> = None;
        let mut m_killer_player_id: Option<Option<u8>> = Some(None);
        let mut m_x: Option<u8> = None;
        let mut m_y: Option<u8> = None;
        let mut m_killer_unit_tag_index: Option<Option<u32>> = Some(None);
        let mut m_killer_unit_tag_recycle: Option<Option<u32>> = Some(None);
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_unit_tag_index");
                    if m_unit_tag_index.is_none() {
                        let (new_tail, parsed_m_unit_tag_index) =
                            Self::parse_m_unit_tag_index(tail)?;
                        tail = new_tail;
                        m_unit_tag_index = Some(parsed_m_unit_tag_index);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_index with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_unit_tag_recycle");
                    if m_unit_tag_recycle.is_none() {
                        let (new_tail, parsed_m_unit_tag_recycle) =
                            Self::parse_m_unit_tag_recycle(tail)?;
                        tail = new_tail;
                        m_unit_tag_recycle = Some(parsed_m_unit_tag_recycle);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_recycle with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!("Field [{i}]: tagged '2' for field m_killer_player_id");
                    if let Some(None) = m_killer_player_id {
                        let (new_tail, parsed_m_killer_player_id) =
                            Self::parse_m_killer_player_id(tail)?;
                        tail = new_tail;
                        m_killer_player_id = Some(parsed_m_killer_player_id);
                        continue;
                    } else {
                        tracing::error!("Field m_killer_player_id with tag 2 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                3 => {
                    tracing::debug!("Field [{i}]: tagged '3' for field m_x");
                    if m_x.is_none() {
                        let (new_tail, parsed_m_x) = Self::parse_m_x(tail)?;
                        tail = new_tail;
                        m_x = Some(parsed_m_x);
                        continue;
                    } else {
                        tracing::error!("Field m_x with tag 3 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                4 => {
                    tracing::debug!("Field [{i}]: tagged '4' for field m_y");
                    if m_y.is_none() {
                        let (new_tail, parsed_m_y) = Self::parse_m_y(tail)?;
                        tail = new_tail;
                        m_y = Some(parsed_m_y);
                        continue;
                    } else {
                        tracing::error!("Field m_y with tag 4 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                5 => {
                    tracing::debug!("Field [{i}]: tagged '5' for field m_killer_unit_tag_index");
                    if let Some(None) = m_killer_unit_tag_index {
                        let (new_tail, parsed_m_killer_unit_tag_index) =
                            Self::parse_m_killer_unit_tag_index(tail)?;
                        tail = new_tail;
                        m_killer_unit_tag_index = Some(parsed_m_killer_unit_tag_index);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_killer_unit_tag_index with tag 5 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                6 => {
                    tracing::debug!("Field [{i}]: tagged '6' for field m_killer_unit_tag_recycle");
                    if let Some(None) = m_killer_unit_tag_recycle {
                        let (new_tail, parsed_m_killer_unit_tag_recycle) =
                            Self::parse_m_killer_unit_tag_recycle(tail)?;
                        tail = new_tail;
                        m_killer_unit_tag_recycle = Some(parsed_m_killer_unit_tag_recycle);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_killer_unit_tag_recycle with tag 6 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_unit_tag_index: m_unit_tag_index.expect("Missing m_unit_tag_index from struct"),
                m_unit_tag_recycle: m_unit_tag_recycle
                    .expect("Missing m_unit_tag_recycle from struct"),
                m_killer_player_id: m_killer_player_id
                    .expect("Missing m_killer_player_id from struct"),
                m_x: m_x.expect("Missing m_x from struct"),
                m_y: m_y.expect("Missing m_y from struct"),
                m_killer_unit_tag_index: m_killer_unit_tag_index
                    .expect("Missing m_killer_unit_tag_index from struct"),
                m_killer_unit_tag_recycle: m_killer_unit_tag_recycle
                    .expect("Missing m_killer_unit_tag_recycle from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSUnitOwnerChangeEvent {
    pub m_unit_tag_index: u32,
    pub m_unit_tag_recycle: u32,
    pub m_control_player_id: u8,
    pub m_upkeep_player_id: u8,
}
impl ReplayTrackerSUnitOwnerChangeEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_index(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_index);
        Ok((tail, u32::try_from(m_unit_tag_index).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_recycle(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_recycle);
        Ok((tail, u32::try_from(m_unit_tag_recycle).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_control_player_id(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_control_player_id) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_control_player_id);
        Ok((tail, u8::try_from(m_control_player_id).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_upkeep_player_id(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_upkeep_player_id) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_upkeep_player_id);
        Ok((tail, u8::try_from(m_upkeep_player_id).unwrap()))
    }
    #[tracing::instrument(name="ReplayTrackerSUnitOwnerChangeEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_unit_tag_index: Option<u32> = None;
        let mut m_unit_tag_recycle: Option<u32> = None;
        let mut m_control_player_id: Option<u8> = None;
        let mut m_upkeep_player_id: Option<u8> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_unit_tag_index");
                    if m_unit_tag_index.is_none() {
                        let (new_tail, parsed_m_unit_tag_index) =
                            Self::parse_m_unit_tag_index(tail)?;
                        tail = new_tail;
                        m_unit_tag_index = Some(parsed_m_unit_tag_index);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_index with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_unit_tag_recycle");
                    if m_unit_tag_recycle.is_none() {
                        let (new_tail, parsed_m_unit_tag_recycle) =
                            Self::parse_m_unit_tag_recycle(tail)?;
                        tail = new_tail;
                        m_unit_tag_recycle = Some(parsed_m_unit_tag_recycle);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_recycle with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!("Field [{i}]: tagged '2' for field m_control_player_id");
                    if m_control_player_id.is_none() {
                        let (new_tail, parsed_m_control_player_id) =
                            Self::parse_m_control_player_id(tail)?;
                        tail = new_tail;
                        m_control_player_id = Some(parsed_m_control_player_id);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_control_player_id with tag 2 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                3 => {
                    tracing::debug!("Field [{i}]: tagged '3' for field m_upkeep_player_id");
                    if m_upkeep_player_id.is_none() {
                        let (new_tail, parsed_m_upkeep_player_id) =
                            Self::parse_m_upkeep_player_id(tail)?;
                        tail = new_tail;
                        m_upkeep_player_id = Some(parsed_m_upkeep_player_id);
                        continue;
                    } else {
                        tracing::error!("Field m_upkeep_player_id with tag 3 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_unit_tag_index: m_unit_tag_index.expect("Missing m_unit_tag_index from struct"),
                m_unit_tag_recycle: m_unit_tag_recycle
                    .expect("Missing m_unit_tag_recycle from struct"),
                m_control_player_id: m_control_player_id
                    .expect("Missing m_control_player_id from struct"),
                m_upkeep_player_id: m_upkeep_player_id
                    .expect("Missing m_upkeep_player_id from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSUnitTypeChangeEvent {
    pub m_unit_tag_index: u32,
    pub m_unit_tag_recycle: u32,
    pub m_unit_type_name: Vec<u8>,
}
impl ReplayTrackerSUnitTypeChangeEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_index(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_index);
        Ok((tail, u32::try_from(m_unit_tag_index).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_recycle(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_recycle);
        Ok((tail, u32::try_from(m_unit_tag_recycle).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_type_name(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (tail, m_unit_type_name) = tagged_blob(input)?;
        tracing::debug!("res: {:?}", m_unit_type_name);
        Ok((tail, m_unit_type_name))
    }
    #[tracing::instrument(name="ReplayTrackerSUnitTypeChangeEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_unit_tag_index: Option<u32> = None;
        let mut m_unit_tag_recycle: Option<u32> = None;
        let mut m_unit_type_name: Option<Vec<u8>> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_unit_tag_index");
                    if m_unit_tag_index.is_none() {
                        let (new_tail, parsed_m_unit_tag_index) =
                            Self::parse_m_unit_tag_index(tail)?;
                        tail = new_tail;
                        m_unit_tag_index = Some(parsed_m_unit_tag_index);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_index with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_unit_tag_recycle");
                    if m_unit_tag_recycle.is_none() {
                        let (new_tail, parsed_m_unit_tag_recycle) =
                            Self::parse_m_unit_tag_recycle(tail)?;
                        tail = new_tail;
                        m_unit_tag_recycle = Some(parsed_m_unit_tag_recycle);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_recycle with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!("Field [{i}]: tagged '2' for field m_unit_type_name");
                    if m_unit_type_name.is_none() {
                        let (new_tail, parsed_m_unit_type_name) =
                            Self::parse_m_unit_type_name(tail)?;
                        tail = new_tail;
                        m_unit_type_name = Some(parsed_m_unit_type_name);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_type_name with tag 2 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_unit_tag_index: m_unit_tag_index.expect("Missing m_unit_tag_index from struct"),
                m_unit_tag_recycle: m_unit_tag_recycle
                    .expect("Missing m_unit_tag_recycle from struct"),
                m_unit_type_name: m_unit_type_name.expect("Missing m_unit_type_name from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSUpgradeEvent {
    pub m_player_id: u8,
    pub m_upgrade_type_name: Vec<u8>,
    pub m_count: i32,
}
impl ReplayTrackerSUpgradeEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_player_id(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_player_id) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_player_id);
        Ok((tail, u8::try_from(m_player_id).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_upgrade_type_name(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (tail, m_upgrade_type_name) = tagged_blob(input)?;
        tracing::debug!("res: {:?}", m_upgrade_type_name);
        Ok((tail, m_upgrade_type_name))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_count(input: &[u8]) -> IResult<&[u8], i32> {
        let (tail, m_count) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_count);
        Ok((tail, i32::try_from(m_count).unwrap()))
    }
    #[tracing::instrument(name="ReplayTrackerSUpgradeEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_player_id: Option<u8> = None;
        let mut m_upgrade_type_name: Option<Vec<u8>> = None;
        let mut m_count: Option<i32> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_player_id");
                    if m_player_id.is_none() {
                        let (new_tail, parsed_m_player_id) = Self::parse_m_player_id(tail)?;
                        tail = new_tail;
                        m_player_id = Some(parsed_m_player_id);
                        continue;
                    } else {
                        tracing::error!("Field m_player_id with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_upgrade_type_name");
                    if m_upgrade_type_name.is_none() {
                        let (new_tail, parsed_m_upgrade_type_name) =
                            Self::parse_m_upgrade_type_name(tail)?;
                        tail = new_tail;
                        m_upgrade_type_name = Some(parsed_m_upgrade_type_name);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_upgrade_type_name with tag 1 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!("Field [{i}]: tagged '2' for field m_count");
                    if m_count.is_none() {
                        let (new_tail, parsed_m_count) = Self::parse_m_count(tail)?;
                        tail = new_tail;
                        m_count = Some(parsed_m_count);
                        continue;
                    } else {
                        tracing::error!("Field m_count with tag 2 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_player_id: m_player_id.expect("Missing m_player_id from struct"),
                m_upgrade_type_name: m_upgrade_type_name
                    .expect("Missing m_upgrade_type_name from struct"),
                m_count: m_count.expect("Missing m_count from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSUnitInitEvent {
    pub m_unit_tag_index: u32,
    pub m_unit_tag_recycle: u32,
    pub m_unit_type_name: Vec<u8>,
    pub m_control_player_id: u8,
    pub m_upkeep_player_id: u8,
    pub m_x: u8,
    pub m_y: u8,
}
impl ReplayTrackerSUnitInitEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_index(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_index);
        Ok((tail, u32::try_from(m_unit_tag_index).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_recycle(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_recycle);
        Ok((tail, u32::try_from(m_unit_tag_recycle).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_type_name(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
        let (tail, m_unit_type_name) = tagged_blob(input)?;
        tracing::debug!("res: {:?}", m_unit_type_name);
        Ok((tail, m_unit_type_name))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_control_player_id(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_control_player_id) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_control_player_id);
        Ok((tail, u8::try_from(m_control_player_id).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_upkeep_player_id(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_upkeep_player_id) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_upkeep_player_id);
        Ok((tail, u8::try_from(m_upkeep_player_id).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_x(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_x) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_x);
        Ok((tail, u8::try_from(m_x).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_y(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_y) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_y);
        Ok((tail, u8::try_from(m_y).unwrap()))
    }
    #[tracing::instrument(name="ReplayTrackerSUnitInitEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_unit_tag_index: Option<u32> = None;
        let mut m_unit_tag_recycle: Option<u32> = None;
        let mut m_unit_type_name: Option<Vec<u8>> = None;
        let mut m_control_player_id: Option<u8> = None;
        let mut m_upkeep_player_id: Option<u8> = None;
        let mut m_x: Option<u8> = None;
        let mut m_y: Option<u8> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_unit_tag_index");
                    if m_unit_tag_index.is_none() {
                        let (new_tail, parsed_m_unit_tag_index) =
                            Self::parse_m_unit_tag_index(tail)?;
                        tail = new_tail;
                        m_unit_tag_index = Some(parsed_m_unit_tag_index);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_index with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_unit_tag_recycle");
                    if m_unit_tag_recycle.is_none() {
                        let (new_tail, parsed_m_unit_tag_recycle) =
                            Self::parse_m_unit_tag_recycle(tail)?;
                        tail = new_tail;
                        m_unit_tag_recycle = Some(parsed_m_unit_tag_recycle);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_recycle with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!("Field [{i}]: tagged '2' for field m_unit_type_name");
                    if m_unit_type_name.is_none() {
                        let (new_tail, parsed_m_unit_type_name) =
                            Self::parse_m_unit_type_name(tail)?;
                        tail = new_tail;
                        m_unit_type_name = Some(parsed_m_unit_type_name);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_type_name with tag 2 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                3 => {
                    tracing::debug!("Field [{i}]: tagged '3' for field m_control_player_id");
                    if m_control_player_id.is_none() {
                        let (new_tail, parsed_m_control_player_id) =
                            Self::parse_m_control_player_id(tail)?;
                        tail = new_tail;
                        m_control_player_id = Some(parsed_m_control_player_id);
                        continue;
                    } else {
                        tracing::error!(
                            "Field m_control_player_id with tag 3 was already provided"
                        );
                        panic!("Unhandled duplicate field.");
                    }
                }
                4 => {
                    tracing::debug!("Field [{i}]: tagged '4' for field m_upkeep_player_id");
                    if m_upkeep_player_id.is_none() {
                        let (new_tail, parsed_m_upkeep_player_id) =
                            Self::parse_m_upkeep_player_id(tail)?;
                        tail = new_tail;
                        m_upkeep_player_id = Some(parsed_m_upkeep_player_id);
                        continue;
                    } else {
                        tracing::error!("Field m_upkeep_player_id with tag 4 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                5 => {
                    tracing::debug!("Field [{i}]: tagged '5' for field m_x");
                    if m_x.is_none() {
                        let (new_tail, parsed_m_x) = Self::parse_m_x(tail)?;
                        tail = new_tail;
                        m_x = Some(parsed_m_x);
                        continue;
                    } else {
                        tracing::error!("Field m_x with tag 5 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                6 => {
                    tracing::debug!("Field [{i}]: tagged '6' for field m_y");
                    if m_y.is_none() {
                        let (new_tail, parsed_m_y) = Self::parse_m_y(tail)?;
                        tail = new_tail;
                        m_y = Some(parsed_m_y);
                        continue;
                    } else {
                        tracing::error!("Field m_y with tag 6 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_unit_tag_index: m_unit_tag_index.expect("Missing m_unit_tag_index from struct"),
                m_unit_tag_recycle: m_unit_tag_recycle
                    .expect("Missing m_unit_tag_recycle from struct"),
                m_unit_type_name: m_unit_type_name.expect("Missing m_unit_type_name from struct"),
                m_control_player_id: m_control_player_id
                    .expect("Missing m_control_player_id from struct"),
                m_upkeep_player_id: m_upkeep_player_id
                    .expect("Missing m_upkeep_player_id from struct"),
                m_x: m_x.expect("Missing m_x from struct"),
                m_y: m_y.expect("Missing m_y from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSUnitDoneEvent {
    pub m_unit_tag_index: u32,
    pub m_unit_tag_recycle: u32,
}
impl ReplayTrackerSUnitDoneEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_index(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_index);
        Ok((tail, u32::try_from(m_unit_tag_index).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_unit_tag_recycle(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_unit_tag_recycle);
        Ok((tail, u32::try_from(m_unit_tag_recycle).unwrap()))
    }
    #[tracing::instrument(name="ReplayTrackerSUnitDoneEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_unit_tag_index: Option<u32> = None;
        let mut m_unit_tag_recycle: Option<u32> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_unit_tag_index");
                    if m_unit_tag_index.is_none() {
                        let (new_tail, parsed_m_unit_tag_index) =
                            Self::parse_m_unit_tag_index(tail)?;
                        tail = new_tail;
                        m_unit_tag_index = Some(parsed_m_unit_tag_index);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_index with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_unit_tag_recycle");
                    if m_unit_tag_recycle.is_none() {
                        let (new_tail, parsed_m_unit_tag_recycle) =
                            Self::parse_m_unit_tag_recycle(tail)?;
                        tail = new_tail;
                        m_unit_tag_recycle = Some(parsed_m_unit_tag_recycle);
                        continue;
                    } else {
                        tracing::error!("Field m_unit_tag_recycle with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_unit_tag_index: m_unit_tag_index.expect("Missing m_unit_tag_index from struct"),
                m_unit_tag_recycle: m_unit_tag_recycle
                    .expect("Missing m_unit_tag_recycle from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSUnitPositionsEvent {
    pub m_first_unit_index: u32,
    pub m_items: Vec<i32>,
}
impl ReplayTrackerSUnitPositionsEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_first_unit_index(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_first_unit_index) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_first_unit_index);
        Ok((tail, u32::try_from(m_first_unit_index).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_items(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
        let (tail, _) = validate_array_tag(input)?;
        let (tail, array_length) = parse_vlq_int(tail)?;
        tracing::debug!("Reading array length: {array_length}");
        let (tail, array) = nom::multi::count(tagged_vlq_int, array_length as usize)(tail)?;
        let array = array
            .iter()
            .map(|val| <_>::try_from(*val).unwrap())
            .collect();
        Ok((tail, array))
    }
    #[tracing::instrument(name="ReplayTrackerSUnitPositionsEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_first_unit_index: Option<u32> = None;
        let mut m_items: Option<Vec<i32>> = None;
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_first_unit_index");
                    if m_first_unit_index.is_none() {
                        let (new_tail, parsed_m_first_unit_index) =
                            Self::parse_m_first_unit_index(tail)?;
                        tail = new_tail;
                        m_first_unit_index = Some(parsed_m_first_unit_index);
                        continue;
                    } else {
                        tracing::error!("Field m_first_unit_index with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_items");
                    if m_items.is_none() {
                        let (new_tail, parsed_m_items) = Self::parse_m_items(tail)?;
                        tail = new_tail;
                        m_items = Some(parsed_m_items);
                        continue;
                    } else {
                        tracing::error!("Field m_items with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_first_unit_index: m_first_unit_index
                    .expect("Missing m_first_unit_index from struct"),
                m_items: m_items.expect("Missing m_items from struct"),
            },
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ReplayTrackerSPlayerSetupEvent {
    pub m_player_id: u8,
    pub m_type: u32,
    pub m_user_id: Option<u32>,
    pub m_slot_id: Option<u32>,
}
impl ReplayTrackerSPlayerSetupEvent {
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_player_id(input: &[u8]) -> IResult<&[u8], u8> {
        let (tail, m_player_id) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_player_id);
        Ok((tail, u8::try_from(m_player_id).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_type(input: &[u8]) -> IResult<&[u8], u32> {
        let (tail, m_type) = tagged_vlq_int(input)?;
        tracing::debug!("res: {:?}", m_type);
        Ok((tail, u32::try_from(m_type).unwrap()))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_user_id(input: &[u8]) -> IResult<&[u8], Option<u32>> {
        let (tail, _) = validate_opt_tag(input)?;
        let (tail, is_provided) = nom::number::complete::u8(tail)?;
        let (tail, m_user_id) = if is_provided != 0 {
            let (tail, res) = tagged_vlq_int(tail)?;
            (tail, Some(<_>::try_from(res).unwrap()))
        } else {
            (tail, None)
        };
        tracing::debug!("res: {:?}", m_user_id);
        Ok((tail, m_user_id))
    }
    #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_m_slot_id(input: &[u8]) -> IResult<&[u8], Option<u32>> {
        let (tail, _) = validate_opt_tag(input)?;
        let (tail, is_provided) = nom::number::complete::u8(tail)?;
        let (tail, m_slot_id) = if is_provided != 0 {
            let (tail, res) = tagged_vlq_int(tail)?;
            (tail, Some(<_>::try_from(res).unwrap()))
        } else {
            (tail, None)
        };
        tracing::debug!("res: {:?}", m_slot_id);
        Ok((tail, m_slot_id))
    }
    #[tracing::instrument(name="ReplayTrackerSPlayerSetupEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (tail, _) = validate_struct_tag(input)?;
        let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
        let mut m_player_id: Option<u8> = None;
        let mut m_type: Option<u32> = None;
        let mut m_user_id: Option<Option<u32>> = Some(None);
        let mut m_slot_id: Option<Option<u32>> = Some(None);
        for i in 0..(struct_field_count as usize) {
            let (new_tail, field_tag) = parse_vlq_int(tail)?;
            tail = new_tail;
            match field_tag {
                0 => {
                    tracing::debug!("Field [{i}]: tagged '0' for field m_player_id");
                    if m_player_id.is_none() {
                        let (new_tail, parsed_m_player_id) = Self::parse_m_player_id(tail)?;
                        tail = new_tail;
                        m_player_id = Some(parsed_m_player_id);
                        continue;
                    } else {
                        tracing::error!("Field m_player_id with tag 0 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                1 => {
                    tracing::debug!("Field [{i}]: tagged '1' for field m_type");
                    if m_type.is_none() {
                        let (new_tail, parsed_m_type) = Self::parse_m_type(tail)?;
                        tail = new_tail;
                        m_type = Some(parsed_m_type);
                        continue;
                    } else {
                        tracing::error!("Field m_type with tag 1 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                2 => {
                    tracing::debug!("Field [{i}]: tagged '2' for field m_user_id");
                    if let Some(None) = m_user_id {
                        let (new_tail, parsed_m_user_id) = Self::parse_m_user_id(tail)?;
                        tail = new_tail;
                        m_user_id = Some(parsed_m_user_id);
                        continue;
                    } else {
                        tracing::error!("Field m_user_id with tag 2 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }
                3 => {
                    tracing::debug!("Field [{i}]: tagged '3' for field m_slot_id");
                    if let Some(None) = m_slot_id {
                        let (new_tail, parsed_m_slot_id) = Self::parse_m_slot_id(tail)?;
                        tail = new_tail;
                        m_slot_id = Some(parsed_m_slot_id);
                        continue;
                    } else {
                        tracing::error!("Field m_slot_id with tag 3 was already provided");
                        panic!("Unhandled duplicate field.");
                    }
                }

                _ => {
                    tracing::error!("Unknown tag {field_tag}");
                    panic!("Unknown tag {field_tag}");
                }
            }
        }
        Ok((
            tail,
            Self {
                m_player_id: m_player_id.expect("Missing m_player_id from struct"),
                m_type: m_type.expect("Missing m_type from struct"),
                m_user_id: m_user_id.expect("Missing m_user_id from struct"),
                m_slot_id: m_slot_id.expect("Missing m_slot_id from struct"),
            },
        ))
    }
}
