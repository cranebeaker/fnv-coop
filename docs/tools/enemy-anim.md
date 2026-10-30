# enemy-anim: NPC animation sync (Track A, round of 2026-10-21)

Status: everything below is unit-tested game-free (fake engine, fake network, lossy live-layer pipeline). **Nothing has run in a game.** The game
side compiles into `nvcoop.dll`, but the engine assumptions listed under "What is proven and what is not" get checked when the helper runs
the in-game proof (`tools/animcheck/npc_proof.py`).

Code: `plugin/actors/anim/` (the layer), `synccore/live/live.h` + `live.cpp` (animState schema v2), `plugin/actors/actor_wire.*` and
`actor_sync.*` (the `MAnimEvent` pipe), `tools/animcheck/npc.py` + `npc_proof.py` (T7 assertions and the in-game proof).

## Run the tests

| Command | What it proves |
|---|---|
| `plugin\actors\anim\run_tests.cmd` | group table check, static slot check (+ twins), then the C++ suite (93 tests) and 43 runtime mutation twins. Prints `enemy-anim: ALL OK`. |
| `python -I tools/animcheck/run_npc_tests.py` | NPC fight assertions (29 tests) and 14 twins. Prints `animcheck npc: ALL OK`. |
| `synccore\live\run_tests.cmd` (also in `synccore\run_tests.bat`) | schema v2 codec tests and 4 new twins (`drop_anim_ext`, `drop_anim_ext_one_shot`, `anim_ext_no_delta`, `accept_bad_anim_ext`). |
| `plugin\actors\run_tests.cmd` | `MAnimEvent` through the real `ActorSync` (compile-time twins 30, 31, 32). |
| `python -I plugin/actors/anim/verify_slots.py` | the BaseProcess slots the game backend may call, read from a reference exe (skips without it). |
| `python -I tools/animcheck/npc_proof.py --selftest` | the in-game proof script against a fake game. |

All of them are in `tools/test_all.py`.

## What it does

```
owner machine                                              other machines
 engine --Read--> NpcAnimSampler --snapshot.anim--> live codec (20 Hz, lossy) --> InterpBuffer (100 ms behind)
                                                                                       |
 actor port drive(): SetPos / SetAngle  <------------ InterpOut -----------------------+
                                                       |
                                  NpcAnimDriver.drive(InterpOut) --PlayGroup / weapon / sneak / rate...--> engine
 dead actor: CorpseWatcher --MAnimEvent (reliable)--> ActorSync --> NpcAnimDriver.onCorpse --> PlaceCorpse once it lies still
```

* **Owner side** (`npc_anim_sampler`): turns engine reads into `live::AnimState`: locomotion code (speed and direction, the same codes as the
  player puppets, `avatar/loco.h`), weapon drawn, aim, sneak, jump (also from vertical speed above 350 u/s), reload, and the schema v2 extension.
  Booleans are debounced (the engine's weapon and aim flags flicker for a frame). A one-shot (attack, reload, stagger, recoil, block hit, dodge)
  is published as `action` = its base group id and `actionSeq` bumps when it STARTS, once; a restart of the same group (play time going back)
  is a new event. The action stays in the stream for 260 ms after it ends, so one lost datagram or a slow frame cannot skip a short stagger.
  A dead actor publishes `dead` and nothing else.
* **Non-owner side** (`npc_anim_driver`): given the same `InterpOut` the position was taken from.
  * Movement group follows the DISPLAYED speed (measured from the positions it is given, smoothed over 120 ms), not only the owner's code:
    an idle code with real speed, or walk code at run speed, that lasts more than 150 ms is corrected from the speed. A dry stream (buffer
    Holding / Empty) means idle, never running in place. Strafe and backward groups come from the direction of the displayed motion.
  * Playback rate follows the displayed speed (`SetGroupRate`) when the backend can do it (it cannot by default; see below).
  * Turn in place (TurnLeft / TurnRight) when the facing turns faster than 1.2 rad/s for at least 20 degrees; jump start / loop / land.
  * Weapon draw and holster: flag set through the backend, then the Equip / Unequip transition group; read back after 700 ms and retried
    (three tries, then a pause of 3 s, so a refusing engine is not hammered). The first sight of a copy sets the weapon without a draw animation.
  * The owner's one-shots are played once per event (counter compare is wrap-safe). A first look adopts the counter without playing, so an old
    event is never replayed after a load or when a stand-in appears. Reactions are not overwritten by the movement group while they play.
  * Knockdown: `KnockDown` if the backend has it, else one Stagger. Death: the driver stops touching the actor, verifies that the engine really
    has it dead (`KillActor` again up to four times), and waits for the owner's corpse event.
  * T-pose guard: nothing is sent before the skeleton exists; a fresh skeleton, a rebuilt one (3D generation changed), a closed menu, a cell
    change and a new world all re-issue the group, the stance and the weapon. The group is read back every 500 ms and re-asserted when the
    engine dropped it (never for transition groups, never for a standing idle when the group cannot be read).
  * Budget: at most 12 animation commands per tick (36 for one-shots, death, weapon, sneak). Refused commands back off 400 ms, 800 ms ... 8 s.
* **Corpses**: only the owner decides where a body rests. `CorpseWatcher` samples the dead actor until it has moved less than 3 units for
  500 ms (never before 800 ms, at the latest after 8 s) and sends ONE reliable `MAnimEvent` (23 bytes: cell, position, yaw, flags). The other
  machines let their own ragdoll play (visual only); once it lies still (and not before 1.5 s) they move the body to the owner's place if it is
  more than 40 units away horizontally or 30 vertically (up to four corrections). A pose that arrives before the death is kept; a pose that
  never arrives is counted (`corpse_missing`, after 15 s) and a late one is still applied (`corpse_late`).

## Schema v2 (synccore/live)

`AnimState` gained, after the old fields: `action` (u8 base group id, 0 = none), `actionSeq` (u8, wraps), `dead`, `knocked`, `swim`, `talk`,
`block`, `ragdoll`. On the wire the flags byte of the anim section carries bit 7 = "extension follows": three bytes `action, actionSeq,
extBits`. An all-zero extension is never sent, so a v2 sender with nothing to say emits the v1 bytes. A v1 receiver rejects the extension
(Malformed), so both ends must run the same schema: `kLiveSchemaVersion = 2` and `kActorWireVersion = 2` (the reliable full state carries
the same fields). The decoder accepts only canonical forms (reserved bits and an all-zero extension are Malformed). Rules for the next change:
append new fields behind a new flag bit that old receivers reject, bump both versions, extend the twins.

`MAnimEvent` (Body index 7 of the actor envelope, append-only): `{netId, epoch, eventId, payload <= 256 bytes}`. `ActorSync::sendAnimEvent`
refuses unless this machine is the committed owner; the host judges a client's event (owner + epoch), relays it, de-duplicates by eventId and
delivers it through `setAnimEventHandler`. The payload is opaque to ActorSync; byte 0 is the event kind (1 = corpse).

## Files

| File | Role |
|---|---|
| `anim_groups.h`, `anim_group_names.inc` (generated by `gen_group_names.py`) | the 245 group names and the classification (action, attack, reload, reaction ...) |
| `npc_anim_engine.h` | `INpcAnimEngine`: the whole engine surface (reads, writes), keyed by NetId |
| `npc_anim_sampler.*` | owner side |
| `npc_anim_driver.*` | non-owner side |
| `corpse.*` | corpse payload codec and the owner-side watcher |
| `npc_anim_system.*` | the one object the glue talks to (+ stats JSON) |
| `script_npc_engine.*` | production `INpcAnimEngine`: console lines + `INpcGameAccess` |
| `anim_game.*`, `anim_module.cpp`, `anim_pins.h` | the game side (x86, xNVSE layouts); self-registering `NVCOOP_MODULE` |
| `verify_slots.py` | static proof of the pinned slots / functions against a reference exe |
| `tests/` | fake engine, sampler / driver / pipeline / ActorSync tests |

Wiring (all through `plugin/actors/engine/actor_engine_api.h`; no edit to `main.cpp` or `coop_module.cpp`): `setAnimSampler` (owner side),
`addDriveListener`, `addActorDiedListener`, `addActorLostListener`; the corpse pipe through `ActorSync::setAnimEventHandler` /
`sendAnimEvent`, re-attached whenever the coop session's ActorSync changes. World epochs come from the NVSE messages (post load, new game,
exit to main menu). Bridge methods: `anim.npc` (counters, pin status), and with `[dev] animprobe=1` also `anim.npc.do` (one backend operation
on a plain reference, for the proof).

nvcoop.ini `[anim]`: `enabled=1`, `directweaponout=1`, `directaiming=1`, `directrate=0`, `directknock=0`, `attacks=1`, `turns=1`, `talking=1`,
`rate=1`, `knockdown=1`.

## What is proven and what is not

Proven statically (against the reference 1.4.0.525 exe, `verify_slots.py`, re-run by `run_tests.cmd`):
* HighProcess vtable slots from the xNVSE header: `SetIsAiming` (0x100, writes byte +0x228), `SetKnockedState` (0x104, +0x13C), `PushActorAway`
  (0x106), `IsWeaponOut` (0x115, reads +0x135), `SetWeaponOut(Actor*, bool)` (0x116, writes +0x135 and then calls two virtual functions of the
  actor, +0x1E4 and +0x1E8). Their addresses are pinned in `anim_pins.h`; the game backend calls one only when the actor's process vtable holds
  exactly that function in that slot, otherwise the console fallback runs.
* The console command names used exist in the command table: `PlayGroup`, `SetForceSneak`, `SetAlert`, `SetPos`, `SetAngle`, `KillActor`
  (`Kill` is not in the table).

NOT proven (each is a step of the in-game proof, and each has a safe fallback):
* that `PlayGroup <name> <flag>` works on a restrained actor, what the flag means (we assume 1 = hold), and that by-name resolves the weapon variant;
* that the xNVSE group numbering is the game's (the sampler and driver both rely on it) and which AnimData slot holds the movement group and the
  weapon-slot action (the backend classifies by group instead of by slot);
* that `SetWeaponOut` draws a weapon on a restrained copy (the driver reads `IsWeaponOut` back and falls back to `SetAlert` and the Equip group);
* that a PlayGroup attack on a remote copy does not fire the weapon through the animation's text keys (combat-hooks must discard hits whose
  attacker `isRemoteDriven`; this layer cannot stop it);
* playback rate (`directrate=1` writes the sequence frequency at +0x28, an offset from the xNVSE header) and knockdown (`directknock=1`,
  `PushActorAway` force units): both OFF by default; without them feet are matched by choosing walk or run, and a knockdown is one Stagger;
* corpse placement with SetPos on a dead body (the proof step 9);
* while an actor is knocked down or ragdolling the actor port keeps writing its position; whether that fights the physics is unknown;
* talking: only the Talking group once per conversation; the voice and lip sync are not touched.

## T7 NPC fight assertions (for the fight playbook)

```python
from animcheck.npc import analyze_npc, check_fight
rep = analyze_npc({"host": host_stream, "client": client_stream}, owner="host", expect_moving={ref: [(t0, t1)]}, ref_maps={"client": {href: cref}})
failures = check_fight(rep)        # [] = pass; add overrides with check_fight(rep, {"pose_err_p95": "<=20"})
```

`FIGHT_ASSERTS` (all must hold): the eight glitch kinds `glitch.sliding / tpose / floating / sinking / jitter / rubberband / stuck / deathpose == 0`;
`wrong_weapon`, `wrong_facing`, `dead_mismatch == 0`; `pose_err_p95 <= 14`; `attack_missing`, `attack_extra`, `reaction_missing`,
`reaction_extra`, `reload_missing`, `reload_extra == 0` (every one-shot the owner plays shows on the copy within 100 ms + 150 ms latency + 300 ms
slack, and the copy plays none the owner did not); `weapon_never_drawn == 0` (owner weapon out for 2 s, copy never); `corpse_missing`,
`corpse_wrong_place`, `corpse_unsettled == 0` and `corpse_err_max <= 40` (resting places within 40 units; a body still faster than 20 u/s
more than 4 s after the death for 0.6 s). Thresholds are first guesses in `NPC_THRESHOLDS` / `NpcThresholds`; none measured. CLI:
`python -I tools/animcheck/npc.py OWNER.jsonl OBSERVER.jsonl [--map OWNERREF=OBSERVERREF] [--json out.json]`.

## In-game proof (the helper runs it, one game at a time)

1. `nvcoop.ini`: `[dev] animprobe=1`, `[anim] enabled=1` (add `directknock=1` and `directrate=1` for the optional steps). Load a save with an
   ordinary NPC standing near the player; give it a weapon (`additem`) for `--weapon`.
2. `python -I tools/animcheck/npc_proof.py caps --instance 0 --ref FalloutNV.esm:<npc id> --weapon` (add `--kill` on a throw-away save).
   PASS looks like: no `[FAIL]` line and `npc_proof caps: N steps, 0 FAIL`. `[INFO]` lines report the AnimData slot numbers and the unsupported
   optional features: copy them back to the maintainer.
3. Two games in one co-op session with NPCs the host owns (the fight scenario): `python -I tools/animcheck/npc_proof.py sync --host 0 --client 1
   --seconds 60 --out out/npc/run1` while the fight playbook runs. PASS: `npc_proof sync: OK` and, per machine, `refused` and
   `weapon_ladder_giveups` near zero in the printed `anim.npc` counters.

## Mutation twins (run-time, `FNVCOOP_MUT=<n>`)

| n | Bug the twin puts in | Caught by |
|---|---|---|
| 1 | no debouncing of weapon / aim flags | `sampler.flickering_flags_are_not_published` |
| 2 | one-shot not held in the stream | `sampler.a_short_one_shot_stays_in_the_stream_long_enough`, pipeline attack test |
| 3 | restart of the same group is not an event | `sampler.the_same_group_played_again_is_a_new_event` |
| 4 | dead actor keeps publishing locomotion / action | `sampler.a_dead_actor_publishes_a_pose_and_nothing_else` |
| 5 | every sample of a playing action is a new event | `sampler.an_attack_is_one_event_with_its_group` |
| 6 | strafe / backward published as forward | `sampler.locomotion_code_has_speed_and_direction` |
| 7, 8 | knocked / block never published | `sampler.knocked_and_blocking_are_published` |
| 20, 21, 22 | corpse pose taken at the death / never sent / sent again and again | `corpse.reported_once_where_the_body_stops`, system and pipeline corpse tests |
| 30 | owner's code alone decides (sliding feet) | `driver.a_lost_or_late_owner_code...`, `pipeline.a_walk_code_at_run_speed...` |
| 31 | no debounce of a speed glitch | `driver.a_speed_glitch_shorter_than_the_debounce...` |
| 32 | dry stream keeps running in place | `driver.a_dry_stream_stands_still...` |
| 33, 34 | weapon flag never set / never read back | `driver` and `pipeline` weapon tests |
| 35, 36, 37 | attacks never played / old event replayed / counter does not wrap | `driver` one-shot tests, pipeline attack test |
| 38 | skeleton assumed present (T-pose) | `driver.waits_for_the_skeleton...` |
| 39, 40 | no re-assert after a menu / a rebuilt skeleton | `driver` menu and rebuild tests |
| 41 | death never verified | `driver.a_death_the_engine_ignored...` |
| 42, 43, 44 | owner's resting place ignored / applied while the ragdoll still moves / body moved though already there | `driver` corpse tests, pipeline corpse test |
| 45 | no command budget | `driver.a_crowd_cannot_flood_the_console` |
| 46 | dead actor still driven | `driver.a_dead_actor_is_never_driven` |
| 47, 48, 49, 50, 51 | no turn animation / no rate / no landing / no knockdown fallback / no sneak | one `driver` test each |
| 52, 53, 54, 55, 56 | world reset ignored / no release on takeover / no read-back / no talking / a drifted weapon state never restored | one `driver` test each |
| 60, 61, 62, 63 | every machine announces the corpse / refused event never retried / counter restarts after a handoff / a body forgotten when its cell unloads | system tests |
| 70 | an unknown group is guessed in the script backend | `script.the_lines_sent_for_each_request` |

Plus the compile-time twins in `ActorSync` (30: anybody may send an anim event, 31: a non-owner event is relayed, 32: a duplicate is applied
twice) and the four live-core twins named above. `ANIMCHECK_MUTANT=npc_attack_match | npc_extra | npc_weapon_never | npc_corpse_err |
npc_corpse_unsettled | npc_proof_pass` for the Python side (plus the shared detector twins, which the fight assertions must also notice).

## Open items (nothing here is hidden by a green test)

* Every item under "NOT proven" above.
* `ScriptNpcAnimEngine` costs console lines. The cost per line is unmeasured; the driver budgets 12 per tick and the actor port budgets its own
  SetPos lines separately, so the sum per frame is not bounded by one number. If the proof shows lines are expensive, the first lever is
  `DriverConfig::budgetPerTick`.
* If the owner leaves the session before its dead NPC's body has settled, nobody announces the resting place: the other machines keep their own
  ragdoll result and count `corpse_missing` after 15 s.
* Ownership handoff while a one-shot plays: the new owner continues the counter, but the animation in progress is not carried over.
* Creatures use the same group ids; creature-specific attack groups beyond the common table are replayed only if their ids are in the table.

## 2026-10-30: why the guest's NPC stayed seated (release 0.1.6 playtest) and what the log shows now

**Root cause.** The hooks module drops every mutating command that a console line (a temporary script) runs on a GUEST unless the line runs
inside the sync layer's own scope (`nvhooks::ApplyScope`; `blockedNow()` in `engine_hooks.cpp`, `SuppressPolicy::blockMutation`). The dropped line
answers success. The actor port's console (`RealEngineHost::console`) opens that scope; the animation layer's runner (`anim_module.cpp`) did not.
So on the guest EVERY animation line of this layer did nothing while reporting Ok: movement groups, sneak, the weapon fallback, the furniture
`Activate`, the stand-up escalation (`SetRestrained 0`, `EvaluatePackage`), the hold (`SetRestrained 1`), the kill retry and the corpse placement.
The port's own `SetPos` lines ran in the scope, so the guest's Doc Mitchell, loaded in the chair like the host's, was moved around the room by
the stream in the pose he had, while the host's Doc (stood up by his scene's own packages) walked. Why no test saw it: every fake engine answered
every line; none had the guest gate. `tests/test_gate.cpp` now has one, and `GuardedScript` (script_npc_engine.h) makes the scope a constructor
argument of the layer's script, so a new wiring cannot forget it (twin 88).

**Also changed.** The last rung of the stand-up ladder is now a reset of the reference (Disable, 400 ms, Enable, hold back; at most 3 per actor,
30 s apart, `[anim] standreset=0` switches it off): the rungs before it only ask the actor's own AI, and on a guest that AI knows the package of
the moment the copy was loaded, not the host's scene. A copy whose engine state reads "leaving the furniture" for 2.5 s is treated as still in
it. The position hold of a stand-up ends after 3.5 s (`lockMaxMs`) and the area digest leaves a held copy alone. The actor port compares a copy
with where the engine really has it (every 250 ms) and writes the pose again when something else moved it more than 28 units (`driftXY`).

**The log (`nvcoop.log`, one grep: `npc:`).** Lines are rate limited per actor and name the actor as `<NetId>(ref <reference id> base <base form id>)`;
find Doc Mitchell by his reference id in the host's lines first (`grep "npc:" nvcoop.log`) and follow the same NetId on the guest.
* `npc: anim: copy <id> first look: engine sit raw=<n> ... stream sit=<n> furniture=<ref> -> driver seat=...` when a copy is first driven and either side
  is in furniture. `raw` is the engine value (0 none, 1-3 going in, 4 sitting, 5 leaving, 6-10 bed). `stream sit`: 0 none, 1 entering, 2 using, 3 leaving.
* `sit-in issued`, `stand-up rung <n> issued ... -> Ok/Refused`, `rung <n> ... did not take`, `is out of the furniture after <ms>`, `given up for now`,
  `last rung: the reference is disabled`, `reset rung finished`. A rung that says Ok and "did not take" with raw unchanged is a line the engine
  ignored (the old failure looked exactly like this).
* `npc: anim: copy <id> DIVERGED for <ms>: stream sit=..., engine sit raw=...` every 3 s while a copy's furniture state differs from its owner's,
  and `agrees with its owner again`.
* `npc: owner <id>: sit state published <a> -> <b>` on the owner's machine (what the host's engine says Doc is doing), and `the sit state is NOT
  readable (published as standing)` if an actor's process does not answer the pinned getter.
* `npc: anim summary: ...` every 10 s while counters move: `commands` vs `groups` vs `mismatches`/`reasserts` (many mismatches with Ok commands =
  swallowed lines), `furniture sits/stands/confirmed/rungsFailed/giveUps/resets`.
* `npc: port <id> AI off/on (...)`, `AI off FAILED`, `AI left as it is: a script already holds the actor restrained`, `snapped to ...`,
  `position held (the copy is getting out of furniture)`, `was moved by something else: engine at ... written again`.
* `npc: areadigest: actor <id> snapped/smoothed: copy at ... authority at ...` and `position heal left alone: the copy is held`.
* `npc: ownership <id>: Transfer/Commit/Grant owner=<slot> ...` for handoffs and named NPCs (who runs Doc on which machine).

New twins: 88 (line runs bare, `test_gate`), 89 (a stuck "leaving" is taken for standing), 90 (no reset rung), 91 (the hold has no end),
92 (no reset cooldown); engine port 88 (never looks where a copy really is); area digest plugin 83 (a held copy is dragged by a heal).
**Not proven in a game:** that Disable/Enable clears the furniture state of a real actor, and what a stood-up Doc does on the guest between the
Enable and the hold. The lines above are what the next run needs to say.
