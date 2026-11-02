//! Traits moving from one value to another over time, as the game's menus
//! do it: `00a07c60` starts one (tile, trait, from, to, seconds, mode),
//! `00a080d0` moves every one each frame, `00a07fc0` says whether one has
//! had its time, `00a07dc0` stops them. The straight-line kind (mode 0),
//! which the HUD uses: value = from + (to − from) × min(elapsed / length,
//! 1), the animation dropped once it has set its end value; and the pulse
//! (mode 1: V.A.T.S.'s part labels and meters), and the counted flash
//! (mode 2: the sneak meter's [DANGER]). Modes 3 and 4 (flashes up to
//! `_TotalFlashCount`, a three-stage fade) aren't here.

use crate::tile::{TileId, Ui};

#[derive(Debug, Clone, PartialEq)]
struct Animation {
    tile: TileId,
    trait_id: i32,
    from: f32,
    to: f32,
    /// When it started and how long it lasts, in seconds.
    start: f64,
    length: f64,
    /// 0 a straight line once, 1 a pulse for ever ([`Animations::pulse`]),
    /// 2 three pulses then up to `to` ([`Animations::flash`]).
    mode: u8,
    /// Mode 2: the rounds finished (`_FlashCount`).
    flashes: u32,
}

/// How many pulses a mode 2 flash makes before it settles (the double 3.0
/// at `01021928`, `00a080d0`).
pub const FLASHES: u32 = 3;

/// The animations under way.
#[derive(Debug, Clone, Default)]
pub struct Animations {
    list: Vec<Animation>,
}

impl Animations {
    /// Starts moving `tile`'s trait from `from` to `to` over `seconds`,
    /// replacing one already moving that trait. As in the game, nothing
    /// happens when `from` equals `to` or `seconds` isn't above 0, and the
    /// trait keeps its value until the next [`Self::step`].
    pub fn start(
        &mut self,
        tile: TileId,
        trait_id: i32,
        from: f32,
        to: f32,
        seconds: f32,
        now: f64,
    ) {
        if from == to || seconds <= 0.0 {
            return;
        }
        self.stop(tile, trait_id);
        self.list.push(Animation {
            tile,
            trait_id,
            from,
            to,
            start: now,
            length: f64::from(seconds),
            mode: 0,
            flashes: 0,
        });
    }

    /// Starts a counted flash (`00a07c60` mode 2, which zeroes
    /// `_FlashCount`; `00a080d0`): pulses as [`Self::pulse`] does, a new
    /// round each time one ends while fewer than [`FLASHES`] have ended,
    /// then goes up from `from` to `to` and stops there.
    pub fn flash(
        &mut self,
        tile: TileId,
        trait_id: i32,
        from: f32,
        to: f32,
        seconds: f32,
        now: f64,
    ) {
        if from == to || seconds <= 0.0 {
            return;
        }
        self.stop(tile, trait_id);
        self.list.push(Animation {
            tile,
            trait_id,
            from,
            to,
            start: now,
            length: f64::from(seconds),
            mode: 2,
            flashes: 0,
        });
    }

    /// Starts a pulse (`00a07c60` mode 1, `00a080d0`): from `from` to `to`
    /// and back every `seconds`, `from + (to − from)(1 − |2t − 1|)` with t
    /// the share of the current round, starting a new round each time one
    /// ends; it goes on until stopped or replaced.
    pub fn pulse(
        &mut self,
        tile: TileId,
        trait_id: i32,
        from: f32,
        to: f32,
        seconds: f32,
        now: f64,
    ) {
        if from == to || seconds <= 0.0 {
            return;
        }
        self.stop(tile, trait_id);
        self.list.push(Animation {
            tile,
            trait_id,
            from,
            to,
            start: now,
            length: f64::from(seconds),
            mode: 1,
            flashes: 0,
        });
    }

    /// What a moving trait is heading for, else its value (`00a07f30`).
    pub fn target(&self, ui: &mut Ui, tile: TileId, trait_id: i32) -> f32 {
        match self
            .list
            .iter()
            .find(|a| a.tile == tile && a.trait_id == trait_id)
        {
            Some(a) => a.to,
            None => ui.number(tile, trait_id),
        }
    }

    /// The mode of what moves a trait, 0 when nothing does (`00a08070`).
    pub fn mode(&self, tile: TileId, trait_id: i32) -> u8 {
        self.list
            .iter()
            .find(|a| a.tile == tile && a.trait_id == trait_id)
            .map_or(0, |a| a.mode)
    }

    /// Stops whatever moves that trait (it keeps its current value).
    pub fn stop(&mut self, tile: TileId, trait_id: i32) {
        self.list
            .retain(|a| !(a.tile == tile && a.trait_id == trait_id));
    }

    /// Whether something moves that trait now.
    pub fn moving(&self, tile: TileId, trait_id: i32) -> bool {
        self.list
            .iter()
            .any(|a| a.tile == tile && a.trait_id == trait_id)
    }

    /// Whether nothing moves that trait, or what moves it has had its time
    /// (`00a07fc0`).
    pub fn done(&self, tile: TileId, trait_id: i32, now: f64) -> bool {
        match self
            .list
            .iter()
            .find(|a| a.tile == tile && a.trait_id == trait_id)
        {
            Some(a) => (now - a.start) / a.length >= 1.0,
            None => true,
        }
    }

    /// One frame: every trait set to where its animation has got to;
    /// finished ones dropped.
    pub fn step(&mut self, ui: &mut Ui, now: f64) {
        let mut finished = Vec::new();
        for (i, a) in self.list.iter_mut().enumerate() {
            if a.mode == 1 {
                let mut t = (now - a.start) / a.length;
                if t >= 1.0 {
                    a.start = now;
                    t = 0.0;
                }
                let k = 1.0 - (t as f32 * 2.0 - 1.0).abs();
                ui.set_number(a.tile, a.trait_id, (a.to - a.from) * k + a.from);
                continue;
            }
            if a.mode == 2 {
                // Translated from 00a080d0 (decompiled, FalloutNV.exe 1.4.0.525)
                let count = a.flashes;
                let mut t = (now - a.start) / a.length;
                if t >= 1.0 {
                    if count < FLASHES {
                        a.start = now;
                        t = 0.0;
                    }
                    a.flashes += 1;
                }
                if count < FLASHES {
                    let k = 1.0 - (t as f32 * 2.0 - 1.0).abs();
                    ui.set_number(a.tile, a.trait_id, (a.to - a.from) * k + a.from);
                } else {
                    let v = (a.to - a.from) * t as f32 + a.from;
                    ui.set_number(a.tile, a.trait_id, v.min(a.to));
                    if v >= a.to {
                        finished.push(i);
                    }
                }
                continue;
            }
            let k = ((now - a.start) / a.length).min(1.0) as f32;
            ui.set_number(a.tile, a.trait_id, (a.to - a.from) * k + a.from);
            if k >= 1.0 {
                finished.push(i);
            }
        }
        for i in finished.into_iter().rev() {
            self.list.remove(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::names::t;
    use crate::tile::{Screen, SystemColors};

    #[test]
    fn traits_move_in_a_straight_line_and_stop_at_the_end() {
        let mut ui = Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|_| None),
        );
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><image name=\"a\"><alpha>0</alpha></image></menu>",
                &mut |_| None,
            )
            .unwrap();
        let a = ui.find(m, "a").unwrap();
        let mut anims = Animations::default();
        anims.start(a, t::ALPHA, 0.0, 200.0, 2.0, 10.0);
        // Nothing changes until a frame moves it.
        assert_eq!(ui.number(a, t::ALPHA), 0.0);
        assert!(!anims.done(a, t::ALPHA, 10.5));
        anims.step(&mut ui, 10.5);
        assert_eq!(ui.number(a, t::ALPHA), 50.0);
        assert!(anims.done(a, t::ALPHA, 12.0));
        anims.step(&mut ui, 13.0);
        assert_eq!(ui.number(a, t::ALPHA), 200.0);
        assert!(!anims.moving(a, t::ALPHA));
        // From and to the same: nothing starts.
        anims.start(a, t::ALPHA, 5.0, 5.0, 1.0, 20.0);
        assert!(!anims.moving(a, t::ALPHA));
    }

    #[test]
    fn pulses_go_up_and_back_every_round_for_ever() {
        let mut ui = Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|_| None),
        );
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><image name=\"a\"><alpha>0</alpha></image></menu>",
                &mut |_| None,
            )
            .unwrap();
        let a = ui.find(m, "a").unwrap();
        let mut anims = Animations::default();
        anims.pulse(a, t::ALPHA, 0.0, 255.0, 1.0, 0.0);
        assert_eq!(anims.mode(a, t::ALPHA), 1);
        assert_eq!(anims.target(&mut ui, a, t::ALPHA), 255.0);
        anims.step(&mut ui, 0.5);
        assert_eq!(ui.number(a, t::ALPHA), 255.0);
        anims.step(&mut ui, 0.75);
        assert_eq!(ui.number(a, t::ALPHA), 127.5);
        // A round ends: the next starts from the bottom.
        anims.step(&mut ui, 1.0);
        assert_eq!(ui.number(a, t::ALPHA), 0.0);
        anims.step(&mut ui, 1.25);
        assert_eq!(ui.number(a, t::ALPHA), 127.5);
        assert!(anims.moving(a, t::ALPHA));
        anims.stop(a, t::ALPHA);
        assert_eq!(anims.mode(a, t::ALPHA), 0);
    }

    #[test]
    fn a_flash_pulses_three_times_then_settles_at_the_top() {
        let mut ui = Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|_| None),
        );
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><image name=\"a\"><alpha>0</alpha></image></menu>",
                &mut |_| None,
            )
            .unwrap();
        let a = ui.find(m, "a").unwrap();
        let mut anims = Animations::default();
        anims.flash(a, t::ALPHA, 0.0, 255.0, 0.5, 0.0);
        assert_eq!(anims.mode(a, t::ALPHA), 2);
        anims.step(&mut ui, 0.25);
        assert_eq!(ui.number(a, t::ALPHA), 255.0);
        // Rounds one and two end: each starts again from the bottom.
        for end in [0.5, 1.0] {
            anims.step(&mut ui, end);
            assert_eq!(ui.number(a, t::ALPHA), 0.0, "{end}");
            anims.step(&mut ui, end + 0.25);
            assert_eq!(ui.number(a, t::ALPHA), 255.0, "{end}");
        }
        // The third has ended: one more round goes straight up to the top
        // and stops there.
        anims.step(&mut ui, 1.5);
        assert_eq!(ui.number(a, t::ALPHA), 0.0);
        anims.step(&mut ui, 1.75);
        assert_eq!(ui.number(a, t::ALPHA), 127.5);
        anims.step(&mut ui, 2.0);
        assert_eq!(ui.number(a, t::ALPHA), 255.0);
        assert!(!anims.moving(a, t::ALPHA));
    }
}
