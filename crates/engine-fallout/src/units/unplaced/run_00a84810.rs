//! `(unplaced)/run_00a84810` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_4810, fn_00a84810(u32, u32, u32) -> u32),
        entry!(0x00a8_4c60, fn_00a84c60(u32) -> u32),
        entry!(0x00a8_4980, fn_00a84980(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> bool),
    ]
}

/// The memory manager object and its allocation method (thiscall: the
/// size) and free method (thiscall: the block).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// The compiler's `vector constructor iterator` (`00401050`, stdcall: the
/// array, the item size, the count, the constructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// The constructor the iterator runs on each 12-byte list (`00a833a0`; Ghidra
/// made no function of it).
const WEIGHT_LIST_CONSTRUCTOR: u32 = 0x00a8_33a0;
/// Grows a list to a capacity (`00a826b0`, thiscall) and the step run twice
/// on each finished list (`00a820e0`, thiscall), both placed in
/// `NiMain/niskinpartition.cpp` by the engine map.
const WEIGHT_LIST_RESERVE: u32 = 0x00a8_26b0;
const WEIGHT_LIST_FINISH: u32 = 0x00a8_20e0;
/// Frees a block through the Ni allocator (`00aa10f0`, cdecl; null is
/// ignored).
const NI_FREE: u32 = 0x00aa_10f0;

// Translated from 00a84810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the per-vertex bone weight lists of a skin (thiscall, `this` not
/// read, `RET 8`; called by `00a84980`; the engine map has no name): an
/// array of `count` 12-byte lists (data, capacity, size of 8-byte entries)
/// after a count word, from the memory manager (the size saturating at all
/// ones on overflow as the code computes it) and constructed by `00a833a0`;
/// then for each bone `i` of `skin` (+0x44 bones, +0x40 their 0x4C-byte
/// records; a record's weights at +0x44, their 16-bit count at +0x48, each
/// 8 bytes: a 16-bit vertex index and a float weight) appends `(i, weight)`
/// to the vertex's list, doubling its capacity (1 when empty, `00a826b0`)
/// when full. Each list is then finished twice (`00a820e0`) and the array
/// answered; when a list is empty at that point, every list's data is freed
/// (`00aa10f0`, last first), the block freed, and 0 answered.
pub fn fn_00a84810(e: &mut Engine, _this: u32, count: u32, skin: u32) -> u32 {
    let product = u64::from(count) * 12;
    let size = if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    };
    let size = size.checked_add(4).unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, size]).u32();
    let lists = if block != 0 {
        e.mem.set_u32(block, count);
        let lists = block + 4;
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &[lists, 12, count, WEIGHT_LIST_CONSTRUCTOR],
        );
        lists
    } else {
        0
    };
    let bones = e.mem.u32(skin + 0x44);
    let mut bone = 0u32;
    while bone < bones {
        let record = e.mem.u32(skin + 0x40).wrapping_add(bone.wrapping_mul(0x4c));
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(record + 0x48)) {
            let weights = e.mem.u32(record + 0x44);
            let entry = weights.wrapping_add(index * 8);
            let weight = x87_quiet_bits(e.mem.u32(entry + 4));
            let vertex = u32::from(e.mem.u16(entry));
            let list = lists.wrapping_add(vertex * 12);
            let capacity = e.mem.u32(list + 4);
            if e.mem.u32(list + 8) == capacity {
                let grown = if capacity > 0 {
                    capacity.wrapping_add(capacity)
                } else {
                    1
                };
                e.call(WEIGHT_LIST_RESERVE, &[list, grown]);
            }
            let data = e.mem.u32(list);
            let size = e.mem.u32(list + 8);
            e.mem.set_u32(data.wrapping_add(size * 8), bone);
            e.mem.set_u32(data.wrapping_add(size * 8) + 4, weight);
            e.mem.set_u32(list + 8, size.wrapping_add(1));
            index += 1;
        }
        bone += 1;
    }
    let mut vertex = 0u32;
    while vertex < count {
        let list = lists.wrapping_add(vertex * 12);
        if e.mem.u32(list + 8) == 0 {
            if lists != 0 {
                let made = e.mem.u32(lists.wrapping_sub(4)) as i32;
                let mut at = made - 1;
                while at >= 0 {
                    let data = e.mem.u32(lists + at as u32 * 12);
                    e.call(NI_FREE, &[data]);
                    at -= 1;
                }
                e.call(MEMORY_FREE, &[MEMORY_MANAGER, lists - 4]);
            }
            return 0;
        }
        e.call(WEIGHT_LIST_FINISH, &[list]);
        e.call(WEIGHT_LIST_FINISH, &[list]);
        vertex += 1;
    }
    lists
}

// Translated from 00a84c60 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Clears the 16-bit value at `this` and answers `this` (thiscall; passed
/// as an item constructor by `00683c10` and `00685ef0`; Ghidra made no
/// function of it).
pub fn fn_00a84c60(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u16(this, 0);
    this
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    fn weight_engine() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_FREE, |_, _| 0u32.into_ret());
        e.register(NI_FREE, |_, _| 0u32.into_ret());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |e, a| {
            for i in 0..a[2] {
                for w in 0..3 {
                    e.mem.set_u32(a[0] + i * a[1] + 4 * w, 0);
                }
            }
            0u32.into_ret()
        });
        e.register(WEIGHT_LIST_RESERVE, |e, a| {
            let data = e.mem.alloc(a[1] * 8);
            let old = e.mem.u32(a[0]);
            let size = e.mem.u32(a[0] + 8);
            if size != 0 {
                let bytes = e.mem.bytes(old, size * 8);
                e.mem.write(data, &bytes);
            }
            e.mem.set_u32(a[0], data);
            e.mem.set_u32(a[0] + 4, a[1]);
            0u32.into_ret()
        });
        e.register(WEIGHT_LIST_FINISH, |_, _| 0u32.into_ret());
        e
    }

    /// A skin of two bones over vertices 0 and 1: bone 0 weighs vertex 0 and
    /// 1, bone 1 weighs vertex 1 (and vertex 2 when `third`).
    fn skin(e: &mut Engine, third: bool) -> u32 {
        let skin = e.mem.alloc(0x48);
        let records = e.mem.alloc(2 * 0x4c);
        e.mem.set_u32(skin + 0x40, records);
        e.mem.set_u32(skin + 0x44, 2);
        let weights = [vec![(0u16, 0.25f32), (1, 0.5)], vec![(1, 0.5), (2, 1.0)]];
        for (bone, list) in weights.iter().enumerate() {
            let record = records + 0x4c * bone as u32;
            let n = if bone == 1 && !third { 1 } else { list.len() };
            let data = e.mem.alloc(16);
            for (i, (vertex, weight)) in list.iter().enumerate() {
                e.mem.set_u16(data + 8 * i as u32, *vertex);
                e.mem.set_u32(data + 8 * i as u32 + 4, weight.to_bits());
            }
            e.mem.set_u32(record + 0x44, data);
            e.mem.set_u16(record + 0x48, n as u16);
        }
        skin
    }

    #[test]
    fn weights_are_gathered_per_vertex() {
        let mut e = weight_engine();
        let skin = skin(&mut e, false);
        e.call_log = Some(vec![]);
        let lists = e.call(0x00a8_4810, &[0, 2, skin]).u32();
        assert_ne!(lists, 0);
        assert_eq!(e.mem.u32(lists - 4), 2);
        let entries = |e: &Engine, v: u32| -> Vec<(u32, f32)> {
            let list = lists + 12 * v;
            (0..e.mem.u32(list + 8))
                .map(|i| {
                    (
                        e.mem.u32(e.mem.u32(list) + 8 * i),
                        e.mem.f32(e.mem.u32(list) + 8 * i + 4),
                    )
                })
                .collect()
        };
        assert_eq!(entries(&e, 0), vec![(0, 0.25)]);
        assert_eq!(entries(&e, 1), vec![(0, 0.5), (1, 0.5)]);
        assert_eq!(e.mem.u32(lists + 12 + 4), 2);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MEMORY_ALLOCATE, vec![MEMORY_MANAGER, 28])));
        assert_eq!(
            log.iter().filter(|(a, _)| *a == WEIGHT_LIST_FINISH).count(),
            4
        );
    }

    #[test]
    fn an_unweighted_vertex_frees_everything() {
        let mut e = weight_engine();
        let skin = skin(&mut e, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_4810, &[0, 3, skin]).u32(), 0);
        let log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|(a, _)| *a == NI_FREE).collect();
        assert_eq!(frees.len(), 3);
        // The last list (vertex 2, never grown) is freed first.
        assert_eq!(frees[0].1, vec![0]);
        assert_eq!(log.iter().filter(|(a, _)| *a == MEMORY_FREE).count(), 1);
    }

    #[test]
    fn item_constructor_clears_a_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0xffff_ffff);
        assert_eq!(e.call(0x00a8_4c60, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0xffff_0000);
    }
}

/// The partitioning steps `00a84980` runs, all thiscall and placed in
/// `NiMain/niskinpartition.cpp` by the engine map: `00a833b0` (four words),
/// `00a84310` (seven words), `00a82190` (one word; answers in AL and leaves
/// ECX unchanged), `00a82d30` (three words) and `00a828d0` (eight words).
const PARTITION_PREPARE: u32 = 0x00a8_33b0;
const PARTITION_GROUP: u32 = 0x00a8_4310;
const PARTITION_SAME_BONES: u32 = 0x00a8_2190;
const PARTITION_COLLECT: u32 = 0x00a8_2d30;
const PARTITION_BUILD: u32 = 0x00a8_28d0;
/// The constructor the iterator runs on each 0x2C-byte partition
/// (`00a81ba0`; Ghidra made no function of it).
const PARTITION_CONSTRUCTOR: u32 = 0x00a8_1ba0;
/// `memcpy` (CRT, cdecl).
const MEMCPY: u32 = 0x00ec_44d0;
/// The size of a partition.
const PARTITION_SIZE: u32 = 0x2c;

/// The memory manager's allocation size for `count` items of `size` bytes
/// after a count word, saturating at all ones as the code computes it.
fn counted_size(count: u32, size: u32) -> u32 {
    let product = u64::from(count) * u64::from(size);
    let total = if product > u64::from(u32::MAX) {
        u32::MAX
    } else {
        product as u32
    };
    total.checked_add(4).unwrap_or(u32::MAX)
}

/// Frees the weight lists [`fn_00a84810`] made: each list's data, last
/// first (`00aa10f0`), then the block.
fn free_weight_lists(e: &mut Engine, lists: u32) {
    let made = e.mem.u32(lists.wrapping_sub(4)) as i32;
    let mut at = made - 1;
    while at >= 0 {
        let data = e.mem.u32(lists + at as u32 * 12);
        e.call(NI_FREE, &[data]);
        at -= 1;
    }
    e.call(MEMORY_FREE, &[MEMORY_MANAGER, lists.wrapping_sub(4)]);
}

// Translated from 00a84980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splits a skin into partitions (thiscall, eight words; called by
/// `00687d90` and `00c1d170`; the engine map has no name): builds the
/// per-vertex weight lists of `skin` for the geometry's vertex count (the
/// 16-bit word at `geometry` +8, [`fn_00a84810`]); without them frees
/// nothing more and answers false. Otherwise answers `00a833b0(geometry,
/// first, second, lists)`, and when that is true goes on: `00a84310`
/// (`geometry`, `skin`, `first`, the lists, a local list, a local word,
/// `entries`) answers instead; with `entries`, `words` and `flags` all set,
/// for each of the local list's items `words[i]` gets the item's 16-bit
/// +0xC and `flags[i]` 1 for the first item and for an item whose bones
/// differ from the previous one's (`00a82190` false, or the words at +8
/// different), else 0. Then `this` +8 partitions of 0x2C bytes (from the
/// memory manager, constructed by `00a81ba0`), each gathered (`00a82d30`)
/// and built (`00a828d0`, given whether the two bytes `first` and `second`
/// differ in `first`'s argument slot) from its item, which is then freed;
/// the local word freed; partitions whose 16-bit +0x1C is 0 removed (the
/// later ones moved down with `memcpy`, and `entries`, `words`, `flags`
/// with them), the array kept at `this` +0xC and the local list freed.
/// In every case after the weight lists exist, they and the local list's
/// data are freed before answering.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a84980(
    e: &mut Engine,
    this: u32,
    geometry: u32,
    skin: u32,
    first: u32,
    second: u32,
    fifth: u32,
    entries: u32,
    words: u32,
    flags: u32,
) -> bool {
    e.with_stack(0x28, |e, frame| {
        // The exe's locals: the item list (data, ?, count) at +0x10, the
        // word at +0x0C and the gather list at +0x1C.
        let item_list = frame.addr() + 0x10;
        let word = frame.addr() + 0xc;
        let gather = frame.addr() + 0x1c;
        for offset in [0u32, 4, 8] {
            e.mem.set_u32(item_list + offset, 0);
        }
        let vertices = u32::from(e.mem.u16(geometry + 8));
        let lists = fn_00a84810(e, this, vertices, skin);
        if lists == 0 {
            e.call(NI_FREE, &[lists]);
            return false;
        }
        let mut answer = e
            .call(PARTITION_PREPARE, &[this, geometry, first, second, lists])
            .bool();
        if answer {
            answer = e
                .call(
                    PARTITION_GROUP,
                    &[this, geometry, skin, first, lists, item_list, word, entries],
                )
                .bool();
            if entries != 0 && words != 0 && flags != 0 {
                let mut i = 0u32;
                while i < e.mem.u32(item_list + 8) {
                    let items = e.mem.u32(item_list);
                    let item = e.mem.u32(items + 4 * i);
                    let value = e.mem.u16(item + 0xc);
                    e.mem.set_u16(words + 2 * i, value);
                    if i == 0 {
                        e.mem.set_u8(flags, 1);
                    } else {
                        let previous = e.mem.u32(items + 4 * (i - 1));
                        let same = e.call(PARTITION_SAME_BONES, &[item, previous]).bool();
                        // ECX still holds `item` after the call.
                        let changed = !same || e.mem.u32(item + 8) != e.mem.u32(previous + 8);
                        e.mem.set_u8(flags + i, u8::from(changed));
                    }
                    i += 1;
                }
            }
            let count = e.mem.u32(this + 8);
            let block = e
                .call(
                    MEMORY_ALLOCATE,
                    &[MEMORY_MANAGER, counted_size(count, PARTITION_SIZE)],
                )
                .u32();
            let partitions = if block != 0 {
                e.mem.set_u32(block, count);
                e.call(
                    VECTOR_CONSTRUCTOR_ITERATOR,
                    &[block + 4, PARTITION_SIZE, count, PARTITION_CONSTRUCTOR],
                );
                block + 4
            } else {
                0
            };
            // `first`'s argument slot gets whether the two bytes differ.
            let differ = u32::from(first as u8 != second as u8);
            let first_slot = (first & !0xff) | differ;
            for offset in [0u32, 4, 8] {
                e.mem.set_u32(gather + offset, 0);
            }
            let mut index = 0u32;
            let mut partition = partitions;
            while index < e.mem.u32(this + 8) {
                let bones = u32::from(e.mem.u16(geometry + 0x40));
                let collected = e.mem.u32(word);
                e.call(PARTITION_COLLECT, &[gather, index, collected, bones]);
                let skin_word = e.mem.u32(skin + 0x44);
                let item = e.mem.u32(e.mem.u32(item_list) + 4 * index);
                e.call(
                    PARTITION_BUILD,
                    &[
                        partition, gather, item, geometry, lists, first_slot, fifth, second,
                        skin_word,
                    ],
                );
                e.mem.set_u32(gather + 8, 0);
                if item != 0 {
                    let data = e.mem.u32(item);
                    e.call(NI_FREE, &[data]);
                    e.call(MEMORY_FREE, &[MEMORY_MANAGER, item]);
                }
                index += 1;
                partition = partition.wrapping_add(PARTITION_SIZE);
            }
            let collected = e.mem.u32(word);
            e.call(NI_FREE, &[collected]);
            let mut count = e.mem.u32(this + 8);
            if count != 0 {
                loop {
                    let mut found = 0u32;
                    if count != 0 {
                        while found < e.mem.u32(this + 8)
                            && e.mem.u16(partitions + found * PARTITION_SIZE + 0x1c) != 0
                        {
                            found += 1;
                        }
                        if found < e.mem.u32(this + 8) {
                            let last = count - 1;
                            let mut at = found;
                            while at < last {
                                let to = partitions + at * PARTITION_SIZE;
                                e.call(MEMCPY, &[to, to + PARTITION_SIZE, PARTITION_SIZE]);
                                let next = e.mem.u32(entries + 4 * (at + 1));
                                e.mem.set_u32(entries + 4 * at, next);
                                let next = e.mem.u16(words + 2 * (at + 1));
                                e.mem.set_u16(words + 2 * at, next);
                                let next = e.mem.u8(flags + at + 1);
                                e.mem.set_u8(flags + at, next);
                                at += 1;
                            }
                            let shrunk = e.mem.u32(this + 8).wrapping_sub(1);
                            e.mem.set_u32(this + 8, shrunk);
                        }
                    }
                    count = e.mem.u32(this + 8);
                    if found == count || count == 0 {
                        break;
                    }
                }
            }
            let gathered = e.mem.u32(gather);
            e.mem.set_u32(this + 0xc, partitions);
            e.call(NI_FREE, &[gathered]);
        }
        free_weight_lists(e, lists);
        let items = e.mem.u32(item_list);
        e.call(NI_FREE, &[items]);
        answer
    })
}

#[cfg(test)]
mod n079_partition_tests {
    use super::*;

    fn partition_engine(prepared: bool) -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_FREE, |_, _| 0u32.into_ret());
        e.register(NI_FREE, |_, _| 0u32.into_ret());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| 0u32.into_ret());
        e.register(WEIGHT_LIST_RESERVE, |e, a| {
            let data = e.mem.alloc(a[1] * 8);
            e.mem.set_u32(a[0], data);
            e.mem.set_u32(a[0] + 4, a[1]);
            0u32.into_ret()
        });
        e.register(WEIGHT_LIST_FINISH, |_, _| 0u32.into_ret());
        e.register_double(PARTITION_PREPARE, move |_, _| {
            u32::from(prepared).into_ret()
        });
        // Two items: the second has other bones than the first.
        e.register(PARTITION_GROUP, |e, a| {
            let items = e.mem.alloc(8);
            for i in 0..2u32 {
                let item = e.mem.alloc(0x10);
                e.mem.set_u16(item + 0xc, 0x30 + i as u16);
                e.mem.set_u32(item + 8, i);
                e.mem.set_u32(items + 4 * i, item);
            }
            e.mem.set_u32(a[5], items);
            e.mem.set_u32(a[5] + 8, 2);
            e.mem.set_u32(a[6], 0x6600);
            0x101u32.into_ret()
        });
        e.register(PARTITION_SAME_BONES, |_, _| 1u32.into_ret());
        e.register(PARTITION_COLLECT, |_, _| 0u32.into_ret());
        // The first partition gets a non-zero +0x1C, the second stays empty.
        e.register(PARTITION_BUILD, |e, a| {
            let index = e.mem.u32(a[2] + 8);
            e.mem.set_u16(a[0] + 0x1c, if index == 0 { 3 } else { 0 });
            0u32.into_ret()
        });
        e.register(MEMCPY, |_, a| a[0].into_ret());
        e
    }

    /// A geometry of one vertex weighted by one bone of `skin`.
    fn inputs(e: &mut Engine) -> (u32, u32) {
        let geometry = e.mem.alloc(0x44);
        e.mem.set_u16(geometry + 8, 1);
        e.mem.set_u16(geometry + 0x40, 4);
        let skin = e.mem.alloc(0x48);
        let record = e.mem.alloc(0x4c);
        let weights = e.mem.alloc(8);
        e.mem.set_u32(weights + 4, 1.0f32.to_bits());
        e.mem.set_u32(record + 0x44, weights);
        e.mem.set_u16(record + 0x48, 1);
        e.mem.set_u32(skin + 0x40, record);
        e.mem.set_u32(skin + 0x44, 1);
        (geometry, skin)
    }

    #[test]
    fn partitions_are_built_and_empty_ones_dropped() {
        let mut e = partition_engine(true);
        let (geometry, skin) = inputs(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 2);
        let entries = e.mem.alloc(8);
        let words = e.mem.alloc(4);
        let flags = e.mem.alloc(2);
        e.call_log = Some(vec![]);
        let answer = e
            .call(
                0x00a8_4980,
                &[
                    this,
                    geometry,
                    skin,
                    0x1234_5602,
                    0x03,
                    7,
                    entries,
                    words,
                    flags,
                ],
            )
            .bool();
        assert!(answer);
        assert_eq!((e.mem.u16(words), e.mem.u16(words + 2)), (0x30, 0x31));
        assert_eq!((e.mem.u8(flags), e.mem.u8(flags + 1)), (1, 1));
        assert_eq!(e.mem.u32(this + 8), 1);
        let partitions = e.mem.u32(this + 0xc);
        assert_ne!(partitions, 0);
        let log = e.call_log.take().unwrap();
        let builds: Vec<_> = log.iter().filter(|(a, _)| *a == PARTITION_BUILD).collect();
        assert_eq!(builds.len(), 2);
        assert_eq!(builds[0].1[0], partitions);
        assert_eq!(builds[1].1[0], partitions + PARTITION_SIZE);
        // `first`'s slot: its upper bytes and whether the low bytes differ.
        assert_eq!(builds[0].1[5..], [0x1234_5601, 7, 3, 1]);
        let collects: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == PARTITION_COLLECT)
            .collect();
        assert_eq!(collects[1].1[1..], [1, 0x6600, 4]);
        assert!(log.contains(&(NI_FREE, vec![0x6600])));
    }

    #[test]
    fn a_failed_preparation_frees_and_answers_false() {
        let mut e = partition_engine(false);
        let (geometry, skin) = inputs(&mut e);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00a8_4980, &[this, geometry, skin, 0, 0, 0, 0, 0, 0])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != PARTITION_GROUP));
        assert_eq!(log.last().unwrap(), &(NI_FREE, vec![0]));
        // No weight lists at all: false straight away.
        let mut e = partition_engine(true);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        let geometry = e.mem.alloc(0x44);
        let bare_skin = e.mem.alloc(0x48);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x00a8_4980, &[this, geometry, bare_skin, 0, 0, 0, 0, 0, 0])
            .bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != PARTITION_PREPARE));
        assert_eq!(log.last().unwrap(), &(NI_FREE, vec![0]));
    }
}
