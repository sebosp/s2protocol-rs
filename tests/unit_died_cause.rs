//! The causes of the UnitDied events of a replay: the drones that became buildings, the larvae
//! that hatched and the MULEs that expired are not killed units.

use s2protocol::basic_replay_data::SC2ReplayBasicData;
use s2protocol::game_events::read_balance_data_from_included_assets;
use s2protocol::state::{SC2EventIterator, SC2EventType, UnitChangeHint};
use s2protocol::tracker_events::{ReplayTrackerEvent, UnitDiedCause};
use std::collections::HashMap;
use std::path::PathBuf;

/// The deaths of a replay by unit type and cause, the structures a drone builds that were done.
fn causes(replay: &str) -> (HashMap<(String, UnitDiedCause), usize>, usize) {
    let path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")).join(replay);
    let data = SC2ReplayBasicData::new(&path, 0).unwrap();
    let balance = read_balance_data_from_included_assets().unwrap();
    let mut causes: HashMap<(String, UnitDiedCause), usize> = HashMap::new();
    let mut drone_buildings = 0;
    for item in SC2EventIterator::new(&data, &balance).unwrap() {
        let SC2EventType::Tracker { event, .. } = item.event_type else {
            continue;
        };
        match (event, item.change_hint) {
            (ReplayTrackerEvent::UnitDied(died), UnitChangeHint::Unregistered { killed, .. }) => {
                if let Some(killer) = died.killer_player_id
                    && Some(killer) != killed.user_id
                    && died.cause != UnitDiedCause::Hallucination
                {
                    assert_eq!(died.cause, UnitDiedCause::Killed, "{died:?} {killed:?}");
                }
                if died.cause == UnitDiedCause::Morphed {
                    assert_eq!(died.killer_player_id, None, "{died:?}");
                }
                *causes.entry((killed.name.clone(), died.cause)).or_default() += 1;
            }
            (ReplayTrackerEvent::UnitDone(_), UnitChangeHint::Registered { unit, .. })
                if [
                    "Hatchery",
                    "Extractor",
                    "SpawningPool",
                    "EvolutionChamber",
                    "RoachWarren",
                    "BanelingNest",
                    "HydraliskDen",
                    "LurkerDenMP",
                    "Spire",
                    "InfestationPit",
                    "UltraliskCavern",
                    "NydusNetwork",
                    "SpineCrawler",
                    "SporeCrawler",
                ]
                .contains(&unit.name.as_str()) =>
            {
                drone_buildings += 1
            }
            _ => {}
        }
    }
    (causes, drone_buildings)
}

#[test]
fn it_tells_the_morphs_and_expiries_from_the_kills() {
    // Zerg against Terran and Protoss computers.
    let (causes, drone_buildings) = causes("FieldsDeath202507.SC2Replay");
    let count = |name: &str, cause: UnitDiedCause| {
        causes
            .get(&(name.to_string(), cause))
            .copied()
            .unwrap_or_default()
    };
    // Every drone that became a building, none of them is lost.
    assert!(drone_buildings > 50);
    assert_eq!(count("Drone", UnitDiedCause::Morphed), drone_buildings);
    assert!(count("Drone", UnitDiedCause::Killed) > 0);
    assert!(count("Larva", UnitDiedCause::Morphed) > 200);
    assert_eq!(count("HighTemplar", UnitDiedCause::Morphed) % 2, 0);
    assert!(count("MULE", UnitDiedCause::Expired) > 0);
    assert!(count("AdeptPhaseShift", UnitDiedCause::Expired) > 0);
    assert!(count("InvisibleTargetDummy", UnitDiedCause::Expired) > 1000);
    assert!(
        !causes
            .keys()
            .any(|(_, cause)| *cause == UnitDiedCause::Unknown)
    );
    // The units of the players lost are kills or friendly fire.
    let losses: usize = causes
        .iter()
        .filter(|((_, cause), _)| cause.is_loss())
        .map(|(_, count)| count)
        .sum();
    let all: usize = causes.values().sum();
    assert!(losses * 3 < all, "{losses} of {all}");
}
