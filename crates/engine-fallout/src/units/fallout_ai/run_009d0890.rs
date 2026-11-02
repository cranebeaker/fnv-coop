//! `fallout/ai/run_009d0890` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d0980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01090ed4` (RTTI `.?AVCombatProcedureAttackRanged@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009d0980(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_0ed4);
    e.call(0x009c_a6c0, &args![this]);
}

/// `009ca680` (unnamed): the base constructor (`ECX` only).
const BASE_CONSTRUCT_009CA680: u32 = 0x009c_a680;
/// The vtable `01090ed4` (RTTI `.?AVCombatProcedureAttackRanged@@`).
const ATTACK_RANGED_VTABLE: u32 = 0x0109_0ed4;
/// The point `0x011f426c` (three words) the code copies as a zero point.
const ZERO_POINT: u32 = 0x011f_426c;
/// `00906f60` (unnamed): stores 0.0 in the two floats at `this` (`ECX` only).
const CLEAR_TWO_FLOATS: u32 = 0x0090_6f60;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009d0890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d0890 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`ECX` only): the constructor of the object with the vtable
/// `01090ed4` (RTTI `.?AVCombatProcedureAttackRanged@@`): the base constructor `009ca680`,
/// the vtable, `+0x10` cleared, the zero point (`0x011f426c`) copied to `+0x14`, and the
/// float pairs at `+0x20` to `+0x40` (`00906f60`). Returns `this`.
pub fn fn_009d0890(e: &mut Engine, this: Ptr) -> Ptr {
    let p = this.addr();
    e.call(BASE_CONSTRUCT_009CA680, &args![this]);
    e.mem.set_u32(p, ATTACK_RANGED_VTABLE);
    e.mem.set_u32(p + 0x10, 0);
    for word in 0..3 {
        let value = e.mem.u32(ZERO_POINT + 4 * word);
        e.mem.set_u32(p + 0x14 + 4 * word, value);
    }
    for offset in [0x20u32, 0x28, 0x30, 0x38, 0x40] {
        e.call(CLEAR_TWO_FLOATS, &args![p + offset]);
    }
    this
}

// Translated from 009d0950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor; the
/// destructor [`fn_009d0980`], then `operator delete` on `this` for bit 0 of `flags`.
/// Returns `this`.
pub fn fn_009d0950(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009d0980(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d_0980, fn_009d0980(Ptr)),
        entry!(0x009d_0890, fn_009d0890(Ptr) -> Ptr),
        entry!(0x009d_0950, fn_009d0950(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009d0980_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x009c_a6c0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009d_0980, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_0ed4);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_0ed4,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    #[test]
    fn fn_009d0890_constructs_the_procedure() {
        let mut e = Engine::new();
        e.map(ZERO_POINT & !0xfff, 0x1000);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(ZERO_POINT + 4 * i as u32, *v);
        }
        e.register(BASE_CONSTRUCT_009CA680, |_, _| Ret::default());
        e.register(CLEAR_TWO_FLOATS, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        e.mem.write(this, &[0xcd; 0x48]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009d_0890, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let cleared: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == CLEAR_TWO_FLOATS)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(cleared, vec![0x20, 0x28, 0x30, 0x38, 0x40]);
        assert!(log.contains(&(BASE_CONSTRUCT_009CA680, vec![this])));
        assert_eq!(e.mem.u32(this), ATTACK_RANGED_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.f32(this + 0x14), 1.0);
        assert_eq!(e.mem.f32(this + 0x1c), 3.0);
    }

    #[test]
    fn fn_009d0950_destroys_and_frees_for_bit_0() {
        let mut e = Engine::new();
        e.register(0x009c_a6c0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x48);
        for flags in [0u32, 1] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009d_0950, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let frees = log.iter().filter(|c| c.0 == OPERATOR_DELETE).count();
            assert_eq!(frees, flags as usize);
        }
        assert_eq!(e.mem.u32(this), ATTACK_RANGED_VTABLE);
    }
}
