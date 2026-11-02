//! `fallout shared/run_005f4d60` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f4e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f4e00 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x71 (0x7c bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0104_891c`) is `TESChallenge`, so this is the constructor of `TESChallenge`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005f4e00(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x24]);
    e.call(0x0048_cc50, &args![base + 0x2c]);
    e.call(0x0040_2c20, &args![base + 0x38]);
    e.call(0x0047_a130, &args![base + 0x44]);
    e.mem.set_u32(base, 0x0104_891c);
    e.mem.set_u32(base + 0x18, 0x0104_8908);
    e.mem.set_u32(base + 0x24, 0x0104_88f0);
    e.mem.set_u32(base + 0x2c, 0x0104_88dc);
    e.mem.set_u32(base + 0x38, 0x0104_88bc);
    e.mem.set_u32(base + 0x44, 0x0104_88a8);
    e.call(0x005f_4fd0, &args![this]);
    e.call(0x004f_15a0, &args![this, 0x71u32]);
    this
}

// Translated from 005f4d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `005f2440` (cdecl) maps the `u16` at +0x10 to 0xE4 or to 0xEC
/// through 0xEF (signed compares).
pub fn fn_005f4d60(e: &mut Engine, this: Ptr) -> bool {
    let number = u32::from(e.mem.u16(this.addr() + 0x10));
    let value = e.call(0x005f_2440, &args![number]).i32();
    value == 0xe4 || (value > 0xeb && value <= 0xef)
}

// Translated from 005f4db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `005f2440` (cdecl; the engine map names it
/// `TESAnimGroup::GetType`) maps the `u16` at +0x10 to 0xE5 or to 0xF1
/// through 0xF4 (signed compares).
pub fn fn_005f4db0(e: &mut Engine, this: Ptr) -> bool {
    let number = u32::from(e.mem.u16(this.addr() + 0x10));
    let value = e.call(0x005f_2440, &args![number]).i32();
    value == 0xe5 || (value > 0xf0 && value <= 0xf4)
}

// Translated from 005f4ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `TESChallenge`: runs the destructor body
/// `005f4f10` on `this`, then frees `this` with `00401030` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_005f4ee0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005f4f10(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 005f4f10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005f4f10 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Destructor body of `TESChallenge` (the vtables `005f4e00` stores): puts
/// back the six vtables, then `00483710` on `this`, the destructors of
/// the components at +0x44 (`0047a1a0`), +0x38 (`00402c40`) and +0x18
/// (`00591300`) and the `TESForm` base (`00483630`).
pub fn fn_005f4f10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in [
        (0x00u32, 0x0104_891cu32),
        (0x18, 0x0104_8908),
        (0x24, 0x0104_88f0),
        (0x2c, 0x0104_88dc),
        (0x38, 0x0104_88bc),
        (0x44, 0x0104_88a8),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    e.call(0x0048_3710, &args![this]);
    e.call(0x0047_a1a0, &args![base + 0x44]);
    e.call(0x0040_2c40, &args![base + 0x38]);
    e.call(0x0059_1300, &args![base + 0x18]);
    e.call(0x0048_3630, &args![this]);
}

// Translated from 005f4fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESChallenge`'s data defaults: 0x18 bytes at +0x54 and 8 at +0x6C
/// cleared (`memset`), the words at +0x74 and +0x78 cleared, then flag bit
/// 2 set (`005f5020(1)`).
pub fn fn_005f4fd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0040_3d30, &args![base + 0x54, 0u32, 0x18u32]);
    e.call(0x0040_3d30, &args![base + 0x6c, 0u32, 8u32]);
    e.mem.set_u32(base + 0x74, 0);
    e.mem.set_u32(base + 0x78, 0);
    fn_005f5020(e, this, 1);
}

// Translated from 005f5020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` not 0) or clears bit 2 (0x4) of the `u16` flags at +0x5C.
pub fn fn_005f5020(e: &mut Engine, this: Ptr, set: u8) {
    let flags = e.mem.u16(this.addr() + 0x5c);
    let flags = if set == 0 { flags & 0xfffb } else { flags | 4 };
    e.mem.set_u16(this.addr() + 0x5c, flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005f_4e00, fn_005f4e00(Ptr) -> Ptr),
        entry!(0x005f_4d60, fn_005f4d60(Ptr) -> bool),
        entry!(0x005f_4ee0, fn_005f4ee0(Ptr, u32) -> Ptr),
        entry!(0x005f_4f10, fn_005f4f10(Ptr)),
        entry!(0x005f_4fd0, fn_005f4fd0(Ptr)),
        entry!(0x005f_5020, fn_005f5020(Ptr, u8)),
        entry!(0x005f_4db0, fn_005f4db0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005f4e00_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x005f_4e00,
            size: 0x7c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x24)]),
                (0x0048_cc50, &[A::T(0x2c)]),
                (0x0040_2c20, &[A::T(0x38)]),
                (0x0047_a130, &[A::T(0x44)]),
                (0x005f_4fd0, &[A::T(0x0)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x71)]),
            ],
            stores: &[
                (0x0, 4, 0x0104_891c),
                (0x18, 4, 0x0104_8908),
                (0x24, 4, 0x0104_88f0),
                (0x2c, 4, 0x0104_88dc),
                (0x38, 4, 0x0104_88bc),
                (0x44, 4, 0x0104_88a8),
            ],
        });
    }
}

#[cfg(test)]
mod challenge_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_005f4d60_accepts_0xe4_and_0xec_to_0xef() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        // The double answers the number itself.
        e.register(0x005f_2440, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        for (number, accepted) in [
            (0xe4u16, true),
            (0xe5, false),
            (0xeb, false),
            (0xec, true),
            (0xef, true),
            (0xf0, false),
            (0, false),
        ] {
            e.mem.set_u16(this + 0x10, number);
            assert_eq!(
                e.call(0x005f_4d60, &args![this]).bool(),
                accepted,
                "{number:x}"
            );
        }
        // Negative answers are below the range.
        e.register(0x005f_2440, |_, _| Ret {
            eax: 0xffff_ffef,
            ..Ret::default()
        });
        assert!(!e.call(0x005f_4d60, &args![this]).bool());
    }

    #[test]
    fn destructor_and_deleting_destructor() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x7c);
        for addr in [
            0x0048_3710u32,
            0x0047_a1a0,
            0x0040_2c40,
            0x0059_1300,
            0x0048_3630,
            0x0040_1030,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_4ee0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0104_891c);
        assert_eq!(e.mem.u32(this + 0x44), 0x0104_88a8);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x005f_4ee0)
            .collect();
        assert_eq!(
            order,
            vec![
                (0x0048_3710, vec![this]),
                (0x0047_a1a0, vec![this + 0x44]),
                (0x0040_2c40, vec![this + 0x38]),
                (0x0059_1300, vec![this + 0x18]),
                (0x0048_3630, vec![this]),
                (0x0040_1030, vec![this]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x005f_4ee0, &args![this, 2u32]);
        assert!(calls(&e, 0x0040_1030).is_empty());
    }

    #[test]
    fn defaults_and_flag_bit() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x7c);
        e.register(0x0040_3d30, |_, _| Ret::default());
        e.mem.set_u32(this + 0x74, 5);
        e.mem.set_u32(this + 0x78, 6);
        e.mem.set_u16(this + 0x5c, 0x0100);
        e.call_log = Some(vec![]);
        e.call(0x005f_4fd0, &args![this]);
        assert_eq!(
            calls(&e, 0x0040_3d30),
            vec![vec![this + 0x54, 0, 0x18], vec![this + 0x6c, 0, 8]]
        );
        assert_eq!((e.mem.u32(this + 0x74), e.mem.u32(this + 0x78)), (0, 0));
        assert_eq!(e.mem.u16(this + 0x5c), 0x0104);
        e.call(0x005f_5020, &args![this, 0x100u32]);
        assert_eq!(e.mem.u16(this + 0x5c), 0x0100);
        e.mem.set_u16(this + 0x5c, 0xffff);
        e.call(0x005f_5020, &args![this, 0u32]);
        assert_eq!(e.mem.u16(this + 0x5c), 0xfffb);
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    #[test]
    fn fn_005f4db0_accepts_e5_and_f1_to_f4() {
        let mut e = Engine::new();
        e.register(0x005f_2440, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x14);
        for (value, expected) in [
            (0xe5u16, true),
            (0xe4, false),
            (0xf0, false),
            (0xf1, true),
            (0xf4, true),
            (0xf5, false),
        ] {
            e.mem.set_u16(this + 0x10, value);
            assert_eq!(
                e.call(0x005f_4db0, &args![this]).bool(),
                expected,
                "{value:x}"
            );
        }
    }
}
