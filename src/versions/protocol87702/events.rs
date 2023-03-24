use super::bit_packed::*;
use crate::game_events::GameEvent;
use crate::peek_bits;
use nom::*;
use nom_mpq::MPQ;

impl GameEEventId {
    /// Reads a delta, GameEvent set
    #[tracing::instrument(name="GameEvents::parse_events", level = "debug", skip(input), fields(peek = peek_bits(input)))]
    pub fn parse_event_triplet(
        input: (&[u8], usize),
    ) -> IResult<(&[u8], usize), (i64, i64, GameEEventId)> {
        let (tail, delta) = SVarUint32::parse(input)?;
        let (tail, user_id) = TUserId::parse(tail)?;
        let (tail, event) = GameEEventId::parse(tail)?;
        let delta = match delta {
            SVarUint32::MUint6(val) => val.value,
            SVarUint32::MUint14(val) => val.value,
            SVarUint32::MUint22(val) => val.value,
            SVarUint32::MUint32(val) => val.value,
        };
        Ok((tail, (delta, user_id.value, event)))
    }

    /// Read the Tracker Events
    pub fn read_events(mpq: &MPQ, file_contents: &[u8]) -> Vec<GameEvent> {
        // TODO: Make it return an Iterator.
        let (_event_tail, game_events) = mpq
            .read_mpq_file_sector("replay.game.events", false, &file_contents)
            .unwrap();
        let mut res = vec![];
        let mut event_tail: (&[u8], usize) = (&game_events, 0usize);
        loop {
            let (new_event_tail, (delta, user_id, event)) =
                Self::parse_event_triplet(event_tail).expect("Unable to parse GameEvents");
            event_tail = new_event_tail;
            match event.try_into() {
                Ok(val) => res.push(GameEvent {
                    delta,
                    user_id,
                    event: val,
                }),
                Err(err) => {
                    tracing::debug!("Skipping event: {:?}", err);
                }
            };
            if event_tail.0.input_len() == 0 {
                break;
            }
        }
        res
    }
}
