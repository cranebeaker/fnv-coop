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
