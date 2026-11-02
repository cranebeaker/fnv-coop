//! The Pip-Boy's weapon mod screen (`menus\item_mod_menu.xml`, class
//! `ItemModMenu` 1061, vtable `01073b7c` in FalloutNV.exe): ITEMS' Mod
//! button (id 19, X) on a chosen weapon opens it (`00780140` case 0x13:
//! `UIMenuMode`); a mod from the player's things is fitted to it. The rules
//! are `world::weapon_mods`'. Read from the code:
//!
//! * opening (`00783440`, `00784710`): the weapon (taken off while it's
//!   modded, put back after); its condition on the health card (4, `user0`
//!   condition / 100), its damage on the figure card (5), its name (0); the
//!   list (1, `IMM_ItemModListTemplate`, `007840f0`): the mods fitted to
//!   it, then the player's mods for it that aren't (`004d4ca0`); the fitted
//!   ones' text dimmed (alpha 127 on the line's first child, `007836d0`
//!   through `007b5370`) and sorted first (`00783810`); the first line
//!   chosen only with a pad (`004b71d0`, `00715860`), then `DoEnter` with
//!   the list's choice.
//! * the pointer on a mod (`DoEnter`, `00783ed0`, id 0xf, after the
//!   interface made the line the list's choice, `00717e70`): when the
//!   chosen line's `listindex` isn't the one the scroll knob last turned
//!   for (`011d9fd0`, a global, 0 at start) the knob turns a notch with
//!   `UIPipBoyScroll` (`007f8610`); the description (13) emptied, then with
//!   a mod chosen its `DESC`, the picture (3) the mod's (`004be200`), the
//!   button (11) bright. Off it (`DoLeave`, `00784050`, after the list lets
//!   its choice go): cards 9 and, with more than one line, 8 hidden.
//! * a click on a line, or Enter on the chosen one (`007838a0` case 0xf):
//!   when the clicked line's text has alpha 255 (a fitted mod's has 127)
//!   and the chosen line isn't the weapon, the chosen mod is fitted
//!   (`00783af0`), one used up, `UIItemGunsSmallUp`, and the list made
//!   again (`00784710(weapon, 0)`). E, Cancel (12, case 0xc): `UIMenuMode`,
//!   back to ITEMS. The wheel is the base menu's (`DoWheelMove`): only the
//!   list's scroll bar reads it.
//!
//! There is no hold-to-confirm meter: the alpha the click tests is the
//! fitted lines' dimming, and the file has no meter but the health cards.

use super::{by_id, text, trait_id, Action, Key};
use crate::listbox::ListBox;
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\item_mod_menu.xml";
/// The list's template.
pub const TEMPLATE: &str = "IMM_ItemModListTemplate";
/// A line's id (`00783ed0`).
pub const LINE_ID: i32 = 0xf;
/// The Cancel button.
pub const CANCEL: i32 = 12;
/// The sounds: a mod fitted (the exe's string has a trailing space), the
/// menu changing.
pub const FIT_SOUND: &str = "UIItemGunsSmallUp";
pub const MENU_SOUND: &str = "UIMenuMode";
/// A fitted mod's text alpha (`007836d0`: 0x42ff0000 = 127).
pub const FITTED_ALPHA: f32 = 127.0;

/// One mod on the list.
#[derive(Debug, Clone, PartialEq)]
pub struct ModRow {
    pub form: u32,
    pub name: String,
    pub description: String,
    /// Its picture (`ICON`), as written.
    pub icon: Option<String>,
    /// Already fitted to the weapon.
    pub fitted: bool,
}

/// What the screen shows.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemModInput {
    pub weapon: u32,
    pub name: String,
    /// Its condition in percent.
    pub condition: f32,
    pub icon: Option<String>,
    /// Its damage as the Pip-Boy shows it.
    pub damage: i32,
    pub rows: Vec<ModRow>,
}

/// The mod screen.
pub struct ItemModMenu {
    pub menu: TileId,
    pub list: ListBox,
    pub rows: Vec<ModRow>,
    pub input: Option<ItemModInput>,
    /// The chosen line's `listindex` the scroll knob last turned for
    /// (`011d9fd0`: a global, 0 at start, kept from one opening to the
    /// next).
    pub knob_index: i32,
}

impl ItemModMenu {
    /// Reads the menu (hidden).
    pub fn load(
        ui: &mut Ui,
        read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
    ) -> Result<ItemModMenu, String> {
        let menu = super::load_menu(ui, FILE, read)?;
        let list_tile = by_id(ui, menu, 1).unwrap_or(menu);
        ui.set_number(menu, t::VISIBLE, 0.0);
        Ok(ItemModMenu {
            menu,
            list: ListBox::new(menu, list_tile, TEMPLATE),
            rows: Vec::new(),
            input: None,
            knob_index: 0,
        })
    }

    fn tile(&self, ui: &Ui, id: i32) -> Option<TileId> {
        by_id(ui, self.menu, id)
    }

    /// Opens it on a weapon and shows it. Returns what the game should do
    /// (the scroll knob).
    pub fn open(&mut self, ui: &mut Ui, input: ItemModInput, pad: bool) -> Vec<Action> {
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        self.fill(ui, input, pad)
    }

    /// Fills it (`00784710` with the weapon, again after a fit): the
    /// weapon's picture, cards and the list made anew, the first line
    /// chosen only with a pad, then `DoEnter`. Returns what the game should
    /// do (the scroll knob).
    pub fn fill(&mut self, ui: &mut Ui, input: ItemModInput, pad: bool) -> Vec<Action> {
        if let Some(icon) = self.tile(ui, 3) {
            match &input.icon {
                Some(path) => {
                    ui.set_string(icon, t::FILENAME, path);
                    ui.set_number(icon, t::VISIBLE, 1.0);
                }
                None => ui.set_number(icon, t::VISIBLE, 0.0),
            }
        }
        if let Some(name) = self.tile(ui, 0) {
            ui.set_string(name, t::STRING, &input.name);
        }
        if let Some(card) = self.tile(ui, 4) {
            ui.set_number(card, t::USER0, input.condition / 100.0);
            ui.set_number(card, t::VISIBLE, 1.0);
        }
        if let Some(card) = self.tile(ui, 5) {
            let (title, value) = (trait_id(ui, "_Title"), trait_id(ui, "_Value"));
            let s = text(ui, "sInventoryDamage");
            ui.set_string(card, title, &s);
            ui.set_number(card, value, input.damage as f32);
        }
        self.list.clear(ui);
        // The fitted ones first (`00783810`), each group in its order.
        let mut rows = input.rows.clone();
        rows.sort_by_key(|r| !r.fitted);
        for row in &rows {
            let Some(tile) = self.list.add(ui, Some(&row.name)) else {
                continue;
            };
            ui.set_number(tile, t::ID, LINE_ID as f32);
            let index = ui.number(tile, t::LISTINDEX);
            ui.set_number(tile, t::VISIBLE, if index >= 0.0 { 1.0 } else { 0.0 });
            if row.fitted {
                // On the line's text (`007b5370`: its first child).
                if let Some(&text) = ui.tiles[tile].children.first() {
                    ui.set_number(text, t::ALPHA, FITTED_ALPHA);
                }
            }
        }
        self.rows = rows;
        self.input = Some(input);
        if pad {
            self.list.select(ui, Some(0));
        }
        let out = self.entered(ui);
        ui.refresh();
        out
    }

    /// `DoEnter` on a line (`00783ed0` case 0xf) with the list's choice as
    /// it is: the scroll knob, the description emptied, then the chosen
    /// mod's description and picture, the button bright.
    fn entered(&mut self, ui: &mut Ui) -> Vec<Action> {
        let mut out = Vec::new();
        let index = self
            .list
            .selected
            .and_then(|i| self.list.rows.get(i).copied())
            .map_or(0, |tile| ui.number(tile, t::LISTINDEX) as i32);
        if index != self.knob_index {
            out.push(Action::Sound("UIPipBoyScroll".into()));
            out.push(Action::ScrollKnob {
                down: self.knob_index < index,
            });
            self.knob_index = index;
        }
        let row = self.list.selected.and_then(|i| self.rows.get(i)).cloned();
        if let Some(desc) = self.tile(ui, 13) {
            ui.set_string(desc, t::STRING, "");
            if let Some(r) = &row {
                ui.set_string(desc, t::STRING, &r.description);
            }
        }
        if let Some(r) = row {
            if let (Some(icon), Some(path)) = (self.tile(ui, 3), r.icon.as_deref()) {
                ui.set_string(icon, t::FILENAME, path);
            }
            if let Some(b) = self.tile(ui, 11) {
                let line_alpha = trait_id(ui, "_line_alpha");
                ui.set_number(b, line_alpha, 255.0);
            }
        }
        ui.refresh();
        out
    }

    /// The pointer onto a tile: a line becomes the list's choice (the
    /// interface's list box, `00717e70`), then `DoEnter` (`00783ed0`).
    pub fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) -> Vec<Action> {
        if id != LINE_ID {
            return Vec::new();
        }
        let Some(index) = self.list.index_of(tile) else {
            return Vec::new();
        };
        self.list.choose(ui, Some(index));
        self.entered(ui)
    }

    /// The pointer off a tile: the list lets its choice go when it was
    /// that line (`00717ef0`), then `DoLeave` (`00784050` case 0xf): card 9
    /// hidden, and with more than one line 8 too.
    pub fn unmouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        let index = self.list.index_of(tile);
        if index.is_some() && self.list.selected == index {
            self.list.choose(ui, None);
        }
        if id != LINE_ID {
            return;
        }
        if let Some(card) = self.tile(ui, 9) {
            ui.set_number(card, t::VISIBLE, 0.0);
        }
        if self.list.rows.len() > 1 {
            if let Some(card) = self.tile(ui, 8) {
                ui.set_number(card, t::VISIBLE, 0.0);
            }
        }
        ui.refresh();
    }

    /// A click on a line (`007838a0` case 0xf; Enter clicks the chosen
    /// line): the chosen mod fitted when the clicked line's text has alpha
    /// 255 (not a fitted one's 127). Cancel (case 0xc) is
    /// [`super::Pipboy::cancel_item_mod`]'s.
    pub fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>) -> Vec<Action> {
        let mut out = Vec::new();
        let Some(tile) = tile.filter(|_| id == LINE_ID) else {
            return out;
        };
        let bright = ui.tiles[tile]
            .children
            .first()
            .copied()
            .is_some_and(|text| ui.number(text, t::ALPHA) >= 255.0);
        let row = self.list.selected.and_then(|i| self.rows.get(i));
        if let (true, Some(row), Some(input)) = (bright, row, self.input.as_ref()) {
            out.push(Action::FitMod {
                weapon: input.weapon,
                item: row.form,
            });
        }
        out
    }

    /// Hides it.
    pub fn hide(&mut self, ui: &mut Ui) {
        ui.set_number(self.menu, t::VISIBLE, 0.0);
        self.input = None;
        ui.refresh();
    }

    /// A key: Up and Down choose a line (then `DoEnter`: the knob), Enter
    /// clicks the chosen line (`007838a0` case 0xf). Cancel is
    /// [`super::Pipboy::cancel_item_mod`]'s.
    pub fn key(&mut self, ui: &mut Ui, key: Key) -> Vec<Action> {
        match key {
            Key::Up | Key::Down => {
                let before = self.list.selected;
                self.list.step(ui, if key == Key::Down { 1 } else { -1 });
                if self.list.selected == before {
                    return Vec::new();
                }
                self.entered(ui)
            }
            Key::Activate => {
                let tile = self
                    .list
                    .selected
                    .and_then(|i| self.list.rows.get(i).copied());
                self.click(ui, LINE_ID, tile)
            }
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::pipboy::tests::ui;

    /// `item_mod_menu.xml` cut down to the tiles the code finds by `id`,
    /// the list laid out for the pointer: lines 30 high from y 100.
    pub(crate) const MENU: &str = r#"<menu name="ItemModMenu"><locus>&true;</locus>
      <_PCButton_E> IMM_CancelButton </_PCButton_E>
      <rect name="IMM_ItemName"><id>0</id><string></string></rect>
      <hotrect name="IMM_ItemModList"><id>1</id><x>0</x><y>100</y><width>430</width><height>400</height>
        <locus>&true;</locus><target>&false;</target><wheelable>&true;</wheelable>
        <_scroll_delta>0</_scroll_delta><_highlight_y>-1</_highlight_y><_selected_height>0</_selected_height>
        <image name="lb_scrollbar"><_number_of_items>1</_number_of_items><_current_value>0</_current_value></image>
      </hotrect>
      <image name="IMM_ItemIcon"><id>3</id></image>
      <rect name="IMM_ItemHealth"><id>4</id><user0>0</user0><visible>&false;</visible></rect>
      <rect name="IMM_ItemStat"><id>5</id><_Title></_Title><_Value></_Value></rect>
      <text name="IMM_ChooseItemText"><id>8</id><visible>&false;</visible></text>
      <rect name="IMM_ModItemHealth"><id>9</id><visible>&false;</visible></rect>
      <image name="IMM_ItemModButton"><id>11</id><_line_alpha>128</_line_alpha></image>
      <image name="IMM_CancelButton"><id>12</id><x>900</x><y>700</y><width>50</width><height>20</height><target>&true;</target></image>
      <text name="IMM_ModDesc"><id>13</id><string></string></text>
      <template name="IMM_ItemModListTemplate"><hotrect name="IMM_ItemModListTemplateRect"><height>30</height><width>393</width>
        <y><copy src="me()" trait="_y"/></y><target>&true;</target>
        <text name="ListItemText"><string><copy src="parent()" trait="string"/></string><alpha>255</alpha></text>
      </hotrect></template>
    </menu>"#;

    pub(crate) fn menu(ui: &mut Ui) -> ItemModMenu {
        ItemModMenu::load(ui, &mut |p| (p == FILE).then(|| MENU.as_bytes().to_vec())).unwrap()
    }

    fn row(form: u32, fitted: bool) -> ModRow {
        ModRow {
            form,
            name: format!("Mod {form:X}"),
            description: format!("Does {form:X}."),
            icon: Some(format!("interface\\icons\\mod{form:x}.dds")),
            fitted,
        }
    }

    pub(crate) fn input() -> ItemModInput {
        ItemModInput {
            weapon: 0x10,
            name: "9mm Pistol".into(),
            condition: 50.0,
            icon: Some("interface\\icons\\pistol.dds".into()),
            damage: 7,
            rows: vec![row(0x21, false), row(0x20, true), row(0x22, false)],
        }
    }

    fn text_of(ui: &mut Ui, m: &ItemModMenu, id: i32) -> String {
        let tile = by_id(ui, m.menu, id).unwrap();
        ui.string(tile, t::STRING).unwrap_or_default()
    }

    /// `00784710`, `007836d0`, `00783ed0`, `007838a0`: the fitted mod first,
    /// its text dimmed; without a pad nothing chosen; the pointer onto a
    /// mod shows its description and picture and turns the knob; a click
    /// on the fitted one does nothing, on another fits it.
    #[test]
    fn the_pointer_shows_and_fits_mods() {
        let mut ui = ui();
        let mut m = menu(&mut ui);
        assert!(m.open(&mut ui, input(), false).is_empty());
        let forms: Vec<u32> = m.rows.iter().map(|r| r.form).collect();
        assert_eq!(forms, vec![0x20, 0x21, 0x22]);
        assert_eq!(m.list.selected, None);
        assert_eq!(text_of(&mut ui, &m, 13), "");
        let fitted_text = ui.tiles[m.list.rows[0]].children[0];
        assert_eq!(ui.number(fitted_text, t::ALPHA), FITTED_ALPHA);
        let other_text = ui.tiles[m.list.rows[1]].children[0];
        assert_eq!(ui.number(other_text, t::ALPHA), 255.0);
        let icon = by_id(&ui, m.menu, 3).unwrap();
        assert_eq!(
            ui.string(icon, t::FILENAME).unwrap(),
            "interface\\icons\\pistol.dds"
        );

        // Onto the fitted one (listindex 0, as the knob's: no turn).
        let first = m.list.rows[0];
        assert!(m.mouseover(&mut ui, LINE_ID, first).is_empty());
        assert_eq!(text_of(&mut ui, &m, 13), "Does 20.");
        assert_eq!(
            ui.string(icon, t::FILENAME).unwrap(),
            "interface\\icons\\mod20.dds"
        );
        let button = by_id(&ui, m.menu, 11).unwrap();
        let line_alpha = ui.names.lookup("_line_alpha").unwrap();
        assert_eq!(ui.number(button, line_alpha), 255.0);
        assert!(m.click(&mut ui, LINE_ID, Some(first)).is_empty());
        m.unmouseover(&mut ui, LINE_ID, first);
        assert_eq!(m.list.selected, None);

        // Onto the last: the knob on, its description; a click fits it.
        let last = m.list.rows[2];
        assert_eq!(
            m.mouseover(&mut ui, LINE_ID, last),
            vec![
                Action::Sound("UIPipBoyScroll".into()),
                Action::ScrollKnob { down: true }
            ]
        );
        assert_eq!(text_of(&mut ui, &m, 13), "Does 22.");
        assert_eq!(
            m.click(&mut ui, LINE_ID, Some(last)),
            vec![Action::FitMod {
                weapon: 0x10,
                item: 0x22
            }]
        );
        // Keys: Up to the middle one, Enter fits it.
        let out = m.key(&mut ui, Key::Up);
        assert_eq!(
            out,
            vec![
                Action::Sound("UIPipBoyScroll".into()),
                Action::ScrollKnob { down: false }
            ]
        );
        assert_eq!(
            m.key(&mut ui, Key::Activate),
            vec![Action::FitMod {
                weapon: 0x10,
                item: 0x21
            }]
        );
    }

    /// With a pad the first line is chosen as it opens (`004b71d0`,
    /// `00715860`), and again when the list is made anew after a fit.
    #[test]
    fn with_a_pad_the_first_line_is_chosen() {
        let mut ui = ui();
        let mut m = menu(&mut ui);
        m.open(&mut ui, input(), true);
        assert_eq!(m.list.selected, Some(0));
        assert_eq!(text_of(&mut ui, &m, 13), "Does 20.");
        m.key(&mut ui, Key::Down);
        let mut after = input();
        after.rows[0].fitted = true;
        m.fill(&mut ui, after, true);
        assert_eq!(m.list.selected, Some(0));
    }
}
