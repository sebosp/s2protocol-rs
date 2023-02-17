[![Crates.io](https://img.shields.io/crates/v/s2protocol.svg)](https://crates.io/crates/s2protocol)
[![Workflow Status](https://github.com/sebosp/s2protocol-rs/workflows/main/badge.svg)](https://github.com/sebosp/s2protocol-rs/actions?query=workflow%3A%22main%22)

# s2protocol-rs

A nom parser for the Starcraft 2 Protocol Replay format.

## Generating protocol-specific code:

The rust code for the protocol87702 was generated by running:

```bash
mkdir src/versions/protocol87702/
RUST_LOG_SPAN_EVENTS=full RUST_LOG=debug cargo run -- --source ../s2protocol/json/protocol87702.json generate --output src/versions/protocol87702/mod.rs
# Add the new module to src/versions/mod.rs
# Run rust format on the new src/versions/protocol87702/mod.rs file
# cargo check, cargo build, etc
# Additionally some code to transform from Protocol-Specific to Protocol-Agnostic was added, TODO: Add to generator.rs
```

In order for this to work, the repo from Blizzard must be cloned at `../s2protocol`.

## Sources
[Blizzard/s2protocol repo](https://github.com/Blizzard/s2protocol)

## Motivation
The goal is to learn how to parse binary files format with `nom` and to learn
how the Starcraft 2 Replay file is so incredibly small for the amount of
information it packs.

From the available data, generative art can be created, for example
by using [nannou](https://github.com/nannou-org/nannou) (PoC missing)
[bevyengine/bevy](https://github.com/bevyengine/bevy) can be used to see:
- An Enhanced Replay Minimap
- Additional statistics.
See the repo [sebosp/swarmy](https://github.com/sebosp/swarmy)
for an example on how this can look like.

## TODO
Read the version and from the version call the correct module so that we can support multiple modules.

## Status

- [x] Replay Tracker for protocol87702
- [x] Replay Tracker for protocol89634
- [ ] Calculating the unit movement is not done yet,
- [ ] Decoding the tag/recycle is missing to match Game Events.
- [ ] Game Events are not parsed yet.
