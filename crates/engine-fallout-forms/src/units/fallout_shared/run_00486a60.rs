//! `fallout shared/run_00486a60` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The map's vtable (`0101ca98`; RTTI `NiTMapBase<NiTPointerAllocator<...>,...>`,
/// read through its locator).
const MAP_VTABLE: u32 = 0x0101_ca98;
/// `00438af0` (`thiscall`, no stack word): empties the map.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `00aa10f0` (`cdecl`, one word): frees the bucket array.
const FREE_BUCKETS: u32 = 0x00aa_10f0;

// Translated from 00486a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of an `NiTMapBase` instance (the class from the
/// vtable's RTTI; the engine map gives it no name; no stack word): the
/// vtable `0101ca98`, the map emptied (`00438af0`), then its bucket array
/// (+8) freed (`00aa10f0`).
pub fn fn_00486a60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE_BUCKETS, &args![buckets]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048_6a60, fn_00486a60(Ptr))]
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_00486a60_empties_then_frees_the_buckets() {
        let mut e = Engine::new();
        // The double reads the bucket word after the vtable store.
        e.register(MAP_REMOVE_ALL, |e, a| {
            e.mem.set_u32(a[0] + 8, 0x7000);
            Ret::default()
        });
        e.register(FREE_BUCKETS, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0048_6a60, &args![this]);
        assert_eq!(e.mem.u32(this), MAP_VTABLE);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x0048_6a60)
            .collect();
        assert_eq!(
            log,
            vec![(MAP_REMOVE_ALL, vec![this]), (FREE_BUCKETS, vec![0x7000])]
        );
    }
}
