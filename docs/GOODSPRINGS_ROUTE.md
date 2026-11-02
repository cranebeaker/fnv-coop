# Goodsprings route: Back in the Saddle (VCG02)

M2 acceptance drive, 2026-10-06, branch `claude/m2-vcg02-route` (on
`claude/overnight-integration`). Quest `VCG02` 0010A214, quest script
`VCG02SCRIPT` 0010A1F0. Installed data, viewer release build. Logs and
screenshots are private (`%USERPROFILE%\nv-re\work\vcg02-2026-10-06`).

**Nothing here has been compared with the original game.** "Reached" means
the viewer's own scripts, packages and dialogue did it; steps marked
*simulated* were done with console lines that stand in for something the
viewer cannot do yet (listed under blockers).

## The quest, from the data

| Stage | Set by | Result |
| --- | --- | --- |
| 5 | `VGenericTimerSCRIPT` event 3 (Doc's farewell) | objective 3 "Talk to Sunny Smiles in the Prospector Saloon" |
| 10 | Sunny's line 0010A1E4 ("Doc Mitchell said you could teach me…") | Sunny/Cheyenne go outside (`VCG02SunnyTravelOutside`), allies, CGTutorial 54 |
| 20 | `VCG02SunnyPatrolTrigger` `OnTrigger SunnyREF` | bark `VCG02SunnyBark` 0010A1E6: bottles enabled (`VCG02BottleMarkerREF`), varmint rifle, objective 10 |
| — | `VCG02TargetSCRIPT` `OnHitWith` ×3 | objective 10 done, `SunnyREF.evp` → `VCG02SunnySmilesDialogueStart` → 0010A1EC |
| 25 | 0010A1E5 ("Okay, I'm in.") | objective 20, geckos enabled, `VCG02SunnyTravelToWell1` |
| 30 | end action of `VCG02SunnyTravelToWell1` | `VCG02SunnySmilesDialogueSneakStart` → 0010A1ED |
| 35 | 0010A1ED | `VCG02SunnySneakCloserToWell` (end action: stage 40), VATS tutorial |
| 40/45 | stage 40 `SayTo VCG02SunnyBark` → 0010A1E7 | objective 30 "Kill the Geckos at the well" |
| 50 | `VCG02GeckoDeathSCRIPT` (2 deaths) | objective 40, `VCG02SunnySmilesDialogueSneakEnd` → 0010A1EE |
| — | 0010A1EB ("Sure, I'll come with you.") | objective 50, `VCG02SunnyTravelToWell2/3`, settler enabled |
| — | well 2/3 gecko `OnDeath` scripts | objective 50 done, 60 "Talk to Sunny about your reward" |
| done | 0010B3DA → 0015D97F → 0015F507 → "Couldn't hurt." 0015D97E | 50 caps, `CompleteQuest VCG02`, `RewardXP 50`, VCG03 10 |

## How far the route got

Command (abridged; the full one is in the private log folder):

```
nv-viewer.exe <Data> WastelandNV --at -68250,5800,8480,180
  --run "SetStage VCG02 5" --run "set VFreeformGoodsprings.bMetSunny to 1"
  --run "StartQuest VCG02" --run "SetObjectiveCompleted VCG02 3 1"
  --run "SetObjectiveDisplayed VCG02 5 1" --run "SetStage VCG02 10"
  --run-at 40 "set VCG02.nTargetCount to 3" --run-at 40 "SetObjectiveCompleted VCG02 10 1"
  --run-at 40 "SunnyREF.evp"
  --run-at 70 "SunnyREF.MoveTo VCG02SunnyWellMarker1" (and Cheyenne, and the player)
  --run-at 80 "SetStage VCG02 30" --run-at 80 "SunnyREF.evp"
  --run-at 120 "SetStage VCG02 40" --run-at 140 "player.MoveTo VCG02SunnySneakMarkerREF"
  --run-at 150/151 "VCG02Gecko1REF.Kill", "VCG02Gecko2REF.Kill"
  --run-at 200..205 "<well 2 and 3 geckos>.Kill"
  --run-at 215 "SunnyREF.StartConversation player"
  --say "I'm in" --say "Sure, I'll come" --say "Couldn't hurt"
  --screenshot ... --wait 330
```

Result: **VCG02 completed** (`CompleteQuest VCG02`, XP +50, 50 caps,
VCG03 started) in one run. Reached by the viewer itself: Sunny leaving the
saloon and walking to her marker, the patrol trigger (stage 20) and her
bark, the bottles appearing, her dialogue packages at the saloon, the
first well and the sneak marker (0010A1EC, 0010A1ED, 0010A1EE), the
stage 40 bark, the gecko deaths (stage 50), her 5,059-unit walk toward
well 2, the reward line with its follow-ups, objectives and journal
lines. Simulated: the dialogue that starts the quest, the three bottle
hits, the walk to the first well (Sunny's and the player's), the two
package end actions (stages 30 and 40), the kills, and the player asking
for the reward (`StartConversation` instead of the player's E).

The bottle-hit path itself was checked separately on installed data (a
throwaway test, not committed): three `Runner::hit` calls with the varmint
rifle drawn count `nTargetCount` to 3, complete objective 10 and queue
Sunny's package evaluation (`GetWeaponAnimType` 5). Aiming at the bottles
in the viewer was not driven.

### Sunny walking herself (long paths, `claude/m2-long-paths`)

Re-run 2026-10-06 on the long-path branch (evidence:
[PATHING.md](PATHING.md); logs private in
`%USERPROFILE%\nv-re\work\longpaths-2026-10-06`, runs a5, b1, c2). The
same opening lines and bottle-hit lines as above, `--say "I'm in"`,
`--say "Sure, I'll come"`, `--say "Couldn't hurt"`, and **no** `MoveTo`
for Sunny or Cheyenne and no `SetStage VCG02 30`/`40` or `SunnyREF.evp`.

- **Player following** (run a5, c2): `--run-at 90,110,…,210,270,290,310
  "player.MoveTo SunnyREF"` stands in for the player walking after her.
  At 50 s Sunny plans the long way (5 nodes, 3 on attached cells) and
  walks 8,649 units, stops at the edge of the attached cells, walks on as
  the grid follows the player, and arrives at the first well:
  `VCG02SunnyTravelToWell1`'s end action sets stage 30, her sneak line
  (0010A1ED, stage 35, "Follow Sunny" completed), her walk
  `VCG02SunnySneakCloserToWell` and its end action (stage 40), the bark
  0010A1E7 and stage 45 ("Kill the Geckos at the well") follow by
  themselves. Run c2 then kills the first-well geckos (console), takes
  "Sure, I'll come with you." and Sunny walks `VCG02SunnyTravelToWell2`
  (5,059 + 3,856 units; she and Cheyenne kill the well-2 geckos) and
  `VCG02SunnyTravelToWell3` (7,636 units); the well-3 geckos are killed by
  console, the reward conversation (`StartConversation`) and "Couldn't
  hurt." complete the quest: `CompleteQuest VCG02`, XP +50, 50 caps,
  VCG03 10, and Sunny sets off on `VCG03SunnyTravelToCampfire` (10,876
  units). In run c1 the well-2 geckos killed Cheyenne first, so the
  reward took the Cheyenne-dead branch (0010B3D9).
- **Player ahead** (run b1): `--run-at 70 "player.MoveTo
  VCG02SunnyWellMarker1"` only. Sunny is then beyond the attached cells:
  out of sight, she walks the long way's nodes in game-time steps, comes
  back into sight at 106.5 s about 8,000 units out, walks the rest and
  arrives (end action, stage 30 at ~214 s), then the same chain to stage
  45.

Still simulated in these runs: the quest start, the bottle hits, the
gecko kills at wells 1 and 3, the player's following (as `MoveTo`
jumps) and the reward request. Fixed on the way: `player.MoveTo
<someone>` took the player to that person's editor cell (into the saloon
for Sunny); it now goes where the state has them (`scripts.rs`
`move_player`). Not compared with the original game.

## Fixes (one commit each)

| Commit | What blocked | Fix and provenance |
| --- | --- | --- |
| Count people who walk into a place among its people | Sunny, walking out of the saloon, was drawn but not among the place's people, so `VCG02SunnyPatrolTrigger` never saw her: no stage 20 | `bring_in_people` adds the people it brings in (and puts drawn ones back after a square rebuild); trigger occupancy follows `TriggerEntry::Update` `0062cc90` as already traced. Viewer test. |
| Draw references a script enables after their place loaded | Disabled-at-load references were never drawn; `VCG02BottleMarkerREF.Enable` showed no bottles ("isn't loaded here"), geckos enabled at stage 25 never appeared | `world::newly_enabled` (children follow their enable parent; `Enable` `005c43d0` ignores a child and queues the reference's enabling `005aa5d0`); viewer `bring_in_enabled` draws them. Generated-data test. |
| Look again for left-out references when the loaded squares change | A reference enabled while a square was still loading stayed left out | `bring_in_enabled` also looks when the place or loaded squares change. |
| Run OnDeath when KillActor kills | `KillActor` (alias `Kill`) skipped `OnDeath`, so `VCG02GeckoDeathSCRIPT` never set stage 50 | `KillActor` `005be2a0` kills through the death routine `0089d900`, the one a fatal hit takes; `OnDeath` now runs as for other deaths. Generated-data test. |
| Keep people brought in among the place's people on square loads | A dialogue package starting right after a square load found Sunny missing and talked to no one | The square rebuild keeps people `world::ai::moved_into` has in the worldspace. |
| Shots and blows meet only shown scripted objects | Disabled bottles could be shot (and counted) before they appeared | `combat::meetable` adds the enabled test (inferred from `0054c740`'s separate "has 3D" and "disabled" cases). Viewer test. |
| Add --run-at and --say | Routes could not be driven without a keyboard | Testing aids: timed console lines and topic choice by text. No behaviour change. |

## Blockers left (not fixed here)

1. **Sunny's first greeting.** With VCG02 running at stage 5 the viewer
   picks 0010B755 "Everything all right?" (VCG02, priority 75) over the
   say-once 00104E7E "Cheyenne, stay…" (VFreeformGoodsprings, priority
   55), and that line's topics lead only to a Goodbye, so the quest cannot
   be started in the viewer. Traced: `SetStage` (`005c7140` →
   `0060d510`) starts a quest (`0060c9c0`) unless it is completed without
   repeatable stages; `GetQuestRunning` `0059e320` tests flag bit 0; the
   topic's quest list (`TESTopic` +0x2c) is built in `DIAL` order by
   `006195d0`/`00905820` (append), `TESTopic::SortQuests` (Xbox PDB,
   PC `0061ba90`) sorts it by priority descending, and `0061a7d0` walks
   it in that order. Every one of those puts VCG02 first, agreeing with
   the viewer, yet the quest's own design needs "Doc Mitchell said you
   could teach me…" (in 00104E7E's topics) to be reachable. Unresolved;
   needs the original game. Dialogue owners' area.
2. ~~Long travel has no path.~~ Fixed on `claude/m2-long-paths`: the
   navmesh info map route and the attached cells' detailed path
   ([PATHING.md](PATHING.md)); Sunny's walks complete by themselves (see
   above).
3. ~~Package end actions are not run.~~ They run now (`docs/PACKAGES.md`);
   both end actions fire in the long-path runs.
4. Not driven: the player following Sunny, the player starting the
   reward conversation, gecko fights. Aiming at the bottles was driven on
   `claude/m2-physics` (injected click on bottle `0010A208`: hit,
   `OnHitWith`, the bottle knocked off the fence by its Havok push;
   [PHYSICS.md](PHYSICS.md)).
5. Seen in passing, not investigated: a sandboxing settler (00104F03)
   "eats" a Super Stimpak and its effect script applies
   `Addiction01ISFX` to the screen; the disabled `VCG02GSSettlerREF`'s
   `GameMode` asks it to `Say` every 10 s ("aren't loaded here"); the VATS
   tutorial hint repeats while CGTutorial stays at stage 70 (the data's
   own timer).

## Not compared with the original game

Everything above: trigger timing, enable timing and drawing, Sunny's
walking, dialogue choice, the reward amounts on screen.

**Next action:** settle blocker 1 in the original game (record which line
Sunny greets with after Doc's farewell), and record in the original how
far Sunny walks toward the first well while the player stays behind the
saloon (the traced rule says she waits at the edge of the attached cells).

# Goodsprings route acceptance

Live viewer runs of Goodsprings quests against the user's installed data,
with the fixes they needed. Each section is self-contained (one owner per
section). Status words: **reached** (seen in a viewer run), **fixed**
(code change with a regression test), **not compared** (not checked
against the original game).

**Message boxes on the routes (2026-10-07).** With the DLCs installed,
their start-up quests open message boxes on a new game (Dead Money's
signal, the pre-order packs' "items added to inventory", ...), as the game
does, and tutorial boxes show the first time something is met. A box left
open holds the AI and the input, so a scripted route never finishes.
VCG01 also asks for the name (the text entry) and then "Before you venture
deeper into the wasteland, you may revise your character." (Edit Name,
Rebuild Character, Finished - Travel Onward). `scripts/acceptance.ps1`
therefore runs every route with the viewer's `--answer-boxes` test aid,
which answers by rule as soon as each prompt is shown, as a player would:
a one-button box (OK) with it, a box with several buttons with the next of
`--box-answers` (button numbers in the box's own order; the routes give
`2`, "Finished - Travel Onward"), a tutorial box closed, the name entry
accepted with Enter. Each answer is logged as `--answer-boxes: ...`; a box
with several buttons and no choice left stays open and is logged as
waiting. The routes' commands and success lines are otherwise unchanged.
Add `--answer-boxes --box-answers 2` when driving these routes by hand too.

Known blocker (2026-10-07): after "Finished - Travel Onward" the revise
prompt comes back (the name entry, then the box again, forever; B29, being
fixed on its own branch), so a route that meets it waits at the second box.

## Ghost Town Gunfight (VMS16)

Branch `claude/m2-vms16-route`, 2026-10-06. Quest `VMS16` (00104EAE),
script `VMS16QuestScript` (00105D4E). Run outputs (logs, screenshots)
stay private in `%USERPROFILE%\nv-re\work\vms16-2026-10-06`.

### How it was driven

`nv-viewer <Data> WastelandNV --at X,Y,Z,180 --walk --weapon
WeapNV9mmPistol --wait 90..150 --screenshot FILE` with `--run` lines that
replay the quest's own result scripts in order (the dialogue choices were
not clicked through):

1. `set VMS16.bTrudyHelp to 1` (Trudy's recruitment INFO; left out for
   the no-help branch).
2. INFO 00105CC2 (Ringo, "All right, I'm ready"): `SetStage VMS16 65`,
   `SunnyRef.ResetHealth`, `SunnyRef.MoveTo SunnySpawnMarker`,
   `SunnyRef.AddScriptPackage SunnyTriggerGunfightDialoguePackage`.
3. INFO 00105CC5 (Sunny, "Time to look alive"):
   `SunnyRef.RemoveScriptPackage`, `GoodspringsPowderGangMarker.Enable`.
4. INFO 00105D09 (Sunny, "I'll be set up near the store"):
   `SunnyRef.AddScriptPackage SunnyTravelPackage`,
   `RingoRef.AddScriptPackage RingoTravelPackage`,
   `RingoRef.AddToFaction GoodspringsFaction 1`, `SetStage VMS16 70`,
   `set VMS16.bGunFightStart to 1`.
5. `player.ModAV Health 5000` so the standing, non-shooting automation
   player survives long enough to watch the fight (not part of the route).

The exact command (one line, run from the worktree root after
`cargo build --release` in `viewer/`; runs 8, 10 and 11):

```
viewer\target\release\nv-viewer.exe "C:\Games\Steam\steamapps\common\Fallout New Vegas\Data" WastelandNV --at -67845,3000,8400,180 --run "set VMS16.bTrudyHelp to 1" --run "SetStage VMS16 65" --run "SunnyRef.ResetHealth" --run "SunnyRef.MoveTo SunnySpawnMarker" --run "SunnyRef.AddScriptPackage SunnyTriggerGunfightDialoguePackage" --run "SunnyRef.RemoveScriptPackage" --run "GoodspringsPowderGangMarker.Enable" --run "SunnyRef.AddScriptPackage SunnyTravelPackage" --run "RingoRef.AddScriptPackage RingoTravelPackage" --run "RingoRef.AddToFaction GoodspringsFaction 1" --run "SetStage VMS16 70" --run "set VMS16.bGunFightStart to 1" --run "player.ModAV Health 5000" --screenshot <OUT>\run.png --wait 130 --walk --weapon WeapNV9mmPistol
```

The player position matters: at -67845,3000 the player is about 800
units north of `PowderGangDestination`, within the gangers' detection
range as they arrive, and the settlers at their markers see the gangers.
From PowerShell, pass the arguments as an array to `Start-Process` (or
the call operator) so each `--run` line stays one argument.

Note: INFO 00105D09 sets `bGunFightStart` itself, so the quest script's
own stage-70 block (`GSJoeCobbRef.AddScriptPackage GSPGTravelPackage`,
the settlers' flee packages) does not run on this path; the gangers and
Joe Cobb take `GSPGTravelPackage` from their own package lists, and the
settlers' own lists carry `GoodspringsFleePackage`/the guard packages
(`docs/PACKAGES.md`). Runs that set only stage 70 exercise the quest
script's block instead; both reach stage 100.

### How far it got

**Reached stage 100** with Trudy's help (runs 5, 7, 8) and the player
standing at -67845,3000 (north of `PowderGangDestination`): stage 70's
moves (Joe Cobb, settlers 01–04, Trudy), all six gangers (Joe Cobb,
GSPG01/02/03/05/06; GSPG04 stays initially disabled with no parent) travel
in on `GSPGTravelPackage`; settlers, Trudy, Sunny and Easy Pete fight them
(Easy Pete helps the player against the gangers); each `OnDeath`
(`GoodspringsPowderGangerScript`, `GoodspringsJoeCobbScript`) counts, and
the sixth sets stage 100: "Completed: Defeat the Powder Gangers", XP +50,
Goodsprings fame and Powder Ganger infamy (Liked / Shunned). After stage
100 Ringo's `GSRingoAfterVMS16DialoguePackage` brings him to the player
and he starts talking (run 8, after fix 2). He opens with his first
meeting ("That's close enough"): this route never meets him, and that
say-once line comes before the stage-100 thanks in his `GREETING` list,
as in the game (B26 on the former task list). Met first, he says "I owe you a
huge favor" (00105D0D).

Without Trudy's help (run 6) the gangers fight the player and Easy Pete;
the automation player does not shoot back and dies; settlers 02–04 do not
take part, as their packages say.

### Fixes

1. **Gangers following an enable parent never appeared** (commit
   `2e178c7`). GSPG01/02/03 have `XESP` → `GoodspringsPowderGangMarker`
   (00105D4C, persistent, initially disabled). A square loaded before the
   marker's `Enable` left them out ("disabled when the cell loads") and
   nothing brought them in later; only gangers in squares that happened to
   finish loading after the `Enable` came. Squares now record the people
   they left out as disabled (`world::ai::disabled_people_in_square`), and
   `bring_in_people` spawns those a script has since enabled, themselves or
   through their parent (`world::ai::enabled_since_load`; `Enable`
   005c43d0 loads the reference's 3D, `docs/SCRIPTS_RUNTIME.md`). People
   already on screen are not drawn a second time when a square reloads.
   Test: `crates/world/tests/ref_scripts.rs`
   `people_left_out_as_disabled_come_in_once_enabled` (generated square
   0,1 with an enable-parent follower and a persistent disabled person).
2. **People brought in after their place loaded were not talkers**
   (commit `a927d6a`). Anyone spawned by `bring_in_people` (script
   `MoveTo`, doors, fix 1) was missing from `Talkers`, so the player could
   not talk to, hit or V.A.T.S.-target them, and a script's talk request
   failed ("A script has 00104C7D talk to the player, but they aren't
   loaded here" for Ringo after stage 100; Joe Cobb, moved out of the
   saloon interior at stage 70, was not a V.A.T.S. target). They are added
   now and survive the squares' talker rebuild while on screen. Test:
   `viewer/src/exterior.rs` `people_brought_in_stay_talkers_when_squares_change`.
   Run 9 (`--vats 4` next to Joe Cobb) now lists him as a V.A.T.S. target.

3. **Gangers and settlers fought with their fists after the NPC-combat
   merge.** The merged arsenal (`world::npc_combat::arsenal`, `009993c0`)
   rates only weapons carried with ammunition, read from
   `GameState::inventory`. For a holder never stocked that is the
   record's direct `CNTO` items only; leveled lists are picked by
   `GameState::stock`. The Powder Gangers, Joe Cobb, Sunny and the
   settlers carry their guns only through `WithAmmoNV…Loot` lists
   (`GSPGAAM2` 00104C76: `WithAmmoNVSingleShotgunLoot` 000F9DCF), so the
   first merged run (run 10) had everyone but Trudy and Easy Pete "take up
   their fists" (stage 100 still came, through Trudy and Easy Pete).
   `choose_weapon` now stocks the fighter before rating (the game's actor
   already holds its leveled items before a fight; when exactly it picks
   them is not traced here). Test: `crates/world/tests/npc_combat.rs`
   `a_gunman_fights_with_the_gun_and_rounds_his_leveled_list_gives`
   (generated `TestRifleWithAmmo` use-all list). Run 11 on the merged
   build: gangers take up varmint rifles, shotguns, revolvers, a bat and a
   cleaver; all six die; stage 100 with XP +50; Ringo then talks to the
   player.

4. **The gang killed itself after realistic NPC spread** (branch
   `claude/m2-friendly-fire`). Stray hits went through the viewer's
   untraced "whoever is hurt fights back" rule, so gangers hit by each
   other's spread fought each other ("GSJoeCobbRef was killed by
   GSPG06Ref", "GSPG05Ref was killed by GSPG06Ref"). Replaced by the
   game's `Actor::AttackedBy` (`008987f0`, `docs/NPC_COMBAT.md` "Hits from
   allies, friends and strays"): `GoodspringsPowderGangFaction` is its own
   ally (`XNAM` reaction 2), allies and friends tolerate hits, and a stray
   from someone fighting somebody else starts no fight. Tests:
   `crates/world/tests/friendly_fire.rs`. Re-run with the exact command
   above and `--wait 330` (private outputs
   `%USERPROFILE%\nv-re\work\friendlyfire-2026-10-06`, runs 1–2): no
   Powder Ganger took on, turned to or helped against another in either
   run; stray pellets still hurt allies (run 2: 10 ganger-on-ganger stray
   hits, one fatal, "GSPG05Ref was killed by GSPG02Ref": a shotgun
   pellet meant for the player struck GSPG05, who was clubbing the player
   at 1.8 health). Run 2 **reached stage 100** at about 197 s (XP +50).
   Run 1 did **not**: the townspeople (Sunny, Trudy, Ringo, Cheyenne,
   settlers 01 and 03) died, four gangers died (Joe Cobb, GSPG05, GSPG03,
   GSPG01), GSPG02 and GSPG06 survived and went back to their packages;
   the automation player doesn't shoot, so nothing could finish them.
   Ringo's stray hit Cheyenne, who was already fighting and isn't allied
   to his factions, so she took him on (`0097f580`) and Trudy then killed
   her: traced behaviour on this data, **not compared**.

### Investigated, not changed

- Target after a kill: when a fighter's target dies the viewer drops it
  out of combat (`world::combat::hurt`), and its `noticed` set means
  enemies noticed meanwhile never "rise" again, so it walks back to its
  package. `008ff350` processes a rise in combat too (it marks the
  detection entry's +0x1f flag and queues the start-combat event), and the
  game keeps fighting the combat group's other targets
  (`CombatGroup::GetBestTarget`, Xbox PDB; ranking not traced). Not
  changed: no run showed it blocking the gunfight (the gangers re-notice
  enemies as they come into sight), and the target choice is untraced.
- Settler 04's `GSSettlerAmbushPackage` takes him into the Prospector
  Saloon: his editor location is in `GSProspectorSaloonInterior`, so this
  follows the data.

### Not compared with the original game / remaining gaps

- Nothing here is compared with the original game: arrival timing, where
  the gang stops, who sees whom (the viewer's line of sight is from 60
  units above the feet), hit rates (gangers hit the player with nearly
  every shot from 3000 units in run 1), fame/infamy titles.
- The dialogue choices (recruitment INFOs, Ringo's and Sunny's lines) were
  replayed as console lines, not chosen in the dialogue menu.
- The player's own fighting was not driven (the automation cannot aim);
  the gangers were killed by the townspeople.
- `GetShouldAttack`, `OnStartCombat`, `SetUnconscious`, NPC weapon
  choice and finite ammo/reloads came from the NPC-combat batch
  (`docs/NPC_COMBAT.md`); run 11 exercised weapon choice and reloads but
  not the VMS16b branch (`GSJoeCobbTriggerScript`, `OnStartCombat
  player`).
- In run 10 Trudy killed Easy Pete (friendly fire during the melee); the
  hit reaction is now traced (fix 4); whether the game's gunfight ends with
  the town or the gang winning when the player doesn't fight is not
  compared.
- The guard packages' intruder scan (`docs/PACKAGES.md`).
- Save/reload during the fight was not tried.
