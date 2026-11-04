//! `fallout shared/run_005a7110` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005a7160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005a7160 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6e (0x70 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_6854`) is `TESLoadScreenType`, so this is the constructor of `TESLoadScreenType`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005a7160(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.mem.set_u32(base, 0x0103_6854);
    e.call(0x005a_71d0, &args![base + 0x18]);
    e.call(0x004f_15a0, &args![this, 0x6eu32]);
    this
}

// Translated from 005a72a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005a72d0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005a72a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x005a_72d0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005a7110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the list at +0x2C is empty (`008256d0`; its `EAX` passed on).
pub fn fn_005a7110(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0082_56d0, &args![this.addr() + 0x2c]).u32()
}

// Translated from 005a7130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x18 of the object at +0x34, or 0 when there is none.
pub fn fn_005a7130(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.mem.u32(this.addr() + 0x34);
    if object == 0 {
        0
    } else {
        e.mem.u32(object.wrapping_add(0x18))
    }
}

// Translated from 005a71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `TESLoadScreenType`'s data at +0x18 (`005a7160` calls
/// it): the two blocks at +4 and +0x2C (`005a7220`), then 0 at +0, 4 at
/// +0x50 and 3 at +0x54. Returns `this`.
pub fn fn_005a71d0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    fn_005a7220(e, Ptr::new(base + 4));
    fn_005a7220(e, Ptr::new(base + 0x2c));
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 0x50, 4);
    e.mem.set_u32(base + 0x54, 3);
    this
}

// Translated from 005a7220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Default values of a 0x28-byte block: four zero words, 0.0 at +0x10, 6
/// at +0x14, the `float` at `01023cd8` at +0x18, +0x1C and +0x20 (FLD/FSTP)
/// and 1 at +0x24. Returns `this`.
pub fn fn_005a7220(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [0u32, 4, 8, 0xc] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_f32(base + 0x10, 0.0);
    e.mem.set_u32(base + 0x14, 6);
    for offset in [0x18u32, 0x1c, 0x20] {
        let value = e.global::<f32>(0x0102_3cd8);
        e.mem.set_f32(base + offset, x87_quiet(value));
    }
    e.mem.set_u32(base + 0x24, 1);
    this
}

// Translated from 005a72d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `TESLoadScreenType` (the vtable `005a7160` stores):
/// puts back the vtable, then the `TESForm` destructor (`00483630`).
pub fn fn_005a72d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0103_6854);
    e.call(0x0048_3630, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005a_7160, fn_005a7160(Ptr) -> Ptr),
        entry!(0x005a_72a0, fn_005a72a0(Ptr, u32) -> Ptr),
        entry!(0x005a_7110, fn_005a7110(Ptr) -> u32),
        entry!(0x005a_7130, fn_005a7130(Ptr) -> u32),
        entry!(0x005a_71d0, fn_005a71d0(Ptr) -> Ptr),
        entry!(0x005a_7220, fn_005a7220(Ptr) -> Ptr),
        entry!(0x005a_72d0, fn_005a72d0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005a7160_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005a_7160,
            size: 0x70,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x005a_71d0, &[A::T(0x18)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6e)]),
            ],
            stores: &[(0x0, 4, 0x0103_6854)],
        });
    }
    #[test]
    fn at_005a72a0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x005a_72d0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x005a_72a0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x005a_72a0)
                .collect();
            let mut expected = vec![(0x005a_72d0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod load_screen_type_tests {
    use super::*;

    #[test]
    fn fn_005a7110_passes_on_the_list_answer() {
        let mut e = Engine::new();
        e.register(0x0082_56d0, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        assert_eq!(e.call(0x005a_7110, &args![0x4000u32]).u32(), 0x412c);
    }

    #[test]
    fn fn_005a7130_reads_through_the_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        assert_eq!(e.call(0x005a_7130, &args![this]).u32(), 0);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x18, 0x1234);
        e.mem.set_u32(this + 0x34, object);
        assert_eq!(e.call(0x005a_7130, &args![this]).u32(), 0x1234);
    }

    #[test]
    fn fn_005a71d0_and_005a7220_set_the_defaults() {
        let mut e = Engine::new();
        e.map(0x0102_3000, 0x1000);
        e.mem.set_u32(0x0102_3cd8, 0x7f80_0001);
        let this = e.mem.alloc(0x58);
        for i in 0..0x16 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x005a_71d0, &args![this]).u32(), this);
        let words: Vec<u32> = (0..0x16).map(|i| e.mem.u32(this + 4 * i)).collect();
        let block = [
            0u32,
            0,
            0,
            0,
            0,
            6,
            0x7fc0_0001,
            0x7fc0_0001,
            0x7fc0_0001,
            1,
        ];
        let mut expected = vec![0u32];
        expected.extend_from_slice(&block);
        // The second block's last word (+0x50) is overwritten with 4.
        expected.extend_from_slice(&block[..9]);
        expected.extend_from_slice(&[4, 3]);
        assert_eq!(words, expected);
        assert_eq!(e.call(0x005a_7220, &args![this + 4]).u32(), this + 4);
    }

    #[test]
    fn fn_005a72d0_puts_back_the_vtable() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.register(0x0048_3630, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005a_72d0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_6854);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0048_3630, vec![this])));
    }
}
