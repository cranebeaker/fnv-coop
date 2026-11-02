# Couch co-op: several players on one PC

nv-rs co-op runs one viewer process per player. "Couch" mode is several of those processes on one
PC, each in its own window (split-screen) with its own controller, all in one session. A separate
launcher (Couchlink) starts and arranges them; this page is the contract between it and nv-rs.
nv-rs needs no extra code in that launcher beyond a recipe.

## Viewer flags

| Flag | Meaning |
|---|---|
| `--coop-couch SEAT/PLAYERS` | this process is local seat `SEAT` (0 to PLAYERS-1) of `PLAYERS` local players. Seat 0 hosts, the others join it on this PC. Uses the fixed local session token, so no join code is needed. |
| `--coop-window X,Y,W,H` | place the window (borderless) at this rectangle of the screen, in pixels |
| `--coop-gamepad N` | the controller this seat uses (0-based XInput index). The viewer records and logs it; routing each pad to its own window is the launcher's job (the recipe's `input.xinput`), the viewer does not filter pads itself |
| `--coop-voice off\|ptt\|vad` | microphone mode. A couch seat other than 0 gets `off` unless told otherwise, so one PC never opens several microphones into the session |
| `--coop-ptt-key KEY` / `--coop-ping-key KEY` | rebind push-to-talk (default Backslash) and ping (default middle mouse button) |
| `--coop-allow-mismatch` | developer override of the join compatibility check (see COOP.md "Join compatibility") |

The same options exist on `nv-launcher` as `--couch SEAT/PLAYERS --window X,Y,W,H --gamepad N
--voice MODE --ptt-key KEY --ping-key KEY --allow-mismatch`; `nv-launcher --print-command host ...`
prints the viewer command line they produce.

Only seat 0 should capture the microphone and play voice chat: the other players on the couch hear
each other in the room. Friends on other PCs join seat 0's host with a normal join code. The
session holds at most four players in total (host and three guests), local and remote together.

## `$PLAYERS` and `$SEAT`

* `$PLAYERS` is the number of local seats on this PC (1 to 4), not the size of the whole session.
  It is the second half of `--coop-couch`. A session of two couch players plus two friends has
  `$PLAYERS` = 2 on the couch PC and the friends' PCs know nothing about it.
* `$SEAT` is the 0-based seat of one process: seat 0 hosts, seats 1 and up join `127.0.0.1` on the
  host's port. `--coop-couch` takes it as its first half, `--coop-gamepad` as the controller index.
* The window rectangles for `$PLAYERS` windows on a `W` x `H` screen (the layout `nv-launcher`
  computes, `extras::layout`): 1 fills the screen; 2 are stacked (top, bottom); 3 are two on top
  and one centred below; 4 are quarters. A recipe may use its own.

## Recipe for Couchlink

The recipe is a Couchlink `schema: 1` JSON recipe. Fields this game needs (everything else is the
launcher's):

| Field | Value |
|---|---|
| `id`, `game` | `nv-rs-coop`, `Fallout: New Vegas (nv-rs co-op)` |
| `players` | `[2, 4]` |
| `mode` | `instances` (one process per seat) |
| `present` | `window` |
| `launch.exe` | the viewer, `nv-viewer.exe` (next to `nv-launcher.exe`) |
| `launch.args` | see below |
| `requires` | `nv-viewer.exe`, and the game's `Data` folder |
| `locks` | none (the viewer holds no single-instance mutex) |
| `input` | `xinput: true` |
| `window` | `borderless: true`, `settle_ms: 6000`, `wait_ms: 120000` |
| `tips` | seat 0 hosts; every player makes a character at Doc Mitchell's; push-to-talk is Backslash on seat 0 |

`launch.args` per seat (the launcher fills the variables):

```
seat 0:  "<Data>" --new-game --coop-host 27961 --coop-couch $SEAT/$PLAYERS --coop-name $PLAYER
         --coop-window $X,$Y,$WIDTH,$HEIGHT --coop-gamepad $SEAT --coop-log $LOGDIR/coop-$SEAT.jsonl
seat 1+: "<Data>" --coop-join 127.0.0.1:27961 --coop-couch $SEAT/$PLAYERS --coop-name $PLAYER
         --coop-window $X,$Y,$WIDTH,$HEIGHT --coop-gamepad $SEAT --coop-log $LOGDIR/coop-$SEAT.jsonl
```

Continuing a campaign: seat 0 gets the cell instead of `--new-game`, as `nv-launcher
--print-command host --continue ID` shows. Each seat runs in its own working folder so its saves do
not collide (`--home` of the launcher does this; a recipe uses `$SEATDIR`).

Rules the recipe relies on:

1. Every seat starts at once; guests keep retrying the join for 90 seconds, so start order does not
   matter.
2. `--coop-couch` selects the fixed local session token, which is only valid for loopback, and the
   host is reachable by the other seats on `127.0.0.1`. A couch session is therefore not protected
   from other PCs on the same network unless the OS firewall blocks the port: keep the port closed
   to the LAN or add friends through a normal join code instead.
3. A seat that exits is a guest leaving (it can rejoin by being started again); seat 0 exiting ends
   the session, as any host does.

What is not done: one window with several views (true shared-screen split). Each seat is its own
process with its own window, which costs one copy of the game's memory per seat (the 4-viewer scenario in
tools/coop-run measures what that is on the target PC).
