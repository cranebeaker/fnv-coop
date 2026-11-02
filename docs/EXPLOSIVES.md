# Explosives: thrown weapons, grenades and explosions

M2 blocker for Ghost Town Gunfight (`VMS16`, objective 40: Easy Pete hands
out `WeapNVDynamite`). Branch `claude/m2-explosives`, 2026-10-06. Launchers,
explosive missiles, the other grenades and mines (M4): branch
`claude/launchers-mines`, 2026-10-07 (Xbox decompiles of the `Projectile`,
`MissileProjectile`, `GrenadeProjectile`, `Explosion` and `CombatFormulas`
functions in `%USERPROFILE%\nv-re\decomp\launchers`).

Sources: FalloutNV.exe 1.4.0.525 (unpacked image, Ghidra project copy
`ghidra_8`; private exports in `%USERPROFILE%\nv-re\work\explosives-2026-10-06`),
class names from the Xbox 360 prototype symbols (Xbox PDB), records read with
`nvinspect` from `FalloutNV.esm`. Earlier notes: `findings\hits.md` §2,
`findings\combat_ai.md` §4.1, §5.5.

Status words: **implemented** (code exists), **tested** (generated
regressions), **not compared** (nothing here has been checked against the
running original game yet).

## Records

| Record | Values (FalloutNV.esm) |
| --- | --- |
| `WeapNVDynamite` `000BA0F3` | animation type 10 (grenade), skill Explosives, range 500–1024, min spread 0, 0.405 attacks/s, projectile `NVDynamiteFragProjectile`, no ammunition |
| `NVDynamiteFragProjectile` `000E3F03` | flags 0x0806 (explosion, alt. trigger, rotation), lobber, gravity 1, speed 1200, range 10000, timer 2.5 s, proximity 0, explosion `NVDynamiteExplosion`, impact force 0.5, countdown sound `0017242D`, bounciness 1 |
| `NVDynamiteExplosion` `000E397F` | force 90, damage 75, radius 750, flags 0x49 (radius in units, knock down by formula, ignore image space swap), image space radius 1500, model `Effects\ExplosionGrenadeFrag.NIF`, two sounds, impact set `00093BA4` |

Launchers and mines (`FalloutNV.esm`; `PROJ` flags 0x02 explosion, 0x04
alt. trigger, 0x08 muzzle flash, 0x20 can be disabled, 0x40 can be picked
up; type 1 missile, 2 lobber):

| Weapon | Projectile | Explosion |
| --- | --- | --- |
| `WeapMissileLauncher` (anim 9, `AmmoListMissile`) | `MissileProjectile` 0x0A, gravity 0, speed 1550; HV ammo `MissileProjectileHV` 5000; HE `MissileProjectileHE` | `MissileExplosion` damage 200, radius 1000 units, flags 0x43; HE `MissileExplosionHE` |
| `WeapNVGrenadeRifle` / `…Launcher` (anim 5) | `40mmGrenadeProjectile` 0x0A, gravity 1.5, speed 1750 (unique 3500); incendiary ammo `40mmGrenadeProjectileInc` | `40mmGrenadeExplosion` 100, 750, 0x49 (knock down by formula); `…Inc` with `EnchIncendiaryGrenadeEffect` |
| `WeapNVGrenadeMachinegun` (anim 8, automatic) | `25mmGrenadeProjectile` gravity 1.5, speed 2000; HE ammo | `25mmGrenadeExplosion` |
| `WeapFatman` (anim 9) | `FatMan` 0x02, gravity 1, speed 2500 | `FatManNukeExplosion` 600, 1700 ft |
| `WeapGrenadeFrag` / `Pulse` / `Plasma`, `WeapNVGrenadeIncendiary` / `Stun`, `WeapGrenadeGas` (anim 10) | lobbers 0x06, speed 1200 (gas 600), timer 2.5 | `GrenadePulseExplosion` with `EMP`; `GrenadeIncendiaryExplosion`, `GrenadeStunExplosion` with their enchantments |
| `WeapMineFrag`, `…Plasma`, `…Pulse`, `WeapNVMinePowderCharge` (anim 11), `WeapMineBottlecap`, `WeapNVC4PlasticExplosive`, `WeapNVTimeBomb` (anim 12) | mines 0x66: lobber, speed 400, proximity 100, timer 3, default weapon source at `DATA` 64; C4 0x462 ("detonates"); time bomb 0x806, timer 15 | `MineFragExplosion` 100, 192, 0x49 |

174 placed mines (`PGRE`): 131 frag, 33 powder charges, 7 C4 (Hoover Dam's
Oliver area), 3 bottlecap mines; some owned by a faction (`XOWN`).

Settings used (data = set by `FalloutNV.esm`, exe = built-in default):
`fGrenadeRestitution` data 0.01, `fGrenadeFriction` data 5,
`fDamageGunWeapCondBase/Mult` data 0.66/0.34, `fDamageSkillBase/Mult` data
0.5/0.5, `fBSUnitsPerFoot` exe 22, `fGrenadeAgeMax` exe 90,
`fExplosionLOSBuffer` exe 6, `fThrowingStrengthPenalty` exe 0.05,
`fGrenadeHighArcSpeedPercentage` exe 0.67, `fGrenadeThrowHitFractionThreshold`
exe 0.8, `iBallisticProjectilePathPickSegments` exe 4,
`fCombatSplashDamageMaxSpeed/MinRadius/MinDamage` exe 3000/50/20;
mines: `fMinesDelayMin` data 0.2, `fMineExteriorRadiusMult` data 1.6,
`iProjectileMineShooterCanTrigger` data 1, `iMineDisarmExperience` data 5,
`fMinesBlinkMax/Fast/Slow` exe 2/0.1/0.25, `fMineAgeMax` exe 0;
`fDangerousProjectileExplosionDamage/Radius` exe 5/30,
`iAvoidHurtingNonTargetsResponsibility` exe 50; `fKnockdownBaseHealthThreshold`
exe 75, `fKnockdownCurrentHealthThreshold` data 50, `fKnockdownAgilBase/Mult`
exe 0/1, `fKnockdownDamageBase/Mult` exe 0/0.3, `fKnockdownChance` exe 0.25.

## Traced behaviour

Addresses are PC function entries; Xbox names where the prototype has them.

- **Record layouts.** `PROJ` `DATA` at form `+0x60`: timer `+0x80`
  (`00644790`), proximity `+0x7c` (`0045cd80`), speed `+0x68`
  (`0069ef80`), bounciness `+0xb0` (`006d2c20`). Flag tests: 0x02
  explosion (`004fd360`), 0x04 alt. trigger (`00975300`), 0x400 detonates
  (`005de080`). `EXPL` `DATA` at `+0x74`: damage `+0x78` (`006a78f0`),
  radius `+0x7c`, flags `+0x88` (`00477950`), image space radius `+0x8c`.
- **Radius units** (`BGSExplosion::GetRadiusBSUnits`, `00477900`): the
  stored radius × `fBSUnitsPerFoot` unless flag 0x01 is set (also for the
  image space radius `009ac550` and radiation radius `009ad5a0`).
- **Throwing** (`00523150`): animation types 10–13 fire from the actor's
  hand node and use up the weapon (the count/unequip step at the end;
  the per-shot decrement through vfunc `+0x3f0` is inferred). Speed
  (`009669c0`, `009bca60`): the record's speed; animation type 13 ×
  (1 + `fThrowingStrengthPenalty` × (STR − Str req)); perk entry 59;
  the AI's high arc × 0.67 (`009cb830` → `00966a00`).
- **Grenade object** (`GrenadeProjectile`, vtable `0108f674`; constructor
  `009b4020`, made for type 2 by `009bd5c0`/`009bca60`). Initialize
  (`009bda10`): the fuse `+0xe4` = the record's timer unless a mine
  (`009bdf80`). Havok setup (`009be0a0`): restitution = bounciness ×
  `fGrenadeRestitution` (the setting alone if bounciness ≤ 0), friction
  `fGrenadeFriction`, when those settings are non-zero.
- **Update** (`GrenadeProjectile::UpdateProjectile`, `009b41b0`): past
  `fGrenadeAgeMax` it is removed (`009bc8f0`); then the explosion check
  (`009c3190`): alt. trigger → fuse −= frame time, explode at ≤ 0 (mines
  with no fuse wait for proximity, `009c39e0`); otherwise explode once the
  impact list is non-empty unless "detonates". Impact-triggered grenades
  explode at the impact point (flag 0x2000 set by `009b40d0`), others at
  their position.
- **Spawn** (`Explosion::SpawnExplosion`, `009ac9c0`; constructor
  `009ac200`; `Explosion::Initialize`, `009acd60`): radius = record radius
  × player perk entry 72 "Adjust Explosion Radius" (asked about the
  weapon) when the projectile's shooter is the player.
- **Targets** (`Explosion::InitHavok`, `009ae280`: a phantom sphere of the
  radius) and **damage** (`Explosion::ProcessTargets`, `009b00a0`): per
  target, share = 1 − (d/R)² inside R (`00647920`, d from the explosion to
  the target's position); damage = `Explosion::GetDamage` (`009b0f80` with
  `006479e0`) × share; actors must pass `Explosion::RunLOSPick`
  (`009b1810`); the hit is built by `009b5770` (attacker = the explosion's
  actor cause, weapon, flag 0x2000: armour `009b5a30`, critical
  `009b7060`, multiplier `009b73d0`) and handled by `0089a760`.
- **GetDamage**: (`fDamageGunWeapCondBase` + `…Mult`) × record damage;
  with an actor cause and weapon × (`fDamageSkillBase` + `fDamageSkillMult`
  × skill × 0.01).
- **Line of sight** (`Explosion::RunLOSPick` (Xbox PDB), `009b1810`):
  skipped with flag 0x10. A pick on layer 0x27 (all hits) that leaves out
  the target's own collision, from the explosion + its
  `ClosestPointNormal` (`+0xf4`) × `fExplosionLOSBufferDistance` (24)
  to the target; every hit blocks unless that normal's z < −0.98 and the
  hit is nearer than `fExplosionLOSBuffer` (6) to the start; on the first
  point an owner (`pOwner` `+0xc8`) with reference flag 0x01000000 lets
  everything through (projectiles and mines have none, `009c3190`).
  Actors with 3D get six more points ±2 × a radius (`0084d030`, `+0xc`)
  on each axis, under the normal's rule only. The normal and
  `ClosestPoint` (`+0xe8`) are set while finding targets (`009ae6e0`):
  for each of the phantom's contacts not on an actor, a ray from the
  explosion along twice the way to it (from 16 back along the normal, 32
  long, when under 1 unit) must meet that body before any other non-actor;
  the first that does gives them.
- **After the hit** (`009b00a0`): the explosion's object effect (`EITM`,
  the form's `+0x68`) is cast on the target by the actor cause's caster
  (`+0x88`), else the explosion's own (`+0x37`); then, for the living, the
  record's flag 0x04 always, or 0x08 by `CombatFormulas::CheckKnockdown`
  (`00646400`), pushes the actor down (`Explosion::PushActor`, `009b0d70`).
  `00646400`: only if the hit leaves them alive; damage over 75 % of full
  health always; else over `fKnockdownCurrentHealthThreshold` % of health
  now gives (damage × 0.3) ÷ (Agility × 10), at most 0.25, against a roll
  of 1000. The damage is the explosion's at their distance before armour
  (`HitData::InitializeExplosionData`, hit `+0x14`).
- **Limb targets** (`Explosion::FindTargets` / `CheckLimbTarget` /
  `AddLimbTarget`, Xbox): each Havok body of an actor overlapping the
  sphere adds its body part, keeping the two nearest per actor; how the
  hit then uses them isn't read. **Forces** (`009b0920`), **shudder** and
  image space: read, not implemented (physics and effects).
- **Missiles** (`MissileProjectile`): they move by their speed along their
  heading (`009bf300`) through a Havok character controller
  (`ProjectileListener`, made by `Projectile::InitHavok` `009be0a0` →
  `009c6440`) whose gravity is the projectile's (`00966980`: 1 for lobbers,
  else `PROJ` gravity; ÷ the time multiplier under the shooter's Turbo):
  so they fall (`MobileObject::Move`). Run-time flag 0x40 (gravity without
  hitscan, `009b7cc0`) only turns the model to its way (`009bf470`, its
  sole reader). A missile striking someone hits them as a bullet would
  (`Projectile::ProcessImpacts`, Xbox: `CombatHit` with the projectile's
  damage), then explodes where it struck (`009c3190`).
- **Ammunition's projectile**: `AMMO` `DAT2` form at 4, else the weapon's.
- **The AI's danger test** (`CombatManager::CheckExplosionAttack`,
  `00992720`, asked by the attack procedures `009d0a30`, `009cb380`, …):
  record damage ≥ `fDangerousProjectileExplosionDamage`, and the distance
  where its blast still does that much (radius × √(1 − 5 ÷ damage),
  `006479a0` → `00647960`) ≥ `fDangerousProjectileExplosionRadius`: then
  those the combat manager holds within it of the aim point (`00992ba0`)
  refuse the attack: the attacker, their combat group, and spectators
  (held, not targeted by the attacker's group) when the attacker's
  Responsibility ≥ 50, each unless the combat style's flag 0x20 / 0x40 /
  0x80 ("ignore damaging self / group / spectators", `009928c0`).
- **Mines**: no fuse at launch (`009bda10`); each frame
  (`Projectile::CheckExplosion` `009c3190`) an alt.-trigger projectile with
  a proximity and no fuse runs `Projectile::CheckExplosionProximity`
  (`009c39e0`): the high-process actors within the proximity (×
  `fMineExteriorRadiusMult` outdoors) it reacts to
  (`Projectile::GetMineReactsToTarget` `009c3930`: not its layer unless
  `iProjectileMineShooterCanTrigger`, not the dead, not its owner
  faction's members, its layer's reaction neutral or enemy), then the
  player, who sets it off only if a roll of 100 is under their perks'
  entry point 4 "Calculate Mine Explode Chance" (Light Step: 0); a failed
  roll spares the player from then on (flag 0x4000). The fuse: for
  characters Explosives × the record's timer ÷ 100 + `fMinesDelayMin`, for
  creatures `fMinesDelayMin`. It then counts down and blinks every
  (slow − fast) ÷ max × fuse + fast seconds (slow above `fMinesBlinkMax`),
  restarting the countdown sound. There is no arming delay in the code.
  Mines last for `fMineAgeMax` (0: for ever, `009b41b0`).
- **Disarming** (`BGSProjectile::Activate`, Xbox; `Projectile::TurnOff`
  `009c43e0`): E on an armed mine whose record "can be disabled" turns it
  off (flag 0x200; the disable sound, `DATA` form at 60); when its fuse was
  running or it reacts to the one disarming, the "Mines Disarmed"
  statistic and `iMineDisarmExperience`. E on a disarmed one that "can be
  picked up" takes it. The crosshair says "Disarm Mine" (`sDisarmMine`)
  while armed, "Take" after (`00775a00` at `007778fe`).
- **AI throws** (`CombatProcedureAttackGrenade`, `009cafe0`; aim
  `009cb6e0` → `009a8bf0` → `009a8df0` → `009a8f00`): aim mode (`009a8bf0`):
  2 for a high arc of a falling projectile, 4 (feet, `009a8460`) for
  splash damage, else 1 (half height); pitch (`009a7c60`): the low/high
  root of the ballistic equation, 45° when out of reach; aim point
  z = tan(pitch) × x + origin z. Arc choice (`009a7670`): low first, then
  high; a path test (`009a6e90`, `iBallisticProjectilePathPickSegments`)
  must not be blocked before `fGrenadeThrowHitFractionThreshold`.
- **Band** (`009a9180`): an exploding projectile keeps the minimum range
  at least the explosion radius.

## Implemented

Core (`crates/world`):

- `world::explosions` (new): `ProjectileRecord`, `ExplosionRecord`,
  `falloff`, `base_damage`/`damage_with`, `blast_radius`, `launch_speed`,
  `is_thrown`, `FlightSettings`, `Flight` (launch, step, fuse, expiry),
  `los_clear`, `blast_targets`, `aim_mode`, `aim_height`, `launch_pitch`,
  `aim_point`, `arc_clear`. Translated functions carry their address.
- `Runner::explosion_hit` (`world::scripting`): `OnHit`/`OnHitWith`,
  armour, critical, health, `OnDeath`, crime and fighting back.
- `combat_ai::ranged_band_blast`; perk entries 4, 59 and 72.
- `projectiles::missile_gravity`, `Missile::stretch` (the fall);
  `combat::fired_projectile`; `explosions::cast_enchantment`,
  `knocks_down`, `knockdown_by_formula`, `explosion_attack_allowed`,
  `reach_of_damage`; `Flight::set_off`, mines' age.
- `mines` (new): `MineSettings`, `reacts_to`, `check_proximity`,
  `fuse_for`, `blink_interval`, `activation`, `disarm`, `placed_in`,
  `take`, `use_placed`, saved `minedisarmed` / `minegone` lines;
  `activation::info` "Disarm Mine".

Viewer:

- `explosives` (new): throw queue, launch (AI arc choice against the
  cell collision, using up the weapon), flight through the collision,
  explosion sounds, hits on everyone alive in range and sight, hit reports.
- `combat`: the player's attack throws weapons of types 10–13.
- `fighting`: people holding one keep out of its blast and throw it;
  people don't fire or throw explosives whose blast would catch their own
  side (`blast_is_safe`); their shots use the ammunition's projectile.
- `bolts`: rockets, 40 mm and 25 mm grenades and the Fat Man's nuke fly as
  bolts and fall under their gravity; explode where they strike.
- `explosives`: the explosion's object effect and knockdown (logged; the
  fall isn't played); placed mines of the player's place and laid mines
  go off for whoever comes near, count down and explode; `scripts`: E
  disarms a placed mine (sound, XP), then takes it.

## Tested (generated inputs, no game files)

`world` integration tests `crates/world/tests/launchers_mines.rs` (7, on
`testdata::launchers` with the game's values: ammunition projectiles,
gravity, EMP cast, knockdowns, the danger test, mine proximity, owners,
Light Step, the layer, disarming, taking, saving, a laid mine's fuse);
`projectiles::tests` (a 40 mm grenade aimed as the AI aims comes down on
its target).

`world` unit tests `explosions::tests` (15: record parsing, radius units,
falloff, damage, restitution, fuse/landing/rest, impact grenades, expiry,
line of sight incl. the floor buffer and offset points, targets, launch
pitch, aim point, aim mode, arc test); `combat_ai::tests::
dynamite_keeps_its_thrower_out_of_the_blast`; integration test
`scripting::explosions_hurt_through_the_hit_path`; viewer
`explosives::tests` (normals, queue).

## Test scenes (live, release viewer, 2026-10-07)

Mines (the mined corridor of `NorthVegasHouseTools`, eleven frag mines):

```powershell
nv-viewer.exe "<Data>" NorthVegasHouseTools --at 3152,3580,7616,180,55 `
  --walk --freeze-ai --run "player.modav health 1000" `
  --run "player.addperk LightStep" --key-at 3 e --key-at 4 e `
  --run-at 5 "player.removeperk LightStep" --run-at 5.5 "player.MoveTo 000EBA66" `
  --screenshot shot.png --wait 9
```

Seen: "E) Disarm Mine / Frag Mine" under the crosshair (red: owned), no
mine set off while Light Step is held; E: "Disarmed 000EBAAB", "XP +5",
the prompt turns to "E) Take"; E: "Frag Mine added"; without the perk,
standing among the rest, three are set off (0.65 s fuse: Explosives 15 ×
3 ÷ 100 + 0.2) and hurt the player by the falloff (96.9 at 34 units, 99.9
at 5, 86.9 at 69); the one 97 units away that had spared the player under
Light Step stays quiet (flag 0x4000).

A frag mine the player lays at their feet in Doc Mitchell's house
(`--weapon WeapMineFrag`, looking down) stays quiet: the data lets a
layer set off their own mine (`iProjectileMineShooterCanTrigger` 1), but
`PlayerFaction` lists itself as an ally (its own `XNAM`, reaction 2), so
the player's mines react to neither the player nor their allies; Doc's
factions are no enemy either.

Launchers (Goodsprings, `WastelandNV --at -68250,5800,8480,180,0 --walk
--freeze-ai --weapon WeapNVGrenadeRifle|WeapMissileLauncher --key-at 5
mouse-left --wait 9`): the 40 mm grenade fired level struck the ground
1032 units out, 164 below the eye (the controller's fall over 0.58 s);
the rocket flew straight 1015 units into the house; damage 57.5 (100 ×
1.0 × (0.5 + 0.5 × Explosives 15 ÷ 100)) and 115.

## Not compared / gaps (labelled in code)

- Nothing compared with the original game yet (fuse timing, throw arcs,
  damage at distance, who is shielded).
- The grenade is a point with a simple contact response, not Havok's rigid
  body (shape, spin, the surface's friction, the model's own restitution
  when a setting is 0). It collides with the world only, not with actors.
- The target point is its position (feet); bodies use the people's
  controller radius for the offsets.
- Throw origin (eye for the player, 60 units up for people), immediate
  release (no throw animation timing), target height 128 for the aim.
- Not done: models (projectile, explosion), light, image space, decals,
  camera shake, the mines' blinking light, forces on bodies (forces on
  clutter: [PHYSICS.md](PHYSICS.md), `009b0920`), the knockdown's fall
  (`009b0d70`: decided and logged only), limb damage from blasts (the
  limb targets are read, their use isn't), damage to destructible objects
  (mines shot or caught in a blast don't go off: `DEST`), the planner
  switching NPCs to grenades or mines (only the weapon in hand is used),
  homing missiles (`ProjectileTarget`, `AimAtPoint` `009c6030`), the
  character controller's own limits on a missile's fall (taken as free
  fall), Turbo's share of the gravity, `DisableAllMines`, E on mines the
  player laid (they aren't references here), and V.A.T.S. grenade aiming.
- The AI's danger test counts as the game does (`00992ba0`,
  `combat_groups::explosion_nearby`): only those the combat manager holds
  (group members and group targets, its map at +0x10: `00992480`,
  `009867d0` → `009929e0`; `00986410` → `00992a50`; `00986560` →
  `00992b00`), alive, within the reach; the attacker, its group (the same
  vtable +0x3f8 group), and spectators (not targeted by the attacker's
  group); the group's targets aren't counted. What's left: positions are
  where each stands now (the game asks vtable +0x4d8 for the place after
  the shot's flight time, `009a7e00`); the manager's map keeps an actor
  once registered (removal not traced), here only those fighting now and
  current group targets; groups follow `world::combat_groups`' model.
- Disarming a placed mine credits the player as `009c43e0` does: its fuse
  running (the projectile's `+0xe4` below `FLT_MAX`; the viewer marks
  `MineStates::fuse_running` when one is set off) or it reacting to them.
  The running fuse isn't saved.
- Line of sight: the closest surface is the collider's nearest triangle
  first (Havok's contact order isn't traced), the normal points from its
  closest point to the centre (up when the centre lies on it: unresolved);
  a placed mine's own collision is left out of the casts (the exploding
  projectile's removal isn't traced); the pick's layer 0x27 filter isn't
  applied (every collider triangle counts).
- Explosion hits sound as the game's do: the hit carries the explosion's
  weapon source (`009b5770` sets hit `+0x30` from `pWeaponSource`
  `+0xe0`, `009b0900`) and the hit sounds come from that weapon's impact
  set (`0088e1e0`, as for any hit). A placed mine's explosion is
  reported with no weapon (default set); whether the game gives it the
  projectile's weapon source isn't traced.

Next action: in the original game, fire the grenade rifle level from the
test scene's spot (where it lands), disarm a `NorthVegasHouseTools` mine
(XP, prompt) and walk onto the next (fuse), comparing with the scenes
above.
