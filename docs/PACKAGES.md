# AI package types and package actions (M2: Ghost Town Gunfight)

Branch `claude/m2-packages`, 2026-10-06. Evidence for the package types
the Goodsprings gunfight (`VMS16`, quest `00104EAE`, script
`VMS16QuestScript` `00105D4E`) and Back in the Saddle (`VCG02`) use,
read from FalloutNV.exe 1.4.0.525 in Ghidra and from `FalloutNV.esm` with
`nvinspect`. Raw exports stay private
(`%USERPROFILE%\nv-re\work\packages-2026-10-06`). Status words:
**implemented** (code exists), **tested** (generated regression), **not
compared** (nothing here has been checked against the running game).

## What the gunfight asks for

- Stage 70 (stage result script): `MoveTo` Joe Cobb to
  `JoeCobbPositionMarker`; with `bTrudyHelp`, the four settlers to
  `GSSettler01..04PositionMarker` and Trudy to `TrudyPositionMarker`.
- `VMS16QuestScript` GameMode, once stage 70 is done: Joe Cobb
  `AddScriptPackage GSPGTravelPackage` (travel, `PLDT` near
  `PowderGangDestination` radius 512); without `bTrudyHelp`, settlers 02–04
  `AddScriptPackage GoodspringsFleePackage`.
- The settlers' own lists start with `GoodspringsFleePackage` (type 10, no
  `PLDT`, no `PTDT`; `(bGunFightStart == 1 OR VMS16b var 1 == 1) AND
  bTrudyHelp == 0`), then a guard package (type 14; `bGunFightStart == 1
  AND bTrudyHelp == 1`): `GSSettlerCFGunfightPackage` and
  `GSSettlerAMGunfightPackage` near the **editor location** with `PTDT`
  their position marker, value 0; `GSSettlerAAMGunfightPackage` and
  `GSTrudyGunfightPackage` near their marker (`XMarkerHeading`s). Settler
  04 has `GSSettlerAmbushPackage` (type 9, `PLDT` and `PLD2` editor
  location, `PLD2` radius 512) instead of a guard package.
- Stage 100 removes the settlers' script packages and gives Ringo
  `GSRingoAfterVMS16DialoguePackage`. `GSJoeCobbLeavePackage` (another
  branch) begins with `set VFreeformGoodsprings.TrudyToBar to 1` and ends
  with `RemoveScriptPackage`: the only package action with content among
  these.
- `VCG02` depends on travel end actions: `VCG02SunnyTravelToWell1` ends
  with `SetStage VCG02 30`, `VCG02SunnySneakCloserToWell` with `SetStage
  VCG02 40`; the opening's `VCG01DocMitchellTravelToSkullTestSpot` ends
  with `DocMitchellREF.Look player`. These now run on arrival (in sight,
  and out of sight through the low process's travel `0090ad40`).
  Packages started out of sight don't run their begin actions yet.

## Traced rules

| Rule | Address | Status |
| --- | --- | --- |
| Procedure lists by list type (`011a3ff0`), record type → list (`006777b0`): flee 32 = FLEE_NON_COMBAT; guard 41 = GUARD; ambush 30 = TRAVEL, AMBUSH_WAIT; use item at 28 and use weapon 44 = ACQUIRE, TRAVEL, own | data `011a3ff0`, `006777b0` | implemented, tested (`world::ai::procedures`) |
| DONE steps back (repeats) for list types 1, 4, 5, 0x29 (guard), 0x2d | `008eeec0` case 0x36 | implemented, tested |
| Procedure dispatch: FLEE_NON_COMBAT → HighProcess +0x7e4 `008ddac0`; GUARD → +0x7f4 `00902290`; AMBUSH_WAIT → +0x7d4 `008ee3e0`; USE_ITEM_AT +0x7d8 `008f9320`; USE_WEAPON +0x7dc `008f7730` | `008eeec0`, vtable `01087864` | read |
| Flee: from = `PTDT` reference / linked reference (`0091aca0` → `0091ab30` → `0091ae00`); to = `PLDT` reference / linked reference / object search (`00924080` → `00923e40`; kinds 1, 2, 3 give none). Neither: procedure over at once. Safe when distance ≥ `PTDT` value (no place) or ≤ location radius (place); stopped and flag 0x4 or 0x2: over. Else `InitiateFlee` (Xbox PDB; actor +0x410 `00897de0`) | `008ddac0` | implemented, tested (`world::ai::flee`); viewer runs to a flee-to place; fleeing from a target without a place is not done (engine flee path search `009f1140` not traced) |
| Guard: guarded = `PTDT` reference / linked / self (`006759e0`); post: editor location (kind 3, `GetEditorLocationCoord` (Xbox PDB), actor +0x160), self (2), linked (6), the reference (0), else the guarded reference; radius = location radius (`00676280`), else `PTDT` value, else 30; path radius max(r/2, 15); at-post = package's own test with a location, else distance < r | `00902290`, `00912db0` | implemented, tested (`world::ai::guard`, viewer `guard_frame`) |
| Gait to the post: run with flag 0x2000 or in combat; walking → run when path left > 2r; running → keep while ≥ r | `008daa20` | implemented, tested |
| At the post with no intruder: wander when the location radius ≠ 0 (or no location and `PTDT` value > 0); else turn to an `XMarkerHeading` location reference's heading (kind 3 gives no reference: no turn) | `009026c0` | turn implemented and tested; wandering not done |
| `MoveTo` keeps the editor location: `SetStartingPosition` (Xbox PDB; actor +0x46c `008a82e0` → `0087f800`) only runs when `HasEditorLocation` (+0x290) is false (`004698a0`, `0055d760`). So CF/AM settlers walk back from their gunfight markers to where they were placed | `004698a0`, `0055d760`, `0087f800` | implemented (existing `destination` kept), tested |
| Location load: kind 1 radius dropped, kinds 2/3 form dropped | `0067f060` | implemented, tested (changes outdoor "in a cell" wander radius to 0) |
| Type data: `PKW3` 24 bytes (always hit, no damage, crouch, hold fire, volley, repeat, burst u16, volley min/max u16, wait min/max f32, weapon), `PTD2` (use weapon only), `PKPT` (repeatable, start at linked ref), `PKE2` escort radius i32, `PKFD` follow radius f32, `PLD2` filed by type (dialogue, eat search, escort search, ambush location, follow start, use item at, use weapon location); `PKAM`/`PKED` have no loader case | `00673a40`, `00671e10`, `0067cb70`, `0067c450`, `0067bc60`, `0067c040`; layouts `TESUseWeaponPackageData`, `TESPatrolPackageData`, `TESEscortPackageData`, `TESFollowPackageData`, `TESAmbushPackageData` (Xbox PDB) | implemented, tested (`world::ai::data`) |
| Actions: begin (+0x598 `00903a80`) from `AddScriptPackage` (`005cc4f0`) and evaluation (`0090a1a0`, `00907ab0`); end (+0x5a0 `00903d10`) when advancing to DONE (`0091ecf0`, once via +0x5a8) and on give-way (`00907ab0`); change (+0x59c `00903bf0`) from `AddScriptPackage` and `ResetAI`. Each: script on the actor (`005ac1e0`), idle (+0x44), topic via greeting (+0x2a4) | as listed | implemented, tested (`world::ai::actions`, viewer `rethink`/travel arrival/flee end, `scripts.rs` dispatch) |
| `AddScriptPackage` installs at once (`PutCreatedPackage` (Xbox PDB), actor +0x2f4); the AI taking the package up does not begin it again | `005cc4f0` | implemented (`GameState::package_begun`), tested |

`PKDT` general flag names used in comments (0x2 "must reach location",
0x4 "must complete", 0x2000 "always run") are editor terms, inferred; the
code only tests the bits.

## Not done (labelled in code)

- Guard intruder scan, warnings and attacks (`009026c0` loop, `00902c80`,
  `iGuardWarnings`, `fGuardPackageAttackRadiusMult`). The gunfight's guard
  packages have `PTDT` value 0, for which the scan finds nobody.
- Guard wandering at a post with a radius; the guarded reference's extra
  data 0x1c override (`0041c8d0`).
- Ambush wait (`008ee3e0`: attack the detected hostile player, hide search
  `006d62e0`, sneaking); use item at; use weapon (`008f7730`); patrol.
  Settler 04's ambush package only travels (back to its editor location in
  the saloon) with the existing travel code.
- Fleeing from a target without a place; object/type searches for targets
  and locations (kinds 1, 2 / 4, 5).
- A begin action without an idle stopping the current idle; the end
  action's flag +0x590; whether re-picking a package that reached DONE
  begins it again (`0090a1a0`).

## Checks

Root: `cargo test --workspace` (the cellview music timing test can fail
under parallel load and passes alone; the repository-hygiene walk needs
GitHub Desktop's git on `PATH` in a worktree), `cargo clippy --workspace
--all-targets`, `cargo fmt --all -- --check`. Viewer: `cargo build`,
`cargo test`. Not run against the installed game or compared with it.

**Next action:** run the gunfight from a save before stage 70 (both
`bTrudyHelp` branches) in the original and in nv-rs and compare where the
settlers stand and run.
