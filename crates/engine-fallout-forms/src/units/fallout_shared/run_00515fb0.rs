//! `fallout shared/run_00515fb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005160d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `00516070`. `00516070` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_005160d0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0051_6070,
        &args![this.addr().wrapping_sub(0x58), first, second],
    );
}

// Translated from 005160e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x58 (`SUB ECX,0x58`) and jumps to `00516040`. `00516040` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_005160e0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0051_6040,
        &args![this.addr().wrapping_sub(0x58), argument],
    );
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `.?AVTESForm@@`
/// (`01183028`) to `.?AVTESObjectCLOT@@` (`01186fcc`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_CLOT: u32 = 0x0118_6fcc;
/// `TESForm::CopyAllComponents` and `CompareAllComponents` (`004851b0`,
/// `00485270`, engine map; `thiscall`, one word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;

/// `form` cast to `TESObjectCLOT`, or 0.
fn cast_to_clot(e: &mut Engine, form: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_CLOT, 0u32],
    )
    .u32()
}

// Translated from 00515fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`; a copy): when `source` casts
/// to `TESObjectCLOT`, `TESForm::CopyAllComponents(source)`.
pub fn fn_00515fb0(e: &mut Engine, this: Ptr, source: u32) {
    if cast_to_clot(e, source) != 0 {
        e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    }
}

// Translated from 00515ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one word, `RET 4`): whether the form differs
/// from `form`: true when it does not cast to `TESObjectCLOT`, else what
/// `TESForm::CompareAllComponents` answers (as 1 or 0).
pub fn fn_00515ff0(e: &mut Engine, this: Ptr, form: u32) -> bool {
    let other = cast_to_clot(e, form);
    if other == 0 {
        return true;
    }
    e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool()
}

// Translated from 00516040 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484c20` on `this`, then `0048e9e0` on `this + 0x58`,
/// each given `argument`. `RET 4`.
pub fn fn_00516040(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4c20, &args![this, argument]);
    e.call(0x0048_e9e0, &args![this.addr() + 0x58, argument]);
}

// Translated from 00516070 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484c50` on `this`, then `0048ea10` on `this + 0x58`,
/// each given both stack words. `RET 8`.
pub fn fn_00516070(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(0x0048_4c50, &args![this, first, second]);
    e.call(0x0048_ea10, &args![this.addr() + 0x58, first, second]);
}

// Translated from 005160a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484da0` on `this`, then `0048ea80` on `this + 0x58`,
/// each given `argument`. `RET 4`.
pub fn fn_005160a0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4da0, &args![this, argument]);
    e.call(0x0048_ea80, &args![this.addr() + 0x58, argument]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_60d0, fn_005160d0(Ptr, u32, u32)),
        entry!(0x0051_60e0, fn_005160e0(Ptr, u32)),
        entry!(0x0051_5fb0, fn_00515fb0(Ptr, u32)),
        entry!(0x0051_5ff0, fn_00515ff0(Ptr, u32) -> bool),
        entry!(0x0051_6040, fn_00516040(Ptr, u32)),
        entry!(0x0051_6070, fn_00516070(Ptr, u32, u32)),
        entry!(0x0051_60a0, fn_005160a0(Ptr, u32)),
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
    fn le81_thunk_005160d0() {
        check(0x0051_60d0, 0x0051_6070, 0x58, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_005160e0() {
        check(0x0051_60e0, 0x0051_6040, 0x58, &[0x77]);
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
    fn fn_00515fb0_and_00515ff0_copy_and_compare_clothing() {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |e, a| {
            assert_eq!(a[1..], [0, TYPE_TES_FORM, TYPE_TES_OBJECT_CLOT, 0]);
            let form = if e.mem.u32(a[0] + 0xc) == 0 { a[0] } else { 0 };
            Ret {
                eax: form,
                ..Ret::default()
            }
        });
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        e.register(COMPARE_ALL_COMPONENTS, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x10),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 0xc, 1);
        e.call_log = Some(vec![]);
        e.call(0x0051_5fb0, &[this, other]);
        assert!(calls_to(&mut e, &[COPY_ALL_COMPONENTS]).is_empty());
        assert!(e.call(0x0051_5ff0, &[this, other]).bool());
        e.mem.set_u32(other + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0051_5fb0, &[this, other]);
        assert_eq!(
            calls_to(&mut e, &[COPY_ALL_COMPONENTS]),
            vec![(COPY_ALL_COMPONENTS, vec![this, other])]
        );
        assert!(!e.call(0x0051_5ff0, &[this, other]).bool());
        e.mem.set_u32(other + 0x10, 1);
        assert!(e.call(0x0051_5ff0, &[this, other]).bool());
    }

    #[test]
    fn fn_00516040_and_friends_run_form_then_component() {
        let cases: [(u32, u32, u32, &[u32]); 3] = [
            (0x0051_6040, 0x0048_4c20, 0x0048_e9e0, &[7]),
            (0x0051_6070, 0x0048_4c50, 0x0048_ea10, &[7, 8]),
            (0x0051_60a0, 0x0048_4da0, 0x0048_ea80, &[7]),
        ];
        for (addr, form, component, extra) in cases {
            let mut e = Engine::new();
            e.register(form, |_, _| Ret::default());
            e.register(component, |_, _| Ret::default());
            let mut words = vec![0x1000];
            words.extend_from_slice(extra);
            e.call_log = Some(vec![]);
            e.call(addr, &words);
            let mut moved = vec![0x1058];
            moved.extend_from_slice(extra);
            assert_eq!(
                calls_to(&mut e, &[form, component]),
                vec![(form, words.clone()), (component, moved)]
            );
        }
    }
}
