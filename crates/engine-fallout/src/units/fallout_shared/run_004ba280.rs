//! `fallout shared/run_004ba280` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_shared::gridcell::GridCellArray;

/// `GridArray`'s constructor (`004b9e00`, `fallout shared/run_004b9e00`).
const GRID_ARRAY_CTOR: u32 = 0x004b_9e00;
/// `GridCellArray`'s vtable (`01020698`; the RTTI locator in the word before
/// it names `.?AVGridCellArray@@`).
const GRID_CELL_ARRAY_VTABLE: u32 = 0x0102_0698;
/// Initializes the `NiPoint3` at `WorldCenter` (`006815c0`, `__thiscall`,
/// returns `this`).
const POINT_INIT: u32 = 0x0068_15c0;
/// Smart-pointer constructor from a raw pointer (`00633c90`, `RET 4`).
const SMART_POINTER_CTOR: u32 = 0x0063_3c90;
/// `004bc4c0` (`fallout shared/run_004bc3d0`): no arguments, no `this`.
const FUNCTION_004BC4C0: u32 = 0x004b_c4c0;
/// `Setting` value getter (`0043d4d0`, `__thiscall` on the setting): returns
/// the address of the setting's value.
const SETTING_VALUE: u32 = 0x0043_d4d0;
/// The `uGridsToLoad:General` setting (`Setting` object at `011c63cc`; its
/// static initializer `00f3f190` passes the name at `01020700`).
const SETTING_GRIDS_TO_LOAD: u32 = 0x011c_63cc;
/// Stores a word at `+0xC` (`0041fd00`, `RET 4`; the map names it
/// `NonActorMagicCaster::SetCurrentSpell`, a shared body): here it sets
/// `GridArray::iDimension`.
const SET_WORD_AT_0C: u32 = 0x0041_fd00;
/// Reads the word at `+0xC` (`0084e3a0`): the grid's dimension.
const GRID_DIMENSION: u32 = 0x0084_e3a0;
/// `operator new` (`00401000`, cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `memset` wrapper (`00403d30`, cdecl: pointer, value, size).
const MEMSET: u32 = 0x0040_3d30;
/// `NiPoint3` constructor from three floats (`00416870`, `RET 0xC`, returns
/// `this`).
const POINT_CTOR: u32 = 0x0041_6870;
/// `GridCellArray::bRecreateCanopyShadowMaskTexture` (Xbox PDB, static).
const RECREATE_CANOPY_SHADOW_MASK_TEXTURE: u32 = 0x0118_9184;
/// Smart-pointer assignment from a raw pointer (`0066b0d0`, `RET 4`).
const SMART_POINTER_ASSIGN: u32 = 0x0066_b0d0;

// Translated from 004ba280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004ba280 - the C++ exception frame (FS:[0] handler 00ef0ec3, states 0 and 1 that destroy the base and the spTexRendered member when a later step throws) is not translated.
/// `GridCellArray`'s constructor (`__thiscall`, plain `RET`; the class is
/// the vtable's RTTI name, the map gives no name). Constructs the
/// `GridArray` base, stores the `GridCellArray` vtable, initializes
/// `WorldCenter` (+0x14) and `spTexRendered` (+0x24, null), runs
/// `004bc4c0`, sets `iDimension` to the value of `uGridsToLoad:General`,
/// allocates `pGridCellA` for `dimension * dimension` pointers and clears
/// it, sets `WorldCenter` to (0, 0, 0), sets the static
/// `bRecreateCanopyShadowMaskTexture`, assigns null to `spTexRendered`
/// again and clears `bLandAttached`. Returns `this`.
pub fn grid_cell_array_ctor(e: &mut Engine, this: Ptr<GridCellArray>) -> Ptr<GridCellArray> {
    let t = this.addr();
    e.call(GRID_ARRAY_CTOR, &args![this]);
    e.set(this, GridCellArray::vfptr, GRID_CELL_ARRAY_VTABLE);
    // GridCellArray::WorldCenter (Xbox PDB) +0x14
    e.call(POINT_INIT, &args![t + 0x14]);
    // GridCellArray::spTexRendered (Xbox PDB) +0x24
    e.call(SMART_POINTER_CTOR, &args![t + 0x24, 0u32]);
    e.call(FUNCTION_004BC4C0, &args![]);
    let value = e.call(SETTING_VALUE, &args![SETTING_GRIDS_TO_LOAD]).u32();
    let grids_to_load = e.mem.u32(value);
    e.call(SET_WORD_AT_0C, &args![this, grids_to_load]);

    // `IMUL` of the two dimensions (32-bit), then an unsigned `MUL` by 4;
    // a carry out of the low word asks for 0xFFFFFFFF bytes (`SETO`, `NEG`,
    // `OR`).
    let first = e.call(GRID_DIMENSION, &args![this]).u32();
    let second = e.call(GRID_DIMENSION, &args![this]).u32();
    let bytes = u64::from(first.wrapping_mul(second)) * 4;
    let request = if bytes >> 32 != 0 {
        u32::MAX
    } else {
        bytes as u32
    };
    let cells = e.call(OPERATOR_NEW, &args![request]).u32();
    e.set(this, GridCellArray::pGridCellA, Ptr::new(cells));

    // `SHL` by 2, then `IMUL`: both wrap at 32 bits.
    let first = e.call(GRID_DIMENSION, &args![this]).u32();
    let second = e.call(GRID_DIMENSION, &args![this]).u32();
    let size = (first << 2).wrapping_mul(second);
    let cells = e.get(this, GridCellArray::pGridCellA);
    e.call(MEMSET, &args![cells, 0u32, size]);

    // A temporary `NiPoint3` (0, 0, 0) on the stack, copied word by word
    // into `WorldCenter`.
    e.with_stack(12, |e, temp| {
        let zero = 0.0f32.to_bits();
        let point = e.call(POINT_CTOR, &args![temp, zero, zero, zero]).u32();
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(point + offset);
            e.mem.set_u32(t + 0x14 + offset, word);
        }
    });

    e.mem.set_u8(RECREATE_CANOPY_SHADOW_MASK_TEXTURE, 1);
    e.call(SMART_POINTER_ASSIGN, &args![t + 0x24, 0u32]);
    e.set(this, GridCellArray::bLandAttached, false);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x004ba280,
        grid_cell_array_ctor(Ptr<GridCellArray>) -> Ptr<GridCellArray>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles every callee; the dimension getter returns `dimension`, the
    /// setting's value is `grids`, `operator new` returns `cells`.
    fn setup(dimension: u32, grids: u32, cells: u32) -> (Engine, Ptr<GridCellArray>, Log) {
        let mut e = Engine::new();
        let this: Ptr<GridCellArray> = Ptr::new(e.mem.alloc(0x28));
        e.mem.map(RECREATE_CANOPY_SHADOW_MASK_TEXTURE, 1);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, grids);
        let log: Log = Rc::new(RefCell::new(vec![]));
        for addr in [
            GRID_ARRAY_CTOR,
            POINT_INIT,
            SMART_POINTER_CTOR,
            FUNCTION_004BC4C0,
            SET_WORD_AT_0C,
            MEMSET,
            SMART_POINTER_ASSIGN,
        ] {
            let log = log.clone();
            e.register_double(addr, move |_, a| {
                log.borrow_mut().push((addr, a.to_vec()));
                Ret::default()
            });
        }
        let l = log.clone();
        e.register_double(SETTING_VALUE, move |_, a| {
            l.borrow_mut().push((SETTING_VALUE, a.to_vec()));
            Ret {
                eax: value,
                ..Ret::default()
            }
        });
        let l = log.clone();
        e.register_double(GRID_DIMENSION, move |_, a| {
            l.borrow_mut().push((GRID_DIMENSION, a.to_vec()));
            Ret {
                eax: dimension,
                ..Ret::default()
            }
        });
        let l = log.clone();
        e.register_double(OPERATOR_NEW, move |_, a| {
            l.borrow_mut().push((OPERATOR_NEW, a.to_vec()));
            Ret {
                eax: cells,
                ..Ret::default()
            }
        });
        let l = log.clone();
        e.register_double(POINT_CTOR, move |e, a| {
            l.borrow_mut().push((POINT_CTOR, a.to_vec()));
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        (e, this, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn builds_the_grid_of_dimension_squared_cells() {
        let (mut e, this, log) = setup(5, 7, 0x0050_0000);
        let t = this.addr();
        for offset in (0..0x28).step_by(4) {
            e.mem.set_u32(t + offset, 0xcccc_cccc);
        }
        let back = e.call(0x004b_a280, &args![this]).ptr::<GridCellArray>();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(t), GRID_CELL_ARRAY_VTABLE);
        assert_eq!(calls(&log, GRID_ARRAY_CTOR), [vec![t]]);
        assert_eq!(calls(&log, POINT_INIT), [vec![t + 0x14]]);
        assert_eq!(calls(&log, SMART_POINTER_CTOR), [vec![t + 0x24, 0]]);
        assert_eq!(calls(&log, FUNCTION_004BC4C0), [Vec::<u32>::new()]);
        assert_eq!(calls(&log, SETTING_VALUE), [vec![SETTING_GRIDS_TO_LOAD]]);
        assert_eq!(calls(&log, SET_WORD_AT_0C), [vec![t, 7]]);
        assert_eq!(calls(&log, GRID_DIMENSION).len(), 4);
        assert_eq!(calls(&log, OPERATOR_NEW), [vec![100]]);
        assert_eq!(e.mem.u32(t + 0x10), 0x0050_0000);
        assert_eq!(calls(&log, MEMSET), [vec![0x0050_0000, 0, 100]]);
        assert_eq!(calls(&log, POINT_CTOR)[0][1..], [0, 0, 0]);
        assert_eq!(e.mem.u32(t + 0x14), 0);
        assert_eq!(e.mem.u32(t + 0x18), 0);
        assert_eq!(e.mem.u32(t + 0x1c), 0);
        assert_eq!(e.mem.u8(RECREATE_CANOPY_SHADOW_MASK_TEXTURE), 1);
        assert_eq!(calls(&log, SMART_POINTER_ASSIGN), [vec![t + 0x24, 0]]);
        assert_eq!(e.mem.u8(t + 0x20), 0);
        // The order of the steps that matter.
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        let at = |addr: u32| order.iter().position(|a| *a == addr).unwrap();
        assert!(at(GRID_ARRAY_CTOR) < at(POINT_INIT));
        assert!(at(SMART_POINTER_CTOR) < at(FUNCTION_004BC4C0));
        assert!(at(SET_WORD_AT_0C) < at(GRID_DIMENSION));
        assert!(at(OPERATOR_NEW) < at(MEMSET));
        assert!(at(POINT_CTOR) < at(SMART_POINTER_ASSIGN));
    }

    #[test]
    fn an_overflowing_size_asks_for_all_ones() {
        // 0x8000 * 0x8000 = 0x4000_0000 cells; times 4 carries out of 32 bits.
        let (mut e, this, log) = setup(0x8000, 3, 0x0060_0000);
        e.call(0x004b_a280, &args![this]);
        assert_eq!(calls(&log, OPERATOR_NEW), [vec![u32::MAX]]);
        // The clear size wraps: (0x8000 << 2) * 0x8000 = 0x1_0000_0000.
        assert_eq!(calls(&log, MEMSET), [vec![0x0060_0000, 0, 0]]);
    }
}
