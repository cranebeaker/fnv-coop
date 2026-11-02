//! The menus' typed lines (FalloutNV.exe's line class: made at `0076b300`,
//! a step `006ffea0`, a new text `006ffda0`, all at once `00700110`),
//! shared by the hacking and terminal menus: a text that appears a few
//! characters at a time into a tile's `string`, turning something on as
//! it starts, and optionally flashing.

use crate::names::t;
use crate::tile::{TileId, Ui};

/// A typed line.
#[derive(Debug, Clone)]
pub(crate) struct Line {
    /// The tile it types into, and the trait (its `string`, or a list
    /// item's `user0`).
    pub tile: Option<TileId>,
    pub string: i32,
    /// What turns on when it starts: a tile's trait.
    pub shows: Option<(TileId, i32)>,
    /// An input line's ">".
    pub prompt: Option<TileId>,
    pub text: String,
    /// Characters typed; `None` before a text (the game's empty buffer).
    pub typed: Option<usize>,
    pub ms: f64,
    /// When the next characters come (0: at once).
    pub next: f64,
    pub done: bool,
    /// Flashing: on and off ms, and the next change.
    pub flash: Option<(f64, f64, f64)>,
}

impl Line {
    pub fn new(tile: Option<TileId>, text: &str, ms: f64) -> Line {
        Line {
            tile,
            string: t::STRING,
            shows: tile.map(|t| (t, t::VISIBLE)),
            prompt: None,
            text: text.to_string(),
            typed: (!text.is_empty()).then_some(0),
            ms,
            next: 0.0,
            done: false,
            flash: None,
        }
    }

    /// A new text, typed from the start (`006ffda0`).
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.typed = (!text.is_empty()).then_some(0);
        self.done = false;
        self.next = 0.0;
    }

    pub fn show(&self, ui: &mut Ui) {
        if let Some((tile, tr)) = self.shows {
            if self.flash.is_none() {
                ui.set_number(tile, tr, 1.0);
            }
            if self.tile.is_some() && self.text.is_empty() {
                ui.set_number(tile, tr, 0.0);
            }
        }
    }

    /// One step (`006ffea0`); the characters typed.
    pub fn step(&mut self, ui: &mut Ui, now: f64) -> usize {
        let mut added = 0;
        if now >= self.next && !self.done {
            self.show(ui);
            match (self.tile, self.typed) {
                (Some(tile), Some(typed)) if typed < self.text.len() => {
                    let mut n = if self.next == 0.0 {
                        1
                    } else {
                        ((now - self.next) / self.ms) as usize + 1
                    };
                    let left = self.text.len() - typed;
                    if left < n {
                        n = left;
                        self.done = true;
                    }
                    self.typed = Some(typed + n);
                    added = n;
                    ui.set_string(tile, self.string, &self.text[..typed + n]);
                }
                _ => self.done = true,
            }
            self.next = now + self.ms;
        }
        if let (Some((on, off, at)), Some((tile, tr))) = (self.flash, self.shows) {
            if now >= at {
                let lit = ui.number(tile, tr) == 1.0;
                ui.set_number(tile, tr, if lit { 0.0 } else { 1.0 });
                self.flash = Some((on, off, now + if lit { off } else { on }));
            }
        }
        added
    }

    /// All at once (`00700110`).
    pub fn finish(&mut self, ui: &mut Ui) {
        if self.done {
            return;
        }
        self.show(ui);
        if let (Some(tile), Some(_)) = (self.tile, self.typed) {
            self.typed = Some(self.text.len());
            ui.set_string(tile, self.string, &self.text);
        }
        self.done = true;
    }
}
