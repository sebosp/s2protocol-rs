use super::*;
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use std::path::PathBuf;
use std::{collections::HashMap, fs::File};
use tracing;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnitAbilityCommandBuilder {
    pub id: Option<String>,
    pub index: Option<i64>,
    pub requires: Option<NamedIdRef>,
    pub cost: Option<UnitCost>,
    pub meta: UnitAbilityCommandMeta,
}

impl UnitAbilityCommandBuilder {
    pub fn build(self) -> Result<UnitAbilityCommand, String> {
        if self.meta.kind == "passive" {
            // Not sure how to handle passive as they do not have id.
            return Ok(UnitAbilityCommand {
                id: String::new(),
                index: 99,
                requires: None,
                cost: None,
                meta: self.meta,
            });
        } else if self.meta.kind != "command" {
            return Err(format!(
                "UnitAbilityCommandBuilder: unexpected kind: {}",
                self.meta.kind
            ));
        }
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
            meta: self.meta,
        })
    }

    /// Creates a new `Self` from the XML attributes
    pub fn from_owned_attributes(
        &mut self,
        path: &str,
        attributes: Vec<xml::attribute::OwnedAttribute>,
    ) {
        if path != "/unit/abilities/ability/command" {
            tracing::debug!("Unexpected path for UnitMeta: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "id" => self.id = Some(attr.value),
                "index" => self.index = Some(attr.value.parse().unwrap_or(0)),
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
pub struct UnitAbilityBuilder {
    pub id: Option<String>,
    pub index: Option<u16>,
    /// The current ability being parsed, if any.
    pub current_cmd: UnitAbilityCommandBuilder,
    pub cmds: Vec<UnitAbilityCommand>,
}

impl UnitAbilityBuilder {
    pub fn build(self) -> Result<Option<UnitAbility>, String> {
        if self.cmds.len() == 1 && self.cmds[0].meta.kind == "passive" {
            // If there is only one command and it is passive, we skip this ability.
            // Not sure how to handle it yet.
            return Ok(None);
        }
        let id = self.id.ok_or("UnitAbilityBuilder: id is required")?;
        let index = self.index.ok_or("UnitAbilityBuilder: index is required")?;
        Ok(Some(UnitAbility {
            id,
            index,
            cmds: self.cmds,
        }))
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
            tracing::debug!("Unexpected path for UnitMeta: {}", path);
            return;
        }
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "id" => self.id = Some(attr.value),
                "index" => self.index = Some(attr.value.parse().unwrap_or(0)),
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

/// Load a specific Unit XML attributes
pub fn read_balance_xml(fname: PathBuf) -> std::io::Result<VersionedBalanceUnit> {
    let mut unit = VersionedBalanceUnit::default();
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
                tracing::trace!(
                    "{:spaces$}+{name}+[{namespace:?}] attrs={attributes:?}",
                    "",
                    spaces = depth * 2
                );
                current_path.push_str(&format!("/{name}"));
                unit.from_xml_start_element(name.local_name.as_str(), &current_path, attributes);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                tracing::debug!("{:spaces$}-{name}", "", spaces = depth * 2);
                if current_path == "/unit" {
                    return Ok(unit);
                }
                current_path = current_path
                    .rsplit_once('/')
                    .map_or(String::new(), |(prefix, _)| prefix.to_string());
                if current_path == "/unit/abilities" && name.local_name.as_str() == "ability" {
                    let ability = unit.state.finish_current_ability().unwrap();
                    if let Some(ability) = ability {
                        unit.abilities.push(ability);
                    }
                }
                if current_path == "/unit/abilities/ability"
                    && name.local_name.as_str() == "command"
                {
                    unit.state.finish_current_command().unwrap();
                }
            }
            Err(e) => {
                tracing::error!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }
    // crate::json_print(serde_json::to_string(&res).unwrap(), false);

    // If we get this far we may not have found the end </unit> tag.
    tracing::warn!("Reached end of file without finding </unit> tag");
    Ok(unit)
}

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
                    let unit = read_balance_xml(path)?;
                    res.insert((protocol_version, unit.id.clone()), unit);
                }
            }
        }
    }
    tracing::info!("Processed {} total units", res.len(),);

    Ok(res)
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VersionedBalanceUnitState {
    /// The current ability being parsed, if any.
    pub ability: UnitAbilityBuilder,
}

impl VersionedBalanceUnitState {
    /// When reading the XML, a </ability> tag may be found
    pub fn finish_current_ability(&mut self) -> Result<Option<UnitAbility>, String> {
        let ability = self.ability.clone().build()?;
        self.ability = UnitAbilityBuilder::default();
        Ok(ability)
    }

    /// When reading the XML, a </command> tag may be found
    pub fn finish_current_command(&mut self) -> Result<(), String> {
        self.ability.finish_current_command()
    }
}
