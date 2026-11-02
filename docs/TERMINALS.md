# Terminals: the terminal's own screen

What the player sees once into a terminal (after hacking it, with its
password note, or unlocked; [HACKING.md](HACKING.md) has getting in):
the game's `ComputersMenu`. Until 2026-10-06 the viewer showed terminals
in a text panel of its own (still the fallback without the game's menu
files).

## Sources

`FalloutNV.exe` 1.4.0.525, read in Ghidra 12.0.4 from the full private
export plus whole-function disassembly; `menus\computers_menu.xml` from
`Fallout - Misc.bsa`.

| What | PC |
| --- | --- |
| `ComputersMenu` vtable (RTTI) | `01072004` |
| Constructor (sounds, note handlers) | `007577f0` |
| Opening | `00757b70` (from `00706130`, menu request 3; hacking is request 2, `00709470`) |
| The logon | `00758ad0` |
| The screen | `00758d30` |
| Filling the list | `007586e0` |
| The typing queue: add, step, finish all | `00759da0`, `007598d0`, `00759f50` |
| Every frame | `00758470` |
| Clicks | `00757f70` |
| Leaving (code 10) | `007583f0` |
| Back a screen | `00758a80` |
| A text note's page | `007590c0` |
| A picture note | `00759470` |
| A sound note | `00759030` |
| An item typed again after a note | `00759560` |
| Running an item's result script | `00501830` |
| Giving a note | `00966a70` |
| `ForceTerminalBack` | `005dc4e0` (command table entry `01195e88`) |

## The record

`TERM`: `DESC` the welcome text, `DNAM` difficulty, flags, server type
(byte 2, the item `+0xA6`: `sTerminalServerText1`–`10`), then items
(`ITXT` text, `RNAM` result, `ANAM` flags, `INAM` note, `TNAM` sub-menu,
the result script, `CTDA` conditions). An item's flags (its `+0x74`): 0x01
gives its note (`00758350`), 0x02 fills the list again (`00758390`); the
code adds 0x04 when it's picked (only a default for the controller's
cursor, `xdefault` 2). In the official master: 895 items, 535 with a text
note, 4 a voice note, 3 a picture; 137 flagged 0x02, 18 0x01, 1 both; 86
sub-menus, 208 result scripts.

## The menu (`ui::menus::computers`, `viewer/src/game_menus/computers.rs`)

Ids 0 the server line, 1 the list (`computers_file_template`), 2 the
display zone, 3 and 4 the header, 5 the result, 6 the cursor, 7 the
welcome, 8 the prompt, 9–12 the logon's lines, 13 the background, 14 the
zone's text, 15 the separator.

- Every line appears through one queue, typed in order, each at the rate
  set when it was queued: menus `1000 / iComputersDisplayRateMenus`
  (150 a second → 6 ms, whole milliseconds), notes `…Notes` (150), the
  player's input `1000 / iHackingInputRate` (50 ms); exe defaults, none
  in the data. Lines typing play `UIHackingCharScroll`.
- Opening (`00757b70`): `UIHackingFanHumLP`. A terminal that isn't unlocked
  (`00501ae0`: hacked, or opened with its password note) logs on first
  (`00758ad0`): "WELCOME TO ROBCO INDUSTRIES (TM) TERMLINK", "> LOGON
  ADMIN", "ENTER PASSWORD NOW", "> " and one `*` a letter of the hacking
  game's words for it (`00501270`); the "> " lines at the input rate with
  `UIHackingCharSingle` a character and `UIHackingCharEnter` at the end;
  0.5 s after each line; 2 s after the last the screen comes (a click on
  the background brings it at once).
- The screen (`00758d30`): the logon's lines hidden; first in the queue
  `sComputersHeader1`, `sComputersHeader2` and the server line, each
  centred (x = (width − 17 × length) / 2); then the welcome (the
  separator shows when it's typed), each item whose conditions pass on
  the placed terminal as "> text" (`_enabled` 0 until the queue is empty),
  "> `sComputersBack`" below a sub-menu, and the prompt "> ".
- An item (`00757f70`): `UIHackingCharEnter`; its result script runs on
  the placed terminal; its result typed beside the prompt and hidden after
  `iComputersResultDisplayTimeout` (5) s; a sub-menu is gone into; a note
  given with flag 0x01 when the player hasn't it ("Note Added: name",
  `sComputersAddedNote`) and shown: a text note in the zone a page at a
  time (`007590c0`: the zone text's `wraplines` less the welcome's lines,
  + 1, broken as `00a12fb0` breaks lines), a picture (`XNAM`) in the zone,
  a sound note's sound played, a voice note nothing (no handler); with no
  note, flag 0x02 fills the list again.
- The zone clicked: the next page, else back to the list, each item typed
  again with its conditions asked again (`00759560`), or filled again with
  flag 0x02.
- Code 10 (`007583f0`): out of a note, else back a screen ("Back" too,
  `00758a80`); from the first screen the menu closes. The viewer sends it
  for Tab and Escape.

`ForceTerminalBack` (`005dc4e0`, 30 calls in the official scripts, all
in terminal items): an open terminal menu goes back a screen, from inside
the item's script, before the item's result and note.

After the password, the hacking menu hands over to this one
(`TransitionToComputersMenu`), so a hacked terminal logs on.

## Checks

Four `ui` tests on a hand-made menu file laid out like the game's (the
screen typing out and the items enabled; the logon with seven asterisks,
the 2 s and the skip; an item asking the world, a long text note in
pages, leaving the note and the result's timeout; a sub-menu and Back).
Live on the official data: `GSSchoolTerminal01Ref` after hacking it (the
logon, then "ROBCO INDUSTRIES UNIFIED OPERATING SYSTEM", "-Server 6-",
the SoftLock welcome, "> Disengage Lock"), and opened directly
(`--open-menu terminal:REF`) with a click on its item: "Clearance granted,
Unlocking..." beside the prompt and the list typed again (flag 0x02).

Rendered (the default): `cellview::rendered_terminal`'s tests (the
model's turn, the 4:3 zoom, the INI over the exe's defaults with the
misspelt fade-in, the camera's share, the pointer meeting the screen and
the power button, the pointer in menu units, the lights with the model,
the fade in and out, the platform texture folder) and the `ui` tests'
`Leave`. Live at 1920 × 1080 (`--screenshot`): `GSSchoolTerminal01Ref`
logging on and `V21MartinaTerminalREF` (Vault 21, cell `0010FDEB`) on its
screen, on the model in front of the room with the world beside it; the
hacking game there (`NV_HACKING_SEED=7`, `--menu-pointer 745,508`: the
word SPOTTED under the pointer highlighted, ">SPOTTED" on the entry
line), and the password (TESTING, `--menu-pointer 1100,718
--menu-click 5,8,9`) handing over to the terminal's logon with the model
staying up; the power button (`--menu-pointer 1265,985 --menu-click 4`): the
terminal gone at once; leaving (`--menu-keys 4:tab`): the model a second
into its 2 s fade out.

Not compared with the running original game.

## The rendered terminal (the PC default)

`[RenderedTerminal] bUseRenderedTerminals` is 1 in the exe's settings
table (`011db620`) and in the shipped `Fallout_default.ini`, so on PC the
terminal's menu and the hacking menu aren't drawn flat over the window:
they're drawn on the screen of a terminal model in front of the player,
with the world still drawn round it. Opening either menu with the
setting on makes a `FORenderedTerminal` (`00706130`, `00705ec0`; ctor
`00705fe0`, vtable `0106ebe4`; a `FORenderedMenu`, `0107841c`, as the
Pip-Boy's screen is); the hacking menu's hand-over to the terminal's
keeps the same one (`0076a540`). Read from the PC code and the Xbox 360
prototype's `FORenderedTerminal` / `FORenderedMenu` / `Main::DrawWorld*`
(Xbox PDB names; same member offsets).

| What | PC |
| --- | --- |
| Initialize | `007feeb0` (base `007fc150`) |
| Draw (the menu into its picture) | `007ff7c0` → `007fba00` |
| Every frame | `007ff820` (base `007faf10`) |
| The pointer through the screen | `007fb790`; the power button `007ffba0` |
| Opening, closing | `007ff650`, `007ff740` |
| Fade out on leaving | `007ffaf0` (from `00757ea0`, `00766aa0`) |
| The power-down sound | `007ffe40` |
| The screen effect | `007fbee0` (`ISIFSCANBLEND`, as the Pip-Boy) |

- The model is `meshes\terminals\TerminalInterface01.NIF` (read once and
  kept): its node "screen" has the screen, "PowerButton" the button.
- Placed (`007feeb0`): its top node turned Z(1.57) · Y(1.57) (`010559cc`;
  model +y away from the camera, +z up, +x right) and moved to (zoom,
  `fRenderedTerminalVPos`, `fRenderedTerminalHPos`), zoom
  `fRenderedTerminalZoom` × 0.75 when the screen is 4:3 (`0101de30`;
  `011c70eb`, "height / width isn't 0.75", from `004dc360`). INI: zoom
  36 (exe 20), VPos 0.38, HPos 0.
- Its camera: at the origin, no turn (looking along +x, +y up), the
  lockpicking menu's frustum: ±0.75 tan(a) across and that × height /
  width up and down, a = `fDefaultFOV` (75) × `fRenderedTerminalFOV`
  (0.15) degrees. The frame (7.3 × 6.1 units, its middle 0.4 below the
  node, which VPos makes up) fills the height; on a 16:9 screen its sides
  are at ±0.67 of the half-width and the world shows beyond them.
- Lit by the model's own three point lights (Omni02, 03, 05: a green one
  behind the screen and two pale yellow ones, dimmer 0.8), given to the
  menus' scene with three times the model's bounding radius (4.58) as
  their reach (`00b5ca70`). The code's own light ("Omni01", the
  `fScreenLight*` settings, `007ff960`) is only for a model without
  lights, so those settings don't matter for this one.
- The menu is drawn into a picture of 1280 × 960 menu units (`007fba00`:
  `0106ec38`, `0106f2dc`, cleared to transparent black). The terminal
  menu's file keeps to its top-left 960 × 720 (`computers_menu.xml`'s
  depth rect: the screen's height × 0.75, "ref: FORenderedTerminal::
  Initialize()"), and the screen's texture coordinates cover just that
  (u 0 to 0.767, v 0 to 0.755).
- The screen effect, as the Pip-Boy's: blur `fDefaultBlurRadius` 0.3 and
  `fDefaultBlurIntensity` 0.2 (`007fc150`; the INI sets neither),
  scanlines `sScanlineTexture` (`PipboyScanlines.dds`) ×
  `fRenderedTerminalScanlineScale` 130 (`007f9050`), the tint the
  terminal's system colour (3: 33, 231, 121), the pulse and the passing
  band. No burst, roll or shudder: only the Pip-Boy's menus start those.
  The pulse doesn't touch a light (`+0x94` off).
- The pointer (`007fb790`): the ray from the camera through it meets the
  screen; its texture coordinates × 960 × 1.333333 across and × 960 down
  are the menu's point. A click on the power button (`007ffba0`) leaves
  at once (`007ffd50`: the power down, the menu gone, the rendered menu
  ended).
- Fading: opening sets the model's `BSFadeNode` range to 100000 and its
  fade to 0 (`007ff650`); leaving sets the range to 0 (`007ffaf0`). The
  fade node steps by the frame's time over `[LOD] fFadeInTime` (exe 1.2;
  the shipped INI's `fFadeInTimet=2.0` is misspelt) or `fFadeOutTime`
  (INI 2.0), at most `maxFadeIncrement` 0.1 a frame (`011ad7e4`,
  `011ad7e8`, `011ad7f8`; Xbox `BSFadeNode::OnVisible`); once the fade
  out stops changing, the rendered menu ends (`007ff820`).
- Leaving (`00757ea0`, `00766aa0`): `OBJComputerTerminalPowerDown`
  (`007ffe40`), the menu's own fade (`menufade` 0.75), the fade out. The
  hacking menu handing over to the terminal's plays nothing.
- Drawn (Xbox `Main::DrawWorldRenderedMenu`): the world, then the model
  with the menu camera (`DrawWorld_DrawRenderedMenuParent`), then the
  image space pass, then the screen again with the menu's picture
  (`DrawWorld_RenderedMenu`).

In nv-rs: `cellview::rendered_terminal` (the settings, the model's place,
the camera, the lights, the pointer's pick, the fade) and
`viewer/src/rendered_terminal.rs`: the menus' pictures into a 1280 × 960
picture, the Pip-Boy's screen effect into the screen's texture, the model
by a camera of its own into a picture the window's size, laid on the
HUD's picture under the menus' pictures and the cursor with the fade as
its alpha. With the setting 0, or without the model, the menus are drawn
flat as before. Differences: the model isn't graded or bloomed (drawn
after the image space pass), the fade is over the finished model, not
each piece's alpha. The menus on the screen fade in and out with every
menu's fade ([MENU_FADES.md](MENU_FADES.md)): 0.75 s when the player
leaves (`menufade` set by `00757ea0`, `00766aa0`), 0.25 s on the hacking
menu's hand-over.

## Not done

- The controller's cursor and keys (codes 1–4, 9).
- The tutorial message (`00718630(0x18, …)`).
- The cursor's exact place after list items (`_text_width`, `_text_y`).
