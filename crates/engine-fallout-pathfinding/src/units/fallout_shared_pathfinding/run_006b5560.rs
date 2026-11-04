//! `fallout shared/pathfinding/run_006b5560` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_006b5a90` calls on the member (`0044ddc0`).
const CALLEE_0044DDC0: u32 = 0x0044_ddc0;

// Translated from 006b5a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0044ddc0` with `ECX` = `this + 0x58` and passes on its result.
pub fn fn_006b5a90(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0044DDC0, &args![this.byte_add(0x58)]).u32()
}

// Translated from 006b5ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b5ab0 - the exception-unwinding frame (FS:[0] chain, handler 00f009ab, state variable) is not translated.
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): copies this object into
/// `destination`. The words at +0x0 to +0x1c and +0x54 are copied directly; the three
/// arrays at +0x24, +0x34 and +0x44 through `006e5330` (`BSSimpleArray<unsigned int>`'s
/// `CopyTo`: the source array as `this`, the destination array as the argument). For the
/// object at +0x58: when this one has one, the destination gets one if it has none (0x38
/// bytes from `00401000`, constructed by `0069bcf0`, or 0 when the allocation failed) and
/// `0069cc80` copies this one's into it (`ECX` this one's, the argument the
/// destination's); when this one has none, the destination's (if any) is destroyed and
/// freed through `006b4850` with 1 and the pointer cleared.
pub fn fn_006b5ab0(e: &mut Engine, this: Ptr, destination: Ptr) {
    let source = this.addr();
    let target = destination.addr();
    for offset in [0x0u32, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x54] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(target + offset, word);
    }
    for offset in [0x24u32, 0x34, 0x44] {
        e.call(0x006e_5330, &args![source + offset, target + offset]);
    }
    let own = e.mem.u32(source + 0x58);
    if own != 0 {
        if e.mem.u32(target + 0x58) == 0 {
            let block = e.call(0x0040_1000, &args![0x38u32]).u32();
            let created = if block == 0 {
                0
            } else {
                e.call(0x0069_bcf0, &args![block]).u32()
            };
            e.mem.set_u32(target + 0x58, created);
        }
        let own = e.mem.u32(source + 0x58);
        let theirs = e.mem.u32(target + 0x58);
        e.call(0x0069_cc80, &args![own, theirs]);
    } else {
        let theirs = e.mem.u32(target + 0x58);
        if theirs != 0 {
            e.call(0x006b_4850, &args![theirs, 1u32]);
        }
        e.mem.set_u32(target + 0x58, 0);
    }
}

/// The callees of `006b5560` (`thiscall` unless noted; the engine map's
/// names where it has one).
mod info_build {
    /// `0084e3a0`: the word at the object's `+0xc`.
    pub const WORD_AT_C: u32 = 0x0084_e3a0;
    /// `NavMesh::GetParentSpace` (engine map, `0068ef70`).
    pub const NAV_MESH_GET_PARENT_SPACE: u32 = 0x0068_ef70;
    /// `008454f0` (`RET 4`): run on each of the three arrays with 1.
    pub const ARRAY_RESET: u32 = 0x0084_54f0;
    /// `006b4850`: the scalar deleting destructor of the object at `+0x58`.
    pub const EXTRA_DELETE: u32 = 0x006b_4850;
    /// `00440da0`: a byte of the navmesh.
    pub const NAV_MESH_FLAG_00440DA0: u32 = 0x0044_0da0;
    /// `00693ef0` (`RET 4`): run on `this` with 1 when that byte is set.
    pub const INFO_00693EF0: u32 = 0x0069_3ef0;
    /// `00558200`: a count of the navmesh (its `0044ddc0`).
    pub const COUNT_00558200: u32 = 0x0055_8200;
    /// `005582f0` (`RET 4`, a 16-bit index): one of those items.
    pub const ITEM_005582F0: u32 = 0x0055_82f0;
    /// `00690e10`: a byte of the item.
    pub const ITEM_FLAG_00690E10: u32 = 0x0069_0e10;
    /// `00693b70`: another byte of the item.
    pub const ITEM_FLAG_00693B70: u32 = 0x0069_3b70;
    /// `0068f1d0` (`RET 4`, 0 to 2): whether that bit of the item's `+0xc` is set.
    pub const ITEM_BIT_0068F1D0: u32 = 0x0068_f1d0;
    /// `0068f2c0` (`RET 4`, 0 to 2): the 16-bit word at the item's `+6 + 2 * n`.
    pub const ITEM_LINK_0068F2C0: u32 = 0x0068_f2c0;
    /// `00693d30` (`RET 4`, a 16-bit index): an entry of the navmesh, whose
    /// `+4` is a reference (`0069e090` tests it).
    pub const NAV_MESH_ENTRY_00693D30: u32 = 0x0069_3d30;
    /// `0069e090`: whether the reference is set.
    pub const REFERENCE_IS_SET: u32 = 0x0069_e090;
    /// `0042fb00` and `0042fa40`: construct and destroy the 4-byte local.
    pub const LOCAL_CONSTRUCT: u32 = 0x0042_fb00;
    pub const LOCAL_DESTRUCT: u32 = 0x0042_fa40;
    /// `00690860` (`RET 4`, the local): a byte; fills the local.
    pub const REFERENCE_FILL: u32 = 0x0069_0860;
    /// `00559450`: the local's object.
    pub const LOCAL_OBJECT: u32 = 0x0055_9450;
    /// `00440d80`: a byte of that object.
    pub const OBJECT_FLAG_00440D80: u32 = 0x0044_0d80;
    /// `0044ddc0`: an array's element count.
    pub const COUNT: u32 = 0x0044_ddc0;
    /// `00877a30` (`RET 4`, index): the address of an element.
    pub const ELEMENT: u32 = 0x0087_7a30;
    /// `BSSimpleArray<BGSBodyPart_P_1024>::_MoveItems` (engine map,
    /// `009a4320`, `RET 8`: index, 1).
    pub const MOVE_ITEMS: u32 = 0x009a_4320;
    /// `BSSimpleArray<BGSBodyPart_P_1024>::AddUninitialized` (engine map,
    /// `007cb2e0`, `RET 4`: the address of the word to add).
    pub const ADD: u32 = 0x007c_b2e0;
    /// `0068f6b0` (`RET 4`, a 16-bit index): the address of a word of the
    ///   navmesh.
    pub const NAV_MESH_ITEM_0068F6B0: u32 = 0x0068_f6b0;
    /// `0059bb30` (the engine map's `D3DTexture_LockRect` is a folded
    /// name): the word at the navmesh's `+0x24`, the object the three
    /// `TESObjectCELL` getters below are called on.
    pub const NAV_MESH_CELL: u32 = 0x0059_bb30;
    /// `TESObjectCELL::GetWorldSpace`, `GetDataY`, `GetDataX` (engine map).
    pub const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
    pub const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
    pub const CELL_GET_DATA_X: u32 = 0x0054_4c30;
    /// `00587410` (cdecl: x, y).
    pub const FROM_GRID_00587410: u32 = 0x0058_7410;
    /// The zero point copied to `+0x10` (`011f426c`, three words).
    pub const POINT_011F426C: u32 = 0x011f_426c;
    /// `00990a60`: another count of the navmesh (its `0044ddc0`; Ghidra's library match names it `CDocument::GetFirstViewPosition`).
    pub const COUNT_00990A60: u32 = 0x0099_0a60;
    /// `0068f0a0` (`RET 4`, a 16-bit index): one of those items.
    pub const ITEM_0068F0A0: u32 = 0x0068_f0a0;
    /// `0063c8a0` (`RET 4`, the item), on the point at `+0x10`.
    pub const POINT_ADD_0063C8A0: u32 = 0x0063_c8a0;
    /// `004941c0` (`RET 4`, a `float`), on the point at `+0x10`.
    pub const POINT_DIVIDE_004941C0: u32 = 0x0049_41c0;
    /// `00693d50` (`RET 4`, a `float`), on `this`.
    pub const INFO_SET_RATIO_00693D50: u32 = 0x0069_3d50;
    /// `004534f0` (`RET 4`), on `this` with 1.
    pub const INFO_004534F0: u32 = 0x0045_34f0;
}

/// The reference-matching step of `006b5560` for one entry whose reference
/// is at `reference` (the entry's `+4`): whether the entry's word is added.
fn keep_entry(e: &mut Engine, this: u32, reference: u32, flag: bool, local: u32) -> bool {
    use info_build::*;
    let mut keep = true;
    let target = e.mem.u32(reference);
    if !e.call(REFERENCE_FILL, &args![target, local]).bool() {
        return false;
    }
    let object = e.call(LOCAL_OBJECT, &args![local]).u32();
    if e.call(OBJECT_FLAG_00440D80, &args![object]).bool() {
        return false;
    }
    if flag {
        let count = e.call(COUNT, &args![this + 0x24]).u32();
        let mut index = 0u32;
        while index < count {
            let slot = e.call(ELEMENT, &args![this + 0x24, index]).u32();
            if e.mem.u32(slot) == e.mem.u32(reference) {
                e.call(MOVE_ITEMS, &args![this + 0x24, index, 1u32]);
                break;
            }
            index += 1;
        }
        let count = e.call(COUNT, &args![this + 0x34]).u32();
        let mut index = 0u32;
        while index < count {
            let slot = e.call(ELEMENT, &args![this + 0x34, index]).u32();
            if e.mem.u32(slot) == e.mem.u32(reference) {
                keep = false;
                break;
            }
            index += 1;
        }
    } else {
        let count = e.call(COUNT, &args![this + 0x24]).u32();
        let mut index = 0u32;
        while index < count {
            let slot = e.call(ELEMENT, &args![this + 0x24, index]).u32();
            if e.mem.u32(slot) == e.mem.u32(reference) {
                keep = false;
                break;
            }
            index += 1;
        }
    }
    keep
}

// Translated from 006b5560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b5560 - the exception-unwinding frame (FS:[0]) that destroys the loop's local on unwind is not translated.
/// Fills this navmesh info from the navmesh `nav_mesh` (`thiscall`; one
/// word; `RET 4`); the engine map gives it no name. In order:
/// - `+0` is `0084e3a0` of the navmesh, `+0x1c` its parent space
///   (`0068ef70`) and `+4` `0084e3a0` of that; the arrays at `+0x24`,
///   `+0x34`, `+0x44` are reset (`008454f0` with 1); the flags at `+8` are
///   0; the object at `+0x58`, when there is one, is deleted (`006b4850`
///   with 1) and the pointer cleared; `00693ef0(1)` runs when the
///   navmesh's `00440da0` is set.
/// - For each item (`00558200`, `005582f0` with a 16-bit index) whose
///   `00690e10` is set, with its `00693b70` byte as the flag: for each of
///   n = 0, 1, 2 whose `0068f1d0` bit is set, the navmesh's
///   entry `00693d30(0068f2c0(n))`; when the reference at the entry's `+4` is
///   set (`0069e090`), a local is constructed (`0042fb00`), the reference
///   fills it (`00690860`), and the reference's word is added
///   (`007cb2e0`) to `+0x34` (flag set) or `+0x24` (flag clear) unless the
///   fill fails, the local's object (`00559450`) answers `00440d80`, or
///   the word is already in `+0x24` (flag clear) or `+0x34` (flag set; the
///   flag also first removes a match from `+0x24` with `_MoveItems`); the
///   local is destroyed (`0042fa40`).
/// - Flag 8 of `+8` is set when the count `00558200` is 0, else cleared.
/// - For each index below `006b5a90`'s count, the address `0068f6b0` gives
///   (16-bit index) is added to `+0x44`.
/// - `+0xc` is `00587410(x, y)` of the `TESObjectCELL::GetDataX` and
///   `GetDataY` (engine map) of the navmesh's `+0x24` word (`0059bb30`) when
///   its `GetWorldSpace` is not 0, else 0.
/// - `+0x10` is the point at `011f426c`; each of the `00990a60` items
///   (`0068f0a0`) is added to it (`0063c8a0`), then `004941c0` gets that
///   count as a `float`, when the count is not 0.
/// - When `00558200` is not 0, `00693d50` gets the share of its items whose
///   `00693b70` is set (two `FILD`s, `FDIVP`, rounded to a `float`).
/// - `004534f0(1)` on `this`.
pub fn fn_006b5560(e: &mut Engine, this: Ptr, nav_mesh: Ptr) {
    use info_build::*;
    let base = this.addr();
    let mesh = nav_mesh.addr();

    let id = e.call(WORD_AT_C, &args![mesh]).u32();
    e.mem.set_u32(base, id);
    let parent = e.call(NAV_MESH_GET_PARENT_SPACE, &args![mesh]).u32();
    e.mem.set_u32(base + 0x1c, parent);
    let parent = e.call(NAV_MESH_GET_PARENT_SPACE, &args![mesh]).u32();
    let id = e.call(WORD_AT_C, &args![parent]).u32();
    e.mem.set_u32(base + 4, id);
    for array in [0x24, 0x34, 0x44] {
        e.call(ARRAY_RESET, &args![base + array, 1u32]);
    }
    e.mem.set_u32(base + 8, 0);
    if e.mem.u32(base + 0x58) != 0 {
        let extra = e.mem.u32(base + 0x58);
        if extra != 0 {
            e.call(EXTRA_DELETE, &args![extra, 1u32]);
        }
        e.mem.set_u32(base + 0x58, 0);
    }
    if e.call(NAV_MESH_FLAG_00440DA0, &args![mesh]).bool() {
        e.call(INFO_00693EF0, &args![this, 1u32]);
    }

    let items = e.call(COUNT_00558200, &args![mesh]).u32();
    // The local `[EBP-0x34]` the reference fills.
    e.with_stack(4, |e, local| {
        let local = local.addr();
        let mut index = 0u32;
        while index < items {
            let item = e.call(ITEM_005582F0, &args![mesh, index & 0xffff]).u32();
            if e.call(ITEM_FLAG_00690E10, &args![item]).bool() {
                let flag = e.call(ITEM_FLAG_00693B70, &args![item]).bool();
                let mut n = 0i32;
                while n < 3 {
                    if e.call(ITEM_BIT_0068F1D0, &args![item, n]).bool() {
                        let link = e.call(ITEM_LINK_0068F2C0, &args![item, n]).u16();
                        let entry = e
                            .call(NAV_MESH_ENTRY_00693D30, &args![mesh, u32::from(link)])
                            .u32();
                        let reference = entry.wrapping_add(4);
                        if e.call(REFERENCE_IS_SET, &args![reference]).bool() {
                            e.call(LOCAL_CONSTRUCT, &args![local]);
                            if keep_entry(e, base, reference, flag, local) {
                                let array = if flag { 0x34 } else { 0x24 };
                                e.call(ADD, &args![base + array, reference]);
                            }
                            e.call(LOCAL_DESTRUCT, &args![local]);
                        }
                    }
                    n += 1;
                }
            }
            index += 1;
        }
    });

    let flags = e.mem.u32(base + 8);
    if items == 0 {
        e.mem.set_u32(base + 8, flags | 8);
    } else {
        e.mem.set_u32(base + 8, flags & 0xffff_fff7);
    }

    let words = fn_006b5a90(e, nav_mesh);
    let mut index = 0u32;
    while index < words {
        let word = e
            .call(NAV_MESH_ITEM_0068F6B0, &args![mesh, index & 0xffff])
            .u32();
        e.call(ADD, &args![base + 0x44, word]);
        index += 1;
    }

    let cell = e.call(NAV_MESH_CELL, &args![mesh]).u32();
    if e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32() != 0 {
        let cell = e.call(NAV_MESH_CELL, &args![mesh]).u32();
        let y = e.call(CELL_GET_DATA_Y, &args![cell]).u32();
        let cell = e.call(NAV_MESH_CELL, &args![mesh]).u32();
        let x = e.call(CELL_GET_DATA_X, &args![cell]).u32();
        let grid = e.call(FROM_GRID_00587410, &args![x, y]).u32();
        e.mem.set_u32(base + 0xc, grid);
    } else {
        e.mem.set_u32(base + 0xc, 0);
    }

    for offset in [0, 4, 8] {
        let word = e.global::<u32>(POINT_011F426C + offset);
        e.mem.set_u32(base + 0x10 + offset, word);
    }
    let others = e.call(COUNT_00990A60, &args![mesh]).u32();
    if others > 0 {
        let mut index = 0u32;
        while index < others {
            let other = e.call(ITEM_0068F0A0, &args![mesh, index & 0xffff]).u32();
            e.call(POINT_ADD_0063C8A0, &args![base + 0x10, other]);
            index += 1;
        }
        // `FILD qword` of the count (high word 0), `FSTP float`.
        let count = x87_store32(f64::from(others));
        e.call(POINT_DIVIDE_004941C0, &args![base + 0x10, count]);
    }

    let items = e.call(COUNT_00558200, &args![mesh]).u32();
    if items != 0 {
        let mut matched = 0u32;
        let mut index = 0u32;
        while index < items {
            let item = e.call(ITEM_005582F0, &args![mesh, index & 0xffff]).u32();
            if e.call(ITEM_FLAG_00693B70, &args![item]).bool() {
                matched = matched.wrapping_add(1);
            }
            index += 1;
        }
        let share = x87_store32(x87_div(f64::from(matched), f64::from(items)));
        e.call(INFO_SET_RATIO_00693D50, &args![this, x87_quiet_bits(share)]);
    }
    e.call(INFO_004534F0, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_5a90, fn_006b5a90(Ptr) -> u32),
        entry!(0x006b_5ab0, fn_006b5ab0(Ptr, Ptr)),
        entry!(0x006b_5560, fn_006b5560(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_006b5a90_passes_this_plus_58_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0044DDC0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_5a90, &args![0x4100u32]).u32(), 0x140d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0044DDC0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x58]);
    }

    /// A source and a destination of 0x5c bytes, the source's words numbered, and doubles
    /// for the callees (the allocator gives `block`, the constructor returns its `this`).
    fn copy_case(e: &mut Engine, own: u32, theirs: u32, block: u32) -> (u32, u32) {
        let source = e.mem.alloc(0x5c);
        let target = e.mem.alloc(0x5c);
        for index in 0..0x16u32 {
            e.mem.set_u32(source + 4 * index, 0x100 + index);
        }
        e.mem.set_u32(source + 0x58, own);
        e.mem.set_u32(target + 0x58, theirs);
        for address in [0x006e_5330u32, 0x0069_cc80, 0x006b_4850] {
            e.register(address, |_, _| Ret::default());
        }
        e.register_double(0x0040_1000, move |_, _| Ret {
            eax: block,
            ..Ret::default()
        });
        e.register(0x0069_bcf0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        (source, target)
    }

    #[test]
    fn fn_006b5ab0_copies_the_words_and_arrays() {
        let mut e = Engine::new();
        let (source, target) = copy_case(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x006b_5ab0, &args![source, target]);
        for offset in [0x0u32, 0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x54] {
            assert_eq!(e.mem.u32(target + offset), 0x100 + offset / 4);
        }
        assert_eq!(e.mem.u32(target + 0x20), 0);
        assert_eq!(e.mem.u32(target + 0x24), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x006e_5330, vec![source + 0x24, target + 0x24]),
                (0x006e_5330, vec![source + 0x34, target + 0x34]),
                (0x006e_5330, vec![source + 0x44, target + 0x44]),
            ]
        );
    }

    #[test]
    fn fn_006b5ab0_creates_or_reuses_the_destination_object() {
        // (destination's object, allocation result, the object the copy goes into)
        for (theirs, block, expected) in
            [(0x7000u32, 0u32, 0x7000u32), (0, 0x8000, 0x8000), (0, 0, 0)]
        {
            let mut e = Engine::new();
            let (source, target) = copy_case(&mut e, 0x6000, theirs, block);
            e.call_log = Some(vec![]);
            e.call(0x006b_5ab0, &args![source, target]);
            assert_eq!(e.mem.u32(target + 0x58), expected);
            let log = e.call_log.take().unwrap();
            assert_eq!(log.last().unwrap(), &(0x0069_cc80, vec![0x6000, expected]));
            let allocations: Vec<_> = log.iter().filter(|call| call.0 == 0x0040_1000).collect();
            assert_eq!(allocations.len(), usize::from(theirs == 0));
            if theirs == 0 {
                assert_eq!(allocations[0].1, vec![0x38]);
            }
            assert_eq!(
                log.iter().filter(|call| call.0 == 0x0069_bcf0).count(),
                usize::from(block != 0)
            );
        }
    }

    #[test]
    fn fn_006b5ab0_drops_the_destination_object_when_the_source_has_none() {
        for theirs in [0u32, 0x7000] {
            let mut e = Engine::new();
            let (source, target) = copy_case(&mut e, 0, theirs, 0);
            e.call_log = Some(vec![]);
            e.call(0x006b_5ab0, &args![source, target]);
            assert_eq!(e.mem.u32(target + 0x58), 0);
            let log = e.call_log.take().unwrap();
            let freed: Vec<_> = log.iter().filter(|call| call.0 == 0x006b_4850).collect();
            if theirs == 0 {
                assert!(freed.is_empty());
            } else {
                assert_eq!(freed[0].1, vec![theirs, 1]);
            }
        }
    }

    /// An array the doubles of `0044ddc0`, `00877a30` and `007cb2e0` treat
    /// as storage at `+4` and count at `+8`.
    fn simple_array(e: &mut Engine, at: u32, words: &[u32]) {
        let storage = e.mem.alloc(0x40);
        e.mem.set_u32(at + 4, storage);
        e.mem.set_u32(at + 8, words.len() as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(storage + 4 * i as u32, *word);
        }
    }

    fn array_words(e: &Engine, at: u32) -> Vec<u32> {
        let storage = e.mem.u32(at + 4);
        (0..e.mem.u32(at + 8))
            .map(|i| e.mem.u32(storage + 4 * i))
            .collect()
    }

    /// An info object, a navmesh and doubles for every callee of
    /// `006b5560`. Items are 0x10 bytes: byte 0 answers `00690e10`, byte 1
    /// `00693b70`, the word at `+0xc` holds the bits `0068f1d0` tests and
    /// the 16-bit words at `+6` the entry indices. Entry `n`'s reference
    /// (its `+4`) is a target object whose first word is `targets[n]`: a
    /// target with 0xdead fails the fill, one with 1 answers `00440d80`.
    struct InfoCase {
        e: Engine,
        this: u32,
        mesh: u32,
        items: Vec<u32>,
        entries: Vec<u32>,
        targets: Vec<u32>,
    }

    fn info_case(items: &[(u8, u8, u32, [u16; 3])], targets: &[u32]) -> InfoCase {
        use info_build::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        let mesh = e.mem.alloc(0x80);
        e.mem.set_u32(mesh + 0xc, 0x1111);
        let parent = e.mem.alloc(0x10);
        e.mem.set_u32(parent + 0xc, 0x2222);
        let cell = e.mem.alloc(0x10);
        e.mem.set_u32(mesh + 0x24, cell);
        e.mem.set_u32(cell, 1);
        simple_array(&mut e, mesh + 0x58, &[9]);
        simple_array(&mut e, this + 0x24, &[]);
        simple_array(&mut e, this + 0x34, &[]);
        simple_array(&mut e, this + 0x44, &[]);
        let item_list: Vec<u32> = items
            .iter()
            .map(|(first, second, bits, words)| {
                let item = e.mem.alloc(0x10);
                e.mem.set_u8(item, *first);
                e.mem.set_u8(item + 1, *second);
                e.mem.set_u32(item + 0xc, *bits);
                for (n, word) in words.iter().enumerate() {
                    e.mem.set_u16(item + 6 + 2 * n as u32, *word);
                }
                item
            })
            .collect();
        let target_list: Vec<u32> = targets
            .iter()
            .map(|word| {
                let object = e.mem.alloc(4);
                e.mem.set_u32(object, *word);
                object
            })
            .collect();
        let entries: Vec<u32> = target_list
            .iter()
            .map(|target| {
                let entry = e.mem.alloc(8);
                e.mem.set_u32(entry + 4, *target);
                entry
            })
            .collect();
        e.map(POINT_011F426C & !0xfff, 0x1000);
        for i in 0..3 {
            e.set_global(POINT_011F426C + 4 * i, 0x100 + i);
        }
        e.register(WORD_AT_C, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register_double(NAV_MESH_GET_PARENT_SPACE, move |_, _| parent.into_ret());
        for addr in [
            ARRAY_RESET,
            EXTRA_DELETE,
            INFO_00693EF0,
            LOCAL_CONSTRUCT,
            LOCAL_DESTRUCT,
            MOVE_ITEMS,
            POINT_ADD_0063C8A0,
            POINT_DIVIDE_004941C0,
            INFO_SET_RATIO_00693D50,
            INFO_004534F0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(NAV_MESH_FLAG_00440DA0, |_, _| 0x101u32.into_ret());
        let count = item_list.len() as u32;
        e.register_double(COUNT_00558200, move |_, _| count.into_ret());
        let list = item_list.clone();
        e.register_double(ITEM_005582F0, move |_, a| list[a[1] as usize].into_ret());
        e.register(ITEM_FLAG_00690E10, |e, a| {
            u32::from(e.mem.u8(a[0])).into_ret()
        });
        e.register(ITEM_FLAG_00693B70, |e, a| {
            u32::from(e.mem.u8(a[0] + 1)).into_ret()
        });
        e.register(ITEM_BIT_0068F1D0, |e, a| {
            u32::from(e.mem.u32(a[0] + 0xc) >> a[1] & 1 != 0).into_ret()
        });
        e.register(ITEM_LINK_0068F2C0, |e, a| {
            u32::from(e.mem.u16(a[0] + 6 + 2 * a[1])).into_ret()
        });
        let entry_list = entries.clone();
        e.register_double(NAV_MESH_ENTRY_00693D30, move |_, a| {
            entry_list[a[1] as usize].into_ret()
        });
        e.register(REFERENCE_IS_SET, |e, a| {
            u32::from(e.mem.u32(a[0]) != 0).into_ret()
        });
        e.register(REFERENCE_FILL, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            u32::from(e.mem.u32(a[0]) != 0xdead).into_ret()
        });
        e.register(LOCAL_OBJECT, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(OBJECT_FLAG_00440D80, |e, a| {
            u32::from(e.mem.u32(a[0]) == 1).into_ret()
        });
        e.register(COUNT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ELEMENT, |e, a| (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret());
        e.register(ADD, |e, a| {
            let count = e.mem.u32(a[0] + 8);
            let storage = e.mem.u32(a[0] + 4);
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(storage + 4 * count, word);
            e.mem.set_u32(a[0] + 8, count + 1);
            Ret::default()
        });
        e.register(NAV_MESH_ITEM_0068F6B0, |e, a| {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, a[1] + 0x40);
            slot.into_ret()
        });
        e.register(NAV_MESH_CELL, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(CELL_GET_WORLD_SPACE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CELL_GET_DATA_Y, |_, _| 5u32.into_ret());
        e.register(CELL_GET_DATA_X, |_, _| 7u32.into_ret());
        e.register(FROM_GRID_00587410, |_, a| (a[0] << 8 | a[1]).into_ret());
        e.register(COUNT_00990A60, |_, _| 3u32.into_ret());
        e.register(ITEM_0068F0A0, |_, a| (0x5000 + a[1]).into_ret());
        InfoCase {
            e,
            this,
            mesh,
            items: item_list,
            entries,
            targets: target_list,
        }
    }

    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_006b5560_fills_the_header_and_the_unflagged_array() {
        use info_build::*;
        // Item 0: 00690e10 set, flag clear, sides 0 and 2 (entries 0 and 1).
        // Item 1: 00690e10 clear (skipped), its 00693b70 set.
        let InfoCase {
            mut e,
            this,
            mesh,
            items,
            entries,
            targets,
        } = info_case(
            &[(1, 0, 0b101, [0, 0, 1]), (0, 1, 0b111, [0, 0, 0])],
            &[0x10, 0x20],
        );
        // Entry 1's target is already in +0x24: not added again.
        simple_array(&mut e, this + 0x24, &[targets[1]]);
        e.mem.set_u32(this + 8, 0xffff_ffff);
        let extra = e.mem.alloc(0x38);
        e.mem.set_u32(this + 0x58, extra);
        e.call_log = Some(vec![]);
        e.call(0x006b_5560, &args![this, mesh]);
        let log = e.call_log.take().unwrap();

        assert_eq!(e.mem.u32(this), 0x1111);
        assert_eq!(e.mem.u32(this + 4), 0x2222);
        assert_eq!(calls(&log, NAV_MESH_GET_PARENT_SPACE).len(), 2);
        assert_eq!(
            calls(&log, ARRAY_RESET),
            vec![
                vec![this + 0x24, 1],
                vec![this + 0x34, 1],
                vec![this + 0x44, 1]
            ]
        );
        assert_eq!(calls(&log, EXTRA_DELETE), vec![vec![extra, 1]]);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(calls(&log, INFO_00693EF0), vec![vec![this, 1]]);
        // Flags: cleared, then bit 8 stays clear with items.
        assert_eq!(e.mem.u32(this + 8), 0);
        // Side 0 adds entry 0's target to +0x24.
        assert_eq!(array_words(&e, this + 0x24), vec![targets[1], targets[0]]);
        assert!(array_words(&e, this + 0x34).is_empty());
        assert_eq!(
            calls(&log, ITEM_BIT_0068F1D0),
            vec![vec![items[0], 0], vec![items[0], 1], vec![items[0], 2]]
        );
        assert_eq!(calls(&log, ADD)[0], vec![this + 0x24, entries[0] + 4]);
        assert_eq!(calls(&log, LOCAL_CONSTRUCT).len(), 2);
        assert_eq!(calls(&log, LOCAL_DESTRUCT).len(), 2);
        assert!(calls(&log, MOVE_ITEMS).is_empty());
        // The 006b5a90 words go to +0x44.
        assert_eq!(array_words(&e, this + 0x44), vec![0x40]);
        assert_eq!(calls(&log, NAV_MESH_ITEM_0068F6B0), vec![vec![mesh, 0]]);
        // Grid word from x 7, y 5.
        assert_eq!(e.mem.u32(this + 0xc), 0x705);
        assert_eq!(calls(&log, FROM_GRID_00587410), vec![vec![7, 5]]);
        // The point: copied, three items added, then 3.0.
        for i in 0..3 {
            assert_eq!(e.mem.u32(this + 0x10 + 4 * i), 0x100 + i);
        }
        assert_eq!(
            calls(&log, POINT_ADD_0063C8A0),
            vec![
                vec![this + 0x10, 0x5000],
                vec![this + 0x10, 0x5001],
                vec![this + 0x10, 0x5002]
            ]
        );
        assert_eq!(
            calls(&log, POINT_DIVIDE_004941C0),
            vec![vec![this + 0x10, 3.0f32.to_bits()]]
        );
        // One of the two items has its 00693b70 set: 0.5.
        assert_eq!(
            calls(&log, INFO_SET_RATIO_00693D50),
            vec![vec![this, 0.5f32.to_bits()]]
        );
        assert_eq!(calls(&log, INFO_004534F0), vec![vec![this, 1]]);
    }

    #[test]
    fn fn_006b5560_flagged_items_move_matches_and_fill_the_second_array() {
        use info_build::*;
        // One flagged item, sides 0, 1, 2: a target already in +0x24, one
        // whose fill fails, one whose object refuses.
        let InfoCase {
            mut e,
            this,
            mesh,
            entries,
            targets,
            ..
        } = info_case(&[(1, 1, 0b111, [0, 1, 2])], &[0x10, 0xdead, 1]);
        simple_array(&mut e, this + 0x24, &[0x77, targets[0]]);
        e.call_log = Some(vec![]);
        e.call(0x006b_5560, &args![this, mesh]);
        let log = e.call_log.take().unwrap();
        // The match in +0x24 (index 1) is moved out, the target goes to
        // +0x34; the failed fill and the refusing object add nothing.
        assert_eq!(calls(&log, MOVE_ITEMS), vec![vec![this + 0x24, 1, 1]]);
        assert_eq!(array_words(&e, this + 0x34), vec![targets[0]]);
        assert_eq!(calls(&log, ADD)[0], vec![this + 0x34, entries[0] + 4]);
        // The failed fill does not ask the object.
        assert_eq!(calls(&log, OBJECT_FLAG_00440D80).len(), 2);
        assert_eq!(calls(&log, LOCAL_DESTRUCT).len(), 3);
        // The only item has 00693b70 set: share 1.0.
        assert_eq!(
            calls(&log, INFO_SET_RATIO_00693D50),
            vec![vec![this, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_006b5560_without_items_sets_flag_8_and_skips_the_share() {
        use info_build::*;
        let InfoCase {
            mut e, this, mesh, ..
        } = info_case(&[], &[]);
        e.register(NAV_MESH_FLAG_00440DA0, |_, _| Ret::default());
        e.register(COUNT_00990A60, |_, _| Ret::default());
        e.register(CELL_GET_WORLD_SPACE, |_, _| Ret::default());
        e.mem.set_u32(this + 0xc, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x006b_5560, &args![this, mesh]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 8), 8);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert!(calls(&log, EXTRA_DELETE).is_empty());
        assert!(calls(&log, INFO_00693EF0).is_empty());
        assert!(calls(&log, POINT_ADD_0063C8A0).is_empty());
        assert!(calls(&log, POINT_DIVIDE_004941C0).is_empty());
        assert!(calls(&log, INFO_SET_RATIO_00693D50).is_empty());
        assert!(calls(&log, FROM_GRID_00587410).is_empty());
        assert_eq!(calls(&log, INFO_004534F0), vec![vec![this, 1]]);
    }
}
