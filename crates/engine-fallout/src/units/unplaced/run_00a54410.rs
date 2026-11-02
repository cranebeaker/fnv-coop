//! `(unplaced)/run_00a54410` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiTArray<NiTPrimitiveSet<NiBoneLODController::SkinInfo*>*,
/// ...>` (its RTTI names the class; the derived `NiTPrimitiveArray`'s vtable
/// `0109a2c8` holds the same destructor in slot 0).
const ARRAY_VTABLE: u32 = 0x0109_a2ac;
/// Frees a block through the Ni allocator (`00aa10f0`, cdecl; null is
/// ignored).
const NI_FREE: u32 = 0x00aa_10f0;
/// The memory manager object and its free method (thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;

// Translated from 00a54410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTArray` of
/// `NiTPrimitiveSet<NiBoneLODController::SkinInfo*>*` (slot 0 of the vtables
/// `0109a2ac` and `0109a2c8`, whose RTTI names `NiTArray` and
/// `NiTPrimitiveArray`): reads the item array (+4), sets the `NiTArray`
/// vtable, frees the array through `00aa10f0`, and with bit 0 of `flags`
/// frees the object through the memory manager. Returns `this`.
pub fn fn_00a54410(e: &mut Engine, this: u32, flags: u32) -> u32 {
    let items = e.mem.u32(this + 4);
    e.mem.set_u32(this, ARRAY_VTABLE);
    e.call(NI_FREE, &[items]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &[MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_4410, fn_00a54410(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_destructor_frees_its_items() {
        for (flags, deleted) in [(3u32, true), (0, false)] {
            let mut e = Engine::new();
            e.register(NI_FREE, |_, _| 0u32.into_ret());
            e.register(MEMORY_FREE, |_, _| 0u32.into_ret());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 4, 0x6000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a5_4410, &[this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (NI_FREE, vec![0x6000]));
            assert_eq!(
                log.contains(&(MEMORY_FREE, vec![MEMORY_MANAGER, this])),
                deleted
            );
            assert_eq!(e.mem.u32(this), ARRAY_VTABLE);
        }
    }
}
