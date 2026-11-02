//! `fallout/ai/run_008d4cc0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `float` constant -1.0 the code loads from the exe's data.
const MINUS_ONE: u32 = 0x0101_2054;

// Translated from 008d4e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the two armour values of a `Character` (`fArmorRating` at +0x1b8 and
/// `fArmorThreshold` at +0x1bc, Xbox PDB names, same offsets on PC) to -1.0,
/// the constant at `01012054`. The engine map has no name for it.
pub fn fn_008d4e80(e: &mut Engine, this: Ptr) {
    let minus_one: f32 = e.global(MINUS_ONE);
    e.mem.set_f32(this.addr() + 0x1b8, minus_one);
    e.mem.set_f32(this.addr() + 0x1bc, minus_one);
}

/// The double 0.0 the armour values are compared with (`.rdata`).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// The double factor (0.01 in the exe) `fn_008d4eb0` and `fn_008d4f10` scale by.
const PERCENT_DOUBLE: u32 = 0x0103_1148;
/// The float (32.0 in the exe) `fn_008d4f70` answers without an animation object.
const DEFAULT_FLOAT: u32 = 0x0101_e340;
/// `0087d880` (`thiscall`, no stack words, no engine-map name): the base value (`ST0`).
const BASE_VALUE: u32 = 0x0087_d880;
/// `Actor::GetAnimGroup` (engine map, `00897910`; `thiscall`, four words).
const GET_ANIM_GROUP: u32 = 0x0089_7910;
/// `00494300` (`thiscall`, one word; unit `fallout shared/animation.cpp`): answers a
/// 16-bit value for the group.
const ANIMATION_GROUP_VALUE: u32 = 0x0049_4300;
/// The allocation and free used by `fn_008d5020` and `fn_008d50a0` (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// The constructor (`008d5590`) and the function (`008d55b0`) run on the 12-byte object
/// `fn_008d5020` keeps (both `thiscall`, no stack words; another unit).
const HELD_CONSTRUCT: u32 = 0x008d_5590;
const HELD_RUN: u32 = 0x008d_55b0;
/// The object at `this` +0xA4 with its own vtable (by its use, the actor-value interface);
/// its slots +0xC, +0x10, +0x14 and +0x18 take an index and answer `ST0`.
const VALUES: u32 = 0xa4;

/// Shared body of `fn_008d4cc0` and `fn_008d4db0`: the base value (`0087d880`, stored as a
/// float), replaced by the value of `this`'s vtable slot +0x48C `(index, &flag)` when that
/// sets the flag, plus each slot of `VALUES` in `slots` (the first `FADD` of the float, the
/// later ones of the running double), stored at `field` and answered.
fn computed_armour_value(e: &mut Engine, this: u32, field: u32, index: u32, slots: &[u32]) -> f32 {
    let mut base = x87_store32(e.call(BASE_VALUE, &args![this]).f64());
    e.with_stack(4, |e, flag| {
        let flag = flag.addr();
        e.mem.set_u8(flag, 0);
        let other = x87_store32(e.vcall(this, 0x48c, &args![index, flag]).f64());
        if e.mem.u8(flag) != 0 {
            base = x87_quiet_bits(other);
        }
    });
    let values = this + VALUES;
    let first = e.vcall(values, slots[0], &args![index]).f64();
    let mut total = x87_add(first, x87_m32(base));
    for slot in &slots[1..] {
        let value = e.vcall(values, *slot, &args![index]).f64();
        total = x87_add(value, total);
    }
    let stored = x87_store32(total);
    e.mem.set_u32(this + field, stored);
    f32::from_bits(x87_quiet_bits(stored))
}

// Translated from 008d4cc0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; answers `ST0`): `fArmorRating`
/// (+0x1B8, see `fn_008d4e80`) while it is not below 0.0 (ordered; an unordered value
/// recomputes); otherwise recomputes it for index 0x12: the base value, the overriding
/// value of vtable slot +0x48C when it sets its flag, plus the values of slots +0x18,
/// +0x10 and +0x14 of the object at +0xA4, stores it and answers it.
pub fn fn_008d4cc0(e: &mut Engine, this: u32) -> f32 {
    let stored = e.mem.u32(this + 0x1b8);
    // FCOMP; TEST AH,0x1; JNZ: below or unordered recomputes.
    if x87_load32(stored) >= e.mem.f64(ZERO_DOUBLE) {
        return f32::from_bits(x87_quiet_bits(stored));
    }
    computed_armour_value(e, this, 0x1b8, 0x12, &[0x18, 0x10, 0x14])
}

// Translated from 008d4db0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; answers `ST0`): as `fn_008d4cc0`
/// for `fArmorThreshold` (+0x1BC) with index 0x4C and the slots +0x18 and +0x14.
pub fn fn_008d4db0(e: &mut Engine, this: u32) -> f32 {
    let stored = e.mem.u32(this + 0x1bc);
    // FCOMP; TEST AH,0x1; JNZ: below or unordered recomputes.
    if x87_load32(stored) >= e.mem.f64(ZERO_DOUBLE) {
        return f32::from_bits(x87_quiet_bits(stored));
    }
    computed_armour_value(e, this, 0x1bc, 0x4c, &[0x18, 0x14])
}

/// Shared body of `fn_008d4eb0` and `fn_008d4f10`: 0.0 when `this`'s vtable slot +0x358
/// answers true; otherwise the value of slot +0xC of the object at +0xA4 for index 0x15
/// times the double at `01031148`, times `fn_008d4f70(group)`, rounded to a float.
fn scaled_group_value(e: &mut Engine, this: u32, group: u32) -> f32 {
    let mut result = 0.0f32.to_bits();
    if !e.vcall(this, 0x358, &[]).bool() {
        let value = e.vcall(this + VALUES, 0xc, &args![0x15u32]).f64();
        let scaled = x87_mul(value, e.mem.f64(PERCENT_DOUBLE));
        let factor = fn_008d4f70(e, this, group);
        result = x87_store32(x87_mul(factor, scaled));
    }
    f32::from_bits(result)
}

// Translated from 008d4eb0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; answers `ST0`): 0.0 when vtable
/// slot +0x358 answers true, else the value of slot +0xC of the object at +0xA4 for index
/// 0x15, times the double at `01031148`, times `fn_008d4f70(3)`.
pub fn fn_008d4eb0(e: &mut Engine, this: u32) -> f32 {
    scaled_group_value(e, this, 3)
}

// Translated from 008d4f10 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; answers `ST0`): as `fn_008d4eb0`
/// with `fn_008d4f70(7)`.
pub fn fn_008d4f10(e: &mut Engine, this: u32) -> f32 {
    scaled_group_value(e, this, 7)
}

// Translated from 008d4f70 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word, `RET 4`; answers `ST0`): the float at
/// `0101e340` when vtable slot +0x1E4 answers null; otherwise
/// `Actor::GetAnimGroup(group, <slot +0x148 of the object at +0x68>, 1, 0)` (the two
/// constants pushed first belong to that call, `RET 0x10`), and `00494300` of slot +0x1E4's
/// object with that 16-bit group, sign-extended to an integer (`FILD`).
pub fn fn_008d4f70(e: &mut Engine, this: u32, group: u32) -> f64 {
    if e.vcall(this, 0x1e4, &[]).u32() == 0 {
        return x87_load32(e.mem.u32(DEFAULT_FLOAT));
    }
    let inner = e.mem.u32(this + 0x68);
    let kind = e.vcall(inner, 0x148, &[]).u32();
    let anim_group = e
        .call(GET_ANIM_GROUP, &args![this, group, kind, 1u32, 0u32])
        .u16();
    let animation = e.vcall(this, 0x1e4, &[]).u32();
    let value = e
        .call(
            ANIMATION_GROUP_VALUE,
            &args![animation, u32::from(anim_group)],
        )
        .u16() as i16;
    f64::from(value)
}

// Translated from 008d5020 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a constructor shape): a 12-byte
/// block from `00401000`, built by `008d5590` when the allocation succeeded; the result
/// (0 otherwise) is stored at `this`.
// Translation incomplete: 008d5020 - the FS:[0] exception frame (freeing the block when its constructor throws) is not translated
pub fn fn_008d5020(e: &mut Engine, this: u32) {
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    let held = if block != 0 {
        e.call(HELD_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(this, held);
}

// Translated from 008d50e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): runs `008d55b0` on the object
/// held at `this` when there is one.
pub fn fn_008d50e0(e: &mut Engine, this: u32) {
    let held = e.mem.u32(this);
    if held != 0 {
        e.call(HELD_RUN, &args![held]);
    }
}

// Translated from 008d50a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): hands the block held at `this`
/// to `00401030` and clears it, when there is one.
pub fn fn_008d50a0(e: &mut Engine, this: u32) {
    let held = e.mem.u32(this);
    if held != 0 {
        e.call(FREE, &args![held]);
        e.mem.set_u32(this, 0);
    }
}

// Translated from 008d4ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `008ab3a0` on `this` with `value`, then clears the word at +0x140.
pub fn fn_008d4ff0(e: &mut Engine, this: u32, value: u32) {
    e.call(0x008a_b3a0, &args![this, value]);
    e.mem.set_u32(this + 0x140, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008d_4ff0, fn_008d4ff0(u32, u32)),
        entry!(0x008d4e80, fn_008d4e80(Ptr)),
        entry!(0x008d_4cc0, fn_008d4cc0(u32) -> f32),
        entry!(0x008d_4db0, fn_008d4db0(u32) -> f32),
        entry!(0x008d_4eb0, fn_008d4eb0(u32) -> f32),
        entry!(0x008d_4f10, fn_008d4f10(u32) -> f32),
        entry!(0x008d_4f70, fn_008d4f70(u32, u32) -> f64),
        entry!(0x008d_5020, fn_008d5020(u32)),
        entry!(0x008d_50e0, fn_008d50e0(u32)),
        entry!(0x008d_50a0, fn_008d50a0(u32)),
    ]
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A character whose own vtable is at `0x0090_0000` and whose value object (+0xA4) has
    /// the vtable at `0x0091_0000`, both 0x200 words of the given slots.
    fn character(e: &mut Engine, own: &[(u32, u32)], values: &[(u32, u32)]) -> u32 {
        let this = e.mem.alloc(0x200);
        let own_table = e.mem.alloc(0x800);
        let values_table = e.mem.alloc(0x100);
        for (slot, function) in own {
            e.mem.set_u32(own_table + slot, *function);
        }
        for (slot, function) in values {
            e.mem.set_u32(values_table + slot, *function);
        }
        e.mem.set_u32(this, own_table);
        e.mem.set_u32(this + VALUES, values_table);
        this
    }

    #[test]
    fn fn_008d4cc0_keeps_or_recomputes_the_rating() {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.register(BASE_VALUE, |_, _| st0(10.0));
        // Slot +0x48C sets the flag only for index 0x12 when the word at 0x0099_0000 is 1.
        e.register(0x0090_1000, |e, a| {
            e.mem.set_u8(a[2], e.mem.u8(a[0] + 0x1f0));
            st0(4.0)
        });
        e.register(0x0091_0018, |_, _| st0(1.0));
        e.register(0x0091_0010, |_, _| st0(2.0));
        e.register(0x0091_0014, |_, _| st0(0.5));
        let this = character(
            &mut e,
            &[(0x48c, 0x0090_1000)],
            &[
                (0x18, 0x0091_0018),
                (0x10, 0x0091_0010),
                (0x14, 0x0091_0014),
            ],
        );
        // Not below 0.0: kept.
        e.mem.set_f32(this + 0x1b8, 7.0);
        assert_eq!(e.call(0x008d_4cc0, &[this]).f64(), 7.0);
        // Below: base 10 + 1 + 2 + 0.5.
        e.mem.set_f32(this + 0x1b8, -1.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008d_4cc0, &[this]).f64(), 13.5);
        assert_eq!(e.mem.f32(this + 0x1b8), 13.5);
        assert_eq!(calls_to(&e, 0x0091_0018)[0], vec![this + VALUES, 0x12]);
        // A NaN recomputes too; the flag replaces the base with 4.0.
        e.mem.set_u32(this + 0x1b8, 0x7fc0_0000);
        e.mem.set_u8(this + 0x1f0, 1);
        assert_eq!(e.call(0x008d_4cc0, &[this]).f64(), 7.5);
    }

    #[test]
    fn fn_008d4db0_keeps_or_recomputes_the_threshold() {
        let mut e = Engine::new();
        e.map(ZERO_DOUBLE & !0xfff, 0x1000);
        e.register(BASE_VALUE, |_, _| st0(3.0));
        e.register(0x0090_1000, |_, _| st0(99.0));
        e.register(0x0091_0018, |_, _| st0(1.0));
        e.register(0x0091_0014, |_, _| st0(0.25));
        let this = character(
            &mut e,
            &[(0x48c, 0x0090_1000)],
            &[(0x18, 0x0091_0018), (0x14, 0x0091_0014)],
        );
        e.mem.set_f32(this + 0x1bc, 0.0);
        assert_eq!(e.call(0x008d_4db0, &[this]).f64(), 0.0);
        e.mem.set_f32(this + 0x1bc, -1.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008d_4db0, &[this]).f64(), 4.25);
        assert_eq!(e.mem.f32(this + 0x1bc), 4.25);
        assert_eq!(calls_to(&e, 0x0090_1000)[0][1], 0x4c);
    }

    #[test]
    fn fn_008d4f70_answers_the_default_or_the_group_value() {
        let mut e = Engine::new();
        e.map(DEFAULT_FLOAT & !0xfff, 0x1000);
        e.mem.set_f32(DEFAULT_FLOAT, 32.0);
        e.register(0x0090_1000, |e, a| e.mem.u32(a[0] + 0x1f0).into_ret());
        e.register(0x0092_0000, |_, _| 0x55u32.into_ret());
        e.register(GET_ANIM_GROUP, |_, _| 0x1_0042u32.into_ret());
        e.register(ANIMATION_GROUP_VALUE, |_, _| 0xffffu32.into_ret());
        let this = character(&mut e, &[(0x1e4, 0x0090_1000)], &[]);
        assert_eq!(e.call(0x008d_4f70, &[this, 3]).f64(), 32.0);
        let inner = e.mem.alloc(8);
        let inner_table = e.mem.alloc(0x200);
        e.mem.set_u32(inner_table + 0x148, 0x0092_0000);
        e.mem.set_u32(inner, inner_table);
        e.mem.set_u32(this + 0x68, inner);
        e.mem.set_u32(this + 0x1f0, 0x7777);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008d_4f70, &[this, 3]).f64(), -1.0);
        assert_eq!(
            calls_to(&e, GET_ANIM_GROUP),
            vec![vec![this, 3, 0x55, 1, 0]]
        );
        assert_eq!(
            calls_to(&e, ANIMATION_GROUP_VALUE),
            vec![vec![0x7777, 0x42]]
        );
    }

    #[test]
    fn fn_008d4eb0_and_fn_008d4f10_scale_by_the_group_value() {
        let mut e = Engine::new();
        e.map(PERCENT_DOUBLE & !0xfff, 0x1000);
        e.mem.set_f64(PERCENT_DOUBLE, 0.5);
        e.register(0x0090_1000, |e, a| e.mem.u32(a[0] + 0x1f0).into_ret());
        e.register(0x0090_2000, |e, a| e.mem.u32(a[0] + 0x1f4).into_ret());
        e.register(0x0091_000c, |_, _| st0(6.0));
        e.register(0x0092_0000, |_, _| 0u32.into_ret());
        e.register(GET_ANIM_GROUP, |_, a| a[1].into_ret());
        e.register(ANIMATION_GROUP_VALUE, |_, a| a[1].into_ret());
        let this = character(
            &mut e,
            &[(0x358, 0x0090_2000), (0x1e4, 0x0090_1000)],
            &[(0xc, 0x0091_000c)],
        );
        let inner = e.mem.alloc(8);
        let inner_table = e.mem.alloc(0x200);
        e.mem.set_u32(inner_table + 0x148, 0x0092_0000);
        e.mem.set_u32(inner, inner_table);
        e.mem.set_u32(this + 0x68, inner);
        e.mem.set_u32(this + 0x1f0, 0x7777);
        e.call_log = Some(vec![]);
        // 6.0 * 0.5 * 3 and 6.0 * 0.5 * 7.
        assert_eq!(e.call(0x008d_4eb0, &[this]).f64(), 9.0);
        assert_eq!(e.call(0x008d_4f10, &[this]).f64(), 21.0);
        assert_eq!(calls_to(&e, 0x0091_000c)[0], vec![this + VALUES, 0x15]);
        // Slot +0x358 true: 0.0 without asking for the value.
        e.mem.set_u32(this + 0x1f4, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008d_4eb0, &[this]).f64(), 0.0);
        assert!(calls_to(&e, 0x0091_000c).is_empty());
    }

    #[test]
    fn fn_008d5020_fn_008d50e0_fn_008d50a0_hold_one_block() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, _| 0x6000u32.into_ret());
        e.register(HELD_CONSTRUCT, |_, a| (a[0] + 1).into_ret());
        e.register(HELD_RUN, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let this = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x008d_5020, &[this]);
        assert_eq!(e.mem.u32(this), 0x6001);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0xc]]);
        assert_eq!(calls_to(&e, HELD_CONSTRUCT), vec![vec![0x6000]]);
        e.call(0x008d_50e0, &[this]);
        assert_eq!(calls_to(&e, HELD_RUN), vec![vec![0x6001]]);
        e.call(0x008d_50a0, &[this]);
        assert_eq!(calls_to(&e, FREE), vec![vec![0x6001]]);
        assert_eq!(e.mem.u32(this), 0);
        // Nothing held: neither runs anything.
        e.call_log = Some(vec![]);
        e.call(0x008d_50e0, &[this]);
        e.call(0x008d_50a0, &[this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        // A failed allocation stores 0 without constructing.
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008d_5020, &[this]);
        assert_eq!(e.mem.u32(this), 0);
        assert!(calls_to(&e, HELD_CONSTRUCT).is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_armour_values_get_the_constant() {
        let mut e = Engine::new();
        e.map(MINUS_ONE & !0xfff, 0x1000);
        e.set_global(MINUS_ONE, -1.0f32);
        let object = e.mem.alloc(0x1d0);
        e.mem.set_f32(object + 0x1b8, 5.0);
        e.mem.set_f32(object + 0x1c0, 6.0);
        e.call(0x008d_4e80, &args![object]);
        assert_eq!(e.mem.f32(object + 0x1b8), -1.0);
        assert_eq!(e.mem.f32(object + 0x1bc), -1.0);
        // Neighbours are untouched.
        assert_eq!(e.mem.f32(object + 0x1c0), 6.0);
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_008d4ff0_forwards_then_clears_0x140() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x150);
        e.mem.set_u32(this + 0x140, 9);
        e.register(0x008a_b3a0, |e, a| {
            e.mem.set_u32(a[0] + 0x144, a[1]);
            Ret::default()
        });
        e.call(0x008d_4ff0, &[this, 7]);
        assert_eq!(e.mem.u32(this + 0x144), 7);
        assert_eq!(e.mem.u32(this + 0x140), 0);
    }
}
