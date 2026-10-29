# combat-hooks: hits, deaths, projectiles and explosions in the plugin (milestone 4, FULL-SYNC-PLAN 3.4)

Code: `plugin/combat/` (game-free core, tests, x86 glue `combat_module.cpp`), on top of `synccore/combat/` (combat-core: the rules) and
`plugin/actors/engine/actor_engine_api.h` (actor-port: who owns which ref). Wire: `docs/protocol.md` section 14 (first byte `0xCB`).
RE: `docs/re/hit-path.md`. Status 2026-10-21: **unit-tested game-free only. NOTHING here has run in a game.** The glue compiles and links into nvcoop.dll.

## Run it
- `plugin\combat\run_tests.cmd` (shared tree `build\all` through `tools/build/build.py`; prints `ALL OK` = the real suite is green and every mutation twin
  is caught). It is in `tools/test_all.py` ("M4 combat-hooks ...").
- `python -I plugin/combat/proof/combat_proof.py --selftest` (the in-game proof checker, game-free).

## What happens to a hit
```
attacker's machine                              host (relay)                     victim's owner
engine hit funnel (ApplyHitData 0x89A760)
 -> HitGateThunk (6 call sites redirected)
    CombatSync::onEngineHit(hit)  -- BEFORE the engine applies the damage
      attacker is a copy here?        -> Neutralize        (its owner reports the hit)
      victim simulated here?          -> Proceed           (native; vitals published ~100 ms later)
      victim is a copy / puppet?      -> MHit ---------->  re-route by the host's table  ----> HitValidator::judge
                                         Clamp (copy keeps >= 1 hp, no fatigue / limb / lethal flags)   applyHit (hp, limbs, named floor)
                                                                                                engine takes the hp off (native replay or console)
                                      <---------- MVitals (health, crippled limbs) to everyone <----
                                      <---------- MHitNack when it could not resolve it (handoff, table behind, victim unknown): re-send (3x)
```
- **Hit capture.** The engine's damage path, not the xNVSE OnHit event: OnHit fires after the damage and carries no numbers. `ApplyHitData` reads the damage
  from the hit data exactly once, so editing the hit data before the call controls what the engine applies (docs/re/hit-path.md). The gate captures
  attacker, victim, health/fatigue/limb damage, body part, impact point, weapon, projectile or explosion object, crit flag, and the VATS mode.
- **Suppression on non-owners.** `Clamp`: the copy takes at most `health - 1`, fatigue 0, limb 0, fatigue knock-out and fatal / dismember / explode / cripple
  bits cleared; it plays the hit reaction but cannot die from a prediction. `Neutralize`: all damage fields 0 (a copy "hitting" something, a visual explosion,
  a visual projectile, a player-vs-player hit with friendly fire off). Death on a copy is also refused one level up by the actor port (`onLocalDeath`).
- **Owner applies.** `HitValidator` (combat-core) checks owner, id, alive, bounds, attacker identity and epoch, cell, range, impact point, line of fire, rate.
  The engine then takes the validated damage off: **native replay** (a stack ActorHitData built by the engine's own constructor 0x9B4D90, source = the
  attacker's puppet or NPC copy, sent through ApplyHitData: hit reaction, blood, OnHit scripts, the victim noticing the attacker, native death) or, when
  `[combat] native=0`, the pins differ, or the replay faults (SEH, automatic), console lines on the victim (`DamageActorValue Health n`, `StartCombat`).
- **Player as victim.** An NPC's owner hits the player's PUPPET (restrained copy on its machine); that hit is forwarded to the player's own machine, which
  replays it on the real player. A copy of that NPC "hitting" the local player on the player's machine is neutralised, so damage arrives once.
- **Death.** The actor port decides THAT it dies (owner, or a host KillActor for a named NPC) and reports it (`addActorDiedListener`). `onActorDied` adds the
  declaration (`MDeath`: killer = the last accepted hit within 15 s, weapon, crippled limbs) once per actor, pays kill XP once (killing blow or party share) and
  decorates the corpse when it exists. A named NPC is declared by the host only; combat alone floors it at 1 hp.
- **Projectiles / explosions.** Declarations travel (`MProjectile`, `MExplosion`); remote ones are visual-only: while one exists, hits from copies, sourceless
  hits at the blast position and sourceless projectile hits are neutralised. Explosion damage is one hit per victim to that victim's owner with an id derived
  from (explosion, victim). What is NOT there: the engine feed that announces a fired projectile and the code that creates the visual copy (TODO-RE below).
- **VATS interface (Track B).** VATS shots must reach the funnel through its normal callers; the gate then marks `vats` from the VATS mode word
  (0x11F2250+8, "proven" in docs/re/dialogmenu-menus-vats-inventory.md). Track B code that resolves a VATS hit itself (a direct ApplyHitData call or a health write)
  bypasses the gate and must call `nvcoop::combat::glueGate(EngineHit)` first (combat_glue.h) and obey Proceed / Clamp / Neutralize. Real-time VATS needs nothing
  else from combat: the hit is a normal `MHit`, ordering and idempotency are the same.

## Interfaces
| With | Contract |
|---|---|
| actor-port (`actors/engine/actor_engine_api.h`) | read: `ownerOfNet`, `refOf`, `netIdOf`; events: `addActorDiedListener`, `addActorTrackedListener` (named flag), `addActorLostListener`. `simulatedHere = mine && !pending && !standIn` (the same as `!isRemoteDriven`). `setHitForwarder` / `onLocalHit` are NOT used: CombatSync classifies by itself with the same ownership view |
| combat-core (`synccore/combat/combat.h`) | `HitEvent`, `HitValidator`, `applyHit`, `LimbState`, `DeathDecl`/`DeathLedger`, `ProjectileRegistry`, `ExplosionLedger`, `planExplosionHits`, `XpLedger`, `HostilityBook`, `consoleLineFor`. Death declarations are judged with the world's `alive` flag MASKED (the copy may already be killed by ActorSync; the ledger decides "already dead") |
| dll-wiring (`plugin/coop/live/live_glue.cpp`) | calls `nvcoop::combat::glueFrame(rt)` each frame (CombatService) and `setPlayerRefMap(slotOfPuppetRef, glueAvatarRef)` when avatars are on. `[live] combat=1` needs `actors=1` |
| avatar-port | a puppet must map ref <-> slot (`setPlayerRefMap`); default is the NV:MP ghost map, which does not know the host's puppet on a guest |
| Track B (VATS UI) | `glueGate()` above; `EngineHit.vats` |

## nvcoop.ini `[combat]` (read by the module at DeferredInit; `[live] combat=1` must also be on)
`enabled=1` (module on), `gate=1` (install the six call-site redirects when a session starts), `native=1` (native replay; falls back to console on a fault),
`friendlyfire=0` (player-vs-player damage), `crime=0` (witnessed attacks -> SetEnemy effects), `xpmode=0` (0 killing blow, 1 party share).
Bridge: `combat.status` (counters; the in-game proof reads it).

## Engine facts and fallbacks
| Piece | Status | Fallback if it is wrong |
|---|---|---|
| ApplyHitData 0x89A760, six `E8` call sites, entry bytes pinned | static RE, bytes unit-tested; confidence M for the name, V for the bytes | pin mismatch => nothing is patched, nothing written, log line |
| ActorHitData offsets (health 0x14, fatigue 0x1C, limb 0x20, flags 0x58, source 0, weapon 0x30, location 0x10) | re-verified in the body of the funnel | SEH-guarded reads/writes; an unreadable hit proceeds untouched |
| Native replay (ctor 0x9B4D90 + ApplyHitData) | static RE only; a stack hit built by the engine's constructor | SEH catch => console damage for the rest of the session |
| Limb condition AV names (PerceptionCondition ...) | strings found in the exe; console parameter parsing not tried | `ExecOn` failure is counted (`consoleFailed`) |
| `DamageActorValue` / `ForceActorValue` / `StartCombat` / `RewardXP` console lines | commands exist in the table; run through `nvhooks::execConsoleLineOn` inside `ApplyScope` | counted |
| Victim/attacker identity (netId <-> ref) | actor port | untracked => the hit proceeds locally |

## Mutation twins (runtime, `FNVCOOP_MUT=n`)
`combat_sync.cpp` 701 copy's hit not neutralised, 702 unowned actor not resolved locally, 703 explosion hits get plain ids / no declaration, 704 copy takes the full
local damage, 705 host trusts the sender's owner, 706 no NACK, 707 owner never takes the health off, 708 no provoke, 709 vitals from a non-owner, 710 stale-epoch
vitals, 711 no vitals sequence, 712 story kill by a non-host, 714 no deferral of a declaration, 716/719 host does not pass projectiles/explosions on, 717 re-send
of a consumed id, 718 no blast planning, 720 every received hit applied, 721 every declaration counts, 722 visual explosion not recognised, 723 visual projectile not
recognised, 724 the world's alive flag believed for a death, 731 no kill credit remembered. `call_patch.cpp` 761 wrong prologue accepted, 762 wrong site bytes
accepted, 763 a chained-over site overwritten. `hit_edit.cpp` (the byte-level gate and the native hit filler) 771 a clamp raises small damage to the cap, 772 a
neutralised hit keeps its health damage, 773 fatigue not zeroed, 774 limb damage not zeroed, 775 lethal flags kept, 776 left and right leg share a body part, 777
base weapon damage left 0 in the native hit. Also caught through these tests (combat-core's numbers): 301 302 307 308 309 317 318 329 330 331 334 337 338 344. 713 and 715 are retired numbers.

## A guest's blow on a host-owned copy: exact damage, light blows and touches (2026-10-29)
Playtest: the guest punched a non-hostile Goodsprings settler the host owns (on the guest a restrained copy driven from snapshots). Nothing happened and the guest's log had
no combat line. The engine's melee target search has no restrained / AI-off / hostility filter (static RE of 0x9A60E0, called from the swing function 0x899200), so the
likely cause is in the gate: `buildHit` rounded the health damage to whole points and refused a result of 0, so a fist under half a point was NEUTRALISED without a log line
(the module logged only Proceed / Clamp). What the gate does now, for any blow by a combatant simulated here on a synced actor copy (hostile or not, armed or not):
- **Exact damage.** `MHit.centi` carries the engine's health damage in hundredths (varint, `kMaxCenti` = 1,000,000 points, a larger value is refused when decoding). The rules'
  own `HitEvent.damage` stays whole points (the nearest, at least 1 for any blow that hurt) and only gets the hit judged. The owner applies the exact amount natively
  (`ICombatEngine::damageHealthExact`, the native replay takes a float): a 0.5 fist takes 0.5 off the settler, never 1. When the rules change the amount (a named / essential
  floor, a lethal replay) their whole points are applied instead. A build that sends no `centi` falls back to the rules' points.
- **Light blows** (exact damage under one point). The rules' whole-point health is not touched; the owner replays the blow natively with exactly that amount
  (`ICombatEngine::touchVictim(..., amount)`), provokes the victim against the attacker's puppet, and publishes the coalesced vitals so the copies follow the engine's float
  health (two 0.5 fists = one point off, on both screens). Counters `lightHitsSent` (attacker) / `lightHitsApplied` (owner).
- **Touches** (exactly 0 health damage, a hundredth counts as 0: armour took it all). Sent as damage 1 for the rules plus `MHit.touch`; the owner replays a token-amount hit
  (0.01 health) for the reaction, provokes, takes nothing off. Only for a blow on an ACTOR by a combatant simulated here; explosions, negative amounts, hits with no attacker
  and hits on a player are still neutralised. Counters `touchesSent` / `touchesApplied` / `touchesEngine` (the engine took the replay).
- **Diagnostics.** The module now logs `Neutralize` decisions on synced victims (own budget of 60) with fatigue, limb and body part. `combat.status` adds `gate.byPlayer` (the
  local player's hits that reached the gate on a synced actor), `gate.byPlayerOnCopy` (... and the victim was a copy: it not rising after a punch means the engine produced
  no hit on the copy at all, an engine-side problem), `native.touchReplays` and `stats.lightHitsSent`, `lightHitsApplied`, `touchesSent`, `touchesApplied`,
  `touchesEngine`, `hitsOnCopies`, `provoked`, `hitsForwarded`.
- **Twins** 801 (a light blow stays 0 and is dropped as a touch), 802 (a zero-damage blow is dropped), 803 (a touch is applied as a point), 804 (the owner's engine never sees a
  light blow / touch), 805 (no provoke), 806 (touch flag not on the wire), 807 (exact amount not on the wire), 808 (hit_edit: a touch's base damage follows its token health),
  809 (the amount is rounded to whole points before sending), 810 (a light blow is applied as the rules' point), 811 (fractions rounded away on the owner), 812 (a light
  blow's result is not published). Tests: `tests/test_melee.cpp`, `test_wire_fuzz.cpp`, `test_hit_edit.cpp`.
- **Not verified in a game.** The token-amount replay for a touch (does the engine play a flinch for 0.01 health?), and whether the guest's engine produces a hit on the copy
  in every case. Proof: `python -I plugin/combat/proof/melee_proof.py run --ref FalloutNV.esm:104F0A` (see its docstring; `--selftest` is game-free). A protected story NPC
  (FriendlyFireGuard) blocks the first two deliberate hits on the host by design: use a non-protected settler.

## In-game proof (for the helper; code-only agents did not run any of it)
1. Two copies, host + client in one session, `nvcoop.ini` `[live] actors=1 avatars=1 combat=1`, `[combat] enabled=1`. `nvbridge combat.status` on both: expect
   `{"enabled":true,"running":true,"gate":{"sites":6,...,"errors":0},"native":{"pinsOk":true,"faults":0,...},"stats":{...}}`. `gate.sites` 0 or `native.pinsOk` false
   means the pins did not match this exe: stop, tell the maintainer.
2. `python -I plugin/combat/proof/combat_proof.py snapshot --out out/combat/before.json`.
3. Fight: `python -I tools/bots/run_coop.py fight` (fight-test's playbook; 2 players against the same raiders), or by hand: both players shoot one raider, then
   one of them kills it, then a raider shoots a player.
4. `... snapshot --out out/combat/after.json` and `... check out/combat/before.json out/combat/after.json`: every line must say PASS.
5. Look for: the raider's health equal on both screens after each hit; no hit flinch without damage on the shooter's screen for more than ~200 ms; the corpse and its
   loot appear once; the killer's XP arrives once; the second player takes damage ONCE per raider bullet (a double hit would show as `hitsApplied` doubling).
6. If a crash or `nativeFaults > 0`: set `[combat] native=0` and repeat 2-4 (console damage path), report the crash address.

## Open items (honest list)
- **Never run in a game.** Everything engine-facing is static RE: the six call sites, the pinned bytes, the hit-data layout, the constructor, the AV names.
- **TODO-RE:** the projectile-create / fire function (no feed for `onProjectileFired`; no visual projectile: `spawnVisualProjectile` returns false), the blast
  radius of an Explosion object (no blast planning for real explosions; their hits are plain hits; `spawnVisualExplosion` returns false: PlaceAtMe needs a
  reference at the blast position), the tier / perk scaling of kill XP, the victim's faction (crime sync reports nothing), where the death logic is entered (so
  a copy could be stopped before the clamp).
- **Kill XP** is the raw value of the victim virtual at vtable +0x730 (found by reading the engine's own award code inside ApplyHitData, confidence M, SEH guarded,
  no tier / perk scaling) paid through the console `RewardXP`; when the LOCAL player's own native hit killed the actor the engine already paid and CombatSync adds
  nothing (`xpEnginePaid`). Compare one kill against vanilla in the in-game proof.
- Damage that does not pass the funnel (poison, radiation, fire damage over time, fall damage, script `DamageActorValue`) is not synchronised or suppressed: a
  copy can still die from it; the actor port refuses that death (`onLocalDeath`), the owner's own copy of the effect decides.
- The player's own death/respawn is polled once per frame (health / life state); the corpse and the death screen are the engine's.
- Limb crippling follows combat-core's accumulated-damage model (head 30, limbs 25 hp of hit damage), not the engine's per-creature limb health; natural hits on
  an actor the machine owns use the engine's own limb logic and publish the resulting crippled mask.

## A natural lethal hit and the life state (2026-10-27, vats lab room)
The owner's engine applies a hit on its own actor natively (`gate PROCEED ... victim is resolved here`; VATS shots go through the same funnel, the gate
logged shots 1, 3 and 4 of the lab run). In the lab's VATS room shot 4 took the health to -14 on the host while the client showed 1 (the actor port's
floor for a mirrored health: it never writes 0, the death is its own message). The death is declared only by the actor port's poll of the LIFE STATE
(Actor+0x108 = 1 or 2); the combat view's `alive` is also false for health <= 0, so a victim with health <= 0 and a life state that never flipped (a VATS
kill: slow motion, kill cam) looked dead to everything that decides and nothing declared it. `CombatSync` now follows up every natural hit whose
published vitals show a victim at health <= 0: after `lethalSettleMs` (1500) the owner kills it (`ICombatEngine::killActor`) unless
`ICombatEngine::lifeDead` says the engine did, a named or essential victim is left to the engine, and the same `lifeDead` test replaces `isDead` in the
accepted-hit follow-up (`killConfirmMs`). Counters in `combat.status`: `naturalLethal` (victims seen at health <= 0 after a natural hit whose own death had not
already declared them) and `naturalLethalStuck` (the ones the owner had to kill; 0 after a VATS kill means the engine did finish them and the missing death is elsewhere).
Twins 792 (never followed up), 793 (isDead instead of lifeDead), 794 (a protected victim is killed), 795 (a healed victim is killed): tests/test_flow.cpp `combat_death`.
A dead copy legitimately differs from the owner's corpse in health (-14 against the port's floor 1), so a lab check that compares the health of a target the
shot has just killed cannot tell a delivered death from a lost one; the check after it ("dead on both copies") can.

