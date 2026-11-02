//! Trading (`menus\barter_menu.xml`, class `BarterMenu` 1053, vtable
//! `010706ec` in FalloutNV.exe): the player's things on the left, the
//! vendor's on the right; picking a thing offers it (it moves across,
//! marked, and the running total changes); Accept settles the trade.
//! Read from the code (`0072d250` and the functions after it):
//!
//! * opening (`0072d250(vendor, adjustment)`): list boxes on ids 6 and 11
//!   (the code asks for `CM_list_template`, which the file lacks, so the
//!   template lookup ends on the file's only one, `BM_list_template`);
//!   `BM_ItemsTitle` `sInventoryItems`, `BM_CapsLabel` `sInventoryCaps`,
//!   Accept (13) `sAccept`, Exit (19) `sExit`, the card's titles; the
//!   price adjustment kept within -100..100; the total 0; `UIMenuMode`;
//!   filled. (No title is given to the vendor's side until its filter
//!   steps.)
//! * filling (`0072dc30`): the player's things (`0072e570` leaves out the
//!   unnamed, things marked not playable, quest items, caps and keys); the
//!   vendor's goods (their own and their merchant container's; `0072e6c0`
//!   also leaves out what their services don't cover); then the offers as
//!   marked lines (`_IsBarterSelected`) on the other side: what the player
//!   is buying on the left, what they're selling on the right. Each line
//!   (`0072f210`): its count less what of it is offered (`_NumBartered`),
//!   "name (count)" above 1, `_IsEquipped` for worn things not offered,
//!   `_Value` the price of one: whole, tenths below 1, "--" for nothing;
//!   filtered (`007304b0`: the side's filter, and lines with nothing left
//!   to offer); sorted (`0072f070`: offers first, then by text); the caps
//!   lines "`sInventoryCaps`   n" (`0072db60`).
//! * a price (`world::barter::price`): the player's things at the selling
//!   price, the vendor's at the buying one.
//! * picking a line (`0072d770` case 0x17): more than
//!   `iInventoryAskQuantityAt` left asks how many; `0072f6f0(n)`: total +=
//!   price × n, plus on the player's side, minus on the vendor's; the
//!   item's sound (picked up from the vendor's side, put down from the
//!   player's); an offered line taken back (`0072fc30`), else offered
//!   (`0072faa0`); the item's lines made again; the caps flow (`BM_CapsFlow`)
//!   shows |min(floor(total), vendor's caps)| with `_ArrowDirection` 1
//!   (paying) or 2 (being paid) while anything is offered; Accept can be
//!   used when the player is paid or can pay, and the flow is dim (alpha
//!   64) when the vendor can't pay it all or the player can't.
//! * Accept (13, `0072fd10`): the trade settles (`world::barter::accept`),
//!   the offers clear, the total 0, the flow hidden, everything filled
//!   again. Exit (19): with offers, "Cancel transaction?" (`sCancelBarter`,
//!   `sYes`, `sNo`) and Yes closes; else it closes (`0072d6d0`).
//! * the focus rectangles (ids 0, 1) switch sides (`0072e370`,
//!   `_IsContainerListSelected` on the menu, which only enables that side's
//!   list); the pointer on a line shows its picture (`BM_ItemIcon`); the
//!   filters, titles, wheel and Left/Right as the container menu's.
//!
//! Not here: the item card (`BM_ItemData`, the Pip-Boy's `00707e30`), the
//! vendor's owned items' handling (`0072eac0`), the dialogue topic said
//! after a trade (`0061a2d0(5, 4)` handed to the dialogue on closing).

use crate::list::ListBox;
use crate::menu::{self, MenuCode};
use crate::menus::container::{self, Item, Side};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\barter_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1053;
/// The template the code asks for (the file has `BM_list_template`).
pub const LIST_TEMPLATE: &str = "CM_list_template";
/// A line's id (`BM_list_template_container`).
pub const LINE_ID: i32 = 23;
/// How many tile ids the menu keeps (`0072d040`: 0 to 21).
pub const TILE_COUNT: usize = 22;

/// One thing in the trade with its two prices (`world::barter::price`
/// for its holder: what the vendor pays for one, what the player pays).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Goods {
    pub item: Item,
    /// The price of one sold (-1: "--").
    pub sell: f32,
    /// The price of one bought.
    pub buy: f32,
}

/// What a line in a list stands for.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Line {
    /// Whose thing it is.
    owner: Side,
    /// Its form.
    form: u32,
    /// An offer shown on the other side (`_IsBarterSelected`).
    bartered: bool,
}

/// What the menu asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// Ask how many of a line (`007aba00`).
    AskQuantity { most: i32 },
    /// An item's sound: picked up (`up`) or put down (`008aded0`).
    Sound { form: u32, up: bool },
    /// Settle the trade: bought (forms and counts) and sold, and the total
    /// (positive: the vendor pays).
    Accept {
        buys: Vec<(u32, i32)>,
        sells: Vec<(u32, i32)>,
        total: f32,
    },
    /// "Cancel transaction?" with Yes and No.
    ConfirmExit,
    /// Close (`traded`: a trade was accepted).
    Close { traded: bool },
}

/// The barter menu.
#[derive(Debug)]
pub struct BarterMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; TILE_COUNT],
    /// The player's list (`+0xa8`, id 6) and the vendor's (`+0xd8`, id 11).
    pub lists: [ListBox; 2],
    lines: [Vec<Line>; 2],
    /// What each side holds.
    pub goods: [Vec<Goods>; 2],
    /// Offers by the owner's side: the player's things being sold
    /// (`+0x114`), the vendor's being bought (`+0x10c`); form and count.
    pub offers: [Vec<(u32, i32)>; 2],
    /// The running total (`+0x84`): positive, the vendor pays.
    pub total: f32,
    /// The player's caps (`+0x8c`) and the vendor's (`+0x90`).
    pub caps: (i32, i32),
    pub filters: [i32; 2],
    pub current: Option<Side>,
    /// The line under the pointer (`011d8fa8`).
    selected: Option<(Side, usize)>,
    pub english: bool,
    pub ask_quantity_at: i32,
    filter_sound: i32,
    /// A trade was accepted (`+0x11c`).
    pub traded: bool,
    pub requests: Vec<Request>,
    pub sounds: Vec<String>,
    pub closed: bool,
    /// The vendor's name, for their side's title at filter 0.
    pub vendor_name: String,
}

fn side_index(side: Side) -> usize {
    match side {
        Side::Player => 0,
        Side::Container => 1,
    }
}

/// The price shown (`0072f210`): whole, tenths between 0 and 1, "--"
/// below 0.
pub fn price_text(p: f32) -> String {
    if p < 0.0 {
        "--".to_string()
    } else if p <= 0.0 || p >= 1.0 {
        format!("{p:.0}")
    } else {
        format!("{p:.1}")
    }
}

/// Whether the filter hides a line (`007304b0`): the side's filter by form
/// type, and lines with nothing left to offer.
fn hidden(item: &Item, filter: i32, left: i32) -> bool {
    use container::form_type::*;
    let ty = item.form_type;
    let out = match filter {
        1 => ty != WEAP,
        2 => !(ty == ARMO || ty == CLOT),
        3 => !(ty == INGR || ty == ALCH || ty == BOOK),
        4 => [WEAP, ARMO, CLOT, INGR, ALCH, AMMO, BOOK, CHIP, CCRD, CMNY].contains(&ty),
        5 => ty != AMMO,
        _ => false,
    };
    out || left <= 0
}

impl BarterMenu {
    pub fn new(menu: TileId) -> BarterMenu {
        BarterMenu {
            menu,
            tiles: [None; TILE_COUNT],
            lists: [ListBox::default(), ListBox::default()],
            lines: [Vec::new(), Vec::new()],
            goods: [Vec::new(), Vec::new()],
            offers: [Vec::new(), Vec::new()],
            total: 0.0,
            caps: (0, 0),
            filters: [0, 0],
            current: None,
            selected: None,
            english: true,
            ask_quantity_at: container::ASK_QUANTITY_AT,
            filter_sound: 3,
            traded: false,
            requests: Vec::new(),
            sounds: Vec::new(),
            closed: false,
            vendor_name: String::new(),
        }
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles.get(id).copied().flatten()
    }

    fn custom(ui: &mut Ui, name: &str) -> i32 {
        ui.names.lookup_or_add(name).unwrap_or(0)
    }

    /// Opens it (`0072d250`); false when the file lacks one of its tiles.
    pub fn open(
        &mut self,
        ui: &mut Ui,
        player: Vec<Goods>,
        vendor: Vec<Goods>,
        caps: (i32, i32),
    ) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        for (side, id) in [(Side::Player, 6), (Side::Container, 11)] {
            if let Some(list) = self.tile(id) {
                self.lists[side_index(side)] = ListBox::new(ui, list, LIST_TEMPLATE);
            }
        }
        let text = |ui: &Ui, name: &str| ui.setting_text(name).unwrap_or_default();
        if let Some(title) = self.tile(3) {
            let s = text(ui, "sInventoryItems");
            ui.set_text(title, t::STRING, &s);
        }
        if let Some(label) = ui.find(self.menu, "BM_CapsLabel") {
            let s = text(ui, "sInventoryCaps");
            ui.set_text(label, t::STRING, &s);
        }
        for (id, setting) in [(13, "sAccept"), (19, "sExit")] {
            if let Some(tile) = self.tile(id) {
                let s = text(ui, setting);
                ui.set_text(tile, t::STRING, &s);
            }
        }
        let title = Self::custom(ui, "_Title");
        for (name, setting) in [
            ("WeightInfo", "sInventoryWeightUpper"),
            ("ValueInfo", "sInventoryValue"),
        ] {
            if let Some(tile) = ui.find(self.menu, name) {
                let s = text(ui, setting);
                ui.set_string(tile, title, &s);
            }
        }
        for (id, setting) in [
            (14, "sInventoryDamage"),
            (15, "sInventoryDamagePerSecond"),
            (16, "sInventoryStrReq"),
            (17, "sInventoryDamageResistance"),
            (18, "sInventoryDamageThreshold"),
        ] {
            if let Some(tile) = self.tile(id) {
                let s = text(ui, setting);
                ui.set_string(tile, title, &s);
            }
        }
        if let Some(s) = menu::menu_sound(0x24) {
            self.sounds.push(s.to_string());
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        self.goods = [player, vendor];
        self.caps = caps;
        self.fill(ui, None);
        true
    }

    fn goods_of(&self, side: Side, form: u32) -> Option<&Goods> {
        self.goods[side_index(side)]
            .iter()
            .find(|g| g.item.form == form)
    }

    fn offered(&self, side: Side, form: u32) -> i32 {
        self.offers[side_index(side)]
            .iter()
            .find(|(f, _)| *f == form)
            .map_or(0, |(_, n)| *n)
    }

    /// Whether a line's thing is listed (`0072e570` for the player's;
    /// the vendor's were sorted by the caller with their services).
    fn listed(item: &Item, side: Side) -> bool {
        use container::form_type::KEYM;
        let named = !item.name.is_empty() && item.playable && item.count > 0;
        match side {
            Side::Player => named && !item.quest_item && !item.is_caps && item.form_type != KEYM,
            Side::Container => named && !item.is_caps,
        }
    }

    /// The list a line goes in: its owner's, or the other for an offer.
    fn list_of(line: &Line) -> Side {
        if line.bartered {
            line.owner.other()
        } else {
            line.owner
        }
    }

    fn add_line(&mut self, ui: &mut Ui, line: Line) {
        let list = side_index(Self::list_of(&line));
        let Some(goods) = self.goods_of(line.owner, line.form) else {
            return;
        };
        let name = goods.item.name.clone();
        let index = self.lines[list].len();
        self.lines[list].push(line);
        if let Some(tile) = self.lists[list].add(ui, self.menu, index as i32, Some(&name)) {
            if line.bartered {
                let selected = Self::custom(ui, "_IsBarterSelected");
                ui.set_number(tile, selected, 1.0);
            }
        }
    }

    /// Fills the lists (`0072dc30`): everything (`None`) or one thing's
    /// lines again; then labels, filters, order, the caps lines.
    pub fn fill(&mut self, ui: &mut Ui, form: Option<u32>) {
        match form {
            None => {
                for s in 0..2 {
                    self.lists[s].clear(ui);
                    self.lines[s].clear();
                }
                for side in [Side::Player, Side::Container] {
                    let forms: Vec<u32> = self.goods[side_index(side)]
                        .iter()
                        .filter(|g| Self::listed(&g.item, side))
                        .map(|g| g.item.form)
                        .collect();
                    for f in forms {
                        self.add_line(
                            ui,
                            Line {
                                owner: side,
                                form: f,
                                bartered: false,
                            },
                        );
                    }
                }
            }
            Some(f) => {
                for s in 0..2 {
                    let tiles: Vec<_> = self.lists[s]
                        .items
                        .iter()
                        .filter(|l| {
                            self.lines[s]
                                .get(l.value as usize)
                                .is_some_and(|x| x.form == f)
                        })
                        .map(|l| l.tile)
                        .collect();
                    for tile in tiles {
                        self.lists[s].remove(ui, tile);
                    }
                }
                for side in [Side::Player, Side::Container] {
                    if self
                        .goods_of(side, f)
                        .is_some_and(|g| Self::listed(&g.item, side))
                    {
                        self.add_line(
                            ui,
                            Line {
                                owner: side,
                                form: f,
                                bartered: false,
                            },
                        );
                    }
                }
            }
        }
        // The offers, on the other side: what's bought first (`+0x10c`),
        // then what's sold (`+0x114`).
        for owner in [Side::Container, Side::Player] {
            let offers = self.offers[side_index(owner)].clone();
            for (f, _) in offers {
                if form.map_or(true, |w| w == f) {
                    self.add_line(
                        ui,
                        Line {
                            owner,
                            form: f,
                            bartered: true,
                        },
                    );
                }
            }
        }
        self.finish(ui);
    }

    /// Labels, filters and order for both lists, and the caps lines.
    fn finish(&mut self, ui: &mut Ui) {
        for list_side in [Side::Player, Side::Container] {
            let s = side_index(list_side);
            let lines = self.lists[s].shown_items(ui, 0, i32::MAX);
            for l in lines {
                if let Some(line) = self.lines[s].get(l.value as usize).copied() {
                    self.label(ui, l.tile, line);
                }
            }
            self.refilter(ui, list_side);
            self.lists[s].sort(ui, &|ui, a, b| {
                let key = ui.names.lookup("_IsBarterSelected");
                let (ba, bb) = match key {
                    Some(k) => (ui.number(a, k) != 0.0, ui.number(b, k) != 0.0),
                    None => (false, false),
                };
                if ba != bb {
                    return ba;
                }
                let sa = ui.string(a, t::STRING).unwrap_or_default();
                let sb = ui.string(b, t::STRING).unwrap_or_default();
                sa.as_bytes() < sb.as_bytes()
            });
        }
        self.caps_lines(ui);
        ui.refresh();
    }

    fn refilter(&mut self, ui: &mut Ui, list_side: Side) {
        let s = side_index(list_side);
        let filter = self.filters[s];
        let lines = self.lines[s].clone();
        let left: Vec<i32> = lines.iter().map(|l| self.left_of(l)).collect();
        let items: Vec<Option<Item>> = lines
            .iter()
            .map(|l| self.goods_of(l.owner, l.form).map(|g| g.item.clone()))
            .collect();
        self.lists[s].filter(ui, &|v| match items.get(v as usize) {
            Some(Some(item)) => hidden(item, filter, left[v as usize]),
            _ => true,
        });
    }

    /// How many a line has to offer: an offer's count, else the thing's
    /// count less what of it is offered.
    fn left_of(&self, line: &Line) -> i32 {
        if line.bartered {
            self.offered(line.owner, line.form)
        } else {
            self.goods_of(line.owner, line.form)
                .map_or(0, |g| g.item.count - self.offered(line.owner, line.form))
        }
    }

    /// The price of one of a line's thing: the player's sell, the vendor's
    /// buy (`0072ed00`).
    fn price_of(&self, line: &Line) -> f32 {
        match self.goods_of(line.owner, line.form) {
            Some(g) => match line.owner {
                Side::Player => g.sell,
                Side::Container => g.buy,
            },
            None => -1.0,
        }
    }

    /// A line's label (`0072f210`).
    fn label(&mut self, ui: &mut Ui, tile: TileId, line: Line) {
        let Some(goods) = self.goods_of(line.owner, line.form).cloned() else {
            return;
        };
        let count = self.left_of(&line);
        if !line.bartered {
            let n = Self::custom(ui, "_NumBartered");
            ui.set_number(tile, n, self.offered(line.owner, line.form) as f32);
        }
        if count > 1 {
            let s = format!("{} ({count})", goods.item.name);
            ui.set_string(tile, t::STRING, &s);
        }
        if goods.item.equipped && !line.bartered {
            let e = Self::custom(ui, "_IsEquipped");
            ui.set_number(tile, e, 1.0);
        }
        let value = Self::custom(ui, "_Value");
        let p = self.price_of(&line);
        ui.set_string(tile, value, &price_text(p));
        ui.set_number(tile, t::HEIGHT, 0.0);
    }

    /// "`sInventoryCaps`   n" for each side (`0072db60`).
    fn caps_lines(&mut self, ui: &mut Ui) {
        let caps = ui.setting_text("sInventoryCaps").unwrap_or_default();
        for (id, n) in [(5, self.caps.0), (10, self.caps.1)] {
            if let Some(tile) = self.tile(id) {
                ui.set_text(tile, t::STRING, &format!("{caps}   {n}"));
            }
        }
    }

    /// The caps flow and Accept (`0072f6f0`'s end).
    fn caps_flow(&mut self, ui: &mut Ui) {
        let (Some(flow), Some(accept)) = (self.tile(12), self.tile(13)) else {
            return;
        };
        let any = self.offers.iter().any(|o| !o.is_empty());
        if !any {
            ui.set_number(accept, t::TARGET, 0.0);
            ui.set_number(flow, t::VISIBLE, 0.0);
            ui.refresh();
            return;
        }
        let whole = self.total.floor() as i32;
        let direction = Self::custom(ui, "_ArrowDirection");
        ui.set_number(
            self.menu,
            direction,
            if self.total <= 0.0 { 1.0 } else { 2.0 },
        );
        let value = Self::custom(ui, "_Value");
        let shown = whole.min(self.caps.1).abs();
        ui.set_string(flow, value, &shown.to_string());
        ui.set_number(flow, t::VISIBLE, 1.0);
        let pays_or_can_pay = self.total > 0.0 || -self.total <= self.caps.0 as f32;
        if pays_or_can_pay {
            ui.set_number(accept, t::TARGET, 1.0);
            let all = self.total <= 0.0 || self.total <= self.caps.1 as f32;
            ui.set_number(flow, t::ALPHA, if all { 255.0 } else { 64.0 });
        } else {
            ui.set_number(accept, t::TARGET, 0.0);
            ui.set_number(flow, t::ALPHA, 64.0);
        }
        ui.refresh();
    }

    /// Offers or takes back `n` of the line under the pointer
    /// (`0072f6f0`).
    pub fn quantity_chosen(&mut self, ui: &mut Ui, n: i32) {
        if n <= 0 {
            return;
        }
        let Some((list_side, index)) = self.selected else {
            return;
        };
        let Some(line) = self.lines[side_index(list_side)].get(index).copied() else {
            return;
        };
        let p = self.price_of(&line);
        let dir = if self.current == Some(Side::Player) {
            1.0
        } else {
            -1.0
        };
        self.total += p * dir * n as f32;
        self.requests.push(Request::Sound {
            form: line.form,
            up: self.current == Some(Side::Container),
        });
        let offers = &mut self.offers[side_index(line.owner)];
        match offers.iter().position(|(f, _)| *f == line.form) {
            Some(at) if line.bartered => {
                offers[at].1 -= n;
                if offers[at].1 <= 0 {
                    offers.remove(at);
                }
            }
            Some(at) => offers[at].1 += n,
            None if !line.bartered => offers.push((line.form, n)),
            None => {}
        }
        self.selected = None;
        self.fill(ui, Some(line.form));
        self.caps_flow(ui);
    }

    /// After a trade was settled (`0072fd10`'s end): new holdings, the
    /// offers cleared, the total 0, the flow hidden, everything filled.
    pub fn settled(
        &mut self,
        ui: &mut Ui,
        player: Vec<Goods>,
        vendor: Vec<Goods>,
        caps: (i32, i32),
    ) {
        self.goods = [player, vendor];
        self.caps = caps;
        self.offers = [Vec::new(), Vec::new()];
        self.total = 0.0;
        self.traded = true;
        self.selected = None;
        if let Some(accept) = self.tile(13) {
            ui.set_number(accept, t::TARGET, 0.0);
        }
        if let Some(flow) = self.tile(12) {
            ui.set_number(flow, t::VISIBLE, 0.0);
        }
        self.fill(ui, None);
    }

    /// Closes it (`0072d6d0`).
    pub fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        self.requests.push(Request::Close {
            traded: self.traded,
        });
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }

    /// A side's title for its filter (`007301f0`): the vendor's name for
    /// the vendor's filter 0.
    fn title_for_filter(&mut self, ui: &mut Ui, side: Side) {
        let filter = self.filters[side_index(side)];
        let Some(tile) = self.tile(if side == Side::Player { 3 } else { 8 }) else {
            return;
        };
        let setting = match filter {
            0 if side == Side::Container => {
                let name = self.vendor_name.clone();
                ui.set_text(tile, t::STRING, &name);
                return;
            }
            0 => "sInventoryItems",
            1 => "sInventoryWeapons",
            2 => "sInventoryApparel",
            3 => "sInventoryAid",
            4 => "sInventoryMisc",
            5 => "sInventoryAmmo",
            _ => return,
        };
        let mut s = ui.setting_text(setting).unwrap_or_default();
        if self.english {
            s = s.to_ascii_uppercase();
        }
        ui.set_text(tile, t::STRING, &s);
    }

    /// Steps a side's filter (`0072d770` cases 2–4, 7–9), on past filters
    /// with nothing to show, until back at 0.
    fn step_filter(&mut self, ui: &mut Ui, side: Side, dir: i32) {
        if let Some(s) = menu::menu_sound(self.filter_sound) {
            self.sounds.push(s.to_string());
        }
        let s = side_index(side);
        loop {
            let mut f = self.filters[s] + dir;
            if f < 0 {
                f = 5;
            } else if f > 5 {
                f = 0;
            }
            self.filters[s] = f;
            self.title_for_filter(ui, side);
            self.refilter(ui, side);
            if self.lists[s].shown_count(ui) != 0 || self.filters[s] == 0 {
                break;
            }
        }
        ui.refresh();
    }

    /// The pointer onto a side's area (`0072e370` cases 0, 1).
    fn switch_to(&mut self, ui: &mut Ui, side: Side) {
        self.unmouseover(ui, LINE_ID, self.menu);
        self.lists[side_index(side.other())].select(ui, None);
        let flag = Self::custom(ui, "_IsContainerListSelected");
        ui.set_number(
            self.menu,
            flag,
            if side == Side::Player { 0.0 } else { 1.0 },
        );
        self.current = Some(side);
        ui.refresh();
    }
}

impl MenuCode for BarterMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..TILE_COUNT as i32).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `0072d770`.
    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, _now: f64) {
        match id {
            2..=4 => self.step_filter(ui, Side::Player, if id == 2 { -1 } else { 1 }),
            7..=9 => self.step_filter(ui, Side::Container, if id == 7 { -1 } else { 1 }),
            13 => {
                let buys = self.offers[side_index(Side::Container)].clone();
                let sells = self.offers[side_index(Side::Player)].clone();
                self.requests.push(Request::Accept {
                    buys,
                    sells,
                    total: self.total,
                });
            }
            19 => {
                if self.offers.iter().all(Vec::is_empty) {
                    self.close(ui);
                } else {
                    self.requests.push(Request::ConfirmExit);
                }
            }
            LINE_ID => {
                if tile.is_none() {
                    return;
                }
                let Some((list_side, index)) = self.selected else {
                    return;
                };
                let Some(line) = self.lines[side_index(list_side)].get(index).copied() else {
                    return;
                };
                let left = self.left_of(&line);
                if left > self.ask_quantity_at {
                    if let Some(c) = self.current {
                        self.lists[side_index(c)].select(ui, None);
                    }
                    self.requests.push(Request::AskQuantity { most: left });
                } else {
                    self.quantity_chosen(ui, 1);
                }
            }
            _ => {}
        }
    }

    /// `0072e370`.
    fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        match id {
            0 => self.switch_to(ui, Side::Player),
            1 => self.switch_to(ui, Side::Container),
            LINE_ID => {
                if self.current.is_none() {
                    let parent = ui.tiles[tile].parent;
                    let side = if parent == self.tile(11) {
                        Side::Container
                    } else {
                        Side::Player
                    };
                    self.switch_to(ui, side);
                }
                let Some(side) = self.current else {
                    return;
                };
                let list = &self.lists[side_index(side)];
                self.selected = list
                    .selected
                    .and_then(|s| list.value_of(s))
                    .map(|v| (side, v as usize));
                let Some((s, index)) = self.selected else {
                    return;
                };
                let Some(line) = self.lines[side_index(s)].get(index).copied() else {
                    return;
                };
                let icon_path = self
                    .goods_of(line.owner, line.form)
                    .map(|g| g.item.icon.clone())
                    .unwrap_or_default();
                if let Some(icon) = self.tile(20) {
                    let path = if icon_path.is_empty() {
                        ui.setting_text("sMissingImage").unwrap_or_default()
                    } else {
                        icon_path
                    };
                    ui.set_string(icon, t::FILENAME, &path);
                    ui.set_number(icon, t::VISIBLE, 1.0);
                }
                ui.refresh();
            }
            _ => {}
        }
    }

    /// `0072e530`: the card and picture hidden.
    fn unmouseover(&mut self, ui: &mut Ui, _id: i32, _tile: TileId) {
        for id in [21, 20] {
            if let Some(tile) = self.tile(id) {
                ui.set_number(tile, t::VISIBLE, 0.0);
            }
        }
        ui.refresh();
    }

    /// `0072f4b0`: Left and Right step the current side's filter.
    fn special_key(&mut self, ui: &mut Ui, code: i32, now: f64) -> bool {
        if code != menu::special::LEFT && code != menu::special::RIGHT {
            return false;
        }
        let left = code == menu::special::LEFT;
        let side = self.current.unwrap_or(Side::Container);
        let id = match (side, left) {
            (Side::Player, true) => 2,
            (Side::Player, false) => 4,
            (Side::Container, true) => 7,
            (Side::Container, false) => 9,
        };
        self.click(ui, id, None, now);
        self.lists[side_index(side)].select(ui, None);
        true
    }

    /// `0072f680`: the wheel over a title.
    fn wheel(&mut self, ui: &mut Ui, id: i32, _tile: TileId, delta: i32) {
        let (back, on) = match id {
            3 => (2, 4),
            8 => (7, 9),
            _ => return,
        };
        self.click(ui, if delta < 0 { back } else { on }, None, 0.0);
    }

    fn lists(&mut self) -> Vec<&mut ListBox> {
        self.lists.iter_mut().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    fn goods(form: u32, name: &str, count: i32, ty: u8, sell: f32, buy: f32) -> Goods {
        Goods {
            item: Item {
                form,
                name: name.into(),
                count,
                form_type: ty,
                playable: true,
                is_caps: form == 0xF,
                ..Item::default()
            },
            sell,
            buy,
        }
    }

    fn opened(player: Vec<Goods>, vendor: Vec<Goods>, caps: (i32, i32)) -> (Ui, BarterMenu) {
        let mut ui = test_support::ui();
        let mut m = BarterMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::barter_menu(), &mut m);
        assert!(m.open(&mut ui, player, vendor, caps));
        (ui, m)
    }

    fn texts(ui: &mut Ui, m: &BarterMenu, side: Side) -> Vec<String> {
        m.lists[side_index(side)]
            .shown_items(ui, 0, i32::MAX)
            .iter()
            .map(|l| ui.string(l.tile, t::STRING).unwrap_or_default())
            .collect()
    }

    fn line_tile(ui: &mut Ui, m: &BarterMenu, side: Side, text: &str) -> TileId {
        m.lists[side_index(side)]
            .shown_items(ui, 0, i32::MAX)
            .into_iter()
            .find(|l| ui.string(l.tile, t::STRING).as_deref() == Some(text))
            .unwrap()
            .tile
    }

    /// `0072dc30`, `0072e570`, `0072f210`: the player's caps and quest
    /// items aren't listed; each line's price of one; the caps lines.
    #[test]
    fn filling_with_prices() {
        let mut quest = goods(3, "Locket", 1, 0x1F, 5.0, 8.0);
        quest.item.quest_item = true;
        let player = vec![
            goods(0xF, "Bottle Cap", 40, 0x1F, 1.0, 1.0),
            goods(2, "Tin Can", 3, 0x1F, 0.5, 1.0),
            quest,
        ];
        let vendor = vec![goods(10, "Beer", 1, 0x2F, 1.0, 3.0)];
        let (mut ui, m) = opened(player, vendor, (40, 160));
        assert_eq!(texts(&mut ui, &m, Side::Player), ["Tin Can (3)"]);
        assert_eq!(texts(&mut ui, &m, Side::Container), ["Beer"]);
        let can = line_tile(&mut ui, &m, Side::Player, "Tin Can (3)");
        let value = ui.names.lookup("_Value").unwrap();
        assert_eq!(ui.string(can, value).as_deref(), Some("0.5"));
        let beer = line_tile(&mut ui, &m, Side::Container, "Beer");
        assert_eq!(ui.string(beer, value).as_deref(), Some("3"));
        assert_eq!(
            ui.string(m.tiles[5].unwrap(), t::STRING).as_deref(),
            Some("Caps   40")
        );
        assert_eq!(
            ui.string(m.tiles[10].unwrap(), t::STRING).as_deref(),
            Some("Caps   160")
        );
        assert_eq!(
            ui.string(m.tiles[13].unwrap(), t::STRING).as_deref(),
            Some("Accept")
        );
        assert_eq!(price_text(-1.0), "--");
        assert_eq!(price_text(0.0), "0");
    }

    /// `0072f6f0`: offering moves a line across, marked; the total, the
    /// caps flow and Accept follow; taking it back undoes it.
    #[test]
    fn offering_and_taking_back() {
        let player = vec![goods(2, "Tin Can", 2, 0x1F, 0.5, 1.0)];
        let vendor = vec![
            goods(10, "Beer", 1, 0x2F, 1.0, 3.0),
            goods(11, "Rifle", 1, 0x28, 50.0, 120.0),
        ];
        let (mut ui, mut m) = opened(player, vendor, (5, 160));
        let mut i = crate::menu::Interface::default();
        let menu = m.menu;
        // The player's side, then a tin can.
        m.mouseover(&mut ui, 0, menu);
        let can = line_tile(&mut ui, &m, Side::Player, "Tin Can (2)");
        i.choose(&mut ui, &mut m, Some(can), false);
        i.click(&mut ui, menu, &mut m, can, 0.0);
        assert_eq!(m.offers[0], [(2, 1)]);
        assert!((m.total - 0.5).abs() < 1e-6);
        assert_eq!(texts(&mut ui, &m, Side::Player), ["Tin Can"]);
        assert_eq!(
            texts(&mut ui, &m, Side::Container),
            ["Tin Can", "Beer", "Rifle"]
        );
        let marked = ui.names.lookup("_IsBarterSelected").unwrap();
        let offered = line_tile(&mut ui, &m, Side::Container, "Tin Can");
        assert_eq!(ui.number(offered, marked), 1.0);
        assert!(m.requests.contains(&Request::Sound { form: 2, up: false }));
        let dir = ui.names.lookup("_ArrowDirection").unwrap();
        assert_eq!(ui.number(menu, dir), 2.0);
        assert_eq!(ui.number(m.tiles[12].unwrap(), t::VISIBLE), 1.0);
        assert_eq!(ui.number(m.tiles[13].unwrap(), t::TARGET), 1.0);
        // The vendor's side: the rifle costs 120, more than the player has.
        m.mouseover(&mut ui, 1, menu);
        let rifle = line_tile(&mut ui, &m, Side::Container, "Rifle");
        i.choose(&mut ui, &mut m, Some(rifle), false);
        i.click(&mut ui, menu, &mut m, rifle, 0.0);
        assert_eq!(m.offers[1], [(11, 1)]);
        assert!((m.total - (0.5 - 120.0)).abs() < 1e-4);
        assert_eq!(ui.number(menu, dir), 1.0);
        let value = ui.names.lookup("_Value").unwrap();
        assert_eq!(
            ui.string(m.tiles[12].unwrap(), value).as_deref(),
            Some("120")
        );
        assert_eq!(ui.number(m.tiles[13].unwrap(), t::TARGET), 0.0);
        assert_eq!(ui.number(m.tiles[12].unwrap(), t::ALPHA), 64.0);
        // The rifle shows on the player's side, offered lines first.
        assert_eq!(texts(&mut ui, &m, Side::Player), ["Rifle", "Tin Can"]);
        // Taking the rifle back.
        m.mouseover(&mut ui, 0, menu);
        let back = line_tile(&mut ui, &m, Side::Player, "Rifle");
        i.choose(&mut ui, &mut m, Some(back), false);
        i.click(&mut ui, menu, &mut m, back, 0.0);
        assert!(m.offers[1].is_empty());
        assert!((m.total - 0.5).abs() < 1e-4);
        // Exit with an offer asks first; Accept hands the trade over.
        m.click(&mut ui, 19, None, 0.0);
        assert_eq!(m.requests.last(), Some(&Request::ConfirmExit));
        m.click(&mut ui, 13, None, 0.0);
        assert_eq!(
            m.requests.last(),
            Some(&Request::Accept {
                buys: vec![],
                sells: vec![(2, 1)],
                total: 0.5
            })
        );
        m.settled(
            &mut ui,
            vec![goods(2, "Tin Can", 1, 0x1F, 0.5, 1.0)],
            vec![],
            (5, 160),
        );
        assert!(m.offers.iter().all(Vec::is_empty));
        assert_eq!(ui.number(m.tiles[12].unwrap(), t::VISIBLE), 0.0);
        m.click(&mut ui, 19, None, 0.0);
        assert_eq!(m.requests.last(), Some(&Request::Close { traded: true }));
    }

    /// More than five left asks how many.
    #[test]
    fn many_ask_how_many() {
        let player = vec![goods(2, "Tin Can", 9, 0x1F, 0.5, 1.0)];
        let (mut ui, mut m) = opened(player, vec![], (0, 0));
        let mut i = crate::menu::Interface::default();
        let menu = m.menu;
        m.mouseover(&mut ui, 0, menu);
        let can = line_tile(&mut ui, &m, Side::Player, "Tin Can (9)");
        i.choose(&mut ui, &mut m, Some(can), false);
        i.click(&mut ui, menu, &mut m, can, 0.0);
        assert_eq!(m.requests.last(), Some(&Request::AskQuantity { most: 9 }));
        // The line was let go while asking: the answer still finds it.
        m.selected = Some((Side::Player, 0));
        m.quantity_chosen(&mut ui, 4);
        assert_eq!(m.offers[0], [(2, 4)]);
        assert_eq!(texts(&mut ui, &m, Side::Player), ["Tin Can (5)"]);
        assert_eq!(texts(&mut ui, &m, Side::Container), ["Tin Can (4)"]);
    }
}
