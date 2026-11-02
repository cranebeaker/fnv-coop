//! `BSShader/run_00b664f0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShaderAccumulator`'s destructor body `00b656e0` (thiscall;
/// `bsshaderaccumulator.cpp`).
const ACCUMULATOR_DESTRUCTOR: u32 = 0x00b6_56e0;
/// The sized deallocation `00aa1460` (cdecl: block, size).
const DEALLOCATE_SIZED: u32 = 0x00aa_1460;
/// Called with the render mode first (`00b4f450`, `bsshadermanager.cpp`;
/// cdecl, one word).
const RENDER_MODE_PREPARE: u32 = 0x00b4_f450;
/// The accumulator's default-mode pass `00b65ae0` (cdecl: the accumulator).
const ACCUMULATOR_PASS_00B65AE0: u32 = 0x00b6_5ae0;
/// The accumulator's default-mode finish `00b65c60` (cdecl: the
/// accumulator).
const ACCUMULATOR_PASS_00B65C60: u32 = 0x00b6_5c60;
/// Table of per-render-mode functions (cdecl, one word: the accumulator);
/// entry 0 is the fallback for an empty entry.
const RENDER_MODE_FUNCTIONS: u32 = 0x011f_9f40;
/// The object whose word +0x0C `00b66570` clears.
const OBJECT_011F4748: u32 = 0x011f_4748;
/// `NiAccumulator::FinishAccumulating` (engine map; thiscall).
const NI_ACCUMULATOR_FINISH_ACCUMULATING: u32 = 0x00aa_0fe0;
/// The helpers `00b665d0` and `00b66600` call first (thiscall; two words
/// and one word, `RET 8` / `RET 4`).
const HELPER_00B702B0: u32 = 0x00b7_02b0;
const HELPER_00B70290: u32 = 0x00b7_0290;
/// The double `0.0` `00b66700` compares with (`FCOM qword`).
const ZERO_01012060: u32 = 0x0101_2060;

// Translated from 00b664f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `BSShaderAccumulator` (slot 0 of its
/// vtable `010adff8`; unnamed in the engine map): runs the destructor
/// `00b656e0` and, when bit 0 of `flags` is set, frees the 0x280-byte
/// object (`00aa1460`). Returns `this`.
pub fn fn_00b664f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(ACCUMULATOR_DESTRUCTOR, &[this.addr()]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE_SIZED, &[this.addr(), 0x280]);
    }
    this
}

// Translated from 00b66520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0xA4 of `BSShaderAccumulator`'s vtable (`010adff8`; unnamed in the
/// engine map): passes the render mode (+0x19C) to `00b4f450`, then reads
/// it again: mode 0 runs `00b65ae0(this)`; any other mode runs the entry of
/// the table at `011f9f40` for it, or the table's entry 0 when that entry
/// is null.
pub fn fn_00b66520(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mode = e.mem.u32(base + 0x19c);
    e.call(RENDER_MODE_PREPARE, &[mode]);
    let mode = e.mem.u32(base + 0x19c);
    if mode == 0 {
        e.call(ACCUMULATOR_PASS_00B65AE0, &[base]);
        return;
    }
    let function = e
        .mem
        .u32(RENDER_MODE_FUNCTIONS.wrapping_add(mode.wrapping_mul(4)));
    if function != 0 {
        e.call(function, &[base]);
    } else {
        let fallback = e.mem.u32(RENDER_MODE_FUNCTIONS);
        e.call(fallback, &[base]);
    }
}

// Translated from 00b66570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot +0xA8 of `BSShaderAccumulator`'s vtable (`010adff8`; unnamed in the
/// engine map): in render mode 0 (+0x19C) runs `00b65c60(this)`; then
/// clears word +0x0C of the object `011f4748` points to and jumps to
/// `NiAccumulator::FinishAccumulating` (`00aa0fe0`).
pub fn fn_00b66570(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x19c) == 0 {
        e.call(ACCUMULATOR_PASS_00B65C60, &[base]);
    }
    let object = e.mem.u32(OBJECT_011F4748);
    e.mem.set_u32(object.wrapping_add(0xc), 0);
    e.call(NI_ACCUMULATOR_FINISH_ACCUMULATING, &[base]);
}

// Translated from 00b665a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): runs the accumulator's
/// default-mode pass `00b65ae0` and then `00b65c60`, both on `accumulator`.
pub fn fn_00b665a0(e: &mut Engine, accumulator: u32) {
    e.call(ACCUMULATOR_PASS_00B65AE0, &[accumulator]);
    e.call(ACCUMULATOR_PASS_00B65C60, &[accumulator]);
}

// Translated from 00b665d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, two words): runs `00b702b0(this,
/// first, second)`, then `this`'s virtual slot +0xD0 with `first`.
pub fn fn_00b665d0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(HELPER_00B702B0, &[this.addr(), first, second]);
    e.vcall(this.addr(), 0xd0, &[first]);
}

// Translated from 00b66600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, a slot of seven vtables (`010ae19c`,
/// `010b83fc`, ...; Ghidra made no function here, read from the bytes;
/// thiscall, one word): runs `00b70290(this, value)`, then `this`'s virtual
/// slot +0xD0 with `value`.
pub fn fn_00b66600(e: &mut Engine, this: Ptr, value: u32) {
    e.call(HELPER_00B70290, &[this.addr(), value]);
    e.vcall(this.addr(), 0xd0, &[value]);
}

// Translated from 00b66640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, `RET 0x28`: ten words, the tenth
/// never read): when the 16-bit word at +0xA8 is 9, stores the nine byte
/// arguments into the nine bytes the pointer at +0xC4 points to (the
/// pointer is read again for each byte).
pub fn fn_00b66640(
    e: &mut Engine,
    this: Ptr,
    byte0: u8,
    byte1: u8,
    byte2: u8,
    byte3: u8,
    byte4: u8,
    byte5: u8,
    byte6: u8,
    byte7: u8,
    byte8: u8,
    _unused_9: u32,
) {
    let base = this.addr();
    if e.mem.u16(base + 0xa8) != 9 {
        return;
    }
    let bytes = [
        byte0, byte1, byte2, byte3, byte4, byte5, byte6, byte7, byte8,
    ];
    for (offset, value) in (0u32..).zip(bytes) {
        let target = e.mem.u32(base + 0xc4);
        e.mem.set_u8(target.wrapping_add(offset), value);
    }
}

// Translated from 00b66700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (thiscall, one word): stores `value` clamped
/// to [0, 1] in the float at +0xE0: below the double at `01012060` (0.0)
/// gives 0, not below 1 (or NaN) gives 1, anything else the value.
pub fn fn_00b66700(e: &mut Engine, this: Ptr, value: f32) {
    let value = x87_load32(value.to_bits());
    let zero = e.mem.f64(ZERO_01012060);
    let stored = if value < zero {
        x87_store32(0.0)
    } else if 1.0 > value {
        x87_store32(value)
    } else {
        x87_store32(1.0)
    };
    e.mem.set_u32(this.addr() + 0xe0, stored);
}

/// `00b70270` (unit `run_00b70260`; `thiscall`, one word: the other object; `bool` in `AL`).
const IS_EQUAL_00B70270: u32 = 0x00b7_0270;

// Translated from 00b665c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x5c of `BSShaderPPLightingProperty`'s vtable `010ae0d0`
/// (and two others; `IsEqual` in the Xbox PDB's slot list; `thiscall`, one word: the other
/// object). Runs `00b70270` with `this` and the word, then answers false whatever it gave
/// (`XOR AL,AL`).
pub fn fn_00b665c0(e: &mut Engine, this: Ptr, other: u32) -> bool {
    e.call(IS_EQUAL_00B70270, &args![this, other]);
    false
}

// Translated from 00b66620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: slot +0xdc of `BSShaderPPLightingProperty`'s vtable
/// `010ae0d0` and six others (`thiscall`, one word). Stores 0 in the word at `+0xd8` when the
/// word is 3, otherwise 3 (`SUB 3; NEG; SBB; AND 3`).
pub fn fn_00b66620(e: &mut Engine, this: Ptr, mode: u32) {
    let value: u32 = if mode.wrapping_sub(3) != 0 { 3 } else { 0 };
    e.mem.set_u32(this.addr().wrapping_add(0xd8), value);
}

// Translated from 00b666d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0xf8 of `BSShaderPPLightingProperty`'s vtable `010ae0d0`
/// and six others (`thiscall`, one word: an object). For the six words at `+0xac` in order
/// calls the object's slot +0x90 with the index (0 to 5) and the word.
pub fn fn_00b666d0(e: &mut Engine, this: Ptr, target: u32) {
    let mut index = 0u32;
    let mut slot = this.addr().wrapping_add(0xac);
    loop {
        let value = e.mem.u32(slot);
        e.vcall(target, 0x90, &args![index, value]);
        index += 1;
        slot = slot.wrapping_add(4);
        if index >= 6 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b6_65c0, fn_00b665c0(Ptr, u32) -> bool),
        entry!(0x00b6_6620, fn_00b66620(Ptr, u32)),
        entry!(0x00b6_66d0, fn_00b666d0(Ptr, u32)),
        entry!(0x00b6_64f0, fn_00b664f0(Ptr, u8) -> Ptr),
        entry!(0x00b6_6520, fn_00b66520(Ptr)),
        entry!(0x00b6_6570, fn_00b66570(Ptr)),
        entry!(0x00b6_65a0, fn_00b665a0(u32)),
        entry!(0x00b6_65d0, fn_00b665d0(Ptr, u32, u32)),
        entry!(0x00b6_6600, fn_00b66600(Ptr, u32)),
        entry!(
            0x00b6_6640,
            fn_00b66640(Ptr, u8, u8, u8, u8, u8, u8, u8, u8, u8, u32)
        ),
        entry!(0x00b6_6700, fn_00b66700(Ptr, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .cloned()
            .collect()
    }

    fn quiet(e: &mut Engine, addr: u32) {
        e.register(addr, |_, _| Ret::default());
    }

    #[test]
    fn deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        quiet(&mut e, ACCUMULATOR_DESTRUCTOR);
        quiet(&mut e, DEALLOCATE_SIZED);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_64f0, &[0x5000, 3]).u32(), 0x5000);
        assert_eq!(
            calls(&e),
            vec![
                (ACCUMULATOR_DESTRUCTOR, vec![0x5000]),
                (DEALLOCATE_SIZED, vec![0x5000, 0x280]),
            ]
        );
        e.call_log = Some(vec![]);
        e.call(0x00b6_64f0, &[0x5000, 0x102]);
        assert_eq!(calls(&e), vec![(ACCUMULATOR_DESTRUCTOR, vec![0x5000])]);
    }

    #[test]
    fn render_mode_picks_the_pass() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        quiet(&mut e, RENDER_MODE_PREPARE);
        quiet(&mut e, ACCUMULATOR_PASS_00B65AE0);
        quiet(&mut e, 0x0066_0000);
        quiet(&mut e, 0x0066_0002);
        e.mem.set_u32(RENDER_MODE_FUNCTIONS, 0x0066_0000);
        e.mem.set_u32(RENDER_MODE_FUNCTIONS + 8, 0x0066_0002);
        let this = e.mem.alloc(0x280);
        for (mode, target) in [
            (0, ACCUMULATOR_PASS_00B65AE0),
            (2, 0x0066_0002),
            (3, 0x0066_0000),
        ] {
            e.mem.set_u32(this + 0x19c, mode);
            e.call_log = Some(vec![]);
            e.call(0x00b6_6520, &[this]);
            assert_eq!(
                calls(&e),
                vec![(RENDER_MODE_PREPARE, vec![mode]), (target, vec![this])]
            );
        }
    }

    #[test]
    fn finish_runs_the_mode_zero_pass_and_the_base() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        quiet(&mut e, ACCUMULATOR_PASS_00B65C60);
        quiet(&mut e, NI_ACCUMULATOR_FINISH_ACCUMULATING);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 0xc, 7);
        e.mem.set_u32(OBJECT_011F4748, object);
        let this = e.mem.alloc(0x280);
        e.call_log = Some(vec![]);
        e.call(0x00b6_6570, &[this]);
        assert_eq!(
            calls(&e),
            vec![
                (ACCUMULATOR_PASS_00B65C60, vec![this]),
                (NI_ACCUMULATOR_FINISH_ACCUMULATING, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(object + 0xc), 0);
        e.mem.set_u32(this + 0x19c, 1);
        e.call_log = Some(vec![]);
        e.call(0x00b6_6570, &[this]);
        assert_eq!(
            calls(&e),
            vec![(NI_ACCUMULATOR_FINISH_ACCUMULATING, vec![this])]
        );
    }

    #[test]
    fn both_default_passes_run_in_order() {
        let mut e = Engine::new();
        quiet(&mut e, ACCUMULATOR_PASS_00B65AE0);
        quiet(&mut e, ACCUMULATOR_PASS_00B65C60);
        e.call_log = Some(vec![]);
        e.call(0x00b6_65a0, &[0x1234]);
        assert_eq!(
            calls(&e),
            vec![
                (ACCUMULATOR_PASS_00B65AE0, vec![0x1234]),
                (ACCUMULATOR_PASS_00B65C60, vec![0x1234]),
            ]
        );
    }

    fn object_with_slot_d0(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0xd4);
        e.mem.set_u32(vtable + 0xd0, 0x0066_00d0);
        quiet(e, 0x0066_00d0);
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn two_word_helper_then_virtual_d0() {
        let mut e = Engine::new();
        quiet(&mut e, HELPER_00B702B0);
        let this = object_with_slot_d0(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b6_65d0, &[this, 5, 6]);
        assert_eq!(
            calls(&e),
            vec![
                (HELPER_00B702B0, vec![this, 5, 6]),
                (0x0066_00d0, vec![this, 5]),
            ]
        );
    }

    #[test]
    fn one_word_helper_then_virtual_d0() {
        let mut e = Engine::new();
        quiet(&mut e, HELPER_00B70290);
        let this = object_with_slot_d0(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b6_6600, &[this, 9]);
        assert_eq!(
            calls(&e),
            vec![
                (HELPER_00B70290, vec![this, 9]),
                (0x0066_00d0, vec![this, 9])
            ]
        );
    }

    #[test]
    fn nine_bytes_are_stored_only_for_kind_nine() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc8);
        let target = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc4, target);
        let args = [this, 1, 2, 3, 4, 5, 6, 7, 8, 0x109, 0xdead];
        e.call(0x00b6_6640, &args);
        assert_eq!(e.mem.u8(target), 0);
        e.mem.set_u16(this + 0xa8, 9);
        e.call(0x00b6_6640, &args);
        assert_eq!(e.mem.bytes(target, 10), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn value_is_clamped_to_zero_and_one() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        let this = e.mem.alloc(0xe4);
        for (input, expected) in [
            ((-0.5f32).to_bits(), 0),
            (0.25f32.to_bits(), 0.25f32.to_bits()),
            (1.0f32.to_bits(), 1.0f32.to_bits()),
            (7.0f32.to_bits(), 1.0f32.to_bits()),
            (0x7f80_0001, 1.0f32.to_bits()),
            (0x8000_0000, 0x8000_0000),
        ] {
            e.call(0x00b6_6700, &[this, input]);
            assert_eq!(e.mem.u32(this + 0xe0), expected, "{input:#x}");
        }
    }
}

#[cfg(test)]
mod n013_tests {
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
    fn fn_00b665c0_runs_the_base_and_answers_false() {
        let mut e = Engine::new();
        e.register(IS_EQUAL_00B70270, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b6_65c0, &args![0x10u32, 0x20u32]).u8(), 0);
        assert_eq!(calls(&e, IS_EQUAL_00B70270), vec![vec![0x10, 0x20]]);
    }

    #[test]
    fn fn_00b66620_maps_3_to_0_and_the_rest_to_3() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xe0);
        e.call(0x00b6_6620, &args![this, 3u32]);
        assert_eq!(e.mem.u32(this + 0xd8), 0);
        e.call(0x00b6_6620, &args![this, 4u32]);
        assert_eq!(e.mem.u32(this + 0xd8), 3);
    }

    #[test]
    fn fn_00b666d0_passes_the_six_words() {
        let mut e = Engine::new();
        e.put_vtable(0x0f00_1000, &[0; 0x25]);
        e.mem.set_u32(0x0f00_1090, 0x0f00_0090);
        e.register(0x0f00_0090, |_, _| Ret::default());
        let target = e.mem.alloc(4);
        e.mem.set_u32(target, 0x0f00_1000);
        let this = e.mem.alloc(0xc4);
        for i in 0..6 {
            e.mem.set_u32(this + 0xac + 4 * i, 0x100 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x00b6_66d0, &args![this, target]);
        let expected: Vec<Vec<u32>> = (0..6).map(|i| vec![target, i, 0x100 + i]).collect();
        assert_eq!(calls(&e, 0x0f00_0090), expected);
    }
}
