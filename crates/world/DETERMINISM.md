# Determinism of the world crate

Goal: the same seed and the same inputs give the same `GameState` on every
machine, so a host and a guest can compare `world::determinism::state_hash`.

## API

- `SimSeed` (`DEFAULT` = the dice a new game always had, so single-player is
  unchanged) and `GameState::set_sim_seed(seed)`.
- `SimSeed::stream(label)` / `GameState::sim_stream(label)`: independent
  seeded xorshift streams (`SimRng`) that do not advance the shared dice.
- `state_hash(&GameState)`: FNV-1a 64 over the canonical save text plus the
  transient fields the save omits (player cell, queued events, noise,
  hit targets).
- Tests: `cargo test -p world --lib determinism`.

## Audit result

Classification of what was found in `crates/world/src` (outside
`scripting.rs`).

Shared-state affecting:

- Randomness: every roll already goes through `GameState::roll()` (one
  xorshift stream in `GameState.dice`): hit chances, fall damage, leveled
  lists, dialogue random lines (`dialogue::choose(.., state.dice)`), crime,
  social, sandbox picks. No thread RNG, no `rand` crate. Nothing to replace;
  the stream is now settable via `SimSeed`.
- Wall clock: no `Instant`, `SystemTime` or `std::time` anywhere in the
  crate. (Only test temp dir names use the process id.)
- HashMap/HashSet order: the save writer sorts every set and map, so
  `save::save` is canonical. One real leak was found and fixed:
  `combat::advance_down` iterated `more.down` (a `HashMap`) and queued
  `Event::GotUp` in map order. It now sorts the actors first.
- Float accumulation: functions such as `combat::advance_down`,
  `noise::update`, `fatigue::advance` take `seconds` and subtract or add it.
  Results depend on the step sizes, not on the machine. Under host
  authority only the host steps; guests apply results.

Presentation only (may stay non-deterministic): `face.rs` blinks,
`particles.rs`, `music/director.rs`, `ai/offmesh.rs` and
`caravan/*` test generators, `grass.rs`, `decals.rs`, `animation/*`,
OnceLock/Mutex caches in `ai.rs` and `combat_ai.rs` (pure memoisation),
`TRIGGER_DEPTH` thread local (a recursion guard, reset to 0).
`idles::IdleAsker` seeds from the dice by value without advancing them.

## Remaining (not fixed here)

- `scripting.rs` (owned by the co-op agent): `GameState` itself;
  `variables: HashMap<FormId, Locals>` and other maps are sorted on save but
  any iteration inside `scripting.rs` must be checked; the dice are drawn at
  `scripting.rs` lines near 3153 (`self.state.roll()` once per run), 3546,
  3652, 3767, 4237 and 4925 (`GetRandomPercent`). If the per-frame advance
  at ~3153 depends on frame rate, it must be driven by simulation ticks, not
  frames.
- The viewer's frame loop (out of scope) decides how often systems tick and
  which `seconds` they pass. Co-op must apply host results rather than
  re-simulate on guests.
- `state_hash` covers what `save` writes. Fields neither saved nor listed in
  `state_hash` are not compared; add them when they matter.
- Float formatting in the save uses Rust `Display`, which is stable for equal
  bit patterns; cross-platform float behaviour (fused multiply-add, libm) was
  not audited.

## Rules for contributors

1. Never read the clock (`Instant`, `SystemTime`) in code that changes
   `GameState`. Take time as a parameter.
2. Never use a random source other than `GameState::roll()` or a `SimRng`
   derived from the seed. Do not draw dice from presentation code in a way
   that changes how many sim draws happen.
3. Never let `HashMap`/`HashSet` iteration order decide an event order, a
   chosen item, a roll order or a saved line. Use `BTreeMap`/`BTreeSet`, or
   collect keys and sort first.
4. Anything saved must be written in sorted order. New persistent fields
   need a `save` line so `state_hash` sees them.
5. Do not draw dice conditionally on presentation state (visibility, camera,
   frame rate).
6. Add a determinism test for new simulation systems: run twice, same
   seed, compare `state_hash`.
