//! `(unplaced)/run_00a9aea0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00aa13e0` (`cdecl`, one word: the size): the Gamebryo allocation.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiNode` constructor (`00a5ecb0`, `thiscall`, one word: the initial child count).
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// `NiPlane::NiPlane` (engine map, `00a69940`; `thiscall`, no stack words).
const PLANE_CONSTRUCT: u32 = 0x00a6_9940;
/// `0060bef0` (`thiscall`, `RET 4`): resizes an `NiPointer` array to the given size.
const POINTER_ARRAY_SET_SIZE: u32 = 0x0060_bef0;
/// `NiBSPNode`'s vtable (RTTI `.?AVNiBSPNode@@` at the locator `01127770`).
const BSP_NODE_VTABLE: u32 = 0x010a_1024;
/// The object's size; the node's child array (+0x9C), the 16-bit field cleared at +0xAA and
/// the two planes at +0xAC and +0xBC.
const OBJECT_SIZE: u32 = 0xcc;
const CHILDREN: u32 = 0x9c;
const SHORT_AA: u32 = 0xaa;
const PLANE_AC: u32 = 0xac;
const PLANE_BC: u32 = 0xbc;
/// The size the child array is set to.
const CHILD_COUNT: u32 = 2;

// Translated from 00a9aea0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no arguments; its address is pushed at `00a89df1`,
/// after the `NiBSPNode` unit): the `NiBSPNode` factory. Allocates 0xCC bytes (`00aa13e0`);
/// on success it runs the `NiNode` constructor with 0 (`00a5ecb0`), stores the `NiBSPNode`
/// vtable, constructs the planes at +0xAC and +0xBC (`00a69940`), sets the child array at
/// +0x9C to two entries (`0060bef0`) and clears the word at +0xAA. Returns the object, or 0
/// when the allocation fails.
pub fn fn_00a9aea0(e: &mut Engine) -> u32 {
    let object = e.call(NI_ALLOCATE, &args![OBJECT_SIZE]).u32();
    if object == 0 {
        return 0;
    }
    e.call(NODE_CONSTRUCT, &args![object, 0u32]);
    e.mem.set_u32(object, BSP_NODE_VTABLE);
    e.call(PLANE_CONSTRUCT, &args![object + PLANE_AC]);
    e.call(PLANE_CONSTRUCT, &args![object + PLANE_BC]);
    e.call(
        POINTER_ARRAY_SET_SIZE,
        &args![object + CHILDREN, CHILD_COUNT],
    );
    e.mem.set_u16(object + SHORT_AA, 0);
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_aea0, fn_00a9aea0() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rig(succeeds: bool) -> Engine {
        let mut e = Engine::new();
        e.register_double(NI_ALLOCATE, move |e, a| {
            assert_eq!(a[0], OBJECT_SIZE);
            let block = if succeeds { e.mem.alloc(a[0]) } else { 0 };
            block.into_ret()
        });
        e.register(NODE_CONSTRUCT, |e, a| {
            for offset in (0..OBJECT_SIZE).step_by(4) {
                e.mem.set_u32(a[0] + offset, 0xdddd_dddd);
            }
            Ret::default()
        });
        e.register(PLANE_CONSTRUCT, |_, _| Ret::default());
        e.register(POINTER_ARRAY_SET_SIZE, |_, _| Ret::default());
        e
    }

    #[test]
    fn the_factory_builds_a_bsp_node() {
        let mut e = rig(true);
        e.call_log = Some(vec![]);
        let object = e.call(0x00a9_aea0, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), BSP_NODE_VTABLE);
        assert_eq!(e.mem.u16(object + 0xaa), 0);
        assert_eq!(e.mem.u16(object + 0xa8), 0xdddd);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .into_iter()
            .filter(|(a, _)| [NODE_CONSTRUCT, PLANE_CONSTRUCT, POINTER_ARRAY_SET_SIZE].contains(a))
            .collect();
        assert_eq!(
            calls,
            vec![
                (NODE_CONSTRUCT, vec![object, 0]),
                (PLANE_CONSTRUCT, vec![object + 0xac]),
                (PLANE_CONSTRUCT, vec![object + 0xbc]),
                (POINTER_ARRAY_SET_SIZE, vec![object + 0x9c, 2]),
            ]
        );
    }

    #[test]
    fn a_failed_allocation_answers_null() {
        let mut e = rig(false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_aea0, &args![]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != NODE_CONSTRUCT));
    }
}
