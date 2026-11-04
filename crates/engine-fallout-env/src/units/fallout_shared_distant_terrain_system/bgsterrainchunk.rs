//! `fallout shared/distant terrain system/bgsterrainchunk.cpp` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The distant-terrain chunk (`BGSTerrainChunk`) and its load task
//! (`BGSTerrainChunkLoadTask`). The unit is compiled without optimisation:
//! every `NiPointer` read goes through the out-of-line `00559450`
//! (`return *this`), which the translations call each time as the exe does.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `BGSTerrainChunk` (Xbox PDB), 0x30 bytes; the PC code uses the same
    /// offsets.
    pub struct BGSTerrainChunk: 0x30 {
        /// `pNode` (Xbox PDB): `BGSTerrainNode*`.
        0x00 pNode: u32,
        /// `spMesh` (Xbox PDB): `NiPointer<NiGeometry>`.
        0x04 spMesh: u32,
        /// `spWater` (Xbox PDB).
        0x08 spWater: u32,
        /// `spWaterAtReflectionHeight` (Xbox PDB).
        0x0C spWaterAtReflectionHeight: u32,
        /// `spNode` (Xbox PDB): `NiPointer<BSMultiBoundNode>`.
        0x10 spNode: u32,
        /// `spWaterNode` (Xbox PDB).
        0x14 spWaterNode: u32,
        /// `spWaterReflNode` (Xbox PDB).
        0x18 spWaterReflNode: u32,
        /// `pLODWaterRef` (Xbox PDB): `TESObjectREFR*`.
        0x1C pLODWaterRef: u32,
        /// `pLODWaterAtReflectionHeightRef` (Xbox PDB).
        0x20 pLODWaterAtReflectionHeightRef: u32,
        /// `spLoadTask` (Xbox PDB): `NiPointer<BGSTerrainChunkLoadTask>`.
        0x24 spLoadTask: u32,
        /// `bDoneLoading` (Xbox PDB).
        0x28 bDoneLoading: u8,
        /// `bAttached` (Xbox PDB).
        0x29 bAttached: u8,
        /// `bPrepared` (Xbox PDB).
        0x2A bPrepared: u8,
        /// `spLightingProp` (Xbox PDB):
        /// `NiPointer<BSShaderPPLightingProperty>`.
        0x2C spLightingProp: u32,
    }

    /// `BGSTerrainChunkLoadTask` (Xbox PDB), 0x60 bytes: a `QueuedFileEntry`
    /// and the chunk's coordinates and results; the PC code uses the same
    /// offsets.
    pub struct BGSTerrainChunkLoadTask: 0x60 {
        /// `bFinishedQueuing` (Xbox PDB).
        0x30 bFinishedQueuing: u8,
        /// `iX` (Xbox PDB).
        0x34 iX: u32,
        /// `iY` (Xbox PDB).
        0x38 iY: u32,
        /// `iLevel` (Xbox PDB).
        0x3C iLevel: u32,
        /// `pChunk` (Xbox PDB): `BGSTerrainChunk*`.
        0x40 pChunk: u32,
        /// `pWorldSpace` (Xbox PDB): `TESWorldSpace*`.
        0x44 pWorldSpace: u32,
        /// `spMesh` (Xbox PDB).
        0x48 spMesh: u32,
        /// `spWater` (Xbox PDB).
        0x4C spWater: u32,
        /// `spWaterAtReflectionHeight` (Xbox PDB).
        0x50 spWaterAtReflectionHeight: u32,
        /// `spNode` (Xbox PDB).
        0x54 spNode: u32,
        /// `bCanceled` (Xbox PDB).
        0x58 bCanceled: u8,
    }
}

/// `00559450` (unnamed): `return *this` (the `NiPointer` read and the
/// first-field read the unoptimised code calls), `thiscall`.
const DEREF: u32 = 0x0055_9450;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `0066b0d0` (unnamed): the `NiPointer` assignment (`thiscall` on the
/// pointer, the object).
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The vtable of `BGSTerrainChunkLoadTask` (RTTI
/// `.?AVBGSTerrainChunkLoadTask@@`).
const LOAD_TASK_VTABLE: u32 = 0x0106_e1dc;
/// `BGSTerrainChunk::iChunkCount` (Xbox PDB static; the word `006facb0`
/// counts up at `011d85ec`).
const CHUNK_COUNT: u32 = 0x011d_85ec;
/// The byte at `0119f154` that makes `006fb170` / `006fb3d0` run without
/// their argument (a water-LOD switch by its use; unnamed).
const WATER_SWITCH: u32 = 0x0119_f154;
/// The global at `011dea10` the water calls start from (`0070ec90` reads a
/// field of it; the engine map's name for that getter is folded code).
const WATER_SYSTEM_HOLDER: u32 = 0x011d_ea10;
const WATER_SYSTEM_GET: u32 = 0x0070_ec90;
/// The singleton at `01202d98` whose vtable slot +0x48 `CheckFinished`
/// calls with the task.
const LOADER_HOLDER: u32 = 0x0120_2d98;
/// The `ModelLoader` singleton (`011c3b3c`) and `ModelLoader::QueueTexture`
/// (`004436c0`, `(path, priority, entry)`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
const QUEUE_TEXTURE: u32 = 0x0044_36c0;
/// The `Setting` at `011c7adc` whose value (`00408d60`) says whether the
/// water shapes are taken from loaded LOD meshes.
const WATER_SETTING: u32 = 0x011c_7adc;
const SETTING_VALUE: u32 = 0x0040_8d60;
/// Strings: the three LOD paths, the missing-property message, the source
/// file name and the three node names.
const MESH_PATH: u32 = 0x0106_e0c8;
const DIFFUSE_PATH: u32 = 0x0106_e100;
const NORMALS_PATH: u32 = 0x0106_e140;
const NO_LIGHTING_MESSAGE: u32 = 0x0106_e180;
const SOURCE_FILE: u32 = 0x0106_e068;
const TERRAIN_NAME: u32 = 0x0106_e274;
const WATER_NAME: u32 = 0x0106_e244;
const REFLECTED_WATER_NAME: u32 = 0x0106_e20c;
/// `sprintf_s` (`00406d00`) and `sprintf` (`00ec623a`), `cdecl`.
const SPRINTF_S: u32 = 0x0040_6d00;
const SPRINTF: u32 = 0x00ec_623a;

/// The path formatters' Rust form (buffer, world space, level, x, y).
type PathFormatter = fn(&mut Engine, u32, u32, u32, i16, i16);

/// `00559450` on `p`.
fn deref(e: &mut Engine, p: u32) -> u32 {
    e.call(DEREF, &args![p]).u32()
}

/// `NiNew` of `size` bytes and the constructor `ctor` on it (0 when the
/// allocation fails, as the code's `if (p) p = ctor(p)`).
fn new_object(e: &mut Engine, size: u32, ctor: u32) -> u32 {
    let block = e.call(NI_NEW, &args![size]).u32();
    if block == 0 {
        0
    } else {
        e.call(ctor, &args![block]).u32()
    }
}

// Translated from 006f7140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainChunkLoadTask::CheckFinished` (Xbox PDB): when `0043caa0` on
/// the task answers true and the task has finished queuing, either the
/// singleton at `01202d98` gets the task through its vtable slot +0x48
/// (when `0069b080` answers true) or `QueuedFile::CheckFinished` runs.
pub fn bgs_terrain_chunk_load_task_check_finished(
    e: &mut Engine,
    this: Ptr<BGSTerrainChunkLoadTask>,
) {
    if !e.call(0x0043_caa0, &args![this]).bool() {
        return;
    }
    if e.get(this, BGSTerrainChunkLoadTask::bFinishedQueuing) == 0 {
        return;
    }
    if e.call(0x0069_b080, &args![this]).bool() {
        let loader = e.global::<u32>(LOADER_HOLDER);
        e.vcall(loader, 0x48, &args![this]);
    } else {
        e.call(0x00c3_c930, &args![this]);
    }
}

// Translated from 006fa670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: hands the chunk (through a stack word
/// holding it) to `00905820` on the object at `011d8624`.
pub fn fn_006fa670(e: &mut Engine, this: Ptr<BGSTerrainChunk>) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), this.addr());
        e.call(0x0090_5820, &args![0x011d_8624u32, slot]);
    });
}

// Translated from 006fa990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: takes the load task's results. For the mesh,
/// water, reflected water and node pointers (+4, +8, +0xc, +0x10) in turn
/// the task (read through `spLoadTask`) hands its result over (`006faa30`,
/// `006f6250`, `006faa50`, `006faa70`, each with the address of the
/// chunk's pointer); the task pointer is cleared (`006f74f0` with 0), the
/// chunk is done loading, and `00929260` runs with 1 on the node's first
/// field.
pub fn fn_006fa990(e: &mut Engine, this: Ptr<BGSTerrainChunk>) {
    let t = this.addr();
    for (off, take) in [
        (BGSTerrainChunk::spMesh.off, 0x006f_aa30u32),
        (BGSTerrainChunk::spWater.off, 0x006f_6250),
        (BGSTerrainChunk::spWaterAtReflectionHeight.off, 0x006f_aa50),
        (BGSTerrainChunk::spNode.off, 0x006f_aa70),
    ] {
        let task = deref(e, t + BGSTerrainChunk::spLoadTask.off);
        e.call(take, &args![task, t + off]);
    }
    e.call(
        0x006f_74f0,
        &args![t + BGSTerrainChunk::spLoadTask.off, 0u32],
    );
    e.set(this, BGSTerrainChunk::bDoneLoading, 1);
    let node = e.get(this, BGSTerrainChunk::pNode);
    let manager = deref(e, node);
    e.call(0x0092_9260, &args![manager, 1u32]);
}

/// The two vtable calls (slot +0x130 of the world space: its name, by the
/// paths they make) the three path formatters make, in the exe's order.
fn world_space_names(e: &mut Engine, world_space: u32) -> (u32, u32) {
    let first = e.vcall(world_space, 0x130, &args![]).u32();
    let second = e.vcall(world_space, 0x130, &args![]).u32();
    (first, second)
}

// Translated from 006faad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names it `BGSTerrainChunkLoadTask::BGSTerrainChunkLoadTask`,
/// but the body (`cdecl`, five words: buffer, world space, level, x, y)
/// formats the LOD mesh path
/// `Data\Meshes\Landscape\LOD\%s\%s.Level%i.X%i.Y%i.NIF` into the 0x104-byte
/// buffer (`sprintf_s`) with the world space's name twice (slot +0x130,
/// the second call's result first), the level and the sign-extended
/// coordinates.
pub fn fn_006faad0(e: &mut Engine, buffer: u32, world_space: u32, level: u32, x: i16, y: i16) {
    let (first, second) = world_space_names(e, world_space);
    e.call(
        SPRINTF_S,
        &args![
            buffer,
            0x104u32,
            MESH_PATH,
            second,
            first,
            level,
            i32::from(x),
            i32::from(y)
        ],
    );
}

// Translated from 006fab20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five words): formats the LOD diffuse
/// texture path `Data\Textures\Landscape\LOD\%s\Diffuse\%s.n.Level%i.X%i.Y%i.dds`
/// into the buffer (`sprintf`), as `006faad0` does.
pub fn fn_006fab20(e: &mut Engine, buffer: u32, world_space: u32, level: u32, x: i16, y: i16) {
    let (first, second) = world_space_names(e, world_space);
    e.call(
        SPRINTF,
        &args![
            buffer,
            DIFFUSE_PATH,
            second,
            first,
            level,
            i32::from(x),
            i32::from(y)
        ],
    );
}

// Translated from 006fab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five words): formats the LOD normal
/// map path `Data\Textures\Landscape\LOD\%s\Normals\%s.n.Level%i.X%i.Y%i.dds`
/// into the buffer (`sprintf`), as `006faad0` does.
pub fn fn_006fab70(e: &mut Engine, buffer: u32, world_space: u32, level: u32, x: i16, y: i16) {
    let (first, second) = world_space_names(e, world_space);
    e.call(
        SPRINTF,
        &args![
            buffer,
            NORMALS_PATH,
            second,
            first,
            level,
            i32::from(x),
            i32::from(y)
        ],
    );
}

// Translated from 006fabc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainChunk::GetTexture` (Xbox PDB): with a mesh, its property of
/// the kind `00438220` names (`NiAVObject::GetProperty`); when that
/// property's type (`00441110`) is 8 to 12, its texture through `00540540`
/// (`which` 0) or `00540570` (otherwise), both with 0; with no such
/// property the message "has a mesh but no PP lighting property" is printed
/// (`005b5e40`, with the node's level `00726070` and coordinates `004a8ae0`
/// and `0047d3f0`). 0 otherwise.
pub fn bgs_terrain_chunk_get_texture(
    e: &mut Engine,
    this: Ptr<BGSTerrainChunk>,
    which: u32,
) -> u32 {
    let t = this.addr();
    let mut texture = 0;
    if deref(e, t + BGSTerrainChunk::spMesh.off) == 0 {
        return texture;
    }
    let kind = e.call(0x0043_8220, &args![]).u32();
    let mesh = deref(e, t + BGSTerrainChunk::spMesh.off);
    let property = e.call(0x00a5_9d30, &args![mesh, kind]).u32();
    let lit = if property == 0 {
        false
    } else {
        let first = e.call(0x0044_1110, &args![property]).i32();
        !(first < 8 || e.call(0x0044_1110, &args![property]).i32() > 0xc)
    };
    let property = if lit { property } else { 0 };
    if property != 0 {
        texture = if which == 0 {
            e.call(0x0054_0540, &args![property, 0u32]).u32()
        } else {
            e.call(0x0054_0570, &args![property, 0u32]).u32()
        };
    } else {
        let node = e.get(this, BGSTerrainChunk::pNode);
        let x = i32::from(e.call(0x0047_d3f0, &args![node]).u16() as i16);
        let node = e.get(this, BGSTerrainChunk::pNode);
        let y = i32::from(e.call(0x004a_8ae0, &args![node]).u16() as i16);
        let node = e.get(this, BGSTerrainChunk::pNode);
        let level = e.call(0x0072_6070, &args![node]).u32();
        e.call(0x005b_5e40, &args![NO_LIGHTING_MESSAGE, level, y, x]);
    }
    texture
}

// Translated from 006facb0 (decompiled, FalloutNV.exe 1.4.0.525)
// open: the C++ exception frame (FS:[0] unwind states around the four
// allocations) is not translated.
// Translation incomplete: 006facb0 - the C++ exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map: attaches a loaded chunk (done loading, not
/// attached yet). The chunk count (`011d85ec`) goes up. With a node, the
/// node is attached (vtable slot +0xdc with 1) to the parent `004ea9a0`
/// gives for the terrain node's manager, the bound (`006629f0`,
/// `0043b230`) gives a centre (`0048d150`) and extent (`00500940`), and
/// for the water node (+0x14) and the reflected water node (+0x18) in turn
/// a `BSMultiBoundAABB` (0x24 bytes, `00c37ec0`) gets that centre with the
/// height `004536e0` and that extent with z 0 (slot +0xb8, `00439680`), a
/// new `BSMultiBoundNode` (0xb4 bytes, `00c46970`) becomes the node and a
/// new `BSMultiBound` (0x10 bytes, `00c35e00`) with the shape becomes its
/// multi-bound (`004addc0`, `00439920`). Without a node the mesh is
/// attached instead. Then the mesh's properties are updated
/// (`NiAVObject::UpdateProperties`) and it and the parent get an update
/// with a zeroed update data (`0043d410(0.0, 0, 0)`, `00a59c60`), the
/// water is set up (`006fb170(0)`), the chunk is prepared (`006fb7e0`) and
/// registered (`006fa670`) and marked attached; when the terrain node's
/// byte (`006fb150`) and `006fa840` say so, the chunk's geometry is shown
/// (`006fb6e0`).
pub fn fn_006facb0(e: &mut Engine, this: Ptr<BGSTerrainChunk>) {
    let t = this.addr();
    if e.get(this, BGSTerrainChunk::bDoneLoading) == 0 {
        return;
    }
    if e.get(this, BGSTerrainChunk::bAttached) != 0 {
        return;
    }
    let count = e.global::<u32>(CHUNK_COUNT).wrapping_add(1);
    e.set_global(CHUNK_COUNT, count);
    e.with_stack(0x40, |e, frame| {
        let center = frame.addr();
        let extent = frame.addr() + 0x10;
        let update = frame.addr() + 0x20;
        let parent_update = frame.addr() + 0x30;
        if deref(e, t + BGSTerrainChunk::spNode.off) != 0 {
            let node = deref(e, t);
            let manager = deref(e, node);
            let parent = e.call(0x004e_a9a0, &args![manager]).u32();
            let scene = deref(e, t + BGSTerrainChunk::spNode.off);
            e.vcall(parent, 0xdc, &args![scene, 1u32]);
            let scene = deref(e, t + BGSTerrainChunk::spNode.off);
            let data = e.call(0x0066_29f0, &args![scene]).u32();
            let bound = e.call(0x0043_b230, &args![data]).u32();
            let source = e.call(0x0048_d150, &args![bound]).u32();
            let bytes = e.mem.bytes(source, 12);
            e.mem.write(center, &bytes);
            let source = e.call(0x0050_0940, &args![bound]).u32();
            let bytes = e.mem.bytes(source, 12);
            e.mem.write(extent, &bytes);
            for node_field in [
                BGSTerrainChunk::spWaterNode.off,
                BGSTerrainChunk::spWaterReflNode.off,
            ] {
                let shape = new_object(e, 0x24, 0x00c3_7ec0);
                let node = e.get(this, BGSTerrainChunk::pNode);
                let manager = deref(e, node);
                let owner = deref(e, manager);
                let height = e.call(0x0045_36e0, &args![owner]).f64();
                e.mem.set_u32(center + 8, x87_store32(height));
                e.mem.set_u32(extent + 8, 0.0f32.to_bits());
                e.vcall(shape, 0xb8, &args![center]);
                e.call(0x0043_9680, &args![shape, extent]);
                let multi_bound_node = new_object(e, 0xb4, 0x00c4_6970);
                e.call(POINTER_ASSIGN, &args![t + node_field, multi_bound_node]);
                let multi_bound = new_object(e, 0x10, 0x00c3_5e00);
                e.call(0x004a_ddc0, &args![multi_bound, shape]);
                let water_node = deref(e, t + node_field);
                e.call(0x0043_9920, &args![water_node, multi_bound]);
            }
        } else {
            let node = deref(e, t);
            let manager = deref(e, node);
            let parent = e.call(0x004e_a9a0, &args![manager]).u32();
            let mesh = deref(e, t + BGSTerrainChunk::spMesh.off);
            e.vcall(parent, 0xdc, &args![mesh, 1u32]);
        }
        let mesh = deref(e, t + BGSTerrainChunk::spMesh.off);
        e.call(0x00a5_a040, &args![mesh]);
        e.call(0x0043_d410, &args![update, 0.0f32.to_bits(), 0u32, 0u32]);
        let mesh = deref(e, t + BGSTerrainChunk::spMesh.off);
        e.call(0x00a5_9c60, &args![mesh, update]);
        fn_006fb170(e, this, 0);
        e.call(
            0x0043_d410,
            &args![parent_update, 0.0f32.to_bits(), 0u32, 0u32],
        );
        let node = deref(e, t);
        let manager = deref(e, node);
        let parent = e.call(0x004e_a9a0, &args![manager]).u32();
        e.call(0x00a5_9c60, &args![parent, parent_update]);
    });
    bgs_terrain_chunk_prepare(e, this);
    fn_006fa670(e, this);
    e.set(this, BGSTerrainChunk::bAttached, 1);
    let node = e.get(this, BGSTerrainChunk::pNode);
    if fn_006fb150(e, Ptr::new(node)) != 0 {
        let node = e.get(this, BGSTerrainChunk::pNode);
        if e.call(0x006f_a840, &args![node]).bool() {
            fn_006fb6e0(e, this);
        }
    }
}

// Translated from 006fb150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the terrain node's byte at +0x5c (PC offset;
/// called on `BGSTerrainChunk::pNode`).
pub fn fn_006fb150(e: &mut Engine, node: Ptr) -> u8 {
    e.mem.u8(node.addr() + 0x5c)
}

// Translated from 006fb170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: makes the LOD water references. With the
/// switch at `0119f154` or `force`, for the reflected water (+0xc, its
/// node +0x18, its reference +0x20, flag 1) and then the water (+8, +0x14,
/// +0x1c, flag 0): with a shape, `004e4c80` on the water system
/// (`0070ec90` on `011dea10`) makes a reference from the shape, the
/// terrain manager's owner, its bound and the node (the node pointer
/// cleared after); a reference's 3D (slot +0x1d0) gives through slot +0xc
/// and `0043b4a0(0)` an object whose slot +0x14 becomes the node; and a
/// segmented shape (slot +0x28) gets all segments enabled (`00a872d0`).
pub fn fn_006fb170(e: &mut Engine, this: Ptr<BGSTerrainChunk>, force: u8) {
    let t = this.addr();
    if e.global::<u8>(WATER_SWITCH) == 0 && force == 0 {
        return;
    }
    for (shape_field, node_field, reference_field, flag) in [
        (
            BGSTerrainChunk::spWaterAtReflectionHeight.off,
            BGSTerrainChunk::spWaterReflNode.off,
            BGSTerrainChunk::pLODWaterAtReflectionHeightRef.off,
            1u32,
        ),
        (
            BGSTerrainChunk::spWater.off,
            BGSTerrainChunk::spWaterNode.off,
            BGSTerrainChunk::pLODWaterRef.off,
            0,
        ),
    ] {
        if deref(e, t + shape_field) == 0 {
            continue;
        }
        let node = deref(e, t + node_field);
        let terrain_node = deref(e, t);
        let manager = deref(e, terrain_node);
        let bound = e.call(0x0043_b230, &args![manager]).u32();
        let terrain_node = deref(e, t);
        let manager = deref(e, terrain_node);
        let owner = deref(e, manager);
        let shape = deref(e, t + shape_field);
        let holder = e.global::<u32>(WATER_SYSTEM_HOLDER);
        let system = e.call(WATER_SYSTEM_GET, &args![holder]).u32();
        let reference = e
            .call(0x004e_4c80, &args![system, shape, owner, bound, node, flag])
            .u32();
        e.mem.set_u32(t + reference_field, reference);
        e.call(POINTER_ASSIGN, &args![t + node_field, 0u32]);
        let reference = e.mem.u32(t + reference_field);
        if reference != 0 {
            let root = e.vcall(reference, 0x1d0, &args![]).u32();
            let object = e.vcall(root, 0xc, &args![]).u32();
            let child = e.call(0x0043_b4a0, &args![object, 0u32]).u32();
            let water_node = e.vcall(child, 0x14, &args![]).u32();
            e.call(POINTER_ASSIGN, &args![t + node_field, water_node]);
        }
        let shape = deref(e, t + shape_field);
        if e.vcall(shape, 0x28, &args![]).u32() != 0 {
            let shape = deref(e, t + shape_field);
            let segmented = e.vcall(shape, 0x28, &args![]).u32();
            e.call(0x00a8_72d0, &args![segmented]);
        }
    }
}

// Translated from 006fb3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes the LOD water references. With the
/// switch at `0119f154` or `force`, for the water (+8, node +0x14,
/// reference +0x1c) and then the reflected water (+0xc, +0x18, +0x20): with
/// a shape and a reference, the water system (when `0070ec90` gives one)
/// drops the reference (`004e5140` with the manager's owner and bound), the
/// node is detached from the bound (its slot +0xe8), the reference is
/// destroyed (slot +0x10 with 1) and cleared.
pub fn fn_006fb3d0(e: &mut Engine, this: Ptr<BGSTerrainChunk>, force: u8) {
    let t = this.addr();
    if e.global::<u8>(WATER_SWITCH) == 0 && force == 0 {
        return;
    }
    for (shape_field, node_field, reference_field) in [
        (
            BGSTerrainChunk::spWater.off,
            BGSTerrainChunk::spWaterNode.off,
            BGSTerrainChunk::pLODWaterRef.off,
        ),
        (
            BGSTerrainChunk::spWaterAtReflectionHeight.off,
            BGSTerrainChunk::spWaterReflNode.off,
            BGSTerrainChunk::pLODWaterAtReflectionHeightRef.off,
        ),
    ] {
        if deref(e, t + shape_field) == 0 || e.mem.u32(t + reference_field) == 0 {
            continue;
        }
        let holder = e.global::<u32>(WATER_SYSTEM_HOLDER);
        if e.call(WATER_SYSTEM_GET, &args![holder]).u32() != 0 {
            let terrain_node = e.get(this, BGSTerrainChunk::pNode);
            let manager = deref(e, terrain_node);
            let bound = e.call(0x0043_b230, &args![manager]).u32();
            let terrain_node = e.get(this, BGSTerrainChunk::pNode);
            let manager = deref(e, terrain_node);
            let owner = deref(e, manager);
            let reference = e.mem.u32(t + reference_field);
            let holder = e.global::<u32>(WATER_SYSTEM_HOLDER);
            let system = e.call(WATER_SYSTEM_GET, &args![holder]).u32();
            e.call(0x004e_5140, &args![system, reference, owner, bound]);
        }
        if deref(e, t + node_field) != 0 {
            let terrain_node = deref(e, t);
            let manager = deref(e, terrain_node);
            let bound = e.call(0x0043_b230, &args![manager]).u32();
            let node = deref(e, t + node_field);
            e.vcall(bound, 0xe8, &args![node]);
        }
        let reference = e.mem.u32(t + reference_field);
        if reference != 0 {
            e.vcall(reference, 0x10, &args![1u32]);
        }
        e.mem.set_u32(t + reference_field, 0);
    }
}

/// `00450f90` with `value` on each of the mesh, water and reflected water
/// (+4, +8, +0xc) that is present.
fn each_geometry(e: &mut Engine, this: Ptr<BGSTerrainChunk>, value: u32) {
    for off in [
        BGSTerrainChunk::spMesh.off,
        BGSTerrainChunk::spWater.off,
        BGSTerrainChunk::spWaterAtReflectionHeight.off,
    ] {
        if deref(e, this.addr() + off) != 0 {
            let geometry = deref(e, this.addr() + off);
            e.call(0x0045_0f90, &args![geometry, value]);
        }
    }
}

// Translated from 006fb6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00450f90` with 1 on the mesh, water and
/// reflected water that are present.
pub fn fn_006fb6e0(e: &mut Engine, this: Ptr<BGSTerrainChunk>) {
    each_geometry(e, this, 1);
}

// Translated from 006fb760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00450f90` with 0 on the mesh, water and
/// reflected water that are present.
pub fn fn_006fb760(e: &mut Engine, this: Ptr<BGSTerrainChunk>) {
    each_geometry(e, this, 0);
}

// Translated from 006fb7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainChunk::Prepare` (Xbox PDB): once, with a mesh: its property
/// of the kind `00438220` names gets flag 0x21 set
/// (`BSShaderProperty::SetFlag`) and the three floats (0, 0, 1) at
/// +0xf0 (`006fb940`); the mesh is prepared (`BSShaderManager::PrepareObject`
/// with 0, 0); the property becomes `spLightingProp`; the mesh's data
/// (`005495f0`) gets `004410d0(0)`, `SetConsistency(0x4000)` and
/// `004410f0(1)`; the renderer object `004030b0` makes (its slot +0x118,
/// with the mesh and the property) what the object `0043c4b0` answers
/// precaches (its slot +0xec, with the mesh, 0, 0 and that); then
/// `00e74120` runs on `0043c4b0`'s object and the chunk is prepared.
pub fn bgs_terrain_chunk_prepare(e: &mut Engine, this: Ptr<BGSTerrainChunk>) {
    let t = this.addr();
    if e.get(this, BGSTerrainChunk::bPrepared) != 0 {
        return;
    }
    let mesh_field = t + BGSTerrainChunk::spMesh.off;
    if deref(e, mesh_field) == 0 {
        return;
    }
    let kind = e.call(0x0043_8220, &args![]).u32();
    let mesh = deref(e, mesh_field);
    let property = e.call(0x00a5_9d30, &args![mesh, kind]).u32();
    e.call(0x0044_1130, &args![property, 0x21u32, 1u32]);
    fn_006fb940(e, Ptr::new(property), 0.0, 0.0, 1.0);
    let mesh = deref(e, mesh_field);
    e.call(0x00b5_7e30, &args![mesh, 0u32, 0u32]);
    e.call(
        POINTER_ASSIGN,
        &args![t + BGSTerrainChunk::spLightingProp.off, property],
    );
    for (call, value) in [
        (0x0044_10d0u32, 0u32),
        (0x00a6_7050, 0x4000),
        (0x0044_10f0, 1),
    ] {
        let mesh = deref(e, mesh_field);
        let data = e.call(0x0054_95f0, &args![mesh]).u32();
        e.call(call, &args![data, value]);
    }
    let mesh = deref(e, mesh_field);
    let renderer_object = e.call(0x0040_30b0, &args![mesh]).u32();
    let precache = e.call(0x0043_c4b0, &args![]).u32();
    let mesh = deref(e, mesh_field);
    let made = e
        .vcall(renderer_object, 0x118, &args![mesh, property])
        .u32();
    let mesh = deref(e, mesh_field);
    e.vcall(precache, 0xec, &args![mesh, 0u32, 0u32, made]);
    let renderer = e.call(0x0043_c4b0, &args![]).u32();
    e.call(0x00e7_4120, &args![renderer]);
    e.set(this, BGSTerrainChunk::bPrepared, 1);
}

// Translated from 006fb940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores three floats at +0xf0, +0xf4 and
/// +0xf8 of the object (the lighting property `Prepare` calls it on); they
/// go through the x87 stack.
pub fn fn_006fb940(e: &mut Engine, this: Ptr, first: f32, second: f32, third: f32) {
    let p = this.addr();
    e.mem.set_u32(p + 0xf0, x87_quiet_bits(first.to_bits()));
    e.mem.set_u32(p + 0xf4, x87_quiet_bits(second.to_bits()));
    e.mem.set_u32(p + 0xf8, x87_quiet_bits(third.to_bits()));
}

// Translated from 006fb980 (decompiled, FalloutNV.exe 1.4.0.525)
// open: the C++ exception frame (FS:[0] unwind states) and the /GS cookie
// check (00ec408c) are not translated.
// Translation incomplete: 006fb980 - the C++ exception frame (FS:[0]) and the /GS cookie check are not translated
/// Unnamed in the engine map: the `BGSTerrainChunkLoadTask` constructor
/// (`thiscall`, `RET 0x14`: chunk, world space, x, y, level). The
/// `QueuedFileEntry` constructor (`00c3ce60` with 3), the vtable, not
/// finished queuing, the coordinates, level, chunk and world space, the
/// four `NiPointer`s constructed null (`00633c90`), not canceled; the mesh
/// path is formatted (`006faad0` with the coordinates' low halves) into a
/// 0x104-byte stack buffer, given past its `Data\` (`00c3cee0`), and
/// `00c3cf60(0)` runs. Returns `this`.
pub fn fn_006fb980(
    e: &mut Engine,
    this: Ptr<BGSTerrainChunkLoadTask>,
    chunk: u32,
    world_space: u32,
    x: u32,
    y: u32,
    level: u32,
) -> Ptr<BGSTerrainChunkLoadTask> {
    let t = this.addr();
    e.call(0x00c3_ce60, &args![this, 3u32]);
    e.mem.set_u32(t, LOAD_TASK_VTABLE);
    e.set(this, BGSTerrainChunkLoadTask::bFinishedQueuing, 0);
    e.set(this, BGSTerrainChunkLoadTask::iX, x);
    e.set(this, BGSTerrainChunkLoadTask::iY, y);
    e.set(this, BGSTerrainChunkLoadTask::iLevel, level);
    e.set(this, BGSTerrainChunkLoadTask::pChunk, chunk);
    e.set(this, BGSTerrainChunkLoadTask::pWorldSpace, world_space);
    for off in [0x48, 0x4c, 0x50, 0x54] {
        e.call(0x0063_3c90, &args![t + off, 0u32]);
    }
    e.set(this, BGSTerrainChunkLoadTask::bCanceled, 0);
    e.with_stack(0x104, |e, buffer| {
        let world_space = e.get(this, BGSTerrainChunkLoadTask::pWorldSpace);
        fn_006faad0(
            e,
            buffer.addr(),
            world_space,
            level,
            x as u16 as i16,
            y as u16 as i16,
        );
        e.call(0x00c3_cee0, &args![this, buffer.addr() + 5]);
    });
    e.call(0x00c3_cf60, &args![this, 0u32]);
    this
}

// Translated from 006fbae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainChunkLoadTask::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor body `006fbb10`, then the free `00401030` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bgs_terrain_chunk_load_task_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BGSTerrainChunkLoadTask>,
    flags: u32,
) -> Ptr<BGSTerrainChunkLoadTask> {
    fn_006fbb10(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 006fbb10 (decompiled, FalloutNV.exe 1.4.0.525)
// open: the C++ exception frame (FS:[0] unwind states) is not translated.
// Translation incomplete: 006fbb10 - the C++ exception frame (FS:[0] unwind states) is not translated
/// Unnamed in the engine map: the `BGSTerrainChunkLoadTask` destructor body.
/// The vtable back to the task's, the mesh, water and reflected water
/// pointers cleared (`0066b0d0(0)`; the node pointer is not), the four
/// `NiPointer`s destroyed last first (`0045cec0`), then the
/// `QueuedFileEntry` destructor (`00c3cea0`).
pub fn fn_006fbb10(e: &mut Engine, this: Ptr<BGSTerrainChunkLoadTask>) {
    let t = this.addr();
    e.mem.set_u32(t, LOAD_TASK_VTABLE);
    for off in [0x48, 0x4c, 0x50] {
        e.call(POINTER_ASSIGN, &args![t + off, 0u32]);
    }
    for off in [0x54, 0x50, 0x4c, 0x48] {
        e.call(0x0045_cec0, &args![t + off]);
    }
    e.call(0x00c3_cea0, &args![this]);
}

// Translated from 006fbbd0 (decompiled, FalloutNV.exe 1.4.0.525)
// open: the /GS cookie check (00ec408c) is not translated.
// Translation incomplete: 006fbbd0 - the /GS cookie check is not translated
/// `BGSTerrainChunkLoadTask::QueueMe` (Xbox PDB): the diffuse and then the
/// normal-map path (`006fab20`, `006fab70`, with the low halves of the
/// coordinates) are queued in turn with the `ModelLoader`
/// (`ModelLoader::QueueTexture` on `011c3b3c`: the path, the task's
/// priority `0043cc60` and the task); the task has finished queuing and its
/// vtable slot +0x28 runs.
pub fn bgs_terrain_chunk_load_task_queue_me(e: &mut Engine, this: Ptr<BGSTerrainChunkLoadTask>) {
    // The two paths have their own stack buffers (0x108 bytes for the
    // diffuse one, then the normal map's).
    let formats: [(PathFormatter, u32); 2] = [(fn_006fab20, 0), (fn_006fab70, 0x108)];
    e.with_stack(0x108 + 0x10c, |e, frame| {
        for (format, offset) in formats {
            let buffer = frame.addr() + offset;
            let world_space = e.get(this, BGSTerrainChunkLoadTask::pWorldSpace);
            let level = e.get(this, BGSTerrainChunkLoadTask::iLevel);
            let x = e.get(this, BGSTerrainChunkLoadTask::iX) as u16 as i16;
            let y = e.get(this, BGSTerrainChunkLoadTask::iY) as u16 as i16;
            format(e, buffer, world_space, level, x, y);
            let priority = e.call(0x0043_cc60, &args![this]).u32();
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(QUEUE_TEXTURE, &args![loader, buffer, priority, this]);
        }
    });
    e.set(this, BGSTerrainChunkLoadTask::bFinishedQueuing, 1);
    e.vcall(this.addr(), 0x28, &args![]);
}

// Translated from 006fbd00 (decompiled, FalloutNV.exe 1.4.0.525)
// open: the C++ exception frame (FS:[0] unwind states) and the /GS cookie
// check (00ec408c) are not translated.
// Translation incomplete: 006fbd00 - the C++ exception frame (FS:[0]) and the /GS cookie check are not translated
/// `BGSTerrainChunkLoadTask::Run` (Xbox PDB): inside a scope marker
/// (`00404eb0(0x23, 1, file, 0x298)` / `00404ee0`), the task's file
/// (`QueuedFileEntry::GetFile(0, 1)`) is loaded into a stack `BSStream`
/// (`0043cfd0` / `BSStream::Load` with `0045cd60`'s answer / `~BSStream`).
/// When the first object is a node (its slot +0xc): it becomes `spNode`,
/// its first child (`0043b4a0(0)`) the mesh, and, when the setting at
/// `011c7adc` is on, the second and third children's geometry (slot +0x18)
/// the water (dropped again when it is a segmented shape of 3 segments)
/// and the reflected water, each when the node has that many children
/// (`00453470`). Otherwise the first object is the mesh and the second,
/// when there is one (`0043b1e0`), the water.
pub fn bgs_terrain_chunk_load_task_run(e: &mut Engine, this: Ptr<BGSTerrainChunkLoadTask>) {
    let t = this.addr();
    e.with_stack(0x5d4, |e, frame| {
        let guard = frame.addr();
        let stream = frame.addr() + 4;
        e.call(
            0x0040_4eb0,
            &args![guard, 0x23u32, 1u32, SOURCE_FILE, 0x298u32],
        );
        let file = e.call(0x00c3_cff0, &args![this, 0u32, 1u32]).u32();
        e.call(0x0043_cfd0, &args![stream]);
        let name = e.call(0x0045_cd60, &args![this]).u32();
        if e.call(0x00c3_a8a0, &args![stream, name, file]).bool() {
            let first = e.call(0x0043_b200, &args![stream, 0u32]).u32();
            if e.vcall(first, 0xc, &args![]).u32() != 0 {
                let first = e.call(0x0043_b200, &args![stream, 0u32]).u32();
                e.call(POINTER_ASSIGN, &args![t + 0x54, first]);
                let node = deref(e, t + 0x54);
                let child = e.call(0x0043_b4a0, &args![node, 0u32]).u32();
                e.call(POINTER_ASSIGN, &args![t + 0x48, child]);
                let node = deref(e, t + 0x54);
                if e.call(0x0045_3470, &args![node]).u32() > 1 && water_setting(e) {
                    let node = deref(e, t + 0x54);
                    let child = e.call(0x0043_b4a0, &args![node, 1u32]).u32();
                    let water = e.vcall(child, 0x18, &args![]).u32();
                    e.call(POINTER_ASSIGN, &args![t + 0x4c, water]);
                    let water = deref(e, t + 0x4c);
                    if e.vcall(water, 0x28, &args![]).u32() != 0 {
                        let water = deref(e, t + 0x4c);
                        let segmented = e.vcall(water, 0x28, &args![]).u32();
                        if bs_segmented_tri_shape_get_num_segments(e, Ptr::new(segmented)) == 3 {
                            e.call(POINTER_ASSIGN, &args![t + 0x4c, 0u32]);
                        }
                    }
                }
                let node = deref(e, t + 0x54);
                if e.call(0x0045_3470, &args![node]).u32() > 2 && water_setting(e) {
                    let node = deref(e, t + 0x54);
                    let child = e.call(0x0043_b4a0, &args![node, 2u32]).u32();
                    let water = e.vcall(child, 0x18, &args![]).u32();
                    e.call(POINTER_ASSIGN, &args![t + 0x50, water]);
                }
            } else {
                let first = e.call(0x0043_b200, &args![stream, 0u32]).u32();
                e.call(POINTER_ASSIGN, &args![t + 0x48, first]);
                if e.call(0x0043_b1e0, &args![stream]).u32() > 1 {
                    let second = e.call(0x0043_b200, &args![stream, 1u32]).u32();
                    e.call(POINTER_ASSIGN, &args![t + 0x4c, second]);
                }
            }
        }
        e.call(0x0043_d090, &args![stream]);
        e.call(0x0040_4ee0, &args![guard]);
    });
}

/// The byte value of the setting at `011c7adc` (`00408d60` answers its
/// address).
fn water_setting(e: &mut Engine) -> bool {
    let value = e.call(SETTING_VALUE, &args![WATER_SETTING]).u32();
    e.mem.u8(value) != 0
}

// Translated from 006fbfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSegmentedTriShape::GetNumSegments` (Xbox PDB): 1 when the byte at
/// +0xcd is set, else the count at +0xc8 (PC offsets).
pub fn bs_segmented_tri_shape_get_num_segments(e: &mut Engine, this: Ptr) -> u32 {
    if e.mem.u8(this.addr() + 0xcd) != 0 {
        1
    } else {
        e.mem.u32(this.addr() + 0xc8)
    }
}

// Translated from 006fc020 (decompiled, FalloutNV.exe 1.4.0.525)
// open: the C++ exception frame (FS:[0] unwind states) and the /GS cookie
// check (00ec408c) are not translated.
// Translation incomplete: 006fc020 - the C++ exception frame (FS:[0]) and the /GS cookie check are not translated
/// `BGSTerrainChunkLoadTask::Finish` (Xbox PDB): with a mesh, unless
/// canceled, the mesh, water and reflected water (each when present) are
/// named `"Terrain LOD | Level %i | Cells (%i,%i)-(%i,%i)"` and the two
/// water forms (the level, the corner cell and the opposite one, `x/y +
/// level - 1`): the format goes through an `NiFixedString`
/// (`00438170`/`0043b1b0`/`004381b0`) into `sprintf` on a stack buffer and
/// the result through another into the object's name (`00a5b950`); then
/// the chunk takes the results (`006fa990`). Canceled, the three pointers
/// are cleared.
pub fn bgs_terrain_chunk_load_task_finish(e: &mut Engine, this: Ptr<BGSTerrainChunkLoadTask>) {
    let t = this.addr();
    if deref(e, t + 0x48) == 0 {
        return;
    }
    if e.get(this, BGSTerrainChunkLoadTask::bCanceled) != 0 {
        if e.get(this, BGSTerrainChunkLoadTask::bCanceled) != 0 {
            for off in [0x48, 0x4c, 0x50] {
                e.call(POINTER_ASSIGN, &args![t + off, 0u32]);
            }
        }
        return;
    }
    e.with_stack(0x10c + 8, |e, frame| {
        let buffer = frame.addr();
        let format_string = frame.addr() + 0x10c;
        let name_string = frame.addr() + 0x110;
        for (index, (off, format)) in [
            (0x48u32, TERRAIN_NAME),
            (0x4c, WATER_NAME),
            (0x50, REFLECTED_WATER_NAME),
        ]
        .into_iter()
        .enumerate()
        {
            if index > 0 && deref(e, t + off) == 0 {
                continue;
            }
            let x = e.get(this, BGSTerrainChunkLoadTask::iX);
            let y = e.get(this, BGSTerrainChunkLoadTask::iY);
            let level = e.get(this, BGSTerrainChunkLoadTask::iLevel);
            let far_y = y.wrapping_add(level).wrapping_sub(1);
            let far_x = x.wrapping_add(level).wrapping_sub(1);
            let held = e.call(0x0043_8170, &args![format_string, format]).u32();
            let text = e.call(0x0043_b1b0, &args![held]).u32();
            e.call(SPRINTF, &args![buffer, text, level, x, y, far_x, far_y]);
            e.call(0x0043_81b0, &args![format_string]);
            let held = e.call(0x0043_8170, &args![name_string, buffer]).u32();
            let object = deref(e, t + off);
            e.call(0x00a5_b950, &args![object, held]);
            e.call(0x0043_81b0, &args![name_string]);
        }
    });
    let chunk = e.get(this, BGSTerrainChunkLoadTask::pChunk);
    fn_006fa990(e, Ptr::new(chunk));
}

// Translated from 006fc440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainChunkLoadTask::GetFileIndex` (Xbox PDB): when `0055b980`
/// answers a file, its archive index (`00af6910` with 1) gives
/// `0043c030 + 4`; 1 otherwise.
pub fn bgs_terrain_chunk_load_task_get_file_index(
    e: &mut Engine,
    this: Ptr<BGSTerrainChunkLoadTask>,
) -> u32 {
    if e.call(0x0055_b980, &args![this]).u32() != 0 {
        let file = e.call(0x0055_b980, &args![this]).u32();
        let archive = e.call(0x00af_6910, &args![file, 1u32]).u32();
        if archive != 0 {
            return e.call(0x0043_c030, &args![archive]).u32().wrapping_add(4);
        }
    }
    1
}

/// The list of chunks the two walks below run over (its head node, `.data`): `008256d0` (thiscall
/// on a node) answers whether it is empty, `006815c0` gives the address of its item and
/// `00726070` the next node.
const CHUNK_LIST_011D8624: u32 = 0x011d_8624;
const LIST_IS_EMPTY_008256D0: u32 = 0x0082_56d0;
const LIST_ITEM_006815C0: u32 = 0x0068_15c0;
const LIST_NEXT_00726070: u32 = 0x0072_6070;

/// The walk shared by [`fn_006fb5d0`] and [`fn_006fb640`]: from the head node until a null node
/// or an empty one, each chunk goes to `visit` and the walk goes on with the next node.
fn each_listed_chunk(e: &mut Engine, visit: fn(&mut Engine, u32)) {
    let mut node = CHUNK_LIST_011D8624;
    while node != 0 {
        if e.call(LIST_IS_EMPTY_008256D0, &args![node]).u8() != 0 {
            return;
        }
        let item = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        let chunk = e.mem.u32(item);
        visit(e, chunk);
        node = e.call(LIST_NEXT_00726070, &args![node]).u32();
    }
}

// Translated from 006fb5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): for each chunk of the list at `011d8624`, [`fn_006fb760`] when
/// [`fn_006fb150`] answers true for its node (`00559450` on the chunk), [`fn_006fb6e0`] otherwise.
pub fn fn_006fb5d0(e: &mut Engine) {
    each_listed_chunk(e, |e, chunk| {
        let node = deref(e, chunk);
        if fn_006fb150(e, Ptr::new(node)) != 0 {
            fn_006fb760(e, Ptr::new(chunk));
        } else {
            fn_006fb6e0(e, Ptr::new(chunk));
        }
    });
}

// Translated from 006fb640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no arguments): for each chunk of the list at `011d8624`, [`fn_006fb6e0`] when
/// [`fn_006fb150`] and `006fa840` (thiscall) answer true for its node and `006fe920(0)` (thiscall
/// on the chunk's first word) does too; [`fn_006fb760`] otherwise.
pub fn fn_006fb640(e: &mut Engine) {
    each_listed_chunk(e, |e, chunk| {
        let node = deref(e, chunk);
        let mut shown = fn_006fb150(e, Ptr::new(node)) != 0;
        if shown {
            let node = deref(e, chunk);
            shown = e.call(0x006f_a840, &args![node]).u8() != 0;
        }
        if shown {
            let first = e.mem.u32(chunk);
            shown = e.call(0x006f_e920, &args![first, 0u32]).u8() != 0;
        }
        if shown {
            fn_006fb6e0(e, Ptr::new(chunk));
        } else {
            fn_006fb760(e, Ptr::new(chunk));
        }
    });
}

// Translated from 006fbcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainChunkLoadTask::Cancel` (the map's name; thiscall, two words, `RET 8`): sets
/// `bCanceled`, then the base `00c3cb70` (the map's `QueuedFile::Cancel`) with both words.
pub fn bgs_terrain_chunk_load_task_cancel(
    e: &mut Engine,
    this: Ptr<BGSTerrainChunkLoadTask>,
    first: u32,
    second: u32,
) {
    e.set(this, BGSTerrainChunkLoadTask::bCanceled, 1);
    e.call(0x00c3_cb70, &args![this, first, second]);
}

// Translated from 006fc400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainChunkLoadTask::GetDescription` (the map's name; thiscall, two words: buffer and
/// size; `RET 8`): formats `"Terrain LOD Task for chunk ( %i, %i ) at level ( %i )"`
/// (`0106e2a4`) with `iX`, `iY` and `iLevel` (`sprintf_s`). Returns true (`AL` = 1).
pub fn bgs_terrain_chunk_load_task_get_description(
    e: &mut Engine,
    this: Ptr<BGSTerrainChunkLoadTask>,
    buffer: u32,
    size: u32,
) -> bool {
    let x = e.get(this, BGSTerrainChunkLoadTask::iX);
    let y = e.get(this, BGSTerrainChunkLoadTask::iY);
    let level = e.get(this, BGSTerrainChunkLoadTask::iLevel);
    e.call(SPRINTF_S, &args![buffer, size, 0x0106_e2a4u32, x, y, level]);
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006f_7140,
            bgs_terrain_chunk_load_task_check_finished(Ptr<BGSTerrainChunkLoadTask>)
        ),
        entry!(0x006f_a670, fn_006fa670(Ptr<BGSTerrainChunk>)),
        entry!(0x006f_a990, fn_006fa990(Ptr<BGSTerrainChunk>)),
        entry!(0x006f_aad0, fn_006faad0(u32, u32, u32, i16, i16)),
        entry!(0x006f_ab20, fn_006fab20(u32, u32, u32, i16, i16)),
        entry!(0x006f_ab70, fn_006fab70(u32, u32, u32, i16, i16)),
        entry!(
            0x006f_abc0,
            bgs_terrain_chunk_get_texture(Ptr<BGSTerrainChunk>, u32) -> u32
        ),
        entry!(0x006f_acb0, fn_006facb0(Ptr<BGSTerrainChunk>)),
        entry!(0x006f_b150, fn_006fb150(Ptr) -> u8),
        entry!(0x006f_b170, fn_006fb170(Ptr<BGSTerrainChunk>, u8)),
        entry!(0x006f_b3d0, fn_006fb3d0(Ptr<BGSTerrainChunk>, u8)),
        entry!(0x006f_b6e0, fn_006fb6e0(Ptr<BGSTerrainChunk>)),
        entry!(0x006f_b760, fn_006fb760(Ptr<BGSTerrainChunk>)),
        entry!(0x006f_b7e0, bgs_terrain_chunk_prepare(Ptr<BGSTerrainChunk>)),
        entry!(0x006f_b940, fn_006fb940(Ptr, f32, f32, f32)),
        entry!(
            0x006f_b980,
            fn_006fb980(
                Ptr<BGSTerrainChunkLoadTask>,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> Ptr<BGSTerrainChunkLoadTask>
        ),
        entry!(
            0x006f_bae0,
            bgs_terrain_chunk_load_task_scalar_deleting_destructor(
                Ptr<BGSTerrainChunkLoadTask>,
                u32,
            )
                -> Ptr<BGSTerrainChunkLoadTask>
        ),
        entry!(0x006f_bb10, fn_006fbb10(Ptr<BGSTerrainChunkLoadTask>)),
        entry!(
            0x006f_bbd0,
            bgs_terrain_chunk_load_task_queue_me(Ptr<BGSTerrainChunkLoadTask>)
        ),
        entry!(
            0x006f_bd00,
            bgs_terrain_chunk_load_task_run(Ptr<BGSTerrainChunkLoadTask>)
        ),
        entry!(0x006f_bfe0, bs_segmented_tri_shape_get_num_segments(Ptr) -> u32),
        entry!(
            0x006f_c020,
            bgs_terrain_chunk_load_task_finish(Ptr<BGSTerrainChunkLoadTask>)
        ),
        entry!(
            0x006f_c440,
            bgs_terrain_chunk_load_task_get_file_index(Ptr<BGSTerrainChunkLoadTask>) -> u32
        ),
        entry!(0x006f_b5d0, fn_006fb5d0()),
        entry!(0x006f_b640, fn_006fb640()),
        entry!(
            0x006f_bcd0,
            bgs_terrain_chunk_load_task_cancel(Ptr<BGSTerrainChunkLoadTask>, u32, u32)
        ),
        entry!(
            0x006f_c400,
            bgs_terrain_chunk_load_task_get_description(
                Ptr<BGSTerrainChunkLoadTask>,
                u32,
                u32,
            ) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn log_of(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The order the given addresses were called in.
    fn order(e: &Engine, addrs: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addrs.contains(a))
            .collect()
    }

    /// Doubles answering 0 for every address given, and `00559450` reading
    /// the word it is called on.
    fn engine(stubs: &[u32]) -> Engine {
        let mut e = Engine::new();
        for addr in stubs {
            e.register(*addr, |_, _| Ret::default());
        }
        e.register(DEREF, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// An object whose vtable slots call the addresses given.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, vtable);
        p
    }

    #[test]
    fn check_finished_hands_on_or_checks() {
        const SLOT: u32 = 0x0f80_0000;
        for (ready, queued, end, expect_slot, expect_check) in [
            (1u32, 1u8, 1u32, 1usize, 0usize),
            (1, 1, 0, 0, 1),
            (1, 0, 1, 0, 0),
            (0, 1, 1, 0, 0),
        ] {
            let mut e = engine(&[0x00c3_c930, SLOT]);
            answer(&mut e, 0x0043_caa0, ready);
            answer(&mut e, 0x0069_b080, end);
            let loader = object(&mut e, 8, &[(0x48, SLOT)]);
            e.map(LOADER_HOLDER & !0xfff, 0x1000);
            e.set_global(LOADER_HOLDER, loader);
            let task = e.new_object::<BGSTerrainChunkLoadTask>();
            e.set(task, BGSTerrainChunkLoadTask::bFinishedQueuing, queued);
            e.call(0x006f_7140, &args![task]);
            assert_eq!(log_of(&e, SLOT).len(), expect_slot);
            assert_eq!(log_of(&e, 0x00c3_c930).len(), expect_check);
        }
    }

    #[test]
    fn registration_passes_the_chunk_through_a_stack_word() {
        let mut e = engine(&[]);
        e.register(0x0090_5820, |e, a| Ret {
            eax: e.mem.u32(a[1]),
            ..Ret::default()
        });
        e.call(0x006f_a670, &args![0x5000u32]);
        assert_eq!(log_of(&e, 0x0090_5820)[0][0], 0x011d_8624);
    }

    #[test]
    fn take_results_hands_each_pointer_over() {
        let takes = [0x006f_aa30u32, 0x006f_6250, 0x006f_aa50, 0x006f_aa70];
        let mut stubs = takes.to_vec();
        stubs.extend([0x006f_74f0, 0x0092_9260]);
        let mut e = engine(&stubs);
        let chunk = e.new_object::<BGSTerrainChunk>();
        let node = e.mem.alloc(0x60);
        e.mem.set_u32(node, 0x7700);
        e.set(chunk, BGSTerrainChunk::pNode, node);
        e.set(chunk, BGSTerrainChunk::spLoadTask, 0x6600);
        e.call(0x006f_a990, &args![chunk]);
        for (take, off) in takes.iter().zip([4u32, 8, 0xc, 0x10]) {
            assert_eq!(log_of(&e, *take), vec![vec![0x6600, chunk.addr() + off]]);
        }
        assert_eq!(log_of(&e, 0x006f_74f0), vec![vec![chunk.addr() + 0x24, 0]]);
        assert_eq!(e.get(chunk, BGSTerrainChunk::bDoneLoading), 1);
        assert_eq!(log_of(&e, 0x0092_9260), vec![vec![0x7700, 1]]);
    }

    const NAME_FN: u32 = 0x0f80_0100;

    #[test]
    fn path_formatters_pass_the_names_and_signed_coordinates() {
        for (addr, printer, format) in [
            (0x006f_aad0u32, SPRINTF_S, MESH_PATH),
            (0x006f_ab20, SPRINTF, DIFFUSE_PATH),
            (0x006f_ab70, SPRINTF, NORMALS_PATH),
        ] {
            let mut e = engine(&[printer]);
            let mut n = 0u32;
            e.register_double(NAME_FN, move |_, _| {
                n += 1;
                Ret {
                    eax: n,
                    ..Ret::default()
                }
            });
            let world = object(&mut e, 8, &[(0x130, NAME_FN)]);
            e.call(
                addr,
                &args![0x5000u32, world, 3u32, 0xfffeu32, 0x0001_0005u32],
            );
            let mut expected = vec![0x5000];
            if printer == SPRINTF_S {
                expected.push(0x104);
            }
            expected.extend([format, 2, 1, 3, (-2i32) as u32, 5]);
            assert_eq!(log_of(&e, printer), vec![expected]);
        }
    }

    #[test]
    fn get_texture_needs_a_lit_property() {
        for (kind, which, texture, message) in [
            (8u32, 0u32, 0x7100u32, 0usize),
            (12, 1, 0x7200, 0),
            (13, 0, 0, 1),
            (7, 0, 0, 1),
        ] {
            let mut e = engine(&[0x005b_5e40]);
            answer(&mut e, 0x0043_8220, 0x99);
            answer(&mut e, 0x00a5_9d30, 0x6000);
            answer(&mut e, 0x0044_1110, kind);
            answer(&mut e, 0x0054_0540, 0x7100);
            answer(&mut e, 0x0054_0570, 0x7200);
            answer(&mut e, 0x0047_d3f0, 0xfffe);
            answer(&mut e, 0x004a_8ae0, 3);
            answer(&mut e, 0x0072_6070, 16);
            let chunk = e.new_object::<BGSTerrainChunk>();
            e.set(chunk, BGSTerrainChunk::spMesh, 0x5500);
            e.set(chunk, BGSTerrainChunk::pNode, 0x5600);
            assert_eq!(e.call(0x006f_abc0, &args![chunk, which]).u32(), texture);
            let printed = log_of(&e, 0x005b_5e40);
            assert_eq!(printed.len(), message);
            if message == 1 {
                assert_eq!(printed[0], vec![NO_LIGHTING_MESSAGE, 16, 3, (-2i32) as u32]);
            }
            assert_eq!(log_of(&e, 0x00a5_9d30), vec![vec![0x5500, 0x99]]);
        }
        // Without a mesh: nothing.
        let mut e = engine(&[]);
        let chunk = e.new_object::<BGSTerrainChunk>();
        assert_eq!(e.call(0x006f_abc0, &args![chunk, 0u32]).u32(), 0);
    }

    const PARENT_ATTACH: u32 = 0x0f80_0200;
    const SHAPE_CENTER: u32 = 0x0f80_0210;

    /// The callees of `006facb0` and what it calls through.
    fn attach_engine() -> (Engine, Ptr<BGSTerrainChunk>) {
        let stubs = [
            0x00a5_a040u32,
            0x0043_d410,
            0x00a5_9c60,
            0x0043_9680,
            POINTER_ASSIGN,
            0x004a_ddc0,
            0x0043_9920,
            0x0090_5820,
            0x0045_0f90,
            SHAPE_CENTER,
            PARENT_ATTACH,
        ];
        let mut e = engine(&stubs);
        let parent = object(&mut e, 8, &[(0xdc, PARENT_ATTACH)]);
        answer(&mut e, 0x004e_a9a0, parent);
        let shape = object(&mut e, 0x24, &[(0xb8, SHAPE_CENTER)]);
        answer(&mut e, 0x00c3_7ec0, shape);
        answer(&mut e, 0x00c4_6970, 0x6400);
        answer(&mut e, 0x00c3_5e00, 0x6500);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 1.0);
        e.mem.set_f32(point + 4, 2.0);
        e.mem.set_f32(point + 8, 3.0);
        answer(&mut e, 0x0066_29f0, 0);
        answer(&mut e, 0x0043_b230, 0);
        answer(&mut e, 0x0048_d150, point);
        answer(&mut e, 0x0050_0940, point);
        e.register(0x0045_36e0, |_, _| Ret {
            st0: 9.0,
            ..Ret::default()
        });
        answer(&mut e, 0x006f_a840, 1);
        e.map(CHUNK_COUNT & !0xfff, 0x1000);
        e.map(WATER_SWITCH & !0xfff, 0x1000);
        let chunk = e.new_object::<BGSTerrainChunk>();
        let node = e.mem.alloc(0x60);
        let manager = e.mem.alloc(4);
        e.mem.set_u32(node, manager);
        e.mem.set_u8(node + 0x5c, 1);
        e.set(chunk, BGSTerrainChunk::pNode, node);
        e.set(chunk, BGSTerrainChunk::bDoneLoading, 1);
        e.set(chunk, BGSTerrainChunk::bPrepared, 1);
        e.set(chunk, BGSTerrainChunk::spMesh, 0x5100);
        (e, chunk)
    }

    #[test]
    fn attach_without_a_node_attaches_the_mesh() {
        let (mut e, chunk) = attach_engine();
        e.call(0x006f_acb0, &args![chunk]);
        let attach = log_of(&e, PARENT_ATTACH);
        assert_eq!(attach.len(), 1);
        assert_eq!(attach[0][1..], [0x5100, 1]);
        assert_eq!(e.global::<u32>(CHUNK_COUNT), 1);
        assert_eq!(e.get(chunk, BGSTerrainChunk::bAttached), 1);
        let updates = log_of(&e, 0x0043_d410);
        assert_eq!(updates.len(), 2);
        assert_eq!(updates[0][1..], [0, 0, 0]);
        assert_eq!(log_of(&e, 0x0090_5820).len(), 1);
        // Shown: 00450f90 with 1 on the mesh.
        assert_eq!(log_of(&e, 0x0045_0f90), vec![vec![0x5100, 1]]);
        // Already attached: nothing more.
        e.call_log = Some(vec![]);
        e.call(0x006f_acb0, &args![chunk]);
        assert!(log_of(&e, PARENT_ATTACH).is_empty());
    }

    #[test]
    fn attach_with_a_node_builds_the_two_water_bounds() {
        let (mut e, chunk) = attach_engine();
        e.set(chunk, BGSTerrainChunk::spNode, 0x5300);
        e.register(SHAPE_CENTER, |e, a| {
            // The centre's height is 004536e0's answer.
            assert_eq!(e.mem.f32(a[1] + 8), 9.0);
            Ret::default()
        });
        e.call(0x006f_acb0, &args![chunk]);
        let attach = log_of(&e, PARENT_ATTACH);
        assert_eq!(attach[0][1..], [0x5300, 1]);
        assert_eq!(log_of(&e, SHAPE_CENTER).len(), 2);
        assert_eq!(
            log_of(&e, POINTER_ASSIGN),
            vec![
                vec![chunk.addr() + 0x14, 0x6400],
                vec![chunk.addr() + 0x18, 0x6400]
            ]
        );
        assert_eq!(log_of(&e, 0x004a_ddc0).len(), 2);
    }

    #[test]
    fn node_byte_reads_offset_0x5c() {
        let mut e = engine(&[]);
        let node = e.mem.alloc(0x60);
        e.mem.set_u8(node + 0x5c, 7);
        assert_eq!(e.call(0x006f_b150, &args![node]).u32(), 7);
    }

    const ROOT_FN: u32 = 0x0f80_0300;
    const IS_NODE_FN: u32 = 0x0f80_0310;
    const MULTI_BOUND_FN: u32 = 0x0f80_0320;
    const SEGMENTED_FN: u32 = 0x0f80_0330;

    #[test]
    fn water_references_are_made_for_each_shape() {
        let mut e = engine(&[POINTER_ASSIGN, 0x00a8_72d0, IS_NODE_FN]);
        e.map(WATER_SWITCH & !0xfff, 0x1000);
        e.map(WATER_SYSTEM_HOLDER & !0xfff, 0x1000);
        e.set_global(WATER_SYSTEM_HOLDER, 0x4400u32);
        answer(&mut e, WATER_SYSTEM_GET, 0x4500);
        let child = object(&mut e, 8, &[(0x14, MULTI_BOUND_FN)]);
        answer(&mut e, MULTI_BOUND_FN, 0x4600);
        answer(&mut e, 0x0043_b4a0, child);
        let reference = object(&mut e, 8, &[(0x1d0, ROOT_FN)]);
        let root = object(&mut e, 8, &[(0xc, IS_NODE_FN)]);
        answer(&mut e, ROOT_FN, root);
        answer(&mut e, 0x004e_4c80, reference);
        answer(&mut e, 0x0043_b230, 0x4700);
        answer(&mut e, SEGMENTED_FN, 0x4800);
        let shape = object(&mut e, 8, &[(0x28, SEGMENTED_FN)]);
        let chunk = e.new_object::<BGSTerrainChunk>();
        let node = e.mem.alloc(8);
        let manager = e.mem.alloc(8);
        e.mem.set_u32(manager, 0x4900);
        e.mem.set_u32(node, manager);
        e.set(chunk, BGSTerrainChunk::pNode, node);
        e.set(chunk, BGSTerrainChunk::spWater, shape);
        e.set(chunk, BGSTerrainChunk::spWaterNode, 0x4a00);
        // Neither the switch nor the argument: nothing.
        e.call(0x006f_b170, &args![chunk, 0u32]);
        assert!(log_of(&e, 0x004e_4c80).is_empty());
        e.call(0x006f_b170, &args![chunk, 1u32]);
        assert_eq!(
            log_of(&e, 0x004e_4c80),
            vec![vec![0x4500, shape, 0x4900, 0x4700, 0x4a00, 0]]
        );
        assert_eq!(log_of(&e, WATER_SYSTEM_GET), vec![vec![0x4400]]);
        assert_eq!(e.get(chunk, BGSTerrainChunk::pLODWaterRef), reference);
        assert_eq!(
            log_of(&e, POINTER_ASSIGN),
            vec![
                vec![chunk.addr() + 0x14, 0],
                vec![chunk.addr() + 0x14, 0x4600]
            ]
        );
        assert_eq!(log_of(&e, 0x0043_b4a0), vec![vec![0, 0]]);
        assert_eq!(log_of(&e, 0x00a8_72d0), vec![vec![0x4800]]);
    }

    const DETACH_FN: u32 = 0x0f80_0400;
    const DESTROY_FN: u32 = 0x0f80_0410;

    #[test]
    fn water_references_are_removed() {
        let mut e = engine(&[0x004e_5140, DETACH_FN, DESTROY_FN]);
        e.map(WATER_SWITCH & !0xfff, 0x1000);
        e.set_global(WATER_SWITCH, 1u8);
        e.map(WATER_SYSTEM_HOLDER & !0xfff, 0x1000);
        answer(&mut e, WATER_SYSTEM_GET, 0x4500);
        let bound = object(&mut e, 8, &[(0xe8, DETACH_FN)]);
        answer(&mut e, 0x0043_b230, bound);
        let reference = object(&mut e, 8, &[(0x10, DESTROY_FN)]);
        let chunk = e.new_object::<BGSTerrainChunk>();
        let node = e.mem.alloc(8);
        let manager = e.mem.alloc(8);
        e.mem.set_u32(manager, 0x4900);
        e.mem.set_u32(node, manager);
        e.set(chunk, BGSTerrainChunk::pNode, node);
        e.set(chunk, BGSTerrainChunk::spWaterAtReflectionHeight, 0x4c00);
        e.set(chunk, BGSTerrainChunk::spWaterReflNode, 0x4d00);
        e.set(
            chunk,
            BGSTerrainChunk::pLODWaterAtReflectionHeightRef,
            reference,
        );
        e.call(0x006f_b3d0, &args![chunk, 0u32]);
        assert_eq!(
            log_of(&e, 0x004e_5140),
            vec![vec![0x4500, reference, 0x4900, bound]]
        );
        assert_eq!(log_of(&e, DETACH_FN), vec![vec![bound, 0x4d00]]);
        assert_eq!(log_of(&e, DESTROY_FN), vec![vec![reference, 1]]);
        assert_eq!(
            e.get(chunk, BGSTerrainChunk::pLODWaterAtReflectionHeightRef),
            0
        );
    }

    #[test]
    fn show_and_hide_touch_the_present_geometry() {
        for (addr, value) in [(0x006f_b6e0u32, 1u32), (0x006f_b760, 0)] {
            let mut e = engine(&[0x0045_0f90]);
            let chunk = e.new_object::<BGSTerrainChunk>();
            e.set(chunk, BGSTerrainChunk::spMesh, 0x100);
            e.set(chunk, BGSTerrainChunk::spWaterAtReflectionHeight, 0x300);
            e.call(addr, &args![chunk]);
            assert_eq!(
                log_of(&e, 0x0045_0f90),
                vec![vec![0x100, value], vec![0x300, value]]
            );
        }
    }

    const MAKE_FN: u32 = 0x0f80_0500;
    const PRECACHE_FN: u32 = 0x0f80_0510;

    #[test]
    fn prepare_sets_up_the_lighting_property_once() {
        let mut e = engine(&[
            0x0044_1130,
            0x00b5_7e30,
            POINTER_ASSIGN,
            0x0044_10d0,
            0x00a6_7050,
            0x0044_10f0,
            0x00e7_4120,
            PRECACHE_FN,
        ]);
        let property = e.mem.alloc(0x100);
        answer(&mut e, 0x0043_8220, 0x99);
        answer(&mut e, 0x00a5_9d30, property);
        answer(&mut e, 0x0054_95f0, 0x4100);
        let renderer = object(&mut e, 8, &[(0x118, MAKE_FN)]);
        answer(&mut e, 0x0040_30b0, renderer);
        answer(&mut e, MAKE_FN, 0x4200);
        let precache = object(&mut e, 8, &[(0xec, PRECACHE_FN)]);
        answer(&mut e, 0x0043_c4b0, precache);
        let chunk = e.new_object::<BGSTerrainChunk>();
        e.set(chunk, BGSTerrainChunk::spMesh, 0x5100);
        e.call(0x006f_b7e0, &args![chunk]);
        assert_eq!(log_of(&e, 0x0044_1130), vec![vec![property, 0x21, 1]]);
        assert_eq!(e.mem.f32(property + 0xf8), 1.0);
        assert_eq!(log_of(&e, 0x00b5_7e30), vec![vec![0x5100, 0, 0]]);
        assert_eq!(
            log_of(&e, POINTER_ASSIGN),
            vec![vec![chunk.addr() + 0x2c, property]]
        );
        assert_eq!(log_of(&e, 0x00a6_7050), vec![vec![0x4100, 0x4000]]);
        assert_eq!(log_of(&e, MAKE_FN), vec![vec![renderer, 0x5100, property]]);
        assert_eq!(
            log_of(&e, PRECACHE_FN),
            vec![vec![precache, 0x5100, 0, 0, 0x4200]]
        );
        assert_eq!(log_of(&e, 0x00e7_4120), vec![vec![precache]]);
        assert_eq!(e.get(chunk, BGSTerrainChunk::bPrepared), 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_b7e0, &args![chunk]);
        assert!(log_of(&e, 0x0044_1130).is_empty());
    }

    #[test]
    fn three_floats_go_to_0xf0() {
        let mut e = engine(&[]);
        let p = e.mem.alloc(0x100);
        e.call(0x006f_b940, &args![p, 0x7f80_0001u32, 2.0f32, 3.0f32]);
        assert_eq!(e.mem.u32(p + 0xf0), 0x7fc0_0001);
        assert_eq!(e.mem.f32(p + 0xf4), 2.0);
        assert_eq!(e.mem.f32(p + 0xf8), 3.0);
    }

    #[test]
    fn load_task_constructor_fills_the_task_and_names_it() {
        let mut e = engine(&[
            0x00c3_ce60,
            0x0063_3c90,
            0x00c3_cee0,
            0x00c3_cf60,
            SPRINTF_S,
        ]);
        let world = object(&mut e, 8, &[(0x130, MAKE_FN)]);
        answer(&mut e, MAKE_FN, 0x4300);
        let task = e.new_object::<BGSTerrainChunkLoadTask>();
        let made = e
            .call(
                0x006f_b980,
                &args![task, 0x6000u32, world, 0x0001_fffeu32, 2u32, 4u32],
            )
            .u32();
        assert_eq!(made, task.addr());
        assert_eq!(log_of(&e, 0x00c3_ce60), vec![vec![task.addr(), 3]]);
        assert_eq!(e.mem.u32(task.addr()), LOAD_TASK_VTABLE);
        assert_eq!(e.get(task, BGSTerrainChunkLoadTask::iX), 0x0001_fffe);
        assert_eq!(e.get(task, BGSTerrainChunkLoadTask::pChunk), 0x6000);
        assert_eq!(log_of(&e, 0x0063_3c90).len(), 4);
        let printed = log_of(&e, SPRINTF_S);
        assert_eq!(printed[0][5..], [4, (-2i32) as u32, 2]);
        let named = log_of(&e, 0x00c3_cee0);
        assert_eq!(named[0][1], printed[0][0] + 5);
        assert_eq!(log_of(&e, 0x00c3_cf60), vec![vec![task.addr(), 0]]);
    }

    #[test]
    fn load_task_destructors() {
        let mut e = engine(&[POINTER_ASSIGN, 0x0045_cec0, 0x00c3_cea0, 0x0040_1030]);
        let task = e.new_object::<BGSTerrainChunkLoadTask>();
        let t = task.addr();
        e.call(0x006f_bae0, &args![task, 1u32]);
        assert_eq!(
            log_of(&e, POINTER_ASSIGN),
            vec![vec![t + 0x48, 0], vec![t + 0x4c, 0], vec![t + 0x50, 0]]
        );
        assert_eq!(
            log_of(&e, 0x0045_cec0),
            vec![
                vec![t + 0x54],
                vec![t + 0x50],
                vec![t + 0x4c],
                vec![t + 0x48]
            ]
        );
        assert_eq!(log_of(&e, 0x00c3_cea0), vec![vec![t]]);
        assert_eq!(log_of(&e, 0x0040_1030), vec![vec![t]]);
        e.call_log = Some(vec![]);
        e.call(0x006f_bae0, &args![task, 0u32]);
        assert!(log_of(&e, 0x0040_1030).is_empty());
        e.call(0x006f_bb10, &args![task]);
        assert_eq!(log_of(&e, 0x00c3_cea0).len(), 2);
    }

    const QUEUED_FN: u32 = 0x0f80_0600;

    #[test]
    fn queue_me_queues_both_textures() {
        let mut e = engine(&[SPRINTF, QUEUE_TEXTURE, QUEUED_FN]);
        e.map(MODEL_LOADER & !0xfff, 0x1000);
        e.set_global(MODEL_LOADER, 0x4400u32);
        answer(&mut e, 0x0043_cc60, 5);
        let world = object(&mut e, 8, &[(0x130, MAKE_FN)]);
        answer(&mut e, MAKE_FN, 0x4300);
        let task = Ptr::<BGSTerrainChunkLoadTask>::new(object(&mut e, 0x60, &[(0x28, QUEUED_FN)]));
        e.set(task, BGSTerrainChunkLoadTask::pWorldSpace, world);
        e.set(task, BGSTerrainChunkLoadTask::iLevel, 8);
        e.call(0x006f_bbd0, &args![task]);
        let printed = log_of(&e, SPRINTF);
        assert_eq!(printed.len(), 2);
        assert_eq!((printed[0][1], printed[1][1]), (DIFFUSE_PATH, NORMALS_PATH));
        let queued = log_of(&e, QUEUE_TEXTURE);
        assert_eq!(queued[0], vec![0x4400, printed[0][0], 5, task.addr()]);
        assert_eq!(queued[1], vec![0x4400, printed[1][0], 5, task.addr()]);
        assert_ne!(printed[0][0], printed[1][0]);
        assert_eq!(e.get(task, BGSTerrainChunkLoadTask::bFinishedQueuing), 1);
        assert_eq!(log_of(&e, QUEUED_FN).len(), 1);
    }

    const CHILD_GEOMETRY_FN: u32 = 0x0f80_0700;

    fn run_engine(is_node: u32, children: u32) -> (Engine, Ptr<BGSTerrainChunkLoadTask>) {
        let mut e = engine(&[
            0x0040_4eb0,
            0x0043_cfd0,
            POINTER_ASSIGN,
            0x0043_d090,
            0x0040_4ee0,
        ]);
        answer(&mut e, 0x00c3_cff0, 0x4100);
        answer(&mut e, 0x0045_cd60, 0x4200);
        answer(&mut e, 0x00c3_a8a0, 1);
        let first = object(&mut e, 8, &[(0xc, IS_NODE_FN)]);
        answer(&mut e, IS_NODE_FN, is_node);
        answer(&mut e, 0x0043_b200, first);
        answer(&mut e, 0x0043_b1e0, 2);
        let child = object(&mut e, 8, &[(0x18, CHILD_GEOMETRY_FN)]);
        // The children's geometry is not segmented (slot +0x28 answers 0);
        // the pointer assignment stores the object.
        let water = object(&mut e, 8, &[(0x28, SEGMENTED_FN)]);
        answer(&mut e, SEGMENTED_FN, 0);
        answer(&mut e, CHILD_GEOMETRY_FN, water);
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.mem.set_u32(water + 4, 0x4300);
        answer(&mut e, 0x0043_b4a0, child);
        answer(&mut e, 0x0045_3470, children);
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, 1);
        answer(&mut e, SETTING_VALUE, setting);
        let task = e.new_object::<BGSTerrainChunkLoadTask>();
        (e, task)
    }

    #[test]
    fn run_takes_a_loaded_node_apart() {
        let (mut e, task) = run_engine(1, 3);
        e.call(0x006f_bd00, &args![task]);
        let t = task.addr();
        let assigned = log_of(&e, POINTER_ASSIGN);
        assert_eq!(assigned.len(), 4);
        assert_eq!(assigned[0][0], t + 0x54);
        assert_eq!(assigned[1][0], t + 0x48);
        let water = e.mem.u32(t + 0x4c);
        assert_eq!(e.mem.u32(water + 4), 0x4300);
        assert_eq!(assigned[2], vec![t + 0x4c, water]);
        assert_eq!(assigned[3], vec![t + 0x50, water]);
        assert_eq!(log_of(&e, SEGMENTED_FN), vec![vec![water]]);
        let guard = log_of(&e, 0x0040_4eb0);
        assert_eq!(guard[0][1..], [0x23, 1, SOURCE_FILE, 0x298]);
        assert_eq!(log_of(&e, 0x00c3_cff0), vec![vec![t, 0, 1]]);
        let load = log_of(&e, 0x00c3_a8a0);
        assert_eq!(load[0][1..], [0x4200, 0x4100]);
        assert_eq!(
            order(&e, &[0x0043_d090, 0x0040_4ee0]),
            vec![0x0043_d090, 0x0040_4ee0]
        );
    }

    #[test]
    fn run_takes_plain_objects_as_mesh_and_water() {
        let (mut e, task) = run_engine(0, 0);
        e.call(0x006f_bd00, &args![task]);
        let t = task.addr();
        let assigned = log_of(&e, POINTER_ASSIGN);
        assert_eq!(assigned.len(), 2);
        assert_eq!(assigned[0][0], t + 0x48);
        assert_eq!(assigned[1][0], t + 0x4c);
    }

    #[test]
    fn segment_count_reads_the_flag_or_the_count() {
        let mut e = engine(&[]);
        let shape = e.mem.alloc(0xd0);
        e.mem.set_u32(shape + 0xc8, 5);
        assert_eq!(e.call(0x006f_bfe0, &args![shape]).u32(), 5);
        e.mem.set_u8(shape + 0xcd, 1);
        assert_eq!(e.call(0x006f_bfe0, &args![shape]).u32(), 1);
    }

    #[test]
    fn finish_names_the_objects_then_hands_over() {
        let takes = [0x006f_aa30u32, 0x006f_6250, 0x006f_aa50, 0x006f_aa70];
        let mut stubs = takes.to_vec();
        stubs.extend([0x006f_74f0, 0x0092_9260, SPRINTF, 0x0043_81b0, 0x00a5_b950]);
        let mut e = engine(&stubs);
        e.register(0x0043_8170, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        answer(&mut e, 0x0043_b1b0, 0x4700);
        let task = e.new_object::<BGSTerrainChunkLoadTask>();
        let chunk = e.new_object::<BGSTerrainChunk>();
        let node = e.mem.alloc(4);
        e.set(chunk, BGSTerrainChunk::pNode, node);
        e.set(task, BGSTerrainChunkLoadTask::pChunk, chunk.addr());
        e.set(task, BGSTerrainChunkLoadTask::spMesh, 0x100);
        e.set(
            task,
            BGSTerrainChunkLoadTask::spWaterAtReflectionHeight,
            0x300,
        );
        e.set(task, BGSTerrainChunkLoadTask::iX, 4);
        e.set(task, BGSTerrainChunkLoadTask::iY, 8);
        e.set(task, BGSTerrainChunkLoadTask::iLevel, 16);
        e.call(0x006f_c020, &args![task]);
        let printed = log_of(&e, SPRINTF);
        assert_eq!(printed.len(), 2);
        assert_eq!(printed[0][1..], [0x4700, 16, 4, 8, 19, 23]);
        let formats: Vec<u32> = log_of(&e, 0x0043_8170).iter().map(|c| c[1]).collect();
        assert_eq!(formats[0], TERRAIN_NAME);
        assert_eq!(formats[2], REFLECTED_WATER_NAME);
        let named: Vec<u32> = log_of(&e, 0x00a5_b950).iter().map(|c| c[0]).collect();
        assert_eq!(named, vec![0x100, 0x300]);
        assert_eq!(log_of(&e, 0x006f_74f0).len(), 1);
        // Canceled: the pointers are cleared instead.
        let mut e = engine(&[POINTER_ASSIGN]);
        let task = e.new_object::<BGSTerrainChunkLoadTask>();
        e.set(task, BGSTerrainChunkLoadTask::spMesh, 0x100);
        e.set(task, BGSTerrainChunkLoadTask::bCanceled, 1);
        e.call(0x006f_c020, &args![task]);
        assert_eq!(log_of(&e, POINTER_ASSIGN).len(), 3);
    }

    #[test]
    fn file_index_offsets_the_archive_index() {
        let mut e = engine(&[]);
        answer(&mut e, 0x0055_b980, 0x4100);
        answer(&mut e, 0x00af_6910, 0x4200);
        answer(&mut e, 0x0043_c030, 6);
        assert_eq!(e.call(0x006f_c440, &args![0x5000u32]).u32(), 10);
        assert_eq!(log_of(&e, 0x00af_6910), vec![vec![0x4100, 1]]);
        answer(&mut e, 0x00af_6910, 0);
        assert_eq!(e.call(0x006f_c440, &args![0x5000u32]).u32(), 1);
        answer(&mut e, 0x0055_b980, 0);
        assert_eq!(e.call(0x006f_c440, &args![0x5000u32]).u32(), 1);
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Two chunks in the list (nodes keep the item at `+4` and the next node at `+8`; the second
    /// node points at an empty third). Chunk 1's node byte is set, chunk 2's is clear; each
    /// chunk has only a mesh. Returns the two meshes.
    fn chunk_list(e: &mut Engine) -> (u32, u32) {
        e.register(DEREF, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_IS_EMPTY_008256D0, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(LIST_ITEM_006815C0, |_, a| (a[0] + 4).into_ret());
        e.register(LIST_NEXT_00726070, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0045_0f90, |_, _| Ret::default());
        e.map(CHUNK_LIST_011D8624, 0x10);
        let second = e.mem.alloc(0x10);
        let third = e.mem.alloc(0x10);
        e.mem.set_u8(third, 1);
        let mut meshes = vec![];
        for (node_at, next, flag) in [(CHUNK_LIST_011D8624, second, 1u8), (second, third, 0)] {
            let chunk = e.mem.alloc(0x30);
            let terrain_node = e.mem.alloc(0x60);
            e.mem.set_u8(terrain_node + 0x5c, flag);
            e.mem.set_u32(chunk, terrain_node);
            let mesh_pointer = chunk + 4;
            e.mem.set_u32(mesh_pointer, 0x0e00 + u32::from(flag));
            meshes.push(0x0e00 + u32::from(flag));
            e.mem.set_u32(node_at + 4, chunk);
            e.mem.set_u32(node_at + 8, next);
        }
        (meshes[0], meshes[1])
    }

    #[test]
    fn n089_006fb5d0_hides_or_shows_each_listed_chunk() {
        let mut e = Engine::new();
        let (flagged, clear) = chunk_list(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006f_b5d0, &args![]);
        assert_eq!(
            calls_to(&e, 0x0045_0f90),
            vec![vec![flagged, 0], vec![clear, 1]]
        );
        assert_eq!(calls_to(&e, LIST_IS_EMPTY_008256D0).len(), 3);
        assert_eq!(calls_to(&e, LIST_NEXT_00726070).len(), 2);
    }

    #[test]
    fn n089_006fb640_shows_chunks_whose_node_is_ready() {
        let mut e = Engine::new();
        let (flagged, clear) = chunk_list(&mut e);
        e.register(0x006f_a840, |_, _| 1u32.into_ret());
        e.register(0x006f_e920, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_b640, &args![]);
        assert_eq!(
            calls_to(&e, 0x0045_0f90),
            vec![vec![flagged, 1], vec![clear, 0]]
        );
        let first = calls_to(&e, 0x006f_e920);
        assert_eq!(first.len(), 1);
        assert_eq!(first[0][1], 0);
        assert_eq!(calls_to(&e, 0x006f_a840)[0], vec![first[0][0]]);
        // A refusal from either check hides the chunk.
        e.register(0x006f_e920, |_, _| 0x100u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_b640, &args![]);
        assert_eq!(calls_to(&e, 0x0045_0f90)[0], vec![flagged, 0]);
        e.register(0x006f_a840, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x006f_b640, &args![]);
        assert!(calls_to(&e, 0x006f_e920).is_empty());
    }

    #[test]
    fn n089_006fbcd0_cancels_the_task() {
        let mut e = Engine::new();
        e.register(0x00c3_cb70, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        e.call(0x006f_bcd0, &args![this, 5u32, 6u32]);
        assert_eq!(e.mem.u8(this + 0x58), 1);
        assert_eq!(calls_to(&e, 0x00c3_cb70), vec![vec![this, 5, 6]]);
    }

    #[test]
    fn n089_006fc400_describes_the_task() {
        let mut e = Engine::new();
        e.register(SPRINTF_S, |_, _| 0x55u32.into_ret());
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x34, 7);
        e.mem.set_u32(this + 0x38, 0xffff_fff9);
        e.mem.set_u32(this + 0x3c, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_c400, &args![this, 0xb0u32, 0x40u32]).u8(), 1);
        assert_eq!(
            calls_to(&e, SPRINTF_S),
            vec![vec![0xb0, 0x40, 0x0106_e2a4, 7, 0xffff_fff9, 2]]
        );
    }
}
