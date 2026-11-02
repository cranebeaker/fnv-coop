//! `fallout shared/run_0050de20` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0050e6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8c (`SUB ECX,0x8c`) and jumps to `0050e580`. `0050e580` (`RET 0x8`) takes two stack words and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack words are passed on unchanged.
pub fn fn_0050e6a0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(
        0x0050_e580,
        &args![this.addr().wrapping_sub(0x8c), first, second],
    );
}

// Translated from 0050e6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8c (`SUB ECX,0x8c`) and jumps to `0050e550`. `0050e550` (`RET 0x4`) takes one stack word and leaves no deliberate value in `EAX` (its decompiled form is `void`), so the thunk returns nothing; the parameter count is from the `RET`. The stack word is passed on unchanged.
pub fn fn_0050e6b0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(
        0x0050_e550,
        &args![this.addr().wrapping_sub(0x8c), argument],
    );
}

/// `TESObjectLIGH` fields the light update reads, at their PC offsets (the
/// Xbox PDB's `data.flags` +0xb8 and `fFade` +0xc4, 0x10 lower as for every
/// form on the PC).
const LIGHT_FLAGS: u32 = 0xa8;
const LIGHT_FADE: u32 = 0xb4;
/// `NiPointer` members: the copy constructor (`00559a40`), the pointer
/// (`00559450`, the first word) and the destructor (`0045cec0`).
const POINTER_COPY: u32 = 0x0055_9a40;
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The frame timer object and its seconds-passed getter (`0084d030`,
/// `thiscall`, answer in ST0).
const TIMER: u32 = 0x011f_6394;
const SECONDS_PASSED: u32 = 0x0084_d030;
/// The game's random number (`00487f50`, no arguments).
const RANDOM: u32 = 0x0048_7f50;
/// The light node's getters (answers in ST0) and setters (one `float`) of
/// its three phases, `0050c630`/`0050c660`/`0050c690` the setters.
const PHASE_ONE_GET: u32 = 0x004e_4600;
const PHASE_TWO_GET: u32 = 0x0064_47d0;
const PHASE_THREE_GET: u32 = 0x004e_4620;
const PHASE_ONE_SET: u32 = 0x0050_c630;
const PHASE_TWO_SET: u32 = 0x0050_c660;
const PHASE_THREE_SET: u32 = 0x0050_c690;
/// The sine-like function of one `float` (`004e44b0`, cdecl, ST0).
const WAVE: u32 = 0x004e_44b0;
/// The float setting at `011c8fcc` and its value-pointer getter
/// (`00403e20`).
const FLICKER_SETTING: u32 = 0x011c_8fcc;
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// The node's colour setter (`004bc1f0`, three floats).
const SET_COLOR: u32 = 0x004b_c1f0;
/// The test whether lights are updated through the manager (`008c7aa0`),
/// the manager (`004537b0`) and its update for a node (`0087ad50`).
const MANAGED: u32 = 0x008c_7aa0;
const MANAGER: u32 = 0x0045_37b0;
const MANAGER_UPDATE: u32 = 0x0087_ad50;
/// The 12-byte point constructor (`0043d410`, three words) and the node's
/// set-point (`00a59c60`).
const POINT_CONSTRUCT: u32 = 0x0043_d410;
const SET_POINT: u32 = 0x00a5_9c60;
/// Clamps (`0040ebd0(limit, value)` and `00404010(limit, value)`, cdecl,
/// answers in ST0).
const CLAMP_HIGH: u32 = 0x0040_ebd0;
const CLAMP_LOW: u32 = 0x0040_4010;
/// The node's dimmer getter (`00644a50`, ST0) and setter (`0050dd20`, one
/// `float`), and `fabs`-like `00408840` (cdecl, ST0).
const DIMMER_GET: u32 = 0x0064_4a50;
const DIMMER_SET: u32 = 0x0050_dd20;
const ABSOLUTE: u32 = 0x0040_8840;
/// `double` constants: 2.0, 2^31 / 6 (357913952), 1.1, 1.2, 2^31 / 9
/// (238609296), 1.3, 2π (as stored), 1.7, 0.5, 0.3, 2.6, 0.0, 3.0, 5.0,
/// 1.0, 0.4, 0.6, 0.01, 0.25 (as the exe stores them).
const TWO: u32 = 0x0101_1590;
const RANDOM_SCALE_ONE: u32 = 0x0102_9100;
const PHASE_ONE_STEP: u32 = 0x0102_90f8;
const PHASE_TWO_STEP: u32 = 0x0102_90f0;
const RANDOM_SCALE_THREE: u32 = 0x0102_90e8;
const PHASE_THREE_STEP: u32 = 0x0102_90e0;
const FULL_TURN: u32 = 0x0101_ff48;
const ONE_POINT_SEVEN: u32 = 0x0102_90d8;
const HALF: u32 = 0x0101_1588;
const ZERO_POINT_THREE: u32 = 0x0102_90d0;
const TWO_POINT_SIX: u32 = 0x0102_90c8;
const ZERO: u32 = 0x0101_2060;
const THREE: u32 = 0x0102_1928;
const FIVE: u32 = 0x0102_0998;
const ONE: u32 = 0x0101_2070;
const ZERO_POINT_FOUR: u32 = 0x0102_90c0;
const ZERO_POINT_SIX: u32 = 0x0102_90b8;
const ONE_HUNDREDTH: u32 = 0x0101_6408;
const QUARTER: u32 = 0x0102_90b0;
/// `float` constants: -1.0, 0.1, 0.05.
const MINUS_ONE_F: u32 = 0x0101_2054;
const DIM_STEP: u32 = 0x0101_e2bc;
const SLOW_DIM_STEP: u32 = 0x0101_81bc;

/// `FSTP float`: the x87 value rounded to a `float`.
fn store32(value: f64) -> f32 {
    f32::from_bits(x87_store32(value))
}

/// A `float` loaded and stored again (`FLD`/`FSTP`), as pushed bits.
fn copied(value: f32) -> u32 {
    x87_quiet_bits(value.to_bits())
}

// Translated from 0050de20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The light update with a node holder (`TESObjectLIGH`, by the fields the
/// callee reads): copies the holder's `NiPointer` (`00559a40`) into an
/// argument and runs `0050de60(pointer, &holder[+4], extra)`, answering
/// its result.
pub fn fn_0050de20(e: &mut Engine, this: Ptr, holder: u32, extra: u32) -> bool {
    let pointer = e.with_stack(4, |e, slot| {
        e.call(POINTER_COPY, &args![slot, holder]);
        e.mem.u32(slot.addr())
    });
    fn_0050de60(e, this, pointer, holder.wrapping_add(4), extra)
}

// Translated from 0050de60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A light's per-frame update of its node (`this` is the `TESObjectLIGH`,
/// by its fields; `node` an `NiPointer` passed by value and released at
/// the end; `fade` points at the current dimmer target; `owner` is the
/// optional object whose `float` at +0xc is tested for 0).
///
/// - Flicker (flag 0x8) or slow flicker (0x40): advances three phases by
///   the frame time (halved for 0x40) scaled by random steps, wraps them
///   at a full turn, stores them in the node, sets the node's colour from
///   the flicker setting and two waves of the phases, updates it through
///   the manager or a zero point, and sets the dimmer from a mix of three
///   waves clamped to [-1, 1].
/// - Otherwise, with pulse (0x80) or slow pulse (0x100), or an owner whose
///   value is 0: moves the dimmer toward `*fade` by 0.1 (0.05 for 0x40 or
///   0x100) per second; once within one step of it, either answers true
///   (an owner with value 0) or picks the next target: a random share of
///   `fFade` for flicker, `fFade` or a quarter of it for pulse.
///
/// Answers whether the owner case finished.
// Translation incomplete: 0050de60 - the FS:[0] exception frame (releases the NiPointer argument when a callee throws) is not translated
pub fn fn_0050de60(e: &mut Engine, this: Ptr, node: u32, fade: u32, owner: u32) -> bool {
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), node);
        let node = e.call(POINTER_GET, &args![argument]).u32();
        let mut finished = false;
        let flags = e.mem.u32(this.addr() + LIGHT_FLAGS);
        let fade_of_light = this.addr() + LIGHT_FADE;
        if flags & 0x8 != 0 || flags & 0x40 != 0 {
            let mut step = store32(e.call(SECONDS_PASSED, &args![TIMER]).f64());
            let mut one = 1.0f32;
            let mut two = 0.0f32;
            let mut three = 0.0f32;
            if node != 0 {
                one = store32(e.call(PHASE_ONE_GET, &args![node]).f64());
                two = store32(e.call(PHASE_TWO_GET, &args![node]).f64());
                three = store32(e.call(PHASE_THREE_GET, &args![node]).f64());
            }
            if e.mem.u32(this.addr() + LIGHT_FLAGS) & 0x40 != 0 {
                step = store32(x87_div(x87_load32(step.to_bits()), e.global::<f64>(TWO)));
            }
            // `local = -1` (stored, never read).
            let advance = |e: &mut Engine, phase: f32, scale: u32, base: u32| -> f32 {
                let random = f64::from(e.call(RANDOM, &[]).u32());
                let share = x87_div(random, e.global::<f64>(scale));
                let factor = x87_add(share, e.global::<f64>(base));
                let moved = x87_mul(factor, x87_m32(step.to_bits()));
                store32(x87_add(moved, x87_m32(phase.to_bits())))
            };
            one = advance(e, one, RANDOM_SCALE_ONE, PHASE_ONE_STEP);
            two = advance(e, two, RANDOM_SCALE_ONE, PHASE_TWO_STEP);
            three = advance(e, three, RANDOM_SCALE_THREE, PHASE_THREE_STEP);
            let full_turn: f64 = e.global(FULL_TURN);
            for phase in [&mut one, &mut two, &mut three] {
                if x87_load32(phase.to_bits()) > full_turn {
                    *phase = store32(x87_sub(x87_load32(phase.to_bits()), full_turn));
                }
            }
            if node != 0 {
                e.call(PHASE_ONE_SET, &args![node, copied(one)]);
                e.call(PHASE_TWO_SET, &args![node, copied(two)]);
                e.call(PHASE_THREE_SET, &args![node, copied(three)]);
            }
            let one_point_seven: f64 = e.global(ONE_POINT_SEVEN);
            let half: f64 = e.global(HALF);
            let zero_point_three: f64 = e.global(ZERO_POINT_THREE);
            let shifted = x87_add(x87_load32(one.to_bits()), one_point_seven);
            let input = store32(x87_add(shifted, shifted));
            let wave_one = store32(e.call(WAVE, &args![copied(input)]).f64());
            let input = store32(x87_mul(
                x87_add(x87_load32(two.to_bits()), half),
                one_point_seven,
            ));
            let wave_two = store32(e.call(WAVE, &args![copied(input)]).f64());
            let input = store32(x87_mul(
                x87_add(x87_load32(three.to_bits()), zero_point_three),
                e.global::<f64>(TWO_POINT_SIX),
            ));
            let wave_three = store32(e.call(WAVE, &args![copied(input)]).f64());
            let setting = e.call(FLOAT_SETTING_VALUE, &args![FLICKER_SETTING]).u32();
            let mut level = f32::from_bits(x87_quiet_bits(e.mem.u32(setting)));
            let swing = store32(x87_div(
                x87_mul(
                    x87_mul(x87_load32(level.to_bits()), x87_m32(wave_one.to_bits())),
                    x87_m32(wave_two.to_bits()),
                ),
                e.global::<f64>(TWO),
            ));
            let total = x87_add(x87_load32(level.to_bits()), x87_m32(swing.to_bits()));
            let kept = if total > e.global::<f64>(ZERO) {
                f32::from_bits(copied(swing))
            } else {
                0.0
            };
            level = f32::from_bits(copied(kept));
            let x = store32(x87_mul(
                x87_load32(wave_three.to_bits()),
                x87_m32(level.to_bits()),
            ));
            let y = store32(x87_mul(
                x87_load32(wave_two.to_bits()),
                x87_m32(level.to_bits()),
            ));
            let z = store32(x87_mul(
                x87_load32(wave_one.to_bits()),
                x87_m32(level.to_bits()),
            ));
            let target = e.call(POINTER_GET, &args![argument]).u32();
            e.call(SET_COLOR, &args![target, copied(z), copied(y), copied(x)]);
            if e.call(MANAGED, &[]).bool() {
                let target = e.call(POINTER_GET, &args![argument]).u32();
                let manager = e.call(MANAGER, &[]).u32();
                e.call(MANAGER_UPDATE, &args![manager, target]);
            } else {
                e.with_stack(0xc, |e, point| {
                    e.call(POINT_CONSTRUCT, &args![point, 0.0f32, 0u32, 0u32]);
                    let target = e.call(POINTER_GET, &args![argument]).u32();
                    e.call(SET_POINT, &args![target, point]);
                });
            }
            let one_constant: f64 = e.global(ONE);
            let three_constant: f64 = e.global(THREE);
            let input = store32(x87_mul(
                x87_add(x87_load32(three.to_bits()), zero_point_three),
                three_constant,
            ));
            let fifth = x87_div(
                e.call(WAVE, &args![copied(input)]).f64(),
                e.global::<f64>(FIVE),
            );
            let input = store32(x87_mul(
                x87_add(x87_load32(two.to_bits()), half),
                e.global::<f64>(PHASE_THREE_STEP),
            ));
            let halved = x87_div(
                x87_add(e.call(WAVE, &args![copied(input)]).f64(), one_constant),
                e.global::<f64>(TWO),
            );
            let input = store32(x87_mul(
                x87_add(x87_load32(one.to_bits()), one_point_seven),
                e.global::<f64>(PHASE_ONE_STEP),
            ));
            let mixed = x87_add(
                x87_div(
                    x87_div(
                        x87_mul(
                            x87_add(e.call(WAVE, &args![copied(input)]).f64(), one_constant),
                            halved,
                        ),
                        e.global::<f64>(TWO),
                    ),
                    three_constant,
                ),
                fifth,
            );
            let mix = store32(mixed);
            let high = e.call(CLAMP_HIGH, &args![1.0f32, copied(mix)]).f64();
            let minus_one = e.mem.u32(MINUS_ONE_F);
            let low = e
                .call(
                    CLAMP_LOW,
                    &args![x87_quiet_bits(minus_one), x87_store32(high)],
                )
                .f64();
            let mix = store32(low);
            let mix = store32(x87_add(x87_mul(x87_load32(mix.to_bits()), half), half));
            let fade_bits = e.mem.u32(fade_of_light);
            let dimmer = store32(x87_mul(
                x87_add(
                    x87_mul(x87_load32(mix.to_bits()), e.global::<f64>(ZERO_POINT_FOUR)),
                    e.global::<f64>(ZERO_POINT_SIX),
                ),
                x87_m32(fade_bits),
            ));
            let target = e.call(POINTER_GET, &args![argument]).u32();
            e.call(DIMMER_SET, &args![target, copied(dimmer)]);
        } else {
            let pulse = flags & 0x80 != 0 || flags & 0x100 != 0;
            if pulse || (owner != 0 && owner_value_is_zero(e, owner)) {
                let target = e.call(POINTER_GET, &args![argument]).u32();
                let mut dimmer = store32(e.call(DIMMER_GET, &args![target]).f64());
                let seconds = store32(e.call(SECONDS_PASSED, &args![TIMER]).f64());
                let mut step = f32::from_bits(x87_quiet_bits(e.mem.u32(DIM_STEP)));
                let flags = e.mem.u32(this.addr() + LIGHT_FLAGS);
                if flags & 0x40 != 0 || flags & 0x100 != 0 {
                    step = f32::from_bits(x87_quiet_bits(e.mem.u32(SLOW_DIM_STEP)));
                }
                let change = x87_mul(x87_load32(step.to_bits()), x87_m32(seconds.to_bits()));
                if x87_load32(e.mem.u32(fade)) > x87_load32(dimmer.to_bits()) {
                    dimmer = store32(x87_add(change, x87_m32(dimmer.to_bits())));
                } else {
                    dimmer = store32(x87_sub(x87_m32(dimmer.to_bits()), change));
                }
                let target = e.call(POINTER_GET, &args![argument]).u32();
                e.call(DIMMER_SET, &args![target, copied(dimmer)]);
                let distance = store32(x87_sub(
                    x87_load32(dimmer.to_bits()),
                    x87_m32(e.mem.u32(fade)),
                ));
                let distance = e.call(ABSOLUTE, &args![copied(distance)]).f64();
                if x87_load32(step.to_bits()) > distance {
                    let flags = e.mem.u32(this.addr() + LIGHT_FLAGS);
                    if owner != 0 && owner_value_is_zero(e, owner) {
                        finished = true;
                    } else if flags & 0x8 != 0 || flags & 0x40 != 0 {
                        // Reached only when a callee set these flags since
                        // the start (both were clear there); the exe reads
                        // them again here.
                        let share = e.call(RANDOM, &[]).u32() % 0x4b;
                        let value = x87_mul(
                            x87_add(
                                x87_mul(f64::from(share), e.global::<f64>(ONE_HUNDREDTH)),
                                e.global::<f64>(QUARTER),
                            ),
                            x87_m32(e.mem.u32(fade_of_light)),
                        );
                        e.mem.set_u32(fade, x87_store32(value));
                    } else if flags & 0x80 != 0 || flags & 0x100 != 0 {
                        if x87_load32(e.mem.u32(fade)) > half_of(e) {
                            let value = x87_mul(
                                x87_load32(e.mem.u32(fade_of_light)),
                                e.global::<f64>(QUARTER),
                            );
                            e.mem.set_u32(fade, x87_store32(value));
                        } else {
                            let value = x87_quiet_bits(e.mem.u32(fade_of_light));
                            e.mem.set_u32(fade, value);
                        }
                    }
                }
            }
        }
        e.call(POINTER_DESTRUCT, &args![argument]);
        finished
    })
}

/// Whether the owner's `float` at +0xc equals 0.0 (`FCOMP`; `TEST AH,0x44;
/// JP`: false for NaN).
fn owner_value_is_zero(e: &Engine, owner: u32) -> bool {
    x87_load32(e.mem.u32(owner.wrapping_add(0xc))) == e.global::<f64>(ZERO)
}

/// The `double` 0.5 (`01011588`).
fn half_of(e: &Engine) -> f64 {
    e.global(HALF)
}

// Translated from 0050e620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `0050e640` with the byte.
pub fn fn_0050e620(e: &mut Engine, this: Ptr, value: u8) -> bool {
    fn_0050e640(e, this, value)
}

// Translated from 0050e640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the object is not null and its byte at +4 equals `value`.
pub fn fn_0050e640(e: &mut Engine, this: Ptr, value: u8) -> bool {
    if this.is_null() {
        return false;
    }
    e.mem.u8(this.addr() + 4) == value
}

/// The byte `0050e6c0` assembles, and the settings it checks (each tested
/// with `0050e620(1)`).
const LIGHT_MODE_BITS: u32 = 0x011c_8fa0;
const MODE_SETTING_ONE: u32 = 0x011c_9060;
const MODE_SETTING_TWO: u32 = 0x011c_8ff0;
const MODE_SETTING_FOUR: u32 = 0x011c_9028;
/// Integer settings (`0043d4d0` gives the value's address, `0045ce80` sets
/// it) and float settings (`00403e20` gives the value's address,
/// `004ed780` sets it) that `0050e6c0` repairs.
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
const INT_SETTING_SET: u32 = 0x0045_ce80;
const FLOAT_SETTING_SET: u32 = 0x004e_d780;
const INT_SETTING_A: u32 = 0x011c_9054;
const INT_SETTING_B: u32 = 0x011c_907c;
const FLOAT_SETTING_A: u32 = 0x011c_8fb0;
const FLOAT_SETTING_B: u32 = 0x011c_906c;

// Translated from 0050e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assembles the byte at `011c8fa0` from three settings (bit 0, 1 and 2
/// for those whose byte at +4 is 1; bit 2 when none is), resets the
/// integer setting at `011c9054` to 1 and the one at `011c907c` to 2
/// unless they hold 0, 1 or 2, and sets the float settings at `011c8fb0`
/// and `011c906c` to 1.0 when they are 0. Answers true.
pub fn fn_0050e6c0(e: &mut Engine) -> bool {
    e.mem.set_u8(LIGHT_MODE_BITS, 0);
    for (setting, bit) in [
        (MODE_SETTING_ONE, 1u8),
        (MODE_SETTING_TWO, 2),
        (MODE_SETTING_FOUR, 4),
    ] {
        if fn_0050e620(e, Ptr::new(setting), 1) {
            let bits = e.mem.u8(LIGHT_MODE_BITS);
            e.mem.set_u8(LIGHT_MODE_BITS, bits | bit);
        }
    }
    if e.mem.u8(LIGHT_MODE_BITS) == 0 {
        let bits = e.mem.u8(LIGHT_MODE_BITS);
        e.mem.set_u8(LIGHT_MODE_BITS, bits | 4);
    }
    for (setting, value) in [(INT_SETTING_A, 1u32), (INT_SETTING_B, 2)] {
        let mut keep = false;
        for allowed in 0..3u32 {
            let at = e.call(INT_SETTING_VALUE, &args![setting]).u32();
            if e.mem.u32(at) == allowed {
                keep = true;
                break;
            }
        }
        if !keep {
            e.call(INT_SETTING_SET, &args![setting, value]);
        }
    }
    for setting in [FLOAT_SETTING_A, FLOAT_SETTING_B] {
        let at = e.call(FLOAT_SETTING_VALUE, &args![setting]).u32();
        if x87_load32(e.mem.u32(at)) == e.global::<f64>(ZERO) {
            e.call(FLOAT_SETTING_SET, &args![setting, 1.0f32]);
        }
    }
    true
}

// Translated from 0050e4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: when `0046f070` on `this` returns a non-zero byte, passes
/// the five stack words on to `0050f670` on `this` (the third as its byte,
/// `MOVZX`) and returns its byte; otherwise returns false. `RET 0x14`.
pub fn fn_0050e4b0(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    flag: u8,
    fourth: u32,
    fifth: u32,
) -> bool {
    if !e.call(0x0046_f070, &args![this]).bool() {
        return false;
    }
    e.call(
        0x0050_f670,
        &args![this, first, second, u32::from(flag), fourth, fifth],
    )
    .bool()
}

// Translated from 0050e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a save-size sum): the 16-bit sum of `00484bf0` on `this`
/// and `0048e9b0` on `this + 0x8C`, each given `argument`. `RET 4`.
pub fn fn_0050e4f0(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let mut size: u16 = 0;
    size = size.wrapping_add(e.call(0x0048_4bf0, &args![this, argument]).u16());
    let component = this.addr().wrapping_add(0x8c);
    size.wrapping_add(e.call(0x0048_e9b0, &args![component, argument]).u16())
}

// Translated from 0050e550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484c20` on `this`, then `0048e9e0` on `this + 0x8C`,
/// each given `argument`. `RET 4`.
pub fn fn_0050e550(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4c20, &args![this, argument]);
    let component = this.addr().wrapping_add(0x8c);
    e.call(0x0048_e9e0, &args![component, argument]);
}

// Translated from 0050e580 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484c50` on `this`, then `0048ea10` on `this + 0x8C`,
/// each given both stack words. `RET 8`.
pub fn fn_0050e580(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    e.call(0x0048_4c50, &args![this, first, second]);
    let component = this.addr().wrapping_add(0x8c);
    e.call(0x0048_ea10, &args![component, first, second]);
}

// Translated from 0050e5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484d60` on `this`, then `0048ea40` on `this + 0x8C`,
/// each given `argument`. `RET 4`.
pub fn fn_0050e5c0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4d60, &args![this, argument]);
    let component = this.addr().wrapping_add(0x8c);
    e.call(0x0048_ea40, &args![component, argument]);
}

// Translated from 0050e5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: `00484da0` on `this`, then `0048ea80` on `this + 0x8C`,
/// each given `argument`. `RET 4`.
pub fn fn_0050e5f0(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(0x0048_4da0, &args![this, argument]);
    let component = this.addr().wrapping_add(0x8c);
    e.call(0x0048_ea80, &args![component, argument]);
}

// Translated from 0050e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8C (`SUB ECX,0x8c`) and jumps to
/// `0050e4f0`, passing its stack word on.
pub fn fn_0050e680(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    fn_0050e4f0(e, Ptr::new(this.addr().wrapping_sub(0x8c)), argument)
}

// Translated from 0050e690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x3C (`SUB ECX,0x3c`) and jumps to the
/// scalar deleting destructor `0050c840`, passing its stack word (the
/// flags) on; returns what it returns.
pub fn fn_0050e690(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    let base = this.addr().wrapping_sub(0x3c);
    e.call(0x0050_c840, &args![base, flags]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_e4b0, fn_0050e4b0(Ptr, u32, u32, u8, u32, u32) -> bool),
        entry!(0x0050_e4f0, fn_0050e4f0(Ptr, u32) -> u16),
        entry!(0x0050_e550, fn_0050e550(Ptr, u32)),
        entry!(0x0050_e580, fn_0050e580(Ptr, u32, u32)),
        entry!(0x0050_e5c0, fn_0050e5c0(Ptr, u32)),
        entry!(0x0050_e5f0, fn_0050e5f0(Ptr, u32)),
        entry!(0x0050_e680, fn_0050e680(Ptr, u32) -> u16),
        entry!(0x0050_e690, fn_0050e690(Ptr, u32) -> u32),
        entry!(0x0050_e6a0, fn_0050e6a0(Ptr, u32, u32)),
        entry!(0x0050_e6b0, fn_0050e6b0(Ptr, u32)),
        entry!(0x0050de20, fn_0050de20(Ptr, u32, u32) -> bool),
        entry!(0x0050de60, fn_0050de60(Ptr, u32, u32, u32) -> bool),
        entry!(0x0050e620, fn_0050e620(Ptr, u8) -> bool),
        entry!(0x0050e640, fn_0050e640(Ptr, u8) -> bool),
        entry!(0x0050e6c0, fn_0050e6c0() -> bool),
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
    fn le81_thunk_0050e6a0() {
        check(0x0050_e6a0, 0x0050_e580, 0x8c, &[0x77, 0x88]);
    }

    #[test]
    fn le81_thunk_0050e6b0() {
        check(0x0050_e6b0, 0x0050_e550, 0x8c, &[0x77]);
    }
}

#[cfg(test)]
mod light_update_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn no_op(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// The constants the update reads, with the exe's values.
    fn constants(e: &mut Engine) {
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_6000,
            0x0101_8000,
            0x0101_e000,
            0x0101_f000,
            0x0102_0000,
            0x0102_1000,
            0x0102_9000,
            0x011c_8000,
            0x011c_9000,
        ] {
            e.map(page, 0x1000);
        }
        for (addr, value) in [
            (TWO, 0x4000_0000_0000_0000u64),
            (RANDOM_SCALE_ONE, 0x41b5_5555_6000_0000),
            (PHASE_ONE_STEP, 0x3ff1_9999_a000_0000),
            (PHASE_TWO_STEP, 0x3ff3_3333_4000_0000),
            (RANDOM_SCALE_THREE, 0x41ac_71c7_2000_0000),
            (PHASE_THREE_STEP, 0x3ff4_cccc_c000_0000),
            (FULL_TURN, 0x4019_21fb_6000_0000),
            (ONE_POINT_SEVEN, 0x3ffb_3333_4000_0000),
            (HALF, 0x3fe0_0000_0000_0000),
            (ZERO_POINT_THREE, 0x3fd3_3333_4000_0000),
            (TWO_POINT_SIX, 0x4004_cccc_c000_0000),
            (ZERO, 0),
            (THREE, 0x4008_0000_0000_0000),
            (FIVE, 0x4014_0000_0000_0000),
            (ONE, 0x3ff0_0000_0000_0000),
            (ZERO_POINT_FOUR, 0x3fd9_9999_a000_0000),
            (ZERO_POINT_SIX, 0x3fe3_3333_4000_0000),
            (ONE_HUNDREDTH, 0x3f84_7ae1_4000_0000),
            (QUARTER, 0x3fd0_0000_0000_0000),
        ] {
            e.mem.set_u64(addr, value);
        }
        e.mem.set_u32(MINUS_ONE_F, 0xbf80_0000);
        e.mem.set_u32(DIM_STEP, 0x3dcc_cccd);
        e.mem.set_u32(SLOW_DIM_STEP, 0x3d4c_cccd);
    }

    fn c(e: &Engine, addr: u32) -> f64 {
        e.global(addr)
    }

    /// A light with `flags` and `fFade`, and the doubles every path uses.
    fn light(e: &mut Engine, flags: u32, fade: f32) -> Ptr {
        constants(e);
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + LIGHT_FLAGS, flags);
        e.mem.set_f32(this + LIGHT_FADE, fade);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_DESTRUCT, no_op);
        e.register(SECONDS_PASSED, |_, _| float_ret(0.5));
        Ptr::new(this)
    }

    #[test]
    fn at_0050de20_copies_the_pointer_and_runs_the_update() {
        let mut e = Engine::new();
        let this = light(&mut e, 0, 1.0);
        e.register(POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            a[0].into_ret()
        });
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, 0x7000);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0050_de20, &args![this, holder, 0u32]).bool());
        let log = e.call_log.take().unwrap();
        let copy = &calls(&log, POINTER_COPY)[0];
        assert_eq!(copy[1], holder);
        // The update's argument slot holds the copied pointer.
        let got = calls(&log, POINTER_GET);
        assert_eq!(got.len(), 1);
        assert_eq!(calls(&log, POINTER_DESTRUCT).len(), 1);
    }

    /// The flicker doubles: phases from the node, a fixed random number,
    /// waves answering 0.5, the flicker setting 2.0, clamps as min/max.
    fn flicker(e: &mut Engine, node_phases: [f32; 3], random: u32, managed: bool) {
        let [one, two, three] = node_phases;
        e.register_double(PHASE_ONE_GET, move |_, _| float_ret(f64::from(one)));
        e.register_double(PHASE_TWO_GET, move |_, _| float_ret(f64::from(two)));
        e.register_double(PHASE_THREE_GET, move |_, _| float_ret(f64::from(three)));
        for addr in [
            PHASE_ONE_SET,
            PHASE_TWO_SET,
            PHASE_THREE_SET,
            SET_COLOR,
            MANAGER_UPDATE,
            POINT_CONSTRUCT,
            SET_POINT,
            DIMMER_SET,
        ] {
            e.register(addr, no_op);
        }
        e.register_double(RANDOM, move |_, _| random.into_ret());
        e.register(WAVE, |_, _| float_ret(0.5));
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, 2.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| setting.into_ret());
        e.register_double(MANAGED, move |_, _| managed.into_ret());
        e.register(MANAGER, |_, _| 0x0a00u32.into_ret());
        e.register(CLAMP_HIGH, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(CLAMP_LOW, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
    }

    fn update(e: &mut Engine, this: Ptr, node: u32, fade: u32, owner: u32) -> (bool, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(0x0050_de60, &args![this, node, fade, owner]).bool();
        (r, e.call_log.take().unwrap())
    }

    #[test]
    fn flicker_advances_the_phases_and_sets_colour_and_dimmer() {
        let mut e = Engine::new();
        let this = light(&mut e, 0x8, 2.0);
        flicker(&mut e, [0.25, 0.5, 6.2], 0, false);
        let (finished, log) = update(&mut e, this, 0x7000, 0, 0);
        assert!(!finished);
        // With random 0 each phase moves by its step times the frame time.
        let step = 0.5f64;
        let one = (c(&e, PHASE_ONE_STEP) * step + 0.25) as f32;
        let two = (c(&e, PHASE_TWO_STEP) * step + 0.5) as f32;
        let three = (c(&e, PHASE_THREE_STEP) * step + f64::from(6.2f32)) as f32;
        let three = (f64::from(three) - c(&e, FULL_TURN)) as f32;
        assert_eq!(calls(&log, PHASE_ONE_SET), vec![vec![0x7000, bits(one)]]);
        assert_eq!(calls(&log, PHASE_TWO_SET), vec![vec![0x7000, bits(two)]]);
        assert_eq!(
            calls(&log, PHASE_THREE_SET),
            vec![vec![0x7000, bits(three)]]
        );
        // Level 2.0, waves 0.5: swing 0.25, kept; colour 0.125 each.
        let colour = bits(0.125);
        assert_eq!(
            calls(&log, SET_COLOR),
            vec![vec![0x7000, colour, colour, colour]]
        );
        // Not managed: a zero point is set.
        let point = &calls(&log, POINT_CONSTRUCT)[0];
        assert_eq!(point[1..], [0, 0, 0]);
        assert_eq!(calls(&log, SET_POINT), vec![vec![0x7000, point[0]]]);
        assert!(calls(&log, MANAGER_UPDATE).is_empty());
        // The dimmer from the three waves.
        let mixed = ((((0.5f64 + 1.0) * ((0.5 + 1.0) / 2.0)) / 2.0) / 3.0 + 0.5 / 5.0) as f32;
        let mix = (f64::from(mixed) * 0.5 + 0.5) as f32;
        let dimmer =
            ((f64::from(mix) * c(&e, ZERO_POINT_FOUR) + c(&e, ZERO_POINT_SIX)) * 2.0) as f32;
        assert_eq!(calls(&log, DIMMER_SET), vec![vec![0x7000, bits(dimmer)]]);
        assert_eq!(calls(&log, CLAMP_HIGH)[0], vec![bits(1.0), bits(mixed)]);
        assert_eq!(calls(&log, CLAMP_LOW)[0], vec![bits(-1.0), bits(mixed)]);
        assert_eq!(calls(&log, POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn slow_flicker_halves_the_step_and_a_managed_light_updates_through_the_manager() {
        let mut e = Engine::new();
        let this = light(&mut e, 0x40, 1.0);
        flicker(&mut e, [0.0, 0.0, 0.0], 0, true);
        let (_, log) = update(&mut e, this, 0x7000, 0, 0);
        let one = (c(&e, PHASE_ONE_STEP) * 0.25) as f32;
        assert_eq!(calls(&log, PHASE_ONE_SET), vec![vec![0x7000, bits(one)]]);
        assert_eq!(calls(&log, MANAGER_UPDATE), vec![vec![0x0a00, 0x7000]]);
        assert!(calls(&log, SET_POINT).is_empty());
    }

    #[test]
    fn flicker_without_a_node_starts_from_fixed_phases() {
        let mut e = Engine::new();
        let this = light(&mut e, 0x8, 1.0);
        flicker(&mut e, [9.0, 9.0, 9.0], 0x4000_0000, false);
        // A negative level total keeps 0: colour 0.
        let setting = e.mem.alloc(4);
        e.mem.set_f32(setting, -2.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, _| setting.into_ret());
        let (_, log) = update(&mut e, this, 0, 0, 0);
        assert!(calls(&log, PHASE_ONE_GET).is_empty());
        assert!(calls(&log, PHASE_ONE_SET).is_empty());
        let zero = bits(0.0);
        assert_eq!(calls(&log, SET_COLOR), vec![vec![0, zero, zero, zero]]);
        assert_eq!(calls(&log, RANDOM).len(), 3);
    }

    /// The pulse doubles: the node's dimmer, a fixed distance function.
    fn pulse(e: &mut Engine, dimmer: f32) {
        e.register_double(DIMMER_GET, move |_, _| float_ret(f64::from(dimmer)));
        e.register(DIMMER_SET, no_op);
        e.register(ABSOLUTE, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0]).abs()))
        });
    }

    #[test]
    fn pulse_moves_the_dimmer_toward_the_target() {
        let mut e = Engine::new();
        let this = light(&mut e, 0x80, 2.0);
        pulse(&mut e, 0.5);
        let fade = e.mem.alloc(4);
        e.mem.set_f32(fade, 1.0);
        let (finished, log) = update(&mut e, this, 0x7000, fade, 0);
        assert!(!finished);
        let step = f32::from_bits(0x3dcc_cccd);
        let up = (f64::from(step) * 0.5 + 0.5) as f32;
        assert_eq!(calls(&log, DIMMER_SET), vec![vec![0x7000, bits(up)]]);
        // Far from the target: unchanged target.
        assert_eq!(e.mem.f32(fade), 1.0);
        // Above the target: moves down.
        pulse(&mut e, 1.5);
        let (_, log) = update(&mut e, this, 0x7000, fade, 0);
        let down = (1.5 - f64::from(step) * 0.5) as f32;
        assert_eq!(calls(&log, DIMMER_SET), vec![vec![0x7000, bits(down)]]);
    }

    #[test]
    fn pulse_near_the_target_picks_the_next_one() {
        let mut e = Engine::new();
        let this = light(&mut e, 0x100, 2.0);
        // Slow pulse: step 0.05; 0.98 + 0.025 is within 0.05 of 1.0.
        pulse(&mut e, 0.98);
        let fade = e.mem.alloc(4);
        e.mem.set_f32(fade, 1.0);
        let (_, log) = update(&mut e, this, 0x7000, fade, 0);
        let step = f32::from_bits(0x3d4c_cccd);
        let up = (f64::from(step) * 0.5 + f64::from(0.98f32)) as f32;
        assert_eq!(calls(&log, DIMMER_SET), vec![vec![0x7000, bits(up)]]);
        // Target above 0.5: a quarter of fFade.
        assert_eq!(e.mem.f32(fade), 0.5);
        // Target at most 0.5: fFade itself.
        pulse(&mut e, 0.5);
        e.mem.set_f32(fade, 0.5);
        update(&mut e, this, 0x7000, fade, 0);
        assert_eq!(e.mem.f32(fade), 2.0);
    }

    #[test]
    fn an_owner_at_zero_finishes_near_the_target() {
        let mut e = Engine::new();
        let this = light(&mut e, 0, 2.0);
        pulse(&mut e, 0.98);
        let fade = e.mem.alloc(4);
        e.mem.set_f32(fade, 1.0);
        let owner = e.mem.alloc(0x10);
        let (finished, _) = update(&mut e, this, 0x7000, fade, owner);
        assert!(finished);
        assert_eq!(e.mem.f32(fade), 1.0);
        // An owner whose value is not 0 (or NaN): nothing happens.
        for value in [1.0f32, f32::NAN] {
            e.mem.set_f32(owner + 0xc, value);
            let (finished, log) = update(&mut e, this, 0x7000, fade, owner);
            assert!(!finished);
            assert!(calls(&log, DIMMER_GET).is_empty());
        }
        // No owner and no pulse: nothing either.
        let (_, log) = update(&mut e, this, 0x7000, fade, 0);
        assert!(calls(&log, DIMMER_GET).is_empty());
    }

    #[test]
    fn at_0050e620_and_0050e640_compare_the_byte_at_plus_4() {
        let mut e = Engine::new();
        let p = e.mem.alloc(8);
        e.mem.set_u8(p + 4, 1);
        assert!(e.call(0x0050_e620, &args![p, 1u32]).bool());
        assert!(!e.call(0x0050_e640, &args![p, 2u32]).bool());
        assert!(e.call(0x0050_e640, &args![p, 0x101u32]).bool());
        assert!(!e.call(0x0050_e640, &args![0u32, 1u32]).bool());
    }

    #[test]
    fn at_0050e6c0_assembles_the_mode_and_repairs_the_settings() {
        let mut e = Engine::new();
        constants(&mut e);
        e.mem.set_u8(MODE_SETTING_TWO + 4, 1);
        let ints = e.mem.alloc(8);
        e.mem.set_u32(ints, 7);
        e.mem.set_u32(ints + 4, 2);
        e.register_double(INT_SETTING_VALUE, move |_, a| {
            (if a[0] == INT_SETTING_A {
                ints
            } else {
                ints + 4
            })
            .into_ret()
        });
        let floats = e.mem.alloc(8);
        e.mem.set_f32(floats, 0.0);
        e.mem.set_f32(floats + 4, 3.0);
        e.register_double(FLOAT_SETTING_VALUE, move |_, a| {
            (if a[0] == FLOAT_SETTING_A {
                floats
            } else {
                floats + 4
            })
            .into_ret()
        });
        e.register(INT_SETTING_SET, no_op);
        e.register(FLOAT_SETTING_SET, no_op);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_e6c0, &[]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(LIGHT_MODE_BITS), 2);
        assert_eq!(calls(&log, INT_SETTING_SET), vec![vec![INT_SETTING_A, 1]]);
        assert_eq!(
            calls(&log, FLOAT_SETTING_SET),
            vec![vec![FLOAT_SETTING_A, bits(1.0)]]
        );
        // The first setting is checked against 0, 1 and 2; the second
        // matches at 2.
        assert_eq!(calls(&log, INT_SETTING_VALUE).len(), 6);
        // No mode setting on: bit 2.
        e.mem.set_u8(MODE_SETTING_TWO + 4, 0);
        e.call(0x0050_e6c0, &[]);
        assert_eq!(e.mem.u8(LIGHT_MODE_BITS), 4);
        e.mem.set_u8(MODE_SETTING_ONE + 4, 1);
        e.mem.set_u8(MODE_SETTING_FOUR + 4, 1);
        e.call(0x0050_e6c0, &[]);
        assert_eq!(e.mem.u8(LIGHT_MODE_BITS), 5);
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
    fn fn_0050e4b0_forwards_only_when_the_check_passes() {
        let mut e = Engine::new();
        e.register(0x0046_f070, |e, a| Ret {
            eax: e.mem.u32(a[0]) | 0x100,
            ..Ret::default()
        });
        e.register(0x0050_f670, |_, _| Ret {
            eax: 0xffff_ff01,
            ..Ret::default()
        });
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let result = e.call(0x0050_e4b0, &[this, 1, 2, 0xffff_ff03, 4, 5]);
        assert!(!result.bool());
        assert!(calls_to(&mut e, &[0x0050_f670]).is_empty());
        e.mem.set_u32(this, 1);
        e.call_log = Some(vec![]);
        let result = e.call(0x0050_e4b0, &[this, 1, 2, 0xffff_ff03, 4, 5]);
        assert!(result.bool());
        assert_eq!(
            calls_to(&mut e, &[0x0050_f670]),
            vec![(0x0050_f670, vec![this, 1, 2, 3, 4, 5])]
        );
    }

    #[test]
    fn fn_0050e4f0_sums_both_sizes_in_16_bits() {
        let mut e = Engine::new();
        e.register(0x0048_4bf0, |_, _| Ret {
            eax: 0x1_fff0,
            ..Ret::default()
        });
        e.register(0x0048_e9b0, |_, _| Ret {
            eax: 0x20,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_e4f0, &[0x1000, 7]).u16(), 0x10);
        assert_eq!(
            calls_to(&mut e, &[0x0048_4bf0, 0x0048_e9b0]),
            vec![
                (0x0048_4bf0, vec![0x1000, 7]),
                (0x0048_e9b0, vec![0x108c, 7])
            ]
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_e680, &[0x108c, 9]).u16(), 0x10);
        assert_eq!(
            calls_to(&mut e, &[0x0048_4bf0, 0x0048_e9b0]),
            vec![
                (0x0048_4bf0, vec![0x1000, 9]),
                (0x0048_e9b0, vec![0x108c, 9])
            ]
        );
    }

    #[test]
    fn fn_0050e550_and_friends_run_form_then_component() {
        let cases: [(u32, u32, u32, &[u32]); 4] = [
            (0x0050_e550, 0x0048_4c20, 0x0048_e9e0, &[7]),
            (0x0050_e580, 0x0048_4c50, 0x0048_ea10, &[7, 8]),
            (0x0050_e5c0, 0x0048_4d60, 0x0048_ea40, &[7]),
            (0x0050_e5f0, 0x0048_4da0, 0x0048_ea80, &[7]),
        ];
        for (addr, form, component, extra) in cases {
            let mut e = Engine::new();
            e.register(form, |_, _| Ret::default());
            e.register(component, |_, _| Ret::default());
            let mut words = vec![0x1000];
            words.extend_from_slice(extra);
            e.call_log = Some(vec![]);
            e.call(addr, &words);
            let mut moved = vec![0x108c];
            moved.extend_from_slice(extra);
            assert_eq!(
                calls_to(&mut e, &[form, component]),
                vec![(form, words.clone()), (component, moved)]
            );
        }
    }

    #[test]
    fn fn_0050e690_moves_this_back_to_the_destructor() {
        let mut e = Engine::new();
        e.register(0x0050_c840, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0050_e690, &[0x203c, 1]).u32(), 0x2000);
        assert_eq!(
            calls_to(&mut e, &[0x0050_c840]),
            vec![(0x0050_c840, vec![0x2000, 1])]
        );
    }
}
