//! What follows the player's death, as `FalloutNV.exe` 1.4.0.525's player
//! control handler (`0093e860`, at `0093fedf`…`00940037`) does it.
//!
//! With `fPlayerDeathReloadTime` above 0 (5 s; the data doesn't change
//! it): while the player is dying or dead (life state, `004f8960`, 1 or
//! 2), the first frame hides the scope overlay (`00709c40(0)`) and starts
//! the timer (`011a3b34`) at the setting; every frame takes the frame's
//! seconds off; once it goes below zero the timer stops (−1) and the most
//! recent save loads (`008512f0`), or, when there is none, the game goes
//! back to the main menu (`007d0a70`, with `MainTitle.mp3`). Alive, the
//! timer is −1.
//!
//! With the setting at 0 or below: once dead (life state 2) and no box is
//! up yet (`011e07c0`), the scope overlay goes and, when a save exists
//! (`00851230`), a message box asks `sMiscPlayerDeadMessage` ("Reloading
//! the most recent save game") with `sMiscPlayerDeadLoadOption` ("Reload")
//! and `sMiscPlayerDeadMenuOption` ("Main Menu"); its answer
//! (`00961d50`) loads the save or goes to the main menu.
//!
//! The death itself (`0089d900` for the player): `MUSDeath` plays,
//! V.A.T.S. ends (`009c8950(0, 0)`).

/// The timer's idle value (the float at `01012054`).
pub const IDLE: f32 = -1.0;

/// What the death asks for this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathStep {
    /// Nothing (alive, or still waiting).
    Wait,
    /// The timer started: the scope overlay goes (`00709c40(0)`).
    Started,
    /// The timer ran out: load the most recent save (or, with none, the
    /// main menu).
    Reload,
    /// With no timer: ask with the message box.
    Ask,
}

/// The player's death timer (`011a3b34`) and whether the box was asked
/// (`011e07c0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeathReload {
    pub timer: f32,
    pub asked: bool,
}

impl Default for DeathReload {
    fn default() -> Self {
        DeathReload {
            timer: IDLE,
            asked: false,
        }
    }
}

impl DeathReload {
    /// One frame: `dying_or_dead` (life state 1 or 2), `dead` (2), the
    /// frame's seconds, `fPlayerDeathReloadTime`, and whether a save exists
    /// (only the box asks it).
    // Translated from 0093e860 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn update(
        &mut self,
        dying_or_dead: bool,
        dead: bool,
        dt: f32,
        reload_time: f32,
        save_exists: bool,
    ) -> DeathStep {
        if reload_time > 0.0 {
            if !dying_or_dead {
                self.timer = IDLE;
                return DeathStep::Wait;
            }
            let mut step = DeathStep::Wait;
            if self.timer < 0.0 {
                self.timer = reload_time;
                step = DeathStep::Started;
            }
            self.timer -= dt;
            if self.timer < 0.0 {
                self.timer = IDLE;
                return DeathStep::Reload;
            }
            return step;
        }
        if dead && !self.asked {
            if save_exists {
                self.asked = true;
                return DeathStep::Ask;
            }
            return DeathStep::Started;
        }
        DeathStep::Wait
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_most_recent_save_loads_five_seconds_after_death() {
        let mut d = DeathReload::default();
        assert_eq!(d.update(false, false, 0.1, 5.0, true), DeathStep::Wait);
        assert_eq!(d.timer, IDLE);
        // Dying: the timer starts (and the scope goes) on the first frame.
        assert_eq!(d.update(true, false, 0.5, 5.0, true), DeathStep::Started);
        assert!((d.timer - 4.5).abs() < 1e-6);
        let mut frames = 0;
        let step = loop {
            frames += 1;
            let s = d.update(true, true, 0.5, 5.0, true);
            if s != DeathStep::Wait {
                break s;
            }
        };
        // 4.5 s left at 0.5 s a frame: below zero on the tenth frame.
        assert_eq!((step, frames), (DeathStep::Reload, 10));
        assert_eq!(d.timer, IDLE);
        // No timer: the box, once, only with a save.
        let mut d = DeathReload::default();
        assert_eq!(d.update(true, false, 0.1, 0.0, true), DeathStep::Wait);
        assert_eq!(d.update(true, true, 0.1, 0.0, true), DeathStep::Ask);
        assert_eq!(d.update(true, true, 0.1, 0.0, true), DeathStep::Wait);
    }
}
