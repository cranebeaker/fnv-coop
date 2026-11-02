# NPC combat: weapon choice, ammunition, reloads, fight events

Batch `claude/m2-npc-combat`, 2026-10-06, for Ghost Town Gunfight
(`VMS16`/`VMS16b`). Code: `crates/world/src/npc_combat.rs` (rules),
`viewer/src/fighting.rs` and `viewer/src/ai.rs` (calls), `GetShouldAttack`
and `SetUnconscious` in `crates/world/src/scripting.rs`. Status:
**implemented and tested on generated data; not compared with the
original game.** Executable: `FalloutNV.exe` 1.4.0.525; names marked
(Xbox PDB) come from the Xbox 360 prototype's symbols.

## What the game does (traced)

### Weapon rating and the arsenal

- `00646060` `CombatFormulas::CalcCombatWeaponDPS` (Xbox PDB): 0 for
  weapons flagged "not used in normal combat" (`DNAM` flags2 0x40,
  `00646230`); otherwise `00645380` (below); grenades ÷ 10, mines 0
  (`00522c80` kinds 3/4); the actor's combat style weapon restriction
  (`TESCombatStyle` +0x40 on PC = `cWeaponRestrictions` (Xbox PDB), CSSD
  u32 40): 1 → guns (animation 3–13, `004c0c30`) ÷ 10000, 2 → melee and
  fists ÷ 10000. Constants: `01020758` 10.0, `0102f078` 10000.0.
- `00645380` `CombatFormulas::CalcWeaponDamagePerSecond` (Xbox PDB), as the
  controller calls it (no mods, no perks, own skill, own critical
  chance), read from its disassembly (the whole figure, with what the item
  cards and `GetBestWeapon` pass, is in `COMPANIONS.md`, "The damage per
  second"; one translation, `world::dps`):
  - damage = `DATA` damage × `fDamageWeaponMult`, ammunition damage
    effects (`0059a030`: multiplying effects first), + projectiles ×
    explosion damage when the projectile explodes (`00525b20`,
    explosion +0x78); grenades and mines (animation 10/11) use the
    explosion damage × `fDamageWeaponMult` instead and no jam index;
  - × (`fDamageSkillBase` + `fDamageSkillMult` × skill ÷ 100);
  - rounds = the clip (`BGSClipRounds`) for animations 3–9 and 12, else 1;
  - time = rounds ÷ (`DNAM` 88 attack shots/s + rounds × jam chance ×
    `DNAM` 96 jam time), jam chance `fWeaponConditionJam1…10` at index
    round-half-up(condition × 10) (`004bd510`, table `0119b26c`,
    `006477b0`); + `DNAM` 92 reload time when rounds ≤
    `iMinClipSizeToAddReloadDelay` (2) and not melee/grenade/mine/thrown;
  - not the player, a gun: semi-automatic (flags1 0x02 clear) adds rounds
    × (average semi-auto delay − 1 ÷ shots/s) when positive; automatic or
    short burst (flags2 0x200) × (`fAutomaticWeaponBurstFireTime` +
    `…CooldownTime`) ÷ cooldown;
  - critical share = clamp(Critical Chance AV 14 × `CRDT` mult, 0, 100) ×
    0.01 (÷ shots/s for automatics) × `CRDT` damage;
  - condition multiplier `00646d00` (1 above 0.75, else 1 − (0.75 − c) ×
    0.67);
  - DPS = (critical share + damage) × rounds ÷ time; 0 at condition 0;
  - fists: `fUnarmedDamageMult` × AV 56 + `fDamageSkillMult` × AV 45 ÷ 100
    over `fUnarmedNPCDPSMult` (1.57), a creature's own damage over
    `fUnarmedCreatureDPSMult`.
- `00522c80` `TESObjectWEAP::GetCombatWeaponType` (Xbox PDB): animation
  0–2 melee (2); 3–9 and 13 ranged (1), or ranged explosive (0) when the
  projectile's explosion damage > `fDangerousProjectileExplosionDamage` (5)
  and radius > `fDangerousProjectileExplosionRadius` (30); 10 grenade (3);
  11–12 mine (4).
- `009993c0` (`CombatState.cpp` per its assert path; the Xbox
  `CombatState::UpdateWeapons`): candidates are inventory weapons that the
  actor can use (`008bc9d0`: not "player only", flags2 0x1) with ammunition
  carried (`00525980`, `009962f0`, `004c7300`) or none needed; DPS ≤ 0
  drops one. Guns (kinds 0/1) with a target: absolute maximum range
  (`009a9180`), × `fCombatCurrentWeaponAbsoluteMaxRangeMult` (1.5) for the
  weapon in hand; out of reach → set aside (score ÷ 10000, and the
  controller keeps the range at which it would reach, +0x68) unless no gun
  reaches, when the best of them stays. Per kind the best score
  (`+0x3c + kind×4`) and weapon (`+0xc + kind×4`); best DPS +0x58, best gun
  DPS +0x5c, best score +0x60; fists in slot 6 (+0x54). The embedded-weapon
  pick (+0x34, +1 000 000 bonus `0106b798`) is for creatures' embedded
  weapons (`fEmbeddedWeaponSwitchTime/Chance` in `00998a50`).
- Rebuilt at combat start (`00997c30`), when the controller's
  `fCombatInventoryUpdateTimer` (5 s, set at the end of `009993c0`) runs
  out or a flag (+0x77) asks (`00997cf0`, `00999160`), after a disarm
  (`0099a720`).
- Planner costs: action table `011a4280` (name, base cost): ranged
  explosive 1.5, ranged 1.5, grenade 1.5, melee 2.0, hand to hand 2.0,
  switch weapon 2.0, …; an attack action's cost (`0097ae00`, `0097b240`,
  `0097bb90`, `0097b4a0`, `0097b5a0`) = base + `0097ac30` = best score −
  the kind's score (FLT_MAX for a kind rated 0). Followers: ranged costs
  FLT_MAX with the global `CombatStyleMelee` = 1; hand to hand FLT_MAX
  when `008a1710` (not done here).

### Ammunition and reloads

- `008a8dd0` `Actor::ShouldUseAmmo` (Xbox PDB): the player; any actor with
  a weapon flagged flags2 0x2 ("NPCs use ammo", `008a8e30`; set on every
  weapon when `bForceNPCsUseAmmo:Combat`, `0051f456` → `0051f590`); a
  teammate (`00566950`) with a playable weapon (flags1 0x80 clear,
  `0047bcf0`). Vanilla: dynamite has 0x2 (flags2 0x10A); the varmint
  rifle (0x3008) and .357 (0x0C) don't.
- `008a89a0` `Actor::UseAmmo`: the clip loses ammo use × shots; the
  inventory too when `ShouldUseAmmo`; at 0 the reload/next-ammo handling
  and a controller notice (`0097f6d0`).
- `008a8420` `Actor::ReloadWeapon`: thrown weapons are their own
  ammunition; loads a full clip when not using ammo (or a clip's worth is
  carried), else what is carried; none carried → no reload.
- `009993c0` drops weapons without ammunition, so a dry weapon is
  replaced at the next arsenal.

### Fight events and script functions

- `OnStartCombat` (block `0118e6d8`, opcode 0x19, event mask 0x8000 in
  `005ca860`): raised by `00980830` (the controller's target set from none
  to one, the target's group detection level > 0) and by `009887b0` (the
  group first detecting a target), for each non-player group member, with
  its combat target (`005ac750`). `OnCombatEnd` is mask 0x40 (`005ca930`;
  not done).
- `GetShouldAttack` (command `01191360`, handler `005c74d0`, condition
  `0059ed30`): both must be actors (form types 0x3B/0x3C; the caller
  vtable +0x100); early 0 when the caller is in combat and `00992640`
  (combat group test) passes; else 100 when `008b06d0`
  (`factions::attacks_on_sight`) passes.
- `SetUnconscious` (command `01192eb8`, handler `005d0760`): 1 → actor
  vtable +0x434(0) (stop its fights; `StopCombatAlarmOnActor` uses the same
  slot per attacker), life state 3 (`008ace10` → `008a1800`), `0087faa0`,
  process +0x338; 0 → life state 0 only from 3.

## Implemented (with tests)

- `world::npc_combat`: `combat_weapon_type`, `should_use_ammo`,
  `condition_mult`, `jam_index`, `dps`/`damage_per_second`, `combat_dps`,
  `assemble`/`arsenal`, `cheapest_kind`, `choose_weapon` (5 s timer, equip
  or fists), `fired` (clip, inventory, reload over `DNAM` 92 ÷
  `combat::reload_rate`, dry → re-choose), `reloading`,
  `start_combat_event`, `combat_over`. `combat::weapon_in_hand` honours a
  fists choice.
- Viewer: the fight frame chooses the weapon, holds fire while
  reloading, accounts each shot/throw, runs `OnStartCombat` after the
  target's first detection (fight memory seen > 0), resets per fight;
  unconscious actors are skipped.
- Scripts: `GetShouldAttack` evaluated; `SetUnconscious 1` ends the
  actor's fight.
- Tests: 9 unit tests (`npc_combat::tests`, formula cases from the varmint
  rifle's and dynamite's record values) and 7 integration tests
  (`crates/world/tests/npc_combat.rs`, generated `testdata::fighting` now
  with a round, a rifle, dynamite and a raider script).

## Inferred or not done (labelled in code)

- **The planner**: the cheapest single attack action decides the weapon
  kind; ties go to the first in action order. The A* search over the
  planner's world state (`0097ade0`/`00984390` preconditions and effects)
  isn't traced. With these costs gunmen carrying dynamite rarely throw it
  (its rating is ÷ 10); whether the game's planner throws more often
  (e.g. `COMBAT_ACTION_ATTACK_GRENADE_FLUSH_TARGET`) is **not compared**.
- Switching is immediate; `CombatProcedureSwitchWeapon` (`009da7c0`)
  waits for the equip animation. The weapon model in the hand isn't
  swapped in the viewer.
- Reload duration = `DNAM` 92 ÷ reload rate (as the player's); the reload
  animation and its length aren't played/read. The first clip is taken as
  full (or what's carried when ammunition is used up).
- Thrown weapons: no clip/reload model; each throw uses one when
  `ShouldUseAmmo`, and running out re-chooses.
- An ammunition's own projectile isn't used for the explosive test.
  Creatures don't choose (their own weapons and embedded-weapon switching
  untouched). The +0x68 recheck range is computed but doesn't trigger an
  early re-choose.
- `GetShouldAttack`'s combat-group early 0 and the vtable +0x344 call;
  followers' `FollowerSwitchAggressive` (as before).
- `OnStartCombat`'s "detected" is the viewer's detection of the target
  above 0 since the fight began; group detection sharing isn't modelled.
- Unconscious actors: frozen, no knock-down pose; the process flags
  `005d0760` changes aren't traced.
- Out of combat, which weapon an NPC holds is still `actor::best_weapon`'s
  guess.

## Next

Record a Goodsprings powder-ganger fight in the original game (weapon
switches, dynamite throws, reload pauses, VMS16b stage 110 on the
gangers' `OnStartCombat`) and compare.

## Aim, spread and the next target (batch `claude/m2-npc-aim`)

2026-10-06. Code: `crates/world/src/npc_aim.rs` (rules),
`viewer/src/fighting.rs` (`resolve_shots`, `Targets`, `next_target`),
`viewer/src/combat.rs` (`first_met_past`). Status: **implemented and
tested on generated inputs; not compared with the original game.**

### What the game does (traced)

- **Who fires how** — `00523150` (weapon fire), branch for an actor other
  than the player with a combat controller (vtable +0x428,
  `Actor::GetCombatController`, Xbox PDB; the PC slots are the Xbox ones
  + 4 from +0x20c on: +0x20c `GetFireNode`, +0x22c `IsDead`, +0x230
  `IsKnockedOut`):
  - origin: the fire node's world position (`Actor::GetFireNode`), else
    feet + 0.75 × height (`009a8050` → `009a7fa0`, `0101de30`);
  - direction: straight from the origin at the attack procedure's
    targeted point (`009807f0` = `CombatController::GetTargetedPoint`,
    Xbox PDB → action procedure (+0x84, Xbox +0x94 `pActionProcedure`)
    vtable +0x18); no point → no shot. The ranged procedure
    (`009d0a30`) makes the point with `009d1c90` → `009a8bf0`: the
    target's feet + `009a8460` (height × 0.25 / 0.5 / 0.75 (0.9 crouched)
    by the segment in view 0/1(3)/2; × 0.1 when dead, knocked out or
    paralysed; 0 for segment 4, which `009a8bf0` picks for slow
    projectiles), led by the target's velocity (`009a8df0` →
    `009a8f00`, `iAimingNumIterations`) unless the projectile is
    hitscan (`009a7f80` = projectile flag 0x1). The segment is the
    controller's combat state `+0x7c` (`00981920`), default 1 without a
    controller; who sets it isn't traced;
  - cone (radians) = (weapon spread × W₁ + min spread) × π/180, the
    ammunition's spread effects (`0059a030` type 3), + W₂ ×
    `fNPCMaxGunWobbleAngle` (data 15) × π/180 + process `+0x1d0`
    (`008d8330`; written by `008d8350`, decaying in `009295c0`; taken as
    0); W₁ is always 0 (`findings\hits.md` §1); × the split-beam mod;
  - each projectile: r = U(0, cone), θ = U(0, 2π), added to heading and
    pitch.
- **W₂** = `008b0dd0(2)` → `00646910` (`world::vats::wobble`): standing
  0.1, walking 0.1 / running 0.2, not aiming 0.2, crippled arms,
  Strength and skill short of the weapon's, × (1 − skill × 0.005), at
  least 0.01; `008b0dd0` then applies perk entry 34 (`005e58f0(0x22)`)
  for anyone (the existing function applies it only to the player).
- **Aiming down the sights** (the "not aiming" term) — `008f74c0` and
  `009d0a30`: on when |aim point − shooter|² > (`fCombatIronSightsDistance`
  512 × the weapon's sight usage)², the weapon's `+0x170` = `DNAM` f32 at
  124 (`008f7710`; `DNAM` starts at `+0xf4`); melee weapons never
  (`006450c0`); stored as the controller's `cUseIronSights` (+0xc6, Xbox
  +0xd6) and the process's aim flag (`008bb650` → process +0x400).
- **NPC gun sway**: `008b2c30` (`Actor::AddGunWobble`, Xbox PDB) runs only
  for the player (the pointer at `011dea3c`), so people's guns don't sway; their
  wobble only widens the cone.
- **Not read anywhere** (settings inventory: initialiser only): every
  `fGunSpread*`, `fWeaponConditionSpread1–10` (condition doesn't change
  spread), `fCombatRangedStandoffTimer`, `fAICombatTargetUnreachable
  PriorityMult`, `fAICombatUnreachableTargetPriorityMult`,
  `fAICombatNoTargetLOSPriorityMult`. There is no `fAIMaxRangedAttack`.
  Range: the projectile's (`PROJ` range); the hitscan cast runs that far.
- **Next target** — `0097f4d0` (a target removed from the controller):
  when it was the current one, `00980830(00986c60(attacker))` =
  `CombatController::SetTarget(CombatGroup::GetBestTarget)`, Xbox PDB;
  no target left ends combat. The controller update (`0097da50`,
  `CombatController::Update`) picks again whenever it plans anew.
  `00986c60` over the group's targets (`CombatTarget`, Xbox PDB):
  skipped outside the style's targeting FOV (`00639aa0`, CSSD 48), in
  another cell under a flag (`00408d60`), or, for teammates, ones
  they wouldn't attack (`008b06d0`); group detection level (`009887b0`:
  the highest member's) below 1 → candidate for the "most recently
  detected" fallback; else, with more than one target, score: +1000 in
  view (detection record +0x1e, `cMemberLOSCount`), +100 360° line of
  sight (+0x1c), +100 current (+1000 more when out of view and seen
  within 2 s, controller +0x98), +100 same melee/ranged kind
  (`009a9630`), + (1 − min(d², 2048²)/2048²) × 1000, + `011f18b0` (0)
  when others attack it, −2000 near an unreachable location
  (`009a0f40`), −2500 another cell not loaded (`009a96f0`), −500
  `IsDead`/`IsKnockedOut`, −500 `008a6650`. Weights `011a4d50`–`011a4d78`
  (500, 500, 2500, 2000, 1000, 2048, 100, 2.0, 100, 100, 1000).

### Implemented (with tests)

- `world::npc_aim`: `aim_height`, `fire_height`, `actor_height`,
  `sight_usage`, `uses_iron_sights`, `npc_cone`, `deviate`,
  `heading_pitch`/`direction`, `best_target` (8 unit tests).
- Viewer: a person's shot is queued and flown after the AI frame
  (`resolve_shots`): from feet + 0.75 × height to the target's feet +
  0.5 × height, cone = `Weapon::shot`'s + wobble (gait: walking, fast
  walk → walking; run → running; aiming by the sights rule) × 15°, per
  pellet a ray through `first_met_past` (skeleton capsules, scripted
  objects, walls; the shooter passed by) and the player's bounds
  cylinder; the pellet's share of the damage on the part struck
  (`hit_at`); misses strike the world (impact effects). Line of fire
  (`009d0a30` → `009a6e90`: the hit actor must be the target or one of
  the group's targets, `009865b0`): someone else first on the straight
  line holds the shot (the round and the sound are already spent: a
  simplification). A scripted object whose bounds contain the shooter
  doesn't stop the shot (bounds stand in for collision).
  Fighters keep their targets (`Targets`): the one they start with,
  everyone rising whom they'd attack (in a fight too), allies' enemies
  they'd help against (out of a fight only); on a kill or give-up,
  `next_target` → `best_target`.
- Leveled inventories: `choose_weapon` stocks the fighter first
  (`GameState::stock`), so a base whose guns come from leveled lists
  (`GSPGHM`: `WithAmmoNVVarmintRifleLoot`, `RaiderLoot`, dynamite lists,
  all `LVLF` 0x04 "use all") is armed; before, every Powder Ganger chose
  its fists. When the game resolves them isn't traced.
- Tests: `crates/world/src/npc_aim.rs` tests; viewer
  `fighting::tests` (target list and choice on generated values);
  `crates/world/tests/npc_combat.rs`
  `a_gun_from_a_leveled_list_is_chosen_with_its_rounds` (generated
  gunman with a use-all rifle kit).

### Gunfight re-run (2026-10-06, private outputs `nv-re\work\npcaim-2026-10-06`)

As `docs/GOODSPRINGS_ROUTE.md` (player at −67845,3000, 150 s): run 7 with
Trudy's help, run 8 without; both reach stage 100 ("Defeat the Powder
Gangers"). The varmint-rifle ganger (GSPG03, 00104C75) shooting the
standing player while walking in: 1 hit in 10 shots (run 7) and 3 in 9
(run 8) between 3000 and 2000 units, cones 1.3° (standing, aiming) to
2.6° (walking); run 1 of the route batch had 10 hits in 10. Many misses
strike the ground short of the player. Gangers now carry their
rifles, revolvers, shotguns, bats, cleavers and dynamite. Fighters
"turn to" their next target after kills; strays occasionally hit
bystanders, and one ganger killed another. **Not compared** with the
original game.

### Inferred or not done (labelled in code)

- The fire node and the animated gun aren't used: the origin is the
  game's no-fire-node fallback. Segment in view fixed at 1 (middle).
  Projectile lead not done (hitscan bullets don't lead; missiles, flames
  and lobbed shots are resolved as rays). Process `+0x1d0` aim offset 0.
- The player is met by their bounds' cylinder (no third-person capsules);
  people crouching (sneaking) aren't modelled, so the crouched heights
  aren't used.
- Perk entry 34 isn't applied to people's wobble (`world::vats::wobble`
  applies it to the player only).
- Target choice: no combat groups — each fighter's own targets and own
  detection stand in for the group's; teammates' rule, the cell/area
  penalties, `008a6650` and the planner's re-choice timer aren't done;
  `same kind` ignores the combat plan's melee actions (`00981940(7|8)`);
  "in view" = line of sight within `fDetectionViewCone` (inferred from the
  LOS/360° LOS counter names).
- Not verified on the CPU with `nv-call` (the functions read actor and
  process objects).
- Stray hits on bystanders went through the old "whoever is hurt
  fights back" rule; replaced by `Actor::AttackedBy` (next section).

## Hits from allies, friends and strays (batch `claude/m2-friendly-fire`)

2026-10-06. Code: `crates/world/src/combat_ai.rs` (`attacked_by`),
`crates/world/src/factions.rs` (`fights_when_hit`),
`crates/world/src/crime.rs` (friend-hit record), `Runner::attacked` in
`crates/world/src/scripting.rs`, `viewer/src/fighting.rs` (new targets).
Status: **implemented and tested on generated inputs; not compared with
the original game.**

### What the game does (traced)

- `008987f0` = `Actor::AttackedBy(Actor*, ActiveEffect*)` (Xbox PDB), the
  victim's reaction to a hit (its callers are the hit handling): returns
  at once when the victim is dead (vtable +0x22c), a ghost (`008ace90`),
  hit by itself, or in life state 4 with the attacker as its commanding
  actor. When the victim has a combat controller (+0x428) and the
  attacker is already one of its targets (`0097fa10` =
  `CombatController::IsActoraCombatTarget`, group target list), only the
  last step runs. Otherwise:
  - the victim's detection of the attacker is updated (process +0x504 /
    +0xf0), level 3 when above a threshold;
  - **the player as attacker**: a Friend (reaction 3) or Ally (2) counts the
    hit in `ExtraFriendHits` (extra 0x45) when the allowance
    (`iFriendHitCombatAllowed` 3 / `iFriendHitNonCombatAllowed` 0 /
    `iAllyHitCombatAllowed` 1000 / `iAllyHitNonCombatAllowed` 3, "combat" =
    the victim has a controller) is above 0 (disassembly `00898ae7`);
    1000 or more clears the record first (`00422670` = `RemoveFriendHits
    Extra`); `00422590` (`AddFriendHit`) → `00435d40`: hits older than
    `fFriendHitTimer` (10 s, `00435e40` `RemoveOldHits`) are dropped, a hit
    within `fFriendMinimumLastHitTime` (0.5 s) of the last isn't added;
    count ≤ allowance → forgiven (a remark, `009839b0`, unless `008a67f0`,
    `009336c0`, `008a78f0(5)`); else the assault alarm `008c0460`.
    `SetIgnoreFriendlyHits` (`005a3790`) returns before counting;
  - an NPC attacker hitting a guard (+0x304 `IsActoraGuard`, Xbox +0x300)
    with no controller raises the assault alarm with that attacker (not
    done);
  - **start of combat**: `008b0670` (`Actor::CanAttackActor`, Xbox PDB):
    attacker the player → `GetShouldAttackActor(player, attacked)`; victim
    the player or a teammate (`00566950`, actor +0x18d) → yes; else
    `008b06d0(attacker, attacked = 1)`. **And** the attacker is the player,
    or the attacker's combat target (+0x42c) is the victim, or the
    victim's commanding actor (`008b0ba0`, process +0x52c) is the player
    and the attacker's target is the player; and the victim isn't the
    player → process +0x33c `EnterCombat` (Xbox PDB). So a stray shot from
    someone fighting somebody else never starts a fight.
  - `SetBeenAttacked(1)` (+0x474); with a controller, `0097f580`
    (probably `CombatController::DamagedByAttacker`, Xbox PDB, not
    confirmed): an attacker with a process, not already a target, whom
    `008b06d0(attacker, attacked = 1)` says to fight → `0097f930`
    (`CombatController::AddTarget`).
- `008b06d0` = `Actor::GetShouldAttackActor` (Xbox PDB) with the attacked
  flag: the victim frenzied (aggression 3) → yes; else yes when the
  attacker is frenzied or the reaction (`008b87a0`) is neither friend (3)
  nor ally (2). A teammate attacker is evaluated as the player; a teammate
  victim uses aggression 1 and the attacker's reaction to the player
  (`FollowerSwitchAggressive` ≠ 0), never fights a child; perk entry 0x0F
  last. Faction reaction `008b87a0`/`008b8740`/`0048c1b0` is only `XNAM`:
  no implicit "same faction" alliance. `GoodspringsPowderGangFaction`
  (00104C7E) has `XNAM` to itself = 2 (ally), so the Powder Gangers and
  Joe Cobb are allies of each other.

### Implemented (with tests)

- `combat_ai::attacked_by` and `factions::fights_when_hit` (translated);
  `Runner::attacked` uses them for hits and explosions between people
  other than the player: allies and friends tolerate hits, strays start
  no fights, and someone fighting takes a non-friend who hurts them on as
  another target (`GameState::hit_targets`, moved into the fighter's
  `Targets` by the fight frame; never an ally, so allies don't become each
  other's targets).
- The player's friend hits: allowance 0 counts nothing, the 10 s window,
  the 0.5 s minimum gap, the reset above 999; `GetFriendHit` = hits in the
  window (`world::crime::friend_hit_count`).
- Tests: `crates/world/tests/friendly_fire.rs` (4, generated fighting
  world: ally tolerance, stray vs aimed, new target, frenzy);
  `crates/world/tests/functions.rs`
  `a_fighting_friend_forgives_three_hits_within_ten_seconds` (and the
  allowance-0 count changed to 0).

### Gunfight re-run (private outputs `nv-re\work\friendlyfire-2026-10-06`)

Two runs of `docs/GOODSPRINGS_ROUTE.md`'s command with `--wait 330`: no
Powder Ganger fought another (none took on, turned to or helped against
a gang member); stray pellets still hurt allies (run 2: 10 hits, one
fatal). Run 2 reached stage 100 (~197 s); in run 1 the town lost (two
gangers survived; the automation player doesn't shoot). Details in the
route file, fix 4. **Not compared** with the original game.

### Not done (labelled in code)

- The player hurt by people and the player's hits past the allowance keep
  the old "fights back" entry (for the player it is only the in-combat
  state used by the HUD and level-ups).
- Detection update on being hit, guards' assault alarm for NPC attackers,
  `SetBeenAttacked`, the remark (`009839b0`), life state 4, the perk entry
  point, `008bffc0` (player attacker), `FollowerSwitchAggressive` = 0.
- "Already a target" is the current target or one queued from hits (the
  viewer's full `Targets` list isn't visible to the world crate).
- Commanding actor = the player's teammates.

## Shots passing through Powder Gangers (batch `claude/m2-npc-hits`)

2026-10-06. Ghost Town Gunfight stopped completing: people's shots at the
gangers at close range almost all missed (replay of `b1d2b95`: 86 missed
shots to 3 hits under 400 units, counted as below; Easy Pete missed a ganger 75 units away with a
5.57° cone again and again).

### Cause (found by logging the ray against the target's capsules)

Not the pose, placement, heading, scale or aim: the straight ray passed
2–7 units from the target's pelvis and spine bodies. The targets were
missing from the viewer's list of the place's people (`Talkers`), which is
all `combat::first_met_past` tests. The gangers come in through
`GoodspringsPowderGangMarker.Enable` (`bring_in_enabled`), which added
them to the list but didn't record them as brought in after loading
(`BroughtIn`); the next change of loaded squares rebuilt the list from the
squares (`exterior::stream_squares`, which left them out while disabled)
and dropped them. Every one of 77 logged close misses was at someone not
in the list. Fixed: `bring_in_enabled` records them (`BroughtIn::remember`);
regression `people_enabled_after_loading_stay_when_squares_change`.

### What the game does (traced, for comparison)

- The line of fire (`009d0a30` → `009a6e90`) is a ray pick on the
  projectile layer 6 (`004a39f0(6)`, pick data → `009c0d80`), from the
  shooter's point `009a82b0` (`009a8050`'s feet + 0.75 × height, moved out
  to the shooter's edge along its heading). A ray meets the living
  bodies' capsules exactly, as the viewer's `RagdollRig::ray_hit` does; the
  skeleton's capsules (`meshes\characters\_male\skeleton.nif`, radius =
  radius 1 = radius 2) are thin: pelvis (on `Bip01 NonAccum`) 3.1, spine
  6.9 / 9.2 / 10.2, head 7.7, upper arms 1.9, forearms 1.6, thighs 5.6.
- Blocked line of fire: `009d0a30` doesn't fire and sets the combat
  state's `bTargetBlocked` (+0x72, `CombatState`, Xbox PDB; `0097cd10`
  via `00586150` = controller +0x9c, Xbox +0xac `pCombatState`). Its only
  reader, `00997cf0`, re-plans (`0097f200`) when the target is in 360° line
  of sight, not blocked, and the plan holds action 0x18
  `COMBAT_ACTION_ACQUIRE_LINE_OF_SIGHT` (table `011a4280`). So moving to a
  clear line is the planner's (not traced; the viewer holds fire and moves
  only by the engage procedure's band and strafe timers).

### Gunfight re-runs (private outputs `nv-re\work\npchits-2026-10-06`)

Route `vms16` of `scripts/acceptance.ps1` (Trudy's help, 330 s). Shots
by people at people (not the player), counted per shot (a shot with any
pellet striking counts as a hit), by the distance logged:

| Run | < 100 | 100–200 | 200–400 | ≥ 400 | XP +50 |
| --- | --- | --- | --- | --- | --- |
| Before (`b1d2b95` replay) | – | 0 / 23 | 3 / 63 | 12 / 482 | no |
| Before (`3fe4ba4`, logged) | 0 / 12 | 0 / 50 | 0 / 33 | 37 / 503 | no |
| After, run 1 | 1 / 0 | 1 / 2 | 9 / 7 | 21 / 37 | yes |
| After, run 2 | 1 / 0 | 7 / 12 | 2 / 4 | 14 / 36 | yes |
| After, run 3 | 1 / 0 | 3 / 2 | 2 / 3 | 17 / 34 | yes |

(hits / misses). Three runs out of three pass; `doc` and `vcg02` pass.
Pellets striking someone other than the target: 1, 4 and 4 a run (the
gangers hit each other or Easy Pete now and then; one ganger was killed
by another in run 3); "holds fire" with the player in the way 0, 2, 0
(7 in the `b1d2b95` replay, when the gangers outlived everyone). From the
first shot to stage 100 took 36–52 s, so nobody stands shooting for long.
**Not compared** with the original game.

### Not done

- The planner's reaction to a blocked line (`ACQUIRE_LINE_OF_SIGHT`,
  `IGNORE_BLOCKED_TARGET`) and the line-of-fire origin's move to the
  shooter's edge (`009a82b0`).
- A ganger staggered low by a leg-hit reaction (`1stPHitLegLeft`,
  `1stPMT_HitLegLeft.kf`) is still aimed at half his standing height
  (`009a8460` lowers the point only when dead, knocked out or paralysed),
  so close shots pass over him; whether the game's controller height
  follows the animation isn't traced.
