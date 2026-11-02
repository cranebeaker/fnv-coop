# Dead Money handoff

For anyone picking up the Dead Money work in nv-rs. Written 2026-10-06, in plain words. It describes
what exists, what to review in what order, and what a newcomer can do first. It contains no
decompiled code, no disassembly and no game bytes; keep it that way.

## 1. Where things stand

Dead Money has 57 quests in `DeadMoney.esm`. The coverage matrix (`docs/DEAD_MONEY_COVERAGE.md`, on
branch `claude/dm-coverage`) says:

* 4 PLAYED, 18 PARTIAL, 34 NOT PLAYED, 1 NEVER STARTED (`NVDLC01MQ03d`: nothing in the data starts
  it, so the original never plays it).
* All 199 script functions Dead Money calls are carried out (`nvinspect <Data> functions`), so no quest
  is blocked by a missing function; the blockers are places not visited, features not built and
  untested behaviour.

**By hand (keys and mouse) in the viewer:** the bunker intro and the slideshow, arriving at the Villa
fountain, Elijah's talks, sitting in Dean's chair and his greeting, Christine's conversation and
recruiting, the Villa walk, the casino loading and walking about.

**By script only (`--run`, `--use`, `--choose`):** the clinic basement terminal and the Auto-Doc scene,
Dog's replies to their endings, the substation terminal, the casino power switch (E on it without
walking there), the recruits' final stage lines. These show the rules work; they do not show a player
can do it with the keys. The matrix says for each quest which parts were which.

**Not built:** casino games (slots, roulette, blackjack menus), the hacking game, radio station
playback, the Cloud's look, Dean's rooftop and Christine's switching station (MQ02b, MQ02c), the Gala
itself (MQ02) and everything in MQ03 after the casino power switch.

## 2. Branches waiting for review

Nothing is merged on `origin/main`. Review in this order. "Base" is what to compare against; a
branch with a base other than `main` must not be merged before its base.

### 2.1 Engine changes (change existing engine code): review first

| Branch | Base | What it does | Tests | Guesses [G] |
| --- | --- | --- | --- | --- |
| `claude/dm-voice` | `claude/dm-intro` | Voice playback: lines decode their `.ogg` with the project's own decoder (see `docs/DEAD_MONEY.md`, "Playing Act 1"). | unit tests in `world`, `cellview` | none recorded; check the branch's commits |
| `claude/dm-act2` | `claude/dm-voice` | Player sits in furniture; essential people go down instead of dying; idle teammates follow the player; shared texture and material caches (fixed an intermittent graphics crash). | tests in `world` and `viewer` | follow distance (200 units); the 12 second get-up and quarter health of a downed essential person; eye height when sitting |
| `claude/package-end-action` | `main` | A script's Travel package that has finished asks for its End action, whose script runs. | `world` tests `script_package_actions` | that a Travel package ends on arrival (or at once when it has nowhere to go) |
| `claude/teammate-wait` | `claude/dm-act2` | A guard package is the "wait here" order: a waiting teammate stays. | `a_teammate_told_to_wait_stays_and_one_with_nothing_to_do_follows` | that guard beats the follow rule |
| `claude/script-cell-grid-lag` | `claude/dm-intro` | A script moving the player into another worldspace waits for the outdoor grid before loading the cell's objects and triggers. | `an_outdoor_grid_that_lags_a_scripted_move_is_waited_for` | none (a fix) |
| `claude/brought-in-talkers` | `main` | People brought into a place after it loaded (companions, moved people) join the list the triggers, sight tests and E use. | `people_brought_in_are_listed_once_and_those_without_a_base_are_not` | none (a fix) |
| `claude/dialogue-info-links` | `main` | A topic also says the lines its `INFC` list names (they stand under another topic). | `a_topic_with_no_lines_says_the_lines_connected_to_it` | that such lines are picked like the topic's own; the `nvinspect dialogue` listing of Dog and Sunny Smiles is unchanged by it |

**Known failing test on `claude/teammate-wait`.** `objects_animations_playing` (in
`crates/world/tests/more_functions.rs`) fails on `claude/teammate-wait`, and also on its base
`claude/dm-act2`, without any change from this branch. The fix is in `claude/dm-verify` (commit
`9ba1478`, "finished one-shot groups stop counting as playing"), which is higher in the stack. So:
merge `claude/dm-verify` (with the stack below it) before, or together with, `claude/teammate-wait`, or
expect that one red test. Suggested pull request text for `claude/teammate-wait`: "Based on
`claude/dm-act2`. `objects_animations_playing` fails here only because its fix is in `claude/dm-verify`
(9ba1478); merge that first."

### 2.2 Shared pieces (reusable by other games or DLCs)

In stack order (each is based on the one before it):

1. `claude/dlc-dead-money` (base `main`): DLC load handling and the research pass (`docs/DEAD_MONEY.md`).
2. `claude/dm-radio`: radio stations, ambient music, collar radio.
3. `claude/dm-los-anim`: `IsAnimPlaying` and `GetLineOfSight`.
4. `claude/dm-shaders`: effect shaders (`PlayMagicShaderVisuals`, `StopMagicShaderVisuals`).
5. `claude/dm-terminal-back`: `ForceTerminalBack`.
6. `claude/dm-caravan-cards`: caravan card functions.
7. `claude/dm-companions-actors`: companion and actor functions (teammate container, push, disposition, cause of death).
8. `claude/dm-dispel`: `DispelAllSpells`.
9. `claude/dm-traps`: `SetVATSTarget`, `FireWeapon`.
10. `claude/dm-conditions`: `GetVATSValue`, `IsFacingUp`.
11. `claude/dm-menus`: the script side of the recipe and casino menus.
12. `claude/dm-entry`: the test-character loader (`--character`) and the bunker entry.
13. `claude/dm-intro`: the intro slideshow, the Villa start, the SayToDone chain and texture swaps.
14. (`claude/dm-voice`, `claude/dm-act2`: engine, see 2.1.)
15. `claude/dm-verify`: fixes from checks in the original game (`IsAnimPlaying` end of group, `GetLineOfSight`).
16. `claude/dm-crafting`: `world::crafting`, the recipe menu (`ui`), `nvinspect craft`, `--open-menu recipes:CATEGORY`.

Each has tests in the crate it changes (`cargo test -p world -p ui -p testdata`, and `cargo test` in
`viewer/`). Guesses in crafting are listed in `docs/DEAD_MONEY.md` ("Crafting"): recipe listing order,
the skill rule, click to make.

Also based on `main`, small: `claude/viewer-use-flag` (`--use REF` presses E on an object once the
place is loaded, for unattended tests; test in `args.rs`).

### 2.3 Dead Money content (docs, test characters, matrix)

* `claude/dm-casino-character` (base `claude/dm-crafting`): `characters/dead-money-casino.txt`,
  `characters/dead-money-act2.txt`, and the docs for Dog's escort and the casino.
* `claude/dm-coverage` (base `main`): `docs/DEAD_MONEY_COVERAGE.md`.
* `claude/dm-handoff` (base `main`): this file.

## 3. What depends on what

* The stack (2.2) is one chain: `dlc-dead-money` → `dm-radio` → `dm-los-anim` → `dm-shaders` →
  `dm-terminal-back` → `dm-caravan-cards` → `dm-companions-actors` → `dm-dispel` → `dm-traps` →
  `dm-conditions` → `dm-menus` → `dm-entry` → `dm-intro` → `dm-voice` → `dm-act2` → `dm-verify` →
  `dm-crafting` → `dm-casino-character`. Merge bottom to top.
* `claude/script-cell-grid-lag` needs `dm-intro` (the outdoor script refresh it fixes was added there).
* `claude/teammate-wait` needs `dm-act2` (the follow rule was added there) and should come with
  `dm-verify` (see the failing test above).
* Independent, based on `main`: `package-end-action`, `viewer-use-flag`, `brought-in-talkers`,
  `dialogue-info-links`, `dm-coverage`, `dm-handoff`. They can merge in any order. The Dog escort in the
  viewer needs all of them plus the stack to play end to end.

## 4. Starter tasks for a newcomer

Marked [C] if they need the original game with Dead Money installed. Your Dead Money load index is the
two digits in front of its form IDs (the maintainer's is 02); the console does not take reference
editor names, use `prid <formid>`. The ones first in the list need no unmerged branch.

1. **Confirm `NVDLC01MQ03d` is never started** [C]. Console: `GetStage NVDLC01MQ03d` on a Dead Money
   save, again after pulling the casino's electrical switch. Expect 0 both times and no quest of that
   name in the Pip-Boy. Write down the numbers. If not 0, something starts it that we missed.
2. **Does Dog start talking by himself at the Salida del Sol substation trigger, and does he repeat?**
   [C] Steps are in the maintainer's test note (Test D) and use `prid 02001306`, `prid 02005D15`,
   `Activate 02001306 1`, `player.moveto 0200ADC5`. Write down seconds to first line and whether he
   starts again after you finish. nv-rs starts him after 1.4 s and repeats at about 20 s (a guess).
3. **Does telling Dog to wait keep him where he is?** [C] Hire him as in task 2, tell him to wait,
   walk 30 steps, `player.getdistance 02001306`, tell him to follow. nv-rs: he stays and follows again.
4. **Check a PARTIAL row of the coverage matrix against the original** [C]. Pick a quest, play it
   there, and compare each "Played" and "Not played" note; write corrections as a pull request on
   `docs/DEAD_MONEY_COVERAGE.md`.
5. **Add a missing check to the matrix generator's rules.** The matrix is generated from a script that
   is not in the repository; adding it (a small Python script reading `nvinspect list QUST` and the
   notes) is a docs-only task. Goal: `python tools/matrix.py` reproduces the file. Check: the diff is empty.
6. **Review any branch of 2.1 against the sources** and write down what you checked in the pull request:
   its test fails without the change and passes with it; no unrelated edits; guesses marked [G].
7. **Find which of the 1,753 topics in the base game that have `INFC` lines and no lines of their own
   now say something** (needs `dialogue-info-links`): use `nvinspect <Data> dialogue <ID>` before and
   after. Goal: no NPC's greeting changes unexpectedly. Check: compare outputs for ten NPCs.
8. **Walk the casino by hand** (needs the stack, `dm-casino-character`): from the launcher line in
   section 5, find the way to the electrical switch (about x -268, y 2822) and the terminal item
   "Unlock Electrical Closet Door". Record the route in `docs/DEAD_MONEY.md`. nv-rs only.
9. **Escort Dean (MQ02b), then Christine (MQ02c)** (needs the stack and the four small branches): follow
   the Dog section of `docs/DEAD_MONEY.md` ("Act 2") as the model: trace what the quest's scripts and
   dialogue do, play it with `--run`, `--choose` and a test character, mark guesses.

## 5. Building and running the viewer

The viewer is its own workspace (Bevy 0.16). From the repository root:

```
cd viewer
cargo build --release
```

A first build takes 15 minutes or more; later ones 2 to 4. Run it with the game's Data folder (never
commit anything from it):

```
viewer\target\release\nv-viewer.exe "<path to>\Fallout New Vegas\Data" <CELL> [options]
```

Cells and options used for Dead Money:

* Villa arrival: `DLC01StartMarker --character characters\dead-money-villa.txt`
* Bunker intro: `SLBoSBunkerINT --official --character characters\dead-money-entry.txt`
* Casino: `NVDLC01Casino --official --character characters\dead-money-casino.txt`
* Dog at the Gala trigger: `NVDLC01EastTownS --official --at -150,-2119,1138,0 --character characters\dead-money-act2.txt`

Options for unattended tests:

* `--character FILE`: start as a ready-made test character. The file is the game's script lines, one
  per line (editor IDs only), plus `level N`. `characters/README.md` lists the files.
* `--use REF`: press E on a placed object once loaded (a form ID or editor ID).
* `--run "LINE"`: run a line of the script language once loaded, as the console would; can be given
  many times. A multi-line block is one argument. `Done` is printed, not a value: to read a value,
  use it in an `if` that shows a notice.
* `--choose N,N,...`: pick these dialogue replies in order, as the number keys would; with `--talk`
  (talk to the nearest person) or when a script starts a conversation.
* `--screenshot FILE --wait SECONDS`: run for that long, save a picture, quit. `--at X,Y,Z,HEADING`
  stands you at a place (the numbers the console's `getpos` and `getangle z` give).
* Environment `NV_LOAD_RADIUS=N`: how many squares around the player an outdoor place loads (the
  dense Dead Money towns load 3 by 3 by default; raising it can exhaust the graphics).
  `NV_SYNC_RENDER=1` draws in step with the frame.

`nvinspect <Data> <command>` (`show`, `source`, `scripted`, `dialogue`, `ai`, `craft`, `play`, ...)
looks inside the data without the viewer; `nvinspect` with no arguments lists the commands.

Tests: `cargo test -p world -p ui -p testdata` from the root; `cargo test --release` inside `viewer/`.
Format and lint: `cargo fmt` and `cargo clippy` in both places.

## 6. Rules

* **No decompiled code, disassembly or executable bytes in this repository**, and no game assets
  (text, models, sounds, textures). Describe behaviour in your own words; function addresses in prose
  are fine. Never write a path into the private research folder into any file here.
* **Tests for every change.** If something can only be checked by playing, say so in the pull request
  and add it to the maintainer's test note.
* **One engine change per branch**, from `main` where it does not need other unmerged work, with a
  short description saying what was copied from the original's behaviour and what was guessed [G].
  Dead Money content (docs, test characters, matrix) goes on its own branch.
* Mark guesses [G] and things to check in the original game [C], in code comments and docs.
* Commit with your own identity; don't change git config in someone else's checkout.
