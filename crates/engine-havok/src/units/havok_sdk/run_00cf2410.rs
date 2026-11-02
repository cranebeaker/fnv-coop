//! `Havok SDK/run_00cf2410` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// A function of the record at `this + 0x40` (`00cf2280`, thiscall, `(first, second)`); its
/// role is unconfirmed.
const INIT_PART: u32 = 0x00cf_2280;
/// A function of the record itself (`00cb2d90`, thiscall, one pointer); its role is
/// unconfirmed.
const INIT_SELF: u32 = 0x00cb_2d90;

// Translated from 00cf2410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall `(this, first, second)`; the callee pops two words. Calls `00cf2280` on
/// `this + 0x40` with both words, then `00cb2d90` on `this` with `second`, stores the vector at
/// `first` (read after the calls) at `this + 0x30`, the float `1.0` at `this + 0xa0` and a
/// zero vector at `this + 0x90`. The class is unconfirmed; the Ghidra decompiler shows the
/// second call without its `this`, the disassembly passes `this` in ECX.
pub fn fn_00cf2410(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let t = this.addr();
    e.call(INIT_PART, &args![t + 0x40, first, second]);
    e.call(INIT_SELF, &args![this, second]);
    let vector = [0, 4, 8, 12].map(|o| e.mem.u32(first.addr() + o));
    e.mem.set_f32(t + 0xa0, 1.0);
    for (i, word) in vector.into_iter().enumerate() {
        e.mem.set_u32(t + 0x30 + 4 * i as u32, word);
    }
    for i in 0..4 {
        e.mem.set_u32(t + 0x90 + 4 * i, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cf_2410, fn_00cf2410(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn the_record_takes_the_vector_the_unit_float_and_a_zero_vector() {
        let mut e = Engine::new();
        let calls: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::new(RefCell::new(Vec::new()));
        for address in [INIT_PART, INIT_SELF] {
            let sink = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0xb0);
        let first = e.mem.alloc(0x10);
        let second = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_f32(first + 4 * i, 1.5 + i as f32);
            e.mem.set_u32(this + 0x90 + 4 * i, 0xffff_ffff);
        }
        e.call(0x00cf_2410, &args![this, first, second]);
        assert_eq!(
            *calls.borrow(),
            vec![
                (INIT_PART, vec![this + 0x40, first, second]),
                (INIT_SELF, vec![this, second]),
            ]
        );
        let stored: Vec<f32> = (0..4).map(|i| e.mem.f32(this + 0x30 + 4 * i)).collect();
        assert_eq!(stored, vec![1.5, 2.5, 3.5, 4.5]);
        assert_eq!(e.mem.f32(this + 0xa0), 1.0);
        assert!((0..4).all(|i| e.mem.u32(this + 0x90 + 4 * i) == 0));
    }
}
