//! The world behind the menus: captured once as a menu opens, with one of
//! four image space modifiers (`IMAD`) applied, and shown, still, until the
//! menus close (FalloutNV.exe 1.4.0.525).
//!
//! - Each frame the render path (`0086e650`) works out "menu mode"
//!   (`011dea2b`: a menu-mode menu is up, `00702360`, or the Pip-Boy is
//!   coming up, its state 2, `00709bc0`) and calls `0086f450`, which
//!   captures the background ([`Background::update`]) through `00871dc0`
//!   when `bStaticMenuBackground:Display` is on (default 1: `00f40570`,
//!   read once at start into `011dea28`) and none of the exceptions hold
//!   ([`captures`]).
//! - `00871dc0` draws the world once with the modifier [`modifier`] picks
//!   (`00718ab0`) applied at strength 1 (`005299a0(imad, 1.0, 0)`), takes
//!   it away again (`00529c90`) and marks the background captured
//!   (`011dea29`). The image space manager shows the picture from then on
//!   (its effect 15); the world isn't drawn under the menus.
//! - Once captured it stays while menu mode lasts, and also while menus
//!   that draw their own scene over it are open (the Vit-o-matic Vigor
//!   Tester 1074, the lock 1014, …); otherwise it is let go (`00877430`:
//!   effect 15 off, `011dea29` cleared) ([`releases`]).
//!
//! The four modifiers (`FalloutNV.esm`; all with flags 0, so not played
//! over time: their first keys hold):
//!
//! | form | editor ID | when | what it sets |
//! |---|---|---|---|
//! | `0004EEE8` | `PauseBackgroundFX` | the pause menu | blur 3, saturation ×0, tracks 18 ×0.8 and 19 ×1.3, tint (0.67, 0.66, 0.24) at 0.59, depth of field 1 |
//! | `00096389` | `PipBackgroundFX` | the Pip-Boy | depth of field 0.7 |
//! | `00044F34` | `InterfaceBackgroundFX` | the lock (1014) on top | blur 2, bright clamp ×2, saturation ×0.1, brightness ×0.5, tint (0.33, 0.58, 0.44) at 0.78 |
//! | `00032B38` | `PopupBackgroundFX` | any other menu | blur 3 |
//!
//! The blur ([`blur_pass`], [`blur_weights`]): the image space manager
//! keeps the largest blur of the modifiers playing (`00b8ccb0`, `+0x25c`,
//! handed to the blur effect as `012003d0`); the blur effect (vtable
//! `010b8000`, `00ba4d20`) draws one of its seven blur passes, the one for
//! radius `ceil(blur)` (`00ec9e10`), 1 to 7; a larger blur draws none. That
//! pass (`00ba4270`) is the game's two-pass blur `ISBLUR(2r+1)`: down then
//! across, taps −r … r texels apart, weighted by the exe's table of seven
//! rows (`011ade38` + row × 0xf0), mixed between row r−1 (at least 1) and
//! row r by `1 − (r − blur)`. The rows are a Gaussian with σ = r / 2
//! normalized over the taps (checked against every row).
//!
//! Not here: a blur under 1 (passes 9 and 10 of the effect, a blend by the
//! blur; none of these modifiers has one), depth of field, and the Pip-Boy's
//! capture (made by other callers of `00871dc0`, `007cbaf0` and `007ce7a0`,
//! not traced; `0086f450` itself never captures with the Pip-Boy up).

use esm::FormId;

/// The four modifiers (`00718ab0`).
pub mod form {
    pub const INTERFACE: u32 = 0x0004_4F34;
    pub const POPUP: u32 = 0x0003_2B38;
    pub const PIP: u32 = 0x0009_6389;
    pub const PAUSE: u32 = 0x0004_EEE8;
}

/// Menu class numbers these tests name.
pub mod menu {
    pub const MESSAGE: u16 = 1001;
    pub const DIALOG: u16 = 1009;
    pub const LOCKPICK: u16 = 1014;
    pub const VATS: u16 = 1056;
    pub const LOVE_TESTER: u16 = 1074;
}

/// Menus that keep a captured background though menu mode has ended
/// (`0086f450`'s second test: 1054, 1014, 1060, 1074, 1080–1083).
pub const KEEPING: [u16; 8] = [
    1054,
    menu::LOCKPICK,
    1060,
    menu::LOVE_TESTER,
    1080,
    1081,
    1082,
    1083,
];

/// The setting's default (`00f40570`: `bStaticMenuBackground:Display`, 1).
pub const STATIC_BACKGROUND_DEFAULT: bool = true;

/// What the tests read about the menus, one frame.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MenuState {
    /// `bStaticMenuBackground:Display` (`011dea28`).
    pub setting: bool,
    /// Menu mode (`011dea2b`).
    pub menu_mode: bool,
    /// The class of the menu on top (`007023c0`).
    pub top: Option<u16>,
    /// The classes of the menus open and not hidden (`00702680` with mask
    /// 0xb: the menu's fade state, `+0x24`, is shown 1, fading out 2 or
    /// fading in 8).
    pub shown: Vec<u16>,
    /// The pause menu is up (`004a4040`: the start menu with its pause
    /// flag).
    pub pause_menu: bool,
    /// The start menu is up without its pause flag: the main menu
    /// (`0070edf0`).
    pub main_menu: bool,
    /// The Pip-Boy is up or coming up (`00705a00`: its state 2 or 3).
    pub pipboy: bool,
    /// A tile named "Player Name Entry Menu" exists (`00a03da0`).
    pub name_entry: bool,
    /// `00703d50`: an object at `011d8ce8` (0x914 bytes) counts above 0 at
    /// `+0x38`; not identified.
    pub held: bool,
    /// The fader manager's fader 1 is running (`007014a0(1)`).
    pub fader_1: bool,
}

impl MenuState {
    fn is_shown(&self, class: u16) -> bool {
        self.shown.contains(&class)
    }
}

/// The modifier for the background (`00718ab0`): the pause menu's, else
/// the Pip-Boy's (`00967ae0`, whose other case, something the player has
/// at `+0x690` in states 0xe2 or 0xf0, isn't traced), else the lock's when
/// it is on top, else the popups', unless the name entry is up.
pub fn modifier(m: &MenuState) -> Option<FormId> {
    let id = if m.pause_menu {
        form::PAUSE
    } else if m.pipboy {
        form::PIP
    } else if m.top == Some(menu::LOCKPICK) {
        form::INTERFACE
    } else if !m.name_entry {
        form::POPUP
    } else {
        return None;
    };
    Some(FormId(id))
}

/// Whether the background is captured now (`0086f450`, first test): in
/// menu mode, with the setting on and nothing captured yet, unless the
/// dialogue menu is on top, the main menu is up, V.A.T.S.'s menu or a
/// message box is shown, the Pip-Boy is up or `00703d50` holds.
pub fn captures(m: &MenuState, captured: bool) -> bool {
    m.menu_mode
        && m.top != Some(menu::DIALOG)
        && !m.main_menu
        && !m.is_shown(menu::VATS)
        && !m.is_shown(menu::MESSAGE)
        && !m.pipboy
        && !m.held
        && !captured
        && m.setting
}

/// Whether a captured background is let go now (`0086f450`, second
/// test): not while menu mode lasts (without the main menu, `00703d50`
/// or the dialogue and V.A.T.S. menus on top), nor while fader 1 runs or a
/// menu of [`KEEPING`] is shown.
pub fn releases(m: &MenuState) -> bool {
    // `00703d50` holding is let through when `007079b0` says so (the
    // Pip-Boy's own test); with it never holding here that isn't needed.
    let kept = m.menu_mode
        && !m.main_menu
        && !m.held
        && m.top != Some(menu::DIALOG)
        && m.top != Some(menu::VATS);
    !kept && !m.fader_1 && !KEEPING.iter().any(|&c| m.is_shown(c))
}

/// The background's state across frames.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Background {
    /// Captured (`011dea29`).
    pub captured: bool,
    /// The modifier it was captured with.
    pub modifier: Option<FormId>,
}

/// What a frame did to the background.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    None,
    Captured,
    Released,
}

impl Background {
    /// One frame of `0086f450`.
    pub fn update(&mut self, m: &MenuState) -> Change {
        if captures(m, self.captured) {
            self.captured = true;
            self.modifier = modifier(m);
            return Change::Captured;
        }
        if self.captured && releases(m) {
            *self = Background::default();
            return Change::Released;
        }
        Change::None
    }
}

/// The blur pass drawn for a blur value (`00ba4d20`): radius
/// `ceil(blur)`, when it is one of the passes 1 to 7. Under 1 not drawn
/// here (see the module notes).
pub fn blur_pass(blur: f32) -> Option<u32> {
    // (A NaN blur draws nothing either.)
    if blur.is_nan() || blur < 1.0 {
        return None;
    }
    let r = blur.ceil();
    (r <= 7.0).then_some(r as u32)
}

/// A row of the exe's blur weight table (`011ade38` + r × 0xf0, rows 1 to
/// 7): taps −7 … 7, a Gaussian with σ = r / 2 over the taps −r … r,
/// normalized, 0 past them.
pub fn weight_row(r: u32) -> [f32; 15] {
    let r = r.clamp(1, 7) as i32;
    let sigma = r as f32 / 2.0;
    let g = |k: i32| (-((k * k) as f32) / (2.0 * sigma * sigma)).exp();
    let total: f32 = (-r..=r).map(g).sum();
    std::array::from_fn(|i| {
        let k = i as i32 - 7;
        if k.abs() <= r {
            g(k) / total
        } else {
            0.0
        }
    })
}

/// The pass's radius and its weights for taps −7 … 7 (`00ba4270`): rows
/// `max(r − 1, 1)` and `r` mixed by `1 − (r − blur)` (a mix of 0 counts as
/// 1).
pub fn blur_weights(blur: f32) -> Option<(u32, [f32; 15])> {
    let r = blur_pass(blur)?;
    let mut f = 1.0 - (r as f32 - blur);
    if f == 0.0 {
        f = 1.0;
    }
    let (low, high) = (weight_row(r.saturating_sub(1).max(1)), weight_row(r));
    Some((r, std::array::from_fn(|i| low[i] + (high[i] - low[i]) * f)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn menus(top: u16) -> MenuState {
        MenuState {
            setting: true,
            menu_mode: true,
            top: Some(top),
            shown: vec![top],
            ..MenuState::default()
        }
    }

    #[test]
    fn each_menu_gets_its_modifier() {
        // The Vigor Tester: the popups' blur.
        assert_eq!(
            modifier(&menus(menu::LOVE_TESTER)),
            Some(FormId(form::POPUP))
        );
        assert_eq!(
            modifier(&menus(menu::LOCKPICK)),
            Some(FormId(form::INTERFACE))
        );
        let mut pause = menus(1013);
        pause.pause_menu = true;
        assert_eq!(modifier(&pause), Some(FormId(form::PAUSE)));
        let mut pip = menus(1002);
        pip.pipboy = true;
        assert_eq!(modifier(&pip), Some(FormId(form::PIP)));
        // The pause menu wins over the Pip-Boy (`00718ab0` asks it first).
        pip.pause_menu = true;
        assert_eq!(modifier(&pip), Some(FormId(form::PAUSE)));
        let mut naming = menus(1051);
        naming.name_entry = true;
        assert_eq!(modifier(&naming), None);
    }

    #[test]
    fn some_menus_never_capture() {
        let mut bg = Background::default();
        // Dialogue on top, a message box, V.A.T.S., the main menu, the
        // Pip-Boy, the setting off: nothing.
        for m in [
            menus(menu::DIALOG),
            menus(menu::MESSAGE),
            menus(menu::VATS),
            MenuState {
                main_menu: true,
                ..menus(1013)
            },
            MenuState {
                pipboy: true,
                ..menus(1002)
            },
            MenuState {
                setting: false,
                ..menus(menu::LOVE_TESTER)
            },
            MenuState {
                menu_mode: false,
                ..menus(menu::LOVE_TESTER)
            },
        ] {
            assert_eq!(bg.update(&m), Change::None, "{m:?}");
        }
        assert!(!bg.captured);
    }

    #[test]
    fn captured_once_then_held_until_the_menus_close() {
        let mut bg = Background::default();
        let tester = menus(menu::LOVE_TESTER);
        assert_eq!(bg.update(&tester), Change::Captured);
        assert_eq!(bg.modifier, Some(FormId(form::POPUP)));
        // Not again while it's up.
        assert_eq!(bg.update(&tester), Change::None);
        // A message box over it keeps it.
        let mut boxed = tester.clone();
        boxed.top = Some(menu::MESSAGE);
        boxed.shown.push(menu::MESSAGE);
        assert_eq!(bg.update(&boxed), Change::None);
        assert!(bg.captured);
        // Out of menu mode the tester still keeps it (it draws over it).
        let mut scene = tester.clone();
        scene.menu_mode = false;
        assert_eq!(bg.update(&scene), Change::None);
        // Everything closed: let go.
        assert_eq!(bg.update(&MenuState::default()), Change::Released);
        assert_eq!(bg, Background::default());
        // Dialogue on top lets it go even in menu mode.
        bg.update(&menus(1002));
        assert!(bg.captured);
        let mut talk = menus(menu::DIALOG);
        talk.shown = vec![1002, menu::DIALOG];
        assert_eq!(bg.update(&talk), Change::Released);
    }

    #[test]
    fn the_blur_pass_is_the_rounded_up_radius_up_to_seven() {
        assert_eq!(blur_pass(0.0), None);
        assert_eq!(blur_pass(0.5), None);
        assert_eq!(blur_pass(1.0), Some(1));
        assert_eq!(blur_pass(2.2), Some(3));
        assert_eq!(blur_pass(3.0), Some(3));
        assert_eq!(blur_pass(7.0), Some(7));
        assert_eq!(blur_pass(7.5), None);
        assert_eq!(blur_pass(f32::NAN), None);
    }

    #[test]
    fn the_weights_match_the_exes_table() {
        // `011ade38` + r × 0xf0, the middle taps (and row 1's sides).
        let middles = [
            0.78699, 0.40262, 0.27068, 0.20416, 0.16397, 0.13702, 0.11770,
        ];
        for (i, want) in middles.into_iter().enumerate() {
            let row = weight_row(i as u32 + 1);
            assert!((row[7] - want).abs() < 2e-5, "row {} {}", i + 1, row[7]);
            assert!((row.iter().sum::<f32>() - 1.0).abs() < 1e-5);
        }
        assert!((weight_row(1)[6] - 0.10651).abs() < 2e-5);
        assert_eq!(weight_row(1)[5], 0.0);
        assert!((weight_row(3)[4] - 0.03663).abs() < 2e-5);
        assert!((weight_row(7)[0] - 0.01593).abs() < 2e-5);
        // A whole blur takes its row; between, rows r − 1 and r mix.
        let (r, w) = blur_weights(3.0).unwrap();
        assert_eq!((r, w), (3, weight_row(3)));
        let (r, w) = blur_weights(2.5).unwrap();
        assert_eq!(r, 3);
        assert!((w[7] - (0.40262 + 0.27068) / 2.0).abs() < 2e-5);
    }
}
