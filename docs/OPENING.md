# Opening foundations: shared handoff

2026-10-04 overnight session: PR #5 save-failure isolation merged as
`637b28d`; PR #6 native appearance reconciliation merged as `843926d`.
The verified PR #6 binary is in play/app. Camera continuation PR #7 is
awaiting amended viewer CI; its local checks, installed-clip comparisons and
live/restart reload evidence are in [PERSISTENCE.md](PERSISTENCE.md).
Active branch `codex/m1-reload-dialogue-cleanup` addresses old dialogue and
voices surviving reload; full conversation continuation remains missing.
Current checks/processes and next action belong to that handoff and
[MILESTONES.md](MILESTONES.md).

The bounded gamedb experiment is complete; full indexing is deferred
([RESEARCH_WORKFLOW.md](RESEARCH_WORKFLOW.md)). Native gaze cap constructors
and face-menu category navigation are traced, but no gaze solver or face
editor is implemented from them. See [OPENING_LOOK_IK.md](OPENING_LOOK_IK.md)
and [FACE_CREATION.md](FACE_CREATION.md). Original-game input automation
remains blocked at the main menu; the original and isolated test viewers
have exited. The older batch evidence below is historical.

2026-10-03. Current owner: Codex; opening camera, attention, HUD and activation fixes.
Use with MILESTONES.md.

## Current reported defects (thirteenth batch published; M1 still open)

The user reports a brief mouse-input leak during lying-to-sitting that leaves
heading changed, Doc looking away, incorrect interaction UI/timing, and an
unusable vigor tester. Do not treat the eleventh batch as acceptance.

- Input: the initial stage runs after mouse input in the Update chain. Input
  now waits for the player to be ready and the pending start stage to run,
  then obeys the traced package lock. Regression holds mouse motion across
  600 frames and package replacements, and checks release. This establishes
  the startup gap; reproducing the user's exact transition is still needed.
- Activation: VCG01VigorTester (00104C0A), reference00104C13, has zero OBND
  despite its cabinet geometry. Exporting actual placed model bounds fixes
  the missed target. This is a bounds-based picking correction, not yet a
  complete port of the native physics picking path. The authored activation
  still requires stage60 and objective30 displayed, then advances to65.
- HUD: native00771700 selects movement-disabled mode12 mask0x1514, hiding
  Info along with HP/AP, compass and reticle. Normal rollover suppression
  clears bit0x200. Info uses the existing XML justify_center_hotrect child,
  its native `_x` field and the native target-text template; the old Bevy
  interaction text overlay is removed. Basic actions and ownership colors are
  wired; advanced item rows, furniture/terminal variants and controller glyphs
  remain incomplete.
- Doc: the intro uses SayTo Player, not a dialogue package. The viewer lacks
  native head/eye tracking over the chair idle. Trace008a3100 selects a
  target and008a3b70 supplies its point to the actor's LookIK controller;
  its target transform and smoothing limits are partly established. The
  selected node is now traced to BPNI (Bip01 Head); final rotation math
  remains unresolved, with no guessed
  look-at applied. See [OPENING_LOOK_IK.md](OPENING_LOOK_IK.md).

Checks: 910 core tests and 82 viewer tests pass; the corrected real-XML Info
layout additionally passed all 126 UI tests. Both workspaces passed clippy,
format checks and release builds. Published to nv-rs-play/app, SHA256
`D2860A56D7D43BBDC19902A3CC77950E0340FB3CB21CDFBD911F9AE6D499E136`.

Live verification used an isolated copy of the user's stage55 save, with only
the copied player's position moved to the tester. Entering its trigger ran
stage60, Doc spoke INFO001074A2, its result enabled objective30, and pressing
E opened the original SPECIAL scene/menu. The Info prompt hid during speech
and returned afterward. Log: nv-re/work/codex-m1/interaction-layout2.log.
This verifies that segment, not the full opening or exact camera transition.
Desktop automation recovered after selecting a fresh test window.

An initial same-cell reload attempt retained the old trigger occupancy and
did not emit OnTriggerEnter. Restarting outside the trigger before loading
established the live route above. Follow-up now clears CellScripts when
replacing the saved GameState, including occupancy and the old seat. Its
regression enters the volume before stage55, loads stage55 in the same cell,
and verifies stage60 runs. All 83 viewer tests, clippy, formatting and release
build pass. Live same-cell F9 at the tester now runs Doc's instruction and E
opens SPECIAL (interaction-reload.log). Thirteenth batch installed in play/app,
matching the built viewer SHA256
`30A5413AD0D737B45E7E3ACF118B764BA897EE091972F88D48DB734B4B92ED07`.
Raw records, assembly, copied saves
and decompilation remain outside the repository in nv-re/work/codex-m1 and
nv-re/decomp/codex-m1.

## B14: Doc's chair, the menu camera and the help-up

Research on `claude/b14-opening` and `claude/b14-navmesh-obstacles`; the
fix on `claude/b14-doc-seated` (2026-10-07). Private logs and window
captures in `%USERPROFILE%\nv-re\work\b14` and `...\b14-seated`.

### Doc seated at the start (fixed)

The maintainer, who played the original: Doc is already sitting in the
chair beside the bed when the view clears, and gets up only to help the
player stand. The earlier notes here said no start-seated path exists and
the game walks him round the chair under the fade. **That was wrong**: it
rested on "the instant sit `0088d2f0` is the player's". Its second caller,
`00972d30`, is the game's pass over every loaded actor once a cell has
loaded, and it seats Doc before the first frame.

The pass (FalloutNV.exe 1.4.0.525, decompiled):
- Callers: the interior and exterior cell loads (`00453dc0`, `00454450`,
  `004512c0`), and the player being moved (`0093cdf0`). `00975d10` also
  calls it.
- Who it covers: each actor in the high process list (`005be5c0(0)`,
  `00968670`) that isn't disabled and isn't dead (`004938e0`). It also
  skips actors where `008a3b30` holds (read here as "in combat") and
  anyone already in furniture (sit state 0 only), and it needs the
  process level to be high (`0045cd60` = 0).
- Which furniture: the current package's target (`00881650`), else its
  location's reference (`00886080` → `0067f390`, "near a reference";
  linked reference `00569b80`), else the process's fallback (+0x514).
  It must be `FURN` (`00568680`, type 0x27) in the actor's own cell
  (`00575ca0`). A patrol package uses its current point. Eat and sleep
  packages search the cell for a free chair or bed instead.
- Which marker: the first one the `MNAM` allows that nobody occupies
  (`005682c0` with 1, so only the occupied bits, extra 0x12, count, not
  reservations). `00568500` places it into the process's marker copy.
- The instant sit `0088d2f0`: heading = marker heading; position = marker
  + the marker number's `fFurnitureMarkerNNDelta*`, turned by the heading
  (`00509920`); SetPos; the marker is marked occupied (`00568020`);
  heading += `HeadingDelta` (`00931d30`); sit state 1, then the seated
  loop is loaded (+0x4d8). If there is none, it logs "%s went to sit at %s
  and had no animation" and sets state 0; otherwise state 4.

Doc's first package `VCG01DocMitchellFirstPosition` (`00104C1D`, travel,
location: the chair `001059B0`, `Chair01F`, MNAM 0x40000004) passes. He is
seated on marker 14 (front) at about 2285,2265,7360 before he ever walks.
His placement about 24 units behind the chair's back, and the chair's
collision in the way, therefore never matter. (The obstacle manager
findings stay true: `Chair01F` isn't an obstacle, only forms with flag
0x02000000 are, `00564bc0` → `00401210`.)

Implemented: `world::furniture::seat_on_load` and `same_cell`. In the
viewer, `sitting::seat_on_load` runs at an actor's first package choice
after their cell or square loads (`Walker::settle`), using the package's
target, then its "near a reference" location. Regression test:
`world/tests/sitting.rs`
`a_cell_load_seats_someone_at_once_and_others_pass_sitters`.

Verified live in the release viewer (`--new-game --no-movies
--answer-boxes`, window captures read back):
- Doc is seated in the chair beside the bed from the first frame. He
  stays seated through the wake-up, the name prompt and the stitches
  conversation.
- At INFO `00104BFA` (68 s) he gets up with
  `SpecialIdle_NVDocChairFrontExit.kf` beside the player's
  `LooseVCG01PlayerStandup`. VCG01's stage `ResetAI` follows at 78 s.
- Easy Pete (`WastelandNV` at 9:00, `EasyPeteChairPackage8x4`) is seated
  on his porch chair `0010634A` as the square loads.

Not done:
- The pass also runs over actors that were already loaded when a later
  load happens (an exterior grid shift, or the player moved); here it
  covers only those who have just loaded.
- It runs at each actor's own first package choice, not before everyone's
  AI. So on the Goodsprings porch, 00104F0A (on a sandbox walk) took
  Pete's chair as its target just before he was seated in it.
- Not carried out: the patrol, linked-reference, fallback (+0x514) and
  eat/sleep search branches, and what the game does when no marker is
  free (`006d6f80`).
- The help-up's timing against the player's stand-up (the
  `NPCDocPlyStand` text key at 7.067 s) hasn't been compared with the
  original.

### People in furniture don't block others (furniture collisions)

Traced while checking why NPCs get caught on furniture:
- `00920d00` (the sit-state setter) sets controller flag 0x08000000 (+0x414,
  via `00629670`) and actor vfunc +0x2e0(0.1) when entering want-to-sit or
  want-to-sleep, or going 1 → 4 or 6 → 9. It does so only if `00920fd0`
  holds: a sit marker (10–20 or 26) or a bed marker (under 10), on
  furniture whose 3D has collision objects (`004b66d0`). Want-to-stand,
  want-to-wake and normal clear the flag.
- The controller's contact callback `00c711d0` zeroes a contact with a
  character (layer 30) whose controller has 0x08000000.
- So from sitting down until getting up begins, a sitter doesn't block the
  player or anyone else. `Sitter::others_pass_through`; the viewer leaves
  those people out of the controllers' people (`ai::passed_through`, also
  used by `walk::people`).
- While someone is in a sit state, `MobileObject::Move` (`0092f260`)
  doesn't integrate their controller: the entry and exit animations carry
  them through the furniture. The viewer already did this (no controller
  in furniture).
- Not modelled: sitters restored by a load or by the 3D-attach path
  (`00925700`, `00927d70`) don't go through `00920d00`, so in the game they
  may lack the flag; here they get it too.

### The camera during the name prompt

Seen live (window captures each second, name menu held open): while the
`TextEditMenu` is up the bedsit camera holds still (the viewer stops the
player's camera animation in menus, `player_idle::animate`, and looking is
blocked by the player's script package, `005cc7a0`); after OK it carries
on with the bedsit loop. A snap wasn't reproduced in these captures, and
the game's camera behaviour in menu mode (`fMenuModeAnimBlend`, whether the
opening's camera idle pauses) wasn't traced. Not changed.

### The help-up

Now reached (see above): Doc's help-up is his chair exit
`SpecialIdle_NVDocChairFrontExit.kf` (the tree's exit for marker 14 during
VCG01 stages 10-50; `NPCDocPlyStand` text key at 7.067 s), timed with the
player's `LooseVCG01PlayerStandup` (VCG01PlayerSection2, added by the
result script of INFO `00104BF9`). Its timing against the original isn't
compared.

#### Help-up timing (`claude/b14-helpup`, 2026-10-08)

Play build 28: Doc helps the player up at the wrong time. Private notes,
dumps and the live log in `%USERPROFILE%\nv-re\work\b14-helpup`.

The data:
- INFO `00104BF9` ("Well, I got most of it right…", voice 3.61 s), end
  script: `player.addscriptpackage VCG01PlayerSection2` (`001055C4`,
  begin idle `LooseVCG01PlayerStandup` `001055C2`), then `SayTo` the next
  line.
- INFO `00104BFA` ("Okay. No sense keeping you in bed anymore…", voice
  7.03 s), end script: `SetStage VCG01 45`, then `DocMitchellREF.evp`
  with the comment "Make Doc stand up before you do". Stage 45: a 3 s
  timer ("delay while the player stands") to stage 50.
- Doc's `VCG01DocMitchellBedsideStandingPackage` (`00107238`, travel to
  `GSDocMitchellBesideMarker` `00107235`) has `GetStage VCG01 >= 40`, so
  any package check from stage 40 on gets him up. Stage 40 is set by
  VCG01SCRIPT's `MenuMode 1036` (the race menu).
- His exit `NVchairStandDoc` (`00169CDE`): `SpecialIdle_NVDocChairFrontExit.kf`,
  11.97 s. Sampled: he rises 1.0–2.5 s, bends down to the player
  6–7.5 s (`Bip01 NonAccum` z 67 → 45; `Sound: NPCDocPlyStand` 7.07 s,
  `0017469A`), pulls back up 8.5–11 s.
- The player's `SpecialIdle_NVCG01PlayerStandup.kf`, 11.13 s: the
  `Camera1st` track leans forward 1.5–4 s and rises 4–6.5 s (z −29 → +3).
  So the two line up when Doc's exit starts about 2–4.5 s before the
  player's stand-up.

What the viewer did wrong (fixed): getting up waited for the seated idle
playing to end. Doc's `SitChairRelaxIdleA` (16.1 s, from 52.1 s) held his
exit to 68.2 s although `evp` had changed his package at about 64 s.
In the game the stand update `00921e80` (cases 4/9) waits only while
`00498f80` holds: the animation's current idle (+0x124) or queued one
(+0x128) has no sequence yet, or its sequence's state (+0x44) is 2
(easing in) or 5 (transition destination). An idle playing at full weight
doesn't hold it; the exit replaces it (`00497ca0` stops it with its
blend-out, `004994f0`). Implemented: `world::animation::Player::idle_starting`,
`sitting::special_idle_starting` and `replace_seated_idle`. Regression
tests: `world/tests/furniture_blend.rs`
`a_seated_idle_holds_getting_up_only_while_it_blends_in`, viewer
`actors::tests::a_tree_idle_counts_as_starting_only_until_it_has_blended_in`.

Live (release viewer, `--new-game --no-movies --answer-boxes --box-answers 2`,
log only): `00104BF9` 52.8 s, player's stand-up from about 57.1 s,
`00104BFA` 57.1 s, Doc `Sitting -> Want to stand` with the front exit at
64.4 s (was 68.2 s), `Want to stand -> Normal` 76.4 s.

**Not resolved:** even so Doc's exit starts about 7 s after the player's
stand-up, so his bend (≈ 70–72 s) comes after the player is up (≈ 61–64 s).
By the data alone the game would do the same unless something checks
Doc's packages between stage 40 and the end of `00104BFA` (his 20 s
package timer or hour check, `008da670`, could; whether `SayTo` or the
race menu's close forces one wasn't found: `SayTo` `005c9100` and the
process's say `008dbe30` don't call it in what was read). The end of a
non-menu line (where its end script runs and whether a package check
follows) wasn't traced. Next: trace the speech-finished path that runs an
INFO's end script for `SayTo`, or record the original from the race
menu's close to the help-up.


#### Seated people's timed package checks (`claude/b14-timer`, 2026-10-08)

`008da670` checks someone's packages in sit states 0, 4 (sitting) and 9
(sleeping): forced, with none, on the 20 s timer or a new game hour. The
viewer only honoured forced checks for a settled sitter, so a seated Doc
waited for `DocMitchellREF.evp` at the end of `00104BFA`. Now his clock runs
seated too (`ai::rethink_queued_package_before_furniture`). Live (`--new-game
--no-movies --answer-boxes --box-answers 2`, `%USERPROFILE%\nv-re\work\b14-timer`):
stage 40 at ~52.7 s (the race menu's `MenuMode 1036`, run once as the menu
isn't drawn), Doc `Sitting -> Want to stand` at 61.3 s (was 64.4 s), on his
timer. Still about 5 s behind the player's stand-up (~57 s, the on-begin
idle of `VCG01PlayerSection2`, which `AddScriptPackage` `005cc4f0` sets at
once). Ruled out: `00762160` (the dialogue menu's close) and
`AddScriptPackage` deferring the player's package. Callers of the forced
check `008a6ce0` not yet read: `00573f40`, `005c9530`, `00886360`,
`008d0e80` (via `00925700`), `008e0f80` (a process update, forced under a
condition near its line 152), `00967da0`. Also open: whether the viewer's
package clocks count menu time (the game's don't).


#### Help-up timing and daylight, from the original (2026-10-08)

The maintainer's two recordings of the original's opening, aligned with
Doc's voice files by their loudness envelopes (correlation 0.95):
"Well, I got most of it right" (`00104BF9`) starts the moment the face
menu closes, and Doc's chair exit starts with it (rising by 0.6 s, bending
down at ≈ 6.4 s); the player's stand-up (on-begin idle of
`VCG01PlayerSection2`, set at that line's end) rises at ≈ 8–10 s, so his
help lands as the player stands. Doc's standing package needs
`GetStage VCG01 >= 40`, set by `VCG01SCRIPT`'s `MenuMode 1036` while the
menu is open. Ruled out as the trigger: `SetStage` (`005c7140` →
`0060d510`, runs the stage only), the line's end (`00935f60`: the end
script, then process vfunc +0x88 `008d8dc0`, which clears the "saying"
byte +0x459), the menu's finish (`007ada40`), the 20 s timer and the hour
(both pause in menu mode: `0086e650` sets `011dea2b` from `00702360`/
`007023a0`, interface +0xc != 1). `EvaluatePackage` (`008a6ce0`) itself
only flags actor +0x145 and sets the process's last hour to the hour − 1
(`00693d50`), so the hour test fires at the next update; many callers
use the same trick, none traced to the menu's close. Implemented from the
recording, labelled: `GameState::evaluate_everyone`, set after the face
menu's MenuMode blocks; every walker's packages are looked at once. Live:
Doc `Sitting -> Want to stand` at 52.7 s, with `00104BF9` (was 61.3 s).

`SayTo` lines now ask for their speaker idle (`005c9100` → `008dbe30` →
`008a20d0`, as greetings): "Whoa, easy there" plays `VCG01DocWhoaThere`
(Doc leaning forward in his chair as the player sits up, as recorded),
"How'd I do?" `VCG01DocGiveMirror`.

Daylight: `VCG00` stage 0 (source "DEMO ONLY") sets `GameHour` to 23;
nothing after changes it, and `VCG01` stage 0 is set only by `VCG00`'s
cleanup, so the game does run it. The recordings show daylight; the
maintainer's decision: it's a leftover, the opening is meant in daylight.
The viewer keeps the hour through that stage (`GameHour`'s own value, 12).

## The whole opening, step by step (`claude/opening-trace`, 2026-10-08)

Every step of the new game from the first frame to Doc's farewell, from the
quests' stage scripts (`VCG00`, `VCG01`, `VCG01SCRIPT`), the INFOs of topic
`VCG01Intro` (`00104BEE`..`00104BFC`), the packages `VCG01PlayerSection0/1/2`
and Doc's `VCG01DocMitchell*` packages (all `FalloutNV.esm`), and runs of
the release viewer (`--new-game --no-movies --answer-boxes --box-answers 2
--walk`; `--stage VCG01 75` and `85` for the later steps; logs in
`%USERPROFILE%\nv-re\work\opening-trace`). Without `--walk` a
`--screenshot` run leaves the player in fly mode (`Player::new(false)`):
the first-person camera idles then never advance, and only "Wakeup" is
accepted, which is why earlier logs showed no sit-up, bed-sit or stand-up
idles. Use `--walk` for any log about the opening camera.

Status: **matches** (viewer follows the data, checked in a run), **fixed
here**, **differs** (why, what is needed), **not run** (traced from data
only).

| Step | What the game does (evidence) | Viewer |
|---|---|---|
| New game | `VCG00` stage 0: `PlayBink FNVIntro.bik`, white/black screen effects, `GameHour` 23 (leftover, see above), `SetStage VCG00 90` | movie skipped by `--no-movies`; daylight by decision |
| Blackout | `VCG00` 90: `MQ08AllBlackedOutISFX`, timer 3 s; 95 (empty); 100 cleanup: removes the effect, restores controls, `SetStage VCG01 0` | **fixed here** (was 0.01 s, see below), now 3.0 s |
| Wake | `VCG01` 0: Victor disabled, timer 0.2 s, only looking allowed, `AddScriptPackage VCG01PlayerSection0` (begin idle `LooseVCG01PlayerWakeup`, looping), white fade effect, Pip-Boy removed | matches (controls log, "Player camera idle: LooseVCG01PlayerWakeup") |
| Doc's first line | 0 -> 1 (`fTimer` 2.8 s) -> 3: `SayTo` "You're awake" (`00104BEE`); 3.0 s after stage 0 (recording: voice at 2.94 s) | **fixed here** (was 0.2 s after stage 0) |
| Sit up | end of that line `SetStage 5`: `Section1` (begin idle `LooseVCG01PlayerSitup`), strain effect, timer 3.25 s | matches (Situp requested at stage 5) |
| Bed-sit | 7: `Section1` again, "triggers the OnChange animation" (`LooseVCG01PlayerBedsit`, looping), timer 0 | matches (Bedsit accepted). The same call also queues Situp again; only the idle gate refuses it while Bedsit blends in (`world::animation::Player::request_special_idle`), see "Open" |
| "Whoa, easy there" | 8: `SayTo` `00104BEF`; its speaker idle `VCG01DocWhoaThere` | matches (idle requested) |
| Relax, name question | `00104BF0`, `00104BF1` (end `SetStage 10`) | matches |
| Name | 10: `GetPlayerName` (name menu), `bRunTimer`, timer 1 s; 15 after the timer: `SayTo` `00104BF2` | matches (name menu, then the line) |
| Doc introduces himself | `00104BF2` -> `00104BF3` ("I'm Doc Mitchell", idle `SitChairTalkPlayerD`, end `SetStage 25`) -> `00104BF6` (`SitChairTalkPlayerA`) -> `00104BF7` (end `SetStage 30`) | matches |
| Sex choice | `VCG01SCRIPT` shows `VCG01ChooseSexMessage` only at stage 17 (`bChooseSex`); nothing in `FalloutNV.esm` ever sets stage 17 (no `SetStage VCG01 17` text anywhere; the race menu takes the choice) | matches (dead branch, nothing to do) |
| Mirror | 30: `PlayIdle VCG01DocGiveMirror`, 3 s; 35: `SayTo` "How'd I do?" (`00104BF8`, end `SetStage 36`) | matches |
| Face menu | 36: `ShowRaceMenu`; `VCG01SCRIPT`'s `MenuMode 1036` sets stage 40 while it is open | differs: the menu isn't built (B15); the viewer auto-accepts and runs the `MenuMode` block once |
| Help-up | 40: `SayTo` `00104BF9`; Doc's exit starts with it, the player's `Section2` (`LooseVCG01PlayerStandup`) at its end, `00104BFA` (end `SetStage 45`, `evp`); 45: 3 s | matches the recording (see the sections above) |
| Walk to the tester | 50: `SayTo` `00104BFB`, `RemoveScriptPackage` (view released); `00104BFC` end `SetStage 55`; 55: controls, `AutoDisplayObjectives`, autosave, objective 10, tutorial, Doc `ResetAI` (leaves the chair for `VCG01DocMitchellTravelToPlayerAtTester`) | matches (log: controls, autosave, objective, Doc walks 574 units) |
| Tester, SPECIAL, reactions | 60 reached; 65 tester used (objective 30, `SetDestroyed`), 1 s; 70 Doc's reaction (`VCG01DocReactions`); 75 -> 76 -> 79 | 60 -> SPECIAL verified live in the thirteenth batch; 75 -> 79 run today: matches |
| Psych exam | 79: `Look Player`, `SayTo` `VCG01DocPsychIntro` (`001055AD`, `001055AE`, end `SetStage 80`); 80: `StopLook`, `evp`, Doc walks to the couch (`VCG01DocMitchellTravelToExamSpot`, stage >= 80) and sits, objective 40 | matches (run today: Doc sits at about 20 s) |
| Tag skills | 85: 1 s; 90: `SetTagSkills 3 1` (menu), timer 1 s | opens ("Tag skills menu: 3 to tag" in a `--stage VCG01 85` run); the rest wasn't run (the menu needs input) |
| Traits, farewell, leaving | 95 `SayTo` `VCG01DocTraitIntro`; 98 -> 100 -> 102 `ShowTraitMenu`; 105 `VCG01DocFarewellTransition`; 110 Doc `evp` to the exit (`VCG01DocMitchellTravelToExit`), controls, objective 50; 115 conversation (`VCG01DocMitchellFarewellDialogueStart`); 200: `GameHour` 8, `StopQuest VCG01`, `StartQuest VMQ01`, `VCG04` | not run in the viewer (needs the tag and trait menus driven); not compared with the original |

### Found and fixed: variables set by a nested stage script were lost

`VCG01SCRIPT` sets stages from its own `GameMode` block (`setstage VCG01 1`).
A stage's result script runs with the quest's own variables (stage 1:
`set VCG01.fTimer to 2.8`). The runner takes the quest's variables out of
the state while its block runs and puts the block's copy back at the end,
so what the nested script set was overwritten with the block's old
`fTimer`. The game keeps one copy of a script's variables. Effects in the
opening: `VCG00` 90 -> 100 (the 3 s blackout) took 0.01 s; `VCG01` 0 -> 1
-> 3 took 0.23 s instead of 3.0 s, so Doc spoke 0.2 s into the scene
instead of 3.0 s after the cleanup (the recording has his voice at 2.94 s,
taking the recording to start at the cleanup: labelled assumption); the
timers of later stages set from the same block (`90`: 1 s before the trait
intro, `102`: 1 s after the trait menu) were lost the same way. Fix:
`Runner::call` hands the block's variables over for the commands that run
other scripts (`SetStage`, `StartQuest`, `StopQuest`) and takes them back
(`crates/world/src/scripting.rs`). Regression: `world/tests/scripting.rs`
`a_stage_set_from_the_quests_script_keeps_the_variables_it_sets` (fails
without the fix, `fTimer` 0). Live (release viewer, `--new-game
--no-movies --walk`): `VCG00` 95 at 3.02 s of script time, `VCG01` 0 at
3.03, 1 at 3.24, 3 at 6.07, Doc speaks at 6.9 s of the viewer's clock
(0.9 s of it the pack-message boxes, when script time stands still); every
later gap is unchanged. The black start is the script's blackout after the
movie.

### Open

- The first-person idle gate refuses the second `Situp` of
  `AddScriptPackage VCG01PlayerSection1` only because `Bedsit` is still
  blending in; whether the game starts a package's begin action again when
  the same package is added again isn't traced (the quest script's own
  comment says the second call "triggers the OnChange animation").
- Stages 105..200 (farewell, leaving the house) and the tag/trait menus
  were not driven; `--box-answers` doesn't answer `CharGenMenu`.
- The recordings only cover up to the help-up; later steps have no
  original footage to compare with.
- Whether the original shows the 3 s blackout before the white fade the
  same way (the recording starts after it) isn't checked.

## The character-revision prompt (B29)

Branch `claude/b29-revise-loop`, 2026-10-07. Playtest bug: the "revise your
character" prompt came again and again after editing, so the player could
never leave. Private probes, logs and window captures in
`%USERPROFILE%\nv-re\work\b29`.

### The data

The prompt is `VCG04` ("Player Character Revision", `0011649E`), started by
`VCG01` stage 200 (Doc's farewell timer), not by Doc's door.
`VCG04QuestScript` waits until the player is in `WastelandNV` east of
x -59433 or north of y 14752 or south of y -21205 (well out of
Goodsprings), then `VCG04ActivatorRef.MoveTo player`,
`VCG04ActivatorRef.Activate player 1`, `Done` 1. The activator
(`001164A3`, persistent, placed at -73702, 1360 west of Goodsprings) shows
`VCG04Message` in `OnActivate`; its `GameMode` reads `GetButtonPressed`:
0 `GetPlayerName` and ask again, 1 level 1, perks and traits removed,
`ShowRaceMenu`, then over the next runs `SetSPECIALPoints 40`,
`SetTagSkills 3`, `ShowTraitMenu`, and ask again; 2 `RewardXP` (what was
earned, if rebuilt) and `StopQuest VCG04`. So after an edit the game asks
once more by design; "Finished - Travel Onward" ends it.

### Traced (FalloutNV.exe 1.4.0.525)

| Address | What | Used for |
| --- | --- | --- |
| `005b4630` | `ShowMessage`: after the text is made, `005b4940` drops any waiting button (`0118c684` = -1); the owner `011cac64` = the reference the script runs on (`0084e3a0`: form ID), or the script when there is none or it is temporary (flag 0x4000, `004077c0`); box callback `005b4a70` | `Runner::message`, `message_owner` |
| `005b4a70` | Box callback: the pressed button (interface +0xe4, `00703fa0`) into `0118c684` | viewer sets `GameState::button` |
| `005b4a80` | `GetButtonPressed`: the button only when the asker's owner equals `011cac64`, then -1 and owner 0; anyone else -1, button kept | `GetButtonPressed` |
| `005c71c0`, `005c7220`, `0060c9c0` | `StartQuest`/`StopQuest`: the quest's running bit and "changed", nothing else (no variables reset, no other quest run) | unchanged |
| `005ccb20` | `MoveTo` on a reference (not the player): its position set to the target's, then `00573800` | `RefScripts::place_moved` |
| `00573800`, `005875a0`, `00548230` | Outdoors the cell of the grid square under it (`x >> 12`, `y >> 12`) takes it: out of its old cell (`0054ca90`), into the new one's list, 3D loaded when that cell is attached (so `OnLoad`) | as above |

### What was wrong

- `GetButtonPressed` was one value for everyone: the first script to ask
  took any box's button. The loop: while the revise activator runs, any
  other pop-up answered "OK" (button 0; the review found Old World Blues'
  start message doing it in the PR branch's vms16 log) reached
  `VCG04ActivatorScript` as "Edit Name": name entry, then the revise box
  again, every time another message came up. A neighbour reading it every frame (the Mojave
  Express box once used, `vMojaveExpressBoxSCRIPT`) could take
  "Finished", so the prompt never ended; a button nobody read stayed for
  the next box.
- The moved activator didn't run where it was moved: its script ran only
  while its placed square (west of Goodsprings) was attached. Answered out
  past the border, the button did nothing there and fired when the player
  next came back to Goodsprings: the rebuild menus and the prompt again,
  out of nowhere (seen live on the old build: "Rebuild Character" at
  x -58000 did nothing; back at Goodsprings the tag skills menu opened).
- Starting a quest re-runs or resets nothing, in the game or here (checked
  for the "pop-ups come back when a quest is added" idea).

### Implemented

- `world::scripting`: `GameState::button_owner`; `ShowMessage` drops a
  waiting button and records the owner; `GetButtonPressed` only for it.
- `world::ref_scripts::RefScripts::place_moved` and `moved_references`:
  references scripts moved (not people or creatures, whose cells follow
  their AI and aren't followed here) leave their old cell's list and join
  the attached cell under them, with `OnLoad`; the viewer calls it each
  frame after attaching cells.
- The viewer logs "Message box shown: ..." when a script's box opens.

### Tested

`crates/world/tests/revise_prompt.rs` on a generated `VCG04`-shaped world
(`testdata::revise`): another script's box answered "OK" no longer reaches
the activator as "Edit Name"; a neighbour asking every frame no longer takes the
box's "Finished"; an unread button is dropped by the next message; the
prompt triggered past the border is answered there ("Rebuild" asks once
more, "Finished" stops the quest) and nothing is asked again back home; a
moved reference changes cells. The other three tests were run against the
old behaviour and fail there; the "OK" test was added after (under the old
single button the activator, the only one asking, takes the 0).

### Verified live (release viewer, installed data)

`WastelandNV --at -58000,3446,8600,90 --run "RewardXP 100" --run-at 5
"StartQuest VCG04"`, answered with real clicks (window capture script in
the private folder): the prompt shows; "Rebuild Character" runs at once
(face menu still a stand-in line, tag skills, traits); the prompt shows
once more; "Finished" restores the XP (level 2 menu) and nothing is asked
again.

### Not done / not compared

- The face menu (`ShowRaceMenu`) and the SPECIAL menu `SetSPECIALPoints`
  opens in the game (`CharGenMenu` kind 0) aren't there yet.
- `GetAV XP` on a player who never earned XP gives nothing (the block
  stops) instead of 0, so the prompt can't show before any XP.
- Not compared with the original game.

## Current priority: opening camera and assistance behavior

User clarification: the clips play. The defects are player camera input
during lying/sit-up, unnatural looking while seated, and Doc not assisting
the player getting up. Start with Ghidra's FNV decompilation. Earlier
track/request logs and stage55 progression did not establish correct behavior.
Scope confirmed by user: opening sequence only, not later bed use.
The input gate and Doc's skipped transition are now corrected; exact
choreography remains an M1 verification item before calling the opening done.
The original Reflectron sex page was inspected after the user opened it;
the reference is recorded in FACE_CREATION.md.

Ghidra-first follow-up (same executable build/hash used below):
- `005c95a0` (EVP) calls `008a6ce0(0,0)`, setting the actor's deferred
  evaluation flag +0x145 and nudging its package timer. `008da670` explicitly
  permits reevaluation in settled sit states 0/4/9. Our furniture early return
  skipped that request. At stage45 the authored EVP can select Doc's bedside
  standing package (GetStage >=40) and `NVchairStandDoc` (stage10..50,
  marker14). Waiting until stage55 ResetAI skips that eligible chair exit.
- Camera trace: `009445b0` reads mouse axes and rejects looking when control
  mask2 is set (`005a03f0`, interface+0x680), or `0093a740` player flags
  +0x798..79b are active. `00931d90` applies the seated downward pitch limit;
  `00931b60`/`00953f20` distinguish body heading from seated look offset.
  `00952290` writes pitch to Bip01 Looking and heading to the first-person
  root; `0094ae40` reads Camera1st and applies residual pitch. The VCG01
  stage0 script explicitly enables looking. The missing independent gate is
  now traced: `005cc4f0` AddScriptPackage calls `005cc7a0(1)` for the player
  in the supported package path, setting +0x79b; `005cc7c0` RemoveScriptPackage
  calls that setter with0. The earlier instruction scan incorrectly limited
  its address range to actor code, missing the script-command setter.
  This blocks mouse looking throughout the player script package, including
  bedsit, irrespective of EnablePlayerControls. No clip-name rule is needed.
  Raw decompilation and assembly remain in nv-re/decomp/codex-m1, including
  the newly recovered player update at `0093e860` and camera-follow.asm.

Original replay observations, 2026-10-03: read-only process sampling captured
player+0x79b=1, other three look-block flags=0, constant pitch0/headingpi
and changing Camera1st transforms during the seated opening. After stage55,
all four flags were0 and the player angles changed. Evidence outside repo:
codex-m1/original-opening-memory.jsonl and original-after-opening-memory.jsonl;
reader observe-opening.ps1 only uses OpenProcess(PROCESS_VM_READ).
Snapshots after character creation showed Doc standing immediately in front
of the player, followed by normal movement instructions. They do not resolve
the precise hand-contact timing. Physical Escape stopped the first capture;
the user explicitly resumed. No original game memory was written.

Eleventh batch implementation: GameState::player_looking_blocked combines the
saved player package with the separate looking-control mask; viewer
look_around obeys it in walking mode. Generated tests exercise script command
replacement/removal, invalid/NPC targets, independent control mask, save/load,
and the actual Bevy mouse-input system including release and free camera.
907 core tests/doc tests and78 viewer tests pass, both clippy and formatting
checks pass, both release builds pass (package-look-* logs). The release
new-game GPU smoke capture reaches bedsit and Doc's bearings line, then exits
normally: codex-m1/package-look-opening.png and accompanying logs. Input
suppression/release is checked by the actual Bevy system regression; this
capture alone does not test mouse motion or the full opening route.
Eleventh batch installed in nv-rs-play; project/play SHA256 match:
223C24345C8412D24B5F606120AB252437F86D47CC37F20F6E15275AACEA4636.
No test viewer, build, memory sampler or agent remains running. Next: precise
Doc/player assist timing and the unfinished face editor; do not retrace the
now-established package look gate. Original game remains open for the user.

Tenth batch: viewer/src/ai.rs now consumes queued package evaluation before
the settled-furniture early return. Both request sources are consumed once;
entry/exit states retain their request until settled. Existing package
selection and stand-up handling choose the animation; no Doc-specific rule.
Two generated regressions cover the real package switch and deferred entry/
exit request. Viewer77 tests, clippy, formatting and release build pass.
Core unchanged. Logs: codex-m1/doc-evp-{tests,clippy,build}.log.

Live new-game run (base-only plugins, research viewer40672/window920990):
accepted Courier with Return. Stage45 EVP now starts the original
SpecialIdle_NVDocChairFrontExit.kf at77.7s; it releases furniture at89.7s,
before stage55 ResetAI at89.9s. Observed Doc leaving his chair. This proves
the previously skipped transition; precise assistance timing/placement has
not been compared with the original. Face creation still auto-accepts, so
this is not full-route acceptance. Logs: codex-m1/doc-evp-opening{,-error}.log.
The authored exit contains NPCDocPlyStand at7.067s (doc-exit-kf.txt).

Historical tenth-batch publication (superseded above):
BED7C70C0BA305878F5DA645FD0848CE9D52F1ED0A9DCEC0AC06F1B65C89533B.
That tenth build does not include the new package look gate described above.
Research viewer40672 has been stopped; no build or agent remains running.
Original game window724636 has now passed character creation and reached
normal movement. The user's replay supplied the input-gate evidence above.
Sky input still does not drive the original game's DirectInput. Earlier research
viewer43928/window722718 is stopped; its animation-repro logs are historical.
Read-only audit identified a possible furniture endpoint overshoot between
Sitter's clock and ActorRig::drive; unconfirmed, not a fix for this report.

## Current handoff: original tester compared; composition fixed

User opened the original tester. Observation: Strength7, remaining00;
original cabinet framing corresponds to our scene but room remains blurred
behind it and bulbs were brighter. Fixed our composition: explicit REPLACE
and transparent output clear on the 3D menu camera; transparent intermediate
clear on the HUD camera. Bevy's implicit blend choice depended on camera query
order and could blend transparent scene output over opaque black. The fixed
GPU capture shows the room behind the menu and stronger glows without tuning
materials. Evidence: VIGOR.md and codex-m1/vigor-background-fixed.png.

Viewer75 tests, clippy, formatting and final release build pass.
Ninth batch installed in nv-rs-play; project/play SHA256 verified equal:
6AFE4F581E5FA8B29144CAB5B6111BA00F2CC75C6788CEC1D502F756C15A2626.
Core unchanged (906 tests, clippy/format/release passed in previous batch).
Only runtime file changed: viewer/src/game_menus/vigor.rs. Logs:
codex-m1/vigor-background-{tests,clippy,build,capture}.log.
Background blur and exact colour/bloom match remain open; this is not a full
visual acceptance. User was asked to open ShowRaceMenu next for reference.
Original game is now process40676 (window724636); earlier handles are stale.
That publication completed with no test viewers, builds or agents remaining.
The subsequent animation investigation above supersedes its next action.

## Previous foundation batch

Current batch adds `world::chargen::appearance` (and its module export):
traced playable/race/sex filters for race, hair and eye choices, with remapped
plugin IDs and deleted-winning-record exclusion. Six generated regression
tests pass. Official-data observations:4 races;18 male/23 female hair and4
eyes per race/sex. Evidence and missing face-editor pieces: FACE_CREATION.md.
All checks complete:906 core tests/doc tests,74 viewer tests; both workspaces'
clippy, format and release pass (appearance-*-tests/clippy/build.log).
Eighth batch installed in nv-rs-play; project/play SHA256 verified equal:
FEFB03580647831CD9E6156EFA2369762599A2A520138AA3204673DD7FA1182E.
WHATS-NEW distinguishes the internal choice reader from the unfinished editor.

Original-game automated input did not work, even after refocusing. The user
subsequently opened the tester manually (current evidence above).
Research viewer42320 was stopped for the build; its stage55 quicksave remains
in nv-re/work/codex-m1. Face auto-accept is still an M1 blocker.

Seventh batch implements the original Vit-o-matic scene through the game's
XML: animated pages, live number/button/bulb appearances, keyboard controls
and original-triangle mouse picking. The old SPECIAL text substitute is
removed. Detailed executable/asset evidence and remaining limits are in
[VIGOR.md](VIGOR.md). MeshData preserves exact NIF shape names separately
from diagnostic labels. Controller input and embedded-light replacement
models remain unsupported.

Checks: 900 core tests/doc tests, 74 viewer tests, both clippy and formatting
clean; both release builds passed. Installed in nv-rs-play at 05:33;
project/play SHA256 match:
7403F13C84093DC74B244203A550C19E4330A766E561ABEF6B7345C4AC89F9BB.
WHATS-NEW.txt records implemented and unverified scope. Logs:
`%USERPROFILE%\nv-re\work\codex-m1\vigor-*-tests/clippy/build.log`.
The viewer regression changes live values on each click, enforces the budget
and limits, rejects early Done, and closes at the exact budget.

Real-model harness: `nv-re/work/codex-m1/vigor-observations`; log
`vigor-scene-check.log`. It loads 274 pieces, all 16 page sequences and all
replacement textures. Rays hit main/index/LookInside buttons in animated
poses. Built-in GPU screenshot mode produced the inspected Strength page
`codex-m1/vigor-page1-clean.png` (base-game-only plugin list avoids DLC notices).
It shows the real cabinet, page, remaining digits and five lit bulbs.
This is implementation/render evidence, not original-game comparison.

Live PC mouse/keyboard validation now passes (VIGOR.md records the sequence).
The tester updated values, played its page transition, rejected early Done,
and closed at40/restored the room. This was an isolated script-opened check,
not the quest route. Process42320 then loaded the existing stage55 quicksave,
and was later stopped for the build (current process status above).
Do not mark M1 accepted; the face menu is still auto-accepted,
movie playback and mid-animation save restoration remain incomplete.

Changed files: cellview/src/{vigor,lib,lockpick}.rs, cellview/tests/load.rs,
ui/src/menus/{vigor,mod}.rs, viewer/src/game_menus/{vigor,mod}.rs,
viewer/src/{main,lockpick,menus}.rs, viewer/Cargo.{toml,lock} and tracking docs.
The internal testdata dev-dependency creates menu test fixtures from scratch.
No original assets or decompilations are stored in the project.

Previous six batches: package action ordering, physical player/camera
separation, actual opening camera tracks, deferred ResetAI furniture release,
NPC scripted loaded-idle ownership and forced held-pose replacement, then the
same player replacement after its request gate. The isolated opening reached
stage55; F5/F9 and cold restart there preserved player/Doc positions, headings
and quest stage. Last published batch6 hash (before this publication):
02E484E377CE6FCC87DCD21F471ACC37537E2FA61C4D8CD8D92F459414924F0A.
Detailed earlier evidence follows.

**Fifth batch checked and published:** NPC script PlayIdle now
queues one pending IDLE per actor, as process +0x350/flag0x80 does. The
viewer consumes requests for loaded actors and unconditional leaf group-7
IDLE records, using their actual KF and record loop count. Other groups,
trees/conditions and unloaded actors are diagnosed, not substituted.
The script owns the animation clock through completion; the free-idle and
furniture overlay clocks cannot replace it. ResetAI clears that ownership.
Active animation persistence remains unsupported.

Replacement evidence: 00498290 checks pending +0x128 and calls
00498910(0,1); that clears current +0x124's section slot when its sequence
matches, then 00498670 removes the sequence through 00a2ec50. It promotes
the pending request before playback. Consequently 004949a0 blends from
the actor's current pose using the incoming blend-in, without the freed
sequence's blend-out. The read-only agent's suggested ordinary cross-fade
was rejected after inspecting the slot writes. Core regression covers
interruption during TransDest, pose continuity, blend duration and repeated
requests. Viewer regression covers clock ownership and completion.
Full checks passed: 888 core tests/doc tests and 73 viewer tests; both
clippy, format and release builds passed. Logs: codex-m1/batch5-*.
Live opening log confirms stage30 starts VCG01DocGiveMirror, the normal
seated idle resumes, and stage55 ResetAI takes Doc to the tester. Report006
captures that checkpoint. This was not an original-game visual comparison;
the existing missing face-menu auto-accept still prevents acceptance.
Published to nv-rs-play/app, SHA-256
27CDEAA002A9B90C709202E5A5E30BEDBA5C4917A8FBC5C8D921527CFD7A9750.
Test viewer and all agents from that batch are closed/finished.
Changed: world/animation.rs; viewer/actors.rs, player_idle.rs, scripts.rs,
sitting.rs, ai.rs and main.rs.

**Fourth batch, checked and published:** ResetAI is now a separate deferred
actor request in world/scripting.rs. The viewer consumes it before furniture
can return early, clears the old procedure/path, and releases furniture
without relocating the actor. Trace: 005c9530 -> 008a6ce0(0,1), then the full
reset path -> 0088d640 -> HighProcess +0x84 / 009287e0. The represented scope
is furniture/procedure restart, not every original combat/process reset.
Two core regressions and a viewer path/position regression pass. Full checks:
887 core tests/doc tests, 72 viewer tests, both clippy/format/release builds.
Installed in nv-rs-play/app, SHA-256 matches the built executable:
CA730B10F3BAE7DEAF43A247B14F30850F57FA240B94E4F9855A4664C3BBC569.
Also fixed non-forced special-idle release to preserve the same startup
EaseIn/TransDest refusal as 00498910; its regression passes.

Live evidence: batch4-opening.log/error in codex-m1. At VCG01 stage55,
ResetAI released Doc and he followed VCG01DocMitchellTravelToPlayerAtTester
for 577 units to the machine. Reports 003 and 004 bracket an F5/F9 test:
player position/heading, Doc position/heading and stage55 match exactly;
neither report keeps Doc in furniture. A separate cold viewer launch then
loaded that save: report 005 matches the same values (batch4-restart.log).
This checks that checkpoint through process restart, not active animation
persistence or the complete creation route. Test viewer is closed.
No build/subagent is running. Source edits this batch: world/scripting.rs,
world/animation.rs, world/tests/script_reset_ai.rs, viewer/ai.rs,
viewer/player_idle.rs, README and these shared tracking docs.

NPC PlayIdle trace confirmed the retail stage-30 mirror request in the live
log. 005cb2d0 stores IDLE at process +0x350 via 008d9410 and sets flag 0x80
via 00903180. Consumer found in 008ba600 (existing furniture/ decomp): it
reads/clears flags at entry, and flag0x80 gets +0x718 pending IDLE then calls
00497f20 with mode3, bypassing the immediate-request busy gate. Flag0x10
instead calls +0x70c. 00498290 frees the prior loaded idle binding with
00498910(0,1), then plays the resolved group. Forced replacement/blending
is now represented by the fifth batch above, awaiting its live checks.
The read-only audit agent has finished. Parent owns Ghidra now.

The viewer now dispatches unconditional, idle-only player package actions
through the actual first-person skeleton and KF files. Camera1st sets the
view while the walking capsule and input heading stay independent. F5,
scripted saves and F12 use physical position/heading. Loading clears transient
camera playback; restoring its clock/sequence from a save is still missing.

Rules: `world::animation::Player::request_special_idle` preserves the
EaseIn/TransDest gate and duplicate rejection. `008dab40` clears the player's
pending request after the immediate `008dae00` call even when busy; this is
not the NPC deferred path. `animation::camera` holds camera placement and
hand-follow math, `viewer/player_idle.rs` loads assets and presents the view.
It runs after walking and before aiming/interactions; script requests enter
the next frame. No package IDs, timed camera stand-ins or fitted offsets.

Camera provenance (same executable hash below): `00888b50` / `00889812`
place the first-person root at actor feet; `00952290` applies root heading
and pitch around Bip01 Looking. `004a0c90`, `00524ac0`, `0043f8d0` establish
matrix order/signs. `0094ae40` reads Camera1st world position/rotation and
right-multiplies residual pitch. Hand-follow defaults are 0.85, chase 2 s
and attack chase 0.05 s (`00f5b5e0`, `00f5b610`, `00f5b640`). The chase
advances dt/chase of the remaining difference, clamped to the target.

The `00933840` override requires UI state and process byte +0x364, whose
meaning is unresolved; it retains ordinary input rotation. Do not equate
it with all dialogue. The viewer leaves interactive-dialogue, weapon-out
and V.A.T.S. camera combinations out; opening line-only SayTo is supported.
IDLE conditions/tree dispatch, action scripts/topics, animation text-key
sounds and in-progress save restoration are also not implemented here.

Live evidence: `codex-m1/batch3-opening.log` and error log; report 002.
All four expected camera idles started; accepted four DLC notices and
Courier name, then reached vigor-tester instruction. The existing face menu
auto-accept still occurred, so this is NOT faithful opening acceptance.
This development run preceded the final body-heading/load-reset/scheduling
fixes. Final release smoke exited successfully (`batch3-final-smoke.png` and
log). Core tests/doc tests: 885 passed; viewer: 71 passed. Both workspaces'
clippy, formatting and release builds passed (`codex-m1/batch3-*`). Installed
the checked release in nv-rs-play/app; executable hashes match. No test
viewer from that batch remains running. Later checks supersede these counts;
all agents from that batch have finished.

Changed: world/animation.rs, new world/animation/camera.rs; viewer/main.rs,
new viewer/player_idle.rs, viewer/scripts.rs and viewer/report.rs. Earlier
batch notes below are historical; their “not dispatched” claims are superseded
by this bounded player-camera implementation.

## Observed baseline

Launched the release viewer with the installed Data folder and
`--official --new-game`, working directory
`%USERPROFILE%\nv-re\work\codex-m1` so tests did not overwrite play saves.
Accepted Classic, Mercenary, Caravan and Tribal Pack notices, then the default
name Courier. Doc's spoken/result-script chain reached VCG01 stage 55 and
asked the player to use the vigor tester. No console stage jumps were used.
The existing face-menu path auto-accepted the missing menu; this prevents
counting the run as a faithful character-creation test.

The view remained upright, facing away from Doc while the player should be
in the scripted bed sequence. `AddScriptPackage` only records a package and
requests evaluation; the player's capsule/camera is outside NPC Walker AI.
NPC `PlayIdle` events are also logged and discarded by viewer/scripts.rs.
SPECIAL still has a text-panel substitute in viewer/menus.rs. These are
foundation blockers, not cosmetic leftovers.

The opening quest's installed stage-0 script requests FNVIntro.bik and jumps
to stage 90. Many earlier graveyard stages remain in the file but are skipped
by that retail path. Do not recreate unused stages merely because their
assets exist. Bink playback is currently log-only. The old description that
`--new-game` starts after the movie was inaccurate.

Local baseline evidence (not for redistribution): `opening.log`,
`opening-error.log`, `reports/001/` (picture, position and complete state),
`vcg01-source.txt`, `vcg00-script.txt`, `vcg01-script.txt`, package/idle
dumps, all under `%USERPROFILE%\nv-re\work\codex-m1`.
The baseline viewer was closed after recording. The user's play saves were
not loaded or overwritten. No original-game screenshot comparison yet.

## Implemented this batch

`world::ai::actions` reads three independent package actions:
POBA (begin), POEA (end), POCA (change). Each owns an INAM idle, embedded
script and TNAM topic. TNAM ends the action. It preserves compiled code,
source, header and local-variable fields. SCRO form IDs follow load order;
SCRV variable indices and compiled bytes stay unchanged; nulls stay null.
Repeated markers replace the prior action, as the original loader does.

`Package::load` exposes these actions; the synthetic trespass package has
empty actions. `nvinspect <Data> ai <PACK>` prints the actions, linked
animation models and loop counts. `ai <REF>` also prints the selected
package's actions. It explicitly reports that action playback is pending.
Four synthetic tests cover separation/boundaries, compiled-only scripts,
load-order remapping, absent/truncated data and repeated action markers.

This is a data-reading foundation. It does not dispatch callbacks, play
animations or repair the observed camera behavior yet.

## Evidence for the reader

Executable: local FalloutNV_unpacked.exe SHA-256
`19406942E48724D797300C4EA6BE9AC69A32F670B8C35DB09279A2258422739F`.
Ghidra: package loader 00673a40 dispatches all three markers to 0067dd20.
0067dd20 clears the action, reads INAM into its idle field, reads the embedded
script fields, then reads TNAM and returns. The existing decompilation is
under `nv-re\decomp\ai\n`; this batch's focused decompilation of 0067dd20
and PlayIdle handler 005cb2d0 is under `nv-re\decomp\codex-m1`.
These addresses prove parsing, not the runtime callback sequence.

| Package | Begin idle | Change idle | Animation loops |
| --- | --- | --- | --- |
| VCG01PlayerSection0 (001055BE) | LooseVCG01PlayerWakeup (001055BF) | None | Wakeup FF/FF |
| VCG01PlayerSection1 (001055C3) | LooseVCG01PlayerSitup (001055C0) | LooseVCG01PlayerBedsit (001055C1) | Situp 00/00, bedsit FF/FF |
| VCG01PlayerSection2 (001055C4) | LooseVCG01PlayerStandup (001055C2) | None | Standup 00/00 |

All three end actions are empty. The linked models are
`Characters\_Male\IdleAnims\SpecialIdle_NVCG01PlayerWakeup.kf`,
`SpecialIdle_NVCG01PlayerSitUp.kf`, `SpecialIdle_NVCG01PlayerBedsit.kf`,
and `SpecialIdle_NVCG01PlayerStandup.kf`. IDLE DATA group byte 0x47:
special-idle group 7 plus loose-idle flag 0x40. Use animation::Player, which
honors StartLoop/EndLoop and transitions, not the simpler IdleClock. The
normal IDLA list is not a replacement for these action fields.

## Next bounded task

### Runtime trace and second foundations batch (2026-10-03)

`AddScriptPackage` (005cc4f0) invokes HighProcess +0x59c (00903bf0)
on the old package, then +0x598 (00903a80) on the new one, before installing
it. This happens even when the package IDs match. Disassembly confirms the
old action is +0x6c (change) and the new action +0x4c (begin); within each,
the script precedes the idle, which precedes the topic. Ghidra incorrectly
labels some wrappers as MFC methods; use their instructions/offsets.

The script runner now emits ordered `PackageAction` requests, including
same-package replacement, and rejects invalid package/actor forms. Three
synthetic regressions pass. Requests do **not** execute action scripts,
topics or idles yet; viewer/inspector diagnostics explicitly say pending.
Removal and ordinary AI package lifecycle dispatch remain unimplemented.

Player positions used by walking-mode scripts, F5 and F12 now come from
the collision capsule rather than subtracting a height from the camera.
Free-camera mode retains its prior convention. Two new viewer regressions
cover displaced cameras and the free-camera behavior. Report/save heading
still comes from the view; separate body heading before animated rotation.

All four opening KFs contain Camera1st tracks (priority 95), and sample
through `meshes\\characters\\_1stperson\\skeleton.nif`. Durations: wakeup
6.000 s, situp 5.400 s, bedsit 8.167 s, standup 11.133 s. Their KF cycle
type is once; IDLE loop counts determine repetition. Camera3rd on the male
skeleton is static under Scene Root, so sampling that node cannot supply
these motions. Raw samples/harness: `nv-re\\work\\codex-m1\\kf-observations`.

Camera trace: 0094e1d0 finds Camera1st into global 011e07d0. The first-person
branch in 0094ae40 reads its world rotation (00461130, node+0x68) and world
position (0045bb80, node+0x8c). It combines rotation with the negative
011e0770 angle except in its dialogue guard (00933840). Skeleton placement,
rotation basis and special-idle replacement/completion still need tracing.
008dab40 validates a requested idle and 008dae00 hands it to 00497f20;
00498f80 guards loading/transition states. Do not flatten two same-frame
requests into a guessed last-wins animation rule.

Second-batch changed files: world/scripting.rs, world/tests/script_package_actions.rs,
nvinspect/play_cmd.rs, viewer/scripts.rs, viewer/walk.rs, viewer/report.rs.
Core tests/doc-tests: 878 passed; viewer tests: 70 passed. Both workspaces'
clippy, format and release builds passed (viewer final logs after annotating
the Bevy system's resource argument count). A walking-mode house screenshot
run exited successfully; `batch2-smoke.png` and log are in `codex-m1`.
Published to `nv-rs-play/app/nv-viewer.exe`, with matching SHA-256; release
notes explain playback is still pending. Logs are `codex-m1/batch2-*`.
Parent continues Ghidra tracing; no viewer/build is running. Bounded agents
have finished and own no ongoing work.

Trace the runtime use of the package's action structures, including same-
package reassignment (the VCG01 source explicitly uses this for bedsit).
Determine action ordering, idle completion/loop handling, and how the
player's animation drives its camera node in the original executable.
Read actual KF tracks and skeleton transforms. Add regression tests around
the resulting state transitions before connecting the viewer. Do not add
package-ID-specific quest logic, arbitrary camera offsets or guessed timing.

Then replay the opening against the original game and continue to menus,
movement, exit, and save/reload. Bink and the missing menus remain required
for M1; passing tests for the package reader does not complete that gate.

## Changed files and checks

Code: `crates/world/src/ai.rs`, new `ai/actions.rs`,
`crates/world/src/living/trespass.rs`, new
`crates/world/tests/package_actions.rs`, `crates/nvinspect/src/play_cmd.rs`,
`crates/nvinspect/src/main.rs`, `crates/nvinspect/src/coverage_table.rs`,
and `viewer/src/args.rs` (help text only).
Documentation: AGENTS.md, README.md, MILESTONES.md and this file. Removed the
redundant docs/SESSION_INSTRUCTIONS.md copy; AGENTS.md is the single shared
instruction source and CLAUDE.md already points there. Archived notes remain.

Core workspace tests/doc-tests passed: 875 total, including four new reader
regressions. All 68 viewer tests passed; clippy for all targets and formatting
checks passed in both workspaces. Build/test logs are under the same evidence
directory. Real-data inspector outputs
are `inspected-VCG01PlayerSection0.txt` through Section2. The source hash
manifest is `source-sha256.txt` there; there is no Git commit for this batch.

Both final release builds passed. Published the checked viewer into
`nv-rs-play\app\nv-viewer.exe` and verified its SHA-256 matches the project
build. Publication used `& %USERPROFILE%\nv-re\work\publish-play.ps1` in
the current PowerShell: the older `powershell -File` invocation refused to
load scripts. No execution-policy setting was changed. No viewer, build or
Ghidra process from this batch remains running.
