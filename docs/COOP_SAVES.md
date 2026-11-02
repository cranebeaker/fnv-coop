# Co-op saves and persistence

Status: code and tests for everything below the viewer; the viewer glue is written but not built or run
(see "Not verified"). Design source: the fnv-coop party-save design, adapted to nv-rs's own text saves.

## What is saved

A **party save** is a folder next to a world save: `nv-rs-quicksave.txt` gets `nv-rs-quicksave.txt.partysave/`.

```
party.txt              nv-rs party save 1, campaign, host, seq, saved (unix), world length+hash, one guest= line per guest
world.txt              the host's world save (world::save::save): shared state + the host player's character
guest-<key>.char       one sidecar per guest: that guest's character (a shadow blob), hashed
```

* The **campaign id** is 16 hex digits, never zero. A new session is a new campaign; `--coop-load` continues
  the save's campaign. It is the same id the launcher uses for `saves/<campaign>/`.
* Each guest has a **persistent player id** (16 hex digits), made once by the launcher and kept in
  `<launcher home>/player-key.txt`, passed as `--coop-player-key`. Slots are not stable between sessions;
  ids are.
* The folder is written as **one unit**: all files go into a sibling temporary folder, then the folders are
  swapped by rename. A failed write (any step) removes the temporary folder and leaves the previous save
  untouched. A crash between the two renames is recovered on load from the `.old` folder.
* Everything is **versioned**: `party.txt` carries the format number (a newer one is refused, not guessed at),
  sidecars and character files carry their own.
* Load is strict about the world and forgiving about guests: a damaged world file refuses the save; a missing
  or corrupted sidecar costs only that guest its character (it gets a fresh one), reported as a problem.
* **No synthetic forms are ever saved.** Puppets (`0xFE000000..`) and shadow forms (`0xFC000000..`) are removed
  from the world text and from every character by `PartySave::scrub`, which the writer and the loader both run.
  Dynamic references (`0xFF000001..`) are real state and stay.

Code: `crates/world/src/coop/party_save.rs` (format), `party_char.rs` (character files, offers, load glue),
`crates/coop-session/src/party.rs` (protocol).

## The host save order

When the host saves (F5, the menu, a script's autosave, a system save), `scripts::save_and_load` writes the
world file as before, and on success pushes the path to `coop_party::PartyBridge`. `coop_party::pump` calls
`Session::party_begin_save(gs, dir, world_text, 0)`:

1. refused when the policy holds saving (`GameState::coop_save_blocked`: a scene, barrier); `save_to_path`
   already refused the world write in that case, so no party save starts either;
2. every connected guest gets `Order{seq}`; each saves its own character (`CharacterFile`: shadow, player id,
   campaign, baseline of shared quest stages), keeps a **local copy** in its `--coop-character` file, and
   sends it to the host in chunks. A guest held by a scene retries for 20 s, then refuses;
3. the host writes the party folder when all answered or after 4 s. A guest that did not answer is saved from
   **the host's own copy** of its character (marked `from_guest=0` in the manifest);
4. characters of guests that are **offline** in this session are carried forward unchanged, so a save made
   while a friend is away does not lose their character;
5. `HostSaved` / `HostSaveFailed` events become HUD lines; guests are told `Stored{ok}`.

One save at a time (`party_begin_save` refuses while one is being collected).

## Continue, rejoin, late joiners

* **Continue**: `--coop-host --coop-load DIR` starts from `DIR/world.txt` (the place, the host's character,
  the shared state), takes the save's campaign and keeps its guests' sidecars.
* **Rejoin**: a guest says `Hello{player id}` after connecting. If the host has a sidecar for that id (loaded,
  or earlier in this session under another slot) the character is installed into the guest's slot and
  replicates to the guest like any shadow. Slots may differ from the saved session; ids decide.
* **Unmatched joiner**: gets a fresh character (level 1, the record's starting kit).
* **Late joiner with a saved character**: if the guest has a character file (`--coop-character` exists) and the
  host has none for its id, the guest offers it once. The host accepts it under its own rules
  (`sanitize_offer`): the character (inventory, skills, perks, level, name, personal quests) is the player's;
  everything the campaign shares wins: rows of quests the campaign shares are dropped, synthetic forms are
  removed, the old position is cleared (the host's place wins). The reply carries how many shared quest
  stages in the file's baseline differ from the host's (`baseline_diff`; informational).

## F5 / F9 / autosave in co-op

| Who | F5 / menu Save | F9 / menu Load | Autosave |
|---|---|---|---|
| host | world save, then a party save order | world load, then the party folder's sidecars are re-applied and guests told `Reloaded` | scripted/system saves go through the same writer and the same order |
| guest | saves **only its character** to its file (no host involved) | restores **only its character** from its file: the host sanitizes and installs it (shared state still wins) | none of its own; it saves on the host's order |

* A guest never writes or loads a world file (`scripts::save_and_load` diverts to the bridge).
* A host load keeps the session: `carry_coop` moves authority, the policy gate, the parked characters and the
  live facts to the loaded state, forces a full diff on the next collect, and re-registers the clock globals
  (they join the coarse export grid only when the clock first moves).
* An offer for another campaign is refused (a file with campaign 0, saved outside any campaign, is accepted).

## `--coop-host --load-fos`

An original `.fos` can start a hosted campaign: the host's character and the shared state are the save's
(`world::fos_import`), guests start with fresh characters, and the first party save of the session writes the
usual party folder. Limits are those of the importer (`world::fos_import::GAPS`, docs/FOS_SAVES.md): kinds of
state it does not import are absent for everyone. A guest cannot use `--load-fos` (it takes the host's game).
Tested by `fos_import::tests::an_original_save_starts_a_coop_campaign`.

## Launcher

`crates/nv-launcher/src/party.rs`: Continue shows the campaign's roster (names, levels) and last-save time and
adds `--coop-load <newest party folder>` for a continuing host; a guest launch adds `--coop-player-key` and
`--coop-character <player folder>/character.char`. The saves list prints one `party save:` line per folder.
A folder from a newer build is listed and not loaded. The launcher only reads `party.txt` (its first lines are
pinned by a test on both sides) and never deletes party folders.

## What a text save does not hold

The world text does not hold NPC runtime (paths, who is mid-package or seated), dialogue and menu state, or
scene contents (re-derived from the cell and `disabled`); see research/06-nvrs-integration-map.md, section 7.
After a load, NPCs rest where `positions`/`furniture`/`script_packages` put them. The per-player/shared split
is the shadow bank's: the world text has the host's character; guests' characters are the sidecars.

## Tests

* `cargo test -p world --lib coop::party` (format: round trip, atomic write failure at every step, crash
  recovery, version mismatch, missing and corrupted sidecar, damaged world, scrub, manifest lines the launcher
  reads, character file, sanitize, baseline diff, carry);
* `cargo test -p world --test coop_save` (host + 2 guests over the simulated transport, perfect and bad_wifi:
  save, drop everything, load, rejoin with the slots swapped; shared hash, per-player data, per-player quests,
  late joiner, leak test, scene save lock);
* `cargo test -p coop-session --test party_save` (the same through `Session`: the order, the guests' answers
  and local copies, rejoin by id, silent guest, offline guest kept, offers, refusals);
* `cargo test -p coop-headless --test party` (scenarios `live_party_save_roundtrip`,
  `live_guest_rejoin_own_character`, `live_late_joiner_with_save` with `twin_` mutation twins in
  `tools/coop-run/scenarios`, headless only, and a soak over all seven chaos profiles and three seeds);
* `cargo test -p nv-launcher`, `python -m unittest` in `tools/coop-run`, `parses_the_party_save_options` in
  `viewer/src/args.rs`.

## Not verified

* The viewer glue (`viewer/src/coop_party.rs` and the hooks in `scripts.rs`, `coop.rs`, `main.rs`, `args.rs`)
  was written against the existing code but the viewer was not built or run here.
* A real two-process session with F5/F9 on real game data.
* Party-save size with real inventories over a real link (chunks are 400 bytes on the reliable channel).
