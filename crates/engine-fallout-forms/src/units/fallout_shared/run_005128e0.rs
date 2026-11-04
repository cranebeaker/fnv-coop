//! `fallout shared/run_005128e0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005129c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `00616640`. `00616640` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_005129c0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0061_6640,
        &args![this.addr().wrapping_sub(0x58), first, second],
    );
}

// Translated from 005129d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `0059adb0`. `0059adb0` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005129d0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0059_adb0,
        &args![this.addr().wrapping_sub(0x58), argument],
    );
}

// Translated from 005129e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `005a7770`. `005a7770` (`RET 0x4`) returns a 16-bit value in `AX` (it ends `MOV AX,word ptr [...]`), so the thunk returns `u16`; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005129e0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    e.call(
        0x005a_7770,
        &args![this.addr().wrapping_sub(0x58), argument],
    )
    .u16()
}

// Translated from 005129f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005129f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x18; `ECX` is the object.
/// The engine map gives it no name, so the class is unconfirmed.
/// It runs the base (`0050e820`) and component constructors by address, stores the vtable pointers and
/// finishes with the form-type registration (`004f15a0`) and `00512e10`.
/// The exception-unwinding frame is not translated.
pub fn fn_005129f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_cc50, &args![base + 0x3c]);
    e.call(0x0048_30a0, &args![base + 0x48]);
    e.call(0x0048_e7a0, &args![base + 0x58]);
    e.call(0x0048_eac0, &args![base + 0x60]);
    e.call(0x0048_71d0, &args![base + 0x68]);
    e.call(0x0048_02f0, &args![base + 0x70]);
    e.call(0x0047_5350, &args![base + 0x14c]);
    e.call(0x0047_9320, &args![base + 0x154]);
    e.call(0x0047_b850, &args![base + 0x15c]);
    e.call(0x0047_4d10, &args![base + 0x164]);
    e.call(0x0047_b4f0, &args![base + 0x16c]);
    for (offset, vtable) in [
        (0x00, 0x0102_a62c),
        (0x30, 0x0102_a614),
        (0x3c, 0x0102_a600),
        (0x48, 0x0102_a5e8),
        (0x58, 0x0102_a5c8),
        (0x60, 0x0102_a5b4),
        (0x68, 0x0102_a59c),
        (0x70, 0x0102_a588),
        (0x14c, 0x0102_a574),
        (0x154, 0x0102_a560),
        (0x15c, 0x0102_a54c),
        (0x164, 0x0102_a538),
        (0x16c, 0x0102_a524),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    e.call(0x004f_15a0, &args![this, 0x18u32]);
    e.call(0x0051_2e10, &args![this]);
    this
}

// Translated from 00512ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00512bd0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00512ba0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_2bd0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The component destructors `00512bd0` runs, in its order: the offset in
/// the object and the destructor.
const COMPONENT_DESTRUCTORS: [(u32, u32); 9] = [
    (0x16c, 0x0047_b520),
    (0x164, 0x0047_4d40),
    (0x15c, 0x0047_b880),
    (0x14c, 0x0047_5380),
    (0x70, 0x0048_0400),
    (0x68, 0x0048_7200),
    (0x60, 0x0048_eaf0),
    (0x58, 0x0048_e7d0),
    (0x30, 0x0059_1300),
];

// Translated from 00512bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00512bd0 - the exception-unwinding frame (FS:[0]) is not translated.
/// The destructor body `00512ba0` runs (vtable `0102a62c`, RTTI
/// `.?AVTESObjectARMO@@`): stores the object's and its components'
/// vtables as the constructor `005129f0` does, runs the components'
/// destructors (`COMPONENT_DESTRUCTORS`, last component first) and then
/// `0050e9f0` on the object.
pub fn fn_00512bd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in [
        (0x00, 0x0102_a62c),
        (0x30, 0x0102_a614),
        (0x3c, 0x0102_a600),
        (0x48, 0x0102_a5e8),
        (0x58, 0x0102_a5c8),
        (0x60, 0x0102_a5b4),
        (0x68, 0x0102_a59c),
        (0x70, 0x0102_a588),
        (0x14c, 0x0102_a574),
        (0x154, 0x0102_a560),
        (0x15c, 0x0102_a54c),
        (0x164, 0x0102_a538),
        (0x16c, 0x0102_a524),
    ] {
        e.mem.set_u32(base + offset, vtable);
    }
    for (offset, destructor) in COMPONENT_DESTRUCTORS {
        e.call(destructor, &args![base + offset]);
    }
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 00512d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (called by `005f2b60`): walks a list of 0xC-byte entries
/// (a form at `+0`, a chance byte at `+4`, a key word at `+8`). The list
/// is the word at `+0x188` when the byte at `+0x18C` is set, otherwise the
/// word at `+0x188` of the object `+0x188` points at (none when that is
/// null). For each entry (the walk going on while the node is not empty,
/// `008256d0`, and nothing is found yet) with a form and the key `key`,
/// a number from `00487f50`, masked with 0x63, below the chance byte
/// (signed compare) picks the entry's form; `RET 4`.
pub fn fn_00512d40(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let base = this.addr();
    let mut found = 0u32;
    let mut node = if e.mem.u8(base + 0x18c) != 0 {
        e.mem.u32(base + 0x188)
    } else {
        let template = e.mem.u32(base + 0x188);
        if template != 0 {
            e.mem.u32(template + 0x188)
        } else {
            0
        }
    };
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() && found == 0 {
        let entry_at = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(entry_at);
        node = e.call(0x0072_6070, &args![node]).u32();
        if e.mem.u32(entry) != 0 && e.mem.u32(entry + 8) == key {
            let roll = (e.call(0x0048_7f50, &args![]).u32() & 0x63) as i32;
            if roll < i32::from(e.mem.u8(entry + 4)) {
                found = e.mem.u32(entry);
            }
        }
    }
    found
}

// Translated from 00512e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start values of the constructor `005129f0`: 0xC zero bytes at
/// `+0x178` (`00403d30`, a `memset` wrapper), the byte at `+0x18C` and the
/// word at `+0x188` cleared, and `0041fd00` on `+0x48` with 3 (the engine
/// map's `NonActorMagicCaster::SetCurrentSpell` name is a folded body's).
pub fn fn_00512e10(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(0x0040_3d30, &args![base + 0x178, 0u32, 0xcu32]);
    e.mem.set_u8(base + 0x18c, 0);
    e.mem.set_u32(base + 0x188, 0);
    e.call(0x0041_fd00, &args![base + 0x48, 3u32]);
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `.?AVTESForm@@`
/// (`01183028`) to `.?AVTESObjectARMO@@` (`01183a34`, the copy) or
/// `.?AVTESObjectARMA@@` (`0118a620`, the comparison).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_ARMO: u32 = 0x0118_3a34;
const TYPE_TES_OBJECT_ARMA: u32 = 0x0118_a620;
/// `TESForm::CopyAllComponents` and `CompareAllComponents` (`004851b0`,
/// `00485270`, engine map; `thiscall`, one word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `00401460` (`cdecl`: destination, source, size; a byte copy) and
/// `memcmp` (`00ec4835`, `cdecl`).
const COPY_BYTES: u32 = 0x0040_1460;
const MEMCMP: u32 = 0x00ec_4835;

// Translated from 005128e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; a copy): when
/// `source` casts to `TESObjectARMO`, copies the 0xC bytes at
/// `source + 0x178` to `this + 0x178` (`00401460`), then
/// `TESForm::CopyAllComponents(source)`.
pub fn fn_005128e0(e: &mut Engine, this: Ptr, source: u32) {
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![source, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_ARMO, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    e.call(
        COPY_BYTES,
        &args![this.addr() + 0x178, source.wrapping_add(0x178), 0xcu32],
    );
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
}

// Translated from 00512940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`): whether the
/// form differs from `form`: true when it does not cast to
/// `TESObjectARMA`, when `TESForm::CompareAllComponents` answers true, or
/// when `memcmp` finds the 0xC bytes at +0x178 of the two different.
pub fn fn_00512940(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_ARMA, 0u32],
        )
        .u32();
    if other == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool() {
        return true;
    }
    let order = e
        .call(
            MEMCMP,
            &args![this.addr() + 0x178, other.wrapping_add(0x178), 0xcu32],
        )
        .u32();
    order != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_29c0, fn_005129c0(Ptr, u32, u32)),
        entry!(0x0051_29d0, fn_005129d0(Ptr, u32)),
        entry!(0x0051_29e0, fn_005129e0(Ptr, u32) -> u16),
        entry!(0x0051_29f0, fn_005129f0(Ptr) -> Ptr),
        entry!(0x0051_2ba0, fn_00512ba0(Ptr, u32) -> Ptr),
        entry!(0x0051_2bd0, fn_00512bd0(Ptr)),
        entry!(0x0051_2d40, fn_00512d40(Ptr, u32) -> u32),
        entry!(0x0051_2e10, fn_00512e10(Ptr)),
        entry!(0x0051_28e0, fn_005128e0(Ptr, u32)),
        entry!(0x0051_2940, fn_00512940(Ptr, u32) -> bool),
    ]
}

#[cfg(test)]
mod b023_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != skip)
            .collect()
    }

    #[test]
    fn destructor_body_and_start_values() {
        let mut e = Engine::new();
        for (_, d) in COMPONENT_DESTRUCTORS {
            e.register(d, |_, _| Ret::default());
        }
        for a in [0x0050_e9f0, 0x0040_3d30, 0x0041_fd00] {
            e.register(a, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x190);
        e.call_log = Some(vec![]);
        e.call(0x0051_2bd0, &[this]);
        let log = calls(&mut e, 0x0051_2bd0);
        let mut expected: Vec<(u32, Vec<u32>)> = COMPONENT_DESTRUCTORS
            .iter()
            .map(|(o, d)| (*d, vec![this + o]))
            .collect();
        expected.push((0x0050_e9f0, vec![this]));
        assert_eq!(log, expected);
        assert_eq!(e.mem.u32(this), 0x0102_a62c);
        assert_eq!(e.mem.u32(this + 0x16c), 0x0102_a524);

        e.mem.set_u8(this + 0x18c, 1);
        e.mem.set_u32(this + 0x188, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0051_2e10, &[this]);
        assert_eq!(
            calls(&mut e, 0x0051_2e10),
            vec![
                (0x0040_3d30, vec![this + 0x178, 0, 0xc]),
                (0x0041_fd00, vec![this + 0x48, 3]),
            ]
        );
        assert_eq!(e.mem.u8(this + 0x18c), 0);
        assert_eq!(e.mem.u32(this + 0x188), 0);
    }

    #[test]
    fn picks_the_first_entry_with_the_key_that_wins_the_roll() {
        let mut e = Engine::new();
        e.register(0x0082_56d0, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
        });
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0072_6070, |e, a| ret(e.mem.u32(a[0] + 4)));
        // The roll is 0x7f & 0x63 = 0x63 (99).
        e.register(0x0048_7f50, |_, _| ret(0xffff_ff7f));
        let entry = |e: &mut Engine, form: u32, chance: u8, key: u32| {
            let p = e.mem.alloc(0xc);
            e.mem.set_u32(p, form);
            e.mem.set_u8(p + 4, chance);
            e.mem.set_u32(p + 8, key);
            p
        };
        let entries = [
            entry(&mut e, 0, 200, 7),
            entry(&mut e, 0x11, 200, 8),
            entry(&mut e, 0x22, 99, 7),
            entry(&mut e, 0x33, 100, 7),
            entry(&mut e, 0x44, 200, 7),
        ];
        let nodes = e.mem.alloc(8 * entries.len() as u32);
        for (i, p) in entries.iter().enumerate() {
            let node = nodes + 8 * i as u32;
            e.mem.set_u32(node, *p);
            let next = if i + 1 < entries.len() { node + 8 } else { 0 };
            e.mem.set_u32(node + 4, next);
        }
        let this = e.mem.alloc(0x190);
        e.mem.set_u8(this + 0x18c, 1);
        e.mem.set_u32(this + 0x188, nodes);
        assert_eq!(e.call(0x0051_2d40, &[this, 7]).u32(), 0x33);
        assert_eq!(e.call(0x0051_2d40, &[this, 8]).u32(), 0x11);
        assert_eq!(e.call(0x0051_2d40, &[this, 9]).u32(), 0);
        // Without the byte, the list comes from the template at +0x188.
        let template = e.mem.alloc(0x190);
        e.mem.set_u32(template + 0x188, nodes);
        e.mem.set_u8(this + 0x18c, 0);
        e.mem.set_u32(this + 0x188, template);
        assert_eq!(e.call(0x0051_2d40, &[this, 7]).u32(), 0x33);
        e.mem.set_u32(this + 0x188, 0);
        assert_eq!(e.call(0x0051_2d40, &[this, 7]).u32(), 0);
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
    fn le81_thunk_005129c0() {
        check(0x0051_29c0, 0x0061_6640, 0x58, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_005129d0() {
        check(0x0051_29d0, 0x0059_adb0, 0x58, &[0x77]);
    }

    #[test]
    fn le81_thunk_005129e0() {
        assert_eq!(check(0x0051_29e0, 0x005a_7770, 0x58, &[0x77]).u16(), 0xbeef);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005129f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_29f0,
            size: 0x180,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_cc50, &[A::T(0x3c)]),
                (0x0048_30a0, &[A::T(0x48)]),
                (0x0048_e7a0, &[A::T(0x58)]),
                (0x0048_eac0, &[A::T(0x60)]),
                (0x0048_71d0, &[A::T(0x68)]),
                (0x0048_02f0, &[A::T(0x70)]),
                (0x0047_5350, &[A::T(0x14c)]),
                (0x0047_9320, &[A::T(0x154)]),
                (0x0047_b850, &[A::T(0x15c)]),
                (0x0047_4d10, &[A::T(0x164)]),
                (0x0047_b4f0, &[A::T(0x16c)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x18)]),
                (0x0051_2e10, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_a62c),
                (0x30, 4, 0x0102_a614),
                (0x3c, 4, 0x0102_a600),
                (0x48, 4, 0x0102_a5e8),
                (0x58, 4, 0x0102_a5c8),
                (0x60, 4, 0x0102_a5b4),
                (0x68, 4, 0x0102_a59c),
                (0x70, 4, 0x0102_a588),
                (0x14c, 4, 0x0102_a574),
                (0x154, 4, 0x0102_a560),
                (0x15c, 4, 0x0102_a54c),
                (0x164, 4, 0x0102_a538),
                (0x16c, 4, 0x0102_a524),
            ],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00512ba0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_2bd0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_2ba0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_2ba0)
                .collect();
            let mut expected = vec![(0x0051_2bd0, vec![this.addr()])];
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

    /// The cast answers the form when its word at +0xc is zero.
    fn cast(e: &mut Engine) {
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[2], TYPE_TES_FORM);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn fn_005128e0_copies_from_armour_only() {
        let mut e = Engine::new();
        cast(&mut e);
        e.register(COPY_BYTES, |_, _| Ret::default());
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x200);
        let source = e.mem.alloc(0x200);
        e.mem.set_u32(source + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_28e0, &[this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1].1,
            vec![source, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_ARMO, 0]
        );
        assert_eq!(log.len(), 2);
        e.mem.set_u32(source + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0051_28e0, &[this, source]);
        assert_eq!(
            calls_to(&mut e, &[COPY_BYTES, COPY_ALL_COMPONENTS]),
            vec![
                (COPY_BYTES, vec![this + 0x178, source + 0x178, 0xc]),
                (COPY_ALL_COMPONENTS, vec![this, source]),
            ]
        );
    }

    #[test]
    fn fn_00512940_compares_armour_addons() {
        let mut e = Engine::new();
        cast(&mut e);
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        e.register(MEMCMP, |e, a| {
            assert_eq!(a[2], 0xc);
            Ret {
                eax: u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])),
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x200);
        let other = e.mem.alloc(0x200);
        e.mem.set_u32(other + 0xc, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0051_2940, &[this, other]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1].1,
            vec![other, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_ARMA, 0]
        );
        e.mem.set_u32(other + 0xc, 0);
        assert!(!e.call(0x0051_2940, &[this, other]).bool());
        e.mem.set_u32(other + 0x178, 5);
        assert!(e.call(0x0051_2940, &[this, other]).bool());
        e.mem.set_u32(other + 0x178, 0);
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0051_2940, &[this, other]).bool());
    }
}
