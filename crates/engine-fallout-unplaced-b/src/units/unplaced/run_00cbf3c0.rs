//! `(unplaced)/run_00cbf3c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread's memory router (`00c85750`, unit `hkmemoryrouter.cpp`: `TlsGetValue` of the
/// slot at `01268108`; no arguments).
const MEMORY_ROUTER_INSTANCE: u32 = 0x00c8_5750;

// Translated from 00cbf3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears and frees the `hkArray` at `this + 0x10` (data +0x10,
/// size +0x14, capacity and flags +0x18). Takes the allocator at +0x10 of the thread's
/// memory router (`00c85750`), sets the size to 0, and when the capacity word is not
/// negative (the "don't deallocate" bit 31 clear) frees the data through that allocator's
/// vtable slot +0x10 (data, `(capacity & 0x3fffffff) * 4` bytes); then the capacity word
/// becomes `0x80000000` and the data null.
pub fn fn_00cbf3c0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let router = e.call(MEMORY_ROUTER_INSTANCE, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.i32(this + 0x18);
    e.mem.set_u32(this + 0x14, 0);
    if capacity >= 0 {
        let bytes = (capacity as u32 & 0x3fff_ffff).wrapping_mul(4);
        let data = e.mem.u32(this + 0x10);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(this + 0x18, 0x8000_0000);
    e.mem.set_u32(this + 0x10, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cb_f3c0, fn_00cbf3c0(Ptr))]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn frees_only_owned_storage_and_resets_the_array() {
        for (capacity, frees) in [(0x4000_0003u32, true), (0x8000_0003, false)] {
            let mut e = Engine::new();
            let vtable = e.mem.alloc(0x14);
            e.mem.set_u32(vtable + 0x10, 0x0079_0000);
            e.register(0x0079_0000, |_, _| Ret::default());
            let allocator = e.mem.alloc(4);
            e.mem.set_u32(allocator, vtable);
            let router = e.mem.alloc(0x14);
            e.mem.set_u32(router + 0x10, allocator);
            e.register_double(MEMORY_ROUTER_INSTANCE, move |_, _| router.into_ret());
            let this = e.mem.alloc(0x1c);
            e.mem.set_u32(this + 0x10, 0x6000);
            e.mem.set_u32(this + 0x14, 3);
            e.mem.set_u32(this + 0x18, capacity);
            e.call_log = Some(vec![]);
            e.call(0x00cb_f3c0, &args![this]);
            let log: Vec<_> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|c| c.0 == 0x0079_0000)
                .collect();
            if frees {
                assert_eq!(log, vec![(0x0079_0000, vec![allocator, 0x6000, 12])]);
            } else {
                assert!(log.is_empty());
            }
            assert_eq!(e.mem.u32(this + 0x10), 0);
            assert_eq!(e.mem.u32(this + 0x14), 0);
            assert_eq!(e.mem.u32(this + 0x18), 0x8000_0000);
        }
    }
}
