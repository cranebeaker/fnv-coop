# Start menu (pause menu on Escape)

`StartMenu` (menu class 1013, vtable `01076d1c`) from `menus\options\start_menu.xml`,
opened by the Escape control over the game or the Pip-Boy (`0070c4a0` at
`0070e651`, `007cb7d0`). Code: `ui::menus::start` (the menu),
`viewer::game_menus::start` (settings, saves, the background),
`viewer::game_menus` (`escape_opens_start_menu`, `start_menu_frame`).

## Verified live (release build, injected input to the viewer's own window, screenshots)

| What | How |
| --- | --- |
| Escape pauses the game and fades the menu in over the `PauseScreen01.NIF` background | `GSDocMitchellHouse`, Escape |
| Settings › Audio: a meter click and the arrow keys move Music (0.48 → 0.44), heard in the music volume | click, Left/Right |
| Escape on a sub-page goes back a page | Escape |
| Save › [NEW SAVE]: "Saving..." then the menu closes after 3 s; `nv-rs-save-0001.txt` written | click |
| Load: the saves list (newest first), a click asks with the unsaved-progress warning beside the row; Yes loads ("Loaded.") | clicks |
| Quit: Main Menu / Exit Game / Cancel; Cancel returns; Exit Game ends the process | clicks |
| Continue resumes the game | click |
| Escape over the Pip-Boy opens it on top | Tab, Escape |

Pictures are kept privately in `%USERPROFILE%\nv-re\work\startmenu-2026-10-06`.

## Traced behaviour (implemented)

| Behaviour | Addresses |
| --- | --- |
| Option lists: main `011dab88`, user options `011dab50`; pages by mask (1 title, 2 pause, 4 settings, 8 gameplay, 0x10 display, 0x20 audio, 0x40 controls, 0x80.. confirms); flags at +0x1a8 (pause, settings changed, save mode, apply, delete, save asked, saving, back to saves) | `007cc6e0`, `007d42b0` |
| Value lists pushed in reverse (Off/On, Very Easy..Very Hard, HUD colour Green/Blue/Amber/White, texture Large/Medium/Small, kill cam None/Player/Cinematic); shown values | `007d4ce0` |
| Fades 0.25 s straight (`01012054` = −1 → `0101622c`), clicks 500 ms apart, saving waits 3000 ms | `007d61e0`, `00a07c60` |
| Mouse sensitivity v = i/(n−1) × 0.0095 + 0.0005 (default 0.15789475); gamma 1.4 .. 0.6 | `007cf6a0` |
| Confirm boxes: placed beside the chosen row (y = row y + height × 1 or 2; x = list x − question x + question width + scrollbar width + `_text_offset`), the row marked `_selected` | `007d40a0`, `007d48a0` |
| Saves list: "[NEW SAVE] n/1000 saves used.", "[NO SAVES FOUND]"; delete and back-to-saves keep the choice | `007d3c70` |
| Back / Escape / click dispatch, meter drags | `007d0e40`, `007cf5e0`, `007ce9b0`, `007cf6a0` |
| Background: `PauseScreen01.NIF`'s three planes drawn flat (screen x = tile x + 1.3333 × x, y = tile y − 1.3333 × y, far first), `Slide01` additive, `Slide02` multiply, `Glare` additive | NIF properties |

Help (`007d0770`) shows the version tile and opens the help manual in
the tutorial menu ([TUTORIALS.md](TUTORIALS.md)). The title picture
(`main_title`, id 3) is sized from its own picture (`filewidth` /
`fileheight`); the viewer sets those on every open game menu's pictures
(`game_menus::draw_menus`), as the game's tile refresh does. In the pause
menu the title is hidden, so nothing there changes.

Saves are the viewer's own text saves ([PERSISTENCE.md](PERSISTENCE.md)):
quick, auto and numbered `nv-rs-save-NNNN.txt` in the working directory.

Labelled guesses: the black under the background planes; the
"Saving..." text tile.

## NOT compared with the original game

Nothing here has been compared side by side with the game. Not
implemented: Main Menu (the viewer has no main menu: the choice is shown
and only logs), settings written back to the INI
(`SavePreferences` does nothing), the Delete path and most settings
pages beyond Audio not checked live. The controller and rumble rows are
hidden without a pad (as traced); not checked with one.
