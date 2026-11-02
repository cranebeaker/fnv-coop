//! `fallout shared/run_00617da0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00617da0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00617da0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0xf (0x60 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00617da0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_2c50, &args![base + 0x18]);
    e.call(0x0048_e270, &args![base + 0x20]);
    e.mem.set_u32(base, 0x0104_cc0c);
    e.mem.set_u32(base + 0x18, 0x0104_cbf4);
    e.mem.set_u32(base + 0x20, 0x0104_cbd4);
    // `vector constructor iterator` is stdcall with four words (`RET 0x10`): no `this`.
    e.call(0x0040_1050, &args![base + 0x40, 0x8, 0x4, 0x0048_2c50]);
    e.call(0x004f_15a0, &args![this, 0xf]);
    e.call(0x0061_7e80, &args![this]);
    this
}

// Translated from 00617e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00617ea0`, a reset routine of the 0x60-byte form
/// (it sets fields in `+0x2c..+0x3c`, calls four stored function pointers, vtable slot +200
/// with 0, then `00460110`), and returns nothing. Parameters: one word, `this`; the
/// disassembly of the function ends in a plain `RET` and passes only `ECX` on. `00617ea0`
/// ends with `CALL 00460110` and then `RET`; `00460110` ends with a conditional call and no
/// result (Ghidra types both void).
pub fn fn_00617e80(e: &mut Engine, this: Ptr) {
    e.call(0x0061_7ea0, &args![this]);
}

/// `00460110` (unnamed; `thiscall`, no stack words): what the reset runs last.
const RESET_TAIL_00460110: u32 = 0x0046_0110;

// Translated from 00617ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the reset of the 0x60-byte form
/// [`fn_00617da0`] builds. Stores -1 at +0x2C, 0 at +0x34, 5 at +0x30 and 1.0 (`FLD1`) at
/// +0x38 and +0x3C; calls slot +0 of the vtable of each of the four 8-byte components at
/// +0x40 (with the component as `this`); then the form's own vtable slot +0xC8 with 0, and
/// `00460110`.
pub fn fn_00617ea0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x2c, 0xffff_ffff);
    e.mem.set_u32(base + 0x34, 0);
    e.mem.set_u32(base + 0x30, 5);
    for index in 0..2u32 {
        e.mem.set_u32(base + 0x38 + 4 * index, x87_store32(1.0));
    }
    for index in 0..4u32 {
        e.vcall(base + 0x40 + 8 * index, 0, &args![]);
    }
    e.vcall(base, 0xc8, &args![0u32]);
    e.call(RESET_TAIL_00460110, &args![this]);
}

// Translated from 00617e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the form `fn_00617da0` builds: runs
/// `fn_00617f40` on `this`, then frees `this` with `00401030` when bit 0 of
/// `flags` is set. Returns `this` (`RET 4`).
pub fn fn_00617e50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00617f40(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00617f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00617f40 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated
/// The destructor body of the form `fn_00617da0` builds: stores the
/// vtables back (`0104cc0c`, +0x18 `0104cbf4`, +0x20 `0104cbd4`), destroys
/// the component at +0x20 (`0048e2e0`) and the form (`TESForm::~TESForm`,
/// `00483630`).
pub fn fn_00617f40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_cc0c);
    e.mem.set_u32(base + 0x18, 0x0104_cbf4);
    e.mem.set_u32(base + 0x20, 0x0104_cbd4);
    e.call(0x0048_e2e0, &args![base + 0x20]);
    e.call(0x0048_3630, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0061_7e80, fn_00617e80(Ptr)),
        entry!(0x0061_7da0, fn_00617da0(Ptr) -> Ptr),
        entry!(0x0061_7ea0, fn_00617ea0(Ptr)),
        entry!(0x0061_7e50, fn_00617e50(Ptr, u32) -> Ptr),
        entry!(0x0061_7f40, fn_00617f40(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00617da0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0061_7da0,
            size: 0x60,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_2c50, &[A::T(0x18)]),
                (0x0048_e270, &[A::T(0x20)]),
                (
                    0x0040_1050,
                    &[A::T(0x40), A::V(0x8), A::V(0x4), A::V(0x0048_2c50)],
                ),
                (0x004f_15a0, &[A::T(0x0), A::V(0xf)]),
                (0x0061_7e80, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_cc0c),
                (0x18, 4, 0x0104_cbf4),
                (0x20, 4, 0x0104_cbd4),
            ],
        });
    }
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_00617e80_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0061_7ea0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0061_7e80, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0061_7ea0), vec![vec![0x5000]]);
    }

    #[test]
    fn reset_sets_the_fields_and_calls_the_components() {
        let mut e = Engine::new();
        e.put_vtable(0x0e00_0000, &[0x0e10_0000]);
        e.register(0x0e10_0000, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x33];
        slots[0x32] = 0x0e20_0000;
        e.put_vtable(0x0e30_0000, &slots);
        e.register(0x0e20_0000, |_, _| Ret::default());
        e.register(RESET_TAIL_00460110, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this, 0x0e30_0000);
        for index in 0..4 {
            e.mem.set_u32(this + 0x40 + 8 * index, 0x0e00_0000);
        }
        e.call_log = Some(vec![]);
        e.call(0x0061_7ea0, &args![this]);
        assert_eq!(
            [0x2cu32, 0x30, 0x34, 0x38, 0x3c].map(|o| e.mem.u32(this + o)),
            [0xffff_ffff, 5, 0, 0x3f80_0000, 0x3f80_0000]
        );
        let components: Vec<u32> = (0..4).map(|i| this + 0x40 + 8 * i).collect();
        let called: Vec<u32> = calls_to(&e, 0x0e10_0000).iter().map(|w| w[0]).collect();
        assert_eq!(called, components);
        assert_eq!(calls_to(&e, 0x0e20_0000), vec![vec![this, 0]]);
        assert_eq!(calls_to(&e, RESET_TAIL_00460110), vec![vec![this]]);
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn destructor_restores_vtables_and_deletes_with_flag_1() {
        let mut e = Engine::new();
        for addr in [0x0048_e2e0, 0x0048_3630, 0x0040_1030] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0061_7e50, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0048_e2e0, vec![this + 0x20]),
                (0x0048_3630, vec![this]),
                (0x0040_1030, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0104_cc0c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0104_cbf4);
        assert_eq!(e.mem.u32(this + 0x20), 0x0104_cbd4);
        e.call_log = Some(vec![]);
        e.call(0x0061_7e50, &args![this, 2u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != 0x0040_1030));
    }
}
