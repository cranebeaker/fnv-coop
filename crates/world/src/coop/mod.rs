//! Co-op integration of the world rules (host-authoritative replication).
//!
//! `coop-core` is game-free and speaks FormID-keyed [`coop_core::schema::Field`]
//! records. This module is the `world` side of that bridge:
//!
//! * [`Authority`] on [`GameState`] (default `Single`: nothing changes for
//!   single player). A `Guest` runs no quest, item or reference scripts and
//!   cannot write shared state through the script `Host` trait.
//! * [`Effect`] sink: semantic mutation sites tell the sink which shared
//!   domain and subject changed. Only emitted when the authority is not
//!   `Single`.
//! * [`export`]: the canonical, sorted export of every shared domain (the full
//!   snapshot). It is also the keyed-diff backstop that catches writers which
//!   emit no effect (reported by [`bridge::HostBridge`]).
//! * [`apply`]: writes fields into a guest `GameState` without running
//!   scripts, queueing only local presentation events.
//! * [`backend::WorldBackend`]: the trait the viewer integrator uses.
//!
//! Shared domains (see [`Domain`]) exclude local-only state on purpose:
//! viewer reports, `events`, camera, UI, per-machine variables, the dice and
//! anything wall-clock driven.
//!
//! Wall-clock reads that cannot be fixed in this crate (for the viewer
//! integrator): `viewer/src/ai.rs` greeting cooldown (`real.elapsed()`),
//! `vats.rs` (`real.elapsed_secs_f64()`), `music.rs`, `movie.rs`, and the
//! hacking minigame seed (`SystemTime` nanos). All are local presentation or
//! per-player; their outcomes must go through host intents. The `dice`
//! xorshift is not shared state: it is excluded from every export and a guest
//! never advances it (`Runner::update` returns early on a guest).

use std::collections::BTreeSet;

use esm::FormId;

pub mod apply;
pub mod backend;
pub mod bridge;
pub mod campaign;
pub mod combat_live;
pub mod dialogue;
pub mod env;
pub mod export;
pub mod facts;
pub mod interact;
pub mod interact_acts;
pub mod live;
pub mod loot;
pub mod party_char;
pub mod party_save;
pub mod players;
pub mod policy_gate;
pub mod presentation;
pub mod roam_ops;
pub mod rules;
pub mod scene;
pub mod shadow;

pub use backend::WorldBackend;
pub use bridge::{CollectReport, GuestBridge, HostBridge};
pub use policy_gate::{Grant, PolicyGate};
pub use shadow::{PlayerShadow, ShadowGuard, ShadowPlan};

use crate::scripting::GameState;

/// Who is allowed to run the game's rules.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Authority {
    /// Single player (the default): everything runs locally, nothing is
    /// recorded for replication.
    #[default]
    Single,
    /// The host of a co-op session: runs everything, records effects.
    Host,
    /// A guest: scripts are suppressed, state arrives from the host.
    Guest,
}

impl Authority {
    /// True for a guest.
    pub fn is_guest(self) -> bool {
        self == Authority::Guest
    }

    /// True when mutations are being recorded for replication.
    pub fn replicates(self) -> bool {
        self != Authority::Single
    }
}

/// A replicated slice of `GameState`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Domain {
    /// `stages`.
    Stage,
    /// `stages_done`.
    StagesDone,
    /// `running`, `completed`, `failed`, `quests_announced`.
    Flags,
    /// `objectives` and the scripts' hidden-completed objectives.
    Objective,
    /// `globals`.
    Global,
    /// `topics`.
    Topic,
    /// `said`.
    Said,
    /// `talked_to`.
    TalkedTo,
    /// `notes`.
    Note,
    /// `disabled`.
    Disabled,
    /// `destroyed`.
    Destroyed,
    /// `items` (holder, item) counts.
    Items,
    /// `stocked` holders.
    Stocked,
    /// `variables` (script variables by owner).
    Vars,
    /// Karma (player actor value).
    Karma,
    /// Experience (player actor value).
    Experience,
    /// Player level.
    Level,
    /// `perks` and `perk_ranks`.
    Perk,
    /// `reputations` (fame, infamy).
    Reputation,
    /// `map_markers` (discovered, so fast travel is unlocked).
    MapMarker,
    /// `faction_changes`.
    Faction,
    /// `faction_relations` (`SetEnemy`, `SetAlly`).
    FactionRelation,
    /// Crime (`world::crime`): `crime_enemies`, `faction_crimes`, the
    /// player's crime counts, steal warnings and murderer flag.
    Crime,
    /// A player's shadow (`shadow::shadow_form`).
    Shadow,
}

/// One semantic mutation: a shared domain and the subject form it touched.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Effect {
    /// Which shared domain changed.
    pub domain: Domain,
    /// The subject (quest, global, holder, ref ...).
    pub form: FormId,
}

/// Counters for tests and diagnostics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    /// Script entry points refused on a guest.
    pub suppressed_runs: u32,
    /// `Host::call`/`set_var` writes refused on a guest.
    pub suppressed_calls: u32,
    /// Effects recorded (not counting dropped overflow).
    pub effects: u64,
}

/// Co-op bookkeeping carried by `GameState`.
#[derive(Clone, Debug, Default)]
pub struct CoopState {
    /// Who runs the rules (default `Single`).
    pub authority: Authority,
    /// Effects since the last host collect.
    pub out: Vec<Effect>,
    /// Set when `out` overflowed: the next collect must do a full diff.
    pub overflow: bool,
    /// Globals the game clock moves every frame (`GameHour`,
    /// `GameDaysPassed`); exported quantized so a 20 Hz stream is possible.
    pub clock: BTreeSet<FormId>,
    /// Counters.
    pub stats: Stats,
    /// Parked per-player shadows (see [`shadow`]); off by default.
    pub bank: shadow::ShadowBank,
    /// Campaign policy enforcement; `None` means no enforcement.
    pub gate: Option<Box<policy_gate::PolicyGate>>,
    /// Live-layer facts (actors, hits, rules); see [`facts`].
    pub live: facts::LiveFacts,
    /// Player-world interaction facts; see [`interact`].
    pub interact: interact::InteractFacts,
    /// Campaign gaps (attribution, owners, storage, downed ...); see [`campaign`].
    pub campaign: campaign::Campaign,
}

/// Longest effect queue before falling back to a full diff.
const MAX_PENDING: usize = 1 << 16;

/// Clock globals are exported on a 1/64 grid (exact in f32, idempotent).
pub fn quantize_clock(v: f32) -> f32 {
    (v * 64.0).floor() / 64.0
}

impl GameState {
    /// Records that `domain` of `form` changed. Zero cost for `Single`.
    #[inline]
    pub fn emit(&mut self, domain: Domain, form: FormId) {
        if self.coop.authority != Authority::Host {
            return;
        }
        if self.coop.out.len() >= MAX_PENDING {
            self.coop.overflow = true;
            return;
        }
        self.coop.stats.effects += 1;
        self.coop.out.push(Effect { domain, form });
    }

    /// The co-op authority of this state.
    pub fn authority(&self) -> Authority {
        self.coop.authority
    }

    /// True on a guest.
    #[inline]
    pub fn is_guest(&self) -> bool {
        self.coop.authority == Authority::Guest
    }

    /// Switches authority. Entering `Guest` or `Host` clears pending effects.
    pub fn set_authority(&mut self, a: Authority) {
        self.coop.authority = a;
        self.coop.out.clear();
        self.coop.overflow = false;
    }

    /// Seeds the dice (xorshift state must be non-zero). The dice are not
    /// shared state: a host may re-seed per session for reproducible tests.
    pub fn reseed_dice(&mut self, seed: u64) {
        self.dice = seed.max(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_records_nothing() {
        let mut s = GameState::default();
        s.emit(Domain::Stage, FormId(1));
        assert!(s.coop.out.is_empty());
        s.set_authority(Authority::Host);
        s.emit(Domain::Stage, FormId(1));
        assert_eq!(s.coop.out.len(), 1);
    }

    #[test]
    fn quantize_is_idempotent() {
        for v in [0.0f32, 10.123457, 23.999, 1234.5678, 77.0001] {
            let q = quantize_clock(v);
            assert_eq!(q, quantize_clock(q));
            assert!(q <= v);
        }
    }

    #[test]
    fn coop_dice_reseed() {
        let mut s = GameState::default();
        s.reseed_dice(0);
        assert_eq!(s.dice, 1);
    }
}
