# Co-op decisions

Each decision went through three independent proposals, a factor-scored comparison and a three-member
committee (two votes win). This file holds the decisions as they were reached.

## D1 (2026-11-09): where co-op lives, and how state travels

**Question.** fnv-coop's co-op was built on the hand-written `world` crate and the Bevy viewer. Upstream nv-rs
removed both on 2026-11-03 (ADR-0011). Those layers only ever played Doc Mitchell's house, the opening and
Goodsprings. Only the translation of FalloutNV.exe can run the whole story. What should co-op stand on, and
how should game state travel between players?

**Proposals.**

- **A: engine backend port.** A `coop-engine` crate implements the existing `BackendPeer` seam
  (`crates/coop-headless/src/backend.rs`) with direct calls into the translated engine. Changes are captured
  with a separate hook for each feature.
- **B: upstream first.** A new branch cut from current upstream main. The engine-agnostic crates
  (`coop-core`, `coop-net`, `coop-policy`, `coop-voice`, `nv-launcher`) move over unchanged, `coop-session`
  and `coop-headless` are split off `world`, and one bridge crate connects them. Co-op takes part through
  ADR-0016's wrap and seam tables as a new owner.
- **C: changed-form streaming.** The host tracks the forms the game itself marks as changed for the save
  game, and sends each one's save record. Guests load the records. All story state is covered by
  construction, and the result can be checked against real `.fos` saves.
- **H: hybrid.** B's structure, A's direct calls for the first conformance rows and for actors and
  presentation, and C's stream as the generic channel for story state. Per-feature hooks only where save
  records do not apply: movement, combat events, the dialogue UI and V.A.T.S.

**Factors and weights.** Whole-story coverage 0.30, functional correctness and verifiability 0.25,
feasibility 0.20, upstream compatibility 0.15, effort 0.10.

| Member | A | B | C | H | Vote |
| --- | --- | --- | --- | --- | --- |
| 1 (sceptic, checked addresses) | 6.20 | 6.80 | 6.50 | 7.15 | H |
| 2 (pragmatic, maintainability) | 6.10 | 6.80 | 6.70 | 7.50 | H |
| 3 (correctness and risk) | 5.50 | 6.80 | 6.10 | 7.20 | H |

**Decision: H, 3 votes to 0.**

**Conditions attached by the committee (binding):**

1. Before the changed-form stream carries a form type, applying that type's record to a guest that has the
   object loaded must be shown to work, one type at a time: quests and globals first, actors and cells
   last. Each type gets a round-trip test against `crates/fos`. Until a type passes, the per-feature hook
   stays its channel.
2. An engine-backend checklist row may only be counted green once it passes against the engine. A row that
   passes only on the old `world` backend never counts for the engine.
3. The nv-rs revision is pinned, and the suite reruns on every upstream bump.
4. Co-op takes part through ADR-0016's tables as its own owner, not by patching addresses at random.

**Facts checked after the vote:**

- Member 1 worried that `MarkAsModified` is virtual, so tracking changes would mean a hook in every form
  class. It does not. Every class's change call (`TESForm::AddChange` `00484b60`, `TESForm::ForceChange`
  `00484b90`) ends in the one non-virtual `BGSSaveLoadGame::AddChange` (`0084a690`), with the words
  `(save/load game, form, flags, force)`. One wrap there sees every change. (`SetStageDone` `0060d510`
  flags its quest with `0x80000000` through it, as the translation's own tests show.)
- Global variables do not travel as changed forms. The save writes them in their own block. Co-op therefore
  replicates globals by comparing every global's value each tick, not through the change journal.
- `Host::start_with` has no install point for outside crates. The fork adds one call, `coop_engine::install`,
  next to `nvse::install`. It does nothing unless `NV_COOP` is set.

## D2 (2026-11-09): the session layer on the engine

**Question.** coop-session (12k lines) drove the old world layer. How does the session go onto the engine, so
that M3 (two processes sharing the opening) is reachable with as much as possible verified headlessly first?

**Proposals.** A: make the old Session generic over a `CoopWorld` trait and bring about 4k lines over. B: a new
thin driver crate (`coop-engine-session`) on coop-core's `HostSync`/`GuestSync`, coop-net and coop-engine, with
each quest's whole record (stage, flags, done-stage bitmap) as one `Field::Custom`, an "engine echo" check of the
guest engine itself, and two-fake-engine loopback tests. C: an `EngineWorld` mirroring the old backend, a testkit,
a small link crate, and traces recorded on the PC replayed here.

**Factors.** Correctness and verifiability here 0.30, path to the whole story 0.25, effort 0.15, reuse 0.15,
maintainability 0.15.

| Member | A | B | C | Vote |
| --- | --- | --- | --- | --- |
| 1 (sceptic) | 6.65 | 7.90 | 7.10 | B + C's record/replay |
| 2 (pragmatist) | 7.25 | 7.05 | 7.40 | A spine + B's record + C's replay |
| 3 (correctness) | 6.80 | 7.50 | 6.95 | B + C's record/replay |

**Decision: B plus C's record/replay corpus ("B+R"), 2 votes to 1.**

**Deciding fact (checked in code).** `StateStore::delta_since` cuts a delta after `max_changes` keys, and
`HostSync::tick` halves that down to 1 when a message does not encode. With a quest's stage, flags and done
stages as separate keys (old encoding, A and C), a guest can hold a stage without its done set for a tick: a
state the host never had. One key per quest cannot be split.

**Binding conditions.**
1. A loopback test forces one key per delta and, after every applied delta, reads the guest *engine* (not its
   store) and finds each quest's stage, flags and done set equal to a record the host actually had.
2. The same test proves no story script ran on the guest.
3. Stage numbers are bytes in the engine (`TESQuestStage::ucIndex`), so a 256-bit done bitmap covers them all;
   the codec refuses malformed records.
4. Noted from the minority: later milestones bring over coop-session's ext, start, party and live pieces as
   they are needed, rather than rebuilding them blind.

## D3 (2026-11-10): the guest in the world, and players seeing each other

**Question.** The story channel works between two real games (PC run, 2026-11-10: 3883 hash checks, 0
desyncs, 0 refused), but the guest sat at its main menu. How does a guest get into the world next to the
host, and how is a remote player shown?

**Proposals.** All three: no own guest character in the first version; the guest presses New Game (its
opening is gated) and is moved to the host; a remote player is an actor with its AI off, moved each frame
from coop-core's avatar interpolation. A: script-command bodies (PlaceAtMe worker, CenterOnCell). B: direct
engine calls (CreateReference, MoveTo's cell-change path, full transform apply, a vanilla NPC as the base).
C: a "ladder": one PC run climbs five logged rungs (read own state; receive the peer's; warp the guest;
create the avatar; move it), capped by an environment variable so a crash at a rung is retried lower
without a rebuild.

**Factors.** Works in the real game and diagnosable in one run 0.35, headless verifiability 0.20, path to
combat, dialogue and an own character 0.20, safety of the proven story channel 0.15, effort 0.10.

| Member | A | B | C | Vote |
| --- | --- | --- | --- | --- |
| 1 (sceptic) | 6.45 | 6.30 | 7.65 | C hybrid |
| 2 (pragmatist) | 6.45 | 6.30 | 7.85 | C hybrid |
| 3 (correctness) | 6.30 | 5.45 | 7.65 | C hybrid |

**Decision: C, 3 votes to 0**, with A's base-form override (`NV_COOP_AVATAR=<hex>`). B's direct creation,
MoveTo arrival and full transform apply follow once the ladder passes.

**Binding conditions.**
1. Each rung writes fixed-format lines: `COOP-RUNG n entering` (flushed before its first engine call) and
   `COOP-RUNG n PASS|FAIL <reason>`, so a hard crash still names the rung.
2. The status line keeps the story counters; a run capped at the log rungs must still show 0 desyncs and 0
   refused.
3. The avatar's reference never enters the story channel (a headless test asserts the replica is unchanged
   by creating, moving and removing it), and it is disabled when its player leaves or the session ends.

## D4: deaths, the generic timer and guest-local scripts (PC run 4, 2026-11-10)

**Question.** After the opening, a guest lacked the pre-order packs' items, its opening quest never
completed, and an NPC the host had killed was alive and trading in the guest's game.

**How it was decided.** From the game's own data (the user's `FalloutNV.esm` and pack plugins, read for
this), not by the three-planner committee: each cause is one script, and the fix follows from what that
script does. The claim path below is the first guest-to-host action and is to be reviewed by the committee
when combat authority (M6) is designed.

**Findings.**
- The packs' items come from four quests (`VPreorderMercenary`, `-Caravan`, `-Tribal`, `-Classic`), whose
  scripts add the items to `Player` once, guarded by `VDLCPackQuest`'s variables. The rules had
  `VDLCPackQuest` as per-player but not the four quests, so their scripts were gated on a guest.
- The opening ends in `VGenericTimer`'s script (event 3: `SetStage VCG01 200`, `SetStage VCG02 5`), started
  by Doc Mitchell's last line. The timer was a shared quest: gated on a guest, and the variables the
  guest's dialogue set on it were put back to the host's.
- Nothing carried an actor's death.

**Decisions.**
1. The four pack quests are per-player rows. The gate also runs the rules' `guest_local` scripts on a guest
   whatever their quest.
2. `VGenericTimer` is per-player: a timer belongs to the player whose script started it. Its events act on
   that player (controls, fades, the end of the player's own opening). What an event writes to a shared
   quest on a guest is put back by the heal, as for any script a guest runs; the host's own timer does the
   shared part.
3. A placed actor's death is shared both ways. `Actor::SetLifeState` flags the actor's change (0x400), so
   the journal has it. Host to guest: a `Field::Custom` (`LIFE_TAG`) on the actor's reference id; the guest
   kills its copy with `Actor::Kill` inside the apply scope (the death's scripts ran on the host). Guest to
   host: the same field as a *claim* in an intent; the host's game does the kill, ungated, and the death
   then reaches every guest through the journal. coop-core's host keeps claims apart from the store
   (`claim_tags`, `claims`): a claim is a report for the host's game to judge, never a write.

**Limits, stated.** Only placed actors (ids below `ff000000`) are shared, not ones made while playing. A
death before the session started is not sent. Where the body lies, what it carries, and the fight itself
(each game still runs its own NPCs) are not shared yet. A guest's game can kill an actor the host's game
would not have (its own NPCs' fights); the host accepts it. An actor alive again on the host is not raised
on a guest.

## D5: a guest's actions and the host's world (PC run 5, 2026-11-10)

**Question.** A guest cannot move the shared story: its talk with Sunny Smiles advanced VCG02 in its own
game and the heal put it back 29,000 times. Scenes (Joe Cobb and Trudy, a trigger's `StartConversation`)
and NPC behaviour exist in the host's game only. Scripts say `player` and mean the local player.

**Options (three independent planners).** A: the guest's world is a mirror: guest AI off, a host actor
stream, every guest action forwarded and enacted on the host (`TESTopicInfo::RunResult` `0061f170`,
`ScriptEventList::SetActionFlag` `005ac750`, `Actor::HitMe` `0089a760`). B: two simulations: a shared write
made by a guest's script is captured by the guest's own journal at the moment of the write and sent as a
claim the host enacts; scenes as host-to-guest directives; actors reconciled softly. C: "drift is a
proposal": the heal's diff is the guest's write-set; tag it with its cause, send it, hold the heal until the
host's echo; scenes as host commands, the guest dropping its own.

**Factors.** Works in the real game and diagnosable 0.35, whole story and DLCs 0.25, headless
verifiability 0.15, effort 0.15, safety of the proven story channel 0.10.

| Member | A | B | C | Vote |
| --- | --- | --- | --- | --- |
| 1 (sceptic) | 3.50 | 6.25 | 7.15 | C's ladder with B's capture for stages and objectives |
| 2 (pragmatist) | 4.35 | 6.70 | 7.65 | C first, B's capture where the census shows the need |
| 3 (correctness) | 4.60 | 6.95 | 6.90 | B's capture, C's ladder and backstop |

**Decision: the hybrid, 3 votes to 0.** C's ladder (census; proposals with a bounded heal hold; host scene
commands with the guest dropping its own actor-to-actor `StartConversation`), with B's journal capture and
cause for stages, objectives, enabled state and life, and the heal's diff as the capture for variables and
globals and as the backstop. A is rejected: on the host `player` is the host's player, and the effects that
would go to the wrong person are silent. A per-script "host runs it" stays as the fallback both plans share.

**Binding conditions.**
1. Rung 1 (the census) ships alone and changes no behaviour: one `COOP-DRIFT` line per distinct drift with
   its cause or "cause unknown". Rung 2 follows a PC run that shows VCG02's drift with its cause.
2. Every proposal has an id and ends in exactly one logged outcome: confirmed by the host's echo, rejected,
   or timed out and put back. No heal hold outlives its timeout (headless test with a silent host).
3. Exactly once: the host ignores a repeated claim (headless test: three sends, one enactment, one result
   script run).
4. No lost updates: counters go as deltas or compare-and-set (headless test: both sides increment, both
   count).
5. No double scenes: a guest's own `StartConversation` outside a host command is dropped and logged
   (headless test: a host command and a local trigger together play once).
6. Proposals and scene commands have separate switches; with both off the story channel behaves as in run 5.

