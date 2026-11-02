# Melee and unarmed: reach, what a swing hits, power attacks, specials

M4 weapon class (the former task list: "unarmed and melee specials (power
attacks, V.A.T.S. specials)"), branch `claude/melee-unarmed`, 2026-10-07.
Sources: FalloutNV.exe 1.4.0.525 (the private Ghidra export; settings'
readers found by their addresses in the disassembly), names from the
Xbox 360 prototype's symbols (Xbox PDB: `Actor::MeleeAttack`,
`Actor::FindMeleeTarget`, `Actor::StartAttack`, `HitData` and its
members), records read from `FalloutNV.esm`. Code: `world::melee`, the
melee parts of `world::combat`, `world::vats`, `world::body_parts`,
`world::scripting::Runner::blow_at`; the player's swing in
`viewer/src/combat.rs`, V.A.T.S.'s melee hits in `viewer/src/vats.rs`.

Status words: **implemented** (code exists), **tested** (generated
regressions on the records' values, `crates/world/tests/melee.rs` on
`testdata::melee`, unit tests in `world::melee`), **live** (seen in the
release viewer on the real data; the test scene below), **not compared**
(nothing here has been checked against the running original game).

## Inventory: what was there, what was wrong

Before this batch the codebase already had, from the maintainer: blocking
(the Aim control, the block skill added to the threshold, `009b5a30`,
`006463a0`), power attacks (the hold past `fPowerAttackDelay`, the
direction groups and the unarmed perks' custom power attacks, the
counter, `00948310`), the weapon damage formula with the skill, the arms,
Melee Damage / Unarmed Damage (`00644ce0`), criticals (`009b7060`),
V.A.T.S.'s melee costs, the unarmed specials offered (`007eb920`), their
damage multipliers (`009b5170`) and their aim, the NPCs' melee AI. Checked
against the exe item by item; these were wrong or missing:

| Item | Before | The game (address) | Now |
| --- | --- | --- | --- |
| Reach of hand-to-hand weapons | 64 (as bare hands) | the weapon's reach × `fCombatDistance` for every melee weapon, animation types 0–2 (`009a69c0`, `008990f0`): brass knuckles 128, mantis gauntlet 153.6 | fixed (`Weapon::melee_reach`, `melee::swing_reach`) |
| What the player's swing hits | the first body along the view's ray within reach | `Actor::FindMeleeTarget` (`009a60e0`): an attacker with a combat target only that one; else everyone within reach (the gap between the bodies, `009a64d0`/`009a6770`, flat in its two cases below) and in the hit cone (`009a6ae0`), the one nearest the cone's middle; then a line of sight (`0088b880`) | implemented (`melee::find_target`, viewer `melee_met`) |
| The hit cone | 35° (× 3 within 128) | the player's by the attack (`009a6a40`): `AttackPower`/`AttackForwardPower` 40 (`fCombatOverheadHitConeAngle`), back/left/right power 100 (`…Sweep…`), `AttackCustom1Power` 50 (`…Uppercut…`), else 35; a dead target × `fCombatDeadActorHitConeMult` 2 | implemented (`melee::Cones`, `cone_check`) |
| Fists' power attack | × `fDamagePowerAttackBonus` inside the damage, before the armour | fists' damage is asked with 1 (`00646310`); the bonus goes to the hit's `fBonusMult` (`HitData` `+0x5c`, `009b7840`: the larger of it and the body part's) and multiplies after the armour (`009b73d0`); weapons (hand-to-hand ones too) keep it inside (`00644ce0`) | fixed (`combat::fists_power_bonus`, `Runner::blow_at`) |
| Unarmed specials outside V.A.T.S. | none | `Actor::StartAttack` (`00893a40`): an unarmed attack (fists or a hand-to-hand weapon not looping/spinning), by a person or the player, not sneaking, outside V.A.T.S.: Unarmed (whole points) above `fUpperCutThreshold` 50 gives Unarmed × `fUpperCutSkillChance` 0.15 % for `Attack6` (the uppercut); above `fCrossThreshold` 75 a further Unarmed × `fCrossSkillChance` 0.15 % for `Attack7` (the cross); roll U(0, 100) | implemented for the player (`melee::unarmed_special_group`); NPCs: not hooked (their AI and animation are the maintainer's) |
| What the uppercut and cross do | nothing | `Actor::MeleeAttack` (`00899200`) passes the hit 1 for `Attack6`, `AttackCustom4Power`, `AttackCustom5Power` (unarmed), 2 for `Attack7`; the hit (`0089a760`): 2 = limb damage × `fCrossSkillDamageMultiplier` 2.5; 1 = the target staggers (actor `+0x1b1`), and a staggered person (not the player, not essential) holding a weapon drops it when the right arm, or the left with a two-handed weapon, was hit | implemented (`melee::special_of`, `body_parts::hurt_part`) |
| Stagger and disarm on crippling | a newly crippled arm drops the weapon | the same "stagger" flag: newly crippled part, or an already crippled torso hit again with `iCombatCrippledTorsoHitStaggerChance` 50 (roll % 100 ≤ it), none for someone ignoring crippled limbs (AV 72), always for the uppercut; essential people keep their weapon | fixed (`PartHurt::staggered`) |
| Knockdowns on hits | none | perk entry point 52 "Knockdown Chance" (attacker, weapon) from 0; knocked down when > 0 and U(0, 1) ≤ it (`0089a760`); Super Slam: 0.15 for weapon anim types below 3, 0.30 for two-handed melee | implemented as the decision (`melee::knockdown_chance`, `Hit::knocked_down`); the knockdown itself (process `+0x418` with `00646580`'s force) is the animation's/physics' |
| V.A.T.S. special's label | the weapon's name (inferred) | the weapon's `VANM` (`+0x368`, `00522be0`): "Mauler", "Back Slash", "Grand Slam", "Lights Out", "Fore!", "Long Cut", "Scrap Heap" | fixed (`vats::attack_name`) |
| V.A.T.S. specials' skill test | Uppercut above 50, Cross above 75 | at least the threshold, on whole skill points (`007eb920`, the actor value owner's `+8`) | fixed |
| V.A.T.S. special's spell | none | the hit of a queued weapon special or Uppercut with a weapon casts the weapon's `VATS` spell (`+0x370`, `0051f510`) on the one struck (`0089a760`): `MaulerKnockdownSpell` (super sledge, fire axe, Blade of the East…: `player.pushactoraway myself 15` unless Stonewall), `VictoryRifleKnockdownSpell` (shovel, 9 Iron, Nephi's driver) | implemented (`vats::special_effect`) |
| V.A.T.S. moves' groups | – | `00948310`: a weapon special plays the forward power attack (the weapon's own when it has one, else `AttackPower`), Uppercut `Attack6` (not sneaking), Cross `Attack7`, Stomp `stomp` (0xa9) | implemented as data (`vats::attack_group`), used for the hit's special |

Checked and found right: blocking (who, the skill used, fists against
guns and projectiles, the counter timer), the power attack groups and the
custom perks' (entry points 61–66), the counter attack, the weapon damage
formula for weapons, critical chance and damage for melee and fists
(`fCombatUnarmedCritDamageMult`, data 1), the sneak attack multipliers
(`fCombatDamageBonusMeleeSneakingMult` 5), V.A.T.S.'s specials' costs and
damage multipliers (`fUpperCutVatsMultiplier` 1.15, `fCrossVatsMultiplier`
1.1, `fGroundAttackVatsMultiplier` 2, the weapon's `VATS` multiplier,
`fVATSAutomaticMeleeDamageMult` 2), the V.A.T.S. reach × 2.

Power attacks cost nothing else: `fActionPointsPowerAttackMult`,
`fPowerAttackFatiguePenalty`, `fAIPowerAttackFatigue…` and the direction
bonuses `fDamagePowerAttackStand/Side/Forward/BackBonus` are only
initialised, never read (no reference outside their constructors), and
there is no cooldown besides the hold (`fPowerAttackDelay` 0.3 s) and the
attack animation.

## What the data has

- Melee weapons (`DNAM` animation 1 one-handed, 2 two-handed; skill 38)
  and hand-to-hand ones (animation 0, skill 45): reach (`DNAM` f32 at 8)
  0.5 (knives, machetes) to 1.3 (Blade of the East); hand-to-hand 1 (the
  mantis gauntlet 1.2); `Fists` (`000001F4`) 0.
- `VATS` (20 bytes: spell, skill, damage mult, cost, silent, mod
  required) and `VANM` on 45 weapons; every one with a cost has a name.
  Bladed weapons "Back Slash" (× 0.7), blunt "Lights Out" (× 1.25),
  two-handed "Mauler" (× 0.5, knockdown spell), "Grand Slam" (× 2 or 1),
  automatic "Long Cut"/"Scrap Heap" (× 0.5).
- Perks: Super Slam (entry 52, conditions on the holder: not an automatic
  melee weapon, not a second list, `GetWeaponAnimType` < 3 → 0.15, = 3 →
  0.30); the unarmed moves Legion Assault (61), Ranger Takedown (62),
  Scribe Counter (64), Khan Trick (65, 66), Paralyzing Palm (29).
- Settings `FalloutNV.esm` changes: `fDamagePowerAttackBonus` 2 (exe 3),
  `fBlockSkillBase` 5, `fBlockSkillMult` 0.3, `fAVDUnarmedDamageBase` 0.5
  and `…Mult` 0.05, `fCombatUnarmedCritDamageMult` 1, `fVATSMelee…`/
  `fVATSH2HWarpDistanceMult` 0.32/0.27. The cones, specials' chances and
  thresholds, `fCrossSkillDamageMultiplier` 2.5,
  `iCombatCrippledTorsoHitStaggerChance` 50, `fHandReachMult` 0.5 and
  `fCombatDistance` 128 are the exe's.

## Not done (with what would be needed)

- **Fatigue damage**: done in `claude/fatigue-blockers`
  ([FATIGUE.md](FATIGUE.md)): fists' half, the bean bag's, regeneration,
  knock-outs.
- **The stagger and knockdown themselves**: the hit reaction animations
  (the reaction flags `0089a760` sets after the limb damage, its "staggered down" message), the process's
  knockdown (vtable `+0x418`, force `00646580`): animation and physics,
  the maintainer's. The decisions are in `PartHurt::staggered` and
  `Hit::knocked_down`, and the viewer prints them.
- **NPCs**: their attacks don't roll the unarmed specials (`00893a40`
  runs for them too) and their hits have no body part, so the cross and
  uppercut wouldn't change them; their target is the combat target
  already (their AI, the maintainer's).
- **A weapon special as a power attack**: V.A.T.S. plays the forward
  power attack for it; whether the hit then also gets
  `fDamagePowerAttackBonus` depends on the animation event handlers'
  power flag (`00895110`, `008ba600` → `00899200`), not resolved: the
  viewer passes no power for V.A.T.S. hits, as before.
- **The player's object hits** (`008ae660`, `fObjectHitH2HReach`) and
  the auto-aim turn added to the player's cone (`00965620` on the
  `ProjectileNode`): not traced; scripted objects are met along the view
  as before.
- **The gap's vertical case** is translated (`melee::Swing::gap`,
  `009a64d0`): flat when the swinger's bounds' top or bottom lies within
  the other's bounds (`OBND` z, vtable `+0x1d8`/`+0x1dc`) and either both
  swim (`005a2030`, actor `+0x14d`) or it's the combat target and their
  heights differ by at least `fAICombatSlopeDifference` (48, `00f63ab0`);
  3D otherwise. The viewer doesn't model swimming yet, so only the
  combat-target case can occur; a base without `OBND` is given zero
  bounds (not traced).
- **Unarmed specials' animations** (`Attack6`, `Attack7`, the perks'
  custom power attacks, `stomp` with `MeleeStomps`): the first-person
  view plays the group's file when the weapon kind has it.

## Test scene (live)

The release viewer in Doc Mitchell's house, people frozen, VCG01
stopped, Unarmed 100 and Super Slam:

```powershell
viewer\target\release\nv-viewer.exe "<Data>" GSDocMitchellHouse `
  --at 2288,2100,7360,0 --walk --freeze-ai --weapon WeapBrassKnuckles `
  --run "StopQuest VCG01" --run "player.setav unarmed 100" `
  --run "player.addperk SuperSlam" --run "player.setav luck 0" `
  --key-at 3 mouse-left --key-at 4 mouse-left --key-at 5 mouse-left `
  --key-at 6 mouse-left --key-at 7 mouse-left --key-at 8 mouse-left `
  --screenshot shot.png --wait 10
```

The player stands 144 units south of Doc (a gap of about 104 between
the bodies): the brass knuckles' 128 reach meets him, bare hands' 64
wouldn't.

Seen on 2026-10-07 (release build, `FalloutNV.esm`):

- Brass knuckles, 144 units south: "Hit 00104C0F in the head at 164
  units for 23.5" (18 + Unarmed Damage 5.5), the first one crippling the
  head ("Head crippled; staggered"); "Unarmed special: attack7" and
  "attack6" between the hits (Unarmed 100: 15% each), the uppercut's hit
  "staggered"; the dead Doc is still hit (the player may strike the
  dead). No Super Slam knockdown came up in those five hits (15% each).
- Bare hands from the same spot (no `--weapon`): "The attack hit
  nothing" every time (64 reach).
- Bare hands 54 units from him (`--at 2288,2190,7360,0`): 6.5 a hit
  (1 + 5.5), a cross ("attack7") followed by the head crippled.
- The scripted routes (doc, vcg02, vms16) pass with these changes (see
  the report).

Next action: run the same scene in the original game (brass knuckles at
that distance; Unarmed 100's uppercut/cross rate; a Mauler special on a
person in V.A.T.S.) and compare.
