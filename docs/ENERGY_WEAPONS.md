# Energy weapons: lasers, plasma and their critical effects

M4 weapon class (the former task list), branch `claude/energy-weapons`,
2026-10-07. Sources: FalloutNV.exe 1.4.0.525 (the unpacked image; the
private Ghidra export, functions Ghidra didn't split disassembled with
capstone), class names from the Xbox 360 prototype's symbols (Xbox PDB),
records read from `FalloutNV.esm`.

Status words: **implemented** (code exists), **tested** (generated
regressions), **live** (seen in the release viewer on the real data; the test scene
below),
**not compared** (nothing here has been checked against the running
original game yet).

## What the data relies on

Energy weapons are weapons whose skill (`DNAM` i32 at 104) is Energy
Weapons (34). Against ballistic guns, the records differ in:

| Field | Energy weapons | Ballistic guns |
| --- | --- | --- |
| Projectile (`DNAM` 36) | lasers: `BeamLaserProjectile` (type 4 beam, flags 0x8C, speed and range 10000); plasma: `PlasmaProjectile`/`02` (type 1 missile **without** the hitscan flag, flags 0x20C, speed 7500, range 10000), `PlasmaCasterProjectile` (20000, explodes) | bullets: missiles with the hitscan flag 0x01 |
| Resist type (`DNAM` i32 120) | 60 Energy Resistance (lasers, plasma), 61 EMP (pulse gun, Tesla cannon), 52 Fire (flamers) | -1 (all 202 others) |
| Critical effect (`CRDT` effect at 12, flag 0x01 "on death" at 8) | `LaserDisintegrationFXSpell` (every laser), `PlasmaEffect` "Gooification" (plasma), `AlienDisintegrationFXSpell` (alien blaster, Tesla cannon), all "on death" | none (the Fists record too) |
| Ammunition | `AmmoList…EnergyCell` / `MicroFusionCell` / `ElectronChargePack` lists: normal (DT − 2), Over Charge (damage × 1.25, DT − 5, wear × 1.5), Max Charge (× 1.5, DT − 10, wear × 2.5), Bulk (× 0.85, wear × 0.85); drained cells come back (`DAT2` item at 12, 10–40%) | rounds, cases |
| Ammo use (`DNAM` u8 14) | plasma pistol and rifle 2, plasma defender 2, laser pistol (GRA unique) 5, pulse gun 5, tri-beam 3 | 1 |
| Automatic (`DNAM` flags 0x02) | Gatling laser (fire rate 30), laser RCW (9) | SMGs, miniguns |
| Recharging | Recharger pistol: regen rate (`DNAM` 176) 1, mod slot 1 effect 5 (regenerate ammo, shots) value 3, `AmmoMicroBreeder` | – |

The critical effects are spells (`SPIT` type 0, 4 s) with one script
effect each (`MGEF` archetype 1, flags 0x10000475 / 0x10000075: flag
0x10000000 "no death dispel"). Their scripts (`LaserDisintegrationEffectScript`,
`PlasmaGooificationEffectScript`, `AlienDisintegrationEffectScript`) set
the actor's critical stage, play effect shaders (`PMS`), call
`AttachAshPile` (the goo with `AttachAshPile 2`) and end with the
"end" stage: the laser one after 1.8 s with the ash pile at 0.5 s left,
the goo after 1.4 s. The laser and goo ones skip Mr. House (`GetIsID
NVCRMrHouse`).

## Traced behaviour

- **Critical chance** (`009b7060` → `00646d80`): the Critical Chance actor
  value (14) ÷ the weapon's fire rate (`DNAM` 64, `00821640`) when it's
  automatic (flags 0x02, `00524b40`; 1 when the rate is 0), × the `CRDT`
  multiplier when ≥ 0 (`008d1eb0`). So the Gatling laser's shots each
  have 1/30 of a semi-automatic's chance.
- **Critical hit** (`009b7060`): hit flag 4; the critical damage
  (`CRDT` u16, `00645c20`) through perk entry 2; hit flag 8 = the weapon's
  "on death" flag (`+0x1c8`, `009b73b0`), hit `+0x50` = its effect
  (`+0x1cc`, `0051f4d0`).
- **Critical effect** (`0089a760`, `0089b28f`–`0089b43a`): after the
  damage (`DamageHealthAndFatigue`, vtable `+0x338` = `0089d6f0`, which
  returns `IsDead`), if the target was alive before the hit (`IsDead`,
  `+0x22c`) and the hit is critical with an effect: skipped when (the
  target is essential `0087f3d0` or not dead) and the hit has the "on
  death" flag; skipped when the target's `GetSitSleepState` (`+0x214`,
  what `GetSitting` maps) isn't 0 and `BannedEffectsOnSitters` (`FLST`
  001768D7: goo, laser and alien disintegration) holds the effect;
  skipped with `bDisableAllGore` (`011df7f8`); else the target's own
  caster (`+0x88`) casts it on itself (`00815b00`, vtable `+0x40`,
  `00815d00`, `00824110` on `+0x94`).
- **Effects on the dead** (`ActiveEffect` update, `00804560`): at the end
  of each update, a target actor that `IsDead` ends the effect unless its
  magic effect has flag 0x10000000; an ended effect that had started runs
  its finish (vtable `+0x58`; `ScriptEffectFinish`).
- **Critical stages** (`SetCriticalStage`, `005dc010` → `008a1a40` actor
  `+0x10c` → `008a1a70`; names at `0119bbb0`: None, GooStart, GooEnd,
  DisintegrateStart, DisintegrateEnd = 0–4): stage 1 sets a shader on
  the 3D; stages 2 and 4 cull the body's 3D (`00450f90(1)`), run
  `0057b520(0)` and vtable `+0x1c0`, and when the victim isn't the player
  and their killer (actor `+0xc0`, written by `Actor::Kill` `0089d900`)
  is, bump miscellaneous statistic 0x1d "Disintegrations" (`004d5c60`).
  A stage on someone alive only logs a warning.
- **Ash piles** (`AttachAshPile`, `005db870`, not split by Ghidra): an
  optional integer; 2 → `DefaultAshPile2` ("Goo Pile", `ACTI` 0x22,
  `Effects\GooPile01.NIF`), else `DefaultAshPile1` ("Ash Pile", 0x1B,
  `Effects\AshPile01.NIF`) (globals `011ca280` / `011ca27c`, looked up at
  start by `0046a370`). A ray from 32 above the actor straight down to
  256 below finds the ground; the pile is made there (`004698a0`), tilted
  to the ground, else at the actor's position, turned to its heading;
  `ExtraAshPileRef` goes on both (`0041e340`). **Activating** the pile
  activates the corpse instead (`TESObjectREFR::Activate`, `00573170`),
  so its inventory stays on the body and is searched through the pile.
- **Armour** (`009b5a30`): energy cells' threshold bypass is an ordinary
  ammunition effect (kind 2, before the perks); the weapon's **resist
  type** is the last step, after the 20% floor: r = min(the target's
  actor value, 100) ÷ 100, and when 0 < r ≤ 1 the damage × (1 − r) (so
  it can go below the floor). The Split Beam mod (effect 12, weapon mods
  aren't on this branch) divides the threshold by the projectile count.
- **Projectiles** (`Projectile::Initialize` `009bda10`): its run-time
  flag 0x1 makes the cast along the whole range as it's made
  (`009bec90` → vtable `+0x328`). `BeamProjectile::Initialize`
  (`00979280`) sets it always: **beams strike at once** (the beam then
  lingers for its fade, removed after 5 s, `00979330`).
  `MissileProjectile::Initialize` (`009b7cc0`) sets it from the record's
  hitscan flag (`009a7f80`), but never while V.A.T.S. plays its queue
  (the manager's mode `[011f2250]+0x08` = 4): bullets strike at once
  (and fly in V.A.T.S.), **plasma flies**:
  `MissileProjectile::UpdateProjectile` (`009b8030`) moves it by its speed
  × the frame's seconds (`009bf300`, `009669c0`) along its heading, checks
  its impacts (`009c3190`), and removes it past `fArrowAgeMax` (90 s,
  `011ce684`) or past its range (`+0xd4`, the record's range, `009a7c40`;
  distance flown `+0x110`) with nothing struck.
- Generic and already done elsewhere (confirmed, not duplicated): the
  Energy Weapons skill in the damage (`fDamageSkillBase` + `…Mult` × skill,
  `world::combat::weapon_damage`), ammo use per shot, spread (min spread,
  `world::combat::Weapon::shot`), drained cells and Vigilant Recycler
  (`ammo_item_recovered`), Laser Commander's critical chance (perk entry
  1), Fast Shot in V.A.T.S.

## Implemented

Core (`crates/world`):

- `combat::Weapon` `crit_effect`, `crit_on_death`, `resist`;
  `combat::critical_divisor` (in `critical`), `combat::resisted_share`
  (end of `hit_through_armour`), `combat::critical_effect` and
  `sit_sleep_state` (called by the hit and explosion paths in
  `scripting::Runner`).
- `magic::survives_death`, `magic::tick` dispels the dead's effects
  (running `ScriptEffectFinish`) unless they survive death.
- `more_functions::set_critical_stage` (stages by name, `body_gone`, the
  Disintegrations statistic), `placed::attach_ash_pile`
  (`AttachAshPile`, saved as `ashpile` lines), `activation::stands_for`.
- `projectiles` (new): `delivery` (at once / flies / lobbed), `Missile`
  (step, range, `fArrowAgeMax`).

Viewer:

- `bolts` (new): the player's and people's plasma shots fly as bolts
  from the eye or the shooter's fire height, at their launch speed,
  meeting bodies, scripted objects, walls and (for others' bolts) the
  player's bounds each frame; hits go through the same hit path and hit
  reports; a bolt whose projectile explodes without an alt. trigger (the
  plasma caster) also goes off where it struck (`explosives`, the
  explosion path of `docs/EXPLOSIVES.md`). Lasers and bullets stay
  instant. Corpses whose critical stage culls the body are hidden.

## Tested

`world` unit tests `projectiles::tests` (2) and integration tests
`crates/world/tests/energy_weapons.rs` (8) on `testdata::energy` (the
laser pistol, plasma pistol, laser RCW, their projectiles, energy cells
and critical spells with `FalloutNV.esm`'s values; test scripts calling
the same functions): records, threshold bypass and resistance order,
automatic critical chance, the laser disintegration (stage 3, the ash
pile at 1.3–1.5 s, stage 4, statistic), the plasma goo pile, "on death",
sitters and the already dead, death dispelling, saving.

## Test scene (live)

The release viewer in Doc Mitchell's house, the player 144 units south of
him, Luck 100 (every hit critical), people frozen, VCG01 stopped (Doc's
`GSDocMitchellScript` `OnHit Player` block resets his health while it
runs, as in the game), eight shots:

```powershell
viewer\target\release\nv-viewer.exe "<Data>" GSDocMitchellHouse `
  --at 2288,2100,7360,0 --walk --freeze-ai --weapon WeapPlasmaPistol `
  --run "StopQuest VCG01" --run "player.setav luck 100" `
  --run "player.setav energyweapons 100" `
  --key-at 3 mouse-left --key-at 3.6 mouse-left --key-at 4.2 mouse-left `
  --key-at 4.8 mouse-left --key-at 5.4 mouse-left --key-at 6 mouse-left `
  --screenshot shot.png --wait 12
```

Seen on 2026-10-07 (release build, `FalloutNV.esm`):

- Plasma pistol: "00000014's bolt: Hit 00104C0F at 134 units for 66.0 (4.0
  left), a critical" (33 + 33 critical damage, Energy Weapons 100), the
  second kills; "DocMitchellREF shows GooShader01 / 02" (stage 1); 1.4 s
  later "a DefaultAshPile2 is placed at 2289,2244,7360", the body is
  hidden and the green goo pile is drawn at his feet.
- Laser pistol (`--weapon WeapLaserPistol`): 24 a hit (12 + 12), the
  third kills; `LaserCritGlowFXShader` and `effectLaserDisintegration`
  shown, the ash pile placed 1.3 s later, the shaders stopped and the
  body gone at 1.8 s; the grey ash pile lies where he stood.

## Not done / gaps

- Nothing compared with the original game.
- After the body is culled nothing meets it, as in the game: stages 2
  and 4 run `0057b520(0)`, which takes the reference's collision out of
  the Havok world (with 1 it adds it back, `00620130`), before culling
  the 3D, and shots, swings and the crosshair's pick all go through
  Havok. Here the crosshair's ragdoll shapes, the melee search, the shots'
  people (dead are never shot here) and scripted references' bounds all
  skip such a body (`more_functions::body_gone`).
- Visuals (the effects batch, B5/B6): the beam and bolt models, muzzle
  flash, the glow and disintegration shaders (`PMS`). The piles are
  drawn (as references scripts make), but the viewer's crosshair doesn't
  pick made references other than dropped items, so E on the pile isn't
  wired (`activation::stands_for` is ready for it).
- The pile's ground ray and tilt (no collision in the world crate: it
  goes at the body's feet); vtable `+0x1c0` at stages 2 and 4;
  `bDisableAllGore` (INI) isn't read.
- Missiles: gravity (run-time flag 0x40) and the Turbo slow-down
  (`009bf370`), the projectile's own collision shape, the player's
  scoped-range multiplier (`009bda10`), people leading or dodging bolts,
  bullets flying during V.A.T.S. playback (`world::projectiles::delivery`
  knows; `vats` still casts at once).
  Flames (the flamer, incinerator) and continuous beams keep the
  instant cast; their classes' `Initialize` isn't read. (The Gatling
  laser and RCW fire ordinary beams, `BeamLaserAutoProjectile`.)
- Recharging weapons (regen rate, mod effect 5/6): not traced.
- Projectiles striking at once that explode on impact (the Gauss rifle's
  `GaussImpactExplosion`) don't explode yet; flying ones (the plasma
  caster's `ExplosionPlasmaCasterEffect`) do (`bolts` → `explosives`).
- The EMP (pulse gun, `EMPPulseGunBeam` enchantment) and Tesla effects
  (`EITM`): weapons' own enchantments aren't applied (`nvinspect`'s
  coverage table: `ENCH`).

Next action: run the test scene in the original game with the same
console lines and compare the shot count to a disintegration, the
pile's time and place, and the plasma bolt's flight time.
