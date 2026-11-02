# Dead Money: research pass

Research status, 2026-10-05. This is the first step of the DLC track, which the
maintainer started alongside M1 (see MILESTONES.md): the main game's mechanics continue
on the M1 route, and this track works ahead on the DLCs, beginning with Dead Money. It
records:

* what the executable does for DLCs;
* what nv-rs can already do with Dead Money's files;
* what the DLC is expected to need;
* the data pass that turns those expectations into checked facts.

Nothing here is implemented yet.

Labels used below:

* **traced**: read in `FalloutNV.exe` 1.4.0.525, with the address;
* **in code**: present in nv-rs, at the file given, with its depth not yet checked against
  the DLC;
* **expected**: the DLC as players know it, not yet checked in `DeadMoney.esm`. The data
  pass confirms or corrects each expected item before any work builds on it.

## What the executable does for DLCs

* **traced**: no gameplay code specific to Dead Money. The executable holds none of the
  DLC's terms (Sierra Madre, collar, hologram, Ghost People, Elijah). Its only DLC names
  are the plugin names, used once, while loading plugins (`004624b0`, TESDataHandler):
  * a loaded file whose name starts with `DeadMoney`, `HonestHearts`, `OldWorldBlues` or
    `LonesomeRoad` is numbered 1–4 (`0046feb0`);
  * bit 1 << (n − 1) is then set in a byte of the object at `011c3f2c` (`00462e10`; that
    object is created in `0044fb20`).

  Where those bits are read is not traced yet.
* **traced**: a console/platform query counts the DLC packages (`dlccount`) and checks one
  by file or display name; its strings are at `0103dc90` and `0103dd20`.
* **Inference**: the DLC's mechanics are in `DeadMoney.esm` itself: records, scripts,
  effects and AI packages run by the same engine as the base game. Rebuilding Dead Money
  therefore means making the systems its data uses work. It does not mean finding new
  engine code.

## What nv-rs already does with the files

* **in code**: the load order knows the official DLC files and combines them with
  `FalloutNV.esm`, renumbering form IDs and resolving overrides
  (`crates/esm/src/load_order.rs`, `OFFICIAL_FILES`). It reads `plugins.txt`, or takes
  `--official` for the base game plus official DLC.
* **in code**: each plugin's archives are found by name, e.g. `DeadMoney - Main.bsa` for
  `DeadMoney.esm` (`crates/assets/src/lib.rs`).
* **in code**: the viewer opens any interior cell by editor ID, form ID or name, so a
  Dead Money cell can be opened directly (`nv-viewer <Data> <CELL> --official`).
* **in code**: nvinspect tags DLC records as DM/HH/OWB/LR in its coverage tables and works
  on a single plugin or a whole Data folder (`functions`, `scripts`, `scripted`, `play`,
  `hostile`, `list`, `cells` below).

## Systems the DLC is expected to need

The nv-rs column comes from a search of the code: present in some form, or nothing found.
Whether the present code covers Dead Money's use is unchecked.

| System | Dead Money's use (expected) | nv-rs |
| --- | --- | --- |
| Starting with an existing character | the DLC expects a finished character; a test route starts from a ready-made one at the entry point | own saves (`world/src/save.rs`); no reading of the game's `.fos` |
| Scripted intro | a radio broadcast in a bunker, the player knocked out, waking in the Villa | quest scripts and package actions (M1 opening work) |
| Inventory taken and returned | the player's gear removed at the start and given back at the end | `RemoveAllItems` and related functions present (`world/src/script_functions.rs`) |
| Bomb collar | **checked in the data**: scripts only, no radio engine. Each speaker or radio (`NVDLC01RadioSpeakerSCRIPT`) compares `GetDistance player` with a radius (512 for radios, 768 for speakers) and counts the player in or out of `NVDLC01BombCollarQuest.iNumRadii`; the quest script (`NVDLC01BombCollarQuestSCRIPT`) runs the countdown, beeps (`PlaySound`, `SetRumble`) and the explosion (`PlaceAtMe`, `Kill`), all gated by the global `NVDLC01Collars`. Destroying a speaker (`OnDestructionStageChange`) takes it out of the count | every function those scripts call is carried out; not run end to end yet |
| The Cloud | an area that damages the player over time | effects and image-space modifiers present; nothing DLC-specific found |
| Ghost People | stay down only when dismembered or disintegrated | dismemberment present (`world/src/stats.rs`) |
| Holograms | invulnerable guards that patrol and shoot on sight | nothing found |
| Companions (Dog/God, Dean, Christine) | follow, fight, talk; Dog/God switch personality | teammates present (`world/src/experience.rs`, UI) |
| Vending machines | Sierra Madre chips turned into items | barter present; chip vending not checked |
| Unconscious/knockout | the intro knock-out and wake-up | unconscious handling present (`world/src/functions.rs`) |

## Data pass (run locally, with your own game)

Each command reads the game's own files. Save outputs outside the repository, and copy
only counts, editor IDs and conclusions into this file: never dialogue or other game text.

All of it in one go, for all four story DLCs, into `dlc-data-pass.txt` (git ignores it):

```powershell
powershell -ExecutionPolicy Bypass -File scripts\dlc-data-pass.ps1 -Game "<your Fallout New Vegas folder>"
```

Or by hand: build nvinspect first (`cargo build --release`, giving `target\release\nvinspect.exe`), then:

```powershell
$Data = "<your Fallout New Vegas folder>\Data"
nvinspect $Data info                                  # DeadMoney.esm in the load order?
nvinspect "$Data\DeadMoney.esm" types                 # what record types the DLC adds
nvinspect "$Data\DeadMoney.esm" list QUST             # its quests (editor IDs)
nvinspect "$Data\DeadMoney.esm" cells                 # its interior cells
nvinspect "$Data\DeadMoney.esm" worlds                # its worldspaces
nvinspect "$Data\DeadMoney.esm" functions             # script functions it calls that nv-rs lacks
nvinspect "$Data\DeadMoney.esm" scripts               # scripts that don't parse yet
nvinspect $Data scripted <entry cell>                 # scripted objects where the DLC begins
nvinspect $Data hostile <cell>                        # who attacks on sight (holograms, Ghost People)
nvinspect $Data play 60 <first quest> <stage>         # run its quest scripts, list missing functions
```

The results fill four things:

1. a Dead Money quest and cell inventory, with entry points;
2. the missing script functions, most used first, which become the track's first work;
3. the intro's actual sequence of records and scripts, replacing the "expected" rows
   above;
4. which systems the base game already exercises (the co-worker's track) and which only
   Dead Money needs (this track).

## Plan

1. **D1 Inventory.** The data pass above; turn the table above into checked rows.
2. **D2 Entry.** Open the DLC's entry cell in the viewer with a ready-made test character,
   with no opening or face menu.
3. **D3 Intro.** The bunker broadcast, the knock-out and the wake-up in the Villa, traced
   from the DLC's scripts and compared in the original game.
4. **D4 onwards**, in the order the DLC's quests need them: the collar, the Cloud, Ghost
   People, holograms, companions, vending.

Each step follows the usual method: trace, write up here with addresses or record IDs,
implement with tests, open a pull request, and compare in the original game.

## D2: Entry with a test character

**Where the DLC begins (checked in the data)**: `NVDLC01MQ00` starts with the game.
* Once Radio New Vegas is on (`RNVTARef` enabled: the player has left Doc Mitchell's
  house), its script waits 30 seconds, sets stage 10 and shows the download messages.
  Stage 10 unlocks the Brotherhood bunker's entrance (`SLBoSBunkerEntranceRef`), shows its
  map marker and objective, and enables the Sierra Madre broadcast.
* The bunker interior, **`SLBoSBunkerINT`** (a base-game cell; Dead Money adds 294
  objects), is the DLC's entry cell.
* Inside, `NVDLC01VillaTravelTrigger` (`NVDLC01VillaTravelTriggerScript`) runs the
  knock-out: the objective completed, stage 100, controls off, the gas, the gear moved to
  `NVDLC01PlayerEquipmentContainerRef`, a fade, then `NVDLC01Intro` (the slideshow in
  `NVDLC01Slideshow`).
* While `bTeleportOut` is 0, MQ00's script moves anyone in the bunker back outside
  (`NVDLC01BunkerExitMarker`; "pre-patch" safety). It sets that flag on its own way to
  stage 10, so setting the stage alone (`--stage`) puts the player outdoors.

**in code**:
* `world::character`: a ready-made test character is a file of the game's script lines
  (editor IDs), run on a new game before its first frame, plus `level N`; it reports lines
  that didn't take.
* The viewer takes it as `--character FILE`, and `nvinspect play` as `--character FILE`,
  with `--cell CELL` for the player's cell.
* `characters/dead-money-entry.txt` is the D2 character: level 20, Radio New Vegas on,
  MQ00 at stage 10 with its own flags set, modest gear equipped. It also marks the other
  DLCs' start-up broadcasts as already received (the pre-order packs' items, Old World
  Blues, Honest Hearts, Lonesome Road, Gun Runners' Arsenal's message), as they would be
  for a courier who has been playing. Without that, each shows its message box on
  arrival.
* Test: `a_character_file_sets_up_the_game`.

```powershell
nv-viewer <Data> SLBoSBunkerINT --official --character characters\dead-money-entry.txt
nvinspect <Data> play 60 --official --character characters\dead-money-entry.txt --cell SLBoSBunkerINT
```

**What works** (viewer, 2026-10-05):
* The bunker loads: 303 objects, its lighting, fog, image space, collision and doors,
  military location music and the bunker's ambient sound. The player starts at the door
  from the Mojave, level 20, with the 9mm in hand (the HUD shows its ammunition).
* No face menu, no opening, and no stray message boxes. MQ00 stays at stage 10 and doesn't
  warp the player out.
* Walking into the travel trigger runs the knock-out in order:
  * MQ00's objective completed, stage 100;
  * controls off and the `ExplosionInFace` screen effect;
  * the move to `NVDLC01PlayerGasMarker` and the player's captured idle;
  * `FadeToBlackPermanent`, then `NVDLC01Intro`: the slideshow music, the move into
    `NVDLC01Slideshow`, the fade lifted, the narrator's first line.
* Headless (`nvinspect play` with the character, 60 s): no messages. Stages set: MQ00 10
  and the other DLCs' 10s.

**Missing** (for D3 and later):
* Objects placed **initially disabled** in the cell the player is in can't be enabled by a
  script: the viewer leaves them out when it loads the cell. This affects the gas jets
  (`FXSprayJet0101Ref`…`0104Ref`), which the trigger script enables in this same cell. A
  viewer issue, not Dead Money's.
* The slideshow's first view looked tilted; check against the game in D3.
* That the gear really moves to the equipment container isn't checked yet (D3).
* The bunker's vending machine, workbench and reloading bench open the recipe menu (see
  "Crafting").
* Calls from other quests that aren't carried out, seen running headless: Lonesome Road's
  `AddItemToLeveledList`, and base-game scripts' `GetInSameCell`, `GetDetected`, and
  `GetAngle` / `GetDistance` on the player (the viewer knows where the player is; the
  headless run doesn't).
* Nothing compared in the original game yet.

## Radio

Dead Money calls six radio functions nv-rs lacked: the Pip-Boy radio switched off for the
intro and the trip to the Villa (`PipBoyRadioOff`), Elijah's broadcast on the collar
(`PipboyRadio Tune`, then `StartRadioConversation`), the Sierra Madre's ambient music
changing with the story (`StartRadioConversation` on `NVDLC01RadioStationAMBREF`), the
Starlet hologram at the fountain playing a station (`SetNPCRadio`), and a refresh at the
start (`ForceRadioStationUpdate`, `ResetPipboyManager`). None of them drives the collar
(above).

**traced** (`FalloutRadio`, `0083xxxx`; script handlers from the command table at
`01190910`):

* `PipboyRadio` (`005d7fb0`) takes a word and an optional station. A word starting with
  `1`, or `enable`/`on`, switches the Pip-Boy radio on (`008324e0`) and tunes it
  (`00832240`); one starting with `0`, or `disable`/`off`, switches it off; `tune` tunes
  it. The words are compared without case: Dead Money writes `Tune`.
* Switching off (`008324e0(0)`, also `PipBoyRadioOff`, `005dc580`) forgets the tuned
  station (`011dd42c`). The on flag is `011dd434`. A radio-wide "disabled" flag
  (`011dd436`) makes all of this do nothing; what sets it isn't traced.
* Tuning only works while on. The station object is found in the radio's list or made
  (`00832cb0`): a reference whose base is a talking activator (form type 0x16) is a
  station itself; activators, NPCs, creatures and levelled lists that aren't actors take
  their base's radio template (`004fd3c0` → `008356e0`). When none can be made, the radio
  switches off. A new station's first update is staggered by a random 0–30,000 ms
  (`00944460`).
* `StartRadioConversation` (`005d82a0` → `00835be0`), on a station, ends what it was
  playing and starts the topic given, or the default (`0061a2d0(7, 0)`: the first entry
  of the radio dialogue list). Its next update is 50 ms later. If the Pip-Boy is on and
  tuned to that station, the radio sound restarts.
* `SetNPCRadio` (`005d8100`), on an actor with a station: 1 plays the station through
  that actor (`00835810`), 0 stops it (`00835980`), other values do nothing.
* `ForceRadioStationUpdate` (`005d8280` → `00832ad0(1)`) makes the stations update at
  once instead of at `iRadioUpdateInterval`.
* `ResetPipboyManager` (`005db490`) sets the player's Pip-Boy manager's reset flag
  (+0x16c); its reader isn't traced.

**in code** (`crates/world/src/more_functions/radio.rs`, tests
`the_pipboy_radio_and_its_stations`, `a_person_plays_a_station_near_the_player`): since
2026-10-06 (`claude/m2-radio-unify`) the six functions act on the one radio the Pip-Boy
shows and plays (`world::radio`, `GameState::radio`; see [PIPBOY.md](PIPBOY.md)): tuning
plays the station, `PipBoyRadioOff` stops it, `StartRadioConversation` replaces the
station's programme with the topic's, and `SetNPCRadio` makes the person a receiver who
plays the station's lines near the player (`00834260`). Saved with the game
(`radioconversation`, `npcradio`, `pipboyreset`; the old `scriptradio` line loads as the
Pip-Boy radio). With them, nvinspect counts 178 of the 199 functions Dead Money uses as
carried out (172 before).

**Not done:** stations made from activators' or actors' radio templates, the receivers
placed in the world (they play without falloff), a station's place in its programme
across a save. Nothing has been compared in the original game.

## Animations playing (`IsAnimPlaying`)

Dead Money asks 26 times, on placed objects only: speakers, gates and Elijah's talking
activator (`LinkedRef.IsAnimPlaying Forward`, `Backward`, `Left`, `Right`), mostly to
avoid starting a group that's already playing.

**traced** (`005c14a0`): on a reference with actor animation data (vtable +0x1e4), its
eight sequence slots. Otherwise the model's controller manager: with a group, the
sequence named after the group (`00438170` → `0047a520`) is checked; without one, any
sequence. "Playing" is the sequence's state (+0x44, read by `008041a0`) not being 0
(inactive). No 3D loaded: 0. **Compared in the original game**: `PlayGroup Backward 1` on
`NVDLC01ElijahTalkingActivatorREF` (a short fade out and back in), then `IsAnimPlaying
Backward` read 1 while it played and 0 once it had finished, and stayed 0. So a finished
one-shot (clamped) group stops counting as playing. (Reading the sequence update,
`00a34ba0`, showed no end-of-cycle deactivation at the sequence level, which had led to
the opposite inference; the code that ends the group at the reference level, in the
group-finished handling, isn't read, nor what ends a clamped group on models with no end
event.) Dead Money's `NVDLC01EnableElijahTriggerSCRIPT` depends on it: it plays `Backward`
once, then every frame plays `Right` only while neither `Backward` nor `Right` is playing.

**in code**: the viewer reports each frame the sequences active on placed objects
(`world::more_functions::report_sequences`, from `move_pieces`): a script's `PlayGroup`
sequence, a door's `Open`/`Close`, or the model's running start-up sequences (ones holding
a frame aren't counted: a guess), each only while it plays: a looping one always, any
other until its end (`preview::cell::sequence_playing`; its last pose is held, but it no
longer counts). `IsAnimPlaying` answers from that report, comparing group names without
case. People's animation data isn't carried out: asked about a group, the script stops;
asked about any, a person on their feet reads 1 and one who is down 0 (Dead Money's ghost
people). Tests: `objects_animations_playing`, and `a_one_shot_sequence_stops_playing_…` in
`preview`. nvinspect: 179 of 199.

## Line of sight (`GetLineOfSight`)

42 uses in 6 scripts. **traced** (`005c1ce0` → `0059c990`): the caller must be an actor
and the target given; otherwise 0.

* **The player asking** (caller `011dea3c`): first whether the target is in view (its 3D
  bound against the camera, `004b5fc0`, or `00444ed0` with a bound test). In view, rays
  are cast (collision layer 0x25, `SpecificItemCollector`) from the camera position to the
  target at 0.75, 0.5 and 0.25 of its bound height (max z − min z, vtable +0x1dc/+0x1d8)
  above its position; a ray hitting nothing or the target itself gives 1. Otherwise the
  actor test below decides.
* **Anyone else** (the script command `0059c990` passes 0, 0, 1, the target, 0, so
  `0088b880`'s first argument is 0; checked in the disassembly against the executable):
  the target needs 3D. Within 2 units of the caller (`005723b0` against 2.0) the answer
  is 1. Otherwise the caller's AI process answers for any target (vtable +0x2cc =
  `HighProcess` `008f6930`), given the target only if it is an actor: it looks the target
  up in its detection list (`008f6650`) and returns that entry's byte at +0x1e, so a
  target that isn't an actor, with no entry, answers 0. The view cone (`0088c570`) and
  the rays from the caller's eye are reached only with a nonzero first argument, which no
  script command passes: they are not part of a script's `GetLineOfSight` for anyone but
  the player.
* With the console's debug flag, it prints "sees" or "can't see".

Dead Money's uses: `Player.GetLOS` on Ghost People (36: the camera path) and
`HoloA/B/C.GetLOS Player`, `NVDLC01DeanRef.GetLOS Player` (16: the actor test). The actor
test's process answer is the `HighProcess`'s (`008f6930`): its detection data on the target
(vtable +0x504), byte +0x1e, the line of sight its last detection run found; other
processes (`008d0510`) say 0. `005723b0` is the distance between the two (at most 2.0
gives 1).

**in code** (`crates/world/src/sight.rs`, test `line_of_sight`): the rules above in the
world. The viewer answers what they ask through `world::sight::Sight`, given to the
`Runner` that runs placed objects' and quests' scripts each frame
(`viewer/src/sight.rs`): a reference's bound (placed objects' rendered bounds; people's
collision shape, a stand-in for their model's bound), the camera's position and view (any
corner or the middle of the box inside the frustum), and rays through the cell's
collision. nv-rs's collision doesn't know which object a ray hit, so a ray stopping where
it enters the target's box counts as hitting the target (a stand-in). The viewer's
detection runs report their line of sight (`report_detection_sight`), which the actor
test reads. An object target for someone other than the player answers 0, and two
within 2 units answer 1 (corrected from the disassembly; they were "not carried out" and
0). Not carried out: the player's test headless, and conditions (`CTDA`) asking it.
Nothing compared in the original game.

## Effect shaders (`PlayMagicShaderVisuals`, `StopMagicShaderVisuals`)

160 calls. Mostly the holograms' moods: scripts swap `NVDLC01HologramNeutral`,
`NVDLC01HologramAttention` and `NVDLC01HologramAggressive` on them (object scripts, the
hologram spell effects' `ScriptEffectStart`/`Finish` blocks, a terminal), plus the heated
knife's `Flames01`/`Smoke01`, Elijah and Dog/God.

**traced**:

* `PlayMagicShaderVisuals` (`005d1b80`) takes an effect shader and seconds (default −1).
  No reference: the player. Unless the reference's cell is attached (`004511e0`) and it
  has 3D (vtable +0x1d0), nothing happens (it still succeeds). Otherwise a new
  `MagicShaderHitEffect` (`0081f580`) is made for the reference and shader: seconds of 0
  or more are its lifetime, below 0 `FLT_MAX` (until stopped). If it initialises
  (vtable +0xc4) it joins the running effects (`00973fd0`); one already running isn't
  replaced, so they stack.
* `StopMagicShaderVisuals` (`005d2130` → `00974a50`) ends every running
  `MagicShaderHitEffect` on the reference with that shader.
* With the console's debug flag both print what they did.

**in code** (`crates/world/src/more_functions/shaders.rs`, test
`effect_shaders_on_references`): the running shaders (reference, shader, end time on the
state's clock) with those rules, and `Shown::ShaderVisual` / `ShaderVisualStopped` for the
viewer. The viewer reports each frame what has 3D (`report_loaded`: rendered placed
objects, people about, the player). nvinspect: 182 of 199.

**Not done**: drawing them. nv-rs doesn't read `EFSH` records yet (fill and edge
textures, colours and their timing, membrane and particle shaders); the viewer gets the
events and draws nothing. Not traced: what makes initialisation fail (taken here as no
shader), whether running shader effects are saved (not saved here), the player's
first-person flag (+0x4c). Nothing compared in the original game.

## Terminals going back (`ForceTerminalBack`)

20 calls in 17 terminal items, all in the Vault and the hologram vault terminals: a
confirmation sub-screen's "No" item runs `ForceTerminalBack` to return to the screen
before (`NVDLC01VaultMainInfoDeanTerminal` and others).

**traced** (`005dc4e0`): if the terminal menu (1057, `ComputersMenu`) is open
(`00a09030`), its screen stack is popped (`00758a80` → `0063f7b0`) and the new top shown
(`007586e0`); with no screen left the terminal closes (`00757ea0`). Otherwise nothing.

**in code**: the function sends `Shown::TerminalBack` while `menu_open` is the terminal
menu (`world::terminal::TERMINAL_MENU`), which the viewer now sets while a terminal is
shown. The viewer's terminal pops its screen stack for each one, closing past the first
screen, right after the item's script and each frame (`viewer/src/menus.rs`,
`terminal_backs`). Tests: `terminals_go_back_only_while_open`,
`force_terminal_back_pops_screens_then_closes`. nvinspect: 183 of 199. **Not traced**: for
an item with both a script and a sub-menu, whether the sub-menu opens before or after the
script's back (here: after). Nothing compared in the original game.

## Caravan cards (`GetContainer`, `RemoveMe`, `AddCardToPlayer`)

12 calls, all in one script: `NVDLC01CardAddToPlayerScript`, on the Sierra Madre's "Dead
Man's Hand" cards (a copy of the base game's `CardAddToPlayerScript`). Its `OnAdd` block:
if `GetContainer` is the player, `AddCardToPlayer` then `RemoveMe`. A card picked up joins
the player's Caravan cards and leaves the inventory.

**traced** (handlers take the script's item and its containing object):

* `GetContainer` (`005ce5c0`): the containing object when there's an item and a
  container; else 0.
* `RemoveMe` (`005b53d0`): with an item and a container, one of the item leaves the
  container (its RemoveItem, vtable +0x17c, count 1), into the container given if any; for
  an actor, the equipped instance's extra data (`004bfda0`); the player's inventory is
  refreshed (`00704af0`). The handler returns failure, which ends the script.
* `AddCardToPlayer` (`005cf3d0`): the item's base must be a `TESCaravanCard`; it joins the
  player's cards unless already there (`00969bc0`). Otherwise an error is reported only.

**in code**: `Runner::on_add` runs an item's `OnAdd` blocks (no container named, or this
one) with the item and its container (`Runner::container`); the viewer runs it when the
player picks up a placed item. The three functions are in
`crates/world/src/more_functions/carried.rs`; the cards are saved. Test:
`caravan_cards_picked_up`. nvinspect: 186 of 199.

**Not done**: `OnAdd` for items arriving other ways (taken from containers, `AddItem`,
barter): what the game gives their scripts as the item isn't traced. `RemoveMe` doesn't end
the script (nv-rs's statements can't; Dead Money calls it last); an actor's equipped
instance isn't told apart. The Caravan game itself and a deck aren't here. Nothing compared
in the original game.

## Companions and actors (`OpenTeammateContainer`, `PushActorAway`, `SetDisposition`, `GetCauseofDeath`)

9 calls. `OpenTeammateContainer` is the result of four companion dialogue lines (the
`FollowersTrade` topic). `PushActorAway`: Elijah pushes the player with 5 in a dialogue
result when `VaultCodeBox.bPlayerBlocks` is set (`NVDLC01ElijahVaultSentryDownTopic03`),
and a spell effect (`NVDLC01StarletKnockdownScript`) has the lobby hologram push whoever
it lands on, other than itself and the player, with 10. `SetDisposition` is in the lobby
hologram's `OnHit` (`NVDLC01StarletLobbyScript`: `NVDLC01StarLobby.SetDisposition player
100`; its comment says it keeps her from turning hostile). `GetCauseofDeath` is in Dog's
death script (`NVDLC01DogScript`): killed by an explosion (`GetCauseOfDeath == 0`) in the
restaurant with the gas traps not all set (`nGasTraps != 3`), the kitchen valves explode
and the player dies; any other death there starts the collar's countdown.

**traced**:

* `OpenTeammateContainer` (`005d9430`): on a person or creature who is the player's
  teammate (actor +0x18d), or any with the optional number not 0, the container menu
  opens on their things in its companion mode (`00709470`, mode 3). Always succeeds.
* `PushActorAway` (`005d6b60`): the caller pushes the actor given away. Not an actor: the
  game prints "SCRIPTS: PushActorAway in script '…' is attempting to push a non-actor
  reference." and nothing else. The force (`00646580`) is (`fKnockbackAgilBase` +
  `fKnockbackAgilMult` × Agility × 10) × (number × `fKnockbackDamageMult` +
  `fKnockbackDamageBase`), with the pushed actor's Agility. None of these settings is in
  `FalloutNV.esm`; the exe's defaults (`00f61a40`…`00f61ad0`) are 1, −0.008, 50 and 10.
  Agility 5 and Elijah's 5 give 60; Agility 10 gives 20. Only an actor with the high AI
  process (process +0x28 is 0) is pushed: from the caller's centre (`Actor` vtable +0x1f4,
  `008ae4c0`) through the process (`0091fee0`), which for an actor that can be knocked
  down sets its knock state to 2 and throws its ragdoll from that point with the force.
  With the player as the caller, something more goes to `005f5950` first (not traced).
* `SetDisposition` (`005d54a0`): on a person or creature, with an actor given, takes the
  disposition now (vtable +0x344, `0087fd90`) from the number and adds the difference
  (vtable +0x460, `0087fb40`). That adds only toward the player: the actor keeps a list of
  (amount, toward whom) at +0xfc (change flag 0x80000), and the amount is cut so the
  disposition stays within 0–100.
* `GetCauseofDeath` (`005be740` → `005a3d30` → `005730d0`): on a person or creature, the
  cause kept in its dismembered-limbs extra data (0x5f, +0x10); else −1. It is written at
  death (`00572fc0`, from `008b4d10`): the hit handler (`0089a760`) picks it by the form
  type of what struck — a weapon (a melee blow) 2, a missile, beam, flame or continuous
  beam projectile 1, a grenade or an explosion 0, an ingestible 5, debris 4, anything
  else (fists, a creature's attack) 3; a source with vtable +0x220 set makes it 0. `Kill`
  with a limb keeps its third number (default −1) as the cause (`005be2a0` → `008b51b0`).

**in code**: `crates/world/src/more_functions/actors.rs`. `OpenTeammateContainer` sends
`Shown::TeammateContainer`, and the viewer opens its container menu on the companion.
`PushActorAway` sends `Shown::PushedAway` with the force, for someone the viewer reports
loaded. Dispositions toward the player and causes of death are kept and saved. Killing
hits keep a cause: no weapon 3, a melee weapon 2, a lobber's projectile 0, other
projectiles 1. Test: `companions_pushes_dispositions_and_causes_of_death`. nvinspect: 190
of 199.

**Not done**: the container menu's companion mode is drawn as an ordinary container (mode
1). Being knocked down alive isn't drawn: the viewer only ragdolls the dead, and the knock
state and getting up aren't kept (`GetKnockedState` stays 0). The rest of the game's
disposition reckoning (`0087fd90`: factions, Charisma and more) isn't carried out, so
the disposition toward the player is only what scripts added (starting at 0), and
`GetDisposition`/`ModDisposition` aren't wired to it yet. nv-rs has no explosions,
poison or debris that kill, so causes 0 (except grenades), 4 and 5 don't happen. The
+0x220 source flag and the player-caller branch of `PushActorAway` aren't traced.
Nothing compared in the original game.

## Dispelling (`DispelAllSpells`)

3 calls, all `player.DispelAllSpells ; Removes all chem effects.`: on arrival
(`NVDLC01IntroSCRIPT`), in `NVDLC01MQ03Script`, and on the way back through the gate to
the Mojave (`NVDLC01SMGateMojaveSCRIPT`).

**traced**: `DispelAllSpells` (`005c2190`) on a person or creature walks its active
effects (`008249d0`) and dispels (`00804210`) each
one whose magic item's type (vtable +0x18) is 0 (a spell), 2 (a power), 3 (a lesser
power), 7 (an ingestible: `AlchemyItem` `009d2510`) or 8 (an ingredient: `IngredientItem`
`006e4ba0`). A spell's type is its `SPIT` type (`SpellItem` +0x1c on the magic item,
`00441110`). An enchantment (type 6, `009d6b60`) goes only when its own type (+0x34, the
first `ENIT` field) is 0. Diseases (1), abilities (4), poisons (5) and addictions stay.
Anything else does nothing; it always succeeds.

**in code**: `DispelAllSpells` in `crates/world/src/more_functions/actors.rs`
(`dispelled_by_all` picks the effects by their source's record); script effects run
their `ScriptEffectFinish`, as `Dispel` does here. Test:
`dispel_all_spells_leaves_abilities_and_poisons`. nvinspect: 191 of 199.

**Not done**: equipment enchantments aren't active effects in nv-rs, so the `ENCH` rule
has nothing to act on yet. Nothing compared in the original game.

## Traps (`SetVATSTarget`, `FireWeapon`)

3 calls. The two tripwire scripts (`NVDLC01TrapTripwireSCRIPT`,
`NVDLC01TrapGenericTripwireSCRIPT`) start their `OnActivate` with `setVatsTarget 1`. The
shotgun trap (`NVDLC01TrapShotgunSCRIPT`) fires `WeapNVSingleShotgun` once when something
other than the player activates it.

**traced**:

* `SetVATSTarget` (`005daae0`): only on a reference with the flag 0x01000000 (`00452370`;
  the destruction code `00477d10` asks the same flag, so it marks a destructible
  reference). It compares the number (not 0 = targetable) with the base's own
  "V.A.T.S. targetable" flag: its destructible data (`00475400`) `DEST` flags bit 0x01
  (`00576100`). Equal clears the reference's flag 0x04000000, different sets it
  (`004846e0`, change flag 1). V.A.T.S. asks `00576070` when it gathers objects
  (`007f52c0`): destructible, the base's flag, turned the other way by 0x04000000.
* `FireWeapon` (`005da570`): the argument must be a weapon (form type 0x28), else the game
  prints "SCRIPTS: FireWeapon in script '…' called with non-weapon parameter.". The
  reference then fires it through the game's weapon fire (`00523150`, deferred off the
  main thread by `008c7aa0`). For something that isn't an actor, the shot leaves the
  object's position along its X (pitch) and Z (heading) angles. When its 3D has a
  projectile node, it leaves that node instead, along the node's facing. The node is found
  by `00525700`: the weapon's own node name when it has one (`005256b0`), else
  `ProjectileNode`, else `##ProjectileNode`.

**in code**: `crates/world/src/more_functions/traps.rs`: the override is kept and saved,
`vats_targetable` is the V.A.T.S. test, `shot_from` works out the shot.
`Shown::WeaponFired` goes to the viewer's `combat::object_shots`. That system finds the
node in the model (`nif::Nif::placed_node`) and fires each pellet within the weapon's
cone, as the player's shots do. The first person met (the player or someone about, by
their bounds) before a wall takes the hit (`Runner::hit_at` with the object as attacker;
they don't fight the object back). Tests: `traps_vats_targets_and_weapons_fired`,
`shots_leave_along_the_objects_facing`, `finds_a_placed_node_by_name`. nvinspect: 193 of
199.

**Not done**: nv-rs's V.A.T.S. doesn't target objects yet, so `vats_targetable` isn't
asked by it. The reference flag 0x01000000 is taken to mean "the base has destruction
data" (inferred). The weapon's own projectile node name isn't read. Shots are instant
rays against bounds, not projectiles in flight against hit shapes; the facing's sign for
pitch follows nv-rs's placement convention (positive X tips the nose down), not checked
in the game. Objects made by `PlaceAtMe` can't fire yet. Nothing compared in the
original game.

## Conditions (`GetVATSValue`, `IsFacingUp`)

4 conditions, no script calls. The Hobbler perk (`NVDLC01Hobbler`) asks `GetVATSValue 5`
with the left and right leg's actor values (29, 30). The ghost people's get-up idles
(`GhostGetUpFaceUp`, `GhostGetUpFaceDown`) ask `IsFacingUp`.

**traced**:

* `GetVATSValue` (`00594dc0`, conditions `00594e40`): the attack V.A.T.S. is playing
  (the player's, `009c71c0`, or an NPC's, `007f5280`), by case. Case 5 is the attack's
  body part actor value against the number. nv-rs already answered it
  (`world::vats::function_value`), but the V.A.T.S. functions were missing from
  `scripting::HANDLED`, so `nvinspect` counted them as not carried out. Cases 2 and 9
  compare forms with TESForm vtable +0x10c (`00603880` for people), which gives 0 when
  they're the same. nv-rs's equality checks match that.
* `IsFacingUp` (`005cb720` → `005a0710`): on a person or creature, finds a node in its
  3D: first the name `004b7920` keeps at `011c61b4` ("Bip01 Spine"), then "Bip01
  Spine01". With a node, the answer is 1 when its world rotation's [2][1] (the node's
  +0x84) is above 0 (`00c6b7b0`). With no 3D or no node, the answer is 1 too. Anything
  else gives 0.

**in code**: the ten `GetVATS…` functions are in `HANDLED`. `IsFacingUp` is a read in
`world::more_functions`, answered from `report_facing_up`. The viewer's `animate_actors`
works out each pose's spine (`actors::spine_up`), and `report_facing_up` sends it every
frame. A person's placement turns only about Z, which leaves that row of the rotation
alone. Tests: `facing_up_as_the_viewer_reports_it`, `the_spine_faces_up_by_its_rotation`.
nvinspect: 195 of 199.

**Not done**: nv-rs has no knockdowns or getting up, so the get-up idles aren't played
yet; a ragdoll asleep keeps its last report. The player isn't reported (counts as no 3D:
1). Nothing compared in the original game.

## Crafting and casino menus (`ShowRecipeMenu`, `Show…MenuParams`)

6 calls.
* The Sierra Madre vending machines' scripts (`CraftingVendingMachineRecipesScript`,
  `NVDLC01CraftingMachineRecipeOFFScript`) call `player.ShowRecipeMenu
  NVDLC01VendingMachineRecipes`.
* The casino's tables call `ShowSlotMachineMenuParams SierraMadreCasinoData 1 25 0`,
  `ShowBlackjackMenuParams … 1 200 0` and `ShowRouletteMenuParams … 1 100 0`, unless the
  player is banned (`NVDLC01CasinoComps.bBanned`).

**traced**:

* `ShowRecipeMenu` (`005deb10`): the vendor is the person the script runs on. For a
  talking activator, the vendor is its base's speaker (+0x90), when the reference's
  +0x81 is set. The category (`RCCT`) is optional. `00704fc0` → `00726ff0` opens
  `Data\Menus\recipe_menu.xml` as menu 1077 (`007273e0`) with the vendor and category.
  With no vendor, the game prints "Recipe menu called with NULL vendor!  Oh, noes!".
* `ShowSlotMachineMenuParams` (`005cf040`), `ShowBlackJackMenuParams` (`005cf0f0`) and
  `ShowRouletteMenuParams` (`005cf1a0`) each take a casino (`CSNO`) and three numbers. They
  hand them to the game's menu: slots `007c0a40` (menu 1080), blackjack `00733630`
  (1081), roulette `007bbe20` (1082). Each menu keeps them in its globals. With no casino,
  the game prints an "Invalid EditorFormID" line; the roulette handler's message names
  blackjack.

**in code**: `crates/world/src/more_functions/menus.rs` sends `Shown::RecipeMenu` or
`Shown::CasinoMenu`, then the menu's number (`Event::Menu`). The viewer counts each as
opened and closed at once, so the scripts' `MenuMode` blocks run. Test:
`recipe_and_casino_menus_open_with_their_data`. nvinspect: 199 of 199.

**Not done**: nv-rs has no casino games (slots, blackjack, roulette, the `CSNO` data,
winnings), so nothing can be played yet (crafting is done: see "Crafting"). That is a separate
piece of work, larger than a function.
What the three numbers mean in each game isn't traced; the reference's +0x81 check for
talking activators is taken to be met. Nothing compared in the original game.

## Crafting (`RCPE`, `RCCT`)

`world::crafting`; `nvinspect <Data> craft [CATEGORY]` lists what a new character is offered.

**Read from the data** (the records, not the game's code):

* A recipe's `DATA` is four numbers: the skill it asks for (an actor value number; `0xFFFFFFFF`
  for none, as the Sierra Madre vending machine's), the level, its category (`RCCT`: Workbench,
  Campfire, Reloading Bench, `NVDLC01VendingMachineRecipes`) and its sub-category (Aid, Weapons,
  Ammo, Chems, Food, Misc ...). `RCIL` + `RCQY` pairs are what it uses, `RCOD` + `RCQY` pairs what it
  makes (a breakdown makes several things); `CTDA` conditions say when it is on offer.
* Learnt recipes' conditions ask `GetHasNote` for their recipe note, which the note's script gives
  (`player.AddNote`) when the recipe item is picked up. The three bench scripts
  (`CraftingWorkbenchRecipesScript` and the campfire's and reloading bench's) call
  `player.ShowRecipeMenu <category>` when the player activates them; the Dead Money vending
  machines do the same with `NVDLC01VendingMachineRecipes` (the first use shows a message instead).
* Against the real data a new character is offered 14 vending machine recipes (all priced in Sierra
  Madre chips, from 5 to 55, and the "[Return]" ones that pay chips for cigarettes), 41 workbench and
  68 campfire ones.

**In code**: `Recipe::load`, `offers` (the list for a category), `sub_categories`, `can_make`,
`make`. Tests: `crates/world/tests/crafting.rs`.

**The menu**: `ui::menus::recipe` (class 1077, `menusecipe_menu.xml`; 6 tests) and
`viewer/src/game_menus/recipe.rs`. A script's `ShowRecipeMenu` opens it on the category
(`Shown::RecipeMenu`); `--open-menu recipes:CampfireRecipes` opens it for testing. It fills the
file's own tiles: recipes in a list (id 3), the title (1) with the filter arrows (0, 2), "Made at"
(4), the skill requirement (5), the ingredients' list (6, "have/need" on the right), the picture of
the first output (9), Accept (7, key A) and Exit (8, key X, also Escape). Accept makes the chosen
recipe through `world::crafting::make` and refills the lists. Seen with the real data: the
vending machine's 14 recipes under the file's own layout. The pointer path (choosing a row with the
mouse, Accept) is unit-tested only, not tried by hand.

**Not done**: the item data card under the list (`RM_ItemData`: damage, weight, value ...) stays
empty; no crafting sound; no experience.

**Guessed, to check in the original game**:

* A recipe is listed when its conditions pass, and also when the player lacks the skill or an
  ingredient (shown as not makeable). The original may hide those.
* The skill check is the player's current value (chems and gear counted) at least the recipe's level.
* With no category given, `ShowRecipeMenu` lists every category's recipes.
* Making one uses up each ingredient in full and adds the outputs; "Items Crafted" (misc stat 32)
  goes up by one for each recipe made, whatever it makes. No experience, no skill gain, no sound.
* The list is sorted by name; the filter arrows go through the sub-categories in the order they first
  appear, "all" first, wrapping round, and the title shows the sub-category's name in capitals.
* Clicking a row makes it, as the Accept button does; the button is off (answering a key with the
  cancel sound) while nothing makeable is chosen. Whether the original needs a second click is
  unknown.
* The ingredients show "have/need" on the right; a recipe that can't be made, and an ingredient the
  player lacks, get the failed-check look.
* "Made at" shows the category's name, and only once a recipe is chosen.

## D3: the intro slideshow and the Villa start

- **`SayToDone` blocks.** A dialogue line said with `SayTo` runs the speaker's
  `SayToDone` blocks for that topic when the line ends (block type 7, table
  entry `0118e408`; where the game dispatches it isn't traced). A block with
  no topic runs for every topic. The narrator's chain (IntroFNV, IntroDLC01,
  the slides, IntroEnd) runs this way: each result script starts the next
  line and swaps the slide.
- **`SwapTextureOnRef`** (`005cf860`) finds the node by its exact name in the
  reference's 3D and loads `Textures\<name>.dds` into its first slot. The
  viewer gives the piece with that NIF shape name its own material
  (`viewer/src/swaps.rs`). The slide's shape is `##NVDLC01_IntroMovie00:0`.
- **Starting in the Villa.** `characters/dead-money-villa.txt` does what the
  intro's end does (`SetInChargen 0`, the equipment container emptied, MQ00
  finished) and the viewer starts at `DLC01StartMarker`. `CELL` may now be the
  editor ID of a placed reference; the viewer opens the place it stands in.
- **Persistent triggers outdoors.** Outdoor persistent references live in the
  worldspace's persistent cell, not in the square's cell, so the fountain
  trigger (`NVDCL01FountainTriggerRef`) never ran. Scripted lists outdoors now
  include them (`interactive_references_outdoors`); the wake-up and
  `NVDLC01FountainStartSequence` run.
- **Ogg crash.** Bevy's ogg playback crashed the viewer in the Villa. Ogg is
  now decoded in the viewer (lewton) and played as PCM like the other sounds.
- **Narrator voice.** The narrator (`NVDLC01Narrator`) is an actor of voice
  type `MaleAdult01Default`, but all his intro lines are recorded under
  Elijah's voice type folder (`nvdlc01maleuniqueelijah`). Traced since
  (2026-10-06, `00616fa0`): a line's own speaker (`INFO` `ANAM`,
  `TESTopicInfo::pSpeaker`) gives the voice type when set, and the intro
  lines name Elijah. `world::dialogue::voice_path` does this; the earlier
  guess (any voice type folder with the same file name) is gone.
- **References enabled after the place loaded.** Ones that start disabled
  (the gas jets, `FXSprayJet0101Ref`…) weren't drawn when a script enabled
  them. The viewer now draws them once a second, if they are in the place
  (`bring_in_enabled`).
- **Talking activators** (Elijah's hologram, `NVDLC01ElijahTalkingActivatorREF`).
  After the wake-up, `NVDLC01FountainStartSequenceSCRIPT` calls `Activate
  Player` on it. A talking activator with a voice type (its `VNAM`) now
  starts a conversation when a script has the player activate it: the
  greeting, with the player's choices.

- **Gear removal** works: after `RemoveAllItems NVDLC01PlayerEquipmentContainerRef`
  the pistol is unequipped and the ammo count leaves the HUD.
- **Slideshow view:** the slide fills the view upright; no tilt shows.

## Playing Act 1 (MQ01), in the order a player meets it

Played at the rules level: the viewer in each place with `--run` lines for
what the player's hands would do, and `--talk` / `--choose` (new: reply
numbers picked in order, unattended) for conversations.

1. **Fountain wake-up and Elijah.** Works: the wake-up, MQ01's start, the
   greeting and his replies; his last reply starts the three finding quests
   (Dog 8, Dean 14, Christine 12). The objectives show as they come.
2. **Collars.** The collar's beeping and its radio interference messages
   work in the Villa.
3. **Dog (police station).** Works: his greeting moves the quest on to the
   basement. Taking the "Dog Command Tape" moves it on again
   (`NVDLC01MQ01aSCRIPT` watches `GetHasNote`). Playing it was missing: the
   Pip-Boy's notes list couldn't play a voice note (`NOTE` of type 3). Now
   Enter on one has its speaker (`SNAM`) say its topic (`TNAM`), line and
   result scripts as for `SayTo`, when that speaker is in the place (unit
   tested; not tried in the Pip-Boy itself). The result needs the player in
   range of the cell (a trigger sets `bPlayerInRange`).
4. **Dean (Residential District).** Broken, now fixed: the district is a
   dense world of its own, and the graphics lost the device within seconds
   at the usual 5 by 5 squares (Direct3D 12 and Vulkan both). It now loads
   3 by 3 (`DENSE_WORLDS` in `viewer/src/exterior.rs`; `NV_LOAD_RADIUS`
   sets it for any place). **Still missing:** his first greeting is "Have a
   seat" until the player sits in the explosive chair, and the player
   can't sit (scripts see it, the player stays where they are). With
   `NVDLC01DeanDialogue.bFirstConversation` set the real conversation
   plays and finishes "Get to Dean".
5. **Christine (clinic).** Works through recruiting: the silent
   conversation (her lines are stage directions, no voice), her quest to
   stage 100, the "Assemble Your Crew" notice, and Christine following.
   Not played: the clinic basement (turning off the shielded speakers'
   power at the terminal) and the Auto-Doc wing.
6. **All three recruited.** Works: the quest's end messages, the ghost
   people enabled for later, the master quest completing, and "Speak to
   Elijah at the Fountain".
7. **Back at the fountain.** Works: his Act 2 greeting and the replies,
   which start MQ02a to c (their objectives show: the switching station,
   the rooftop, the substation). The companions come into view at the
   fountain.

Also found: Elijah's hologram is a talking activator, so it had no way to
talk (fixed, above); all voice lines now decode their ogg here, like the
music did; one run in about ten stopped with a native crash after the
companions came into view, not repeated.

Not played: fighting the ghost people on the way, the Villa's other
destinations, and Act 2 itself.

## Act 2 groundwork: sitting, ghost people, followers, and a crash

- **The player sits.** E on a chair (or a script's `Activate` on it with
  the player) now puts the player in the nearest sit marker of the
  furniture, facing as the marker does, with a lower eye (0.68 of standing,
  a guess); W, A, S, D or Space gets them up where they stood. Scripts see
  it as before (`GetSitting` 3), so Dean's trigger now starts his real
  greeting: "The Sierra Madre…" (`viewer/src/walk.rs`, `sitting::player_seat`).
- **Ghost people don't die: they go down.** Their script keeps them
  essential (`SetActorRefEssential`); a blow that takes an essential
  person (not the player) to 0 health now leaves them down (`Event::
  KnockedOut`, `GetKnockedState` 2, `IsAnimPlaying` 0, the body limp as a
  dead one) and they get up after 12 s with a quarter of their health
  (`Event::GotUp`). While down and essential they take no harm; once the
  script strips the flag (its semi-dead state) the next blow kills them.
  The 12 s and the quarter are guesses: how the game decides isn't traced
  (`world::combat::hurt`, `advance_down`). In a Villa fight a ghost goes
  down, gets up, and ghosts attack the player.
- **Followers follow.** A teammate with nothing else to do (no package, or a
  sandbox, wander, guard, patrol or find one) keeps within 200 units of the
  player (a guess, `world::ai::TEAMMATE_DISTANCE`); Dead Money's own
  follow packages still win. Teammates come along when the player moves to
  another place (now the traced `world::companions::come_along`, which
  replaced the guessed `bring_teammates`).
- **The intermittent native crash.** It was the graphics running out: every
  square of a town made its own copy of each texture and one material for
  each model, so a 3 by 3 or 5 by 5 spread of the Villa's dense squares held
  thousands of bind groups. Direct3D 12 reports "unable to allocate
  descriptors"; the Vulkan driver here just lost the device or crashed
  (sometimes through a wgpu panic, "snatch lock", a symptom). Standing at
  `NVDLC01PoliceStationBaitREF` crashed 4 of 4 starts and moving there 16 of
  16; no runs crash now, over about 50. Fixed by keeping what places
  upload: textures by what they are, materials by what's in them
  (`TextureCache`, `MaterialCache` in `viewer/src/main.rs`; the Villa's
  squares made 461 materials and 485 more were reused). The squares now
  share one list of lights, the nearest to the player's square, since they
  share materials. The Residential District's smaller load radius isn't
  needed any more. `NV_LOAD_RADIUS` still sets it; `NV_SYNC_RENDER` draws
  in step with the frame (no difference found in about 70 runs).

### Act 2 (MQ02a to c)

Elijah's Act 2 greeting starts the three companion quests and their first
objectives (Dog to the Salida del Sol substation, Dean to the Puesta del Sol
rooftop, Christine to the Puesta del Sol switching station). Played one
companion at a time, Dog first (`characters/dead-money-act2.txt`: Act 1 done,
Elijah's greeting said, Dog hired by the game's own hire script).

**Dog (MQ02a "Fires in the Sky").**

* *Telling him to wait.* The "wait here" and "let's go" replies only set
  `Dog.Waiting` and re-evaluate his packages (`FollowersWait`,
  `FollowersLetsGo`); the waiting itself is the data's guard package
  (`NVDLC01FollowersDogFollowPlayerWAIT`, true while `Waiting` is 1, as
  Boone's `FollowersBooneFollowPlayerWAIT` is). nv-rs treated a teammate's
  guard package as having nothing to do and walked them after the player
  anyway; fixed (branch `claude/teammate-wait`): a guard package now wins.
  Checked in the viewer: with `Waiting` 1 Dog stays while the player moves
  away; with 0 he follows (`NVDLC01FollowersDogFollowPlayerDEFAULT`).
* *Reaching the Gala trigger.* The trigger's script runs when Dog himself
  enters it (it names his reference): it completes "Get Dog to the
  Switching Substation", shows "Get Dog to wait in the Switching Substation"
  and gives Dog the package `NVDLC01DogGalaPositionDialogue`, which walks
  him up to the player and starts his talk. Three engine faults stood in the
  way, each fixed on its own branch: a script that moves the player into
  another worldspace left the old one's triggers loaded
  (`claude/script-cell-grid-lag`); people brought into a place, such as a
  companion who came along, were not on the list the triggers, sight tests
  and E use (`claude/brought-in-talkers`); and a dialogue package's topic
  (`NVDLC01DogGalaGreeting`) has no lines of its own but names two lines
  that stand under the greeting topic, through the topic record's `INFC`
  list (`claude/dialogue-info-links`). With them Dog walks up and says
  "Why are we here? ..." and the reply menu opens.
* *What the endings do.* The result scripts of one ending reply (the
  persuade path's success reply) and of the substation terminal's "Lock
  Gate" item were run as written (`--run`, in the state with Dog at the
  trigger). The reply gave 70 XP, completed the objective, enabled the ghost
  people at the north door and made the autosave; the terminal item took
  the quest to stage 100. Whether Dog is then released from the party and
  walks to his Gala position wasn't seen.
* *Not played:* picking the replies by hand through to an ending (the
  conversation has three endings: persuade, lie, or feed him; only the first
  reply of two paths was picked in the viewer), the feeding
  path (two slabs of ghost harvester remains, stage 40), the terminal's
  screen in the viewer (its items were run as script lines), and Dog walking
  to his Gala position. Not checked either: a second Dog-like actor
  (`01013F66`) shows up at the fountain and follows with Dog's own follow
  package; whether the original also enables it there isn't known.
* *Guessed [G]:* that the INFC lines are used like a topic's own lines (the
  game's topic loader, `00618aa0`, puts them in the topic's list; how a
  conversation picks from that list isn't traced). The `nvinspect dialogue`
  listing of Dog and Sunny Smiles is unchanged by it.

Not played yet: Dean's rooftop (the holograms, the switches), Christine's
switching station (the electrical box, the elevator, the remote terminal),
and the Gala Event itself (MQ02).

### The casino (MQ03)

`characters/dead-money-casino.txt` starts in the casino (`NVDLC01Casino`,
the cell the lobby leads to) as the Lobby's knock-out script leaves the
player after the Gala: MQ02 done, `NVDLC01MQ03a` begun. The interior loads
in about 5 seconds (770 objects, 43 lights, 40,000 collision triangles) and
looks right: the arches, the slot machines, the roulette table, the lights.
The room's scripts run.

* *The power switch.* The casino's electrical switch (`01006392`, in the
  casino) is what MQ03a "Put the Beast Down" calls its first objective. Used
  (`--use 01006392`, the player not near it), its script ran as written: it
  set MQ03a to stage 10 (which starts MQ03b and MQ03c: the Tampico and the
  suites objectives appeared), enabled the slot machines, hologram dealers,
  bartender and the collar radio stations (they came into view), and
  Elijah's intercom said "You woke it up...". The script also disables the
  security holograms and unlocks the lobby doors; those two weren't looked
  at.
* *`NVDLC01MQ03d` "Wake Up the Sierra Madre" is never started.* Nothing in
  Dead Money starts it or sets one of its stages: no script, dialogue result,
  terminal or other record names it (all of the plugin was searched for the
  quest's form). Its two objectives ("Gain access to the Casino electrical
  closet", "Restore power to the rest of the Sierra Madre") are the ones
  MQ03a shows as its stage 10. So it is leftover content: it can't be played
  and the original game never plays it [C: the
  maintainer's note has the console steps to confirm it stays at stage 0].
* *Not played:* walking from the lobby to the switch (the route, the key
  doors, the holograms before the switch), the casino games (slots,
  roulette, blackjack: the menus aren't built), the intercom bark triggers,
  and the later MQ03 steps.

## The clinic basement and the Auto-Doc wing (MQ01c, stages 20 to 40)

Played at the rules level (`world`, with the real data) and in the viewer (`--use`, see below):

* **The basement terminal** (`NVDLC01ClinicBasementTerminal`, the Clinic Power Status terminal). Its
  screens follow its own switch (`bSwitchState`): while the power is on it offers "Disengage Main
  Power"; once off it shows the other status screen. The item's result script disables the clinic's
  parent group, turns the two shielded speakers off (`bActive` 0), plays the power-off sound and moves
  MQ01c from "turn off the power" (objective 20) to "find Christine" (objective 30). Checked headlessly:
  the objective events, the speakers' flags and the screens' conditions all come out as above.
* **Christine's Auto-Doc** (`NVDLC01ClinicAutoDocChristineRef`). Using it runs a short scene: controls
  off, the player moved to `NVDLC01ClinicPlayerMarker`, the pod's Forward group played, the pounding
  object disabled, and Christine given the exit package. In the viewer the pod opens and Christine is
  standing in it, collar and all.
* **The collar rule is the game's own and it bites.** The shielded speakers count the player in their
  radius through `NVDLC01BombCollarQuest.iNumRadii`. A speaker turned off by setting `bActive` to 0
  (which is all the terminal does) never takes the player out of the count if they were inside the
  radius at that moment; only destruction or the speaker's own radio switch do. In the real route the
  player is down in the basement when the speakers go off, so it doesn't matter; with the player
  already inside the radius the collar kills them a few seconds later, which is what nv-rs showed in a
  first test run. Also: a speaker's first run after loading sets `bActive` to 1, so turning it off
  before it has ever run is overwritten; the test marks them initialised first.
* **Not played by hand**: the terminal's screen in the viewer (no unattended way to pick a terminal
  item), and walking the route between the basement and the wing.

**A package's End action** (branch `claude/package-end-action`, from main). Christine's exit package
(`NVDLC01ChristineExitAutoDocPackage`, a Travel package) has an End action, the script
`StartConversation Player`, which makes her talk to the player once she's out. Read from the game's
code: the process runs a package's actions through three virtual functions (slots 358, 359 and 360,
`0x598`, `0x59c`, `0x5a0`), each running the action's script with the person as its reference, then
its topic and idle; slot 360 is the End action, called when the process finishes a package (it sets
the package's `0x400` flag first, then runs the End action, then starts the next package's Begin).
nv-rs now has `PackageActionKind::End`, `GameState::end_script_package` (once for each time a script
package is given), `Runner::package_action` (runs an action's script) and `world::ai::finish_travel`;
the viewer asks for the End action when a script's Travel package has no destination (as "near the
current location") or has arrived, and runs its script. With it the scene plays on: the End script
ran, Christine talked within 2 seconds, the controls came back, MQ01c went to 40 and, through her
conversation, to 100 with Christine recruited.
* **Guessed [G]**: that a Travel package finishes when its person has arrived (or has nowhere to go).
  What finishes a package of each kind isn't traced; the game finishes some kinds in other ways (the
  completion tests for the process's procedure types were only glanced at).
* **Not done**: the Begin and Change scripts still don't run (the same function family runs them; one
  line more in the viewer), and the End action's idle and topic.

**A script's `Activate`** needed no fix. Main already does what the code does (`005b59f0`): a second
argument of 1 runs the target's `OnActivate` block (not past 5 deep) and without it the usual
activation happens at once with the block skipped (two flags are raised around the call; the
activation function then skips the script). Dead Money uses it for hiring and firing companions
(`NVDLC01HireTriggerREF.Activate NVDLC01ChristineREF 1`). My test's `Activate Player` without the 1
was correctly not running the block. One more thing the handler does: a call on the player itself
(`player.Activate X`) is refused with the warning "SCRIPTS: Never have the player character activate
something in a script very Bad"; no script in the base game or Dead Money does that, so it isn't built.

**Test flag**: `--use REF` presses E on an object once the place is loaded (branch
`claude/viewer-use-flag`, from main). Used for the scene above, after `--run` lines that set MQ01c's
stages and the speakers' flags.

## Open questions

* Which DLCs are installed in the maintainer's Data folder (the data pass's `info`)?
* What does the test character start with? **Answered for now (D2)**: level 20, the player
  record's own S.P.E.C.I.A.L. and skills, modest gear (`characters/dead-money-entry.txt`).
  Change the file if a different arrival is wanted.
* Where are the DLC bits at `011c3f2c` read, and does any rule depend on them?
