//! `fallout/ai/run_009d5ad0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d5b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01091984` (RTTI `.?AVCombatProcedureEvade@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `009ca6c0`. One word, plain `RET`, returns nothing.
pub fn fn_009d5b40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_1984);
    e.call(0x009c_a6c0, &args![this]);
}

/// `009ca680` (unnamed): the base constructor (`ECX` only).
const BASE_CONSTRUCT_009CA680: u32 = 0x009c_a680;
/// The vtable `01091984` (RTTI `.?AVCombatProcedureEvade@@`).
const EVADE_VTABLE: u32 = 0x0109_1984;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009d5ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the constructor of the object with the
/// vtable `01091984` (RTTI `.?AVCombatProcedureEvade@@`): the base constructor `009ca680`,
/// the vtable, `+0x10` cleared and `+0x14` set to `value`. Returns `this`.
pub fn fn_009d5ad0(e: &mut Engine, this: Ptr, value: u32) -> Ptr {
    let p = this.addr();
    e.call(BASE_CONSTRUCT_009CA680, &args![this]);
    e.mem.set_u32(p, EVADE_VTABLE);
    e.mem.set_u32(p + 0x10, 0);
    e.mem.set_u32(p + 0x14, value);
    this
}

// Translated from 009d5b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor; the
/// destructor [`fn_009d5b40`], then `operator delete` on `this` for bit 0 of `flags`.
/// Returns `this`.
pub fn fn_009d5b10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009d5b40(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009d_5b40, fn_009d5b40(Ptr)),
        entry!(0x009d_5ad0, fn_009d5ad0(Ptr, u32) -> Ptr),
        entry!(0x009d_5b10, fn_009d5b10(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009d5b40_stores_the_vtable_then_forwards_this() {
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
        e.call(0x009d_5b40, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x009c_a6c0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_1984);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_1984,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    #[test]
    fn fn_009d5ad0_constructs_the_procedure() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCT_009CA680, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xcd; 0x18]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009d_5ad0, &args![this, 0x42u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_CONSTRUCT_009CA680, vec![this])));
        assert_eq!(e.mem.u32(this), EVADE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0x42);
        assert_eq!(e.mem.u32(this + 4), 0xcdcd_cdcd);
    }

    #[test]
    fn fn_009d5b10_destroys_and_frees_for_bit_0() {
        let mut e = Engine::new();
        e.register(0x009c_a6c0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        for flags in [0u32, 1] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009d_5b10, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let frees = log.iter().filter(|c| c.0 == OPERATOR_DELETE).count();
            assert_eq!(frees, flags as usize);
        }
        assert_eq!(e.mem.u32(this), EVADE_VTABLE);
    }
}
