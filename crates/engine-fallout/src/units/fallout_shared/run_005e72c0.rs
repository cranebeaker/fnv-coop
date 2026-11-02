//! `fallout shared/run_005e72c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e72c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e72c0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 9 (0x50 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005e72c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_a3d0, &args![base + 0x24]);
    e.mem.set_u32(base, 0x0104_64b4);
    e.mem.set_u32(base + 0x18, 0x0104_64a0);
    e.mem.set_u32(base + 0x24, 0x0104_647c);
    e.call(0x0096_a2d0, &args![base + 0x48]);
    e.call(0x004f_15a0, &args![this, 0x9]);
    e.call(0x005e_7440, &args![this]);
    this
}

// Translated from 005e7440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `this + 0x44` (`ECX`, no stack words). The
/// constructor `fn_005e72c0` runs it last. The engine map gives it no name.
pub fn fn_005e7440(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x44, 0);
}

// Translated from 005e7460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list `00624700` gives for `this` (`00470470`). `ECX`, no
/// stack words. The engine map gives it no name.
pub fn fn_005e7460(e: &mut Engine, this: Ptr) {
    let list = e.call(0x0062_4700, &args![this]).u32();
    e.call(0x0047_0470, &args![list]);
}

// Translated from 005e73a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005e73a0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The destructor body of the type-9 form `fn_005e72c0` constructs
/// (`BGSHeadPart`, the RTTI name of the vtable `010464b4`): it
/// stores the vtables back (`010464b4`, `010464a0` at +0x18, `0104647c` at
/// +0x24), empties the list (`fn_005e7460`), then runs the component
/// destructors `0046ffb0` (+0x48), `0048a490` (+0x24), `00591300` (+0x18)
/// and `TESForm::~TESForm` (`00483630`, engine map). `ECX`, no stack words.
/// The exception-unwinding frame is not translated.
pub fn fn_005e73a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_64b4);
    e.mem.set_u32(base + 0x18, 0x0104_64a0);
    e.mem.set_u32(base + 0x24, 0x0104_647c);
    fn_005e7460(e, this);
    e.call(0x0046_ffb0, &args![base + 0x48]);
    e.call(0x0048_a490, &args![base + 0x24]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_72c0, fn_005e72c0(Ptr) -> Ptr),
        entry!(0x005e_7440, fn_005e7440(Ptr)),
        entry!(0x005e_7460, fn_005e7460(Ptr)),
        entry!(0x005e_73a0, fn_005e73a0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005e72c0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005e_72c0,
            size: 0x50,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_a3d0, &[A::T(0x24)]),
                (0x0096_a2d0, &[A::T(0x48)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x9)]),
                (0x005e_7440, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_64b4),
                (0x18, 4, 0x0104_64a0),
                (0x24, 4, 0x0104_647c),
            ],
        });
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;

    #[test]
    fn the_flag_at_0x44_is_cleared() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x50);
        e.mem.set_u8(object + 0x44, 7);
        e.mem.set_u8(object + 0x45, 7);
        e.call(0x005e_7440, &[object]);
        assert_eq!(e.mem.u8(object + 0x44), 0);
        assert_eq!(e.mem.u8(object + 0x45), 7);
    }

    #[test]
    fn the_destructor_empties_the_list_and_runs_the_bases() {
        let mut e = Engine::new();
        e.register(0x0062_4700, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        for callee in [
            0x0047_0470,
            0x0046_ffb0,
            0x0048_a490,
            0x0059_1300,
            0x0048_3630,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        e.call(0x005e_73a0, &[object]);
        assert_eq!(e.mem.u32(object), 0x0104_64b4);
        assert_eq!(e.mem.u32(object + 0x18), 0x0104_64a0);
        assert_eq!(e.mem.u32(object + 0x24), 0x0104_647c);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0062_4700, vec![object]),
                (0x0047_0470, vec![object + 0x100]),
                (0x0046_ffb0, vec![object + 0x48]),
                (0x0048_a490, vec![object + 0x24]),
                (0x0059_1300, vec![object + 0x18]),
                (0x0048_3630, vec![object]),
            ]
        );
    }
}
