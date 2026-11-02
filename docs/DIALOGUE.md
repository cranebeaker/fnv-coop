# Dialogue menu flow

Branch `claude/m1-dialogue`, 2026-10-05. M1 blocker: after Doc Mitchell
walks the player to his front door the conversation could not be left, so
the player could not leave the house.

## The blocker

Data (`FalloutNV.esm`, read with `nvinspect`):

- `VCG01` stage 110 (`DocMitchellREF.evp`, `DisablePlayerControls`) sends
  Doc to the door; `VCG01DocMitchellFarewellDialogueStart` (PACK 00105BD2,
  type 15, `GetStage VCG01 >= 115`, target the player, no topic) then
  opens the dialogue menu on `GREETING`.
- At that point Doc's `GREETING` line is INFO 001057E8 ("Here. These are
  yours.", say once). It has no topics (`TCLT`) but a follow-up (`TCFU`)
  to INFO 001057E4 (the Pip-Boy), which follows up to 001057E5/E6 (by the
  player's sex), which offer topics 083/084/085; their answers follow up
  to INFO 001057E7, flagged Goodbye. Its first result script is
  `GSDocMitchellHouseIntDoorREF.SetDestroyed 0` ("so player can leave"),
  its second `EnablePlayerControls ...` and the `VGenericTimer` event.
- nv-rs did not read `TCFU`. After 001057E8 the menu offered the main
  topic list instead, which has no Goodbye line, and the game's dialogue
  menu has no other way out (its key handler `007628c0` only takes
  code 9, A/Enter). The Pip-Boy, the door and the controls were never
  restored.

## Traced (FalloutNV.exe 1.4.0.525, decompiled; private exports in
`%USERPROFILE%\nv-re\work\dialogue-2026-10-06`)

| Address | What | Used for |
| --- | --- | --- |
| `0061dbd0` | INFO load: `TCLF`, `TCLT`, `TCFU` into the line's conversation data | `Info::follow_ups` |
| `00762ff0` | Menu: next response, or after the last: end script, follow-up, topics or close | `dialogue::after_line` |
| `0061af30` | Matching follow-up info (`TESTopic::GetMatchingFollowUpInfo`, Xbox PDB) | `dialogue::follow_up` |
| `0061a7d0` | Matching info of a topic: per quest, random runs | `dialogue::pick`, `choose` |
| `0061e600`, `0061e720` | Line availability: say once, Intelligence class vs `iDialogueDummySpeakThisIntOrBelow` (`011d0dc8`) | `dialogue::line_available` |
| `00762860` | Goodbye state `+0x2c`: 2 Goodbye flag, 1 `GOODBYE` topic (`0061a2d0(1,2)`) | `dialogue::ending` |
| `00762160` | Close: end script for states 2/3 unless flag 0x08 | `menu_runs_end_script` |
| `0083ebb0` | Begin script unless flag 0x40 | `menu_runs_begin_script` |
| `0083ec30`, `0083ed50`, `0083f110` | Topic list: info's `TCLT` or the player's list; topics 0xFD/0x118 excluded; priority sort | carried out in the next batch (below) |
| `00762950` | Menu update: state 1 cuts a clicked-away voice after 500 ms; state 3 waits the line timer | viewer skip timing |
| `008a20d0`, `008bc590` | Say: missing voice file sets the menu timer to `fDialogSpeechDelaySeconds` (`011d32c4`, exe 2.0) | silent-line duration |
| `0061b320` | NPC say: same matching (`0061a790`); "run immediately" runs script 0 at once | `social::pick_for` |

INFO flag bits as the code tests them (`INFO+0x25`, `+0x26`): 0x01 Goodbye,
0x02 random, 0x04 say once, 0x08 run immediately, 0x20 random end, 0x40 no
menu begin script, 0x80 speech challenge; byte 3: 0x01 say once a day,
0x02 always darken, 0x10 low Intelligence, 0x20 high Intelligence. GECK
names are unverified. `0061e600` first refuses a deleted form (`+8` flag
0x20), then a say-once line whose said byte (`INFO+0x22`) is set; a
say-once first-meeting line with no other conditions therefore wins
until it has been said (Ringo's 00104C5F, B26).

## Implemented

- `world::dialogue`: `TCFU` follow-ups; `follow_up`, `after_line`,
  `ending`, random runs (`choose`), Intelligence classes, flag 0x08/0x40
  script rules. `social::pick_for` uses the same line choice.
- Viewer `dialogue.rs`: after a line, follow-ups are said straight on;
  Goodbye lines and `GOODBYE` answers close the menu; silent lines last
  `fDialogSpeechDelaySeconds` instead of an invented reading pace; a
  clicked-away voice is cut 0.5 s later.

## Tested

`crates/world/tests/dialogue_flow.rs` on `testdata::dialogue` (generated,
shaped like Doc's farewell): follow-up chain to the Goodbye and its
scripts, Goodbye/`GOODBYE`/run-immediately endings, random runs, the
Intelligence filter.

A throwaway program (not committed) walked the real chain through
`world::dialogue` on the installed `FalloutNV.esm` with `VCG01` at stage
115, objective 40 complete and the psych-test greeting said: 001057E8 →
001057E4 → 001057E6 (male player) → topics 083/084 (085 has no line in
the data, so it isn't offered) → 0015E139 → 001057E7 → close; the door
`GSDocMitchellHouseIntDoorREF` is no longer destroyed afterwards.

## Not compared with the original game

Nothing here has been checked side by side with the original yet; the
installed-data route through the door is the next check.

## The speaker, the world and the view during the menu

Branch `claude/m1-dialogue-npc`, 2026-10-06. Private exports in
`%USERPROFILE%\nv-re\work\dialogue-npc-2026-10-06`. Names marked (Xbox
PDB) come from the prototype's symbols: `DialogMenu` (states
`eCameraZoomIn` 0, `eLoadSpeech` 1, `eSpeechIdle` 2, `ePlaySpeech` 3,
`eCameraZoomOut` 4, `eServiceFadeOut` 5; `+0x128 fPercentZoomed`,
`fPackagePercentZoom`), `MenuTopicManager` (`+0x10
bSpeechChallengeLoss`), `MenuTopic` (`+0x8 bTopicIsChoice`). Actor
vtable slots on PC are the Xbox ones + 4 from `+0x258` on (checked
against `Character`'s vtable `01086a6c`: `+0x264` `UpdateInDialogue`
`008a5580`, `+0x280` `InitiateDialogue` `008b2170`, `+0x288`
`EndDialogue` `008b1070`).

| Address | What | Used for |
| --- | --- | --- |
| `0086e650` | Main loop: in menu mode (`00702360`) the process lists aren't updated | everyone but the speaker holds still (unchanged) |
| `00761a20` | `DialogMenu::Create`: speaker's dialogue package zoom (`00672850`, `PKDD` float), first line said at once | `MenuZoom`, `focus_percent` |
| `00762950` | `DoIdle`: zoom in over `fDialogZoomInSeconds` (1.5), out over `fDialogZoomOutSeconds` (0.5), menu destroyed at 0; every frame `FocusOnActor` and, while in dialogue with the PC (`00933840`), the speaker's `UpdateInDialogue` | `world::dialogue_view::MenuZoom`, viewer close after zoom-out |
| `00953060` | `PlayerCharacter::FocusOnActor` (own error text): face node bound (or `Bip01 Head`/`Bip01 Speaker`, radius 32), look point raised by `fDlgLookAdj`, zoom atan(`fDlgFocus` × r / d) × 100 ≤ `fDefaultFOV`, eased over the zoom in; pitch/heading start/stop thresholds `fDlgLookDegStart/Stop`, `fDlgHeadingDegStart/Stop`, rate `fDlgLookMult` | `Focus::frame`, `dialogue::focus_camera` |
| `00fa8d40`…`00fa8e60`, `00f6e610`, `00f6e640`, `00fbc020` | Setting initialisers: 3.2, 13, 0.2, 13, 0.2, −5, 2; 1.5, 0.5; `fDefaultFOV` 75 | `ViewSettings::DEFAULT` |
| `0095de30` | Outside dialogue and V.A.T.S. the field of view returns at 30 ÷ `fIronSightsFOVTimeChange` °/s | `fov_back` after the menu |
| `008a5580` | `Actor::UpdateInDialogue`: not seated, not in combat (`+0x104`), mover not rotating (state 4), > 1° off (`01023128`) → `RequestRotateActor` toward the player (`009dce80`); mover updated with its dialogue flag (`009c9900`) | `speaker_turns`, viewer `ai::move_actors` |
| `0083ec30`, `0083ed50`, `0083f0d0` | `LoadNextTopicList`/`FillTopicList`: a line's `TCLT` in stored order, else the player's topics (`+0x6a8`) sorted by priority (`0083f110`); 0xFD `SpeechChallengeFailure` and 0x118 `InfoRefusal` left out; topic `DATA` flags 0x10/0x20 Intelligence classes | `dialogue::menu_topics`, `answered` |
| `00619030`, `00619410`, `00952830` | `TESTopic::InitItem` adds every topic flagged 0x02 (any kind) to the player's list; `AddTopic` | `Topic::is_top_level` (the `GOODBYE` topic, kind 1, is one) |
| `0083e850` | `DoSpeechChallengeCheck` for lines flagged 0x80: chance from Speech, disposition and difficulty, `rand() % 100`; a loss says the `SpeechChallengeFailure` line and pops back to the previous list | not carried out (see gaps) |

### Implemented

- `world::dialogue_view` (new): `ViewSettings` (game settings and INI),
  `MenuZoom`, `focus_percent`, `Focus::frame` (translated from
  `00953060`), `fov_back`, `speaker_turns` (translated from `008a5580`).
- Viewer: the menu zooms in as it opens and, once to close, zooms out for
  `fDialogZoomOutSeconds` before it goes (world still frozen, nothing more
  said); `dialogue::focus_camera` turns the player's view onto the
  speaker's head and narrows the field of view, then lets it return.
- Viewer `ai::move_actors`: the speaker's in-menu turn follows
  `008a5580` (replaces the inferred "face every frame" rule; combat
  excluded, turn played out once started).
- `world::dialogue`: the main list is the player's topics (top-level of
  any kind plus learned), without the opening line's follow-ups (that
  guess is gone: Sunny Smiles' main list reaches "Goodbye." through the
  top-level `GOODBYE` topic, kind 1, priority 5, her "Until next time.");
  the refusal topics and wrong-Intelligence topics are left out.

### Tested

`crates/world/src/dialogue_view.rs` unit tests (zoom timing, package
zoom, zoom on the head by the end of the zoom in, the FOV clamp, the
start/stop thresholds, no turn while zooming out, the FOV return, the
speaker's turn rule); `crates/world/tests/dialogue_flow.rs`
`the_main_list_is_the_players_topics` on new generated topics.

### Not compared with the original game

None of this has been checked side by side yet; the zoom and turn rates
and the head bound especially need a recording of a conversation.

## Service menus over the conversation (B13)

Branch `claude/b13-barter-menu`, 2026-10-07. Bug: opening barter from
dialogue left the dialogue menu drawn with the barter menu (its box and
text showing through and over it). The game doesn't stack the two: it
hides the conversation while a service menu is up.

| Address | What | Used for |
| --- | --- | --- |
| `0072d250` | `BarterMenu` create: after the file loads, `00763ff0`, then shown (`00a1dc20`: visible 0, fade in, depth `00a1dfb0`) | `game_menus::open_menus` |
| `00726ff0` | Recipe menu create: also calls `00763ff0` | same |
| `0075bc80` mode 3, `007b7570`, `00705870` | A companion's trade, the repair menu, the face menu from dialogue: also `00763ff0` (none of the three is shown in nv-rs yet); containers in mode 1/2 don't | not carried out |
| `00763ff0` | If `DialogMenu` is open and not ending (`+0x2c` 0): `+0x138`, `+0x13a` = 1; the response being said dropped (`0083e4c0(0)`, speaker stops, `+0x7c` done); `00a1d910` on the dialogue menu | `DialogMenu::service_opened`, `cut_line` |
| `00a1d910` / `00a1db20` | Menu fade out (state `+0x24` 2) / in (8) over `menufade`, else `explorefade` (menus default 0.25); out only when visible; trait 6002 decides whether the menu goes when faded | `ui::fade::Fades::start_fade_out` / `start_fade_in` (every menu's fade, [MENU_FADES.md](MENU_FADES.md)) |
| `00716320`, `00711ea0`, `00712450` | The fade list's time ÷ length each frame; fading: visible, tree alpha 1 − t (out) or t (in); out done: state 4, visible 0 (or destroyed with 6002); in done: state 1; `disablefade` tiles only at full alpha | `ui::fade::Fades::frame`, `recursive_fade`, `fade_items` |
| `00762950` | `DialogMenu` idle: state 2 with both flags → 5 (`eServiceFadeOut` (Xbox PDB)); state 5 once faded out → topics reloaded (`00762ff0`), `+0x13a` 0, state 2 | the conversation keeps its list (see gaps) |
| `0072d6d0`, `00727430`, `0075b750` mode 3, `007b78e0`, `007ada40` | Service menu close: `007640a0(topic)`, then its own 6002 and fade out | `game_menus::run_open_menus` |
| `007640a0` | `+0x138` 0; with a topic, the speaker says it; `00a1db20` fades the dialogue menu back in | `DialogMenu::service_closed` |

Input goes to the top menu shown and not closing (`00720e60`), so the
hidden dialogue menu takes no clicks, keys or `--say`/`--choose` picks
while the service menu is up.

### Implemented and tested

- The interface's fades, `ui::fade` (one system for every menu, shared
  with the menus' own fades in and out, [MENU_FADES.md](MENU_FADES.md));
  `DialogMenu::service_opened`/`service_closed`/
  `hidden`; the viewer fades the dialogue menu out when a barter or
  recipe menu is made and back in when it closes; the talk system cuts
  a line in progress and chooses nothing while hidden.
- Tests: `menu::tests::menus_fade_out_and_in`,
  `draw::tests::a_fading_menu_is_drawn_at_its_alpha`,
  `menus::dialog::tests::a_service_menu_hides_the_conversation_until_it_closes`,
  `menus::dialog::tests::a_service_cuts_the_line_but_not_an_ending`.
- Verified live (release viewer, `GSGeneralStore`, `--talk --say "for
  sale"`): Chet's barter menu alone on screen; after Exit (clicked with
  `--menu-pointer 1510,833 --key-at 37 mouse-left`) the dialogue menu is
  back at full alpha with the same topics ("Show me what you have for
  sale." dimmed as said). Not compared with the original game.

### Gaps

- `007640a0`'s topic: after a trade (`BarterMenu +0x11c`) the speaker
  says the `BarterExit` service topic (`0061a2d0(5, 4)`; `RepairExit`
  `(5, 5)` after repairs); how it's said (no menu text, the
  `DialogueItem` made and freed at once) isn't traced, so it's left out.
- State 5's reload of the topic list once hidden isn't repeated: the
  list the conversation computed when the line ended is kept.
- Menus other than the dialogue menu still open and close at once (the
  barter menu's own fade in, `00a1dc20`, and every menu's close fade).
- Companion trade, repair and face menus aren't shown, so their
  `00763ff0` calls aren't either.

## After the conversation: the package talks again (B12)

Branch `claude/b12-doc-dialogue`, 2026-10-07. Playtest bug: after Doc's
door conversation, leaving it started another at once, forever. Private
exports in `%USERPROFILE%\nv-re\work\b12`. Names marked (Xbox PDB); PC
process vtable `01087864` slots match the prototype's `BaseProcess` ones.

### The data

`VCG01DocMitchellFarewellDialogueStart` (00105BD2) stays valid while
`GetStage VCG01 >= 115`. The Goodbye line 001057E7's end script activates
`VCG01CasualHardcoreMessageREF` (its `OnActivate` asks about hardcore mode,
a message box) and starts `VGenericTimer` (quest delay 0.1, `fTimer` 0.1,
event 3): `SetStage VCG01 200` (complete quest flag; `Set GameHour to 8`,
`StopQuest VCG01`) and `VCG02` stage 5. Doc's list puts
`VCG01DocMitchellSandbox` (`GetQuestCompleted VCG01`) before the farewell,
and the new game hour has his package looked at again.

### Traced (FalloutNV.exe 1.4.0.525)

| Address | What | Used for |
| --- | --- | --- |
| `008e8600` | DIALOGUE_ACTIVATE: a type-15 package (not Say To) only calls `InitiateDialogue` (actor +0x27c); no distance gate, no "already talked" test | `world::ai::talk` |
| `008b19c0` | `InitiateDialogue`: `SavePackageToExtraData` (process +0x710, `009130f0` → `0041c930`, extra 0x19: package, step, target, flags), made type-0x1c package installed (`PutCreatedPackage`, actor +0x2f4, `0087eac0`) at step 1 (`SetCurrentProcedureIndex`) | the menu opens one update later |
| `008e9640` | ACTIVATE (from the made package's DIALOGUE_ACTIVATE): in reach and `CanForceGreet` (process +0x3fc, `008da420`: not in VATS, the speaker detects the player, player not swimming, the player's anim action none or 7) → activates the player | as above |
| `00762160` | Menu close: `EndDialogue` on the speaker, then `CheckforNewPackage` forced (process +0x24, `008da670`) before and after the end script; `SetGreetingTimer` | viewer `move_actors` |
| `005fa330` | The activated NPC's base `Activate`: when the activator's saved package (extra 0x19) is a dialogue package (type 15), it is saved again at step 4, list 10's closing DONE (`0041c930(package, 4, target, 0, 0, 0)`), and `PackageDone` (process +0x5a0: end action; "once a day" notes the day, actor +0x28c); then the dialogue menu is asked for (`00709470(4, ...)`, opened outside menu mode, `007094f0`) | `DialogueRun::talk` marks the package done; viewer asks its end action |
| `008b1070` | `EndDialogue`: the made package current → `LoadPackageFromExtraData` (process +0x714, `00913250`): the saved package back at its saved step (a step past the list's end would be 0): DONE after a talk; list 10 doesn't step back from DONE (`008eeec0` case 0x36) | `DialogueRun::conversation_over` |
| `0090a1a0`, `00907700` | Package evaluation: the list's pick (`0067e780`) equal to the current package → no new start, step kept, even when forced | viewer `rethink` no longer restarts |
| `008ec460` | DIALOGUE procedure (NPC conversations): sets the saved type-15 package's action-complete byte, ends with `EndDialogue` | NPC chats end the same way |
| `005ab400`, `005ac1e0`, `0059c430` | Quest scripts (run every frame from the main loop, `008705d0`): a quest with its own delay starts with its countdown at 0; `GetSecondsPassed` = time gathered since the last run, else the frame's | `Runner::update` first run |
| `0086e650`, `00702360` | In menu mode the process lists aren't updated | AI held still while any menu (message box, Pip-Boy) is up |

So a dialogue package talks once: talking to someone finishes it, and the
conversation's end restores it finished. It talks again only when a
package starts anew (a different one picked, then this one again).

### What was wrong

The viewer's forced look at the package on close restarted the dialogue
package (an untraced rule) and its Talk step opened the menu again in that
same update. No game time passed between conversations, so
`VGenericTimer` never ran and the loop never ended. Its AI also ran under
the hardcore message box, so the new conversation opened beneath it.

### Implemented

- `world::ai::talk::DialogueRun`: travel done, conversation package made
  (an update with no menu), talk under way, done; `conversation_over`
  restores the saved step (DONE after a talk).
- Viewer: the same package goes on as it was on any look (forced or
  not); activating the player (or starting an NPC's conversation) asks
  the package's end action and marks it done; menu close and NPC
  conversation ends call `conversation_over`; AI held still in menu mode.
- `Runner::update`: an own-delay quest's first run passes the frame's
  seconds.

### Tested

`world::ai::talk` unit tests; `crates/world/tests/scripting.rs`
`an_own_delay_quests_first_run_counts_the_frame`; viewer
`a_dialogue_package_talks_once` (the old code opened a new conversation
on the close's own update).

### Verified live (release viewer, installed data)

Route: `GSDocMitchellHouse --stage VCG01 110 --run "SetObjectiveCompleted
VCG01 40 1" --run-at 3 "DocMitchellREF.StartConversation player"` (the
psych test said first, as in the real route), the psych test and the
farewell answered with `--say`, the hardcore box answered "No" with
`--menu-pointer 963,809 --key-at 112 mouse-left`. Seen: the farewell
package talks at the door (001057E8 → … → 001057E7), the hardcore box
shows over a still Doc, and after it no conversation starts; 0.1–0.2 s
later `VGenericTimer` sets VCG01 200 (the main quest's and "Talk to Sunny
Smiles" objectives show) and Doc's sandbox is picked. The screenshot
after it is the hallway with the HUD, no menu. Before the fix the same
route reopened Doc's conversation ("Welcome back.") at once, forever, and
under the hardcore box.

### Not compared with the original game

Nothing here has been checked side by side with the original. Doc's
sandbox walk after the farewell sticks at (2292, 2311) behind the player
in the doorway (not part of this fix). Not carried out: "once a day"
packages' day note (actor +0x28c), whether DONE runs the end action a
second time (`008eeec0` case 0x36 with `IsPackageDoneOnce`).

## Greetings, and activating someone who only says a line (B10)

Branch `claude/b10-greetings`, 2026-10-07. Playtest bug: people greeted
too often, sometimes as the player started talking to them; some people
should only say a line when activated. Private exports in
`%USERPROFILE%\nv-re\work\b10`. Vtables read from the exe file:
`HighProcess` (Xbox PDB) `01087864`, `Character` `01086a6c`,
`PlayerCharacter` `0108aa3c` (RTTI names); package type names from the
table at `0119bcb0` (21 "Alarm", 22 "Flee", 0x1c "In Game Dialogue").

### Greeting the player (`008eeec0`, each high-process update)

| Address | What | Used for |
| --- | --- | --- |
| `008eeec0` (`008ef5cc`…`008ef6c0`) | First gate: not in combat (+0x104), fleeing (`008a6650`), unconscious (life state 3), knocked down (knock state, process +0x40c), saying something (process +0x48c slot 0: the voice handle `00934250` stops), greeting already (process +0x32c), and nobody saying a GREET line to the player (player +0x6cc). Fails: no greeting and no idle chatter | `GreetingCheck::busy` |
| `008ef715`…`008ef808` | Near: detection > 0 (`008a0d10`), not asleep (actor +0x1ac == 9), the player not sneaking (move flags 0x400 without 0x800, `004997b0`) or trespassing (player +0x1c0, `008d2a40` → `00546da0`), within `fAIMinGreetingDistance` (150, exe; not in the data). Near: no idle chatter at all | `Social::greeting` → `Near`/`Greet` |
| `008ef80e`…`008ef8e4` | Greet: the player not in combat (+0xdf0), the package allows hellos (`008a78f0(0)`: general flag 0x1000 set and behaviour flag 0x01 clear forbids), not an alarm package (21), not standing still in a made conversation package (0x1c, `009336c0`/`004938e0`), greeting timer (process +0x330) ≤ 0 → `008bc3d0(HELLO)` | `Greeting::Greet` |
| `008bc3d0` | Greet: only when the player's cooldown (+0xe24) is negative (`008bc520`); then the GREET procedure and the cooldown noted as `GetTickCount` (`008bc560`) | `HelloCooldown::free`, `greeted` |
| `00944179`…`009441e4` (player update, no function in the database) | +0xe24 back to −1 once `fHelloCooldownTime` (30 s, `00f65740`; not in the data) × 1000 ms have passed; −1 from the start (`00938180`) | `HelloCooldown::update` |
| `008dbe30` | GREET: with a listener, process +0x330 = `fAIGreetingTimer` (20 s) on every update while the line is said (also when no line was found), player +0x6cc set (`008dd880`) and cleared at the end (`00953ce0`) | `Lines::say_to`, `greeting`, `spoken_to`; the timer held while the line lasts |
| `008ef8ee`…`008ef9af` | Idle chatter only when not near: counts down, also while in a made conversation package; due with no package, or a package that isn't a dialogue package and has behaviour flag 0x80 (`0067abd0`) | `Social::chatter_due` |
| `008efa4e` | Conversations with others only when the GREET flag is clear and nobody speaks to the player | viewer `social_frame` |

So one person greets the player at a time, and after a greeting nobody
else greets for 30 s (real time); the greeter waits 20 s after the line.
The viewer greeted with only the 20 s per person, began the timer when the
line began, let everyone near greet at once, and ran idle chatter for
anyone whose package forbade it.

### Activating someone (`005fa330`)

| Address | What | Used for |
| --- | --- | --- |
| `005fa9be`…`005faa8f` | The player activating: the `GREETING` line found (`0061a2d0(0, 0)`, `0061b320`); flagged Goodbye (`00619df0`, INFO +0x25 & 1) with a single response (`0083c7b0`, `0083c7e0`) → `0057b7c0`: the actor says the topic (picked again) to the player through GREET, no menu; else `00709470(4, …)`, the dialogue menu; no line: nothing | `dialogue::activation_says_a_line`, viewer `talk` |
| `0057b7c0`, `005c9100`, `008a20d0` → `00934250` | Saying (and a script's `SayTo`) first stops the speech in progress | viewer `talk` hushes the speaker's line as the menu or line starts |

Goodsprings settlers (`GSSettlerCM` and the like) greet with 0015D8AA
"Way too many strangers coming into town these days. No offense." (DATA
flags 0x03: Goodbye, random; one response): activating them only says a
line. Easy Pete, Trudy, Chet, Sunny open the menu.

### Implemented

- `world::social`: `GreetingCheck`, `Greeting`, `Social::greeting`,
  `HelloCooldown`, `package_forbids_hellos`, `package_allows_chatter`;
  `chatter_due` takes the package's chatter flag and the made package.
  `world::dialogue::activation_says_a_line`. `world::ai::kinds::ALARM`.
  `DialogueRun::conversation_package_made`.
- Viewer: `social_frame` asks the whole check, the cooldown on the real
  clock, holds the greeter's timer while a GREET line (or a line said on
  its own) lasts; `Lines::say_to` for greetings and "Say To" lines; `talk`
  says a one-response Goodbye greeting without the menu and stops the
  speaker's line before saying.

### Tested

`world::social` unit tests (the decision, busy people, what keeps a near
person quiet, the package flags, the cooldown, chatter flags);
`crates/world/tests/dialogue_flow.rs`
`a_one_response_goodbye_greeting_is_said_without_the_menu`; viewer
`one_greeting_at_a_time_and_none_till_the_cooldown_is_over`.

### Verified live (release viewer, installed data)

- `WastelandNV --at -70618,-1000,8200,180 --talk` (Goodsprings Settler
  01): "only says a line", 0015A47D "Hey there." said with no dialogue
  menu (the HUD alone in the screenshot). `--at -67845,3250,8400,0 --talk`:
  "Dialogue menu: talking to Easy Pete."
- Walking past (11 `player.MoveTo` over 115 s between settlers 01–03 and
  Easy Pete): 3 greetings, all Easy Pete's, 35 and 31 s apart. Settler 03
  (`GSSettlerSandbox`) never greets: its package forbids hellos (general
  flag 0x1000, behaviour 0x01 clear); the settlers' packages don't allow
  idle chatter either.
- Standing at Easy Pete: one greeting at 0.5 s; at 13–18 s he was ready
  again (timer out, detecting the player) and the 30 s cooldown held him.
  From about 20 s his detection of the player standing on him fell to −10
  (the detection rules, not this fix), so no second greeting.

Not compared with the original game, and the old build wasn't replayed
on these routes.

### Not carried out

The GREET line's subtitle path in `0057b7c0` (`00705210`). (The four
conditions left out here are decoded under B33 below.)

### B33: the four other conditions, and the look a greeting leaves

Branch `claude/b33-greetings-chatter`, 2026-10-08. Playtest report: people
greeted on their own, and every 30 s. Names from the Xbox PDB
(`Fallout_Release_MemDebug.pdb`, `pdbdump`); the PC's `TESForm` lacks the
prototype's editor ID string, so PC offsets after it are the Xbox's less
0x10 (player +0x6cc `bGreetingPlayer` is Xbox +0x6dc, +0xe24
`fLastHelloTime` Xbox +0xe34). Vtable slots read from the exe.

| Address | What | Used for |
| --- | --- | --- |
| `00969860` | `PlayerCharacter::IsImportantConversationRunning`: +0x224 `pAIConversationRunning` (a `DialoguePackage`) with its +0x94 `pTalkingActor` set. In the first gate (busy) and before conversations with others (`008efa4e`) | `GreetingCheck::important_conversation`; the viewer has no such conversation (talking with the player opens the menu, the world stands still), so false |
| `008ef78c`…`008ef7b1`: Character `+0x2c8` `008815a0`, `00881570` → process `+0x35c` `008d8310` | Near only when their target (process +0x40 `pTarget`, `GetTarget` `+0x128` `008d6f30`; set as a package starts, `0090a1a0` `006780e0` → `+0x12c`) isn't the player, or they run a run-once package (`IsRunningRunOnce`: `GetRunOncePackage` `+0x20c` → +0xe4 not null) | `target_is_player` (the package's reference target, or a `StartConversation` walk to the player), `running_run_once` (run-once packages aren't carried out: false) |
| `008ef7b7`: process `+0x66c` `00901460` | `CanSetActionHeadTrackTarget`: none of `HeadTrackingTargetFlags` 1–5 (+0x411…+0x415) set. Else not near | `head_track_asked`, `HeadTrack::action_free` |
| `008ef841`: `008a69d0` → process `+0x4e0` `008d9050` | `ContinuingPackageforPC` (+0x374): no greeting. Set by `SetContinuingPackage` (`+0x4e4`) in `0090a1a0` when a package with general flag 0x200 ("continue if PC near", `008840f0`) would have given way | `continuing_for_player`; not carried out in the package choice, so false |
| `008ef88d` | No greeting while their target (`+0x128`) is the player | `target_is_player` |
| `008bc3d0` | Before the cooldown check: the player put in their ACTION head-track slot (`+0x628`), whether the greeting is said or not. Said, the GREET procedure's end clears it with demote (`008dbe30`, `+0x644(1)`); not said (cooldown, or no line: `008dbe30` only resets the timer and the flag), it stays till the head-track update empties the slots (`008a3100` step 4: the player no longer detected) | viewer `social_frame`, `Walker::greet_look`, `head_track_asks` |

So someone ready to greet while the cooldown holds (or with nothing to
say) looks at the player and isn't near again, so neither greets nor
loses the look, until they lose sight of the player; meanwhile the idle
chatter path runs for them. Someone who greeted is free again after
their line: standing by them, they greet again once their 20 s and the
player's 30 s are over (traced; still to compare with the original).

Conversations with others (`00904800`, read again): only with a
package that isn't one the game makes (`00678610`: types 0x12…0x24 but
0x1a and 0x1e) and doesn't forbid them (`008a78f0(1)`: general flag 0x1000
with behaviour flag 0x02 clear, `0067a8d0`): `made_package_kind`,
`package_forbids_conversations`. Its candidates are the people in their
detection list (+0x25c) at level 3 (`piVar[1] == 3`); the viewer still
offers everyone near (its detection has no such levels).

Idle chatter (`008ef8ee`…`008ef9af`): `fIdleChatterCommentTimer`
(`011cd008`, exe 5, data 15) and `…Max` (`011cd18c`, exe 30, data 60), set
by `00f5a590`/`00f5a5c0`; `IdleChatter` said through `ProcessGreet`
(`+0x2a4`) with no topic listener: the GREET procedure's listener is the
actor's +0x70 (`pDialogueItemTarget`, Xbox +0x80; `004fd380`), else the
last one (process +0x370). The viewer still asks the lines' conditions
with the player as listener.

Verified live (release viewer, installed data, 240 s): `WastelandNV --at
-70618,-1000,8200,180 --answer-boxes`, `player.MoveTo` settlers 01–03
then Easy Pete (standing there 60 s), three rounds. Build 25 (≈ main):
9 greetings, Easy Pete every 30 s while the player stood by him (38, 68;
127, 157; 216, 246 s). This branch: 5, Easy Pete once a visit (39, 126,
217 s): 20 s after his line he was ready while the cooldown still held,
so he looked at the player and stayed so; settler 01 twice (9, 96 s), not
on the third round. No idle chatter or conversations with others on
either (the settlers' packages forbid both). Not compared with the
original game.
## Silent lines after skipping: the voice file's name (B11)

Branch `claude/b11-voice-skip`, 2026-10-07. Playtest bug: after skipping
some lines, the next lines showed their text but played no voice. Private
exports in `%USERPROFILE%\nv-re\work\b11`.

### What was wrong

Not the skip. Reproduced in the release viewer on Doc's psych test: the
skipped `GREETING` line's voices played and were cut, then every answer
(`House.`, `Night.`, ...) and later the Pip-Boy line 001057E4 were silent,
skipped or not. Their voice files are `vcg01_vcg01docmitchelltopi_…` in
`Fallout - Voices1.bsa`; nv-rs looked for `vcg01_vcg01docmitchel_…`. The
game's name rule (`006172c0`, called by the response's file name builder
`00617400`, `"%s_%08X_%u"`): quest and topic editor IDs whole when
together at most 25 bytes; else, with a quest under 11 bytes, the quest
whole and the topic cut to 25 − the quest's length; else the quest cut to
10 and the topic to 15. nv-rs always cut 10 + 15, so every line of a
short-named quest with a long topic name (the psych test's
`VCG01DocMitchellTopic0xx` topics: Doc's answers and follow-ups) was
silent. The skip itself already matched the game: a click moves the menu
to state 1 (`007624f0`), which stops the voice 500 ms later (`00762950`
case 1, `008bc590`), and the next response starts once the speaker is done
(`00762ff0`).

### Fixed and tested

- `world::dialogue::voice_name_parts` translated from `006172c0`; used by
  the dialogue menu and by NPC chatter (`chatter.rs`).
- Test: `crates/world/tests/impacts.rs`
  `a_short_quest_leaves_the_topic_the_rest_of_25_letters` (the old rule
  gave `vcg01docmitchel`).
- The viewer now logs `voice started`/`playing`/`stopped`/`ended`, `line
  skipped` and `no voice file` for each response.

### Verified live (release viewer, installed data)

`GSDocMitchellHouse --stage VCG01 110 --run "SetObjectiveCompleted VCG01
40 1" --run-at 3 "DocMitchellREF.StartConversation player"`, fourteen
`--say e`, `--menu-keys` Enter each second from 38 to 85 s (skipping every
line): every response from the psych test through 001057E8, 001057E4
(three) and 001057E6 logged `voice started` and `voice playing` with their
`vcg01_vcg01docmitchelltopi_…` or `vcg01_greeting_…` file, and each skipped
one `voice stopped`. Before the fix the same route logged no voice for
any answer after the first skipped greeting. Not compared with the
original game; Sunny's lines weren't run (her quest, `VCG02`, names are
also short, so the same rule applies).

## Remaining gaps

- The head's bound: the game merges the face node's skinned pieces'
  bounds; here a box-middle/farthest-vertex sphere over the head parts as
  skinned now (an approximation, labelled in `dialogue::head_bound`).
- Not carried out from `00953060`/`00761a20`: the depth of field
  (`fDialogFocalDepthRange` 300, `…Strength` 0.65 × percentage), the
  forced first-person view (`00951a10`, `00950110`) and its restoration,
  a seated player's look offset (`+0x6e4`), the menu tiles showing only
  from 10 % zoom (`00762950` state 0).
- Which update calls `0095de30` (the FOV return) is untraced; the
  first-person view model's own FOV isn't zoomed (it is hidden during
  conversations).
- The speaker's talking idles in the menu are now traced and played
  ([ANIMATION.md](ANIMATION.md)); the face emotion handling in
  `UpdateInDialogue` (`+0xb4` cases 1/5/8) and idles for lines said
  outside the menu are not.
- `0083e850`'s random speech challenge (0x80 lines), the loss pop-back,
  XP and disposition change; rumors (topic flag 0x01, `0042df90`);
  `InfoRefusal` lines; say once a day (`00935a40`); the `+0x24`
  next-speaker rules; the pause after a voiced line (`008bc590`).
