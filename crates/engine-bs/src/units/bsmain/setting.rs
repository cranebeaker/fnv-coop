//! `BSMain/setting.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `strcpy_s` (`00ec65a6`) and `strcat_s` (`00ec6bd2`), both `cdecl` (destination, size,
/// source).
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCAT_S: u32 = 0x00ec_6bd2;
/// The empty text the viewer name starts from (`01080198`).
const EMPTY_TEXT: u32 = 0x0108_0198;
/// The setting's type from the first letter of its name (`00c33090`, `cdecl`: the name).
const SETTING_TYPE: u32 = 0x00c3_3090;
/// The `NiGetViewerString` overloads (`cdecl`: name, value): `00a7d370` (`_ov2`, a byte),
/// `00a7d3d0` (`_ov3`, a byte), `00a7d4d0` (`_ov7`, a word), `00a7d510` (`_ov8`, a word),
/// `00a7d550` (`_ov9`, a `float`) and `00a7d5e0` (`_ov11`, a word).
const VIEWER_STRING_OV2: u32 = 0x00a7_d370;
const VIEWER_STRING_OV3: u32 = 0x00a7_d3d0;
const VIEWER_STRING_OV7: u32 = 0x00a7_d4d0;
const VIEWER_STRING_OV8: u32 = 0x00a7_d510;
const VIEWER_STRING_OV9: u32 = 0x00a7_d550;
const VIEWER_STRING_OV11: u32 = 0x00a7_d5e0;
/// The viewer string array's add (`0096a610`, `thiscall`, `RET 4`: address of the string).
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;
/// The static value a null setting reads (`01202800`): cleared before each use.
const NULL_SETTING_VALUE: u32 = 0x0120_2800;
/// `MessageHandler::VSend` (`00c3be40`, engine map; `cdecl`: kind, format, argument list).
const MESSAGE_HANDLER_VSEND: u32 = 0x00c3_be40;

/// The address of the setting's value (`+4`); for a null setting the static word at
/// `01202800`, cleared first (its low byte only for a byte-sized value).
fn value_address(e: &mut Engine, this: u32, byte: bool) -> u32 {
    if this != 0 {
        return this.wrapping_add(4);
    }
    if byte {
        e.mem.set_u8(NULL_SETTING_VALUE, 0);
    } else {
        e.mem.set_u32(NULL_SETTING_VALUE, 0);
    }
    NULL_SETTING_VALUE
}

/// Adds the viewer string `text` (by address, as the exe passes it) to `strings`.
fn add_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
    });
}

// Translated from 00c33280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c33280 - the /GS stack cookie check (00ec408c) is not translated
/// `Setting::GetViewerStrings` (engine map; `thiscall`, `RET 4`: the viewer string array).
/// The name (the empty text `01080198` with the setting's name, +8, appended into a 0x100-byte
/// stack buffer) and the value make one viewer string, by the type `00c33090` answers for the
/// name (the compiler's jump table at `00c33580`): 0, the byte through `00a7d370`; 1 and 2,
/// the byte through `00a7d3d0` and then (the case falls through) the word through `00a7d4d0`;
/// 3, the word through `00a7d4d0`; 5, the `float` through `00a7d550`; 6, the word at +4 (0 for
/// a null setting) through `00a7d5e0`; 4 and any other type, the word through `00a7d510`.
/// Each string is added to the array (`0096a610`).
pub fn setting_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let this = this.addr();
    e.with_stack(0x100, |e, name| {
        e.call(STRCPY_S, &args![name, 0x100u32, EMPTY_TEXT]);
        let setting_name = e.mem.u32(this.wrapping_add(8));
        e.call(STRCAT_S, &args![name, 0x100u32, setting_name]);
        let setting_name = e.mem.u32(this.wrapping_add(8));
        let kind = e.call(SETTING_TYPE, &args![setting_name]).u32();
        match kind {
            0 => {
                let value = value_address(e, this, true);
                let byte = u32::from(e.mem.u8(value));
                let text = e.call(VIEWER_STRING_OV2, &args![name, byte]).u32();
                add_viewer_string(e, strings, text);
            }
            1..=3 => {
                if kind != 3 {
                    let value = value_address(e, this, true);
                    let byte = u32::from(e.mem.u8(value));
                    let text = e.call(VIEWER_STRING_OV3, &args![name, byte]).u32();
                    add_viewer_string(e, strings, text);
                }
                let value = value_address(e, this, false);
                let word = e.mem.u32(value);
                let text = e.call(VIEWER_STRING_OV7, &args![name, word]).u32();
                add_viewer_string(e, strings, text);
            }
            5 => {
                let value = value_address(e, this, false);
                let pushed = x87_quiet_bits(e.mem.u32(value));
                let text = e.call(VIEWER_STRING_OV9, &args![name, pushed]).u32();
                add_viewer_string(e, strings, text);
            }
            6 => {
                let word = if this != 0 {
                    e.mem.u32(this.wrapping_add(4))
                } else {
                    0
                };
                let text = e.call(VIEWER_STRING_OV11, &args![name, word]).u32();
                add_viewer_string(e, strings, text);
            }
            _ => {
                let value = value_address(e, this, false);
                let word = e.mem.u32(value);
                let text = e.call(VIEWER_STRING_OV8, &args![name, word]).u32();
                add_viewer_string(e, strings, text);
            }
        }
    });
}

// Translated from 00c335a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingWarning` (engine map; `cdecl`, a format and variable arguments): sends the
/// message through `MessageHandler::VSend` (`00c3be40`) with kind 1, the format and the
/// address of the arguments after it (copied into game memory here). Answers what `VSend`
/// answers.
pub fn setting_warning(e: &mut Engine, format: u32, rest: &[u32]) -> u32 {
    let size = (4 * rest.len() as u32).max(4);
    e.with_stack(size, |e, list| {
        for (i, word) in rest.iter().enumerate() {
            e.mem.set_u32(list.addr() + 4 * i as u32, *word);
        }
        e.call(MESSAGE_HANDLER_VSEND, &args![1u32, format, list])
            .u32()
    })
}

/// The uniform-form entry of [`setting_warning`]: the format, then every further word.
fn setting_warning_entry(e: &mut Engine, words: &[u32]) -> Ret {
    setting_warning(e, words[0], &words[1..]).into_ret()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_3280, setting_get_viewer_strings(Ptr, Ptr)),
        (0x00c3_35a0, setting_warning_entry as AbiFn),
    ]
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Runs the viewer strings of a setting of type `kind` (value word `value`); answers the
    /// overloads called (with their value) and the strings added.
    fn strings_of(this_null: bool, kind: u32, value: u32) -> (Vec<(u32, u32)>, Vec<u32>) {
        let mut e = Engine::new();
        e.map(NULL_SETTING_VALUE, 4);
        e.mem.set_u32(NULL_SETTING_VALUE, 0xffff_ffff);
        e.register(STRCPY_S, |e, a| {
            e.mem.set_cstr(a[0], b"");
            Ret::default()
        });
        e.register(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register_double(SETTING_TYPE, move |_, _| kind.into_ret());
        let log: Log = Default::default();
        for address in [
            VIEWER_STRING_OV2,
            VIEWER_STRING_OV3,
            VIEWER_STRING_OV7,
            VIEWER_STRING_OV8,
            VIEWER_STRING_OV9,
            VIEWER_STRING_OV11,
        ] {
            let log = log.clone();
            e.register_double(address, move |e, a| {
                assert_eq!(e.mem.cstr(a[0]), b"fName".to_vec());
                log.borrow_mut().push((address, vec![a[1]]));
                address.into_ret()
            });
        }
        let added = log.clone();
        e.register_double(VIEWER_STRINGS_ADD, move |e, a| {
            added
                .borrow_mut()
                .push((VIEWER_STRINGS_ADD, vec![e.mem.u32(a[1])]));
            Ret::default()
        });
        let this = if this_null {
            0
        } else {
            let setting = e.mem.alloc(0xc);
            let name = e.mem.alloc(8);
            e.mem.set_cstr(name, b"fName");
            e.mem.set_u32(setting + 4, value);
            e.mem.set_u32(setting + 8, name);
            setting
        };
        if this_null {
            e.map(0, 0x10);
            let name = e.mem.alloc(8);
            e.mem.set_cstr(name, b"fName");
            e.mem.set_u32(8, name);
        }
        e.call(0x00c3_3280, &args![this, 0x900u32]);
        let log = log.borrow();
        let made = log
            .iter()
            .filter(|c| c.0 != VIEWER_STRINGS_ADD)
            .map(|c| (c.0, c.1[0]))
            .collect();
        let adds = log
            .iter()
            .filter(|c| c.0 == VIEWER_STRINGS_ADD)
            .map(|c| c.1[0])
            .collect();
        (made, adds)
    }

    #[test]
    fn each_type_has_its_viewer_string() {
        let value = 0x3f80_0181;
        assert_eq!(
            strings_of(false, 0, value).0,
            vec![(VIEWER_STRING_OV2, 0x81)]
        );
        assert_eq!(
            strings_of(false, 1, value).0,
            vec![(VIEWER_STRING_OV3, 0x81), (VIEWER_STRING_OV7, value)]
        );
        assert_eq!(
            strings_of(false, 2, value).1,
            vec![VIEWER_STRING_OV3, VIEWER_STRING_OV7]
        );
        assert_eq!(
            strings_of(false, 3, value).0,
            vec![(VIEWER_STRING_OV7, value)]
        );
        assert_eq!(
            strings_of(false, 4, value).0,
            vec![(VIEWER_STRING_OV8, value)]
        );
        assert_eq!(
            strings_of(false, 5, value).0,
            vec![(VIEWER_STRING_OV9, value)]
        );
        assert_eq!(
            strings_of(false, 6, value).0,
            vec![(VIEWER_STRING_OV11, value)]
        );
        assert_eq!(
            strings_of(false, 9, value).0,
            vec![(VIEWER_STRING_OV8, value)]
        );
        // A signalling NaN float is pushed through the x87 stack, quiet.
        assert_eq!(
            strings_of(false, 5, 0x7f80_0001).0,
            vec![(VIEWER_STRING_OV9, 0x7fc0_0001)]
        );
    }

    #[test]
    fn a_null_setting_reads_the_cleared_static_value() {
        assert_eq!(strings_of(true, 0, 0).0, vec![(VIEWER_STRING_OV2, 0)]);
        assert_eq!(
            strings_of(true, 1, 0).0,
            vec![(VIEWER_STRING_OV3, 0), (VIEWER_STRING_OV7, 0)]
        );
        assert_eq!(strings_of(true, 6, 0).0, vec![(VIEWER_STRING_OV11, 0)]);
        assert_eq!(strings_of(true, 4, 0).0, vec![(VIEWER_STRING_OV8, 0)]);
    }

    #[test]
    fn warnings_pass_the_argument_list() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let inner = seen.clone();
        e.register_double(MESSAGE_HANDLER_VSEND, move |e, a| {
            inner
                .borrow_mut()
                .push((a[0], a[1], e.mem.u32(a[2]), e.mem.u32(a[2] + 4)));
            9u32.into_ret()
        });
        let result = e
            .call(0x00c3_35a0, &args![0x0100_0000u32, 5u32, 6u32])
            .u32();
        assert_eq!(result, 9);
        assert_eq!(*seen.borrow(), vec![(1, 0x0100_0000, 5, 6)]);
    }
}
