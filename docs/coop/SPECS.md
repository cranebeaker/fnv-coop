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


## S12: shared fights (implemented with unit tests; not yet run on a PC)

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

**Limits, stated.** Only placed actors (ids below `ff000000`) among the up to 50 the host's game runs near
its player; an actor made while playing, or one near a guest who is elsewhere, runs in each game by itself.
A held actor slides without animation and is not interpolated between states. A hit's kind and limb are not
read (a direct hit, no limb), so limbs are not crippled through this path. Damage that does not go through
`Actor::HitMe` (explosions, falls, effects) stays local to the game it happens in. Friendly fire is off
(coop-core's default). A revive needs no key: standing next to the downed player starts it. A respawn does
not move the player. Only the controls byte is written for a downed player, not the rest of what
`DisablePlayerControls` does.

**Not done.** V.A.T.S. for two players (the host's own V.A.T.S. still stops or slows its game, and so the
actors everyone sees): the time multiplier's setter (`00aa4db0`) and the V.A.T.S. mode (`011f2250` +8) are
in the translation, but what stops the game while a target is picked was not traced in this work, so
nothing was wired rather than half of it.

**Verified here:** 7 unit tests in `coop-engine` (`fight.rs`) and 17 two-game tests
(`coop-engine-session/tests/fights.rs`); in each the story channel's echo, desyncs, refusals and guest script
count are unchanged. **Not verified here:** every game call above (stand-ins only), and whether the game's
own combat lets an actor fight an avatar whose AI is off.
