//! The controls' names in text: `&-sUActnForward;` and the other 27
//! action settings become the name of the key or mouse button the action
//! is bound to (FalloutNV.exe 1.4.0.525: the text pass `00a12fb0` asks
//! `007070c0` first, which finds the setting among the actions' settings
//! (`00707330`, the table at `011d51d0`) and writes the binding's name,
//! `007039b0`). Without a pad: the action's mouse button (below 9) by the
//! mouse buttons' settings (`011d5240`), else its key (below 0xEE) by the
//! keys' settings (`011d52f0`, by DirectInput key number); a key without a
//! setting, or no binding, gives nothing.
//!
//! The bindings are the exe's defaults (`00a24b70`); a player's own
//! bindings (the controls menu, kept in the INI) aren't read here.

/// The actions' settings and their defaults (`00f79710`), in control
/// order (`011d51d0`).
pub const ACTIONS: [(&str, &str); 28] = [
    ("sUActnForward", "Forward"),
    ("sUActnBack", "Back"),
    ("sUActnSldleft", "Strafe Left"),
    ("sUActnSldright", "Strafe Right"),
    ("sUActnUse", "Attack"),
    ("sUActnActivate", "Activate"),
    ("sUActnBlock", "Block"),
    ("sUActnRdyitem", "Ready/Reload"),
    ("sUActnCrouch", "Sneak"),
    ("sUActnRun", "Run"),
    ("sUActnTogglerun", "Always Run"),
    ("sUActnAutomove", "Auto Move"),
    ("sUActnJump", "Jump"),
    ("sUActnTogglepov", "Change View"),
    ("sUActnMenumode", "Pip-Boy"),
    ("sUActnRestmenu", "Wait"),
    ("sUActnVats", "VATS"),
    ("sUActnHotKey1", "Hotkey 1"),
    ("sUActnAMmoSwap", "Ammo Swap"),
    ("sUActnHotKey3", "Hotkey 3"),
    ("sUActnHotKey4", "Hotkey 4"),
    ("sUActnHotKey5", "Hotkey 5"),
    ("sUActnHotKey6", "Hotkey 6"),
    ("sUActnHotKey7", "Hotkey 7"),
    ("sUActnHotKey8", "Hotkey 8"),
    ("sUActnQuicksave", "Quicksave"),
    ("sUActnQuickload", "Quickload"),
    ("sUActnGrab", "Grab"),
];

/// The default keyboard binding of each control (`00a24b70`, device 0;
/// 0xFF none): W S A D, (Use on the mouse), E, L-Alt, R, L-Ctrl, L-Shift,
/// Caps, Q, Space, F, Tab, T, V, 1–8, F5, F9, Z.
pub const DEFAULT_KEYS: [u8; 28] = [
    0x11, 0x1F, 0x1E, 0x20, 0xFF, 0x12, 0x38, 0x13, 0x1D, 0x2A, 0x3A, 0x10, 0x39, 0x21, 0x0F, 0x14,
    0x2F, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x3F, 0x43, 0x2C,
];

/// The default mouse bindings (`00a24b70`, device 1): Use the left button,
/// Block the right, Change View the middle.
pub const DEFAULT_MOUSE: [(usize, u8); 3] = [(4, 0), (6, 1), (13, 2)];

/// The keys' settings by DirectInput key number, with their defaults
/// (`00f7a300`).
pub const KEYS: &[(u8, &str, &str)] = &[
    (0x01, "sKBEsc", "Esc"),
    (0x02, "sKB1", "1"),
    (0x03, "sKB2", "2"),
    (0x04, "sKB3", "3"),
    (0x05, "sKB4", "4"),
    (0x06, "sKB5", "5"),
    (0x07, "sKB6", "6"),
    (0x08, "sKB7", "7"),
    (0x09, "sKB8", "8"),
    (0x0A, "sKB9", "9"),
    (0x0B, "sKB0", "0"),
    (0x0C, "sKBMinus", "-"),
    (0x0D, "sKBEquals", "="),
    (0x0E, "sKBBack", "Backspace"),
    (0x0F, "sKBTab", "Tab"),
    (0x10, "sKBQ", "Q"),
    (0x11, "sKBW", "W"),
    (0x12, "sKBE", "E"),
    (0x13, "sKBR", "R"),
    (0x14, "sKBT", "T"),
    (0x15, "sKBY", "Y"),
    (0x16, "sKBU", "U"),
    (0x17, "sKBI", "I"),
    (0x18, "sKBO", "O"),
    (0x19, "sKBP", "P"),
    (0x1A, "sKBLBracket", "["),
    (0x1B, "sKBRBracket", "]"),
    (0x1C, "sKBReturn", "Enter"),
    (0x1D, "sKBLControl", "L-Ctrl"),
    (0x1E, "sKBA", "A"),
    (0x1F, "sKBS", "S"),
    (0x20, "sKBD", "D"),
    (0x21, "sKBF", "F"),
    (0x22, "sKBG", "G"),
    (0x23, "sKBH", "H"),
    (0x24, "sKBJ", "J"),
    (0x25, "sKBK", "K"),
    (0x26, "sKBL", "L"),
    (0x27, "sKBSemicolon", ";"),
    (0x28, "sKBApostrophe", "'"),
    (0x29, "sKBTilde", "~"),
    (0x2A, "sKBLShift", "L-Shift"),
    (0x2B, "sKBBackslash", "\\"),
    (0x2C, "sKBZ", "Z"),
    (0x2D, "sKBX", "X"),
    (0x2E, "sKBC", "C"),
    (0x2F, "sKBV", "V"),
    (0x30, "sKBB", "B"),
    (0x31, "sKBN", "N"),
    (0x32, "sKBM", "M"),
    (0x33, "sKBComma", ","),
    (0x34, "sKBPeriod", "."),
    (0x35, "sKBSlash", "/"),
    (0x36, "sKBRShift", "R-Shift"),
    (0x37, "sKBMultiply", "Pad *"),
    (0x38, "sKBLMenu", "L-Alt"),
    (0x39, "sKBSpace", "Space"),
    (0x3A, "sKBCapital", "Caps"),
    (0x3B, "sKBF1", "F1"),
    (0x3C, "sKBF2", "F2"),
    (0x3D, "sKBF3", "F3"),
    (0x3E, "sKBF4", "F4"),
    (0x3F, "sKBF5", "F5"),
    (0x40, "sKBF6", "F6"),
    (0x41, "sKBF7", "F7"),
    (0x42, "sKBF8", "F8"),
    (0x43, "sKBF9", "F9"),
    (0x44, "sKBF10", "F10"),
    (0x45, "sKBNumlock", "Num"),
    (0x46, "sKBScroll", "Scroll"),
    (0x47, "sKBNumpad7", "Pad 7"),
    (0x48, "sKBNumpad8", "Pad 8"),
    (0x49, "sKBNumpad9", "Pad 9"),
    (0x4A, "sKBSubtract", "Pad -"),
    (0x4B, "sKBNumpad4", "Pad 4"),
    (0x4C, "sKBNumpad5", "Pad 5"),
    (0x4D, "sKBNumpad6", "Pad 6"),
    (0x4E, "sKBAdd", "Pad +"),
    (0x4F, "sKBNumpad1", "Pad 1"),
    (0x50, "sKBNumpad2", "Pad 2"),
    (0x51, "sKBNumpad3", "Pad 3"),
    (0x52, "sKBNumpad0", "Pad 0"),
    (0x53, "sKBDecimal", "Pad ."),
    (0x56, "sKBOEM102", "<"),
    (0x57, "sKBF11", "F11"),
    (0x58, "sKBF12", "F12"),
    (0x64, "sKBF13", "F13"),
    (0x65, "sKBF14", "F14"),
    (0x66, "sKBF15", "F15"),
    (0x73, "sKBABNTC1", "/"),
    (0x7E, "sKBABNTC2", "Pad ."),
    (0x8D, "sKBNumPadEquals", "Pad ="),
    (0x90, "sKBPrevtrack", "Prev"),
    (0x91, "sKBAt", "@"),
    (0x92, "sKBColon", ":"),
    (0x93, "sKBUnderline", "_"),
    (0x95, "sKBStop", "Stop"),
    (0x97, "sKBUnlabeled", "Unlabeled"),
    (0x99, "sKBNextTrack", "Next"),
    (0x9C, "sKBNumPadEnter", "Pad Enter"),
    (0x9D, "sKBRControl", "R-Ctrl"),
    (0xA0, "sKBMute", "Mute"),
    (0xA1, "sKBCalculator", "Calc"),
    (0xA2, "sKBPlayPause", "Play"),
    (0xA4, "sKBMediaStop", "Stop"),
    (0xAE, "sKBVolumeDown", "Vol -"),
    (0xB0, "sKBVolumeUp", "Vol +"),
    (0xB2, "sKBWebHome", "Web"),
    (0xB3, "sKBNumPadComma", "Pad ,"),
    (0xB5, "sKBDivide", "Pad /"),
    (0xB7, "sKBSysRq", "PrntScrn"),
    (0xB8, "sKBRMenu", "R-Alt"),
    (0xC5, "sKBPause", "Pause"),
    (0xC7, "sKBHome", "Home"),
    (0xC8, "sKBUp", "Up"),
    (0xC9, "sKBPrior", "PgUp"),
    (0xCB, "sKBLeft", "Left"),
    (0xCD, "sKBRight", "Right"),
    (0xCF, "sKBEnd", "End"),
    (0xD0, "sKBDown", "Down"),
    (0xD1, "sKBNext", "PgDn"),
    (0xD2, "sKBInsert", "Insert"),
    (0xD3, "sKBDelete", "Delete"),
    (0xDB, "sKBLWin", "LWin"),
    (0xDC, "sKBRWin", "RWin"),
    (0xDD, "sKBApps", "Apps"),
    (0xDE, "sKBPower", "Power"),
    (0xDF, "sKBSleep", "Sleep"),
    (0xE3, "sKBWake", "Wake"),
    (0xE5, "sKBWebSearch", "Search"),
    (0xE6, "sKBWebFavorites", "Fav"),
    (0xE7, "sKBWebRefresh", "Refresh"),
    (0xE8, "sKBWebStop", "Web Stop"),
    (0xE9, "sKBWebForward", "Web ->"),
    (0xEA, "sKBWebBack", "Web <-"),
    (0xEB, "sKBMyComputer", "My Comp"),
    (0xEC, "sKBMail", "Mail"),
    (0xED, "sKBMediaSelect", "Select"),
];

/// The mouse buttons' settings, by button (`00f7dd10`, `011d5240`).
pub const MOUSE_BUTTONS: [(&str, &str); 9] = [
    ("sMouseLeftButton", "Mouse1"),
    ("sMouseRightButton", "Mouse2"),
    ("sMouseMiddleButton", "Wheel"),
    ("sMouseButton3", "Button 3"),
    ("sMouseButton4", "Button 4"),
    ("sMouseButton5", "Button 5"),
    ("sMouseButton6", "Button 6"),
    ("sMouseButton7", "Button 7"),
    ("sMouseButton8", "Button 8"),
];

/// The key setting for a DirectInput key number.
pub fn key_setting(scan: u8) -> Option<&'static str> {
    KEYS.iter().find(|(s, _, _)| *s == scan).map(|(_, n, _)| *n)
}

/// The control an action setting names (`00707330`), compared without
/// case.
pub fn action(setting: &str) -> Option<usize> {
    ACTIONS
        .iter()
        .position(|(n, _)| n.eq_ignore_ascii_case(setting))
}

/// The setting naming a control's binding (`007039b0` without a pad): its
/// mouse button, else its key, with the default bindings.
pub fn binding_setting(control: usize) -> Option<&'static str> {
    if let Some(&(_, button)) = DEFAULT_MOUSE.iter().find(|(c, _)| *c == control) {
        return MOUSE_BUTTONS.get(usize::from(button)).map(|(n, _)| *n);
    }
    let key = *DEFAULT_KEYS.get(control)?;
    if key < 0xEE {
        key_setting(key)
    } else {
        None
    }
}

/// What `&-sUActn…;` becomes: the binding's setting's text (`setting`
/// looks settings up), or nothing.
pub fn action_text(control: usize, setting: &dyn Fn(&str) -> Option<String>) -> String {
    binding_setting(control)
        .and_then(setting)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions_name_their_default_bindings() {
        let defaults = |name: &str| {
            KEYS.iter()
                .map(|(_, n, v)| (*n, *v))
                .chain(MOUSE_BUTTONS.iter().copied())
                .find(|(n, _)| n.eq_ignore_ascii_case(name))
                .map(|(_, v)| v.to_string())
        };
        let text = |s: &str| action_text(action(s).unwrap(), &defaults);
        // The lockpicking tutorial's "&-sUActnForward;&-sUActnBack;
        // &-sUActnSldleft;&-sUActnSldright;".
        assert_eq!(
            [
                "sUActnForward",
                "sUActnBack",
                "sUActnSldleft",
                "sUActnSldright"
            ]
            .map(text),
            ["W", "S", "A", "D"]
        );
        assert_eq!(text("suactnuse"), "Mouse1");
        assert_eq!(text("sUActnTogglepov"), "Wheel");
        assert_eq!(text("sUActnMenumode"), "Tab");
        assert_eq!(text("sUActnQuicksave"), "F5");
        assert_eq!(text("sUActnHotKey8"), "8");
        assert_eq!(action("sKBW"), None);
    }

    #[test]
    fn every_action_has_a_binding() {
        assert!((0..28).all(|c| binding_setting(c).is_some()));
        assert_eq!(key_setting(0x2B), Some("sKBBackslash"));
    }
}
