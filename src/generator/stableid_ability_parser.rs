use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Ability {
    #[serde(rename = "buttonname")]
    button_name: String,
    friendly_name: Option<String>,
    id: u16,
    index: u16,
    name: String,
    #[serde(default)]
    group: Option<String>,
    #[serde(default)]
    variant_name: String,
}

impl PartialEq for Ability {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {
    // Read the file ./stableid.json and parse it.
    let file = std::fs::File::open("./stableid.json").unwrap();
    let reader = io::BufReader::new(file);
    let stableid: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let predefined_groups = vec![
        "Adept",
        "Aiur",
        "Archon",
        "Armory",
        "Artanis",
        "Attack",
        "Baneling",
        "Banshee",
        "Barracks",
        "Battlecruiser",
        "Beacon",
        "BroodLord",
        "BuildAuto",
        "BuildNydus",
        "Buildin",
        "BuildingShield",
        "BuildingStasis",
        "Bunker",
        "Burrow",
        "Cancel",
        "Carrier",
        "CheckStation",
        "CliffDoor",
        "Colonist",
        "Colony",
        "CommandCenter",
        "CompoundMansion",
        "Corruption",
        "CreepTumor",
        "CyberneticsCoreResearch",
        "DarkShrine",
        "Defiler",
        "Destrucible",
        "Disguise",
        "Domination",
        "Drone",
        "Dukes",
        "DuskWing",
        "Effect",
        "EngineeringBayResearch",
        "EvolutionChamberResearch",
        "ExtendingBridge",
        "Factory",
        "FleetBeaconResearch",
        "ForceField",
        "ForgeResearch",
        "FusionCoreResearch",
        "GatewayTrain",
        "General",
        "GenerateCreep",
        "GhostAcademy",
        "Ghost",
        "GravitonBeam",
        "Hallucination",
        "Harvest",
        "Heal",
        "Hercules",
        "Hero",
        "HoldFire",
        "HutTransport",
        "Hybrid",
        "HydraliskDenResearch",
        "HyperionYamatoSpecial",
        "InfestableHutTransport",
        "InfestationPitResearch",
        "InfestedMonsterTrain",
        "Infested",
        "Irradiate",
        "Karass",
        "LairResearch",
        "LarvaTrain",
        "Leech",
        "LeviathanSpawn",
        "Liberator",
        "LightBridge",
        "LockOn",
        "Locust",
        "Loki",
        "LurkerAspect",
        "LurkerDenResearch",
        "Lurker",
        "MakeVultureSpiderMines",
        "MaxiumThrust",
        "MedivacTransport",
        "Medivac",
        "MercCompoundResearch",
        "MetalGate",
        "Morph",
        "Mothership",
        "Move",
        "MuleGather",
        "Mule",
        "NanoRepair",
        "NeuralParasite",
        "Nexus",
        "Nydus",
        "Observer",
        "Odin",
        "Oracle",
        "Orbital",
        "Overlord",
        "Overseer",
        "ParasiticBomb",
        "PerditionTurret",
        "Phase",
        "Phasing",
        "Pickup",
        "PortCity",
        "Probe",
        "ProtossBuild",
        "PurificationNova",
        "Purifier",
        "PurifyMorphPylon",
        "Que1",
        "Que5",
        "Que8",
        "QueenMp",
        "Queen",
        "Rally",
        "Raven",
        "Raynor",
        "RedstoneLavaCritter",
        "RefineryToAutomatedRefinery",
        "Repair",
        "ResearchLab",
        "ResearchProtoss",
        "ResearchTerran",
        "ResearchZerg",
        "RoachWarrenResearch",
        "RoboticsBayResearch",
        "RoboticsFacilityTrain",
        "RogueGhostCloakCloak",
        "Salvage",
        "Scv",
        "SecurityGate",
        "Seeker",
        "SelendisHangar",
        "SelfRepair",
        "SentryGun",
        "ShakurasLightBridge",
        "SiegeBreaker",
        "SnowRefineryTerranExtendingBridge",
        "SpawningPoolResearch",
        "Spawn",
        "SpecOpsDropshipTransport",
        "Spectre",
        "SpineCrawler",
        "SpireResearch",
        "SporeCrawler",
        "Spray",
        "SsBattlecruiser",
        "SsCarrier",
        "SsFighter",
        "SsLeviathan",
        "SsPowerupMorph",
        "SsScienceVessel",
        "Ss",
        "StargateTrain",
        "StarportTechLabMorph",
        "Starport",
        "Stimpack",
        "Stop",
        "SummonMercenaries",
        "SuperWarpGateTrain",
        "SupplyDepot",
        "SwarmHostSpawnLocusts",
        "TacNukeStrike",
        "TarsonisDoorE",
        "TaurenOuthouse",
        "TempestDisruptionBlast",
        "TemplarArchivesResearch",
        "TempleDoor",
        "TerranBuild",
        "Test",
        "Thor",
        "Transport",
        "TwilightCouncilResearch",
        "UltraliskCavernResearch",
        "UpgradeTo",
        "Val03QueenOfBlades",
        "Viper",
        "VoidSeeker",
        "Vulture",
        "WarpGateTrain",
        "WarpPrismTransport",
        "WidowMine",
        "XelNagaCavernsDoor",
        "XelNagaCavernsFloatingBridge",
        "ZergBuild",
    ];
    let mut abilities: Vec<Ability> = Vec::new();
    let mut registered_ability_names = Vec::new();
    for i in 0..stableid["Abilities"].as_array().unwrap().len() {
        let ability = Ability::deserialize(stableid["Abilities"][i].clone()).unwrap();
        abilities.push(ability);
    }
    for ability in abilities.iter_mut() {
        let mut variant_name = ability.name.to_case(Case::Pascal);
        let button_name = ability.button_name.to_case(Case::Pascal);
        if ability.name.is_empty() {
            variant_name = ability.button_name.to_case(Case::Pascal);
        }
        // If the variant name starts with a number we will prepend an "a" to it, since we'll add
        // it to a specific rust file as an enum variant as well.
        if variant_name.chars().next().unwrap().is_numeric() {
            variant_name = format!("A{}", variant_name);
        }
        if !button_name.is_empty() && !variant_name.contains(&button_name) {
            // If the button name is not part of the name we add it to the variant name.
            variant_name = format!("{}{}", variant_name, button_name,);
        }
        // if the first character of the variant is lowercase, we need to capitalize it.
        if let Some(c) = variant_name.chars().next() {
            if c.is_lowercase() {
                variant_name = variant_name.to_case(Case::Pascal);
            }
        }
        if let Some(friendly_name) = &ability.friendly_name {
            let friendly_name = friendly_name.to_case(Case::Pascal).to_string();
            if friendly_name.contains(&variant_name) {
                // If the friendly name contains the variant name, we don't need to add it.
                variant_name = friendly_name.clone();
            } else if friendly_name != variant_name && !variant_name.contains(&friendly_name) {
                variant_name = format!("{}{}", variant_name, friendly_name);
            }
        }
        ability.variant_name = variant_name.clone();
        if !registered_ability_names.contains(&variant_name) {
            registered_ability_names.push(variant_name.clone());
        }
        for group in &predefined_groups {
            if variant_name.starts_with(group) {
                ability.group = Some(group.to_string());
                break;
            }
        }
    }
    // Since there are so many abilities, we need to sort them and bucket them based on the first
    // letter.
    let mut ability_groups: HashMap<String, Vec<Ability>> = HashMap::new();
    for ability in abilities.iter() {
        let first_letter = ability.variant_name.chars().next().unwrap().to_string();
        if !ability_groups.contains_key(&first_letter) {
            ability_groups.insert(first_letter.clone(), Vec::new());
        }
        ability_groups
            .get_mut(&first_letter)
            .unwrap()
            .push(ability.clone());
    }
    // Sort the abilities in each group.
    for (_key, value) in ability_groups.iter_mut() {
        value.sort_by(|a, b| a.variant_name.cmp(&b.variant_name));
    }
    let mut keys: Vec<String> = ability_groups.keys().cloned().collect();
    keys.sort();
    let file_header = String::from(
        r#"
#[cfg(feature = "arrow")]
use arrow_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

use serde::{Deserialize, Serialize};
"#,
    );

    let enum_header = String::from(
        r#"
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "arrow",
  derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
"#,
    );
    let mut catch_all_mods = String::new();
    let mut catch_all_group_enum = String::new();
    catch_all_group_enum.push_str(&file_header);
    catch_all_group_enum.push_str(&enum_header);
    catch_all_group_enum.push_str("pub enum Ability{\n");
    let mut catch_all_group_from_matches = String::new();
    for key in keys.iter() {
        let value = ability_groups.get(key).unwrap();
        let mut letter_enum = String::new();
        let mut letter_matcher = String::new();
        let mut curr_abil = String::new();
        catch_all_mods.push_str(&format!("pub mod {};\n", key.to_lowercase()));
        catch_all_mods.push_str(&format!("pub use {}::*;\n", key.to_lowercase()));
        for ability in value.iter() {
            let variant_name = ability.variant_name.clone();
            if curr_abil != variant_name {
                letter_enum.push_str(&format!("  {},\n", variant_name));
                curr_abil = variant_name.clone();
            }
            letter_matcher.push_str(&format!(
                "      {} => Some(Self::{}),\n",
                ability.id, variant_name
            ));
        }
        let mut enum_contents = String::new();
        enum_contents.push_str(&file_header);
        enum_contents.push_str(&enum_header);
        enum_contents.push_str(&format!("pub enum {} ", key));
        enum_contents.push_str("{\n");
        enum_contents.push_str(&letter_enum);
        enum_contents.push_str("}\n");
        let mut matcher = format!("impl {} ", key);
        matcher.push_str(
            r#"{
    pub fn from_id(id: u16) -> Option<Self> {
      match id {
"#,
        );
        matcher.push_str(&letter_matcher);
        matcher.push_str("      _ => None,\n");
        matcher.push_str("    }\n");
        matcher.push_str("  }\n");
        matcher.push_str("}\n");
        let mut buffer = File::create(format!("src/ability/{}.rs", key.to_lowercase())).unwrap();
        buffer.write_all(enum_contents.as_bytes()).unwrap();
        buffer.write_all(matcher.as_bytes()).unwrap();
        //println!("{}", enum_contents);
        //println!("{}", matcher);
        catch_all_group_enum.push_str(format!("    {}({}),\n", key, key).as_str());
        catch_all_group_from_matches.push_str(&format!(
            "    if let Some(ability) = {}::from_id(id) {{ return Self::{}(ability); }}\n",
            key, key,
        ));
    }
    /*let fh = std::fs::File::create("abilities.rs").unwrap();
    let mut writer = io::BufWriter::new(fh);
    writer.write(global_matcher.as_bytes()).unwrap();
    writer.write(catch_all_group_enum.as_bytes()).unwrap();
    writer.write("}".as_bytes()).unwrap();*/
    // close the file
    catch_all_group_enum.push_str("impl Ability {\n");
    catch_all_group_enum.push_str("pub fn from_id(id: u16) -> Self {\n");
    catch_all_group_enum.push_str("  match id {\n");
    catch_all_group_enum.push_str(&catch_all_group_from_matches);
    catch_all_group_enum.push_str("    _ => {\n");
    catch_all_group_enum.push_str("      tracing::error!(\"Unknown ability id: {}\", id);\n");
    catch_all_group_enum.push_str("      Self::N(N::Null)\n");
    catch_all_group_enum.push_str("    }\n");
    catch_all_group_enum.push_str("  }\n");
    catch_all_group_enum.push_str("}\n");
    let mut buffer = File::create("src/ability/mod.rs").unwrap();
    buffer.write_all(file_header.as_bytes()).unwrap();
    buffer.write_all(catch_all_mods.as_bytes()).unwrap();
    buffer.write_all(catch_all_group_enum.as_bytes()).unwrap();
    //println!("{}", file_header);
    //println!("{}", catch_all_mods);
    //println!("{}", catch_all_group_enum);
}
