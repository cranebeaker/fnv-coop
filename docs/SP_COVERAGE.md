# Single-player coverage: what is missing, measured

First measured 2026-11-02 with `crates/nv-coverage` on the five official
ESMs (`FalloutNV.esm`, `DeadMoney.esm`, `HonestHearts.esm`,
`OldWorldBlues.esm`, `LonesomeRoad.esm`; no meshes or archives). The load
sweep ran in records mode only: the asset half of the sweep needs the full
`Data` folder and has not run yet (see "Next step on the test PC").

Nothing here is compared with the original game. "Handled" means the world
host carries a function out; "runnable" means a result script runs against
a stub world. Neither says the game plays correctly.

## How to run it

```
cargo run --release -p nv-coverage -- report --data <Data> --out out
powershell -File scripts\nv-coverage.ps1 -Data <Data> -Out out        # CI form, builds and prints the headline
nv-coverage functions|quests|dialogue|sweep|checklist --data <Data>   # one section as JSON
nv-coverage script <EditorId> --data <Data>                           # print a script's source
```

`FNV_ESM_DIR` stands in for `--data`. `report` writes `nv-coverage.json`
(machine readable) and `nv-coverage.md` (summary). Options: `--sweep
auto|records|full|off` (auto picks `full` when meshes are present),
`--stride N` (every Nth exterior square), `--max-interiors N`,
`--max-squares N`. Set `NV_RS_REPO` when running outside the repository so
the source-text checks (events, perk entry points, translated handlers,
checklist file pointers) can find it. About 30 s in release on the ESMs.

## What it measures

| Part | Method | Code |
|---|---|---|
| Script functions | Parses every script source in the load order (object, quest, effect scripts, quest stage results, dialogue results, terminal results), counts each call and each condition function, marks it handled when it is in `world::scripting::HANDLED` or `world::script_functions::handled()`, groups by system, and lists the main-line quests that use each missing function | `funcs.rs`, `scan.rs`, `systems.rs` |
| Events | Script block kinds, and whether the quoted kind appears in the world or viewer source | `checklist.rs` |
| Quest walk | One `GameState` per questline. Every stage in order with `Runner::set_stage`, then one second of `update`. Verdicts: clean, unhandled (the host returned no value for a function that is not in the handled lists), needs live world (a handled function the stub world cannot answer: `GetDistance`, `GetInCell`...), static missing (a missing function on a path the run did not take), parse error, panic, refused. Objective targets are checked to be placed references in a cell | `quests.rs` |
| Quest graph | Which script kind sets each stage (`SetStage` calls), stages nothing sets, quests that depend on a missing function | `graph.rs` |
| Object scripts | Each distinct script on a placed reference runs once with all its event blocks, in a worker process so a stack overflow is caught and named | `objects.rs` |
| Dialogue | Every INFO condition function against the handled list; every result script statically and run | `dialogue.rs` |
| Load sweep | `records`: the world crate's loader on every interior cell and every exterior square of every worldspace. `full`: the viewer's own loader (`cellview::Game`: models, textures, collision, terrain, doors, water, particles) with the loader's notes classified (missing model, missing texture, unreadable model, unreadable collision) and counted by path | `sweep.rs` |
| Perks | Entry points per perk, and whether the world crate names the entry point's constant outside its definition | `content.rs` |
| Checklist | Content types with world and viewer file pointers, checked to exist | `checklist.rs` |
| Fix list | Missing functions in batches of up to eight per system, ranked by quest stages blocked, then main-line quests, then uses; each with size, translation source and an agent brief; translated exe handlers already in `crates/engine-fallout` are named | `fixlist.rs`, `content.rs` |

Main-line quests: `VCG*`, `CG*`, `VMQ*`, `VEFR*` (base game main), `VMS*`
(its own line), and every quest with `MQ` in its editor id in the four DLC
plugins.

## Results (ESM-only run, 2026-11-02)

| Measure | Value |
|---|---|
| Functions in the exe's script table (named) | 622, of which 376 are carried out |
| Functions the data's scripts and conditions use | 410: 358 carried out (87.3%), 52 missing |
| Uses weighted | 147,637 uses, 310 on missing functions (99.8% covered) |
| Scripts parsed | 14,825 sources; 1 failure (a PACK script, `vault106FatherWalk3A`) |
| Main quests (VCG, CG, VMQ, VEFR) | 47 quests, 283 stages, 275 run (97.2%); 8 stop on a missing function |
| VMS series | 79 quests, 486 stages, all run |
| Dead Money / Honest Hearts / Lonesome Road / Old World Blues main | 75/75, 71/72, 46/46, 30/31 |
| Objective targets | about 1,500 checked across all lines; none unresolved |
| Dialogue | 27,813 INFOs with conditions, 14 use an unsupported function (`GetDisposition`); 8,898 of 8,901 result scripts run clean |
| Object scripts | 11,502 placed references carry 2,259 distinct scripts: 2,243 clean, 14 hit a missing function, 0 panics, 2 crash the process |
| Load sweep, records mode | 491 interiors and 44,025 exterior squares (32 worldspaces) all load; 875 cells have disabled references, 232 have a base object missing |
| Perks | 256 perks, 261 entry points, 230 on entry points the world crate applies |

The main reading: script-function coverage is nearly complete. The data
does not show missing script functions as the thing standing between the
port and a finished game. What stands there is behaviour the stub world
cannot reach (AI packages, triggers firing in a live cell, animation,
presentation), assets the sweep has not yet checked, and a few host bugs.

## Findings that are bugs, not gaps

1. **Stack overflow in the host.** `VPitPlayerDoorPitAccessOpen`
   (ref `0013811B`) and `VMS18DumpsterActivatorSCRIPT` (ref `00127DD9`) kill
   the process when their `OnActivate` runs: `Activate` on another
   reference re-enters `OnActivate` with no guard (`MAX_DEPTH` protects
   `set_stage` only). Any player activation of those objects would crash
   the viewer. Reproduce: `nv-coverage report` lists them under "Object
   scripts". Fix needs the exe's re-entry rule for `Activate` from a
   script (address to be cited before coding).
2. **One script does not parse**: PACK `vault106FatherWalk3A`, an
   unexpected `.` token.

## Top 20 gaps, in order

Order: quest stages blocked, then main-line quests, then uses. "Exe" means
the behaviour has to come from the decompilation; "data" means the records
settle it. Items with a translated handler already in the tree
(`crates/engine-fallout/src/units/fallout_shared/tesscriptfunctions_p*.rs`)
name the address.

| # | Gap | Evidence | Size | Source |
|---|---|---|---|---|
| 1 | Host re-entry of `OnActivate` crashes (2 scripts, probably more in live play) | worker crashes | S | exe (re-entry guard) |
| 2 | `TriggerScreenSplatter`, `SetScreenSplatterFade` | VCG00 stages 36 to 39 and 50 (the opening) | M | exe, plus a viewer overlay |
| 3 | `AutoDisplayObjectives` | VCG01 stages 55 and 200 | S | exe |
| 4 | `AgeRace` | VCG00 stage 5 (character creation) | S | exe, `chargen.rs` |
| 5 | `AddItemToLeveledList` | Honest Hearts and Lonesome Road start quests, VMS01 stage 10 | M | translated at `005de700`; needs runtime leveled-list edits |
| 6 | `SetCellImageSpace` | NVDLC03MQ05 | S | exe |
| 7 | `IsPlayerMovingIntoNewSpace` (condition) | VMQ03a and 8 conditions | S | translated `005d4af0`, condition `005a3a00` |
| 8 | `GetAnimAction` | 44 calls in 15 scripts | M | translated `005d79e0`; needs animation state from the viewer |
| 9 | Condition functions `IsLeftUp` (22), `GetDialogueEmotion` (20), `GetIsUsedItem`/`Type`/`EquipType` (23), `GetDisposition` (15), `GetAttacked` (11), `GetVatsTargetHeight` (7), `GetRadiationLevel` (7) | INFO and package conditions | L | mostly data-only plus exe for semantics |
| 10 | `UseWeapon` (13), `SetPackDuration` (12), `Reset3DState` (12), `GetOwnerLastTarget` (5), `PickIdle` (2) | object scripts | L | exe (`005d3ae0`, `005cb0c0` translated) |
| 11 | Events never dispatched: `OnReset` (45 scripts), `OnCombatEnd` (36), `OnPackageStart/Change/End/Done` (36), `OnDestructionStageChange` (16), `OnFire`, `OnMagicEffectHit`, `OnGrab`, `OnRelease`, `OnSell` | event table | L | exe, `ref_scripts.rs` and the viewer's `scripts.rs` |
| 12 | 88 main-line stages in 33 quests are set by no script in the data (VEFR faction reactions, VMQHouse, CGTutorial, VMQTops, DLC MQ): find the path that sets each, or confirm it is the player-visible route | quest graph | M | data first |
| 13 | Perk entry points not applied: `Activate` #27 (7 perks), `Modify Addiction Chance` #23 (6), `Add Leveled List On Death` #21 (2), 14 others with one perk each (Mysterious Stranger, Miss Fortune, Meltdown, Jury Rigging, unarmed power attacks, VATS helper, Improved Spotting) | perk census | L | exe |
| 14 | The full-data load sweep has not run: missing meshes, textures, animations, unsupported NIF blocks, collision and terrain failures are unknown | `nv-coverage sweep` on the test PC | M to fix after the run | data |
| 15 | `SetMinimalUse`, `DisableNavMesh`/`EnableNavMesh`, `HasLoaded3D`, `Rotate`, `IsGoreDisabled`, `PlaceAtReticle`, `ShowBarberMenu`, `ShowPlasticSurgeonMenu`, `EnableLoadingMenu`, `ResetXP` | used 1 to 5 times each | M | translated for `Rotate` `005c03d0`, `SetMinimalUse` `005dc5e0`, `ResetXP` `005de9f0` |
| 16 | Parse failure in PACK `vault106FatherWalk3A` | parse errors | S | data |
| 17 | AI package coverage per package type, navmesh paths across cells, animation availability: not measurable headless | needs bot runs | L | bot playthroughs (`docs/PLAYTESTING.md`) |
| 18 | Partial systems in the checklist: radio stations, fast travel in the viewer, crafting gates, packages per type, original-save import, Old World Blues implants | checklist | L | mixed |
| 19 | Object scripts the stub world cannot judge (`GetDistance` 26, `HasPerk` 17, `GetDetected` 13): need a live-cell harness | object walk | M | harness, no exe |
| 20 | NVSE functions: none used by the vanilla data (no parse failures), so this only matters for mods | checklist | n/a | xNVSE source |

Agent briefs for items 1 to 16 are in the JSON (`fix_list[].agent_brief`)
and `research/09-sp-gaps.md`. Each brief tells the agent to implement a
function only with a cited exe address or record field, to add it to the
right `HANDLED` list, to add a unit test on real records, and to re-run
`nv-coverage functions` to see the count fall.

## What was not implemented, and why

No function was added. AGENTS.md requires behaviour from the exe or the
data with provenance. The tree has translated handlers for some of the
missing functions (named above), but they are written against the engine
crate's `Engine` memory model, not the world host's state, so porting them
is real work, not a data fix. The rest have no address in the tree.

## What the numbers do not say

- A runnable stage is a result script that runs, not an objective that can
  be reached. 97 percent of main-line stages running says the script layer
  is close to complete, nothing about AI, triggers, pathing, animation,
  sound or rendering.
- The stub world answers no question that needs a live cell. Those show as
  "needs live world", not as gaps.
- The checklist status is a guess from which files exist.
- No result is compared with the original game.

## Next step on the test PC

```
powershell -File scripts\nv-coverage.ps1 -Data "<path to the full Data>" -Out sweep
```

This runs the load sweep through the viewer's loader on every interior and
every exterior square (stride 1 is slow; `-Stride 4` first). Its
`load_sweep` section lists missing and unreadable assets by path with cell
counts. Triage those into the fix list, then use bot playthroughs to
measure what a headless run cannot.
