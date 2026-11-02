//! `fallout shared/distant terrain system/bgsdistantobjectblock.cpp` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSDistantObjectBlock` (Xbox PDB, 0x2C bytes): `pNode` (the
//! `BGSTerrainNode`, +0), the `NiPointer`s `spMesh` (+4), `spNode` (+8),
//! `spOldMesh` (+0xC), `spNodeStinger` (+0x10), `spNodeStingerOld`
//! (+0x14), `spLoadTask` (+0x18) and `spLoadTaskStinger` (+0x1C), the
//! flags `bDoneLoading` (+0x20), `bAttached` (+0x21), `bPrepared` (+0x22),
//! `bHighPriority` (+0x23), `bDetachFlag` (+0x24) and `nDetachCount`
//! (+0x28); the statics `BlocksLoaded` (`BSSimpleList`, `011d8524`) and two
//! counters (`011d8510`, `011d8514`; the PDB names them
//! `iBlocksLoadedLow`/`iBlocksLoadedHigh`, which is which is not
//! confirmed).
//!
//! `BGSDistantObjectBlockLoadTask` (Xbox PDB, 0x58 bytes): a
//! `QueuedFileEntry` (0x30 bytes) plus `bFinishedQueuing` (+0x30), `iX`,
//! `iY`, `iLevel` (+0x34, +0x38, +0x3C), `pBlock` (+0x40), `pWorldSpace`
//! (+0x44), `spMesh` (+0x48), `spNode` (+0x4C), `bCanceled` (+0x50) and
//! `bStingerVersion` (+0x51). The PC code uses the same offsets.
//!
//! An `NiPointer`'s object is read through `00559450` (its first word), as
//! the code does; `0066b0d0` assigns one, `006e5cc0` copies one into
//! another and `006f74f0` assigns a load task pointer. The compiler's
//! exception frames (`FS:[0]`) and stack cookie checks (`00ec408c`) are
//! not translated; the functions that have them carry a `Translation
//! incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

// Offsets in `BGSDistantObjectBlock` (Xbox PDB names).
const BLOCK_NODE: u32 = 0x0;
const BLOCK_MESH: u32 = 0x4;
const BLOCK_SCENE_NODE: u32 = 0x8;
const BLOCK_OLD_MESH: u32 = 0xc;
const BLOCK_STINGER: u32 = 0x10;
const BLOCK_OLD_STINGER: u32 = 0x14;
const BLOCK_LOAD_TASK: u32 = 0x18;
const BLOCK_LOAD_TASK_STINGER: u32 = 0x1c;
const BLOCK_DONE_LOADING: u32 = 0x20;
const BLOCK_ATTACHED: u32 = 0x21;
const BLOCK_PREPARED: u32 = 0x22;
const BLOCK_HIGH_PRIORITY: u32 = 0x23;
const BLOCK_DETACH_FLAG: u32 = 0x24;
const BLOCK_DETACH_COUNT: u32 = 0x28;

// Offsets in `BGSDistantObjectBlockLoadTask` (Xbox PDB names).
const TASK_FINISHED_QUEUING: u32 = 0x30;
const TASK_X: u32 = 0x34;
const TASK_Y: u32 = 0x38;
const TASK_LEVEL: u32 = 0x3c;
const TASK_BLOCK: u32 = 0x40;
const TASK_WORLD_SPACE: u32 = 0x44;
const TASK_MESH: u32 = 0x48;
const TASK_NODE: u32 = 0x4c;
const TASK_CANCELED: u32 = 0x50;
const TASK_STINGER_VERSION: u32 = 0x51;

/// `BGSDistantObjectBlock::BlocksLoaded` (Xbox PDB static), a `BSSimpleList`.
const BLOCKS_LOADED: u32 = 0x011d_8524;
/// The two counters of loaded blocks (see the module comment).
const COUNTER_011D8510: u32 = 0x011d_8510;
const COUNTER_011D8514: u32 = 0x011d_8514;
/// `BGSDistantObjectBlockLoadTask`'s vtable (RTTI `.?AVBGSDistantObjectBlockLoadTask@@`).
const LOAD_TASK_VTABLE: u32 = 0x0106_dc3c;
/// The player (`PlayerCharacter *`).
const PLAYER: u32 = 0x011d_ea3c;
/// The task manager pointer (`01202d98`) whose `CancelTask` the reloads call.
const TASK_MANAGER: u32 = 0x0120_2d98;
/// The model loader pointer (`011c3b3c`) `QueueMe` queues the textures with.
const MODEL_LOADER: u32 = 0x011c_3b3c;

/// The file-name formats (`Data\Meshes\Landscape\LOD\%s\Blocks\...`).
const BLOCK_FILE_FORMAT: u32 = 0x0106_da7c;
const BLOCK_FILE_HIGH_PRIORITY_FORMAT: u32 = 0x0106_dab8;
const DIFFUSE_TEXTURE_FORMAT: u32 = 0x0106_dbc4;
const NORMAL_TEXTURE_FORMAT: u32 = 0x0106_dbfc;
/// The source file name the memory-context guards get.
const SOURCE_FILE_NAME: u32 = 0x0106_da18;
/// `"MODELS: Found a distant object block for %i:%i,%i but it is ..."`.
const EMPTY_BLOCK_MESSAGE: u32 = 0x0106_dc70;

/// The world space's vtable slot answering its name (+0x130, no words).
const WORLD_SPACE_SLOT_NAME: u32 = 0x130;
/// The parent node's vtable slots: detach a child (+0xE8) and attach one (+0xDC, child and 1).
const NODE_SLOT_DETACH_CHILD: u32 = 0xe8;
const NODE_SLOT_ATTACH_CHILD: u32 = 0xdc;

// Callees outside this unit.
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// cdecl (buffer, size, format, ...): formatted print into the buffer.
const PRINT_FORMATTED: u32 = 0x0040_6d00;
/// thiscall on an `NiPointer`: the object it holds.
const POINTER_GET: u32 = 0x0055_9450;
/// thiscall (pointer, object): `NiPointer` assignment.
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// thiscall (pointer, other pointer): `NiPointer` copy.
const POINTER_COPY: u32 = 0x006e_5cc0;
/// thiscall (pointer, task): load task `NiPointer` assignment.
const TASK_POINTER_ASSIGN: u32 = 0x006f_74f0;
/// thiscall (pointer, object): `NiPointer` constructor.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// thiscall: `NiPointer` destructor.
const POINTER_DESTROY: u32 = 0x0045_cec0;
/// thiscall on a list node: answers the node itself.
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
/// thiscall on a list node: the next node (+4).
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// thiscall (list, &item): adds the item to a `BSSimpleList`.
const SIMPLE_LIST_ADD: u32 = 0x0090_5820;
/// thiscall (list, &item): removes the item from a `BSSimpleList`.
const SIMPLE_LIST_REMOVE: u32 = 0x0090_5330;
/// thiscall on the terrain node's object: the parent node the meshes attach to.
const TERRAIN_PARENT_NODE: u32 = 0x004e_a9c0;
/// thiscall on a mesh: the object detached in its place (`009611e0`).
const MESH_DETACH_TARGET: u32 = 0x0096_11e0;
/// thiscall on a block: a check `0006f5840` skips blocks on (`00543c30`).
const BLOCK_SKIP_CHECK: u32 = 0x0054_3c30;
/// thiscall chain from a block's node to the model data it refreshes:
/// `006629f0`, `0043b230`, `00528820`.
const NODE_DATA_6629F0: u32 = 0x0066_29f0;
const MODEL_ROOT: u32 = 0x0043_b230;
const REFRESH_528820: u32 = 0x0052_8820;
/// thiscall (task, pointer): copies the task's mesh into the pointer (`006faa30`).
const TASK_COPY_MESH: u32 = 0x006f_aa30;
/// thiscall on a load task: whether it has finished (`00449110`).
const TASK_IS_FINISHED: u32 = 0x0044_9110;
/// The memory-context guard's constructor (`(guard, context, 1, file, line)`) and destructor.
const MEMORY_CONTEXT_ENTER: u32 = 0x0040_4eb0;
const MEMORY_CONTEXT_LEAVE: u32 = 0x0040_4ee0;
/// `BSTaskManager<__int64>::CancelTask` (engine map), thiscall (manager, task, 0).
const CANCEL_TASK: u32 = 0x0044_ac40;
/// The load task's vtable slot that queues it (+0x20, no words).
const TASK_SLOT_QUEUE: u32 = 0x20;
/// thiscall (pointer, other): whether the two meshes differ (`00631820`).
const MESHES_DIFFER: u32 = 0x0063_1820;
/// cdecl: the property type removed before the new one (`0040df90`).
const PROPERTY_TYPE_40DF90: u32 = 0x0040_df90;
/// `NiAVObject::RemoveProperty_ov2` (engine map), thiscall (object, type).
const REMOVE_PROPERTY: u32 = 0x00a5_b230;
/// cdecl (size): the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The property's constructor (`004391c0`, 0x1C bytes) and its setters.
const PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
const PROPERTY_SET_4393C0: u32 = 0x0043_93c0;
const PROPERTY_SET_49ED90: u32 = 0x0049_ed90;
const PROPERTY_SET_94DB80: u32 = 0x0094_db80;
const PROPERTY_SET_4393E0: u32 = 0x0043_93e0;
/// `NiAVObject::AttachProperty` (engine map), thiscall (object, property).
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// cdecl: the shader property type (`00438220`).
const SHADER_PROPERTY_TYPE: u32 = 0x0043_8220;
/// `NiAVObject::GetProperty` (engine map), thiscall (object, type).
const GET_PROPERTY: u32 = 0x00a5_9d30;
/// `BSShaderProperty::SetFlag` (engine map), thiscall (property, flag, on).
const SHADER_SET_FLAG: u32 = 0x0044_1130;
/// The shader property's vtable slot that clones it for a mesh (+0xB0, `(mesh, 0, 1)`).
const SHADER_SLOT_CLONE: u32 = 0xb0;
/// thiscall on the shader property: its shader id (`006286d0`).
const SHADER_ID: u32 = 0x0062_86d0;
/// `BSShaderManager::GetShader` (engine map), cdecl (id).
const GET_SHADER: u32 = 0x00b5_5560;
/// thiscall (mesh, shader): sets the mesh's shader (`00874460`).
const SET_SHADER: u32 = 0x0087_4460;
/// thiscall (local, 0.0, 0, 0): builds the update data (`0043d410`, 0xC bytes).
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// `NiAVObject::UpdateProperties` (engine map), thiscall.
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// thiscall (object, data): the downward update (`00a59c60`).
const UPDATE_DOWNWARD: u32 = 0x00a5_9c60;
/// `BGSTerrainNode::UpdateBlockVisibility` (engine map), thiscall (node, 0).
const UPDATE_BLOCK_VISIBILITY: u32 = 0x006f_ee90;
/// thiscall on a mesh: its geometry data (`005495f0`).
const MESH_DATA: u32 = 0x0054_95f0;
const DATA_SET_4410D0: u32 = 0x0044_10d0;
/// `NiGeometryData::SetConsistency` (engine map), thiscall (data, 0x4000).
const DATA_SET_CONSISTENCY: u32 = 0x00a6_7050;
const DATA_SET_4410F0: u32 = 0x0044_10f0;
/// thiscall on a mesh: the object whose slot +0x118 is asked (`004030b0`).
const MESH_OWNER_4030B0: u32 = 0x0040_30b0;
/// cdecl: the renderer (`0043c4b0`).
const RENDERER: u32 = 0x0043_c4b0;
/// The renderer's vtable slot +0xEC (`(mesh, 0, 0, value)`) and the slot +0x118 of the
/// mesh's owner (`(mesh, property)`).
const RENDERER_SLOT_EC: u32 = 0xec;
const OWNER_SLOT_118: u32 = 0x118;
/// thiscall on the renderer (`00e74120`).
const RENDERER_E74120: u32 = 0x00e7_4120;
/// thiscall (block, node): prepares the stinger node (`006f6a90`).
const PREPARE_STINGER: u32 = 0x006f_6a90;
/// The `QueuedFileEntry` constructor (`(entry, 3)`) and destructor.
const QUEUED_FILE_ENTRY_CONSTRUCT: u32 = 0x00c3_ce60;
const QUEUED_FILE_ENTRY_DESTROY: u32 = 0x00c3_cea0;
/// thiscall (entry, name): sets the entry's file name (`00c3cee0`).
const QUEUED_FILE_SET_NAME: u32 = 0x00c3_cee0;
/// thiscall (entry, 0) (`00c3cf60`).
const QUEUED_FILE_C3CF60: u32 = 0x00c3_cf60;
/// `QueuedFileEntry::GetFile` (engine map), thiscall (entry, 0, 1).
const QUEUED_FILE_GET_FILE: u32 = 0x00c3_cff0;
/// thiscall on the entry: its file name (`0045cd60`).
const QUEUED_FILE_NAME: u32 = 0x0045_cd60;
/// The other file-name builders (not functions of this list).
const BUILD_BLOCK_FILE_NAME_PA: u32 = 0x006f_5690;
const BUILD_STINGER_BLOCK_FILE_NAME: u32 = 0x006f_56e0;
const BUILD_STINGER_BLOCK_FILE_NAME_HIGH_PRIORITY: u32 = 0x006f_5730;
/// `FileFinder::Exist` (engine map), cdecl (name, 0, a, b).
const FILE_EXISTS: u32 = 0x0045_6a20;
/// thiscall on the task: the word `QueueTexture` gets (`0043cc60`).
const TASK_QUEUE_WORD: u32 = 0x0043_cc60;
/// `ModelLoader::QueueTexture` (engine map), thiscall (loader, name, word, task).
const QUEUE_TEXTURE: u32 = 0x0044_36c0;
/// The load task's vtable slot `QueueMe` ends with (+0x28, no words).
const TASK_SLOT_28: u32 = 0x28;
/// The stream's constructor, `BSStream::Load` and `BSStream::~BSStream` (engine map).
const STREAM_CONSTRUCT: u32 = 0x0043_cfd0;
const STREAM_LOAD: u32 = 0x00c3_a8a0;
const STREAM_DESTROY: u32 = 0x0043_d090;
/// thiscall (stream, index): a top-level object.
const STREAM_OBJECT: u32 = 0x0043_b200;
/// The loaded object's vtable slot asked before it is kept as a node (+0xC, no words).
const OBJECT_SLOT_C: u32 = 0xc;
/// thiscall (node, 0): the mesh found in a node (`0045bc00`).
const NODE_FIND_MESH: u32 = 0x0045_bc00;
/// thiscall (object, 1) on the terrain node's object (`00929260`).
const TERRAIN_929260: u32 = 0x0092_9260;
/// The mesh's vtable slot `Finish` checks (+0x28, no words).
const MESH_SLOT_28: u32 = 0x28;
/// cdecl (text, ...): prints a message.
const PRINT_MESSAGE: u32 = 0x005b_5e40;

fn pointer(e: &mut Engine, at: u32) -> u32 {
    e.call(POINTER_GET, &args![at]).u32()
}

/// The parent node of a block's meshes: `004ea9c0` on the object of the
/// block's terrain node (`pNode`'s first word).
fn parent_node(e: &mut Engine, block: u32) -> u32 {
    let terrain = e.mem.u32(block + BLOCK_NODE);
    let object = pointer(e, terrain);
    e.call(TERRAIN_PARENT_NODE, &args![object]).u32()
}

/// [`parent_node`] the way `Attach` reaches it (`00559450` on the block, then on its result).
fn parent_node_through_block(e: &mut Engine, block: u32) -> u32 {
    let terrain = pointer(e, block);
    let object = pointer(e, terrain);
    e.call(TERRAIN_PARENT_NODE, &args![object]).u32()
}

/// A world space's name (its vtable +0x130).
fn world_space_name(e: &mut Engine, world_space: u32) -> u32 {
    e.vcall(world_space, WORLD_SPACE_SLOT_NAME, &args![]).u32()
}

// Translated from 006f55f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock::BuildBlockFileName` (Xbox PDB), cdecl `(buffer,
/// world space, level, x, y)`: prints the block's mesh path
/// (`...\%s\Blocks\%s.Level%i.X%i.Y%i.NIF`) into the 0x104-byte buffer,
/// the world space's name twice (asked twice; the second answer is the
/// first `%s`) and the 16-bit coordinates sign-extended.
pub fn bgs_distant_object_block_build_block_file_name(
    e: &mut Engine,
    buffer: u32,
    world_space: u32,
    level: u32,
    x: i16,
    y: i16,
) {
    build_name(e, buffer, world_space, level, x, y, BLOCK_FILE_FORMAT);
}

fn build_name(
    e: &mut Engine,
    buffer: u32,
    world_space: u32,
    level: u32,
    x: i16,
    y: i16,
    format: u32,
) {
    let first = world_space_name(e, world_space);
    let second = world_space_name(e, world_space);
    e.call(
        PRINT_FORMATTED,
        &args![
            buffer,
            0x104u32,
            format,
            second,
            first,
            level,
            i32::from(x),
            i32::from(y)
        ],
    );
}

// Translated from 006f5640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock::BuildBlockFileNameHighPriority` (Xbox PDB):
/// [`bgs_distant_object_block_build_block_file_name`] with the
/// `...Level%i.High.X%i.Y%i...` format.
pub fn bgs_distant_object_block_build_block_file_name_high_priority(
    e: &mut Engine,
    buffer: u32,
    world_space: u32,
    level: u32,
    x: i16,
    y: i16,
) {
    build_name(
        e,
        buffer,
        world_space,
        level,
        x,
        y,
        BLOCK_FILE_HIGH_PRIORITY_FORMAT,
    );
}

fn build_texture_name(e: &mut Engine, buffer: u32, world_space: u32, format: u32) {
    let first = world_space_name(e, world_space);
    let second = world_space_name(e, world_space);
    e.call(
        PRINT_FORMATTED,
        &args![buffer, 0x104u32, format, second, first],
    );
}

// Translated from 006f5780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock::BuildBlockDiffuseTextureFileName` (Xbox PDB),
/// cdecl `(buffer, world space)`: prints
/// `...\%s\Blocks\%s.Buildings.dds` with the world space's name twice.
pub fn bgs_distant_object_block_build_block_diffuse_texture_file_name(
    e: &mut Engine,
    buffer: u32,
    world_space: u32,
) {
    build_texture_name(e, buffer, world_space, DIFFUSE_TEXTURE_FORMAT);
}

// Translated from 006f57c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock::BuildBlockNormalTextureFileName` (Xbox PDB),
/// cdecl `(buffer, world space)`: prints
/// `...\%s\Blocks\%s.Buildings_n.dds` with the world space's name twice.
pub fn bgs_distant_object_block_build_block_normal_texture_file_name(
    e: &mut Engine,
    buffer: u32,
    world_space: u32,
) {
    build_texture_name(e, buffer, world_space, NORMAL_TEXTURE_FORMAT);
}

// Translated from 006f5800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the block's stinger node (`spNodeStinger`)
/// when the flag is set, else its node (`spNode`).
pub fn fn_006f5800(e: &mut Engine, this: Ptr, stinger: bool) -> u32 {
    let at = if stinger {
        this.addr() + BLOCK_STINGER
    } else {
        this.addr() + BLOCK_SCENE_NODE
    };
    pointer(e, at)
}

// Translated from 006f5840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, static: for each block of `BlocksLoaded`
/// that `00543c30` does not skip, refreshes the model data of its node and
/// of its stinger node (`006629f0`, `0043b230`, `00528820`), when it has
/// them.
pub fn fn_006f5840(e: &mut Engine) {
    let mut node = BLOCKS_LOADED;
    while node != 0 {
        let at = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let block = e.mem.u32(at);
        if block != 0 && !e.call(BLOCK_SKIP_CHECK, &args![block]).bool() {
            for stinger in [false, true] {
                let scene_node = fn_006f5800(e, Ptr::new(block), stinger);
                if scene_node != 0 {
                    let data = e.call(NODE_DATA_6629F0, &args![scene_node]).u32();
                    let root = e.call(MODEL_ROOT, &args![data]).u32();
                    e.call(REFRESH_528820, &args![root]);
                }
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 006f5af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the block's detach): takes the block off
/// `BlocksLoaded`; counts it off `011d8510` when it has a node, mesh or
/// stinger; with a node or mesh, detaches the node, the mesh and the old
/// mesh from the parent node and clears them and the load task; with a
/// stinger, detaches and clears the stinger, the old stinger and the
/// stinger load task; clears `bDetachFlag`.
pub fn fn_006f5af0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    fn_006f60c0(e, this);
    if pointer(e, base + BLOCK_SCENE_NODE) != 0
        || pointer(e, base + BLOCK_MESH) != 0
        || pointer(e, base + BLOCK_STINGER) != 0
    {
        let count: u32 = e.global(COUNTER_011D8510);
        e.set_global(COUNTER_011D8510, count.wrapping_sub(1));
    }
    if pointer(e, base + BLOCK_SCENE_NODE) != 0 || pointer(e, base + BLOCK_MESH) != 0 {
        for member in [BLOCK_SCENE_NODE, BLOCK_MESH, BLOCK_OLD_MESH] {
            let parent = parent_node(e, base);
            let child = pointer(e, base + member);
            e.vcall(parent, NODE_SLOT_DETACH_CHILD, &args![child]);
        }
        for member in [BLOCK_SCENE_NODE, BLOCK_MESH, BLOCK_OLD_MESH] {
            e.call(POINTER_ASSIGN, &args![base + member, 0u32]);
        }
        e.call(TASK_POINTER_ASSIGN, &args![base + BLOCK_LOAD_TASK, 0u32]);
    }
    if pointer(e, base + BLOCK_STINGER) != 0 {
        let parent = parent_node(e, base);
        let child = pointer(e, base + BLOCK_STINGER);
        e.vcall(parent, NODE_SLOT_DETACH_CHILD, &args![child]);
        e.call(POINTER_ASSIGN, &args![base + BLOCK_STINGER, 0u32]);
        e.call(POINTER_ASSIGN, &args![base + BLOCK_OLD_STINGER, 0u32]);
        e.call(
            TASK_POINTER_ASSIGN,
            &args![base + BLOCK_LOAD_TASK_STINGER, 0u32],
        );
    }
    e.mem.set_u8(base + BLOCK_DETACH_FLAG, 0);
}

/// Detaches `old` from the parent and attaches `new` (each through its own
/// parent lookup), as `Attach` does for a mesh replacing an old one.
fn swap_child(e: &mut Engine, base: u32, old: u32, new: u32) {
    let parent = parent_node_through_block(e, base);
    let child = pointer(e, base + old);
    e.vcall(parent, NODE_SLOT_DETACH_CHILD, &args![child]);
    attach_child(e, base, new);
}

fn attach_child(e: &mut Engine, base: u32, member: u32) {
    let parent = parent_node_through_block(e, base);
    let child = pointer(e, base + member);
    e.vcall(parent, NODE_SLOT_ATTACH_CHILD, &args![child, 1u32]);
}

fn adjust_counter(e: &mut Engine, counter: u32, up: bool) {
    let count: u32 = e.global(counter);
    let count = if up {
        count.wrapping_add(1)
    } else {
        count.wrapping_sub(1)
    };
    e.set_global(counter, count);
}

// Translated from 006f5ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the block's attach): with a mesh or a
/// stinger and not yet attached, takes the block off `BlocksLoaded`,
/// replaces the old mesh by the node (detaching the old mesh's
/// `009611e0` object) or by the mesh, or attaches the node or the mesh;
/// replaces the old stinger by the stinger or attaches it; moves the
/// block's count between `011d8510` and `011d8514` by `bHighPriority`
/// (old meshes counted off, new ones on); prepares the block
/// (`006f66e0`); puts it back on `BlocksLoaded` when it has a mesh or a
/// stinger; clears the old mesh and stinger and sets `bAttached`.
pub fn fn_006f5ca0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if pointer(e, base + BLOCK_MESH) == 0 && pointer(e, base + BLOCK_STINGER) == 0 {
        return;
    }
    if e.mem.u8(base + BLOCK_ATTACHED) != 0 {
        return;
    }
    fn_006f60c0(e, this);
    if pointer(e, base + BLOCK_OLD_MESH) != 0 {
        if pointer(e, base + BLOCK_SCENE_NODE) != 0 {
            let parent = parent_node_through_block(e, base);
            let old = pointer(e, base + BLOCK_OLD_MESH);
            let target = e.call(MESH_DETACH_TARGET, &args![old]).u32();
            e.vcall(parent, NODE_SLOT_DETACH_CHILD, &args![target]);
            attach_child(e, base, BLOCK_SCENE_NODE);
        } else {
            swap_child(e, base, BLOCK_OLD_MESH, BLOCK_MESH);
        }
    } else if pointer(e, base + BLOCK_SCENE_NODE) != 0 {
        attach_child(e, base, BLOCK_SCENE_NODE);
    } else if pointer(e, base + BLOCK_MESH) != 0 {
        attach_child(e, base, BLOCK_MESH);
    }
    if pointer(e, base + BLOCK_OLD_STINGER) != 0 {
        if pointer(e, base + BLOCK_STINGER) != 0 {
            swap_child(e, base, BLOCK_OLD_STINGER, BLOCK_STINGER);
        }
    } else if pointer(e, base + BLOCK_STINGER) != 0 {
        attach_child(e, base, BLOCK_STINGER);
    }
    let high_priority = fn_006f6070(e, this) != 0;
    let (old_counter, new_counter) = if high_priority {
        (COUNTER_011D8510, COUNTER_011D8514)
    } else {
        (COUNTER_011D8514, COUNTER_011D8510)
    };
    if pointer(e, base + BLOCK_OLD_MESH) != 0 || pointer(e, base + BLOCK_OLD_STINGER) != 0 {
        adjust_counter(e, old_counter, false);
    }
    if pointer(e, base + BLOCK_MESH) != 0 || pointer(e, base + BLOCK_STINGER) != 0 {
        adjust_counter(e, new_counter, true);
    }
    fn_006f66e0(e, this);
    if (pointer(e, base + BLOCK_MESH) != 0 || pointer(e, base + BLOCK_STINGER) != 0)
        && !fn_006f6100(e, this)
    {
        fn_006f6090(e, this);
    }
    e.call(POINTER_ASSIGN, &args![base + BLOCK_OLD_MESH, 0u32]);
    e.call(POINTER_ASSIGN, &args![base + BLOCK_OLD_STINGER, 0u32]);
    e.mem.set_u8(base + BLOCK_ATTACHED, 1);
}

// Translated from 006f6070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `bHighPriority` (+0x23).
pub fn fn_006f6070(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + BLOCK_HIGH_PRIORITY)
}

// Translated from 006f6090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: adds the block to `BlocksLoaded` (through a
/// local holding it).
pub fn fn_006f6090(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, item| {
        e.mem.set_u32(item.addr(), this.addr());
        e.call(SIMPLE_LIST_ADD, &args![BLOCKS_LOADED, item]);
    });
}

// Translated from 006f60c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: removes the block from `BlocksLoaded` as
/// long as it is in it.
pub fn fn_006f60c0(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, item| {
        while fn_006f6100(e, this) {
            e.mem.set_u32(item.addr(), this.addr());
            e.call(SIMPLE_LIST_REMOVE, &args![BLOCKS_LOADED, item]);
        }
    });
}

// Translated from 006f6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the block is in `BlocksLoaded`.
pub fn fn_006f6100(e: &mut Engine, this: Ptr) -> bool {
    let mut node = BLOCKS_LOADED;
    while node != 0 {
        let at = e.call(LIST_NODE_ITEM, &args![node]).u32();
        let block = e.mem.u32(at);
        if block != 0 && block == this.addr() {
            return true;
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 006f6150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, thiscall `(take)`: for the load task and
/// then the stinger load task, when there is one and it has finished:
/// with `take` set, takes its results (the mesh and the node into
/// `spMesh`/`spNode`, the stinger's node into `spNodeStinger`), drops the
/// task, and marks `bDoneLoading` when the other task is gone too.
pub fn fn_006f6150(e: &mut Engine, this: Ptr, take: bool) {
    let base = this.addr();
    if pointer(e, base + BLOCK_LOAD_TASK) != 0 && {
        let task = pointer(e, base + BLOCK_LOAD_TASK);
        e.call(TASK_IS_FINISHED, &args![task]).bool()
    } {
        if take {
            let task = pointer(e, base + BLOCK_LOAD_TASK);
            e.call(TASK_COPY_MESH, &args![task, base + BLOCK_MESH]);
            let task = pointer(e, base + BLOCK_LOAD_TASK);
            fn_006f6250(e, Ptr::new(task), base + BLOCK_SCENE_NODE);
        }
        e.call(TASK_POINTER_ASSIGN, &args![base + BLOCK_LOAD_TASK, 0u32]);
        if pointer(e, base + BLOCK_LOAD_TASK_STINGER) == 0 {
            e.mem.set_u8(base + BLOCK_DONE_LOADING, 1);
        }
    }
    if pointer(e, base + BLOCK_LOAD_TASK_STINGER) != 0 && {
        let task = pointer(e, base + BLOCK_LOAD_TASK_STINGER);
        e.call(TASK_IS_FINISHED, &args![task]).bool()
    } {
        if take {
            let task = pointer(e, base + BLOCK_LOAD_TASK_STINGER);
            fn_006f6250(e, Ptr::new(task), base + BLOCK_STINGER);
        }
        e.call(
            TASK_POINTER_ASSIGN,
            &args![base + BLOCK_LOAD_TASK_STINGER, 0u32],
        );
        if pointer(e, base + BLOCK_LOAD_TASK) == 0 {
            e.mem.set_u8(base + BLOCK_DONE_LOADING, 1);
        }
    }
}

// Translated from 006f6250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, thiscall on a load task `(pointer)`: copies
/// the task's `spNode` (+0x4C) into the pointer (`006e5cc0`) and answers
/// that call's result.
pub fn fn_006f6250(e: &mut Engine, this: Ptr, target: u32) -> u32 {
    e.call(POINTER_COPY, &args![target, this.addr() + TASK_NODE])
        .u32()
}

/// The common part of the two reloads (`006f6370`, `006f6540`): keeps the
/// current meshes as the old ones, cancels and drops both load tasks,
/// makes and queues a new load task (its flag byte is the player's
/// `006f6520` flag when `flag` is `None`; not a stinger), and resets the
/// flags (`bHighPriority` to `high_priority`), under the memory context
/// 0x24 with the source line given.
#[allow(clippy::too_many_arguments)]
fn reload(
    e: &mut Engine,
    base: u32,
    line: u32,
    world_space: u32,
    level: u32,
    x: i16,
    y: i16,
    flag: Option<u8>,
    high_priority: bool,
) {
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![guard, 0x24u32, 1u32, SOURCE_FILE_NAME, line],
        );
        e.call(
            POINTER_COPY,
            &args![base + BLOCK_OLD_MESH, base + BLOCK_MESH],
        );
        e.call(
            POINTER_COPY,
            &args![base + BLOCK_OLD_STINGER, base + BLOCK_STINGER],
        );
        for member in [BLOCK_LOAD_TASK, BLOCK_LOAD_TASK_STINGER] {
            if pointer(e, base + member) != 0 {
                let task = pointer(e, base + member);
                let manager: u32 = e.global(TASK_MANAGER);
                e.call(CANCEL_TASK, &args![manager, task, 0u32]);
                e.call(TASK_POINTER_ASSIGN, &args![base + member, 0u32]);
            }
        }
        let block = e.call(ALLOCATE, &args![0x58u32]).u32();
        let task = if block != 0 {
            let flag = match flag {
                Some(flag) => flag,
                None => {
                    let player: u32 = e.global(PLAYER);
                    fn_006f6520(e, Ptr::new(player))
                }
            };
            bgs_distant_object_block_load_task_construct(
                e,
                Ptr::new(block),
                base,
                world_space,
                i32::from(x) as u32,
                i32::from(y) as u32,
                level,
                flag,
                u8::from(high_priority),
                0,
            )
            .addr()
        } else {
            0
        };
        e.call(TASK_POINTER_ASSIGN, &args![base + BLOCK_LOAD_TASK, task]);
        e.mem.set_u32(base + BLOCK_DETACH_COUNT, 0);
        let task = pointer(e, base + BLOCK_LOAD_TASK);
        e.vcall(task, TASK_SLOT_QUEUE, &args![]);
        e.mem.set_u8(base + BLOCK_DONE_LOADING, 0);
        e.mem.set_u8(base + BLOCK_ATTACHED, 0);
        e.mem
            .set_u8(base + BLOCK_HIGH_PRIORITY, u8::from(high_priority));
        e.mem.set_u8(base + BLOCK_PREPARED, 0);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

// Translated from 006f6370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f6370 - the C++ exception-unwinding frame (FS:[0]) that frees the task allocation and leaves the memory context when a call throws is not translated
/// Unnamed in the engine map (the block's load), thiscall `(world space,
/// level, x, y)` (the coordinates' low 16 bits, sign-extended): keeps the
/// meshes as the old ones, cancels the load tasks, makes a load task
/// (0x58 bytes; the player's `006f6520` flag, not high priority, not a
/// stinger), queues it and clears the block's flags, under the memory
/// context 0x24 (source line 0x250).
pub fn fn_006f6370(e: &mut Engine, this: Ptr, world_space: u32, level: u32, x: i16, y: i16) {
    reload(e, this.addr(), 0x250, world_space, level, x, y, None, false);
}

// Translated from 006f6520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the code calls it on the player): the byte
/// at +0xDF2.
pub fn fn_006f6520(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xdf2)
}

// Translated from 006f6540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f6540 - the C++ exception-unwinding frame (FS:[0]) that frees the task allocation and leaves the memory context when a call throws is not translated
/// Unnamed in the engine map (the block's high-priority load), thiscall
/// `(world space, level, x, y)`: as `006f6370`, but the load task is made
/// with flag 0 and high priority 1, `bHighPriority` is set, and the memory
/// context's source line is 0x272.
pub fn fn_006f6540(e: &mut Engine, this: Ptr, world_space: u32, level: u32, x: i16, y: i16) {
    reload(
        e,
        this.addr(),
        0x272,
        world_space,
        level,
        x,
        y,
        Some(0),
        true,
    );
}

/// The "needs preparing" test of [`fn_006f66e0`] for one mesh pair: true
/// when the block is not prepared, else when it has both meshes and
/// `00631820` answers true for them.
fn needs_prepare(e: &mut Engine, base: u32, new: u32, old: u32) -> bool {
    if e.mem.u8(base + BLOCK_PREPARED) == 0 {
        return true;
    }
    pointer(e, base + new) != 0
        && pointer(e, base + old) != 0
        && e.call(MESHES_DIFFER, &args![base + new, base + old]).bool()
}

fn mesh(e: &mut Engine, base: u32) -> u32 {
    pointer(e, base + BLOCK_MESH)
}

// Translated from 006f66e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f66e0 - the C++ exception-unwinding frame (FS:[0]) that frees the property allocation when its constructor throws is not translated
/// Unnamed in the engine map (the block's prepare): when the mesh needs
/// preparing, replaces its property of type `0040df90` by a new property
/// (0x1C bytes, set up with 1, 0, 0x80, 4), sets three flags of its shader
/// property (0x22 on, 0x21 and 0 off), swaps in the shader property's
/// clone for the mesh when there is one, gives the mesh its shader,
/// updates its properties and runs the downward update with zeroed data,
/// updates the terrain node's block visibility, sets up the geometry data
/// (`004410d0(0)`, consistency 0x4000, `004410f0(1)`), hands the mesh to
/// the renderer and sets `bPrepared`. When the stinger needs preparing,
/// prepares it (`006f6a90`), hands it to the renderer and sets
/// `bPrepared`.
pub fn fn_006f66e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let do_mesh = needs_prepare(e, base, BLOCK_MESH, BLOCK_OLD_MESH);
    let do_stinger = needs_prepare(e, base, BLOCK_STINGER, BLOCK_OLD_STINGER);
    if do_mesh && mesh(e, base) != 0 {
        prepare_mesh(e, base);
    }
    if do_stinger && pointer(e, base + BLOCK_STINGER) != 0 {
        let stinger = pointer(e, base + BLOCK_STINGER);
        e.call(PREPARE_STINGER, &args![base, stinger]);
        let renderer = e.call(RENDERER, &args![]).u32();
        e.call(RENDERER_E74120, &args![renderer]);
        e.mem.set_u8(base + BLOCK_PREPARED, 1);
    }
}

/// The mesh part of [`fn_006f66e0`].
fn prepare_mesh(e: &mut Engine, base: u32) {
    let kind = e.call(PROPERTY_TYPE_40DF90, &args![]).u32();
    let target = mesh(e, base);
    e.call(REMOVE_PROPERTY, &args![target, kind]);
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    let property = if block != 0 {
        e.call(PROPERTY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(PROPERTY_SET_4393C0, &args![property, 1u32]);
    e.call(PROPERTY_SET_49ED90, &args![property, 0u32]);
    e.call(PROPERTY_SET_94DB80, &args![property, 0x80u32]);
    e.call(PROPERTY_SET_4393E0, &args![property, 4u32]);
    let target = mesh(e, base);
    e.call(ATTACH_PROPERTY, &args![target, property]);
    let shader_type = e.call(SHADER_PROPERTY_TYPE, &args![]).u32();
    let target = mesh(e, base);
    let mut shader_property = e.call(GET_PROPERTY, &args![target, shader_type]).u32();
    e.call(SHADER_SET_FLAG, &args![shader_property, 0x22u32, 1u32]);
    e.call(SHADER_SET_FLAG, &args![shader_property, 0x21u32, 0u32]);
    e.call(SHADER_SET_FLAG, &args![shader_property, 0u32, 0u32]);
    let target = mesh(e, base);
    let clone = e
        .vcall(
            shader_property,
            SHADER_SLOT_CLONE,
            &args![target, 0u32, 1u32],
        )
        .u32();
    if clone != 0 {
        let target = mesh(e, base);
        e.call(REMOVE_PROPERTY, &args![target, 3u32]);
        shader_property = clone;
        let target = mesh(e, base);
        e.call(ATTACH_PROPERTY, &args![target, shader_property]);
    }
    let shader_id = e.call(SHADER_ID, &args![shader_property]).u32();
    let shader = e.call(GET_SHADER, &args![shader_id]).u32();
    let target = mesh(e, base);
    e.call(SET_SHADER, &args![target, shader]);
    e.with_stack(0xc, |e, data| {
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![data, 0.0f32.to_bits(), 0u32, 0u32],
        );
        let target = mesh(e, base);
        e.call(UPDATE_PROPERTIES, &args![target]);
        let target = mesh(e, base);
        e.call(UPDATE_DOWNWARD, &args![target, data]);
    });
    let terrain = e.mem.u32(base + BLOCK_NODE);
    e.call(UPDATE_BLOCK_VISIBILITY, &args![terrain, 0u32]);
    for (setter, value) in [
        (DATA_SET_4410D0, 0u32),
        (DATA_SET_CONSISTENCY, 0x4000),
        (DATA_SET_4410F0, 1),
    ] {
        let target = mesh(e, base);
        let data = e.call(MESH_DATA, &args![target]).u32();
        e.call(setter, &args![data, value]);
    }
    let target = mesh(e, base);
    let owner = e.call(MESH_OWNER_4030B0, &args![target]).u32();
    let renderer = e.call(RENDERER, &args![]).u32();
    let target = mesh(e, base);
    let value = e
        .vcall(owner, OWNER_SLOT_118, &args![target, shader_property])
        .u32();
    let target = mesh(e, base);
    e.vcall(
        renderer,
        RENDERER_SLOT_EC,
        &args![target, 0u32, 0u32, value],
    );
    let renderer = e.call(RENDERER, &args![]).u32();
    e.call(RENDERER_E74120, &args![renderer]);
    e.mem.set_u8(base + BLOCK_PREPARED, 1);
}

// Translated from 006f6d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f6d10 - the C++ exception-unwinding frame (FS:[0]) that destroys the members when a call throws, and the /GS stack cookie check (00ec408c), are not translated
/// `BGSDistantObjectBlockLoadTask::BGSDistantObjectBlockLoadTask` (Xbox
/// PDB), thiscall `(block, world space, x, y, level, flag, high priority,
/// stinger)` (the last three are bytes): constructs the `QueuedFileEntry`
/// (3), sets the vtable and the fields, clears both `NiPointer`s and
/// `bCanceled`, keeps `bStingerVersion`, builds the file name into a
/// 0x104-byte local (stinger: the stinger name, high-priority or not;
/// otherwise the high-priority name when `high priority` is set, else,
/// when `flag` or the player's `006f6520` flag is set, the `PA` name,
/// falling back to the plain name when that file does not exist, and the
/// plain name otherwise; the coordinates go as 16-bit words), gives the
/// entry the name from its sixth character on and calls `00c3cf60(0)`.
/// Answers `this`.
#[allow(clippy::too_many_arguments)]
pub fn bgs_distant_object_block_load_task_construct(
    e: &mut Engine,
    this: Ptr,
    block: u32,
    world_space: u32,
    x: u32,
    y: u32,
    level: u32,
    flag: u8,
    high_priority: u8,
    stinger: u8,
) -> Ptr {
    let base = this.addr();
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![base, 3u32]);
    e.mem.set_u32(base, LOAD_TASK_VTABLE);
    e.mem.set_u8(base + TASK_FINISHED_QUEUING, 0);
    e.mem.set_u32(base + TASK_X, x);
    e.mem.set_u32(base + TASK_Y, y);
    e.mem.set_u32(base + TASK_LEVEL, level);
    e.mem.set_u32(base + TASK_BLOCK, block);
    e.mem.set_u32(base + TASK_WORLD_SPACE, world_space);
    e.call(POINTER_CONSTRUCT, &args![base + TASK_MESH, 0u32]);
    e.call(POINTER_CONSTRUCT, &args![base + TASK_NODE, 0u32]);
    e.mem.set_u8(base + TASK_CANCELED, 0);
    e.mem.set_u8(base + TASK_STINGER_VERSION, stinger);
    e.with_stack(0x104, |e, name| {
        let name = name.addr();
        let world_space = e.mem.u32(base + TASK_WORLD_SPACE);
        let (x16, y16) = (x as u16 as i16, y as u16 as i16);
        let words = (u32::from(x as u16), u32::from(y as u16));
        if stinger == 0 {
            let other = flag != 0 || {
                let player: u32 = e.global(PLAYER);
                fn_006f6520(e, Ptr::new(player)) != 0
            };
            if high_priority != 0 {
                bgs_distant_object_block_build_block_file_name_high_priority(
                    e,
                    name,
                    world_space,
                    level,
                    x16,
                    y16,
                );
            } else if other {
                e.call(
                    BUILD_BLOCK_FILE_NAME_PA,
                    &args![name, world_space, level, words.0, words.1],
                );
                if e.call(FILE_EXISTS, &args![name, 0u32, 0u32, 0xffff_ffffu32])
                    .u32()
                    == 0
                {
                    let world_space = e.mem.u32(base + TASK_WORLD_SPACE);
                    bgs_distant_object_block_build_block_file_name(
                        e,
                        name,
                        world_space,
                        level,
                        x16,
                        y16,
                    );
                }
            } else {
                bgs_distant_object_block_build_block_file_name(
                    e,
                    name,
                    world_space,
                    level,
                    x16,
                    y16,
                );
            }
        } else {
            let builder = if high_priority != 0 {
                BUILD_STINGER_BLOCK_FILE_NAME_HIGH_PRIORITY
            } else {
                BUILD_STINGER_BLOCK_FILE_NAME
            };
            e.call(builder, &args![name, world_space, level, words.0, words.1]);
        }
        e.call(QUEUED_FILE_SET_NAME, &args![base, name + 5]);
    });
    e.call(QUEUED_FILE_C3CF60, &args![base, 0u32]);
    this
}

// Translated from 006f6fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlockLoadTask::_scalar_deleting_destructor_` (Xbox
/// PDB): runs the destructor (`006f6fe0`) and, when bit 0 of the flags is
/// set, frees the object. Answers `this`.
pub fn bgs_distant_object_block_load_task_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006f6fe0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006f6fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f6fe0 - the C++ exception-unwinding frame (FS:[0]) that destroys the remaining members when a destructor throws is not translated
/// `BGSDistantObjectBlockLoadTask::~BGSDistantObjectBlockLoadTask`
/// (unnamed in the engine map; the body is the destructor): sets the
/// vtable, clears `spMesh` and `spNode`, destroys them in reverse order and
/// destroys the `QueuedFileEntry` base.
pub fn fn_006f6fe0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, LOAD_TASK_VTABLE);
    e.call(POINTER_ASSIGN, &args![base + TASK_MESH, 0u32]);
    e.call(POINTER_ASSIGN, &args![base + TASK_NODE, 0u32]);
    e.call(POINTER_DESTROY, &args![base + TASK_NODE]);
    e.call(POINTER_DESTROY, &args![base + TASK_MESH]);
    e.call(QUEUED_FILE_ENTRY_DESTROY, &args![base]);
}

// Translated from 006f7070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f7070 - the /GS stack cookie check (00ec408c) is not translated
/// `BGSDistantObjectBlockLoadTask::QueueMe` (Xbox PDB): unless it is the
/// stinger version, queues the block's diffuse and normal textures (each
/// name built into its own 0x104-byte local) with the model loader; sets
/// `bFinishedQueuing` and calls vtable +0x28.
pub fn bgs_distant_object_block_load_task_queue_me(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + TASK_STINGER_VERSION) == 0 {
        e.with_stack(0x104, |e, diffuse| {
            e.with_stack(0x104, |e, normal| {
                for (buffer, builder) in [
                    (
                        diffuse.addr(),
                        bgs_distant_object_block_build_block_diffuse_texture_file_name
                            as fn(&mut Engine, u32, u32),
                    ),
                    (
                        normal.addr(),
                        bgs_distant_object_block_build_block_normal_texture_file_name,
                    ),
                ] {
                    let world_space = e.mem.u32(base + TASK_WORLD_SPACE);
                    builder(e, buffer, world_space);
                    let word = e.call(TASK_QUEUE_WORD, &args![base]).u32();
                    let loader: u32 = e.global(MODEL_LOADER);
                    e.call(QUEUE_TEXTURE, &args![loader, buffer, word, base]);
                }
            });
        });
    }
    e.mem.set_u8(base + TASK_FINISHED_QUEUING, 1);
    e.vcall(base, TASK_SLOT_28, &args![]);
}

// Translated from 006f71e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f71e0 - the C++ exception-unwinding frame (FS:[0]) that destroys the stream and leaves the memory context when a call throws, and the /GS stack cookie check (00ec408c), are not translated
/// `BGSDistantObjectBlockLoadTask::Run` (Xbox PDB): under the memory
/// context 0x24 (source line 0x39a), gets the entry's file; when there is
/// one or the named file exists (`FileFinder::Exist(name, 0, 6, 1)`),
/// loads it into a stream (a 0x5D4-byte local) and keeps its first object
/// as `spNode` with the mesh found in it as `spMesh` when the object's
/// vtable +0xC answers non-zero, else as `spMesh`; when it does not load,
/// sets the high-priority block's `bDetachFlag`.
pub fn bgs_distant_object_block_load_task_run(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![guard, 0x24u32, 1u32, SOURCE_FILE_NAME, 0x39au32],
        );
        let file = e.call(QUEUED_FILE_GET_FILE, &args![base, 0u32, 1u32]).u32();
        let mut found = true;
        if file == 0 {
            let name = e.call(QUEUED_FILE_NAME, &args![base]).u32();
            if e.call(FILE_EXISTS, &args![name, 0u32, 6u32, 1u32]).u32() == 0 {
                found = false;
            }
        }
        e.with_stack(0x5d4, |e, stream| {
            e.call(STREAM_CONSTRUCT, &args![stream]);
            let loaded = found && {
                let name = e.call(QUEUED_FILE_NAME, &args![base]).u32();
                e.call(STREAM_LOAD, &args![stream, name, file]).bool()
            };
            if loaded {
                let object = e.call(STREAM_OBJECT, &args![stream, 0u32]).u32();
                if e.vcall(object, OBJECT_SLOT_C, &args![]).u32() != 0 {
                    let object = e.call(STREAM_OBJECT, &args![stream, 0u32]).u32();
                    e.call(POINTER_ASSIGN, &args![base + TASK_NODE, object]);
                    let node = pointer(e, base + TASK_NODE);
                    let found_mesh = e.call(NODE_FIND_MESH, &args![node, 0u32]).u32();
                    e.call(POINTER_ASSIGN, &args![base + TASK_MESH, found_mesh]);
                } else {
                    let object = e.call(STREAM_OBJECT, &args![stream, 0u32]).u32();
                    e.call(POINTER_ASSIGN, &args![base + TASK_MESH, object]);
                }
            } else {
                let block = e.mem.u32(base + TASK_BLOCK);
                if fn_006f6070(e, Ptr::new(block)) != 0 {
                    let block = e.mem.u32(base + TASK_BLOCK);
                    fn_006f73b0(e, Ptr::new(block));
                }
            }
            e.call(STREAM_DESTROY, &args![stream]);
        });
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

// Translated from 006f73b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the block's `bDetachFlag` (+0x24).
pub fn fn_006f73b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + BLOCK_DETACH_FLAG, 1);
}

// Translated from 006f73d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlockLoadTask::Finish` (Xbox PDB): with a mesh and not
/// canceled, lets the block take the results (`006f6150(1)`), calls
/// `00929260(1)` on the block's terrain node's object and, when the mesh's
/// vtable +0x28 answers 0, prints the empty-block message with the level,
/// x and y; canceled, drops the mesh and the node. Without a mesh and not
/// canceled, lets the block drop the task (`006f6150(0)`) when there is a
/// block.
pub fn bgs_distant_object_block_load_task_finish(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if pointer(e, base + TASK_MESH) != 0 {
        if e.mem.u8(base + TASK_CANCELED) == 0 {
            let block = e.mem.u32(base + TASK_BLOCK);
            fn_006f6150(e, Ptr::new(block), true);
            let block = e.mem.u32(base + TASK_BLOCK);
            let terrain = pointer(e, block);
            let object = pointer(e, terrain);
            e.call(TERRAIN_929260, &args![object, 1u32]);
            let found_mesh = pointer(e, base + TASK_MESH);
            if e.vcall(found_mesh, MESH_SLOT_28, &args![]).u32() == 0 {
                let level = e.mem.u32(base + TASK_LEVEL);
                let x = e.mem.u32(base + TASK_X);
                let y = e.mem.u32(base + TASK_Y);
                e.call(PRINT_MESSAGE, &args![EMPTY_BLOCK_MESSAGE, level, x, y]);
            }
        } else {
            e.call(POINTER_ASSIGN, &args![base + TASK_MESH, 0u32]);
            e.call(POINTER_ASSIGN, &args![base + TASK_NODE, 0u32]);
        }
    } else if e.mem.u8(base + TASK_CANCELED) == 0 && e.mem.u32(base + TASK_BLOCK) != 0 {
        let block = e.mem.u32(base + TASK_BLOCK);
        fn_006f6150(e, Ptr::new(block), false);
    }
}

/// `...\%s\Blocks\%s.Level%i.X%i.Y%i.PostApocalypse.NIF`.
const BLOCK_FILE_PA_FORMAT: u32 = 0x0106_daf8;
/// `...\%s\Stinger\%s.Level%i.X%i.Y%i.NIF`.
const STINGER_FILE_FORMAT: u32 = 0x0106_db44;
/// `...\%s\Stinger\%s.Level%i.High.X%i.Y%i.NIF`.
const STINGER_FILE_HIGH_PRIORITY_FORMAT: u32 = 0x0106_db80;
/// `"Object LOD Task for block ( %i, %i ) at level ( %i )"`.
const TASK_DESCRIPTION_FORMAT: u32 = 0x0106_dcbc;
/// `00450f90` (`thiscall`, `RET 4`): sets a flag of the object to the byte
/// given.
const SET_HIDDEN_00450F90: u32 = 0x0045_0f90;
/// `00453470` (no stack word): a node's child count.
const CHILD_COUNT_00453470: u32 = 0x0045_3470;
/// `0045bb80` (no stack word): an object's world position.
const WORLD_POSITION_0045BB80: u32 = 0x0045_bb80;
/// The `TES` singleton pointer.
const TES: u32 = 0x011d_ea10;
/// `TES::GetWorldSpace` (`004fd3e0`, engine map).
const TES_GET_WORLD_SPACE: u32 = 0x004f_d3e0;
/// `TESWorldSpace::GetCellFromWorldCoord` (`00587550`, engine map; `RET 4`).
const WORLD_SPACE_CELL_AT: u32 = 0x0058_7550;
/// `TES::IsCellLoaded` (`004511e0`, engine map; `RET 8`).
const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
/// `TESObjectCELL::All3DWithVisibleDistantFadingIn` (`00557d30`, engine map).
const CELL_ALL_3D_FADING_IN: u32 = 0x0055_7d30;
/// The object's slot (+0x18) that answers it as geometry.
const OBJECT_SLOT_AS_GEOMETRY: u32 = 0x18;
/// `QueuedFile::Cancel` (`00c3cb70`, engine map; `RET 8`).
const QUEUED_FILE_CANCEL: u32 = 0x00c3_cb70;

// Translated from 006f5690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock::BuildBlockFileNamePA` (engine map), cdecl
/// `(buffer, world space, level, x, y)`: as
/// [`bgs_distant_object_block_build_block_file_name`] with the
/// `...Level%i.X%i.Y%i.PostApocalypse.NIF` format.
pub fn bgs_distant_object_block_build_block_file_name_pa(
    e: &mut Engine,
    buffer: u32,
    world_space: u32,
    level: u32,
    x: i16,
    y: i16,
) {
    build_name(e, buffer, world_space, level, x, y, BLOCK_FILE_PA_FORMAT);
}

// Translated from 006f56e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock::BuildStingerBlockFileName` (engine map): as
/// [`bgs_distant_object_block_build_block_file_name`] with the
/// `...\Stinger\%s.Level%i.X%i.Y%i.NIF` format.
pub fn bgs_distant_object_block_build_stinger_block_file_name(
    e: &mut Engine,
    buffer: u32,
    world_space: u32,
    level: u32,
    x: i16,
    y: i16,
) {
    build_name(e, buffer, world_space, level, x, y, STINGER_FILE_FORMAT);
}

// Translated from 006f5730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock::BuildStingerBlockFileNameHighPriority` (engine
/// map): as [`bgs_distant_object_block_build_block_file_name`] with the
/// `...\Stinger\%s.Level%i.High.X%i.Y%i.NIF` format.
pub fn bgs_distant_object_block_build_stinger_block_file_name_high_priority(
    e: &mut Engine,
    buffer: u32,
    world_space: u32,
    level: u32,
    x: i16,
    y: i16,
) {
    build_name(
        e,
        buffer,
        world_space,
        level,
        x,
        y,
        STINGER_FILE_HIGH_PRIORITY_FORMAT,
    );
}

// Translated from 006f58e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, `(object, value)`): a non-null
/// object gets `00450f90(value == 0)` (the low byte compared); then, when
/// the object is a node (slot +0xC; asked even of a null object, as the
/// exe does), each child (`0045bc00`) gets the same with the `value` byte.
pub fn fn_006f58e0(e: &mut Engine, object: u32, value: u32) {
    if object != 0 {
        e.call(
            SET_HIDDEN_00450F90,
            &args![object, u32::from(value as u8 == 0)],
        );
    }
    if e.vcall(object, OBJECT_SLOT_C, &args![]).u32() == 0 {
        return;
    }
    let node = e.vcall(object, OBJECT_SLOT_C, &args![]).u32();
    let mut index: u32 = 0;
    while index < e.call(CHILD_COUNT_00453470, &args![node]).u32() {
        let child = e.call(NODE_FIND_MESH, &args![node, index]).u32();
        fn_006f58e0(e, child, u32::from(value as u8));
        index += 1;
    }
}

// Translated from 006f5970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock::UpdateStingerVisibility` (engine map;
/// `thiscall`, `RET 0x10`): a node's non-null children are visited with
/// the same words (the two 16-bit ones zero-extended). A leaf object is
/// shown ([`fn_006f58e0`] with 0) when the cell at its world position in
/// the current world space is loaded (`TES::IsCellLoaded` with 1) and all
/// its 3D is fading in, else hidden (with 1).
pub fn bgs_distant_object_block_update_stinger_visibility(
    e: &mut Engine,
    this: Ptr,
    object: u32,
    word: u32,
    first: u16,
    second: u16,
) {
    if e.vcall(object, OBJECT_SLOT_C, &args![]).u32() != 0 {
        let node = e.vcall(object, OBJECT_SLOT_C, &args![]).u32();
        let mut index: u32 = 0;
        while index < e.call(CHILD_COUNT_00453470, &args![node]).u32() {
            let child = e.call(NODE_FIND_MESH, &args![node, index]).u32();
            if child != 0 {
                bgs_distant_object_block_update_stinger_visibility(
                    e, this, child, word, first, second,
                );
            }
            index += 1;
        }
        return;
    }
    let cell_ready = e.with_stack(0xc, |e, point| {
        let position = e.call(WORLD_POSITION_0045BB80, &args![object]).u32();
        for i in 0..3 {
            let value = e.mem.u32(position + 4 * i);
            e.mem.set_u32(point.addr() + 4 * i, value);
        }
        let tes: u32 = e.global(TES);
        let world_space = e.call(TES_GET_WORLD_SPACE, &args![tes]).u32();
        let cell = e
            .call(WORLD_SPACE_CELL_AT, &args![world_space, point])
            .u32();
        let tes: u32 = e.global(TES);
        e.call(TES_IS_CELL_LOADED, &args![tes, cell, 1u32]).u8() != 0
            && e.call(CELL_ALL_3D_FADING_IN, &args![cell]).u8() != 0
    });
    fn_006f58e0(e, object, if cell_ready { 0 } else { 1 });
}

// Translated from 006f5a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): a non-null `object`
/// gets `00450f90(0)` and, when it is a node (slot +0xC;
/// asked even of a null object), each of its children the same way (null
/// children included).
pub fn fn_006f5a70(e: &mut Engine, this: Ptr, object: u32) {
    if object != 0 {
        e.call(SET_HIDDEN_00450F90, &args![object, 0u32]);
    }
    if e.vcall(object, OBJECT_SLOT_C, &args![]).u32() == 0 {
        return;
    }
    let node = e.vcall(object, OBJECT_SLOT_C, &args![]).u32();
    let mut index: u32 = 0;
    while index < e.call(CHILD_COUNT_00453470, &args![node]).u32() {
        let child = e.call(NODE_FIND_MESH, &args![node, index]).u32();
        fn_006f5a70(e, this, child);
        index += 1;
    }
}

// Translated from 006f6270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f6270 - the C++ exception-unwinding frame (FS:[0]) that frees the task allocation and leaves the memory context when a call throws is not translated
/// Unnamed in the engine map (the block's stinger load), `thiscall`
/// `(world space, level, x, y)`: under the memory context 0x24 (source
/// line 0x240) the mesh is kept as the old mesh, a new load task (0x58
/// bytes; flag 1, high priority 1, not a stinger, the coordinates
/// sign-extended) becomes `spLoadTask` and is queued (slot +0x20), and
/// `bDoneLoading` and `bAttached` are cleared. The old load task is not
/// cancelled here.
pub fn fn_006f6270(e: &mut Engine, this: Ptr, world_space: u32, level: u32, x: i16, y: i16) {
    let base = this.addr();
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![guard, 0x24u32, 1u32, SOURCE_FILE_NAME, 0x240u32],
        );
        e.call(
            POINTER_COPY,
            &args![base + BLOCK_OLD_MESH, base + BLOCK_MESH],
        );
        let block = e.call(ALLOCATE, &args![0x58u32]).u32();
        let task = if block != 0 {
            bgs_distant_object_block_load_task_construct(
                e,
                Ptr::new(block),
                base,
                world_space,
                i32::from(x) as u32,
                i32::from(y) as u32,
                level,
                1,
                1,
                0,
            )
            .addr()
        } else {
            0
        };
        e.call(TASK_POINTER_ASSIGN, &args![base + BLOCK_LOAD_TASK, task]);
        let task = pointer(e, base + BLOCK_LOAD_TASK);
        e.vcall(task, TASK_SLOT_QUEUE, &args![]);
        e.mem.set_u8(base + BLOCK_DONE_LOADING, 0);
        e.mem.set_u8(base + BLOCK_ATTACHED, 0);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
    });
}

// Translated from 006f6a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006f6a90 - the C++ exception-unwinding frame (FS:[0]) that frees the property allocation when its constructor throws is not translated
/// Unnamed in the engine map (the stinger's prepare, `thiscall`, `RET 4`):
/// nothing for a null object; a node's children are prepared in turn. A
/// geometry (slot +0x18) loses its property of type `0040df90` for a new
/// one (0x1C bytes, set up with 1, 0, 0x80, 4), its shader property gets
/// flags 0x22 and 0x37 set, the shader property's clone (slot +0xB0 with
/// the geometry, 0, 1) replaces it when there is one, the geometry gets
/// its shader, its properties updated and the downward update with zeroed
/// data; the terrain node's block visibility is updated; the geometry data
/// is set up (`004410d0(0)`, consistency 0x4000, `004410f0(1)`) and the
/// geometry handed to the renderer (slot +0xEC with the owner's slot
/// +0x118 answer).
pub fn fn_006f6a90(e: &mut Engine, this: Ptr, object: u32) {
    if object == 0 {
        return;
    }
    if e.vcall(object, OBJECT_SLOT_C, &args![]).u32() != 0 {
        let node = e.vcall(object, OBJECT_SLOT_C, &args![]).u32();
        let mut index: u32 = 0;
        while index < e.call(CHILD_COUNT_00453470, &args![node]).u32() {
            let child = e.call(NODE_FIND_MESH, &args![node, index]).u32();
            fn_006f6a90(e, this, child);
            index += 1;
        }
        return;
    }
    let geometry = e.vcall(object, OBJECT_SLOT_AS_GEOMETRY, &args![]).u32();
    let shader_type = e.call(SHADER_PROPERTY_TYPE, &args![]).u32();
    // The exe asks for this property twice and keeps only the second answer.
    e.call(GET_PROPERTY, &args![geometry, shader_type]);
    let shader_type = e.call(SHADER_PROPERTY_TYPE, &args![]).u32();
    let mut shader_property = e.call(GET_PROPERTY, &args![geometry, shader_type]).u32();
    let kind = e.call(PROPERTY_TYPE_40DF90, &args![]).u32();
    e.call(REMOVE_PROPERTY, &args![geometry, kind]);
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    let property = if block != 0 {
        e.call(PROPERTY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(PROPERTY_SET_4393C0, &args![property, 1u32]);
    e.call(PROPERTY_SET_49ED90, &args![property, 0u32]);
    e.call(PROPERTY_SET_94DB80, &args![property, 0x80u32]);
    e.call(PROPERTY_SET_4393E0, &args![property, 4u32]);
    e.call(ATTACH_PROPERTY, &args![geometry, property]);
    e.call(SHADER_SET_FLAG, &args![shader_property, 0x22u32, 1u32]);
    e.call(SHADER_SET_FLAG, &args![shader_property, 0x37u32, 1u32]);
    let clone = e
        .vcall(
            shader_property,
            SHADER_SLOT_CLONE,
            &args![geometry, 0u32, 1u32],
        )
        .u32();
    if clone != 0 {
        e.call(REMOVE_PROPERTY, &args![geometry, 3u32]);
        shader_property = clone;
        e.call(ATTACH_PROPERTY, &args![geometry, shader_property]);
    }
    let shader_id = e.call(SHADER_ID, &args![shader_property]).u32();
    let shader = e.call(GET_SHADER, &args![shader_id]).u32();
    e.call(SET_SHADER, &args![geometry, shader]);
    e.with_stack(0xc, |e, data| {
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![data, x87_store32(0.0), 0u32, 0u32],
        );
        e.call(UPDATE_PROPERTIES, &args![geometry]);
        e.call(UPDATE_DOWNWARD, &args![geometry, data]);
    });
    let terrain = e.mem.u32(this.addr() + BLOCK_NODE);
    e.call(UPDATE_BLOCK_VISIBILITY, &args![terrain, 0u32]);
    for (setter, value) in [
        (DATA_SET_4410D0, 0u32),
        (DATA_SET_CONSISTENCY, 0x4000),
        (DATA_SET_4410F0, 1),
    ] {
        let data = e.call(MESH_DATA, &args![geometry]).u32();
        e.call(setter, &args![data, value]);
    }
    let owner = e.call(MESH_OWNER_4030B0, &args![geometry]).u32();
    let renderer = e.call(RENDERER, &args![]).u32();
    let value = e
        .vcall(owner, OWNER_SLOT_118, &args![geometry, shader_property])
        .u32();
    e.vcall(
        renderer,
        RENDERER_SLOT_EC,
        &args![geometry, 0u32, 0u32, value],
    );
}

// Translated from 006f71a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlockLoadTask::Cancel` (engine map; `RET 8`): once
/// only (`bCanceled`, +0x50, then set), `QueuedFile::Cancel` with the two
/// words.
pub fn bgs_distant_object_block_load_task_cancel(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    let base = this.addr();
    if e.mem.u8(base + TASK_CANCELED) != 0 {
        return;
    }
    e.mem.set_u8(base + TASK_CANCELED, 1);
    e.call(QUEUED_FILE_CANCEL, &args![this, a, b]);
}

// Translated from 006f74b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlockLoadTask::GetDescription` (engine map; `RET 8`):
/// prints `"Object LOD Task for block ( %i, %i ) at level ( %i )"` with
/// `iX`, `iY` and `iLevel` into the buffer of the given size; true.
pub fn bgs_distant_object_block_load_task_get_description(
    e: &mut Engine,
    this: Ptr,
    buffer: u32,
    size: u32,
) -> bool {
    let base = this.addr();
    let x = e.mem.u32(base + TASK_X);
    let y = e.mem.u32(base + TASK_Y);
    let level = e.mem.u32(base + TASK_LEVEL);
    e.call(
        PRINT_FORMATTED,
        &args![buffer, size, TASK_DESCRIPTION_FORMAT, x, y, level],
    );
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006f_5690,
            bgs_distant_object_block_build_block_file_name_pa(u32, u32, u32, i16, i16)
        ),
        entry!(
            0x006f_56e0,
            bgs_distant_object_block_build_stinger_block_file_name(u32, u32, u32, i16, i16)
        ),
        entry!(
            0x006f_5730,
            bgs_distant_object_block_build_stinger_block_file_name_high_priority(
                u32, u32, u32, i16, i16
            )
        ),
        entry!(0x006f_58e0, fn_006f58e0(u32, u32)),
        entry!(
            0x006f_5970,
            bgs_distant_object_block_update_stinger_visibility(Ptr, u32, u32, u16, u16)
        ),
        entry!(0x006f_5a70, fn_006f5a70(Ptr, u32)),
        entry!(0x006f_6270, fn_006f6270(Ptr, u32, u32, i16, i16)),
        entry!(0x006f_6a90, fn_006f6a90(Ptr, u32)),
        entry!(
            0x006f_71a0,
            bgs_distant_object_block_load_task_cancel(Ptr, u32, u32)
        ),
        entry!(
            0x006f_74b0,
            bgs_distant_object_block_load_task_get_description(Ptr, u32, u32) -> bool
        ),
        entry!(
            0x006f55f0,
            bgs_distant_object_block_build_block_file_name(u32, u32, u32, i16, i16)
        ),
        entry!(
            0x006f5640,
            bgs_distant_object_block_build_block_file_name_high_priority(u32, u32, u32, i16, i16)
        ),
        entry!(
            0x006f5780,
            bgs_distant_object_block_build_block_diffuse_texture_file_name(u32, u32)
        ),
        entry!(
            0x006f57c0,
            bgs_distant_object_block_build_block_normal_texture_file_name(u32, u32)
        ),
        entry!(0x006f5800, fn_006f5800(Ptr, bool) -> u32),
        entry!(0x006f5840, fn_006f5840()),
        entry!(0x006f5af0, fn_006f5af0(Ptr)),
        entry!(0x006f5ca0, fn_006f5ca0(Ptr)),
        entry!(0x006f6070, fn_006f6070(Ptr) -> u8),
        entry!(0x006f6090, fn_006f6090(Ptr)),
        entry!(0x006f60c0, fn_006f60c0(Ptr)),
        entry!(0x006f6100, fn_006f6100(Ptr) -> bool),
        entry!(0x006f6150, fn_006f6150(Ptr, bool)),
        entry!(0x006f6250, fn_006f6250(Ptr, u32) -> u32),
        entry!(0x006f6370, fn_006f6370(Ptr, u32, u32, i16, i16)),
        entry!(0x006f6520, fn_006f6520(Ptr) -> u8),
        entry!(0x006f6540, fn_006f6540(Ptr, u32, u32, i16, i16)),
        entry!(0x006f66e0, fn_006f66e0(Ptr)),
        entry!(
            0x006f6d10,
            bgs_distant_object_block_load_task_construct(
                Ptr, u32, u32, u32, u32, u32, u8, u8, u8
            ) -> Ptr
        ),
        entry!(
            0x006f6fb0,
            bgs_distant_object_block_load_task_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006f6fe0, fn_006f6fe0(Ptr)),
        entry!(0x006f7070, bgs_distant_object_block_load_task_queue_me(Ptr)),
        entry!(0x006f71e0, bgs_distant_object_block_load_task_run(Ptr)),
        entry!(0x006f73b0, fn_006f73b0(Ptr)),
        entry!(0x006f73d0, bgs_distant_object_block_load_task_finish(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME_SLOT_DOUBLE: u32 = 0xdead_4000;
    const DETACH_DOUBLE: u32 = 0xdead_4010;
    const ATTACH_DOUBLE: u32 = 0xdead_4020;
    const QUEUE_SLOT_DOUBLE: u32 = 0xdead_4030;
    const CLONE_SLOT_DOUBLE: u32 = 0xdead_4040;
    const OWNER_SLOT_DOUBLE: u32 = 0xdead_4050;
    const RENDERER_SLOT_DOUBLE: u32 = 0xdead_4060;
    const OBJECT_SLOT_DOUBLE: u32 = 0xdead_4070;
    const TASK_28_DOUBLE: u32 = 0xdead_4080;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    /// An object whose vtable has each double at its slot.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        for &(slot, double) in slots {
            e.mem.set_u32(vtable + slot, double);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// An engine with the pointer and list helpers doubled as the game's
    /// and the statics mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_8000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        for assign in [POINTER_ASSIGN, TASK_POINTER_ASSIGN] {
            e.register(assign, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
        }
        e.register(POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e
    }

    /// A block whose terrain node leads to a parent node with the detach
    /// and attach slots doubled.
    fn block(e: &mut Engine) -> (u32, u32) {
        let parent = object(
            e,
            0x10,
            &[
                (NODE_SLOT_DETACH_CHILD, DETACH_DOUBLE),
                (NODE_SLOT_ATTACH_CHILD, ATTACH_DOUBLE),
            ],
        );
        quiet(e, &[DETACH_DOUBLE, ATTACH_DOUBLE]);
        let terrain = e.mem.alloc(8);
        e.mem.set_u32(terrain, 0x4400);
        e.register_double(TERRAIN_PARENT_NODE, move |_, a| {
            ret(if a[0] == 0x4400 { parent } else { 0 })
        });
        let block = e.mem.alloc(0x2c);
        e.mem.set_u32(block + BLOCK_NODE, terrain);
        (block, parent)
    }

    fn world_space(e: &mut Engine) -> u32 {
        object(e, 8, &[(WORLD_SPACE_SLOT_NAME, NAME_SLOT_DOUBLE)])
    }

    #[test]
    fn file_names_put_the_world_space_name_twice() {
        let mut e = engine();
        let world_space = world_space(&mut e);
        let mut answer = 0;
        e.register_double(NAME_SLOT_DOUBLE, move |_, _| {
            answer += 1;
            ret(answer)
        });
        quiet(&mut e, &[PRINT_FORMATTED]);
        e.call_log = Some(vec![]);
        e.call(
            0x006f_55f0,
            &args![0x900u32, world_space, 4u32, 0x1_ffffu32, 7u32],
        );
        e.call(
            0x006f_5640,
            &args![0x900u32, world_space, 4u32, 2u32, 0x8000u32],
        );
        e.call(0x006f_5780, &args![0x900u32, world_space]);
        e.call(0x006f_57c0, &args![0x900u32, world_space]);
        assert_eq!(
            calls(&e, PRINT_FORMATTED),
            vec![
                vec![0x900, 0x104, BLOCK_FILE_FORMAT, 2, 1, 4, 0xffff_ffff, 7],
                vec![
                    0x900,
                    0x104,
                    BLOCK_FILE_HIGH_PRIORITY_FORMAT,
                    4,
                    3,
                    4,
                    2,
                    0xffff_8000
                ],
                vec![0x900, 0x104, DIFFUSE_TEXTURE_FORMAT, 6, 5],
                vec![0x900, 0x104, NORMAL_TEXTURE_FORMAT, 8, 7],
            ]
        );
    }

    #[test]
    fn small_accessors_read_and_set_their_fields() {
        let mut e = engine();
        let block = e.mem.alloc(0xe00);
        e.mem.set_u32(block + BLOCK_SCENE_NODE, 0x81);
        e.mem.set_u32(block + BLOCK_STINGER, 0x82);
        e.mem.set_u8(block + BLOCK_HIGH_PRIORITY, 5);
        e.mem.set_u8(block + 0xdf2, 6);
        assert_eq!(e.call(0x006f_5800, &args![block, 0u32]).u32(), 0x81);
        assert_eq!(e.call(0x006f_5800, &args![block, 0x100u32]).u32(), 0x81);
        assert_eq!(e.call(0x006f_5800, &args![block, 1u32]).u32(), 0x82);
        assert_eq!(e.call(0x006f_6070, &args![block]).u32(), 5);
        assert_eq!(e.call(0x006f_6520, &args![block]).u32(), 6);
        e.call(0x006f_73b0, &args![block]);
        assert_eq!(e.mem.u8(block + BLOCK_DETACH_FLAG), 1);
        e.call_log = Some(vec![]);
        quiet(&mut e, &[POINTER_COPY]);
        e.call(0x006f_6250, &args![0x5000u32, 0x6000u32]);
        assert_eq!(calls(&e, POINTER_COPY), vec![vec![0x6000, 0x504c]]);
    }

    #[test]
    fn blocks_loaded_membership_add_and_remove() {
        let mut e = engine();
        let block = e.mem.alloc(0x2c);
        let other = e.mem.alloc(0x2c);
        let second = e.mem.alloc(8);
        e.mem.set_u32(BLOCKS_LOADED, other);
        e.mem.set_u32(BLOCKS_LOADED + 4, second);
        e.mem.set_u32(second, block);
        assert!(e.call(0x006f_6100, &args![block]).bool());
        let item = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let added = item.clone();
        e.register_double(SIMPLE_LIST_ADD, move |e, a| {
            added.set(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006f_6090, &args![block]);
        assert_eq!(item.get(), block);
        assert_eq!(calls(&e, SIMPLE_LIST_ADD)[0][0], BLOCKS_LOADED);
        e.register_double(SIMPLE_LIST_REMOVE, move |e, a| {
            let removed = e.mem.u32(a[1]);
            if e.mem.u32(second) == removed {
                e.mem.set_u32(second, 0);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x006f_60c0, &args![block]);
        assert_eq!(calls(&e, SIMPLE_LIST_REMOVE).len(), 1);
        assert!(!e.call(0x006f_6100, &args![block]).bool());
    }

    #[test]
    fn refresh_walks_the_loaded_blocks() {
        let mut e = engine();
        let shown = e.mem.alloc(0x2c);
        e.mem.set_u32(shown + BLOCK_SCENE_NODE, 0x81);
        e.mem.set_u32(shown + BLOCK_STINGER, 0x82);
        let skipped = e.mem.alloc(0x2c);
        e.mem.set_u32(skipped + BLOCK_SCENE_NODE, 0x91);
        let second = e.mem.alloc(8);
        e.mem.set_u32(BLOCKS_LOADED, shown);
        e.mem.set_u32(BLOCKS_LOADED + 4, second);
        e.mem.set_u32(second, skipped);
        e.register_double(BLOCK_SKIP_CHECK, move |_, a| {
            ret(u32::from(a[0] == skipped))
        });
        e.register(NODE_DATA_6629F0, |_, a| ret(a[0] + 0x100));
        e.register(MODEL_ROOT, |_, a| ret(a[0] + 0x1000));
        quiet(&mut e, &[REFRESH_528820]);
        e.call_log = Some(vec![]);
        e.call(0x006f_5840, &args![]);
        assert_eq!(calls(&e, REFRESH_528820), vec![vec![0x1181], vec![0x1182]]);
    }

    #[test]
    fn detach_takes_the_meshes_off_the_parent() {
        let mut e = engine();
        let (block, parent) = block(&mut e);
        quiet(&mut e, &[SIMPLE_LIST_REMOVE]);
        e.mem.set_u32(block + BLOCK_SCENE_NODE, 0x81);
        e.mem.set_u32(block + BLOCK_MESH, 0x84);
        e.mem.set_u32(block + BLOCK_OLD_MESH, 0x8c);
        e.mem.set_u32(block + BLOCK_STINGER, 0x90);
        e.mem.set_u32(block + BLOCK_LOAD_TASK, 0x98);
        e.mem.set_u8(block + BLOCK_DETACH_FLAG, 1);
        e.set_global(COUNTER_011D8510, 5u32);
        e.call_log = Some(vec![]);
        e.call(0x006f_5af0, &args![block]);
        assert_eq!(e.global::<u32>(COUNTER_011D8510), 4);
        assert_eq!(
            calls(&e, DETACH_DOUBLE),
            vec![
                vec![parent, 0x81],
                vec![parent, 0x84],
                vec![parent, 0x8c],
                vec![parent, 0x90]
            ]
        );
        for member in [
            BLOCK_SCENE_NODE,
            BLOCK_MESH,
            BLOCK_OLD_MESH,
            BLOCK_STINGER,
            BLOCK_LOAD_TASK,
        ] {
            assert_eq!(e.mem.u32(block + member), 0);
        }
        assert_eq!(e.mem.u8(block + BLOCK_DETACH_FLAG), 0);
        // An empty block only clears the flag.
        e.call_log = Some(vec![]);
        e.call(0x006f_5af0, &args![block]);
        assert_eq!(e.global::<u32>(COUNTER_011D8510), 4);
        assert!(calls(&e, DETACH_DOUBLE).is_empty());
    }

    #[test]
    fn attach_swaps_the_meshes_and_moves_the_counts() {
        let mut e = engine();
        let (block, parent) = block(&mut e);
        quiet(&mut e, &[SIMPLE_LIST_REMOVE, SIMPLE_LIST_ADD]);
        e.register(MESH_DETACH_TARGET, |_, a| ret(a[0] + 0x1000));
        // The prepare finds the block prepared and nothing to redo.
        e.mem.set_u8(block + BLOCK_PREPARED, 1);
        e.register(MESHES_DIFFER, |_, _| ret(0));
        e.mem.set_u32(block + BLOCK_SCENE_NODE, 0x81);
        e.mem.set_u32(block + BLOCK_MESH, 0x84);
        e.mem.set_u32(block + BLOCK_OLD_MESH, 0x8c);
        e.mem.set_u32(block + BLOCK_STINGER, 0x90);
        e.mem.set_u8(block + BLOCK_HIGH_PRIORITY, 1);
        e.set_global(COUNTER_011D8510, 5u32);
        e.set_global(COUNTER_011D8514, 7u32);
        e.call_log = Some(vec![]);
        e.call(0x006f_5ca0, &args![block]);
        assert_eq!(calls(&e, DETACH_DOUBLE), vec![vec![parent, 0x108c]]);
        assert_eq!(
            calls(&e, ATTACH_DOUBLE),
            vec![vec![parent, 0x81, 1], vec![parent, 0x90, 1]]
        );
        assert_eq!(e.global::<u32>(COUNTER_011D8510), 4);
        assert_eq!(e.global::<u32>(COUNTER_011D8514), 8);
        assert_eq!(calls(&e, SIMPLE_LIST_ADD).len(), 1);
        assert_eq!(e.mem.u32(block + BLOCK_OLD_MESH), 0);
        assert_eq!(e.mem.u8(block + BLOCK_ATTACHED), 1);
        // Attached already: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x006f_5ca0, &args![block]);
        assert!(calls(&e, ATTACH_DOUBLE).is_empty());
        // The old mesh without a node is replaced by the mesh.
        e.mem.set_u8(block + BLOCK_ATTACHED, 0);
        e.mem.set_u32(block + BLOCK_SCENE_NODE, 0);
        e.mem.set_u32(block + BLOCK_OLD_MESH, 0x8c);
        e.mem.set_u32(block + BLOCK_OLD_STINGER, 0x94);
        e.mem.set_u8(block + BLOCK_HIGH_PRIORITY, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_5ca0, &args![block]);
        assert_eq!(
            calls(&e, DETACH_DOUBLE),
            vec![vec![parent, 0x8c], vec![parent, 0x94]]
        );
        assert_eq!(
            calls(&e, ATTACH_DOUBLE),
            vec![vec![parent, 0x84, 1], vec![parent, 0x90, 1]]
        );
        assert_eq!(e.global::<u32>(COUNTER_011D8514), 7);
        assert_eq!(e.global::<u32>(COUNTER_011D8510), 5);
    }

    #[test]
    fn take_results_copies_finished_tasks_and_marks_done() {
        let mut e = engine();
        let block = e.mem.alloc(0x2c);
        let task = e.mem.alloc(0x58);
        e.mem.set_u32(task + TASK_NODE, 0x77);
        let stinger_task = e.mem.alloc(0x58);
        e.mem.set_u32(stinger_task + TASK_NODE, 0x88);
        e.mem.set_u32(block + BLOCK_LOAD_TASK, task);
        e.mem.set_u32(block + BLOCK_LOAD_TASK_STINGER, stinger_task);
        e.register(TASK_IS_FINISHED, |_, _| ret(1));
        quiet(&mut e, &[TASK_COPY_MESH]);
        e.call_log = Some(vec![]);
        e.call(0x006f_6150, &args![block, 1u32]);
        assert_eq!(calls(&e, TASK_COPY_MESH), vec![vec![task, block + 4]]);
        assert_eq!(e.mem.u32(block + BLOCK_SCENE_NODE), 0x77);
        assert_eq!(e.mem.u32(block + BLOCK_STINGER), 0x88);
        assert_eq!(e.mem.u32(block + BLOCK_LOAD_TASK), 0);
        assert_eq!(e.mem.u32(block + BLOCK_LOAD_TASK_STINGER), 0);
        assert_eq!(e.mem.u8(block + BLOCK_DONE_LOADING), 1);
        // Without `take` the tasks are only dropped.
        e.mem.set_u8(block + BLOCK_DONE_LOADING, 0);
        e.mem.set_u32(block + BLOCK_SCENE_NODE, 0);
        e.mem.set_u32(block + BLOCK_LOAD_TASK, task);
        e.call_log = Some(vec![]);
        e.call(0x006f_6150, &args![block, 0u32]);
        assert!(calls(&e, TASK_COPY_MESH).is_empty());
        assert_eq!(e.mem.u32(block + BLOCK_SCENE_NODE), 0);
        assert_eq!(e.mem.u8(block + BLOCK_DONE_LOADING), 1);
    }

    /// Doubles for the load task's construction.
    fn construct_doubles(e: &mut Engine) {
        quiet(
            e,
            &[
                QUEUED_FILE_ENTRY_CONSTRUCT,
                POINTER_CONSTRUCT,
                PRINT_FORMATTED,
                BUILD_BLOCK_FILE_NAME_PA,
                BUILD_STINGER_BLOCK_FILE_NAME,
                BUILD_STINGER_BLOCK_FILE_NAME_HIGH_PRIORITY,
                FILE_EXISTS,
                QUEUED_FILE_SET_NAME,
                QUEUED_FILE_C3CF60,
                NAME_SLOT_DOUBLE,
            ],
        );
        let player = e.mem.alloc(0xe00);
        e.set_global(PLAYER, player);
    }

    #[test]
    fn reloads_queue_a_new_load_task() {
        let mut e = engine();
        construct_doubles(&mut e);
        let world_space = world_space(&mut e);
        quiet(
            &mut e,
            &[MEMORY_CONTEXT_ENTER, MEMORY_CONTEXT_LEAVE, CANCEL_TASK],
        );
        e.set_global(TASK_MANAGER, 0x3300u32);
        let queued = object(&mut e, 0x58, &[(TASK_SLOT_QUEUE, QUEUE_SLOT_DOUBLE)]);
        quiet(&mut e, &[QUEUE_SLOT_DOUBLE]);
        let fresh = e.mem.alloc(0x58);
        e.register_double(ALLOCATE, move |_, _| ret(fresh));
        let block = e.mem.alloc(0x2c);
        e.mem.set_u32(block + BLOCK_MESH, 0x84);
        e.mem.set_u32(block + BLOCK_STINGER, 0x90);
        e.mem.set_u32(block + BLOCK_LOAD_TASK, queued);
        e.mem.set_u32(block + BLOCK_DETACH_COUNT, 3);
        e.mem.set_u8(block + BLOCK_ATTACHED, 1);
        // The new task gets the load task's vtable; give it the queue slot.
        e.put_vtable(LOAD_TASK_VTABLE, &[0; 9]);
        e.mem
            .set_u32(LOAD_TASK_VTABLE + TASK_SLOT_QUEUE, QUEUE_SLOT_DOUBLE);
        e.call_log = Some(vec![]);
        e.call(
            0x006f_6370,
            &args![block, world_space, 2u32, 0xfffeu32, 3u32],
        );
        assert_eq!(
            calls(&e, MEMORY_CONTEXT_ENTER)[0][1..],
            [0x24, 1, SOURCE_FILE_NAME, 0x250]
        );
        assert_eq!(e.mem.u32(block + BLOCK_OLD_MESH), 0x84);
        assert_eq!(e.mem.u32(block + BLOCK_OLD_STINGER), 0x90);
        assert_eq!(calls(&e, CANCEL_TASK), vec![vec![0x3300, queued, 0]]);
        assert_eq!(e.mem.u32(block + BLOCK_LOAD_TASK), fresh);
        assert_eq!(e.mem.u32(fresh + TASK_BLOCK), block);
        assert_eq!(e.mem.u32(fresh + TASK_X), 0xffff_fffe);
        assert_eq!(e.mem.u32(fresh + TASK_LEVEL), 2);
        assert_eq!(calls(&e, QUEUE_SLOT_DOUBLE), vec![vec![fresh]]);
        assert_eq!(e.mem.u32(block + BLOCK_DETACH_COUNT), 0);
        assert_eq!(e.mem.u8(block + BLOCK_ATTACHED), 0);
        assert_eq!(e.mem.u8(block + BLOCK_HIGH_PRIORITY), 0);
        assert_eq!(calls(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_6540, &args![block, world_space, 2u32, 1u32, 3u32]);
        assert_eq!(calls(&e, MEMORY_CONTEXT_ENTER)[0][4], 0x272);
        assert_eq!(e.mem.u8(block + BLOCK_HIGH_PRIORITY), 1);
        // A high-priority task builds the high-priority name.
        assert_eq!(
            calls(&e, PRINT_FORMATTED)[0][2],
            BLOCK_FILE_HIGH_PRIORITY_FORMAT
        );
    }

    #[test]
    fn construct_picks_the_file_name_by_flags() {
        let mut e = engine();
        construct_doubles(&mut e);
        let world_space = world_space(&mut e);
        let task = e.mem.alloc(0x58);
        let construct = |e: &mut Engine, flag: u32, high: u32, stinger: u32| {
            e.call_log = Some(vec![]);
            let words = args![
                task,
                0x500u32,
                world_space,
                0x1_0005u32,
                6u32,
                7u32,
                flag,
                high,
                stinger
            ];
            assert_eq!(e.call(0x006f_6d10, &words).u32(), task);
        };
        construct(&mut e, 0, 0, 0);
        assert_eq!(e.mem.u32(task), LOAD_TASK_VTABLE);
        assert_eq!(e.mem.u32(task + TASK_X), 0x1_0005);
        assert_eq!(e.mem.u32(task + TASK_WORLD_SPACE), world_space);
        assert_eq!(calls(&e, QUEUED_FILE_ENTRY_CONSTRUCT), vec![vec![task, 3]]);
        let printed = calls(&e, PRINT_FORMATTED);
        assert_eq!(printed[0][2], BLOCK_FILE_FORMAT);
        assert_eq!(printed[0][5..], [7, 5, 6]);
        let name = calls(&e, QUEUED_FILE_SET_NAME)[0][1];
        assert_eq!(name, printed[0][0] + 5);
        assert_eq!(calls(&e, QUEUED_FILE_C3CF60), vec![vec![task, 0]]);
        // The flag: the PA name, then the plain one when it does not exist.
        construct(&mut e, 1, 0, 0);
        assert_eq!(
            calls(&e, BUILD_BLOCK_FILE_NAME_PA)[0][1..],
            [world_space, 7, 5, 6]
        );
        assert_eq!(calls(&e, FILE_EXISTS)[0][1..], [0, 0, 0xffff_ffff]);
        assert_eq!(calls(&e, PRINT_FORMATTED).len(), 1);
        e.register(FILE_EXISTS, |_, _| ret(1));
        construct(&mut e, 1, 0, 0);
        assert!(calls(&e, PRINT_FORMATTED).is_empty());
        // The stinger version.
        construct(&mut e, 0, 1, 2);
        assert_eq!(e.mem.u8(task + TASK_STINGER_VERSION), 2);
        assert_eq!(
            calls(&e, BUILD_STINGER_BLOCK_FILE_NAME_HIGH_PRIORITY).len(),
            1
        );
        construct(&mut e, 0, 0, 1);
        assert_eq!(calls(&e, BUILD_STINGER_BLOCK_FILE_NAME).len(), 1);
    }

    #[test]
    fn prepare_sets_up_the_mesh_and_the_stinger() {
        let mut e = engine();
        let block = e.mem.alloc(0x2c);
        e.mem.set_u32(block + BLOCK_NODE, 0x4300);
        e.mem.set_u32(block + BLOCK_MESH, 0x84);
        e.mem.set_u32(block + BLOCK_STINGER, 0x90);
        let shader_property = object(&mut e, 8, &[(SHADER_SLOT_CLONE, CLONE_SLOT_DOUBLE)]);
        let owner = object(&mut e, 8, &[(OWNER_SLOT_118, OWNER_SLOT_DOUBLE)]);
        let renderer = object(&mut e, 8, &[(RENDERER_SLOT_EC, RENDERER_SLOT_DOUBLE)]);
        e.register(PROPERTY_TYPE_40DF90, |_, _| ret(9));
        e.register(NI_ALLOCATE, |_, _| ret(0x6600));
        e.register(PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register(SHADER_PROPERTY_TYPE, |_, _| ret(3));
        e.register_double(GET_PROPERTY, move |_, _| ret(shader_property));
        e.register(CLONE_SLOT_DOUBLE, |_, _| ret(0x6700));
        e.register(SHADER_ID, |_, _| ret(0x11));
        e.register(GET_SHADER, |_, a| ret(a[0] + 0x100));
        e.register(MESH_DATA, |_, _| ret(0x6800));
        e.register_double(MESH_OWNER_4030B0, move |_, _| ret(owner));
        e.register_double(RENDERER, move |_, _| ret(renderer));
        e.register(OWNER_SLOT_DOUBLE, |_, _| ret(0x55));
        quiet(
            &mut e,
            &[
                REMOVE_PROPERTY,
                PROPERTY_SET_4393C0,
                PROPERTY_SET_49ED90,
                PROPERTY_SET_94DB80,
                PROPERTY_SET_4393E0,
                ATTACH_PROPERTY,
                SHADER_SET_FLAG,
                SET_SHADER,
                UPDATE_DATA_CONSTRUCT,
                UPDATE_PROPERTIES,
                UPDATE_DOWNWARD,
                UPDATE_BLOCK_VISIBILITY,
                DATA_SET_4410D0,
                DATA_SET_CONSISTENCY,
                DATA_SET_4410F0,
                RENDERER_SLOT_DOUBLE,
                RENDERER_E74120,
                PREPARE_STINGER,
                MESHES_DIFFER,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x006f_66e0, &args![block]);
        assert_eq!(
            calls(&e, REMOVE_PROPERTY),
            vec![vec![0x84, 9], vec![0x84, 3]]
        );
        assert_eq!(
            calls(&e, ATTACH_PROPERTY),
            vec![vec![0x84, 0x6600], vec![0x84, 0x6700]]
        );
        assert_eq!(calls(&e, PROPERTY_SET_94DB80), vec![vec![0x6600, 0x80]]);
        assert_eq!(
            calls(&e, SHADER_SET_FLAG),
            vec![
                vec![shader_property, 0x22, 1],
                vec![shader_property, 0x21, 0],
                vec![shader_property, 0, 0]
            ]
        );
        assert_eq!(
            calls(&e, CLONE_SLOT_DOUBLE),
            vec![vec![shader_property, 0x84, 0, 1]]
        );
        assert_eq!(calls(&e, SHADER_ID), vec![vec![0x6700]]);
        assert_eq!(calls(&e, SET_SHADER), vec![vec![0x84, 0x111]]);
        assert_eq!(calls(&e, UPDATE_BLOCK_VISIBILITY), vec![vec![0x4300, 0]]);
        assert_eq!(calls(&e, DATA_SET_CONSISTENCY), vec![vec![0x6800, 0x4000]]);
        assert_eq!(
            calls(&e, OWNER_SLOT_DOUBLE),
            vec![vec![owner, 0x84, 0x6700]]
        );
        assert_eq!(
            calls(&e, RENDERER_SLOT_DOUBLE),
            vec![vec![renderer, 0x84, 0, 0, 0x55]]
        );
        assert_eq!(calls(&e, PREPARE_STINGER), vec![vec![block, 0x90]]);
        assert_eq!(calls(&e, RENDERER_E74120).len(), 2);
        assert_eq!(e.mem.u8(block + BLOCK_PREPARED), 1);
        // Prepared, with old meshes that do not differ: nothing is redone.
        e.mem.set_u32(block + BLOCK_OLD_MESH, 0x8c);
        e.mem.set_u32(block + BLOCK_OLD_STINGER, 0x94);
        e.call_log = Some(vec![]);
        e.call(0x006f_66e0, &args![block]);
        assert_eq!(calls(&e, MESHES_DIFFER).len(), 2);
        assert!(calls(&e, REMOVE_PROPERTY).is_empty());
        assert!(calls(&e, PREPARE_STINGER).is_empty());
    }

    #[test]
    fn task_destructor_releases_the_pointers() {
        let mut e = engine();
        quiet(&mut e, &[POINTER_DESTROY, QUEUED_FILE_ENTRY_DESTROY, FREE]);
        let task = e.mem.alloc(0x58);
        e.mem.set_u32(task + TASK_MESH, 0x48);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_6fb0, &args![task, 1u32]).u32(), task);
        assert_eq!(e.mem.u32(task), LOAD_TASK_VTABLE);
        assert_eq!(e.mem.u32(task + TASK_MESH), 0);
        assert_eq!(
            calls(&e, POINTER_DESTROY),
            vec![vec![task + 0x4c], vec![task + 0x48]]
        );
        assert_eq!(calls(&e, QUEUED_FILE_ENTRY_DESTROY), vec![vec![task]]);
        assert_eq!(calls(&e, FREE), vec![vec![task]]);
        e.call_log = Some(vec![]);
        e.call(0x006f_6fb0, &args![task, 0u32]);
        assert!(calls(&e, FREE).is_empty());
    }

    #[test]
    fn queue_me_queues_both_textures_unless_stinger() {
        let mut e = engine();
        let world_space = world_space(&mut e);
        quiet(
            &mut e,
            &[
                NAME_SLOT_DOUBLE,
                PRINT_FORMATTED,
                QUEUE_TEXTURE,
                TASK_28_DOUBLE,
            ],
        );
        e.register(TASK_QUEUE_WORD, |_, _| ret(0x31));
        e.set_global(MODEL_LOADER, 0x3100u32);
        let task = object(&mut e, 0x58, &[(TASK_SLOT_28, TASK_28_DOUBLE)]);
        e.mem.set_u32(task + TASK_WORLD_SPACE, world_space);
        e.call_log = Some(vec![]);
        e.call(0x006f_7070, &args![task]);
        let printed = calls(&e, PRINT_FORMATTED);
        assert_eq!(printed[0][2], DIFFUSE_TEXTURE_FORMAT);
        assert_eq!(printed[1][2], NORMAL_TEXTURE_FORMAT);
        let queued = calls(&e, QUEUE_TEXTURE);
        assert_eq!(queued.len(), 2);
        assert_eq!(queued[0], vec![0x3100, printed[0][0], 0x31, task]);
        assert_eq!(queued[1][1], printed[1][0]);
        assert_eq!(e.mem.u8(task + TASK_FINISHED_QUEUING), 1);
        assert_eq!(calls(&e, TASK_28_DOUBLE), vec![vec![task]]);
        e.mem.set_u8(task + TASK_STINGER_VERSION, 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_7070, &args![task]);
        assert!(calls(&e, QUEUE_TEXTURE).is_empty());
        assert_eq!(calls(&e, TASK_28_DOUBLE).len(), 1);
    }

    #[test]
    fn run_keeps_the_loaded_node_and_its_mesh() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MEMORY_CONTEXT_ENTER,
                MEMORY_CONTEXT_LEAVE,
                STREAM_CONSTRUCT,
                STREAM_DESTROY,
            ],
        );
        let loaded = object(&mut e, 8, &[(OBJECT_SLOT_C, OBJECT_SLOT_DOUBLE)]);
        e.register(QUEUED_FILE_GET_FILE, |_, _| ret(0x2200));
        e.register(QUEUED_FILE_NAME, |_, _| ret(0x2300));
        e.register(STREAM_LOAD, |_, _| ret(1));
        e.register_double(STREAM_OBJECT, move |_, _| ret(loaded));
        e.register(OBJECT_SLOT_DOUBLE, |_, _| ret(1));
        e.register(NODE_FIND_MESH, |_, a| ret(a[0] + 4));
        let task = e.mem.alloc(0x58);
        e.call_log = Some(vec![]);
        e.call(0x006f_71e0, &args![task]);
        assert_eq!(
            calls(&e, MEMORY_CONTEXT_ENTER)[0][1..],
            [0x24, 1, SOURCE_FILE_NAME, 0x39a]
        );
        assert_eq!(calls(&e, QUEUED_FILE_GET_FILE), vec![vec![task, 0, 1]]);
        assert_eq!(calls(&e, STREAM_LOAD)[0][1..], [0x2300, 0x2200]);
        assert_eq!(e.mem.u32(task + TASK_NODE), loaded);
        assert_eq!(e.mem.u32(task + TASK_MESH), loaded + 4);
        // An object that is not a node becomes the mesh.
        e.register(OBJECT_SLOT_DOUBLE, |_, _| ret(0));
        e.mem.set_u32(task + TASK_NODE, 0);
        e.call(0x006f_71e0, &args![task]);
        assert_eq!(e.mem.u32(task + TASK_MESH), loaded);
        assert_eq!(e.mem.u32(task + TASK_NODE), 0);
    }

    #[test]
    fn run_without_a_file_flags_a_high_priority_block() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                MEMORY_CONTEXT_ENTER,
                MEMORY_CONTEXT_LEAVE,
                STREAM_CONSTRUCT,
                STREAM_DESTROY,
                QUEUED_FILE_GET_FILE,
                FILE_EXISTS,
                STREAM_LOAD,
            ],
        );
        e.register(QUEUED_FILE_NAME, |_, _| ret(0x2300));
        let block = e.mem.alloc(0x2c);
        e.mem.set_u8(block + BLOCK_HIGH_PRIORITY, 1);
        let task = e.mem.alloc(0x58);
        e.mem.set_u32(task + TASK_BLOCK, block);
        e.call_log = Some(vec![]);
        e.call(0x006f_71e0, &args![task]);
        assert_eq!(calls(&e, FILE_EXISTS), vec![vec![0x2300, 0, 6, 1]]);
        assert!(calls(&e, STREAM_LOAD).is_empty());
        assert_eq!(e.mem.u8(block + BLOCK_DETACH_FLAG), 1);
        assert_eq!(calls(&e, STREAM_DESTROY).len(), 1);
    }

    #[test]
    fn finish_hands_the_results_to_the_block() {
        let mut e = engine();
        quiet(&mut e, &[TERRAIN_929260, PRINT_MESSAGE, TASK_IS_FINISHED]);
        let found_mesh = object(&mut e, 8, &[(MESH_SLOT_28, TASK_28_DOUBLE)]);
        quiet(&mut e, &[TASK_28_DOUBLE]);
        let terrain = e.mem.alloc(8);
        e.mem.set_u32(terrain, 0x4400);
        let block = e.mem.alloc(0x2c);
        e.mem.set_u32(block + BLOCK_NODE, terrain);
        let task = e.mem.alloc(0x58);
        e.mem.set_u32(task + TASK_MESH, found_mesh);
        e.mem.set_u32(task + TASK_BLOCK, block);
        e.mem.set_u32(task + TASK_X, 1);
        e.mem.set_u32(task + TASK_Y, 2);
        e.mem.set_u32(task + TASK_LEVEL, 4);
        e.call_log = Some(vec![]);
        e.call(0x006f_73d0, &args![task]);
        assert_eq!(calls(&e, TERRAIN_929260), vec![vec![0x4400, 1]]);
        assert_eq!(
            calls(&e, PRINT_MESSAGE),
            vec![vec![EMPTY_BLOCK_MESSAGE, 4, 1, 2]]
        );
        // Canceled: the results are dropped.
        e.mem.set_u8(task + TASK_CANCELED, 1);
        e.mem.set_u32(task + TASK_NODE, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x006f_73d0, &args![task]);
        assert_eq!(e.mem.u32(task + TASK_MESH), 0);
        assert_eq!(e.mem.u32(task + TASK_NODE), 0);
        assert!(calls(&e, TERRAIN_929260).is_empty());
        // No mesh and not canceled: the block drops the task.
        e.mem.set_u8(task + TASK_CANCELED, 0);
        let pending = e.mem.alloc(0x58);
        e.mem.set_u32(block + BLOCK_LOAD_TASK, pending);
        e.register(TASK_IS_FINISHED, |_, _| ret(1));
        e.call(0x006f_73d0, &args![task]);
        assert_eq!(e.mem.u32(block + BLOCK_LOAD_TASK), 0);
        assert_eq!(e.mem.u8(block + BLOCK_DONE_LOADING), 1);
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn file_name_builders_use_their_formats() {
        let mut e = Engine::new();
        let mut slots = vec![0u32; 0x4d];
        slots[0x130 / 4] = 0x0150_1130;
        e.put_vtable(0x0150_0000, &slots);
        e.register(0x0150_1130, |_, _| ret(0x4a4e));
        e.register(PRINT_FORMATTED, |_, _| Ret::default());
        let world = e.mem.alloc(4);
        e.mem.set_u32(world, 0x0150_0000);
        for (address, format) in [
            (0x006f_5690u32, BLOCK_FILE_PA_FORMAT),
            (0x006f_56e0, STINGER_FILE_FORMAT),
            (0x006f_5730, STINGER_FILE_HIGH_PRIORITY_FORMAT),
        ] {
            e.call_log = Some(vec![]);
            e.call(address, &args![0x900u32, world, 3u32, 0x1_fffeu32, 5u32]);
            assert_eq!(
                calls_to(&e.call_log.take().unwrap(), PRINT_FORMATTED),
                vec![vec![
                    0x900,
                    0x104,
                    format,
                    0x4a4e,
                    0x4a4e,
                    3,
                    (-2i32) as u32,
                    5
                ]]
            );
        }
    }

    /// A tree: a node (slot 0xC answers itself) whose children are at +0xC
    /// on, their count at +8; a leaf's slot 0xC answers 0.
    fn tree(e: &mut Engine, children: &[u32]) -> u32 {
        e.put_vtable(0x0150_0000, &[0, 0, 0, 0x0150_100c, 0, 0, 0x0150_1018]);
        e.put_vtable(0x0150_0100, &[0, 0, 0, 0x0150_110c, 0, 0, 0x0150_1118]);
        e.register(0x0150_100c, |_, a| ret(a[0]));
        e.register(0x0150_110c, |_, _| ret(0));
        e.register(0x0150_1118, |_, a| ret(a[0] + 0x1000));
        e.register(CHILD_COUNT_00453470, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(NODE_FIND_MESH, |e, a| ret(e.mem.u32(a[0] + 0xc + 4 * a[1])));
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node, 0x0150_0000);
        e.mem.set_u32(node + 8, children.len() as u32);
        for (i, child) in (0u32..).zip(children) {
            e.mem.set_u32(node + 0xc + 4 * i, *child);
        }
        node
    }

    fn leaf(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, 0x0150_0100);
        object
    }

    #[test]
    fn visibility_walks_the_tree() {
        let mut e = Engine::new();
        let a = leaf(&mut e);
        let b = leaf(&mut e);
        let root = tree(&mut e, &[a, b]);
        e.register(SET_HIDDEN_00450F90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_58e0, &args![root, 0x100u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), SET_HIDDEN_00450F90),
            vec![vec![root, 1], vec![a, 1], vec![b, 1]]
        );
        e.call_log = Some(vec![]);
        e.call(0x006f_58e0, &args![root, 1u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), SET_HIDDEN_00450F90),
            vec![vec![root, 0], vec![a, 0], vec![b, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x006f_5a70, &args![0u32, root]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), SET_HIDDEN_00450F90),
            vec![vec![root, 0], vec![a, 0], vec![b, 0]]
        );
    }

    #[test]
    fn stinger_visibility_follows_the_cells() {
        let mut e = Engine::new();
        let a = leaf(&mut e);
        let root = tree(&mut e, &[0, a]);
        e.register(SET_HIDDEN_00450F90, |_, _| Ret::default());
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position, 4.0);
        e.register_double(WORLD_POSITION_0045BB80, move |_, _| ret(position));
        e.map(TES & !0xfff, 0x1000);
        e.set_global(TES, 0x7e5u32);
        e.register(TES_GET_WORLD_SPACE, |_, _| ret(0x3a3a));
        e.register(WORLD_SPACE_CELL_AT, |e, a| {
            assert_eq!(e.mem.f32(a[1]), 4.0);
            ret(0xce11)
        });
        e.register(TES_IS_CELL_LOADED, |_, _| ret(1));
        e.register(CELL_ALL_3D_FADING_IN, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x006f_5970, &args![0u32, root, 9u32, 0xffff_0001u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SET_HIDDEN_00450F90), vec![vec![a, 1]]);
        assert_eq!(
            calls_to(&log, TES_IS_CELL_LOADED),
            vec![vec![0x7e5, 0xce11, 1]]
        );
        e.register(CELL_ALL_3D_FADING_IN, |_, _| ret(0x100));
        e.call_log = Some(vec![]);
        e.call(0x006f_5970, &args![0u32, a, 0u32, 0u32, 0u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), SET_HIDDEN_00450F90),
            vec![vec![a, 0]]
        );
    }

    #[test]
    fn stinger_load_queues_a_high_priority_task() {
        let mut e = Engine::new();
        noop(
            &mut e,
            &[
                MEMORY_CONTEXT_ENTER,
                MEMORY_CONTEXT_LEAVE,
                POINTER_COPY,
                TASK_POINTER_ASSIGN,
            ],
        );
        e.register(ALLOCATE, |_, _| ret(0));
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        let mut slots = vec![0u32; 9];
        slots[8] = 0x0150_1020;
        e.put_vtable(0x0150_0000, &slots);
        e.register(0x0150_1020, |_, _| Ret::default());
        let task = e.mem.alloc(0x58);
        e.mem.set_u32(task, 0x0150_0000);
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + BLOCK_LOAD_TASK, task);
        e.mem.set_u8(this + BLOCK_DONE_LOADING, 1);
        e.mem.set_u8(this + BLOCK_ATTACHED, 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_6270, &args![this, 0x3a3au32, 2u32, 0xffffu32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMORY_CONTEXT_ENTER)[0][1..],
            [0x24, 1, SOURCE_FILE_NAME, 0x240]
        );
        assert_eq!(
            calls_to(&log, POINTER_COPY),
            vec![vec![this + BLOCK_OLD_MESH, this + BLOCK_MESH]]
        );
        assert_eq!(
            calls_to(&log, TASK_POINTER_ASSIGN),
            vec![vec![this + BLOCK_LOAD_TASK, 0]]
        );
        assert_eq!(calls_to(&log, 0x0150_1020), vec![vec![task]]);
        assert_eq!(e.mem.u8(this + BLOCK_DONE_LOADING), 0);
        assert_eq!(e.mem.u8(this + BLOCK_ATTACHED), 0);
        assert!(calls_to(&log, CANCEL_TASK).is_empty());
    }

    #[test]
    fn stinger_prepare_sets_up_each_geometry() {
        let mut e = Engine::new();
        let a = leaf(&mut e);
        let root = tree(&mut e, &[a]);
        noop(
            &mut e,
            &[
                REMOVE_PROPERTY,
                PROPERTY_SET_4393C0,
                PROPERTY_SET_49ED90,
                PROPERTY_SET_94DB80,
                PROPERTY_SET_4393E0,
                ATTACH_PROPERTY,
                SHADER_SET_FLAG,
                SET_SHADER,
                UPDATE_DATA_CONSTRUCT,
                UPDATE_PROPERTIES,
                UPDATE_DOWNWARD,
                UPDATE_BLOCK_VISIBILITY,
                DATA_SET_4410D0,
                DATA_SET_CONSISTENCY,
                DATA_SET_4410F0,
            ],
        );
        e.register(SHADER_PROPERTY_TYPE, |_, _| ret(0x5a));
        e.register(PROPERTY_TYPE_40DF90, |_, _| ret(0x40));
        e.register(NI_ALLOCATE, |_, _| ret(0x1c1c));
        e.register(PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        let shader_property = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x2d];
        slots[0xb0 / 4] = 0x0150_10b0;
        e.put_vtable(0x0150_0200, &slots);
        e.mem.set_u32(shader_property, 0x0150_0200);
        e.register(0x0150_10b0, |_, _| ret(0));
        e.register_double(GET_PROPERTY, move |_, _| ret(shader_property));
        e.register(SHADER_ID, |_, _| ret(7));
        e.register(GET_SHADER, |_, _| ret(0x5ade));
        e.register(MESH_DATA, |_, a| ret(a[0] + 0x10));
        let owner = e.mem.alloc(4);
        let renderer = e.mem.alloc(4);
        let mut slots = vec![0u32; 0x47];
        slots[0x118 / 4] = 0x0150_1218;
        slots[0xec / 4] = 0x0150_12ec;
        e.put_vtable(0x0150_0300, &slots);
        e.mem.set_u32(owner, 0x0150_0300);
        e.mem.set_u32(renderer, 0x0150_0300);
        e.register(0x0150_1218, |_, _| ret(0x118));
        e.register(0x0150_12ec, |_, _| Ret::default());
        e.register_double(MESH_OWNER_4030B0, move |_, _| ret(owner));
        e.register_double(RENDERER, move |_, _| ret(renderer));
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + BLOCK_NODE, 0x7e77);
        e.call_log = Some(vec![]);
        e.call(0x006f_6a90, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_6a90, &args![this, root]);
        let log = e.call_log.take().unwrap();
        let geometry = a + 0x1000;
        assert_eq!(calls_to(&log, REMOVE_PROPERTY), vec![vec![geometry, 0x40]]);
        assert_eq!(
            calls_to(&log, SHADER_SET_FLAG),
            vec![
                vec![shader_property, 0x22, 1],
                vec![shader_property, 0x37, 1]
            ]
        );
        assert_eq!(
            calls_to(&log, ATTACH_PROPERTY),
            vec![vec![geometry, 0x1c1c]]
        );
        assert_eq!(calls_to(&log, SET_SHADER), vec![vec![geometry, 0x5ade]]);
        assert_eq!(
            calls_to(&log, UPDATE_BLOCK_VISIBILITY),
            vec![vec![0x7e77, 0]]
        );
        assert_eq!(
            calls_to(&log, DATA_SET_CONSISTENCY),
            vec![vec![geometry + 0x10, 0x4000]]
        );
        assert_eq!(
            calls_to(&log, 0x0150_12ec),
            vec![vec![renderer, geometry, 0, 0, 0x118]]
        );
        assert_eq!(calls_to(&log, GET_PROPERTY).len(), 2);
    }

    #[test]
    fn task_cancel_once_and_description() {
        let mut e = Engine::new();
        e.register(QUEUED_FILE_CANCEL, |_, _| Ret::default());
        e.register(PRINT_FORMATTED, |_, _| Ret::default());
        let task = e.mem.alloc(0x58);
        e.call_log = Some(vec![]);
        e.call(0x006f_71a0, &args![task, 1u32, 2u32]);
        e.call(0x006f_71a0, &args![task, 1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, QUEUED_FILE_CANCEL), vec![vec![task, 1, 2]]);
        assert_eq!(e.mem.u8(task + TASK_CANCELED), 1);
        e.mem.set_u32(task + TASK_X, 4);
        e.mem.set_u32(task + TASK_Y, 5);
        e.mem.set_u32(task + TASK_LEVEL, 16);
        e.call_log = Some(vec![]);
        assert!(e.call(0x006f_74b0, &args![task, 0x900u32, 0x40u32]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), PRINT_FORMATTED),
            vec![vec![0x900, 0x40, TASK_DESCRIPTION_FORMAT, 4, 5, 16]]
        );
    }
}
