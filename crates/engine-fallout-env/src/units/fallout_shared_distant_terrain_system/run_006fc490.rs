//! `fallout shared/distant terrain system/run_006fc490` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructor and destructor of the distant-terrain manager (the
//! engine map has no names for them; the layout of the object they work on
//! is `BGSTerrainManager` of `bgsterrainmanager.rs`, and the source-file
//! string passed to the scope guard is `BGSTerrainManager.cpp`), two
//! helpers that read a number of the settings file into a field, and the
//! root node's deleting destructor.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bgsterrainmanager::BGSTerrainManager;

/// `NiPointer` dereference (`*this`).
const POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer::operator=(pointer)` for a smart pointer at `this`: stores the
/// argument, releasing the old and retaining the new object; returns `this`.
const POINTER_SET: u32 = 0x0066_b0d0;
/// The `NiPointer(pointer)` constructor: stores the argument and retains it.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer::~NiPointer`: releases the object.
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `BSSimpleArray<T,1024>` constructor / destructor.
const ARRAY_CONSTRUCT: u32 = 0x005e_04c0;
const ARRAY_DESTRUCT: u32 = 0x005e_04f0;
/// The scope guard constructed in the constructor (a 4-byte local): level
/// `0x22`, flag 1, the source-file string and a line (`0x2f`), then its
/// destructor.
const GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// `"D:\\_Fallout3\\Platforms\\Common\\Code\\Fallout Shared\\Distant
/// Terrain System\\BGSTerrainManager.cpp"`.
const SOURCE_FILE_STRING: u32 = 0x0106_e338;
/// `"Data\\LODSettings\\%s.DLODSettings"`.
const SETTINGS_PATH_FORMAT: u32 = 0x0106_e310;
/// The CRT's `sprintf`, cdecl.
const SPRINTF: u32 = 0x00ec_623a;
/// `FileFinder::Exist(path, 0, 6, 0x100)` (Xbox PDB map name), cdecl.
const FILE_EXISTS: u32 = 0x0045_6a20;
/// `FileFinder::GetFile(path, 0, 100)` (Xbox PDB map name), cdecl; returns
/// the opened file object (or 0).
const FILE_OPEN: u32 = 0x00af_df00;
/// The file object's reader: `this` = the file, then the address to read
/// to and the byte count; returns the value read.
const FILE_READ: u32 = 0x0046_2d80;
/// The allocator (`MemoryManager::Allocate`).
const ALLOCATE: u32 = 0x0040_1000;
/// The allocator's matching free.
const FREE: u32 = 0x0040_1030;
/// `BGSTerrainNode::BGSTerrainNode(manager, parent, x, y, level, flag)` of
/// `bgsterrainnode.cpp` (Xbox PDB map name), 0x60 bytes.
const NODE_CONSTRUCT: u32 = 0x006f_d210;
/// The root node's destructor body (`bgsterrainnode.cpp`), called by
/// [`fn_006fca40`].
const NODE_DESTRUCT: u32 = 0x006f_d750;
/// A function of `bgsterrainnode.cpp` that [`fn_006fca70`] calls with the
/// word at `+4` of its `this`.
const NODE_CALL_006FDA10: u32 = 0x006f_da10;
/// The allocation of a `NiNode` (size `0xac`, cdecl) and its constructor
/// (argument 0).
const NODE_ALLOCATE: u32 = 0x00aa_13e0;
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// A global object whose `+0x10` (read by `0044edb0`) and `+0x14` (read by
/// `00825c00`) fields the constructor stores into the manager's smart
/// pointers.
const GLOBAL_OBJECT_011DEA10: u32 = 0x011d_ea10;
const OBJECT_FIELD_10: u32 = 0x0044_edb0;
const OBJECT_FIELD_14: u32 = 0x0082_5c00;
/// `009611e0(this)`: the word at `+0x18`.
const OBJECT_FIELD_18: u32 = 0x0096_11e0;
/// `00546780(this, 1)` of `tesobjectcell.cpp`, called on the two objects.
const OBJECT_CALL_00546780: u32 = 0x0054_6780;
/// Two global smart pointers (`011d86a8`, `011d8690`) the constructor fills
/// with new `NiNode`s when the first is empty.
const SMART_POINTER_011D86A8: u32 = 0x011d_86a8;
const SMART_POINTER_011D8690: u32 = 0x011d_8690;

// Translated from 006fc490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the distant-terrain manager, as its layout and source
/// file say (no name in the engine map). Builds the manager for `world`:
/// reads `Data\LODSettings\<world's name>.DLODSettings` (the world's virtual
/// at `+0x130` gives the name), the cell rectangle and the levels from it,
/// and, when the file was opened, creates the root node (`0x60` bytes,
/// `006fd210`); stores the global object's `+0x10` and `+0x14` fields in the
/// two smart pointers at `+8` and `+0xc` (running `00546780(…, 1)` on the
/// first and on what `009611e0` finds in it), and, if the global smart
/// pointer at `011d86a8` is empty, creates two `NiNode`s into it and
/// `011d8690` and calls the first one's virtual at `+0xdc` with each.
/// Returns `this`.
///
/// The file is deleted through its virtual destructor once read.
// Translation incomplete: 006fc490 - SEH exception unwinding (FS:[0] frame) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_006fc490(
    e: &mut Engine,
    this: Ptr<BGSTerrainManager>,
    world: Ptr,
) -> Ptr<BGSTerrainManager> {
    let base = this.addr();
    e.call(POINTER_CONSTRUCT, &args![base + 8, 0u32]);
    e.call(POINTER_CONSTRUCT, &args![base + 0xc, 0u32]);
    e.call(ARRAY_CONSTRUCT, &args![base + 0x2c]);
    e.with_stack(4, |e, guard| {
        e.call(
            GUARD_CONSTRUCT,
            &args![guard, 0x22u32, 1u32, SOURCE_FILE_STRING, 0x2fu32],
        );
        e.set(this, BGSTerrainManager::bHasLOD, false);
        e.set(this, BGSTerrainManager::pWorld, world);
        let world_name = e.vcall(world.addr(), 0x130, &[]).u32();
        let file = e.with_stack(0x108, |e, path| {
            e.call(SPRINTF, &args![path, SETTINGS_PATH_FORMAT, world_name]);
            let mut file = 0;
            if e.call(FILE_EXISTS, &args![path, 0u32, 6u32, 0x100u32])
                .u32()
                != 0
            {
                file = e.call(FILE_OPEN, &args![path, 0u32, 100u32]).u32();
            }
            file
        });
        if file == 0 {
            e.set(this, BGSTerrainManager::pRootNode, Ptr::NULL);
        } else {
            let file = Ptr::<()>::new(file);
            let file = fn_006fc950(e, file, Ptr::new(base + 0x1c));
            let file = fn_006fc950(e, file, Ptr::new(base + 0x18));
            let file = fn_006fc950(e, file, Ptr::new(base + 0x20));
            let file = fn_006fc920(e, file, Ptr::new(base + 0x10));
            let file = fn_006fc920(e, file, Ptr::new(base + 0x12));
            let file = fn_006fc920(e, file, Ptr::new(base + 0x14));
            let file = fn_006fc920(e, file, Ptr::new(base + 0x16));
            let file = fn_006fc950(e, file, Ptr::new(base + 0x24));
            // The file object goes away through its virtual destructor.
            e.vcall(file.addr(), 0, &args![1u32]);
            let storage = e.call(ALLOCATE, &args![0x60u32]).u32();
            let root = if storage != 0 {
                let min_x = u32::from(e.get(this, BGSTerrainManager::sMinCellX) as u16);
                let min_y = u32::from(e.get(this, BGSTerrainManager::sMinCellY) as u16);
                let root_level = e.get(this, BGSTerrainManager::iRootLevel);
                e.call(
                    NODE_CONSTRUCT,
                    &args![storage, this, 0u32, min_x, min_y, root_level, 0u32],
                )
                .u32()
            } else {
                0
            };
            e.set(this, BGSTerrainManager::pRootNode, Ptr::new(root));
            e.set(this, BGSTerrainManager::bHasLOD, true);
        }
        let global_object = e.global::<u32>(GLOBAL_OBJECT_011DEA10);
        let field_10 = e.call(OBJECT_FIELD_10, &args![global_object]).u32();
        e.call(POINTER_SET, &args![base + 8, field_10]);
        if e.call(POINTER_GET, &args![base + 8]).u32() != 0 {
            let global_object = e.global::<u32>(GLOBAL_OBJECT_011DEA10);
            let field_14 = e.call(OBJECT_FIELD_14, &args![global_object]).u32();
            e.call(POINTER_SET, &args![base + 0xc, field_14]);
            let first = e.call(POINTER_GET, &args![base + 8]).u32();
            e.call(OBJECT_CALL_00546780, &args![first, 1u32]);
            let again = e.call(POINTER_GET, &args![base + 8]).u32();
            let second = e.call(OBJECT_FIELD_18, &args![again]).u32();
            e.call(OBJECT_CALL_00546780, &args![second, 1u32]);
        }
        e.set(this, BGSTerrainManager::bNeedsImmediateUpdate, false);
        if e.call(POINTER_GET, &args![SMART_POINTER_011D86A8]).u32() == 0
            && e.call(POINTER_GET, &args![base + 8]).u32() != 0
        {
            attach_new_node(e, SMART_POINTER_011D86A8, base);
            attach_new_node(e, SMART_POINTER_011D8690, base);
        }
        e.call(GUARD_DESTRUCT, &args![guard]);
    });
    this
}

/// Creates a `NiNode` (`0xac` bytes), stores it in the smart pointer at
/// `slot` and calls the virtual at `+0xdc` of the manager's smart pointer
/// at `+8` with the pointer now in `slot` and 1.
fn attach_new_node(e: &mut Engine, slot: u32, manager: u32) {
    let storage = e.call(NODE_ALLOCATE, &args![0xacu32]).u32();
    let node = if storage != 0 {
        e.call(NI_NODE_CONSTRUCT, &args![storage, 0u32]).u32()
    } else {
        0
    };
    e.call(POINTER_SET, &args![slot, node]);
    let parent = e.call(POINTER_GET, &args![manager + 8]).u32();
    let child = e.call(POINTER_GET, &args![slot]).u32();
    e.vcall(parent, 0xdc, &args![child, 1u32]);
}

// Translated from 006fc920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads 2 bytes from the file object `this` into `out` (through the file's
/// reader `00462d80`), stores what the reader returns in the file object's
/// field at `+0x148` and returns `this`.
pub fn fn_006fc920(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let read = e.call(FILE_READ, &args![this, out, 2u32]).u32();
    e.mem.set_u32(this.addr() + 0x148, read);
    this
}

// Translated from 006fc950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads 4 bytes from the file object `this` into `out`, stores what the
/// reader returns in the file object's field at `+0x148` and returns `this`.
pub fn fn_006fc950(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let read = e.call(FILE_READ, &args![this, out, 4u32]).u32();
    e.mem.set_u32(this.addr() + 0x148, read);
    this
}

// Translated from 006fc980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the distant-terrain manager (no name in the engine map;
/// the layout is that of `BGSTerrainManager`): deletes the root node through
/// its deleting destructor, clears the two smart pointers at `+8` and
/// `+0xc`, destroys the array at `+0x2c`, then the two smart pointers.
// Translation incomplete: 006fc980 - SEH exception unwinding (FS:[0] frame) is unsupported.
pub fn fn_006fc980(e: &mut Engine, this: Ptr<BGSTerrainManager>) {
    let root = e.get(this, BGSTerrainManager::pRootNode);
    if !root.is_null() {
        fn_006fca40(e, root, 1);
    }
    let base = this.addr();
    e.call(POINTER_SET, &args![base + 8, 0u32]);
    e.call(POINTER_SET, &args![base + 0xc, 0u32]);
    e.call(ARRAY_DESTRUCT, &args![base + 0x2c]);
    e.call(POINTER_DESTRUCT, &args![base + 0xc]);
    e.call(POINTER_DESTRUCT, &args![base + 8]);
}

// Translated from 006fca40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the root node (`BGSTerrainNode`, 0x60
/// bytes): runs the node's destructor body `006fd750` and, when bit 0 of
/// `flags` is set, frees the memory. Returns `this`.
pub fn fn_006fca40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(NODE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 006fca70 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the word at `this + 4` is set, calls `006fda10` on it.
pub fn fn_006fca70(e: &mut Engine, this: Ptr) {
    let pointer = e.mem.u32(this.addr() + 4);
    if pointer != 0 {
        e.call(NODE_CALL_006FDA10, &args![pointer]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006fc490,
            fn_006fc490(Ptr<BGSTerrainManager>, Ptr) -> Ptr<BGSTerrainManager>
        ),
        entry!(0x006fc920, fn_006fc920(Ptr, Ptr) -> Ptr),
        entry!(0x006fc950, fn_006fc950(Ptr, Ptr) -> Ptr),
        entry!(0x006fc980, fn_006fc980(Ptr<BGSTerrainManager>)),
        entry!(0x006fca40, fn_006fca40(Ptr, u32) -> Ptr),
        entry!(0x006fca70, fn_006fca70(Ptr)),
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

    /// An object with a vtable (mapped at `vtable`) whose slot at byte
    /// offset `slot` is the function `target`.
    fn object_with_slot(e: &mut Engine, vtable: u32, slot: u32, target: u32) -> u32 {
        let mut slots = vec![0u32; (slot / 4 + 1) as usize];
        slots[(slot / 4) as usize] = target;
        e.put_vtable(vtable, &slots);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Doubles for everything the constructor and destructor call outside
    /// this file; the smart-pointer functions store and read plain words.
    fn manager_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011d_8000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(ARRAY_CONSTRUCT, |_, _| Ret::default());
        e.register(ARRAY_DESTRUCT, |_, _| Ret::default());
        e.register(GUARD_CONSTRUCT, |_, _| Ret::default());
        e.register(GUARD_DESTRUCT, |_, _| Ret::default());
        e.register(SPRINTF, |_, _| Ret::default());
        e.register(OBJECT_FIELD_10, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(OBJECT_FIELD_14, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(OBJECT_FIELD_18, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(OBJECT_CALL_00546780, |_, _| Ret::default());
        e.register(NODE_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NI_NODE_CONSTRUCT, |_, a| ret(a[0]));
        e
    }

    /// A world space whose virtual at `+0x130` returns a name string.
    fn world(e: &mut Engine) -> (Ptr, u32) {
        let name = e.mem.alloc(8);
        e.register(0x0300_0130, |e, a| ret(e.mem.u32(a[0] + 4)));
        let world = object_with_slot(e, 0x0300_0000, 0x130, 0x0300_0130);
        e.mem.set_u32(world + 4, name);
        (Ptr::new(world), name)
    }

    /// Makes the file read double return the successive `values` and write
    /// them (as 2 or 4 bytes) to the output address; records the sizes.
    fn script_reads(e: &mut Engine, values: Vec<u32>) {
        let mut next = 0usize;
        e.register_double(FILE_READ, move |e, a| {
            let value = values[next];
            next += 1;
            if a[2] == 2 {
                e.mem.set_u16(a[1], value as u16);
            } else {
                e.mem.set_u32(a[1], value);
            }
            ret(value)
        });
    }

    /// Prepares the settings file: it exists, opens as a file object whose
    /// destructor (virtual 0) is the double that records its argument.
    fn settings_file(e: &mut Engine) -> u32 {
        e.register(FILE_EXISTS, |_, _| ret(1));
        e.register(0x0300_1000, |e, a| {
            e.mem.set_u32(0x011d_8f00, a[0]);
            e.mem.set_u32(0x011d_8f04, a[1]);
            Ret::default()
        });
        e.put_vtable(0x0300_2000, &[0x0300_1000]);
        let file = e.mem.alloc(0x150);
        e.mem.set_u32(file, 0x0300_2000);
        e.register_double(FILE_OPEN, move |_, _| ret(file));
        file
    }

    #[test]
    fn constructor_reads_the_settings_file_and_builds_the_root_node() {
        let mut e = manager_engine();
        let (world, name) = world(&mut e);
        let file = settings_file(&mut e);
        // Read order: +0x1c, +0x18, +0x20 (4 bytes), +0x10..+0x16 (2 bytes), +0x24.
        script_reads(&mut e, vec![3, 9, 7, 0xfffe, 5, 6, 8, 4]);
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NODE_CONSTRUCT, |_, a| ret(a[0]));
        // The global object's +0x10 and +0x14 fields; no root exists yet.
        let object = e.mem.alloc(0x20);
        let root_node = e.mem.alloc(0x20);
        let water_node = e.mem.alloc(0x20);
        let mut slots = vec![0u32; 0x38];
        slots[0xdc / 4] = 0x0300_3000;
        e.put_vtable(0x0300_4000, &slots);
        e.mem.set_u32(root_node, 0x0300_4000);
        e.mem.set_u32(object + 0x10, root_node);
        e.mem.set_u32(object + 0x14, water_node);
        e.mem.set_u32(root_node + 0x18, 0x7777);
        e.set_global(GLOBAL_OBJECT_011DEA10, object);
        e.register(0x0300_3000, |_, _| Ret::default());
        let manager: Ptr<BGSTerrainManager> = e.new_object();
        e.set(manager, BGSTerrainManager::bNeedsImmediateUpdate, true);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x006f_c490, &args![manager, world])
            .ptr::<BGSTerrainManager>();
        assert_eq!(back, manager);
        assert_eq!(e.get(manager, BGSTerrainManager::pWorld), world);
        assert_eq!(e.get(manager, BGSTerrainManager::iMinLevel), 3);
        assert_eq!(e.get(manager, BGSTerrainManager::iMaxLevel), 9);
        assert_eq!(e.get(manager, BGSTerrainManager::iRootLevel), 7);
        assert_eq!(e.get(manager, BGSTerrainManager::sMinCellX), -2);
        assert_eq!(e.get(manager, BGSTerrainManager::sMinCellY), 5);
        assert_eq!(e.get(manager, BGSTerrainManager::sMaxCellX), 6);
        assert_eq!(e.get(manager, BGSTerrainManager::sMaxCellY), 8);
        assert_eq!(e.get(manager, BGSTerrainManager::iBlockLevel), 4);
        // The file's reader stored its last result in the file object.
        assert_eq!(e.mem.u32(file + 0x148), 4);
        // The path comes from the format and the world's name.
        let sprintf = calls_to(&e, SPRINTF);
        assert_eq!(sprintf.len(), 1);
        assert_eq!(&sprintf[0][1..], &[SETTINGS_PATH_FORMAT, name]);
        let path = sprintf[0][0];
        assert_eq!(calls_to(&e, FILE_EXISTS), vec![vec![path, 0, 6, 0x100]]);
        assert_eq!(calls_to(&e, FILE_OPEN), vec![vec![path, 0, 100]]);
        // The file object is destroyed through its virtual with 1.
        assert_eq!(e.mem.u32(0x011d_8f00), file);
        assert_eq!(e.mem.u32(0x011d_8f04), 1);
        // The root node: storage of 0x60 bytes, the manager, 0, the minimum
        // cell as unsigned words, the root level, 0.
        let root_call = calls_to(&e, NODE_CONSTRUCT);
        assert_eq!(root_call.len(), 1);
        assert_eq!(&root_call[0][1..], &[manager.addr(), 0, 0xfffe, 5, 7, 0]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x60]]);
        assert_eq!(
            e.get(manager, BGSTerrainManager::pRootNode).addr(),
            root_call[0][0]
        );
        assert!(e.get(manager, BGSTerrainManager::bHasLOD));
        assert!(!e.get(manager, BGSTerrainManager::bNeedsImmediateUpdate));
        // The smart pointers took the global object's fields and 00546780(.., 1)
        // ran on the first and on what 009611e0 reads from it.
        assert_eq!(
            e.get(manager, BGSTerrainManager::spLODRoot).addr(),
            root_node
        );
        assert_eq!(
            e.get(manager, BGSTerrainManager::spLODWaterRoot).addr(),
            water_node
        );
        assert_eq!(
            calls_to(&e, OBJECT_CALL_00546780),
            vec![vec![root_node, 1], vec![0x7777, 1]]
        );
        // The global smart pointer at 011d86a8 was empty: two nodes of 0xac
        // bytes were created and attached to the first smart pointer's object.
        let first = e.mem.u32(SMART_POINTER_011D86A8);
        let second = e.mem.u32(SMART_POINTER_011D8690);
        assert!(first != 0 && second != 0 && first != second);
        assert_eq!(calls_to(&e, NODE_ALLOCATE), vec![vec![0xac], vec![0xac]]);
        assert_eq!(calls_to(&e, NI_NODE_CONSTRUCT).len(), 2);
        assert_eq!(
            calls_to(&e, 0x0300_3000),
            vec![vec![root_node, first, 1], vec![root_node, second, 1]]
        );
    }

    #[test]
    fn constructor_without_a_settings_file_has_no_root_node() {
        let mut e = manager_engine();
        let (world, _) = world(&mut e);
        e.register(FILE_EXISTS, |_, _| ret(0));
        e.register(FILE_OPEN, |_, _| ret(0xbad));
        let object = e.mem.alloc(0x20);
        e.set_global(GLOBAL_OBJECT_011DEA10, object);
        // A non-empty 011d86a8 and no spLODRoot: nothing else happens.
        e.mem.set_u32(SMART_POINTER_011D86A8, 0x1);
        let manager: Ptr<BGSTerrainManager> = e.new_object();
        e.set(manager, BGSTerrainManager::pRootNode, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        e.call(0x006f_c490, &args![manager, world]);
        assert!(e.get(manager, BGSTerrainManager::pRootNode).is_null());
        assert!(!e.get(manager, BGSTerrainManager::bHasLOD));
        assert!(calls_to(&e, FILE_OPEN).is_empty());
        assert!(calls_to(&e, FILE_READ).is_empty());
        assert!(calls_to(&e, OBJECT_CALL_00546780).is_empty());
        assert!(calls_to(&e, NODE_ALLOCATE).is_empty());
    }

    #[test]
    fn constructor_handles_an_unopened_file_and_a_failed_allocation() {
        let mut e = manager_engine();
        let (world, _) = world(&mut e);
        e.register(FILE_EXISTS, |_, _| ret(1));
        // The file exists but cannot be opened.
        e.register(FILE_OPEN, |_, _| ret(0));
        let object = e.mem.alloc(0x20);
        e.set_global(GLOBAL_OBJECT_011DEA10, object);
        e.mem.set_u32(SMART_POINTER_011D86A8, 0x1);
        let manager: Ptr<BGSTerrainManager> = e.new_object();
        e.call(0x006f_c490, &args![manager, world]);
        assert!(e.get(manager, BGSTerrainManager::pRootNode).is_null());
        assert!(!e.get(manager, BGSTerrainManager::bHasLOD));
        // The file opens, but the allocator returns 0: the root stays null
        // while the manager is marked as having LOD.
        settings_file(&mut e);
        script_reads(&mut e, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        e.register(ALLOCATE, |_, _| ret(0));
        e.register(NODE_CONSTRUCT, |_, _| ret(0xdead));
        let manager: Ptr<BGSTerrainManager> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006f_c490, &args![manager, world]);
        assert!(calls_to(&e, NODE_CONSTRUCT).is_empty());
        assert!(e.get(manager, BGSTerrainManager::pRootNode).is_null());
        assert!(e.get(manager, BGSTerrainManager::bHasLOD));
    }

    #[test]
    fn file_readers_store_the_result_in_the_file_object_and_return_it() {
        let mut e = Engine::new();
        e.register(FILE_READ, |_, a| ret(0x40 + a[2]));
        let file = Ptr::<()>::new(e.mem.alloc(0x150));
        let out = Ptr::<()>::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_c920, &args![file, out]).ptr::<()>(), file);
        assert_eq!(e.mem.u32(file.addr() + 0x148), 0x42);
        assert_eq!(e.call(0x006f_c950, &args![file, out]).ptr::<()>(), file);
        assert_eq!(e.mem.u32(file.addr() + 0x148), 0x44);
        assert_eq!(
            calls_to(&e, FILE_READ),
            vec![
                vec![file.addr(), out.addr(), 2],
                vec![file.addr(), out.addr(), 4]
            ]
        );
    }

    #[test]
    fn destructor_deletes_the_root_node_and_clears_the_members_in_order() {
        let mut e = manager_engine();
        e.register(NODE_DESTRUCT, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let manager: Ptr<BGSTerrainManager> = e.new_object();
        e.set(manager, BGSTerrainManager::pRootNode, Ptr::new(0x6000));
        e.mem.set_u32(manager.addr() + 8, 0x11);
        e.mem.set_u32(manager.addr() + 0xc, 0x22);
        e.call_log = Some(vec![]);
        e.call(0x006f_c980, &args![manager]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap();
        let m = manager.addr();
        assert_eq!(
            order,
            vec![
                (0x006f_c980, vec![m]),
                (NODE_DESTRUCT, vec![0x6000]),
                (FREE, vec![0x6000]),
                (POINTER_SET, vec![m + 8, 0]),
                (POINTER_SET, vec![m + 0xc, 0]),
                (ARRAY_DESTRUCT, vec![m + 0x2c]),
                (POINTER_DESTRUCT, vec![m + 0xc]),
                (POINTER_DESTRUCT, vec![m + 8]),
            ]
        );
        assert_eq!(e.mem.u32(m + 8), 0);
        assert_eq!(e.mem.u32(m + 0xc), 0);
    }

    #[test]
    fn destructor_without_a_root_node_skips_the_node_destructor() {
        let mut e = manager_engine();
        e.register(NODE_DESTRUCT, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let manager: Ptr<BGSTerrainManager> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006f_c980, &args![manager]);
        assert!(calls_to(&e, NODE_DESTRUCT).is_empty());
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(calls_to(&e, POINTER_SET).len(), 2);
    }

    #[test]
    fn deleting_destructor_frees_only_when_bit_zero_is_set() {
        let mut e = Engine::new();
        e.register(NODE_DESTRUCT, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let node = Ptr::<()>::new(0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006f_ca40, &args![node, 0u32]).ptr::<()>(), node);
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(e.call(0x006f_ca40, &args![node, 1u32]).ptr::<()>(), node);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x6000]]);
        // Only bit 0 counts.
        e.call_log = Some(vec![]);
        e.call(0x006f_ca40, &args![node, 2u32]);
        assert!(calls_to(&e, FREE).is_empty());
        assert_eq!(calls_to(&e, NODE_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_006fca70_calls_the_function_with_the_word_at_plus_four() {
        let mut e = Engine::new();
        e.register(NODE_CALL_006FDA10, |_, _| Ret::default());
        let object = Ptr::<()>::new(e.mem.alloc(8));
        e.call_log = Some(vec![]);
        e.call(0x006f_ca70, &args![object]);
        assert!(calls_to(&e, NODE_CALL_006FDA10).is_empty());
        e.mem.set_u32(object.addr() + 4, 0x6000);
        e.call(0x006f_ca70, &args![object]);
        assert_eq!(calls_to(&e, NODE_CALL_006FDA10), vec![vec![0x6000]]);
    }
}
