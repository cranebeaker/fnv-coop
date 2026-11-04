//! `(unplaced)/run_005eeb70` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BGSRagdoll`'s `Copy` and `Compare` virtuals, two array element
//! accessors and the constructors and destructors of two `BSSimpleArray`
//! instances.
//!
//! `BGSRagdoll` fields are read at their PC offsets, 0x10 below the Xbox
//! PDB's (`TESForm` is 0x18 bytes on PC): +0x40 `FeedbackData`, +0x90
//! `FootIKData`, +0xd0 `LookIKData`, +0xf4 `GrabIKData`, +0x118
//! `PoseMatchingData`, +0x13c `iSkeletonID`, +0x140 `pBodyPartData`, +0x144
//! `pPreviewActor`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast(object, offset, source type, target type, is_reference)`
/// (MSVC runtime, `cdecl`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `TESForm` RTTI type descriptor (`.?AVTESForm@@`).
const TES_FORM_TYPE: u32 = 0x0118_3028;
/// The `BGSRagdoll` RTTI type descriptor (`.?AVBGSRagdoll@@`).
const BGS_RAGDOLL_TYPE: u32 = 0x0118_614c;
/// `TESForm::CopyAllComponents` (Xbox PDB name in the engine map,
/// `thiscall(this, source)`).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// `TESForm::CompareAllComponents` (Xbox PDB name in the engine map,
/// `thiscall(this, other)`, a `bool` answer that is true when they differ).
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;

/// The field copies `Copy` makes, `thiscall(field, other's field)`, by PC
/// field offset: `FeedbackData`, `FootIKData`, `LookIKData`, `GrabIKData`
/// and `PoseMatchingData` (the engine map names none of them).
const FIELD_COPIES: [(u32, u32); 5] = [
    (0x40, 0x00c7_a790),
    (0x90, 0x005e_cb30),
    (0xd0, 0x005e_cfd0),
    (0xf4, 0x005e_d360),
    (0x118, 0x005e_d5f0),
];
/// The field comparisons `Compare` makes, `thiscall(field, other's field)`
/// (a `bool` answer that is true when they differ), in the same order.
const FIELD_COMPARES: [(u32, u32); 5] = [
    (0x40, 0x00c7_4fb0),
    (0x90, 0x005e_c920),
    (0xd0, 0x005e_ce50),
    (0xf4, 0x005e_d1e0),
    (0x118, 0x005e_d4f0),
];

/// `BSSimpleArray<const char *, 1024>`'s vtable (its RTTI complete object
/// locator, at `0x011103f8`, names `.?AV?$BSSimpleArray@PBD$0EAA@@@`).
const STRING_ARRAY_VTABLE: u32 = 0x0104_72d0;
/// `BSSimpleArray<short, 1024>`'s vtable (locator `0x01110440`,
/// `.?AV?$BSSimpleArray@F$0EAA@@@`).
const SHORT_ARRAY_VTABLE: u32 = 0x0104_72e4;
/// The string array's set-up, `thiscall(this, 0, 0)` (`006b3eb0`).
const STRING_ARRAY_INITIALIZE: u32 = 0x006b_3eb0;
/// The short array's set-up, `thiscall(this, 0, 0)` (`00692290`).
const SHORT_ARRAY_INITIALIZE: u32 = 0x0069_2290;
/// The arrays' release, `thiscall(this, 1)` (`008454f0`).
const ARRAY_RELEASE: u32 = 0x0084_54f0;

/// `source` cast to `BGSRagdoll` (`dynamic_cast` from `TESForm`), or 0.
fn as_ragdoll(e: &mut Engine, source: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![source, 0u32, TES_FORM_TYPE, BGS_RAGDOLL_TYPE, 0u32],
    )
    .u32()
}

// Translated from 005eeb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRagdoll::Copy(source)` (the `TESForm::Copy` slot, +0x108, of the
/// `BGSRagdoll` vtable at `0x010470ec`; `thiscall`): nothing unless `source`
/// is a `BGSRagdoll`. Otherwise copies the components (with `source`), the
/// preview actor, body part data and skeleton id words, then the five data
/// blocks.
pub fn bgs_ragdoll_copy(e: &mut Engine, this: Ptr, source: u32) {
    let other = as_ragdoll(e, source);
    if other == 0 {
        return;
    }
    let this_addr = this.addr();
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    for offset in [0x144u32, 0x140, 0x13c] {
        let word = e.mem.u32(other.wrapping_add(offset));
        e.mem.set_u32(this_addr.wrapping_add(offset), word);
    }
    for (offset, copy) in FIELD_COPIES {
        e.call(
            copy,
            &args![this_addr.wrapping_add(offset), other.wrapping_add(offset)],
        );
    }
}

// Translated from 005eec60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSRagdoll::Compare(other)` (the `TESForm::Compare` slot, +0x10c, of the
/// same vtable; `thiscall`): true when they differ. True when `other` is not
/// a `BGSRagdoll`, when the components differ (compared with the cast
/// pointer), when the skeleton ids differ, when one of the five data blocks
/// differs, or when the body part data differ; checked in that order and
/// stopping at the first difference.
pub fn bgs_ragdoll_compare(e: &mut Engine, this: Ptr, other: u32) -> bool {
    let ragdoll = as_ragdoll(e, other);
    if ragdoll == 0 {
        return true;
    }
    let this_addr = this.addr();
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, ragdoll]).bool() {
        return true;
    }
    if e.mem.u32(this_addr + 0x13c) != e.mem.u32(ragdoll.wrapping_add(0x13c)) {
        return true;
    }
    for (offset, compare) in FIELD_COMPARES {
        let fields = args![this_addr.wrapping_add(offset), ragdoll.wrapping_add(offset)];
        if e.call(compare, &fields).bool() {
            return true;
        }
    }
    e.mem.u32(this_addr + 0x140) != e.mem.u32(ragdoll.wrapping_add(0x140))
}

// Translated from 005eeda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall(this, index)`: the address of 2-byte element `index` of the
/// array whose data pointer is at +4.
pub fn fn_005eeda0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.mem
        .u32(this.addr() + 4)
        .wrapping_add(index.wrapping_mul(2))
}

// Translated from 005eedc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `thiscall(this, index)`: the address of 0x38-byte element `index` of the
/// array whose data pointer is at +4.
pub fn fn_005eedc0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0x38)
        .wrapping_add(e.mem.u32(this.addr() + 4))
}

// Translated from 005eede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<const char *, 1024>` constructor (`thiscall`): the
/// vtable, then `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_005eede0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), STRING_ARRAY_VTABLE);
    e.call(STRING_ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 005eee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<const char *, 1024>` destructor (`thiscall`): the
/// vtable, then `008454f0(this, 1)`.
pub fn fn_005eee10(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), STRING_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE, &args![this, 1u32]);
}

// Translated from 005eee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<short, 1024>` constructor (`thiscall`): the vtable,
/// then `00692290(this, 0, 0)`. Returns `this`.
pub fn fn_005eee30(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SHORT_ARRAY_VTABLE);
    e.call(SHORT_ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 005eee60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<short, 1024>` destructor (`thiscall`): the vtable,
/// then `008454f0(this, 1)`.
pub fn fn_005eee60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SHORT_ARRAY_VTABLE);
    e.call(ARRAY_RELEASE, &args![this, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005eeb70, bgs_ragdoll_copy(Ptr, u32)),
        entry!(0x005eec60, bgs_ragdoll_compare(Ptr, u32) -> bool),
        entry!(0x005eeda0, fn_005eeda0(Ptr, u32) -> u32),
        entry!(0x005eedc0, fn_005eedc0(Ptr, u32) -> u32),
        entry!(0x005eede0, fn_005eede0(Ptr) -> Ptr),
        entry!(0x005eee10, fn_005eee10(Ptr)),
        entry!(0x005eee30, fn_005eee30(Ptr) -> Ptr),
        entry!(0x005eee60, fn_005eee60(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn cast_double(e: &mut Engine, result: u32) {
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!(&a[1..], &[0, TES_FORM_TYPE, BGS_RAGDOLL_TYPE, 0]);
            Ret {
                eax: result,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn copy_skips_other_forms_and_copies_ragdolls() {
        let mut e = Engine::new();
        cast_double(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_eb70, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, COPY_ALL_COMPONENTS).is_empty());

        let mut e = Engine::new();
        let this = e.mem.alloc(0x148);
        let other = e.mem.alloc(0x148);
        cast_double(&mut e, other);
        e.register(COPY_ALL_COMPONENTS, |_, _| Ret::default());
        for (_, copy) in FIELD_COPIES {
            e.register(copy, |_, _| Ret::default());
        }
        e.mem.set_u32(other + 0x13c, 1);
        e.mem.set_u32(other + 0x140, 2);
        e.mem.set_u32(other + 0x144, 3);
        e.call_log = Some(vec![]);
        e.call(0x005e_eb70, &args![this, 0x7777u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, COPY_ALL_COMPONENTS),
            vec![vec![this, 0x7777]]
        );
        assert_eq!(
            [
                e.mem.u32(this + 0x13c),
                e.mem.u32(this + 0x140),
                e.mem.u32(this + 0x144)
            ],
            [1, 2, 3]
        );
        for (offset, copy) in FIELD_COPIES {
            assert_eq!(
                calls_to(&log, copy),
                vec![vec![this + offset, other + offset]]
            );
        }
    }

    fn compare_setup(e: &mut Engine, differing: Option<u32>) -> (u32, u32) {
        let this = e.mem.alloc(0x148);
        let other = e.mem.alloc(0x148);
        cast_double(e, other);
        let components_differ = differing == Some(COMPARE_ALL_COMPONENTS);
        e.register_double(COMPARE_ALL_COMPONENTS, move |_, _| {
            components_differ.into_ret()
        });
        for (_, compare) in FIELD_COMPARES {
            let answer = differing == Some(compare);
            e.register_double(compare, move |_, _| answer.into_ret());
        }
        (this, other)
    }

    #[test]
    fn compare_reports_each_difference_in_order() {
        // Not a ragdoll.
        let mut e = Engine::new();
        cast_double(&mut e, 0);
        assert!(e.call(0x005e_ec60, &args![0x1000u32, 0x2000u32]).bool());
        // Equal in everything.
        let mut e = Engine::new();
        let (this, other) = compare_setup(&mut e, None);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005e_ec60, &args![this, 0x9999u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, COMPARE_ALL_COMPONENTS),
            vec![vec![this, other]]
        );
        for (offset, compare) in FIELD_COMPARES {
            assert_eq!(
                calls_to(&log, compare),
                vec![vec![this + offset, other + offset]]
            );
        }
        // Each callee's difference stops the comparison there.
        for (index, (_, compare)) in FIELD_COMPARES.iter().enumerate() {
            let mut e = Engine::new();
            let (this, _) = compare_setup(&mut e, Some(*compare));
            e.call_log = Some(vec![]);
            assert!(e.call(0x005e_ec60, &args![this, 1u32]).bool());
            let log = e.call_log.take().unwrap();
            if let Some((_, later)) = FIELD_COMPARES.get(index + 1) {
                assert!(calls_to(&log, *later).is_empty());
            }
        }
        let mut e = Engine::new();
        let (this, _) = compare_setup(&mut e, Some(COMPARE_ALL_COMPONENTS));
        e.call_log = Some(vec![]);
        assert!(e.call(0x005e_ec60, &args![this, 1u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FIELD_COMPARES[0].1).is_empty());
        // The skeleton id is compared before the blocks, the body part data
        // after them.
        for (offset, blocks_compared) in [(0x13cu32, false), (0x140, true)] {
            let mut e = Engine::new();
            let (this, other) = compare_setup(&mut e, None);
            e.mem.set_u32(other + offset, 5);
            e.call_log = Some(vec![]);
            assert!(e.call(0x005e_ec60, &args![this, 1u32]).bool());
            let log = e.call_log.take().unwrap();
            assert_eq!(
                !calls_to(&log, FIELD_COMPARES[0].1).is_empty(),
                blocks_compared
            );
        }
    }

    #[test]
    fn element_accessors_scale_the_index() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x1000);
        assert_eq!(e.call(0x005e_eda0, &args![this, 3u32]).u32(), 0x1006);
        assert_eq!(e.call(0x005e_edc0, &args![this, 3u32]).u32(), 0x10a8);
        assert_eq!(e.call(0x005e_eda0, &args![this, u32::MAX]).u32(), 0x0ffe);
    }

    #[test]
    fn simple_array_constructors_and_destructors() {
        for (constructor, destructor, vtable, initialize) in [
            (
                0x005e_ede0u32,
                0x005e_ee10u32,
                STRING_ARRAY_VTABLE,
                STRING_ARRAY_INITIALIZE,
            ),
            (
                0x005e_ee30,
                0x005e_ee60,
                SHORT_ARRAY_VTABLE,
                SHORT_ARRAY_INITIALIZE,
            ),
        ] {
            let mut e = Engine::new();
            e.register(initialize, |e, a| {
                assert_ne!(e.mem.u32(a[0]), 0, "the vtable is stored first");
                Ret::default()
            });
            e.register(ARRAY_RELEASE, |e, a| {
                assert_ne!(e.mem.u32(a[0]), 0, "the vtable is stored first");
                Ret::default()
            });
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(constructor, &args![this]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            e.mem.set_u32(this, 0);
            e.call(destructor, &args![this]);
            assert_eq!(e.mem.u32(this), vtable);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, initialize), vec![vec![this, 0, 0]]);
            assert_eq!(calls_to(&log, ARRAY_RELEASE), vec![vec![this, 1]]);
        }
    }
}
