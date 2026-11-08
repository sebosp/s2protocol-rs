# Working on s2protocol-rs

Instructions for the coding agents (Claude Code reads them through `CLAUDE.md`) and the people
working on s2protocol-rs, the StarCraft II replay parser and Arrow snapshot writer.

## The projects that depend on it

- **swarmy** (the replay analyser) and its `swarmy --serve` backends read the snapshot and the
  map caches.
- **swarmy-bevy** reads replays and map caches.
- **s2-polars-data-analysis** (Jupyter notebooks) queries the snapshot with Polars.
- **cooper** reads replays.

## When s2protocol-rs changes

A change of the data (columns, values, the ids of the replays), of the snapshot layout or of the
behavior is not done until its dependents follow, in the same piece of work:

1. Bump the version (a schema change bumps the snapshot compatibility, so that the apps ask for
   a Rebuild) and add a line to `CHANGELOG.md`.
2. Update the dependents to the new version: build them, run their tests, and fix the queries
   that used what changed (swarmy's queries, the notebooks of s2-polars-data-analysis, run them).
3. Rebuild the snapshots that the apps and servers use, and check a few queries on them.
4. Update the docs and screenshots of the dependents that show what changed, briefly.

Write the decisions taken with the author here, explicitly, so that the next agent follows them.

## Decisions

- 2026-11-04: views read the aggregations (`arrow_store::aggregates`), not the event files, for
  anything that spans games. A new question over many games adds a column or table there.
- 2026-11-04: measured on an AI Arena day and on the pro snapshot, and not adopted: zstd stays
  (lz4 and uncompressed scan no faster), day partitions stay (month files were 9% faster on 2,095
  days and rewrite a month for each new replay), Parquet stays out, dictionary encoding of the event
  files stays out (the IPC file format has no dictionary replacement, Polars would then need a cast
  in every string query).
- 2026-11-04: the strings stay Utf8, not string views: views were 17% smaller on the pro snapshot
  but slower in every Polars query measured, and DuckDB's `read_arrow` (1.5.2) fails on them.
- 2026-11-08: the applications never read the replays to answer a query, only to find the files
  missing from the snapshot (and to play a single replay in Rerun or swarmy-bevy). What a view
  needs from the games is exported here as facts: a row per event when a game has a few hundred
  of them, counts per player and minute otherwise (`game_actions`, `game_selections`). Values that
  need the game state are computed in the event pass of `ReplayEventRows::collect`.

## Code

- `cargo fmt`, `cargo clippy` without warnings and `cargo test` before a change is done; the
  tests on real replays and snapshots are `#[ignore]` tests with their paths in environment
  variables.
- Compile on the desktops, not on the laptop.
