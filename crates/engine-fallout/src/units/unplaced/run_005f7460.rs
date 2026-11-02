//! `(unplaced)/run_005f7460` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks `this` through its vtable slot +8 with `value` and answers the
/// integer result as the exe's `FILD` leaves it in ST0 (exact, so
/// answered as a `double`; the engine map has no name).
pub fn fn_005f7480(e: &mut Engine, this: Ptr, value: u32) -> f64 {
    let result = e.vcall(this.addr(), 8, &args![value]).i32();
    f64::from(result)
}

/// `_ftol2` (`00ec62c0`): truncates ST0 to an integer in `EAX` (ST0 passed as a leading
/// `f64` argument).
const FTOL: u32 = 0x00ec_62c0;

// Translated from 005f7460 (decompiled, FalloutNV.exe 1.4.0.525)
/// A base-adjusting forwarder (a slot of the vtables at `01048df0`, `0104913c`, `0104a188`;
/// unnamed in the engine map; `ECX`, plain `RET`): `0047ded0` on the object 0xd0 bytes
/// before `this`, answering its `EAX`.
pub fn fn_005f7460(e: &mut Engine, this: Ptr) -> u32 {
    let object = this.addr().wrapping_sub(0xd0);
    e.call(0x0047_ded0, &args![object]).u32()
}

// Translated from 005f74b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A slot of the vtables at `010490ec`, `0104943c`, `0104a484` (unnamed in the engine map;
/// `ECX` and two stack words, `RET 8`): truncates the `float` `value` (`_ftol2`) and hands
/// it with `index` to `this`'s own vtable slot +0x194, answering its `EAX`.
pub fn fn_005f74b0(e: &mut Engine, this: Ptr, index: u32, value: u32) -> u32 {
    let whole = e.call(FTOL, &args![x87_load32(value)]).u32();
    e.vcall(this.addr(), 0x194, &args![index, whole]).u32()
}

// Translated from 005f74e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A slot of the vtables at `010490f8`, `01049448`, `0104a490` (unnamed in the engine map;
/// `ECX` and two stack words, `RET 8`): the integer the sub-object at +0x100 answers for
/// `index` (its vtable slot +8) plus `delta`, handed with `index` to `this`'s vtable slot
/// +0x194; answers its `EAX`.
pub fn fn_005f74e0(e: &mut Engine, this: Ptr, index: u32, delta: u32) -> u32 {
    let current = e.vcall(this.addr() + 0x100, 8, &args![index]).u32();
    let value = current.wrapping_add(delta);
    e.vcall(this.addr(), 0x194, &args![index, value]).u32()
}

// Translated from 005f7520 (decompiled, FalloutNV.exe 1.4.0.525)
/// A slot of the vtables at `010490f4`, `01049444`, `0104a48c` (unnamed in the engine map;
/// `ECX` and two stack words, `RET 8`): the float the sub-object at +0x100 answers for
/// `index` (its vtable slot +0xc, ST0) plus the `float` `delta` (`FADD`, stored as a
/// `float`), handed with `index` to `this`'s vtable slot +0x190; answers its `EAX`.
pub fn fn_005f7520(e: &mut Engine, this: Ptr, index: u32, delta: u32) -> u32 {
    let current = e.vcall(this.addr() + 0x100, 0xc, &args![index]).f64();
    let sum = x87_store32(x87_add(current, x87_m32(delta)));
    e.vcall(this.addr(), 0x190, &args![index, x87_quiet_bits(sum)])
        .u32()
}

// Translated from 005f7570 (decompiled, FalloutNV.exe 1.4.0.525)
/// A slot of the vtables at `01048dec`, `01049138`, `0104a184` (unnamed in the engine map;
/// `ECX`, plain `RET`): the address 0x100 bytes before `this` (the object the sub-object
/// at +0x100 belongs to).
pub fn fn_005f7570(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_sub(0x100))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_7480, fn_005f7480(Ptr, u32) -> f64),
        entry!(0x005f_7460, fn_005f7460(Ptr) -> u32),
        entry!(0x005f_74b0, fn_005f74b0(Ptr, u32, u32) -> u32),
        entry!(0x005f_74e0, fn_005f74e0(Ptr, u32, u32) -> u32),
        entry!(0x005f_7520, fn_005f7520(Ptr, u32, u32) -> u32),
        entry!(0x005f_7570, fn_005f7570(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_8_result_is_loaded_as_an_integer() {
        const TARGET: u32 = 0x0300_0000;
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0xc);
        e.mem.set_u32(vtable + 8, TARGET);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        e.register(TARGET, |_, a| (a[1].wrapping_neg()).into_ret());
        assert_eq!(e.call(0x005f_7480, &args![this, 7u32]).f64(), -7.0);
        assert_eq!(
            e.call(0x005f_7480, &args![this, 0x7fff_ffffu32]).f64(),
            -2_147_483_647.0
        );
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object with a vtable whose slots +0x190/+0x194 are doubles, and a sub-object at
    /// +0x100 whose slot +8 answers 40 and slot +0xc answers 1.5.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x66];
        slots[0x190 / 4] = 0x0060_0190;
        slots[0x194 / 4] = 0x0060_0194;
        e.put_vtable(0x0070_0000, &slots);
        e.put_vtable(0x0071_0000, &[0, 0, 0x0061_0008, 0x0061_000c]);
        e.register(0x0060_0190, |_, _| 0x190u32.into_ret());
        e.register(0x0060_0194, |_, _| 0x194u32.into_ret());
        e.register(0x0061_0008, |_, _| 40u32.into_ret());
        e.register(0x0061_000c, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this, 0x0070_0000);
        e.mem.set_u32(this + 0x100, 0x0071_0000);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_005f7460_and_005f7570_adjust_this() {
        let mut e = Engine::new();
        e.register(0x0047_ded0, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x005f_7460, &args![0x10d0u32]).u32(), 0x1000);
        assert_eq!(e.call(0x005f_7570, &args![0x1100u32]).u32(), 0x1000);
    }

    #[test]
    fn fn_005f74b0_sets_the_truncated_value() {
        let (mut e, this) = engine();
        assert_eq!(
            e.call(0x005f_74b0, &args![this, 5u32, 7.75f32.to_bits()])
                .u32(),
            0x194
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0060_0194), vec![vec![this, 5, 7]]);
    }

    #[test]
    fn fn_005f74e0_adds_to_the_integer() {
        let (mut e, this) = engine();
        e.call(0x005f_74e0, &args![this, 5u32, (-2i32) as u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0061_0008), vec![vec![this + 0x100, 5]]);
        assert_eq!(calls(&log, 0x0060_0194), vec![vec![this, 5, 38]]);
    }

    #[test]
    fn fn_005f7520_adds_to_the_float() {
        let (mut e, this) = engine();
        assert_eq!(
            e.call(0x005f_7520, &args![this, 5u32, 0.25f32.to_bits()])
                .u32(),
            0x190
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x0060_0190),
            vec![vec![this, 5, 1.75f32.to_bits()]]
        );
    }
}
