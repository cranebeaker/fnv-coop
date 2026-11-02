# Quest targets (compass quest markers)

M2 blocker for Back in the Saddle (`VCG02` 0010A214) and Ghost Town
Gunfight (`VMS16` 00104EAE): objectives had no targets, so the compass showed
no quest markers. Branch `claude/m2-quest-targets`.

Status: **implemented and tested** on generated data, and checked against
the installed `FalloutNV.esm` (below). **Not compared** with the original
game running.

## Record

`QUST` objectives: `QOBJ` (index), `NNAM` (text), then per target `QSTA`
(8 bytes: reference u32, flags u8, 3 bytes the game leaves unread) followed by
that target's `CTDA` conditions. `00610120` (`TESQuestTarget::Load` (Xbox PDB))
reads them into `TESQuestTarget` (Xbox PDB: `cFlags` +0x00, `objConditions`
+0x04, `m_pTargetRef` +0x0c, `m_TargetPath` +0x10, size 0x48). Flag 0x01 is
`GetIgnoreLocks` (`00610190`). Installed data: `VCG02` objectives 5/20/40/60/3
target `SunnyRef`, 30 and 50 the geckos, 70 `TrudyRef`; `VMS16` 70 targets six
Powder Gangers, each with `GetDeadCount <base> == 0`.

## Rules traced (FalloutNV.exe 1.4.0.525)

| Address | What |
| --- | --- |
| `005ec5d0` | Objective state change. Showing one (state 1) of a quest not completed (flag 0x02) makes it the active quest when there is none (`009529d0`, `PlayerCharacter::SetActiveQuest` (Xbox PDB), player +0x6b8). |
| `0060fb60`, `0077a480`, `0060ca30` | A stage entry completing/failing the quest, or the completion notice, clears the active quest if it is that quest. |
| `005d9ac0` | `ForceActiveQuest`. |
| `00952ba0` | `GetCurrentTargetList` (Xbox PDB): none without an active quest; rebuilt when flagged (+0x206) or when `0060efd0` (`ValidateTargetList`) finds it stale. |
| `0060f110` | `TESQuest::UpdateCurrentTargetList` (Xbox PDB): nothing for a completed quest; per player objective of this quest, `005ec500`. |
| `005ec500` | `BGSQuestObjective::UpdateCurrentTargetList` (Xbox PDB): only state 1 (shown, not completed); each target whose conditions pass (`00680c30`, subject = target reference, no target) is added and its path built (`00952d60`). |
| `00952d60` | `BuildPathToTarget` (Xbox PDB): path from the player; third argument (ignore locks) is never read. |
| `006d4f70` | Navmesh-level search first (`006b8c50`, **not translated**), else `TeleportDoorSearch` (`006f34e0`, vtable `0106d8fc`). |
| `006f36d0` | Door search expansion: the place's load doors (`0054db50`: enabled, not deleted, base `DOOR`, has teleport; worldspace: its persistent cell, `00588270`); cost = straight distance to the door from the start or the last arrival point; +409600 (`0106c238`) if the player can't pass the lock (`00502450`), +409600 if the door base has flag 0x08 (`00518000`, minimal use). |
| `006f3b00`, `006f3fb0` | Goal = the target's place; cheapest first. |
| `005cbb70` | What the compass follows: the path's first door (`Doors[0].pDoor`, target +0x20), else the target reference. |
| `00779070` | HUD compass quest loop: only with `bShowQuestMarkers` (`011e07f0`, default 1); icon `glow_hud_compass_objective_marker.dds`; bearing clamped to ±70 (not hidden); x capped at 0.8 × 2 × half + 70; `_Heading`, `_Distance` (squared distance), visible; blink only if the compass's menu state (+0x24) is 1. The "door: full alpha" branch compares the reference's own form type with 0x1c, so it never applies. |
| `00778c20` | Blink: squared settings (`0077f9c0`), defaults `iHUDCompassIconBlink*` MaxDist 2000, MinDist 256, ThresholdDist 500, Slowest/Fastest/ThresholdBlinkTime 750/1500/1000, Longest/Shortest/ThresholdPause 1500/50/600 (no `GMST` overrides in the ESM). `_AlphaDown` 0 rising / 1 falling / 3 holding; pause end in `user10` (0x100e). |
| `00952c30` | `CheckForQuestTargetUpdate` (Xbox PDB) flags a rebuild when the player or a target changes cell. |

Map markers' `_Distance` is the squared distance too (`004a7290` then
`fabs` `00408860`); the viewer had set its square root. Fixed in both
compasses.

## Implemented

- `world::quest::QuestTarget`, `Objective::targets`.
- `world::quest_targets`: `objective_shown`/`quest_ended` (active quest),
  `current_targets`, `DoorGraph` + `door_path` (the door search), `Tracker`
  (the followed reference; a path is kept per player cell, target and target
  cell).
- `ui::compass::{place_quests, blink_icon}` (translated `00778c20`), used by
  the HUD (blinking) and V.A.T.S.'s compass (placed only).
- Viewer: `hud::QuestCompass` / `follow_quest_targets` feed the compass, live
  actor positions for followed people.

Tests: `crates/world/tests/quest_targets.rs` (parsing, active quest, current
targets, door paths, lock/key/disabled), `ui` `hud::tests::quest_targets_on_the_compass`.

Installed-data check (temporary local test, not committed): with `VCG02`
active and its objectives shown, from inside `GSDocMitchellHouse` every target
follows `GSDocMitchellHouseIntDoorREF` (00103E61); outside, Sunny and Trudy
follow the saloon door 0010AEDC, Ringo `GSGasStationDoorRef`, the Powder
Gangers themselves (same worldspace). First rebuild 0.2–0.5 s in a debug
build (persistent cell scan and door reads, then cached).

## Unresolved / not done

- The navmesh-level path search (`006b8c50`) the game tries first; door
  choices may differ where it applies.
- Lock check details in `00502450`/`00502310` (ownership, factions);
  here: locked without the key = penalty.
- The door base the game's list leaves out (`011ca258`); a deleted target's
  stand-in (`GetReference(true)`, extra 0x1c).
- Order of the player's objective list (`00952a20`); record order used.
- Exactly when the game rebuilds paths (who calls `00952c30`); here per
  player cell. Rebuilds outdoors can cost time when crossing cells.
- V.A.T.S.'s menu state for the blink check (assumed not 1: no blinking).
- The player's custom map marker on the compass (`0077a400`, player +0x6f4).
- Pip-Boy local/world map quest markers: `MapMenu::AddQuestMarkers` (Xbox PDB)
  = `0079e0a0` (world map follows `0079f7e0`, the last worldspace door of
  the path; local map follows `005cbb70`; `Local/WorldMapQuestMarkerTemplate`,
  `fWorld/LocalQuestMarkerMin/MaxSize`). Not implemented (Pip-Boy files
  belong to the Pip-Boy work).
- Comparison with the original game.

Next action: run Goodsprings with `VCG02` active in the original and the
viewer; compare the marker's door and blink.
