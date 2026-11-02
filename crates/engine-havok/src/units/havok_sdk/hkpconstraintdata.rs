//! `Havok SDK/hkpconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d21a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintData::getConstraintInfoUtil` (map name). cdecl, three words:
/// `data` is a constraint atom stream of `size` bytes and `out` a constraint
/// info record, filled as the code does: `out[4] = data`, `out[5] = size`,
/// `out[0..4] = 0`.
///
/// When the first atom's type (a `u16`) is 0x16, the count `n` is the `u16`
/// at `data + 4`; `out[1] = (n >> 1) * 0x70 + (n & 1) * 0x30 + 0x80` and
/// `out[0] = out[1] + 0x20`, `out[2] = n + 2`, `out[3] = n + 3`, and for
/// `n >= 2` they become `out[1] = out[0]`, `out[2] = n + 3`, `out[3] = n + 4`.
///
/// Otherwise `out[1]` starts at 0x10 and the atoms are walked while the
/// cursor is below `data + size` (unsigned), adding per atom type the sizes
/// the jump table at `00d21c48` selects (type: bytes added to `out[1]`,
/// `out[2]`, `out[3]`, bytes the cursor advances):
/// 0 aligns the cursor to 16 bytes and looks at the atom there; 2, 3 and 4
/// only advance by 0x90, 0x30 and 0x70; 5: 0x90, 3, 3, 8; 6: 0x30, 1, 1, 8;
/// 7: 0x30, 1, 1, 4; 8 and 9: 0x40, 1, 1, 0xc; 10: 0x40, 1, 2, 8;
/// 11: 0x50, 1, 3, 0x10; 12: 0x60, 2, 2, 4; 13: n * 0x30, n, n, 4 with `n`
/// the byte at `+3`; 14: 0x30, 1, 1, 0x10; 15 and 16: 0x30, 1, 1, 0x14;
/// 17: n * 0x30, n, 2n, 0xc with `n` the byte at `+4`; 18: 0x40, 1, 3, 0x14;
/// 19: 0xc0, 3, 9, 0x50; 20: 0x40, 1, 1, 0x40. Types 1 and 23 to 27 end the
/// walk.
///
/// The code loops forever on types 21, 22 and above 27 (and on a type 0
/// that is already aligned): the table sends them back to the loop test
/// without moving the cursor. This translation panics there instead, so the
/// game's hang is a loud stop.
pub fn hkp_constraint_data_get_constraint_info_util(
    e: &mut Engine,
    data: Ptr,
    size: u32,
    out: Ptr,
) {
    let record = out.addr();
    e.mem.set_u32(record + 0x10, data.addr());
    e.mem.set_u32(record + 0x14, size);
    for i in 0..4 {
        e.mem.set_u32(record + 4 * i, 0);
    }
    if e.mem.u16(data.addr()) == 0x16 {
        let n = u32::from(e.mem.u16(data.addr() + 4));
        let base = (n >> 1) * 0x70 + (n & 1) * 0x30 + 0x80;
        let total = base + 0x20;
        e.mem.set_u32(record + 4, base);
        e.mem.set_u32(record, total);
        e.mem.set_u32(record + 8, n + 2);
        e.mem.set_u32(record + 0xc, n + 3);
        if n >= 2 {
            e.mem.set_u32(record + 0xc, n + 4);
            e.mem.set_u32(record + 8, n + 3);
            e.mem.set_u32(record + 4, total);
        }
        return;
    }
    let end = data.addr().wrapping_add(size);
    e.mem.set_u32(record + 4, 0x10);
    let mut cursor = data.addr();
    if cursor >= end {
        return;
    }
    loop {
        let top = cursor;
        let mut kind = u32::from(e.mem.u16(cursor));
        loop {
            // (bytes for out[1], out[2], out[3], cursor advance)
            let step: (u32, u32, u32, u32) = match kind {
                _ if kind > 0x1b => break,
                0 => {
                    let aligned = cursor.wrapping_add(0xf) & 0xffff_fff0;
                    assert!(
                        aligned != cursor,
                        "00d21a60 loops forever on an aligned type-0 atom at {cursor:08x}"
                    );
                    cursor = aligned;
                    kind = u32::from(e.mem.u16(cursor));
                    continue;
                }
                1 | 23..=27 => return,
                21 | 22 => break,
                2 => (0, 0, 0, 0x90),
                3 => (0, 0, 0, 0x30),
                4 => (0, 0, 0, 0x70),
                5 => (0x90, 3, 3, 8),
                6 => (0x30, 1, 1, 8),
                7 => (0x30, 1, 1, 4),
                8 | 9 => (0x40, 1, 1, 0xc),
                10 => (0x40, 1, 2, 8),
                11 => (0x50, 1, 3, 0x10),
                12 => (0x60, 2, 2, 4),
                13 => {
                    let n = u32::from(e.mem.u8(cursor + 3));
                    (n * 0x30, n, n, 4)
                }
                14 => (0x30, 1, 1, 0x10),
                15 | 16 => (0x30, 1, 1, 0x14),
                17 => {
                    let n = u32::from(e.mem.u8(cursor + 4));
                    (n * 0x30, n, 2 * n, 0xc)
                }
                18 => (0x40, 1, 3, 0x14),
                19 => (0xc0, 3, 9, 0x50),
                20 => (0x40, 1, 1, 0x40),
                _ => unreachable!("type {kind} is handled above"),
            };
            for (i, add) in [step.0, step.1, step.2].into_iter().enumerate() {
                let field = record + 4 + 4 * i as u32;
                let value = e.mem.u32(field).wrapping_add(add);
                e.mem.set_u32(field, value);
            }
            cursor = cursor.wrapping_add(step.3);
            break;
        }
        if cursor >= end {
            return;
        }
        assert!(
            cursor != top,
            "00d21a60 loops forever on an atom of type {kind} at {cursor:08x}"
        );
    }
}

// Translated from 00d21a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintData::addInstance` (Xbox PDB): `this` is the data (ECX, unused), then
/// `constraint` (unused), `runtime` and `size`. When `runtime` is not null the `size` bytes
/// there are cleared with `hkString::memSet(runtime, 0, size)` (`00cb1280`).
pub fn hkp_constraint_data_add_instance(
    e: &mut Engine,
    _this: Ptr,
    _constraint: Ptr,
    runtime: Ptr,
    size: u32,
) {
    if !runtime.is_null() {
        e.call(0x00cb_1280, &args![runtime, 0u32, size]);
    }
}

// Translated from 00d21a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintData::getSolverResults` (Xbox PDB): `this` is the data (ECX, unused);
/// returns `runtime` as it is.
pub fn hkp_constraint_data_get_solver_results(_e: &mut Engine, _this: Ptr, runtime: Ptr) -> Ptr {
    runtime
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2_1a30,
            hkp_constraint_data_add_instance(Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00d2_1a50, hkp_constraint_data_get_solver_results(Ptr, Ptr) -> Ptr),
        entry!(
            0x00d21a60,
            hkp_constraint_data_get_constraint_info_util(Ptr, u32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The info record after running the function on `bytes` (placed at a
    /// 16-byte aligned address) with `size`.
    fn info_00d21a60(bytes: &[(u32, u8)], size: u32) -> [u32; 6] {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x400);
        let data = (block + 15) & !15;
        for (offset, byte) in bytes {
            e.mem.set_u8(data + offset, *byte);
        }
        let out = e.mem.alloc(0x20);
        e.call(
            0x00d2_1a60,
            &args![Ptr::<()>::new(data), size, Ptr::<()>::new(out)],
        );
        assert_eq!(e.mem.u32(out + 0x10), data);
        [0, 4, 8, 12, 0x10, 0x14]
            .map(|offset| e.mem.u32(out + offset))
            .map(|v| if v == data { 0xdddd } else { v })
    }

    #[test]
    fn hkp_constraint_data_get_constraint_info_util_type_0x16_uses_the_count() {
        // n = 1: base 0x80 + 0x30; n = 5: (5 >> 1) * 0x70 + 0x30 + 0x80.
        let one = info_00d21a60(&[(0, 0x16), (4, 1)], 8);
        assert_eq!(one, [0xb0 + 0x20, 0xb0, 3, 4, 0xdddd, 8]);
        let five = info_00d21a60(&[(0, 0x16), (4, 5)], 8);
        let base = 2 * 0x70 + 0x30 + 0x80;
        assert_eq!(five, [base + 0x20, base + 0x20, 8, 9, 0xdddd, 8]);
    }

    #[test]
    fn hkp_constraint_data_get_constraint_info_util_walks_the_atoms() {
        // type 5, type 13 (n = 2), type 17 (n = 3), type 7, an unaligned
        // type 0 (aligned to 0x20), then type 2 advancing to the end.
        let atoms = [
            (0, 5),
            (8, 13),
            (11, 2),
            (12, 17),
            (16, 3),
            (0x18, 7),
            (0x1c, 0),
            (0x20, 2),
        ];
        let info = info_00d21a60(&atoms, 0xb0);
        // 0x10 + 0x90 + 2*0x30 + 3*0x30 + 0x30; 3 + 2 + 3 + 1; 3 + 2 + 6 + 1.
        assert_eq!(info, [0, 0x1c0, 9, 12, 0xdddd, 0xb0]);
    }

    #[test]
    fn hkp_constraint_data_get_constraint_info_util_stops_at_an_end_atom_or_an_empty_range() {
        assert_eq!(
            info_00d21a60(&[(0, 5), (8, 1)], 0x10),
            [0, 0xa0, 3, 3, 0xdddd, 0x10]
        );
        assert_eq!(info_00d21a60(&[(0, 5)], 0), [0, 0x10, 0, 0, 0xdddd, 0]);
        assert_eq!(
            info_00d21a60(&[(0, 5), (8, 27)], 0x10),
            [0, 0xa0, 3, 3, 0xdddd, 0x10]
        );
    }

    #[test]
    #[should_panic(expected = "loops forever")]
    fn hkp_constraint_data_get_constraint_info_util_panics_where_the_game_hangs() {
        info_00d21a60(&[(0, 21)], 8);
    }
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn add_instance_clears_the_runtime_only_when_there_is_one() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(0x00cb_1280, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00d2_1a30, &args![0x10u32, 0x20u32, 0u32, 0x40u32]);
        assert!(seen.borrow().is_empty());
        e.call(0x00d2_1a30, &args![0x10u32, 0x20u32, 0x9000u32, 0x40u32]);
        assert_eq!(*seen.borrow(), [[0x9000, 0, 0x40]]);
    }

    #[test]
    fn get_solver_results_answers_the_runtime() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00d2_1a50, &args![0x10u32, 0x9876u32]).u32(),
            0x9876
        );
        assert_eq!(e.call(0x00d2_1a50, &args![0x10u32, 0u32]).u32(), 0);
    }
}
