# Testing the nv-rs Windows preview on your PC

So far nv-rs has only run on one PC. Your PC is the test. About 15 minutes. You need Fallout: New
Vegas installed (any store), keyboard and mouse. Nothing here touches your game folder, your saves or
your settings; everything stays in the folder you unzip to.

Windows will probably warn you, because the programs are not signed: "More info", then "Run anyway",
if you trust the download. If your antivirus removes something or Windows blocks it with no way
through, that is a useful result too: tell us which.

## What to run

1. Unzip the whole ZIP into a new folder, for example `C:\Games\nv-rs`.
2. Double-click `Play.exe`. A small window says what it is doing, then the game starts.
   - If a window says it can't start, copy its words, or take a picture of it, and stop here: go to
     "What to send".
3. In the game's menu start a **New Game** (or Continue, after step 7). Get through the opening to
   Doc Mitchell's house. Walk around for a few minutes. Open the Pip-Boy. Listen: is there sound?
4. Save (Esc, Save, or F5 for a quick save).
5. Press **F12** once, anywhere. You should hear a beep. It makes a report in `userdata\reports`, even if
   nothing is wrong.
6. Exit Game from the menu. Start `Play.exe` again: this second start should take a moment only.
   Does Continue load your save?
7. Optional, if you have saves of your own: run `Import my saves.cmd`, then Continue or Load one.
8. Optional: run `Settings.cmd`, change the resolution or Window, and start again.
9. Optional, a regular `.esp` mod: put it in the `mods` folder and start `Play with mods.cmd`.

## Special setups, if you have one

- Move the folder to another drive, or a path with spaces or letters like `é` or `日本語`, and start again.
- Change your default sound device (speakers, headset), or set it to 5.1 in Windows, and start again.
- A non-English Windows, a GOG or Epic copy, a non-English copy of the game, Windows 10, an AMD or Intel
  graphics card: just say so. These are the ones nobody has tried.

## What to send

In one message, on the maintainer's Discord or at <https://github.com/slaterain/nv-rs/issues>:

1. **Your PC:** Windows version and language, graphics card, sound device, game store and language.
2. **How far it got:** started? menu? New Game? Save and Continue? Any wrong looks or sound? Frames a
   second, if you can tell.
3. **What Windows or your antivirus did** at the start.
4. **The text of any error window**, exactly.
5. **Files, zipped:** `userdata\logs\launcher.log`, `userdata\machine\tables.log` and the newest
   `userdata\logs\game-*.log`; and the folder from step 5 in `userdata\reports`, with your words in its
   `note.txt`. If something looked or sounded wrong, press F12 when it happens and send that folder too.

Logs can show your Windows user name in folder paths: look through them first if that matters to you.
**Don't send** `userdata\machine\*.tsv` unless asked, your saves, or any game files.

If the game stops, please also say whether the original does the same thing in the same place.
