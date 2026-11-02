//! `Havok SDK/run_00e4ae00` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00e4aaa0`: thiscall on a 16-byte element array, `(allocator, index, remove count, items,
/// insert count)`, `RET 0x14`.
const SPLICE_ELEMENTS: u32 = 0x00e4_aaa0;

// Translated from 00e4ae00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x10`: inserts `count` elements from `items` at `index` of the array at `this`:
/// `00e4aaa0` with `this`, `allocator`, `index`, a remove count of 0, `items` and `count`.
pub fn fn_00e4ae00(e: &mut Engine, this: Ptr, allocator: u32, index: u32, items: u32, count: u32) {
    e.call(
        SPLICE_ELEMENTS,
        &args![this, allocator, index, 0u32, items, count],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e4_ae00, fn_00e4ae00(Ptr, u32, u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn insert_is_a_splice_without_removal() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(SPLICE_ELEMENTS, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(
            0x00e4_ae00,
            &args![0x1000u32, 0x2000u32, 3u32, 0x4000u32, 5u32],
        );
        assert_eq!(*seen.borrow(), vec![vec![0x1000, 0x2000, 3, 0, 0x4000, 5]]);
    }
}
