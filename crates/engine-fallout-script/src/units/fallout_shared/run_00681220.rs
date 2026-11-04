//! `fallout shared/run_00681220` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Unnamed condition item helpers (`TESConditionItem`, laid out in
//! `tesconditionitem.rs`): setting an item up for a function number
//! (`00681220`, the item constructor hands it 0x1048), the function data,
//! the comparison operator and value, and the global flag.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesconditionitem::TesConditionItem;

/// The game's `memset` wrapper (`00403d30`, cdecl: destination, value,
/// count).
const MEMSET: u32 = 0x0040_3d30;
/// `00526ac0` (`ECX` = global): the global's value, in `ST0`.
const GLOBAL_VALUE: u32 = 0x0052_6ac0;
/// The float the exe loads for the 0x1001 case (`01016410`).
const VALUE_0X1001: u32 = 0x0101_6410;
/// 1.0 (`FLD1`), as a float's bits.
const ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00681220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the item (`ECX`) up for a function number (one stack word, `RET
/// 4`; the item constructor passes 0x1048). A number in 0x1000..0x1280 is
/// taken as 0x1000 plus an index, one in 0..0x280 as the index (and its
/// 0x1000 form is used below); anything else does nothing. The index goes
/// to `00681460`. Then, by the 0x1000 form (the exe's jump table at
/// `00681360`/`0068134c`): 0x1001 sets operator 4 and the float at
/// `01016410`; 0x106b operator 1 and 0.0; 0x10f7 stores 0x28 at +0xc, then
/// operator 0 and 1.0; 0x103e, 0x1040, 0x1043-0x1048, 0x105b,
/// 0x1065-0x1067, 0x106a, 0x106f, 0x1070 and 0x1136 operator 0 and 1.0
/// (operator through `00681530`, value through `00681500`); others
/// nothing more.
pub fn fn_00681220(e: &mut Engine, this: Ptr<TesConditionItem>, number: i32) {
    let mut index = number;
    let mut number = number;
    if (0x1000..0x1280).contains(&number) {
        index = number - 0x1000;
    } else if (0..0x280).contains(&number) {
        number += 0x1000;
    }
    if !(0..0x280).contains(&index) {
        return;
    }
    fn_00681460(e, this, index as u32);
    match number {
        0x1001 => {
            fn_00681530(e, this, 4);
            let value = x87_quiet_bits(e.global::<u32>(VALUE_0X1001));
            fn_00681500(e, this, f32::from_bits(value));
        }
        0x106b => {
            fn_00681530(e, this, 1);
            fn_00681500(e, this, 0.0);
        }
        0x10f7 => {
            e.mem.set_u32(this.addr() + 0x0c, 0x28);
            fn_00681530(e, this, 0);
            fn_00681500(e, this, f32::from_bits(ONE_BITS));
        }
        0x103e
        | 0x1040
        | 0x1043..=0x1048
        | 0x105b
        | 0x1065..=0x1067
        | 0x106a
        | 0x106f
        | 0x1070
        | 0x1136 => {
            fn_00681530(e, this, 0);
            fn_00681500(e, this, f32::from_bits(ONE_BITS));
        }
        _ => {}
    }
}

// Translated from 00681460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the item's function data (`ECX`; 0xc bytes at +8, `memset`) and
/// stores the low half of the one stack word (`RET 4`) as the function
/// number at +8.
pub fn fn_00681460(e: &mut Engine, this: Ptr<TesConditionItem>, function: u32) {
    e.call(MEMSET, &args![this.addr() + 0x08, 0u32, 0x0cu32]);
    e.set(this, TesConditionItem::iFunction, function as u16);
}

// Translated from 00681490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The item's comparison value (`ECX`, no stack words; in `ST0`): with flag
/// bit 2 set, the global's value (`00526ac0`) or 0.0 without a global;
/// otherwise the float at +4.
pub fn fn_00681490(e: &mut Engine, this: Ptr<TesConditionItem>) -> f32 {
    let mut value = 0.0f32;
    if fn_006814e0(e, this) {
        let global = e.get(this, TesConditionItem::pGlobal);
        if global != 0 {
            value = e.call(GLOBAL_VALUE, &args![global]).f32();
        }
    } else {
        value = f32::from_bits(x87_quiet_bits(e.get(this, TesConditionItem::pGlobal)));
    }
    value
}

// Translated from 006814e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the item's (`ECX`) flag bit 2 is set: the comparison value is a
/// global.
pub fn fn_006814e0(e: &mut Engine, this: Ptr<TesConditionItem>) -> bool {
    e.get(this, TesConditionItem::iFlags) & 0x04 != 0
}

// Translated from 00681500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float (one stack word, `RET 4`) as the item's (`ECX`)
/// comparison value at +4, unless flag bit 2 (a global) is set.
pub fn fn_00681500(e: &mut Engine, this: Ptr<TesConditionItem>, value: f32) {
    if !fn_006814e0(e, this) {
        e.set(
            this,
            TesConditionItem::pGlobal,
            x87_quiet_bits(value.to_bits()),
        );
    }
}

// Translated from 00681530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the comparison operator (one stack word, `RET 4`; 0 when it is
/// not in 0..6) in bits 5-7 of the item's (`ECX`) flag byte.
pub fn fn_00681530(e: &mut Engine, this: Ptr<TesConditionItem>, operator: i32) {
    let operator = if (0..6).contains(&operator) {
        operator
    } else {
        0
    };
    let bits = (operator << 5) as u8;
    let flags = e.get(this, TesConditionItem::iFlags) & 0x1f;
    e.set(this, TesConditionItem::iFlags, flags | bits);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00681220, fn_00681220(Ptr<TesConditionItem>, i32)),
        entry!(0x00681460, fn_00681460(Ptr<TesConditionItem>, u32)),
        entry!(0x00681490, fn_00681490(Ptr<TesConditionItem>) -> f32),
        entry!(0x006814e0, fn_006814e0(Ptr<TesConditionItem>) -> bool),
        entry!(0x00681500, fn_00681500(Ptr<TesConditionItem>, f32)),
        entry!(0x00681530, fn_00681530(Ptr<TesConditionItem>, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn setup() -> Engine {
        let mut e = Engine::new();
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(GLOBAL_VALUE, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    /// An item filled with 0xaa bytes.
    fn item(e: &mut Engine) -> u32 {
        let p = e.mem.alloc(0x1c);
        for i in 0..0x1c {
            e.mem.set_u8(p + i, 0xaa);
        }
        p
    }

    /// Runs `00681220` on a fresh item with `flags` and returns it.
    fn set_up(e: &mut Engine, number: i32, flags: u8) -> u32 {
        let p = item(e);
        e.mem.set_u8(p, flags);
        e.call(0x0068_1220, &args![Ptr::<()>::new(p), number]);
        p
    }

    #[test]
    fn set_up_takes_the_index_from_either_form_of_the_number() {
        let mut e = setup();
        e.map(VALUE_0X1001, 4);
        e.set_global(VALUE_0X1001, 100.0f32);
        // 0x1048 and its index 0x48 do the same: operator 0, value 1.0.
        for number in [0x1048, 0x48] {
            let p = set_up(&mut e, number, 0xff);
            assert_eq!(e.mem.u16(p + 8), 0x48);
            assert_eq!(e.mem.u32(p + 0xc), 0);
            assert_eq!(e.mem.u32(p + 0x10), 0);
            assert_eq!(e.mem.u8(p), 0x1f);
            // Flag bit 2 is set: the value is a global and stays.
            assert_eq!(e.mem.u32(p + 4), 0xaaaa_aaaa);
            let p = set_up(&mut e, number, 0x00);
            assert_eq!(e.mem.u32(p + 4), 0x3f80_0000);
        }
        // 0x1001: operator 4 and the float at 01016410.
        let p = set_up(&mut e, 1, 0);
        assert_eq!(e.mem.u8(p), 0x80);
        assert_eq!(e.mem.f32(p + 4), 100.0);
        // 0x106b: operator 1, value 0.0.
        let p = set_up(&mut e, 0x106b, 0);
        assert_eq!((e.mem.u8(p), e.mem.u32(p + 4)), (0x20, 0));
        // 0x10f7: 0x28 at +0xc after the function data was cleared.
        let p = set_up(&mut e, 0x10f7, 0);
        assert_eq!(e.mem.u32(p + 0xc), 0x28);
        assert_eq!((e.mem.u8(p), e.mem.u32(p + 4)), (0, 0x3f80_0000));
        // 0x1136 is outside the jump table but the same case.
        let p = set_up(&mut e, 0x136, 0x04 | 0xe0);
        assert_eq!(e.mem.u8(p), 0x04);
    }

    #[test]
    fn set_up_ignores_bad_numbers_and_leaves_other_cases_alone() {
        let mut e = setup();
        for number in [-1, 0x280, 0xfff, 0x1280, 0x2000] {
            let p = set_up(&mut e, number, 0x55);
            assert_eq!(e.mem.u32(p + 8), 0xaaaa_aaaa, "{number:#x}");
            assert_eq!(e.mem.u8(p), 0x55);
        }
        assert!(e.call_log.as_ref().unwrap().iter().all(|c| c.0 != MEMSET));
        // A number without a case: only the function data.
        for number in [0x1000, 0x1002, 0x1041, 0x1137, 0x127f] {
            let p = set_up(&mut e, number, 0x55);
            assert_eq!(e.mem.u16(p + 8) as i32, number - 0x1000);
            assert_eq!((e.mem.u8(p), e.mem.u32(p + 4)), (0x55, 0xaaaa_aaaa));
        }
    }

    #[test]
    fn the_function_data_is_cleared_and_numbered() {
        let mut e = setup();
        let p = item(&mut e);
        e.call(0x0068_1460, &args![Ptr::<()>::new(p), 0x0001_0123u32]);
        assert_eq!(e.mem.u16(p + 8), 0x0123);
        assert_eq!(e.mem.u16(p + 0xa), 0);
        assert_eq!(e.mem.u64(p + 0xc), 0);
        assert_eq!(e.mem.u32(p + 0x14), 0xaaaa_aaaa);
    }

    #[test]
    fn the_comparison_value_is_the_float_or_the_global() {
        let mut e = setup();
        let p = item(&mut e);
        let this = Ptr::<()>::new(p);
        e.mem.set_u8(p, 0);
        e.mem.set_f32(p + 4, 3.5);
        assert_eq!(e.call(0x0068_1490, &args![this]).f32(), 3.5);
        // A signalling NaN comes back quiet.
        e.mem.set_u32(p + 4, 0x7f80_0001);
        let v = e.call(0x0068_1490, &args![this]).f32();
        assert_eq!(v.to_bits(), 0x7fc0_0001);
        e.mem.set_u8(p, 0x04);
        e.mem.set_u32(p + 4, 0x1234);
        assert_eq!(e.call(0x0068_1490, &args![this]).f32(), 2.5);
        e.mem.set_u32(p + 4, 0);
        assert_eq!(e.call(0x0068_1490, &args![this]).f32(), 0.0);
        let globals: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == GLOBAL_VALUE)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(globals, vec![vec![0x1234]]);
    }

    #[test]
    fn the_global_flag_and_the_value_setter() {
        let mut e = setup();
        let p = item(&mut e);
        let this = Ptr::<()>::new(p);
        e.mem.set_u8(p, 0xfb);
        assert!(!e.call(0x0068_14e0, &args![this]).bool());
        e.call(0x0068_1500, &args![this, 0x7f80_0002u32]);
        assert_eq!(e.mem.u32(p + 4), 0x7fc0_0002);
        e.mem.set_u8(p, 0x04);
        assert!(e.call(0x0068_14e0, &args![this]).bool());
        e.call(0x0068_1500, &args![this, 1.0f32]);
        assert_eq!(e.mem.u32(p + 4), 0x7fc0_0002);
    }

    #[test]
    fn the_operator_goes_to_the_top_three_bits() {
        let mut e = setup();
        let p = item(&mut e);
        let this = Ptr::<()>::new(p);
        for (operator, flags) in [(0, 0x15), (1, 0x35), (5, 0xb5), (6, 0x15), (-1, 0x15)] {
            e.mem.set_u8(p, 0xf5);
            e.call(0x0068_1530, &args![this, operator]);
            assert_eq!(e.mem.u8(p), flags, "{operator}");
        }
    }
}
