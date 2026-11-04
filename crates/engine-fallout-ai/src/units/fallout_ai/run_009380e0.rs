//! `fallout/ai/run_009380e0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0043b370` (unnamed in the engine map): `thiscall`, `RET 8` (a flag byte, a mask); sets or
/// clears the mask in the dword at `this + 0x30`.
const SET_OR_CLEAR_FLAG_MASK: u32 = 0x0043_b370;
/// `0043b480` (unnamed in the engine map): `thiscall`; the number of children of a container.
const CONTAINER_COUNT: u32 = 0x0043_b480;
/// `0043b4a0` (unnamed in the engine map): `thiscall`, `RET 4`; the child at an index.
const CONTAINER_ITEM: u32 = 0x0043_b4a0;
/// The virtual slot (byte offset) that answers the object's container, or null.
const SLOT_CONTAINER: u32 = 0x0c;
/// The mask `00938160` sets or clears.
const FLAG_MASK_8000000: u32 = 0x0800_0000;

// Translated from 00938160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout/ai`). `thiscall`, `RET 4`: sets (non-zero byte) or clears
/// the mask `0x8000000` in the flags dword at `this + 0x30` through `0043b370`.
pub fn fn_00938160(e: &mut Engine, this: Ptr, set: u8) {
    e.call(SET_OR_CLEAR_FLAG_MASK, &args![this, set, FLAG_MASK_8000000]);
}

// Translated from 009380e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `cdecl`, one stack word: for a non-null object, sets the mask
/// `0x8000000` on it (`00938160` with 1) and then does the same, recursively, to every non-null
/// child of the container its virtual slot +0xc answers (the count is read again on every
/// iteration, and the slot is called a second time to get the container, as the exe does).
pub fn fn_009380e0(e: &mut Engine, object: Ptr) {
    if object.is_null() {
        return;
    }
    fn_00938160(e, object, 1);
    if e.vcall(object.addr(), SLOT_CONTAINER, &args![]).u32() == 0 {
        return;
    }
    let container = e.vcall(object.addr(), SLOT_CONTAINER, &args![]).u32();
    let mut index = 0u32;
    while index < e.call(CONTAINER_COUNT, &args![container]).u32() {
        let child = e.call(CONTAINER_ITEM, &args![container, index]).u32();
        if child != 0 {
            fn_009380e0(e, Ptr::new(child));
        }
        index = index.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0093_8160, fn_00938160(Ptr, u8)),
        entry!(0x0093_80e0, fn_009380e0(Ptr)),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_00938160_passes_the_flag_and_the_mask() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(SET_OR_CLEAR_FLAG_MASK, |_, _| Ret::default());
        e.call(0x0093_8160, &args![0x4000u32, 1u8]);
        e.call(0x0093_8160, &args![0x4000u32, 0u8]);
        assert_eq!(
            calls(&e, SET_OR_CLEAR_FLAG_MASK),
            vec![vec![0x4000, 1, 0x0800_0000], vec![0x4000, 0, 0x0800_0000]]
        );
    }

    /// Objects whose vtable slot +0xc answers the container stored at +8; a container keeps
    /// its children as `count` at +0 and the child pointers from +4.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut slots = [0u32; 4];
        slots[SLOT_CONTAINER as usize / 4] = 0x0f00_9000;
        e.put_vtable(0x0f00_9100, &slots);
        e.register(0x0f00_9000, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(SET_OR_CLEAR_FLAG_MASK, |_, _| Ret::default());
        e.register(CONTAINER_COUNT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CONTAINER_ITEM, |e, a| {
            e.mem.u32(a[0] + 4 + 4 * a[1]).into_ret()
        });
        e
    }

    fn object(e: &mut Engine, container: u32) -> u32 {
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0f00_9100);
        e.mem.set_u32(object + 8, container);
        object
    }

    fn container(e: &mut Engine, children: &[u32]) -> u32 {
        let container = e.mem.alloc(0x40);
        e.mem.set_u32(container, children.len() as u32);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(container + 4 + 4 * index as u32, *child);
        }
        container
    }

    #[test]
    fn at_009380e0_ignores_a_null_object() {
        let mut e = rig();
        e.call(0x0093_80e0, &args![0u32]);
        assert!(calls(&e, SET_OR_CLEAR_FLAG_MASK).is_empty());
    }

    #[test]
    fn at_009380e0_marks_the_object_and_walks_the_children_recursively() {
        let mut e = rig();
        let grandchild = object(&mut e, 0);
        let inner = container(&mut e, &[grandchild]);
        let child_with_children = object(&mut e, inner);
        let childless = object(&mut e, 0);
        let outer = container(&mut e, &[child_with_children, 0, childless]);
        let root = object(&mut e, outer);
        e.call(0x0093_80e0, &args![root]);
        let marked: Vec<u32> = calls(&e, SET_OR_CLEAR_FLAG_MASK)
            .iter()
            .map(|c| c[0])
            .collect();
        // Depth first; the null child is skipped.
        assert_eq!(
            marked,
            vec![root, child_with_children, grandchild, childless]
        );
        assert!(calls(&e, SET_OR_CLEAR_FLAG_MASK)
            .iter()
            .all(|c| c[1] == 1 && c[2] == 0x0800_0000));
        // The container slot is asked twice per object with a container.
        assert_eq!(calls(&e, 0x0f00_9000).len(), 4 + 2);
    }
}
