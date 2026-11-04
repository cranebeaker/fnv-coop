//! `fallout shared/run_00515220` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00515440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00515440 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x1a (0x154 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00515440(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_cc50, &args![base + 0x3c]);
    e.call(0x0048_30a0, &args![base + 0x48]);
    e.call(0x0048_e7a0, &args![base + 0x58]);
    e.call(0x0048_eac0, &args![base + 0x60]);
    e.call(0x0048_02f0, &args![base + 0x68]);
    e.call(0x0047_5350, &args![base + 0x144]);
    e.call(0x0047_9320, &args![base + 0x14c]);
    e.mem.set_u32(base, 0x0102_ac0c);
    e.mem.set_u32(base + 0x30, 0x0102_abf8);
    e.mem.set_u32(base + 0x3c, 0x0102_abe4);
    e.mem.set_u32(base + 0x48, 0x0102_abcc);
    e.mem.set_u32(base + 0x58, 0x0102_abac);
    e.mem.set_u32(base + 0x60, 0x0102_ab98);
    e.mem.set_u32(base + 0x68, 0x0102_ab84);
    e.mem.set_u32(base + 0x144, 0x0102_ab70);
    e.mem.set_u32(base + 0x14c, 0x0102_ab5c);
    e.call(0x004f_15a0, &args![this, 0x1a]);
    e.call(0x0051_56b0, &args![this]);
    this
}

// Translated from 00515400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3c (`SUB ECX,0x3c`) and jumps to `005146a0`. `005146a0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00515400(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0051_46a0, &args![this.addr().wrapping_sub(0x3c), flags])
        .u32()
}

// Translated from 00515410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x84 (`SUB ECX,0x84`) and jumps to `005152e0`. `005152e0` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_00515410(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0051_52e0,
        &args![this.addr().wrapping_sub(0x84), first, second],
    );
}

// Translated from 00515420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x84 (`SUB ECX,0x84`) and jumps to `00515290`. `00515290` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_00515420(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0051_5290,
        &args![this.addr().wrapping_sub(0x84), argument],
    );
}

// Translated from 00515430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x84 (`SUB ECX,0x84`) and jumps to `00515220`. `00515220` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_00515430(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x0051_5220,
        &args![this.addr().wrapping_sub(0x84), argument],
    )
    .u16()
}

// Translated from 00515580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `005155b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00515580(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_55b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The object pointer at `011de45c` that `008579b0` and `008579e0`
/// (`tessaveloadgame.cpp`, `thiscall`, two words) run on.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `00428110` (`thiscall` on the save or load buffer, one word: an out
/// word), then `004280f0` (`thiscall` on its result, one word).
const BUFFER_00428110: u32 = 0x0042_8110;
const CHECK_004280F0: u32 = 0x0042_80f0;

// Translated from 00515220 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a save-size sum, `RET 4`): the 16-bit sum of `00484bf0` on
/// `this` and `0048e9b0` on `this + 0x84`, each given `flags`, plus one when
/// bit 0x20 of `flags` is set.
pub fn fn_00515220(e: &mut Engine, this: Ptr, flags: u32) -> u16 {
    let mut size: u16 = 0;
    size = size.wrapping_add(e.call(0x0048_4bf0, &args![this, flags]).u16());
    let component = this.addr().wrapping_add(0x84);
    size = size.wrapping_add(e.call(0x0048_e9b0, &args![component, flags]).u16());
    if flags & 0x20 != 0 {
        size = size.wrapping_add(1);
    }
    size
}

// Translated from 00515290 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`): `00484c20` on `this` and `0048e9e0` on
/// `this + 0x84`, each given `flags`; with bit 0x20 of `flags`, `008579b0`
/// on the object at `011de45c` with the byte address `this + 0xC1` and 1.
pub fn fn_00515290(e: &mut Engine, this: Ptr, flags: u32) {
    e.call(0x0048_4c20, &args![this, flags]);
    e.call(0x0048_e9e0, &args![this.addr() + 0x84, flags]);
    if flags & 0x20 != 0 {
        let target = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(0x0085_79b0, &args![target, this.addr() + 0xc1, 1u32]);
    }
}

// Translated from 005152e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 8`): `00484c50` on `this` and `0048ea10` on
/// `this + 0x84`, each given both words; with bit 0x20 of `flags`,
/// `008579e0` on the object at `011de45c` with `this + 0xC1` and 1.
pub fn fn_005152e0(e: &mut Engine, this: Ptr, flags: u32, second: u32) {
    e.call(0x0048_4c50, &args![this, flags, second]);
    e.call(0x0048_ea10, &args![this.addr() + 0x84, flags, second]);
    if flags & 0x20 != 0 {
        let target = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(0x0085_79e0, &args![target, this.addr() + 0xc1, 1u32]);
    }
}

// Translated from 00515340 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`; a save): `00484d60` on `this` and `0048ea40` on
/// `this + 0x84`, each given `buffer`; then when `004280f0(0x20)` on what
/// `00428110` (given an out word on the stack) answers for the buffer is
/// true, `00865e50` on the buffer with `this + 0xC1`, 1 and 0.
pub fn fn_00515340(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4d60, &args![this, buffer]);
    e.call(0x0048_ea40, &args![this.addr() + 0x84, buffer]);
    e.with_stack(4, |e, word| {
        let checker = e.call(BUFFER_00428110, &args![buffer, word]).u32();
        if e.call(CHECK_004280F0, &args![checker, 0x20u32]).bool() {
            e.call(0x0086_5e50, &args![buffer, this.addr() + 0xc1, 1u32, 0u32]);
        }
    });
}

// Translated from 005153a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`RET 4`; a load): `00484da0` on `this` and `0048ea80` on
/// `this + 0x84`, each given `buffer`; then when `004280f0(0x20)` on what
/// `00428110` (given an out word on the stack) answers for the buffer is
/// true, `00864980` on the buffer with `this + 0xC1` and 1.
pub fn fn_005153a0(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4da0, &args![this, buffer]);
    e.call(0x0048_ea80, &args![this.addr() + 0x84, buffer]);
    e.with_stack(4, |e, word| {
        let checker = e.call(BUFFER_00428110, &args![buffer, word]).u32();
        if e.call(CHECK_004280F0, &args![checker, 0x20u32]).bool() {
            e.call(0x0086_4980, &args![buffer, this.addr() + 0xc1, 1u32]);
        }
    });
}

/// The vtables `005155b0` stores, by offset.
const DESTRUCTOR_VTABLES: [(u32, u32); 9] = [
    (0x00, 0x0102_ac0c),
    (0x30, 0x0102_abf8),
    (0x3c, 0x0102_abe4),
    (0x48, 0x0102_abcc),
    (0x58, 0x0102_abac),
    (0x60, 0x0102_ab98),
    (0x68, 0x0102_ab84),
    (0x144, 0x0102_ab70),
    (0x14c, 0x0102_ab5c),
];
/// The component destructors `005155b0` runs, in its order.
const DESTRUCTOR_COMPONENTS: [(u32, u32); 5] = [
    (0x144, 0x0047_5380),
    (0x68, 0x0048_0400),
    (0x60, 0x0048_eaf0),
    (0x58, 0x0048_e7d0),
    (0x30, 0x0059_1300),
];

// Translated from 005155b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005155b0 - the FS:[0] exception frame (running the remaining destructors when one throws) is not translated
/// No map name: a destructor body (`thiscall`): stores the nine vtables
/// (`DESTRUCTOR_VTABLES`), runs the component destructors
/// (`DESTRUCTOR_COMPONENTS`, in that order), then `0050e9f0` on `this`.
pub fn fn_005155b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    for (offset, vtable) in DESTRUCTOR_VTABLES {
        e.mem.set_u32(at + offset, vtable);
    }
    for (offset, destructor) in DESTRUCTOR_COMPONENTS {
        e.call(destructor, &args![at + offset]);
    }
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 005156b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `NonActorMagicCaster::SetCurrentSpell` (`0041fd00`, engine
/// map) on the member at +0x48 with 3.
pub fn fn_005156b0(e: &mut Engine, this: Ptr) {
    e.call(0x0041_fd00, &args![this.addr() + 0x48, 3u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_5440, fn_00515440(Ptr) -> Ptr),
        entry!(0x0051_5400, fn_00515400(Ptr, u32) -> u32),
        entry!(0x0051_5410, fn_00515410(Ptr, u32, u32)),
        entry!(0x0051_5420, fn_00515420(Ptr, u32)),
        entry!(0x0051_5430, fn_00515430(Ptr, u32) -> u16),
        entry!(0x0051_5580, fn_00515580(Ptr, u32) -> Ptr),
        entry!(0x0051_5220, fn_00515220(Ptr, u32) -> u16),
        entry!(0x0051_5290, fn_00515290(Ptr, u32)),
        entry!(0x0051_52e0, fn_005152e0(Ptr, u32, u32)),
        entry!(0x0051_5340, fn_00515340(Ptr, u32)),
        entry!(0x0051_53a0, fn_005153a0(Ptr, u32)),
        entry!(0x0051_55b0, fn_005155b0(Ptr)),
        entry!(0x0051_56b0, fn_005156b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00515440_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_5440,
            size: 0x154,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_cc50, &[A::T(0x3c)]),
                (0x0048_30a0, &[A::T(0x48)]),
                (0x0048_e7a0, &[A::T(0x58)]),
                (0x0048_eac0, &[A::T(0x60)]),
                (0x0048_02f0, &[A::T(0x68)]),
                (0x0047_5350, &[A::T(0x144)]),
                (0x0047_9320, &[A::T(0x14c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1a)]),
                (0x0051_56b0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_ac0c),
                (0x30, 4, 0x0102_abf8),
                (0x3c, 4, 0x0102_abe4),
                (0x48, 4, 0x0102_abcc),
                (0x58, 4, 0x0102_abac),
                (0x60, 4, 0x0102_ab98),
                (0x68, 4, 0x0102_ab84),
                (0x144, 4, 0x0102_ab70),
                (0x14c, 4, 0x0102_ab5c),
            ],
        });
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00515400() {
        assert_eq!(
            check(0x0051_5400, 0x0051_46a0, 0x3c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00515410() {
        check(0x0051_5410, 0x0051_52e0, 0x84, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_00515420() {
        check(0x0051_5420, 0x0051_5290, 0x84, &[0x77]);
    }

    #[test]
    fn le81_thunk_00515430() {
        assert_eq!(check(0x0051_5430, 0x0051_5220, 0x84, &[0x77]).u16(), 0xbeef);
    }
    #[test]
    fn at_00515580_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_55b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_5580, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_5580)
                .collect();
            let mut expected = vec![(0x0051_55b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    fn calls_to(e: &mut Engine, targets: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| targets.contains(a))
            .collect()
    }

    #[test]
    fn fn_00515220_counts_the_extra_byte() {
        let mut e = Engine::new();
        e.register(0x0048_4bf0, |_, _| Ret {
            eax: 0x10,
            ..Ret::default()
        });
        e.register(0x0048_e9b0, |_, _| Ret {
            eax: 0xffef,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0051_5220, &[0x1000, 0]).u16(), 0xffff);
        assert_eq!(e.call(0x0051_5220, &[0x1000, 0x20]).u16(), 0);
    }

    #[test]
    fn fn_00515290_and_005152e0_save_and_load_the_byte() {
        let mut e = Engine::new();
        let targets = [
            0x0048_4c20,
            0x0048_e9e0,
            0x0048_4c50,
            0x0048_ea10,
            0x0085_79b0,
            0x0085_79e0,
        ];
        for t in targets {
            e.register(t, |_, _| Ret::default());
        }
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x7000u32);
        e.call_log = Some(vec![]);
        e.call(0x0051_5290, &[0x1000, 0x1f]);
        e.call(0x0051_52e0, &[0x1000, 0xdf, 5]);
        assert_eq!(
            calls_to(&mut e, &targets),
            vec![
                (0x0048_4c20, vec![0x1000, 0x1f]),
                (0x0048_e9e0, vec![0x1084, 0x1f]),
                (0x0048_4c50, vec![0x1000, 0xdf, 5]),
                (0x0048_ea10, vec![0x1084, 0xdf, 5]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x0051_5290, &[0x1000, 0x20]);
        e.call(0x0051_52e0, &[0x1000, 0x20, 5]);
        let log = calls_to(&mut e, &[0x0085_79b0, 0x0085_79e0]);
        assert_eq!(
            log,
            vec![
                (0x0085_79b0, vec![0x7000, 0x10c1, 1]),
                (0x0085_79e0, vec![0x7000, 0x10c1, 1]),
            ]
        );
    }

    #[test]
    fn fn_00515340_and_005153a0_check_the_buffer() {
        let mut e = Engine::new();
        let targets = [
            0x0048_4d60,
            0x0048_ea40,
            0x0048_4da0,
            0x0048_ea80,
            0x0086_5e50,
            0x0086_4980,
        ];
        for t in targets {
            e.register(t, |_, _| Ret::default());
        }
        e.register(BUFFER_00428110, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(CHECK_004280F0, |_, a| {
            assert_eq!(a[1], 0x20);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let buffer = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x0051_5340, &[0x1000, buffer]);
        e.call(0x0051_53a0, &[0x1000, buffer]);
        assert_eq!(
            calls_to(&mut e, &targets),
            vec![
                (0x0048_4d60, vec![0x1000, buffer]),
                (0x0048_ea40, vec![0x1084, buffer]),
                (0x0048_4da0, vec![0x1000, buffer]),
                (0x0048_ea80, vec![0x1084, buffer]),
            ]
        );
        e.mem.set_u32(buffer, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_5340, &[0x1000, buffer]);
        e.call(0x0051_53a0, &[0x1000, buffer]);
        assert_eq!(
            calls_to(&mut e, &[0x0086_5e50, 0x0086_4980]),
            vec![
                (0x0086_5e50, vec![buffer, 0x10c1, 1, 0]),
                (0x0086_4980, vec![buffer, 0x10c1, 1]),
            ]
        );
    }

    #[test]
    fn fn_005155b0_and_005156b0() {
        let mut e = Engine::new();
        for (_, d) in DESTRUCTOR_COMPONENTS {
            e.register(d, |_, _| Ret::default());
        }
        e.register(0x0050_e9f0, |_, _| Ret::default());
        e.register(0x0041_fd00, |_, _| Ret::default());
        let this = e.mem.alloc(0x150);
        e.call_log = Some(vec![]);
        e.call(0x0051_55b0, &[this]);
        let mut expected: Vec<(u32, Vec<u32>)> = DESTRUCTOR_COMPONENTS
            .iter()
            .map(|(o, d)| (*d, vec![this + o]))
            .collect();
        expected.push((0x0050_e9f0, vec![this]));
        let mut targets: Vec<u32> = DESTRUCTOR_COMPONENTS.iter().map(|(_, d)| *d).collect();
        targets.push(0x0050_e9f0);
        assert_eq!(calls_to(&mut e, &targets), expected);
        for (offset, vtable) in DESTRUCTOR_VTABLES {
            assert_eq!(e.mem.u32(this + offset), vtable);
        }
        e.call_log = Some(vec![]);
        e.call(0x0051_56b0, &[this]);
        assert_eq!(
            calls_to(&mut e, &[0x0041_fd00]),
            vec![(0x0041_fd00, vec![this + 0x48, 3])]
        );
    }
}
