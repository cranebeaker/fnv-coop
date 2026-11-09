# Co-op specs (M0 to M3)

How each part must behave, with the exe addresses it relies on. Addresses are FalloutNV.exe 1.4.0.525 and
were checked against the translation in this tree and against fnv_decomp.

## S0: the base (M0)

- Branch `coop-next`, cut from upstream nv-rs main `815b187`. fnv-coop's `main` shares no history with
  upstream, so this branch has unrelated history by design.
- `crates/coop-core`, `coop-net`, `coop-policy`, `coop-voice` and `nv-launcher` are copied byte for byte
  from fnv-coop `main` (2107c9b). Their dependencies name no removed crate (checked: no `world::`,
  `testdata` or `esm::` in their code). Four doc comments name `world::` (`coop-core/src/env.rs`,
  `coop-core/src/interact/mod.rs`, `nv-launcher/src/party.rs` twice); they are updated when `coop-model`
  exists.
- They and `crates/coop-engine` are added to the root `members`.
- `Cargo.lock` must be regenerated on a machine with crates.io access (`cargo generate-lockfile` or one
  `cargo build`), because CI runs `--locked`. New third-party crates: `serde`, `postcard`, `lz4_flex`,
  `xxhash-rust`, `thiserror`, `proptest`, `rand`, `serde_json`, and optionally `libloading`, `opus`,
  `cpal` and `rubato`. Their licence notices go to `distribution/` as fnv-coop's did.
- `nv-launcher` still builds the old Bevy viewer's command line. Until M5 it is left alone (it builds and
  its tests pass). M5 changes it to start `nv-viewer` with `NV_COOP` and the session's settings.

## S1: the `Coop` extension owner (M1)

In `crates/engine-core/src/extend.rs`:

- `Owner::Coop` is added. Its rank is 3, so it runs after `Row`, `Nvse` and `Jip`. Co-op sees what the game
  and the extenders did, and is the outermost link of a wrap chain.
- `WRAPS` gets one row: `0x0084_a690` (`BGSSaveLoadGame::AddChange`), owners `[Coop]`, step `"C-1"`.
  The existing row `0x005e_0d20` (`ScriptRunner::Run`) gets `Coop` after `Nvse` for the guest gate (S5).
- The table checks' messages name the new order (rows, nvse, jip, coop).
- Nothing else in the engine changes. With co-op not installed, each address calls what it called before,
  and the dispatch check (`Engine::unlisted_wraps`, used by `nvse`'s tests and `extend.rs`'s) still holds.

## S2: `crates/coop-engine` (M1)

It depends on `engine-core` only, as `nvse` does, and does not depend on the game data. Its unit tests use
their own `engine_core::assemble!()`.

### `addr` module

Every exe address the bridge uses lives in one place, each with its name and the translation that defines
it:

| Name | Address | What |
| --- | --- | --- |
| `LOOKUP_FORM` | `004839c0` | `TESForm::GetFormByID(id)`: the form, or 0 |
| `QUEST_SET_STAGE_DONE` | `0060d510` | `TESQuest::SetStageDone(quest, stage)`: false for an unknown stage |
| `QUEST_GET_CURRENT_STAGE` | `0060d700` | `TESQuest::GetCurrentStage(quest)`: the byte at +0x60 |
| `SAVE_LOAD_ADD_CHANGE` | `0084a690` | `BGSSaveLoadGame::AddChange(this, form, flags, force)` |
| `SAVE_LOAD_RECORDS_CHANGE` | `0084a620` | whether the save records a change of the form `(this, form, force)` (only reads) |
| `SCRIPT_RUNNER_RUN` | `005e0d20` | `ScriptRunner::Run(this, script, reference, locals, container, ...)` |
| `SCRIPT_IS_QUEST_OFFSET` | +0x28 | `Script::m_header` (+0x18) `.bIsQuestScript` (+0x10) |
| `SCRIPT_OWNER_QUEST_OFFSET` | +0x40 | `Script::pOwnerQuest` |
| `FORM_ID_OFFSET` | +0x0c | `TESForm` refID |
| `FORM_TYPE_OFFSET` | +0x04 | `TESForm` type byte |
| `GLOBAL_VALUE_OFFSET` | +0x24 | `TESGlobal::fValue` |
| `FORM_TYPE_GLOBAL` | 6 | `GLOB` |
| `FORM_TYPE_QUEST` | 0x47 | `QUST` |

### Change journal

- `Journal` is the bridge's game-wide state (a shared extension slot, as `nvse` keeps its own), so changes
  made on any game thread land in one place.
- `armed: bool`. Changes are recorded only while it is set. The session arms it on the host once the world
  is loaded. Loading a save makes thousands of changes that are not news.
- `changes: BTreeMap<u32, (u8, u32, bool)>`: form id to its form type, the union of the flags added
  since the last drain, and whether any was forced (`ForceChange`). A form changed twice in a tick is
  sent once, with both flags.
- The state lives in a lock-free shared slot (`Engine::ext_free`): atomics for `installed` and `armed`
  and a mutex around the map, held only while an entry is added or the map is taken. Any game thread
  may record, and the viewer's thread may drain between frames, without engine-core's slot lock.
- The wrap at `SAVE_LOAD_ADD_CHANGE` calls the next link first (the game's own behaviour, unchanged and
  its return passed back). Then, if armed, the form word is not 0, and the game recorded the change
  (`SAVE_LOAD_RECORDS_CHANGE` says yes, the same test the call made), it reads the form's id at +0x0c
  and its type at +0x04 and records `(id, type, flags, forced)`.
- The flag `0x80000000` means a quest's stages only on a quest. On a cell it is the seen data, on a
  topic the said-once, on a package the never-run (`tessaveloadgame.rs`). Readers match the type too.
- `drain()` hands back the changes in form-id order and empties the journal.
- Out of scope here: deciding whether a change is shared or per-player. That is `coop-policy`'s job, on the
  session side.

### Form, quest and global access

- `form(e, id) -> Option<u32>`: `LOOKUP_FORM`, `None` for 0.
- `quest_stage(e, id) -> Option<u8>`: the form must be a quest (type byte `0x47`); calls
  `QUEST_GET_CURRENT_STAGE`.
- `set_quest_stage(e, id, stage) -> bool`: the form must be a quest; calls `QUEST_SET_STAGE_DONE`, so the
  stage's own result scripts and the change flag happen exactly as when a script runs `SetStage`. A guest
  applying the host's stage uses this only once M3 has gated the guest's quest scripts. Until then it is
  the host's call.
- `global_value(e, id) -> Option<f32>` and `set_global_value(e, id, v) -> bool`: the form must be a
  global (type byte 6); read or write `fValue` at +0x24. Writing does not flag a change. The save keeps
  every global in its own block whatever the flags say.
- `GlobalWatch`: the ids of every global the session tracks and their last values. `diff(e)` returns
  `(id, new value)` for each one whose bits changed, comparing bits so that -0.0 and NaN are exact.

### Install

- `install(e)`: once per engine (a second call does nothing). Wraps `SAVE_LOAD_ADD_CHANGE` as
  `Owner::Coop` and makes `Journal` a shared slot. Must run before real threads start (`Engine::wrap`
  asserts it).
- `wanted()`: true when the environment variable `NV_COOP` is set and not `0`.

## S3: the host's install point (M1)

In `Host::start_with`, after `nvse::install(&mut engine)`: `if coop_engine::wanted() { coop_engine::install(&mut engine) }`.
With `NV_COOP=journal`, nv-viewer arms the journal after the first main frame and, once a second, prints
each quest whose stages changed and a count of the other changed forms (`coop_engine::diag`). This is the PC check for M1, and is removed once M3's session drives the journal.

## S4: engine backend for conformance (M2)

- New crate `coop-model`: the types `coop-session` and `coop-headless` take from `world::coop` (listed by
  Planner B: `PartyRules`, `ShadowPlan`, `PlayerShadow`, `apply::Ctx`, `RulesBridge`), moved with their
  tests. `coop-session` and `coop-headless` then depend on `coop-model` instead of `world`.
- `EngineBackend::peer` starts an engine with `host::Host::start_with` on the given `Data`, steps the main
  frames until the main menu, loads the test save, and hands back an `EnginePeer` whose methods are S2's
  calls. `set_stage_as(slot, ...)` uses the per-player rows (S6) once they exist; until then it returns
  false and its rows stay PENDING. It never reports a pass.

## S5: the story channel and guest gating (M3)

- On the host, each network tick: drain the journal; for each change of a quest (type `0x47`) with the
  `0x80000000` flag, send `(quest id, current stage, stage-done bits)`; send the globals watch's diff. On
  the guest, apply each through S2 inside `gate::applying`.
- Guest gating (`coop_engine::gate`, implemented): a wrap of `ScriptRunner::Run` (`005e0d20`, `Coop`
  after `Nvse`) skips, on a guest, (a) every run while the guest applies the host's state
  (`gate::applying`, nested, unwind-safe), because `SetStageDone` reaches the stage items' result scripts
  and their effects arrive from the host, and (b) the story's scripts at any time: a quest script
  (`bIsQuestScript`) or one with an owner quest (`pOwnerQuest`). A skipped run answers 0 (no references
  changed). The guest's own object and effect scripts run. Skips are counted (`gate::skipped`) for M3's
  check that no story script ran on the guest. Whether every result script carries an owner quest is
  checked on the PC in M3: a story script reaching the guest's runner without one shows as a quest stage
  or global changing on the guest without a host record (a hash drift).
- Quest records (`coop_engine::quest`, implemented): `(form, current stage, flags, done stage numbers)`,
  read from the quest's flags (+0x3c), stage list (+0x44: nodes of item and next, both words zero is
  empty; a stage's number at +0, done byte at +1) and current stage (+0x60). A guest applies a record
  inside `gate::applying`: each stage done on the host and not on the guest through `SetStageDone`, then
  the current stage and flags set to the host's, and a stage done only on the guest undone. A record
  naming a stage the guest's quest lacks (different data) is refused whole. Applying a record twice
  changes nothing.
- Which per-player quests run on their player's machine instead is `coop-policy`'s table (S6).
- Every message carries the host's tick. Guests ack, and coop-core's existing reliability and snapshot
  rules apply unchanged.

## S6: per-player state (M3 onward)

The game has one player. The other players are actors that the host's game treats as followers with their
own inventory, skills and perks, and each guest's machine plays its own character as the player. A
per-player quest (from the policy table) runs on the machine of the player it belongs to. Its records go
to the host for the party save, never into the shared world.
