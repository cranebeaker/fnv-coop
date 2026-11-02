nv-rs experimental Windows x64 playtest
=====================================

Requires your own installed Fallout: New Vegas. No game assets are included.
This is an incomplete development build, not a replacement for a normal playthrough.
VR and performance superiority are not established. Existing NVSE DLLs are unsupported.

Extract the entire ZIP into a writable folder. Double-click Play.cmd and enter
your installed game's Data folder, or run from PowerShell:

  .\Play.ps1 -DataPath 'C:\path\to\Fallout New Vegas\Data'
  .\Play.ps1 -DataPath 'C:\path\to\Fallout New Vegas\Data' -NewGame
  .\Play.ps1 -DataPath 'C:\path\to\Fallout New Vegas\Data' -Place Goodsprings

NV_RS_DATA can provide the Data path. Default location is Doc Mitchell's house.
Default load order uses official plugins only. -UseActivePlugins opts into the
existing active-plugin list; mod compatibility is incomplete. MO2 virtual files
are not automatically available to a standalone launch.

If Windows blocks a downloaded launcher, inspect the files and unblock only
that downloaded package if you trust it; do not disable system security settings.
You can also run app\nv-viewer.exe directly with the Data path and cell arguments.

Controls: WASD move; hold right mouse to look; E interact; Tab Pip-Boy;
F5 quicksave; F9 quickload; F12 bug report; Esc quit.
Saves and F12 reports are written under userdata, not your game installation.
These are nv-rs saves, not compatible with the original game's saves.

Opening: the stage55 -> tester instruction -> E -> SPECIAL segment was tested.
The full opening is unfinished: camera transition replay, Doc gaze/assistance,
face creation, and complete route acceptance remain open.

Report issues at https://github.com/slaterain/nv-rs/issues
Include BUILD.txt and steps to reproduce. Review F12 report contents for private
information before attaching. Never attach game assets or extracted game code.
Keep your userdata folder when updating; don't overwrite it with another user's saves.

Source: https://github.com/slaterain/nv-rs
License terms and dependency notices are included in this package.
