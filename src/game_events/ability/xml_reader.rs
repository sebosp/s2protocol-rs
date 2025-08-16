use super::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tracing;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitAbilityCommandBuilder {
    pub id: Option<String>,
    pub index: Option<u32>,
    pub requires: Option<NamedIdRef>,
    pub cost: Option<UnitCost>,
}

impl UnitAbilityCommandBuilder {
    pub fn build(self) -> Result<UnitAbilityCommand, String> {
        let id = self.id.ok_or("UnitAbilityCommandBuilder: id is required")?;
        let index = self
            .index
            .ok_or("UnitAbilityCommandBuilder: index is required")?;
        let requires = self.requires;
        let cost = self.cost;
        Ok(UnitAbilityCommand {
            id,
            index,
            requires,
            cost,
        })
    }

    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/abilities/ability/command" {
            tracing::warn!("Unexpected path for UnitMeta: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "id" => self.id = Some(attr.value),
                "index" => self.index = Some(attr.value.parse().unwrap_or(0)),
                _ => {
                    tracing::warn!(
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
pub struct UnitAbilityBuilder {
    pub id: Option<String>,
    pub index: Option<i64>,
    /// The current ability being parsed, if any.
    pub current_cmd: UnitAbilityCommandBuilder,
    pub cmds: Vec<UnitAbilityCommand>,
}

impl UnitAbilityBuilder {
    pub fn build(self) -> Result<UnitAbility, String> {
        let id = self.id.ok_or("UnitAbilityBuilder: id is required")?;
        let index = self.index.ok_or("UnitAbilityBuilder: index is required")?;
        Ok(UnitAbility {
            id,
            index,
            cmds: self.cmds,
        })
    }

    pub fn finish_current_command(&mut self) -> Result<(), String> {
        let cmd = self.current_cmd.clone().build()?;
        self.cmds.push(cmd);
        self.current_cmd = UnitAbilityCommandBuilder::default();
        Ok(())
    }

    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/abilities/ability" {
            tracing::warn!("Unexpected path for UnitMeta: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "id" => self.id = Some(attr.value),
                "index" => self.index = Some(attr.value.parse().unwrap_or(0)),
                _ => {
                    tracing::warn!(
                        "{path} Unknown attribute: {} with value: {}",
                        attr.name.local_name,
                        attr.value
                    );
                }
            }
        }
    }
}

pub fn read_balance_xml(fname: PathBuf) -> std::io::Result<Vec<VersionedBalanceUnit>> {
    let mut res: Vec<VersionedBalanceUnit> = vec![];
    let mut current_unit = VersionedBalanceUnit::default();
    let file = File::open(fname)?;
    let file = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file);
    let mut depth = 0;
    let mut current_path = String::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            }) => {
                tracing::debug!(
                    "{:spaces$}+{name}+[{namespace:?}] attrs={attributes:?}",
                    "",
                    spaces = depth * 2
                );
                current_path.push_str(&format!("/{name}"));
                current_unit.from_xml_start_element(
                    name.local_name.as_str(),
                    &current_path,
                    attributes,
                );
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                tracing::debug!("{:spaces$}-{name}", "", spaces = depth * 2);
                if current_path == "/unit" {
                    // End of a unit element, push it to the result
                    res.push(current_unit.clone());
                    current_unit = VersionedBalanceUnit::default(); // Reset for the next unit
                }
                current_path = current_path
                    .rsplit_once('/')
                    .map_or(String::new(), |(prefix, _)| prefix.to_string());
                if current_path == "/unit/abilities" && name.local_name.as_str() == "ability" {
                    let ability = current_unit.state.finish_current_ability().unwrap();
                    current_unit.abilities.push(ability);
                }
                if current_path == "/unit/abilities/ability"
                    && name.local_name.as_str() == "command"
                {
                    current_unit.state.finish_current_command().unwrap();
                }
            }
            Err(e) => {
                tracing::warn!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }
    tracing::info!("Parsed {} units", res.len());
    crate::json_print(serde_json::to_string(&res).unwrap(), false);

    Ok(res)
}

pub fn traverse_versioned_balance_abilities(root_dir: impl Into<PathBuf>) -> std::io::Result<()> {
    let root_dir: PathBuf = root_dir.into();
    tracing::info!(
        "Traversing versioned balance abilities in {}",
        root_dir.display()
    );
    if !root_dir.is_dir() {
        return Ok(());
    }
    for versioned_dir in std::fs::read_dir(root_dir)? {
        let versioned_dir = versioned_dir?.path();
        tracing::info!("Traversing versioned directory: {:?}", versioned_dir);
        if !versioned_dir.is_dir() {
            continue;
        }
        let versioned_balance_data_dir = versioned_dir.join("BalanceData");
        if !versioned_balance_data_dir.is_dir() {
            continue;
        }
        tracing::info!(
            "Traversing versioned balance data directory: {:?}",
            versioned_balance_data_dir
        );
        for entry in std::fs::read_dir(versioned_balance_data_dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.ends_with("Zergling.xml") || !path.is_file() {
                continue;
            }
            tracing::info!("Processing entry: {:?}", path);
            if let Some(ext) = path.extension() {
                if ext == "xml" && path.is_file() {
                    tracing::info!("Found XML file: {:?}", path);
                    let _ = read_balance_xml(path);
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionedBalanceUnitState {
    /// The current ability being parsed, if any.
    pub ability: UnitAbilityBuilder,
}

impl VersionedBalanceUnitState {
    /// When reading the XML, a </ability> tag may be found
    pub fn finish_current_ability(&mut self) -> Result<UnitAbility, String> {
        let ability = self.ability.clone().build()?;
        self.ability = UnitAbilityBuilder::default();
        Ok(ability)
    }

    /// When reading the XML, a </command> tag may be found
    pub fn finish_current_command(&mut self) -> Result<(), String> {
        self.ability.finish_current_command()
    }
}
