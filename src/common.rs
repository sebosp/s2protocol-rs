//! A collection of common types used across different modules

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum GameSpeed {
    ESlower,
    ESlow,
    ENormal,
    EFast,
    EFaster,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum EObserve {
    ENone,
    ESpectator,
    EReferee,
}
