//! `Havok SDK/run_00d31730` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d31730 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x10`: four stack words `first`, `second`, `third`, `fourth`; the
/// names are descriptions, not confirmed): builds a 12-byte record on the stack (vtable word
/// `0x010c_e7fc`, the float at `0104da48` copied through `FLD`/`FSTP` at `+4`, `fourth` at `+8`)
/// and calls `00d315d0` on `this` with `(second, first, third, record)`.
pub fn fn_00d31730(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32, fourth: u32) {
    e.with_stack(0xc, |e, record| {
        let r = record.addr();
        e.mem.set_u32(r + 8, fourth);
        let large = x87_quiet_bits(e.mem.u32(0x0104_da48));
        e.mem.set_u32(r + 4, large);
        e.mem.set_u32(r, 0x010c_e7fc);
        e.call(0x00d3_15d0, &args![this, second, first, third, record]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d3_1730, fn_00d31730(Ptr, u32, u32, u32, u32))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn fn_00d31730_builds_the_record_and_calls_00d315d0() {
        let mut e = Engine::new();
        e.map(0x0104_d000, 0x1000);
        e.mem.set_u32(0x0104_da48, 0x7f7f_ffee);
        let seen = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d3_15d0, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..3 {
                words.push(e.mem.u32(a[4] + word * 4));
            }
            log.borrow_mut().push(words);
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.call(0x00d3_1730, &args![this, 1u32, 2u32, 3u32, 4u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][..4], [this, 2, 1, 3]);
        assert_eq!(seen[0][5..], [0x010c_e7fc, 0x7f7f_ffee, 4]);
    }
}
