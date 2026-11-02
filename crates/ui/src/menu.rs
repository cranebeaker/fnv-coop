//! Menus as the game's interface manager runs them (`InterfaceManager`,
//! its update `0070c4a0` in FalloutNV.exe): a menu file loaded under the
//! screen and given to its menu class's code, the tile under the pointer
//! (`007126c0`), "mouse over" and clicks reaching the menu (`00717e70`,
//! `00717ef0`, the click at `0070d0xx`), the keyboard (`007154b0` turns
//! keys into menu codes; the top menu's `HandleKeyboardInput`, then its
//! `_PCButton_<key>` traits, then the arrows and Enter, `0070f6e0`).
//!
//! Every menu class is a C++ class with the same virtual functions (the
//! `Menu` vtable, `01095484`): set a tile by id (`+0x04`), a click
//! (`+0x0c`), the pointer moving onto and off a tile (`+0x10`, `+0x14`), an
//! update every frame (`+0x2c`), typed keys (`+0x30`), its class number
//! (`+0x34`) and the arrows and Enter (`+0x38`). [`MenuCode`] is that set.

use crate::list::ListBox;
use crate::names::{kind, op, t};
use crate::tile::{Operand, TileId, Ui};

/// What `007154b0` makes of a key for the menus: a character code, or one
/// of these (the interface's codes with the top bit set).
pub mod key {
    pub const BACKSPACE: u32 = 0x8000_0000;
    pub const LEFT: u32 = 0x8000_0001;
    pub const RIGHT: u32 = 0x8000_0002;
    pub const UP: u32 = 0x8000_0003;
    pub const DOWN: u32 = 0x8000_0004;
    pub const HOME: u32 = 0x8000_0005;
    pub const END: u32 = 0x8000_0006;
    pub const DELETE: u32 = 0x8000_0007;
    pub const ENTER: u32 = 0x8000_0008;
    pub const PAGE_UP: u32 = 0x8000_0009;
    pub const PAGE_DOWN: u32 = 0x8000_000A;
    /// What the Escape control (control 28) sends to the message box
    /// (`0070c4a0`: a `1` when the message menu is open).
    pub const ESCAPE_TO_MESSAGE: u32 = 1;
}

/// The menus' "special" codes for the arrows and Enter (`0070c4a0` maps
/// the key codes onto them; `0070f6e0` hands them on): the same numbers
/// the game's controller buttons use.
pub mod special {
    pub const UP: i32 = 1;
    pub const DOWN: i32 = 2;
    pub const RIGHT: i32 = 3;
    pub const LEFT: i32 = 4;
    /// The A button; Enter arrives as -2 and is turned into this.
    pub const A: i32 = 9;
    pub const ENTER: i32 = -2;
    pub const SHIFT_ENTER: i32 = 11;
    pub const ALT_ENTER: i32 = 12;
    pub const PAGE_UP: i32 = 15;
    pub const PAGE_DOWN: i32 = 16;
}

/// The special code a key turns into (`0070c4a0`): the arrows, Enter (with
/// Shift or Alt), Page Up and Down.
pub fn special_of(code: u32, shift: bool, alt: bool) -> Option<i32> {
    match code {
        key::LEFT => Some(special::LEFT),
        key::RIGHT => Some(special::RIGHT),
        key::UP => Some(special::UP),
        key::DOWN => Some(special::DOWN),
        key::ENTER if shift => Some(special::SHIFT_ENTER),
        key::ENTER if alt => Some(special::ALT_ENTER),
        key::ENTER => Some(special::ENTER),
        key::PAGE_UP => Some(special::PAGE_UP),
        key::PAGE_DOWN => Some(special::PAGE_DOWN),
        _ => None,
    }
}

/// The traits the arrows and buttons look up (`0070f6e0`): `xup`, `xdown`,
/// `xleft`, `xright` for the arrows, `xbuttona`.. for the others.
pub fn special_trait(code: i32) -> Option<i32> {
    Some(match code {
        special::UP => 4055,
        special::DOWN => 4056,
        special::LEFT => 4057,
        special::RIGHT => 4058,
        9 => 4061,
        10 => 4062,
        11 => 4063,
        12 => 4064,
        13 => 4065,
        14 => 4066,
        15 => 4067,
        16 => 4068,
        5 => 4071,
        _ => return None,
    })
}

/// `xdefault`: which tile the arrows start from.
pub const XDEFAULT: i32 = 4054;
/// `mouseoversound`.
pub const MOUSEOVERSOUND: i32 = 4072;
/// `wheelable` and `wheelmoved`.
pub const WHEELABLE: i32 = 4082;
pub const WHEELMOVED: i32 = 4083;
/// The menu tile's "take it off the menu stack when it closes" flag (trait
/// 6002, set by the menus' code: the message box clears it while shown).
pub const LEAVE_STACK: i32 = 6002;

/// Dragging (`0070c4a0`): `draggable` marks a tile the pointer can drag;
/// while it's dragged `dragx`/`dragy` hold the pointer in the tile's own
/// frame (the pointer less the tile's position on screen less its own
/// `x`/`y`), `dragstartx`/`y` where it went down (whole units; -1 once let
/// go), `dragoffsetx`/`y` the pointer less the tile's position on screen
/// at that moment, `dragdeltax`/`y` the whole units moved since the last
/// frame.
pub mod drag {
    pub const DRAGGABLE: i32 = 4073;
    pub const START_X: i32 = 4074;
    pub const START_Y: i32 = 4075;
    pub const OFFSET_X: i32 = 4076;
    pub const OFFSET_Y: i32 = 4077;
    pub const DELTA_X: i32 = 4078;
    pub const DELTA_Y: i32 = 4079;
    pub const X: i32 = 4080;
    pub const Y: i32 = 4081;
}

/// The menus' own sounds by number (`00717280`, which the menus' code
/// calls): 1 `UIMenuOK`, 2 and 0x14 `UIMenuCancel`, 3 `UIMenuPrevNext`, 4
/// `UIMenuFocus`, 8 `UIPopUpQuestNew`, 10 and 0x13
/// `UIPopUpMessageGeneral`, 0x15 `UILevelUp`, 0x24 `UIMenuMode`; others
/// (and -1) none.
pub fn menu_sound(number: i32) -> Option<&'static str> {
    Some(match number {
        1 => "UIMenuOK",
        2 | 0x14 => "UIMenuCancel",
        3 => "UIMenuPrevNext",
        4 => "UIMenuFocus",
        8 => "UIPopUpQuestNew",
        10 | 0x13 => "UIPopUpMessageGeneral",
        0x15 => "UILevelUp",
        0x24 => "UIMenuMode",
        _ => return None,
    })
}

/// The menu code's side of a menu (the `Menu` vtable).
pub trait MenuCode {
    /// Its class number (`+0x34`): 1001 the message box, 1009 dialogue...
    fn class(&self) -> i32;
    /// A tile with an `id` in its file (`+0x04`, `SetTile`).
    fn set_tile(&mut self, _id: i32, _tile: TileId) {}
    /// A click on a tile (or a button acting as one) (`+0x0c`).
    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, now: f64);
    /// The pointer onto a tile (`+0x10`).
    fn mouseover(&mut self, _ui: &mut Ui, _id: i32, _tile: TileId) {}
    /// The pointer off a tile (`+0x14`).
    fn unmouseover(&mut self, _ui: &mut Ui, _id: i32, _tile: TileId) {}
    /// A typed key (`+0x30`): a character, or a [`key`] code. True when the
    /// menu used it.
    fn key(&mut self, _ui: &mut Ui, _code: u32, _now: f64) -> bool {
        false
    }
    /// An arrow, Enter or a page key as a [`special`] code (`+0x38`). True
    /// when the menu used it.
    fn special_key(&mut self, _ui: &mut Ui, _code: i32, _now: f64) -> bool {
        false
    }
    /// The mouse wheel over a tile (`+0x28`); `delta` is the interface's
    /// wheel movement (`0070ec70`: 120 a notch, away from the player
    /// positive).
    fn wheel(&mut self, _ui: &mut Ui, _id: i32, _tile: TileId, _delta: i32) {}
    /// Its list boxes (the interface keeps a list of every list box: the
    /// pointer chooses their items, the arrows step through them).
    fn lists(&mut self) -> Vec<&mut ListBox> {
        Vec::new()
    }
}

/// Loads a menu file under the screen and starts it as the game does: the
/// file read (`00a01b00`), its depth put above the menus open (`depth`,
/// from [`next_depth`]), and every tile with an `id` handed to the menu's
/// code (`SetTile`). The menu stays hidden (menus start hidden) until its
/// code shows it.
pub fn load(
    ui: &mut Ui,
    read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
    path: &str,
    code: &mut dyn MenuCode,
    depth: f32,
) -> Result<TileId, String> {
    let text = read(path).ok_or_else(|| format!("{path} not found"))?;
    let menu = ui.load_menu(&text, read)?;
    ui.set_number(menu, t::DEPTH, depth);
    for tile in ui.descendants(menu) {
        if ui.has(tile, t::ID) {
            let id = ui.number(tile, t::ID) as i32;
            if id >= 0 {
                code.set_tile(id, tile);
            }
        }
    }
    Ok(menu)
}

/// The depth a menu opening now gets (`00a1dfb0`): 2 above the highest
/// open menu's own depth plus the depth used inside it.
pub fn next_depth(ui: &mut Ui, open: &[TileId]) -> f32 {
    let mut top = 0.0f32;
    for &menu in open {
        let base = ui.number(menu, t::DEPTH);
        let mut inner = 0.0f32;
        for tile in ui.descendants(menu) {
            inner = inner.max(ui.screen_depth(tile) - base);
        }
        top = top.max(base + inner);
    }
    top + 2.0
}

/// Whether a tile is shown and on screen: its own `visible` and its
/// ancestors' (`00a040a0`).
fn shown(ui: &mut Ui, tile: TileId) -> bool {
    ui.shown(tile)
}

/// Whether a tile clips and the rectangle its clip window gives it: the
/// drawing's rules (`00a037e0`), which picking follows too.
pub use crate::draw::{clip_rect, clips};

fn inside(r: [f32; 4], x: f32, y: f32) -> bool {
    x >= r[0] && x < r[0] + r[2] && y >= r[1] && y < r[1] + r[3]
}

/// Whether the point (menu units) is on the tile's picture: an image's
/// rectangle, a text's glyphs (the game picks the drawn geometry); a
/// radial tile's slice of its circle besides ([`in_slice`]).
fn hits(ui: &mut Ui, tile: TileId, x: f32, y: f32) -> bool {
    let (tx, ty) = ui.screen_position(tile);
    match ui.tiles[tile].kind {
        kind::IMAGE | kind::HOTRECT => {
            let w = ui.number(tile, t::WIDTH);
            let h = ui.number(tile, t::HEIGHT);
            inside([tx, ty, w, h], x, y)
        }
        kind::RADIAL => {
            let w = ui.number(tile, t::WIDTH);
            let h = ui.number(tile, t::HEIGHT);
            inside([tx, ty, w, h], x, y) && in_slice(ui, tile, x, y)
        }
        kind::TEXT => match ui.layout(tile) {
            Some(layout) => layout.quads.iter().any(|q| {
                q.right > q.left
                    && inside(
                        [tx + q.left, ty + q.top, q.right - q.left, q.bottom - q.top],
                        x,
                        y,
                    )
            }),
            None => false,
        },
        _ => false,
    }
}

/// A radial tile's own test (`RadialTile`'s slot 0x14, `00a216b0`): the
/// point's angle around the centre (`user0`, `user1`), clockwise from
/// straight up (a point level with it: π/2 to the right, 3π/2 to the left),
/// within `user2` .. `user3`, and its distance within `user4` .. `user5`.
/// (The game scales the pointer to the menu's units first; it's in them
/// here.)
pub fn in_slice(ui: &mut Ui, tile: TileId, x: f32, y: f32) -> bool {
    let u = |ui: &mut Ui, i: i32| ui.number(tile, t::USER0 + i);
    let (cx, cy) = (u(ui, 0), u(ui, 1));
    let (fx, fy) = (x - cx, y - cy);
    let angle = slice_angle(fx, fy);
    if !(u(ui, 2) <= angle && angle <= u(ui, 3)) {
        return false;
    }
    let distance = (fx * fx + fy * fy).sqrt();
    u(ui, 4) <= distance && distance <= u(ui, 5)
}

/// `00a216b0`'s angle of a point `fx`, `fy` from the centre (y down):
/// atan(−fx / fy), + π below the centre, + 2π up and to the left (with the
/// exe's own rounded constants).
#[allow(clippy::approx_constant)]
pub fn slice_angle(fx: f32, fy: f32) -> f32 {
    let mut a = if fy == 0.0 {
        if fx <= 0.0 {
            4.71238
        } else {
            1.57075
        }
    } else {
        (-fx / fy).atan()
    };
    if fy > 0.0 {
        a += 3.14159;
    }
    if fx < 0.0 && fy < 0.0 {
        a += 6.28318;
    }
    a
}

/// The tile under the point (`007126c0`): the drawn tiles of the menu
/// (`menu`: the top menu, which takes all clicks when it stacks "no click
/// past", `00716910`) from the nearest (highest depth, then the later in
/// the tree) on, skipping those that aren't `target`s or are outside their
/// clip window; the first hit, or the nearest ancestor with an `id` when it
/// has none.
pub fn pick(ui: &mut Ui, menu: TileId, x: f32, y: f32) -> Option<TileId> {
    let mut candidates: Vec<(f32, usize, TileId)> = Vec::new();
    for (order, tile) in ui.descendants(menu).into_iter().enumerate() {
        if !matches!(
            ui.tiles[tile].kind,
            kind::IMAGE | kind::HOTRECT | kind::TEXT | kind::RADIAL
        ) {
            continue;
        }
        if !shown(ui, tile) {
            continue;
        }
        candidates.push((ui.screen_depth(tile), order, tile));
    }
    candidates.sort_by(|a, b| {
        b.0.partial_cmp(&a.0)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(b.1.cmp(&a.1))
    });
    for (_, _, tile) in candidates {
        if !hits(ui, tile, x, y) {
            continue;
        }
        if let Some(r) = clip_rect(ui, tile) {
            if !inside(r, x, y) {
                continue;
            }
        }
        if ui.number(tile, t::TARGET) == 0.0 {
            continue;
        }
        if ui.has(tile, t::ID) {
            return Some(tile);
        }
        let mut at = ui.tiles[tile].parent;
        while let Some(p) = at {
            if ui.has(p, t::ID) {
                return Some(p);
            }
            at = ui.tiles[p].parent;
        }
        return Some(tile);
    }
    None
}

/// A tile's `id` as the interface passes it on (rounded down to a whole
/// number).
pub fn id_of(ui: &mut Ui, tile: TileId) -> i32 {
    ui.number(tile, t::ID) as i32
}

/// What the interface asks the game to do besides the menus' own code:
/// sounds by their editor ID (`clicksound`, `mouseoversound` traits).
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    Sound(String),
}

/// The interface manager's pointer and keyboard state for the menus.
#[derive(Debug, Clone, Default)]
pub struct Interface {
    /// The tile the pointer is over (`+0xcc`).
    pub over: Option<TileId>,
    /// The tile chosen with the keyboard (`+0xbc`).
    pub focus: Option<TileId>,
    /// Where the mouse button went down (`+0xd4`).
    pressed: Option<TileId>,
    /// The tile being dragged (`+0x4c`), how far the pointer has gone
    /// since the drag began in whole units (`+0x60`, `+0x64`) and how far
    /// it had gone at the last frame (`+0x54`, `+0x58`).
    pub dragging: Option<TileId>,
    travel: (f32, f32),
    last_travel: (f32, f32),
    /// Sounds and the like for the caller.
    pub effects: Vec<Effect>,
}

impl Interface {
    fn sound_of(&mut self, ui: &mut Ui, tile: TileId, trait_id: i32) {
        if let Some(name) = ui.string(tile, trait_id) {
            let name = name.trim();
            if !name.is_empty() {
                self.effects.push(Effect::Sound(name.to_string()));
            }
        }
    }

    /// The pointer onto a tile (`00717e70`): `mouseover` 1, the first list
    /// box that takes it chooses it, then the menu hears of it; its
    /// `mouseoversound` plays.
    fn enter(&mut self, ui: &mut Ui, code: &mut dyn MenuCode, tile: TileId, sound: bool) {
        ui.set_number(tile, t::MOUSEOVER, 1.0);
        if sound {
            self.sound_of(ui, tile, MOUSEOVERSOUND);
        }
        for list in code.lists() {
            if list.select(ui, Some(tile)) {
                break;
            }
        }
        let id = id_of(ui, tile);
        code.mouseover(ui, id, tile);
        ui.refresh();
    }

    /// The pointer off a tile (`00717ef0`): `mouseover` 0, a list box that
    /// had it chosen lets it go, then the menu hears of it.
    fn leave(&mut self, ui: &mut Ui, code: &mut dyn MenuCode, tile: TileId) {
        ui.set_number(tile, t::MOUSEOVER, 0.0);
        for list in code.lists() {
            if list.selected == Some(tile) && list.select(ui, None) {
                break;
            }
        }
        let id = id_of(ui, tile);
        code.unmouseover(ui, id, tile);
        ui.refresh();
    }

    /// A click on a tile (`0070c4a0`): its `clicksound`, `clicked` pulsed to
    /// 1 and back to 0, then the menu's `HandleClick` with its id.
    pub fn click(
        &mut self,
        ui: &mut Ui,
        menu: TileId,
        code: &mut dyn MenuCode,
        tile: TileId,
        now: f64,
    ) {
        self.sound_of(ui, tile, t::CLICKSOUND);
        ui.set_number(tile, t::CLICKED, 1.0);
        ui.work_out_all(menu);
        ui.set_number(tile, t::CLICKED, 0.0);
        ui.work_out_all(menu);
        let id = id_of(ui, tile);
        code.click(ui, id, Some(tile), now);
        ui.refresh();
    }

    /// The pointer at (x, y) in menu units, with the left button's state:
    /// `down` held, `pressed` / `released` this frame, in the order
    /// `0070c4a0` takes them: a tile being dragged follows the pointer; the
    /// tile under it changes only while the button isn't held; a release
    /// over the tile the button went down on clicks it and ends a drag;
    /// then a press over a `draggable` tile starts dragging it; while held,
    /// the drag's movement since the last frame.
    #[allow(clippy::too_many_arguments)]
    pub fn pointer(
        &mut self,
        ui: &mut Ui,
        menu: TileId,
        code: &mut dyn MenuCode,
        x: f32,
        y: f32,
        down: bool,
        pressed: bool,
        released: bool,
        now: f64,
    ) {
        self.drop_gone(ui);
        if let Some(d) = self.dragging {
            self.travel = (
                (x - ui.number(d, drag::START_X)).trunc(),
                (y - ui.number(d, drag::START_Y)).trunc(),
            );
            let (sx, sy) = ui.screen_position(d);
            let own_x = ui.number(d, t::X);
            let own_y = ui.number(d, t::Y);
            ui.set_number(d, drag::X, x - (sx - own_x));
            ui.set_number(d, drag::Y, y - (sy - own_y));
            ui.refresh();
        }
        let under = pick(ui, menu, x, y);
        if under != self.over && !down {
            if let Some(old) = self.over {
                self.leave(ui, code, old);
            }
            self.over = under;
            if let Some(new) = under {
                // The keyboard's choice gives way to the pointer.
                self.focus = Some(new);
                self.enter(ui, code, new, true);
            }
        }
        if released {
            if let (Some(p), Some(o)) = (self.pressed, self.over) {
                if p == o && ui.number(o, t::TARGET) != 0.0 {
                    self.click(ui, menu, code, o, now);
                }
            }
            self.pressed = None;
            if let Some(d) = self.dragging.take() {
                ui.set_number(d, drag::START_X, -1.0);
                ui.set_number(d, drag::START_Y, -1.0);
                ui.refresh();
            }
        }
        if pressed {
            self.pressed = self.over;
            if let Some(o) = self.over.filter(|&o| ui.number(o, drag::DRAGGABLE) != 0.0) {
                self.dragging = Some(o);
                let (sx, sy) = ui.screen_position(o);
                let own_x = ui.number(o, t::X);
                let own_y = ui.number(o, t::Y);
                self.travel = (0.0, 0.0);
                self.last_travel = (0.0, 0.0);
                ui.set_number(o, drag::X, x - (sx - own_x));
                ui.set_number(o, drag::Y, y - (sy - own_y));
                ui.set_number(o, drag::START_X, x.trunc());
                ui.set_number(o, drag::START_Y, y.trunc());
                ui.set_number(o, drag::OFFSET_X, x - sx);
                ui.set_number(o, drag::OFFSET_Y, y - sy);
                ui.refresh();
            }
        }
        if down {
            if let Some(d) = self.dragging {
                ui.set_number(d, drag::DELTA_X, self.travel.0 - self.last_travel.0);
                ui.set_number(d, drag::DELTA_Y, self.travel.1 - self.last_travel.1);
                self.last_travel = self.travel;
                ui.refresh();
            }
        }
    }

    /// Whether the left button is held down on a tile.
    pub fn held(&self) -> bool {
        self.pressed.is_some()
    }

    /// Forgets tiles the menu's code took away (a list's lines filled
    /// again): the game's tiles let the interface go of them as they're
    /// deleted, so the tile under the pointer is picked afresh.
    pub fn drop_gone(&mut self, ui: &Ui) {
        let gone = |t: Option<TileId>| t.is_some_and(|t| !ui.is_under(t, ui.screen));
        if gone(self.over) {
            self.over = None;
        }
        if gone(self.focus) {
            self.focus = None;
        }
        if gone(self.pressed) {
            self.pressed = None;
        }
        if gone(self.dragging) {
            self.dragging = None;
        }
    }

    /// Lets go of a menu's tiles without telling its code: the pointer is
    /// over nothing, nothing pressed or dragged there and no keyboard
    /// choice in it. The interface does this when the menu under the
    /// pointer isn't shown (fading in or out, `0070c4a0`: the tile under
    /// the pointer and its menu cleared, no "mouse off" for a menu not
    /// shown).
    pub fn let_go_of(&mut self, ui: &Ui, menu: TileId) {
        let of = |t: Option<TileId>| t.is_some_and(|t| ui.is_under(t, menu));
        if of(self.over) {
            self.over = None;
        }
        if of(self.focus) {
            self.focus = None;
        }
        if of(self.pressed) {
            self.pressed = None;
        }
        if of(self.dragging) {
            self.dragging = None;
        }
    }

    /// The mouse wheel turned by `notches` (up positive) over the menu: the
    /// nearest `wheelable` tile under the pointer gets `wheelmoved` (the
    /// game's count of steps, down positive) and the menu hears of it.
    pub fn wheel(&mut self, ui: &mut Ui, menu: TileId, code: &mut dyn MenuCode, notches: i32) {
        if notches == 0 {
            return;
        }
        let mut at = self.over;
        while let Some(tile) = at {
            if ui.number(tile, WHEELABLE) != 0.0 {
                ui.set_number(tile, WHEELMOVED, -notches as f32);
                ui.work_out_all(menu);
                ui.set_number(tile, WHEELMOVED, 0.0);
                ui.work_out_all(menu);
                let id = id_of(ui, tile);
                code.wheel(ui, id, tile, notches * 120);
                return;
            }
            at = ui.tiles[tile].parent;
        }
    }

    /// Moves the keyboard's choice to a tile (`00715860` with `mouseover`):
    /// the old one let go, the new one entered.
    pub fn choose(
        &mut self,
        ui: &mut Ui,
        code: &mut dyn MenuCode,
        tile: Option<TileId>,
        sound: bool,
    ) {
        if self.focus == tile {
            return;
        }
        if let Some(old) = self.focus.or(self.over) {
            self.leave(ui, code, old);
        }
        self.over = None;
        self.focus = tile;
        if let Some(new) = tile {
            self.enter(ui, code, new, sound);
        }
    }

    /// A key for the top menu (`0070c4a0`): first the menu's own handling
    /// (`HandleKeyboardInput`); then a `_PCButton_<key>` trait on the menu
    /// naming a tile, which is clicked; then the arrows and Enter.
    #[allow(clippy::too_many_arguments)]
    pub fn key(
        &mut self,
        ui: &mut Ui,
        menu: TileId,
        code: &mut dyn MenuCode,
        key_code: u32,
        shift: bool,
        alt: bool,
        now: f64,
    ) {
        if code.key(ui, key_code, now) {
            ui.refresh();
            return;
        }
        if key_code < 0x100 {
            let name = format!(
                "_PCButton_{}",
                (key_code as u8 as char).to_ascii_uppercase()
            );
            if let Some(trait_id) = ui.names.lookup(&name) {
                if let Some(target) = ui.string(menu, trait_id) {
                    if let Some(tile) = ui.find(menu, target.trim()) {
                        if ui.shown(tile) {
                            // A button that can't be used now answers with
                            // the cancel sound (`00717280(2)`).
                            if ui.number(tile, t::TARGET) != 0.0 {
                                self.click(ui, menu, code, tile, now);
                            } else if let Some(s) = menu_sound(2) {
                                self.effects.push(Effect::Sound(s.to_string()));
                            }
                        }
                    }
                }
            }
            return;
        }
        if let Some(s) = special_of(key_code, shift, alt) {
            self.special(ui, menu, code, s, now);
        }
    }

    /// An arrow, Enter or page key (`0070f6e0`): the menu's own handling,
    /// then its list boxes stepping their choice (`00717f80`), then the
    /// `xup`.. `xbuttona` traits: on the chosen tile (or its ancestors) a
    /// `ref` to another tile's `mouseover` moves the choice there, a `ref`
    /// to `clicked` clicks it; with nothing chosen yet, the tile with the
    /// highest `xdefault` is chosen first (`007160f0`). Enter clicks the
    /// chosen tile.
    pub fn special(
        &mut self,
        ui: &mut Ui,
        menu: TileId,
        code: &mut dyn MenuCode,
        s: i32,
        now: f64,
    ) {
        if code.special_key(ui, s, now) {
            ui.refresh();
            return;
        }
        // List boxes with something chosen step through their items.
        let mut stepped = None;
        for list in code.lists() {
            if list.selected.is_some() {
                if let Some(next) = list.next(ui, s) {
                    stepped = Some(next);
                    break;
                }
            }
        }
        if let Some(next) = stepped {
            self.choose(ui, code, Some(next), true);
            for list in code.lists() {
                if list.selected == Some(next) {
                    list.scroll_to_selected(ui);
                }
            }
            return;
        }
        let current = self.focus.or(self.over);
        // Enter clicks the chosen tile itself (`0070f6e0` turns -2 into the
        // A button with the chosen tile as its target).
        if s == special::ENTER {
            if let Some(tile) = current {
                if ui.shown(tile) && ui.number(tile, t::TARGET) != 0.0 {
                    self.click(ui, menu, code, tile, now);
                }
            }
            return;
        }
        let arrow = matches!(
            s,
            special::UP | special::DOWN | special::LEFT | special::RIGHT
        );
        if current.is_none() && arrow {
            if let Some(first) = default_tile(ui, menu) {
                self.choose(ui, code, Some(first), true);
                return;
            }
        }
        let Some(trait_id) = special_trait(s) else {
            return;
        };
        let from = current.unwrap_or(menu);
        match reference(ui, from, trait_id) {
            Some((tile, target_trait)) if ui.shown(tile) => {
                if target_trait == t::CLICKED {
                    if ui.number(tile, t::TARGET) != 0.0 {
                        self.click(ui, menu, code, tile, now);
                    }
                } else if target_trait == t::MOUSEOVER {
                    self.choose(ui, code, Some(tile), true);
                }
            }
            _ => {
                if s == special::A {
                    if let Some(tile) = current {
                        if ui.number(tile, t::TARGET) != 0.0 {
                            self.click(ui, menu, code, tile, now);
                        }
                    }
                }
            }
        }
    }
}

/// The tile a navigation trait refers to (`00a03f70`): the trait's `ref`
/// operator on the tile, else on its ancestors; the tile and the trait it
/// names.
pub fn reference(ui: &Ui, from: TileId, trait_id: i32) -> Option<(TileId, i32)> {
    let mut at = Some(from);
    while let Some(tile) = at {
        if let Some(tr) = ui.tiles[tile].traits.get(&trait_id) {
            for a in &tr.actions {
                if a.op == op::REF {
                    if let Operand::Link {
                        tile: target,
                        trait_id: target_trait,
                    } = a.operand
                    {
                        return Some((target, target_trait));
                    }
                }
            }
            return None;
        }
        at = ui.tiles[tile].parent;
    }
    None
}

/// The tile the arrows start from (`007160f0`): among the shown `target`
/// tiles with an `xdefault`, the highest `xdefault`, ties to the lowest
/// `listindex`.
pub fn default_tile(ui: &mut Ui, menu: TileId) -> Option<TileId> {
    let mut best: Option<(f32, f32, TileId)> = None;
    for tile in ui.descendants(menu) {
        if !ui.has(tile, XDEFAULT) || !ui.shown(tile) || ui.number(tile, t::TARGET) == 0.0 {
            continue;
        }
        let d = ui.number(tile, XDEFAULT);
        let index = if ui.has(tile, t::LISTINDEX) {
            ui.number(tile, t::LISTINDEX)
        } else {
            f32::MAX
        };
        let better = match best {
            None => true,
            Some((bd, bi, _)) => d > bd || (d == bd && index < bi),
        };
        if better {
            best = Some((d, index, tile));
        }
    }
    best.map(|(_, _, tile)| tile)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    /// A menu's code that notes what reaches it.
    #[derive(Default)]
    struct Recorder {
        clicks: Vec<i32>,
        over: Vec<i32>,
        keys: Vec<u32>,
        list: ListBox,
    }

    impl MenuCode for Recorder {
        fn class(&self) -> i32 {
            1
        }
        fn click(&mut self, _ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
            self.clicks.push(id);
        }
        fn mouseover(&mut self, _ui: &mut Ui, id: i32, _tile: TileId) {
            self.over.push(id);
        }
        fn key(&mut self, _ui: &mut Ui, code: u32, _now: f64) -> bool {
            self.keys.push(code);
            code == u32::from(b'q')
        }
        fn lists(&mut self) -> Vec<&mut ListBox> {
            vec![&mut self.list]
        }
    }

    const MENU: &str = "<menu name=\"M\"><class>&MessageMenu;</class><_PCButton_E>Exit</_PCButton_E>
        <xup><ref src=\"Up\" trait=\"clicked\"/></xup>
        <image name=\"Back\"><id>1</id><target>&true;</target><filename>a.dds</filename><width>100</width><height>100</height></image>
        <image name=\"Front\"><id>2</id><target>&true;</target><depth>5</depth><filename>a.dds</filename><width>50</width><height>50</height>
          <image name=\"Inner\"><target>&true;</target><depth>6</depth><filename>a.dds</filename><width>10</width><height>10</height></image></image>
        <image name=\"Ghost\"><id>3</id><depth>9</depth><filename>a.dds</filename><width>100</width><height>100</height></image>
        <image name=\"Exit\"><id>4</id><target>&true;</target><filename>a.dds</filename><x>500</x><width>10</width><height>10</height><xdefault>2</xdefault></image>
        <image name=\"Up\"><id>5</id><target>&true;</target><filename>a.dds</filename><x>600</x><width>10</width><height>10</height></image>
        <image name=\"Hidden\"><id>6</id><target>&true;</target><visible>&false;</visible><filename>a.dds</filename><x>500</x><width>10</width><height>10</height><xdefault>9</xdefault></image>
        </menu>";

    fn setup() -> (Ui, Recorder, TileId) {
        let mut ui = test_support::ui();
        let mut code = Recorder::default();
        let menu = test_support::load(&mut ui, MENU, &mut code);
        ui.set_number(menu, t::VISIBLE, 1.0);
        (ui, code, menu)
    }

    /// `007126c0`: the nearest target under the point; a tile that isn't
    /// a target is passed over; one without an id gives its ancestor's.
    #[test]
    fn picking_the_tile_under_the_pointer() {
        let (mut ui, _, menu) = setup();
        let front = ui.find(menu, "Front").unwrap();
        let back = ui.find(menu, "Back").unwrap();
        // "Inner" (no id) is nearest: its parent "Front" is picked; "Ghost"
        // on top isn't a target.
        assert_eq!(pick(&mut ui, menu, 5.0, 5.0), Some(front));
        assert_eq!(pick(&mut ui, menu, 80.0, 80.0), Some(back));
        assert_eq!(pick(&mut ui, menu, 300.0, 300.0), None);
    }

    /// A menu not shown lets the pointer go without a "mouse off".
    #[test]
    fn letting_go_of_a_menu() {
        let (mut ui, mut code, menu) = setup();
        let mut i = Interface::default();
        i.pointer(
            &mut ui, menu, &mut code, 80.0, 80.0, false, true, false, 0.0,
        );
        assert!(i.over.is_some() && i.held());
        let other = test_support::load(&mut ui, MENU, &mut Recorder::default());
        i.let_go_of(&ui, other);
        assert!(i.over.is_some(), "another menu's tiles stay");
        i.let_go_of(&ui, menu);
        assert_eq!((i.over, i.focus, i.held()), (None, None, false));
        assert_eq!(code.over, vec![1]);
    }

    #[test]
    fn pointer_over_and_clicks() {
        let (mut ui, mut code, menu) = setup();
        let mut i = Interface::default();
        i.pointer(
            &mut ui, menu, &mut code, 80.0, 80.0, false, false, false, 0.0,
        );
        let back = ui.find(menu, "Back").unwrap();
        assert_eq!(ui.number(back, t::MOUSEOVER), 1.0);
        assert_eq!(code.over, [1]);
        // Down and up over the same tile: a click.
        i.pointer(&mut ui, menu, &mut code, 80.0, 80.0, true, true, false, 0.0);
        i.pointer(
            &mut ui, menu, &mut code, 80.0, 80.0, false, false, true, 0.0,
        );
        assert_eq!(code.clicks, [1]);
        // Moving off: mouseover back to 0.
        i.pointer(
            &mut ui, menu, &mut code, 300.0, 300.0, false, false, false, 0.0,
        );
        assert_eq!(ui.number(back, t::MOUSEOVER), 0.0);
        // Down on one, up on another: no click.
        i.pointer(&mut ui, menu, &mut code, 80.0, 80.0, true, true, false, 0.0);
        i.pointer(&mut ui, menu, &mut code, 5.0, 5.0, true, false, false, 0.0);
        i.pointer(&mut ui, menu, &mut code, 5.0, 5.0, false, false, true, 0.0);
        assert_eq!(code.clicks, [1]);
    }

    /// `0070c4a0`: the menu's own key handling first, then
    /// `_PCButton_<key>`, then the arrows and Enter.
    #[test]
    fn keys_reach_the_menu_then_its_buttons() {
        let (mut ui, mut code, menu) = setup();
        let mut i = Interface::default();
        i.key(&mut ui, menu, &mut code, u32::from(b'q'), false, false, 0.0);
        assert!(code.clicks.is_empty());
        i.key(&mut ui, menu, &mut code, u32::from(b'e'), false, false, 0.0);
        assert_eq!(code.clicks, [4]);
        // An arrow with nothing chosen: the shown tile with the highest
        // `xdefault` ("Hidden" isn't shown).
        i.key(&mut ui, menu, &mut code, key::DOWN, false, false, 0.0);
        let exit = ui.find(menu, "Exit").unwrap();
        assert_eq!(i.focus, Some(exit));
        // Up follows the menu's `xup` ref and clicks "Up".
        i.key(&mut ui, menu, &mut code, key::UP, false, false, 0.0);
        assert_eq!(code.clicks, [4, 5]);
        // Enter clicks the chosen tile.
        i.key(&mut ui, menu, &mut code, key::ENTER, false, false, 0.0);
        assert_eq!(code.clicks, [4, 5, 4]);
        assert_eq!(
            special_of(key::ENTER, true, false),
            Some(special::SHIFT_ENTER)
        );
        // A `_PCButton_` naming a tile that can't be used: the cancel sound.
        let exit = ui.find(menu, "Exit").unwrap();
        ui.set_number(exit, t::TARGET, 0.0);
        i.key(&mut ui, menu, &mut code, u32::from(b'e'), false, false, 0.0);
        assert_eq!(code.clicks, [4, 5, 4]);
        assert_eq!(
            i.effects.last(),
            Some(&Effect::Sound("UIMenuCancel".into()))
        );
    }

    /// `00a1d910`/`00a1db20`: fades last `menufade` seconds, `explorefade`
    /// when that is 0; a hidden menu isn't faded out again; `00711ea0`:
    /// visible while fading, off once faded out (a menu not marked to leave
    /// the stack is kept), on once faded in. (The interface's fades,
    /// `crate::fade`, which every menu's open, close and the dialogue
    /// menu's service fade go through.)
    #[test]
    fn menus_fade_out_and_in() {
        use crate::fade::{Ended, Fades, State};
        let mut ui = test_support::ui();
        let mut code = Recorder::default();
        let xml = "<menu name=\"M\"><class>&MessageMenu;</class><menufade>0</menufade><explorefade>0.5</explorefade></menu>";
        let menu = test_support::load(&mut ui, xml, &mut code);
        let mut fades = Fades::default();
        // Hidden (menus start so): nothing to fade.
        assert!(!fades.start_fade_out(&mut ui, menu, 0.0));
        assert!(!fades.knows(menu));
        ui.set_number(menu, t::VISIBLE, 1.0);
        assert!(fades.start_fade_out(&mut ui, menu, 0.0));
        assert_eq!(fades.state(menu), State::FadingOut);
        fades.update(0.25);
        fades.frame(&mut ui, &[menu]);
        assert_eq!(fades.value(menu), 0.5);
        assert_eq!(ui.number(menu, t::VISIBLE), 1.0);
        fades.update(0.25);
        assert_eq!(fades.frame(&mut ui, &[menu]), vec![(menu, Ended::Hidden)]);
        assert_eq!(fades.value(menu), 0.0);
        assert_eq!(fades.state(menu), State::Hidden);
        assert_eq!(ui.number(menu, t::VISIBLE), 0.0);
        fades.update(1.0);
        assert!(fades.frame(&mut ui, &[menu]).is_empty());
        assert_eq!(fades.value(menu), 0.0);
        fades.start_fade_in(&mut ui, menu, 0.125);
        fades.frame(&mut ui, &[menu]);
        assert_eq!(fades.value(menu), 0.25);
        assert_eq!(ui.number(menu, t::VISIBLE), 1.0);
        fades.update(1.0);
        fades.frame(&mut ui, &[menu]);
        assert_eq!(fades.value(menu), 1.0);
        assert_eq!(fades.state(menu), State::Shown);
    }

    /// `0070c4a0`: pressing on a `draggable` tile drags it: `dragx`/`dragy`
    /// follow the pointer in the tile's frame, `dragstartx`/`y` and
    /// `dragoffsetx`/`y` from the press, `dragdeltax`/`y` the whole units
    /// moved each frame; letting go sets the starts to -1.
    #[test]
    fn dragging_a_tile() {
        let mut ui = test_support::ui();
        let mut code = Recorder::default();
        let xml = "<menu name=\"M\"><class>&MessageMenu;</class>
            <rect name=\"Frame\"><x>100</x><y>50</y><locus>&true;</locus>
              <image name=\"Knob\"><id>1</id><target>&true;</target><draggable>&true;</draggable>
                <filename>a.dds</filename><x>20</x><y>10</y><width>40</width><height>40</height></image></rect></menu>";
        let menu = test_support::load(&mut ui, xml, &mut code);
        ui.set_number(menu, t::VISIBLE, 1.0);
        let knob = ui.find(menu, "Knob").unwrap();
        let mut i = Interface::default();
        // The knob is on screen at (120, 60).
        i.pointer(
            &mut ui, menu, &mut code, 130.0, 70.0, false, false, false, 0.0,
        );
        i.pointer(
            &mut ui, menu, &mut code, 130.5, 70.0, true, true, false, 0.0,
        );
        assert_eq!(i.dragging, Some(knob));
        assert_eq!(ui.number(knob, drag::X), 30.5);
        assert_eq!(ui.number(knob, drag::Y), 20.0);
        assert_eq!(ui.number(knob, drag::START_X), 130.0);
        assert_eq!(ui.number(knob, drag::OFFSET_X), 10.5);
        assert_eq!(ui.number(knob, drag::DELTA_X), 0.0);
        // Held and moved: dragx follows, the delta is whole units moved.
        i.pointer(
            &mut ui, menu, &mut code, 145.75, 72.0, true, false, false, 0.0,
        );
        assert_eq!(ui.number(knob, drag::X), 45.75);
        assert_eq!(ui.number(knob, drag::DELTA_X), 15.0);
        assert_eq!(ui.number(knob, drag::DELTA_Y), 2.0);
        i.pointer(
            &mut ui, menu, &mut code, 147.0, 72.0, true, false, false, 0.0,
        );
        assert_eq!(ui.number(knob, drag::DELTA_X), 2.0);
        // Let go: no longer dragged, starts -1, dragx kept.
        i.pointer(
            &mut ui, menu, &mut code, 147.0, 72.0, false, false, true, 0.0,
        );
        assert_eq!(i.dragging, None);
        assert_eq!(ui.number(knob, drag::START_X), -1.0);
        assert_eq!(ui.number(knob, drag::X), 47.0);
    }

    /// The DATA maps' `x` adds `dragdeltax` (`map_menu.xml`) and stays in
    /// its limits: a drag moves the map by what the pointer moved, once,
    /// and nothing moves it after (the delta stays set but nothing sets it
    /// again, as in the game, which works `x` out when a source is set).
    #[test]
    fn a_drag_moves_a_map_once() {
        let mut ui = test_support::ui();
        let mut code = Recorder::default();
        let xml = "<menu name=\"M\"><class>&MessageMenu;</class>
            <hotrect name=\"Win\"><locus>&true;</locus><clipwindow>&true;</clipwindow><width>855</width><height>500</height>
              <hotrect name=\"Map\"><id>2</id><target>&true;</target><draggable>&true;</draggable><locus>&true;</locus>
                <width>4608</width><height>3000</height>
                <x><add src=\"me()\" trait=\"dragdeltax\"/>
                  <max><copy>427.5</copy><sub src=\"me()\" trait=\"width\"/></max><min>427.5</min></x>
                <y><add src=\"me()\" trait=\"dragdeltay\"/>
                  <max><copy>250</copy><sub src=\"me()\" trait=\"height\"/></max><min>250</min></y>
              </hotrect></hotrect></menu>";
        let menu = test_support::load(&mut ui, xml, &mut code);
        ui.set_number(menu, t::VISIBLE, 1.0);
        let map = ui.find(menu, "Map").unwrap();
        ui.set_base(map, t::X, -1000.0);
        ui.set_base(map, t::Y, -500.0);
        ui.refresh();
        assert_eq!(ui.number(map, t::X), -1000.0);
        let mut i = Interface::default();
        let at =
            |ui: &mut Ui, i: &mut Interface, code: &mut Recorder, x, down, pressed, released| {
                i.pointer(ui, menu, code, x, 200.0, down, pressed, released, 0.0);
            };
        at(&mut ui, &mut i, &mut code, 300.0, false, false, false);
        at(&mut ui, &mut i, &mut code, 300.0, true, true, false);
        at(&mut ui, &mut i, &mut code, 290.0, true, false, false);
        at(&mut ui, &mut i, &mut code, 270.0, true, false, false);
        assert_eq!(ui.number(map, t::X), -1030.0);
        // Many passes with the same delta still set add nothing.
        for _ in 0..5 {
            ui.refresh();
            assert_eq!(ui.number(map, t::X), -1030.0);
        }
        // The same delta set again (the pointer moved as much again) is a
        // move again.
        at(&mut ui, &mut i, &mut code, 250.0, true, false, false);
        assert_eq!(ui.number(map, t::X), -1050.0);
        at(&mut ui, &mut i, &mut code, 250.0, false, false, true);
        for _ in 0..5 {
            ui.refresh();
            assert_eq!(ui.number(map, t::X), -1050.0);
        }
        // The limits hold: dragged far right, the map's left edge stops at
        // the window's middle.
        at(&mut ui, &mut i, &mut code, 100.0, false, false, false);
        at(&mut ui, &mut i, &mut code, 100.0, true, true, false);
        at(&mut ui, &mut i, &mut code, 3000.0, true, false, false);
        assert_eq!(ui.number(map, t::X), 427.5);
        at(&mut ui, &mut i, &mut code, 3000.0, false, false, true);
        assert_eq!(ui.number(map, t::X), 427.5);
    }

    #[test]
    fn clip_windows_and_clipping_children() {
        let mut ui = test_support::ui();
        let mut code = Recorder::default();
        let menu = test_support::load(
            &mut ui,
            "<menu name=\"M\"><class>&MessageMenu;</class>
               <hotrect name=\"Win\"><locus>&true;</locus><clipwindow>&true;</clipwindow><x>10</x><y>20</y><width>100</width><height>50</height>
                 <hotrect name=\"Item\"><clips>&true;</clips><text name=\"Label\"></text></hotrect>
                 <rect name=\"Plain\"><text name=\"Deep\"></text></rect></hotrect></menu>",
            &mut code,
        );
        let label = ui.find(menu, "Label").unwrap();
        assert_eq!(clip_rect(&mut ui, label), Some([10.0, 20.0, 100.0, 50.0]));
        let deep = ui.find(menu, "Deep").unwrap();
        assert_eq!(clip_rect(&mut ui, deep), None);
    }
}

#[cfg(test)]
mod radial_tests {
    use super::*;

    /// `00a216b0`: up and right is 0..π/2, right π/2, down π, left 3π/2.
    #[test]
    fn slice_angles() {
        let close = |a: f32, b: f32| (a - b).abs() < 1e-3;
        assert!(close(slice_angle(0.0, -10.0), 0.0));
        assert!(close(slice_angle(10.0, -10.0), std::f32::consts::FRAC_PI_4));
        assert!(close(slice_angle(10.0, 0.0), std::f32::consts::FRAC_PI_2));
        assert!(close(slice_angle(0.0, 10.0), std::f32::consts::PI));
        assert!(close(
            slice_angle(-10.0, 0.0),
            3.0 * std::f32::consts::FRAC_PI_2
        ));
        assert!(close(
            slice_angle(-10.0, -10.0),
            7.0 * std::f32::consts::FRAC_PI_4
        ));
        // The wheel's first slice (30 to 60 degrees) holds its icon's
        // middle: (557, 250) around (390, 390).
        let a = slice_angle(557.0 - 390.0, 250.0 - 390.0);
        let (from, to) = (std::f32::consts::FRAC_PI_6, std::f32::consts::FRAC_PI_3);
        assert!((from..=to).contains(&a));
    }
}
