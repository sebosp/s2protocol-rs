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
   a Rebuild) and add a line to the changelog of the README.
2. Update the dependents to the new version: build them, run their tests, and fix the queries
   that used what changed (swarmy's queries, the notebooks of s2-polars-data-analysis, run them).
3. Rebuild the snapshots that the apps and servers use, and check a few queries on them.
4. Update the docs and screenshots of the dependents that show what changed, briefly.

Write the decisions taken with the author here, explicitly, so that the next agent follows them.

## Code

- `cargo fmt`, `cargo clippy` without warnings and `cargo test` before a change is done; the
  tests on real replays and snapshots are `#[ignore]` tests with their paths in environment
  variables.
- Compile on the desktops, not on the laptop.
