//! Transforms an ability ID into a string.
//! TODO: Create arrow table.
//!
use serde::{Deserialize, Serialize};
use tracing;

pub mod xml_reader;
pub use xml_reader::*;

//<meta name="27" icon="btn-unit-zerg-zergling" race="Zerg" hotkey="1103" tooltip="203" source="Liberty.SC2Mod" index="128"/>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitMeta {
    pub icon: String,
    pub race: String,
}

impl UnitMeta {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/meta" {
            tracing::debug!("Unexpected path for UnitMeta: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "icon" => self.icon = attr.value,
                "race" => self.race = attr.value,
                "name" | "hotkey" | "tooltip" | "source" | "index" => {
                    // Unused/unknown not used for now
                    // name is a numeric "id" of sorts.
                    // When a unit references another unit, it uses both the "id" which is the unit
                    // name and its "index" which is a numeric identifier.
                    // Zergling meta is 27.
                    // Stalker meta is 128.
                    // Zergling.strengths.Stalker points to name 128.
                }
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitLife {
    pub start: u32,
    pub max: u32,
    pub regen: f32,
}

impl UnitLife {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/life" {
            tracing::debug!("Unexpected path for UnitLife: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "start" => self.start = attr.value.parse().unwrap_or_default(),
                "max" => self.max = attr.value.parse().unwrap_or_default(),
                "regenRate" => self.regen = attr.value.parse().unwrap_or_default(),
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitArmor {
    pub start: u32,
    pub max: u32,
}

impl UnitArmor {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/armor" {
            tracing::debug!("Unexpected path for UnitArmor: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "start" => self.start = attr.value.parse().unwrap_or_default(),
                "max" => self.max = attr.value.parse().unwrap_or_default(),
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitShields {
    pub start: u32,
    pub max: u32,
    pub regen_rate: f32,
    pub delay: f32,
}

impl UnitShields {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/shields" {
            tracing::debug!("Unexpected path for UnitShields: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "start" => self.start = attr.value.parse().unwrap_or_default(),
                "max" => self.max = attr.value.parse().unwrap_or_default(),
                "regenRate" => self.regen_rate = attr.value.parse().unwrap_or_default(),
                "delay" => self.delay = attr.value.parse().unwrap_or_default(),
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitShieldArmor {
    pub start: u32,
    pub max: u32,
}

impl UnitShieldArmor {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/shieldArmor" {
            tracing::debug!("Unexpected path for UnitShieldArmor: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "start" => self.start = attr.value.parse().unwrap_or_default(),
                "max" => self.max = attr.value.parse().unwrap_or_default(),
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitCost {
    pub minerals: u32,
    pub vespene: u32,
    pub supply: u32,
    pub time: u32,
    pub cooldown: u32,
}

impl UnitCost {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/cost" {
            tracing::debug!("Unexpected path for UnitCost: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "minerals" => self.minerals = attr.value.parse().unwrap_or_default(),
                "vespene" => self.vespene = attr.value.parse().unwrap_or_default(),
                "supply" => self.supply = attr.value.parse().unwrap_or_default(),
                "time" => self.time = attr.value.parse().unwrap_or_default(),
                "cooldown" => self.cooldown = attr.value.parse().unwrap_or_default(),
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

// Unit references contain an "id" string attribute inside.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NamedIdRef {
    pub id: String,
}

impl NamedIdRef {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "id" => self.id = attr.value,
                "name" => {
                    // This is a "name" that is numeric, not sure what it pertains to.
                }
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

// <movement type="Ground" speed="2.9531" acceleration="1000" deceleration="0" turnRate="999.8437"/>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitMovement {
    pub kind: String,
    pub speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub turn_rate: f32,
}

impl UnitMovement {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/movement" {
            tracing::debug!("Unexpected path for UnitMovement: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "type" => self.kind = attr.value,
                "speed" => self.speed = attr.value.parse().unwrap_or_default(),
                "acceleration" => self.acceleration = attr.value.parse().unwrap_or_default(),
                "deceleration" => self.deceleration = attr.value.parse().unwrap_or_default(),
                "turnRate" => self.turn_rate = attr.value.parse().unwrap_or_default(),
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

/// A unit score for building and gained score for killing such unit.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitScore {
    pub build: u32,
    pub kill: u32,
}

impl UnitScore {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/score" {
            tracing::debug!("Unexpected path for UnitScore: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "build" => self.build = attr.value.parse().unwrap_or_default(),
                "kill" => self.kill = attr.value.parse().unwrap_or_default(),
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

/// A unit score for building and gained score for killing such unit.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitMiscValues {
    pub radius: f32,
    pub cargo_size: u32,
    pub sight_radius: u32,
    pub supply: f32,
    pub footprint: Option<String>,
}

impl UnitMiscValues {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/misc" {
            tracing::debug!("Unexpected path for UnitScore: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "radius" => self.radius = attr.value.parse().unwrap_or_default(),
                "cargoSize" => self.cargo_size = attr.value.parse().unwrap_or_default(),
                "sightRadius" => self.sight_radius = attr.value.parse().unwrap_or_default(),
                "supply" => self.supply = attr.value.parse().unwrap_or_default(),
                "footprint" => self.footprint = Some(attr.value),
                _ => {
                    tracing::debug!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitAbilityCommandMeta {
    pub icon: String,
    pub kind: String,
}

impl UnitAbilityCommandMeta {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/abilities/ability/command/meta" {
            tracing::trace!("Unexpected path for UnitAbilityCommandMeta: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "icon" => self.icon = attr.value,
                "type" => self.kind = attr.value,
                _ => {
                    tracing::trace!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitAbilityCommand {
    pub id: String,
    pub index: i64,
    pub requires: Option<NamedIdRef>,
    pub cost: Option<UnitCost>,
    pub meta: UnitAbilityCommandMeta,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitAbility {
    pub id: String,
    pub index: u16,
    pub cmds: Vec<UnitAbilityCommand>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionedBalanceUnit {
    /// A unique identifier for the unit, typically a string like "Zergling" or "Stalker".
    pub id: String,
    /// The meta information for the unit, such as icon, "unit id", etc
    pub meta: UnitMeta,
    /// The life of the unit
    pub life: UnitLife,
    /// The armor of the unit
    pub armor: UnitArmor,
    /// The shield armor of the unit
    pub shield_armor: Option<UnitShieldArmor>,
    /// The shields of the unit, with regeneration and delay.
    pub shields: Option<UnitShields>,
    /// The requirements for the unit to be created.
    pub requires: Vec<NamedIdRef>,
    /// The movement speed of the unit, some units such as Nexus have no movement speed.
    pub movement: Option<UnitMovement>,
    /// A score for the unit, with "build score" and "kill score"
    pub score: UnitScore,
    /// Miscellaneous values for the unit, such as radius, cargoSize, supply requirements.
    pub misc: UnitMiscValues,
    /// The upgrades available for the unit, such as "ZergMeleeWeapons"
    pub upgrades: Vec<String>,
    /// The cost of the unit, in minerals, vespene, supply, time and cooldown.
    pub cost: UnitCost,
    /// The producer of the unit.
    pub producer: Option<NamedIdRef>,
    /// The attributes of the unit
    pub attributes: Vec<String>,
    /// The unit is strong vs these types of units
    pub strengths: Vec<NamedIdRef>,
    /// The unit is weak vs these types of units
    pub weaknesses: Vec<NamedIdRef>,
    /// The abilities of the unit, such as "ZerglingAttack" or "StalkerBlink"
    pub abilities: Vec<UnitAbility>,
    /// The state of the current XML parsing, it may be the active ability, weaponn, etc.
    pub state: VersionedBalanceUnitState,
}

impl VersionedBalanceUnit {
    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path == "/unit" {
            for attr in attributes {
                match attr.name.local_name.as_str() {
                    "id" => self.id = attr.value,
                    _ => {
                        tracing::debug!(
                            "{path} Unknown attribute: {} with value: {}",
                            attr.name.local_name,
                            attr.value
                        );
                    }
                }
            }
        } else if path == "/unit/requires/unit" {
            let mut requires = NamedIdRef::default();
            requires.from_owned_attributes(path, attributes);
            self.requires.push(requires);
        } else if path == "/unit/strengths/unit" {
            let mut strength = NamedIdRef::default();
            strength.from_owned_attributes(path, attributes);
            self.strengths.push(strength);
        } else if path == "/unit/weaknesses/unit" {
            let mut weakness = NamedIdRef::default();
            weakness.from_owned_attributes(path, attributes);
            self.weaknesses.push(weakness);
        } else {
            tracing::debug!("Unknown path: {}", path);
        }
    }

    pub fn from_xml_start_element(
        &mut self,
        name: &str,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        match name {
            "unit" => {
                self.from_owned_attributes(path, attributes);
            }
            "meta" => {
                if path == "/unit/meta" {
                    self.meta.from_owned_attributes(path, attributes);
                } else if path == "/unit/abilities/ability/command/meta" {
                    self.state
                        .ability
                        .current_cmd
                        .meta
                        .from_owned_attributes(path, attributes);
                }
            }
            "life" => {
                self.life.from_owned_attributes(path, attributes);
            }
            "armor" => {
                self.armor.from_owned_attributes(path, attributes);
            }
            "shieldArmor" => {
                let mut shield_armor = UnitShieldArmor::default();
                shield_armor.from_owned_attributes(path, attributes);
                self.shield_armor = Some(shield_armor);
            }
            "shields" => {
                let mut shields = UnitShields::default();
                shields.from_owned_attributes(path, attributes);
                self.shields = Some(shields);
            }
            "cost" => {
                self.cost.from_owned_attributes(path, attributes);
            }
            "movement" => {
                let mut movement = UnitMovement::default();
                movement.from_owned_attributes(path, attributes);
                self.movement = Some(movement);
            }
            "score" => {
                self.score.from_owned_attributes(path, attributes);
            }
            "misc" => {
                self.misc.from_owned_attributes(path, attributes);
            }
            "producer" => {
                let mut producer = NamedIdRef::default();
                producer.from_owned_attributes(path, attributes);
                self.producer = Some(producer);
            }
            "attribute" => {
                if path == "/unit/attributes/attribute" {
                    if let Some(attr) = attributes.first() {
                        self.attributes.push(attr.value.clone());
                    } else {
                        tracing::debug!("No attribute value found in /unit/attributes");
                    }
                } else {
                    tracing::debug!("Unexpected path for attribute: {}", path);
                }
            }
            "command" => {
                self.state
                    .ability
                    .current_cmd
                    .from_owned_attributes(path, attributes);
            }
            "ability" => {
                self.state.ability.from_owned_attributes(path, attributes);
            }
            "attributes" | "requires" | "strengths" | "weaknesses" | "weapons" | "weapon"
            | "effect" | "abilities" => {
                // NOTE: Most are unknown so far.
                // TODO: hanndle "weapeons.weapon.effect"
            }
            _ => {
                tracing::debug!("Unknown start element: {}", name);
            }
        }
    }
}
