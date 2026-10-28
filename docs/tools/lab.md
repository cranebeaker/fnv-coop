# lab.py: every co-op mechanic checked in about 20 minutes

Private doc. Contract: `docs/LAB.md`. Code: `tools/lab/` (the plugin generator is `tools/lab/esp/`, another tool). It ran in a game on the second PC from 2026-10-27 (what that found is under "Uncertain until it runs in a game" and "Measured in game"); the whole
runner is tested on a fake two-copy world (`python -I tools/lab/run_tests.py` prints ALL OK).

## Run it

```
python -I tools/lab/lab.py run --pairs 3                      # all rooms over 3 pairs of copies, spawn first on every pair
python -I tools/lab/lab.py run --pairs 1 --rooms fight,loot   # some rooms (spawn always runs first, it puts the pair into the lab)
python -I tools/lab/lab.py smoke                              # spawn room only, about 3 minutes
python -I tools/lab/lab.py summary out/lab/<stamp>            # print a finished run's summary again
python -I tools/lab/lab.py down                               # stop the copies a run left running (only ones this tool started)
python -I tools/lab/lab.py run --pairs 3 --net good_wifi,bad_wifi   # the clean pass, then the rooms again behind each chaos profile (see "The network pass")
python -I tools/lab/lab.py run --pairs 3 --net good_wifi,bad_wifi,mobile_tether,transatlantic   # the standard network run: clean + 4 profiles (second PC: `lab_run.ps1 -Net std`)
python -I tools/lab/lab.py run --split                        # two-PC smoke: host on the second PC, client on this PC (see "The two-PC smoke")
```

Options: `--budget-min 20` (the rooms of one pair together; counted from its first room, so boot and the save load are not in it; `wall_seconds` in results.json has the total), `--room-budget 120` (seconds per room, a hard time box; spawn has its own larger box of 240 s, because it also loads the save and enters the lab: `lab_rooms.SPAWN_BOX_S`), `--layout out/lab/layout.json`, `--confirm K` (rooms that passed
run K more times, each on a fresh load; a failed room is never rerun), `--param room.key=value` (change a threshold for one run), `--keep` (leave the copies
running), `--fake` (the game-free run) with `--bug NAME` (one fake bug on, the matching room must fail; names in `tools/lab/labfake.py`).

Needs: `out/lab/layout.json` and `out/lab/NVCoopLab.esp` from `tools/lab/esp`, a built `out/L1*/nvcoop.dll`, Steam running, the owner not playing (the same
guards as `prove.py`, which this tool reuses: slot lease owner `lab`, RAM guard, `run.py provision`, boot, saves, crash watch). The lab plugin goes only
into the test copies: before each boot the runner calls `python -I tools/lab/esp/install_lab.py --instance-dir <copy>`; without that script the pair is BLOCKED with
that reason.

On the second PC: `tools/bench/remote/task.ps1 -Script C:/fnv/fnv-coop/tools/bench/remote/lab_run.ps1`, which runs 3 pairs and writes `C:\fnv\lab_done.txt` when it ends
(`lab_out.txt` holds the summary, `lab_err.txt` the progress).

## The network pass (`--net`)

`lab.py run --net good_wifi,bad_wifi` (any names in `tools/chaos/profiles.json`; `python -I tools/chaos/chaos.py profiles` lists them) runs the clean pass as always, then
for each profile the rooms again: the save is loaded again (the clean pass left doors open, hostiles dead and the quest at stage 30), both copies `coc` into the lab, and then the
client copy leaves the session and joins the host THROUGH a chaos relay (`tools/chaos/chaos_pair.py`'s ChaosPair, in this process, one relay per pair in front of the host's
real port; the relay starts on `lan` and switches to the profile once the client has joined). The relay's packet counters must show traffic in both directions, or the pair is
BLOCKED ("nothing crossed the relay": a client that bypassed it would measure the clean link). After the pass the client joins the host's real port again and the relay stops.
`--net-seed` (default 7) seeds the relay's loss and jitter. `--total-min` (default 15) caps all passes of one pair together; a pass that has under 30 s left is a TIMEOUT row.
On the second PC: `task.ps1 -Script C:/fnv/fnv-coop/tools/bench/remote/lab_run.ps1 -Wait 5 -ScriptArgs "-Net good_wifi,bad_wifi"` (`-ScriptArgs` is handed to the script; lab_run.ps1 takes `-Net` and `-TotalMin`).
The standard run is `-ScriptArgs "-Net std"` (or `-NetStd`): `good_wifi,bad_wifi,mobile_tether,transatlantic`, clean + 4 profiles over 3 pairs. A PowerShell string parameter cannot be given without a value, so
the "default list" is the word `std`.

Every room reports its timings in milliseconds in `metrics`, in the clean pass too, measured by the runner on one clock (`time.monotonic`, both copies are on one PC; the check
loop polls the acting copy and the other copy alternately every 50 ms, so the resolution is about one bridge call, and 0 means "already there at the first poll"). They are
kept as the room goes, so a room that fails later still reports what it measured:

| number | metric | room |
|---|---|---|
| kill to death seen on the other copy | `host_kill_ms`, `client_kill_ms` | fight |
| pickup to gone on the other copy | `host_pickup_ms`, `client_pickup_ms` (and `host_take_ms`) | loot |
| door unlock to seen on the other copy | `unlock_ms` (and `*_open_ms`, `*_close_ms`) | door |
| quest stage to seen | `host_stage_ms` (host sets 10), `client_stage_ms` (client sets 20, via the intent), `host_stage30_ms` | quest |
| NPC handoff | `npc_handoff_client_ms` (host leaves the cell), `npc_handoff_host_ms` (host comes back) | npc |
| puppet error p95 / max (units) | `host_err_p95`, `client_err_p95`, `*_err_max` | walk |
| sync lag (puppet back within 80 units after the walker stops) | `host_lag_ms`, `client_lag_ms` | walk |
| puppet first seen near the player | `puppet_seen_ms` | spawn |
| slowest VATS shot to agreement | `vats_ms` | vats |

`summary.md` gets "Network: room x profile": one row per room, one column per pass (clean first), each cell the verdict and the key numbers (host copy/client copy; a number is the spread between the two copies showing a change, whichever showed it first: a guest's kill or stage is applied by the host first and comes back). A failed network
room has its own block, headed with its profile. `results.json` has `nets`, `net_rooms` (the rows, each with `profile`), `net_info` (the relay's counters per pair and profile) and
`clean_verdict`; `verdict` covers both passes. `--param fight.latency_s@bad_wifi=2` changes one profile's limit.

Limits per profile are in `lab_rooms.PARAMS` as `room.key@profile` (the clean `room.key` is used when a key has none). They are the clean limit plus an allowance derived from the
profile's own numbers in `tools/lab/lab_net.py` (its docstring is the justification: legs a change crosses, ENet's retransmit timeout, resends a lossy or bursty link may need, a safety
factor of 2, the walking speed for position error). `python -I tools/lab/lab_net.py` prints the allowances:

```
profile          one-way ms   loss   resends  +seconds  +units
lan              0.5+-0.2         0.0    0        0         0
good_wifi        12+-4            0.005  1        0.25      22
bad_wifi         35+-25           0.02   2        0.92      86
mobile_tether    70+-40           0.01   2        1.64      119
transatlantic    90+-8            0.003  1        0.82      49
hell             200+-150         0.08   3        7.4       280
```

The clean rule (no digest repair, no area heal) stays strict behind every profile, as `repair_budget` does for the chaos playbooks: a link that only delays and drops must not need a repair.
A network pass spends less time idle (`spawn.idle_s@profile` 10 s) to keep the whole run (clean + 2 profiles, 3 pairs) near 15 minutes.

Measured on the second PC, 3 pairs, clean + good_wifi + bad_wifi, 2026-10-27 (`out/lab/20261027-193458`, 5 min 48 s of room time, 6 min 15 s with boots; each profile pass restarts the pair, because a second
loadSave in the same game got the client's rejoin refused with "your mod list differs from the host's"). The relay carried about 3000 packets each way per profile and dropped 0.3 percent (good_wifi) and
2.5 to 4 percent (bad_wifi). Host copy/client copy, ms (e95 in units):

| room | clean | good_wifi | bad_wifi |
|---|---|---|---|
| spawn: puppet near | 156 | 171 | 610 |
| walk: puppet error p95, sync lag | 97/128u, 141/94 | 79/82u, 109/109 | 77/88u, 94/78 |
| fight: kill to death on the other copy | 31/32 | 78/125 | 47/94 |
| loot: pickup to gone on the other copy (host) | 0 | 31 | 31 |
| door: unlock to seen | 0 | 47 | 31 |
| npc: handoff to client / back to host | 281/234 | 328/141 | 500/250 |
| quest: stage to seen | 0/0 | 16/15 | 219/0 |

Numbers under about 50 ms are at the polling resolution (a watcher poll is one bridge call, about 16 to 30 ms), so "0" and "31" are the same fact. The limits are far above these (bad_wifi: kill 3.9 s, stage 10.9 s):
the clean limits were guesses, and the allowance only adds what the link may cost; tighten the clean ones (`--param`) once a few runs agree. What that run found that is not about the link, and what it was (all four fixed, see "Measured in game" below): the loot room's "only the host gained loose item 1" was the
bridge's `activate.ref` (it gives the lab's placed Stimpak to nobody), the door room's "host opens the door" was `Bot.open` taking a placed door for a container
(it activated the LOCKED door and left the lockpick menu open) and then a host-native door change that never reached the client (the script gate), VATS did not
enter because that lockpick menu was still open and then because the gecko had run up to the shooter, and the NPC "snap" of 328 units was a step between two
samples that were 2 s apart.

## What a run does

1. Per pair (instances 2k and 2k+1): provision, install the lab plugin, boot once, wait until both copies serve requests.
2. The spawn room first: load the save (`--save`, default `nvcoop_autosave_ready`) once on both copies, wait until the session is synced, `coc NVCoopLab` on both, then its checks.
3. The other rooms in order, each from its stand marker, each in a function with a hard time box. The first failed check ends the room and records what differed.
4. `out/lab/<stamp>/results.json` and `summary.md`, then the comparison with the newest earlier run in `out/lab`.

Rooms are split over the pairs with the most expensive first (spawn is on every pair, so a bad pair shows up as its own spawn row).

## The rooms

Every check compares the same placed ref (same id) on both copies, never counts.

| room | what it checks | likely source |
|---|---|---|
| spawn | both copies in the lab cell, connected (host sees the peer joined, client joined and transport up), each shows the other's puppet near him, 20 s idle with at least one digest ok and no mismatch or repair | plugin/coop, plugin/avatar, plugin/areadigest |
| walk | host then client walks four waypoints; the other copy's puppet: error p95 and max, teleports, catch-up time after he stops | plugin/avatar |
| fight | 3 hostiles; host shoots one dead, client shoots one dead; each death shows on the other copy within the latency limit, deaths and the survivor's health agree, no corpse gets up for 8 s | plugin/combat, plugin/actors |
| loot | host and client take from the container, pick up loose items, race for one: container and world agree, nobody gets a duplicate, totals match | plugin/loot, plugin/intents |
| door | a locked door stays shut for a try, the client unlocks it through the terminal, host and client each open and the other closes: lock and open state the same on both | plugin/hooks_world, plugin/intents |
| npc | the client stands next to the NPC and the host far away in the same cell: the host keeps it (an eligible host always owns NPCs, synccore pickOwner). The host `coc`s into GSDocMitchellHouse: ownership moves to the client with a new epoch and both copies agree. The host `coc`s back to the lab, stands at the npc stand marker: ownership returns to it, new epoch. No position snap, the copies agree while both have the NPC loaded (the host's copy is not sampled while it is away), it patrols | plugin/actors, synccore ownership |
| vats | real-time VATS by host and client on their own target: health after every shot agrees, targets die on both | plugin/vats, plugin/combat |
| quest | host sets stage 10 and a variable, client sees them; client sets stage 20, host agrees; host sets 30 and it does not roll back | plugin/hooks, plugin/snapshot |

Every room ends with the clean rule: no digest repair and no area heal during the room (as in `prove.py`).

## Frame rate of the copies (`fps` in results.json and summary.md)

Every room samples the plugin's frame counter of both copies (`coop.status` counters.frames, the counter `tools/bench/coop_bench.py` turns into fps) about every 0.5 s of its own clock, once
when it starts and once when it ends (`lab_fps.py`, `Ctx.fps_sample`). Between two samples a copy ran (frames b - frames a) / (t1 - t0) frames per second.

- `rooms[].fps` and `net_rooms[].fps`: `{"host": S, "client": S, "copies": N}` with `S = {"mean", "p5", "min", "n", "secs"}`. `mean` is all frames of the room over all its seconds (a stall counts as
  long as it lasted); `p5` is the fps the slowest 5 percent of the room's TIME ran at or below (weighted by interval length, not by sample count); `copies` is the number of game copies that were
  running when the room ended (6 for three pairs: the fps of one copy is a share of one PC).
- `fps.passes`: the same pooled over the rooms of the clean pass and of each profile, `fps.overall` over everything, `fps.provision`: per copy its process's Windows power throttling state, the
  bridge's window options (`minimized`, `focusGuard`) and the frame-rate keys of its ini files (`iFPSClamp`, `iPresentInterval`, `bAlwaysActive`, window size).
- `summary.md`: "FPS per copy" is one table, rooms x passes, each cell `host/client mean p5 host/client`, then the pooled row and the number of copies running. The spawn room includes the save
  load and the walk into the lab, so its mean is the lowest.
- The walk room adds `host_leg_fps_walker`, `host_leg_fps_viewer`, `host_leg_fps` (the lower of the two: the position is sampled, sent, received and applied once per game frame on both copies, so both
  bound the error), the same for `client_`, and the stream's own rates over the leg, `*_leg_published_hz` (snapshots the walker published per second), `*_leg_received_hz` (snapshots the viewer
  received) and `*_leg_fps_fed_hz` poses fed to the puppet. 30 Hz near rate (`[avatar] nearHz`, the lag round of 2026-10-27) shows as about 30 when the copies run at 30 fps or more, and 20 before it;
  neither can exceed the copy's fps. The summary's network table shows `fps a/b` in the walk cell next to the error.

### Windows power throttling of minimized copies

Test copies are minimized on purpose (`[bridge] minimized=1`, the owner's desktop). Since Windows 11 a process whose windows are all minimized is "not visible": its timer resolution requests are ignored
(a `Sleep(1)` lasts a whole 15.6 ms tick whatever `timeBeginPeriod` asked) and it can be put in EcoQoS (execution speed throttling). `winproc.unthrottle` (`SetProcessInformation(ProcessPowerThrottling)`,
control bits set, state bits clear) opts a copy out; `run.launch_instance` calls it at the start and again after the windows are minimized. `NVCOOP_UNTHROTTLE=0` (or `lab_run.ps1 -NoUnthrottle`)
starts copies the old way, `lab_run.ps1 -Visible` shows the windows: both are only for comparing fps.

### Measured: frame rate of the copies (second PC, 2026-10-28)

Three pairs (6 copies, minimized, 1280x720 windowed, `iPresentInterval=0`, `iFPSClamp=0`) run at a steady **60 fps**, with or without the throttling opt-out: copies started the old way
(`lab_run.ps1 -NoUnthrottle`, run `20261028-025122`, process throttle control mask 0) gave spawn 60/60, walk 60/60 (p5 59/59); with the opt-out (mask 5) the clean pass of
`20261028-030421` gave 60/60 over all rooms (p5 58/58), the profile passes 59 to 60 (p5 54 to 58). Two copies alone (one pair) give the same 60. Only the NPC room dips (mean 58, p5 42 to 44
on the host copy: its `coc` into another interior). So the first guess (the copies run at about 10 fps; the model reproduces the old lab numbers at 10 fps) is **wrong**: nothing throttles
the copies and the opt-out changes nothing measurable. It is kept because it is free, reported (`fps.provision` shows each copy's mask) and can be switched off (`NVCOOP_UNTHROTTLE=0`).

Walk error against fps (60 in every row; the model, `avatar_lag_tests --report --fps 60`, predicts p95 15 clean, 19 good Wi-Fi, 34 bad, 48 mobile, 51 transatlantic; at 10 fps: 62 / 68 / 98 / 103 / 103;
the old buffer at 60 fps: 46 / 49 / 52 / 64 / 79). `lag est` is the playback delay found by sliding the puppet's path over the walker's (`lab_rooms.lag_estimate`), free of the 55 to 65 ms the two
bridge reads take; `aligned` is the error p95 at that shift. Host leg / client leg:

| pass | fps | error p95 raw | lag est ms | error p95 aligned | published / received Hz (host leg) |
|---|---|---|---|---|---|
| clean | 60 / 60 | 71 / 114 | 150 / 130 | 57 / 89 | 27.8 / 23.9 |
| good_wifi | 60 / 60 | 83 / 111 | 180 / 200 | 70 / 68 | 27.8 / 23.9 |
| bad_wifi | 60 / 60 | 84 / 78 | 180 / 170 | 49 / 48 | 27.9 / 21.4 |
| mobile_tether | 59 / 60 | 90 / 92 | 220 / 260 | 63 / 56 | 28.0 / 21.6 |
| transatlantic | 60 / 60 | 84 / 98 | 220 / 300 | 57 / 55 | 27.7 / 26.4 |

What it says: the sender is the new one (28 snapshots a second published while walking, 20 before the lag round; 30 Hz is the cap and a walk has stops), the puppet is fed a pose every
frame (59 a second), and the copies are not slow, yet the puppet trails the walker by 130 to 150 ms on a clean link and 170 to 300 ms behind the profiles (one-way delay of the profile
plus about 100 ms). The model's lag at 60 fps is 27 ms clean and 37 to 116 ms behind the profiles. So the low frame rate is not why the fix does not show; about 100 ms of delay between
the sender and the puppet is not in the model. `avatar.status` does not report whether the low-latency buffer is on or its live delay (`[avatar] lowLatency` defaults to 1 and the ini of the copies
does not set it), so "active" is inferred from the 28 Hz sender; the receiver side needs a field there (plugin/avatar, not this tool). Suspects named in `docs/tools/avatar.md` "Not done": the
net tick running one frame after the avatar module's sample (module order), and the host's relay hop for the client's stream.

## Measured: first appearances and the NPC handoff (2026-10-28)

- **Puppet near** (`puppet_seen_ms`, spawn room): 150 to 200 ms in 13 of 15 spawns whatever the profile. That is the first poll (a poll is about a dozen bridge calls, each served at the
  next frame), so "already there". The two slow ones (734 ms good_wifi, 656 ms bad_wifi in `20261028-030421`; 2719 ms good_wifi, 1313 ms transatlantic in the run before) are the CLIENT copy: its
  puppet of the host is listed and placed within 190 to 230 ms ("Driving", spawn calls 3, nothing throttled) but 773 units away (the host's position before the room teleported both players to
  the spawn markers), and takes until about 650 to 730 ms to arrive. Not a join or handshake retry, not a missed first snapshot (snapshots arriving: 34 to 86), not a hold time in the lab: the
  puppet's catch-up after a teleport is slow and uneven (`puppet_trace` in the metrics has the phases). Finding for plugin/avatar. The spawn room now records `{host,client}_puppet_{listed,placed,near}_ms`,
  `puppet_wait_started_ms_after_enter` and, for a wait over 0.5 s, `puppet_trace`.
- **NPC handoff** (`npc_handoff_client_ms`, host leaves the cell): the old number was the time until BOTH copies' owner views agreed, and the host's copy answers no bridge call while it loads the
  cell it `coc`s into, so it contained that load (about 1 s) on every link. It is now the time to the new owner's own copy saying it owns the NPC under a newer epoch, with the pieces:
  `npc_{client,host}_coc_ms` (the coc call returned), `_left_ms` (the host's copy first showing the new cell), `_owns_ms` (the new owner's copy says it owns). Clean, good, bad Wi-Fi: the client owns
  the NPC 50 to 300 ms after the host's copy shows it left (owns 1000, 1218, 1188 ms after the coc started, left 953, 1093, 891). mobile_tether and transatlantic: 1110 and 1125 ms after
  the host's copy left (owns 2094 and 2109 ms): real, a bit over a second behind a link with 70 and 90 ms one way, so a handshake of several round trips or a timer in the ownership switch.
  Finding for plugin/actors and synccore ownership (not measured further here). Handing the NPC back (host returns): 170 to 410 ms after its own coc.

## Guest take and quest variable (2026-10-28)

- Quest variable: `bot.questvar {quest, var}` (plugin/bots, `nvhooks::questVarValue`, twin 75) reads the live value on the copy. The quest room now runs "the client sees the quest variable": the host sets
  7 through `test.quest.setvar` and the client reads 7 (stage and variable both synced, 0 to 60 ms).
- Guest take from the container: there is no way to click a line of the container menu (no pinned call for the menu's transfer, nothing in plugin/bots). The guest takes with `coop.loot grab` (test
  helper, `NVCOOP_TEST_HELPERS=1`, which the lab sets): RemoveItem on the container with the guest gate lifted, AddItem on the player, either with the container closed (the loot service's
  menu-less path, `docs/tools/loot.md` item 13) or with its menu open and held 2 s. Both were measured: the guest's inventory gains the item, the container reads 2 on the guest and **3 on the host
  for 10 s** (`20261028-025904`, `-030148`); the guest's `coop.loot` status after the take says containers 0, polls 0, actions 0, so the service never saw the chest. Finding for plugin/loot (is
  a never-opened placed chest invisible to the menu-less scan?). Because the driver is not a real player path the check is **skipped by default** with that reason; `lab_run.ps1 -GuestTake helper`
  (or `LAB_GUEST_TAKE=helper`) makes it, and `LAB_GUEST_TAKE=menu` is the open-menu variant. The host's take is unchanged (console lines).

## Fight under lag (2026-10-28)

`client kills enemy 2: no clear shot even from the standoff (659 / 875 units)` behind bad_wifi and transatlantic: the second hostile drifted out of the pistol's range and `step_back` took the stand
marker as "far enough" (it only asked the marker to be 0.8 x 350 units from the target) and hopped there 24 times. Now the host restrains each target as soon as it is enabled (`SetRestrained 1`,
as the vats room does, check "the host holds enemy N in place"), the stand marker is used only when it is a good spot for where the target is (200 to 450 units from it), otherwise the shooter
goes along the line to the target, 350, 275, then 200 units away. The kill still has to cross the link (the room times the death on the other copy). A weapon left holstered with the fighting
controls off by a scene or a talk (story run `20261028-023656`, S05 and S15, "weapon still holstered after the shots, the pulses never reached the game") is fixed in the same place: `shoot` first
draws the weapon, and when that fails runs `player.EnablePlayerControls` and draws again (`ready_weapon`). That cause is a guess from the symptom (quest scripts switch controls off around scenes);
it was not reproduced in the lab, where the fight room never starts holstered.

### The standard network run after these changes (`20261028-032920`, `-NetStd`, 9 min 38 s)

9 of 10 clean rooms and 38 of 40 network rooms PASS, fps 59 to 60 in every pass (p5 53 to 58, one dip to 31 in mobile_tether's fight room). Three failures, each seen once in about five runs of
that room and not reproduced: clean `npc` (the two copies' views of the NPC 178.8 units apart, limit 150; 24.8 in the other runs, a plugin/actors position sync finding), `vats` behind bad_wifi
(`areadigest.status.heals.total +1` on the client, the clean rule), `fight` behind transatlantic (client: 24 volleys, ammo used, a held target at 340 units stayed at health 5: no hit landed;
the same room passed behind all four profiles in the run before, plugin/combat finding or aim under 90 ms). Two runs before it were lost to the environment: a copy that exited before its pipe came up
(pair 1, good_wifi) and a run started while copies of the previous run were still up (join refused: "your mod list differs from the host's"): stop copies (`lab.py down`) and check
`tasklist` before a run.

## Network sync round (wt/netsync2, 2026-10-28)

Run `20261028-032920` left three intermittent failures and one latency finding. What the flight files of that run and the repeat runs showed:

- **NPC handoff 1.1 s on mobile_tether / transatlantic** (found, fixed). The client's flight records (`NetIn`/`NetOut` of the 0xAC channel, decoded by hand: Transfer, final `MSnapFull`, Ack, Commit) show the new owner sending NO Ack for the actors whose handoff took about a second, and the
  host's Commit arriving 996 to 1013 ms after the Transfer = `handoffTimeoutMs`, a forced commit. Cause: the host relays every actor's newest snapshot UNRELIABLY, including the final it has just sent reliably (same seq). Behind a link with jitter the copy
  arrives first, the reliable final then reads "the same seq again", `acceptSnapshot` drops it and the new owner neither places the state nor acknowledges. Fix (`ActorSync::onSnapFull`): a final that only repeats what the epoch already delivered still completes the handoff
  (`Stats::finalsRepeated`). A second finding in the same files: one actor was forced after every host move on every profile (clean too) because the host had no state for it, so no final was ever sent; the host now commits at once when it has nothing to hand over
  (`OwnershipService::commitNothingToHandOver`, `immediateCommits`). Same DLL, switches `[actors] norepeatfinal=1 nocommitnofinal=1` (old behaviour) against the default: client owns the NPC after good_wifi 1031 / bad_wifi 1047 / transatlantic 1125 ms with 7 forced commits per pass (old) against
  16 to 594 ms and no forced commit (new), in four runs of all five passes.
- **vats / bad_wifi `heals.total +1`** (reproduced once, with the audit switched off, on mobile_tether: `client healed actors key=1606745181 ... alive hp -55 -> alive hp 5`; the flight record of the original run has the same line, the client's copy at -55 and the owner at 5). The copy's health was
  wrong by the owner's weakening (5) minus 60, with nothing in the combat channel: the client had sent no hit for that actor. An actor value is mirrored once, when it changes, and only while the ref is loaded; the engine keeps an unloaded actor's damage elsewhere (`docs` of `av_read.h`),
  and the copy reads another health once it loads, with nothing new from the owner to correct it until the area digest's 1 s settle + diff exchange healed it. Fix: `ActorSync::auditAvs` checks a loaded copy's health against the owner's every 500 ms (under the settle time) and writes it back
  (`EnginePort::auditAv`, `PortStats::avDrifts`, `actors.port`). The exact moment the engine moves the damage is inferred, not observed. With the fix: 0 heals in 20 vats rooms (four runs of five passes); with the audit switched off: 1 heal in 5.
- **npc / clean position error 178.8** (not fixed, limit unchanged). Not an artifact of reading the two copies apart: in the repeat runs the worst pairs came 0 to 47 ms apart. Twice (132 and 161 units, clean and bad_wifi, stage `host returns`) the host's NPC was 130 to 160 units behind the client's, the client
  still owning it; the other stages stay at 14 to 40. The lab now records the stage with the ownership views (`npc_err_stage`), the reads' distance in time (`npc_err_reads_apart_ms`), the pair and the time-aligned error. Not found yet why the host's copy trails right after it re-enters the cell.
- **fight / transatlantic, 24 volleys and the target stays at 5** (not found). The client's flight record of that pass has no combat message sent at all (0 `MHit`): the shots never reached the forwarding step, so they were not lost on the link and not rejected by the owner. A free-roaming host target
  failed the same way in three older clean passes (`target health 5, 318 units`), and the story run `20261028-045950` S15 (host, unweakened Powder Ganger, target health 40) too: not latency specific. 25 fight rooms in the repeat runs all passed. The failure reason now carries both copies' combat
  counters, the target's position on both copies, the shooter's position, and the target's 3D state: `engineHits` 0 = the bullets hit nothing (not combat sync); `clamped`/`hitsSent` > 0 with `hitsRejected` on the other copy = the owner refused.
- `LAB_INI_EXTRA` / `lab_run.ps1 -IniExtra "k=v+k=v"` puts extra keys into the copies' `nvcoop.ini` for one run (an A/B run). A key set to 0 counts as "absent" to the ini check, so switches must be 1 = off-by-default.
- `coop.owner {"stats":true}` answers the actor module's counters (finals repeated, stale snapshots dropped, the host's forced and immediate commits); the npc room records their change, the vats room the health drifts written back.

## Reading the results

`summary.md` (under 5 KB): a header line with the verdict and build, one table (room, pair, verdict, class, seconds, checks passed, first failed check), then 3 to 6 lines per
failed room (the check, expected and actual per machine, the first digest key that differed with both values, counters that moved, the plugin folders to look in), then
"vs previous": rooms that were fixed or regressed, and numbers (walk error, kill latency) that moved by a quarter or more.

`results.json`: `rooms[]` with `verdict` PASS, FAIL, BLOCKED or TIMEOUT; `class` SYNC (the copies disagree), CRASH (a copy exited), ENV (the PC or load, requeue), TEST
(the lab or the layout is wrong), BUILD (a bridge method is missing); every check with `expected`, `actual`, `secs`; `first_divergence`, `counters_moved`, `digests_moved`, `metrics`.
Exit code: 0 all PASS, 1 FAIL or TIMEOUT or CRASH, 3 BLOCKED (layout, plugin, build), 4 ENV, 2 bad arguments.

## Measured in game (second PC, 2026-10-27)

- Failure texts are kept at 400 characters (`actual`) and 200 (`expected`) in results.json; a tight summary.md cuts only the tail of the got clause.
- `bot.look` does nothing on the CLIENT copy (reply ok, the angle reads back unchanged); the console `SetAngle` works on both. `LiveMachine` sets `bot.look_via_console`, so a walk steers with the console line.
- This build has no `RemoveItemTarget` (the console answers ok:true and prints "not found" in the output). `bot.take` then does `RemoveItem` on the source and `AddItem` on the destination.
- A guest's console `RemoveItem` on a container changes nothing (blocked on purpose), so the lab does not take from the container as the guest: `Machine.can_take()` says why and the check is skipped. It needs a plugin/bots method that clicks the container menu.
- A picked-up world item shows the deleted flag (0x20) in `ref.info`, not the disabled one: both count as gone.
- The fight room enables one hostile at a time (all three charge the players, and by the second shooter's turn one stood 60 units away, inside aim_at's 100 units); a shooter that is too close steps back to the stand marker.
- The lab terminal is the stock `DefaultUnlockTermWALL` (unlocked flag set). The DESK0VeryEasy one opened the hacking menu. It has ONE entry, "Disengage Lock", with the stock result script `ref myLink / set myLink to getLinkedRef / myLink.unlock`; the lab provides what that needs (the terminal links to the door with an XLKR), the generator is unchanged.
- The door check failed because `dev.terminal` walked the terminal menu's list from the wrong node (the ListBox embeds its tList head at +4; the code took the value there for a node address): no entry text was readable and the click got a vtable as its tile. Fixed in `plugin/bots` (`terminalEntries`); measured afterwards on the host copy: entries `['Disengage Lock']`, the click unlocks the door within a second.
- A first-use hint box (TutorialMenu 0x423) opens over a terminal, VATS and lockpicking and eats the input. `bot.menu.close {kind: "tutorial"|"any"}` now closes it with the engine's own close (0x7E8D60); `Bot.open`, `Bot.use_terminal` and the input pre-flight close it first.
- Door room: a guest's terminal click does not run the result script on its own copy; the TerminalEntryRun hook forwards it as an intent and the host runs it (client clicked: host showed the door unlocked within 1 s, the client's copy stayed locked 15 s). The control check therefore accepts the unlock on either copy; "unlocked on both copies" is the SYNC check and fails today (host unlocked, client still locked): a lock-state sync gap, plugin/hooks_world or plugin/intents.
- VATS room: the two geckos are enabled one at a time and weakened to 30 health first (the host is the owner). Enabled together they charged the host and were on top of it when its shot started (`vats_not_entered`); `LiveMachine.vats` also steps back when the target is closer than 150 units and retries when the target is not in view. Last run: shots 1 to 3 enter, play and exit with the health agreeing; shot 4 shows host -14 and client 1 (the run stopped at that check, so it is not known whether the client's gecko was dead: a dead copy shows the port's floor 1 for the owner's -14; plugin/combat now kills the victim of a natural lethal hit whose life state stayed alive, see docs/tools/combat-hooks.md; the check should compare health only while the target lives and then ask "dead on both copies").
- NPC room (ownership, rewritten): the eligible host always owns NPCs, so the room sends the host to GSDocMitchellHouse and back; it passes in game (29.9 s).
- The fight room's hostiles are weakened first: the host (owner) sets `SetAV Health 5` and the room waits until both copies show it, then the shooter fires (a 65 health gecko took 24 volleys and was not dead in the first run). Before every volley `line_of_fire` checks the distance (150..500 units) and `GetLOS`; a bad spot steps to the 350 units standoff.
- VATS opens only with the weapon drawn (the lab draws it) and the target in view; no limb can be chosen (`unsupported: limb selection in VATS is not implemented`). With the weapon drawn, `bot.vats` made the game's main thread stop answering the bridge (a hang, reproduced on two copies). Cause (a minidump of the hung copy, 2026-10-27): with real-time VATS on (`[vats] realtime=1`, the lab's setting) the main loop still runs the menu update for the open VATS menu, and that update waits with an INFINITE timeout for a worker-thread event (`0x713C70` -> `0x8C7A70`) that belongs to a paused world; nobody signals it in the VATS zoom (camera mode 3), CPU 0, window still answering. `plugin/vats` first redirected the call at `0x713C8D` so the wait was skipped; that let the worker's interface update overlap the main thread's highlight draw (the 0x400F66 crash below), and the redirect was replaced by moving the update to the main thread. The bot job also had three smaller faults, all measured: an accept in the frame after the queue started no playback (it now waits 400 ms and repeats, at most 3 times), a hint box that opens over the accept swallows it (the job closes the box and accepts again), and the accept gate had no limit (now 16 answers and 3 s). Result on the host copy: enter, queue, accept, playback, done in 1.6 s with the target hit (65 to 55.8). `Bot.vats_target` also ends its wait with `main_thread_stuck` after three bridge answers "main thread did not service the request" in a row.
- The spawn room's idle digest window: it waits at most `spawn.digest_wait_s` (12 s) for the first digest verdict and ends the moment one is bad.

- The fixes of 2026-10-27 (second PC, one pair, by hand and by the rooms):
  - **loot, "only the host gained loose item 1"**: not the loot sync (its counters showed one won claim, no revert, no remove on the picker). The bridge method `activate.ref`
    calls the engine's `Activate(player, 0, 0, 0)`; the console command calls `Activate(player, 0, 0, 1)` (0x5B5B18 pushes 1 last). With the last argument 0 the engine deletes the placed
    Stimpak and gives the player nothing (host and client; the Nuka, Beer and RadAway of the same room are given). `Bot.pickup(console=True)` and `LiveMachine.pickup` for the host use the
    console line (a guest's console line is dropped by the script block, so a guest still uses `activate.ref`). **Request for plugin/hooks:** `MActivateRef` (hooks_module.cpp) should pass 1
    as the last argument, as `Cmd_Activate` does.
  - **door, "host opens the door"**: `ref.info` answers the type of the REFERENCE (0x3A for every placed object), so `Bot._kind` never saw a door: `open()` took the container/terminal path,
    activated the locked door ("the host cannot open the locked door" - the lockpick menu stayed open and ate the next open and the VATS of the next room) and called an opened door failed.
    `Bot._kind` now asks `GetType` (door 28, container 27, terminal 23); the rooms close a menu a step left open (`Machine.close_menus`, called by `stand_all` and after the locked try).
  - **door, "the door the host opened is open on both copies"**: a host-native open (the world-time poll sends an `ObjectState`) never changed the client's door. `WorldApplier::applyAny`,
    `tick` and `applyBaseline` held a `ScriptGate` around the whole plan; the gate makes `Script::Execute` pretend the script ran and a console line (`SetOpenState 1`, `UnLock`) is a script,
    so the plan reported Ok and did nothing. The world copy of a script's change (`SetObjectState`, applied without a gate) hid it. The gate is now held per step, only on the steps that
    are not console lines (`stepRunsScripts`); the client's heal in `wt_glue.cpp` had the same gate and healed nothing, 8 times a minute.
  - **vats**: the first cause was the door room's lockpick menu on the same pair. Then an enabled gecko ran 368 units in about 4 s and stood on the shooter (no view, aim_at under 100
    units, hit chance 0): the room's host now holds each target with `SetRestrained 1` (`Machine.hold`). A VATS job whose camera closed before the job saw the queue, with the target hit or dead,
    is a shot (the mod's real-time VATS fires the queued attack itself). A failed job leaves the game IN VATS (camera mode 2); disabling the target then crashed the host (FalloutNV.exe+4016C7).
    **Fixed: a VATS entry crashed the game** (`FalloutNV.exe+400F66`, about 4 in 15 entries, host and client, right after `phase tutorial -> zoom (camera mode 1)`).
    0x800F30 draws the VATS target highlight (InterfaceManager+0x1DC) and reads its target (+4) unchecked; the crash log had ecx = eax = 0 on the main thread.
    Cause: real-time VATS lets the world run under the VATS menu, so the main loop kicks the AI linear task threads with a menu open, which the game never does,
    and thread 1 runs the InterfaceManager update (0x86FD70), whose highlight update clears the target (0x800495) while the main thread is drawing it (gate on +8
    in 0x8749B0, unchecked read at 0x8741AF several ms later). The earlier hang was the same overlap the other way round (the menu update waited for that worker,
    which waited for the renderer lock the drawing main thread held). plugin/vats now runs the update on the main thread before the kick while a menu is open and
    the worker skips it (`worker_kick`, `worker_ui_update`); the 0x713C8D redirect is gone. Not the bot job: no tutorial close was made in the crashing entries.
    In game after the fix (2026-10-27, 6 runs of `-Rooms vats -Pairs 3`, 5 of them with `-Net std`): 62 VATS entries, 0 crashes (no crash-filter line, no
    WER event, no flight crash file), 0 hangs; each entry logged the handoff (main-thread frames = worker skips, about 35 per entry). Two runs lost game
    copies without any crash record, also in rooms without VATS, while a copy of an earlier run was still up; after stopping it the runs were clean apart
    from the area-heal check (`client:areadigest.status.heals.total +1`, plugin/areadigest).
  - **npc**: the steps are judged by what the patrol covers in the time between the two samples (`npc.speed`, 200 units/s, at most 1.5 s counted), the watch samples all through the host's `coc`, and a
    failing step names its distance, seconds, copy and stage; `npc_max_step` and `npc_max_speed` are in the metrics.

## Uncertain until it runs in a game

- Everything in `tools/lab/lab_live.py`: the bridge side was written from `tools/bots` and has only run against the bots' own fake bridge.
- The thresholds in `tools/lab/lab_rooms.py` `PARAMS` (puppet error 150 p95 and 300 max, kill latency 3 s, NPC error 150, handoff 25 s) are guesses; change one with `--param`.
- Layout keys: the runner reads the names the generator writes (`tools/lab/tests/layout_generated.json` is a copy, a test loads it): refs `wp*`, `enemy*`, `chest`, `loose*`,
  `door`, `terminal`, `npc`, `target*`, `quest`; `initially_disabled`; `info.container_items`, `info.loose_items` (same order as the loose refs), `info.var`. The fight and vats hostiles
  start disabled: the host enables them (`enable` through exec with the ref) when the room starts, both copies must show them, and the host disables them again when the room ends.
  Verified on the second PC by the generator's author: the plugin loads through `install_lab.py`, and `coc NVCoopLab` after `loadSave nvcoop_autosave_ready` lands on the lab floor.
  Not verified: the unlock terminal's entry index (assumed 0, `info.terminal_entry` overrides), and the loose items being in `loose_items` order.
- The npc room parks the away player at the origin of the room farthest from the NPC (the stand marker may be too close for an ownership handoff).
- Quest variables: read through `bot.questvar`; a copy where it cannot be read (no script, never ran) skips the check with the bridge reason. Setting one needs `test.quest.setvar` (test helpers on).
- Shooting and VATS need the 9mm pistol and ammo (`FalloutNV.esm:0E3778`, `:08ED03`) and health 5000, set by console at the start of the room.
- A copy that boots without the plugin (started before it was installed) is BLOCKED with "run `lab.py down` and again". `provision` rewrites `Fallout.ini`, so the runner installs the plugin right before each boot.
- The bridge `screenshot` method is not used (it is not available on the second PC).
- `--confirm` and the hung-room watchdog (a room still running 20 s after its box) are only tested on the fake world.
