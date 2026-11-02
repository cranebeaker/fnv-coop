# Tutorial messages

New Vegas shows a help box (the "VDSG Manual" box, `TutorialMenu`, class
1059, `menus\tutorial_menu.xml`) the first time certain menus come up, once
per saved game. Read from FalloutNV.exe 1.4.0.525, with the Xbox 360
prototype's PDB names (`InterfaceManager::TutorialManager`,
`Interface::TutorialMessageID`, `TutorialMenu`).

## The manager

The interface manager keeps it at `+0x4d4`: a 32-bit word for each of the
41 ids (`TUT_PIPBOY_STATUS` 0 … `TUT_HARDCORE_NEEDS` 40), then the id about
to be shown (`+0xa4`, 41 for none) and when it's due (`+0xa8`, a
`GetTickCount`). A word's bits: 0 asked for, 1 shown, 2–7 the menu it waits
for (its class − 1001; 0 any menu), 8–31 a delay in milliseconds.

- An id's message is the form `0x168 + id` of FalloutNV.esm (`HelpHacking`
  0x17B, `HelpCaravanBetting` 0x186…).
- `ShowMessage(id, menu, delay)` (`00718630`): stores the delay; a menu
  class of 1001–1084 or 0 is stored and the message is asked for if its
  record exists, has "Auto Display" (`BGSMessage` +0x38 bit 1, the `DNAM`
  flags' 2) and hasn't been shown. The answer tells the menu whether to
  wait. Vanilla's barter, container, terminal, dialogue, levelling, race,
  tag skills and Pip-Boy stats / data / items / repair messages have only
  the "Message Box" flag, so they never come up.
- `IsShown` (`00718840`), `MarkShown` (`007185e0`: shown, no longer asked
  for).
- The update (`007182e0`, from the interface manager's update `0070c4a0`,
  every frame): nothing while the start menu is up (`011daac0` flag 1) or
  the name entry (`011d8950`); outside the game (`011d8a80` +0xc not 2) the
  choice is dropped. Once the chosen id is due, its menu (if any) is the
  top menu and the top menu has faded in (`007024e0`: `Menu` +0x24 is 1),
  the tutorial menu opens with it and it's marked shown (on failure the
  line "Error occurred while trying to display tutorial message"). Until
  then each update looks for one asked for, not shown and not the chosen
  one; choosing it drops the previous choice's request (when that one's
  menu is up) and sets it due after its delay (a delay of 0 looks again at
  once). The menu test the game makes reads the *chosen* word's menu bits
  to decide whether to look at the candidate's menu; with none chosen it
  reads the manager's next field (41) as word 41, whose menu bits are 10,
  so a candidate always needs its own menu on top, and one for any menu
  (0) needs the message box (1001) on top. That is how the reputation
  message (asked for with menu 0, 500 ms) comes up: over the message box
  that announces the reputation change (`006155f0`).
- Saved (`007187a0`, in the interface's part of the save, `007066d0`): bit
  1 of each id, eight to a byte, six bytes; loading (`00718890`) sets only
  those bits.
- `bHelpEnabled:Interface` (`011db094`) is in the exe but nothing reads it
  on PC.

## Who asks

| Menu | Id | How |
| --- | --- | --- |
| `HackingMenu::Create` `00765b80` | 0x13 hacking | `ShowMessage(0x13, 1055, 512)`; the update `00767c90` waits (`+0x1da`) |
| lockpicking update `0078eb50` | 0x14 (0x1C with a pad) | in its "ready" state, `ShowMessage(id, 1014, 0)`; waits (`+0x99`) |
| `ComputersMenu` `00757b70` | 0x18 terminal | `ShowMessage(0x18, 1057, 512)`; waits (`+0xc0`, `00758470`) |
| Caravan `00741060`, `00741500` | 0x1E–0x21 | opens the tutorial menu itself (by editor ID) if not shown, marks it, waits (`+0xe78`, id `+0xe7c`) until back on top |
| crafting `00726ff0` | 0x26 | opens `HelpCrafting` itself if not shown, marks it |
| reputation change `006155f0` | 0x27 | `ShowMessage(0x27, 0, 500)` |
| `VATSMenu::Create` `007e9200` | 0x15 (0x1B with a pad) | `ShowMessage(id, 1056, 512)` |
| `InventoryMenu::Create` `0077fc10` | 0x22 weapons | `ShowMessage(0x22, 1002, 512)` as ITEMS is made |
| ITEMS' tab buttons `00780140` | 0x23 apparel, 0x24 ammo | Apparel (0x19) `ShowMessage(0x23, 1002, 512)`, Ammo (0x1c) `ShowMessage(0x24, 1002, 500)`, each press (the tab shown or not; the arrow keys press the next tab's button, `00782190`) |
| barter, container, dialogue, level up, repair, race, stats, map, chargen | | `ShowMessage` (none of vanilla's is "Auto Display") |

## The tutorial menu

`TutorialMenu::Create(message, manual)` (`007e8890`; every caller above
passes `manual` false): an open tutorial menu is closed first; ids 0–7 are
the text, Next, Previous, Close, title, page, scrollbar and `TM_VDSG_text`;
`sVDSGManual` and `sCloseButton` go on their tiles, Next, Previous and the
page are hidden; `SetTitleAndText` (`007e9060`) puts the message's `FULL`
and `DESC` (HTML when it starts with `<`) and the scrollbar back to the top;
`UpdatePrevNext` (`007e9010`) makes Next and Previous clickable by page.
Close (id 3, E) and the cancel code (10, `007e8e80`) close it.

With `manual` true (the start menu's Help) it is the help manual: the form
list `HelpManual` (FLST 0x163, 37 messages; `HelpManualXBox` 0x165 with a
pad), and no message means its first. Next and Previous keep the file's
visibility and get `sNext` / `sPrevious`; the message's place in the list
is the page, shown "n/N" on the page tile (a message not in the list, and
every single message, has no pages: the page tile hidden). Next and
Previous (`007e8e20` / `007e8e40` → `007e8ed0`) turn to the page after or
before (nothing past either end), with its message and `UpdatePrevNext`.
No message at all: "Warning:  Unable to find valid starting message for
Tutorial Menu." and the menu closes again.

The start menu's Help (`007d0770`) shows the version (the start menu's
tile 10), then goes up the menu stack under the start menu (`+0x114`,
ten entries) for the first menu with a tutorial word waiting for it
(`007d09c0`: the first id whose menu bits + 1001 are its class, so a
word with no menu counts as the message box's) whose message is in the
manual; the Pip-Boy (class 1) counts as its page shown (stats 1003,
repair 1035, inventory 1002, map 1023). It opens the manual on that
message, or on its first page.

A script's `ShowTutorialMenu <message>` (`005da630`; vanilla's
`CGTutorial` quest shows `HelpHealingLimbs` this way) opens the tutorial
menu with that message at once, not through the manager, and doesn't mark
anything shown.

## Here

`world::tutorial` (`Tutorials` in `GameState::tutorials`, saved as a
`tutorials` line; `ask` reads the record's flag), `ui::menus::tutorial`
(`TutorialMenu`), the viewer's `game_menus::tutorial` (the update each
frame over the open menus, `show` / `show_once`; `show_form` for
`ShowTutorialMenu`, `Event::TutorialMenu`; `open_manual` for the start
menu's Help, which the viewer's start menu can only open over the game,
so on the manual's first page). The manager waits while the start menu
is up as the pause menu. Hooked: Caravan's four,
crafting, hacking, the terminal menu and lockpicking (`world::lockpick`'s
`Effect::Tutorial` and `tutorial_wait`; the viewer now draws the
lockpicking menu's scene and pictures before the HUD's camera, which lays
the game's menus over them); V.A.T.S. (`viewer/src/vats.rs`: the ask as
it opens, `Vats::in_menu` for the manager, its keys held back while a
game menu is over it); the Pip-Boy's ITEMS (`viewer/src/pipboy.rs`: the
weapons ask when ITEMS comes up; `ui::pipboy::Action::Tutorial` from the
Apparel and Ammo buttons; `Pipboy::top_class`, the repair and mod
screens 1035 / 1061 over ITEMS); the reputation title
(`world::reputation::change`, as the level changes). The manager's top
menu is the game's menu on top, else the lockpicking menu, V.A.T.S.'s
or the Pip-Boy's (`game_menus::run_tutorials`). Seen live: Caravan's betting and
deck-building messages against Ringo, closed with E; the hacking message
over a waiting hacking screen (GSSchoolTerminal01Ref); the lockpicking
message over the lock (DinoBiteStorageDoorREF, NovacGiftShop), closed
with E, the lock then picked as usual.

The lockpicking message names the controls (`&-sUActnForward;` …): the
game's text pass (`00a12fb0`) first asks `007070c0`, which reads the 28
actions' settings (`sUActn…`, `011d51d0`) as the name of the key or mouse
button bound to them (`007039b0`: mouse button below 9 by `sMouse…`,
else key by `sKB…`, the DirectInput key number's setting, `011d52f0`).
`ui::controls` has the tables and the exe's default bindings
(`00a24b70`); `ui::game::new_ui` puts the names in the text settings, so
the box reads "WSAD: Apply torque…" and "F: Force Lock" as in the game.
A player's own bindings aren't read.

A `DESC` starting with `<` sets the text tile's `isHTML`, laid out by
the game's HTML layout (`00a17390`; [HTML_TEXT.md](HTML_TEXT.md)):
`HelpHealingLimbs`, the one help message written that way
(`ShowTutorialMenu`'s, page 9 of the manual), shows its paragraphs
without its `<div>` and `<p>` tags.

Not done: the reputation title's own box (`006155f0`'s message box with
the title's picture and sound; the title is a corner message here, so the
reputation help waits for the next message box); the pad's messages
(0x1B, 0x1C, `HelpManualXBox`); "outside the game" (the viewer has no
main menu) and the menus' fade-in (shown at once here). "Held" is the
start menu up as the pause menu (`viewer/src/game_menus/tutorial.rs`,
`held`).
