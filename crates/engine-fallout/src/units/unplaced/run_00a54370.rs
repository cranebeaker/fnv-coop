//! `(unplaced)/run_00a54370` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTPointerMap<NiNode*, unsigned int>`'s vtable (its RTTI names the
/// class).
const POINTER_MAP_VTABLE: u32 = 0x0109_a284;
/// Its base `NiTMapBase<NiTPointerAllocator<unsigned int>, NiNode*,
/// unsigned int>`'s vtable.
const MAP_BASE_VTABLE: u32 = 0x0109_a264;
/// The map's `RemoveAll` (`00b601a0`, thiscall).
const MAP_REMOVE_ALL: u32 = 0x00b6_01a0;
/// Frees a block through the Ni allocator (`00aa10f0`, cdecl; null is
/// ignored).
const NI_FREE: u32 = 0x00aa_10f0;
/// The memory manager object and its free method (thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_FREE: u32 = 0x00aa_4060;

// Translated from 00a54370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `NiTPointerMap<NiNode*, unsigned int>`
/// (slot 0 of its vtable `0109a284`, whose RTTI names the class): its own
/// vtable and `RemoveAll` (`00b601a0`), then the base class's vtable and
/// `RemoveAll` again, frees the bucket array (+8) through `00aa10f0`, and
/// with bit 0 of `flags` frees the object through the memory manager.
/// Returns `this`.
pub fn fn_00a54370(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.mem.set_u32(this, POINTER_MAP_VTABLE);
    e.call(MAP_REMOVE_ALL, &[this]);
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
    vec![entry!(0x00a5_4370, fn_00a54370(u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pointer_map_destructor_runs_both_levels() {
        for (flags, deleted) in [(1u32, true), (0, false)] {
            let mut e = Engine::new();
            let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
            let record = seen.clone();
            e.register_double(MAP_REMOVE_ALL, move |e, a| {
                record.borrow_mut().push(e.mem.u32(a[0]));
                0u32.into_ret()
            });
            e.register(NI_FREE, |_, _| 0u32.into_ret());
            e.register(MEMORY_FREE, |_, _| 0u32.into_ret());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 8, 0x5000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a5_4370, &[this, flags]).u32(), this);
            assert_eq!(*seen.borrow(), vec![POINTER_MAP_VTABLE, MAP_BASE_VTABLE]);
            let log = e.call_log.take().unwrap();
            assert!(log.contains(&(NI_FREE, vec![0x5000])));
            assert_eq!(
                log.contains(&(MEMORY_FREE, vec![MEMORY_MANAGER, this])),
                deleted
            );
        }
    }
}
