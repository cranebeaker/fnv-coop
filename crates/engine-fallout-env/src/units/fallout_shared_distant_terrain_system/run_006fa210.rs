//! `fallout shared/distant terrain system/run_006fa210` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructor and destructor of `BGSTerrainChunk`: the engine map has
//! no names for them, but the layout they work on is `BGSTerrainChunk` of
//! the Xbox PDB (0x30 bytes) and the constructor's scope guard names
//! `BGSTerrainChunk.cpp` with line 0x30. Field offsets are the PDB's, checked
//! against the code here: `+0` `pNode`, `+4` `spMesh`, `+8` `spWater`, `+0xc`
//! `spWaterAtReflectionHeight`, `+0x10` `spNode`, `+0x14` `spWaterNode`,
//! `+0x18` `spWaterReflNode`, `+0x1c`/`+0x20` the two `TESObjectREFR*`,
//! `+0x24` `spLoadTask`, `+0x28`..`+0x2a` the three flags, `+0x2c`
//! `spLightingProp`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPointer` dereference (`*this`).
const POINTER_GET: u32 = 0x0055_9450;
/// The `NiPointer(pointer)` constructor: stores the argument and retains it.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// The same for the smart pointer type at `+0x24`.
const LOAD_TASK_POINTER_CONSTRUCT: u32 = 0x0052_8cb0;
/// `NiPointer::operator=(pointer)`: stores the argument, releasing the old
/// and retaining the new object; returns `this`.
const POINTER_SET: u32 = 0x0066_b0d0;
/// `NiPointer::operator=(NiPointer const&)`: `this` is the target, the
/// argument the address of the source smart pointer; returns the target.
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// The `NiPointer<QueuedFile>::operator=(pointer)` of the smart pointer at
/// `+0x24` (Xbox PDB map name; releases the old, retains the new).
const LOAD_TASK_POINTER_SET: u32 = 0x006f_74f0;
/// The destructors of the two smart pointer types.
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
const LOAD_TASK_POINTER_DESTRUCT: u32 = 0x0044_cbf0;
/// The scope guard in the constructor (a 4-byte local): level `0x23`, flag
/// 1, the source-file string and the line `0x30`, then its destructor.
const GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// `"D:\\_Fallout3\\Platforms\\Common\\Code\\Fallout Shared\\Distant
/// Terrain System\\BGSTerrainChunk.cpp"`.
const SOURCE_FILE_STRING: u32 = 0x0106_e068;
/// The allocator (`MemoryManager::Allocate`).
const ALLOCATE: u32 = 0x0040_1000;
/// The constructor of the 0x60-byte object stored at `+0x24`:
/// `(object, chunk, world, first, second, extra)` (`bgsterrainchunk.cpp`).
const LOAD_TASK_CONSTRUCT: u32 = 0x006f_b980;
/// `bgsterrainchunk.cpp`, called with `(chunk, 0)` by the destructor.
const CHUNK_CALL_006FB3D0: u32 = 0x006f_b3d0;
/// `fn_006fa6a0` (`run_006fa6a0.rs`): removes a chunk from the list of
/// loaded chunks.
const CHUNK_LIST_REMOVE: u32 = 0x006f_a6a0;
/// `004ea9a0(manager)` returns the pointer in the global smart pointer at
/// `011d86a8`; the destructor calls its virtual at `+0xe8`.
const GLOBAL_OBJECT_GETTER: u32 = 0x004e_a9a0;
/// `BSTaskManager<__int64>::CancelTask` (Xbox PDB map name): `this` = the
/// global task manager, then the task and 0.
const TASK_MANAGER_CANCEL: u32 = 0x0044_ac40;
/// The global task manager pointer.
const TASK_MANAGER: u32 = 0x0120_2d98;
/// Releases the object (cdecl, one argument).
const OBJECT_RELEASE: u32 = 0x00c4_5830;
/// A word counted down by the destructor (`BGSTerrainChunk::iChunkCount` in
/// the Xbox PDB; the match is unconfirmed).
const CHUNK_COUNT: u32 = 0x011d_85ec;
/// `BSTexturePalette::PurgeUnusedTextures` (Xbox PDB map name), cdecl, no
/// argument; the destructor calls it when `005bb4d0` of the global at
/// `011dea0c` says so (the byte at `+6` of that object).
const PURGE_UNUSED_TEXTURES: u32 = 0x00a6_1cd0;
const PURGE_FLAG_GETTER: u32 = 0x005b_b4d0;
const PURGE_FLAG_OBJECT: u32 = 0x011d_ea0c;

// Translated from 006fa210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `BGSTerrainChunk` (identified by the layout and the
/// source-file string, not by a map name): zeroes the smart pointers,
/// stores `node` at `+0`, builds the 0x60-byte load-task object with
/// `006fb980(object, this, world, x, y, extra)` (the world is the first word
/// of the first word of `node`; `x` and `y` are sign-extended), stores it at
/// `+0x24`, calls its virtual at `+0x20`, clears the three flag bytes and
/// the water pointers and returns `this`.
///
/// When the allocation fails the task pointer is stored as null and the
/// virtual call then reads through null, as the game does.
// Translation incomplete: 006fa210 - SEH exception unwinding (FS:[0] frame) is unsupported.
pub fn fn_006fa210(e: &mut Engine, this: Ptr, node: Ptr, extra: u32, x: i16, y: i16) -> Ptr {
    let base = this.addr();
    for offset in [0x04, 0x08, 0x0c, 0x10, 0x14, 0x18] {
        e.call(POINTER_CONSTRUCT, &args![base + offset, 0u32]);
    }
    e.call(LOAD_TASK_POINTER_CONSTRUCT, &args![base + 0x24, 0u32]);
    e.call(POINTER_CONSTRUCT, &args![base + 0x2c, 0u32]);
    e.with_stack(4, |e, guard| {
        e.call(
            GUARD_CONSTRUCT,
            &args![guard, 0x23u32, 1u32, SOURCE_FILE_STRING, 0x30u32],
        );
        e.mem.set_u32(base, node.addr());
        let manager = e.call(POINTER_GET, &args![node]).u32();
        let world = e.call(POINTER_GET, &args![manager]).u32();
        let storage = e.call(ALLOCATE, &args![0x60u32]).u32();
        let task = if storage != 0 {
            e.call(
                LOAD_TASK_CONSTRUCT,
                &args![storage, this, world, x, y, extra],
            )
            .u32()
        } else {
            0
        };
        e.call(LOAD_TASK_POINTER_SET, &args![base + 0x24, task]);
        let task = e.call(POINTER_GET, &args![base + 0x24]).u32();
        e.vcall(task, 0x20, &[]);
        e.mem.set_u8(base + 0x28, 0);
        e.mem.set_u8(base + 0x29, 0);
        e.mem.set_u8(base + 0x2a, 0);
        let slot_0c = e.call(POINTER_SET, &args![base + 0xc, 0u32]).u32();
        let slot_08 = e.call(POINTER_ASSIGN, &args![base + 8, slot_0c]).u32();
        e.call(POINTER_ASSIGN, &args![base + 4, slot_08]);
        e.call(POINTER_SET, &args![base + 0x10, 0u32]);
        let slot_18 = e.call(POINTER_SET, &args![base + 0x18, 0u32]).u32();
        e.call(POINTER_ASSIGN, &args![base + 0x14, slot_18]);
        e.call(POINTER_SET, &args![base + 0x2c, 0u32]);
        e.mem.set_u32(base + 0x1c, 0);
        e.mem.set_u32(base + 0x20, 0);
        e.call(GUARD_DESTRUCT, &args![guard]);
    });
    this
}

// Translated from 006fa420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `BGSTerrainChunk` (identified by the layout, not by a
/// map name). With a mesh (`+4`): removes the chunk from the list of loaded
/// chunks (`006fa6a0`), decrements the word at `011d85ec`
/// and has the object `004ea9a0` returns run its virtual at `+0xe8` with the
/// `spNode` (or, when that is null, the mesh). Without a mesh and with a
/// load task (`+0x24`) it cancels the task in the global task manager. Then
/// it runs `006fb3d0(this, 0)`, releases the three node pointers with
/// `00c45830`, clears the smart pointers, purges unused textures when
/// `005bb4d0` of the object at `011dea0c` says so and destroys the members.
// Translation incomplete: 006fa420 - SEH exception unwinding (FS:[0] frame) is unsupported.
pub fn fn_006fa420(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.call(POINTER_GET, &args![base + 4]).u32() != 0 {
        e.call(CHUNK_LIST_REMOVE, &args![this]);
    }
    if e.call(POINTER_GET, &args![base + 4]).u32() != 0 {
        let count = e.global::<u32>(CHUNK_COUNT);
        e.set_global(CHUNK_COUNT, count.wrapping_sub(1));
        if e.call(POINTER_GET, &args![base + 0x10]).u32() != 0 {
            let node = e.mem.u32(base);
            let manager = e.call(POINTER_GET, &args![node]).u32();
            let object = e.call(GLOBAL_OBJECT_GETTER, &args![manager]).u32();
            let sp_node = e.call(POINTER_GET, &args![base + 0x10]).u32();
            e.vcall(object, 0xe8, &args![sp_node]);
        } else {
            let node = e.mem.u32(base);
            let manager = e.call(POINTER_GET, &args![node]).u32();
            let object = e.call(GLOBAL_OBJECT_GETTER, &args![manager]).u32();
            let mesh = e.call(POINTER_GET, &args![base + 4]).u32();
            e.vcall(object, 0xe8, &args![mesh]);
        }
    } else if e.call(POINTER_GET, &args![base + 0x24]).u32() != 0 {
        let task = e.call(POINTER_GET, &args![base + 0x24]).u32();
        let manager = e.global::<u32>(TASK_MANAGER);
        e.call(TASK_MANAGER_CANCEL, &args![manager, task, 0u32]);
    }
    e.call(CHUNK_CALL_006FB3D0, &args![this, 0u32]);
    for offset in [0x10, 0x14, 0x18] {
        let pointer = e.call(POINTER_GET, &args![base + offset]).u32();
        e.call(OBJECT_RELEASE, &args![pointer]);
    }
    e.call(LOAD_TASK_POINTER_SET, &args![base + 0x24, 0u32]);
    for offset in [0x04, 0x08, 0x10, 0x14, 0x18, 0x2c] {
        e.call(POINTER_SET, &args![base + offset, 0u32]);
    }
    let purge_object = e.global::<u32>(PURGE_FLAG_OBJECT);
    if e.call(PURGE_FLAG_GETTER, &args![purge_object]).u8() != 0 {
        e.call(PURGE_UNUSED_TEXTURES, &args![]);
    }
    e.call(POINTER_DESTRUCT, &args![base + 0x2c]);
    e.call(LOAD_TASK_POINTER_DESTRUCT, &args![base + 0x24]);
    for offset in [0x18, 0x14, 0x10, 0x0c, 0x08, 0x04] {
        e.call(POINTER_DESTRUCT, &args![base + offset]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006fa210, fn_006fa210(Ptr, Ptr, u32, i16, i16) -> Ptr),
        entry!(0x006fa420, fn_006fa420(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The vtable of the objects the tests call through, with the virtual at
    /// byte offset `slot` set to `target`.
    fn put_slot(e: &mut Engine, vtable: u32, slot: u32, target: u32) {
        let mut slots = vec![0u32; (slot / 4 + 1) as usize];
        slots[(slot / 4) as usize] = target;
        e.put_vtable(vtable, &slots);
    }

    /// Doubles for the smart-pointer functions, which store and read plain
    /// words, and for the helpers outside this file.
    fn chunk_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011d_8000, 0x011d_e000, 0x0120_2000] {
            e.map(page, 0x1000);
        }
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        for constructor in [
            POINTER_CONSTRUCT,
            LOAD_TASK_POINTER_CONSTRUCT,
            POINTER_SET,
            LOAD_TASK_POINTER_SET,
        ] {
            e.register(constructor, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                ret(a[0])
            });
        }
        e.register(POINTER_ASSIGN, |e, a| {
            let source = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], source);
            ret(a[0])
        });
        e.register(GUARD_CONSTRUCT, |_, _| Ret::default());
        e.register(GUARD_DESTRUCT, |_, _| Ret::default());
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(LOAD_TASK_POINTER_DESTRUCT, |_, _| Ret::default());
        e
    }

    /// A chunk-sized block filled with a recognizable non-zero word.
    fn dirty_chunk(e: &mut Engine) -> Ptr {
        let chunk = e.mem.alloc(0x30);
        for offset in (0..0x30).step_by(4) {
            e.mem.set_u32(chunk + offset, 0xdddd_dddd);
        }
        Ptr::new(chunk)
    }

    #[test]
    fn constructor_zeroes_the_members_and_starts_the_load_task() {
        const WORLD: u32 = 0x0abc_0000;
        let mut e = chunk_engine();
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        // The task object gets a vtable whose virtual at +0x20 is recorded.
        put_slot(&mut e, 0x0300_0000, 0x20, 0x0300_0020);
        e.register(0x0300_0020, |_, _| Ret::default());
        e.register(LOAD_TASK_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0300_0000);
            ret(a[0])
        });
        // node -> manager -> world
        let manager = e.mem.alloc(8);
        e.mem.set_u32(manager, WORLD);
        let node = Ptr::<()>::new(e.mem.alloc(0x60));
        e.mem.set_u32(node.addr(), manager);
        let chunk = dirty_chunk(&mut e);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x006f_a210, &args![chunk, node, 0x55u32, -3i16, 7i16])
            .ptr::<()>();
        assert_eq!(back, chunk);
        let base = chunk.addr();
        assert_eq!(e.mem.u32(base), node.addr());
        for offset in [0x04, 0x08, 0x0c, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x2c] {
            assert_eq!(e.mem.u32(base + offset), 0, "offset {offset:#x}");
        }
        for offset in [0x28, 0x29, 0x2a] {
            assert_eq!(e.mem.u8(base + offset), 0, "flag {offset:#x}");
        }
        let construct = calls_to(&e, LOAD_TASK_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        let task = construct[0][0];
        // (object, chunk, world, x sign-extended, y, extra)
        assert_eq!(&construct[0][1..], &[base, WORLD, 0xffff_fffd, 7, 0x55]);
        assert_eq!(e.mem.u32(base + 0x24), task);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x60]]);
        assert_eq!(calls_to(&e, 0x0300_0020), vec![vec![task]]);
        // The guard is built with the level, the flag, the file string and the line.
        let guard = calls_to(&e, GUARD_CONSTRUCT);
        assert_eq!(&guard[0][1..], &[0x23, 1, SOURCE_FILE_STRING, 0x30]);
        assert_eq!(calls_to(&e, GUARD_DESTRUCT), vec![vec![guard[0][0]]]);
    }

    #[test]
    #[should_panic]
    fn constructor_reads_through_null_when_the_allocation_fails() {
        let mut e = chunk_engine();
        e.register(ALLOCATE, |_, _| ret(0));
        let manager = e.mem.alloc(8);
        let node = Ptr::<()>::new(e.mem.alloc(0x60));
        e.mem.set_u32(node.addr(), manager);
        let chunk = dirty_chunk(&mut e);
        // The load task is stored as null and its virtual is read through
        // null, which the game cannot do either.
        e.call(0x006f_a210, &args![chunk, node, 0u32, 0i16, 0i16]);
    }

    /// A chunk with the mesh, the three node pointers and a load task as
    /// given (0 for none), whose node leads to a manager and `object`.
    fn chunk_with(e: &mut Engine, mesh: u32, nodes: [u32; 3], task: u32) -> Ptr {
        let chunk = dirty_chunk(e);
        let manager = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, manager);
        e.mem.set_u32(chunk.addr(), node);
        e.mem.set_u32(chunk.addr() + 4, mesh);
        e.mem.set_u32(chunk.addr() + 0x10, nodes[0]);
        e.mem.set_u32(chunk.addr() + 0x14, nodes[1]);
        e.mem.set_u32(chunk.addr() + 0x18, nodes[2]);
        e.mem.set_u32(chunk.addr() + 0x24, task);
        chunk
    }

    /// Doubles for the destructor's callees outside the file; the object
    /// `004ea9a0` returns records the argument of its virtual at `+0xe8`.
    fn destructor_engine() -> (Engine, u32) {
        let mut e = chunk_engine();
        put_slot(&mut e, 0x0300_0000, 0xe8, 0x0300_00e8);
        e.register(0x0300_00e8, |e, a| {
            e.mem.set_u32(0x011d_8f00, a[1]);
            Ret::default()
        });
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0300_0000);
        e.register_double(GLOBAL_OBJECT_GETTER, move |_, _| ret(object));
        e.register(CHUNK_LIST_REMOVE, |_, _| Ret::default());
        e.register(CHUNK_CALL_006FB3D0, |_, _| Ret::default());
        e.register(OBJECT_RELEASE, |_, _| Ret::default());
        e.register(TASK_MANAGER_CANCEL, |_, _| Ret::default());
        e.register(PURGE_UNUSED_TEXTURES, |_, _| Ret::default());
        e.register(PURGE_FLAG_GETTER, |_, a| ret(a[0] & 0xff));
        e.set_global(CHUNK_COUNT, 5u32);
        (e, object)
    }

    #[test]
    fn destructor_with_a_mesh_and_a_node_runs_the_whole_sequence() {
        let (mut e, object) = destructor_engine();
        let chunk = chunk_with(&mut e, 0x1111, [0x2222, 0x3333, 0x4444], 0x5555);
        e.mem.set_u32(PURGE_FLAG_OBJECT, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_a420, &args![chunk]);
        let base = chunk.addr();
        // The count went down; the hook got the node pointer, not the mesh.
        assert_eq!(e.global::<u32>(CHUNK_COUNT), 4);
        assert_eq!(e.mem.u32(0x011d_8f00), 0x2222);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| *a != POINTER_GET)
            .collect();
        let mut expected = vec![
            0x006f_a420,
            CHUNK_LIST_REMOVE,
            GLOBAL_OBJECT_GETTER,
            0x0300_00e8,
            CHUNK_CALL_006FB3D0,
            OBJECT_RELEASE,
            OBJECT_RELEASE,
            OBJECT_RELEASE,
            LOAD_TASK_POINTER_SET,
        ];
        expected.extend([POINTER_SET; 6]);
        expected.push(PURGE_FLAG_GETTER);
        expected.extend([POINTER_DESTRUCT, LOAD_TASK_POINTER_DESTRUCT]);
        expected.extend([POINTER_DESTRUCT; 6]);
        assert_eq!(order, expected);
        assert_eq!(calls_to(&e, CHUNK_LIST_REMOVE), vec![vec![base]]);
        assert_eq!(calls_to(&e, CHUNK_CALL_006FB3D0), vec![vec![base, 0]]);
        assert_eq!(
            calls_to(&e, OBJECT_RELEASE),
            vec![vec![0x2222], vec![0x3333], vec![0x4444]]
        );
        assert_eq!(calls_to(&e, 0x0300_00e8), vec![vec![object, 0x2222]]);
        // The load task is released, not cancelled, once a mesh exists.
        assert!(calls_to(&e, TASK_MANAGER_CANCEL).is_empty());
        assert_eq!(
            calls_to(&e, POINTER_SET),
            [4, 8, 0x10, 0x14, 0x18, 0x2c]
                .iter()
                .map(|offset| vec![base + offset, 0])
                .collect::<Vec<_>>()
        );
        assert_eq!(
            calls_to(&e, POINTER_DESTRUCT),
            [0x2c, 0x18, 0x14, 0x10, 0x0c, 0x08, 0x04]
                .iter()
                .map(|offset| vec![base + offset])
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn destructor_without_a_node_pointer_hands_the_mesh_to_the_hook() {
        let (mut e, _) = destructor_engine();
        let chunk = chunk_with(&mut e, 0x1111, [0, 0x3333, 0], 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_a420, &args![chunk]);
        assert_eq!(e.mem.u32(0x011d_8f00), 0x1111);
        assert_eq!(e.global::<u32>(CHUNK_COUNT), 4);
        // OBJECT_RELEASE is called for the null pointers too.
        assert_eq!(
            calls_to(&e, OBJECT_RELEASE),
            vec![vec![0], vec![0x3333], vec![0]]
        );
    }

    #[test]
    fn destructor_purges_textures_when_the_flag_getter_says_so() {
        let (mut e, _) = destructor_engine();
        let chunk = chunk_with(&mut e, 0, [0, 0, 0], 0);
        e.mem.set_u32(PURGE_FLAG_OBJECT, 1);
        e.call_log = Some(vec![]);
        e.call(0x006f_a420, &args![chunk]);
        assert_eq!(calls_to(&e, PURGE_FLAG_GETTER), vec![vec![1]]);
        assert_eq!(calls_to(&e, PURGE_UNUSED_TEXTURES), vec![Vec::<u32>::new()]);
    }

    #[test]
    fn destructor_without_a_mesh_cancels_the_load_task() {
        let (mut e, _) = destructor_engine();
        let chunk = chunk_with(&mut e, 0, [0, 0, 0], 0x5555);
        e.set_global(TASK_MANAGER, 0x6666u32);
        e.call_log = Some(vec![]);
        e.call(0x006f_a420, &args![chunk]);
        assert_eq!(
            calls_to(&e, TASK_MANAGER_CANCEL),
            vec![vec![0x6666, 0x5555, 0]]
        );
        assert!(calls_to(&e, CHUNK_LIST_REMOVE).is_empty());
        assert!(calls_to(&e, GLOBAL_OBJECT_GETTER).is_empty());
        // The counter is left alone.
        assert_eq!(e.global::<u32>(CHUNK_COUNT), 5);
        assert_eq!(calls_to(&e, CHUNK_CALL_006FB3D0).len(), 1);
    }

    #[test]
    fn destructor_without_a_mesh_or_a_task_cancels_nothing() {
        let (mut e, _) = destructor_engine();
        let chunk = chunk_with(&mut e, 0, [0, 0, 0], 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_a420, &args![chunk]);
        assert!(calls_to(&e, TASK_MANAGER_CANCEL).is_empty());
        assert_eq!(e.global::<u32>(CHUNK_COUNT), 5);
        assert_eq!(calls_to(&e, POINTER_DESTRUCT).len(), 7);
    }

    #[test]
    fn destructor_wraps_the_counter_like_the_exe() {
        let (mut e, _) = destructor_engine();
        e.set_global(CHUNK_COUNT, 0u32);
        let chunk = chunk_with(&mut e, 0x1111, [0, 0, 0], 0);
        e.call(0x006f_a420, &args![chunk]);
        assert_eq!(e.global::<u32>(CHUNK_COUNT), u32::MAX);
    }
}
