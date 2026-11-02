//! `(unplaced)/run_008001b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The object here is the VATS effect manager by the message `00800370`
//! prints (`"====== VATSEffectOnIdle ======"`); its fields are unnamed and
//! read at their offsets: two arrays of 16 effect pointers at +0x12C and
//! +0x16C, 16 bytes at +0x119 and +0x1AC, two arrays of 16 words at +0x1BC
//! and +0x1FC, and a slot list (count +0xC, 8-byte slots from +0x14).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0066b0d0` (`thiscall`, one word): sets the reference-counted pointer at
/// `this` (here always to null).
const SET_POINTER_0066B0D0: u32 = 0x0066_b0d0;
/// `004a4410` (`thiscall`, one word): the effects' scalar deleting
/// destructor, called with 1.
const EFFECT_DELETE: u32 = 0x004a_4410;
/// `0045cec0` (`thiscall`): the destructor of the members at +0x114,
/// +0x2C4 and +0x2CC.
const MEMBER_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
/// The compiler's `eh vector destructor iterator` (`00ec5fce`, `stdcall`:
/// array, element size, count, destructor) and the element destructor
/// (`00800190`, `thiscall`), also run on +4.
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
const ELEMENT_DESTRUCT_00800190: u32 = 0x0080_0190;
/// `00559450` (`thiscall` on +8): the pointer the idle update needs.
const OWNER_00559450: u32 = 0x0055_9450;
/// `00825c00` (`thiscall` on `011f6394`): the current time word.
const TIMER_OBJECT: u32 = 0x011f_6394;
const TIMER_READ: u32 = 0x0082_5c00;
/// The update steps (`thiscall`, one float each, except `008006b0`).
const STEP_00800900: u32 = 0x0080_0900;
const STEP_00800870: u32 = 0x0080_0870;
const STEP_008006B0: u32 = 0x0080_06b0;
const STEP_008007B0: u32 = 0x0080_07b0;
const STEP_00800820: u32 = 0x0080_0820;
const STEP_00800740: u32 = 0x0080_0740;
/// `00800ac0` (`thiscall`, three words).
const STEP_00800AC0: u32 = 0x0080_0ac0;
/// `Error` (`0040fbe0`, engine map; `cdecl`) and
/// `"====== VATSEffectOnIdle ======"` (`01078dd4`).
const ERROR: u32 = 0x0040_fbe0;
const ON_IDLE_MESSAGE: u32 = 0x0107_8dd4;
/// `0043c4b0` (no arguments): the lock object `004a0370` locks and
/// `004a03c0` unlocks (`thiscall` both).
const LOCK_OBJECT: u32 = 0x0043_c4b0;
const LOCK: u32 = 0x004a_0370;
const UNLOCK: u32 = 0x004a_03c0;
/// `00c4edf0` (`thiscall`: an out word, a flag): whether the effect has
/// finished, with a value written to the out word.
const EFFECT_QUERY_00C4EDF0: u32 = 0x00c4_edf0;

// Translation incomplete: 008001b0 - the FS:[0] exception frame (unwinding the member destructors when one throws) is not translated
// Translated from 008001b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the destructor
/// body. Clears +4, nulls the pointer at +8 (`0066b0d0`); for each of the
/// 16 slots clears the words at +0x1BC and +0x1FC and deletes (`004a4410`
/// with 1) and clears the effects at +0x16C and +0x12C; nulls the pointer
/// at +0x2C4, empties the slot list (`00800ed0`), destroys the members at
/// +0x2CC, +0x2C4 and +0x114 (`0045cec0`), the 0x20 elements of 8 bytes at
/// +0x14 (`00800190` through the vector destructor iterator) and the member
/// at +4 (`00800190`).
pub fn fn_008001b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u32(at + 4, 0);
    e.call(SET_POINTER_0066B0D0, &args![at + 8, 0u32]);
    for i in 0..0x10u32 {
        e.mem.set_u32(at + 0x1bc + i * 4, 0);
        e.mem.set_u32(at + 0x1fc + i * 4, 0);
        for array in [0x16cu32, 0x12c] {
            let effect = e.mem.u32(at + array + i * 4);
            if effect != 0 {
                e.call(EFFECT_DELETE, &args![effect, 1u32]);
            }
            e.mem.set_u32(at + array + i * 4, 0);
        }
    }
    e.call(SET_POINTER_0066B0D0, &args![at + 0x2c4, 0u32]);
    fn_00800ed0(e, this);
    e.call(MEMBER_DESTRUCT_0045CEC0, &args![at + 0x2cc]);
    e.call(MEMBER_DESTRUCT_0045CEC0, &args![at + 0x2c4]);
    e.call(MEMBER_DESTRUCT_0045CEC0, &args![at + 0x114]);
    e.call(
        VECTOR_DESTRUCTOR_ITERATOR,
        &args![at + 0x14, 8u32, 0x20u32, ELEMENT_DESTRUCT_00800190],
    );
    e.call(ELEMENT_DESTRUCT_00800190, &args![at + 4]);
}

// Translated from 00800370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the VATS effect
/// idle update. Without `00559450`'s pointer it clears the bytes at +0x2B4
/// and +0x2B5. Otherwise, with the time (`00825c00`, unsigned, as a float):
/// byte +0x2B4 set runs `00800900`; else byte +0x284 set runs `00800870`,
/// or the time past the float at +0x294 (ordered) runs `008006b0`; then
/// `008007b0`, and with +0 == 1 and +0x10 != -1 `00800820`. Byte +0x298 set
/// runs `00800740`. When the object at +4 answers true from its vtable slot
/// +0x22C (with 0), `00800ac0(0, -1, 0)`. Then it prints the idle message
/// and, for each of the 16 slots with both effects (+0x16C, +0x12C) while
/// `00559450` still answers, under the lock (`0043c4b0`): when both effects
/// report finished (`00c4edf0` with 1, each writing a word) and the slot's
/// byte at +0x1AC is clear, deletes both effects, sets byte +0x1AC, clears
/// byte +0x119, and stores the first word at +0x1BC and the second times
/// the word at +0x2C8 at +0x1FC.
pub fn fn_00800370(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.call(OWNER_00559450, &args![at + 8]).u32() == 0 {
        e.mem.set_u8(at + 0x2b4, 0);
        e.mem.set_u8(at + 0x2b5, 0);
        return;
    }
    let ticks = e.call(TIMER_READ, &args![TIMER_OBJECT]).u32();
    // FILD qword (the word zero-extended), FSTP float.
    let now = x87_store32(f64::from(ticks));
    if e.mem.u8(at + 0x2b4) != 0 {
        e.call(STEP_00800900, &args![at, now]);
    } else {
        if e.mem.u8(at + 0x284) != 0 {
            e.call(STEP_00800870, &args![at, now]);
        } else {
            let start = x87_load32(e.mem.u32(at + 0x294));
            if start < x87_load32(now) {
                e.call(STEP_008006B0, &args![at]);
            }
        }
        e.call(STEP_008007B0, &args![at, now]);
        if e.mem.u32(at) == 1 && e.mem.u32(at + 0x10) != 0xffff_ffff {
            e.call(STEP_00800820, &args![at, now]);
        }
    }
    if e.mem.u8(at + 0x298) != 0 {
        e.call(STEP_00800740, &args![at, now]);
    }
    let object = e.mem.u32(at + 4);
    if object != 0 && e.vcall(object, 0x22c, &args![0u32]).bool() {
        e.call(STEP_00800AC0, &args![at, 0u32, 0xffff_ffffu32, 0u32]);
    }
    e.call(ERROR, &args![ON_IDLE_MESSAGE]);
    for i in 0..0x10u32 {
        if e.call(OWNER_00559450, &args![at + 8]).u32() == 0
            || e.mem.u32(at + 0x16c + i * 4) == 0
            || e.mem.u32(at + 0x12c + i * 4) == 0
        {
            continue;
        }
        let lock = e.call(LOCK_OBJECT, &[]).u32();
        e.call(LOCK, &args![lock]);
        e.with_stack(8, |e, words| {
            let first = words.addr();
            let second = words.addr() + 4;
            let effect = e.mem.u32(at + 0x16c + i * 4);
            if !e
                .call(EFFECT_QUERY_00C4EDF0, &args![effect, first, 1u32])
                .bool()
            {
                return;
            }
            let effect = e.mem.u32(at + 0x12c + i * 4);
            if !e
                .call(EFFECT_QUERY_00C4EDF0, &args![effect, second, 1u32])
                .bool()
            {
                return;
            }
            if e.mem.u8(at + 0x1ac + i) != 0 {
                return;
            }
            for array in [0x16cu32, 0x12c] {
                let effect = e.mem.u32(at + array + i * 4);
                if effect != 0 {
                    e.call(EFFECT_DELETE, &args![effect, 1u32]);
                }
                e.mem.set_u32(at + array + i * 4, 0);
            }
            e.mem.set_u8(at + 0x1ac + i, 1);
            e.mem.set_u8(at + 0x119 + i, 0);
            let value = e.mem.u32(first);
            e.mem.set_u32(at + 0x1bc + i * 4, value);
            let scaled = e.mem.u32(second).wrapping_mul(e.mem.u32(at + 0x2c8));
            e.mem.set_u32(at + 0x1fc + i * 4, scaled);
        });
        let lock = e.call(LOCK_OBJECT, &[]).u32();
        e.call(UNLOCK, &args![lock]);
    }
}

// Translated from 00800ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): empties the slot
/// list: for each of the first `count` (+0xC, signed) 8-byte slots from
/// +0x14, clears its first word and nulls its pointer (`0066b0d0` on the
/// second word); then the count becomes 0 and the word at +0x10 -1.
pub fn fn_00800ed0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let mut i = 0i32;
    while i < e.mem.u32(at + 0xc) as i32 {
        let slot = at.wrapping_add((i as u32).wrapping_mul(8));
        e.mem.set_u32(slot + 0x14, 0);
        e.call(SET_POINTER_0066B0D0, &args![slot + 0x18, 0u32]);
        i += 1;
    }
    e.mem.set_u32(at + 0xc, 0);
    e.mem.set_u32(at + 0x10, 0xffff_ffff);
}

// Translated from 008009d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): when the byte at +0x2B4 is set,
/// sets the byte at +0x2B5 to 1.
pub fn fn_008009d0(e: &mut Engine, this: Ptr) {
    if e.mem.u8(this.addr() + 0x2b4) != 0 {
        e.mem.set_u8(this.addr() + 0x2b5, 1);
    }
}

/// `00403e20` (`thiscall` on a float setting): the address of its value (`this + 4`).
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// `0043d4d0` (`thiscall` on an integer setting): the address of its value.
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
/// The settings the update steps read (objects in `.data`; their names are not confirmed
/// here).
const SETTING_011DB6C8: u32 = 0x011d_b6c8;
const SETTING_011DB6D8: u32 = 0x011d_b6d8;
const SETTING_011DB6E4: u32 = 0x011d_b6e4;
const SETTING_011DB730: u32 = 0x011d_b730;
const SETTING_011DB750: u32 = 0x011d_b750;
const SETTING_011DB75C: u32 = 0x011d_b75c;
const SETTING_011DB78C: u32 = 0x011d_b78c;
const SETTING_011DB7B4: u32 = 0x011d_b7b4;
const SETTING_011DB7C4: u32 = 0x011d_b7c4;
const SETTING_011DB7D8: u32 = 0x011d_b7d8;
/// `float -1.0` (`01012054`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// `double 1.0` (`01012070`), `double 0.5` (`01011588`), `double -1.0` (`0101a6b0`).
const ONE_DOUBLE: u32 = 0x0101_2070;
const HALF_DOUBLE: u32 = 0x0101_1588;
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `rand` (`00ecadb8`, LIBCMT).
const RAND: u32 = 0x00ec_adb8;
/// `00c5d320` (`cdecl`, one float; the result in ST0; unit `BSMain`).
const WAVE_00C5D320: u32 = 0x00c5_d320;
/// `004e44d0` (`cdecl`, one float; the result in ST0; unit `fallout shared/teswater.cpp`).
const WAVE_004E44D0: u32 = 0x004e_44d0;
/// `005b5e40` (`cdecl`, one word: a message) and `"..."` at `01078e78` (the message
/// `00800d50` passes when the slot list is full).
const REPORT_005B5E40: u32 = 0x005b_5e40;
const LIST_FULL_MESSAGE: u32 = 0x0107_8e78;
/// `Error` messages of `00800a90` (`01078df4`) and `00800ac0` (`01078e44` without an
/// actor, `01078e10` with one, two more words).
const CLEAR_MESSAGE: u32 = 0x0107_8df4;
const NO_ACTOR_MESSAGE: u32 = 0x0107_8e44;
const ACTOR_MESSAGE: u32 = 0x0107_8e10;
/// `BSShaderProperty::SetFlagRecurse` (`00ba91a0`, engine map; `cdecl`: object, flag,
/// value).
const SET_FLAG_RECURSE: u32 = 0x00ba_91a0;

/// The address of a float setting's value (`00403e20`).
fn float_setting(e: &mut Engine, setting: u32) -> u32 {
    e.call(FLOAT_SETTING_VALUE, &args![setting]).u32()
}

// Translated from 00800650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words, the third unread): starts the timed
/// effect: byte +0x298 = 1, the floats +0x2A0 = `duration`, +0x2A8 and +0x2A4 = `strength`,
/// and +0x29C = the time (`00825c00`, unsigned, as a float).
pub fn fn_00800650(e: &mut Engine, this: Ptr, duration: u32, strength: u32, _unused_3: u32) {
    let at = this.addr();
    e.mem.set_u8(at + 0x298, 1);
    e.mem.set_u32(at + 0x2a0, x87_quiet_bits(duration));
    e.mem.set_u32(at + 0x2a8, x87_quiet_bits(strength));
    e.mem.set_u32(at + 0x2a4, x87_quiet_bits(strength));
    let ticks = e.call(TIMER_READ, &args![TIMER_OBJECT]).u32();
    // FILD qword (the word zero-extended), FSTP float.
    e.mem.set_u32(at + 0x29c, x87_store32(f64::from(ticks)));
}

// Translated from 008006b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): byte +0x284 = 1, the float +0x288
/// = the time (`00825c00`, unsigned), +0x28C = 0.0.
pub fn fn_008006b0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.mem.set_u8(at + 0x284, 1);
    let ticks = e.call(TIMER_READ, &args![TIMER_OBJECT]).u32();
    e.mem.set_u32(at + 0x288, x87_store32(f64::from(ticks)));
    e.mem.set_u32(at + 0x28c, 0.0f32.to_bits());
}

// Translated from 00800700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): when byte +0x2B4 is clear, sets
/// it, the float +0x2B8 to -1.0 (`01012054`) and +0x2BC to 0.0.
pub fn fn_00800700(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.mem.u8(at + 0x2b4) == 0 {
        e.mem.set_u8(at + 0x2b4, 1);
        let minus_one = e.mem.u32(MINUS_ONE_FLOAT);
        e.mem.set_u32(at + 0x2b8, x87_quiet_bits(minus_one));
        e.mem.set_u32(at + 0x2bc, 0.0f32.to_bits());
    }
}

// Translated from 00800740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float): the timed effect's fade. With the time
/// since the start (`now` - +0x29C) past the duration +0x2A0 (ordered), clears byte +0x298;
/// otherwise +0x2A8 = +0x2A4 * (1 - elapsed / duration).
pub fn fn_00800740(e: &mut Engine, this: Ptr, now: u32) {
    let at = this.addr();
    let elapsed = x87_store32(x87_sub(x87_load32(now), x87_m32(e.mem.u32(at + 0x29c))));
    let duration = x87_load32(e.mem.u32(at + 0x2a0));
    if duration < x87_load32(elapsed) {
        e.mem.set_u8(at + 0x298, 0);
        return;
    }
    let part = x87_div(x87_load32(elapsed), x87_m32(e.mem.u32(at + 0x2a0)));
    // FLD1; FSUBRP: 1 - part.
    let left = x87_store32(x87_sub(1.0, part));
    let strength = x87_mul(x87_load32(e.mem.u32(at + 0x2a4)), x87_m32(left));
    e.mem.set_u32(at + 0x2a8, x87_store32(strength));
}

// Translated from 008007b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float): a wave: `00c5d320(now * setting
/// 011db7d8)` doubled, then +0x2AC = that times setting `011db7c4` and +0x2B0 = that times
/// setting `011db6c8`.
pub fn fn_008007b0(e: &mut Engine, this: Ptr, now: u32) {
    let at = this.addr();
    let rate = float_setting(e, SETTING_011DB7D8);
    let phase = x87_store32(x87_mul(x87_load32(now), x87_m32(e.mem.u32(rate))));
    let wave = e.call(WAVE_00C5D320, &args![x87_quiet_bits(phase)]).f64();
    let wave = x87_store32(x87_add(wave, wave));
    let first = float_setting(e, SETTING_011DB7C4);
    let value = x87_mul(x87_load32(wave), x87_m32(e.mem.u32(first)));
    e.mem.set_u32(at + 0x2ac, x87_store32(value));
    let second = float_setting(e, SETTING_011DB6C8);
    let value = x87_mul(x87_load32(wave), x87_m32(e.mem.u32(second)));
    e.mem.set_u32(at + 0x2b0, x87_store32(value));
}

// Translated from 00800820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float): +0x2C0 = (`004e44d0(now * setting
/// 011db78c)` + 1.0) * 0.5.
pub fn fn_00800820(e: &mut Engine, this: Ptr, now: u32) {
    let rate = float_setting(e, SETTING_011DB78C);
    let phase = x87_store32(x87_mul(x87_load32(now), x87_m32(e.mem.u32(rate))));
    let wave = e.call(WAVE_004E44D0, &args![x87_quiet_bits(phase)]).f64();
    let value = x87_add(wave, e.mem.f64(ONE_DOUBLE));
    let value = x87_store32(x87_mul(value, e.mem.f64(HALF_DOUBLE)));
    e.mem.set_u32(this.addr() + 0x2c0, x87_quiet_bits(value));
}

// Translated from 00800870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float): +0x28C = (`now` - +0x288) / +0x290;
/// once that is past setting `011db6e4` + 1.0 (ordered), clears byte +0x284 and sets +0x294
/// to `now` plus `rand()` modulo the integer setting `011db75c`.
pub fn fn_00800870(e: &mut Engine, this: Ptr, now: u32) {
    let at = this.addr();
    let elapsed = x87_store32(x87_sub(x87_load32(now), x87_m32(e.mem.u32(at + 0x288))));
    let progress = x87_div(x87_load32(elapsed), x87_m32(e.mem.u32(at + 0x290)));
    e.mem.set_u32(at + 0x28c, x87_store32(progress));
    let limit = float_setting(e, SETTING_011DB6E4);
    let limit = x87_add(x87_load32(e.mem.u32(limit)), e.mem.f64(ONE_DOUBLE));
    if x87_load32(e.mem.u32(at + 0x28c)) > limit {
        e.mem.set_u8(at + 0x284, 0);
        let random = e.call(RAND, &[]).i32();
        let modulus = e.call(INT_SETTING_VALUE, &args![SETTING_011DB75C]).u32();
        // IDIV: the remainder (the exe faults on a zero modulus, as this does).
        let remainder = random.wrapping_rem(e.mem.i32(modulus));
        let next = x87_add(f64::from(remainder), x87_m32(now));
        e.mem.set_u32(at + 0x294, x87_store32(next));
    }
}

// Translated from 00800900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one float): with +0x2B8 at -1.0 it becomes `now`;
/// +0x2BC = (`now` - +0x2B8) / setting `011db730`. With byte +0x2B5 set and +0x2BC past
/// setting `011db7b4` + 1.0 (ordered), clears bytes +0x2B4 and +0x2B5 and starts the timed
/// effect ([`fn_00800650`] with settings `011db6d8`, `011db750` and 1).
pub fn fn_00800900(e: &mut Engine, this: Ptr, now: u32) {
    let at = this.addr();
    if x87_load32(e.mem.u32(at + 0x2b8)) == e.mem.f64(MINUS_ONE_DOUBLE) {
        e.mem.set_u32(at + 0x2b8, x87_quiet_bits(now));
    }
    let elapsed = x87_store32(x87_sub(x87_load32(now), x87_m32(e.mem.u32(at + 0x2b8))));
    let scale = float_setting(e, SETTING_011DB730);
    let progress = x87_div(x87_load32(elapsed), x87_m32(e.mem.u32(scale)));
    e.mem.set_u32(at + 0x2bc, x87_store32(progress));
    if e.mem.u8(at + 0x2b5) == 0 {
        return;
    }
    let limit = float_setting(e, SETTING_011DB7B4);
    let limit = x87_add(x87_load32(e.mem.u32(limit)), e.mem.f64(ONE_DOUBLE));
    if x87_load32(e.mem.u32(at + 0x2bc)) > limit {
        e.mem.set_u8(at + 0x2b4, 0);
        e.mem.set_u8(at + 0x2b5, 0);
        let strength = float_setting(e, SETTING_011DB750);
        let strength = x87_quiet_bits(e.mem.u32(strength));
        let duration = float_setting(e, SETTING_011DB6D8);
        let duration = x87_quiet_bits(e.mem.u32(duration));
        fn_00800650(e, this, duration, strength, 1);
    }
}

// Translated from 00800a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words): when slot `index`'s byte at +0x1AC is
/// set, writes the float +0x1FC[index] / +0x1BC[index] (both unsigned; 0.0 when the second
/// is 0) to `out` and returns true; otherwise writes -1.0 (`01012054`) and returns false.
pub fn fn_00800a00(e: &mut Engine, this: Ptr, index: u32, out: u32) -> bool {
    let at = this.addr();
    if e.mem.u8(at.wrapping_add(index) + 0x1ac) == 0 {
        let minus_one = e.mem.u32(MINUS_ONE_FLOAT);
        e.mem.set_u32(out, x87_quiet_bits(minus_one));
        return false;
    }
    let slot = at.wrapping_add(index.wrapping_mul(4));
    let count = e.mem.u32(slot + 0x1bc);
    let ratio = if count != 0 {
        // FILD qword twice; FDIVP ST(1),ST(0) (bytes DE F9); FSTP double.
        x87_div(f64::from(e.mem.u32(slot + 0x1fc)), f64::from(count))
    } else {
        0.0
    };
    e.mem.set_u32(out, x87_store32(ratio));
    true
}

// Translated from 00800a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): prints the message at `01078df4`
/// (`Error`) and clears byte +0x118.
pub fn fn_00800a90(e: &mut Engine, this: Ptr) {
    e.call(ERROR, &args![CLEAR_MESSAGE]);
    e.mem.set_u8(this.addr() + 0x118, 0);
}

/// Deletes slot `index`'s effect in the array at `array` (`004a4410` with 1) and clears it.
fn delete_effect(e: &mut Engine, at: u32, array: u32, index: u32) {
    let field = at.wrapping_add(index.wrapping_mul(4)) + array;
    let effect = e.mem.u32(field);
    if effect != 0 {
        e.call(EFFECT_DELETE, &args![effect, 1u32]);
    }
    e.mem.set_u32(field, 0);
}

// Translated from 00800ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words): sets the effect target. Prints a
/// message (`01078e44` without an actor; `01078e10` with the actor and index when `flag` is
/// set), clears +0x240, and with the actor's model root (vtable +0x1D0) not the one held at
/// +8 clears bytes +0x2B4 and +0x2B5 (when +0x2B4 is set). With byte +0x129 set, +0x2B4
/// clear and the actor's root the one held, starts the timed effect ([`fn_00800650`] with
/// settings `011db6d8`, `011db750` and 1). Then byte +0x118 = `flag` (and byte +0x119 +
/// `index` = 1 when set), +0x23C = `index`, +4 = the actor, the pointer at +8 = its root
/// (`0066b0d0`), bytes +0x119 + `index` and +0x1AC + `index` cleared and, unless `index` is
/// -1, slot `index`'s effects at +0x16C and +0x12C deleted.
pub fn fn_00800ac0(e: &mut Engine, this: Ptr, actor: u32, index: u32, flag: u8) {
    let at = this.addr();
    if actor == 0 {
        e.call(ERROR, &args![NO_ACTOR_MESSAGE]);
    } else if flag != 0 {
        e.call(ERROR, &args![ACTOR_MESSAGE, actor, index]);
    }
    let root = if actor != 0 {
        e.vcall(actor, 0x1d0, &[]).u32()
    } else {
        0
    };
    e.mem.set_u32(at + 0x240, 0);
    if root != e.call(OWNER_00559450, &args![at + 8]).u32() && e.mem.u8(at + 0x2b4) != 0 {
        e.mem.set_u8(at + 0x2b4, 0);
        e.mem.set_u8(at + 0x2b5, 0);
    }
    if e.mem.u8(at + 0x129) != 0 && e.mem.u8(at + 0x2b4) == 0 && actor != 0 {
        let root = e.vcall(actor, 0x1d0, &[]).u32();
        if root == e.call(OWNER_00559450, &args![at + 8]).u32() {
            let strength = float_setting(e, SETTING_011DB750);
            let strength = x87_quiet_bits(e.mem.u32(strength));
            let duration = float_setting(e, SETTING_011DB6D8);
            let duration = x87_quiet_bits(e.mem.u32(duration));
            fn_00800650(e, this, duration, strength, 1);
        }
    }
    e.mem.set_u8(at + 0x118, flag);
    if e.mem.u8(at + 0x118) != 0 {
        e.mem.set_u8(at.wrapping_add(index) + 0x119, 1);
    }
    e.mem.set_u32(at + 0x23c, index);
    e.mem.set_u32(at + 4, actor);
    let root = if actor != 0 {
        e.vcall(actor, 0x1d0, &[]).u32()
    } else {
        0
    };
    e.call(SET_POINTER_0066B0D0, &args![at + 8, root]);
    e.mem.set_u8(at.wrapping_add(index) + 0x119, 0);
    e.mem.set_u8(at.wrapping_add(index) + 0x1ac, 0);
    if index != 0xffff_ffff {
        if e.mem.u32(at.wrapping_add(index.wrapping_mul(4)) + 0x16c) != 0 {
            delete_effect(e, at, 0x16c, index);
        }
        if e.mem.u32(at.wrapping_add(index.wrapping_mul(4)) + 0x12c) != 0 {
            delete_effect(e, at, 0x12c, index);
        }
    }
}

// Translated from 00800d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): appends `value` to the word list at
/// +0x244 (count +0x240; no bound check).
pub fn fn_00800d10(e: &mut Engine, this: Ptr, value: u32) {
    let at = this.addr();
    let count = e.mem.u32(at + 0x240);
    e.mem
        .set_u32(at.wrapping_add(count.wrapping_mul(4)) + 0x244, value);
    let count = e.mem.u32(at + 0x240).wrapping_add(1);
    e.mem.set_u32(at + 0x240, count);
}

// Translated from 00800d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): clears byte +0x2B4 and adds `actor` to
/// the slot list (count +0xC, 8-byte slots from +0x14: the actor, then its model root held
/// through `0066b0d0`) unless it is there. A full list (count + 1 >= 0x20) reports
/// `01078e78` (`005b5e40`) instead. The first actor added also becomes +4, with its root at
/// +8. Then `SetFlagRecurse(0x31, 0)` on the root held one slot past the new count.
pub fn fn_00800d50(e: &mut Engine, this: Ptr, actor: Ptr) {
    let at = this.addr();
    e.mem.set_u8(at + 0x2b4, 0);
    let mut i = 0i32;
    while i < e.mem.i32(at + 0xc) {
        if e.mem
            .u32(at.wrapping_add((i as u32).wrapping_mul(8)) + 0x14)
            == actor.addr()
        {
            return;
        }
        i += 1;
    }
    if e.mem.i32(at + 0xc).wrapping_add(1) >= 0x20 {
        e.call(REPORT_005B5E40, &args![LIST_FULL_MESSAGE]);
        return;
    }
    let count = e.mem.u32(at + 0xc);
    e.mem
        .set_u32(at.wrapping_add(count.wrapping_mul(8)) + 0x14, actor.addr());
    let root = e.vcall(actor.addr(), 0x1d0, &[]).u32();
    let count = e.mem.u32(at + 0xc);
    e.call(
        SET_POINTER_0066B0D0,
        &args![at.wrapping_add(count.wrapping_mul(8)) + 0x18, root],
    );
    let count = e.mem.u32(at + 0xc).wrapping_add(1);
    e.mem.set_u32(at + 0xc, count);
    if e.mem.u32(at + 4) == 0 {
        e.mem.set_u32(at + 4, actor.addr());
        let root = e.vcall(actor.addr(), 0x1d0, &[]).u32();
        e.call(SET_POINTER_0066B0D0, &args![at + 8, root]);
    }
    let count = e.mem.u32(at + 0xc);
    let root = e
        .call(
            OWNER_00559450,
            &args![at.wrapping_add(count.wrapping_mul(8)) + 0x18],
        )
        .u32();
    e.call(SET_FLAG_RECURSE, &args![root, 0x31u32, 0u32]);
}

// Translated from 00800e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): selects `actor`'s slot: +0x10 = the
/// last slot holding it; when none does and the list has room (count < 0x20), +0x10 = the
/// count and the actor is added ([`fn_00800d50`]).
pub fn fn_00800e50(e: &mut Engine, this: Ptr, actor: Ptr) {
    let at = this.addr();
    let mut found = -1i32;
    let mut i = 0i32;
    while i < e.mem.i32(at + 0xc) {
        if e.mem
            .u32(at.wrapping_add((i as u32).wrapping_mul(8)) + 0x14)
            == actor.addr()
        {
            found = i;
        }
        i += 1;
    }
    if found != -1 {
        e.mem.set_u32(at + 0x10, found as u32);
    } else if e.mem.i32(at + 0xc) < 0x20 {
        let count = e.mem.u32(at + 0xc);
        e.mem.set_u32(at + 0x10, count);
        fn_00800d50(e, this, actor);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008001b0, fn_008001b0(Ptr)),
        entry!(0x00800370, fn_00800370(Ptr)),
        entry!(0x00800ed0, fn_00800ed0(Ptr)),
        entry!(0x0080_09d0, fn_008009d0(Ptr)),
        entry!(0x0080_0650, fn_00800650(Ptr, u32, u32, u32)),
        entry!(0x0080_06b0, fn_008006b0(Ptr)),
        entry!(0x0080_0700, fn_00800700(Ptr)),
        entry!(0x0080_0740, fn_00800740(Ptr, u32)),
        entry!(0x0080_07b0, fn_008007b0(Ptr, u32)),
        entry!(0x0080_0820, fn_00800820(Ptr, u32)),
        entry!(0x0080_0870, fn_00800870(Ptr, u32)),
        entry!(0x0080_0900, fn_00800900(Ptr, u32)),
        entry!(0x0080_0a00, fn_00800a00(Ptr, u32, u32) -> bool),
        entry!(0x0080_0a90, fn_00800a90(Ptr)),
        entry!(0x0080_0ac0, fn_00800ac0(Ptr, u32, u32, u8)),
        entry!(0x0080_0d10, fn_00800d10(Ptr, u32)),
        entry!(0x0080_0d50, fn_00800d50(Ptr, Ptr)),
        entry!(0x0080_0e50, fn_00800e50(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            SET_POINTER_0066B0D0,
            EFFECT_DELETE,
            MEMBER_DESTRUCT_0045CEC0,
            VECTOR_DESTRUCTOR_ITERATOR,
            ELEMENT_DESTRUCT_00800190,
            STEP_00800900,
            STEP_00800870,
            STEP_008006B0,
            STEP_008007B0,
            STEP_00800820,
            STEP_00800740,
            STEP_00800AC0,
            ERROR,
            LOCK,
            UNLOCK,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(LOCK_OBJECT, |_, _| 0x7000u32.into_ret());
        e.register(TIMER_READ, |_, _| 100u32.into_ret());
        e
    }

    #[test]
    fn slot_list_is_emptied() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x14, 5);
        e.mem.set_u32(this + 0x1c, 6);
        e.call_log = Some(vec![]);
        e.call(0x0080_0ed0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0xffff_ffff);
        assert_eq!(
            calls_to(&e, SET_POINTER_0066B0D0),
            vec![vec![this + 0x18, 0], vec![this + 0x20, 0]]
        );
    }

    #[test]
    fn destructor_deletes_the_effects() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u32(this + 4, 9);
        e.mem.set_u32(this + 0x16c + 3 * 4, 0x5000);
        e.mem.set_u32(this + 0x12c + 15 * 4, 0x6000);
        e.mem.set_u32(this + 0x1bc, 7);
        e.call_log = Some(vec![]);
        e.call(0x0080_01b0, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0x1bc), 0);
        assert_eq!(e.mem.u32(this + 0x16c + 3 * 4), 0);
        assert_eq!(
            calls_to(&e, EFFECT_DELETE),
            vec![vec![0x5000, 1], vec![0x6000, 1]]
        );
        assert_eq!(
            calls_to(&e, MEMBER_DESTRUCT_0045CEC0),
            vec![vec![this + 0x2cc], vec![this + 0x2c4], vec![this + 0x114]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![this + 0x14, 8, 0x20, ELEMENT_DESTRUCT_00800190]]
        );
        assert_eq!(
            calls_to(&e, ELEMENT_DESTRUCT_00800190),
            vec![vec![this + 4]]
        );
    }

    #[test]
    fn idle_without_owner_clears_the_flags() {
        let mut e = engine();
        e.register(OWNER_00559450, |_, _| Ret::default());
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u8(this + 0x2b4, 1);
        e.mem.set_u8(this + 0x2b5, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_0370, &args![this]);
        assert_eq!(e.mem.u8(this + 0x2b4), 0);
        assert_eq!(e.mem.u8(this + 0x2b5), 0);
        assert!(calls_to(&e, ERROR).is_empty());
    }

    #[test]
    fn idle_runs_the_steps_and_retires_finished_effects() {
        let mut e = engine();
        e.register(OWNER_00559450, |_, _| 1u32.into_ret());
        // Effects finish; the out word is the effect's address / 0x1000.
        e.register(EFFECT_QUERY_00C4EDF0, |e, a| {
            e.mem.set_u32(a[1], a[0] / 0x1000);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 0x10, 0);
        e.mem.set_f32(this + 0x294, 50.0);
        e.mem.set_u8(this + 0x298, 1);
        e.mem.set_u32(this + 0x2c8, 3);
        e.mem.set_u32(this + 0x16c + 2 * 4, 0x5000);
        e.mem.set_u32(this + 0x12c + 2 * 4, 0x6000);
        e.mem.set_u8(this + 0x119 + 2, 1);
        // A slot already retired keeps its effects.
        e.mem.set_u32(this + 0x16c + 4 * 4, 0x7000);
        e.mem.set_u32(this + 0x12c + 4 * 4, 0x8000);
        e.mem.set_u8(this + 0x1ac + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_0370, &args![this]);
        let now = 100.0f32.to_bits();
        assert_eq!(calls_to(&e, STEP_008006B0), vec![vec![this]]);
        assert_eq!(calls_to(&e, STEP_008007B0), vec![vec![this, now]]);
        assert_eq!(calls_to(&e, STEP_00800820), vec![vec![this, now]]);
        assert_eq!(calls_to(&e, STEP_00800740), vec![vec![this, now]]);
        assert!(calls_to(&e, STEP_00800900).is_empty());
        assert_eq!(calls_to(&e, ERROR), vec![vec![ON_IDLE_MESSAGE]]);
        assert_eq!(
            calls_to(&e, EFFECT_DELETE),
            vec![vec![0x5000, 1], vec![0x6000, 1]]
        );
        assert_eq!(e.mem.u8(this + 0x1ac + 2), 1);
        assert_eq!(e.mem.u8(this + 0x119 + 2), 0);
        assert_eq!(e.mem.u32(this + 0x1bc + 2 * 4), 5);
        assert_eq!(e.mem.u32(this + 0x1fc + 2 * 4), 18);
        assert_eq!(e.mem.u32(this + 0x16c + 4 * 4), 0x7000);
        assert_eq!(calls_to(&e, LOCK).len(), 2);
        assert_eq!(calls_to(&e, UNLOCK).len(), 2);
        // With +0x2B4 set only the other step runs.
        e.mem.set_u8(this + 0x2b4, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_0370, &args![this]);
        assert_eq!(calls_to(&e, STEP_00800900), vec![vec![this, now]]);
        assert!(calls_to(&e, STEP_008007B0).is_empty());
    }

    #[test]
    fn fn_008009d0_marks_only_when_the_flag_is_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c0);
        e.call(0x0080_09d0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x2b5), 0);
        e.mem.set_u8(this + 0x2b4, 2);
        e.call(0x0080_09d0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x2b5), 1);
    }
}

#[cfg(test)]
mod tests_n077 {
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

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// An engine with the constants mapped, the settings' values at `setting + 4`, the
    /// timer at 100 and the other callees doubled.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1_0000);
        e.mem.set_u32(MINUS_ONE_FLOAT, bits(-1.0));
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f64(MINUS_ONE_DOUBLE, -1.0);
        e.map(0x011d_b000, 0x1000);
        e.register(FLOAT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(INT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(TIMER_READ, |_, _| 100u32.into_ret());
        for address in [
            ERROR,
            EFFECT_DELETE,
            SET_POINTER_0066B0D0,
            REPORT_005B5E40,
            SET_FLAG_RECURSE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e
    }

    fn set_setting(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_u32(setting + 4, bits(value));
    }

    /// An object whose vtable slot +0x1D0 is a double returning `root`.
    fn actor_with_root(e: &mut Engine, root: u32) -> u32 {
        let vtable = e.mem.alloc(0x200);
        let target = 0x7e00_0000 + root;
        e.mem.set_u32(vtable + 0x1d0, target);
        e.register_double(target, move |_, _| root.into_ret());
        let actor = e.mem.alloc(0x10);
        e.mem.set_u32(actor, vtable);
        actor
    }

    #[test]
    fn fn_00800650_starts_the_timed_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.call(0x0080_0650, &args![this, bits(4.0), bits(2.0), 1u32]);
        assert_eq!(e.mem.u8(this + 0x298), 1);
        assert_eq!(e.mem.f32(this + 0x2a0), 4.0);
        assert_eq!(e.mem.f32(this + 0x2a4), 2.0);
        assert_eq!(e.mem.f32(this + 0x2a8), 2.0);
        assert_eq!(e.mem.f32(this + 0x29c), 100.0);
    }

    #[test]
    fn fn_008006b0_starts_the_progress() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.mem.set_f32(this + 0x28c, 3.0);
        e.call(0x0080_06b0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x284), 1);
        assert_eq!(e.mem.f32(this + 0x288), 100.0);
        assert_eq!(e.mem.f32(this + 0x28c), 0.0);
    }

    #[test]
    fn fn_00800700_only_when_not_running() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.mem.set_f32(this + 0x2bc, 3.0);
        e.call(0x0080_0700, &args![this]);
        assert_eq!(e.mem.u8(this + 0x2b4), 1);
        assert_eq!(e.mem.f32(this + 0x2b8), -1.0);
        assert_eq!(e.mem.f32(this + 0x2bc), 0.0);
        e.mem.set_f32(this + 0x2b8, 7.0);
        e.call(0x0080_0700, &args![this]);
        assert_eq!(e.mem.f32(this + 0x2b8), 7.0);
    }

    #[test]
    fn fn_00800740_fades_then_stops() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u8(this + 0x298, 1);
        e.mem.set_f32(this + 0x29c, 10.0);
        e.mem.set_f32(this + 0x2a0, 4.0);
        e.mem.set_f32(this + 0x2a4, 2.0);
        e.call(0x0080_0740, &args![this, bits(12.0)]);
        assert_eq!(e.mem.f32(this + 0x2a8), 1.0);
        assert_eq!(e.mem.u8(this + 0x298), 1);
        // Exactly the duration still fades (to 0).
        e.call(0x0080_0740, &args![this, bits(14.0)]);
        assert_eq!(e.mem.f32(this + 0x2a8), 0.0);
        e.call(0x0080_0740, &args![this, bits(20.0)]);
        assert_eq!(e.mem.u8(this + 0x298), 0);
    }

    #[test]
    fn fn_008007b0_scales_the_doubled_wave() {
        let mut e = engine();
        e.register(WAVE_00C5D320, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) / 12.0,
            ..Ret::default()
        });
        set_setting(&mut e, SETTING_011DB7D8, 2.0);
        set_setting(&mut e, SETTING_011DB7C4, 3.0);
        set_setting(&mut e, SETTING_011DB6C8, 4.0);
        let this = e.mem.alloc(0x2d0);
        e.call_log = Some(vec![]);
        e.call(0x0080_07b0, &args![this, bits(3.0)]);
        assert_eq!(calls_to(&e, WAVE_00C5D320), vec![vec![bits(6.0)]]);
        assert_eq!(e.mem.f32(this + 0x2ac), 3.0);
        assert_eq!(e.mem.f32(this + 0x2b0), 4.0);
    }

    #[test]
    fn fn_00800820_maps_the_wave_to_zero_one() {
        let mut e = engine();
        e.register(WAVE_004E44D0, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) / 4.0,
            ..Ret::default()
        });
        set_setting(&mut e, SETTING_011DB78C, 2.0);
        let this = e.mem.alloc(0x2d0);
        e.call_log = Some(vec![]);
        e.call(0x0080_0820, &args![this, bits(1.0)]);
        assert_eq!(calls_to(&e, WAVE_004E44D0), vec![vec![bits(2.0)]]);
        assert_eq!(e.mem.f32(this + 0x2c0), 0.75);
    }

    #[test]
    fn fn_00800870_ends_the_progress_and_picks_the_next_start() {
        let mut e = engine();
        e.register(RAND, |_, _| 17u32.into_ret());
        e.mem.set_u32(SETTING_011DB75C + 4, 5);
        set_setting(&mut e, SETTING_011DB6E4, 3.0);
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u8(this + 0x284, 1);
        e.mem.set_f32(this + 0x288, 1.0);
        e.mem.set_f32(this + 0x290, 2.0);
        e.call(0x0080_0870, &args![this, bits(5.0)]);
        assert_eq!(e.mem.f32(this + 0x28c), 2.0);
        assert_eq!(e.mem.u8(this + 0x284), 1);
        set_setting(&mut e, SETTING_011DB6E4, 0.5);
        e.call(0x0080_0870, &args![this, bits(5.0)]);
        assert_eq!(e.mem.u8(this + 0x284), 0);
        assert_eq!(e.mem.f32(this + 0x294), 7.0);
    }

    #[test]
    fn fn_00800900_tracks_the_phase_and_restarts_the_effect() {
        let mut e = engine();
        set_setting(&mut e, SETTING_011DB730, 2.0);
        set_setting(&mut e, SETTING_011DB7B4, 1.0);
        set_setting(&mut e, SETTING_011DB6D8, 6.0);
        set_setting(&mut e, SETTING_011DB750, 0.25);
        let this = e.mem.alloc(0x2d0);
        e.mem.set_f32(this + 0x2b8, -1.0);
        e.mem.set_u8(this + 0x2b4, 1);
        e.mem.set_u8(this + 0x2b5, 1);
        e.call(0x0080_0900, &args![this, bits(8.0)]);
        assert_eq!(e.mem.f32(this + 0x2b8), 8.0);
        assert_eq!(e.mem.f32(this + 0x2bc), 0.0);
        assert_eq!(e.mem.u8(this + 0x2b4), 1);
        // 14 - 8 over 2 is 3, past 1 + 1.
        e.call(0x0080_0900, &args![this, bits(14.0)]);
        assert_eq!(e.mem.f32(this + 0x2bc), 3.0);
        assert_eq!((e.mem.u8(this + 0x2b4), e.mem.u8(this + 0x2b5)), (0, 0));
        assert_eq!(e.mem.u8(this + 0x298), 1);
        assert_eq!(e.mem.f32(this + 0x2a0), 6.0);
        assert_eq!(e.mem.f32(this + 0x2a8), 0.25);
    }

    #[test]
    fn fn_00800a00_reports_the_ratio_of_a_retired_slot() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        let out = e.mem.alloc(4);
        assert!(!e.call(0x0080_0a00, &args![this, 3u32, out]).bool());
        assert_eq!(e.mem.f32(out), -1.0);
        e.mem.set_u8(this + 0x1ac + 3, 1);
        assert!(e.call(0x0080_0a00, &args![this, 3u32, out]).bool());
        assert_eq!(e.mem.f32(out), 0.0);
        e.mem.set_u32(this + 0x1bc + 12, 4);
        e.mem.set_u32(this + 0x1fc + 12, 3);
        assert!(e.call(0x0080_0a00, &args![this, 3u32, out]).bool());
        assert_eq!(e.mem.f32(out), 0.75);
    }

    #[test]
    fn fn_00800a90_prints_and_clears() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u8(this + 0x118, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_0a90, &args![this]);
        assert_eq!(calls_to(&e, ERROR), vec![vec![CLEAR_MESSAGE]]);
        assert_eq!(e.mem.u8(this + 0x118), 0);
    }

    #[test]
    fn fn_00800ac0_sets_the_target_and_clears_its_slot() {
        let mut e = engine();
        e.register(OWNER_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        set_setting(&mut e, SETTING_011DB6D8, 6.0);
        set_setting(&mut e, SETTING_011DB750, 0.25);
        let actor = actor_with_root(&mut e, 0x900);
        let this = e.mem.alloc(0x2d0);
        // The held root is another: the running effect stops.
        e.mem.set_u32(this + 8, 0x800);
        e.mem.set_u8(this + 0x2b4, 1);
        e.mem.set_u8(this + 0x2b5, 1);
        e.mem.set_u32(this + 0x240, 5);
        e.mem.set_u32(this + 0x16c + 8, 0x5000);
        e.mem.set_u32(this + 0x12c + 8, 0x6000);
        e.mem.set_u8(this + 0x1ac + 2, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_0ac0, &args![this, actor, 2u32, 1u32]);
        assert_eq!(calls_to(&e, ERROR), vec![vec![ACTOR_MESSAGE, actor, 2]]);
        assert_eq!((e.mem.u8(this + 0x2b4), e.mem.u8(this + 0x2b5)), (0, 0));
        assert_eq!(e.mem.u32(this + 0x240), 0);
        assert_eq!(e.mem.u8(this + 0x118), 1);
        assert_eq!(e.mem.u8(this + 0x119 + 2), 0);
        assert_eq!(e.mem.u8(this + 0x1ac + 2), 0);
        assert_eq!(e.mem.u32(this + 0x23c), 2);
        assert_eq!(e.mem.u32(this + 4), actor);
        assert_eq!(
            calls_to(&e, SET_POINTER_0066B0D0),
            vec![vec![this + 8, 0x900]]
        );
        assert_eq!(
            calls_to(&e, EFFECT_DELETE),
            vec![vec![0x5000, 1], vec![0x6000, 1]]
        );
        assert_eq!(e.mem.u8(this + 0x298), 0);
        // No actor: the other message, nothing started.
        e.mem.set_u32(this + 8, 0x900);
        e.mem.set_u8(this + 0x129, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_0ac0, &args![this, 0u32, 0xffff_ffffu32, 0u32]);
        assert_eq!(calls_to(&e, ERROR), vec![vec![NO_ACTOR_MESSAGE]]);
        assert_eq!(e.mem.u8(this + 0x298), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        // The same root with +0x129 set starts the timed effect.
        e.mem.set_u32(this + 8, 0x900);
        e.call(0x0080_0ac0, &args![this, actor, 1u32, 0u32]);
        assert_eq!(e.mem.u8(this + 0x298), 1);
        assert_eq!(e.mem.f32(this + 0x2a0), 6.0);
        assert_eq!(e.mem.u8(this + 0x118), 0);
    }

    #[test]
    fn fn_00800d10_appends() {
        let mut e = engine();
        let this = e.mem.alloc(0x2d0);
        e.call(0x0080_0d10, &args![this, 7u32]);
        e.call(0x0080_0d10, &args![this, 9u32]);
        assert_eq!(e.mem.u32(this + 0x240), 2);
        assert_eq!((e.mem.u32(this + 0x244), e.mem.u32(this + 0x248)), (7, 9));
    }

    #[test]
    fn fn_00800d50_adds_once_and_reports_a_full_list() {
        let mut e = engine();
        e.register(OWNER_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        let actor = actor_with_root(&mut e, 0x900);
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u8(this + 0x2b4, 1);
        e.mem.set_u32(this + 0x18 + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0080_0d50, &args![this, actor]);
        assert_eq!(e.mem.u8(this + 0x2b4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0x14), actor);
        assert_eq!(e.mem.u32(this + 4), actor);
        assert_eq!(
            calls_to(&e, SET_POINTER_0066B0D0),
            vec![vec![this + 0x18, 0x900], vec![this + 8, 0x900]]
        );
        assert_eq!(calls_to(&e, SET_FLAG_RECURSE), vec![vec![0x4444, 0x31, 0]]);
        // Already there: nothing.
        e.call_log = Some(vec![]);
        e.call(0x0080_0d50, &args![this, actor]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // Full.
        e.mem.set_u32(this + 0xc, 0x1f);
        e.call_log = Some(vec![]);
        e.call(0x0080_0d50, &args![this, 0x1234u32]);
        assert_eq!(calls_to(&e, REPORT_005B5E40), vec![vec![LIST_FULL_MESSAGE]]);
    }

    #[test]
    fn fn_00800e50_selects_or_adds() {
        let mut e = engine();
        e.register(OWNER_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        let actor = actor_with_root(&mut e, 0x900);
        let this = e.mem.alloc(0x2d0);
        e.mem.set_u32(this + 0xc, 3);
        e.mem.set_u32(this + 0x14, actor);
        e.mem.set_u32(this + 0x14 + 16, actor);
        e.call(0x0080_0e50, &args![this, actor]);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        let other = actor_with_root(&mut e, 0xa00);
        e.call(0x0080_0e50, &args![this, other]);
        assert_eq!(e.mem.u32(this + 0x10), 3);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        assert_eq!(e.mem.u32(this + 0x14 + 24), other);
        // No room: nothing.
        e.mem.set_u32(this + 0xc, 0x20);
        e.mem.set_u32(this + 0x10, 9);
        e.call(0x0080_0e50, &args![this, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0x10), 9);
    }
}
