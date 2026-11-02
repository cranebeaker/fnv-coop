//! `(unplaced)/run_00a6ea00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiTFixedStringMapBase<DFALL<NiTMapItem<NiFixedString,
/// NiAVObject*>>, NiAVObject*>` (its RTTI names the class).
const STRING_MAP_VTABLE: u32 = 0x0109_ca08;
/// That map's `RemoveAll` (engine map), `00aa01f0` (thiscall).
const STRING_MAP_REMOVE_ALL: u32 = 0x00aa_01f0;
/// Frees a block through the Ni allocator (`00aa10f0`, cdecl; null is
/// ignored).
const NI_FREE: u32 = 0x00aa_10f0;
/// The memory manager object and its free method (thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;

// Translated from 00a6ea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `NiTFixedStringMapBase<DFALL<NiTMapItem<
/// NiFixedString, NiAVObject*>>, NiAVObject*>` (slot 0 of its vtable
/// `0109ca08`, whose RTTI names the class): sets that vtable, empties the
/// map (`RemoveAll`, `00aa01f0`), frees the bucket array (+8) through
/// `00aa10f0`, and with bit 0 of `flags` frees the object through the
/// memory manager. Returns `this`.
pub fn fn_00a6ea00(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.mem.set_u32(this, STRING_MAP_VTABLE);
    e.call(STRING_MAP_REMOVE_ALL, &[this]);
    let buckets = e.mem.u32(this + 8);
    e.call(NI_FREE, &[buckets]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &[MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a6_ea00, fn_00a6ea00(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_map_destructor_empties_frees_and_deletes_on_request() {
        for (flags, deleted) in [(1u32, true), (0, false)] {
            let mut e = Engine::new();
            for addr in [STRING_MAP_REMOVE_ALL, NI_FREE, MEMORY_FREE] {
                e.register(addr, |_, _| 0u32.into_ret());
            }
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x7000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a6_ea00, &[this, flags]).u32(), this);
            let calls: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
            let mut expected = vec![(STRING_MAP_REMOVE_ALL, vec![this]), (NI_FREE, vec![0x7000])];
            if deleted {
                expected.push((MEMORY_FREE, vec![MEMORY_MANAGER, this]));
            }
            assert_eq!(calls, expected);
            assert_eq!(e.mem.u32(this), STRING_MAP_VTABLE);
        }
    }
}
