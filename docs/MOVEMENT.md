# Player movement and the character controller

Topic file for the player's walking, running, sneaking and jumping, and
the character controller's ground and air states. Read from
`FalloutNV.exe` 1.4.0.525 (unpacked image) with Ghidra; names marked
(Xbox PDB) come from the Xbox 360 prototype symbols (ADR-0002). Earlier
controller facts (shape, step, slope, gravity, fall damage) are in
`ENGINE_REFERENCE.md` and the private
`findings/physics.md`; they are not repeated here.

The game's own translated movement path (`crates/engine`, Build 43) is wired
in beside this one: ENGINE_CRATE.md, "The player's movement"; the viewer's
`--movement old|engine|compare` chooses.

Status words: **implemented** (code exists), **tested** (regression test),
**compared** (checked against the running original game). Nothing in this
batch has been compared.

## Speed (`world::locomotion`)

| Address | What | Status |
| --- | --- | --- |
| `00885a50` | `Actor::GetWalkSpeed` (Xbox PDB): gathers the inputs and calls `00647d10`; × a factor from actor vfunc +0x428 when it returns an object (the player's `00acbb70` returns none) | implemented, tested |
| `00885bf0` | `Actor::GetRunSpeed` (Xbox PDB): `00647f00`, the same vfunc +0x428 factor, then perk entry point 42 (Modify Run Speed) | implemented, tested (perk entry through the existing perk code) |
| `00647d10` | The formula, translated in `locomotion::walk_speed` | implemented, tested |
| `00647f00` | Running = walking × `fMoveRunMult` | implemented, tested |
| `00514410` | Armour class from the biped model's general flags (`ARMO` +0x70 + 8 = `BMDT` byte 4): 0x80 heavy (`004c0bd0`), else 0x08 medium (`00514450`) | implemented, tested |
| `00891b90` | The armour asked about: the item equipped in biped slot 2 (upper body) | implemented |
| `00646cb0`, `00446390` | Weapon animation type at form +0xf4; types 2, 5, 6, 8, 9 pass the two-handed test | implemented, tested |

The formula (`00647d10`): SpeedMult (AV 21) × 0.01 (double constant
`01031148`) × `fMoveBaseSpeed` × legs, × `fMoveNoWeaponMult` when the
weapon is away and actor vfunc +0x218 is true (people `008d0360` → 1,
creatures `0047c850` → 0), × (1 − armour penalty − drawn weapon penalty),
× `fMoveSneakMult` sneaking (movement flags 0x400 without 0x800,
`004997b0`); negative → 0.

- Legs: 1.0, `fMoveOneCrippledLegSpeedMult` with one crippled,
  `fMoveTwoCrippledLegsSpeedMult` with two (`00646800`: condition ≤ 0).
  IgnoreCrippledLimbs (AV 72) resets it to 1 only when actor vfunc +0x358
  is false; for the player that vfunc is `00954cc0`, over-encumbered. So
  an over-encumbered player with IgnoreCrippledLimbs is still slowed.
  (`body_parts::leg_speed_mult`, used by AI code, does not have this
  exception; not changed here.)
- Weapon penalty, drawn only: animation types 5 and 6 `fMove2HRPenalty`
  (0.1), 8 and 9 `fMove2HBigPenalty` (data 0.1), 2 (two-handed melee) and
  7 (energy rifles) none.
- Armour penalty: `fMoveHeavyArmorPenalty` (data 0.15) or
  `fMoveMediumArmorPenalty` (data 0.075).
- `FalloutNV.esm` values: base 77, run × 4, sneak × 0.57; exe defaults
  for `fMoveNoWeaponMult` 1.1 and `fMove2HRPenalty` 0.1.

Deviations fixed: the viewer had hard-coded 77/4/0.57, no weapon-away
× 1.1, no armour or weapon penalties, and applied the run perk to walking
too. Float order follows the exe as single-precision steps; the tests
declare CPU tier B assuming PC=24 (the D3D thread's control word), not
confirmed for this caller and not run on `nv-call`.

## Running and jumping allowed (`0093e860`)

The player's control handler (an aligned-stack prologue at `0093e860`;
Ghidra had no function there). Keyboard path: with Always Run (control
10) on, running unless Run (control 9) is held; off, running while it is
held; in both cases not when process vfunc +0x3e4 `GetAnimAction` (Xbox
PDB) is 7, the player is over-encumbered (vfunc +0x358), or process vfunc
+0x404 `GetIronSights` (Xbox PDB) is set. Then running is cleared when the
grabbed object's weight (player +0x640) exceeds `fGrabMaxWeightRunning`
(50). Each direction key sets its flag and walk (0x100) unless running
(0x200) is set. Jump (control 12) is ignored while over-encumbered.

Implemented: `may_run` (over-encumbered, iron sights, grabbed weight),
`may_jump`. Tested. The viewer passes no iron sights and no grabbed weight
(neither exists there yet); anim action 7 is not modelled.

## Jump and air control (`physics::controller`)

The controller is the game's own since B1 PR 10 (PHYSICS.md, "The
character proxy"); its states set the velocity once per move.

| Address | What | Status |
| --- | --- | --- |
| `00930640` | Jump height `fJumpHeightMin` × `GetScale` (`00567400`), × `fJumpSwimmingMult` swimming | implemented, tested (player scale taken as 1) |
| `00884aa0` | Player vfunc +0x254: `00930640` then the jump sound | sound not done here |
| `00c6d4b0` | The jump asked for: wanted state jumping, `fJumpHeight` | implemented |
| `00cd4800` | On-ground state: a wanted jump is taken only on walkable ground (flag 0x400); the jumping state runs at the next move | implemented, tested |
| `00cd4280` | Jumping state: √(2 \|g × mult\| h) up **added to the proxy's own velocity** (`hkpCharacterProxy::getLinearVelocity` `00cac950`; the zero vector `01267e30` only without a proxy): the run-up is kept; then into the air at once | implemented, tested |
| `00cd3fb0` | In-air state: gain `fAcrobatics` × 0.3 + `01267bbc` (0, no writer), or 1 with flags 0x1800; at most 2000 Havok/s an update (`bhkCharacterStateInAir` +8); the up component kept; gravity × `fGravity` × dt added | implemented, tested |
| `0087d6c0` | Actor vfunc +0x2e4, air control (`fAcrobatics`): 1.0 for everyone | implemented |
| `00d6aef0` | Havok's movement util: the velocity moved toward the wanted one in the surface's frame (forward (0, −1, 0), the support normal), gap cut to the state's maximum | implemented |

Correction: an earlier batch read the jumping state as replacing the
horizontal velocity with `01267e30` (zeros); that is only the branch
without a proxy. With one, the jump keeps the run-up.

Unresolved, labelled in code:
- The wanted velocity is the key direction at the formula speed; in the
  game it is the PlayerMover's move delta (`009e9e50`), whose length comes
  from the movement animation's root motion (`00494390`).
- Backward and sideways speeds follow each animation's own root speed in
  the game (`00895110`); here every direction moves at the formula speed.
- Camera follow: the camera sits at `cellview::EYE_HEIGHT` above the feet.
## Kept on the land (`0092f260`)

`MobileObject::Move` (`0092f260`) runs for the player too: after the
controller's move, outdoors, feet more than 30 under the land's height
(`LAND`, `004572e0`) are put on it (`world::ground::kept_above_land`;
`walk::walk`). Its other rule, moving people further than
`fCharControllerWarpDistSqr` from the camera without their controller,
never applies to the player. Details and the live runs: PHYSICS.md,
"People on the ground (B4)". Implemented, tested (`world::ground`),
verified live (the player started 54 under the land is put on it); not
compared.

## Settings

| Setting | Exe | Data | Read by |
| --- | --- | --- | --- |
| `fMoveBaseSpeed` | 85 | 77 | `00647d10` |
| `fMoveRunMult` | 4 | 4 | `00647f00` |
| `fMoveSneakMult` | 0.6 | 0.57 | `00647d10` |
| `fMoveNoWeaponMult` | 1.1 | – | `00647d10` |
| `fMoveHeavyArmorPenalty` | 0.2 | 0.15 | `00647d10` |
| `fMoveMediumArmorPenalty` | 0.1 | 0.075 | `00647d10` |
| `fMove2HRPenalty` | 0.1 | – | `00647d10` |
| `fMove2HBigPenalty` | 0.15 | 0.1 | `00647d10` |
| `fJumpHeightMin` | 64 | – | `00930640` |
| `fJumpSwimmingMult` | 2 | 2 | `00930640` |
| `fGrabMaxWeightRunning` | 50 | – | `0093e860` |
| `fJumpMoveMult`, `fJumpMoveBase` (GMST) | 0.3, 0 | – | no reader; the controller uses its own statics |

## Checks and handoff

Branch `claude/m1-movement`. Tests: `world` `locomotion::tests` (5),
`physics` `a_jump_rises_its_height_keeps_its_run_up_and_lands` (B1 PR 10).
Not compared against the original game. Next action: record the player's
speed and jump arc in the original (holstered/drawn rifle, heavy armour,
over-encumbered, running jump distance) with `nv-probe` on `00885bf0` and
the controller's velocity, and compare with these rules.
