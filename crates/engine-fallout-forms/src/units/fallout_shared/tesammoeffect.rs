//! `fallout shared/tesammoeffect.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The effect's operation kind (`0045cd60`, thiscall, no stack words).
const EFFECT_OPERATION: u32 = 0x0045_cd60;
/// The effect's type (`0059bb30`, thiscall; the engine map's name
/// `D3DTexture_LockRect` is folded code).
const EFFECT_TYPE: u32 = 0x0059_bb30;
/// The effect's value (`00759450`, thiscall, a `float` in ST0).
const EFFECT_VALUE: u32 = 0x0075_9450;
/// A string setting's text (`00403df0`, thiscall on the setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// `strcpy_s` (`00406d30`, cdecl: destination, size, source).
const STRCPY_S: u32 = 0x0040_6d30;
/// `sprintf_s` (`00406d00`, cdecl: buffer, size, format, its words).
const SPRINTF_S: u32 = 0x0040_6d00;
/// The operation signs: `"+"` (`010345e8`), `"x"` (`010345e4`) and `"-"`
/// (`010345e0`).
const SIGN_ADD: u32 = 0x0103_45e8;
const SIGN_MULTIPLY: u32 = 0x0103_45e4;
const SIGN_SUBTRACT: u32 = 0x0103_45e0;
/// `"%s %s %.2f"` (`010345d4`).
const MENU_FORMAT: u32 = 0x0103_45d4;
/// The string setting naming effect type `kind` (cases 0 to 5 of the
/// code's jump table at `0059a350`, each its own code block).
fn type_name_setting(kind: u32) -> Option<u32> {
    match kind {
        0 => Some(0x011c_aa94),
        1 => Some(0x011c_aa84),
        2 => Some(0x011c_aaa0),
        3 => Some(0x011c_aa78),
        4 => Some(0x011c_aaac),
        5 => Some(0x011c_aab8),
        _ => None,
    }
}
/// The stack frame: the 64-byte name buffer, then (after the cookie word)
/// the 4-byte sign buffer.
const NAME_SIZE: u32 = 0x40;
const SIGN_OFFSET: u32 = 0x44;

// Translated from 0059a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059a1e0 - the /GS stack cookie check (00ec408c) is not translated
/// `TESAmmoEffect::BuildMenuString` (engine map; thiscall, `RET 8`):
/// writes `"<type name> <sign> <value>"` (`"%s %s %.2f"`) into `buffer` of
/// `size` bytes. The sign is `"+"`, `"x"` or `"-"` for operations 0, 1, 2
/// (empty otherwise); the type name is the text of one of six string
/// settings for types 0 to 5 (otherwise the name buffer is left as the
/// stack holds it); the value is the effect's `float` (`00759450`) passed
/// as a `double`.
pub fn tes_ammo_effect_build_menu_string(e: &mut Engine, this: Ptr, buffer: u32, size: u32) {
    e.with_stack(SIGN_OFFSET + 4, |e, frame| {
        let name = frame.addr();
        let sign = frame.addr() + SIGN_OFFSET;
        e.mem.set_u8(sign, 0);
        let operation = e.call(EFFECT_OPERATION, &args![this]).u32();
        match operation {
            0 => {
                e.call(STRCPY_S, &args![sign, 2u32, SIGN_ADD]);
            }
            1 => {
                e.call(STRCPY_S, &args![sign, 2u32, SIGN_MULTIPLY]);
            }
            2 => {
                e.call(STRCPY_S, &args![sign, 2u32, SIGN_SUBTRACT]);
            }
            _ => {}
        }
        let kind = e.call(EFFECT_TYPE, &args![this]).u32();
        if let Some(setting) = type_name_setting(kind) {
            let text = e.call(SETTING_STRING, &args![setting]).u32();
            e.call(STRCPY_S, &args![name, NAME_SIZE, text]);
        }
        let value = e.call(EFFECT_VALUE, &args![this]).f64();
        e.call(
            SPRINTF_S,
            &args![buffer, size, MENU_FORMAT, name, sign, value],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0059_a1e0,
        tes_ammo_effect_build_menu_string(Ptr, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn run(operation: u32, kind: u32) -> Engine {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, operation);
        e.mem.set_u32(this + 4, kind);
        e.register(EFFECT_OPERATION, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(EFFECT_TYPE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EFFECT_VALUE, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register(SETTING_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(SPRINTF_S, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0059_a1e0, &args![this, 0x0abc_0000u32, 0x80u32]);
        e
    }

    #[test]
    fn build_menu_string_formats_name_sign_and_value() {
        let e = run(1, 3);
        let copies = calls_to(&e, STRCPY_S);
        assert_eq!(copies.len(), 2);
        let sign = copies[0][0];
        assert_eq!(&copies[0][1..], &[2, SIGN_MULTIPLY]);
        let name = copies[1][0];
        assert_eq!(&copies[1][1..], &[0x40, 0x011c_aa78 + 1]);
        assert_eq!(calls_to(&e, SETTING_STRING), vec![vec![0x011c_aa78]]);
        let value = 1.5f64.to_bits();
        assert_eq!(
            calls_to(&e, SPRINTF_S),
            vec![vec![
                0x0abc_0000,
                0x80,
                MENU_FORMAT,
                name,
                sign,
                value as u32,
                (value >> 32) as u32
            ]]
        );
        for (operation, sign) in [(0, SIGN_ADD), (2, SIGN_SUBTRACT)] {
            let e = run(operation, 0);
            assert_eq!(calls_to(&e, STRCPY_S)[0][2], sign);
        }
        for (kind, setting) in [
            (0, 0x011c_aa94),
            (1, 0x011c_aa84),
            (2, 0x011c_aaa0),
            (4, 0x011c_aaac),
            (5, 0x011c_aab8),
        ] {
            let e = run(3, kind);
            assert_eq!(calls_to(&e, SETTING_STRING), vec![vec![setting]]);
        }
    }

    #[test]
    fn build_menu_string_leaves_unknown_sign_and_name_empty_of_copies() {
        let e = run(3, 6);
        assert!(calls_to(&e, STRCPY_S).is_empty());
        assert!(calls_to(&e, SETTING_STRING).is_empty());
        let printed = calls_to(&e, SPRINTF_S);
        assert_eq!(printed.len(), 1);
        // The sign buffer was cleared to an empty string.
        assert_eq!(printed[0][4] - printed[0][3], SIGN_OFFSET);
    }
}
