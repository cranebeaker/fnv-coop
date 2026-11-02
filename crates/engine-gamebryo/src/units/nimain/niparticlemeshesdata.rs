//! `NiMain/niparticlemeshesdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiParticleMeshesData::ms_RTTI` (the class's `NiRTTI` record).
const NI_PARTICLE_MESHES_DATA_RTTI: u32 = 0x011f_5f0c;

// Translated from 00a98b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_particle_meshes_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PARTICLE_MESHES_DATA_RTTI)
}

// Translated from 00a98dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a98b90`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x78 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_particle_meshes_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a9_8b90;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x78u32]);
    }
    this
}

layout! {
    /// `NiParticleMeshesData` (Xbox PDB), 0x78 bytes (the sized delete frees 0x78); its own
    /// fields at the PC offsets the code uses.
    pub struct NiParticleMeshesData: 0x78 {
        /// `m_spParticleMeshes` (Xbox PDB): the `NiNode` holding one mesh per particle.
        0x70 m_spParticleMeshes: u32,
        /// `m_bNeedsPropertyEffectUpdate` (Xbox PDB).
        0x74 m_bNeedsPropertyEffectUpdate: u8,
    }
}

/// Inherited fields read here (Xbox PDB names, PC offsets): `NiGeometryData::m_usVertices`
/// +0x08 (16 bits), `NiGeometryData::m_pkVertex` +0x20, and `NiParticlesData`'s
/// `m_bIsOldRotatingParticlesObject` +0x40 (byte), `m_pfRadii` +0x4c, `m_usActive` +0x50
/// (16 bits), `m_pfSizes` +0x54 and `m_pkRotations` +0x58.
const VERTEX_COUNT: u32 = 0x08;
const VERTICES: u32 = 0x20;
const OLD_ROTATING: u32 = 0x40;
const RADII: u32 = 0x4c;
const ACTIVE: u32 = 0x50;
const SIZES: u32 = 0x54;
const ROTATIONS: u32 = 0x58;
/// `NiNode`'s children array (`m_kChildren` base, +0xa0) and the node's virtuals used here
/// (offsets as the code indexes the vtable; Xbox PDB slots of `NiNode`): +0xec detaches the
/// child at an index into a pointer out-parameter, +0xf0 detaches the child at an index,
/// +0xf8 sets the child at an index.
const NODE_CHILDREN: u32 = 0xa0;
const NODE_DETACH_CHILD_AT_OUT: u32 = 0xec;
const NODE_DETACH_CHILD_AT: u32 = 0xf0;
const NODE_SET_AT: u32 = 0xf8;
/// `NiParticleMeshesData`'s vtable (its RTTI names `NiParticleMeshesData`).
const NI_PARTICLE_MESHES_DATA_VTABLE: u32 = 0x010a_09bc;
/// `NiParticlesData::LoadBinary`, `SaveBinary`, `IsEqual`, `RemoveParticle` and
/// `GetViewerStrings` (Xbox PDB, `thiscall`, one stack word each), its constructor
/// (`00a963f0`) and destructor body (`00a96800`).
const PARTICLES_DATA_LOAD_BINARY: u32 = 0x00a9_6cf0;
const PARTICLES_DATA_SAVE_BINARY: u32 = 0x00a9_7290;
const PARTICLES_DATA_IS_EQUAL: u32 = 0x00a9_6a30;
const PARTICLES_DATA_REMOVE_PARTICLE: u32 = 0x00a9_64c0;
const PARTICLES_DATA_GET_VIEWER_STRINGS: u32 = 0x00a9_7420;
const PARTICLES_DATA_CONSTRUCTOR: u32 = 0x00a9_63f0;
const PARTICLES_DATA_DESTRUCTOR: u32 = 0x00a9_6800;
/// `NiLinesData::RegisterStreamables` and `NiLinesData::LinkObject` (Xbox PDB, `thiscall`,
/// the stream).
const LINES_DATA_REGISTER_STREAMABLES: u32 = 0x00a8_a9b0;
const LINES_DATA_LINK_OBJECT: u32 = 0x00a8_a9a0;
/// `00a645b0` (`thiscall` on the stream, no stack word): reads the next link id;
/// `NiStream::GetObjectFromLinkID` (Xbox PDB, `thiscall`, no stack word).
const READ_LINK_ID: u32 = 0x00a6_45b0;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `00aa13e0` (`cdecl`, one word): allocates a block of that size.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `00a5ecb0` (`thiscall`, one word; unit `ninode.cpp`): the `NiNode` constructor with a
/// child count.
const NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
/// `NiQuaternion::ToRotation` (engine map, `004f0180`; `thiscall` on the quaternion, the
/// matrix written).
const QUATERNION_TO_ROTATION: u32 = 0x004f_0180;
/// The import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word); the viewer-string array's set-size
/// (`0096ad30`) and set-at (`0096ae90`).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

/// Drops one reference of the counted object at `object` (`InterlockedDecrement` of its
/// count at +4; at zero its virtual +4).
fn release(e: &mut Engine, object: u32) {
    if object != 0 {
        let count = e
            .call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32();
        if count == 0 {
            e.vcall(object, 4, &args![]);
        }
    }
}

/// The counted assignment of `object` to the pointer at `slot`: nothing when it is already
/// there; otherwise the old one released, `object` stored and counted.
fn assign(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    release(e, old);
    e.mem.set_u32(slot, object);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
    }
}

// Translated from 00a98a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::LoadBinary` (Xbox PDB): `NiParticlesData::LoadBinary`, then the
/// mesh node's link id (`00a645b0`).
pub fn ni_particle_meshes_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PARTICLES_DATA_LOAD_BINARY, &args![this, stream]);
    e.call(READ_LINK_ID, &args![stream]);
}

// Translated from 00a98a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::SetActiveVertexCount` (Xbox PDB): every mesh from `count` up to
/// the active count (+0x50) is detached from the mesh node (its virtual +0xf0 with the
/// 16-bit index; the node and the active count read again each time), then the active
/// count becomes `count`, at most the vertex count (+8).
pub fn ni_particle_meshes_data_set_active_vertex_count(e: &mut Engine, this: Ptr, count: u16) {
    let base = this.addr();
    let mut index = count;
    if count < e.mem.u16(base + ACTIVE) {
        loop {
            let node = e.mem.u32(base + 0x70);
            e.vcall(node, NODE_DETACH_CHILD_AT, &args![u32::from(index)]);
            index = index.wrapping_add(1);
            if index >= e.mem.u16(base + ACTIVE) {
                break;
            }
        }
    }
    let vertices = e.mem.u16(base + VERTEX_COUNT);
    let active = if count > vertices { vertices } else { count };
    e.mem.set_u16(base + ACTIVE, active);
}

// Translated from 00a98a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::RegisterStreamables` (Xbox PDB): false when
/// `NiLinesData::RegisterStreamables` refuses; otherwise the mesh node registers itself
/// (virtual +0x54, no null check) and the answer is true.
pub fn ni_particle_meshes_data_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    if !e
        .call(LINES_DATA_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let node = e.mem.u32(this.addr() + 0x70);
    e.vcall(node, 0x54, &args![stream]);
    true
}

// Translated from 00a98ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::SaveBinary` (Xbox PDB): `NiParticlesData::SaveBinary`, then the
/// mesh node's link id (the stream's virtual +0x30).
pub fn ni_particle_meshes_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(PARTICLES_DATA_SAVE_BINARY, &args![this, stream]);
    let node = e.mem.u32(this.addr() + 0x70);
    e.vcall(stream.addr(), 0x30, &args![node]);
}

// Translated from 00a98ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::IsEqual` (Xbox PDB): `NiParticlesData::IsEqual`, then the mesh
/// nodes' `IsEqual` (virtual +0x5c, no null check).
pub fn ni_particle_meshes_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(PARTICLES_DATA_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let node = e.mem.u32(this.addr() + 0x70);
    let other_node = e.mem.u32(other.addr() + 0x70);
    e.vcall(node, 0x5c, &args![other_node]).bool()
}

// Translated from 00a98b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `NiParticleMeshesData`'s
/// constructor. `NiParticlesData`'s constructor (`00a963f0`), the vtable, a new mesh node
/// (0xac bytes from `00aa13e0`, `NiNode`'s constructor with 0 children; null when the
/// allocation fails) assigned as a counted pointer, `m_bNeedsPropertyEffectUpdate`
/// cleared. Answers `this`.
pub fn fn_00a98b10(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(PARTICLES_DATA_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, NI_PARTICLE_MESHES_DATA_VTABLE);
    e.mem.set_u32(base + 0x70, 0);
    let block = e.call(ALLOCATE, &args![0xacu32]).u32();
    let node = if block == 0 {
        0
    } else {
        e.call(NODE_CONSTRUCTOR, &args![block, 0u32]).u32()
    };
    assign(e, base + 0x70, node);
    e.mem.set_u8(base + 0x74, 0);
    this
}

// Translated from 00a98b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `NiParticleMeshesData`'s
/// destructor body. The vtable, the mesh node released and cleared, released again if
/// anything put one back (without clearing it, as the inlined `NiPointer` destructor does),
/// then `NiParticlesData`'s destructor body (`00a96800`, a `JMP`; its result is passed on).
pub fn fn_00a98b90(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    e.mem.set_u32(base, NI_PARTICLE_MESHES_DATA_VTABLE);
    let node = e.mem.u32(base + 0x70);
    if node != 0 {
        release(e, node);
        e.mem.set_u32(base + 0x70, 0);
    }
    let node = e.mem.u32(base + 0x70);
    release(e, node);
    e.call(PARTICLES_DATA_DESTRUCTOR, &args![this]).u32()
}

// Translated from 00a98bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::RemoveParticle` (Xbox PDB): for the last active particle its
/// mesh is detached (the node's virtual +0xf0 with the last index); otherwise the last mesh
/// is detached into a pointer temporary (virtual +0xec, the last index and the temporary's
/// address; the exe keeps it in the argument's stack word), the mesh at `index` detached
/// (+0xf0), the last one set at `index` (+0xf8) and the temporary released. Then
/// `NiParticlesData::RemoveParticle` with the full `index` word.
pub fn ni_particle_meshes_data_remove_particle(e: &mut Engine, this: Ptr, index: u32) {
    let base = this.addr();
    let last = u32::from(e.mem.u16(base + ACTIVE).wrapping_sub(1));
    let node = e.mem.u32(base + 0x70);
    if index as u16 as u32 == last {
        e.vcall(node, NODE_DETACH_CHILD_AT, &args![last]);
        e.call(PARTICLES_DATA_REMOVE_PARTICLE, &args![this, index]);
        return;
    }
    e.with_stack(4, |e, temporary| {
        e.mem.set_u32(temporary.addr(), 0);
        e.vcall(node, NODE_DETACH_CHILD_AT_OUT, &args![last, temporary]);
        let node = e.mem.u32(base + 0x70);
        let short_index = index & 0xffff;
        e.vcall(node, NODE_DETACH_CHILD_AT, &args![short_index]);
        let node = e.mem.u32(base + 0x70);
        let mesh = e.mem.u32(temporary.addr());
        e.vcall(node, NODE_SET_AT, &args![short_index, mesh]);
        let mesh = e.mem.u32(temporary.addr());
        release(e, mesh);
    });
    e.call(PARTICLES_DATA_REMOVE_PARTICLE, &args![this, index]);
}

// Translated from 00a98c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): by its body the update of the
/// particle meshes from the data. For each active particle with a mesh (the node's
/// children array, +0xa0): the mesh's local translation (+0x58) becomes the particle's
/// vertex, its local rotation (+0x34) the particle's rotation when there are rotations
/// (`NiQuaternion::ToRotation`), and its local scale (+0x64) `|size * radius|` (x87, each
/// step stored as a float).
pub fn fn_00a98c90(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut i = 0u32;
    let mut vertex = 0u32;
    let mut rotation = 0u32;
    while i < u32::from(e.mem.u16(base + ACTIVE)) {
        let node = e.mem.u32(base + 0x70);
        let children = e.mem.u32(node.wrapping_add(NODE_CHILDREN));
        let mesh = e.mem.u32(children.wrapping_add(4 * i));
        if mesh != 0 {
            let vertices = e.mem.u32(base + VERTICES).wrapping_add(vertex);
            for word in 0..3 {
                let value = e.mem.u32(vertices.wrapping_add(4 * word));
                e.mem.set_u32(mesh.wrapping_add(0x58 + 4 * word), value);
            }
            let rotations = e.mem.u32(base + ROTATIONS);
            if rotations != 0 {
                e.call(
                    QUATERNION_TO_ROTATION,
                    &args![rotations.wrapping_add(rotation), mesh.wrapping_add(0x34)],
                );
            }
            let sizes = e.mem.u32(base + SIZES);
            let size = x87_load32(e.mem.u32(sizes.wrapping_add(4 * i)));
            let radii = e.mem.u32(base + RADII);
            let radius = x87_m32(e.mem.u32(radii.wrapping_add(4 * i)));
            let scaled = x87_store32(x87_mul(size, radius));
            // FSTP, FLD, FABS, FSTP, then FLD/FSTP into the mesh.
            let magnitude = x87_store32(x87_load32(scaled).abs());
            e.mem.set_u32(mesh.wrapping_add(0x64), magnitude);
        }
        rotation = rotation.wrapping_add(0x10);
        i += 1;
        vertex = vertex.wrapping_add(0xc);
    }
}

// Translated from 00a98d30 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; the creation function the
/// stream registers for `NiParticleMeshesData`, `0109f0e0`): a new particle meshes data
/// (0x78 bytes from `00aa13e0`, [`fn_00a98b10`]) with `m_bIsOldRotatingParticlesObject`
/// (+0x40) set. When the allocation fails the exe still writes that byte at address 0x40.
pub fn fn_00a98d30(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x78u32]).u32();
    let object = if block == 0 {
        0
    } else {
        fn_00a98b10(e, Ptr::new(block)).addr()
    };
    e.mem.set_u8(object.wrapping_add(OLD_ROTATING), 1);
    Ptr::new(object)
}

// Translated from 00a98d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::LinkObject` (Xbox PDB): `NiLinesData::LinkObject`, then the mesh
/// node becomes the stream's object for the next link id (`GetObjectFromLinkID`), assigned
/// as a counted pointer.
pub fn ni_particle_meshes_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(LINES_DATA_LINK_OBJECT, &args![this, stream]);
    let node = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign(e, this.addr() + 0x70, node);
}

// Translated from 00a98de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::GetViewerStrings` (Xbox PDB): `NiParticlesData::GetViewerStrings`,
/// then appends `NiGetViewerString` of the class name (the first word of
/// `NiParticleMeshesData::ms_RTTI`, `011f5f0c`) to `strings`: when the count (+0xa) is not
/// below the capacity (+8) the array first grows by `m_usGrowBy` (+0xe, `0096ad30`), then
/// `0096ae90` stores the string held in the `strings` argument's stack word at the old
/// count.
pub fn ni_particle_meshes_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(PARTICLES_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(NI_PARTICLE_MESHES_DATA_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        let count = u32::from(e.mem.u16(strings.addr() + 0xa));
        if count >= u32::from(e.mem.u16(strings.addr() + 8)) {
            let grow_by = u32::from(e.mem.u16(strings.addr() + 0xe));
            e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
        }
        e.call(ARRAY_SET_AT, &args![strings, count, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a98b00, ni_particle_meshes_data_get_rtti() -> Ptr),
        entry!(0x00a98dc0, ni_particle_meshes_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a9_8a10, ni_particle_meshes_data_load_binary(Ptr, Ptr)),
        entry!(
            0x00a9_8a30,
            ni_particle_meshes_data_set_active_vertex_count(Ptr, u16)
        ),
        entry!(
            0x00a9_8a80,
            ni_particle_meshes_data_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a9_8ab0, ni_particle_meshes_data_save_binary(Ptr, Ptr)),
        entry!(0x00a9_8ad0, ni_particle_meshes_data_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_8b10, fn_00a98b10(Ptr) -> Ptr),
        entry!(0x00a9_8b90, fn_00a98b90(Ptr) -> u32),
        entry!(
            0x00a9_8bf0,
            ni_particle_meshes_data_remove_particle(Ptr, u32)
        ),
        entry!(0x00a9_8c90, fn_00a98c90(Ptr)),
        entry!(0x00a9_8d30, fn_00a98d30() -> Ptr),
        entry!(0x00a9_8d60, ni_particle_meshes_data_link_object(Ptr, Ptr)),
        entry!(
            0x00a9_8de0,
            ni_particle_meshes_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_particle_meshes_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a98b00, &args![]).u32(),
            NI_PARTICLE_MESHES_DATA_RTTI
        );
    }

    #[test]
    fn ni_particle_meshes_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a9_8b90, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a98dc0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a98dc0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x78]]);
        assert_eq!(calls_to(&e, 0x00a9_8b90), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::engine_with_counting;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    /// A counted object (count `count`) whose vtable holds `slots`.
    fn counted(e: &mut Engine, count: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0xb0);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        for target in [
            0x0300_1000u32,
            0x0300_2000,
            0x0300_3000,
            0x0300_4000,
            0x0300_7000,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// A data object whose mesh node's vtable has doubles at +0xec, +0xf0, +0xf8, +0x54 and +4.
    fn data(e: &mut Engine, active: u16, vertices: u16) -> (u32, u32) {
        let node = counted(
            e,
            1,
            &[
                (NODE_DETACH_CHILD_AT_OUT, 0x0300_1000),
                (NODE_DETACH_CHILD_AT, 0x0300_2000),
                (NODE_SET_AT, 0x0300_3000),
                (0x54, 0x0300_4000),
                (4, 0x0300_7000),
            ],
        );
        let this = e.mem.alloc(0x78);
        e.mem.set_u16(this + ACTIVE, active);
        e.mem.set_u16(this + VERTEX_COUNT, vertices);
        e.mem.set_u32(this + 0x70, node);
        (this, node)
    }

    #[test]
    fn load_save_and_link_go_through_the_base_and_the_node() {
        let mut e = engine();
        for double in [
            PARTICLES_DATA_LOAD_BINARY,
            PARTICLES_DATA_SAVE_BINARY,
            READ_LINK_ID,
            LINES_DATA_LINK_OBJECT,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        let (this, node) = data(&mut e, 0, 0);
        let stream = counted(&mut e, 1, &[(0x30, 0x0300_4000)]);
        e.call(0x00a9_8a10, &args![this, stream]);
        assert_eq!(
            calls_to(&e, PARTICLES_DATA_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls_to(&e, READ_LINK_ID), vec![vec![stream]]);
        e.call(0x00a9_8ab0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, PARTICLES_DATA_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls_to(&e, 0x0300_4000), vec![vec![stream, node]]);
        let linked = counted(&mut e, 0, &[]);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| linked.into_ret());
        e.call(0x00a9_8d60, &args![this, stream]);
        assert_eq!(
            calls_to(&e, LINES_DATA_LINK_OBJECT),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(this + 0x70), linked);
        assert_eq!(e.mem.u32(linked + 4), 1);
        assert_eq!(e.mem.u32(node + 4), 0);
    }

    #[test]
    fn set_active_vertex_count_detaches_and_clamps() {
        let mut e = engine();
        let (this, node) = data(&mut e, 5, 8);
        e.call(0x00a9_8a30, &args![this, 2u32]);
        assert_eq!(
            calls_to(&e, 0x0300_2000),
            vec![vec![node, 2], vec![node, 3], vec![node, 4]]
        );
        assert_eq!(e.mem.u16(this + ACTIVE), 2);
        // Above the vertex count: clamped, nothing detached.
        e.call(0x00a9_8a30, &args![this, 0x1_0009u32]);
        assert_eq!(e.mem.u16(this + ACTIVE), 8);
        assert_eq!(calls_to(&e, 0x0300_2000).len(), 3);
    }

    #[test]
    fn register_streamables_and_is_equal_ask_the_node() {
        let mut e = engine();
        let (this, node) = data(&mut e, 0, 0);
        e.register(LINES_DATA_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_8a80, &args![this, 0x4444u32]).bool());
        assert!(calls_to(&e, 0x0300_4000).is_empty());
        e.register(LINES_DATA_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a9_8a80, &args![this, 0x4444u32]).bool());
        assert_eq!(calls_to(&e, 0x0300_4000), vec![vec![node, 0x4444]]);
        let vtable = e.mem.u32(node);
        e.mem.set_u32(vtable + 0x5c, 0x0300_5000);
        e.register(0x0300_5000, |_, _| 1u32.into_ret());
        let (other, other_node) = data(&mut e, 0, 0);
        e.register(PARTICLES_DATA_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_8ad0, &args![this, other]).bool());
        e.register(PARTICLES_DATA_IS_EQUAL, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a9_8ad0, &args![this, other]).bool());
        assert_eq!(calls_to(&e, 0x0300_5000), vec![vec![node, other_node]]);
        e.register(0x0300_5000, |_, _| 0x100u32.into_ret());
        assert!(!e.call(0x00a9_8ad0, &args![this, other]).bool());
    }

    #[test]
    fn constructor_creation_and_destructor() {
        let mut e = engine();
        e.register(PARTICLES_DATA_CONSTRUCTOR, |_, _| Ret::default());
        e.register(NODE_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(PARTICLES_DATA_DESTRUCTOR, |_, _| 0x99u32.into_ret());
        let object = e.call(0x00a9_8d30, &args![]).u32();
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x78], vec![0xac]]);
        assert_eq!(e.mem.u32(object), NI_PARTICLE_MESHES_DATA_VTABLE);
        let node = e.mem.u32(object + 0x70);
        assert_eq!(calls_to(&e, NODE_CONSTRUCTOR), vec![vec![node, 0]]);
        assert_eq!(e.mem.u32(node + 4), 1);
        assert_eq!(e.mem.u8(object + 0x74), 0);
        assert_eq!(e.mem.u8(object + OLD_ROTATING), 1);
        // The destructor body releases the node and clears the pointer.
        e.put_vtable(0x0300_8000, &[0, 0x0300_9000]);
        e.register(0x0300_9000, |_, _| Ret::default());
        e.mem.set_u32(node, 0x0300_8000);
        assert_eq!(e.call(0x00a9_8b90, &args![object]).u32(), 0x99);
        assert_eq!(calls_to(&e, 0x0300_9000), vec![vec![node]]);
        assert_eq!(e.mem.u32(object + 0x70), 0);
        assert_eq!(calls_to(&e, PARTICLES_DATA_DESTRUCTOR), vec![vec![object]]);
    }

    #[test]
    fn remove_particle_moves_the_last_mesh() {
        let mut e = engine();
        e.register(PARTICLES_DATA_REMOVE_PARTICLE, |_, _| Ret::default());
        let (this, node) = data(&mut e, 4, 8);
        // The last one: only detached.
        e.call(0x00a9_8bf0, &args![this, 0xab_0003u32]);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![node, 3]]);
        assert!(calls_to(&e, 0x0300_1000).is_empty());
        assert_eq!(
            calls_to(&e, PARTICLES_DATA_REMOVE_PARTICLE),
            vec![vec![this, 0xab_0003]]
        );
        // Another one: the last mesh moves into its place.
        let mesh = counted(&mut e, 1, &[(4, 0x0300_6000)]);
        e.register(0x0300_6000, |_, _| Ret::default());
        e.register_double(0x0300_1000, move |e, a| {
            e.mem.set_u32(a[2], mesh);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00a9_8bf0, &args![this, 0x1_0001u32]);
        let out = calls_to(&e, 0x0300_1000);
        assert_eq!(out[0][..2], [node, 3]);
        assert_eq!(calls_to(&e, 0x0300_2000), vec![vec![node, 1]]);
        assert_eq!(calls_to(&e, 0x0300_3000), vec![vec![node, 1, mesh]]);
        assert_eq!(calls_to(&e, 0x0300_6000), vec![vec![mesh]]);
        assert_eq!(
            calls_to(&e, PARTICLES_DATA_REMOVE_PARTICLE),
            vec![vec![this, 0x1_0001]]
        );
    }

    #[test]
    fn mesh_update_copies_position_rotation_and_scale() {
        let mut e = engine();
        e.register(QUATERNION_TO_ROTATION, |_, _| Ret::default());
        let (this, node) = data(&mut e, 2, 2);
        let children = e.mem.alloc(8);
        let mesh = e.mem.alloc(0x68);
        e.mem.set_u32(children + 4, mesh);
        e.mem.set_u32(node + NODE_CHILDREN, children);
        let vertices = e.mem.alloc(24);
        for i in 0..6 {
            e.mem.set_f32(vertices + 4 * i, i as f32);
        }
        e.mem.set_u32(this + VERTICES, vertices);
        let sizes = e.mem.alloc(8);
        e.mem.set_f32(sizes + 4, -2.0);
        e.mem.set_u32(this + SIZES, sizes);
        let radii = e.mem.alloc(8);
        e.mem.set_f32(radii + 4, 1.5);
        e.mem.set_u32(this + RADII, radii);
        e.call(0x00a9_8c90, &args![this]);
        let position: Vec<f32> = (0..3).map(|i| e.mem.f32(mesh + 0x58 + 4 * i)).collect();
        assert_eq!(position, vec![3.0, 4.0, 5.0]);
        assert_eq!(e.mem.f32(mesh + 0x64), 3.0);
        assert!(calls_to(&e, QUATERNION_TO_ROTATION).is_empty());
        let rotations = e.mem.alloc(32);
        e.mem.set_u32(this + ROTATIONS, rotations);
        e.call(0x00a9_8c90, &args![this]);
        assert_eq!(
            calls_to(&e, QUATERNION_TO_ROTATION),
            vec![vec![rotations + 0x10, mesh + 0x34]]
        );
    }

    #[test]
    fn viewer_strings_append_the_class_name() {
        let mut e = engine();
        e.mem.set_u32(NI_PARTICLE_MESHES_DATA_RTTI, 0x011f_5000);
        e.register(PARTICLES_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x5000u32.into_ret());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x5000);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.call(0x00a9_8de0, &args![0x7777u32, strings]);
        assert_eq!(
            calls_to(&e, PARTICLES_DATA_GET_VIEWER_STRINGS),
            vec![vec![0x7777, strings]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x011f_5000]]);
        assert!(calls_to(&e, ARRAY_SET_SIZE).is_empty());
        assert_eq!(calls_to(&e, ARRAY_SET_AT)[0][..2], [strings, 0]);
    }
}
