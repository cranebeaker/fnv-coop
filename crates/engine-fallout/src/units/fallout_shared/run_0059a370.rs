//! `fallout shared/run_0059a370` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0059a370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059a370 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x73 (0xbc bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0103_478c`) is `TESCaravanCard`, so this is the constructor of `TESCaravanCard`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_0059a370(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0047_a130, &args![base + 0x68]);
    e.call(0x0048_e7a0, &args![base + 0x78]);
    e.call(0x0048_cc50, &args![base + 0x80]);
    e.call(0x0047_b4f0, &args![base + 0x8c]);
    e.mem.set_u32(base, 0x0103_478c);
    e.mem.set_u32(base + 0x30, 0x0103_4774);
    e.mem.set_u32(base + 0x3c, 0x0103_4750);
    e.mem.set_u32(base + 0x5c, 0x0103_4730);
    e.mem.set_u32(base + 0x68, 0x0103_471c);
    e.mem.set_u32(base + 0x78, 0x0103_46fc);
    e.mem.set_u32(base + 0x80, 0x0103_46e8);
    e.mem.set_u32(base + 0x8c, 0x0103_46d4);
    e.call(0x0048_e270, &args![base + 0x9c]);
    e.call(0x0048_e270, &args![base + 0xa8]);
    e.call(0x004f_15a0, &args![this, 0x73u32]);
    e.call(0x0059_a670, &args![this]);
    this
}

/// `TESCaravanCard`'s vtables, stored at the start of its destructor (the same words the
/// constructor `0059a370` stores), by field offset.
const CARD_VTABLES: [(u32, u32); 8] = [
    (0x00, 0x0103_478c),
    (0x30, 0x0103_4774),
    (0x3c, 0x0103_4750),
    (0x5c, 0x0103_4730),
    (0x68, 0x0103_471c),
    (0x78, 0x0103_46fc),
    (0x80, 0x0103_46e8),
    (0x8c, 0x0103_46d4),
];

// Translated from 0059a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): calls vtable slot +4 of the two
/// sub-objects at +0x9c and +0xa8 (no arguments), then `00403d30` (cdecl) with
/// `this+0xb4`, 0 and 8. The destructor `0059a4f0` runs it first.
pub fn fn_0059a620(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.vcall(base + 0x9c, 4, &args![]);
    e.vcall(base + 0xa8, 4, &args![]);
    e.call(0x0040_3d30, &args![base + 0xb4, 0u32, 8u32]);
}

// Translated from 0059a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only, plain `RET`): calls vtable slot +0 of the two
/// sub-objects at +0x9c and +0xa8 (no arguments), then `00403d30` (cdecl) with
/// `this+0xb4`, 0 and 8. The constructor `0059a370` runs it last.
pub fn fn_0059a670(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.vcall(base + 0x9c, 0, &args![]);
    e.vcall(base + 0xa8, 0, &args![]);
    e.call(0x0040_3d30, &args![base + 0xb4, 0u32, 8u32]);
}

// Translated from 0059a4f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0059a4f0 - the exception-unwinding frame (FS:[0] chain, handler 00ef8949, state variable) is not translated.
/// Destructor of `TESCaravanCard` (the class whose vtable `0x0103_478c` the constructor
/// `0059a370` stores; unnamed in the engine map; `ECX` only, plain `RET`): stores the
/// vtables again, runs `0059a620`, then the member and base destructors in reverse order of
/// construction: `0048e2e0` on +0xa8 and +0x9c, `0047b520` on +0x8c, `0048e7d0` on +0x78,
/// `0047a1a0` on +0x68, `00402c40` on +0x5c, `0048a490` on +0x3c, `00591300` on +0x30 and
/// `0050e9f0` on the object.
pub fn fn_0059a4f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in CARD_VTABLES {
        e.mem.set_u32(base + offset, vtable);
    }
    fn_0059a620(e, this);
    e.call(0x0048_e2e0, &args![base + 0xa8]);
    e.call(0x0048_e2e0, &args![base + 0x9c]);
    e.call(0x0047_b520, &args![base + 0x8c]);
    e.call(0x0048_e7d0, &args![base + 0x78]);
    e.call(0x0047_a1a0, &args![base + 0x68]);
    e.call(0x0040_2c40, &args![base + 0x5c]);
    e.call(0x0048_a490, &args![base + 0x3c]);
    e.call(0x0059_1300, &args![base + 0x30]);
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 0059a4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `TESCaravanCard` (unnamed in the engine map; `ECX` and
/// one stack word, `RET 4`): runs the destructor `0059a4f0`, frees the object through
/// `00401030` (cdecl) when bit 0 of `flags` is set, and returns the object.
pub fn fn_0059a4c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0059a4f0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0059_a370, fn_0059a370(Ptr) -> Ptr),
        entry!(0x0059_a620, fn_0059a620(Ptr)),
        entry!(0x0059_a670, fn_0059a670(Ptr)),
        entry!(0x0059_a4f0, fn_0059a4f0(Ptr)),
        entry!(0x0059_a4c0, fn_0059a4c0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_0059a370_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0059_a370,
            size: 0xbc,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0047_a130, &[A::T(0x68)]),
                (0x0048_e7a0, &[A::T(0x78)]),
                (0x0048_cc50, &[A::T(0x80)]),
                (0x0047_b4f0, &[A::T(0x8c)]),
                (0x0048_e270, &[A::T(0x9c)]),
                (0x0048_e270, &[A::T(0xa8)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x73)]),
                (0x0059_a670, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0103_478c),
                (0x30, 4, 0x0103_4774),
                (0x3c, 4, 0x0103_4750),
                (0x5c, 4, 0x0103_4730),
                (0x68, 4, 0x0103_471c),
                (0x78, 4, 0x0103_46fc),
                (0x80, 4, 0x0103_46e8),
                (0x8c, 4, 0x0103_46d4),
            ],
        });
    }

    /// The calls a test made, without the top-level call.
    fn calls_after(e: &mut crate::prelude::Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(address, _)| *address != top)
            .collect()
    }

    /// An object of 0xbc bytes whose sub-objects at +0x9c and +0xa8 have one vtable each
    /// (slots 0 and 1 at the given addresses), with doubles for every callee.
    fn card_with_members(e: &mut crate::prelude::Engine) -> u32 {
        use crate::prelude::*;
        let object = e.mem.alloc(0xbc);
        e.put_vtable(0x0300_0000, &[0x0300_1000, 0x0300_1004]);
        e.put_vtable(0x0300_0100, &[0x0300_1100, 0x0300_1104]);
        e.mem.set_u32(object + 0x9c, 0x0300_0000);
        e.mem.set_u32(object + 0xa8, 0x0300_0100);
        for address in [
            0x0300_1000u32,
            0x0300_1004,
            0x0300_1100,
            0x0300_1104,
            0x0040_3d30,
            0x0048_e2e0,
            0x0047_b520,
            0x0048_e7d0,
            0x0047_a1a0,
            0x0040_2c40,
            0x0048_a490,
            0x0059_1300,
            0x0050_e9f0,
            0x0040_1030,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        object
    }

    #[test]
    fn fn_0059a620_calls_slot_four_of_both_members_then_clears() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let object = card_with_members(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0059_a620, &args![object]);
        let calls = calls_after(&mut e, 0x0059_a620);
        assert_eq!(
            calls,
            vec![
                (0x0300_1004, vec![object + 0x9c]),
                (0x0300_1104, vec![object + 0xa8]),
                (0x0040_3d30, vec![object + 0xb4, 0, 8]),
            ]
        );
    }

    #[test]
    fn fn_0059a670_calls_slot_zero_of_both_members_then_clears() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let object = card_with_members(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0059_a670, &args![object]);
        let calls = calls_after(&mut e, 0x0059_a670);
        assert_eq!(
            calls,
            vec![
                (0x0300_1000, vec![object + 0x9c]),
                (0x0300_1100, vec![object + 0xa8]),
                (0x0040_3d30, vec![object + 0xb4, 0, 8]),
            ]
        );
    }

    #[test]
    fn fn_0059a4f0_stores_the_vtables_and_runs_the_member_destructors() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let object = card_with_members(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0059_a4f0, &args![object]);
        let calls = calls_after(&mut e, 0x0059_a4f0);
        assert_eq!(
            calls,
            vec![
                (0x0300_1004, vec![object + 0x9c]),
                (0x0300_1104, vec![object + 0xa8]),
                (0x0040_3d30, vec![object + 0xb4, 0, 8]),
                (0x0048_e2e0, vec![object + 0xa8]),
                (0x0048_e2e0, vec![object + 0x9c]),
                (0x0047_b520, vec![object + 0x8c]),
                (0x0048_e7d0, vec![object + 0x78]),
                (0x0047_a1a0, vec![object + 0x68]),
                (0x0040_2c40, vec![object + 0x5c]),
                (0x0048_a490, vec![object + 0x3c]),
                (0x0059_1300, vec![object + 0x30]),
                (0x0050_e9f0, vec![object]),
            ]
        );
        for (offset, vtable) in super::CARD_VTABLES {
            assert_eq!(e.mem.u32(object + offset), vtable);
        }
    }

    #[test]
    fn fn_0059a4c0_destroys_and_frees_on_bit_zero() {
        use crate::prelude::*;
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (3, true)] {
            let mut e = Engine::new();
            let object = card_with_members(&mut e);
            e.call_log = Some(vec![]);
            let result = e.call(0x0059_a4c0, &args![object, flags]);
            assert_eq!(result.u32(), object);
            let calls = calls_after(&mut e, 0x0059_a4c0);
            assert_eq!(calls.len(), if frees { 13 } else { 12 });
            assert_eq!(calls[11], (0x0050_e9f0, vec![object]));
            assert_eq!(
                calls.iter().filter(|call| call.0 == 0x0040_1030).count(),
                usize::from(frees)
            );
            if frees {
                assert_eq!(calls[12], (0x0040_1030, vec![object]));
            }
        }
    }
}
