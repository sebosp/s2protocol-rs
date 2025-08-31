use super::bit_packed::*;
use crate::S2ProtocolError;
use crate::message_events::{MessageEvent, MessageEventError, ReplayMessageEvent};
use crate::*;
use nom::*;
use nom_mpq::MPQ;

impl GameEMessageId {
    /// Reads a delta, GameMessage set
    #[tracing::instrument(name="GameEMessageId::parse_events", level = "debug", skip(input), fields(peek = peek_bits(input)))]
    pub fn parse_event_triplet(
        input: (&[u8], usize),
    ) -> S2ProtoResult<(&[u8], usize), (i64, i64, GameEMessageId)> {
        let (tail, delta) = SVarUint32::parse(input)?;
        let (tail, user_id) = ReplaySGameUserId::parse(tail)?;
        let (tail, event) = GameEMessageId::parse(tail)?;
        let delta = match delta {
            SVarUint32::MUint6(val) => val.value,
            SVarUint32::MUint14(val) => val.value,
            SVarUint32::MUint22(val) => val.value,
            SVarUint32::MUint32(val) => val.value,
        };
        // The next event is byte aligned
        let (tail, _) = byte_align(tail)?;
        Ok((tail, (delta, user_id.m_user_id, event)))
    }

    /// Read the Message Events
    pub fn read_events(
        mpq: &MPQ,
        file_contents: &[u8],
    ) -> Result<Vec<MessageEvent>, S2ProtocolError> {
        // TODO: Make it return an Iterator
        let (_event_tail, game_events) =
            mpq.read_mpq_file_sector("replay.message.events", false, file_contents)?;
        let mut res = vec![];
        let mut count = 1usize;
        let mut event_tail: (&[u8], usize) = (&game_events, 0usize);
        loop {
            tracing::debug!("-----------------------------------------------");
            tracing::debug!("Event number: {}", count);
            let (new_event_tail, (delta, user_id, event)) = Self::parse_event_triplet(event_tail)?;
            count += 1;
            event_tail = new_event_tail;
            match event.try_into() {
                Ok(val) => res.push(MessageEvent {
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
        Ok(res)
    }
}

impl TryFrom<GameEMessageId> for ReplayMessageEvent {
    type Error = MessageEventError;
    fn try_from(value: GameEMessageId) -> Result<Self, Self::Error> {
        match value {
            GameEMessageId::EChat(e) => Ok(e.try_into()?),
            _ => Err(MessageEventError::UnsupportedEventType),
        }
    }
}

impl TryFrom<GameSChatMessage> for ReplayMessageEvent {
    type Error = MessageEventError;
    fn try_from(source: GameSChatMessage) -> Result<Self, Self::Error> {
        Ok(message_events::ReplayMessageEvent::EChat(
            message_events::ChatMessage {
                m_recipient: source.m_recipient.into(),
                m_string: str::from_utf8(&source.m_string.value)?.to_string(),
            },
        ))
    }
}

impl From<GameEMessageRecipient> for message_events::GameEMessageRecipient {
    fn from(source: GameEMessageRecipient) -> message_events::GameEMessageRecipient {
        match source {
            GameEMessageRecipient::EAll => message_events::GameEMessageRecipient::EAll,
            GameEMessageRecipient::EAllies => message_events::GameEMessageRecipient::EAllies,
            GameEMessageRecipient::EIndividual => {
                message_events::GameEMessageRecipient::EIndividual
            }
            GameEMessageRecipient::EBattlenet => message_events::GameEMessageRecipient::EBattlenet,
            GameEMessageRecipient::EObservers => message_events::GameEMessageRecipient::EObservers,
        }
    }
}
