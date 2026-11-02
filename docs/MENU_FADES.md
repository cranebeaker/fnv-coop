# Menus fading in and out

Every one of the game's own menus fades in when it opens and out when it
closes, run by the interface manager, not by the menus. nv-rs: `ui::fade`
(the rules), `viewer/src/game_menus/mod.rs` (`Screen::show_new`, `close`,
`end_fades`; the drawing in `draw_menus`).

## What the game does (FalloutNV.exe 1.4.0.525)

Names from the Xbox 360 prototype (Xbox PDB); its decompile of the named
functions is in the private `~/nv-re/decomp/x360_menufade.c`.

- **State.** `Menu::xFadeState` (`+0x24`, the same on PC),
  `Interface::FADE_STATE`: shown 1, fading out 2, hidden 4 (a new menu's,
  `Menu::Menu`), fading in 8.
- **Time.** The menu tile's `menufade` (the menu tile's constructor
  `00a1ef30` sets 0.25; of the vanilla files only `loading_menu.xml` gives
  another, 0.75), or `explorefade` when `menufade` is 0; 0.25 for a menu
  without a tile (`0101622c`).
- **Opening** (`Menu::PrepForVisibility`, `00a1dc20`): the menu tile
  hidden, then `StartFadeIn` (`00a1db20`) unless the caller asks for it
  at once. Of the 34 callers only the HUD, the Pip-Boy's inventory, DATA,
  stats, item-mod and repair pages open at once; every other menu fades
  in.
- **Closing** (`Menu::StartFadeOut`, `00a1d910`, called by 37 menus'
  close code): nothing unless the menu tile is shown; else the fade out
  starts, and a menu that marked itself to leave the menu stack (trait
  6002) leaves it now (`00706fd0`), so the keys go to the menu below.
  The message box, "how many?" and two others close with
  `InstantFadeOut` (`00a1d9e0`) instead while a rendered menu (the
  Pip-Boy, a rendered terminal: `00707af0`) is up: hidden at once, gone at
  the frame's end. The terminal and the hacking menu set `menufade` to
  0.75 before closing when the player leaves (`00757ea0`, `00766aa0`); the
  hacking menu handing over to the terminal's (`0076a540`) fades over the
  usual 0.25. The rendered terminal's power button deletes the menu
  outright (`007ffd50`).
- **The fade list** (interface manager `+0x164`): menu, seconds gone,
  seconds it takes; adding one replaces the menu's earlier one
  (`007164c0`). Every frame before the menus run (`0070b8f0` →
  `00716320`), and again whenever a fade starts (`00706f70`), every fade
  moves on by the frame's seconds (`011f6394`+0xc; divided by the time
  multiplier `011ac3a0` in V.A.T.S. playback); finished ones leave. How
  far a fade is (`00716660`): 1 with none, -1 when it takes no time, else
  gone / total held to 0..1 (`0040ebd0`, `00404010`).
- **After the menus run** (`00711ea0`, Xbox
  `InterfaceManager::PostIdleStuff`), for each menu: fading in, part-way:
  the tile shown, faded to the amount; whole: shown at 1, state shown.
  Fading out, part-way: faded to 1 − the amount; whole: state hidden, and
  the menu deleted if it marked itself to leave the stack (the loop stops
  there for the frame), else its tile hidden. While any fade is part-way
  (`+0x11`) the game stays in menu mode when the last menu has closed.
- **The fade itself** (`00712450`, `InterfaceManager::RecursiveFade`):
  down the menu's scene graph, each piece's material alpha becomes its
  tile's `alpha` / 255 × the fade, held to 0..alpha (0 when the fade is
  under 0.0001); a tile with `disablefade` is instead shown only at a
  whole fade and nothing under it is touched; the children of a tile with
  id 9000 are left alone. (No vanilla menu file uses either.)
- **Input** (`0070c4a0`): the pointer's moving onto and off tiles,
  presses, clicks and drags reach a menu only while it's shown (the tile
  under the pointer is let go of, without a "mouse off", while its menu
  fades); the keys go to the deepest menu not fading out or hidden
  (`00720e60`), so a menu fading in takes keys.

## In nv-rs

All the game menus the viewer shows now fade in over 0.25 s and out over
0.25 s (the terminal and hacking menus 0.75 s when the player leaves,
drawn on the rendered terminal's screen while they fade). Closed menus are
drawn, not run, until their fade ends; the player stays held (menu mode)
until then. The message box and "how many?" close at once over the
Pip-Boy or a rendered terminal; the power button still removes the
terminal at once. Changed behaviour: every game menu (message box,
dialogue, container, barter, recipe, repair services, companion wheel,
Caravan, how many, level-up, traits, character generation, text entry,
sleep/wait, Vigor tester, start/pause, hacking, terminal) where before
they appeared and vanished at once. The start menu's own page fades
(`START_MENU.md`) run inside this one. Log lines `Menu NAME fading in/out
(S s).` and `Menu NAME faded out.` show them.

Not done: a menu closed before it was ever shown (whose
`StartFadeOut` does nothing in the game, leaving it to fade in again) is
removed at once here; the menus' own pictures aren't faded by the
material alpha but by the drawn alpha (the same for the flat pictures the
menus use).

## The world behind the menus (B31)

The image space manager's effect 15 is the menus' background: the world
drawn once, with an image space modifier, and held still while the menus
are up (`world::menu_background`, `viewer/src/menu_background.rs`; the
blur in `viewer/src/grade.rs` / `grade.wgsl`). Traced in FalloutNV.exe
1.4.0.525:

- Each frame the render path (`0086e650`) sets "menu mode" (`011dea2b`: a
  menu-mode menu is up, `00702360`, or the Pip-Boy is coming up, its state
  2, `00709bc0`) and calls `0086f450`. It captures (`00871dc0`) when in
  menu mode, `bStaticMenuBackground:Display` is on (default 1,
  `00f40570`; read once into `011dea28`), nothing is captured yet
  (`011dea29`), and not: the dialogue menu (1009) on top, the main menu (the
  start menu without its pause flag, `0070edf0`), V.A.T.S. (1056) or a
  message box (1001) shown (`00702680` with mask 0xb: fade state shown,
  fading out or fading in), the Pip-Boy up or coming up (`00705a00`), or
  `00703d50` (an unidentified object at `011d8ce8`).
- `00871dc0` draws the world with the modifier `00718ab0` picks applied at
  strength 1 (`005299a0(imad, 1.0, 0)`), takes it away again (`00529c90`)
  and sets `011dea29`. `00718ab0`: `PauseBackgroundFX` (`0004EEE8`) for
  the pause menu (`004a4040`), else `PipBackgroundFX` (`00096389`) for the
  Pip-Boy (`00967ae0`), else `InterfaceBackgroundFX` (`00044F34`) when the
  lock (1014) is on top, else `PopupBackgroundFX` (`00032B38`) unless a
  tile is named "Player Name Entry Menu" (then none).
- Once captured it stays while menu mode lasts (unless the main menu,
  `00703d50`, or the dialogue or V.A.T.S. menu on top), while fader 1 runs
  (`007014a0(1)`) or while menu 1054, 1014, 1060, 1074 (the Vigor
  Tester), 1080, 1081, 1082 or 1083 is shown; otherwise `00877430` lets it
  go (effect 15 off: `007123f0`/`007123a0`; `011dea29` cleared).
- The modifiers (all flags 0, so their first keys hold): popup: blur 3;
  pause: blur 3, saturation × 0, tracks 18 × 0.8 and 19 × 1.3, tint
  (0.67, 0.66, 0.24) at 0.59, depth of field 1; interface: blur 2, bright
  clamp × 2, saturation × 0.1, brightness × 0.5, tint (0.33, 0.58, 0.44)
  at 0.78; Pip-Boy: depth of field 0.7 only.
- The blur: the manager keeps the largest blur of the modifiers playing
  (`00b8ccb0`, `+0x25c`), hands it to the blur effect (`00b8d020` →
  `012003d0`; vtable `010b8000`), which draws its pass for radius
  `ceil(blur)` (`00ba4d20`, `00ec9e10`), 1 to 7 (none above 7). That pass
  (`00ba4270`) is `ISBLUR(2r+1)` down then across, taps −r … r texels,
  weights from the exe's 7 × 15 table (`011ade38` + r × 0xf0) mixed
  between rows max(r − 1, 1) and r by `1 − (r − blur)`. Every row is a
  Gaussian with σ = r / 2 normalized over its taps (the same as the bloom
  blur's, checked in `world::menu_background`'s tests).

In the viewer the menus up are read into those tests each frame; on a
capture the modifier's values join the final pass's (tint, saturation,
brightness, …) and the picture before the final pass is blurred once into
a held texture, which the final pass then reads until the background is
let go. The Vigor Tester's machine, the HUD and the menus are drawn over
it (they come in at the final pass). A blur from any other modifier (a
script's or a hit's) is drawn by the same blur, live.

Not done: a blur under 1 (the effect's passes 9 and 10, a blend; none of
the four has one), depth of field (`PipBackgroundFX`'s only effect, the
pause one's too), the Pip-Boy's own capture (other callers of `00871dc0`:
`007cbaf0`, `007ce7a0`, behind `007079b0`; not traced, so the Pip-Boy shows
the world as it is, which its modifier only changes by depth of field),
`00967ae0`'s other case (a player state at `+0x690`), fader 1, and where
the blur falls among the game's other passes (here before the bloom is
added and the grade applied; the bloom is still taken from the live world
while the picture is held). Whether the copy pass before the blur (pass 0)
works at full size isn't traced; it is taken as full size.
## Checks

The dialogue menu fading out under a service menu and back in
(`00763ff0` / `007640a0`, [DIALOGUE.md](DIALOGUE.md)) is the same
`StartFadeOut` / `StartFadeIn` on the same list: a menu kept open, not
marked to leave the stack, so hidden when faded out
(`DialogMenu::service_opened` / `service_closed` take the screen's
`Fades`).

`ui::fade` tests (the times, fading in then shown, fading out and gone,
kept menus hidden, a hidden menu not fading out, instant fade out, every
fade moving on when one starts, a fade taking no time, the loop stopping
after a menu goes, the alpha rule with `disablefade` and id 9000),
`ui::menu`'s `letting_go_of_a_menu`, the hacking and terminal tests'
`menufade` 0.75 on leaving, and the viewer's `game_menus::tests` (fade in,
pointer only once shown, fade out drawn and holding the game, instant and
immediate closes).
