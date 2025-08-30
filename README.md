[![Crates.io](https://img.shields.io/crates/v/s2protocol.svg)](https://crates.io/crates/s2protocol)
[![Workflow Status](https://github.com/sebosp/s2protocol-rs/workflows/Rust/badge.svg)](https://github.com/sebosp/s2protocol-rs/actions?query=workflow%3A%22Rust%22)

# s2protocol-rs

A nom parser for the Starcraft 2 Protocol Replay format.
Provides Iterators to transist a minimal state machine through the game loops
Keeping track of unit groups, upgrades, targets, etc.

## Motivation

The goal is to learn how to parse binary files format with `nom` and to learn
how the Starcraft 2 Replay file is so incredibly small for the amount of
information it packs.

From the available data, analytics, visualizations and generative art can be created, for example
by using 
- jupyter notebook in [s2-polars-data-analysis](https://github.com/sebosp/s2-polars-data-analysis) - [rerun](https://github.com/rerun-io/rerun) : See the repo [swarmy](https://github.com/sebosp/swarmy)
- [yew](https://github.com/yewstack/yew) [cooper](https://github.com/sebosp/cooper)
- [eframe/egui](https://github.com/emilk/egui): See repo [eframes-c2](https://github.com/sebosp/eframe-sc2)
- [bevyengine/bevy](https://github.com/bevyengine/bevy) can be used to see:
  - An Enhanced Replay Minimap
  - Additional statistics.

## Consuming events

### Transition Iterators

- `SC2EventIterator` collects both TrackerEvents and GameEvents. Events are provided as they appear, be them Tracker or Game, they are sorted by the adjusted game loop.

Event changes transist a minimal state machine that updates:
- names
- positions
- Attack events
- etc.

The iterators returns a struct of type `SC2EventIteratorItem`.
The `event` field contains the event itself as it was deserialized from the SC2Replay.
The `change_hint` field hints the consumers of the iterator about the changes performed to the state machine due to this event.
For example, units may have been deleted, added, changed position, etc.

```rust
let source: PathBuf = PathBuf::new();
let res = s2protocol::state::SC2EventIterator::new(&source)?;
for (event, change_hint) in res.into_iter() {
    println!("{},", serde_json::to_string(&event)?);
}
```

## Balance Data

Units perform abilities that are specific to different patches.
The SCII Editor allows the balance data to be exported.
This includes:
- The abilities for each unit (in XML format)
- The ability Icons in DDS format
- The map in .tga format.
So, a unit, say, for an Overlord, may run ability link "43", within this ability, a command indexed as "1",
Inside the `Overlord.xml` there is:

```xml
<?xml version="1.0" encoding="utf-8"?>
<unit id="Overlord">
  <abilities>
    <ability id="move" index="43">
      <command id="Move" index="0">
        <! <meta icon, hotkey, range, etc -->
      </command>
      <command id="Patrol" index="1">
        <! <meta icon, hotkey, range, etc -->
      </command>
    </ability>
  </abilities>
</unit>
```

To use the exported balance data, data must be exported and stored in per-version directory with the following structure:
`<somepath>/94137/BalanceData/Overlord.xml`
where `94137` is the version of the exported XMLs.

To transform the --xml-data into an local-jsonified-version (stored in assets directory for now):
```
$ cargo run -r -- -v info --timing --source $HOME/SC2Replays/BalanceData/ --json-balance-data-dir $PWD/assets/BalanceData/ --output $PWD/assets/BalanceData/ balance-data-to-json
2025-08-30T18:18:04.528202Z  INFO s2protocol::game_events::ability::balance_data: Processed 1058 total units
2025-08-30T18:18:04.528218Z  INFO s2protocol::game_events::ability::balance_data::json_handler: Writing balance data to JSON files in /home/seb/git/s2protocol-rs/assets/BalanceData/
```

For example:

```
$ cargo run -- --source assets/FieldsDeath202507.SC2Replay --max-events 450 --color --json-balance-data-dir $PWD/assets/BalanceData/ --verbosity-level info get transist-events
     Running `target/debug/s2protocol --source assets/FieldsDeath202507.SC2Replay --max-events 450 --color --xml-balance-data-dir /home/seb/SC2Replays/BalanceData/ --verbosity-level warning get transist-events`
2025-08-30T18:19:47.983171Z  INFO s2protocol::game_events::ability::balance_data::json_handler: Reading balance data from JSON files in /home/seb/git/s2protocol-rs/assets/BalanceData/
2025-08-30T18:19:48.532272Z  INFO s2protocol::game_events::ability::balance_data::json_handler: Read 1058 versioned balance units from JSON files
2025-08-30T18:19:48.532290Z  INFO s2protocol::cli: Processing "assets/FieldsDeath202507.SC2Replay"
2025-08-30T18:19:48.532848Z  INFO s2protocol::cli: Transducing through both Game and Tracker Events
2025-08-30T18:19:48.576246Z  INFO s2protocol::state: Collected 1058 unit definitions for protocol version 94137 out of 1058 total definitions
...
```

If the XMLs/JSON for a specific version doesn't exist, all ability Strings will appear as "".

### TODO:

- The json exports from BalanceData could be loaded from a remote repo (github maybe?), otherwise people need to have this repo cloned.

## Displaying the replay on the terminal

Suppose you wanted to debug/inspect a replay, you can draw the events to the terminal, for example:

```bash
$ cargo run --features syntax,dep_ratatui -- --max-loop 1000 --color --source /mnt/replays/"Pylon LE.SC2Replay" get transist-events
```

[![Ratatui asciicast](https://asciinema.org/a/726584.svg)](https://asciinema.org/a/726584)

## BREAKING CHANGES

- Feature `arrow`  has been renamed to `dep_arrow` as we now use the `arrow` crate.
- Previously the generated arrow IPC files relied on a sha256 of the file as "key", this wasted a lot of space and has been changed into a sequential
  As a result, queries must use the new `ext_fs_id`, which also means all the files must be generated in bulk and are no longer independent.
  The IPCs then is valid as a snapshot generated as a whole.

## Interacting with polars

### TODO:
- The UpdateTargetPoint Cmd arrow snapshot should contain the unit name, otherwise kindof to have an ability withouth it.

### Generating the IPC Arrow datasets

In the directory ipcs/ one .ipc file will be created per implemented data type.
The `--source` is the directory that contains the replay directory (Or a single file).
Multiple subdirectories are supported.
Files are processed using parallel operations.
For 17K replays (2.3 GBs) it takes 120 seconds to parse/transform/split them. YMMV, in this case only 10K files had valid init data (as in are supported protocol versions).

```bash
$ mkdir ipcs/
$ cargo watch -i ipcs -x "run -r -- -v error --timing --source $HOME/SCReplaysOnNVMe --xml-balance-data-dir $HOME/SC2Replays/BalanceData/ --output $HOME/git/s2protocol-rs/ipcs/ write-arrow-ipc --process-max-files 1000000"
36752 files have valid init data, processing...
Total time: 280.125785662s
$ du -sh ipcs
20G     ipcs
$ ls -ltra ipc
total 20575896
drwxr-xr-x 11 seb seb       4096 Aug 16 18:14 ..
drwxr-xr-x  2 seb seb       4096 Aug 17 18:27 .
-rw-r--r--  1 seb seb  117183106 Aug 17 19:35 lobby_init_data.ipc
-rw-r--r--  1 seb seb   10095306 Aug 17 19:35 details.ipc
-rw-r--r--  1 seb seb 2173322874 Aug 17 19:35 stats.ipc
-rw-r--r--  1 seb seb  179426002 Aug 17 19:35 upgrades.ipc
-rw-r--r--  1 seb seb 9958263882 Aug 17 19:36 unit_born.ipc
-rw-r--r--  1 seb seb 5859714578 Aug 17 19:37 unit_died.ipc
-rw-r--r--  1 seb seb 1950258354 Aug 17 19:38 cmd_target_point.ipc
-rw-r--r--  1 seb seb  821400850 Aug 17 19:39 cmd_target_unit.ipc
```

### Jupyter Notebooks

The jupyter notebook with examples on how to interact with the data are available in [s2-polars-data-analysis](https://github.com/sebosp/s2-polars-data-analysis)

### polars-cli

```bash
$ cargo install polars-cli
$ # List the max number of minerals that were lost in per map when the army was killed.
❯ echo "SELECT ext_fs_replay_file_name, MAX(minerals_lost_army) FROM read_ipc('/home/seb/git/s2protocol-rs/ipcs/stats.ipc') GROUP BY ext_fs_replay_file_name ORDER BY minerals_lost_army DESC;"|polars
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

## JSON Spec Sources

[Blizzard/s2protocol repo](https://github.com/Blizzard/s2protocol)
