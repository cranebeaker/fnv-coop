//! `Havok SDK/run_00e30f50` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Function-pointer tables (cdecl, indexed by a kind number): `00e30f50` calls the entries at
/// `011be488` with two words (the first is `packSignedQuaternion32`, `00e52320`, by the exe's
/// own data names), `00e30ff0` those at `011be4a0` with one.
const PACK_TABLE: u32 = 0x011b_e488;
const WALK_TABLE: u32 = 0x011b_e4a0;
/// Per kind: the alignment (`010eb3b8`) and the size in bytes (`010eb3d0`) of an element.
const ALIGNMENT_TABLE: u32 = 0x010e_b3b8;
const SIZE_TABLE: u32 = 0x010e_b3d0;
/// `255.0` and `65535.0` (`double`).
const BYTE_SCALE: u32 = 0x0101_e568;
const WORD_SCALE: u32 = 0x0102_17d0;

// Translated from 00e30f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: calls the cdecl function in entry `kind` of the table at
/// `011be488` with the words `first` and `second`; the result is the callee's `EAX`. (The
/// function takes three stack words and pops none.)
pub fn fn_00e30f50(e: &mut Engine, kind: u32, first: u32, second: u32) -> u32 {
    let function = e.mem.u32(PACK_TABLE.wrapping_add(kind.wrapping_mul(4)));
    e.call(function, &args![first, second]).u32()
}

/// `FISTP dword` with the truncating rounding mode the two functions below set (`FNSTCW`, `OR
/// 0xc00`, `FLDCW`): the integer part, or the "integer indefinite" `0x80000000` when it does not
/// fit.
fn truncating_fistp(value: f64) -> i32 {
    if value > -2_147_483_649.0 && value < 2_147_483_648.0 {
        value.trunc() as i32
    } else {
        i32::MIN
    }
}

/// `(value - low) / (high - low) * scale` on the x87 stack: `FLD value; FLD low; FLD ST0;
/// FSUBP` (value minus low), `FSUBR high` (high minus low), `FDIVP`, `FMUL scale`.
fn scaled_fraction(e: &Engine, low: f32, high: f32, value: f32, scale: u32) -> f64 {
    let numerator = x87_sub(x87_load32(value.to_bits()), x87_load32(low.to_bits()));
    let span = x87_sub(x87_m32(high.to_bits()), x87_load32(low.to_bits()));
    x87_mul(x87_div(numerator, span), e.global::<f64>(scale))
}

// Translated from 00e30f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: packs `value` between `low` and `high` into a byte: `(value - low)
/// / (high - low) * 255.0` computed on the x87 stack and stored truncated (`FISTP` with the
/// rounding control forced to truncate); the result is in `AL` (the low byte of the integer).
/// The function takes three float words (the stack order is `low`, `high`, `value`) and pops
/// none (`RET`).
pub fn fn_00e30f70(e: &mut Engine, low: f32, high: f32, value: f32) -> u8 {
    truncating_fistp(scaled_fraction(e, low, high, value, BYTE_SCALE)) as u8
}

// Translated from 00e30fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: like `00e30f70` with the factor `65535.0` and a 16-bit result in
/// `AX`.
pub fn fn_00e30fb0(e: &mut Engine, low: f32, high: f32, value: f32) -> u16 {
    truncating_fistp(scaled_fraction(e, low, high, value, WORD_SCALE)) as u16
}

// Translated from 00e30ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: calls the cdecl function in entry `kind` of the table at
/// `011be4a0` with the one word `value`; the result is the callee's `EAX`. (The function takes
/// two stack words and pops none.)
pub fn fn_00e30ff0(e: &mut Engine, kind: u32, value: u32) -> u32 {
    let function = e.mem.u32(WALK_TABLE.wrapping_add(kind.wrapping_mul(4)));
    e.call(function, &args![value]).u32()
}

// Translated from 00e31010 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: walks one field of a packed buffer. Does nothing when the low byte
/// of `flags` is zero. Otherwise the offset stored at `cursor` is rounded up to the alignment of
/// `kind` (`010eb3b8`); then, when the high nibble of `flags` is clear, `00e30ff0(kind, offset)`
/// runs once and the size of `kind` (`010eb3d0`) is added to the offset; when the nibble is set
/// and `count` is not negative, that is done `count + 1` times (the offset read again each
/// time). The function takes four stack words and pops none (`RET`) and returns nothing.
pub fn fn_00e31010(e: &mut Engine, flags: u8, count: i32, kind: u32, cursor: Ptr) {
    if flags == 0 {
        return;
    }
    let alignment = e
        .mem
        .u32(ALIGNMENT_TABLE.wrapping_add(kind.wrapping_mul(4)));
    let offset = e.mem.u32(cursor.addr());
    let aligned = offset.wrapping_add(alignment).wrapping_sub(1) & !alignment.wrapping_sub(1);
    e.mem.set_u32(cursor.addr(), aligned);
    if flags & 0xf0 == 0 {
        fn_00e30ff0(e, kind, aligned);
        advance_by_size(e, kind, cursor);
    } else if count >= 0 {
        let mut remaining = count.wrapping_add(1);
        loop {
            let at = e.mem.u32(cursor.addr());
            fn_00e30ff0(e, kind, at);
            advance_by_size(e, kind, cursor);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    }
}

/// `*cursor += size of kind` (`ADD [ESI],EDX` on the memory word as it is then).
fn advance_by_size(e: &mut Engine, kind: u32, cursor: Ptr) {
    let size = e.mem.u32(SIZE_TABLE.wrapping_add(kind.wrapping_mul(4)));
    let at = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), at.wrapping_add(size));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_0f50, fn_00e30f50(u32, u32, u32) -> u32),
        entry!(0x00e3_0f70, fn_00e30f70(f32, f32, f32) -> u8),
        entry!(0x00e3_0fb0, fn_00e30fb0(f32, f32, f32) -> u16),
        entry!(0x00e3_0ff0, fn_00e30ff0(u32, u32) -> u32),
        entry!(0x00e3_1010, fn_00e31010(u8, i32, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// A double at `address` that logs its words and answers `result`.
    fn logging_double(e: &mut Engine, address: u32, result: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let calls = Rc::clone(&log);
        e.register_double(address, move |_, a| {
            calls.borrow_mut().push(a.to_vec());
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
        log
    }

    /// The function in the table entry gets the two words, and its result comes back.
    #[test]
    fn fn_00e30f50_calls_the_table_entry() {
        let mut e = engine();
        e.mem.set_u32(0x011b_e488 + 4 * 3, 0x0900_0001);
        let log = logging_double(&mut e, 0x0900_0001, 0x1234);
        assert_eq!(e.call(0x00e3_0f50, &args![3u32, 7u32, 9u32]).u32(), 0x1234);
        assert_eq!(*log.borrow(), vec![vec![7, 9]]);
    }

    /// The function in the table entry gets the one word, and its result comes back.
    #[test]
    fn fn_00e30ff0_calls_the_table_entry() {
        let mut e = engine();
        e.mem.set_u32(0x011b_e4a0 + 4 * 2, 0x0900_0002);
        let log = logging_double(&mut e, 0x0900_0002, 0x55);
        assert_eq!(e.call(0x00e3_0ff0, &args![2u32, 0x77u32]).u32(), 0x55);
        assert_eq!(*log.borrow(), vec![vec![0x77]]);
    }

    fn set_scales(e: &mut Engine) {
        e.mem.set_f64(0x0101_e568, 255.0);
        e.mem.set_f64(0x0102_17d0, 65535.0);
    }

    /// The fraction between low and high, times 255, truncated; low maps to 0, high to 255.
    #[test]
    fn fn_00e30f70_packs_a_byte() {
        let mut e = engine();
        set_scales(&mut e);
        let pack = |e: &mut Engine, low: f32, high: f32, value: f32| {
            e.call(0x00e3_0f70, &args![low, high, value]).u8()
        };
        assert_eq!(pack(&mut e, 0.0, 1.0, 0.0), 0);
        assert_eq!(pack(&mut e, 0.0, 1.0, 1.0), 255);
        assert_eq!(pack(&mut e, 0.0, 1.0, 0.5), 127);
        assert_eq!(pack(&mut e, -1.0, 1.0, 0.0), 127);
        assert_eq!(pack(&mut e, 10.0, 20.0, 15.0), 127);
        // Beyond the range the integer's low byte comes back: 1.5 * 255 = 382 = 0x17e.
        assert_eq!(pack(&mut e, 0.0, 1.0, 1.5), 0x7e);
        // Not a number: the integer indefinite, whose low byte is 0.
        assert_eq!(pack(&mut e, 0.0, 0.0, 0.0), 0);
    }

    /// The same for 16 bits.
    #[test]
    fn fn_00e30fb0_packs_a_word() {
        let mut e = engine();
        set_scales(&mut e);
        let pack = |e: &mut Engine, low: f32, high: f32, value: f32| {
            e.call(0x00e3_0fb0, &args![low, high, value]).u16()
        };
        assert_eq!(pack(&mut e, 0.0, 1.0, 1.0), 0xffff);
        assert_eq!(pack(&mut e, 0.0, 1.0, 0.5), 32767);
        assert_eq!(pack(&mut e, -2.0, 2.0, 0.0), 32767);
        assert_eq!(pack(&mut e, 0.0, 1.0, 2.0), 0xfffe);
    }

    fn set_tables(e: &mut Engine) {
        // Kind 1: alignment 4, size 4; kind 3: alignment 2, size 1.
        e.mem.set_u32(0x010e_b3b8 + 4, 4);
        e.mem.set_u32(0x010e_b3d0 + 4, 4);
        e.mem.set_u32(0x010e_b3b8 + 12, 2);
        e.mem.set_u32(0x010e_b3d0 + 12, 1);
    }

    /// A zero flags byte does nothing.
    #[test]
    fn fn_00e31010_zero_flags_do_nothing() {
        let mut e = engine();
        set_tables(&mut e);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, 5);
        e.call(0x00e3_1010, &args![0u32, 3i32, 1u32, cursor]);
        assert_eq!(e.mem.u32(cursor), 5);
    }

    /// A clear high nibble: align, one call, advance by the size.
    #[test]
    fn fn_00e31010_single_field() {
        let mut e = engine();
        set_tables(&mut e);
        e.mem.set_u32(0x011b_e4a0 + 4, 0x0900_0003);
        let log = logging_double(&mut e, 0x0900_0003, 0);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, 5);
        e.call(0x00e3_1010, &args![0x01u32, 9i32, 1u32, cursor]);
        assert_eq!(*log.borrow(), vec![vec![8]]);
        assert_eq!(e.mem.u32(cursor), 12);
    }

    /// A set high nibble: `count + 1` calls, each at the advanced offset; a negative count
    /// stops after the alignment.
    #[test]
    fn fn_00e31010_repeats_count_plus_one_times() {
        let mut e = engine();
        set_tables(&mut e);
        e.mem.set_u32(0x011b_e4a0 + 12, 0x0900_0004);
        let log = logging_double(&mut e, 0x0900_0004, 0);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, 5);
        e.call(0x00e3_1010, &args![0x10u32, 2i32, 3u32, cursor]);
        assert_eq!(*log.borrow(), vec![vec![6], vec![7], vec![8]]);
        assert_eq!(e.mem.u32(cursor), 9);
        log.borrow_mut().clear();
        e.mem.set_u32(cursor, 5);
        e.call(0x00e3_1010, &args![0x10u32, -1i32, 3u32, cursor]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(cursor), 6);
    }
}
