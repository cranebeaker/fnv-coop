# Goal: whole-story co-op (improved brief)

This is a tightened version of the goal the project owner gave on 2026-11-09. It makes "fully working" testable.

## Outcome

Two to four players finish Fallout: New Vegas together over the internet: the base game from Doc Mitchell's
house to every one of the four endings, plus Dead Money, Honest Hearts, Old World Blues and Lonesome Road.
One player hosts and the others join with a join code. Everyone plays in the same world, and nobody has to
quit to single-player to get past a quest.

## What "100% functional" means

Each of these must pass on two real Windows PCs, each with its own copy of FNV 1.4.0.525:

1. **Story completion.** Every main quest and every DLC main quest can be completed in a co-op session,
   with either player triggering the stages. Proof: a scripted run reaches each quest's final stage, and the
   guest's quest log matches the host's.
2. **Per-player state stays separate.** Each player keeps their own inventory, skills, perks, karma and
   reputation, and gets their own rewards. Proof: the existing `crates/coop-policy` rules hold on the
   **engine** backend, not just on the old `world` backend.
3. **Shared world.** NPCs, creatures, containers, doors, cells, weather and time are the same for everyone.
   Proof: the shard hashes match after every scripted step, with no unrepaired drift.
4. **Playability.** The guest sees the host's dialogue choices, can fight and die and be revived, and V.A.T.S.
   never freezes the other player. Party saves resume a campaign.
5. **Robustness.** A session survives packet loss and lets a dropped player rejoin. It refuses mismatched
   game versions or mod lists.

## Constraints

- Every change follows from a stated need and is checked: a test, and where the checklist asks for one, a
  mutation twin. No shortcuts that only make a test pass (no stubbed behaviour reported as working).
- Behaviour matches the original game, using fnv_decomp and the exe as the reference, as nv-rs requires.
- The project stays able to take upstream nv-rs updates. Co-op must not depend on code that upstream has deleted.
- Work goes from strategic to granular: architecture first, then subsystem specs, then code.
- When a decision is hard: three independent proposals, a factor-scored comparison, a three-member
  committee where two votes win, and a record of the decision in `DECISIONS.md`.
- Peer review only when the owner asks for one (the owner's instruction of 2026-11-11, evening; it
  replaces the same day's rule of a review at every milestone, which cost more than it found). When asked:
  independent reviewers who did not write the change read it against its need, its tests and the PC
  evidence; each finding is fixed, or recorded as open with its reason, in the milestone's row of `PLAN.md`.
- Push at each finished milestone.

## Prerequisites outside the code (owner action)

- A GitHub account with write access, linked to Claude (to push).
- Network/package access for the build session (so `cargo build` and the headless tests can run).
- A Windows PC with the game for the real-game rows (the simulator also needs the game's `Data` folder).
