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

## S7: the session on the engine (implemented; docs/coop/DECISIONS.md D2)

`crates/coop-engine-session`, a thin driver over coop-core's `HostSync` and `GuestSync`:

- **Fields.** Each quest is one `Field::Custom` (tag `0x5152`): stage, flags, a 256-bit done-stage
  set, then its objectives (count; number of variable length, one byte below 128; state as 1 byte). A delta can never split it.
  A record that cannot be carried exactly (more than 255 objectives or 512 bytes, a state above
  255) is not sent, counted, and named once in the host's log. Each quest script variable is a `Field::ScriptVar` (index = the
  variable's id, the `f64`'s bits). Each story global is a `Field::Global`.
- **Host.** `start` arms the journal and writes every quest of the load order (the viewer lists
  the plugins' `QUST` records) and every story global, so a join snapshot holds the whole story
  whatever either game did before. Each step: the journal's quest changes are read again
  (record and variables), and the globals' watch compares values (form addresses cached after the
  first lookup).
- **Guest.** Gated from the first frame of a join (`gate_guest`). Each step applies the replica's
  entries newer than the applied mark, or all of them after a snapshot or repair. Quests go through
  `quest::apply`; variables and globals are written in place; all inside `gate::applying`. Every 20
  steps the game is read back (`echo`) and drifted entries are applied again (`heal`). `rejoin`
  resumes from the replica after the transport joins again; a host with a new session answers
  with a fresh join and a snapshot.
- **Viewer.** `NV_COOP=host`, `host:PORT` or `join:CODE`. The globals the clock moves on every
  machine (coop-core's volatile keys and GameDay, GameMonth, GameYear) are not replicated; time is
  shared separately (later milestone). Objective changes are written as state; the "objective
  added" HUD line is presentation, for M6.
- **Known limits.** A stage undone on a guest (drift) clears its done byte but not its items' log
  dates, so the game's log may not show it again if redone; a guest's game only drifts from the
  host's through its own scripts, and the gate stops the story's.

**Verified here** (no game): 8 loopback tests on two and three games joined over coop-core's
loopback and chaos transports, with one key per delta, a late join, a bad network, rejoin, a host
restarting its session, drift healing and data that differ; the guest's game is read back after
every step and must hold, for every quest, a record the host really had, and no story script may
run on a guest while its own quest scripts are tried every frame. 2 tests over coop-net's real UDP
endpoint on 127.0.0.1. Each guarded behaviour was removed once to see the tests fail.

**Not verified here:** the real translated functions behind the stand-ins, the real load order,
and two real games (docs/coop/TESTING.md).

## S8: the live ladder (implemented; docs/coop/DECISIONS.md D3)

Players in the world, one logged rung at a time (`coop_engine::live`, `coop_engine_session::ladder`):

| Rung | What | Game calls |
| --- | --- | --- |
| 1 | This game's player is read: parent cell (+0x40, its form id), position (+0x30), heading (+0x2c), 3D (+0x64) | none (the pointer at `011dea3c`) |
| 2 | Another player's state arrives (coop-core's avatar stream, 20 Hz, interpolated) | none |
| 3 | A guest's player, once in the world itself (a cell and its 3D), is moved to the host's place; again, at most every 5 s, when the host is in another space. A space is an interior cell, or an exterior's world space | a 0x34-byte position request (`operator new` `00401000`, constructor `00456540`) given to `PlayerCharacter::RequestPositionPlayer` `0093be30`, which the game carries out in its own frame; not while the player's slot `0x22c` says it cannot be moved |
| 4 | A remote player in this player's cell gets an avatar at this player's place; its AI off (`bProcessMe` +0xbc = 0, `EndMovement` `0087faa0`, `SetGhost(1)` `008acf10`) | `PlaceAtMe` worker `005c4b30` |
| 5 | The avatar is moved every frame; an AI switched back on is switched off and counted | `SetPosBase` `005c0740` x3, `SetAngleBase` `005c0b10` |

- Lines: `COOP-RUNG n entering: ...` at once, before the rung's first call into the game, and
  `COOP-RUNG n PASS|FAIL ...` once. Rung 5 passes after 100 moves.
- `NV_COOP_LIVE=log|warp|avatar|off` caps the ladder (default `avatar`); `NV_COOP_AVATAR=<hex>` is the
  avatar's base form (default 7, the player's own NPC).
- The host's own state goes to every guest whatever their cell (a guest needs it to come); guests'
  states are relayed by coop-core's interest rules.
- No avatar is placed or moved while this player's own move to another cell is under way. An avatar is
  removed (`TESObjectREFR::Disable` `00574400`) when its player leaves, is in another cell, or is not in
  the world; it is kept by form id and looked up each frame, never by pointer.
- A failed rung is reported once and not retried in that run (a failed avatar), or reported and retried on
  the next host cell change (a failed warp).
- First version limits: no own guest character (the guest's game is the default courier; the opening is
  gated), a sliding figure without animation, no avatar across cells, avatars are not removed before a
  save (nv-viewer keeps saves in memory unless `NV_SAVE_OUT` is set).

**Verified here:** 8 tests on two games in one process: the whole ladder, a guest still at its menu, each
cap, a refused avatar, a cell the guest lacks, an avatar woken by the game, following the host to another
cell, a guest leaving; the story channel's revision, echo, desyncs and guest script count are unchanged by
all of it. Three behaviours were removed once each to see the tests fail.

**PC run 2026-11-10 (first ladder):** rungs 1 and 2 passed in the real game. Rung 3 then used
`PlayerCharacter::CenterOnCell` on a guest still at its main menu; the guest's game stayed on its loading
screen. Rung 3 now uses the game's own request path and waits for the guest to be in the world.

**Not verified here:** the game calls of rungs 3 to 5 (the PC run of docs/coop/TESTING.md).

## S9: each player's own quests (implemented; first step of M7)

**PC run 2026-11-10 (second ladder):** a guest that pressed New Game stayed out of the world (a black screen
with the HUD; its player in no cell): the opening is a quest script and the gate held it back. The gate's
log showed that in the real game every quest script names its owner quest (`Script::pOwnerQuest`), the
opening's among them (`00104c00` of VCG01 `00104c1c`).

- The rules (`crates/coop-policy/data/policy.json`) class 83 quests as per-player: the opening and character
  creation (VCG00, VCG01, CGTutorial), controller and achievement bookkeeping, and others each player goes
  through for themself. nv-viewer resolves them for the load order (the plugin's index and the row's local
  id).
- On a guest their scripts run (`gate::set_own_quests`): a story script whose owner quest is one of them is
  not held back, except while the host's state is applied. So a guest that presses New Game goes through its
  own opening and makes its own character.
- The host does not replicate them (they are left out of the session's quests, at the start and from the
  journal), so one player's opening never overwrites another's.
- Every entry a guest heals is named once in its log (`co-op guest healed: ...`): shared state a guest's own
  scripts keep changing shows up there.

**Verified here:** the gate's unit test (own runs, others held back, nothing while applying) and a two-game
test (the host's own quest does not reach the guest, the guest's own quest script runs every frame, the
shared quest arrives; removing the host's filter fails it); the rules' rows resolve for a load order with
DLCs. **Not verified here:** that the guest's opening then plays through in the real game.

## S10: what the first outdoor run taught (implemented; PC run 3, 2026-11-10)

- **Objectives go through the game.** `quest::apply` sets an objective whose state differs with
  `BGSQuestObjective::SetState` (`005ec5d0`), the body behind `SetObjectiveDisplayed` and
  `SetObjectiveCompleted`. That function adds the objective to the player's objective list
  (`00952a20`; the list at player +0x6bc), and the Pip-Boy's quest page lists what is in that list. A
  state that is only written leaves the quest out of the Pip-Boy. The guest's heal also puts back any
  shown objective of a shared quest that is missing from the list (`quest::relist`).
- **The clock is shared apart from the story.** GameHour, GameDaysPassed, GameDay, GameMonth and
  GameYear move by themselves on every machine, so they are not story globals. The host writes them about
  once a second as `Field::Custom` with `CLOCK_TAG`; a guest sets its own to the last value it holds, again
  every second (a host whose clock stands still sends nothing new). `echo` and the heal ignore the tag.
  Weather is not shared yet.
- **An avatar belongs to the space it was placed in.** Moving a reference changes its position, not its
  cell. The ladder records each avatar's space and places it anew when the player it stands for is in
  another one, including when this game's player followed in the same frame.

## S11: deaths, guest-local scripts, per-player helpers (implemented; docs/coop/DECISIONS.md D4)

- `coop_engine::actors`: `actor`, `dead` (life state 1 or 2 at +0x108), `kill` (`Actor::Kill`, `0089d900`,
  no attacker), `shareable` (a placed actor's id, not the player's).
- `coop_engine::gate::set_own_scripts`: story scripts a guest runs itself whatever their quest (the rules'
  `guest_local`), next to `set_own_quests`.
- Session: `LIFE_TAG` (`Field::Custom`, one byte). The host writes a death from its journal and enacts
  guests' claims (`claims_enacted`, `claims_dropped`, `claim_notes`). The guest arms its own journal, sends
  a claim for each actor that died in its game and is not dead in the replica (`claims_sent`), applies a
  death inside the apply scope, and its heal kills an actor that was not in the game when the death came.
  `echo` reports only a death the guest's game lacks.
- Rules: `VPreorder*` and `VGenericTimer` are per-player rows (491 rows).
- References (PC run 5): a placed reference whose form flags changed in the host's game (`Enable`,
  `Disable`; change flag 1) is written as `Field::RefDisabled`; the guest sets its own through the game's
  `Enable` (`00573f40`) and `Disable` (`00574400`) inside the apply scope, and its heal keeps it there.

## S12: the drift census (implemented; docs/coop/DECISIONS.md D5, rung 1)

- `gate::Cause` (script id, owner quest, quest-script flag, reference) and `gate::cause`: the innermost
  script running now, none while the host's state is applied. The gate's `Run` wrap keeps the stack.
- `journal::Change::cause`: the script running when the form was first changed.
- The guest keeps the last cause per form; the heal writes one `COOP-DRIFT` line per distinct (form, kind)
  it puts back, with that cause or "cause unknown" (`drift_named`, `drift_unnamed` in the status line).

## S13: host commands, the first: evaluate your package (implemented; D5's host commands)

- PC run 6: after her dialogue Sunny stood still. Stage 10 of VCG02 says `SunnyREF.evp` in its result
  script, which runs on the host only; the guest's copy was never told.
- `coop_engine::commands`: a wrap of `Actor::EvaluatePackage` (`008a6ce0`, WRAPS step C-4) notes, on a host
  whose journal is armed, each evaluation asked for while a script runs (`gate::cause`) of an actor both
  games know. The AI's own evaluations are not noted.
- Session: `EVALUATE_TAG` (`Field::Custom`, a count). The guest tells its copy the same, inside the apply
  scope; `echo` and the heal leave the tag alone.
- The host logs each shared quest's stage change (`stage_notes`), so a run shows whether the story moved.
- The census of run 6 (D5 condition 1): VCG02's drift was named (the guest's own generic timer, event 3)
  and VMQ01's (a result script of quest 00104c1c on the player). Script variables had no cause: the journal
  does not see them, as B's planner expected, so the heal's diff stays their capture.

## S14: the rest of the world (implemented on stand-ins; docs/coop/DECISIONS.md D1, option C)

Nothing in this section has run in the real game. Its tests use stand-ins for the game's functions
(`coop_engine::worldkit`).

### The changed-form stream (`coop_engine::changes`, `coop_engine_session::world`)

- **A record** is what the game writes for one form into a save: the nine-byte form header (the
  form's index, change flags, save type, version) and the data. `capture` makes it as
  `BGSSaveLoadGame::SaveGame` and `UnloadForm` do for each changed form: a `BGSSaveFormBuffer`
  (`008659c0`), its header (`00865ad0`), the form (`0050f9c0`), the form's `CheckSaveGame` (slot
  0x80), the initial data (`0084eb80`) and the form's `SaveGame` (slot 0x54). It is written with
  every change flag the game's save holds for the form (`008456e0`), so it is the whole changed
  form.
- **Loading** is what `BGSSaveLoadGame::LoadForm` (`008495d0`) does when the game loads a form's
  stored record while playing: a `BGSLoadFormBuffer` (`008643b0`) with the record's bytes, the
  form set (`007037c0`), the header's flags (`00428130`), the form marked (`0046a010`), the
  initial data (`0084f330`), the form's `LoadGame` (slot 0x5c), the data freed, the buffer handed
  to the game's form buffer array (`007cb2e0`), then the game's unload pass (`008492b0`, which
  runs `InitLoadGame` and `FinishLoadGame` and deletes the buffer). Not while a load runs
  (`0042ce10`). On a guest all of it is inside the script gate's apply scope. Nothing is reverted
  first.
- **The numbering.** A record names other forms by three-byte indices into a
  `BGSSaveLoadFormIDMap`, through the map the game object points at (+0x08), and every game
  numbers for itself. Co-op makes one map of its own per game (`00846b60`) and puts it in the game
  object's place only while a record is made or loaded. The host's grows as it captures; its ids
  travel in index order (`NUMBERS_TAG`, 120 ids a field); a guest enters them in the same order
  (`00846c90`). The game's own save numbering is never used or changed. A record says how many ids
  it needs and waits on a guest until they are there.
- **What travels** (`changes::class_of`, in D1's order): a placed object (`REFR`) whose form
  flags (enabled or not), place, lock or open state changed; a placed object whose contents
  changed; a faction; a reputation; a note read; a dialogue response said once. Not: actors, cells,
  quests (their own channel), objects only pushed about by physics, forms made while playing (ids
  from `ff000000`). A record that does not fit one field (512 bytes) is not sent, counted and named
  once in the host's log.
- **On a guest** a record waits while its form is not in the game or the game is loading. After a
  load the guest makes its own record of the form with the same flags and counts a difference
  (`records_differ`, named once): on a PC that number says, per kind of form, whether its live load
  holds. A form of another type than the record's is refused.
- **D1's condition 1.** No kind of form has its live-load proof on a PC yet. `NV_COOP_WORLD=doors`
  runs without records, `off` without any of this section.

### Doors and locks (`coop_engine::world`, `coop_engine_session::world`)

- A door is read with `BGSOpenCloseForm::GetOpenState` (`0047b250`) and moved with `SetOpenState`
  (`0047aec0`, not immediate). A door opened gets its change flag through `AddChange`; shut again
  it loses the flag through `RemoveChange`, which does not pass the journal's wrap. So the host
  reads every door it has seen change once a step and writes its state as `DOOR_TAG`.
- A lock (`REFR_LOCK`: level +0x00, flags +0x08, tries +0x0c) is read from the reference's own
  lock (`00569140`) and set with `TESObjectREFR::Lock` (`00569250`) and `UnLock` (`005692a0`),
  then the other values in place and `AddLockChange` (`005789b0`). The host writes it as `LOCK_TAG`
  when the journal names the lock's change. A lock picked, a lock opened from a terminal and a
  lock's failed tries (a broken lock is one whose tries reached the game's limit) are all in it.
- A guest's own door goes to the host as coop-core's `Act::World(Shared::Door)`; the host's game
  moves the door and the state comes back to everyone. A guest's own lock change goes as a claim
  (as a death does, D4); the host's game sets the lock. A guest's change the host does not take up
  (the object is not in the host's game) is put back after 120 steps.
- The host refuses every other interaction request for now, so a guest is never left waiting.

**Verified here** (no game): `coop-engine` 44 tests, among them a round trip per kind of form
(host record, guest load, guest record, byte for byte) with `crates/fos` decoding the reference's
place, the faction, the reputation, the note and the response; `coop-engine-session` 9 tests on two
and three test games over loopback. Twins: a record loaded with another numbering names other
forms; a load on a game that is not a guest runs the object's script; with doors not shared a door
shut on the host stays open on the guest.

**Not verified here:** every game function above (the stand-ins only do what co-op relies on), what
a placed object's real record holds after its initial data, that a live `LoadGame` shows on screen
(a door swinging, a container's menu), and two real games.

**Not done:** a guest's take from a container and a guest's trade do not reach the host (loot can
be taken twice; each game has its own trader stock and caps); terminals' own state beyond their
lock; references moved to another cell; records above 512 bytes.

## S15: shared fights (implemented with unit tests; not yet run on a PC)

The host's game is the authority for actors that are not players (`coop_engine::fight`,
`coop_engine_session::fight`). `NV_COOP_FIGHT=state|damage|threat|revive|off` caps it (default `revive`);
it needs the live ladder. With it off, or with co-op off, every game runs its own fights as before.

| Part | What | Game facts (each from this tree's translation) |
| --- | --- | --- |
| State | About 20 times a second the host reads the actors its game runs around its player and tracks each placed one in coop-core's actor ledger (`ActorHost`), pinned to the host as owner. The ledger announces it to the guests in that space and relays place, heading, health, life, combat flag and combat target. A guest holds each such actor that is in its own space: AI off (`bProcessMe`), moved with `SetPosBase`/`SetAngleBase`, health set, killed inside the apply scope when the host says it died. An actor the host stops running goes back to the guest's game | the process lists' close-to-player array (`011e0e80` +0x88, count +0x150, 50 entries; `processlists.rs`); health through the `ActorValueOwner` at +0xa4, virtual `0x0c`, value `0x10`; the in-combat test `00493bb0`; the combat target, virtual `0x42c`; a value change, virtual `0x3ac` |
| Damage | A wrap of `Actor::HitMe` decides whose game applies a weapon hit. A guest's hit on a held actor is kept from its own copy and sent as `ActorMsg::Hit`; the ledger judges it and the host's game applies it, credited to the guest's avatar; a kill is `Actor::Kill`. The host's actors' hits on an avatar are kept from the avatar and become `PlayerHurt` through coop-core's vitals; the guest's game applies it to its player. A guest's game applies no hit of a held actor on anyone | `Actor::HitMe` `0089a760` and `HitData` (+0x00 aggressor, +0x10 limb, +0x14 health damage); the damage applier, virtual `0x338` (`fn_0089d6f0`); `Actor::GetEssential` `0087f3d0` |
| Threat | Twice a second coop-core's `ThreatDirector` picks the player each of the host's actors that is fighting a player attends to; the host's game is asked to start that combat. Avatars on the host stop being ghosts | the process's start of combat, virtual `0x33c`, with the words the `StartCombat` command passes; `Actor::pCurrentProcess` +0x68; the alarmed test, virtual `0x304` |
| Downed and revive | A wrap of `Actor::Kill` holds the kill of the game's own player in a session. The player is left with 1 health and no controls and reports down; coop-core's `Reviver` on the host decides revive (a player standing within 150 units, after 3 s, 30 percent health), respawn (after 60 s, full health in place) or wipe (everyone down: every game lets its player die) | `Actor::Kill` `0089d900`; the player's disabled-controls byte at +0x680 |

- `engine-core`'s wrap table gets two Coop rows: `0089a760` (`C-2`) and `0089d900` (`C-3`). Each wrap is installed
  only where the address is registered, and passes every call on unless a session set its role.
- No new wire message: coop-core's `Msg::Actor`, `Msg::Combat` and `Msg::Rules` carry everything.

**Limits, stated.** Only placed actors (ids below `ff000000`); an actor made while playing runs in each
game by itself. A hit's kind and limb are not read (a direct hit, no limb), so limbs are not crippled
through this path. Friendly fire is off (coop-core's default). A revive needs no key: standing next to the
downed player starts it. A respawn does not move the player. Only the controls byte is written for a downed
player, not the rest of what `DisablePlayerControls` does. A held actor slides without the game's walk or
run animation. V.A.T.S. for several players, damage outside `Actor::HitMe` and the steps between two states of a held
actor are S21 and S22; the actors the host's game does not run are S27 (hand-off).

**Verified here:** 7 unit tests in `coop-engine` (`fight.rs`) and 17 two-game tests
(`coop-engine-session/tests/fights.rs`); in each the story channel's echo, desyncs, refusals and guest script
count are unchanged. **Not verified here:** every game call above (stand-ins only), and whether the game's
own combat lets an actor fight an avatar whose AI is off.
## S16: conversations (implemented with unit tests; not run on a PC)

The player who talks to an NPC drives the conversation in the own game's dialogue menu. The other
players see each line; a line's results happen once, in the host's game.

- **In the game** (`coop_engine::talk`). Co-op wraps `TESTopicInfo::RunResult` (`0061f170`: the info,
  index 0 when the line is said and 1 when it ends, the speaker) and `DialogMenu::Create` (`00761a20`),
  both as owner `Coop` (new rows of the wrap table). With sharing off (no session, or a host with no
  guest) both pass every call on. With it on, a call that belongs to the player's own conversation (the
  open dialogue menu's speaker, `011d9510` +0x80) is noted, and on a guest a line of the shared story
  runs inside the apply scope, so its result script does not run there. Which story a line belongs to
  is its quest: the quest its topic holds it under (`0061a310`, `00619d00`), else the info's own owner
  quest (+0x48). A line of one of the player's own quests (the opening), and a line whose quest cannot
  be told, runs on the guest as before. An info's result script has no owner quest of its own
  (`Script::InitItem` gives one only for a quest), so until now the script gate let it run on a guest
  and the heal put the story back.
- **In the session** (`coop_engine_session::talk`, coop-core's `dialogue`). The first shared line asks
  the host for the NPC (`Open`); each line said is a `Pick` of its info; its end, or the next line, a
  `LineDone`; the menu closing a `Close`. The host's own conversations go through the same table. The
  host's game runs a guest's result through `RunResult` itself (`talk::enact`) at the protocol's
  `RunBegin` and `RunEnd`, when the line's quest is one the session shares and the speaker is in the
  host's game; anything else is counted and named in the log.
- **Watching.** A machine that is not the talker shows a picked line when its player is in the NPC's
  cell: what was chosen (the info's prompt, when it has one) and each response, as the game's own HUD
  messages (`007052f0`; the text from `TESTopicInfo::GetInfoDisplayText`, `0061e2b0`). It reports the
  line done at once, so the talker never waits for a watcher.
- **One conversation per NPC.** The host's table grants an NPC to one player. A game that knows
  another player has the NPC opens no menu on it (`Create` answers 0, the game's own answer for a
  conversation that could not start) and says so on the HUD. Two players who start in the same moment:
  the host grants one, and the other's menu is closed through `DialogMenu::Close` (`00762160`).
- **Limits, stated.** A result that gives something to "the player" gives it to the host's character
  (the host's game runs the script; a guest's reward is not carried to the guest). A result's
  conditions were judged in the talker's game. Only the newest conversation of another player is known
  to a machine (the host still refuses a second one). No voice or lip movement on a watcher; the line
  is text. A choice with no prompt of its own (the topic's name was the choice) shows the response
  only. A conversation that started before a player joined is not replayed to it.

**Verified here:** 7 tests in the bridge (the wraps with sharing off and on, a host's and a guest's
lines, lines outside the player's conversation, the host running a guest's result, a busy NPC, the text
and the message) and 7 two- and three-game tests (a guest's line once on the host and seen by the
others, a line whose end never comes, the host's line, an own-quest line, one conversation per NPC, two
players at once, a pick the host cannot run). Removing the hold on a guest fails five of them.
**Not verified here:** every game function named above in the real game; in particular that the
dialogue menu's lines all pass `RunResult` with the menu open, and that closing the menu from outside
its own code leaves the game sound.

## S17: party saves (implemented with unit tests; not run on a PC)

- **A part** (`coop_engine_session::party::Part`) is a guest's own piece of the game: where the player
  is (space, position, heading) and the state of the quests each player runs for themself (the story
  channel's quest record and the script's variables). The file: `NVCP`, a version byte (1), the
  player's key, then tagged sections (tag, length, body). A reader refuses a higher version and skips
  a section it does not know.
- **A player's key** is a hash of the name the player chose (`NV_COOP_NAME` as the viewer passes it),
  not the session slot. Two guests who keep the default name share a key, and so one part: each player sets a name.
- **Guest to host.** A guest in the world reads its part every 5 s and sends it when it changed, as
  claims (`Field::Custom` tag 60, chunks of at most 480 bytes). The host keeps the newest part of each
  key in memory; nothing is written to the story.
- **Saving.** Co-op is a second owner of the script extender's seams in `BGSSaveLoadGame::SaveGame`
  (`00847c45`) and `LoadGame` (`00848c3c`), after `nvse`; it takes neither call and only notes the
  save's name (`coop_engine::saves`). When the host's game writes a save, the host writes each part it
  holds as `<save>.<key as 16 hex digits>.nvcoop` in its party folder (`NV_COOP_PARTY_DIR`, default
  `coop-party` in the folder the game was started from). The game's `.fos` is not touched.
- **Loading.** When the host's game reads a save, the parts found for it are published in the
  replicated store (`Field::Custom` tag 61 on ids from `fe500000`), so deltas, the join snapshot and
  the hashes carry them. Each guest takes the newest part with its key once: its own quests are put
  back as on a load (no script runs) and the player is asked to be moved to the saved place, once the
  player is in the world.
- **Limits, stated.** The inventory (S20) and the character (S32) were added as sections later. A
  guest's part is as old as its last upload (at most 5 s). The party
  folder is not the save folder: the platform layer keeps saves, and co-op writes plain files.

## S18: rejoining and joining late (implemented with unit tests; not run on a PC)

- A guest that joins a running game gets coop-core's snapshot, which now also holds the published
  parts; one that lost the host resumes from its replica (S7). Either ends with the host's hashes.
- When a guest leaves, the host publishes its last part, marked as left behind. A guest takes such a
  part only in a game that just started (its driver's first join): a guest that merely lost the
  connection keeps what its still running game has; one whose game was started anew gets its place and
  own quests back. A part from a load is always taken.

**Verified here (S17, S18):** 5 unit tests of the format (round trip, unknown sections, refusals, chunks,
file names) and 4 multi-game tests: a save writes one file per guest and a load gives each part back
once (place and own quests, files unchanged by later play); a player who joins late gets the snapshot
and its saved part while a stranger gets none; a dropped guest keeps its game while a restarted one
gets its part back; a guest at its main menu gets its part once it is in the world. Hash checks ran
with 0 desyncs in each. **Not verified here:** the seams in the real game, the real save names, the
move of a player to a saved place across cells.

## S19: sleep and wait (implemented with unit tests; not run on a PC)

- **In the game** (`coop_engine::rest`). Co-op wraps `SleepWaitMenu::DoClick` (`007c0220`). With
  sharing on, the rest button (4) does not start the rest: the hours (the slider's value plus one, as
  the button reads it) and the bed flag are noted and the click goes on as the game's own cancel (5),
  which closes the menu.
- **In the session** (`coop_engine_session::rest`, coop-core's `rules::vote`). The click proposes the
  rest to the party (sleep in a bed, wait otherwise). The others are told on the HUD and agree by
  pressing rest in their own menu while the vote is open; coop-core's vote needs everyone and runs out
  after 30 s. When all agreed, every game passes the hours itself as the menu does for each hour (the
  scripts once, `00455490`; the player's hour step, `0094df80`, which moves the clock and restores the
  player), and the host sends its clock at once, which holds every guest on the host's time (S10).
- **Limits, stated.** The fade and the menu's hour counter are not shown (the hours pass in one frame).
  Hardcore needs are not updated per hour as the menu does. A vote is not refused in combat yet (the
  rules' combat flag is not set by the engine session).

**Verified here:** 3 tests in the bridge (the button with sharing off, the click turned into a cancel
with the hours asked, the hours through the hour step with its limits) and 3 multi-game tests (three
players agree and the hours pass in every game, ending on the host's clock; one player does not agree
and nothing passes; a host alone rests as the game does). **Not verified here:** the real menu and
hour step.

## S20: items, loot and barter (implemented with unit tests; not run on a PC)

Nothing in this section has run in the real game. Its tests use stand-ins for the game's functions
(`coop_engine::itemkit`).

- **A reference's inventory** (`coop_engine::items`). The items are the base container's objects
  (at +0x30 of a container's base form, +0x64 of an actor's; leveled items left out) and the entries
  of the reference's inventory changes (`004bf220`), walked as the built-in script extender walks
  them; each count is the game's own (`InventoryChanges::GetItemCount`, `004c8f30`). A stack with a
  health of its own (extra data 0x25) is listed with its condition as a part of the item's full
  health. Adding is the `AddItem` command's way: a temporary container (`00481610`) with the item
  (`004818e0`), its health set (`00482090`), given to the reference without the player's message
  (`004821a0`), destroyed (`00481680`). Removing is the `RemoveItem` command's way: the reference's
  virtual slot 0x17c with the ten words the command passes. Caps are the item 0xf, read with
  `InventoryChanges::GetGoldAmount` (`004cb320`).
- **What the player is about to move.** Co-op wraps `ContainerMenu::Create` (`0075b310`),
  `BarterMenu::Create` (`0072d250`) and the player's pick-up (`00953ff0`) as owner `Coop` (new rows
  of the wrap table). With item sharing off they note nothing and pass every call on. With it on
  they note the reference whose menu opened (for a trader also its merchant container, `00421400`)
  and what a pick-up gave the player; every call still goes on unchanged.
- **A guest's take** (`coop_engine_session::items`). The guest's game moves the item at once. The
  guest reads an opened holder and its own player when its change journal names them, and takes a
  change of the holder matched by the opposite change of the player as a move. One item taken or put
  goes to the host as coop-core's `Act::Steal` (`Loot`, `Plant`); items both ways in one step (a
  purchase or a sale, the caps being an item) as one `Act::Barter`; a placed item picked up as
  `Act::Pickup`.
- **The host's game decides.** `HostWorld::enact_with` is the session's one dispatcher of
  interaction requests: doors as before, the item layer's requests to `HostItems::enact`, anything
  else refused. The host's game is asked when the request is carried out: the item must be in its
  holder (the placed item still in its world) and then leaves it. A trade is carried out whole or
  refused whole, so a sold-out stock or a trader without the caps refuses it. The first request
  carried out wins; the host's own player always had its item first.
- **The result.** The asker gets coop-core's verdict. Refused, its game takes the items back from
  its player and gives back what it put or paid. The host publishes the contents of every holder a
  guest asked about or its own player opened (`Field::Custom` tags 70 to 73: a generation, the count
  of pieces, each player's last request handled on the holder, then item and count pairs); a guest
  sets its own holder to them, less its own requests the contents do not show yet. A picked-up
  item's reference is published as gone (tag 74) and leaves every world (`TESObjectREFR::Disable`).
- **Party saves** (S17). A guest's part has a new section (3): the caps and every stack the player
  carries (item, count, condition). The version stays 1; an older build skips the section. On
  taking its part back the guest's player carries what it carried then: the caps set, what is too
  much removed, what is missing added.
- **Switch.** Off unless asked: `NV_COOP_ITEMS=on` runs this section (merged from another line of
  work, switched on one PC run at a time). A quest's reward (S30) is not a take: see S30.
- **Limits, stated.** A container whose contents each game rolled for itself (leveled lists) differs
  until someone opens or takes from it: a guest's first take of something the host's game does not
  have there is refused and taken back, and from then on every game shows the host's contents. A
  holder's condition per stack does not travel (the count per item does); the game chooses which
  stack a removal takes. A move in the same frame as the menu's opening is not seen. Contents above
  four pieces (about 250 kinds of item) are not published. A trader's stock in several references
  is one trade per reference. What is worn and the character's values are in a part since S32. A
  conversation's reward still goes to the host's character (S16).

**Verified here:** 5 tests in the bridge (the listing with base items, changes and conditions; add
and remove; caps; the wraps with sharing off and on), 2 unit tests of the encodings and 7 two- to
four-game tests: a guest's take and put reach the host's container and every game's; two guests who
take the same item in the same moment get it once, and so do the host and a guest (twin: with the
layer off the item exists twice more); a placed item is picked up by one player and leaves every
world; a purchase and a sale through the host, the second buyer and the unpaid seller refused whole;
a party save holds what a guest carries and a load gives it back.
**Not verified here:** every game function above in the real game; that the game's change journal
names a holder and the player on every move of an item; what the real pick-up does to a reference
that persists; two real games.

## S21: V.A.T.S. with several players (implemented with unit tests; not run on a PC)

coop-core's model (`combat::vats`): the world's clock is shared, so with another player in the session
`vats_scales` gives the world speed 1.0 whatever V.A.T.S. wants. Alone, V.A.T.S. is the game's own.

- **What stops the game while a target is picked** is menu mode. The V.A.T.S. menu is a menu like any
  other (class `0x420`, `vatsmenu.rs`); the idle update (`main.rs`, `fn_0086e650`) keeps the answer of
  `Interface::IsInMenuMode` (`00702360`) in the byte at `011dea2b`, and while it is set the process lists,
  the scene and the calendar do not move. Co-op wraps `IsInMenuMode` (`coop_engine::vats`): with a shared
  world, while the V.A.T.S. mode (`011f2250` +8) is neither 0 nor 4 (playback) and the V.A.T.S. menu is the
  top menu (`Interface::IsTopMenuID`, `00702450`), the answer is "no menu". Any other menu, and a menu
  opened over the V.A.T.S. menu, stops the game as before.
- **What slows the game while the shots play** is `BSTimer::SetGlobalTimeMultiplier` (`00aa4db0`), which
  `vats.rs` calls with less than 1. Co-op wraps it: with a shared world a multiplier below the world's
  speed that is asked for while the V.A.T.S. mode is not 0 goes to the game as the world's speed.
- **On every machine.** The host's game runs the actors everyone sees, and a guest's game runs the ones
  the host's does not (S27), so neither may stand still. `coop_engine_session::vats::Share` gives the
  game the world's speed when the first other player is there and takes it back when the last one left
  or the session ends. Off unless asked (merged from another line of work, switched on one PC run at a
  time): `NV_COOP_VATS=on` shares all of this, `slow` the speed only (the menu still stops the game);
  unset or `off`, nothing.
- **The queued shots** are played by the game as attacks. Each lands through `Actor::HitMe`, so a guest's
  shot at a held actor goes to the host's ledger as any hit does (S15), and the host's own go to its game.
- **No advantage.** The player who picks targets is hit in real time (the host's actors go on, and their
  damage reaches the player at once), and the playback gets no slow motion.
- **Presentation.** Each machine tells the others when its player enters and leaves V.A.T.S.
  (coop-core's `CombatMsg::VatsMark`; the host's `VatsHost::active`, a guest's `vats_active`). Nothing
  is drawn from it yet.
- `engine-core`'s wrap table gets two Coop rows: `00702360` and `00aa4db0` (`C-7`).

**Limits, stated.** The translated game has one clock, so the acting player gets no slow presentation of
their own: in a session V.A.T.S. plays at full speed. coop-core's host-side resolution of a queue
(`VatsQueue`, the host's own roll and action point check) is not used: the shooter's game rolls and pays,
and the host's ledger judges each hit. A shot the game gives to another actor than the player (the
Mysterious Stranger) on a guest is not sent. `IsInMenuMode` has many callers; what else in the game changes
when it answers "no menu" under the V.A.T.S. menu (the player's own step runs instead of the menu's) is
only known from a PC run. What `Actor::HitMe` does for a player who is in V.A.T.S. was not traced.

**Verified here:** 3 tests in the bridge (`vats.rs`: alone, the menu, the multiplier) and 5 two-game tests
(`tests/vats.rs`: alone; the host's V.A.T.S. with a guest; a guest's V.A.T.S. and its shot through the
host; twins with V.A.T.S. not shared and with the speed only; the session's end). **Not verified here:**
every game function above in the real game.

## S22: the rest of shared fights (implemented with unit tests; not run on a PC)

### Damage that does not pass `Actor::HitMe` (`coop_engine::harm`)

- An explosion damages an actor through `Actor::HitMe` (`explosion.rs`), so S15 already has it.
- A fall (`actor_p4.rs`, `actor_move`), a trap (`actor_p2.rs`, `actor_do_trap`), drowning and dangerous
  water (`actor_p2.rs`) call the damage applier (virtual `0x338`) straight; an effect
  (`valuemodifiereffect.rs`) calls the value change (virtual `0x3ac`) straight. All of them, and the
  applier itself (`fn_0089d6f0`), lower health through the function behind slot `0x3ac`, `00881130`.
- Co-op wraps `00881130` (wrap row `C-8`). For the health value and a negative amount it asks the
  question the hit wrap asks (`fight::verdict`, one function for both): a guest's game applies no loss
  to an actor another machine runs; a loss on a player's avatar belongs to that player's game; what held
  actors and avatars do to a guest's player comes from the host. Kept losses the session is to hear of
  (a guest's player's on a held actor or an avatar; anything on an avatar in the host's game) are added
  up per victim and source and taken every 100 ms as whole points, then go the way a kept hit goes
  (`ActorMsg::Hit`, `PlayerHit`, the host's vitals). What co-op itself writes passes the wrap.
- **Switch.** Off unless asked: `NV_COOP_HARM=on` (with fights shared up to `damage` or more) has the
  wrap decide; unset, every loss outside `Actor::HitMe` is the game's own, as before
  (`harm::set_on`).
- **Limits, stated.** What made the loss is not read; it is carried as coop-core's damage over time, which
  is neither scaled for the party nor tied to a limb. A held actor does not fall on a guest at all (its AI
  is off); its fall happens in its owner's game. Gains of health are not routed. Other values than health
  (limbs, fatigue) are each game's own. If the player's class has its own function in slot `0x3ac` that
  does not end in `00881130`, the player's losses do not pass the wrap: the repository shows none.

### Actors the host's game does not run (taken out)

- This build had them owned by the guest that reported them first (`coop_engine_session::far`,
  `ActorMsg::Track`). The merge of hayato/coop-next (2026-11-12) took that out: its hand-off (S27), proven
  on a PC, is the behaviour now. The nearest player's game runs an actor, the host tracks the loaded actors
  near a guest's avatar, and every player's hit on a handed actor goes through the host's ledger. The hit
  and value-change wraps share one decision (`fight::verdict`), which keeps a hit on a handed actor
  (`fight::set_ledgered`) from the game as it keeps one on a held actor.
### Steps between two states (`coop_engine_session::smooth`)

- A held actor gets coop-core's `avatar::InterpBuffer`: the newest state is fed at the avatars' rate and
  the actor is put where the buffer says, 100 ms behind and blended between two states. Another space or
  a long jump is taken at once. Off unless asked: `NV_COOP_SMOOTH=on` (`Fight::smooth_on`). Only held
  actors (`NV_COOP_FIGHT=state` up to `revive`) are moved this way; a followed copy goes along its
  owner's path with its gait and stands where its owner's actor stands (DECISIONS.md D6, D7), which
  this does not touch.
- **Not done:** the game's own walk or run animation on a held actor. The avatar code's approach
  (restrained, moved by the game's own mover) was not examined for held actors.

**Verified here (S22):** 4 tests in the bridge (`harm.rs`), 2 unit tests (`smooth.rs`), 3 two-game tests
(`tests/vats.rs`: a held actor's loss, an effect on an avatar, the twin with state only) and 1 two-game test
(`tests/fights.rs`: a held actor's steps and their twin). The six three-game tests of the guest-owned actors
went with them. **Not verified here:** every game function above in the real game.

## S23: scenes and followed actors (implemented; docs/coop/DECISIONS.md D5, rung 3; not yet run on a PC)

- **Scenes.** A wrap of the `StartConversation` command's body (`005c8740`, WRAPS step C-5) parses the
  command's parameters itself (the offset into the script data put back for the body's own parse). A
  conversation with the player on either side is that player's own and runs as it is. One between two
  placed actors is the host's: the host's game runs it and notes it (`commands::Scene`: both ids, the topic,
  the two locations, the flags, both places); a guest's game does not run its own (the parameters are still
  read past, so the script goes on) and counts it. The host writes a `Field::Custom` (`SCENE_TAG`) on the
  actor; the guest puts both where the host had them, when they are in that space, and starts the
  conversation through the actor's virtual slot 0x280 as the body does. One that comes with a join or a
  repair is not played.
- **Followed actors** (`NV_COOP_FIGHT=follow`, the viewer's default). PC run 10 showed that a held actor
  (AI off, moved every frame) keeps the pose it had. Followed, the host's actors keep their own AI on a
  guest; one further than `FOLLOW_DISTANCE` (200 units) from the host's place for `FOLLOW_MS` (1.5 s) is put
  there (`fight::nudge`) and picks its package again. A copy in another space than the host's actor is left
  alone. Holding (`state` and up) remains for the shared fights that need it.

## S24: packages, doors, and the first peer review (implemented; not yet run on a PC)

- **The host's package.** PC run 12: the two copies of an actor picked different packages (Sunny led the
  host outdoors and walked back indoors on the guest; Chet sat on the host and walked on the spot on the
  guest), because a package's schedule and conditions are each game's own and "the player" is that game's
  player. The host writes the package each actor near it runs (`PACKAGE_TAG`; 0 when it has none both
  games have, when it is aimed at the player other than to keep company, and when the host stops running
  the actor). On a guest a wrap of `PackageList::FindCurrentPackage` (`0067e780`, WRAPS step C-7) answers an
  evaluation of a followed actor with that package, after doing what the game's own pick does (`picked`:
  procedure type, the old package's end, the new one's beginning). The follow step asks a copy that does not
  run it to pick again (no AI reset), at once when the host's changes, else every 3 s, 5 times at most.
- **Through doors.** A followed copy in another space than the host's actor is brought with the game's own
  `MoveTo` (`005ccb20`) to this player's space at the host's place, every 3 s at most, 3 times at most.
- **Switches** (D5, condition 6): `NV_COOP_SCENES=off`; `NV_COOP_FIGHT=follow-plain` (no leading, bringing or
  packages); `NV_COOP_PACKAGES=off`.
- **Peer review of this milestone (two reviewers, 2026-11-11).** Fixed: a scene asked of actors the game's
  own command refuses (now its checks, plus loaded and alive); the package pin outliving the host's
  interest; `picked`'s bookkeeping skipped; packages aimed at the player; unbounded asks and brings; an AI
  reset on every package change; a scene lost when it came with a repair (now by count); the scene noted
  before the game's body ran; no switches; counters and lines missing from the log. Open, recorded: a
  guest's own scripts' conversations between actors are dropped and not forwarded to the host (a scene only
  a guest triggers does not happen until the host's game triggers it); two scenes of one actor between two
  sends keep the last; an actor in dialogue with the guest's player, or its companion, is not exempt from
  following; the stand-ins move and pick at once where the game takes frames, so timing faults show only on
  a PC.

## S25: spent triggers, actors that leave a space (implemented; not yet run on a PC)

PC run 13 (2026-11-11), from the two logs and the game's data.

- **A spent trigger is its player's own.** The guest could draw no weapon. `VCG01RemoveDocCarebearModeTriggerSCRIPT`
  (`0011e65e`, on `DocHouseTriggerREF` `0011e660`) gives the player who walks out of Doc Mitchell's house
  their controls back and then deletes itself. The host's player spent it; the journal put its
  `RefDisabled` into the story; the guest's was switched off before the guest's player reached it. A
  trigger volume (a reference with a primitive: `ExtraDataList::GetPrimitive`, `0041fbe0`) switched by its
  own object script (`actors::own_trigger_change`: the journal's cause is an object script running on that
  same reference) is not written by the host. A guest remembers the triggers its own player spent, with
  what the story said of each then, and neither heals nor applies the story over them until the story says
  something new (a quest's script switching the trigger is still the story's).
- **The space an actor leaves is told.** Actor states go to the players in the actor's space
  (`Audience::Cell`). A guest never learned that the host's actor had gone through a door, and kept putting
  its copy at the last place it knew there (Chet, Joe Cobb: "stands at the door"). The host sends the first
  state in the new space to the players of the old one too. A guest whose copy is still in its player's
  space then only asks it to take the host's package (which walks it out as it did the host's); it is not
  moved.
- **The host's package wherever the copy is.** The pin held only for copies in the guest's player's space:
  Sunny's copy walked out after the host's and, out of sight, picked this game's own package and came back.
  It now holds for every actor the host has said a package of (the host takes its word back when it stops
  running the actor).
- **Not in this step, recorded.** A trigger that starts a scene or sets the story counts only when the
  host's player walks into it (`GSJoeCobbTriggerScript` `0010521f`: the guest's ran, its conversation was
  dropped and its effects put back; nothing told the host). What a host-run script gives "the player"
  reaches the host's player only (Sunny's Varmint Rifle). An actor that leaves without a package both games
  have (fleeing) stays where this game has it. A body lies where each game dropped it. A led actor has no
  walk animation.

## S26: proposals (D5, rung 2; implemented; not yet run on a PC)

PC run 16: the guest shot Sunny's bottles (`VCG02TargetSCRIPT`, `0010a1ef`: `set VCG02.nTargetCount to
VCG02.nTargetCount + 1`, `SetObjectiveCompleted`, a stage) and the heal put each change back.

- **What is proposed.** A difference between a guest's game and the story whose cause, by the guest's
  journal, is one of its own object scripts: a script with a form id that is not a quest's (the gate holds
  those) and not a dialogue result (id 0; the conversations carry those). A script variable of a quest
  (`before`, `after`), a reference switched off or on, a quest's stage newly done, an objective's new state.
  A reference the story has no entry for is proposed straight from the journal.
- **The claim.** `PROPOSE_TAG` (`0x5052`), sent with the guest's other claims. The guest leaves its own
  change alone for `PROPOSE_HOLD_MS` (3 s); if the story has not come to agree by then it is put back.
- **The host.** A variable is set to `after` when the host's is still `before`; left when it is already
  `after`; otherwise the guest's step is added on top. A reference is switched only from the state the
  guest saw, never a trigger volume. A stage is set through the game's `SetStage` (its result script runs,
  on the host) when not yet done; an objective's state through the quest's own setter. The outcome goes
  into the story for every guest.
- **Switch.** `NV_COOP_PROPOSE=off` on a guest: put back as before.
- **Known limits, recorded.** Two players changing one counter inside one round trip to the same value
  count once (the host cannot tell "both set it to 1" from "both added 1"). What the script said or gave
  to "the player" happens in the game it ran in only. A conversation between two actors that a guest's
  script asked for is still dropped, not proposed. Sunny's own lines and packages aimed at "the player"
  are the host's player's.

## S15 addendum: following with the fights shared

`NV_COOP_FIGHT=follow-damage`, `follow-threat`, `follow-full`: a followed actor keeps its AI, is in the set
the hit wrap reads (a guest's hit on it is a claim for the host; its own blows on the guest's player are
dropped, the host's game says what hits), and has the host's health. PC run 16 at `follow-threat`: geckos
died to the guest's shots on both games and turned on the guest.

## S27: hand-off, the nearest player's game runs an actor (implemented; not yet run on a PC)

PC run 21: a game runs its actors at full detail only near its own player. With the host's player at
the saloon door, the host's Sunny and geckos near the guest stood still or moved in jumps (low process),
and the guest's copies, following them, did the same.

- **Who runs an actor.** coop-core's ownership (nearest player in the actor's space, 300 units of
  hysteresis, 1 s hold) is no longer pinned to the host, except for an actor whose package keeps
  company with the host's player (`commands::keeps_company`: follow, escort, accompany aimed at the
  player). `NV_COOP_HANDOFF=off` pins every actor to the host as before.
- **What the host tracks.** The actors near its player (the close-actors list) and every loaded actor
  (the process array, all four levels: `005de0f0`, `005be5c0`, `00968670`) within 4096 units of a
  guest's avatar in the same space.
- **A guest that runs an actor.** It leaves it to its own AI (with the host's package still pinned),
  and sends its state every 100 ms when it changes (`ActorClient::publish`; the target names this
  guest's player).
- **The host's copy.** Put at the guest's place and heading every frame, health from the ledger.
- **Damage.** Every player's hit on a handed actor is kept from that game (`fight::set_ledgered`) and
  goes to the host's ledger; the game that runs the actor does the damage the ledger records
  (`damage_log`), its death is that game's and reaches the host as a claim. What the actor does to a
  player is that player's game's own.
- **Known limits.** The host's copy of a handed actor is moved without its walk. A guest's game runs a
  handed actor with the host's package; a package that only the host's game has started may not be
  where the guest's copy is.

## S27 addendum (PC run 22)

A handed actor stays tracked while it lives, whatever the host's own lists say from one moment to the
next; the guests' places are read from their avatar reports (`avatars.state`), not from their avatars in
the host's game, which are not loaded while the host is in another space; a dead actor's package is
withdrawn (0).

## S28: lines said to the player away from the host (implemented; not yet run on a PC)

`SayTo` (`005c9100`) is wrapped on the host. A line a script has an actor say to the player
(`thisObj` an actor, the target the player) is noted with its topic before the body runs. If the
speaker is not loaded or has no process here, is in another space, or is more than 1500 units from
the host's player, or a guest's game runs it (S27), the host writes `SAY_TAG` (`0x5359`: a count, the
topic) on the speaker. A guest whose player is within 2500 units of its copy, in the same space, says it
there (`commands::say_to_player`: as `SayTo`'s high-process branch: stop the current dialogue, pair the
processes, the process's say slot `0x2a4`); its result runs in that game and its story changes reach the
host as proposals (S26). One that came with the join is not said. Not tested against the game's own
command: the stand-ins have no `SayTo`.

## S29: a guest's completed objectives and items between players (in part implemented)

Found in the harness on 2026-11-13 (VCG03, Sunny's healing powder). A quest script runs in the
host's game only, and its `Player` is the host's player alone. VCG03's script completes "find a
Xander root and a Broc flower" while that player holds both and takes it back while it does not.
So when a guest who held both completed it, through its own gather trigger, the host's script took
it back at once.

Decided by a committee of three (unanimous), between three options:
- A. Per player, as the original game counts.
- B. The party's items summed for quest scripts.
- C. A, plus giving items between players.

The vote was C. B makes a check pass on items that a removal (`Player.RemoveItem` in the speaker's
game) then cannot take: deliveries, purchases. Every quest that counts items would be exposed, and
a split pair still cannot be crafted. Plants are picked separately in each game, so each player
can gather and cook. A split pair is pooled by giving one item to the other player.

- Implemented: on the host, an objective a guest's game completed (its proposal applied) is not set
  back by the host's own scripts. The wrap is at the objective state setter `005ec5d0`, step C-9.
  The guest's own taking back is applied as ever.
- To do: giving an item to another player, done by the host as one step (taken from one, given to
  the other, or neither). Quest items are allowed between players only. The owner decided
  (2026-11-13): giving is one-way only, a player picks an item and an amount and hands it over,
  with nothing asked in return; no trade window where both sides offer and confirm.
- Rewards from host-only scripts (experience, caps, items): S30.

## S30: the story's experience, caps and items go to every player (implemented)

Found in the harness on 2026-11-13 (VCG03): Trudy's line that ends Sunny's last tutorial gives
`RewardXP 50`. Said to the guest, its result ran in the host's game (the guest's results are held
for the host), and the host's player alone got the 50. Sunny's line that starts VCG03 gives 50 too.

The rules already decide this: every row of the bundled table has `reward: everyone` (every player
gets the full reward once; `RewardLedger` keeps who has which). This spec carries it out on the
engine backend for experience, caps and items (the owner, 2026-11-13: both players get any caps,
experience and items a quest gives).

- The host's game wraps `PlayerCharacter::RewardExperience` (`0094fd10`, vtable +0x488, what the
  `RewardXP` command `005d5140` calls), step C-10. The amount is given as ever. While a story
  script runs (its owner quest is set: a stage's or a line's result, a quest's script) and that
  quest is not one each player plays for themself, the reward is noted (`coop_engine::reward`).
  A line's result script names no owner quest: the quest of the line whose result runs
  (`talk::result_quest`, kept by the `TESTopicInfo::RunResult` wrap) is the one.
  Not noted: an object script with no quest (each game that has the object runs it), anything while
  the host's state is applied, anything on a guest.
- Items: the host's game wraps the hand-over of a container's items to a reference (`004821a0`,
  what `AddItem` gives through, caps included; a leveled list is already rolled into the
  container), step C-11. When the reference is the player and a story script runs (as above), each
  item and its count is noted.
- The host's driver numbers each noted reward and hands it to the rules (`RulesHost::reward`): every
  other player in the party gets `RulesMsg::Reward { id, gift }` once (protocol 7; `Gift::Xp` or
  `Gift::Item { form, count }`).
- A guest's driver gives its player the same the game's own way, the message included: experience
  through `RewardExperience`, items through a container of its own handed to its player, as
  `AddItem` does. One that cannot be given yet (no player) waits; an item its data lacks is not
  given.
- Open: a player who joins after a reward does not get it (the ledger can name what it misses;
  claiming needs the player's identity across sessions). Not mirrored: what a story script takes
  from the player (`Player.RemoveItem`: a payment, a hand-in) is taken from the host's player only,
  even when the guest said the line (the rules' `Payment` goes to the line's initiator: to do);
  items moved to the player from a container or an NPC (`RemoveAllItems player`) are not copied.
- With items shared (S20, `NV_COOP_ITEMS=on`) the reward path alone gives a quest's items. S20 never
  gives a player anything on the host's word (it moves holders' contents and takes back what the host
  refused), so the two cannot give a reward twice; but a reward reaching a guest's player in a step in
  which a holder it has open lost the same item would read as a take, and be put to the host. S20
  stands aside: each item reward a guest's game gives (sent by the host, or noted for a line said
  there) is counted into that player's known items (`GuestItems::story_gave`), so it is never asked
  for (test `a_quest_reward_is_not_read_as_a_take`).

## S31: the other players on the compass (implemented; run in two headless games, not yet on a PC)

The owner (2026-11-15): a marker of its own for the other players (not for companions), on the
HUD's compass, in purple, a colour none of the game's HUD colours (amber, white, blue, green, and
the red of a hostile tick) is near.

- Each other player this game shows (their avatar, which exists while both players are in the same
  interior or worldspace) has a tick on the compass in purple (190, 80, 255), step C-14
  (`crates/coop-engine/src/compass.rs`, a wrap of `HUDMainMenu::UpdateCompass` `00779070`).
- The game's own actor ticks are untouched: the avatars are kept off them for that call (the
  player's compass list's keep-off byte, set and put back around the call), so a player is never
  ticked twice or in an NPC's colour. The avatar's tick then goes on a tile the game left unused
  that frame (the last one when every tile was used), placed and faded as the game places and
  fades an actor tick, hidden while the player is outside the strip (behind, or exactly to the
  side). Its system colour is taken away so the tile shows its own red, green and blue; the game
  gives a tile it uses again its system colour back, so no NPC is left purple.
- Unlike an NPC's tick, the player's is not limited by distance or by being detected.
- Checked in two headless games (`/tmp/s_compass*.sh`, harness `compass`, which reads each shown
  tick's traits and the colour its picture is drawn in): after loading the saves, outside the
  saloon, inside the saloon (through its door) and at the Goodsprings Source after a fast travel,
  each game showed the other player ahead of it in purple (drawn 0.75, 0.31, 1.00) and its NPC
  ticks in the HUD's amber; none when the other player was behind or at the side.

## S32: a guest's character in a party save (implemented with unit tests; not run on a PC)

Nothing in this section has run in the real game. Its tests use stand-ins for the game's functions
(`coop_engine::statkit`, with `itemkit` for what is worn).

- **The section.** A guest's part (S17) has a new section (4), the version stays 1 and an older build
  skips it. It is a list of tagged parts of its own (`coop_engine_session::sheet`): 1 the character,
  2 its condition, 3 what it wears; a part with a tag this build does not know is skipped, one with
  other counts of values than this build's is read as missing and its values are left alone.
- **The character** (`coop_engine::stats`). The SPECIAL attributes (actor values 5 to 0xb) and the
  fourteen skill values (0x20 to 0x2d) as base values: read through the value owner's `+0x04`, set
  through `00881360`, which adds to a base value as the game does. The tag skills: the four words at
  +0x44 of the player's class (`Actor::GetClass`, `00884350`), set through `TESClass::SetTagSkill`
  (`005f6e80`) as the `SetPlayerTagSkill` command sets them. The level through
  `CharacterProgression::ForceLevel` (`008d5300`), as the set-level command; the experience (0x18)
  then by the difference through the actor's integer change of a value (`+0x3a8`), with the game's
  quiet byte set as `ForceLevel` sets it, so no message shows and no level-up is judged. The perks
  with their ranks: the player's list at +0x87c, given through `00963790` and taken through
  `00963900`; a perk this game's data lack is left out. Traits travel as the perks of that list
  they are; nothing here shows the game keeping them anywhere else.
- **The condition.** Health (0x10), action points (0x0c), radiation (0x36) and the seven body
  conditions (0x19 to 0x1f) as current values: read through the value owner's `+0x0c`, set by the
  difference through the actor's change of a value (`+0x3ac`), the call damage, restoring and
  radiation make. Karma (0x17) by the difference through `+0x3a8`, without `RewardKarma`'s message.
- **What is worn.** The inventory's stacks marked worn (extra data 0x16); on taking the part back
  what is worn and not saved is taken off (`Actor::QueueUnEquipObject`, `0088c790`, as
  `UnequipItem`), and what was saved and is carried is put on (`Actor::QueueEquipObject`,
  `0088c650`, as `EquipItem`).
- **Order.** On taking its part back a guest's game puts back the quests and the place, then the
  pack (S20), then the character: the level and experience, the perks, the tag skills, and the base
  values last, so that what giving a perk did to a base value is set back to the saved value; then
  the condition, whose limits follow from the character; then what is worn, which the pack has just
  given back.
- **Limits, stated.** Name, sex, race, head parts, hair, hair colour and face values are not in the
  part: the translation reads the sex (`0087f4c0`) and has the race menu's own functions, but shows
  no function that sets these on a player in the world and rebuilds its head (only the menu does,
  through its own state), nor where the entered name is taken. A level-up the player had not taken
  yet is not kept (`ForceLevel` clears the ready byte). An item that cannot be taken off stays on.
  Equipping is queued in the game: the worn list a guest uploads right after taking its part back may
  not show it yet. Skill values are bases: whether the game's base of a skill already holds what the
  attributes give is not established here, so a saved skill is set back to the saved base whatever
  the attributes are.

**Verified here:** 9 tests in the bridge (the value codes; a game without the functions; a character
read and applied in another game, counted; perks given before base values; the condition through the
change of a value, by the difference; what is worn put on and taken off; a sheet whole and in part;
experience changed quietly), 3 unit tests of the section (round trip, a cut section and an unknown
part, another build's counts) and 3 multi-game tests: a party save holds a guest's character and a
load gives it back after it changed; a guest whose game started anew gets its character back with
its pack; a part without the section leaves the character alone. **Not verified here:** every game
function above in the real game, that `00881360` gives the base value the owner's `+0x04` reads for
the player, what the real perk functions do to values, two real games.

## S33: companions follow the player who hired them (implemented with unit tests; not run on a PC)

Replaces the earlier note "Not done: followers". Bridge: `crates/coop-engine/src/companion.rs`
(stand-ins in `companion_kit.rs`); session: `crates/coop-engine-session/src/companion.rs`; step C-15.

- **Who hired it.** A wrap of `Actor::SetPlayerTeammate` (`008bca90`, `actor_p6.rs`) notes every
  change of a placed actor's teammate flag (+0x18d) while a session runs, except co-op's own. The host
  puts a hire down to the guest whose line its game was running then (`talk::enact`, wrapped by
  `companion::with_hirer`), else to the host when its own dialogue menu was open on the actor, else to
  the player who last spoke with the actor (a conversation's first line), else to the host. A let-go
  has no keeper. The host writes `Field::Custom` tag 90 on the actor: the keeper's slot (0xff: none)
  and the companion wheel's four switches.
- **Following.** The package is not changed (other actors may run it). Each `ProcessFollow`
  (`HighProcess` `008e0f80`, `MiddleHighProcess` `00918ee0`, `LowProcess` `00909de0`) follows the
  process's target (`pTarget`, +0x40); a wrap of each sets that target, through the process's own
  `SetTarget` (slot 0x12c), to the keeper's avatar when another player keeps the companion, the avatar
  is loaded in the companion's space and the target is this game's player (or none). The game's own
  `ResetTarget` puts the player back each update, and the next follow step aims it again. After an aimed
  step the companion is taken off this game's player's followers (`ExtraDataList::RemoveFollower`), which
  the high process's step adds it to. So on the host a guest's companion follows that guest's avatar,
  and on every guest a companion another player keeps follows that player's avatar. In its keeper's game
  it follows that game's own player as the game has it, and co-op makes it that player's teammate there
  through the game's own `SetPlayerTeammate`. A companion the host hired is unchanged in the host's game.
- **Who runs it.** With hand-off on, a companion a guest keeps is pinned to that guest
  (coop-core's `set_follower`), so the keeper's game runs it and the other games' copies follow its
  path (D6, D7). The host's actors that keep company with the host's player stay the host's.
- **Doors, fast travel, cells.** The keeper's game brings a following companion (its package keeps
  company with "the player" and the wheel's "Waiting" is 0) that is in another space than its player,
  or more than 4096 units away, to 64 units behind the player with the game's own `MoveTo`
  (`fight::bring`), every 3 s at most. A waiting companion, or one let go, stays.
- **Fights.** While a session runs, a blow of a companion any player keeps on a player (this game's or
  an avatar) is dropped and not told (`fight::verdict`). Whom it fights is the keeper's game's own
  teammate logic, and the copies fight their owner's target (D6).
- **The wheel.** The companion wheel (`companionwheelmenu.rs`) is the keeper's game's own menu, and
  its commands run there. Its four switches ("FollowerSwitchAggressive", "IsFollowingLong",
  "CombatStyleRanged", "Waiting", the exe's own texts) are read every 500 ms through the actor's
  variable getter (`008c1740`) and a change is proposed to the host (claim tag 91). The host takes it
  only from the companion's keeper, sets the switches on its copy (`008c16c0`) and has it pick its
  package again, and every game gets them in tag 90. The wheel's inventory is the game's own container
  menu on the keeper's copy (S20 with `NV_COOP_ITEMS=on`).
- **Party saves.** A guest's part has a section 90: each companion it keeps with its switches. A
  part given back (a load, or a game started anew) is claimed of the host (claim tag 92, again every
  2 s, 5 times at most). The host takes it for an actor its game has as a teammate that no other player
  in the session keeps, and sets the saved switches; a companion that player no longer names is the
  host's. A load in the host's game first gives every companion to the host. A guest that comes back to
  a running session claims the companions it kept.
- **Limits, stated.** Not run on a PC: every function above is a stand-in in the tests. A hire whose
  line runs in a guest's own game (not held for the host) is not seen; the repository does not show
  which lines hire. The guest-kept companion is still the host player's teammate in the host's game (the
  story's state): the host's own wheel opens on it there, and when the host's game runs it (no hand-off,
  or the guest's game does not have it) its teammate fights are the host player's. Whether the game moves
  teammates through a door by the follower list this takes it off, or by the teammate list, was not
  traced. The repository's wheel has no "use stealth" switch: a teammate sneaking with the player is the
  game's own. The keeper's copy picks its own package with its own game's state: a hire line's result
  that sets the companion's own script variables runs in the host's game only, so a package that reads
  them may differ until the next hire or switch. No test drives a real guest's line through
  `Enactor` to the hire (the tests call `with_hirer`; the last-speaker rule covers the same case).

<!-- split -->
## S34: split-screen on one PC with Couchlink (implemented with unit tests; never run)

Couchlink (github.com/cranebeaker/couchlink) runs one copy of a game per player on one PC, puts
each window on its part of a screen, gives each copy only its own controller or keyboard and
mouse, and keeps each copy's files apart. Its recipe `fallout-new-vegas-nvrs-coop` (in its
`core/recipes/`) starts this package; the older `fallout-new-vegas-coop` stays for the older
co-op build.

- **A package copy per seat.** Couchlink's copy of the package folder is a tree of hardlinks
  (`<state>/instances/<recipe>/<seat>/`). The launcher's mutex is per folder, so each copy runs
  one game. The recipe leaves `userdata\game` (the game's Documents, saves, settings and AppData),
  `userdata\mods`, `modgame`, `logs`, `reports` and `coop-party` out of the copies: each seat's
  Play.exe makes its own on its first start, and they stay in that seat's copy between sessions.
  The game writes nothing outside the package folder (the launcher points its Documents and
  AppData into `userdata`), so no save folder needs redirecting. The machine's tables and
  `game-folder.txt` are linked from the package (the package must have run once alone).
- **The window.** `Play.exe --window WIDTHxHEIGHT` (the seat's size, Couchlink's
  `$WIDTH`/`$HEIGHT`) sets the game's own window to that size, not full screen
  (`bFull Screen=0`, `iSize W`, `iSize H`), and framed (`NV_BORDERLESS=0`, over settings.ini and
  Fallout.ini): the launcher's borderless window covers the whole primary monitor and sizes the
  game's drawing window to it, which a seat's smaller region would cut. Couchlink takes the frame
  off and moves the window onto the seat's region (`window.borderless`, kept in place), and
  Direct3D stretches the picture over it. The engine's framed window is not DPI aware, so with
  display scaling Windows may scale its picture.
- **Input.** The engine reads no controller: its `XInputGetState` answers that none is connected.
  A seat's pad plays through the recipe's `controls` profile (keys and the mouse: left stick WASD,
  right stick the mouse, triggers the mouse buttons), which Couchlink's input hook gives the copy
  as window messages (`WM_KEYDOWN`/`UP` with the scan code, `WM_INPUT` for the mouse the window
  registered) and as answers to the cursor calls. The engine reads its keys and mouse from those
  messages. The hook also fakes the focus (`WM_ACTIVATE` inactive and `WM_KILLFOCUS` never
  arrive), so a window that is not in front keeps its player's input. A player with a keyboard
  and mouse of their own gets them the same way.
- **Sound.** Every copy plays through WASAPI in shared mode; nothing in the engine stops sound
  when its window is not in front.
- **Joining.** `NV_COOP=host:local` hosts on the loopback address (UDP 27961) with a fixed token
  (`LOCAL_TOKEN`), `join:local` joins it, and `local:SEAT` is the first for seat 0 and the second
  for any other seat, so one setting (`local:$SEAT`) serves every copy and nobody reads a join
  code. A guest that starts before the host listens tries again until it answers. Such a host
  cannot be reached from another PC: friends elsewhere need a normal `host` session.
- Tests: the launcher's `--window` reading and the seat's prefs lines (scripts/launcher), the
  `local` values and the host's socket (viewer/src/coop.rs), and the recipe's values in
  Couchlink's built-in recipe test. Not run: the game under Couchlink, two copies on one PC, the
  hook's input reaching the engine's window, the window's placement.

## S35: the other player's avatar as that player (implemented with unit tests; not run on a PC)

Until now an avatar was placed from a base form (this game's own player's by default) with its AI
off, and slid from place to place: it did not walk, did not hold the other player's weapon, and
looked like this game's player. `NV_COOP_LOOK=walk|pose|gear|face` (off unless asked; each level
includes the ones before it) changes that (`coop_engine::look`, `coop_engine_session::look`). Nothing
new travels: coop-core's avatar stream already carried the fields, unused until now.

| Level | What the owner sends | What the other games do with its avatar | Game facts (each from this tree's translation) |
| --- | --- | --- | --- |
| walk | its movement flags in each state (`AvatarState::anim`, the mover's `GetMoveMode`), sneaking as `FLAG_SNEAK` | its AI is kept on (the game animates it) and it is put on its player's place every frame; the flags are forced on it when they change (`ForceMoveMode`), so it walks, runs or sneaks as its player does and stands (the idle) when they are 0, as D6 B2's led copies; a fight its own AI starts is ended at once | `ACTOR_MOVE_FLAGS` `008846e0`, `ACTOR_FORCE_MOVE_MODE` `008b3a20` (as D6); `bProcessMe` +0xbc |
| pose | its weapon (`AvatarState::weapon`), drawn as `FLAG_DRAWN`; an action event for each attack its game starts (`ActionKind::Shoot`, the animation group) and each reload (`ActionKind::Reload`) | the weapon is given and put in its hand, with ammunition for it; drawn or put away as its player's is; each attack is the game's own `QueueAttack` with the same group, each reload `QueueReload`; an attack is a cue, its hits decided by the hit wrap as any actor's (S15) | `Actor::StartAttack` `00893a40` (the player's attack button, `fn_00948310`, starts there; a wrap, step C-15, notes the player's), `Actor::IsWeaponDrawn` `008a16d0`, the draw call `008a6840`, `Actor::GetCurrentWeapon` `008a1710`, `Actor::GetAnimAction` `008a7570` (0x11 a reload), `Actor::QueueReload` `008a8840`, `TESObjectWEAP::GetCurrentAmmo` `00525980` |
| gear | what it wears (the parts' equipment: each inventory item its game says it wears), read every second and sent when it changes | it is given each item it lacks (as `AddItem`), puts it on and takes off whatever else it wears, through the `EquipItem` and `UnequipItem` commands' calls | `InventoryChanges::WearingObject` `004bfda0`, `Actor::QueueEquipObject` `0088c650`, `Actor::QueueUnEquipObject` `0088c790` |
| face | its face, body and name (the parts' appearance, `look::encode`: sex, race, hair, hair length, eyes, hair colour, height, weight, head parts, the four face-gen matrices in thousandths, the name), what character creation set on its base form | a new NPC base is made by the game (`CreateFormOfType`, a new id of its own) as a copy of the avatar base (`TESNPC::Copy`), the player's look is written onto it (its face NPC cleared, which the copy had made the source), and the avatar is placed from it; a changed look (a new name) is a new base and the avatar is placed anew. The crosshair and anything else that names the reference read the base's name | `CreateFormOfType` `00465110`, `TESNPC::Copy` virtual `0x108` (`006035e0`), the `TESNPC` layout (`tesnpc.rs`), `BSSimpleList` clear `00470470` and append `00905820`, the face-gen coordinate `00601800` and matrix resize `0060b340`, `TESFullName::GetFullName` `00482720`, `BSStringT` set `004037f0` |

- A guest sends its parts and actions only once the host has taken it in (the host drops what a
  guest sends before); an attack started before that is not played later. The host's hub gives a
  late joiner the parts it holds.
- Below `face` no base form is made: the avatar keeps its default base, as before.
- With `NV_COOP_LOOK` off the ladder is as it was (S8): AI off, a sliding figure.
- The compass (S31) ticks the avatar whatever its base.

**Verified here:** 8 unit tests in `coop-engine` (`look_tests.rs`, on the testkit's look kit:
the pose read, the attack wrap noting only the player's started attacks, the lead keeping the AI
on, gait and calm, draw, attack and reload, wearing with strip and ammunition, the base copy and
its face, forms the game lacks), 4 in the session (`look.rs`: the appearance's encoding and the
cap's words), 7 two-game tests (`tests/look.rs`: walking, running, sneaking and standing both
ways; the look off as before; weapon in hand, drawn, attacks once each, a refused attack not sent,
reload, a weapon change, put away, both directions; an attack from before the guest was in; gear
put on and taken off with its player; face, body and name both ways and a new base on a new name;
no base below the face level), each with the story channel untouched. Mutation checks: the AI kept
on, the gait in the state, the face NPC cleared, the strip, the started-attack test, the first
frame's count; each killed by a test.

**Not verified here:** every game call above (stand-ins only). In particular, on a PC:

- whether a placed actor of the player's base, its AI on, with no package of its own, does
  anything of its own that this does not undo (only its fights are ended);
- whether the forced movement flags give the sneak and turn animations (D6 B2 showed walk and run
  for NPCs); the animation's speed is the avatar's own (its speed values are not synced), its
  place is its player's;
- whether `QueueEquipObject` on a placed actor puts the item on at once or a frame later, whether
  equipped ammunition counts as worn, and whether a replayed attack plays without the avatar being
  in combat;
- whether the head of an NPC base made while playing is built from its face-gen values (as the
  player's is) or looked for in face-gen files that do not exist for it;
- what a save made while such a base and avatar exist holds (avatars were already not removed
  before a save, S8).

**Not done:** jumping and falling animations (the place follows the player's path, height
included, but nothing read here shows how the game starts a jump or a fall on another actor), and
turning on the spot (the heading is set, no turn animation is asked for). A look changed after
character creation (a barber, a surgeon) is read again within a second and gives a new base.

## S36: voice chat (implemented with unit tests; not run on a PC)

`NV_COOP_VOICE=off|ptt|open` (default `ptt`). The design is the old viewer's (`main`'s
`viewer/src/coop_live/voice.rs`); what changed is where the frames travel.

- **Frames.** `nv-viewer`'s `coop_voice` (cargo feature `voice`, on by default) opens Windows' default
  microphone and speakers through cpal on a session's first frame; `coop_voice::pipeline::VoiceTx`
  resamples, gates (push to talk, or an open microphone with the energy gate) and encodes Opus (48 kHz,
  20 ms, 24 kbit/s); each frame goes into the session's `VoiceLink` (`coop_engine_session::voice`).
- **Wire.** No new message: coop-core's `Msg::Social(SocialMsg::Voice)` (unreliable, at most 480 bytes of
  payload) carries each frame, so none of the tags 120 to 129 is used. A guest sends only once the host
  let it in (before that the frames are counted and dropped). The host's live layer checks the speaker
  against the sender and the rate (`VoiceHub`: 80 frames, 24 KB a second) and relays to everyone but the
  speaker; the host player's copies come out of the live layer's local inbox into the host's view. Every
  receiver orders and deduplicates by speaker (`VoiceRx`). At most 50 frames wait to be sent and 400 to
  be played: older ones are dropped and counted.
- **Places.** Each step the link holds this player's place (`coop_engine::live::local`: position,
  heading, interior cell or world space) and every other player's from the live ladder's interpolated
  avatar sample. `coop_voice::pipeline::VoiceRx` uses them: full volume within 1200 units, down to 15%
  at 7000, panned by the listener's heading; a player in another space, or with no place (ladder off),
  is heard flat at 50%.
- **Keys and volume.** Read from the game's window (`engine_platform::d3d9real::held_keys`, DirectInput
  codes, only while it has the focus): `NV_COOP_PTT_KEY` (default BACKSLASH, a name or `0x2B`),
  `NV_COOP_MUTE_KEY` (F8, this player's microphone), `NV_COOP_DEAFEN_KEY` (F7, the others' voices); the
  game also sees these keys. Muting or deafening shows a HUD message. `NV_COOP_VOICE_VOLUME` (0 to 100),
  `NV_COOP_VOICE_MUTE=name,name` and `NV_COOP_VOICE_VOLUMES=name:percent` go by the names the endpoint
  knows (on a guest, the host's; another guest has no name there and keeps the defaults).
- **Status.** Every 5 s: frames sent, heard, dropped, not sent before joining, players with a place.
- **Tests.** `coop-engine-session` `voice::tests` (settings, keys, queue bounds, numbering, own frames,
  the mixing clock) and `tests/voice.rs` (a host and two guests in one process: every frame reaches every
  other player once and never its speaker; nothing before a guest is in; places from the ladder; voice
  without the ladder). `coop-voice`'s pipeline tests run through Opus with `--features opus`.
- **Not done.** Nothing has played through a real microphone or speaker in this engine. No in-game
  list to mute a player by name (only the variables). No mark for who is speaking.

## S37: Steam invites (implemented; tested on MockSteam only)

- **Host.** `NV_COOP_STEAM=on` (or `invite`) with `NV_COOP=host`: besides the UDP port, a friends-only
  Steam lobby on Spacewar (AppID 480) with the marker, protocol tag and the session token in its data
  (`coop_net::steam::link::host`). The host's endpoint carries both (`coop_net::Dual`: the UDP backend's
  peer keys are marked with bit 63, which a SteamID64 never has), so the UDP join code still works for a
  player without Steam, and the lobby's join code (`NVRS-...` naming the lobby) works for one with it.
  When Steam is missing the host hosts over UDP only and the log says why.
- **Invites.** `invite` opens the Steam overlay's invite dialog once, 3 s after this player is in the
  world; `NV_COOP_INVITE_KEY` (default F6) opens it again. The friends list's "Invite to game" works too
  (the lobby and the `connect` rich presence are set).
- **Guest.** `NV_COOP=steam`: before the game starts, the first lobby of a friend shown playing
  Spacewar in a lobby, or of an invite that arrives (callbacks 333, 337, 503), within two minutes; the
  token is read from the lobby's data. A join code naming a lobby joins it. Traffic then goes through
  ISteamNetworkingMessages: nobody forwards a port. Accepting an invite in the Steam client while this
  game is not running starts Spacewar itself: the friend starts "Join a Steam friend" first.
- **The library.** Valve's `steam_api64.dll` is never in the repository. The game program looks at
  `NV_COOP_STEAM_DLL` (the launcher sets the one next to `Play.exe`), next to itself (`app\`), then the
  folder above it (the package's), and says plainly which places it looked at when it is missing. The
  package ships it only when `third_party/steam/steam_api64.dll` has the pinned SHA-256 (`eb17909a...`,
  Steamworks.NET Standalone 2025.164.1, Windows-x64), the old package script's rule, and its `STEAM.txt`
  says which.
- **The overlay in this engine's window.** The game's window is the real Win32 window `d3d9real`'s
  window thread makes (class `nv-rs-d3d9`), drawn by a real Direct3D 9 device. Steam's overlay draws only
  into a device made after its library is in the process, so `coop_steam::early` starts Steam in
  `main` before `host::Host::start_with`. Its key (Shift+Tab by default) is an ordinary key message on
  that window; nothing in the window procedure swallows it (only Alt and F10 are kept from the system
  menu). Whether the overlay hooks this device has not been seen on a PC; the friends list is the way
  when it does not.
- **Tests.** `coop-net` `steam::link::tests` (a code with the token, a friend found with no code, an
  invite followed, nobody hosting said plainly, wrong lobbies refused, the handle's invite and dialog,
  where the library is looked for) and `dual::tests` (guests of two backends on one host, key marks,
  readiness). `RealSteam` itself, as before, only a two-account run can check.

## S38: the co-op launcher window (implemented with unit tests; not run on a PC)

- `Play Co-op.exe` (the package's second copy of `Play.exe`; `Play.exe --coop` too) shows a window
  before anything else: the player's name, the join code box, the address for the host's code
  (`NV_COOP_ADDRESS`), "Host on Steam too", voice chat (push to talk, open microphone, off) and the
  push-to-talk key, and three check boxes for the parts that are off unless asked (`NV_COOP_ITEMS`,
  `NV_COOP_VATS`, `NV_COOP_HARM`). Buttons: Play alone, Host, Join, Invite a Steam friend, Join a Steam
  friend; the Steam ones are off when `steam_api64.dll` is not next to `Play.exe`, and the window says so.
- The choices are kept in `userdata\coop.ini`, written after the launcher's own place checks (its first
  write). The window sets or clears all its switches for the game (`scripts/launcher/src/coop.rs`
  `SWITCHES`), over settings.ini's. A bad name, address or code is said in a message and the window stays.
- A host's launcher watches the game's log for the join codes, copies the direct one to the clipboard
  and shows both (the Steam one with how friends join).
- `Play Co-op.cmd` still works as before, as the fallback.
- Tests: `scripts/launcher` `coop::tests` (the file read back, the environment of each button, Steam
  without the library refused, codes cleaned and refused, the codes read from a game log). The window
  itself (`coop_ui.rs`) has never been shown.
