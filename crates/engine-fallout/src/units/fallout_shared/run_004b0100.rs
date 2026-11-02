//! `fallout shared/run_004b0100` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiTMapBase` vtable this destructor stores (RTTI
/// `.?AV?$NiTMapBase@V?$NiTPointerAllocator@I@@...`).
const MAP_VTABLE: u32 = 0x0101_fbfc;
/// `00438af0` (thiscall): the map's teardown.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `00aa10f0` (cdecl, pointer): releases a block.
const FREE_BLOCK: u32 = 0x00aa_10f0;

// Translated from 004b0100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an `NiTMapBase` destructor: stores the
/// vtable, tears the map down (`00438af0`) and releases its bucket table at
/// +8 (`00aa10f0`). `EAX` is the release call's result, returned here.
pub fn fn_004b0100(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.call(FREE_BLOCK, &args![table]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004b_0100, fn_004b0100(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_004b0100_stores_the_vtable_tears_down_and_frees() {
        let mut e = Engine::new();
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0x7000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004b_0100, &args![this]).u32(), 0x7001);
        assert_eq!(e.mem.u32(this), MAP_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MAP_REMOVE_ALL, vec![this]));
        assert_eq!(log[2], (FREE_BLOCK, vec![0x7000]));
    }
}
