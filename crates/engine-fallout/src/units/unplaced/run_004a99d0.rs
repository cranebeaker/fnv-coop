//! `(unplaced)/run_004a99d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable [`fn_004a99d0`] stores (`0101ed4c`; RTTI
/// `NiTMapBase<DFALL<NiTMapItem<TESObjectCELL*,bool>>,...>`).
const CELL_MAP_BASE_VTABLE: u32 = 0x0101_ed4c;
/// `00438af0` (`ECX` = the map): removes every item.
const REMOVE_ALL: u32 = 0x0043_8af0;
/// `00aa10f0` (`cdecl`, one word): frees a block.
const FREE: u32 = 0x00aa_10f0;

// Translated from 004a99d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<..., TESObjectCELL*, bool>` destructor body (by its vtable's RTTI; the
/// engine map does not name it; `ECX`, plain `RET`; run by `004a99b0` and the scalar
/// deleting destructor at `004a9a20`): stores the vtable (`0101ed4c`), removes every item
/// (`00438af0`) and frees the bucket array at +8 (`00aa10f0`).
pub fn fn_004a99d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), CELL_MAP_BASE_VTABLE);
    e.call(REMOVE_ALL, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE, &args![buckets]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004a_99d0, fn_004a99d0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004a99d0_stores_the_vtable_and_frees_the_buckets() {
        let mut e = Engine::new();
        e.register(REMOVE_ALL, |e, a| {
            assert_eq!(e.mem.u32(a[0]), CELL_MAP_BASE_VTABLE);
            Ret::default()
        });
        e.register(FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x5550);
        e.call_log = Some(vec![]);
        e.call(0x004a_99d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(REMOVE_ALL, vec![this]), (FREE, vec![0x5550])]);
    }
}
