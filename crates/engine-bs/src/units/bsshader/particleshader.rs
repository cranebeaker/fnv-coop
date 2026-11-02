//! `BSShader/particleshader.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// Code the shaders' `CreateConstantMaps` functions expand in place: a constant map made into a
/// shader's `NiPointer` and the entries added to it through its vtable. The shader units of
/// this collection call these helpers where their functions contain that code.
pub(crate) mod constant_maps {
    use crate::prelude::*;
    use crate::units::bsshader::geometrydecalshader::shader_inline::ni_pointer_assign;

    /// The Ni allocation `00aa13e0` (`cdecl`, size).
    pub(crate) const NI_ALLOCATE: u32 = 0x00aa_13e0;
    /// `NiD3DShaderConstantMap::NiD3DShaderConstantMap` (engine map, `00b7e330`, `thiscall`,
    /// `RET 8`: renderer, program type).
    pub(crate) const CONSTANT_MAP_CONSTRUCT: u32 = 0x00b7_e330;
    /// The empty string `""` (`01011584`) the entries pass.
    pub(crate) const EMPTY_STRING_01011584: u32 = 0x0101_1584;

    /// A new constant map into the `NiPointer` at `slot`: 0x44 bytes from `00aa13e0`, the
    /// constructor run with the renderer word and the program type when the block is not
    /// null (a null block gives a null map), then the pointer assigned (inlined
    /// `NiPointer::operator=`).
    pub(crate) fn make_constant_map(e: &mut Engine, slot: u32, renderer_word: u32, program: u32) {
        let block = e.call(NI_ALLOCATE, &args![0x44u32]).u32();
        let map = if block != 0 {
            let renderer = e.mem.u32(renderer_word);
            e.call(CONSTANT_MAP_CONSTRUCT, &args![block, renderer, program])
                .u32()
        } else {
            0
        };
        ni_pointer_assign(e, slot, map);
    }

    /// One entry added through the map's vtable slot +0x8 (ten words; `AddEntry` in the Xbox
    /// PDB's slot order, not confirmed for the PC build): the map is read again from `slot`.
    pub(crate) fn add_entry(e: &mut Engine, slot: u32, words: &[u32; 10]) {
        let map = e.mem.u32(slot);
        e.vcall(map, 0x8, words);
    }

    /// One entry added through the map's vtable slot +0x68 (eight words; the Xbox PDB has an
    /// `AddPredefinedEntry` there, not confirmed for the PC build).
    pub(crate) fn add_predefined_entry(e: &mut Engine, slot: u32, words: &[u32; 8]) {
        let map = e.mem.u32(slot);
        e.vcall(map, 0x68, words);
    }
}

use constant_maps::*;
// This unit's own helper, not the shared one of the same name in shader_inline.
use constant_maps::make_constant_map;

/// The particle shader's vtable (`010b80a8`; no RTTI locator before it).
const PARTICLE_VTABLE_010B80A8: u32 = 0x010b_80a8;
/// The shared `NiPointer` the destructor clears (`012003f4`).
const SHARED_POINTER_012003F4: u32 = 0x0120_03f4;
/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `BSShader::~BSShader` (engine map, `00be0f10`, `thiscall`, no stack words).
const BS_SHADER_DESTRUCT: u32 = 0x00be_0f10;
/// The sized free `00aa1460` (`cdecl`: block, size).
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

/// A pass reference dropped without clearing the pointer (the members' own destructors): when
/// the pass is not null its count at +0x64 goes down by one and the pass is deleted through
/// `00e80190` at 0.
pub(crate) fn pass_release(e: &mut Engine, pass: u32) {
    if pass != 0 {
        let left = e.mem.u32(pass + 0x64).wrapping_sub(1);
        e.mem.set_u32(pass + 0x64, left);
        if left == 0 {
            e.call(PASS_DELETE_00E80190, &args![pass]);
        }
    }
}

/// A reference-counted object's reference dropped when it is not null (the members' own
/// `NiPointer` destructors; the pointer is left as it is).
pub(crate) fn reference_release(e: &mut Engine, object: u32) {
    if object != 0 {
        release_reference(e, object);
    }
}

// Translated from 00ba5740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `particleshader.cpp`): the particle shader's destructor,
/// `thiscall`, no stack words. Stores the vtable `010b80a8`, clears the shared `NiPointer` at
/// `012003f4`, frees the block at +0x70 (the memory manager's free `00aa4060`) and clears the
/// word, clears the pass pointers at +0x74 and +0x78 and the `NiPointer`s at +0x7c and +0x80;
/// then the members' destructors run on what is left (+0x80, +0x7c, then the passes +0x78,
/// +0x74) and `BSShader::~BSShader` (`00be0f10`).
// Translation incomplete: 00ba5740 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the members) is not translated.
pub fn fn_00ba5740(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, PARTICLE_VTABLE_010B80A8);
    ni_pointer_clear(e, SHARED_POINTER_012003F4);
    let block = e.mem.u32(base + 0x70);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block]);
    e.mem.set_u32(base + 0x70, 0);
    pass_pointer_clear(e, base + 0x74);
    pass_pointer_clear(e, base + 0x78);
    ni_pointer_clear(e, base + 0x7c);
    ni_pointer_clear(e, base + 0x80);
    for offset in [0x80u32, 0x7c] {
        let object = e.mem.u32(base + offset);
        reference_release(e, object);
    }
    for offset in [0x78u32, 0x74] {
        let pass = e.mem.u32(base + offset);
        pass_release(e, pass);
    }
    e.call(BS_SHADER_DESTRUCT, &args![base]);
}

/// The render states (state, value) `00ba59e0` sets on each of its two passes, in order.
const PARTICLE_RENDER_STATES: [(u32, u32); 9] = [
    (0xa8, 0xf),
    (0x1b, 1),
    (0x13, 5),
    (0x14, 6),
    (0xf, 0),
    (0x34, 0),
    (7, 1),
    (0x17, 4),
    (0xe, 0),
];

/// One pass set up the way the particle and bolt shaders' pass set-ups expand it: a texture
/// stage when the pass has none (`[pass + 0x18]` below 1: `CreateNewStage` into the stage
/// pointer, `SetupDefaultStage(stage, 0, 1, 2, 1)`, `NiD3DPass::SetStage` with the pass's word
/// at +0x14), the shader's vertex and pixel shaders (the words at `vertex` and `pixel`)
/// assigned to the pass's `NiPointer`s at +0x5c and +0x44, then the render states in order.
pub(crate) fn set_up_pass(
    e: &mut Engine,
    pass: u32,
    stage_local: u32,
    vertex: u32,
    pixel: u32,
    states: &[(u32, u32)],
) {
    if e.mem.u32(pass + 0x18) < 1 {
        e.call(CREATE_NEW_STAGE, &args![stage_local]);
        let stage = e.mem.u32(stage_local);
        e.call(SETUP_DEFAULT_STAGE, &args![stage, 0u32, 1u32, 2u32, 1u32]);
        let stage = e.mem.u32(stage_local);
        let position = e.mem.u32(pass + 0x14);
        e.call(PASS_SET_STAGE, &args![pass, position, stage]);
    }
    let vertex = e.mem.u32(vertex);
    ni_pointer_assign(e, pass + 0x5c, vertex);
    let pixel = e.mem.u32(pixel);
    ni_pointer_assign(e, pass + 0x44, pixel);
    for (state, value) in states {
        pass_render_state(e, pass, *state, *value);
    }
}

/// One pass of `00ba59e0`: [`set_up_pass`] with the vertex shader at +0x7c and the pixel
/// shader at +0x80.
fn particle_pass(e: &mut Engine, this: u32, pass: u32, stage_local: u32) {
    set_up_pass(
        e,
        pass,
        stage_local,
        this + 0x7c,
        this + 0x80,
        &PARTICLE_RENDER_STATES,
    );
}

/// A pass held for the time of a set-up (a local `NiPointer` to a pass): the count at +0x64 goes
/// up by one when the pass is not null; answers the pass.
pub(crate) fn pass_hold(e: &mut Engine, pass: u32) -> u32 {
    if pass != 0 {
        let count = e.mem.u32(pass + 0x64).wrapping_add(1);
        e.mem.set_u32(pass + 0x64, count);
    }
    pass
}

/// The held pass dropped at the end of a set-up: its count at +0x64 goes down by one (no null
/// test) and it is deleted through `00e80190` at 0.
pub(crate) fn pass_drop(e: &mut Engine, pass: u32) {
    let left = e.mem.u32(pass + 0x64).wrapping_sub(1);
    e.mem.set_u32(pass + 0x64, left);
    if left == 0 {
        e.call(PASS_DELETE_00E80190, &args![pass]);
    }
}

// Translated from 00ba59e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `particleshader.cpp`): the particle shader's pass set-up,
/// `thiscall`, no stack words. Holds the pass at +0x74 (count at +0x64 up by one when not null)
/// and sets it up (`particle_pass`); then, when the pass at +0x78 is another one, drops the held
/// pass (deleted through `00e80190` at 0), holds +0x78 and sets it up the same way. Drops the
/// stage pointer's reference (one stage pointer for both passes) and the held pass's (no null
/// test on the pass; the code reads the pass's fields without one either).
// Translation incomplete: 00ba59e0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the stage and the pass) is not translated.
pub fn fn_00ba59e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        let first = e.mem.u32(base + 0x74);
        let mut pass = pass_hold(e, first);
        particle_pass(e, base, pass, local);
        if pass != e.mem.u32(base + 0x78) {
            pass_drop(e, pass);
            let second = e.mem.u32(base + 0x78);
            pass = pass_hold(e, second);
        }
        particle_pass(e, base, pass, local);
        let stage = e.mem.u32(local);
        stage_release(e, stage);
        pass_drop(e, pass);
    });
}

// Translated from 00ba6370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `particleshader.cpp`): the particle shader's scalar deleting
/// destructor, `thiscall`, one stack word (the flags). Runs the destructor `00ba5740`; with bit
/// 0 of the flags set, frees the 0x12c bytes (`00aa1460`). Answers `this`.
pub fn fn_00ba6370(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00ba5740(e, this);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x12cu32]);
    }
    this
}

// Translated from 00ba63a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `particleshader.cpp`): `thiscall`, no stack words. Gives
/// the pass pointers at +0x74 and +0x78 a new pass each (`00e804d0`, with the pointer's
/// address) when they are null, sets the passes up (`00ba59e0`) and answers true.
pub fn fn_00ba63a0(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    for offset in [0x74u32, 0x78] {
        if e.mem.u32(base + offset) == 0 {
            e.call(NEW_PASS_00E804D0, &args![base + offset]);
        }
    }
    fn_00ba59e0(e, this);
    true
}

/// `NiObject::NiObject` (`00a5d3a0`, `thiscall`, no stack words).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiAdditionalGeometryData`'s vtable (`0102e274`, RTTI `.?AVNiAdditionalGeometryData@@`) and
/// its data block array's (`0102e474`, RTTI
/// `.?AV?$NiTPrimitiveArray@PAVNiAGDDataBlock@NiAdditionalGeometryData@@@@`).
const ADDITIONAL_GEOMETRY_VTABLE_0102E274: u32 = 0x0102_e274;
const DATA_BLOCK_ARRAY_VTABLE_0102E474: u32 = 0x0102_e474;
/// The compiler's `vector constructor iterator` (`00401050`, `stdcall`: array, element size,
/// count, constructor) and the stream entries' constructor (`00639940`).
const VECTOR_CONSTRUCT_00401050: u32 = 0x0040_1050;
const STREAM_CONSTRUCT_00639940: u32 = 0x0063_9940;

layout! {
    /// `NiAdditionalGeometryData` (Xbox PDB), 0x2c bytes; the PC offsets match.
    pub struct NiAdditionalGeometryData: 0x2c {
        /// `m_uiRevID` (Xbox PDB).
        0x08 m_uiRevID: u32,
        /// `m_usVertexCount` (Xbox PDB).
        0x0c m_usVertexCount: u16,
        /// `m_uiDataStreamCount` (Xbox PDB).
        0x10 m_uiDataStreamCount: u32,
        /// `m_pkDataStreamEntries` (Xbox PDB): 0x1c bytes each.
        0x14 m_pkDataStreamEntries: u32,
        /// `m_uiD3DStream` (Xbox PDB).
        0x18 m_uiD3DStream: u32,
        /// `m_aDataBlocks` (Xbox PDB): a `NiTPrimitiveArray` (vtable +0x1c, base +0x20, max
        /// size +0x24, size +0x26, element count +0x28, grow-by +0x2a).
        0x1c m_aDataBlocks: u32,
    }
}

// Translated from 00ba63e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAdditionalGeometryData::NiAdditionalGeometryData` (engine map, `_ov2`), `thiscall` with two
/// stack words: the vertex count (its low 16 bits) and the data stream count. Runs
/// `NiObject::NiObject` (`00a5d3a0`), stores the vtable `0102e274`, makes the empty data block
/// array (max size, size and element count 0, grow-by 1, no base, vtable `0102e474`), stores
/// the vertex count, clears `m_uiRevID`, stores the stream count and allocates the stream
/// entries (`operator new[]` of count elements of 0x1c bytes from the memory manager; the
/// non-null block's entries built by the `vector constructor iterator` with `00639940`). Then
/// clears the data block array's elements up to its size (none: it was just made empty), its
/// element count and size, and `m_uiD3DStream`. Answers `this`.
// Translation incomplete: 00ba63e0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the array) is not translated.
pub fn ni_additional_geometry_data_ni_additional_geometry_data_ov2(
    e: &mut Engine,
    this: Ptr<NiAdditionalGeometryData>,
    vertex_count: u16,
    stream_count: u32,
) -> Ptr<NiAdditionalGeometryData> {
    let base = this.addr();
    e.call(NI_OBJECT_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, ADDITIONAL_GEOMETRY_VTABLE_0102E274);
    e.mem.set_u16(base + 0x24, 0);
    e.mem.set_u16(base + 0x2a, 1);
    e.mem.set_u16(base + 0x26, 0);
    e.mem.set_u16(base + 0x28, 0);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x1c, DATA_BLOCK_ARRAY_VTABLE_0102E474);
    e.set(
        this,
        NiAdditionalGeometryData::m_usVertexCount,
        vertex_count,
    );
    e.set(this, NiAdditionalGeometryData::m_uiRevID, 0u32);
    e.set(
        this,
        NiAdditionalGeometryData::m_uiDataStreamCount,
        stream_count,
    );
    let block = allocate_array(e, stream_count, 0x1c);
    let entries = if block != 0 {
        e.call(
            VECTOR_CONSTRUCT_00401050,
            &args![block, 0x1cu32, stream_count, STREAM_CONSTRUCT_00639940],
        );
        block
    } else {
        0
    };
    e.set(
        this,
        NiAdditionalGeometryData::m_pkDataStreamEntries,
        entries,
    );
    let mut index = 0u16;
    while index < e.mem.u16(base + 0x26) {
        let elements = e.mem.u32(base + 0x20);
        e.mem.set_u32(elements + 4 * u32::from(index), 0);
        index = index.wrapping_add(1);
    }
    e.mem.set_u16(base + 0x28, 0);
    e.mem.set_u16(base + 0x26, 0);
    e.set(this, NiAdditionalGeometryData::m_uiD3DStream, 0u32);
    this
}

/// Strings of `ParticleShader::CreateConstantMaps` (read in Ghidra): `"WorldViewProjTranspose"`
/// (`010a1e50`), `"Wind"` (`010b8234`), `"Color1"` (`010aff68`), `"Color2"` (`010aff60`),
/// `"Color3"` (`010b822c`), `"Velocity"` (`010b8220`), `"Acceleration"` (`010b8210`),
/// `"Scale Adjustment"` (`010b81fc`), `"fVars"` (`010af454`) and `"particle data"`
/// (`010b81ec`).
const NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50: u32 = 0x010a_1e50;
const NAME_WIND_010B8234: u32 = 0x010b_8234;
const NAME_COLOR1_010AFF68: u32 = 0x010a_ff68;
const NAME_COLOR2_010AFF60: u32 = 0x010a_ff60;
const NAME_COLOR3_010B822C: u32 = 0x010b_822c;
const NAME_VELOCITY_010B8220: u32 = 0x010b_8220;
const NAME_ACCELERATION_010B8210: u32 = 0x010b_8210;
const NAME_SCALE_ADJUSTMENT_010B81FC: u32 = 0x010b_81fc;
const NAME_FVARS_010AF454: u32 = 0x010a_f454;
const NAME_PARTICLE_DATA_010B81EC: u32 = 0x010b_81ec;
/// The scale adjustment constant's data (`011adccc`).
const SCALE_ADJUSTMENT_011ADCCC: u32 = 0x011a_dccc;
/// The particle count the shader's constants are sized for (`012003ec`, set on first use from
/// the shader model setting `011f91c0`).
const PARTICLE_COUNT_012003EC: u32 = 0x0120_03ec;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00ba69f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParticleShader::CreateConstantMaps` (engine map), `thiscall`, no stack words. When the
/// `NiPointer` at +0x34 is null: a constant map (program type 0, the word at +0x18 as the
/// renderer) into it, the predefined entry `"WorldViewProjTranspose"` (0x20000009) and the
/// entries `"Wind"`, `"Color1"` to `"Color3"`, `"Velocity"`, `"Acceleration"`, `"Scale
/// Adjustment"` (0x10000007, registers 0xf, 9 to 0xe; the data at +0xec, +0xfc, +0x10c,
/// +0x11c, +0xd4, +0xe0 and `011adccc`), `"fVars"` (0x10000009, register 4, 5 registers, 0x50
/// bytes at +0x84) and `"particle data"` (0x10000009, register 0x12, 3 registers per particle,
/// 0x30 bytes each, the block at +0x70). The particle count (`012003ec`) becomes 0x1a (shader
/// model setting below 2, signed) or 0x4e when it is 0. When the `NiPointer` at +0x30 is null:
/// a constant map of program type 2 into it.
// Translation incomplete: 00ba69f0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the allocations) is not translated.
pub fn particle_shader_create_constant_maps(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let vertex_map = base + 0x34;
    if e.mem.u32(vertex_map) == 0 {
        make_constant_map(e, vertex_map, base + 0x18, 0);
        add_predefined_entry(
            e,
            vertex_map,
            &[
                NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                0x2000_0009,
                0,
                EMPTY_STRING_01011584,
                0,
                0,
                0,
                0,
            ],
        );
        for (name, register, size, data) in [
            (NAME_WIND_010B8234, 0xfu32, 0x10u32, base + 0xec),
            (NAME_COLOR1_010AFF68, 9, 0x10, base + 0xfc),
            (NAME_COLOR2_010AFF60, 0xa, 0x10, base + 0x10c),
            (NAME_COLOR3_010B822C, 0xb, 0x10, base + 0x11c),
            (NAME_VELOCITY_010B8220, 0xc, 0xc, base + 0xd4),
            (NAME_ACCELERATION_010B8210, 0xd, 0xc, base + 0xe0),
            (
                NAME_SCALE_ADJUSTMENT_010B81FC,
                0xe,
                0x10,
                SCALE_ADJUSTMENT_011ADCCC,
            ),
        ] {
            add_entry(
                e,
                vertex_map,
                &[
                    name,
                    0x1000_0007,
                    0,
                    register,
                    1,
                    EMPTY_STRING_01011584,
                    size,
                    4,
                    data,
                    0,
                ],
            );
        }
        add_entry(
            e,
            vertex_map,
            &[
                NAME_FVARS_010AF454,
                0x1000_0009,
                0,
                4,
                5,
                EMPTY_STRING_01011584,
                0x50,
                4,
                base + 0x84,
                0,
            ],
        );
        if e.global::<u32>(PARTICLE_COUNT_012003EC) == 0 {
            let count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
                0x1a
            } else {
                0x4e
            };
            e.set_global::<u32>(PARTICLE_COUNT_012003EC, count);
        }
        let count = e.global::<u32>(PARTICLE_COUNT_012003EC);
        let data = e.mem.u32(base + 0x70);
        add_entry(
            e,
            vertex_map,
            &[
                NAME_PARTICLE_DATA_010B81EC,
                0x1000_0009,
                0,
                0x12,
                count.wrapping_mul(3),
                EMPTY_STRING_01011584,
                count.wrapping_mul(3).wrapping_shl(4),
                4,
                data,
                0,
            ],
        );
    }
    let pixel_map = base + 0x30;
    if e.mem.u32(pixel_map) == 0 {
        make_constant_map(e, pixel_map, base + 0x18, 2);
    }
}

/// The geometry's shader property (+0xc) when its type word (+0x1c) is 0x11 (the particle
/// shader property's, by its use here), else null (`SETZ`/`NEG`/`SBB`/`AND`).
fn particle_property(e: &Engine, geometry: u32) -> u32 {
    let property = e.mem.u32(geometry + 0xc);
    if property != 0 && e.mem.u32(property + 0x1c) == 0x11 {
        property
    } else {
        0
    }
}

/// The render-state setters of `bsrenderstate_xenon.cpp` (unnamed; `cdecl`): `00b97e80` (two
/// words), `00b97fa0` (two words) and `00b97ff0` (three words), and the index map `00ba5060`
/// (unit `run_00ba3e30`, `cdecl`, one word).
const RENDER_STATE_00B97E80: u32 = 0x00b9_7e80;
const RENDER_STATE_00B97FA0: u32 = 0x00b9_7fa0;
const RENDER_STATE_00B97FF0: u32 = 0x00b9_7ff0;
const INDEX_MAP_00BA5060: u32 = 0x00ba_5060;

// Translated from 00ba5de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 34 of the vtable `010b80a8`, `thiscall`, two
/// stack words (the geometry, one not read). Builds the eight words 7, 1, 2, 3, 4, 5, 6, 0 in a
/// stack local and passes the one at index `property[+0x114] - 1` with 0 to `00b97e80` (the
/// property of type 0x11, else null: its +0x114 read at 0x114). The exe does not check the
/// index: one outside 1 to 8 reads beside the local (here the memory beside it).
pub fn fn_00ba5de0(e: &mut Engine, _this: Ptr, geometry: Ptr, _unused_1: u32) {
    let property = particle_property(e, geometry.addr());
    let index = e.mem.u32(property.wrapping_add(0x114));
    let word = e.with_stack(0x20, |e, local| {
        for (i, word) in [7u32, 1, 2, 3, 4, 5, 6, 0].iter().enumerate() {
            e.mem.set_u32(local.addr() + 4 * i as u32, *word);
        }
        e.mem.u32(
            local
                .addr()
                .wrapping_add(index.wrapping_mul(4))
                .wrapping_sub(4),
        )
    });
    e.call(RENDER_STATE_00B97E80, &args![word, 0u32]);
}

// Translated from 00ba5e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 32 of the vtable `010b80a8`, `thiscall`, one
/// stack word (the geometry): `00b97fa0(1, 0)`; then with a particle property (type 0x11)
/// `00b97ff0` with the index map (`00ba5060`) of its +0x108 and of its +0x10c and 0, else
/// `00b97ff0(6, 7, 0)`.
pub fn fn_00ba5e60(e: &mut Engine, _this: Ptr, geometry: Ptr) {
    let property = particle_property(e, geometry.addr());
    e.call(RENDER_STATE_00B97FA0, &args![1u32, 0u32]);
    if property != 0 {
        let second = e.mem.u32(property + 0x10c);
        let second = e.call(INDEX_MAP_00BA5060, &args![second]).u32();
        let first = e.mem.u32(property + 0x108);
        let first = e.call(INDEX_MAP_00BA5060, &args![first]).u32();
        e.call(RENDER_STATE_00B97FF0, &args![first, second, 0u32]);
    } else {
        e.call(RENDER_STATE_00B97FF0, &args![6u32, 7u32, 0u32]);
    }
}

/// `00ba5320` (unit `run_00ba3e30`, no arguments: the aspect constants).
const ASPECT_CONSTANTS_00BA5320: u32 = 0x00ba_5320;

// Translated from 00ba5ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 63 of the vtable `010b80a8`, `thiscall`, one
/// word it does not read: `00ba5320`.
pub fn fn_00ba5ed0(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    e.call(ASPECT_CONSTANTS_00BA5320, &args![]);
}

/// The accumulator holder (`011f91e0`), `00e6fbb0` (unnamed; `thiscall` on the shader's
/// +0x18, two words), `NiTransform::InvertNonUniform` (engine map, `004b4880`; `thiscall`, the
/// out transform) and `00524c40` (unnamed; `thiscall` on a transform: the out point, the
/// point; answers the out point), the fallback object `011f4288`, the current pass
/// (`0126f74c`), `memcpy` (`00ec44d0`) and the `float` -1.0 (`01012054`).
const ACCUMULATOR_011F91E0: u32 = 0x011f_91e0;
const FN_00E6FBB0: u32 = 0x00e6_fbb0;
const INVERT_NON_UNIFORM_004B4880: u32 = 0x004b_4880;
const TRANSFORM_POINT_00524C40: u32 = 0x0052_4c40;
const FALLBACK_011F4288: u32 = 0x011f_4288;
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;
const MEMCPY_00EC44D0: u32 = 0x00ec_44d0;
const MINUS_ONE_01012054: u32 = 0x0101_2054;

/// The particle count (`012003ec`), set on first use to 0x1a (shader model setting `011f91c0`
/// below 2, signed) or 0x4e.
fn particle_count(e: &mut Engine) -> u32 {
    let mut count = e.global::<u32>(PARTICLE_COUNT_012003EC);
    if count == 0 {
        count = if (e.global::<u32>(SHADER_MODEL_011F91C0) as i32) < 2 {
            0x1a
        } else {
            0x4e
        };
        e.set_global(PARTICLE_COUNT_012003EC, count);
    }
    count
}

/// `FLD`/`FSTP` copies of `float`s (a signalling NaN quieted): `to + pairs.1` from
/// `from + pairs.0`.
fn copy_floats(e: &mut Engine, from: u32, to: u32, pairs: &[(u32, u32)]) {
    for (source, target) in pairs {
        let word = x87_quiet_bits(e.mem.u32(from + source));
        e.mem.set_u32(to + target, word);
    }
}

// Translated from 00ba5ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 31 of the vtable `010b80a8`, `thiscall`, one
/// stack word (the geometry). With A the accumulator (`*011f91e0`): `00e6fbb0` on the shader's
/// +0x18 with A + 0x68 and 0. With a particle property P (type 0x11) the shader's constants
/// are filled from it: `float`s through the FPU (+0x84 to +0x90 from P+0x104, +0x7c, +0x84,
/// +0x8c; +0x94 to +0x9c from P+0xb8; +0xa4 to +0xb0 from P+0xf4; +0xc0 from P+0x130; +0xf8 from
/// P+0x148), words copied (+0xd4 to +0xe8 from P+0x90, +0xfc to +0x128 from P+0xc4), +0xec to
/// +0xf4 P's point +0x13c (moved into A's +0x68 space when P's byte +0x6c is clear: the
/// transform inverted with `004b4880`, the point through `00524c40`), +0xb4 to +0xbc A's +0x8c
/// to +0x94 with the byte set else 0.0, +0xc4 = 1 / P+0x134 and +0xc8 = 1 / P+0x138; the
/// particle count's (`012003ec`) 0x30-byte records copied from P+0x60 to the block at +0x70.
/// Then each constant map (+0x30 with the current pass's +0x44, +0x34 with its +0x5c), when
/// both are set, gets its slot +0x78 with (that word, A, A[+0xbc], 0, A[+0xb8][+0x34], the
/// geometry, 0, A + 0x68, A[+0x20] or `011f4288` when null, 0, 1).
pub fn fn_00ba5ee0(e: &mut Engine, this: Ptr, geometry: Ptr) {
    let base = this.addr();
    let accumulator = e.mem.u32(e.global::<u32>(ACCUMULATOR_011F91E0));
    let space = accumulator + 0x68;
    let member = e.mem.u32(base + 0x18);
    e.call(FN_00E6FBB0, &args![member, space, 0u32]);
    let property = particle_property(e, geometry.addr());
    if property != 0 {
        copy_floats(
            e,
            property,
            base,
            &[(0x104, 0x84), (0x7c, 0x88), (0x84, 0x8c), (0x8c, 0x90)],
        );
        for index in 0..6u32 {
            let word = e.mem.u32(property + 0x90 + 4 * index);
            e.mem.set_u32(base + 0xd4 + 4 * index, word);
        }
        let local_point = e.mem.u8(property + 0x6c) != 0;
        let point = e.with_stack(0x58, |e, frame| {
            // The point at +0, the out point at +0xc, the inverted transform at +0x18.
            let point = frame.addr();
            for index in 0..3u32 {
                let word = e.mem.u32(property + 0x13c + 4 * index);
                e.mem.set_u32(point + 4 * index, word);
            }
            if !local_point {
                let inverse = point + 0x18;
                e.call(INVERT_NON_UNIFORM_004B4880, &args![space, inverse]);
                let out = e
                    .call(
                        TRANSFORM_POINT_00524C40,
                        &args![inverse, point + 0xc, point],
                    )
                    .u32();
                for index in 0..3u32 {
                    let word = e.mem.u32(out + 4 * index);
                    e.mem.set_u32(point + 4 * index, word);
                }
            }
            [0u32, 4, 8].map(|offset| e.mem.u32(point + offset))
        });
        for (index, word) in point.iter().enumerate() {
            e.mem
                .set_u32(base + 0xec + 4 * index as u32, x87_quiet_bits(*word));
        }
        copy_floats(e, property, base, &[(0x148, 0xf8)]);
        for index in 0..12u32 {
            let word = e.mem.u32(property + 0xc4 + 4 * index);
            e.mem.set_u32(base + 0xfc + 4 * index, word);
        }
        copy_floats(
            e,
            property,
            base,
            &[
                (0xb8, 0x94),
                (0xbc, 0x98),
                (0xc0, 0x9c),
                (0xf4, 0xa4),
                (0xf8, 0xa8),
                (0xfc, 0xac),
                (0x100, 0xb0),
            ],
        );
        if e.mem.u8(property + 0x6c) != 0 {
            copy_floats(
                e,
                accumulator,
                base,
                &[(0x8c, 0xb4), (0x90, 0xb8), (0x94, 0xbc)],
            );
        } else {
            for offset in [0xb4u32, 0xb8, 0xbc] {
                e.mem.set_u32(base + offset, 0);
            }
        }
        copy_floats(e, property, base, &[(0x130, 0xc0)]);
        let size = x87_load32(e.mem.u32(property + 0x134));
        e.mem.set_u32(base + 0xc4, x87_store32(x87_div(1.0, size)));
        let rate = x87_div(1.0, x87_m32(e.mem.u32(property + 0x138)));
        e.mem.set_u32(base + 0xc8, x87_store32(rate));
        let count = particle_count(e);
        let records = e.mem.u32(property + 0x60);
        let block = e.mem.u32(base + 0x70);
        e.call(
            MEMCPY_00EC44D0,
            &args![block, records, count.wrapping_mul(3).wrapping_shl(4)],
        );
    }
    let mut object = e.mem.u32(accumulator + 0x20);
    if object == 0 {
        object = FALLBACK_011F4288;
    }
    let word_34 = e.mem.u32(e.mem.u32(accumulator + 0xb8) + 0x34);
    let word_bc = e.mem.u32(accumulator + 0xbc);
    for (map_offset, pass_offset) in [(0x30u32, 0x44u32), (0x34, 0x5c)] {
        let map = e.mem.u32(base + map_offset);
        if map == 0 {
            continue;
        }
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        let shader = e.mem.u32(pass + pass_offset);
        if shader == 0 {
            continue;
        }
        e.vcall(
            map,
            0x78,
            &args![
                shader,
                accumulator,
                word_bc,
                0u32,
                word_34,
                geometry,
                0u32,
                space,
                object,
                0u32,
                1u32
            ],
        );
    }
}

/// The stage functions `00e7de90` and `00e7eb00` (unnamed; `thiscall` on a stage, one word).
const STAGE_SET_00E7DE90: u32 = 0x00e7_de90;
const STAGE_SET_00E7EB00: u32 = 0x00e7_eb00;

// Translated from 00ba6240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 30 of the vtable `010b80a8`, `thiscall`, one
/// stack word (the geometry): with a particle property (type 0x11) the current pass's
/// (`0126f74c`) first stage takes the property's texture (+0x118) at +8, then `00e7de90(3)`
/// and `00e7eb00(0)` on the stage.
pub fn fn_00ba6240(e: &mut Engine, _this: Ptr, geometry: Ptr) {
    let property = particle_property(e, geometry.addr());
    if property == 0 {
        return;
    }
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    let stage = e.mem.u32(e.mem.u32(pass + 0x24));
    let texture = e.mem.u32(property + 0x118);
    e.mem.set_u32(stage + 8, texture);
    e.call(STAGE_SET_00E7DE90, &args![stage, 3u32]);
    e.call(STAGE_SET_00E7EB00, &args![stage, 0u32]);
}

/// `NiAVObject::GetProperty` (engine map, `00a59d30`; `thiscall`, the property type).
const GET_PROPERTY_00A59D30: u32 = 0x00a5_9d30;
/// The count of started particle systems (`012003f0`).
const STARTED_012003F0: u32 = 0x0120_03f0;

/// A node's child's particle property (type 0x11 among its type-3 properties), else null.
fn child_particle_property(e: &mut Engine, child: u32) -> u32 {
    if child == 0 {
        return 0;
    }
    let property = e.call(GET_PROPERTY_00A59D30, &args![child, 3u32]).u32();
    if property != 0 && e.mem.u32(property + 0x1c) == 0x11 {
        property
    } else {
        0
    }
}

/// The shared geometry `NiPointer` (`012003f4`) released: the count at +4 decremented
/// (`InterlockedDecrement`), slot +4 at 0.
fn release_shared(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object + 4])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

// Translated from 00ba6290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParticleShader::Stop` (Xbox PDB), `cdecl`, two words (an owner, a particle property).
/// With an owner whose slot +0xc answers a node and a property: each child of the node (count
/// u16 +0xa6, array +0xa0) whose particle property is the given one is removed (the node's slot
/// +0xf0 with the index); then the started count (`012003f0`) goes down, and at 0 or below the
/// shared geometry (`012003f4`) is released and cleared and the count set to 0.
pub fn particle_shader_stop(e: &mut Engine, owner: u32, property: u32) {
    if owner == 0 {
        return;
    }
    let node = e.vcall(owner, 0xc, &args![]).u32();
    if node == 0 || property == 0 {
        return;
    }
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(node + 0xa6)) {
        let child = e.mem.u32(e.mem.u32(node + 0xa0).wrapping_add(4 * index));
        let found = child_particle_property(e, child);
        if found != 0 && found == property {
            e.vcall(node, 0xf0, &args![index]);
        }
        index += 1;
    }
    let started = e.global::<u32>(STARTED_012003F0).wrapping_sub(1);
    e.set_global(STARTED_012003F0, started);
    if started as i32 <= 0 {
        let shared = e.global::<u32>(SHARED_POINTER_012003F4);
        if shared != 0 {
            release_shared(e, shared);
            e.set_global(SHARED_POINTER_012003F4, 0u32);
        }
        e.set_global(STARTED_012003F0, 0u32);
    }
}

/// What `GetParticleGeometry` makes: `NiTriShapeData::NiTriShapeData` (`00a7b630`, nine
/// words), `NiAdditionalGeometryData::SetDataBlockCount` (`00a72ca0`, one word),
/// `ParticleShaderProperty::GetInstanceIndex` (`00ba73a0`, `cdecl`, two words; they stay on
/// the stack as `SetDataBlock`'s last two), `NiAdditionalGeometryData::SetDataBlock`
/// (`00a72bb0`, four words), `::SetDataStream` (`00a726a0`, seven words), `00a67260`
/// (unnamed, unit `nigeometrydata.cpp`; one word: the additional data), `NiTriShape::NiTriShape`
/// (`00a74480`, one word), `00ba7f70` (unnamed, unit `particleshaderproperty.cpp`: a
/// constructor), `NiAVObject::AttachProperty` (`00439410`, one word) (engine map), the `float`
/// 512.0 (`010231a0`) and the Ni allocation (`00aa13e0`).
const TRI_SHAPE_DATA_00A7B630: u32 = 0x00a7_b630;
const SET_DATA_BLOCK_COUNT_00A72CA0: u32 = 0x00a7_2ca0;
const GET_INSTANCE_INDEX_00BA73A0: u32 = 0x00ba_73a0;
const SET_DATA_BLOCK_00A72BB0: u32 = 0x00a7_2bb0;
const SET_DATA_STREAM_00A726A0: u32 = 0x00a7_26a0;
const SET_ADDITIONAL_00A67260: u32 = 0x00a6_7260;
const TRI_SHAPE_00A74480: u32 = 0x00a7_4480;
const PROPERTY_CONSTRUCT_00BA7F70: u32 = 0x00ba_7f70;
const ATTACH_PROPERTY_00439410: u32 = 0x0043_9410;
const RADIUS_010231A0: u32 = 0x0102_31a0;

// Translated from 00ba64e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ba64e0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the allocations) and the security-cookie bookkeeping are not translated.
/// `ParticleShader::GetParticleGeometry` (Xbox PDB), no arguments: null with the shader model
/// setting (`011f91c0`) 0; the shared geometry (`012003f4`) when there is one. Otherwise
/// builds it for the particle count `n` (`012003ec`, set on first use): `4n` vertices (16-bit)
/// at (-1, -1, 0), (1, -1, 0), (1, 1, 0), (-1, 1, 0) per quad with UVs (0, 0), (1, 0), (1,
/// 1), (0, 1) and `2n` triangles (v, v+1, v+2), (v+3, v, v+2), the three arrays from the memory
/// manager (sizes checked for overflow); an `NiTriShapeData` over them (0x58 bytes), an
/// `NiAdditionalGeometryData` (0x2c bytes, `00ba63e0` with the vertex count and 1) with one data
/// block (the instance indices, `4 * 4n` bytes) and one stream (`SetDataStream(0, 0, 0, 1,
/// 4n, 4, 4)`) given to the data, an `NiTriShape` (0xc4 bytes) whose bound radius becomes 512
/// and which gets a new particle shader property (0x14c bytes, `00ba7f70`). It replaces the
/// shared `NiPointer` (old one released, new one held). Null allocations are used as they are.
pub fn particle_shader_get_particle_geometry(e: &mut Engine) -> u32 {
    if e.global::<u32>(SHADER_MODEL_011F91C0) == 0 {
        return 0;
    }
    let shared = e.global::<u32>(SHARED_POINTER_012003F4);
    if shared != 0 {
        return shared;
    }
    let count = particle_count(e) & 0xffff;
    let triangles = count.wrapping_mul(2) & 0xffff;
    let vertices = count.wrapping_mul(4) & 0xffff;
    let allocate =
        |e: &mut Engine, size: u32| e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    let index_bytes = triangles.wrapping_mul(3).checked_mul(2).unwrap_or(u32::MAX);
    let indices = allocate(e, index_bytes);
    let positions = allocate(e, vertices.checked_mul(0xc).unwrap_or(u32::MAX));
    let uvs = allocate(e, vertices.checked_mul(8).unwrap_or(u32::MAX));
    if count != 0 {
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE_01012054));
        let one = 0x3f80_0000u32;
        let quad = [
            minus_one, minus_one, 0, one, minus_one, 0, one, one, 0, minus_one, one, 0,
        ];
        let coordinates = [0u32, 0, one, 0, one, one, 0, one];
        let mut vertex = 0u32;
        let mut cursor = indices;
        for _ in 0..count {
            let first = vertex & 0xffff;
            for (index, word) in quad.iter().enumerate() {
                e.mem.set_u32(
                    positions.wrapping_add(first * 0xc + 4 * index as u32),
                    *word,
                );
            }
            for (index, word) in coordinates.iter().enumerate() {
                e.mem
                    .set_u32(uvs.wrapping_add(first * 8 + 4 * index as u32), *word);
            }
            let v = vertex as u16;
            let words = [
                v,
                v.wrapping_add(1),
                v.wrapping_add(2),
                v.wrapping_add(3),
                v,
                v.wrapping_add(2),
            ];
            for (index, word) in words.iter().enumerate() {
                e.mem.set_u16(cursor + 2 * index as u32, *word);
            }
            vertex = vertex.wrapping_add(4);
            cursor = cursor.wrapping_add(0xc);
        }
    }
    let block = e.call(NI_ALLOCATE, &args![0x58u32]).u32();
    let data = if block != 0 {
        e.call(
            TRI_SHAPE_DATA_00A7B630,
            &args![block, vertices, positions, 0u32, 0u32, uvs, 1u32, 0u32, triangles, indices],
        )
        .u32()
    } else {
        0
    };
    let block = e.call(NI_ALLOCATE, &args![0x2cu32]).u32();
    let extra = if block != 0 {
        ni_additional_geometry_data_ni_additional_geometry_data_ov2(
            e,
            Ptr::new(block),
            vertices as u16,
            1,
        )
        .addr()
    } else {
        0
    };
    e.call(SET_DATA_BLOCK_COUNT_00A72CA0, &args![extra, 1u32]);
    let bytes = vertices.wrapping_mul(4);
    let instance = e
        .call(GET_INSTANCE_INDEX_00BA73A0, &args![bytes, 1u32])
        .u32();
    e.call(
        SET_DATA_BLOCK_00A72BB0,
        &args![extra, 0u32, instance, bytes, 1u32],
    );
    e.call(
        SET_DATA_STREAM_00A726A0,
        &args![extra, 0u32, 0u32, 0u32, 1u32, vertices, 4u32, 4u32],
    );
    e.call(SET_ADDITIONAL_00A67260, &args![data, extra]);
    let block = e.call(NI_ALLOCATE, &args![0xc4u32]).u32();
    let shape = if block != 0 {
        e.call(TRI_SHAPE_00A74480, &args![block, data]).u32()
    } else {
        0
    };
    let bound = e.mem.u32(shape.wrapping_add(0xb8));
    let center = [0x10u32, 0x14, 0x18].map(|offset| e.mem.u32(bound + offset));
    let radius = x87_quiet_bits(e.global::<u32>(RADIUS_010231A0));
    e.mem.set_u32(bound + 0x1c, radius);
    for (offset, word) in [0x10u32, 0x14, 0x18].iter().zip(center) {
        e.mem.set_u32(bound + offset, word);
    }
    let block = e.call(NI_ALLOCATE, &args![0x14cu32]).u32();
    let property = if block != 0 {
        e.call(PROPERTY_CONSTRUCT_00BA7F70, &args![block]).u32()
    } else {
        0
    };
    e.call(ATTACH_PROPERTY_00439410, &args![shape, property]);
    let old = e.global::<u32>(SHARED_POINTER_012003F4);
    if old != shape {
        if old != 0 {
            release_shared(e, old);
        }
        e.set_global(SHARED_POINTER_012003F4, shape);
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![shape.wrapping_add(4)]);
    }
    e.global::<u32>(SHARED_POINTER_012003F4)
}

/// `NiObject::Clone` (engine map, `00a5d680`; `thiscall`, no stack words), the node update
/// `00a59c60` (unnamed; `thiscall`, the address of an update record) and
/// `BSShaderManager::PrepareObject` (engine map, `00b57e30`; `cdecl`, three words).
const CLONE_00A5D680: u32 = 0x00a5_d680;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
const PREPARE_OBJECT_00B57E30: u32 = 0x00b5_7e30;

// Translated from 00ba68d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParticleShader::Start` (Xbox PDB), `cdecl`, two words (an owner, an object with a camera
/// at +0x20). With an owner whose slot +0xc answers a node: clones the particle geometry
/// (`GetParticleGeometry`, `NiObject::Clone`), sets the clone's bound radius to the larger of
/// it and twice the camera's (+0x20, `011f4288` when null) `float` +0xc (`FCOM`: the radius
/// when the doubled value is below it, else the doubled value, NaN included), attaches it to
/// the node (slot +0xdc with 1), updates the node (`00a59c60` with a zero record), prepares the
/// clone (`PrepareObject(clone, 0, 0)`); with a particle property (type 0x11) on the clone its
/// slot +0x9c gets the clone, `GetParticleGeometry` runs again and the started count
/// (`012003f0`) goes up. Answers that property (null without an owner, node or property).
pub fn particle_shader_start(e: &mut Engine, owner: u32, holder: u32) -> u32 {
    if owner == 0 {
        return 0;
    }
    let node = e.vcall(owner, 0xc, &args![]).u32();
    if node == 0 {
        return 0;
    }
    let geometry = particle_shader_get_particle_geometry(e);
    let clone = e.call(CLONE_00A5D680, &args![geometry]).u32();
    let bound = e.mem.u32(clone + 0xb8);
    let center = [0x10u32, 0x14, 0x18].map(|offset| e.mem.u32(bound + offset));
    let radius_bits = e.mem.u32(bound + 0x1c);
    let mut camera = e.mem.u32(holder + 0x20);
    if camera == 0 {
        camera = FALLBACK_011F4288;
    }
    let value = x87_load32(e.mem.u32(camera + 0xc));
    let doubled_bits = x87_store32(x87_add(value, value));
    let radius = x87_load32(radius_bits);
    let doubled = x87_load32(doubled_bits);
    let chosen = if doubled < radius { radius } else { doubled };
    e.mem.set_u32(bound + 0x10, center[0]);
    let chosen_bits = x87_quiet_bits(x87_store32(chosen));
    e.mem.set_u32(bound + 0x14, center[1]);
    e.mem.set_u32(bound + 0x1c, chosen_bits);
    e.mem.set_u32(bound + 0x18, center[2]);
    e.vcall(node, 0xdc, &args![clone, 1u32]);
    e.with_stack(0xc, |e, record| {
        e.mem.set_u32(record.addr(), x87_store32(0.0));
        for offset in 4..9u32 {
            e.mem.set_u8(record.addr() + offset, 0);
        }
        e.call(UPDATE_00A59C60, &args![node, record]);
    });
    e.call(PREPARE_OBJECT_00B57E30, &args![clone, 0u32, 0u32]);
    let found = e.call(GET_PROPERTY_00A59D30, &args![clone, 3u32]).u32();
    let property = if found != 0 && e.mem.u32(found + 0x1c) == 0x11 {
        found
    } else {
        0
    };
    if property != 0 {
        e.vcall(property, 0x9c, &args![clone]);
        particle_shader_get_particle_geometry(e);
        let started = e.global::<u32>(STARTED_012003F0).wrapping_add(1);
        e.set_global(STARTED_012003F0, started);
    }
    property
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_5de0, fn_00ba5de0(Ptr, Ptr, u32)),
        entry!(0x00ba_5e60, fn_00ba5e60(Ptr, Ptr)),
        entry!(0x00ba_5ed0, fn_00ba5ed0(Ptr, u32)),
        entry!(0x00ba_5ee0, fn_00ba5ee0(Ptr, Ptr)),
        entry!(0x00ba_6240, fn_00ba6240(Ptr, Ptr)),
        entry!(0x00ba_6290, particle_shader_stop(u32, u32)),
        entry!(0x00ba_64e0, particle_shader_get_particle_geometry() -> u32),
        entry!(0x00ba_68d0, particle_shader_start(u32, u32) -> u32),
        entry!(0x00ba_5740, fn_00ba5740(Ptr)),
        entry!(0x00ba_59e0, fn_00ba59e0(Ptr)),
        entry!(0x00ba_6370, fn_00ba6370(Ptr, u32) -> Ptr),
        entry!(0x00ba_63a0, fn_00ba63a0(Ptr) -> bool),
        entry!(
            0x00ba_63e0,
            ni_additional_geometry_data_ni_additional_geometry_data_ov2(
                Ptr<NiAdditionalGeometryData>,
                u16,
                u32,
            ) -> Ptr<
                NiAdditionalGeometryData,
            >
        ),
        entry!(0x00ba_69f0, particle_shader_create_constant_maps(Ptr)),
    ]
}

/// Test doubles the collection's constant map tests share.
#[cfg(test)]
pub(crate) mod constant_map_rig {
    use super::constant_maps::*;
    use crate::prelude::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// The doubles behind a test map's vtable slots +0x8 and +0x68.
    pub(crate) const MAP_ADD_ENTRY: u32 = 0x0e72_0008;
    pub(crate) const MAP_ADD_PREDEFINED: u32 = 0x0e72_0068;
    /// The double behind slot +0x70 (the entry lookup by name): answers the name plus 1.
    pub(crate) const MAP_LOOKUP: u32 = 0x0e72_0070;
    const MAP_VTABLE: u32 = 0x0e73_0000;

    /// [`rig`] with doubles for the allocation (a new block), the map constructor (the test map
    /// vtable, count 0, the renderer and program type noted at +0x10 and +0x14) and the map's
    /// slots +0x8, +0x68 and +0x70.
    pub(crate) fn map_rig() -> Engine {
        let mut e = rig();
        let mut slots = vec![0u32; 0x1d];
        slots[1] = REFERENCE_DELETE;
        slots[2] = MAP_ADD_ENTRY;
        slots[0x1a] = MAP_ADD_PREDEFINED;
        slots[0x1c] = MAP_LOOKUP;
        e.register(MAP_LOOKUP, |_, a| (a[1] + 1).into_ret());
        e.put_vtable(MAP_VTABLE, &slots);
        e.register(MAP_ADD_ENTRY, |_, _| Ret::default());
        e.register(MAP_ADD_PREDEFINED, |_, _| Ret::default());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTANT_MAP_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], MAP_VTABLE);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 0x10, a[1]);
            e.mem.set_u32(a[0] + 0x14, a[2]);
            a[0].into_ret()
        });
        e
    }

    /// The (renderer, program type) a map made by [`map_rig`] was constructed with.
    pub(crate) fn map_made_with(e: &Engine, map: u32) -> (u32, u32) {
        (e.mem.u32(map + 0x10), e.mem.u32(map + 0x14))
    }

    /// The words (without the map) of every entry added to `map` through slot +0x8.
    pub(crate) fn entries(e: &Engine, map: u32) -> Vec<Vec<u32>> {
        calls(e, MAP_ADD_ENTRY)
            .into_iter()
            .filter(|a| a[0] == map)
            .map(|a| a[1..].to_vec())
            .collect()
    }

    /// The words (without the map) of every entry added to `map` through slot +0x68.
    pub(crate) fn predefined(e: &Engine, map: u32) -> Vec<Vec<u32>> {
        calls(e, MAP_ADD_PREDEFINED)
            .into_iter()
            .filter(|a| a[0] == map)
            .map(|a| a[1..].to_vec())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::constant_map_rig::*;
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn destructor_engine() -> Engine {
        let mut e = rig();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(BS_SHADER_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE_SIZED_00AA1460, |_, _| Ret::default());
        e.map(0x0120_0000, 0x1000);
        e
    }

    #[test]
    fn fn_00ba5740_releases_the_members_then_the_base() {
        let mut e = destructor_engine();
        let shared = object(&mut e, 1);
        e.set_global::<u32>(SHARED_POINTER_012003F4, shared);
        let this = e.mem.alloc(0x12c);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        let kept = e.mem.alloc(0x70);
        e.mem.set_u32(kept + 0x64, 2);
        let vertex = object(&mut e, 2);
        e.mem.set_u32(this + 0x70, 0x0e55_0000);
        e.mem.set_u32(this + 0x74, pass);
        e.mem.set_u32(this + 0x78, kept);
        e.mem.set_u32(this + 0x7c, vertex);
        e.call(0x00ba_5740, &args![this]);
        assert_eq!(e.mem.u32(this), PARTICLE_VTABLE_010B80A8);
        assert_eq!(e.global::<u32>(SHARED_POINTER_012003F4), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![shared]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x0e55_0000]]
        );
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(e.mem.u32(kept + 0x64), 1);
        assert_eq!(e.mem.u32(vertex + 4), 1);
        for offset in [0x70u32, 0x74, 0x78, 0x7c, 0x80] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn fn_00ba6370_frees_only_with_bit_0() {
        let mut e = destructor_engine();
        let this = e.mem.alloc(0x12c);
        assert_eq!(e.call(0x00ba_6370, &args![this, 2u32]).u32(), this);
        assert!(calls(&e, NI_FREE_SIZED_00AA1460).is_empty());
        e.call(0x00ba_6370, &args![this, 1u32]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x12c]]);
        assert_eq!(calls(&e, BS_SHADER_DESTRUCT).len(), 2);
    }

    /// A shader with a vertex and a pixel shader at +0x7c and +0x80.
    fn pass_shader(e: &mut Engine) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x12c);
        let vertex = object(e, 1);
        let pixel = object(e, 1);
        e.mem.set_u32(this + 0x7c, vertex);
        e.mem.set_u32(this + 0x80, pixel);
        (this, vertex, pixel)
    }

    #[test]
    fn fn_00ba63a0_makes_two_passes_and_sets_them_up() {
        let mut e = pass_rig();
        let (this, vertex, pixel) = pass_shader(&mut e);
        assert!(e.call(0x00ba_63a0, &args![this]).bool());
        let first = e.mem.u32(this + 0x74);
        let second = e.mem.u32(this + 0x78);
        assert_eq!(
            calls(&e, NEW_PASS_00E804D0),
            vec![vec![this + 0x74], vec![this + 0x78]]
        );
        for pass in [first, second] {
            assert_eq!(stage_setups(&e, pass), vec![(0, [1, 2, 1])]);
            assert_eq!(e.mem.u32(pass + 0x5c), vertex);
            assert_eq!(e.mem.u32(pass + 0x44), pixel);
            assert_eq!(render_states(&e, pass), PARTICLE_RENDER_STATES.to_vec());
            // Held and dropped again: the count is back to the new pass's 1.
            assert_eq!(e.mem.u32(pass + 0x64), 1);
        }
        assert_eq!(e.mem.u32(vertex + 4), 3);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), Vec::<Vec<u32>>::new());
    }

    /// The (index, three words) `SetupDefaultStage` got for each stage `SetStage` put on the
    /// pass (the particle passes set no filter mode).
    fn stage_setups(e: &Engine, pass: u32) -> Vec<(u32, [u32; 3])> {
        let setups = calls(e, SETUP_DEFAULT_STAGE);
        calls(e, PASS_SET_STAGE)
            .iter()
            .filter(|a| a[0] == pass)
            .map(|a| {
                let setup = setups.iter().find(|s| s[0] == a[2]).unwrap();
                (setup[1], [setup[2], setup[3], setup[4]])
            })
            .collect()
    }

    #[test]
    fn fn_00ba59e0_skips_stages_on_passes_that_have_them_and_one_shared_pass() {
        let mut e = pass_rig();
        let (this, _, _) = pass_shader(&mut e);
        let pass = e.mem.alloc(0x70);
        e.mem.set_u32(pass + 0x64, 1);
        e.mem.set_u32(pass + 0x18, 1);
        e.mem.set_u32(this + 0x74, pass);
        e.mem.set_u32(this + 0x78, pass);
        e.call(0x00ba_59e0, &args![this]);
        assert!(calls(&e, CREATE_NEW_STAGE).is_empty());
        assert_eq!(render_states(&e, pass).len(), 18);
        assert_eq!(e.mem.u32(pass + 0x64), 1);
        // A pass with the last reference: deleted when the second pass replaces it.
        let (this, _, _) = pass_shader(&mut e);
        let other = e.mem.alloc(0x70);
        e.mem.set_u32(other + 0x64, 1);
        e.mem.set_u32(this + 0x74, pass);
        e.mem.set_u32(this + 0x78, other);
        e.mem.set_u32(pass + 0x64, 0);
        e.call(0x00ba_59e0, &args![this]);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![pass]]);
        assert_eq!(calls(&e, CREATE_NEW_STAGE).len(), 1);
    }

    #[test]
    fn additional_geometry_data_constructor_allocates_the_stream_entries() {
        let mut e = rig();
        e.register(NI_OBJECT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(VECTOR_CONSTRUCT_00401050, |_, _| Ret::default());
        let this = e.mem.alloc(0x2c);
        e.mem.write(this, &[0xcc; 0x2c]);
        let answer = e
            .call(0x00ba_63e0, &args![this, 0xabcd_0012u32, 3u32])
            .u32();
        assert_eq!(answer, this);
        assert_eq!(e.mem.u32(this), ADDITIONAL_GEOMETRY_VTABLE_0102E274);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u16(this + 0xc), 0x12);
        assert_eq!(e.mem.u16(this + 0xe), 0xcccc);
        assert_eq!(e.mem.u32(this + 0x10), 3);
        let entries = e.mem.u32(this + 0x14);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x54]]);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT_00401050),
            vec![vec![entries, 0x1c, 3, STREAM_CONSTRUCT_00639940]]
        );
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), DATA_BLOCK_ARRAY_VTABLE_0102E474);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.bytes(this + 0x24, 6), vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(e.mem.u16(this + 0x2a), 1);
        // An overflowing count asks for all ones; a null block keeps no entries.
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.call(0x00ba_63e0, &args![this, 1u32, 0x1000_0000u32]);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE)[1],
            vec![MEMORY_MANAGER, u32::MAX]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(calls(&e, VECTOR_CONSTRUCT_00401050).len(), 1);
    }

    #[test]
    fn particle_create_constant_maps_makes_both_maps_once() {
        let mut e = map_rig();
        for page in [0x0120_0000u32, 0x011f_9000] {
            e.map(page, 0x1000);
        }
        e.set_global::<i32>(SHADER_MODEL_011F91C0, 1);
        let this = e.mem.alloc(0x12c);
        e.mem.set_u32(this + 0x18, 0x0e50_0000);
        e.mem.set_u32(this + 0x70, 0x0e56_0000);
        e.call(0x00ba_69f0, &args![this]);
        let vertex = e.mem.u32(this + 0x34);
        let pixel = e.mem.u32(this + 0x30);
        assert_eq!(map_made_with(&e, vertex), (0x0e50_0000, 0));
        assert_eq!(map_made_with(&e, pixel), (0x0e50_0000, 2));
        assert_eq!(e.mem.u32(vertex + 4), 1);
        assert_eq!(
            predefined(&e, vertex),
            vec![vec![
                NAME_WORLD_VIEW_PROJ_TRANSPOSE_010A1E50,
                0x2000_0009,
                0,
                EMPTY_STRING_01011584,
                0,
                0,
                0,
                0
            ]]
        );
        let added = entries(&e, vertex);
        assert_eq!(added.len(), 9);
        assert_eq!(
            added[0],
            vec![
                NAME_WIND_010B8234,
                0x1000_0007,
                0,
                0xf,
                1,
                EMPTY_STRING_01011584,
                0x10,
                4,
                this + 0xec,
                0
            ]
        );
        assert_eq!(added[6][8], SCALE_ADJUSTMENT_011ADCCC);
        assert_eq!(
            added[7][3..9],
            [4, 5, EMPTY_STRING_01011584, 0x50, 4, this + 0x84]
        );
        assert_eq!(e.global::<u32>(PARTICLE_COUNT_012003EC), 0x1a);
        assert_eq!(
            added[8],
            vec![
                NAME_PARTICLE_DATA_010B81EC,
                0x1000_0009,
                0,
                0x12,
                0x4e,
                EMPTY_STRING_01011584,
                0x4e0,
                4,
                0x0e56_0000,
                0
            ]
        );
        // Both maps there: nothing more.
        e.call(0x00ba_69f0, &args![this]);
        assert_eq!(calls(&e, CONSTANT_MAP_CONSTRUCT).len(), 2);
        // A set count is kept; a model from 2 gives 0x4e.
        let mut e = map_rig();
        for page in [0x0120_0000u32, 0x011f_9000] {
            e.map(page, 0x1000);
        }
        e.set_global::<i32>(SHADER_MODEL_011F91C0, 2);
        let this = e.mem.alloc(0x12c);
        let held = object(&mut e, 1);
        e.mem.set_u32(this + 0x30, held);
        e.call(0x00ba_69f0, &args![this]);
        assert_eq!(e.global::<u32>(PARTICLE_COUNT_012003EC), 0x4e);
        assert_eq!(calls(&e, CONSTANT_MAP_CONSTRUCT).len(), 1);
    }
}

#[cfg(test)]
mod n070_tests {
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

    /// Slot doubles: `SLOTS + offset` answers `0x900 + offset`.
    const SLOTS: u32 = 0x0e80_0000;

    fn rig() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011f_9000u32,
            0x0120_0000,
            0x0126_f000,
            0x0101_2000,
            0x0102_3000,
        ] {
            e.map(page, 0x1000);
        }
        for offset in (0..0x100u32).step_by(4) {
            e.register(SLOTS + offset, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// An object whose vtable's every slot is `SLOTS + offset`.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for offset in (0..0x100u32).step_by(4) {
            e.mem.set_u32(vtable + offset, SLOTS + offset);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A geometry whose shader property has the type `kind`.
    fn make_geometry(e: &mut Engine, kind: u32) -> (u32, u32) {
        let property = e.mem.alloc(0x1c0);
        e.mem.set_u32(property + 0x1c, kind);
        let geometry = e.mem.alloc(0x10);
        e.mem.set_u32(geometry + 0xc, property);
        (geometry, property)
    }

    #[test]
    fn fn_00ba5de0_picks_the_blend_word() {
        let mut e = rig();
        e.register(RENDER_STATE_00B97E80, |_, _| Ret::default());
        let (geometry, property) = make_geometry(&mut e, 0x11);
        for (index, word) in [(1u32, 7u32), (3, 2), (8, 0)] {
            e.mem.set_u32(property + 0x114, index);
            e.call(0x00ba_5de0, &args![0u32, geometry, 0u32]);
            assert_eq!(
                calls(&e, RENDER_STATE_00B97E80).last().unwrap(),
                &vec![word, 0]
            );
        }
    }

    #[test]
    fn fn_00ba5e60_sets_the_blend_states() {
        let mut e = rig();
        e.register(RENDER_STATE_00B97FA0, |_, _| Ret::default());
        e.register(RENDER_STATE_00B97FF0, |_, _| Ret::default());
        e.register(INDEX_MAP_00BA5060, |_, a| (a[0] * 10).into_ret());
        let (geometry, property) = make_geometry(&mut e, 0x11);
        e.mem.set_u32(property + 0x108, 1);
        e.mem.set_u32(property + 0x10c, 2);
        e.call(0x00ba_5e60, &args![0u32, geometry]);
        assert_eq!(calls(&e, RENDER_STATE_00B97FA0), vec![vec![1, 0]]);
        assert_eq!(calls(&e, RENDER_STATE_00B97FF0), vec![vec![10, 20, 0]]);
        // Another property type: the defaults.
        let (geometry, _) = make_geometry(&mut e, 0x10);
        e.call(0x00ba_5e60, &args![0u32, geometry]);
        assert_eq!(calls(&e, RENDER_STATE_00B97FF0)[1], vec![6, 7, 0]);
    }

    #[test]
    fn fn_00ba5ed0_sets_the_aspect_constants() {
        let mut e = rig();
        e.register(ASPECT_CONSTANTS_00BA5320, |_, _| Ret::default());
        e.call(0x00ba_5ed0, &args![0u32, 0u32]);
        assert_eq!(
            calls(&e, ASPECT_CONSTANTS_00BA5320),
            vec![Vec::<u32>::new()]
        );
    }

    /// `00ba5ee0`'s rig: the accumulator (+0x8c.. 1.0, 2.0, 3.0; +0xbc, +0xb8 -> +0x34), the
    /// shader with both constant maps, the current pass with shaders.
    fn constants_rig() -> (Engine, u32, u32, u32) {
        let mut e = rig();
        let accumulator = e.mem.alloc(0xc0);
        for (index, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(accumulator + 0x8c + 4 * index as u32, *value);
        }
        let inner = e.mem.alloc(0x40);
        e.mem.set_u32(inner + 0x34, 0x3434);
        e.mem.set_u32(accumulator + 0xb8, inner);
        e.mem.set_u32(accumulator + 0xbc, 0xbcbc);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, accumulator);
        e.set_global(ACCUMULATOR_011F91E0, holder);
        let pass = e.mem.alloc(0x60);
        e.mem.set_u32(pass + 0x44, 0x4444);
        e.mem.set_u32(pass + 0x5c, 0x5c5c);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        e.register(FN_00E6FBB0, |_, _| Ret::default());
        e.register(MEMCPY_00EC44D0, |_, _| Ret::default());
        e.register(INVERT_NON_UNIFORM_004B4880, |_, _| Ret::default());
        e.register(TRANSFORM_POINT_00524C40, |e, a| {
            for index in 0..3 {
                let value = e.mem.f32(a[2] + 4 * index) * 2.0;
                e.mem.set_f32(a[1] + 4 * index, value);
            }
            a[1].into_ret()
        });
        let this = object(&mut e, 0x130);
        e.mem.set_u32(this + 0x18, 0x1818);
        e.mem.set_u32(this + 0x70, 0x7070);
        let pixel = object(&mut e, 4);
        let vertex = object(&mut e, 4);
        e.mem.set_u32(this + 0x30, pixel);
        e.mem.set_u32(this + 0x34, vertex);
        (e, this, accumulator, pixel)
    }

    #[test]
    fn fn_00ba5ee0_fills_the_constants_from_the_property() {
        let (mut e, this, accumulator, pixel) = constants_rig();
        let (geometry, property) = make_geometry(&mut e, 0x11);
        for index in 0..0x54u32 {
            e.mem.set_f32(property + 0x60 + 4 * index, index as f32);
        }
        e.mem.set_u32(property + 0x60, 0x6060);
        e.mem.set_u32(property + 0x104, 0x7f80_0001);
        e.mem.set_f32(property + 0x134, 4.0);
        e.mem.set_f32(property + 0x138, 0.5);
        e.set_global(PARTICLE_COUNT_012003EC, 2u32);
        e.call(0x00ba_5ee0, &args![this, geometry]);
        assert_eq!(
            calls(&e, FN_00E6FBB0),
            vec![vec![0x1818, accumulator + 0x68, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x84), 0x7fc0_0001);
        // Raw words +0xd4 from +0x90 ((0x90 - 0x60) / 4 = 12).
        assert_eq!(e.mem.f32(this + 0xd4), 12.0);
        // The point +0x13c (index 0x37) moved through the transform (doubled here).
        assert_eq!(e.mem.f32(this + 0xec), 2.0 * 55.0);
        assert_eq!(
            calls(&e, INVERT_NON_UNIFORM_004B4880)[0][0],
            accumulator + 0x68
        );
        assert_eq!(e.mem.f32(this + 0xb4), 0.0);
        assert_eq!(e.mem.f32(this + 0xc4), 0.25);
        assert_eq!(e.mem.f32(this + 0xc8), 2.0);
        assert_eq!(calls(&e, MEMCPY_00EC44D0), vec![vec![0x7070, 0x6060, 0x60]]);
        let fed = calls(&e, SLOTS + 0x78);
        assert_eq!(fed.len(), 2);
        assert_eq!(
            fed[0],
            vec![
                pixel,
                0x4444,
                accumulator,
                0xbcbc,
                0,
                0x3434,
                geometry,
                0,
                accumulator + 0x68,
                FALLBACK_011F4288,
                0,
                1
            ]
        );
        // With the byte +0x6c set the point is kept and +0xb4 comes from the accumulator.
        e.mem.set_u8(property + 0x6c, 1);
        e.call(0x00ba_5ee0, &args![this, geometry]);
        assert_eq!(e.mem.f32(this + 0xec), 55.0);
        assert_eq!(e.mem.f32(this + 0xb4), 1.0);
        assert_eq!(e.mem.f32(this + 0xbc), 3.0);
        assert_eq!(calls(&e, INVERT_NON_UNIFORM_004B4880).len(), 1);
    }

    #[test]
    fn fn_00ba5ee0_without_a_particle_property_only_feeds_the_maps() {
        let (mut e, this, _, _) = constants_rig();
        let (geometry, _) = make_geometry(&mut e, 3);
        let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
        e.mem.set_u32(pass + 0x44, 0);
        e.call(0x00ba_5ee0, &args![this, geometry]);
        assert!(calls(&e, MEMCPY_00EC44D0).is_empty());
        assert_eq!(calls(&e, SLOTS + 0x78).len(), 1);
    }

    #[test]
    fn fn_00ba6240_sets_the_stage_texture() {
        let mut e = rig();
        e.register(STAGE_SET_00E7DE90, |_, _| Ret::default());
        e.register(STAGE_SET_00E7EB00, |_, _| Ret::default());
        let stage = e.mem.alloc(0x10);
        let stages = e.mem.alloc(4);
        e.mem.set_u32(stages, stage);
        let pass = e.mem.alloc(0x28);
        e.mem.set_u32(pass + 0x24, stages);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        let (geometry, property) = make_geometry(&mut e, 0x11);
        e.mem.set_u32(property + 0x118, 0x1180);
        e.call(0x00ba_6240, &args![0u32, geometry]);
        assert_eq!(e.mem.u32(stage + 8), 0x1180);
        assert_eq!(calls(&e, STAGE_SET_00E7DE90), vec![vec![stage, 3]]);
        assert_eq!(calls(&e, STAGE_SET_00E7EB00), vec![vec![stage, 0]]);
        let (geometry, _) = make_geometry(&mut e, 0x12);
        e.call(0x00ba_6240, &args![0u32, geometry]);
        assert_eq!(calls(&e, STAGE_SET_00E7DE90).len(), 1);
    }

    fn interlocked(e: &mut Engine) {
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
    }

    #[test]
    fn stop_removes_the_property_children_and_releases_the_geometry() {
        let mut e = rig();
        interlocked(&mut e);
        let (_, wanted) = make_geometry(&mut e, 0x11);
        let (_, other) = make_geometry(&mut e, 0x11);
        let children = e.mem.alloc(12);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first + 4, wanted);
        e.mem.set_u32(second + 4, other);
        e.mem.set_u32(children, first);
        e.mem.set_u32(children + 8, second);
        e.register(GET_PROPERTY_00A59D30, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let node = object(&mut e, 0xb0);
        e.mem.set_u32(node + 0xa0, children);
        e.mem.set_u16(node + 0xa6, 3);
        e.register(SLOTS + 0xc, |e, _| e.mem.u32(0x0120_0f00).into_ret());
        e.set_global(0x0120_0f00, node);
        let owner = object(&mut e, 4);
        let shared = object(&mut e, 8);
        e.mem.set_u32(shared + 4, 1);
        e.set_global(SHARED_POINTER_012003F4, shared);
        e.set_global(STARTED_012003F0, 2u32);
        e.call(0x00ba_6290, &args![owner, wanted]);
        assert_eq!(calls(&e, SLOTS + 0xf0), vec![vec![node, 0]]);
        assert_eq!(e.global::<u32>(STARTED_012003F0), 1);
        assert_eq!(e.global::<u32>(SHARED_POINTER_012003F4), shared);
        e.call(0x00ba_6290, &args![owner, wanted]);
        assert_eq!(e.global::<u32>(STARTED_012003F0), 0);
        assert_eq!(e.global::<u32>(SHARED_POINTER_012003F4), 0);
        assert_eq!(calls(&e, SLOTS + 4), vec![vec![shared]]);
        // No property: nothing at all.
        e.set_global(STARTED_012003F0, 5u32);
        e.call(0x00ba_6290, &args![owner, 0u32]);
        assert_eq!(e.global::<u32>(STARTED_012003F0), 5);
        e.call(0x00ba_6290, &args![0u32, wanted]);
        assert_eq!(e.global::<u32>(STARTED_012003F0), 5);
    }

    /// The geometry builder's callees doubled: constructors answer their block, the
    /// additional data's base and vector constructors do nothing.
    fn geometry_rig() -> Engine {
        let mut e = rig();
        interlocked(&mut e);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        for target in [
            TRI_SHAPE_DATA_00A7B630,
            PROPERTY_CONSTRUCT_00BA7F70,
            NI_OBJECT_CONSTRUCT,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.register(TRI_SHAPE_00A74480, |e, a| {
            let bound = e.mem.alloc(0x20);
            e.mem.set_u32(bound + 0x10, 0x1010);
            e.mem.set_u32(a[0] + 0xb8, bound);
            a[0].into_ret()
        });
        e.register(GET_INSTANCE_INDEX_00BA73A0, |_, _| 0x7300u32.into_ret());
        for target in [
            VECTOR_CONSTRUCT_00401050,
            SET_DATA_BLOCK_COUNT_00A72CA0,
            SET_DATA_BLOCK_00A72BB0,
            SET_DATA_STREAM_00A726A0,
            SET_ADDITIONAL_00A67260,
            ATTACH_PROPERTY_00439410,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.set_global(MINUS_ONE_01012054, -1.0f32);
        e.set_global(RADIUS_010231A0, 512.0f32);
        e.set_global(SHADER_MODEL_011F91C0, 1u32);
        e
    }

    #[test]
    fn get_particle_geometry_builds_the_shared_quads() {
        let mut e = geometry_rig();
        e.set_global(PARTICLE_COUNT_012003EC, 2u32);
        let shape = e.call(0x00ba_64e0, &args![]).u32();
        assert_eq!(e.global::<u32>(SHARED_POINTER_012003F4), shape);
        assert_eq!(e.mem.u32(shape + 4), 1);
        let sizes: Vec<u32> = calls(&e, MEMORY_ALLOCATE)
            .iter()
            .map(|a| a[1])
            .take(3)
            .collect();
        assert_eq!(sizes, vec![24, 96, 64]);
        let data = &calls(&e, TRI_SHAPE_DATA_00A7B630)[0];
        assert_eq!(data[1], 8);
        assert_eq!(data[3..5], [0, 0]);
        assert_eq!(data[6..8], [1, 0]);
        assert_eq!(data[8], 4);
        let (positions, indices) = (data[2], data[9]);
        let floats: Vec<f32> = (0..12).map(|i| e.mem.f32(positions + 4 * i)).collect();
        assert_eq!(
            floats,
            vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0]
        );
        let words: Vec<u16> = (0..12).map(|i| e.mem.u16(indices + 2 * i)).collect();
        assert_eq!(words, vec![0, 1, 2, 3, 0, 2, 4, 5, 6, 7, 4, 6]);
        let uv_block = data[5];
        let uv: Vec<f32> = (0..8).map(|i| e.mem.f32(uv_block + 4 * i)).collect();
        assert_eq!(uv, vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0]);
        assert_eq!(
            calls(&e, SET_DATA_BLOCK_00A72BB0)[0][1..],
            [0, 0x7300, 32, 1]
        );
        assert_eq!(
            calls(&e, SET_DATA_STREAM_00A726A0)[0][1..],
            [0, 0, 0, 1, 8, 4, 4]
        );
        let bound = e.mem.u32(shape + 0xb8);
        assert_eq!(e.mem.f32(bound + 0x1c), 512.0);
        assert_eq!(e.mem.u32(bound + 0x10), 0x1010);
        // Asked again: the shared one.
        assert_eq!(e.call(0x00ba_64e0, &args![]).u32(), shape);
        assert_eq!(calls(&e, TRI_SHAPE_00A74480).len(), 1);
        // Shader model 0: none.
        e.set_global(SHADER_MODEL_011F91C0, 0u32);
        assert_eq!(e.call(0x00ba_64e0, &args![]).u32(), 0);
    }

    #[test]
    fn start_clones_the_geometry_and_counts_it() {
        let mut e = geometry_rig();
        e.set_global(PARTICLE_COUNT_012003EC, 1u32);
        let node = object(&mut e, 0xb0);
        e.register(SLOTS + 0xc, |e, _| e.mem.u32(0x0120_0f00).into_ret());
        e.set_global(0x0120_0f00, node);
        let owner = object(&mut e, 4);
        let clone = object(&mut e, 0xc0);
        let bound = e.mem.alloc(0x20);
        e.mem.set_f32(bound + 0x1c, 10.0);
        e.mem.set_u32(bound + 0x18, 0x1818);
        e.mem.set_u32(clone + 0xb8, bound);
        e.set_global(0x0120_0f04, clone);
        e.register(CLONE_00A5D680, |e, _| e.mem.u32(0x0120_0f04).into_ret());
        e.register(UPDATE_00A59C60, |_, _| Ret::default());
        e.register(PREPARE_OBJECT_00B57E30, |_, _| Ret::default());
        let property = object(&mut e, 0x150);
        e.mem.set_u32(property + 0x1c, 0x11);
        e.set_global(0x0120_0f08, property);
        e.register(GET_PROPERTY_00A59D30, |e, _| {
            e.mem.u32(0x0120_0f08).into_ret()
        });
        let camera = e.mem.alloc(0x10);
        e.mem.set_f32(camera + 0xc, 8.0);
        let holder = e.mem.alloc(0x24);
        e.mem.set_u32(holder + 0x20, camera);
        assert_eq!(e.call(0x00ba_68d0, &args![owner, holder]).u32(), property);
        // 2 * 8 = 16 is not below 10: 16.
        assert_eq!(e.mem.f32(bound + 0x1c), 16.0);
        assert_eq!(e.mem.u32(bound + 0x18), 0x1818);
        assert_eq!(calls(&e, SLOTS + 0xdc), vec![vec![node, clone, 1]]);
        assert_eq!(calls(&e, UPDATE_00A59C60)[0][0], node);
        assert_eq!(calls(&e, PREPARE_OBJECT_00B57E30), vec![vec![clone, 0, 0]]);
        assert_eq!(calls(&e, SLOTS + 0x9c), vec![vec![property, clone]]);
        assert_eq!(e.global::<u32>(STARTED_012003F0), 1);
        // A bigger radius stays; a property of another type: none answered, no count.
        e.mem.set_f32(bound + 0x1c, 40.0);
        e.mem.set_u32(property + 0x1c, 0x10);
        assert_eq!(e.call(0x00ba_68d0, &args![owner, holder]).u32(), 0);
        assert_eq!(e.mem.f32(bound + 0x1c), 40.0);
        assert_eq!(e.global::<u32>(STARTED_012003F0), 1);
        // Without an owner: null.
        assert_eq!(e.call(0x00ba_68d0, &args![0u32, holder]).u32(), 0);
    }
}
