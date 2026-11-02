# Terminal hacking: working evidence

Using a locked terminal opens the hacking game: a screen of words hidden
in garbage, the password among them, a few guesses, each answered with
how many letters are in the right place. Until 2026-10-06 nv-rs had a
guess in its place (the terminal opened when Science reached 25 × its
difficulty, no game).

## Sources

`FalloutNV.exe` 1.4.0.525 (SHA-256 `19406942...739F`), read in Ghidra
12.0.4 from the full private export, plus whole-function disassembly for
the `this` pointers the decompiler loses. Names from the Xbox 360
prototype's symbols (Xbox PDB, read with `llvm-pdbutil` after
[research/pdb](../research/pdb/README.md)); its `HackingMenu` (476 bytes)
has every field at the same offset as the PC's. The dictionary
(`menus\falloutdict.txt`, one line of 4,710 upper-case words separated by
spaces) and `menus\hacking_menu.xml` are in `Fallout - Misc.bsa`.

| What | PC | Xbox PDB name |
| --- | --- | --- |
| Who gets in (0 the password note, 1 open, 2 locked out, 3 hack, 4 too little Science) | `00966c60` | |
| Using a terminal (too little Science: `sHackIneligible`) | `00501310` (vtable +0x124), `005015f0` | |
| Opening the hacking menu | `00705ec0` → `00765b80` | `HackingMenu::Create` |
| Word length | `00501270` | |
| Difficulty (lock level; leveled terminals) | `005011a0`, `00501a30` | |
| Minimum Science by difficulty | `005017a0` | |
| Picking the words | `00768070` | `HackingMenu::Prepare` |
| Likeness | `00768a10` | `HackingMenu::CommonPositions` |
| Attempts | `00769520`, `0076ae90`, `0076af90` | `Grid::GetMaxTally`, `GridRow::GetMaxTally`, `Grid::MakeSubGrid` |
| The screen | `00768aa0` | `HackingMenu::MakePasswordFile` |
| The address column | `00768970` | `HackingMenu::MakeHexString` |
| What the pointer picks | `00769b50`, `0076a670` | |
| A choice | `00766b80` (vtable +0x0c) | `HackingMenu::DoClick` |
| A dud removed | `0076a730` | `HackingMenu::RemoveDud` |
| Shuffling (list sort, random comparator) | `0083fd60`, `007653f0`, `0076b2e0` | `WordList::ShuffleFn` |
| Random numbers | `00aa5230` | |
| `Lock`, `Unlock` on a terminal | `005cbf80`, `005cc120` | |
| `GetLocked`, `GetLockLevel` on a terminal | `0059d010`, `0059d100` | |

`HackingMenu`'s vtable is `010728f4` (found through its RTTI).

## Getting in (`world::terminal`)

A placed terminal keeps the game's `ExtraTerminalState` (extra data 0x50):
a flags byte (0x80 hacked; the low 7 bits count lockouts) and a lock level
(−1 the record's, −2 unlocked, 0..5 a script's). The record's `DNAM`:
difficulty, flags (0x01 leveled, 0x02 unlocked), server type; `PNAM` the
password note.

`00966c60`, in order: the player holds the password note → open; hacked or
unlocked → open; locked out, or the lock level is 5 → locked out (the
hacking menu opens on "TERMINAL LOCKED"); Science (current, clamped
0..100) at least `fHackingMinSkill<difficulty>` → the hacking game;
otherwise "A Science skill of N is required to hack this terminal."

- Unlocked (`00501ae0`): a script's level −2; a script's level 0 or more
  is always locked; otherwise the record's flag.
- Difficulty (`005011a0`): the lock level; a leveled terminal adds
  ⌊level × `fHackLevelMult`⌋ (0.25), at most 4. The level is the
  reference's encounter zone's when it has one (`00567e10`); nv-rs has no
  zone levels and uses the player's.
- Locked out (`00501990`): one lockout counts unless a perk's "Ignore
  Locked Terminal" (entry point 20) is other than 0 (Computer Whiz, the
  only perk in the game's data with it); two always count.
- `fHackingMinSkill…` are INI settings (`[Hacking]`); the game's INI files
  set none, so 0, 25, 50, 75, 100 (exe defaults).
- Hacked (`00766b80`): the flag, experience by difficulty (`006705b0(2, d)`:
  the reward beside the first `iXPLevelHackComputer…` the difficulty
  doesn't pass), "Computers Hacked" + 1. Locked out: the lockout count + 1.
- `Lock [level]` (`005cbf80`): the level clamped to 0..5, 0 when not given,
  and the hacked flag cleared. `Unlock` (`005cc120`): unlocked and every
  flag cleared.
- `GetLocked` (`0059d010`): 2 when locked out, 1 when not unlocked (even
  hacked), else 0; Mr. House's terminal script asks `GetLocked == 2`.
  `GetLockLevel` (`0059d100`): −1 unlocked, else the difficulty.

Of the official master's 344 terminals, 237 are very easy, 19 easy, 44
average, 21 hard, 18 very hard and 5 need a key; 185 are flagged unlocked,
11 leveled, 23 have a password note. None of its 219 placed terminals
sets a lock level (`XLOC`).

## The game (`world::hacking`)

- Word length (`00501270`): difficulty × 2 + 4, + 1 when the base
  terminal's form ID is odd, at most 12.
- Word count (`00765b80`): with `max` = min(20, `iHackingMaxWords` 20) and
  `min` = min(max, `iHackingMinWords` 5), the share of Science above the
  minimum (0.5 for very hard) takes words away:
  `min + round((max − min) × (1 − above / (100 − needed)))`, at most 20
  (`004bd510` rounds halves up). Science goes through the player's perks'
  "Hacking Science Bonus" (entry point 30) first; no official perk has it.
- Words (`00768070`): the dictionary's words of the length, in the game's
  list order (each added at the front), shuffled three times. Each try
  reshuffles and takes words in order while the new word, against each
  word taken, is not the same word and, before the last, shares a letter
  in place (a word sharing none passes on a roll of 10 under an allowance);
  the last word may share each likeness with at most (count − 2) / length
  + 1 words. Tries run for 100 ms of the clock; then the allowance rises;
  after 10 rises it resets and the last word's limit rises, until it
  passes half the count, when the words of the last try stand. Shuffled
  four more times; the password is one at random.
- Shuffling is the list's Shell sort (gaps 1, 4, 13, … while at most a
  ninth of the span) with a comparator answering −1 or 1 on a coin flip.
- Attempts (`00769520`): the least, over the words, of the largest group of
  other words sharing one likeness with it; at 2 or less that + 1;
  otherwise, recursively on the groups that size, the smallest
  "worst case", + 1. At least 4.
- The screen (`00768aa0`): 408 characters (2 columns × 17 lines × 12) of
  `` !@#$%^*()_+=-`[]{}|;':,./<>?\" ``; each word dropped at a random
  place below `0x197 − length` with no letter within 2 of it, 100 misses
  allowed per word, the whole screen redrawn up to 5 times, then the words
  that didn't fit are dropped (the game keeps its password pointer even if
  the password was one of them). Each line starts with "0x" and four
  upper-case hex digits: the low 16 bits of the screen buffer's stack
  address, + 12 a line. That address can't be reproduced.
- The pointer (`00769b50`): a word under it; else one character when a
  bracket there was used; else an opening `(`, `[`, `{` or `<` whose own
  closing bracket follows on the same line with no letter between picks
  the whole bracket run (`0076a670`); else the one character.
- A choice (`00766b80`), log lines led by ">":
  - brackets (first character not a letter, longer than one): with only
    one word left "Entry denied"; else the first time only, on a roll of 4
    under 1, the attempts refill ("Allowance", "replenished."); otherwise a
    random word other than the password turns to dots ("Dud removed.").
    The bracket's start is then used.
  - the password: "Exact match!", "Please wait", "while system",
    "is accessed."; the terminal's menu opens 3 s later.
  - anything else, a single character included: "Entry denied", one
    attempt gone; "n/length correct.", or at none left "Lockout in" and
    "progress." (that one without ">"). At one left the header turns to
    "!!! WARNING: LOCKOUT IMMINENT !!!" and flashes.
- Every random number is the game's generator (`00aa5230`: an untempered
  Mersenne twister, `n` → word mod `n`, 0 → 0), shared with the whole game
  and seeded from the clock; sequences can't match the original's, only
  the rules.

## Checks

`crates/world/tests/hacking.rs` (generated words): word length and count,
likeness, the dictionary's order, the shuffle's comparisons, attempts on
known sets, the word rules over many seeds, a dictionary too small, the
screen's garbage and spacing, the pointer's brackets, wrong guesses, the
password, duds, the refill and the lone password. `locks_keys_and_terminals`
(`crates/world/tests/scripting.rs`): access, hacking's experience, a hacked
terminal's `GetLocked`, scripts' `Lock`/`Unlock`, locked out and level 5.

Menu: six `ui` tests on a hand-made menu file laid out like the game's
(the intro and its retry time, skipping by click, the dump, the pointer
and a word's two-part highlight, wrong and right choices with the log,
boxes and the 3 s close, the warning's flashing, the lockout's scroll,
locked out on opening, leaving). Live on the official data
(`NV_HACKING_SEED=7 nv-viewer <Data> GSSchoolHouse --open-menu
hacking:GSSchoolTerminal01Ref`, `--menu-pointer`, `--menu-click`; the
seed also makes the clock count readings so the screen repeats): the
intro typing with the input lines' ">", 20 seven-letter words (the
terminal's base `00064b87` is odd), the header and guess boxes, the
hover highlight across lines with the entry typed, a wrong choice
(">SEALING", ">Entry denied", ">4/7 correct.", a box gone), the password
(">TESTING", ">Exact match!", …), +30 XP, and the terminal's own screen
3 s later.

Not compared with the running original game yet.

## The menu (`ui::menus::hacking`, `viewer/src/game_menus/hacking.rs`)

Tile ids from `hacking_menu.xml`: 0 the intro, 1 the header, 2 the screen
(its `user0`–`user5` the highlight), 3 the log, 4 the cursor
(`_blink_interval` 400), 5 the entry line, 6 and 7 the lockout's lines;
the templates `hacking_intro_template`, `hacking_password_file_template`,
`hacking_password_log_template`, `hacking_guess_template`. Read from the
code (the menu's fields from the Xbox PDB's `HackingMenu`: `IOLines`,
`xEntryText`, `eStage`, `bSuppressEntry`, `dwAccessTime`, `dwRetryTime`):

- Typed lines (`0076b300`; a step `006ffea0`; new text `006ffda0`; at once
  `00700110`): every `1000 / rate` ms (rounded half up, at least 1; 1000
  below a rate of 2) the characters the time since the last step covers;
  the line shows as it starts; done one step after its last character.
  Rates (exe defaults, none in the data): `iHackingOutputRate` 67,
  `iHackingInputRate` 20, `iHackingDumpRate` 500 characters a second;
  flashing `iHackingFlashOnDuration` 750 / `OffDuration` 500 ms.
- Opening (`00765b80`): `iYStart` 0 and `iYInterval` 30 (the constructor,
  `007654f0`); the intro's 14 lines at rows 0, 2, 4, 6, 8, 9, 11–17, 19:
  `sHackingSecurityReset`, `sHackingIntro01`–`13`; the 3rd, 5th, 6th and
  last are the player's (a ">" before them, the input rate). The menu's
  `user0` is ">". `UIHackingFanHumLP` plays.
- The intro (`00769830`): the first unfinished line types; an input line
  shows its ">" and waits 1 s, plays `UIHackingCharSingle` for each
  character and `UIHackingCharEnter` when done, then waits 0.5 s; machine
  lines play `UIHackingCharScroll`. "SECURITY RESET..." is hidden each
  frame unless the menu last closed less than `iHackingRetryMilliseconds`
  (10 s, the destructor `007658e0`) ago, so outside that it shows only
  while it types. A click makes the screen at once, outside the retry
  time only.
- The screen (`00768aa0`): the header's lines under id 1 at y 0, 30, 90;
  id 1's `user1` = the attempts line's x + 17 × its length, `user2` its y;
  a guess box per attempt, shown (id 1's `user0`) once the dump reaches
  them (an empty line, `007694f0`); the 34 lines under id 2 (`listindex`
  the row, `user0` the column) at the dump rate, one after another; id 0
  hidden. A click finishes the dump.
- The pointer (`00769b50`, `00769700`): the character under it is
  (x − the line's x) / 17; a new character or line picks again (the
  selection's text typed on the entry line, a `UIHackingCharSingle` per
  character, earlier ones stopped); the highlight at the line's x + 17 ×
  the character's index, a word's second part on the next line (the next
  column's first line from the bottom of the first; +119, the 7-character
  address, found in the disassembly where the decompiler dropped it).
  After a choice nothing is picked until the pointer leaves the letters
  (`bSuppressEntry`), and the pointer off the screen clears it (`00767ff0`).
- A choice (`00766b80`): `UIHackingCharEnter`; the log lines (`00767ef0`:
  older lines move up by the count, the newest at the bottom); a wrong
  guess takes a guess box and plays `UIHackingPassBad` 250 ms later, the
  password `UIHackingPassGood`; a refill adds boxes back; the attempts
  line rewritten.
- One attempt left: id 1's second line becomes `sHackingWarning`, typed at
  once, and flashes; a refill puts `sHackingHeader2` back.
- Locked out (`0076a3f0`): the menu's `user1` rises by 30 every 50 ms
  (`UIHackingCharScroll`) until the depth rect is above the top; then its
  `user2` shows ids 6 and 7.
- The password: 3 s later (`dwAccessTime`, `0076a510`) the menu closes and
  the terminal's screen opens (`TransitionToComputersMenu`, `0076a540`).
- Leaving: special code 10 (`00767b80`; how the PC's keys reach it isn't
  traced: the viewer sends it for Tab and Escape).

In the viewer, using a terminal decides as the game does (`00501310`):
straight to the terminal's screen, the hacking menu, or "A Science skill of
N is required to hack this terminal." with `UIPopUpMessageGeneral`.
Hacking calls `world::terminal::hacked` (experience, the statistic) or
`lock_out`. The address column's base is a random multiple of 4.

## On the terminal's screen

With `[RenderedTerminal] bUseRenderedTerminals` on (the PC default), the
hacking menu is drawn on the terminal model's screen as the terminal's
own menu is ([TERMINALS.md](TERMINALS.md), "The rendered terminal"): it
opens one (`00705ec0`), and its hand-over to the terminal's menu after the
password keeps it (`0076a540`). Its file (`hacking_menu.xml`) keeps its
words within the same top-left 960 × 720 of the 1280 × 960 picture
(its depth rect 920 × 630 at x 32; only its background covers the whole
picture, of which the screen shows that corner). The
pointer goes through the screen, so `--menu-pointer` positions are where
a word shows on the model's screen in the window, not the flat menu's.
Leaving (code 10, `00766aa0`) plays `OBJComputerTerminalPowerDown`
(`007ffe40`) and fades the terminal out.

## Not done

- The controller's cursor (special codes 1–4 and 9, `00767610`).
- (Done since: hacking an owned terminal raises the trespass alarm,
  `008c0ec0`; see [FACTIONS_CRIME.md](FACTIONS_CRIME.md).)
- Encounter-zone levels for leveled terminals.
- (Done since: the tutorial message the menu asks for on opening,
  `00718630(0x13, …)`; see [TUTORIALS.md](TUTORIALS.md).)
