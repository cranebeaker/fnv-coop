//! ITEMS (`inventory_menu.xml`, the `InventoryMenu` class): five tabs
//! along a tab line (Weapons, Apparel, Aid, Misc, Ammo), the items of the
//! tab in a list, and a card for the chosen item. Filled as `0077fc10`
//! (setup), `00782a90` (the headline), `00782850` (a row) and `00707e30`
//! (the item card, shared with the container and barter menus) fill it.

use super::{by_id, text, trait_id, Action, ItemLine, ItemTab, Key, PipboyInput};
use crate::listbox::ListBox;
use crate::names::t;
use crate::tabline;
use crate::tile::{TileId, Ui};

/// The item card's cards (`item_stats_display.xml`'s children, in order),
/// and the bit of `00707e30`'s mask that shows each (read from its
/// disassembly: bit 0x1 the first child, 0x2 the second, ... 0x800 the
/// twelfth, 0x1000 the thirteenth).
pub const CARDS: [(&str, u32); 13] = [
    ("DamageResistInfo", 0x1),
    ("DPSInfo", 0x2),
    ("WeightInfo", 0x4),
    ("ValueInfo", 0x8),
    ("ConditionInfo", 0x10),
    ("AmmoInfo", 0x20),
    ("EffectsInfo", 0x40),
    ("ModInfoOne", 0x80),
    ("ModInfoTwo", 0x100),
    ("ModInfoThree", 0x200),
    ("StrengthReqInfo", 0x400),
    ("DAMInfo", 0x800),
    ("DamageThresholdInfo", 0x1000),
];

/// What a pad button did ([`ItemsMenu::pad_button`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PadPress {
    /// The button with this `id` clicked.
    Click(i32),
    /// Its button can't be clicked now.
    Refused,
    Nothing,
}

/// The ITEMS menu.
pub struct ItemsMenu {
    pub menu: TileId,
    pub tab: usize,
    pub list: ListBox,
    pub tabline: Option<TileId>,
    pub tabs: Vec<TileId>,
    card: Option<TileId>,
    icon: Option<TileId>,
    /// The forms of the rows now, in order (0 for the keyring's row).
    pub shown: Vec<u32>,
    filled: Option<(usize, bool, Vec<ItemLine>, bool)>,
    /// The last row the pointer was over (`011d9f34`, its
    /// `listindex`): the knob clicks when it changes.
    pub(crate) hovered: Option<usize>,
    /// The keyring is open (the menu's `_KeyringOpen`, `011d9eb8`): the
    /// list shows the keys (`00782810`).
    pub keyring: bool,
    /// The hot key wheel (`IM_HotKeyWheel`, id 5; `HotKeysWheel` (Xbox
    /// PDB)), shown while a number key is held, and the hot key it
    /// highlights (`+0x24` / `+0x28`).
    wheel: Option<TileId>,
    pub hotkey: Option<usize>,
}

/// How many hot keys there are (`006e4ba0`: 8), the controls they are
/// (0x11 + n: Hotkey1 .. Hotkey8), and the one that isn't a hot key on the
/// wheel (n 1: the "2" key, Ammo Swap's control; the Pip-Boy's wheel never
/// shows or fills it, `00781ba0`, `00701bd0`, `007017b0`, so nothing is
/// ever put on it; `0077da60` still uses slot 1 when the key comes up, which
/// does nothing).
pub const HOTKEYS: usize = 8;
pub const NOT_A_HOTKEY: usize = 1;

/// The keyring's row's `id` (`00782a90`: a row without an item, `sKeyring`,
/// last on the Misc tab when keys are carried).
pub const KEYRING_ID: i32 = 0x1e;
/// The Cancel button's `id` (`IM_CancelButton`, E: shown while the
/// keyring is open; closes it, `00780140` case 10).
pub const CANCEL_ID: i32 = 10;
/// The keyring's picture (`00780ff0` on its row).
pub const KEYRING_PICTURE: &str = "Interface\\Icons\\PipboyImages\\Items\\item_keyring.dds";

/// The rows' `id` (0x1d), which the click and mouse-over handlers look for
/// (`00780140`, `00780ff0`).
pub const ROW_ID: i32 = 0x1d;
/// The Drop button's `id` (`IM_DropButton`, shown with a pad only; the
/// right mouse button clicks it, `00781ba0`).
pub const DROP_ID: i32 = 7;
/// The Repair button's `id` (`IM_RepairButton`, `00780140` case 8).
pub const REPAIR_ID: i32 = 8;
/// The Mod button's `id` (`00780140` case 0x13).
pub const MOD_ID: i32 = 0x13;
/// The first tab button's `id` (0x18 Weapons .. 0x1c Ammo, `0077fc10`).
pub const FIRST_TAB_ID: i32 = 0x18;

/// The help message a tab button asks for as it's pressed, the tab shown
/// or not (`00780140`): Apparel (0x19) the apparel message (0x23,
/// 512 ms), Ammo (0x1c) the ammo message (0x24, 500 ms), over ITEMS.
fn tab_tutorial(id: i32) -> Option<Action> {
    let (tutorial, delay) = match id {
        0x19 => (0x23, 512),
        0x1c => (0x24, 500),
        _ => return None,
    };
    Some(Action::Tutorial {
        id: tutorial,
        menu: super::ITEMS_CLASS,
        delay,
    })
}

/// An item's row text (`00782850`): "name (count)" for more than one,
/// "name+ (count)" when they're a modded weapon.
pub fn row_text(item: &ItemLine) -> String {
    if item.count > 1 {
        let plus = if item.modded { "+" } else { "" };
        format!("{}{plus} ({})", item.name, item.count)
    } else {
        item.name.clone()
    }
}

/// The card mask for an item (`00707e30`): weapons 0xc3e (DPS, weight,
/// value, condition, ammunition ("--" without), strength, damage; the
/// damage card stays at the file's alpha 0, so it doesn't show), apparel
/// 0x3c (weight, value, condition, and the ammunition card holding the
/// weight class) with DR or DT (0x1 / 0x1000; "--" in DR with neither),
/// everything else weight and value (0xc), with the effects (0x40) when
/// it has any (aid, ammunition, a weapon mod, an enchanted weapon or
/// piece of apparel; never a modded weapon: `world::item_card::
/// card_effects` leaves those without).
pub fn card_mask(item: &ItemLine) -> u32 {
    let mut m = match item.tab {
        ItemTab::Weapons if item.damage.is_some() => 0xc3e,
        ItemTab::Apparel => {
            let mut m = 0x3c;
            let dr = item.damage_resistance.unwrap_or(0.0);
            let dt = item.damage_threshold.unwrap_or(0.0);
            if dr == 0.0 && dt == 0.0 {
                m |= 0x1;
            }
            if dr != 0.0 {
                m |= 0x1;
            }
            if dt != 0.0 {
                m |= 0x1000;
            }
            m
        }
        _ => 0xc,
    };
    if item.effects.as_deref().is_some_and(|e| !e.is_empty()) {
        m |= 0x40;
    }
    m
}

/// The damage card's text (`00707e30`, the value of `006450f0`): "%d" of
/// the damage rounded to a whole number (`004bd510`: halves up), or with
/// more than one projectile a shot "%.1fx%d" (each one's share × the
/// count).
pub fn damage_text(damage: f32, projectiles: u32) -> String {
    if projectiles < 2 {
        format!("{}", round_half_up(damage))
    } else {
        format!("{:.1}x{}", damage / projectiles as f32, projectiles)
    }
}

/// `004bd510` with a step of 1: the whole part, plus one when what's left
/// is at least a half.
pub(crate) fn round_half_up(v: f32) -> i32 {
    let whole = v.trunc();
    whole as i32 + i32::from(v - whole >= 0.5)
}

/// A count × amount as the card writes it (`00707e30`): "--" for nothing,
/// `%.1f` below 1, else `%.0f` (the value); the weight `%.2f`.
fn value_text(v: f32) -> String {
    if v <= 0.0 {
        "--".into()
    } else if v < 1.0 {
        format!("{v:.1}")
    } else {
        format!("{v:.0}")
    }
}

impl ItemsMenu {
    /// Reads the menu and builds its tab line (`0077fc10`: ids from 24,
    /// `sInventoryWeapons` .. `sInventoryAmmoTab`) and the cards' titles.
    pub fn load(
        ui: &mut Ui,
        read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
    ) -> Result<ItemsMenu, String> {
        let menu = super::load_menu(ui, super::ITEMS_FILE, read)?;
        let list_tile = by_id(ui, menu, 4).unwrap_or(menu);
        let list = ListBox::new(menu, list_tile, "IM_InventoryListTemplate");
        let tabline_tile = by_id(ui, menu, 13);
        let labels: Vec<String> = [
            "sInventoryWeapons",
            "sInventoryApparel",
            "sInventoryAid",
            "sInventoryMisc",
            "sInventoryAmmoTab",
        ]
        .iter()
        .map(|s| text(ui, s))
        .collect();
        let refs: Vec<&str> = labels.iter().map(String::as_str).collect();
        let tabs = match tabline_tile {
            Some(tl) => tabline::build(ui, menu, tl, 0x18, &refs),
            None => Vec::new(),
        };
        let title = trait_id(ui, "_Title");
        for (id, setting) in [
            (14, "sInventoryDamage"),
            (15, "sInventoryDamagePerSecond"),
            (16, "sInventoryStrReq"),
            (17, "sInventoryDamageResistance"),
            (18, "sInventoryDamageThreshold"),
        ] {
            if let Some(tile) = by_id(ui, menu, id) {
                let v = text(ui, setting);
                ui.set_string(tile, title, &v);
            }
        }
        if let Some(cancel) = by_id(ui, menu, 10) {
            let v = text(ui, "sCancel");
            ui.set_string(cancel, t::STRING, &v);
        }
        // The headline's DR card sits where the DT card does: New Vegas
        // shows damage threshold (a guess at which the code hides).
        if let Some(dr) = by_id(ui, menu, 2) {
            ui.set_number(dr, t::VISIBLE, 0.0);
        }
        let mut m = ItemsMenu {
            menu,
            tab: 0,
            list,
            tabline: tabline_tile,
            tabs,
            card: by_id(ui, menu, 12),
            icon: by_id(ui, menu, 11),
            shown: Vec::new(),
            filled: None,
            hovered: None,
            keyring: false,
            wheel: by_id(ui, menu, 5),
            hotkey: None,
        };
        if let Some(tl) = m.tabline {
            tabline::set_current(ui, tl, 0);
        }
        m.show_card(ui, None);
        Ok(m)
    }

    /// The headline (`00782a90`): caps, hit points "%d/%d", damage
    /// threshold "%1.1f", weight "%d/%d"; then the tab's rows.
    pub fn fill(&mut self, ui: &mut Ui, input: &PipboyInput) {
        let value = trait_id(ui, "_Value");
        let set = |ui: &mut Ui, id: i32, v: String| {
            if let Some(tile) = by_id(ui, self.menu, id) {
                ui.set_string(tile, value, &v);
            }
        };
        set(ui, 0, format!("{}", input.caps));
        set(
            ui,
            1,
            format!("{}/{}", input.health.0 as i32, input.health.1 as i32),
        );
        set(ui, 2, format!("{}", input.damage_resistance as i32));
        set(ui, 21, format!("{:.1}", input.damage_threshold));
        set(
            ui,
            3,
            format!("{}/{}", input.weight.0 as i32, input.weight.1 as i32),
        );
        let items: Vec<ItemLine> = self.tab_items(input);
        let keyring_row = self.keyring_row(input);
        let now = (self.tab, self.keyring, items.clone(), keyring_row);
        if self.filled.as_ref() != Some(&now) {
            self.fill_rows(ui, &items, keyring_row);
            self.filled = Some(now);
        }
    }

    /// The items in the list (`007824e0`, the list's order: names
    /// compared; the same name by condition, not kept here): the tab's,
    /// or with the keyring open the keys (`00782810`).
    fn tab_items(&self, input: &PipboyInput) -> Vec<ItemLine> {
        let mut items: Vec<ItemLine> = if self.keyring {
            input.keys.clone()
        } else {
            input
                .items
                .iter()
                .filter(|i| i.tab as usize == self.tab)
                .cloned()
                .collect()
        };
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    /// Whether the keyring's row shows: on the Misc tab (`00782620` lets
    /// the row without an item through there only) with keys carried
    /// (`004c6ba0(0x2e)`), the keyring closed; sorted last (`007824e0`).
    fn keyring_row(&self, input: &PipboyInput) -> bool {
        !self.keyring && self.tab == ItemTab::Misc as usize && !input.keys.is_empty()
    }

    fn fill_rows(&mut self, ui: &mut Ui, items: &[ItemLine], keyring_row: bool) {
        let keep = self.list.selected;
        self.list.clear(ui);
        self.shown.clear();
        for item in items {
            let Some(row) = self.list.add(ui, Some(&row_text(item))) else {
                continue;
            };
            // Rows' `id` 0x1d (29), what the click handler looks for.
            ui.set_number(row, t::ID, ROW_ID as f32);
            if let Some(marker) = ui.find_below(row, "IM_Template_ItemMarker") {
                ui.set_number(marker, t::VISIBLE, if item.equipped { 1.0 } else { 0.0 });
            }
            self.shown.push(item.form);
        }
        if keyring_row {
            let name = text(ui, "sKeyring");
            if let Some(row) = self.list.add(ui, Some(&name)) {
                ui.set_number(row, t::ID, KEYRING_ID as f32);
                if let Some(marker) = ui.find_below(row, "IM_Template_ItemMarker") {
                    ui.set_number(marker, t::VISIBLE, 0.0);
                }
                self.shown.push(0);
            }
        }
        let rows = self.shown.len();
        let chosen = (rows > 0).then(|| keep.unwrap_or(0).min(rows - 1));
        self.list.select(ui, chosen);
        self.show_row(ui, chosen, items);
    }

    /// The card for a row: an item's, or the keyring's picture alone.
    fn show_row(&mut self, ui: &mut Ui, row: Option<usize>, items: &[ItemLine]) {
        match row.and_then(|i| self.shown.get(i).copied()) {
            Some(0) => self.show_keyring(ui),
            _ => self.show_card(ui, row.and_then(|i| items.get(i))),
        }
    }

    /// The keyring's row chosen (`00780ff0` case 0x1e): no item chosen
    /// (`00781b10`), its picture shown.
    fn show_keyring(&mut self, ui: &mut Ui) {
        self.show_card(ui, None);
        if let Some(icon) = self.icon {
            ui.set_string(icon, t::FILENAME, KEYRING_PICTURE);
            ui.set_number(icon, t::VISIBLE, 1.0);
        }
    }

    /// Opens or closes the keyring (`00780140` cases 0x1e and 10): the
    /// menu's `_KeyringOpen`, the list's filter, the list made again.
    pub fn set_keyring(&mut self, ui: &mut Ui, open: bool, input: &PipboyInput) {
        self.keyring = open;
        let id = trait_id(ui, "_KeyringOpen");
        ui.set_number(self.menu, id, if open { 1.0 } else { 0.0 });
        self.list.selected = None;
        self.filled = None;
        self.fill(ui, input);
    }

    /// The chosen item's card and picture (`00780ff0` on a row,
    /// `00707e30` the card): `_EquippableItem`, the picture
    /// (`Interface\Icons\` + the item's own), the cards its mask shows.
    fn show_card(&mut self, ui: &mut Ui, item: Option<&ItemLine>) {
        self.update_buttons(ui, item);
        let equippable = trait_id(ui, "_EquippableItem");
        ui.set_number(
            self.menu,
            equippable,
            if item.is_some_and(|i| i.usable) {
                1.0
            } else {
                0.0
            },
        );
        if let Some(icon) = self.icon {
            match item.and_then(|i| i.icon.clone()) {
                Some(path) => {
                    ui.set_string(icon, t::FILENAME, &path);
                    ui.set_number(icon, t::VISIBLE, 1.0);
                }
                None => ui.set_number(icon, t::VISIBLE, 0.0),
            }
        }
        let Some(card) = self.card else {
            return;
        };
        let mask = item.map_or(0, card_mask);
        let title = trait_id(ui, "_Title");
        let value = trait_id(ui, "_Value");
        for (name, bit) in CARDS {
            let Some(tile) = ui.find_below(card, name) else {
                continue;
            };
            ui.set_number(tile, t::VISIBLE, if mask & bit != 0 { 1.0 } else { 0.0 });
            let Some(item) = item else {
                continue;
            };
            let count = item.count.max(1) as f32;
            let v = match name {
                "DamageResistInfo" => item
                    .damage_resistance
                    .filter(|&v| v != 0.0)
                    .map_or("--".into(), |v| format!("{}", v as i32)),
                "DamageThresholdInfo" => item
                    .damage_threshold
                    .map_or("--".into(), |v| format!("{}", v as i32)),
                "DAMInfo" => item
                    .damage
                    .map_or("--".into(), |v| damage_text(v, item.projectiles)),
                // `00645380`'s value, "%d" rounded half up (`004bd510`).
                "DPSInfo" => item
                    .dps
                    .map_or(String::new(), |v| format!("{}", round_half_up(v))),
                "StrengthReqInfo" => {
                    // Shown for weapons, made opaque by the code (the
                    // file's alpha is 0): "%d".
                    if mask & bit != 0 {
                        ui.set_number(tile, t::ALPHA, 255.0);
                    }
                    format!("{}", item.strength.unwrap_or(0).max(0))
                }
                "ValueInfo" => value_text(item.value as f32 * count),
                "WeightInfo" => {
                    let w = item.weight * count;
                    if w <= 0.0 {
                        "--".into()
                    } else {
                        format!("{w:.2}")
                    }
                }
                "EffectsInfo" => item.effects.clone().unwrap_or_default(),
                "AmmoInfo" => {
                    // The card's title: a weapon's ammunition ("--"
                    // without), apparel's weight class
                    // (`sArmorWeightLight` / `Medium` / `Heavy`).
                    let text = match (item.tab, item.weight_class) {
                        (ItemTab::Apparel, Some(c)) => super::text(
                            ui,
                            [
                                "sArmorWeightLight",
                                "sArmorWeightMedium",
                                "sArmorWeightHeavy",
                            ][usize::from(c.min(2))],
                        ),
                        _ => item.ammo.clone().unwrap_or_else(|| "--".into()),
                    };
                    ui.set_string(tile, title, &text);
                    continue;
                }
                "ConditionInfo" => {
                    // The condition meter reads the card's `user5`
                    // (at most 1); `user6` 1 for weapons moves the repair
                    // arrows, 0 for apparel.
                    ui.set_number(tile, t::USER0 + 5, item.condition.unwrap_or(1.0).min(1.0));
                    let weapon = item.tab == ItemTab::Weapons;
                    ui.set_number(tile, t::USER0 + 6, if weapon { 1.0 } else { 0.0 });
                    continue;
                }
                _ => continue,
            };
            ui.set_string(tile, value, &v);
        }
        // The effects card's height (`00707e30`): half the card's height
        // less 20, twice that when the condition or ammunition card shows
        // (a second row).
        if let Some(effects) = ui.find_below(card, "EffectsInfo") {
            let mut y = ui.number(card, t::HEIGHT) / 2.0 - 20.0;
            if mask & 0x30 != 0 {
                y += y;
            }
            ui.set_number(effects, t::Y, y);
        }
    }

    /// The buttons' `target`s (their lines brighten with it). Translated
    /// from 00781680 (decompiled, FalloutNV.exe 1.4.0.525),
    /// `InventoryMenu::UpdateButtons` (Xbox PDB): with no item chosen,
    /// Equip (6), Drop (7), Repair (8), Hot key (9) and Mod (19) can't be
    /// pressed; with one, Equip as the item can be equipped or used, Drop
    /// and Mod always, Hot key unless it's ammunition (form type 0x29),
    /// Repair when it can be repaired (`00781860`). (The Equip button's
    /// text, set there too, shows with a pad only: not here.)
    fn update_buttons(&mut self, ui: &mut Ui, item: Option<&ItemLine>) {
        let targets: [(i32, bool); 5] = match item {
            None => [(6, false), (7, false), (8, false), (9, false), (19, false)],
            Some(i) => [
                (6, i.usable),
                (7, true),
                (9, i.tab != ItemTab::Ammo),
                (19, true),
                (8, i.repairable),
            ],
        };
        for (id, on) in targets {
            if let Some(tile) = by_id(ui, self.menu, id) {
                ui.set_number(tile, t::TARGET, if on { 1.0 } else { 0.0 });
            }
        }
    }

    /// The chosen row's item.
    pub fn chosen(&self, input: &PipboyInput) -> Option<ItemLine> {
        let items = self.tab_items(input);
        self.list.selected.and_then(|i| items.get(i)).cloned()
    }

    /// Whether the hot key wheel shows (`00701740`).
    pub fn wheel_shown(&self, ui: &mut Ui) -> bool {
        self.wheel.is_some_and(|w| ui.number(w, t::VISIBLE) != 0.0)
    }

    /// The wheel's eight places (`007017b0`, `HotKeysWheel::UpdateHotkeyList`
    /// (Xbox PDB)): each hot key's item's picture, `_HotKeyAssigned`, and its
    /// name ("%s (%d)" for more than one); the "2" place is the ammunition
    /// swap's picture and isn't filled.
    fn fill_wheel(&mut self, ui: &mut Ui, input: &PipboyInput) {
        let Some(wheel) = self.wheel else {
            return;
        };
        let assigned = trait_id(ui, "_HotKeyAssigned");
        let icon = trait_id(ui, "_HotKeyIcon");
        for n in (0..HOTKEYS).filter(|&n| n != NOT_A_HOTKEY) {
            let Some(place) = ui.find_below(wheel, &format!("HK_Item_{n}")) else {
                continue;
            };
            let item = input.hotkeys[n].and_then(|f| input.items.iter().find(|i| i.form == f));
            match item {
                None => {
                    ui.set_number(place, assigned, 0.0);
                    ui.set_string(place, t::STRING, "");
                }
                Some(item) => {
                    ui.set_string(place, icon, item.icon.as_deref().unwrap_or(""));
                    ui.set_number(place, assigned, 1.0);
                    let text = if item.count < 2 {
                        item.name.clone()
                    } else {
                        format!("{} ({})", item.name, item.count)
                    };
                    ui.set_string(place, t::STRING, &text);
                }
            }
        }
    }

    /// Highlights a hot key on the wheel (`00701e00`): its `_SelectedHotkey`
    /// and `_SelectedText` (that place's name; none for -1 and the "2").
    fn highlight_hotkey(&mut self, ui: &mut Ui, n: usize) {
        let Some(wheel) = self.wheel else {
            return;
        };
        let selected = trait_id(ui, "_SelectedHotkey");
        let text_id = trait_id(ui, "_SelectedText");
        ui.set_number(wheel, selected, n as f32);
        let text = if n == NOT_A_HOTKEY {
            String::new()
        } else {
            ui.find_below(wheel, &format!("HK_Item_{n}"))
                .and_then(|p| ui.string(p, t::STRING))
                .unwrap_or_default()
        };
        ui.set_string(wheel, text_id, &text);
        self.hotkey = Some(n);
    }

    /// The number keys held, every frame (`00781ba0` with a keyboard): the
    /// first hot key whose key is down (or held) that isn't the "2" shows
    /// the wheel (filled, `007017b0`) and highlights it; with none down the
    /// wheel hides. Not while the keyring is open. `down[n]` is hot key n's
    /// key held now.
    // Translated from 00781ba0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn hotkey_keys(&mut self, ui: &mut Ui, down: [bool; HOTKEYS], input: &PipboyInput) {
        let Some(wheel) = self.wheel else {
            return;
        };
        let mut found = false;
        if !self.keyring {
            for (n, &d) in down.iter().enumerate() {
                if d && n != NOT_A_HOTKEY {
                    if !self.wheel_shown(ui) {
                        self.fill_wheel(ui, input);
                        ui.set_number(wheel, t::VISIBLE, 1.0);
                        // The item's picture gives way to the wheel (a
                        // tile of the menu's hidden here, back below; read
                        // as `IM_ItemIcon`).
                        if let Some(icon) = self.icon {
                            ui.set_number(icon, t::VISIBLE, 0.0);
                        }
                    }
                    self.highlight_hotkey(ui, n);
                    found = true;
                    break;
                }
            }
        }
        if !found && self.wheel_shown(ui) {
            ui.set_number(wheel, t::VISIBLE, 0.0);
            self.hotkey = None;
            // An item chosen: its picture back.
            if let (Some(icon), Some(_)) = (self.icon, self.list.selected) {
                ui.set_number(icon, t::VISIBLE, 1.0);
            }
        }
    }

    /// A row clicked with the wheel up (`00780140` case 0x1d with a
    /// keyboard): a broken item can't go on a hot key
    /// (`sCantHotkeyBrokenItem`), nor one that can't be equipped or used,
    /// or ammunition (`sCantHotkeyItem`); else it goes on the highlighted
    /// one (`007019e0`), and the wheel shows it.
    fn assign_hotkey(&mut self, ui: &mut Ui, item: &ItemLine, input: &PipboyInput) -> Vec<Action> {
        let refuse = |name: &str| {
            vec![
                Action::Notice(text(ui, name)),
                Action::Sound("UIVATSInsufficientAP".into()),
            ]
        };
        if item.condition == Some(0.0) {
            return refuse("sCantHotkeyBrokenItem");
        }
        if !item.usable || item.tab == ItemTab::Ammo {
            return refuse("sCantHotkeyItem");
        }
        let Some(slot) = self.hotkey.filter(|&n| n != NOT_A_HOTKEY) else {
            return Vec::new();
        };
        let mut after = input.clone();
        for h in after.hotkeys.iter_mut() {
            if *h == Some(item.form) {
                *h = None;
            }
        }
        after.hotkeys[slot] = Some(item.form);
        self.fill_wheel(ui, &after);
        self.highlight_hotkey(ui, slot);
        vec![Action::SetHotkey {
            slot,
            item: item.form,
        }]
    }

    /// Shows a tab (0 Weapons .. 4 Ammo).
    pub fn show_tab(&mut self, ui: &mut Ui, tab: usize, input: &PipboyInput) {
        // Another tab closes the keyring (`00780140` cases 0x18 .. 0x1c).
        if self.keyring {
            self.keyring = false;
            let id = trait_id(ui, "_KeyringOpen");
            ui.set_number(self.menu, id, 0.0);
        }
        self.tab = tab.min(4);
        if let Some(tl) = self.tabline {
            tabline::set_current(ui, tl, self.tab);
        }
        self.list.selected = None;
        self.filled = None;
        self.fill(ui, input);
    }

    /// A key (`00782190`): left and right change tab, wrapping round the
    /// five (the knob turning, `UIPipBoyTab`), up and down choose a row
    /// (`UIPipBoyScroll`), the A button equips, takes off or uses the
    /// chosen item. (Page Up presses Mod and Page Down the hot keys: not
    /// here yet.)
    /// The pad's X or Y (Shift + Enter, Alt + Enter: `0070c4a0` turns
    /// them into 0xb and 0xc for `0070f6e0`): the `xbuttonx` / `xbuttony`
    /// reference found from the chosen row up to the menu (`IM_DropButton`,
    /// `IM_RepairButton`). A shown tile that can be clicked is; a shown one
    /// that can't gets `UIMenuCancel` (`00717280(2)`); a hidden one
    /// (without a pad, `_Has360Controller` off) nothing.
    pub fn pad_button(&self, ui: &mut Ui, key: Key) -> PadPress {
        let trait_id = match key {
            Key::ButtonX => 4063,
            Key::ButtonY => 4064,
            _ => return PadPress::Nothing,
        };
        let from = self
            .list
            .selected
            .and_then(|i| self.list.rows.get(i).copied())
            .unwrap_or(self.menu);
        match crate::menu::reference(ui, from, trait_id) {
            Some((tile, target)) if target == t::CLICKED && ui.shown(tile) => {
                if ui.number(tile, t::TARGET) != 0.0 {
                    PadPress::Click(ui.number(tile, t::ID) as i32)
                } else {
                    PadPress::Refused
                }
            }
            _ => PadPress::Nothing,
        }
    }

    pub fn key(&mut self, ui: &mut Ui, key: Key, input: &PipboyInput) -> Vec<Action> {
        let mut out = Vec::new();
        match key {
            Key::Left | Key::Right => {
                let tab = if key == Key::Right {
                    (self.tab + 1) % 5
                } else {
                    (self.tab + 4) % 5
                };
                self.show_tab(ui, tab, input);
                out.push(Action::Sound("UIPipBoyTab".into()));
                // The key presses the tab's button (`00782190` → the
                // click handler).
                out.extend(tab_tutorial(FIRST_TAB_ID + tab as i32));
            }
            Key::Up | Key::Down => {
                let before = self.list.selected;
                self.list.step(ui, if key == Key::Down { 1 } else { -1 });
                if self.list.selected != before {
                    let items = self.tab_items(input);
                    self.show_row(ui, self.list.selected, &items);
                    out.push(Action::Sound("UIPipBoyScroll".into()));
                }
            }
            // The A button presses the chosen row: the keyring's opens it.
            Key::Activate
                if self
                    .list
                    .selected
                    .is_some_and(|i| self.shown.get(i) == Some(&0)) =>
            {
                self.set_keyring(ui, true, input);
            }
            Key::Activate if self.wheel_shown(ui) => {
                let items = self.tab_items(input);
                if let Some(item) = self.list.selected.and_then(|i| items.get(i)).cloned() {
                    out.extend(self.assign_hotkey(ui, &item, input));
                }
            }
            Key::Activate => {
                let items = self.tab_items(input);
                if let Some(action) = self
                    .list
                    .selected
                    .and_then(|i| items.get(i))
                    .and_then(|i| self.activate(i))
                {
                    out.push(action);
                }
            }
            _ => {}
        }
        out
    }

    /// What equipping or using a row's item asks of the game (`00780140`
    /// case 0x1d: weapons and apparel are equipped or taken off, the rest
    /// used), when it can be.
    fn activate(&self, item: &ItemLine) -> Option<Action> {
        item.usable.then_some(match item.tab {
            ItemTab::Weapons | ItemTab::Apparel => Action::Equip(item.form),
            _ => Action::Use(item.form),
        })
    }

    /// A tile clicked (`00780140`, slot 0x0c): a tab button (0x18 .. 0x1c)
    /// turns to its tab when it isn't the one shown; a row (0x1d) equips,
    /// takes off or uses its item; Drop (7, the right button on PC) drops
    /// the chosen item (the game's checks and "how many?" are the
    /// caller's); Repair (8) on a chosen item that can be mended and Mod
    /// (0x13) on a chosen weapon open their screens (`UIMenuMode`; the
    /// caller answers with [`super::Pipboy::open_repair`] /
    /// [`super::Pipboy::open_item_mod`]); the keyring (0x1e) and its
    /// Cancel (10) open and close it.
    pub fn click(
        &mut self,
        ui: &mut Ui,
        id: i32,
        tile: Option<TileId>,
        input: &PipboyInput,
    ) -> Vec<Action> {
        let mut out = Vec::new();
        match id {
            FIRST_TAB_ID..=0x1c => {
                let tab = (id - FIRST_TAB_ID) as usize;
                if tab != self.tab {
                    self.show_tab(ui, tab, input);
                    out.push(Action::Sound("UIPipBoyTab".into()));
                }
                out.extend(tab_tutorial(id));
            }
            // Not from the keyring: the refusal sound.
            DROP_ID => {
                let items = self.tab_items(input);
                if let Some(item) = self.list.selected.and_then(|i| items.get(i)) {
                    out.push(if self.keyring {
                        Action::Sound("UIVATSInsufficientAP".into())
                    } else {
                        Action::Drop(item.form)
                    });
                }
            }
            REPAIR_ID => {
                if let Some(item) = self.chosen(input).filter(|i| i.repairable) {
                    out.push(Action::Sound(super::repair::MENU_SOUND.into()));
                    out.push(Action::OpenRepair(item.form));
                }
            }
            MOD_ID => {
                if let Some(item) = self.chosen(input).filter(|i| i.tab == ItemTab::Weapons) {
                    out.push(Action::Sound(super::item_mod::MENU_SOUND.into()));
                    out.push(Action::OpenItemMod(item.form));
                }
            }
            KEYRING_ID if !self.keyring => self.set_keyring(ui, true, input),
            CANCEL_ID if self.keyring => self.set_keyring(ui, false, input),
            ROW_ID if self.wheel_shown(ui) => {
                let items = self.tab_items(input);
                let index = tile
                    .and_then(|t| self.list.index_of(t))
                    .or(self.list.selected);
                if let Some(item) = index.and_then(|i| items.get(i)).cloned() {
                    out.extend(self.assign_hotkey(ui, &item, input));
                }
            }
            ROW_ID => {
                let items = self.tab_items(input);
                let index = tile.and_then(|t| self.list.index_of(t));
                if let Some(action) = index
                    .and_then(|i| items.get(i))
                    .and_then(|i| self.activate(i))
                {
                    out.push(action);
                }
            }
            _ => {}
        }
        out
    }

    /// The pointer onto a tile (`00780ff0`, slot 0x10): a row becomes the
    /// chosen item, its card and picture shown; the knob clicks
    /// (`007f8610`, `UIPipBoyScroll`) when the row's `listindex` differs from
    /// the last one the pointer was over.
    pub fn mouseover(
        &mut self,
        ui: &mut Ui,
        id: i32,
        tile: TileId,
        input: &PipboyInput,
    ) -> Vec<Action> {
        let mut out = Vec::new();
        if id == KEYRING_ID {
            // The keyring's row (`00780ff0` case 0x1e): chosen, its
            // picture; no knob.
            if let Some(index) = self.list.index_of(tile) {
                self.list.choose(ui, Some(index));
                self.show_keyring(ui);
            }
            return out;
        }
        if id != ROW_ID {
            return out;
        }
        let Some(index) = self.list.index_of(tile) else {
            return out;
        };
        if self.hovered != Some(index) {
            self.hovered = Some(index);
            out.push(Action::Sound("UIPipBoyScroll".into()));
        }
        self.list.choose(ui, Some(index));
        let items = self.tab_items(input);
        self.show_card(ui, items.get(index));
        out
    }

    /// The pointer off a tile (`00781620`, slot 0x14; the interface first
    /// lets the list drop its choice, `00717ef0`): leaving a row clears the
    /// card and `_EquippableItem`.
    pub fn unmouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        if id != ROW_ID && id != KEYRING_ID {
            return;
        }
        if let Some(index) = self.list.index_of(tile) {
            if self.list.selected == Some(index) {
                self.list.choose(ui, None);
            }
        }
        self.show_card(ui, None);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn item(name: &str, tab: ItemTab) -> ItemLine {
        ItemLine {
            form: 0x100,
            name: name.into(),
            count: 1,
            tab,
            equipped: false,
            usable: true,
            value: 100,
            weight: 1.5,
            icon: None,
            damage: None,
            dps: None,
            projectiles: 1,
            damage_resistance: None,
            damage_threshold: None,
            condition: None,
            strength: None,
            ammo: None,
            weight_class: None,
            effects: None,
            repairable: false,
            modded: false,
        }
    }

    /// `00782850`: the count after more than one; "+" for modded ones.
    #[test]
    fn row_texts() {
        let mut gun = item("9mm Pistol", ItemTab::Weapons);
        assert_eq!(row_text(&gun), "9mm Pistol");
        gun.count = 2;
        assert_eq!(row_text(&gun), "9mm Pistol (2)");
        gun.modded = true;
        assert_eq!(row_text(&gun), "9mm Pistol+ (2)");
    }

    #[test]
    fn the_cards_shown_follow_the_item_card_code() {
        let mut gun = item("9mm Pistol", ItemTab::Weapons);
        gun.damage = Some(10.0);
        // Ammunition or not, a weapon's cards are 0xc3e.
        assert_eq!(card_mask(&gun), 0xc3e);
        gun.ammo = Some("9mm (13/11)".into());
        assert_eq!(card_mask(&gun), 0xc3e);
        // Apparel: weight, value, condition, weight class; DR shows "--"
        // with neither DR nor DT, DT only when it has one.
        let mut armour = item("Leather Armor", ItemTab::Apparel);
        armour.damage_resistance = Some(0.0);
        armour.damage_threshold = Some(0.0);
        assert_eq!(card_mask(&armour), 0x3d);
        armour.damage_threshold = Some(6.0);
        assert_eq!(card_mask(&armour), 0x103c);
        // Aid and ammunition with effects add the effects card.
        let mut stimpak = item("Stimpak", ItemTab::Aid);
        assert_eq!(card_mask(&stimpak), 0xc);
        stimpak.effects = Some("Restore Health".into());
        assert_eq!(card_mask(&stimpak), 0x4c);
        assert_eq!(card_mask(&item("Tin Can", ItemTab::Misc)), 0xc);
        // An enchanted weapon shows its effects too (`00707e30`; a modded
        // one has none to show).
        gun.effects = Some("PER +1".into());
        assert_eq!(card_mask(&gun), 0xc7e);
    }

    #[test]
    fn numbers_are_written_as_the_card_writes_them() {
        // `004bd510`: halves up.
        assert_eq!(damage_text(28.5, 1), "29");
        assert_eq!(damage_text(28.4, 1), "28");
        // "%.1fx%d" with more than one projectile a shot.
        assert_eq!(damage_text(45.0, 9), "5.0x9");
        assert_eq!(value_text(0.0), "--");
        assert_eq!(value_text(0.5), "0.5");
        assert_eq!(value_text(12.4), "12");
        let mut stimpaks = item("Stimpak", ItemTab::Aid);
        assert_eq!(row_text(&stimpaks), "Stimpak");
        stimpaks.count = 3;
        assert_eq!(row_text(&stimpaks), "Stimpak (3)");
    }

    /// `inventory_menu.xml` and `item_stats_display.xml` cut down to the
    /// tiles the code finds by `id` and name.
    const MENU: &str = r#"<menu name="InventoryMenu"><locus>&true;</locus>
      <xbuttonx><ref src="IM_DropButton" trait="clicked"/></xbuttonx>
      <image name="IM_DropButton"><id>7</id><target>&false;</target>
        <visible><copy src="globals()" trait="_Has360Controller"/></visible></image>
      <rect name="caps"><id>0</id></rect><rect name="hp"><id>1</id></rect>
      <rect name="dr"><id>2</id><visible>&true;</visible></rect><rect name="wg"><id>3</id></rect>
      <rect name="dt"><id>21</id></rect>
      <hotrect name="list"><id>4</id><x>0</x><y>100</y><width>300</width><height>400</height>
        <_scroll_delta>0</_scroll_delta><_highlight_y>-1</_highlight_y><_selected_height>0</_selected_height>
        <image name="lb_scrollbar"><_number_of_items>1</_number_of_items><_current_value>0</_current_value></image>
      </hotrect>
      <template name="IM_InventoryListTemplate"><hotrect name="row"><height>30</height>
        <image name="IM_Template_ItemMarker"><visible>&false;</visible></image></hotrect></template>
      <image name="IM_ItemIcon"><id>11</id><visible>&false;</visible></image>
      <rect name="ItemData"><id>12</id><height>100</height>
        <rect name="DamageResistInfo"><visible>&false;</visible></rect>
        <rect name="DPSInfo"><visible>&false;</visible></rect>
        <rect name="WeightInfo"><visible>&false;</visible></rect>
        <rect name="ValueInfo"><visible>&false;</visible></rect>
        <rect name="ConditionInfo"><visible>&false;</visible></rect>
        <rect name="AmmoInfo"><visible>&false;</visible></rect>
        <rect name="EffectsInfo"><visible>&false;</visible></rect>
        <rect name="StrengthReqInfo"><alpha>0</alpha><visible>&true;</visible></rect>
        <rect name="DAMInfo"><alpha>0</alpha><visible>&false;</visible></rect>
        <rect name="DamageThresholdInfo"><visible>&false;</visible></rect>
      </rect>
    </menu>"#;

    fn input() -> PipboyInput {
        let mut gun = item("9mm Pistol", ItemTab::Weapons);
        gun.form = 0x200;
        gun.equipped = true;
        gun.damage = Some(10.0);
        gun.dps = Some(28.5);
        gun.strength = Some(2);
        gun.condition = Some(1.0);
        gun.ammo = Some("9mm (13/11)".into());
        let mut armour = item("Combat Armor", ItemTab::Apparel);
        armour.form = 0x300;
        armour.damage_resistance = Some(0.0);
        armour.damage_threshold = Some(0.0);
        armour.weight_class = Some(1);
        armour.condition = Some(1.0);
        PipboyInput {
            items: vec![armour, gun],
            caps: 250,
            health: (150.0, 200.0),
            damage_threshold: 6.5,
            weight: (17.0, 200.0),
            ..PipboyInput::default()
        }
    }

    #[test]
    fn the_menu_shows_the_headline_the_tab_and_the_chosen_items_card() {
        let mut ui = crate::pipboy::tests::ui();
        let mut read = |p: &str| (p == crate::pipboy::ITEMS_FILE).then(|| MENU.as_bytes().to_vec());
        let mut m = ItemsMenu::load(&mut ui, &mut read).unwrap();
        let input = input();
        m.fill(&mut ui, &input);
        ui.refresh();
        let value = ui.names.lookup("_Value").unwrap();
        let shown = |ui: &mut Ui, id: i32| {
            let tile = by_id(ui, m.menu, id).unwrap();
            ui.string(tile, value).unwrap_or_default()
        };
        assert_eq!(shown(&mut ui, 0), "250");
        assert_eq!(shown(&mut ui, 1), "150/200");
        assert_eq!(shown(&mut ui, 21), "6.5");
        assert_eq!(shown(&mut ui, 3), "17/200");
        // The DR card gives way to DT.
        let dr = by_id(&ui, m.menu, 2).unwrap();
        assert_eq!(ui.number(dr, t::VISIBLE), 0.0);
        // The weapons tab: one row, its marker on (it's equipped).
        assert_eq!(m.shown, [0x200]);
        let marker = ui
            .find_below(m.list.rows[0], "IM_Template_ItemMarker")
            .unwrap();
        assert_eq!(ui.number(marker, t::VISIBLE), 1.0);
        let card = by_id(&ui, m.menu, 12).unwrap();
        let tile = |ui: &Ui, name: &str| ui.find_below(card, name).unwrap();
        let title = ui.names.lookup("_Title").unwrap();
        let dps = tile(&ui, "DPSInfo");
        assert_eq!(ui.number(dps, t::VISIBLE), 1.0);
        assert_eq!(ui.string(dps, value).unwrap(), "29");
        // Strength made opaque, the damage card left at the file's alpha 0.
        let strength = tile(&ui, "StrengthReqInfo");
        assert_eq!(ui.number(strength, t::ALPHA), 255.0);
        assert_eq!(ui.string(strength, value).unwrap(), "2");
        let dam = tile(&ui, "DAMInfo");
        assert_eq!(ui.number(dam, t::ALPHA), 0.0);
        assert_eq!(ui.string(dam, value).unwrap(), "10");
        let ammo = tile(&ui, "AmmoInfo");
        assert_eq!(ui.string(ammo, title).unwrap(), "9mm (13/11)");
        assert_eq!(ui.number(tile(&ui, "ConditionInfo"), t::USER0 + 6), 1.0);
        // The effects card: half the card's height less 20, twice that
        // under a second row.
        assert_eq!(ui.number(tile(&ui, "EffectsInfo"), t::Y), 60.0);
        // Activating equips; right turns to Apparel, whose button asks
        // for the apparel help (`00780140`).
        assert_eq!(
            m.key(&mut ui, Key::Activate, &input),
            [Action::Equip(0x200)]
        );
        assert_eq!(
            m.key(&mut ui, Key::Right, &input),
            [
                Action::Sound("UIPipBoyTab".into()),
                Action::Tutorial {
                    id: 0x23,
                    menu: 1002,
                    delay: 512
                }
            ]
        );
        ui.refresh();
        assert_eq!(m.shown, [0x300]);
        assert_eq!(ui.string(ammo, title).unwrap(), "Medium");
        let dr_card = tile(&ui, "DamageResistInfo");
        assert_eq!(ui.number(dr_card, t::VISIBLE), 1.0);
        assert_eq!(ui.string(dr_card, value).unwrap(), "--");
        assert_eq!(ui.number(tile(&ui, "ConditionInfo"), t::USER0 + 6), 0.0);
        assert_eq!(ui.number(tile(&ui, "DPSInfo"), t::VISIBLE), 0.0);
    }

    /// Only Apparel and Ammo ask for help (`00780140`).
    #[test]
    fn the_tabs_help() {
        let asked: Vec<_> = (FIRST_TAB_ID..=0x1c).map(tab_tutorial).collect();
        let ask = |id, delay| {
            Some(Action::Tutorial {
                id,
                menu: 1002,
                delay,
            })
        };
        assert_eq!(asked, [None, ask(0x23, 512), None, None, ask(0x24, 500)]);
    }

    #[test]
    fn the_pads_x_drops_only_with_a_pad_and_an_item() {
        let globals =
            r#"<rect name="Strings"><_Has360Controller>&false;</_Has360Controller></rect>"#;
        let mut read = |p: &str| match p {
            crate::pipboy::ITEMS_FILE => Some(MENU.as_bytes().to_vec()),
            crate::game::GLOBALS_FILE => Some(globals.as_bytes().to_vec()),
            _ => None,
        };
        let mut ui = crate::game::new_ui(
            &mut read,
            &|_: &str, _: &str| None,
            std::collections::HashMap::new(),
            1920,
            1080,
        );
        let mut m = ItemsMenu::load(&mut ui, &mut read).unwrap();
        ui.set_number(m.menu, t::VISIBLE, 1.0);
        m.fill(&mut ui, &input());
        ui.refresh();
        let drop = by_id(&ui, m.menu, 7).unwrap();
        assert_eq!(ui.number(drop, t::TARGET), 1.0);
        // No pad: the button is hidden and X does nothing.
        assert_eq!(m.pad_button(&mut ui, Key::ButtonX), PadPress::Nothing);
        crate::game::set_pad(&mut ui, true);
        ui.refresh();
        assert_eq!(m.pad_button(&mut ui, Key::ButtonX), PadPress::Click(7));
        assert_eq!(m.pad_button(&mut ui, Key::ButtonY), PadPress::Nothing);
        // Nothing chosen: shown but not clickable.
        m.show_card(&mut ui, None);
        ui.refresh();
        assert_eq!(m.pad_button(&mut ui, Key::ButtonX), PadPress::Refused);
    }
}
