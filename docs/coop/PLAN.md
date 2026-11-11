# Co-op on the translated engine: the plan

Goal: [GOAL.md](GOAL.md). Decision: [DECISIONS.md](DECISIONS.md), D1 (hybrid H). Specs: [SPECS.md](SPECS.md).

## Strategy

The story runs where the game runs: in the translation of FalloutNV.exe (`crates/engine*`), started by
`crates/host` and drawn by `nv-viewer`. Co-op adds three layers on top. None of them changes how the game
plays when co-op is off.

1. **Session layer (engine-agnostic; already written and tested in fnv-coop).** `coop-core` (deltas,
   snapshots, shard hashes, authority, interest, transport framing, rules), `coop-net` (UDP, Steam),
   `coop-policy` (the 487 quest rules), `coop-voice` and `nv-launcher`. These moved over unchanged.
2. **Bridge layer (`crates/coop-engine`, new).** This is the only crate that touches the engine. It is
   installed by the host as the fourth extension owner (after fix rows, `nvse` and `jip`) and takes part
   through ADR-0016's wrap and seam tables. It has three state channels:
   - **Story channel (generic).** The host's change journal (a wrap of `BGSSaveLoadGame::AddChange`)
     names every form that changed. A form type that has passed its live-apply proof travels as its
     save-game record. Until then it travels as a field delta through a direct engine call.
   - **Globals channel.** Every global's value is compared each tick.
   - **Live channel.** Player and actor movement, animation, combat events, dialogue UI, V.A.T.S., menus.
     These are per-feature hooks, because none of them are save state.
3. **Presentation layer (in nv-viewer and the host).** Remote players as actors, name tags, the join UI,
   subtitles and the party HUD, drawn through the game's own menus and actors rather than a second
   renderer.

## Milestones (strategic to granular)

Each milestone ends with exit criteria a test or a run on the owner's PC can check. "PC" means a Windows
machine with FNV 1.4.0.525. "Headless" means `cargo test` with no game files.

| # | Milestone | Exit criteria |
| --- | --- | --- |
| M0 | **Base**: branch `coop-next` cut from upstream main 815b187, carrying the five agnostic crates | Workspace builds; `cargo test -p coop-core -p coop-net -p coop-policy -p coop-voice -p nv-launcher` green; repo hygiene green; `Cargo.lock` regenerated |
| M1 | **Bridge core**: `coop-engine` with the `Coop` owner, the change journal, form lookup, quest stage and global access, and `NV_COOP` install in the host | Headless unit tests: the journal records every `AddChange` with its flags, only while armed, and leaves the wrapped call's behaviour and return unchanged (twin: with the wrap removed the record test fails); stage and global accessors round-trip on in-memory forms. PC: `NV_COOP=journal` prints the quest change when Doc Mitchell's quest advances |
| M2 | **Engine backend for conformance**: `coop-headless` split off `world` (new `coop-model` crate for `PartyRules`, `ShadowPlan`, `PlayerShadow`, `apply::Ctx`); `EngineBackend` returns a real peer | The seven conformance rows run on `--backend engine` on the PC and pass; the twins fail |
| M3 | **Two engines, one story**: host-to-guest story channel for quests (stage records) and globals, guest quest scripts gated at `ScriptRunner::Run` | PC, two processes over loopback: the host plays the opening to Goodsprings; the guest's quest log, globals and shard hashes match at every stage; zero quest scripts run on the guest |
| M4 | **Live-apply proofs per form type**: quests, globals, notes, topics, factions and reputation, then containers and enable state, then references, actors and cells | Per type: a round-trip test (host save record, then guest apply, then guest save record, byte-identical, decoded by `crates/fos`) and a 30-minute soak with no unrepaired hash drift |
| M5 | **Seeing each other**: remote player avatar as an actor, movement and animation stream, name tags | PC, internet: both see each other walk Goodsprings at 20 Hz or better, avatar never AI-driven |
| M6 | **Playing together**: combat authority, downed and revive, loot once, doors and cells, dialogue session (host drives, guest sees the options), barter, lockpick, hack, V.A.T.S. real time, sleep and wait vote | The fnv-coop checklist rows for each feature, ported to the engine backend, green with their twins; PC playtest of the gecko fight, the Goodsprings shootout and the Prospector Saloon dialogue |
| M7 | **Starting and saving**: guest character creation, party saves (host save plus per-player save), rejoin, late join from a snapshot | A party saved and continued; a guest dropped mid-quest rejoins with matching hashes |
| M8 | **Whole story**: base game main quest to every ending, then Dead Money, Honest Hearts, Old World Blues, Lonesome Road; DLC FormIDs resolved by load order | Recorded two-player runs reach each ending; all 709 checklist rows green on the engine backend |
| M9 | **Release**: packaged launcher, join codes, Steam invites, voice wired in | A player with no build tools hosts and joins from the package |

## Status (2026-11-09)

| # | State |
| --- | --- |
| M0 | Done here: branch, agnostic crates, `Cargo.lock`; their tests pass (four nv-launcher tests expect a Windows install and fail on Linux, as before) |
| M1 | Done, confirmed on a PC (2026-11-10): the journal names the opening's quests, objectives and the DLC start quests |
| M2 | Superseded for M3 by D2's driver; the checklist's engine backend follows |
| M3 | Done: two games on a PC, the host played the opening, the guest held its story (3883 hash checks, 0 desyncs, 0 refused; 2026-11-10) |
| M4 | Quests, objectives, quest variables and globals carried as fields. The generic changed-form stream is written (SPECS S12): the host sends a changed form's own save record, a guest's game loads it, for placed objects (enable state, place, door, lock), containers' contents, factions, reputations, notes and dialogue responses said once. It has unit tests on stand-ins for the game's functions (a round trip per kind of form, read by `crates/fos`; two and three test games over loopback) and has not run on a PC: no kind of form has its live-load proof yet (D1, condition 1), and `NV_COOP_WORLD=doors` or `off` takes the records out of a run. Records larger than one field (512 bytes) are not sent. Actors and cells are not carried |
| M5 | First version confirmed on a PC (2026-11-10): both players made their own characters, all five rungs passed on both games, each saw the other move (20002 avatar moves, AI woke 0, 0 desyncs, 0 healed). Not yet exercised: the move of a guest to a host in another space (they started in the same cell). Next: animation, the other player's looks, leaving the house together |
| M7 | First step confirmed on a PC (2026-11-10): a guest played its own opening and character creation while the shared story stayed in step. Next: party saves, rejoin into a running game |
| PC run 3 | 2026-11-10, both players outdoors after the opening. Worked: the guest was moved to the host in another space (`warps 1`). Three faults, fixed with tests: the guest kept the avatar it had placed indoors (now placed anew when its player's space changes); day on the host and night on the guest (the clock is now shared, the host's about once a second); the guest's Pip-Boy lacked the shared quests (objectives now go through the game's own `SetState`, which is what lists them). Awaiting the retest |
| PC run 4 | 2026-11-10. Confirmed on a PC: the same quest list in both Pip-Boys, the same clock, each player seeing the other across four moves between spaces (`warps 4`, avatars placed 5 times), 10217 hash checks with 0 desyncs. Three faults, fixed with tests (D4): no pre-order pack items on the guest, the guest's opening quest never completed, an NPC dead on the host alive on the guest. Awaiting the retest |
| M6 | Started: deaths of placed actors are shared both ways (D4). Doors and locks are shared both ways (SPECS S12): a door a player opens or shuts, and a lock picked, opened from a hacked terminal or broken by forcing (level, flags, tries), go through the host's game to everyone; unit tests on stand-ins only, not run on a PC. Not started: the fight itself (NPC positions and damage), dialogue. Loot and barter: a container the host's player changes reaches the guests as its record, but a guest's take or trade does not go to the host yet, so loot can still be taken twice and a trader's stock and caps are each game's own |
| M6, fights | Shared fights (SPECS S13) are written and have unit tests (the host's actors held on a guest; hits through the host both ways; an enemy turning on a guest; downed and revive), but have not run on a PC. V.A.T.S. for several players: the row after the next. Not started here: loot, dialogue, barter |
| M6 | Started: deaths of placed actors are shared both ways (D4). Not started: the fight itself (NPC positions and damage), loot, dialogue, barter |
| M6, V.A.T.S. and the rest of the fights | Written with unit tests, not run on a PC (docs/coop/SPECS.md S20, S21): with another player in the session a player's V.A.T.S. neither stops nor slows that player's game, and the queued shots go through the hit path; health lost outside `Actor::HitMe` (a fall, a trap, drowning, an effect) on the host's actors and on players goes through the host; an actor the host's game does not run has the guest whose game runs it as its one authority, and the others hold it; held actors move between two states in steps. 7 tests in the bridge, 2 unit and 15 multi-game tests in the session. Not done: followers (the repository does not show how a follow package is made to follow another reference than the player), the game's own walk and run animation on a held actor, a presentation speed of its own for the player in V.A.T.S. |
| M6, M7 social | Written with unit tests, not run on a PC (docs/coop/SPECS.md S14 to S17): conversations (the talker drives, the others see each line as a HUD message, a guest's results run once in the host's game, one conversation per NPC); party saves (a file per guest next to the host's save: place and own quests; given back on a load); rejoin and late join with the saved part; the sleep and wait vote (all agree, every game passes the hours, the host's clock holds). 13 tests in the bridge, 6 unit and 14 multi-game tests in the session. Not in a part yet: the character's values and the inventory. Not started: followers |
| M8, M9 | Not started |

## Ownership and order

M0 and M1 come first, because every later milestone stands on them. M2 runs next to M3. M4 gates the
generic story channel, type by type. M5 to M7 follow each other's dependencies. M8 is a sweep that feeds
fixes back into M4 to M7.

## Limits stated plainly

- Co-op cannot be more complete than the engine under it. Where nv-rs does not yet run a part of the game,
  the fix belongs upstream (we contribute it there), not in co-op.
- The upstream maintainer has said co-op is intended but has no design. This plan is offered upstream as an
  issue once M1 runs on a PC.
