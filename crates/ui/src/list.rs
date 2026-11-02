//! The menus' list boxes: a `hotrect` built from `list_box.xml` (a scroll
//! bar, a highlight box) whose items are instances of a template made with
//! `list_box_template.xml`. The game keeps one helper object per list
//! (vtable `0107286c` in FalloutNV.exe, made by `007642f0`, set up by
//! `0075a0d0`); this follows its code:
//!
//! * adding an item (`00764430`): the template instanced under the list,
//!   its `id` -1 unless the template gives one, its `string` the item's
//!   text, then placed and counted;
//! * placing (`007269d0`): the item's `_y` is the height of the items
//!   before it; a template without a height of its own takes its first text
//!   child's measured height (`text::measure` with the text's font and wrap
//!   width) plus its `_VerticalSpacing`;
//! * the scroll bar (`00765210`): with the items taller than the list, the
//!   bar gets max(2, ceil((items' height - list height) × n / items'
//!   height) + 1) positions (n the items not filtered out), each
//!   `_scroll_delta` = (items' height - list height) / (positions - 1);
//! * the first item with `_number_of_visible_items` set makes the list's
//!   height that many items high (operators `copy` of the item's height,
//!   `mul` the number);
//! * the highlight (`00764ef0`): the list's `_highlight_y` and
//!   `_selected_height` copy the chosen item's `y` and `height` (-1 and 0
//!   with none), which the XML's `lb_highlight_box` reads;
//! * choosing (`00764a00`) only an enabled item of an enabled list;
//!   stepping (`00764a70`) by listindex, up, down or a page (the list's
//!   height over `_scroll_delta`), skipping disabled items;
//! * bringing the chosen item into view (`007a5e70`): the scroll bar's
//!   `_current_value` moves one step at a time until the item's `y` is in
//!   the list, then `_SetInCode` is poked.

use crate::font::Font;
use crate::names::{kind, op, t};
use crate::text;
use crate::tile::{Action, Operand, TileId, Ui};

/// One item: its tile, the number the menu gave it, and whether it's
/// filtered out (not counted, not placed).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ListItem {
    pub tile: TileId,
    pub value: i32,
    pub filtered: bool,
}

/// A list box and its items.
#[derive(Debug, Clone, Default)]
pub struct ListBox {
    /// The list's tile (the `hotrect` with `list_box.xml`), `+0xc`.
    pub list: Option<TileId>,
    /// Its `lb_scrollbar` (`child(lb_scrollbar)`), `+0x14`.
    pub scrollbar: Option<TileId>,
    /// The template items are made from, `+0x18`.
    pub template: String,
    /// The chosen item, `+0x10`.
    pub selected: Option<TileId>,
    pub items: Vec<ListItem>,
    /// The next `listindex`, `+0x1c`.
    count: i32,
    /// The items' height so far, `+0x20`.
    total: f32,
}

impl ListBox {
    /// `0075a0d0`: the list tile, its template, and its scroll bar.
    pub fn new(ui: &Ui, list: TileId, template: &str) -> ListBox {
        ListBox {
            list: Some(list),
            scrollbar: ui.select(list, "child(lb_scrollbar)"),
            template: template.to_string(),
            ..ListBox::default()
        }
    }

    fn custom(ui: &mut Ui, name: &str) -> i32 {
        ui.names.lookup_or_add(name).unwrap_or(0)
    }

    /// Adds an item (`00764430`): the template instanced under the list
    /// (from `menu`'s templates), `id` -1 if the template has none, its
    /// `string` set when there's text; placed, the scroll bar updated and
    /// its `listindex` given. Returns its tile.
    pub fn add(
        &mut self,
        ui: &mut Ui,
        menu: TileId,
        value: i32,
        text: Option<&str>,
    ) -> Option<TileId> {
        let list = self.list?;
        let tile = ui.instantiate(menu, list, &self.template)?;
        if !ui.has(tile, t::ID) {
            ui.set_number(tile, t::ID, -1.0);
        }
        if let Some(text) = text {
            ui.set_string(tile, t::STRING, text);
        }
        self.items.push(ListItem {
            tile,
            value,
            filtered: false,
        });
        self.place(ui, tile);
        self.update_scrollbar(ui);
        ui.set_number(tile, t::LISTINDEX, self.count as f32);
        self.count += 1;
        if self.items.len() == 1 {
            let visible = Self::custom(ui, "_number_of_visible_items");
            let n = ui.number(list, visible);
            if n > 0.0 {
                ui.link(list, t::HEIGHT, tile, t::HEIGHT);
                ui.tiles[list]
                    .traits
                    .get_mut(&t::HEIGHT)
                    .expect("just linked")
                    .actions
                    .push(Action {
                        op: op::MUL,
                        operand: Operand::Constant(n),
                    });
                ui.refresh();
            }
        }
        Some(tile)
    }

    /// Places an item (`007269d0`): `_y` the height so far; its height its
    /// own, or (0) its first text child's measured height plus its
    /// `_VerticalSpacing`.
    fn place(&mut self, ui: &mut Ui, tile: TileId) {
        let y = Self::custom(ui, "_y");
        ui.set_number(tile, y, self.total);
        let mut h = ui.number(tile, t::HEIGHT);
        if h == 0.0 {
            let text_child = ui.tiles[tile]
                .children
                .iter()
                .copied()
                .find(|&c| ui.tiles[c].kind == kind::TEXT);
            if let Some(text_tile) = text_child {
                let mut wrap = ui.number(text_tile, t::WRAPWIDTH);
                if wrap <= 0.0 {
                    wrap = f32::MAX;
                }
                let font_index = ui.number(text_tile, t::FONT) as i32;
                let string = ui.string(text_tile, t::STRING).unwrap_or_default();
                let measured = font_for(ui, font_index)
                    .map(|f| text::measure(&f, font_index as usize, string.as_bytes(), wrap, 0))
                    .map_or(0.0, |m| m.height);
                let spacing = Self::custom(ui, "_VerticalSpacing");
                h = measured + ui.number(tile, spacing);
            }
        }
        ui.set_number(tile, t::HEIGHT, h);
        self.total += h;
    }

    /// The scroll bar's positions and the list's `_scroll_delta`
    /// (`00765210`).
    fn update_scrollbar(&mut self, ui: &mut Ui) {
        let Some(list) = self.list else {
            return;
        };
        let h = ui.number(list, t::HEIGHT);
        let mut count = 1;
        if self.total > 0.0 && h > 0.0 {
            if h < self.total {
                let filtered = Self::custom(ui, "_num_filtered");
                let n = self.items.len() as i32 - text::round_half_even(ui.number(list, filtered));
                let v = ((self.total - h) * n as f32) / self.total;
                count = (v.ceil() as i32 + 1).max(2);
            }
            if count > 1 {
                let delta = Self::custom(ui, "_scroll_delta");
                ui.set_number(list, delta, (self.total - h) / (count - 1) as f32);
            }
        }
        if let Some(bar) = self.scrollbar {
            let items = Self::custom(ui, "_number_of_items");
            ui.set_number(bar, items, count as f32);
        }
    }

    /// Lays every item out again (`00764930`): `listindex` and place for the
    /// ones not filtered out, the scroll bar and the highlight.
    pub fn refresh(&mut self, ui: &mut Ui) {
        self.count = 0;
        self.total = 0.0;
        for item in self.items.clone() {
            if !item.filtered {
                ui.set_number(item.tile, t::LISTINDEX, self.count as f32);
                self.count += 1;
                self.place(ui, item.tile);
            }
        }
        self.update_scrollbar(ui);
        self.highlight(ui);
    }

    /// Empties the list (`007646d0`): every item's tile taken away, the
    /// counts and `_num_filtered` back to 0, nothing chosen.
    pub fn clear(&mut self, ui: &mut Ui) {
        for item in self.items.drain(..) {
            ui.detach(item.tile);
        }
        self.count = 0;
        self.total = 0.0;
        if let Some(list) = self.list {
            let filtered = Self::custom(ui, "_num_filtered");
            ui.set_number(list, filtered, 0.0);
        }
        self.update_scrollbar(ui);
        self.selected = None;
        self.highlight(ui);
    }

    /// The highlight box follows the chosen item (`00764ef0`).
    pub fn highlight(&mut self, ui: &mut Ui) {
        let Some(list) = self.list else {
            return;
        };
        let highlight_y = Self::custom(ui, "_highlight_y");
        let selected_height = Self::custom(ui, "_selected_height");
        match self.selected {
            None => {
                ui.set_number(list, highlight_y, -1.0);
                ui.set_number(list, selected_height, 0.0);
            }
            Some(item) => {
                ui.link(list, highlight_y, item, t::Y);
                ui.link(list, selected_height, item, t::HEIGHT);
                // The box (and its four edges) take the item's "failed
                // check" look (`0071aa60` / `0071aa80`).
                let failed = ui.tiles[item].failed;
                if let Some(bx) = ui.find_below(list, "lb_highlight_box") {
                    for edge in ["top", "bottom", "left", "right"] {
                        if let Some(e) = ui.find_below(bx, edge) {
                            ui.tiles[e].failed = failed;
                        }
                    }
                    ui.tiles[bx].failed = failed;
                }
            }
        }
        ui.refresh();
    }

    /// Whether the list takes choices: its `_enabled` (`00764e40`).
    pub fn enabled(&self, ui: &mut Ui) -> bool {
        let Some(list) = self.list else {
            return false;
        };
        let enabled = Self::custom(ui, "_enabled");
        ui.number(list, enabled) != 0.0
    }

    fn item_enabled(ui: &mut Ui, tile: TileId) -> bool {
        let enabled = Self::custom(ui, "_enabled");
        ui.number(tile, enabled) != 0.0
    }

    /// Chooses an item (`00764a00`, the list's first virtual function: the
    /// interface calls it with the tile the pointer is over). `None` clears
    /// the choice. Only an item of this list that is enabled, in an enabled
    /// list. Returns whether the list took it.
    pub fn select(&mut self, ui: &mut Ui, tile: Option<TileId>) -> bool {
        if !self.enabled(ui) {
            return false;
        }
        match tile {
            None => {
                self.selected = None;
                self.highlight(ui);
                true
            }
            Some(tile) => {
                if self.items.iter().any(|i| i.tile == tile) && Self::item_enabled(ui, tile) {
                    self.selected = Some(tile);
                    self.highlight(ui);
                    true
                } else {
                    false
                }
            }
        }
    }

    /// The item at a `listindex` (`007a30a0`).
    pub fn item_at(&self, ui: &mut Ui, index: i32) -> Option<TileId> {
        self.items
            .iter()
            .find(|i| ui.number(i.tile, t::LISTINDEX) as i32 == index)
            .map(|i| i.tile)
    }

    /// The number an item was added with (`007e1110`).
    pub fn value_of(&self, tile: TileId) -> Option<i32> {
        self.items.iter().find(|i| i.tile == tile).map(|i| i.value)
    }

    /// The item a key moves the choice to (`00764a70`): `special` 1 up, 2
    /// down, 15 a page up, 16 a page down (the list's height over its
    /// `_scroll_delta`, rounded down). From the chosen item by listindex
    /// (with none: the last item going up, the first going down), clamped to
    /// the list, then on past disabled items (and back from the far end).
    /// `None` when nothing else can be chosen.
    pub fn next(&self, ui: &mut Ui, special: i32) -> Option<TileId> {
        let list = self.list?;
        let page = {
            let h = ui.number(list, t::HEIGHT);
            let delta = match ui.names.lookup("_scroll_delta") {
                Some(d) => ui.number(list, d),
                None => 0.0,
            };
            if delta == 0.0 {
                0.0
            } else {
                h / delta
            }
        };
        // The page as the game rounds it (`004bd510`: to the nearest whole
        // number, the fraction measured from the number truncated, so a
        // half rounds up going down and never going up).
        let round = |v: f32| v.trunc() + if v - v.trunc() >= 0.5 { 1.0 } else { 0.0 };
        let step = match special {
            1 => -1,
            2 => 1,
            15 => round(-page) as i32,
            16 => round(page) as i32,
            _ => 0,
        };
        if !self.enabled(ui) || step == 0 || self.count == 0 {
            return None;
        }
        let last = self.count - 1;
        let mut index = match self.selected {
            None => {
                if step > 0 {
                    0
                } else {
                    last
                }
            }
            Some(s) => (ui.number(s, t::LISTINDEX) as i32 + step).clamp(0, last),
        };
        // Onwards in the step's direction past disabled items.
        loop {
            match self.item_at(ui, index) {
                Some(tile) if Self::item_enabled(ui, tile) => {
                    return (Some(tile) != self.selected).then_some(tile);
                }
                None => return None,
                _ => {}
            }
            if step > 0 {
                index += 1;
                if index > last {
                    break;
                }
            } else {
                index -= 1;
                if index < 0 {
                    break;
                }
            }
        }
        // None that way: the game then looks from the other end.
        let order: Vec<i32> = if step > 0 {
            (0..last).rev().collect()
        } else {
            (1..=last).collect()
        };
        for i in order {
            if let Some(tile) = self.item_at(ui, i) {
                if Self::item_enabled(ui, tile) {
                    return (Some(tile) != self.selected).then_some(tile);
                }
            }
        }
        None
    }

    /// How many items aren't filtered out (`0071ae60`: the count less the
    /// list's `_num_filtered`).
    pub fn shown_count(&self, ui: &mut Ui) -> usize {
        let Some(list) = self.list else {
            return 0;
        };
        let filtered = Self::custom(ui, "_num_filtered");
        let n = text::round_half_even(ui.number(list, filtered)).max(0) as usize;
        self.items.len().saturating_sub(n)
    }

    /// Filters the items (`00730bb0`): `hidden` says which (by value) are
    /// left out; those get `listindex` -1 (the template hides them), the
    /// rest are numbered and placed in order; the list's `_num_filtered` is
    /// the count left out. A chosen item that's left out gives its place to
    /// the item that now has its `listindex` (else the last one shown).
    pub fn filter(&mut self, ui: &mut Ui, hidden: &dyn Fn(i32) -> bool) {
        self.count = 0;
        self.total = 0.0;
        let mut left_out = 0;
        let mut lost: Option<i32> = None;
        for i in 0..self.items.len() {
            let item = self.items[i];
            let out = hidden(item.value);
            self.items[i].filtered = out;
            if !out {
                if lost == Some(self.count) {
                    self.selected = Some(item.tile);
                }
                ui.set_number(item.tile, t::LISTINDEX, self.count as f32);
                self.count += 1;
                self.place(ui, item.tile);
            } else {
                if self.selected == Some(item.tile) {
                    lost = Some(ui.number(item.tile, t::LISTINDEX) as i32);
                    self.selected = None;
                }
                ui.set_number(item.tile, t::LISTINDEX, -1.0);
                left_out += 1;
            }
        }
        if let Some(list) = self.list {
            let filtered = Self::custom(ui, "_num_filtered");
            ui.set_number(list, filtered, left_out as f32);
        }
        self.update_scrollbar(ui);
        if let Some(index) = lost.filter(|_| self.selected.is_none()) {
            self.selected = self.item_at(ui, index);
            if self.selected.is_none() {
                let last = self.shown_count(ui) as i32 - 1;
                self.selected = self.item_at(ui, last);
            }
        }
        self.highlight(ui);
    }

    /// Sorts the items (the inventory lists' `00730b80`: `0083fd60` puts
    /// them in an array and `007653f0` sorts it with a Shell sort, gaps
    /// 1, 4, 13 … from the first above (n - 1) / 9, an item moving back
    /// while `before(it, the one a gap back)`), then lays them out again
    /// (`0071a670`).
    pub fn sort(&mut self, ui: &mut Ui, before: &dyn Fn(&mut Ui, TileId, TileId) -> bool) {
        let n = self.items.len();
        let mut gap = 1usize;
        while n > 0 && gap <= (n - 1) / 9 {
            gap = gap * 3 + 1;
        }
        while gap > 0 {
            for i in gap..n {
                let it = self.items[i];
                let mut j = i;
                while j >= gap && before(ui, it.tile, self.items[j - gap].tile) {
                    self.items[j] = self.items[j - gap];
                    j -= gap;
                }
                self.items[j] = it;
            }
            gap /= 3;
        }
        self.refresh(ui);
    }

    /// Takes an item out (its tile too).
    pub fn remove(&mut self, ui: &mut Ui, tile: TileId) {
        if let Some(at) = self.items.iter().position(|i| i.tile == tile) {
            self.items.remove(at);
            ui.detach(tile);
            if self.selected == Some(tile) {
                self.selected = None;
            }
        }
    }

    /// The items with a `listindex` from `from` (`007314c0`'s walk: in
    /// order, stopping at the first at or past `to`).
    pub fn shown_items(&self, ui: &mut Ui, from: i32, to: i32) -> Vec<ListItem> {
        let mut out = Vec::new();
        for item in &self.items {
            let index = ui.number(item.tile, t::LISTINDEX) as i32;
            if index >= to {
                break;
            }
            if index >= from {
                out.push(*item);
            }
        }
        out
    }

    /// Scrolls the chosen item into view (`007a5e70`): while its `y` is
    /// above the list or its bottom below it, the scroll bar's
    /// `_current_value` moves a step toward it (stopping if it no longer
    /// changes); then `_SetInCode` is poked so the bar's marker follows.
    pub fn scroll_to_selected(&mut self, ui: &mut Ui) {
        let (Some(list), Some(bar), Some(item)) = (self.list, self.scrollbar, self.selected) else {
            return;
        };
        if ui.number(list, t::HEIGHT) <= 0.0 {
            return;
        }
        let current = Self::custom(ui, "_current_value");
        let out_of_view = |ui: &mut Ui| {
            let y = ui.number(item, t::Y);
            let above = y < 0.0;
            let below = ui.number(list, t::HEIGHT) < ui.number(item, t::HEIGHT) + y;
            (above, below)
        };
        let (mut above, mut below) = out_of_view(ui);
        let moved = above || below;
        let mut guard = 0;
        while ui.number(item, t::LISTINDEX) >= 0.0 && (above || below) && guard < 10_000 {
            guard += 1;
            let before = ui.number(bar, current);
            let to = before + if above { -1.0 } else { 1.0 };
            // Set without dropping the bar's operators (`00a012d0` with its
            // flag 0): the clamps in the XML still apply when worked out.
            set_keeping_operators(ui, bar, current, to);
            if ui.number(bar, current) == before {
                break;
            }
            let (a, b) = out_of_view(ui);
            above = a;
            below = b;
        }
        if moved {
            let set_in_code = Self::custom(ui, "_SetInCode");
            ui.set_number(bar, set_in_code, 1.0);
        }
    }
}

/// Gives a trait a new value but leaves its operators (the game's setter
/// with its "refresh" flag off, `00a012d0(trait, value, 0)`).
pub fn set_keeping_operators(ui: &mut Ui, tile: TileId, trait_id: i32, value: f32) {
    let actions = ui.tiles[tile]
        .traits
        .get(&trait_id)
        .map(|tr| tr.actions.clone())
        .unwrap_or_default();
    ui.set_number(tile, trait_id, value);
    if let Some(tr) = ui.tiles[tile].traits.get_mut(&trait_id) {
        tr.actions = actions;
    }
    ui.refresh();
}

/// A font by its number (1 to 8), if it's loaded.
pub fn font_for(ui: &Ui, index: i32) -> Option<Font> {
    let i = index.max(1) as usize;
    ui.fonts.get(i - 1).cloned().flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu::MenuCode;
    use crate::menus::test_support;

    struct Nothing;
    impl MenuCode for Nothing {
        fn class(&self) -> i32 {
            1
        }
        fn click(&mut self, _ui: &mut Ui, _id: i32, _tile: Option<TileId>, _now: f64) {}
    }

    fn list_menu(height: f32) -> (Ui, TileId, ListBox) {
        let mut ui = test_support::ui();
        let xml = format!(
            "<menu name=\"M\"><class>&MessageMenu;</class>{}
               <template name=\"T\"><hotrect name=\"Item\">{}
                 <text name=\"ListItemText\"><font>2</font><wrapwidth>60</wrapwidth><string><copy src=\"parent()\" trait=\"string\"/></string></text>
               </hotrect></template></menu>",
            test_support::list_box("L", 4, 300.0, &format!("{height}")),
            test_support::LIST_ITEM
        );
        let menu = test_support::load(&mut ui, &xml, &mut Nothing);
        let tile = ui.find(menu, "L").unwrap();
        let list = ListBox::new(&ui, tile, "T");
        (ui, menu, list)
    }

    /// `00764430`, `007269d0`: each item placed under the last, as high as
    /// its text measured at its wrap width plus `_VerticalSpacing`;
    /// numbered by `listindex`.
    #[test]
    fn items_are_placed_by_their_measured_text() {
        let (mut ui, menu, mut list) = list_menu(1000.0);
        let a = list.add(&mut ui, menu, 10, Some("One")).unwrap();
        // "two words" wraps at 60 into two lines: 20, + 24 and font 2's
        // extra 4, + 20.
        let b = list.add(&mut ui, menu, 11, Some("two words")).unwrap();
        let y = ui.names.lookup("_y").unwrap();
        assert_eq!(ui.number(a, y), 0.0);
        assert_eq!(ui.number(a, t::HEIGHT), 40.0);
        assert_eq!(ui.number(b, y), 40.0);
        assert_eq!(ui.number(b, t::HEIGHT), 20.0 + 24.0 + 4.0 + 20.0);
        assert_eq!(ui.number(b, t::LISTINDEX), 1.0);
        assert_eq!(ui.number(a, t::ID), -1.0);
        assert_eq!(list.value_of(b), Some(11));
        assert_eq!(list.item_at(&mut ui, 1), Some(b));
    }

    /// `00765210`: more items than fit give the scroll bar positions and a
    /// `_scroll_delta`.
    #[test]
    fn the_scroll_bar_counts_what_doesnt_fit() {
        let (mut ui, menu, mut list) = list_menu(100.0);
        for i in 0..5 {
            list.add(&mut ui, menu, i, Some("Item"));
        }
        // 5 × 40 = 200 in 100: ceil(100 × 5 / 200) + 1 = 4 positions.
        let bar = list.scrollbar.unwrap();
        let items = ui.names.lookup("_number_of_items").unwrap();
        assert_eq!(ui.number(bar, items), 4.0);
        let delta = ui.names.lookup("_scroll_delta").unwrap();
        let l = list.list.unwrap();
        assert!((ui.number(l, delta) - 100.0 / 3.0).abs() < 1e-4);
    }

    /// `00764ef0`, `00764a00`, `00764a70`: choosing moves the highlight;
    /// the keys step through enabled items.
    #[test]
    fn choosing_and_stepping() {
        let (mut ui, menu, mut list) = list_menu(1000.0);
        let a = list.add(&mut ui, menu, 0, Some("A")).unwrap();
        let b = list.add(&mut ui, menu, 1, Some("B")).unwrap();
        let c = list.add(&mut ui, menu, 2, Some("C")).unwrap();
        let l = list.list.unwrap();
        let hy = ui.names.lookup("_highlight_y").unwrap();
        assert_eq!(ui.number(l, hy), -1.0);
        assert!(list.select(&mut ui, Some(b)));
        assert_eq!(ui.number(l, hy), 40.0);
        let sh = ui.names.lookup("_selected_height").unwrap();
        assert_eq!(ui.number(l, sh), 40.0);
        assert_eq!(list.next(&mut ui, 2), Some(c));
        assert_eq!(list.next(&mut ui, 1), Some(a));
        // A disabled item is stepped over.
        let enabled = ui.names.lookup("_enabled").unwrap();
        ui.set_number(c, enabled, 0.0);
        assert_eq!(list.next(&mut ui, 2), None);
        assert!(!list.select(&mut ui, Some(c)));
        // The failed-check look follows the chosen item onto the box.
        ui.tiles[a].failed = true;
        list.select(&mut ui, Some(a));
        let bx = ui.find_below(l, "lb_highlight_box").unwrap();
        assert!(ui.tiles[bx].failed);
        // Nothing chosen: the first going down, the last enabled going up.
        list.select(&mut ui, None);
        assert_eq!(list.next(&mut ui, 2), Some(a));
        assert_eq!(list.next(&mut ui, 1), Some(b));
        list.clear(&mut ui);
        assert!(list.items.is_empty());
        assert_eq!(ui.number(l, hy), -1.0);
    }
}
