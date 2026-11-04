//! `fallout shared/gridarray.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004b9e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridArray` scalar deleting destructor (engine map name): runs the destructor body `004b9e50` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn grid_array_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004b_9e50, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

layout! {
    /// `GridArray` (Xbox PDB), 0x10 bytes.
    pub struct GridArray: 0x10 {
        /// `iCenterX` (Xbox PDB).
        0x04 iCenterX: i32,
        /// `iCenterY` (Xbox PDB).
        0x08 iCenterY: i32,
        /// `iDimension` (Xbox PDB): cells per side.
        0x0c iDimension: u32,
    }
}

/// `GridArray`'s virtual slots (Xbox PDB vtable: `+0x08 DetachAll`,
/// `+0x14 Shift`, `+0x18 Detach`, `+0x1c ClearItem`, `+0x20 MoveItem`).
const SLOT_DETACH_ALL: u32 = 0x08;
const SLOT_SHIFT: u32 = 0x14;
const SLOT_DETACH: u32 = 0x18;
const SLOT_CLEAR_ITEM: u32 = 0x1c;
const SLOT_MOVE_ITEM: u32 = 0x20;
/// `00726070` and `0044ddc0` (`thiscall`, unnamed in the engine map): the
/// word getters `SetCenter` reads the centre with (`+4` and `+8` of a
/// `GridArray`).
const GET_WORD_4: u32 = 0x0072_6070;
const GET_WORD_8: u32 = 0x0044_ddc0;
/// `00ec7d40` (`cdecl`, one word): `abs`.
const ABS: u32 = 0x00ec_7d40;
/// `GridArray`'s vtable (`01020634`; RTTI `.?AVGridArray@@`).
const GRID_ARRAY_VTABLE: u32 = 0x0102_0634;

// Translated from 004b9e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `GridArray` destructor body (unnamed in the engine map; `thiscall`,
/// plain `RET`): stores the `GridArray` vtable `01020634`.
pub fn fn_004b9e50(e: &mut Engine, this: Ptr<GridArray>) {
    e.mem.set_u32(this.addr(), GRID_ARRAY_VTABLE);
}

// Translated from 004b9e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridArray::Init` (Xbox PDB; `thiscall`, plain `RET`): sets `iCenterY`
/// and `iCenterX` to -10000.
pub fn grid_array_init(e: &mut Engine, this: Ptr<GridArray>) {
    e.set(this, GridArray::iCenterY, -10000);
    e.set(this, GridArray::iCenterX, -10000);
}

// Translated from 004b9e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): calls the object's
/// virtual slot `+0x8` (`DetachAll` in the Xbox PDB's `GridArray` vtable).
pub fn fn_004b9e90(e: &mut Engine, this: Ptr<GridArray>) {
    e.vcall(this.addr(), SLOT_DETACH_ALL, &[]);
}

// Translated from 004b9eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridArray::DetachAll` (Xbox PDB; `thiscall`, plain `RET`): virtual slot
/// `+0x18` (`Detach`) on every cell `(x, y)`, `x` outer, both below
/// `iDimension` (read again at each step).
pub fn grid_array_detach_all(e: &mut Engine, this: Ptr<GridArray>) {
    let mut x = 0u32;
    while x < e.get(this, GridArray::iDimension) {
        let mut y = 0u32;
        while y < e.get(this, GridArray::iDimension) {
            e.vcall(this.addr(), SLOT_DETACH, &args![x, y]);
            y = y.wrapping_add(1);
        }
        x = x.wrapping_add(1);
    }
}

// Translated from 004b9f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridArray::SetCenter` (Xbox PDB; `thiscall`, `RET 8`): shifts the grid
/// (virtual slot `+0x14`, `Shift`) by the difference between `(x, y)` and
/// the current centre (read with `00726070` and `0044ddc0`); true when
/// either difference is non-zero.
pub fn grid_array_set_center(e: &mut Engine, this: Ptr<GridArray>, x: i32, y: i32) -> bool {
    let dx = x.wrapping_sub(e.call(GET_WORD_4, &args![this]).i32());
    let dy = y.wrapping_sub(e.call(GET_WORD_8, &args![this]).i32());
    e.vcall(this.addr(), SLOT_SHIFT, &args![dx, dy]);
    dx != 0 || dy != 0
}

// Translated from 004b9f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GridArray::Shift` (Xbox PDB; `thiscall`, `RET 8`): moves the centre by
/// `(dx, dy)`. When `abs(dx)` or `abs(dy)` (`00ec7d40`) is not below
/// `iDimension`, everything is detached (virtual slot `+0x8`). Otherwise the
/// cells move one step at a time, first along x until `dx` is used up, then
/// along y. A negative step, for each row (or column) `i`: `Detach` (slot
/// `+0x18`) of the last cell, `MoveItem` (slot `+0x20`: from, to) of every
/// cell one place up from the end, `ClearItem` (slot `+0x1c`) of the first.
/// A positive step: `Detach` of the first, `MoveItem` one place down,
/// `ClearItem` of the last.
pub fn grid_array_shift(e: &mut Engine, this: Ptr<GridArray>, dx: i32, dy: i32) {
    let mut dx = dx;
    let mut dy = dy;
    let center_x = e.get(this, GridArray::iCenterX);
    e.set(this, GridArray::iCenterX, center_x.wrapping_add(dx));
    let center_y = e.get(this, GridArray::iCenterY);
    e.set(this, GridArray::iCenterY, center_y.wrapping_add(dy));
    let dim = |e: &Engine| e.get(this, GridArray::iDimension);
    if e.call(ABS, &args![dx]).u32() >= dim(e) || e.call(ABS, &args![dy]).u32() >= dim(e) {
        e.vcall(this.addr(), SLOT_DETACH_ALL, &[]);
        return;
    }
    let me = this.addr();
    while dx != 0 {
        if dx < 0 {
            let mut row = 0u32;
            while row < dim(e) {
                let mut x = dim(e).wrapping_sub(1);
                e.vcall(me, SLOT_DETACH, &args![x, row]);
                while x > 0 {
                    e.vcall(me, SLOT_MOVE_ITEM, &args![x.wrapping_sub(1), row, x, row]);
                    x = x.wrapping_sub(1);
                }
                e.vcall(me, SLOT_CLEAR_ITEM, &args![x, row]);
                row = row.wrapping_add(1);
            }
            dx = dx.wrapping_add(1);
        } else {
            let mut row = 0u32;
            while row < dim(e) {
                let mut x = 0u32;
                e.vcall(me, SLOT_DETACH, &args![x, row]);
                while x < dim(e).wrapping_sub(1) {
                    e.vcall(me, SLOT_MOVE_ITEM, &args![x.wrapping_add(1), row, x, row]);
                    x = x.wrapping_add(1);
                }
                e.vcall(me, SLOT_CLEAR_ITEM, &args![x, row]);
                row = row.wrapping_add(1);
            }
            dx = dx.wrapping_sub(1);
        }
    }
    while dy != 0 {
        if dy < 0 {
            let mut column = 0u32;
            while column < dim(e) {
                let mut y = dim(e).wrapping_sub(1);
                e.vcall(me, SLOT_DETACH, &args![column, y]);
                while y > 0 {
                    e.vcall(
                        me,
                        SLOT_MOVE_ITEM,
                        &args![column, y.wrapping_sub(1), column, y],
                    );
                    y = y.wrapping_sub(1);
                }
                e.vcall(me, SLOT_CLEAR_ITEM, &args![column, y]);
                column = column.wrapping_add(1);
            }
            dy = dy.wrapping_add(1);
        } else {
            let mut column = 0u32;
            while column < dim(e) {
                let mut y = 0u32;
                e.vcall(me, SLOT_DETACH, &args![column, y]);
                while y < dim(e).wrapping_sub(1) {
                    e.vcall(
                        me,
                        SLOT_MOVE_ITEM,
                        &args![column, y.wrapping_add(1), column, y],
                    );
                    y = y.wrapping_add(1);
                }
                e.vcall(me, SLOT_CLEAR_ITEM, &args![column, y]);
                column = column.wrapping_add(1);
            }
            dy = dy.wrapping_sub(1);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b_9e20, grid_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004b_9e50, fn_004b9e50(Ptr<GridArray>)),
        entry!(0x004b_9e70, grid_array_init(Ptr<GridArray>)),
        entry!(0x004b_9e90, fn_004b9e90(Ptr<GridArray>)),
        entry!(0x004b_9eb0, grid_array_detach_all(Ptr<GridArray>)),
        entry!(
            0x004b_9f10,
            grid_array_set_center(Ptr<GridArray>, i32, i32) -> bool
        ),
        entry!(0x004b_9f70, grid_array_shift(Ptr<GridArray>, i32, i32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004b9e20_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004b_9e50, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004b_9e20, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004b_9e20)
                .collect();
            let mut expected = vec![(0x004b_9e50, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b015_tests {
    use super::*;

    const DETACH_ALL: u32 = 0x0090_0008;
    const SHIFT: u32 = 0x0090_0014;
    const DETACH: u32 = 0x0090_0018;
    const CLEAR: u32 = 0x0090_001c;
    const MOVE: u32 = 0x0090_0020;

    fn grid(e: &mut Engine, dimension: u32) -> Ptr<GridArray> {
        let mut slots = vec![0u32; 9];
        for (slot, target) in [
            (SLOT_DETACH_ALL, DETACH_ALL),
            (SLOT_SHIFT, SHIFT),
            (SLOT_DETACH, DETACH),
            (SLOT_CLEAR_ITEM, CLEAR),
            (SLOT_MOVE_ITEM, MOVE),
        ] {
            slots[(slot / 4) as usize] = target;
            e.register(target, |_, _| Ret::default());
        }
        e.put_vtable(0x0090_0000, &slots);
        let g: Ptr<GridArray> = e.new_object();
        e.mem.set_u32(g.addr(), 0x0090_0000);
        e.set(g, GridArray::iDimension, dimension);
        g
    }

    /// The grid's slot calls, as (target, arguments without `this`).
    fn slot_calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| (0x0090_0000..0x0090_0100).contains(&c.0))
            .map(|(a, w)| (a, w[1..].to_vec()))
            .collect()
    }

    #[test]
    fn destructor_body_restores_the_vtable() {
        let mut e = Engine::new();
        let g: Ptr<GridArray> = e.new_object();
        e.mem.set_u32(g.addr(), 0x1234);
        e.call(0x004b_9e50, &args![g]);
        assert_eq!(e.mem.u32(g.addr()), GRID_ARRAY_VTABLE);
    }

    #[test]
    fn init_puts_the_centre_far_away() {
        let mut e = Engine::new();
        let g = grid(&mut e, 5);
        e.call(0x004b_9e70, &args![g]);
        assert_eq!(e.get(g, GridArray::iCenterX), -10000);
        assert_eq!(e.get(g, GridArray::iCenterY), -10000);
    }

    #[test]
    fn slot_8_wrapper_calls_detach_all() {
        let mut e = Engine::new();
        let g = grid(&mut e, 5);
        e.call_log = Some(vec![]);
        e.call(0x004b_9e90, &args![g]);
        assert_eq!(slot_calls(&mut e), vec![(DETACH_ALL, vec![])]);
    }

    #[test]
    fn detach_all_visits_every_cell_x_outer() {
        let mut e = Engine::new();
        let g = grid(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x004b_9eb0, &args![g]);
        assert_eq!(
            slot_calls(&mut e),
            vec![
                (DETACH, vec![0, 0]),
                (DETACH, vec![0, 1]),
                (DETACH, vec![1, 0]),
                (DETACH, vec![1, 1]),
            ]
        );
    }

    #[test]
    fn set_center_shifts_by_the_difference() {
        let mut e = Engine::new();
        let g = grid(&mut e, 5);
        e.register(GET_WORD_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(GET_WORD_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.set(g, GridArray::iCenterX, 10);
        e.set(g, GridArray::iCenterY, -3);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004b_9f10, &args![g, 12i32, -5i32]).bool());
        assert_eq!(slot_calls(&mut e), vec![(SHIFT, vec![2, (-2i32) as u32])]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004b_9f10, &args![g, 10i32, -3i32]).bool());
        assert_eq!(slot_calls(&mut e), vec![(SHIFT, vec![0, 0])]);
    }

    fn abs_double(e: &mut Engine) {
        e.register(ABS, |_, a| (a[0] as i32).wrapping_abs().into_ret());
    }

    #[test]
    fn shift_too_far_detaches_everything() {
        let mut e = Engine::new();
        abs_double(&mut e);
        let g = grid(&mut e, 3);
        e.call_log = Some(vec![]);
        e.call(0x004b_9f70, &args![g, 0i32, -3i32]);
        assert_eq!(slot_calls(&mut e), vec![(DETACH_ALL, vec![])]);
        assert_eq!(e.get(g, GridArray::iCenterY), -3);
    }

    #[test]
    fn shift_moves_cells_one_step_at_a_time_x_then_y() {
        let mut e = Engine::new();
        abs_double(&mut e);
        let g = grid(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x004b_9f70, &args![g, -1i32, 1i32]);
        assert_eq!(e.get(g, GridArray::iCenterX), -1);
        assert_eq!(e.get(g, GridArray::iCenterY), 1);
        let mut expected = vec![];
        for row in 0..2 {
            expected.push((DETACH, vec![1, row]));
            expected.push((MOVE, vec![0, row, 1, row]));
            expected.push((CLEAR, vec![0, row]));
        }
        for column in 0..2 {
            expected.push((DETACH, vec![column, 0]));
            expected.push((MOVE, vec![column, 1, column, 0]));
            expected.push((CLEAR, vec![column, 1]));
        }
        assert_eq!(slot_calls(&mut e), expected);
        // The other two directions.
        e.call_log = Some(vec![]);
        e.call(0x004b_9f70, &args![g, 1i32, -1i32]);
        let mut expected = vec![];
        for row in 0..2 {
            expected.push((DETACH, vec![0, row]));
            expected.push((MOVE, vec![1, row, 0, row]));
            expected.push((CLEAR, vec![1, row]));
        }
        for column in 0..2 {
            expected.push((DETACH, vec![column, 1]));
            expected.push((MOVE, vec![column, 0, column, 1]));
            expected.push((CLEAR, vec![column, 0]));
        }
        assert_eq!(slot_calls(&mut e), expected);
    }
}
