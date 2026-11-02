//! The player's control bindings as the game reads them: the INI's
//! `[Controls]` (`FalloutPrefs.ini` keeps them), one value per control,
//! eight hex digits: the keyboard's DirectInput key number in the second
//! byte from the left, the mouse button in the third (0 left, 1 right, 2
//! middle; FF none), the controller's in the last. `Block=00380110`: Left
//! Alt or the right mouse button. When a control isn't in the INI, the
//! default is the exe's own (`00a24b70`, [`exe_default`]).
//!
//! The controls' INI names are the exe's table at `011a7e60` (28 of them,
//! [`NAMES`]); FNV has no "Hotkey2": control 18 is "Ammo Swap", on the 2
//! key. Every control is looked up on its own (`00a24660` reads the
//! control's key from the keyboard map at the input manager's `+0x1b94`,
//! then that key's state), so two controls bound to one key both act on
//! it; each user of a control decides what it does. The 2 key (control
//! 0x12) is read by both the player's controls (`0093e860`: going down,
//! the ammunition swap) and the HUD's hot keys (`0077da60`: slot 1, which
//! the Pip-Boy never lets anything onto, so it does nothing; held, it
//! never shows the wheel), tap or hold alike.

use bevy::prelude::*;

/// The controls' INI names by number (`011a7e60` in FalloutNV.exe
/// 1.4.0.525; `00a22e10` writes them to `FalloutPrefs.ini`).
pub const NAMES: [&str; 28] = [
    "Forward",
    "Back",
    "Slide Left",
    "Slide Right",
    "Use",
    "Activate",
    "Block",
    "Ready Item",
    "Crouch/Sneak",
    "Run",
    "Always Run",
    "Auto Move",
    "Jump",
    "Toggle POV",
    "Menu Mode",
    "Rest",
    "Vats",
    "Hotkey1",
    "Ammo Swap",
    "Hotkey3",
    "Hotkey4",
    "Hotkey5",
    "Hotkey6",
    "Hotkey7",
    "Hotkey8",
    "QuickSave",
    "QuickLoad",
    "Grab",
];

/// The first hot key control (Hotkey1, 0x11); hot key n is control
/// 0x11 + n, n = 1 being Ammo Swap.
pub const FIRST_HOTKEY: usize = 0x11;

/// A control's default as the INI writes it (keyboard key << 16 | mouse
/// button << 8 | controller button, 0xFF none).
// Translated from 00a24b70 (decompiled, FalloutNV.exe 1.4.0.525): every
// device's map filled with 0xFF, then the keyboard's, the mouse's and the
// controller's defaults written byte by byte.
pub fn exe_default(control: usize) -> u32 {
    const KEYS: [u8; 28] = [
        0x11, 0x1f, 0x1e, 0x20, 0xff, 0x12, 0x38, 0x13, 0x1d, 0x2a, 0x3a, 0x10, 0x39, 0x21, 0x0f,
        0x14, 0x2f, 2, 3, 4, 5, 6, 7, 8, 9, 0x3f, 0x43, 0x2c,
    ];
    let mouse = match control {
        4 => 0,
        6 => 1,
        13 => 2,
        _ => 0xff,
    };
    let pad = match control {
        0 => 0x13,
        1 => 0x14,
        2 => 0x17,
        3 => 0x16,
        4 => 0x11,
        5 => 10,
        6 => 0x10,
        7 => 0xc,
        8 => 8,
        12 => 0xd,
        13 => 0xf,
        14 => 0xb,
        15 => 7,
        16 => 0xe,
        18 => 1,
        27 => 9,
        _ => 0xff,
    };
    let key = KEYS.get(control).copied().unwrap_or(0xff);
    (u32::from(key) << 16) | (mouse << 8) | pad
}

/// A control's keyboard key and mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Binding {
    pub key: Option<KeyCode>,
    pub mouse: Option<MouseButton>,
}

impl Binding {
    /// From the INI's value.
    pub fn from_value(value: u32) -> Binding {
        Binding {
            key: crate::lockpick::scan_code_key((value >> 16) & 0xFF),
            mouse: match (value >> 8) & 0xFF {
                0 => Some(MouseButton::Left),
                1 => Some(MouseButton::Right),
                2 => Some(MouseButton::Middle),
                _ => None,
            },
        }
    }

    pub fn pressed(&self, keys: &ButtonInput<KeyCode>, mouse: &ButtonInput<MouseButton>) -> bool {
        self.key.is_some_and(|k| keys.pressed(k)) || self.mouse.is_some_and(|m| mouse.pressed(m))
    }

    pub fn just_pressed(
        &self,
        keys: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        self.key.is_some_and(|k| keys.just_pressed(k))
            || self.mouse.is_some_and(|m| mouse.just_pressed(m))
    }
}

/// The controls the player's actions here read.
#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct Controls {
    /// Control 6, "Block" in the INI: aim (iron sights) or block.
    pub aim: Binding,
    /// Control 8, "Crouch/Sneak".
    pub sneak: Binding,
    /// Control 10, "Always Run": toggles running by default.
    pub always_run: Binding,
    /// Control 11, "Auto Move".
    pub auto_move: Binding,
    /// Control 18, "Ammo Swap" (the 2 key here; `world::ammo_swap`).
    pub ammo_swap: Binding,
    /// Control 4, "Use": attacking.
    pub attack: Binding,
    /// Control 27, "Grab": pick up and carry clutter (`clutter`).
    pub grab: Binding,
    /// `bAlwaysRunByDefault` (the player's +0x651 at the start).
    pub always_run_default: bool,
    /// The hot key wheel's eight controls, 0x11 .. 0x18 (Hotkey1, Ammo
    /// Swap, Hotkey3 .. Hotkey8): what the Pip-Boy's wheel and the hot
    /// keys read (`00781ba0`, `0077da60`).
    pub hotkeys: [Binding; 8],
}

impl Default for Controls {
    fn default() -> Self {
        let d = |control: usize| Binding::from_value(exe_default(control));
        Controls {
            aim: d(6),
            sneak: d(8),
            always_run: d(10),
            auto_move: d(11),
            ammo_swap: d(18),
            attack: d(4),
            grab: d(27),
            always_run_default: true,
            hotkeys: std::array::from_fn(|n| d(FIRST_HOTKEY + n)),
        }
    }
}

impl Controls {
    /// From the game's INI files, the exe's defaults where they're
    /// silent.
    pub fn read(settings: &assets::IniSettings) -> Controls {
        let base = Controls::default();
        let binding = |control: usize, default: Binding| {
            settings
                .get("Controls", NAMES[control])
                .and_then(|v| u32::from_str_radix(v.trim(), 16).ok())
                .map_or(default, Binding::from_value)
        };
        Controls {
            aim: binding(6, base.aim),
            sneak: binding(8, base.sneak),
            always_run: binding(10, base.always_run),
            auto_move: binding(11, base.auto_move),
            ammo_swap: binding(18, base.ammo_swap),
            attack: binding(4, base.attack),
            grab: binding(27, base.grab),
            always_run_default: settings
                .get("Controls", "bAlwaysRunByDefault")
                .map_or(base.always_run_default, |v| v.trim() != "0"),
            hotkeys: std::array::from_fn(|n| binding(FIRST_HOTKEY + n, base.hotkeys[n])),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_are_read_as_the_ini_writes_them() {
        // This install's Block: Left Alt and the right mouse button.
        let aim = Binding::from_value(0x0038_0110);
        assert_eq!(aim.key, Some(KeyCode::AltLeft));
        assert_eq!(aim.mouse, Some(MouseButton::Right));
        let sneak = Binding::from_value(0x001D_FF08);
        assert_eq!((sneak.key, sneak.mouse), (Some(KeyCode::ControlLeft), None));
        assert_eq!(
            Binding::from_value(0x003A_FFFF).key,
            Some(KeyCode::CapsLock)
        );
        let mut ini = assets::IniSettings::default();
        ini.add("[Controls]\nCrouch/Sneak=002EFF08\nbAlwaysRunByDefault=0\n");
        let c = Controls::read(&ini);
        assert_eq!(c.sneak.key, Some(KeyCode::KeyC));
        assert!(!c.always_run_default);
        assert_eq!(c.aim, Controls::default().aim);
        // Ammo Swap: the 2 key (DirectInput 3); Use: the left button.
        assert_eq!(c.ammo_swap.key, Some(KeyCode::Digit2));
        assert_eq!(c.attack.mouse, Some(MouseButton::Left));
        // This install's Grab: Z.
        assert_eq!(Controls::default().grab.key, Some(KeyCode::KeyZ));
    }

    /// `00a24b70`'s defaults are what an untouched INI holds; the hot key
    /// wheel's second control is Ammo Swap, so it follows that binding.
    #[test]
    fn the_exe_defaults_and_the_hot_keys() {
        assert_eq!(exe_default(6), 0x0038_0110);
        assert_eq!(exe_default(4), 0x00FF_0011);
        assert_eq!(exe_default(8), 0x001D_FF08);
        assert_eq!(exe_default(18), 0x0003_FF01);
        assert_eq!(exe_default(17), 0x0002_FFFF);
        assert_eq!(exe_default(25), 0x003F_FFFF);
        assert_eq!(NAMES[18], "Ammo Swap");
        let c = Controls::default();
        assert_eq!(c.hotkeys[0].key, Some(KeyCode::Digit1));
        assert_eq!(c.hotkeys[1], c.ammo_swap);
        assert_eq!(c.hotkeys[7].key, Some(KeyCode::Digit8));
        let mut ini = assets::IniSettings::default();
        ini.add("[Controls]\nAmmo Swap=0013FF01\nHotkey3=0003FFFF\n");
        let c = Controls::read(&ini);
        assert_eq!(c.ammo_swap.key, Some(KeyCode::KeyR));
        assert_eq!(c.hotkeys[1].key, Some(KeyCode::KeyR));
        assert_eq!(c.hotkeys[2].key, Some(KeyCode::Digit2));
    }
}
