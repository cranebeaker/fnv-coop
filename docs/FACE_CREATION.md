# Face creation: M1 working evidence

## Current research handoff

Active branch `codex/m1-face-menu-research`, based on camera PR #7
(`5b3ec14`, local checks/live camera verification complete; amended CI running).
Appearance PR #6 merged as `843926d`; its verified binary is in play/app,
SHA256 `9391A5176CD9A829B99642C9252EE945FC7E6BDB6E255A72E2DAC6750BA9FD98`.
Saves/reports were preserved. Camera publication waits for its own merge.

The bounded native page/control trace below is complete; no public runtime
edits or agent processes remain for it. Private source snapshots, findings
and hash manifests: nv-re/work/m1-overnight-2026-10-04/face-menu.
No original-game menu comparison succeeded this session. Next: trace the
remaining preset/morph field meanings and palette application before
implementing edits. The immediate callback findings follow below.

## Native category navigation and slider construction

For the same 1.4.0.525 executable identified below, setup `007acb60` creates
20 category/header objects through `007b39e0`, not 20 face sliders. Assembly
uses a label array at EBP-0x5c; inferred decompiler local-array boundaries are
misleading. Installed XML separately defines list-item and slider templates.

| IDs | Categories in order |
| --- | --- |
| 0–4 | Sex, Race, Face, Hair, Customize |
| 5–9 | Hair Style, Hair Color, Facial Hair, Eye Color, Shape |
| 10–14 | General, Forehead, Brow, Eyes, Nose |
| 15–19 | Mouth, Cheeks, Jaw, Chin, Tone |

Normal setup links 0 ↔ 1 ↔ 2 ↔ 3 with `007b4050`; 0 starts and 3 is final.
Barber mode 2 starts/ends at 3; surgeon mode 3 starts/ends at 2. Their first
four labels omit the numeric prefixes used by normal setup. Child insertion
`007b3bb0` records the parent as Back and adds a labeled `>` choice:

- Face 2 → Customize 4 (`007ad0a3`).
- Hair 3 → Hair Style 5, Hair Color 6, and conditionally Facial Hair 7
  (`007ad125`, `007ad148`, `007ad180`; the latter requires `005f0cc0()==0`).
- Customize 4 → Shape 9, Tone 19, Eye Color 8
  (`007ad1b5`, `007ad1d8`, `007ad1fb`).
- Shape 9 → categories 10 through 18 (`007ad3ee` loop).

Header previous/next links initially use sentinel 20. Click `007adce0` handles
Back tile 4 through previous and Next tile 5 through next; on the final page,
Next enters the confirmation callback `007ada40`. Page activation `007af180`
updates root user0, refreshes dirty hair/eyes choices on pages 5/8 and hair
root state on page 3, and derives Back/Next visibility from these links.
Done replaces Next on the final page. These are native control-flow findings,
not a reproduced menu or confirmation outcome.

Slider constructor `007b3ca0` uses explicit current/min/max arguments. Helpers
`007b3890` and `007b3760` set bounds and clamp the current value. Its separate
float argument is a jump fraction, clamped to [0,1], multiplied by inclusive
range size and quantized at step 1; it writes user4 (`0x1008`). It is not an
initial selection fraction. Construction calls supply ranges 1–20 (ID 0x18),
1–10 (0x1a), and 0–15 (0x1d), jump fraction 0.25, and a signed sentinel current
value that clamps to each minimum. RGB controls 0x1e–0x20 use ranges 0–255,
jump fraction 0.125, and bytes from `004169d0`. Later refresh may overwrite
these constructor values; native field mappings and final visible values
remain to be traced. XML snapshot SHA256:
`1C5E9DAA5AA5EB9AE11044718874D0D27CB3665EC994487B2CC77A828805AF98`.

The selected-control dispatcher `007b4b50` applies ID 0x18 through the
authored preset list at menu+0xdc, indexed by current value minus one;
`00603790` receives the selected record. It refreshes category callbacks,
morph rows and preview through `007b25a0`, `007b2b50(1)`, `007b27c0`, then
resynchronizes color controls. Preset record contents remain untraced.

On hair-color page 6, `007af900` handles IDs 0x1e–0x20 through `007b3660`:
read current R/G/B, pack `R | G<<8 | B<<16`, write preview actor+0x1d8 via
`007b3720`, then call `007b2b50(0)`. That path tints geometry named
`FaceGenHair` and `FaceGenAccessory` and updates material/scene state.
Palette ID 0x1d reads authored `iHairColor%02d` and `sHairColor%d` settings
and updates RGB controls/label, but its subsequent application path is not
yet proved. Control 0x1a changes two indexed morph-array values; exact
constant precision and semantic field names remain under review. Private
`face-menu/CALLBACK_FINDINGS.md` retains the branch/call evidence. No new
Rust helper is justified solely by the existence of these controls.

## Race/sex part reconciliation, 2026-10-04

Implemented `appearance::reconcile_parts` and `PartSelection`, as core
groundwork; the viewer face editor remains unimplemented and auto-accepts.
No gameplay integration or original-game fallback comparison is claimed.

Ghidra `007b1ca0` retains hair/eyes when the selected race contains the part
and its sex flags permit it (`005fdfa0`, `005fc5f0`). These predicates do not
require the playable bit. Invalid hair first uses the selected sex's RACE
DNAM slot (`00613870`, race+0x94+sex*4; loader `00610cd0`, setter `00613890`).
Only a zero slot scans the race's HNAM list for the first playable,
sex-compatible hair. This corrects the older note's global-list inference:
assembly shows `004ac110 -> 0045bb80`, the race's list at+0x8c. A nonzero
authored default does not recheck membership, playable or sex restrictions.
Invalid eyes use the first loaded race ENAM entry (`007b1e50`, +0xa8),
without playable/sex filtering. The visible choices retain their stricter
filters and independently unresolved display order.

The loader skips malformed non-multiple-of-four lists, omits unresolved or
wrong-type entries, then deduplicates and appends resolved forms in source
order (`00613810`/`00613910`). Reconciliation respects owner-plugin FormID
mapping and winning/deleted records. Malformed DNAM is explicitly unsupported:
the native fixed-eight-byte reader's malformed-input behavior is not traced.
Unresolved nonzero default hair fails closed; native fixup semantics for
that invalid data are not claimed.

Installed official data gives these authored defaults (male/female hair):
African American `000306BE`/`0005DC78`, Asian `00014B90`/`00022E50`, Hispanic
`000A9D6F`/`0005DC76`, Caucasian `00014B90`/`0005DC6B`. All four have Blue
`00004253` first in ENAM. These are data observations, not a native UI replay.

Six generated regressions cover compatible non-playable retention, sex changes,
unfiltered defaults, HNAM order versus global choice order, first loaded
ENAM, unresolved list entries/defaults, malformed DNAM, and reordered-master
mapping, and last-pair-wins duplicate DNAM. Core919/viewer84 tests, both
clippy and formatting checks, and both release builds pass on
`codex/m1-native-appearance`. Installed-data new-game smoke reached the
situp/bedsit camera idles and Doc's opening speech, then exited after its
inspected screenshot; stderr contained Vulkan present-mode warnings only.
This is a startup regression check, not face-menu acceptance. A private
implementation harness passed all eight official race/sex cases and retained
each returned selection unchanged; `appearance/implementation-check/run.log`.
Private evidence and hashes:
`nv-re/work/m1-overnight-2026-10-04/appearance/{FINDINGS.md,provenance.json}`.
Analysis executable 1.4.0.525 SHA256:
`19406942E48724D797300C4EA6BE9AC69A32F670B8C35DB09279A2258422739F`.
Next: persistent editable appearance and native preview/menu callbacks.

## Earlier evidence

Original-game reference inspected 2026-10-03 after the user opened ShowRaceMenu:
the Reflectron cabinet fills most of the view, oval live head/upper-body preview
on the left, green sex-selection screen on the right (Male selected, Female,
Next), and Sex/Race/Face/Hair labels with indicator lamps below the preview.
The room remains visible and blurred behind it. This confirms the starting
page's presentation only, not the unobserved editing controls or transitions.
Opening animation failures reported by the user take priority before extending
this menu; see OPENING.md.

2026-10-03. Choice reader implemented; no replacement menu published yet.
Executable hash and shared priorities: OPENING.md and MILESTONES.md.
Decompilation stays outside the project in nv-re/decomp/codex-m1.

ShowRaceMenu handler005cedf0 calls00705870 with mode1 outside the script
thread flag, mode0 inside. Barber005cee30 passes2; surgeon005cee50 passes3.
00705870 opens immediately or schedules mode5 with the argument retained.
007ac730 loads Data/Menus/CharGen/race_sex_menu.xml. The original XML
successfully parses through nvinspect (codex-m1/racesex-menu.txt).

RaceSexMenu constructor007ac1f0; vtable01075974. SetTile007ac500 accepts
IDs0..5 into+78. Click007adce0; drag start007ae1b0; update007ae420;
special keys007aecb0. Update checks whether the menu is active before input.
Setup007acb60 loads Terminals/NV_reflectron_UI.NIF for the opening;
BarberInterface01.NIF and PlasticSurgeryInterface01.NIF for modes2/3.
The opening starts page0, barber3, surgeon2. Detailed page-tree/preview
behavior still needs tracing; do not invent controls or a substitute panel.

Choice rules traced so far:
- Setup enumerates races filtered by0059f610: runtime flags at+70 bit1.
  Existing functions.md maps this field to RACE DATA u32 at32.
- Hair page007af300 enumerates the global hair list, requires playable
  (005fdf40, hair+48 bit1) and005fdfa0: membership in the actor's race list,
  then sex restriction (005fdf60: male allowed when bit2 clear;
  005fdf80: female allowed when bit4 clear).
- Eyes page007af450 similarly requires playable005fc4d0 and005fc5f0
  membership/sex restrictions. Eyes flags are at+30; bits1/2/4 have the
  same meanings.005fdcb0 (hair) and005fc220 (eyes) each copy the single
  DATA byte into those fields.00610cd0 copies36-byte RACE DATA to+50;
  HNAM and ENAM are mapped form arrays, skipped unless length is divisible4.
- Changing race/sex validates current hair/eyes in007b1ca0; the defaults,
  ordered race-owned fallback and eye fallback are now traced above.

Existing infrastructure: actor::ActorLook and Face hold assembled models
and symmetric/asymmetric morphs; Game::actor_scene builds their meshes.
Player first_person_look deliberately has no head. GameState/save currently
persist only player_name/player_female for identity, no editable race/hair/
eyes/morphs. Menu creation needs those fields, selection rules, a preview
and native XML callbacks before replacing the current face auto-accept.

The menu's pages, items, clicks and sliders: [RACE_SEX_MENU.md](RACE_SEX_MENU.md).

## Implemented and checked

`world::chargen::appearance::{races,hair,eyes}` enumerates eligible choices
from winning records, rejects deleted entries, resolves membership IDs via
the race record's owning plugin, and keeps actual FULL names (missing names
stay missing). Native menu ordering is not yet established. No default
selection or appearance mutation is invented here.

Six generated-input regressions cover flags/sex, malformed lists (including
a valid ID followed by a bad tail), wrong/missing records, deleted winning
overrides, master-index remapping, and missing/empty labels. A surviving race
is also checked against deleted hair/eyes, independently of a deleted race.

Real official-data harness outside the project:
`nv-re/work/codex-m1/appearance-observations`; `appearance-real-data.log`.
Observed four playable races: African American, Asian, Hispanic, Caucasian.
Each exposes18 male/23 female hair choices and4 eyes per sex. These are data
observations, not yet a native menu comparison. Full check status: OPENING.md.

Original game process40680 opened via Steam (no capture DLL installed).
Computer Use keys/clicks did not affect its main menu, including after an
explicit focus retry. User was asked to load a save and open the tester with
ShowLoveTesterMenuParams40 for reference inspection. No original saves changed.
