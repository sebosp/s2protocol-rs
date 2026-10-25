//! The button textures of the units, upgrades and abilities, exported with the balance data from
//! the StarCraft II Editor, see the README. The textures are the same in all the exported patch
//! versions so they are stored once, in `assets/Textures`, as DDS files with lowercase names.
//!
//! The balance data references them by their icon name, i.e. the `meta.icon` of a unit is
//! `btn-unit-zerg-zergling`, the texture is `btn-unit-zerg-zergling.dds`.

use crate::game_events::VersionedBalanceUnits;
use include_dir::{Dir, include_dir};
use std::collections::HashMap;

static TEXTURES_DIR: Dir = include_dir!("assets/Textures");

/// The DDS file of the icon, the icon names do not always match the case of the files.
pub fn icon_dds(icon: &str) -> Option<&'static [u8]> {
    TEXTURES_DIR
        .get_file(format!("{}.dds", icon.to_lowercase()))
        .map(|file| file.contents())
}

/// The icon names of all the textures.
pub fn icon_names() -> impl Iterator<Item = &'static str> {
    TEXTURES_DIR
        .files()
        .filter_map(|file| file.path().file_stem()?.to_str())
}

/// The icon of the unit, i.e. `Zergling`.
pub fn unit_icon<'a>(units: &'a VersionedBalanceUnits, unit_name: &str) -> Option<&'a str> {
    units.get(unit_name)?.meta.icon.as_deref()
}

/// The icons of the upgrades researched by the units, by their lowercase id, the upgrade names of
/// the tracker events, i.e. `zerglingmovementspeed`.
pub fn upgrade_icons(units: &VersionedBalanceUnits) -> HashMap<String, String> {
    units
        .values()
        .filter_map(|unit| unit.researches.as_ref())
        .flat_map(|researches| researches.inner.iter())
        .filter_map(|upgrade| {
            let icon = upgrade.meta.icon.clone()?;
            Some((upgrade.id.to_lowercase(), icon))
        })
        .collect()
}

/// The icons of the ability commands by the name of the game events, `<ability>.<command>`, see
/// [`crate::game_events::get_indexed_ability_command_name`], i.e. `Blink.Execute`.
pub fn ability_command_icons(units: &VersionedBalanceUnits) -> HashMap<String, String> {
    units
        .values()
        .flat_map(|unit| unit.abilities.inner.iter())
        .flat_map(|ability| {
            ability.commands.iter().filter_map(|command| {
                let name = format!(
                    "{}.{}",
                    ability.id,
                    command.id.as_deref().unwrap_or_default()
                );
                Some((name, command.meta.icon.clone()?))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_finds_the_textures() {
        // DDS files start with the "DDS " magic number.
        let zergling = icon_dds("btn-unit-zerg-zergling").unwrap();
        assert_eq!(&zergling[..4], b"DDS ");
        // The case of the icon names does not matter.
        assert!(icon_dds("BTN-ability-Protoss-cliffwalk-color").is_some());
        assert!(icon_dds("not-a-texture").is_none());
        assert!(icon_names().count() > 500);
    }

    #[test]
    fn it_finds_the_upgrade_and_ability_icons() {
        let balance_data = crate::game_events::read_balance_data_from_included_assets().unwrap();
        let units = &balance_data[&98310];
        assert_eq!(unit_icon(units, "Zergling"), Some("btn-unit-zerg-zergling"));
        let upgrades = upgrade_icons(units);
        assert_eq!(
            upgrades["zerglingmovementspeed"],
            "btn-upgrade-zerg-metabolicboost"
        );
        let commands = ability_command_icons(units);
        assert!(icon_dds(&commands["Blink.Execute"]).is_some());
        // Most of the icons have a texture.
        let missing = upgrades
            .values()
            .filter(|icon| icon_dds(icon).is_none())
            .count();
        assert!(missing * 10 < upgrades.len(), "{missing} missing");
    }
}
