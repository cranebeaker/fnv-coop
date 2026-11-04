//! `fallout shared/run_005805c0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vtables `0x0103_168c` and `0x0103_1674` the constructor `005807f0`
//! and the destructor `005808e0` store have the RTTI name `TESWeather`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `BGSEntryPoint::HandleEntryPoint` (`005e58f0`, cdecl: entry point,
/// actor, then the value to modify).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The game's deallocator (`00401030`, cdecl pointer).
const DEALLOCATE: u32 = 0x0040_1030;
/// Called on the buffer with an out word and the mask; its result is the
/// word `004280f0` tests (`00428110`, `thiscall`, two words).
const CHANGE_FLAGS_00428110: u32 = 0x0042_8110;
/// The same shape as [`CHANGE_FLAGS_00428110`], used by the revert (`0042ce30`).
const CHANGE_FLAGS_0042CE30: u32 = 0x0042_ce30;
/// Tests the flags word (`004280f0`, `thiscall`, one mask word).
const FLAGS_HAVE: u32 = 0x0042_80f0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESForm` type descriptor (`.?AVTESForm@@`).
const TESFORM_TYPE: u32 = 0x0118_3028;
/// `TESWaterForm` type descriptor (`.?AVTESWaterForm@@`).
const TESWATERFORM_TYPE: u32 = 0x0118_4118;
/// The bit `005806e0`, `00580730` and `005807a0` test, and `00580680` passes to its virtual calls.
const CHANGE_BIT: u32 = 0x8000_0000;

/// A float copied through the x87 stack (`FLD`/`FSTP`): a signalling NaN
/// comes out quiet.
fn quiet(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

// Translated from 005805c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the object at `this + 0x178` is set and `actor` is not null: asks
/// `actor` for an object through its virtual slot `0x198`; if there is one,
/// starts a multiplier at 1.0, lets `BGSEntryPoint::HandleEntryPoint`
/// (entry point 0x1a) modify it when the actor's slot `0x100` answers true,
/// and calls the object's slot `0x0c` with (`this+0x178` + 0x18 or null, 0,
/// 0, multiplier, 1). Returns whether that call was made. The function
/// takes five stack words and reads only the second.
pub fn fn_005805c0(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    actor: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> bool {
    let base = this.addr();
    if e.mem.u32(base + 0x178) == 0 || actor == 0 {
        return false;
    }
    let target = e.vcall(actor, 0x198, &args![]).u32();
    if target == 0 {
        return false;
    }
    e.with_stack(4, |e, multiplier| {
        e.mem.set_f32(multiplier.addr(), 1.0);
        if e.vcall(actor, 0x100, &args![]).bool() {
            e.call(HANDLE_ENTRY_POINT, &args![0x1au32, actor, multiplier]);
        }
        let source = e.mem.u32(base + 0x178);
        let source_part = if source == 0 { 0 } else { source + 0x18 };
        let value = quiet(e.mem.f32(multiplier.addr()));
        e.vcall(target, 0x0c, &args![source_part, 0u32, 0u32, value, 1u32]);
    });
    true
}

// Translated from 00580680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `other` at `+0x80` and marks the change bit `0x8000_0000`
/// through the virtual slot `0x48`; when `other` is the object itself,
/// clears `+0x80` instead and calls slot `0x4c` with the same bit.
pub fn fn_00580680(e: &mut Engine, this: Ptr, other: u32) {
    let base = this.addr();
    if base == other {
        e.mem.set_u32(base + 0x80, 0);
        e.vcall(base, 0x4c, &args![CHANGE_BIT]);
    } else {
        e.mem.set_u32(base + 0x80, other);
        e.vcall(base, 0x48, &args![CHANGE_BIT]);
    }
}

// Translated from 005806e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00484d60(buffer)` on the object, then, when `00428110` and
/// `004280f0` on `buffer` report bit `0x8000_0000`, writes the form at
/// `+0x80` through `00865df0(form, 0)` on `buffer` (the engine map names it
/// `BGSSaveGameBuffer::SaveFormID`).
pub fn fn_005806e0(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4d60, &args![this, buffer]);
    let has = e.with_stack(4, |e, flags| {
        let word = e
            .call(CHANGE_FLAGS_00428110, &args![buffer, flags, CHANGE_BIT])
            .u32();
        e.call(FLAGS_HAVE, &args![word, CHANGE_BIT]).bool()
    });
    if has {
        let form = e.mem.u32(this.addr() + 0x80);
        e.call(0x0086_5df0, &args![buffer, form, 0u32]);
    }
}

// Translated from 00580730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00484da0(buffer)` on the object, then, when `00428110` and
/// `004280f0` on `buffer` report bit `0x8000_0000`, reads a form id
/// (`008648a0` on `buffer`, `BGSLoadGameBuffer::LoadFormID` in the map), looks the form up (`004839c0`) and stores it
/// at `+0x80` cast from `TESForm` to `TESWaterForm` (`__RTDynamicCast`).
pub fn fn_00580730(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0048_4da0, &args![this, buffer]);
    let has = e.with_stack(4, |e, flags| {
        let word = e
            .call(CHANGE_FLAGS_00428110, &args![buffer, flags, CHANGE_BIT])
            .u32();
        e.call(FLAGS_HAVE, &args![word, CHANGE_BIT]).bool()
    });
    if has {
        let id = e.call(0x0086_48a0, &args![buffer]).u32();
        let form = e.call(0x0048_39c0, &args![id]).u32();
        let water = e
            .call(
                DYNAMIC_CAST,
                &args![form, 0u32, TESFORM_TYPE, TESWATERFORM_TYPE, 0u32],
            )
            .u32();
        e.mem.set_u32(this.addr() + 0x80, water);
    }
}

// Translated from 005807a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004534f0(buffer)` on the object, then clears
/// `+0x80` when `0042ce30` and `004280f0` on `buffer`
/// report bit `0x8000_0000`.
pub fn fn_005807a0(e: &mut Engine, this: Ptr, buffer: u32) {
    e.call(0x0045_34f0, &args![this, buffer]);
    let has = e.with_stack(4, |e, flags| {
        let word = e
            .call(CHANGE_FLAGS_0042CE30, &args![buffer, flags, CHANGE_BIT])
            .u32();
        e.call(FLAGS_HAVE, &args![word, CHANGE_BIT]).bool()
    });
    if has {
        e.mem.set_u32(this.addr() + 0x80, 0);
    }
}

// Translated from 005807f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005807f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x35 (0x36c bytes); `ECX` is the object.
/// The vtables it stores (`0x0103_168c` at `+0`, `0x0103_1674` at `+0x18`) have the RTTI name `TESWeather`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005807f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0058_1fd0, &args![base + 0x18]);
    e.mem.set_u32(base, 0x0103_168c);
    e.mem.set_u32(base + 0x18, 0x0103_1674);
    // `eh vector constructor iterator` is stdcall with five words (`RET 0x14`): no `this`.
    e.call(
        0x00ec_782f,
        &args![base + 0x34, 0xc, 0x4, 0x0051_b9b0, 0x0040_2c40],
    );
    e.call(0x0048_8f50, &args![base + 0xc8]);
    e.call(0x0068_0890, &args![base + 0x1f8]);
    e.call(0x004f_15a0, &args![this, 0x35]);
    e.call(0x0058_1b40, &args![this]);
    this
}

// Translated from 005808b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `TESWeather`: the destructor `005808e0`,
/// then frees the object when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005808b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005808e0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 005808e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005808e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Destructor of `TESWeather`: stores both vtables, runs `00581be0` on the
/// object, destroys the components at `+0x1f8` (`007b9000`) and `+0xc8`
/// (`00489070`) and the four 12-byte elements at `+0x34` (`00ec5fce` with
/// `00402c40`), then the `TESForm` base destructor (`00483630`).
pub fn fn_005808e0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_168c);
    e.mem.set_u32(base + 0x18, 0x0103_1674);
    e.call(0x0058_1be0, &args![this]);
    e.call(0x007b_9000, &args![base + 0x1f8]);
    e.call(0x0048_9070, &args![base + 0xc8]);
    e.call(
        0x00ec_5fce,
        &args![base + 0x34, 0xcu32, 0x4u32, 0x0040_2c40u32],
    );
    e.call(0x0048_3630, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0058_05c0,
            fn_005805c0(Ptr, u32, u32, u32, u32, u32) -> bool
        ),
        entry!(0x0058_0680, fn_00580680(Ptr, u32)),
        entry!(0x0058_06e0, fn_005806e0(Ptr, u32)),
        entry!(0x0058_0730, fn_00580730(Ptr, u32)),
        entry!(0x0058_07a0, fn_005807a0(Ptr, u32)),
        entry!(0x0058_07f0, fn_005807f0(Ptr) -> Ptr),
        entry!(0x0058_08b0, fn_005808b0(Ptr, u32) -> Ptr),
        entry!(0x0058_08e0, fn_005808e0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const ACTOR_VTABLE: u32 = 0x6100_0000;
    const TARGET_VTABLE: u32 = 0x6200_0000;
    const SLOT_198: u32 = 0x6300_0198;
    const SLOT_100: u32 = 0x6300_0100;
    const SLOT_0C: u32 = 0x6300_000c;

    fn entry_point_engine(has_target: bool, perk: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let mut actor_slots = vec![0u32; 0x19c / 4];
        actor_slots[0x198 / 4] = SLOT_198;
        actor_slots[0x100 / 4] = SLOT_100;
        e.put_vtable(ACTOR_VTABLE, &actor_slots);
        e.put_vtable(TARGET_VTABLE, &[0, 0, 0, SLOT_0C]);
        let actor = e.mem.alloc(8);
        e.mem.set_u32(actor, ACTOR_VTABLE);
        let target = e.mem.alloc(8);
        e.mem.set_u32(target, TARGET_VTABLE);
        let answer = if has_target { target } else { 0 };
        e.register_double(SLOT_198, move |_, _| answer.into_ret());
        e.register_double(SLOT_100, move |_, _| u32::from(perk).into_ret());
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            let value = e.mem.f32(a[2]);
            e.mem.set_f32(a[2], value * 2.5);
            Ret::default()
        });
        e.register(SLOT_0C, noop);
        (e, actor, target)
    }

    #[test]
    fn fn_005805c0_passes_the_modified_multiplier() {
        let (mut e, actor, target) = entry_point_engine(true, true);
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x178, 0x7000);
        e.call_log = Some(vec![]);
        let made = e
            .call(0x0058_05c0, &args![this, 9u32, actor, 0u32, 0u32, 0u32])
            .bool();
        assert!(made);
        let entry = calls_to(&e, HANDLE_ENTRY_POINT);
        assert_eq!(entry.len(), 1);
        assert_eq!(&entry[0][..2], &[0x1a, actor]);
        assert_eq!(
            calls_to(&e, SLOT_0C),
            vec![vec![target, 0x7018, 0, 0, 2.5f32.to_bits(), 1]]
        );
    }

    #[test]
    fn fn_005805c0_keeps_one_without_the_entry_point() {
        let (mut e, actor, target) = entry_point_engine(true, false);
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x178, 0x7000);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0058_05c0, &args![this, 0u32, actor, 0u32, 0u32, 0u32])
            .bool());
        assert!(calls_to(&e, HANDLE_ENTRY_POINT).is_empty());
        assert_eq!(
            calls_to(&e, SLOT_0C),
            vec![vec![target, 0x7018, 0, 0, 1.0f32.to_bits(), 1]]
        );
    }

    #[test]
    fn fn_005805c0_answers_false_without_source_actor_or_target() {
        let (mut e, actor, _) = entry_point_engine(false, true);
        let this = e.mem.alloc(0x180);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0058_05c0, &args![this, 0u32, actor, 0u32, 0u32, 0u32])
            .bool());
        e.mem.set_u32(this + 0x178, 0x7000);
        assert!(!e
            .call(0x0058_05c0, &args![this, 0u32, 0u32, 0u32, 0u32, 0u32])
            .bool());
        assert!(!e
            .call(0x0058_05c0, &args![this, 0u32, actor, 0u32, 0u32, 0u32])
            .bool());
        assert!(calls_to(&e, SLOT_0C).is_empty());
        assert_eq!(calls_to(&e, SLOT_198).len(), 1);
    }

    #[test]
    fn fn_00580680_sets_or_clears_the_water() {
        let mut e = Engine::new();
        e.put_vtable(0x6400_0000, &[0; 0x50 / 4]);
        e.mem.set_u32(0x6400_0048, 0x6500_0048);
        e.mem.set_u32(0x6400_004c, 0x6500_004c);
        e.register(0x6500_0048, noop);
        e.register(0x6500_004c, noop);
        let this = e.mem.alloc(0x84);
        e.mem.set_u32(this, 0x6400_0000);
        e.call_log = Some(vec![]);
        e.call(0x0058_0680, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0x80), 0x1234);
        assert_eq!(calls_to(&e, 0x6500_0048), vec![vec![this, 0x8000_0000]]);
        e.call(0x0058_0680, &args![this, this]);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(calls_to(&e, 0x6500_004c), vec![vec![this, 0x8000_0000]]);
    }

    fn change_flags_engine(set: bool) -> Engine {
        let mut e = Engine::new();
        for a in [0x0048_4d60, 0x0048_4da0, 0x0045_34f0, 0x0086_5df0] {
            e.register(a, noop);
        }
        e.register(CHANGE_FLAGS_00428110, |_, _| 0x5000u32.into_ret());
        e.register(CHANGE_FLAGS_0042CE30, |_, _| 0x5000u32.into_ret());
        e.register_double(FLAGS_HAVE, move |_, _| u32::from(set).into_ret());
        e.register(0x0086_48a0, |_, _| 0x99u32.into_ret());
        e.register(0x0048_39c0, |_, _| 0x7700u32.into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        e
    }

    #[test]
    fn fn_005806e0_saves_the_water_form_when_flagged() {
        for set in [false, true] {
            let mut e = change_flags_engine(set);
            let this = e.mem.alloc(0x84);
            e.mem.set_u32(this + 0x80, 0x4242);
            e.call_log = Some(vec![]);
            e.call(0x0058_06e0, &args![this, 0x3000u32]);
            assert_eq!(calls_to(&e, 0x0048_4d60), vec![vec![this, 0x3000]]);
            let flags = calls_to(&e, CHANGE_FLAGS_00428110);
            assert_eq!(flags[0][0], 0x3000);
            assert_eq!(flags[0][2], 0x8000_0000);
            assert_eq!(calls_to(&e, FLAGS_HAVE), vec![vec![0x5000, 0x8000_0000]]);
            let saved = calls_to(&e, 0x0086_5df0);
            if set {
                assert_eq!(saved, vec![vec![0x3000, 0x4242, 0]]);
            } else {
                assert!(saved.is_empty());
            }
        }
    }

    #[test]
    fn fn_00580730_loads_and_casts_the_water_form() {
        let mut e = change_flags_engine(true);
        let this = e.mem.alloc(0x84);
        e.call_log = Some(vec![]);
        e.call(0x0058_0730, &args![this, 0x3000u32]);
        assert_eq!(calls_to(&e, 0x0048_4da0), vec![vec![this, 0x3000]]);
        assert_eq!(calls_to(&e, 0x0086_48a0), vec![vec![0x3000]]);
        assert_eq!(calls_to(&e, 0x0048_39c0), vec![vec![0x99]]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![0x7700, 0, TESFORM_TYPE, TESWATERFORM_TYPE, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x80), 0x7701);

        let mut e = change_flags_engine(false);
        let this = e.mem.alloc(0x84);
        e.mem.set_u32(this + 0x80, 5);
        e.call_log = Some(vec![]);
        e.call(0x0058_0730, &args![this, 0x3000u32]);
        assert!(calls_to(&e, 0x0086_48a0).is_empty());
        assert_eq!(e.mem.u32(this + 0x80), 5);
    }

    #[test]
    fn fn_005807a0_clears_the_water_form_when_flagged() {
        for set in [false, true] {
            let mut e = change_flags_engine(set);
            let this = e.mem.alloc(0x84);
            e.mem.set_u32(this + 0x80, 0x4242);
            e.call_log = Some(vec![]);
            e.call(0x0058_07a0, &args![this, 0x3000u32]);
            assert_eq!(calls_to(&e, 0x0045_34f0), vec![vec![this, 0x3000]]);
            assert_eq!(calls_to(&e, CHANGE_FLAGS_0042CE30)[0][0], 0x3000);
            assert_eq!(e.mem.u32(this + 0x80), if set { 0 } else { 0x4242 });
        }
    }

    #[test]
    fn fn_005807f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0058_07f0,
            size: 0x36c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0058_1fd0, &[A::T(0x18)]),
                (
                    0x00ec_782f,
                    &[
                        A::T(0x34),
                        A::V(0xc),
                        A::V(0x4),
                        A::V(0x0051_b9b0),
                        A::V(0x0040_2c40),
                    ],
                ),
                (0x0048_8f50, &[A::T(0xc8)]),
                (0x0068_0890, &[A::T(0x1f8)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x35)]),
                (0x0058_1b40, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0103_168c), (0x18, 4, 0x0103_1674)],
        });
    }

    fn destructor_engine() -> Engine {
        let mut e = Engine::new();
        for a in [
            0x0058_1be0,
            0x007b_9000,
            0x0048_9070,
            0x00ec_5fce,
            0x0048_3630,
            DEALLOCATE,
        ] {
            e.register(a, noop);
        }
        e
    }

    #[test]
    fn fn_005808e0_destroys_the_parts_in_order() {
        let mut e = destructor_engine();
        let this = e.mem.alloc(0x36c);
        e.call_log = Some(vec![]);
        e.call(0x0058_08e0, &args![this]);
        let log = e.call_log.take().unwrap();
        let seen: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        assert_eq!(
            seen,
            vec![
                (0x0058_1be0, vec![this]),
                (0x007b_9000, vec![this + 0x1f8]),
                (0x0048_9070, vec![this + 0xc8]),
                (0x00ec_5fce, vec![this + 0x34, 0xc, 4, 0x0040_2c40]),
                (0x0048_3630, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0103_168c);
        assert_eq!(e.mem.u32(this + 0x18), 0x0103_1674);
    }

    #[test]
    fn fn_005808b0_frees_only_with_bit_0() {
        let mut e = destructor_engine();
        let this = e.mem.alloc(0x36c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_08b0, &args![this, 2u32]).u32(), this);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        assert_eq!(e.call(0x0058_08b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x0048_3630).len(), 2);
    }
}
