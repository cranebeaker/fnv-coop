# Tale of Two Wastelands (TTW)

TTW merges Fallout 3 and its DLCs into New Vegas (18 masters: FalloutNV + 5 DLCs, Fallout3 + 5 DLCs, the 4 packs,
TaleOfTwoWastelands.esm, usually YUPTTW.esm). The co-op plugin, the sync core and the tools work on it with the switches
below. Tested on Steam 1.4.0.525 (4GB-patched), xNVSE 6.4.9, TTW + YUP, Mod Organizer 2, the Best of Times guide.

## What TTW needs from a co-op install

- **Its xNVSE plugins.** TTW checks for xNVSE >= 6.3.5, JIP LN >= 57.2, JohnnyGuitar >= 501, lStewieAl's Tweaks >= 941,
  ShowOff >= 180, ttw_nvse.dll and the 4GB patch at startup (`TTWGenericHandlerScript`). Runner instances skip
  `Data/NVSE/Plugins`; set `NVCOOP_GAME_PLUGINS=1` (or a comma list of stems) so `tools/runner/instances.py` copies them.
- **The guide's INI settings.** TTW warns "Misconfigured INI settings detected!" when `bBackgroundLoadLipFiles` is 0. MO2
  applies the profile's `FalloutCustom.ini` on top of the base INI; merge it into the INI the runner starts from
  (`FNV_DOCUMENTS_DIR/My Games/FalloutNV/Fallout.ini`). A save made after dismissing that warning crashed often.
- **NV:MP is not an option:** TTW shows `TTWNVMPIncompatibleMessage` when NV:MP's `client` DLL is loaded.
- **Story capture on:** `[hooks] story_capture=1` on every machine. TTW's ending and train use `PlayBink`, which
  replicates as the `PlayBink` story op.

## Running TTW in the runner

MO2 keeps TTW in mod folders, so the runner (which mirrors one game folder) needs a flattened copy: the game folder
hardlinked, then the enabled mods layered lowest priority first into `Data`, the ESMs as real copies with file times in
plugins.txt order (FNV sorts masters by time), and the profile's `plugins.txt` as `Plugins.txt` in the game folder. With
`bUseMyGamesDirectory=0` (the runner sets it) the game reads `.\Plugins.txt` from the instance, not
`%LOCALAPPDATA%\FalloutNV\plugins.txt`, so the owner's files are never touched.

```
FNV_GAME_DIR=<flattened TTW folder>  FNV_DOCUMENTS_DIR=<folder with My Games/FalloutNV/Fallout.ini incl. FalloutCustom.ini>
NVCOOP_GAME_PLUGINS=1  python -I tools/runner/run.py provision --n 2 --plugin-dll out/L5/nvcoop.dll --ini coop.enable=1 --ini hooks.story_capture=1
```

Videos (`PlayBink`) stall while a copy's window is minimized; run scenes with videos with `NVCOOP_VISIBLE=1`.

## Tools on TTW data

| switch | used by | meaning |
|---|---|---|
| `FNVCOOP_LOADORDER=tools/questdata/loadorder.ttw.json` | questdata, scenarios, sweeps | the TTW master order (verified in game with the bridge `plugins` method) |
| `FNVCOOP_QUESTDATA=<TTW questdata>` | coverage, scenarios, gen_coop_data | read another `run_all.py` output |
| `run_all.py --data <TTW Data> --plugins <18 masters>` | questdata | 917 quests, 4382 stages |
| `tools/coverage/build.py --qdir <TTW questdata>` | rules | adds the `ttw` lane (only when TaleOfTwoWastelands.esm is in the data) |
| `plugin/coop/tools/gen_coop_data.py --qdir <TTW questdata> --rules <TTW rules.json>` | runtime data | `nvcoop_rules.json`, `nvcoop_edids.json`, `nvcoop_infoquest.json` next to nvcoop.dll |

EditorIDs do not collide between the two games, so one `rules.json` holds both. The console here only takes hex FormIDs;
TTW's masters load at index 0x06 (Fallout3.esm) .. 0x11 (YUPTTW.esm).

## What changed in the plugin for TTW

- **Hooks next to TTW's required plugins.** ShowOff and Stewie's Tweaks re-point the call at +0xC of
  `BGSQuestObjective::SetState`; the hook prologue check accepts a re-pointed E8/E9 past the stolen bytes when its target is
  executable code outside the exe (`MemoryOps::foreignCode`). Intents call pins accept a foreign 5-byte JMP over an entry
  they only call. Bots accept the exact in-place rewrites listed in `bots_addrs.h` (`kKnownRewrites`).
- **Quest-script causes.** A quest's own GameMode script now names its quest as the cause (`questOfScript`), so the quest's
  rules apply (before, no rule ever matched effects of a quest's per-frame script).
- **Inventory stash.** `ContainerTransfer` (RemoveAllItems with a player on either side, or into a container): each machine
  moves its own player's items through its own copy of the container (The Pitt, Anchorage, Zeta, Tranquility Lane, the
  Enclave capture, TTW's train, Dead Money's bunker).
- **Videos.** `PlayBink` is a story op (event, every player); its file name travels as quoted text.

## Open

- Fallout 3 ending: provisional default, the player who enters the purifier code dies and everyone sees the ending.
- Curated rules exist for the Fallout 3 prologue (CG01 / CG03 per player, CG04 shared), MQ04, MQ09 and the train; the rest
  of the TTW quests carry derived rules.
- Guest-originated console `SetStage` does not reach the host (also on vanilla).
- One rare host crash after a guest joins (13 of 14 rounds clean on a clean save).
