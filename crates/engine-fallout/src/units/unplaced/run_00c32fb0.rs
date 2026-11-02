//! `(unplaced)/run_00c32fb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Helpers of `Setting` (Xbox PDB, 0x0C bytes: vtable, `uValue` at `+4`,
//! `pKey` at `+8`), next to `BSMain/setting.obj` in the exe; the engine map
//! names neither function.

#[allow(unused_imports)]
use crate::prelude::*;

/// `strlen` (`00ec6130`, cdecl), `strcpy` (`00ec6370`, cdecl `(dst, src)`)
/// and `strcpy_s` (`00ec65a6`, cdecl `(dst, size, src)`).
const STRLEN: u32 = 0x00ec_6130;
const STRCPY: u32 = 0x00ec_6370;
const STRCPY_S: u32 = 0x00ec_65a6;
/// The heap object (`011f6238`) and its allocate (`00aa3e40`, one size
/// word) and free (`00aa4060`, one pointer word).
const HEAP: u32 = 0x011f_6238;
const HEAP_ALLOCATE: u32 = 0x00aa_3e40;
const HEAP_FREE: u32 = 0x00aa_4060;

layout! {
    /// `Setting` (Xbox PDB), 0x0C bytes.
    pub struct Setting: 0x0C {
        /// `uValue` (Xbox PDB): the value; for a string setting the text.
        0x04 uValue: u32,
        /// `pKey` (Xbox PDB): the name, whose first character gives the
        /// value's type.
        0x08 pKey: u32,
    }
}

// Translated from 00c33090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The type code of a setting key (cdecl): by its first character (signed),
/// through the exe's byte map and jump table at 00c3314c/00c33124: `b` 0,
/// `c` 1, `h` 2, `i` 3, `u` 4, `f` 5, `S` and `s` 6, `r` 7, `a` 8; any other
/// character, or a null key, 9.
pub fn fn_00c33090(e: &mut Engine, key: u32) -> u32 {
    if key == 0 {
        return 9;
    }
    let first = i32::from(e.mem.u8(key) as i8);
    match first.wrapping_sub(0x53) as u32 {
        0x00 | 0x20 => 6,
        0x0e => 8,
        0x0f => 0,
        0x10 => 1,
        0x13 => 5,
        0x15 => 2,
        0x16 => 3,
        0x1f => 7,
        0x22 => 4,
        _ => 9,
    }
}

// Translated from 00c33170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a string value in a setting (`this` may be null: then nothing
/// happens). A null `text` clears `uValue`. Otherwise one block holds the
/// key (when there is one, first, with its first character set to `S`) and
/// the text after it: allocated from the heap at `011f6238` for both
/// lengths with their terminators; when it can't be, nothing changes. With
/// a key the setting's key moves into the block, and an old key that
/// itself started with `S` (so was such a block) is freed. Returns `this`.
pub fn fn_00c33170(e: &mut Engine, this: Ptr<Setting>, text: u32) -> Ptr<Setting> {
    if this.is_null() {
        return this;
    }
    let key = e.get(this, Setting::pKey);
    let key_size = if key != 0 {
        e.call(STRLEN, &args![key]).u32().wrapping_add(1)
    } else {
        0
    };
    if text == 0 {
        e.set(this, Setting::uValue, 0u32);
        return this;
    }
    let text_size = e.call(STRLEN, &args![text]).u32().wrapping_add(1);
    let block = e
        .call(
            HEAP_ALLOCATE,
            &args![HEAP, text_size.wrapping_add(key_size)],
        )
        .u32();
    if block == 0 {
        return this;
    }
    let value = block.wrapping_add(key_size);
    e.call(STRCPY, &args![value, text]);
    e.set(this, Setting::uValue, value);
    if key_size != 0 {
        let key = e.get(this, Setting::pKey);
        e.call(STRCPY_S, &args![block, key_size, key]);
        e.mem.set_u8(block, b'S');
        let key = e.get(this, Setting::pKey);
        if e.mem.u8(key) as i8 == 0x53 {
            let key = e.get(this, Setting::pKey);
            e.call(HEAP_FREE, &args![HEAP, key]);
        }
        e.set(this, Setting::pKey, block);
    }
    this
}

// Translated from 00c32fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the size of a setting's value (`thiscall`, no words), by
/// the key's first character (signed) through the exe's byte map and jump table at
/// `00c3306c`/`00c3305c`: `S` and `s` the text's length plus its terminator (`strlen`;
/// 0 without text), `a`, `f`, `i`, `r` and `u` 4, `b`, `c` and `h` 1; any other
/// character, or no key, 0. The exe tests `this` for null again before reading the text
/// (after reading the key through it); that test is kept.
pub fn fn_00c32fb0(e: &mut Engine, this: Ptr<Setting>) -> u32 {
    let key = e.get(this, Setting::pKey);
    if key == 0 {
        return 0;
    }
    let first = i32::from(e.mem.u8(key) as i8);
    match first.wrapping_sub(0x53) as u32 {
        0x00 | 0x20 => {
            let text = if this.is_null() {
                0
            } else {
                e.get(this, Setting::uValue)
            };
            if text == 0 {
                return 0;
            }
            let text = if this.is_null() {
                0
            } else {
                e.get(this, Setting::uValue)
            };
            e.call(STRLEN, &args![text]).u32().wrapping_add(1)
        }
        0x0e | 0x13 | 0x16 | 0x1f | 0x22 => 4,
        0x0f | 0x10 | 0x15 => 1,
        _ => 0,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c3_2fb0, fn_00c32fb0(Ptr<Setting>) -> u32),
        entry!(0x00c3_3090, fn_00c33090(u32) -> u32),
        entry!(0x00c3_3170, fn_00c33170(Ptr<Setting>, u32) -> Ptr<Setting>),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(e: &mut Engine, value: &[u8]) -> u32 {
        let address = e.mem.alloc(value.len() as u32 + 1);
        e.mem.write(address, value);
        address
    }

    #[test]
    fn key_type_comes_from_the_first_character() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_3090, &args![0u32]).u32(), 9);
        for (key, code) in [
            (&b"bFlag"[..], 0),
            (b"cChar", 1),
            (b"hHex", 2),
            (b"iInt", 3),
            (b"uUnsigned", 4),
            (b"fFloat", 5),
            (b"SName", 6),
            (b"sName", 6),
            (b"rColor", 7),
            (b"aColor", 8),
            (b"xOther", 9),
            (b"Rcap", 9),
            // A negative (signed) first character falls outside the table.
            (b"\xe9", 9),
        ] {
            let address = text(&mut e, key);
            assert_eq!(e.call(0x00c3_3090, &args![address]).u32(), code);
        }
    }

    /// Doubles for the string functions on real memory and a heap that
    /// hands out blocks from the engine's allocator.
    fn string_engine() -> Engine {
        let mut e = Engine::new();
        e.register(STRLEN, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            n.into_ret()
        });
        e.register(STRCPY, |e, a| {
            let mut i = 0;
            loop {
                let c = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, c);
                if c == 0 {
                    break;
                }
                i += 1;
            }
            Ret::default()
        });
        e.register(STRCPY_S, |e, a| {
            for i in 0..a[1] {
                let c = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, c);
            }
            Ret::default()
        });
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(HEAP_FREE, |_, _| Ret::default());
        e
    }

    fn read(e: &Engine, mut address: u32) -> String {
        let mut out = String::new();
        while e.mem.u8(address) != 0 {
            out.push(e.mem.u8(address) as char);
            address += 1;
        }
        out
    }

    #[test]
    fn string_value_shares_a_block_with_the_key() {
        let mut e = string_engine();
        let setting: Ptr<Setting> = e.new_object();
        let key = text(&mut e, b"sName");
        e.mem.set_u32(setting.addr() + 8, key);
        let value = text(&mut e, b"abc");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c3_3170, &args![setting, value]).u32(),
            setting.addr()
        );
        let block = e.mem.u32(setting.addr() + 8);
        assert_ne!(block, key);
        assert_eq!(read(&e, block), "SName");
        assert_eq!(e.mem.u32(setting.addr() + 4), block + 6);
        assert_eq!(read(&e, block + 6), "abc");
        // The old key started with `s`, not `S`: not freed.
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == HEAP_FREE));
        // Now the key starts with `S`: the old block is freed.
        e.call_log = Some(vec![]);
        e.call(0x00c3_3170, &args![setting, value]);
        let log = e.call_log.take().unwrap();
        let freed: Vec<_> = log.iter().filter(|(a, _)| *a == HEAP_FREE).collect();
        assert_eq!(freed, vec![&(HEAP_FREE, vec![HEAP, block])]);
    }

    #[test]
    fn string_value_without_key_or_text() {
        let mut e = string_engine();
        let setting: Ptr<Setting> = e.new_object();
        let value = text(&mut e, b"xy");
        e.call(0x00c3_3170, &args![setting, value]);
        let stored = e.mem.u32(setting.addr() + 4);
        assert_eq!(read(&e, stored), "xy");
        assert_eq!(e.mem.u32(setting.addr() + 8), 0);
        e.call(0x00c3_3170, &args![setting, 0u32]);
        assert_eq!(e.mem.u32(setting.addr() + 4), 0);
        // A failed allocation leaves the setting alone.
        e.register(HEAP_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00c3_3170, &args![setting, value]);
        assert_eq!(e.mem.u32(setting.addr() + 4), 0);
        assert_eq!(e.call(0x00c3_3170, &args![0u32, value]).u32(), 0);
    }

    #[test]
    fn value_size_follows_the_key_type() {
        let mut e = Engine::new();
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        let setting: Ptr<Setting> = e.new_object();
        assert_eq!(e.call(0x00c3_2fb0, &args![setting]).u32(), 0);
        let cases: [(&[u8], u32); 6] = [
            (b"fScale", 4),
            (b"uCount", 4),
            (b"bFlag", 1),
            (b"hShort", 1),
            (b"xOther", 0),
            (b"sText", 0),
        ];
        for (key, size) in cases {
            let key = text(&mut e, key);
            e.mem.set_u32(setting.addr() + 8, key);
            assert_eq!(e.call(0x00c3_2fb0, &args![setting]).u32(), size);
        }
        let value = text(&mut e, b"abc");
        e.mem.set_u32(setting.addr() + 4, value);
        assert_eq!(e.call(0x00c3_2fb0, &args![setting]).u32(), 4);
    }
}
