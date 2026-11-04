//! `fallout shared/run_0057cb20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Called on the array first (`00626870`, `this` only: `00626790` on the
/// data and size, then the size word at +4 cleared).
const ARRAY_CLEAR: u32 = 0x0062_6870;
/// The array's capacity (`0062a100`: the word at +8 masked with
/// `0x3fffffff`).
const ARRAY_CAPACITY: u32 = 0x0062_a100;
/// Hands the storage back (`0057cc10`: `this` = allocator, data, capacity;
/// it calls the allocator's vtable slot +0x10 with the data, capacity * 8,
/// the allocator and 8).
const ALLOCATOR_FREE: u32 = 0x0057_cc10;

// Translated from 0057cb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases a Havok-style array (no Xbox PDB name; `thiscall`, `RET 4`):
/// `00626870` on it, then, unless the capacity word at +8 has its
/// `0x80000000` (do not free) bit, its data (+0) goes back to `allocator`
/// through `0057cc10` with the capacity `0062a100` gives. The data becomes 0
/// and the capacity word `0x80000000`.
pub fn fn_0057cb20(e: &mut Engine, this: Ptr, allocator: Ptr) {
    e.call(ARRAY_CLEAR, &args![this]);
    if e.mem.u32(this.addr() + 8) & 0x8000_0000 == 0 {
        let size = e.call(ARRAY_CAPACITY, &args![this]).u32();
        let data = e.mem.u32(this.addr());
        e.call(ALLOCATOR_FREE, &args![allocator, data, size]);
    }
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 8, 0x8000_0000);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0057cb20, fn_0057cb20(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ARRAY_CLEAR, |_, _| Ret::default());
        e.register(ARRAY_CAPACITY, |_, _| Ret {
            eax: 0x40,
            ..Ret::default()
        });
        e.register(ALLOCATOR_FREE, |_, _| Ret::default());
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn owned_storage_goes_back_to_the_allocator() {
        let mut e = engine();
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array, 0x5000);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, 8);
        e.call_log = Some(vec![]);
        e.call(0x0057_cb20, &args![array, 0x9000u32]);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array]]);
        assert_eq!(calls(&e, ALLOCATOR_FREE), vec![vec![0x9000, 0x5000, 0x40]]);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }

    #[test]
    fn borrowed_storage_is_only_dropped() {
        let mut e = engine();
        let array = e.mem.alloc(0xc);
        e.mem.set_u32(array, 0x5000);
        e.mem.set_u32(array + 8, 0x8000_0004);
        e.call_log = Some(vec![]);
        e.call(0x0057_cb20, &args![array, 0x9000u32]);
        assert!(calls(&e, ALLOCATOR_FREE).is_empty());
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }
}
