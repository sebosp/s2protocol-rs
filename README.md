[![Crates.io](https://img.shields.io/crates/v/s2protocol.svg)](https://crates.io/crates/s2protocol)
[![Workflow Status](https://github.com/sebosp/s2protocol-rs/workflows/Rust/badge.svg)](https://github.com/sebosp/s2protocol-rs/actions?query=workflow%3A%22Rust%22)

# s2protocol-rs

A nom parser for the Starcraft 2 Protocol Replay format.
Additionally transduces through the replays to provide SC2ReplayState that keeps track of units, players, buildings, movement, units initialized and died.

## Generating protocol-specific code:

The rust code for the protocols versions available were generated using:
This would now be compared with ./src/versions/protocol99999.template file and from there we can analyze what has changed.
Notably, the number of bits used for the Chat Message is still miscalculated to 3 so it needs to be dismissed.

```bash
mkdir src/versions/protocol89720/
RUST_LOG_SPAN_EVENTS=full RUST_LOG=debug cargo watch -i src/versions/protocol89720/mod.rs -x 'run -- --source ../s2protocol/json/protocol89720.json generate --output src/versions/protocol89720/mod.rs'
# Add the new module to src/versions/mod.rs
# Run rust format on the new src/versions/protocol87702/mod.rs file
# cargo check, cargo build, etc
# Additionally some code to transform from Protocol-Specific to Protocol-Agnostic was added, TODO: Add to generator.rs
```

## version compatibility.
After further testing, it seems most of the types are compatible between versions, so only when they differ would they make part of the protocol version.
Since I started this exercise on protocol87702, all types would be relative to it. That is, most modules would re-use protocol87702 as much as possible.
This explains why old-er versions such as 75689 would still reference 87702 as much as possible.

The generator above thus would show example code and does not reflect anymore the current `S2ProtoResult` created in favour of unwrapping/panic'ing.

## JSON Sources
[Blizzard/s2protocol repo](https://github.com/Blizzard/s2protocol)

## Motivation
The goal is to learn how to parse binary files format with `nom` and to learn
how the Starcraft 2 Replay file is so incredibly small for the amount of
information it packs.

From the available data, generative art can be created, for example
by using 
- [rerun](https://github.com/rerun-io/rerun) : See the repo [swarmy](https://github.com/sebosp/swarmy)
- [lyon](https://github.com/nical/lyon) (PoC in progress in cooper)
- [yew](https://github.com/yewstack/yew) [cooper](https://github.com/sebosp/cooper)
- [eframe/egui](https://github.com/emilk/egui): See repo [eframes-c2](https://github.com/sebosp/eframe-sc2)
- [bevyengine/bevy](https://github.com/bevyengine/bevy) can be used to see:
  - An Enhanced Replay Minimap
  - Additional statistics.

## Consuming events

To consume events, they are currently loaded in memory in a HashMap:

`GameLoop` -> `Vec<EventTypes>`

```rust
let include_stats = false;
let mut replay = SC2ReplayState::new(file_path, SC2ReplayFilters::default(), include_stats).unwrap();
// at this point, all events frcom the MPQ file at `file_path` have been loaded to memory.
// To progress through the game loop, the `replay` state machine transduces from one gameloop to the next one.
// This means it recycles variables, sets position, maintains active units, etc.
// For each transduce step, an SC2EventType is returned and the unit IDs that have been changed.
// These "units" properties can be looked up in the `replay` state machine further.
// In this example, the `add_tracker_event` and the `add_game_event` also are sent a reference to the SC2ReplayState
// For a working example, see the swarmy repo referenced above.
while let Some((event, updated_units)) = replay.transduce() {
    match event {
        SC2EventType::Tracker {
            tracker_loop,
            event,
        } => add_tracker_event(&self, tracker_loop, &event, updated_units)?, // Some code accessing the Tracker Events
        SC2EventType::Game {
            game_loop,
            user_id,
            event,
        } => add_game_event(&self, game_loop, user_id, &event, updated_units)?,
    }
}
```

## Current issues

Currently we load all events in memory, Perhaps we can try to read batches on events by keeping MPQ nom parser &[u8] reference.
For example, we could read different sections, and return events in different sections in a batch of evenst through a game loop.

## TODO

We can check that, if some module is exactly the same everywhere, we only create it once and re-use it everywhere.
This because the compilation time is getting out of hand.

## Status

- [x] Replay Tracker, Game Events and Chat Message Events for protocol75689
- [x] Replay Tracker, Game Events and Chat Message Events for protocol84643
- [x] Replay Tracker, Game Events and Chat Message Events for protocol87702
- [x] Replay Tracker, Game Events and Chat Message Events for protocol88500
- [x] Replay Tracker, Game Events and Chat Message Events for protocol89634
- [x] Replay Tracker, Game Events and Chat Message Events for protocol89720
- [x] Replay Tracker, Game Events and Chat Message Events for protocol90136
- [x] Replay Tracker, Game Events and Chat Message Events for protocol90779
- [x] Replay Tracker, Game Events and Chat Message Events for protocol90870
- [x] Parsing unit movement is done.
- [x] Decoding the tag/recycle done to match Game Events.
- [x] Game Events are parsed (tho some that seem irrelevant are skipped).
- [x] Read the version and from the version call the correct module so that we can support multiple modules.
- [ ] Add the remaining Tracker/Game event types.
- [ ] Support for MPQ embedded file: `replay.gamemetadata.json`
- [ ] Support for MPQ embedded file: `replay.initData`
- [ ] Support for MPQ embedded file: `replay.attributes.events`
