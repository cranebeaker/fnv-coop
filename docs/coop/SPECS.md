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

## S12: the rest of the world (implemented on stand-ins; docs/coop/DECISIONS.md D1, option C)

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

## S13: shared fights (implemented with unit tests; not yet run on a PC)

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
run animation. V.A.T.S. for several players, damage outside `Actor::HitMe`, the actors the host's game does
not run and the steps between two states of a held actor are S20 and S21.

**Verified here:** 7 unit tests in `coop-engine` (`fight.rs`) and 17 two-game tests
(`coop-engine-session/tests/fights.rs`); in each the story channel's echo, desyncs, refusals and guest script
count are unchanged. **Not verified here:** every game call above (stand-ins only), and whether the game's
own combat lets an actor fight an avatar whose AI is off.
## S14: conversations (implemented with unit tests; not run on a PC)

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

## S15: party saves (implemented with unit tests; not run on a PC)

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
- **Limits, stated.** The character's values and the inventory are not in the part yet: no function
  of the repository was established for reading and writing them here, so a guest keeps those in its
  own game and its own `.fos`. A guest's part is as old as its last upload (at most 5 s). The party
  folder is not the save folder: the platform layer keeps saves, and co-op writes plain files.

## S16: rejoining and joining late (implemented with unit tests; not run on a PC)

- A guest that joins a running game gets coop-core's snapshot, which now also holds the published
  parts; one that lost the host resumes from its replica (S7). Either ends with the host's hashes.
- When a guest leaves, the host publishes its last part, marked as left behind. A guest takes such a
  part only in a game that just started (its driver's first join): a guest that merely lost the
  connection keeps what its still running game has; one whose game was started anew gets its place and
  own quests back. A part from a load is always taken.

**Verified here (S15, S16):** 5 unit tests of the format (round trip, unknown sections, refusals, chunks,
file names) and 4 multi-game tests: a save writes one file per guest and a load gives each part back
once (place and own quests, files unchanged by later play); a player who joins late gets the snapshot
and its saved part while a stranger gets none; a dropped guest keeps its game while a restarted one
gets its part back; a guest at its main menu gets its part once it is in the world. Hash checks ran
with 0 desyncs in each. **Not verified here:** the seams in the real game, the real save names, the
move of a player to a saved place across cells.

## S17: sleep and wait (implemented with unit tests; not run on a PC)

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

## S20: V.A.T.S. with several players (implemented with unit tests; not run on a PC)

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
  the host's does not (S21), so neither may stand still. `coop_engine_session::vats::Share` gives the
  game the world's speed when the first other player is there and takes it back when the last one left
  or the session ends. `NV_COOP_VATS=slow` shares the speed only (the menu still stops the game), `off`
  nothing.
- **The queued shots** are played by the game as attacks. Each lands through `Actor::HitMe`, so a guest's
  shot at a held actor goes to the host's ledger as any hit does (S13), and the host's own go to its game.
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

## S21: the rest of shared fights (implemented with unit tests; not run on a PC)

### Damage that does not pass `Actor::HitMe` (`coop_engine::harm`)

- An explosion damages an actor through `Actor::HitMe` (`explosion.rs`), so S13 already has it.
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
- **Limits, stated.** What made the loss is not read; it is carried as coop-core's damage over time, which
  is neither scaled for the party nor tied to a limb. A held actor does not fall on a guest at all (its AI
  is off); its fall happens in its owner's game. Gains of health are not routed. Other values than health
  (limbs, fatigue) are each game's own. If the player's class has its own function in slot `0x3ac` that
  does not end in `00881130`, the player's losses do not pass the wrap: the repository shows none.

### Actors the host's game does not run (`coop_engine_session::far`)

- The host's game is the authority for every actor it runs (S13). An actor it does not run belongs to the
  guest whose game runs it in that guest's own space and reported it first (`ActorMsg::Track`, once a
  second, only actors nobody owns). The host enters it in coop-core's ledger pinned to the reporter
  (`Ownership::set_follower`), and the ledger's own rules announce the owner. A report is the fights' to
  judge: it is not passed on to coop-core's own handling, and one for another space than the reporter's,
  for an actor the host runs, a dead one or an owned one is dropped.
- The owner's game goes on running the actor and sends its state when it changed (about 20 times a
  second). The ledger takes the owner's health and death from the state. Every other guest in that space
  holds the actor as it holds the host's.
- Another player's hit is judged by the ledger; its verdict (`Damaged`, `Died`) reaches the owner, whose
  game applies it. The host's game applies damage only to actors it runs. A death in the owner's game also
  reaches the story as any guest's death does (D4).
- When the host's game comes to run the actor, the host is pinned as its owner and the ledger hands it
  over; when it stops, nobody owns it and the guests report it again.
- **Limits, stated.** An owner who walks away keeps the ledger's entry until the ledger gives the actor
  to another player in that space or to nobody. Two guests in one space who both run an actor: the first
  report wins. The host's own copy of a guest's actor is not moved. Threat is not directed for these
  actors.

### Steps between two states (`coop_engine_session::smooth`)

- A held actor gets coop-core's `avatar::InterpBuffer`: the newest state is fed at the avatars' rate and
  the actor is put where the buffer says, 100 ms behind and blended between two states. Another space or
  a long jump is taken at once. `Fight::smooth_on` switches it off.
- **Not done:** the game's own walk or run animation on a held actor. The avatar code's approach
  (restrained, moved by the game's own mover) was not examined for held actors.

**Verified here (S21):** 4 tests in the bridge (`harm.rs`), 2 unit tests (`smooth.rs`), 3 two-game tests
(`tests/vats.rs`: a held actor's loss, an effect on an avatar, the twin with state only) and 7 three-game
tests (`tests/far.rs`: a guest's actor held by another guest; one owner for two; another player's hit and
kill through the owner; a death in the owner's game; the host taking over and giving back; a report for
another space; the steps and their twin). **Not verified here:** every game function above in the real
game.

## Not done: followers

A companion that follows the player who hired it, as the host's actor, was not started. The translation
shows the teammate byte (`Actor::bPlayerTeammate`, +0x18d) and `Actor::SetPlayerTeammate` (`actor_p6.rs`),
but following is a package whose target is the player, and nothing read here shows how a package is made
to follow another reference (a guest's avatar) without changing the package for every actor that uses it.
