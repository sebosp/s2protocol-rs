//! Common utilities and transformations between different message types

impl From<super::bit_packed::EObserve> for crate::common::EObserve {
    fn from(value: super::bit_packed::EObserve) -> Self {
        match value {
            super::bit_packed::EObserve::ENone => Self::ENone,
            super::bit_packed::EObserve::ESpectator => Self::ESpectator,
            super::bit_packed::EObserve::EReferee => Self::EReferee,
        }
    }
}

impl From<super::byte_aligned::EObserve> for crate::common::EObserve {
    fn from(value: super::byte_aligned::EObserve) -> Self {
        match value {
            super::byte_aligned::EObserve::ENone => Self::ENone,
            super::byte_aligned::EObserve::ESpectator => Self::ESpectator,
            super::byte_aligned::EObserve::EReferee => Self::EReferee,
        }
    }
}

impl From<super::bit_packed::Uint32> for u32 {
    fn from(source: super::bit_packed::Uint32) -> u32 {
        source.value as u32
    }
}

impl From<super::bit_packed::Int8> for i8 {
    fn from(source: super::bit_packed::Int8) -> i8 {
        source.value as i8
    }
}

impl From<super::bit_packed::Uint8> for u8 {
    fn from(source: super::bit_packed::Uint8) -> u8 {
        source.value as u8
    }
}

impl From<super::bit_packed::Uint16> for u16 {
    fn from(source: super::bit_packed::Uint16) -> u16 {
        source.value as u16
    }
}

impl From<super::bit_packed::Int16> for i16 {
    fn from(source: super::bit_packed::Int16) -> i16 {
        source.value as i16
    }
}

impl From<super::bit_packed::Int32> for i32 {
    fn from(source: super::bit_packed::Int32) -> i32 {
        source.value as i32
    }
}

impl From<super::bit_packed::TUserId> for u8 {
    fn from(source: super::bit_packed::TUserId) -> u8 {
        source.value as u8
    }
}
