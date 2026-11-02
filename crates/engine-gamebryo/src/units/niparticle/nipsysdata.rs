//! `NiParticle/nipsysdata.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a8a9b0`: the jump target of `00a75f00`.
const JUMP_TARGET_00A75F00: u32 = 0x00a8_a9b0;

/// `00a8a9a0`: the jump target of `00a7b7d0`.
const JUMP_TARGET_00A7B7D0: u32 = 0x00a8_a9a0;

// Translated from 00a75f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a8a9b0` (engine map: `NiLinesData::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiPSysData@@UAA_NAAVNiStream@@@Z`.
pub fn ni_p_sys_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A75F00, &args![this, stream]).bool()
}

// Translated from 00a7b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::LinkObject` (Xbox PDB): a 5-byte jump to `00a8a9a0` (engine map: `NiLinesData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiPSysData@@UAAXAAVNiStream@@@Z`.
pub fn ni_p_sys_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A7B7D0, &args![this, stream]);
}

// Translated from 00c24e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2558.
pub fn ni_p_sys_data_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2558
}

use crate::types::NiTArray;
use crate::units::niparticle::nipsysmodifier::stream_read;
use crate::units::platform::MEMORY_MANAGER;

layout! {
    /// `NiPSysData` (Xbox PDB), 0x7c bytes; the PC code uses the same
    /// offsets. The fields before +0x40 are `NiGeometryData`'s and those
    /// from +0x40 `NiParticlesData`'s (both Xbox PDB); they are listed here
    /// as the particle units use them. Float fields are declared as their
    /// bit patterns (the code copies them through the x87 stack or by
    /// `MOV`).
    pub struct NiPSysData: 0x7c {
        /// `NiGeometryData::m_usVertices` (Xbox PDB): the particle capacity.
        0x08 m_usVertices: u16,
        /// `NiGeometryData::m_kBound` (Xbox PDB): centre (+0x10..+0x18) and
        /// radius (+0x1c).
        0x10 m_kBound: u32,
        0x14 m_kBoundCenterY: u32,
        0x18 m_kBoundCenterZ: u32,
        0x1C m_kBoundRadius: u32,
        /// `NiGeometryData::m_pkVertex` (Xbox PDB): `NiPoint3*`.
        0x20 m_pkVertex: u32,
        /// `NiGeometryData::m_pkNormal` (Xbox PDB): `NiPoint3*`.
        0x24 m_pkNormal: u32,
        /// `NiGeometryData::m_pkColor` (Xbox PDB): `NiColorA*`.
        0x28 m_pkColor: u32,
        /// `NiParticlesData::m_pfRadii` (Xbox PDB).
        0x4C m_pfRadii: u32,
        /// `NiParticlesData::m_usActive` (Xbox PDB).
        0x50 m_usActive: u16,
        /// `NiParticlesData::m_pfSizes` (Xbox PDB).
        0x54 m_pfSizes: u32,
        /// `NiParticlesData::m_pkRotations` (Xbox PDB): `NiQuaternion*`.
        0x58 m_pkRotations: u32,
        /// `NiParticlesData::m_pfRotationAngles` (Xbox PDB).
        0x5C m_pfRotationAngles: u32,
        /// `NiParticlesData::m_pkRotationAxes` (Xbox PDB): `NiPoint3*`.
        0x60 m_pkRotationAxes: u32,
        /// `NiParticlesData::m_pucTextureIndices` (Xbox PDB).
        0x64 m_pucTextureIndices: u32,
        /// `NiParticlesData::m_pkSubTextureOffsets` (Xbox PDB): `NiColorA*`.
        0x68 m_pkSubTextureOffsets: u32,
        /// `NiParticlesData::m_ucSubTextureOffsetCount` (Xbox PDB).
        0x6C m_ucSubTextureOffsetCount: u8,
        /// `m_pkParticleInfo` (Xbox PDB): `NiParticleInfo*` (0x1c bytes
        /// each).
        0x70 m_pkParticleInfo: u32,
        /// `m_pfRotationSpeeds` (Xbox PDB).
        0x74 m_pfRotationSpeeds: u32,
        /// `m_usNumAddedParticles` (Xbox PDB).
        0x78 m_usNumAddedParticles: u16,
        /// `m_usAddedParticlesBase` (Xbox PDB).
        0x7A m_usAddedParticlesBase: u16,
    }
}

/// The vtable of `NiPSysData` (RTTI `.?AVNiPSysData@@`).
const VTABLE: u32 = 0x010b_e254;
/// `NiParticleInfo`'s size.
const PARTICLE_INFO_SIZE: u32 = 0x1c;
/// `NiParticleInfo`'s constructor (`00c31e50`, `thiscall`).
const PARTICLE_INFO_CONSTRUCTOR: u32 = 0x00c3_1e50;
/// `NiParticleInfo::LoadBinary` (`00c31f10`, `thiscall`, the stream).
const PARTICLE_INFO_LOAD_BINARY: u32 = 0x00c3_1f10;
/// `MemoryManager::Allocate` / `Deallocate` (`thiscall` on the memory
/// manager at `011f6238`, one word).
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;
/// `00aa1070` / `00aa10f0` (unnamed, `cdecl`, one word): the array
/// allocation and free the particle arrays use.
const ARRAY_ALLOCATE: u32 = 0x00aa_1070;
const ARRAY_FREE: u32 = 0x00aa_10f0;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `00aa1460` (unnamed, `cdecl`, `(block, size)`): the sized object free.
const NI_DELETE: u32 = 0x00aa_1460;
/// `_memcpy` (`00ec44d0`) and `_memset` (`00ec61c0`), `cdecl`.
const MEMCPY: u32 = 0x00ec_44d0;
const MEMSET: u32 = 0x00ec_61c0;
/// `00401050` (unnamed; `stdcall`, `RET 0x10`): the vector constructor
/// iterator `(array, element size, count, constructor)`.
const VECTOR_CONSTRUCTOR: u32 = 0x0040_1050;
/// `004a7800` (unnamed): the `NiColorA` element constructor the colour copy
/// hands the vector constructor iterator.
const COLOR_CONSTRUCTOR: u32 = 0x004a_7800;
/// `NiParticlesData`'s constructor (`00a963f0`, `thiscall`).
const PARTICLES_DATA_CONSTRUCTOR: u32 = 0x00a9_63f0;
/// `NiParticlesData`'s destructor body (`00a96800`, `thiscall`).
const PARTICLES_DATA_DESTRUCTOR: u32 = 0x00a9_6800;
/// `NiParticlesData::RemoveParticle` (`00a964c0`, one word).
const PARTICLES_DATA_REMOVE_PARTICLE: u32 = 0x00a9_64c0;
/// `NiParticlesData::LoadBinary` (`00a96cf0`, one word).
const PARTICLES_DATA_LOAD_BINARY: u32 = 0x00a9_6cf0;
/// `NiObject::CopyMembers` (`00a5d450`, `(dest, process)`).
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00a5_d450;
/// `00a670c0` (unnamed, `NiMain/nigeometrydata.cpp`; `thiscall`, `RET
/// 0x1c`): replaces the geometry data's arrays `(vertex count, vertices,
/// normals, colours, texture coordinates, ...)`.
const GEOMETRY_DATA_REPLACE: u32 = 0x00a6_70c0;
/// The `NiParticlesData` array setters (`thiscall`, one word; each frees
/// the old array): sizes (`00a96460`), radii (`00a96440`), rotation angles
/// (`00a96480`), texture indices (`00a964a0`), and the rotation axes
/// (`00a968c0`).
const SET_SIZES: u32 = 0x00a9_6460;
const SET_RADII: u32 = 0x00a9_6440;
const SET_ROTATION_ANGLES: u32 = 0x00a9_6480;
const SET_TEXTURE_INDICES: u32 = 0x00a9_64a0;
const SET_ROTATION_AXES: u32 = 0x00a9_68c0;
/// `NiQuaternion::ToAngleAxis` (`00a6de50`, `thiscall` on the quaternion,
/// `(&angle, &axis)`).
const QUATERNION_TO_ANGLE_AXIS: u32 = 0x00a6_de50;
/// `AddParticle`'s answer when no particle is free.
const INVALID_PARTICLE: u16 = 0xffff;
/// File versions: before 10.3.0.5 rotations are quaternions (converted to
/// angle and axis); from 20.0.0.2 the rotation speeds are stored.
const VERSION_ANGLE_AXIS: u32 = 0x0a03_0005;
const VERSION_ROTATION_SPEEDS: u32 = 0x1400_0002;
/// `NiStream`'s `m_BSStreamHeader.uiVersion` (+4): the Bethesda stream
/// version; from 19 the particle info, the rotation speeds and the added
/// particle counts are not stored.
const STREAM_BS_VERSION: u32 = 0x4;
const BS_VERSION_NO_PARTICLE_INFO: u32 = 0x13;

/// `MemoryManager::Allocate` of `count * size` bytes (all ones when the
/// product overflows, as the code's `SETO`/`NEG`/`OR`).
fn allocate(e: &mut Engine, count: u32, size: u32) -> u32 {
    let bytes = count.checked_mul(size).unwrap_or(u32::MAX);
    e.call(ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32()
}

fn memcpy(e: &mut Engine, dest: u32, source: u32, size: u32) {
    e.call(MEMCPY, &args![dest, source, size]);
}

// Translated from 00a923a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::GetActiveVertexCount` (Xbox PDB): the active particle
/// count.
pub fn ni_particles_data_get_active_vertex_count(e: &mut Engine, this: Ptr<NiPSysData>) -> u16 {
    e.get(this, NiPSysData::m_usActive)
}

// Translated from 00a968a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: replaces the rotation quaternions
/// (`m_pkRotations`), the old array given back to the memory manager.
pub fn fn_00a968a0(e: &mut Engine, this: Ptr<NiPSysData>, rotations: u32) {
    let old = e.get(this, NiPSysData::m_pkRotations);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, old]);
    e.set(this, NiPSysData::m_pkRotations, rotations);
}

// Translated from 00a968e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: replaces the sub-texture offsets and their
/// count (at most 16; a larger count is cut to 16), the old array freed
/// (`00aa10f0`).
pub fn fn_00a968e0(e: &mut Engine, this: Ptr<NiPSysData>, offsets: u32, count: u8) {
    let count = if count < 0x10 { count } else { 0x10 };
    let old = e.get(this, NiPSysData::m_pkSubTextureOffsets);
    e.call(ARRAY_FREE, &args![old]);
    e.set(this, NiPSysData::m_ucSubTextureOffsetCount, count);
    e.set(this, NiPSysData::m_pkSubTextureOffsets, offsets);
}

// Translated from 00c24cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::RemoveParticle` (Xbox PDB): `NiParticlesData`'s
/// `RemoveParticle` (with the whole argument word), then, unless `index`
/// was the last active particle (taken before that call), the last
/// particle's info (0x1c bytes, copied word by word) and rotation speed
/// (when there are rotation speeds) move into `index`.
pub fn ni_p_sys_data_remove_particle(e: &mut Engine, this: Ptr<NiPSysData>, index: u32) {
    let last = e.get(this, NiPSysData::m_usActive).wrapping_sub(1);
    e.call(PARTICLES_DATA_REMOVE_PARTICLE, &args![this, index]);
    let index = index as u16;
    if index == last {
        return;
    }
    let info = e.get(this, NiPSysData::m_pkParticleInfo);
    let from = info.wrapping_add(u32::from(last) * PARTICLE_INFO_SIZE);
    let to = info.wrapping_add(u32::from(index) * PARTICLE_INFO_SIZE);
    for word in 0..7 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
    let speeds = e.get(this, NiPSysData::m_pfRotationSpeeds);
    if speeds != 0 {
        let speed = x87_quiet_bits(e.mem.u32(speeds + 4 * u32::from(last)));
        e.mem.set_u32(speeds + 4 * u32::from(index), speed);
    }
}

// Translated from 00c24d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::ResolveAddedParticles` (Xbox PDB). When the active count is
/// below the added particles' base, the active count becomes base + added
/// and the particles from the old active count up to the base are removed
/// (vtable slot +0xa4, `RemoveParticle`) while the active count stays above
/// the base; if that stops before the base, the active count, the base and
/// the added count become that particle index and 0. Otherwise the active
/// count grows by the added count. Either way (except that early stop) the
/// added count is cleared and the base becomes the active count.
pub fn ni_p_sys_data_resolve_added_particles(e: &mut Engine, this: Ptr<NiPSysData>) {
    let active = e.get(this, NiPSysData::m_usActive);
    let base = e.get(this, NiPSysData::m_usAddedParticlesBase);
    if active < base {
        let mut index = u32::from(active);
        let added = e.get(this, NiPSysData::m_usNumAddedParticles);
        e.set(this, NiPSysData::m_usActive, added.wrapping_add(base));
        if (index as u16) < base {
            loop {
                if e.get(this, NiPSysData::m_usActive)
                    <= e.get(this, NiPSysData::m_usAddedParticlesBase)
                {
                    break;
                }
                e.vcall(this.addr(), 0xa4, &args![index]);
                index = index.wrapping_add(1);
                if (index as u16) >= e.get(this, NiPSysData::m_usAddedParticlesBase) {
                    break;
                }
            }
        }
        if (index as u16) < e.get(this, NiPSysData::m_usAddedParticlesBase) {
            e.set(this, NiPSysData::m_usActive, index as u16);
            e.set(this, NiPSysData::m_usNumAddedParticles, 0);
            e.set(this, NiPSysData::m_usAddedParticlesBase, index as u16);
            return;
        }
    } else {
        let added = e.get(this, NiPSysData::m_usNumAddedParticles);
        e.set(this, NiPSysData::m_usActive, added.wrapping_add(active));
    }
    let active = e.get(this, NiPSysData::m_usActive);
    e.set(this, NiPSysData::m_usNumAddedParticles, 0);
    e.set(this, NiPSysData::m_usAddedParticlesBase, active);
}

/// The `NiPSysData` part of its constructor the code inlines after
/// `NiParticlesData`'s: the vtable, no particle info, no rotation speeds,
/// no added particles.
fn construct_fields(e: &mut Engine, data: Ptr<NiPSysData>) {
    e.set(data, NiPSysData::m_usAddedParticlesBase, 0);
    e.mem.set_u32(data.addr(), VTABLE);
    e.set(data, NiPSysData::m_pkParticleInfo, 0);
    e.set(data, NiPSysData::m_pfRotationSpeeds, 0);
    e.set(data, NiPSysData::m_usNumAddedParticles, 0);
}

// Translated from 00c24da0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Not a function in the Ghidra project (registered through `push
/// 00c1aae9`): the `NiPSysData` stream factory (`cdecl`, no arguments). A
/// 0x7c-byte object from `00aa13e0`, constructed (`NiParticlesData`'s
/// constructor, then the vtable `010be254` and cleared particle info,
/// rotation speeds and added particles); 0 when the allocation fails.
pub fn fn_00c24da0(e: &mut Engine) -> Ptr<NiPSysData> {
    let data = Ptr::<NiPSysData>::new(e.call(NI_NEW, &args![0x7cu32]).u32());
    if data.is_null() {
        return Ptr::NULL;
    }
    e.call(PARTICLES_DATA_CONSTRUCTOR, &args![data]);
    e.set(data, NiPSysData::m_usNumAddedParticles, 0);
    e.mem.set_u32(data.addr(), VTABLE);
    e.set(data, NiPSysData::m_pkParticleInfo, 0);
    e.set(data, NiPSysData::m_pfRotationSpeeds, 0);
    e.set(data, NiPSysData::m_usAddedParticlesBase, 0);
    data
}

// Translated from 00c24ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::AddParticle` (Xbox PDB): with no particles added yet, the
/// base becomes the active count and, when that is below the capacity, the
/// added count becomes 1 and the base is the new particle; otherwise, when
/// base + added is below the capacity, the added count grows and the new
/// particle is base + the old added count. `0xffff` when there is no room.
pub fn ni_p_sys_data_add_particle(e: &mut Engine, this: Ptr<NiPSysData>) -> u16 {
    let added = e.get(this, NiPSysData::m_usNumAddedParticles);
    let capacity = e.get(this, NiPSysData::m_usVertices);
    if added == 0 {
        let active = e.get(this, NiPSysData::m_usActive);
        e.set(this, NiPSysData::m_usAddedParticlesBase, active);
        if active < capacity {
            e.set(this, NiPSysData::m_usNumAddedParticles, 1);
            return active;
        }
        return INVALID_PARTICLE;
    }
    let base = e.get(this, NiPSysData::m_usAddedParticlesBase);
    if (u32::from(base) + u32::from(added)) < u32::from(capacity) {
        e.set(
            this,
            NiPSysData::m_usNumAddedParticles,
            added.wrapping_add(1),
        );
        return base.wrapping_add(added);
    }
    INVALID_PARTICLE
}

// Translated from 00c24f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysData` member copy into `dest`
/// (`thiscall`, `RET 8`: `dest`, cloning process). `NiObject::CopyMembers`,
/// then copies of every particle array sized by `this`'s capacity: the
/// vertices, normals (when present) and colours (when present; their array
/// constructed element by element first, `004a7800`) go to `dest` through
/// `00a670c0` (no texture coordinates); the rotations (when present, else
/// none) through `00a968a0`; sizes and radii (always copied, from the
/// arrays `00aa1070` makes); the active count; the rotation angles, axes,
/// texture indices and sub-texture offsets when present; the particle info
/// (constructed element by element, `00c31e50`) and rotation speeds replace
/// `dest`'s (the old ones given back); the added counts and the bound.
pub fn fn_00c24f20(e: &mut Engine, this: Ptr<NiPSysData>, dest: Ptr<NiPSysData>, process: Ptr) {
    e.call(NI_OBJECT_COPY_MEMBERS, &args![this, dest, process]);
    let capacity = |e: &Engine| u32::from(e.get(this, NiPSysData::m_usVertices));

    let n = capacity(e);
    let vertices = allocate(e, n, 0xc);
    let point_bytes = capacity(e) * 0xc;
    let source = e.get(this, NiPSysData::m_pkVertex);
    memcpy(e, vertices, source, point_bytes);

    let mut normals = 0;
    if e.get(this, NiPSysData::m_pkNormal) != 0 {
        let n = capacity(e);
        normals = allocate(e, n, 0xc);
        let source = e.get(this, NiPSysData::m_pkNormal);
        memcpy(e, normals, source, point_bytes);
    }

    let mut colors = 0;
    if e.get(this, NiPSysData::m_pkColor) != 0 {
        let n = capacity(e);
        colors = allocate(e, n, 0x10);
        if colors != 0 {
            e.call(
                VECTOR_CONSTRUCTOR,
                &args![colors, 0x10u32, n, COLOR_CONSTRUCTOR],
            );
        }
        let bytes = capacity(e) << 4;
        let source = e.get(this, NiPSysData::m_pkColor);
        memcpy(e, colors, source, bytes);
    }

    let n = capacity(e);
    e.call(
        GEOMETRY_DATA_REPLACE,
        &args![dest, n, vertices, normals, colors, 0u32, 0u32, 0u32],
    );

    let mut rotations = 0;
    if e.get(this, NiPSysData::m_pkRotations) != 0 {
        let n = capacity(e);
        rotations = allocate(e, n, 0x10);
        let source = e.get(this, NiPSysData::m_pkRotations);
        let bytes = capacity(e) << 4;
        memcpy(e, rotations, source, bytes);
    }
    fn_00a968a0(e, dest, rotations);

    let bytes = capacity(e) * 4;
    let sizes = e.call(ARRAY_ALLOCATE, &args![bytes]).u32();
    let float_bytes = capacity(e) * 4;
    let source = e.get(this, NiPSysData::m_pfSizes);
    memcpy(e, sizes, source, float_bytes);
    e.call(SET_SIZES, &args![dest, sizes]);

    let bytes = capacity(e) * 4;
    let radii = e.call(ARRAY_ALLOCATE, &args![bytes]).u32();
    let source = e.get(this, NiPSysData::m_pfRadii);
    memcpy(e, radii, source, float_bytes);
    e.call(SET_RADII, &args![dest, radii]);

    let active = e.get(this, NiPSysData::m_usActive);
    e.set(dest, NiPSysData::m_usActive, active);

    if e.get(this, NiPSysData::m_pfRotationAngles) != 0 {
        let bytes = capacity(e) * 4;
        let angles = e.call(ARRAY_ALLOCATE, &args![bytes]).u32();
        let source = e.get(this, NiPSysData::m_pfRotationAngles);
        memcpy(e, angles, source, float_bytes);
        e.call(SET_ROTATION_ANGLES, &args![dest, angles]);
    }

    if e.get(this, NiPSysData::m_pkRotationAxes) != 0 {
        let n = capacity(e);
        let axes = allocate(e, n, 0xc);
        let source = e.get(this, NiPSysData::m_pkRotationAxes);
        let bytes = capacity(e) * 0xc;
        memcpy(e, axes, source, bytes);
        e.call(SET_ROTATION_AXES, &args![dest, axes]);
    }

    if e.get(this, NiPSysData::m_pucTextureIndices) != 0 {
        let n = capacity(e);
        let indices = e.call(ARRAY_ALLOCATE, &args![n]).u32();
        let source = e.get(this, NiPSysData::m_pucTextureIndices);
        let bytes = capacity(e);
        memcpy(e, indices, source, bytes);
        e.call(SET_TEXTURE_INDICES, &args![dest, indices]);
    }

    if e.get(this, NiPSysData::m_pkSubTextureOffsets) != 0 {
        let bytes = u32::from(e.get(this, NiPSysData::m_ucSubTextureOffsetCount)) << 4;
        let offsets = e.call(ARRAY_ALLOCATE, &args![bytes]).u32();
        let source = e.get(this, NiPSysData::m_pkSubTextureOffsets);
        let bytes = u32::from(e.get(this, NiPSysData::m_ucSubTextureOffsetCount)) << 4;
        memcpy(e, offsets, source, bytes);
        let count = e.get(this, NiPSysData::m_ucSubTextureOffsetCount);
        fn_00a968e0(e, dest, offsets, count);
    }

    let mut info = 0;
    if e.get(this, NiPSysData::m_pkParticleInfo) != 0 {
        let n = capacity(e);
        info = allocate(e, n, PARTICLE_INFO_SIZE);
        if info != 0 {
            e.call(
                VECTOR_CONSTRUCTOR,
                &args![info, PARTICLE_INFO_SIZE, n, PARTICLE_INFO_CONSTRUCTOR],
            );
        }
        let bytes = capacity(e) * PARTICLE_INFO_SIZE;
        let source = e.get(this, NiPSysData::m_pkParticleInfo);
        memcpy(e, info, source, bytes);
    }
    let old = e.get(dest, NiPSysData::m_pkParticleInfo);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, old]);
    e.set(dest, NiPSysData::m_pkParticleInfo, info);

    let mut speeds = 0;
    if e.get(this, NiPSysData::m_pfRotationSpeeds) != 0 {
        let bytes = capacity(e) * 4;
        speeds = e.call(ARRAY_ALLOCATE, &args![bytes]).u32();
        let bytes = capacity(e) * 4;
        let source = e.get(this, NiPSysData::m_pfRotationSpeeds);
        memcpy(e, speeds, source, bytes);
    }
    let old = e.get(dest, NiPSysData::m_pfRotationSpeeds);
    e.call(ARRAY_FREE, &args![old]);
    e.set(dest, NiPSysData::m_pfRotationSpeeds, speeds);

    let added = e.get(this, NiPSysData::m_usNumAddedParticles);
    e.set(dest, NiPSysData::m_usNumAddedParticles, added);
    let base = e.get(this, NiPSysData::m_usAddedParticlesBase);
    e.set(dest, NiPSysData::m_usAddedParticlesBase, base);
    // The radius goes through the x87 stack twice (`FLD`/`FSTP` to a stack
    // word and back); the centre is copied by `MOV`.
    let radius = x87_quiet_bits(e.get(this, NiPSysData::m_kBoundRadius));
    for field in [
        NiPSysData::m_kBound,
        NiPSysData::m_kBoundCenterY,
        NiPSysData::m_kBoundCenterZ,
    ] {
        let value = e.get(this, field);
        e.set(dest, field, value);
    }
    e.set(dest, NiPSysData::m_kBoundRadius, radius);
}

// Translated from 00c25280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::_scalar_deleting_destructor_` (Xbox PDB): the vtable back to
/// `NiPSysData`'s, the particle info given back to the memory manager, the
/// rotation speeds freed (`00aa10f0`), `NiParticlesData`'s destructor body,
/// then the sized delete of 0x7c bytes when bit 0 of `flags` is set.
/// Returns `this`.
pub fn ni_p_sys_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPSysData>,
    flags: u32,
) -> Ptr<NiPSysData> {
    let info = e.get(this, NiPSysData::m_pkParticleInfo);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, info]);
    let speeds = e.get(this, NiPSysData::m_pfRotationSpeeds);
    e.call(ARRAY_FREE, &args![speeds]);
    e.call(PARTICLES_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x7cu32]);
    }
    this
}

// Translated from 00c252d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::CreateClone` (Xbox PDB): a 0x7c-byte object from
/// `00aa13e0`, constructed (`NiParticlesData`'s constructor and the
/// `NiPSysData` fields), then the member copy `00c24f20` into it; when the
/// allocation fails the member copy still runs, into a null `dest`, and 0
/// comes back.
pub fn ni_p_sys_data_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysData>,
    process: Ptr,
) -> Ptr<NiPSysData> {
    let clone = Ptr::<NiPSysData>::new(e.call(NI_NEW, &args![0x7cu32]).u32());
    if clone.is_null() {
        fn_00c24f20(e, this, Ptr::NULL, process);
        return Ptr::NULL;
    }
    e.call(PARTICLES_DATA_CONSTRUCTOR, &args![clone]);
    construct_fields(e, clone);
    fn_00c24f20(e, this, clone, process);
    clone
}

// Translated from 00c25340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::LoadBinary` (Xbox PDB): `NiParticlesData::LoadBinary`; for
/// files before 10.3.0.5 with rotations, the quaternions become angles
/// (`00aa1070`) and axes (memory manager) through
/// `NiQuaternion::ToAngleAxis` and the rotations are dropped
/// (`00a968a0(0)`). The particle info array is made (memory manager,
/// element by element `00c31e50`; 0 when the allocation fails) and, for
/// Bethesda stream versions below 19, each particle's info is read. From
/// file version 20.0.0.2 a byte says whether rotation speeds follow (they
/// are allocated, and read when the stream version is below 19; at 19 and
/// above the function ends there); older files get zeroed rotation speeds
/// when there are rotation angles. Below stream version 19 the added
/// count and base follow (2 bytes each). The byte and the sizes use the
/// stack word of the stream argument (which by then holds the particle info
/// array).
pub fn ni_p_sys_data_load_binary(e: &mut Engine, this: Ptr<NiPSysData>, stream: Ptr) {
    let stream = stream.addr();
    e.call(PARTICLES_DATA_LOAD_BINARY, &args![this, stream]);
    let version = e.mem.u32(stream + 0xd8);
    if version < VERSION_ANGLE_AXIS && e.get(this, NiPSysData::m_pkRotations) != 0 {
        let bytes = u32::from(e.get(this, NiPSysData::m_usVertices)) * 4;
        let angles = e.call(ARRAY_ALLOCATE, &args![bytes]).u32();
        e.set(this, NiPSysData::m_pfRotationAngles, angles);
        let n = u32::from(e.get(this, NiPSysData::m_usVertices));
        let axes = allocate(e, n, 0xc);
        e.set(this, NiPSysData::m_pkRotationAxes, axes);
        let mut i: u16 = 0;
        while i < e.get(this, NiPSysData::m_usVertices) {
            let k = u32::from(i);
            let axis = e.get(this, NiPSysData::m_pkRotationAxes) + k * 0xc;
            let angle = e.get(this, NiPSysData::m_pfRotationAngles) + k * 4;
            let quaternion = e.get(this, NiPSysData::m_pkRotations) + (k << 4);
            e.call(QUATERNION_TO_ANGLE_AXIS, &args![quaternion, angle, axis]);
            i = i.wrapping_add(1);
        }
        fn_00a968a0(e, this, 0);
    }

    let n = u32::from(e.get(this, NiPSysData::m_usVertices));
    let block = allocate(e, n, PARTICLE_INFO_SIZE);
    e.with_stack(8, |e, frame| {
        let size_slot = frame.addr();
        let argument_slot = frame.addr() + 4;
        e.mem.set_u32(argument_slot, block);
        let info = if block == 0 {
            0
        } else {
            let mut left = n as i32 - 1;
            let mut element = block;
            while left >= 0 {
                e.call(PARTICLE_INFO_CONSTRUCTOR, &args![element]);
                element += PARTICLE_INFO_SIZE;
                left -= 1;
            }
            e.mem.u32(argument_slot)
        };
        e.set(this, NiPSysData::m_pkParticleInfo, info);

        if e.mem.u32(stream + STREAM_BS_VERSION) < BS_VERSION_NO_PARTICLE_INFO {
            let mut i: u16 = 0;
            while i < e.get(this, NiPSysData::m_usVertices) {
                let element =
                    e.get(this, NiPSysData::m_pkParticleInfo) + u32::from(i) * PARTICLE_INFO_SIZE;
                e.call(PARTICLE_INFO_LOAD_BINARY, &args![element, stream]);
                i = i.wrapping_add(1);
            }
        }

        if e.mem.u32(stream + 0xd8) >= VERSION_ROTATION_SPEEDS {
            stream_read(e, stream, argument_slot, 1, 1, size_slot);
            if e.mem.u8(argument_slot) != 0 {
                let bytes = u32::from(e.get(this, NiPSysData::m_usVertices)) * 4;
                let speeds = e.call(ARRAY_ALLOCATE, &args![bytes]).u32();
                e.set(this, NiPSysData::m_pfRotationSpeeds, speeds);
                if e.mem.u32(stream + STREAM_BS_VERSION) >= BS_VERSION_NO_PARTICLE_INFO {
                    return;
                }
                let bytes = u32::from(e.get(this, NiPSysData::m_usVertices)) * 4;
                stream_read(e, stream, speeds, bytes, 4, size_slot);
            }
        } else if e.get(this, NiPSysData::m_pfRotationAngles) != 0 {
            let bytes = u32::from(e.get(this, NiPSysData::m_usVertices)) * 4;
            let speeds = e.call(ARRAY_ALLOCATE, &args![bytes]).u32();
            let bytes = u32::from(e.get(this, NiPSysData::m_usVertices)) * 4;
            e.set(this, NiPSysData::m_pfRotationSpeeds, speeds);
            e.call(MEMSET, &args![speeds, 0u32, bytes]);
        }

        if e.mem.u32(stream + STREAM_BS_VERSION) < BS_VERSION_NO_PARTICLE_INFO {
            let added = this.addr() + NiPSysData::m_usNumAddedParticles.off;
            stream_read(e, stream, added, 2, 2, argument_slot);
            let base = this.addr() + NiPSysData::m_usAddedParticlesBase.off;
            stream_read(e, stream, base, 2, 2, argument_slot);
        }
    });
}

/// The viewer-string array's `SetSize` (`0096ad30`, `thiscall`, one word)
/// and `SetAt` (`0096ae90`, `thiscall`: index and the address of the
/// element), as the particle units' `GetViewerStrings` call them.
pub(crate) const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
pub(crate) const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// The `NiTArray<char *>` add every `GetViewerStrings` of these units
/// inlines: the index is `m_usSize`; when it is not below `m_usMaxSize`
/// the array is resized to `m_usGrowBy` more than it (`0096ad30`), then
/// the string is stored there through a local word (`0096ae90`).
pub(crate) fn add_viewer_string(e: &mut Engine, strings: u32, string: u32) {
    let array = Ptr::<NiTArray>::new(strings);
    let index = u32::from(e.get(array, NiTArray::m_usSize));
    if index >= u32::from(e.get(array, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
        e.call(STRINGS_SET_SIZE, &args![strings, grow.wrapping_add(index)]);
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), string);
        e.call(STRINGS_SET_AT, &args![strings, index, local]);
    });
}

/// `NiStream`'s output stream (`m_pkOstr`, +0x250 as the PC code reads it),
/// and the output stream's position (+4) and write function (+0xC,
/// `cdecl` `(stream, buffer, size, &component size, component count)`,
/// returns the bytes written).
pub(crate) const STREAM_OUTPUT: u32 = 0x250;
const OUTPUT_POSITION: u32 = 0x4;
const OUTPUT_WRITE: u32 = 0xc;

/// One write through the output stream's write function from `buffer`:
/// the word at `slot` is set to `component` and passed as the component
/// size (one component); the position advances by the bytes written.
pub(crate) fn stream_write(
    e: &mut Engine,
    stream: u32,
    buffer: u32,
    size: u32,
    component: u32,
    slot: u32,
) {
    let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    let write = e.mem.u32(output.wrapping_add(OUTPUT_WRITE));
    e.mem.set_u32(slot, component);
    let bytes = e
        .call(write, &args![output, buffer, size, slot, 1u32])
        .u32();
    let position = e.mem.u32(output.wrapping_add(OUTPUT_POSITION));
    e.mem.set_u32(
        output.wrapping_add(OUTPUT_POSITION),
        position.wrapping_add(bytes),
    );
}

/// `NiParticlesData::IsEqual` (`00a96a30`), `SaveBinary` (`00a97290`) and
/// `GetViewerStrings` (`00a97420`) (Xbox PDB names; `thiscall`, one word).
const PARTICLES_DATA_IS_EQUAL: u32 = 0x00a9_6a30;
const PARTICLES_DATA_SAVE_BINARY: u32 = 0x00a9_7290;
const PARTICLES_DATA_GET_VIEWER_STRINGS: u32 = 0x00a9_7420;
/// `NiParticleInfo::IsEqual` (`00c31e90`, Xbox PDB name; `thiscall`, one
/// word).
const PARTICLE_INFO_IS_EQUAL: u32 = 0x00c3_1e90;
/// `NiGetViewerString` (`00a7d330`, one word: the text) and its overloads
/// `_ov6` (`00a7d490`: name, a 16-bit value) and `_ov10` (`00a7d5a0`:
/// name, a pointer) (Xbox PDB names; `cdecl`; they return the string).
const VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRING_U16: u32 = 0x00a7_d490;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
/// `NiPSysData::ms_RTTI`'s name (the word at `01202558`).
const RTTI_NAME: u32 = 0x0120_2558;
/// The labels `GetViewerStrings` passes: "m_pkParticleInfo" (`010be340`),
/// "m_pfRotationSpeeds" (`010be32c`), "NumAddedParticles" (`010be318`) and
/// "AddedParticlesBase" (`010be304`).
const LABEL_PARTICLE_INFO: u32 = 0x010b_e340;
const LABEL_ROTATION_SPEEDS: u32 = 0x010b_e32c;
const LABEL_NUM_ADDED: u32 = 0x010b_e318;
const LABEL_ADDED_BASE: u32 = 0x010b_e304;

// Translated from 00a967e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticlesData::SetActiveVertexCount` (Xbox PDB): the active count
/// becomes `count`, at most the capacity (`m_usVertices`).
pub fn ni_particles_data_set_active_vertex_count(
    e: &mut Engine,
    this: Ptr<NiPSysData>,
    count: u16,
) {
    let vertices = e.get(this, NiPSysData::m_usVertices);
    if count <= vertices {
        e.set(this, NiPSysData::m_usActive, count);
        return;
    }
    e.set(this, NiPSysData::m_usActive, vertices);
}

// Translated from 00a968c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: replaces the rotation axes
/// (`m_pkRotationAxes`), the old array given back to the memory manager.
pub fn fn_00a968c0(e: &mut Engine, this: Ptr<NiPSysData>, axes: u32) {
    let old = e.get(this, NiPSysData::m_pkRotationAxes);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, old]);
    e.set(this, NiPSysData::m_pkRotationAxes, axes);
}

// Translated from 00c24de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::IsEqual` (Xbox PDB): `NiParticlesData::IsEqual` first; then
/// both or neither must have rotation speeds; for each active particle of
/// `this` the particle infos must be equal (`NiParticleInfo::IsEqual`) and,
/// when `this` has rotation speeds, the speeds equal (`FUCOMPP`: a NaN is
/// unequal); finally the added-particle count and base must match.
pub fn ni_p_sys_data_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysData>,
    other: Ptr<NiPSysData>,
) -> bool {
    if !e.call(PARTICLES_DATA_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let speeds = e.get(this, NiPSysData::m_pfRotationSpeeds);
    let other_speeds = e.get(other, NiPSysData::m_pfRotationSpeeds);
    if (speeds != 0 && other_speeds == 0) || (speeds == 0 && other_speeds != 0) {
        return false;
    }
    let mut index: u16 = 0;
    while index < e.get(this, NiPSysData::m_usActive) {
        let offset = u32::from(index) * PARTICLE_INFO_SIZE;
        let theirs = e
            .get(other, NiPSysData::m_pkParticleInfo)
            .wrapping_add(offset);
        let ours = e
            .get(this, NiPSysData::m_pkParticleInfo)
            .wrapping_add(offset);
        if !e.call(PARTICLE_INFO_IS_EQUAL, &args![ours, theirs]).bool() {
            return false;
        }
        let speeds = e.get(this, NiPSysData::m_pfRotationSpeeds);
        if speeds != 0 {
            let ours = e.mem.f32(speeds.wrapping_add(u32::from(index) * 4));
            let other_speeds = e.get(other, NiPSysData::m_pfRotationSpeeds);
            let theirs = e.mem.f32(other_speeds.wrapping_add(u32::from(index) * 4));
            if theirs != ours {
                return false;
            }
        }
        index = index.wrapping_add(1);
    }
    e.get(this, NiPSysData::m_usNumAddedParticles)
        == e.get(other, NiPSysData::m_usNumAddedParticles)
        && e.get(this, NiPSysData::m_usAddedParticlesBase)
            == e.get(other, NiPSysData::m_usAddedParticlesBase)
}

// Translated from 00c24ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiPSysData`'s destructor body. The vtable
/// back to `NiPSysData`'s, the particle info given back to the memory
/// manager, the rotation speeds freed (`00aa10f0`), then `NiParticlesData`'s
/// destructor body (a tail jump).
pub fn fn_00c24ef0(e: &mut Engine, this: Ptr<NiPSysData>) {
    let info = e.get(this, NiPSysData::m_pkParticleInfo);
    e.mem.set_u32(this.addr(), VTABLE);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, info]);
    let speeds = e.get(this, NiPSysData::m_pfRotationSpeeds);
    e.call(ARRAY_FREE, &args![speeds]);
    e.call(PARTICLES_DATA_DESTRUCTOR, &args![this]);
}

// Translated from 00c25560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::SaveBinary` (Xbox PDB): `NiParticlesData::SaveBinary`, then
/// one byte, whether there are rotation speeds (1 or 0).
pub fn ni_p_sys_data_save_binary(e: &mut Engine, this: Ptr<NiPSysData>, stream: Ptr) {
    e.call(PARTICLES_DATA_SAVE_BINARY, &args![this, stream]);
    let has_speeds = u8::from(e.get(this, NiPSysData::m_pfRotationSpeeds) != 0);
    e.with_stack(8, |e, locals| {
        let flag = locals.addr();
        let size_slot = flag + 4;
        e.mem.set_u8(flag, has_speeds);
        stream_write(e, stream.addr(), flag, 1, 1, size_slot);
    });
}

// Translated from 00c255b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysData::GetViewerStrings` (Xbox PDB): `NiParticlesData`'s strings,
/// then the class name, the particle-info and rotation-speed pointers and
/// the added-particle count and base.
pub fn ni_p_sys_data_get_viewer_strings(e: &mut Engine, this: Ptr<NiPSysData>, strings: Ptr) {
    let strings = strings.addr();
    e.call(PARTICLES_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.global::<u32>(RTTI_NAME);
    let string = e.call(VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, string);
    let info = e.get(this, NiPSysData::m_pkParticleInfo);
    let string = e
        .call(VIEWER_STRING_POINTER, &args![LABEL_PARTICLE_INFO, info])
        .u32();
    add_viewer_string(e, strings, string);
    let speeds = e.get(this, NiPSysData::m_pfRotationSpeeds);
    let string = e
        .call(VIEWER_STRING_POINTER, &args![LABEL_ROTATION_SPEEDS, speeds])
        .u32();
    add_viewer_string(e, strings, string);
    let added = u32::from(e.get(this, NiPSysData::m_usNumAddedParticles));
    let string = e
        .call(VIEWER_STRING_U16, &args![LABEL_NUM_ADDED, added])
        .u32();
    add_viewer_string(e, strings, string);
    let base = u32::from(e.get(this, NiPSysData::m_usAddedParticlesBase));
    let string = e
        .call(VIEWER_STRING_U16, &args![LABEL_ADDED_BASE, base])
        .u32();
    add_viewer_string(e, strings, string);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a9_23a0,
            ni_particles_data_get_active_vertex_count(Ptr<NiPSysData>) -> u16
        ),
        entry!(0x00a9_68a0, fn_00a968a0(Ptr<NiPSysData>, u32)),
        entry!(0x00a9_68e0, fn_00a968e0(Ptr<NiPSysData>, u32, u8)),
        entry!(
            0x00c2_4cc0,
            ni_p_sys_data_remove_particle(Ptr<NiPSysData>, u32)
        ),
        entry!(
            0x00c2_4d20,
            ni_p_sys_data_resolve_added_particles(Ptr<NiPSysData>)
        ),
        entry!(0x00c2_4da0, fn_00c24da0() -> Ptr<NiPSysData>),
        entry!(
            0x00c2_4ea0,
            ni_p_sys_data_add_particle(Ptr<NiPSysData>) -> u16
        ),
        entry!(
            0x00c2_4f20,
            fn_00c24f20(Ptr<NiPSysData>, Ptr<NiPSysData>, Ptr)
        ),
        entry!(
            0x00c2_5280,
            ni_p_sys_data_scalar_deleting_destructor(Ptr<NiPSysData>, u32) -> Ptr<NiPSysData>
        ),
        entry!(
            0x00c2_52d0,
            ni_p_sys_data_create_clone(Ptr<NiPSysData>, Ptr) -> Ptr<NiPSysData>
        ),
        entry!(0x00c2_5340, ni_p_sys_data_load_binary(Ptr<NiPSysData>, Ptr)),
        entry!(0x00c24e90, ni_p_sys_data_get_r_t_t_i() -> u32),
        entry!(0x00a7_5f00, ni_p_sys_data_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a7_b7d0, ni_p_sys_data_link_object(Ptr, Ptr)),
        entry!(
            0x00a9_67e0,
            ni_particles_data_set_active_vertex_count(Ptr<NiPSysData>, u16)
        ),
        entry!(0x00a9_68c0, fn_00a968c0(Ptr<NiPSysData>, u32)),
        entry!(
            0x00c2_4de0,
            ni_p_sys_data_is_equal(Ptr<NiPSysData>, Ptr<NiPSysData>) -> bool
        ),
        entry!(0x00c2_4ef0, fn_00c24ef0(Ptr<NiPSysData>)),
        entry!(0x00c2_5560, ni_p_sys_data_save_binary(Ptr<NiPSysData>, Ptr)),
        entry!(
            0x00c2_55b0,
            ni_p_sys_data_get_viewer_strings(Ptr<NiPSysData>, Ptr)
        ),
    ]
}

/// Test support shared by this crate's particle units: a viewer-string
/// array, a counter standing in for the string makers, and an output
/// stream that keeps what is written.
#[cfg(test)]
pub(crate) mod viewer_test {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// An `NiTArray<char *>` with room for 64 strings, capacity `max`,
    /// growing by 4. The `SetSize` double sets the capacity; the `SetAt`
    /// double stores the string its element pointer points at and makes
    /// the size one more than the index.
    pub fn strings(e: &mut Engine, max: u16) -> u32 {
        let array = Ptr::<NiTArray>::new(e.mem.alloc(0x10));
        let base = e.mem.alloc(64 * 4);
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, max);
        e.set(array, NiTArray::m_usGrowBy, 4);
        e.register(STRINGS_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRINGS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let string = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], string);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        array.addr()
    }

    /// The strings stored so far, in index order.
    pub fn collected(e: &Engine, strings: u32) -> Vec<u32> {
        let base = e.mem.u32(strings + 4);
        let size = u32::from(e.mem.u16(strings + 0xa));
        (0..size).map(|i| e.mem.u32(base + 4 * i)).collect()
    }

    /// Doubles for the string makers at `addrs`: each call, whichever,
    /// returns the next number from 1.
    pub fn counting(e: &mut Engine, addrs: &[u32]) {
        let next = Rc::new(Cell::new(0u32));
        for &addr in addrs {
            let next = next.clone();
            e.register_double(addr, move |_, _| {
                next.set(next.get() + 1);
                Ret {
                    eax: next.get(),
                    ..Ret::default()
                }
            });
        }
    }

    /// The logged calls to `callee`, their argument words.
    pub fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A double's answer in `EAX`.
    pub fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An object of 0x100 bytes whose vtable (0x100 bytes of its own) has
    /// `slots` pointing at the given addresses.
    pub fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        object
    }

    /// Doubles answering 0 for every address in `addrs`.
    pub fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    /// The write function's address in tests.
    pub const WRITE_FN: u32 = 0x0f7f_1000;

    /// An `NiStream` whose output stream appends what is written to a
    /// buffer (up to 0x400 bytes) and answers the size; returns the
    /// stream's address.
    pub fn out_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x14);
        let buffer = e.mem.alloc(0x404);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        e.mem.set_u32(output + OUTPUT_WRITE, WRITE_FN);
        e.mem.set_u32(output + 0x10, buffer);
        e.register(WRITE_FN, |e, a| {
            let (output, source, size) = (a[0], a[1], a[2]);
            let buffer = e.mem.u32(output + 0x10);
            let used = e.mem.u32(buffer);
            let bytes = e.mem.bytes(source, size);
            e.mem.write(buffer + 4 + used, &bytes);
            e.mem.set_u32(buffer, used + size);
            Ret {
                eax: size,
                ..Ret::default()
            }
        });
        stream
    }

    /// The bytes written so far, and the output stream's position word.
    pub fn written(e: &Engine, stream: u32) -> (Vec<u8>, u32) {
        let output = e.mem.u32(stream + STREAM_OUTPUT);
        let buffer = e.mem.u32(output + 0x10);
        let used = e.mem.u32(buffer);
        (
            e.mem.bytes(buffer + 4, used),
            e.mem.u32(output + OUTPUT_POSITION),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a75f00_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a7_5f00;
        const TARGET: u32 = 0x00a8_a9b0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    #[test]
    fn thunk_00a7b7d0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_b7d0;
        const TARGET: u32 = 0x00a8_a9a0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn ni_p_sys_data_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_4e90, &args![]).u32(), 0x0120_2558);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Doubles for the allocators: each allocation is a fresh heap block of
    /// the size asked (at least 4 bytes); frees do nothing.
    fn allocators(e: &mut Engine) {
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1].max(4)),
            ..Ret::default()
        });
        e.register(ARRAY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0].max(4)),
            ..Ret::default()
        });
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0].max(4)),
            ..Ret::default()
        });
        for addr in [DEALLOCATE, ARRAY_FREE, NI_DELETE] {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn data(e: &mut Engine) -> Ptr<NiPSysData> {
        e.new_object::<NiPSysData>()
    }

    #[test]
    fn get_active_vertex_count_reads_the_active_count() {
        let mut e = Engine::new();
        let this = data(&mut e);
        e.set(this, NiPSysData::m_usActive, 37);
        assert_eq!(e.call(0x00a9_23a0, &args![this]).u16(), 37);
    }

    #[test]
    fn rotations_and_sub_texture_offsets_are_replaced() {
        let mut e = Engine::new();
        allocators(&mut e);
        let this = data(&mut e);
        e.set(this, NiPSysData::m_pkRotations, 0x111);
        e.set(this, NiPSysData::m_pkSubTextureOffsets, 0x222);
        e.call_log = Some(vec![]);
        e.call(0x00a9_68a0, &args![this, 0x333u32]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x111]]);
        assert_eq!(e.get(this, NiPSysData::m_pkRotations), 0x333);
        e.call(0x00a9_68e0, &args![this, 0x444u32, 0x0fu32]);
        assert_eq!(calls_to(&e, ARRAY_FREE), vec![vec![0x222]]);
        assert_eq!(e.get(this, NiPSysData::m_pkSubTextureOffsets), 0x444);
        assert_eq!(e.get(this, NiPSysData::m_ucSubTextureOffsetCount), 0x0f);
        // Only the low byte of the count word is read; 16 and more give 16.
        e.call(0x00a9_68e0, &args![this, 0x555u32, 0xff20u32]);
        assert_eq!(e.get(this, NiPSysData::m_ucSubTextureOffsetCount), 0x10);
    }

    #[test]
    fn remove_particle_moves_the_last_particle_into_the_hole() {
        let mut e = Engine::new();
        e.register(PARTICLES_DATA_REMOVE_PARTICLE, |e, a| {
            let active = e.mem.u16(a[0] + 0x50);
            e.mem.set_u16(a[0] + 0x50, active - 1);
            Ret::default()
        });
        let this = data(&mut e);
        e.set(this, NiPSysData::m_usActive, 3);
        let info = e.mem.alloc(3 * PARTICLE_INFO_SIZE);
        for i in 0..21 {
            e.mem.set_u32(info + 4 * i, i);
        }
        let speeds = e.mem.alloc(12);
        e.mem.set_u32(speeds + 8, 0x7f80_0001);
        e.set(this, NiPSysData::m_pkParticleInfo, info);
        e.set(this, NiPSysData::m_pfRotationSpeeds, speeds);
        e.call_log = Some(vec![]);
        e.call(0x00c2_4cc0, &args![this, 0x0001_0000u32]);
        assert_eq!(
            calls_to(&e, PARTICLES_DATA_REMOVE_PARTICLE),
            vec![vec![this.addr(), 0x0001_0000]]
        );
        for i in 0..7 {
            assert_eq!(e.mem.u32(info + 4 * i), 14 + i);
        }
        assert_eq!(e.mem.u32(speeds), 0x7fc0_0001);
        // Removing the last one moves nothing.
        e.set(this, NiPSysData::m_usActive, 3);
        e.mem.set_u32(info + 0x1c, 0xabcd);
        e.call(0x00c2_4cc0, &args![this, 2u32]);
        assert_eq!(e.mem.u32(info + 0x1c), 0xabcd);
    }

    #[test]
    fn resolve_added_particles_grows_the_active_count() {
        let mut e = Engine::new();
        let this = data(&mut e);
        e.set(this, NiPSysData::m_usActive, 5);
        e.set(this, NiPSysData::m_usAddedParticlesBase, 5);
        e.set(this, NiPSysData::m_usNumAddedParticles, 3);
        e.call(0x00c2_4d20, &args![this]);
        assert_eq!(e.get(this, NiPSysData::m_usActive), 8);
        assert_eq!(e.get(this, NiPSysData::m_usNumAddedParticles), 0);
        assert_eq!(e.get(this, NiPSysData::m_usAddedParticlesBase), 8);
    }

    #[test]
    fn resolve_added_particles_removes_up_to_the_base() {
        const REMOVE: u32 = 0x0f30_0000;
        let mut e = Engine::new();
        let this = data(&mut e);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xa4, REMOVE);
        e.mem.set_u32(this.addr(), vtable);
        // Each removal lowers the active count by one.
        e.register(REMOVE, |e, a| {
            let active = e.mem.u16(a[0] + 0x50);
            e.mem.set_u16(a[0] + 0x50, active - 1);
            Ret::default()
        });
        e.set(this, NiPSysData::m_usActive, 2);
        e.set(this, NiPSysData::m_usAddedParticlesBase, 5);
        e.set(this, NiPSysData::m_usNumAddedParticles, 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_4d20, &args![this]);
        // Active becomes 6; removing particle 2 brings it down to the base
        // (5), which stops the loop at index 3, below the base.
        assert_eq!(calls_to(&e, REMOVE), vec![vec![this.addr(), 2]]);
        assert_eq!(e.get(this, NiPSysData::m_usActive), 3);
        assert_eq!(e.get(this, NiPSysData::m_usAddedParticlesBase), 3);
        assert_eq!(e.get(this, NiPSysData::m_usNumAddedParticles), 0);
        // Stopping before the base: the active count drops to the base early.
        e.set(this, NiPSysData::m_usActive, 0);
        e.set(this, NiPSysData::m_usAddedParticlesBase, 4);
        e.set(this, NiPSysData::m_usNumAddedParticles, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_4d20, &args![this]);
        assert!(calls_to(&e, REMOVE).is_empty());
        assert_eq!(e.get(this, NiPSysData::m_usActive), 0);
        assert_eq!(e.get(this, NiPSysData::m_usAddedParticlesBase), 0);
        assert_eq!(e.get(this, NiPSysData::m_usNumAddedParticles), 0);
    }

    #[test]
    fn factory_constructs_or_answers_null() {
        let mut e = Engine::new();
        allocators(&mut e);
        e.register(PARTICLES_DATA_CONSTRUCTOR, |_, _| Ret::default());
        let made = e.call(0x00c2_4da0, &args![]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(made), VTABLE);
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_4da0, &args![]).u32(), 0);
        assert!(calls_to(&e, PARTICLES_DATA_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn add_particle_hands_out_slots_until_full() {
        let mut e = Engine::new();
        let this = data(&mut e);
        e.set(this, NiPSysData::m_usVertices, 4);
        e.set(this, NiPSysData::m_usActive, 2);
        assert_eq!(e.call(0x00c2_4ea0, &args![this]).u32(), 2);
        assert_eq!(e.get(this, NiPSysData::m_usAddedParticlesBase), 2);
        assert_eq!(e.call(0x00c2_4ea0, &args![this]).u32(), 3);
        assert_eq!(e.call(0x00c2_4ea0, &args![this]).u32(), 0xffff);
        assert_eq!(e.get(this, NiPSysData::m_usNumAddedParticles), 2);
        // Full from the start: the base still moves to the active count.
        e.set(this, NiPSysData::m_usNumAddedParticles, 0);
        e.set(this, NiPSysData::m_usActive, 4);
        assert_eq!(e.call(0x00c2_4ea0, &args![this]).u32(), 0xffff);
        assert_eq!(e.get(this, NiPSysData::m_usAddedParticlesBase), 4);
        assert_eq!(e.get(this, NiPSysData::m_usNumAddedParticles), 0);
    }

    fn filled(e: &mut Engine, bytes: u32, seed: u8) -> u32 {
        let block = e.mem.alloc(bytes);
        let data: Vec<u8> = (0..bytes).map(|i| seed.wrapping_add(i as u8)).collect();
        e.mem.write(block, &data);
        block
    }

    #[test]
    fn copy_members_copies_every_present_array() {
        let mut e = Engine::new();
        allocators(&mut e);
        e.register(NI_OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCTOR, |_, _| Ret::default());
        for addr in [
            GEOMETRY_DATA_REPLACE,
            SET_SIZES,
            SET_RADII,
            SET_ROTATION_ANGLES,
            SET_ROTATION_AXES,
            SET_TEXTURE_INDICES,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = data(&mut e);
        let dest = data(&mut e);
        e.set(this, NiPSysData::m_usVertices, 2);
        e.set(this, NiPSysData::m_usActive, 1);
        let vertices = filled(&mut e, 24, 1);
        let colors = filled(&mut e, 32, 2);
        let sizes = filled(&mut e, 8, 3);
        let radii = filled(&mut e, 8, 4);
        let info = filled(&mut e, 56, 5);
        let indices = filled(&mut e, 2, 6);
        e.set(this, NiPSysData::m_pkVertex, vertices);
        e.set(this, NiPSysData::m_pkColor, colors);
        e.set(this, NiPSysData::m_pfSizes, sizes);
        e.set(this, NiPSysData::m_pfRadii, radii);
        e.set(this, NiPSysData::m_pkParticleInfo, info);
        e.set(this, NiPSysData::m_pucTextureIndices, indices);
        e.set(this, NiPSysData::m_usNumAddedParticles, 7);
        e.set(this, NiPSysData::m_usAddedParticlesBase, 8);
        e.set(this, NiPSysData::m_kBound, 1);
        e.set(this, NiPSysData::m_kBoundCenterY, 2);
        e.set(this, NiPSysData::m_kBoundCenterZ, 3);
        e.set(this, NiPSysData::m_kBoundRadius, 0x7f80_0002);
        e.set(dest, NiPSysData::m_pkParticleInfo, 0x999);
        e.set(dest, NiPSysData::m_pfRotationSpeeds, 0x888);
        e.call_log = Some(vec![]);
        e.call(0x00c2_4f20, &args![this, dest, 0x4000u32]);
        let replace = calls_to(&e, GEOMETRY_DATA_REPLACE);
        assert_eq!(replace.len(), 1);
        let r = &replace[0];
        assert_eq!((r[0], r[1], r[3]), (dest.addr(), 2, 0));
        assert_eq!(&r[5..], &[0, 0, 0]);
        assert_eq!(e.mem.bytes(r[2], 24), e.mem.bytes(vertices, 24));
        assert_eq!(e.mem.bytes(r[4], 32), e.mem.bytes(colors, 32));
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCTOR),
            vec![
                vec![r[4], 0x10, 2, COLOR_CONSTRUCTOR],
                vec![
                    e.get(dest, NiPSysData::m_pkParticleInfo),
                    PARTICLE_INFO_SIZE,
                    2,
                    PARTICLE_INFO_CONSTRUCTOR
                ]
            ]
        );
        let set_sizes = calls_to(&e, SET_SIZES);
        assert_eq!(e.mem.bytes(set_sizes[0][1], 8), e.mem.bytes(sizes, 8));
        let set_radii = calls_to(&e, SET_RADII);
        assert_eq!(e.mem.bytes(set_radii[0][1], 8), e.mem.bytes(radii, 8));
        let set_indices = calls_to(&e, SET_TEXTURE_INDICES);
        assert_eq!(e.mem.bytes(set_indices[0][1], 2), e.mem.bytes(indices, 2));
        assert!(calls_to(&e, SET_ROTATION_ANGLES).is_empty());
        assert!(calls_to(&e, SET_ROTATION_AXES).is_empty());
        assert_eq!(e.get(dest, NiPSysData::m_pkRotations), 0);
        assert_eq!(e.get(dest, NiPSysData::m_usActive), 1);
        let copied = e.get(dest, NiPSysData::m_pkParticleInfo);
        assert_eq!(e.mem.bytes(copied, 56), e.mem.bytes(info, 56));
        assert_eq!(
            calls_to(&e, DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0], vec![MEMORY_MANAGER, 0x999]]
        );
        assert_eq!(calls_to(&e, ARRAY_FREE), vec![vec![0x888]]);
        assert_eq!(e.get(dest, NiPSysData::m_pfRotationSpeeds), 0);
        assert_eq!(e.get(dest, NiPSysData::m_usNumAddedParticles), 7);
        assert_eq!(e.get(dest, NiPSysData::m_usAddedParticlesBase), 8);
        assert_eq!(e.get(dest, NiPSysData::m_kBoundCenterZ), 3);
        assert_eq!(e.get(dest, NiPSysData::m_kBoundRadius), 0x7fc0_0002);
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_arrays() {
        let mut e = Engine::new();
        allocators(&mut e);
        e.register(PARTICLES_DATA_DESTRUCTOR, |_, _| Ret::default());
        let this = data(&mut e);
        e.set(this, NiPSysData::m_pkParticleInfo, 0x11);
        e.set(this, NiPSysData::m_pfRotationSpeeds, 0x22);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_5280, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x11]]);
        assert_eq!(calls_to(&e, ARRAY_FREE), vec![vec![0x22]]);
        assert!(calls_to(&e, NI_DELETE).is_empty());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        e.call(0x00c2_5280, &args![this, 1u32]);
        assert_eq!(calls_to(&e, NI_DELETE), vec![vec![this.addr(), 0x7c]]);
    }

    #[test]
    fn create_clone_copies_into_a_new_object_or_null() {
        let mut e = Engine::new();
        allocators(&mut e);
        e.register(PARTICLES_DATA_CONSTRUCTOR, |_, _| Ret::default());
        // The member copy is a direct call; its first callee shows which
        // `dest` it got. The null `dest` of a failed allocation is written
        // through as the exe does, so page 0 is mapped here.
        e.map(0, 0x1000);
        e.register(NI_OBJECT_COPY_MEMBERS, |_, _| Ret::default());
        e.register(GEOMETRY_DATA_REPLACE, |_, _| Ret::default());
        for addr in [SET_SIZES, SET_RADII] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = data(&mut e);
        let empty = e.mem.alloc(4);
        e.set(this, NiPSysData::m_pkVertex, empty);
        e.set(this, NiPSysData::m_pfSizes, empty);
        e.set(this, NiPSysData::m_pfRadii, empty);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_52d0, &args![this, 0x4000u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(
            calls_to(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x4000]]
        );
        e.register(NI_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c2_52d0, &args![this, 0x4000u32]).u32(), 0);
        assert_eq!(
            calls_to(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), 0, 0x4000]]
        );
    }

    fn load_setup(e: &mut Engine) {
        allocators(e);
        e.register(PARTICLES_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(PARTICLE_INFO_CONSTRUCTOR, |_, _| Ret::default());
        e.register(PARTICLE_INFO_LOAD_BINARY, |_, _| Ret::default());
        e.register(QUATERNION_TO_ANGLE_AXIS, |_, _| Ret::default());
    }

    #[test]
    fn load_binary_new_files_read_speeds_and_added_counts() {
        let mut e = Engine::new();
        load_setup(&mut e);
        let mut bytes = vec![1u8];
        bytes.extend_from_slice(&1.5f32.to_le_bytes());
        bytes.extend_from_slice(&2.5f32.to_le_bytes());
        bytes.extend_from_slice(&[3, 0, 4, 0]);
        let stream = test_stream::new(&mut e, 0x1400_0002, &bytes);
        e.mem.set_u32(stream + STREAM_BS_VERSION, 0x12);
        let this = data(&mut e);
        e.set(this, NiPSysData::m_usVertices, 2);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5340, &args![this, stream]);
        let info = e.get(this, NiPSysData::m_pkParticleInfo);
        assert_eq!(
            calls_to(&e, PARTICLE_INFO_CONSTRUCTOR),
            vec![vec![info], vec![info + 0x1c]]
        );
        assert_eq!(
            calls_to(&e, PARTICLE_INFO_LOAD_BINARY),
            vec![vec![info, stream], vec![info + 0x1c, stream]]
        );
        let speeds = e.get(this, NiPSysData::m_pfRotationSpeeds);
        assert_eq!(e.mem.f32(speeds), 1.5);
        assert_eq!(e.mem.f32(speeds + 4), 2.5);
        assert_eq!(e.get(this, NiPSysData::m_usNumAddedParticles), 3);
        assert_eq!(e.get(this, NiPSysData::m_usAddedParticlesBase), 4);
        assert!(calls_to(&e, QUATERNION_TO_ANGLE_AXIS).is_empty());
    }

    #[test]
    fn load_binary_bethesda_19_stops_after_allocating_speeds() {
        let mut e = Engine::new();
        load_setup(&mut e);
        let stream = test_stream::new(&mut e, 0x1402_0007, &[1]);
        e.mem.set_u32(stream + STREAM_BS_VERSION, 0x13);
        let this = data(&mut e);
        e.set(this, NiPSysData::m_usVertices, 2);
        e.set(this, NiPSysData::m_usNumAddedParticles, 9);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5340, &args![this, stream]);
        assert!(calls_to(&e, PARTICLE_INFO_LOAD_BINARY).is_empty());
        assert_ne!(e.get(this, NiPSysData::m_pfRotationSpeeds), 0);
        assert_eq!(e.get(this, NiPSysData::m_usNumAddedParticles), 9);
        assert_eq!(test_stream::position(&e, stream), 1);
        // No speeds: nothing more is read either at 19.
        let stream = test_stream::new(&mut e, 0x1402_0007, &[0]);
        e.mem.set_u32(stream + STREAM_BS_VERSION, 0x13);
        let this = data(&mut e);
        e.call(0x00c2_5340, &args![this, stream]);
        assert_eq!(e.get(this, NiPSysData::m_pfRotationSpeeds), 0);
        assert_eq!(test_stream::position(&e, stream), 1);
    }

    #[test]
    fn load_binary_old_files_convert_rotations_and_zero_speeds() {
        let mut e = Engine::new();
        load_setup(&mut e);
        e.register(DEALLOCATE, |_, _| Ret::default());
        let stream = test_stream::new(&mut e, 0x0a03_0004, &[1, 0, 2, 0]);
        let this = data(&mut e);
        e.set(this, NiPSysData::m_usVertices, 2);
        e.set(this, NiPSysData::m_pkRotations, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5340, &args![this, stream]);
        let angles = e.get(this, NiPSysData::m_pfRotationAngles);
        let axes = e.get(this, NiPSysData::m_pkRotationAxes);
        assert_eq!(
            calls_to(&e, QUATERNION_TO_ANGLE_AXIS),
            vec![
                vec![0x5000, angles, axes],
                vec![0x5010, angles + 4, axes + 12]
            ]
        );
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x5000]]);
        assert_eq!(e.get(this, NiPSysData::m_pkRotations), 0);
        let speeds = e.get(this, NiPSysData::m_pfRotationSpeeds);
        assert_ne!(speeds, 0);
        assert_eq!(e.mem.u64(speeds), 0);
        assert_eq!(e.get(this, NiPSysData::m_usNumAddedParticles), 1);
        assert_eq!(e.get(this, NiPSysData::m_usAddedParticlesBase), 2);
    }

    #[test]
    fn load_binary_failed_info_allocation_leaves_no_info() {
        let mut e = Engine::new();
        load_setup(&mut e);
        e.register(ALLOCATE, |_, _| Ret::default());
        let stream = test_stream::new(&mut e, 0x0a03_0005, &[0, 0, 0, 0]);
        let this = data(&mut e);
        e.set(this, NiPSysData::m_usVertices, 0);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5340, &args![this, stream]);
        assert_eq!(e.get(this, NiPSysData::m_pkParticleInfo), 0);
        assert!(calls_to(&e, PARTICLE_INFO_CONSTRUCTOR).is_empty());
        assert_eq!(e.get(this, NiPSysData::m_pfRotationSpeeds), 0);
    }

    #[test]
    fn set_active_vertex_count_is_capped_at_the_capacity() {
        let mut e = Engine::new();
        let this = data(&mut e);
        e.set(this, NiPSysData::m_usVertices, 10);
        e.call(0x00a9_67e0, &args![this, 0xffff_0007u32]);
        assert_eq!(e.get(this, NiPSysData::m_usActive), 7);
        e.call(0x00a9_67e0, &args![this, 10u32]);
        assert_eq!(e.get(this, NiPSysData::m_usActive), 10);
        e.call(0x00a9_67e0, &args![this, 11u32]);
        assert_eq!(e.get(this, NiPSysData::m_usActive), 10);
    }

    #[test]
    fn rotation_axes_are_replaced() {
        let mut e = Engine::new();
        allocators(&mut e);
        let this = data(&mut e);
        e.set(this, NiPSysData::m_pkRotationAxes, 0x111);
        e.call_log = Some(vec![]);
        e.call(0x00a9_68c0, &args![this, 0x333u32]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x111]]);
        assert_eq!(e.get(this, NiPSysData::m_pkRotationAxes), 0x333);
    }

    fn equal_pair(e: &mut Engine) -> (Ptr<NiPSysData>, Ptr<NiPSysData>) {
        e.register(PARTICLES_DATA_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(PARTICLE_INFO_IS_EQUAL, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0]) == e.mem.u32(a[1])),
            ..Ret::default()
        });
        let this = data(e);
        let other = data(e);
        for p in [this, other] {
            e.set(p, NiPSysData::m_usActive, 2);
            let info = e.mem.alloc(2 * PARTICLE_INFO_SIZE);
            e.set(p, NiPSysData::m_pkParticleInfo, info);
            e.set(p, NiPSysData::m_usNumAddedParticles, 3);
            e.set(p, NiPSysData::m_usAddedParticlesBase, 4);
        }
        (this, other)
    }

    #[test]
    fn is_equal_compares_infos_speeds_and_added_counts() {
        let mut e = Engine::new();
        let (this, other) = equal_pair(&mut e);
        let call = |e: &mut Engine| e.call(0x00c2_4de0, &args![this, other]).bool();
        assert!(call(&mut e));
        // Rotation speeds on one side only.
        let speeds = e.mem.alloc(8);
        e.set(this, NiPSysData::m_pfRotationSpeeds, speeds);
        assert!(!call(&mut e));
        e.set(this, NiPSysData::m_pfRotationSpeeds, 0);
        e.set(other, NiPSysData::m_pfRotationSpeeds, speeds);
        assert!(!call(&mut e));
        // Both: equal speeds, then a NaN speed is unequal.
        let theirs = e.mem.alloc(8);
        e.set(this, NiPSysData::m_pfRotationSpeeds, theirs);
        e.mem.set_f32(speeds + 4, 2.5);
        e.mem.set_f32(theirs + 4, 2.5);
        assert!(call(&mut e));
        e.mem.set_u32(speeds + 4, 0x7fc0_0000);
        e.mem.set_u32(theirs + 4, 0x7fc0_0000);
        assert!(!call(&mut e));
        e.mem.set_u32(speeds + 4, 0);
        e.mem.set_u32(theirs + 4, 0);
        // The second particle's info differs.
        let info = e.get(other, NiPSysData::m_pkParticleInfo);
        e.mem.set_u32(info + PARTICLE_INFO_SIZE, 9);
        e.call_log = Some(vec![]);
        assert!(!call(&mut e));
        let ours = e.get(this, NiPSysData::m_pkParticleInfo);
        assert_eq!(
            calls_to(&e, PARTICLE_INFO_IS_EQUAL),
            vec![vec![ours, info], vec![ours + 0x1c, info + 0x1c]]
        );
        e.mem.set_u32(info + PARTICLE_INFO_SIZE, 0);
        assert!(call(&mut e));
        // Added counts and base.
        e.set(other, NiPSysData::m_usNumAddedParticles, 5);
        assert!(!call(&mut e));
        e.set(other, NiPSysData::m_usNumAddedParticles, 3);
        e.set(other, NiPSysData::m_usAddedParticlesBase, 5);
        assert!(!call(&mut e));
        // NiParticlesData::IsEqual decides first.
        e.register(PARTICLES_DATA_IS_EQUAL, |_, _| Ret::default());
        e.set(other, NiPSysData::m_usAddedParticlesBase, 4);
        assert!(!call(&mut e));
    }

    #[test]
    fn destructor_body_frees_info_and_speeds_then_runs_the_base() {
        let mut e = Engine::new();
        allocators(&mut e);
        e.register(PARTICLES_DATA_DESTRUCTOR, |_, _| Ret::default());
        let this = data(&mut e);
        e.set(this, NiPSysData::m_pkParticleInfo, 0x111);
        e.set(this, NiPSysData::m_pfRotationSpeeds, 0x222);
        e.call_log = Some(vec![]);
        e.call(0x00c2_4ef0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![MEMORY_MANAGER, 0x111]]);
        assert_eq!(calls_to(&e, ARRAY_FREE), vec![vec![0x222]]);
        assert_eq!(
            calls_to(&e, PARTICLES_DATA_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn save_binary_writes_whether_there_are_rotation_speeds() {
        let mut e = Engine::new();
        e.register(PARTICLES_DATA_SAVE_BINARY, |_, _| Ret::default());
        let stream = viewer_test::out_stream(&mut e);
        let this = data(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c2_5560, &args![this, stream]);
        assert_eq!(
            calls_to(&e, PARTICLES_DATA_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        e.set(this, NiPSysData::m_pfRotationSpeeds, 0x40);
        e.call(0x00c2_5560, &args![this, stream]);
        assert_eq!(viewer_test::written(&e, stream), (vec![0, 1], 2));
        let writes = calls_to(&e, viewer_test::WRITE_FN);
        assert_eq!((writes[0][2], writes[0][4]), (1, 1));
    }

    #[test]
    fn viewer_strings_add_the_name_pointers_and_counts() {
        let mut e = Engine::new();
        e.register(PARTICLES_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        viewer_test::counting(
            &mut e,
            &[VIEWER_STRING, VIEWER_STRING_U16, VIEWER_STRING_POINTER],
        );
        e.map(RTTI_NAME, 4);
        e.set_global::<u32>(RTTI_NAME, 0x0120_2600);
        let strings = viewer_test::strings(&mut e, 2);
        let this = data(&mut e);
        e.set(this, NiPSysData::m_pkParticleInfo, 0x111);
        e.set(this, NiPSysData::m_pfRotationSpeeds, 0x222);
        e.set(this, NiPSysData::m_usNumAddedParticles, 0xfffe);
        e.set(this, NiPSysData::m_usAddedParticlesBase, 5);
        e.call_log = Some(vec![]);
        e.call(0x00c2_55b0, &args![this, strings]);
        assert_eq!(viewer_test::collected(&e, strings), vec![1, 2, 3, 4, 5]);
        assert_eq!(
            calls_to(&e, PARTICLES_DATA_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING), vec![vec![0x0120_2600]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_POINTER),
            vec![
                vec![LABEL_PARTICLE_INFO, 0x111],
                vec![LABEL_ROTATION_SPEEDS, 0x222]
            ]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_U16),
            vec![vec![LABEL_NUM_ADDED, 0xfffe], vec![LABEL_ADDED_BASE, 5]]
        );
        // Full at index 2: grown to 2 + 4.
        assert_eq!(calls_to(&e, STRINGS_SET_SIZE), vec![vec![strings, 6]]);
    }
}
