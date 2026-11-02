//! `Havok SDK/run_00d73380` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMathUtil::decompose4x4ColTransform` (`00d72e90`, engine map name): cdecl, two words.
const DECOMPOSE_4X4_COL_TRANSFORM: u32 = 0x00d7_2e90;
/// `00de8f10`: `thiscall`, no stack words; clears the last lane of the first three columns
/// (`+0xc`, `+0x1c`, `+0x2c`) and sets `+0x3c` to `1.0` (the bottom row of a 4x4 transform).
const SET_BOTTOM_ROW: u32 = 0x00de_8f10;
/// The 16-byte vector at `01267e30` copied in as the translation column.
const TRANSLATION_COLUMN: u32 = 0x0126_7e30;

// Translated from 00d73380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Jump thunk to `hkMathUtil::decompose4x4ColTransform` (`00d72e90`, engine map name): the two
/// stack words are the callee's and so is its `RET`.
pub fn fn_00d73380(e: &mut Engine, matrix: u32, decomposition: u32) {
    e.call(DECOMPOSE_4X4_COL_TRANSFORM, &args![matrix, decomposition]);
}

// Translated from 00d73390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three 16-byte columns at `source` and the vector at `01267e30` into a 0x40-byte
/// stack matrix, sets its bottom row with `00de8f10`, and decomposes it
/// (`00d73380` -> `hkMathUtil::decompose4x4ColTransform`) into `decomposition`. Cdecl, two stack
/// words.
pub fn fn_00d73390(e: &mut Engine, source: u32, decomposition: u32) {
    e.with_stack(0x40, |e, matrix| {
        let matrix = matrix.addr();
        for offset in (0..0x30).step_by(4) {
            let word = e.mem.u32(source.wrapping_add(offset));
            e.mem.set_u32(matrix + offset, word);
        }
        for offset in (0..0x10).step_by(4) {
            let word = e.mem.u32(TRANSLATION_COLUMN + offset);
            e.mem.set_u32(matrix + 0x30 + offset, word);
        }
        e.call(SET_BOTTOM_ROW, &args![matrix]);
        fn_00d73380(e, matrix, decomposition);
    });
}

// Translated from 00d733e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 9-byte record: two words cleared and a byte set to 1 at `+8`. Returns
/// `this`. The class is unconfirmed.
pub fn fn_00d733e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
    e.mem.set_u8(this.addr().wrapping_add(8), 1);
    this
}

// Translated from 00d73400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the four words at `this`. `EAX` is 0 on return (it is the zero the stores used). The
/// class is unconfirmed.
pub fn fn_00d73400(e: &mut Engine, this: Ptr) -> u32 {
    for offset in [0u32, 4, 8, 0xc] {
        e.mem.set_u32(this.addr().wrapping_add(offset), 0);
    }
    0
}

// Translated from 00d73410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the four words at `this`, then again through `00d73400`, and returns `this`. The
/// class is unconfirmed.
pub fn fn_00d73410(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in [0u32, 4, 8, 0xc] {
        e.mem.set_u32(this.addr().wrapping_add(offset), 0);
    }
    fn_00d73400(e, this);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d7_3380, fn_00d73380(u32, u32)),
        entry!(0x00d7_3390, fn_00d73390(u32, u32)),
        entry!(0x00d7_33e0, fn_00d733e0(Ptr) -> Ptr),
        entry!(0x00d7_3400, fn_00d73400(Ptr) -> u32),
        entry!(0x00d7_3410, fn_00d73410(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    #[test]
    fn jump_thunk_passes_both_words_to_the_decomposition() {
        let mut e = Engine::new();
        e.register(DECOMPOSE_4X4_COL_TRANSFORM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d7_3380, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == DECOMPOSE_4X4_COL_TRANSFORM)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x1000, 0x2000]);
    }

    #[test]
    fn matrix_is_copied_completed_and_decomposed() {
        let mut e = Engine::new();
        e.map(0x0126_7000, 0x1000);
        for lane in 0..4u32 {
            e.set_global(TRANSLATION_COLUMN + 4 * lane, 0x4000_0000 + lane);
        }
        let source = e.mem.alloc(0x30);
        for word in 0..12u32 {
            e.mem.set_u32(source + 4 * word, 0x100 + word);
        }
        e.register(SET_BOTTOM_ROW, |e, a| {
            e.mem.set_u32(a[0] + 0xc, 0);
            e.mem.set_u32(a[0] + 0x3c, 0x3f80_0000);
            Ret::default()
        });
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = std::rc::Rc::clone(&seen);
        e.register_double(DECOMPOSE_4X4_COL_TRANSFORM, move |e, a| {
            let words: Vec<u32> = (0..16).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            log.borrow_mut().push((a[1], words));
            Ret::default()
        });
        e.call(0x00d7_3390, &args![source, 0x5555u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0, 0x5555);
        let words = &seen[0].1;
        // Columns copied (the bottom lane of the first one cleared), then the vector, then 1.0.
        assert_eq!(words[0], 0x100);
        assert_eq!(words[3], 0);
        assert_eq!(
            words[4..11],
            [0x104, 0x105, 0x106, 0x107, 0x108, 0x109, 0x10a]
        );
        assert_eq!(words[12], 0x4000_0000);
        assert_eq!(words[13], 0x4000_0001);
        assert_eq!(words[14], 0x4000_0002);
        assert_eq!(words[15], 0x3f80_0000);
    }

    #[test]
    fn small_record_constructor_clears_two_words_and_sets_the_flag_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for offset in 0..3 {
            e.mem.set_u32(this + 4 * offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00d7_33e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u8(this + 8), 1);
        // The bytes after the flag are not touched.
        assert_eq!(e.mem.u8(this + 9), 0xff);
    }

    #[test]
    fn clearing_functions_zero_four_words() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        for offset in 0..5 {
            e.mem.set_u32(this + 4 * offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00d7_3400, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
        for offset in 0..4 {
            e.mem.set_u32(this + 4 * offset, 0x1234);
        }
        assert_eq!(e.call(0x00d7_3410, &args![this]).u32(), this);
        for offset in 0..4 {
            assert_eq!(e.mem.u32(this + 4 * offset), 0);
        }
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
    }
}
