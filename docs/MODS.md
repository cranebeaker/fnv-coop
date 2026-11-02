# Mods and plugins (M8, milestone M5)

How New Vegas picks and orders plugins and archives, when a loose file
beats an archived one, and which script extender functions mods call;
what nv-rs does for each. Everything below was traced in FalloutNV.exe
1.4.0.525 (PC, decompiled; addresses are the exe's) and, where named,
checked against the Xbox 360 prototype's symbols and decompile
(`Xbox PDB`). Settings are from the exe's settings table
(`%USERPROFILE%\nv-re\logs\settings.tsv`). Nothing here was compared with
the running original game yet.

Code: `esm::load_order` (plugins), `assets` (archives and loose files),
`script::nvse` and `nvinspect … coverage nvse` (script extender),
`testdata::mods` and its `mod-cases` program (the test setups).

## Which plugins load, and in what order

Traced from `Main::InitTES` (`0086cf20`), `Main::LoadPluginsFromFile`
(`00872430`), `TESDataHandler::BuildFileList` (`004624b0`) and the start
of the data handler's file loading (`00463070`).

1. **The file list.** Every non-empty `Data\*.esm`, then every non-empty
   `Data\*.esp`, each in `FindFirstFile`'s order (by name on NTFS), is
   read (its header only) and inserted into a list: files with the
   master flag (the `TES4` record's flag 1; the extension doesn't count)
   before the others; within each group by modification time, oldest
   first; a file with the same time as one already in the list goes
   before it, so equal times end up in reverse name order. FalloutNV.esm
   gets no special place.
2. **Masters first, for master files.** Walking the list, a
   master-flagged file's masters that come after it are moved to just
   before it (and the moved file is checked in turn). Plugins without the
   flag are not reordered: an `.esp` can load before an `.esp` it depends
   on.
3. **Active files.** `[General] sTestFile1` to `sTestFile10` (only
   `sTestFile1=FalloutNV.esm` is set by default), every line of
   `plugins.txt` (read from the folder the game keeps it in,
   `%LOCALAPPDATA%\FalloutNV\`), and every plugin with a `<name>.nam` file
   beside it in `Data` (`"%s%s.NAM"`; this is how the DLC switch
   themselves on; `IsDLCPackageName` `0046feb0` also flags the four story
   DLC). `plugins.txt` lines: those starting with `#` and those of one
   character or none (counting the line feed) are skipped, the line feed is
   cut off and nothing else is trimmed, and names compare without case.
   **The order of `plugins.txt` doesn't matter**: only file dates do.
4. **Masters made active.** In list order, each active file's masters are
   made active (`TESFile` master table, `00471870`, finds them by name
   anywhere in the list). A master missing from `Data` stops the loading
   ("Unable to find masterfile: %s"). A master with a format version over
   1.34 isn't made active ("File %s is a higher version than this EXE can
   load.", `004738a0`).
5. **The load order** is the active files in list order, at most 255
   ("Too many selected files to compile!"); each gets its index (the top
   byte of its form IDs).
6. **Master sizes.** For plugins without the master flag, each master's
   size is compared with the size stored after its `MAST` (`DATA`); a
   difference gives the warning "One of the files that "%s" is dependent
   on has changed since the last save." (`00471af0`,
   `sMasterMismatchWarning`, then `sGeneralMasterMismatchWarning`).

### Form IDs and overrides

- A form ID's top byte indexes the plugin's own master list; an index at
  or past its end means the plugin itself (`TESFile::GetIndexFile`
  `00471a10` finds nothing and `TESFile::ReadFormHeader` `00472bc0` uses
  the file's own index). Masters are found by name, so the order of a
  plugin's master list doesn't have to match the load order.
- IDs 1 to 0x7FF are the engine's own forms (`TESForm::IsDefaultForm`
  `00484b40`). A reference to one in a plugin's data keeps its ID as
  written (`00485d50`), and a record whose object ID is 1 to 0x7FF is the
  base game's form whatever its top byte (`00472bc0`).
- When several plugins have a record with the same ID, the one loaded
  last wins, record by record.

## Archives

Traced from `ArchiveManager` (names from the Xbox PDB): the setup
`00876d20`, `Init` (`00af43a0`), `SetInvalidation` (`00af4490`),
`OpenMasterArchives` (`00af4550`), `OpenArchive` (`00af4be0`),
`GetArchiveForFile` (`00af6160`), and `00463070`.

Settings (`[Archive]`): `bUseArchives` (default 1; off, no archive is
opened), `SArchiveList` (the exe's own default is
`Fallout - Textures.bsa, Fallout - Meshes.bsa, Oblivion - Voices1.bsa`;
the shipped `Fallout_default.ini` lists the six `Fallout - *.bsa`),
`bInvalidateOlderFiles` (default 1), `SInvalidationFile` (default
`ArchiveInvalidation.txt`).

**Which archives open, in this order:**

1. Each `SArchiveList` name (comma-separated, leading blanks skipped).
   `OpenMasterArchives` also appends `<plugin>.bsa` for the lines of a
   `Plugins.txt` in the game's working folder (its path prefix is an empty
   string), which on PC normally doesn't exist; nv-rs doesn't do this.
2. `Update.bsa`, if it exists (`BuildFileList`).
3. For each loaded plugin in load order, every non-empty
   `Data\<plugin name without extension>*.bsa` not already open. The
   pattern is a prefix: `ModA.esp` opens `ModA.bsa`, `ModA - Main.bsa`
   and also `ModAExtra.bsa`.

An archive that can't be opened is left out.

**Which archive wins.** The archive manager keeps one list and every
lookup walks it from the front; the first archive holding the file wins.
`OpenArchive` places each new archive by its name (case-sensitive
substrings): a name containing `Fallo` goes to the end of the list; any
other is ranked `DeadM` 1, `Hones` 2, `Lones` 3, `OldWo` 4, `Updat` 5,
anything else 6 ("Loading a user created or misnamed BSA") and inserted
before the first listed archive whose name contains `Fallo` or `DeadM`,
or `Hones` (rank over 1), `Lones` (over 2), `OldWo` or `Updat` (over 3);
at the front if there is none. The Xbox prototype's `OpenArchive` has the
same scheme without `Updat`. Results:

- the base game's `Fallout - *.bsa`: the **first** listed wins (in the
  vanilla archives no path is in two of them; only `Update.bsa` shares
  39 paths with Meshes, Textures2 and Sound);
- `Update.bsa` wins over every `Fallout - *.bsa`;
- DLC archives win over the base game's; Old World Blues' even over
  `Update.bsa`;
- mods' archives win over all the official ones, but among mods' archives
  **the first loaded wins** (each is inserted after the mod archives
  already listed). This is the opposite of plugins, where the last
  loaded wins.

The archive header's content flags (meshes 1, textures 2, menus 4, sounds
8, voices 16, shaders 32, trees 64, fonts 128, misc 256) are compared with
a type from the file's extension (`GetArchiveTypeFromFileExtension`,
`00af7db0`, a 26-entry table at `011ace08`; unknown extensions are misc)
when a caller doesn't name a type. nv-rs doesn't apply this filter: the
vanilla `Fallout - Textures2.bsa` is flagged textures only yet holds the
fonts, and `Fallout - Misc.bsa` is flagged misc yet holds XML files, so
the callers that read those must pass their own type; which caller passes
what wasn't traced.

## Loose files and archive invalidation

Traced from the file finder (`00afe220`), `Archive::CheckInvalidateFile`
(`00afb190`, Xbox `82935f18`), `ArchiveManager::LoadInvalidationFile`
(`00af5ab0`), the archive's invalidation at open (`00afad00`) and
`Archive::InvalidateOlderFilesByPath` (`00b01590`).

- **Archives are asked first.** The loose file (`Data\<path>`) is used
  only if no archive has the path.
- **`bInvalidateOlderFiles` on (the default):** the first time a path is
  looked up in an archive, the archived entry is dropped if
  `Data\<path>` exists loose. So **a loose file beats every archive,
  whatever the dates** (despite the setting's name).
- **The invalidation file** (`SInvalidationFile`, found like any file: the
  archives, the game's folder, then `Data`) is read once before the
  archives open, and only used with `bInvalidateOlderFiles` on. Lines end
  at a carriage return (the character after it is skipped, so a file with
  bare line feeds reads as one line). A line without a backslash names a
  file (its name only, after `_splitpath`); a line with one names the
  folder it is in (a leading backslash dropped). When an archive with
  folder and file names opens, every file in a listed folder is dropped,
  and files with a listed name are dropped from folders that exist loose
  under `Data`. A dropped file with no loose copy can't be found at all.
- Archives without folder or file names instead drop every entry for
  which a loose file under `Data` is newer than the archive. nv-rs reads
  only named archives, so this branch isn't implemented.
- **`bInvalidateOlderFiles` off:** nothing is dropped; the archive wins.

## What nv-rs does now

| Rule | nv-rs | Test |
| --- | --- | --- |
| File list order (master flag, dates, ties, masters moved) | `esm::load_order::game_load_order` | `esm/tests/mods.rs` |
| Active files: `plugins.txt` (game's parsing), `.nam`, masters made active, missing master stops | done; `sTestFile2..10` not read | `esm/tests/mods.rs`, `load_order.rs` |
| Masters found by name anywhere; one loading after its dependent warns | done | `load_order.rs` |
| Master size warning | `LoadOrder::warnings` | `esm/tests/mods.rs` |
| Form ID renumbering, past-the-end = itself, engine forms 1–0x7FF | `LoadedPlugin::to_global` / `record_id` | `esm/tests/mods.rs` |
| Overrides, last loaded wins, moved references | done (was already) | `esm/tests/mods.rs` |
| Archive open order (list, Update.bsa, `<plugin>*.bsa`, non-empty, unreadable left out) | `assets::Assets::open_with_settings` | `assets/tests/mods.rs` |
| Archive priority | `assets::archive_priority` | `assets/tests/mods.rs` |
| `bUseArchives`, `bInvalidateOlderFiles`, `SInvalidationFile` | `assets::ArchiveSettings::from_ini` | `assets/tests/mods.rs` |
| Loose against archived | `Assets::locate` | `assets/tests/mods.rs` |
| Invalidation file | `assets::Invalidation` | `assets/tests/mods.rs` |
| Content-type filter | not applied (see above) | — |
| Unnamed archives' date check | not implemented | — |
| Format version over 1.34 | not checked | — |

Effect on the base game: none expected. With only FalloutNV.esm the load
order is the same; the only archive paths shared in the vanilla archives
are `Update.bsa`'s, which won before and still wins; loose files still
beat archives because `bInvalidateOlderFiles` is on in the shipped INI
and by default.

Before this work, nv-rs put FalloutNV.esm first regardless of its date,
required every master to load before the plugins using it, trimmed
`plugins.txt` lines and accepted a leading `*`, ignored `.nam` files and
inactive masters, let a later archive win over an earlier one, loaded
only `<plugin>.bsa` and `<plugin> - *.bsa`, failed on an unreadable
archive, and always let loose files win.

## Script extender functions

`script::nvse` holds xNVSE's functions by opcode: the names in the order
xNVSE registers them (`CommandTable::AddCommandsV1` to `V6`, from opcode
0x1400; xNVSE commit `0ccd23ad885d`, 2026-09-27; 654 functions), checked
against the 20 opcodes xNVSE's compiler hard-codes. Opcodes from 0x2000
belong to NVSE plugins (JIP LN, JohnnyGuitar…), each at a base it
registers at run time, so only their numbers are known.

`nvinspect <plugin or Data folder> coverage nvse` lists every xNVSE and
plugin function the compiled scripts (`SCDA`) and conditions (`CTDA`)
call, how (as a statement, inside an expression stored the game's way,
or inside one stored xNVSE's way, found by scanning for xNVSE's call
token), which scripts, and whether nv-rs carries it out; then every xNVSE
function. It works on a single plugin, so a mod can be checked on its
own. For the base game it finds none (11,215 compiled scripts in
FalloutNV.esm). nv-rs runs none of these functions yet; a script calling
one doesn't parse from its source.

## Test setups

`testdata::mods` builds the cases the tests use, each a folder with a
`Data` folder, `plugins.txt`, INI files and a `CASE.txt` saying what the
game does with it:

```text
cargo run -p testdata --bin mod-cases -- <folder> [case ...]
```

- `load-order`: master flag, dates, equal dates, `.nam`, a master made
  active, an empty plugin, a missing listed plugin, a master size
  mismatch.
- `overrides`: four plugins with different master lists, renamed and
  new records, a moved reference and a new one in an overridden cell.
- `archives`: the list, `Update.bsa`, a DLC archive, mods' archives
  (including a prefix match), unlisted, empty and inactive archives.
- `loose-files`: loose against archived with `bInvalidateOlderFiles` on
  and off, and an `ArchiveInvalidation.txt` with a folder line and a
  name line.
- `nvse-scripts`: a plugin whose script calls xNVSE functions and an
  NVSE plugin's opcode.
- `doc-house-override`: `DocHouseOverride.esp` for the real game's data,
  changing the model of Doc Mitchell's lit wall lamps (STAT `000F1E85`)
  to the cave chandelier; it names form IDs and a model path only.

Checked in the viewer (2026-10-07): a Data folder of hard links to the
game's files plus `DocHouseOverride.esp`, opened with
`--plugins <case>\plugins.txt` on GSDocMitchellHouse, shows the hall's
two wall lamps as chandeliers; the same view of the unmodified Data
folder is unchanged.

The cases use a small stand-in FalloutNV.esm, so they show the rules,
not the game's content; to try a load order with the real game data,
make a separate Data folder (the game's files linked, not copied) and
point the viewer or nvinspect at it with `--plugins`.

## Not done

- Comparing these rules with the original game running (a load order
  with test plugins and archives built to tell the cases apart).
- `sTestFile2` to `sTestFile10`, the format version check, the
  content-type filter and the unnamed-archive date check (above).
- Running script extender functions, and NVSE plugins' function names.
