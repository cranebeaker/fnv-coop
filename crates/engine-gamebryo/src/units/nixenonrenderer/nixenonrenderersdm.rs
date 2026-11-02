//! `NiXenonRenderer/nixenonrenderersdm.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions the compiler emitted into this unit: a pointer list's
//! `AddNodeTail` and the shader-program creators' small virtuals.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiTListItem, NiTPointerList};

layout! {
    /// `NiD3DShaderProgramCreator` (Xbox PDB), 8 bytes: a vtable and the
    /// creation flags (the HLSL creator adds nothing).
    pub struct NiD3DShaderProgramCreator: 0x08 {
        /// `m_uiShaderCreationFlags` (Xbox PDB).
        0x04 m_uiShaderCreationFlags: u32,
    }

    /// `NiD3DShaderProgramCreatorObj` (Xbox PDB), 0x10 bytes.
    pub struct NiD3DShaderProgramCreatorObj: 0x10 {
        /// `m_uiShaderCreationFlags` (Xbox PDB).
        0x04 m_uiShaderCreationFlags: u32,
        /// `m_pvConstantBuffer` (Xbox PDB, `NiD3DShaderProgramCreatorAsm`).
        0x08 m_pvConstantBuffer: u32,
        /// `m_uiConstantBufferSize` (Xbox PDB).
        0x0C m_uiConstantBufferSize: u32,
    }
}

/// `NiD3DShaderProgramCreator`'s and `NiD3DShaderProgramCreatorAsm`'s
/// vtables (`010f0a68`, `010f0ac0`; RTTI `.?AVNiD3DShaderProgramCreator@@`,
/// `.?AVNiD3DShaderProgramCreatorAsm@@`).
const CREATOR_VTABLE: u32 = 0x010f_0a68;
const CREATOR_ASM_VTABLE: u32 = 0x010f_0ac0;
/// Gamebryo's object release (`00aa1460`, cdecl: object, size) and
/// `NiFree` (`00aa10f0`, cdecl).
const NI_DELETE: u32 = 0x00aa_1460;
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00a11db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTListBase<NiTPointerAllocator<unsigned int>, unsigned int>::AddNodeTail`
/// (Xbox PDB): links `node` after the tail (or as the head of an empty
/// list), makes it the tail and counts it.
pub fn ni_t_list_base_add_node_tail(
    e: &mut Engine,
    this: Ptr<NiTPointerList>,
    node: Ptr<NiTListItem>,
) {
    e.set(node, NiTListItem::m_pkNext, 0);
    let tail = e.get(this, NiTPointerList::m_pkTail);
    e.set(node, NiTListItem::m_pkPrev, tail);
    if e.get(this, NiTPointerList::m_pkTail) != 0 {
        let tail = Ptr::<NiTListItem>::new(e.get(this, NiTPointerList::m_pkTail));
        e.set(tail, NiTListItem::m_pkNext, node.addr());
    } else {
        e.set(this, NiTPointerList::m_pkHead, node.addr());
    }
    e.set(this, NiTPointerList::m_pkTail, node.addr());
    let count = e.get(this, NiTPointerList::m_uiCount);
    e.set(this, NiTPointerList::m_uiCount, count.wrapping_add(1));
}

// Translated from 00aa1d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL::GetShaderCreationFlags` (Xbox PDB): the
/// creation flags.
pub fn ni_d3d_shader_program_creator_hlsl_get_shader_creation_flags(
    e: &mut Engine,
    this: Ptr<NiD3DShaderProgramCreator>,
) -> u32 {
    e.get(this, NiD3DShaderProgramCreator::m_uiShaderCreationFlags)
}

// Translated from 00e91c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorHLSL`'s scalar deleting destructor (Xbox PDB):
/// the base vtable, and the 8 bytes freed when `flags & 1`. Returns `this`.
pub fn ni_d3d_shader_program_creator_hlsl_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiD3DShaderProgramCreator>,
    flags: u32,
) -> Ptr<NiD3DShaderProgramCreator> {
    e.mem.set_u32(this.addr(), CREATOR_VTABLE);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 8u32]);
    }
    this
}

// Translated from 00e91c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreatorObj`'s scalar deleting destructor (Xbox PDB):
/// as an `NiD3DShaderProgramCreatorAsm`, frees its constant buffer
/// (`NiFree`), then takes the base vtable and frees the 0x10 bytes when
/// `flags & 1`. Returns `this`.
pub fn ni_d3d_shader_program_creator_obj_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiD3DShaderProgramCreatorObj>,
    flags: u32,
) -> Ptr<NiD3DShaderProgramCreatorObj> {
    let buffer = e.get(this, NiD3DShaderProgramCreatorObj::m_pvConstantBuffer);
    e.mem.set_u32(this.addr(), CREATOR_ASM_VTABLE);
    e.call(NI_FREE, &args![buffer]);
    e.mem.set_u32(this.addr(), CREATOR_VTABLE);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00e91be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiD3DShaderProgramCreator`'s scalar deleting destructor (engine map name; vtable
/// `010f0a68` +0): stores the class's vtable and, when `flags & 1`, gives 4 bytes back (the
/// size the exe passes). Returns `this`.
pub fn ni_d3d_shader_program_creator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiD3DShaderProgramCreator>,
    flags: u32,
) -> Ptr<NiD3DShaderProgramCreator> {
    e.mem.set_u32(this.addr(), CREATOR_VTABLE);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 4u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e9_1be0,
            ni_d3d_shader_program_creator_scalar_deleting_destructor(
                Ptr<NiD3DShaderProgramCreator>,
                u32,
            ) -> Ptr<
                NiD3DShaderProgramCreator,
            >
        ),
        entry!(
            0x00a1_1db0,
            ni_t_list_base_add_node_tail(Ptr<NiTPointerList>, Ptr<NiTListItem>)
        ),
        entry!(
            0x00aa_1d50,
            ni_d3d_shader_program_creator_hlsl_get_shader_creation_flags(
                Ptr<NiD3DShaderProgramCreator>,
            ) -> u32
        ),
        entry!(
            0x00e9_1c10,
            ni_d3d_shader_program_creator_hlsl_scalar_deleting_destructor(
                Ptr<NiD3DShaderProgramCreator>,
                u32,
            ) -> Ptr<
                NiD3DShaderProgramCreator,
            >
        ),
        entry!(
            0x00e9_1c40,
            ni_d3d_shader_program_creator_obj_scalar_deleting_destructor(
                Ptr<NiD3DShaderProgramCreatorObj>,
                u32,
            ) -> Ptr<
                NiD3DShaderProgramCreatorObj,
            >
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn ni_t_list_base_add_node_tail_links_the_nodes() {
        let mut e = Engine::new();
        let list = e.new_object::<NiTPointerList>();
        let first = e.new_object::<NiTListItem>();
        let second = e.new_object::<NiTListItem>();
        e.set(first, NiTListItem::m_pkNext, 0xdead);
        e.call(0x00a1_1db0, &args![list, first]);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), first.addr());
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), first.addr());
        assert_eq!(e.get(first, NiTListItem::m_pkNext), 0);
        assert_eq!(e.get(first, NiTListItem::m_pkPrev), 0);
        e.call(0x00a1_1db0, &args![list, second]);
        assert_eq!(e.get(list, NiTPointerList::m_pkHead), first.addr());
        assert_eq!(e.get(list, NiTPointerList::m_pkTail), second.addr());
        assert_eq!(e.get(first, NiTListItem::m_pkNext), second.addr());
        assert_eq!(e.get(second, NiTListItem::m_pkPrev), first.addr());
        assert_eq!(e.get(list, NiTPointerList::m_uiCount), 2);
    }

    #[test]
    fn ni_d3d_shader_program_creator_hlsl_get_shader_creation_flags_reads_the_flags() {
        let mut e = Engine::new();
        let this = e.new_object::<NiD3DShaderProgramCreator>();
        e.set(
            this,
            NiD3DShaderProgramCreator::m_uiShaderCreationFlags,
            0x41,
        );
        assert_eq!(e.call(0x00aa_1d50, &args![this]).u32(), 0x41);
    }

    #[test]
    fn ni_d3d_shader_program_creator_hlsl_scalar_deleting_destructor_frees_with_bit_zero() {
        let mut e = Engine::new();
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.new_object::<NiD3DShaderProgramCreator>();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_1c10, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), CREATOR_VTABLE);
        assert!(calls_to(&e, NI_DELETE).is_empty());
        e.call(0x00e9_1c10, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 8]]);
    }

    #[test]
    fn ni_d3d_shader_program_creator_obj_scalar_deleting_destructor_frees_the_buffer() {
        let mut e = Engine::new();
        e.register(NI_DELETE, |_, _| Ret::default());
        e.register(NI_FREE, |_, _| Ret::default());
        let this = e.new_object::<NiD3DShaderProgramCreatorObj>();
        e.set(
            this,
            NiD3DShaderProgramCreatorObj::m_pvConstantBuffer,
            0x6000,
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_1c40, &args![this, 3u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![0x6000]]);
        assert_eq!(e.mem.u32(this.addr()), CREATOR_VTABLE);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x10]]);
        e.call_log = Some(vec![]);
        e.call(0x00e9_1c40, &args![this, 0u32]);
        assert!(calls_to(&e, NI_DELETE).is_empty());
    }

    #[test]
    fn ni_d3d_shader_program_creator_scalar_deleting_destructor_frees_four_bytes() {
        let mut e = Engine::new();
        e.register(NI_DELETE, |_, _| Ret::default());
        let this = e.new_object::<NiD3DShaderProgramCreator>();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e9_1be0, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), CREATOR_VTABLE);
        assert!(calls_to(&e, NI_DELETE).is_empty());
        e.call(0x00e9_1be0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 4]]);
    }
}
