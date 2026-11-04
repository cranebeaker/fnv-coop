//! `(unplaced)/run_00559080` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiTMapBase<DFALL<NiTMapItem<TESForm*,TESObjectREFR*>>,
/// TESForm*,TESObjectREFR*>` (RTTI
/// `.?AV?$NiTMapBase@V?$DFALL@V?$NiTMapItem@PAVTESForm@@PAVTESObjectREFR@@@@@@PAVTESForm@@PAVTESObjectREFR@@@@`).
const MAP_VTABLE: u32 = 0x0102_f378;
/// The map's removal of its items (`00438af0`, thiscall, no stack words).
const REMOVE_ALL_00438AF0: u32 = 0x0043_8af0;
/// The allocator's free (`00aa10f0`, cdecl, one word), given the hash
/// table (+8).
const NI_FREE: u32 = 0x00aa_10f0;
const HASH_TABLE: u32 = 0x08;

// Translated from 00559080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map's destructor body (by the vtable it stores; unnamed in the
/// engine map): the vtable, its items removed (`00438af0`), then the hash
/// table freed (`00aa10f0`).
pub fn fn_00559080(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    e.call(REMOVE_ALL_00438AF0, &args![this]);
    let table = e.mem.u32(this.addr() + HASH_TABLE);
    e.call(NI_FREE, &args![table]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0055_9080, fn_00559080(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_vtable_removes_the_items_then_frees_the_table() {
        let mut e = Engine::new();
        e.register(REMOVE_ALL_00438AF0, |e, a| {
            // The table pointer is read after the removal.
            e.mem.set_u32(a[0] + HASH_TABLE, 0x0abc_0000);
            Ret::default()
        });
        e.register(NI_FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0055_9080, &args![this]);
        assert_eq!(e.mem.u32(this), MAP_VTABLE);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.into_iter().filter(|(a, _)| *a != 0x0055_9080).collect();
        assert_eq!(
            calls,
            vec![
                (REMOVE_ALL_00438AF0, vec![this]),
                (NI_FREE, vec![0x0abc_0000])
            ]
        );
    }
}
