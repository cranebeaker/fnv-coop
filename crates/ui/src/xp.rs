//! The HUD's XP meter and its "LEVEL UP" (`XPMeter` in `hud_main_menu.xml`;
//! `0077c4e0` in FalloutNV.exe runs it each frame, `0076bfe0` places it).
//!
//! When experience comes in and the meter is hidden, the gains waiting are
//! added up and the meter fades in over 2 seconds: its bracket, the gain
//! ("+%i"), "XP", the pointer where the experience stood before the gain,
//! and the levels either side ("%i"). Then the pointer moves to the new
//! experience over 2 seconds (to the right end over 1 second when the
//! experience passes the next level; out of combat the levels then step
//! on and the pointer starts again from the left), and everything fades
//! out over 2 seconds. With a level-up waiting, once the meter has gone,
//! "LEVEL UP" fades in over half a second, stays until 1.5 seconds after it
//! started, and fades out; no meter shows after that until the level-up
//! is taken (`0077da30`).
//!
//! The pointer's place: `_x_min` + (`_x_max` − `_x_min`) × (experience −
//! the level's start) / (the next level's start − the level's start)
//! (`004b3ab0`), the experience clamped to the level (`00647b70`,
//! `004a8f20`), the first place clamped to the pointer's range (`00404010`,
//! `0040ebd0`). A level's start: 0 below level 2, else `iXPBase` + the sum
//! of `iXPBase` + n × `iXPBumpBase` for n = 1 .. level − 2 (`00648b50`).
//!
//! Two things are inferred, not read: "in combat" for the player's value
//! that holds the levels back (`00953c20`), and "a level-up waits" for the
//! flag at player + 0x878 (`008d51f0`). A third player flag (+0x75c,
//! `005c7870`) that also holds the level-up text back is taken as off, and
//! the menu `0077d1c1` opens when the pointer reaches the end with a
//! level-up waiting isn't modelled.

use crate::anim::Animations;
use crate::names::t;
use crate::tile::{TileId, Ui};

/// How long the meter fades in, the pointer moves and the meter fades out
/// (`2.0` at `010162c0`), and the pointer's run to the end of a level
/// (`1.0`).
pub const METER_SECONDS: f32 = 2.0;
pub const TO_THE_END_SECONDS: f32 = 1.0;
/// The "LEVEL UP" text's fades (`0.5` at `01016248`) and how long after it
/// starts it begins to go (`1500.0` ms at `010737c8`).
pub const LEVEL_UP_FADE: f32 = 0.5;
pub const LEVEL_UP_HOLD: f64 = 1.5;

/// The player's experience, as the meter reads it.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Experience {
    /// Experience points (actor value 24).
    pub xp: i32,
    pub level: i32,
    /// `iMaxCharacterLevel`.
    pub max_level: i32,
    /// `iXPBase`, `iXPBumpBase`.
    pub base: i32,
    pub bump: i32,
    /// A level-up waits to be taken.
    pub level_up_ready: bool,
    pub in_combat: bool,
}

/// Where a level starts (`00648b50`).
pub fn level_start(level: i32, base: i32, bump: i32) -> i32 {
    if level < 2 {
        return 0;
    }
    let mut total = base;
    let mut step = base;
    for _ in 1..level - 1 {
        step += bump;
        total += step;
    }
    total
}

/// `a` + (`b` − `a`) × (`v` − `lo`) / (`hi` − `lo`) (`004b3ab0`).
fn map(a: f32, b: f32, lo: f32, hi: f32, v: f32) -> f32 {
    (b - a) * ((v - lo) / (hi - lo)) + a
}

/// The meter's pieces (the HUD object's fields +0xd8 .. +0xf0).
#[derive(Debug, Clone)]
pub struct XpTiles {
    pub meter: TileId,
    pub bracket: TileId,
    pub amount: TileId,
    pub label: TileId,
    pub pointer: TileId,
    pub last_level: TileId,
    pub next_level: TileId,
    pub level_up: TileId,
}

impl XpTiles {
    /// The six that show and hide together.
    fn six(&self) -> [TileId; 6] {
        [
            self.bracket,
            self.amount,
            self.label,
            self.pointer,
            self.last_level,
            self.next_level,
        ]
    }
}

/// The meter and what it remembers between frames.
#[derive(Debug, Clone)]
pub struct XpMeter {
    pub tiles: XpTiles,
    x_min: i32,
    x_max: i32,
    /// `011a010c`: 1 the meter may show, 2 "LEVEL UP" may show, 4 a
    /// level-up waits, 8 "LEVEL UP" has been shown.
    flags: u32,
    /// HUD + 0x204: no meter until the level-up is taken.
    held: bool,
    /// The levels the meter shows (`011d9e7c`, `011d9e78`).
    last: i32,
    next: i32,
    /// Gains waiting (HUD + 0x264).
    gains: Vec<i32>,
    /// The level last seen, to notice a level-up being taken.
    seen_level: Option<i32>,
}

/// The pointer's "moved" mark, and "LEVEL UP"'s start time (ms) and
/// "shown" mark: `user11`, `user9`, `user10` on those tiles.
const MOVED: i32 = t::USER0 + 11;
const STARTED: i32 = t::USER0 + 9;
const SHOWN: i32 = t::USER0 + 10;

impl XpMeter {
    /// Finds the meter's pieces and places it as `0076bfe0` does: x = W −
    /// 2 sx − 390, y = H − 2 sy − 280 (also kept in `user3`). `None` when
    /// the menu has no meter.
    pub fn create(ui: &mut Ui, menu: TileId) -> Option<XpMeter> {
        let meter = ui.find(menu, "XPMeter")?;
        let find = |ui: &Ui, n: &str| ui.find(meter, n);
        let tiles = XpTiles {
            meter,
            bracket: find(ui, "XPBracket")?,
            amount: find(ui, "XPAmount")?,
            label: find(ui, "XPLabel")?,
            pointer: find(ui, "XPPointer")?,
            last_level: find(ui, "XPLastLevel")?,
            next_level: find(ui, "XPNextLevel")?,
            level_up: find(ui, "XPLevelUp")?,
        };
        let x_min = ui.names.lookup_or_add("_x_min").unwrap_or(0);
        let x_max = ui.names.lookup_or_add("_x_max").unwrap_or(0);
        let screen = ui.screen_size;
        let (w, h) = (screen.width() as i32, screen.height() as i32);
        let (sx, sy) = (screen.safe_x as i32, screen.safe_y as i32);
        ui.set_number(meter, t::X, (w - sx * 2 - 390) as f32);
        let y = (h - sy * 2 - 280) as f32;
        ui.set_number(meter, t::Y, y);
        ui.set_number(meter, t::USER0 + 3, y);
        Some(XpMeter {
            tiles,
            x_min,
            x_max,
            flags: 1,
            held: false,
            last: 0,
            next: 0,
            gains: Vec::new(),
            seen_level: None,
        })
    }

    /// Experience gained, to show when the meter is next free.
    pub fn add(&mut self, amount: i32) {
        self.gains.push(amount);
    }

    /// One frame (`0077c4e0`). `menu_open`: a menu other than the dialogue
    /// menu is open; `opacity`: the HUD's alpha (`fHudOpacity` × 255);
    /// `now` in seconds; `xp_label` and `level_up` the settings' texts.
    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        ui: &mut Ui,
        anims: &mut Animations,
        e: &Experience,
        menu_open: bool,
        opacity: f32,
        now: f64,
        xp_label: &str,
        level_up: &str,
    ) {
        let tl = self.tiles.clone();
        // A level-up taken (the level went up): the meter may show again
        // (`0077da30`, which the game calls after the level-up menu; when
        // exactly is inferred).
        if self.seen_level.is_some_and(|l| e.level > l) {
            self.held = false;
            self.flags = 1;
        }
        self.seen_level = Some(e.level);
        if menu_open {
            for tile in tl.six() {
                ui.set_number(tile, t::VISIBLE, 0.0);
            }
            ui.set_number(tl.bracket, t::ALPHA, 0.0);
            anims.stop(tl.bracket, t::ALPHA);
            self.flags |= 1;
            self.held = false;
            return;
        }
        if opacity == 0.0 {
            self.held = true;
            return;
        }
        let mut shown = ui.number(tl.bracket, t::ALPHA) > 0.0;
        let idle = anims.done(tl.bracket, t::ALPHA, now);
        let (x_min, x_max) = (
            ui.number(tl.pointer, self.x_min),
            ui.number(tl.pointer, self.x_max),
        );
        let start = |level: i32| level_start(level, e.base, e.bump);
        if self.flags & 1 != 0 {
            if !shown && idle && !self.gains.is_empty() {
                let sum: i32 = self.gains.drain(..).sum();
                if sum > 0 && !self.held {
                    self.last = e.level;
                    self.next = e.level + 1;
                    let (lo, hi) = (start(self.last), start(self.next));
                    let x = if lo == hi {
                        lo as f32
                    } else {
                        map(x_min, x_max, lo as f32, hi as f32, (e.xp - sum) as f32)
                    };
                    ui.set_number(tl.pointer, t::X, x.max(x_min).min(x_max));
                    for tile in tl.six() {
                        ui.set_number(tile, t::VISIBLE, 1.0);
                        ui.set_number(tile, t::ALPHA, 0.0);
                    }
                    ui.set_string(tl.label, t::STRING, xp_label);
                    ui.set_string(tl.amount, t::STRING, &format!("+{sum}"));
                    ui.set_string(tl.last_level, t::STRING, &self.last.to_string());
                    ui.set_string(tl.next_level, t::STRING, &self.next.to_string());
                    for tile in tl.six() {
                        anims.start(tile, t::ALPHA, 0.0, opacity, METER_SECONDS, now);
                    }
                    shown = true;
                }
            } else {
                let pointer_done = anims.done(tl.pointer, t::X, now);
                let moved = ui.number(tl.pointer, MOVED) != 0.0;
                if !moved && idle && shown {
                    // Faded in: the pointer moves to the experience now.
                    if self.last == self.next {
                        self.next += 1;
                    }
                    let (lo, hi) = (start(self.last), start(self.next));
                    ui.set_number(tl.pointer, MOVED, 1.0);
                    if e.xp > hi {
                        if ui.number(tl.pointer, t::X) > x_max {
                            ui.set_number(tl.pointer, t::X, x_max);
                        }
                        let x = ui.number(tl.pointer, t::X);
                        anims.start(tl.pointer, t::X, x, x_max, TO_THE_END_SECONDS, now);
                    } else {
                        let v = e.xp.max(lo).min(hi) as f32;
                        let to = map(x_min, x_max, lo as f32, hi as f32, v);
                        let x = ui.number(tl.pointer, t::X);
                        anims.start(tl.pointer, t::X, x, to, METER_SECONDS, now);
                    }
                } else if idle && !shown {
                    // Faded out.
                    for tile in tl.six() {
                        ui.set_number(tile, t::VISIBLE, 0.0);
                    }
                    if self.flags & 8 != 0 && !self.held {
                        self.held = true;
                        ui.set_number(tl.level_up, SHOWN, 0.0);
                    }
                } else if !pointer_done && idle && shown {
                    // The pointer moving: past the next level (out of
                    // combat, below the top level), the levels step on.
                    let hi = start(self.next);
                    if e.xp > hi && self.next < e.max_level && !e.in_combat {
                        self.last += 1;
                        self.next += 1;
                        let (lo, hi) = (hi, start(self.next));
                        ui.set_number(tl.pointer, t::X, x_min);
                        ui.set_string(tl.last_level, t::STRING, &self.last.to_string());
                        ui.set_string(tl.next_level, t::STRING, &self.next.to_string());
                        if e.xp > hi {
                            let from = x_min.min(x_max);
                            anims.start(tl.pointer, t::X, from, x_max, TO_THE_END_SECONDS, now);
                        } else {
                            let to = map(x_min, x_max, lo as f32, hi as f32, e.xp as f32);
                            anims.start(tl.pointer, t::X, x_min, to, METER_SECONDS, now);
                        }
                    }
                } else if pointer_done && moved && idle && ui.number(tl.bracket, t::ALPHA) > 0.0 {
                    // The pointer has arrived: fade out.
                    ui.set_number(tl.pointer, MOVED, 0.0);
                    for tile in tl.six() {
                        let a = ui.number(tile, t::ALPHA);
                        anims.start(tile, t::ALPHA, a, 0.0, METER_SECONDS, now);
                    }
                }
            }
            if e.level_up_ready {
                if !e.in_combat && !shown && self.flags & 8 == 0 {
                    self.flags = 2;
                } else {
                    self.flags |= 4;
                }
            }
            return;
        }
        // "LEVEL UP".
        let lu = tl.level_up;
        let lu_idle = anims.done(lu, t::ALPHA, now);
        let lu_zero = ui.number(lu, t::ALPHA) == 0.0;
        let now_ms = now * 1000.0;
        if self.flags & 2 != 0 && self.flags & 8 == 0 && !shown && idle {
            if lu_idle && lu_zero && ui.number(lu, SHOWN) == 0.0 && !e.in_combat {
                ui.set_number(lu, t::ALPHA, 0.0);
                anims.start(lu, t::ALPHA, 0.0, opacity, LEVEL_UP_FADE, now);
                ui.set_string(lu, t::STRING, level_up);
                ui.set_number(lu, t::VISIBLE, 1.0);
                ui.set_number(lu, STARTED, now_ms as f32);
                ui.set_number(lu, SHOWN, 1.0);
            } else if now_ms - f64::from(ui.number(lu, STARTED)) > LEVEL_UP_HOLD * 1000.0
                && !lu_zero
                && lu_idle
            {
                ui.set_number(lu, STARTED, (now_ms + 2000.0) as f32);
                anims.start(lu, t::ALPHA, opacity, 0.0, LEVEL_UP_FADE, now);
            } else if !self.held && lu_idle && lu_zero {
                self.held = true;
                self.flags = 8;
                ui.set_number(lu, SHOWN, 0.0);
            }
        } else if self.flags & 4 != 0 {
            if self.flags & 8 == 0 {
                if e.in_combat {
                    self.flags |= 1;
                } else if !shown {
                    self.flags = 2;
                }
            } else if !self.held && lu_idle && lu_zero {
                self.held = true;
                self.flags = 8;
                ui.set_number(lu, SHOWN, 0.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Screen, SystemColors};

    #[test]
    fn levels_start_where_the_game_says() {
        // `iXPBase` 200, `iXPBumpBase` 150: 0, 0, 200, 550, 1050.
        let starts: Vec<i32> = (0..5).map(|l| level_start(l, 200, 150)).collect();
        assert_eq!(starts, [0, 0, 200, 550, 1050]);
    }

    const MENU: &str = r#"<menu name="HUDMainMenu"><rect name="XPMeter"><locus>&true;</locus>
        <image name="XPBracket"><visible>&false;</visible><alpha>0</alpha></image>
        <image name="XPPointer"><_x_min>55</_x_min><_x_max>270</_x_max><x>65</x><visible>&false;</visible></image>
        <text name="XPAmount"><visible>&false;</visible></text>
        <text name="XPLabel"><visible>&false;</visible></text>
        <text name="XPLastLevel"><visible>&false;</visible></text>
        <text name="XPNextLevel"><visible>&false;</visible></text>
        <text name="XPLevelUp"><visible>&false;</visible></text>
        </rect></menu>"#;

    fn meter() -> (Ui, XpMeter) {
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
        let m = ui.load_menu(MENU.as_bytes(), &mut |_| None).unwrap();
        let x = XpMeter::create(&mut ui, m).unwrap();
        (ui, x)
    }

    #[test]
    fn a_gain_fades_in_moves_the_pointer_and_fades_out() {
        let (mut ui, mut x) = meter();
        let mut anims = Animations::default();
        let tl = x.tiles.clone();
        // Placed 390 left of the right edge less the safe zone, 280 up.
        assert_eq!(
            ui.screen_position(tl.meter),
            (1706.0 - 30.0 - 390.0, 960.0 - 30.0 - 280.0)
        );
        // Level 2 runs from 200 to 550; 10 more makes 300.
        let e = Experience {
            xp: 300,
            level: 2,
            max_level: 30,
            base: 200,
            bump: 150,
            ..Experience::default()
        };
        x.add(10);
        let frame = |ui: &mut Ui, x: &mut XpMeter, anims: &mut Animations, now: f64| {
            anims.step(ui, now);
            x.update(ui, anims, &e, false, 255.0, now, "XP", "LEVEL UP");
        };
        frame(&mut ui, &mut x, &mut anims, 0.0);
        assert_eq!(ui.string(tl.amount, t::STRING).as_deref(), Some("+10"));
        assert_eq!(ui.string(tl.last_level, t::STRING).as_deref(), Some("2"));
        assert_eq!(ui.string(tl.next_level, t::STRING).as_deref(), Some("3"));
        // The pointer where 290 stood: 55 + 215 × 90 / 350.
        let before = 55.0 + 215.0 * 90.0 / 350.0;
        assert!((ui.number(tl.pointer, t::X) - before).abs() < 1e-3);
        assert_eq!(ui.number(tl.bracket, t::VISIBLE), 1.0);
        // Half way through the fade in.
        frame(&mut ui, &mut x, &mut anims, 1.0);
        assert!((ui.number(tl.bracket, t::ALPHA) - 127.5).abs() < 1e-3);
        // Faded in: the pointer sets off for 300.
        frame(&mut ui, &mut x, &mut anims, 2.0);
        frame(&mut ui, &mut x, &mut anims, 4.0);
        let after = 55.0 + 215.0 * 100.0 / 350.0;
        assert!((ui.number(tl.pointer, t::X) - after).abs() < 1e-3);
        // Arrived: fading out, then hidden.
        frame(&mut ui, &mut x, &mut anims, 4.1);
        frame(&mut ui, &mut x, &mut anims, 6.2);
        frame(&mut ui, &mut x, &mut anims, 6.3);
        assert_eq!(ui.number(tl.bracket, t::ALPHA), 0.0);
        assert_eq!(ui.number(tl.bracket, t::VISIBLE), 0.0);
    }

    #[test]
    fn a_level_up_shows_once_the_meter_has_gone() {
        let (mut ui, mut x) = meter();
        let mut anims = Animations::default();
        let lu = x.tiles.level_up;
        let e = Experience {
            xp: 560,
            level: 2,
            max_level: 30,
            base: 200,
            bump: 150,
            level_up_ready: true,
            in_combat: true,
        };
        let mut t_now = 0.0;
        x.add(100);
        let mut started = None;
        while t_now < 20.0 {
            anims.step(&mut ui, t_now);
            x.update(
                &mut ui, &mut anims, &e, false, 255.0, t_now, "XP", "LEVEL UP",
            );
            if started.is_none() && ui.number(lu, t::VISIBLE) == 1.0 {
                started = Some(t_now);
            }
            t_now += 0.05;
        }
        // In combat the levels don't step on, and "LEVEL UP" waits.
        assert_eq!(started, None);
        let calm = Experience {
            in_combat: false,
            ..e
        };
        let mut t_now = 20.0;
        while t_now < 30.0 {
            anims.step(&mut ui, t_now);
            x.update(
                &mut ui, &mut anims, &calm, false, 255.0, t_now, "XP", "LEVEL UP",
            );
            if started.is_none() && ui.number(lu, t::VISIBLE) == 1.0 {
                started = Some(t_now);
            }
            t_now += 0.05;
        }
        assert!(started.is_some());
        assert_eq!(ui.string(lu, t::STRING).as_deref(), Some("LEVEL UP"));
        // Faded in and out again by now; no meter until the level-up is
        // taken.
        assert_eq!(ui.number(lu, t::ALPHA), 0.0);
        x.add(5);
        anims.step(&mut ui, 31.0);
        x.update(
            &mut ui, &mut anims, &calm, false, 255.0, 31.0, "XP", "LEVEL UP",
        );
        assert_eq!(ui.number(x.tiles.bracket, t::VISIBLE), 0.0);
    }
}
