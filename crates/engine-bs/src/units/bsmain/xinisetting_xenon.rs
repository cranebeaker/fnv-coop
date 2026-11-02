//! `BSMain/xinisetting_xenon.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The PC build reads and writes INI settings through the Windows profile
//! functions. A `Setting` (Xbox PDB) keeps its value at `+4` and its key
//! (`"name:section"`) at `+8`; the key's first character is its type
//! (`00c33090`). The collection's file name is at `+4` and its open handle
//! at `+0x108`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `strchr` (`00ec7690`), `strcpy_s` (`00ec65a6`), `strlen` (`00ec6130`),
/// `strncpy_s` (`00ec8d5f`), `strcmp` (`00ec6da0`), `strcpy` (`00ec6370`),
/// `sscanf` (`00eca4a6`) and the game's `sprintf_s` (`00406d00`); all cdecl.
const STRCHR: u32 = 0x00ec_7690;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRLEN: u32 = 0x00ec_6130;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const STRCMP: u32 = 0x00ec_6da0;
const STRCPY: u32 = 0x00ec_6370;
const SSCANF: u32 = 0x00ec_a4a6;
const SPRINTF: u32 = 0x0040_6d00;
/// Import slots: `GetPrivateProfileIntA`, `WritePrivateProfileStringA`,
/// `GetPrivateProfileStringA` (Ghidra's names).
const GET_PRIVATE_PROFILE_INT: u32 = 0x00fd_f0c4;
const WRITE_PRIVATE_PROFILE_STRING: u32 = 0x00fd_f0bc;
const GET_PRIVATE_PROFILE_STRING: u32 = 0x00fd_f164;
/// The type code of a setting key (`00c33090`, cdecl) and the string-value
/// store (`00c33170`, `thiscall` on the setting).
const KEY_TYPE: u32 = 0x00c3_3090;
const SET_STRING_VALUE: u32 = 0x00c3_3170;
/// Strings: `"MAIN"`, `"%d"`, `"%u"`, `"%.4f"`, `"%u,%u,%u"`,
/// `"%u,%u,%u,%u"`, `"%f"`, `""`, `"LANGUAGE"`, `"General"`,
/// `"sLanguage"`.
const DEFAULT_SECTION: u32 = 0x010c_1a90;
const FORMAT_INT: u32 = 0x0102_0764;
const FORMAT_UNSIGNED: u32 = 0x010c_1a98;
const FORMAT_FLOAT_WRITE: u32 = 0x010c_1ab4;
const FORMAT_RGB: u32 = 0x010c_1aa8;
const FORMAT_RGBA: u32 = 0x010c_1a9c;
const FORMAT_FLOAT_READ: u32 = 0x0109_4cf0;
const EMPTY_STRING: u32 = 0x0101_1584;
const LANGUAGE_SECTION: u32 = 0x010c_1ac8;
const GENERAL_SECTION: u32 = 0x0105_d548;
const LANGUAGE_KEY: u32 = 0x010c_1abc;
/// The language setting object whose string (`+4`) is the default of
/// `sLanguage:General`.
const LANGUAGE_SETTING: u32 = 0x011c_3cb4;
/// The scratch word a null setting's value is read from (zeroed first).
const NULL_SETTING_VALUE: u32 = 0x0120_2800;

/// `INISettingCollection` fields: the file name (`+4`) and the open handle
/// (`+0x108`); `Setting` fields: the value (`+4`) and the key (`+8`).
const FILE_NAME_OFFSET: u32 = 4;
const OPEN_HANDLE_OFFSET: u32 = 0x108;
const SETTING_VALUE_OFFSET: u32 = 4;
const SETTING_KEY_OFFSET: u32 = 8;

// Translated from 00c43da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The section of a setting's key (cdecl): the text after the first `:`,
/// or `"MAIN"` without one (or without a key), copied into the 0x40-byte
/// `section` (`strcpy_s`); nothing for a null `section`.
pub fn fn_00c43da0(e: &mut Engine, setting: Ptr, section: u32) {
    let mut source = DEFAULT_SECTION;
    if section == 0 {
        return;
    }
    let key = e.mem.u32(setting.addr().wrapping_add(SETTING_KEY_OFFSET));
    if key != 0 {
        let colon = e.call(STRCHR, &args![key, 0x3au32]).u32();
        if colon != 0 {
            source = colon.wrapping_add(1);
        }
    }
    e.call(STRCPY_S, &args![section, 0x40u32, source]);
}

// Translated from 00c43e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The name of a setting's key (cdecl): the text before the first `:` (the
/// whole key without one) copied into `name` (`strncpy_s` with that length
/// plus one as the size) and terminated; an empty `name` without a key,
/// nothing for a null `name`.
pub fn fn_00c43e00(e: &mut Engine, setting: Ptr, name: u32) {
    if name == 0 {
        return;
    }
    e.mem.set_u8(name, 0);
    let key = e.mem.u32(setting.addr().wrapping_add(SETTING_KEY_OFFSET));
    if key == 0 {
        return;
    }
    let colon = e.call(STRCHR, &args![key, 0x3au32]).u32();
    let length = if colon != 0 {
        colon.wrapping_sub(key)
    } else {
        e.call(STRLEN, &args![key]).u32()
    };
    e.call(STRNCPY_S, &args![name, length.wrapping_add(1), key, length]);
    e.mem.set_u8(name.wrapping_add(length), 0);
}

/// Opens the collection (vtable +0x18 with 1) when it isn't; returns whether
/// this call opened it.
fn open_if_closed(e: &mut Engine, this: Ptr) -> bool {
    let opened_here = e.mem.u32(this.addr().wrapping_add(OPEN_HANDLE_OFFSET)) == 0;
    if opened_here {
        e.vcall(this.addr(), 0x18, &args![1u32]);
    }
    opened_here
}

// Translated from 00c43e90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c43e90 - the /GS security cookie check (00ec408c) is not translated
/// `INISettingCollection::WriteSetting` (Xbox PDB, engine map): for a
/// setting with a key, opens the collection when needed (vtable +0x18,
/// closed again through +0x1C), formats the value by the key's type (`c`
/// and `i` `"%d"`, `f` `"%.4f"`, `r` `"%u,%u,%u"` and `a` `"%u,%u,%u,%u"`
/// of the colour bytes from the top, `S`/`s` the text itself, everything
/// else `"%u"`; the exe's jump table at 00c44258) and writes it to the
/// file (`WritePrivateProfileStringA`). Returns whether the write
/// succeeded (false without a key or an open file).
pub fn ini_setting_collection_write_setting(e: &mut Engine, this: Ptr, setting: Ptr) -> u8 {
    let mut written = 0u8;
    let key = e.mem.u32(setting.addr().wrapping_add(SETTING_KEY_OFFSET));
    if key == 0 {
        return written;
    }
    let opened_here = open_if_closed(e, this);
    if e.mem.u32(this.addr().wrapping_add(OPEN_HANDLE_OFFSET)) != 0 {
        e.with_stack(0x1c8, |e, frame| {
            // The frame below EBP: the value text at EBP-0x190, the name at
            // EBP-0x90 and the section at EBP-0x48.
            let value_text = frame.addr() + 0x38;
            let name = frame.addr() + 0x138;
            let section = frame.addr() + 0x180;
            let key = e.mem.u32(setting.addr().wrapping_add(SETTING_KEY_OFFSET));
            let kind = e.call(KEY_TYPE, &args![key]).u32();
            fn_00c43da0(e, setting, section);
            fn_00c43e00(e, setting, name);
            let file = this.addr().wrapping_add(FILE_NAME_OFFSET);
            let value = e.mem.u32(setting.addr().wrapping_add(SETTING_VALUE_OFFSET));
            let text = match kind.wrapping_sub(1) {
                // Type 6: the string itself (null for a null setting).
                5 => {
                    if setting.is_null() {
                        0
                    } else {
                        value
                    }
                }
                0 | 2 => {
                    e.call(SPRINTF, &args![value_text, 0x100u32, FORMAT_INT, value]);
                    value_text
                }
                4 => {
                    let wide = x87_load32(value);
                    e.call(
                        SPRINTF,
                        &args![value_text, 0x100u32, FORMAT_FLOAT_WRITE, wide],
                    );
                    value_text
                }
                6 => {
                    let (red, green, blue) =
                        (value >> 24, (value >> 16) & 0xff, (value >> 8) & 0xff);
                    e.call(
                        SPRINTF,
                        &args![value_text, 0x100u32, FORMAT_RGB, red, green, blue],
                    );
                    value_text
                }
                7 => {
                    let (red, green, blue, alpha) = (
                        value >> 24,
                        (value >> 16) & 0xff,
                        (value >> 8) & 0xff,
                        value & 0xff,
                    );
                    e.call(
                        SPRINTF,
                        &args![value_text, 0x100u32, FORMAT_RGBA, red, green, blue, alpha],
                    );
                    value_text
                }
                _ => {
                    e.call(
                        SPRINTF,
                        &args![value_text, 0x100u32, FORMAT_UNSIGNED, value],
                    );
                    value_text
                }
            };
            let result = e
                .call(
                    WRITE_PRIVATE_PROFILE_STRING,
                    &args![section, name, text, file],
                )
                .u32();
            written = u8::from(result != 0);
        });
    }
    if opened_here {
        e.vcall(this.addr(), 0x1c, &[]);
    }
    written
}

/// The value address `GetPrivateProfileIntA` takes its default from: the
/// setting's `+4`, or for a null setting the scratch word at `01202800`
/// after zeroing it (a byte, or the whole word).
fn default_source(e: &mut Engine, setting: Ptr, byte_only: bool) -> u32 {
    if !setting.is_null() {
        setting.addr().wrapping_add(SETTING_VALUE_OFFSET)
    } else {
        if byte_only {
            e.mem.set_u8(NULL_SETTING_VALUE, 0);
        } else {
            e.mem.set_u32(NULL_SETTING_VALUE, 0);
        }
        NULL_SETTING_VALUE
    }
}

// Translated from 00c44280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c44280 - the /GS security cookie check (00ec408c) is not translated
/// `INISettingCollection::ReadSetting` (Xbox PDB, engine map): for a
/// setting with a key, opens the collection when needed (vtable +0x18,
/// closed again through +0x1C) and reads the value by the key's type (the
/// exe's jump table at 00c448c8): `b` as an integer defaulting to the
/// current flag, stored as `!= 0`; `c` as an integer stored as a byte; `i`
/// (and `h`, `u`, any other) as an integer, its current value the default;
/// `f` as text defaulting to `"%f"` of the current value, parsed back with
/// `"%f"`; `r` / `a` as `"%u,%u,%u"` / `"%u,%u,%u,%u"` colour bytes (`r`
/// with alpha 0xFF); `S`/`s` as text stored through `00c33170`, the
/// section `LANGUAGE` first replaced by `sLanguage:General` (defaulting to
/// the language setting's text). Returns true only when a float or colour
/// was parsed completely.
pub fn ini_setting_collection_read_setting(e: &mut Engine, this: Ptr, setting: Ptr) -> u8 {
    let mut parsed = 0u8;
    let key = e.mem.u32(setting.addr().wrapping_add(SETTING_KEY_OFFSET));
    if key == 0 {
        return parsed;
    }
    let opened_here = open_if_closed(e, this);
    if e.mem.u32(this.addr().wrapping_add(OPEN_HANDLE_OFFSET)) != 0 {
        e.with_stack(0xb08, |e, frame| {
            // The frame below EBP (EBP - 0xb08 is `frame`).
            let at = |offset: u32| frame.addr() + 0xb08 - offset;
            let section = at(0x150);
            let name = at(0x198);
            let default_text = at(0x110);
            let read_text = at(0x298);
            let string_text = at(0x698);
            let language_text = at(0xa98);
            let file = this.addr().wrapping_add(FILE_NAME_OFFSET);
            let value_slot = setting.addr().wrapping_add(SETTING_VALUE_OFFSET);
            let key = e.mem.u32(setting.addr().wrapping_add(SETTING_KEY_OFFSET));
            let kind = e.call(KEY_TYPE, &args![key]).u32();
            fn_00c43da0(e, setting, section);
            fn_00c43e00(e, setting, name);
            match kind {
                6 => {
                    if e.call(STRCMP, &args![section, LANGUAGE_SECTION]).u32() == 0 {
                        // The code tests the address of the language setting
                        // (a constant, never null) before reading its text.
                        let language = if LANGUAGE_SETTING != 0 {
                            e.mem.u32(LANGUAGE_SETTING + 4)
                        } else {
                            0
                        };
                        e.call(
                            GET_PRIVATE_PROFILE_STRING,
                            &args![
                                GENERAL_SECTION,
                                LANGUAGE_KEY,
                                language,
                                language_text,
                                0x100u32,
                                file
                            ],
                        );
                        e.call(STRCPY, &args![section, language_text]);
                    }
                    let current = if setting.is_null() {
                        0
                    } else {
                        e.mem.u32(value_slot)
                    };
                    e.call(
                        GET_PRIVATE_PROFILE_STRING,
                        &args![section, name, current, string_text, 0x100u32, file],
                    );
                    e.call(SET_STRING_VALUE, &args![setting, string_text]);
                }
                1 => {
                    let source = default_source(e, setting, false);
                    let current = e.mem.u32(source);
                    let value = e
                        .call(
                            GET_PRIVATE_PROFILE_INT,
                            &args![section, name, current, file],
                        )
                        .u32();
                    e.mem.set_u8(value_slot, value as u8);
                }
                3 => {
                    let source = default_source(e, setting, false);
                    let current = e.mem.u32(source);
                    let value = e
                        .call(
                            GET_PRIVATE_PROFILE_INT,
                            &args![section, name, current, file],
                        )
                        .u32();
                    if !setting.is_null() {
                        e.mem.set_u32(value_slot, value);
                    }
                }
                5 => {
                    let source = if !setting.is_null() {
                        value_slot
                    } else {
                        e.mem.set_u32(NULL_SETTING_VALUE, 0);
                        NULL_SETTING_VALUE
                    };
                    let current = x87_load32(e.mem.u32(source));
                    e.call(
                        SPRINTF,
                        &args![default_text, 0x100u32, FORMAT_FLOAT_READ, current],
                    );
                    let length = e
                        .call(
                            GET_PRIVATE_PROFILE_STRING,
                            &args![section, name, default_text, read_text, 0x100u32, file],
                        )
                        .u32();
                    if length != 0 {
                        let count = e
                            .call(SSCANF, &args![read_text, FORMAT_FLOAT_READ, value_slot])
                            .u32();
                        parsed = u8::from(count == 1);
                    }
                }
                7 | 8 => {
                    let length = e
                        .call(
                            GET_PRIVATE_PROFILE_STRING,
                            &args![section, name, EMPTY_STRING, read_text, 0x100u32, file],
                        )
                        .u32();
                    if length != 0 {
                        let value = if kind == 7 {
                            let (red, green, blue) = (at(0xaa4), at(0xaa0), at(0xa9c));
                            let count = e
                                .call(SSCANF, &args![read_text, FORMAT_RGB, red, green, blue])
                                .u32();
                            parsed = u8::from(count == 3);
                            e.mem.u32(red) << 24
                                | (e.mem.u32(green) & 0xff) << 16
                                | (e.mem.u32(blue) & 0xff) << 8
                                | 0xff
                        } else {
                            let (red, green, blue, alpha) =
                                (at(0xab4), at(0xab0), at(0xaac), at(0xaa8));
                            let count = e
                                .call(
                                    SSCANF,
                                    &args![read_text, FORMAT_RGBA, red, green, blue, alpha],
                                )
                                .u32();
                            parsed = u8::from(count == 4);
                            e.mem.u32(red) << 24
                                | (e.mem.u32(green) & 0xff) << 16
                                | (e.mem.u32(blue) & 0xff) << 8
                                | (e.mem.u32(alpha) & 0xff)
                        };
                        e.mem.set_u32(value_slot, value);
                    }
                }
                0 => {
                    let source = default_source(e, setting, true);
                    let current = u32::from(e.mem.u8(source) != 0);
                    let value = e
                        .call(
                            GET_PRIVATE_PROFILE_INT,
                            &args![section, name, current, file],
                        )
                        .u32();
                    e.mem.set_u8(value_slot, u8::from(value != 0));
                }
                // 2, 4 and anything above 8.
                _ => {
                    let source = default_source(e, setting, false);
                    let current = e.mem.u32(source);
                    let value = e
                        .call(
                            GET_PRIVATE_PROFILE_INT,
                            &args![section, name, current, file],
                        )
                        .u32();
                    if !setting.is_null() {
                        e.mem.set_u32(value_slot, value);
                    }
                }
            }
        });
    }
    if opened_here {
        e.vcall(this.addr(), 0x1c, &[]);
    }
    parsed
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c4_3da0, fn_00c43da0(Ptr, u32)),
        entry!(0x00c4_3e00, fn_00c43e00(Ptr, u32)),
        entry!(0x00c4_3e90, ini_setting_collection_write_setting(Ptr, Ptr) -> u8),
        entry!(0x00c4_4280, ini_setting_collection_read_setting(Ptr, Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn text(e: &mut Engine, value: &str) -> u32 {
        let address = e.mem.alloc(value.len() as u32 + 1);
        e.mem.write(address, value.as_bytes());
        address
    }

    fn read(e: &Engine, mut address: u32) -> String {
        let mut out = String::new();
        while e.mem.u8(address) != 0 {
            out.push(e.mem.u8(address) as char);
            address += 1;
        }
        out
    }

    fn copy(e: &mut Engine, to: u32, from: u32, limit: u32) {
        for i in 0..limit {
            let c = e.mem.u8(from + i);
            e.mem.set_u8(to + i, c);
            if c == 0 {
                break;
            }
        }
    }

    /// String doubles on real memory, a key-type double by the first
    /// character, and an open collection unless `closed`.
    fn ini_engine(closed: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [
            0x0101_0000,
            0x0101_1000,
            0x0102_0000,
            0x0105_d000,
            0x0109_4000,
            0x010c_1000,
            0x011c_3000,
            0x0120_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(STRCHR, |e, a| {
            let mut i = 0;
            loop {
                let c = e.mem.u8(a[0] + i);
                if u32::from(c) == a[1] {
                    return (a[0] + i).into_ret();
                }
                if c == 0 {
                    return 0u32.into_ret();
                }
                i += 1;
            }
        });
        e.register(STRLEN, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            n.into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            copy(e, a[0], a[2], a[1]);
            Ret::default()
        });
        e.register(STRNCPY_S, |e, a| {
            copy(e, a[0], a[2], a[3]);
            Ret::default()
        });
        e.register(STRCPY, |e, a| {
            copy(e, a[0], a[1], 0x1000);
            Ret::default()
        });
        e.register(STRCMP, |e, a| {
            u32::from(read(e, a[0]) != read(e, a[1])).into_ret()
        });
        e.register(KEY_TYPE, |e, a| {
            match e.mem.u8(a[0]) {
                b'b' => 0u32,
                b'c' => 1,
                b'h' => 2,
                b'i' => 3,
                b'u' => 4,
                b'f' => 5,
                b'S' | b's' => 6,
                b'r' => 7,
                b'a' => 8,
                _ => 9,
            }
            .into_ret()
        });
        e.register(SPRINTF, |e, a| {
            e.mem.set_u8(a[0], b'x');
            e.mem.set_u8(a[0] + 1, 0);
            Ret::default()
        });
        e.register(WRITE_PRIVATE_PROFILE_STRING, |_, _| 1u32.into_ret());
        e.put_vtable(0x0070_0000, &[0, 0, 0, 0, 0, 0, 0x0071_0018, 0x0071_001c]);
        e.register(0x0071_0018, |e, a| {
            e.mem.set_u32(a[0] + 0x108, a[0]);
            Ret::default()
        });
        e.register(0x0071_001c, |e, a| {
            e.mem.set_u32(a[0] + 0x108, 0);
            Ret::default()
        });
        let collection = e.mem.alloc(0x10c);
        e.mem.set_u32(collection, 0x0070_0000);
        if !closed {
            e.mem.set_u32(collection + 0x108, collection);
        }
        (e, collection)
    }

    fn setting(e: &mut Engine, key: &str, value: u32) -> u32 {
        let key = text(e, key);
        let setting = e.mem.alloc(0xc);
        e.mem.set_u32(setting + 4, value);
        e.mem.set_u32(setting + 8, key);
        setting
    }

    #[test]
    fn key_splits_into_name_and_section() {
        let (mut e, _) = ini_engine(false);
        e.mem.write(DEFAULT_SECTION, b"MAIN\0");
        let buffer = e.mem.alloc(0x80);
        let with = setting(&mut e, "fFov:Display", 0);
        e.call(0x00c4_3da0, &args![with, buffer]);
        assert_eq!(read(&e, buffer), "Display");
        e.call(0x00c4_3e00, &args![with, buffer]);
        assert_eq!(read(&e, buffer), "fFov");
        let without = setting(&mut e, "bPlain", 0);
        e.call(0x00c4_3da0, &args![without, buffer]);
        assert_eq!(read(&e, buffer), "MAIN");
        e.call(0x00c4_3e00, &args![without, buffer]);
        assert_eq!(read(&e, buffer), "bPlain");
        // No key: an empty name.
        let none = e.mem.alloc(0xc);
        e.call(0x00c4_3e00, &args![none, buffer]);
        assert_eq!(read(&e, buffer), "");
    }

    #[test]
    fn write_formats_by_type_and_closes_what_it_opened() {
        let (mut e, collection) = ini_engine(true);
        let integer = setting(&mut e, "iSize:Display", 42);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_3e90, &args![collection, integer]).u8(), 1);
        let printed = calls_to(&e, SPRINTF);
        assert_eq!(printed[0][1..], [0x100, FORMAT_INT, 42]);
        let written = calls_to(&e, WRITE_PRIVATE_PROFILE_STRING);
        assert_eq!(read(&e, written[0][0]), "Display");
        assert_eq!(read(&e, written[0][1]), "iSize");
        assert_eq!(read(&e, written[0][2]), "x");
        assert_eq!(written[0][3], collection + 4);
        assert_eq!(calls_to(&e, 0x0071_0018), vec![vec![collection, 1]]);
        assert_eq!(calls_to(&e, 0x0071_001c), vec![vec![collection]]);

        let colour = setting(&mut e, "aColor:Display", 0x1122_3344);
        let float = setting(&mut e, "fScale:Display", 1.5f32.to_bits());
        let string = setting(&mut e, "sName:General", 0x5000);
        let boolean = setting(&mut e, "bFlag:General", 1);
        e.call_log = Some(vec![]);
        e.call(0x00c4_3e90, &args![collection, colour]);
        e.call(0x00c4_3e90, &args![collection, float]);
        e.call(0x00c4_3e90, &args![collection, string]);
        e.call(0x00c4_3e90, &args![collection, boolean]);
        let printed = calls_to(&e, SPRINTF);
        assert_eq!(
            printed[0][1..],
            [0x100, FORMAT_RGBA, 0x11, 0x22, 0x33, 0x44]
        );
        assert_eq!(
            printed[1][1..],
            args![0x100u32, FORMAT_FLOAT_WRITE, 1.5f64][..]
        );
        assert_eq!(printed[2][1..], [0x100, FORMAT_UNSIGNED, 1]);
        let written = calls_to(&e, WRITE_PRIVATE_PROFILE_STRING);
        assert_eq!(written[2][2], 0x5000);
        // No key: nothing.
        let none = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_3e90, &args![collection, none]).u8(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn read_integers_and_flags() {
        let (mut e, collection) = ini_engine(false);
        e.register(GET_PRIVATE_PROFILE_INT, |_, a| (a[2] + 5).into_ret());
        let integer = setting(&mut e, "iSize:Display", 42);
        let flag = setting(&mut e, "bFlag:Display", 0x0200);
        let character = setting(&mut e, "cChar:Display", 0x1234_5600);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_4280, &args![collection, integer]).u8(), 0);
        assert_eq!(e.mem.u32(integer + 4), 47);
        e.call(0x00c4_4280, &args![collection, flag]);
        // The default is the low byte as a flag (0); the result 5 is set.
        assert_eq!(calls_to(&e, GET_PRIVATE_PROFILE_INT)[1][2], 0);
        assert_eq!(e.mem.u32(flag + 4), 0x0201);
        e.call(0x00c4_4280, &args![collection, character]);
        assert_eq!(e.mem.u32(character + 4), 0x1234_5605);
        // An open collection is left open.
        assert!(calls_to(&e, 0x0071_001c).is_empty());
    }

    #[test]
    fn read_floats_and_colours_report_full_parses() {
        let (mut e, collection) = ini_engine(true);
        e.register(GET_PRIVATE_PROFILE_STRING, |_, _| 3u32.into_ret());
        e.register(SSCANF, |e, a| {
            let count = (a.len() - 2) as u32;
            for (i, slot) in a[2..].iter().enumerate() {
                e.mem.set_u32(*slot, 0x10 * (i as u32 + 1));
            }
            count.into_ret()
        });
        let float = setting(&mut e, "fScale:Display", 1.5f32.to_bits());
        let colour = setting(&mut e, "rColor:Display", 0);
        let colour4 = setting(&mut e, "aColor:Display", 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c4_4280, &args![collection, float]).u8(), 1);
        assert_eq!(e.mem.u32(float + 4), 0x10);
        assert_eq!(
            calls_to(&e, SPRINTF)[0][1..],
            args![0x100u32, FORMAT_FLOAT_READ, 1.5f64][..]
        );
        assert_eq!(e.call(0x00c4_4280, &args![collection, colour]).u8(), 1);
        assert_eq!(e.mem.u32(colour + 4), 0x1020_30ff);
        assert_eq!(e.call(0x00c4_4280, &args![collection, colour4]).u8(), 1);
        assert_eq!(e.mem.u32(colour4 + 4), 0x1020_3040);
        assert_eq!(calls_to(&e, 0x0071_0018).len(), 3);
        // Nothing read: nothing parsed or stored.
        e.register(GET_PRIVATE_PROFILE_STRING, |_, _| 0u32.into_ret());
        e.mem.set_u32(colour + 4, 7);
        assert_eq!(e.call(0x00c4_4280, &args![collection, colour]).u8(), 0);
        assert_eq!(e.mem.u32(colour + 4), 7);
    }

    #[test]
    fn read_strings_use_the_language_for_its_section() {
        let (mut e, collection) = ini_engine(false);
        e.mem.write(LANGUAGE_SECTION, b"LANGUAGE\0");
        e.mem.set_u32(LANGUAGE_SETTING + 4, 0x7777);
        e.register(GET_PRIVATE_PROFILE_STRING, |e, a| {
            if a[1] == LANGUAGE_KEY {
                e.mem.write(a[3], b"ENGLISH\0");
            }
            Ret::default()
        });
        e.register(SET_STRING_VALUE, |_, a| a[0].into_ret());
        let string = setting(&mut e, "sText:LANGUAGE", 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x00c4_4280, &args![collection, string]);
        let reads = calls_to(&e, GET_PRIVATE_PROFILE_STRING);
        assert_eq!(reads[0][..3], [GENERAL_SECTION, LANGUAGE_KEY, 0x7777]);
        assert_eq!(read(&e, reads[1][0]), "ENGLISH");
        assert_eq!(reads[1][2], 0x5000);
        let stored = calls_to(&e, SET_STRING_VALUE);
        assert_eq!(stored, vec![vec![string, reads[1][3]]]);
    }
}
