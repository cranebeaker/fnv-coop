//! `fallout shared/run_004fc860` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004fcbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x30 (`SUB ECX,0x30`) and jumps to `004fc240`. `004fc240` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_004fcbf0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x004f_c240, &args![this.addr().wrapping_sub(0x30), flags])
        .u32()
}

/// `00624700`: returns `this + 0x48` (the flags word of the water form).
const FLAGS_ADDRESS: u32 = 0x0062_4700;

// Translated from 004fc990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` nonzero) or clears (`value` zero) the bits `mask` of the
/// flags word whose address `00624700` returns for `this` (`RET 8`).
pub fn fn_004fc990(e: &mut Engine, this: Ptr, mask: u32, value: u8) {
    if value != 0 {
        let flags = e.call(FLAGS_ADDRESS, &args![this]).u32();
        let word = e.mem.u32(flags) | mask;
        e.mem.set_u32(flags, word);
    } else {
        let flags = e.call(FLAGS_ADDRESS, &args![this]).u32();
        let word = !mask & e.mem.u32(flags);
        e.mem.set_u32(flags, word);
    }
}

// Translated from 004fc940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The form at +0x4C, or what `004fb070` (`bgsexplosion.cpp`, thiscall on
/// that form) gives for it when the form is there, form flag 0x8 is set
/// (`004013e0`) and `004fb070` gives nonzero (it is then called a second time
/// for the result).
pub fn fn_004fc940(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + 0x4c) != 0 && e.call(0x0040_13e0, &args![this]).bool() {
        let form = e.mem.u32(base + 0x4c);
        if e.call(0x004f_b070, &args![form]).u32() != 0 {
            let form = e.mem.u32(base + 0x4c);
            return e.call(0x004f_b070, &args![form]).u32();
        }
    }
    e.mem.u32(base + 0x4c)
}

// Translated from 004fc860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies from `other` (`RET 4`) when it casts (`__RTDynamicCast` from `01183028` to
/// `01189d70`): the components (`TESForm::CopyAllComponents`), then the 8 bytes at the
/// flags address (`00624700`) of the cast object to this one's (`00401460`).
pub fn fn_004fc860(e: &mut Engine, this: Ptr, other: u32) {
    let cast = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_9d70u32, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    e.call(0x0048_51b0, &args![this, cast]);
    let source = e.call(FLAGS_ADDRESS, &args![cast]).u32();
    let destination = e.call(FLAGS_ADDRESS, &args![this]).u32();
    e.call(0x0040_1460, &args![destination, source, 8u32]);
}

// Translated from 004fc8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs (`RET 4`, `AL`): true unless it casts (`__RTDynamicCast` from
/// `01183028` to `01189d70`), `TESForm::CompareAllComponents` finds no difference and the 8
/// bytes at +0x48 equal those at the cast object's flags address (`00624700`, `memcmp`
/// `00ec4835`).
pub fn fn_004fc8c0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let cast = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, 0x0118_3028u32, 0x0118_9d70u32, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    if e.call(0x0048_5270, &args![this, cast]).bool() {
        return true;
    }
    let theirs = e.call(FLAGS_ADDRESS, &args![cast]).u32();
    e.call(0x00ec_4835, &args![this.addr() + 0x48, theirs, 8u32])
        .u32()
        != 0
}

// Translated from 004fc9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Activation through the form's target (`RET 0x14`, `AL`): false when vtable slot 0x140
/// gives none. When `actor` is set and its slot 0x100 answers, the idle manager's used item
/// is set around a test (`TESIdleManager::SetUsedItem` `00600900` with this form,
/// `00600940` with 1, `00600920` with whether `reference`'s slot 0x218 answers or the
/// actor's eye level (`Actor::GetEyeLevel`) plus the float at +8 of its slot 0x1F4 minus its
/// slot 0x380 is below the float at +8 of `reference`'s slot 0x1F4), the test being the
/// actor's saved acquire object's (`008d8520`) slot 0x44 (`actor`, 0, 3, 1, 0, 1), then reset
/// (0, 0, -1); a failed test answers false, and for the player (`011dea3c`) the target goes to
/// `011e0788`, `004fcbd0` gets 1 and both animations (`00950a60` with 1 and 0) have their
/// group 1 cleared (`00496080` with 0.0). Then the target's slot 0x124 (`reference`, `actor`,
/// `flag`, `fourth`, `fifth`) answers.
pub fn fn_004fc9e0(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    actor: u32,
    flag: u8,
    fourth: u32,
    fifth: u32,
) -> bool {
    let target = e.vcall(this.addr(), 0x140, &args![]).u32();
    if target == 0 {
        return false;
    }
    if actor != 0 && e.vcall(actor, 0x100, &args![]).bool() {
        e.call(0x0060_0900, &args![this]);
        e.call(0x0060_0940, &args![1u32]);
        let eye = e.call(0x008b_e940, &args![actor]).f64();
        let position = e.vcall(actor, 0x1f4, &args![]).u32();
        let raised = x87_add(x87_load32(e.mem.u32(position + 8)), eye);
        let lowered = e.vcall(actor, 0x380, &args![]).f64();
        let height = x87_store32(x87_sub(raised, lowered));
        let position = e.vcall(reference, 0x1f4, &args![]).u32();
        let other_height = x87_quiet_bits(e.mem.u32(position + 8));
        // FCOMPP, `TEST AH,0x5`, `JP`: set only when ordered and below.
        let below = u32::from(
            e.vcall(reference, 0x218, &args![]).bool()
                || x87_load32(height) < x87_load32(other_height),
        );
        e.call(0x0060_0920, &args![below]);
        let acquire = e.call(0x008d_8520, &args![actor]).u32();
        let passed = e
            .vcall(acquire, 0x44, &args![actor, 0u32, 3u32, 1u32, 0u32, 1u32])
            .u8();
        e.call(0x0060_0900, &args![0u32]);
        e.call(0x0060_0940, &args![0u32]);
        e.call(0x0060_0920, &args![0xffff_ffffu32]);
        if passed == 0 {
            return false;
        }
        let player = e.global::<u32>(0x011d_ea3c);
        if actor == player && passed != 0 {
            e.set_global::<u32>(0x011e_0788, target);
            let player = e.global::<u32>(0x011d_ea3c);
            fn_004fcbd0(e, Ptr::new(player), 1);
            for first_person in [1u32, 0] {
                let player = e.global::<u32>(0x011d_ea3c);
                let animation = e.call(0x0095_0a60, &args![player, first_person]).u32();
                e.call(0x0049_6080, &args![animation, 1u32, 0u32]);
            }
        }
    }
    e.vcall(
        target,
        0x124,
        &args![reference, actor, u32::from(flag), fourth, fifth],
    )
    .bool()
}

// Translated from 004fcbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at +0x79A (`RET 4`).
pub fn fn_004fcbd0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x79a, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_cbf0, fn_004fcbf0(Ptr, u32) -> u32),
        entry!(0x004f_c990, fn_004fc990(Ptr, u32, u8)),
        entry!(0x004f_c940, fn_004fc940(Ptr) -> u32),
        entry!(0x004f_c860, fn_004fc860(Ptr, u32)),
        entry!(0x004f_c8c0, fn_004fc8c0(Ptr, u32) -> bool),
        entry!(0x004f_c9e0, fn_004fc9e0(Ptr, u32, u32, u8, u32, u32) -> bool),
        entry!(0x004f_cbd0, fn_004fcbd0(Ptr, u8)),
    ]
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
    fn le81_thunk_004fcbf0() {
        assert_eq!(
            check(0x004f_cbf0, 0x004f_c240, 0x30, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod flag_tests {
    use crate::prelude::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn at_004fc990_sets_or_clears_the_mask() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        e.register(0x0062_4700, |_, a| ret(a[0] + 0x48));
        e.mem.set_u32(this + 0x48, 0x0000_00f0);
        e.call(0x004f_c990, &[this, 0x8000_0001, 1]);
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_00f1);
        e.call(0x004f_c990, &[this, 0x0000_0030, 0x100]);
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_00c1);
        e.call(0x004f_c990, &[this, 0x0000_0002, 0x80]);
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_00c3);
    }

    fn getter(form: u32, flag: u32, mapped: u32) -> (u32, usize) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x4c, form);
        e.register_double(0x0040_13e0, move |_, _| ret(flag));
        e.register_double(0x004f_b070, move |_, _| ret(mapped));
        e.call_log = Some(vec![]);
        let r = e.call(0x004f_c940, &[this]).u32();
        let n = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == 0x004f_b070)
            .count();
        (r, n)
    }

    #[test]
    fn at_004fc940_maps_the_form_only_when_flag_8_and_a_mapping_exist() {
        assert_eq!(getter(0, 1, 0x77), (0, 0));
        assert_eq!(getter(0x40, 0x100, 0x77), (0x40, 0));
        assert_eq!(getter(0x40, 1, 0), (0x40, 1));
        assert_eq!(getter(0x40, 1, 0x77), (0x77, 2));
    }
}

#[cfg(test)]
mod n039_tests {
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
    fn fn_004fc860_copies_components_and_flags() {
        let mut e = Engine::new();
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x0048_51b0, |_, _| Ret::default());
        e.register(FLAGS_ADDRESS, |_, a| (a[0] + 0x48).into_ret());
        e.register(0x0040_1460, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004f_c860, &args![0x1000u32, 0x2000u32]);
        assert_eq!(calls_to(&e, 0x0048_51b0), vec![vec![0x1000, 0x2000]]);
        assert_eq!(calls_to(&e, 0x0040_1460), vec![vec![0x1048, 0x2048, 8]]);
        e.register(0x00ec_43fb, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004f_c860, &args![0x1000u32, 0x2000u32]);
        assert!(calls_to(&e, 0x0040_1460).is_empty());
    }

    #[test]
    fn fn_004fc8c0_compares_components_then_flags() {
        let mut e = Engine::new();
        e.register(0x00ec_43fb, |_, a| a[0].into_ret());
        e.register(0x0048_5270, |_, _| false.into_ret());
        e.register(FLAGS_ADDRESS, |_, a| (a[0] + 0x48).into_ret());
        e.register(0x00ec_4835, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_c8c0, &args![0x1000u32, 0x2000u32]).bool());
        assert_eq!(calls_to(&e, 0x00ec_4835), vec![vec![0x1048, 0x2048, 8]]);
        e.register(0x00ec_4835, |_, _| 1u32.into_ret());
        assert!(e.call(0x004f_c8c0, &args![0x1000u32, 0x2000u32]).bool());
        e.register(0x0048_5270, |_, _| true.into_ret());
        assert!(e.call(0x004f_c8c0, &args![0x1000u32, 0x2000u32]).bool());
        e.register(0x00ec_43fb, |_, _| Ret::default());
        assert!(e.call(0x004f_c8c0, &args![0x1000u32, 0x2000u32]).bool());
    }

    #[test]
    fn fn_004fc9e0_tests_the_player_activation() {
        let mut e = Engine::new();
        // One vtable for every object: slot 0x140 the target, 0x100 true, 0x1F4 a position,
        // 0x380 1.0, 0x218 false, 0x44 the test, 0x124 the activation.
        let position = e.mem.alloc(12);
        e.mem.set_u32(position + 8, 5.0f32.to_bits());
        let mut table = vec![0x00e8_0000u32; 0x200];
        table[0x140 / 4] = 0x00e8_0140;
        table[0x100 / 4] = 0x00e8_0100;
        table[0x1f4 / 4] = 0x00e8_01f4;
        table[0x380 / 4] = 0x00e8_0380;
        table[0x218 / 4] = 0x00e8_0218;
        table[0x44 / 4] = 0x00e8_0044;
        table[0x124 / 4] = 0x00e8_0124;
        e.put_vtable(0x00db_0000, &table);
        let object = |e: &mut Engine| {
            let o = e.mem.alloc(0x800);
            e.mem.set_u32(o, 0x00db_0000);
            o
        };
        let this = object(&mut e);
        let target = object(&mut e);
        let actor = object(&mut e);
        let reference = object(&mut e);
        e.register_double(0x00e8_0140, move |_, _| target.into_ret());
        e.register(0x00e8_0100, |_, _| true.into_ret());
        e.register_double(0x00e8_01f4, move |_, _| position.into_ret());
        e.register(0x00e8_0380, |_, _| 1.0f32.into_ret());
        e.register(0x00e8_0218, |_, _| false.into_ret());
        e.register(0x00e8_0044, |_, _| true.into_ret());
        e.register(0x00e8_0124, |_, a| (a[3] == 9).into_ret());
        e.register(0x008b_e940, |_, _| 2.0f32.into_ret());
        e.register(0x008d_8520, |_, a| a[0].into_ret());
        for address in [0x0060_0900, 0x0060_0920, 0x0060_0940, 0x0049_6080] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0095_0a60, |_, a| (0x100 + a[1]).into_ret());
        e.map(0x011d_ea3c, 4);
        e.set_global::<u32>(0x011d_ea3c, actor);
        e.map(0x011e_0788, 4);
        e.call_log = Some(vec![]);
        // Height 5 + 2 - 1 = 6 is not below 5.
        assert!(e
            .call(
                0x004f_c9e0,
                &args![this, reference, actor, 9u32, 4u32, 5u32]
            )
            .bool());
        assert_eq!(calls_to(&e, 0x0060_0920), vec![vec![0], vec![0xffff_ffff]]);
        assert_eq!(
            calls_to(&e, 0x00e8_0044),
            vec![vec![actor, actor, 0, 3, 1, 0, 1]]
        );
        assert_eq!(e.global::<u32>(0x011e_0788), target);
        assert_eq!(e.mem.u8(actor + 0x79a), 1);
        assert_eq!(
            calls_to(&e, 0x0049_6080),
            vec![vec![0x101, 1, 0], vec![0x100, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, 0x00e8_0124),
            vec![vec![target, reference, actor, 9, 4, 5]]
        );
        // Height 5 + 2 - 3 = 4 is below 5.
        e.register(0x00e8_0380, |_, _| 3.0f32.into_ret());
        e.call_log = Some(vec![]);
        e.call(
            0x004f_c9e0,
            &args![this, reference, actor, 9u32, 4u32, 5u32],
        );
        assert_eq!(calls_to(&e, 0x0060_0920)[0], vec![1]);
        // A failed test answers false; no target answers false.
        e.register(0x00e8_0044, |_, _| false.into_ret());
        assert!(!e
            .call(
                0x004f_c9e0,
                &args![this, reference, actor, 9u32, 4u32, 5u32]
            )
            .bool());
        e.register(0x00e8_0140, |_, _| Ret::default());
        assert!(!e
            .call(
                0x004f_c9e0,
                &args![this, reference, actor, 9u32, 4u32, 5u32]
            )
            .bool());
    }

    #[test]
    fn fn_004fcbd0_stores_the_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x800);
        e.call(0x004f_cbd0, &args![this, 0x1234_5601u32]);
        assert_eq!(e.mem.u8(this + 0x79a), 1);
    }
}
