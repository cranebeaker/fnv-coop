//! `fallout shared/run_005fda00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005fdaa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fdaa0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 10 (0x4c bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005fdaa0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_8f50, &args![base + 0x24]);
    e.call(0x0048_e270, &args![base + 0x3c]);
    e.mem.set_u32(base, 0x0104_9b9c);
    e.mem.set_u32(base + 0x18, 0x0104_9b88);
    e.mem.set_u32(base + 0x24, 0x0104_9b64);
    e.mem.set_u32(base + 0x3c, 0x0104_9b44);
    e.call(0x004f_15a0, &args![this, 0xa]);
    e.call(0x005f_dc30, &args![this]);
    this
}

// Translated from 005fdc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `this + 0x48` (`ECX`, no stack words). The
/// constructor `fn_005fdaa0` runs it last. The engine map gives it no name.
pub fn fn_005fdc30(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x48, 0);
}

// Translated from 005fdb90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005fdb90 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// The destructor body of the type-10 form `fn_005fdaa0` constructs
/// (`TESHair`, the RTTI name of the vtable `01049b9c`): it
/// stores the vtables back (`01049b9c`, and the components' at +0x18,
/// +0x24, +0x3c), then runs the component destructors `0048e2e0` (+0x3c),
/// `00489070` (+0x24), `00591300` (+0x18) and `TESForm::~TESForm`
/// (`00483630`, engine map). `ECX`, no stack words. The
/// exception-unwinding frame is not translated.
pub fn fn_005fdb90(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0104_9b9c);
    e.mem.set_u32(base + 0x18, 0x0104_9b88);
    e.mem.set_u32(base + 0x24, 0x0104_9b64);
    e.mem.set_u32(base + 0x3c, 0x0104_9b44);
    e.call(0x0048_e2e0, &args![base + 0x3c]);
    e.call(0x0048_9070, &args![base + 0x24]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005fdb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the type-10 form: runs `fn_005fdb90` on
/// `this`, then frees `this` with `00401030` when bit 0 of `flags` is set.
/// Returns `this`. `__thiscall`, `RET 4`. The engine map gives it no name.
pub fn fn_005fdb60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005fdb90(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// Adds `amount` to the word at `offset`; when the object at +0x38 is set
/// and the byte `notify` is non-zero, runs `00616c20(which)` on it; then
/// vtable slot 0x48 with 0x80000000.
fn add_and_notify(e: &mut Engine, this: Ptr, offset: u32, amount: u32, notify: u8, which: u32) {
    let at = this.addr() + offset;
    let value = e.mem.u32(at).wrapping_add(amount);
    e.mem.set_u32(at, value);
    let target = e.mem.u32(this.addr() + 0x38);
    if target != 0 && notify != 0 {
        e.call(0x0061_6c20, &args![target, which]);
    }
    e.vcall(this.addr(), 0x48, &args![0x8000_0000u32]);
}

// Translated from 005fda00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `amount` to the word at +0x48 (`RET 8`: the amount, a byte; the
/// engine map has no name); when the object at +0x38 is set and the byte
/// is non-zero, `00616c20(0)` runs on it; then vtable slot 0x48 with
/// 0x80000000.
pub fn fn_005fda00(e: &mut Engine, this: Ptr, amount: u32, notify: u8) {
    add_and_notify(e, this, 0x48, amount, notify, 0);
}

// Translated from 005fda50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `amount` to the word at +0x44 (`RET 8`: the amount, a byte; the
/// engine map has no name); when the object at +0x38 is set and the byte
/// is non-zero, `00616c20(1)` runs on it; then vtable slot 0x48 with
/// 0x80000000.
pub fn fn_005fda50(e: &mut Engine, this: Ptr, amount: u32, notify: u8) {
    add_and_notify(e, this, 0x44, amount, notify, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_da00, fn_005fda00(Ptr, u32, u8)),
        entry!(0x005f_da50, fn_005fda50(Ptr, u32, u8)),
        entry!(0x005f_daa0, fn_005fdaa0(Ptr) -> Ptr),
        entry!(0x005f_dc30, fn_005fdc30(Ptr)),
        entry!(0x005f_db90, fn_005fdb90(Ptr)),
        entry!(0x005f_db60, fn_005fdb60(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005fdaa0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_daa0,
            size: 0x4c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_8f50, &[A::T(0x24)]),
                (0x0048_e270, &[A::T(0x3c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0xa)]),
                (0x005f_dc30, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_9b9c),
                (0x18, 4, 0x0104_9b88),
                (0x24, 4, 0x0104_9b64),
                (0x3c, 4, 0x0104_9b44),
            ],
        });
    }
}

#[cfg(test)]
mod b022_tests {
    use super::*;

    #[test]
    fn the_flag_at_0x48_is_cleared() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x4c);
        e.mem.set_u8(object + 0x48, 3);
        e.mem.set_u8(object + 0x49, 3);
        e.call(0x005f_dc30, &[object]);
        assert_eq!(e.mem.u8(object + 0x48), 0);
        assert_eq!(e.mem.u8(object + 0x49), 3);
    }

    #[test]
    fn the_destructor_runs_the_bases_and_deletes_on_flag_one() {
        let mut e = Engine::new();
        for callee in [
            0x0048_e2e0,
            0x0048_9070,
            0x0059_1300,
            0x0048_3630,
            0x0040_1030,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let object = e.mem.alloc(0x4c);
        for flags in [0u32, 1, 2] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x005f_db60, &[object, flags]).u32(), object);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![
                (0x0048_e2e0, vec![object + 0x3c]),
                (0x0048_9070, vec![object + 0x24]),
                (0x0059_1300, vec![object + 0x18]),
                (0x0048_3630, vec![object]),
            ];
            if flags & 1 != 0 {
                expected.push((0x0040_1030, vec![object]));
            }
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(object), 0x0104_9b9c);
            assert_eq!(e.mem.u32(object + 0x18), 0x0104_9b88);
            assert_eq!(e.mem.u32(object + 0x24), 0x0104_9b64);
            assert_eq!(e.mem.u32(object + 0x3c), 0x0104_9b44);
        }
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    #[test]
    fn counters_add_and_notify() {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x50);
        e.mem.set_u32(vtable + 0x48, 0x7000_0048);
        e.register(0x7000_0048, |_, _| Ret::default());
        e.register(0x0061_6c20, |_, _| Ret::default());
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x48, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x005f_da00, &args![this, 2u32, 1u32]);
        assert_eq!(e.mem.u32(this + 0x48), 1);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == 0x0061_6c20));
        assert!(log.contains(&(0x7000_0048, vec![this, 0x8000_0000])));
        e.mem.set_u32(this + 0x38, 0x4000);
        for (addr, offset, which) in [(0x005f_da00u32, 0x48u32, 0u32), (0x005f_da50, 0x44, 1)] {
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, 3u32, 0x100u32]);
            e.call(addr, &args![this, 3u32, 1u32]);
            let log = e.call_log.take().unwrap();
            let notified: Vec<_> = log.iter().filter(|(a, _)| *a == 0x0061_6c20).collect();
            assert_eq!(notified, vec![&(0x0061_6c20, vec![0x4000, which])]);
            assert_eq!(e.mem.u32(this + offset), if which == 0 { 7 } else { 6 });
        }
    }
}
