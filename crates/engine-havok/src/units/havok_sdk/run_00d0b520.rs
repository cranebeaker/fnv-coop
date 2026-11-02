//! `Havok SDK/run_00d0b520` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c91a70` and `00c918e0` (`this` = the object at `this + 0xc`, one word each) and `00c905b0`
/// (`this` only: removes a reference): the calls `00d0b530` makes.
const OBJECT_CALL_FIRST: u32 = 0x00c9_1a70;
const OBJECT_CALL_SECOND: u32 = 0x00c9_18e0;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00d0b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method that does nothing (its only reference is the vtable slot at `010cd3a8`; the
/// class is unconfirmed): the `RET 4` takes one word it never reads.
pub fn fn_00d0b520(_e: &mut Engine, _this: Ptr, _unused_1: u32) {}

// Translated from 00d0b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method of the second table of a multiply-inheriting object (vtable slot at
/// `010cd3b8`; `this` is the sub-object at `+8` of the object, the class is unconfirmed; the
/// `RET 4` word is never read). With `owner = this - 8` and `target` the word at `this + 0xc`:
/// `00c91a70` runs on `target` with `this` (0 when `owner` is 0), then `00c918e0` on `target`
/// (read again) with `this + 8` (0 when `owner` is 0), and a reference is removed from `owner`
/// (`00c905b0`).
pub fn fn_00d0b530(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let base = this.addr();
    let owner = base.wrapping_sub(8);
    let target = e.mem.u32(base + 0xc);
    let first = if owner != 0 { base } else { 0 };
    e.call(OBJECT_CALL_FIRST, &args![target, first]);
    let second = if owner != 0 { base.wrapping_add(8) } else { 0 };
    let target = e.mem.u32(base + 0xc);
    e.call(OBJECT_CALL_SECOND, &args![target, second]);
    e.call(REMOVE_REFERENCE, &args![owner]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_b520, fn_00d0b520(Ptr, u32)),
        entry!(0x00d0_b530, fn_00d0b530(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The three calls run in order with the adjusted pointers (null when the owner would be);
    /// the no-op does nothing.
    #[test]
    fn adjusts_the_pointers_and_removes_the_reference() {
        let mut e = Engine::new();
        let log = Rc::new(RefCell::new(Vec::new()));
        for (address, name) in [
            (OBJECT_CALL_FIRST, "first"),
            (OBJECT_CALL_SECOND, "second"),
            (REMOVE_REFERENCE, "remove"),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((name, a.to_vec()));
                Ret::default()
            });
        }
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(block + 0x14, 0x5555);
        let this = block + 8;
        e.call(0x00d0_b530, &args![this, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                ("first", vec![0x5555, this]),
                ("second", vec![0x5555, this + 8]),
                ("remove", vec![block])
            ]
        );
        // `this - 8` null: both words are null and the owner is 0.
        log.borrow_mut().clear();
        e.map(0x0, 0x1000);
        e.mem.set_u32(0x8 + 0xc, 0x6666);
        e.call(0x00d0_b530, &args![8u32, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                ("first", vec![0x6666, 0]),
                ("second", vec![0x6666, 0]),
                ("remove", vec![0])
            ]
        );
        log.borrow_mut().clear();
        e.call(0x00d0_b520, &args![this, 0u32]);
        assert!(log.borrow().is_empty());
    }
}
