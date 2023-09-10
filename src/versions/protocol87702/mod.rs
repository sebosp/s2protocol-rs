pub mod events;
pub use events::*;
pub mod byte_aligned {
    //! Generated code from source: ../s2protocol/json/protocol87702.json
    use crate::*;
    use nom_mpq::parser::peek_hex;

    #[derive(Debug, PartialEq, Clone)]
    pub enum SVarUint32 {
        MUint6(u8),
        MUint14(u32),
        MUint22(u32),
        MUint32(u32),
    }
    impl SVarUint32 {
        #[tracing::instrument(name="87702::SVarUint32::ChoiceType::parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_choice_tag(input)?;
            let (tail, variant_tag) = parse_vlq_int(tail)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant tagged '0' for MUint6");
                    let (tail, res) = tagged_vlq_int(tail)?;
                    Ok((tail, Self::MUint6(u8::try_from(res)?)))
                }
                1 => {
                    tracing::debug!("Variant tagged '1' for MUint14");
                    let (tail, res) = tagged_vlq_int(tail)?;
                    Ok((tail, Self::MUint14(u32::try_from(res)?)))
                }
                2 => {
                    tracing::debug!("Variant tagged '2' for MUint22");
                    let (tail, res) = tagged_vlq_int(tail)?;
                    Ok((tail, Self::MUint22(u32::try_from(res)?)))
                }
                3 => {
                    tracing::debug!("Variant tagged '3' for MUint32");
                    let (tail, res) = tagged_vlq_int(tail)?;
                    Ok((tail, Self::MUint32(u32::try_from(res)?)))
                }

                _ => {
                    tracing::error!("Unknown variant for tag {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum EObserve {
        ENone,
        ESpectator,
        EReferee,
    }
    impl EObserve {
        #[tracing::instrument(name="87702::EObserve::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_int_tag(input)?;
            let (tail, variant_tag) = parse_vlq_int(tail)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ENone for value '0'");
                    Ok((tail, Self::ENone))
                }
                1 => {
                    tracing::debug!("Variant ESpectator for value '1'");
                    Ok((tail, Self::ESpectator))
                }
                2 => {
                    tracing::debug!("Variant EReferee for value '2'");
                    Ok((tail, Self::EReferee))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
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
        pub fn parse_m_flags(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_flags) = tagged_vlq_int(input)?;

            tracing::debug!("m_flags: {:?}", m_flags);
            Ok((tail, u8::try_from(m_flags)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_major(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_major) = tagged_vlq_int(input)?;

            tracing::debug!("m_major: {:?}", m_major);
            Ok((tail, u8::try_from(m_major)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_minor(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_minor) = tagged_vlq_int(input)?;

            tracing::debug!("m_minor: {:?}", m_minor);
            Ok((tail, u8::try_from(m_minor)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_revision(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_revision) = tagged_vlq_int(input)?;

            tracing::debug!("m_revision: {:?}", m_revision);
            Ok((tail, u8::try_from(m_revision)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_build(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_build) = tagged_vlq_int(input)?;

            tracing::debug!("m_build: {:?}", m_build);
            Ok((tail, u32::try_from(m_build)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_base_build(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_base_build) = tagged_vlq_int(input)?;

            tracing::debug!("m_base_build: {:?}", m_base_build);
            Ok((tail, u32::try_from(m_base_build)?))
        }
        #[tracing::instrument(name="87702::byte_aligned::SVersion::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_flags"), 0));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_major"), 1));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_minor"), 2));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_revision"),
                                3,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_build"), 4));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_base_build"),
                                5,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            let res = Self {
                m_flags: ok_or_return_missing_field_err!(m_flags),
                m_major: ok_or_return_missing_field_err!(m_major),
                m_minor: ok_or_return_missing_field_err!(m_minor),
                m_revision: ok_or_return_missing_field_err!(m_revision),
                m_build: ok_or_return_missing_field_err!(m_build),
                m_base_build: ok_or_return_missing_field_err!(m_base_build),
            };
            Ok((tail, res))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Smd5 {
        pub m_data_deprecated: Option<Vec<u8>>,
        pub m_data: Vec<u8>,
    }
    impl Smd5 {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_data_deprecated(input: &[u8]) -> S2ProtoResult<&[u8], Option<Vec<u8>>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_data_deprecated) = if is_provided != 0 {
                let (tail, _) = validate_array_tag(tail)?;
                let (mut tail, array_length) = parse_vlq_int(tail)?;
                tracing::debug!("Reading array length: {array_length}");

                let array_length = array_length as usize;
                let max_initial_capacity =
                    MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<u8>().max(1);
                let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
                for _ in 0..array_length {
                    let (new_tail, data) = tagged_vlq_int(tail)?;
                    tail = new_tail;
                    array.push(<_>::try_from(data)?);
                }
                (tail, Some(array))
            } else {
                (tail, None)
            };
            tracing::debug!("m_data_deprecated: {:?}", m_data_deprecated);
            Ok((tail, m_data_deprecated))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_data(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_data) = tagged_blob(input)?;

            tracing::debug!("m_data: {:?}", m_data);
            Ok((tail, m_data))
        }
        #[tracing::instrument(name="87702::byte_aligned::Smd5::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            tracing::error!(
                                "Field m_data_deprecated with tag 0 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_data_deprecated"),
                                0,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_data"), 1));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_data_deprecated: ok_or_return_missing_field_err!(m_data_deprecated),
                    m_data: ok_or_return_missing_field_err!(m_data),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEGameSpeed {
        ESlower,
        ESlow,
        ENormal,
        EFast,
        EFaster,
    }
    impl GameEGameSpeed {
        #[tracing::instrument(name="87702::GameEGameSpeed::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_int_tag(input)?;
            let (tail, variant_tag) = parse_vlq_int(tail)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ESlower for value '0'");
                    Ok((tail, Self::ESlower))
                }
                1 => {
                    tracing::debug!("Variant ESlow for value '1'");
                    Ok((tail, Self::ESlow))
                }
                2 => {
                    tracing::debug!("Variant ENormal for value '2'");
                    Ok((tail, Self::ENormal))
                }
                3 => {
                    tracing::debug!("Variant EFast for value '3'");
                    Ok((tail, Self::EFast))
                }
                4 => {
                    tracing::debug!("Variant EFaster for value '4'");
                    Ok((tail, Self::EFaster))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSThumbnail {
        pub m_file: Vec<u8>,
    }
    impl GameSThumbnail {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_file(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_file) = tagged_blob(input)?;

            tracing::debug!("m_file: {:?}", m_file);
            Ok((tail, m_file))
        }
        #[tracing::instrument(name="87702::byte_aligned::GameSThumbnail::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_struct_tag(input)?;
            let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
            let mut m_file: Option<Vec<u8>> = None;
            for i in 0..(struct_field_count as usize) {
                let (new_tail, field_tag) = parse_vlq_int(tail)?;
                tail = new_tail;
                match field_tag {
                    0 => {
                        tracing::debug!("Field [{i}]: tagged '0' for field m_file");
                        if m_file.is_none() {
                            let (new_tail, parsed_m_file) = Self::parse_m_file(tail)?;
                            tail = new_tail;
                            m_file = Some(parsed_m_file);
                            continue;
                        } else {
                            tracing::error!("Field m_file with tag 0 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_file"), 0));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_file: ok_or_return_missing_field_err!(m_file),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSColor {
        pub m_a: u8,
        pub m_r: u8,
        pub m_g: u8,
        pub m_b: u8,
    }
    impl GameSColor {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_a(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_a) = tagged_vlq_int(input)?;

            tracing::debug!("m_a: {:?}", m_a);
            Ok((tail, u8::try_from(m_a)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_r(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_r) = tagged_vlq_int(input)?;

            tracing::debug!("m_r: {:?}", m_r);
            Ok((tail, u8::try_from(m_r)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_g(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_g) = tagged_vlq_int(input)?;

            tracing::debug!("m_g: {:?}", m_g);
            Ok((tail, u8::try_from(m_g)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_b(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_b) = tagged_vlq_int(input)?;

            tracing::debug!("m_b: {:?}", m_b);
            Ok((tail, u8::try_from(m_b)?))
        }
        #[tracing::instrument(name="87702::byte_aligned::GameSColor::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_struct_tag(input)?;
            let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
            let mut m_a: Option<u8> = None;
            let mut m_r: Option<u8> = None;
            let mut m_g: Option<u8> = None;
            let mut m_b: Option<u8> = None;
            for i in 0..(struct_field_count as usize) {
                let (new_tail, field_tag) = parse_vlq_int(tail)?;
                tail = new_tail;
                match field_tag {
                    0 => {
                        tracing::debug!("Field [{i}]: tagged '0' for field m_a");
                        if m_a.is_none() {
                            let (new_tail, parsed_m_a) = Self::parse_m_a(tail)?;
                            tail = new_tail;
                            m_a = Some(parsed_m_a);
                            continue;
                        } else {
                            tracing::error!("Field m_a with tag 0 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_a"), 0));
                        }
                    }
                    1 => {
                        tracing::debug!("Field [{i}]: tagged '1' for field m_r");
                        if m_r.is_none() {
                            let (new_tail, parsed_m_r) = Self::parse_m_r(tail)?;
                            tail = new_tail;
                            m_r = Some(parsed_m_r);
                            continue;
                        } else {
                            tracing::error!("Field m_r with tag 1 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_r"), 1));
                        }
                    }
                    2 => {
                        tracing::debug!("Field [{i}]: tagged '2' for field m_g");
                        if m_g.is_none() {
                            let (new_tail, parsed_m_g) = Self::parse_m_g(tail)?;
                            tail = new_tail;
                            m_g = Some(parsed_m_g);
                            continue;
                        } else {
                            tracing::error!("Field m_g with tag 2 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_g"), 2));
                        }
                    }
                    3 => {
                        tracing::debug!("Field [{i}]: tagged '3' for field m_b");
                        if m_b.is_none() {
                            let (new_tail, parsed_m_b) = Self::parse_m_b(tail)?;
                            tail = new_tail;
                            m_b = Some(parsed_m_b);
                            continue;
                        } else {
                            tracing::error!("Field m_b with tag 3 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_b"), 3));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_a: ok_or_return_missing_field_err!(m_a),
                    m_r: ok_or_return_missing_field_err!(m_r),
                    m_g: ok_or_return_missing_field_err!(m_g),
                    m_b: ok_or_return_missing_field_err!(m_b),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEResultDetails {
        EUndecided,
        EWin,
        ELoss,
        ETie,
    }
    impl GameEResultDetails {
        #[tracing::instrument(name="87702::GameEResultDetails::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_int_tag(input)?;
            let (tail, variant_tag) = parse_vlq_int(tail)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EUndecided for value '0'");
                    Ok((tail, Self::EUndecided))
                }
                1 => {
                    tracing::debug!("Variant EWin for value '1'");
                    Ok((tail, Self::EWin))
                }
                2 => {
                    tracing::debug!("Variant ELoss for value '2'");
                    Ok((tail, Self::ELoss))
                }
                3 => {
                    tracing::debug!("Variant ETie for value '3'");
                    Ok((tail, Self::ETie))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSToonNameDetails {
        pub m_region: u8,
        pub m_program_id: u32,
        pub m_realm: u32,
        pub m_name: Vec<u8>,
        pub m_id: u64,
    }
    impl GameSToonNameDetails {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_region(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_region) = tagged_vlq_int(input)?;

            tracing::debug!("m_region: {:?}", m_region);
            Ok((tail, u8::try_from(m_region)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_program_id(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_program_id) = tagged_fourcc(input)?;

            tracing::debug!("m_program_id: {:?}", m_program_id);
            Ok((tail, m_program_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_realm(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_realm) = tagged_vlq_int(input)?;

            tracing::debug!("m_realm: {:?}", m_realm);
            Ok((tail, u32::try_from(m_realm)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_name(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_name) = tagged_blob(input)?;

            tracing::debug!("m_name: {:?}", str::from_utf8(&m_name));
            Ok((tail, m_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_id(input: &[u8]) -> S2ProtoResult<&[u8], u64> {
            let (tail, m_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_id: {:?}", m_id);
            Ok((tail, u64::try_from(m_id)?))
        }
        #[tracing::instrument(name="87702::byte_aligned::GameSToonNameDetails::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_struct_tag(input)?;
            let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
            let mut m_region: Option<u8> = None;
            let mut m_program_id: Option<u32> = None;
            let mut m_realm: Option<u32> = None;
            let mut m_name: Option<Vec<u8>> = None;
            let mut m_id: Option<u64> = None;
            for i in 0..(struct_field_count as usize) {
                let (new_tail, field_tag) = parse_vlq_int(tail)?;
                tail = new_tail;
                match field_tag {
                    0 => {
                        tracing::debug!("Field [{i}]: tagged '0' for field m_region");
                        if m_region.is_none() {
                            let (new_tail, parsed_m_region) = Self::parse_m_region(tail)?;
                            tail = new_tail;
                            m_region = Some(parsed_m_region);
                            continue;
                        } else {
                            tracing::error!("Field m_region with tag 0 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_region"), 0));
                        }
                    }
                    1 => {
                        tracing::debug!("Field [{i}]: tagged '1' for field m_program_id");
                        if m_program_id.is_none() {
                            let (new_tail, parsed_m_program_id) = Self::parse_m_program_id(tail)?;
                            tail = new_tail;
                            m_program_id = Some(parsed_m_program_id);
                            continue;
                        } else {
                            tracing::error!("Field m_program_id with tag 1 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_program_id"),
                                1,
                            ));
                        }
                    }
                    2 => {
                        tracing::debug!("Field [{i}]: tagged '2' for field m_realm");
                        if m_realm.is_none() {
                            let (new_tail, parsed_m_realm) = Self::parse_m_realm(tail)?;
                            tail = new_tail;
                            m_realm = Some(parsed_m_realm);
                            continue;
                        } else {
                            tracing::error!("Field m_realm with tag 2 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_realm"), 2));
                        }
                    }
                    3 => {
                        tracing::debug!("Field [{i}]: tagged '3' for field m_name");
                        if m_name.is_none() {
                            let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                            tail = new_tail;
                            m_name = Some(parsed_m_name);
                            continue;
                        } else {
                            tracing::error!("Field m_name with tag 3 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_name"), 3));
                        }
                    }
                    4 => {
                        tracing::debug!("Field [{i}]: tagged '4' for field m_id");
                        if m_id.is_none() {
                            let (new_tail, parsed_m_id) = Self::parse_m_id(tail)?;
                            tail = new_tail;
                            m_id = Some(parsed_m_id);
                            continue;
                        } else {
                            tracing::error!("Field m_id with tag 4 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_id"), 4));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_region: ok_or_return_missing_field_err!(m_region),
                    m_program_id: ok_or_return_missing_field_err!(m_program_id),
                    m_realm: ok_or_return_missing_field_err!(m_realm),
                    m_name: m_name.unwrap_or(b""[..].to_vec()),
                    m_id: ok_or_return_missing_field_err!(m_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPlayerDetails {
        pub m_name: Vec<u8>,
        pub m_toon: GameSToonNameDetails,
        pub m_race: Vec<u8>,
        pub m_color: GameSColor,
        pub m_control: u8,
        pub m_team_id: u8,
        pub m_handicap: u32,
        pub m_observe: EObserve,
        pub m_result: GameEResultDetails,
        pub m_working_set_slot_id: Option<u8>,
        pub m_hero: Vec<u8>,
    }
    impl GameSPlayerDetails {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_name(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_name) = tagged_blob(input)?;

            tracing::debug!("m_name: {:?}", str::from_utf8(&m_name));
            Ok((tail, m_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_toon(input: &[u8]) -> S2ProtoResult<&[u8], GameSToonNameDetails> {
            let (tail, m_toon) = GameSToonNameDetails::parse(input)?;

            tracing::debug!("m_toon: {:?}", m_toon);
            Ok((tail, m_toon))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_race(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_race) = tagged_blob(input)?;

            tracing::debug!("m_race: {:?}", m_race);
            Ok((tail, m_race))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_color(input: &[u8]) -> S2ProtoResult<&[u8], GameSColor> {
            let (tail, m_color) = GameSColor::parse(input)?;

            tracing::debug!("m_color: {:?}", m_color);
            Ok((tail, m_color))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_control(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_control) = tagged_vlq_int(input)?;

            tracing::debug!("m_control: {:?}", m_control);
            Ok((tail, u8::try_from(m_control)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_team_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_team_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_team_id: {:?}", m_team_id);
            Ok((tail, u8::try_from(m_team_id)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_handicap(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_handicap) = tagged_vlq_int(input)?;

            tracing::debug!("m_handicap: {:?}", m_handicap);
            Ok((tail, u32::try_from(m_handicap)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_observe(input: &[u8]) -> S2ProtoResult<&[u8], EObserve> {
            let (tail, m_observe) = EObserve::parse(input)?;

            tracing::debug!("m_observe: {:?}", m_observe);
            Ok((tail, m_observe))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_result(input: &[u8]) -> S2ProtoResult<&[u8], GameEResultDetails> {
            let (tail, m_result) = GameEResultDetails::parse(input)?;

            tracing::debug!("m_result: {:?}", m_result);
            Ok((tail, m_result))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_working_set_slot_id(input: &[u8]) -> S2ProtoResult<&[u8], Option<u8>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_working_set_slot_id) = if is_provided != 0 {
                let (tail, res) = tagged_vlq_int(tail)?;
                (tail, Some(<_>::try_from(res)?))
            } else {
                (tail, None)
            };
            tracing::debug!("m_working_set_slot_id: {:?}", m_working_set_slot_id);
            Ok((tail, m_working_set_slot_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_hero(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_hero) = tagged_blob(input)?;

            tracing::debug!("m_hero: {:?}", m_hero);
            Ok((tail, m_hero))
        }
        #[tracing::instrument(name="87702::byte_aligned::GameSPlayerDetails::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_struct_tag(input)?;
            let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
            let mut m_name: Option<Vec<u8>> = None;
            let mut m_toon: Option<GameSToonNameDetails> = None;
            let mut m_race: Option<Vec<u8>> = None;
            let mut m_color: Option<GameSColor> = None;
            let mut m_control: Option<u8> = None;
            let mut m_team_id: Option<u8> = None;
            let mut m_handicap: Option<u32> = None;
            let mut m_observe: Option<EObserve> = None;
            let mut m_result: Option<GameEResultDetails> = None;
            let mut m_working_set_slot_id: Option<Option<u8>> = Some(None);
            let mut m_hero: Option<Vec<u8>> = None;
            for i in 0..(struct_field_count as usize) {
                let (new_tail, field_tag) = parse_vlq_int(tail)?;
                tail = new_tail;
                match field_tag {
                    0 => {
                        tracing::debug!("Field [{i}]: tagged '0' for field m_name");
                        if m_name.is_none() {
                            let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                            tail = new_tail;
                            m_name = Some(parsed_m_name);
                            continue;
                        } else {
                            tracing::error!("Field m_name with tag 0 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_name"), 0));
                        }
                    }
                    1 => {
                        tracing::debug!("Field [{i}]: tagged '1' for field m_toon");
                        if m_toon.is_none() {
                            let (new_tail, parsed_m_toon) = Self::parse_m_toon(tail)?;
                            tail = new_tail;
                            m_toon = Some(parsed_m_toon);
                            continue;
                        } else {
                            tracing::error!("Field m_toon with tag 1 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_toon"), 1));
                        }
                    }
                    2 => {
                        tracing::debug!("Field [{i}]: tagged '2' for field m_race");
                        if m_race.is_none() {
                            let (new_tail, parsed_m_race) = Self::parse_m_race(tail)?;
                            tail = new_tail;
                            m_race = Some(parsed_m_race);
                            continue;
                        } else {
                            tracing::error!("Field m_race with tag 2 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_race"), 2));
                        }
                    }
                    3 => {
                        tracing::debug!("Field [{i}]: tagged '3' for field m_color");
                        if m_color.is_none() {
                            let (new_tail, parsed_m_color) = Self::parse_m_color(tail)?;
                            tail = new_tail;
                            m_color = Some(parsed_m_color);
                            continue;
                        } else {
                            tracing::error!("Field m_color with tag 3 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_color"), 3));
                        }
                    }
                    4 => {
                        tracing::debug!("Field [{i}]: tagged '4' for field m_control");
                        if m_control.is_none() {
                            let (new_tail, parsed_m_control) = Self::parse_m_control(tail)?;
                            tail = new_tail;
                            m_control = Some(parsed_m_control);
                            continue;
                        } else {
                            tracing::error!("Field m_control with tag 4 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_control"),
                                4,
                            ));
                        }
                    }
                    5 => {
                        tracing::debug!("Field [{i}]: tagged '5' for field m_team_id");
                        if m_team_id.is_none() {
                            let (new_tail, parsed_m_team_id) = Self::parse_m_team_id(tail)?;
                            tail = new_tail;
                            m_team_id = Some(parsed_m_team_id);
                            continue;
                        } else {
                            tracing::error!("Field m_team_id with tag 5 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_team_id"),
                                5,
                            ));
                        }
                    }
                    6 => {
                        tracing::debug!("Field [{i}]: tagged '6' for field m_handicap");
                        if m_handicap.is_none() {
                            let (new_tail, parsed_m_handicap) = Self::parse_m_handicap(tail)?;
                            tail = new_tail;
                            m_handicap = Some(parsed_m_handicap);
                            continue;
                        } else {
                            tracing::error!("Field m_handicap with tag 6 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_handicap"),
                                6,
                            ));
                        }
                    }
                    7 => {
                        tracing::debug!("Field [{i}]: tagged '7' for field m_observe");
                        if m_observe.is_none() {
                            let (new_tail, parsed_m_observe) = Self::parse_m_observe(tail)?;
                            tail = new_tail;
                            m_observe = Some(parsed_m_observe);
                            continue;
                        } else {
                            tracing::error!("Field m_observe with tag 7 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_observe"),
                                7,
                            ));
                        }
                    }
                    8 => {
                        tracing::debug!("Field [{i}]: tagged '8' for field m_result");
                        if m_result.is_none() {
                            let (new_tail, parsed_m_result) = Self::parse_m_result(tail)?;
                            tail = new_tail;
                            m_result = Some(parsed_m_result);
                            continue;
                        } else {
                            tracing::error!("Field m_result with tag 8 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_result"), 8));
                        }
                    }
                    9 => {
                        tracing::debug!("Field [{i}]: tagged '9' for field m_working_set_slot_id");
                        if let Some(None) = m_working_set_slot_id {
                            let (new_tail, parsed_m_working_set_slot_id) =
                                Self::parse_m_working_set_slot_id(tail)?;
                            tail = new_tail;
                            m_working_set_slot_id = Some(parsed_m_working_set_slot_id);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_working_set_slot_id with tag 9 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_working_set_slot_id"),
                                9,
                            ));
                        }
                    }
                    10 => {
                        tracing::debug!("Field [{i}]: tagged '10' for field m_hero");
                        if m_hero.is_none() {
                            let (new_tail, parsed_m_hero) = Self::parse_m_hero(tail)?;
                            tail = new_tail;
                            m_hero = Some(parsed_m_hero);
                            continue;
                        } else {
                            tracing::error!("Field m_hero with tag 10 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_hero"), 10));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_name: ok_or_return_missing_field_err!(m_name),
                    m_toon: ok_or_return_missing_field_err!(m_toon),
                    m_race: ok_or_return_missing_field_err!(m_race),
                    m_color: ok_or_return_missing_field_err!(m_color),
                    m_control: ok_or_return_missing_field_err!(m_control),
                    m_team_id: ok_or_return_missing_field_err!(m_team_id),
                    m_handicap: ok_or_return_missing_field_err!(m_handicap),
                    m_observe: ok_or_return_missing_field_err!(m_observe),
                    m_result: ok_or_return_missing_field_err!(m_result),
                    m_working_set_slot_id: ok_or_return_missing_field_err!(m_working_set_slot_id),
                    m_hero: ok_or_return_missing_field_err!(m_hero),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCPlayerDetailsArray {
        pub value: Vec<GameSPlayerDetails>,
    }
    impl GameCPlayerDetailsArray {
        #[tracing::instrument(name="87702::GameCPlayerDetailsArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_array_tag(input)?;
            let (mut tail, array_length) = parse_vlq_int(tail)?;
            tracing::debug!("Reading array length: {array_length}");
            // compat_count(GameSPlayerDetails::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameSPlayerDetails>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameSPlayerDetails::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSDetails {
        pub m_player_list: Option<Vec<GameSPlayerDetails>>,
        pub m_title: Vec<u8>,
        pub m_difficulty: Vec<u8>,
        pub m_thumbnail: GameSThumbnail,
        pub m_is_blizzard_map: bool,
        pub m_time_utc: i64,
        pub m_time_local_offset: i64,
        pub m_restart_as_transition_map: Option<bool>,
        pub m_disable_recover_game: bool,
        pub m_description: Vec<u8>,
        pub m_image_file_path: Vec<u8>,
        pub m_campaign_index: u8,
        pub m_map_file_name: Vec<u8>,
        pub m_cache_handles: Option<Vec<Vec<u8>>>,
        pub m_mini_save: bool,
        pub m_game_speed: GameEGameSpeed,
        pub m_default_difficulty: u32,
        pub m_mod_paths: Option<Vec<Vec<u8>>>,
    }
    impl GameSDetails {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_player_list(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], Option<Vec<GameSPlayerDetails>>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_player_list) = if is_provided != 0 {
                let (tail, _) = validate_array_tag(tail)?;
                let (mut tail, array_length) = parse_vlq_int(tail)?;
                tracing::debug!("Reading array length: {array_length}");
                // compat_count(GameSPlayerDetails::parse, array_length as usize)(tail)?;
                let array_length = array_length as usize;
                tracing::debug!("Reading array length: {array_length}");
                let max_initial_capacity =
                    MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameSPlayerDetails>().max(1);
                let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
                for _ in 0..array_length {
                    let (new_tail, data) = GameSPlayerDetails::parse(tail)?;
                    tail = new_tail;
                    array.push(data);
                }
                (tail, Some(array))
            } else {
                (tail, None)
            };
            tracing::debug!("m_player_list: {:?}", m_player_list);
            Ok((tail, m_player_list))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_title(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_title) = tagged_blob(input)?;

            tracing::debug!("m_title: {:?}", m_title);
            Ok((tail, m_title))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_difficulty(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_difficulty) = tagged_blob(input)?;

            tracing::debug!("m_difficulty: {:?}", m_difficulty);
            Ok((tail, m_difficulty))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_thumbnail(input: &[u8]) -> S2ProtoResult<&[u8], GameSThumbnail> {
            let (tail, m_thumbnail) = GameSThumbnail::parse(input)?;

            tracing::debug!("m_thumbnail: {:?}", m_thumbnail);
            Ok((tail, m_thumbnail))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_is_blizzard_map(input: &[u8]) -> S2ProtoResult<&[u8], bool> {
            let (tail, m_is_blizzard_map) = tagged_bool(input)?;

            tracing::debug!("m_is_blizzard_map: {:?}", m_is_blizzard_map);
            Ok((tail, m_is_blizzard_map))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_time_utc(input: &[u8]) -> S2ProtoResult<&[u8], i64> {
            let (tail, m_time_utc) = tagged_vlq_int(input)?;

            tracing::debug!("m_time_utc: {:?}", m_time_utc);
            Ok((tail, m_time_utc))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_time_local_offset(input: &[u8]) -> S2ProtoResult<&[u8], i64> {
            let (tail, m_time_local_offset) = tagged_vlq_int(input)?;

            tracing::debug!("m_time_local_offset: {:?}", m_time_local_offset);
            Ok((tail, m_time_local_offset))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_restart_as_transition_map(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], Option<bool>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_restart_as_transition_map) = if is_provided != 0 {
                let (tail, res) = tagged_bool(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!(
                "m_restart_as_transition_map: {:?}",
                m_restart_as_transition_map
            );
            Ok((tail, m_restart_as_transition_map))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_disable_recover_game(input: &[u8]) -> S2ProtoResult<&[u8], bool> {
            let (tail, m_disable_recover_game) = tagged_bool(input)?;

            tracing::debug!("m_disable_recover_game: {:?}", m_disable_recover_game);
            Ok((tail, m_disable_recover_game))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_description(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_description) = tagged_blob(input)?;

            tracing::debug!("m_description: {:?}", m_description);
            Ok((tail, m_description))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_image_file_path(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_image_file_path) = tagged_blob(input)?;

            tracing::debug!("m_image_file_path: {:?}", m_image_file_path);
            Ok((tail, m_image_file_path))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_campaign_index(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_campaign_index) = tagged_vlq_int(input)?;

            tracing::debug!("m_campaign_index: {:?}", m_campaign_index);
            Ok((tail, u8::try_from(m_campaign_index)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_map_file_name(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_map_file_name) = tagged_blob(input)?;

            tracing::debug!("m_map_file_name: {:?}", str::from_utf8(&m_map_file_name));
            Ok((tail, m_map_file_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_cache_handles(input: &[u8]) -> S2ProtoResult<&[u8], Option<Vec<Vec<u8>>>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_cache_handles) = if is_provided != 0 {
                let (tail, _) = validate_array_tag(tail)?;
                let (mut tail, array_length) = parse_vlq_int(tail)?;
                tracing::debug!("Reading array length: {array_length}");
                // compat_count(tagged_blob, array_length as usize)(tail)?;
                let array_length = array_length as usize;
                tracing::debug!("Reading array length: {array_length}");
                let max_initial_capacity =
                    MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<u8>().max(1);
                let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
                for _ in 0..array_length {
                    let (new_tail, data) = tagged_blob(tail)?;
                    tail = new_tail;
                    array.push(data);
                }
                (tail, Some(array))
            } else {
                (tail, None)
            };
            tracing::debug!("m_cache_handles: {:?}", m_cache_handles);
            Ok((tail, m_cache_handles))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_mini_save(input: &[u8]) -> S2ProtoResult<&[u8], bool> {
            let (tail, m_mini_save) = tagged_bool(input)?;

            tracing::debug!("m_mini_save: {:?}", m_mini_save);
            Ok((tail, m_mini_save))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_game_speed(input: &[u8]) -> S2ProtoResult<&[u8], GameEGameSpeed> {
            let (tail, m_game_speed) = GameEGameSpeed::parse(input)?;

            tracing::debug!("m_game_speed: {:?}", m_game_speed);
            Ok((tail, m_game_speed))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_default_difficulty(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_default_difficulty) = tagged_vlq_int(input)?;

            tracing::debug!("m_default_difficulty: {:?}", m_default_difficulty);
            Ok((tail, u32::try_from(m_default_difficulty)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_mod_paths(input: &[u8]) -> S2ProtoResult<&[u8], Option<Vec<Vec<u8>>>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_mod_paths) = if is_provided != 0 {
                let (tail, _) = validate_array_tag(tail)?;
                let (mut tail, array_length) = parse_vlq_int(tail)?;
                tracing::debug!("Reading array length: {array_length}");
                // compat_count(tagged_blob, array_length as usize)(tail)?;
                let array_length = array_length as usize;
                tracing::debug!("Reading array length: {array_length}");
                let max_initial_capacity =
                    MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<u8>().max(1);
                let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
                for _ in 0..array_length {
                    let (new_tail, data) = tagged_blob(tail)?;
                    tail = new_tail;
                    array.push(data);
                }
                (tail, Some(array))
            } else {
                (tail, None)
            };
            tracing::debug!("m_mod_paths: {:?}", m_mod_paths);
            Ok((tail, m_mod_paths))
        }
        #[tracing::instrument(name="87702::byte_aligned::GameSDetails::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
            let (tail, _) = validate_struct_tag(input)?;
            let (mut tail, struct_field_count) = parse_vlq_int(tail)?;
            let mut m_player_list: Option<Option<Vec<GameSPlayerDetails>>> = Some(None);
            let mut m_title: Option<Vec<u8>> = None;
            let mut m_difficulty: Option<Vec<u8>> = None;
            let mut m_thumbnail: Option<GameSThumbnail> = None;
            let mut m_is_blizzard_map: Option<bool> = None;
            let mut m_time_utc: Option<i64> = None;
            let mut m_time_local_offset: Option<i64> = None;
            let mut m_restart_as_transition_map: Option<Option<bool>> = Some(None);
            let mut m_disable_recover_game: Option<bool> = None;
            let mut m_description: Option<Vec<u8>> = None;
            let mut m_image_file_path: Option<Vec<u8>> = None;
            let mut m_campaign_index: Option<u8> = None;
            let mut m_map_file_name: Option<Vec<u8>> = None;
            let mut m_cache_handles: Option<Option<Vec<Vec<u8>>>> = Some(None);
            let mut m_mini_save: Option<bool> = None;
            let mut m_game_speed: Option<GameEGameSpeed> = None;
            let mut m_default_difficulty: Option<u32> = None;
            let mut m_mod_paths: Option<Option<Vec<Vec<u8>>>> = Some(None);
            for i in 0..(struct_field_count as usize) {
                let (new_tail, field_tag) = parse_vlq_int(tail)?;
                tail = new_tail;
                match field_tag {
                    0 => {
                        tracing::debug!("Field [{i}]: tagged '0' for field m_player_list");
                        if let Some(None) = m_player_list {
                            let (new_tail, parsed_m_player_list) = Self::parse_m_player_list(tail)?;
                            tail = new_tail;
                            m_player_list = Some(parsed_m_player_list);
                            continue;
                        } else {
                            tracing::error!("Field m_player_list with tag 0 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_player_list"),
                                0,
                            ));
                        }
                    }
                    1 => {
                        tracing::debug!("Field [{i}]: tagged '1' for field m_title");
                        if m_title.is_none() {
                            let (new_tail, parsed_m_title) = Self::parse_m_title(tail)?;
                            tail = new_tail;
                            m_title = Some(parsed_m_title);
                            continue;
                        } else {
                            tracing::error!("Field m_title with tag 1 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_title"), 1));
                        }
                    }
                    2 => {
                        tracing::debug!("Field [{i}]: tagged '2' for field m_difficulty");
                        if m_difficulty.is_none() {
                            let (new_tail, parsed_m_difficulty) = Self::parse_m_difficulty(tail)?;
                            tail = new_tail;
                            m_difficulty = Some(parsed_m_difficulty);
                            continue;
                        } else {
                            tracing::error!("Field m_difficulty with tag 2 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_difficulty"),
                                2,
                            ));
                        }
                    }
                    3 => {
                        tracing::debug!("Field [{i}]: tagged '3' for field m_thumbnail");
                        if m_thumbnail.is_none() {
                            let (new_tail, parsed_m_thumbnail) = Self::parse_m_thumbnail(tail)?;
                            tail = new_tail;
                            m_thumbnail = Some(parsed_m_thumbnail);
                            continue;
                        } else {
                            tracing::error!("Field m_thumbnail with tag 3 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_thumbnail"),
                                3,
                            ));
                        }
                    }
                    4 => {
                        tracing::debug!("Field [{i}]: tagged '4' for field m_is_blizzard_map");
                        if m_is_blizzard_map.is_none() {
                            let (new_tail, parsed_m_is_blizzard_map) =
                                Self::parse_m_is_blizzard_map(tail)?;
                            tail = new_tail;
                            m_is_blizzard_map = Some(parsed_m_is_blizzard_map);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_is_blizzard_map with tag 4 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_is_blizzard_map"),
                                4,
                            ));
                        }
                    }
                    5 => {
                        tracing::debug!("Field [{i}]: tagged '5' for field m_time_utc");
                        if m_time_utc.is_none() {
                            let (new_tail, parsed_m_time_utc) = Self::parse_m_time_utc(tail)?;
                            tail = new_tail;
                            m_time_utc = Some(parsed_m_time_utc);
                            continue;
                        } else {
                            tracing::error!("Field m_time_utc with tag 5 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_time_utc"),
                                5,
                            ));
                        }
                    }
                    6 => {
                        tracing::debug!("Field [{i}]: tagged '6' for field m_time_local_offset");
                        if m_time_local_offset.is_none() {
                            let (new_tail, parsed_m_time_local_offset) =
                                Self::parse_m_time_local_offset(tail)?;
                            tail = new_tail;
                            m_time_local_offset = Some(parsed_m_time_local_offset);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_time_local_offset with tag 6 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_time_local_offset"),
                                6,
                            ));
                        }
                    }
                    16 => {
                        tracing::debug!(
                            "Field [{i}]: tagged '16' for field m_restart_as_transition_map"
                        );
                        if let Some(None) = m_restart_as_transition_map {
                            let (new_tail, parsed_m_restart_as_transition_map) =
                                Self::parse_m_restart_as_transition_map(tail)?;
                            tail = new_tail;
                            m_restart_as_transition_map = Some(parsed_m_restart_as_transition_map);
                            continue;
                        } else {
                            tracing::error!("Field m_restart_as_transition_map with tag 16 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_restart_as_transition_map"),
                                16,
                            ));
                        }
                    }
                    17 => {
                        tracing::debug!(
                            "Field [{i}]: tagged '17' for field m_disable_recover_game"
                        );
                        if m_disable_recover_game.is_none() {
                            let (new_tail, parsed_m_disable_recover_game) =
                                Self::parse_m_disable_recover_game(tail)?;
                            tail = new_tail;
                            m_disable_recover_game = Some(parsed_m_disable_recover_game);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_disable_recover_game with tag 17 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_disable_recover_game"),
                                17,
                            ));
                        }
                    }
                    7 => {
                        tracing::debug!("Field [{i}]: tagged '7' for field m_description");
                        if m_description.is_none() {
                            let (new_tail, parsed_m_description) = Self::parse_m_description(tail)?;
                            tail = new_tail;
                            m_description = Some(parsed_m_description);
                            continue;
                        } else {
                            tracing::error!("Field m_description with tag 7 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_description"),
                                7,
                            ));
                        }
                    }
                    8 => {
                        tracing::debug!("Field [{i}]: tagged '8' for field m_image_file_path");
                        if m_image_file_path.is_none() {
                            let (new_tail, parsed_m_image_file_path) =
                                Self::parse_m_image_file_path(tail)?;
                            tail = new_tail;
                            m_image_file_path = Some(parsed_m_image_file_path);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_image_file_path with tag 8 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_image_file_path"),
                                8,
                            ));
                        }
                    }
                    15 => {
                        tracing::debug!("Field [{i}]: tagged '15' for field m_campaign_index");
                        if m_campaign_index.is_none() {
                            let (new_tail, parsed_m_campaign_index) =
                                Self::parse_m_campaign_index(tail)?;
                            tail = new_tail;
                            m_campaign_index = Some(parsed_m_campaign_index);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_campaign_index with tag 15 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_campaign_index"),
                                15,
                            ));
                        }
                    }
                    9 => {
                        tracing::debug!("Field [{i}]: tagged '9' for field m_map_file_name");
                        if m_map_file_name.is_none() {
                            let (new_tail, parsed_m_map_file_name) =
                                Self::parse_m_map_file_name(tail)?;
                            tail = new_tail;
                            m_map_file_name = Some(parsed_m_map_file_name);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_map_file_name with tag 9 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_map_file_name"),
                                9,
                            ));
                        }
                    }
                    10 => {
                        tracing::debug!("Field [{i}]: tagged '10' for field m_cache_handles");
                        if let Some(None) = m_cache_handles {
                            let (new_tail, parsed_m_cache_handles) =
                                Self::parse_m_cache_handles(tail)?;
                            tail = new_tail;
                            m_cache_handles = Some(parsed_m_cache_handles);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_cache_handles with tag 10 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_cache_handles"),
                                10,
                            ));
                        }
                    }
                    11 => {
                        tracing::debug!("Field [{i}]: tagged '11' for field m_mini_save");
                        if m_mini_save.is_none() {
                            let (new_tail, parsed_m_mini_save) = Self::parse_m_mini_save(tail)?;
                            tail = new_tail;
                            m_mini_save = Some(parsed_m_mini_save);
                            continue;
                        } else {
                            tracing::error!("Field m_mini_save with tag 11 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_mini_save"),
                                11,
                            ));
                        }
                    }
                    12 => {
                        tracing::debug!("Field [{i}]: tagged '12' for field m_game_speed");
                        if m_game_speed.is_none() {
                            let (new_tail, parsed_m_game_speed) = Self::parse_m_game_speed(tail)?;
                            tail = new_tail;
                            m_game_speed = Some(parsed_m_game_speed);
                            continue;
                        } else {
                            tracing::error!("Field m_game_speed with tag 12 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_game_speed"),
                                12,
                            ));
                        }
                    }
                    13 => {
                        tracing::debug!("Field [{i}]: tagged '13' for field m_default_difficulty");
                        if m_default_difficulty.is_none() {
                            let (new_tail, parsed_m_default_difficulty) =
                                Self::parse_m_default_difficulty(tail)?;
                            tail = new_tail;
                            m_default_difficulty = Some(parsed_m_default_difficulty);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_default_difficulty with tag 13 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_default_difficulty"),
                                13,
                            ));
                        }
                    }
                    14 => {
                        tracing::debug!("Field [{i}]: tagged '14' for field m_mod_paths");
                        if let Some(None) = m_mod_paths {
                            let (new_tail, parsed_m_mod_paths) = Self::parse_m_mod_paths(tail)?;
                            tail = new_tail;
                            m_mod_paths = Some(parsed_m_mod_paths);
                            continue;
                        } else {
                            tracing::error!("Field m_mod_paths with tag 14 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_mod_paths"),
                                14,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_player_list: ok_or_return_missing_field_err!(m_player_list),
                    m_title: ok_or_return_missing_field_err!(m_title),
                    m_difficulty: ok_or_return_missing_field_err!(m_difficulty),
                    m_thumbnail: ok_or_return_missing_field_err!(m_thumbnail),
                    m_is_blizzard_map: ok_or_return_missing_field_err!(m_is_blizzard_map),
                    m_time_utc: ok_or_return_missing_field_err!(m_time_utc),
                    m_time_local_offset: ok_or_return_missing_field_err!(m_time_local_offset),
                    m_restart_as_transition_map: ok_or_return_missing_field_err!(
                        m_restart_as_transition_map
                    ),
                    m_disable_recover_game: ok_or_return_missing_field_err!(m_disable_recover_game),
                    m_description: ok_or_return_missing_field_err!(m_description),
                    m_image_file_path: ok_or_return_missing_field_err!(m_image_file_path),
                    m_campaign_index: ok_or_return_missing_field_err!(m_campaign_index),
                    m_map_file_name: ok_or_return_missing_field_err!(m_map_file_name),
                    m_cache_handles: ok_or_return_missing_field_err!(m_cache_handles),
                    m_mini_save: ok_or_return_missing_field_err!(m_mini_save),
                    m_game_speed: ok_or_return_missing_field_err!(m_game_speed),
                    m_default_difficulty: ok_or_return_missing_field_err!(m_default_difficulty),
                    m_mod_paths: ok_or_return_missing_field_err!(m_mod_paths),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
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
        pub fn parse_m_signature(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_signature) = tagged_blob(input)?;

            tracing::debug!("m_signature: {:?}", m_signature);
            Ok((tail, m_signature))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_version(input: &[u8]) -> S2ProtoResult<&[u8], SVersion> {
            let (tail, m_version) = SVersion::parse(input)?;

            tracing::debug!("m_version: {:?}", m_version);
            Ok((tail, m_version))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_type(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_type) = tagged_vlq_int(input)?;

            tracing::debug!("m_type: {:?}", m_type);
            Ok((tail, u8::try_from(m_type)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_elapsed_game_loops(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_elapsed_game_loops) = tagged_vlq_int(input)?;

            tracing::debug!("m_elapsed_game_loops: {:?}", m_elapsed_game_loops);
            Ok((tail, u32::try_from(m_elapsed_game_loops)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_use_scaled_time(input: &[u8]) -> S2ProtoResult<&[u8], bool> {
            let (tail, m_use_scaled_time) = tagged_bool(input)?;

            tracing::debug!("m_use_scaled_time: {:?}", m_use_scaled_time);
            Ok((tail, m_use_scaled_time))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_ngdp_root_key(input: &[u8]) -> S2ProtoResult<&[u8], Smd5> {
            let (tail, m_ngdp_root_key) = Smd5::parse(input)?;

            tracing::debug!("m_ngdp_root_key: {:?}", m_ngdp_root_key);
            Ok((tail, m_ngdp_root_key))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_data_build_num(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_data_build_num) = tagged_vlq_int(input)?;

            tracing::debug!("m_data_build_num: {:?}", m_data_build_num);
            Ok((tail, u32::try_from(m_data_build_num)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_replay_compatibility_hash(input: &[u8]) -> S2ProtoResult<&[u8], Smd5> {
            let (tail, m_replay_compatibility_hash) = Smd5::parse(input)?;

            tracing::debug!(
                "m_replay_compatibility_hash: {:?}",
                m_replay_compatibility_hash
            );
            Ok((tail, m_replay_compatibility_hash))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_ngdp_root_key_is_dev_data(input: &[u8]) -> S2ProtoResult<&[u8], bool> {
            let (tail, m_ngdp_root_key_is_dev_data) = tagged_bool(input)?;

            tracing::debug!(
                "m_ngdp_root_key_is_dev_data: {:?}",
                m_ngdp_root_key_is_dev_data
            );
            Ok((tail, m_ngdp_root_key_is_dev_data))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplaySHeader::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_signature"),
                                0,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_version"),
                                1,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_type"), 2));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_elapsed_game_loops"),
                                3,
                            ));
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
                            tracing::error!(
                                "Field m_use_scaled_time with tag 4 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_use_scaled_time"),
                                4,
                            ));
                        }
                    }
                    5 => {
                        tracing::debug!("Field [{i}]: tagged '5' for field m_ngdp_root_key");
                        if m_ngdp_root_key.is_none() {
                            let (new_tail, parsed_m_ngdp_root_key) =
                                Self::parse_m_ngdp_root_key(tail)?;
                            tail = new_tail;
                            m_ngdp_root_key = Some(parsed_m_ngdp_root_key);
                            continue;
                        } else {
                            tracing::error!(
                                "Field m_ngdp_root_key with tag 5 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_ngdp_root_key"),
                                5,
                            ));
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
                            tracing::error!(
                                "Field m_data_build_num with tag 6 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_data_build_num"),
                                6,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_replay_compatibility_hash"),
                                7,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_ngdp_root_key_is_dev_data"),
                                8,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_signature: ok_or_return_missing_field_err!(m_signature),
                    m_version: ok_or_return_missing_field_err!(m_version),
                    m_type: ok_or_return_missing_field_err!(m_type),
                    m_elapsed_game_loops: ok_or_return_missing_field_err!(m_elapsed_game_loops),
                    m_use_scaled_time: ok_or_return_missing_field_err!(m_use_scaled_time),
                    m_ngdp_root_key: ok_or_return_missing_field_err!(m_ngdp_root_key),
                    m_data_build_num: ok_or_return_missing_field_err!(m_data_build_num),
                    m_replay_compatibility_hash: ok_or_return_missing_field_err!(
                        m_replay_compatibility_hash
                    ),
                    m_ngdp_root_key_is_dev_data: ok_or_return_missing_field_err!(
                        m_ngdp_root_key_is_dev_data
                    ),
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
        #[tracing::instrument(name="87702::ReplayTrackerEEventId::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
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
        pub fn parse_m_score_value_minerals_current(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_current) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_current: {:?}",
                m_score_value_minerals_current
            );
            Ok((tail, i32::try_from(m_score_value_minerals_current)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_current(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_current) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_current: {:?}",
                m_score_value_vespene_current
            );
            Ok((tail, i32::try_from(m_score_value_vespene_current)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_collection_rate(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_collection_rate) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_collection_rate: {:?}",
                m_score_value_minerals_collection_rate
            );
            Ok((tail, i32::try_from(m_score_value_minerals_collection_rate)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_collection_rate(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_collection_rate) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_collection_rate: {:?}",
                m_score_value_vespene_collection_rate
            );
            Ok((tail, i32::try_from(m_score_value_vespene_collection_rate)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_workers_active_count(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_workers_active_count) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_workers_active_count: {:?}",
                m_score_value_workers_active_count
            );
            Ok((tail, i32::try_from(m_score_value_workers_active_count)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_used_in_progress_army(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_used_in_progress_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_used_in_progress_army: {:?}",
                m_score_value_minerals_used_in_progress_army
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_used_in_progress_army)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_used_in_progress_economy(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_used_in_progress_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_used_in_progress_economy: {:?}",
                m_score_value_minerals_used_in_progress_economy
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_used_in_progress_economy)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_used_in_progress_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_used_in_progress_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "res: {:?}",
                m_score_value_minerals_used_in_progress_technology
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_used_in_progress_technology)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_used_in_progress_army(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_used_in_progress_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_used_in_progress_army: {:?}",
                m_score_value_vespene_used_in_progress_army
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_used_in_progress_army)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_used_in_progress_economy(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_used_in_progress_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_used_in_progress_economy: {:?}",
                m_score_value_vespene_used_in_progress_economy
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_used_in_progress_economy)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_used_in_progress_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_used_in_progress_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "res: {:?}",
                m_score_value_vespene_used_in_progress_technology
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_used_in_progress_technology)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_used_current_army(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_used_current_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_used_current_army: {:?}",
                m_score_value_minerals_used_current_army
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_used_current_army)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_used_current_economy(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_used_current_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_used_current_economy: {:?}",
                m_score_value_minerals_used_current_economy
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_used_current_economy)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_used_current_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_used_current_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_used_current_technology: {:?}",
                m_score_value_minerals_used_current_technology
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_used_current_technology)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_used_current_army(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_used_current_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_used_current_army: {:?}",
                m_score_value_vespene_used_current_army
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_used_current_army)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_used_current_economy(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_used_current_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_used_current_economy: {:?}",
                m_score_value_vespene_used_current_economy
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_used_current_economy)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_used_current_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_used_current_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_used_current_technology: {:?}",
                m_score_value_vespene_used_current_technology
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_used_current_technology)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_lost_army(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_lost_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_lost_army: {:?}",
                m_score_value_minerals_lost_army
            );
            Ok((tail, i32::try_from(m_score_value_minerals_lost_army)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_lost_economy(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_lost_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_lost_economy: {:?}",
                m_score_value_minerals_lost_economy
            );
            Ok((tail, i32::try_from(m_score_value_minerals_lost_economy)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_lost_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_lost_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_lost_technology: {:?}",
                m_score_value_minerals_lost_technology
            );
            Ok((tail, i32::try_from(m_score_value_minerals_lost_technology)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_lost_army(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_lost_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_lost_army: {:?}",
                m_score_value_vespene_lost_army
            );
            Ok((tail, i32::try_from(m_score_value_vespene_lost_army)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_lost_economy(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_lost_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_lost_economy: {:?}",
                m_score_value_vespene_lost_economy
            );
            Ok((tail, i32::try_from(m_score_value_vespene_lost_economy)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_lost_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_lost_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_lost_technology: {:?}",
                m_score_value_vespene_lost_technology
            );
            Ok((tail, i32::try_from(m_score_value_vespene_lost_technology)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_killed_army(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_killed_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_killed_army: {:?}",
                m_score_value_minerals_killed_army
            );
            Ok((tail, i32::try_from(m_score_value_minerals_killed_army)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_killed_economy(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_killed_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_killed_economy: {:?}",
                m_score_value_minerals_killed_economy
            );
            Ok((tail, i32::try_from(m_score_value_minerals_killed_economy)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_killed_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_killed_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_killed_technology: {:?}",
                m_score_value_minerals_killed_technology
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_killed_technology)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_killed_army(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_killed_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_killed_army: {:?}",
                m_score_value_vespene_killed_army
            );
            Ok((tail, i32::try_from(m_score_value_vespene_killed_army)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_killed_economy(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_killed_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_killed_economy: {:?}",
                m_score_value_vespene_killed_economy
            );
            Ok((tail, i32::try_from(m_score_value_vespene_killed_economy)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_killed_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_killed_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_killed_technology: {:?}",
                m_score_value_vespene_killed_technology
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_killed_technology)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_food_used(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_food_used) = tagged_vlq_int(input)?;

            tracing::debug!("m_score_value_food_used: {:?}", m_score_value_food_used);
            Ok((tail, i32::try_from(m_score_value_food_used)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_food_made(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_food_made) = tagged_vlq_int(input)?;

            tracing::debug!("m_score_value_food_made: {:?}", m_score_value_food_made);
            Ok((tail, i32::try_from(m_score_value_food_made)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_used_active_forces(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_used_active_forces) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_used_active_forces: {:?}",
                m_score_value_minerals_used_active_forces
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_used_active_forces)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_used_active_forces(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_used_active_forces) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_used_active_forces: {:?}",
                m_score_value_vespene_used_active_forces
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_used_active_forces)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_friendly_fire_army(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_friendly_fire_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_friendly_fire_army: {:?}",
                m_score_value_minerals_friendly_fire_army
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_friendly_fire_army)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_friendly_fire_economy(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_friendly_fire_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_friendly_fire_economy: {:?}",
                m_score_value_minerals_friendly_fire_economy
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_friendly_fire_economy)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_minerals_friendly_fire_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_minerals_friendly_fire_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_minerals_friendly_fire_technology: {:?}",
                m_score_value_minerals_friendly_fire_technology
            );
            Ok((
                tail,
                i32::try_from(m_score_value_minerals_friendly_fire_technology)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_friendly_fire_army(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_friendly_fire_army) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_friendly_fire_army: {:?}",
                m_score_value_vespene_friendly_fire_army
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_friendly_fire_army)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_friendly_fire_economy(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_friendly_fire_economy) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_friendly_fire_economy: {:?}",
                m_score_value_vespene_friendly_fire_economy
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_friendly_fire_economy)?,
            ))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_score_value_vespene_friendly_fire_technology(
            input: &[u8],
        ) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_score_value_vespene_friendly_fire_technology) = tagged_vlq_int(input)?;

            tracing::debug!(
                "m_score_value_vespene_friendly_fire_technology: {:?}",
                m_score_value_vespene_friendly_fire_technology
            );
            Ok((
                tail,
                i32::try_from(m_score_value_vespene_friendly_fire_technology)?,
            ))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSPlayerStats::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            tracing::error!("Field m_score_value_minerals_current with tag 0 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_current"),
                                0,
                            ));
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
                            m_score_value_vespene_current =
                                Some(parsed_m_score_value_vespene_current);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_vespene_current with tag 1 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_current"),
                                1,
                            ));
                        }
                    }
                    2 => {
                        tracing::debug!("Field [{i}]: tagged '2' for field m_score_value_minerals_collection_rate");
                        if m_score_value_minerals_collection_rate.is_none() {
                            let (new_tail, parsed_m_score_value_minerals_collection_rate) =
                                Self::parse_m_score_value_minerals_collection_rate(tail)?;
                            tail = new_tail;
                            m_score_value_minerals_collection_rate =
                                Some(parsed_m_score_value_minerals_collection_rate);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_minerals_collection_rate with tag 2 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_collection_rate"),
                                2,
                            ));
                        }
                    }
                    3 => {
                        tracing::debug!("Field [{i}]: tagged '3' for field m_score_value_vespene_collection_rate");
                        if m_score_value_vespene_collection_rate.is_none() {
                            let (new_tail, parsed_m_score_value_vespene_collection_rate) =
                                Self::parse_m_score_value_vespene_collection_rate(tail)?;
                            tail = new_tail;
                            m_score_value_vespene_collection_rate =
                                Some(parsed_m_score_value_vespene_collection_rate);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_vespene_collection_rate with tag 3 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_collection_rate"),
                                3,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_workers_active_count"),
                                4,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_used_in_progress_army"),
                                5,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_used_in_progress_economy"),
                                6,
                            ));
                        }
                    }
                    7 => {
                        tracing::debug!("Field [{i}]: tagged '7' for field m_score_value_minerals_used_in_progress_technology");
                        if m_score_value_minerals_used_in_progress_technology.is_none() {
                            let (
                                new_tail,
                                parsed_m_score_value_minerals_used_in_progress_technology,
                            ) = Self::parse_m_score_value_minerals_used_in_progress_technology(
                                tail,
                            )?;
                            tail = new_tail;
                            m_score_value_minerals_used_in_progress_technology =
                                Some(parsed_m_score_value_minerals_used_in_progress_technology);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_minerals_used_in_progress_technology with tag 7 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_used_in_progress_technology"),
                                7,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_used_in_progress_army"),
                                8,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_used_in_progress_economy"),
                                9,
                            ));
                        }
                    }
                    10 => {
                        tracing::debug!("Field [{i}]: tagged '10' for field m_score_value_vespene_used_in_progress_technology");
                        if m_score_value_vespene_used_in_progress_technology.is_none() {
                            let (
                                new_tail,
                                parsed_m_score_value_vespene_used_in_progress_technology,
                            ) = Self::parse_m_score_value_vespene_used_in_progress_technology(
                                tail,
                            )?;
                            tail = new_tail;
                            m_score_value_vespene_used_in_progress_technology =
                                Some(parsed_m_score_value_vespene_used_in_progress_technology);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_vespene_used_in_progress_technology with tag 10 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_used_in_progress_technology"),
                                10,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_used_current_army"),
                                11,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_used_current_economy"),
                                12,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_used_current_technology"),
                                13,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_used_current_army"),
                                14,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_used_current_economy"),
                                15,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_used_current_technology"),
                                16,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_lost_army"),
                                17,
                            ));
                        }
                    }
                    18 => {
                        tracing::debug!("Field [{i}]: tagged '18' for field m_score_value_minerals_lost_economy");
                        if m_score_value_minerals_lost_economy.is_none() {
                            let (new_tail, parsed_m_score_value_minerals_lost_economy) =
                                Self::parse_m_score_value_minerals_lost_economy(tail)?;
                            tail = new_tail;
                            m_score_value_minerals_lost_economy =
                                Some(parsed_m_score_value_minerals_lost_economy);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_minerals_lost_economy with tag 18 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_lost_economy"),
                                18,
                            ));
                        }
                    }
                    19 => {
                        tracing::debug!("Field [{i}]: tagged '19' for field m_score_value_minerals_lost_technology");
                        if m_score_value_minerals_lost_technology.is_none() {
                            let (new_tail, parsed_m_score_value_minerals_lost_technology) =
                                Self::parse_m_score_value_minerals_lost_technology(tail)?;
                            tail = new_tail;
                            m_score_value_minerals_lost_technology =
                                Some(parsed_m_score_value_minerals_lost_technology);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_minerals_lost_technology with tag 19 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_lost_technology"),
                                19,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_lost_army"),
                                20,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_lost_economy"),
                                21,
                            ));
                        }
                    }
                    22 => {
                        tracing::debug!("Field [{i}]: tagged '22' for field m_score_value_vespene_lost_technology");
                        if m_score_value_vespene_lost_technology.is_none() {
                            let (new_tail, parsed_m_score_value_vespene_lost_technology) =
                                Self::parse_m_score_value_vespene_lost_technology(tail)?;
                            tail = new_tail;
                            m_score_value_vespene_lost_technology =
                                Some(parsed_m_score_value_vespene_lost_technology);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_vespene_lost_technology with tag 22 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_lost_technology"),
                                22,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_killed_army"),
                                23,
                            ));
                        }
                    }
                    24 => {
                        tracing::debug!("Field [{i}]: tagged '24' for field m_score_value_minerals_killed_economy");
                        if m_score_value_minerals_killed_economy.is_none() {
                            let (new_tail, parsed_m_score_value_minerals_killed_economy) =
                                Self::parse_m_score_value_minerals_killed_economy(tail)?;
                            tail = new_tail;
                            m_score_value_minerals_killed_economy =
                                Some(parsed_m_score_value_minerals_killed_economy);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_minerals_killed_economy with tag 24 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_killed_economy"),
                                24,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_killed_technology"),
                                25,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_killed_army"),
                                26,
                            ));
                        }
                    }
                    27 => {
                        tracing::debug!("Field [{i}]: tagged '27' for field m_score_value_vespene_killed_economy");
                        if m_score_value_vespene_killed_economy.is_none() {
                            let (new_tail, parsed_m_score_value_vespene_killed_economy) =
                                Self::parse_m_score_value_vespene_killed_economy(tail)?;
                            tail = new_tail;
                            m_score_value_vespene_killed_economy =
                                Some(parsed_m_score_value_vespene_killed_economy);
                            continue;
                        } else {
                            tracing::error!("Field m_score_value_vespene_killed_economy with tag 27 was already provided");
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_killed_economy"),
                                27,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_killed_technology"),
                                28,
                            ));
                        }
                    }
                    29 => {
                        tracing::debug!(
                            "Field [{i}]: tagged '29' for field m_score_value_food_used"
                        );
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_food_used"),
                                29,
                            ));
                        }
                    }
                    30 => {
                        tracing::debug!(
                            "Field [{i}]: tagged '30' for field m_score_value_food_made"
                        );
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_food_made"),
                                30,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_used_active_forces"),
                                31,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_used_active_forces"),
                                32,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_friendly_fire_army"),
                                33,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_friendly_fire_economy"),
                                34,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_minerals_friendly_fire_technology"),
                                35,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_friendly_fire_army"),
                                36,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_friendly_fire_economy"),
                                37,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_score_value_vespene_friendly_fire_technology"),
                                38,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_score_value_minerals_current: ok_or_return_missing_field_err!(
                        m_score_value_minerals_current
                    ),
                    m_score_value_vespene_current: ok_or_return_missing_field_err!(
                        m_score_value_vespene_current
                    ),
                    m_score_value_minerals_collection_rate: ok_or_return_missing_field_err!(
                        m_score_value_minerals_collection_rate
                    ),
                    m_score_value_vespene_collection_rate: ok_or_return_missing_field_err!(
                        m_score_value_vespene_collection_rate
                    ),
                    m_score_value_workers_active_count: ok_or_return_missing_field_err!(
                        m_score_value_workers_active_count
                    ),
                    m_score_value_minerals_used_in_progress_army: ok_or_return_missing_field_err!(
                        m_score_value_minerals_used_in_progress_army
                    ),
                    m_score_value_minerals_used_in_progress_economy: ok_or_return_missing_field_err!(
                        m_score_value_minerals_used_in_progress_economy
                    ),
                    m_score_value_minerals_used_in_progress_technology: ok_or_return_missing_field_err!(
                        m_score_value_minerals_used_in_progress_technology
                    ),
                    m_score_value_vespene_used_in_progress_army: ok_or_return_missing_field_err!(
                        m_score_value_vespene_used_in_progress_army
                    ),
                    m_score_value_vespene_used_in_progress_economy: ok_or_return_missing_field_err!(
                        m_score_value_vespene_used_in_progress_economy
                    ),
                    m_score_value_vespene_used_in_progress_technology: ok_or_return_missing_field_err!(
                        m_score_value_vespene_used_in_progress_technology
                    ),
                    m_score_value_minerals_used_current_army: ok_or_return_missing_field_err!(
                        m_score_value_minerals_used_current_army
                    ),
                    m_score_value_minerals_used_current_economy: ok_or_return_missing_field_err!(
                        m_score_value_minerals_used_current_economy
                    ),
                    m_score_value_minerals_used_current_technology: ok_or_return_missing_field_err!(
                        m_score_value_minerals_used_current_technology
                    ),
                    m_score_value_vespene_used_current_army: ok_or_return_missing_field_err!(
                        m_score_value_vespene_used_current_army
                    ),
                    m_score_value_vespene_used_current_economy: ok_or_return_missing_field_err!(
                        m_score_value_vespene_used_current_economy
                    ),
                    m_score_value_vespene_used_current_technology: ok_or_return_missing_field_err!(
                        m_score_value_vespene_used_current_technology
                    ),
                    m_score_value_minerals_lost_army: ok_or_return_missing_field_err!(
                        m_score_value_minerals_lost_army
                    ),
                    m_score_value_minerals_lost_economy: ok_or_return_missing_field_err!(
                        m_score_value_minerals_lost_economy
                    ),
                    m_score_value_minerals_lost_technology: ok_or_return_missing_field_err!(
                        m_score_value_minerals_lost_technology
                    ),
                    m_score_value_vespene_lost_army: ok_or_return_missing_field_err!(
                        m_score_value_vespene_lost_army
                    ),
                    m_score_value_vespene_lost_economy: ok_or_return_missing_field_err!(
                        m_score_value_vespene_lost_economy
                    ),
                    m_score_value_vespene_lost_technology: ok_or_return_missing_field_err!(
                        m_score_value_vespene_lost_technology
                    ),
                    m_score_value_minerals_killed_army: ok_or_return_missing_field_err!(
                        m_score_value_minerals_killed_army
                    ),
                    m_score_value_minerals_killed_economy: ok_or_return_missing_field_err!(
                        m_score_value_minerals_killed_economy
                    ),
                    m_score_value_minerals_killed_technology: ok_or_return_missing_field_err!(
                        m_score_value_minerals_killed_technology
                    ),
                    m_score_value_vespene_killed_army: ok_or_return_missing_field_err!(
                        m_score_value_vespene_killed_army
                    ),
                    m_score_value_vespene_killed_economy: ok_or_return_missing_field_err!(
                        m_score_value_vespene_killed_economy
                    ),
                    m_score_value_vespene_killed_technology: ok_or_return_missing_field_err!(
                        m_score_value_vespene_killed_technology
                    ),
                    m_score_value_food_used: ok_or_return_missing_field_err!(
                        m_score_value_food_used
                    ),
                    m_score_value_food_made: ok_or_return_missing_field_err!(
                        m_score_value_food_made
                    ),
                    m_score_value_minerals_used_active_forces: ok_or_return_missing_field_err!(
                        m_score_value_minerals_used_active_forces
                    ),
                    m_score_value_vespene_used_active_forces: ok_or_return_missing_field_err!(
                        m_score_value_vespene_used_active_forces
                    ),
                    m_score_value_minerals_friendly_fire_army: ok_or_return_missing_field_err!(
                        m_score_value_minerals_friendly_fire_army
                    ),
                    m_score_value_minerals_friendly_fire_economy: ok_or_return_missing_field_err!(
                        m_score_value_minerals_friendly_fire_economy
                    ),
                    m_score_value_minerals_friendly_fire_technology: ok_or_return_missing_field_err!(
                        m_score_value_minerals_friendly_fire_technology
                    ),
                    m_score_value_vespene_friendly_fire_army: ok_or_return_missing_field_err!(
                        m_score_value_vespene_friendly_fire_army
                    ),
                    m_score_value_vespene_friendly_fire_economy: ok_or_return_missing_field_err!(
                        m_score_value_vespene_friendly_fire_economy
                    ),
                    m_score_value_vespene_friendly_fire_technology: ok_or_return_missing_field_err!(
                        m_score_value_vespene_friendly_fire_technology
                    ),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReplayTrackerSPlayerStatsEvent {
        pub m_player_id: u8,
        pub m_stats: ReplayTrackerSPlayerStats,
    }
    impl ReplayTrackerSPlayerStatsEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_player_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_player_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_player_id: {:?}", m_player_id);
            Ok((tail, u8::try_from(m_player_id)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_stats(input: &[u8]) -> S2ProtoResult<&[u8], ReplayTrackerSPlayerStats> {
            let (tail, m_stats) = ReplayTrackerSPlayerStats::parse(input)?;

            tracing::debug!("m_stats: {:?}", m_stats);
            Ok((tail, m_stats))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSPlayerStatsEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_player_id"),
                                0,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_stats"), 1));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_player_id: ok_or_return_missing_field_err!(m_player_id),
                    m_stats: ok_or_return_missing_field_err!(m_stats),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
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
        pub fn parse_m_unit_tag_index(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_index: {:?}", m_unit_tag_index);
            Ok((tail, u32::try_from(m_unit_tag_index)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_tag_recycle(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_recycle: {:?}", m_unit_tag_recycle);
            Ok((tail, u32::try_from(m_unit_tag_recycle)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_type_name(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_unit_type_name) = tagged_blob(input)?;

            tracing::debug!("m_unit_type_name: {:?}", str::from_utf8(&m_unit_type_name));
            Ok((tail, m_unit_type_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_control_player_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_control_player_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_control_player_id: {:?}", m_control_player_id);
            Ok((tail, u8::try_from(m_control_player_id)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_upkeep_player_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_upkeep_player_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_upkeep_player_id: {:?}", m_upkeep_player_id);
            Ok((tail, u8::try_from(m_upkeep_player_id)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_x(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_x) = tagged_vlq_int(input)?;

            tracing::debug!("m_x: {:?}", m_x);
            Ok((tail, u8::try_from(m_x)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_y(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_y) = tagged_vlq_int(input)?;

            tracing::debug!("m_y: {:?}", m_y);
            Ok((tail, u8::try_from(m_y)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_creator_unit_tag_index(input: &[u8]) -> S2ProtoResult<&[u8], Option<u32>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_creator_unit_tag_index) = if is_provided != 0 {
                let (tail, res) = tagged_vlq_int(tail)?;
                (tail, Some(<_>::try_from(res)?))
            } else {
                (tail, None)
            };
            tracing::debug!("m_creator_unit_tag_index: {:?}", m_creator_unit_tag_index);
            Ok((tail, m_creator_unit_tag_index))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_creator_unit_tag_recycle(input: &[u8]) -> S2ProtoResult<&[u8], Option<u32>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_creator_unit_tag_recycle) = if is_provided != 0 {
                let (tail, res) = tagged_vlq_int(tail)?;
                (tail, Some(<_>::try_from(res)?))
            } else {
                (tail, None)
            };
            tracing::debug!(
                "m_creator_unit_tag_recycle: {:?}",
                m_creator_unit_tag_recycle
            );
            Ok((tail, m_creator_unit_tag_recycle))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_creator_ability_name(input: &[u8]) -> S2ProtoResult<&[u8], Option<Vec<u8>>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_creator_ability_name) = if is_provided != 0 {
                let (tail, res) = tagged_blob(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!(
                "m_creator_ability_name: {:?}",
                str::from_utf8(m_creator_ability_name.as_ref().unwrap_or(&vec![]))
            );
            Ok((tail, m_creator_ability_name))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSUnitBornEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            tracing::error!(
                                "Field m_unit_tag_index with tag 0 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_index"),
                                8,
                            ));
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
                            tracing::error!(
                                "Field m_unit_tag_recycle with tag 1 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_recycle"),
                                8,
                            ));
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
                            tracing::error!(
                                "Field m_unit_type_name with tag 2 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_type_name"),
                                8,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_control_player_id"),
                                8,
                            ));
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
                            tracing::error!(
                                "Field m_upkeep_player_id with tag 4 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_upkeep_player_id"),
                                8,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_x"), 8));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_y"), 8));
                        }
                    }
                    7 => {
                        tracing::debug!(
                            "Field [{i}]: tagged '7' for field m_creator_unit_tag_index"
                        );
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_creator_unit_tag_index"),
                                8,
                            ));
                        }
                    }
                    8 => {
                        tracing::debug!(
                            "Field [{i}]: tagged '8' for field m_creator_unit_tag_recycle"
                        );
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_creator_unit_tag_recycle"),
                                8,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_creator_ability_name"),
                                8,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_unit_tag_index: ok_or_return_missing_field_err!(m_unit_tag_index),
                    m_unit_tag_recycle: ok_or_return_missing_field_err!(m_unit_tag_recycle),
                    m_unit_type_name: ok_or_return_missing_field_err!(m_unit_type_name),
                    m_control_player_id: ok_or_return_missing_field_err!(m_control_player_id),
                    m_upkeep_player_id: ok_or_return_missing_field_err!(m_upkeep_player_id),
                    m_x: ok_or_return_missing_field_err!(m_x),
                    m_y: ok_or_return_missing_field_err!(m_y),
                    m_creator_unit_tag_index: ok_or_return_missing_field_err!(
                        m_creator_unit_tag_index
                    ),
                    m_creator_unit_tag_recycle: ok_or_return_missing_field_err!(
                        m_creator_unit_tag_recycle
                    ),
                    m_creator_ability_name: ok_or_return_missing_field_err!(m_creator_ability_name),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
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
        pub fn parse_m_unit_tag_index(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_index: {:?}", m_unit_tag_index);
            Ok((tail, u32::try_from(m_unit_tag_index)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_tag_recycle(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_recycle: {:?}", m_unit_tag_recycle);
            Ok((tail, u32::try_from(m_unit_tag_recycle)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_killer_player_id(input: &[u8]) -> S2ProtoResult<&[u8], Option<u8>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_killer_player_id) = if is_provided != 0 {
                let (tail, res) = tagged_vlq_int(tail)?;
                (tail, Some(<_>::try_from(res)?))
            } else {
                (tail, None)
            };
            tracing::debug!("m_killer_player_id: {:?}", m_killer_player_id);
            Ok((tail, m_killer_player_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_x(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_x) = tagged_vlq_int(input)?;

            tracing::debug!("m_x: {:?}", m_x);
            Ok((tail, u8::try_from(m_x)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_y(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_y) = tagged_vlq_int(input)?;

            tracing::debug!("m_y: {:?}", m_y);
            Ok((tail, u8::try_from(m_y)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_killer_unit_tag_index(input: &[u8]) -> S2ProtoResult<&[u8], Option<u32>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_killer_unit_tag_index) = if is_provided != 0 {
                let (tail, res) = tagged_vlq_int(tail)?;
                (tail, Some(<_>::try_from(res)?))
            } else {
                (tail, None)
            };
            tracing::debug!("m_killer_unit_tag_index: {:?}", m_killer_unit_tag_index);
            Ok((tail, m_killer_unit_tag_index))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_killer_unit_tag_recycle(input: &[u8]) -> S2ProtoResult<&[u8], Option<u32>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_killer_unit_tag_recycle) = if is_provided != 0 {
                let (tail, res) = tagged_vlq_int(tail)?;
                (tail, Some(<_>::try_from(res)?))
            } else {
                (tail, None)
            };
            tracing::debug!("m_killer_unit_tag_recycle: {:?}", m_killer_unit_tag_recycle);
            Ok((tail, m_killer_unit_tag_recycle))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSUnitDiedEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            tracing::error!(
                                "Field m_unit_tag_index with tag 0 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_index"),
                                8,
                            ));
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
                            tracing::error!(
                                "Field m_unit_tag_recycle with tag 1 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_recycle"),
                                8,
                            ));
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
                            tracing::error!(
                                "Field m_killer_player_id with tag 2 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_killer_player_id"),
                                8,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_x"), 8));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_y"), 8));
                        }
                    }
                    5 => {
                        tracing::debug!(
                            "Field [{i}]: tagged '5' for field m_killer_unit_tag_index"
                        );
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_killer_unit_tag_index"),
                                8,
                            ));
                        }
                    }
                    6 => {
                        tracing::debug!(
                            "Field [{i}]: tagged '6' for field m_killer_unit_tag_recycle"
                        );
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_killer_unit_tag_recycle"),
                                8,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_unit_tag_index: ok_or_return_missing_field_err!(m_unit_tag_index),
                    m_unit_tag_recycle: ok_or_return_missing_field_err!(m_unit_tag_recycle),
                    m_killer_player_id: ok_or_return_missing_field_err!(m_killer_player_id),
                    m_x: ok_or_return_missing_field_err!(m_x),
                    m_y: ok_or_return_missing_field_err!(m_y),
                    m_killer_unit_tag_index: ok_or_return_missing_field_err!(
                        m_killer_unit_tag_index
                    ),
                    m_killer_unit_tag_recycle: ok_or_return_missing_field_err!(
                        m_killer_unit_tag_recycle
                    ),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReplayTrackerSUnitOwnerChangeEvent {
        pub m_unit_tag_index: u32,
        pub m_unit_tag_recycle: u32,
        pub m_control_player_id: u8,
        pub m_upkeep_player_id: u8,
    }
    impl ReplayTrackerSUnitOwnerChangeEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_tag_index(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_index: {:?}", m_unit_tag_index);
            Ok((tail, u32::try_from(m_unit_tag_index)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_tag_recycle(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_recycle: {:?}", m_unit_tag_recycle);
            Ok((tail, u32::try_from(m_unit_tag_recycle)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_control_player_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_control_player_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_control_player_id: {:?}", m_control_player_id);
            Ok((tail, u8::try_from(m_control_player_id)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_upkeep_player_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_upkeep_player_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_upkeep_player_id: {:?}", m_upkeep_player_id);
            Ok((tail, u8::try_from(m_upkeep_player_id)?))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSUnitOwnerChangeEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            tracing::error!(
                                "Field m_unit_tag_index with tag 0 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_index"),
                                8,
                            ));
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
                            tracing::error!(
                                "Field m_unit_tag_recycle with tag 1 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_recycle"),
                                8,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_control_player_id"),
                                2,
                            ));
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
                            tracing::error!(
                                "Field m_upkeep_player_id with tag 3 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_upkeep_player_id"),
                                3,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_unit_tag_index: ok_or_return_missing_field_err!(m_unit_tag_index),
                    m_unit_tag_recycle: ok_or_return_missing_field_err!(m_unit_tag_recycle),
                    m_control_player_id: ok_or_return_missing_field_err!(m_control_player_id),
                    m_upkeep_player_id: ok_or_return_missing_field_err!(m_upkeep_player_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReplayTrackerSUnitTypeChangeEvent {
        pub m_unit_tag_index: u32,
        pub m_unit_tag_recycle: u32,
        pub m_unit_type_name: Vec<u8>,
    }
    impl ReplayTrackerSUnitTypeChangeEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_tag_index(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_index: {:?}", m_unit_tag_index);
            Ok((tail, u32::try_from(m_unit_tag_index)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_tag_recycle(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_recycle: {:?}", m_unit_tag_recycle);
            Ok((tail, u32::try_from(m_unit_tag_recycle)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_type_name(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_unit_type_name) = tagged_blob(input)?;

            tracing::debug!("m_unit_type_name: {:?}", str::from_utf8(&m_unit_type_name));
            Ok((tail, m_unit_type_name))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSUnitTypeChangeEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            tracing::error!(
                                "Field m_unit_tag_index with tag 0 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_index"),
                                0,
                            ));
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
                            tracing::error!(
                                "Field m_unit_tag_recycle with tag 1 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_recycle"),
                                1,
                            ));
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
                            tracing::error!(
                                "Field m_unit_type_name with tag 2 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_type_name"),
                                2,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_unit_tag_index: ok_or_return_missing_field_err!(m_unit_tag_index),
                    m_unit_tag_recycle: ok_or_return_missing_field_err!(m_unit_tag_recycle),
                    m_unit_type_name: ok_or_return_missing_field_err!(m_unit_type_name),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReplayTrackerSUpgradeEvent {
        pub m_player_id: u8,
        pub m_upgrade_type_name: Vec<u8>,
        pub m_count: i32,
    }
    impl ReplayTrackerSUpgradeEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_player_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_player_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_player_id: {:?}", m_player_id);
            Ok((tail, u8::try_from(m_player_id)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_upgrade_type_name(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_upgrade_type_name) = tagged_blob(input)?;

            tracing::debug!(
                "m_upgrade_type_name: {:?}",
                str::from_utf8(&m_upgrade_type_name)
            );
            Ok((tail, m_upgrade_type_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_count(input: &[u8]) -> S2ProtoResult<&[u8], i32> {
            let (tail, m_count) = tagged_vlq_int(input)?;

            tracing::debug!("m_count: {:?}", m_count);
            Ok((tail, i32::try_from(m_count)?))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSUpgradeEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_player_id"),
                                0,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_upgrade_type_name"),
                                1,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_count"), 2));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_player_id: ok_or_return_missing_field_err!(m_player_id),
                    m_upgrade_type_name: ok_or_return_missing_field_err!(m_upgrade_type_name),
                    m_count: ok_or_return_missing_field_err!(m_count),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
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
        pub fn parse_m_unit_tag_index(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_index: {:?}", m_unit_tag_index);
            Ok((tail, u32::try_from(m_unit_tag_index)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_tag_recycle(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_recycle: {:?}", m_unit_tag_recycle);
            Ok((tail, u32::try_from(m_unit_tag_recycle)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_type_name(input: &[u8]) -> S2ProtoResult<&[u8], Vec<u8>> {
            let (tail, m_unit_type_name) = tagged_blob(input)?;

            tracing::debug!("m_unit_type_name: {:?}", str::from_utf8(&m_unit_type_name));
            Ok((tail, m_unit_type_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_control_player_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_control_player_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_control_player_id: {:?}", m_control_player_id);
            Ok((tail, u8::try_from(m_control_player_id)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_upkeep_player_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_upkeep_player_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_upkeep_player_id: {:?}", m_upkeep_player_id);
            Ok((tail, u8::try_from(m_upkeep_player_id)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_x(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_x) = tagged_vlq_int(input)?;

            tracing::debug!("m_x: {:?}", m_x);
            Ok((tail, u8::try_from(m_x)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_y(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_y) = tagged_vlq_int(input)?;

            tracing::debug!("m_y: {:?}", m_y);
            Ok((tail, u8::try_from(m_y)?))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSUnitInitEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            tracing::error!(
                                "Field m_unit_tag_index with tag 0 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_index"),
                                0,
                            ));
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
                            tracing::error!(
                                "Field m_unit_tag_recycle with tag 1 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_recycle"),
                                1,
                            ));
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
                            tracing::error!(
                                "Field m_unit_type_name with tag 2 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_type_name"),
                                2,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_control_player_id"),
                                3,
                            ));
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
                            tracing::error!(
                                "Field m_upkeep_player_id with tag 4 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_upkeep_player_id"),
                                4,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_x"), 5));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_y"), 6));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_unit_tag_index: ok_or_return_missing_field_err!(m_unit_tag_index),
                    m_unit_tag_recycle: ok_or_return_missing_field_err!(m_unit_tag_recycle),
                    m_unit_type_name: ok_or_return_missing_field_err!(m_unit_type_name),
                    m_control_player_id: ok_or_return_missing_field_err!(m_control_player_id),
                    m_upkeep_player_id: ok_or_return_missing_field_err!(m_upkeep_player_id),
                    m_x: ok_or_return_missing_field_err!(m_x),
                    m_y: ok_or_return_missing_field_err!(m_y),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReplayTrackerSUnitDoneEvent {
        pub m_unit_tag_index: u32,
        pub m_unit_tag_recycle: u32,
    }
    impl ReplayTrackerSUnitDoneEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_tag_index(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_index) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_index: {:?}", m_unit_tag_index);
            Ok((tail, u32::try_from(m_unit_tag_index)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_unit_tag_recycle(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_unit_tag_recycle) = tagged_vlq_int(input)?;

            tracing::debug!("m_unit_tag_recycle: {:?}", m_unit_tag_recycle);
            Ok((tail, u32::try_from(m_unit_tag_recycle)?))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSUnitDoneEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            tracing::error!(
                                "Field m_unit_tag_index with tag 0 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_index"),
                                0,
                            ));
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
                            tracing::error!(
                                "Field m_unit_tag_recycle with tag 1 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_unit_tag_recycle"),
                                1,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_unit_tag_index: ok_or_return_missing_field_err!(m_unit_tag_index),
                    m_unit_tag_recycle: ok_or_return_missing_field_err!(m_unit_tag_recycle),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReplayTrackerSUnitPositionsEvent {
        pub m_first_unit_index: u32,
        pub m_items: Vec<i32>,
    }
    impl ReplayTrackerSUnitPositionsEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_first_unit_index(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_first_unit_index) = tagged_vlq_int(input)?;

            tracing::debug!("m_first_unit_index: {:?}", m_first_unit_index);
            Ok((tail, u32::try_from(m_first_unit_index)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_items(input: &[u8]) -> S2ProtoResult<&[u8], Vec<i32>> {
            let (tail, _) = validate_array_tag(input)?;
            let (mut tail, array_length) = parse_vlq_int(tail)?;
            tracing::debug!("Reading array length: {array_length}");
            // compat_count(tagged_vlq_int, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<i32>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = tagged_vlq_int(tail)?;
                tail = new_tail;
                array.push(data.try_into()?);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSUnitPositionsEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            tracing::error!(
                                "Field m_first_unit_index with tag 0 was already provided"
                            );
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_first_unit_index"),
                                0,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_items"), 1));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_first_unit_index: ok_or_return_missing_field_err!(m_first_unit_index),
                    m_items: ok_or_return_missing_field_err!(m_items),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReplayTrackerSPlayerSetupEvent {
        pub m_player_id: u8,
        pub m_type: u32,
        pub m_user_id: Option<u32>,
        pub m_slot_id: Option<u32>,
    }
    impl ReplayTrackerSPlayerSetupEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_player_id(input: &[u8]) -> S2ProtoResult<&[u8], u8> {
            let (tail, m_player_id) = tagged_vlq_int(input)?;

            tracing::debug!("m_player_id: {:?}", m_player_id);
            Ok((tail, u8::try_from(m_player_id)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_type(input: &[u8]) -> S2ProtoResult<&[u8], u32> {
            let (tail, m_type) = tagged_vlq_int(input)?;

            tracing::debug!("m_type: {:?}", m_type);
            Ok((tail, u32::try_from(m_type)?))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_user_id(input: &[u8]) -> S2ProtoResult<&[u8], Option<u32>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_user_id) = if is_provided != 0 {
                let (tail, res) = tagged_vlq_int(tail)?;
                (tail, Some(<_>::try_from(res)?))
            } else {
                (tail, None)
            };
            tracing::debug!("m_user_id: {:?}", m_user_id);
            Ok((tail, m_user_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse_m_slot_id(input: &[u8]) -> S2ProtoResult<&[u8], Option<u32>> {
            let (tail, _) = validate_opt_tag(input)?;
            let (tail, is_provided) = nom::number::complete::u8(tail)?;
            let (tail, m_slot_id) = if is_provided != 0 {
                let (tail, res) = tagged_vlq_int(tail)?;
                (tail, Some(<_>::try_from(res)?))
            } else {
                (tail, None)
            };
            tracing::debug!("m_slot_id: {:?}", m_slot_id);
            Ok((tail, m_slot_id))
        }
        #[tracing::instrument(name="87702::byte_aligned::ReplayTrackerSPlayerSetupEvent::Parse", level = "debug", skip(input), fields(peek = peek_hex(input)))]
        pub fn parse(input: &[u8]) -> S2ProtoResult<&[u8], Self> {
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_player_id"),
                                0,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(String::from("m_type"), 1));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_user_id"),
                                2,
                            ));
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
                            return Err(S2ProtocolError::DuplicateTag(
                                String::from("m_slot_id"),
                                3,
                            ));
                        }
                    }

                    _ => {
                        tracing::error!("Unknown tag {field_tag}");
                        return Err(S2ProtocolError::UnknownTag(field_tag));
                    }
                }
            }
            Ok((
                tail,
                Self {
                    m_player_id: ok_or_return_missing_field_err!(m_player_id),
                    m_type: ok_or_return_missing_field_err!(m_type),
                    m_user_id: ok_or_return_missing_field_err!(m_user_id),
                    m_slot_id: ok_or_return_missing_field_err!(m_slot_id),
                },
            ))
        }
    }

}
pub mod bit_packed {
    //! Generated code from source: ../s2protocol/json/protocol87702.json
    use crate::*;

    #[derive(Debug, PartialEq, Clone)]
    pub struct CFilePath {
        pub value: Vec<u8>,
    }
    impl CFilePath {
        #[tracing::instrument(name="87702::CFilePath::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 11;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct TRaceId {
        pub value: i64,
    }
    impl TRaceId {
        #[tracing::instrument(name="87702::TRaceId::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 8;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct TRaceCount {
        pub value: i64,
    }
    impl TRaceCount {
        #[tracing::instrument(name="87702::TRaceCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 1;
            let num_bits: usize = 8;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct TRacePreference {
        pub m_race: Option<TRaceId>,
    }
    impl TRacePreference {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_race(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<TRaceId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_race) = if is_provided {
                let (tail, res) = TRaceId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_race: {:?}", m_race);
            Ok((tail, m_race))
        }
        #[tracing::instrument(name="87702::bit_packed::TRacePreference::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_race: Option<Option<TRaceId>> = Some(None);
            if let Some(None) = m_race {
                let (new_tail, parsed_m_race) = Self::parse_m_race(tail)?;
                tail = new_tail;
                m_race = Some(parsed_m_race);
            }
            Ok((
                tail,
                Self {
                    m_race: ok_or_return_missing_field_err!(m_race),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CAllowedRaces {
        pub value: i64, // Initially Vec<u8> but these are 8 bits and fits in i64 and easy to
                        // compare with blizzard's python implementation
    }
    impl CAllowedRaces {
        #[tracing::instrument(name="87702::CAllowedRaces::BitArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let bitarray_length_bits: usize = 8;
            let (tail, bitarray_length) = take_n_bits_into_i64(input, bitarray_length_bits)?;
            tracing::debug!("Reading bitarray length: {bitarray_length}");
            let (tail, value) = take_n_bits_into_i64(tail, bitarray_length as usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Int8 {
        pub value: i64,
    }
    impl Int8 {
        #[tracing::instrument(name="87702::Int8::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = -128;
            let num_bits: usize = 8;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Int16 {
        pub value: i64,
    }
    impl Int16 {
        #[tracing::instrument(name="87702::Int16::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = -32768;
            let num_bits: usize = 16;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Int32 {
        pub value: i64,
    }
    impl Int32 {
        #[tracing::instrument(name="87702::Int32::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = -2147483648;
            let num_bits: usize = 32;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Int64 {
        pub value: i64,
    }
    impl Int64 {
        #[tracing::instrument(name="87702::Int64::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = -9223372036854775808;
            let num_bits: usize = 64;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Uint8 {
        pub value: i64,
    }
    impl Uint8 {
        #[tracing::instrument(name="87702::Uint8::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 8;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Uint16 {
        pub value: i64,
    }
    impl Uint16 {
        #[tracing::instrument(name="87702::Uint16::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 16;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Uint32 {
        pub value: i64,
    }
    impl Uint32 {
        #[tracing::instrument(name="87702::Uint32::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 32;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Uint64 {
        pub value: i64,
    }
    impl Uint64 {
        #[tracing::instrument(name="87702::Uint64::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 64;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Uint6 {
        pub value: i64,
    }
    impl Uint6 {
        #[tracing::instrument(name="87702::Uint6::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 6;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Uint14 {
        pub value: i64,
    }
    impl Uint14 {
        #[tracing::instrument(name="87702::Uint14::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 14;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Uint22 {
        pub value: i64,
    }
    impl Uint22 {
        #[tracing::instrument(name="87702::Uint22::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 22;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum SVarUint32 {
        MUint6(Uint6),
        MUint14(Uint14),
        MUint22(Uint22),
        MUint32(Uint32),
    }
    impl SVarUint32 {
        #[tracing::instrument(name="87702::SVarUint32::ChoiceType::parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // ChoiceType:
            // Use the number of elements in the json .fields to calculate how many
            // bits to have unique tags.
            // let num_fields: usize = 4;
            let offset = 0i64;
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, offset, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant tagged '0' for MUint6");
                    let (tail, res) = Uint6::parse(tail)?;
                    Ok((tail, Self::MUint6(res)))
                }
                1 => {
                    tracing::debug!("Variant tagged '1' for MUint14");
                    let (tail, res) = Uint14::parse(tail)?;
                    Ok((tail, Self::MUint14(res)))
                }
                2 => {
                    tracing::debug!("Variant tagged '2' for MUint22");
                    let (tail, res) = Uint22::parse(tail)?;
                    Ok((tail, Self::MUint22(res)))
                }
                3 => {
                    tracing::debug!("Variant tagged '3' for MUint32");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::MUint32(res)))
                }

                _ => {
                    tracing::error!("Unknown variant for tag {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct TUserId {
        pub value: i64,
    }
    impl TUserId {
        #[tracing::instrument(name="87702::TUserId::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 4;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct TUserCount {
        pub value: i64,
    }
    impl TUserCount {
        #[tracing::instrument(name="87702::TUserCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 5;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CCacheHandle {
        pub value: Vec<u8>,
    }
    impl CCacheHandle {
        #[tracing::instrument(name="87702::CCacheHandle::BlobType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, _) = byte_align(input)?;
            let num_bits: usize = 6;
            let (tail, value) = take_bit_array(tail, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CUserName {
        pub value: Vec<u8>,
    }
    impl CUserName {
        #[tracing::instrument(name="87702::CUserName::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 8;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CClanTag {
        pub value: Vec<u8>,
    }
    impl CClanTag {
        #[tracing::instrument(name="87702::CClanTag::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 8;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CHeroHandle {
        pub value: Vec<u8>,
    }
    impl CHeroHandle {
        #[tracing::instrument(name="87702::CHeroHandle::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 10;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CSkinHandle {
        pub value: Vec<u8>,
    }
    impl CSkinHandle {
        #[tracing::instrument(name="87702::CSkinHandle::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 10;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CMountHandle {
        pub value: Vec<u8>,
    }
    impl CMountHandle {
        #[tracing::instrument(name="87702::CMountHandle::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 10;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CArtifactHandle {
        pub value: Vec<u8>,
    }
    impl CArtifactHandle {
        #[tracing::instrument(name="87702::CArtifactHandle::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 10;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CToonHandle {
        pub value: Vec<u8>,
    }
    impl CToonHandle {
        #[tracing::instrument(name="87702::CToonHandle::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 7;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CCommanderHandle {
        pub value: Vec<u8>,
    }
    impl CCommanderHandle {
        #[tracing::instrument(name="87702::CCommanderHandle::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 10;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum EObserve {
        ENone,
        ESpectator,
        EReferee,
    }
    impl EObserve {
        #[tracing::instrument(name="87702::EObserve::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 3
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ENone for value '0'");
                    Ok((tail, Self::ENone))
                }
                1 => {
                    tracing::debug!("Variant ESpectator for value '1'");
                    Ok((tail, Self::ESpectator))
                }
                2 => {
                    tracing::debug!("Variant EReferee for value '2'");
                    Ok((tail, Self::EReferee))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CAllowedObserveTypes {
        pub value: i64, // Initially Vec<u8> but these are 8 bits and fits in i64 and easy to
                        // compare with blizzard's python implementation
    }
    impl CAllowedObserveTypes {
        #[tracing::instrument(name="87702::CAllowedObserveTypes::BitArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let bitarray_length_bits: usize = 2;
            let (tail, bitarray_length) = take_n_bits_into_i64(input, bitarray_length_bits)?;
            tracing::debug!("Reading bitarray length: {bitarray_length}");
            let (tail, value) = take_n_bits_into_i64(tail, bitarray_length as usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct TTeamPreference {
        pub m_team: Option<Uint8>,
    }
    impl TTeamPreference {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_team(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Option<Uint8>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_team) = if is_provided {
                let (tail, res) = Uint8::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_team: {:?}", m_team);
            Ok((tail, m_team))
        }
        #[tracing::instrument(name="87702::bit_packed::TTeamPreference::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_team: Option<Option<Uint8>> = Some(None);
            if let Some(None) = m_team {
                let (new_tail, parsed_m_team) = Self::parse_m_team(tail)?;
                tail = new_tail;
                m_team = Some(parsed_m_team);
            }
            Ok((
                tail,
                Self {
                    m_team: ok_or_return_missing_field_err!(m_team),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct SUserInitialData {
        pub m_name: CUserName,
        pub m_clan_tag: Option<CClanTag>,
        pub m_clan_logo: Option<CCacheHandle>,
        pub m_highest_league: Option<Uint8>,
        pub m_combined_race_levels: Option<Uint32>,
        pub m_random_seed: Uint32,
        pub m_race_preference: TRacePreference,
        pub m_team_preference: TTeamPreference,
        pub m_test_map: bool,
        pub m_test_auto: bool,
        pub m_examine: bool,
        pub m_custom_interface: bool,
        pub m_test_type: Uint32,
        pub m_observe: EObserve,
        pub m_hero: CHeroHandle,
        pub m_skin: CSkinHandle,
        pub m_mount: CMountHandle,
        pub m_toon_handle: CToonHandle,
        pub m_scaled_rating: Option<Int32>,
    }
    impl SUserInitialData {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CUserName> {
            let (tail, m_name) = CUserName::parse(input)?;
            tracing::debug!("m_name: {:?}", str::from_utf8(&m_name.value));
            Ok((tail, m_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_clan_tag(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<CClanTag>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_clan_tag) = if is_provided {
                let (tail, res) = CClanTag::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_clan_tag: {:?}", m_clan_tag);
            Ok((tail, m_clan_tag))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_clan_logo(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<CCacheHandle>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_clan_logo) = if is_provided {
                let (tail, res) = CCacheHandle::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_clan_logo: {:?}", m_clan_logo);
            Ok((tail, m_clan_logo))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_highest_league(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Uint8>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_highest_league) = if is_provided {
                let (tail, res) = Uint8::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_highest_league: {:?}", m_highest_league);
            Ok((tail, m_highest_league))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_combined_race_levels(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Uint32>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_combined_race_levels) = if is_provided {
                let (tail, res) = Uint32::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_combined_race_levels: {:?}", m_combined_race_levels);
            Ok((tail, m_combined_race_levels))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_random_seed(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_random_seed) = Uint32::parse(input)?;
            tracing::debug!("m_random_seed: {:?}", m_random_seed);
            Ok((tail, m_random_seed))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_race_preference(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TRacePreference> {
            let (tail, m_race_preference) = TRacePreference::parse(input)?;
            tracing::debug!("m_race_preference: {:?}", m_race_preference);
            Ok((tail, m_race_preference))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_team_preference(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TTeamPreference> {
            let (tail, m_team_preference) = TTeamPreference::parse(input)?;
            tracing::debug!("m_team_preference: {:?}", m_team_preference);
            Ok((tail, m_team_preference))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_test_map(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_test_map) = parse_bool(input)?;
            tracing::debug!("m_test_map: {:?}", m_test_map);
            Ok((tail, m_test_map))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_test_auto(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_test_auto) = parse_bool(input)?;
            tracing::debug!("m_test_auto: {:?}", m_test_auto);
            Ok((tail, m_test_auto))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_examine(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_examine) = parse_bool(input)?;
            tracing::debug!("m_examine: {:?}", m_examine);
            Ok((tail, m_examine))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_custom_interface(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_custom_interface) = parse_bool(input)?;
            tracing::debug!("m_custom_interface: {:?}", m_custom_interface);
            Ok((tail, m_custom_interface))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_test_type(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_test_type) = Uint32::parse(input)?;
            tracing::debug!("m_test_type: {:?}", m_test_type);
            Ok((tail, m_test_type))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_observe(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), EObserve> {
            let (tail, m_observe) = EObserve::parse(input)?;
            tracing::debug!("m_observe: {:?}", m_observe);
            Ok((tail, m_observe))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_hero(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CHeroHandle> {
            let (tail, m_hero) = CHeroHandle::parse(input)?;
            tracing::debug!("m_hero: {:?}", m_hero);
            Ok((tail, m_hero))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_skin(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CSkinHandle> {
            let (tail, m_skin) = CSkinHandle::parse(input)?;
            tracing::debug!("m_skin: {:?}", m_skin);
            Ok((tail, m_skin))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_mount(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CMountHandle> {
            let (tail, m_mount) = CMountHandle::parse(input)?;
            tracing::debug!("m_mount: {:?}", m_mount);
            Ok((tail, m_mount))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_toon_handle(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CToonHandle> {
            let (tail, m_toon_handle) = CToonHandle::parse(input)?;
            tracing::debug!("m_toon_handle: {:?}", m_toon_handle);
            Ok((tail, m_toon_handle))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_scaled_rating(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Int32>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_scaled_rating) = if is_provided {
                let (tail, res) = Int32::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_scaled_rating: {:?}", m_scaled_rating);
            Ok((tail, m_scaled_rating))
        }
        #[tracing::instrument(name="87702::bit_packed::SUserInitialData::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_name: Option<CUserName> = None;
            let mut m_clan_tag: Option<Option<CClanTag>> = Some(None);
            let mut m_clan_logo: Option<Option<CCacheHandle>> = Some(None);
            let mut m_highest_league: Option<Option<Uint8>> = Some(None);
            let mut m_combined_race_levels: Option<Option<Uint32>> = Some(None);
            let mut m_random_seed: Option<Uint32> = None;
            let mut m_race_preference: Option<TRacePreference> = None;
            let mut m_team_preference: Option<TTeamPreference> = None;
            let mut m_test_map: Option<bool> = None;
            let mut m_test_auto: Option<bool> = None;
            let mut m_examine: Option<bool> = None;
            let mut m_custom_interface: Option<bool> = None;
            let mut m_test_type: Option<Uint32> = None;
            let mut m_observe: Option<EObserve> = None;
            let mut m_hero: Option<CHeroHandle> = None;
            let mut m_skin: Option<CSkinHandle> = None;
            let mut m_mount: Option<CMountHandle> = None;
            let mut m_toon_handle: Option<CToonHandle> = None;
            let mut m_scaled_rating: Option<Option<Int32>> = Some(None);
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            if let Some(None) = m_clan_tag {
                let (new_tail, parsed_m_clan_tag) = Self::parse_m_clan_tag(tail)?;
                tail = new_tail;
                m_clan_tag = Some(parsed_m_clan_tag);
            }
            if let Some(None) = m_clan_logo {
                let (new_tail, parsed_m_clan_logo) = Self::parse_m_clan_logo(tail)?;
                tail = new_tail;
                m_clan_logo = Some(parsed_m_clan_logo);
            }
            if let Some(None) = m_highest_league {
                let (new_tail, parsed_m_highest_league) = Self::parse_m_highest_league(tail)?;
                tail = new_tail;
                m_highest_league = Some(parsed_m_highest_league);
            }
            if let Some(None) = m_combined_race_levels {
                let (new_tail, parsed_m_combined_race_levels) =
                    Self::parse_m_combined_race_levels(tail)?;
                tail = new_tail;
                m_combined_race_levels = Some(parsed_m_combined_race_levels);
            }
            if m_random_seed.is_none() {
                let (new_tail, parsed_m_random_seed) = Self::parse_m_random_seed(tail)?;
                tail = new_tail;
                m_random_seed = Some(parsed_m_random_seed);
            }
            if m_race_preference.is_none() {
                let (new_tail, parsed_m_race_preference) = Self::parse_m_race_preference(tail)?;
                tail = new_tail;
                m_race_preference = Some(parsed_m_race_preference);
            }
            if m_team_preference.is_none() {
                let (new_tail, parsed_m_team_preference) = Self::parse_m_team_preference(tail)?;
                tail = new_tail;
                m_team_preference = Some(parsed_m_team_preference);
            }
            if m_test_map.is_none() {
                let (new_tail, parsed_m_test_map) = Self::parse_m_test_map(tail)?;
                tail = new_tail;
                m_test_map = Some(parsed_m_test_map);
            }
            if m_test_auto.is_none() {
                let (new_tail, parsed_m_test_auto) = Self::parse_m_test_auto(tail)?;
                tail = new_tail;
                m_test_auto = Some(parsed_m_test_auto);
            }
            if m_examine.is_none() {
                let (new_tail, parsed_m_examine) = Self::parse_m_examine(tail)?;
                tail = new_tail;
                m_examine = Some(parsed_m_examine);
            }
            if m_custom_interface.is_none() {
                let (new_tail, parsed_m_custom_interface) = Self::parse_m_custom_interface(tail)?;
                tail = new_tail;
                m_custom_interface = Some(parsed_m_custom_interface);
            }
            if m_test_type.is_none() {
                let (new_tail, parsed_m_test_type) = Self::parse_m_test_type(tail)?;
                tail = new_tail;
                m_test_type = Some(parsed_m_test_type);
            }
            if m_observe.is_none() {
                let (new_tail, parsed_m_observe) = Self::parse_m_observe(tail)?;
                tail = new_tail;
                m_observe = Some(parsed_m_observe);
            }
            if m_hero.is_none() {
                let (new_tail, parsed_m_hero) = Self::parse_m_hero(tail)?;
                tail = new_tail;
                m_hero = Some(parsed_m_hero);
            }
            if m_skin.is_none() {
                let (new_tail, parsed_m_skin) = Self::parse_m_skin(tail)?;
                tail = new_tail;
                m_skin = Some(parsed_m_skin);
            }
            if m_mount.is_none() {
                let (new_tail, parsed_m_mount) = Self::parse_m_mount(tail)?;
                tail = new_tail;
                m_mount = Some(parsed_m_mount);
            }
            if m_toon_handle.is_none() {
                let (new_tail, parsed_m_toon_handle) = Self::parse_m_toon_handle(tail)?;
                tail = new_tail;
                m_toon_handle = Some(parsed_m_toon_handle);
            }
            if let Some(None) = m_scaled_rating {
                let (new_tail, parsed_m_scaled_rating) = Self::parse_m_scaled_rating(tail)?;
                tail = new_tail;
                m_scaled_rating = Some(parsed_m_scaled_rating);
            }
            Ok((
                tail,
                Self {
                    m_name: ok_or_return_missing_field_err!(m_name),
                    m_clan_tag: ok_or_return_missing_field_err!(m_clan_tag),
                    m_clan_logo: ok_or_return_missing_field_err!(m_clan_logo),
                    m_highest_league: ok_or_return_missing_field_err!(m_highest_league),
                    m_combined_race_levels: ok_or_return_missing_field_err!(m_combined_race_levels),
                    m_random_seed: ok_or_return_missing_field_err!(m_random_seed),
                    m_race_preference: ok_or_return_missing_field_err!(m_race_preference),
                    m_team_preference: ok_or_return_missing_field_err!(m_team_preference),
                    m_test_map: ok_or_return_missing_field_err!(m_test_map),
                    m_test_auto: ok_or_return_missing_field_err!(m_test_auto),
                    m_examine: ok_or_return_missing_field_err!(m_examine),
                    m_custom_interface: ok_or_return_missing_field_err!(m_custom_interface),
                    m_test_type: ok_or_return_missing_field_err!(m_test_type),
                    m_observe: ok_or_return_missing_field_err!(m_observe),
                    m_hero: ok_or_return_missing_field_err!(m_hero),
                    m_skin: ok_or_return_missing_field_err!(m_skin),
                    m_mount: ok_or_return_missing_field_err!(m_mount),
                    m_toon_handle: ok_or_return_missing_field_err!(m_toon_handle),
                    m_scaled_rating: ok_or_return_missing_field_err!(m_scaled_rating),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct CUserInitialDataArray {
        pub value: Vec<SUserInitialData>,
    }
    impl CUserInitialDataArray {
        #[tracing::instrument(name="87702::CUserInitialDataArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 5;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(SUserInitialData::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<SUserInitialData>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = SUserInitialData::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum ELeaveReason {
        EUserLeft,
        EUserDropped,
        EUserBanned,
        EUserVictory,
        EUserDefeat,
        EUserTied,
        EUserDesynced,
        EUserOutOfTime,
        EWeWereUnresponsive,
        EWeContinuedAlone,
        EReplayDesynced,
        EUserTimeout,
        EUserDisconnected,
        EUnrecoverable,
        EUserCatchupDesynced,
        ETakeCommandDropped,
    }
    impl ELeaveReason {
        #[tracing::instrument(name="87702::ELeaveReason::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 16
            let num_bits: usize = 4;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EUserLeft for value '0'");
                    Ok((tail, Self::EUserLeft))
                }
                1 => {
                    tracing::debug!("Variant EUserDropped for value '1'");
                    Ok((tail, Self::EUserDropped))
                }
                2 => {
                    tracing::debug!("Variant EUserBanned for value '2'");
                    Ok((tail, Self::EUserBanned))
                }
                3 => {
                    tracing::debug!("Variant EUserVictory for value '3'");
                    Ok((tail, Self::EUserVictory))
                }
                4 => {
                    tracing::debug!("Variant EUserDefeat for value '4'");
                    Ok((tail, Self::EUserDefeat))
                }
                5 => {
                    tracing::debug!("Variant EUserTied for value '5'");
                    Ok((tail, Self::EUserTied))
                }
                6 => {
                    tracing::debug!("Variant EUserDesynced for value '6'");
                    Ok((tail, Self::EUserDesynced))
                }
                7 => {
                    tracing::debug!("Variant EUserOutOfTime for value '7'");
                    Ok((tail, Self::EUserOutOfTime))
                }
                8 => {
                    tracing::debug!("Variant EWeWereUnresponsive for value '8'");
                    Ok((tail, Self::EWeWereUnresponsive))
                }
                9 => {
                    tracing::debug!("Variant EWeContinuedAlone for value '9'");
                    Ok((tail, Self::EWeContinuedAlone))
                }
                10 => {
                    tracing::debug!("Variant EReplayDesynced for value '10'");
                    Ok((tail, Self::EReplayDesynced))
                }
                11 => {
                    tracing::debug!("Variant EUserTimeout for value '11'");
                    Ok((tail, Self::EUserTimeout))
                }
                12 => {
                    tracing::debug!("Variant EUserDisconnected for value '12'");
                    Ok((tail, Self::EUserDisconnected))
                }
                13 => {
                    tracing::debug!("Variant EUnrecoverable for value '13'");
                    Ok((tail, Self::EUnrecoverable))
                }
                14 => {
                    tracing::debug!("Variant EUserCatchupDesynced for value '14'");
                    Ok((tail, Self::EUserCatchupDesynced))
                }
                15 => {
                    tracing::debug!("Variant ETakeCommandDropped for value '15'");
                    Ok((tail, Self::ETakeCommandDropped))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum EReconnectStatus {
        EConnected,
        EReconnected,
        EDisconnected,
        EUnrecoverable,
    }
    impl EReconnectStatus {
        #[tracing::instrument(name="87702::EReconnectStatus::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 4
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EConnected for value '0'");
                    Ok((tail, Self::EConnected))
                }
                1 => {
                    tracing::debug!("Variant EReconnected for value '1'");
                    Ok((tail, Self::EReconnected))
                }
                2 => {
                    tracing::debug!("Variant EDisconnected for value '2'");
                    Ok((tail, Self::EDisconnected))
                }
                3 => {
                    tracing::debug!("Variant EUnrecoverable for value '3'");
                    Ok((tail, Self::EUnrecoverable))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct SVersion {
        pub m_flags: Uint8,
        pub m_major: Uint8,
        pub m_minor: Uint8,
        pub m_revision: Uint8,
        pub m_build: Uint32,
        pub m_base_build: Uint32,
    }
    impl SVersion {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_flags(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_flags) = Uint8::parse(input)?;
            tracing::debug!("m_flags: {:?}", m_flags);
            Ok((tail, m_flags))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_major(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_major) = Uint8::parse(input)?;
            tracing::debug!("m_major: {:?}", m_major);
            Ok((tail, m_major))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_minor(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_minor) = Uint8::parse(input)?;
            tracing::debug!("m_minor: {:?}", m_minor);
            Ok((tail, m_minor))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_revision(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_revision) = Uint8::parse(input)?;
            tracing::debug!("m_revision: {:?}", m_revision);
            Ok((tail, m_revision))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_build(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_build) = Uint32::parse(input)?;
            tracing::debug!("m_build: {:?}", m_build);
            Ok((tail, m_build))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_base_build(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_base_build) = Uint32::parse(input)?;
            tracing::debug!("m_base_build: {:?}", m_base_build);
            Ok((tail, m_base_build))
        }
        #[tracing::instrument(name="87702::bit_packed::SVersion::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_flags: Option<Uint8> = None;
            let mut m_major: Option<Uint8> = None;
            let mut m_minor: Option<Uint8> = None;
            let mut m_revision: Option<Uint8> = None;
            let mut m_build: Option<Uint32> = None;
            let mut m_base_build: Option<Uint32> = None;
            if m_flags.is_none() {
                let (new_tail, parsed_m_flags) = Self::parse_m_flags(tail)?;
                tail = new_tail;
                m_flags = Some(parsed_m_flags);
            }
            if m_major.is_none() {
                let (new_tail, parsed_m_major) = Self::parse_m_major(tail)?;
                tail = new_tail;
                m_major = Some(parsed_m_major);
            }
            if m_minor.is_none() {
                let (new_tail, parsed_m_minor) = Self::parse_m_minor(tail)?;
                tail = new_tail;
                m_minor = Some(parsed_m_minor);
            }
            if m_revision.is_none() {
                let (new_tail, parsed_m_revision) = Self::parse_m_revision(tail)?;
                tail = new_tail;
                m_revision = Some(parsed_m_revision);
            }
            if m_build.is_none() {
                let (new_tail, parsed_m_build) = Self::parse_m_build(tail)?;
                tail = new_tail;
                m_build = Some(parsed_m_build);
            }
            if m_base_build.is_none() {
                let (new_tail, parsed_m_base_build) = Self::parse_m_base_build(tail)?;
                tail = new_tail;
                m_base_build = Some(parsed_m_base_build);
            }
            Ok((
                tail,
                Self {
                    m_flags: ok_or_return_missing_field_err!(m_flags),
                    m_major: ok_or_return_missing_field_err!(m_major),
                    m_minor: ok_or_return_missing_field_err!(m_minor),
                    m_revision: ok_or_return_missing_field_err!(m_revision),
                    m_build: ok_or_return_missing_field_err!(m_build),
                    m_base_build: ok_or_return_missing_field_err!(m_base_build),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Smd5 {
        pub m_data_deprecated: Option<Vec<Uint8>>,
        pub m_data: Vec<u8>,
    }
    impl Smd5 {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_data_deprecated(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Vec<Uint8>>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_data_deprecated) = if is_provided {
                let (mut tail, array_length) = take_n_bits_into_i64(input, 5)?;
                let array_length = array_length as usize;
                tracing::debug!("Reading array length: {array_length}");
                let max_initial_capacity =
                    MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<u8>().max(1);
                let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
                for _ in 0..array_length {
                    let (new_tail, data) = Uint8::parse(tail)?;
                    tail = new_tail;
                    array.push(data);
                }
                (tail, Some(array))
            } else {
                (tail, None)
            };
            tracing::debug!("m_data_deprecated: {:?}", m_data_deprecated);
            Ok((tail, m_data_deprecated))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_data(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 5)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::Smd5::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_data_deprecated: Option<Option<Vec<Uint8>>> = Some(None);
            let mut m_data: Option<Vec<u8>> = None;
            if let Some(None) = m_data_deprecated {
                let (new_tail, parsed_m_data_deprecated) = Self::parse_m_data_deprecated(tail)?;
                tail = new_tail;
                m_data_deprecated = Some(parsed_m_data_deprecated);
            }
            if m_data.is_none() {
                let (new_tail, parsed_m_data) = Self::parse_m_data(tail)?;
                tail = new_tail;
                m_data = Some(parsed_m_data);
            }
            Ok((
                tail,
                Self {
                    m_data_deprecated: ok_or_return_missing_field_err!(m_data_deprecated),
                    m_data: ok_or_return_missing_field_err!(m_data),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTColorId {
        pub value: i64,
    }
    impl GameTColorId {
        #[tracing::instrument(name="87702::GameTColorId::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 5;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTColorCount {
        pub value: i64,
    }
    impl GameTColorCount {
        #[tracing::instrument(name="87702::GameTColorCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 6;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTColorPreference {
        pub m_color: Option<GameTColorId>,
    }
    impl GameTColorPreference {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_color(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTColorId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_color) = if is_provided {
                let (tail, res) = GameTColorId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_color: {:?}", m_color);
            Ok((tail, m_color))
        }
        #[tracing::instrument(name="87702::bit_packed::GameTColorPreference::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_color: Option<Option<GameTColorId>> = Some(None);
            if let Some(None) = m_color {
                let (new_tail, parsed_m_color) = Self::parse_m_color(tail)?;
                tail = new_tail;
                m_color = Some(parsed_m_color);
            }
            Ok((
                tail,
                Self {
                    m_color: ok_or_return_missing_field_err!(m_color),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCAllowedColors {
        pub value: i64, // Initially Vec<u8> but these are 8 bits and fits in i64 and easy to
                        // compare with blizzard's python implementation
    }
    impl GameCAllowedColors {
        #[tracing::instrument(name="87702::GameCAllowedColors::BitArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let bitarray_length_bits: usize = 6;
            let (tail, bitarray_length) = take_n_bits_into_i64(input, bitarray_length_bits)?;
            tracing::debug!("Reading bitarray_length[{bitarray_length_bits}]: {bitarray_length}");
            let (tail, value) = take_n_bits_into_i64(tail, bitarray_length as usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameESynchronous {
        ELocal,
        ESession(GameSSetLobbySlotEvent),
        EGame(GameSBankFileEvent),
    }
    impl GameESynchronous {
        #[tracing::instrument(name="87702::GameESynchronous::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 3
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ELocal for value '0'");
                    Ok((tail, Self::ELocal))
                }
                1 => {
                    tracing::debug!("Variant ESession for value '1'");

                    let (tail, res) = GameSSetLobbySlotEvent::parse(tail)?;
                    Ok((tail, Self::ESession(res)))
                }
                2 => {
                    tracing::debug!("Variant EGame for value '2'");

                    let (tail, res) = GameSBankFileEvent::parse(tail)?;
                    Ok((tail, Self::EGame(res)))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameESynthesized {
        ESynthesized(GameSDropOurselvesEvent),
        ENotSynthesized(GameSSetLobbySlotEvent),
    }
    impl GameESynthesized {
        #[tracing::instrument(name="87702::GameESynthesized::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 2
            let num_bits: usize = 1;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ESynthesized for value '0'");

                    let (tail, res) = GameSDropOurselvesEvent::parse(tail)?;
                    Ok((tail, Self::ESynthesized(res)))
                }
                1 => {
                    tracing::debug!("Variant ENotSynthesized for value '1'");

                    let (tail, res) = GameSSetLobbySlotEvent::parse(tail)?;
                    Ok((tail, Self::ENotSynthesized(res)))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEDebug {
        EDebug(GameSPickMapTagEvent),
        ENotDebug(GameSSetLobbySlotEvent),
    }
    impl GameEDebug {
        #[tracing::instrument(name="87702::GameEDebug::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 2
            let num_bits: usize = 1;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EDebug for value '0'");

                    let (tail, res) = GameSPickMapTagEvent::parse(tail)?;
                    Ok((tail, Self::EDebug(res)))
                }
                1 => {
                    tracing::debug!("Variant ENotDebug for value '1'");

                    let (tail, res) = GameSSetLobbySlotEvent::parse(tail)?;
                    Ok((tail, Self::ENotDebug(res)))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEHijackMethod {
        ERecover,
        ETakeCommand,
    }
    impl GameEHijackMethod {
        #[tracing::instrument(name="87702::GameEHijackMethod::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 2
            let num_bits: usize = 1;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ERecover for value '0'");
                    Ok((tail, Self::ERecover))
                }
                1 => {
                    tracing::debug!("Variant ETakeCommand for value '1'");
                    Ok((tail, Self::ETakeCommand))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTQueryId {
        pub value: Uint16,
    }
    impl GameTQueryId {
        #[tracing::instrument(name="87702::GameTQueryId::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEEventId {
        ESetLobbySlot(GameSSetLobbySlotEvent),
        EDropUser(GameSDropUserEvent),
        EStartGame(GameSStartGameEvent),
        EDropOurselves(GameSDropOurselvesEvent),
        EUserFinishedLoading(GameSUserFinishedLoadingEvent),
        EUserFinishedLoadingSync(GameSUserFinishedLoadingSyncEvent),
        ESetGameDuration(GameSSetGameDurationEvent),
        EUserOptions(GameSUserOptionsEvent),
        EPickMapTag(GameSPickMapTagEvent),
        ETurn(GameSTurnEvent),
        EBankFile(GameSBankFileEvent),
        EBankSection(GameSBankSectionEvent),
        EBankKey(GameSBankKeyEvent),
        EBankValue(GameSBankValueEvent),
        EBankSignature(GameSBankSignatureEvent),
        ECameraSave(GameSCameraSaveEvent),
        EPauseGame(GameSPauseGameEvent),
        EUnpauseGame(GameSUnpauseGameEvent),
        ESingleStepGame(GameSSingleStepGameEvent),
        ESetGameSpeed(GameSSetGameSpeedEvent),
        EAddGameSpeed(GameSAddGameSpeedEvent),
        EReplayJump(GameSReplayJumpEvent),
        ESaveGame(GameSSaveGameEvent),
        ESaveGameDone(GameSSaveGameDoneEvent),
        ELoadGameDone(GameSLoadGameDoneEvent),
        ESessionCheat(GameSSessionCheatEvent),
        ECommandManagerReset(GameSCommandManagerResetEvent),
        EGameCheat(GameSGameCheatEvent),
        ECmd(GameSCmdEvent),
        ESelectionDelta(GameSSelectionDeltaEvent),
        EControlGroupUpdate(GameSControlGroupUpdateEvent),
        ESelectionSyncCheck(GameSSelectionSyncCheckEvent),
        EResourceTrade(GameSResourceTradeEvent),
        ETriggerChatMessage(GameSTriggerChatMessageEvent),
        EAiCommunicate(GameSaiCommunicateEvent),
        ESetAbsoluteGameSpeed(GameSSetAbsoluteGameSpeedEvent),
        EAddAbsoluteGameSpeed(GameSAddAbsoluteGameSpeedEvent),
        ETriggerPing(GameSTriggerPingEvent),
        EBroadcastCheat(GameSBroadcastCheatEvent),
        EAlliance(GameSAllianceEvent),
        EUnitClick(GameSUnitClickEvent),
        EUnitHighlight(GameSUnitHighlightEvent),
        ETriggerReplySelected(GameSTriggerReplySelectedEvent),
        EHijackReplaySession(GameSHijackReplaySessionEvent),
        EHijackReplayGame(GameSHijackReplayGameEvent),
        ETriggerSkipped(GameSTriggerSkippedEvent),
        ETriggerSoundLengthQuery(GameSTriggerSoundLengthQueryEvent),
        ETriggerSoundOffset(GameSTriggerSoundOffsetEvent),
        ETriggerTransmissionOffset(GameSTriggerTransmissionOffsetEvent),
        ETriggerTransmissionComplete(GameSTriggerTransmissionCompleteEvent),
        ECameraUpdate(GameSCameraUpdateEvent),
        ETriggerAbortMission(GameSTriggerAbortMissionEvent),
        ETriggerPurchaseMade(GameSTriggerPurchaseMadeEvent),
        ETriggerPurchaseExit(GameSTriggerPurchaseExitEvent),
        ETriggerPlanetMissionLaunched(GameSTriggerPlanetMissionLaunchedEvent),
        ETriggerPlanetPanelCanceled(GameSTriggerPlanetPanelCanceledEvent),
        ETriggerDialogControl(GameSTriggerDialogControlEvent),
        ETriggerSoundLengthSync(GameSTriggerSoundLengthSyncEvent),
        ETriggerConversationSkipped(GameSTriggerConversationSkippedEvent),
        ETriggerMouseClicked(GameSTriggerMouseClickedEvent),
        ETriggerMouseMoved(GameSTriggerMouseMovedEvent),
        EAchievementAwarded(GameSAchievementAwardedEvent),
        ETriggerHotkeyPressed(GameSTriggerHotkeyPressedEvent),
        ETriggerTargetModeUpdate(GameSTriggerTargetModeUpdateEvent),
        ETriggerPlanetPanelPanelReplay(GameSTriggerPlanetPanelReplayEvent),
        ETriggerSoundtrackDone(GameSTriggerSoundtrackDoneEvent),
        ETriggerPlanetMissionSelected(GameSTriggerPlanetMissionSelectedEvent),
        ETriggerKeyPressed(GameSTriggerKeyPressedEvent),
        ETriggerMovieFunction(GameSTriggerMovieFunctionEvent),
        ETriggerPlanetPanelPanelBirthComplete(GameSTriggerPlanetPanelBirthCompleteEvent),
        ETriggerPlanetPanelPanelDeathComplete(GameSTriggerPlanetPanelDeathCompleteEvent),
        EResourceRequest(GameSResourceRequestEvent),
        EResourceRequestFulfill(GameSResourceRequestFulfillEvent),
        EResourceRequestCancel(GameSResourceRequestCancelEvent),
        ETriggerResearchPanelExit(GameSTriggerResearchPanelExitEvent),
        ETriggerResearchPanelPurchase(GameSTriggerResearchPanelPurchaseEvent),
        ETriggerResearchPanelSelectionChanged(GameSTriggerResearchPanelSelectionChangedEvent),
        ETriggerCommandError(GameSTriggerCommandErrorEvent),
        ETriggerMercenaryPanelExit(GameSTriggerMercenaryPanelExitEvent),
        ETriggerMercenaryPanelPurchase(GameSTriggerMercenaryPanelPurchaseEvent),
        ETriggerMercenaryPanelSelectionChanged(GameSTriggerMercenaryPanelSelectionChangedEvent),
        ETriggerVictoryPanelExit(GameSTriggerVictoryPanelExitEvent),
        ETriggerBattleReportPanelExit(GameSTriggerBattleReportPanelExitEvent),
        ETriggerBattleReportPanelPlayMission(GameSTriggerBattleReportPanelPlayMissionEvent),
        ETriggerBattleReportPanelPlayScene(GameSTriggerBattleReportPanelPlaySceneEvent),
        ETriggerBattleReportSelectionChanged(GameSTriggerBattleReportPanelSelectionChangedEvent),
        ETriggerVictoryPanelPlayMissionAgain(GameSTriggerVictoryPanelPlayMissionAgainEvent),
        ETriggerMovieStarted(GameSTriggerMovieStartedEvent),
        ETriggerMovieFinished(GameSTriggerMovieFinishedEvent),
        EDecrementGameTimeRemaining(GameSDecrementGameTimeRemainingEvent),
        ETriggerPortraitLoaded(GameSTriggerPortraitLoadedEvent),
        ETriggerQueryDialogDismissed(GameSTriggerCustomDialogDismissedEvent),
        ETriggerGameMenuItemSelected(GameSTriggerGameMenuItemSelectedEvent),
        ETriggerMouseWheel(GameSTriggerMouseWheelEvent),
        ETriggerPurchasePanelSelectedPurchaseItemChanged(
            GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent,
        ),
        ETriggerPurchasePanelSelectedPurchaseCategoryChanged(
            GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent,
        ),
        ETriggerButtonPressed(GameSTriggerButtonPressedEvent),
        ETriggerGameCreditsFinished(GameSTriggerGameCreditsFinishedEvent),
        ETriggerCutsceneBookmarkFired(GameSTriggerCutsceneBookmarkFiredEvent),
        ETriggerCutsceneEndSceneFired(GameSTriggerCutsceneEndSceneFiredEvent),
        ETriggerCutsceneConversationLine(GameSTriggerCutsceneConversationLineEvent),
        ETriggerCutsceneConversationLineMissing(GameSTriggerCutsceneConversationLineMissingEvent),
        EGameUserLeave(GameSGameUserLeaveEvent),
        EGameUserJoin(GameSGameUserJoinEvent),
        ECommandManagerState(GameSCommandManagerStateEvent),
        ECmdUpdateTargetPoint(GameSCmdUpdateTargetPointEvent),
        ECmdUpdateTargetUnit(GameSCmdUpdateTargetUnitEvent),
        ETriggerAnimLengthQueryByName(GameSTriggerAnimLengthQueryByNameEvent),
        ETriggerAnimLengthQueryByProps(GameSTriggerAnimLengthQueryByPropsEvent),
        ETriggerAnimOffset(GameSTriggerAnimOffsetEvent),
        ECatalogModify(GameSCatalogModifyEvent),
        EHeroTalentTreeSelected(GameSHeroTalentTreeSelectedEvent),
        ETriggerProfilerLoggingFinished(GameSTriggerProfilerLoggingFinishedEvent),
        EHeroTalentTreeSelectionPanelToggled(GameSHeroTalentTreeSelectionPanelToggledEvent),
        EMuteUserChanged(GameSMuteChatEvent),
        EConvertToReplaySession(GameSConvertToReplaySessionEvent),
        ESetSyncLoadingTime(GameSSetSyncLoadingTimeEvent),
        ESetSyncPlayingTime(GameSSetSyncPlayingTimeEvent),
        EPeerSetSyncLoadingTime(GameSPeerSetSyncLoadingTimeEvent),
        EPeerSetSyncPlayingTime(GameSPeerSetSyncPlayingTimeEvent),
    }
    impl GameEEventId {
        #[tracing::instrument(name="87702::GameEEventId::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 120
            let num_bits: usize = 7;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ESetLobbySlot for value '0'");

                    let (tail, res) = GameSSetLobbySlotEvent::parse(tail)?;
                    Ok((tail, Self::ESetLobbySlot(res)))
                }
                1 => {
                    tracing::debug!("Variant EDropUser for value '1'");

                    let (tail, res) = GameSDropUserEvent::parse(tail)?;
                    Ok((tail, Self::EDropUser(res)))
                }
                2 => {
                    tracing::debug!("Variant EStartGame for value '2'");

                    let (tail, res) = GameSStartGameEvent::parse(tail)?;
                    Ok((tail, Self::EStartGame(res)))
                }
                3 => {
                    tracing::debug!("Variant EDropOurselves for value '3'");

                    let (tail, res) = GameSDropOurselvesEvent::parse(tail)?;
                    Ok((tail, Self::EDropOurselves(res)))
                }
                4 => {
                    tracing::debug!("Variant EUserFinishedLoading for value '4'");

                    let (tail, res) = GameSUserFinishedLoadingEvent::parse(tail)?;
                    Ok((tail, Self::EUserFinishedLoading(res)))
                }
                5 => {
                    tracing::debug!("Variant EUserFinishedLoadingSync for value '5'");

                    let (tail, res) = GameSUserFinishedLoadingSyncEvent::parse(tail)?;
                    Ok((tail, Self::EUserFinishedLoadingSync(res)))
                }
                6 => {
                    tracing::debug!("Variant ESetGameDuration for value '6'");

                    let (tail, res) = GameSSetGameDurationEvent::parse(tail)?;
                    Ok((tail, Self::ESetGameDuration(res)))
                }
                7 => {
                    tracing::debug!("Variant EUserOptions for value '7'");

                    let (tail, res) = GameSUserOptionsEvent::parse(tail)?;
                    Ok((tail, Self::EUserOptions(res)))
                }
                114 => {
                    tracing::debug!("Variant EPickMapTag for value '114'");

                    let (tail, res) = GameSPickMapTagEvent::parse(tail)?;
                    Ok((tail, Self::EPickMapTag(res)))
                }
                8 => {
                    tracing::debug!("Variant ETurn for value '8'");

                    let (tail, res) = GameSTurnEvent::parse(tail)?;
                    Ok((tail, Self::ETurn(res)))
                }
                9 => {
                    tracing::debug!("Variant EBankFile for value '9'");

                    let (tail, res) = GameSBankFileEvent::parse(tail)?;
                    Ok((tail, Self::EBankFile(res)))
                }
                10 => {
                    tracing::debug!("Variant EBankSection for value '10'");

                    let (tail, res) = GameSBankSectionEvent::parse(tail)?;
                    Ok((tail, Self::EBankSection(res)))
                }
                11 => {
                    tracing::debug!("Variant EBankKey for value '11'");

                    let (tail, res) = GameSBankKeyEvent::parse(tail)?;
                    Ok((tail, Self::EBankKey(res)))
                }
                12 => {
                    tracing::debug!("Variant EBankValue for value '12'");

                    let (tail, res) = GameSBankValueEvent::parse(tail)?;
                    Ok((tail, Self::EBankValue(res)))
                }
                13 => {
                    tracing::debug!("Variant EBankSignature for value '13'");

                    let (tail, res) = GameSBankSignatureEvent::parse(tail)?;
                    Ok((tail, Self::EBankSignature(res)))
                }
                14 => {
                    tracing::debug!("Variant ECameraSave for value '14'");

                    let (tail, res) = GameSCameraSaveEvent::parse(tail)?;
                    Ok((tail, Self::ECameraSave(res)))
                }
                15 => {
                    tracing::debug!("Variant EPauseGame for value '15'");

                    let (tail, res) = GameSPauseGameEvent::parse(tail)?;
                    Ok((tail, Self::EPauseGame(res)))
                }
                16 => {
                    tracing::debug!("Variant EUnpauseGame for value '16'");

                    let (tail, res) = GameSUnpauseGameEvent::parse(tail)?;
                    Ok((tail, Self::EUnpauseGame(res)))
                }
                17 => {
                    tracing::debug!("Variant ESingleStepGame for value '17'");

                    let (tail, res) = GameSSingleStepGameEvent::parse(tail)?;
                    Ok((tail, Self::ESingleStepGame(res)))
                }
                18 => {
                    tracing::debug!("Variant ESetGameSpeed for value '18'");

                    let (tail, res) = GameSSetGameSpeedEvent::parse(tail)?;
                    Ok((tail, Self::ESetGameSpeed(res)))
                }
                19 => {
                    tracing::debug!("Variant EAddGameSpeed for value '19'");

                    let (tail, res) = GameSAddGameSpeedEvent::parse(tail)?;
                    Ok((tail, Self::EAddGameSpeed(res)))
                }
                20 => {
                    tracing::debug!("Variant EReplayJump for value '20'");

                    let (tail, res) = GameSReplayJumpEvent::parse(tail)?;
                    Ok((tail, Self::EReplayJump(res)))
                }
                21 => {
                    tracing::debug!("Variant ESaveGame for value '21'");

                    let (tail, res) = GameSSaveGameEvent::parse(tail)?;
                    Ok((tail, Self::ESaveGame(res)))
                }
                22 => {
                    tracing::debug!("Variant ESaveGameDone for value '22'");

                    let (tail, res) = GameSSaveGameDoneEvent::parse(tail)?;
                    Ok((tail, Self::ESaveGameDone(res)))
                }
                23 => {
                    tracing::debug!("Variant ELoadGameDone for value '23'");

                    let (tail, res) = GameSLoadGameDoneEvent::parse(tail)?;
                    Ok((tail, Self::ELoadGameDone(res)))
                }
                24 => {
                    tracing::debug!("Variant ESessionCheat for value '24'");

                    let (tail, res) = GameSSessionCheatEvent::parse(tail)?;
                    Ok((tail, Self::ESessionCheat(res)))
                }
                25 => {
                    tracing::debug!("Variant ECommandManagerReset for value '25'");

                    let (tail, res) = GameSCommandManagerResetEvent::parse(tail)?;
                    Ok((tail, Self::ECommandManagerReset(res)))
                }
                26 => {
                    tracing::debug!("Variant EGameCheat for value '26'");

                    let (tail, res) = GameSGameCheatEvent::parse(tail)?;
                    Ok((tail, Self::EGameCheat(res)))
                }
                27 => {
                    tracing::debug!("Variant ECmd for value '27'");

                    let (tail, res) = GameSCmdEvent::parse(tail)?;
                    Ok((tail, Self::ECmd(res)))
                }
                28 => {
                    tracing::debug!("Variant ESelectionDelta for value '28'");

                    let (tail, res) = GameSSelectionDeltaEvent::parse(tail)?;
                    Ok((tail, Self::ESelectionDelta(res)))
                }
                29 => {
                    tracing::debug!("Variant EControlGroupUpdate for value '29'");

                    let (tail, res) = GameSControlGroupUpdateEvent::parse(tail)?;
                    Ok((tail, Self::EControlGroupUpdate(res)))
                }
                30 => {
                    tracing::debug!("Variant ESelectionSyncCheck for value '30'");

                    let (tail, res) = GameSSelectionSyncCheckEvent::parse(tail)?;
                    Ok((tail, Self::ESelectionSyncCheck(res)))
                }
                31 => {
                    tracing::debug!("Variant EResourceTrade for value '31'");

                    let (tail, res) = GameSResourceTradeEvent::parse(tail)?;
                    Ok((tail, Self::EResourceTrade(res)))
                }
                32 => {
                    tracing::debug!("Variant ETriggerChatMessage for value '32'");

                    let (tail, res) = GameSTriggerChatMessageEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerChatMessage(res)))
                }
                33 => {
                    tracing::debug!("Variant EAiCommunicate for value '33'");

                    let (tail, res) = GameSaiCommunicateEvent::parse(tail)?;
                    Ok((tail, Self::EAiCommunicate(res)))
                }
                34 => {
                    tracing::debug!("Variant ESetAbsoluteGameSpeed for value '34'");

                    let (tail, res) = GameSSetAbsoluteGameSpeedEvent::parse(tail)?;
                    Ok((tail, Self::ESetAbsoluteGameSpeed(res)))
                }
                35 => {
                    tracing::debug!("Variant EAddAbsoluteGameSpeed for value '35'");

                    let (tail, res) = GameSAddAbsoluteGameSpeedEvent::parse(tail)?;
                    Ok((tail, Self::EAddAbsoluteGameSpeed(res)))
                }
                36 => {
                    tracing::debug!("Variant ETriggerPing for value '36'");

                    let (tail, res) = GameSTriggerPingEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPing(res)))
                }
                37 => {
                    tracing::debug!("Variant EBroadcastCheat for value '37'");

                    let (tail, res) = GameSBroadcastCheatEvent::parse(tail)?;
                    Ok((tail, Self::EBroadcastCheat(res)))
                }
                38 => {
                    tracing::debug!("Variant EAlliance for value '38'");

                    let (tail, res) = GameSAllianceEvent::parse(tail)?;
                    Ok((tail, Self::EAlliance(res)))
                }
                39 => {
                    tracing::debug!("Variant EUnitClick for value '39'");

                    let (tail, res) = GameSUnitClickEvent::parse(tail)?;
                    Ok((tail, Self::EUnitClick(res)))
                }
                40 => {
                    tracing::debug!("Variant EUnitHighlight for value '40'");

                    let (tail, res) = GameSUnitHighlightEvent::parse(tail)?;
                    Ok((tail, Self::EUnitHighlight(res)))
                }
                41 => {
                    tracing::debug!("Variant ETriggerReplySelected for value '41'");

                    let (tail, res) = GameSTriggerReplySelectedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerReplySelected(res)))
                }
                42 => {
                    tracing::debug!("Variant EHijackReplaySession for value '42'");

                    let (tail, res) = GameSHijackReplaySessionEvent::parse(tail)?;
                    Ok((tail, Self::EHijackReplaySession(res)))
                }
                43 => {
                    tracing::debug!("Variant EHijackReplayGame for value '43'");

                    let (tail, res) = GameSHijackReplayGameEvent::parse(tail)?;
                    Ok((tail, Self::EHijackReplayGame(res)))
                }
                44 => {
                    tracing::debug!("Variant ETriggerSkipped for value '44'");

                    let (tail, res) = GameSTriggerSkippedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSkipped(res)))
                }
                45 => {
                    tracing::debug!("Variant ETriggerSoundLengthQuery for value '45'");

                    let (tail, res) = GameSTriggerSoundLengthQueryEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSoundLengthQuery(res)))
                }
                46 => {
                    tracing::debug!("Variant ETriggerSoundOffset for value '46'");

                    let (tail, res) = GameSTriggerSoundOffsetEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSoundOffset(res)))
                }
                47 => {
                    tracing::debug!("Variant ETriggerTransmissionOffset for value '47'");

                    let (tail, res) = GameSTriggerTransmissionOffsetEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerTransmissionOffset(res)))
                }
                48 => {
                    tracing::debug!("Variant ETriggerTransmissionComplete for value '48'");

                    let (tail, res) = GameSTriggerTransmissionCompleteEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerTransmissionComplete(res)))
                }
                49 => {
                    tracing::debug!("Variant ECameraUpdate for value '49'");

                    let (tail, res) = GameSCameraUpdateEvent::parse(tail)?;
                    Ok((tail, Self::ECameraUpdate(res)))
                }
                50 => {
                    tracing::debug!("Variant ETriggerAbortMission for value '50'");

                    let (tail, res) = GameSTriggerAbortMissionEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerAbortMission(res)))
                }
                51 => {
                    tracing::debug!("Variant ETriggerPurchaseMade for value '51'");

                    let (tail, res) = GameSTriggerPurchaseMadeEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPurchaseMade(res)))
                }
                52 => {
                    tracing::debug!("Variant ETriggerPurchaseExit for value '52'");

                    let (tail, res) = GameSTriggerPurchaseExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPurchaseExit(res)))
                }
                53 => {
                    tracing::debug!("Variant ETriggerPlanetMissionLaunched for value '53'");

                    let (tail, res) = GameSTriggerPlanetMissionLaunchedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetMissionLaunched(res)))
                }
                54 => {
                    tracing::debug!("Variant ETriggerPlanetPanelCanceled for value '54'");

                    let (tail, res) = GameSTriggerPlanetPanelCanceledEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetPanelCanceled(res)))
                }
                55 => {
                    tracing::debug!("Variant ETriggerDialogControl for value '55'");

                    let (tail, res) = GameSTriggerDialogControlEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerDialogControl(res)))
                }
                56 => {
                    tracing::debug!("Variant ETriggerSoundLengthSync for value '56'");

                    let (tail, res) = GameSTriggerSoundLengthSyncEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSoundLengthSync(res)))
                }
                57 => {
                    tracing::debug!("Variant ETriggerConversationSkipped for value '57'");

                    let (tail, res) = GameSTriggerConversationSkippedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerConversationSkipped(res)))
                }
                58 => {
                    tracing::debug!("Variant ETriggerMouseClicked for value '58'");

                    let (tail, res) = GameSTriggerMouseClickedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMouseClicked(res)))
                }
                59 => {
                    tracing::debug!("Variant ETriggerMouseMoved for value '59'");

                    let (tail, res) = GameSTriggerMouseMovedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMouseMoved(res)))
                }
                60 => {
                    tracing::debug!("Variant EAchievementAwarded for value '60'");

                    let (tail, res) = GameSAchievementAwardedEvent::parse(tail)?;
                    Ok((tail, Self::EAchievementAwarded(res)))
                }
                61 => {
                    tracing::debug!("Variant ETriggerHotkeyPressed for value '61'");

                    let (tail, res) = GameSTriggerHotkeyPressedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerHotkeyPressed(res)))
                }
                62 => {
                    tracing::debug!("Variant ETriggerTargetModeUpdate for value '62'");

                    let (tail, res) = GameSTriggerTargetModeUpdateEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerTargetModeUpdate(res)))
                }
                63 => {
                    tracing::debug!("Variant ETriggerPlanetPanelPanelReplay for value '63'");

                    let (tail, res) = GameSTriggerPlanetPanelReplayEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetPanelPanelReplay(res)))
                }
                64 => {
                    tracing::debug!("Variant ETriggerSoundtrackDone for value '64'");

                    let (tail, res) = GameSTriggerSoundtrackDoneEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerSoundtrackDone(res)))
                }
                65 => {
                    tracing::debug!("Variant ETriggerPlanetMissionSelected for value '65'");

                    let (tail, res) = GameSTriggerPlanetMissionSelectedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetMissionSelected(res)))
                }
                66 => {
                    tracing::debug!("Variant ETriggerKeyPressed for value '66'");

                    let (tail, res) = GameSTriggerKeyPressedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerKeyPressed(res)))
                }
                67 => {
                    tracing::debug!("Variant ETriggerMovieFunction for value '67'");

                    let (tail, res) = GameSTriggerMovieFunctionEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMovieFunction(res)))
                }
                68 => {
                    tracing::debug!("Variant ETriggerPlanetPanelPanelBirthComplete for value '68'");

                    let (tail, res) = GameSTriggerPlanetPanelBirthCompleteEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetPanelPanelBirthComplete(res)))
                }
                69 => {
                    tracing::debug!("Variant ETriggerPlanetPanelPanelDeathComplete for value '69'");

                    let (tail, res) = GameSTriggerPlanetPanelDeathCompleteEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPlanetPanelPanelDeathComplete(res)))
                }
                70 => {
                    tracing::debug!("Variant EResourceRequest for value '70'");

                    let (tail, res) = GameSResourceRequestEvent::parse(tail)?;
                    Ok((tail, Self::EResourceRequest(res)))
                }
                71 => {
                    tracing::debug!("Variant EResourceRequestFulfill for value '71'");

                    let (tail, res) = GameSResourceRequestFulfillEvent::parse(tail)?;
                    Ok((tail, Self::EResourceRequestFulfill(res)))
                }
                72 => {
                    tracing::debug!("Variant EResourceRequestCancel for value '72'");

                    let (tail, res) = GameSResourceRequestCancelEvent::parse(tail)?;
                    Ok((tail, Self::EResourceRequestCancel(res)))
                }
                73 => {
                    tracing::debug!("Variant ETriggerResearchPanelExit for value '73'");

                    let (tail, res) = GameSTriggerResearchPanelExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerResearchPanelExit(res)))
                }
                74 => {
                    tracing::debug!("Variant ETriggerResearchPanelPurchase for value '74'");

                    let (tail, res) = GameSTriggerResearchPanelPurchaseEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerResearchPanelPurchase(res)))
                }
                75 => {
                    tracing::debug!("Variant ETriggerResearchPanelSelectionChanged for value '75'");

                    let (tail, res) = GameSTriggerResearchPanelSelectionChangedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerResearchPanelSelectionChanged(res)))
                }
                76 => {
                    tracing::debug!("Variant ETriggerCommandError for value '76'");

                    let (tail, res) = GameSTriggerCommandErrorEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCommandError(res)))
                }
                77 => {
                    tracing::debug!("Variant ETriggerMercenaryPanelExit for value '77'");

                    let (tail, res) = GameSTriggerMercenaryPanelExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMercenaryPanelExit(res)))
                }
                78 => {
                    tracing::debug!("Variant ETriggerMercenaryPanelPurchase for value '78'");

                    let (tail, res) = GameSTriggerMercenaryPanelPurchaseEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMercenaryPanelPurchase(res)))
                }
                79 => {
                    tracing::debug!(
                        "Variant ETriggerMercenaryPanelSelectionChanged for value '79'"
                    );

                    let (tail, res) = GameSTriggerMercenaryPanelSelectionChangedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMercenaryPanelSelectionChanged(res)))
                }
                80 => {
                    tracing::debug!("Variant ETriggerVictoryPanelExit for value '80'");

                    let (tail, res) = GameSTriggerVictoryPanelExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerVictoryPanelExit(res)))
                }
                81 => {
                    tracing::debug!("Variant ETriggerBattleReportPanelExit for value '81'");

                    let (tail, res) = GameSTriggerBattleReportPanelExitEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerBattleReportPanelExit(res)))
                }
                82 => {
                    tracing::debug!("Variant ETriggerBattleReportPanelPlayMission for value '82'");

                    let (tail, res) = GameSTriggerBattleReportPanelPlayMissionEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerBattleReportPanelPlayMission(res)))
                }
                83 => {
                    tracing::debug!("Variant ETriggerBattleReportPanelPlayScene for value '83'");

                    let (tail, res) = GameSTriggerBattleReportPanelPlaySceneEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerBattleReportPanelPlayScene(res)))
                }
                84 => {
                    tracing::debug!("Variant ETriggerBattleReportSelectionChanged for value '84'");

                    let (tail, res) =
                        GameSTriggerBattleReportPanelSelectionChangedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerBattleReportSelectionChanged(res)))
                }
                85 => {
                    tracing::debug!("Variant ETriggerVictoryPanelPlayMissionAgain for value '85'");

                    let (tail, res) = GameSTriggerVictoryPanelPlayMissionAgainEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerVictoryPanelPlayMissionAgain(res)))
                }
                86 => {
                    tracing::debug!("Variant ETriggerMovieStarted for value '86'");

                    let (tail, res) = GameSTriggerMovieStartedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMovieStarted(res)))
                }
                87 => {
                    tracing::debug!("Variant ETriggerMovieFinished for value '87'");

                    let (tail, res) = GameSTriggerMovieFinishedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMovieFinished(res)))
                }
                88 => {
                    tracing::debug!("Variant EDecrementGameTimeRemaining for value '88'");

                    let (tail, res) = GameSDecrementGameTimeRemainingEvent::parse(tail)?;
                    Ok((tail, Self::EDecrementGameTimeRemaining(res)))
                }
                89 => {
                    tracing::debug!("Variant ETriggerPortraitLoaded for value '89'");

                    let (tail, res) = GameSTriggerPortraitLoadedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerPortraitLoaded(res)))
                }
                90 => {
                    tracing::debug!("Variant ETriggerQueryDialogDismissed for value '90'");

                    let (tail, res) = GameSTriggerCustomDialogDismissedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerQueryDialogDismissed(res)))
                }
                91 => {
                    tracing::debug!("Variant ETriggerGameMenuItemSelected for value '91'");

                    let (tail, res) = GameSTriggerGameMenuItemSelectedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerGameMenuItemSelected(res)))
                }
                92 => {
                    tracing::debug!("Variant ETriggerMouseWheel for value '92'");

                    let (tail, res) = GameSTriggerMouseWheelEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerMouseWheel(res)))
                }
                93 => {
                    tracing::debug!(
                        "Variant ETriggerPurchasePanelSelectedPurchaseItemChanged for value '93'"
                    );

                    let (tail, res) =
                        GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent::parse(tail)?;
                    Ok((
                        tail,
                        Self::ETriggerPurchasePanelSelectedPurchaseItemChanged(res),
                    ))
                }
                94 => {
                    tracing::debug!("Variant ETriggerPurchasePanelSelectedPurchaseCategoryChanged for value '94'");

                    let (tail, res) =
                        GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent::parse(tail)?;
                    Ok((
                        tail,
                        Self::ETriggerPurchasePanelSelectedPurchaseCategoryChanged(res),
                    ))
                }
                95 => {
                    tracing::debug!("Variant ETriggerButtonPressed for value '95'");

                    let (tail, res) = GameSTriggerButtonPressedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerButtonPressed(res)))
                }
                96 => {
                    tracing::debug!("Variant ETriggerGameCreditsFinished for value '96'");

                    let (tail, res) = GameSTriggerGameCreditsFinishedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerGameCreditsFinished(res)))
                }
                97 => {
                    tracing::debug!("Variant ETriggerCutsceneBookmarkFired for value '97'");

                    let (tail, res) = GameSTriggerCutsceneBookmarkFiredEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCutsceneBookmarkFired(res)))
                }
                98 => {
                    tracing::debug!("Variant ETriggerCutsceneEndSceneFired for value '98'");

                    let (tail, res) = GameSTriggerCutsceneEndSceneFiredEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCutsceneEndSceneFired(res)))
                }
                99 => {
                    tracing::debug!("Variant ETriggerCutsceneConversationLine for value '99'");

                    let (tail, res) = GameSTriggerCutsceneConversationLineEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCutsceneConversationLine(res)))
                }
                100 => {
                    tracing::debug!(
                        "Variant ETriggerCutsceneConversationLineMissing for value '100'"
                    );

                    let (tail, res) =
                        GameSTriggerCutsceneConversationLineMissingEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerCutsceneConversationLineMissing(res)))
                }
                101 => {
                    tracing::debug!("Variant EGameUserLeave for value '101'");

                    let (tail, res) = GameSGameUserLeaveEvent::parse(tail)?;
                    Ok((tail, Self::EGameUserLeave(res)))
                }
                102 => {
                    tracing::debug!("Variant EGameUserJoin for value '102'");

                    let (tail, res) = GameSGameUserJoinEvent::parse(tail)?;
                    Ok((tail, Self::EGameUserJoin(res)))
                }
                103 => {
                    tracing::debug!("Variant ECommandManagerState for value '103'");

                    let (tail, res) = GameSCommandManagerStateEvent::parse(tail)?;
                    Ok((tail, Self::ECommandManagerState(res)))
                }
                104 => {
                    tracing::debug!("Variant ECmdUpdateTargetPoint for value '104'");

                    let (tail, res) = GameSCmdUpdateTargetPointEvent::parse(tail)?;
                    Ok((tail, Self::ECmdUpdateTargetPoint(res)))
                }
                105 => {
                    tracing::debug!("Variant ECmdUpdateTargetUnit for value '105'");

                    let (tail, res) = GameSCmdUpdateTargetUnitEvent::parse(tail)?;
                    Ok((tail, Self::ECmdUpdateTargetUnit(res)))
                }
                106 => {
                    tracing::debug!("Variant ETriggerAnimLengthQueryByName for value '106'");

                    let (tail, res) = GameSTriggerAnimLengthQueryByNameEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerAnimLengthQueryByName(res)))
                }
                107 => {
                    tracing::debug!("Variant ETriggerAnimLengthQueryByProps for value '107'");

                    let (tail, res) = GameSTriggerAnimLengthQueryByPropsEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerAnimLengthQueryByProps(res)))
                }
                108 => {
                    tracing::debug!("Variant ETriggerAnimOffset for value '108'");

                    let (tail, res) = GameSTriggerAnimOffsetEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerAnimOffset(res)))
                }
                109 => {
                    tracing::debug!("Variant ECatalogModify for value '109'");

                    let (tail, res) = GameSCatalogModifyEvent::parse(tail)?;
                    Ok((tail, Self::ECatalogModify(res)))
                }
                110 => {
                    tracing::debug!("Variant EHeroTalentTreeSelected for value '110'");

                    let (tail, res) = GameSHeroTalentTreeSelectedEvent::parse(tail)?;
                    Ok((tail, Self::EHeroTalentTreeSelected(res)))
                }
                111 => {
                    tracing::debug!("Variant ETriggerProfilerLoggingFinished for value '111'");

                    let (tail, res) = GameSTriggerProfilerLoggingFinishedEvent::parse(tail)?;
                    Ok((tail, Self::ETriggerProfilerLoggingFinished(res)))
                }
                112 => {
                    tracing::debug!("Variant EHeroTalentTreeSelectionPanelToggled for value '112'");

                    let (tail, res) = GameSHeroTalentTreeSelectionPanelToggledEvent::parse(tail)?;
                    Ok((tail, Self::EHeroTalentTreeSelectionPanelToggled(res)))
                }
                113 => {
                    tracing::debug!("Variant EMuteUserChanged for value '113'");

                    let (tail, res) = GameSMuteChatEvent::parse(tail)?;
                    Ok((tail, Self::EMuteUserChanged(res)))
                }
                115 => {
                    tracing::debug!("Variant EConvertToReplaySession for value '115'");

                    let (tail, res) = GameSConvertToReplaySessionEvent::parse(tail)?;
                    Ok((tail, Self::EConvertToReplaySession(res)))
                }
                116 => {
                    tracing::debug!("Variant ESetSyncLoadingTime for value '116'");

                    let (tail, res) = GameSSetSyncLoadingTimeEvent::parse(tail)?;
                    Ok((tail, Self::ESetSyncLoadingTime(res)))
                }
                117 => {
                    tracing::debug!("Variant ESetSyncPlayingTime for value '117'");

                    let (tail, res) = GameSSetSyncPlayingTimeEvent::parse(tail)?;
                    Ok((tail, Self::ESetSyncPlayingTime(res)))
                }
                118 => {
                    tracing::debug!("Variant EPeerSetSyncLoadingTime for value '118'");

                    let (tail, res) = GameSPeerSetSyncLoadingTimeEvent::parse(tail)?;
                    Ok((tail, Self::EPeerSetSyncLoadingTime(res)))
                }
                119 => {
                    tracing::debug!("Variant EPeerSetSyncPlayingTime for value '119'");

                    let (tail, res) = GameSPeerSetSyncPlayingTimeEvent::parse(tail)?;
                    Ok((tail, Self::EPeerSetSyncPlayingTime(res)))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCmdAbil {
        pub m_abil_link: GameTAbilLink,
        pub m_abil_cmd_index: i64,
        pub m_abil_cmd_data: Option<Uint8>,
    }
    impl GameSCmdAbil {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_abil_link(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTAbilLink> {
            let (tail, m_abil_link) = GameTAbilLink::parse(input)?;
            tracing::debug!("m_abil_link: {:?}", m_abil_link);
            Ok((tail, m_abil_link))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_abil_cmd_index(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_abil_cmd_index) = parse_packed_int(input, 0, 5usize)?;
            tracing::debug!("m_abil_cmd_index: {:?}", m_abil_cmd_index);
            Ok((tail, m_abil_cmd_index))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_abil_cmd_data(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Uint8>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_abil_cmd_data) = if is_provided {
                let (tail, res) = Uint8::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_abil_cmd_data: {:?}", m_abil_cmd_data);
            Ok((tail, m_abil_cmd_data))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCmdAbil::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_abil_link: Option<GameTAbilLink> = None;
            let mut m_abil_cmd_index: Option<i64> = None;
            let mut m_abil_cmd_data: Option<Option<Uint8>> = Some(None);
            if m_abil_link.is_none() {
                let (new_tail, parsed_m_abil_link) = Self::parse_m_abil_link(tail)?;
                tail = new_tail;
                m_abil_link = Some(parsed_m_abil_link);
            }
            if m_abil_cmd_index.is_none() {
                let (new_tail, parsed_m_abil_cmd_index) = Self::parse_m_abil_cmd_index(tail)?;
                tail = new_tail;
                m_abil_cmd_index = Some(parsed_m_abil_cmd_index);
            }
            if let Some(None) = m_abil_cmd_data {
                let (new_tail, parsed_m_abil_cmd_data) = Self::parse_m_abil_cmd_data(tail)?;
                tail = new_tail;
                m_abil_cmd_data = Some(parsed_m_abil_cmd_data);
            }
            Ok((
                tail,
                Self {
                    m_abil_link: ok_or_return_missing_field_err!(m_abil_link),
                    m_abil_cmd_index: ok_or_return_missing_field_err!(m_abil_cmd_index),
                    m_abil_cmd_data: ok_or_return_missing_field_err!(m_abil_cmd_data),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCmdDataTargetUnit {
        pub m_target_unit_flags: Uint16,
        pub m_timer: Uint8,
        pub m_tag: GameTUnitTag,
        pub m_snapshot_unit_link: GameTUnitLink,
        pub m_snapshot_control_player_id: Option<GameTPlayerId>,
        pub m_snapshot_upkeep_player_id: Option<GameTPlayerId>,
        pub m_snapshot_point: GameSMapCoord3D,
    }
    impl GameSCmdDataTargetUnit {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target_unit_flags(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint16> {
            let (tail, m_target_unit_flags) = Uint16::parse(input)?;
            tracing::debug!("m_target_unit_flags: {:?}", m_target_unit_flags);
            Ok((tail, m_target_unit_flags))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_timer(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_timer) = Uint8::parse(input)?;
            tracing::debug!("m_timer: {:?}", m_timer);
            Ok((tail, m_timer))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_tag(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTUnitTag> {
            let (tail, m_tag) = GameTUnitTag::parse(input)?;
            tracing::debug!("m_tag: {:?}", m_tag);
            Ok((tail, m_tag))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_snapshot_unit_link(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTUnitLink> {
            let (tail, m_snapshot_unit_link) = GameTUnitLink::parse(input)?;
            tracing::debug!("m_snapshot_unit_link: {:?}", m_snapshot_unit_link);
            Ok((tail, m_snapshot_unit_link))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_snapshot_control_player_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTPlayerId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_snapshot_control_player_id) = if is_provided {
                let (tail, res) = GameTPlayerId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!(
                "m_snapshot_control_player_id: {:?}",
                m_snapshot_control_player_id
            );
            Ok((tail, m_snapshot_control_player_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_snapshot_upkeep_player_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTPlayerId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_snapshot_upkeep_player_id) = if is_provided {
                let (tail, res) = GameTPlayerId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!(
                "m_snapshot_upkeep_player_id: {:?}",
                m_snapshot_upkeep_player_id
            );
            Ok((tail, m_snapshot_upkeep_player_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_snapshot_point(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSMapCoord3D> {
            let (tail, m_snapshot_point) = GameSMapCoord3D::parse(input)?;
            tracing::debug!("m_snapshot_point: {:?}", m_snapshot_point);
            Ok((tail, m_snapshot_point))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCmdDataTargetUnit::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_target_unit_flags: Option<Uint16> = None;
            let mut m_timer: Option<Uint8> = None;
            let mut m_tag: Option<GameTUnitTag> = None;
            let mut m_snapshot_unit_link: Option<GameTUnitLink> = None;
            let mut m_snapshot_control_player_id: Option<Option<GameTPlayerId>> = Some(None);
            let mut m_snapshot_upkeep_player_id: Option<Option<GameTPlayerId>> = Some(None);
            let mut m_snapshot_point: Option<GameSMapCoord3D> = None;
            if m_target_unit_flags.is_none() {
                let (new_tail, parsed_m_target_unit_flags) = Self::parse_m_target_unit_flags(tail)?;
                tail = new_tail;
                m_target_unit_flags = Some(parsed_m_target_unit_flags);
            }
            if m_timer.is_none() {
                let (new_tail, parsed_m_timer) = Self::parse_m_timer(tail)?;
                tail = new_tail;
                m_timer = Some(parsed_m_timer);
            }
            if m_tag.is_none() {
                let (new_tail, parsed_m_tag) = Self::parse_m_tag(tail)?;
                tail = new_tail;
                m_tag = Some(parsed_m_tag);
            }
            if m_snapshot_unit_link.is_none() {
                let (new_tail, parsed_m_snapshot_unit_link) =
                    Self::parse_m_snapshot_unit_link(tail)?;
                tail = new_tail;
                m_snapshot_unit_link = Some(parsed_m_snapshot_unit_link);
            }
            if let Some(None) = m_snapshot_control_player_id {
                let (new_tail, parsed_m_snapshot_control_player_id) =
                    Self::parse_m_snapshot_control_player_id(tail)?;
                tail = new_tail;
                m_snapshot_control_player_id = Some(parsed_m_snapshot_control_player_id);
            }
            if let Some(None) = m_snapshot_upkeep_player_id {
                let (new_tail, parsed_m_snapshot_upkeep_player_id) =
                    Self::parse_m_snapshot_upkeep_player_id(tail)?;
                tail = new_tail;
                m_snapshot_upkeep_player_id = Some(parsed_m_snapshot_upkeep_player_id);
            }
            if m_snapshot_point.is_none() {
                let (new_tail, parsed_m_snapshot_point) = Self::parse_m_snapshot_point(tail)?;
                tail = new_tail;
                m_snapshot_point = Some(parsed_m_snapshot_point);
            }
            Ok((
                tail,
                Self {
                    m_target_unit_flags: ok_or_return_missing_field_err!(m_target_unit_flags),
                    m_timer: ok_or_return_missing_field_err!(m_timer),
                    m_tag: ok_or_return_missing_field_err!(m_tag),
                    m_snapshot_unit_link: ok_or_return_missing_field_err!(m_snapshot_unit_link),
                    m_snapshot_control_player_id: ok_or_return_missing_field_err!(
                        m_snapshot_control_player_id
                    ),
                    m_snapshot_upkeep_player_id: ok_or_return_missing_field_err!(
                        m_snapshot_upkeep_player_id
                    ),
                    m_snapshot_point: ok_or_return_missing_field_err!(m_snapshot_point),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameSCmdData {
        None(()),
        TargetPoint(GameSMapCoord3D),
        TargetUnit(GameSCmdDataTargetUnit),
        Data(Uint32),
    }
    impl GameSCmdData {
        #[tracing::instrument(name="87702::GameSCmdData::ChoiceType::parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // ChoiceType:
            // Use the number of elements in the json .fields to calculate how many
            // bits to have unique tags.
            // let num_fields: usize = 4;
            let offset = 0i64;
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, offset, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant tagged '0' for None");
                    let (tail, res) = take_null(tail)?;
                    Ok((tail, Self::None(res)))
                }
                1 => {
                    tracing::debug!("Variant tagged '1' for TargetPoint");
                    let (tail, res) = GameSMapCoord3D::parse(tail)?;
                    Ok((tail, Self::TargetPoint(res)))
                }
                2 => {
                    tracing::debug!("Variant tagged '2' for TargetUnit");
                    let (tail, res) = GameSCmdDataTargetUnit::parse(tail)?;
                    Ok((tail, Self::TargetUnit(res)))
                }
                3 => {
                    tracing::debug!("Variant tagged '3' for Data");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::Data(res)))
                }

                _ => {
                    tracing::error!("Unknown variant for tag {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSetLobbySlotEvent {
        pub m_slot_id: GameTLobbySlotId,
        pub m_slot_change: GameSLobbySlotChange,
    }
    impl GameSSetLobbySlotEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_slot_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTLobbySlotId> {
            let (tail, m_slot_id) = GameTLobbySlotId::parse(input)?;
            tracing::debug!("m_slot_id: {:?}", m_slot_id);
            Ok((tail, m_slot_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_slot_change(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSLobbySlotChange> {
            let (tail, m_slot_change) = GameSLobbySlotChange::parse(input)?;
            tracing::debug!("m_slot_change: {:?}", m_slot_change);
            Ok((tail, m_slot_change))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSetLobbySlotEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_slot_id: Option<GameTLobbySlotId> = None;
            let mut m_slot_change: Option<GameSLobbySlotChange> = None;
            if m_slot_id.is_none() {
                let (new_tail, parsed_m_slot_id) = Self::parse_m_slot_id(tail)?;
                tail = new_tail;
                m_slot_id = Some(parsed_m_slot_id);
            }
            if m_slot_change.is_none() {
                let (new_tail, parsed_m_slot_change) = Self::parse_m_slot_change(tail)?;
                tail = new_tail;
                m_slot_change = Some(parsed_m_slot_change);
            }
            Ok((
                tail,
                Self {
                    m_slot_id: ok_or_return_missing_field_err!(m_slot_id),
                    m_slot_change: ok_or_return_missing_field_err!(m_slot_change),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSDropUserEvent {
        pub m_drop_session_user_id: TUserId,
        pub m_reason: ELeaveReason,
    }
    impl GameSDropUserEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_drop_session_user_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TUserId> {
            let (tail, m_drop_session_user_id) = TUserId::parse(input)?;
            tracing::debug!("m_drop_session_user_id: {:?}", m_drop_session_user_id);
            Ok((tail, m_drop_session_user_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_reason(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), ELeaveReason> {
            let (tail, m_reason) = ELeaveReason::parse(input)?;
            tracing::debug!("m_reason: {:?}", m_reason);
            Ok((tail, m_reason))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSDropUserEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_drop_session_user_id: Option<TUserId> = None;
            let mut m_reason: Option<ELeaveReason> = None;
            if m_drop_session_user_id.is_none() {
                let (new_tail, parsed_m_drop_session_user_id) =
                    Self::parse_m_drop_session_user_id(tail)?;
                tail = new_tail;
                m_drop_session_user_id = Some(parsed_m_drop_session_user_id);
            }
            if m_reason.is_none() {
                let (new_tail, parsed_m_reason) = Self::parse_m_reason(tail)?;
                tail = new_tail;
                m_reason = Some(parsed_m_reason);
            }
            Ok((
                tail,
                Self {
                    m_drop_session_user_id: ok_or_return_missing_field_err!(m_drop_session_user_id),
                    m_reason: ok_or_return_missing_field_err!(m_reason),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSStartGameEvent {}
    impl GameSStartGameEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSStartGameEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSDropOurselvesEvent {}
    impl GameSDropOurselvesEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSDropOurselvesEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSBankFileEvent {
        pub m_name: Vec<u8>,
    }
    impl GameSBankFileEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSBankFileEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_name: Option<Vec<u8>> = None;
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            Ok((
                tail,
                Self {
                    m_name: ok_or_return_missing_field_err!(m_name),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSBankSectionEvent {
        pub m_name: Vec<u8>,
    }
    impl GameSBankSectionEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 6)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSBankSectionEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_name: Option<Vec<u8>> = None;
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            Ok((
                tail,
                Self {
                    m_name: ok_or_return_missing_field_err!(m_name),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSBankKeyEvent {
        pub m_name: Vec<u8>,
        pub m_type: Uint32,
        pub m_data: Vec<u8>,
    }
    impl GameSBankKeyEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 6)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_type(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_type) = Uint32::parse(input)?;
            tracing::debug!("m_type: {:?}", m_type);
            Ok((tail, m_type))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_data(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 5)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSBankKeyEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_name: Option<Vec<u8>> = None;
            let mut m_type: Option<Uint32> = None;
            let mut m_data: Option<Vec<u8>> = None;
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            if m_type.is_none() {
                let (new_tail, parsed_m_type) = Self::parse_m_type(tail)?;
                tail = new_tail;
                m_type = Some(parsed_m_type);
            }
            if m_data.is_none() {
                let (new_tail, parsed_m_data) = Self::parse_m_data(tail)?;
                tail = new_tail;
                m_data = Some(parsed_m_data);
            }
            Ok((
                tail,
                Self {
                    m_name: ok_or_return_missing_field_err!(m_name),
                    m_type: ok_or_return_missing_field_err!(m_type),
                    m_data: ok_or_return_missing_field_err!(m_data),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSBankValueEvent {
        pub m_type: Uint32,
        pub m_name: Vec<u8>,
        pub m_data: Vec<u8>,
    }
    impl GameSBankValueEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_type(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_type) = Uint32::parse(input)?;
            tracing::debug!("m_type: {:?}", m_type);
            Ok((tail, m_type))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 6)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_data(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 10)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSBankValueEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_type: Option<Uint32> = None;
            let mut m_name: Option<Vec<u8>> = None;
            let mut m_data: Option<Vec<u8>> = None;
            if m_type.is_none() {
                let (new_tail, parsed_m_type) = Self::parse_m_type(tail)?;
                tail = new_tail;
                m_type = Some(parsed_m_type);
            }
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            if m_data.is_none() {
                let (new_tail, parsed_m_data) = Self::parse_m_data(tail)?;
                tail = new_tail;
                m_data = Some(parsed_m_data);
            }
            Ok((
                tail,
                Self {
                    m_type: ok_or_return_missing_field_err!(m_type),
                    m_name: ok_or_return_missing_field_err!(m_name),
                    m_data: ok_or_return_missing_field_err!(m_data),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSBankSignatureEvent {
        pub m_signature: Vec<Uint8>,
        pub m_toon_handle: CToonHandle,
    }
    impl GameSBankSignatureEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_signature(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<Uint8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 5)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = Uint8::parse(tail)?;
                tail = new_tail;
                let data = data;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_toon_handle(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CToonHandle> {
            let (tail, m_toon_handle) = CToonHandle::parse(input)?;
            tracing::debug!("m_toon_handle: {:?}", m_toon_handle);
            Ok((tail, m_toon_handle))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSBankSignatureEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_signature: Option<Vec<Uint8>> = None;
            let mut m_toon_handle: Option<CToonHandle> = None;
            if m_signature.is_none() {
                let (new_tail, parsed_m_signature) = Self::parse_m_signature(tail)?;
                tail = new_tail;
                m_signature = Some(parsed_m_signature);
            }
            if m_toon_handle.is_none() {
                let (new_tail, parsed_m_toon_handle) = Self::parse_m_toon_handle(tail)?;
                tail = new_tail;
                m_toon_handle = Some(parsed_m_toon_handle);
            }
            Ok((
                tail,
                Self {
                    m_signature: ok_or_return_missing_field_err!(m_signature),
                    m_toon_handle: ok_or_return_missing_field_err!(m_toon_handle),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSUserOptionsEvent {
        pub m_game_fully_downloaded: bool,
        pub m_development_cheats_enabled: bool,
        pub m_test_cheats_enabled: bool,
        pub m_multiplayer_cheats_enabled: bool,
        pub m_sync_checksumming_enabled: bool,
        pub m_is_map_to_map_transition: bool,
        pub m_debug_pause_enabled: bool,
        pub m_use_galaxy_asserts: bool,
        pub m_platform_mac: bool,
        pub m_camera_follow: bool,
        pub m_base_build_num: Uint32,
        pub m_build_num: Uint32,
        pub m_version_flags: Uint32,
        pub m_hotkey_profile: Vec<u8>,
    }
    impl GameSUserOptionsEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_fully_downloaded(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_game_fully_downloaded) = parse_bool(input)?;
            tracing::debug!("m_game_fully_downloaded: {:?}", m_game_fully_downloaded);
            Ok((tail, m_game_fully_downloaded))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_development_cheats_enabled(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_development_cheats_enabled) = parse_bool(input)?;
            tracing::debug!(
                "m_development_cheats_enabled: {:?}",
                m_development_cheats_enabled
            );
            Ok((tail, m_development_cheats_enabled))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_test_cheats_enabled(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_test_cheats_enabled) = parse_bool(input)?;
            tracing::debug!("m_test_cheats_enabled: {:?}", m_test_cheats_enabled);
            Ok((tail, m_test_cheats_enabled))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_multiplayer_cheats_enabled(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_multiplayer_cheats_enabled) = parse_bool(input)?;
            tracing::debug!(
                "m_multiplayer_cheats_enabled: {:?}",
                m_multiplayer_cheats_enabled
            );
            Ok((tail, m_multiplayer_cheats_enabled))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sync_checksumming_enabled(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_sync_checksumming_enabled) = parse_bool(input)?;
            tracing::debug!(
                "m_sync_checksumming_enabled: {:?}",
                m_sync_checksumming_enabled
            );
            Ok((tail, m_sync_checksumming_enabled))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_is_map_to_map_transition(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_is_map_to_map_transition) = parse_bool(input)?;
            tracing::debug!(
                "m_is_map_to_map_transition: {:?}",
                m_is_map_to_map_transition
            );
            Ok((tail, m_is_map_to_map_transition))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_debug_pause_enabled(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_debug_pause_enabled) = parse_bool(input)?;
            tracing::debug!("m_debug_pause_enabled: {:?}", m_debug_pause_enabled);
            Ok((tail, m_debug_pause_enabled))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_use_galaxy_asserts(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_use_galaxy_asserts) = parse_bool(input)?;
            tracing::debug!("m_use_galaxy_asserts: {:?}", m_use_galaxy_asserts);
            Ok((tail, m_use_galaxy_asserts))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_platform_mac(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_platform_mac) = parse_bool(input)?;
            tracing::debug!("m_platform_mac: {:?}", m_platform_mac);
            Ok((tail, m_platform_mac))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_camera_follow(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_camera_follow) = parse_bool(input)?;
            tracing::debug!("m_camera_follow: {:?}", m_camera_follow);
            Ok((tail, m_camera_follow))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_base_build_num(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_base_build_num) = Uint32::parse(input)?;
            tracing::debug!("m_base_build_num: {:?}", m_base_build_num);
            Ok((tail, m_base_build_num))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_build_num(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_build_num) = Uint32::parse(input)?;
            tracing::debug!("m_build_num: {:?}", m_build_num);
            Ok((tail, m_build_num))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_version_flags(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_version_flags) = Uint32::parse(input)?;
            tracing::debug!("m_version_flags: {:?}", m_version_flags);
            Ok((tail, m_version_flags))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_hotkey_profile(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSUserOptionsEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_game_fully_downloaded: Option<bool> = None;
            let mut m_development_cheats_enabled: Option<bool> = None;
            let mut m_test_cheats_enabled: Option<bool> = None;
            let mut m_multiplayer_cheats_enabled: Option<bool> = None;
            let mut m_sync_checksumming_enabled: Option<bool> = None;
            let mut m_is_map_to_map_transition: Option<bool> = None;
            let mut m_debug_pause_enabled: Option<bool> = None;
            let mut m_use_galaxy_asserts: Option<bool> = None;
            let mut m_platform_mac: Option<bool> = None;
            let mut m_camera_follow: Option<bool> = None;
            let mut m_base_build_num: Option<Uint32> = None;
            let mut m_build_num: Option<Uint32> = None;
            let mut m_version_flags: Option<Uint32> = None;
            let mut m_hotkey_profile: Option<Vec<u8>> = None;
            if m_game_fully_downloaded.is_none() {
                let (new_tail, parsed_m_game_fully_downloaded) =
                    Self::parse_m_game_fully_downloaded(tail)?;
                tail = new_tail;
                m_game_fully_downloaded = Some(parsed_m_game_fully_downloaded);
            }
            if m_development_cheats_enabled.is_none() {
                let (new_tail, parsed_m_development_cheats_enabled) =
                    Self::parse_m_development_cheats_enabled(tail)?;
                tail = new_tail;
                m_development_cheats_enabled = Some(parsed_m_development_cheats_enabled);
            }
            if m_test_cheats_enabled.is_none() {
                let (new_tail, parsed_m_test_cheats_enabled) =
                    Self::parse_m_test_cheats_enabled(tail)?;
                tail = new_tail;
                m_test_cheats_enabled = Some(parsed_m_test_cheats_enabled);
            }
            if m_multiplayer_cheats_enabled.is_none() {
                let (new_tail, parsed_m_multiplayer_cheats_enabled) =
                    Self::parse_m_multiplayer_cheats_enabled(tail)?;
                tail = new_tail;
                m_multiplayer_cheats_enabled = Some(parsed_m_multiplayer_cheats_enabled);
            }
            if m_sync_checksumming_enabled.is_none() {
                let (new_tail, parsed_m_sync_checksumming_enabled) =
                    Self::parse_m_sync_checksumming_enabled(tail)?;
                tail = new_tail;
                m_sync_checksumming_enabled = Some(parsed_m_sync_checksumming_enabled);
            }
            if m_is_map_to_map_transition.is_none() {
                let (new_tail, parsed_m_is_map_to_map_transition) =
                    Self::parse_m_is_map_to_map_transition(tail)?;
                tail = new_tail;
                m_is_map_to_map_transition = Some(parsed_m_is_map_to_map_transition);
            }
            if m_debug_pause_enabled.is_none() {
                let (new_tail, parsed_m_debug_pause_enabled) =
                    Self::parse_m_debug_pause_enabled(tail)?;
                tail = new_tail;
                m_debug_pause_enabled = Some(parsed_m_debug_pause_enabled);
            }
            if m_use_galaxy_asserts.is_none() {
                let (new_tail, parsed_m_use_galaxy_asserts) =
                    Self::parse_m_use_galaxy_asserts(tail)?;
                tail = new_tail;
                m_use_galaxy_asserts = Some(parsed_m_use_galaxy_asserts);
            }
            if m_platform_mac.is_none() {
                let (new_tail, parsed_m_platform_mac) = Self::parse_m_platform_mac(tail)?;
                tail = new_tail;
                m_platform_mac = Some(parsed_m_platform_mac);
            }
            if m_camera_follow.is_none() {
                let (new_tail, parsed_m_camera_follow) = Self::parse_m_camera_follow(tail)?;
                tail = new_tail;
                m_camera_follow = Some(parsed_m_camera_follow);
            }
            if m_base_build_num.is_none() {
                let (new_tail, parsed_m_base_build_num) = Self::parse_m_base_build_num(tail)?;
                tail = new_tail;
                m_base_build_num = Some(parsed_m_base_build_num);
            }
            if m_build_num.is_none() {
                let (new_tail, parsed_m_build_num) = Self::parse_m_build_num(tail)?;
                tail = new_tail;
                m_build_num = Some(parsed_m_build_num);
            }
            if m_version_flags.is_none() {
                let (new_tail, parsed_m_version_flags) = Self::parse_m_version_flags(tail)?;
                tail = new_tail;
                m_version_flags = Some(parsed_m_version_flags);
            }
            if m_hotkey_profile.is_none() {
                let (new_tail, parsed_m_hotkey_profile) = Self::parse_m_hotkey_profile(tail)?;
                tail = new_tail;
                m_hotkey_profile = Some(parsed_m_hotkey_profile);
            }
            Ok((
                tail,
                Self {
                    m_game_fully_downloaded: ok_or_return_missing_field_err!(
                        m_game_fully_downloaded
                    ),
                    m_development_cheats_enabled: ok_or_return_missing_field_err!(
                        m_development_cheats_enabled
                    ),
                    m_test_cheats_enabled: ok_or_return_missing_field_err!(m_test_cheats_enabled),
                    m_multiplayer_cheats_enabled: ok_or_return_missing_field_err!(
                        m_multiplayer_cheats_enabled
                    ),
                    m_sync_checksumming_enabled: ok_or_return_missing_field_err!(
                        m_sync_checksumming_enabled
                    ),
                    m_is_map_to_map_transition: ok_or_return_missing_field_err!(
                        m_is_map_to_map_transition
                    ),
                    m_debug_pause_enabled: ok_or_return_missing_field_err!(m_debug_pause_enabled),
                    m_use_galaxy_asserts: ok_or_return_missing_field_err!(m_use_galaxy_asserts),
                    m_platform_mac: ok_or_return_missing_field_err!(m_platform_mac),
                    m_camera_follow: ok_or_return_missing_field_err!(m_camera_follow),
                    m_base_build_num: ok_or_return_missing_field_err!(m_base_build_num),
                    m_build_num: ok_or_return_missing_field_err!(m_build_num),
                    m_version_flags: ok_or_return_missing_field_err!(m_version_flags),
                    m_hotkey_profile: ok_or_return_missing_field_err!(m_hotkey_profile),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPickMapTagEvent {
        pub m_picked_map_tag: Uint8,
    }
    impl GameSPickMapTagEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_picked_map_tag(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_picked_map_tag) = Uint8::parse(input)?;
            tracing::debug!("m_picked_map_tag: {:?}", m_picked_map_tag);
            Ok((tail, m_picked_map_tag))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSPickMapTagEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_picked_map_tag: Option<Uint8> = None;
            if m_picked_map_tag.is_none() {
                let (new_tail, parsed_m_picked_map_tag) = Self::parse_m_picked_map_tag(tail)?;
                tail = new_tail;
                m_picked_map_tag = Some(parsed_m_picked_map_tag);
            }
            Ok((
                tail,
                Self {
                    m_picked_map_tag: ok_or_return_missing_field_err!(m_picked_map_tag),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSUserFinishedLoadingEvent {}
    impl GameSUserFinishedLoadingEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSUserFinishedLoadingEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSUserFinishedLoadingSyncEvent {}
    impl GameSUserFinishedLoadingSyncEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSUserFinishedLoadingSyncEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSetGameDurationEvent {
        pub m_game_duration: Uint32,
    }
    impl GameSSetGameDurationEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_duration(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_game_duration) = Uint32::parse(input)?;
            tracing::debug!("m_game_duration: {:?}", m_game_duration);
            Ok((tail, m_game_duration))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSetGameDurationEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_game_duration: Option<Uint32> = None;
            if m_game_duration.is_none() {
                let (new_tail, parsed_m_game_duration) = Self::parse_m_game_duration(tail)?;
                tail = new_tail;
                m_game_duration = Some(parsed_m_game_duration);
            }
            Ok((
                tail,
                Self {
                    m_game_duration: ok_or_return_missing_field_err!(m_game_duration),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTurnEvent {}
    impl GameSTurnEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTurnEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCameraSaveEvent {
        pub m_which: i64,
        pub m_target: GameSPointMini,
    }
    impl GameSCameraSaveEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_which(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_which) = parse_packed_int(input, 0, 3usize)?;
            tracing::debug!("m_which: {:?}", m_which);
            Ok((tail, m_which))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSPointMini> {
            let (tail, m_target) = GameSPointMini::parse(input)?;
            tracing::debug!("m_target: {:?}", m_target);
            Ok((tail, m_target))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCameraSaveEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_which: Option<i64> = None;
            let mut m_target: Option<GameSPointMini> = None;
            if m_which.is_none() {
                let (new_tail, parsed_m_which) = Self::parse_m_which(tail)?;
                tail = new_tail;
                m_which = Some(parsed_m_which);
            }
            if m_target.is_none() {
                let (new_tail, parsed_m_target) = Self::parse_m_target(tail)?;
                tail = new_tail;
                m_target = Some(parsed_m_target);
            }
            Ok((
                tail,
                Self {
                    m_which: ok_or_return_missing_field_err!(m_which),
                    m_target: ok_or_return_missing_field_err!(m_target),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPauseGameEvent {
        pub m_pause_type_index: Uint8,
    }
    impl GameSPauseGameEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_pause_type_index(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_pause_type_index) = Uint8::parse(input)?;
            tracing::debug!("m_pause_type_index: {:?}", m_pause_type_index);
            Ok((tail, m_pause_type_index))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSPauseGameEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_pause_type_index: Option<Uint8> = None;
            if m_pause_type_index.is_none() {
                let (new_tail, parsed_m_pause_type_index) = Self::parse_m_pause_type_index(tail)?;
                tail = new_tail;
                m_pause_type_index = Some(parsed_m_pause_type_index);
            }
            Ok((
                tail,
                Self {
                    m_pause_type_index: ok_or_return_missing_field_err!(m_pause_type_index),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSUnpauseGameEvent {
        pub m_pause_type_index: Uint8,
    }
    impl GameSUnpauseGameEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_pause_type_index(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_pause_type_index) = Uint8::parse(input)?;
            tracing::debug!("m_pause_type_index: {:?}", m_pause_type_index);
            Ok((tail, m_pause_type_index))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSUnpauseGameEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_pause_type_index: Option<Uint8> = None;
            if m_pause_type_index.is_none() {
                let (new_tail, parsed_m_pause_type_index) = Self::parse_m_pause_type_index(tail)?;
                tail = new_tail;
                m_pause_type_index = Some(parsed_m_pause_type_index);
            }
            Ok((
                tail,
                Self {
                    m_pause_type_index: ok_or_return_missing_field_err!(m_pause_type_index),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSingleStepGameEvent {}
    impl GameSSingleStepGameEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSSingleStepGameEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSetGameSpeedEvent {
        pub m_speed: GameEGameSpeed,
    }
    impl GameSSetGameSpeedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_speed(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEGameSpeed> {
            let (tail, m_speed) = GameEGameSpeed::parse(input)?;
            tracing::debug!("m_speed: {:?}", m_speed);
            Ok((tail, m_speed))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSetGameSpeedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_speed: Option<GameEGameSpeed> = None;
            if m_speed.is_none() {
                let (new_tail, parsed_m_speed) = Self::parse_m_speed(tail)?;
                tail = new_tail;
                m_speed = Some(parsed_m_speed);
            }
            Ok((
                tail,
                Self {
                    m_speed: ok_or_return_missing_field_err!(m_speed),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSAddGameSpeedEvent {
        pub m_delta: Int8,
    }
    impl GameSAddGameSpeedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_delta(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_delta) = Int8::parse(input)?;
            tracing::debug!("m_delta: {:?}", m_delta);
            Ok((tail, m_delta))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSAddGameSpeedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_delta: Option<Int8> = None;
            if m_delta.is_none() {
                let (new_tail, parsed_m_delta) = Self::parse_m_delta(tail)?;
                tail = new_tail;
                m_delta = Some(parsed_m_delta);
            }
            Ok((
                tail,
                Self {
                    m_delta: ok_or_return_missing_field_err!(m_delta),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSReplayJumpEvent {
        pub m_replay_jump_game_loop: Option<Uint32>,
    }
    impl GameSReplayJumpEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_replay_jump_game_loop(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Uint32>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_replay_jump_game_loop) = if is_provided {
                let (tail, res) = Uint32::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_replay_jump_game_loop: {:?}", m_replay_jump_game_loop);
            Ok((tail, m_replay_jump_game_loop))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSReplayJumpEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_replay_jump_game_loop: Option<Option<Uint32>> = Some(None);
            if let Some(None) = m_replay_jump_game_loop {
                let (new_tail, parsed_m_replay_jump_game_loop) =
                    Self::parse_m_replay_jump_game_loop(tail)?;
                tail = new_tail;
                m_replay_jump_game_loop = Some(parsed_m_replay_jump_game_loop);
            }
            Ok((
                tail,
                Self {
                    m_replay_jump_game_loop: ok_or_return_missing_field_err!(
                        m_replay_jump_game_loop
                    ),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSaveGameEvent {
        pub m_file_name: CFilePath,
        pub m_automatic: bool,
        pub m_overwrite: bool,
        pub m_name: Vec<u8>,
        pub m_description: Vec<u8>,
    }
    impl GameSSaveGameEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_file_name(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CFilePath> {
            let (tail, m_file_name) = CFilePath::parse(input)?;
            tracing::debug!("m_file_name: {:?}", str::from_utf8(&m_file_name.value));
            Ok((tail, m_file_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_automatic(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_automatic) = parse_bool(input)?;
            tracing::debug!("m_automatic: {:?}", m_automatic);
            Ok((tail, m_automatic))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_overwrite(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_overwrite) = parse_bool(input)?;
            tracing::debug!("m_overwrite: {:?}", m_overwrite);
            Ok((tail, m_overwrite))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 6)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_description(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 9)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSaveGameEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_file_name: Option<CFilePath> = None;
            let mut m_automatic: Option<bool> = None;
            let mut m_overwrite: Option<bool> = None;
            let mut m_name: Option<Vec<u8>> = None;
            let mut m_description: Option<Vec<u8>> = None;
            if m_file_name.is_none() {
                let (new_tail, parsed_m_file_name) = Self::parse_m_file_name(tail)?;
                tail = new_tail;
                m_file_name = Some(parsed_m_file_name);
            }
            if m_automatic.is_none() {
                let (new_tail, parsed_m_automatic) = Self::parse_m_automatic(tail)?;
                tail = new_tail;
                m_automatic = Some(parsed_m_automatic);
            }
            if m_overwrite.is_none() {
                let (new_tail, parsed_m_overwrite) = Self::parse_m_overwrite(tail)?;
                tail = new_tail;
                m_overwrite = Some(parsed_m_overwrite);
            }
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            if m_description.is_none() {
                let (new_tail, parsed_m_description) = Self::parse_m_description(tail)?;
                tail = new_tail;
                m_description = Some(parsed_m_description);
            }
            Ok((
                tail,
                Self {
                    m_file_name: ok_or_return_missing_field_err!(m_file_name),
                    m_automatic: ok_or_return_missing_field_err!(m_automatic),
                    m_overwrite: ok_or_return_missing_field_err!(m_overwrite),
                    m_name: ok_or_return_missing_field_err!(m_name),
                    m_description: ok_or_return_missing_field_err!(m_description),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSaveGameDoneEvent {}
    impl GameSSaveGameDoneEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSSaveGameDoneEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSLoadGameDoneEvent {}
    impl GameSLoadGameDoneEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSLoadGameDoneEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCheatEventData {
        pub m_point: GameSPoint,
        pub m_time: Int32,
        pub m_verb: GameCCheatString,
        pub m_arguments: GameCCheatString,
    }
    impl GameSCheatEventData {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_point(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameSPoint> {
            let (tail, m_point) = GameSPoint::parse(input)?;
            tracing::debug!("m_point: {:?}", m_point);
            Ok((tail, m_point))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_time(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_time) = Int32::parse(input)?;
            tracing::debug!("m_time: {:?}", m_time);
            Ok((tail, m_time))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_verb(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCCheatString> {
            let (tail, m_verb) = GameCCheatString::parse(input)?;
            tracing::debug!("m_verb: {:?}", m_verb);
            Ok((tail, m_verb))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_arguments(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCCheatString> {
            let (tail, m_arguments) = GameCCheatString::parse(input)?;
            tracing::debug!("m_arguments: {:?}", m_arguments);
            Ok((tail, m_arguments))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCheatEventData::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_point: Option<GameSPoint> = None;
            let mut m_time: Option<Int32> = None;
            let mut m_verb: Option<GameCCheatString> = None;
            let mut m_arguments: Option<GameCCheatString> = None;
            if m_point.is_none() {
                let (new_tail, parsed_m_point) = Self::parse_m_point(tail)?;
                tail = new_tail;
                m_point = Some(parsed_m_point);
            }
            if m_time.is_none() {
                let (new_tail, parsed_m_time) = Self::parse_m_time(tail)?;
                tail = new_tail;
                m_time = Some(parsed_m_time);
            }
            if m_verb.is_none() {
                let (new_tail, parsed_m_verb) = Self::parse_m_verb(tail)?;
                tail = new_tail;
                m_verb = Some(parsed_m_verb);
            }
            if m_arguments.is_none() {
                let (new_tail, parsed_m_arguments) = Self::parse_m_arguments(tail)?;
                tail = new_tail;
                m_arguments = Some(parsed_m_arguments);
            }
            Ok((
                tail,
                Self {
                    m_point: ok_or_return_missing_field_err!(m_point),
                    m_time: ok_or_return_missing_field_err!(m_time),
                    m_verb: ok_or_return_missing_field_err!(m_verb),
                    m_arguments: ok_or_return_missing_field_err!(m_arguments),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSessionCheatEvent {
        pub m_data: GameSCheatEventData,
    }
    impl GameSSessionCheatEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_data(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSCheatEventData> {
            let (tail, m_data) = GameSCheatEventData::parse(input)?;
            tracing::debug!("m_data: {:?}", m_data);
            Ok((tail, m_data))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSessionCheatEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_data: Option<GameSCheatEventData> = None;
            if m_data.is_none() {
                let (new_tail, parsed_m_data) = Self::parse_m_data(tail)?;
                tail = new_tail;
                m_data = Some(parsed_m_data);
            }
            Ok((
                tail,
                Self {
                    m_data: ok_or_return_missing_field_err!(m_data),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCommandManagerResetEvent {
        pub m_sequence: Uint32,
    }
    impl GameSCommandManagerResetEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sequence(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_sequence) = Uint32::parse(input)?;
            tracing::debug!("m_sequence: {:?}", m_sequence);
            Ok((tail, m_sequence))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCommandManagerResetEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sequence: Option<Uint32> = None;
            if m_sequence.is_none() {
                let (new_tail, parsed_m_sequence) = Self::parse_m_sequence(tail)?;
                tail = new_tail;
                m_sequence = Some(parsed_m_sequence);
            }
            Ok((
                tail,
                Self {
                    m_sequence: ok_or_return_missing_field_err!(m_sequence),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSGameCheatEvent {
        pub m_data: GameSCheatEventData,
    }
    impl GameSGameCheatEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_data(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSCheatEventData> {
            let (tail, m_data) = GameSCheatEventData::parse(input)?;
            tracing::debug!("m_data: {:?}", m_data);
            Ok((tail, m_data))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSGameCheatEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_data: Option<GameSCheatEventData> = None;
            if m_data.is_none() {
                let (new_tail, parsed_m_data) = Self::parse_m_data(tail)?;
                tail = new_tail;
                m_data = Some(parsed_m_data);
            }
            Ok((
                tail,
                Self {
                    m_data: ok_or_return_missing_field_err!(m_data),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCmdEvent {
        pub m_cmd_flags: i64,
        pub m_abil: Option<GameSCmdAbil>,
        pub m_data: GameSCmdData,
        pub m_sequence: i64,
        pub m_other_unit: Option<GameTUnitTag>,
        pub m_unit_group: Option<Uint32>,
    }
    impl GameSCmdEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cmd_flags(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_cmd_flags) = parse_packed_int(input, 0, 27usize)?;
            tracing::debug!("m_cmd_flags: {:?}", m_cmd_flags);
            Ok((tail, m_cmd_flags))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_abil(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameSCmdAbil>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_abil) = if is_provided {
                let (tail, res) = GameSCmdAbil::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_abil: {:?}", m_abil);
            Ok((tail, m_abil))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_data(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameSCmdData> {
            let (tail, m_data) = GameSCmdData::parse(input)?;
            tracing::debug!("m_data: {:?}", m_data);
            Ok((tail, m_data))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sequence(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_sequence) = parse_packed_int(input, 1, 32usize)?;
            tracing::debug!("m_sequence: {:?}", m_sequence);
            Ok((tail, m_sequence))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_other_unit(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTUnitTag>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_other_unit) = if is_provided {
                let (tail, res) = GameTUnitTag::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_other_unit: {:?}", m_other_unit);
            Ok((tail, m_other_unit))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_group(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Uint32>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_unit_group) = if is_provided {
                let (tail, res) = Uint32::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_unit_group: {:?}", m_unit_group);
            Ok((tail, m_unit_group))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCmdEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_cmd_flags: Option<i64> = None;
            let mut m_abil: Option<Option<GameSCmdAbil>> = Some(None);
            let mut m_data: Option<GameSCmdData> = None;
            let mut m_sequence: Option<i64> = None;
            let mut m_other_unit: Option<Option<GameTUnitTag>> = Some(None);
            let mut m_unit_group: Option<Option<Uint32>> = Some(None);
            if m_cmd_flags.is_none() {
                let (new_tail, parsed_m_cmd_flags) = Self::parse_m_cmd_flags(tail)?;
                tail = new_tail;
                m_cmd_flags = Some(parsed_m_cmd_flags);
            }
            if let Some(None) = m_abil {
                let (new_tail, parsed_m_abil) = Self::parse_m_abil(tail)?;
                tail = new_tail;
                m_abil = Some(parsed_m_abil);
            }
            if m_data.is_none() {
                let (new_tail, parsed_m_data) = Self::parse_m_data(tail)?;
                tail = new_tail;
                m_data = Some(parsed_m_data);
            }
            if m_sequence.is_none() {
                let (new_tail, parsed_m_sequence) = Self::parse_m_sequence(tail)?;
                tail = new_tail;
                m_sequence = Some(parsed_m_sequence);
            }
            if let Some(None) = m_other_unit {
                let (new_tail, parsed_m_other_unit) = Self::parse_m_other_unit(tail)?;
                tail = new_tail;
                m_other_unit = Some(parsed_m_other_unit);
            }
            if let Some(None) = m_unit_group {
                let (new_tail, parsed_m_unit_group) = Self::parse_m_unit_group(tail)?;
                tail = new_tail;
                m_unit_group = Some(parsed_m_unit_group);
            }
            Ok((
                tail,
                Self {
                    m_cmd_flags: ok_or_return_missing_field_err!(m_cmd_flags),
                    m_abil: ok_or_return_missing_field_err!(m_abil),
                    m_data: ok_or_return_missing_field_err!(m_data),
                    m_sequence: ok_or_return_missing_field_err!(m_sequence),
                    m_other_unit: ok_or_return_missing_field_err!(m_other_unit),
                    m_unit_group: ok_or_return_missing_field_err!(m_unit_group),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSelectionDeltaEvent {
        pub m_control_group_id: GameTControlGroupId,
        pub m_delta: GameSSelectionDelta,
    }
    impl GameSSelectionDeltaEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_control_group_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTControlGroupId> {
            let (tail, m_control_group_id) = GameTControlGroupId::parse(input)?;
            tracing::debug!("m_control_group_id: {:?}", m_control_group_id);
            Ok((tail, m_control_group_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_delta(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSSelectionDelta> {
            let (tail, m_delta) = GameSSelectionDelta::parse(input)?;
            tracing::debug!("m_delta: {:?}", m_delta);
            Ok((tail, m_delta))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSelectionDeltaEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_control_group_id: Option<GameTControlGroupId> = None;
            let mut m_delta: Option<GameSSelectionDelta> = None;
            if m_control_group_id.is_none() {
                let (new_tail, parsed_m_control_group_id) = Self::parse_m_control_group_id(tail)?;
                tail = new_tail;
                m_control_group_id = Some(parsed_m_control_group_id);
            }
            if m_delta.is_none() {
                let (new_tail, parsed_m_delta) = Self::parse_m_delta(tail)?;
                tail = new_tail;
                m_delta = Some(parsed_m_delta);
            }
            Ok((
                tail,
                Self {
                    m_control_group_id: ok_or_return_missing_field_err!(m_control_group_id),
                    m_delta: ok_or_return_missing_field_err!(m_delta),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSControlGroupUpdateEvent {
        pub m_control_group_index: GameTControlGroupIndex,
        pub m_control_group_update: GameEControlGroupUpdate,
        pub m_mask: GameSSelectionMask,
    }
    impl GameSControlGroupUpdateEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_control_group_index(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTControlGroupIndex> {
            let (tail, m_control_group_index) = GameTControlGroupIndex::parse(input)?;
            tracing::debug!("m_control_group_index: {:?}", m_control_group_index);
            Ok((tail, m_control_group_index))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_control_group_update(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEControlGroupUpdate> {
            let (tail, m_control_group_update) = GameEControlGroupUpdate::parse(input)?;
            tracing::debug!("m_control_group_update: {:?}", m_control_group_update);
            Ok((tail, m_control_group_update))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_mask(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSSelectionMask> {
            let (tail, m_mask) = GameSSelectionMask::parse(input)?;
            tracing::debug!("m_mask: {:?}", m_mask);
            Ok((tail, m_mask))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSControlGroupUpdateEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_control_group_index: Option<GameTControlGroupIndex> = None;
            let mut m_control_group_update: Option<GameEControlGroupUpdate> = None;
            let mut m_mask: Option<GameSSelectionMask> = None;
            if m_control_group_index.is_none() {
                let (new_tail, parsed_m_control_group_index) =
                    Self::parse_m_control_group_index(tail)?;
                tail = new_tail;
                m_control_group_index = Some(parsed_m_control_group_index);
            }
            if m_control_group_update.is_none() {
                let (new_tail, parsed_m_control_group_update) =
                    Self::parse_m_control_group_update(tail)?;
                tail = new_tail;
                m_control_group_update = Some(parsed_m_control_group_update);
            }
            if m_mask.is_none() {
                let (new_tail, parsed_m_mask) = Self::parse_m_mask(tail)?;
                tail = new_tail;
                m_mask = Some(parsed_m_mask);
            }
            Ok((
                tail,
                Self {
                    m_control_group_index: ok_or_return_missing_field_err!(m_control_group_index),
                    m_control_group_update: ok_or_return_missing_field_err!(m_control_group_update),
                    m_mask: ok_or_return_missing_field_err!(m_mask),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSelectionSyncCheckEvent {
        pub m_control_group_id: GameTControlGroupId,
        pub m_selection_sync_data: GameSSelectionSyncData,
    }
    impl GameSSelectionSyncCheckEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_control_group_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTControlGroupId> {
            let (tail, m_control_group_id) = GameTControlGroupId::parse(input)?;
            tracing::debug!("m_control_group_id: {:?}", m_control_group_id);
            Ok((tail, m_control_group_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_selection_sync_data(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSSelectionSyncData> {
            let (tail, m_selection_sync_data) = GameSSelectionSyncData::parse(input)?;
            tracing::debug!("m_selection_sync_data: {:?}", m_selection_sync_data);
            Ok((tail, m_selection_sync_data))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSelectionSyncCheckEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_control_group_id: Option<GameTControlGroupId> = None;
            let mut m_selection_sync_data: Option<GameSSelectionSyncData> = None;
            if m_control_group_id.is_none() {
                let (new_tail, parsed_m_control_group_id) = Self::parse_m_control_group_id(tail)?;
                tail = new_tail;
                m_control_group_id = Some(parsed_m_control_group_id);
            }
            if m_selection_sync_data.is_none() {
                let (new_tail, parsed_m_selection_sync_data) =
                    Self::parse_m_selection_sync_data(tail)?;
                tail = new_tail;
                m_selection_sync_data = Some(parsed_m_selection_sync_data);
            }
            Ok((
                tail,
                Self {
                    m_control_group_id: ok_or_return_missing_field_err!(m_control_group_id),
                    m_selection_sync_data: ok_or_return_missing_field_err!(m_selection_sync_data),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSResourceTradeEvent {
        pub m_recipient_id: GameTPlayerId,
        pub m_resources: Vec<Int32>,
    }
    impl GameSResourceTradeEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_recipient_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTPlayerId> {
            let (tail, m_recipient_id) = GameTPlayerId::parse(input)?;
            tracing::debug!("m_recipient_id: {:?}", m_recipient_id);
            Ok((tail, m_recipient_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_resources(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<Int32>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 3)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = Int32::parse(tail)?;
                tail = new_tail;
                let data = data;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSResourceTradeEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_recipient_id: Option<GameTPlayerId> = None;
            let mut m_resources: Option<Vec<Int32>> = None;
            if m_recipient_id.is_none() {
                let (new_tail, parsed_m_recipient_id) = Self::parse_m_recipient_id(tail)?;
                tail = new_tail;
                m_recipient_id = Some(parsed_m_recipient_id);
            }
            if m_resources.is_none() {
                let (new_tail, parsed_m_resources) = Self::parse_m_resources(tail)?;
                tail = new_tail;
                m_resources = Some(parsed_m_resources);
            }
            Ok((
                tail,
                Self {
                    m_recipient_id: ok_or_return_missing_field_err!(m_recipient_id),
                    m_resources: ok_or_return_missing_field_err!(m_resources),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerChatMessageEvent {
        pub m_chat_message: GameCTriggerChatMessageString,
    }
    impl GameSTriggerChatMessageEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_chat_message(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCTriggerChatMessageString> {
            let (tail, m_chat_message) = GameCTriggerChatMessageString::parse(input)?;
            tracing::debug!("m_chat_message: {:?}", m_chat_message);
            Ok((tail, m_chat_message))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerChatMessageEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_chat_message: Option<GameCTriggerChatMessageString> = None;
            if m_chat_message.is_none() {
                let (new_tail, parsed_m_chat_message) = Self::parse_m_chat_message(tail)?;
                tail = new_tail;
                m_chat_message = Some(parsed_m_chat_message);
            }
            Ok((
                tail,
                Self {
                    m_chat_message: ok_or_return_missing_field_err!(m_chat_message),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSaiCommunicateEvent {
        pub m_beacon: Int8,
        pub m_ally: Int8,
        pub m_flags: Int8,
        pub m_build: Int8,
        pub m_target_unit_tag: GameTUnitTag,
        pub m_target_unit_snapshot_unit_link: GameTUnitLink,
        pub m_target_unit_snapshot_upkeep_player_id: Int8,
        pub m_target_unit_snapshot_control_player_id: Int8,
        pub m_target_point: GameSPoint3,
    }
    impl GameSaiCommunicateEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_beacon(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_beacon) = Int8::parse(input)?;
            tracing::debug!("m_beacon: {:?}", m_beacon);
            Ok((tail, m_beacon))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_ally(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_ally) = Int8::parse(input)?;
            tracing::debug!("m_ally: {:?}", m_ally);
            Ok((tail, m_ally))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_flags(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_flags) = Int8::parse(input)?;
            tracing::debug!("m_flags: {:?}", m_flags);
            Ok((tail, m_flags))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_build(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_build) = Int8::parse(input)?;
            tracing::debug!("m_build: {:?}", m_build);
            Ok((tail, m_build))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target_unit_tag(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTUnitTag> {
            let (tail, m_target_unit_tag) = GameTUnitTag::parse(input)?;
            tracing::debug!("m_target_unit_tag: {:?}", m_target_unit_tag);
            Ok((tail, m_target_unit_tag))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target_unit_snapshot_unit_link(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTUnitLink> {
            let (tail, m_target_unit_snapshot_unit_link) = GameTUnitLink::parse(input)?;
            tracing::debug!(
                "m_target_unit_snapshot_unit_link: {:?}",
                m_target_unit_snapshot_unit_link
            );
            Ok((tail, m_target_unit_snapshot_unit_link))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target_unit_snapshot_upkeep_player_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_target_unit_snapshot_upkeep_player_id) = Int8::parse(input)?;
            tracing::debug!(
                "m_target_unit_snapshot_upkeep_player_id: {:?}",
                m_target_unit_snapshot_upkeep_player_id
            );
            Ok((tail, m_target_unit_snapshot_upkeep_player_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target_unit_snapshot_control_player_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_target_unit_snapshot_control_player_id) = Int8::parse(input)?;
            tracing::debug!(
                "m_target_unit_snapshot_control_player_id: {:?}",
                m_target_unit_snapshot_control_player_id
            );
            Ok((tail, m_target_unit_snapshot_control_player_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target_point(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSPoint3> {
            let (tail, m_target_point) = GameSPoint3::parse(input)?;
            tracing::debug!("m_target_point: {:?}", m_target_point);
            Ok((tail, m_target_point))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSaiCommunicateEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_beacon: Option<Int8> = None;
            let mut m_ally: Option<Int8> = None;
            let mut m_flags: Option<Int8> = None;
            let mut m_build: Option<Int8> = None;
            let mut m_target_unit_tag: Option<GameTUnitTag> = None;
            let mut m_target_unit_snapshot_unit_link: Option<GameTUnitLink> = None;
            let mut m_target_unit_snapshot_upkeep_player_id: Option<Int8> = None;
            let mut m_target_unit_snapshot_control_player_id: Option<Int8> = None;
            let mut m_target_point: Option<GameSPoint3> = None;
            if m_beacon.is_none() {
                let (new_tail, parsed_m_beacon) = Self::parse_m_beacon(tail)?;
                tail = new_tail;
                m_beacon = Some(parsed_m_beacon);
            }
            if m_ally.is_none() {
                let (new_tail, parsed_m_ally) = Self::parse_m_ally(tail)?;
                tail = new_tail;
                m_ally = Some(parsed_m_ally);
            }
            if m_flags.is_none() {
                let (new_tail, parsed_m_flags) = Self::parse_m_flags(tail)?;
                tail = new_tail;
                m_flags = Some(parsed_m_flags);
            }
            if m_build.is_none() {
                let (new_tail, parsed_m_build) = Self::parse_m_build(tail)?;
                tail = new_tail;
                m_build = Some(parsed_m_build);
            }
            if m_target_unit_tag.is_none() {
                let (new_tail, parsed_m_target_unit_tag) = Self::parse_m_target_unit_tag(tail)?;
                tail = new_tail;
                m_target_unit_tag = Some(parsed_m_target_unit_tag);
            }
            if m_target_unit_snapshot_unit_link.is_none() {
                let (new_tail, parsed_m_target_unit_snapshot_unit_link) =
                    Self::parse_m_target_unit_snapshot_unit_link(tail)?;
                tail = new_tail;
                m_target_unit_snapshot_unit_link = Some(parsed_m_target_unit_snapshot_unit_link);
            }
            if m_target_unit_snapshot_upkeep_player_id.is_none() {
                let (new_tail, parsed_m_target_unit_snapshot_upkeep_player_id) =
                    Self::parse_m_target_unit_snapshot_upkeep_player_id(tail)?;
                tail = new_tail;
                m_target_unit_snapshot_upkeep_player_id =
                    Some(parsed_m_target_unit_snapshot_upkeep_player_id);
            }
            if m_target_unit_snapshot_control_player_id.is_none() {
                let (new_tail, parsed_m_target_unit_snapshot_control_player_id) =
                    Self::parse_m_target_unit_snapshot_control_player_id(tail)?;
                tail = new_tail;
                m_target_unit_snapshot_control_player_id =
                    Some(parsed_m_target_unit_snapshot_control_player_id);
            }
            if m_target_point.is_none() {
                let (new_tail, parsed_m_target_point) = Self::parse_m_target_point(tail)?;
                tail = new_tail;
                m_target_point = Some(parsed_m_target_point);
            }
            Ok((
                tail,
                Self {
                    m_beacon: ok_or_return_missing_field_err!(m_beacon),
                    m_ally: ok_or_return_missing_field_err!(m_ally),
                    m_flags: ok_or_return_missing_field_err!(m_flags),
                    m_build: ok_or_return_missing_field_err!(m_build),
                    m_target_unit_tag: ok_or_return_missing_field_err!(m_target_unit_tag),
                    m_target_unit_snapshot_unit_link: ok_or_return_missing_field_err!(
                        m_target_unit_snapshot_unit_link
                    ),
                    m_target_unit_snapshot_upkeep_player_id: ok_or_return_missing_field_err!(
                        m_target_unit_snapshot_upkeep_player_id
                    ),
                    m_target_unit_snapshot_control_player_id: ok_or_return_missing_field_err!(
                        m_target_unit_snapshot_control_player_id
                    ),
                    m_target_point: ok_or_return_missing_field_err!(m_target_point),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSetAbsoluteGameSpeedEvent {
        pub m_speed: GameEGameSpeed,
    }
    impl GameSSetAbsoluteGameSpeedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_speed(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEGameSpeed> {
            let (tail, m_speed) = GameEGameSpeed::parse(input)?;
            tracing::debug!("m_speed: {:?}", m_speed);
            Ok((tail, m_speed))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSetAbsoluteGameSpeedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_speed: Option<GameEGameSpeed> = None;
            if m_speed.is_none() {
                let (new_tail, parsed_m_speed) = Self::parse_m_speed(tail)?;
                tail = new_tail;
                m_speed = Some(parsed_m_speed);
            }
            Ok((
                tail,
                Self {
                    m_speed: ok_or_return_missing_field_err!(m_speed),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSAddAbsoluteGameSpeedEvent {
        pub m_delta: Int8,
    }
    impl GameSAddAbsoluteGameSpeedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_delta(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_delta) = Int8::parse(input)?;
            tracing::debug!("m_delta: {:?}", m_delta);
            Ok((tail, m_delta))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSAddAbsoluteGameSpeedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_delta: Option<Int8> = None;
            if m_delta.is_none() {
                let (new_tail, parsed_m_delta) = Self::parse_m_delta(tail)?;
                tail = new_tail;
                m_delta = Some(parsed_m_delta);
            }
            Ok((
                tail,
                Self {
                    m_delta: ok_or_return_missing_field_err!(m_delta),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPingEvent {
        pub m_point: GameSPoint,
        pub m_unit: GameTUnitTag,
        pub m_unit_link: GameTUnitLink,
        pub m_unit_control_player_id: Option<GameTPlayerId>,
        pub m_unit_upkeep_player_id: Option<GameTPlayerId>,
        pub m_unit_position: GameSMapCoord3D,
        pub m_unit_is_under_construction: bool,
        pub m_pinged_minimap: bool,
        pub m_option: Int32,
    }
    impl GameSTriggerPingEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_point(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameSPoint> {
            let (tail, m_point) = GameSPoint::parse(input)?;
            tracing::debug!("m_point: {:?}", m_point);
            Ok((tail, m_point))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTUnitTag> {
            let (tail, m_unit) = GameTUnitTag::parse(input)?;
            tracing::debug!("m_unit: {:?}", m_unit);
            Ok((tail, m_unit))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_link(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTUnitLink> {
            let (tail, m_unit_link) = GameTUnitLink::parse(input)?;
            tracing::debug!("m_unit_link: {:?}", m_unit_link);
            Ok((tail, m_unit_link))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_control_player_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTPlayerId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_unit_control_player_id) = if is_provided {
                let (tail, res) = GameTPlayerId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_unit_control_player_id: {:?}", m_unit_control_player_id);
            Ok((tail, m_unit_control_player_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_upkeep_player_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTPlayerId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_unit_upkeep_player_id) = if is_provided {
                let (tail, res) = GameTPlayerId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_unit_upkeep_player_id: {:?}", m_unit_upkeep_player_id);
            Ok((tail, m_unit_upkeep_player_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_position(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSMapCoord3D> {
            let (tail, m_unit_position) = GameSMapCoord3D::parse(input)?;
            tracing::debug!("m_unit_position: {:?}", m_unit_position);
            Ok((tail, m_unit_position))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_is_under_construction(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_unit_is_under_construction) = parse_bool(input)?;
            tracing::debug!(
                "m_unit_is_under_construction: {:?}",
                m_unit_is_under_construction
            );
            Ok((tail, m_unit_is_under_construction))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_pinged_minimap(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_pinged_minimap) = parse_bool(input)?;
            tracing::debug!("m_pinged_minimap: {:?}", m_pinged_minimap);
            Ok((tail, m_pinged_minimap))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_option(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_option) = Int32::parse(input)?;
            tracing::debug!("m_option: {:?}", m_option);
            Ok((tail, m_option))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPingEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_point: Option<GameSPoint> = None;
            let mut m_unit: Option<GameTUnitTag> = None;
            let mut m_unit_link: Option<GameTUnitLink> = None;
            let mut m_unit_control_player_id: Option<Option<GameTPlayerId>> = Some(None);
            let mut m_unit_upkeep_player_id: Option<Option<GameTPlayerId>> = Some(None);
            let mut m_unit_position: Option<GameSMapCoord3D> = None;
            let mut m_unit_is_under_construction: Option<bool> = None;
            let mut m_pinged_minimap: Option<bool> = None;
            let mut m_option: Option<Int32> = None;
            if m_point.is_none() {
                let (new_tail, parsed_m_point) = Self::parse_m_point(tail)?;
                tail = new_tail;
                m_point = Some(parsed_m_point);
            }
            if m_unit.is_none() {
                let (new_tail, parsed_m_unit) = Self::parse_m_unit(tail)?;
                tail = new_tail;
                m_unit = Some(parsed_m_unit);
            }
            if m_unit_link.is_none() {
                let (new_tail, parsed_m_unit_link) = Self::parse_m_unit_link(tail)?;
                tail = new_tail;
                m_unit_link = Some(parsed_m_unit_link);
            }
            if let Some(None) = m_unit_control_player_id {
                let (new_tail, parsed_m_unit_control_player_id) =
                    Self::parse_m_unit_control_player_id(tail)?;
                tail = new_tail;
                m_unit_control_player_id = Some(parsed_m_unit_control_player_id);
            }
            if let Some(None) = m_unit_upkeep_player_id {
                let (new_tail, parsed_m_unit_upkeep_player_id) =
                    Self::parse_m_unit_upkeep_player_id(tail)?;
                tail = new_tail;
                m_unit_upkeep_player_id = Some(parsed_m_unit_upkeep_player_id);
            }
            if m_unit_position.is_none() {
                let (new_tail, parsed_m_unit_position) = Self::parse_m_unit_position(tail)?;
                tail = new_tail;
                m_unit_position = Some(parsed_m_unit_position);
            }
            if m_unit_is_under_construction.is_none() {
                let (new_tail, parsed_m_unit_is_under_construction) =
                    Self::parse_m_unit_is_under_construction(tail)?;
                tail = new_tail;
                m_unit_is_under_construction = Some(parsed_m_unit_is_under_construction);
            }
            if m_pinged_minimap.is_none() {
                let (new_tail, parsed_m_pinged_minimap) = Self::parse_m_pinged_minimap(tail)?;
                tail = new_tail;
                m_pinged_minimap = Some(parsed_m_pinged_minimap);
            }
            if m_option.is_none() {
                let (new_tail, parsed_m_option) = Self::parse_m_option(tail)?;
                tail = new_tail;
                m_option = Some(parsed_m_option);
            }
            Ok((
                tail,
                Self {
                    m_point: ok_or_return_missing_field_err!(m_point),
                    m_unit: ok_or_return_missing_field_err!(m_unit),
                    m_unit_link: ok_or_return_missing_field_err!(m_unit_link),
                    m_unit_control_player_id: ok_or_return_missing_field_err!(
                        m_unit_control_player_id
                    ),
                    m_unit_upkeep_player_id: ok_or_return_missing_field_err!(
                        m_unit_upkeep_player_id
                    ),
                    m_unit_position: ok_or_return_missing_field_err!(m_unit_position),
                    m_unit_is_under_construction: ok_or_return_missing_field_err!(
                        m_unit_is_under_construction
                    ),
                    m_pinged_minimap: ok_or_return_missing_field_err!(m_pinged_minimap),
                    m_option: ok_or_return_missing_field_err!(m_option),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSBroadcastCheatEvent {
        pub m_verb: GameCCheatString,
        pub m_arguments: GameCCheatString,
    }
    impl GameSBroadcastCheatEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_verb(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCCheatString> {
            let (tail, m_verb) = GameCCheatString::parse(input)?;
            tracing::debug!("m_verb: {:?}", m_verb);
            Ok((tail, m_verb))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_arguments(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCCheatString> {
            let (tail, m_arguments) = GameCCheatString::parse(input)?;
            tracing::debug!("m_arguments: {:?}", m_arguments);
            Ok((tail, m_arguments))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSBroadcastCheatEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_verb: Option<GameCCheatString> = None;
            let mut m_arguments: Option<GameCCheatString> = None;
            if m_verb.is_none() {
                let (new_tail, parsed_m_verb) = Self::parse_m_verb(tail)?;
                tail = new_tail;
                m_verb = Some(parsed_m_verb);
            }
            if m_arguments.is_none() {
                let (new_tail, parsed_m_arguments) = Self::parse_m_arguments(tail)?;
                tail = new_tail;
                m_arguments = Some(parsed_m_arguments);
            }
            Ok((
                tail,
                Self {
                    m_verb: ok_or_return_missing_field_err!(m_verb),
                    m_arguments: ok_or_return_missing_field_err!(m_arguments),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSAllianceEvent {
        pub m_alliance: Uint32,
        pub m_control: Uint32,
    }
    impl GameSAllianceEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_alliance(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_alliance) = Uint32::parse(input)?;
            tracing::debug!("m_alliance: {:?}", m_alliance);
            Ok((tail, m_alliance))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_control(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_control) = Uint32::parse(input)?;
            tracing::debug!("m_control: {:?}", m_control);
            Ok((tail, m_control))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSAllianceEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_alliance: Option<Uint32> = None;
            let mut m_control: Option<Uint32> = None;
            if m_alliance.is_none() {
                let (new_tail, parsed_m_alliance) = Self::parse_m_alliance(tail)?;
                tail = new_tail;
                m_alliance = Some(parsed_m_alliance);
            }
            if m_control.is_none() {
                let (new_tail, parsed_m_control) = Self::parse_m_control(tail)?;
                tail = new_tail;
                m_control = Some(parsed_m_control);
            }
            Ok((
                tail,
                Self {
                    m_alliance: ok_or_return_missing_field_err!(m_alliance),
                    m_control: ok_or_return_missing_field_err!(m_control),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSUnitClickEvent {
        pub m_unit_tag: GameTUnitTag,
    }
    impl GameSUnitClickEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_tag(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTUnitTag> {
            let (tail, m_unit_tag) = GameTUnitTag::parse(input)?;
            tracing::debug!("m_unit_tag: {:?}", m_unit_tag);
            Ok((tail, m_unit_tag))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSUnitClickEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_unit_tag: Option<GameTUnitTag> = None;
            if m_unit_tag.is_none() {
                let (new_tail, parsed_m_unit_tag) = Self::parse_m_unit_tag(tail)?;
                tail = new_tail;
                m_unit_tag = Some(parsed_m_unit_tag);
            }
            Ok((
                tail,
                Self {
                    m_unit_tag: ok_or_return_missing_field_err!(m_unit_tag),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSUnitHighlightEvent {
        pub m_unit_tag: GameTUnitTag,
        pub m_flags: Uint8,
    }
    impl GameSUnitHighlightEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_tag(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTUnitTag> {
            let (tail, m_unit_tag) = GameTUnitTag::parse(input)?;
            tracing::debug!("m_unit_tag: {:?}", m_unit_tag);
            Ok((tail, m_unit_tag))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_flags(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_flags) = Uint8::parse(input)?;
            tracing::debug!("m_flags: {:?}", m_flags);
            Ok((tail, m_flags))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSUnitHighlightEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_unit_tag: Option<GameTUnitTag> = None;
            let mut m_flags: Option<Uint8> = None;
            if m_unit_tag.is_none() {
                let (new_tail, parsed_m_unit_tag) = Self::parse_m_unit_tag(tail)?;
                tail = new_tail;
                m_unit_tag = Some(parsed_m_unit_tag);
            }
            if m_flags.is_none() {
                let (new_tail, parsed_m_flags) = Self::parse_m_flags(tail)?;
                tail = new_tail;
                m_flags = Some(parsed_m_flags);
            }
            Ok((
                tail,
                Self {
                    m_unit_tag: ok_or_return_missing_field_err!(m_unit_tag),
                    m_flags: ok_or_return_missing_field_err!(m_flags),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerReplySelectedEvent {
        pub m_conversation_id: Int32,
        pub m_reply_id: Int32,
    }
    impl GameSTriggerReplySelectedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_conversation_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_conversation_id) = Int32::parse(input)?;
            tracing::debug!("m_conversation_id: {:?}", m_conversation_id);
            Ok((tail, m_conversation_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_reply_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_reply_id) = Int32::parse(input)?;
            tracing::debug!("m_reply_id: {:?}", m_reply_id);
            Ok((tail, m_reply_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerReplySelectedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_conversation_id: Option<Int32> = None;
            let mut m_reply_id: Option<Int32> = None;
            if m_conversation_id.is_none() {
                let (new_tail, parsed_m_conversation_id) = Self::parse_m_conversation_id(tail)?;
                tail = new_tail;
                m_conversation_id = Some(parsed_m_conversation_id);
            }
            if m_reply_id.is_none() {
                let (new_tail, parsed_m_reply_id) = Self::parse_m_reply_id(tail)?;
                tail = new_tail;
                m_reply_id = Some(parsed_m_reply_id);
            }
            Ok((
                tail,
                Self {
                    m_conversation_id: ok_or_return_missing_field_err!(m_conversation_id),
                    m_reply_id: ok_or_return_missing_field_err!(m_reply_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSHijackReplaySessionUserInfo {
        pub m_session_user_id: TUserId,
        pub m_add_new_game_user: bool,
        pub m_game_user_id: TUserId,
    }
    impl GameSHijackReplaySessionUserInfo {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_session_user_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TUserId> {
            let (tail, m_session_user_id) = TUserId::parse(input)?;
            tracing::debug!("m_session_user_id: {:?}", m_session_user_id);
            Ok((tail, m_session_user_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_add_new_game_user(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_add_new_game_user) = parse_bool(input)?;
            tracing::debug!("m_add_new_game_user: {:?}", m_add_new_game_user);
            Ok((tail, m_add_new_game_user))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_user_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TUserId> {
            let (tail, m_game_user_id) = TUserId::parse(input)?;
            tracing::debug!("m_game_user_id: {:?}", m_game_user_id);
            Ok((tail, m_game_user_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSHijackReplaySessionUserInfo::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_session_user_id: Option<TUserId> = None;
            let mut m_add_new_game_user: Option<bool> = None;
            let mut m_game_user_id: Option<TUserId> = None;
            if m_session_user_id.is_none() {
                let (new_tail, parsed_m_session_user_id) = Self::parse_m_session_user_id(tail)?;
                tail = new_tail;
                m_session_user_id = Some(parsed_m_session_user_id);
            }
            if m_add_new_game_user.is_none() {
                let (new_tail, parsed_m_add_new_game_user) = Self::parse_m_add_new_game_user(tail)?;
                tail = new_tail;
                m_add_new_game_user = Some(parsed_m_add_new_game_user);
            }
            if m_game_user_id.is_none() {
                let (new_tail, parsed_m_game_user_id) = Self::parse_m_game_user_id(tail)?;
                tail = new_tail;
                m_game_user_id = Some(parsed_m_game_user_id);
            }
            Ok((
                tail,
                Self {
                    m_session_user_id: ok_or_return_missing_field_err!(m_session_user_id),
                    m_add_new_game_user: ok_or_return_missing_field_err!(m_add_new_game_user),
                    m_game_user_id: ok_or_return_missing_field_err!(m_game_user_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSHijackReplaySessionEvent {
        pub m_user_infos: Vec<GameSHijackReplaySessionUserInfo>,
        pub m_method: GameEHijackMethod,
    }
    impl GameSHijackReplaySessionEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_user_infos(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<GameSHijackReplaySessionUserInfo>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 5)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = GameSHijackReplaySessionUserInfo::parse(tail)?;
                tail = new_tail;
                let data = data;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_method(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEHijackMethod> {
            let (tail, m_method) = GameEHijackMethod::parse(input)?;
            tracing::debug!("m_method: {:?}", m_method);
            Ok((tail, m_method))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSHijackReplaySessionEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_user_infos: Option<Vec<GameSHijackReplaySessionUserInfo>> = None;
            let mut m_method: Option<GameEHijackMethod> = None;
            if m_user_infos.is_none() {
                let (new_tail, parsed_m_user_infos) = Self::parse_m_user_infos(tail)?;
                tail = new_tail;
                m_user_infos = Some(parsed_m_user_infos);
            }
            if m_method.is_none() {
                let (new_tail, parsed_m_method) = Self::parse_m_method(tail)?;
                tail = new_tail;
                m_method = Some(parsed_m_method);
            }
            Ok((
                tail,
                Self {
                    m_user_infos: ok_or_return_missing_field_err!(m_user_infos),
                    m_method: ok_or_return_missing_field_err!(m_method),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSHijackReplayGameUserInfo {
        pub m_game_user_id: TUserId,
        pub m_observe: EObserve,
        pub m_name: CUserName,
        pub m_toon_handle: Option<CToonHandle>,
        pub m_clan_tag: Option<CClanTag>,
        pub m_clan_logo: Option<GameCCacheHandle>,
    }
    impl GameSHijackReplayGameUserInfo {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_user_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TUserId> {
            let (tail, m_game_user_id) = TUserId::parse(input)?;
            tracing::debug!("m_game_user_id: {:?}", m_game_user_id);
            Ok((tail, m_game_user_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_observe(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), EObserve> {
            let (tail, m_observe) = EObserve::parse(input)?;
            tracing::debug!("m_observe: {:?}", m_observe);
            Ok((tail, m_observe))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CUserName> {
            let (tail, m_name) = CUserName::parse(input)?;
            tracing::debug!("m_name: {:?}", str::from_utf8(&m_name.value));
            Ok((tail, m_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_toon_handle(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<CToonHandle>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_toon_handle) = if is_provided {
                let (tail, res) = CToonHandle::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_toon_handle: {:?}", m_toon_handle);
            Ok((tail, m_toon_handle))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_clan_tag(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<CClanTag>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_clan_tag) = if is_provided {
                let (tail, res) = CClanTag::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_clan_tag: {:?}", m_clan_tag);
            Ok((tail, m_clan_tag))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_clan_logo(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameCCacheHandle>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_clan_logo) = if is_provided {
                let (tail, res) = GameCCacheHandle::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_clan_logo: {:?}", m_clan_logo);
            Ok((tail, m_clan_logo))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSHijackReplayGameUserInfo::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_game_user_id: Option<TUserId> = None;
            let mut m_observe: Option<EObserve> = None;
            let mut m_name: Option<CUserName> = None;
            let mut m_toon_handle: Option<Option<CToonHandle>> = Some(None);
            let mut m_clan_tag: Option<Option<CClanTag>> = Some(None);
            let mut m_clan_logo: Option<Option<GameCCacheHandle>> = Some(None);
            if m_game_user_id.is_none() {
                let (new_tail, parsed_m_game_user_id) = Self::parse_m_game_user_id(tail)?;
                tail = new_tail;
                m_game_user_id = Some(parsed_m_game_user_id);
            }
            if m_observe.is_none() {
                let (new_tail, parsed_m_observe) = Self::parse_m_observe(tail)?;
                tail = new_tail;
                m_observe = Some(parsed_m_observe);
            }
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            if let Some(None) = m_toon_handle {
                let (new_tail, parsed_m_toon_handle) = Self::parse_m_toon_handle(tail)?;
                tail = new_tail;
                m_toon_handle = Some(parsed_m_toon_handle);
            }
            if let Some(None) = m_clan_tag {
                let (new_tail, parsed_m_clan_tag) = Self::parse_m_clan_tag(tail)?;
                tail = new_tail;
                m_clan_tag = Some(parsed_m_clan_tag);
            }
            if let Some(None) = m_clan_logo {
                let (new_tail, parsed_m_clan_logo) = Self::parse_m_clan_logo(tail)?;
                tail = new_tail;
                m_clan_logo = Some(parsed_m_clan_logo);
            }
            Ok((
                tail,
                Self {
                    m_game_user_id: ok_or_return_missing_field_err!(m_game_user_id),
                    m_observe: ok_or_return_missing_field_err!(m_observe),
                    m_name: ok_or_return_missing_field_err!(m_name),
                    m_toon_handle: ok_or_return_missing_field_err!(m_toon_handle),
                    m_clan_tag: ok_or_return_missing_field_err!(m_clan_tag),
                    m_clan_logo: ok_or_return_missing_field_err!(m_clan_logo),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSHijackReplayGameEvent {
        pub m_user_infos: Vec<GameSHijackReplayGameUserInfo>,
        pub m_method: GameEHijackMethod,
    }
    impl GameSHijackReplayGameEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_user_infos(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<GameSHijackReplayGameUserInfo>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 5)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = GameSHijackReplayGameUserInfo::parse(tail)?;
                tail = new_tail;
                let data = data;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_method(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEHijackMethod> {
            let (tail, m_method) = GameEHijackMethod::parse(input)?;
            tracing::debug!("m_method: {:?}", m_method);
            Ok((tail, m_method))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSHijackReplayGameEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_user_infos: Option<Vec<GameSHijackReplayGameUserInfo>> = None;
            let mut m_method: Option<GameEHijackMethod> = None;
            if m_user_infos.is_none() {
                let (new_tail, parsed_m_user_infos) = Self::parse_m_user_infos(tail)?;
                tail = new_tail;
                m_user_infos = Some(parsed_m_user_infos);
            }
            if m_method.is_none() {
                let (new_tail, parsed_m_method) = Self::parse_m_method(tail)?;
                tail = new_tail;
                m_method = Some(parsed_m_method);
            }
            Ok((
                tail,
                Self {
                    m_user_infos: ok_or_return_missing_field_err!(m_user_infos),
                    m_method: ok_or_return_missing_field_err!(m_method),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerAbortMissionEvent {}
    impl GameSTriggerAbortMissionEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerAbortMissionEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPurchaseMadeEvent {
        pub m_purchase_item_id: Int32,
    }
    impl GameSTriggerPurchaseMadeEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_purchase_item_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_purchase_item_id) = Int32::parse(input)?;
            tracing::debug!("m_purchase_item_id: {:?}", m_purchase_item_id);
            Ok((tail, m_purchase_item_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPurchaseMadeEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_purchase_item_id: Option<Int32> = None;
            if m_purchase_item_id.is_none() {
                let (new_tail, parsed_m_purchase_item_id) = Self::parse_m_purchase_item_id(tail)?;
                tail = new_tail;
                m_purchase_item_id = Some(parsed_m_purchase_item_id);
            }
            Ok((
                tail,
                Self {
                    m_purchase_item_id: ok_or_return_missing_field_err!(m_purchase_item_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPurchaseExitEvent {}
    impl GameSTriggerPurchaseExitEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPurchaseExitEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPlanetMissionLaunchedEvent {
        pub m_difficulty_level: Int32,
    }
    impl GameSTriggerPlanetMissionLaunchedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_difficulty_level(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_difficulty_level) = Int32::parse(input)?;
            tracing::debug!("m_difficulty_level: {:?}", m_difficulty_level);
            Ok((tail, m_difficulty_level))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPlanetMissionLaunchedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_difficulty_level: Option<Int32> = None;
            if m_difficulty_level.is_none() {
                let (new_tail, parsed_m_difficulty_level) = Self::parse_m_difficulty_level(tail)?;
                tail = new_tail;
                m_difficulty_level = Some(parsed_m_difficulty_level);
            }
            Ok((
                tail,
                Self {
                    m_difficulty_level: ok_or_return_missing_field_err!(m_difficulty_level),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPlanetPanelCanceledEvent {}
    impl GameSTriggerPlanetPanelCanceledEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPlanetPanelCanceledEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerDialogControlEvent {
        pub m_control_id: Int32,
        pub m_event_type: Int32,
        pub m_event_data: MEventData,
    }
    impl GameSTriggerDialogControlEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_control_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_control_id) = Int32::parse(input)?;
            tracing::debug!("m_control_id: {:?}", m_control_id);
            Ok((tail, m_control_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_event_type(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_event_type) = Int32::parse(input)?;
            tracing::debug!("m_event_type: {:?}", m_event_type);
            Ok((tail, m_event_type))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_event_data(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), MEventData> {
            let (tail, m_event_data) = MEventData::parse(input)?;
            tracing::debug!("m_event_data: {:?}", m_event_data);
            Ok((tail, m_event_data))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerDialogControlEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_control_id: Option<Int32> = None;
            let mut m_event_type: Option<Int32> = None;
            let mut m_event_data: Option<MEventData> = None;
            if m_control_id.is_none() {
                let (new_tail, parsed_m_control_id) = Self::parse_m_control_id(tail)?;
                tail = new_tail;
                m_control_id = Some(parsed_m_control_id);
            }
            if m_event_type.is_none() {
                let (new_tail, parsed_m_event_type) = Self::parse_m_event_type(tail)?;
                tail = new_tail;
                m_event_type = Some(parsed_m_event_type);
            }
            if m_event_data.is_none() {
                let (new_tail, parsed_m_event_data) = Self::parse_m_event_data(tail)?;
                tail = new_tail;
                m_event_data = Some(parsed_m_event_data);
            }
            Ok((
                tail,
                Self {
                    m_control_id: ok_or_return_missing_field_err!(m_control_id),
                    m_event_type: ok_or_return_missing_field_err!(m_event_type),
                    m_event_data: ok_or_return_missing_field_err!(m_event_data),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum MEventData {
        None(()),
        Checked(bool),
        ValueChanged(Uint32),
        SelectionChanged(Int32),
        TextChanged(GameCChatString),
        MouseButton(Uint32),
    }
    impl MEventData {
        #[tracing::instrument(name="87702::MEventData::ChoiceType::parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // ChoiceType:
            // Use the number of elements in the json .fields to calculate how many
            // bits to have unique tags.
            // let num_fields: usize = 6;
            let offset = 0i64;
            let num_bits: usize = 3;
            let (tail, variant_tag) = parse_packed_int(input, offset, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant tagged '0' for None");
                    let (tail, res) = take_null(tail)?;
                    Ok((tail, Self::None(res)))
                }
                1 => {
                    tracing::debug!("Variant tagged '1' for Checked");
                    let (tail, res) = parse_bool(tail)?;
                    Ok((tail, Self::Checked(res)))
                }
                2 => {
                    tracing::debug!("Variant tagged '2' for ValueChanged");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::ValueChanged(res)))
                }
                3 => {
                    tracing::debug!("Variant tagged '3' for SelectionChanged");
                    let (tail, res) = Int32::parse(tail)?;
                    Ok((tail, Self::SelectionChanged(res)))
                }
                4 => {
                    tracing::debug!("Variant tagged '4' for TextChanged");
                    let (tail, res) = GameCChatString::parse(tail)?;
                    Ok((tail, Self::TextChanged(res)))
                }
                5 => {
                    tracing::debug!("Variant tagged '5' for MouseButton");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::MouseButton(res)))
                }

                _ => {
                    tracing::error!("Unknown variant for tag {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerSkippedEvent {}
    impl GameSTriggerSkippedEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerSkippedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerSoundLengthQueryEvent {
        pub m_sound_hash: Uint32,
        pub m_length: Uint32,
    }
    impl GameSTriggerSoundLengthQueryEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sound_hash(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_sound_hash) = Uint32::parse(input)?;
            tracing::debug!("m_sound_hash: {:?}", m_sound_hash);
            Ok((tail, m_sound_hash))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_length(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_length) = Uint32::parse(input)?;
            tracing::debug!("m_length: {:?}", m_length);
            Ok((tail, m_length))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerSoundLengthQueryEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sound_hash: Option<Uint32> = None;
            let mut m_length: Option<Uint32> = None;
            if m_sound_hash.is_none() {
                let (new_tail, parsed_m_sound_hash) = Self::parse_m_sound_hash(tail)?;
                tail = new_tail;
                m_sound_hash = Some(parsed_m_sound_hash);
            }
            if m_length.is_none() {
                let (new_tail, parsed_m_length) = Self::parse_m_length(tail)?;
                tail = new_tail;
                m_length = Some(parsed_m_length);
            }
            Ok((
                tail,
                Self {
                    m_sound_hash: ok_or_return_missing_field_err!(m_sound_hash),
                    m_length: ok_or_return_missing_field_err!(m_length),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerSoundLengthSyncEvent {
        pub m_sync_info: GameSSyncSoundLength,
    }
    impl GameSTriggerSoundLengthSyncEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sync_info(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSSyncSoundLength> {
            let (tail, m_sync_info) = GameSSyncSoundLength::parse(input)?;
            tracing::debug!("m_sync_info: {:?}", m_sync_info);
            Ok((tail, m_sync_info))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerSoundLengthSyncEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sync_info: Option<GameSSyncSoundLength> = None;
            if m_sync_info.is_none() {
                let (new_tail, parsed_m_sync_info) = Self::parse_m_sync_info(tail)?;
                tail = new_tail;
                m_sync_info = Some(parsed_m_sync_info);
            }
            Ok((
                tail,
                Self {
                    m_sync_info: ok_or_return_missing_field_err!(m_sync_info),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerAnimLengthQueryByNameEvent {
        pub m_query_id: GameTQueryId,
        pub m_length_ms: Uint32,
        pub m_finish_game_loop: Uint32,
    }
    impl GameSTriggerAnimLengthQueryByNameEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_query_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTQueryId> {
            let (tail, m_query_id) = GameTQueryId::parse(input)?;
            tracing::debug!("m_query_id: {:?}", m_query_id);
            Ok((tail, m_query_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_length_ms(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_length_ms) = Uint32::parse(input)?;
            tracing::debug!("m_length_ms: {:?}", m_length_ms);
            Ok((tail, m_length_ms))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_finish_game_loop(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_finish_game_loop) = Uint32::parse(input)?;
            tracing::debug!("m_finish_game_loop: {:?}", m_finish_game_loop);
            Ok((tail, m_finish_game_loop))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerAnimLengthQueryByNameEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_query_id: Option<GameTQueryId> = None;
            let mut m_length_ms: Option<Uint32> = None;
            let mut m_finish_game_loop: Option<Uint32> = None;
            if m_query_id.is_none() {
                let (new_tail, parsed_m_query_id) = Self::parse_m_query_id(tail)?;
                tail = new_tail;
                m_query_id = Some(parsed_m_query_id);
            }
            if m_length_ms.is_none() {
                let (new_tail, parsed_m_length_ms) = Self::parse_m_length_ms(tail)?;
                tail = new_tail;
                m_length_ms = Some(parsed_m_length_ms);
            }
            if m_finish_game_loop.is_none() {
                let (new_tail, parsed_m_finish_game_loop) = Self::parse_m_finish_game_loop(tail)?;
                tail = new_tail;
                m_finish_game_loop = Some(parsed_m_finish_game_loop);
            }
            Ok((
                tail,
                Self {
                    m_query_id: ok_or_return_missing_field_err!(m_query_id),
                    m_length_ms: ok_or_return_missing_field_err!(m_length_ms),
                    m_finish_game_loop: ok_or_return_missing_field_err!(m_finish_game_loop),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerAnimLengthQueryByPropsEvent {
        pub m_query_id: GameTQueryId,
        pub m_length_ms: Uint32,
    }
    impl GameSTriggerAnimLengthQueryByPropsEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_query_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTQueryId> {
            let (tail, m_query_id) = GameTQueryId::parse(input)?;
            tracing::debug!("m_query_id: {:?}", m_query_id);
            Ok((tail, m_query_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_length_ms(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_length_ms) = Uint32::parse(input)?;
            tracing::debug!("m_length_ms: {:?}", m_length_ms);
            Ok((tail, m_length_ms))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerAnimLengthQueryByPropsEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_query_id: Option<GameTQueryId> = None;
            let mut m_length_ms: Option<Uint32> = None;
            if m_query_id.is_none() {
                let (new_tail, parsed_m_query_id) = Self::parse_m_query_id(tail)?;
                tail = new_tail;
                m_query_id = Some(parsed_m_query_id);
            }
            if m_length_ms.is_none() {
                let (new_tail, parsed_m_length_ms) = Self::parse_m_length_ms(tail)?;
                tail = new_tail;
                m_length_ms = Some(parsed_m_length_ms);
            }
            Ok((
                tail,
                Self {
                    m_query_id: ok_or_return_missing_field_err!(m_query_id),
                    m_length_ms: ok_or_return_missing_field_err!(m_length_ms),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerAnimOffsetEvent {
        pub m_anim_wait_query_id: GameTQueryId,
    }
    impl GameSTriggerAnimOffsetEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_anim_wait_query_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTQueryId> {
            let (tail, m_anim_wait_query_id) = GameTQueryId::parse(input)?;
            tracing::debug!("m_anim_wait_query_id: {:?}", m_anim_wait_query_id);
            Ok((tail, m_anim_wait_query_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerAnimOffsetEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_anim_wait_query_id: Option<GameTQueryId> = None;
            if m_anim_wait_query_id.is_none() {
                let (new_tail, parsed_m_anim_wait_query_id) =
                    Self::parse_m_anim_wait_query_id(tail)?;
                tail = new_tail;
                m_anim_wait_query_id = Some(parsed_m_anim_wait_query_id);
            }
            Ok((
                tail,
                Self {
                    m_anim_wait_query_id: ok_or_return_missing_field_err!(m_anim_wait_query_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerSoundOffsetEvent {
        pub m_sound: GameTTriggerSoundTag,
    }
    impl GameSTriggerSoundOffsetEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sound(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTTriggerSoundTag> {
            let (tail, m_sound) = GameTTriggerSoundTag::parse(input)?;
            tracing::debug!("m_sound: {:?}", m_sound);
            Ok((tail, m_sound))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerSoundOffsetEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sound: Option<GameTTriggerSoundTag> = None;
            if m_sound.is_none() {
                let (new_tail, parsed_m_sound) = Self::parse_m_sound(tail)?;
                tail = new_tail;
                m_sound = Some(parsed_m_sound);
            }
            Ok((
                tail,
                Self {
                    m_sound: ok_or_return_missing_field_err!(m_sound),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerTransmissionOffsetEvent {
        pub m_transmission_id: Int32,
        pub m_thread: GameTTriggerThreadTag,
    }
    impl GameSTriggerTransmissionOffsetEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_transmission_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_transmission_id) = Int32::parse(input)?;
            tracing::debug!("m_transmission_id: {:?}", m_transmission_id);
            Ok((tail, m_transmission_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_thread(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTTriggerThreadTag> {
            let (tail, m_thread) = GameTTriggerThreadTag::parse(input)?;
            tracing::debug!("m_thread: {:?}", m_thread);
            Ok((tail, m_thread))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerTransmissionOffsetEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_transmission_id: Option<Int32> = None;
            let mut m_thread: Option<GameTTriggerThreadTag> = None;
            if m_transmission_id.is_none() {
                let (new_tail, parsed_m_transmission_id) = Self::parse_m_transmission_id(tail)?;
                tail = new_tail;
                m_transmission_id = Some(parsed_m_transmission_id);
            }
            if m_thread.is_none() {
                let (new_tail, parsed_m_thread) = Self::parse_m_thread(tail)?;
                tail = new_tail;
                m_thread = Some(parsed_m_thread);
            }
            Ok((
                tail,
                Self {
                    m_transmission_id: ok_or_return_missing_field_err!(m_transmission_id),
                    m_thread: ok_or_return_missing_field_err!(m_thread),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerTransmissionCompleteEvent {
        pub m_transmission_id: Int32,
    }
    impl GameSTriggerTransmissionCompleteEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_transmission_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_transmission_id) = Int32::parse(input)?;
            tracing::debug!("m_transmission_id: {:?}", m_transmission_id);
            Ok((tail, m_transmission_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerTransmissionCompleteEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_transmission_id: Option<Int32> = None;
            if m_transmission_id.is_none() {
                let (new_tail, parsed_m_transmission_id) = Self::parse_m_transmission_id(tail)?;
                tail = new_tail;
                m_transmission_id = Some(parsed_m_transmission_id);
            }
            Ok((
                tail,
                Self {
                    m_transmission_id: ok_or_return_missing_field_err!(m_transmission_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCameraUpdateEvent {
        pub m_target: Option<GameSPointMini>,
        pub m_distance: Option<GameTFixedMiniBitsUnsigned>,
        pub m_pitch: Option<GameTFixedMiniBitsUnsigned>,
        pub m_yaw: Option<GameTFixedMiniBitsUnsigned>,
        pub m_reason: Option<Int8>,
        pub m_follow: bool,
    }
    impl GameSCameraUpdateEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameSPointMini>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_target) = if is_provided {
                let (tail, res) = GameSPointMini::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_target: {:?}", m_target);
            Ok((tail, m_target))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_distance(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTFixedMiniBitsUnsigned>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_distance) = if is_provided {
                let (tail, res) = GameTFixedMiniBitsUnsigned::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_distance: {:?}", m_distance);
            Ok((tail, m_distance))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_pitch(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTFixedMiniBitsUnsigned>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_pitch) = if is_provided {
                let (tail, res) = GameTFixedMiniBitsUnsigned::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_pitch: {:?}", m_pitch);
            Ok((tail, m_pitch))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_yaw(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameTFixedMiniBitsUnsigned>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_yaw) = if is_provided {
                let (tail, res) = GameTFixedMiniBitsUnsigned::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_yaw: {:?}", m_yaw);
            Ok((tail, m_yaw))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_reason(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Int8>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_reason) = if is_provided {
                let (tail, res) = Int8::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_reason: {:?}", m_reason);
            Ok((tail, m_reason))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_follow(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_follow) = parse_bool(input)?;
            tracing::debug!("m_follow: {:?}", m_follow);
            Ok((tail, m_follow))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCameraUpdateEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_target: Option<Option<GameSPointMini>> = Some(None);
            let mut m_distance: Option<Option<GameTFixedMiniBitsUnsigned>> = Some(None);
            let mut m_pitch: Option<Option<GameTFixedMiniBitsUnsigned>> = Some(None);
            let mut m_yaw: Option<Option<GameTFixedMiniBitsUnsigned>> = Some(None);
            let mut m_reason: Option<Option<Int8>> = Some(None);
            let mut m_follow: Option<bool> = None;
            if let Some(None) = m_target {
                let (new_tail, parsed_m_target) = Self::parse_m_target(tail)?;
                tail = new_tail;
                m_target = Some(parsed_m_target);
            }
            if let Some(None) = m_distance {
                let (new_tail, parsed_m_distance) = Self::parse_m_distance(tail)?;
                tail = new_tail;
                m_distance = Some(parsed_m_distance);
            }
            if let Some(None) = m_pitch {
                let (new_tail, parsed_m_pitch) = Self::parse_m_pitch(tail)?;
                tail = new_tail;
                m_pitch = Some(parsed_m_pitch);
            }
            if let Some(None) = m_yaw {
                let (new_tail, parsed_m_yaw) = Self::parse_m_yaw(tail)?;
                tail = new_tail;
                m_yaw = Some(parsed_m_yaw);
            }
            if let Some(None) = m_reason {
                let (new_tail, parsed_m_reason) = Self::parse_m_reason(tail)?;
                tail = new_tail;
                m_reason = Some(parsed_m_reason);
            }
            if m_follow.is_none() {
                let (new_tail, parsed_m_follow) = Self::parse_m_follow(tail)?;
                tail = new_tail;
                m_follow = Some(parsed_m_follow);
            }
            Ok((
                tail,
                Self {
                    m_target: ok_or_return_missing_field_err!(m_target),
                    m_distance: ok_or_return_missing_field_err!(m_distance),
                    m_pitch: ok_or_return_missing_field_err!(m_pitch),
                    m_yaw: ok_or_return_missing_field_err!(m_yaw),
                    m_reason: ok_or_return_missing_field_err!(m_reason),
                    m_follow: ok_or_return_missing_field_err!(m_follow),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerConversationSkippedEvent {
        pub m_skip_type: GameEConversationSkip,
    }
    impl GameSTriggerConversationSkippedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_skip_type(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEConversationSkip> {
            let (tail, m_skip_type) = GameEConversationSkip::parse(input)?;
            tracing::debug!("m_skip_type: {:?}", m_skip_type);
            Ok((tail, m_skip_type))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerConversationSkippedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_skip_type: Option<GameEConversationSkip> = None;
            if m_skip_type.is_none() {
                let (new_tail, parsed_m_skip_type) = Self::parse_m_skip_type(tail)?;
                tail = new_tail;
                m_skip_type = Some(parsed_m_skip_type);
            }
            Ok((
                tail,
                Self {
                    m_skip_type: ok_or_return_missing_field_err!(m_skip_type),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerMouseClickedEvent {
        pub m_button: Uint32,
        pub m_down: bool,
        pub m_pos_ui: GameSuiCoord,
        pub m_pos_world: GameSMapCoord3D,
        pub m_flags: Int8,
    }
    impl GameSTriggerMouseClickedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_button(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_button) = Uint32::parse(input)?;
            tracing::debug!("m_button: {:?}", m_button);
            Ok((tail, m_button))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_down(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_down) = parse_bool(input)?;
            tracing::debug!("m_down: {:?}", m_down);
            Ok((tail, m_down))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_pos_ui(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSuiCoord> {
            let (tail, m_pos_ui) = GameSuiCoord::parse(input)?;
            tracing::debug!("m_pos_ui: {:?}", m_pos_ui);
            Ok((tail, m_pos_ui))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_pos_world(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSMapCoord3D> {
            let (tail, m_pos_world) = GameSMapCoord3D::parse(input)?;
            tracing::debug!("m_pos_world: {:?}", m_pos_world);
            Ok((tail, m_pos_world))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_flags(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_flags) = Int8::parse(input)?;
            tracing::debug!("m_flags: {:?}", m_flags);
            Ok((tail, m_flags))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerMouseClickedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_button: Option<Uint32> = None;
            let mut m_down: Option<bool> = None;
            let mut m_pos_ui: Option<GameSuiCoord> = None;
            let mut m_pos_world: Option<GameSMapCoord3D> = None;
            let mut m_flags: Option<Int8> = None;
            if m_button.is_none() {
                let (new_tail, parsed_m_button) = Self::parse_m_button(tail)?;
                tail = new_tail;
                m_button = Some(parsed_m_button);
            }
            if m_down.is_none() {
                let (new_tail, parsed_m_down) = Self::parse_m_down(tail)?;
                tail = new_tail;
                m_down = Some(parsed_m_down);
            }
            if m_pos_ui.is_none() {
                let (new_tail, parsed_m_pos_ui) = Self::parse_m_pos_ui(tail)?;
                tail = new_tail;
                m_pos_ui = Some(parsed_m_pos_ui);
            }
            if m_pos_world.is_none() {
                let (new_tail, parsed_m_pos_world) = Self::parse_m_pos_world(tail)?;
                tail = new_tail;
                m_pos_world = Some(parsed_m_pos_world);
            }
            if m_flags.is_none() {
                let (new_tail, parsed_m_flags) = Self::parse_m_flags(tail)?;
                tail = new_tail;
                m_flags = Some(parsed_m_flags);
            }
            Ok((
                tail,
                Self {
                    m_button: ok_or_return_missing_field_err!(m_button),
                    m_down: ok_or_return_missing_field_err!(m_down),
                    m_pos_ui: ok_or_return_missing_field_err!(m_pos_ui),
                    m_pos_world: ok_or_return_missing_field_err!(m_pos_world),
                    m_flags: ok_or_return_missing_field_err!(m_flags),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerMouseMovedEvent {
        pub m_pos_ui: GameSuiCoord,
        pub m_pos_world: GameSMapCoord3D,
        pub m_flags: Int8,
    }
    impl GameSTriggerMouseMovedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_pos_ui(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSuiCoord> {
            let (tail, m_pos_ui) = GameSuiCoord::parse(input)?;
            tracing::debug!("m_pos_ui: {:?}", m_pos_ui);
            Ok((tail, m_pos_ui))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_pos_world(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSMapCoord3D> {
            let (tail, m_pos_world) = GameSMapCoord3D::parse(input)?;
            tracing::debug!("m_pos_world: {:?}", m_pos_world);
            Ok((tail, m_pos_world))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_flags(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_flags) = Int8::parse(input)?;
            tracing::debug!("m_flags: {:?}", m_flags);
            Ok((tail, m_flags))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerMouseMovedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_pos_ui: Option<GameSuiCoord> = None;
            let mut m_pos_world: Option<GameSMapCoord3D> = None;
            let mut m_flags: Option<Int8> = None;
            if m_pos_ui.is_none() {
                let (new_tail, parsed_m_pos_ui) = Self::parse_m_pos_ui(tail)?;
                tail = new_tail;
                m_pos_ui = Some(parsed_m_pos_ui);
            }
            if m_pos_world.is_none() {
                let (new_tail, parsed_m_pos_world) = Self::parse_m_pos_world(tail)?;
                tail = new_tail;
                m_pos_world = Some(parsed_m_pos_world);
            }
            if m_flags.is_none() {
                let (new_tail, parsed_m_flags) = Self::parse_m_flags(tail)?;
                tail = new_tail;
                m_flags = Some(parsed_m_flags);
            }
            Ok((
                tail,
                Self {
                    m_pos_ui: ok_or_return_missing_field_err!(m_pos_ui),
                    m_pos_world: ok_or_return_missing_field_err!(m_pos_world),
                    m_flags: ok_or_return_missing_field_err!(m_flags),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSAchievementAwardedEvent {
        pub m_achievement_link: GameTAchievementLink,
    }
    impl GameSAchievementAwardedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_achievement_link(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTAchievementLink> {
            let (tail, m_achievement_link) = GameTAchievementLink::parse(input)?;
            tracing::debug!("m_achievement_link: {:?}", m_achievement_link);
            Ok((tail, m_achievement_link))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSAchievementAwardedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_achievement_link: Option<GameTAchievementLink> = None;
            if m_achievement_link.is_none() {
                let (new_tail, parsed_m_achievement_link) = Self::parse_m_achievement_link(tail)?;
                tail = new_tail;
                m_achievement_link = Some(parsed_m_achievement_link);
            }
            Ok((
                tail,
                Self {
                    m_achievement_link: ok_or_return_missing_field_err!(m_achievement_link),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerHotkeyPressedEvent {
        pub m_hotkey: Uint32,
        pub m_down: bool,
    }
    impl GameSTriggerHotkeyPressedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_hotkey(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_hotkey) = Uint32::parse(input)?;
            tracing::debug!("m_hotkey: {:?}", m_hotkey);
            Ok((tail, m_hotkey))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_down(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_down) = parse_bool(input)?;
            tracing::debug!("m_down: {:?}", m_down);
            Ok((tail, m_down))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerHotkeyPressedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_hotkey: Option<Uint32> = None;
            let mut m_down: Option<bool> = None;
            if m_hotkey.is_none() {
                let (new_tail, parsed_m_hotkey) = Self::parse_m_hotkey(tail)?;
                tail = new_tail;
                m_hotkey = Some(parsed_m_hotkey);
            }
            if m_down.is_none() {
                let (new_tail, parsed_m_down) = Self::parse_m_down(tail)?;
                tail = new_tail;
                m_down = Some(parsed_m_down);
            }
            Ok((
                tail,
                Self {
                    m_hotkey: ok_or_return_missing_field_err!(m_hotkey),
                    m_down: ok_or_return_missing_field_err!(m_down),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerTargetModeUpdateEvent {
        pub m_abil_link: GameTAbilLink,
        pub m_abil_cmd_index: i64,
        pub m_state: Int8,
    }
    impl GameSTriggerTargetModeUpdateEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_abil_link(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTAbilLink> {
            let (tail, m_abil_link) = GameTAbilLink::parse(input)?;
            tracing::debug!("m_abil_link: {:?}", m_abil_link);
            Ok((tail, m_abil_link))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_abil_cmd_index(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_abil_cmd_index) = parse_packed_int(input, 0, 5usize)?;
            tracing::debug!("m_abil_cmd_index: {:?}", m_abil_cmd_index);
            Ok((tail, m_abil_cmd_index))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_state(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_state) = Int8::parse(input)?;
            tracing::debug!("m_state: {:?}", m_state);
            Ok((tail, m_state))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerTargetModeUpdateEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_abil_link: Option<GameTAbilLink> = None;
            let mut m_abil_cmd_index: Option<i64> = None;
            let mut m_state: Option<Int8> = None;
            if m_abil_link.is_none() {
                let (new_tail, parsed_m_abil_link) = Self::parse_m_abil_link(tail)?;
                tail = new_tail;
                m_abil_link = Some(parsed_m_abil_link);
            }
            if m_abil_cmd_index.is_none() {
                let (new_tail, parsed_m_abil_cmd_index) = Self::parse_m_abil_cmd_index(tail)?;
                tail = new_tail;
                m_abil_cmd_index = Some(parsed_m_abil_cmd_index);
            }
            if m_state.is_none() {
                let (new_tail, parsed_m_state) = Self::parse_m_state(tail)?;
                tail = new_tail;
                m_state = Some(parsed_m_state);
            }
            Ok((
                tail,
                Self {
                    m_abil_link: ok_or_return_missing_field_err!(m_abil_link),
                    m_abil_cmd_index: ok_or_return_missing_field_err!(m_abil_cmd_index),
                    m_state: ok_or_return_missing_field_err!(m_state),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPlanetPanelReplayEvent {}
    impl GameSTriggerPlanetPanelReplayEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPlanetPanelReplayEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerSoundtrackDoneEvent {
        pub m_soundtrack: Uint32,
    }
    impl GameSTriggerSoundtrackDoneEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_soundtrack(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_soundtrack) = Uint32::parse(input)?;
            tracing::debug!("m_soundtrack: {:?}", m_soundtrack);
            Ok((tail, m_soundtrack))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerSoundtrackDoneEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_soundtrack: Option<Uint32> = None;
            if m_soundtrack.is_none() {
                let (new_tail, parsed_m_soundtrack) = Self::parse_m_soundtrack(tail)?;
                tail = new_tail;
                m_soundtrack = Some(parsed_m_soundtrack);
            }
            Ok((
                tail,
                Self {
                    m_soundtrack: ok_or_return_missing_field_err!(m_soundtrack),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPlanetMissionSelectedEvent {
        pub m_planet_id: Int32,
    }
    impl GameSTriggerPlanetMissionSelectedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_planet_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_planet_id) = Int32::parse(input)?;
            tracing::debug!("m_planet_id: {:?}", m_planet_id);
            Ok((tail, m_planet_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPlanetMissionSelectedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_planet_id: Option<Int32> = None;
            if m_planet_id.is_none() {
                let (new_tail, parsed_m_planet_id) = Self::parse_m_planet_id(tail)?;
                tail = new_tail;
                m_planet_id = Some(parsed_m_planet_id);
            }
            Ok((
                tail,
                Self {
                    m_planet_id: ok_or_return_missing_field_err!(m_planet_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerKeyPressedEvent {
        pub m_key: Int8,
        pub m_flags: Int8,
    }
    impl GameSTriggerKeyPressedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_key(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_key) = Int8::parse(input)?;
            tracing::debug!("m_key: {:?}", m_key);
            Ok((tail, m_key))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_flags(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_flags) = Int8::parse(input)?;
            tracing::debug!("m_flags: {:?}", m_flags);
            Ok((tail, m_flags))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerKeyPressedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_key: Option<Int8> = None;
            let mut m_flags: Option<Int8> = None;
            if m_key.is_none() {
                let (new_tail, parsed_m_key) = Self::parse_m_key(tail)?;
                tail = new_tail;
                m_key = Some(parsed_m_key);
            }
            if m_flags.is_none() {
                let (new_tail, parsed_m_flags) = Self::parse_m_flags(tail)?;
                tail = new_tail;
                m_flags = Some(parsed_m_flags);
            }
            Ok((
                tail,
                Self {
                    m_key: ok_or_return_missing_field_err!(m_key),
                    m_flags: ok_or_return_missing_field_err!(m_flags),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPlanetPanelBirthCompleteEvent {}
    impl GameSTriggerPlanetPanelBirthCompleteEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPlanetPanelBirthCompleteEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPlanetPanelDeathCompleteEvent {}
    impl GameSTriggerPlanetPanelDeathCompleteEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPlanetPanelDeathCompleteEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSResourceRequestEvent {
        pub m_resources: Vec<Int32>,
    }
    impl GameSResourceRequestEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_resources(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<Int32>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 3)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = Int32::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSResourceRequestEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_resources: Option<Vec<Int32>> = None;
            if m_resources.is_none() {
                let (new_tail, parsed_m_resources) = Self::parse_m_resources(tail)?;
                tail = new_tail;
                m_resources = Some(parsed_m_resources);
            }
            Ok((
                tail,
                Self {
                    m_resources: ok_or_return_missing_field_err!(m_resources),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSResourceRequestFulfillEvent {
        pub m_fulfill_request_id: Int32,
    }
    impl GameSResourceRequestFulfillEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_fulfill_request_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_fulfill_request_id) = Int32::parse(input)?;
            tracing::debug!("m_fulfill_request_id: {:?}", m_fulfill_request_id);
            Ok((tail, m_fulfill_request_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSResourceRequestFulfillEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_fulfill_request_id: Option<Int32> = None;
            if m_fulfill_request_id.is_none() {
                let (new_tail, parsed_m_fulfill_request_id) =
                    Self::parse_m_fulfill_request_id(tail)?;
                tail = new_tail;
                m_fulfill_request_id = Some(parsed_m_fulfill_request_id);
            }
            Ok((
                tail,
                Self {
                    m_fulfill_request_id: ok_or_return_missing_field_err!(m_fulfill_request_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSResourceRequestCancelEvent {
        pub m_cancel_request_id: Int32,
    }
    impl GameSResourceRequestCancelEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cancel_request_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_cancel_request_id) = Int32::parse(input)?;
            tracing::debug!("m_cancel_request_id: {:?}", m_cancel_request_id);
            Ok((tail, m_cancel_request_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSResourceRequestCancelEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_cancel_request_id: Option<Int32> = None;
            if m_cancel_request_id.is_none() {
                let (new_tail, parsed_m_cancel_request_id) = Self::parse_m_cancel_request_id(tail)?;
                tail = new_tail;
                m_cancel_request_id = Some(parsed_m_cancel_request_id);
            }
            Ok((
                tail,
                Self {
                    m_cancel_request_id: ok_or_return_missing_field_err!(m_cancel_request_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerResearchPanelExitEvent {}
    impl GameSTriggerResearchPanelExitEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerResearchPanelExitEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerResearchPanelPurchaseEvent {}
    impl GameSTriggerResearchPanelPurchaseEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerResearchPanelPurchaseEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerCommandErrorEvent {
        pub m_error: Int32,
        pub m_abil: Option<GameSCmdAbil>,
    }
    impl GameSTriggerCommandErrorEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_error(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_error) = Int32::parse(input)?;
            tracing::debug!("m_error: {:?}", m_error);
            Ok((tail, m_error))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_abil(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameSCmdAbil>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_abil) = if is_provided {
                let (tail, res) = GameSCmdAbil::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_abil: {:?}", m_abil);
            Ok((tail, m_abil))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerCommandErrorEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_error: Option<Int32> = None;
            let mut m_abil: Option<Option<GameSCmdAbil>> = Some(None);
            if m_error.is_none() {
                let (new_tail, parsed_m_error) = Self::parse_m_error(tail)?;
                tail = new_tail;
                m_error = Some(parsed_m_error);
            }
            if let Some(None) = m_abil {
                let (new_tail, parsed_m_abil) = Self::parse_m_abil(tail)?;
                tail = new_tail;
                m_abil = Some(parsed_m_abil);
            }
            Ok((
                tail,
                Self {
                    m_error: ok_or_return_missing_field_err!(m_error),
                    m_abil: ok_or_return_missing_field_err!(m_abil),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerResearchPanelSelectionChangedEvent {
        pub m_research_item_id: Int32,
    }
    impl GameSTriggerResearchPanelSelectionChangedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_research_item_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_research_item_id) = Int32::parse(input)?;
            tracing::debug!("m_research_item_id: {:?}", m_research_item_id);
            Ok((tail, m_research_item_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerResearchPanelSelectionChangedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_research_item_id: Option<Int32> = None;
            if m_research_item_id.is_none() {
                let (new_tail, parsed_m_research_item_id) = Self::parse_m_research_item_id(tail)?;
                tail = new_tail;
                m_research_item_id = Some(parsed_m_research_item_id);
            }
            Ok((
                tail,
                Self {
                    m_research_item_id: ok_or_return_missing_field_err!(m_research_item_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerMercenaryPanelExitEvent {}
    impl GameSTriggerMercenaryPanelExitEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerMercenaryPanelExitEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerMercenaryPanelPurchaseEvent {}
    impl GameSTriggerMercenaryPanelPurchaseEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerMercenaryPanelPurchaseEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerMercenaryPanelSelectionChangedEvent {
        pub m_mercenary_id: Int32,
    }
    impl GameSTriggerMercenaryPanelSelectionChangedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_mercenary_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_mercenary_id) = Int32::parse(input)?;
            tracing::debug!("m_mercenary_id: {:?}", m_mercenary_id);
            Ok((tail, m_mercenary_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerMercenaryPanelSelectionChangedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_mercenary_id: Option<Int32> = None;
            if m_mercenary_id.is_none() {
                let (new_tail, parsed_m_mercenary_id) = Self::parse_m_mercenary_id(tail)?;
                tail = new_tail;
                m_mercenary_id = Some(parsed_m_mercenary_id);
            }
            Ok((
                tail,
                Self {
                    m_mercenary_id: ok_or_return_missing_field_err!(m_mercenary_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerVictoryPanelExitEvent {}
    impl GameSTriggerVictoryPanelExitEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerVictoryPanelExitEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerBattleReportPanelExitEvent {}
    impl GameSTriggerBattleReportPanelExitEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerBattleReportPanelExitEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerBattleReportPanelPlayMissionEvent {
        pub m_battle_report_id: Int32,
        pub m_difficulty_level: Int32,
    }
    impl GameSTriggerBattleReportPanelPlayMissionEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_battle_report_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_battle_report_id) = Int32::parse(input)?;
            tracing::debug!("m_battle_report_id: {:?}", m_battle_report_id);
            Ok((tail, m_battle_report_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_difficulty_level(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_difficulty_level) = Int32::parse(input)?;
            tracing::debug!("m_difficulty_level: {:?}", m_difficulty_level);
            Ok((tail, m_difficulty_level))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerBattleReportPanelPlayMissionEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_battle_report_id: Option<Int32> = None;
            let mut m_difficulty_level: Option<Int32> = None;
            if m_battle_report_id.is_none() {
                let (new_tail, parsed_m_battle_report_id) = Self::parse_m_battle_report_id(tail)?;
                tail = new_tail;
                m_battle_report_id = Some(parsed_m_battle_report_id);
            }
            if m_difficulty_level.is_none() {
                let (new_tail, parsed_m_difficulty_level) = Self::parse_m_difficulty_level(tail)?;
                tail = new_tail;
                m_difficulty_level = Some(parsed_m_difficulty_level);
            }
            Ok((
                tail,
                Self {
                    m_battle_report_id: ok_or_return_missing_field_err!(m_battle_report_id),
                    m_difficulty_level: ok_or_return_missing_field_err!(m_difficulty_level),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerBattleReportPanelPlaySceneEvent {
        pub m_battle_report_id: Int32,
    }
    impl GameSTriggerBattleReportPanelPlaySceneEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_battle_report_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_battle_report_id) = Int32::parse(input)?;
            tracing::debug!("m_battle_report_id: {:?}", m_battle_report_id);
            Ok((tail, m_battle_report_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerBattleReportPanelPlaySceneEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_battle_report_id: Option<Int32> = None;
            if m_battle_report_id.is_none() {
                let (new_tail, parsed_m_battle_report_id) = Self::parse_m_battle_report_id(tail)?;
                tail = new_tail;
                m_battle_report_id = Some(parsed_m_battle_report_id);
            }
            Ok((
                tail,
                Self {
                    m_battle_report_id: ok_or_return_missing_field_err!(m_battle_report_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerBattleReportPanelSelectionChangedEvent {
        pub m_battle_report_id: Int32,
    }
    impl GameSTriggerBattleReportPanelSelectionChangedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_battle_report_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_battle_report_id) = Int32::parse(input)?;
            tracing::debug!("m_battle_report_id: {:?}", m_battle_report_id);
            Ok((tail, m_battle_report_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerBattleReportPanelSelectionChangedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_battle_report_id: Option<Int32> = None;
            if m_battle_report_id.is_none() {
                let (new_tail, parsed_m_battle_report_id) = Self::parse_m_battle_report_id(tail)?;
                tail = new_tail;
                m_battle_report_id = Some(parsed_m_battle_report_id);
            }
            Ok((
                tail,
                Self {
                    m_battle_report_id: ok_or_return_missing_field_err!(m_battle_report_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerVictoryPanelPlayMissionAgainEvent {
        pub m_difficulty_level: Int32,
    }
    impl GameSTriggerVictoryPanelPlayMissionAgainEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_difficulty_level(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_difficulty_level) = Int32::parse(input)?;
            tracing::debug!("m_difficulty_level: {:?}", m_difficulty_level);
            Ok((tail, m_difficulty_level))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerVictoryPanelPlayMissionAgainEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_difficulty_level: Option<Int32> = None;
            if m_difficulty_level.is_none() {
                let (new_tail, parsed_m_difficulty_level) = Self::parse_m_difficulty_level(tail)?;
                tail = new_tail;
                m_difficulty_level = Some(parsed_m_difficulty_level);
            }
            Ok((
                tail,
                Self {
                    m_difficulty_level: ok_or_return_missing_field_err!(m_difficulty_level),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerMovieStartedEvent {}
    impl GameSTriggerMovieStartedEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerMovieStartedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerMovieFinishedEvent {}
    impl GameSTriggerMovieFinishedEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerMovieFinishedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSDecrementGameTimeRemainingEvent {
        pub m_decrement_seconds: Int32,
    }
    impl GameSDecrementGameTimeRemainingEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_decrement_seconds(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_decrement_seconds) = Int32::parse(input)?;
            tracing::debug!("m_decrement_seconds: {:?}", m_decrement_seconds);
            Ok((tail, m_decrement_seconds))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSDecrementGameTimeRemainingEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_decrement_seconds: Option<Int32> = None;
            if m_decrement_seconds.is_none() {
                let (new_tail, parsed_m_decrement_seconds) = Self::parse_m_decrement_seconds(tail)?;
                tail = new_tail;
                m_decrement_seconds = Some(parsed_m_decrement_seconds);
            }
            Ok((
                tail,
                Self {
                    m_decrement_seconds: ok_or_return_missing_field_err!(m_decrement_seconds),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPortraitLoadedEvent {
        pub m_portrait_id: Int32,
    }
    impl GameSTriggerPortraitLoadedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_portrait_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_portrait_id) = Int32::parse(input)?;
            tracing::debug!("m_portrait_id: {:?}", m_portrait_id);
            Ok((tail, m_portrait_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPortraitLoadedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_portrait_id: Option<Int32> = None;
            if m_portrait_id.is_none() {
                let (new_tail, parsed_m_portrait_id) = Self::parse_m_portrait_id(tail)?;
                tail = new_tail;
                m_portrait_id = Some(parsed_m_portrait_id);
            }
            Ok((
                tail,
                Self {
                    m_portrait_id: ok_or_return_missing_field_err!(m_portrait_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerMovieFunctionEvent {
        pub m_function_name: Vec<u8>,
    }
    impl GameSTriggerMovieFunctionEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_function_name(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerMovieFunctionEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_function_name: Option<Vec<u8>> = None;
            if m_function_name.is_none() {
                let (new_tail, parsed_m_function_name) = Self::parse_m_function_name(tail)?;
                tail = new_tail;
                m_function_name = Some(parsed_m_function_name);
            }
            Ok((
                tail,
                Self {
                    m_function_name: ok_or_return_missing_field_err!(m_function_name),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerCustomDialogDismissedEvent {
        pub m_result: Int32,
    }
    impl GameSTriggerCustomDialogDismissedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_result(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_result) = Int32::parse(input)?;
            tracing::debug!("m_result: {:?}", m_result);
            Ok((tail, m_result))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerCustomDialogDismissedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_result: Option<Int32> = None;
            if m_result.is_none() {
                let (new_tail, parsed_m_result) = Self::parse_m_result(tail)?;
                tail = new_tail;
                m_result = Some(parsed_m_result);
            }
            Ok((
                tail,
                Self {
                    m_result: ok_or_return_missing_field_err!(m_result),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerGameMenuItemSelectedEvent {
        pub m_game_menu_item_index: Int32,
    }
    impl GameSTriggerGameMenuItemSelectedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_menu_item_index(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_game_menu_item_index) = Int32::parse(input)?;
            tracing::debug!("m_game_menu_item_index: {:?}", m_game_menu_item_index);
            Ok((tail, m_game_menu_item_index))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerGameMenuItemSelectedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_game_menu_item_index: Option<Int32> = None;
            if m_game_menu_item_index.is_none() {
                let (new_tail, parsed_m_game_menu_item_index) =
                    Self::parse_m_game_menu_item_index(tail)?;
                tail = new_tail;
                m_game_menu_item_index = Some(parsed_m_game_menu_item_index);
            }
            Ok((
                tail,
                Self {
                    m_game_menu_item_index: ok_or_return_missing_field_err!(m_game_menu_item_index),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerMouseWheelEvent {
        pub m_wheel_spin: GameTFixedMiniBitsSigned,
        pub m_flags: Int8,
    }
    impl GameSTriggerMouseWheelEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_wheel_spin(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTFixedMiniBitsSigned> {
            let (tail, m_wheel_spin) = GameTFixedMiniBitsSigned::parse(input)?;
            tracing::debug!("m_wheel_spin: {:?}", m_wheel_spin);
            Ok((tail, m_wheel_spin))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_flags(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int8> {
            let (tail, m_flags) = Int8::parse(input)?;
            tracing::debug!("m_flags: {:?}", m_flags);
            Ok((tail, m_flags))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerMouseWheelEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_wheel_spin: Option<GameTFixedMiniBitsSigned> = None;
            let mut m_flags: Option<Int8> = None;
            if m_wheel_spin.is_none() {
                let (new_tail, parsed_m_wheel_spin) = Self::parse_m_wheel_spin(tail)?;
                tail = new_tail;
                m_wheel_spin = Some(parsed_m_wheel_spin);
            }
            if m_flags.is_none() {
                let (new_tail, parsed_m_flags) = Self::parse_m_flags(tail)?;
                tail = new_tail;
                m_flags = Some(parsed_m_flags);
            }
            Ok((
                tail,
                Self {
                    m_wheel_spin: ok_or_return_missing_field_err!(m_wheel_spin),
                    m_flags: ok_or_return_missing_field_err!(m_flags),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent {
        pub m_purchase_item_id: Int32,
    }
    impl GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_purchase_item_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_purchase_item_id) = Int32::parse(input)?;
            tracing::debug!("m_purchase_item_id: {:?}", m_purchase_item_id);
            Ok((tail, m_purchase_item_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPurchasePanelSelectedPurchaseItemChangedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_purchase_item_id: Option<Int32> = None;
            if m_purchase_item_id.is_none() {
                let (new_tail, parsed_m_purchase_item_id) = Self::parse_m_purchase_item_id(tail)?;
                tail = new_tail;
                m_purchase_item_id = Some(parsed_m_purchase_item_id);
            }
            Ok((
                tail,
                Self {
                    m_purchase_item_id: ok_or_return_missing_field_err!(m_purchase_item_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent {
        pub m_purchase_category_id: Int32,
    }
    impl GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_purchase_category_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_purchase_category_id) = Int32::parse(input)?;
            tracing::debug!("m_purchase_category_id: {:?}", m_purchase_category_id);
            Ok((tail, m_purchase_category_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerPurchasePanelSelectedPurchaseCategoryChangedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_purchase_category_id: Option<Int32> = None;
            if m_purchase_category_id.is_none() {
                let (new_tail, parsed_m_purchase_category_id) =
                    Self::parse_m_purchase_category_id(tail)?;
                tail = new_tail;
                m_purchase_category_id = Some(parsed_m_purchase_category_id);
            }
            Ok((
                tail,
                Self {
                    m_purchase_category_id: ok_or_return_missing_field_err!(m_purchase_category_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerButtonPressedEvent {
        pub m_button: GameTButtonLink,
    }
    impl GameSTriggerButtonPressedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_button(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTButtonLink> {
            let (tail, m_button) = GameTButtonLink::parse(input)?;
            tracing::debug!("m_button: {:?}", m_button);
            Ok((tail, m_button))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerButtonPressedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_button: Option<GameTButtonLink> = None;
            if m_button.is_none() {
                let (new_tail, parsed_m_button) = Self::parse_m_button(tail)?;
                tail = new_tail;
                m_button = Some(parsed_m_button);
            }
            Ok((
                tail,
                Self {
                    m_button: ok_or_return_missing_field_err!(m_button),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerGameCreditsFinishedEvent {}
    impl GameSTriggerGameCreditsFinishedEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerGameCreditsFinishedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerCutsceneBookmarkFiredEvent {
        pub m_cutscene_id: Int32,
        pub m_bookmark_name: Vec<u8>,
    }
    impl GameSTriggerCutsceneBookmarkFiredEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cutscene_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_cutscene_id) = Int32::parse(input)?;
            tracing::debug!("m_cutscene_id: {:?}", m_cutscene_id);
            Ok((tail, m_cutscene_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_bookmark_name(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerCutsceneBookmarkFiredEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_cutscene_id: Option<Int32> = None;
            let mut m_bookmark_name: Option<Vec<u8>> = None;
            if m_cutscene_id.is_none() {
                let (new_tail, parsed_m_cutscene_id) = Self::parse_m_cutscene_id(tail)?;
                tail = new_tail;
                m_cutscene_id = Some(parsed_m_cutscene_id);
            }
            if m_bookmark_name.is_none() {
                let (new_tail, parsed_m_bookmark_name) = Self::parse_m_bookmark_name(tail)?;
                tail = new_tail;
                m_bookmark_name = Some(parsed_m_bookmark_name);
            }
            Ok((
                tail,
                Self {
                    m_cutscene_id: ok_or_return_missing_field_err!(m_cutscene_id),
                    m_bookmark_name: ok_or_return_missing_field_err!(m_bookmark_name),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerCutsceneEndSceneFiredEvent {
        pub m_cutscene_id: Int32,
    }
    impl GameSTriggerCutsceneEndSceneFiredEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cutscene_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_cutscene_id) = Int32::parse(input)?;
            tracing::debug!("m_cutscene_id: {:?}", m_cutscene_id);
            Ok((tail, m_cutscene_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerCutsceneEndSceneFiredEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_cutscene_id: Option<Int32> = None;
            if m_cutscene_id.is_none() {
                let (new_tail, parsed_m_cutscene_id) = Self::parse_m_cutscene_id(tail)?;
                tail = new_tail;
                m_cutscene_id = Some(parsed_m_cutscene_id);
            }
            Ok((
                tail,
                Self {
                    m_cutscene_id: ok_or_return_missing_field_err!(m_cutscene_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerCutsceneConversationLineEvent {
        pub m_cutscene_id: Int32,
        pub m_conversation_line: Vec<u8>,
        pub m_alt_conversation_line: Vec<u8>,
    }
    impl GameSTriggerCutsceneConversationLineEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cutscene_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_cutscene_id) = Int32::parse(input)?;
            tracing::debug!("m_cutscene_id: {:?}", m_cutscene_id);
            Ok((tail, m_cutscene_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_conversation_line(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_alt_conversation_line(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerCutsceneConversationLineEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_cutscene_id: Option<Int32> = None;
            let mut m_conversation_line: Option<Vec<u8>> = None;
            let mut m_alt_conversation_line: Option<Vec<u8>> = None;
            if m_cutscene_id.is_none() {
                let (new_tail, parsed_m_cutscene_id) = Self::parse_m_cutscene_id(tail)?;
                tail = new_tail;
                m_cutscene_id = Some(parsed_m_cutscene_id);
            }
            if m_conversation_line.is_none() {
                let (new_tail, parsed_m_conversation_line) = Self::parse_m_conversation_line(tail)?;
                tail = new_tail;
                m_conversation_line = Some(parsed_m_conversation_line);
            }
            if m_alt_conversation_line.is_none() {
                let (new_tail, parsed_m_alt_conversation_line) =
                    Self::parse_m_alt_conversation_line(tail)?;
                tail = new_tail;
                m_alt_conversation_line = Some(parsed_m_alt_conversation_line);
            }
            Ok((
                tail,
                Self {
                    m_cutscene_id: ok_or_return_missing_field_err!(m_cutscene_id),
                    m_conversation_line: ok_or_return_missing_field_err!(m_conversation_line),
                    m_alt_conversation_line: ok_or_return_missing_field_err!(
                        m_alt_conversation_line
                    ),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerCutsceneConversationLineMissingEvent {
        pub m_cutscene_id: Int32,
        pub m_conversation_line: Vec<u8>,
    }
    impl GameSTriggerCutsceneConversationLineMissingEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cutscene_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_cutscene_id) = Int32::parse(input)?;
            tracing::debug!("m_cutscene_id: {:?}", m_cutscene_id);
            Ok((tail, m_cutscene_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_conversation_line(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerCutsceneConversationLineMissingEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_cutscene_id: Option<Int32> = None;
            let mut m_conversation_line: Option<Vec<u8>> = None;
            if m_cutscene_id.is_none() {
                let (new_tail, parsed_m_cutscene_id) = Self::parse_m_cutscene_id(tail)?;
                tail = new_tail;
                m_cutscene_id = Some(parsed_m_cutscene_id);
            }
            if m_conversation_line.is_none() {
                let (new_tail, parsed_m_conversation_line) = Self::parse_m_conversation_line(tail)?;
                tail = new_tail;
                m_conversation_line = Some(parsed_m_conversation_line);
            }
            Ok((
                tail,
                Self {
                    m_cutscene_id: ok_or_return_missing_field_err!(m_cutscene_id),
                    m_conversation_line: ok_or_return_missing_field_err!(m_conversation_line),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSGameUserLeaveEvent {
        pub m_leave_reason: ELeaveReason,
    }
    impl GameSGameUserLeaveEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_leave_reason(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), ELeaveReason> {
            let (tail, m_leave_reason) = ELeaveReason::parse(input)?;
            tracing::debug!("m_leave_reason: {:?}", m_leave_reason);
            Ok((tail, m_leave_reason))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSGameUserLeaveEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_leave_reason: Option<ELeaveReason> = None;
            if m_leave_reason.is_none() {
                let (new_tail, parsed_m_leave_reason) = Self::parse_m_leave_reason(tail)?;
                tail = new_tail;
                m_leave_reason = Some(parsed_m_leave_reason);
            }
            Ok((
                tail,
                Self {
                    m_leave_reason: ok_or_return_missing_field_err!(m_leave_reason),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSGameUserJoinEvent {
        pub m_observe: EObserve,
        pub m_name: CUserName,
        pub m_toon_handle: Option<CToonHandle>,
        pub m_clan_tag: Option<CClanTag>,
        pub m_clan_logo: Option<GameCCacheHandle>,
        pub m_hijack: bool,
        pub m_hijack_clone_game_user_id: Option<TUserId>,
    }
    impl GameSGameUserJoinEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_observe(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), EObserve> {
            let (tail, m_observe) = EObserve::parse(input)?;
            tracing::debug!("m_observe: {:?}", m_observe);
            Ok((tail, m_observe))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CUserName> {
            let (tail, m_name) = CUserName::parse(input)?;
            tracing::debug!("m_name: {:?}", str::from_utf8(&m_name.value));
            Ok((tail, m_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_toon_handle(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<CToonHandle>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_toon_handle) = if is_provided {
                let (tail, res) = CToonHandle::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_toon_handle: {:?}", m_toon_handle);
            Ok((tail, m_toon_handle))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_clan_tag(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<CClanTag>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_clan_tag) = if is_provided {
                let (tail, res) = CClanTag::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_clan_tag: {:?}", m_clan_tag);
            Ok((tail, m_clan_tag))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_clan_logo(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameCCacheHandle>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_clan_logo) = if is_provided {
                let (tail, res) = GameCCacheHandle::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_clan_logo: {:?}", m_clan_logo);
            Ok((tail, m_clan_logo))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_hijack(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_hijack) = parse_bool(input)?;
            tracing::debug!("m_hijack: {:?}", m_hijack);
            Ok((tail, m_hijack))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_hijack_clone_game_user_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<TUserId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_hijack_clone_game_user_id) = if is_provided {
                let (tail, res) = TUserId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!(
                "m_hijack_clone_game_user_id: {:?}",
                m_hijack_clone_game_user_id
            );
            Ok((tail, m_hijack_clone_game_user_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSGameUserJoinEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_observe: Option<EObserve> = None;
            let mut m_name: Option<CUserName> = None;
            let mut m_toon_handle: Option<Option<CToonHandle>> = Some(None);
            let mut m_clan_tag: Option<Option<CClanTag>> = Some(None);
            let mut m_clan_logo: Option<Option<GameCCacheHandle>> = Some(None);
            let mut m_hijack: Option<bool> = None;
            let mut m_hijack_clone_game_user_id: Option<Option<TUserId>> = Some(None);
            if m_observe.is_none() {
                let (new_tail, parsed_m_observe) = Self::parse_m_observe(tail)?;
                tail = new_tail;
                m_observe = Some(parsed_m_observe);
            }
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            if let Some(None) = m_toon_handle {
                let (new_tail, parsed_m_toon_handle) = Self::parse_m_toon_handle(tail)?;
                tail = new_tail;
                m_toon_handle = Some(parsed_m_toon_handle);
            }
            if let Some(None) = m_clan_tag {
                let (new_tail, parsed_m_clan_tag) = Self::parse_m_clan_tag(tail)?;
                tail = new_tail;
                m_clan_tag = Some(parsed_m_clan_tag);
            }
            if let Some(None) = m_clan_logo {
                let (new_tail, parsed_m_clan_logo) = Self::parse_m_clan_logo(tail)?;
                tail = new_tail;
                m_clan_logo = Some(parsed_m_clan_logo);
            }
            if m_hijack.is_none() {
                let (new_tail, parsed_m_hijack) = Self::parse_m_hijack(tail)?;
                tail = new_tail;
                m_hijack = Some(parsed_m_hijack);
            }
            if let Some(None) = m_hijack_clone_game_user_id {
                let (new_tail, parsed_m_hijack_clone_game_user_id) =
                    Self::parse_m_hijack_clone_game_user_id(tail)?;
                tail = new_tail;
                m_hijack_clone_game_user_id = Some(parsed_m_hijack_clone_game_user_id);
            }
            Ok((
                tail,
                Self {
                    m_observe: ok_or_return_missing_field_err!(m_observe),
                    m_name: ok_or_return_missing_field_err!(m_name),
                    m_toon_handle: ok_or_return_missing_field_err!(m_toon_handle),
                    m_clan_tag: ok_or_return_missing_field_err!(m_clan_tag),
                    m_clan_logo: ok_or_return_missing_field_err!(m_clan_logo),
                    m_hijack: ok_or_return_missing_field_err!(m_hijack),
                    m_hijack_clone_game_user_id: ok_or_return_missing_field_err!(
                        m_hijack_clone_game_user_id
                    ),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameECommandManagerState {
        EFireDone,
        EFireOnce,
        EFireMany,
    }
    impl GameECommandManagerState {
        #[tracing::instrument(name="87702::GameECommandManagerState::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 3
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EFireDone for value '0'");
                    Ok((tail, Self::EFireDone))
                }
                1 => {
                    tracing::debug!("Variant EFireOnce for value '1'");
                    Ok((tail, Self::EFireOnce))
                }
                2 => {
                    tracing::debug!("Variant EFireMany for value '2'");
                    Ok((tail, Self::EFireMany))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCommandManagerStateEvent {
        pub m_state: GameECommandManagerState,
        pub m_sequence: Option<i64>,
    }
    impl GameSCommandManagerStateEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_state(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameECommandManagerState> {
            let (tail, m_state) = GameECommandManagerState::parse(input)?;
            tracing::debug!("m_state: {:?}", m_state);
            Ok((tail, m_state))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sequence(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<i64>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_sequence) = if is_provided {
                let (tail, res) = parse_packed_int(input, 1, 32usize)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_sequence: {:?}", m_sequence);
            Ok((tail, m_sequence))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCommandManagerStateEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_state: Option<GameECommandManagerState> = None;
            let mut m_sequence: Option<Option<i64>> = Some(None);
            if m_state.is_none() {
                let (new_tail, parsed_m_state) = Self::parse_m_state(tail)?;
                tail = new_tail;
                m_state = Some(parsed_m_state);
            }
            if let Some(None) = m_sequence {
                let (new_tail, parsed_m_sequence) = Self::parse_m_sequence(tail)?;
                tail = new_tail;
                m_sequence = Some(parsed_m_sequence);
            }
            Ok((
                tail,
                Self {
                    m_state: ok_or_return_missing_field_err!(m_state),
                    m_sequence: ok_or_return_missing_field_err!(m_sequence),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCmdUpdateTargetPointEvent {
        pub m_target: GameSMapCoord3D,
    }
    impl GameSCmdUpdateTargetPointEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSMapCoord3D> {
            let (tail, m_target) = GameSMapCoord3D::parse(input)?;
            tracing::debug!("m_target: {:?}", m_target);
            Ok((tail, m_target))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCmdUpdateTargetPointEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_target: Option<GameSMapCoord3D> = None;
            if m_target.is_none() {
                let (new_tail, parsed_m_target) = Self::parse_m_target(tail)?;
                tail = new_tail;
                m_target = Some(parsed_m_target);
            }
            Ok((
                tail,
                Self {
                    m_target: ok_or_return_missing_field_err!(m_target),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCmdUpdateTargetUnitEvent {
        pub m_target: GameSCmdDataTargetUnit,
    }
    impl GameSCmdUpdateTargetUnitEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSCmdDataTargetUnit> {
            let (tail, m_target) = GameSCmdDataTargetUnit::parse(input)?;
            tracing::debug!("m_target: {:?}", m_target);
            Ok((tail, m_target))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCmdUpdateTargetUnitEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_target: Option<GameSCmdDataTargetUnit> = None;
            if m_target.is_none() {
                let (new_tail, parsed_m_target) = Self::parse_m_target(tail)?;
                tail = new_tail;
                m_target = Some(parsed_m_target);
            }
            Ok((
                tail,
                Self {
                    m_target: ok_or_return_missing_field_err!(m_target),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSCatalogModifyEvent {
        pub m_catalog: Uint8,
        pub m_entry: Uint16,
        pub m_field: Vec<u8>,
        pub m_value: Vec<u8>,
    }
    impl GameSCatalogModifyEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_catalog(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_catalog) = Uint8::parse(input)?;
            tracing::debug!("m_catalog: {:?}", m_catalog);
            Ok((tail, m_catalog))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_entry(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint16> {
            let (tail, m_entry) = Uint16::parse(input)?;
            tracing::debug!("m_entry: {:?}", m_entry);
            Ok((tail, m_entry))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_field(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 9)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_value(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 9)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSCatalogModifyEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_catalog: Option<Uint8> = None;
            let mut m_entry: Option<Uint16> = None;
            let mut m_field: Option<Vec<u8>> = None;
            let mut m_value: Option<Vec<u8>> = None;
            if m_catalog.is_none() {
                let (new_tail, parsed_m_catalog) = Self::parse_m_catalog(tail)?;
                tail = new_tail;
                m_catalog = Some(parsed_m_catalog);
            }
            if m_entry.is_none() {
                let (new_tail, parsed_m_entry) = Self::parse_m_entry(tail)?;
                tail = new_tail;
                m_entry = Some(parsed_m_entry);
            }
            if m_field.is_none() {
                let (new_tail, parsed_m_field) = Self::parse_m_field(tail)?;
                tail = new_tail;
                m_field = Some(parsed_m_field);
            }
            if m_value.is_none() {
                let (new_tail, parsed_m_value) = Self::parse_m_value(tail)?;
                tail = new_tail;
                m_value = Some(parsed_m_value);
            }
            Ok((
                tail,
                Self {
                    m_catalog: ok_or_return_missing_field_err!(m_catalog),
                    m_entry: ok_or_return_missing_field_err!(m_entry),
                    m_field: ok_or_return_missing_field_err!(m_field),
                    m_value: ok_or_return_missing_field_err!(m_value),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSHeroTalentTreeSelectedEvent {
        pub m_index: Uint32,
    }
    impl GameSHeroTalentTreeSelectedEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_index(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_index) = Uint32::parse(input)?;
            tracing::debug!("m_index: {:?}", m_index);
            Ok((tail, m_index))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSHeroTalentTreeSelectedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_index: Option<Uint32> = None;
            if m_index.is_none() {
                let (new_tail, parsed_m_index) = Self::parse_m_index(tail)?;
                tail = new_tail;
                m_index = Some(parsed_m_index);
            }
            Ok((
                tail,
                Self {
                    m_index: ok_or_return_missing_field_err!(m_index),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSTriggerProfilerLoggingFinishedEvent {}
    impl GameSTriggerProfilerLoggingFinishedEvent {
        #[tracing::instrument(name="87702::bit_packed::GameSTriggerProfilerLoggingFinishedEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSHeroTalentTreeSelectionPanelToggledEvent {
        pub m_shown: bool,
    }
    impl GameSHeroTalentTreeSelectionPanelToggledEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_shown(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_shown) = parse_bool(input)?;
            tracing::debug!("m_shown: {:?}", m_shown);
            Ok((tail, m_shown))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSHeroTalentTreeSelectionPanelToggledEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_shown: Option<bool> = None;
            if m_shown.is_none() {
                let (new_tail, parsed_m_shown) = Self::parse_m_shown(tail)?;
                tail = new_tail;
                m_shown = Some(parsed_m_shown);
            }
            Ok((
                tail,
                Self {
                    m_shown: ok_or_return_missing_field_err!(m_shown),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSMuteChatEvent {
        pub m_target_user_id: TUserId,
        pub m_muted: bool,
    }
    impl GameSMuteChatEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_target_user_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TUserId> {
            let (tail, m_target_user_id) = TUserId::parse(input)?;
            tracing::debug!("m_target_user_id: {:?}", m_target_user_id);
            Ok((tail, m_target_user_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_muted(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_muted) = parse_bool(input)?;
            tracing::debug!("m_muted: {:?}", m_muted);
            Ok((tail, m_muted))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSMuteChatEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_target_user_id: Option<TUserId> = None;
            let mut m_muted: Option<bool> = None;
            if m_target_user_id.is_none() {
                let (new_tail, parsed_m_target_user_id) = Self::parse_m_target_user_id(tail)?;
                tail = new_tail;
                m_target_user_id = Some(parsed_m_target_user_id);
            }
            if m_muted.is_none() {
                let (new_tail, parsed_m_muted) = Self::parse_m_muted(tail)?;
                tail = new_tail;
                m_muted = Some(parsed_m_muted);
            }
            Ok((
                tail,
                Self {
                    m_target_user_id: ok_or_return_missing_field_err!(m_target_user_id),
                    m_muted: ok_or_return_missing_field_err!(m_muted),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSConvertToReplaySessionEvent {
        pub m_replay_jump_game_loop: Option<Int32>,
    }
    impl GameSConvertToReplaySessionEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_replay_jump_game_loop(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Int32>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_replay_jump_game_loop) = if is_provided {
                let (tail, res) = Int32::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_replay_jump_game_loop: {:?}", m_replay_jump_game_loop);
            Ok((tail, m_replay_jump_game_loop))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSConvertToReplaySessionEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_replay_jump_game_loop: Option<Option<Int32>> = Some(None);
            if let Some(None) = m_replay_jump_game_loop {
                let (new_tail, parsed_m_replay_jump_game_loop) =
                    Self::parse_m_replay_jump_game_loop(tail)?;
                tail = new_tail;
                m_replay_jump_game_loop = Some(parsed_m_replay_jump_game_loop);
            }
            Ok((
                tail,
                Self {
                    m_replay_jump_game_loop: ok_or_return_missing_field_err!(
                        m_replay_jump_game_loop
                    ),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSetSyncLoadingTimeEvent {
        pub m_sync_time: Uint32,
    }
    impl GameSSetSyncLoadingTimeEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sync_time(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_sync_time) = Uint32::parse(input)?;
            tracing::debug!("m_sync_time: {:?}", m_sync_time);
            Ok((tail, m_sync_time))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSetSyncLoadingTimeEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sync_time: Option<Uint32> = None;
            if m_sync_time.is_none() {
                let (new_tail, parsed_m_sync_time) = Self::parse_m_sync_time(tail)?;
                tail = new_tail;
                m_sync_time = Some(parsed_m_sync_time);
            }
            Ok((
                tail,
                Self {
                    m_sync_time: ok_or_return_missing_field_err!(m_sync_time),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSetSyncPlayingTimeEvent {
        pub m_sync_time: Uint32,
    }
    impl GameSSetSyncPlayingTimeEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sync_time(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_sync_time) = Uint32::parse(input)?;
            tracing::debug!("m_sync_time: {:?}", m_sync_time);
            Ok((tail, m_sync_time))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSetSyncPlayingTimeEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sync_time: Option<Uint32> = None;
            if m_sync_time.is_none() {
                let (new_tail, parsed_m_sync_time) = Self::parse_m_sync_time(tail)?;
                tail = new_tail;
                m_sync_time = Some(parsed_m_sync_time);
            }
            Ok((
                tail,
                Self {
                    m_sync_time: ok_or_return_missing_field_err!(m_sync_time),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPeerSetSyncLoadingTimeEvent {
        pub m_sync_time: Uint32,
    }
    impl GameSPeerSetSyncLoadingTimeEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sync_time(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_sync_time) = Uint32::parse(input)?;
            tracing::debug!("m_sync_time: {:?}", m_sync_time);
            Ok((tail, m_sync_time))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSPeerSetSyncLoadingTimeEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sync_time: Option<Uint32> = None;
            if m_sync_time.is_none() {
                let (new_tail, parsed_m_sync_time) = Self::parse_m_sync_time(tail)?;
                tail = new_tail;
                m_sync_time = Some(parsed_m_sync_time);
            }
            Ok((
                tail,
                Self {
                    m_sync_time: ok_or_return_missing_field_err!(m_sync_time),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPeerSetSyncPlayingTimeEvent {
        pub m_sync_time: Uint32,
    }
    impl GameSPeerSetSyncPlayingTimeEvent {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sync_time(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_sync_time) = Uint32::parse(input)?;
            tracing::debug!("m_sync_time: {:?}", m_sync_time);
            Ok((tail, m_sync_time))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSPeerSetSyncPlayingTimeEvent::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sync_time: Option<Uint32> = None;
            if m_sync_time.is_none() {
                let (new_tail, parsed_m_sync_time) = Self::parse_m_sync_time(tail)?;
                tail = new_tail;
                m_sync_time = Some(parsed_m_sync_time);
            }
            Ok((
                tail,
                Self {
                    m_sync_time: ok_or_return_missing_field_err!(m_sync_time),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEGameSpeed {
        ESlower,
        ESlow,
        ENormal,
        EFast,
        EFaster,
    }
    impl GameEGameSpeed {
        #[tracing::instrument(name="87702::GameEGameSpeed::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 5
            let num_bits: usize = 3;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ESlower for value '0'");
                    Ok((tail, Self::ESlower))
                }
                1 => {
                    tracing::debug!("Variant ESlow for value '1'");
                    Ok((tail, Self::ESlow))
                }
                2 => {
                    tracing::debug!("Variant ENormal for value '2'");
                    Ok((tail, Self::ENormal))
                }
                3 => {
                    tracing::debug!("Variant EFast for value '3'");
                    Ok((tail, Self::EFast))
                }
                4 => {
                    tracing::debug!("Variant EFaster for value '4'");
                    Ok((tail, Self::EFaster))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEPhase {
        EInitializing,
        ELobby,
        EClosed,
        ELoading,
        EPlaying,
        EGameover,
    }
    impl GameEPhase {
        #[tracing::instrument(name="87702::GameEPhase::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 6
            let num_bits: usize = 3;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EInitializing for value '0'");
                    Ok((tail, Self::EInitializing))
                }
                1 => {
                    tracing::debug!("Variant ELobby for value '1'");
                    Ok((tail, Self::ELobby))
                }
                2 => {
                    tracing::debug!("Variant EClosed for value '2'");
                    Ok((tail, Self::EClosed))
                }
                3 => {
                    tracing::debug!("Variant ELoading for value '3'");
                    Ok((tail, Self::ELoading))
                }
                4 => {
                    tracing::debug!("Variant EPlaying for value '4'");
                    Ok((tail, Self::EPlaying))
                }
                5 => {
                    tracing::debug!("Variant EGameover for value '5'");
                    Ok((tail, Self::EGameover))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEConversationSkip {
        ESkipOneLine,
        ESkipAllLines,
    }
    impl GameEConversationSkip {
        #[tracing::instrument(name="87702::GameEConversationSkip::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 2
            let num_bits: usize = 1;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ESkipOneLine for value '0'");
                    Ok((tail, Self::ESkipOneLine))
                }
                1 => {
                    tracing::debug!("Variant ESkipAllLines for value '1'");
                    Ok((tail, Self::ESkipAllLines))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCCheatString {
        pub value: Vec<u8>,
    }
    impl GameCCheatString {
        #[tracing::instrument(name="87702::GameCCheatString::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 11;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCTriggerChatMessageString {
        pub value: Vec<u8>,
    }
    impl GameCTriggerChatMessageString {
        #[tracing::instrument(name="87702::GameCTriggerChatMessageString::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 11;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTAchievementLink {
        pub value: Uint16,
    }
    impl GameTAchievementLink {
        #[tracing::instrument(name="87702::GameTAchievementLink::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTAchievementTermLink {
        pub value: Uint16,
    }
    impl GameTAchievementTermLink {
        #[tracing::instrument(name="87702::GameTAchievementTermLink::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTButtonLink {
        pub value: Uint16,
    }
    impl GameTButtonLink {
        #[tracing::instrument(name="87702::GameTButtonLink::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTUnitLink {
        pub value: Uint16,
    }
    impl GameTUnitLink {
        #[tracing::instrument(name="87702::GameTUnitLink::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTUnitTag {
        pub value: Uint32,
    }
    impl GameTUnitTag {
        #[tracing::instrument(name="87702::GameTUnitTag::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint32::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTTriggerThreadTag {
        pub value: Uint32,
    }
    impl GameTTriggerThreadTag {
        #[tracing::instrument(name="87702::GameTTriggerThreadTag::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint32::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTTriggerSoundTag {
        pub value: Uint32,
    }
    impl GameTTriggerSoundTag {
        #[tracing::instrument(name="87702::GameTTriggerSoundTag::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint32::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTAbilLink {
        pub value: Uint16,
    }
    impl GameTAbilLink {
        #[tracing::instrument(name="87702::GameTAbilLink::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTFixedBits {
        pub value: Int32,
    }
    impl GameTFixedBits {
        #[tracing::instrument(name="87702::GameTFixedBits::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Int32::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTFixedMiniBitsUnsigned {
        pub value: Uint16,
    }
    impl GameTFixedMiniBitsUnsigned {
        #[tracing::instrument(name="87702::GameTFixedMiniBitsUnsigned::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTFixedMiniBitsSigned {
        pub value: Int16,
    }
    impl GameTFixedMiniBitsSigned {
        #[tracing::instrument(name="87702::GameTFixedMiniBitsSigned::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Int16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTPlayerLogoIndex {
        pub value: Uint32,
    }
    impl GameTPlayerLogoIndex {
        #[tracing::instrument(name="87702::GameTPlayerLogoIndex::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint32::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTFixedInt {
        pub value: i64,
    }
    impl GameTFixedInt {
        #[tracing::instrument(name="87702::GameTFixedInt::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = -524288;
            let num_bits: usize = 20;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTFixedUInt {
        pub value: i64,
    }
    impl GameTFixedUInt {
        #[tracing::instrument(name="87702::GameTFixedUInt::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 19;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTMapCoordFixedBits {
        pub value: i64,
    }
    impl GameTMapCoordFixedBits {
        #[tracing::instrument(name="87702::GameTMapCoordFixedBits::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 20;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTuiCoordX {
        pub value: i64,
    }
    impl GameTuiCoordX {
        #[tracing::instrument(name="87702::GameTuiCoordX::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 11;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTuiCoordY {
        pub value: i64,
    }
    impl GameTuiCoordY {
        #[tracing::instrument(name="87702::GameTuiCoordY::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 11;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTHeroLink {
        pub value: Uint16,
    }
    impl GameTHeroLink {
        #[tracing::instrument(name="87702::GameTHeroLink::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPoint {
        pub x: GameTFixedBits,
        pub y: GameTFixedBits,
    }
    impl GameSPoint {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_x(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTFixedBits> {
            let (tail, x) = GameTFixedBits::parse(input)?;
            tracing::debug!("x: {:?}", x);
            Ok((tail, x))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_y(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTFixedBits> {
            let (tail, y) = GameTFixedBits::parse(input)?;
            tracing::debug!("y: {:?}", y);
            Ok((tail, y))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSPoint::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut x: Option<GameTFixedBits> = None;
            let mut y: Option<GameTFixedBits> = None;
            if x.is_none() {
                let (new_tail, parsed_x) = Self::parse_x(tail)?;
                tail = new_tail;
                x = Some(parsed_x);
            }
            if y.is_none() {
                let (new_tail, parsed_y) = Self::parse_y(tail)?;
                tail = new_tail;
                y = Some(parsed_y);
            }
            Ok((
                tail,
                Self {
                    x: ok_or_return_missing_field_err!(x),
                    y: ok_or_return_missing_field_err!(y),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPoint3 {
        pub x: GameTFixedBits,
        pub y: GameTFixedBits,
        pub z: GameTFixedBits,
    }
    impl GameSPoint3 {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_x(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTFixedBits> {
            let (tail, x) = GameTFixedBits::parse(input)?;
            tracing::debug!("x: {:?}", x);
            Ok((tail, x))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_y(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTFixedBits> {
            let (tail, y) = GameTFixedBits::parse(input)?;
            tracing::debug!("y: {:?}", y);
            Ok((tail, y))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_z(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTFixedBits> {
            let (tail, z) = GameTFixedBits::parse(input)?;
            tracing::debug!("z: {:?}", z);
            Ok((tail, z))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSPoint3::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut x: Option<GameTFixedBits> = None;
            let mut y: Option<GameTFixedBits> = None;
            let mut z: Option<GameTFixedBits> = None;
            if x.is_none() {
                let (new_tail, parsed_x) = Self::parse_x(tail)?;
                tail = new_tail;
                x = Some(parsed_x);
            }
            if y.is_none() {
                let (new_tail, parsed_y) = Self::parse_y(tail)?;
                tail = new_tail;
                y = Some(parsed_y);
            }
            if z.is_none() {
                let (new_tail, parsed_z) = Self::parse_z(tail)?;
                tail = new_tail;
                z = Some(parsed_z);
            }
            Ok((
                tail,
                Self {
                    x: ok_or_return_missing_field_err!(x),
                    y: ok_or_return_missing_field_err!(y),
                    z: ok_or_return_missing_field_err!(z),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPointMini {
        pub x: GameTFixedMiniBitsUnsigned,
        pub y: GameTFixedMiniBitsUnsigned,
    }
    impl GameSPointMini {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_x(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTFixedMiniBitsUnsigned> {
            let (tail, x) = GameTFixedMiniBitsUnsigned::parse(input)?;
            tracing::debug!("x: {:?}", x);
            Ok((tail, x))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_y(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTFixedMiniBitsUnsigned> {
            let (tail, y) = GameTFixedMiniBitsUnsigned::parse(input)?;
            tracing::debug!("y: {:?}", y);
            Ok((tail, y))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSPointMini::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut x: Option<GameTFixedMiniBitsUnsigned> = None;
            let mut y: Option<GameTFixedMiniBitsUnsigned> = None;
            if x.is_none() {
                let (new_tail, parsed_x) = Self::parse_x(tail)?;
                tail = new_tail;
                x = Some(parsed_x);
            }
            if y.is_none() {
                let (new_tail, parsed_y) = Self::parse_y(tail)?;
                tail = new_tail;
                y = Some(parsed_y);
            }
            Ok((
                tail,
                Self {
                    x: ok_or_return_missing_field_err!(x),
                    y: ok_or_return_missing_field_err!(y),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSMapCoord {
        pub x: GameTMapCoordFixedBits,
        pub y: GameTMapCoordFixedBits,
    }
    impl GameSMapCoord {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_x(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTMapCoordFixedBits> {
            let (tail, x) = GameTMapCoordFixedBits::parse(input)?;
            tracing::debug!("x: {:?}", x);
            Ok((tail, x))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_y(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTMapCoordFixedBits> {
            let (tail, y) = GameTMapCoordFixedBits::parse(input)?;
            tracing::debug!("y: {:?}", y);
            Ok((tail, y))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSMapCoord::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut x: Option<GameTMapCoordFixedBits> = None;
            let mut y: Option<GameTMapCoordFixedBits> = None;
            if x.is_none() {
                let (new_tail, parsed_x) = Self::parse_x(tail)?;
                tail = new_tail;
                x = Some(parsed_x);
            }
            if y.is_none() {
                let (new_tail, parsed_y) = Self::parse_y(tail)?;
                tail = new_tail;
                y = Some(parsed_y);
            }
            Ok((
                tail,
                Self {
                    x: ok_or_return_missing_field_err!(x),
                    y: ok_or_return_missing_field_err!(y),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSMapCoord3D {
        pub x: GameTMapCoordFixedBits,
        pub y: GameTMapCoordFixedBits,
        pub z: GameTFixedBits,
    }
    impl GameSMapCoord3D {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_x(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTMapCoordFixedBits> {
            let (tail, x) = GameTMapCoordFixedBits::parse(input)?;
            tracing::debug!("x: {:?}", x);
            Ok((tail, x))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_y(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTMapCoordFixedBits> {
            let (tail, y) = GameTMapCoordFixedBits::parse(input)?;
            tracing::debug!("y: {:?}", y);
            Ok((tail, y))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_z(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTFixedBits> {
            let (tail, z) = GameTFixedBits::parse(input)?;
            tracing::debug!("z: {:?}", z);
            Ok((tail, z))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSMapCoord3D::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut x: Option<GameTMapCoordFixedBits> = None;
            let mut y: Option<GameTMapCoordFixedBits> = None;
            let mut z: Option<GameTFixedBits> = None;
            if x.is_none() {
                let (new_tail, parsed_x) = Self::parse_x(tail)?;
                tail = new_tail;
                x = Some(parsed_x);
            }
            if y.is_none() {
                let (new_tail, parsed_y) = Self::parse_y(tail)?;
                tail = new_tail;
                y = Some(parsed_y);
            }
            if z.is_none() {
                let (new_tail, parsed_z) = Self::parse_z(tail)?;
                tail = new_tail;
                z = Some(parsed_z);
            }
            Ok((
                tail,
                Self {
                    x: ok_or_return_missing_field_err!(x),
                    y: ok_or_return_missing_field_err!(y),
                    z: ok_or_return_missing_field_err!(z),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSuiCoord {
        pub x: GameTuiCoordX,
        pub y: GameTuiCoordY,
    }
    impl GameSuiCoord {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_x(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTuiCoordX> {
            let (tail, x) = GameTuiCoordX::parse(input)?;
            tracing::debug!("x: {:?}", x);
            Ok((tail, x))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_y(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameTuiCoordY> {
            let (tail, y) = GameTuiCoordY::parse(input)?;
            tracing::debug!("y: {:?}", y);
            Ok((tail, y))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSuiCoord::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut x: Option<GameTuiCoordX> = None;
            let mut y: Option<GameTuiCoordY> = None;
            if x.is_none() {
                let (new_tail, parsed_x) = Self::parse_x(tail)?;
                tail = new_tail;
                x = Some(parsed_x);
            }
            if y.is_none() {
                let (new_tail, parsed_y) = Self::parse_y(tail)?;
                tail = new_tail;
                y = Some(parsed_y);
            }
            Ok((
                tail,
                Self {
                    x: ok_or_return_missing_field_err!(x),
                    y: ok_or_return_missing_field_err!(y),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTHandicap {
        pub value: Uint32,
    }
    impl GameTHandicap {
        #[tracing::instrument(name="87702::GameTHandicap::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint32::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTDifficulty {
        pub value: i64,
    }
    impl GameTDifficulty {
        #[tracing::instrument(name="87702::GameTDifficulty::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 6;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCAllowedDifficulty {
        pub value: i64, // Initially Vec<u8> but these are 8 bits and fits in i64 and easy to
                        // compare with blizzard's python implementation
    }
    impl GameCAllowedDifficulty {
        #[tracing::instrument(name="87702::GameCAllowedDifficulty::BitArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let bitarray_length_bits: usize = 6;
            let (tail, bitarray_length) = take_n_bits_into_i64(input, bitarray_length_bits)?;
            tracing::debug!("Reading bitarray length: {bitarray_length}");
            let (tail, value) = take_n_bits_into_i64(tail, bitarray_length as usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTaiBuild {
        pub value: i64,
    }
    impl GameTaiBuild {
        #[tracing::instrument(name="87702::GameTaiBuild::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 8;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCAllowedAiBuild {
        pub value: Vec<u8>,
    }
    impl GameCAllowedAiBuild {
        #[tracing::instrument(name="87702::GameCAllowedAiBuild::BitArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let bitarray_length_bits: usize = 8;
            let (tail, bitarray_length) = take_n_bits_into_i64(input, bitarray_length_bits)?;
            tracing::debug!("Reading bitarray length: {bitarray_length}");
            let (tail, value) = take_bit_array(tail, bitarray_length as usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSyncSoundLength {
        pub m_sound_hash: Vec<Uint32>,
        pub m_length: Vec<Uint32>,
    }
    impl GameSSyncSoundLength {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sound_hash(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<Uint32>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = Uint32::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_length(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<Uint32>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = Uint32::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSyncSoundLength::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sound_hash: Option<Vec<Uint32>> = None;
            let mut m_length: Option<Vec<Uint32>> = None;
            if m_sound_hash.is_none() {
                let (new_tail, parsed_m_sound_hash) = Self::parse_m_sound_hash(tail)?;
                tail = new_tail;
                m_sound_hash = Some(parsed_m_sound_hash);
            }
            if m_length.is_none() {
                let (new_tail, parsed_m_length) = Self::parse_m_length(tail)?;
                tail = new_tail;
                m_length = Some(parsed_m_length);
            }
            Ok((
                tail,
                Self {
                    m_sound_hash: ok_or_return_missing_field_err!(m_sound_hash),
                    m_length: ok_or_return_missing_field_err!(m_length),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSThumbnail {
        pub m_file: Vec<u8>,
    }
    impl GameSThumbnail {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_file(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 10)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSThumbnail::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_file: Option<Vec<u8>> = None;
            if m_file.is_none() {
                let (new_tail, parsed_m_file) = Self::parse_m_file(tail)?;
                tail = new_tail;
                m_file = Some(parsed_m_file);
            }
            Ok((
                tail,
                Self {
                    m_file: ok_or_return_missing_field_err!(m_file),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSColor {
        pub m_a: Uint8,
        pub m_r: Uint8,
        pub m_g: Uint8,
        pub m_b: Uint8,
    }
    impl GameSColor {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_a(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_a) = Uint8::parse(input)?;
            tracing::debug!("m_a: {:?}", m_a);
            Ok((tail, m_a))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_r(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_r) = Uint8::parse(input)?;
            tracing::debug!("m_r: {:?}", m_r);
            Ok((tail, m_r))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_g(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_g) = Uint8::parse(input)?;
            tracing::debug!("m_g: {:?}", m_g);
            Ok((tail, m_g))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_b(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_b) = Uint8::parse(input)?;
            tracing::debug!("m_b: {:?}", m_b);
            Ok((tail, m_b))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSColor::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_a: Option<Uint8> = None;
            let mut m_r: Option<Uint8> = None;
            let mut m_g: Option<Uint8> = None;
            let mut m_b: Option<Uint8> = None;
            if m_a.is_none() {
                let (new_tail, parsed_m_a) = Self::parse_m_a(tail)?;
                tail = new_tail;
                m_a = Some(parsed_m_a);
            }
            if m_r.is_none() {
                let (new_tail, parsed_m_r) = Self::parse_m_r(tail)?;
                tail = new_tail;
                m_r = Some(parsed_m_r);
            }
            if m_g.is_none() {
                let (new_tail, parsed_m_g) = Self::parse_m_g(tail)?;
                tail = new_tail;
                m_g = Some(parsed_m_g);
            }
            if m_b.is_none() {
                let (new_tail, parsed_m_b) = Self::parse_m_b(tail)?;
                tail = new_tail;
                m_b = Some(parsed_m_b);
            }
            Ok((
                tail,
                Self {
                    m_a: ok_or_return_missing_field_err!(m_a),
                    m_r: ok_or_return_missing_field_err!(m_r),
                    m_g: ok_or_return_missing_field_err!(m_g),
                    m_b: ok_or_return_missing_field_err!(m_b),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEResultDetails {
        EUndecided,
        EWin,
        ELoss,
        ETie,
    }
    impl GameEResultDetails {
        #[tracing::instrument(name="87702::GameEResultDetails::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 4
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EUndecided for value '0'");
                    Ok((tail, Self::EUndecided))
                }
                1 => {
                    tracing::debug!("Variant EWin for value '1'");
                    Ok((tail, Self::EWin))
                }
                2 => {
                    tracing::debug!("Variant ELoss for value '2'");
                    Ok((tail, Self::ELoss))
                }
                3 => {
                    tracing::debug!("Variant ETie for value '3'");
                    Ok((tail, Self::ETie))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSToonNameDetails {
        pub m_region: Uint8,
        pub m_program_id: Vec<u8>,
        pub m_realm: Uint32,
        pub m_name: Vec<u8>,
        pub m_id: Uint64,
    }
    impl GameSToonNameDetails {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_region(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_region) = Uint8::parse(input)?;
            tracing::debug!("m_region: {:?}", m_region);
            Ok((tail, m_region))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_program_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (tail, m_program_id) = take_fourcc(input)?;
            tracing::debug!("m_program_id: {:?}", m_program_id);
            Ok((tail, m_program_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_realm(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_realm) = Uint32::parse(input)?;
            tracing::debug!("m_realm: {:?}", m_realm);
            Ok((tail, m_realm))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 5)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint64> {
            let (tail, m_id) = Uint64::parse(input)?;
            tracing::debug!("m_id: {:?}", m_id);
            Ok((tail, m_id))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSToonNameDetails::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_region: Option<Uint8> = None;
            let mut m_program_id: Option<Vec<u8>> = None;
            let mut m_realm: Option<Uint32> = None;
            let mut m_name: Option<Vec<u8>> = None;
            let mut m_id: Option<Uint64> = None;
            if m_region.is_none() {
                let (new_tail, parsed_m_region) = Self::parse_m_region(tail)?;
                tail = new_tail;
                m_region = Some(parsed_m_region);
            }
            if m_program_id.is_none() {
                let (new_tail, parsed_m_program_id) = Self::parse_m_program_id(tail)?;
                tail = new_tail;
                m_program_id = Some(parsed_m_program_id);
            }
            if m_realm.is_none() {
                let (new_tail, parsed_m_realm) = Self::parse_m_realm(tail)?;
                tail = new_tail;
                m_realm = Some(parsed_m_realm);
            }
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            if m_id.is_none() {
                let (new_tail, parsed_m_id) = Self::parse_m_id(tail)?;
                tail = new_tail;
                m_id = Some(parsed_m_id);
            }
            Ok((
                tail,
                Self {
                    m_region: ok_or_return_missing_field_err!(m_region),
                    m_program_id: ok_or_return_missing_field_err!(m_program_id),
                    m_realm: ok_or_return_missing_field_err!(m_realm),
                    m_name: m_name.unwrap_or(b""[..].to_vec()),
                    m_id: ok_or_return_missing_field_err!(m_id),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPlayerDetails {
        pub m_name: CUserName,
        pub m_toon: GameSToonNameDetails,
        pub m_race: Vec<u8>,
        pub m_color: GameSColor,
        pub m_control: GameTControlId,
        pub m_team_id: GameTTeamId,
        pub m_handicap: GameTHandicap,
        pub m_observe: EObserve,
        pub m_result: GameEResultDetails,
        pub m_working_set_slot_id: Option<Uint8>,
        pub m_hero: Vec<u8>,
    }
    impl GameSPlayerDetails {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_name(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CUserName> {
            let (tail, m_name) = CUserName::parse(input)?;
            tracing::debug!("m_name: {:?}", str::from_utf8(&m_name.value));
            Ok((tail, m_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_toon(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSToonNameDetails> {
            let (tail, m_toon) = GameSToonNameDetails::parse(input)?;
            tracing::debug!("m_toon: {:?}", m_toon);
            Ok((tail, m_toon))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_race(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_color(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameSColor> {
            let (tail, m_color) = GameSColor::parse(input)?;
            tracing::debug!("m_color: {:?}", m_color);
            Ok((tail, m_color))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_control(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTControlId> {
            let (tail, m_control) = GameTControlId::parse(input)?;
            tracing::debug!("m_control: {:?}", m_control);
            Ok((tail, m_control))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_team_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTTeamId> {
            let (tail, m_team_id) = GameTTeamId::parse(input)?;
            tracing::debug!("m_team_id: {:?}", m_team_id);
            Ok((tail, m_team_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_handicap(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTHandicap> {
            let (tail, m_handicap) = GameTHandicap::parse(input)?;
            tracing::debug!("m_handicap: {:?}", m_handicap);
            Ok((tail, m_handicap))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_observe(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), EObserve> {
            let (tail, m_observe) = EObserve::parse(input)?;
            tracing::debug!("m_observe: {:?}", m_observe);
            Ok((tail, m_observe))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_result(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEResultDetails> {
            let (tail, m_result) = GameEResultDetails::parse(input)?;
            tracing::debug!("m_result: {:?}", m_result);
            Ok((tail, m_result))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_working_set_slot_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Uint8>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_working_set_slot_id) = if is_provided {
                let (tail, res) = Uint8::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_working_set_slot_id: {:?}", m_working_set_slot_id);
            Ok((tail, m_working_set_slot_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_hero(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSPlayerDetails::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_name: Option<CUserName> = None;
            let mut m_toon: Option<GameSToonNameDetails> = None;
            let mut m_race: Option<Vec<u8>> = None;
            let mut m_color: Option<GameSColor> = None;
            let mut m_control: Option<GameTControlId> = None;
            let mut m_team_id: Option<GameTTeamId> = None;
            let mut m_handicap: Option<GameTHandicap> = None;
            let mut m_observe: Option<EObserve> = None;
            let mut m_result: Option<GameEResultDetails> = None;
            let mut m_working_set_slot_id: Option<Option<Uint8>> = Some(None);
            let mut m_hero: Option<Vec<u8>> = None;
            if m_name.is_none() {
                let (new_tail, parsed_m_name) = Self::parse_m_name(tail)?;
                tail = new_tail;
                m_name = Some(parsed_m_name);
            }
            if m_toon.is_none() {
                let (new_tail, parsed_m_toon) = Self::parse_m_toon(tail)?;
                tail = new_tail;
                m_toon = Some(parsed_m_toon);
            }
            if m_race.is_none() {
                let (new_tail, parsed_m_race) = Self::parse_m_race(tail)?;
                tail = new_tail;
                m_race = Some(parsed_m_race);
            }
            if m_color.is_none() {
                let (new_tail, parsed_m_color) = Self::parse_m_color(tail)?;
                tail = new_tail;
                m_color = Some(parsed_m_color);
            }
            if m_control.is_none() {
                let (new_tail, parsed_m_control) = Self::parse_m_control(tail)?;
                tail = new_tail;
                m_control = Some(parsed_m_control);
            }
            if m_team_id.is_none() {
                let (new_tail, parsed_m_team_id) = Self::parse_m_team_id(tail)?;
                tail = new_tail;
                m_team_id = Some(parsed_m_team_id);
            }
            if m_handicap.is_none() {
                let (new_tail, parsed_m_handicap) = Self::parse_m_handicap(tail)?;
                tail = new_tail;
                m_handicap = Some(parsed_m_handicap);
            }
            if m_observe.is_none() {
                let (new_tail, parsed_m_observe) = Self::parse_m_observe(tail)?;
                tail = new_tail;
                m_observe = Some(parsed_m_observe);
            }
            if m_result.is_none() {
                let (new_tail, parsed_m_result) = Self::parse_m_result(tail)?;
                tail = new_tail;
                m_result = Some(parsed_m_result);
            }
            if let Some(None) = m_working_set_slot_id {
                let (new_tail, parsed_m_working_set_slot_id) =
                    Self::parse_m_working_set_slot_id(tail)?;
                tail = new_tail;
                m_working_set_slot_id = Some(parsed_m_working_set_slot_id);
            }
            if m_hero.is_none() {
                let (new_tail, parsed_m_hero) = Self::parse_m_hero(tail)?;
                tail = new_tail;
                m_hero = Some(parsed_m_hero);
            }
            Ok((
                tail,
                Self {
                    m_name: m_name.unwrap_or(CUserName {
                        value: b""[..].to_vec(),
                    }),
                    m_toon: ok_or_return_missing_field_err!(m_toon),
                    m_race: ok_or_return_missing_field_err!(m_race),
                    m_color: ok_or_return_missing_field_err!(m_color),
                    m_control: ok_or_return_missing_field_err!(m_control),
                    m_team_id: ok_or_return_missing_field_err!(m_team_id),
                    m_handicap: ok_or_return_missing_field_err!(m_handicap),
                    m_observe: ok_or_return_missing_field_err!(m_observe),
                    m_result: ok_or_return_missing_field_err!(m_result),
                    m_working_set_slot_id: ok_or_return_missing_field_err!(m_working_set_slot_id),
                    m_hero: ok_or_return_missing_field_err!(m_hero),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCPlayerDetailsArray {
        pub value: Vec<GameSPlayerDetails>,
    }
    impl GameCPlayerDetailsArray {
        #[tracing::instrument(name="87702::GameCPlayerDetailsArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 5;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(GameSPlayerDetails::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameSPlayerDetails>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameSPlayerDetails::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCModPaths {
        pub value: Vec<CFilePath>,
    }
    impl GameCModPaths {
        #[tracing::instrument(name="87702::GameCModPaths::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 6;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(CFilePath::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<CFilePath>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = CFilePath::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSDetails {
        pub m_player_list: Option<GameCPlayerDetailsArray>,
        pub m_title: Vec<u8>,
        pub m_difficulty: Vec<u8>,
        pub m_thumbnail: GameSThumbnail,
        pub m_is_blizzard_map: bool,
        pub m_time_utc: Int64,
        pub m_time_local_offset: Int64,
        pub m_restart_as_transition_map: Option<bool>,
        pub m_disable_recover_game: bool,
        pub m_description: Vec<u8>,
        pub m_image_file_path: CFilePath,
        pub m_campaign_index: Uint8,
        pub m_map_file_name: CFilePath,
        pub m_cache_handles: Option<GameCCacheHandles>,
        pub m_mini_save: bool,
        pub m_game_speed: GameEGameSpeed,
        pub m_default_difficulty: GameTDifficulty,
        pub m_mod_paths: Option<GameCModPaths>,
    }
    impl GameSDetails {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_player_list(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameCPlayerDetailsArray>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_player_list) = if is_provided {
                let (tail, res) = GameCPlayerDetailsArray::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_player_list: {:?}", m_player_list);
            Ok((tail, m_player_list))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_title(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 9)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_difficulty(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 7)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_thumbnail(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSThumbnail> {
            let (tail, m_thumbnail) = GameSThumbnail::parse(input)?;
            tracing::debug!("m_thumbnail: {:?}", m_thumbnail);
            Ok((tail, m_thumbnail))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_is_blizzard_map(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_is_blizzard_map) = parse_bool(input)?;
            tracing::debug!("m_is_blizzard_map: {:?}", m_is_blizzard_map);
            Ok((tail, m_is_blizzard_map))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_time_utc(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int64> {
            let (tail, m_time_utc) = Int64::parse(input)?;
            tracing::debug!("m_time_utc: {:?}", m_time_utc);
            Ok((tail, m_time_utc))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_time_local_offset(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Int64> {
            let (tail, m_time_local_offset) = Int64::parse(input)?;
            tracing::debug!("m_time_local_offset: {:?}", m_time_local_offset);
            Ok((tail, m_time_local_offset))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_restart_as_transition_map(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<bool>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_restart_as_transition_map) = if is_provided {
                let (tail, res) = parse_bool(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!(
                "m_restart_as_transition_map: {:?}",
                m_restart_as_transition_map
            );
            Ok((tail, m_restart_as_transition_map))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_disable_recover_game(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_disable_recover_game) = parse_bool(input)?;
            tracing::debug!("m_disable_recover_game: {:?}", m_disable_recover_game);
            Ok((tail, m_disable_recover_game))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_description(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<u8>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 11)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = take_unaligned_byte(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_image_file_path(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CFilePath> {
            let (tail, m_image_file_path) = CFilePath::parse(input)?;
            tracing::debug!("m_image_file_path: {:?}", m_image_file_path);
            Ok((tail, m_image_file_path))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_campaign_index(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_campaign_index) = Uint8::parse(input)?;
            tracing::debug!("m_campaign_index: {:?}", m_campaign_index);
            Ok((tail, m_campaign_index))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_map_file_name(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CFilePath> {
            let (tail, m_map_file_name) = CFilePath::parse(input)?;
            tracing::debug!(
                "m_map_file_name: {:?}",
                str::from_utf8(&m_map_file_name.value)
            );
            Ok((tail, m_map_file_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cache_handles(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameCCacheHandles>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_cache_handles) = if is_provided {
                let (tail, res) = GameCCacheHandles::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_cache_handles: {:?}", m_cache_handles);
            Ok((tail, m_cache_handles))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_mini_save(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_mini_save) = parse_bool(input)?;
            tracing::debug!("m_mini_save: {:?}", m_mini_save);
            Ok((tail, m_mini_save))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_speed(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEGameSpeed> {
            let (tail, m_game_speed) = GameEGameSpeed::parse(input)?;
            tracing::debug!("m_game_speed: {:?}", m_game_speed);
            Ok((tail, m_game_speed))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_default_difficulty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTDifficulty> {
            let (tail, m_default_difficulty) = GameTDifficulty::parse(input)?;
            tracing::debug!("m_default_difficulty: {:?}", m_default_difficulty);
            Ok((tail, m_default_difficulty))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_mod_paths(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<GameCModPaths>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_mod_paths) = if is_provided {
                let (tail, res) = GameCModPaths::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_mod_paths: {:?}", m_mod_paths);
            Ok((tail, m_mod_paths))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSDetails::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_player_list: Option<Option<GameCPlayerDetailsArray>> = Some(None);
            let mut m_title: Option<Vec<u8>> = None;
            let mut m_difficulty: Option<Vec<u8>> = None;
            let mut m_thumbnail: Option<GameSThumbnail> = None;
            let mut m_is_blizzard_map: Option<bool> = None;
            let mut m_time_utc: Option<Int64> = None;
            let mut m_time_local_offset: Option<Int64> = None;
            let mut m_restart_as_transition_map: Option<Option<bool>> = Some(None);
            let mut m_disable_recover_game: Option<bool> = None;
            let mut m_description: Option<Vec<u8>> = None;
            let mut m_image_file_path: Option<CFilePath> = None;
            let mut m_campaign_index: Option<Uint8> = None;
            let mut m_map_file_name: Option<CFilePath> = None;
            let mut m_cache_handles: Option<Option<GameCCacheHandles>> = Some(None);
            let mut m_mini_save: Option<bool> = None;
            let mut m_game_speed: Option<GameEGameSpeed> = None;
            let mut m_default_difficulty: Option<GameTDifficulty> = None;
            let mut m_mod_paths: Option<Option<GameCModPaths>> = Some(None);
            if let Some(None) = m_player_list {
                let (new_tail, parsed_m_player_list) = Self::parse_m_player_list(tail)?;
                tail = new_tail;
                m_player_list = Some(parsed_m_player_list);
            }
            if m_title.is_none() {
                let (new_tail, parsed_m_title) = Self::parse_m_title(tail)?;
                tail = new_tail;
                m_title = Some(parsed_m_title);
            }
            if m_difficulty.is_none() {
                let (new_tail, parsed_m_difficulty) = Self::parse_m_difficulty(tail)?;
                tail = new_tail;
                m_difficulty = Some(parsed_m_difficulty);
            }
            if m_thumbnail.is_none() {
                let (new_tail, parsed_m_thumbnail) = Self::parse_m_thumbnail(tail)?;
                tail = new_tail;
                m_thumbnail = Some(parsed_m_thumbnail);
            }
            if m_is_blizzard_map.is_none() {
                let (new_tail, parsed_m_is_blizzard_map) = Self::parse_m_is_blizzard_map(tail)?;
                tail = new_tail;
                m_is_blizzard_map = Some(parsed_m_is_blizzard_map);
            }
            if m_time_utc.is_none() {
                let (new_tail, parsed_m_time_utc) = Self::parse_m_time_utc(tail)?;
                tail = new_tail;
                m_time_utc = Some(parsed_m_time_utc);
            }
            if m_time_local_offset.is_none() {
                let (new_tail, parsed_m_time_local_offset) = Self::parse_m_time_local_offset(tail)?;
                tail = new_tail;
                m_time_local_offset = Some(parsed_m_time_local_offset);
            }
            if let Some(None) = m_restart_as_transition_map {
                let (new_tail, parsed_m_restart_as_transition_map) =
                    Self::parse_m_restart_as_transition_map(tail)?;
                tail = new_tail;
                m_restart_as_transition_map = Some(parsed_m_restart_as_transition_map);
            }
            if m_disable_recover_game.is_none() {
                let (new_tail, parsed_m_disable_recover_game) =
                    Self::parse_m_disable_recover_game(tail)?;
                tail = new_tail;
                m_disable_recover_game = Some(parsed_m_disable_recover_game);
            }
            if m_description.is_none() {
                let (new_tail, parsed_m_description) = Self::parse_m_description(tail)?;
                tail = new_tail;
                m_description = Some(parsed_m_description);
            }
            if m_image_file_path.is_none() {
                let (new_tail, parsed_m_image_file_path) = Self::parse_m_image_file_path(tail)?;
                tail = new_tail;
                m_image_file_path = Some(parsed_m_image_file_path);
            }
            if m_campaign_index.is_none() {
                let (new_tail, parsed_m_campaign_index) = Self::parse_m_campaign_index(tail)?;
                tail = new_tail;
                m_campaign_index = Some(parsed_m_campaign_index);
            }
            if m_map_file_name.is_none() {
                let (new_tail, parsed_m_map_file_name) = Self::parse_m_map_file_name(tail)?;
                tail = new_tail;
                m_map_file_name = Some(parsed_m_map_file_name);
            }
            if let Some(None) = m_cache_handles {
                let (new_tail, parsed_m_cache_handles) = Self::parse_m_cache_handles(tail)?;
                tail = new_tail;
                m_cache_handles = Some(parsed_m_cache_handles);
            }
            if m_mini_save.is_none() {
                let (new_tail, parsed_m_mini_save) = Self::parse_m_mini_save(tail)?;
                tail = new_tail;
                m_mini_save = Some(parsed_m_mini_save);
            }
            if m_game_speed.is_none() {
                let (new_tail, parsed_m_game_speed) = Self::parse_m_game_speed(tail)?;
                tail = new_tail;
                m_game_speed = Some(parsed_m_game_speed);
            }
            if m_default_difficulty.is_none() {
                let (new_tail, parsed_m_default_difficulty) =
                    Self::parse_m_default_difficulty(tail)?;
                tail = new_tail;
                m_default_difficulty = Some(parsed_m_default_difficulty);
            }
            if let Some(None) = m_mod_paths {
                let (new_tail, parsed_m_mod_paths) = Self::parse_m_mod_paths(tail)?;
                tail = new_tail;
                m_mod_paths = Some(parsed_m_mod_paths);
            }
            Ok((
                tail,
                Self {
                    m_player_list: ok_or_return_missing_field_err!(m_player_list),
                    m_title: ok_or_return_missing_field_err!(m_title),
                    m_difficulty: ok_or_return_missing_field_err!(m_difficulty),
                    m_thumbnail: ok_or_return_missing_field_err!(m_thumbnail),
                    m_is_blizzard_map: ok_or_return_missing_field_err!(m_is_blizzard_map),
                    m_time_utc: ok_or_return_missing_field_err!(m_time_utc),
                    m_time_local_offset: ok_or_return_missing_field_err!(m_time_local_offset),
                    m_restart_as_transition_map: ok_or_return_missing_field_err!(
                        m_restart_as_transition_map
                    ),
                    m_disable_recover_game: ok_or_return_missing_field_err!(m_disable_recover_game),
                    m_description: ok_or_return_missing_field_err!(m_description),
                    m_image_file_path: ok_or_return_missing_field_err!(m_image_file_path),
                    m_campaign_index: ok_or_return_missing_field_err!(m_campaign_index),
                    m_map_file_name: ok_or_return_missing_field_err!(m_map_file_name),
                    m_cache_handles: ok_or_return_missing_field_err!(m_cache_handles),
                    m_mini_save: ok_or_return_missing_field_err!(m_mini_save),
                    m_game_speed: ok_or_return_missing_field_err!(m_game_speed),
                    m_default_difficulty: ok_or_return_missing_field_err!(m_default_difficulty),
                    m_mod_paths: ok_or_return_missing_field_err!(m_mod_paths),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEOptionFog {
        EDefault,
        EHideTerrain,
        EMapExplored,
        EAlwaysVisible,
    }
    impl GameEOptionFog {
        #[tracing::instrument(name="87702::GameEOptionFog::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 4
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EDefault for value '0'");
                    Ok((tail, Self::EDefault))
                }
                1 => {
                    tracing::debug!("Variant EHideTerrain for value '1'");
                    Ok((tail, Self::EHideTerrain))
                }
                2 => {
                    tracing::debug!("Variant EMapExplored for value '2'");
                    Ok((tail, Self::EMapExplored))
                }
                3 => {
                    tracing::debug!("Variant EAlwaysVisible for value '3'");
                    Ok((tail, Self::EAlwaysVisible))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEOptionObservers {
        ENone,
        EOnJoin,
        EOnJoinAndDefeat,
        ERefereesOnJoin,
    }
    impl GameEOptionObservers {
        #[tracing::instrument(name="87702::GameEOptionObservers::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 4
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ENone for value '0'");
                    Ok((tail, Self::ENone))
                }
                1 => {
                    tracing::debug!("Variant EOnJoin for value '1'");
                    Ok((tail, Self::EOnJoin))
                }
                2 => {
                    tracing::debug!("Variant EOnJoinAndDefeat for value '2'");
                    Ok((tail, Self::EOnJoinAndDefeat))
                }
                3 => {
                    tracing::debug!("Variant ERefereesOnJoin for value '3'");
                    Ok((tail, Self::ERefereesOnJoin))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEOptionUserDifficulty {
        ENone,
        EGlobal,
        EIndividual,
    }
    impl GameEOptionUserDifficulty {
        #[tracing::instrument(name="87702::GameEOptionUserDifficulty::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 3
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ENone for value '0'");
                    Ok((tail, Self::ENone))
                }
                1 => {
                    tracing::debug!("Variant EGlobal for value '1'");
                    Ok((tail, Self::EGlobal))
                }
                2 => {
                    tracing::debug!("Variant EIndividual for value '2'");
                    Ok((tail, Self::EIndividual))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEGameLaunch {
        EInvalid,
        EMap,
        EReplay,
        ESave,
        ETransition,
        EServerReplay,
    }
    impl GameEGameLaunch {
        #[tracing::instrument(name="87702::GameEGameLaunch::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 6
            let num_bits: usize = 3;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EInvalid for value '0'");
                    Ok((tail, Self::EInvalid))
                }
                1 => {
                    tracing::debug!("Variant EMap for value '1'");
                    Ok((tail, Self::EMap))
                }
                2 => {
                    tracing::debug!("Variant EReplay for value '2'");
                    Ok((tail, Self::EReplay))
                }
                3 => {
                    tracing::debug!("Variant ESave for value '3'");
                    Ok((tail, Self::ESave))
                }
                4 => {
                    tracing::debug!("Variant ETransition for value '4'");
                    Ok((tail, Self::ETransition))
                }
                5 => {
                    tracing::debug!("Variant EServerReplay for value '5'");
                    Ok((tail, Self::EServerReplay))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameEClientDebugFlags {
        pub value: i64,
    }
    impl GameEClientDebugFlags {
        #[tracing::instrument(name="87702::GameEClientDebugFlags::IntType::Parse::PowExpr", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 64;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSGameOptions {
        pub m_lock_teams: bool,
        pub m_teams_together: bool,
        pub m_advanced_shared_control: bool,
        pub m_random_races: bool,
        pub m_battle_net: bool,
        pub m_amm: bool,
        pub m_competitive: bool,
        pub m_practice: bool,
        pub m_cooperative: bool,
        pub m_no_victory_or_defeat: bool,
        pub m_hero_duplicates_allowed: bool,
        pub m_fog: GameEOptionFog,
        pub m_observers: GameEOptionObservers,
        pub m_user_difficulty: GameEOptionUserDifficulty,
        pub m_client_debug_flags: GameEClientDebugFlags,
        pub m_build_coach_enabled: bool,
    }
    impl GameSGameOptions {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_lock_teams(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_lock_teams) = parse_bool(input)?;
            tracing::debug!("m_lock_teams: {:?}", m_lock_teams);
            Ok((tail, m_lock_teams))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_teams_together(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_teams_together) = parse_bool(input)?;
            tracing::debug!("m_teams_together: {:?}", m_teams_together);
            Ok((tail, m_teams_together))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_advanced_shared_control(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_advanced_shared_control) = parse_bool(input)?;
            tracing::debug!("m_advanced_shared_control: {:?}", m_advanced_shared_control);
            Ok((tail, m_advanced_shared_control))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_random_races(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_random_races) = parse_bool(input)?;
            tracing::debug!("m_random_races: {:?}", m_random_races);
            Ok((tail, m_random_races))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_battle_net(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_battle_net) = parse_bool(input)?;
            tracing::debug!("m_battle_net: {:?}", m_battle_net);
            Ok((tail, m_battle_net))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_amm(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_amm) = parse_bool(input)?;
            tracing::debug!("m_amm: {:?}", m_amm);
            Ok((tail, m_amm))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_competitive(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_competitive) = parse_bool(input)?;
            tracing::debug!("m_competitive: {:?}", m_competitive);
            Ok((tail, m_competitive))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_practice(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_practice) = parse_bool(input)?;
            tracing::debug!("m_practice: {:?}", m_practice);
            Ok((tail, m_practice))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cooperative(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_cooperative) = parse_bool(input)?;
            tracing::debug!("m_cooperative: {:?}", m_cooperative);
            Ok((tail, m_cooperative))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_no_victory_or_defeat(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_no_victory_or_defeat) = parse_bool(input)?;
            tracing::debug!("m_no_victory_or_defeat: {:?}", m_no_victory_or_defeat);
            Ok((tail, m_no_victory_or_defeat))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_hero_duplicates_allowed(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_hero_duplicates_allowed) = parse_bool(input)?;
            tracing::debug!("m_hero_duplicates_allowed: {:?}", m_hero_duplicates_allowed);
            Ok((tail, m_hero_duplicates_allowed))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_fog(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameEOptionFog> {
            let (tail, m_fog) = GameEOptionFog::parse(input)?;
            tracing::debug!("m_fog: {:?}", m_fog);
            Ok((tail, m_fog))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_observers(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEOptionObservers> {
            let (tail, m_observers) = GameEOptionObservers::parse(input)?;
            tracing::debug!("m_observers: {:?}", m_observers);
            Ok((tail, m_observers))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_user_difficulty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEOptionUserDifficulty> {
            let (tail, m_user_difficulty) = GameEOptionUserDifficulty::parse(input)?;
            tracing::debug!("m_user_difficulty: {:?}", m_user_difficulty);
            Ok((tail, m_user_difficulty))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_client_debug_flags(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEClientDebugFlags> {
            let (tail, m_client_debug_flags) = GameEClientDebugFlags::parse(input)?;
            tracing::debug!("m_client_debug_flags: {:?}", m_client_debug_flags);
            Ok((tail, m_client_debug_flags))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_build_coach_enabled(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_build_coach_enabled) = parse_bool(input)?;
            tracing::debug!("m_build_coach_enabled: {:?}", m_build_coach_enabled);
            Ok((tail, m_build_coach_enabled))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSGameOptions::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_lock_teams: Option<bool> = None;
            let mut m_teams_together: Option<bool> = None;
            let mut m_advanced_shared_control: Option<bool> = None;
            let mut m_random_races: Option<bool> = None;
            let mut m_battle_net: Option<bool> = None;
            let mut m_amm: Option<bool> = None;
            let mut m_competitive: Option<bool> = None;
            let mut m_practice: Option<bool> = None;
            let mut m_cooperative: Option<bool> = None;
            let mut m_no_victory_or_defeat: Option<bool> = None;
            let mut m_hero_duplicates_allowed: Option<bool> = None;
            let mut m_fog: Option<GameEOptionFog> = None;
            let mut m_observers: Option<GameEOptionObservers> = None;
            let mut m_user_difficulty: Option<GameEOptionUserDifficulty> = None;
            let mut m_client_debug_flags: Option<GameEClientDebugFlags> = None;
            let mut m_build_coach_enabled: Option<bool> = None;
            if m_lock_teams.is_none() {
                let (new_tail, parsed_m_lock_teams) = Self::parse_m_lock_teams(tail)?;
                tail = new_tail;
                m_lock_teams = Some(parsed_m_lock_teams);
            }
            if m_teams_together.is_none() {
                let (new_tail, parsed_m_teams_together) = Self::parse_m_teams_together(tail)?;
                tail = new_tail;
                m_teams_together = Some(parsed_m_teams_together);
            }
            if m_advanced_shared_control.is_none() {
                let (new_tail, parsed_m_advanced_shared_control) =
                    Self::parse_m_advanced_shared_control(tail)?;
                tail = new_tail;
                m_advanced_shared_control = Some(parsed_m_advanced_shared_control);
            }
            if m_random_races.is_none() {
                let (new_tail, parsed_m_random_races) = Self::parse_m_random_races(tail)?;
                tail = new_tail;
                m_random_races = Some(parsed_m_random_races);
            }
            if m_battle_net.is_none() {
                let (new_tail, parsed_m_battle_net) = Self::parse_m_battle_net(tail)?;
                tail = new_tail;
                m_battle_net = Some(parsed_m_battle_net);
            }
            if m_amm.is_none() {
                let (new_tail, parsed_m_amm) = Self::parse_m_amm(tail)?;
                tail = new_tail;
                m_amm = Some(parsed_m_amm);
            }
            if m_competitive.is_none() {
                let (new_tail, parsed_m_competitive) = Self::parse_m_competitive(tail)?;
                tail = new_tail;
                m_competitive = Some(parsed_m_competitive);
            }
            if m_practice.is_none() {
                let (new_tail, parsed_m_practice) = Self::parse_m_practice(tail)?;
                tail = new_tail;
                m_practice = Some(parsed_m_practice);
            }
            if m_cooperative.is_none() {
                let (new_tail, parsed_m_cooperative) = Self::parse_m_cooperative(tail)?;
                tail = new_tail;
                m_cooperative = Some(parsed_m_cooperative);
            }
            if m_no_victory_or_defeat.is_none() {
                let (new_tail, parsed_m_no_victory_or_defeat) =
                    Self::parse_m_no_victory_or_defeat(tail)?;
                tail = new_tail;
                m_no_victory_or_defeat = Some(parsed_m_no_victory_or_defeat);
            }
            if m_hero_duplicates_allowed.is_none() {
                let (new_tail, parsed_m_hero_duplicates_allowed) =
                    Self::parse_m_hero_duplicates_allowed(tail)?;
                tail = new_tail;
                m_hero_duplicates_allowed = Some(parsed_m_hero_duplicates_allowed);
            }
            if m_fog.is_none() {
                let (new_tail, parsed_m_fog) = Self::parse_m_fog(tail)?;
                tail = new_tail;
                m_fog = Some(parsed_m_fog);
            }
            if m_observers.is_none() {
                let (new_tail, parsed_m_observers) = Self::parse_m_observers(tail)?;
                tail = new_tail;
                m_observers = Some(parsed_m_observers);
            }
            if m_user_difficulty.is_none() {
                let (new_tail, parsed_m_user_difficulty) = Self::parse_m_user_difficulty(tail)?;
                tail = new_tail;
                m_user_difficulty = Some(parsed_m_user_difficulty);
            }
            if m_client_debug_flags.is_none() {
                let (new_tail, parsed_m_client_debug_flags) =
                    Self::parse_m_client_debug_flags(tail)?;
                tail = new_tail;
                m_client_debug_flags = Some(parsed_m_client_debug_flags);
            }
            if m_build_coach_enabled.is_none() {
                let (new_tail, parsed_m_build_coach_enabled) =
                    Self::parse_m_build_coach_enabled(tail)?;
                tail = new_tail;
                m_build_coach_enabled = Some(parsed_m_build_coach_enabled);
            }
            Ok((
                tail,
                Self {
                    m_lock_teams: ok_or_return_missing_field_err!(m_lock_teams),
                    m_teams_together: ok_or_return_missing_field_err!(m_teams_together),
                    m_advanced_shared_control: ok_or_return_missing_field_err!(
                        m_advanced_shared_control
                    ),
                    m_random_races: ok_or_return_missing_field_err!(m_random_races),
                    m_battle_net: ok_or_return_missing_field_err!(m_battle_net),
                    m_amm: ok_or_return_missing_field_err!(m_amm),
                    m_competitive: ok_or_return_missing_field_err!(m_competitive),
                    m_practice: ok_or_return_missing_field_err!(m_practice),
                    m_cooperative: ok_or_return_missing_field_err!(m_cooperative),
                    m_no_victory_or_defeat: ok_or_return_missing_field_err!(m_no_victory_or_defeat),
                    m_hero_duplicates_allowed: ok_or_return_missing_field_err!(
                        m_hero_duplicates_allowed
                    ),
                    m_fog: ok_or_return_missing_field_err!(m_fog),
                    m_observers: ok_or_return_missing_field_err!(m_observers),
                    m_user_difficulty: ok_or_return_missing_field_err!(m_user_difficulty),
                    m_client_debug_flags: ok_or_return_missing_field_err!(m_client_debug_flags),
                    m_build_coach_enabled: ok_or_return_missing_field_err!(m_build_coach_enabled),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEGameType {
        EMelee,
        EFreeForAll,
        EUseSettings,
        EOneOnOne,
        ETwoTeamPlay,
        EThreeTeamPlay,
        EFourTeamPlay,
    }
    impl GameEGameType {
        #[tracing::instrument(name="87702::GameEGameType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 7
            let num_bits: usize = 3;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EMelee for value '0'");
                    Ok((tail, Self::EMelee))
                }
                1 => {
                    tracing::debug!("Variant EFreeForAll for value '1'");
                    Ok((tail, Self::EFreeForAll))
                }
                2 => {
                    tracing::debug!("Variant EUseSettings for value '2'");
                    Ok((tail, Self::EUseSettings))
                }
                3 => {
                    tracing::debug!("Variant EOneOnOne for value '3'");
                    Ok((tail, Self::EOneOnOne))
                }
                4 => {
                    tracing::debug!("Variant ETwoTeamPlay for value '4'");
                    Ok((tail, Self::ETwoTeamPlay))
                }
                5 => {
                    tracing::debug!("Variant EThreeTeamPlay for value '5'");
                    Ok((tail, Self::EThreeTeamPlay))
                }
                6 => {
                    tracing::debug!("Variant EFourTeamPlay for value '6'");
                    Ok((tail, Self::EFourTeamPlay))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEControl {
        EOpen,
        EClosed,
        EUser,
        EComputer,
    }
    impl GameEControl {
        #[tracing::instrument(name="87702::GameEControl::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 4
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EOpen for value '0'");
                    Ok((tail, Self::EOpen))
                }
                1 => {
                    tracing::debug!("Variant EClosed for value '1'");
                    Ok((tail, Self::EClosed))
                }
                2 => {
                    tracing::debug!("Variant EUser for value '2'");
                    Ok((tail, Self::EUser))
                }
                3 => {
                    tracing::debug!("Variant EComputer for value '3'");
                    Ok((tail, Self::EComputer))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTControlId {
        pub value: i64,
    }
    impl GameTControlId {
        #[tracing::instrument(name="87702::GameTControlId::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 8;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTControlCount {
        pub value: i64,
    }
    impl GameTControlCount {
        #[tracing::instrument(name="87702::GameTControlCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 8;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCAllowedControls {
        pub value: Vec<u8>,
    }
    impl GameCAllowedControls {
        #[tracing::instrument(name="87702::GameCAllowedControls::BitArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let bitarray_length_bits: usize = 8;
            let (tail, bitarray_length) = take_n_bits_into_i64(input, bitarray_length_bits)?;
            tracing::debug!("Reading bitarray length: {bitarray_length}");
            let (tail, value) = take_bit_array(tail, bitarray_length as usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSlotDescription {
        pub m_allowed_colors: GameCAllowedColors,
        pub m_allowed_races: CAllowedRaces,
        pub m_allowed_difficulty: GameCAllowedDifficulty,
        pub m_allowed_controls: GameCAllowedControls,
        pub m_allowed_observe_types: CAllowedObserveTypes,
        pub m_allowed_ai_builds: GameCAllowedAiBuild,
    }
    impl GameSSlotDescription {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_allowed_colors(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCAllowedColors> {
            let (tail, m_allowed_colors) = GameCAllowedColors::parse(input)?;
            tracing::debug!("m_allowed_colors: {:?}", m_allowed_colors);
            Ok((tail, m_allowed_colors))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_allowed_races(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CAllowedRaces> {
            let (tail, m_allowed_races) = CAllowedRaces::parse(input)?;
            tracing::debug!("m_allowed_races: {:?}", m_allowed_races);
            Ok((tail, m_allowed_races))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_allowed_difficulty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCAllowedDifficulty> {
            let (tail, m_allowed_difficulty) = GameCAllowedDifficulty::parse(input)?;
            tracing::debug!("m_allowed_difficulty: {:?}", m_allowed_difficulty);
            Ok((tail, m_allowed_difficulty))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_allowed_controls(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCAllowedControls> {
            let (tail, m_allowed_controls) = GameCAllowedControls::parse(input)?;
            tracing::debug!("m_allowed_controls: {:?}", m_allowed_controls);
            Ok((tail, m_allowed_controls))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_allowed_observe_types(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CAllowedObserveTypes> {
            let (tail, m_allowed_observe_types) = CAllowedObserveTypes::parse(input)?;
            tracing::debug!("m_allowed_observe_types: {:?}", m_allowed_observe_types);
            Ok((tail, m_allowed_observe_types))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_allowed_ai_builds(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCAllowedAiBuild> {
            let (tail, m_allowed_ai_builds) = GameCAllowedAiBuild::parse(input)?;
            tracing::debug!("m_allowed_ai_builds: {:?}", m_allowed_ai_builds);
            Ok((tail, m_allowed_ai_builds))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSlotDescription::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_allowed_colors: Option<GameCAllowedColors> = None;
            let mut m_allowed_races: Option<CAllowedRaces> = None;
            let mut m_allowed_difficulty: Option<GameCAllowedDifficulty> = None;
            let mut m_allowed_controls: Option<GameCAllowedControls> = None;
            let mut m_allowed_observe_types: Option<CAllowedObserveTypes> = None;
            let mut m_allowed_ai_builds: Option<GameCAllowedAiBuild> = None;
            if m_allowed_colors.is_none() {
                let (new_tail, parsed_m_allowed_colors) = Self::parse_m_allowed_colors(tail)?;
                tail = new_tail;
                m_allowed_colors = Some(parsed_m_allowed_colors);
            }
            if m_allowed_races.is_none() {
                let (new_tail, parsed_m_allowed_races) = Self::parse_m_allowed_races(tail)?;
                tail = new_tail;
                m_allowed_races = Some(parsed_m_allowed_races);
            }
            if m_allowed_difficulty.is_none() {
                let (new_tail, parsed_m_allowed_difficulty) =
                    Self::parse_m_allowed_difficulty(tail)?;
                tail = new_tail;
                m_allowed_difficulty = Some(parsed_m_allowed_difficulty);
            }
            if m_allowed_controls.is_none() {
                let (new_tail, parsed_m_allowed_controls) = Self::parse_m_allowed_controls(tail)?;
                tail = new_tail;
                m_allowed_controls = Some(parsed_m_allowed_controls);
            }
            if m_allowed_observe_types.is_none() {
                let (new_tail, parsed_m_allowed_observe_types) =
                    Self::parse_m_allowed_observe_types(tail)?;
                tail = new_tail;
                m_allowed_observe_types = Some(parsed_m_allowed_observe_types);
            }
            if m_allowed_ai_builds.is_none() {
                let (new_tail, parsed_m_allowed_ai_builds) = Self::parse_m_allowed_ai_builds(tail)?;
                tail = new_tail;
                m_allowed_ai_builds = Some(parsed_m_allowed_ai_builds);
            }
            Ok((
                tail,
                Self {
                    m_allowed_colors: ok_or_return_missing_field_err!(m_allowed_colors),
                    m_allowed_races: ok_or_return_missing_field_err!(m_allowed_races),
                    m_allowed_difficulty: ok_or_return_missing_field_err!(m_allowed_difficulty),
                    m_allowed_controls: ok_or_return_missing_field_err!(m_allowed_controls),
                    m_allowed_observe_types: ok_or_return_missing_field_err!(
                        m_allowed_observe_types
                    ),
                    m_allowed_ai_builds: ok_or_return_missing_field_err!(m_allowed_ai_builds),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCCacheHandle {
        pub value: Vec<u8>,
    }
    impl GameCCacheHandle {
        #[tracing::instrument(name="87702::GameCCacheHandle::BlobType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, _) = byte_align(input)?;
            let num_bits: usize = 40 * 8;
            // TODO: The generator for open_bit_packed_blob_main_parse_fn
            // does not work for this case. It should literally 40 bits.
            // Not 40 maximum integer value that fits in 6 bits
            let (tail, value) = take_bit_array(tail, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCCacheHandles {
        pub value: Vec<GameCCacheHandle>,
    }
    impl GameCCacheHandles {
        #[tracing::instrument(name="87702::GameCCacheHandles::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 6;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(GameCCacheHandle::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameCCacheHandle>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameCCacheHandle::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCGameCacheName {
        pub value: Vec<u8>,
    }
    impl GameCGameCacheName {
        #[tracing::instrument(name="87702::GameCGameCacheName::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 11;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCAuthorName {
        pub value: Vec<u8>,
    }
    impl GameCAuthorName {
        #[tracing::instrument(name="87702::GameCAuthorName::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 8;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSlotDescriptions {
        pub value: Vec<GameSSlotDescription>,
    }
    impl GameSSlotDescriptions {
        #[tracing::instrument(name="87702::GameSSlotDescriptions::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 5;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(GameSSlotDescription::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameSSlotDescription>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameSSlotDescription::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSGameDescription {
        pub m_random_value: Uint32,
        pub m_game_cache_name: GameCGameCacheName,
        pub m_game_options: GameSGameOptions,
        pub m_game_speed: GameEGameSpeed,
        pub m_game_type: GameEGameType,
        pub m_max_users: TUserCount,
        pub m_max_observers: TUserCount,
        pub m_max_players: GameTPlayerCount,
        pub m_max_teams: GameTTeamCount,
        pub m_max_colors: GameTColorCount,
        pub m_max_races: TRaceCount,
        pub m_max_controls: GameTControlCount,
        pub m_map_size_x: Uint8,
        pub m_map_size_y: Uint8,
        pub m_map_file_sync_checksum: GameTSyncChecksum,
        pub m_map_file_name: CFilePath,
        pub m_map_author_name: GameCAuthorName,
        pub m_mod_file_sync_checksum: GameTSyncChecksum,
        pub m_slot_descriptions: GameSSlotDescriptions,
        pub m_default_difficulty: GameTDifficulty,
        pub m_default_ai_build: GameTaiBuild,
        pub m_cache_handles: GameCCacheHandles,
        pub m_has_extension_mod: bool,
        pub m_has_non_blizzard_extension_mod: bool,
        pub m_is_blizzard_map: bool,
        pub m_is_premade_ffa: bool,
        pub m_is_coop_mode: bool,
        pub m_is_realtime_mode: bool,
    }
    impl GameSGameDescription {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_random_value(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_random_value) = Uint32::parse(input)?;
            tracing::debug!("m_random_value: {:?}", m_random_value);
            Ok((tail, m_random_value))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_cache_name(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCGameCacheName> {
            let (tail, m_game_cache_name) = GameCGameCacheName::parse(input)?;
            tracing::debug!(
                "m_game_cache_name: {:?}",
                str::from_utf8(&m_game_cache_name.value)
            );
            Ok((tail, m_game_cache_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_options(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSGameOptions> {
            let (tail, m_game_options) = GameSGameOptions::parse(input)?;
            tracing::debug!("m_game_options: {:?}", m_game_options);
            Ok((tail, m_game_options))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_speed(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEGameSpeed> {
            let (tail, m_game_speed) = GameEGameSpeed::parse(input)?;
            tracing::debug!("m_game_speed: {:?}", m_game_speed);
            Ok((tail, m_game_speed))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_type(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEGameType> {
            let (tail, m_game_type) = GameEGameType::parse(input)?;
            tracing::debug!("m_game_type: {:?}", m_game_type);
            Ok((tail, m_game_type))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_max_users(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TUserCount> {
            let (tail, m_max_users) = TUserCount::parse(input)?;
            tracing::debug!("m_max_users: {:?}", m_max_users);
            Ok((tail, m_max_users))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_max_observers(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TUserCount> {
            let (tail, m_max_observers) = TUserCount::parse(input)?;
            tracing::debug!("m_max_observers: {:?}", m_max_observers);
            Ok((tail, m_max_observers))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_max_players(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTPlayerCount> {
            let (tail, m_max_players) = GameTPlayerCount::parse(input)?;
            tracing::debug!("m_max_players: {:?}", m_max_players);
            Ok((tail, m_max_players))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_max_teams(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTTeamCount> {
            let (tail, m_max_teams) = GameTTeamCount::parse(input)?;
            tracing::debug!("m_max_teams: {:?}", m_max_teams);
            Ok((tail, m_max_teams))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_max_colors(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTColorCount> {
            let (tail, m_max_colors) = GameTColorCount::parse(input)?;
            tracing::debug!("m_max_colors: {:?}", m_max_colors);
            Ok((tail, m_max_colors))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_max_races(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TRaceCount> {
            let (tail, m_max_races) = TRaceCount::parse(input)?;
            tracing::debug!("m_max_races: {:?}", m_max_races);
            Ok((tail, m_max_races))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_max_controls(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTControlCount> {
            let (tail, m_max_controls) = GameTControlCount::parse(input)?;
            tracing::debug!("m_max_controls: {:?}", m_max_controls);
            Ok((tail, m_max_controls))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_map_size_x(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_map_size_x) = Uint8::parse(input)?;
            tracing::debug!("m_map_size_x: {:?}", m_map_size_x);
            Ok((tail, m_map_size_x))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_map_size_y(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_map_size_y) = Uint8::parse(input)?;
            tracing::debug!("m_map_size_y: {:?}", m_map_size_y);
            Ok((tail, m_map_size_y))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_map_file_sync_checksum(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSyncChecksum> {
            let (tail, m_map_file_sync_checksum) = GameTSyncChecksum::parse(input)?;
            tracing::debug!("m_map_file_sync_checksum: {:?}", m_map_file_sync_checksum);
            Ok((tail, m_map_file_sync_checksum))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_map_file_name(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CFilePath> {
            let (tail, m_map_file_name) = CFilePath::parse(input)?;
            tracing::debug!(
                "m_map_file_name: {:?}",
                str::from_utf8(&m_map_file_name.value)
            );
            Ok((tail, m_map_file_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_map_author_name(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCAuthorName> {
            let (tail, m_map_author_name) = GameCAuthorName::parse(input)?;
            tracing::debug!(
                "m_map_author_name: {:?}",
                str::from_utf8(&m_map_author_name.value)
            );
            Ok((tail, m_map_author_name))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_mod_file_sync_checksum(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSyncChecksum> {
            let (tail, m_mod_file_sync_checksum) = GameTSyncChecksum::parse(input)?;
            tracing::debug!("m_mod_file_sync_checksum: {:?}", m_mod_file_sync_checksum);
            Ok((tail, m_mod_file_sync_checksum))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_slot_descriptions(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSSlotDescriptions> {
            let (tail, m_slot_descriptions) = GameSSlotDescriptions::parse(input)?;
            tracing::debug!("m_slot_descriptions: {:?}", m_slot_descriptions);
            Ok((tail, m_slot_descriptions))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_default_difficulty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTDifficulty> {
            let (tail, m_default_difficulty) = GameTDifficulty::parse(input)?;
            tracing::debug!("m_default_difficulty: {:?}", m_default_difficulty);
            Ok((tail, m_default_difficulty))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_default_ai_build(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTaiBuild> {
            let (tail, m_default_ai_build) = GameTaiBuild::parse(input)?;
            tracing::debug!("m_default_ai_build: {:?}", m_default_ai_build);
            Ok((tail, m_default_ai_build))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_cache_handles(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCCacheHandles> {
            let (tail, m_cache_handles) = GameCCacheHandles::parse(input)?;
            tracing::debug!("m_cache_handles: {:?}", m_cache_handles);
            Ok((tail, m_cache_handles))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_has_extension_mod(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_has_extension_mod) = parse_bool(input)?;
            tracing::debug!("m_has_extension_mod: {:?}", m_has_extension_mod);
            Ok((tail, m_has_extension_mod))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_has_non_blizzard_extension_mod(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_has_non_blizzard_extension_mod) = parse_bool(input)?;
            tracing::debug!(
                "m_has_non_blizzard_extension_mod: {:?}",
                m_has_non_blizzard_extension_mod
            );
            Ok((tail, m_has_non_blizzard_extension_mod))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_is_blizzard_map(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_is_blizzard_map) = parse_bool(input)?;
            tracing::debug!("m_is_blizzard_map: {:?}", m_is_blizzard_map);
            Ok((tail, m_is_blizzard_map))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_is_premade_ffa(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_is_premade_ffa) = parse_bool(input)?;
            tracing::debug!("m_is_premade_ffa: {:?}", m_is_premade_ffa);
            Ok((tail, m_is_premade_ffa))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_is_coop_mode(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_is_coop_mode) = parse_bool(input)?;
            tracing::debug!("m_is_coop_mode: {:?}", m_is_coop_mode);
            Ok((tail, m_is_coop_mode))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_is_realtime_mode(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_is_realtime_mode) = parse_bool(input)?;
            tracing::debug!("m_is_realtime_mode: {:?}", m_is_realtime_mode);
            Ok((tail, m_is_realtime_mode))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSGameDescription::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_random_value: Option<Uint32> = None;
            let mut m_game_cache_name: Option<GameCGameCacheName> = None;
            let mut m_game_options: Option<GameSGameOptions> = None;
            let mut m_game_speed: Option<GameEGameSpeed> = None;
            let mut m_game_type: Option<GameEGameType> = None;
            let mut m_max_users: Option<TUserCount> = None;
            let mut m_max_observers: Option<TUserCount> = None;
            let mut m_max_players: Option<GameTPlayerCount> = None;
            let mut m_max_teams: Option<GameTTeamCount> = None;
            let mut m_max_colors: Option<GameTColorCount> = None;
            let mut m_max_races: Option<TRaceCount> = None;
            let mut m_max_controls: Option<GameTControlCount> = None;
            let mut m_map_size_x: Option<Uint8> = None;
            let mut m_map_size_y: Option<Uint8> = None;
            let mut m_map_file_sync_checksum: Option<GameTSyncChecksum> = None;
            let mut m_map_file_name: Option<CFilePath> = None;
            let mut m_map_author_name: Option<GameCAuthorName> = None;
            let mut m_mod_file_sync_checksum: Option<GameTSyncChecksum> = None;
            let mut m_slot_descriptions: Option<GameSSlotDescriptions> = None;
            let mut m_default_difficulty: Option<GameTDifficulty> = None;
            let mut m_default_ai_build: Option<GameTaiBuild> = None;
            let mut m_cache_handles: Option<GameCCacheHandles> = None;
            let mut m_has_extension_mod: Option<bool> = None;
            let mut m_has_non_blizzard_extension_mod: Option<bool> = None;
            let mut m_is_blizzard_map: Option<bool> = None;
            let mut m_is_premade_ffa: Option<bool> = None;
            let mut m_is_coop_mode: Option<bool> = None;
            let mut m_is_realtime_mode: Option<bool> = None;
            if m_random_value.is_none() {
                let (new_tail, parsed_m_random_value) = Self::parse_m_random_value(tail)?;
                tail = new_tail;
                m_random_value = Some(parsed_m_random_value);
            }
            if m_game_cache_name.is_none() {
                let (new_tail, parsed_m_game_cache_name) = Self::parse_m_game_cache_name(tail)?;
                tail = new_tail;
                m_game_cache_name = Some(parsed_m_game_cache_name);
            }
            if m_game_options.is_none() {
                let (new_tail, parsed_m_game_options) = Self::parse_m_game_options(tail)?;
                tail = new_tail;
                m_game_options = Some(parsed_m_game_options);
            }
            if m_game_speed.is_none() {
                let (new_tail, parsed_m_game_speed) = Self::parse_m_game_speed(tail)?;
                tail = new_tail;
                m_game_speed = Some(parsed_m_game_speed);
            }
            if m_game_type.is_none() {
                let (new_tail, parsed_m_game_type) = Self::parse_m_game_type(tail)?;
                tail = new_tail;
                m_game_type = Some(parsed_m_game_type);
            }
            if m_max_users.is_none() {
                let (new_tail, parsed_m_max_users) = Self::parse_m_max_users(tail)?;
                tail = new_tail;
                m_max_users = Some(parsed_m_max_users);
            }
            if m_max_observers.is_none() {
                let (new_tail, parsed_m_max_observers) = Self::parse_m_max_observers(tail)?;
                tail = new_tail;
                m_max_observers = Some(parsed_m_max_observers);
            }
            if m_max_players.is_none() {
                let (new_tail, parsed_m_max_players) = Self::parse_m_max_players(tail)?;
                tail = new_tail;
                m_max_players = Some(parsed_m_max_players);
            }
            if m_max_teams.is_none() {
                let (new_tail, parsed_m_max_teams) = Self::parse_m_max_teams(tail)?;
                tail = new_tail;
                m_max_teams = Some(parsed_m_max_teams);
            }
            if m_max_colors.is_none() {
                let (new_tail, parsed_m_max_colors) = Self::parse_m_max_colors(tail)?;
                tail = new_tail;
                m_max_colors = Some(parsed_m_max_colors);
            }
            if m_max_races.is_none() {
                let (new_tail, parsed_m_max_races) = Self::parse_m_max_races(tail)?;
                tail = new_tail;
                m_max_races = Some(parsed_m_max_races);
            }
            if m_max_controls.is_none() {
                let (new_tail, parsed_m_max_controls) = Self::parse_m_max_controls(tail)?;
                tail = new_tail;
                m_max_controls = Some(parsed_m_max_controls);
            }
            if m_map_size_x.is_none() {
                let (new_tail, parsed_m_map_size_x) = Self::parse_m_map_size_x(tail)?;
                tail = new_tail;
                m_map_size_x = Some(parsed_m_map_size_x);
            }
            if m_map_size_y.is_none() {
                let (new_tail, parsed_m_map_size_y) = Self::parse_m_map_size_y(tail)?;
                tail = new_tail;
                m_map_size_y = Some(parsed_m_map_size_y);
            }
            if m_map_file_sync_checksum.is_none() {
                let (new_tail, parsed_m_map_file_sync_checksum) =
                    Self::parse_m_map_file_sync_checksum(tail)?;
                tail = new_tail;
                m_map_file_sync_checksum = Some(parsed_m_map_file_sync_checksum);
            }
            if m_map_file_name.is_none() {
                let (new_tail, parsed_m_map_file_name) = Self::parse_m_map_file_name(tail)?;
                tail = new_tail;
                m_map_file_name = Some(parsed_m_map_file_name);
            }
            if m_map_author_name.is_none() {
                let (new_tail, parsed_m_map_author_name) = Self::parse_m_map_author_name(tail)?;
                tail = new_tail;
                m_map_author_name = Some(parsed_m_map_author_name);
            }
            if m_mod_file_sync_checksum.is_none() {
                let (new_tail, parsed_m_mod_file_sync_checksum) =
                    Self::parse_m_mod_file_sync_checksum(tail)?;
                tail = new_tail;
                m_mod_file_sync_checksum = Some(parsed_m_mod_file_sync_checksum);
            }
            if m_slot_descriptions.is_none() {
                let (new_tail, parsed_m_slot_descriptions) = Self::parse_m_slot_descriptions(tail)?;
                tail = new_tail;
                m_slot_descriptions = Some(parsed_m_slot_descriptions);
            }
            if m_default_difficulty.is_none() {
                let (new_tail, parsed_m_default_difficulty) =
                    Self::parse_m_default_difficulty(tail)?;
                tail = new_tail;
                m_default_difficulty = Some(parsed_m_default_difficulty);
            }
            if m_default_ai_build.is_none() {
                let (new_tail, parsed_m_default_ai_build) = Self::parse_m_default_ai_build(tail)?;
                tail = new_tail;
                m_default_ai_build = Some(parsed_m_default_ai_build);
            }
            if m_cache_handles.is_none() {
                let (new_tail, parsed_m_cache_handles) = Self::parse_m_cache_handles(tail)?;
                tail = new_tail;
                m_cache_handles = Some(parsed_m_cache_handles);
            }
            if m_has_extension_mod.is_none() {
                let (new_tail, parsed_m_has_extension_mod) = Self::parse_m_has_extension_mod(tail)?;
                tail = new_tail;
                m_has_extension_mod = Some(parsed_m_has_extension_mod);
            }
            if m_has_non_blizzard_extension_mod.is_none() {
                let (new_tail, parsed_m_has_non_blizzard_extension_mod) =
                    Self::parse_m_has_non_blizzard_extension_mod(tail)?;
                tail = new_tail;
                m_has_non_blizzard_extension_mod = Some(parsed_m_has_non_blizzard_extension_mod);
            }
            if m_is_blizzard_map.is_none() {
                let (new_tail, parsed_m_is_blizzard_map) = Self::parse_m_is_blizzard_map(tail)?;
                tail = new_tail;
                m_is_blizzard_map = Some(parsed_m_is_blizzard_map);
            }
            if m_is_premade_ffa.is_none() {
                let (new_tail, parsed_m_is_premade_ffa) = Self::parse_m_is_premade_ffa(tail)?;
                tail = new_tail;
                m_is_premade_ffa = Some(parsed_m_is_premade_ffa);
            }
            if m_is_coop_mode.is_none() {
                let (new_tail, parsed_m_is_coop_mode) = Self::parse_m_is_coop_mode(tail)?;
                tail = new_tail;
                m_is_coop_mode = Some(parsed_m_is_coop_mode);
            }
            if m_is_realtime_mode.is_none() {
                let (new_tail, parsed_m_is_realtime_mode) = Self::parse_m_is_realtime_mode(tail)?;
                tail = new_tail;
                m_is_realtime_mode = Some(parsed_m_is_realtime_mode);
            }
            Ok((
                tail,
                Self {
                    m_random_value: ok_or_return_missing_field_err!(m_random_value),
                    m_game_cache_name: ok_or_return_missing_field_err!(m_game_cache_name),
                    m_game_options: ok_or_return_missing_field_err!(m_game_options),
                    m_game_speed: ok_or_return_missing_field_err!(m_game_speed),
                    m_game_type: ok_or_return_missing_field_err!(m_game_type),
                    m_max_users: ok_or_return_missing_field_err!(m_max_users),
                    m_max_observers: ok_or_return_missing_field_err!(m_max_observers),
                    m_max_players: ok_or_return_missing_field_err!(m_max_players),
                    m_max_teams: ok_or_return_missing_field_err!(m_max_teams),
                    m_max_colors: ok_or_return_missing_field_err!(m_max_colors),
                    m_max_races: ok_or_return_missing_field_err!(m_max_races),
                    m_max_controls: ok_or_return_missing_field_err!(m_max_controls),
                    m_map_size_x: ok_or_return_missing_field_err!(m_map_size_x),
                    m_map_size_y: ok_or_return_missing_field_err!(m_map_size_y),
                    m_map_file_sync_checksum: ok_or_return_missing_field_err!(
                        m_map_file_sync_checksum
                    ),
                    m_map_file_name: ok_or_return_missing_field_err!(m_map_file_name),
                    m_map_author_name: ok_or_return_missing_field_err!(m_map_author_name),
                    m_mod_file_sync_checksum: ok_or_return_missing_field_err!(
                        m_mod_file_sync_checksum
                    ),
                    m_slot_descriptions: ok_or_return_missing_field_err!(m_slot_descriptions),
                    m_default_difficulty: ok_or_return_missing_field_err!(m_default_difficulty),
                    m_default_ai_build: ok_or_return_missing_field_err!(m_default_ai_build),
                    m_cache_handles: ok_or_return_missing_field_err!(m_cache_handles),
                    m_has_extension_mod: ok_or_return_missing_field_err!(m_has_extension_mod),
                    m_has_non_blizzard_extension_mod: ok_or_return_missing_field_err!(
                        m_has_non_blizzard_extension_mod
                    ),
                    m_is_blizzard_map: ok_or_return_missing_field_err!(m_is_blizzard_map),
                    m_is_premade_ffa: ok_or_return_missing_field_err!(m_is_premade_ffa),
                    m_is_coop_mode: ok_or_return_missing_field_err!(m_is_coop_mode),
                    m_is_realtime_mode: ok_or_return_missing_field_err!(m_is_realtime_mode),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTLobbySlotCount {
        pub value: i64,
    }
    impl GameTLobbySlotCount {
        #[tracing::instrument(name="87702::GameTLobbySlotCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 5;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTLobbySlotId {
        pub value: i64,
    }
    impl GameTLobbySlotId {
        #[tracing::instrument(name="87702::GameTLobbySlotId::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 4;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCArtifactArray {
        pub value: Vec<CArtifactHandle>,
    }
    impl GameCArtifactArray {
        #[tracing::instrument(name="87702::GameCArtifactArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 4;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(CArtifactHandle::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<CArtifactHandle>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = CArtifactHandle::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCCommanderMasteryTalentArray {
        pub value: Vec<Uint32>,
    }
    impl GameCCommanderMasteryTalentArray {
        #[tracing::instrument(name="87702::GameCCommanderMasteryTalentArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 3;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(Uint32::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<Uint32>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = Uint32::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCRetryMutationIndexArray {
        pub value: Vec<Uint32>,
    }
    impl GameCRetryMutationIndexArray {
        #[tracing::instrument(name="87702::GameCRetryMutationIndexArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 3;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(Uint32::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<Uint32>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = Uint32::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTReward {
        pub value: Uint32,
    }
    impl GameTReward {
        #[tracing::instrument(name="87702::GameTReward::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint32::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCRewardArray {
        pub value: Vec<GameTReward>,
    }
    impl GameCRewardArray {
        #[tracing::instrument(name="87702::GameCRewardArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 17;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(GameTReward::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameTReward>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameTReward::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCRewardOverride {
        pub m_key: Uint32,
        pub m_rewards: GameCRewardArray,
    }
    impl GameCRewardOverride {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_key(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_key) = Uint32::parse(input)?;
            tracing::debug!("m_key: {:?}", m_key);
            Ok((tail, m_key))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_rewards(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCRewardArray> {
            let (tail, m_rewards) = GameCRewardArray::parse(input)?;
            tracing::debug!("m_rewards: {:?}", m_rewards);
            Ok((tail, m_rewards))
        }
        #[tracing::instrument(name="87702::bit_packed::GameCRewardOverride::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_key: Option<Uint32> = None;
            let mut m_rewards: Option<GameCRewardArray> = None;
            if m_key.is_none() {
                let (new_tail, parsed_m_key) = Self::parse_m_key(tail)?;
                tail = new_tail;
                m_key = Some(parsed_m_key);
            }
            if m_rewards.is_none() {
                let (new_tail, parsed_m_rewards) = Self::parse_m_rewards(tail)?;
                tail = new_tail;
                m_rewards = Some(parsed_m_rewards);
            }
            Ok((
                tail,
                Self {
                    m_key: ok_or_return_missing_field_err!(m_key),
                    m_rewards: ok_or_return_missing_field_err!(m_rewards),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCRewardOverrideArray {
        pub value: Vec<GameCRewardOverride>,
    }
    impl GameCRewardOverrideArray {
        #[tracing::instrument(name="87702::GameCRewardOverrideArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 17;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(GameCRewardOverride::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameCRewardOverride>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameCRewardOverride::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTLicense {
        pub value: Uint32,
    }
    impl GameTLicense {
        #[tracing::instrument(name="87702::GameTLicense::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint32::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCLicenseArray {
        pub value: Vec<GameTLicense>,
    }
    impl GameCLicenseArray {
        #[tracing::instrument(name="87702::GameCLicenseArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 16;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(GameTLicense::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameTLicense>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameTLicense::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTFlexLicenseName {
        pub value: Vec<u8>,
    }
    impl GameTFlexLicenseName {
        #[tracing::instrument(name="87702::GameTFlexLicenseName::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 8;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTFlexLicenseAttributeName {
        pub value: Vec<u8>,
    }
    impl GameTFlexLicenseAttributeName {
        #[tracing::instrument(name="87702::GameTFlexLicenseAttributeName::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 8;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTFlexLicenseAttributeValue {
        pub value: Vec<u8>,
    }
    impl GameTFlexLicenseAttributeValue {
        #[tracing::instrument(name="87702::GameTFlexLicenseAttributeValue::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 11;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSLobbySlot {
        pub m_control: GameTControlId,
        pub m_user_id: Option<TUserId>,
        pub m_team_id: GameTTeamId,
        pub m_color_pref: GameTColorPreference,
        pub m_race_pref: TRacePreference,
        pub m_difficulty: GameTDifficulty,
        pub m_ai_build: GameTaiBuild,
        pub m_handicap: GameTHandicap,
        pub m_observe: EObserve,
        pub m_logo_index: GameTPlayerLogoIndex,
        pub m_hero: CHeroHandle,
        pub m_skin: CSkinHandle,
        pub m_mount: CMountHandle,
        pub m_artifacts: GameCArtifactArray,
        pub m_working_set_slot_id: Option<Uint8>,
        pub m_rewards: GameCRewardArray,
        pub m_toon_handle: CToonHandle,
        pub m_licenses: GameCLicenseArray,
        pub m_tandem_leader_id: Option<TUserId>,
        pub m_commander: CCommanderHandle,
        pub m_commander_level: Uint32,
        pub m_has_silence_penalty: bool,
        pub m_tandem_id: Option<TUserId>,
        pub m_commander_mastery_level: Uint32,
        pub m_commander_mastery_talents: GameCCommanderMasteryTalentArray,
        pub m_trophy_id: Uint32,
        pub m_reward_overrides: GameCRewardOverrideArray,
        pub m_brutal_plus_difficulty: Uint32,
        pub m_retry_mutation_indexes: GameCRetryMutationIndexArray,
        pub m_a_c_enemy_race: Uint32,
        pub m_a_c_enemy_wave_type: Uint32,
        pub m_selected_commander_prestige: Uint32,
    }
    impl GameSLobbySlot {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_control(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTControlId> {
            let (tail, m_control) = GameTControlId::parse(input)?;
            tracing::debug!("m_control: {:?}", m_control);
            Ok((tail, m_control))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_user_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<TUserId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_user_id) = if is_provided {
                let (tail, res) = TUserId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_user_id: {:?}", m_user_id);
            Ok((tail, m_user_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_team_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTTeamId> {
            let (tail, m_team_id) = GameTTeamId::parse(input)?;
            tracing::debug!("m_team_id: {:?}", m_team_id);
            Ok((tail, m_team_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_color_pref(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTColorPreference> {
            let (tail, m_color_pref) = GameTColorPreference::parse(input)?;
            tracing::debug!("m_color_pref: {:?}", m_color_pref);
            Ok((tail, m_color_pref))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_race_pref(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TRacePreference> {
            let (tail, m_race_pref) = TRacePreference::parse(input)?;
            tracing::debug!("m_race_pref: {:?}", m_race_pref);
            Ok((tail, m_race_pref))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_difficulty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTDifficulty> {
            let (tail, m_difficulty) = GameTDifficulty::parse(input)?;
            tracing::debug!("m_difficulty: {:?}", m_difficulty);
            Ok((tail, m_difficulty))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_ai_build(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTaiBuild> {
            let (tail, m_ai_build) = GameTaiBuild::parse(input)?;
            tracing::debug!("m_ai_build: {:?}", m_ai_build);
            Ok((tail, m_ai_build))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_handicap(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTHandicap> {
            let (tail, m_handicap) = GameTHandicap::parse(input)?;
            tracing::debug!("m_handicap: {:?}", m_handicap);
            Ok((tail, m_handicap))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_observe(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), EObserve> {
            let (tail, m_observe) = EObserve::parse(input)?;
            tracing::debug!("m_observe: {:?}", m_observe);
            Ok((tail, m_observe))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_logo_index(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTPlayerLogoIndex> {
            let (tail, m_logo_index) = GameTPlayerLogoIndex::parse(input)?;
            tracing::debug!("m_logo_index: {:?}", m_logo_index);
            Ok((tail, m_logo_index))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_hero(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CHeroHandle> {
            let (tail, m_hero) = CHeroHandle::parse(input)?;
            tracing::debug!("m_hero: {:?}", m_hero);
            Ok((tail, m_hero))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_skin(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CSkinHandle> {
            let (tail, m_skin) = CSkinHandle::parse(input)?;
            tracing::debug!("m_skin: {:?}", m_skin);
            Ok((tail, m_skin))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_mount(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), CMountHandle> {
            let (tail, m_mount) = CMountHandle::parse(input)?;
            tracing::debug!("m_mount: {:?}", m_mount);
            Ok((tail, m_mount))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_artifacts(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCArtifactArray> {
            let (tail, m_artifacts) = GameCArtifactArray::parse(input)?;
            tracing::debug!("m_artifacts: {:?}", m_artifacts);
            Ok((tail, m_artifacts))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_working_set_slot_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<Uint8>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_working_set_slot_id) = if is_provided {
                let (tail, res) = Uint8::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_working_set_slot_id: {:?}", m_working_set_slot_id);
            Ok((tail, m_working_set_slot_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_rewards(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCRewardArray> {
            let (tail, m_rewards) = GameCRewardArray::parse(input)?;
            tracing::debug!("m_rewards: {:?}", m_rewards);
            Ok((tail, m_rewards))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_toon_handle(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CToonHandle> {
            let (tail, m_toon_handle) = CToonHandle::parse(input)?;
            tracing::debug!("m_toon_handle: {:?}", m_toon_handle);
            Ok((tail, m_toon_handle))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_licenses(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCLicenseArray> {
            let (tail, m_licenses) = GameCLicenseArray::parse(input)?;
            tracing::debug!("m_licenses: {:?}", m_licenses);
            Ok((tail, m_licenses))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_tandem_leader_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<TUserId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_tandem_leader_id) = if is_provided {
                let (tail, res) = TUserId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_tandem_leader_id: {:?}", m_tandem_leader_id);
            Ok((tail, m_tandem_leader_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_commander(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CCommanderHandle> {
            let (tail, m_commander) = CCommanderHandle::parse(input)?;
            tracing::debug!("m_commander: {:?}", m_commander);
            Ok((tail, m_commander))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_commander_level(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_commander_level) = Uint32::parse(input)?;
            tracing::debug!("m_commander_level: {:?}", m_commander_level);
            Ok((tail, m_commander_level))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_has_silence_penalty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_has_silence_penalty) = parse_bool(input)?;
            tracing::debug!("m_has_silence_penalty: {:?}", m_has_silence_penalty);
            Ok((tail, m_has_silence_penalty))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_tandem_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<TUserId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_tandem_id) = if is_provided {
                let (tail, res) = TUserId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_tandem_id: {:?}", m_tandem_id);
            Ok((tail, m_tandem_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_commander_mastery_level(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_commander_mastery_level) = Uint32::parse(input)?;
            tracing::debug!("m_commander_mastery_level: {:?}", m_commander_mastery_level);
            Ok((tail, m_commander_mastery_level))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_commander_mastery_talents(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCCommanderMasteryTalentArray> {
            let (tail, m_commander_mastery_talents) =
                GameCCommanderMasteryTalentArray::parse(input)?;
            tracing::debug!(
                "m_commander_mastery_talents: {:?}",
                m_commander_mastery_talents
            );
            Ok((tail, m_commander_mastery_talents))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_trophy_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_trophy_id) = Uint32::parse(input)?;
            tracing::debug!("m_trophy_id: {:?}", m_trophy_id);
            Ok((tail, m_trophy_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_reward_overrides(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCRewardOverrideArray> {
            let (tail, m_reward_overrides) = GameCRewardOverrideArray::parse(input)?;
            tracing::debug!("m_reward_overrides: {:?}", m_reward_overrides);
            Ok((tail, m_reward_overrides))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_brutal_plus_difficulty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_brutal_plus_difficulty) = Uint32::parse(input)?;
            tracing::debug!("m_brutal_plus_difficulty: {:?}", m_brutal_plus_difficulty);
            Ok((tail, m_brutal_plus_difficulty))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_retry_mutation_indexes(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCRetryMutationIndexArray> {
            let (tail, m_retry_mutation_indexes) = GameCRetryMutationIndexArray::parse(input)?;
            tracing::debug!("m_retry_mutation_indexes: {:?}", m_retry_mutation_indexes);
            Ok((tail, m_retry_mutation_indexes))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_a_c_enemy_race(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_a_c_enemy_race) = Uint32::parse(input)?;
            tracing::debug!("m_a_c_enemy_race: {:?}", m_a_c_enemy_race);
            Ok((tail, m_a_c_enemy_race))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_a_c_enemy_wave_type(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_a_c_enemy_wave_type) = Uint32::parse(input)?;
            tracing::debug!("m_a_c_enemy_wave_type: {:?}", m_a_c_enemy_wave_type);
            Ok((tail, m_a_c_enemy_wave_type))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_selected_commander_prestige(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_selected_commander_prestige) = Uint32::parse(input)?;
            tracing::debug!(
                "m_selected_commander_prestige: {:?}",
                m_selected_commander_prestige
            );
            Ok((tail, m_selected_commander_prestige))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSLobbySlot::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_control: Option<GameTControlId> = None;
            let mut m_user_id: Option<Option<TUserId>> = Some(None);
            let mut m_team_id: Option<GameTTeamId> = None;
            let mut m_color_pref: Option<GameTColorPreference> = None;
            let mut m_race_pref: Option<TRacePreference> = None;
            let mut m_difficulty: Option<GameTDifficulty> = None;
            let mut m_ai_build: Option<GameTaiBuild> = None;
            let mut m_handicap: Option<GameTHandicap> = None;
            let mut m_observe: Option<EObserve> = None;
            let mut m_logo_index: Option<GameTPlayerLogoIndex> = None;
            let mut m_hero: Option<CHeroHandle> = None;
            let mut m_skin: Option<CSkinHandle> = None;
            let mut m_mount: Option<CMountHandle> = None;
            let mut m_artifacts: Option<GameCArtifactArray> = None;
            let mut m_working_set_slot_id: Option<Option<Uint8>> = Some(None);
            let mut m_rewards: Option<GameCRewardArray> = None;
            let mut m_toon_handle: Option<CToonHandle> = None;
            let mut m_licenses: Option<GameCLicenseArray> = None;
            let mut m_tandem_leader_id: Option<Option<TUserId>> = Some(None);
            let mut m_commander: Option<CCommanderHandle> = None;
            let mut m_commander_level: Option<Uint32> = None;
            let mut m_has_silence_penalty: Option<bool> = None;
            let mut m_tandem_id: Option<Option<TUserId>> = Some(None);
            let mut m_commander_mastery_level: Option<Uint32> = None;
            let mut m_commander_mastery_talents: Option<GameCCommanderMasteryTalentArray> = None;
            let mut m_trophy_id: Option<Uint32> = None;
            let mut m_reward_overrides: Option<GameCRewardOverrideArray> = None;
            let mut m_brutal_plus_difficulty: Option<Uint32> = None;
            let mut m_retry_mutation_indexes: Option<GameCRetryMutationIndexArray> = None;
            let mut m_a_c_enemy_race: Option<Uint32> = None;
            let mut m_a_c_enemy_wave_type: Option<Uint32> = None;
            let mut m_selected_commander_prestige: Option<Uint32> = None;
            if m_control.is_none() {
                let (new_tail, parsed_m_control) = Self::parse_m_control(tail)?;
                tail = new_tail;
                m_control = Some(parsed_m_control);
            }
            if let Some(None) = m_user_id {
                let (new_tail, parsed_m_user_id) = Self::parse_m_user_id(tail)?;
                tail = new_tail;
                m_user_id = Some(parsed_m_user_id);
            }
            if m_team_id.is_none() {
                let (new_tail, parsed_m_team_id) = Self::parse_m_team_id(tail)?;
                tail = new_tail;
                m_team_id = Some(parsed_m_team_id);
            }
            if m_color_pref.is_none() {
                let (new_tail, parsed_m_color_pref) = Self::parse_m_color_pref(tail)?;
                tail = new_tail;
                m_color_pref = Some(parsed_m_color_pref);
            }
            if m_race_pref.is_none() {
                let (new_tail, parsed_m_race_pref) = Self::parse_m_race_pref(tail)?;
                tail = new_tail;
                m_race_pref = Some(parsed_m_race_pref);
            }
            if m_difficulty.is_none() {
                let (new_tail, parsed_m_difficulty) = Self::parse_m_difficulty(tail)?;
                tail = new_tail;
                m_difficulty = Some(parsed_m_difficulty);
            }
            if m_ai_build.is_none() {
                let (new_tail, parsed_m_ai_build) = Self::parse_m_ai_build(tail)?;
                tail = new_tail;
                m_ai_build = Some(parsed_m_ai_build);
            }
            if m_handicap.is_none() {
                let (new_tail, parsed_m_handicap) = Self::parse_m_handicap(tail)?;
                tail = new_tail;
                m_handicap = Some(parsed_m_handicap);
            }
            if m_observe.is_none() {
                let (new_tail, parsed_m_observe) = Self::parse_m_observe(tail)?;
                tail = new_tail;
                m_observe = Some(parsed_m_observe);
            }
            if m_logo_index.is_none() {
                let (new_tail, parsed_m_logo_index) = Self::parse_m_logo_index(tail)?;
                tail = new_tail;
                m_logo_index = Some(parsed_m_logo_index);
            }
            if m_hero.is_none() {
                let (new_tail, parsed_m_hero) = Self::parse_m_hero(tail)?;
                tail = new_tail;
                m_hero = Some(parsed_m_hero);
            }
            if m_skin.is_none() {
                let (new_tail, parsed_m_skin) = Self::parse_m_skin(tail)?;
                tail = new_tail;
                m_skin = Some(parsed_m_skin);
            }
            if m_mount.is_none() {
                let (new_tail, parsed_m_mount) = Self::parse_m_mount(tail)?;
                tail = new_tail;
                m_mount = Some(parsed_m_mount);
            }
            if m_artifacts.is_none() {
                let (new_tail, parsed_m_artifacts) = Self::parse_m_artifacts(tail)?;
                tail = new_tail;
                m_artifacts = Some(parsed_m_artifacts);
            }
            if let Some(None) = m_working_set_slot_id {
                let (new_tail, parsed_m_working_set_slot_id) =
                    Self::parse_m_working_set_slot_id(tail)?;
                tail = new_tail;
                m_working_set_slot_id = Some(parsed_m_working_set_slot_id);
            }
            if m_rewards.is_none() {
                let (new_tail, parsed_m_rewards) = Self::parse_m_rewards(tail)?;
                tail = new_tail;
                m_rewards = Some(parsed_m_rewards);
            }
            if m_toon_handle.is_none() {
                let (new_tail, parsed_m_toon_handle) = Self::parse_m_toon_handle(tail)?;
                tail = new_tail;
                m_toon_handle = Some(parsed_m_toon_handle);
            }
            if m_licenses.is_none() {
                let (new_tail, parsed_m_licenses) = Self::parse_m_licenses(tail)?;
                tail = new_tail;
                m_licenses = Some(parsed_m_licenses);
            }
            if let Some(None) = m_tandem_leader_id {
                let (new_tail, parsed_m_tandem_leader_id) = Self::parse_m_tandem_leader_id(tail)?;
                tail = new_tail;
                m_tandem_leader_id = Some(parsed_m_tandem_leader_id);
            }
            if m_commander.is_none() {
                let (new_tail, parsed_m_commander) = Self::parse_m_commander(tail)?;
                tail = new_tail;
                m_commander = Some(parsed_m_commander);
            }
            if m_commander_level.is_none() {
                let (new_tail, parsed_m_commander_level) = Self::parse_m_commander_level(tail)?;
                tail = new_tail;
                m_commander_level = Some(parsed_m_commander_level);
            }
            if m_has_silence_penalty.is_none() {
                let (new_tail, parsed_m_has_silence_penalty) =
                    Self::parse_m_has_silence_penalty(tail)?;
                tail = new_tail;
                m_has_silence_penalty = Some(parsed_m_has_silence_penalty);
            }
            if let Some(None) = m_tandem_id {
                let (new_tail, parsed_m_tandem_id) = Self::parse_m_tandem_id(tail)?;
                tail = new_tail;
                m_tandem_id = Some(parsed_m_tandem_id);
            }
            if m_commander_mastery_level.is_none() {
                let (new_tail, parsed_m_commander_mastery_level) =
                    Self::parse_m_commander_mastery_level(tail)?;
                tail = new_tail;
                m_commander_mastery_level = Some(parsed_m_commander_mastery_level);
            }
            if m_commander_mastery_talents.is_none() {
                let (new_tail, parsed_m_commander_mastery_talents) =
                    Self::parse_m_commander_mastery_talents(tail)?;
                tail = new_tail;
                m_commander_mastery_talents = Some(parsed_m_commander_mastery_talents);
            }
            if m_trophy_id.is_none() {
                let (new_tail, parsed_m_trophy_id) = Self::parse_m_trophy_id(tail)?;
                tail = new_tail;
                m_trophy_id = Some(parsed_m_trophy_id);
            }
            if m_reward_overrides.is_none() {
                let (new_tail, parsed_m_reward_overrides) = Self::parse_m_reward_overrides(tail)?;
                tail = new_tail;
                m_reward_overrides = Some(parsed_m_reward_overrides);
            }
            if m_brutal_plus_difficulty.is_none() {
                let (new_tail, parsed_m_brutal_plus_difficulty) =
                    Self::parse_m_brutal_plus_difficulty(tail)?;
                tail = new_tail;
                m_brutal_plus_difficulty = Some(parsed_m_brutal_plus_difficulty);
            }
            if m_retry_mutation_indexes.is_none() {
                let (new_tail, parsed_m_retry_mutation_indexes) =
                    Self::parse_m_retry_mutation_indexes(tail)?;
                tail = new_tail;
                m_retry_mutation_indexes = Some(parsed_m_retry_mutation_indexes);
            }
            if m_a_c_enemy_race.is_none() {
                let (new_tail, parsed_m_a_c_enemy_race) = Self::parse_m_a_c_enemy_race(tail)?;
                tail = new_tail;
                m_a_c_enemy_race = Some(parsed_m_a_c_enemy_race);
            }
            if m_a_c_enemy_wave_type.is_none() {
                let (new_tail, parsed_m_a_c_enemy_wave_type) =
                    Self::parse_m_a_c_enemy_wave_type(tail)?;
                tail = new_tail;
                m_a_c_enemy_wave_type = Some(parsed_m_a_c_enemy_wave_type);
            }
            if m_selected_commander_prestige.is_none() {
                let (new_tail, parsed_m_selected_commander_prestige) =
                    Self::parse_m_selected_commander_prestige(tail)?;
                tail = new_tail;
                m_selected_commander_prestige = Some(parsed_m_selected_commander_prestige);
            }
            Ok((
                tail,
                Self {
                    m_control: ok_or_return_missing_field_err!(m_control),
                    m_user_id: ok_or_return_missing_field_err!(m_user_id),
                    m_team_id: ok_or_return_missing_field_err!(m_team_id),
                    m_color_pref: ok_or_return_missing_field_err!(m_color_pref),
                    m_race_pref: ok_or_return_missing_field_err!(m_race_pref),
                    m_difficulty: ok_or_return_missing_field_err!(m_difficulty),
                    m_ai_build: ok_or_return_missing_field_err!(m_ai_build),
                    m_handicap: ok_or_return_missing_field_err!(m_handicap),
                    m_observe: ok_or_return_missing_field_err!(m_observe),
                    m_logo_index: ok_or_return_missing_field_err!(m_logo_index),
                    m_hero: ok_or_return_missing_field_err!(m_hero),
                    m_skin: ok_or_return_missing_field_err!(m_skin),
                    m_mount: ok_or_return_missing_field_err!(m_mount),
                    m_artifacts: ok_or_return_missing_field_err!(m_artifacts),
                    m_working_set_slot_id: ok_or_return_missing_field_err!(m_working_set_slot_id),
                    m_rewards: ok_or_return_missing_field_err!(m_rewards),
                    m_toon_handle: ok_or_return_missing_field_err!(m_toon_handle),
                    m_licenses: ok_or_return_missing_field_err!(m_licenses),
                    m_tandem_leader_id: ok_or_return_missing_field_err!(m_tandem_leader_id),
                    m_commander: ok_or_return_missing_field_err!(m_commander),
                    m_commander_level: ok_or_return_missing_field_err!(m_commander_level),
                    m_has_silence_penalty: ok_or_return_missing_field_err!(m_has_silence_penalty),
                    m_tandem_id: ok_or_return_missing_field_err!(m_tandem_id),
                    m_commander_mastery_level: ok_or_return_missing_field_err!(
                        m_commander_mastery_level
                    ),
                    m_commander_mastery_talents: ok_or_return_missing_field_err!(
                        m_commander_mastery_talents
                    ),
                    m_trophy_id: ok_or_return_missing_field_err!(m_trophy_id),
                    m_reward_overrides: ok_or_return_missing_field_err!(m_reward_overrides),
                    m_brutal_plus_difficulty: ok_or_return_missing_field_err!(
                        m_brutal_plus_difficulty
                    ),
                    m_retry_mutation_indexes: ok_or_return_missing_field_err!(
                        m_retry_mutation_indexes
                    ),
                    m_a_c_enemy_race: ok_or_return_missing_field_err!(m_a_c_enemy_race),
                    m_a_c_enemy_wave_type: ok_or_return_missing_field_err!(m_a_c_enemy_wave_type),
                    m_selected_commander_prestige: ok_or_return_missing_field_err!(
                        m_selected_commander_prestige
                    ),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCLobbySlotArray {
        pub value: Vec<GameSLobbySlot>,
    }
    impl GameCLobbySlotArray {
        #[tracing::instrument(name="87702::GameCLobbySlotArray::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 5;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(GameSLobbySlot::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameSLobbySlot>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameSLobbySlot::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameSLobbySlotChange {
        MControl(GameTControlId),
        MUserId(Option<TUserId>),
        MTeamId(GameTTeamId),
        MColorPref(GameTColorPreference),
        MRacePref(TRacePreference),
        MDifficulty(GameTDifficulty),
        MAiBuild(GameTaiBuild),
        MHandicap(GameTHandicap),
        MObserve(EObserve),
        MLogoIndex(GameTPlayerLogoIndex),
        MHero(CHeroHandle),
        MSkin(CSkinHandle),
        MMount(CMountHandle),
        MLicenses(GameCLicenseArray),
        MTandemLeaderId(Option<TUserId>),
        MCommander(CCommanderHandle),
        MCommanderLevel(Uint32),
        MHasSilencePenalty(bool),
        MTandemId(Option<TUserId>),
        MCommanderMasteryLevel(Uint32),
        MBrutalPlusDifficulty(Uint32),
        MRetryMutationIndexes(GameCRetryMutationIndexArray),
        MACEnemyRace(Uint32),
        MACEnemyWaveType(Uint32),
        MSelectedCommanderPrestige(Uint32),
    }
    impl GameSLobbySlotChange {
        #[tracing::instrument(name="87702::GameSLobbySlotChange::ChoiceType::parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // ChoiceType:
            // Use the number of elements in the json .fields to calculate how many
            // bits to have unique tags.
            // let num_fields: usize = 25;
            let offset = 0i64;
            let num_bits: usize = 5;
            let (tail, variant_tag) = parse_packed_int(input, offset, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant tagged '0' for MControl");
                    let (tail, res) = GameTControlId::parse(tail)?;
                    Ok((tail, Self::MControl(res)))
                }
                1 => {
                    tracing::debug!("Variant tagged '1' for MUserId");
                    let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(tail)?;
                    if is_provided {
                        let (tail, res) = TUserId::parse(tail)?;
                        Ok((tail, Self::MUserId(Some(res))))
                    } else {
                        Ok((tail, Self::MUserId(None)))
                    }
                }
                2 => {
                    tracing::debug!("Variant tagged '2' for MTeamId");
                    let (tail, res) = GameTTeamId::parse(tail)?;
                    Ok((tail, Self::MTeamId(res)))
                }
                3 => {
                    tracing::debug!("Variant tagged '3' for MColorPref");
                    let (tail, res) = GameTColorPreference::parse(tail)?;
                    Ok((tail, Self::MColorPref(res)))
                }
                4 => {
                    tracing::debug!("Variant tagged '4' for MRacePref");
                    let (tail, res) = TRacePreference::parse(tail)?;
                    Ok((tail, Self::MRacePref(res)))
                }
                5 => {
                    tracing::debug!("Variant tagged '5' for MDifficulty");
                    let (tail, res) = GameTDifficulty::parse(tail)?;
                    Ok((tail, Self::MDifficulty(res)))
                }
                6 => {
                    tracing::debug!("Variant tagged '6' for MAiBuild");
                    let (tail, res) = GameTaiBuild::parse(tail)?;
                    Ok((tail, Self::MAiBuild(res)))
                }
                7 => {
                    tracing::debug!("Variant tagged '7' for MHandicap");
                    let (tail, res) = GameTHandicap::parse(tail)?;
                    Ok((tail, Self::MHandicap(res)))
                }
                8 => {
                    tracing::debug!("Variant tagged '8' for MObserve");
                    let (tail, res) = EObserve::parse(tail)?;
                    Ok((tail, Self::MObserve(res)))
                }
                9 => {
                    tracing::debug!("Variant tagged '9' for MLogoIndex");
                    let (tail, res) = GameTPlayerLogoIndex::parse(tail)?;
                    Ok((tail, Self::MLogoIndex(res)))
                }
                10 => {
                    tracing::debug!("Variant tagged '10' for MHero");
                    let (tail, res) = CHeroHandle::parse(tail)?;
                    Ok((tail, Self::MHero(res)))
                }
                11 => {
                    tracing::debug!("Variant tagged '11' for MSkin");
                    let (tail, res) = CSkinHandle::parse(tail)?;
                    Ok((tail, Self::MSkin(res)))
                }
                12 => {
                    tracing::debug!("Variant tagged '12' for MMount");
                    let (tail, res) = CMountHandle::parse(tail)?;
                    Ok((tail, Self::MMount(res)))
                }
                13 => {
                    tracing::debug!("Variant tagged '13' for MLicenses");
                    let (tail, res) = GameCLicenseArray::parse(tail)?;
                    Ok((tail, Self::MLicenses(res)))
                }
                14 => {
                    tracing::debug!("Variant tagged '14' for MTandemLeaderId");
                    let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(tail)?;
                    if is_provided {
                        let (tail, res) = TUserId::parse(tail)?;
                        Ok((tail, Self::MTandemLeaderId(Some(res))))
                    } else {
                        Ok((tail, Self::MTandemLeaderId(None)))
                    }
                }
                15 => {
                    tracing::debug!("Variant tagged '15' for MCommander");
                    let (tail, res) = CCommanderHandle::parse(tail)?;
                    Ok((tail, Self::MCommander(res)))
                }
                16 => {
                    tracing::debug!("Variant tagged '16' for MCommanderLevel");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::MCommanderLevel(res)))
                }
                17 => {
                    tracing::debug!("Variant tagged '17' for MHasSilencePenalty");
                    let (tail, res) = parse_bool(tail)?;
                    Ok((tail, Self::MHasSilencePenalty(res)))
                }
                18 => {
                    tracing::debug!("Variant tagged '18' for MTandemId");
                    let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(tail)?;
                    if is_provided {
                        let (tail, res) = TUserId::parse(tail)?;
                        Ok((tail, Self::MTandemId(Some(res))))
                    } else {
                        Ok((tail, Self::MTandemId(None)))
                    }
                }
                19 => {
                    tracing::debug!("Variant tagged '19' for MCommanderMasteryLevel");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::MCommanderMasteryLevel(res)))
                }
                20 => {
                    tracing::debug!("Variant tagged '20' for MBrutalPlusDifficulty");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::MBrutalPlusDifficulty(res)))
                }
                21 => {
                    tracing::debug!("Variant tagged '21' for MRetryMutationIndexes");
                    let (tail, res) = GameCRetryMutationIndexArray::parse(tail)?;
                    Ok((tail, Self::MRetryMutationIndexes(res)))
                }
                22 => {
                    tracing::debug!("Variant tagged '22' for MACEnemyRace");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::MACEnemyRace(res)))
                }
                23 => {
                    tracing::debug!("Variant tagged '23' for MACEnemyWaveType");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::MACEnemyWaveType(res)))
                }
                24 => {
                    tracing::debug!("Variant tagged '24' for MSelectedCommanderPrestige");
                    let (tail, res) = Uint32::parse(tail)?;
                    Ok((tail, Self::MSelectedCommanderPrestige(res)))
                }

                _ => {
                    tracing::error!("Unknown variant for tag {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSLobbyState {
        pub m_phase: GameEPhase,
        pub m_max_users: TUserCount,
        pub m_max_observers: TUserCount,
        pub m_slots: GameCLobbySlotArray,
        pub m_random_seed: Uint32,
        pub m_host_user_id: Option<TUserId>,
        pub m_is_single_player: bool,
        pub m_picked_map_tag: Uint8,
        pub m_game_duration: Uint32,
        pub m_default_difficulty: GameTDifficulty,
        pub m_default_ai_build: GameTaiBuild,
    }
    impl GameSLobbyState {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_phase(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameEPhase> {
            let (tail, m_phase) = GameEPhase::parse(input)?;
            tracing::debug!("m_phase: {:?}", m_phase);
            Ok((tail, m_phase))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_max_users(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TUserCount> {
            let (tail, m_max_users) = TUserCount::parse(input)?;
            tracing::debug!("m_max_users: {:?}", m_max_users);
            Ok((tail, m_max_users))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_max_observers(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), TUserCount> {
            let (tail, m_max_observers) = TUserCount::parse(input)?;
            tracing::debug!("m_max_observers: {:?}", m_max_observers);
            Ok((tail, m_max_observers))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_slots(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCLobbySlotArray> {
            let (tail, m_slots) = GameCLobbySlotArray::parse(input)?;
            tracing::debug!("m_slots: {:?}", m_slots);
            Ok((tail, m_slots))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_random_seed(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_random_seed) = Uint32::parse(input)?;
            tracing::debug!("m_random_seed: {:?}", m_random_seed);
            Ok((tail, m_random_seed))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_host_user_id(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Option<TUserId>> {
            let (tail, is_provided): ((&[u8], usize), bool) = parse_bool(input)?;
            let (tail, m_host_user_id) = if is_provided {
                let (tail, res) = TUserId::parse(tail)?;
                (tail, Some(res))
            } else {
                (tail, None)
            };
            tracing::debug!("m_host_user_id: {:?}", m_host_user_id);
            Ok((tail, m_host_user_id))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_is_single_player(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), bool> {
            let (tail, m_is_single_player) = parse_bool(input)?;
            tracing::debug!("m_is_single_player: {:?}", m_is_single_player);
            Ok((tail, m_is_single_player))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_picked_map_tag(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint8> {
            let (tail, m_picked_map_tag) = Uint8::parse(input)?;
            tracing::debug!("m_picked_map_tag: {:?}", m_picked_map_tag);
            Ok((tail, m_picked_map_tag))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_duration(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Uint32> {
            let (tail, m_game_duration) = Uint32::parse(input)?;
            tracing::debug!("m_game_duration: {:?}", m_game_duration);
            Ok((tail, m_game_duration))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_default_difficulty(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTDifficulty> {
            let (tail, m_default_difficulty) = GameTDifficulty::parse(input)?;
            tracing::debug!("m_default_difficulty: {:?}", m_default_difficulty);
            Ok((tail, m_default_difficulty))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_default_ai_build(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTaiBuild> {
            let (tail, m_default_ai_build) = GameTaiBuild::parse(input)?;
            tracing::debug!("m_default_ai_build: {:?}", m_default_ai_build);
            Ok((tail, m_default_ai_build))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSLobbyState::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_phase: Option<GameEPhase> = None;
            let mut m_max_users: Option<TUserCount> = None;
            let mut m_max_observers: Option<TUserCount> = None;
            let mut m_slots: Option<GameCLobbySlotArray> = None;
            let mut m_random_seed: Option<Uint32> = None;
            let mut m_host_user_id: Option<Option<TUserId>> = Some(None);
            let mut m_is_single_player: Option<bool> = None;
            let mut m_picked_map_tag: Option<Uint8> = None;
            let mut m_game_duration: Option<Uint32> = None;
            let mut m_default_difficulty: Option<GameTDifficulty> = None;
            let mut m_default_ai_build: Option<GameTaiBuild> = None;
            if m_phase.is_none() {
                let (new_tail, parsed_m_phase) = Self::parse_m_phase(tail)?;
                tail = new_tail;
                m_phase = Some(parsed_m_phase);
            }
            if m_max_users.is_none() {
                let (new_tail, parsed_m_max_users) = Self::parse_m_max_users(tail)?;
                tail = new_tail;
                m_max_users = Some(parsed_m_max_users);
            }
            if m_max_observers.is_none() {
                let (new_tail, parsed_m_max_observers) = Self::parse_m_max_observers(tail)?;
                tail = new_tail;
                m_max_observers = Some(parsed_m_max_observers);
            }
            if m_slots.is_none() {
                let (new_tail, parsed_m_slots) = Self::parse_m_slots(tail)?;
                tail = new_tail;
                m_slots = Some(parsed_m_slots);
            }
            if m_random_seed.is_none() {
                let (new_tail, parsed_m_random_seed) = Self::parse_m_random_seed(tail)?;
                tail = new_tail;
                m_random_seed = Some(parsed_m_random_seed);
            }
            if let Some(None) = m_host_user_id {
                let (new_tail, parsed_m_host_user_id) = Self::parse_m_host_user_id(tail)?;
                tail = new_tail;
                m_host_user_id = Some(parsed_m_host_user_id);
            }
            if m_is_single_player.is_none() {
                let (new_tail, parsed_m_is_single_player) = Self::parse_m_is_single_player(tail)?;
                tail = new_tail;
                m_is_single_player = Some(parsed_m_is_single_player);
            }
            if m_picked_map_tag.is_none() {
                let (new_tail, parsed_m_picked_map_tag) = Self::parse_m_picked_map_tag(tail)?;
                tail = new_tail;
                m_picked_map_tag = Some(parsed_m_picked_map_tag);
            }
            if m_game_duration.is_none() {
                let (new_tail, parsed_m_game_duration) = Self::parse_m_game_duration(tail)?;
                tail = new_tail;
                m_game_duration = Some(parsed_m_game_duration);
            }
            if m_default_difficulty.is_none() {
                let (new_tail, parsed_m_default_difficulty) =
                    Self::parse_m_default_difficulty(tail)?;
                tail = new_tail;
                m_default_difficulty = Some(parsed_m_default_difficulty);
            }
            if m_default_ai_build.is_none() {
                let (new_tail, parsed_m_default_ai_build) = Self::parse_m_default_ai_build(tail)?;
                tail = new_tail;
                m_default_ai_build = Some(parsed_m_default_ai_build);
            }
            Ok((
                tail,
                Self {
                    m_phase: ok_or_return_missing_field_err!(m_phase),
                    m_max_users: ok_or_return_missing_field_err!(m_max_users),
                    m_max_observers: ok_or_return_missing_field_err!(m_max_observers),
                    m_slots: ok_or_return_missing_field_err!(m_slots),
                    m_random_seed: ok_or_return_missing_field_err!(m_random_seed),
                    m_host_user_id: ok_or_return_missing_field_err!(m_host_user_id),
                    m_is_single_player: ok_or_return_missing_field_err!(m_is_single_player),
                    m_picked_map_tag: ok_or_return_missing_field_err!(m_picked_map_tag),
                    m_game_duration: ok_or_return_missing_field_err!(m_game_duration),
                    m_default_difficulty: ok_or_return_missing_field_err!(m_default_difficulty),
                    m_default_ai_build: ok_or_return_missing_field_err!(m_default_ai_build),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSLobbySyncState {
        pub m_user_initial_data: CUserInitialDataArray,
        pub m_game_description: GameSGameDescription,
        pub m_lobby_state: GameSLobbyState,
    }
    impl GameSLobbySyncState {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_user_initial_data(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), CUserInitialDataArray> {
            let (tail, m_user_initial_data) = CUserInitialDataArray::parse(input)?;
            tracing::debug!("m_user_initial_data: {:?}", m_user_initial_data);
            Ok((tail, m_user_initial_data))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_game_description(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSGameDescription> {
            let (tail, m_game_description) = GameSGameDescription::parse(input)?;
            tracing::debug!("m_game_description: {:?}", m_game_description);
            Ok((tail, m_game_description))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_lobby_state(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSLobbyState> {
            let (tail, m_lobby_state) = GameSLobbyState::parse(input)?;
            tracing::debug!("m_lobby_state: {:?}", m_lobby_state);
            Ok((tail, m_lobby_state))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSLobbySyncState::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_user_initial_data: Option<CUserInitialDataArray> = None;
            let mut m_game_description: Option<GameSGameDescription> = None;
            let mut m_lobby_state: Option<GameSLobbyState> = None;
            if m_user_initial_data.is_none() {
                let (new_tail, parsed_m_user_initial_data) = Self::parse_m_user_initial_data(tail)?;
                tail = new_tail;
                m_user_initial_data = Some(parsed_m_user_initial_data);
            }
            if m_game_description.is_none() {
                let (new_tail, parsed_m_game_description) = Self::parse_m_game_description(tail)?;
                tail = new_tail;
                m_game_description = Some(parsed_m_game_description);
            }
            if m_lobby_state.is_none() {
                let (new_tail, parsed_m_lobby_state) = Self::parse_m_lobby_state(tail)?;
                tail = new_tail;
                m_lobby_state = Some(parsed_m_lobby_state);
            }
            Ok((
                tail,
                Self {
                    m_user_initial_data: ok_or_return_missing_field_err!(m_user_initial_data),
                    m_game_description: ok_or_return_missing_field_err!(m_game_description),
                    m_lobby_state: ok_or_return_missing_field_err!(m_lobby_state),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEMessageRecipient {
        EAll,
        EAllies,
        EIndividual,
        EBattlenet,
        EObservers,
    }
    impl GameEMessageRecipient {
        #[tracing::instrument(name="87702::GameEMessageRecipient::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 5
            let num_bits: usize = 3;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EAll for value '0'");
                    Ok((tail, Self::EAll))
                }
                1 => {
                    tracing::debug!("Variant EAllies for value '1'");
                    Ok((tail, Self::EAllies))
                }
                2 => {
                    tracing::debug!("Variant EIndividual for value '2'");
                    Ok((tail, Self::EIndividual))
                }
                3 => {
                    tracing::debug!("Variant EBattlenet for value '3'");
                    Ok((tail, Self::EBattlenet))
                }
                4 => {
                    tracing::debug!("Variant EObservers for value '4'");
                    Ok((tail, Self::EObservers))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameCChatString {
        pub value: Vec<u8>,
    }
    impl GameCChatString {
        #[tracing::instrument(name="87702::GameCChatString::StringType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let str_size_num_bits: usize = 11;
            let (tail, str_size) = parse_packed_int(input, 0, str_size_num_bits)?;
            let (tail, _) = byte_align(tail)?;
            let (tail, value) = take_bit_array(tail, str_size as usize * 8usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEMessageId {
        EChat(GameSChatMessage),
        EPing(GameSPingMessage),
        ELoadingProgress(GameSLoadingProgressMessage),
        EServerPing(GameSServerPingMessage),
        EReconnectNotify(GameSReconnectNotifyMessage),
    }
    impl GameEMessageId {
        #[tracing::instrument(name="87702::GameEMessageId::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 5
            let num_bits: usize = 4;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EChat for value '0'");

                    let (tail, res) = GameSChatMessage::parse(tail)?;
                    Ok((tail, Self::EChat(res)))
                }
                1 => {
                    tracing::debug!("Variant EPing for value '1'");

                    let (tail, res) = GameSPingMessage::parse(tail)?;
                    Ok((tail, Self::EPing(res)))
                }
                2 => {
                    tracing::debug!("Variant ELoadingProgress for value '2'");

                    let (tail, res) = GameSLoadingProgressMessage::parse(tail)?;
                    Ok((tail, Self::ELoadingProgress(res)))
                }
                3 => {
                    tracing::debug!("Variant EServerPing for value '3'");

                    let (tail, res) = GameSServerPingMessage::parse(tail)?;
                    Ok((tail, Self::EServerPing(res)))
                }
                4 => {
                    tracing::debug!("Variant EReconnectNotify for value '4'");

                    let (tail, res) = GameSReconnectNotifyMessage::parse(tail)?;
                    Ok((tail, Self::EReconnectNotify(res)))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSChatMessage {
        pub m_recipient: GameEMessageRecipient,
        pub m_string: GameCChatString,
    }
    impl GameSChatMessage {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_recipient(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEMessageRecipient> {
            let (tail, m_recipient) = GameEMessageRecipient::parse(input)?;
            tracing::debug!("m_recipient: {:?}", m_recipient);
            Ok((tail, m_recipient))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_string(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameCChatString> {
            let (tail, m_string) = GameCChatString::parse(input)?;
            tracing::debug!("m_string: {:?}", m_string);
            Ok((tail, m_string))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSChatMessage::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_recipient: Option<GameEMessageRecipient> = None;
            let mut m_string: Option<GameCChatString> = None;
            if m_recipient.is_none() {
                let (new_tail, parsed_m_recipient) = Self::parse_m_recipient(tail)?;
                tail = new_tail;
                m_recipient = Some(parsed_m_recipient);
            }
            if m_string.is_none() {
                let (new_tail, parsed_m_string) = Self::parse_m_string(tail)?;
                tail = new_tail;
                m_string = Some(parsed_m_string);
            }
            Ok((
                tail,
                Self {
                    m_recipient: ok_or_return_missing_field_err!(m_recipient),
                    m_string: ok_or_return_missing_field_err!(m_string),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSPingMessage {
        pub m_recipient: GameEMessageRecipient,
        pub m_point: GameSPoint,
    }
    impl GameSPingMessage {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_recipient(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameEMessageRecipient> {
            let (tail, m_recipient) = GameEMessageRecipient::parse(input)?;
            tracing::debug!("m_recipient: {:?}", m_recipient);
            Ok((tail, m_recipient))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_point(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), GameSPoint> {
            let (tail, m_point) = GameSPoint::parse(input)?;
            tracing::debug!("m_point: {:?}", m_point);
            Ok((tail, m_point))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSPingMessage::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_recipient: Option<GameEMessageRecipient> = None;
            let mut m_point: Option<GameSPoint> = None;
            if m_recipient.is_none() {
                let (new_tail, parsed_m_recipient) = Self::parse_m_recipient(tail)?;
                tail = new_tail;
                m_recipient = Some(parsed_m_recipient);
            }
            if m_point.is_none() {
                let (new_tail, parsed_m_point) = Self::parse_m_point(tail)?;
                tail = new_tail;
                m_point = Some(parsed_m_point);
            }
            Ok((
                tail,
                Self {
                    m_recipient: ok_or_return_missing_field_err!(m_recipient),
                    m_point: ok_or_return_missing_field_err!(m_point),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSLoadingProgressMessage {
        pub m_progress: Int32,
    }
    impl GameSLoadingProgressMessage {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_progress(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Int32> {
            let (tail, m_progress) = Int32::parse(input)?;
            tracing::debug!("m_progress: {:?}", m_progress);
            Ok((tail, m_progress))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSLoadingProgressMessage::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_progress: Option<Int32> = None;
            if m_progress.is_none() {
                let (new_tail, parsed_m_progress) = Self::parse_m_progress(tail)?;
                tail = new_tail;
                m_progress = Some(parsed_m_progress);
            }
            Ok((
                tail,
                Self {
                    m_progress: ok_or_return_missing_field_err!(m_progress),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSServerPingMessage {}
    impl GameSServerPingMessage {
        #[tracing::instrument(name="87702::bit_packed::GameSServerPingMessage::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let tail = input;
            Ok((tail, Self {}))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSReconnectNotifyMessage {
        pub m_status: EReconnectStatus,
    }
    impl GameSReconnectNotifyMessage {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_status(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), EReconnectStatus> {
            let (tail, m_status) = EReconnectStatus::parse(input)?;
            tracing::debug!("m_status: {:?}", m_status);
            Ok((tail, m_status))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSReconnectNotifyMessage::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_status: Option<EReconnectStatus> = None;
            if m_status.is_none() {
                let (new_tail, parsed_m_status) = Self::parse_m_status(tail)?;
                tail = new_tail;
                m_status = Some(parsed_m_status);
            }
            Ok((
                tail,
                Self {
                    m_status: ok_or_return_missing_field_err!(m_status),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTPlayerId {
        pub value: i64,
    }
    impl GameTPlayerId {
        #[tracing::instrument(name="87702::GameTPlayerId::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 4;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTPlayerCount {
        pub value: i64,
    }
    impl GameTPlayerCount {
        #[tracing::instrument(name="87702::GameTPlayerCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 5;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEResultCode {
        EUndecided,
        ELoss,
        ETie,
        EWin,
    }
    impl GameEResultCode {
        #[tracing::instrument(name="87702::GameEResultCode::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 4
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant EUndecided for value '0'");
                    Ok((tail, Self::EUndecided))
                }
                1 => {
                    tracing::debug!("Variant ELoss for value '1'");
                    Ok((tail, Self::ELoss))
                }
                2 => {
                    tracing::debug!("Variant ETie for value '2'");
                    Ok((tail, Self::ETie))
                }
                3 => {
                    tracing::debug!("Variant EWin for value '3'");
                    Ok((tail, Self::EWin))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameEControlGroupUpdate {
        ESet,
        EAppend,
        ERecall,
        EClear,
        ESetAndSteal,
        EAppendAndSteal,
    }
    impl GameEControlGroupUpdate {
        #[tracing::instrument(name="87702::GameEControlGroupUpdate::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // Total fields: 6
            let num_bits: usize = 3;
            let (tail, variant_tag) = parse_packed_int(input, 0, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant ESet for value '0'");
                    Ok((tail, Self::ESet))
                }
                1 => {
                    tracing::debug!("Variant EAppend for value '1'");
                    Ok((tail, Self::EAppend))
                }
                2 => {
                    tracing::debug!("Variant ERecall for value '2'");
                    Ok((tail, Self::ERecall))
                }
                3 => {
                    tracing::debug!("Variant EClear for value '3'");
                    Ok((tail, Self::EClear))
                }
                4 => {
                    tracing::debug!("Variant ESetAndSteal for value '4'");
                    Ok((tail, Self::ESetAndSteal))
                }
                5 => {
                    tracing::debug!("Variant EAppendAndSteal for value '5'");
                    Ok((tail, Self::EAppendAndSteal))
                }

                _ => {
                    tracing::error!("Unknown variant value {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTSelectionCount {
        pub value: i64,
    }
    impl GameTSelectionCount {
        #[tracing::instrument(name="87702::GameTSelectionCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 9;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTSelectionIndex {
        pub value: i64,
    }
    impl GameTSelectionIndex {
        #[tracing::instrument(name="87702::GameTSelectionIndex::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 9;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTSubgroupPriority {
        pub value: i64,
    }
    impl GameTSubgroupPriority {
        #[tracing::instrument(name="87702::GameTSubgroupPriority::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 8;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTSubgroupCount {
        pub value: i64,
    }
    impl GameTSubgroupCount {
        #[tracing::instrument(name="87702::GameTSubgroupCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 9;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTSubgroupIndex {
        pub value: i64,
    }
    impl GameTSubgroupIndex {
        #[tracing::instrument(name="87702::GameTSubgroupIndex::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 9;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTControlGroupCount {
        pub value: i64,
    }
    impl GameTControlGroupCount {
        #[tracing::instrument(name="87702::GameTControlGroupCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 4;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTControlGroupIndex {
        pub value: i64,
    }
    impl GameTControlGroupIndex {
        #[tracing::instrument(name="87702::GameTControlGroupIndex::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 4;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTControlGroupId {
        pub value: i64,
    }
    impl GameTControlGroupId {
        #[tracing::instrument(name="87702::GameTControlGroupId::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 4;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSelectionDeltaSubgroup {
        pub m_unit_link: GameTUnitLink,
        pub m_subgroup_priority: GameTSubgroupPriority,
        pub m_intra_subgroup_priority: GameTSubgroupPriority,
        pub m_count: GameTSelectionCount,
    }
    impl GameSSelectionDeltaSubgroup {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_link(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTUnitLink> {
            let (tail, m_unit_link) = GameTUnitLink::parse(input)?;
            tracing::debug!("m_unit_link: {:?}", m_unit_link);
            Ok((tail, m_unit_link))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_subgroup_priority(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSubgroupPriority> {
            let (tail, m_subgroup_priority) = GameTSubgroupPriority::parse(input)?;
            tracing::debug!("m_subgroup_priority: {:?}", m_subgroup_priority);
            Ok((tail, m_subgroup_priority))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_intra_subgroup_priority(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSubgroupPriority> {
            let (tail, m_intra_subgroup_priority) = GameTSubgroupPriority::parse(input)?;
            tracing::debug!("m_intra_subgroup_priority: {:?}", m_intra_subgroup_priority);
            Ok((tail, m_intra_subgroup_priority))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_count(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSelectionCount> {
            let (tail, m_count) = GameTSelectionCount::parse(input)?;
            tracing::debug!("m_count: {:?}", m_count);
            Ok((tail, m_count))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSelectionDeltaSubgroup::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_unit_link: Option<GameTUnitLink> = None;
            let mut m_subgroup_priority: Option<GameTSubgroupPriority> = None;
            let mut m_intra_subgroup_priority: Option<GameTSubgroupPriority> = None;
            let mut m_count: Option<GameTSelectionCount> = None;
            if m_unit_link.is_none() {
                let (new_tail, parsed_m_unit_link) = Self::parse_m_unit_link(tail)?;
                tail = new_tail;
                m_unit_link = Some(parsed_m_unit_link);
            }
            if m_subgroup_priority.is_none() {
                let (new_tail, parsed_m_subgroup_priority) = Self::parse_m_subgroup_priority(tail)?;
                tail = new_tail;
                m_subgroup_priority = Some(parsed_m_subgroup_priority);
            }
            if m_intra_subgroup_priority.is_none() {
                let (new_tail, parsed_m_intra_subgroup_priority) =
                    Self::parse_m_intra_subgroup_priority(tail)?;
                tail = new_tail;
                m_intra_subgroup_priority = Some(parsed_m_intra_subgroup_priority);
            }
            if m_count.is_none() {
                let (new_tail, parsed_m_count) = Self::parse_m_count(tail)?;
                tail = new_tail;
                m_count = Some(parsed_m_count);
            }
            Ok((
                tail,
                Self {
                    m_unit_link: ok_or_return_missing_field_err!(m_unit_link),
                    m_subgroup_priority: ok_or_return_missing_field_err!(m_subgroup_priority),
                    m_intra_subgroup_priority: ok_or_return_missing_field_err!(
                        m_intra_subgroup_priority
                    ),
                    m_count: ok_or_return_missing_field_err!(m_count),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSelectionIndexArrayType {
        pub value: Vec<GameTSelectionIndex>,
    }
    impl GameSelectionIndexArrayType {
        #[tracing::instrument(name="87702::GameSelectionIndexArrayType::ArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let array_length_num_bits: usize = 9;
            let (mut tail, array_length) = parse_packed_int(input, 0, array_length_num_bits)?;
            // compat_count(GameTSelectionIndex::parse, array_length as usize)(tail)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let max_initial_capacity =
                MAX_INITIAL_CAPACITY_BYTES / std::mem::size_of::<GameTSelectionIndex>().max(1);
            let mut array = Vec::with_capacity(array_length.min(max_initial_capacity));
            for _ in 0..array_length {
                let (new_tail, data) = GameTSelectionIndex::parse(tail)?;
                tail = new_tail;
                array.push(data);
            }
            Ok((tail, Self { value: array }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSelectionMaskType {
        pub value: Vec<u8>,
    }
    impl GameSelectionMaskType {
        #[tracing::instrument(name="87702::GameSelectionMaskType::BitArrayType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let bitarray_length_bits: usize = 9;
            let (tail, bitarray_length) = take_n_bits_into_i64(input, bitarray_length_bits)?;
            tracing::debug!("Reading bitarray length: {bitarray_length}");
            let (tail, value) = take_bit_array(tail, bitarray_length as usize)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum GameSSelectionMask {
        None(()),
        Mask(GameSelectionMaskType),
        OneIndices(GameSelectionIndexArrayType),
        ZeroIndices(GameSelectionIndexArrayType),
    }
    impl GameSSelectionMask {
        #[tracing::instrument(name="87702::GameSSelectionMask::ChoiceType::parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            // ChoiceType:
            // Use the number of elements in the json .fields to calculate how many
            // bits to have unique tags.
            // let num_fields: usize = 4;
            let offset = 0i64;
            let num_bits: usize = 2;
            let (tail, variant_tag) = parse_packed_int(input, offset, num_bits)?;
            match variant_tag {
                0 => {
                    tracing::debug!("Variant tagged '0' for None");
                    let (tail, res) = take_null(tail)?;
                    Ok((tail, Self::None(res)))
                }
                1 => {
                    tracing::debug!("Variant tagged '1' for Mask");
                    let (tail, res) = GameSelectionMaskType::parse(tail)?;
                    Ok((tail, Self::Mask(res)))
                }
                2 => {
                    tracing::debug!("Variant tagged '2' for OneIndices");
                    let (tail, res) = GameSelectionIndexArrayType::parse(tail)?;
                    Ok((tail, Self::OneIndices(res)))
                }
                3 => {
                    tracing::debug!("Variant tagged '3' for ZeroIndices");
                    let (tail, res) = GameSelectionIndexArrayType::parse(tail)?;
                    Ok((tail, Self::ZeroIndices(res)))
                }

                _ => {
                    tracing::error!("Unknown variant for tag {variant_tag}");
                    Err(S2ProtocolError::UnknownTag(variant_tag))
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSelectionDelta {
        pub m_subgroup_index: GameTSubgroupIndex,
        pub m_remove_mask: GameSSelectionMask,
        pub m_add_subgroups: Vec<GameSSelectionDeltaSubgroup>,
        pub m_add_unit_tags: Vec<GameTUnitTag>,
    }
    impl GameSSelectionDelta {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_subgroup_index(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSubgroupIndex> {
            let (tail, m_subgroup_index) = GameTSubgroupIndex::parse(input)?;
            tracing::debug!("m_subgroup_index: {:?}", m_subgroup_index);
            Ok((tail, m_subgroup_index))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_remove_mask(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSSelectionMask> {
            let (tail, m_remove_mask) = GameSSelectionMask::parse(input)?;
            tracing::debug!("m_remove_mask: {:?}", m_remove_mask);
            Ok((tail, m_remove_mask))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_add_subgroups(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<GameSSelectionDeltaSubgroup>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 9)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = GameSSelectionDeltaSubgroup::parse(tail)?;
                tail = new_tail;
                let data = data;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_add_unit_tags(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<GameTUnitTag>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 9)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = GameTUnitTag::parse(tail)?;
                tail = new_tail;
                let data = data;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSelectionDelta::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_subgroup_index: Option<GameTSubgroupIndex> = None;
            let mut m_remove_mask: Option<GameSSelectionMask> = None;
            let mut m_add_subgroups: Option<Vec<GameSSelectionDeltaSubgroup>> = None;
            let mut m_add_unit_tags: Option<Vec<GameTUnitTag>> = None;
            if m_subgroup_index.is_none() {
                let (new_tail, parsed_m_subgroup_index) = Self::parse_m_subgroup_index(tail)?;
                tail = new_tail;
                m_subgroup_index = Some(parsed_m_subgroup_index);
            }
            if m_remove_mask.is_none() {
                let (new_tail, parsed_m_remove_mask) = Self::parse_m_remove_mask(tail)?;
                tail = new_tail;
                m_remove_mask = Some(parsed_m_remove_mask);
            }
            if m_add_subgroups.is_none() {
                let (new_tail, parsed_m_add_subgroups) = Self::parse_m_add_subgroups(tail)?;
                tail = new_tail;
                m_add_subgroups = Some(parsed_m_add_subgroups);
            }
            if m_add_unit_tags.is_none() {
                let (new_tail, parsed_m_add_unit_tags) = Self::parse_m_add_unit_tags(tail)?;
                tail = new_tail;
                m_add_unit_tags = Some(parsed_m_add_unit_tags);
            }
            Ok((
                tail,
                Self {
                    m_subgroup_index: ok_or_return_missing_field_err!(m_subgroup_index),
                    m_remove_mask: ok_or_return_missing_field_err!(m_remove_mask),
                    m_add_subgroups: ok_or_return_missing_field_err!(m_add_subgroups),
                    m_add_unit_tags: ok_or_return_missing_field_err!(m_add_unit_tags),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSelectionSyncData {
        pub m_count: GameTSelectionCount,
        pub m_subgroup_count: GameTSubgroupCount,
        pub m_active_subgroup_index: GameTSubgroupIndex,
        pub m_unit_tags_checksum: GameTSyncChecksum,
        pub m_subgroup_indices_checksum: GameTSyncChecksum,
        pub m_subgroups_checksum: GameTSyncChecksum,
    }
    impl GameSSelectionSyncData {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_count(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSelectionCount> {
            let (tail, m_count) = GameTSelectionCount::parse(input)?;
            tracing::debug!("m_count: {:?}", m_count);
            Ok((tail, m_count))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_subgroup_count(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSubgroupCount> {
            let (tail, m_subgroup_count) = GameTSubgroupCount::parse(input)?;
            tracing::debug!("m_subgroup_count: {:?}", m_subgroup_count);
            Ok((tail, m_subgroup_count))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_active_subgroup_index(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSubgroupIndex> {
            let (tail, m_active_subgroup_index) = GameTSubgroupIndex::parse(input)?;
            tracing::debug!("m_active_subgroup_index: {:?}", m_active_subgroup_index);
            Ok((tail, m_active_subgroup_index))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_unit_tags_checksum(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSyncChecksum> {
            let (tail, m_unit_tags_checksum) = GameTSyncChecksum::parse(input)?;
            tracing::debug!("m_unit_tags_checksum: {:?}", m_unit_tags_checksum);
            Ok((tail, m_unit_tags_checksum))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_subgroup_indices_checksum(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSyncChecksum> {
            let (tail, m_subgroup_indices_checksum) = GameTSyncChecksum::parse(input)?;
            tracing::debug!(
                "m_subgroup_indices_checksum: {:?}",
                m_subgroup_indices_checksum
            );
            Ok((tail, m_subgroup_indices_checksum))
        }
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_subgroups_checksum(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameTSyncChecksum> {
            let (tail, m_subgroups_checksum) = GameTSyncChecksum::parse(input)?;
            tracing::debug!("m_subgroups_checksum: {:?}", m_subgroups_checksum);
            Ok((tail, m_subgroups_checksum))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSelectionSyncData::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_count: Option<GameTSelectionCount> = None;
            let mut m_subgroup_count: Option<GameTSubgroupCount> = None;
            let mut m_active_subgroup_index: Option<GameTSubgroupIndex> = None;
            let mut m_unit_tags_checksum: Option<GameTSyncChecksum> = None;
            let mut m_subgroup_indices_checksum: Option<GameTSyncChecksum> = None;
            let mut m_subgroups_checksum: Option<GameTSyncChecksum> = None;
            if m_count.is_none() {
                let (new_tail, parsed_m_count) = Self::parse_m_count(tail)?;
                tail = new_tail;
                m_count = Some(parsed_m_count);
            }
            if m_subgroup_count.is_none() {
                let (new_tail, parsed_m_subgroup_count) = Self::parse_m_subgroup_count(tail)?;
                tail = new_tail;
                m_subgroup_count = Some(parsed_m_subgroup_count);
            }
            if m_active_subgroup_index.is_none() {
                let (new_tail, parsed_m_active_subgroup_index) =
                    Self::parse_m_active_subgroup_index(tail)?;
                tail = new_tail;
                m_active_subgroup_index = Some(parsed_m_active_subgroup_index);
            }
            if m_unit_tags_checksum.is_none() {
                let (new_tail, parsed_m_unit_tags_checksum) =
                    Self::parse_m_unit_tags_checksum(tail)?;
                tail = new_tail;
                m_unit_tags_checksum = Some(parsed_m_unit_tags_checksum);
            }
            if m_subgroup_indices_checksum.is_none() {
                let (new_tail, parsed_m_subgroup_indices_checksum) =
                    Self::parse_m_subgroup_indices_checksum(tail)?;
                tail = new_tail;
                m_subgroup_indices_checksum = Some(parsed_m_subgroup_indices_checksum);
            }
            if m_subgroups_checksum.is_none() {
                let (new_tail, parsed_m_subgroups_checksum) =
                    Self::parse_m_subgroups_checksum(tail)?;
                tail = new_tail;
                m_subgroups_checksum = Some(parsed_m_subgroups_checksum);
            }
            Ok((
                tail,
                Self {
                    m_count: ok_or_return_missing_field_err!(m_count),
                    m_subgroup_count: ok_or_return_missing_field_err!(m_subgroup_count),
                    m_active_subgroup_index: ok_or_return_missing_field_err!(
                        m_active_subgroup_index
                    ),
                    m_unit_tags_checksum: ok_or_return_missing_field_err!(m_unit_tags_checksum),
                    m_subgroup_indices_checksum: ok_or_return_missing_field_err!(
                        m_subgroup_indices_checksum
                    ),
                    m_subgroups_checksum: ok_or_return_missing_field_err!(m_subgroups_checksum),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTSyncChecksum {
        pub value: Uint32,
    }
    impl GameTSyncChecksum {
        #[tracing::instrument(name="87702::GameTSyncChecksum::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint32::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTSyncValue {
        pub value: Uint16,
    }
    impl GameTSyncValue {
        #[tracing::instrument(name="87702::GameTSyncValue::UserType::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let (tail, value) = Uint16::parse(input)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSSessionSyncInfo {
        pub m_checksums: Vec<GameTSyncChecksum>,
    }
    impl GameSSessionSyncInfo {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_checksums(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<GameTSyncChecksum>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 6)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = GameTSyncChecksum::parse(tail)?;
                tail = new_tail;
                let data = data;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSSessionSyncInfo::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_checksums: Option<Vec<GameTSyncChecksum>> = None;
            if m_checksums.is_none() {
                let (new_tail, parsed_m_checksums) = Self::parse_m_checksums(tail)?;
                tail = new_tail;
                m_checksums = Some(parsed_m_checksums);
            }
            Ok((
                tail,
                Self {
                    m_checksums: ok_or_return_missing_field_err!(m_checksums),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameSGameSyncInfo {
        pub m_checksums: Vec<GameTSyncChecksum>,
    }
    impl GameSGameSyncInfo {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_checksums(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), Vec<GameTSyncChecksum>> {
            let (mut tail, array_length) = take_n_bits_into_i64(input, 8)?;
            let array_length = array_length as usize;
            tracing::debug!("Reading array length: {array_length}");
            let mut array = vec![];
            for _ in 0..array_length {
                let (new_tail, data) = GameTSyncChecksum::parse(tail)?;
                tail = new_tail;
                let data = data;
                array.push(data);
            }
            Ok((tail, array))
        }
        #[tracing::instrument(name="87702::bit_packed::GameSGameSyncInfo::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_checksums: Option<Vec<GameTSyncChecksum>> = None;
            if m_checksums.is_none() {
                let (new_tail, parsed_m_checksums) = Self::parse_m_checksums(tail)?;
                tail = new_tail;
                m_checksums = Some(parsed_m_checksums);
            }
            Ok((
                tail,
                Self {
                    m_checksums: ok_or_return_missing_field_err!(m_checksums),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTTeamId {
        pub value: i64,
    }
    impl GameTTeamId {
        #[tracing::instrument(name="87702::GameTTeamId::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 0;
            let num_bits: usize = 4;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct GameTTeamCount {
        pub value: i64,
    }
    impl GameTTeamCount {
        #[tracing::instrument(name="87702::GameTTeamCount::IntType::Parse::MinMaxConstraint", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let offset: i64 = 1;
            let num_bits: usize = 5;
            let (tail, value) = parse_packed_int(input, offset, num_bits)?;
            Ok((tail, Self { value }))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReplaySInitData {
        pub m_sync_lobby_state: GameSLobbySyncState,
    }
    impl ReplaySInitData {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_sync_lobby_state(
            input: (&[u8], usize),
        ) -> S2ProtoResult<(&[u8], usize), GameSLobbySyncState> {
            let (tail, m_sync_lobby_state) = GameSLobbySyncState::parse(input)?;
            tracing::debug!("m_sync_lobby_state: {:?}", m_sync_lobby_state);
            Ok((tail, m_sync_lobby_state))
        }
        #[tracing::instrument(name="87702::bit_packed::ReplaySInitData::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_sync_lobby_state: Option<GameSLobbySyncState> = None;
            if m_sync_lobby_state.is_none() {
                let (new_tail, parsed_m_sync_lobby_state) = Self::parse_m_sync_lobby_state(tail)?;
                tail = new_tail;
                m_sync_lobby_state = Some(parsed_m_sync_lobby_state);
            }
            Ok((
                tail,
                Self {
                    m_sync_lobby_state: ok_or_return_missing_field_err!(m_sync_lobby_state),
                },
            ))
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct ReplaySGameUserId {
        pub m_user_id: i64,
    }
    impl ReplaySGameUserId {
        #[tracing::instrument(level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse_m_user_id(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), i64> {
            let (tail, m_user_id) = parse_packed_int(input, 0, 5usize)?;
            tracing::debug!("m_user_id: {:?}", m_user_id);
            Ok((tail, m_user_id))
        }
        #[tracing::instrument(name="87702::bit_packed::ReplaySGameUserId::Parse", level = "debug", skip(input), fields(peek = peek_bits(input)))]
        pub fn parse(input: (&[u8], usize)) -> S2ProtoResult<(&[u8], usize), Self> {
            let mut tail = input;
            let mut m_user_id: Option<i64> = None;
            if m_user_id.is_none() {
                let (new_tail, parsed_m_user_id) = Self::parse_m_user_id(tail)?;
                tail = new_tail;
                m_user_id = Some(parsed_m_user_id);
            }
            Ok((
                tail,
                Self {
                    m_user_id: ok_or_return_missing_field_err!(m_user_id),
                },
            ))
        }
    }
}
