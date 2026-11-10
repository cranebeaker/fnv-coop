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

## M5: the live ladder (players in the world)

Same two windows as M3 (host with `NV_COOP=host`, guest with `NV_COOP=join:<code>`), logs kept with
`2>&1 | Tee-Object host-log.txt` and `guest-log.txt`.

1. Guest: press New Game first and wait until something is on screen (or a minute passes).
2. Host: start a new game and play until you stand in Doc Mitchell's house and can walk.
3. Host: walk around the room for a minute, then go outside and walk a little.
4. Close both, then in the folder run:

```powershell
Select-String "COOP-RUNG|co-op live" host-log.txt
Select-String "COOP-RUNG|co-op live|co-op gate" guest-log.txt
Select-String "co-op guest:" guest-log.txt | Select-Object -Last 2
```

Expected on the guest: `COOP-RUNG 1` to `5` with `PASS`; on the host rungs 1, 2, 4 and 5. On screen:
the guest stands in the host's room, and each window shows a figure where the other player is (it slides;
it does not walk yet).

If the game stops, the last `COOP-RUNG n entering` line names the step. To retry below it, set
`$env:NV_COOP_LIVE = "warp"` (no avatars) or `"log"` (nothing in the game is touched) in both windows.
If rung 4 fails, another base form can be tried with `$env:NV_COOP_AVATAR = "<hex form id>"`.

## M4 and M6: the rest of the world (records, doors, locks)

Same two windows as M5. Nothing here has run on a PC yet (docs/coop/SPECS.md S12).

1. Both in Goodsprings. Host: open and shut a door, pick a lock, take everything from a container,
   read a note. Guest: look at the same door, lock and container.
2. Guest: open a door and pick another lock. Host: look at them.
3. Every 5 seconds each window prints a `co-op host world:` or `co-op guest world:` line. Expected
   on the guest: `records loaded` rising, `own record differs after 0`, `refused 0`.

If a game stops while a record loads, set `$env:NV_COOP_WORLD = "doors"` in both windows (doors and
locks only) and run again; `"off"` takes this layer out. Report both logs, especially the
`co-op guest world: form ...` lines: they name each form whose record did not read back the same.
