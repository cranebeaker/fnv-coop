//! `fallout/ai/run_0097ee60` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0097f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x70 of `this` (no name in the engine map; callers use the
/// body on more than one class, so the field is not named here).
pub fn fn_0097f2f0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x70))
}

/// The `double` 0.0 and the `float` -1.0 the code reads (`.rdata`).
const DOUBLE_ZERO: u32 = 0x0101_2060;
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
/// `00403e20` (`thiscall` on a `Setting`): the address of its value; the
/// settings `011ce9b4` and `011ce528` are compared with a timer here.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00435e00` (`thiscall`, a float in ST0) on the timer at +0x94; `00435dd0`
/// (no stack words, a float in ST0) and `00435de0` (`thiscall(float)`) build
/// the word stored at +0xE0.
const TIMER_VALUE: u32 = 0x0043_5e00;
const CLOCK_VALUE: u32 = 0x0043_5dd0;
/// `008d8520` (`thiscall` on a reference): its process (the engine map's name
/// `MiddleHighProcess::GetSavedAcquireObject` is folded code).
const REFERENCE_PROCESS: u32 = 0x008d_8520;

// Translated from 0097ee60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four words at `source` to `this` (`RET 4`); `EAX` is left
/// holding the last word copied, which is what the caller gets.
pub fn fn_0097ee60(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    for i in 0..4 {
        let word = e.mem.u32(source.addr() + 4 * i);
        e.mem.set_u32(this.addr() + 4 * i, word);
    }
    e.mem.u32(source.addr() + 0xc)
}

/// The timer at +0x94 (`00435e00`) against the value of `setting`.
fn timer_and_setting(e: &mut Engine, this: Ptr, setting: u32) -> (f64, f64) {
    let timer = e.call(TIMER_VALUE, &args![this.addr() + 0x94]).f64();
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    (x87_load32(e.mem.u32(value)), timer)
}

// Translated from 0097ee90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the `011ce9b4` setting is above the timer at +0x94 (`FCOMP;
/// TEST AH,0x41; JNZ`: ordered and greater).
pub fn fn_0097ee90(e: &mut Engine, this: Ptr) -> bool {
    let (setting, timer) = timer_and_setting(e, this, 0x011c_e9b4);
    setting > timer
}

// Translated from 0097eee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the `011ce528` setting is at or below the timer at +0x94
/// (`FCOMP; TEST AH,0x41; JP`: ordered and not greater).
pub fn fn_0097eee0(e: &mut Engine, this: Ptr) -> bool {
    let (setting, timer) = timer_and_setting(e, this, 0x011c_e528);
    setting <= timer
}

// Translated from 0097ef30 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 when the word at +0xD0 is not zero, else 0 (`CMP 0, word; SBB; NEG`).
pub fn fn_0097ef30(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u32(this.addr() + 0xd0) != 0)
}

// Translated from 0097ef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// With a positive float at +0x148: when the actor at +0xBC is pathing
/// (`Actor::IsPathing`, `008b3bd0`) it is given that value (`008b3230`, then
/// slot 0x348 with the value and 0); the float is cleared either way.
pub fn fn_0097ef50(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    // `FCOMP 0.0; TEST AH,0x41; JNZ`: only above zero (ordered).
    if !(x87_load32(e.mem.u32(at + 0x148)) > e.mem.f64(DOUBLE_ZERO)) {
        return;
    }
    let actor = e.mem.u32(at + 0xbc);
    if e.call(0x008b_3bd0, &args![actor]).bool() {
        let value = x87_quiet_bits(e.mem.u32(at + 0x148));
        let actor = e.mem.u32(at + 0xbc);
        e.call(0x008b_3230, &args![actor, value]);
        let value = x87_quiet_bits(e.mem.u32(at + 0x148));
        let actor = e.mem.u32(at + 0xbc);
        e.vcall(actor, 0x348, &args![value, 0u32]);
    }
    e.mem.set_u32(at + 0x148, 0.0f32.to_bits());
}

// Translated from 0097efe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes the controller's target to `target` (`state` 0 only: the byte at
/// +0xE4 cleared, `fn_0097f200`, the process (`008d8520` of the actor at
/// +0xBC) told through slot 0x450 whether `00662a80` of the object at +0x9C
/// failed, `fn_0097f240` with `00435dd0` when it held; with 3D, the weapon
/// `0099a2d0` gives is equipped (`Actor::EquipObject`, the count from
/// `Actor::GetItemCountinContainer` when `004c0bf0` holds) unless the process'
/// current item (slot 0x148, `0044ddc0`) is it, or without one the current
/// item is unequipped (`Actor::UnEquipObject`); without 3D the process' slot
/// 0x458 runs), then `00997ba0(target, state)` on the object at +0x9C, and
/// without both words the three resets (`00980550`, `00980250`, `00980360`)
/// and -1.0 at +0x148.
pub fn fn_0097efe0(e: &mut Engine, this: Ptr, target: u32, state: u32) {
    let at = this.addr();
    if state == 0 {
        e.mem.set_u8(at + 0xe4, 0);
        fn_0097f200(e, this);
        let object = e.mem.u32(at + 0x9c);
        let held = e.call(0x0066_2a80, &args![object]).u8() != 0;
        let actor = e.mem.u32(at + 0xbc);
        let process = e.call(REFERENCE_PROCESS, &args![actor]).u32();
        e.vcall(process, 0x450, &args![u32::from(!held)]);
        if held {
            let clock = e.call(CLOCK_VALUE, &[]).f64();
            let object = Ptr::new(e.mem.u32(at + 0x9c));
            fn_0097f240(e, object, f32::from_bits(x87_store32(clock)));
        }
        let actor = e.mem.u32(at + 0xbc);
        if e.vcall(actor, 0x1d0, &[]).u32() != 0 {
            let object = e.mem.u32(at + 0x9c);
            let weapon = e.call(0x0099_a2d0, &args![object]).u32();
            let actor = e.mem.u32(at + 0xbc);
            let process = e.call(REFERENCE_PROCESS, &args![actor]).u32();
            let entry = e.vcall(process, 0x148, &[]).u32();
            let current = if entry == 0 {
                0
            } else {
                e.call(0x0044_ddc0, &args![entry]).u32()
            };
            if weapon != 0 && weapon != current {
                let count = if e.call(0x004c_0bf0, &args![weapon]).bool() {
                    let actor = e.mem.u32(at + 0xbc);
                    e.call(0x0057_5610, &args![actor, weapon]).u32()
                } else {
                    1
                };
                let actor = e.mem.u32(at + 0xbc);
                e.call(
                    0x0088_c830,
                    &args![actor, weapon, count, 0u32, 1u32, 0u32, 1u32],
                );
            } else if weapon == 0 && current != 0 {
                let extra = e.call(0x0072_6070, &args![entry]).u32();
                let actor = e.mem.u32(at + 0xbc);
                e.call(
                    0x0088_d7d0,
                    &args![actor, current, extra, 0u32, 0u32, 0u32, 1u32],
                );
            }
        } else {
            let actor = e.mem.u32(at + 0xbc);
            let process = e.call(REFERENCE_PROCESS, &args![actor]).u32();
            let actor = e.mem.u32(at + 0xbc);
            e.vcall(process, 0x458, &args![actor, u32::from(!held)]);
        }
    }
    let object = e.mem.u32(at + 0x9c);
    e.call(0x0099_7ba0, &args![object, target, state]);
    if target == 0 || state == 0 {
        e.call(0x0098_0550, &args![this]);
        e.call(0x0098_0250, &args![this]);
        e.call(0x0098_0360, &args![this]);
        let minus_one = x87_quiet_bits(e.mem.u32(FLOAT_MINUS_ONE));
        e.mem.set_u32(at + 0x148, minus_one);
    }
}

// Translated from 0097f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_0097f220` on the member at +0xB4.
pub fn fn_0097f200(e: &mut Engine, this: Ptr) {
    fn_0097f220(e, this.byte_add(0xb4));
}

// Translated from 0097f220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores -1.0 (the float at `01012054`) at +4.
pub fn fn_0097f220(e: &mut Engine, this: Ptr) {
    let minus_one = x87_quiet_bits(e.mem.u32(FLOAT_MINUS_ONE));
    e.mem.set_u32(this.addr() + 4, minus_one);
}

// Translated from 0097f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores at +0xE0 the word `00435de0` builds from `value` (`RET 4`).
pub fn fn_0097f240(e: &mut Engine, this: Ptr, value: f32) {
    let word = e.with_stack(4, |e, local| {
        e.call(0x0043_5de0, &args![local, x87_quiet_bits(value.to_bits())]);
        e.mem.u32(local.addr())
    });
    e.mem.set_u32(this.addr() + 0xe0, word);
}

// Translated from 0097f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Remembers a procedure (`RET 4`): the 0x30 bytes at +0x158 copied to
/// +0x14C (`00401460`, cdecl), then `form`'s slot 0x34 at +0x17C, its saved
/// id (`0084e3a0`) at +0x180 and `00435dd0`'s float at +0x184.
pub fn fn_0097f270(e: &mut Engine, this: Ptr, form: u32) {
    let at = this.addr();
    e.call(0x0040_1460, &args![at + 0x14c, at + 0x158, 0x30u32]);
    let value = e.vcall(form, 0x34, &[]).u32();
    e.mem.set_u32(at + 0x17c, value);
    let id = e.call(0x0084_e3a0, &args![form]).u32();
    e.mem.set_u32(at + 0x180, id);
    let clock = e.call(CLOCK_VALUE, &[]).f64();
    e.mem.set_u32(at + 0x184, x87_store32(clock));
}

// Translated from 0097f2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_0097f2f0` (the byte at +0x70) of the object at +0x9C.
pub fn fn_0097f2d0(e: &mut Engine, this: Ptr) -> u8 {
    let object = Ptr::new(e.mem.u32(this.addr() + 0x9c));
    fn_0097f2f0(e, object)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_f2f0, fn_0097f2f0(Ptr) -> u8),
        entry!(0x0097_ee60, fn_0097ee60(Ptr, Ptr) -> u32),
        entry!(0x0097_ee90, fn_0097ee90(Ptr) -> bool),
        entry!(0x0097_eee0, fn_0097eee0(Ptr) -> bool),
        entry!(0x0097_ef30, fn_0097ef30(Ptr) -> u32),
        entry!(0x0097_ef50, fn_0097ef50(Ptr)),
        entry!(0x0097_efe0, fn_0097efe0(Ptr, u32, u32)),
        entry!(0x0097_f200, fn_0097f200(Ptr)),
        entry!(0x0097_f220, fn_0097f220(Ptr)),
        entry!(0x0097_f240, fn_0097f240(Ptr, f32)),
        entry!(0x0097_f270, fn_0097f270(Ptr, u32)),
        entry!(0x0097_f2d0, fn_0097f2d0(Ptr) -> u8),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0097f2f0_reads_the_byte_at_0x70() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x70, 0xaabb_cc01);
        assert_eq!(e.call(0x0097_f2f0, &args![this]).u8(), 1);
        e.mem.set_u8(this + 0x70, 0);
        assert_eq!(e.call(0x0097_f2f0, &args![this]).u8(), 0);
    }
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_f(st0: f64) -> Ret {
        Ret {
            st0,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn stub(e: &mut Engine, callee: u32, eax: u32) {
        e.register_double(callee, move |_, _| ret(eax));
    }

    fn stub_all(e: &mut Engine, callees: &[u32]) {
        for &callee in callees {
            e.register(callee, |_, _| Ret::default());
        }
    }

    fn object(e: &mut Engine, size: u32, base: u32) -> u32 {
        let this = e.mem.alloc(size);
        let table = e.mem.alloc(0x800);
        for slot in (0..0x800).step_by(4) {
            e.mem.set_u32(table + slot, base + slot);
        }
        e.mem.set_u32(this, table);
        this
    }

    fn map_data(e: &mut Engine) {
        e.map(DOUBLE_ZERO, 8);
        e.map(FLOAT_MINUS_ONE, 4);
        e.mem.set_f32(FLOAT_MINUS_ONE, -1.0);
    }

    #[test]
    fn fn_0097ee60_copies_four_words_and_leaves_the_last() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let source = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(source + 4 * i, 0x10 + i);
        }
        assert_eq!(e.call(0x0097_ee60, &[this, source]).u32(), 0x13);
        assert_eq!(e.mem.bytes(this, 0x10), e.mem.bytes(source, 0x10));
    }

    #[test]
    fn the_timer_tests_compare_with_their_settings() {
        let mut e = Engine::new();
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 2.0);
        stub(&mut e, SETTING_VALUE, value);
        e.register(TIMER_VALUE, |_, _| ret_f(1.0));
        assert!(e.call(0x0097_ee90, &[0x9000]).bool());
        assert!(!e.call(0x0097_eee0, &[0x9000]).bool());
        e.register(TIMER_VALUE, |_, _| ret_f(2.0));
        assert!(!e.call(0x0097_ee90, &[0x9000]).bool());
        assert!(e.call(0x0097_eee0, &[0x9000]).bool());
        e.register(TIMER_VALUE, |_, _| ret_f(f64::NAN));
        assert!(!e.call(0x0097_ee90, &[0x9000]).bool());
        assert!(!e.call(0x0097_eee0, &[0x9000]).bool());
        e.call_log = Some(vec![]);
        e.call(0x0097_ee90, &[0x9000]);
        assert_eq!(calls_to(&e, TIMER_VALUE), vec![vec![0x9094]]);
        assert_eq!(calls_to(&e, SETTING_VALUE), vec![vec![0x011c_e9b4]]);
    }

    #[test]
    fn fn_0097ef30_tests_the_word_at_0xd0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd4);
        assert_eq!(e.call(0x0097_ef30, &[this]).u32(), 0);
        e.mem.set_u32(this + 0xd0, 0x8000_0000);
        assert_eq!(e.call(0x0097_ef30, &[this]).u32(), 1);
    }

    #[test]
    fn fn_0097ef50_hands_a_positive_value_to_a_pathing_actor() {
        let mut e = Engine::new();
        map_data(&mut e);
        let this = e.mem.alloc(0x150);
        let actor = object(&mut e, 0x10, 0x7200_0000);
        e.mem.set_u32(this + 0xbc, actor);
        e.mem.set_f32(this + 0x148, 2.5);
        stub(&mut e, 0x008b_3bd0, 1);
        stub_all(&mut e, &[0x008b_3230, 0x7200_0348]);
        e.call_log = Some(vec![]);
        e.call(0x0097_ef50, &[this]);
        assert_eq!(
            calls_to(&e, 0x008b_3230),
            vec![vec![actor, 2.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, 0x7200_0348),
            vec![vec![actor, 2.5f32.to_bits(), 0]]
        );
        assert_eq!(e.mem.f32(this + 0x148), 0.0);
        e.mem.set_f32(this + 0x148, -1.0);
        e.call_log = Some(vec![]);
        e.call(0x0097_ef50, &[this]);
        assert_eq!(e.mem.f32(this + 0x148), -1.0);
        assert!(calls_to(&e, 0x008b_3bd0).is_empty());
    }

    /// The controller with an actor (+0xBC), its process and the object at
    /// +0x9C.
    fn controller(e: &mut Engine) -> (u32, u32, u32) {
        map_data(e);
        let this = e.mem.alloc(0x190);
        let actor = object(e, 0x10, 0x7200_0000);
        let process = object(e, 0x10, 0x7300_0000);
        e.mem.set_u32(this + 0xbc, actor);
        e.mem.set_u32(this + 0x9c, 0x0b0b);
        stub(e, REFERENCE_PROCESS, process);
        stub_all(
            e,
            &[
                0x7300_0450,
                0x7300_0458,
                0x0099_7ba0,
                0x0098_0550,
                0x0098_0250,
                0x0098_0360,
                0x0043_5de0,
                0x0088_c830,
                0x0088_d7d0,
            ],
        );
        e.register(CLOCK_VALUE, |_, _| ret_f(7.0));
        (this, actor, process)
    }

    #[test]
    fn fn_0097efe0_equips_the_weapon_for_a_new_target() {
        let mut e = Engine::new();
        let (this, actor, process) = controller(&mut e);
        e.mem.set_u8(this + 0xe4, 1);
        let field = e.mem.alloc(4);
        e.register_double(0x0066_2a80, move |e, _| ret(e.mem.u32(field)));
        stub(&mut e, 0x7200_01d0, 0x3d00);
        stub(&mut e, 0x0099_a2d0, 0x3e00);
        stub(&mut e, 0x7300_0148, 0);
        stub(&mut e, 0x004c_0bf0, 1);
        stub(&mut e, 0x0057_5610, 4);
        e.call_log = Some(vec![]);
        e.call(0x0097_efe0, &[this, 0x7a00, 0]);
        assert_eq!(e.mem.u8(this + 0xe4), 0);
        assert_eq!(e.mem.f32(this + 0xb4 + 4), -1.0);
        assert_eq!(calls_to(&e, 0x7300_0450), vec![vec![process, 1]]);
        assert_eq!(
            calls_to(&e, 0x0088_c830),
            vec![vec![actor, 0x3e00, 4, 0, 1, 0, 1]]
        );
        assert_eq!(calls_to(&e, 0x0099_7ba0), vec![vec![0x0b0b, 0x7a00, 0]]);
        assert_eq!(calls_to(&e, 0x0098_0360), vec![vec![this]]);
        assert_eq!(e.mem.f32(this + 0x148), -1.0);
    }

    #[test]
    fn fn_0097efe0_unequips_or_tells_the_process() {
        let mut e = Engine::new();
        let (this, actor, process) = controller(&mut e);
        let object_9c = e.mem.alloc(0xe4);
        e.mem.set_u32(this + 0x9c, object_9c);
        stub(&mut e, 0x0066_2a80, 1);
        stub(&mut e, 0x7200_01d0, 0x3d00);
        stub(&mut e, 0x0099_a2d0, 0);
        stub(&mut e, 0x7300_0148, 0x0e0e);
        stub(&mut e, 0x0044_ddc0, 0x3f00);
        stub(&mut e, 0x0072_6070, 0x0f0f);
        e.register(0x0043_5de0, |e, a| {
            e.mem.set_u32(a[0], a[1] ^ 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0097_efe0, &[this, 0x7a00, 1]);
        // `state` 1: only the object at +0x9C hears of it.
        assert!(calls_to(&e, 0x7300_0450).is_empty());
        assert_eq!(calls_to(&e, 0x0099_7ba0), vec![vec![object_9c, 0x7a00, 1]]);
        assert!(calls_to(&e, 0x0098_0550).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0097_efe0, &[this, 0x7a00, 0]);
        assert_eq!(calls_to(&e, 0x7300_0450), vec![vec![process, 0]]);
        assert_eq!(e.mem.u32(object_9c + 0xe0), 7.0f32.to_bits() ^ 1);
        assert_eq!(
            calls_to(&e, 0x0088_d7d0),
            vec![vec![actor, 0x3f00, 0x0f0f, 0, 0, 0, 1]]
        );
        stub(&mut e, 0x7200_01d0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0097_efe0, &[this, 0x7a00, 0]);
        assert_eq!(calls_to(&e, 0x7300_0458), vec![vec![process, actor, 0]]);
    }

    #[test]
    fn fn_0097f270_remembers_the_procedure() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x188);
        let form = object(&mut e, 0x10, 0x7400_0000);
        stub_all(&mut e, &[0x0040_1460]);
        stub(&mut e, 0x7400_0034, 0x3434);
        stub(&mut e, 0x0084_e3a0, 0x0001_2345);
        e.register(CLOCK_VALUE, |_, _| ret_f(3.25));
        e.call_log = Some(vec![]);
        e.call(0x0097_f270, &[this, form]);
        assert_eq!(
            calls_to(&e, 0x0040_1460),
            vec![vec![this + 0x14c, this + 0x158, 0x30]]
        );
        assert_eq!(e.mem.u32(this + 0x17c), 0x3434);
        assert_eq!(e.mem.u32(this + 0x180), 0x0001_2345);
        assert_eq!(e.mem.f32(this + 0x184), 3.25);
    }

    #[test]
    fn fn_0097f2d0_reads_the_byte_of_the_object_at_0x9c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa0);
        let object_9c = e.mem.alloc(0x74);
        e.mem.set_u8(object_9c + 0x70, 3);
        e.mem.set_u32(this + 0x9c, object_9c);
        assert_eq!(e.call(0x0097_f2d0, &[this]).u8(), 3);
    }
}
