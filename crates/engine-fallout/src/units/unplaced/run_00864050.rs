//! `(unplaced)/run_00864050` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The destructors the scalar deleting destructors here run (`thiscall`; unit
/// `fallout shared/tessaveloadgame.cpp`).
const DESTRUCTOR_00863D60: u32 = 0x0086_3d60;
const DESTRUCTOR_00857350: u32 = 0x0085_7350;
const DESTRUCTOR_00863F00: u32 = 0x0086_3f00;
/// The game's `operator delete` (`00401030`, `cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

/// A scalar deleting destructor: `destructor` on `this`, then `operator delete` when bit 0
/// of `flags` is set; `this` is returned.
fn scalar_deleting(e: &mut Engine, destructor: u32, this: Ptr, flags: u32) -> Ptr {
    e.call(destructor, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00864050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): the scalar deleting destructor over
/// `00863d60`. Returns `this`.
pub fn fn_00864050(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, DESTRUCTOR_00863D60, this, flags)
}

// Translated from 00864080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): the scalar deleting destructor over
/// `00857350`. Returns `this`.
pub fn fn_00864080(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, DESTRUCTOR_00857350, this, flags)
}

// Translated from 008640b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): the scalar deleting destructor over
/// `00863f00`. Returns `this`.
pub fn fn_008640b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, DESTRUCTOR_00863F00, this, flags)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086_4050, fn_00864050(Ptr, u32) -> Ptr),
        entry!(0x0086_4080, fn_00864080(Ptr, u32) -> Ptr),
        entry!(0x0086_40b0, fn_008640b0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(address: u32, destructor: u32) {
        let mut e = Engine::new();
        e.register(destructor, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![this, 2u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(destructor, vec![this])]);
        e.call_log = Some(vec![]);
        e.call(address, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(destructor, vec![this]), (OPERATOR_DELETE, vec![this])]
        );
    }

    #[test]
    fn fn_00864050_destroys_and_deletes_with_bit_0() {
        check(0x0086_4050, DESTRUCTOR_00863D60);
    }

    #[test]
    fn fn_00864080_destroys_and_deletes_with_bit_0() {
        check(0x0086_4080, DESTRUCTOR_00857350);
    }

    #[test]
    fn fn_008640b0_destroys_and_deletes_with_bit_0() {
        check(0x0086_40b0, DESTRUCTOR_00863F00);
    }
}
