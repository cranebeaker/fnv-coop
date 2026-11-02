//! The companion wheel (`menus\companion_wheel_menu.xml`, class
//! `CompanionWheelMenu` 1075, vtable `01071d0c` in FalloutNV.exe): using a
//! teammate brings it up (`world::companions`). Eight slices of a pie
//! (`radial` tiles, ids 0 to 7, picked by angle and distance from the
//! screen's centre), each an order said to the companion as a dialogue
//! topic. Read from the code:
//!
//! * opening (`00754de0`): the companion kept (`+0x68`); their four
//!   switches read from their script (`world::companions::switches`: the
//!   menu's bytes +0x6C keep distance, +0x6D aggressive, +0x6E ranged,
//!   +0x6F following) and buttons 0, 2, 5 and 7 given their other picture
//!   (`user11`) when on (2 when not following); the subtitle (id 12)
//!   emptied, the title (8) `sCWheelTitle`, the callouts (13, 14, 15)
//!   `sExit`, `sSelect`, `sNavigate`; `UIPopUpMessageGeneral`. No slice is
//!   chosen (`+0x70` -1, `00754b40`).
//! * the pointer on a slice or Exit (`00755760` → `007557d0`; 0 to 7 and
//!   11): `UIPipBoyScroll`; the slice chosen before loses its highlight
//!   (`user10` 0) and shows its switch as it is; the new one is lit
//!   (`user10` 1), a switch showing what a click would make it; the
//!   preview (9) says what a click does ("Be Passive" / "Be Aggressive",
//!   "Use Stimpak", "Wait Here" / "Follow Me", "Talk To", "Back Up", "Stay
//!   Close" / "Keep Distance", "Open Inventory", "Use Melee" / "Use
//!   Ranged", "Exit"); the context (10, `00755dc0`) the companion's health
//!   and the player's Stimpaks on 1 ("%s  %d/%d\n%s  %d"), what they carry
//!   on 6 ("%s  %d/%d"), their weapon's name on 7. Moving off does nothing
//!   (`+0x14` the default).
//! * a click (`007552e0`), then `UIMenuOK` (`00717280(1)`): a switch flips
//!   and its topic is said and done (`007561b0` aggressive:
//!   `FollowersTacticsCombatAggressive` / `…Passive`, and
//!   `FollowerSwitchAggressive` set to match; `007563a0` `FollowersWait` /
//!   `FollowersLetsGo`; `007560c0` `FollowersTacticsDistanceLong` /
//!   `…Default`; `007562b0` `FollowersTacticsCombatRanged` / `…Melee`); 1
//!   the Stimpak (`00756490`); those five then shown again as if the
//!   pointer came onto them (`007557d0(id, 1)`). 3 closes it to talk to
//!   them (`00756960`, `+0x74`, the talk starting as it closes,
//!   `00754c60` → `00756980`); 4 steps them back (`00756930` →
//!   `008a7760`) and closes; 6 closes and says `FollowersTrade` (whose
//!   result script opens their things, `OpenTeammateContainer`); 11
//!   closes.
//! * keys (`007556b0`): the A button / Enter (9) clicks the chosen slice,
//!   or Exit when none is; B (10) is Exit.
//! * a line said (`007573d0` → `00757690`): on the subtitle (12) until
//!   its voice ends, or its length × `fNoticeTextTimePerCharacter` (0.06)
//!   seconds without one; the update (`00755480`) empties it after.
//! * the pad's left stick (`00755480` → `00755c50`, [`CompanionWheelMenu::
//!   stick`]): a slice chosen by the stick's angle.

use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\companion_wheel_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1075;
/// The slices.
pub const AGGRESSIVE: i32 = 0;
pub const STIMPAK: i32 = 1;
pub const WAIT: i32 = 2;
pub const TALK: i32 = 3;
pub const BACK_UP: i32 = 4;
pub const DISTANCE: i32 = 5;
pub const INVENTORY: i32 = 6;
pub const RANGED: i32 = 7;
/// The other tiles.
pub const TITLE: i32 = 8;
pub const PREVIEW: i32 = 9;
pub const CONTEXT: i32 = 10;
pub const EXIT: i32 = 11;
pub const SUBTITLE: i32 = 12;
/// How many tile ids the menu keeps (`00754c00`: 0 to 15).
pub const TILE_COUNT: usize = 16;
/// The trait lighting a slice, and the one showing its other picture.
pub const HIGHLIGHT: i32 = t::USER0 + 10;
pub const ALTERNATE: i32 = t::USER0 + 11;
/// `fNoticeTextTimePerCharacter`'s default.
pub const TIME_PER_CHARACTER: f32 = 0.06;

/// What the menu asks of the game, in order.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// The companion says their line for a topic and its result scripts
    /// run (`007573d0`, `007575b0`).
    Topic(&'static str),
    /// A script variable of theirs set (`008c16c0`).
    Variable(&'static str, f32),
    /// The Stimpak (`00756490`).
    Stimpak,
    /// Stepping back from the player (`008a7760`).
    BackUp,
    /// Talking to them, once it's closed (`00756980`).
    Talk,
    /// Close it.
    Close,
}

/// What the context line shows (`00755dc0`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Facts {
    /// Their health now (truncated) and full.
    pub health: i32,
    pub max_health: i32,
    /// The player's Stimpaks.
    pub stimpaks: i32,
    /// What they carry and can carry (truncated).
    pub carried: i32,
    pub carry_limit: i32,
    /// Their weapon's name.
    pub weapon: Option<String>,
}

/// The wheel.
#[derive(Debug)]
pub struct CompanionWheelMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; TILE_COUNT],
    /// The switches (`+0x6C` to `+0x6F`).
    pub keep_distance: bool,
    pub aggressive: bool,
    pub ranged: bool,
    pub following: bool,
    /// The slice chosen (`+0x70`), -1 none.
    pub selected: i32,
    /// Talk once closed (`+0x74`).
    pub talk: bool,
    /// When the subtitle empties, in ms (`+0x78`).
    pub subtitle_until: Option<f64>,
    pub facts: Facts,
    pub requests: Vec<Request>,
    pub sounds: Vec<String>,
    pub closed: bool,
}

impl CompanionWheelMenu {
    pub fn new(menu: TileId) -> CompanionWheelMenu {
        CompanionWheelMenu {
            menu,
            tiles: [None; TILE_COUNT],
            keep_distance: false,
            aggressive: false,
            ranged: false,
            following: false,
            selected: -1,
            talk: false,
            subtitle_until: None,
            facts: Facts::default(),
            requests: Vec::new(),
            sounds: Vec::new(),
            closed: false,
        }
    }

    fn tile(&self, id: i32) -> Option<TileId> {
        usize::try_from(id)
            .ok()
            .and_then(|i| self.tiles.get(i).copied().flatten())
    }

    fn text(ui: &Ui, setting: &str) -> String {
        ui.setting_text(setting).unwrap_or_default()
    }

    fn set_text(&self, ui: &mut Ui, id: i32, text: &str) {
        if let Some(tile) = self.tile(id) {
            ui.set_text(tile, t::STRING, text);
        }
    }

    /// A slice's other picture shown or not.
    fn alternate(&self, ui: &mut Ui, id: i32, on: bool) {
        if let Some(tile) = self.tile(id) {
            ui.set_number(tile, ALTERNATE, if on { 1.0 } else { 0.0 });
        }
    }

    /// Opens it (`00754de0`) with the companion's switches; false when
    /// the file lacks one of its tiles ("MENUS: Companion Wheel Menu
    /// Creation Failed…").
    pub fn open(
        &mut self,
        ui: &mut Ui,
        switches: world::companions::Switches,
        facts: Facts,
    ) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        self.aggressive = switches.aggressive;
        self.keep_distance = switches.keep_distance;
        self.ranged = switches.ranged;
        self.following = !switches.waiting;
        self.facts = facts;
        self.alternate(ui, AGGRESSIVE, self.aggressive);
        self.alternate(ui, WAIT, !self.following);
        self.alternate(ui, DISTANCE, self.keep_distance);
        self.alternate(ui, RANGED, self.ranged);
        self.set_text(ui, SUBTITLE, "");
        for (id, setting) in [
            (TITLE, "sCWheelTitle"),
            (13, "sExit"),
            (14, "sSelect"),
            (15, "sNavigate"),
        ] {
            let s = Self::text(ui, setting);
            self.set_text(ui, id, &s);
        }
        self.sounds.push("UIPopUpMessageGeneral".into());
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
        true
    }

    /// The slice chosen (`007557d0`); `again` shows it afresh after a
    /// click.
    pub fn choose(&mut self, ui: &mut Ui, id: i32, again: bool) {
        if !again && self.selected == id {
            return;
        }
        self.sounds.push("UIPipBoyScroll".into());
        let before = self.selected;
        if let Some(tile) = self.tile(before) {
            ui.set_number(tile, HIGHLIGHT, 0.0);
        }
        match before {
            AGGRESSIVE => self.alternate(ui, AGGRESSIVE, self.aggressive),
            WAIT => self.alternate(ui, WAIT, !self.following),
            DISTANCE => self.alternate(ui, DISTANCE, self.keep_distance),
            RANGED => self.alternate(ui, RANGED, self.ranged),
            _ => {}
        }
        self.selected = id;
        if let Some(tile) = self.tile(id) {
            ui.set_number(tile, HIGHLIGHT, 1.0);
        }
        let either = |on: bool, yes: &str, no: &str| (if on { yes } else { no }).to_string();
        let preview = match id {
            AGGRESSIVE => {
                self.alternate(ui, AGGRESSIVE, !self.aggressive);
                either(self.aggressive, "sCWheelBePassive", "sCWheelBeAggressive")
            }
            STIMPAK => "sCWheelUseStimpak".into(),
            WAIT => {
                self.alternate(ui, WAIT, self.following);
                either(self.following, "sCWheelWaitHere", "sCWheelFollowMe")
            }
            TALK => "sCWheelTalkTo".into(),
            BACK_UP => "sCWheelBackUp".into(),
            DISTANCE => {
                self.alternate(ui, DISTANCE, !self.keep_distance);
                either(
                    self.keep_distance,
                    "sCWheelStayClose",
                    "sCWheelKeepDistance",
                )
            }
            INVENTORY => "sCWheelOpenInventory".into(),
            RANGED => {
                self.alternate(ui, RANGED, !self.ranged);
                either(self.ranged, "sCWheelUseMelee", "sCWheelUseRanged")
            }
            EXIT => "sExit".into(),
            _ => String::new(),
        };
        let preview = if preview.is_empty() {
            preview
        } else {
            Self::text(ui, &preview)
        };
        self.set_text(ui, PREVIEW, &preview);
        self.show_context(ui);
        ui.refresh();
    }

    /// The context line for the chosen slice (`00755dc0`).
    pub fn show_context(&mut self, ui: &mut Ui) {
        let f = &self.facts;
        let text = match self.selected {
            STIMPAK => format!(
                "{}  {}/{}\n{}  {}",
                Self::text(ui, "sHitPointsShort"),
                f.health,
                f.max_health,
                Self::text(ui, "sStimpak"),
                f.stimpaks
            ),
            INVENTORY => format!(
                "{}  {}/{}",
                Self::text(ui, "sInventoryWeight"),
                f.carried,
                f.carry_limit
            ),
            RANGED => f.weapon.clone().unwrap_or_default(),
            _ => String::new(),
        };
        self.set_text(ui, CONTEXT, &text);
    }

    /// New facts, and the context line shown again.
    pub fn set_facts(&mut self, ui: &mut Ui, facts: Facts) {
        self.facts = facts;
        self.show_context(ui);
        ui.refresh();
    }

    /// A line said (`00757690`): on the subtitle until `until` (ms).
    pub fn say(&mut self, ui: &mut Ui, text: &str, until: f64) {
        self.set_text(ui, SUBTITLE, text);
        self.subtitle_until = Some(until);
        ui.refresh();
    }

    /// How long a line without a voice stays (`00757690`): its length ×
    /// `fNoticeTextTimePerCharacter`, in ms.
    pub fn line_time(text: &str, per_character: f32) -> f64 {
        f64::from(per_character * text.len() as f32 * 1000.0)
    }

    /// Each frame (`00755480`): the subtitle emptied once its time is up.
    pub fn update(&mut self, ui: &mut Ui, now: f64) {
        if self.subtitle_until.is_some_and(|until| now > until) {
            self.subtitle_until = None;
            self.set_text(ui, SUBTITLE, "");
            ui.refresh();
        }
    }

    /// The pad's left stick (`00755480`, each frame while the wheel is the
    /// top menu and the pad is what's in use): XInput's state before this
    /// frame's read (`011d8a54`) and now (`011d8a6c`), each (x, y) with y
    /// up. When the earlier y or the present x is past the deadzone
    /// (7849, XInput's left-stick one) the earlier (x, y) gives an angle
    /// as the pointer's would ([`menu::slice_angle`] of x and −y: 0 up,
    /// clockwise), and `00755c50` moves it out of the gaps above and below
    /// (0 to 30° → 0.55, 150 to 180° → 2.6, 180 to 210° → 3.7, from 330°
    /// → 5.7 radians) and chooses the radial tile (of ids 0 to 15) whose
    /// `user2` ≤ angle < `user3`.
    ///
    /// Translated from 00755480, 00755c50 (decompiled, FalloutNV.exe
    /// 1.4.0.525).
    pub fn stick(&mut self, ui: &mut Ui, before: (i16, i16), now: (i16, i16)) {
        if self.closed
            || (i32::from(before.1).abs() <= STICK_DEADZONE
                && i32::from(now.0).abs() <= STICK_DEADZONE)
        {
            return;
        }
        let angle = menu::slice_angle(f32::from(before.0), -f32::from(before.1));
        let angle = stick_angle(angle);
        for id in 0..TILE_COUNT as i32 {
            let Some(tile) = self.tile(id) else { continue };
            if ui.tiles[tile].kind != crate::names::kind::RADIAL {
                continue;
            }
            let (from, to) = (ui.number(tile, t::USER0 + 2), ui.number(tile, t::USER0 + 3));
            if from <= angle && angle < to {
                self.choose(ui, id, false);
                return;
            }
        }
    }

    /// Closes it (`00755270`); `Talk` follows when asked for (`00754c60`).
    pub fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        self.requests.push(Request::Close);
        if self.talk {
            self.requests.push(Request::Talk);
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }

    fn flip(&mut self, ui: &mut Ui, id: i32) {
        match id {
            AGGRESSIVE => {
                self.aggressive = !self.aggressive;
                self.requests.push(Request::Topic(if self.aggressive {
                    "FollowersTacticsCombatAggressive"
                } else {
                    "FollowersTacticsCombatPassive"
                }));
                let v = if self.aggressive { 1.0 } else { 0.0 };
                self.requests
                    .push(Request::Variable("FollowerSwitchAggressive", v));
                self.alternate(ui, AGGRESSIVE, self.aggressive);
            }
            WAIT => {
                self.following = !self.following;
                self.requests.push(Request::Topic(if self.following {
                    "FollowersLetsGo"
                } else {
                    "FollowersWait"
                }));
                self.alternate(ui, WAIT, !self.following);
            }
            DISTANCE => {
                self.keep_distance = !self.keep_distance;
                self.requests.push(Request::Topic(if self.keep_distance {
                    "FollowersTacticsDistanceLong"
                } else {
                    "FollowersTacticsDistanceDefault"
                }));
                self.alternate(ui, DISTANCE, self.keep_distance);
            }
            RANGED => {
                self.ranged = !self.ranged;
                self.requests.push(Request::Topic(if self.ranged {
                    "FollowersTacticsCombatRanged"
                } else {
                    "FollowersTacticsCombatMelee"
                }));
                self.alternate(ui, RANGED, self.ranged);
            }
            _ => {}
        }
    }
}

impl MenuCode for CompanionWheelMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..TILE_COUNT as i32).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `007552e0`.
    fn click(&mut self, ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        if self.closed {
            return;
        }
        let ok = || "UIMenuOK".to_string();
        match id {
            AGGRESSIVE | WAIT | DISTANCE | RANGED => {
                self.flip(ui, id);
                self.choose(ui, id, true);
                self.sounds.push(ok());
            }
            STIMPAK => {
                self.requests.push(Request::Stimpak);
                self.choose(ui, id, true);
                self.sounds.push(ok());
            }
            TALK => {
                self.talk = true;
                self.close(ui);
                self.sounds.push(ok());
            }
            BACK_UP => {
                self.requests.push(Request::BackUp);
                self.close(ui);
                self.sounds.push(ok());
            }
            INVENTORY => {
                self.close(ui);
                self.requests.push(Request::Topic("FollowersTrade"));
                self.sounds.push(ok());
            }
            EXIT => {
                self.sounds.push(ok());
                self.close(ui);
            }
            _ => {}
        }
    }

    /// `00755760`.
    fn mouseover(&mut self, ui: &mut Ui, id: i32, _tile: TileId) {
        if (0..8).contains(&id) || id == EXIT {
            self.choose(ui, id, false);
        }
    }

    /// `007556b0`.
    fn special_key(&mut self, ui: &mut Ui, code: i32, now: f64) -> bool {
        match code {
            menu::special::A => {
                let id = if self.selected == -1 {
                    EXIT
                } else {
                    self.selected
                };
                let tile = self.tile(id);
                self.click(ui, id, tile, now);
                true
            }
            LEAVE => {
                self.click(ui, EXIT, self.tile(EXIT), now);
                true
            }
            _ => false,
        }
    }
}

/// The B button's code: Exit (`007556b0`).
pub const LEAVE: i32 = 10;

/// The left stick's deadzone the wheel uses (`00755480`: 0x1ea9).
pub const STICK_DEADZONE: i32 = 7849;

/// `00755c50`: a stick's angle (radians, 0 up, clockwise) moved out of the
/// gaps between the wheel's halves to their nearest slice.
#[allow(clippy::approx_constant)]
pub fn stick_angle(angle: f32) -> f32 {
    if (0.0..0.523598).contains(&angle) {
        0.55
    } else if (2.61799..3.14159).contains(&angle) {
        2.6
    } else if (3.14159..3.665186).contains(&angle) {
        3.7
    } else if angle >= 5.759578 {
        5.7
    } else {
        angle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;
    use world::companions::Switches;

    fn opened(switches: Switches) -> (Ui, CompanionWheelMenu) {
        let mut ui = test_support::ui();
        let mut m = CompanionWheelMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::companion_wheel_menu(), &mut m);
        let facts = Facts {
            health: 75,
            max_health: 150,
            stimpaks: 3,
            carried: 40,
            carry_limit: 150,
            weapon: Some("Hunting Rifle".into()),
        };
        assert!(m.open(&mut ui, switches, facts));
        (ui, m)
    }

    fn text(ui: &mut Ui, m: &CompanionWheelMenu, id: i32) -> String {
        ui.string(m.tiles[id as usize].unwrap(), t::STRING)
            .unwrap_or_default()
    }

    fn number(ui: &mut Ui, m: &CompanionWheelMenu, id: i32, trait_id: i32) -> f32 {
        ui.number(m.tiles[id as usize].unwrap(), trait_id)
    }

    /// `00754de0`: the switches on the buttons, the strings, the sound.
    #[test]
    fn opening() {
        let (mut ui, m) = opened(Switches {
            aggressive: true,
            waiting: true,
            keep_distance: false,
            ranged: false,
        });
        assert_eq!(number(&mut ui, &m, AGGRESSIVE, ALTERNATE), 1.0);
        assert_eq!(number(&mut ui, &m, WAIT, ALTERNATE), 1.0);
        assert_eq!(number(&mut ui, &m, DISTANCE, ALTERNATE), 0.0);
        assert_eq!(number(&mut ui, &m, RANGED, ALTERNATE), 0.0);
        assert!(!m.following);
        assert_eq!(text(&mut ui, &m, TITLE), "Companion Commands");
        assert_eq!(text(&mut ui, &m, 14), "Select");
        assert_eq!(m.selected, -1);
        assert_eq!(m.sounds, vec!["UIPopUpMessageGeneral"]);
    }

    /// `007557d0`, `00755dc0`: the pointer on slices; the previews, the
    /// highlight moving, a switch previewing its other state, the context
    /// lines.
    #[test]
    fn choosing() {
        let (mut ui, mut m) = opened(Switches::default());
        let tile = |m: &CompanionWheelMenu, id: i32| m.tiles[id as usize].unwrap();
        m.mouseover(&mut ui, AGGRESSIVE, tile(&m, AGGRESSIVE));
        assert_eq!(text(&mut ui, &m, PREVIEW), "Be Aggressive");
        assert_eq!(number(&mut ui, &m, AGGRESSIVE, HIGHLIGHT), 1.0);
        assert_eq!(number(&mut ui, &m, AGGRESSIVE, ALTERNATE), 1.0);
        m.mouseover(&mut ui, STIMPAK, tile(&m, STIMPAK));
        assert_eq!(number(&mut ui, &m, AGGRESSIVE, HIGHLIGHT), 0.0);
        assert_eq!(number(&mut ui, &m, AGGRESSIVE, ALTERNATE), 0.0);
        assert_eq!(text(&mut ui, &m, PREVIEW), "Use Stimpak");
        assert_eq!(text(&mut ui, &m, CONTEXT), "HP  75/150\nStimpak  3");
        m.mouseover(&mut ui, INVENTORY, tile(&m, INVENTORY));
        assert_eq!(text(&mut ui, &m, CONTEXT), "Wg  40/150");
        m.mouseover(&mut ui, RANGED, tile(&m, RANGED));
        assert_eq!(text(&mut ui, &m, PREVIEW), "Use Ranged");
        assert_eq!(text(&mut ui, &m, CONTEXT), "Hunting Rifle");
        // Following: a click would make them wait (the waiting picture).
        m.mouseover(&mut ui, WAIT, tile(&m, WAIT));
        assert_eq!(text(&mut ui, &m, PREVIEW), "Wait Here");
        assert_eq!(number(&mut ui, &m, WAIT, ALTERNATE), 1.0);
        // The same slice again: nothing new.
        let sounds = m.sounds.len();
        m.mouseover(&mut ui, WAIT, tile(&m, WAIT));
        assert_eq!(m.sounds.len(), sounds);
    }

    /// The pointer picks a slice by its angle and distance from the
    /// screen's centre (`radial`, `00a216b0`): up and right of it slice 0,
    /// down and left slice 4; straight up (between the slices) or too near
    /// the middle, none.
    #[test]
    fn pointing_at_slices() {
        let (mut ui, m) = opened(Switches::default());
        let at = |ui: &mut Ui, angle: f32, r: f32| {
            let (x, y) = (960.0 + r * angle.sin(), 540.0 - r * angle.cos());
            menu::pick(ui, m.menu, x, y)
        };
        let slice = |id: i32| m.tiles[id as usize];
        assert_eq!(at(&mut ui, 0.8, 200.0), slice(0));
        assert_eq!(at(&mut ui, 3.9, 200.0), slice(4));
        assert_eq!(at(&mut ui, 1.3, 300.0), slice(1));
        let none = |t: Option<TileId>| !m.tiles[..8].contains(&t);
        assert!(none(at(&mut ui, 0.1, 200.0)));
        assert!(none(at(&mut ui, 0.8, 40.0)));
    }

    /// The pad's stick (`00755480`, `00755c50`): pushed up and right,
    /// slice 0; down and left, slice 4; straight up is moved into slice 0
    /// (0.55 rad), straight down into slice 4's side (3.7); within the
    /// deadzone nothing; only the earlier frame's y and the present x are
    /// looked at for that.
    #[test]
    fn choosing_with_the_stick() {
        let (mut ui, mut m) = opened(Switches::default());
        let s = |a: f32, r: f32| ((r * a.sin()) as i16, (r * a.cos()) as i16);
        m.stick(&mut ui, s(0.8, 20000.0), (0, 0));
        assert_eq!(m.selected, AGGRESSIVE);
        m.stick(&mut ui, s(3.9, 20000.0), (0, 0));
        assert_eq!(m.selected, BACK_UP);
        m.stick(&mut ui, (0, 20000), (0, 0));
        assert_eq!(m.selected, AGGRESSIVE);
        m.stick(&mut ui, (0, -20000), (0, 0));
        assert_eq!(m.selected, BACK_UP);
        // Right, but only x past the deadzone in the earlier frame and the
        // present x not: nothing.
        m.stick(&mut ui, (20000, 0), (0, 0));
        assert_eq!(m.selected, BACK_UP);
        // With the present x past it, the earlier (x, y) counts: straight
        // right is 1.57075, just inside slice 1 (to 1.5707943).
        m.stick(&mut ui, (20000, 0), (20000, 0));
        assert_eq!(m.selected, STIMPAK);
        assert_eq!(stick_angle(0.1), 0.55);
        assert_eq!(stick_angle(6.0), 5.7);
        assert_eq!(stick_angle(1.0), 1.0);
    }

    /// `007552e0` and its buttons; `007556b0`'s keys.
    #[test]
    fn clicking() {
        let (mut ui, mut m) = opened(Switches {
            waiting: false,
            ..Switches::default()
        });
        let tile = |m: &CompanionWheelMenu, id: i32| m.tiles[id as usize];
        m.click(&mut ui, AGGRESSIVE, tile(&m, AGGRESSIVE), 0.0);
        assert_eq!(
            m.requests,
            vec![
                Request::Topic("FollowersTacticsCombatAggressive"),
                Request::Variable("FollowerSwitchAggressive", 1.0)
            ]
        );
        assert!(m.aggressive);
        // Shown again: now a click would make them passive.
        assert_eq!(text(&mut ui, &m, PREVIEW), "Be Passive");
        assert_eq!(m.sounds.last().map(String::as_str), Some("UIMenuOK"));
        m.requests.clear();
        m.click(&mut ui, WAIT, tile(&m, WAIT), 0.0);
        assert_eq!(m.requests, vec![Request::Topic("FollowersWait")]);
        assert_eq!(number(&mut ui, &m, WAIT, ALTERNATE), 0.0);
        assert_eq!(text(&mut ui, &m, PREVIEW), "Follow Me");
        m.requests.clear();
        m.click(&mut ui, STIMPAK, tile(&m, STIMPAK), 0.0);
        assert_eq!(m.requests, vec![Request::Stimpak]);
        m.requests.clear();
        // Enter clicks the chosen slice.
        m.special_key(&mut ui, menu::special::A, 0.0);
        assert_eq!(m.requests, vec![Request::Stimpak]);
        m.requests.clear();
        // Open Inventory: closed, then the trade topic.
        m.click(&mut ui, INVENTORY, tile(&m, INVENTORY), 0.0);
        assert_eq!(
            m.requests,
            vec![Request::Close, Request::Topic("FollowersTrade")]
        );
        assert!(m.closed);
    }

    /// Talk To closes it and the talk starts after (`00756960`,
    /// `00754c60`); B is Exit.
    #[test]
    fn talking_and_leaving() {
        let (mut ui, mut m) = opened(Switches::default());
        m.click(&mut ui, TALK, m.tiles[TALK as usize], 0.0);
        assert_eq!(m.requests, vec![Request::Close, Request::Talk]);
        let (mut ui, mut m) = opened(Switches::default());
        assert!(m.special_key(&mut ui, LEAVE, 0.0));
        assert_eq!(m.requests, vec![Request::Close]);
        // With nothing chosen Enter is Exit too.
        let (mut ui, mut m) = opened(Switches::default());
        m.special_key(&mut ui, menu::special::A, 0.0);
        assert_eq!(m.requests, vec![Request::Close]);
    }

    /// `00757690`, `00755480`: a line on the subtitle, emptied later.
    #[test]
    fn subtitles() {
        let (mut ui, mut m) = opened(Switches::default());
        let until = 1000.0 + CompanionWheelMenu::line_time("Much better.", TIME_PER_CHARACTER);
        assert!((until - 1720.0).abs() < 1e-3);
        m.say(&mut ui, "Much better.", until);
        assert_eq!(text(&mut ui, &m, SUBTITLE), "Much better.");
        m.update(&mut ui, 1500.0);
        assert_eq!(text(&mut ui, &m, SUBTITLE), "Much better.");
        m.update(&mut ui, 1800.0);
        assert_eq!(text(&mut ui, &m, SUBTITLE), "");
    }
}
