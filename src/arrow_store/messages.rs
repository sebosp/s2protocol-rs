//! The chat messages of the games, a row each in [`super::MESSAGES_ARROW_FNAME`].

use crate::message_events::{GameEMessageRecipient, ReplayMessageEvent};
use arrow::datatypes::Schema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A chat message of a game.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageFlatRow {
    pub ext_fs_id: u64,
    pub user_id: i64,
    /// The player, or the observer, who wrote the message.
    pub player_name: Option<String>,
    /// `All`, `Allies`, `Individual`, `Battlenet` or `Observers`.
    pub recipient: String,
    pub text: String,
    pub ext_replay_loop: i64,
    pub ext_replay_seconds: u32,
}

impl MessageFlatRow {
    pub fn schema() -> Schema {
        <Self as crate::arrow_store::rows::ArrowRow>::schema()
    }
}

crate::impl_arrow_row!(MessageFlatRow);

/// The chat messages of the replay file, `names` are the names of the users by user id. A replay
/// whose messages cannot be read has none, its other events are still written.
pub fn message_rows(
    file_name: &str,
    ext_fs_id: u64,
    names: &HashMap<i64, String>,
) -> Vec<MessageFlatRow> {
    let read = || -> Result<Vec<crate::message_events::MessageEvent>, crate::S2ProtocolError> {
        let contents = crate::read_file(&std::path::PathBuf::from(file_name))?;
        let (_, mpq) = crate::parser::parse(&contents)?;
        crate::read_message_events(file_name, &mpq, &contents)
    };
    let events = match read() {
        Ok(events) => events,
        Err(err) => {
            tracing::warn!("Unable to read the messages of {file_name}: {err:?}");
            return vec![];
        }
    };
    // The message events store the loops since the previous message.
    let mut game_loop = 0;
    let mut rows = vec![];
    for message in events {
        game_loop += message.delta;
        let ReplayMessageEvent::EChat(chat) = message.event;
        rows.push(MessageFlatRow {
            ext_fs_id,
            user_id: message.user_id,
            player_name: names.get(&message.user_id).cloned(),
            recipient: match chat.m_recipient {
                GameEMessageRecipient::EAll => "All",
                GameEMessageRecipient::EAllies => "Allies",
                GameEMessageRecipient::EIndividual => "Individual",
                GameEMessageRecipient::EBattlenet => "Battlenet",
                GameEMessageRecipient::EObservers => "Observers",
            }
            .to_string(),
            text: chat.m_string,
            ext_replay_loop: game_loop,
            ext_replay_seconds: crate::convert_game_loop_to_seconds(game_loop),
        });
    }
    rows
}
