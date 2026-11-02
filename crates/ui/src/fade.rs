//! Menus fading in and out, as the game's interface manager runs it
//! (FalloutNV.exe 1.4.0.525; names from the Xbox 360 prototype's symbols,
//! (Xbox PDB)).
//!
//! * Every menu has a fade state (`Menu::xFadeState`, `+0x24`, the enum
//!   `Interface::FADE_STATE`): shown 1, fading out 2, hidden 4 (a new
//!   menu's, `Menu::Menu`), fading in 8 ([`State`]).
//! * Showing a menu (`Menu::PrepForVisibility`, `00a1dc20`): its tile
//!   hidden (`visible` 0) and, unless asked for at once, `StartFadeIn`
//!   (`00a1db20`): fading in over the menu tile's `menufade` (0.25 unless
//!   its file says otherwise: the menu tile's constructor `00a1ef30`), or
//!   `explorefade` when that is 0 (0.25 with no tile, `0101622c`).
//! * Closing (`Menu::StartFadeOut`, `00a1d910`, which almost every menu's
//!   close calls): only a menu whose tile is shown fades out, over the same
//!   time; it leaves the interface's menu stack at once (a menu that marks
//!   itself to leave it, `6002`), so the keys go to the menu below.
//!   `InstantFadeOut` (`00a1d9e0`) hides it and ends the fade at the next
//!   frame (the message box and "how many?" close this way while a
//!   rendered menu, the Pip-Boy, is up: `007a8df0`, `007abdf0`).
//! * The fades themselves are a list on the interface manager (`+0x164`):
//!   a menu, the seconds gone and the seconds it takes (`007164c0` adds one,
//!   replacing the menu's earlier one). Each frame before the menus run,
//!   and again every time a fade starts, every fade moves on by the frame's
//!   seconds and the finished ones leave the list (`00716320`). How far a
//!   menu's fade is (`00716660`): 1 when it has none, -1 when it takes no
//!   time, else the seconds gone over the seconds it takes held to 0..1.
//! * After the menus run (`00711ea0`, Xbox `InterfaceManager::
//!   PostIdleStuff`): a menu fading in shows its tile and is faded to how
//!   far its fade is, and once it is all the way, shown; a menu fading out
//!   is faded to one less that, and at the end hidden: taken away when it
//!   marked itself to leave the stack (`6002`), else its tile hidden. While
//!   any fade is part-way the game stays in menu mode (`+0x11`).
//! * The fade itself (`00712450`, `InterfaceManager::RecursiveFade`): down
//!   the menu's tiles, each picture's alpha becomes its tile's `alpha` /
//!   255 times the fade, held to 0..`alpha` (0 under 0.0001); a tile with
//!   `disablefade` is instead shown only at a whole fade (and what's under
//!   it isn't faded); the tiles under one with id 9000 aren't faded.
//! * Input (`0070c4a0`): the pointer's moving onto and off tiles, presses,
//!   clicks and drags reach a menu only while it is shown; the keys go to
//!   the deepest menu that isn't fading out or hidden (`00720e60`).
//!
//! Not here: the image space effect the game fades along with the first
//! menu opened and the last one closed (the image space manager's effect
//! 15, `007123f0`).

use std::collections::BTreeMap;

use crate::draw::DrawItem;
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The fade time of a menu without a tile (`0101622c`).
pub const NO_TILE_SECONDS: f32 = 0.25;

/// The id whose tiles' children a fade leaves alone (`00712450`).
pub const UNFADED_CHILDREN_ID: f32 = 9000.0;

/// A menu's fade state (`Interface::FADE_STATE` (Xbox PDB)).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Shown = 1,
    FadingOut = 2,
    Hidden = 4,
    FadingIn = 8,
}

/// What became of a menu at the end of the frame (`00711ea0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ended {
    /// Faded out, and marked to leave the stack: taken away.
    Gone,
    /// Faded out and kept, hidden.
    Hidden,
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    menu: TileId,
    elapsed: f32,
    seconds: f32,
}

#[derive(Debug, Clone, Copy)]
struct MenuFade {
    state: State,
    /// The fade last laid on its pictures (1: as they are).
    value: f32,
}

/// The menus' fade states and the interface's list of fades.
#[derive(Debug, Clone, Default)]
pub struct Fades {
    list: Vec<Entry>,
    menus: BTreeMap<TileId, MenuFade>,
    /// A menu's fade was part-way this frame (`+0x11`): menu mode lasts.
    pub fading: bool,
}

/// A menu's fade time (`00a1db20`, `00a1d910`): its `menufade`, or its
/// `explorefade` when that is 0.
pub fn seconds(ui: &mut Ui, menu: TileId) -> f32 {
    let s = ui.number(menu, t::MENUFADE);
    if s == 0.0 {
        ui.number(menu, t::EXPLOREFADE)
    } else {
        s
    }
}

/// `0040ebd0`: the smaller of two, the second when they can't be
/// compared.
fn smaller(a: f32, b: f32) -> f32 {
    if b > a {
        a
    } else {
        b
    }
}

/// `00404010`: the larger of two, the second when they can't be compared.
fn larger(a: f32, b: f32) -> f32 {
    if b < a {
        a
    } else {
        b
    }
}

/// How far a fade is: the seconds gone over its seconds, held to 0..1 the
/// game's way (`00716320`, `00716660`).
fn progress(e: &Entry) -> f32 {
    larger(0.0, smaller(1.0, e.elapsed / e.seconds))
}

impl Fades {
    /// The menus' fades move on by the frame's seconds; finished ones
    /// leave the list (`00716320`).
    pub fn update(&mut self, dt: f32) {
        self.list.retain_mut(|e| {
            e.elapsed += dt;
            progress(e) != 1.0
        });
    }

    /// Adds a fade for a menu, replacing its earlier one (`007164c0`).
    fn add(&mut self, menu: TileId, seconds: f32) {
        self.remove(menu);
        self.list.push(Entry {
            menu,
            elapsed: 0.0,
            seconds: larger(seconds, 0.0),
        });
    }

    /// Takes a menu's fade off the list (`007165d0`).
    fn remove(&mut self, menu: TileId) {
        self.list.retain(|e| e.menu != menu);
    }

    /// How far a menu's fade is (`00716660`): 1 without one, -1 when it
    /// takes no time.
    pub fn amount(&self, menu: TileId) -> f32 {
        match self.list.iter().find(|e| e.menu == menu) {
            None => 1.0,
            Some(e) if e.seconds > 0.0 => progress(e),
            Some(_) => -1.0,
        }
    }

    /// A menu's fade state (a menu not seen yet is hidden, `Menu::Menu`).
    pub fn state(&self, menu: TileId) -> State {
        self.menus.get(&menu).map_or(State::Hidden, |m| m.state)
    }

    /// Whether this menu has been shown or hidden here.
    pub fn knows(&self, menu: TileId) -> bool {
        self.menus.contains_key(&menu)
    }

    /// The fade its pictures are drawn with (1: as they are).
    pub fn value(&self, menu: TileId) -> f32 {
        self.menus.get(&menu).map_or(1.0, |m| m.value)
    }

    /// Whether the pointer reaches the menu: only while it's shown
    /// (`0070c4a0`).
    pub fn takes_pointer(&self, menu: TileId) -> bool {
        self.state(menu) == State::Shown
    }

    /// Whether the keys can go to the menu: not while it's fading out or
    /// hidden (`00720e60`).
    pub fn takes_keys(&self, menu: TileId) -> bool {
        !matches!(self.state(menu), State::FadingOut | State::Hidden)
    }

    fn set_state(&mut self, menu: TileId, state: State) {
        self.menus
            .entry(menu)
            .or_insert(MenuFade { state, value: 1.0 })
            .state = state;
    }

    /// Shows a menu just opened (`Menu::PrepForVisibility`, `00a1dc20`):
    /// its tile hidden until the frame's end shows it, fading in unless
    /// `at_once`. `dt` is the frame's seconds.
    pub fn show(&mut self, ui: &mut Ui, menu: TileId, at_once: bool, dt: f32) {
        ui.set_number(menu, t::VISIBLE, 0.0);
        self.set_state(menu, self.state(menu));
        if !at_once {
            self.start_fade_in(ui, menu, dt);
        }
    }

    /// `Menu::StartFadeIn` (`00a1db20`).
    pub fn start_fade_in(&mut self, ui: &mut Ui, menu: TileId, dt: f32) {
        let s = seconds(ui, menu);
        self.add(menu, s);
        self.set_state(menu, State::FadingIn);
        self.update(dt);
    }

    /// `Menu::StartFadeOut` (`00a1d910`): false (nothing done) when the
    /// menu's tile isn't shown.
    pub fn start_fade_out(&mut self, ui: &mut Ui, menu: TileId, dt: f32) -> bool {
        if ui.number(menu, t::VISIBLE) == 0.0 {
            return false;
        }
        let s = seconds(ui, menu);
        self.add(menu, s);
        self.set_state(menu, State::FadingOut);
        self.update(dt);
        true
    }

    /// `Menu::InstantFadeOut` (`00a1d9e0`, Xbox `82905a00`): hidden at
    /// once, its pictures back to a whole fade, and gone at the frame's
    /// end.
    pub fn instant_fade_out(&mut self, ui: &mut Ui, menu: TileId) {
        if self.state(menu) == State::Hidden {
            return;
        }
        ui.set_number(menu, t::VISIBLE, 0.0);
        recursive_fade(ui, menu, 1.0);
        self.set_menu_value(menu, 1.0);
        self.set_state(menu, State::FadingOut);
        self.remove(menu);
    }

    fn set_menu_value(&mut self, menu: TileId, value: f32) {
        if let Some(m) = self.menus.get_mut(&menu) {
            m.value = value;
        }
    }

    /// Forgets a menu taken away.
    pub fn forget(&mut self, menu: TileId) {
        self.remove(menu);
        self.menus.remove(&menu);
    }

    /// The frame's end for these menus (`00711ea0`), bottom first: the
    /// fades laid on, states moved on; the menus that finished fading out,
    /// and how (after one is taken away the others wait for the next
    /// frame, as the game's loop stops there).
    pub fn frame(&mut self, ui: &mut Ui, menus: &[TileId]) -> Vec<(TileId, Ended)> {
        self.fading = false;
        let mut ended = Vec::new();
        for &menu in menus {
            let amount = self.amount(menu);
            let whole = amount.abs() == 1.0;
            match self.state(menu) {
                State::FadingOut if !whole => {
                    self.fading = true;
                    ui.set_number(menu, t::VISIBLE, 1.0);
                    recursive_fade(ui, menu, 1.0 - amount);
                    self.set_menu_value(menu, 1.0 - amount);
                }
                State::FadingOut => {
                    self.set_state(menu, State::Hidden);
                    if ui.number(menu, crate::menu::LEAVE_STACK) != 0.0 {
                        // Taken away, and the rest wait for the next frame.
                        ended.push((menu, Ended::Gone));
                        break;
                    } else {
                        ui.set_number(menu, t::VISIBLE, 0.0);
                        recursive_fade(ui, menu, 0.0);
                        self.set_menu_value(menu, 0.0);
                        ended.push((menu, Ended::Hidden));
                    }
                }
                State::FadingIn if !whole => {
                    self.fading = true;
                    ui.set_number(menu, t::VISIBLE, 1.0);
                    recursive_fade(ui, menu, amount);
                    self.set_menu_value(menu, amount);
                }
                State::FadingIn => {
                    ui.set_number(menu, t::VISIBLE, 1.0);
                    recursive_fade(ui, menu, 1.0);
                    self.set_menu_value(menu, 1.0);
                    self.set_state(menu, State::Shown);
                }
                State::Shown | State::Hidden => {}
            }
        }
        ended
    }
}

/// The part of the fade (`00712450`) that changes the tiles: a tile with
/// `disablefade` is shown only at a whole fade (what's under it untouched);
/// the children of a tile with id 9000 are left alone.
pub fn recursive_fade(ui: &mut Ui, tile: TileId, value: f32) {
    if ui.number(tile, t::DISABLEFADE) != 0.0 {
        ui.set_number(tile, t::VISIBLE, f32::from(value.abs() >= 1.0));
        return;
    }
    if ui.has(tile, t::ID) && ui.number(tile, t::ID) == UNFADED_CHILDREN_ID {
        return;
    }
    for c in ui.tiles[tile].children.clone() {
        recursive_fade(ui, c, value);
    }
}

/// A picture's alpha under a fade (`00712450`): the tile's alpha (0..1)
/// times the fade, held to 0..alpha; 0 when the fade is under 0.0001.
pub fn faded_alpha(alpha: f32, value: f32) -> f32 {
    if value.abs() < 0.0001 {
        0.0
    } else {
        (alpha * value).min(alpha).max(0.0)
    }
}

/// Lays a menu's fade on what it draws (the pictures' part of
/// `00712450`): each item's alpha faded, except under a `disablefade` tile
/// (that tile included) or below a tile with id 9000.
pub fn fade_items(ui: &mut Ui, menu: TileId, value: f32, items: &mut [DrawItem]) {
    if value == 1.0 {
        return;
    }
    for item in items.iter_mut() {
        if faded(ui, menu, item.tile) {
            item.color[3] = faded_alpha(item.color[3], value);
        }
    }
}

/// Whether the fade reaches a tile of the menu.
fn faded(ui: &mut Ui, menu: TileId, tile: TileId) -> bool {
    let mut at = Some(tile);
    let mut own = true;
    while let Some(c) = at {
        if ui.number(c, t::DISABLEFADE) != 0.0 {
            return false;
        }
        if !own && ui.has(c, t::ID) && ui.number(c, t::ID) == UNFADED_CHILDREN_ID {
            return false;
        }
        if c == menu {
            break;
        }
        own = false;
        at = ui.tiles[c].parent;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu::{MenuCode, LEAVE_STACK};
    use crate::menus::test_support;

    struct Code;
    impl MenuCode for Code {
        fn class(&self) -> i32 {
            0
        }
        fn click(&mut self, _: &mut Ui, _: i32, _: Option<TileId>, _: f64) {}
    }

    fn menu(ui: &mut Ui, extra: &str) -> TileId {
        test_support::load(
            ui,
            &format!(
                "<menu name=\"M\">{extra}<image name=\"a\"><filename>solid.dds</filename><width>10</width><height>10</height><alpha>200</alpha></image>
                 <rect name=\"keep\"><id>9000</id><image name=\"b\"><filename>solid.dds</filename></image></rect>
                 <image name=\"d\"><disablefade>&true;</disablefade><filename>solid.dds</filename></image></menu>"
            ),
            &mut Code,
        )
    }

    fn child(ui: &Ui, under: TileId, name: &str) -> TileId {
        ui.descendants(under)
            .into_iter()
            .find(|&c| ui.tiles[c].name == name)
            .unwrap()
    }

    #[test]
    fn fade_time_is_menufade_or_explorefade() {
        let mut ui = test_support::ui();
        let m = menu(&mut ui, "");
        // The menu tile's own 0.25 (`00a1ef30`).
        assert_eq!(seconds(&mut ui, m), 0.25);
        ui.set_number(m, t::MENUFADE, 0.75);
        assert_eq!(seconds(&mut ui, m), 0.75);
        ui.set_number(m, t::MENUFADE, 0.0);
        ui.set_number(m, t::EXPLOREFADE, 0.5);
        assert_eq!(seconds(&mut ui, m), 0.5);
    }

    #[test]
    fn a_menu_fades_in_then_is_shown() {
        let mut ui = test_support::ui();
        let m = menu(&mut ui, "");
        let mut f = Fades::default();
        assert_eq!(f.state(m), State::Hidden);
        // Opened: hidden, fading in, already a frame along (`00706f70`).
        f.show(&mut ui, m, false, 0.05);
        assert_eq!(ui.number(m, t::VISIBLE), 0.0);
        assert_eq!(f.state(m), State::FadingIn);
        assert!((f.amount(m) - 0.2).abs() < 1e-6);
        assert!(!f.takes_pointer(m));
        assert!(f.takes_keys(m));
        // The frame's end shows it at that fade.
        assert!(f.frame(&mut ui, &[m]).is_empty());
        assert!(f.fading);
        assert_eq!(ui.number(m, t::VISIBLE), 1.0);
        assert!((f.value(m) - 0.2).abs() < 1e-6);
        // Next frames: further, then whole and shown.
        f.update(0.1);
        f.frame(&mut ui, &[m]);
        assert!((f.value(m) - 0.6).abs() < 1e-6);
        f.update(0.1);
        f.frame(&mut ui, &[m]);
        assert_eq!(f.state(m), State::Shown);
        assert_eq!(f.value(m), 1.0);
        assert!(!f.fading);
        assert!(f.takes_pointer(m));
    }

    #[test]
    fn a_closed_menu_fades_out_and_goes() {
        let mut ui = test_support::ui();
        let m = menu(&mut ui, "<menufade>0.75</menufade>");
        let mut f = Fades::default();
        f.show(&mut ui, m, false, 1.0);
        f.frame(&mut ui, &[m]);
        assert_eq!(f.state(m), State::Shown);
        ui.set_number(m, LEAVE_STACK, 1.0);
        assert!(f.start_fade_out(&mut ui, m, 0.25));
        assert_eq!(f.state(m), State::FadingOut);
        assert!(!f.takes_keys(m));
        assert!(!f.takes_pointer(m));
        assert!(f.frame(&mut ui, &[m]).is_empty());
        assert!((f.value(m) - (1.0 - 0.25 / 0.75)).abs() < 1e-6);
        assert!(f.fading);
        f.update(0.5);
        assert_eq!(f.frame(&mut ui, &[m]), vec![(m, Ended::Gone)]);
        assert_eq!(f.state(m), State::Hidden);
    }

    #[test]
    fn after_a_menu_goes_the_rest_wait_a_frame() {
        let mut ui = test_support::ui();
        let a = menu(&mut ui, "");
        let b = menu(&mut ui, "");
        let mut f = Fades::default();
        for m in [a, b] {
            f.show(&mut ui, m, false, 1.0);
        }
        f.frame(&mut ui, &[a, b]);
        for m in [a, b] {
            ui.set_number(m, LEAVE_STACK, 1.0);
            f.start_fade_out(&mut ui, m, 1.0);
        }
        assert_eq!(f.frame(&mut ui, &[a, b]), vec![(a, Ended::Gone)]);
        assert_eq!(f.frame(&mut ui, &[b]), vec![(b, Ended::Gone)]);
    }

    #[test]
    fn a_kept_menu_is_hidden_after_its_fade() {
        let mut ui = test_support::ui();
        let m = menu(&mut ui, "");
        let mut f = Fades::default();
        f.show(&mut ui, m, false, 1.0);
        f.frame(&mut ui, &[m]);
        ui.set_number(m, LEAVE_STACK, 0.0);
        f.start_fade_out(&mut ui, m, 1.0);
        assert_eq!(f.frame(&mut ui, &[m]), vec![(m, Ended::Hidden)]);
        assert_eq!(ui.number(m, t::VISIBLE), 0.0);
    }

    #[test]
    fn a_hidden_menu_does_not_fade_out() {
        let mut ui = test_support::ui();
        let m = menu(&mut ui, "");
        let mut f = Fades::default();
        f.show(&mut ui, m, false, 0.0);
        // Still hidden: `StartFadeOut` does nothing (`00a1d910`).
        assert!(!f.start_fade_out(&mut ui, m, 0.0));
        assert_eq!(f.state(m), State::FadingIn);
    }

    #[test]
    fn instant_fade_out_ends_at_the_frame_end() {
        let mut ui = test_support::ui();
        let m = menu(&mut ui, "");
        let mut f = Fades::default();
        f.show(&mut ui, m, false, 1.0);
        f.frame(&mut ui, &[m]);
        ui.set_number(m, LEAVE_STACK, 1.0);
        f.instant_fade_out(&mut ui, m);
        assert_eq!(ui.number(m, t::VISIBLE), 0.0);
        assert_eq!(f.frame(&mut ui, &[m]), vec![(m, Ended::Gone)]);
    }

    #[test]
    fn starting_a_fade_moves_every_fade_on() {
        let mut ui = test_support::ui();
        let a = menu(&mut ui, "");
        let b = menu(&mut ui, "");
        let mut f = Fades::default();
        f.show(&mut ui, a, false, 0.05);
        // b's start moves a's on by the frame's seconds too (`00706f70`).
        f.show(&mut ui, b, false, 0.05);
        assert!((f.amount(a) - 0.4).abs() < 1e-6);
        assert!((f.amount(b) - 0.2).abs() < 1e-6);
        // A new fade for a menu replaces its old one (`007164c0`).
        f.start_fade_in(&mut ui, a, 0.0);
        assert_eq!(f.amount(a), 0.0);
    }

    #[test]
    fn a_fade_that_takes_no_time() {
        let mut ui = test_support::ui();
        let m = menu(
            &mut ui,
            "<menufade>0</menufade><explorefade>0</explorefade>",
        );
        let mut f = Fades::default();
        f.add(m, 0.0);
        assert_eq!(f.amount(m), -1.0);
        // Gone from the list at the next step (0/0 isn't a whole fade but
        // anything after is).
        f.update(0.01);
        assert_eq!(f.amount(m), 1.0);
    }

    #[test]
    fn the_fade_on_pictures() {
        assert_eq!(faded_alpha(0.8, 0.5), 0.4);
        assert_eq!(faded_alpha(0.8, 0.00001), 0.0);
        assert_eq!(faded_alpha(0.8, 2.0), 0.8);
        assert_eq!(faded_alpha(0.8, -1.0), 0.0);
        let mut ui = test_support::ui();
        let m = menu(&mut ui, "");
        let a = child(&ui, m, "a");
        let keep = child(&ui, m, "keep");
        let b = child(&ui, m, "b");
        let d = child(&ui, m, "d");
        let item = |tile| DrawItem {
            tile,
            depth: 0.0,
            color: [1.0, 1.0, 1.0, 0.8],
            kind: crate::draw::DrawKind::Text {
                font: 0,
                glyphs: Vec::new(),
            },
        };
        let mut items = vec![item(a), item(keep), item(b), item(d)];
        fade_items(&mut ui, m, 0.5, &mut items);
        let alphas: Vec<f32> = items.iter().map(|i| i.color[3]).collect();
        // The id-9000 tile itself fades, its child doesn't; the
        // `disablefade` one doesn't.
        assert_eq!(alphas, vec![0.4, 0.4, 0.8, 0.8]);
        // And `disablefade` shows only at a whole fade.
        recursive_fade(&mut ui, m, 0.5);
        assert_eq!(ui.number(d, t::VISIBLE), 0.0);
        recursive_fade(&mut ui, m, 1.0);
        assert_eq!(ui.number(d, t::VISIBLE), 1.0);
    }
}
