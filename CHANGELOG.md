# Changelog

The changes of each version of s2protocol, newest first. A change of the snapshot format bumps
the minor version, see [Snapshot metadata](README.md#snapshot-metadata): the applications
compare the major and minor versions of the snapshot with theirs and ask for a Rebuild.

## 5.1.0 (unreleased)

The snapshot format changed, generate the snapshots again.

- `cache_handles::graphics` and `cache_handles::m3`: the objects of a map that use a model of its
  caches, with the geometry and the diffuse textures of the models.
- The cache handles that are a `Standard Data: <mod>` text file instead of an archive are skipped
  without an error, with `MPQParserError::StandardData` of nom-mpq 2.0.13.
- The [aggregations](README.md#aggregations): per game in each day, and usage tables per day, month
  and year, `arrow_store::aggregates`. `write-arrow-ipc` writes the ones that are missing without
  reading the replays again, `find_gaps` lists them.
- A row of the cmd files is a command, it was a row per unit of the selection. The new `unit_names`
  has the unit types of the selection and `selection_size` its units, `unit_name` is the first unit.
  The rows of the commands of a full snapshot of pro games went from 118 million to 13.7 million;
  on AI Arena the commands have few units, a day has about a quarter less. A query that counted
  `unit_name` per row now counts the commands, not the units that received them.
- The rows are written in record batches of about 256 thousand rows (`ipc_writer::RECORD_BATCH_ROWS`),
  it was a batch per replay. On a day of 2,376 AI Arena games a filtered scan of a file is 2.5 times
  faster.
- The snapshot generation has a new step, aggregating the days, months and years.
- New files, so that the applications compute their views from the snapshot without reading the
  replays again: `unit_init.arrow` (the structures started), `cmd_no_target.arrow` (train,
  research, morph, stop...), `game_actions.arrow` (per player and minute: commands, camera moves,
  control groups, selection changes), `game_selections.arrow` (per player, minute and unit type
  selected) and `messages.arrow` (the chat).
- `details.arrow` has `player_scaled_rating`, the rating (MMR) of the player when the game started.
- The right clicks are in the cmd files, with an empty `ability`: the state now gives the units of
  the selection of a command without an ability (`UnitChangeHint::Abilities`), they were left out
  and the commands with them, about 40% of the commands of a pro game. The bots' ones are dropped
  with their moves and attacks. The commands whose selection is not known have a row too, with an
  empty `unit_name`: the cmd files have every command of the humans.

## 5.0.2 (2026-11-03)

Not breaking, the snapshot format did not change.

- The ids of the new replays (`ext_fs_id`) follow the time of their game, then their file name,
  instead of the order of the files: a snapshot generated again (`--recreate`) has the same ids.

## 5.0.1 (2026-11-01)

Not breaking, the snapshot format did not change.

- `snapshot::snapshot_state` tells whether a snapshot is ready, being generated or was
  interrupted, i.e. the application was closed: the generation writes `in_progress.json` in the
  snapshot directory and keeps it locked, the lock dies with the process and the file stays. A day
  written after the root metadata, without its metadata or with files still being written is an
  interrupted generation too. A second generation of the same directory cannot start while one
  runs.

## 5.0.0 (2026-11-01)

The snapshot format did not change.

- The protocol types are derived with [serde_mpq](serde_mpq/README.md), see
  [The protocol types and serde_mpq](README.md#the-protocol-types-and-serde_mpq): the `generator`
  module and the `generate` command of the CLI are gone, `bit_packed_decoder` and
  `versioned_decoder` re-export `serde_mpq::bit_packed` and `serde_mpq::byte_aligned`, the
  parsers return `serde_mpq::Error` (`S2ProtocolError::SerdeMpq`), and the types can be written
  back.
- The bit arrays are `serde_mpq::BitArray`, the integer Blizzard's `read_bits` reads with its
  length: `GameSelectionMaskType.value` (unit `i` of the selection is bit `i`) and, in the
  `InitData`, `allowed_controls` and `allowed_ai_builds` are the big endian bytes of that integer.
  Previously these were read in chunks of 8 bits from the current bit, which is the same integer
  only when the array starts at a byte boundary: the selection masks of more than 8 units that
  did not were applied to the wrong units.
- Fixes found by checking every type against Blizzard's `protocol87702.py`: the `m_sequence` of
  `SCommandManagerStateEvent` was read one bit early (its value had the presence bit as high
  bits); 16 blob fields (`m_hotkey_profile` of the user options, the bank events, save game,
  cutscene and catalog events) were read without the byte alignment and some with the wrong
  number of length bits; `GameCCheatString`, `GameCTriggerChatMessageString` and
  `GameCGameCacheName` have 10 length bits, not 11. An empty string is empty, it was a NUL
  character: the `clan_tag`, `hero`, `skin` and `mount` of the players without one are `""` in
  the snapshot instead of `"\u0000"`, regenerate the snapshot to update them.

## 4.1.1 (2026-10-30)

- The replays added while their map caches could not be downloaded, i.e. because the caches
  directory did not exist, had no `map_info_sha256` and kept none. `write-arrow-ipc` now
  downloads their missing caches first and parses again the days of the replays whose MapInfo
  can be read, `partitioned::remove_partitions_missing_map_info`. The replays whose caches cannot
  be downloaded or have no MapInfo, i.e. the local maps of AI Arena, are not parsed again.

## 4.1.0 (2026-10-30)

The snapshot format changed, generate the snapshots again.

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

## 3.6.0 (2026-10-25)

The snapshot format changed, generate the snapshots again.

- `write-arrow-ipc` writes the snapshot partitioned by day, `year=YYYY/month=MM/day=DD`, and only
  adds the replays that are not in it yet. The files are no longer at the root of the output
  directory, read them with `year=*/month=*/day=*/<file>.arrow`. `WriteArrowIpcProps` has a new
  `recreate` field (`--recreate`) and `SnapshotProgress` a new `AlreadyInSnapshot` variant. The
  days written by another version of s2protocol are removed and their replays parsed again, the
  replays of the other days are not parsed again.
- Not breaking: the `textures` feature embeds the button textures, see
  [Textures](README.md#textures). The players of older replays without `working_set_slot_id` are
  matched to the lobby slots by order, previously they had no details and these replays were
  added again on every snapshot run.

## 3.4

- There is only one iterator provided now, that goes through both the Tracker and Game events to
  build the game state. Previously two iterators were provided that build partial state only for
  either tracker/game events and the result was inconsistent, plus lots of duplicated code.

## 3

- Feature `arrow` has been renamed to `dep_arrow` as we now use the `arrow` crate.
- Previously the generated arrow IPC files relied on a sha256 of the file as "key", this wasted a
  lot of space and has been changed into a sequential id. As a result, queries must use the new
  `ext_fs_id`, which also means all the files must be generated in bulk and are no longer
  independent. The IPCs are then valid as a snapshot generated as a whole.
