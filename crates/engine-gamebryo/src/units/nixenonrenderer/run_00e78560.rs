//! `NiXenonRenderer/run_00e78560` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00ba3aa0` (`thiscall`, `RET 8`): stores the word `value` points to at an index of an array.
const ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// `0126f550`: the outer table (entries are objects with an array at `+4`).
const OUTER_TABLE_SLOT: u32 = 0x0126_f550;
/// The handler for the number 0 (and the "none of the groups" default of the later
/// groups): `00e7cd40`.
const DEFAULT_HANDLER: u32 = 0x00e7_cd40;
/// The case of each number (1 to 0x20) for the first group of numbers 4 (`T2` of the exe).
const FOURTH_GROUP_CASES: [u8; 32] = [
    0, 0, 1, 1, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 5, 6, 6, 6, 6, 7, 7, 7, 7, 1, 1, 1, 1,
];
/// Handlers by case for the first number 4.
const FOURTH_GROUP_HANDLERS: [u32; 8] = [
    0x00e7_cd40,
    0x00e7_9d00,
    0x00e7_8880,
    0x00e7_8880,
    0x00e7_8910,
    0x00e7_8a70,
    0x00e7_9f40,
    0x00e7_a0a0,
];
/// Handlers by case (the number divided by 4) for the first number 5.
const FIFTH_GROUP_HANDLERS: [u32; 8] = [
    0x00e7_7950,
    0x00e7_7b50,
    0x00e7_7d00,
    0x00e7_7eb0,
    0x00e7_8060,
    0x00e7_8210,
    0x00e7_83b0,
    0x00e7_cd40,
];
/// Handlers by case for the first numbers 6 and 7.
const SIXTH_GROUP_HANDLERS: [u32; 8] = [
    0x00e7_6c30,
    0x00e7_6e30,
    0x00e7_7010,
    0x00e7_71f0,
    0x00e7_73b0,
    0x00e7_7590,
    0x00e7_7770,
    0x00e7_cd40,
];
/// Handlers by case for the first numbers 0 to 3 (case 0 depends on the number: see
/// `handler_for`).
const FIRST_GROUP_HANDLERS: [u32; 8] = [
    0,
    0x00e7_9160,
    0x00e7_9370,
    0x00e7_9540,
    0x00e7_9760,
    0x00e7_9920,
    0x00e7_9b40,
    0x00e7_8e30,
];

/// The handler address the exe's switch picks for the pair (`first`, `second`), 0 for none.
/// `second` 1 to 0x20 select by `first`: 0 to 3, 4, 5, 6 and 7 are the groups; anything
/// else gets none.
fn handler_for(first: u32, second: u32) -> u32 {
    let key = second.wrapping_sub(1);
    if key > 0x1f {
        return 0;
    }
    let case = (key >> 2) as usize;
    if first <= 3 {
        if case == 0 {
            return if key == first {
                0x00e7_8f80
            } else if key < first {
                0x00e7_8e30
            } else {
                0x00e7_8d50
            };
        }
        FIRST_GROUP_HANDLERS[case]
    } else if first == 4 {
        FOURTH_GROUP_HANDLERS[FOURTH_GROUP_CASES[key as usize] as usize]
    } else if first == 5 {
        FIFTH_GROUP_HANDLERS[case]
    } else if first == 6 || first == 7 {
        SIXTH_GROUP_HANDLERS[case]
    } else {
        0
    }
}

// Translated from 00e78560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words, `first` and `second`): the entry
/// `first` of the outer table at `0126f550` has an array at `+4`; stores in it, at index
/// `second` (through `00ba3aa0` with the address of a word holding the handler), the handler
/// the exe's switch picks (`handler_for`): `00e7cd40` for `second` 0; none (nothing is
/// stored) for `second` above 0x20 or a `first` of 8 or more. The switch reads its case
/// numbers from byte and word tables in the exe's code, which are translated here as the
/// tables above.
pub fn fn_00e78560(e: &mut Engine, first: u32, second: u32) {
    let table = e.mem.u32(OUTER_TABLE_SLOT);
    let array = e.mem.u32(table + 4);
    let entry = e.mem.u32(array.wrapping_add(first.wrapping_mul(4)));
    let handler = if second == 0 {
        DEFAULT_HANDLER
    } else {
        handler_for(first, second)
    };
    if second != 0 && handler == 0 {
        return;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), handler);
        e.call(ARRAY_SET_AT, &args![entry + 4, second, local]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e7_8560, fn_00e78560(u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handlers_follow_the_switch() {
        // First group: case 0 depends on the number against `first`.
        assert_eq!(handler_for(0, 1), 0x00e7_8f80);
        assert_eq!(handler_for(1, 1), 0x00e7_8e30);
        assert_eq!(handler_for(0, 2), 0x00e7_8d50);
        assert_eq!(handler_for(3, 4), 0x00e7_8f80);
        assert_eq!(handler_for(2, 5), 0x00e7_9160);
        assert_eq!(handler_for(1, 0x20), 0x00e7_8e30);
        // Fourth group by its byte table.
        assert_eq!(handler_for(4, 1), 0x00e7_cd40);
        assert_eq!(handler_for(4, 3), 0x00e7_9d00);
        assert_eq!(handler_for(4, 5), 0x00e7_8880);
        assert_eq!(handler_for(4, 16), 0x00e7_8910);
        assert_eq!(handler_for(4, 20), 0x00e7_8a70);
        assert_eq!(handler_for(4, 25), 0x00e7_a0a0);
        assert_eq!(handler_for(4, 29), 0x00e7_9d00);
        // Fifth, sixth and seventh.
        assert_eq!(handler_for(5, 1), 0x00e7_7950);
        assert_eq!(handler_for(5, 0x20), 0x00e7_cd40);
        assert_eq!(handler_for(6, 0x15), 0x00e7_7590);
        assert_eq!(handler_for(7, 1), 0x00e7_6c30);
        // Outside: nothing.
        assert_eq!(handler_for(8, 1), 0);
        assert_eq!(handler_for(0x11, 5), 0);
        assert_eq!(handler_for(0, 0x21), 0);
        assert_eq!(handler_for(0, 0), 0);
    }

    #[test]
    fn stores_the_handler_in_the_entry_array() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let table = e.mem.alloc(8);
        let array = e.mem.alloc(0x20);
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(table + 4, array);
        e.mem.set_u32(array + 8, entry);
        e.mem.set_u32(OUTER_TABLE_SLOT, table);
        let stores = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let log = stores.clone();
        e.register_double(ARRAY_SET_AT, move |e, a| {
            log.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        e.call(0x00e7_8560, &args![2u32, 0u32]);
        e.call(0x00e7_8560, &args![2u32, 4u32]);
        e.call(0x00e7_8560, &args![2u32, 0x21u32]);
        assert_eq!(
            *stores.borrow(),
            vec![(entry + 4, 0, 0x00e7_cd40), (entry + 4, 4, 0x00e7_8d50),]
        );
    }
}
