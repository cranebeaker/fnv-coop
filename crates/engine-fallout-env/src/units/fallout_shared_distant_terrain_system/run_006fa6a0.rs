//! `fallout shared/distant terrain system/run_006fa6a0` (Xbox PDB source unit), subsystem `fallout shared/distant terrain system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions work on the list of loaded terrain chunks at `011d8624`
//! (a `BSSimpleList` whose head node is the global itself: item at `+0`,
//! next at `+4`). The Xbox PDB has a static `BGSTerrainChunk::ChunksLoaded`
//! of that type; that this global is it is not confirmed by name, but the
//! chunk destructor (`006fa420`) removes the chunk from this list. Chunk
//! fields follow `BGSTerrainChunk` (Xbox PDB): `+0` `pNode`, `+4` `spMesh`,
//! `+0x10` `spNode`, `+0x14` `spWaterNode`, `+0x18` `spWaterReflNode`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The list head of the loaded chunks.
const CHUNKS_LOADED_LIST: u32 = 0x011d_8624;
/// `BSSimpleList` node accessors: the address of the node's item word
/// (`this`) and the next node (`[this + 4]`).
const LIST_ITEM_ADDRESS: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
/// `BSSimpleList::Remove` of the item whose address is given (list head
/// `this`; the argument is the address of a word holding the item).
const LIST_REMOVE: u32 = 0x0090_5330;
/// `NiPointer` dereference (`*this`).
const POINTER_GET: u32 = 0x0055_9450;
/// The word at `+0x8c` of its `this` is the address of three floats
/// (`this + 0x8c`).
const VECTOR_ADDRESS: u32 = 0x0045_bb80;
/// Global table lookup by index (`[0x11f91c8 + 4 * index]`).
const GLOBAL_TABLE_ENTRY: u32 = 0x0045_0b80;
/// `00b5bbe0(this, node, x, y, z, flag)` of `shadowscenenode.cpp`.
const NODE_UPDATE_AT: u32 = 0x00b5_bbe0;
/// Three links of the chain that clears a field of an object found through
/// the node (`006629f0`, then `0043b230`, then `00528820`).
const NODE_FIELD_AC: u32 = 0x0066_29f0;
const OBJECT_FIELD_0C: u32 = 0x0043_b230;
const OBJECT_CLEAR_FIELD_08: u32 = 0x0052_8820;
/// Chunk functions (`bgsterrainchunk.cpp`) called with `(chunk, 1)`.
const CHUNK_CALL_006FB170: u32 = 0x006f_b170;
const CHUNK_CALL_006FB3D0: u32 = 0x006f_b3d0;
/// The byte at `0119f154` that [`fn_006fa920`] sets.
const MODE_BYTE: u32 = 0x0119_f154;

/// The item of the list node at `node` (through the node accessor).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let address = e.call(LIST_ITEM_ADDRESS, &args![node]).u32();
    e.mem.u32(address)
}

// Translated from 006fa6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `this` (a chunk) from the list of loaded chunks: the list's remove
/// takes the address of a local holding the pointer.
pub fn fn_006fa6a0(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), this.addr());
        e.call(LIST_REMOVE, &args![CHUNKS_LOADED_LIST, local]);
    });
}

// Translated from 006fa820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer held by the smart pointer at `this + 0x10` (`spNode` of a
/// chunk).
pub fn fn_006fa820(e: &mut Engine, this: Ptr) -> u32 {
    e.call(POINTER_GET, &args![this.addr() + 0x10]).u32()
}

// Translated from 006fa840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this + 0x5d` (`bSplitNode` of a `BGSTerrainNode` in the Xbox
/// PDB; the receiver's class is confirmed only by that offset).
pub fn fn_006fa840(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x5d)
}

/// Calls `00b5bbe0` for `target` the way [`fn_006fa6d0`] does: on the global
/// table's entry 0, with the node, the three position words and `flag`.
fn update_node_at(e: &mut Engine, target: u32, position: [u32; 3], flag: u32) {
    let table_entry = e.call(GLOBAL_TABLE_ENTRY, &args![0u32]).u32();
    e.call(
        NODE_UPDATE_AT,
        &args![
            table_entry,
            target,
            position[0],
            position[1],
            position[2],
            flag
        ],
    );
}

// Translated from 006fa6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every loaded chunk whose node is not a split node (`006fa840`), runs
/// `00b5bbe0` on its `spNode`, `spWaterNode` and `spWaterReflNode` (each when
/// set) with the three position words found at `+0x8c` of `source` and
/// `flag`. A cdecl function.
pub fn fn_006fa6d0(e: &mut Engine, source: u32, flag: u32) {
    let vector = e.call(VECTOR_ADDRESS, &args![source]).u32();
    let position = [
        e.mem.u32(vector),
        e.mem.u32(vector + 4),
        e.mem.u32(vector + 8),
    ];
    let mut node = CHUNKS_LOADED_LIST;
    while node != 0 {
        let chunk = node_item(e, node);
        if chunk != 0 {
            let chunk_node = e.call(POINTER_GET, &args![chunk]).u32();
            if fn_006fa840(e, Ptr::new(chunk_node)) == 0 {
                let sp_node = fn_006fa820(e, Ptr::new(chunk));
                if sp_node != 0 {
                    update_node_at(e, sp_node, position, flag);
                }
                for offset in [0x14, 0x18] {
                    let child = e.call(POINTER_GET, &args![chunk + offset]).u32();
                    if child != 0 {
                        update_node_at(e, child, position, flag);
                    }
                }
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

/// The chain `006629f0`, `0043b230`, `00528820` on `target`: the last call
/// clears the word at `+8` of the object the first two find.
fn clear_through_chain(e: &mut Engine, target: u32) {
    let first = e.call(NODE_FIELD_AC, &args![target]).u32();
    let second = e.call(OBJECT_FIELD_0C, &args![first]).u32();
    e.call(OBJECT_CLEAR_FIELD_08, &args![second]);
}

// Translated from 006fa860 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every loaded chunk, runs the clearing chain on its `spNode`,
/// `spWaterNode` and `spWaterReflNode` (each when set).
pub fn fn_006fa860(e: &mut Engine) {
    let mut node = CHUNKS_LOADED_LIST;
    while node != 0 {
        let chunk = node_item(e, node);
        if chunk != 0 {
            let sp_node = fn_006fa820(e, Ptr::new(chunk));
            if sp_node != 0 {
                clear_through_chain(e, sp_node);
            }
            for offset in [0x14, 0x18] {
                let child = e.call(POINTER_GET, &args![chunk + offset]).u32();
                if child != 0 {
                    clear_through_chain(e, child);
                }
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

// Translated from 006fa920 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `mode` differs from the byte at `0119f154`, calls `006fb170` (mode
/// nonzero) or `006fb3d0` (mode 0) with `(chunk, 1)` for every loaded chunk
/// and then stores `mode` in the byte. A cdecl function.
pub fn fn_006fa920(e: &mut Engine, mode: u8) {
    if e.global::<u8>(MODE_BYTE) == mode {
        return;
    }
    let mut node = CHUNKS_LOADED_LIST;
    while node != 0 {
        let chunk = node_item(e, node);
        if chunk != 0 {
            if mode != 0 {
                e.call(CHUNK_CALL_006FB170, &args![chunk, 1u32]);
            } else {
                e.call(CHUNK_CALL_006FB3D0, &args![chunk, 1u32]);
            }
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    e.set_global(MODE_BYTE, mode);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006fa6a0, fn_006fa6a0(Ptr)),
        entry!(0x006fa6d0, fn_006fa6d0(u32, u32)),
        entry!(0x006fa820, fn_006fa820(Ptr) -> u32),
        entry!(0x006fa840, fn_006fa840(Ptr) -> u8),
        entry!(0x006fa860, fn_006fa860()),
        entry!(0x006fa920, fn_006fa920(u8)),
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

    /// An engine with the list pages mapped and the list accessors and the
    /// smart-pointer read doubled.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0119_f000, 0x011d_8000] {
            e.map(page, 0x1000);
        }
        e.register(LIST_ITEM_ADDRESS, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e
    }

    /// Builds a chunk object (`0x20` bytes) with `pNode`, `spNode`,
    /// `spWaterNode` and `spWaterReflNode`.
    fn chunk(e: &mut Engine, node: u32, parts: [u32; 3]) -> u32 {
        let chunk = e.mem.alloc(0x20);
        e.mem.set_u32(chunk, node);
        e.mem.set_u32(chunk + 0x10, parts[0]);
        e.mem.set_u32(chunk + 0x14, parts[1]);
        e.mem.set_u32(chunk + 0x18, parts[2]);
        chunk
    }

    /// Puts `items` in the list of loaded chunks (the head node is the
    /// global itself, further nodes are heap blocks).
    fn set_list(e: &mut Engine, items: &[u32]) {
        let mut node = CHUNKS_LOADED_LIST;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    /// A terrain node object whose byte at `+0x5d` is `split`.
    fn terrain_node(e: &mut Engine, split: u8) -> u32 {
        let node = e.mem.alloc(0x60);
        e.mem.set_u8(node + 0x5d, split);
        node
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
    fn removing_a_chunk_passes_the_address_of_a_local_holding_it() {
        let mut e = list_engine();
        e.register(LIST_REMOVE, |e, a| {
            // Records the list head and the word the argument points at.
            e.mem.set_u32(0x011d_8f00, a[0]);
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(0x011d_8f04, item);
            Ret::default()
        });
        e.call(0x006f_a6a0, &args![Ptr::<()>::new(0x2222)]);
        assert_eq!(e.mem.u32(0x011d_8f00), CHUNKS_LOADED_LIST);
        assert_eq!(e.mem.u32(0x011d_8f04), 0x2222);
    }

    #[test]
    fn node_pointer_and_split_flag_getters() {
        let mut e = list_engine();
        let chunk = chunk(&mut e, 0, [0x7001, 0, 0]);
        assert_eq!(
            e.call(0x006f_a820, &args![Ptr::<()>::new(chunk)]).u32(),
            0x7001
        );
        let node = terrain_node(&mut e, 1);
        assert_eq!(e.call(0x006f_a840, &args![Ptr::<()>::new(node)]).u8(), 1);
        let node = terrain_node(&mut e, 0);
        assert_eq!(e.call(0x006f_a840, &args![Ptr::<()>::new(node)]).u8(), 0);
    }

    /// A source object whose position words are `x`, `y`, `z` (at `+0x8c`).
    fn position_source(e: &mut Engine, words: [u32; 3]) -> u32 {
        let source = e.mem.alloc(0x98);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(source + 0x8c + 4 * i as u32, *word);
        }
        source
    }

    #[test]
    fn position_update_visits_the_parts_of_chunks_with_unsplit_nodes() {
        let mut e = list_engine();
        e.register(VECTOR_ADDRESS, |_, a| ret(a[0] + 0x8c));
        e.register(GLOBAL_TABLE_ENTRY, |_, _| ret(0xabc0));
        e.register(NODE_UPDATE_AT, |_, _| Ret::default());
        let plain = terrain_node(&mut e, 0);
        let split = terrain_node(&mut e, 1);
        // All three parts set, a split chunk, a chunk with only the water node.
        let all = chunk(&mut e, plain, [0x7001, 0x7002, 0x7003]);
        let skipped = chunk(&mut e, split, [0x7101, 0x7102, 0x7103]);
        let water_only = chunk(&mut e, plain, [0, 0x7202, 0]);
        set_list(&mut e, &[all, 0, skipped, water_only]);
        let source = position_source(&mut e, [11, 22, 33]);
        e.call_log = Some(vec![]);
        e.call(0x006f_a6d0, &args![source, 5u32]);
        assert_eq!(
            calls_to(&e, NODE_UPDATE_AT),
            vec![
                vec![0xabc0, 0x7001, 11, 22, 33, 5],
                vec![0xabc0, 0x7002, 11, 22, 33, 5],
                vec![0xabc0, 0x7003, 11, 22, 33, 5],
                vec![0xabc0, 0x7202, 11, 22, 33, 5],
            ]
        );
        assert_eq!(calls_to(&e, GLOBAL_TABLE_ENTRY), vec![vec![0]; 4]);
    }

    #[test]
    fn position_update_with_an_empty_list_does_nothing() {
        let mut e = list_engine();
        e.register(VECTOR_ADDRESS, |_, a| ret(a[0] + 0x8c));
        e.register(NODE_UPDATE_AT, |_, _| Ret::default());
        let source = position_source(&mut e, [1, 2, 3]);
        e.call_log = Some(vec![]);
        e.call(0x006f_a6d0, &args![source, 0u32]);
        assert!(calls_to(&e, NODE_UPDATE_AT).is_empty());
    }

    #[test]
    fn clearing_runs_the_chain_on_every_set_part_of_every_chunk() {
        let mut e = list_engine();
        e.register(NODE_FIELD_AC, |_, a| ret(a[0] + 1));
        e.register(OBJECT_FIELD_0C, |_, a| ret(a[0] + 1));
        e.register(OBJECT_CLEAR_FIELD_08, |_, _| Ret::default());
        let node = terrain_node(&mut e, 0);
        let all = chunk(&mut e, node, [0x100, 0x200, 0x300]);
        let split = terrain_node(&mut e, 1);
        // The clearing does not look at the split flag.
        let only_first = chunk(&mut e, split, [0x400, 0, 0]);
        let none = chunk(&mut e, node, [0, 0, 0]);
        set_list(&mut e, &[all, only_first, none]);
        e.call_log = Some(vec![]);
        e.call(0x006f_a860, &args![]);
        assert_eq!(
            calls_to(&e, OBJECT_CLEAR_FIELD_08),
            vec![vec![0x102], vec![0x202], vec![0x302], vec![0x402]]
        );
        assert_eq!(calls_to(&e, NODE_FIELD_AC).len(), 4);
    }

    #[test]
    fn mode_change_visits_every_chunk_and_stores_the_mode() {
        let mut e = list_engine();
        e.register(CHUNK_CALL_006FB170, |_, _| Ret::default());
        e.register(CHUNK_CALL_006FB3D0, |_, _| Ret::default());
        let node = terrain_node(&mut e, 0);
        let first = chunk(&mut e, node, [0, 0, 0]);
        let second = chunk(&mut e, node, [0, 0, 0]);
        set_list(&mut e, &[first, 0, second]);
        e.set_global(MODE_BYTE, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x006f_a920, &args![1u32]);
        assert_eq!(
            calls_to(&e, CHUNK_CALL_006FB170),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert!(calls_to(&e, CHUNK_CALL_006FB3D0).is_empty());
        assert_eq!(e.global::<u8>(MODE_BYTE), 1);
        // Back to 0: the other function runs.
        e.call_log = Some(vec![]);
        e.call(0x006f_a920, &args![0u32]);
        assert_eq!(
            calls_to(&e, CHUNK_CALL_006FB3D0),
            vec![vec![first, 1], vec![second, 1]]
        );
        assert_eq!(e.global::<u8>(MODE_BYTE), 0);
    }

    #[test]
    fn mode_change_to_the_same_byte_does_nothing_and_compares_only_the_low_byte() {
        let mut e = list_engine();
        e.register(CHUNK_CALL_006FB170, |_, _| Ret::default());
        e.register(CHUNK_CALL_006FB3D0, |_, _| Ret::default());
        let node = terrain_node(&mut e, 0);
        let first = chunk(&mut e, node, [0, 0, 0]);
        set_list(&mut e, &[first]);
        e.set_global(MODE_BYTE, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x006f_a920, &args![1u32]);
        // The caller's high bytes do not matter: only the byte is read.
        e.call(0x006f_a920, &args![0x101u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.global::<u8>(MODE_BYTE), 1);
    }
}
