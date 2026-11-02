//! `fallout shared/run_004aabb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `TES` singleton pointer (`ECX` of `005f36f0` and `00457050`).
const TES_POINTER: u32 = 0x011d_ea10;
/// `005f36f0` (`thiscall`, no stack word; the engine map's
/// `ActorMover::GetPreferredMoveMode` is folded code): `TES`'s interior
/// cell, or null.
const INTERIOR_CELL: u32 = 0x005f_36f0;
/// The object `0043d4d0` is asked on (its answer points at the grid size).
const GRID_SETTING: u32 = 0x011c_63cc;
/// `0043d4d0` (`thiscall`, no stack word).
const GRID_SIZE: u32 = 0x0043_d4d0;
/// `00457050` (`thiscall`, `RET 8`: x, y): the address of the loaded cell
/// slot at grid position (x, y).
const CELL_AT: u32 = 0x0045_7050;
/// `TESObjectCELL::Get3D` (engine map, `00545cb0`, `thiscall`).
const CELL_3D: u32 = 0x0054_5cb0;
/// `004aa3a0` (`thiscall`, `RET 0xc`: the cell's 3D, the cell, the flag).
const PROCESS_CELL: u32 = 0x004a_a3a0;

// Translated from 004aabb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `004aa3a0` on the loaded cells (`thiscall`, `RET 4`, a byte flag;
/// the engine map gives it no name): with the cell's 3D (`00545cb0`), the
/// cell and the zero-extended flag. In an interior (`005f36f0` on `TES`)
/// only that cell; otherwise every non-empty slot of the grid (`00457050`)
/// from `size / 2 - 2` to `size / 2 + 2` in both coordinates (`size` the
/// word `0043d4d0` points at; unsigned bounds, x outer).
pub fn fn_004aabb0(e: &mut Engine, this: Ptr, flag: u32) {
    let flag = u32::from(flag as u8);
    let tes = e.mem.u32(TES_POINTER);
    if e.call(INTERIOR_CELL, &args![tes]).u32() != 0 {
        let tes = e.mem.u32(TES_POINTER);
        let cell = e.call(INTERIOR_CELL, &args![tes]).u32();
        let cell_3d = e.call(CELL_3D, &args![cell]).u32();
        e.call(PROCESS_CELL, &args![this, cell_3d, cell, flag]);
        return;
    }
    let size_at = e.call(GRID_SIZE, &args![GRID_SETTING]).u32();
    let half = e.mem.u32(size_at) >> 1;
    let low = half.wrapping_sub(2);
    let high = half.wrapping_add(2);
    let mut x = low;
    while x <= high {
        let mut y = low;
        while y <= high {
            let tes = e.mem.u32(TES_POINTER);
            let slot = e.call(CELL_AT, &args![tes, x, y]).u32();
            let cell = e.mem.u32(slot);
            if cell != 0 {
                let cell_3d = e.call(CELL_3D, &args![cell]).u32();
                e.call(PROCESS_CELL, &args![this, cell_3d, cell, flag]);
            }
            y = y.wrapping_add(1);
        }
        x = x.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004a_abb0, fn_004aabb0(Ptr, u32))]
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The grid slots live at 0x0f00_0000 + 4 * (x * 16 + y).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(0x011d_e000, 0x1000);
        e.mem.map(0x0f00_0000, 0x1000);
        e.mem.map(0x0f00_2000, 8);
        e.mem.set_u32(TES_POINTER, 0x0f00_2000);
        e.register(INTERIOR_CELL, |e, a| word(e.mem.u32(a[0])));
        e.register(GRID_SIZE, |_, _| word(0x0f00_2004));
        e.register(CELL_AT, |_, a| word(0x0f00_0000 + 4 * (a[1] * 16 + a[2])));
        e.register(CELL_3D, |_, a| word(a[0] + 1));
        e.register(PROCESS_CELL, |_, _| Ret::default());
        e
    }

    fn processed(e: &mut Engine, flag: u32) -> Vec<Vec<u32>> {
        e.call_log = Some(vec![]);
        e.call(0x004a_abb0, &args![0x4000u32, flag]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == PROCESS_CELL)
            .map(|c| c.1)
            .collect()
    }

    #[test]
    fn at_004aabb0_runs_on_the_interior_cell() {
        let mut e = engine();
        e.mem.set_u32(0x0f00_2000, 0x7000);
        assert_eq!(
            processed(&mut e, 0x101),
            vec![vec![0x4000, 0x7001, 0x7000, 1]]
        );
    }

    #[test]
    fn at_004aabb0_walks_the_grid() {
        let mut e = engine();
        // Size 5: half 2, from 0 to 4.
        e.mem.set_u32(0x0f00_2004, 5);
        let slot = |x: u32, y: u32| 0x0f00_0000 + 4 * (x * 16 + y);
        e.mem.set_u32(slot(0, 0), 0x7100);
        e.mem.set_u32(slot(4, 4), 0x7200);
        e.mem.set_u32(slot(5, 0), 0x7300);
        e.call_log = Some(vec![]);
        e.call(0x004a_abb0, &args![0x4000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == CELL_AT).count(), 25);
        let cells: Vec<_> = log
            .iter()
            .filter(|c| c.0 == PROCESS_CELL)
            .map(|c| c.1[2])
            .collect();
        assert_eq!(cells, vec![0x7100, 0x7200]);
        // Size 2: half 1, the low bound wraps above the high one: nothing.
        e.mem.set_u32(0x0f00_2004, 2);
        assert!(processed(&mut e, 0).is_empty());
    }
}
