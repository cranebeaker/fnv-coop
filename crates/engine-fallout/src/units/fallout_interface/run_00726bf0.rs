//! `fallout/interface/run_00726bf0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006e6da0(4, address)` (cdecl): the per-word step; the result is unused.
const WORD_STEP: u32 = 0x006e_6da0;

// Translated from 00726bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: calls `006e6da0(4, address + 4 * i)` for each
/// `i` in `0..count`. `this` is not read.
pub fn fn_00726bf0(e: &mut Engine, _this: Ptr, address: u32, count: u32) {
    let mut i = 0u32;
    while i < count {
        e.call(
            WORD_STEP,
            &args![4u32, address.wrapping_add(i.wrapping_mul(4))],
        );
        i += 1;
    }
}

/// `005ae380` (thiscall on a list; the map names it `VATS::GetCount`): counts the
/// list's non-null items.
const LIST_COUNT: u32 = 0x005a_e380;

// Translated from 00726c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the item count (`005ae380`) of the list at `this + 4`; the callee's `EAX`
/// is left as the result.
pub fn fn_00726c40(e: &mut Engine, this: Ptr) -> u32 {
    e.call(LIST_COUNT, &args![this.addr() + 4]).u32()
}

// Translated from 00726c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`RET 4`): copies the word at `source` into `this` unless it is 0.
pub fn fn_00726c60(e: &mut Engine, this: Ptr, source: u32) {
    let value = e.mem.u32(source);
    if value != 0 {
        e.mem.set_u32(this.addr(), value);
    }
}

/// `BSString`'s constructor (`004037b0`) and destructor (`004037d0`), `006ffe10`
/// (`thiscall`), `MemoryManager` free (`00401030`, cdecl) and `Tile::Value::SetFloat`
/// (`00a0a270`, `RET 8`: a `float` and a flag).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const INIT_006FFE10: u32 = 0x006f_fe10;
const FREE: u32 = 0x0040_1030;
const TILE_VALUE_SET_FLOAT: u32 = 0x00a0_a270;

// Translated from 00726c90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00726c90 - the C++ exception-unwinding frame (FS:[0] around the string) is not translated.
/// Unnamed (`thiscall`; a constructor): the `BSString` at +0 constructed (`004037b0`),
/// then `006ffe10`. Answers `this`.
pub fn fn_00726c90(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(STRING_CONSTRUCT, &args![this]);
    e.call(INIT_006FFE10, &args![this]);
    this
}

// Translated from 00726cf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00726cf0 - the C++ exception-unwinding frame (FS:[0] around the string) is not translated.
/// Unnamed (`thiscall`; the matching destructor): frees the block at +0x24, then
/// destroys the `BSString` at +0 (`004037d0`).
pub fn fn_00726cf0(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr() + 0x24);
    e.call(FREE, &args![block]);
    e.call(STRING_DESTRUCT, &args![this]);
}

// Translated from 00726d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a `bool`): sets or clears bit 2 of the flags byte
/// (+0x28) through `00726d70`.
pub fn fn_00726d50(e: &mut Engine, this: Ptr, set: u8) {
    fn_00726d70(e, this, 2, set);
}

// Translated from 00726d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 8`: a mask and a `bool`): sets the mask's bits in the flags
/// byte (+0x28), or clears them.
pub fn fn_00726d70(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let at = this.addr() + 0x28;
    let flags = u32::from(e.mem.u8(at));
    let flags = if set != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.mem.set_u8(at, flags as u8);
}

// Translated from 00726db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`: a `bool`): with a tile value at +0xc, sets it to 1.0 or
/// 0.0 (`Tile::Value::SetFloat`, flag 1).
pub fn fn_00726db0(e: &mut Engine, this: Ptr, on: u8) {
    let value = e.mem.u32(this.addr() + 0xc);
    if value == 0 {
        return;
    }
    let number = x87_store32(if on != 0 { 1.0 } else { 0.0 });
    let value = e.mem.u32(this.addr() + 0xc);
    e.call(
        TILE_VALUE_SET_FLOAT,
        &args![value, x87_quiet_bits(number), 1u32],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00726bf0, fn_00726bf0(Ptr, u32, u32)),
        entry!(0x0072_6c40, fn_00726c40(Ptr) -> u32),
        entry!(0x0072_6c60, fn_00726c60(Ptr, u32)),
        entry!(0x0072_6c90, fn_00726c90(Ptr) -> Ptr),
        entry!(0x0072_6cf0, fn_00726cf0(Ptr)),
        entry!(0x0072_6d50, fn_00726d50(Ptr, u8)),
        entry!(0x0072_6d70, fn_00726d70(Ptr, u32, u8)),
        entry!(0x0072_6db0, fn_00726db0(Ptr, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_once_per_word() {
        let mut e = Engine::new();
        e.register(WORD_STEP, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00726bf0, &args![0u32, 0x6000u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            &log[1..],
            &[
                (WORD_STEP, vec![4, 0x6000]),
                (WORD_STEP, vec![4, 0x6004]),
                (WORD_STEP, vec![4, 0x6008]),
            ]
        );
    }

    #[test]
    fn a_zero_count_steps_nothing() {
        let mut e = Engine::new();
        e.register(WORD_STEP, |_, _| panic!("no words"));
        e.call(0x00726bf0, &args![0u32, 0x6000u32, 0u32]);
    }

    #[test]
    fn fn_00726c40_counts_the_list_at_four() {
        let mut e = Engine::new();
        e.register(LIST_COUNT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0072_6c40, &args![0x6000u32]).u32(), 0x6005);
    }

    #[test]
    fn fn_00726c60_copies_a_non_zero_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        let source = e.mem.alloc(4);
        e.mem.set_u32(this, 7);
        e.call(0x0072_6c60, &args![this, source]);
        assert_eq!(e.mem.u32(this), 7);
        e.mem.set_u32(source, 9);
        e.call(0x0072_6c60, &args![this, source]);
        assert_eq!(e.mem.u32(this), 9);
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn constructor_and_destructor() {
        let mut e = Engine::new();
        for address in [STRING_CONSTRUCT, STRING_DESTRUCT, INIT_006FFE10, FREE] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x24, 0x4400);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0072_6c90, &args![this]).u32(), this);
        assert_eq!(calls(&e, STRING_CONSTRUCT), vec![vec![this]]);
        assert_eq!(calls(&e, INIT_006FFE10), vec![vec![this]]);
        e.call(0x0072_6cf0, &args![this]);
        assert_eq!(calls(&e, FREE), vec![vec![0x4400]]);
        assert_eq!(calls(&e, STRING_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn flags_and_the_tile_value() {
        let mut e = Engine::new();
        e.register(TILE_VALUE_SET_FLOAT, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.call(0x0072_6d70, &args![this, 0x81u32, 1u32]);
        assert_eq!(e.mem.u8(this + 0x28), 0x81);
        e.call(0x0072_6d50, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 0x28), 0x83);
        e.call(0x0072_6d50, &args![this, 0u32]);
        e.call(0x0072_6d70, &args![this, 0x80u32, 0u32]);
        assert_eq!(e.mem.u8(this + 0x28), 1);
        e.call_log = Some(vec![]);
        e.call(0x0072_6db0, &args![this, 1u32]);
        assert!(calls(&e, TILE_VALUE_SET_FLOAT).is_empty());
        e.mem.set_u32(this + 0xc, 0x5500);
        e.call(0x0072_6db0, &args![this, 1u32]);
        e.call(0x0072_6db0, &args![this, 0u32]);
        assert_eq!(
            calls(&e, TILE_VALUE_SET_FLOAT),
            vec![vec![0x5500, 1.0f32.to_bits(), 1], vec![0x5500, 0, 1]]
        );
    }
}
