# release: installer, launcher, release zip, player guide (roadmap step 10 without split-screen)

Round: full-sync code round (2026-10-21). Owner: release. **All of this is unit-tested against a fake game folder and a fake
release. Nothing here has been run against a game session, and no zip has been given to another PC.** Split-screen is claimed by
a contributor and is not touched (`plugin/splitscreen`, `tools/splitscreen`).

Plain Python 3 (3.8 or newer, standard library only, tkinter for the window). Windows is the target; the logic runs anywhere.

## Where things are

| Path | What |
|---|---|
| `tools/installer/fnv_install.py` | CLI: `find`, `check`, `install`, `status`, `uninstall`, `restore-inis` |
| `tools/installer/fnv_find.py` | finds installs (Steam registry + library folders + appmanifest, GOG registry, retail registry, common folders on every drive); reads game and xNVSE versions; `inspect_game` |
| `tools/installer/fnv_pe.py` | reads the file version and machine type of a PE file without running it (checked on the real FalloutNV.exe: 1.4.0.525) |
| `tools/installer/fnv_manifest.py` | release manifest: schema, safe destinations, payload hashes, `SHA256SUMS.txt` format |
| `tools/installer/fnv_ini.py` | careful INI editing (ANSI code page, atomic writes, rejects line breaks in values) |
| `tools/installer/package_release.py` | builds and verifies the release zip |
| `tools/launcher/launcher_gui.py` | the window (tkinter); `fnv_launcher_core.py` holds all logic; `fnv_portcheck.py` the port checks |
| `docs/PLAYER-GUIDE.md` | player guide in plain language (shipped inside the zip) |
| `tools/installer/run_tests.py`, `tools/launcher/run_tests.py` | the tests; each prints `ALL OK`; both are in `tools/test_all.py` ("Release: ...") |

## Installer behaviour

`install` stops at the first problem, before it has changed anything:

1. manifest and payload: every destination must be a file directly inside `Data/NVSE/Plugins` (a manifest with `..`, a drive, a
   folder or another place is refused as a whole), every payload file must match its SHA-256 and size;
2. game: `FalloutNV.exe` is 1.4.0.525 (read from the PE version resource), `Data\FalloutNV.esm` exists, xNVSE (`nvse_loader.exe`,
   `nvse_1_4.dll`) is installed and 6.3.0 or newer (the plugin refuses older: `plugin/main.cpp`). A wrong game or xNVSE version can be
   overridden with `--force`; a missing game, Data folder or xNVSE cannot. The tool links to xNVSE's official page and downloads nothing;
3. the game is not running (`tasklist`), no target is a folder or link, the plugin folder does not resolve outside the game folder.

Then: backup to `<game>\FNVCoOp\backups\<utc time>\` (every file it is about to replace that was not ours, plus `Fallout.ini` and
`FalloutPrefs.ini` from the game folder and from Documents\My Games\FalloutNV; the INIs are copied only, never edited); an
`install.json` record is written with state `installing`; every file is copied next to its name, hashed, renamed; the state becomes
`installed`. Any failure rolls everything back (replaced files, new files, created folders, record, backup). An existing `nvcoop.ini`
is kept (the manifest marks it `keep_existing`); only a PC without one gets the default (co-op off).

`uninstall` removes the files whose hash still matches the record, copies a file the player changed to `FNVCoOp\removed\...` before
removing it, restores originals from the backup (hash-checked; `nvcoop.ini` goes back to the pre-install content), removes folders it
created, and removes `FNVCoOp\`. It never touches other plugins or saves. `nvcoop.log` and the other runtime files stay unless
`--remove-logs`. An upgrade keeps the ORIGINAL pre-install files as the restore target. `restore-inis` puts the backed-up game INIs
back (the current ones are first saved as `.before-restore`).

An explicit `--game` folder is the only folder used; it never falls back to another install that happens to be found.

## Release zip

```
python -I tools/installer/package_release.py build --version 1.0.0 --license <licence file> [--dll out/L1/nvcoop.dll] [--data-dir DIR ...]
python -I tools/installer/package_release.py verify out/release/FNV-CoOp-1.0.0.zip
```

Payload (all next to `nvcoop.dll` in `Data/NVSE/Plugins`): `nvcoop.dll` (default `out/L1/nvcoop.dll`), `nvcoop_rules.json`
(`docs/quest-policies/rules.json`), `nvcoop_edids.json`, `nvcoop_infoquest.json`, `nvcoop_pack_watch.json` (guest pack facts: the item forms a guest reports), `nvcoop_guest_local_run.json` (searched in
`--data-dir`, `out/L10/plugin`, `out/L1`, `out/L10/data`), `nvcoop_consent_messages.txt` (`plugin/dialogue/data/consent_messages.txt`),
`nvcoop_volatile_keys.txt` (`plugin/snapshot/data/volatile_keys.txt`; all required, because the join check compares the rules and the volatile
list), and a default `nvcoop.ini` with co-op off. The zip also holds `installer\`, `launcher\`, `Install.cmd`, `Uninstall.cmd`,
`Check.cmd`, `Launcher.cmd`, `README-FIRST.txt`, `PLAYER-GUIDE.md`, `LICENSE.txt`, `THIRD-PARTY.txt` (ENet), `manifest.json` (every
payload file with size and SHA-256, supported game version, minimum xNVSE, protocol number) and `SHA256SUMS.txt` (every file).

Checks that stop the build: a required file is missing; `nvcoop.dll` is not a 32-bit DLL or lacks `NVSEPlugin_Query`/`NVSEPlugin_Load`; any
shipped file holds a personal Windows path (`<drive>:\Users\<name>\...`, ASCII or UTF-16, generic names like Public excepted) or a `--forbid`
text (also env `FNVCOOP_FORBID`, `;` separated). The zip is reproducible (sorted names, fixed times, `SOURCE_DATE_EPOCH` or the newest input time) and is
re-opened and verified after writing. The `.cmd` files find Python (`py -3`, then `python`) and print the python.org link if it is missing.

**Before publishing the zip:** check the account and URL it goes to and grep it for the owner's name (the packager's privacy scan is not a
substitute). Publish only under the co-op identities.

## Findings during this work

* **The built `nvcoop.dll` embeds an absolute source path.** `plugin/snapshot/CMakeLists.txt:60` defines `FNVCOOP_DEFAULT_VOLATILE_KEYS` as
  `${CMAKE_CURRENT_LIST_DIR}/data/volatile_keys.txt`, so `out/L1/nvcoop.dll` contains the build machine's path with the user name (one string).
  The packager blanks any C string that is only such a path (the code then sees "no file", which is what a player's PC would see anyway) and
  refuses a path inside a longer string; the manifest hashes the blanked DLL. The proper fix is in the build: do not set that define for a release
  build. That file belongs to another lane, so it was not edited.

## nvcoop.ini written by the launcher

Only these keys are written; all other lines stay. The previous file is kept as `nvcoop.ini.prev`.

| Section | Keys | Read by |
|---|---|---|
| `[coop]` | `enable=1`, `role=host` or `role=client`, `listen=0.0.0.0` (host), `host=<addr>` (join), `port`, `maxplayers` (host), `token` (join, stable per PC), `handshake=1`, `baseline=1`, `dialogue=1` | `plugin/coop` (`ParseConfig`) |
| `[coop]` | `name=<player name>` | nothing yet (the config parser ignores unknown keys); a player name display needs a Hello field or an avatar-channel field |
| `[combat]` | `xpmode=1` or `0` | `plugin/combat/combat_module.cpp` (1 = party share, 0 = killing blow) |
| `[party]` | `slowmo=0` or `1`, `tether=1` or `0` | **nothing yet.** These are the agreed names. The reader for `slowmo` belongs with the real-time VATS work (host time scale while any player is in VATS, capped); `tether` is a host switch for the quest rules' `PartyTether` gate |
| `[live]` | `actors`, `avatars`, `combat`, `spawns` | `plugin/coop/live/live_switches.*`; the launcher applies the same dependency rule (combat and spawns switch actors on) |

`SESSION_DEFAULTS` in `fnv_launcher_core.py` is the single place for `handshake`, `baseline`, `dialogue`; change it there when the lead decides
the defaults of a release.

## Launcher behaviour

Host: validates name and port, checks the UDP port is free (`bind_check`), writes the ini, starts `nvse_loader.exe` in the game folder. Join:
accepts `ip`, `ip:port`, `name`, `name:port` (IPv4 or host name; IPv6, URLs and spaces are refused with a plain message), writes the ini, starts the
loader. A refusal (bad input, plugin not installed, game version wrong, game already running, port busy) changes nothing. The port tools: free-port
check with the local network addresses and a router/firewall hint, a test listener that answers one fixed UDP message, and a "Test connection" that
sends it. No outside service is contacted (no public-IP lookup).

## Tests and twins

* `python -I tools/installer/run_tests.py`: 34 checks, 33 twins (PE reader, discovery with a fake registry, manifest safety, install, upgrade,
  rollback on a failed copy, corrupt copy, uninstall leaves no trace, user-changed file kept, original files restored, CLI, packaging, determinism,
  privacy scan, path blanking, zip tampering).
* `python -I tools/launcher/run_tests.py`: 10 checks, 9 twins (address and name input, plan and ini writing, settings file, port checks, UDP test
  round trip on 127.0.0.1, start/refuse paths with a fake process starter, the window built hidden).
* Twins use `fnv_mut.use(name)`, an in-process switch that nothing in a shipped tool reads from the environment.
* Both runners fail if a real game folder on the PC changes while they run (names, sizes, times of its plugin folder, and `FNVCoOp`).

## Open items (honest)

* Nothing was run in the game or on a second PC: the first real check is Install.cmd, Launcher.cmd and a Host/Join pair on two machines.
* `[party] slowmo` and `[party] tether` and `[coop] name` have no reader in the plugin yet (see table).
* The release needs a licence file passed with `--license` (the working tree has none; the public repo carries one) and the build-path fix above.
* GOG and Steam registry keys and the GOG game ids are the commonly documented ones and the folder scan covers the rest; only the Steam
  path was seen on this PC.
* A single-file `.exe` (no Python needed) is not built; the zip needs Python 3 on every PC.
