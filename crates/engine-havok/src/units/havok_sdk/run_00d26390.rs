//! `Havok SDK/run_00d26390` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d26390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x08` of the vtable at `010ce370` (`hkpTriSampledHeightFieldCollection`, Xbox PDB name of
/// the vtable's class; `hkpShapeContainer::getFirstKey` by the slot, not confirmed): answers 0
/// (`XOR EAX,EAX`).
pub fn fn_00d26390(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00d263a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x0c` of the same vtable (`getNextKey` by the slot, not confirmed; thiscall, one stack
/// word `key`). A key packs a cell and a triangle: bit 0 is the triangle, bits 1 to 15 the column
/// `x`, bits 16 and up the row `z`. An even key (first triangle) answers `key | 1`. An odd key
/// moves to the next cell: `x + 1`, and when that is the last column (`x + 1 == width - 1`, the
/// word at `(this + 8 object) + 0x10`) to column 0 of the next row, answering `0xffffffff` when
/// that row is the last (`z + 1 == depth - 1`, the word at `+0x14`); else `((z << 15) + x) * 2`.
pub fn fn_00d263a0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    if key & 1 == 0 {
        return key | 1;
    }
    let mut column = ((key >> 1) & 0x7fff) + 1;
    let shape = e.mem.u32(this.addr() + 8);
    let last_column = e.mem.u32(shape + 0x10).wrapping_sub(1);
    let mut row = key >> 16;
    if column == last_column {
        let last_row = e.mem.u32(shape + 0x14).wrapping_sub(1);
        row = row.wrapping_add(1);
        column = 0;
        if row == last_row {
            return 0xffff_ffff;
        }
    }
    let cell = (row << 15).wrapping_add(column);
    cell.wrapping_add(cell)
}

// Translated from 00d263f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x18` of the vtable of the class at `010ce390` (thiscall, `RET 8`, two stack words
/// `key` and `size`; descriptions, not confirmed): asks the object at `+0x18` (its slot `0x18`,
/// arguments `key` and `size - 0x40`) for a length. A length below 0 or above `size - 0x40`
/// (signed) answers `-1`. Otherwise, when the object at `+0x18` is the embedded one at
/// `this + 0x40`, clears `+0x1c` and answers `length + 0x40`; else stores the length at `+0x1c`
/// and answers `0x40`.
pub fn fn_00d263f0(e: &mut Engine, this: Ptr, key: u32, size: u32) -> i32 {
    let base = this.addr();
    let inner = e.mem.u32(base + 0x18);
    let limit = size.wrapping_sub(0x40) as i32;
    let length = e.vcall(inner, 0x18, &args![key, limit]).i32();
    if length < 0 || length > limit {
        return -1;
    }
    if e.mem.u32(base + 0x18) == base + 0x40 {
        e.mem.set_u32(base + 0x1c, 0);
        length.wrapping_add(0x40)
    } else {
        e.mem.set_i32(base + 0x1c, length);
        0x40
    }
}

// Translated from 00d26440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x10` of the vtable at `010ce370` (`getCollisionFilterInfo` by the slot, not confirmed;
/// thiscall, `RET 4`, the stack word is never read): answers 0.
pub fn fn_00d26440(_e: &mut Engine, _this: Ptr, _unused_1: u32) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_6390, fn_00d26390(Ptr) -> u32),
        entry!(0x00d2_63a0, fn_00d263a0(Ptr, u32) -> u32),
        entry!(0x00d2_63f0, fn_00d263f0(Ptr, u32, u32) -> i32),
        entry!(0x00d2_6440, fn_00d26440(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d26390_and_fn_00d26440_answer_zero() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00d2_6390, &args![this]).u32(), 0);
        assert_eq!(e.call(0x00d2_6440, &args![this, 0x1234u32]).u32(), 0);
    }

    /// An object whose word at `+8` is a shape with the given width and depth.
    fn collection(e: &mut Engine, width: u32, depth: u32) -> u32 {
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0x10, width);
        e.mem.set_u32(shape + 0x14, depth);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, shape);
        this
    }

    #[test]
    fn fn_00d263a0_walks_the_cells_and_triangles() {
        let mut e = Engine::new();
        let this = collection(&mut e, 5, 4);
        let next = |e: &mut Engine, key: u32| e.call(0x00d2_63a0, &args![this, key]).u32();
        // The first triangle of a cell goes to its second.
        assert_eq!(next(&mut e, 0), 1);
        assert_eq!(next(&mut e, 0x0001_0004), 0x0001_0005);
        // The second triangle of column 1, row 0 goes to column 2, row 0, first triangle.
        assert_eq!(next(&mut e, 1 | (1 << 1)), 4);
        // Row 1, column 2: the next cell is column 3.
        assert_eq!(next(&mut e, (1 << 16) | (2 << 1) | 1), ((1 << 15) + 3) * 2);
        // Column 3 gives x + 1 = 4 = width - 1: the start of the next row.
        assert_eq!(next(&mut e, (3 << 1) | 1), (1 << 15) * 2);
        // On the row before the last (depth 4: rows 0 to 2 are used) the end is 0xffffffff.
        assert_eq!(next(&mut e, (1 << 16) | (3 << 1) | 1), (2 << 15) * 2);
        assert_eq!(next(&mut e, (2 << 16) | (3 << 1) | 1), 0xffff_ffff);
    }

    #[test]
    fn fn_00d263f0_asks_the_inner_object_for_a_length() {
        for embedded in [false, true] {
            let mut e = Engine::new();
            let this = e.mem.alloc(0x80);
            let inner = if embedded {
                this + 0x40
            } else {
                e.mem.alloc(0x20)
            };
            e.mem.set_u32(this + 0x18, inner);
            e.mem.set_u32(this + 0x1c, 0x77);
            let vtable = e.mem.alloc(0x40);
            e.mem.set_u32(vtable + 0x18, 0x0900_0041);
            e.mem.set_u32(inner, vtable);
            e.register(0x0900_0041, |_, a| {
                // `key` 1 answers 10, 2 answers -3, 3 answers the limit plus one, 4 the limit.
                let limit = a[2] as i32;
                let answer = match a[1] {
                    1 => 10,
                    2 => -3,
                    3 => limit + 1,
                    _ => limit,
                };
                Ret {
                    eax: answer as u32,
                    ..Ret::default()
                }
            });
            e.call_log = Some(vec![]);
            let result = e.call(0x00d2_63f0, &args![this, 1u32, 0x140u32]).i32();
            let log = e.call_log.take().unwrap();
            let asked: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0900_0041).collect();
            assert_eq!(asked[0].1, vec![inner, 1, 0x100]);
            if embedded {
                assert_eq!(result, 10 + 0x40);
                assert_eq!(e.mem.u32(this + 0x1c), 0);
            } else {
                assert_eq!(result, 0x40);
                assert_eq!(e.mem.u32(this + 0x1c), 10);
            }
            // Out of range on either side: -1, and nothing stored.
            e.mem.set_u32(this + 0x1c, 0x77);
            assert_eq!(e.call(0x00d2_63f0, &args![this, 2u32, 0x140u32]).i32(), -1);
            assert_eq!(e.call(0x00d2_63f0, &args![this, 3u32, 0x140u32]).i32(), -1);
            assert_eq!(e.mem.u32(this + 0x1c), 0x77);
            // The limit itself is allowed.
            let result = e.call(0x00d2_63f0, &args![this, 4u32, 0x140u32]).i32();
            assert_eq!(result, if embedded { 0x100 + 0x40 } else { 0x40 });
        }
    }
}
