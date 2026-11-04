//! `fallout shared/run_00680fe0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00407a90` (cdecl: address, a byte passed as a word): swaps the bytes of
/// the 16-bit word at the address.
const SWAP_HALF: u32 = 0x0040_7a90;
/// `00401080` (cdecl: address, a byte passed as a word): swaps the bytes of
/// the word at the address.
const SWAP_WORD: u32 = 0x0040_1080;

// Translated from 00681110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the byte order of a condition item's function data (`ECX`, no
/// stack words; `Data.FunctionData`, Xbox PDB): the 16-bit function number
/// at +0 (`00407a90`), then the parameter words at +4 and +8 (`00401080`),
/// each with a zero byte. The engine map gives it no name.
pub fn fn_00681110(e: &mut Engine, this: Ptr) {
    e.call(SWAP_HALF, &args![this, 0u32]);
    e.call(SWAP_WORD, &args![this.addr() + 4, 0u32]);
    e.call(SWAP_WORD, &args![this.addr() + 8, 0u32]);
}

// Translated from 00680fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list `006815c0` answers for `this`: while it is not null
/// and not empty (`008256d0`), deletes the head item (`007b3fa0` with flag
/// 1) when it is not null and takes the head off (`0063f7b0`). The engine
/// map gives it no name.
pub fn fn_00680fe0(e: &mut Engine, this: Ptr) {
    let list = e.call(0x0068_15c0, &args![this]).u32();
    while list != 0 && !e.call(0x0082_56d0, &args![list]).bool() {
        let slot = e.call(0x0068_15c0, &args![list]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            e.call(0x007b_3fa0, &args![item, 1u32]);
        }
        e.call(0x0063_f7b0, &args![list]);
    }
}

// Translated from 00681050 (decompiled, FalloutNV.exe 1.4.0.525)
/// A condition's comparison (cdecl: operator, value, comparison value):
/// 0 equal, 1 not equal, 2 greater, 3 greater or equal, 4 less, 5 less or
/// equal; any other operator holds. The x87 flag tests make an unordered
/// pair (a NaN) fail 0 and hold for 1 to 5. The engine map gives it no
/// name.
pub fn fn_00681050(_e: &mut Engine, operator: u32, value: f32, comparand: f32) -> bool {
    let value = f64::from(value);
    let comparand = f64::from(comparand);
    let unordered = value.is_nan() || comparand.is_nan();
    match operator {
        // FUCOMPP; TEST AH,0x44; JNP: C3 alone.
        0 => !unordered && value == comparand,
        // TEST AH,0x44; JP: C3 and C2 both clear or both set.
        1 => unordered || value != comparand,
        // FCOMPP; TEST AH,1; JNZ: C0 (comparand below value, or unordered).
        2 => unordered || comparand < value,
        // TEST AH,0x41; JNZ: C0 or C3.
        3 => unordered || comparand <= value,
        // TEST AH,0x41; JP: C0 and C3 both clear or both set.
        4 => unordered || comparand > value,
        // TEST AH,5; JP: C0 and C2 both clear or both set.
        5 => unordered || comparand >= value,
        _ => true,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0068_1110, fn_00681110(Ptr)),
        entry!(0x0068_0fe0, fn_00680fe0(Ptr)),
        entry!(0x0068_1050, fn_00681050(u32, f32, f32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps_the_number_and_both_parameters() {
        let mut e = Engine::new();
        e.register(SWAP_HALF, |_, _| Ret::default());
        e.register(SWAP_WORD, |_, _| Ret::default());
        let data: Ptr = Ptr::new(e.mem.alloc(12));
        e.call_log = Some(vec![]);
        e.call(0x0068_1110, &args![data]);
        let log = e.call_log.take().unwrap();
        let swaps: Vec<_> = log
            .iter()
            .filter(|c| c.0 == SWAP_HALF || c.0 == SWAP_WORD)
            .cloned()
            .collect();
        assert_eq!(
            swaps,
            [
                (SWAP_HALF, vec![data.addr(), 0]),
                (SWAP_WORD, vec![data.addr() + 4, 0]),
                (SWAP_WORD, vec![data.addr() + 8, 0]),
            ]
        );
    }
}

#[cfg(test)]
mod comparison_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn compare(e: &mut Engine, operator: u32, value: f32, comparand: f32) -> bool {
        e.call(0x0068_1050, &args![operator, value, comparand])
            .bool()
    }

    #[test]
    fn fn_00681050_compares_with_the_x87_flag_tests() {
        let mut e = Engine::new();
        let nan = f32::NAN;
        // (operator, below, equal, above, unordered) for value against 1.0.
        let table = [
            (0u32, false, true, false, false),
            (1, true, false, true, true),
            (2, false, false, true, true),
            (3, false, true, true, true),
            (4, true, false, false, true),
            (5, true, true, false, true),
            (6, true, true, true, true),
            (0xffff_ffff, true, true, true, true),
        ];
        for (operator, below, equal, above, unordered) in table {
            assert_eq!(compare(&mut e, operator, 0.5, 1.0), below, "{operator}");
            assert_eq!(compare(&mut e, operator, 1.0, 1.0), equal, "{operator}");
            assert_eq!(compare(&mut e, operator, 2.0, 1.0), above, "{operator}");
            assert_eq!(compare(&mut e, operator, nan, 1.0), unordered, "{operator}");
            assert_eq!(compare(&mut e, operator, 1.0, nan), unordered, "{operator}");
        }
    }

    #[test]
    fn fn_00680fe0_deletes_the_items_until_empty() {
        let mut e = Engine::new();
        let items = Rc::new(RefCell::new(vec![0x5000u32, 0, 0x6000]));
        let slot = e.mem.alloc(4);
        let list = e.mem.alloc(8);
        let holder = e.mem.alloc(8);
        e.register_double(0x0068_15c0, move |_, a| {
            if a[0] == holder {
                Ret {
                    eax: list,
                    ..Ret::default()
                }
            } else {
                Ret {
                    eax: slot,
                    ..Ret::default()
                }
            }
        });
        let i = items.clone();
        e.register_double(0x0082_56d0, move |e, _| {
            let i = i.borrow();
            if let Some(first) = i.first() {
                e.mem.set_u32(slot, *first);
            }
            Ret {
                eax: u32::from(i.is_empty()),
                ..Ret::default()
            }
        });
        let i = items;
        e.register_double(0x0063_f7b0, move |_, _| {
            i.borrow_mut().remove(0);
            Ret::default()
        });
        e.register(0x007b_3fa0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0068_0fe0, &args![holder]);
        let log = e.call_log.take().unwrap();
        let deleted: Vec<Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == 0x007b_3fa0)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(deleted, vec![vec![0x5000, 1], vec![0x6000, 1]]);
        assert_eq!(log.iter().filter(|(a, _)| *a == 0x0063_f7b0).count(), 3);

        // A null list: nothing.
        e.register(0x0068_15c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0068_0fe0, &args![holder]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != 0x0082_56d0));
    }
}
