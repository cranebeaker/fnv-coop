//! The tile tree: menus read from their files, templates instanced, and
//! every trait worked out the way FalloutNV.exe works them out (`00a09410`
//! for a trait's operators, `00a033b0` for building, `00a08b20` for `src`).
//!
//! The game updates traits when what they read changes; here a trait is
//! worked out when asked, from the value it had (as the game's operators
//! run on the current value, not from zero), once per [`Ui::refresh`].
//! For chains that start with `copy` (all of the game's) that comes to the
//! same.

use std::collections::{BTreeMap, HashMap};

use crate::font::{extra_line_gap, Font};
use crate::names::{kind, op, sel, t, Names, NOT_FOUND};
use crate::text;
use crate::xml::{self, token, Token};

/// A tile, by its place in [`Ui`]'s list.
pub type TileId = usize;

/// A trait's value: a number, and text when it has some.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Value {
    pub number: f32,
    pub string: Option<String>,
}

/// What an operator works with.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Constant(f32),
    /// Another tile's trait (`src`, `trait`).
    Link {
        tile: TileId,
        trait_id: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub op: i32,
    pub operand: Operand,
}

/// One trait: its value (as set, then as last worked out) and its
/// operators.
#[derive(Debug, Clone, Default)]
pub struct Trait {
    pub value: Value,
    pub actions: Vec<Action>,
    worked_out: u32,
    busy: bool,
    /// How many times code has set it (`00a012d0`): what a trait that
    /// adds the mouse's drag is fed by.
    sets: u32,
    /// For a trait that adds a drag delta: what it was last worked out
    /// from (see [`Ui::value`]).
    fed: Option<Vec<(u32, u32)>>,
}

/// A text tile's laid-out text.
#[derive(Debug, Clone)]
pub struct TextLayout {
    pub font: usize,
    pub quads: Vec<text::GlyphQuad>,
    /// Each quad's font (from 1) when they differ (HTML text); empty: all
    /// in `font`.
    pub quad_fonts: Vec<usize>,
    /// Pictures in HTML text.
    pub pictures: Vec<crate::html::Picture>,
    /// HTML text's number of pages (`pagecount`); 0 for plain text.
    pub pages: i32,
    pub width: i32,
    pub height: i32,
}

impl TextLayout {
    /// The font of quad `i` (from 1).
    pub fn quad_font(&self, i: usize) -> usize {
        self.quad_fonts.get(i).copied().unwrap_or(self.font)
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    /// `kind::RECT` .. `kind::RADIAL`.
    pub kind: i32,
    pub name: String,
    pub parent: Option<TileId>,
    pub children: Vec<TileId>,
    pub traits: BTreeMap<i32, Trait>,
    layout: Option<(u32, TextLayout)>,
    /// The tile's "failed check" flag (`+0x35`, `0071aa60`): a dialogue
    /// topic whose skill check the player fails, copied onto the list's
    /// highlight box while that topic is chosen (`00764ef0`).
    pub failed: bool,
}

/// The screen the menus are drawn on, in pixels, and the safe zone (the
/// `iSafeZoneX`/`Y` INI settings, `iSafeZoneXWide`/`YWide` when the screen
/// is wide).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Screen {
    pub width_px: u32,
    pub height_px: u32,
    pub safe_x: f32,
    pub safe_y: f32,
}

impl Screen {
    /// The screen tile's width in menu units (`00715d40`): (pixel width /
    /// pixel height) x 960 when wider than tall, else 1280.
    pub fn width(&self) -> f32 {
        let (w, h) = (self.width_px as f32, self.height_px as f32);
        if h < w {
            (w / h) * 960.0
        } else {
            1280.0
        }
    }

    /// Its height (`00715da0`): 960, or (h / w) x 1280 when taller than
    /// wide.
    pub fn height(&self) -> f32 {
        let (w, h) = (self.width_px as f32, self.height_px as f32);
        if w < h {
            (h / w) * 1280.0
        } else {
            960.0
        }
    }

    /// Menu units per pixel (`resolutionconverter`, `00707b60`): the menu
    /// height over the pixel height.
    pub fn resolution_converter(&self) -> f32 {
        self.height() / self.height_px as f32
    }
}

/// A system colour (`&hudmain;` and the others), red, green, blue from 0
/// to 1 (`00718c90`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SystemColors(pub [[f32; 3]; 7]);

impl SystemColors {
    /// The engine's colours, with the HUD's and the Pip-Boy's from the
    /// `uHUDColor` / `uPipboyColor` settings (packed R<<24 | G<<16 | B<<8 |
    /// A, `00718e90`; 0 or missing gives 255, 182, 66).
    pub fn new(hud: Option<u32>, pipboy: Option<u32>) -> SystemColors {
        let default = [1.0, 182.0 / 255.0, 66.0 / 255.0];
        let unpack = |v: Option<u32>| match v {
            Some(v) if v != 0 => [
                ((v >> 24) & 0xff) as f32 / 255.0,
                ((v >> 16) & 0xff) as f32 / 255.0,
                ((v >> 8) & 0xff) as f32 / 255.0,
            ],
            _ => default,
        };
        SystemColors([
            [1.0, 1.0, 1.0],
            unpack(hud),
            [1.0, 67.0 / 255.0, 42.0 / 255.0],
            [33.0 / 255.0, 231.0 / 255.0, 121.0 / 255.0],
            unpack(pipboy),
            default,
            default,
        ])
    }

    /// A colour by number (1 to 6); 0 and others aren't registered.
    pub fn get(&self, index: i32) -> Option<[f32; 3]> {
        (1..=6).contains(&index).then(|| self.0[index as usize])
    }
}

/// Text settings for `&-sName;` (by name, case ignored). Shareable between
/// threads, so a [`Ui`] can live in a game engine's world.
pub type SettingText = Box<dyn Fn(&str) -> Option<String> + Send + Sync>;

/// All menus, the screen and the globals.
pub struct Ui {
    pub names: Names,
    pub tiles: Vec<Tile>,
    /// The screen tile ("MenuRoot"), every menu's parent.
    pub screen: TileId,
    /// The tile `globals.xml` made (`globals()`).
    pub globals: Option<TileId>,
    pub screen_size: Screen,
    pub colors: SystemColors,
    templates: HashMap<TileId, Vec<(String, Vec<Token>)>>,
    /// Fonts 1 to 8 (index 0 is font 1).
    pub fonts: Vec<Option<Font>>,
    setting: SettingText,
    pass: u32,
    pub warnings: Vec<String>,
}

impl Ui {
    /// The screen tile (`0070adb0`): locus, alpha 0, its size, the safe
    /// zone as `cropx`/`cropy` and `resolutionconverter`.
    pub fn new(screen: Screen, colors: SystemColors, setting: SettingText) -> Ui {
        let widescreen = screen.width_px * 3 > screen.height_px * 4;
        let mut ui = Ui {
            names: Names::new(widescreen),
            tiles: Vec::new(),
            screen: 0,
            globals: None,
            screen_size: screen,
            colors,
            templates: HashMap::new(),
            fonts: vec![None; 8],
            setting,
            pass: 1,
            warnings: Vec::new(),
        };
        let root = ui.add_tile(kind::RECT, "MenuRoot", None);
        ui.screen = root;
        ui.set_number(root, t::LOCUS, 1.0);
        ui.set_number(root, t::MENU_CLASS_INTERNAL, 1003.0);
        ui.set_number(root, t::ALPHA, 0.0);
        ui.set_number(root, t::WIDTH, screen.width());
        ui.set_number(root, t::HEIGHT, screen.height());
        ui.set_number(root, t::CROPX, screen.safe_x);
        ui.set_number(root, t::CROPY, screen.safe_y);
        ui.set_number(root, t::RESOLUTIONCONVERTER, screen.resolution_converter());
        ui
    }

    /// A game setting's text, as `&-sName;` reads it.
    pub fn setting_text(&self, name: &str) -> Option<String> {
        (self.setting)(name)
    }

    fn add_tile(&mut self, kind: i32, name: &str, parent: Option<TileId>) -> TileId {
        let id = self.tiles.len();
        self.tiles.push(Tile {
            kind,
            name: name.to_string(),
            parent,
            children: Vec::new(),
            traits: BTreeMap::new(),
            layout: None,
            failed: false,
        });
        if let Some(p) = parent {
            self.tiles[p].children.push(id);
        }
        // Each type's own starting traits (its constructor).
        match kind {
            // `00a1f370`.
            kind::RECT => self.set_number(id, t::VISIBLE, 1.0),
            // `00a1ef30`: menus start at 0, 0 with locus, a menu fade of
            // 0.25, the HUD's system colour (1) and hidden.
            kind::MENU => {
                self.set_number(id, t::X, 0.0);
                self.set_number(id, t::Y, 0.0);
                self.set_number(id, t::LOCUS, 1.0);
                self.set_number(id, t::MENUFADE, 0.25);
                self.set_number(id, t::SYSTEMCOLOR, 1.0);
                self.set_number(id, t::VISIBLE, 0.0);
            }
            // `00a1f6e0` (a radial tile's too: `RadialTile` is an image
            // with its own picking).
            kind::IMAGE | kind::HOTRECT | kind::RADIAL => {
                for trait_id in [t::BRIGHTNESS, t::RED, t::GREEN, t::BLUE, t::ALPHA] {
                    self.set_number(id, trait_id, 255.0);
                }
                self.set_number(id, t::DEPTH, 1.0);
                self.set_number(id, t::VISIBLE, 1.0);
            }
            // `00a21a10`.
            kind::TEXT => {
                self.set_string(id, t::STRING, "");
                for trait_id in [t::BRIGHTNESS, t::RED, t::GREEN, t::BLUE, t::ALPHA] {
                    self.set_number(id, trait_id, 255.0);
                }
                self.set_number(id, t::VISIBLE, 1.0);
            }
            _ => {}
        }
        if kind == kind::HOTRECT {
            // `00a031b0`: hot rectangles are an invisible solid picture.
            self.set_number(id, t::ID, -1.0);
            self.set_number(id, t::TARGET, 1.0);
            self.set_number(id, t::BRIGHTNESS, -1.0);
            self.set_number(id, t::ALPHA, 0.0);
            self.set_string(id, t::FILENAME, "solid.dds");
            self.set_string(id, t::TEXATLAS, "Interface\\InterfaceShared.tai");
        }
        id
    }

    /// Reads `globals.xml` (its `<rect name="Strings">` becomes
    /// `globals()`).
    pub fn load_globals(&mut self, text: &[u8]) -> Result<TileId, String> {
        let id = self.load_file(text, &mut |_| None)?;
        self.globals = Some(id);
        Ok(id)
    }

    /// Reads a menu file (`menus\...xml`), pasting in its prefabs (`read`
    /// gets a file under `Data`), and puts it on the screen. Returns its
    /// top tile.
    pub fn load_menu(
        &mut self,
        text: &[u8],
        read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
    ) -> Result<TileId, String> {
        self.load_file(text, read)
    }

    fn load_file(
        &mut self,
        text: &[u8],
        read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
    ) -> Result<TileId, String> {
        let text = xml::expand_includes(&xml::clean(text), read);
        let setting = &self.setting;
        let parsed = xml::tokenize(&text, &mut self.names, &|s| {
            let name = s
                .trim_start_matches('&')
                .trim_start_matches('-')
                .trim_end_matches(';');
            setting(name)
        })?;
        self.warnings.extend(parsed.warnings);
        let screen = self.screen;
        let top = self
            .build(screen, &parsed.tokens)
            .ok_or_else(|| "the file makes no tile".to_string())?;
        self.templates.insert(top, parsed.templates);
        self.refresh();
        Ok(top)
    }

    /// Makes the tiles of a token list under `parent` (`00a031b0`), then
    /// applies their traits (`00a033b0`). Returns the first tile made.
    fn build(&mut self, parent: TileId, tokens: &[Token]) -> Option<TileId> {
        let mut made: Vec<Option<TileId>> = vec![None; tokens.len()];
        let mut current = parent;
        let mut first = None;
        for (i, tok) in tokens.iter().enumerate() {
            match tok.kind {
                token::TILE_START => {
                    let id = self.add_tile(tok.value as i32, &tok.text, Some(current));
                    made[i] = Some(id);
                    first.get_or_insert(id);
                    current = id;
                }
                token::TILE_END => {
                    current = self.tiles[current].parent.unwrap_or(parent);
                }
                _ => {}
            }
        }
        let mut tile: Option<TileId> = None;
        let mut open_trait: Option<i32> = None;
        for (i, tok) in tokens.iter().enumerate() {
            match tok.kind {
                token::TILE_START => tile = made[i],
                token::TILE_END => {
                    tile = tile.and_then(|c| self.tiles[c].parent);
                }
                token::TRAIT_OPEN => open_trait = Some(tok.id),
                token::TRAIT_CLOSE => open_trait = None,
                token::TRAIT_VALUE => {
                    let Some(c) = tile else {
                        self.warnings
                            .push("MENUS: Trait defined outside of any tile.".into());
                        continue;
                    };
                    let text_value = (tok.value == NOT_FOUND && !tok.text.is_empty())
                        || tok.id == t::STRING
                        || tok.id == t::FILENAME;
                    if tok.id == crate::names::attr::NAME {
                        self.tiles[c].name = tok.text.clone();
                    } else if text_value {
                        self.set_string(c, tok.id, &tok.text);
                    } else {
                        self.set_number(c, tok.id, tok.value);
                    }
                }
                token::ACTION_CONST | token::ACTION_BEGIN | token::ACTION_END => {
                    let (Some(c), Some(trait_id)) = (tile, open_trait) else {
                        self.warnings
                            .push("MENUS: Action defined outside of any trait.".into());
                        continue;
                    };
                    let action = match tok.kind {
                        token::ACTION_CONST => Action {
                            op: tok.id,
                            operand: Operand::Constant(tok.value),
                        },
                        token::ACTION_BEGIN => Action {
                            op: op::GROUP_BEGIN,
                            operand: Operand::Constant(tok.value.round()),
                        },
                        _ => Action {
                            op: op::GROUP_END,
                            operand: Operand::Constant(tok.value.round()),
                        },
                    };
                    self.trait_mut(c, trait_id).actions.push(action);
                }
                token::ACTION_LINK => {
                    let (Some(c), Some(trait_id)) = (tile, open_trait) else {
                        self.warnings
                            .push("MENUS: Action link defined outside of any trait.".into());
                        continue;
                    };
                    // `src` is found once, now (`00a08b20`); a source that
                    // isn't there drops the operator.
                    if let Some(source) = self.select(c, &tok.text) {
                        self.trait_mut(c, trait_id).actions.push(Action {
                            op: tok.id,
                            operand: Operand::Link {
                                tile: source,
                                trait_id: tok.value as i32,
                            },
                        });
                    }
                }
                _ => {}
            }
        }
        first
    }

    fn trait_mut(&mut self, tile: TileId, trait_id: i32) -> &mut Trait {
        self.tiles[tile].traits.entry(trait_id).or_default()
    }

    /// Instances a template of `menu` under `parent` (`00a1ddb0`; the name
    /// as written, case counts). Returns the template's top tile.
    pub fn instantiate(&mut self, menu: TileId, parent: TileId, name: &str) -> Option<TileId> {
        // 00a1ddb0 walks the menu's templates comparing names (`strcmp`)
        // and, finding none, goes on with the last one it looked at: an
        // unknown name makes the last template. The barter menu's code
        // asks for `CM_list_template`; its file has only
        // `BM_list_template`. (The order of the game's template list isn't
        // traced; the file's order is taken.)
        let templates = self.templates.get(&menu)?;
        let tokens = templates
            .iter()
            .find(|(n, _)| n == name)
            .or_else(|| templates.last())
            .map(|(_, tokens)| tokens.clone())?;
        let made = self.build(parent, &tokens);
        self.refresh();
        made
    }

    /// Takes a tile and everything under it out of its menu (the game
    /// deletes the tiles; list boxes empty themselves this way before
    /// they're filled again). Its slot stays, so a link still pointing at
    /// it reads its last values until the code that set the link resets it.
    pub fn remove(&mut self, tile: TileId) {
        if let Some(p) = self.tiles[tile].parent.take() {
            self.tiles[p].children.retain(|&c| c != tile);
        }
        self.pass += 1;
    }

    /// The menu a tile belongs to: its ancestor just below the screen
    /// (`00a03d40`).
    pub fn menu_of(&self, tile: TileId) -> TileId {
        let mut t = tile;
        while let Some(p) = self.tiles[t].parent {
            if self.tiles[p].parent.is_none() {
                break;
            }
            t = p;
        }
        t
    }

    /// A tile by name, searched depth first from `from` itself, case
    /// ignored (`00a08f20`).
    pub fn find_below(&self, from: TileId, name: &str) -> Option<TileId> {
        if self.tiles[from].name.eq_ignore_ascii_case(name) {
            return Some(from);
        }
        self.tiles[from]
            .children
            .iter()
            .find_map(|&c| self.find_below(c, name))
    }

    /// A tile named anywhere in `from`'s menu (`00a08b20` with a plain
    /// name).
    pub fn find(&self, from: TileId, name: &str) -> Option<TileId> {
        self.select(from, name)
    }

    /// What a `src` names, seen from `from` (`00a08b20`, `00a018d0`):
    /// `me()`, `parent()`, `grandparent()`, `screen()`, `globals()`,
    /// `sibling(N)` (the parent's child named N, case ignored; `sibling()`
    /// alone the child just before it), `child(N)` (the first descendant
    /// named N, depth first, case counts), `io()` (the tile's menu: the
    /// game's `00a03c90` goes from the tile to its menu tile and on to the
    /// menu object behind it, whose traits the menu files read as the menu
    /// tile's `user` traits, which the code fills), or a tile's name,
    /// searched through the menu.
    pub fn select(&self, from: TileId, src: &str) -> Option<TileId> {
        let (word, arg) = match src.find('(') {
            Some(open) => {
                let close = src[open..].find(')').map_or(src.len(), |c| open + c);
                (&src[..open], &src[open + 1..close])
            }
            None => (src, ""),
        };
        match self.names.lookup(word) {
            Some(sel::ME) => Some(from),
            Some(sel::PARENT) => self.tiles[from].parent,
            Some(sel::GRANDPARENT) => self.tiles[from].parent.and_then(|p| self.tiles[p].parent),
            Some(sel::SCREEN) => Some(self.screen),
            Some(sel::GLOBALS) => self.globals,
            Some(sel::SIBLING) => {
                let parent = self.tiles[from].parent?;
                let children = &self.tiles[parent].children;
                if arg.is_empty() {
                    // The one before (`stats_no_effects` reads the effects
                    // list just before it this way).
                    let at = children.iter().position(|&c| c == from)?;
                    return at.checked_sub(1).map(|i| children[i]);
                }
                children
                    .iter()
                    .copied()
                    .find(|&c| self.tiles[c].name.eq_ignore_ascii_case(arg))
            }
            Some(sel::CHILD) => {
                if arg.is_empty() {
                    return None;
                }
                self.child_named(from, arg)
            }
            Some(sel::IO) => self.io(from),
            _ => {
                let menu = self.menu_of(from);
                self.find_below(menu, src)
            }
        }
    }

    /// `io()` (`00a08b20` case 5008, read from the disassembly): the menu
    /// object of the tile's menu (`00a03c90`: the menu tile's `+0x3c`, made
    /// when its `class` trait is set, `00a1f160`), and from that its own
    /// tile (the object's `+0x4`). So: the menu tile, once it has a class.
    pub fn io(&self, from: TileId) -> Option<TileId> {
        let menu = self.menu_of(from);
        (self.tiles[menu].kind == kind::MENU && self.has(menu, t::CLASS)).then_some(menu)
    }

    /// Makes a trait copy another tile's trait, as the menus' code does
    /// (`00a00f30` clears the trait's operators, `00a09130` adds a `copy`
    /// of the source, `00a09410` works it out): the list box's highlight
    /// following the chosen item (`00764ef0`).
    pub fn link(&mut self, tile: TileId, trait_id: i32, source: TileId, source_trait: i32) {
        let tr = self.trait_mut(tile, trait_id);
        tr.actions.clear();
        tr.actions.push(Action {
            op: op::COPY,
            operand: Operand::Link {
                tile: source,
                trait_id: source_trait,
            },
        });
        self.pass += 1;
    }

    /// Takes a tile (and everything under it) off the screen: a menu
    /// closed. Its slots stay in the list (ids don't move) but nothing
    /// reaches it any more.
    pub fn detach(&mut self, tile: TileId) {
        if let Some(p) = self.tiles[tile].parent.take() {
            self.tiles[p].children.retain(|&c| c != tile);
        }
        self.templates.remove(&tile);
        self.pass += 1;
    }

    /// Whether `tile` is `top` or under it (not taken away).
    pub fn is_under(&self, tile: TileId, top: TileId) -> bool {
        let mut at = Some(tile);
        while let Some(t) = at {
            if t == top {
                return true;
            }
            at = self.tiles.get(t).and_then(|x| x.parent);
        }
        false
    }

    /// Every tile under `top` (itself first), depth first in file order.
    pub fn descendants(&self, top: TileId) -> Vec<TileId> {
        let mut out = Vec::new();
        let mut stack = vec![top];
        while let Some(tile) = stack.pop() {
            out.push(tile);
            for &c in self.tiles[tile].children.iter().rev() {
                stack.push(c);
            }
        }
        out
    }

    /// Works out every trait of every tile under `top` once, as the game's
    /// eager updating does after a value changes (for `clicked` going to 1
    /// and back to 0 in the same frame, which the scroll bars add up).
    pub fn work_out_all(&mut self, top: TileId) {
        self.pass += 1;
        for tile in self.descendants(top) {
            let ids: Vec<i32> = self.tiles[tile].traits.keys().copied().collect();
            for id in ids {
                self.value(tile, id);
            }
        }
    }

    fn child_named(&self, from: TileId, name: &str) -> Option<TileId> {
        for &c in &self.tiles[from].children {
            if self.tiles[c].name == name {
                return Some(c);
            }
            if let Some(found) = self.child_named(c, name) {
                return Some(found);
            }
        }
        None
    }

    /// Sets a trait's number from code (`00a012d0` with its refresh flag):
    /// its operators are dropped and its text cleared.
    pub fn set_number(&mut self, tile: TileId, trait_id: i32, value: f32) {
        let tr = self.trait_mut(tile, trait_id);
        tr.sets = tr.sets.wrapping_add(1);
        tr.actions.clear();
        tr.value = Value {
            number: value,
            string: None,
        };
        self.pass += 1;
    }

    /// Sets a trait's text from code (`00a01350`): operators dropped, the
    /// number 0.
    pub fn set_string(&mut self, tile: TileId, trait_id: i32, value: &str) {
        let tr = self.trait_mut(tile, trait_id);
        tr.actions.clear();
        tr.value = Value {
            number: 0.0,
            string: Some(value.to_string()),
        };
        self.pass += 1;
    }

    /// Sets the value a trait's operators start from, keeping them (the
    /// game's setter without its refresh flag, `00a012d0(.., 0)`: the DATA
    /// menu moves its maps this way while their `x` keeps adding the
    /// mouse's drag and staying inside the window).
    pub fn set_base(&mut self, tile: TileId, trait_id: i32, value: f32) {
        let tr = self.trait_mut(tile, trait_id);
        tr.value = Value {
            number: value,
            string: None,
        };
        tr.worked_out = 0;
        tr.fed = None;
        self.pass += 1;
    }

    /// Adds an operator to a trait from code, after the ones it has
    /// (`00a09130` with another tile's trait, `00a09080` with a constant;
    /// list boxes link their highlight to the chosen row this way).
    pub fn add_action(&mut self, tile: TileId, trait_id: i32, op: i32, operand: Operand) {
        self.trait_mut(tile, trait_id)
            .actions
            .push(Action { op, operand });
        self.pass += 1;
    }

    /// Sets a trait's text from code as the game passes an empty string:
    /// its strings hold nothing at all when empty (`004037f0` frees the
    /// buffer), so the trait gets no text (`00a0a300` with a null pointer),
    /// which `onlyif` and the like then read as 0.
    pub fn set_text(&mut self, tile: TileId, trait_id: i32, value: &str) {
        if value.is_empty() {
            let tr = self.trait_mut(tile, trait_id);
            tr.actions.clear();
            tr.value = Value {
                number: 0.0,
                string: None,
            };
            self.pass += 1;
        } else {
            self.set_string(tile, trait_id, value);
        }
    }

    /// Starts a new round of working traits out (after inputs change).
    pub fn refresh(&mut self) {
        self.pass += 1;
    }

    /// Whether a tile has a trait at all.
    pub fn has(&self, tile: TileId, trait_id: i32) -> bool {
        self.tiles[tile].traits.contains_key(&trait_id)
    }

    /// A trait's number now (0 when the tile hasn't got it, as the game's
    /// getter gives).
    pub fn number(&mut self, tile: TileId, trait_id: i32) -> f32 {
        self.value(tile, trait_id).number
    }

    /// A trait's text now.
    pub fn string(&mut self, tile: TileId, trait_id: i32) -> Option<String> {
        self.value(tile, trait_id).string
    }

    /// A trait's value now: worked out from its operators (`00a09410`).
    /// A text tile's width and height are its laid-out text's
    /// (`00a21af0` writes them over whatever they were).
    pub fn value(&mut self, tile: TileId, trait_id: i32) -> Value {
        // How many children a tile has: the engine keeps it as tiles come
        // and go (the stats menu's "no effects" text shows when the effects
        // list holds only its scrollbar and highlight box, 2).
        if trait_id == t::CHILDCOUNT {
            return Value {
                number: self.tiles[tile].children.len() as f32,
                string: None,
            };
        }
        // An HTML text tile's `pagecount` is its layout's too (`00a21af0`).
        if self.tiles[tile].kind == kind::TEXT
            && (trait_id == t::WIDTH
                || trait_id == t::HEIGHT
                || (trait_id == t::PAGECOUNT && self.number(tile, t::ISHTML) > 0.0))
        {
            if let Some(layout) = self.layout(tile) {
                let v = match trait_id {
                    t::WIDTH => layout.width,
                    t::HEIGHT => layout.height,
                    _ => layout.pages,
                };
                return Value {
                    number: v as f32,
                    string: None,
                };
            }
        }
        let pass = self.pass;
        let Some(tr) = self.tiles[tile].traits.get_mut(&trait_id) else {
            return Value::default();
        };
        if tr.actions.is_empty() || tr.worked_out == pass || tr.busy {
            return tr.value.clone();
        }
        tr.busy = true;
        let actions = tr.actions.clone();
        // A trait that adds the mouse's drag (`x`, `y` of the DATA maps)
        // is worked out when what it reads is set or changes, once for each
        // time (the game updates a trait when a source is set, `00a012d0`),
        // not again on every pass: else the last drag's movement is added
        // again and again and the map runs away to the edge of its limits.
        let mut fed = None;
        let sources: Vec<(TileId, i32)> = actions
            .iter()
            .filter_map(|a| match a.operand {
                Operand::Link { tile, trait_id } => Some((tile, trait_id)),
                _ => None,
            })
            .collect();
        if sources
            .iter()
            .any(|&(_, id)| id == crate::menu::drag::DELTA_X || id == crate::menu::drag::DELTA_Y)
        {
            let signature: Vec<(u32, u32)> = sources
                .iter()
                .map(|&(source, id)| {
                    let number = self.value(source, id).number;
                    let sets = self.tiles[source].traits.get(&id).map_or(0, |t| t.sets);
                    (sets, number.to_bits())
                })
                .collect();
            let tr = self.trait_mut(tile, trait_id);
            if tr.fed.as_ref() == Some(&signature) {
                tr.worked_out = pass;
                tr.busy = false;
                return tr.value.clone();
            }
            fed = Some(signature);
        }
        let tr = self.trait_mut(tile, trait_id);
        let before = tr.value.clone();
        let mut value = before.clone();
        let mut stack: Vec<f32> = Vec::new();
        for action in &actions {
            let (mut operand, mut text): (f32, Option<String>) = match &action.operand {
                Operand::Constant(v) => (*v, None),
                Operand::Link {
                    tile: source,
                    trait_id: read,
                } => {
                    // A trait named ending `_` reads `<name><n>`, n this
                    // trait's value rounded (`00a0a0b0`).
                    let read = if self.names.is_indexed(*read) {
                        let base = self
                            .names
                            .custom_name(*read)
                            .unwrap_or_default()
                            .to_string();
                        let index = value.number.round() as i32;
                        self.names
                            .lookup(&format!("{base}{index}"))
                            .unwrap_or(NOT_FOUND as i32)
                    } else {
                        *read
                    };
                    let v = self.value(*source, read);
                    (v.number, v.string)
                }
            };
            let mut code = action.op;
            if code == op::GROUP_BEGIN {
                stack.push(value.number);
                value.number = 0.0;
                continue;
            }
            if code == op::GROUP_END {
                code = operand as i32;
                operand = value.number;
                value.number = stack.pop().unwrap_or(0.0);
                text = None;
            }
            apply(code, &mut value, operand, text);
        }
        // A `string` trait whose number changed shows it as a whole number.
        if trait_id == t::STRING && value.number != before.number {
            value.string = Some(format!("{}", text::round_half_even(value.number)));
        }
        let tr = self.trait_mut(tile, trait_id);
        tr.value = value.clone();
        tr.worked_out = pass;
        tr.busy = false;
        if fed.is_some() {
            tr.fed = fed;
        }
        value
    }

    /// A text tile's text laid out with its font, wrap width, lines, line
    /// gap and justification (`00a21af0`); `None` when its font isn't
    /// loaded.
    pub fn layout(&mut self, tile: TileId) -> Option<TextLayout> {
        let pass = self.pass;
        if let Some((p, layout)) = &self.tiles[tile].layout {
            if *p == pass {
                return Some(layout.clone());
            }
        }
        let font_index = (self.number(tile, t::FONT) as i32).max(1) as usize;
        let font = self.fonts.get(font_index - 1).cloned().flatten()?;
        let mut wrap = self.number(tile, t::WRAPWIDTH) as i32;
        if wrap < 1 {
            wrap = self.screen_size.width() as i32;
        }
        if self.number(tile, t::ISHTML) > 0.0 {
            let layout = self.html_layout(tile, font_index, wrap);
            self.tiles[tile].layout = Some((self.pass, layout.clone()));
            return Some(layout);
        }
        let lines = self.number(tile, t::WRAPLINES) as i32;
        let gap = self.number(tile, t::LINEGAP) as i32;
        let justify = self.number(tile, t::JUSTIFY) as i32;
        let raw = self.string(tile, t::STRING).unwrap_or_default();
        let setting = &self.setting;
        let text = text::substitute(raw.as_bytes(), &|n| setting(n));
        let line_height = (font.line_height as i32 + gap) as f32;
        let extra = extra_line_gap(font_index);
        let prepared = text::prepare(&font, line_height, extra, &text, wrap, lines);
        let (quads, width) = text::glyphs(&font, line_height, extra, &prepared, justify);
        let layout = TextLayout {
            font: font_index,
            quads,
            quad_fonts: Vec::new(),
            pictures: Vec::new(),
            pages: 0,
            width,
            height: prepared.height,
        };
        self.tiles[tile].layout = Some((self.pass, layout.clone()));
        Some(layout)
    }

    /// An `ishtml` text tile's layout (`00a21af0` → `00a18a30`): the
    /// parser's for text starting with a tag, else each character in the
    /// tile's font and justification; a wrap width or `wraplimit` below 2
    /// is no limit; the page shown is `pagenum`.
    fn html_layout(&mut self, tile: TileId, font_index: usize, wrap: i32) -> TextLayout {
        let wrap = if wrap < 2 {
            crate::html::UNLIMITED
        } else {
            wrap
        };
        let limit = self.number(tile, t::WRAPLIMIT) as i32;
        let limit = if limit < 2 {
            crate::html::UNLIMITED
        } else {
            limit
        };
        let page = self.number(tile, t::PAGENUM) as i32;
        let justify = self.number(tile, t::JUSTIFY) as i32;
        let raw = self.string(tile, t::STRING).unwrap_or_default();
        let setting = &self.setting;
        let layout = crate::html::parse(&self.fonts, raw.as_bytes(), wrap, limit, &|n| setting(n))
            .unwrap_or_else(|| {
                crate::html::plain(
                    &self.fonts,
                    raw.as_bytes(),
                    font_index - 1,
                    justify,
                    wrap,
                    limit,
                )
            });
        let (glyphs, pictures) = crate::html::draw(&self.fonts, &layout, page);
        let (width, height) = layout.size(page);
        let (quad_fonts, quads) = glyphs.into_iter().unzip();
        TextLayout {
            font: font_index,
            quads,
            quad_fonts,
            pictures,
            pages: layout.pages.len() as i32,
            width,
            height,
        }
    }

    /// A tile's position on the screen: its x, y plus those of every
    /// ancestor whose `locus` isn't 0 (`00a013d0`, `00a01440`).
    pub fn screen_position(&mut self, tile: TileId) -> (f32, f32) {
        let mut x = self.number(tile, t::X);
        let mut y = self.number(tile, t::Y);
        let mut a = self.tiles[tile].parent;
        while let Some(p) = a {
            if self.number(p, t::LOCUS) != 0.0 {
                x += self.number(p, t::X);
                y += self.number(p, t::Y);
            }
            a = self.tiles[p].parent;
        }
        (x, y)
    }

    /// Its depth: its own plus every ancestor's that has `locus` or is a
    /// menu (`00a014b0`).
    pub fn screen_depth(&mut self, tile: TileId) -> f32 {
        let mut d = self.number(tile, t::DEPTH);
        let mut a = self.tiles[tile].parent;
        while let Some(p) = a {
            let is_menu = self.tiles[p].parent == Some(self.screen);
            if self.number(p, t::LOCUS) != 0.0 || is_menu {
                d += self.number(p, t::DEPTH);
            }
            a = self.tiles[p].parent;
        }
        d
    }

    /// Whether it's shown: its `visible` and every ancestor's (a trait
    /// never set counts as shown; the engine only hides a tile when the
    /// trait changes to 0).
    pub fn shown(&mut self, tile: TileId) -> bool {
        let mut a = Some(tile);
        while let Some(t_) = a {
            if self.has(t_, t::VISIBLE) && self.number(t_, t::VISIBLE) == 0.0 {
                return false;
            }
            a = self.tiles[t_].parent;
        }
        true
    }
}

/// One operator on the value (`00a09410`'s switch).
fn apply(code: i32, value: &mut Value, operand: f32, text: Option<String>) {
    let v = value.number;
    let b = |c: bool| if c { 1.0 } else { 0.0 };
    match code {
        op::COPY => match text {
            Some(s) => value.string = Some(s),
            None => value.number = operand,
        },
        op::ADD => value.number = v + operand,
        op::SUB => value.number = v - operand,
        op::MUL => value.number = v * operand,
        op::DIV => {
            if operand != 0.0 {
                value.number = v / operand;
            }
        }
        op::MIN => value.number = if operand <= v { operand } else { v },
        op::MAX => value.number = if v <= operand { operand } else { v },
        op::MOD => {
            let o = text::round_half_even(operand);
            if operand != 0.0 && o != 0 {
                value.number = (text::round_half_even(v) % o) as f32;
            }
        }
        // Of the value plus the operand (`00a0993d`; the files give 0).
        op::FLOOR => value.number = (operand + v).floor(),
        op::CEIL => value.number = (operand + v).ceil(),
        op::ABS => value.number = (operand + v).abs(),
        op::ROUND => {
            // To the nearest multiple of the operand, halves up.
            let q = v / operand;
            let whole = q.trunc();
            let up = if q - whole >= 0.5 { 1.0 } else { 0.0 };
            value.number = operand * (whole + up);
        }
        op::GT => value.number = b(operand < v),
        op::GTE => value.number = b(operand <= v),
        op::EQ => value.number = b(operand == v),
        op::NEQ => value.number = b(operand != v),
        op::LT => value.number = b(v < operand),
        op::LTE => value.number = b(v <= operand),
        op::AND => value.number = b(v != 0.0 && (operand != 0.0 || text.is_some())),
        op::OR => value.number = b(v != 0.0 || operand != 0.0 || text.is_some()),
        op::NOT => value.number = b(operand == 0.0 && text.is_none()),
        op::ONLYIF if operand == 0.0 && text.is_none() => value.number = 0.0,
        op::ONLYIFNOT if operand != 0.0 || text.is_some() => value.number = 0.0,
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn ui() -> Ui {
        Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|n| (n == "sStatsXP").then(|| "XP".to_string())),
        )
    }

    fn load(ui: &mut Ui, xml: &str) -> TileId {
        ui.load_menu(xml.as_bytes(), &mut |_| None).unwrap()
    }

    #[test]
    fn the_screen_tile() {
        let mut ui = ui();
        let s = ui.screen;
        assert!((ui.number(s, t::WIDTH) - 1706.6666).abs() < 1e-3);
        assert_eq!(ui.number(s, t::HEIGHT), 960.0);
        assert!((ui.number(s, t::RESOLUTIONCONVERTER) - 960.0 / 1080.0).abs() < 1e-6);
        assert_eq!(ui.number(s, t::CROPX), 15.0);
        let tall = Screen {
            width_px: 1080,
            height_px: 1920,
            safe_x: 0.0,
            safe_y: 0.0,
        };
        assert_eq!(tall.width(), 1280.0);
        assert!((tall.height() - 1920.0 / 1080.0 * 1280.0).abs() < 1e-3);
    }

    #[test]
    fn operators_as_the_game_runs_them() {
        let mut ui = ui();
        let m = load(
            &mut ui,
            "<menu name=\"m\"><rect name=\"a\">
                <x><copy src=\"screen()\" trait=\"width\"/><sub src=\"me()\" trait=\"width\"/><div>2</div></x>
                <width> 100 </width>
                <_t><copy> 1 </copy><onlyif> 0 </onlyif><add> 2 </add></_t>
                <_n><not> 0 </not></_n>
                <_g><copy> 10 </copy><add><copy src=\"me()\" trait=\"width\"/><mul> 2 </mul></add></_g>
                <_r><copy> 7 </copy><round> 5 </round></_r>
                <_m><copy> 7.6 </copy><mod> 3 </mod></_m>
                <_abs><copy> -3 </copy><abs> 1 </abs></_abs>
                <_gt><copy> 3 </copy><gt> 2 </gt></_gt>
            </rect></menu>",
        );
        let a = ui.find(m, "a").unwrap();
        assert!((ui.number(a, t::X) - (1706.6666 - 100.0) / 2.0).abs() < 1e-3);
        let id = |ui: &Ui, n: &str| ui.names.lookup(n).unwrap();
        // onlyif 0 zeroes the value; the chain carries on.
        let tt = id(&ui, "_t");
        assert_eq!(ui.number(a, tt), 2.0);
        // not: of the operand.
        let n = id(&ui, "_n");
        assert_eq!(ui.number(a, n), 1.0);
        // A group: 10 + 100 * 2.
        let g = id(&ui, "_g");
        assert_eq!(ui.number(a, g), 210.0);
        let r = id(&ui, "_r");
        assert_eq!(ui.number(a, r), 5.0);
        let m_ = id(&ui, "_m");
        assert_eq!(ui.number(a, m_), 2.0);
        let ab = id(&ui, "_abs");
        assert_eq!(ui.number(a, ab), 2.0);
        let gt = id(&ui, "_gt");
        assert_eq!(ui.number(a, gt), 1.0);
    }

    #[test]
    fn selectors_strings_and_settings() {
        let mut ui = ui();
        let m = load(
            &mut ui,
            "<menu name=\"m\"><rect name=\"box\"><_label>&-sStatsXP;</_label>
               <text name=\"t\"><string><copy src=\"parent()\" trait=\"_label\"/></string></text>
             </rect><rect name=\"other\"><x><copy src=\"t\" trait=\"y\"/><add> 5 </add></x>
             <_s><copy src=\"sibling(box)\" trait=\"_label\"/></_s>
             <_c><copy src=\"child(nothing)\" trait=\"x\"/></_c></rect></menu>",
        );
        let t_ = ui.find(m, "t").unwrap();
        assert_eq!(ui.string(t_, t::STRING).as_deref(), Some("XP"));
        let other = ui.find(m, "other").unwrap();
        assert_eq!(ui.number(other, t::X), 5.0);
        let s = ui.names.lookup("_s").unwrap();
        assert_eq!(ui.string(other, s).as_deref(), Some("XP"));
        // A source that isn't there drops the operator, so the trait never
        // gets made.
        let c = ui.names.lookup("_c").unwrap();
        assert!(!ui.has(other, c));
    }

    #[test]
    fn io_is_the_menu() {
        // `lockpick_menu.xml` hides its debugging pieces with the menu's
        // own `_DebugMode`, read through `io()` (the menu tile, once it has
        // a class, as every menu file gives it); the code sets it.
        let mut ui = ui();
        let m = load(
            &mut ui,
            "<menu name=\"m\"><class>&LockPickMenu;</class><_DebugMode>&false;</_DebugMode><rect name=\"r\">
               <rect name=\"debug\"><visible><copy src=\"io()\" trait=\"_DebugMode\"/></visible></rect>
             </rect></menu>",
        );
        ui.set_number(m, t::VISIBLE, 1.0);
        let debug = ui.find(m, "debug").unwrap();
        assert!(!ui.shown(debug));
        let flag = ui.names.lookup("_DebugMode").unwrap();
        ui.set_number(m, flag, 1.0);
        assert!(ui.shown(debug));
    }

    #[test]
    fn numbers_written_into_strings() {
        let mut ui = ui();
        let m = load(
            &mut ui,
            "<menu name=\"m\"><rect name=\"r\"><_n> 0 </_n></rect>
             <text name=\"t\"><string><copy src=\"r\" trait=\"_n\"/></string></text></menu>",
        );
        let r = ui.find(m, "r").unwrap();
        let t_ = ui.find(m, "t").unwrap();
        // 0 copied onto 0: unchanged, so the text stays empty.
        assert_eq!(ui.string(t_, t::STRING).as_deref(), Some(""));
        let n = ui.names.lookup("_n").unwrap();
        ui.set_number(r, n, 42.6);
        assert_eq!(ui.string(t_, t::STRING).as_deref(), Some("43"));
    }

    #[test]
    fn indexed_traits() {
        let mut ui = ui();
        let m = load(
            &mut ui,
            "<menu name=\"m\"><image name=\"i\"><_filename_1> a.dds </_filename_1><_filename_2> b.dds </_filename_2>
             <filename><copy> 2 </copy><copy src=\"me()\" trait=\"_filename_\"/></filename></image></menu>",
        );
        let i = ui.find(m, "i").unwrap();
        assert_eq!(ui.string(i, t::FILENAME).as_deref(), Some("b.dds"));
    }

    #[test]
    fn templates_positions_and_visibility() {
        let mut ui = ui();
        let m = load(
            &mut ui,
            "<menu name=\"m\"><locus>&true;</locus><x>0</x>
               <template name=\"tp\"><image name=\"icon\"><x> 3 </x><y> 4 </y></image></template>
               <rect name=\"holder\"><locus>&true;</locus><x>100</x><y>200</y>
                 <rect name=\"plain\"><x>10</x><y>20</y><visible>&false;</visible>
                   <image name=\"deep\"><x>1</x><y>2</y><depth>5</depth></image></rect></rect></menu>",
        );
        let holder = ui.find(m, "holder").unwrap();
        let icon = ui.instantiate(m, holder, "tp").unwrap();
        assert_eq!(ui.tiles[icon].parent, Some(holder));
        assert_eq!(ui.screen_position(icon), (103.0, 204.0));
        // Names are compared exactly; with no match the last template is
        // made (`00a1ddb0`).
        let other = ui.instantiate(m, holder, "TP").unwrap();
        assert_eq!(ui.tiles[other].name, "icon");
        assert_ne!(other, icon);
        // `plain` has no locus: its own x, y don't move `deep`.
        let deep = ui.find(m, "deep").unwrap();
        assert_eq!(ui.screen_position(deep), (101.0, 202.0));
        assert!(!ui.shown(deep));
        // Menus start hidden (`00a1ef30`).
        assert!(!ui.shown(icon));
        ui.set_number(m, t::VISIBLE, 1.0);
        assert!(ui.shown(icon));
        // Depth: the image's 5 plus the menu's (0).
        assert_eq!(ui.screen_depth(deep), 5.0);
        // An image starts with depth 1, brightness 255.
        assert_eq!(ui.number(icon, t::DEPTH), 1.0);
        assert_eq!(ui.number(icon, t::BRIGHTNESS), 255.0);
    }

    #[test]
    fn io_childcount_previous_sibling_and_removing() {
        let mut ui = ui();
        // The stats menu's way of reading values the code writes: `io()`
        // is the menu; "no effects" shows while the list holds only its
        // two own tiles.
        let m = load(
            &mut ui,
            "<menu name=\"m\"><class>&StatsMenu;</class><user5> 7 </user5><rect name=\"list\"><rect name=\"a\"/><rect name=\"b\"/></rect>
             <text name=\"none\"><visible><copy src=\"sibling()\" trait=\"childcount\"/><eq> 2 </eq></visible>
               <_v><copy src=\"io()\" trait=\"user5\"/></_v></text></menu>",
        );
        let none = ui.find(m, "none").unwrap();
        let v = ui.names.lookup("_v").unwrap();
        assert_eq!(ui.number(none, v), 7.0);
        assert_eq!(ui.number(none, t::VISIBLE), 1.0);
        let list = ui.find(m, "list").unwrap();
        let tp = r#"<menu name="x"><template name="row"><rect name="row"/></template></menu>"#;
        let other = load(&mut ui, tp);
        let row = ui.instantiate(other, list, "row").unwrap();
        assert_eq!(ui.number(list, t::CHILDCOUNT), 3.0);
        assert_eq!(ui.number(none, t::VISIBLE), 0.0);
        ui.remove(row);
        assert_eq!(ui.number(list, t::CHILDCOUNT), 2.0);
        assert_eq!(ui.number(none, t::VISIBLE), 1.0);
        assert!(ui.find(m, "row").is_none());
    }

    #[test]
    fn code_settings_drop_operators() {
        let mut ui = ui();
        let m = load(
            &mut ui,
            "<menu name=\"m\"><rect name=\"r\"><x><copy> 5 </copy></x></rect></menu>",
        );
        let r = ui.find(m, "r").unwrap();
        assert_eq!(ui.number(r, t::X), 5.0);
        ui.set_number(r, t::X, 9.0);
        assert!(ui.tiles[r].traits[&t::X].actions.is_empty());
        assert_eq!(ui.number(r, t::X), 9.0);
    }

    /// `io()` is the menu's own tile once the menu has a class
    /// (`00a03c90` and the object's `+0x4`); `link` copies a trait from
    /// code; `set_text` with nothing gives no text; `detach` takes a menu
    /// off the screen.
    #[test]
    fn io_links_and_closing() {
        let mut ui = ui();
        let m = load(
            &mut ui,
            "<menu name=\"m\"><class>&MessageMenu;</class><_w>640</_w>
               <rect name=\"a\"><width><copy src=\"io()\" trait=\"_w\"/></width><_v>3</_v></rect>
               <rect name=\"b\"><_link>0</_link></rect></menu>",
        );
        let a = ui.find(m, "a").unwrap();
        assert_eq!(ui.number(a, t::WIDTH), 640.0);
        let plain = load(&mut ui, "<menu name=\"p\"><rect name=\"c\"><width><copy src=\"io()\" trait=\"_w\"/></width></rect></menu>");
        let c = ui.find(plain, "c").unwrap();
        assert!(!ui.has(c, t::WIDTH));
        let b = ui.find(m, "b").unwrap();
        let link = ui.names.lookup("_link").unwrap();
        let v = ui.names.lookup("_v").unwrap();
        ui.link(b, link, a, v);
        assert_eq!(ui.number(b, link), 3.0);
        ui.set_number(a, v, 7.0);
        assert_eq!(ui.number(b, link), 7.0);
        ui.set_text(b, t::STRING, "");
        assert_eq!(ui.string(b, t::STRING), None);
        ui.set_text(b, t::STRING, "x");
        assert_eq!(ui.string(b, t::STRING).as_deref(), Some("x"));
        ui.detach(m);
        assert!(!ui.tiles[ui.screen].children.contains(&m));
        assert_eq!(ui.descendants(a), vec![a]);
    }
    #[test]
    fn system_colours() {
        let c = SystemColors::new(Some(4290134783), None);
        assert_eq!(c.get(1), Some([1.0, 182.0 / 255.0, 66.0 / 255.0]));
        let green = SystemColors::new(Some(0x1AFF80FF), None);
        assert_eq!(green.get(1), Some([26.0 / 255.0, 1.0, 128.0 / 255.0]));
        assert_eq!(c.get(0), None);
        assert_eq!(c.get(2), Some([1.0, 67.0 / 255.0, 42.0 / 255.0]));
    }
}
