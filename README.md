[![Crates.io](https://img.shields.io/crates/v/s2protocol.svg)](https://crates.io/crates/s2protocol)
[![Workflow Status](https://github.com/sebosp/s2protocol-rs/workflows/Rust/badge.svg)](https://github.com/sebosp/s2protocol-rs/actions?query=workflow%3A%22Rust%22)

# s2protocol-rs

A nom parser for the Starcraft 2 Protocol Replay format.
Additionally transduces through the replays to provide SC2ReplayState that keeps track of units, players, buildings, movement, units initialized and died.

## Motivation
The goal is to learn how to parse binary files format with `nom` and to learn
how the Starcraft 2 Replay file is so incredibly small for the amount of
information it packs.

From the available data, analytics, visualizations and generative art can be created, for example
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

## Interacting with polars

### Generating the IPC Arrow datasets

In the directory ipcs/ one .ipc file will be created per implemented data type.
The `--source` is the directory that contains the replay of interest.
Files are processed using parallel operations. For 3600 files (500 MBs) it takes 30 seconds to transform/split them.

```bash
$ mkdir ipcs/
$ cargo run -r -- --source "/mnt/windows/Users/sebos/Documents/StarCraft II/Accounts/51504154/2-S2-1-8459957/Replays/Multiplayer/" --output ipcs/ write-arrow-ipc all
Total time: 29.83001854s
```

### Jupyter Notebooks

[Basic UnitBorn Queries](./jupyter_notebooks/Basic-UnitBorn-Queries.ipynb)

### polars-cli

```bash
$ cargo install polars-cli
$ # List the max number of minerals that were lost in per map when the army was killed.
❯ echo "SELECT ext_fs_replay_file_name, MAX(minerals_lost_army) FROM read_ipc('/home/seb/git/s2protocol-rs/stats.ipc') GROUP BY ext_fs_replay_file_name ORDER BY minerals_lost_army DESC;"|polars
┌───────────────────────────────────┬────────────────────┐
│ ext_fs_replay_file_name           ┆ minerals_lost_army │
│ ---                               ┆ ---                │
│ str                               ┆ i32                │
╞═══════════════════════════════════╪════════════════════╡
│ Heavy Artillery LE (349).SC2Repl… ┆ 71362              │
│ Arctic Dream LE (398).SC2Replay   ┆ 59375              │
│ Nightscape LE (52).SC2Replay      ┆ 54846              │
│ …                                 ┆ …                  │
│ Emerald City LE (223).SC2Replay   ┆ 43450              │
│ Rhoskallian LE (101).SC2Replay    ┆ 41614              │
│ Fields of Death (345).SC2Replay   ┆ 41529              │
│ Rhoskallian LE (346).SC2Replay    ┆ 41425              │
└───────────────────────────────────┴────────────────────┘
```


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
- [x] Support for MPQ embedded file: `replay.initData`
- [ ] Add the remaining Tracker/Game event types.
- [ ] Support for MPQ embedded file: `replay.gamemetadata.json`
- [ ] Support for MPQ embedded file: `replay.attributes.events`

## Current issues

Currently we load the file contents in memory.
Since the replays are a few hundred KBs size, this seems not be an issue. (Maybe for AI ladder this is different).
Future iterators will work with `nom::streaming` to be able to avoid loading too much stuff into memory.

## version compatibility.

After a bit of testing, it seems most of the types are compatible between versions, so only when they differ would they make part of the protocol version.
Since I started this exercise on protocol87702, all types would be relative to it. That is, most modules would re-use protocol87702 as much as possible.
This explains why old-er versions such as 75689 would still reference 87702 as much as possible.

The generator above thus would show example code `S2ProtoResult` created in favour of unwrapping/panic'ing.

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

## JSON Sources
[Blizzard/s2protocol repo](https://github.com/Blizzard/s2protocol)

