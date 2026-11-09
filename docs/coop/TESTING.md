# Testing co-op on a PC

What only a Windows PC with Fallout: New Vegas 1.4.0.525 can check, step by step. Everything else
runs with `cargo test` (docs/coop/SPECS.md S7 lists what).

## Build

```sh
git clone https://github.com/Alexanger-dev/fnv-coop -b coop-next
cd fnv-coop
cargo build --profile play -p nv-viewer
```

The program is `target/play/nv-viewer.exe`. It reads the game from its `Data` folder; nothing is
written there.

## M1: the change journal

```powershell
$env:NV_D3D9 = "real"; $env:NV_SOUND = "out"; $env:NV_COOP = "journal"
target\play\nv-viewer.exe "<path to Fallout New Vegas>\Data"
```

Start a new game and leave Doc Mitchell's house. Expected in the console: lines
`coop journal: quest ... stages changed` while the opening's quests advance, and counts of other
changed forms. No such lines: report the console output.

## M3: two games, one story

On one PC (two windows) or two PCs on one network:

1. Host: `$env:NV_COOP = "host"` (UDP port 27961; `host:PORT` for another), then start nv-viewer as
   above. The console prints `co-op: hosting on UDP port 27961; join code ...: NVRS-...`.
2. Guest: `$env:NV_COOP = "join:NVRS-..."` with the host's code, then start nv-viewer.
   Expected: `co-op: joined as player 1`.
3. Both start a new game. The host plays the opening (Doc Mitchell's questions, leaving the house,
   Sunny Smiles); the guest stands by.
4. Every 5 seconds the guest prints a status line. Expected throughout:
   `desyncs 0`, `game differs from the host's story on 0 entries` (it may show a few for a moment),
   `refused 0`, and `story scripts not run` rising.
5. Open the Pip-Boy's quest list on both: the same quests, stages and objectives.

Over the internet: forward the host's UDP port and replace the address in the join code with the
host's public address (the launcher does this later).

Report the console output of both windows, especially any `refused` above 0 or a status line whose
differing entries never go back to 0.
