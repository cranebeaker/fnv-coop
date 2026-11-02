//! `fallout shared/pathfinding/run_006c00d0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006c0170 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `006c0150`, which stores the vtable `0106c3c4` at
/// `+0`, and returns nothing. Parameters: one word, `this`; the disassembly of the function
/// ends in a plain `RET` and passes only `ECX` on. `006c0150`'s disassembly ends on that
/// vtable store with no result computed (Ghidra types it void); its `MOV EAX,[EBP-4]` only
/// addresses the store.
pub fn fn_006c0170(e: &mut Engine, this: Ptr) {
    e.call(0x006c_0150, &args![this]);
}

/// The `hkpEntityActivationListener` vtable (its RTTI locator names
/// `.?AVhkpEntityActivationListener@@`).
const ENTITY_ACTIVATION_LISTENER_VTABLE: u32 = 0x0106_c3c4;
/// The game's `operator delete` (`00401030`, cdecl, the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 006c0100 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `hkpEntityActivationListener` constructor: stores the interface's
/// vtable and returns `this`.
pub fn fn_006c0100(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem
        .set_u32(this.addr(), ENTITY_ACTIVATION_LISTENER_VTABLE);
    this
}

// Translated from 006c0150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpEntityActivationListener` destructor body: stores the
/// interface's vtable (`EAX` is left holding `this` only as the store's
/// address; the callers do not read it).
pub fn fn_006c0150(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), ENTITY_ACTIVATION_LISTENER_VTABLE);
}

// Translated from 006c00d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpEntityActivationListener` scalar deleting destructor: runs the
/// destructor (`006c0170`), frees the object when bit 0 of `flags` is set,
/// and returns `this`.
pub fn fn_006c00d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006c0170(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006c0120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpEntityActivationListener` scalar deleting destructor
/// (`thiscall`; flags; `RET 4`): the destructor body `006c0150`, then the
/// object is freed (`00401030`) when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_006c0120(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006c0150(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c_0170, fn_006c0170(Ptr)),
        entry!(0x006c0100, fn_006c0100(Ptr) -> Ptr),
        entry!(0x006c0150, fn_006c0150(Ptr)),
        entry!(0x006c00d0, fn_006c00d0(Ptr, u32) -> Ptr),
        entry!(0x006c_0120, fn_006c0120(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_006c0170_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x006c_0150, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006c_0170, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x006c_0150), vec![vec![0x5000]]);
    }

    #[test]
    fn constructor_006c0100_stores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        assert_eq!(e.call(0x006c_0100, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ENTITY_ACTIVATION_LISTENER_VTABLE);
    }

    #[test]
    fn destructor_006c0150_stores_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.call(0x006c_0150, &args![this]);
        assert_eq!(e.mem.u32(this), ENTITY_ACTIVATION_LISTENER_VTABLE);
    }

    #[test]
    fn deleting_destructor_006c00d0_frees_only_on_bit_0() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006c_00d0, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ENTITY_ACTIVATION_LISTENER_VTABLE);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x006c_00d0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn fn_006c0120_destroys_and_frees_on_bit_0() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(8);
        for (flags, frees) in [(0u32, 0usize), (1, 1), (2, 0), (3, 1)] {
            e.mem.set_u32(this, 0x1234);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006c_0120, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), ENTITY_ACTIVATION_LISTENER_VTABLE);
            let log = e.call_log.take().unwrap();
            let freed: Vec<_> = log.iter().filter(|(a, _)| *a == OPERATOR_DELETE).collect();
            assert_eq!(freed.len(), frees);
            if frees == 1 {
                assert_eq!(freed[0].1, vec![this]);
            }
        }
    }
}
