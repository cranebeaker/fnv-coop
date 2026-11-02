//! `fallout shared/havok/run_0062e630` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00cb75d0` (unnamed in the engine map): `thiscall`, plain `RET`; the base constructor.
const BASE_CONSTRUCT: u32 = 0x00cb_75d0;
/// `0061ffb0` (`bgszonetargetlistener.rs`): `thiscall`, plain `RET`; the second base's
/// constructor.
const SECOND_BASE_CONSTRUCT: u32 = 0x0061_ffb0;
/// `0048f200` (unnamed in the engine map): `thiscall`, plain `RET`.
const MEMBER_CONSTRUCT: u32 = 0x0048_f200;
/// `00401000` (unnamed in the engine map): `cdecl`, the size; the allocation.
const ALLOCATE: u32 = 0x0040_1000;
/// `004a3e90` (unnamed in the engine map): `cdecl`, a `float`; answers in `ST0`.
const CONVERT_VALUE: u32 = 0x004a_3e90;
/// The vtable `0062e630` stores at `+0`.
const VTABLE: u32 = 0x0104_e450;
/// The vtable `0062e630` stores at `+0x14`.
const SECOND_VTABLE: u32 = 0x0104_e440;

// Translated from 0062e710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 0xc`, a `float` and two words. Stores the
/// converted `value` (`004a3e90`, rounded to a `float`) in the table at `+0x20`, at index
/// `row * width + column` (the width at `+0x28`, 32-bit `IMUL`).
pub fn fn_0062e710(e: &mut Engine, this: Ptr, value: f32, column: u32, row: u32) {
    let width = e.mem.u32(this.addr().wrapping_add(0x28));
    let index = row.wrapping_mul(width).wrapping_add(column);
    let converted = e
        .call(CONVERT_VALUE, &args![x87_quiet_bits(value.to_bits())])
        .f64();
    let table = e.mem.u32(this.addr().wrapping_add(0x20));
    e.mem.set_u32(
        table.wrapping_add(index.wrapping_mul(4)),
        x87_store32(converted),
    );
}

// Translated from 0062e630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0062e630 - C++ exception unwinding (FS:[0] frame) not translated
/// Unnamed in the engine map. `thiscall`, `RET 4`, a `float`. A constructor: the bases
/// (`00cb75d0` on `this`, `0061ffb0` on `this + 0x14`), the vtables `0104e450` and `0104e440`,
/// 1.0 at `+0x1c`, the member at `+0x34` (`0048f200`), a width of 1 at `+0x28` and a one-entry
/// table of 4 bytes (`00401000`) at `+0x20` holding `value` (`0062e710` at 0, 0); then clears
/// `+0x2c` and `+0x30`. Answers `this`.
pub fn fn_0062e630(e: &mut Engine, this: Ptr, value: f32) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT, &args![this]);
    e.call(SECOND_BASE_CONSTRUCT, &args![base.wrapping_add(0x14)]);
    e.mem.set_u32(base, VTABLE);
    e.mem.set_u32(base.wrapping_add(0x14), SECOND_VTABLE);
    e.mem.set_u32(base.wrapping_add(0x1c), 1.0f32.to_bits());
    e.call(MEMBER_CONSTRUCT, &args![base.wrapping_add(0x34)]);
    e.mem.set_u32(base.wrapping_add(0x28), 1);
    let table = e.call(ALLOCATE, &args![4u32]).u32();
    e.mem.set_u32(base.wrapping_add(0x20), table);
    fn_0062e710(e, this, x87_quiet(value), 0, 0);
    e.mem.set_u32(base.wrapping_add(0x2c), 0);
    e.mem.set_u32(base.wrapping_add(0x30), 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_e630, fn_0062e630(Ptr, f32) -> Ptr),
        entry!(0x0062_e710, fn_0062e710(Ptr, f32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_0062e710_stores_the_converted_value_at_row_and_column() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(CONVERT_VALUE, |_, a| {
            (f64::from(f32::from_bits(a[0])) * 2.0).into_ret()
        });
        let table = e.mem.alloc(0x40);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, table);
        e.mem.set_u32(this + 0x28, 3);
        e.call(0x0062_e710, &args![this, 1.25f32, 1u32, 2u32]);
        assert_eq!(e.mem.f32(table + 7 * 4), 2.5);
        assert_eq!(calls(&e, CONVERT_VALUE), vec![vec![1.25f32.to_bits()]]);
    }

    #[test]
    fn at_0062e630_constructs_a_one_entry_table() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for addr in [BASE_CONSTRUCT, SECOND_BASE_CONSTRUCT, MEMBER_CONSTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        let table = e.mem.alloc(4);
        e.register_double(ALLOCATE, move |_, _| table.into_ret());
        e.register(CONVERT_VALUE, |_, a| {
            f64::from(f32::from_bits(a[0])).into_ret()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, 0x55);
        e.mem.set_u32(this + 0x30, 0x55);
        let nan = f32::from_bits(0x7f80_0001);
        assert_eq!(e.call(0x0062_e630, &args![this, nan]).u32(), this);
        assert_eq!(calls(&e, SECOND_BASE_CONSTRUCT), vec![vec![this + 0x14]]);
        assert_eq!(calls(&e, MEMBER_CONSTRUCT), vec![vec![this + 0x34]]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![4]]);
        assert_eq!(calls(&e, CONVERT_VALUE), vec![vec![0x7fc0_0001]]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u32(this + 0x14), SECOND_VTABLE);
        assert_eq!(e.mem.f32(this + 0x1c), 1.0);
        assert_eq!(e.mem.u32(this + 0x20), table);
        assert_eq!(e.mem.u32(this + 0x28), 1);
        assert_eq!(e.mem.u32(table), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
    }
}
