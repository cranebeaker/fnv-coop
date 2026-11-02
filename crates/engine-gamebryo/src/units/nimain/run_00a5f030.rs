//! `NiMain/run_00a5f030` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// The game's `operator new` (`cdecl`, the size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiAVObject::NiAVObject` (engine map, `00a5aa20`; `thiscall`).
const AV_OBJECT_CONSTRUCT: u32 = 0x00a5_aa20;
/// The `NiNode` vtable (RTTI) and the `NiTObjectArray<NiPointer<NiAVObject>>` vtable (RTTI)
/// of the child array at +0x9C.
const NODE_VTABLE: u32 = 0x0109_b5ac;
const CHILD_ARRAY_VTABLE: u32 = 0x0101_fbd4;

// Translated from 00a5f030 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function, no engine-map name; by its body the creation function of
/// `NiNode`, the RTTI of the vtable it stores; no arguments): a new 0xac-byte block from
/// `operator new`; null when that fails. Otherwise `NiAVObject::NiAVObject`, the `NiNode`
/// vtable, and at +0x9C an empty child array (`NiTArray` layout: vtable `0101fbd4`, no
/// elements, capacity, size and element count 0, grow-by 1). Answers the block.
pub fn fn_00a5f030(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0xacu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(AV_OBJECT_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, NODE_VTABLE);
    let children = Ptr::<NiTArray>::new(block + 0x9c);
    e.set(children, NiTArray::m_usMaxSize, 0);
    e.set(children, NiTArray::m_usESize, 0);
    e.set(children, NiTArray::m_pBase, 0);
    e.set(children, NiTArray::m_usGrowBy, 1);
    e.set(children, NiTArray::m_usSize, 0);
    e.mem.set_u32(children.addr(), CHILD_ARRAY_VTABLE);
    block
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a5_f030, fn_00a5f030() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a5f030_builds_a_node_with_an_empty_child_array() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0xac);
        for offset in (0..0xac).step_by(4) {
            e.mem.set_u32(block + offset, 0xdead_beef);
        }
        e.register_double(OPERATOR_NEW, move |_, _| block.into_ret());
        e.register(AV_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_f030, &[]).u32(), block);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(OPERATOR_NEW, vec![0xac])));
        assert!(log.contains(&(AV_OBJECT_CONSTRUCT, vec![block])));
        assert_eq!(e.mem.u32(block), NODE_VTABLE);
        assert_eq!(e.mem.u32(block + 0x9c), CHILD_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(block + 0xa0), 0);
        assert_eq!(e.mem.u32(block + 0xa4), 0);
        assert_eq!(e.mem.u16(block + 0xa8), 0);
        assert_eq!(e.mem.u16(block + 0xaa), 1);

        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_f030, &[]).u32(), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }
}
