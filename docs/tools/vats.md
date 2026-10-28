# vats: real-time VATS for every player (milestone 4, FULL-SYNC-PLAN 3.4, owner requirement)

Code: `plugin/vats/` (game-free core, tests, x86 glue `vats_module.cpp`), playbooks `tools/bots/playbooks/pb_vats.py` + `tools/bots/run_vats.py`.
RE: `docs/re/vats-realtime.md`. Wire: `docs/protocol.md` section 20 (first byte `0xB7`). Status 2026-10-21: **unit-tested game-free only. NOTHING here has run in a
game.** The glue compiles and links into nvcoop.dll; it is inert unless `nvcoop.ini [vats] enabled=1`.

## The idea (the Fallout 76 model)
The world is shared, so time cannot stop or slow for one player. Real-time VATS keeps everything the player uses VATS for and removes only what freezes or slows
the world:

| Kept (the engine's own, per player) | Removed in a co-op session |
|---|---|
| target highlighting, the VATS UI, limb choice | the menu pause of the main loop |
| the hit chance from THAT player's skills, perks and range | the global time scale requests of the VATS camera (freeze, kill-shot slow motion, per-action factors) |
| AP use and the AP pool, criticals, damage (all computed by the shooter's engine) | the player 3x / target multipliers that compensate for a slow world |
| the VATS camera and the kill cam (they run for the local player only and no longer pause anything) | |

A shot fires in real time. Its hit goes through the engine's hit funnel like any other hit, so `plugin/combat` forwards it as an ordinary `MHit` to the victim's
owner (the `vats` flag is informational). Damage, limb crippling and death are therefore identical on every machine and other players see the attack play out
live. Without a co-op session (`sessionActive` false) every hook returns the engine's own answer: single-player VATS is untouched.

## Run it
- `plugin\vats\run_tests.cmd` (shared tree `build\all` through `tools/build/build.py`; prints `ALL OK` = the suite is green, all 60 runtime twins are caught, the pin
  verifier and the proof checker pass their selftests). Registered in `tools/test_all.py`.
- `python -I tools/bots/playbooks/tests/run_vats_tests.py` (the four playbooks on the fake VATS world, 19 twins). Registered in `tools/test_all.py`.
- `python -I plugin/vats/tools/verify_pins.py` re-checks every pin against `<reference exe>` (28 rules, all hold; SKIPs when the image is absent).

## What is where
```
vats_types.h        modes, limb ids (the limb ActorValue codes 25..31), Settings (+ clamping)
vats_policy.*       the three hook decisions, stateless: menuModeForWorld / scaleForVatsRequest / timeMultFor
vats_slowmo.*       party slow-mo: SlowMoArbiter (host) and SlowMoFollower (everyone)
vats_perks.*        VatsLedger (VATS kills), Grim Reaper's Sprint refund, Stranger credit, the perk table
vats_api.*          the public VATS state and actions (bots, tests, other modules): readView, selectLimb, selectTarget, fire, cancelAll
vats_wire.*         the 0xB7 channel
vats_sync.*         VatsSync: the per-machine session logic (phases, queue watching, auto fire, guard, slow-mo, config, perks)
vats_runtime.*      VatsRuntime: owns the VatsSync of the running session, routes the mux channel
vats_patch.*        SitePatcher: call-site redirects with every byte checked
vats_addrs.h        the engine pins (verified by tools/verify_pins.py)
vats_module.cpp     the x86 glue: LiveEngine (IVatsEngine), the thunks, the frame driver, the bridge methods, the ini
proof/vats_proof.py the in-game proof checker (snapshot / check / --selftest)
```

## The hooks (docs/re/vats-realtime.md section 1)
| Hook | Sites | Thunk | Decision |
|---|---|---|---|
| main-loop `MenuMode()` (fills the world-pause byte 0x11DEA2B) | 2 (`pausesites=1`), +9 more with `pausesites=2` | `MenuModeThunk` | `menuModeForWorld`: false while the VATS menu is the top menu and VATS is active |
| `TimeGlobal::SetTimeScale` from VATS code | 11 | `SetScaleThunk` | `scaleForVatsRequest`: 1.0, or the party scale in a slow-mo window |
| `GetPlayerTimeMult` / `GetTargetTimeMult` | 5 + 9 | `PlayerMultThunk` / `TargetMultThunk` | `timeMultFor`: 1.0 |
| worker kick 0x8C78C0 (main loop 0x86EC87) and the workers' InterfaceManager update 0x86FD70 (0x8C7DB9, 0x8C7BE9) | 1 + 2 (with `pausesites>=1`) | `WorkerKickThunk` / `WorkerInterfaceThunk` | `interfaceUpdateOnMain` + `InterfaceHandoff`: while the real `MenuMode()` is true at the kick, the main thread runs the update just before it and the worker skips it |
The last row exists because the game never kicks its worker threads with a menu open; real-time VATS does, and the worker's interface update then cleared the
VATS highlight target while the main thread drew it (crash FalloutNV.exe+400F66) or, waited for, deadlocked on the renderer lock (vats_addrs.h has the trace).
Every thunk calls the real function first. A pin mismatch patches nothing (log line). If the multiplier sites cannot be patched, the two game settings
(`fVATSPlayerTimeUpdateMult`, `fVATSTargetTimeUpdateMult`) are set to 1.0 instead and restored at shutdown.

## What each player gets
- **Hit chance, AP, criticals, kill cam:** native, per player (each machine runs the engine for its own player). The kill cam keeps playing for the local player
  and never pauses the world. `[vats] killcam=0` shortens the camera cuts (`fVATSCameraMinTime` / `MaxTime`) for the local machine; the host's value wins everywhere.
- **Auto fire:** a queued action starts its playback `autofiredelay` ms after the last queued action (default 350 ms, so a second shot can still be queued). It calls
  the engine's Accept. Never during playback, never twice for one queue, never for a party member.
- **Perks** (vats_perks.h):

| Perk | Treatment | Why |
|---|---|---|
| Grim Reaper's Sprint | **glue** | the engine refunds AP when ITS hit code kills a target in VATS; for a victim owned by another machine that code never runs. VatsLedger recognises the kill (an action was queued at that victim, the death arrived during or up to 4 s after the playback, the local engine did not decide it) and refunds once. The amount is `grs_ap` (default 20, a PLACEHOLDER: compare with the perk text in game) or `grs_all=1` for a full refill, capped at the maximum |
| Mysterious Stranger | **glue** | the Stranger is a temporary local actor; a hit from an actor no machine tracks would be refused by the victim's owner. `vats::liveAttackerFor(ref)` credits an untracked, non-party attacker to the player during the player's own playback |
| Four Leaf Clover, Better Criticals, Ninja, Concentrated Fire, Action Boy / Girl, Sniper, Commando, Center of Mass and the other hit-chance perks | native | computed by the shooter's engine; the critical flag and the damage travel in the hit |
- **Companions:** a companion belongs to its recruiter's machine (actor ownership), so it keeps fighting in real time there while the recruiter is in VATS. The
  guard (`guard=1`) skips another player's puppet as a VATS target (`isPartyRef`: the avatar layer's puppet refs and the NV:MP ghosts) and takes back an action
  queued at one. **Not covered:** another player's companion on a machine that does not own it is not recognised as a party member (the actor port does not export
  the recruiter); friendly fire off in `plugin/combat` still neutralises the damage.
- **Party slow-mo** (host setting `partyslowmo=1`, off by default): when any player ENTERS VATS all players slow together to `slowscale` (0.35) for at most
  `slowmaxms` (3 s), at most one window at a time, then a cooldown of `slowcooldownms` (10 s). The host decides and broadcasts (`MSlowMo`); every machine applies the
  scale through the engine's own SetTimeScale and expires it by itself if the host goes silent. The host's `vats.set` changes settings at run time.

## nvcoop.ini `[vats]` (read at DeferredInit; all OFF unless `enabled=1`)
`enabled=0` `realtime=1` `partyslowmo=0` `slowscale=0.35` `slowmaxms=3000` `slowcooldownms=10000` `autofire=1` `autofiredelay=350` `killcam=1` `guard=1` `perks=1`
`pausesites=1` (0 none, 1 main loop, 2 plus nine candidate sites) `grs_ap=20` `grs_all=0` `grs_edid=GrimReapersSprint` (the perk's editor id; UNVERIFIED name, a failed
lookup is logged once and the refund stays off). The host's values win on every machine once a client has joined (`MConfig`).

## Bridge methods (registered when enabled; main thread)
| Method | Params | Reply |
|---|---|---|
| `vats.status` | | module, pins, hook groups (sites / total / pinsOk), thunk counters, engine call counters, `sync` (settings, party slow-mo, ledger, stats). Always registered |
| `vats.state` | | `{mode, modeName, menuOpen, target, targetCount, queued, ap, maxAp, limb, limbAv, rememberedLimb, limbs:[{limb, av, index, highlighted}]}` |
| `vats.select_limb` | `{limb: "head" \| "left_arm" \| ... \| 25..31}` | `{ok, limb, limbAv, target, reason}` |
| `vats.select_target` | `{ref: 0x... \| number}` | `{ok, limb, target, reason}` (cycles the engine's next-target action, verifying each step) |
| `vats.fire` | | `{ok, ...}` starts the playback of what is queued (auto fire does this by itself) |
| `vats.cancel` | | pops every queued action |
| `vats.set` | `{killcam, partyslowmo, autofire, realtime, guard: 0\|1, slowscale}` | host only; the new settings go to every client at once |

## How bots drive VATS (limb choice included)
`plugin/bots` (Track B) has no field for the highlighted limb, so `bot.vats {limb}` stays "unsupported". This module exposes it. A bot does:
1. `bot.input {tap:["Vats"]}` and poll `vats.state` until `mode` is 1, 2 or 3 (the existing input driver; nothing new there).
2. `vats.select_target {ref}` then `vats.select_limb {limb}`; check `vats.state.limb` (the call already reads it back and says what the engine holds).
3. `bot.input {tap:["Activate"]}` queues the action. Auto fire plays it after `autofiredelay`; or call `vats.fire`.
4. Poll `vats.state.mode` until 0.
`tools/bots/playbooks/pb_vats.py` has this as Python (`enter`, `select`, `attack`, `limbs_of`, `limb_conditions`); inside the DLL use `vats_api.h`
(`readView`, `selectLimb`, `selectTarget`, `fire`, `cancelAll`, and the `live*` forms). A limb the target does not have, a wrong mode, an engine that refuses or
reverts the write, a target that does not move: all come back as `ok:false` with a reason, never as a silent success.

## Playbooks (one two-player playbook per weapon class; NOT run)
`vats_gun`, `vats_melee`, `vats_explosive`, `vats_energy` (`python -I tools/bots/run_vats.py --list`). Each one: (1) both players equip the class weapon and stand at the
target, same health and limb conditions everywhere; (2) every machine reads the VATS state and highlights EVERY limb (bot-side limb choice); (3) the host fires a head shot
while the other machines' frame counters keep advancing (world not paused) and the damage lands once; (4) the client fires at the right leg through the same victim owner;
(5) repeated shots cripple the left arm and every machine agrees on the limb conditions; (6) kill-cam ON and (7) OFF: a killing shot ends the target everywhere, once, and
nobody's world pauses; (8) nothing spoofed / malformed on the channel. `--slowmo` adds the party slow-mo step. Class specifics: melee checks the engine's lunge brings
the attacker to the target, explosive checks the blast hurts a neighbour (`--neighbor Plugin.esm:HEX`) and at most 2 hits land per action, gun and energy assume a
single-projectile weapon (`hitsApplied` is read from `combat.status`). A live run needs `--weapon <class>=<editor id or Plugin.esm:HEX>`; there is no default on purpose
(the editor ids were not verified). The fake `VatsWorld` has 18 deliberate bugs (pausing, kill-cam pause, dropped limb, double shot, unsynced crippled state, ...), each
caught by at least one playbook.

`vats_world_moves` (`python -I tools/bots/run_vats.py --instances 0,1 --playbook vats_world_moves`; not in the default set, which stays the four weapon classes) replaces the
old "by hand, once" checks with measurements, using bridge calls and `bot.input` / `bot.move` only. It needs no `--weapon`: it equips `--weapon any=<item>` when given, else uses the
weapon the player already holds (clear failure when there is none). Steps: (1) everyone armed and at the target; (2) the host in VATS holds `bot.move` backwards for 1 s and moves
at most 12 units, then after leaving VATS the same move covers at least 40 units; (3) for each player as shooter: a baseline window of 3 s with nobody in VATS, then the same window
with the shooter in the VATS menu, while another player runs away from the target (`bot.look` + `bot.move`, the hostile Powder Ganger chases it). On every other copy it samples
the game clock (`worldtime.clock`, falling back to the `GameHour` and `TimeScale` globals through the expression evaluator) and the NPC's position (`ref.info`, path length of 6
samples). Pass: the clock moved at least half of the baseline and of what `TimeScale` x elapsed predicts, and the NPC walked at least 40 % of the baseline distance; a baseline that
does not move (no chase, `TimeScale` 0) is a FAIL ("nothing to compare"), never a skip; (4) the limb: every limb reads back as highlighted after `vats.select_limb`, then a shot at
a leg (arm if crippled) must drop that limb's condition by at least as much as any other limb (up to 3 tries, VATS can miss). Twins: `vats_world_paused`, `vats_clock_frozen`,
`vats_npcs_frozen` (frames keep running, only the clock or the NPCs stop), `vats_player_walks`, `vats_controls_stay_locked`, `vats_limb_hits_neighbour`, plus the older
`vats_pauses_others`, `vats_limb_ignored`, `vats_reports_wrong_limb`. The NPC chase is the one live assumption (a hostile NPC pursues a fleeing player); it is guarded by the baseline.

## In-game proof (for the helper; code-only agents did not run any of it)
1. Two copies, host + client in one session, `nvcoop.ini` `[live] actors=1 avatars=1 combat=1`, `[dev] bots=1`, `[vats] enabled=1`. `nvbridge vats.status` on both: expect
   `pins` all true and `hooks` sites `11 / 5 / 9 / 2`. A pin that is false or a group with 0 sites means the exe differs: stop and tell the maintainer.
2. `python -I plugin/vats/proof/vats_proof.py snapshot --out out/vats/before.json`.
3. `python -I tools/bots/run_vats.py --instances 0,1 --weapon gun=... --weapon melee=... --weapon explosive=... --weapon energy=...` (add `--slowmo` once).
4. `... snapshot --out out/vats/after.json`, then `... check out/vats/before.json out/vats/after.json --expect-autofire --expect-hooks-fired [--expect-slowmo]`: every line must PASS.
5. `python -I tools/bots/run_vats.py --instances 0,1 --playbook vats_world_moves` (no `--weapon` needed): the formerly by-hand checks, measured, see "vats_world_moves" above.
   The kill cam still freezing nothing is covered by the class playbooks of step 3.
6. If something freezes: `[vats] pausesites=2` (more `MenuMode()` call sites), or `realtime=0` to compare with vanilla. If a crash: `[vats] enabled=0`, report the address.
Things the helper decides: the real `grs_ap` number, the real `grs_edid`, the weapon items for the four classes.

## Mutation twins (runtime, `FNVCOOP_MUT=n`; 60 in the C++ suite, 28 in the playbooks)
types 801 802; policy 811..817; slow-mo 821..829 831; wire 841 842; limb API 851..857; perks 861..865 871..875; sync 881..887 891..899 905 906; patcher 901..904.
Each breaks one rule (a cap not enforced, a stale grant accepted, the limb read-back skipped, a spoofed author believed ...) and the suite must fail.

## Integration requests (not done here: other agents own these files)
- **combat-hooks:** in `HitGateThunk` (plugin/combat/combat_module.cpp) credit the attacker through `nvcoop::vats::liveAttackerFor(rawSourceRef)` (declared in
  `vats/vats_api.h` under `NVCOOP_WITH_VATS`) before `entityOfRef`. Without it the Mysterious Stranger's hits on actors owned by other machines are refused (unknown attacker).
- **dll-wiring:** nothing is needed in `main.cpp` or `coop_module.cpp`: the module registers itself (`NVCOOP_MODULE`) and finds the session through
  `Runtime::active()`. If the live layer should switch it from `[live]`, add a `vats=` line there.
- **plugin/bots (Track B):** `bot.vats {limb}` can call `vats.select_limb` instead of refusing.

## Open items (honest list)
- **Never run in a game.** Everything engine-facing is static RE (docs/re/vats-realtime.md section 4): whether the two main-loop `MenuMode()` sites are enough for the whole
  world to keep running, what the VATS player's own character does meanwhile, whether the written highlight (VATSMenu+0x100, 0x11A59F0) is followed by the tiles and the
  queued action, the exact Accept semantics.
- The Grim Reaper's Sprint amount and the perk's editor id are placeholders. Nothing here reads the perk's own numbers.
- A Mysterious Stranger kill is credited only once combat calls `liveAttackerFor` (above).
- Another player's companion is not excluded as a VATS target on a machine that does not own it.
- Turbo (`fTurboTimeMultiplier`) slows the world through the same SetTimeScale and is NOT touched: it would slow every machine's world locally. Left as is (not VATS).
- `killcam=0` is a guess at what "off" means (shortened camera settings); the engine has no kill-cam switch.
- Party slow-mo uses the game's time scale, so game time, scripts timers and the weather advance slower during a window on every machine, consistently.
