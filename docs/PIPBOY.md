# Pip-Boy: mouse and keyboard

Branches `claude/m1-pipboy` (2026-10-05) and `claude/m2-pipboy-live`
(2026-10-06). Owner of the Pip-Boy menu code (`crates/ui/src/pipboy/`,
`viewer/src/pipboy.rs`, `viewer/src/game_menus/asks.rs`). Executable:
FalloutNV.exe 1.4.0.525. Decompiler exports stay private.

## Why the mouse didn't work (found live, 2026-10-06)

The first batch was unit-tested only. Driving the running release viewer
with real Windows input (PowerShell, `user32` `SetCursorPos` /
`mouse_event` / `keybd_event`, screenshots by `CopyFromScreen`) showed:
hovering highlighted rows and tabs, but **no click ever happened**. The
Pip-Boy keeps the mouse from the rest of the viewer by clearing Bevy's
`ButtonInput<MouseButton>` every frame (`reset_all`); a cleared button is
no longer "pressed", so Bevy never reports it coming up, and the menus'
click (press and release over the same tile) never completed. The
Pip-Boy now reads the buttons' own events (`MouseButtonInput`), every
frame, open or not (`viewer::pipboy::mouse_buttons`, regression test
`a_click_finishes_although_the_buttons_are_cleared`). The cursor itself
was never grabbed or hidden (the viewer looks around only with the right
button held), so that was not the cause.

Also fixed while there: the Pip-Boy now runs after the game's own menus
and leaves the input to one open over it (a question, "how many?"); a box
closing over the Pip-Boy no longer makes the player ready to walk.

## Verified live (release build, injected input, screenshots)

| What | How |
| --- | --- |
| Tab opens; mouse-over chooses rows (SPECIAL list), tabs highlight | `GSDocMitchellHouse`, Tab, cursor moved over rows/tabs |
| Click a tab (S.P.E.C.I.A.L.) turns the page | left click on the tab |
| Model buttons ITEMS / DATA switch menus (lamp lights) | left click on the arm's buttons |
| DATA tabs by click (Quests) | left click |
| World map: highlight box follows, nearest marker named | `Goodsprings`, F3 |
| Zoom: wheel (in/out) and Page Down / Page Up | wheel notches, PgDn/PgUp (extended keys) |
| Right click on the map: "Do you want to set your marker?" Yes / No; Yes places the marker icon there | right click, then click Yes |
| Click a marker: "Do you want to travel to Goodsprings?"; No stays; Yes lowers the Pip-Boy, then travels (to the Prospector Saloon front) | left clicks |
| ITEMS right click on Stimpak (7): "How many?" over the Pip-Boy; arrows to 3, Ok; list shows Stimpak (4); the dropped reference comes into view | `--run "player.additem Stimpak 7"` |
| STATS healing mode: hover the damaged left leg (meter at 40%), click: Stimpak (3) → (2), the leg's meter to about 80%, health a tenth | `--run "player.damageav LeftMobilityCondition 60"`, F1 |
| ITEMS buttons: Mod lit with the pistol chosen, Repair dim; both dim with nothing chosen | F2, hover |
| Keyring: Misc ends with "Keyring" (its picture); click lists Becky Hostetler's Key and Van Graff Key with "Cancel E)"; E closes it | `--run "player.additem VFSVanGraffKey 1"` … |
| Hot keys: a number key held over ITEMS shows the hot key wheel (the chosen key highlighted); the 9mm Pistol row clicked with it shown goes on that key (its icon on the wheel); Pip-Boy put away, that number key equips the pistol (HUD condition bar appears) | F2, number key held, click, Tab, number key |
| Notes: a text note shows its text, an image note its picture (DCTA Metro Map), an audio note (Post-War Audio Log) plays with "00:04:3 remaining" counting down | DATA › Misc after `player.additem` of the notes |
| World map: the active quest's target shows as `glow_hud_compass_objective_marker.dds` | DATA › Map with a quest running |
| Knobs: the tab knob turns to the tab chosen, the scroll knob turns on row changes / zoom | screenshots before / after a tab click |
| Pip-Boy light: Tab held past the timer lights the corridor walls in front of the player (off again the same way) | `GSDocMitchellHouse`, Tab held 1.8 s, compared with light off |
| Dropped items: a Stimpak dropped (3 → 2) lies on the carpet in front of where the view faces; E on it: "Stimpak added", gone from the floor, Stimpak (3) | ITEMS right click, Tab, look down, E |
| Radio: DATA › Radio lists Mojave Music Radio; a click tunes it (filled square), a song plays on the radio deck (`mus_lazy_day_blues`, in the log) | `Goodsprings`, DATA › Radio tab, click on the row |
| Local map indoors: Doc Mitchell's house laid out from above, unexplored rooms dark, door marker named on hover, the arrow turning with the view (west, then north), wheel zoom | `GSDocMitchellHouse`, `--pipboy data:0`, mouse turns, wheel |
| Local map outdoors: Goodsprings' ground and objects around the player, the fog of war's edge soft, wheel zoom out shrinks the map round the arrow | `Goodsprings`, Tab, wheel |
| Escape over the Pip-Boy opens the pause menu on top of it | Tab, Escape ([START_MENU.md](START_MENU.md)) |
| Scripts drive the same radio: `PipboyRadio on 0016B74C` fills Mojave Music Radio's square in DATA › Radio and its song plays on the radio deck; `PipBoyRadioOff` 6 s later clears the deck and the place's music comes back | `Goodsprings`, `--run`, `--run-at 6`, `--pipboy data:4` |
| `SunnyREF.SetNPCRadio 1 0016B74C` (Pip-Boy radio off): Sunny plays `mus_lazy_day_blues_mono.ogg`; `0016B74C.StartRadioConversation` restarts her with the new programme's song (`mus_i_m_movin_out_mono.ogg`); `SetNPCRadio 0` and nothing more plays | `Goodsprings`, `--run-at 3/9/14` |
| The 2 key, weapon out: a tap swaps 9mm Round → Hollow Point, a 1.5 s hold swaps back; no hot key used | `--walk --weapon WeapNV9mmPistol`, `--key-at 6 2`, `--key-at 10 2:1.5` (these inject into `ButtonInput` only, so the HUD hot keys, which read key events, weren't exercised) |

Pictures are kept privately in `%USERPROFILE%\nv-re\work\pipboylive-2026-10-06`,
`%USERPROFILE%\nv-re\work\pipboycomplete-2026-10-06`,
`%USERPROFILE%\nv-re\work\startmenu-2026-10-06` and
`%USERPROFILE%\nv-re\work\radiounify-2026-10-06`.

## Repair and Mod

The Repair menu (ITEMS R, id 8), the weapon mod menu (ITEMS X, id 0x13)
and merchant repair are Chazm's (merged 2026-10-06, see
[CONTRIB_CHAZM.md](CONTRIB_CHAZM.md)): [REPAIR.md](REPAIR.md) and
[WEAPON_MODS.md](WEAPON_MODS.md). Clicking R or X with an item that can be
mended (`world::repair::can_repair`, `00781860`) or a weapon chosen opens
the screen over ITEMS (`ui::pipboy::repair`, `ui::pipboy::item_mod`). While
one is up it is the menu on top: the interface hands it the pointer (its
`DoEnter`, `DoLeave` and `DoClick`, through `ui::pipboy`'s `Code`), the
wheel moves its list's scroll bar, ITEMS' right-button drop doesn't run,
and the pointer's tiles are let go as it opens and closes (2026-10-07,
`claude/repair-mod-followups`; the details in REPAIR.md and
WEAPON_MODS.md). For screenshots `--menu-pointer` and `--menu-click` put
the Pip-Boy's pointer on a pixel and click there.

## Traced behaviour (implemented)

| Behaviour | Addresses |
| --- | --- |
| Cursor on the screen: pick `pipboyscreen`, x = u·H·4/3 (`01078128` 1.333333), y = v·H; model buttons `PipBoyButton01..03` (+0xe8..), press stores the index (`011a0ba0`), release on the same one gives 1..3 | `007f8720`, caller `008761e0`, stored by `00709b80` (+0x4ac/+0x4b0/+0x4b4/+0x4b8) |
| Model button: `UIMenuMode`, then STATS/ITEMS/DATA | `0070c4a0` (0x0070d84f), `00704c10`, `007048f0`, `00704170` |
| No tile under the cursor with a Pip-Boy menu on top and the cursor off the screen | `007126c0`, `00717990`, `00717920` |
| Mouse over/off, click, drag, wheel via the interface | `0070c4a0`, `00717e70`, `00717ef0`, `00718080` |
| ITEMS: row 0x1d mouse-over chooses + card + `UIPipBoyScroll` when listindex changes; off clears card; click equips/uses; tabs 0x18..0x1c with the knob | `00780ff0`, `00781620`, `00780140`, `00782470` → `007fa0f0`, `007f8610` |
| ITEMS Drop: right button going down with an item chosen and Tab up clicks 7 (`00781ba0`); the pad's X (`xbuttonx` → `IM_DropButton`, shown with a pad only; not clickable: `UIMenuCancel`) clicks it too (`0070c4a0`, `0070f6e0`); refusals in turn (`world::items::drop_refusal`): quest item → `sDropQuestItemWarning`, in the air → `sNoJumpWarning` (equipped during an action → `sDropEquippedItemWarning`, the action not tracked); count above `iInventoryAskQuantityAt` (5) → "How many?" from all of them (`007aba00`, callback `00780c50`), else one; dropped `fPlayerDropDistance` (100) + the item's size in front, one reference keeping the count | `00780140` case 7, `00780c50`, `009614b0` |
| STATS: rows chosen on mouse-over with per-page knob click; status buttons 0x1c/0x1d/0x1e (+0x35/0x37/0x39 hardcore) | `007dc1b0`, `007dfd20`, `007db380`, `007e0160`, `007e0060` |
| STATS healing mode: the pointer onto a damaged limb (the file makes only those targets) with Stimpaks (Doctor's Bags in hardcore) aims the body part controller at it, `UIPipBoyHighlight`, turns healing mode on; leaving the limb turns it off; a click on it presses the Stimpak's (hardcore: Doctor's Bag's) button. An effect of archetype 34 or 35 added while it's on is aimed at that limb's condition (`00823210` → `00589f50`, part → actor value `007df9c0`); a "value and parts" effect then gives that part all of it and health × `fMagicVACPartTargetedMult` (`0082b970`) | `007dc1b0` case 0, `007dc490`, `007db380`, `007e0230` |
| ITEMS keyring: keys (`KEYM`) never in the tabs (`00782620`); with keys carried the Misc tab ends with a row `sKeyring`, id 0x1e (`00782a90`, sorted last by `007824e0`); its row shows `item_keyring.dds`; clicked, `_KeyringOpen` and the keys listed (`00782810`); Cancel (10) or another tab closes it; Drop there plays `UIVATSInsufficientAP` | `00780140` cases 0x1e, 10, 7; `00780ff0` |
| ITEMS buttons: no item chosen, Equip/Drop/Repair/Hot key/Mod not targets; with one, Equip as equippable, Drop and Mod yes, Hot key unless ammunition, Repair when repairable (weapon or apparel below full condition, two or more carried or a mender from its `REPL` list carried unequipped, `00781860`) | `00781680` |
| DATA: quest rows 0x17 (click = active quest unless finished), notes 0x18, radio 0x19; tabs 0x20..0x24; map click (id 4) presses the highlighted marker when the map didn't move since the press (`00798480`, `MapMenu::DoDownClick` (Xbox PDB), stores the map's place) | `00798cb0`, `00796fd0` |
| Fast travel: `0093d660`'s refusal first; then "%s %s?" (`sTravelQuestion`, the marker's name), Yes / No (`00703e80`, callback `00798710`); Yes puts the Pip-Boy away and travels once it's down (`0070f690` → `00798a00`) | `00796fd0` case 0x1a |
| Player's own marker: right button on the map area (`0079a130`, `00a23a50(1,1)`, control 0xe up) clicks 0x0c; point = (pointer − map) ÷ map width, `MapToWorldCoord` (`0079c450`); `UIPopUpMapMarkerAdded ` (with the exe's trailing space); no marker: `sSetMarkerQuestion` Yes/No numbered from 3; else `sMoveMarkerQuestion` Move It / Remove It / Leave It; callback `00798840` sets (`00952e60`) or removes (`00952f90`); shown as `WorldMapQuestMarkerTemplate` with `glow_hud_compass_pc_marker.dds`, id 0x1c (`0079f360`); saved (`custommarker` line) | `00796fd0` case 0x0c |
| World map border: a place's share × 0.796875 + 0.1015625 on the picture (`01075010`, `01075008`), back × 1.2549 − 0.12745 | `0079c380`, `0079c450` |
| Zoom: wheel ÷120 > 0 in by 1.1 (`01056528`), Page Down in / Page Up out by 1.2 (`01018204`); magnification clamped to `fWorldMapMinZoom` 0.75 .. `MaxZoom` 5; marker, quest marker, arrow sizes lerped; the point under the window's middle kept; `UIPipBoyScroll` via the knob when it changed | `0079c530`, `00799790` 0x0f/0x10, `0079c5a0` |
| DATA map tabs: highlight box centred on the pointer, cursor alpha 0 inside 0..850 × 0..500 (`01074f68`, `010301a8`), nearest marker within half the box height, `UIPipBoyHighlight`, "Companion" keeps the title | `0079a130`, `00799dc0` |
| Hot keys (controls 0x11 + n, n = 1 being ammo swap, left out): the number key held over ITEMS shows `IM_HotKeyWheel` (id 5) with each key's icon (`_HotKeyAssigned`, `_HotKeyIcon`, its name in `string`; `_SelectedHotkey` / `_SelectedText`); a row clicked while it shows puts the item on that key and off any other (`007019e0` → `004bf800`); broken items `sCantHotkeyBrokenItem`, others that can't `sCantHotkeyItem`; with the Pip-Boy away the key coming up equips / takes off / uses it; kept in a save | `00781ba0`, `00701e00`, `007017b0`, `00701e80` |
| Notes (`NOTE` DATA kind): 0 sound (`SNAM`), 1 text (`TNAM`), 2 image (`XNAM`), 3 voice (topic's responses, speaker); text into the data text, image into the data image (`_ItemType` 3/1/2); sound and voice play their pieces 500 ms apart; "%02d:%02d:%01d remaining" while playing, "--:--:- remaining" / "00:00:0 remaining" otherwise; `UIPipBoyHolotapeStart` | `007993d0`, `0079a660` |
| World map quest targets: `WorldMapQuestMarkerTemplate` with `glow_hud_compass_objective_marker.dds`, id 0x1b, sized and lit as the map's markers; a target inside goes to the last exterior door on the door path to it, else the player | `0079e0a0`, `0079f7e0` |
| STATS aimed limb blinks: its picture's alpha pulsed 255 → 0 over 1 s (the face, id 24, too for the head) | `007dbfa0` → `007dbfe0` → `007dc040` |
| Knobs: tab knob positions `fTabKnobMin` .. `fTabKnobMax` in 4 steps, moved at `fTabKnobMoveRate`; scroll knob by `fScrollKnobIncrement` at `fScrollKnobRate` on `UIPipBoyScroll`; rad needle at π/2 − rads/1000·π; nodes `TabKnob` (Z), `ScrollKnob` (X), `RadNeedle`; rotations as `004168a0` | `007f99d0`, `007f8320`, `007f8610`, `007fa0f0` |
| Pip-Boy light: `PipBoyLight` → `PipLight` (archetype 13) → `PipboyLight640` (194, 245, 209); radius `fMagicUnitsPerFoot` × (magnitude + `fMagicLightRadiusBase`); placed the bound's far y + `fMagicLightForwardOffset` forward, height + `fMagicLightHeightOffset` up; lights the place's surfaces (not the first-person view) | `0080e970` |
| Dropped items: lie on the static collision below the drop point (ray down, the item's `OBND` bottom on it) in front of the view's heading; listed with the place's things E can take; taken, the reference goes | `world::more_functions::placed::{dropped_items, taken}` |
| F1/F2/F3 (DIK 0x3B..0x3D, raw keys): open on STATS/ITEMS/DATA, switch, or close on the shown one; Tab release closes | `0070c4a0`, `00a24180`, `0070f4e0`, `0070f690` |
| Boxes over the Pip-Boy are ordinary menus on the main screen (only files whose `id` is `&pipboymenu;` draw on its screen); the HUD keeps its messages over the Pip-Boy's pages | `menus\*.xml`, `ui::hud::parts_for_menu` mode 3 |
| Menu class numbers: Stats 0x3eb, Inventory 0x3ea, Map 0x3ff | `00717920`, rtti vtables `0106ffd4`, `010739b4`, `01074d44` |
| Hot key controls: Hotkey1, Ammo Swap, Hotkey3..8 are controls 0x11..0x18 (names `011a7e60`); their keys from the INI's `[Controls]` lines, else the exe's defaults (the digits 1..8); each control looks its key up on its own (`00a24660`) | `00a24b70`, `00a24660`; `viewer::controls` |
| The 2 key (control 0x12): the player's controls swap ammunition when it goes down (`0093e860`, `fPlayerAmmoSwapTimer`); the HUD's hot keys (`HUDMainMenu::UpdateHotkeysWheel` (Xbox PDB), `0077da60`) treat it as slot 1: down only selects slot 1 on the wheel (`0061cc40(1)`, no name, `00701bd0` not called), held never shows the wheel (`n != 1`), up uses slot 1 (`UseHotkeyItem`, `00701e80`); nothing can be put on slot 1 (the Pip-Boy's wheel skips it on PC and pad: `00781ba0`, `00701bd0(1)` gives −1, `007017b0`), so tap or hold, the 2 key only swaps ammunition | `0077da60`, `00781ba0`; `viewer::pipboy` |
| Radio stations (`TACT` with flag 0x20000, not 0x10000000): in range by `XRDO` (radius, everywhere, worldspace, linked ref), signal strength truncated, rows sorted in-range first then by name; click (row 0x19) tunes / turns off; the music director held while a station plays; a station's programme from its scripts (`RadioHello` etc.), songs on the radio deck in step, DJ lines through the dialogue voice paths 50 ms apart; static volume ((100 − s)/100)^0.75; plays with the Pip-Boy away; saved (`radio`, `radiofound` lines) | `004ff1a0`, `00833d00`, `0061b440`, `00796fd0` case 0x19, `011dd313`; `world::radio`, `viewer::radio` |
| One radio for the Pip-Boy and the scripts (`FalloutRadio` (Xbox PDB)): `PipboyRadio` on / off / tune and `PipBoyRadioOff` switch and tune the radio DATA › Radio shows and plays (`008324e0`, `00832240(station, 1)` with a range pass now; no station: the first in range); `StartRadioConversation` replaces a station's programme with one from the topic (default `RadioHello`), 50 ms on, stopping what it played (`00835be0`); `ForceRadioStationUpdate` a range pass (`00832ad0(1)`) | `005d7fb0`, `005dc580`, `005d82a0`, `005d8280`; `world::more_functions::radio` |
| `SetNPCRadio 1` (`EnableNPCRadio`, `00835810`): a receiver (`FORadioReceiver` (Xbox PDB)) on the station of that base already playing (`00832830`), else the placed one in range of the person (`00832cb0` → `008356e0`); `0` (`DisableNPCRadio`, `00835980`) silences it. A station with receivers runs though the Pip-Boy isn't on it (`00834260`); each receiver starts the line when the player is within hearing (creature: `fCreatureRadioMax` 2000 × 1.1; else the station base's `SNAM` sound, or `AMLRadio`, largest attenuation × 1.1; else 3000), in step with the line; a song as its `_mono` file streamed as `.ogg` (`00af1e00`); in another worldspace / interior than the player it stops for good. Saved: `npcradio`, `radioconversation` (and old saves' `scriptradio` read as the `radio` line) | `00834260` (disassembly), `00553b90`, `0104fca0`, `011dd62c`; `world::radio`, `viewer::radio` |
| Local map: 5 × 5 tiles of 4096 units (interiors: offset 2048 and turned by the `NorthMarker`), each a 128 × 128 picture from above (outdoors the land's top + 20000, indoors the bound's top + 40000, half-size 2176, outdoors 32 lower), colour the view-space normal × 0.5 + 0.5 (`SLS2084.vso`/`SLS2087.pso`), cleared black; one picture a frame; tiles drawn as 17 × 17 meshes, vertex alpha = seen count / 4, picture × Pip-Boy colour | `0054e830`, `0054ee80`, `0054f500`, `0079ffb0`, `00556870` |
| Fog of war: 16 × 16 points a cell (256 apart), set within `fSeenDataUpdateRadius` (1024) of the player, fully-seen at 248 points; saved (`seen` lines) | `00555c20`, `00556ef0` |
| Local map markers: doors (`icon_local_door.dds`, alpha the fog's, name where they lead: cell name, else worldspace), quest targets and the custom marker (`LocalMapQuestMarkerTemplate`), the arrow at −heading − 0.3 − north; zoom `fLocalMapMinZoom` 0.1 .. `MaxZoom` 0.9, marker sizes 20..75 / 40..75 / 40..70 | `0079d410`, `0079dbb0`, `0079e0a0`, `0079c5a0` |
| Local map scale, re-traced for B16 (found as implemented): a tile is 1024 menu units a cell (2048 indoors) × `_Magnification`, the picture's `filewidth` = that × `uGridsToLoad` (5) (`0079ffb0`: `0x400 (+0x400 indoors)`, `0xfcd`), the 3D tiles scaled by the same magnification (`0079c5a0`), so the 855 × 500 window shows 855 ÷ (1024 × 0.9) ≈ 0.93 cells outdoors, 0.46 indoors at the start; the start is the file's `_Magnification` 1 clamped to 0.9 (`0079c5a0(1, 1.0)` from `0079d410`; `fLocalMapMaxZoom` `0x3f666666` at `011d2f64`, `MinZoom` `0x3dcccccd` at `011d4c60`, neither in the ESMs), `min(max)` as `0040ebd0` / `00404010`; steps: wheel ×1.1 a call (`0079c530`), Page Up / Down (bumpers) ×1.2 (`00799790` cases 0xf / 0x10), the pad's left stick up / down (axis 8, dead zone 7849) × up to 1.075 a frame; pan: the mouse drags the map (`x` adds `dragdeltax`, kept between the window's middle and the map's far edge by the file), the pad's right stick (axes 9 / 10, dead zone 8689) sets `dragdeltax` = −x × 20 / 32767, `dragdeltay` = y × 20 / 32767 a frame; centred on the player (`0079dbb0`: window ÷ 2 − width × the player's place) whenever `0079d410` runs: the tab shown (`0079af30` case 0x20, so DATA put up or the tab chosen again) and the last picture drawn (`0079ffb0`); zoom keeps the window's middle (`0079c5a0`) | `0079c5a0`, `0079c530`, `00799790`, `0079d410`, `0079dbb0`, `0079ffb0`, `0079af30`; `ui::pipboy::data` (`zoom`, `sticks`, `shown`), `ui::tile` |
| A drag's movement is added to the map once: the game works `x` out when `dragdeltax` is set (`00a012d0`), not on every refresh; here a trait that adds `dragdeltax` / `dragdeltay` is worked out again only when a source is set or changes (before, every refresh added the last drag's movement again and the map ran off to its limits) | `00a012d0`, `0070c4a0` (the drag's frame code); `ui::tile::Ui::value` |

Labelled guesses: the menus' camera covering 1280 × 960 units over the
4:3 rectangle (so menu point = u × 1280, v × 960); the marker distance
measured between screen centres (the game uses `_x`/`_y` in the map
frame); the dropped item's size as half its `OBND` diagonal (the game's
`0050ebf0` isn't traced) and its height the player's; the drop's sound as
the item's put-down sound; the dropped item set on the ground by a ray
straight down (the game casts its shape, `009614b0`, then physics); the
hot keys' default keys as the digits 1..8; the light's brightness 1 (what
`0080ed20(255)` sets isn't traced); the player's bound read from its
`OBND`. Local map: which objects are drawn (the game's runtime "Show in
Local Map" flag is set by code not traced; here every non-actor object
not flagged hidden (0x00800000) with a bound of 50 or more), faces turned
away left out (the pass's cull mode isn't traced), the normal per vertex,
the map tile's own square not drawn under the pictures, outdoors the
camera's height from the square's highest vertex, the pictures made again
when the place changes; a map dragged with the pointer off the Pip-Boy's
screen stays where the pointer last was on it (the game's drag delta is
the change of two counts at `+0x60` / `+0x64` of the input object, whose
start and scale in `0070c4a0` aren't traced; here the pointer's change in
menu units); the pad's sticks are written from the code and unit-tested
only (no pad to try). Radio: the ranges are straight-line (no path
finding); script functions take the last frame's audio clock as now; a
receiver's sound is taken to end with the station's line (its length the
song's own file, not the mono one); receivers' loudness as the Pip-Boy's
(their sound's category, flags 0x500102, not traced); vfunc +0x21c read
as "is a creature" (it picks `fCreatureRadioMax`); `TESSound` +0x45 read
as the largest attenuation byte; one receiver a person (the game lists
the same receiver again when a person is given a second station).

## Tests

- `ui::pipboy::tests::the_mouse_chooses_equips_and_turns_tabs_on_items`
- `ui::pipboy::tests::the_models_buttons_and_the_world_maps_markers`
- `ui::pipboy::tests::the_status_pages_mode_buttons`
- `ui::pipboy::data::tests::zooming_the_world_map_keeps_its_middle`
- `ui::pipboy::tests::the_right_button_drops_and_marks_the_map`
- `ui::pipboy::tests::the_keyring_opens_and_closes`
- `ui::pipboy::tests::items_buttons_follow_the_chosen_item`
- `ui::pipboy::stats::tests::healing_mode_aims_a_stimpak_at_a_limb`
- `world` `limbs::crippled_legs_slow_and_the_player_takes_half_limb_damage` (aimed Stimpak), `limbs::an_aimed_effect_keeps_its_part_in_a_save`
- `ui::pipboy::gather::tests::places_land_on_the_world_map_between_its_corner_cells`
- `world::map::tests::places_and_points_on_the_world_maps_picture`
- `world` `more_functions::the_player_drops_items_in_front`
- `viewer pipboy::tests::the_pointer_lands_on_the_screens_picture`
- `viewer pipboy::tests::a_click_finishes_although_the_buttons_are_cleared`
- `world` `more_functions::the_pipboy_radio_and_its_stations` (script
  functions on the Pip-Boy's radio, saves old and new),
  `more_functions::a_person_plays_a_station_near_the_player`,
  `radio::tests::a_receivers_song_is_its_mono_ogg`
- `ui::pipboy::tests::number_keys_put_items_on_hot_keys`,
  `ui::pipboy::tests::the_aimed_limb_blinks`,
  `ui::pipboy::data::tests::notes_show_by_kind_and_play`
- viewer `pipboy::tests` for the knobs, rotations, the light's point,
  audio lengths and hot key events

Generated cut-down menus and plugins; no game files.

## NOT compared with the original game

Nothing here has been compared side by side with the original game; the
live checks above are of the viewer only. Not implemented:

- The aimed limb's blink, the rad needle and the scroll knob's turn are
  only checked in tests, not seen live.
- With the light on the Pip-Boy's screen looks washed green: that is the
  light's cone (`PipboyLightEffect`, `007fa310`) over the arm, as before;
  not compared with the game.
- STATS: the pad's X toggle and the buttons' texts with a pad
  (`007df620`), the Stimpak / Doctor's Bag buttons' `target` by
  `007e05f0` / `007e06f0`; "limb condition" effects (archetype 35, the
  Doctor's Bag) do nothing in the world yet, aimed or not.
- ITEMS: the hot keys' pad
  assignment; the other Drop refusals (the player's current action,
  worn items that can't come off, no room), the drop's shape cast,
  ten headings and physics; the sort's ties by condition and equipped.
- DATA: the local map and the radio are implemented and seen live in the
  viewer only (pictures, fog, colours, marker placement not compared with
  the game). Not done: the radio's waveform picture (`MM_Waveform`),
  Radio New Vegas's news lines seen live (only the station's enable path
  is traced), challenges, waypoints on the local map, the custom marker
  on the compass, `OutputLocalMapPictures`, `ToggleFogOfWar`.
- Radio receivers: not placed in the world (no distance falloff or
  direction once started, as the viewer's other voices); the actor
  saying a line of its own voice with lip sync (`00834260`, vfunc +0x284)
  isn't done; a station's programme position isn't saved (a conversation
  a script started begins again after a load); the Pip-Boy tuned to a
  station mid-line (receivers already playing it) waits for the next line;
  stations from radio templates (`004fd3c0`) aren't made; the radio's
  "disabled" flag (`011dd436`) is taken as clear.
- Buttons moving, the PC button-label textures, held keys repeating,
  swapped mouse buttons (`+0x1b4c`), the game's own cursor speed (the
  system pointer is used).

## Equip sounds, the arm's swap, container sounds (2026-10-08, `claude/pipboy-equip`)

| What | Game | Viewer |
| --- | --- | --- |
| ITEMS Equip / take off | The row's click (`00780140` case 0x1d) calls `00780d60`, which calls `0088c790` (carried and worn: take off) or `0088c650` (put on) with the sound flag set. The player's sound is `008aded0` -> `008adcf0`: put on, `008aded0(item, 1, 0)` = the item's own pick-up sound (`YNAM`), else a weapon's `UIItem{GunsBig,GunsSmall,Melee}Up` by `ETYP`, else `UIItemGenericUp`; taken off (`0088d7d0`), `008aded0(item, 0, 0)` = its own `ZNAM`, else the `...Down` ones. A refused equip (a broken item, `0088c830`) returns before it. Read a book (`0088c830` case 0x19): `008aded0(book, 1, 1)`, the flag set = `UIItemGenericUp`. Aid keeps `ENIT`'s consume sound (case 0x2f). | `Action::Equip` plays `world::sound::item_sound` (up on equipping, down on taking off, only when it changed); a read book plays `UIItemGenericUp`. Seen live: pistol `ITMPistolUp` / `Down`, machete `UIItemMeleeUp`, armour `UIItemsClothingUp` / `Down`. |
| The arm after a change of what's worn or held | The Pip-Boy's own model stays up. | The first-person arm is rebuilt on a change; the old one was despawned at once, the new one unseen (and its screen picture unset) for a frame or two, so the arm blinked out on every equip. The new arm stays hidden until its screen has its picture (or 3 frames), the old one kept until then (`pipboy::Arm::waited`, `Pipboy::retired`). Not seen frame by frame (screenshots are single frames); the lists, markers, header and scrolling were checked live with 15 weapons and 6 armours and behaved. |
| Container open / close sounds | `0075baf0`: a container's `SNAM` / `QNAM` only (`CONT` +0x8c / +0x90). Most containers (lockers, ammunition boxes, footlockers, cabinets: of 40 sampled `CONT`s, 24 have neither) have none; their sound is the `Sound:` text key of their model's `Open` / `Close` sequence (`LockerCabinet01.NIF`: `Sound: DRSLocker02Open` at 0.00 s), fired as a door's (`004eef00`). Models of containers that have `SNAM` have no such sequence. | `game_menus::container::model_sounds` plays those keys when the menu opens and closes (`Private Crenshaw's Footlocker`: `0004E906` on opening, seen live). The lid itself isn't animated yet. |
