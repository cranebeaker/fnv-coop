//! `fallout shared/run_005f6ed0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Callees: the base step `00484d60` (`thiscall`, `(buffer)`), the buffer's `00428110`
/// (`thiscall`, `(&local)`, answers the local) and the local's test `004280f0`
/// (`thiscall`, `(2)`, answers a byte), and the buffer's read `00865e50` (`thiscall`,
/// `(destination, size, 0)`).
const BASE_00484D60: u32 = 0x0048_4d60;
const BUFFER_LOCAL_00428110: u32 = 0x0042_8110;
const LOCAL_TEST_004280F0: u32 = 0x0042_80f0;
const BUFFER_READ_00865E50: u32 = 0x0086_5e50;

// Translated from 005f7150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(buffer)`): the base step `00484d60(buffer)`, then,
/// when the buffer's local (`00428110`) passes `004280f0(2)`, four 4-byte reads from the
/// buffer into `this + 0x44`, `+0x48`, `+0x4C` and `+0x50`.
pub fn fn_005f7150(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(BASE_00484D60, &args![this, buffer]);
    let passed = e.with_stack(4, |e, local| {
        let filled = e.call(BUFFER_LOCAL_00428110, &args![buffer, local]).u32();
        e.call(LOCAL_TEST_004280F0, &args![filled, 2u32]).bool()
    });
    if passed {
        for index in 0..4u32 {
            let destination = this.addr() + 0x44 + 4 * index;
            e.call(
                BUFFER_READ_00865E50,
                &args![buffer, destination, 4u32, 0u32],
            );
        }
    }
}

/// The string text getter `MapMarkerData::GetLocationName` (engine map;
/// `00408da0`, thiscall on a string), `strlen` (`0044a670`, cdecl) and the
/// save buffer at `[011de45c]` with its write (`008579b0`) and read
/// (`008579e0`), both thiscall (address, size).
const STRING_TEXT: u32 = 0x0040_8da0;
const TEXT_LENGTH: u32 = 0x0044_a670;
const SAVE_BUFFER: u32 = 0x011d_e45c;
const BUFFER_WRITE: u32 = 0x0085_79b0;
const BUFFER_READ: u32 = 0x0085_79e0;
/// The two strings these functions save, at +0x18 and +0x2C.
const STRINGS: [u32; 2] = [0x18, 0x2c];

// Translated from 005f6ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The saved size of the object (no stack words; the engine map has no
/// name), a 16-bit sum: 0x1C, then for each string at +0x18 and +0x2C one
/// length byte plus its text's `strlen` when it has a text.
pub fn fn_005f6ed0(e: &mut Engine, this: Ptr) -> u16 {
    let mut size = 0u16;
    size = size.wrapping_add(0x1c);
    for offset in STRINGS {
        size = size.wrapping_add(1);
        if e.call(STRING_TEXT, &args![this.addr() + offset]).u32() != 0 {
            let text = e.call(STRING_TEXT, &args![this.addr() + offset]).u32();
            let length = e.call(TEXT_LENGTH, &args![text]).u32();
            size = size.wrapping_add(length as u16);
        }
    }
    size
}

// Translated from 005f6f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves the object (no stack words; the engine map has no name): the
/// 0x1C bytes at +0x44, then for each string at +0x18 and +0x2C its
/// length as a byte (0 without a text; `strlen` truncated) and, when not
/// 0, that many bytes of its text, each through the write of the save
/// buffer at `[011de45c]`.
pub fn fn_005f6f70(e: &mut Engine, this: Ptr) {
    let buffer = e.global::<u32>(SAVE_BUFFER);
    e.call(BUFFER_WRITE, &args![buffer, this.addr() + 0x44, 0x1cu32]);
    e.with_stack(4, |e, length| {
        for offset in STRINGS {
            let string = this.addr() + offset;
            e.mem.set_u8(length.addr(), 0);
            if e.call(STRING_TEXT, &args![string]).u32() != 0 {
                let text = e.call(STRING_TEXT, &args![string]).u32();
                let count = e.call(TEXT_LENGTH, &args![text]).u32();
                e.mem.set_u8(length.addr(), count as u8);
            }
            let buffer = e.global::<u32>(SAVE_BUFFER);
            e.call(BUFFER_WRITE, &args![buffer, length, 1u32]);
            let count = e.mem.u8(length.addr());
            if count != 0 {
                let text = e.call(STRING_TEXT, &args![string]).u32();
                let buffer = e.global::<u32>(SAVE_BUFFER);
                e.call(BUFFER_WRITE, &args![buffer, text, u32::from(count)]);
            }
        }
    });
}

// Translated from 005f7050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f7050 - the /GS stack cookie check (00ec408c) is not translated.
/// Loads what [`fn_005f6f70`] saves (no stack words; the engine map has no
/// name): the 0x1C bytes at +0x44, then for each of the strings at +0x18
/// and +0x2C a length byte and, when not 0, that many bytes into a
/// cleared 0x104-byte stack buffer (`00403d30`) set as the sound file
/// (`TESSoundFile::SetSoundFile`), each through the read of the save
/// buffer at `[011de45c]`.
pub fn fn_005f7050(e: &mut Engine, this: Ptr) {
    let buffer = e.global::<u32>(SAVE_BUFFER);
    e.call(BUFFER_READ, &args![buffer, this.addr() + 0x44, 0x1cu32]);
    e.with_stack(4, |e, length| {
        for offset in STRINGS {
            let buffer = e.global::<u32>(SAVE_BUFFER);
            e.call(BUFFER_READ, &args![buffer, length, 1u32]);
            let count = e.mem.u8(length.addr());
            if count != 0 {
                e.with_stack(0x104, |e, text| {
                    e.call(0x0040_3d30, &args![text, 0u32, 0x104u32]);
                    let buffer = e.global::<u32>(SAVE_BUFFER);
                    e.call(BUFFER_READ, &args![buffer, text, u32::from(count)]);
                    e.call(0x0048_9100, &args![this.addr() + offset, text]);
                });
            }
        }
    });
}

// Translated from 005f71c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(buffer)`), the load counterpart of
/// [`fn_005f7150`]: `00484da0(buffer)`, then, when the buffer's local
/// (`00428110`) passes `004280f0(2)`, four 4-byte reads (`00864980`, on the
/// buffer: address, size) into `this + 0x44` .. `+0x50`.
pub fn fn_005f71c0(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4da0, &args![this, buffer]);
    let passed = e.with_stack(4, |e, local| {
        let filled = e.call(BUFFER_LOCAL_00428110, &args![buffer, local]).u32();
        e.call(LOCAL_TEST_004280F0, &args![filled, 2u32]).bool()
    });
    if passed {
        for index in 0..4u32 {
            let destination = this.addr() + 0x44 + 4 * index;
            e.call(0x0086_4980, &args![buffer, destination, 4u32]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f7150, fn_005f7150(Ptr, Ptr)),
        entry!(0x005f_6ed0, fn_005f6ed0(Ptr) -> u16),
        entry!(0x005f_6f70, fn_005f6f70(Ptr)),
        entry!(0x005f_7050, fn_005f7050(Ptr)),
        entry!(0x005f_71c0, fn_005f71c0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_005f7150_reads_four_words_when_the_test_passes() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_00484D60, |_, _| Ret::default());
        e.register(BUFFER_LOCAL_00428110, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e.register(LOCAL_TEST_004280F0, |_, _| Ret::default());
        e.register(BUFFER_READ_00865E50, |_, _| Ret::default());
        e.call(0x005f7150, &args![0x5000u32, 0x6000u32]);
        let reads = |e: &Engine| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == BUFFER_READ_00865E50)
                .map(|c| c.1.clone())
                .collect()
        };
        assert!(reads(&e).is_empty());
        e.register(LOCAL_TEST_004280F0, |_, a| Ret {
            eax: u32::from(a[1] == 2),
            ..Ret::default()
        });
        e.call(0x005f7150, &args![0x5000u32, 0x6000u32]);
        assert_eq!(
            reads(&e),
            vec![
                vec![0x6000, 0x5044, 4, 0],
                vec![0x6000, 0x5048, 4, 0],
                vec![0x6000, 0x504c, 4, 0],
                vec![0x6000, 0x5050, 4, 0]
            ]
        );
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose string at +0x18 has a 300-byte text (its length
    /// truncates) and whose string at +0x2C has none; strings keep their
    /// text pointer in their first word, `strlen` reads the word at
    /// text + 0x100.
    fn object(e: &mut Engine) -> u32 {
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_BUFFER, 0x5000u32);
        e.register(STRING_TEXT, |e, a| ret(e.mem.u32(a[0])));
        e.register(TEXT_LENGTH, |e, a| ret(e.mem.u32(a[0] + 0x100)));
        let this = e.mem.alloc(0x60);
        let text = e.mem.alloc(0x104);
        e.mem.set_u32(text + 0x100, 300);
        e.mem.set_u32(this + 0x18, text);
        this
    }

    #[test]
    fn fn_005f6ed0_sums_the_saved_size() {
        let mut e = Engine::new();
        let this = object(&mut e);
        assert_eq!(
            e.call(0x005f_6ed0, &args![this]).u32() & 0xffff,
            0x1c + 1 + 300 + 1
        );
        e.mem.set_u32(this + 0x18, 0);
        assert_eq!(e.call(0x005f_6ed0, &args![this]).u32() & 0xffff, 0x1e);
    }

    #[test]
    fn fn_005f6f70_writes_the_block_and_the_strings() {
        let mut e = Engine::new();
        let this = object(&mut e);
        let text = e.mem.u32(this + 0x18);
        let lengths = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = lengths.clone();
        e.register_double(BUFFER_WRITE, move |e, a| {
            if a[2] == 1 {
                seen.borrow_mut().push(e.mem.u8(a[1]));
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x005f_6f70, &args![this]);
        let writes = calls(&e, BUFFER_WRITE);
        assert_eq!(writes.len(), 4);
        assert_eq!(writes[0], vec![0x5000, this + 0x44, 0x1c]);
        assert_eq!(writes[2], vec![0x5000, text, 300 & 0xff]);
        assert_eq!(*lengths.borrow(), vec![(300 & 0xff) as u8, 0]);
    }

    #[test]
    fn fn_005f7050_reads_the_block_and_the_sound_files() {
        let mut e = Engine::new();
        let this = object(&mut e);
        // The lengths read: 5 for the first string, 0 for the second.
        let reads = std::rc::Rc::new(std::cell::RefCell::new(vec![5u8, 0]));
        let next = reads.clone();
        e.register_double(BUFFER_READ, move |e, a| {
            if a[2] == 1 {
                let value = next.borrow_mut().remove(0);
                e.mem.set_u8(a[1], value);
            }
            Ret::default()
        });
        e.register(0x0040_3d30, |_, _| Ret::default());
        e.register(0x0048_9100, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005f_7050, &args![this]);
        let read = calls(&e, BUFFER_READ);
        assert_eq!(read[0], vec![0x5000, this + 0x44, 0x1c]);
        assert_eq!(read.len(), 4);
        let clear = calls(&e, 0x0040_3d30);
        assert_eq!(clear.len(), 1);
        assert_eq!(&clear[0][1..], &[0, 0x104]);
        assert_eq!(read[2], vec![0x5000, clear[0][0], 5]);
        assert_eq!(calls(&e, 0x0048_9100), vec![vec![this + 0x18, clear[0][0]]]);
    }

    #[test]
    fn fn_005f71c0_reads_four_words_when_the_test_passes() {
        let mut e = Engine::new();
        e.register(0x0048_4da0, |_, _| Ret::default());
        e.register(BUFFER_LOCAL_00428110, |_, a| ret(a[1]));
        e.register(0x0086_4980, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        for (passed, reads) in [(1u32, 4usize), (0, 0)] {
            e.register_double(LOCAL_TEST_004280F0, move |_, a| {
                assert_eq!(a[1], 2);
                ret(passed)
            });
            e.call_log = Some(vec![]);
            e.call(0x005f_71c0, &args![this, 0x7000u32]);
            assert_eq!(calls(&e, 0x0048_4da0), vec![vec![this, 0x7000]]);
            let words = calls(&e, 0x0086_4980);
            assert_eq!(words.len(), reads);
            if reads > 0 {
                assert_eq!(words[3], vec![0x7000, this + 0x50, 4]);
            }
        }
    }
}
