//! `fallout/ai/run_009cbad0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009cbb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01090aec` (RTTI `.?AVCombatProcedureAttackLow@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009cbb50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_0aec);
    e.call(0x009c_a6c0, &args![this]);
}

// Translated from 009cbb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: returns 3 (`thiscall`, `this` unread). The
/// body is shared by the linker (several vtables point to it).
pub fn fn_009cbb10(_e: &mut Engine, _this: Ptr) -> u32 {
    3
}

/// `009ca680` (unnamed, `combatprocedureactivateobject.cpp` per the engine map): the base
/// constructor (`ECX` only).
const BASE_CONSTRUCT_009CA680: u32 = 0x009c_a680;
/// The vtable `01090aec` (RTTI `.?AVCombatProcedureAttackLow@@`).
const ATTACK_LOW_VTABLE: u32 = 0x0109_0aec;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009cbad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the constructor of the object with the vtable
/// `01090aec` (RTTI `.?AVCombatProcedureAttackLow@@`): the base constructor `009ca680`, the
/// vtable, `+0x10` cleared, the float at `+0x14` set to 0.0 and `+0x1c` cleared. Returns
/// `this`.
pub fn fn_009cbad0(e: &mut Engine, this: Ptr) -> Ptr {
    let p = this.addr();
    e.call(BASE_CONSTRUCT_009CA680, &args![this]);
    e.mem.set_u32(p, ATTACK_LOW_VTABLE);
    e.mem.set_u32(p + 0x10, 0);
    e.mem.set_u32(p + 0x14, 0.0f32.to_bits());
    e.mem.set_u32(p + 0x1c, 0);
    this
}

// Translated from 009cbb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor; the
/// destructor [`fn_009cbb50`], then `operator delete` on `this` for bit 0 of `flags`.
/// Returns `this`.
pub fn fn_009cbb20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009cbb50(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009c_bb50, fn_009cbb50(Ptr)),
        entry!(0x009c_bb10, fn_009cbb10(Ptr) -> u32),
        entry!(0x009c_bad0, fn_009cbad0(Ptr) -> Ptr),
        entry!(0x009c_bb20, fn_009cbb20(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod get_type_tests {
    use super::*;

    #[test]
    fn at_009cbb10_returns_three() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009c_bb10, &args![0x1234u32]).u32(), 3);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009cbb50_stores_the_vtable_then_forwards_this() {
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
        e.call(0x009c_bb50, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_0aec);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_0aec,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    #[test]
    fn fn_009cbad0_constructs_the_procedure() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_009CA680, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xcd; 0x20]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009c_bad0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_CONSTRUCT_009CA680, vec![this])));
        assert_eq!(e.mem.u32(this), ATTACK_LOW_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }

    #[test]
    fn fn_009cbb20_destroys_and_frees_for_bit_0() {
        let mut e = Engine::new();
        e.register(0x009c_a6c0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        for flags in [0u32, 1, 2, 3] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009c_bb20, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let frees = log.iter().filter(|c| c.0 == OPERATOR_DELETE).count();
            assert_eq!(frees, (flags & 1) as usize);
            assert!(log.contains(&(0x009c_a6c0, vec![this])));
        }
        assert_eq!(e.mem.u32(this), ATTACK_LOW_VTABLE);
    }
}
