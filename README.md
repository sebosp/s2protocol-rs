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
- jupyter notebooks in [s2-polars-data-analysis](https://github.com/sebosp/s2-polars-data-analysis)
- the replay analyser [swarmy](https://github.com/sebosp/swarmy), and its 3D map viewer [swarmy-bevy](https://github.com/sebosp/swarmy-bevy)
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

## Changelog

The changes of each version, and the ones that need the snapshots to be generated again, are in
[CHANGELOG.md](CHANGELOG.md).

## AI Arena replays

The replays of [AI Arena](https://aiarena.net), the StarCraft II bot ladder, are planned to be
supported as a first class source: the client of AI Arena is 4.10 (protocol 75689), whose init
data is read since 4.1.0, and its bots give thousands of commands per minute, so their move and
attack commands are left out of the snapshot (`arrow_store::bot_filter`). What is left to do:
the maps of AI Arena are local maps that are not on the Blizzard depots, so their replays have no
`map_info_sha256` and no map cache; and a few of the replays cannot be read yet. A snapshot of tens
of thousands of bot games has many times the command rows of a human ladder, the applications read
its [aggregations](#aggregations) instead of those rows.

## Snapshots

A snapshot is a directory of Arrow IPC files (`.arrow`, compressed with zstd) written by
`write-arrow-ipc` from a directory of replays, that the applications and the notebooks query
with Polars, pyarrow or DuckDB. The files, one of each per day the games were played, see
[Partitioned snapshots](#partitioned-snapshots):

| File | A row per |
|---|---|
| `details.arrow` | player of a game: the names, races, result, map, patch and lobby slot |
| `stats.arrow` | `PlayerStats` tracker event, every 10 seconds of each player |
| `upgrades.arrow` | upgrade completed |
| `unit_born.arrow` | unit born, finished (UnitDone) or changing type, with its position |
| `unit_init.arrow` | structure whose construction started (UnitInit), same columns as `unit_born.arrow`; its first `unit_born` row with the same tag is when it finished |
| `unit_died.arrow` | unit died, with its position and killer |
| `cmd_target_point.arrow` | command aimed at a point, with the named ability and the units of the selection |
| `cmd_target_unit.arrow` | command aimed at a unit, with the named ability and the units of the selection |
| `cmd_no_target.arrow` | command without a target: train, research, morph, stop, hold... |
| `game_actions.arrow` | player and minute of a game: commands, camera moves, control group assigns and recalls, the groups assigned (a bit each) and selection changes |
| `game_selections.arrow` | player, minute and unit type selected: the selections, the units, the first and last second |
| `messages.arrow` | chat message, with the sender and the recipients |

The record batches have about 256 thousand rows. A command is one row: `unit_name` is the first
unit of the selection, `unit_names` its unit types separated by commas and `selection_size` how
many units it had. The right clicks (moves and attacks without a named ability) are commands with
an empty `ability`. The camera moves, control groups and selections are counted per minute in
`game_actions.arrow` and `game_selections.arrow`, a row each would be tens of thousands of rows
per game.

The applications read the snapshot, not the replays: what a view needs of the games is in these
files or in the [aggregations](#aggregations), a question the files cannot answer adds a column or
a file here.

The rows of a game share its `ext_fs_id`, the event rows have the game loop and the game
seconds (`ext_replay_loop`, `ext_replay_seconds`) of the event.

### Generating a snapshot

`--source` is the directory of the replays (subdirectories included) or a single replay,
`--output` the snapshot directory and `--cache-path` the directory of the map caches (`.s2ma`),
downloaded from the Blizzard depots when they are missing. Without `--cache-path` the caches are
downloaded into the working directory. The replays are read in parallel:

```bash
$ cargo run --example full -r --no-default-features --features=dep_arrow,dep_tokio,tracing_off -- \
    -v error --timing \
    --source $HOME/SC2Replays/ \
    --output $HOME/git/s2protocol-rs/ipcs/ \
    --cache-path $HOME/SC2Replays/swarmy/ \
    write-arrow-ipc --process-max-files 1000000
Located 59696 matching files by extension
0 replays are already in the snapshot, 6413 are copies of another replay, 16889 could not be read, 36394 new replays have valid init data, processing...
Wrote 36394 replays into 2095 days after 97.995619873s
Aggregated 2095 days, 76 months and 8 years after 102.119933809s
Total time: 102.618965264s
$ du -sh ipcs
4.2G    ipcs
```

The times per version and machine are in [bench.md](bench.md). `snapshot::snapshot_state` tells
whether a snapshot is ready, being generated (`in_progress.json`, locked by the process) or was
interrupted, the applications ask for a Rebuild in that case; the generation reports its steps,
see [Snapshot progress](#snapshot-progress).

### Aggregations

Most questions do not need the hundreds of millions of event rows. The generation writes, from the
files of each day and without reading the replays again, see `arrow_store::aggregates`:

| File | Where | A row per |
|---|---|---|
| `game_abilities.arrow` | each day | game, player and ability: uses, first use |
| `game_units.arrow` | each day | game, player and unit: made, lost, kills, resources killed |
| `game_players.arrow` | each day | game and player: tracker player id, length of the game |
| `game_minutes.arrow` | each day | game, player and minute: workers, supply, income, army |
| `ability_usage.arrow` | each day, month, year | period, protocol version, race, player, ability |
| `unit_usage.arrow` | each day, month, year | period, protocol version, race, player, unit |
| `player_usage.arrow` | each day, month, year | period, protocol version, race, player: games, wins |

```bash
$ echo "SELECT ability, SUM(uses) FROM read_ipc('ipcs/year=*/ability_usage.arrow') GROUP BY ability"|polars
$ echo "SELECT period, SUM(uses) FROM read_ipc('ipcs/year=*/month=*/ability_usage.arrow') WHERE ability = 'SpawnLarva.Execute' GROUP BY period"|polars
```

A day is computed from its files, a month from its days and a year from its months. An
`aggregates.json` in each directory tells what it was computed from: `aggregates::find_gaps` lists
the periods that are missing or older than their days, i.e. after an interrupted generation, and
running `write-arrow-ipc` again only computes those.

### Partitioned snapshots

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

The files of all the days are read with the glob of the partitions:

```bash
$ cargo install polars-cli
$ echo "SELECT ext_fs_id, MAX(minerals_lost_army) FROM read_ipc('ipcs/year=*/month=*/day=*/stats.arrow') GROUP BY ext_fs_id ORDER BY minerals_lost_army DESC;"|polars
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
$ echo "SELECT ability, count() FROM read_ipc('ipcs/year=*/month=*/day=*/cmd_target_unit.arrow') GROUP BY ability ORDER BY len DESC LIMIT 10"|polars
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


## The protocol types and serde_mpq

The types of each protocol version, `src/versions/protocol87702/mod.rs`, are structs and enums
with `#[mpq(...)]` attributes and the derives of [serde_mpq](serde_mpq/README.md), a crate of this
repository: `MPQBitPackedDeserialize` and `MPQBitPackedSerialize` for the game events, message
events and init data, `MPQByteAlignedDeserialize` and `MPQByteAlignedSerialize` for the header,
details and tracker events. The declaration order of the fields is the wire format of the bit
packed types, as in `deku` or `binrw`; the byte aligned ones are matched by `#[mpq(tag = n)]`.

The derives generate a `parse` method and a `parse_<field>` method per field with `tracing`
spans, so a replay can be followed field by field with `RUST_LOG=trace`, and they write the types
back: `src/versions/round_trip_tests.rs` parses every section of the test replays, writes it and
compares the bits.

Until 4.1.1 the parsers were generated from the JSON of
[Blizzard/s2protocol](https://github.com/Blizzard/s2protocol) by a generator in this crate, 17,034
lines for protocol 87702 against 2,634 lines of types now, the generator is gone. The types were
checked against the `typeinfos` of Blizzard's `protocol87702.py` field by field (names, order,
tags, bits, offsets, lengths), which found the fixes listed in the 5.0.0 changes. A new protocol
version is added by declaring the types that differ, as `protocol75689` does.

## JSON Spec Sources

[Blizzard/s2protocol repo](https://github.com/Blizzard/s2protocol)
