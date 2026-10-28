# Testing under Proton (Steam Deck, Linux)

How to run the game copies, the bridge tools (nvbridge, the proofs, the playbooks, the benchmark) and a two-copy co-op
session on Linux through Steam's Proton, with no Windows PC. Tried on a Steam Deck (SteamOS, Game Mode and Desktop Mode)
with GE-Proton9-27. A result from here is "proven under Proton on Steam Deck", not "proven on Windows".

What it does not cover: building `nvcoop.dll` (that needs MSVC, see `tools/runner/build_l1.cmd` or CI's plugin job) and the
game-free work (see `docs/tools/linux-dev.md`).

## How it fits together
- **Steam starts copy 0.** Fallout: New Vegas checks Steam when it starts ("Application load error P:0000065432"
  otherwise), and only a game that Steam itself starts passes that check under Proton. A Steam launch option runs
  `tools/runner/proton/launch_copy.sh`, which swaps Steam's exe for copy 0's `nvse_loader.exe`.
- **Everything else runs inside that game's Proton session.** The plugin's pipes `\\.\pipe\nvcoop-<n>` exist in that
  session's wineserver only. A Windows Python started with protontricks or a bare `proton run` gets a wineserver of its
  own and finds no pipe. `launch_copy.sh` opens Steam's launch service for the game
  (`STEAM_COMPAT_LAUNCHER_SERVICE=proton`), and `tools/runner/proton/in_game.sh` uses it (through Valve's
  `steam-runtime-launch-client`) to run Windows Python and to start copy 1 in the same session. Copy 1 passes the
  Steam check from there.

## Once
1. **Install the game in Steam** and start it once through Steam, so that Proton makes the prefix and the game writes
   `FalloutPrefs.ini`. In the game's Properties, pick the Proton version under Compatibility.
2. **Windows Python 3.11 in the game's prefix.** The repo's tools are standard library only, so no pip packages are
   needed. Install Protontricks (Flatpak, from Discover or Flathub). Then, in a terminal in Desktop Mode (the installer
   needs a display even with `/quiet`), in the checkout and with the game closed:
   ```
   PFX="$(python -I tools/runner/steamlib.py prefix)"
   mkdir -p "$PFX/drive_c/nvcoop-setup"
   curl -sSfL -o "$PFX/drive_c/nvcoop-setup/python-3.11.9-amd64.exe" https://www.python.org/ftp/python/3.11.9/python-3.11.9-amd64.exe
   flatpak run --command=protontricks-launch com.github.Matoking.protontricks --no-bwrap --appid 22380 'C:\nvcoop-setup\python-3.11.9-amd64.exe' /quiet InstallAllUsers=0 'TargetDir=C:\Python311' PrependPath=0 Include_test=0 Include_launcher=0 Shortcuts=0 AssociateFiles=0
   ```
   The tools only need `drive_c/Python311/python.exe`. Run again over an existing install, the installer can fail with
   0x36b7; if `python.exe` is there, that's fine. `tools/runner/steamlib.py` prints the game's Proton prefix (`prefix`),
   the game folder (`game`) or the prefix's Documents (`documents`), wherever Steam installed the game (an SD card
   too; `documents` is `FNV_DOCUMENTS_DIR` when you set it, as for provision), so the commands here work as written
   for any library.
3. **xNVSE 6.4.9 and the quest data**, from the checkout:
   ```
   python -I tools/runner/fetch_deps.py
   python -I tools/questdata/run_all.py --data "$(python -I tools/runner/steamlib.py game)/Data"
   ```
   `fetch_deps.py` needs `7z` (SteamOS has it). It also fetches DXVK. Under Proton you don't need that one (Proton has
   its own), so provision below says `--no-dxvk`. The quest data reads the game's master files read-only (about
   30 s on a Deck). The playbooks and scenarios need it.
4. **A Win32 `nvcoop.dll`** at `out/L1/nvcoop.dll` (provision's default), with the data files the build writes next to it
   (`nvcoop_guest_local_run.json`; without it a guest blocks every DLC setup script). Build it on Windows
   (`tools/runner/build_l1.cmd`), or take CI's build of the commit you have checked out: every CI run keeps the plugin
   job's DLL and data files as the artifact `nvcoop-plugin-win32` for 14 days. With the GitHub CLI (`gh`, logged in
   through `gh auth login`), from the checkout. It looks up the `ci` run of the checked-out commit, downloads into a
   temporary folder, and replaces the files in `out/L1` only when that worked. With no run for the commit it stops:
   without a run ID, `gh run download` would take the newest artifact of any commit.
   ```
   RUN="$(gh run list -R cranebeaker/fnv-coop --workflow ci --commit "$(git rev-parse HEAD)" --limit 1 --json databaseId --jq '.[0].databaseId')"
   DL="$(mktemp -d)"
   if [ -n "$RUN" ] && gh run download -R cranebeaker/fnv-coop -n nvcoop-plugin-win32 -D "$DL" "$RUN"; then mkdir -p out/L1 && mv -f "$DL"/* out/L1/; else echo "no CI build of $(git rev-parse --short HEAD) downloaded; out/L1 left as it was"; fi
   rm -rf "$DL"
   ```
   That needs a commit CI ran on: one on `main`, or the head of a pull request (CI builds a pull request merged into
   `main`). In a browser, also logged in to GitHub: the commit's `ci` run, Summary, Artifacts; unzip
   `nvcoop-plugin-win32.zip` into `out/L1`.

## Each time the plugin or the settings change: provision two copies
```
export NVCOOP_VISIBLE=1
python -I tools/runner/instances.py provision --n 2 --nvse-dir out/cache/deps/nvse --no-dxvk --no-pad --ini coop.enable=1 --ini coop.baseline=1 --ini coop.handshake=1 --ini coop.dialogue=1 --ini live.avatars=1 --ini dev.bots=1 --ini dev.animprobe=1
```
- The copies go to `fnv-instances` next to the checkout. Every game file is hardlinked, so the copies take almost no disk
  space, and nothing is written to the Steam folder.
- Provision finds the game and its Proton prefix by itself (`tools/runner/steamlib.py`): the Steam libraries under your
  home folder, an SD card library included, and the prefix in the game's library. Your INIs come from the prefix's
  Documents. If the game is somewhere Steam doesn't list, set `FNV_GAME_DIR` to the game folder, and `FNV_DOCUMENTS_DIR`
  to the folder that has `My Games/FalloutNV` if the prefix isn't next to it. If the game never ran under Proton (no
  prefix yet), provision stops with `PrefsUnavailable`: start it once through Steam first.
- `NVCOOP_VISIBLE=1`: without it the copies start minimized.
- `coop.baseline=1 handshake=1 dialogue=1` are the player launcher's session defaults (`tools/launcher/fnv_launcher_core.py`),
  so the copies run what players run. Without `baseline=1` the host's desync repair stops quests on the client that
  are running on the host (#17).
- `dev.bots=1` turns on the bot module (`bot.fire`, `bot.move`, the input probe). Without it the playbooks can't shoot or
  walk with input, and in the runs on #17 the game clock stood still the whole time (so nothing that depends on game time
  was tested).
- `--no-pad` makes the copies ignore game controllers (`bDisable360Controller=1` in each copy's `FalloutPrefs.ini`). Steam
  Input always shows the Deck's own controls to the game as a pad, and in gamepad mode the game ignores the keyboard
  movement controls that `bot.move` holds: without `--no-pad` the input probe and every bot walk with input stand still.
  The Deck's sticks don't move the player in these copies. Your own game install isn't changed.

Each copy loads saves from its own `Saves` folder. The playbooks' scenarios start from a save named `autosave`. Copy one of
yours into both copies (they only read it):
```
SAVES="$(python -I tools/runner/steamlib.py documents)/My Games/FalloutNV/Saves"
cp "$SAVES/autosave.fos" ../fnv-instances/0/Saves/autosave.fos
cp "$SAVES/autosave.fos" ../fnv-instances/1/Saves/autosave.fos
```

## Start copy 0 through Steam
In Steam: Fallout: New Vegas, Properties, General, Launch Options:
```
<full path of the checkout>/tools/runner/proton/launch_copy.sh 0 %command%
```
Press Play. Copy 0 starts with xNVSE and the plugin; `../fnv-instances/logs/steam-launch-0.log` shows the command it ran.
Clear the launch option to play the normal game again.

## Drive it from the checkout
With copy 0 at the main menu:
```
tools/runner/proton/in_game.sh py tools/runner/nvbridge.py --instance 0 ping
tools/runner/proton/in_game.sh py tools/runner/nvbridge.py --instance 0 load autosave
tools/runner/proton/in_game.sh start 1
tools/runner/proton/in_game.sh py tools/runner/nvbridge.py --instance 1 --wait 60 load autosave
tools/runner/proton/in_game.sh py tools/runner/nvbridge.py --instance 1 call coop.status
tools/runner/proton/in_game.sh py tools/runner/live_wiring_proof.py --instances 0,1 --expect avatars
tools/runner/proton/in_game.sh py tools/bots/run_coop.py --playbook fight --instances 0,1 --keep-going
tools/runner/proton/in_game.sh py tools/bench/coop_bench.py --no-boot
```
`in_game.sh py` runs any of the repo's Python tools the same way, with paths relative to the checkout. `start 1` runs in the
background and logs to `../fnv-instances/logs/start-1.log`. Copy 1 runs in copy 0's session, so quit copy 1 before copy 0
(`in_game.sh py tools/runner/nvbridge.py --instance 1 quit`). `--instance 1 call coop.status` should show `"joined": true`
under `client`.

After each `load`, close any message box the save brings up, on both copies (for example Lonesome Road's "The Reunion"),
before the proof. A message box counts as a menu, and the host spawns no puppet for the client while a menu is open: the
proof then fails "host has a puppet ref for the client", and `call avatar.status` on copy 0 shows the puppet `Absent` with
`spawnCalls` 0.

## Known gaps
- The host commits the clock globals about 27 times a second while game time runs, so the playbooks' convergence wait
  fails at step 1 (#17). The `fight` playbook stops there.
- `L7 snapshot` fails once `out/questdata` exists, on two generated files that aren't in the repo (#62).
- The benchmark reads CPU and RAM through the runner's state file, which only `run.py up` writes, so `--no-boot` here has
  no CPU or RAM numbers.
- Copies provisioned without `--no-pad` are in gamepad mode on a Deck, and `bot.move` doesn't move the player there: the
  input probe fails `stuck: 150 units left` with the avatar never leaving its start. A fix in `bot.move` itself (driving the
  stick, or leaving gamepad mode while a bot moves) is on #17.
- A save that doesn't exist makes `loadSave` wait its full 180 s before it fails.
