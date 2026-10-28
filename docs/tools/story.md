# story.py: Goodsprings played with real actions by a host and a client

Private doc. Code: `tools/story/`. It is the lab's sibling (`docs/LAB.md`, `docs/tools/lab.md`): the same step engine (`tools/lab/lab_core.py`: time box, fail-fast checks,
the digest clean rule, first digest divergence, crash probe), the same report style and exit codes, but the content is the real Goodsprings (milestone M1) instead of a
generated test cell. The whole runner is tested on a fake two-copy world: `python -I tools/story/run_tests.py` prints ALL OK.

## Run it

```
python -I tools/story/story.py run                       # all 18 steps on one pair of copies (S00 first: loads the save, resets the Goodsprings quests)
python -I tools/story/story.py run --steps S03,S05       # some steps (S00 always runs first); a step's start state is forced by the host's console and written in the report
python -I tools/story/story.py run --stop-on-fail        # stop at the first step that does not pass (default: later steps force their own start state and go on)
python -I tools/story/story.py run --fake [--bug NAME]   # the game-free run; with a bug the matching step must FAIL (names: tools/story/storyfake.py BUGS)
python -I tools/story/story.py list | summary out/story/<stamp> | down
```

Options: `--budget-min 25`, `--param story.KEY=VALUE` (`story_core.PARAMS`: settle, prep, walk, talk, choices, pick seconds), `--save` (default `nvcoop_autosave_ready`, the prepared
save of the lab and the bench), `--keep`. Output: `out/story/<stamp>/results.json` and `summary.md` (under 5 KB) plus the comparison with the previous story run.
Exit: 0 all PASS, 1 FAIL / TIMEOUT / CRASH, 3 BLOCKED (build, save, method missing), 4 ENV, 2 bad arguments.

Second PC: `tools/bench/remote/task.ps1 -Script C:/fnv/fnv-coop/tools/bench/remote/story_run.ps1 -Wait 5 [-ScriptArgs "-Steps S03,S05"]`; it writes `C:\fnv\story_done.txt` when the run ends
(`story_out.txt` the summary, `story_err.txt` the progress). The PC is shared through `C:\fnv\dino.lock` (create it before launching, delete it after). Needs a build in
`out/L1split` (both DLLs), `nvcoop_autosave_ready` in the Saves folder of copy 0 (the harness copies it to copy 1), Steam running, the owner not playing (the proof harness guards).
The switches are the lab's plus `intents.dialogue=1`, `intents.dialogue_results=1`, `intents.stage=1`, `hooks.story_capture=1` (docs/handover/P6-requests.md G46); the game
runs with `NVCOOP_TEST_HELPERS=1` (`test.quest.reset`). The lab plugin is NOT installed.

## What a step is

`tools/story/goodsprings.json` holds the steps, the NPCs with the INFO ids they offer, the quests and reputations. A step has a lead (`host`, `client`, or `both`), a time box (`box_s`),
`prep`, actions (`do`), a `goal` and optional `expect`. Everything is real play except `prep`:

| part | what it does |
|---|---|
| `prep` | brings the step to its start state through the HOST's console: `startquest`, `setstage` (only when the quest is behind), `give` an item. The client must show it within `story.prep_s`. Every forced line is listed under `prep` of the step in results.json and counted in summary.md. This is the only console use. |
| `goto` | walks to a ref (`bot.goto` mode walk, the input driver; the talk steps ask for 100 units). A walk that says ok but ends farther than 1.5 times the radius in 3D (the first story run ended 247 units from Doc; the engine opens no menu at that distance, the floor of an interior counts) gets one closer try, then the teleport (`walks[].dist3d`, `teleported`, `notes`). A walk the navigation cannot do is replaced by a teleport and the step says so (`walks[].teleported`, `notes`). |
| `talk` | activates the NPC. A client's talk is the guest-initiated dialogue: the host must grant it; the time to the open menu is `talk_ms`. |
| `pick` | the initiator picks lines. The script's INFO ids first (each once, in order), else (`fallback: first`) the first line not said yet, until `stop_when` shows on the host, the talk is over or `max` picks. Every pick is recorded: who, INFO, the words, `how` (script or fallback), whether the engine said it, ms. |
| `activate`, `take`, `kill` | a door, a pickup (gone on both copies is checked), shooting (arm, walk to 400 units, shoot until dead on that copy, dead on both is checked). |
| `expect_not` | a barrier: the stage must NOT have fired yet (VCG01 stage 200 until both players went through the door). |
| `goal` | `stages`: the quest is at least at that stage on one copy within the box, then on the other within `story.settle_s`; `objectives`: flags on both copies. |
| after the goal | `snapshot` of both copies (`state`, docs/snapshot-schema.md) before and after; the plan's objective effects of the goal stage must hold on both; `expect.items` (who gains how many), `expect.reward` (XP, who). |

Failures carry a class like the lab's: SYNC (the copies disagree, or the guest's pick did not reach the host), TEST (the route or the script is wrong), ENV, CRASH, BUILD. A goal that
neither copy reaches is SYNC when the client picked a line and every action said ok (the pick must have been lost on the way to the host), else TEST.

## The report

Per step, in `results.json` under `steps[].story`: `prep`, `picks`, `walks`, `talk_ms`, `stage_ms` (host / client, ms from the last action), `before` / `after` (stage and objectives of the
step's quest, karma, reputation, XP, caps on both copies), `delta` (what each copy gained: stages, objectives, items, XP, caps, karma, reputation), `divergence` (the first SHARED value
the copies disagree on: quest running / stage / completed / done list, objectives, karma, reputation), `notes`. `summary.md` has one table row per step (stage host/client, objectives
that changed, picks said/made, what each copy gained, the first failed check), a block of up to 7 lines per failed step, the guest-dialogue line and the comparison with the previous run.

## The steps (18)

| step | lead | what |
|---|---|---|
| S00 | both | save loaded, session synced, the six quests reset, both players at Doc Mitchell |
| S01 | both | VCG01: the saved start state is what it should be (stage 110, objective 50; the exam talk of the first design is not played any more, see "The start save") |
| S02 | both | VCG01: both follow Doc out; barrier of stage 200 (not before the client passed), the quest ends and starts VMQ01 |
| S03 | client | VCG02: Doc Mitchell said you could teach me (INFO 10A1E4, stage 10) |
| S04 | host | VCG02: Okay, I'm in (10A1E5, stage 25, geckos enabled) |
| S05 | both | VCG02: host shoots gecko 1, client gecko 2 (stage 50) |
| S06 | client | VCG02 on the quest path: the client picks "Sure, I'll come with you" (10A1EB); its result runs on the host (objective 50 of VCG02 shown, Sunny's ammo) and objective 50 shows on both copies. Not "Couldn't hurt" (15D97E): in `autosave_story_start` forced to VCG02 stage 50, Sunny does not offer it (on the host either, so the saved game's own state decides, not co-op); she offers 10A1EB and the tutorial exit 10A1E3. The step tests that a guest's pick runs its result on the host and reaches both copies, not one particular line; the talky fallback never picks a line that ends the talk |
| S07, S08 | client, host | VCG03: the Broc flower / the Xander root: only the picker gets it, the plant is gone on both. Their start state forces VCG03 to stage 10 (the stage that enables the plants), and "gone" is the plant's own `setdestroyed 1` mark (`harvested()`), not the disabled flag |
| S09 | host | VCG03: Sunny's greeting takes both plants (objective 20); the start state gives the host the plant the client picked |
| S10 to S16 | host / client | VMS16: Trudy (5), Ringo (10), Sunny (50), Ringo (60, 65), Sunny (70), the fight (80), Ringo's greeting (100, XP 50 for all, reputation) |
| S17 | host | VFreeformGoodsprings: Chet's ambient dialogue |

Not covered: the opening cinematic and chargen of VCG00/VCG01 (timers, SayTo scenes, the race menu and the Vigor tester are menus no bot drives; S01 starts at the exam, stage 80),
the bottle shooting of VCG02 (no placed refs), VMS16's optional Trudy / Chet / Easy Pete skill-check lines, VMS16b (the exclusive other side of the fight).

## The fake world

`storyfake.py`: two sides with message latency, the plan's stage effects through `tools/bots/playbooks/questlines/model.py`, a guest's talk and pick travelling to the host and the result
coming back, barriers, rewards. What the plan does not say (which kill or pickup moves which quest, what a greeting does) is `KILL_RULES`, `REP_RULES`, `greeting()`: the fake's own, marked
as such. It has 15 bugs (`BUGS`), each with a test that the matching step fails at its check; 9 runner twins in `run_tests.py`.

## In game

See the end of this file for what the first runs measured and found.

## Measured in game (second PC, 2026-10-28, 6 runs, build of master eb40639)

The runner works end to end in a game: boot, the save, the quest reset and the sync of S00 take 68 s; all 18 steps take 6 to 12 minutes; summary.md is 4.2 KB.
The last run (`out/story/20261028-023656` on the second PC): 3 PASS, 15 FAIL. The failures are mostly the script meeting the real game (below) and the findings.

What the runs showed (evidence is in each step's `story` block of results.json):

- **Start states sync.** A `startquest` / `setstage` by the host shows on the client in under 25 s every time, objectives included (S05: `30dc 40d` on both; S09: `20c` on both). The old
  "quest X running on every machine" timeout did not come back.
- **Guest-initiated talk opens in 62 to 203 ms when it opens** (client talks to Sunny, Ringo, Doc), and a guest pick runs on the host: the client picked "I wanted to ask you some questions"
  (`said`), `intents.status service.dialogueResults.ran` rose on the host. But **most client talks do not open at all** (20 s, no menu, the NPC 40 units away on the client's own copy):
  S01 (Doc), S06, S13, S16 in the last run. `intents.status` counted `rejected.OutOfRange` and `rejected.NotLoaded` (up to 47 in a run) on the host. The host judges the guest's reach from the
  puppet's place and needs the NPC's cell loaded; the story now brings the other player along, and S01 still failed. Likely plugin/intents (reach check) and plugin/avatar (puppet position after a teleport).
  The plugin logs nothing for a refused talk (no veto / grant line): the log lines `engine call not supported: rebuildTopicList` and `forceCloseDialog: no DialogMenu open` are all it says.
- **The client lacks stage-done bits the host ran.** After the host set VCG01 stages 9..80 one after the other, `completedStages` is `[1,3,5,7,8,9,10,15,17,20,...,80]` on the host and `[1,3,5,7,8,76,79,80]` on
  the client, while the current stage (80, later 110) agrees. Likely plugin/hooks and synccore (a console `setstage` run by the host is applied on the client only for the stage it names, or the done bit is dropped).
- **A guest's harvest of a quest plant gives no item.** S07: the client picks the Broc flower; the plant is gone on both copies and the client gains 10 XP, but no `13B2BA` arrives on either copy in
  15 s. The host's own harvest (S08, Xander root) gives its item at once. Likely plugin/intents (PickUp claim, AddItem to the origin) or plugin/loot.
- **A copy crashed once** (run 4, S08, the host picking the Xander root): the client, `C0000005 at FalloutNV.exe+4DDCA`. The three other runs passed S08.
- **The client's shots do nothing.** S05: the host shoots the gecko (ammo spent, XP +5, VCG02 reaches stage 50 on both copies, a clean natural stage); the client's `fire` fails with "weapon still
  holstered after the shots, the pulses never reached the game" although `draw_weapon` ran. Likely plugin/bots input on a client copy. The host did not kill a Powder Ganger in 24 volleys at 350 units (S15; TEST).
- **bot.goto(ref) refuses a walk to a ref in another exterior grid cell** (`other_cell`); `walk_ref` walks to the ref's position instead. Walks from an interior to the exterior
  are impossible (another coordinate space) and are teleports, listed per step.

Not a plugin finding, the next work on the script: the save `nvcoop_autosave_ready` is past the opening (say-once state of Goodsprings is spent: Sunny offers "I wanted to ask you some questions" and
"Until next time", not "Doc Mitchell said you could teach me"; `test.quest.reset` resets quests, not say-once flags or NPC places). The steps S03 to S16 need a save made at the start of the game
(before Doc Mitchell's exam) for the scripted lines to be on offer; S02 (the exit scene and the barrier of VCG01 stage 200) needs the real scene, not a walk to a marker.

## Say-once divergence in S06 (follow-ups 1, run 20261028-095118)

`SayOnceFlag 10A1E6` is on the client and not on the host after the host forced VCG02 up to stage 50 (and `areadigest.heals` / `digest.repairs` rise by 2: the step's "no digest repair" check fails, honestly). The guest's pick path is not the cause: the trace lines the dialogue glue now writes (`dialogue: say-once mark ... on the host / on a guest`) show the host marking the greeting (10A1EE) and the picked line (10A1EB) and the guest marking the host-spoken 10B755 and 10A1EB. 10A1E6 is neither: the stage results of the forced chain include scripted `SunnyREF.SayTo player ...` lines (VCG02 stage 40) that every machine runs for ITS player when it applies the stage; the copy whose Sunny is loaded and near speaks the line and the engine marks it said, the other copy cannot (the host stood in another cell), so the flags differ by an engine side effect nobody reports. The union rule (`SayOnceFlag` is a set union in the shadow, D11) needs an observer of the engine's own mark on every machine (a detour of `TESTopicInfo::MarkSaid` 0x61F220, today only a pinned call) that sends the guest's marks to the host; the digest repair only ever copies the host's flags to a guest and cannot clear one. Fixed by the MarkSaid detour (next section).

### The fix: the engine's own marks are reported (wt/sayonce)

`TESTopicInfo::MarkSaid` (0x61F220, thiscall(info), plain ret; prologue `55 8B EC 51 89 4D FC` stolen, 7 bytes, `plugin/hooks_ui` hook `InfoMarkSaid`, both roles) is detoured observe-only: the original always runs, and a 0 -> 1 change of `[info+0x22]` goes to a sink (`nvui::setSaidSink`). Calls made by our own glue (`SaidMute`) are not reported.
The routing is game-free in `fnvcoop_dlg::SayOncePolicy::onEngineMark` / `onGuestReport` (plugin/dialogue/include/fnvcoop_dlg/sayonce.h), driven by `fndlg::EngineGlue::onEngineMark`:

* guest, shared line: `SayOnceMark{info}` (dialogue message 19, guest -> host) over the 0xD1 channel;
* host, own engine mark: `nvui::ops::recordSayOnce` records `SayOnceFlag{info}` (inside the open capture scope when there is one, e.g. the result script of a guest's pick, else its own Script scope);
* host, a guest's report: if the host's byte is not set yet, the host's engine marks its own copy (muted) and records `SayOnceFlag{info}`; the clients apply it with `markInfoSaidOnce`, so every copy agrees and the digest has nothing to repair;
* per-player lines (D11: Speech-XP and trainer lines, `sayonce_per_player.txt`) are never reported or committed, on either side, whatever a guest sends.

Trace lines: `dialogue: say-once engine mark <id> on a guest|the host: report to the host|commit|kept local` and `dialogue: say-once report <id> from slot N: mark and commit|already set on the host|kept with that player`; `coop.status dialogue` shows `engineMarks`, `sayReported`, `sayReports`, `sayCommitted`.
Tests: `plugin/dialogue/tests/engine_test.cpp` (`sayonce_engine_marks`, `sayonce_engine_marks_forged_and_known`, six twins), the codec round trip in `tests/main.cpp`, the pin of the new hook row in `plugin/hooks_ui/tests/pins.inc`.
In-game proof (second PC, 2026-10-28, `--steps S06` and `S05,S06,S07`, host and client copies of the same build, the digest entries listed in full): with the build before the detour, S06's digest report holds
`SayOnceFlag 10A1E6`, `10B755` and `10A1EB`, each present on the client and absent on the host (first divergence). With the detour none of the three is in any report of S05, S06 or S07, and `coop.status dialogue`
shows the host's `engineMarks` / `sayCommitted` rising (5 after S06). The step's "no digest repair" check still fails, now for another reason: the client starts DLC quests the host has not started
(`StartQuest 13653` and `SetStage 2FCA` of Old World Blues, `8891` and `E74B` of Honest Hearts, `3603` of Lonesome Road: `host=0 client=10`), the same entries in the run before the detour; they are not say-once state.
Also seen, not caused by this change (the same in the runs before it): S05 cannot kill its gecko (24 volleys), S07 raises on `player.GetItemCount FalloutNV.esm:13B2BA` (the bridge console takes no `plugin:id` there), and the caps reward of the S06 pick arrives
once, three or four times from run to run (+60, +180, +240; reward-dup).
Known limit: the baseline dump and the digest list every INFO whose byte is set, per-player lines included, so a trainer line the HOST player used is still copied to the guests by a digest repair (the dump has no per-player filter; `plugin/snapshot`).

## The start save (2026-10-28)

The story starts from `autosave_story_start`, a saved game at the end of Doc Mitchell's exam: chargen done, VCG01 at stage 110 (objective 50: follow Doc to the exit), VCG02 and later untouched,
so Goodsprings' say-once lines are unspent. `nvcoop_autosave_ready` (the lab's save) is past the opening and cannot be used for this.

How it is made, without a human playing: the owner's own early game saves ("Save 5 ... Doc Mitchell s House 00 04 08" and "Save 6 ... Goodsprings 00 04 37" in `Documents\My Games\FalloutNV\Saves`) are only READ
and copied into each test copy's Saves folder as `story_save5.fos` / `story_save6.fos` (the owner's folder is never written to). Then, on the pair:

```
# on the second PC (C:\fnv\dino.lock held): line 1 = the save to load, line 2 = the name to save it as
Set-Content C:\fnv\story_probe_args.txt @('story_save5','autosave_story_start')
task.ps1 -Script C:/fnv/fnv-coop/tools/bench/remote/story_probe.ps1 -Wait 5      # python -I tools/story/story.py probe --save story_save5 --save-as autosave_story_start
```

`probe` loads the save on both copies (the lab's way: load, press the load boxes away, wait for the session), prints where it stands (cell, position, stage / running / done / objectives of every story quest
on both copies) and saves it through the bridge's `saveGame` on both copies. `story_save5` read: both copies in cell 1064441 at the same spot, VCG00 100, VCG01 110 running with objectives 10, 30, 40 done and 50 shown,
VCG02, VCG03, VMS16, VMS16b not started. Each copy now holds `autosave_story_start.fos` with its own `.fnvcoop` sidecars (copies 0 and 1 of `C:\fnv\fnv-instances`; the proof harness seeds any later copy from them
like `nvcoop_autosave_ready`, `tools/proof/pf_saves.py`). The script says `start.keep: true`, so S00 loads the save as it is: no quest reset, no move.

S01 only checks that start state. S02 is the real exit scene: both players walk to the door; the exit trigger (`GSDocMitchellExitTriggerScript`) fires VCG01 stage 115 on both copies (measured), Doc's
farewell conversation (INFO 1057E7) should end with the casual / hardcore message box and a timer that sets stage 200 and starts VCG02 (`VGenericTimer` event 3).

### First run from the start save (`out/story/20261028-045950` on the second PC)

S00 (35 s) and S01 pass: both copies load the save and show VCG01 110 with objective 50. S02: walking to the door fires the exit trigger, VCG01 115 and Doc's farewell rewards (Stimpaks, ammo, caps 18) reach both
copies identically; but Doc's farewell INFO 1057E7 never runs its end script (no casual / hardcore message box on either copy in 40 s, no timer), so stage 200 and the start of VCG02 do not follow.
S03 therefore completes VCG01 with the host's `setstage 200` plus a `scenePass` by each player (recorded as forced), and VCG02 is started at stage 5; Sunny still offers only "I wanted to ask you some questions" and
"Until next time", not "Doc Mitchell said you could teach me" (INFO 10A1E4, a say-once top-level topic with only a GetIsID condition), so the guest-initiated pick of the scripted line is still not proven;
the client's pick of the line that IS offered works (said, `dialogueResults.ran` rises on the host). To find out next: why 10A1E4 is not listed (its topic is `VFreeformGoodspringsGSSunnySmilesTopic019`).
Also seen: a guest's dialogue pick ran its result four times on the host (S06: "Sure, I'll come with you", ammo 60 became 240 on both players, `dialogueResults.ran` +4, `committed` +2).

### Dialogue findings (story-dlg, 2026-10-28; runs 20261028-074440 .. 20261028-093347 on the second PC)

* **Doc's farewell (S02) works when somebody picks the goodbye.** The engine speaks 1057E8, 1057E4 and 1057E6 by itself, then stops at Doc's topic list (state 2, three rows: 15E139, 15E13C, 1057E7).
  1057E7 is a row of that list ("<Doc Outro>"), the player's to pick. Run 20261028-092440: the host picked 1057E7, answered the casual / hardcore box, and VCG01 reached 200 on BOTH copies, so the end script runs in co-op.
  The old S02 never picked: `end_dialogue` of the next step closed the menu and the end script had no reason to run. The box is the host's (the origin decides, the guest sees a notice).
  Not fixed in S02: the barrier check ("200 not fired until the client passed") fails because the client walked first, and moving the client's walk after the host's pick made the client copy exit twice (20261028-093120,
  093347: "game copy exited: client", no crash line in nvcoop.log, ~20 s into S02 while the client's farewell menu was open). The original order is back; the client exit is an open finding for plugin/dialogue / hooks_ui.
* **"A pick ran 4 times" (S06) is two picks.** A guest's talk opens on a GREETING INFO (10A1EE), reported as a pick, so the host runs its Begin and End, then the chosen line's (10A1EB). `dialogueResults.ran` +4, `committed` +2 is right.
  The committed txn of 10A1EB holds ONE `AddItem(Ammo556mm x60, mode 3, mask 15)`; the item count jumped to +180 (earlier +240) on both copies before the first sample after the pick and stayed (host 210 / client 210
  at 0 s, 2 s, 6 s). The multiplication is not the runner and not how the committed AddItem lands: the host's own `<npc>.Say` for the spectator voice made the engine run the INFO's Begin script again, once per response line, each time as a host-origin commit (seq 176, 177 next to the runner's 175); fixed by `SpectatorSayGate` (docs/handover/DLG-requests.md, "Resolved"). Guards added regardless: runner key, host repeat-pick rule, glue pick latch.
* **Sunny's VFreeformGoodsprings topics are not offered, host or guest** (S03/S04 lists: only VCG02's 10B754 and Goodbye; S06: 10A1EE 10A1EB 10A1E3). Not guest state (host too), not say-once (`coop.status dialogue.saidProbe`
  reads 0 for 10A1E4 on both copies), not topic sync (nothing in the game data adds or starts it after the new-game start; quest 104C66 is start_enabled). No clean S03 list was captured (queued message boxes eat the
  talk). The one thing left to read: `running` of VFreeformGoodsprings in the start save.
* The trace (`dialogue: ... menu state / pick sent / offer / line done` in nvcoop.log) and `coop.status dialogue` (glue counters, `host.scriptRuns / repeatPicks / rejected`, `saidProbe`) are in each step's
  `story.dialogue` block of results.json; a pick with `watch` records item counts before and 0 / 2 / 6 s after it.

### Story-fix2 (2026-10-28): the DLC intro quests, S05's gecko, S07's item read

* **DLC intro quests (S06's clean-digest rule).** NVDLC02/03/04MQ00's quest scripts (`HonestHearts.esm:008A32`, `OldWorldBlues.esm:002FD0`, `LonesomeRoad.esm:003604`, plus Dead Money's `0062B1`) are on the
  GuestLocalRun allow-list (per-machine DLC setup). When the player is no longer in Doc Mitchell's house they set a 5 s timer, then `SetStage <MQ00> 10` and (OWB, HH) `StartQuest` the radio quest. Both copies ran
  them, but a copy's run was muted (per machine, never captured), so the copy whose player left the house first had the quests the other did not: `StartQuest 13653` / `SetStage 2FCA` (OWB), `8891` / `E74B` (HH), `3603`
  (LR) `host=0 client=10`, and the host's digest repaired the client back (measured on the pair: a client out of the house showed the OWB radio quest running at 5 s and not running at 10 s, stage 0 although the script
  had set nEnableDLC to 2). The volatile list could not take them: it only has `owner` (script variables) lines for these quests because `volatile_probe.py` refuses to exclude the running / stage / objectives of a story quest.
  Decision: a quest start, stage or objective is the host's, whichever script makes the call. `SuppressPolicy::blockQuestState` drops them on a guest even inside an allow-listed script when that script is one of
  `hostQuestStateScripts()` (the four above; the rest of the script, form lists, the DLC faction, messages, still runs there), without turning the drop into a stage request. On the host the same calls are captured: the hub's
  mute is lifted for the call and an implicit script scope opened (`QuestStateLift` in `engine_hooks.cpp`, `CaptureHub::liftMute`), so the txn (and the stage's own result) reaches the client, which applies it.
  Tests: `plugin/hooks/tests/test_hooks.cpp` (`guest_dlc_setup_script_runs_but_its_quest_calls_are_the_hosts`, `host_dlc_setup_script_quest_calls_are_captured_though_its_effects_are_muted`,
  `host_quest_state_scripts_are_all_on_the_guest_local_run_allow_list`), all three fail in the `NVHOOKS_MUTANT` twin.
* **S05 (the client could not kill its gecko).** Three causes, found with the combat counters and `avatar.status` of both copies (runs 20261028-112856, 113423, 114805 on the second PC):
  1. The geckos were shot as the game leaves them (free, full health): 24 volleys left one at health 11, 53 units away, no line of sight (run 104118). The `kill` action now does what the lab's fight room
     does first: the host (owner) holds the target (`SetRestrained 1`) and sets its health to `story.weak_hp` (5); both copies must show it (`story.weak_s`). Twin `kill_unweakened`.
  2. **The host refused the client's hits: `attacker in another cell: player#1 cell 1064441 (Doc's house), victim cell 924350`.** The DLC setup scripts show their load messages on BOTH copies a few seconds after a
     player leaves the house, and a modal box pauses the avatar layer (`PuppetManager::Tick` returns while a menu is open): the host's puppet of the client stays "Loading3D" in the cell it was spawned in, the
     combat module reads the attacker's cell from that puppet, and every forwarded hit is rejected. Pressing the boxes (they keep coming one after the other) sends both puppets to "Driving" in the right cell within
     two seconds (probe: all puppets `Loading3D 0x00103DF9` for 16 s, `Driving 0x000E1ABE` right after the boxes were pressed). The same stale puppet is the likely cause of the guest talks the host refused as
     OutOfRange / NotLoaded. `settle_world` (story_core.py) presses the boxes on both copies until none shows for two seconds and checks every puppet (`StoryMachine.puppets_follow`, avatar.status) before a shot
     and before a talk. Twin `no_settle_world`. Not a plugin fix: a pause of the puppets while a box is up is how the avatar layer is built; a box that stays up for a minute would freeze them for a minute.
  3. **The host stood in the client's line of fire**: 19 of the client's 21 hits landed on the host's puppet next to the first gecko ("player-vs-player damage is off", `neutralized=19`). The shooter now hops to the far
     side of the target from the other player (`opposite_spot`, 300 units). Twin `stand_anywhere`.
  With the three the host and the client kill their geckos (S05's goal is reached, VCG02 stage 50 on both copies). The step still fails its last check, the clean digest, for a reason that is not this one: see below.
* **S07.** `StoryLiveMachine.item_count` read `player.GetItemCount FalloutNV.esm:13B2BA`; the console takes a hex form id. It uses the bots' `inventory_count` (the lab's way), which turns Plugin:ID into the hex runtime id
  with the copy's load order. Twin `item_console_plugin_id`.
* **Left after the three fixes (run 20261028-114805, S05 to S07 on the second PC).** S05: both geckos dead, VCG02 50 on both copies; S06: the client's talk opens (203 ms) and its pick is said, objective 50 shows on both;
  S07: the client's Broc flower gives `13B2BA` +1 to the client only (`item_count` works). No DLC entry is left in any digest: the client received `SetStage` 10 of NVDLC02/03/04MQ00 from the host
  (`applyremote: SetStage oldworldblues.esm 10`, `lonesomeroad.esm 10`, `honesthearts.esm 10`). S05 and S06 still raise the clean-digest check (`repairs +1/+2`) on one entry: `SetMapMarker 157F7D` flags 1 on the host
  and missing on the client (NorthernPassageMapMarker, `FalloutNV.esm`; no script names it, so it is the engine's own discovery by proximity or a stage result's, and only the host's copy has it).
