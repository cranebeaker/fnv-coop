//! `(unplaced)/run_006f50b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two save-game methods of a 0x30-byte node and the constructor and
//! destructor of `BGSDistantObjectBlock`; the engine map names none of
//! them.
//!
//! - `006f50b0`/`006f5100` are called by `PathingSolution::SaveGame`/
//!   `LoadGame` on the elements of one of its arrays. What they touch is
//!   `VirtualPathingNode`'s layout (Xbox PDB: `iFlags` +0, a
//!   `PathingLocation` +4 whose vtable slots +0/+4 are `SaveGame`/`LoadGame`,
//!   `pDoorRef` +0x2C, cast to `TESObjectREFR` on load).
//! - `006f5160` builds a `BGSDistantObjectBlock` (its memory-context scope
//!   names `BGSDistantObjectBlock.cpp`, and it creates a
//!   `BGSDistantObjectBlockLoadTask`); `006f5370` tears one down. The
//!   class's fields are read at their offsets (Xbox PDB names, 0x2C bytes).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_pathfinding::pathingsolution::VirtualPathingNode;

/// `00865e50` (engine map: `bgssavegamebuffer.cpp`): writes `size` bytes
/// from an address into the save buffer (thiscall: data, size, 0).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `00865df0`: `BGSSaveGameBuffer::SaveFormID_ov2` (engine map).
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// `00864980` (engine map: `bgsloadgamebuffer.cpp`): reads `size` bytes
/// from the load buffer into an address.
const LOAD_DATA: u32 = 0x0086_4980;
/// `008648a0`: `BGSLoadGameBuffer::LoadFormID` (engine map).
const LOAD_FORM_ID: u32 = 0x0086_48a0;
/// `004839c0` (engine map: `tesform.cpp`): the form for a form ID (cdecl).
const FORM_FOR_ID: u32 = 0x0048_39c0;
/// `00ec43fb`: `__RTDynamicCast` (cdecl: object, vfdelta, source type,
/// target type, is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptor of `TESForm` (RTTI name at +8).
const TESFORM_TYPE: u32 = 0x0118_3028;
/// The type descriptor of `TESObjectREFR` (RTTI name at +8).
const TESOBJECTREFR_TYPE: u32 = 0x0118_41cc;
/// `PathingLocation` vtable slots (Xbox PDB).
const SLOT_SAVE_GAME: u32 = 0x00;
const SLOT_LOAD_GAME: u32 = 0x04;

/// `BGSDistantObjectBlock` (Xbox PDB) field offsets.
const BLOCK_NODE: u32 = 0x00; // pNode, BGSTerrainNode*
const BLOCK_MESH: u32 = 0x04; // spMesh
const BLOCK_SCENE_NODE: u32 = 0x08; // spNode
const BLOCK_OLD_MESH: u32 = 0x0C; // spOldMesh
const BLOCK_NODE_STINGER: u32 = 0x10; // spNodeStinger
const BLOCK_NODE_STINGER_OLD: u32 = 0x14; // spNodeStingerOld
const BLOCK_LOAD_TASK: u32 = 0x18; // spLoadTask
const BLOCK_LOAD_TASK_STINGER: u32 = 0x1C; // spLoadTaskStinger
const BLOCK_DONE_LOADING: u32 = 0x20; // bDoneLoading
const BLOCK_ATTACHED: u32 = 0x21; // bAttached
const BLOCK_PREPARED: u32 = 0x22; // bPrepared
const BLOCK_HIGH_PRIORITY: u32 = 0x23; // bHighPriority
const BLOCK_DETACH_FLAG: u32 = 0x24; // bDetachFlag
const BLOCK_DETACH_COUNT: u32 = 0x28; // nDetachCount

/// `00633c90`: constructs a `NiPointer` holding a value (one word).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `00528cb0`: constructs the load-task `NiPointer`s (one word).
const TASK_POINTER_CONSTRUCT: u32 = 0x0052_8cb0;
/// `0066b0d0`: assigns a `NiPointer` (one word).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `006f74f0`: `NiPointer<QueuedFile>::operator=` (engine map).
const TASK_POINTER_ASSIGN: u32 = 0x006f_74f0;
/// `00559450`: reads the first word of an object (a `NiPointer`'s value).
const FIRST_WORD: u32 = 0x0055_9450;
/// `00404eb0` / `00404ee0`: a memory-context scope guard (pushed with a
/// context, a flag, a source file and a line) and its destructor.
const SCOPE_BEGIN: u32 = 0x0040_4eb0;
const SCOPE_END: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\...\BGSDistantObjectBlock.cpp"`.
const SOURCE_FILE: u32 = 0x0106_da18;
/// `00401000`: allocates a block (cdecl, size).
const ALLOCATE: u32 = 0x0040_1000;
/// `006f6d10`: `BGSDistantObjectBlockLoadTask::BGSDistantObjectBlockLoadTask`
/// (engine map), eight words.
const LOAD_TASK_CONSTRUCT: u32 = 0x006f_6d10;
/// Vtable slot +0x20 of the load task, called with no arguments after the
/// task is stored.
const SLOT_TASK_QUEUE: u32 = 0x20;

/// `006f60c0` (engine map: `bgsdistantobjectblock.cpp`): first step of the
/// teardown (thiscall, no arguments).
const BLOCK_DETACH: u32 = 0x006f_60c0;
/// A counter the teardown decrements once per attached mesh pair.
const ATTACHED_BLOCKS: u32 = 0x011d_8514;
/// `004ea9c0` (engine map: `teswater.cpp`): an object of the terrain node's
/// first word, whose vtable slot +0xE8 detaches a scene node.
const NODE_OWNER: u32 = 0x004e_a9c0;
const SLOT_DETACH_CHILD: u32 = 0xe8;
/// The task manager (`01202d98`) and `BSTaskManager<__int64>::CancelTask`
/// (`0044ac40`, engine map).
const TASK_MANAGER: u32 = 0x0120_2d98;
const CANCEL_TASK: u32 = 0x0044_ac40;
/// `00c45830`: releases a scene node's resources (cdecl, one word).
const RELEASE_NODE: u32 = 0x00c4_5830;
/// Global object (`011dea0c`) asked by `005bb4d0` whether textures are
/// purged; `00a61cd0` is `BSTexturePalette::PurgeUnusedTextures` (engine
/// map, no arguments).
const PURGE_SETTING_OWNER: u32 = 0x011d_ea0c;
const SHOULD_PURGE: u32 = 0x005b_b4d0;
const PURGE_UNUSED_TEXTURES: u32 = 0x00a6_1cd0;
/// Destructors of the two load-task pointers (`0044cbf0`) and of the five
/// scene pointers (`0045cec0`).
const TASK_POINTER_DESTROY: u32 = 0x0044_cbf0;
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;

// Translated from 006f50b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Saves a `VirtualPathingNode`: its 4-byte flags, the door reference's form
/// ID, then the location through its `SaveGame` slot.
pub fn fn_006f50b0(e: &mut Engine, this: Ptr<VirtualPathingNode>, buffer: Ptr) {
    e.call(SAVE_DATA, &args![buffer, this, 4u32, 0u32]);
    let door = e.get(this, VirtualPathingNode::pDoorRef);
    e.call(SAVE_FORM_ID, &args![buffer, door, 0u32]);
    let location = this.at(VirtualPathingNode::Location);
    e.vcall(location.addr(), SLOT_SAVE_GAME, &args![buffer]);
}

// Translated from 006f5100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads a `VirtualPathingNode`: 4 bytes of flags, a form ID resolved and
/// dynamic-cast from `TESForm` to `TESObjectREFR` into the door reference,
/// then the location through its `LoadGame` slot.
pub fn fn_006f5100(e: &mut Engine, this: Ptr<VirtualPathingNode>, buffer: Ptr) {
    e.call(LOAD_DATA, &args![buffer, this, 4u32]);
    let id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(FORM_FOR_ID, &args![id]).u32();
    let door = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TESFORM_TYPE, TESOBJECTREFR_TYPE, 0u32],
        )
        .u32();
    e.set(this, VirtualPathingNode::pDoorRef, Ptr::new(door));
    let location = this.at(VirtualPathingNode::Location);
    e.vcall(location.addr(), SLOT_LOAD_GAME, &args![buffer]);
}

// Translated from 006f5160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock`'s constructor: empty pointers, the terrain node,
/// cleared flags (`bHighPriority` is `!low_priority`), then a new
/// `BGSDistantObjectBlockLoadTask` (0x58 bytes) for the node's grid
/// position `x`, `y` stored in `spLoadTask` and started through its slot
/// +0x20, all inside a memory-context scope. Returns `this`.
// Translation incomplete: 006f5160 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_006f5160(
    e: &mut Engine,
    this: Ptr,
    node: Ptr,
    task_argument: u32,
    x: i16,
    y: i16,
    low_priority: u8,
) -> Ptr {
    let base = this.addr();
    for off in [
        BLOCK_MESH,
        BLOCK_SCENE_NODE,
        BLOCK_OLD_MESH,
        BLOCK_NODE_STINGER,
        BLOCK_NODE_STINGER_OLD,
    ] {
        e.call(NI_POINTER_CONSTRUCT, &args![base + off, 0u32]);
    }
    e.call(TASK_POINTER_CONSTRUCT, &args![base + BLOCK_LOAD_TASK, 0u32]);
    e.call(
        TASK_POINTER_CONSTRUCT,
        &args![base + BLOCK_LOAD_TASK_STINGER, 0u32],
    );
    e.with_stack(4, |e, scope| {
        e.call(
            SCOPE_BEGIN,
            &args![scope, 0x24u32, 1u32, SOURCE_FILE, 0x34u32],
        );
        e.mem.set_u32(base + BLOCK_NODE, node.addr());
        for off in [
            BLOCK_SCENE_NODE,
            BLOCK_MESH,
            BLOCK_OLD_MESH,
            BLOCK_NODE_STINGER_OLD,
            BLOCK_NODE_STINGER,
        ] {
            e.call(NI_POINTER_ASSIGN, &args![base + off, 0u32]);
        }
        e.mem.set_u8(base + BLOCK_DONE_LOADING, 0);
        e.mem.set_u8(base + BLOCK_ATTACHED, 0);
        e.mem.set_u8(base + BLOCK_PREPARED, 0);
        e.mem
            .set_u8(base + BLOCK_HIGH_PRIORITY, (low_priority == 0) as u8);
        e.mem.set_u8(base + BLOCK_DETACH_FLAG, 0);
        e.mem.set_u32(base + BLOCK_DETACH_COUNT, 0);

        let first = e.call(FIRST_WORD, &args![node]).u32();
        let node_value = e.call(FIRST_WORD, &args![first]).u32();
        let block = e.call(ALLOCATE, &args![0x58u32]).u32();
        let task = if block == 0 {
            0
        } else {
            e.call(
                LOAD_TASK_CONSTRUCT,
                &args![
                    block,
                    this,
                    node_value,
                    x as i32,
                    y as i32,
                    task_argument,
                    0u32,
                    (low_priority == 0) as u32,
                    0u32
                ],
            )
            .u32()
        };
        e.call(TASK_POINTER_ASSIGN, &args![base + BLOCK_LOAD_TASK, task]);
        e.mem.set_u32(base + BLOCK_DETACH_COUNT, 0);
        let task = e.call(FIRST_WORD, &args![base + BLOCK_LOAD_TASK]).u32();
        e.vcall(task, SLOT_TASK_QUEUE, &args![]);
        e.call(SCOPE_END, &args![scope]);
    });
    this
}

/// `this->pNode`'s owner object (`004ea9c0` on the node's first word)
/// detaches the scene node held by the pointer at `pointer`.
fn detach_from_node(e: &mut Engine, base: u32, pointer: u32) {
    let node = e.mem.u32(base + BLOCK_NODE);
    let node_value = e.call(FIRST_WORD, &args![node]).u32();
    let owner = e.call(NODE_OWNER, &args![node_value]).u32();
    let child = e.call(FIRST_WORD, &args![pointer]).u32();
    e.vcall(owner, SLOT_DETACH_CHILD, &args![child]);
}

fn decrement_attached_blocks(e: &mut Engine) {
    let count: u32 = e.global(ATTACHED_BLOCKS);
    e.set_global(ATTACHED_BLOCKS, count.wrapping_sub(1));
}

// Translated from 006f5370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDistantObjectBlock`'s destructor body: `006f60c0`; detaches the mesh
/// pair (scene node and mesh) when either is set and the stinger node when
/// set (each detachment decrements the counter at `011d8514`); cancels both
/// load tasks; releases the scene node and the stinger node (`00c45830`);
/// clears every pointer; purges unused textures when `005bb4d0` says so;
/// then destroys the pointers.
// Translation incomplete: 006f5370 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_006f5370(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(BLOCK_DETACH, &args![this]);
    let has_scene_node = e.call(FIRST_WORD, &args![base + BLOCK_SCENE_NODE]).u32() != 0;
    if has_scene_node || e.call(FIRST_WORD, &args![base + BLOCK_MESH]).u32() != 0 {
        decrement_attached_blocks(e);
        detach_from_node(e, base, base + BLOCK_SCENE_NODE);
        detach_from_node(e, base, base + BLOCK_MESH);
    }
    if e.call(FIRST_WORD, &args![base + BLOCK_NODE_STINGER]).u32() != 0 {
        decrement_attached_blocks(e);
        detach_from_node(e, base, base + BLOCK_NODE_STINGER);
    }
    for off in [BLOCK_LOAD_TASK, BLOCK_LOAD_TASK_STINGER] {
        if e.call(FIRST_WORD, &args![base + off]).u32() != 0 {
            let task = e.call(FIRST_WORD, &args![base + off]).u32();
            let manager: u32 = e.global(TASK_MANAGER);
            e.call(CANCEL_TASK, &args![manager, task, 0u32]);
        }
    }
    for off in [BLOCK_SCENE_NODE, BLOCK_NODE_STINGER] {
        let node = e.call(FIRST_WORD, &args![base + off]).u32();
        e.call(RELEASE_NODE, &args![node]);
    }
    e.call(TASK_POINTER_ASSIGN, &args![base + BLOCK_LOAD_TASK, 0u32]);
    for off in [BLOCK_MESH, BLOCK_OLD_MESH, BLOCK_SCENE_NODE] {
        e.call(NI_POINTER_ASSIGN, &args![base + off, 0u32]);
    }
    e.call(
        TASK_POINTER_ASSIGN,
        &args![base + BLOCK_LOAD_TASK_STINGER, 0u32],
    );
    for off in [BLOCK_NODE_STINGER_OLD, BLOCK_NODE_STINGER] {
        e.call(NI_POINTER_ASSIGN, &args![base + off, 0u32]);
    }
    let owner: u32 = e.global(PURGE_SETTING_OWNER);
    if e.call(SHOULD_PURGE, &args![owner]).bool() {
        e.call(PURGE_UNUSED_TEXTURES, &args![]);
    }
    e.call(TASK_POINTER_DESTROY, &args![base + BLOCK_LOAD_TASK_STINGER]);
    e.call(TASK_POINTER_DESTROY, &args![base + BLOCK_LOAD_TASK]);
    for off in [
        BLOCK_NODE_STINGER_OLD,
        BLOCK_NODE_STINGER,
        BLOCK_OLD_MESH,
        BLOCK_SCENE_NODE,
        BLOCK_MESH,
    ] {
        e.call(NI_POINTER_DESTROY, &args![base + off]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006f50b0, fn_006f50b0(Ptr<VirtualPathingNode>, Ptr)),
        entry!(0x006f5100, fn_006f5100(Ptr<VirtualPathingNode>, Ptr)),
        entry!(0x006f5160, fn_006f5160(Ptr, Ptr, u32, i16, i16, u8) -> Ptr),
        entry!(0x006f5370, fn_006f5370(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const LOCATION_VTABLE: u32 = 0x0002_0000;
    const LOCATION_SAVE: u32 = 0x0001_0000;
    const LOCATION_LOAD: u32 = 0x0001_0010;

    fn node_engine() -> (Engine, Ptr<VirtualPathingNode>) {
        let mut e = Engine::new();
        for addr in [
            SAVE_DATA,
            SAVE_FORM_ID,
            LOAD_DATA,
            LOCATION_SAVE,
            LOCATION_LOAD,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(LOAD_FORM_ID, |_, _| 0x1234u32.into_ret());
        e.register(FORM_FOR_ID, |_, a| (a[0] + 0x10_0000).into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        e.put_vtable(LOCATION_VTABLE, &[LOCATION_SAVE, LOCATION_LOAD]);
        let node: Ptr<VirtualPathingNode> = e.new_object();
        e.mem.set_u32(node.addr() + 4, LOCATION_VTABLE);
        (e, node)
    }

    #[test]
    fn save_writes_flags_door_and_location() {
        let (mut e, node) = node_engine();
        e.set(node, VirtualPathingNode::pDoorRef, Ptr::new(0x4444));
        e.call_log = Some(vec![]);
        e.call(0x006f_50b0, &args![node, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SAVE_DATA),
            vec![vec![0x9000, node.addr(), 4, 0]]
        );
        assert_eq!(calls(&log, SAVE_FORM_ID), vec![vec![0x9000, 0x4444, 0]]);
        assert_eq!(
            calls(&log, LOCATION_SAVE),
            vec![vec![node.addr() + 4, 0x9000]]
        );
    }

    #[test]
    fn load_reads_flags_casts_the_door_and_loads_the_location() {
        let (mut e, node) = node_engine();
        e.call_log = Some(vec![]);
        e.call(0x006f_5100, &args![node, 0x9000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LOAD_DATA), vec![vec![0x9000, node.addr(), 4]]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![0x10_1234, 0, TESFORM_TYPE, TESOBJECTREFR_TYPE, 0]]
        );
        assert_eq!(e.get(node, VirtualPathingNode::pDoorRef).addr(), 0x10_1235);
        assert_eq!(
            calls(&log, LOCATION_LOAD),
            vec![vec![node.addr() + 4, 0x9000]]
        );
    }

    /// Doubles for the block's callees: pointers are plain words
    /// (`00559450` reads, the assignments write).
    fn block_engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            NI_POINTER_CONSTRUCT,
            TASK_POINTER_CONSTRUCT,
            NI_POINTER_ASSIGN,
            TASK_POINTER_ASSIGN,
        ] {
            e.register(addr, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
        }
        e.register(FIRST_WORD, |e, a| e.mem.u32(a[0]).into_ret());
        for addr in [
            SCOPE_BEGIN,
            SCOPE_END,
            BLOCK_DETACH,
            CANCEL_TASK,
            RELEASE_NODE,
            PURGE_UNUSED_TEXTURES,
            TASK_POINTER_DESTROY,
            NI_POINTER_DESTROY,
            0x0001_0000,
            0x0001_0010,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.put_vtable(0x0002_0000, &[0; 9]);
        e.mem.set_u32(0x0002_0000 + SLOT_TASK_QUEUE, 0x0001_0000);
        e.put_vtable(0x0002_1000, &[0; 0x3c]);
        e.mem.set_u32(0x0002_1000 + SLOT_DETACH_CHILD, 0x0001_0010);
        e.register(LOAD_TASK_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0002_0000);
            a[0].into_ret()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_OWNER, |_, _| 0x0003_0000u32.into_ret());
        e.map(0x0003_0000, 4);
        e.mem.set_u32(0x0003_0000, 0x0002_1000);
        e.register(SHOULD_PURGE, |_, _| 1u32.into_ret());
        e.map(0x011d_8000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.set_global(TASK_MANAGER, 0x7000u32);
        e.set_global(PURGE_SETTING_OWNER, 0x7100u32);
        e
    }

    #[test]
    fn constructor_fills_the_block_and_starts_its_load_task() {
        let mut e = block_engine();
        let block: Ptr = Ptr::new(e.mem.alloc(0x2c));
        e.mem.write(block.addr(), &[0xaa; 0x2c]);
        // The terrain node's first word points at a word holding 0x5150.
        let inner = e.mem.alloc(4);
        e.mem.set_u32(inner, 0x5150);
        let node: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(node.addr(), inner);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x006f_5160, &args![block, node, 0x77u32, -3i16, 9i16, 0u8])
            .ptr::<()>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, block);
        assert_eq!(e.mem.u32(block.addr()), node.addr());
        let task = e.mem.u32(block.addr() + 0x18);
        assert_ne!(task, 0);
        let construct = &calls(&log, LOAD_TASK_CONSTRUCT)[0];
        assert_eq!(
            &construct[1..],
            &[block.addr(), 0x5150, -3i32 as u32, 9, 0x77, 0, 1, 0]
        );
        assert_eq!(e.mem.u8(block.addr() + 0x23), 1);
        for off in [0x20, 0x21, 0x22, 0x24] {
            assert_eq!(e.mem.u8(block.addr() + off), 0);
        }
        assert_eq!(e.mem.u32(block.addr() + 0x28), 0);
        assert_eq!(calls(&log, 0x0001_0000), vec![vec![task]]);
        let scope = calls(&log, SCOPE_BEGIN)[0].clone();
        assert_eq!(&scope[1..], &[0x24, 1, SOURCE_FILE, 0x34]);
        assert_eq!(calls(&log, SCOPE_END), vec![vec![scope[0]]]);
    }

    #[test]
    fn constructor_with_failed_allocation_stores_no_task() {
        let mut e = block_engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        // The game would call slot +0x20 on the null task; the pointer
        // double stores a stand-in task object so the call has a target.
        e.map(0x0002_2000, 4);
        e.mem.set_u32(0x0002_2000, 0x0002_0000);
        e.register(TASK_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], 0x0002_2000);
            Ret::default()
        });
        let block: Ptr = Ptr::new(e.mem.alloc(0x2c));
        let inner = e.mem.alloc(4);
        let node: Ptr = Ptr::new(e.mem.alloc(4));
        e.mem.set_u32(node.addr(), inner);
        e.call_log = Some(vec![]);
        e.call(0x006f_5160, &args![block, node, 0u32, 0i16, 0i16, 1u8]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, LOAD_TASK_CONSTRUCT).is_empty());
        assert_eq!(
            calls(&log, TASK_POINTER_ASSIGN),
            vec![vec![block.addr() + 0x18, 0]]
        );
        assert_eq!(e.mem.u8(block.addr() + 0x23), 0);
    }

    #[test]
    fn destructor_detaches_cancels_and_clears() {
        let mut e = block_engine();
        e.set_global(ATTACHED_BLOCKS, 5u32);
        let block: Ptr = Ptr::new(e.mem.alloc(0x2c));
        let node = e.mem.alloc(4);
        e.mem.set_u32(block.addr(), node);
        e.mem.set_u32(block.addr() + 0x04, 0x41);
        e.mem.set_u32(block.addr() + 0x08, 0x42);
        e.mem.set_u32(block.addr() + 0x10, 0x43);
        e.mem.set_u32(block.addr() + 0x18, 0x44);
        e.call_log = Some(vec![]);
        e.call(0x006f_5370, &args![block]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u32>(ATTACHED_BLOCKS), 3);
        assert_eq!(
            calls(&log, 0x0001_0010),
            vec![
                vec![0x0003_0000, 0x42],
                vec![0x0003_0000, 0x41],
                vec![0x0003_0000, 0x43]
            ]
        );
        assert_eq!(calls(&log, CANCEL_TASK), vec![vec![0x7000, 0x44, 0]]);
        assert_eq!(calls(&log, RELEASE_NODE), vec![vec![0x42], vec![0x43]]);
        for off in [4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c] {
            assert_eq!(e.mem.u32(block.addr() + off), 0);
        }
        assert_eq!(calls(&log, SHOULD_PURGE), vec![vec![0x7100]]);
        assert_eq!(calls(&log, PURGE_UNUSED_TEXTURES).len(), 1);
        assert_eq!(calls(&log, NI_POINTER_DESTROY).len(), 5);
        assert_eq!(
            calls(&log, TASK_POINTER_DESTROY),
            vec![vec![block.addr() + 0x1c], vec![block.addr() + 0x18]]
        );
    }

    #[test]
    fn destructor_without_meshes_detaches_nothing() {
        let mut e = block_engine();
        e.register(SHOULD_PURGE, |_, _| Ret::default());
        e.set_global(ATTACHED_BLOCKS, 5u32);
        let block: Ptr = Ptr::new(e.mem.alloc(0x2c));
        e.call_log = Some(vec![]);
        e.call(0x006f_5370, &args![block]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u32>(ATTACHED_BLOCKS), 5);
        assert!(calls(&log, 0x0001_0010).is_empty());
        assert!(calls(&log, CANCEL_TASK).is_empty());
        assert!(calls(&log, PURGE_UNUSED_TEXTURES).is_empty());
        assert_eq!(calls(&log, RELEASE_NODE), vec![vec![0], vec![0]]);
    }
}
