//! `(unplaced)/run_00631180` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00559450` (unnamed in the engine map): `thiscall`, answers the dword at `this`.
const LIST_FIRST_NODE: u32 = 0x0055_9450;
/// `007b52d0` (`run_007b52a0.rs`): `thiscall`, `RET 4`; the next node (`*node`, null for null).
const LIST_NEXT_NODE: u32 = 0x007b_52d0;
/// `006317a0` (`run_006317a0.rs`): `thiscall`, `RET 4`; the address of the node's item
/// (`node + 8`).
const LIST_ITEM_SLOT: u32 = 0x0063_17a0;
/// `0062fc70` (unnamed in the engine map, `teswaterlistener.cpp` range): `thiscall`, `RET 4`;
/// takes a pair back (what it does is unconfirmed).
const PAIR_TAKE_BACK: u32 = 0x0062_fc70;
/// `0049c5d0` (unnamed in the engine map): `thiscall`, `RET 4`; unlinks the node the iterator
/// cell points to from the list and moves the cell to the next node.
const LIST_REMOVE_NODE: u32 = 0x0049_c5d0;
/// `00401030` (unnamed in the engine map): `cdecl`, one stack word; frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;
/// `00437c10` (unnamed in the engine map): `thiscall`; constructs two vectors (at +0 and +0x10)
/// with `006815c0` and returns `this`.
const VECTOR_PAIR_CONSTRUCTOR: u32 = 0x0043_7c10;
/// `00560d30` (unnamed in the engine map): `thiscall`, `RET 4`; the address of element `index`
/// of a dword array (`this + index * 4`).
const DWORD_ARRAY_ELEMENT: u32 = 0x0056_0d30;
/// `004a3e90` (unnamed in the engine map): `cdecl`, one `float` stack word; multiplies it by the
/// `float` at `011c582c` and answers it in `ST0`.
const FLOAT_SCALE: u32 = 0x004a_3e90;
/// The virtual slot (byte offset) of the pair's first object that says the pair is still
/// valid (zero: it is not).
const SLOT_IS_VALID: u32 = 0x1d0;
/// The virtual slot of the second object that fills the vector pair.
const SLOT_FILL_VECTORS: u32 = 0xf8;
/// The virtual slot of the third object that answers a pointer to a record with a float at +8.
const SLOT_RECORD: u32 = 0x1f4;

// Translated from 00631370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00631370 - the /GS stack-cookie check (00ec408c) at the end is not translated
/// Unnamed in the engine map. `thiscall`, `RET 0xc` (three stack words: a form pointer and two
/// flag bytes). Walks the list at `this + 0x34`; each node's item is a heap triple (`first`,
/// `second`, `third`). A triple whose first object is no longer valid (virtual slot +0x1d0 is
/// zero) is taken back (`0062fc70`), unlinked and freed. Otherwise it is selected when `matching`
/// is null or equals the triple's first or third pointer; when the first flag byte is set the
/// selection is replaced by "the scaled `float` at +8 of the record the third object answers is
/// below the `float` at index 2 of the vector pair the second object fills". A selected triple
/// is taken back first unless the second flag byte is set, then unlinked and freed. The next
/// node is fetched before a node is touched, and the loop test on the current node inside the
/// body can never fail (kept for fidelity).
pub fn fn_00631370(e: &mut Engine, this: Ptr, matching: u32, check_distance: u8, keep: u8) {
    let list = this.addr().wrapping_add(0x34);
    let mut node = e.call(LIST_FIRST_NODE, &args![list]).u32();
    while node != 0 {
        let next = e.call(LIST_NEXT_NODE, &args![list, node]).u32();
        if node != 0 {
            let item_slot = e.call(LIST_ITEM_SLOT, &args![list, node]).u32();
            let triple = e.mem.u32(item_slot);
            let first = e.mem.u32(triple);
            if e.vcall(first, SLOT_IS_VALID, &args![]).u32() == 0 {
                e.call(PAIR_TAKE_BACK, &args![this, triple]);
                unlink_node(e, list, node);
                e.call(FREE_BLOCK, &args![triple]);
            } else {
                let mut selected = matching == 0
                    || e.mem.u32(triple) == matching
                    || e.mem.u32(triple.wrapping_add(8)) == matching;
                if check_distance != 0 {
                    selected = below_limit(e, triple);
                }
                if selected {
                    if keep == 0 {
                        e.call(PAIR_TAKE_BACK, &args![this, triple]);
                    }
                    unlink_node(e, list, node);
                    e.call(FREE_BLOCK, &args![triple]);
                }
            }
        }
        node = next;
    }
}

/// `0049c5d0` on a node held in a one-word cell (the exe passes the address of its local).
fn unlink_node(e: &mut Engine, list: u32, node: u32) {
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), node);
        e.call(LIST_REMOVE_NODE, &args![list, cell]);
    });
}

/// The distance test of [`fn_00631370`]: fills the vector pair through the second object, takes
/// element 2 of it and compares it with `004a3e90` of the `float` at +8 of the third object's
/// record: true when the scaled value is strictly below it (a NaN on either side gives false,
/// as the exe's `FCOMPP` flag test does).
fn below_limit(e: &mut Engine, triple: u32) -> bool {
    let vectors = e.mem.alloc(0x20);
    e.call(VECTOR_PAIR_CONSTRUCTOR, &args![vectors]);
    let second = e.mem.u32(triple.wrapping_add(4));
    e.vcall(second, SLOT_FILL_VECTORS, &args![vectors]);
    let limit_slot = e.call(DWORD_ARRAY_ELEMENT, &args![vectors, 2u32]).u32();
    let third = e.mem.u32(triple.wrapping_add(8));
    let record = e.vcall(third, SLOT_RECORD, &args![]).u32();
    let value = x87_quiet_bits(e.mem.u32(record.wrapping_add(8)));
    let scaled = e.call(FLOAT_SCALE, &args![value]).f32();
    let limit = x87_quiet(e.mem.f32(limit_slot));
    e.mem.free(vectors);
    scaled < limit
}

/// `00822510` (`ECX` = the source, one stack word), in `magicshaderhiteffect.cpp`'s range;
/// answers a byte (`AL`) that, when nonzero, makes `00631540` do nothing.
const SOURCE_TEST_00822510: u32 = 0x0082_2510;
/// `0052aa80` (`ECX`, one stack word): whether the word at `this` differs from the value.
const WORD_DIFFERS_0052AA80: u32 = 0x0052_aa80;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `006317f0` (`ECX` = a new 8-byte block, one stack word): `00559a40(this, source)`, then a
/// zero word at +4; answers `this`.
const NODE_CTOR_006317F0: u32 = 0x0063_17f0;
/// `NiPointer<..>::operator=` (`006e5cc0`, `ECX` = the pointer, one stack word: the source
/// pointer's address).
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;

// Translated from 00631540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00631540 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eeb0fb, state variable) is not translated
/// Unnamed in the engine map. `thiscall`, `RET 4` (one stack word, `source`): nothing when
/// `00822510` on `source` (with 0) answers nonzero. Otherwise, when the word at `this` is
/// nonzero (`0052aa80` with 0), a new 8-byte node (`00401000`, made by `006317f0` from `this`,
/// null when the allocation fails) takes the word at `this + 4` into its +4 and becomes the
/// word at `this + 4`; in both cases `this` is then assigned `source` (`006e5cc0`).
pub fn fn_00631540(e: &mut Engine, this: Ptr, source: Ptr) {
    if e.call(SOURCE_TEST_00822510, &args![source, 0u32]).u8() != 0 {
        return;
    }
    if e.call(WORD_DIFFERS_0052AA80, &args![this, 0u32]).u8() != 0 {
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        let node = if block != 0 {
            e.call(NODE_CTOR_006317F0, &args![block, this]).u32()
        } else {
            0
        };
        let previous = e.mem.u32(this.addr() + 4);
        e.mem.set_u32(node + 4, previous);
        e.mem.set_u32(this.addr() + 4, node);
    }
    e.call(NI_POINTER_ASSIGN, &args![this, source]);
}

/// The `TES` singleton pointer; `0070ec90` (`ECX` = it): the object `004e59f0` is asked
/// of; `004e59f0` (`ECX` = that, three stack words).
const TES: u32 = 0x011d_ea10;
const TES_OBJECT_0070EC90: u32 = 0x0070_ec90;
const LOOKUP_004E59F0: u32 = 0x004e_59f0;
/// `WaterZone`'s vtable (its RTTI name) and its base constructor `0061fc10` (`ECX`, two
/// stack words).
const WATER_ZONE_VTABLE: u32 = 0x0104_e4b0;
const ZONE_BASE_CONSTRUCT: u32 = 0x0061_fc10;

// Translated from 00631180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00631180 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eeb0fb, state variable around the constructor) and the frame's cookie word are not translated.
/// A factory slot (the vtable slot at `0104e444`; unnamed in the engine map; `ECX` not
/// read, two stack words, `RET 8`): a new `WaterZone` (0x20 bytes from `00401000`,
/// [`fn_00631230`] with both words; null when the allocation fails) whose +0x1c gets
/// `004e59f0(reference, 0, 0.0)` on the `TES`'s `0070ec90` object. The decompiler hangs the
/// three words on `0070ec90`, which takes none (`RET`); `004e59f0` pops them (`RET 0xc`).
pub fn fn_00631180(e: &mut Engine, _this: Ptr, reference: u32, second: u32) -> u32 {
    let block = e.call(0x0040_1000, &args![0x20u32]).u32();
    let zone = if block == 0 {
        0
    } else {
        fn_00631230(e, Ptr::new(block), reference, second).addr()
    };
    let tes: u32 = e.global(TES);
    let object = e.call(TES_OBJECT_0070EC90, &args![tes]).u32();
    let value = e
        .call(
            LOOKUP_004E59F0,
            &args![object, reference, 0u32, 0.0f32.to_bits()],
        )
        .u32();
    e.mem.set_u32(zone + 0x1c, value);
    zone
}

// Translated from 00631230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00631230 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00efc7f8, state variable around the base constructor) and the frame's cookie word are not translated.
/// The `WaterZone` constructor (by its vtable's RTTI; unnamed in the engine map; `ECX` and
/// two stack words, `RET 8`): the base constructor `0061fc10` with both words, the vtable
/// `0104e4b0`, the `float` at +8 of `reference`'s vtable slot +0x1f4 answer (0.0 for a null
/// reference) at +0x18, and 0 at +0x1c. Answers `this`.
pub fn fn_00631230(e: &mut Engine, this: Ptr, reference: u32, second: u32) -> Ptr {
    e.call(ZONE_BASE_CONSTRUCT, &args![this, reference, second]);
    let base = this.addr();
    e.mem.set_u32(base, WATER_ZONE_VTABLE);
    let height = if reference != 0 {
        let data = e.vcall(reference, 0x1f4, &args![]).u32();
        x87_quiet_bits(e.mem.u32(data + 8))
    } else {
        0.0f32.to_bits()
    };
    e.mem.set_u32(base + 0x18, x87_quiet_bits(height));
    e.mem.set_u32(base + 0x1c, 0);
    this
}

// Translated from 006312d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`; called by `0056fcb0`):
/// with the object `0062fe80` (`cdecl`) answers for `key`, for each node of its list
/// (`00825c00`, next `0061fef0`) whose +0x10 word (`0084e3a0`) is a reference: while
/// `0042f130(object)` answers true on that reference's extra list (`005d43c0`), calls
/// `0042f030(object, 0)` on it. Then `006205e0(object)` on the part at +0x14 of `this`. The
/// decompiler hangs the object on `005d43c0`, which takes no stack word.
pub fn fn_006312d0(e: &mut Engine, this: Ptr, key: u32) {
    let object = e.call(0x0062_fe80, &args![key]).u32();
    if object == 0 {
        return;
    }
    let mut node = e.call(0x0082_5c00, &args![object]).u32();
    while node != 0 {
        let reference = e.call(0x0084_e3a0, &args![node + 0x10]).u32();
        if reference != 0 {
            loop {
                let extra = e.call(0x005d_43c0, &args![reference]).u32();
                if e.call(0x0042_f130, &args![extra, object]).u8() == 0 {
                    break;
                }
                let extra = e.call(0x005d_43c0, &args![reference]).u32();
                e.call(0x0042_f030, &args![extra, object, 0u32]);
            }
        }
        node = e.call(0x0061_fef0, &args![node]).u32();
    }
    e.call(0x0062_05e0, &args![this.addr() + 0x14, object]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_1180, fn_00631180(Ptr, u32, u32) -> u32),
        entry!(0x0063_1230, fn_00631230(Ptr, u32, u32) -> Ptr),
        entry!(0x0063_12d0, fn_006312d0(Ptr, u32)),
        entry!(0x0063_1370, fn_00631370(Ptr, u32, u8, u8)),
        entry!(0x0063_1540, fn_00631540(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod history_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn rig(test: u32, current: u32, block_ok: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register_double(SOURCE_TEST_00822510, move |_, _| test.into_ret());
        e.register(WORD_DIFFERS_0052AA80, |e, a| {
            (e.mem.u32(a[0]) != a[1]).into_ret()
        });
        let block = e.mem.alloc(8);
        e.register_double(ALLOCATE, move |_, _| {
            if block_ok { block } else { 0 }.into_ret()
        });
        e.register(NODE_CTOR_006317F0, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
        e.register(NI_POINTER_ASSIGN, |_, a| a[0].into_ret());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, current);
        e.mem.set_u32(this + 4, 0x0777);
        e.call_log = Some(vec![]);
        (e, this, block)
    }

    #[test]
    fn fn_00631540_keeps_the_old_value_in_a_node() {
        let (mut e, this, block) = rig(0, 0x1234, true);
        e.call(0x0063_1540, &args![this, 0x9000u32]);
        assert_eq!(calls(&e, SOURCE_TEST_00822510), vec![vec![0x9000, 0]]);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(calls(&e, NODE_CTOR_006317F0), vec![vec![block, this]]);
        assert_eq!(e.mem.u32(block), 0x1234);
        assert_eq!(e.mem.u32(block + 4), 0x0777);
        assert_eq!(e.mem.u32(this + 4), block);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN), vec![vec![this, 0x9000]]);
    }

    #[test]
    fn fn_00631540_without_a_value_only_assigns() {
        let (mut e, this, _) = rig(0, 0, true);
        e.call(0x0063_1540, &args![this, 0x9000u32]);
        assert!(calls(&e, ALLOCATE).is_empty());
        assert_eq!(e.mem.u32(this + 4), 0x0777);
        assert_eq!(calls(&e, NI_POINTER_ASSIGN), vec![vec![this, 0x9000]]);
    }

    #[test]
    fn fn_00631540_does_nothing_when_the_source_test_answers() {
        // Only the low byte counts.
        let (mut e, this, _) = rig(0x100, 0x1234, true);
        e.call(0x0063_1540, &args![this, 0x9000u32]);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
        let (mut e, this, _) = rig(1, 0x1234, true);
        e.call(0x0063_1540, &args![this, 0x9000u32]);
        assert!(calls(&e, ALLOCATE).is_empty());
        assert!(calls(&e, NI_POINTER_ASSIGN).is_empty());
    }

    #[test]
    fn fn_00631540_with_a_failed_allocation_writes_at_null() {
        let (mut e, this, _) = rig(0, 0x1234, false);
        e.map(0, 0x10);
        e.call(0x0063_1540, &args![this, 0x9000u32]);
        assert!(calls(&e, NODE_CTOR_006317F0).is_empty());
        assert_eq!(e.mem.u32(4), 0x0777);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Three objects per triple: the first is valid when its +8 is non-zero, the second fills
    /// the vector pair's element 2 from its +8, the third answers the record at its +8.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let mut valid = [0u32; 0x80];
        valid[SLOT_IS_VALID as usize / 4] = 0x0f00_b000;
        e.put_vtable(0x0f00_b100, &valid);
        let mut fill = [0u32; 0x80];
        fill[SLOT_FILL_VECTORS as usize / 4] = 0x0f00_b010;
        e.put_vtable(0x0f00_b600, &fill);
        let mut record = [0u32; 0x80];
        record[SLOT_RECORD as usize / 4] = 0x0f00_b020;
        e.put_vtable(0x0f00_bb00, &record);
        e.register(0x0f00_b000, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0f00_b010, |e, a| {
            let limit = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[1] + 8, limit);
            Ret::default()
        });
        e.register(0x0f00_b020, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(LIST_FIRST_NODE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[1]).into_ret());
        e.register(LIST_ITEM_SLOT, |_, a| (a[1] + 8).into_ret());
        e.register(PAIR_TAKE_BACK, |_, _| Ret::default());
        e.register(LIST_REMOVE_NODE, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(VECTOR_PAIR_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(DWORD_ARRAY_ELEMENT, |_, a| (a[0] + a[1] * 4).into_ret());
        // The scale doubles its argument.
        e.register(FLOAT_SCALE, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]) * 2.0),
            ..Ret::default()
        });
        e
    }

    struct Fixture {
        this: u32,
        triples: Vec<u32>,
        nodes: Vec<u32>,
    }

    /// A target with one node per entry of `specs` = (valid, limit bits, record float bits).
    fn target(e: &mut Engine, specs: &[(u32, u32, u32)]) -> Fixture {
        let this = e.mem.alloc(0x40);
        let mut triples = Vec::new();
        let mut nodes = Vec::new();
        for (valid, limit, record_value) in specs {
            let first = e.mem.alloc(0x10);
            e.mem.set_u32(first, 0x0f00_b100);
            e.mem.set_u32(first + 8, *valid);
            let second = e.mem.alloc(0x10);
            e.mem.set_u32(second, 0x0f00_b600);
            e.mem.set_u32(second + 8, *limit);
            let third = e.mem.alloc(0x10);
            e.mem.set_u32(third, 0x0f00_bb00);
            let record = e.mem.alloc(0x10);
            e.mem.set_u32(record + 8, *record_value);
            e.mem.set_u32(third + 8, record);
            let triple = e.mem.alloc(0xc);
            e.mem.set_u32(triple, first);
            e.mem.set_u32(triple + 4, second);
            e.mem.set_u32(triple + 8, third);
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node + 8, triple);
            triples.push(triple);
            nodes.push(node);
        }
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0], pair[1]);
        }
        if let Some(first) = nodes.first() {
            e.mem.set_u32(this + 0x34, *first);
        }
        Fixture {
            this,
            triples,
            nodes,
        }
    }

    #[test]
    fn at_00631370_takes_back_and_frees_triples_that_are_no_longer_valid() {
        let mut e = rig();
        let t = target(&mut e, &[(0, 0, 0), (1, 0, 0), (0, 0, 0)]);
        e.call(0x0063_1370, &args![t.this, 0x1234u32, 0u8, 0u8]);
        assert_eq!(
            calls(&e, PAIR_TAKE_BACK),
            vec![vec![t.this, t.triples[0]], vec![t.this, t.triples[2]]]
        );
        assert_eq!(
            calls(&e, FREE_BLOCK),
            vec![vec![t.triples[0]], vec![t.triples[2]]]
        );
        let removed: Vec<u32> = calls(&e, LIST_REMOVE_NODE).iter().map(|c| c[0]).collect();
        assert_eq!(removed, vec![t.this + 0x34, t.this + 0x34]);
        // The next node is fetched before each node is handled.
        let nexts = calls(&e, LIST_NEXT_NODE);
        assert_eq!(nexts.len(), 3);
        assert_eq!(nexts[0], vec![t.this + 0x34, t.nodes[0]]);
    }

    #[test]
    fn at_00631370_selects_by_the_first_or_third_pointer() {
        // Offset 0 is the triple's first pointer, offset 8 its third.
        for field in [0u32, 8] {
            let mut e = rig();
            let t = target(&mut e, &[(1, 0, 0), (1, 0, 0), (1, 0, 0)]);
            let wanted = e.mem.u32(t.triples[1] + field);
            e.call(0x0063_1370, &args![t.this, wanted, 0u8, 0u8]);
            assert_eq!(calls(&e, FREE_BLOCK), vec![vec![t.triples[1]]]);
            assert_eq!(calls(&e, PAIR_TAKE_BACK), vec![vec![t.this, t.triples[1]]]);
        }
    }

    #[test]
    fn at_00631370_a_null_match_selects_everything_and_keep_skips_the_take_back() {
        let mut e = rig();
        let t = target(&mut e, &[(1, 0, 0), (1, 0, 0)]);
        e.call(0x0063_1370, &args![t.this, 0u32, 0u8, 1u8]);
        assert!(calls(&e, PAIR_TAKE_BACK).is_empty());
        assert_eq!(
            calls(&e, FREE_BLOCK),
            vec![vec![t.triples[0]], vec![t.triples[1]]]
        );
        let mut e = rig();
        let t = target(&mut e, &[(1, 0, 0), (1, 0, 0)]);
        e.call(0x0063_1370, &args![t.this, 0u32, 0u8, 0u8]);
        assert_eq!(calls(&e, PAIR_TAKE_BACK).len(), 2);
    }

    #[test]
    fn at_00631370_the_distance_test_compares_the_scaled_value_with_the_limit() {
        let mut e = rig();
        // The scale doubles: record 1.0 -> 2.0. Limit 3.0: below, selected. Limit 2.0: equal,
        // not selected. Limit 1.0: above, not selected. NaN limit: not selected.
        let specs = [
            (1, 3.0f32.to_bits(), 1.0f32.to_bits()),
            (1, 2.0f32.to_bits(), 1.0f32.to_bits()),
            (1, 1.0f32.to_bits(), 1.0f32.to_bits()),
            (1, f32::NAN.to_bits(), 1.0f32.to_bits()),
        ];
        let t = target(&mut e, &specs);
        // With the distance flag the match pointer is ignored.
        e.call(0x0063_1370, &args![t.this, 0x7777u32, 1u8, 0u8]);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![t.triples[0]]]);
        assert_eq!(calls(&e, VECTOR_PAIR_CONSTRUCTOR).len(), 4);
        assert_eq!(calls(&e, FLOAT_SCALE).len(), 4);
    }

    #[test]
    fn at_00631370_a_nan_record_value_is_not_below_any_limit() {
        let mut e = rig();
        let t = target(&mut e, &[(1, 3.0f32.to_bits(), f32::NAN.to_bits())]);
        e.call(0x0063_1370, &args![t.this, 0u32, 1u8, 0u8]);
        assert!(calls(&e, FREE_BLOCK).is_empty());
    }

    #[test]
    fn at_00631370_an_empty_list_does_nothing() {
        let mut e = rig();
        let this = e.mem.alloc(0x40);
        e.call(0x0063_1370, &args![this, 0u32, 0u8, 0u8]);
        assert_eq!(calls(&e, LIST_FIRST_NODE), vec![vec![this + 0x34]]);
        assert!(calls(&e, LIST_NEXT_NODE).is_empty());
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A reference whose vtable slot +0x1f4 answers data holding 3.5 at +8.
    fn reference(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data + 8, 3.5f32.to_bits());
        e.register_double(0x0060_01f4, move |_, _| data.into_ret());
        let mut slots = vec![0u32; 0x7e];
        slots[0x1f4 / 4] = 0x0060_01f4;
        e.put_vtable(0x0070_0000, &slots);
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, 0x0070_0000);
        reference
    }

    #[test]
    fn fn_00631230_builds_the_water_zone() {
        let mut e = Engine::new();
        e.register(ZONE_BASE_CONSTRUCT, |_, _| Ret::default());
        let reference = reference(&mut e);
        let zone = e.mem.alloc(0x20);
        e.mem.set_u32(zone + 0x1c, 9);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0063_1230, &args![zone, reference, 5u32]).u32(),
            zone
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ZONE_BASE_CONSTRUCT),
            vec![vec![zone, reference, 5]]
        );
        assert_eq!(e.mem.u32(zone), WATER_ZONE_VTABLE);
        assert_eq!(e.mem.f32(zone + 0x18), 3.5);
        assert_eq!(e.mem.u32(zone + 0x1c), 0);
        e.call(0x0063_1230, &args![zone, 0u32, 5u32]);
        assert_eq!(e.mem.f32(zone + 0x18), 0.0);
    }

    #[test]
    fn fn_00631180_makes_a_zone_and_looks_its_word_up() {
        let mut e = Engine::new();
        e.register(ZONE_BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(0x0040_1000, |e, _| e.mem.alloc(0x20).into_ret());
        e.map(0x011d_e000, 0x1000);
        e.set_global(TES, 0x9000u32);
        e.register(TES_OBJECT_0070EC90, |_, _| 0x9100u32.into_ret());
        e.register(LOOKUP_004E59F0, |_, _| 0x4242u32.into_ret());
        let reference = reference(&mut e);
        e.call_log = Some(vec![]);
        let zone = e
            .call(0x0063_1180, &args![0x1000u32, reference, 5u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, TES_OBJECT_0070EC90), vec![vec![0x9000]]);
        assert_eq!(
            calls(&log, LOOKUP_004E59F0),
            vec![vec![0x9100, reference, 0, 0]]
        );
        assert_eq!(e.mem.u32(zone + 0x1c), 0x4242);
        assert_eq!(e.mem.u32(zone), WATER_ZONE_VTABLE);
    }

    #[test]
    fn fn_006312d0_clears_each_reference_then_the_object() {
        let mut e = Engine::new();
        e.register(0x0062_fe80, |_, a| {
            (if a[0] == 7 { 0x5000u32 } else { 0 }).into_ret()
        });
        // Two nodes; the first holds a reference, the second none.
        let second = e.mem.alloc(0x20);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 0x10, 0x6000);
        e.register_double(0x0082_5c00, move |_, _| first.into_ret());
        e.register(0x0061_fef0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0084_e3a0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x005d_43c0, |_, a| (a[0] + 0x44).into_ret());
        // The extra list holds the object twice.
        let left = std::rc::Rc::new(std::cell::Cell::new(2u32));
        let seen = left.clone();
        e.register_double(0x0042_f130, move |_, _| {
            u32::from(seen.get() > 0).into_ret()
        });
        e.register_double(0x0042_f030, move |_, _| {
            left.set(left.get() - 1);
            Ret::default()
        });
        e.register(0x0062_05e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0063_12d0, &args![0x1000u32, 7u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0042_f030), vec![vec![0x6044, 0x5000, 0]; 2]);
        assert_eq!(calls(&log, 0x0042_f130).len(), 3);
        assert_eq!(calls(&log, 0x0062_05e0), vec![vec![0x1014, 0x5000]]);
        e.call_log = Some(vec![]);
        e.call(0x0063_12d0, &args![0x1000u32, 8u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
}
