# Reference scripts at runtime

Which placed objects' scripts run, when `OnLoad` fires and how trigger
volumes are evaluated. Code: `crates/world/src/ref_scripts.rs`
(`world::ref_scripts`), used by `viewer/src/scripts.rs`
(`attached_cells`, `refresh_cell_scripts`, `run_cell_scripts`).
Branch `claude/m2-cell-scripts`, 2026-10-06.

Status: implemented and tested with generated worlds. **Not compared with
the original game.** The Goodsprings route (VCG02 stage 20 from
`VCG02SunnyPatrolTriggerSCRIPT`, `GSSaloonTriggerScript`,
`SunnySmilesTriggerSCRIPT`, `GSJoeCobbTriggerScript`,
`VCG02TargetSCRIPT`'s `OnLoad`) has not been played through.

## Previous behaviour (replaced)

Outdoors only the grid square under the player had running object scripts.
`OnLoad` ran whenever the player crossed into another square, disabled
objects' scripts never ran, persistent objects (in the worldspace's
persistent cell) never ran outdoors, and a trigger ran `OnTriggerEnter` and
`OnTrigger` together on entry.

## Traced rules (FalloutNV.exe 1.4.0.525)

Names marked (Xbox PDB) come from `Fallout_Release_MemDebug.pdb`; PC
addresses were matched by structure, strings and vtable slots.

| Rule | Address | Notes |
| --- | --- | --- |
| Per-frame pass `TES::RunScripts` (Xbox PDB) | `00455490` | Running quests (TESDataHandler +0x118, flag bit 0), then the interior cell, or each grid cell `uGridsToLoad`² (setting `011c63cc`) in state 6 (`CS_ATTACHED`, Xbox PDB enum), x outer, y inner (`GridCellArray::Get` `004ba490`: index x·n+y); then the player (`011dea3c`). Guarded by `011c3ed1` (re-entry) and `0118c685`. |
| Cell pass `TESObjectCELL::RunScripts` (Xbox PDB) | `0054c740` | List at loaded data (+0xc4) +0x4c = `ScriptedRefs` (Xbox PDB `LOADED_CELL_DATA`). A reference runs if `Get3D` (`0043fcd0`) is set **or** form flag 0x800 (disabled, `00440da0`). If `RunScript` returns true the cell loop stops and `TES::RunScripts` skips the remaining grid cells. |
| `TESObjectREFR::RunScript` | `00565870` | Deleted (0x20) or no base: nothing. Actors (form types 0x2A/0x2B) with a process (+0x68) not at level 0 and no 3D (vtable +0x1d0 = `Get3D`, Xbox +0x1cc): skipped. Containers/actors run inventory item scripts (`004d2480`). Then `Script::Run` (`005ac1e0`) with the `ExtraScript` (extra 0x0D) event list. |
| Return value | `005e0d20`, `005e1550` | `ScriptRunner::Run` returns runner +0xa1, set before executing a command whose table entry has byte +0x25 set (flags 0x100): `Activate`, `MoveToMarker`, `ForceFlee`, `ForceTakeCover`, `ExitGame`, `MoveToMarkerWithFade` (command table dump). Events are cleared after the run (`005a8ea0`). |
| Event flags | `005ac750` → `005a8e20` | Sets the mask on the action reference's entry and on the "anyone" entry. Masks from the block handlers: `OnLoad` 0x1000 (`005cab00`, block 0x15), `OnTriggerEnter` 0x20000000 (`005cac40`, block 0x1A), `OnTrigger` 0x10000000, `OnTriggerLeave` 0x40000000. |
| `OnLoad` on cell attach | `0054bcf0` (from `00452ff0`, `00450dd0`, `00453dc0`) | For every reference of the attaching cell (two passes by base), before the disabled check, unless global `011ddf38` +0x244 bit 1 is set (unidentified; suspected save loading). Cells that stay in the grid are not attached again (`00452ff0` only finishes cells in state 5). |
| `OnLoad` on 3D attach | `00451ef0`, `0056f700` | After a reference's 3D is loaded (vtable +0x1c8 = `Load3D`), e.g. via the model loader `00440ba0`; disabled references in a buffered cell are flagged directly (`0056f700`). `Enable` (`005c43d0`) leads here. |
| Grid centre `TES::UpdateCurrentGridCell` (Xbox PDB) | `00452580` | Position → integer by FISTP (`00406d90`), `>> 12`. Kept while 4096 − \|p − (c·4096 + 2048)\| > 0 on both axes (3072 if the grid is 3; floats `01017a3c`, `01017a38`), else re-centred on the player's cell. |
| Trigger step `TriggerEntry::Update` (Xbox PDB) | `0062cc90` (called per entry by `0062de20`) | Skips a disabled/deleted trigger or one without 3D. Each overlapping reference: `RefList::Add` (`0062c9d0`) — known and not yet seen this step → `OnTrigger` flagged; new → added and an enter event queued. Living actors are counted through their controller, not ragdoll bodies (`004b59f0`, `IsDead`/`IsKnockedOut` vtable +0x22c/+0x230). `UpdateExitList` (`0062cb90`): unseen → leave event queued, removed. Then **one** queued event is taken (`0062cb60`): `OnTriggerEnter` or `OnTriggerLeave`, and the trigger's `ExtraAction` (extra 0x0E, `0041b4b0`) set to that reference. Scripts run later in the frame pass. |
| Force leave `TriggerEntry::RefList::ForceLeaveTrigger` (Xbox PDB) | `0062c860` via `0062d510`, `0062de90`, `00576760` | Every occupant: `OnTriggerLeave` flagged, action reference set, `RunScript` at once. Caller chain (`00576760` ← `00574400`, `0055a430`, …) presumed to be the trigger's Havok/3D removal; not confirmed. |

`uGridsToLoad` is 5 here (`world::ref_scripts::GRIDS_TO_LOAD`), as the
viewer's streaming radius assumes; the INI value is not read for this.

Actors outside the grid in middle/low process run scripts from the process
lists (`0096b050` calls `RunScript` for list 3; `00978550` for list 0);
not implemented here (AI owners). `TESDataHandler::RunAllPersistentRefScripts`
(Xbox PDB `822b1850`) exists; its PC address and caller were not traced.

## What is implemented

- Attached cells: the interior, or the 5×5 grid around the traced grid
  centre, limited to squares the viewer has loaded. Each square's objects
  include the persistent objects standing in it
  (`scripting::interactive_in_square`); their order within a square
  (cell objects, then persistent) is not traced.
- Every scripted object of an attached cell runs once per game-mode frame,
  disabled or not, with its pending events, `GameMode` and event blocks in
  script order (`Runner::run_reference_script`). The pass stops after a run
  that executed one of the commands above.
- `OnLoad` is flagged for all objects of a newly attached cell and for an
  object that becomes enabled; it runs at the object's next run, once.
- Triggers as `0062cc90`; disabling or detaching a trigger forces its
  occupants out (guess for the caller chain, see above). Body points
  (feet/middle/head) stand for the character's collision shape, as before
  (approximation).
- Script variables belong to their owner, never to a cell: a quest's run
  from the data handler's quest list (`00455490`, +0x118) whatever is
  attached, and a reference's live in its `ExtraScript` event list
  (extra 0x0D, `00565870`), which attaching or detaching its cell doesn't
  rebuild. Here both are `GameState::variables`, which no cell change
  touches; only `ResetQuest` (`005da180`) and a new game or a load start
  them over. So the DLC start quests' guards (`nEnableDLC`,
  `DoOnceMessage`, `VDLCPackQuest`'s `b…ItemsGiven`) hold after any number
  of cell changes (B32: checked live with three `MoveTo` cell changes;
  the repeat seen came from the acceptance script starting each route as a
  new game).
- F9 reloads reset the pass: every cell attaches again (`OnLoad` again,
  triggers re-entered). In the original, event flags are saved with
  references (`005a9f20` restores a pending 0x1000); not compared.

## Tests

`crates/world/tests/ref_scripts.rs` (generated world
`testdata::ref_scripts`): grid hysteresis and order, persistent objects in
squares, neighbouring cells running, `OnLoad` once per attach and not on
grid moves, disabled objects running and `OnLoad` on enable, trigger
enter/trigger/leave order, one event per step, force leave on disable and
detach, the pass stopping, interiors, a start-game quest's once-only
guard and a reference's counts surviving two cell changes (B32). Viewer tests in `scripts.rs` use the
new attach path for the VCG01 trigger and tester fixtures.

## Gaps

- Not compared against the original game; no live Goodsprings run.
- Nested `Script::Run` calls share the stop flag here (the game gives each
  its own runner, `005e2590`).
- The order of objects within a cell's `ScriptedRefs` list is not traced;
  it only matters when the pass stops early.
- `011ddf38` +0x244 bit 1 (suppresses `OnLoad` flags) is unidentified.
- Trigger overlap uses three body points, not Havok phantoms; only the
  player and the viewer's talkers are tested against triggers.
- The viewer streams squares around the player's own square, not the
  grid centre; a grid square outside the streamed set is treated as not
  attached.
