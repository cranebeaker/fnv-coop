# FaceGen controls (`FACEGEN\SI.CTL`)

Research status, 2026-10-04. `FalloutNV.exe` 1.4.0.525 addresses. The face
menu's sliders, its Age slider and Randomize all work on a face's FaceGen
coordinates through this file (pages and sliders:
[RACE_SEX_MENU.md](RACE_SEX_MENU.md)). Implemented: the reader
(`nif::Ctl`, `crates/nif/src/ctl.rs`) and the evaluation
(`world::chargen::facegen`), tested on synthetic files. Not yet compared
with the game's own `SI.CTL`.

## Coordinates

A face is four coordinate vectors, by kind and symmetry: shape symmetric
and asymmetric (an NPC's `FGGS`, `FGGA`), texture symmetric (`FGTS`) and
texture asymmetric (no record holds it). The face object keeps them at
`+kind × 0x40 + symmetry × 0x20` (`00652230`).

## Body tints made from the texture coordinates (B27)

An NPC's bare skin below the head (arms, hands, gloves' fingers) takes a
body tint as `FaceGenMap0` (`world::actor::MadeBodyTint`). `006149b0`
looks for `textures\characters\bodymods\<plugin>\<id>ModBody<Male|Female>.dds`
(names from `006147e0`, the folder from `006148f0`); when that isn't there
("Failed to find body mod texture '%s' for '%s' (%08X). Creating from
scratch.", not logged for the player, who never has one) it makes it:

- the race's body texture morphs for the sex (`006131a0`: race
  `+0x3c8 + sex × 0x18`, the race's body part 3, `UpperBodyHumanMale.egt`,
  32 × 32, 50 symmetric morphs; layout in `crates/nif/src/egt.rs`);
- the face's coordinates: the race's for the sex plus the NPC's own
  (`00652af0(race face, NPC face, out, 0, 0)` adds all four vectors), of
  which the texture symmetric ones (`FGTS`) are used;
- `0065b410`: per morph, `_ftol(scale × 256) × _ftol(value × 256)` (the
  256 is the double at `010231d8`) times each signed byte, summed as
  integers, × 1/65536; `0064ceb0(−255, 255, 0.5)`: each sum rounded down
  (`00404040`), kept within ±255, `(v + 255) × 0.5` stored truncated
  (so no change is 127).

Implemented: `nif::Egt` (reader and `tint`, tested on synthetic files),
`world::actor::MadeBodyTint` (the file, else the morphs and values), made
in `cellview`'s texture cache and laid on as the files are
(`TextureData::plus_face_tint`). The player's first-person arms and hands
and every NPC without a tint file get it.

## Loading

The FaceGen manager (`00651b30`, created by `00650ea0`) loads
`FACEGEN\SI.CTL` through `00aaa7d0` and logs "MODELS: Unable to load CTL
file" when it can't. The file is read in this order (32-bit little-endian
values; the binary reader's read is `(pointer, element size, count)`):

1. The magic `FRCTL001` (8 bytes; the binary file class checks it,
   `00ac5540`).
2. Six values (`00aaa7d0`, one 0x18-byte read): two kept by the manager at
   `+0` and `+4`, then the four basis sizes, shape symmetric, shape
   asymmetric, texture symmetric, texture asymmetric (manager `+0x1184`).
   The first two are not checked anywhere traced; **inferred**: basis
   identifiers like the one in an `.egm` header.
3. Controls (`00aaac10`, into manager `+0xb8 + kind × 0x30 + symmetry ×
   0x18`): for shape then texture, symmetric then asymmetric, a count, then
   per control as many coefficients as the basis has, a label length and
   the label.
4. Statistics (`00aaaf10`, manager `+0x118`): for five groups, for age
   then gender, for shape then texture, coefficients over the symmetric
   basis and an offset.
5. Group differences: for each ordered pair of different groups, shape and
   texture coefficients, then one offset.
6. Distributions, per group: the mean shape and texture vectors, their
   joint covariance (shape size + texture size, squared), the shape
   covariance and the texture covariance.

After reading, the loader works out per group and kind an orthonormal
basis of the age and gender directions (Gram–Schmidt, at `+0xd4c`; it
asserts both lengths exceed 0.001, FanControls.cpp lines 0x90 and 0x98)
and the inverse of the 2 × 2 matrix of their dot products (`+0xfcc`, via
`00aadd20`), then marks the statistics loaded.

Only the controls, the first group's statistics and that inverse are used.
The functions that read group differences, distributions and the
orthonormal basis (`00aac3f0`, `00aac820`, `00aacdb0`, `00aacf40`,
`00aad140`, `00aad500`, `00aadac0`) have no callers outside FanControls.cpp
itself.

## Controls

* Value (`00652230(face, kind, symmetry, index)`): the control's
  coefficients times the face's coordinates of that kind and symmetry (a
  1 × n by n × 1 product); 0 for no face or an index past the end.
* Set (`00652320(face, kind, symmetry, index, value)`): the coordinates
  gain (value − current value) × the coefficients. It does not divide by
  the coefficients' squared length, so unless that is 1 the control then
  reads something else; the reimplementation does the same.
* `006521b0(kind, symmetry)` is the control count minus one; the face menu
  lists controls below it, so it never shows the last shape control.

The menu's FaceGen sliders (`007afaf0`) show a control's value clamped to
the slider's range, × 10; a slider change sets the control to the slider's
integer / 10 (`007af770`; shape for item type 0x22, texture otherwise),
then rebuilds the face.

## Age and gender

* Read (`00652440(face, stat, kind)` → `00aabdb0` with group 0): the
  statistic's coefficients times the face's symmetric coordinates of that
  kind, plus its offset. 0 when the statistics are not loaded.
* Set both (`006524e0(face, kind, values)` → `00aac170`, group 0): each
  target clamped (age 15–65, gender −4–4), d = targets − current values,
  y = M d with M the stored inverse (`00aaf2b0`), and the coordinates gain
  y₀ × the age coefficients + y₁ × the gender coefficients: the smallest
  change that reaches both.
* Set one (`00652470(face, stat, kind, value)`): sets both, the other at
  its current value.
* A single-statistic setter (`00aabf30`, same clamps, moving along that
  statistic's own direction only) has no callers.

The face menu's Age slider and Randomize set age through `00652470` for
shape and texture separately (RACE_SEX_MENU.md).

## Randomize (partly traced)

The menu's Randomize (`007b51f0`) calls `00601830(1, 0, 1)` on the edited
NPC. What is read so far:

* `00652fc0(scale, from, to)` → `00aad6b0`: each symmetric coordinate of
  `from` plus a normal random number (polar method, `00ac6360`) × scale ×
  1.5, asymmetric coordinates 0. It does not use the control file. Where
  `from` and the scale come from (`005f0cc0`, `00403e20`) is not traced.
* The first argument 1 keeps the age; otherwise it would be 15 + 50 ×
  a random number. The shape and texture ages are set keeping their
  difference, each clamped to 15–65 (`00404010`) and passed through
  `0040ebd0`.
* The second argument 0 draws gender in −2 to 2, set the same way and
  clamped to −2–2.
* The third argument 1 skips `00601c10(random)`.

## Still open

* Compare with the game's `SI.CTL`: the counts, labels, and that the file
  ends after the distributions.
* Randomize's base face and scale, `0040ebd0`, and what `005f0cc0` and
  `00652e00` do with the result (the face rebuild).
* Whether the menu's faces are the NPC's own coordinates or the NPC's plus
  its race's (`00603ad0`; `world::actor` adds the race's for drawing).
