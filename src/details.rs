//! Decodes the Details.

use serde::{Deserialize, Serialize};
use std::str::Utf8Error;

/// A list of errors when handling MessageEvents
#[derive(Debug, thiserror::Error)]
pub enum DetailsError {
    /// Conversion to UTF-8 failed, from the Vec<u8> _name fields in the proto fields
    #[error("Utf8 conversion error")]
    Utf8Error(#[from] Utf8Error),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Details {
    pub player_list: Vec<PlayerDetails>,
    pub title: String,
    pub difficulty: String,
    pub thumbnail: Thumbnail,
    pub is_blizzard_map: bool,
    pub time_utc: i64,
    pub time_local_offset: i64,
    pub restart_as_transition_map: Option<bool>,
    pub disable_recover_game: bool,
    pub description: String,
    pub image_file_path: String,
    pub campaign_index: u8,
    pub map_file_name: String,
    pub cache_handles: Vec<String>,
    pub mini_save: bool,
    pub game_speed: GameSpeed,
    pub default_difficulty: u32,
    pub mod_paths: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlayerDetails {
    pub name: String,
    pub toon: ToonNameDetails,
    pub race: String,
    pub color: Color,
    pub control: u8,
    pub team_id: u8,
    pub handicap: u32,
    pub observe: EObserve,
    pub result: ResultDetails,
    pub working_set_slot_id: Option<u8>,
    pub hero: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ToonNameDetails {
    pub region: u8,
    pub program_id: u32,
    pub realm: u32,
    pub id: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum EObserve {
    ENone,
    ESpectator,
    EReferee,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ResultDetails {
    EUndecided,
    EWin,
    ELoss,
    ETie,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub file: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum GameSpeed {
    ESlower,
    ESlow,
    ENormal,
    EFast,
    EFaster,
}