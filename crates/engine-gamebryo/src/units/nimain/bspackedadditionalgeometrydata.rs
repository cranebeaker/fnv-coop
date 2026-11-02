//! `NiMain/bspackedadditionalgeometrydata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSPackedAdditionalGeometryData` (Xbox PDB, 0x34 bytes, offsets confirmed by the PC
//! code): `NiAdditionalGeometryData` (0x2c bytes), `m_uiShaderDeclarationIndex` +0x2c and
//! `m_uiVertexStride` +0x30. Its data blocks (`NiBSPackedAGDDataBlock`, 0x10 bytes, the
//! `NiAGDDataBlock` layout: `m_uiDataBlockSize` +4, `m_pucDataBlock` +8, `m_bOwnData` +0xc,
//! `m_bBSHeap` +0xd) keep their buffers through the renderer (the object at `011f4748`).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BSPackedAdditionalGeometryData` (Xbox PDB), 0x34 bytes.
    pub struct BSPackedAdditionalGeometryData: 0x34 {
        /// `m_uiShaderDeclarationIndex` (Xbox PDB).
        0x2C m_uiShaderDeclarationIndex: u32,
        /// `m_uiVertexStride` (Xbox PDB).
        0x30 m_uiVertexStride: u32,
    }
}

/// The renderer object (`011f4748`) whose virtual slots keep the packed blocks' buffers:
/// +0x12c allocates (size), +0x130 and +0x134 take (buffer, size), +0x138 frees (buffer).
const RENDERER: u32 = 0x011f_4748;
const RENDERER_SLOT_ALLOCATE: u32 = 0x12c;
const RENDERER_SLOT_PROTECT: u32 = 0x130;
const RENDERER_SLOT_UNPROTECT: u32 = 0x134;
const RENDERER_SLOT_FREE: u32 = 0x138;
/// `NiAGDDataBlock` fields.
const BLOCK_DATA_SIZE: u32 = 0x04;
const BLOCK_DATA: u32 = 0x08;
const BLOCK_OWN_DATA: u32 = 0x0c;
const BLOCK_BS_HEAP: u32 = 0x0d;
/// The memory manager (`011f6238`) and its `Allocate` (`00aa3e40`, thiscall: size).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// `NiNew`'s allocator (`00aa13e0`, cdecl: size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// Vtables (by their RTTI names): `NiBSPackedAGDDataBlock` and
/// `BSPackedAdditionalGeometryData`.
const PACKED_BLOCK_VTABLE: u32 = 0x0109_ded8;
const PACKED_DATA_VTABLE: u32 = 0x0102_e1dc;
/// `NiAdditionalGeometryData`'s `IsEqual` (`00a727f0`), `SaveBinary` (`00a73370`),
/// `LoadBinary` (`00a73160`), `GetViewerStrings` (`00a73e40`) and constructor (`00a734c0`).
const AGD_IS_EQUAL: u32 = 0x00a7_27f0;
const AGD_SAVE_BINARY: u32 = 0x00a7_3370;
const AGD_LOAD_BINARY: u32 = 0x00a7_3160;
const AGD_GET_VIEWER_STRINGS: u32 = 0x00a7_3e40;
const AGD_CONSTRUCT: u32 = 0x00a7_34c0;
/// `NiStream`'s input (+0x24c, read function at +8) and output (+0x250, write function at
/// +0xc); both cdecl (object, buffer, size, element sizes, element count), the answer added
/// to the object's position at +4.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
/// The string array's `SetSize` (`0096ad30`) and `SetAt` (`0096ae90`).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `NiGetViewerString` (`00a7d330`: text) and `NiGetViewerString_ov8` (`00a7d510`: label,
/// unsigned word); cdecl.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
/// The word at `011f4aac`: the class name the viewer strings start with.
const CLASS_NAME: u32 = 0x011f_4aac;
/// `"m_uiShaderDeclarationIndex"`.
const TEXT_SHADER_DECLARATION_INDEX: u32 = 0x0109_deec;

/// The renderer object.
fn renderer(e: &Engine) -> u32 {
    e.mem.u32(RENDERER)
}

/// The block's own buffer and size for a null `buffer` when the block owns its data,
/// else `buffer` and `size` as given.
fn own_buffer(e: &Engine, block: u32, buffer: u32, size: u32) -> (u32, u32) {
    if buffer == 0 && e.mem.u8(block + BLOCK_OWN_DATA) != 0 {
        (
            e.mem.u32(block + BLOCK_DATA),
            e.mem.u32(block + BLOCK_DATA_SIZE),
        )
    } else {
        (buffer, size)
    }
}

/// One 4-byte element moved through the stream's input or output (`object_field`
/// +0x24c or +0x250, its function at `function`), the element size in the stack word
/// `sizes`.
fn stream_word(
    e: &mut Engine,
    stream: u32,
    object_field: u32,
    function: u32,
    data: u32,
    sizes: u32,
) {
    let object = e.mem.u32(stream.wrapping_add(object_field));
    let call = e.mem.u32(object.wrapping_add(function));
    e.mem.set_u32(sizes, 4);
    let moved = e.call(call, &args![object, data, 4u32, sizes, 1u32]).u32();
    let position = e.mem.u32(object.wrapping_add(4)).wrapping_add(moved);
    e.mem.set_u32(object.wrapping_add(4), position);
}

/// Adds the string word at `slot` at the array's end (`SetSize` when full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let index = u32::from(e.mem.u16(strings + 0xa));
    if index >= u32::from(e.mem.u16(strings + 8)) {
        let grow = u32::from(e.mem.u16(strings + 0xe));
        e.call(
            STRING_ARRAY_SET_SIZE,
            &args![strings, grow.wrapping_add(index)],
        );
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a7f560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::NiBSPackedAGDDataBlock::AllocateBuffer` (Xbox PDB;
/// thiscall, `size` on the stack): a jump to the renderer's virtual slot +0x12c with the
/// same stack word, its answer (the buffer) returned.
pub fn ni_bs_packed_agd_data_block_allocate_buffer(e: &mut Engine, _this: Ptr, size: u32) -> u32 {
    let renderer = renderer(e);
    e.vcall(renderer, RENDERER_SLOT_ALLOCATE, &args![size])
        .u32()
}

// Translated from 00a7f570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::NiBSPackedAGDDataBlock::ProtectBuffer` (Xbox PDB;
/// thiscall, `RET 8`): a null `buffer` means the block's own data and size when it owns
/// them; unless the block's buffer is on the BS heap (`m_bBSHeap`), the renderer's virtual
/// slot +0x130 with (buffer, size).
pub fn ni_bs_packed_agd_data_block_protect_buffer(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
) {
    let block = this.addr();
    let (buffer, size) = own_buffer(e, block, buffer, size);
    if e.mem.u8(block + BLOCK_BS_HEAP) == 0 {
        let renderer = renderer(e);
        e.vcall(renderer, RENDERER_SLOT_PROTECT, &args![buffer, size]);
    }
}

// Translated from 00a7f5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::NiBSPackedAGDDataBlock::UnprotectBuffer` (Xbox PDB;
/// thiscall, `RET 8`): as [`ni_bs_packed_agd_data_block_protect_buffer`], through the
/// renderer's virtual slot +0x134.
pub fn ni_bs_packed_agd_data_block_unprotect_buffer(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
) {
    let block = this.addr();
    let (buffer, size) = own_buffer(e, block, buffer, size);
    if e.mem.u8(block + BLOCK_BS_HEAP) == 0 {
        let renderer = renderer(e);
        e.vcall(renderer, RENDERER_SLOT_UNPROTECT, &args![buffer, size]);
    }
}

// Translated from 00a7f5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::NiBSPackedAGDDataBlock::FreeBuffer` (Xbox PDB;
/// thiscall, `buffer` on the stack): a null `buffer` means the block's own data when it owns
/// it; the buffer is written back into the stack word and the code jumps to the renderer's
/// virtual slot +0x138.
pub fn ni_bs_packed_agd_data_block_free_buffer(e: &mut Engine, this: Ptr, buffer: u32) {
    let block = this.addr();
    let mut buffer = buffer;
    if buffer == 0 && e.mem.u8(block + BLOCK_OWN_DATA) != 0 {
        buffer = e.mem.u32(block + BLOCK_DATA);
    }
    let renderer = renderer(e);
    e.vcall(renderer, RENDERER_SLOT_FREE, &args![buffer]);
}

// Translated from 00a7f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::IsEqual` (Xbox PDB; thiscall, `RET 4`):
/// `NiAdditionalGeometryData::IsEqual` (`00a727f0`), then the same shader declaration
/// index and vertex stride.
pub fn bs_packed_additional_geometry_data_is_equal(
    e: &mut Engine,
    this: Ptr<BSPackedAdditionalGeometryData>,
    other: Ptr<BSPackedAdditionalGeometryData>,
) -> bool {
    if e.call(AGD_IS_EQUAL, &args![this, other]).u8() == 0 {
        return false;
    }
    if e.get(
        this,
        BSPackedAdditionalGeometryData::m_uiShaderDeclarationIndex,
    ) != e.get(
        other,
        BSPackedAdditionalGeometryData::m_uiShaderDeclarationIndex,
    ) {
        return false;
    }
    e.get(this, BSPackedAdditionalGeometryData::m_uiVertexStride)
        == e.get(other, BSPackedAdditionalGeometryData::m_uiVertexStride)
}

// Translated from 00a7f650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::CreateAGDDataBlock` (Xbox PDB): a new 0x10-byte
/// `NiBSPackedAGDDataBlock` from the memory manager (no size, no data, not owning, not on
/// the BS heap, its vtable `0109ded8`), or null.
pub fn bs_packed_additional_geometry_data_create_agd_data_block(e: &mut Engine) -> Ptr {
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
        .u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.mem.set_u32(block + BLOCK_DATA_SIZE, 0);
    e.mem.set_u32(block + BLOCK_DATA, 0);
    e.mem.set_u8(block + BLOCK_OWN_DATA, 0);
    e.mem.set_u32(block, PACKED_BLOCK_VTABLE);
    e.mem.set_u8(block + BLOCK_BS_HEAP, 0);
    Ptr::new(block)
}

// Translated from 00a7f680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::SaveBinary` (Xbox PDB; thiscall, `RET 4`):
/// `NiAdditionalGeometryData::SaveBinary` (`00a73370`), then the shader declaration index
/// and the vertex stride, 4 bytes each (the element size in the argument's stack word).
pub fn bs_packed_additional_geometry_data_save_binary(
    e: &mut Engine,
    this: Ptr<BSPackedAdditionalGeometryData>,
    stream: u32,
) {
    e.call(AGD_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, sizes| {
        for field in [0x2c, 0x30] {
            stream_word(
                e,
                stream,
                STREAM_OUTPUT,
                0xc,
                this.addr() + field,
                sizes.addr(),
            );
        }
    });
}

// Translated from 00a7f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::LoadBinary` (Xbox PDB; thiscall, `RET 4`):
/// `NiAdditionalGeometryData::LoadBinary` (`00a73160`), then the shader declaration index
/// and the vertex stride, 4 bytes each.
pub fn bs_packed_additional_geometry_data_load_binary(
    e: &mut Engine,
    this: Ptr<BSPackedAdditionalGeometryData>,
    stream: u32,
) {
    e.call(AGD_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, sizes| {
        for field in [0x2c, 0x30] {
            stream_word(
                e,
                stream,
                STREAM_INPUT,
                8,
                this.addr() + field,
                sizes.addr(),
            );
        }
    });
}

// Translated from 00a7f740 (PC disassembly, FalloutNV.exe 1.4.0.525; Ghidra made no function here)
/// The class's create function (`BSPackedAdditionalGeometryData::CreateObject` by what it
/// builds; plain `RET`): 0x34 bytes (`00aa13e0`) constructed as `NiAdditionalGeometryData`
/// (`00a734c0`), then this class's vtable (`0102e1dc`) and a zero shader declaration index
/// and vertex stride; null when the allocation fails.
pub fn fn_00a7f740(e: &mut Engine) -> Ptr<BSPackedAdditionalGeometryData> {
    let object = e.call(NI_ALLOCATE, &args![0x34u32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(AGD_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, PACKED_DATA_VTABLE);
    let this = Ptr::<BSPackedAdditionalGeometryData>::new(object);
    e.set(
        this,
        BSPackedAdditionalGeometryData::m_uiShaderDeclarationIndex,
        0,
    );
    e.set(this, BSPackedAdditionalGeometryData::m_uiVertexStride, 0);
    this
}

// Translated from 00a7f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPackedAdditionalGeometryData::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiAdditionalGeometryData::GetViewerStrings` (`00a73e40`), the class name (the word at
/// `011f4aac`) and `m_uiShaderDeclarationIndex`, each held in the argument word and added
/// through `SetSize`/`SetAt`.
pub fn bs_packed_additional_geometry_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<BSPackedAdditionalGeometryData>,
    strings: u32,
) {
    e.call(AGD_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(CLASS_NAME);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let index = e.get(
            this,
            BSPackedAdditionalGeometryData::m_uiShaderDeclarationIndex,
        );
        let text = e
            .call(
                VIEWER_STRING_UNSIGNED,
                &args![TEXT_SHADER_DECLARATION_INDEX, index],
            )
            .u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a7_f560,
            ni_bs_packed_agd_data_block_allocate_buffer(Ptr, u32) -> u32
        ),
        entry!(
            0x00a7_f570,
            ni_bs_packed_agd_data_block_protect_buffer(Ptr, u32, u32)
        ),
        entry!(
            0x00a7_f5b0,
            ni_bs_packed_agd_data_block_unprotect_buffer(Ptr, u32, u32)
        ),
        entry!(
            0x00a7_f5f0,
            ni_bs_packed_agd_data_block_free_buffer(Ptr, u32)
        ),
        entry!(
            0x00a7_f620,
            bs_packed_additional_geometry_data_is_equal(
                Ptr<BSPackedAdditionalGeometryData>,
                Ptr<BSPackedAdditionalGeometryData>,
            ) -> bool
        ),
        entry!(
            0x00a7_f650,
            bs_packed_additional_geometry_data_create_agd_data_block() -> Ptr
        ),
        entry!(
            0x00a7_f680,
            bs_packed_additional_geometry_data_save_binary(
                Ptr<BSPackedAdditionalGeometryData>,
                u32,
            )
        ),
        entry!(
            0x00a7_f6e0,
            bs_packed_additional_geometry_data_load_binary(
                Ptr<BSPackedAdditionalGeometryData>,
                u32,
            )
        ),
        entry!(0x00a7_f740, fn_00a7f740() -> Ptr<BSPackedAdditionalGeometryData>),
        entry!(
            0x00a7_f780,
            bs_packed_additional_geometry_data_get_viewer_strings(
                Ptr<BSPackedAdditionalGeometryData>,
                u32,
            )
        ),
    ]
}

#[cfg(test)]
mod n014_tests {
    use super::*;

    const SLOT_TARGET: u32 = 0x0360_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A renderer whose slot `slot` is a double answering 0x5000.
    fn engine_with_renderer(slot: u32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(SLOT_TARGET, |_, _| 0x5000u32.into_ret());
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + slot, SLOT_TARGET);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, vtable);
        e.mem.map(RENDERER, 4);
        e.mem.set_u32(RENDERER, renderer);
        e.call_log = Some(vec![]);
        (e, renderer)
    }

    fn block(e: &mut Engine, own: u8, bs_heap: u8) -> u32 {
        let block = e.mem.alloc(0x10);
        e.mem.set_u32(block + BLOCK_DATA_SIZE, 0x40);
        e.mem.set_u32(block + BLOCK_DATA, 0x7000);
        e.mem.set_u8(block + BLOCK_OWN_DATA, own);
        e.mem.set_u8(block + BLOCK_BS_HEAP, bs_heap);
        block
    }

    #[test]
    fn allocate_buffer_asks_the_renderer() {
        let (mut e, renderer) = engine_with_renderer(RENDERER_SLOT_ALLOCATE);
        let this = block(&mut e, 0, 0);
        assert_eq!(e.call(0x00a7_f560, &args![this, 0x80u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, SLOT_TARGET), vec![vec![renderer, 0x80]]);
    }

    #[test]
    fn protect_buffer_uses_the_own_data_unless_on_the_bs_heap() {
        let (mut e, renderer) = engine_with_renderer(RENDERER_SLOT_PROTECT);
        let owning = block(&mut e, 1, 0);
        e.call(0x00a7_f570, &args![owning, 0u32, 9u32]);
        e.call(0x00a7_f570, &args![owning, 0x6000u32, 9u32]);
        let foreign = block(&mut e, 0, 0);
        e.call(0x00a7_f570, &args![foreign, 0u32, 9u32]);
        let heap = block(&mut e, 1, 1);
        e.call(0x00a7_f570, &args![heap, 0u32, 9u32]);
        assert_eq!(
            calls_to(&e, SLOT_TARGET),
            vec![
                vec![renderer, 0x7000, 0x40],
                vec![renderer, 0x6000, 9],
                vec![renderer, 0, 9]
            ]
        );
    }

    #[test]
    fn unprotect_buffer_uses_the_own_data_unless_on_the_bs_heap() {
        let (mut e, renderer) = engine_with_renderer(RENDERER_SLOT_UNPROTECT);
        let owning = block(&mut e, 1, 0);
        e.call(0x00a7_f5b0, &args![owning, 0u32, 9u32]);
        e.call(0x00a7_f5b0, &args![owning, 0x6000u32, 9u32]);
        let heap = block(&mut e, 1, 1);
        e.call(0x00a7_f5b0, &args![heap, 0u32, 9u32]);
        assert_eq!(
            calls_to(&e, SLOT_TARGET),
            vec![vec![renderer, 0x7000, 0x40], vec![renderer, 0x6000, 9]]
        );
    }

    #[test]
    fn free_buffer_frees_the_given_or_the_own_buffer() {
        let (mut e, renderer) = engine_with_renderer(RENDERER_SLOT_FREE);
        let owning = block(&mut e, 1, 1);
        e.call(0x00a7_f5f0, &args![owning, 0u32]);
        e.call(0x00a7_f5f0, &args![owning, 0x6000u32]);
        let foreign = block(&mut e, 0, 0);
        e.call(0x00a7_f5f0, &args![foreign, 0u32]);
        assert_eq!(
            calls_to(&e, SLOT_TARGET),
            vec![
                vec![renderer, 0x7000],
                vec![renderer, 0x6000],
                vec![renderer, 0]
            ]
        );
    }

    #[test]
    fn is_equal_also_compares_the_declaration_and_the_stride() {
        const BASE_ANSWER: u32 = 0x0361_0000;
        let mut e = Engine::new();
        e.mem.map(BASE_ANSWER, 4);
        e.mem.set_u32(BASE_ANSWER, 1);
        e.register(AGD_IS_EQUAL, |e, _| e.mem.u32(BASE_ANSWER).into_ret());
        let a = e.mem.alloc(0x34);
        let b = e.mem.alloc(0x34);
        for this in [a, b] {
            e.mem.set_u32(this + 0x2c, 3);
            e.mem.set_u32(this + 0x30, 0x20);
        }
        assert!(e.call(0x00a7_f620, &args![a, b]).bool());
        e.mem.set_u32(b + 0x30, 0x24);
        assert!(!e.call(0x00a7_f620, &args![a, b]).bool());
        e.mem.set_u32(b + 0x30, 0x20);
        e.mem.set_u32(b + 0x2c, 4);
        assert!(!e.call(0x00a7_f620, &args![a, b]).bool());
        e.mem.set_u32(b + 0x2c, 3);
        e.mem.set_u32(BASE_ANSWER, 0x100);
        assert!(!e.call(0x00a7_f620, &args![a, b]).bool());
    }

    #[test]
    fn create_agd_data_block_makes_an_empty_packed_block() {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[1]);
            for i in 0..4 {
                e.mem.set_u32(block + 4 * i, 0xffff_ffff);
            }
            block.into_ret()
        });
        let block = e.call(0x00a7_f650, &args![]).u32();
        assert_eq!(e.mem.u32(block), PACKED_BLOCK_VTABLE);
        assert_eq!((e.mem.u32(block + 4), e.mem.u32(block + 8)), (0, 0));
        assert_eq!(e.mem.u32(block + 0xc), 0xffff_0000);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a7_f650, &args![]).u32(), 0);
    }

    #[test]
    fn save_binary_writes_the_declaration_and_the_stride() {
        const WRITE: u32 = 0x0362_0000;
        let mut e = Engine::new();
        e.register(AGD_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
            a[2].into_ret()
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00a7_f680, &args![this, stream]);
        assert_eq!(calls_to(&e, AGD_SAVE_BINARY), vec![vec![this, stream]]);
        let writes: Vec<(u32, u32)> = calls_to(&e, WRITE).iter().map(|c| (c[1], c[2])).collect();
        assert_eq!(writes, vec![(this + 0x2c, 4), (this + 0x30, 4)]);
        assert_eq!(e.mem.u32(output + 4), 8);
    }

    #[test]
    fn load_binary_reads_the_declaration_and_the_stride() {
        const READ: u32 = 0x0363_0000;
        let mut e = Engine::new();
        e.register(AGD_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
            e.mem.set_u32(a[1], a[1] & 0xff);
            a[2].into_ret()
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        let this = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        e.call(0x00a7_f6e0, &args![this, stream]);
        assert_eq!(calls_to(&e, AGD_LOAD_BINARY), vec![vec![this, stream]]);
        assert_eq!(e.mem.u32(this + 0x2c), (this + 0x2c) & 0xff);
        assert_eq!(e.mem.u32(this + 0x30), (this + 0x30) & 0xff);
        assert_eq!(e.mem.u32(input + 4), 8);
    }

    #[test]
    fn create_object_constructs_the_base_and_clears_the_fields() {
        let mut e = Engine::new();
        e.register(AGD_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| {
            let object = e.mem.alloc(a[0]);
            e.mem.set_u32(object + 0x2c, 7);
            e.mem.set_u32(object + 0x30, 7);
            object.into_ret()
        });
        e.call_log = Some(vec![]);
        let object = e.call(0x00a7_f740, &args![]).u32();
        assert_eq!(calls_to(&e, NI_ALLOCATE), vec![vec![0x34]]);
        assert_eq!(calls_to(&e, AGD_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), PACKED_DATA_VTABLE);
        assert_eq!((e.mem.u32(object + 0x2c), e.mem.u32(object + 0x30)), (0, 0));
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a7_f740, &args![]).u32(), 0);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_the_declaration_index() {
        let mut e = Engine::new();
        e.mem.map(CLASS_NAME, 4);
        e.mem.set_u32(CLASS_NAME, 0x0109_0000);
        e.register(AGD_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_UNSIGNED, |_, a| (a[0] + a[1]).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 4);
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 0x2c, 5);
        e.call_log = Some(vec![]);
        e.call(0x00a7_f780, &args![this, strings]);
        assert_eq!(
            calls_to(&e, AGD_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        let base = e.mem.u32(strings + 4);
        assert_eq!(e.mem.u16(strings + 0xa), 2);
        assert_eq!(
            (e.mem.u32(base), e.mem.u32(base + 4)),
            (0x0109_0001, TEXT_SHADER_DECLARATION_INDEX + 5)
        );
    }
}
