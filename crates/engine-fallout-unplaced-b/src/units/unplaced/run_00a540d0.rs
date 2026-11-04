//! `(unplaced)/run_00a540d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTMapBase<NiTPointerAllocator<unsigned int>, NiNode*, unsigned int>`'s
/// vtable (its RTTI names the class).
const MAP_BASE_VTABLE: u32 = 0x0109_a264;
/// The map's `RemoveAll` (`00b601a0`, thiscall).
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
/// Frees a block through the Ni allocator (`00aa10f0`, cdecl; null is
/// ignored).
const NI_FREE: u32 = 0x00aa_10f0;
/// The memory manager object and its free method (thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;

// Translated from 00a540d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `NiTMapBase<NiTPointerAllocator<unsigned
/// int>, NiNode*, unsigned int>` (slot 0 of its vtable `0109a264`, whose
/// RTTI names the class): sets that vtable, empties the map (`00b601a0`),
/// frees its bucket array (+8) through `00aa10f0`, and with bit 0 of
/// `flags` frees the object through the memory manager. Returns `this`.
pub fn fn_00a540d0(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.mem.set_u32(this, MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &[this]);
    let buckets = e.mem.u32(this + 8);
    e.call(NI_FREE, &[buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &[MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_40d0, fn_00a540d0(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_destructor_empties_frees_and_deletes_on_request() {
        for (flags, deleted) in [(1u32, true), (2, false)] {
            let mut e = Engine::new();
            for addr in [MAP_REMOVE_ALL, NI_FREE, MEMORY_FREE] {
                e.register(addr, |_, _| 0u32.into_ret());
            }
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x5000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a5_40d0, &[this, flags]).u32(), this);
            let calls: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
            let mut expected = vec![(MAP_REMOVE_ALL, vec![this]), (NI_FREE, vec![0x5000])];
            if deleted {
                expected.push((MEMORY_FREE, vec![MEMORY_MANAGER, this]));
            }
            assert_eq!(calls, expected);
            assert_eq!(e.mem.u32(this), MAP_BASE_VTABLE);
        }
    }
}
