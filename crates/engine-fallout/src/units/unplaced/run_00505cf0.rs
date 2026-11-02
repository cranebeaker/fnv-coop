//! `(unplaced)/run_00505cf0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00505cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00505d20` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00505cf0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0050_5d20, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `0048e2e0` (`thiscall`, no stack word): the member destructor run on +0x164, +0x158 and
/// +0x14c.
const MEMBER_DESTRUCTOR_0048E2E0: u32 = 0x0048_e2e0;
/// `TESForm::~TESForm` (`00483630`, `thiscall`).
const TES_FORM_DESTRUCTOR: u32 = 0x0048_3630;

// Translated from 00505d20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00505d20 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef3ad4, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word; the destructor `fn_00505cf0` runs):
/// sets the vtable `0102685c`, runs `0048e2e0` on +0x164, +0x158 and +0x14c, then
/// `TESForm::~TESForm` (`00483630`).
pub fn fn_00505d20(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0102_685c);
    for offset in [0x164u32, 0x158, 0x14c] {
        e.call(MEMBER_DESTRUCTOR_0048E2E0, &args![this.addr() + offset]);
    }
    e.call(TES_FORM_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_5cf0, fn_00505cf0(Ptr, u32) -> Ptr),
        entry!(0x0050_5d20, fn_00505d20(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00505cf0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0050_5d20, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0050_5cf0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0050_5cf0)
                .collect();
            let mut expected = vec![(0x0050_5d20, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00505d20_destroys_the_three_members_then_the_form() {
        let mut e = Engine::new();
        e.register(MEMBER_DESTRUCTOR_0048E2E0, |_, _| Ret::default());
        e.register(TES_FORM_DESTRUCTOR, |_, _| Ret::default());
        let this = e.mem.alloc(0x170);
        e.call_log = Some(vec![]);
        e.call(0x0050_5d20, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (MEMBER_DESTRUCTOR_0048E2E0, vec![this + 0x164]),
                (MEMBER_DESTRUCTOR_0048E2E0, vec![this + 0x158]),
                (MEMBER_DESTRUCTOR_0048E2E0, vec![this + 0x14c]),
                (TES_FORM_DESTRUCTOR, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_685c);
    }
}
