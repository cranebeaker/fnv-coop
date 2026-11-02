# The original game's saves (`.fos`)

M7 research (the former task list): the layout of Fallout: New Vegas's own save
files, what nv-rs needs from them to continue a game, and what nv-rs is
missing to take it. Branch `claude/fos-saves`.

Status: **format researched and checked** against real saves: every
change form in all nine files decodes to its exact length, references,
actors, their AI processes and the player included. A read-only reader
(`crates/fos`) and inspector (`nvinspect fos`) are implemented and
tested, and `world::fos_import` builds nv-rs's `GameState` from a save
(see [Import](#import); `nvinspect fos-import` shows what it takes).

- Every structure below was read from the writer (and, where it matters, the
  reader) in FalloutNV.exe 1.4.0.525 (addresses are PC unless marked) and
  named from the Xbox 360 prototype's symbols where they help (Xbox PDB).
- Every structure was then checked against nine real saves made by the
  1.4.0.525 game (five named saves, the autosave and quicksave and their two
  `.bak` copies; 1.65 to 1.87 MB, all early game, plugin list `FalloutNV.esm`
  only). All nine walk from the first byte to the last with no byte left over,
  and every one of their 3,585 to 4,509 change forms decodes to its exact
  length (36,322 in all). The saves themselves stay private.
- Parts no file holds were read from the code alone and are marked so
  ("from code"); the decoder reads them, but only a save that has them
  can confirm them.
- Community pages (UESP's save format pages) were not needed; nothing below
  is taken from them.
- The `.nvse` files next to the saves are the script extender's co-saves, a
  separate format, not covered here.

## Immutable inventory sources

`fos::inventory_source::InventorySource` captures an owned immutable save
and preserves its REFR/ACHR/ACRE inventories as original item, stack and
extra byte spans. Capture runs inside the existing complete reference and
actor decoders, including the player's path. It does not introduce another
record parser or change the existing public decoded structures. ItemChange
save/load boundaries remain those of `004bed60` / `004bee00`, and extra-list
boundaries those of `00426a30` (class names from Xbox PDB).

Each item retains the saved ID and its resolution through that save's own
form-ID array, the signed item count delta, ordered stacks, and its complete
record/save context. An absent count or health extra remains distinct from
an explicitly stored default value. Health bytes retain their raw bits.
Record-relative and file spans remain correct when Havok data rebases the
reader. Cloned envelopes own their immutable source lifetime; no engine
arena pointers or mutable gameplay state are stored here.

Extras with known lengths but opaque semantics retain their entire original
payload, even when the existing decoded enum only names their kind. Unknown
lengths, malformed fields and failures after the inventory preserve the
whole containing record with an explicit failure and publish no partial
items. Unresolved holders are conservatively preserved undecoded; unresolved
items in a completely decoded record retain an explicit resolution status.
Records without an inventory flag are reported without decoding other fields.

All 38 package tests and strict package lint pass, including eleven new
generated regressions covering source boundaries, opaque payloads, failures,
raw float bits, actor/player paths, Havok rebasing, separate save contexts
and ownership. Full workspace checks pass (2026-10-12). This read-only API does
not remap to installed plugins, import gameplay state, edit inventory or
write an original save. Engine ownership and whole-save acceptance remain
tracked in [ENGINE_STATE.md](ENGINE_STATE.md).

## Conventions

- Little-endian throughout. `u8/u16/u32/i32/f32/f64` as usual.
- **Pipe buffer.** Most of the file is written through `BGSSaveGameBuffer`
  (Xbox PDB), which appends a `|` (0x7C) after every value it writes
  (`00865ce0`; `00865570` with its last argument 0). Below, `x|` means "x
  then a `|` byte". Parts written straight to the file have no `|`.
- **String** `wstr`: `u16 length|` then, only if the length isn't 0, the
  characters (no terminator) and `|` (`00865e70`).
- **vsval** (variable-sized value, `BGSSaveGameBuffer::SaveVariableSizedValue`
  (Xbox PDB), `00865ff0`): the low two bits of the first byte give the size,
  0 = 1 byte, 1 = 2 bytes, 2 = 4 bytes; the value is the whole little-endian
  number shifted right by 2. Then `|`. The writer first puts a `#`
  placeholder (`00865f20`) and widens it once the count is known, so counts
  written before their items always use this encoding. Values of 2^30 or
  more can't be stored (the writer logs an error).
- **refID** (`BGSSaveLoadFormInfo` reference, `00853570` writes, `00853500`
  reads): 3 bytes, most significant first (`b0 << 16 | b1 << 8 | b2`).
  - bit 0x800000 set: a form created in game; its form id is
    `0xFF000000 | (refID & 0x7FFFFF)`.
  - otherwise: an index into the **form id array** (below), counting from 1;
    0 is "none". The writer assigns indices in the order it first meets each
    form (`00846c90`).
- Form ids inside the form id array carry the **save's** plugin index in
  their top byte. The loader maps that index to the current load order by
  plugin name, ignoring case (`00847660` builds `cFileIndexArray` (Xbox PDB);
  `00846d80` remaps); a form whose plugin is missing becomes 0 and is
  dropped ("SAVELOAD: Cannot find file %s referenced in the save game").
  `0xFF` ids are not remapped.

## File layout

Written by `008503b0` (the save driver): header `0084d4b0` (through
`00850ea0`), then `BGSSaveLoadGame::SaveGame` (Xbox PDB) `00847850`. Read by
`00850760` → version check `00850ed0` → `BGSSaveLoadGame::LoadGame` `00847df0`.

| Part | Layout | Notes |
| --- | --- | --- |
| Magic | `"FO3SAVEGAME"` (11 bytes, raw) | The FO3 name kept. |
| Header size | `u32` (raw) | Bytes of the header block that follows (140 in every file seen). |
| Header | pipe buffer, see below | |
| Screenshot | `width × height × 3` bytes, RGB, raw | Dimensions from the header. |
| Minor version | `u8` (raw) | `cCurrentMinorVersion` (Xbox PDB). The writer uses `00851110` = 0x1B (27); every change form repeats it. |
| Plugins | `u32 size` (raw), then pipe buffer: `u8 count|`, `count × wstr` | `00847590`. Size covers the pipe buffer. |
| File location table | 0x6E bytes (raw), see below | Written as zeros first, filled in at the end (`00847850`). |
| Global data table 1 | `count1 ×` global data | Types 0 to 11, ascending (`0084bb50(file, 0, 12)`). |
| Change forms | `changeFormCount ×` change form | |
| Global data table 2 | `count2 ×` global data | Type 1000 only (`0084bb50(file, 1000, 1001)`). |
| Form id array | `u32 count`, `count × u32` form id (raw) | `pFormIDMap` (Xbox PDB), `00846e00`. Element *i* is refID *i + 1*. |
| Worldspace id array | `u32 count`, `count × u32` form id (raw) | `pWorldspaceFormIDMap` (Xbox PDB), same writer. Indexed by the exterior-cell initial data (below). In every file seen: one entry, `WastelandNV`. |
| History | `u32 size` (raw), pipe buffer: `u32 count|`, `count × wstr` | `pHistory` (Xbox PDB), `0084e0d0`. Empty (count 0) in every file seen. End of file. |

### Header (pipe buffer, `0084d4b0`)

| Field | Layout | Source |
| --- | --- | --- |
| Version | `u32|` | `0066d730` = 0x30 (48). The loader refuses anything but 0x2F or 0x30 ("outdated" / "comes from a newer executable", `00850ed0`). |
| Language | 64 bytes, NUL-padded, then `|` | The game's language setting (`ENGLISH`). |
| Screenshot width, height | `u32|`, `u32|` | 512 or 432 wide, chosen by a test of the display's proportions against 0.75 in `0084d4b0`; height from the capture (`00878f60`). Every file seen: 512 × 288. |
| Save number | `u32|` | `BGSSaveLoadManager` (`011de134`) +8, incremented after a save unless `00851a20` or `00851a60` holds for its name (apparently the autosave and quicksave tests; the files agree) (`008503b0`). |
| Player name | `wstr` | Player's full name (`0055d520`). |
| Karma title | `wstr` | From level and karma (`0047e0e0`). |
| Level | `u32|` | Player level (`0087f9f0`, low 16 bits). |
| Location | `wstr` | The place shown in the load menu (`00851b40`). |
| Play time | `wstr` | `"hhh.mm.ss"` (`00851bf0`). |

### File location table (0x6E bytes)

| Offset | Field |
| --- | --- |
| 0x00 | `u32` offset of the form id array |
| 0x04 | `u32` offset of the history block (just past the worldspace id array) |
| 0x08 | `u32` offset of global data table 1 |
| 0x0C | `u32` offset of the change forms |
| 0x10 | `u32` offset of global data table 2 |
| 0x14 | `u32` count of global data table 1 (12) |
| 0x18 | `u32` count of global data table 2 (1) |
| 0x1C | `u32` count of change forms |
| 0x20 | 0x4E bytes, zero (unused) |

The loader seeks to the form id arrays first (`00846e70` twice), then reads
the global data and change forms.

## Global data

Each entry: `u32 type`, `u32 size`, `size` bytes (raw header, the data is a
pipe buffer). Types (`BGSSaveLoadGlobalData::GLOBAL_DATA` (Xbox PDB);
switch `0084bcd0`, loader `0084be40`):

| Type | Name (Xbox PDB) | Writer | Layout (as far as decoded) |
| --- | --- | --- | --- |
| 0 | `GLOBAL_DATA_MISC_STATS` | `MiscStatManager::SaveGame` `004d5fa0` | `u32 count|` (43), then `count × u32|`: the Pip-Boy's misc statistics in the order of the name table `01189280` (`004d5c80` builds them in that order), i.e. `world::stats::NAMES`. **Decoded.** |
| 1 | `GLOBAL_DATA_LOCATION` | `SaveLocationData` `0084c490` | `u32|` next created form id (`TESDataHandler::iNextID`, +0x208), refID `|` of the current worldspace (`TES::pWorldSpace`, +0x88), `i32|` `i32|` current grid x, y (`TES` +0x24, +0x28), refID `|` of the player's worldspace (or parent cell when inside), `3 × f32` player position `|`, then `LoadingMenu::SaveGame` `0078d6b0` (refID `|`, three 4-byte values `|`, `u8|`; not interpreted). **Decoded** except the loading-menu part's meaning. |
| 2 | `GLOBAL_DATA_TES` | `TES::SaveGame` `00459490` | vsval count, each refID `|` + `u16|`; then `u32 n|` and `n × n` refIDs `|` (n = 5 in every file, apparently the loaded cell grid). Not interpreted further. |
| 3 | `GLOBAL_DATA_GLOBALS` | `SaveGlobals` `0084cc90` | vsval count, each refID `|` (a `GLOB`) and `f32|` value. Constant globals (form flag 0x40) are skipped. Game time lives here (`GameHour`, `GameDaysPassed`, `GameDay`, `GameMonth`, `GameYear`, `TimeScale`). **Decoded.** |
| 4 | `GLOBAL_DATA_PROCESS_LISTS` | `ProcessLists::SaveGame` `00975840` | Four `u32|` then five vsval-counted lists. Not decoded (nv-rs rebuilds its process lists). |
| 5 | `GLOBAL_DATA_COMBAT` | `CombatManager::SaveGame` `009932d0` | Not decoded. |
| 6 | `GLOBAL_DATA_INTERFACE` | `Interface::SaveGame` `007066d0` | Not decoded. |
| 7 | `GLOBAL_DATA_EFFECTS` | `SaveEffects` `0084cf30` | vsval count, each `f32|` `f32|` refID `|`. Not interpreted. |
| 8 | `GLOBAL_DATA_WEATHER` | `Sky::SaveGame` `0063eb70` | refIDs `|` of `pCurrentWeather`, `pLastWeather`, `pDefaultWeather`, `pOverrideWeather` (`Sky` +0x10 to +0x1C), then `fCurrentGameHour`, `fLastWeatherUpdate`, `fCurrentWeatherPct`, `uiFlags`, `fAccelBeginPct`, `WaterFogColor` (3 × `f32`), `fFogHeight`, `fFogPower`, `eMode`, each `|` (names: Xbox `Sky`, same offsets). **Decoded.** |
| 9 | `GLOBAL_DATA_ACTOR_CAUSES` | `ActorCause::SaveActorCausesList` `0066e6d0` | Not decoded. |
| 10 | `GLOBAL_DATA_RADIO` | `FalloutRadio::SaveGame` `008366e0` | `u32|` (`011dd428`), `u8|` radio on (`011dd434`), `wstr` (`011dd448`); vsval count of station states (`00835f10`: refID `|`, four `u8|`); vsval count of stations playing (`00836600`: refID `|`, then `00836110`: four values, the lines left as a vsval-counted refID list, `u8|` and, if set, a dialogue list as in the high process); refID `|` tuned (`011dd42c`), refID `|` lost (`011dd430`); vsval count of discovered stations, refIDs `|` (`011dd59c`). **Decoded.** |
| 11 | `GLOBAL_DATA_AUDIO` | `FalloutAudio::SaveGame` `0082ec40` | One refID `|`. |
| 1000 | `GLOBAL_DATA_TEMP_EFFECTS` | `004534f0` (empty on PC) | Always size 0 on PC. |

## Change forms

One record per changed form (`00865a30` for a form in memory, `00866200`
for one kept as an unloaded buffer from an earlier save; both write the same
record):

| Field | Layout |
| --- | --- |
| refID | 3 bytes (above) |
| Change flags | `u32` |
| Type | `u8`: low 6 bits the save type (table below), high 2 bits the size of the length field (0: `u8`, 1: `u16`, 2: `u32`; `00845d40`) |
| Version | `u8`: the minor version when written (27 in every file seen) |
| Length | `u8`, `u16` or `u32` |
| Data | `length` bytes, a pipe buffer |

The data is, in order (`00847850`):

1. **Initial data** for references and cells (`0084eb80`; kind chosen by
   `0084e730`, `BGSSaveLoadInitialData::INITIAL_DATA_TYPE` (Xbox PDB)),
   written as one block then `|`:

   | Kind | When | Bytes |
   | --- | --- | --- |
   | 0 none | anything else | 0 |
   | 1 exterior cell, char | `CELL`, flags 0x40000000 and 0x20000000 | `u16` worldspace index (into the worldspace id array, `0084e340` reads), `i8` x, `i8` y, `u32` detach time (8) |
   | 2 exterior cell, short | `CELL`, flags 0x40000000 and 0x10000000 | `u16` worldspace index, `i16` x, `i16` y, `u32` detach time (10, `0084e3c0`) |
   | 3 interior cell | `CELL`, flag 0x40000000 only | `u32` detach time (4) |
   | 4 reference location | reference (form types 58 to 64 or 105), flags 0x2 or 0x4, not 0x8 | refID of the cell, or of the worldspace for an exterior reference; `3 × f32` position; `3 × f32` rotation (27) |
   | 5 created reference | reference with a `0xFF` form id | refID cell/worldspace, position, rotation, `u8` flags (1; \|2 per `005653d0`; \|4 when the reference is a mobile object, vtable +0xFC, and `00574900` holds), refID of the base object (the leveled base when flag 0x40000) (31) |
   | 6 moved reference | reference with flag 0x8 (`CHANGE_REFR_CELL_CHANGED`) | refID cell/worldspace, position, rotation, refID of the editor-location cell, `i16` `i16` its grid (editor x, y >> 12), zeros if it has none (34) |

2. For a reference with `CHANGE_REFR_HAVOK_MOVE` (0x4): a vsval length and
   the Havok data written by `00562de0` (only when vtable +0xF0, the
   reference test, holds; nv-rs's `GameState::havok_moved` and
   `havok_velocity` describe the pose and velocities the game keeps).
3. The form's own `SaveGame` (vtable +0x54 on PC, as in the Xbox class
   layout) for the change flags it has. `CheckSaveGame` (+0x80) runs first
   and may clear flags but writes nothing.

The save type → form type table is `011a2428` (55 entries, set up by
`BGSSaveLoadFormInfo::InitSaveGameFormTypes` (Xbox PDB) `008535e0`; form
type names from `01187004`). Every change form in the nine files had the
record type its form has in `FalloutNV.esm`.

### Change flags and data by type

Change flag names are `CHANGE_TYPE` (Xbox PDB). Bit 0x1
(`CHANGE_FORM_FLAGS`) always adds `u32|` form flags first (`00484d60`,
`TESForm::SaveGame`).

| Save type | Form | `SaveGame` (PC) | Flags → data (in written order) | Status |
| --- | --- | --- | --- | --- |
| 9 | `QUST` | `0060e810` | 0x2 `QUEST_FLAGS`: `u8|` (`QuestFlag`: 0x01 enabled/running, 0x02 completed, 0x04 allow repeats, 0x08 allow repeated stages, 0x10 starts enabled, 0x20 shown in the HUD, 0x40 failed). 0x4 `QUEST_SCRIPT_DELAY`: `f32|`. 0x80000000 `QUEST_STAGES`: vsval stage count; per stage `u8 index|`, `u8 done|`, vsval item count, per item `u8 index|`, `u8 has log date|`, if so a `Date` (`u16` day of year, `u16` year) `|`. 0x40000000 `QUEST_SCRIPT`: script locals (below). 0x20000000 `QUEST_OBJECTIVES`: vsval count; per objective `u32 index|`, `u32 state|` (`QUEST_OBJECTIVE_STATE`: 0 dormant, 1 displayed, 2 completed, 3 completed and displayed). | **Decoded** |
| 7 | `CELL` | `00555630` | 0x2 `CELL_FLAGS`: `u8|` (cell flags & 0x60 \| flags2). 0x80000000 `CELL_SEENDATA`: exterior 32 bytes `|` (`SeenData` `0087a0c0`); interior vsval count, per part two `u8|` and 32 bytes `|` (`IntSeenData` `0087a4e0`). 0x4 `CELL_FULLNAME`: `wstr`. 0x8 `CELL_OWNERSHIP`: refID `|`. 0x40000000 `CELL_DETACHTIME`, 0x20000000 / 0x10000000: initial data only. | **Decoded** |
| 8 | `INFO` | `00484d60` | Flags only: 0x80000000 `TOPIC_SAIDONCE` has no data. | **Decoded** |
| 10, 11 | `NPC_`, `CREA` | `00608f00`, `005fb330` | Actor base (`005f1f30`): 0x2 `ACTOR_BASE_DATA`: 24 bytes `|` (the `ACBS` data, level included); 0x10 `ACTOR_BASE_SPELLLIST`: two vsval-counted refID lists; 0x4 `ACTOR_BASE_ATTRIBUTES`: 7 bytes `|` (S.P.E.C.I.A.L.); 0x8 `ACTOR_BASE_AIDATA`: 20 bytes `|`; 0x20 `ACTOR_BASE_FULLNAME`: `wstr`. `NPC_` then: 0x200 `NPC_SKILLS`: 28 bytes `|`; 0x400 `NPC_CLASS`: refID `|`; 0x2000000 `NPC_RACE`: two refIDs `|`; 0x800 `NPC_FACE`: `u8|`, the FaceGen coefficients as `f32|` (geometry symmetric and asymmetric, texture symmetric and asymmetric; the counts are the NPC's own arrays at run time, the record's `FGGS`/`FGGA`/`FGTS` sizes 50, 30, 50, 0), two refIDs `|`, two 4-byte values `|`, a vsval-counted refID list; 0x1000000 `NPC_GENDER`: `u8|`. `CREA` 0x200: three `u8|`. | **Decoded**; `NPC_FACE` from code (no file has it) |
| 34 | `FACT` | `005fd690` | 0x4 `FACTION_REACTIONS`: vsval count, per entry refID `|`, `i32|` modifier, `i32|` group reaction (`0048c9d0`). 0x2 `FACTION_FLAGS`: `u32|`. 0x80000000 `FACTION_CRIME_COUNTS`: `i32|` major, `i32|` minor (`iMajorCrime`, `iMinorCrime`). | **Decoded** |
| 33 | `CLAS` | `005f7150` | 0x2 `CLASS_TAG_SKILLS`: `4 × i32|` actor values (−1 unused). | **Decoded** |
| 50 | `CHAL` | `005f5780` | Always `i32|` `iProgress`, `i32|` `nProgressFlags` (`CHALLENGE_PROGRESS`). | **Decoded** |
| 43 | `REPU` | `00616660` | Always `f32|` fame (`fPositiveValue`), `f32|` infamy (`fNegativeValue`). | Read from code; no file has one yet |
| 37 | `FLST` | `00590080` | 0x80000000 `FORM_LIST_ADDED_FORM`: `u32 count|` (not a vsval) then `count` refIDs `|` (the forms added in game). | From code |
| 38 to 40 | `LVLC`, `LVLN`, `LVLI` | `0050adb0` | 0x80000000 `LEVELED_LIST_ADDED_OBJECT` (`00488c50`): vsval count; per entry refID `|`, `u16|` level, `u16|` count, `f32|` health (−1 none). | From code |
| 16, 22, 28, 27, 47, 53 | `BOOK`, `MISC`, `KEYM`, `AMMO`, `CHIP`, `CMNY` | `00515340`, `0051b0d0`, `00504450` | 0x2 `BASE_OBJECT_VALUE`: `u32|` (`0048ea40`); `BOOK` 0x20 `BOOK_TEACHES_SKILL`: `u8|`. | From code |
| 15, 17, 21, 26, 42 | `ARMO`, `CLOT`, `LIGH`, `WEAP`, `IMOD` | `005144c0`, `0050e5c0`, `005230f0`, `0051a600` | Form flags and 0x2 `BASE_OBJECT_VALUE` `u32|` only. | From code |
| 12, 25, 13, 14 | `ACTI`, `FURN`, `TACT`, `TERM` | `00511860` (`ACTI`, `FURN`, `TERM` through `00501b90`), `004ff800` (`TACT`) | 0x4 `BASE_OBJECT_FULLNAME`: `wstr`; `TACT` then 0x800000 `TALKING_ACTIVATOR_SPEAKER`: refID `|`. | From code |
| 32 | `ECZN` | `00526410` | 0x2 `ENCOUNTER_ZONE_FLAGS`: `u8|`; 0x80000000 `ENCOUNTER_ZONE_GAME_DATA`: 16 bytes `|`. | From code |
| 35 | `PACK` | `006797c0` | Nothing for a package from a plugin (the writer only saves packages made in game, as in [Packages](#packages)). | From code |
| 41 | `WATR` | `005806e0` | 0x80000000 `WATER_REMAPPED`: refID `|`. | From code |
| 31 | `NOTE` | `00484d60` | Flags only: 0x80000000 `NOTE_READ`. | From code |
| 18 to 20, 23, 29, 30, 36, 45, 46, 48, 49, 51, 52, 54 | `CONT`, `DOOR`, `INGR`, `STAT`, `ALCH`, `IDLM`, `NAVM`, `RCPE`, `RCCT`, `CSNO`, `LSCT`, `AMEF`, `CCRD`, `CDCK` | `00484d60` (`LSCT`, `CCRD`: `0059add0`) | Form flags only. | From code |
| 24, 44 | `MSTT`, `PCBE` | ? | Their classes' `SaveGame` wasn't found; the reader reports them as not decoded. | Not decoded |
| 0 | `REFR` | `00562230` | Form flags, then 0x10 `REFR_SCALE`: `f32|`; the [extra data list](#extra-data) when any of 0xA4021C40 is set (0xA4061840 for actors; the table there says which extra types each flag brings); 0x20 `REFR_INVENTORY` or 0x8000000 `REFR_LEVELED_INVENTORY`: the [inventory](#inventories); 0x10000000 `REFR_ANIMATION` (not actors): vsval length and the animation's own data (`00563650`), kept whole. 0x2 `REFR_MOVE`, 0x4 `REFR_HAVOK_MOVE`, 0x8 `REFR_CELL_CHANGED` are initial data. 0x400000 / 0x800000 `OBJECT_OPEN_DEFAULT_STATE` / `OBJECT_OPEN_STATE` and 0x200000 `OBJECT_EMPTY` add no data. | **Decoded** |
| 1, 2 | `ACHR`, `ACRE` | `008d33a0`, `008aaf40` | [Mobile object, actor and mover](#actors); the player's reference adds [its own data](#the-player). `ACHR` (`Character`, `008d33a0`) ends with two `u8|`; `ACRE` (`Creature`) uses `Actor::SaveGame` itself. | **Decoded** |
| 3 to 6 | `PMIS`, `PGRE`, `PBEA`, `PFLA` | `009baaa0`, `00979bb0`, `00979bb0`, `009b3eb0` | [Projectiles](#projectiles) in flight. | **Decoded** (`PMIS`, `PGRE`); `PBEA`, `PFLA` from code |

**Script locals** (`ScriptLocals` (Xbox PDB), `005a9db0`; read by
`005a9f20`): vsval count; per variable `u32 id|` and either `f64|` (a
number) or, when the id has bit 0x80000000, a refID `|` (a reference
variable; the id is the low 31 bits); then `u8 has event list|` and, if so,
8 bytes `|`; then `u8|` (flag 0x1000), which the loader reads only when the
form's version is over 0x14 (20). This is the one versioned field met in the
decoded types; the other loaders read the version-27 layout written above.

On loading a quest, its current stage is not read from the file: it is set
to the highest stage marked done (`0060d670`).

### Extra data

`ExtraDataList::SaveGame` (Xbox PDB), `00426a30`: a vsval count, then
per entry `u8|` the extra type (`EXTRA_DATA_TYPE` (Xbox PDB)) and its
data. An entry is written only if the change flags meet the type's mask
in the table at `01183d30` (one `u32` per type); a mask with 0x40000000
is for created references only, and `EXTRA_ASHPILE_REF` is skipped for
one kind of buffer. Inside an inventory the flags are forced to 0x400.
The switch's jump table is at `00427ef0` (index `00427fe4`).

| Type | Name | Saved under | Data |
| --- | --- | --- | --- |
| 0x0D | `SCRIPT` | 0x80000400 | refID `|` script, then [script locals](#change-flags-and-data-by-type) |
| 0x16 | `WORN` | 0x400 | none |
| 0x18 | `PACKAGESTARTLOC` | 0x800 | refID `|`, 12 bytes `|` position, `f32|` |
| 0x19 | `PACKAGE` | 0x80000000 | two refIDs `|`, `u32|`, three `u8|` |
| 0x1A | `TRESPASS_PACKAGE` | 0x80000000 | refID `|`; if not 0, the trespass package's `SaveGame` (`009f9790`) — from code |
| 0x1B | `RUN_ONCE_PACKAGES` | 0x80000000 | vsval count, per entry refID `|`, `u8|` |
| 0x1C | `REFERENCE_POINTER` | 0x400 | refID `|` |
| 0x1D | `FOLLOWER` | 0x800 | vsval count of refIDs `|` |
| 0x1E | `LEVCREA_MOD` | 0x40000000 | `u32|` |
| 0x1F | `GHOST` | 0x80000000 | none |
| 0x21, 0x22 | `OWNERSHIP`, `GLOBAL` | 0x440 | refID `|` |
| 0x23 | `RANK` | 0x440 | `i32|` |
| 0x24 | `COUNT` | 0x400 | `i16|` |
| 0x25, 0x27, 0x28 | `HEALTH`, `TIMELEFT`, `CHARGE` | 0x400 | `f32|` |
| 0x26 | `USES` | 0x400 | `u8|` |
| 0x29 | `LIGHT` | 0x400 | none: the switch has no case for it (it logs "Unknown extra data type") |
| 0x2A | `LOCK` | 0x1000 | `REFR_LOCK` (Xbox PDB): `u8|` `cBaseLevel`, `u8|` `cFlags` (0x1 locked), refID `|` key, `u32|` `uiNumTries`, `u32|` `uiTimesUnlocked` |
| 0x2B | `TELEPORT` | 0x20000 | `DoorTeleportData` (`0043aa40`): 12 bytes `|` position, 12 bytes `|` rotation, `u8|` flags, refID `|` linked door |
| 0x2C | `MAPMARKER` | 0x80000000 | `u8|` `MapMarkerData::cFlags` (0x1 visible, 0x2 can travel to) |
| 0x2E | `LEVELEDCREATURE` | 0x40000 | refID `|` leveled base, refID `|` the base made from it, `u32|` that base's change flags, then that base's change data as an `NPC_` (for `ACHR`) or `CREA` change form with those flags |
| 0x2F | `LEVELITEM` | 0x400 | `u32|`, `u8|` |
| 0x30 | `SCALE` | 0x400 | `f32|` |
| 0x32 | `MAGICCASTER` | 0x80000000 | three refIDs `|` |
| 0x33 | `MAGICTARGET` | 0x80000000 | refID `|`, then an [active effect list](#actors) |
| 0x35 | `PLAYERCRIMELIST` | 0x80000000 | vsval count, per entry two 4-byte values `|` |
| 0x39, 0x3C, 0x3F, 0x46, 0x49, 0x55, 0x6C, 0x74, 0x89 | `ITEMDROPPER`, `MERCHANTCONTAINER`, `POISON`, `HEAD_TRACK_TARGET`, `STARTINGWORLDORCELL`, `TALKING_ACTOR`, `OPENCLOSEACTIVATE_REF`, `ENCOUNTERZONE`, `ASHPILE_REF` | 0x80000000; `MERCHANTCONTAINER` 0x1000; `POISON`, `STARTINGWORLDORCELL` 0x400; `ENCOUNTERZONE` 0x20000000 | refID `|` |
| 0x3E | `CANNOTWEAR` | 0x400 | none |
| 0x45 | `FRIEND_HITS` | 0x80000000 | vsval count of 4-byte times `|` (`009a5f90`) |
| 0x4A, 0x4E, 0x8D | `HOT_KEY`, `NO_RUMORS`, `WEAPON_MOD_SLOTS` | 0x400; `NO_RUMORS` 0x80000000 | `u8|` |
| 0x4D | `INFO_GENERAL_TOPIC` | 0x80000000 | `wstr`, five `u8|`, four refIDs `|` (`0083fb10`) |
| 0x50 | `TERMINALSTATE` | 0x80000000 | `u8|` flags, `u8|` lock level |
| 0x54, 0x56, 0x5C, 0x5D, 0x60 | `ACTIVATE_REF_CHILDREN`, `OBJECT_HEALTH`, `RADIUS`, `RADIATION`, `ACTOR_CAUSE` | 0x4000000; 0x80000000; 0x40000000; 0x40000000; 0x80000000 | a 4-byte value `|` |
| 0x5B | `MODEL_SWAP` | 0x80000000 | refID `|`, `u32|` |
| 0x5E | `FACTION_CHANGES` | 0x80000000 | vsval count, per entry refID `|` faction, `i8|` rank |
| 0x5F | `DISMEMBERED_LIMBS` | 0x20000 | `u16|`, two 4-byte values `|`, `u8|`, refID `|`, vsval count of entries: four `u8|` and a vsval-counted refID list |
| 0x6E | `AMMO` | 0x800 | refID `|`, `i32|` count |
| 0x70 | `PACKAGE_DATA` | 0x80000000 | `u8|` package type (0xFF none), then that [actor package data](#packages) |
| 0x73 | `SAY_ONCE_A_DAY_TOPIC_INFO` | 0x80000000 | vsval count, per entry refID `|`, two 4-byte values `|` |
| 0x75 | `SAY_TO_TOPIC_INFO` | 0x80000000 | two refIDs `|`, `u8|` |
| 0x7C | `GUARDED_REF_DATA` | 0x80000000 | vsval count of refIDs `|` |
| 0x8B | `FOLLOWER_SWIM_BREADCRUMBS` | 0x80000000 | 12 bytes `|`, refID `|`, `u32|`, vsval count of (12 bytes, refID, 12 bytes, refID, `u8`, each `|`) |
| 0x8F | `SECURITRON_FACE` | 0x80000000 | two `wstr` |
| 0x92 | `SPECIAL_RENDER_FLAGS` | 0x80000000 | two 4-byte values `|` |

Seen in the nine files: `SCRIPT`, `COUNT`, `HEALTH`, `LEVELITEM`, `LOCK`,
`MAPMARKER`, `ENCOUNTERZONE`, `GUARDED_REF_DATA`, `SPECIAL_RENDER_FLAGS`,
`LEVELEDCREATURE` (some 20,000 times) and `PACKAGE_DATA`; the rest from
code.

### Inventories

`InventoryChanges::SaveGame` (Xbox PDB), `004d4090`: a vsval count of
item changes; each (`ItemChange::SaveGame`, `004bed60`) is refID `|` of
the item, `i32|` how many more (or fewer) than the base record holds, and
a vsval count of extra data lists, one per stack that has extra data
(`COUNT`, `HEALTH`, `WORN`, `SCRIPT`, ...), each written with the change
flags set to 0x400.

### Actors

`MobileObject::SaveGame` (`00932880`), for actors and projectiles:

1. `u8|` the process level (`PROCESS_LEVEL`: 0 high, 1 middle high,
   2 middle low, 3 low; 0xFF no process).
2. The `REFR` data (form flags, scale, extra data, inventory; no
   animation for actors).
3. Eight `u8|`, two 4-byte values `|`, two `u8|`, two refIDs `|`.
4. The process's `SaveGame` (vtable +0x54C), each level calling the one
   below first:
   - `BaseProcess` (`008d0f30`): three 4-byte values `|`, then the
     running package (`ActorPackage`, `008c65d0`): refID `|` of the
     package; if not 0: for a package made in game `u8|` its type and,
     unless 0xFF, its [`SaveGame`](#packages); `u8|` the per-actor
     package data's type and, unless 0xFF, that data; three 4-byte
     values `|` and a refID `|`.
   - `LowProcess` (`00910450`): `u8|`, `u32|`, refID `|`, three 4-byte
     values `|`, `u8|`, `u16|`, a time and value (`009a5eb0`, two 4-byte
     values `|`), five refIDs `|`, a vsval-counted refID list, and with
     0x200000 `ACTOR_DAMAGE_MODIFIERS` a modifier list.
   - `MiddleLowProcess` (`0092ea30`): `u32|`; with 0x100000
     `ACTOR_TEMP_MODIFIERS` a modifier list.
   - `MiddleHighProcess` (`00926a20`): 37 fixed values `|`, four refIDs
     `|`, `u32|`, a vsval-counted refID list, a second running package
     (as above), with `REFR_ANIMATION` a vsval-sized animation block, the
     active effects (`00806a10`: vsval count; per effect refID `|` of
     the magic item, `u8|` effect index, vsval effect type and a
     vsval-sized block of the effect's own data, `00806840`), three
     refIDs `|`, a vsval count of (refID, two 4-byte values, six `u8`,
     each `|`) (`00913d60`).
   - `HighProcess` (`008fc4f0`): 64 fixed values `|`, seven refIDs `|`,
     six (refID, `u8`) `|`, three vsval-counted refID lists, `u8|` and if
     set a dialogue list (`0083ce40`: vsval count of items, each two
     `wstr`, two 4-byte values, `u8`, three refIDs; then `u16|` and four
     refIDs `|`), a vsval count of path avoid nodes with two values and
     two refIDs, two vsval-counted lists of queued items (`008d7070`),
     `u8|` and if set five values and a refID (`008d7270`), and a
     vsval-sized block (`00928880`).

`Actor::SaveGame` (`008aaf40`) then writes 31 fixed values `|` and three
refIDs `|`; 0x400 `ACTOR_LIFESTATE`: `u8|` (`ACTOR_LIFE_STATE`: 0
alive, 1 dying, 2 dead, 3 unconscious, ...); 0x80000
`ACTOR_DISPOSITION_MODIFIERS`: vsval count, per entry refID `|` and 4
bytes `|`; 0x800000 `ACTOR_PERMANENT_MODIFIERS` and 0x400000
`ACTOR_OVERRIDE_MODIFIERS`: a modifier list each (`ModifierList`,
`00937b50`: vsval count, per entry `u8|` actor value and `f32|`); then
the mover (`pActorMover`, +0x190 on PC, vtable +0x28):

- `ActorMover::SaveGame` (`009df100`): 19 fixed values `|`, the failed
  path destination (`PathingLocation::SaveGame`, `006def40`: 12 bytes
  `|`, three refIDs `|`, `u32|`, `u16|`, two `u8|`), refID `|` of the
  detection door, and `u8|` parts: 0x1 a pathing request (`u8|` its
  type, then `PathingRequest::SaveGame` `006e3230` — two pathing
  locations, 24 fixed values, a vsval count of avoid nodes (`006dc890`:
  `u8|` kind, two 4-byte values, 12 bytes, and 12 more for kind 1) — and
  the subclass's part: types 0 and 1 none, 2 and 7 two 4-byte values, 3
  one, 4 a pathing location and a value, 5 a list of points and three
  values, 6 two values and a refID, 8 nine values and two lists); 0x2 a
  pathing solution (`006e8b10`); 0x4 a virtual path handler (three
  values, `009eb390`) or, with 0x8, a detailed one (`009e8f80`). The
  player's `PlayerMover` (`009ea5a0`) adds 12 bytes and three values.

Member names come from the Xbox prototype; for `MobileObject`, `Actor`
and `PlayerCharacter` a PC offset is the Xbox one less 0x10.

### Packages

nv-rs re-evaluates AI on loading, so these are only read past. The per-
actor package data (`ActorPackageData` subclasses, type vtable +0x8,
saved by vtable +0xC) by package type: 2 escort (`009f0bb0`), 12 sandbox
(`009f5a60`), 13 patrol (`009f33d0`), 14 guard (`009f2850`), 16 use
weapon (nothing). A package made in game is saved by its class's
`SaveGame`, the class `TESPackage::CreatePackage` (`00670b90`) makes for
the type (`PTYPE` (Xbox PDB)): 18 `CombatController` (`009819f0`: the
package, two refIDs, a value, the combat state `009a17a0`, two combat
procedures and a list of them — `CombatProcedure` subclasses, type
vtable +0x34, saved by vtable +0x38 — and the planner `009948f0`), 21
`AlarmPackage` (`009ecec0`), 22 `FleePackage` (`009f2380`), 23
`TrespassPackage` (`009f9790`), 24 `SpectatorPackage` (`009f87d0`), 15
and 28 `DialoguePackage` (`009f01d0`), 39 `BackUpPackage` (`009edb60`),
others `TESPackage` (`006797c0`: 12 bytes, `u8|` parts — location
`0067fb60`, target `00680720`, data by type — and a value). The files
hold combat controllers (three, in the quicksave); the others are from
code.

### The player

`PlayerCharacter::SaveGame` (`009590f0`) writes, before
`Character::SaveGame`, three lists of 77 `f32|` actor values
(`TemporaryActorValueModifiers`, `ScriptActorValueModifiers`,
`DamageActorValueModifiers`) and `fHealthModifier`; after it, with
`REFR_ANIMATION` a vsval-sized block, then 55 fixed values, the five
`pCrimeCounts`, `u8|` and `u32|` (`008d56c0`), eleven refIDs `|`
(`pActiveQuest` first), and vsval-counted lists: `listTopics` (refIDs),
`listNotes` (refIDs), `RockItLauncherAmmoList` (item changes),
perceived actors (refID, two `u8`), **`Perks`** (refID `|`, `u8|`
rank), actions (two values, refID), casino data (two values, `u16`),
inactive and active caravan cards (refIDs), then five caravan totals,
`listQuestLog` (refID, two `u8`), `listObjectives` (refID, `u32`), the
active effects, three values, `CompanionPerks` (refID, `u8`), three
`u8|`, the eight hot keys as plain `u32|` form ids, and four refID lists
kept outside the player (`005aa930`).

### Projectiles

`Projectile::SaveGame` (`009c4ff0`): the mobile object (no process in
any file seen), nine 4-byte values `|`, three refIDs `|`, twelve more
values `|` (a 16-byte one from `00865ee0` among them), `u8|` and if set
an item change, a vsval count of impacts (two 12-byte values, two
4-byte, `u8`, two `u16`, refID, each `|`), `u8|`. `MissileProjectile`
(`009baaa0`) adds `u32|`, `FlameProjectile` (`009b3eb0`) two;
`GrenadeProjectile` and `BeamProjectile` (`00979bb0`) nothing.

### What the nine files hold

Per save (first named save as the example): 3,898 change forms, of which
2,430 `ACHR`, 657 `ACRE`, 390 `REFR`, 301 `QUST`, 82 `INFO`, 31 `CELL`, 3
`NPC_`, 2 `CHAL`, 1 `CLAS`, 1 `FACT`; 8,225 form ids; 172 globals; 43 misc
stats. Other saves add `PMIS`/`PGRE` (projectiles in flight). All 301 quest
change forms have `QUEST_SCRIPT`; 6 or 7 have stages and 3 have objectives.
Of the actors, most have a low process (level 3) or none; a few dozen per
file have middle or high processes, and the quicksave has three running
combat controllers.

## Scope: what nv-rs needs to continue a game

What the import has to fill in nv-rs (`GameState` in
`crates/world/src/scripting.rs`, saved by `crates/world/src/save.rs`), where
it is in a `.fos`, and how far this research got.

| nv-rs state | In the `.fos` | Status |
| --- | --- | --- |
| Game time (`GameHour`, `GameDaysPassed`, …) and other globals (`globals`) | Global data 3 | Decoded |
| Quest stages done, current stage (`stages`, `stages_done`) | `QUST` 0x80000000; current = highest done | Decoded |
| Running / completed / failed quests (`running`, `completed`, `failed`) | `QUST` 0x2 flags 0x01, 0x02, 0x40 | Decoded |
| Objectives (`objectives`) | `QUST` 0x20000000 (state 2 or 3 = completed) | Decoded |
| Quest script variables (`variables`) | `QUST` 0x40000000; the ids are the script's local variable indices (`SCRO`/`SLSD`), names come from the quest's script | Decoded (values); naming needs the script record |
| Quest delays (`quest_delays`) | `QUST` 0x4 | Decoded |
| Topics said once (`said`) | `INFO` 0x80000000 | Decoded |
| Misc statistics (`misc_stats`) | Global data 0 | Decoded |
| Player cell, worldspace, position (`player_cell`, `player_world`, `player_position`) | The player's `ACHR` (form id 0x14) initial data (kind 4 or 6), and global data 1 | Decoded |
| Player name, sex, level, S.P.E.C.I.A.L. (`player_name`, `player_female`, `player_level`) | `NPC_` 0x7 (the player's base): 0x20 full name, 0x1000000 gender, 0x2 base data (level), 0x4 attributes | Decoded |
| Tag skills (`tag_skills`) | `CLAS` `PlayerClass` 0x2 | Decoded |
| Faction crimes (`faction_crimes`), reactions (`faction_relations`) | `FACT` 0x80000000 (major, minor: nv-rs keeps (minor, major)), 0x4 | Decoded |
| Reputations (`reputations`) | `REPU` (fame, infamy) | Layout read from code, untested (no file has one) |
| Challenges (`more` challenge progress) | `CHAL` | Decoded |
| Local map fog (`seen`) | `CELL` 0x80000000 | Decoded (bits not mapped onto `local_map::Seen` yet) |
| Weather (`weather`) | Global data 8 | Decoded |
| References moved (`positions`, `spaces`), Havok-moved (`havok_moved`) | Initial data 4 / 6, Havok block | Decoded (initial data); Havok block located |
| References disabled (`disabled`), deleted | The form flags word of `CHANGE_FORM_FLAGS` | Decoded (the word); which bit means disabled or deleted at run time is not traced yet |
| Scale (`scales`) | `REFR` 0x10 | Decoded |
| Inventory and container contents (`items`, `stocked`), equipped (`equipped`), weapon condition (`weapon_health`) | `REFR`/`ACHR` 0x20 / 0x8000000 (`InventoryChanges::SaveGame`, `ItemChange::SaveGame`): count changes and stacks' `COUNT`, `HEALTH`, `WORN` | Decoded |
| Locks (`locks`, `broken_locks`), door teleports, ownership (`set_by_scripts`), open state | Extra data `LOCK`, `TELEPORT`, `OWNERSHIP`; the open state is the change flag 0x800000 itself | Decoded |
| Dead, health lost, actor values (`dead`, `damage`, `actor_values`, `value_damage`) | `ACHR`/`ACRE` `Actor::SaveGame`: life state, permanent and override modifiers, the processes' damage and temporary modifiers; the player's three actor value lists | Decoded |
| Perks and ranks, active quest, hot keys, notes (`perks`, `perk_ranks`, `active_quest`, `hotkeys`, `notes`) | `PlayerCharacter::SaveGame` `009590f0` | Decoded |
| Map markers found (`discovered`, `map_markers`) | Each marker reference's `MAPMARKER` extra data (flags 0x1 visible, 0x2 can travel) | Decoded |
| Radio (`radio`) | Global data 10 | Decoded |
| Skill points, experience, karma | Actor values: the player's base (`NPC_` data) and modifier lists | Decoded as values; which actor value each is comes from `world` |
| Active effects (`active_effects`) | Global data 7 and actors' process data (per effect its magic item, index and type; the effect's own data is a sized block) | Located (blocks not interpreted) |
| Packages, combat, AI process state | Process data in `ACHR`/`ACRE`, global data 4, 5 | Decoded (read past); global data 4 and 5 not decoded. nv-rs re-evaluates packages on load anyway |
| Created references (`more` made references) | Initial data 5 (base object, place); their own extra data | Initial data decoded |
| Form lists and leveled lists changed by scripts (`set_by_scripts`) | `FLST`, `LVLx` 0x80000000 | Decoded from code (no file has them) |

What nv-rs is missing to take a `.fos` at all:

- **Form id remapping.** The save's plugin list must be matched against
  nv-rs's load order (by name), and the 0xFF created forms need a home:
  nv-rs's created references would have to adopt the save's ids or keep a
  map.
- **Script variable names.** nv-rs keeps quest and reference variables by
  name (`Locals`); the save keeps them by index, so the import needs the
  owning script's `SLSD`/`SCVR` list.
- **State nv-rs doesn't model.** Interface state, combat manager, actor
  process levels and AI procedures, temporary effects, loaded cell grid:
  the import would drop these and let nv-rs rebuild them, as its own loader
  does.
- **Unknown flags per bit.** Form flags bits for disabled/deleted, `CELL`
  flags2, `ACBS` and AI data byte meanings must be checked against the code
  that reads them before they are applied.

## Import

`world::fos_import::import(order, bytes)` builds a `GameState` and the
player's place from a save: a new game's state (`GameState::new`:
start-game quests running, the plugins' globals, the player's record
items) with what the save changed laid over it. `nvinspect fos-import
<SAVE> <PLUGIN|DATA>` prints what it took (counts, the player's place, the
game time, the quests' stages). Every mapping is tested on synthetic
saves (`crates/world/src/fos_import/tests.rs`).

**Form ids.** The save's plugin list is matched to the load order by
name, ignoring case (as `00847660` and `00846d80` do); a form whose plugin
isn't loaded is dropped and counted. Forms made in game (`0xFF......`)
keep their ids. Change forms are applied in file order, references last
(the player's added values go onto the base's).

| nv-rs state | From | Notes |
| --- | --- | --- |
| `globals` | Global data 3 | Game time included. |
| `misc_stats` | Global data 0 | By `world::stats::NAMES` index. |
| `weather` | Global data 8 | `current`, `previous` (`pLastWeather`), `picked` (`pDefaultWeather`), `forced` (`pOverrideWeather`), `started` (`fLastWeatherUpdate`), `fade` (`fCurrentWeatherPct`), `sped_up` (`fAccelBeginPct` while fading). The climate and the player's region are worked out again. |
| `radio` | Global data 10 | On, the station tuned to, the one lost, the stations found. |
| `running`, `completed`, `failed` | `QUST` 0x2 flags 0x01, 0x02, 0x40 | Quests without a change form keep the new game's state. |
| `stages_done`, `stages` | `QUST` 0x80000000 | Current = highest done (`0060d670`). |
| `objectives`, `set_by_scripts.hidden_completed` | `QUST` 0x20000000 | State 1 shown, 3 completed and shown, 2 completed and hidden. |
| `quest_delays` | `QUST` 0x4 | |
| `variables` | `QUST` 0x40000000, references' `SCRIPT` extra data | Names from the script's `SCVR` by `SLSD` index; whole numbers by `SLSD` flag 0x01; reference variables as the form id. |
| `said` | `INFO` 0x80000000 | |
| `player_name`, `player_female`, S.P.E.C.I.A.L. (`actor_values` 5..11), `player_level`, karma (23) | `NPC_` 0x7 | The header's level when the base data isn't saved. |
| `tag_skills`, `tag_slots` | The `CLAS` with `CLASS_TAG_SKILLS` | |
| `faction_crimes`, `faction_relations` | `FACT` 0x80000000, 0x4 | (major, minor) turned into nv-rs's (minor, major); the group reaction (0 neutral, 1 enemy, 2 ally, 3 friend). |
| `more.challenges` | `CHAL` | Progress and flags. |
| `reputations` | `REPU` | Fame, infamy. |
| `seen` | `CELL` 0x80000000 | 256 points per exterior cell (by the record's grid) or interior section, as `world::local_map` keeps them (translated from the game's own `SeenData` code); fully seen when 248 or more are. |
| `set_by_scripts.cell_owners` | `CELL` 0x8 | |
| `set_by_scripts.list_additions` | `FLST` 0x80000000 | |
| `disabled` | The form flags (`CHANGE_FORM_FLAGS`): 0x800 disabled (`00440da0`), 0x20 deleted (picked up) | Only where they differ from the record, so references that follow an enable parent keep following it. |
| `positions`, `spaces` | Initial data 4 and 6 | Position and z rotation; with a cell change the worldspace and the cell at the position's grid square. |
| `scales` | `REFR` 0x10 | |
| `more.placed` | Initial data 5 | References made in game: base (a leveled actor's `pTemplate`), place, `COUNT`; not those whose base is itself made in game. |
| `locks`, `broken_locks` | `LOCK` extra data | Locked (`cFlags` 0x1) at `cBaseLevel`; `uiNumTries`. |
| `discovered`, `map_markers` | `MAPMARKER` extra data | Flags 0x3 where the record hasn't them: found; 0x1 alone: shown. |
| `set_by_scripts.owners` | `OWNERSHIP` with `CHANGE_REFR_EXTRA_OWNERSHIP` | |
| `more.ghosts`, `faction_changes` | `GHOST`, `FACTION_CHANGES` extra data | |
| `inventory_stacks` | Inventories, through `fos::inventory_source` | Each holder's item changes in saved order: saved id and its resolution, the nv-rs form, the signed count delta, and per stack `COUNT`, raw `HEALTH` bits and `WORN`; other extras kept as their original bytes. Kept by nv-rs's own saves; gameplay doesn't change them ([ENGINE_STATE.md](ENGINE_STATE.md), "World inventory stacks"). |
| `items`, `stocked`, `equipped`, `weapon_health` | Derived from `inventory_stacks` | The record's items plus the saved changes; the record's leveled lists rolled by nv-rs unless `CHANGE_REFR_LEVELED_INVENTORY` says the game already did (its picks are among the changes); `WORN` stacks equipped; a weapon's `HEALTH` over its `DATA` health (the last stack with health, when there are several). |
| `dead`, `unconscious`, `set_by_scripts.restrained` | `ACTOR_LIFESTATE` | Dying (1) and dead (2), 3, 5. |
| `teammates`, `set_by_scripts.ignoring_crime`, `more.forced_sneak`, `more.critical_stage` | Actor fields `bPlayerTeammate`, `bIgnoreCrime`, `bForceSneak`, `eCriticalStage` | |
| `more.dispositions` | `ACTOR_DISPOSITION_MODIFIERS` toward the player | |
| `actor_values` | `ACTOR_OVERRIDE_MODIFIERS` (set), `ACTOR_PERMANENT_MODIFIERS` (added onto it or the record's value) | Not the player's skills (nv-rs works them out). |
| `damage`, `value_damage` | The low process's `ACTOR_DAMAGE_MODIFIERS`; the player's `DamageActorValueModifiers` | Health lost, other values damaged; not radiation and the hardcore needs. |
| Experience (24), S.P.E.C.I.A.L., karma | The player's `ScriptActorValueModifiers` | Added onto the base. |
| `perks`, `perk_ranks` | The player's `Perks` | `world::perks::add` once per rank (abilities included); the rank is a count from 1 (`009639e0`). |
| `active_quest`, `notes`, `topics`, `hotkeys` | The player's data | |
| `player_crimes`, `steal_warnings`, `more.in_chargen`, `set_by_scripts.fast_travel` | The player's `iMinorCrimes`/`iMajorCrimes`, +0x228, +0x75c, +0x66d/+0x66e | |
| `player_cell`, `player_world`, `player_position` and the place | The player's initial data (else global data 1) | An interior cell, or the worldspace and the cell at the grid square. |

**In the viewer.** `nv-viewer <DATA> --load-fos <SAVE>` starts from a
save instead of a new game (the CELL can be left out): it prints what it
imported, the game time and the quests' stages, and opens the player's
place as F9 opens nv-rs's own saves (an interior through
`load_cell_now` with the imported disabled references, an exterior at the
saved feet and heading). With `--screenshot` it makes a picture and quits,
as usual.

Checked live on 2026-10-07 with two of the nine saves: the autosave's
backup (in `GSProspectorSaloonInterior` at 133, -822, 3459, facing north)
opens in the saloon at the door, facing north, with Doc Mitchell's quest
(`VCG01`) at stage 200 completed and `VCG02` at stage 5; a named save in
Goodsprings (outside, at -67912, 3054, 8358, heading 225 degrees) opens on
the road there facing south-west at 11:25, the weapon the player held
drawn with its ammunition count. No comparison with the original game
running the same save was made yet.

Not imported (`world::fos_import::GAPS`): active effects (each effect's
data is a block not interpreted), AI processes, packages, combat and
pathing (nv-rs re-evaluates AI on loading), which actor a leveled list
picked for a placed actor (nv-rs picks again), forms made in game other
than references, projectiles in flight, the Havok poses of moved clutter
(their place is imported), doors' open state, the player's skills and
other script-changed values beyond S.P.E.C.I.A.L., experience and karma,
companion perks, radiation and the hardcore needs, caravan cards and
casino winnings, terminals, weapon mods, item condition outside weapons,
placed items' counts, the player's disabled controls, and global data 4 to
7 and 9.

## Next steps

1. Check the import against the original game: load the same save in
   both and compare (quest stages, inventories, where things stand).
2. Global data 4 to 7 and 9 (process lists, combat, interface, effects,
   actor causes) if anything in nv-rs comes to need them.
3. A save that has the parts read from code only (`NPC_FACE`, created
   packages other than combat, form and leveled list additions,
   reputations) to confirm them.

## Reader and inspector

`crates/fos` (no dependencies) reads a save read-only: `fos::Save::parse`
splits it into the parts above and fails unless every part ends where the
location table says and the history block ends the file, so a parsed save
accounts for every byte. `fos::decode` reads quests (stages, log dates,
script locals, objectives), globals, misc statistics, the location data,
initial data and the Havok block of references, cells (both seen data
layouts), topics, notes, actor bases, factions, classes, challenges and
reputations, references' extra data and inventories, actors with their
processes and movers, the player, projectiles, the small base form types,
the weather and the radio; `decode::coverage_of` says whether a change
form decodes to exactly its length, is not decoded (`MSTT`, `PCBE`), or
fails. Tests build
synthetic saves with the test-only writer `fos::write` (feature `write`);
no real save is in the repository.

`nvinspect fos <SAVE> [PLUGIN]` prints the header, plugins, the location
table, the global data, the change forms counted by type and by change
flag (with the Xbox names) and how many decode exactly, the quests with
stages or objectives (current stage, stages done, objectives), the
globals, the misc statistics, the player's place, active quest, perks
and inventory, the weather and the radio, and the file's bytes part by
part. With a plugin (`FalloutNV.esm`) forms from it get their
editor IDs. It exits with an error if any decodable change form doesn't
decode to its length.

Run on all nine real saves (2026-10-07): every one parses with every byte
accounted for, and every change form decodes to exactly its length (3,585
to 4,509 per file; none not decoded, none failed).

Read-only baseline on 2026-10-10: the user's current save directory holds
11 `.fos` files, a different set from the nine above. Seven pass the
inspector; four files with 123-124 plugins report 13 change-form failures
in total (reference trailing bytes, actor separator errors and unknown
types). All 11 outer file layouts account for their bytes, but that does
not establish change-form support. The four saves with only the base
game and nine official add-ons pass, including an early Doc Mitchell
autosave. These results do not prove import or round-trip fidelity.

Private per-file SHA256 values and inspector output are in
`%USERPROFILE%\nv-re\work\phase0\codex-audit\save-baselines.json` and the
linked logs. Input hashes were identical before and after inspection;
no save bytes are committed. Phase 2 should first validate the ExtraData
adapter against the supported early save. The heavily modded failures
remain recorded gaps, not a reason to expand the active milestone now.
