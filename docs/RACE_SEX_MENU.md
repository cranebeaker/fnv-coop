# The face menu (RaceSexMenu)

Research status, 2026-10-04. `FalloutNV.exe` 1.4.0.525 addresses. How
`ShowRaceMenu` opens it, its models and the choice predicates:
[FACE_CREATION.md](FACE_CREATION.md). This file covers the menu's pages,
items, clicks and sliders. Nothing here is implemented yet beyond
`world::chargen::appearance` (choices, race/sex fallbacks).

The menu edits the player's base NPC (`+0xd8`, set in setup `007acb60`).
Labels are game settings (strings in `FalloutNV.esm`'s `GMST`s); in modes 0
and 1 (the opening) the four top pages are numbered "1. %s" … "4. %s".

## Pages

Twenty page objects (`007b39e0(index, label)`, kept at menu `+0x28 + 4 ×
index`), each a list of items made from two XML templates:
`RSM_list_item_template` (`007b3b80(label, type)`) and
`RSM_slider_option_template` (`007b3ca0(label, type, min, max, value,
start)`). A link to another page is a list item "label >" whose type is the
page's index (`007b3bb0`). Each page can have a predicate (page `+0x1c`)
that marks the item matching the NPC as selected.

| page | label | items (setup `007acb60`) | selected when |
| --- | --- | --- | --- |
| 0 | `sRSMSex` | `sMale` (type 0x15), `sFemale` (0x16) | the NPC's sex matches (`007ad940`) |
| 1 | `sRSMRace` | every race passing `0059f610` (0x17, the race; its name) | the NPC's race (`007ad7f0`) |
| 2 | `sRSMFace` | slider `sRSMPreset` (0x18, 1–20); link `sRSMCustomize` → 4; `sRSMRandomize` (0x19); slider `sRSMAge` (0x1a, 1–10) | |
| 3 | `sRSMHair` | links `sRSMHairStyle` → 5, `sRSMHairColor` → 6, `sRSMFacialHair` → 7 (only for a male NPC; otherwise page 7 is hidden, `0041fd00(3)`) | |
| 4 | `sRSMCustomize` | links `sShape` → 9, `sTone` → 19, `sRSMEyeColor` → 8 | |
| 5 | `sRSMHairStyle` | hairstyles (0x1b), filled when opened (`007af300`) | the NPC's hair (`007ad860`) |
| 6 | `sRSMHairColor` | slider `sRSMPreset` (0x1d, 0–15) with text `sRSMCustom`; sliders `sRSMRedAbbrev`, `sRSMGreenAbbrev`, `sRSMBlueAbbrev` (0x1e–0x20, 0–255, from the NPC's hair colour `004169d0`) | |
| 7 | `sRSMFacialHair` | head parts (0x1c) from the global head-part list passing `005e7a60` | the NPC's head part (`007ad9c0`) |
| 8 | `sRSMEyeColor` | eyes (0x21), filled when opened (`007af450`) | the NPC's eyes (`007ad8d0`) |
| 9 | `sShape` | links to pages 10–18 | |
| 10–18 | `sRSMGeneral`, `sForehead`, `sBrow`, `sEyes`, `sNose`, `sMouth`, `sCheeks`, `sJaw`, `sChin` | FaceGen shape sliders (0x22), see below | |
| 19 | `sTone` | FaceGen texture sliders (0x23), see below | |

Each page keeps a previous and a next page (`+0xc`, `+0x10`, 20 for
none, `007b39e0`). A link sets the linked page's previous page to the
page it is on (`007b3bb0`); in the opening `007b4050` chains Sex → Race →
Face → Hair as next pages (previous the other way).

Opening a page (`007af180(page)`, pages below 20 only) refreshes it
(`007b4410`); pages 5 and 8 refill their lists first when their refresh
bits (0, 1 of `+0xec`) are set, and page 3 runs `007af520`. The menu
remembers the current page in trait `0x1004`.

## Clicks (`007adce0`, virtual 3)

* With no item under the click, by tile (`SetTile` `007ac500` keeps tiles
  0–5 at `+0x78`):
  * tile 4 (Back): the current page's previous page (page `+0xc`,
    `0084e3a0`).
  * tile 5 (Next): on the menu's last page (`+0x120`: 3, Hair, in the
    opening and for the barber; 2 for the surgeon) a confirmation message
    box (text "%s %s?" built at `+0x98`, callback `007ada40`) to finish;
    elsewhere the next page (page `+0x10`, `0044edb0`).
  * tiles 2 and 3, when trait `0xfa3` allows: the selected slider one step
    down or up (`007b4530(±1)`, then `007af6b0`).
  * otherwise an item under the selected tile starts a slider drag: the
    item and tile are remembered (`+0xc0`, `+0xc4`), `007b4930`.
* An item, by type:
  * 0x15/0x16 (sex), 0x17 (race): unless already selected (`007ae160`:
    `_selected` is 2), set the race (`0060b240`) or sex (`0047dd50`,
    female for 0x16; 0x16 first empties the NPC's head-part list), then
    `007b5180`, `007b25a0`, the hair and eyes fallbacks (`007b1ca0`, see
    FACE_CREATION.md) and, for sex, `008b78c0(1)`.
  * 0x19 Randomize: `007b51f0`, after a confirmation unless refresh bit 7
    is set.
  * 0x1b hairstyle: unless selected, set the hair (`006031e0`) and rebuild
    the head (`007b2b50(1)`).
  * 0x1c head part: the NPC's head-part list emptied, then this one added
    unless it was the selected one (so a click on the selected one removes
    it); `007b1e70(1,1,0)`.
  * 0x21 eyes: unless selected, set the eyes (`00603200`), rebuild them
    (`007b27c0`).
  * a type below 20: open that page (`007af180`).
  * After any change: `007b40d0` refreshes the items and bit 7 of `+0xec`
    is cleared.

## Sliders (`007b4930`, `007b4b50`)

A slider's parts (tile ids): 100 one step down, 104 one step up, 102 and
103 a page down/up (trait `0x1008`), 105 the thumb (drag, while refresh bit
2 allows). For the hair-colour preset (0x1d) the arrows first try the
previous/next preset by name. A changed value is applied by `007b4b50`:

* on page 6, the hair colour (`007af900`);
* 0x22/0x23: the FaceGen control (`007af770`), then the face is rebuilt
  (`007b18f0`; `007b1e70` when refresh bit 8 is set);
* 0x18 Preset: the race's preset face number n − 1 (`00877a30`) is copied
  onto the NPC (`00603790`), every page refreshed and the whole head
  rebuilt (hair, eyes, head parts, hair colour, the hair-colour preset
  slider);
* 0x1a Age: FaceGen's age statistic. The NPC's face (`00603ad0`) and
  the race's (`+0x124`) are read (`00652440(…, 0, 0)` and `(…, 0, 1)`);
  new age = race's + (slider − NPC's), clamped to 15–65 (`00652470`);
  then the face is rebuilt.

### FaceGen sliders (`007afaf0`)

Each slider shows a FaceGen control's value (`00652230(face, kind, 0,
control)`, kind 0 shape, 1 texture), clamped to [min, max], as integers ×
10 (`01020758`), step 0.25 (`0101622c`). `fRSMFaceSliderDefaultMin` and
`…Max` are −5 and 5.

Shape (0x22): a table indexed by symmetric shape control (`ebp−0x600`, 16
bytes each: page, label, min, max), walked up to the control count
(`006521b0(0, 0)`). Controls not listed below have page 20 (none).

| control | page | label | range |
| --- | --- | --- | --- |
| 0, 1, 2 | 12 Brow | `sRSMShapeOption01`, `…02`, `…03` | default |
| 3, 4, 6 | 16 Cheeks | `…04`, `…05`, `…07` | default |
| 8, 9, 11, 13 | 18 Chin | `…09`, `…10`, `…12`, `…14` | default |
| 15, 16, 18 | 13 Eyes | `…16`, `…13`, `…19` | default |
| 21, 23 | 10 General | `…22`, `…06` | −2 to 2 |
| 24, 25, 26 | 11 Forehead | `…13`, `…14`, `…27` | default |
| 27, 28, 29, 30 | 17 Jaw | `…11`, `…15`, `…30`, `…07` | default |
| 35, 39 | 15 Mouth | `…36`, `…40` | default |
| 41, 43, 44, 45, 47 | 14 Nose | `…42`, `…16`, `…45`, `…46`, `…48` | default |
| 49, 53 | 14 Nose | `…50`, `…54` | default min to 3 |

Tone (0x23), page 19, in this order, symmetric texture controls: 29
`sRSMToneOption24`, 28 `…01`, 30 `…25`, 31 `…26`, 8 `sRSMEyeSockets`, 9
`sEyebrows`, 18 `sEyeliner`, 27 `sNose`, 22 `sLips`, 5 `sRSMToneOption06`,
6 `sCheeks`, 3 `sBeard`; all with the default range.

The controls themselves, their file and how a value is read and set:
[FACEGEN_CONTROLS.md](FACEGEN_CONTROLS.md). A slider change sets its
control to the slider's integer / 10 (`007af770`); the menu lists the shape
controls below `006521b0`, the count minus one.

## Still open

* `007af300`/`007af450` list order and contents beyond the predicates in
  FACE_CREATION.md; `007af520` (page 3); `007af6b0`; `007b4530`.
* Randomize (`007b51f0`, partly traced in FACEGEN_CONTROLS.md), the
  preset faces (`00877a30`), hair colour (`007af900`).
* Keys (`007aecb0`, virtual 14), the update (`007ae420`, virtual 11), the
  closing confirmation (`007ada40`) and what closing commits.
* The preview (camera, rotation, lighting).
