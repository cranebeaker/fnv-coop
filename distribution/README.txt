nv-rs experimental Windows x64 playtest
=====================================

Requires your own installed Fallout: New Vegas, version 1.4.0.525 (FalloutNV.exe is read
from your installation; no game files are included). This is an incomplete development build
of the engine port: the game's own code, translated function by function, runs on your
FalloutNV.exe. It stops where it reaches code that is not translated yet.

Extract the entire ZIP into a writable folder. Double-click Play.cmd and enter
your installed game's Data folder, or run from PowerShell:

  .\Play.ps1 -DataPath 'C:\path\to\Fallout New Vegas\Data'

NV_RS_DATA can provide the Data path. The launchers set NV_D3D9=real (the game's own
window through Direct3D 9) and NV_SOUND=out (its sound through DirectSound); you can also
run app\nv-viewer.exe directly with the Data path after setting both.

Not ready for other machines yet: the game's code answers Windows' questions (the display
adapter, the folders, the sound devices) from tables measured on the machine it runs on
(scripts\d3d9-capture, NV_D3D9_ADAPTER, default %LOCALAPPDATA%\nv-rs\d3d9_adapter.tsv;
scripts\dsound-capture, NV_DSOUND_TABLE), and this package does not make them yet. The
folders in that table are your own, so the game reads its settings and saves where the
original does (Documents\My Games\FalloutNV): back up your saves first. The controls are
the game's own.

If Windows blocks a downloaded launcher, inspect the files and unblock only
that downloaded package if you trust it; do not disable system security settings.

When the game stops, the console window's last line names where (the game's main stopped
in frame N, and the addresses it was in). Report issues at https://github.com/slaterain/nv-rs/issues
with BUILD.txt, that line and the steps to reproduce. Never attach game files.

Source: https://github.com/slaterain/nv-rs
License terms and dependency notices are included in this package.
