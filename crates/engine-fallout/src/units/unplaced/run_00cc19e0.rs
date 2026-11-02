//! `(unplaced)/run_00cc19e0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable this destructor sets (`010c9490`; RTTI
/// `.?AV?$NiTLargeArray@V?$NiPointer@VNiTimeController@@@@...`).
const VTABLE_LARGE_ARRAY: u32 = 0x010c_9490;
/// The buffer release `00aa10f0` (cdecl: block; nothing for null).
const BUFFER_FREE: u32 = 0x00aa_10f0;
/// The `MemoryManager` singleton and its deallocation (`00aa4060`, thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00cc19e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the deleting destructor of
/// `NiTLargeArray<NiPointer<NiTimeController>>` (its vtable's RTTI): the vtable, the buffer
/// at +4 released (`00aa10f0`), then with bit 0 of `flags` the object freed through the
/// memory manager. Returns `this`.
pub fn fn_00cc19e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let buffer = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), VTABLE_LARGE_ARRAY);
    e.call(BUFFER_FREE, &args![buffer]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cc_19e0, fn_00cc19e0(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn releases_the_buffer_and_frees_with_bit_0() {
        for flags in [0u32, 1, 2] {
            let mut e = Engine::new();
            e.register(BUFFER_FREE, |_, _| Ret::default());
            e.register(DEALLOCATE, |_, _| Ret::default());
            let this = e.mem.alloc(0x18);
            e.mem.set_u32(this + 4, 0x6000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00cc_19e0, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (BUFFER_FREE, vec![0x6000]));
            assert_eq!(
                log.iter().filter(|c| c.0 == DEALLOCATE).count(),
                (flags & 1) as usize
            );
            assert_eq!(e.mem.u32(this), VTABLE_LARGE_ARRAY);
        }
    }
}
