# Player furniture use: implementation evidence

M1 work, 2026-10-06, branch `claude/m1-sitting`. Needed for the opening's
psych exam: VCG01 stage 27 shows objective 40 (sit on Doc's couch);
`VCG01DocMitchellCouchTriggerSCRIPT` completes it only when
`player.IsCurrentFurnitureRef DocMitchellCouchREF` holds inside its trigger,
then (with Doc seated, `VCG01DocMitchellChairTriggerSCRIPT`) starts Doc's
conversation after 2 s and disables everything but looking and the view
key. People's sitting (entry/exit, idle tree, sandbox) is older work, see
ENGINE_REFERENCE.md "Furniture" and `%USERPROFILE%\nv-re\findings\furniture.md`.

Status: implemented and tested with generated data. **Not compared** with
the original game; no live run of the couch route yet.

## Traced (FalloutNV.exe 1.4.0.525)

Names marked (Xbox PDB) come from `Fallout_Release_MemDebug.pdb`; PC
addresses were found independently (RTTI, vtables, call sites).

- `TESFurniture::Activate` (Xbox PDB) = `005095b0`, slot 73 of the
  TESFurniture vtable `01026d0c` (COL `0110ab88`, type descriptor
  `0118a220`). Activator sit state != 0: temporary third person
  (`00950340(1)`) and `Actor::StandUp` (vfunc +0x418; player
  `00953ee0` = `PlayerCharacter::InitiateGetUpPackage`, Xbox PDB). Else the
  nearest free usable marker (`005686b0`; none: activation fails); a bed
  marker (1–9, `005094f0`) takes the ownership/sleep checks and the sleep
  menu; any other: `00950340(1)`, then `00953d80`
  (`PlayerCharacter::InitiateSitSleepPackage`, Xbox PDB), which creates a
  type-6 package targeting the furniture.
- Player branch of the sit procedure `00904f50`: 40 units or more from the
  marker copy, the player is `SetPos` to it and `SetAngleZ` to its heading
  (people walk); within 40 the common sit update `009213e0` runs.
- Temporary third person: `PlayerCharacter` +0x64a/+0x64c/+0x64d/+0x64e/
  +0x64f are `b3rdPerson`, `bWant3rdPerson`, `bTemp3rdPerson`,
  `bTemp3rdPersonSwitchBack`, `bTemp1stPerson` (Xbox PDB +0x65a.., PC 0x10
  lower). `00950340` begins it (not while +0x64f/+0x64d), switching from
  first person and marking the switch back. `009503d0`, called every
  player update (`00941d83`), ends it when `GetAnimAction() == -1`
  (process vfunc +0x3e4), `GetKnockState() == 0` (+0x40c) and the view key
  isn't held (`011e07b8`/`011e07b9`); then first person comes back
  (`00950110(1)`). So the view is third person while the entry/exit plays.
- Activate key (player update, unanalysed code around `00940460`):
  refused when `GetSitSleepState()` is 1,2,3,5,6,7,8 or 10 (byte table
  `00944228`, jump table `00944220`), while an anim action plays
  (`00940605`), or while the POV is switching. When nothing under the
  crosshair was activated (`0094076c`..`009407c8`) and the player has a
  current furniture (process +0x4c8), that furniture is activated
  (`00573170`), i.e. E on nothing gets a seated player up.
- Pitch limit `00931d90`: [-1.5533430576324463 (double `0108a7f0`), set to
  -1.553343], upper 1.553343 (`0108a7f8`), or `fSittingMaxLookingDown` ×
  π/180 (double `01023128`) for sit states 1–5.
- Idle tree (FalloutNV.esm): the player's seated loop in first person is
  `ChairDynamicIdle1stPerson` (`1stP_ChairDynamicIdle.kf`, `GetIsID` player
  and `IsPC1stPerson`), whose file has a `Camera1st` track; in third person
  the player gets the people's entries/exits (`Chair_ForwardEnter.kf`) and
  `PlayerSittingChairIdles` (only when not first person).
- Doc's couch `DocMitchellCouchREF`: MNAM 0x40000004, so only marker index
  2 (number 14, front) is usable (`nvinspect <Data> sit DocMitchellCouchREF`).

## Implemented

- `world::furniture`: `activate`, `player_approach`,
  `player_activation_blocked`, `TempThirdPerson`, `clamp_pitch` (translated
  from the addresses above); `IdleQuestion::first_person` answers
  `IsPC1stPerson`.
- `viewer/src/sitting.rs` `player_furniture` (after `walk::walk`): E on
  furniture (`scripts::use_object` → `Used::Furniture`) or, seated, E on
  nothing (no door/person/object, crosshair use allowed) runs the
  activation; the player is put on the marker and goes through the same
  `Sitter` procedure as people (entry root travel to the seat,
  +HeadingDelta, exit, +π); the camera takes the procedure's heading and the
  pitch limit; once the temporary view ends seated, the tree's first-person
  loop plays in `PlayerIdle` (`set_seated_loop`), its `Camera1st` placing
  the eye; a player in furniture without a procedure (load, script) is
  seated at once. `scripts.rs` drops the old "sit in place, walk 40 away to
  stand" stand-in and refuses object activation in the blocked states.

Tests: `world` unit tests for each rule; `world/tests/sitting.rs`
`the_player_sits_and_stands_through_the_games_procedure` (generated
plugin: GetSitting 0→2→3→4→0 for the player, view switches, blocked E,
exit back to the marker).

## Gaps (not implemented, not substituted)

- The third-person camera and player body now exist (branch
  `claude/m2-third-person`, [CAMERA.md](CAMERA.md)): the temporary view
  is the camera's (`world::player_camera`), the body plays the entry,
  seated loop and exit. Not compared with the original's framing.
- Movement keys while seated: no handler traced; the player is held on the
  seat. The HUD mode switch `00771700` (11 seated, 10 in transition, 2 on
  release), holstering (`0093a5f0`, `008a6840`), the knock state and the
  view key hold are not modelled.
- Which of the player's two animation sets gets the first-person loop on
  the switch back (`00950110` → `00951a10`) isn't traced.
- The furniture prompt text ("Sit (name)") is still the viewer's.
- Next: live run `--stage VCG01 27`, sit on the couch, check the trigger
  completes objective 40 and Doc's questionnaire starts; compare entry,
  seated eye height and standing up with the original.
