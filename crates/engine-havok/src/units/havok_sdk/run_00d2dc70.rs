//! `Havok SDK/run_00d2dc70` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d2dc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `queue`; descriptions, not confirmed): registers three groups of
/// functions with `00cfbbe0` on `queue`. Each call passes the address of one 0x14-byte record on
/// the stack (four function words, then two bytes) and two numbers: `(0x00d2e680, 0x00d2e970,
/// 0x00d2e9b0, 0x00d2daf0)` with bytes `(1, 1)` and `(0xa, 0x12)`; `(0x00d2df50, 0x00d312f0,
/// 0x00d31110, 0x00d2f0d0)` with `(0, 1)` and `(0x12, 0xa)`; `(0x00d2f240, 0x00d312f0,
/// 0x00d31110, 0x00d2f0d0)` with `(0, 1)` and `(0xa, 0xa)`. The record's last two padding bytes
/// are left uninitialised by the exe (zero here).
pub fn fn_00d2dc70(e: &mut Engine, queue: Ptr) {
    e.with_stack(0x14, |e, record| {
        let r = record.addr();
        let groups = [
            (
                [0x00d2_e680u32, 0x00d2_e970, 0x00d2_e9b0, 0x00d2_daf0],
                1u8,
                0xau32,
                0x12u32,
            ),
            (
                [0x00d2_df50, 0x00d3_12f0, 0x00d3_1110, 0x00d2_f0d0],
                0,
                0x12,
                0xa,
            ),
            (
                [0x00d2_f240, 0x00d3_12f0, 0x00d3_1110, 0x00d2_f0d0],
                0,
                0xa,
                0xa,
            ),
        ];
        for (functions, first_flag, first_number, second_number) in groups {
            for (index, function) in functions.into_iter().enumerate() {
                e.mem.set_u32(r + index as u32 * 4, function);
            }
            e.mem.set_u8(r + 0x10, first_flag);
            e.mem.set_u8(r + 0x11, 1);
            e.call(
                0x00cf_bbe0,
                &args![queue, record, first_number, second_number],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d2_dc70, fn_00d2dc70(Ptr))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00d2dc70_registers_the_three_groups() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00cf_bbe0, move |e, a| {
            let mut words = a.to_vec();
            for index in 0..4 {
                words.push(e.mem.u32(a[1] + index * 4));
            }
            words.push(u32::from(e.mem.u8(a[1] + 0x10)));
            words.push(u32::from(e.mem.u8(a[1] + 0x11)));
            log.borrow_mut().push(words);
            Ret::default()
        });
        let queue = e.mem.alloc(0x10);
        e.call(0x00d2_dc70, &args![queue]);
        let seen = seen.borrow();
        let expected = [
            [
                0x00d2_e680,
                0x00d2_e970,
                0x00d2_e9b0,
                0x00d2_daf0,
                1,
                1,
                0xa,
                0x12,
            ],
            [
                0x00d2_df50,
                0x00d3_12f0,
                0x00d3_1110,
                0x00d2_f0d0,
                0,
                1,
                0x12,
                0xa,
            ],
            [
                0x00d2_f240,
                0x00d3_12f0,
                0x00d3_1110,
                0x00d2_f0d0,
                0,
                1,
                0xa,
                0xa,
            ],
        ];
        assert_eq!(seen.len(), 3);
        for (call, group) in seen.iter().zip(expected) {
            assert_eq!(call[0], queue);
            assert_eq!(call[2..4], group[6..8]);
            assert_eq!(call[4..], group[..6]);
        }
    }
}
