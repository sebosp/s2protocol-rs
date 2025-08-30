use serde::{Deserialize, Serialize};
use serde_xml_rs;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing;

pub mod json_handler;
pub use json_handler::*;

// when self.cmds[0].meta.kind == "passive" we need to return None.

pub fn traverse_versioned_balance_abilities(
    root_dir: impl Into<PathBuf>,
) -> std::io::Result<HashMap<(u32, String), VersionedBalanceUnit>> {
    let mut res: HashMap<(u32, String), VersionedBalanceUnit> = HashMap::new();
    let root_dir: PathBuf = root_dir.into();
    tracing::debug!(
        "Traversing versioned balance abilities in {}",
        root_dir.display()
    );
    if !root_dir.is_dir() {
        return Ok(res);
    }
    for versioned_dir in std::fs::read_dir(root_dir)? {
        let versioned_dir = versioned_dir?.path();
        if !versioned_dir.is_dir() {
            continue;
        }
        let versioned_balance_data_dir = versioned_dir.join("BalanceData");
        if !versioned_balance_data_dir.is_dir() {
            continue;
        }
        let protocol_version: u32 = match versioned_dir
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|s| s.parse().ok())
        {
            Some(v) => v,
            None => {
                tracing::warn!(
                    "Skipping versioned directory {:?} as it does not contain a number",
                    versioned_dir
                );
                continue;
            }
        };
        tracing::debug!(
            "Traversing versioned balance data directory: {:?}",
            versioned_balance_data_dir
        );
        for entry in std::fs::read_dir(versioned_balance_data_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension() {
                if ext == "xml" && path.is_file() {
                    tracing::debug!("Processing XML file: {:?}", path);
                    let files_content = std::fs::read_to_string(&path)?;
                    let unit = match serde_xml_rs::from_str::<VersionedBalanceUnit>(&files_content)
                    {
                        Ok(val) => val,
                        Err(err) => {
                            tracing::error!("Failed to parse XML file {:?}: {}", path, err);
                            continue;
                        }
                    };
                    res.insert((protocol_version, unit.id.clone()), unit);
                }
            }
        }
    }
    tracing::info!("Processed {} total units", res.len(),);

    Ok(res)
}

/// The Meta has the name and the tooltip required.
/// <meta name="710" icon="btn-building-protoss-stargate" race="Prot" hotkey="734" tooltip="735" source="Liberty.SC2Mod" index="90"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@tooltip")]
    pub tooltip: String,
    #[serde(rename = "@icon")]
    pub icon: Option<String>,
    #[serde(rename = "@race")]
    pub race: Option<String>,
    #[serde(rename = "@hotkey")]
    pub hotkey: Option<String>,
    #[serde(rename = "@source")]
    pub source: Option<String>,
    #[serde(rename = "@index")]
    pub index: Option<u32>,
    #[serde(rename = "@type")]
    pub kind: Option<String>,
}

/// The Life of a unit.
/// <life start="600" max="600"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Life {
    #[serde(rename = "@start")]
    pub start: u32,
    #[serde(rename = "@max")]
    pub max: u32,
    #[serde(rename = "@regenRate")]
    pub regen_rate: Option<f32>,
}

/// The Shields of a unit.
/// <shields start="600" max="600" regenRate="2" delay="10"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shields {
    #[serde(rename = "@start")]
    pub start: u32,
    #[serde(rename = "@max")]
    pub max: u32,
    #[serde(rename = "@regenRate")]
    pub regen_rate: f32,
    #[serde(rename = "@delay", default)]
    pub delay: Option<f32>,
}

/// The Armor of a unit.
/// <armor start="1" max="1"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Armor {
    #[serde(rename = "@start")]
    pub start: u32,
    #[serde(rename = "@max")]
    pub max: u32,
}

/// A short reference to a unit, rather than a full VersionedBalanceUnit reference (because
/// recursive ugh.)
/// <unit id="CyberneticsCore" name="679"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "unit")]
pub struct UnitShortRef {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
}

/// A longer reference to a unit, rather than a full VersionedBalanceUnit reference (because
/// recursive ugh.)
/// <unit id="CyberneticsCore" name="679">
/// <meta name="736" icon="btn-unit-protoss-phoenix" hotkey="737" type="command" tooltip="738"/>
/// <cost minerals="150" vespene="100" supply="2" time="35"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "unit")]
pub struct UnitRef {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    // This may be a reference to a unit that is being trained by an ability "id"
    #[serde(rename = "@ability")]
    pub ability_id: Option<u16>,
    // This may be a reference to a command index within the ability
    #[serde(rename = "@index")]
    pub command_index: Option<i64>,
    pub meta: Option<Meta>,
    pub cost: Option<Cost>,
    #[serde(default)]
    pub requires: RequiresTag,
}

/// The Cost of a unit.
/// <cost minerals="150" vespene="150" time="60"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cost {
    #[serde(rename = "@minerals")]
    pub minerals: Option<u32>,
    #[serde(rename = "@vespene")]
    pub vespene: Option<u32>,
    #[serde(rename = "@supply")]
    pub supply: Option<f32>,
    #[serde(rename = "@time")]
    pub time: Option<f32>,
    #[serde(rename = "@energy")]
    pub energy: Option<u32>,
    #[serde(rename = "@cooldown")]
    pub cooldown: Option<f32>,
    #[serde(rename = "@life")]
    pub life: Option<u32>,
}

/// The Movement of a unit.
/// <movement type="Ground" speed="2.9531" acceleration="1000" deceleration="0" turnRate="999.8437"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movement {
    #[serde(rename = "@type")]
    pub kind: String,
    #[serde(rename = "@speed")]
    pub speed: f32,
    #[serde(rename = "@acceleration")]
    pub acceleration: f32,
    #[serde(rename = "@deceleration")]
    pub deceleration: f32,
    #[serde(rename = "@turnRate", default)]
    pub turn_rate: Option<f32>,
}

/// The ShieldArmor of a unit.
/// <shieldArmor start="0" max="3"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "shieldArmor")]
pub struct ShieldArmor {
    #[serde(rename = "@start")]
    pub start: u32,
    #[serde(rename = "@max")]
    pub max: u32,
}

/// The Score of a unit.
/// <score build="300" kill="300"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    #[serde(rename = "@build")]
    pub build: Option<u32>,
    #[serde(rename = "@kill")]
    pub kill: Option<u32>,
}

/// Miscellaneous properties of a unit.
/// <misc radius="1.75" cargoSize="0" footprint="Footprint3x3Contour" sightRadius="9" supply="0"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Misc {
    #[serde(rename = "@radius")]
    pub radius: Option<f32>,
    #[serde(rename = "@cargoSize")]
    pub cargo_size: Option<u32>,
    #[serde(rename = "@footprint")]
    pub footprint: Option<String>,
    #[serde(rename = "@sightRadius")]
    pub sight_radius: Option<f32>,
    #[serde(rename = "@supply")]
    pub supply: Option<f32>,
    #[serde(rename = "@range")]
    pub range: Option<f32>,
    #[serde(rename = "@speed")]
    pub speed: Option<f32>,
    #[serde(rename = "@targets")]
    pub targets: Option<String>,
    #[serde(rename = "@count")]
    pub count: Option<u32>,
    #[serde(rename = "@cargoCapacity")]
    pub cargo_capacity: Option<u32>,
}

/// The Producer of a unit.
/// <producer id="Probe" name="438"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producer {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
}

/// A bonus effect of a Weapon.
/// <bonus damage="10" max="10" type="Armored"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectBonus {
    #[serde(rename = "@damage")]
    pub damage: u32,
    #[serde(rename = "@max")]
    pub max: u32,
    #[serde(rename = "@type")]
    pub kind: Option<String>,
}

/// The area of an effect
/// <area radius="2" fraction="0.33"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectArea {
    #[serde(rename = "@radius")]
    pub radius: f32,
    #[serde(rename = "@fraction")]
    pub fraction: Option<f32>,
}

/// An Effect of a Weapon.
/// <effect id="ParticleBeam" index="535" radius="-1" damage="5" max="5" death="Normal" kind="Melee"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@index")]
    pub index: u32,
    #[serde(rename = "@radius")]
    pub radius: f32,
    #[serde(rename = "@damage")]
    pub damage: u32,
    #[serde(rename = "@max")]
    pub max: u32,
    #[serde(rename = "@death")]
    pub death: String,
    #[serde(rename = "@kind")]
    pub kind: String,
    pub bonus: Option<EffectBonus>,
}

/// A Weapon of a unit.
/// <weapon id="ParticleBeam" index="16">
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@index")]
    pub index: u32,
    pub meta: Meta,
    pub misc: Option<Misc>,
    pub score: Option<Effect>,
}

/// An Attribute of a unit.
/// <attribute type="Armored"/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    #[serde(rename = "@type")]
    pub kind: Option<String>,
}

/// A Command of an Ability.
/// <command id="Cancel" index="0">
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// The "passive" command has no id.
    #[serde(rename = "@id", default)]
    pub id: Option<String>,
    /// i64 matches the events in the SC2Replayfiles.
    #[serde(rename = "@index")]
    pub index: i64,
    pub meta: Meta,
}

/// An Ability of a unit.
/// <ability id="BuildInProgress" index="129">
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    #[serde(rename = "@id")]
    pub id: String,
    /// u16 to match the event type in the SC2Replay files.
    #[serde(rename = "@index")]
    pub index: Option<u16>,
    #[serde(rename = "command", default)]
    pub commands: Vec<Command>,
}

/// A short reference to an Upgrade.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "upgrade")]
pub struct UpgradeRef {
    #[serde(rename = "@id", default)]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
}

/// A requirement for an UpgradeLevel.
/// May be a unit and a previous upgrade for example.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeLevelRequirement {
    #[serde(rename = "unit", default)]
    pub unit: Vec<UnitShortRef>,
    #[serde(rename = "upgrade", default)]
    pub upgrade: Vec<UpgradeRef>,
}

/// A level of an Upgrade.
/// <level id="ProtossShieldsLevel1" index="48">
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "level")]
pub struct UpgradeLevel {
    #[serde(rename = "@id", default)]
    pub id: String,
    #[serde(rename = "@index", default)]
    pub index: u32,
    pub meta: Meta,
    #[serde(default)]
    pub requires: Vec<UpgradeLevelRequirement>,
    pub cost: Cost,
}

/// An Upgrade of a unit.
/// <upgrade id="ProtossShields">
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrade {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "level", default)]
    pub levels: Vec<UpgradeLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "requires")]
pub struct RequiresTag {
    #[serde(rename = "unit", default)]
    pub inner: Vec<UnitShortRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "attributes")]
pub struct AttributesTag {
    #[serde(rename = "attribute", default)]
    pub inner: Vec<Attribute>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "strengths")]
pub struct StrengthsTag {
    #[serde(rename = "unit", default)]
    pub inner: Vec<UnitShortRef>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "weaknesses")]
pub struct WeaknessesTag {
    #[serde(rename = "unit", default)]
    pub inner: Vec<UnitShortRef>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "weapons")]
pub struct WeaponsTag {
    #[serde(rename = "weapon", default)]
    pub inner: Vec<Weapon>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "abilities")]
pub struct AbilitiesTag {
    #[serde(rename = "ability", default)]
    pub inner: Vec<Ability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "trains")]
pub struct TrainsTag {
    #[serde(rename = "unit", default)]
    pub inner: Vec<UnitRef>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "builds")]
pub struct BuildsTag {
    #[serde(rename = "unit", default)]
    pub inner: Vec<UnitRef>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename = "upgrades")]
pub struct UpgradesTag {
    #[serde(rename = "upgrade", default)]
    pub inner: Vec<Upgrade>,
}

/// A <unit> definition that belongs to a specific protocol version.
/// They may be compatible with other versions but no idea yet, I wish we could find a dump of
/// previous versions or that we could get the Starcrat II Editor to load previous versions of the
/// games.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "unit")]
pub struct VersionedBalanceUnit {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(skip)]
    pub version: u32,
    pub meta: Meta,
    pub life: Life,
    #[serde(default)]
    pub shields: Option<Shields>,
    pub armor: Armor,
    #[serde(rename = "shieldArmor")]
    pub shield_armor: ShieldArmor,
    #[serde(default)]
    pub requires: RequiresTag,
    pub cost: Option<Cost>,
    #[serde(default)]
    pub movement: Option<Movement>,
    pub score: Score,
    pub misc: Misc,
    #[serde(default)]
    pub producer: Option<Producer>,
    #[serde(default)]
    pub attributes: AttributesTag,
    #[serde(default)]
    pub strengths: StrengthsTag,
    #[serde(default)]
    pub weaknesses: WeaknessesTag,
    #[serde(default)]
    pub weapons: WeaponsTag,
    #[serde(default)]
    pub abilities: AbilitiesTag,
    #[serde(default)]
    pub builds: BuildsTag,
    #[serde(default)]
    pub trains: TrainsTag,
    #[serde(default)]
    pub upgrades: UpgradesTag,
}

pub type MultiVersionedBalanceUnits = HashMap<(u32, String), VersionedBalanceUnit>;
pub type VersionedBalanceUnits = HashMap<String, VersionedBalanceUnit>;

pub fn get_indexed_ability_command_name(
    balance_data_units: &VersionedBalanceUnits,
    unit_name: &str,
    ability_id: u16,
    command_index: i64,
) -> String {
    if let Some(balance_unit) = balance_data_units.get(unit_name) {
        for ability in &balance_unit.abilities.inner {
            match ability.index {
                None => continue,
                Some(idx) if idx != ability_id => continue,
                Some(_) => {
                    for command in &ability.commands {
                        if command_index == command.index {
                            return format!(
                                "{}.{}",
                                ability.id,
                                command.id.as_ref().unwrap_or(&"".to_string())
                            );
                        }
                    }
                }
            }
        }
        for train in &balance_unit.trains.inner {
            if let (Some(train_ability_id), Some(train_command_index)) =
                (train.ability_id, train.command_index)
            {
                if ability_id == train_ability_id && command_index == train_command_index {
                    return format!("Train.{}", train.id);
                }
            }
        }
        for build in &balance_unit.builds.inner {
            if let (Some(train_ability_id), Some(build_command_index)) =
                (build.ability_id, build.command_index)
            {
                if ability_id == train_ability_id && command_index == build_command_index {
                    return format!("Build.{}", build.id);
                }
            }
        }
        tracing::warn!("Unable to correlate ability for {:?}", balance_unit);
    } else {
        tracing::warn!("Balance ability not found for unit {}", unit_name);
    }
    Default::default()
}
