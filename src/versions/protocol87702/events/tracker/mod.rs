//! Converts Tracker Events from protocol-version specific to protocol-agnostic versions
use super::byte_aligned::*;
use crate::tracker_events::{
    ReplayTrackerEvent, TrackerEvent, TrackerEventError, UnitBornEvent, UnitDiedEvent,
    UnitDoneEvent, UnitInitEvent, UnitPositionsEvent, UnitTypeChangeEvent,
};
use crate::*;
use nom::*;
use nom_mpq::parser::peek_hex;
use nom_mpq::MPQ;

impl ReplayTrackerEEventId {
    /// Reads a delta, TrackerEvent pair
    #[tracing::instrument(name="TrackerEvent::parse_event_pair", level = "debug", skip(input), fields(peek = peek_hex(input)))]
    pub fn parse_event_pair(input: &[u8]) -> IResult<&[u8], (u32, ReplayTrackerEEventId)> {
        let (tail, delta) = SVarUint32::parse(input)?;
        let (tail, event) = ReplayTrackerEEventId::parse(tail)?;
        let delta = match delta {
            SVarUint32::MUint6(val) => val as u32,
            SVarUint32::MUint14(val) | SVarUint32::MUint22(val) | SVarUint32::MUint32(val) => val,
        };
        Ok((tail, (delta, event)))
    }

    /// Read the Tracker Events
    pub fn read_tracker_events(mpq: &MPQ, file_contents: &[u8]) -> Vec<TrackerEvent> {
        // TODO: Make it return an Iterator.
        let (_event_tail, tracker_events) = mpq
            .read_mpq_file_sector("replay.tracker.events", false, &file_contents)
            .unwrap();
        let mut res = vec![];
        let mut event_tail: &[u8] = &tracker_events;
        loop {
            let (new_event_tail, (delta, event)) =
                Self::parse_event_pair(&event_tail).expect("Unable to parse TrackerEvents");
            event_tail = new_event_tail;
            match event.try_into() {
                Ok(val) => res.push(TrackerEvent { delta, event: val }),
                Err(err) => {
                    tracing::debug!("Skipping event: {:?}", err);
                }
            };
            if event_tail.input_len() == 0 {
                break;
            }
        }
        res
    }
}

impl TryFrom<ReplayTrackerEEventId> for ReplayTrackerEvent {
    type Error = TrackerEventError;

    fn try_from(value: ReplayTrackerEEventId) -> Result<Self, Self::Error> {
        match value {
            ReplayTrackerEEventId::EPlayerStats(_)
            | ReplayTrackerEEventId::EUnitOwnerChange(_)
            | ReplayTrackerEEventId::EUpgrade(_)
            | ReplayTrackerEEventId::EPlayerSetup(_) => {
                Err(TrackerEventError::UnsupportedEventType)
            }
            ReplayTrackerEEventId::EUnitBorn(e) => Ok(e.try_into()?),
            ReplayTrackerEEventId::EUnitDied(e) => Ok(e.into()),
            ReplayTrackerEEventId::EUnitTypeChange(e) => Ok(e.try_into()?),
            ReplayTrackerEEventId::EUnitInit(e) => Ok(e.try_into()?),
            ReplayTrackerEEventId::EUnitDone(e) => Ok(e.into()),
            ReplayTrackerEEventId::EUnitPosition(e) => Ok(e.into()),
        }
    }
}

impl TryFrom<ReplayTrackerSUnitBornEvent> for ReplayTrackerEvent {
    type Error = TrackerEventError;
    fn try_from(source: ReplayTrackerSUnitBornEvent) -> Result<Self, Self::Error> {
        let creator_ability_name = if let Some(val) = source.m_creator_ability_name {
            Some(str::from_utf8(&val)?.to_string())
        } else {
            None
        };
        Ok(ReplayTrackerEvent::UnitBorn(UnitBornEvent {
            unit_tag_index: source.m_unit_tag_index,
            unit_tag_recycle: source.m_unit_tag_recycle,
            unit_type_name: str::from_utf8(&source.m_unit_type_name)?.to_string(),
            control_player_id: source.m_control_player_id,
            upkeep_player_id: source.m_upkeep_player_id,
            x: source.m_x,
            y: source.m_y,
            creator_unit_tag_index: source.m_creator_unit_tag_index,
            creator_unit_tag_recycle: source.m_creator_unit_tag_recycle,
            creator_ability_name,
        }))
    }
}

impl From<ReplayTrackerSUnitDiedEvent> for ReplayTrackerEvent {
    fn from(source: ReplayTrackerSUnitDiedEvent) -> ReplayTrackerEvent {
        ReplayTrackerEvent::UnitDied(UnitDiedEvent {
            unit_tag_index: source.m_unit_tag_index,
            unit_tag_recycle: source.m_unit_tag_recycle,
            killer_player_id: source.m_killer_player_id,
            x: source.m_x,
            y: source.m_y,
            killer_unit_tag_index: source.m_killer_unit_tag_index,
            killer_unit_tag_recycle: source.m_killer_unit_tag_recycle,
        })
    }
}

impl TryFrom<ReplayTrackerSUnitTypeChangeEvent> for ReplayTrackerEvent {
    type Error = TrackerEventError;
    fn try_from(source: ReplayTrackerSUnitTypeChangeEvent) -> Result<Self, Self::Error> {
        Ok(ReplayTrackerEvent::UnitTypeChange(UnitTypeChangeEvent {
            unit_tag_index: source.m_unit_tag_index,
            unit_tag_recycle: source.m_unit_tag_recycle,
            unit_type_name: str::from_utf8(&source.m_unit_type_name)?.to_string(),
        }))
    }
}

impl TryFrom<ReplayTrackerSUnitInitEvent> for ReplayTrackerEvent {
    type Error = TrackerEventError;
    fn try_from(source: ReplayTrackerSUnitInitEvent) -> Result<Self, Self::Error> {
        Ok(ReplayTrackerEvent::UnitInit(UnitInitEvent {
            unit_tag_index: source.m_unit_tag_index,
            unit_tag_recycle: source.m_unit_tag_recycle,
            unit_type_name: str::from_utf8(&source.m_unit_type_name)?.to_string(),
            control_player_id: source.m_control_player_id,
            upkeep_player_id: source.m_upkeep_player_id,
            x: source.m_x,
            y: source.m_y,
        }))
    }
}

impl From<ReplayTrackerSUnitDoneEvent> for ReplayTrackerEvent {
    fn from(source: ReplayTrackerSUnitDoneEvent) -> ReplayTrackerEvent {
        ReplayTrackerEvent::UnitDone(UnitDoneEvent {
            unit_tag_index: source.m_unit_tag_index,
            unit_tag_recycle: source.m_unit_tag_recycle,
        })
    }
}

impl From<ReplayTrackerSUnitPositionsEvent> for ReplayTrackerEvent {
    fn from(source: ReplayTrackerSUnitPositionsEvent) -> ReplayTrackerEvent {
        ReplayTrackerEvent::UnitPosition(UnitPositionsEvent {
            first_unit_index: source.m_first_unit_index,
            items: source.m_items,
        })
    }
}