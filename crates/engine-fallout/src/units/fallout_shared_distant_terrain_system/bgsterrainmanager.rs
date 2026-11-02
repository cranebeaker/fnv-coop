//! `fallout shared/distant terrain system/bgsterrainmanager.cpp` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The distant-terrain manager keeps the root `BGSTerrainNode` of the quad
//! tree at `+4` and the trees queued to hide in an array at `+0x2c`. The
//! node, chunk and tree-block functions it calls belong to other units and
//! are called by address.

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPointer` dereference (`*this`): the pointer stored in the smart
/// pointer at `this`.
const POINTER_GET: u32 = 0x0055_9450;
/// `BSSimpleArray<T,1024>` element count (`iSize`).
const ARRAY_COUNT: u32 = 0x0044_ddc0;
/// `BSSimpleArray<T,1024>` element address: `pBuffer + 4 * index`.
const ARRAY_ELEMENT_ADDRESS: u32 = 0x0087_7a30;
/// The reference's form key (its body calls `00485bc0` and `00484e60`; the
/// high byte carries a flag).
const REFERENCE_FORM_KEY: u32 = 0x0048_5b60;
/// Returns `this + 4`, or the address of a scratch byte (set to 0) when
/// `this` is null. The code reads the byte it returns.
const VALUE_BYTE_ADDRESS: u32 = 0x0040_8d60;
/// `BSSimpleArray<T,1024>` constructor / destructor.
const ARRAY_CONSTRUCT: u32 = 0x005e_04c0;
const ARRAY_DESTRUCT: u32 = 0x005e_04f0;
/// `BSSimpleArray<T,1024>`: appends the word at the given address.
const ARRAY_APPEND: u32 = 0x007c_b2e0;
/// `BSSimpleArray<T,1024>` (Xbox PDB map name `BSSimpleArray<ItemChange*,
/// 1024>::Remove`; the body is shared by the instances): removes `count`
/// elements from `index`.
const ARRAY_REMOVE: u32 = 0x006b_f8f0;
/// `BSSimpleArray<T,1024>`: empties the array, freeing the buffer when the
/// argument is nonzero.
const ARRAY_CLEAR: u32 = 0x0084_54f0;
/// `BSSimpleArray<T,1024>` (Xbox PDB map name `BSSimpleArray<TESObjectWEAP*,
/// 1024>::SetSize`): sets the element count; the second argument says
/// whether to free the old buffer.
const ARRAY_SET_SIZE: u32 = 0x006f_2290;
/// `BSSimpleArray<T,1024>`: the address of element `index` (calls
/// [`ARRAY_ELEMENT_ADDRESS`]).
const ARRAY_AT: u32 = 0x006a_7ad0;
/// `BSSimpleArray<T,1024>`: element `index` = the word at the given address.
const ARRAY_SET_AT: u32 = 0x0047_a110;

/// `BGSTerrainNode::UpdateTerrain` (Xbox PDB map): the node, the position
/// and a flag.
const NODE_UPDATE_TERRAIN: u32 = 0x006f_daa0;
/// `BGSTerrainNode::UpdateTrees` (Xbox PDB map): the node and the position.
const NODE_UPDATE_TREES: u32 = 0x006f_e330;
/// Node functions without an Xbox PDB name (`bgsterrainnode.cpp`).
const NODE_CALL_006FDFC0: u32 = 0x006f_dfc0;
const NODE_CALL_006FEA70: u32 = 0x006f_ea70;
const NODE_CALL_006FEC90: u32 = 0x006f_ec90;
const NODE_CALL_006FED30: u32 = 0x006f_ed30;
const NODE_CALL_006FD980: u32 = 0x006f_d980;
/// `BGSTerrainChunk::GetTexture` (Xbox PDB map): a chunk and an index.
const CHUNK_GET_TEXTURE: u32 = 0x006f_abc0;
/// A function of `bgsdistanttreeblock.cpp` called with a tree block, a
/// form key and 1.
const TREE_BLOCK_CALL_006F9230: u32 = 0x006f_9230;

/// The byte at `0119f188` that gates the update (identity unconfirmed).
const FEATURE_BYTE: u32 = 0x0119_f188;
/// Three objects whose byte at `+4` the update tests (through
/// [`VALUE_BYTE_ADDRESS`]).
const VALUE_OBJECT_011D86AC: u32 = 0x011d_86ac;
const VALUE_OBJECT_011D869C: u32 = 0x011d_869c;
const VALUE_OBJECT_011D8680: u32 = 0x011d_8680;
/// A global word the update sets to the manager it runs for (the Xbox PDB
/// has a static `BGSTerrainManager::pActiveManager`; the match is
/// unconfirmed).
const ACTIVE_MANAGER: u32 = 0x011d_8670;
/// A smart pointer at `011d86bc`, tested and then handed to `00572160`.
const SMART_POINTER_011D86BC: u32 = 0x011d_86bc;
/// `00572160(this)`: walks the array at `this + 0x9c`.
const OBJECT_CALL_00572160: u32 = 0x0057_2160;
/// The global word returned by [`fn_006fcda0`].
const GLOBAL_VALUE_011D8598: u32 = 0x011d_8598;
/// The global object pointer used as `this` of [`RECT_CALL_00451020`] and
/// [`INIT_DISTANT_TEXTURE_BLENDING`].
const GLOBAL_OBJECT_011DEA10: u32 = 0x011d_ea10;
/// `00451020(this, out, float)`: writes four words to `out` (a rectangle).
const RECT_CALL_00451020: u32 = 0x0045_1020;
/// `00b4f300(out)` (cdecl): reads that rectangle and stores four floats at
/// `011f95f4`.
const RECT_CALL_00B4F300: u32 = 0x00b4_f300;
/// `TES::InitDistantTextureBlending` (Xbox PDB map name): `this` = the
/// object at [`GLOBAL_OBJECT_011DEA10`].
const INIT_DISTANT_TEXTURE_BLENDING: u32 = 0x0045_cac0;

layout! {
    /// `BGSTerrainManager` (Xbox PDB), 0x3C bytes on the Xbox (the PC size is
    /// not confirmed).
    pub struct BGSTerrainManager: 0x3C {
        /// `pWorld` (Xbox PDB): `TESWorldSpace*`.
        0x00 pWorld: Ptr,
        /// `pRootNode` (Xbox PDB): `BGSTerrainNode*`.
        0x04 pRootNode: Ptr,
        /// `spLODRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x08 spLODRoot: Ptr,
        /// `spLODWaterRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x0C spLODWaterRoot: Ptr,
        /// `sMinCellX` (Xbox PDB).
        0x10 sMinCellX: i16,
        /// `sMinCellY` (Xbox PDB).
        0x12 sMinCellY: i16,
        /// `sMaxCellX` (Xbox PDB).
        0x14 sMaxCellX: i16,
        /// `sMaxCellY` (Xbox PDB).
        0x16 sMaxCellY: i16,
        /// `iMaxLevel` (Xbox PDB).
        0x18 iMaxLevel: u32,
        /// `iMinLevel` (Xbox PDB).
        0x1C iMinLevel: u32,
        /// `iRootLevel` (Xbox PDB).
        0x20 iRootLevel: u32,
        /// `iBlockLevel` (Xbox PDB).
        0x24 iBlockLevel: u32,
        /// `bNeedsImmediateUpdate` (Xbox PDB).
        0x28 bNeedsImmediateUpdate: bool,
        /// `bHasLOD` (Xbox PDB).
        0x29 bHasLOD: bool,
        /// `TreesQueuedToHide` (Xbox PDB): `BSSimpleArray<TESObjectREFR*,1024>`.
        0x2C TreesQueuedToHide: Inline<crate::types::BSSimpleArray>,
    }
}

/// The value byte of the object at `object`.
fn value_byte(e: &mut Engine, object: u32) -> u8 {
    let address = e.call(VALUE_BYTE_ADDRESS, &args![object]).u32();
    e.mem.u8(address)
}

// Translated from 006fca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainManager::Update` (Xbox PDB): when the feature byte and the
/// first value object's byte are on, publishes the view rectangle, makes
/// this manager the active one (running `006fce00` on the previous one and
/// `00572160` on the smart pointer at `011d86bc`), and, with a root node,
/// updates it by `flags` (bit 0 is always set here and runs the terrain
/// update with bit 2 as its flag, then, with the second value object on, the
/// `006fdfc0` update with bit 1; bit 3 runs the trees update when the third
/// value object is on). It then hands every queued tree to the tree block
/// that owns it and keeps, in order, the trees nobody owns in the queue.
/// `bNeedsImmediateUpdate` is cleared on every path.
///
/// `position` points at the three words of the viewer position (copied to a
/// local before use).
// Translation incomplete: 006fca90 - SEH exception unwinding (FS:[0] frame) is unsupported.
pub fn bgs_terrain_manager_update(
    e: &mut Engine,
    this: Ptr<BGSTerrainManager>,
    position: Ptr,
    flags: u32,
) {
    if e.global::<u8>(FEATURE_BYTE) != 0 && value_byte(e, VALUE_OBJECT_011D86AC) != 0 {
        let words = [
            e.mem.u32(position.addr()),
            e.mem.u32(position.addr() + 4),
            e.mem.u32(position.addr() + 8),
        ];
        let flags = flags | 1;
        e.with_stack(0x10, |e, rect| {
            let object = e.global::<u32>(GLOBAL_OBJECT_011DEA10);
            e.call(RECT_CALL_00451020, &args![object, rect, 0.0f32]);
            e.call(RECT_CALL_00B4F300, &args![rect]);
        });
        let active = e.global::<u32>(ACTIVE_MANAGER);
        if active != 0 && active != this.addr() {
            e.set_global(ACTIVE_MANAGER, this.addr());
            fn_006fce00(e, Ptr::new(active));
            if e.call(POINTER_GET, &args![SMART_POINTER_011D86BC]).u32() != 0 {
                let pointer = e.call(POINTER_GET, &args![SMART_POINTER_011D86BC]).u32();
                e.call(OBJECT_CALL_00572160, &args![pointer]);
            }
        }
        e.set_global(ACTIVE_MANAGER, this.addr());
        if !e.get(this, BGSTerrainManager::pRootNode).is_null() {
            e.with_stack(12, |e, local| {
                for (i, word) in words.iter().enumerate() {
                    e.mem.set_u32(local.addr() + 4 * i as u32, *word);
                }
                update_root_node(e, this, local, flags);
            });
        }
    }
    e.set(this, BGSTerrainManager::bNeedsImmediateUpdate, false);
}

/// The part of [`bgs_terrain_manager_update`] that runs with a root node;
/// `local` is the copy of the position.
fn update_root_node(e: &mut Engine, this: Ptr<BGSTerrainManager>, local: Ptr, flags: u32) {
    if flags & 1 != 0 {
        let root = e.get(this, BGSTerrainManager::pRootNode);
        let terrain_flag = u32::from(flags & 4 != 0);
        e.call(NODE_UPDATE_TERRAIN, &args![root, local, terrain_flag]);
        let object = e.global::<u32>(GLOBAL_OBJECT_011DEA10);
        e.call(INIT_DISTANT_TEXTURE_BLENDING, &args![object]);
        if value_byte(e, VALUE_OBJECT_011D869C) != 0 {
            let root = e.get(this, BGSTerrainManager::pRootNode);
            let split_flag = u32::from(flags & 2 != 0);
            e.call(NODE_CALL_006FDFC0, &args![root, local, split_flag]);
        }
    }
    if flags & 8 != 0 && value_byte(e, VALUE_OBJECT_011D8680) != 0 {
        let root = e.get(this, BGSTerrainManager::pRootNode);
        e.call(NODE_UPDATE_TREES, &args![root, local]);
        if fn_006fcda0(e) != 0 && e.call(ARRAY_COUNT, &args![this.addr() + 0x2c]).u32() != 0 {
            hand_over_queued_trees(e, this);
        }
    }
}

/// The tail of [`bgs_terrain_manager_update`]: every tree in the queue goes
/// to the tree block that owns it, or into a temporary array that then
/// replaces the queue.
fn hand_over_queued_trees(e: &mut Engine, this: Ptr<BGSTerrainManager>) {
    let queue = this.addr() + 0x2c;
    e.with_stack(0x10, |e, kept| {
        e.call(ARRAY_CONSTRUCT, &args![kept]);
        let count = e.call(ARRAY_COUNT, &args![queue]).u32();
        for index in 0..count {
            let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![queue, index]).u32();
            let tree = e.mem.u32(slot);
            let tree_key = e.call(REFERENCE_FORM_KEY, &args![tree]).u32();
            let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![queue, index]).u32();
            let tree = e.mem.u32(slot);
            let block_key = e.vcall(tree, 0x1f4, &[]).u32();
            let block = fn_006fcef0(e, this, block_key);
            if block != 0 {
                e.call(TREE_BLOCK_CALL_006F9230, &args![block, tree_key, 1u32]);
            } else {
                let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![queue, index]).u32();
                e.call(ARRAY_APPEND, &args![kept, slot]);
            }
        }
        e.call(ARRAY_CLEAR, &args![queue, 1u32]);
        if e.call(ARRAY_COUNT, &args![kept]).u32() != 0 {
            let kept_count = e.call(ARRAY_COUNT, &args![kept]).u32();
            e.call(ARRAY_SET_SIZE, &args![queue, kept_count, 1u32]);
            for index in 0..kept_count {
                let element = e.call(ARRAY_AT, &args![kept, index]).u32();
                e.call(ARRAY_SET_AT, &args![queue, index, element]);
            }
        }
        e.call(ARRAY_DESTRUCT, &args![kept]);
    });
}

// Translated from 006fcda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global word at `011d8598` (`BGSTerrainManager::Update` tests it
/// before handing queued trees over; its meaning is unconfirmed).
pub fn fn_006fcda0(e: &mut Engine) -> u32 {
    e.global::<u32>(GLOBAL_VALUE_011D8598)
}

// Translated from 006fcdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainManager::UpdateMorphParameters` (Xbox PDB): with the feature
/// byte and the first value object's byte on and a root node, passes `value`
/// on to the root node's function `006fea70`.
pub fn bgs_terrain_manager_update_morph_parameters(
    e: &mut Engine,
    this: Ptr<BGSTerrainManager>,
    value: u32,
) {
    if e.global::<u8>(FEATURE_BYTE) != 0 && value_byte(e, VALUE_OBJECT_011D86AC) != 0 {
        let root = e.get(this, BGSTerrainManager::pRootNode);
        if !root.is_null() {
            e.call(NODE_CALL_006FEA70, &args![root, value]);
        }
    }
}

// Translated from 006fce00 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a root node, calls its function `006fd980` with 7.
pub fn fn_006fce00(e: &mut Engine, this: Ptr<BGSTerrainManager>) {
    let root = e.get(this, BGSTerrainManager::pRootNode);
    if !root.is_null() {
        e.call(NODE_CALL_006FD980, &args![root, 7u32]);
    }
}

// Translated from 006fce30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The chunk `006fceb0(key)` finds, asked for its texture 0
/// (`BGSTerrainChunk::GetTexture`); 0 when there is none.
pub fn fn_006fce30(e: &mut Engine, this: Ptr<BGSTerrainManager>, key: u32) -> u32 {
    let chunk = fn_006fceb0(e, this, key);
    if chunk != 0 {
        e.call(CHUNK_GET_TEXTURE, &args![chunk, 0u32]).u32()
    } else {
        0
    }
}

// Translated from 006fce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The chunk `006fceb0(key)` finds, asked for its texture 1
/// (`BGSTerrainChunk::GetTexture`); 0 when there is none.
pub fn fn_006fce70(e: &mut Engine, this: Ptr<BGSTerrainManager>, key: u32) -> u32 {
    let chunk = fn_006fceb0(e, this, key);
    if chunk != 0 {
        e.call(CHUNK_GET_TEXTURE, &args![chunk, 1u32]).u32()
    } else {
        0
    }
}

// Translated from 006fceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the root node for what `006fec90` returns for `key`; 0 without a
/// root node.
pub fn fn_006fceb0(e: &mut Engine, this: Ptr<BGSTerrainManager>, key: u32) -> u32 {
    let root = e.get(this, BGSTerrainManager::pRootNode);
    if root.is_null() {
        0
    } else {
        e.call(NODE_CALL_006FEC90, &args![root, key]).u32()
    }
}

// Translated from 006fcef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the root node for what `006fed30` returns for `key` (the code uses
/// the result as a distant tree block); 0 without a root node.
pub fn fn_006fcef0(e: &mut Engine, this: Ptr<BGSTerrainManager>, key: u32) -> u32 {
    let root = e.get(this, BGSTerrainManager::pRootNode);
    if root.is_null() {
        0
    } else {
        e.call(NODE_CALL_006FED30, &args![root, key]).u32()
    }
}

// Translated from 006fcf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainManager::ReleaseTree` (Xbox PDB): removes every entry equal to
/// `tree` from the queued-to-hide array.
pub fn bgs_terrain_manager_release_tree(e: &mut Engine, this: Ptr<BGSTerrainManager>, tree: u32) {
    let queue = this.addr() + 0x2c;
    let mut count = e.call(ARRAY_COUNT, &args![queue]).u32();
    let mut index = 0u32;
    while index < count {
        let slot = e.call(ARRAY_ELEMENT_ADDRESS, &args![queue, index]).u32();
        if e.mem.u32(slot) == tree {
            e.call(ARRAY_REMOVE, &args![queue, index, 1u32]);
            count -= 1;
        } else {
            index += 1;
        }
    }
}

// Translated from 006fcfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSTerrainManager::HideTree` (Xbox PDB): hands the tree reference `tree`
/// to the tree block that owns it (key from the reference's virtual at
/// `+0x1f4`), or, when no block owns it and `queue_if_missing` is set,
/// appends it to the queued-to-hide array.
pub fn bgs_terrain_manager_hide_tree(
    e: &mut Engine,
    this: Ptr<BGSTerrainManager>,
    tree: u32,
    queue_if_missing: u8,
) {
    let tree_key = e.call(REFERENCE_FORM_KEY, &args![tree]).u32();
    let block_key = e.vcall(tree, 0x1f4, &[]).u32();
    let block = fn_006fcef0(e, this, block_key);
    if block != 0 {
        e.call(TREE_BLOCK_CALL_006F9230, &args![block, tree_key, 1u32]);
    } else if queue_if_missing != 0 {
        e.with_stack(4, |e, argument| {
            e.mem.set_u32(argument.addr(), tree);
            e.call(ARRAY_APPEND, &args![this.addr() + 0x2c, argument]);
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006fca90,
            bgs_terrain_manager_update(Ptr<BGSTerrainManager>, Ptr, u32)
        ),
        entry!(0x006fcda0, fn_006fcda0() -> u32),
        entry!(
            0x006fcdb0,
            bgs_terrain_manager_update_morph_parameters(Ptr<BGSTerrainManager>, u32)
        ),
        entry!(0x006fce00, fn_006fce00(Ptr<BGSTerrainManager>)),
        entry!(0x006fce30, fn_006fce30(Ptr<BGSTerrainManager>, u32) -> u32),
        entry!(0x006fce70, fn_006fce70(Ptr<BGSTerrainManager>, u32) -> u32),
        entry!(0x006fceb0, fn_006fceb0(Ptr<BGSTerrainManager>, u32) -> u32),
        entry!(0x006fcef0, fn_006fcef0(Ptr<BGSTerrainManager>, u32) -> u32),
        entry!(
            0x006fcf30,
            bgs_terrain_manager_release_tree(Ptr<BGSTerrainManager>, u32)
        ),
        entry!(
            0x006fcfa0,
            bgs_terrain_manager_hide_tree(Ptr<BGSTerrainManager>, u32, u8)
        ),
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

    /// Scratch words the doubles write their observations to.
    const SCRATCH: u32 = 0x011d_8f00;

    /// An engine with the globals' pages mapped and the helpers every test
    /// needs doubled (smart-pointer read, value-byte getter, array count and
    /// element address).
    fn manager_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0119_f000, 0x011d_8000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(VALUE_BYTE_ADDRESS, |_, a| ret(a[0] + 4));
        e.register(ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 4 * a[1])
        });
        e
    }

    /// Turns on the feature byte and the three value objects' bytes.
    fn enable_all(e: &mut Engine) {
        e.set_global(FEATURE_BYTE, 1u8);
        for object in [
            VALUE_OBJECT_011D86AC,
            VALUE_OBJECT_011D869C,
            VALUE_OBJECT_011D8680,
        ] {
            e.set_global(object + 4, 1u8);
        }
    }

    /// A manager whose root node is `root` (0 for none).
    fn manager_with_root(e: &mut Engine, root: u32) -> Ptr<BGSTerrainManager> {
        let manager: Ptr<BGSTerrainManager> = e.new_object();
        e.set(manager, BGSTerrainManager::pRootNode, Ptr::new(root));
        manager
    }

    /// Fills the queued-to-hide array of `manager` with `items`.
    fn fill_queue(e: &mut Engine, manager: Ptr<BGSTerrainManager>, items: &[u32]) {
        let buffer = e.mem.alloc(64);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *item);
        }
        e.mem.set_u32(manager.addr() + 0x2c + 4, buffer);
        e.mem.set_u32(manager.addr() + 0x2c + 8, items.len() as u32);
        e.mem.set_u32(manager.addr() + 0x2c + 0xc, 16);
    }

    fn queue_items(e: &Engine, manager: Ptr<BGSTerrainManager>) -> Vec<u32> {
        let buffer = e.mem.u32(manager.addr() + 0x2c + 4);
        let count = e.mem.u32(manager.addr() + 0x2c + 8);
        (0..count).map(|i| e.mem.u32(buffer + 4 * i)).collect()
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

    #[test]
    fn global_word_is_returned() {
        let mut e = manager_engine();
        e.set_global(GLOBAL_VALUE_011D8598, 0x1234_5678u32);
        assert_eq!(e.call(0x006f_cda0, &args![]).u32(), 0x1234_5678);
    }

    #[test]
    fn update_morph_parameters_passes_the_value_to_the_root_node() {
        let mut e = manager_engine();
        enable_all(&mut e);
        let manager = manager_with_root(&mut e, 0x5000);
        e.register(NODE_CALL_006FEA70, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x006f_cdb0, &args![manager, 0x77u32]);
        assert_eq!(calls_to(&e, NODE_CALL_006FEA70), vec![vec![0x5000, 0x77]]);
    }

    #[test]
    fn update_morph_parameters_needs_the_feature_the_value_byte_and_a_root() {
        let mut e = manager_engine();
        e.register(NODE_CALL_006FEA70, |_, _| Ret::default());
        let manager = manager_with_root(&mut e, 0x5000);
        e.call_log = Some(vec![]);
        // Feature byte off.
        e.call(0x006f_cdb0, &args![manager, 1u32]);
        // Feature on, value byte off.
        e.set_global(FEATURE_BYTE, 1u8);
        e.call(0x006f_cdb0, &args![manager, 1u32]);
        // Both on, no root node.
        e.set_global(VALUE_OBJECT_011D86AC + 4, 1u8);
        let rootless = manager_with_root(&mut e, 0);
        e.call(0x006f_cdb0, &args![rootless, 1u32]);
        assert!(calls_to(&e, NODE_CALL_006FEA70).is_empty());
    }

    #[test]
    fn fn_006fce00_calls_the_root_node_with_seven() {
        let mut e = manager_engine();
        e.register(NODE_CALL_006FD980, |_, _| Ret::default());
        let manager = manager_with_root(&mut e, 0x5000);
        let rootless = manager_with_root(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_ce00, &args![manager]);
        e.call(0x006f_ce00, &args![rootless]);
        assert_eq!(calls_to(&e, NODE_CALL_006FD980), vec![vec![0x5000, 7]]);
    }

    #[test]
    fn chunk_lookups_return_the_texture_or_zero() {
        let mut e = manager_engine();
        // The node finds a chunk for key 5 only; the texture is index + 0x100.
        e.register(NODE_CALL_006FEC90, |_, a| {
            ret(if a[1] == 5 { 0x6000 } else { 0 })
        });
        e.register(CHUNK_GET_TEXTURE, |_, a| ret(a[0] + 0x100 + a[1]));
        let manager = manager_with_root(&mut e, 0x5000);
        let rootless = manager_with_root(&mut e, 0);
        assert_eq!(e.call(0x006f_ceb0, &args![manager, 5u32]).u32(), 0x6000);
        assert_eq!(e.call(0x006f_ceb0, &args![manager, 6u32]).u32(), 0);
        assert_eq!(e.call(0x006f_ceb0, &args![rootless, 5u32]).u32(), 0);
        assert_eq!(e.call(0x006f_ce30, &args![manager, 5u32]).u32(), 0x6100);
        assert_eq!(e.call(0x006f_ce70, &args![manager, 5u32]).u32(), 0x6101);
        assert_eq!(e.call(0x006f_ce30, &args![manager, 6u32]).u32(), 0);
        assert_eq!(e.call(0x006f_ce70, &args![rootless, 5u32]).u32(), 0);
    }

    #[test]
    fn tree_block_lookup_asks_the_root_node() {
        let mut e = manager_engine();
        e.register(NODE_CALL_006FED30, |_, a| ret(a[1] + 0x9000));
        let manager = manager_with_root(&mut e, 0x5000);
        let rootless = manager_with_root(&mut e, 0);
        assert_eq!(e.call(0x006f_cef0, &args![manager, 3u32]).u32(), 0x9003);
        assert_eq!(e.call(0x006f_cef0, &args![rootless, 3u32]).u32(), 0);
    }

    /// Removes `a[2]` elements at index `a[1]` of the array `a[0]`.
    fn array_remove_double(e: &mut Engine, a: &[u32]) -> Ret {
        let buffer = e.mem.u32(a[0] + 4);
        let count = e.mem.u32(a[0] + 8);
        for i in a[1]..count - a[2] {
            let next = e.mem.u32(buffer + 4 * (i + a[2]));
            e.mem.set_u32(buffer + 4 * i, next);
        }
        e.mem.set_u32(a[0] + 8, count - a[2]);
        Ret::default()
    }

    #[test]
    fn release_tree_removes_every_matching_entry() {
        let mut e = manager_engine();
        e.register(ARRAY_REMOVE, array_remove_double);
        let manager = manager_with_root(&mut e, 0);
        fill_queue(&mut e, manager, &[5, 5, 7, 5, 9, 5]);
        e.call(0x006f_cf30, &args![manager, 5u32]);
        assert_eq!(queue_items(&e, manager), vec![7, 9]);
        // Nothing to remove.
        e.call_log = Some(vec![]);
        e.call(0x006f_cf30, &args![manager, 4u32]);
        assert!(calls_to(&e, ARRAY_REMOVE).is_empty());
        assert_eq!(queue_items(&e, manager), vec![7, 9]);
    }

    /// A tree reference whose virtual at `+0x1f4` returns `key`; the key is
    /// stored at `+4`.
    fn tree_reference(e: &mut Engine, key: u32) -> u32 {
        const VTABLE: u32 = 0x0250_0000;
        const KEY_FUNCTION: u32 = 0x0250_1000;
        if !e.mem.is_mapped(VTABLE) {
            let mut slots = vec![0u32; 0x7e];
            slots[0x1f4 / 4] = KEY_FUNCTION;
            e.put_vtable(VTABLE, &slots);
            e.register(KEY_FUNCTION, |e, a| ret(e.mem.u32(a[0] + 4)));
        }
        let tree = e.mem.alloc(8);
        e.mem.set_u32(tree, VTABLE);
        e.mem.set_u32(tree + 4, key);
        tree
    }

    #[test]
    fn hide_tree_hands_the_tree_to_its_block() {
        let mut e = manager_engine();
        e.register(REFERENCE_FORM_KEY, |_, a| ret(a[0] + 1));
        e.register(NODE_CALL_006FED30, |_, a| ret(a[1] + 0x9000));
        e.register(TREE_BLOCK_CALL_006F9230, |_, _| Ret::default());
        let manager = manager_with_root(&mut e, 0x5000);
        let tree = tree_reference(&mut e, 0x11);
        e.call_log = Some(vec![]);
        e.call(0x006f_cfa0, &args![manager, tree, 1u32]);
        assert_eq!(
            calls_to(&e, TREE_BLOCK_CALL_006F9230),
            vec![vec![0x9011, tree + 1, 1]]
        );
        assert!(calls_to(&e, ARRAY_APPEND).is_empty());
    }

    #[test]
    fn hide_tree_queues_a_tree_nobody_owns_when_asked() {
        let mut e = manager_engine();
        e.register(REFERENCE_FORM_KEY, |_, a| ret(a[0] + 1));
        e.register(NODE_CALL_006FED30, |_, _| ret(0));
        e.register(TREE_BLOCK_CALL_006F9230, |_, _| Ret::default());
        e.register(ARRAY_APPEND, |e, a| {
            // Records the queue address and the word it was asked to append.
            e.mem.set_u32(SCRATCH, a[0]);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(SCRATCH + 4, value);
            Ret::default()
        });
        let manager = manager_with_root(&mut e, 0x5000);
        let tree = tree_reference(&mut e, 0x11);
        e.call_log = Some(vec![]);
        // Not asked to queue: nothing happens.
        e.call(0x006f_cfa0, &args![manager, tree, 0u32]);
        assert!(calls_to(&e, ARRAY_APPEND).is_empty());
        assert!(calls_to(&e, TREE_BLOCK_CALL_006F9230).is_empty());
        e.call(0x006f_cfa0, &args![manager, tree, 1u32]);
        assert_eq!(e.mem.u32(SCRATCH), manager.addr() + 0x2c);
        assert_eq!(e.mem.u32(SCRATCH + 4), tree);
    }

    /// Doubles for the node functions the update calls; the terrain update
    /// copies the position words it receives to the scratch words.
    fn update_engine() -> Engine {
        let mut e = manager_engine();
        enable_all(&mut e);
        e.register(RECT_CALL_00451020, |_, _| Ret::default());
        e.register(RECT_CALL_00B4F300, |_, _| Ret::default());
        e.register(NODE_CALL_006FD980, |_, _| Ret::default());
        e.register(OBJECT_CALL_00572160, |_, _| Ret::default());
        e.register(INIT_DISTANT_TEXTURE_BLENDING, |_, _| Ret::default());
        e.register(NODE_UPDATE_TERRAIN, |e, a| {
            for i in 0..3 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(SCRATCH + 4 * i, word);
            }
            Ret::default()
        });
        e.register(NODE_CALL_006FDFC0, |_, _| Ret::default());
        e.register(NODE_UPDATE_TREES, |_, _| Ret::default());
        e
    }

    fn position(e: &mut Engine) -> Ptr {
        let position = Ptr::new(e.mem.alloc(12));
        for (i, value) in [1.5f32, -2.0, 3.25].iter().enumerate() {
            e.mem.set_f32(position.addr() + 4 * i as u32, *value);
        }
        position
    }

    #[test]
    fn update_does_nothing_but_clear_the_flag_when_disabled() {
        let mut e = update_engine();
        e.set_global(FEATURE_BYTE, 0u8);
        let manager = manager_with_root(&mut e, 0x5000);
        e.set(manager, BGSTerrainManager::bNeedsImmediateUpdate, true);
        let at = position(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006f_ca90, &args![manager, at, 0xfu32]);
        assert!(!e.get(manager, BGSTerrainManager::bNeedsImmediateUpdate));
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // The first value object off disables it as well.
        e.set_global(FEATURE_BYTE, 1u8);
        e.set_global(VALUE_OBJECT_011D86AC + 4, 0u8);
        e.set(manager, BGSTerrainManager::bNeedsImmediateUpdate, true);
        e.call_log = Some(vec![]);
        e.call(0x006f_ca90, &args![manager, at, 0xfu32]);
        assert!(!e.get(manager, BGSTerrainManager::bNeedsImmediateUpdate));
        // The call itself and the value-byte getter.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.global::<u32>(ACTIVE_MANAGER), 0);
    }

    #[test]
    fn update_runs_the_terrain_update_with_the_copied_position() {
        let mut e = update_engine();
        let manager = manager_with_root(&mut e, 0x5000);
        e.set(manager, BGSTerrainManager::bNeedsImmediateUpdate, true);
        let at = position(&mut e);
        e.set_global(GLOBAL_OBJECT_011DEA10, 0x7777u32);
        e.call_log = Some(vec![]);
        // Flags 0: bit 0 is added, so the terrain update still runs (with
        // flag 0), the split update runs with flag 0, the trees do not.
        e.call(0x006f_ca90, &args![manager, at, 0u32]);
        assert_eq!(e.global::<u32>(ACTIVE_MANAGER), manager.addr());
        assert!(!e.get(manager, BGSTerrainManager::bNeedsImmediateUpdate));
        let terrain = calls_to(&e, NODE_UPDATE_TERRAIN);
        assert_eq!(terrain.len(), 1);
        assert_eq!(terrain[0][0], 0x5000);
        assert_eq!(terrain[0][2], 0);
        assert_eq!(e.mem.f32(SCRATCH), 1.5);
        assert_eq!(e.mem.f32(SCRATCH + 4), -2.0);
        assert_eq!(e.mem.f32(SCRATCH + 8), 3.25);
        let rect = calls_to(&e, RECT_CALL_00B4F300)[0][0];
        assert_eq!(
            calls_to(&e, RECT_CALL_00451020),
            vec![vec![0x7777, rect, 0]]
        );
        assert_eq!(
            calls_to(&e, INIT_DISTANT_TEXTURE_BLENDING),
            vec![vec![0x7777]]
        );
        let split = calls_to(&e, NODE_CALL_006FDFC0);
        assert_eq!(split.len(), 1);
        assert_eq!(split[0][2], 0);
        assert!(calls_to(&e, NODE_UPDATE_TREES).is_empty());
    }

    #[test]
    fn update_passes_flag_bits_two_and_four_and_skips_the_split_update_when_off() {
        let mut e = update_engine();
        let manager = manager_with_root(&mut e, 0x5000);
        let at = position(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006f_ca90, &args![manager, at, 6u32]);
        assert_eq!(calls_to(&e, NODE_UPDATE_TERRAIN)[0][2], 1);
        assert_eq!(calls_to(&e, NODE_CALL_006FDFC0)[0][2], 1);
        // Split-node value object off: no split update.
        e.set_global(VALUE_OBJECT_011D869C + 4, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x006f_ca90, &args![manager, at, 6u32]);
        assert_eq!(calls_to(&e, NODE_UPDATE_TERRAIN).len(), 1);
        assert!(calls_to(&e, NODE_CALL_006FDFC0).is_empty());
    }

    #[test]
    fn update_without_a_root_node_only_registers_the_manager() {
        let mut e = update_engine();
        let manager = manager_with_root(&mut e, 0);
        let at = position(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006f_ca90, &args![manager, at, 0xfu32]);
        assert_eq!(e.global::<u32>(ACTIVE_MANAGER), manager.addr());
        assert!(calls_to(&e, NODE_UPDATE_TERRAIN).is_empty());
    }

    #[test]
    fn update_clears_the_previous_active_manager() {
        let mut e = update_engine();
        let previous = manager_with_root(&mut e, 0x4000);
        let manager = manager_with_root(&mut e, 0x5000);
        e.set_global(ACTIVE_MANAGER, previous.addr());
        e.mem.set_u32(SMART_POINTER_011D86BC, 0x8800);
        let at = position(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006f_ca90, &args![manager, at, 0u32]);
        assert_eq!(calls_to(&e, NODE_CALL_006FD980), vec![vec![0x4000, 7]]);
        assert_eq!(calls_to(&e, OBJECT_CALL_00572160), vec![vec![0x8800]]);
        assert_eq!(e.global::<u32>(ACTIVE_MANAGER), manager.addr());
        // The same manager again: nothing is cleared.
        e.call_log = Some(vec![]);
        e.call(0x006f_ca90, &args![manager, at, 0u32]);
        assert!(calls_to(&e, NODE_CALL_006FD980).is_empty());
        // A null smart pointer: the previous manager is cleared, 00572160 is not called.
        e.set_global(ACTIVE_MANAGER, previous.addr());
        e.mem.set_u32(SMART_POINTER_011D86BC, 0);
        e.call_log = Some(vec![]);
        e.call(0x006f_ca90, &args![manager, at, 0u32]);
        assert_eq!(calls_to(&e, NODE_CALL_006FD980).len(), 1);
        assert!(calls_to(&e, OBJECT_CALL_00572160).is_empty());
    }

    /// Doubles for a small `BSSimpleArray` model over the real memory.
    fn array_engine() -> Engine {
        let mut e = update_engine();
        e.register(REFERENCE_FORM_KEY, |_, a| ret(a[0] + 1));
        // Keys 0x11 and 0x33 have a tree block, 0x22 has none.
        e.register(NODE_CALL_006FED30, |_, a| {
            ret(if a[1] == 0x22 { 0 } else { 0x9000 + a[1] })
        });
        e.register(TREE_BLOCK_CALL_006F9230, |_, _| Ret::default());
        e.register(ARRAY_CONSTRUCT, |e, a| {
            let buffer = e.mem.alloc(64);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 0xc, 16);
            Ret::default()
        });
        e.register(ARRAY_DESTRUCT, |_, _| Ret::default());
        e.register(ARRAY_APPEND, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            ret(size)
        });
        e.register(ARRAY_CLEAR, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(ARRAY_SET_AT, |e, a| {
            let buffer = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(buffer + 4 * a[1], value);
            Ret::default()
        });
        e.set_global(GLOBAL_VALUE_011D8598, 1u32);
        e
    }

    #[test]
    fn update_hands_queued_trees_to_their_blocks_and_keeps_the_rest_in_order() {
        let mut e = array_engine();
        let manager = manager_with_root(&mut e, 0x5000);
        let a = tree_reference(&mut e, 0x11);
        let b = tree_reference(&mut e, 0x22);
        let c = tree_reference(&mut e, 0x33);
        let d = tree_reference(&mut e, 0x22);
        fill_queue(&mut e, manager, &[a, b, c, d]);
        let at = position(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x006f_ca90, &args![manager, at, 8u32]);
        assert_eq!(
            calls_to(&e, TREE_BLOCK_CALL_006F9230),
            vec![vec![0x9011, a + 1, 1], vec![0x9033, c + 1, 1]]
        );
        assert_eq!(queue_items(&e, manager), vec![b, d]);
        assert_eq!(calls_to(&e, NODE_UPDATE_TREES).len(), 1);
    }

    #[test]
    fn update_empties_the_queue_when_every_tree_has_a_block() {
        let mut e = array_engine();
        let manager = manager_with_root(&mut e, 0x5000);
        let a = tree_reference(&mut e, 0x11);
        fill_queue(&mut e, manager, &[a]);
        let at = position(&mut e);
        e.call(0x006f_ca90, &args![manager, at, 8u32]);
        assert!(queue_items(&e, manager).is_empty());
    }

    #[test]
    fn update_leaves_the_queue_alone_without_the_trees_bit_the_setting_or_the_word() {
        let mut e = array_engine();
        let manager = manager_with_root(&mut e, 0x5000);
        let b = tree_reference(&mut e, 0x22);
        fill_queue(&mut e, manager, &[b]);
        let at = position(&mut e);
        e.call_log = Some(vec![]);
        // Bit 3 clear.
        e.call(0x006f_ca90, &args![manager, at, 7u32]);
        assert!(calls_to(&e, NODE_UPDATE_TREES).is_empty());
        // Trees value object off.
        e.set_global(VALUE_OBJECT_011D8680 + 4, 0u8);
        e.call(0x006f_ca90, &args![manager, at, 8u32]);
        assert!(calls_to(&e, NODE_UPDATE_TREES).is_empty());
        // The global word is zero: the trees update runs, the queue is untouched.
        e.set_global(VALUE_OBJECT_011D8680 + 4, 1u8);
        e.set_global(GLOBAL_VALUE_011D8598, 0u32);
        e.call(0x006f_ca90, &args![manager, at, 8u32]);
        assert_eq!(calls_to(&e, NODE_UPDATE_TREES).len(), 1);
        assert!(calls_to(&e, ARRAY_CLEAR).is_empty());
        assert_eq!(queue_items(&e, manager), vec![b]);
        // An empty queue is not processed either.
        e.set_global(GLOBAL_VALUE_011D8598, 1u32);
        fill_queue(&mut e, manager, &[]);
        e.call(0x006f_ca90, &args![manager, at, 8u32]);
        assert!(calls_to(&e, ARRAY_CLEAR).is_empty());
    }
}
