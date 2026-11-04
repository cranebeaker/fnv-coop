//! `fallout shared/pathfinding/run_006b8310` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006b8310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Folded map allocator release, forwarding even a null item.
pub fn free_item(e: &mut Engine, _this: Ptr, item: Ptr) {
    e.call(0x00401030, &args![item]);
}

// Translated from 006b8340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b8340 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// A constructor (no name in the engine map): the base part (`006b8d90`),
/// the vtable `0106c22c`, `006815c0` on the members at +0x2058 and +0x2068,
/// the member at +0x2084 (`00501cd0`), and the words at +0x2078, +0x2064 and
/// +0x2074 cleared, in that order. Returns `this`.
pub fn fn_006b8340(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x006b_8d90, &args![this]);
    e.mem.set_u32(base, 0x0106_c22c);
    e.call(0x0068_15c0, &args![base + 0x2058]);
    e.call(0x0068_15c0, &args![base + 0x2068]);
    e.call(0x0050_1cd0, &args![base + 0x2084]);
    e.mem.set_u32(base + 0x2078, 0);
    e.mem.set_u32(base + 0x2064, 0);
    e.mem.set_u32(base + 0x2074, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b8310, free_item(Ptr, Ptr)),
        entry!(0x006b_8340, fn_006b8340(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod b039_tests {
    use super::*;

    #[test]
    fn fn_006b8340_constructs_in_order() {
        let mut e = Engine::new();
        let callees = [0x006b_8d90u32, 0x0068_15c0, 0x0050_1cd0];
        for callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x20a0);
        for offset in [0x2064u32, 0x2074, 0x2078] {
            e.mem.set_u32(this + offset, 0x5555);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_8340, &args![this]).u32(), this);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| callees.contains(a))
            .collect();
        assert_eq!(
            log,
            vec![
                (0x006b_8d90, vec![this]),
                (0x0068_15c0, vec![this + 0x2058]),
                (0x0068_15c0, vec![this + 0x2068]),
                (0x0050_1cd0, vec![this + 0x2084]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0106_c22c);
        for offset in [0x2064u32, 0x2074, 0x2078] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }
}
