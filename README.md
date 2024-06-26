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

These are different ways to consume the events:

- `SC2EventIterator` collects both TrackerEvents and GameEvents. Events are provided as they appear, be them Tracker or Game
- `TrackerEventIterator` allows consuming only Tracker Events
- `GameEventIterator` allows consuming only the Game Events

Event changes transist a minimal state machine that updates:
- names
- positions
- Attack events
- etc.

The iterators returns a tuple `(SC2EventType, UnitChangeHint)`
The first item contains the event itself as it was deserialized from the SC2Replay.
The second item hints the consumers of the iterator about the changes performed to the state machine due to this event.
For example, units may have been deleted, added, changed position, etc.

```rust
let source: PathBuf = PathBuf::new();
let res = s2protocol::state::SC2EventIterator::new(&source)?;
for (event, change_hint) in res.into_iter() {
    println!("{},", serde_json::to_string(&event)?);
}
```

## Interacting with polars

### Generating the IPC Arrow datasets

In the directory ipcs/ one .ipc file will be created per implemented data type.
The `--source` is the directory that contains the replay directory (Or a single file).
Multiple subdirectories are supported.
Files are processed using parallel operations.
For 17K replays (2.3 GBs) it takes 120 seconds to parse/transform/split them. YMMV, in this case only 10K files had valid init data (as in are supported protocol versions).

```bash
$ mkdir ipcs/
$ cargo run -r -- -v error --timing --source /home/seb/SCReplaysOnNVMe --output /home/seb/git/s2protocol-rs/ipcs/ write-arrow-ipc --process-max-files 10000000 all
Located 17021 matching files by extension
10299 files have valid init data, processing...
Total time: 121.943999981s
& du -sh ipcs
13G     ipcs

& ls -ltra ipcs
total 13123004
drwxr-xr-x  2 seb seb       4096 Jun 16 09:33 ./
drwxr-xr-x 11 seb seb       4096 Jun 16 22:09 ../
-rw-r--r--  1 seb seb   81070479 Jun 16 22:10 init_data.ipc
-rw-r--r--  1 seb seb   10789826 Jun 16 22:10 details.ipc
-rw-r--r--  1 seb seb  843902480 Jun 16 22:10 stats.ipc
-rw-r--r--  1 seb seb   67967152 Jun 16 22:10 upgrades.ipc
-rw-r--r--  1 seb seb 5712488337 Jun 16 22:10 unit_born.ipc
-rw-r--r--  1 seb seb 4652841877 Jun 16 22:11 unit_died.ipc
-rw-r--r--  1 seb seb 2068859942 Jun 16 22:12 cmd.ipc
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

