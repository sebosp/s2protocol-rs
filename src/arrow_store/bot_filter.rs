//! Tells apart the bots, i.e. the ones of AI Arena, by how fast they give commands, and drops their
//! move and attack commands from the snapshot.
//!
//! A human gives a few hundred commands per minute, the pros about 250 to 400. A bot gives
//! thousands, i.e. 1,100 to 16,800 in the AI Arena games, mostly moving and attacking with every
//! unit at every step. Each command is a row per unit selected in the cmd files, a bot game has
//! about 30,000 commands and 47,000 rows, eight times the rows of a human game, and 88% of them
//! are move and attack commands. The other commands of the bots are kept: builds, trains,
//! harvests, abilities.

use crate::game_events::GameSCmdAbil;
use std::collections::HashMap;

/// The commands of a user are measured in windows of this many, the bots command slowly at the
/// start of a game and faster as their army grows. The rows before a user is a bot are kept.
pub const BOT_DETECTION_COMMANDS: u32 = 1000;

/// A user commanding faster than this during a window is a bot for the rest of the game.
pub const BOT_COMMANDS_PER_MINUTE: f64 = 1000.;

/// The game loops per minute at Faster speed.
const GAME_LOOPS_PER_MINUTE: f64 = crate::GAME_LOOPS_PER_SECOND * 60.;

/// The abilities dropped for the bots, by the start of their name, i.e. `move.Move` or
/// `attack.Execute`.
const DROPPED_ABILITIES: [&str; 2] = ["move", "attack"];

/// The links of the move and attack abilities in protocol 75689 (4.10), for the commands without
/// a name, i.e. with balance data without 4.10. They are the ones of 94137 (43 and 46) minus one,
/// the bots of AI Arena command their workers with 42 and their army with 45.
const DROPPED_ABILITY_LINKS_75689: [u16; 2] = [42, 45];

#[derive(Debug, Default, Clone, Copy)]
struct UserCommands {
    /// The commands of the current window.
    count: u32,
    /// The loop of the first command of the current window.
    first_loop: i64,
    is_bot: bool,
}

/// Counts the commands of each user of a replay and keeps the ones that are needed.
#[derive(Debug, Default)]
pub struct BotFilter {
    protocol_version: u32,
    users: HashMap<i64, UserCommands>,
}

impl BotFilter {
    pub fn new(protocol_version: u32) -> Self {
        Self {
            protocol_version,
            ..Default::default()
        }
    }

    /// Counts the command of the user, the user is a bot once [`BOT_DETECTION_COMMANDS`] of its
    /// commands in a row came faster than [`BOT_COMMANDS_PER_MINUTE`].
    pub fn count(&mut self, user_id: i64, game_loop: i64) {
        let user = self.users.entry(user_id).or_insert(UserCommands {
            first_loop: game_loop,
            ..Default::default()
        });
        if user.is_bot {
            return;
        }
        user.count += 1;
        if user.count >= BOT_DETECTION_COMMANDS {
            let minutes =
                ((game_loop - user.first_loop) as f64 / GAME_LOOPS_PER_MINUTE).max(1. / 60.);
            let per_minute = user.count as f64 / minutes;
            if per_minute > BOT_COMMANDS_PER_MINUTE {
                user.is_bot = true;
                tracing::info!("User {user_id} is a bot, {per_minute:.0} commands per minute");
            }
            // The next window.
            user.count = 0;
            user.first_loop = game_loop;
        }
    }

    pub fn is_bot(&self, user_id: i64) -> bool {
        self.users.get(&user_id).is_some_and(|user| user.is_bot)
    }

    /// Whether the rows of the command are kept: all the commands of the humans, and the ones of
    /// the bots that are not a move or an attack.
    pub fn keeps(&self, user_id: i64, ability: Option<&GameSCmdAbil>) -> bool {
        if !self.is_bot(user_id) {
            return true;
        }
        let Some(ability) = ability else {
            return true;
        };
        let is_dropped = if ability.ability.is_empty() && self.protocol_version == 75689 {
            DROPPED_ABILITY_LINKS_75689.contains(&ability.m_abil_link)
        } else {
            DROPPED_ABILITIES
                .iter()
                .any(|name| ability.ability.split('.').next() == Some(name))
        };
        !is_dropped
    }

    /// The users detected as bots.
    pub fn bots(&self) -> Vec<i64> {
        let mut bots: Vec<i64> = self
            .users
            .iter()
            .filter(|(_, user)| user.is_bot)
            .map(|(user_id, _)| *user_id)
            .collect();
        bots.sort();
        bots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ability(name: &str, link: u16) -> GameSCmdAbil {
        GameSCmdAbil {
            m_abil_link: link,
            ability: name.to_string(),
            m_abil_cmd_index: 0,
            m_abil_cmd_data: None,
        }
    }

    #[test]
    fn it_tells_the_bots_apart() {
        let mut filter = BotFilter::new(97563);
        // A human, 300 commands per minute: 1000 commands in 3:20.
        for idx in 0..1000 {
            filter.count(0, idx * 4480 / 1000);
        }
        // A bot, 3000 commands per minute: 1000 commands in 20 seconds.
        for idx in 0..1000 {
            filter.count(1, idx * 448 / 1000);
        }
        assert!(!filter.is_bot(0));
        assert!(filter.is_bot(1));
        assert_eq!(filter.bots(), vec![1]);
        assert!(filter.keeps(0, Some(&ability("move.Move", 43))));
        assert!(!filter.keeps(1, Some(&ability("move.Move", 43))));
        assert!(!filter.keeps(1, Some(&ability("attack.Execute", 46))));
        assert!(filter.keeps(1, Some(&ability("Build.Barracks", 100))));
        assert!(filter.keeps(1, Some(&ability("", 42))));
    }

    #[test]
    fn it_drops_the_unnamed_moves_of_4_10() {
        let mut filter = BotFilter::new(75689);
        for idx in 0..1000 {
            filter.count(0, idx / 10);
        }
        assert!(!filter.keeps(0, Some(&ability("", 42))));
        assert!(!filter.keeps(0, Some(&ability("", 45))));
        assert!(filter.keeps(0, Some(&ability("", 120))));
    }

    #[test]
    fn it_finds_the_bots_that_speed_up() {
        let mut filter = BotFilter::new(97563);
        // 300 commands per minute for 10 minutes, then 3000 per minute.
        let mut game_loop = 0;
        for _ in 0..3000 {
            game_loop += 4;
            filter.count(0, game_loop);
        }
        assert!(!filter.is_bot(0));
        for _ in 0..1000 {
            game_loop += 1;
            filter.count(0, game_loop);
        }
        assert!(filter.is_bot(0));
    }

    #[test]
    fn it_keeps_the_commands_before_deciding() {
        let mut filter = BotFilter::new(75689);
        for idx in 0..999 {
            filter.count(0, idx / 10);
        }
        assert!(filter.keeps(0, Some(&ability("", 42))));
    }
}
