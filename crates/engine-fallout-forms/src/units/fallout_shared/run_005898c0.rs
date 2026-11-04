//! `fallout shared/run_005898c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005898c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005898c0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0xe (0x54 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005898c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.mem.set_u32(base, 0x0103_20fc);
    e.call(0x004f_15a0, &args![this, 0xe]);
    e.call(0x0058_9980, &args![this]);
    this
}

// Translated from 00589930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00589960` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00589930(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0058_9960, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00589980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the start values of the form `fn_005898c0` constructs (the vtable
/// it stores, `010320fc`, has the RTTI name `.?AVBGSAcousticSpace@@`): the
/// words at +0x34..+0x44, +0x48 and +0x50 to 0, the byte at +0x30 to 1 and
/// the word at +0x4c to 1. `ECX`, no stack words. The engine map gives it
/// no name.
pub fn fn_00589980(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0x34, 0x38, 0x3c, 0x40, 0x44, 0x50] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u8(base + 0x30, 1);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x4c, 1);
}

// Translated from 00589960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body `fn_00589930` runs: stores the vtable `010320fc`
/// back and runs the base destructor `0050e9f0` (tesobject.cpp). `ECX`, no
/// stack words. The engine map gives it no name.
pub fn fn_00589960(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0103_20fc);
    e.call(0x0050_e9f0, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_98c0, fn_005898c0(Ptr) -> Ptr),
        entry!(0x0058_9930, fn_00589930(Ptr, u32) -> Ptr),
        entry!(0x0058_9980, fn_00589980(Ptr)),
        entry!(0x0058_9960, fn_00589960(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005898c0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_98c0,
            size: 0x54,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xe)]),
                (0x0058_9980, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_20fc)],
        });
    }
    #[test]
    fn at_00589930_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0058_9960, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0058_9930, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0058_9930)
                .collect();
            let mut expected = vec![(0x0058_9960, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;

    #[test]
    fn the_start_values() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x54);
        e.mem.write(object, &[0xaa; 0x54]);
        e.call(0x0058_9980, &[object]);
        for offset in [0x34, 0x38, 0x3c, 0x40, 0x44, 0x48, 0x50] {
            assert_eq!(e.mem.u32(object + offset), 0);
        }
        assert_eq!(e.mem.u8(object + 0x30), 1);
        assert_eq!(e.mem.u8(object + 0x31), 0xaa);
        assert_eq!(e.mem.u32(object + 0x4c), 1);
        assert_eq!(e.mem.u32(object + 0x2c), 0xaaaa_aaaa);
    }

    #[test]
    fn the_destructor_restores_the_vtable_and_runs_the_base() {
        let mut e = Engine::new();
        e.register(0x0050_e9f0, |_, _| Ret::default());
        let object = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        e.call(0x0058_9960, &[object]);
        assert_eq!(e.mem.u32(object), 0x0103_20fc);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0050_e9f0, vec![object])]);
    }
}
