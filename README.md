[![Crates.io](https://img.shields.io/crates/v/s2protocol.svg)](https://crates.io/crates/s2protocol)
[![Workflow Status](https://github.com/sebosp/s2protocol-rs/workflows/Rust/badge.svg)](https://github.com/sebosp/s2protocol-rs/actions?query=workflow%3A%22Rust%22)

# s2protocol-rs

A nom parser for the Starcraft 2 Protocol Replay format.

## Generating protocol-specific code:

The rust code for the protocols versions available were generated using:

```bash
mkdir src/versions/protocol89720/
RUST_LOG_SPAN_EVENTS=full RUST_LOG=debug cargo watch -i src/versions/protocol89720/mod.rs -x 'run -- --source ../s2protocol/json/protocol89720.json generate --output src/versions/protocol89720/mod.rs'
# Add the new module to src/versions/mod.rs
# Run rust format on the new src/versions/protocol87702/mod.rs file
# cargo check, cargo build, etc
# Additionally some code to transform from Protocol-Specific to Protocol-Agnostic was added, TODO: Add to generator.rs
```

## version compatibility.
For the current parsed versions, it seems they are compatible enough and so for now a symlink is created for version specific to version 87702.
Even tho the structs are called the same, the bit/byte sizes may be different.

In order for this to work, the repo from Blizzard must be cloned at `../s2protocol`.

## JSON Sources
[Blizzard/s2protocol repo](https://github.com/Blizzard/s2protocol)

## Motivation
The goal is to learn how to parse binary files format with `nom` and to learn
how the Starcraft 2 Replay file is so incredibly small for the amount of
information it packs.

From the available data, generative art can be created, for example
by using 
- [rerun](https://github.com/rerun-io/rerun)
- [nannou](https://github.com/nannou-org/nannou) (PoC missing)
- [bevyengine/bevy](https://github.com/bevyengine/bevy) can be used to see:
  - An Enhanced Replay Minimap
  - Additional statistics.

See the repo [sebosp/swarmy](https://github.com/sebosp/swarmy)
for an example on how this can look like.

## Status

Tho the status below is something to add, the focus is going to be on using the Game and Tracker events to generate visualization.
From time to time as versions come out I will try to generate the files.

- [x] Replay Tracker and Game Events for protocol87702
- [x] Replay Tracker and Game Events for protocol88500
- [x] Replay Tracker and Game Events for protocol89634
- [x] Replay Tracker and Game Events for protocol89720
- [x] Parsing unit movement is done.
- [x] Decoding the tag/recycle done to match Game Events.
- [x] Game Events are parsed (tho some that seem irrelevant are skipped).
- [x] Read the version and from the version call the correct module so that we can support multiple modules.
- [ ] Add the remaining Tracker/Game event types.
- [ ] Support for MPQ embedded file: `replay.gamemetadata.json`
- [ ] Support for MPQ embedded file: `replay.details`
- [ ] Support for MPQ embedded file: `replay.initData`
- [ ] Support for MPQ embedded file: `replay.message.events`
- [ ] Support for MPQ embedded file: `replay.attributes.events`
