//! The lockpicking menu (`menus\lockpick_menu.xml`, class `LockPickMenu`
//! 1014, vtable `0107439c` in FalloutNV.exe), set up and filled the way its
//! code does it (`0078db00`, `00791000`, `0078eb50`). The rules of the
//! game itself are in `world::lockpick`; this is the menu's tiles.
//!
//! The menu keeps sixteen tiles by their `id` (its `SetTile`, `0078daa0`:
//! ids 0..15 into the menu object at +0x2c): 0 `LPM_SkillDisplay`, 1
//! `LPM_PickCountDisplay`, 2 `LPM_LevelDisplay`, 3–8 the debug tweak
//! buttons and values, 9 `LPM_ForceLockButton`, 10 `LPM_ExitButton`, 11
//! `LPM_StatusText`, 12 `LPM_PickHealthText`, 13 `LPM_DetectArea`, 14
//! `LPM_DetectMeter`, 15 `LPM_DetectArrow`; a menu missing one isn't made
//! ("MENUS: Lockpicking Menu Creation Failed…", `0078e090`). The meter
//! (shown only in the menu's debug mode, `_DebugMode`) is where the pick's
//! position and the sweet spot's rings live: the arrow's `_x` is the pick,
//! and six `LPM_SweetSpotTemplate` pictures under the meter are the rings.

use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const MENU_FILE: &str = "menus\\lockpick_menu.xml";
/// The rings' template (`00791000`).
pub const SWEET_SPOT_TEMPLATE: &str = "LPM_SweetSpotTemplate";
/// The tiles the menu keeps (ids 0..15).
pub const TILES: usize = 16;
pub const SKILL: usize = 0;
pub const PICK_COUNT: usize = 1;
pub const LEVEL: usize = 2;
pub const FORCE_LOCK: usize = 9;
pub const EXIT: usize = 10;
pub const STATUS: usize = 11;
pub const PICK_HEALTH: usize = 12;
pub const DETECT_AREA: usize = 13;
pub const METER: usize = 14;
pub const ARROW: usize = 15;

/// The debug line the code writes before any straining (`0078db00`,
/// `0078eb50` stage 6; shown only in debug mode).
pub const FULL_PICK_HEALTH: &str = "PICK HEALTH: 100%";

/// What the menu shows, given by the caller (`0078db00`).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Texts {
    /// The player's Lockpick ("%d").
    pub skill: i32,
    /// "Force Lock [n%]" (`sForceLock` and the chance).
    pub force_lock: String,
    /// The lock level's name (`sLockLevelName…`).
    pub level: String,
    /// Bobby pins.
    pub pins: i32,
    /// With the game's language `ENGLISH` (`[General] sLanguage`) the code
    /// writes the titles itself: `sLockpickSkillText`,
    /// `sPicksRemainingText`, `sLockLevelText`, and the exit button's
    /// `sExit`.
    pub english: Option<[String; 4]>,
}

/// One ring round the sweet spot: its `id`, `x`, `width`, `depth` and
/// colour (0..1, written × 255 into red, green, blue).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RingTile {
    pub id: i32,
    pub x: i32,
    pub width: f32,
    pub depth: i32,
    pub color: [f32; 3],
}

/// Works out every trait under `root` now, as the game's tiles update
/// when what they read changes (so a value set and set again in one go
/// counts as two changes, as in the game).
fn settle(ui: &mut Ui, root: TileId) {
    let mut stack = vec![root];
    while let Some(tile) = stack.pop() {
        let traits: Vec<i32> = ui.tiles[tile].traits.keys().copied().collect();
        for id in traits {
            ui.value(tile, id);
        }
        stack.extend(ui.tiles[tile].children.iter().copied());
    }
    ui.refresh();
}

/// The lockpicking menu, made from its file.
#[derive(Debug, Clone)]
pub struct LockpickMenu {
    pub menu: TileId,
    pub tiles: [TileId; TILES],
    /// The custom traits the code writes: `_Value` (the displays' values)
    /// and `_x` (the arrow's place on the meter).
    pub value: i32,
    pub x: i32,
}

impl LockpickMenu {
    /// Loads the menu (`0078db00`): its tiles by id, the menu's
    /// `_DebugMode` (false: the meter, arrow and tweak buttons stay
    /// hidden), and shown (`00a1dc20`).
    pub fn load(
        ui: &mut Ui,
        read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
        debug: bool,
    ) -> Result<LockpickMenu, String> {
        let text = read(MENU_FILE).ok_or_else(|| format!("{MENU_FILE} not found"))?;
        let menu = ui.load_menu(&text, read)?;
        let mut tiles: [Option<TileId>; TILES] = [None; TILES];
        let mut stack = vec![menu];
        let mut order = Vec::new();
        while let Some(tile) = stack.pop() {
            order.push(tile);
            for &c in ui.tiles[tile].children.iter().rev() {
                stack.push(c);
            }
        }
        for tile in order {
            if !ui.has(tile, t::ID) {
                continue;
            }
            let id = ui.number(tile, t::ID);
            if id >= 0.0 && (id as usize) < TILES && id.fract() == 0.0 {
                tiles[id as usize] = Some(tile);
            }
        }
        if tiles.iter().any(Option::is_none) {
            return Err(
                "MENUS: Lockpicking Menu Creation Failed... Are your menu and art resources up to date?"
                    .into(),
            );
        }
        let value = ui.names.lookup_or_add("_Value").ok_or("no _Value trait")?;
        let x = ui.names.lookup_or_add("_x").ok_or("no _x trait")?;
        let debug_trait = ui
            .names
            .lookup_or_add("_DebugMode")
            .ok_or("no _DebugMode trait")?;
        ui.set_number(menu, debug_trait, if debug { 1.0 } else { 0.0 });
        ui.set_number(menu, t::VISIBLE, 1.0);
        ui.refresh();
        Ok(LockpickMenu {
            menu,
            tiles: tiles.map(|t| t.unwrap_or_default()),
            value,
            x,
        })
    }

    /// Fills the displays (`0078db00`): the skill's `_Value` "%d", the
    /// Force Lock button's text, the level's name, the debug pick health,
    /// the pins' `_Value` (set to 1 first, then the count, so the text
    /// copying it changes even for 0); in English the titles and "Exit".
    pub fn fill(&self, ui: &mut Ui, texts: &Texts) {
        let tiles = &self.tiles;
        ui.set_string(tiles[SKILL], self.value, &texts.skill.to_string());
        ui.set_string(tiles[FORCE_LOCK], t::STRING, &texts.force_lock);
        ui.set_string(tiles[LEVEL], self.value, &texts.level);
        ui.set_string(tiles[PICK_HEALTH], t::STRING, FULL_PICK_HEALTH);
        ui.set_number(tiles[PICK_COUNT], self.value, 1.0);
        settle(ui, self.menu);
        ui.set_number(tiles[PICK_COUNT], self.value, texts.pins as f32);
        if let Some([skill, pins, level, exit]) = &texts.english {
            for (name, text) in [
                ("LPM_SkillTitle", skill),
                ("LPM_PickCountTitle", pins),
                ("LPM_LevelTitle", level),
            ] {
                if let Some(tile) = ui.find(self.menu, name) {
                    ui.set_string(tile, t::STRING, text);
                }
            }
            ui.set_string(tiles[EXIT], t::STRING, exit);
        }
        ui.refresh();
        settle(ui, self.menu);
    }

    /// The meter's width: the pick's whole travel (`LPM_DetectMeter`).
    pub fn meter_width(&self, ui: &mut Ui) -> f32 {
        ui.number(self.tiles[METER], t::WIDTH)
    }

    /// The arrow's `_x`: where the pick is on the meter.
    pub fn pick_x(&self, ui: &mut Ui) -> f32 {
        ui.number(self.tiles[ARROW], self.x)
    }

    /// Moves the arrow (`007908f0`, `0078eb50` stage 6).
    pub fn set_pick_x(&self, ui: &mut Ui, x: f32) {
        ui.set_number(self.tiles[ARROW], self.x, x);
    }

    /// The bobby pins shown (`0078eb50`: one less when a pin breaks).
    pub fn set_pins(&self, ui: &mut Ui, pins: i32) {
        ui.set_number(self.tiles[PICK_COUNT], self.value, pins as f32);
    }

    /// The rings under the meter (`00791000`): each one the template
    /// instanced with its `id`, `width`, `x`, `depth` and colour.
    pub fn place_rings(&self, ui: &mut Ui, rings: &[RingTile]) -> Vec<TileId> {
        let meter = self.tiles[METER];
        let mut made = Vec::new();
        for ring in rings {
            let Some(tile) = ui.instantiate(self.menu, meter, SWEET_SPOT_TEMPLATE) else {
                continue;
            };
            ui.set_number(tile, t::ID, ring.id as f32);
            ui.set_number(tile, t::WIDTH, ring.width);
            ui.set_number(tile, t::X, ring.x as f32);
            ui.set_number(tile, t::DEPTH, ring.depth as f32);
            ui.set_number(tile, t::RED, ring.color[0] * 255.0);
            ui.set_number(tile, t::GREEN, ring.color[1] * 255.0);
            ui.set_number(tile, t::BLUE, ring.color[2] * 255.0);
            made.push(tile);
        }
        ui.refresh();
        made
    }

    /// The cursor is hidden while it's over the detect area (`007902b0`,
    /// `007902f0`: tile id 13 sets the cursor's alpha to 0, leaving it 255).
    pub fn hides_cursor(&self, hovered: Option<TileId>) -> bool {
        hovered == Some(self.tiles[DETECT_AREA])
    }

    /// The menu's `_PCButton_<key>` trait: the tile a key clicks, by name
    /// (`0070c4a0`: `_PCButton_F` the Force Lock button, `_PCButton_E`
    /// the Exit button).
    pub fn pc_button(&self, ui: &mut Ui, key: char) -> Option<TileId> {
        let name = format!("_PCButton_{}", key.to_ascii_uppercase());
        let trait_id = ui.names.lookup(&name)?;
        let target = ui.string(self.menu, trait_id)?;
        ui.find(self.menu, target.trim())
    }

    /// The id of a tile the menu keeps (`HandleClick` gets it).
    pub fn id_of(&self, tile: TileId) -> Option<usize> {
        self.tiles.iter().position(|&t| t == tile)
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

    /// A menu shaped like the game's: every id 0..15, the meter as wide as
    /// the main rectangle, the arrow's `_x` its middle, the template, the
    /// keys' traits and debug-only pieces.
    fn menu_file() -> String {
        let mut s = String::from(
            "<menu name=\"LockPickMenu\"><class>&LockPickMenu;</class><_PCButton_F> LPM_ForceLockButton </_PCButton_F><_PCButton_E> LPM_ExitButton </_PCButton_E>
             <_DebugMode>&false;</_DebugMode>
             <rect name=\"LPM_MainRect\"><locus>&true;</locus>
               <x><copy src=\"screen()\" trait=\"width\"/><sub src=\"me()\" trait=\"width\"/><div>2</div></x>
               <width><copy src=\"screen()\" trait=\"width\"/><sub><copy src=\"screen()\" trait=\"cropx\"/><mul>4</mul></sub></width>
               <text name=\"LPM_SkillTitle\"><string>x</string></text>
               <rect name=\"LPM_SkillDisplay\"><id>0</id><_Value></_Value><text name=\"v\"><string><copy src=\"parent()\" trait=\"_Value\"/></string></text></rect>
               <rect name=\"LPM_PickCountDisplay\"><id>1</id><_Value></_Value><text name=\"pc\"><string><copy src=\"parent()\" trait=\"_Value\"/></string></text></rect>
               <rect name=\"LPM_LevelDisplay\"><id>2</id><_Value></_Value></rect>",
        );
        for id in 3..=8 {
            s.push_str(&format!("<rect name=\"tweak{id}\"><id>{id}</id></rect>"));
        }
        s.push_str(
            "<hotrect name=\"LPM_ForceLockButton\"><id>9</id><string></string></hotrect>
               <hotrect name=\"LPM_ExitButton\"><id>10</id><string>Exit</string></hotrect>
               <text name=\"LPM_StatusText\"><id>11</id></text>
               <text name=\"LPM_PickHealthText\"><id>12</id><visible><copy src=\"io()\" trait=\"_DebugMode\"/></visible></text>
               <hotrect name=\"LPM_DetectArea\"><id>13</id></hotrect>
               <hotrect name=\"LPM_DetectMeter\"><id>14</id><width><copy src=\"parent()\" trait=\"width\"/></width>
                 <visible><copy src=\"io()\" trait=\"_DebugMode\"/></visible></hotrect>
               <image name=\"LPM_DetectArrow\"><id>15</id><_x><copy src=\"sibling(LPM_DetectMeter)\" trait=\"width\"/><div>2</div></_x></image>
             </rect>
             <template name=\"LPM_SweetSpotTemplate\"><image name=\"LPM_SweetSpot\"><height>10</height></image></template>
             </menu>",
        );
        s
    }

    #[test]
    fn the_menu_keeps_its_tiles_and_is_filled_as_the_code_does() {
        let mut ui = ui();
        let file = menu_file();
        let m = LockpickMenu::load(
            &mut ui,
            &mut |p| (p == MENU_FILE).then(|| file.clone().into_bytes()),
            false,
        )
        .unwrap();
        // The meter is the main rectangle's width: 1706.67 − 4 × 15; the
        // pick starts in its middle.
        let w = m.meter_width(&mut ui);
        assert!((w - 1646.6666).abs() < 1e-3);
        assert!((m.pick_x(&mut ui) - w / 2.0).abs() < 1e-3);
        // The debug pieces stay hidden.
        assert!(!ui.shown(m.tiles[METER]));
        assert!(!ui.shown(m.tiles[PICK_HEALTH]));
        m.fill(
            &mut ui,
            &Texts {
                skill: 57,
                force_lock: "Force Lock [17%]".into(),
                level: "Average".into(),
                pins: 12,
                english: Some([
                    "Lockpick Skill".into(),
                    "Bobby Pins".into(),
                    "Lock Level".into(),
                    "Exit".into(),
                ]),
            },
        );
        let v = ui.find(m.menu, "v").unwrap();
        assert_eq!(ui.string(v, t::STRING).as_deref(), Some("57"));
        assert_eq!(ui.number(m.tiles[PICK_COUNT], m.value), 12.0);
        let title = ui.find(m.menu, "LPM_SkillTitle").unwrap();
        assert_eq!(
            ui.string(title, t::STRING).as_deref(),
            Some("Lockpick Skill")
        );
        // Keys: F and E click the buttons the menu names.
        assert_eq!(m.pc_button(&mut ui, 'f'), Some(m.tiles[FORCE_LOCK]));
        assert_eq!(m.pc_button(&mut ui, 'E'), Some(m.tiles[EXIT]));
        assert_eq!(m.pc_button(&mut ui, 'q'), None);
        // Rings go under the meter.
        let rings = m.place_rings(
            &mut ui,
            &[RingTile {
                id: 17,
                x: 773,
                width: 100.5,
                depth: 6,
                color: [0.0, 1.0, 0.0],
            }],
        );
        assert_eq!(ui.tiles[rings[0]].parent, Some(m.tiles[METER]));
        assert_eq!(ui.number(rings[0], t::ID), 17.0);
        assert_eq!(ui.number(rings[0], t::GREEN), 255.0);
    }

    #[test]
    fn no_pins_still_shows_a_number() {
        // The code sets the count to 1 first: copying 0 onto 0 changes
        // nothing, so without it the text would stay empty.
        let mut ui = ui();
        let file = menu_file();
        let m = LockpickMenu::load(
            &mut ui,
            &mut |p| (p == MENU_FILE).then(|| file.clone().into_bytes()),
            false,
        )
        .unwrap();
        m.fill(&mut ui, &Texts::default());
        let pc = ui.find(m.menu, "pc").unwrap();
        assert_eq!(ui.string(pc, t::STRING).as_deref(), Some("0"));
        m.set_pins(&mut ui, 3);
        assert_eq!(ui.string(pc, t::STRING).as_deref(), Some("3"));
    }

    #[test]
    fn a_menu_missing_a_tile_is_not_made() {
        let mut ui = ui();
        let file = menu_file().replace("<id>12</id>", "");
        assert!(LockpickMenu::load(
            &mut ui,
            &mut |p| (p == MENU_FILE).then(|| file.clone().into_bytes()),
            false,
        )
        .is_err());
    }
}
