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

### How the balance data was collected

The data in `assets/BalanceData` was exported by opening the StarCraft II Editor after each patch and
clicking `File` -> `Export Balance Data` for Legacy of the Void (LotV, the melee multiplayer data).
The export contains the XMLs of the units of the patch and the `Assets/Textures` with their button
icons.

We only started doing this a few months ago, so the oldest export is for protocol version 94137, the
replays of older patches use the closest version available. **If you have exported the balance data
of older patches (or of patches missing here, see `assets/BalanceData`), we would really appreciate
it if you share it with us**, please open an issue or a pull request at
https://github.com/sebosp/s2protocol-rs, the exported directory as it is, or a link to it, is enough.

### Textures

The button textures referenced by the `meta.icon` of the units, upgrades and ability commands, i.e.
`btn-unit-zerg-zergling`, are in `assets/Textures` as DDS files (76x76, DXT5). They were identical in
all the exported patches, so they are stored once, with lowercase names because the case of the icon
names in the balance data does not always match the file names. They are about 7MB (2.3MB compressed).

The `textures` feature embeds them, see the `textures` module:

```rust
let balance_data = s2protocol::game_events::read_balance_data_from_included_assets()?;
let units = &balance_data[&98310];
let icon = s2protocol::textures::upgrade_icons(units)["zerglingmovementspeed"].clone();
let dds: &[u8] = s2protocol::textures::icon_dds(&icon).unwrap();
```

To add the textures of a new export:

```bash
for f in $HOME/SC2Replays/BalanceData/<version>/Assets/Textures/*.dds; do
  cp -n "$f" assets/Textures/$(basename "$f" | tr 'A-Z' 'a-z')
done
```

### Using the exported data

To use the exported balance data, data must be exported and stored in per-version directory with the following structure:
`<somepath>/94137/BalanceData/Overlord.xml`
where `94137` is the version of the exported XMLs.

To transform the --xml-data into an local-jsonified-version (stored in assets directory for now):
```
❯ cargo run --no-default-features --features dep_tokio --features tracing_debug_level --example full -r -- -v debug --source $HOME/SC2Replays/BalanceData/ --json-balance-data-dir $PWD/assets/BalanceData/ --output $PWD/assets/BalanceData/ balance-data-to-json
    Finished `release` profile [optimized] target(s) in 29.23s
     Running `target/release/examples/full -v info --source /home/seb/SC2Replays/BalanceData/ --json-balance-data-dir /home/seb/git/s2protocol-rs/assets/BalanceData/ --output /home/seb/git/s2protocol-rs/assets/BalanceData/ balance-data-to-json`
2026-06-23T18:03:56.284789Z  INFO s2protocol::game_events::ability::balance_data::json_handler: Reading balance data from JSON files in /home/seb/git/s2protocol-rs/assets/BalanceData/
2026-06-23T18:03:58.347528Z  INFO s2protocol::game_events::ability::balance_data::json_handler: Read 7406 versioned balance units from JSON files
2026-06-23T18:04:53.779981Z  INFO s2protocol::game_events::ability::balance_data: Processed 7320 total units
2026-06-23T18:04:53.779998Z  INFO s2protocol::game_events::ability::balance_data::json_handler: Writing balance data to JSON files in /home/seb/git/s2protocol-rs/assets/BalanceData/
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
- The Balance Units contain strengths and weaknesses, we could return as part of ChangeHint a ratio of how good/bad the units are towards the existing enemy army.
- The ability-index/command-index may be compatible between multiple protocol versions.
- arrow-ipc tables can be created for BalanceUnits

## Displaying the replay on the terminal

Suppose you wanted to debug/inspect a replay, you can draw the events to the terminal, for example:

```bash
$ cargo run --features syntax,dep_ratatui,tracing_off -- --max-loop 1000 --color --json-balance-data-dir $PWD/assets/BalanceData/ --source $PWD/assets/FieldsDeath202507.SC2Replay get transist-events
```

[![Ratatui asciicast](https://asciinema.org/a/726584.svg)](https://asciinema.org/a/726584)

## BREAKING CHANGES

v3

- Feature `arrow`  has been renamed to `dep_arrow` as we now use the `arrow` crate.
- Previously the generated arrow IPC files relied on a sha256 of the file as "key", this wasted a lot of space and has been changed into a sequential
  As a result, queries must use the new `ext_fs_id`, which also means all the files must be generated in bulk and are no longer independent.
  The IPCs then is valid as a snapshot generated as a whole.
v3.4
- There is only one iterator provided now, that goes through both the Tracker and Game events to build the game state.
  Previously two iterators were provided that build partial state only for either track/game and the result was inconsistent, plus lots of duplicated code.
v3.6.0 (2026-10-25), the snapshot format changed, generate the snapshots again
- `write-arrow-ipc` writes the snapshot partitioned by day, `year=YYYY/month=MM/day=DD`, and only adds the replays that are not in it yet.
  The files are no longer at the root of the output directory, read them with `year=*/month=*/day=*/<file>.arrow`.
  `WriteArrowIpcProps` has a new `recreate` field (`--recreate`) and `SnapshotProgress` a new `AlreadyInSnapshot` variant.
  The days written by another version of s2protocol are removed and their replays parsed again, the replays of the
  other days are not parsed again.
- Not breaking: the `textures` feature embeds the button textures, see [Textures](#textures). The
  players of older replays without `working_set_slot_id` are matched to the lobby slots by order,
  previously they had no details and these replays were added again on every snapshot run.
v4.1.0 (2026-10-30), the snapshot format changed, generate the snapshots again
- `ext_replay_seconds` is in game seconds at the Faster speed, 22.4 loops per second (the new
  `GAME_LOOPS_PER_SECOND`), it was computed with 68.58 ms per loop and the times were 1.5 times
  longer than in the game. `convert_game_loop_to_seconds` and `convert_tracker_loop_to_seconds`
  changed accordingly.
- The snapshot files are compressed with zstd, about ten times smaller. Polars, pyarrow and DuckDB
  read them as before, readers built on the `arrow` crate need its `ipc_compression` feature.
  The version of s2protocol must change on release so that the uncompressed days are written
  again.
- The details have the difficulty, build and handicap of the lobby slot of each player
  (`lobby_slot_difficulty`, `lobby_slot_ai_build`, `lobby_slot_handicap`), to tell apart the
  games against the computer, i.e. Cheater 3 (Insane) Economic Focus, and the random seed of the
  game (`lobby_random_seed`).
- The replays of 4.10 (protocol 75689), i.e. the ones of AI Arena, are read: their lobby slots have
  a 7 bits handicap, a 13 bits length of the licenses and not the fields added in 4.11.
- The maximum number of teams of the game description was read with 5 bits instead of 4, the
  fields after it were misread until the next byte aligned field: `lobby_slot_map_size_y` was
  wrong for about half of the replays, and in the games where two users share a player (Archon
  mode) the name of the units of that player can be the other user.
- The move and attack commands of the bots are not in the cmd files: a user that gives more than
  1,000 commands per minute in a window of 1,000 commands is a bot for the rest of the game, see
  `arrow_store::bot_filter`. The bots of AI Arena give 1,100 to 16,800 commands per minute, a
  human a few hundred; their snapshot is about 70% smaller. In 4.10 the abilities have no names,
  the links 42 (move) and 45 (attack) are dropped.
- The commands of 4.10 (protocol 75689) are named: `assets/BalanceData/75689` has the units of
  94137 with the ability links of 4.10, one less until attack (46) and two less from the sprays
  (62), measured on the AI Arena replays, see `scripts/balance_data_75689`. The costs,
  strengths and weaknesses are the ones of 94137, `DERIVED_BALANCE_DATA_VERSIONS` lists it. The
  commands that 94137 has not, i.e. the ScanMove of the bots, have the names of sc2reader.
- The commands of the 5.0 versions without balance data are named with the balance data of 94137,
  `DERIVED_ABILITY_LINKS`: 80949 to 92440 have one ability less from HydraliskFrenzy (699), and
  trained the Banelings with ZerglingTrain (75, named MorphToBaneling as in 94137), 93139 to 93333
  have the links of 94137. They were 92% of the replays of the full snapshot (27 versions) and
  had no command named, now 97% to 100% of the commands of each version are named. Their units
  get the sizes and colors of 94137 too.
- A command given to a selection whose units do not all have the ability is named after the first
  one that has it, it was named after the last unit and often empty, and a command that no unit
  of the selection has, i.e. a move to Battlecruisers or the rally of a Barracks with a Command
  Center, after any unit of the balance data with that link: in the pro replays of 94137 99.9% of
  the commands are named instead of 86.8%. `tests/ability_names.rs` measures it on a directory.
- The map caches are downloaded into the cache path even when the directory does not exist (it
  was created by no one and every download failed), 4 at a time instead of all at once, through a
  temporary file. `cache_handles::download_caches` returns what was downloaded and the errors
  (`CacheDownloads`), they were printed to the console. `download_snapshot_caches` downloads the
  missing caches of an existing snapshot, the ones that failed before, and the CLI does it with
  `--source <snapshot> --cache-path <dir> cache download`. The `map_info_sha256` of the replays
  written while their caches were missing stays empty until the snapshot is generated again.

## Interacting with polars

### TODO:
- The UpdateTargetPoint Cmd arrow snapshot should contain the unit name, otherwise kindof to have an ability withouth it.

### Generating the IPC Arrow datasets

In the directory ipcs/ one .arrow file will be created per implemented data type and day, see
[Partitioned snapshots](#partitioned-snapshots).
The `--source` is the directory that contains the replay directory (Or a single file).
Multiple subdirectories are supported.
Files are processed using parallel operations.
For 17K replays (2.3 GBs) it takes 120 seconds to parse/transform/split them. YMMV, in this case only 10K files had valid init data (as in are supported protocol versions).

```bash
$ mkdir -p ipcs/
$ cargo run --example full -r --no-default-features --features=dep_arrow,tracing_off -- -v error --timing --source $HOME/SCReplaysOnNVMe --output $HOME/git/s2protocol-rs/ipcs/ write-arrow-ipc --process-max-files 1000000
36752 files have valid init data, processing...
Total time: 396.363965713s
$ du -sh ipcs
22G     ipcs
$ ls -ltra ipc
total 22239120
drwxr-xr-x  2 seb seb       4096 Aug 17 18:27 .
drwxr-xr-x 12 seb seb       4096 Aug 30 18:14 ..
-rw-r--r--  1 seb seb  117183106 Aug 30 20:45 lobby_init_data.ipc
-rw-r--r--  1 seb seb   10095306 Aug 30 20:45 details.ipc
-rw-r--r--  1 seb seb 2173322874 Aug 30 20:46 stats.ipc
-rw-r--r--  1 seb seb  179426002 Aug 30 20:47 upgrades.ipc
-rw-r--r--  1 seb seb 9958263882 Aug 30 20:48 unit_born.ipc
-rw-r--r--  1 seb seb 5859714578 Aug 30 20:49 unit_died.ipc
-rw-r--r--  1 seb seb 3479428050 Aug 30 20:50 cmd_target_point.ipc
-rw-r--r--  1 seb seb  995379658 Aug 30 20:51 cmd_target_unit.ipc
```

### Partitioned snapshots

The time it takes to generate a snapshot on different versions and machines is in [bench.md](bench.md).

The snapshot is partitioned by the day the games were played, each day directory has all the
`.arrow` files and its own `metadata.json`, the root has the `metadata.json` of the whole snapshot:

```
ipcs/
├── metadata.json
└── year=2026/
    └── month=04/
        ├── day=07/
        │   ├── details.arrow
        │   ├── stats.arrow
        │   ├── ...
        │   └── metadata.json
        └── day=14/
```

Running `write-arrow-ipc` again only adds the replays that are not in the snapshot yet: the details
of every day are read first and the replays are skipped by file name (without reading them) and by
sha256 (a replay copied to another directory). The new replays are added to their days, the
existing rows of a day are copied into the new files, which replace the previous ones once the day
is complete. The new replays get the `ext_fs_id`s that follow the highest one in the snapshot.

`--recreate` deletes the snapshot (the `.arrow` files, the `metadata.json` and the `year=*`
directories, other files are kept) and generates it from scratch. A snapshot written before the
partitions, with the files at the root, is always regenerated. A day written with a different
schema fails with an error asking to recreate the snapshot.

The files of all the days can be read with a glob, the partition columns are not needed since
`ext_datetime` has the date:

```bash
$ echo "SELECT COUNT(DISTINCT ext_fs_id) FROM read_ipc('ipcs/year=*/month=*/day=*/details.arrow')"|polars
```

`partitioned::partition_glob(DETAILS_ARROW_FNAME)` returns the glob relative to the snapshot root.
With polars, disable the hive partitions (`HiveOptions::new_disabled()`) to keep the schemas.

### Snapshot metadata

The `metadata.json` files record how the snapshot was generated, so that consumers can detect a
snapshot written by an incompatible version of s2protocol. At the root, `files` has the totals of
all the days, `partitions` the number of days and `added` the replays added by the last run:

```json
{
  "s2protocol_version": "3.5.10",
  "created_at": "2026-10-24T10:15:00Z",
  "elapsed_ms": 95021,
  "replays": 1470,
  "files": [
    { "name": "details.arrow", "bytes": 1062818, "records": 2940 },
    { "name": "stats.arrow", "bytes": 171035386, "records": 331000 }
  ],
  "partitions": 212,
  "added": 15
}
```

`SnapshotMetadata::read_from_dir` reads it and `SnapshotMetadata::is_compatible` checks that the
major and minor versions match the version of the crate reading it (patch versions do not change
the snapshot schemas). Snapshots generated before this file existed do not have it and should be
regenerated.

### Snapshot progress

`ArrowIpcTypes::handle_arrow_ipc_cmd_with_progress` takes an optional
`tokio::sync::mpsc::UnboundedSender<SnapshotProgress>` and sends the files found, the replays
already in the snapshot, the new replays with valid init data, each step as it starts, the replays processed (every 1%), and the size and records
of each file written. `SnapshotProgress` serializes with a `kind` tag, i.e.
`{"kind":"files_found","count":3}`, and does not need the `dep_arrow` feature, so a frontend can
deserialize it.

### Map caches

`CacheCollection::build_map_cache` reads the MapInfo, DocumentHeader, t3HeightMap, t3Terrain.xml
(with the ramps) and Objects of the map caches, and the `Minimap.tga` as `MapCache::minimap`, an
RGBA image. The image is a window of 256x256 cells centered on the playable area,
`Minimap::crop_to_playable` keeps only the playable area of the MapInfo. The `Resources` of the
placed mineral fields and geysers is read when the map sets it, `ObjectUnit::resources`.

### Jupyter Notebooks

The jupyter notebook with examples on how to interact with the data are available in [s2-polars-data-analysis](https://github.com/sebosp/s2-polars-data-analysis)

### polars-cli

```bash
$ cargo install polars-cli
$ echo "SELECT ext_fs_id, MAX(minerals_lost_army) FROM read_ipc('/home/seb/git/s2protocol-rs/ipcs/stats.ipc') GROUP BY ext_fs_id ORDER BY minerals_lost_army DESC;"|polars
┌───────────┬────────────────────┐
│ ext_fs_id ┆ minerals_lost_army │
│ ---       ┆ ---                │
│ u64       ┆ i32                │
╞═══════════╪════════════════════╡
│ 46076     ┆ 40078725           │
│ 50221     ┆ 26858850           │
│ 44624     ┆ 23460100           │
│ 25898     ┆ 14187775           │
│ 46088     ┆ 9492100            │
│ …         ┆ …                  │
│ 28169     ┆ 217075             │
│ 32336     ┆ 190500             │
│ 47505     ┆ 178256             │
│ 38654     ┆ 177675             │
│ 24162     ┆ 168120             │
└───────────┴────────────────────┘
& echo "SELECT ability, count() FROM read_ipc('/home/seb/git/s2protocol-rs/ipcs/cmd_target_unit.ipc') GROUP BY ability ORDER BY len DESC LIMIT 10"|polars
┌───────────────────────────────┬─────────┐
│ ability                       ┆ len     │
│ ---                           ┆ ---     │
│ str                           ┆ u32     │
╞═══════════════════════════════╪═════════╡
│                               ┆ 7457415 │
│ attack.Execute                ┆ 36377   │
│ Transfusion.Execute           ┆ 16004   │
│ SpawnLarva.Execute            ┆ 12291   │
│ ChronoBoostEnergyCost.Execute ┆ 10111   │
│ SCVHarvest.Gather             ┆ 9680    │
│ Build.Extractor               ┆ 4896    │
│ Build.Refinery                ┆ 3363    │
│ DroneHarvest.Gather           ┆ 2212    │
│ Build.Assimilator             ┆ 1754    │
└───────────────────────────────┴─────────┘
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
