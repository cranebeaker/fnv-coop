//! List boxes: a tile with `list_box.xml` included (its rows' parent, a
//! highlight box and a scrollbar), filled by code with rows made from a
//! template, as FalloutNV.exe's list box class does it (one copy of it per
//! kind of row; the stats menu's at `007d7da0` add, `007269d0` place,
//! `007d9d50` scroll sizes, `007d97a0` highlight, `007d81f0` empty).
//!
//! The rows place themselves from traits the code sets: `_y` (where the
//! row starts with the list scrolled to the top), `listindex` (their number;
//! negative hides a row), `height`; the list's `_highlight_y` and
//! `_selected_height` follow the chosen row, `_scroll_delta` is how far one
//! step of the scrollbar moves the rows, and the scrollbar's
//! `_number_of_items` how many steps it has.

use crate::names::t;
use crate::tile::{Operand, TileId, Ui};

/// One list box and its rows.
#[derive(Debug, Clone)]
pub struct ListBox {
    /// The list box tile (the rows' parent).
    pub tile: TileId,
    /// The menu whose template makes the rows.
    menu: TileId,
    template: String,
    /// The rows, in order.
    pub rows: Vec<TileId>,
    /// The chosen row (its index in `rows`).
    pub selected: Option<usize>,
    /// The rows' heights added up (the list object's `+0x20`).
    total: f32,
}

/// A custom trait's number (`_y` and its kind), made when not yet known.
fn name(ui: &mut Ui, n: &str) -> i32 {
    ui.names.lookup_or_add(n).unwrap_or(0)
}

impl ListBox {
    /// A list box tile and the template its rows come from (`0075a0d0`
    /// stores the template's name).
    pub fn new(menu: TileId, tile: TileId, template: &str) -> ListBox {
        ListBox {
            tile,
            menu,
            template: template.to_string(),
            rows: Vec::new(),
            selected: None,
            total: 0.0,
        }
    }

    /// Its scrollbar (`lb_scrollbar`).
    fn scrollbar(&self, ui: &Ui) -> Option<TileId> {
        ui.find_below(self.tile, "lb_scrollbar")
    }

    /// Takes every row away (`007d81f0`): the rows are deleted, the count
    /// and heights start again, `_num_filtered` goes to 0, the scroll sizes
    /// are worked out again and nothing is chosen.
    pub fn clear(&mut self, ui: &mut Ui) {
        for row in self.rows.drain(..) {
            ui.remove(row);
        }
        self.total = 0.0;
        let filtered = name(ui, "_num_filtered");
        ui.set_number(self.tile, filtered, 0.0);
        self.update_scroll(ui);
        self.selected = None;
        self.update_highlight(ui);
    }

    /// Adds a row (`007d7da0`): the template made under the list, its `id`
    /// -1 when the template gives none, its `string` the text when there is
    /// one, placed below the last row (`007269d0`), the scroll sizes worked
    /// out again, and its `listindex` the count so far. With the first row
    /// and a list that sets `_number_of_visible_items` above 0, the list's
    /// height becomes that row's height × that number (operators: `copy`
    /// the row's height, `mul` the number).
    pub fn add(&mut self, ui: &mut Ui, text: Option<&str>) -> Option<TileId> {
        let row = ui.instantiate(self.menu, self.tile, &self.template)?;
        if !ui.has(row, t::ID) {
            ui.set_number(row, t::ID, -1.0);
        }
        if let Some(text) = text {
            ui.set_string(row, t::STRING, text);
        }
        self.place(ui, row);
        self.update_scroll(ui);
        let index = self.rows.len();
        ui.set_number(row, t::LISTINDEX, index as f32);
        self.rows.push(row);
        if self.rows.len() == 1 {
            let visible = name(ui, "_number_of_visible_items");
            let n = ui.number(self.tile, visible);
            if n > 0.0 {
                ui.set_number(self.tile, t::HEIGHT, 0.0);
                ui.add_action(
                    self.tile,
                    t::HEIGHT,
                    crate::names::op::COPY,
                    Operand::Link {
                        tile: row,
                        trait_id: t::HEIGHT,
                    },
                );
                ui.add_action(
                    self.tile,
                    t::HEIGHT,
                    crate::names::op::MUL,
                    Operand::Constant(n),
                );
            }
        }
        Some(row)
    }

    /// Places a row (`007269d0`): its `_y` is the heights so far; its
    /// height is its own `height`, or, when that's 0, its first text's
    /// height measured with that text's font and wrap width plus the row's
    /// `_VerticalSpacing` (the game measures with fractional advances,
    /// `00a1b020`; here the text's laid-out height: lines can break one
    /// word differently in rare cases).
    fn place(&mut self, ui: &mut Ui, row: TileId) {
        let y = name(ui, "_y");
        ui.set_number(row, y, self.total);
        let mut h = ui.number(row, t::HEIGHT);
        if h == 0.0 {
            let text = ui.tiles[row]
                .children
                .iter()
                .copied()
                .find(|&c| ui.tiles[c].kind == crate::names::kind::TEXT);
            if let Some(text) = text {
                let spacing = name(ui, "_VerticalSpacing");
                h = ui.number(text, t::HEIGHT) + ui.number(row, spacing);
            }
        }
        ui.set_number(row, t::HEIGHT, h);
        self.total += h;
    }

    /// The scroll sizes (`007d9d50`): with the rows taller than the list
    /// (height H, rows T, n rows not filtered out), the scrollbar gets
    /// max(ceil((T - H) × n / T) + 1, 1) steps and `_scroll_delta` (T - H)
    /// / (steps - 1); otherwise one step.
    fn update_scroll(&mut self, ui: &mut Ui) {
        let h = ui.number(self.tile, t::HEIGHT);
        let total = self.total;
        let mut steps = 1i32;
        if total > 0.0 && h > 0.0 {
            if h < total {
                let filtered = name(ui, "_num_filtered");
                let n = self.rows.len() as f32 - ui.number(self.tile, filtered).round();
                steps = ((((total - h) * n) / total).ceil() as i32 + 1).max(1);
            }
            if steps > 1 {
                let delta = name(ui, "_scroll_delta");
                ui.set_number(self.tile, delta, (total - h) / (steps - 1) as f32);
            }
        }
        if let Some(bar) = self.scrollbar(ui) {
            let items = name(ui, "_number_of_items");
            ui.set_number(bar, items, steps as f32);
        }
    }

    /// The highlight follows the chosen row (`007d97a0`): the list's
    /// `_highlight_y` copies the row's `y` and `_selected_height` its
    /// height; with none chosen they're -1 and 0.
    fn update_highlight(&mut self, ui: &mut Ui) {
        let hy = name(ui, "_highlight_y");
        let sh = name(ui, "_selected_height");
        match self.selected.and_then(|i| self.rows.get(i)).copied() {
            None => {
                ui.set_number(self.tile, hy, -1.0);
                ui.set_number(self.tile, sh, 0.0);
            }
            Some(row) => {
                ui.set_number(self.tile, hy, 0.0);
                ui.add_action(
                    self.tile,
                    hy,
                    crate::names::op::COPY,
                    Operand::Link {
                        tile: row,
                        trait_id: t::Y,
                    },
                );
                ui.set_number(self.tile, sh, 0.0);
                ui.add_action(
                    self.tile,
                    sh,
                    crate::names::op::COPY,
                    Operand::Link {
                        tile: row,
                        trait_id: t::HEIGHT,
                    },
                );
            }
        }
    }

    /// The scrollbar's position (its `_current_value`).
    pub fn scroll(&self, ui: &mut Ui) -> f32 {
        let Some(bar) = self.scrollbar(ui) else {
            return 0.0;
        };
        let current = name(ui, "_current_value");
        ui.number(bar, current)
    }

    /// Moves the scrollbar to a step (0 to its steps - 1), as the code does
    /// when it scrolls (`007db680`): `_current_value` set (its operators
    /// dropped), `_SetInCode` poked so the marker follows.
    pub fn set_scroll(&mut self, ui: &mut Ui, step: f32) {
        let Some(bar) = self.scrollbar(ui) else {
            return;
        };
        let items = name(ui, "_number_of_items");
        let last = (ui.number(bar, items) - 1.0).max(0.0);
        let current = name(ui, "_current_value");
        ui.set_number(bar, current, step.clamp(0.0, last).floor());
        let poke = name(ui, "_SetInCode");
        ui.set_number(bar, poke, 1.0);
    }

    /// Chooses a row (the code's selection, `007d8950`) and scrolls it into
    /// view: up to its top when it starts above the list, down until its
    /// bottom shows when it ends below (how the game's list keeps the
    /// choice in view isn't traced: a guess).
    pub fn select(&mut self, ui: &mut Ui, index: Option<usize>) {
        self.selected = index.filter(|&i| i < self.rows.len());
        if let Some(i) = self.selected {
            let row = self.rows[i];
            let y = name(ui, "_y");
            let delta_id = name(ui, "_scroll_delta");
            let top = ui.number(row, y);
            let bottom = top + ui.number(row, t::HEIGHT);
            let delta = ui.number(self.tile, delta_id);
            let h = ui.number(self.tile, t::HEIGHT);
            if delta > 0.0 {
                let now = self.scroll(ui);
                if top < now * delta {
                    self.set_scroll(ui, (top / delta).floor());
                } else if bottom > now * delta + h {
                    self.set_scroll(ui, ((bottom - h) / delta).ceil());
                }
            }
        }
        self.update_highlight(ui);
    }

    /// Chooses a row without scrolling (the list's own choosing,
    /// `00764a00`, which the interface calls with the tile under the
    /// pointer: only the highlight follows), or none.
    pub fn choose(&mut self, ui: &mut Ui, index: Option<usize>) {
        self.selected = index.filter(|&i| i < self.rows.len());
        self.update_highlight(ui);
    }

    /// The row a tile is (its index in `rows`), for the pointer: the
    /// interface hands the menu the tile under it (`00717e70`).
    pub fn index_of(&self, tile: TileId) -> Option<usize> {
        self.rows.iter().position(|&r| r == tile)
    }

    /// Moves the choice by `by` rows, stopping at the ends (nothing chosen
    /// yet: the first row).
    pub fn step(&mut self, ui: &mut Ui, by: i32) {
        if self.rows.is_empty() {
            return;
        }
        let last = self.rows.len() as i32 - 1;
        let next = match self.selected {
            None => 0,
            Some(i) => (i as i32 + by).clamp(0, last),
        };
        self.select(ui, Some(next as usize));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::{Screen, SystemColors};

    fn ui() -> Ui {
        Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|_| None),
        )
    }

    /// `list_box.xml` and `list_box_template.xml` cut down to the traits
    /// that place rows (the game's files aren't part of the tests).
    const MENU: &str = r#"<menu name="m"><locus>&true;</locus>
      <hotrect name="list"><locus>&true;</locus><clipwindow>&true;</clipwindow><x>0</x><y>100</y><width>300</width><height>90</height>
        <_number_of_visible_items>0</_number_of_visible_items><_scroll_delta>26</_scroll_delta><_highlight_y>-1</_highlight_y><_selected_height>0</_selected_height>
        <hotrect name="lb_highlight_box"><y><copy src="parent()" trait="_highlight_y"/></y><height><copy src="parent()" trait="_selected_height"/><sub>1</sub></height></hotrect>
        <image name="lb_scrollbar"><_number_of_items>1</_number_of_items><_number_of_visible_items>1</_number_of_visible_items><_current_value>0</_current_value></image>
      </hotrect>
      <template name="row"><hotrect name="item"><height>0</height><_VerticalSpacing>20</_VerticalSpacing><_y></_y><locus>&true;</locus>
        <y><copy src="me()" trait="_y"/><sub><copy src="sibling(lb_scrollbar)" trait="_current_value"/><mul src="parent()" trait="_scroll_delta"/></sub></y>
        <visible><copy src="me()" trait="listindex"/><gte>0</gte></visible>
        <text name="ListItemText"><string><copy src="parent()" trait="string"/></string></text></hotrect></template>
      <template name="tall"><hotrect name="item"><height>40</height><_y></_y>
        <y><copy src="me()" trait="_y"/><sub><copy src="sibling(lb_scrollbar)" trait="_current_value"/><mul src="parent()" trait="_scroll_delta"/></sub></y></hotrect></template>
    </menu>"#;

    fn menu(ui: &mut Ui) -> TileId {
        ui.load_menu(MENU.as_bytes(), &mut |_| None).unwrap()
    }

    #[test]
    fn rows_stack_scroll_and_highlight() {
        let mut ui = ui();
        let m = menu(&mut ui);
        let list = ui.find(m, "list").unwrap();
        let mut lb = ListBox::new(m, list, "tall");
        for _ in 0..4 {
            lb.add(&mut ui, Some("x")).unwrap();
        }
        let y = ui.names.lookup("_y").unwrap();
        let ys: Vec<f32> = lb.rows.iter().map(|&r| ui.number(r, y)).collect();
        assert_eq!(ys, [0.0, 40.0, 80.0, 120.0]);
        let index: Vec<f32> = lb
            .rows
            .iter()
            .map(|&r| ui.number(r, t::LISTINDEX))
            .collect();
        assert_eq!(index, [0.0, 1.0, 2.0, 3.0]);
        // 160 of rows in 90: ceil(70 × 4 / 160) + 1 = 3 steps of 35.
        let bar = ui.find(m, "lb_scrollbar").unwrap();
        let items = ui.names.lookup("_number_of_items").unwrap();
        assert_eq!(ui.number(bar, items), 3.0);
        let delta = ui.names.lookup("_scroll_delta").unwrap();
        assert_eq!(ui.number(list, delta), 35.0);
        // Choosing the last row scrolls down until its bottom shows.
        lb.select(&mut ui, Some(3));
        assert_eq!(lb.scroll(&mut ui), 2.0);
        let hy = ui.names.lookup("_highlight_y").unwrap();
        assert_eq!(ui.number(list, hy), 120.0 - 70.0);
        let hb = ui.find(m, "lb_highlight_box").unwrap();
        assert_eq!(ui.number(hb, t::HEIGHT), 39.0);
        lb.step(&mut ui, -3);
        assert_eq!(lb.selected, Some(0));
        assert_eq!(lb.scroll(&mut ui), 0.0);
        lb.clear(&mut ui);
        assert!(lb.rows.is_empty());
        assert_eq!(ui.number(list, hy), -1.0);
        assert_eq!(ui.number(bar, items), 1.0);
    }

    #[test]
    fn rows_without_a_height_measure_their_text() {
        let mut ui = ui();
        let m = menu(&mut ui);
        let list = ui.find(m, "list").unwrap();
        let mut lb = ListBox::new(m, list, "row");
        // No fonts loaded: the text has no size, so the row is its
        // spacing alone.
        let row = lb.add(&mut ui, Some("Strength")).unwrap();
        assert_eq!(ui.number(row, t::HEIGHT), 20.0);
        assert_eq!(ui.number(row, t::ID), -1.0);
        let text = ui.find(m, "ListItemText").unwrap();
        assert_eq!(ui.string(text, t::STRING).as_deref(), Some("Strength"));
    }

    #[test]
    fn a_list_sized_by_its_first_row() {
        let mut ui = ui();
        let m = menu(&mut ui);
        let list = ui.find(m, "list").unwrap();
        let visible = ui.names.lookup("_number_of_visible_items").unwrap();
        ui.set_number(list, visible, 9.0);
        let mut lb = ListBox::new(m, list, "tall");
        lb.add(&mut ui, None).unwrap();
        // The stats menu's lists: 9 rows tall.
        assert_eq!(ui.number(list, t::HEIGHT), 360.0);
    }
}
