//! V.A.T.S. with several players (docs/coop/SPECS.md S20; coop-core's `combat::vats`).
//!
//! coop-core's model has two clocks: the world's, which everyone shares, and the acting
//! player's own presentation. `vats_scales(shared, wanted)` gives the world speed 1.0 in a
//! session whatever V.A.T.S. wants, and the wanted speed with nobody else there. The translated
//! game has one clock, so this module gives the game the world's speed: while another player is
//! in the session the V.A.T.S. menu does not stop this game and the playback does not slow it
//! (`coop_engine::vats`), on the host and on a guest alike, because each game runs actors the
//! others see. Alone, V.A.T.S. is the game's own again.
//!
//! The queued shots are played by the game as attacks. Each lands through `Actor::HitMe`, so a
//! guest's shot at an actor the host runs is kept from the guest's copy and judged by the host's
//! ledger like any other hit (`crate::fight`). The player in V.A.T.S. is hit in real time, as
//! everyone else is, and gets no time the others do not have.
//!
//! Each machine also tells the others when its player enters and leaves V.A.T.S. (coop-core's
//! `CombatMsg::VatsMark`), which is presentation only.

use coop_core::combat::vats_scales;
use coop_engine::vats;
use engine_core::Engine;

/// How much of V.A.T.S. a run shares (`NV_COOP_VATS`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    /// V.A.T.S. is the game's own on every machine: it stops and slows that machine's game.
    Off,
    /// The playback does not slow the game; the menu still stops it.
    Slow,
    /// Also: the menu does not stop the game.
    Live,
}

impl Mode {
    /// `off` or `slow`; anything else, or nothing, is everything this build shares.
    pub fn parse(v: &str) -> Mode {
        match v.trim() {
            "off" => Mode::Off,
            "slow" => Mode::Slow,
            _ => Mode::Live,
        }
    }
}

/// One machine's V.A.T.S. in a session.
#[derive(Debug)]
pub struct Share {
    mode: Mode,
    /// The world speed the game was last given (0: V.A.T.S. is the game's own).
    world: f32,
    /// Whether the others were last told this player is in V.A.T.S.
    told: bool,
    /// Times this player entered V.A.T.S. with the world shared.
    pub entered: u32,
}

impl Share {
    /// V.A.T.S. shared up to `mode`.
    pub fn new(mode: Mode) -> Share {
        Share {
            mode,
            world: 0.0,
            told: false,
            entered: 0,
        }
    }

    /// The mode this run was given.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// One frame, between two of the game's frames. `others` says whether another player is in
    /// the session now. Gives the game the world's speed when that changed, and answers
    /// `Some(active)` when the others are to be told that this player entered or left V.A.T.S.
    pub fn step(&mut self, e: &mut Engine, others: bool) -> Option<bool> {
        if self.mode == Mode::Off {
            return None;
        }
        // The world's clock, as coop-core's model has it: 1.0 when shared; alone, whatever
        // V.A.T.S. wants (0 while planning), which is the game's own behaviour.
        let world = vats_scales(others, 0.0).world;
        if world != self.world {
            self.world = world;
            vats::set_world(e, world, self.mode >= Mode::Live);
        }
        let active = others && vats::mode(e) != 0;
        if active == self.told {
            return None;
        }
        self.told = active;
        self.entered += u32::from(active);
        Some(active)
    }

    /// Gives V.A.T.S. back to the game (the session ends).
    pub fn close(&mut self, e: &mut Engine) {
        if self.world != 0.0 {
            vats::set_world(e, 0.0, false);
        }
        self.world = 0.0;
        self.told = false;
    }

    /// One status phrase for the log.
    pub fn status(&self, e: &mut Engine) -> String {
        let (unpaused, unslowed) = vats::counts(e);
        format!(
            "V.A.T.S. {:?}: entered {}, menu kept from stopping the game {} times, slow motion lifted {}",
            self.mode, self.entered, unpaused, unslowed
        )
    }
}
