//! `Havok SDK/run_00cdb2f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The setup call `00cdb2f0` ends with (`00cdb150`, `this` and three stack words, cleaned by
/// the caller); named by neither the map nor Ghidra.
const SETUP: u32 = 0x00cd_b150;
/// The capacity word of an `hkArray` that does not own its memory (bit 31 set).
const NOT_OWNED: u32 = 0x8000_0000;

// Translated from 00cdb2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of an object that keeps four `hkArray`-shaped members (the class is
/// unconfirmed): `+0` takes `arg_1`; empty arrays (data 0, size 0, capacity word `0x80000000`)
/// are written at `+4`, `+0x10`, `+0x1c` and `+0x2c` (the word at `+0x28` is not written);
/// then `00cdb150` runs on `this` with `(arg_0, arg_1, array)` where `array` is a temporary
/// `hkArray` on the stack holding data `arg_2`, size `arg_3` and capacity word `arg_3 |
/// 0x80000000` (not owned). Returns `this`.
pub fn fn_00cdb2f0(
    e: &mut Engine,
    this: Ptr,
    arg_0: u32,
    arg_1: u32,
    arg_2: u32,
    arg_3: u32,
) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, arg_1);
    for array in [0x4, 0x10, 0x1c, 0x2c] {
        e.mem.set_u32(base + array, 0);
        e.mem.set_u32(base + array + 4, 0);
        e.mem.set_u32(base + array + 8, NOT_OWNED);
    }
    e.with_stack(12, |e, array| {
        let array = array.addr();
        e.mem.set_u32(array, arg_2);
        e.mem.set_u32(array + 4, arg_3);
        e.mem.set_u32(array + 8, arg_3 | NOT_OWNED);
        e.call(SETUP, &args![this, arg_0, arg_1, array]);
    });
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cd_b2f0, fn_00cdb2f0(Ptr, u32, u32, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The members are initialised (the word at `+0x28` untouched) and the setup runs with
    /// `this` and the temporary array filled in.
    #[test]
    fn initialises_four_empty_arrays_and_runs_the_setup() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SETUP, move |e, a| {
            let array = (e.mem.u32(a[3]), e.mem.u32(a[3] + 4), e.mem.u32(a[3] + 8));
            sink.borrow_mut().push((a[..4].to_vec(), array));
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.write(this, &[0xcc; 0x40]);
        let got = e
            .call(
                0x00cd_b2f0,
                &args![this, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
            )
            .u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), 0x22);
        for array in [0x4, 0x10, 0x1c, 0x2c] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x28), 0xcccc_cccc);
        // The address of the temporary array is not known to the test.
        let log = seen.borrow();
        let (words, array) = &log[0];
        assert_eq!(words[..3], [this, 0x11, 0x22]);
        assert_ne!(words[3], 0);
        assert_eq!(*array, (0x33, 0x44, 0x8000_0044));
    }
}
