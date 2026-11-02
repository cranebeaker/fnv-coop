//! `NiXenonRenderer/nigeometrybufferdata.cpp` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiGeometryBufferData` is the renderer data of one geometry: its vertex-buffer chips (one
//! per stream, made and released by the geometry group at +0x08), their strides, its index
//! buffer and its vertex declaration. The index buffer and the declaration are Direct3D
//! objects: `IUnknown::AddRef` (slot +0x04) and `Release` (slot +0x08) take the object as
//! their one pushed word.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiGeometryBufferData` (Xbox PDB), 0x50 bytes; the offsets used here are the PC
    /// code's own and match the PDB.
    pub struct NiGeometryBufferData: 0x50 {
        /// `m_pkGeometryGroup` (Xbox PDB): `NiGeometryGroup*`.
        0x08 m_pkGeometryGroup: u32,
        /// `m_uiFVF` (Xbox PDB).
        0x0C m_uiFVF: u32,
        /// `m_hDeclaration` (Xbox PDB): the vertex declaration (a Direct3D object).
        0x10 m_hDeclaration: u32,
        /// `m_bSoftwareVP` (Xbox PDB).
        0x14 m_bSoftwareVP: u8,
        /// `m_uiMaxVertCount` (Xbox PDB).
        0x1C m_uiMaxVertCount: u32,
        /// `m_uiStreamCount` (Xbox PDB).
        0x20 m_uiStreamCount: u32,
        /// `m_puiVertexStride` (Xbox PDB): one stride per stream.
        0x24 m_puiVertexStride: u32,
        /// `m_ppkVBChip` (Xbox PDB): one `NiVBChip*` per stream.
        0x28 m_ppkVBChip: u32,
        /// `m_uiIBSize` (Xbox PDB).
        0x30 m_uiIBSize: u32,
        /// `m_pkIB` (Xbox PDB): the index buffer (a Direct3D object).
        0x34 m_pkIB: u32,
    }
}

/// `NiVBChip::m_pkVB` (Xbox PDB) +0x08: a chip's vertex buffer.
const CHIP_VB: u32 = 0x08;

/// `IUnknown::AddRef` (slot 1) and `Release` (slot 2).
const ADD_REF: u32 = 0x4;
const RELEASE: u32 = 0x8;
/// `NiGeometryGroup::ReleaseChip` (Xbox PDB vtable slot +0x1C, `thiscall`: the data and the
/// stream).
const GROUP_RELEASE_CHIP: u32 = 0x1c;

/// `00aa1070` (`cdecl`, the size): allocates a block.
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
/// `00aa10f0` (`cdecl`, the block): frees a block.
const FREE_00AA10F0: u32 = 0x00aa_10f0;
/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, the block).
const MEMORY_MANAGER_FREE: u32 = 0x00aa_4060;
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `memcpy` (`00ec44d0`) and `memset` (`00ec61c0`), both `cdecl`.
const MEMCPY: u32 = 0x00ec_44d0;
const MEMSET: u32 = 0x00ec_61c0;

/// The vtables `00e8f0f0` stores: `NiGeometryBufferData`'s, then its base
/// (`NiGeometryData::RendererData`)'s.
const BUFFER_DATA_VTABLE: u32 = 0x010f_017c;
const RENDERER_DATA_VTABLE: u32 = 0x010f_0170;

// Translated from 00e8eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the data has at least one stream and every stream's
/// chip exists and has a vertex buffer (`NiVBChip::m_pkVB`, +0x08).
pub fn fn_00e8eeb0(e: &mut Engine, this: Ptr<NiGeometryBufferData>) -> bool {
    let count = e.get(this, NiGeometryBufferData::m_uiStreamCount);
    if count == 0 {
        return false;
    }
    let mut slot = e.get(this, NiGeometryBufferData::m_ppkVBChip);
    for _ in 0..count {
        let chip = e.mem.u32(slot);
        if chip == 0 || e.mem.u32(chip.wrapping_add(CHIP_VB)) == 0 {
            return false;
        }
        slot = slot.wrapping_add(4);
    }
    true
}

// Translated from 00e8eee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryBufferData::RemoveIB` (Xbox PDB): releases the index buffer (when there is one)
/// and clears it and its size.
pub fn ni_geometry_buffer_data_remove_ib(e: &mut Engine, this: Ptr<NiGeometryBufferData>) {
    let ib = e.get(this, NiGeometryBufferData::m_pkIB);
    if ib != 0 {
        e.vcall(ib, RELEASE, &[]);
        e.set(this, NiGeometryBufferData::m_pkIB, 0);
    }
    e.set(this, NiGeometryBufferData::m_uiIBSize, 0);
}

// Translated from 00e8ef10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the stream count. Nothing happens when it is unchanged.
/// Otherwise two new arrays of `count` words are allocated for the chips and the strides.
/// When the count shrinks, the first `count` words of each old array are copied and, when
/// the data has a geometry group, its `ReleaseChip` (slot +0x1C) releases every stream from
/// `count` up to the old count (still using the old arrays). When it grows, the old arrays
/// are copied whole and the new tail of each is zeroed. The old chip array goes back through
/// the memory manager's free (`00aa4060`), the old stride array through `00aa10f0`, and the
/// new arrays and count are stored.
pub fn fn_00e8ef10(e: &mut Engine, this: Ptr<NiGeometryBufferData>, count: u32) {
    if count == e.get(this, NiGeometryBufferData::m_uiStreamCount) {
        return;
    }
    let bytes = count.wrapping_mul(4);
    let chips = e.call(ALLOCATE_00AA1070, &args![bytes]).u32();
    let strides = e.call(ALLOCATE_00AA1070, &args![bytes]).u32();
    let old_count = e.get(this, NiGeometryBufferData::m_uiStreamCount);
    if count < old_count {
        let old_chips = e.get(this, NiGeometryBufferData::m_ppkVBChip);
        e.call(MEMCPY, &args![chips, old_chips, bytes]);
        let old_strides = e.get(this, NiGeometryBufferData::m_puiVertexStride);
        e.call(MEMCPY, &args![strides, old_strides, bytes]);
        if e.get(this, NiGeometryBufferData::m_pkGeometryGroup) != 0 {
            let mut stream = count;
            while stream < e.get(this, NiGeometryBufferData::m_uiStreamCount) {
                let group = e.get(this, NiGeometryBufferData::m_pkGeometryGroup);
                e.vcall(group, GROUP_RELEASE_CHIP, &args![this, stream]);
                stream = stream.wrapping_add(1);
            }
        }
    } else {
        let old_chips = e.get(this, NiGeometryBufferData::m_ppkVBChip);
        e.call(MEMCPY, &args![chips, old_chips, old_count.wrapping_mul(4)]);
        let old_count = e.get(this, NiGeometryBufferData::m_uiStreamCount);
        let old_strides = e.get(this, NiGeometryBufferData::m_puiVertexStride);
        e.call(
            MEMCPY,
            &args![strides, old_strides, old_count.wrapping_mul(4)],
        );
        let old_count = e.get(this, NiGeometryBufferData::m_uiStreamCount);
        let tail = count.wrapping_sub(old_count).wrapping_mul(4);
        e.call(
            MEMSET,
            &args![chips.wrapping_add(old_count.wrapping_mul(4)), 0u32, tail],
        );
        let old_count = e.get(this, NiGeometryBufferData::m_uiStreamCount);
        let tail = count.wrapping_sub(old_count).wrapping_mul(4);
        e.call(
            MEMSET,
            &args![strides.wrapping_add(old_count.wrapping_mul(4)), 0u32, tail],
        );
    }
    let old_chips = e.get(this, NiGeometryBufferData::m_ppkVBChip);
    e.call(
        MEMORY_MANAGER_FREE,
        &args![MEMORY_MANAGER_OBJECT, old_chips],
    );
    let old_strides = e.get(this, NiGeometryBufferData::m_puiVertexStride);
    e.call(FREE_00AA10F0, &args![old_strides]);
    e.set(this, NiGeometryBufferData::m_ppkVBChip, chips);
    e.set(this, NiGeometryBufferData::m_puiVertexStride, strides);
    e.set(this, NiGeometryBufferData::m_uiStreamCount, count);
}

// Translated from 00e8f040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the vertex declaration. A different declaration releases
/// the old one (when set), is stored and gets a reference (`AddRef`, when not null); in
/// every case the FVF (+0x0C) is cleared.
pub fn fn_00e8f040(e: &mut Engine, this: Ptr<NiGeometryBufferData>, declaration: u32) {
    let old = e.get(this, NiGeometryBufferData::m_hDeclaration);
    if declaration != old {
        if old != 0 {
            e.vcall(old, RELEASE, &[]);
        }
        e.set(this, NiGeometryBufferData::m_hDeclaration, declaration);
        if declaration != 0 {
            e.vcall(declaration, ADD_REF, &[]);
        }
    }
    e.set(this, NiGeometryBufferData::m_uiFVF, 0);
}

// Translated from 00e8f0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryBufferData::~NiGeometryBufferData` (the body the scalar deleting destructor
/// calls): sets the class's vtable; when there is a geometry group, its `ReleaseChip` (slot
/// +0x1C) releases every stream; releases and clears the index buffer, clears its size, frees
/// the chip and stride arrays (`00aa10f0`), releases the declaration when set, and leaves the
/// base class's vtable.
pub fn fn_00e8f0f0(e: &mut Engine, this: Ptr<NiGeometryBufferData>) {
    e.mem.set_u32(this.addr(), BUFFER_DATA_VTABLE);
    if e.get(this, NiGeometryBufferData::m_pkGeometryGroup) != 0 {
        let mut stream = 0u32;
        while stream < e.get(this, NiGeometryBufferData::m_uiStreamCount) {
            let group = e.get(this, NiGeometryBufferData::m_pkGeometryGroup);
            e.vcall(group, GROUP_RELEASE_CHIP, &args![this, stream]);
            stream = stream.wrapping_add(1);
        }
    }
    let ib = e.get(this, NiGeometryBufferData::m_pkIB);
    if ib != 0 {
        e.vcall(ib, RELEASE, &[]);
        e.set(this, NiGeometryBufferData::m_pkIB, 0);
    }
    let chips = e.get(this, NiGeometryBufferData::m_ppkVBChip);
    e.set(this, NiGeometryBufferData::m_uiIBSize, 0);
    e.call(FREE_00AA10F0, &args![chips]);
    let strides = e.get(this, NiGeometryBufferData::m_puiVertexStride);
    e.call(FREE_00AA10F0, &args![strides]);
    let declaration = e.get(this, NiGeometryBufferData::m_hDeclaration);
    if declaration != 0 {
        e.vcall(declaration, RELEASE, &[]);
    }
    e.mem.set_u32(this.addr(), RENDERER_DATA_VTABLE);
}

// Translated from 00e8f170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryBufferData::` scalar deleting destructor (Xbox PDB): runs the destructor
/// (`00e8f0f0`) and, when bit 0 of `flags` is set, frees the object through the memory
/// manager (`00aa4060`). Returns `this`.
pub fn ni_geometry_buffer_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiGeometryBufferData>,
    flags: u32,
) -> Ptr<NiGeometryBufferData> {
    fn_00e8f0f0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

/// `NiGeometryBufferData::m_uiFlags` (Xbox PDB) +0x04.
const FLAGS: u32 = 0x04;

// Translated from 00e8ee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryData::RendererData::'scalar deleting destructor'` (Xbox PDB, engine map),
/// `thiscall` with one stack word (the flags): installs the base vtable (`010f0170`) and, when
/// bit 0 of the flags is set, frees the object through the memory manager (`00aa4060` on
/// `011f6238`). Answers `this`.
pub fn ni_geometry_data_renderer_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), RENDERER_DATA_VTABLE);
    if flags & 1 != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, this]);
    }
    this
}

// Translated from 00e8f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): sets the FVF (+0x0C) and, when the data
/// holds a vertex declaration, releases it (`Release`) and clears it.
pub fn fn_00e8f010(e: &mut Engine, this: Ptr<NiGeometryBufferData>, fvf: u32) {
    e.set(this, NiGeometryBufferData::m_uiFVF, fvf);
    let declaration = e.get(this, NiGeometryBufferData::m_hDeclaration);
    if declaration != 0 {
        e.vcall(declaration, RELEASE, &[]);
        e.set(this, NiGeometryBufferData::m_hDeclaration, 0);
    }
}

// Translated from 00e8f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometryBufferData::ContainsVertexData` (Xbox PDB, engine map), `thiscall`, one word
/// (the kind of data asked about). The compiler's switch on `kind - 3` (byte table
/// `00e8f0e0`, jump table `00e8f0d0`), as `match` arms: 3 answers bit 23 of `m_uiFlags`
/// (+0x04), 4 bit 22, 13 and 14 bit 21, 5 to 12 whether `kind - 5` is below the flags' top
/// byte (+0x07, unsigned); anything else answers 0 (in `AL`).
pub fn ni_geometry_buffer_data_contains_vertex_data(
    e: &mut Engine,
    this: Ptr<NiGeometryBufferData>,
    kind: u32,
) -> bool {
    let flags = e.mem.u32(this.addr() + FLAGS);
    match kind.wrapping_sub(3) {
        0 => (flags >> 0x17) & 1 != 0,
        1 => (flags >> 0x16) & 1 != 0,
        2..=9 => kind.wrapping_sub(5) < u32::from(e.mem.u8(this.addr() + 7)),
        10 | 11 => (flags >> 0x15) & 1 != 0,
        _ => false,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00e8_ee80,
            ni_geometry_data_renderer_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00e8_f010, fn_00e8f010(Ptr<NiGeometryBufferData>, u32)),
        entry!(
            0x00e8_f080,
            ni_geometry_buffer_data_contains_vertex_data(Ptr<NiGeometryBufferData>, u32) -> bool
        ),
        entry!(0x00e8_eeb0, fn_00e8eeb0(Ptr<NiGeometryBufferData>) -> bool),
        entry!(
            0x00e8_eee0,
            ni_geometry_buffer_data_remove_ib(Ptr<NiGeometryBufferData>)
        ),
        entry!(0x00e8_ef10, fn_00e8ef10(Ptr<NiGeometryBufferData>, u32)),
        entry!(0x00e8_f040, fn_00e8f040(Ptr<NiGeometryBufferData>, u32)),
        entry!(0x00e8_f0f0, fn_00e8f0f0(Ptr<NiGeometryBufferData>)),
        entry!(
            0x00e8_f170,
            ni_geometry_buffer_data_scalar_deleting_destructor(
                Ptr<NiGeometryBufferData>,
                u32,
            )
                -> Ptr<NiGeometryBufferData>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const RELEASE_FN: u32 = 0x0040_1000;
    const ADD_REF_FN: u32 = 0x0040_1010;
    const RELEASE_CHIP_FN: u32 = 0x0040_1020;

    /// An engine with doubles for the allocator, `memcpy`/`memset` (real copies) and a COM
    /// object vtable whose AddRef/Release only log.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ALLOCATE_00AA1070, |e, a| {
            e.mem.alloc(a[0].max(4)).into_ret()
        });
        e.register(FREE_00AA10F0, |_, _| Ret::default());
        e.register(MEMORY_MANAGER_FREE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(MEMSET, |e, a| {
            let fill = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &fill);
            a[0].into_ret()
        });
        e.register(RELEASE_FN, |_, _| Ret::default());
        e.register(ADD_REF_FN, |_, _| Ret::default());
        e.register(RELEASE_CHIP_FN, |_, _| Ret::default());
        e.put_vtable(0x0090_0000, &[0, ADD_REF_FN, RELEASE_FN]);
        e.put_vtable(0x0090_1000, &[0, 0, 0, 0, 0, 0, 0, RELEASE_CHIP_FN]);
        e
    }

    fn com_object(e: &mut Engine) -> u32 {
        let o = e.mem.alloc(8);
        e.mem.set_u32(o, 0x0090_0000);
        o
    }

    fn data_with_streams(e: &mut Engine, words: &[(u32, u32)]) -> Ptr<NiGeometryBufferData> {
        let data: Ptr<NiGeometryBufferData> = e.new_object();
        let n = words.len() as u32;
        let chips = e.mem.alloc(4 * n.max(1));
        let strides = e.mem.alloc(4 * n.max(1));
        for (i, (chip, stride)) in words.iter().enumerate() {
            e.mem.set_u32(chips + 4 * i as u32, *chip);
            e.mem.set_u32(strides + 4 * i as u32, *stride);
        }
        e.set(data, NiGeometryBufferData::m_uiStreamCount, n);
        e.set(data, NiGeometryBufferData::m_ppkVBChip, chips);
        e.set(data, NiGeometryBufferData::m_puiVertexStride, strides);
        data
    }

    #[test]
    fn packed_needs_streams_chips_and_buffers() {
        let mut e = engine();
        let empty = data_with_streams(&mut e, &[]);
        assert!(!e.call(0x00e8_eeb0, &args![empty]).bool());
        let chip_a = e.mem.alloc(0x20);
        let chip_b = e.mem.alloc(0x20);
        e.mem.set_u32(chip_a + CHIP_VB, 0x1234);
        let data = data_with_streams(&mut e, &[(chip_a, 0), (chip_b, 0)]);
        assert!(!e.call(0x00e8_eeb0, &args![data]).bool());
        e.mem.set_u32(chip_b + CHIP_VB, 0x5678);
        assert!(e.call(0x00e8_eeb0, &args![data]).bool());
        let missing = data_with_streams(&mut e, &[(chip_a, 0), (0, 0)]);
        assert!(!e.call(0x00e8_eeb0, &args![missing]).bool());
    }

    #[test]
    fn remove_ib_releases_and_clears() {
        let mut e = engine();
        let data: Ptr<NiGeometryBufferData> = e.new_object();
        let ib = com_object(&mut e);
        e.set(data, NiGeometryBufferData::m_pkIB, ib);
        e.set(data, NiGeometryBufferData::m_uiIBSize, 99);
        e.call_log = Some(vec![]);
        e.call(0x00e8_eee0, &args![data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RELEASE_FN), vec![vec![ib]]);
        assert_eq!(e.get(data, NiGeometryBufferData::m_pkIB), 0);
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiIBSize), 0);
        // Without an index buffer only the size is cleared.
        e.set(data, NiGeometryBufferData::m_uiIBSize, 5);
        e.call_log = Some(vec![]);
        e.call(0x00e8_eee0, &args![data]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, RELEASE_FN).is_empty());
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiIBSize), 0);
    }

    #[test]
    fn set_stream_count_grows_shrinks_and_keeps() {
        let mut e = engine();
        let data = data_with_streams(&mut e, &[(0x11, 3), (0x22, 4)]);
        let group = e.mem.alloc(0xc);
        e.mem.set_u32(group, 0x0090_1000);
        e.set(data, NiGeometryBufferData::m_pkGeometryGroup, group);
        // Unchanged: nothing is called.
        e.call_log = Some(vec![]);
        e.call(0x00e8_ef10, &args![data, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);

        // Grow to 4: copies and a zeroed tail.
        let old_chips = e.get(data, NiGeometryBufferData::m_ppkVBChip);
        let old_strides = e.get(data, NiGeometryBufferData::m_puiVertexStride);
        e.call_log = Some(vec![]);
        e.call(0x00e8_ef10, &args![data, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiStreamCount), 4);
        let chips = e.get(data, NiGeometryBufferData::m_ppkVBChip);
        let strides = e.get(data, NiGeometryBufferData::m_puiVertexStride);
        let words = |e: &Engine, p: u32| (0..4).map(|i| e.mem.u32(p + 4 * i)).collect::<Vec<_>>();
        assert_eq!(words(&e, chips), vec![0x11, 0x22, 0, 0]);
        assert_eq!(words(&e, strides), vec![3, 4, 0, 0]);
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, old_chips]]
        );
        assert_eq!(calls_to(&log, FREE_00AA10F0), vec![vec![old_strides]]);
        assert!(calls_to(&log, RELEASE_CHIP_FN).is_empty());

        // Shrink to 1: streams 1..4 are released through the group.
        e.call_log = Some(vec![]);
        e.call(0x00e8_ef10, &args![data, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RELEASE_CHIP_FN),
            vec![
                vec![group, data.addr(), 1],
                vec![group, data.addr(), 2],
                vec![group, data.addr(), 3]
            ]
        );
        let chips = e.get(data, NiGeometryBufferData::m_ppkVBChip);
        assert_eq!(e.mem.u32(chips), 0x11);
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiStreamCount), 1);

        // Shrink without a group: nothing released.
        e.set(data, NiGeometryBufferData::m_pkGeometryGroup, 0);
        e.call_log = Some(vec![]);
        e.call(0x00e8_ef10, &args![data, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, RELEASE_CHIP_FN).is_empty());
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiStreamCount), 0);
    }

    #[test]
    fn set_declaration_swaps_references_and_clears_the_fvf() {
        let mut e = engine();
        let data: Ptr<NiGeometryBufferData> = e.new_object();
        let first = com_object(&mut e);
        let second = com_object(&mut e);
        e.set(data, NiGeometryBufferData::m_uiFVF, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x00e8_f040, &args![data, first]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ADD_REF_FN), vec![vec![first]]);
        assert!(calls_to(&log, RELEASE_FN).is_empty());
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiFVF), 0);
        // The same one again: no reference change.
        e.set(data, NiGeometryBufferData::m_uiFVF, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x00e8_f040, &args![data, first]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiFVF), 0);
        // Another: the old is released, the new referenced.
        e.call_log = Some(vec![]);
        e.call(0x00e8_f040, &args![data, second]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RELEASE_FN), vec![vec![first]]);
        assert_eq!(calls_to(&log, ADD_REF_FN), vec![vec![second]]);
        // Null: released, nothing referenced.
        e.call_log = Some(vec![]);
        e.call(0x00e8_f040, &args![data, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RELEASE_FN), vec![vec![second]]);
        assert!(calls_to(&log, ADD_REF_FN).is_empty());
        assert_eq!(e.get(data, NiGeometryBufferData::m_hDeclaration), 0);
    }

    #[test]
    fn destructor_releases_everything() {
        let mut e = engine();
        let data = data_with_streams(&mut e, &[(0x11, 3), (0x22, 4)]);
        let group = e.mem.alloc(0xc);
        e.mem.set_u32(group, 0x0090_1000);
        e.set(data, NiGeometryBufferData::m_pkGeometryGroup, group);
        let ib = com_object(&mut e);
        let declaration = com_object(&mut e);
        e.set(data, NiGeometryBufferData::m_pkIB, ib);
        e.set(data, NiGeometryBufferData::m_uiIBSize, 7);
        e.set(data, NiGeometryBufferData::m_hDeclaration, declaration);
        let chips = e.get(data, NiGeometryBufferData::m_ppkVBChip);
        let strides = e.get(data, NiGeometryBufferData::m_puiVertexStride);
        e.call_log = Some(vec![]);
        e.call(0x00e8_f0f0, &args![data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RELEASE_CHIP_FN),
            vec![vec![group, data.addr(), 0], vec![group, data.addr(), 1]]
        );
        assert_eq!(
            calls_to(&log, RELEASE_FN),
            vec![vec![ib], vec![declaration]]
        );
        assert_eq!(
            calls_to(&log, FREE_00AA10F0),
            vec![vec![chips], vec![strides]]
        );
        assert_eq!(e.mem.u32(data.addr()), RENDERER_DATA_VTABLE);
        assert_eq!(e.get(data, NiGeometryBufferData::m_pkIB), 0);
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiIBSize), 0);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        let data = data_with_streams(&mut e, &[(0x11, 3)]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_f170, &args![data, 1u32]).u32(), data.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, data.addr()]]
        );
        // Without a group, an IB or a declaration nothing is released.
        assert!(calls_to(&log, RELEASE_FN).is_empty());
        assert!(calls_to(&log, RELEASE_CHIP_FN).is_empty());
        assert_eq!(calls_to(&log, FREE_00AA10F0).len(), 2);

        let bare = data_with_streams(&mut e, &[(0x11, 3)]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_f170, &args![bare, 0u32]).u32(), bare.addr());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MEMORY_MANAGER_FREE).is_empty());
        assert_eq!(e.mem.u32(bare.addr()), RENDERER_DATA_VTABLE);
    }

    #[test]
    fn renderer_data_deleting_destructor_frees_on_bit_0() {
        let mut e = engine();
        let data = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e8_ee80, &args![data, 0u32]).u32(), data);
        assert_eq!(e.mem.u32(data), RENDERER_DATA_VTABLE);
        e.call(0x00e8_ee80, &args![data, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, data]]
        );
    }

    #[test]
    fn set_fvf_releases_the_declaration() {
        let mut e = engine();
        let data: Ptr<NiGeometryBufferData> = e.new_object();
        let declaration = com_object(&mut e);
        e.set(data, NiGeometryBufferData::m_hDeclaration, declaration);
        e.call_log = Some(vec![]);
        e.call(0x00e8_f010, &args![data, 0x142u32]);
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiFVF), 0x142);
        assert_eq!(e.get(data, NiGeometryBufferData::m_hDeclaration), 0);
        e.call(0x00e8_f010, &args![data, 0x2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RELEASE_FN), vec![vec![declaration]]);
        assert_eq!(e.get(data, NiGeometryBufferData::m_uiFVF), 2);
    }

    #[test]
    fn contains_vertex_data_reads_the_flags() {
        let mut e = engine();
        let data: Ptr<NiGeometryBufferData> = e.new_object();
        let ask = |e: &mut Engine, kind: u32| e.call(0x00e8_f080, &args![data, kind]).bool();
        e.mem.set_u32(data.addr() + FLAGS, 0x0380_0000);
        assert!(ask(&mut e, 3));
        assert!(!ask(&mut e, 4));
        assert!(!ask(&mut e, 13));
        // Top byte 3: kinds 5, 6 and 7 are below it.
        assert!(ask(&mut e, 7));
        assert!(!ask(&mut e, 8));
        e.mem.set_u32(data.addr() + FLAGS, 0x0060_0000);
        assert!(!ask(&mut e, 3));
        assert!(ask(&mut e, 4));
        assert!(ask(&mut e, 14));
        assert!(!ask(&mut e, 5));
        for kind in [0u32, 2, 15, 0xffff_ffff] {
            assert!(!ask(&mut e, kind));
        }
    }
}
