//! `fallout shared/run_004d5930` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::lipsynchanim::LipSynchAnim;

/// `0.2` (`float`), the first argument of `0040ebd0`.
const ARGUMENT_0101DF44: u32 = 0x0101_df44;
/// `1/30` (`float`), passed with each row.
const FRAME_TIME_FLOAT: u32 = 0x0102_09a0;
/// `1/30` (`double`).
const FRAME_TIME_DOUBLE: u32 = 0x0102_09a8;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `0040ebd0`: `cdecl`, two floats, a float in ST0.
const COMBINE_0040EBD0: u32 = 0x0040_ebd0;
/// The game's `memcpy` wrapper (`00401460`, `cdecl`: destination, source,
/// count).
const MEMCPY: u32 = 0x0040_1460;
/// `00559450`: the word at `this` (a string's text).
const WORD_AT_ZERO: u32 = 0x0055_9450;
/// `00406d30`: copies a C string into a buffer (`cdecl`: buffer, size,
/// text).
const STRING_COPY_INTO: u32 = 0x0040_6d30;
/// `0040ab30`: the last occurrence of a character (`cdecl`: text, char).
const STRING_FIND_LAST: u32 = 0x0040_ab30;
/// `00406f60`: formats into a string (`cdecl`: string, format, ...).
const STRING_FORMAT: u32 = 0x0040_6f60;
/// `"%s.lip"`.
const FORMAT_LIP: u32 = 0x0102_09b0;

// Translated from 004d5930 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: plays the lip-sync animation `this` on `target`: its slot
/// `0xb8` with `0040ebd0(0.2, start time)` ([`fn_004d5a90`]), then for each
/// frame its slots `0xec` (a copy of the 16 phoneme floats) and `0xf0` (a
/// copy of the 17 modifier floats, 0x44 bytes), each with 1/30; when
/// `blend` is above 0 (not NaN), slot `0xf4` with `(0, blend, 1)` and
/// `(2, blend, 1)`. False without a target.
///
/// Slot `0xb8` gets six words: the code pushes `0, 1, 1, 1, 0` before
/// computing the float and leaves them under it (`004d5a90` takes no
/// arguments; `0040ebd0`'s caller pops one of its two words and stores the
/// result in the other), so the callee finds the float followed by
/// `0, 1, 1, 1, 0`.
pub fn fn_004d5930(e: &mut Engine, this: Ptr<LipSynchAnim>, target: u32, blend: f32) -> bool {
    if target == 0 {
        return false;
    }
    let start = fn_004d5a90(e, this);
    let first = x87_quiet_bits(e.global::<u32>(ARGUMENT_0101DF44));
    let start_bits = x87_quiet_bits(start.to_bits());
    let r = e.call(COMBINE_0040EBD0, &args![first, start_bits]).f64();
    let value = x87_store32(r);
    e.vcall(target, 0xb8, &args![value, 0u32, 1u32, 1u32, 1u32, 0u32]);
    e.with_stack(0x88, |e, frame| {
        let modifiers = frame.addr();
        let phonemes = frame.addr() + 0x48;
        let mut index = 0u32;
        while index < e.get(this, LipSynchAnim::iNumFrames) {
            let rows = e.get(this, LipSynchAnim::ppPhonemes);
            let row = e.mem.u32(rows.wrapping_add(index * 4));
            e.call(MEMCPY, &args![phonemes, row, 0x40u32]);
            let rows = e.get(this, LipSynchAnim::ppModifiers);
            let row = e.mem.u32(rows.wrapping_add(index * 4));
            e.call(MEMCPY, &args![modifiers, row, 0x44u32]);
            let step = x87_quiet_bits(e.global::<u32>(FRAME_TIME_FLOAT));
            e.vcall(target, 0xec, &args![phonemes, step]);
            let step = x87_quiet_bits(e.global::<u32>(FRAME_TIME_FLOAT));
            e.vcall(target, 0xf0, &args![modifiers, step]);
            index = index.wrapping_add(1);
        }
    });
    let zero: f64 = e.global(ZERO);
    if x87_load32(blend.to_bits()) > zero {
        let bits = x87_quiet_bits(blend.to_bits());
        e.vcall(target, 0xf4, &args![0u32, bits, 1u32]);
        e.vcall(target, 0xf4, &args![2u32, bits, 1u32]);
    }
    true
}

// Translated from 004d5a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the animation's start time in seconds: for a negative
/// starting frame its negation times 1/30 (`double`), else 0; returned as
/// a `float`.
pub fn fn_004d5a90(e: &mut Engine, this: Ptr<LipSynchAnim>) -> f32 {
    let start = e.get(this, LipSynchAnim::iStartingFrame);
    let value = if start < 0 {
        let frames = start.wrapping_mul(-1);
        x87_mul(f64::from(frames), e.global::<f64>(FRAME_TIME_DOUBLE))
    } else {
        0.0
    };
    f32::from_bits(x87_store32(value))
}

// Translated from 004d5ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004d5ad0 - the /GS cookie check (00ec408c) is not translated
/// No map name, `cdecl`: turns the path in the string `path` into its
/// `.lip` file's (the text up to its last `.`, then `"%s.lip"`). False,
/// with `path` unchanged, when it has no `.`.
pub fn fn_004d5ad0(e: &mut Engine, path: Ptr) -> bool {
    e.with_stack(0x10c, |e, buffer| {
        let text = e.call(WORD_AT_ZERO, &args![path]).u32();
        e.call(STRING_COPY_INTO, &args![buffer, 0x104u32, text]);
        let dot = e.call(STRING_FIND_LAST, &args![buffer, 0x2eu32]).u32();
        if dot == 0 {
            return false;
        }
        e.mem.set_u8(dot, 0);
        e.call(STRING_FORMAT, &args![path, FORMAT_LIP, buffer]);
        true
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004d_5930,
            fn_004d5930(Ptr<LipSynchAnim>, u32, f32) -> bool
        ),
        entry!(0x004d_5a90, fn_004d5a90(Ptr<LipSynchAnim>) -> f32),
        entry!(0x004d_5ad0, fn_004d5ad0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn constants(e: &mut Engine) {
        e.map(0x0101_d000, 0x1000);
        e.mem.set_f32(ARGUMENT_0101DF44, 0.2);
        e.map(0x0102_0000, 0x1000);
        e.mem.set_u32(FRAME_TIME_FLOAT, 0x3d08_8889);
        e.set_global(FRAME_TIME_DOUBLE, f64::from(f32::from_bits(0x3d08_8889)));
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
    }

    #[test]
    fn start_time_of_a_negative_start() {
        let mut e = Engine::new();
        constants(&mut e);
        let this = e.new_object::<LipSynchAnim>();
        e.set(this, LipSynchAnim::iStartingFrame, -30);
        let expected = (30.0 * f64::from(f32::from_bits(0x3d08_8889))) as f32;
        assert_eq!(e.call(0x004d_5a90, &args![this]).f32(), expected);
        e.set(this, LipSynchAnim::iStartingFrame, 5);
        assert_eq!(e.call(0x004d_5a90, &args![this]).f32(), 0.0);
    }

    #[test]
    fn play_sends_rows_and_blend() {
        let mut e = Engine::new();
        constants(&mut e);
        let this = e.new_object::<LipSynchAnim>();
        let rows = e.mem.alloc(8);
        let mods = e.mem.alloc(8);
        for i in 0..2 {
            let row = e.mem.alloc(0x40);
            e.mem.set_u32(rows + 4 * i, row);
            let row = e.mem.alloc(0x44);
            e.mem.set_u32(mods + 4 * i, row);
        }
        e.set(this, LipSynchAnim::iNumFrames, 2);
        e.set(this, LipSynchAnim::ppPhonemes, rows);
        e.set(this, LipSynchAnim::ppModifiers, mods);
        let target = e.mem.alloc(4);
        let table = e.mem.alloc(0x100);
        e.mem.set_u32(target, table);
        for slot in [0xb8u32, 0xec, 0xf0, 0xf4] {
            e.mem.set_u32(table + slot, 0x7800_0000 + slot);
            e.register(0x7800_0000 + slot, |_, _| Ret::default());
        }
        e.register(COMBINE_0040EBD0, |_, _| Ret {
            st0: 0.75,
            ..Ret::default()
        });
        e.register(MEMCPY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x004d_5930, &args![this, target, 0.5f32]).bool());
        assert_eq!(
            calls_to(&e, COMBINE_0040EBD0),
            vec![vec![0.2f32.to_bits(), 0]]
        );
        assert_eq!(
            calls_to(&e, 0x7800_00b8),
            vec![vec![target, 0.75f32.to_bits(), 0, 1, 1, 1, 0]]
        );
        let copies = calls_to(&e, MEMCPY);
        assert_eq!(copies.len(), 4);
        assert_eq!(copies[1][1], e.mem.u32(mods));
        assert_eq!(copies[1][2], 0x44);
        assert_eq!(copies[0][0], copies[1][0] + 0x48);
        let phonemes = calls_to(&e, 0x7800_00ec);
        assert_eq!(phonemes[0], vec![target, copies[0][0], 0x3d08_8889]);
        assert_eq!(calls_to(&e, 0x7800_00f0).len(), 2);
        assert_eq!(
            calls_to(&e, 0x7800_00f4),
            vec![
                vec![target, 0, 0.5f32.to_bits(), 1],
                vec![target, 2, 0.5f32.to_bits(), 1]
            ]
        );
        // No blend for 0 or NaN; nothing without a target.
        for blend in [0.0f32, f32::NAN] {
            e.call_log = Some(vec![]);
            assert!(e.call(0x004d_5930, &args![this, target, blend]).bool());
            assert!(calls_to(&e, 0x7800_00f4).is_empty());
        }
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004d_5930, &args![this, 0u32, 0.5f32]).bool());
        assert!(calls_to(&e, 0x7800_00b8).is_empty());
    }

    #[test]
    fn lip_path_replaces_the_extension() {
        let mut e = Engine::new();
        e.register(WORD_AT_ZERO, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(STRING_COPY_INTO, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(STRING_FIND_LAST, |e, a| {
            let s = e.mem.cstr(a[0]);
            match s.iter().rposition(|&c| u32::from(c) == a[1]) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(STRING_FORMAT, |_, _| Ret::default());
        let path = e.mem.alloc(8);
        let text = e.mem.alloc(32);
        e.mem.set_cstr(text, b"Sound\\a.b\\line.ogg");
        e.mem.set_u32(path, text);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004d_5ad0, &args![path]).bool());
        let format = calls_to(&e, STRING_FORMAT);
        assert_eq!(format[0][..2], [path, FORMAT_LIP]);
        e.mem.set_cstr(text, b"noext");
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004d_5ad0, &args![path]).bool());
        assert!(calls_to(&e, STRING_FORMAT).is_empty());
    }
}
