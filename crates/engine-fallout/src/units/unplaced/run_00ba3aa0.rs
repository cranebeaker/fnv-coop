//! `(unplaced)/run_00ba3aa0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Offsets of the array object: the element pointer array (`+4`), the number of
/// slots in use (`+0xa`, 16 bits) and the number of non-null elements (`+0xc`,
/// 16 bits). The class is unconfirmed (the engine map gives it no name).
const ELEMENTS: u32 = 0x04;
const SIZE: u32 = 0x0a;
const COUNT: u32 = 0x0c;

// Translated from 00ba3aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: an index and the address of a
/// word holding the value): stores the word in slot `index` of the element array
/// at `+4`. A slot at or past the size at `+0xa` makes the size `index + 1`
/// (truncated to 16 bits). The 16-bit count at `+0xc` of non-null elements goes up
/// when a null slot receives a non-null value and down when a non-null slot
/// receives null (a slot past the old size counts as null). The value is read
/// from the word each time it is needed.
pub fn fn_00ba3aa0(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    let base = this.addr();
    let size = u32::from(e.mem.u16(base + SIZE));
    let new_value = e.mem.u32(value.addr());
    let was_null = if index >= size {
        e.mem.set_u16(base + SIZE, index.wrapping_add(1) as u16);
        true
    } else {
        let elements = e.mem.u32(base + ELEMENTS);
        e.mem.u32(elements.wrapping_add(index.wrapping_mul(4))) == 0
    };
    let count = e.mem.u16(base + COUNT);
    if new_value != 0 && was_null {
        e.mem.set_u16(base + COUNT, count.wrapping_add(1));
    } else if new_value == 0 && !was_null {
        e.mem.set_u16(base + COUNT, count.wrapping_add(0xffff));
    }
    let elements = e.mem.u32(base + ELEMENTS);
    let new_value = e.mem.u32(value.addr());
    e.mem
        .set_u32(elements.wrapping_add(index.wrapping_mul(4)), new_value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ba3aa0, fn_00ba3aa0(Ptr, u32, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An array object with a 6-slot element array, `size` slots in use and the
    /// given elements.
    fn array(e: &mut Engine, size: u16, elements: &[u32]) -> (u32, u32) {
        let object = e.mem.alloc(0x10);
        let table = e.mem.alloc(6 * 4);
        e.mem.set_u32(object + ELEMENTS, table);
        e.mem.set_u16(object + SIZE, size);
        let non_null = elements.iter().filter(|word| **word != 0).count();
        e.mem.set_u16(object + COUNT, non_null as u16);
        for (i, word) in elements.iter().enumerate() {
            e.mem.set_u32(table + 4 * i as u32, *word);
        }
        (object, table)
    }

    fn store(e: &mut Engine, object: u32, index: u32, value: u32) {
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, value);
        e.call(0x00ba3aa0, &args![object, index, word]);
    }

    #[test]
    fn a_slot_past_the_size_grows_it() {
        let mut e = Engine::new();
        let (object, table) = array(&mut e, 2, &[7, 8]);
        store(&mut e, object, 4, 0x55);
        assert_eq!(e.mem.u16(object + SIZE), 5);
        assert_eq!(e.mem.u16(object + COUNT), 3);
        assert_eq!(e.mem.u32(table + 16), 0x55);
        // A null stored past the size grows it and does not count.
        store(&mut e, object, 5, 0);
        assert_eq!(e.mem.u16(object + SIZE), 6);
        assert_eq!(e.mem.u16(object + COUNT), 3);
        assert_eq!(e.mem.u32(table + 20), 0);
    }

    #[test]
    fn the_count_follows_null_and_non_null_stores_inside_the_size() {
        let mut e = Engine::new();
        let (object, table) = array(&mut e, 4, &[7, 0, 9, 0]);
        // null slot, non-null value: up
        store(&mut e, object, 1, 0x11);
        assert_eq!(e.mem.u16(object + COUNT), 3);
        // non-null slot, non-null value: unchanged
        store(&mut e, object, 0, 0x22);
        assert_eq!(e.mem.u16(object + COUNT), 3);
        assert_eq!(e.mem.u32(table), 0x22);
        // non-null slot, null value: down
        store(&mut e, object, 2, 0);
        assert_eq!(e.mem.u16(object + COUNT), 2);
        assert_eq!(e.mem.u32(table + 8), 0);
        // null slot, null value: unchanged
        store(&mut e, object, 3, 0);
        assert_eq!(e.mem.u16(object + COUNT), 2);
        assert_eq!(e.mem.u16(object + SIZE), 4);
    }
}
