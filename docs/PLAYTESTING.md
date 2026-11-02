# Playtesting

Playtesting uses the tester's own Fallout: New Vegas installation. Never include game data in a test package or copy, modify or redistribute the installation. The repository's generated test fixtures are designed to run without game files.

## Local package launch

The Windows package launchers are `Play.ps1` and `Play.cmd`. The PowerShell launcher accepts a data path and an optional place; without a place it starts at `GSDocMitchellHouse`. Use `-NewGame` to begin with a fresh game state. If `-DataPath` is omitted, `NV_RS_DATA` supplies the fallback path.

For example:

```powershell
.\Play.ps1 -DataPath 'C:\Games\Steam\steamapps\common\Fallout New Vegas\Data'
.\Play.ps1 -DataPath 'C:\Games\Steam\steamapps\common\Fallout New Vegas\Data' -Place GSDocMitchellHouse -NewGame
```

Download the experimental ZIP from GitHub Releases and extract the entire folder. Official plugins are loaded by default; -UseActivePlugins opts into your active plugin list. Read BUILD.txt and the release notes before testing. Local saves and reports belong under the package's `userdata` directory.

## Native macOS viewer

The source viewer builds for Apple Silicon and selects Metal. Launch it with
the `Data` folder from your own PC installation, as described in the
[macOS build instructions](#build-and-run-on-macos) below. On
2026-10-07, the release viewer rendered the `WastelandNV` exterior using a
PC `Data` folder on Apple Silicon. On 2026-10-10, Doc Mitchell's scripted
acceptance route reached the dialogue marker on macOS. Back in the Saddle
and Ghost Town Gunfight have not been run on macOS, and the Doc route's
dialogue-marker result has not been checked on Windows. This does not
establish gameplay-route parity or support for running `FalloutNV.exe` on
macOS.

The macOS distant-land material has no prepass or shadow pass, so it may
look different from other platforms.

### Build and run on macOS

Install stable Rust and the Xcode Command Line Tools, then pass the Data folder
from your own PC installation:

```sh
cd viewer
cargo run --release -- "/path/to/Fallout New Vegas/Data" GSDocMitchellHouse
```

Apple Silicon builds a native arm64 viewer by default. The game executable
and Windows release packages remain Windows-specific.

### Build and run on Linux (Vulkan and Wayland)

The viewer builds from source on Linux and supports native Wayland and X11
windows. On Fedora, install `alsa-lib-devel`, `libudev-devel`,
`wayland-devel`, and `libxkbcommon-devel`. Debian and Ubuntu need
`libasound2-dev`, `libudev-dev`, `libwayland-dev`, and `libxkbcommon-dev`.
Install a Vulkan loader and a working GPU driver for your system.

From a Wayland session, pass your own game data folder. When
`WAYLAND_DISPLAY` is set, winit prefers Wayland. Unsetting `DISPLAY` is
optional and forces the native Wayland path; `WGPU_BACKEND=vulkan` selects
Vulkan rendering:

```sh
cd viewer
env -u DISPLAY WGPU_BACKEND=vulkan cargo run --release -- \
  "/path/to/Fallout New Vegas/Data" GSDocMitchellHouse
```

For Steam/Proton installations, pass the active plugin list and INI when
needed with `--plugins` and `--ini`. Linux builds currently run from source;
Windows remains the packaged playtest platform.

## Current live route

The verified route is a copied stage-55 opening save moved to the vigor tester in Doc Mitchell's house. Triggering the tester advances to stage 60, Doc gives the instruction, the objective appears, and pressing E opens the original SPECIAL scene. Same-cell F9 reload at the tester also works after script occupancy is reset. This verifies that segment of the route only; it does not demonstrate completion of the opening or campaign.

The opening still needs a full acceptance run. In particular, the reported brief camera turn still needs replay verification during the lying-to-sitting transition, Doc's seated head/eye gaze is unresolved, and the exact opening choreography needs comparison with the original. See [docs/OPENING.md](OPENING.md) and [docs/OPENING_LOOK_IK.md](OPENING_LOOK_IK.md) for evidence and handoff.

A screenshot or a successful launch alone does not establish feature completion. Record the starting state, steps taken, observed result, logs or report, and comparison with the original game. Do not skip progression when checking an acceptance route.
