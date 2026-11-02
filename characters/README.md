# Test characters

Ready-made characters for starting the viewer somewhere later in the game, without the
opening or the face menu (`nv-viewer … --character FILE`, `world::character`).

Each file is the game's script language, one line at a time, as its console runs it, on a
new game before its first frame. Use editor IDs, never form IDs (those depend on the load
order). Blank lines and `;` comments are skipped. One line isn't script: `level N` sets the
player's level and experience directly, with no level-up menus. The viewer prints any line
that didn't take: a name that isn't a form, or a function nv-rs doesn't carry out yet.

Files hold record names only: no game text, models or other assets.

| File | Where | What |
| --- | --- | --- |
| `dead-money-entry.txt` | `SLBoSBunkerINT` | Dead Money step D2: level 20, past character creation, the Sierra Madre broadcast followed (`NVDLC01MQ00` stage 10) |
| `dead-money-villa.txt` | `DLC01StartMarker` | Dead Money after the intro: the D2 character gassed, gear taken, arriving at the Villa fountain (its wake-up runs) |
| `dead-money-casino.txt` | `NVDLC01Casino` | Dead Money Act 3 start: the Villa character with MQ02 finished and `NVDLC01MQ03a` begun, as the Lobby's knockout script leaves it |
| `dead-money-act2.txt` | `DLC01StartMarker` | Dead Money Act 2 start: the three found (MQ01a to d at stage 100), Elijah's Act 2 greeting said, Dog hired and following (MQ02a begun) |
