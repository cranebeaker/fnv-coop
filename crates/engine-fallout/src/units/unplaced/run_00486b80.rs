//! `(unplaced)/run_00486b80` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Fields of the array `00486b80` works on (32-bit fields): the element
/// buffer (`+4`), one past the highest used index (`+0xc`) and the number
/// of non-null elements (`+0x10`).
const BUFFER: u32 = 0x04;
const SIZE: u32 = 0x0c;
const NON_NULL_COUNT: u32 = 0x10;

// Translated from 00486b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the element at `index` and answers it: below the used size,
/// clears the slot, decrements the non-null count when the element was
/// non-null, and shrinks the used size by one when `index` was the last
/// used slot; otherwise answers 0. `TESForm::RemoveFromDataStructures`
/// (`00483c70`) calls it on the array the global `011c54c4` points to.
pub fn fn_00486b80(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = this.addr();
    let size = e.mem.u32(base + SIZE);
    if index >= size {
        return 0;
    }
    let slot = e.mem.u32(base + BUFFER).wrapping_add(index.wrapping_mul(4));
    let element = e.mem.u32(slot);
    e.mem.set_u32(slot, 0);
    if element != 0 {
        let count = e.mem.u32(base + NON_NULL_COUNT).wrapping_sub(1);
        e.mem.set_u32(base + NON_NULL_COUNT, count);
    }
    let size = e.mem.u32(base + SIZE);
    if index == size.wrapping_sub(1) {
        e.mem.set_u32(base + SIZE, size.wrapping_sub(1));
    }
    element
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00486b80, fn_00486b80(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn array(e: &mut Engine, elements: &[u32]) -> u32 {
        let this = e.mem.alloc(0x18);
        let buffer = e.mem.alloc(4 * elements.len() as u32);
        for (i, value) in elements.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *value);
        }
        e.mem.set_u32(this + BUFFER, buffer);
        e.mem.set_u32(this + SIZE, elements.len() as u32);
        let non_null = elements.iter().filter(|v| **v != 0).count() as u32;
        e.mem.set_u32(this + NON_NULL_COUNT, non_null);
        this
    }

    #[test]
    fn removes_a_middle_element() {
        let mut e = Engine::new();
        let this = array(&mut e, &[0x10, 0x20, 0x30]);
        assert_eq!(e.call(0x0048_6b80, &[this, 1]).u32(), 0x20);
        let buffer = e.mem.u32(this + BUFFER);
        assert_eq!(e.mem.u32(buffer + 4), 0);
        assert_eq!(e.mem.u32(this + NON_NULL_COUNT), 2);
        assert_eq!(e.mem.u32(this + SIZE), 3);
    }

    #[test]
    fn removing_the_last_shrinks_and_a_null_keeps_the_count() {
        let mut e = Engine::new();
        let this = array(&mut e, &[0x10, 0]);
        assert_eq!(e.call(0x0048_6b80, &[this, 1]).u32(), 0);
        assert_eq!(e.mem.u32(this + NON_NULL_COUNT), 1);
        assert_eq!(e.mem.u32(this + SIZE), 1);
    }

    #[test]
    fn out_of_range_answers_zero() {
        let mut e = Engine::new();
        let this = array(&mut e, &[0x10]);
        assert_eq!(e.call(0x0048_6b80, &[this, 1]).u32(), 0);
        assert_eq!(e.mem.u32(this + SIZE), 1);
        assert_eq!(e.mem.u32(this + NON_NULL_COUNT), 1);
    }
}
