//! Contains the code to generate from version-specific to versionless.
//! These should become templates and probably won't w ork for many protocols.Contains

use std::fs::File;
use std::io::prelude::*;
/// Generates the protocol-agnostic version of the TrackerEvents
/// This may fail in new/older protocols, and the generated code should be
/// validated, luckily at compile time, it is expected after generation
/// `cargo check` is run and code adapted if needed.
#[tracing::instrument(level = "debug", skip(output,))]
pub fn generate_replay_tracker_convert_into_versionless(output: &mut File) -> std::io::Result<()> {
    output.write_all(r#"
// Translate to the protocol agnostic version of tracker events.

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
            ReplayTrackerEEventId::EUnitBorn(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitDied(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitTypeChange(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitInit(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitDone(e) => Ok(e.to_versionless()?),
            ReplayTrackerEEventId::EUnitPosition(e) => Ok(e.to_versionless()?),
        }
    }
}

impl ReplayTrackerSUnitBornEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        let creator_ability_name = if let Some(val) = self.m_creator_ability_name {
            Some(str::from_utf8(&val)?.to_string())
        } else {
            None
        };
        Ok(ReplayTrackerEvent::UnitBorn(UnitBornEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
            unit_type_name: str::from_utf8(&self.m_unit_type_name)?.to_string(),
            control_player_id: self.m_control_player_id,
            upkeep_player_id: self.m_upkeep_player_id,
            x: self.m_x,
            y: self.m_y,
            creator_unit_tag_index: self.m_creator_unit_tag_index,
            creator_unit_tag_recycle: self.m_creator_unit_tag_recycle,
            creator_ability_name,
        }))
    }
}

impl ReplayTrackerSUnitDiedEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitDied(UnitDiedEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
            killer_player_id: self.m_killer_player_id,
            x: self.m_x,
            y: self.m_y,
            killer_unit_tag_index: self.m_killer_unit_tag_index,
            killer_unit_tag_recycle: self.m_killer_unit_tag_recycle,
        }))
    }
}

impl ReplayTrackerSUnitTypeChangeEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitTypeChange(UnitTypeChangeEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
            unit_type_name: str::from_utf8(&self.m_unit_type_name)?.to_string(),
        }))
    }
}

impl ReplayTrackerSUnitInitEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitInit(UnitInitEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
            unit_type_name: str::from_utf8(&self.m_unit_type_name)?.to_string(),
            control_player_id: self.m_control_player_id,
            upkeep_player_id: self.m_upkeep_player_id,
            x: self.m_x,
            y: self.m_y,
        }))
    }
}

impl ReplayTrackerSUnitDoneEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitDone(UnitDoneEvent {
            unit_tag_index: self.m_unit_tag_index,
            unit_tag_recycle: self.m_unit_tag_recycle,
        }))
    }
}

impl ReplayTrackerSUnitPositionsEvent {
    pub fn to_versionless(self) -> Result<ReplayTrackerEvent, TrackerEventError> {
        Ok(ReplayTrackerEvent::UnitPosition(UnitPositionsEvent {
            first_unit_index: self.m_first_unit_index,
            items: self.m_items,
        }))
    }
}
    "#.as_bytes())?;
    Ok(())
}

/// Generates the protocol-agnostic version of the TrackerEvents
/// This may fail in new/older protocols, and the generated code should be
/// validated, luckily at compile time, it is expected after generation
/// `cargo check` is run and code adapted if needed.
#[tracing::instrument(level = "debug", skip(output,))]
pub fn generate_game_events_convert_into_versionless(output: &mut File) -> std::io::Result<()> {
    output.write_all(
        r#"
    // TODO:
    "#
        .as_bytes(),
    )?;
    Ok(())
}
