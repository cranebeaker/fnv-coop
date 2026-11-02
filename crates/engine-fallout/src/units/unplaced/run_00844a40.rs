//! `(unplaced)/run_00844a40` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00844a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates through the folded map's allocator subobject at +0xc.
pub fn new_item(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x006f99e0, &args![this.addr().wrapping_add(0xc)])
        .ptr()
}

/// The vtable `00844a40` stores (RTTI
/// `.?AV?$NiTMapBase@V?$DFALL@V?$NiTMapItem@IPAVBGSReconstructCellBlocksMap@@@@@@...`).
const VTABLE: u32 = 0x0107_ece8;
/// `00438af0` (`thiscall`): empties the map.
const REMOVE_ALL_00438AF0: u32 = 0x0043_8af0;
/// `00aa10f0` (`cdecl`, one word): frees the bucket array.
const FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00844a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): the destructor body of
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int, BGSReconstructCellBlocksMap *>>, ..>` (by the
/// RTTI of the vtable it stores): the vtable `0107ece8`, `00438af0` on `this`, then the
/// word at +8 (the bucket array) to `00aa10f0`.
pub fn fn_00844a40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(REMOVE_ALL_00438AF0, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE_00AA10F0, &args![buckets]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00844a70, new_item(Ptr) -> Ptr),
        entry!(0x0084_4a40, fn_00844a40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empties_then_frees_the_buckets() {
        let mut e = Engine::new();
        e.register(REMOVE_ALL_00438AF0, |e, a| {
            e.mem.set_u32(a[0] + 8, 0x6600);
            Ret::default()
        });
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0084_4a40, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (REMOVE_ALL_00438AF0, vec![this]),
                (FREE_00AA10F0, vec![0x6600])
            ]
        );
        assert_eq!(e.mem.u32(this), VTABLE);
    }
}
