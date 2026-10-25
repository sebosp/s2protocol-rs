# Snapshot benchmarks

The time it takes `write-arrow-ipc` to generate a snapshot, per s2protocol-rs version, to compare
the versions and the machines. The replays are read from a local NVMe, an NFS mount is much slower.

The command, from the s2protocol-rs checkout, the release build is done before so it is not timed:

```bash
cargo build --example full -r --no-default-features --features=dep_arrow,tracing_off
time cargo run --example full -r --no-default-features --features=dep_arrow,tracing_off -- \
  -v error --timing \
  --source $HOME/SCReplaysOnNVMe/ \
  --output $HOME/git/s2protocol-rs/ipcs/ \
  --cache-path $HOME/SCReplaysOnNVMe/swarmy/ \
  write-arrow-ipc --process-max-files 1000000 [--recreate]
```

- **Full**: `--recreate`, the snapshot is deleted and every replay is read.
- **Incremental**: only the replays that are not in the snapshot yet are read, see
  [Partitioned snapshots](README.md#partitioned-snapshots).

The output prints the replays found, already in the snapshot, copies of another replay (the same
replay in two directories, by sha256) and that could not be read, then `Wrote <n> replays into
<days> days after <time>` and `Total time`. The `metadata.json` of the snapshot has the rows and
bytes of each file and `elapsed_ms`.

## Machines

| Name | CPU | Threads | RAM | Disk | OS |
|---|---|---|---|---|---|
| corvid | Intel Core i9-13900KF | 32 | 62 GB | NVMe | Arch Linux, kernel 7.2.7 |
| local | AMD Ryzen 9 9950X | 32 | 60 GB | NVMe | CachyOS, kernel 7.3.0 |

## Results

| Date | s2protocol-rs | Machine | Mode | Files found | Copies | Unreadable | Replays written | Days | Size | Total time | Replays/s |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 2026-10-25 | 3.5.10 `df7e640` | corvid | Full | 59,696 | 6,413 | 17,132 | 36,151 | 2,090 | 30 GB | 340.1 s (5m40s) | 106 |
| 2026-10-25 | 3.5.10 `df7e640` | corvid | Incremental, nothing new | 59,696 | 6,413 | 17,132 | 0 | 0 | 30 GB | 45.4 s | - |
| 2026-10-25 | 3.5.10 `8422fc6` | local | Full, `SC2Replays/PRO/RSL` | 834 | 180 | 0 | 654 | 61 | - | 17.8 s | 37 |

Notes on the corvid full run (`df7e640`):

- The map caches step (`populate_map_info_digest_from_caches`) took 20 s of the 340 s, it downloads
  the missing map caches; 16 of them are no longer on the Blizzard depots (404).
- `user` 19m31s for 5m40s `real`, about 3.4 of the 32 threads busy on average: the replay headers
  and the map caches are read before the events, which are written in batches of 24 days so that
  the open files stay under the limit (7 files per open day).
- The rows of the snapshot: details 91,067, stats 11.4M, upgrades 3.1M, unit_born 133.2M,
  unit_died 92.2M, cmd_target_point 54.7M, cmd_target_unit 10.7M. unit_born is 13.9 GB of the
  30 GB.
- The incremental run reads the headers of the replays that are not in the snapshot by file name,
  the 6,413 copies and 17,132 unreadable files are read every time, that is most of its 45 s.

Before `8422fc6` the full run failed on corvid with `Too many open files (os error 24)` after 80 s,
every day written kept its 7 files open until the end. Before `df7e640` the 268 replays whose
players have no `working_set_slot_id` (2021) had no details and were added again by every
incremental run.

## TODO

- Remember the files that could not be read and the copies, i.e. in the root `metadata.json`, so
  that the incremental run does not read them again.
- Use more of the threads: read the events of the replays while the map caches are downloaded.
