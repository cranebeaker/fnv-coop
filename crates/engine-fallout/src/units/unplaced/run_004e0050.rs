//! `(unplaced)/run_004e0050` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A class tag made from `"SPD"` and freed again, and a per-frame update
//! that ages a list of nodes' timers (their classes are not confirmed).

#[allow(unused_imports)]
use crate::prelude::*;

/// The tag `004e0050` makes and `004e00d0` frees, and its string (`"SPD"`).
const SPD_TAG: u32 = 0x011c_7830;
const SPD_TAG_TEXT: u32 = 0x0102_2a18;
/// The byte that enables `004e0110`'s update.
const UPDATE_ENABLED: u32 = 0x011c_7834;
/// The pointer (read through `00559450`) to the object whose nodes
/// `004e0110` walks.
const NODE_OWNER: u32 = 0x011c_786c;
/// A running total the update lowers by an expired entry's word at `+0xC`.
const RUNNING_TOTAL: u32 = 0x011c_7828;
/// The frame timer object (`0084d030` on it gives the frame's seconds).
const FRAME_TIMER: u32 = 0x011f_6394;
/// Settings whose floats the update reads (`00403e20` gives the address).
const FADE_SETTING: u32 = 0x011c_fc2c;
const SETTING_A: u32 = 0x011c_eeb4;
const SETTING_B: u32 = 0x011c_f6a8;
/// `1.0` and `0.4` (`double`).
const ONE: u32 = 0x0101_2070;
const RATE_DIVISOR: u32 = 0x0102_28a0;

const FLOAT_ZERO: u32 = 0;
const FLOAT_ONE: u32 = 0x3f80_0000;

// Translated from 004e0050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004e0050 - the compiler's exception-unwinding frame (FS:[0]) is not translated
/// Makes the class tag at `011c7830`: allocates 4 bytes and, when that
/// succeeds, constructs them with `00438170` from `"SPD"`; stores the
/// result (or null).
pub fn fn_004e0050(e: &mut Engine) {
    let block = e.call(0x0040_1000, &args![4u32]).u32();
    let tag = if block == 0 {
        0
    } else {
        e.call(0x0043_8170, &args![block, SPD_TAG_TEXT]).u32()
    };
    e.set_global(SPD_TAG, tag);
}

// Translated from 004e00d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the class tag at `011c7830` when set (`004b9900` with flag 1) and
/// clears it.
pub fn fn_004e00d0(e: &mut Engine) {
    let tag: u32 = e.global(SPD_TAG);
    if tag != 0 {
        e.call(0x004b_9900, &args![tag, 1u32]);
    }
    e.set_global(SPD_TAG, 0u32);
}

/// The value of the `Setting` at `setting`, as `FLD dword` from the pointer
/// `00403e20` returns sees its bits.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(0x0040_3e20, &args![setting]).u32();
    e.mem.u32(value)
}

// Translated from 004e0110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-frame update enabled by the byte at `011c7834`: for every node
/// of the object at `011c786c` (`0043b480` count, `0043b4a0` node `i`), its
/// extra data (`NiObjectNET::GetExtraData` with the key `004e0320` gives)
/// holds a timer record (`0048d150`: elapsed, duration, a fade value, a
/// word). The frame's seconds (`0084d030`) are added to the elapsed time;
/// once it reaches the duration the record's word is taken off the running
/// total at `011c7828` and the node is removed from the owner (its vtable
/// +0xF0 with the index); otherwise, past the fraction of the duration in
/// the setting at `011cfc2c`, the fade value becomes `1 - 004b3ab0(0, 1,
/// setting * duration, duration, elapsed)`, and the elapsed time goes to
/// the node (`0043d410` builds a 12-byte value from it, `00a59c60` takes
/// it). Afterwards two settings are passed on (`004de8a0`, `004de8b0`), and
/// the level from `004de8d0`, when below 1, grows by `seconds / 0.4`
/// (limited by `0040ebd0` with 1.0) and is stored back (`004de8c0`).
pub fn fn_004e0110(e: &mut Engine) {
    if e.global::<u8>(UPDATE_ENABLED) == 0 {
        return;
    }
    let owner = e.call(0x0055_9450, &args![NODE_OWNER]).u32();
    let count = e.call(0x0043_b480, &args![owner]).u32();
    let seconds = x87_store32(e.call(0x0084_d030, &args![FRAME_TIMER]).st0);
    let mut index = 0u32;
    while index < count {
        let owner = e.call(0x0055_9450, &args![NODE_OWNER]).u32();
        let node = e.call(0x0043_b4a0, &args![owner, index]).u32();
        if node != 0 {
            let key = e.call(0x004e_0320, &[]).u32();
            let extra = e.call(0x00a5_bdd0, &args![node, key]).u32();
            let timer = e.call(0x0048_d150, &args![extra]).u32();
            let elapsed = x87_store32(x87_add(x87_load32(e.mem.u32(timer)), x87_m32(seconds)));
            e.mem.set_u32(timer, elapsed);
            let elapsed = x87_load32(e.mem.u32(timer));
            let duration = x87_load32(e.mem.u32(timer + 4));
            if duration <= elapsed {
                let word = e.mem.u32(timer + 0xc);
                let total: u32 = e.global(RUNNING_TOTAL);
                e.set_global(RUNNING_TOTAL, total.wrapping_sub(word));
                let owner = e.call(0x0055_9450, &args![NODE_OWNER]).u32();
                e.vcall(owner, 0xf0, &args![index]);
            } else {
                let ratio = x87_div(x87_load32(e.mem.u32(timer)), x87_m32(e.mem.u32(timer + 4)));
                let threshold = x87_load32(setting_bits(e, FADE_SETTING));
                if threshold <= ratio {
                    let elapsed = x87_quiet_bits(e.mem.u32(timer));
                    let duration = x87_quiet_bits(e.mem.u32(timer + 4));
                    let fraction = setting_bits(e, FADE_SETTING);
                    let start =
                        x87_store32(x87_mul(x87_load32(fraction), x87_m32(e.mem.u32(timer + 4))));
                    let curve = e
                        .call(
                            0x004b_3ab0,
                            &args![
                                FLOAT_ZERO,
                                FLOAT_ONE,
                                x87_quiet_bits(start),
                                duration,
                                elapsed
                            ],
                        )
                        .st0;
                    e.mem.set_u32(timer + 8, x87_store32(x87_sub(1.0, curve)));
                }
                let elapsed = x87_quiet_bits(e.mem.u32(timer));
                e.with_stack(0xc, |e, value| {
                    e.call(0x0043_d410, &args![value, elapsed, 1u32, 0u32]);
                    e.call(0x00a5_9c60, &args![node, value]);
                });
            }
        }
        index += 1;
    }
    let value = x87_quiet_bits(setting_bits(e, SETTING_A));
    e.call(0x004d_e8a0, &args![value]);
    let value = x87_quiet_bits(setting_bits(e, SETTING_B));
    e.call(0x004d_e8b0, &args![value]);
    let level = x87_store32(e.call(0x004d_e8d0, &[]).st0);
    if x87_load32(level) < e.global::<f64>(ONE) {
        let level = x87_store32(x87_add(
            x87_div(x87_load32(seconds), e.global::<f64>(RATE_DIVISOR)),
            x87_m32(level),
        ));
        let limited = x87_store32(
            e.call(0x0040_ebd0, &args![FLOAT_ONE, x87_quiet_bits(level)])
                .st0,
        );
        e.call(0x004d_e8c0, &args![x87_quiet_bits(limited)]);
    }
}

// Translated from 004e0320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): the word at `011c7830` (the tag `004e0050`
/// makes).
pub fn fn_004e0320(e: &mut Engine) -> u32 {
    e.global(SPD_TAG)
}

/// `00a7b440` (`ECX` = the new extra data, one stack word: the tag): the base constructor.
const EXTRA_BASE_CONSTRUCT: u32 = 0x00a7_b440;
/// The vtable [`fn_004e04e0`] stores.
const SPLATTER_DATA_VTABLE: u32 = 0x0102_2a24;
/// The setting whose value address `0043d4d0` answers: the splatter limit.
const LIMIT_SETTING: u32 = 0x011c_f2b8;
/// `0043d4d0` (`ECX` = a setting) and `00403e20` (`ECX` = a setting): its value's address.
const SETTING_VALUE: u32 = 0x0043_d4d0;
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// The setting whose float is the default duration.
const DURATION_SETTING: u32 = 0x011c_fd4c;
/// `0.0` (`double`), the duration threshold.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `"ScreenSplatterRoot"`.
const ROOT_NAME: u32 = 0x0102_28a8;
/// `0042f5a0` (`cdecl`, two words: a request, the room left), `004e08a0` (`cdecl`, one
/// word: the count): the count granted and the node made for it.
const GRANT_0042F5A0: u32 = 0x0042_f5a0;
const MAKE_NODE_004E08A0: u32 = 0x004e_08a0;
/// The allocator `00aa13e0` (`cdecl`, one word).
const ALLOCATE: u32 = 0x00aa_13e0;
/// `NiObjectNET::AddExtraData_ov2` (engine map, `00a5bca0`, `ECX`, one stack word).
const ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// `00438170` (`ECX` = a string local, one stack word: the text) and `004381b0` (its
/// destructor).
const STRING_CONSTRUCT: u32 = 0x0043_8170;
const STRING_DESTRUCT: u32 = 0x0043_81b0;
/// `00a5b950` (`ECX` = the node, one stack word: the name).
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
/// `00559450` (`ECX` = a pointer holder): its object.
const HOLDER_OBJECT: u32 = 0x0055_9450;
/// `007d6bb0` (`ECX` = the extra data) and `00b81420` (`cdecl`, two words).
const DATA_WORD_007D6BB0: u32 = 0x007d_6bb0;
const ATTACH_00B81420: u32 = 0x00b8_1420;
/// `0043d410` (`ECX` = a 12-byte local, three stack words) and `00a59c60` (`ECX` = the
/// root, one stack word: that local).
const UPDATE_DATA_0043D410: u32 = 0x0043_d410;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;

// Translated from 004e04e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x1c-byte extra data [`fn_004e0330`] makes (unnamed in the engine
/// map; `ECX` and one stack word, `RET 4`): the base constructor `00a7b440` with the tag
/// [`fn_004e0320`] answers, the vtable `01022a24`, then the four words at `values` into
/// +0xc..+0x18. Answers `this`.
pub fn fn_004e04e0(e: &mut Engine, this: Ptr, values: Ptr) -> Ptr {
    let tag = fn_004e0320(e);
    e.call(EXTRA_BASE_CONSTRUCT, &args![this, tag]);
    let base = this.addr();
    e.mem.set_u32(base, SPLATTER_DATA_VTABLE);
    for word in 0..4u32 {
        let value = e.mem.u32(values.addr() + 4 * word);
        e.mem.set_u32(base + 0xc + 4 * word, value);
    }
    this
}

// Translated from 004e0330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004e0330 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef2117, state variable around the constructor and the name string) and the frame's cookie word are not translated.
/// Adds a screen splatter (unnamed in the engine map; `cdecl`, two words: a request and a
/// `float` duration; called by `0088e8d0` and `005d8730`), when the byte at `011c7834` is
/// set: `0042f5a0` grants a count from the request and the room the limit setting leaves
/// over the running total at `011c7828`; a nonzero count makes a node (`004e08a0`), is
/// added to the total, and gets an extra data ([`fn_004e04e0`], 0x1c bytes, values
/// 0.0, the duration, 1.0 and the count; the duration is the setting at `011cfd4c` unless
/// `duration` is above the `double` 0.0, NaN not being above), the node's name
/// `"ScreenSplatterRoot"`, its attachment to the root object (vtable slot +0xdc with the
/// node and 1), `00b81420` with `007d6bb0`'s word of the extra data, and an update of the
/// root (`00a59c60` with a local set by `0043d410(0.0, 0, 0)`).
pub fn fn_004e0330(e: &mut Engine, request: u32, duration: u32) {
    if e.global::<u8>(UPDATE_ENABLED) == 0 {
        return;
    }
    let limit = e.call(SETTING_VALUE, &args![LIMIT_SETTING]).u32();
    let room = e
        .mem
        .u32(limit)
        .wrapping_sub(e.global::<u32>(RUNNING_TOTAL));
    let count = e.call(GRANT_0042F5A0, &args![request, room]).u32();
    if count == 0 {
        return;
    }
    let node = e.call(MAKE_NODE_004E08A0, &args![count]).u32();
    let total = e.global::<u32>(RUNNING_TOTAL).wrapping_add(count);
    e.set_global(RUNNING_TOTAL, total);
    // The frame: the four values at +0 (0x10 bytes), the name string at +0x10, the update
    // local at +0x14 (12 bytes).
    e.with_stack(0x20, |e, frame| {
        let values = frame.addr();
        let name = values + 0x10;
        let update = values + 0x14;
        e.mem.set_u32(values + 8, FLOAT_ONE);
        e.mem.set_u32(values, FLOAT_ZERO);
        let threshold: f64 = e.global(ZERO_DOUBLE);
        let value = x87_load32(duration);
        // `FCOMP`, `TEST AH,0x41`, `JNZ`: the given duration only when it is above.
        let seconds = if value > threshold {
            x87_quiet_bits(duration)
        } else {
            let address = e.call(SETTING_FLOAT, &args![DURATION_SETTING]).u32();
            x87_quiet_bits(e.mem.u32(address))
        };
        e.mem.set_u32(values + 4, seconds);
        e.mem.set_u32(values + 0xc, count);
        let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
        let data = if block == 0 {
            0
        } else {
            fn_004e04e0(e, Ptr::new(block), Ptr::new(values)).addr()
        };
        e.call(ADD_EXTRA_DATA, &args![node, data]);
        let text = e.call(STRING_CONSTRUCT, &args![name, ROOT_NAME]).u32();
        e.call(SET_NAME_00A5B950, &args![node, text]);
        e.call(STRING_DESTRUCT, &args![name]);
        let root = e.call(HOLDER_OBJECT, &args![NODE_OWNER]).u32();
        e.vcall(root, 0xdc, &args![node, 1u32]);
        let word = e.call(DATA_WORD_007D6BB0, &args![data]).u32();
        e.call(ATTACH_00B81420, &args![node, word]);
        e.call(UPDATE_DATA_0043D410, &args![update, FLOAT_ZERO, 0u32, 0u32]);
        let root = e.call(HOLDER_OBJECT, &args![NODE_OWNER]).u32();
        e.call(UPDATE_00A59C60, &args![root, update]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004e_04e0, fn_004e04e0(Ptr, Ptr) -> Ptr),
        entry!(0x004e_0330, fn_004e0330(u32, u32)),
        entry!(0x004e_0320, fn_004e0320() -> u32),
        entry!(0x004e_0050, fn_004e0050()),
        entry!(0x004e_00d0, fn_004e00d0()),
        entry!(0x004e_0110, fn_004e0110()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == target)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn spd_tag_is_made_and_freed() {
        let mut e = Engine::new();
        e.map(0x011c_7000, 0x1000);
        e.register(0x0040_1000, |_, _| 0x4000u32.into_ret());
        e.register(0x0043_8170, |_, a| {
            assert_eq!(a[1], SPD_TAG_TEXT);
            (a[0] + 8).into_ret()
        });
        e.register(0x004b_9900, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004e_0050, &[]);
        assert_eq!(e.global::<u32>(SPD_TAG), 0x4008);
        e.call(0x004e_00d0, &[]);
        e.call(0x004e_00d0, &[]);
        assert_eq!(calls_to(&e, 0x004b_9900), vec![vec![0x4008, 1]]);
        e.register(0x0040_1000, |_, _| 0u32.into_ret());
        e.call(0x004e_0050, &[]);
        assert_eq!(e.global::<u32>(SPD_TAG), 0);
    }

    /// The engine with doubles for the update: two nodes (the second
    /// null), timers in memory, settings at `+4`, and recording doubles.
    fn update_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0102_2000,
            0x011c_7000,
            0x011c_e000,
            0x011c_f000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE, 1.0f64);
        e.set_global(RATE_DIVISOR, f64::from_bits(0x3fd9_9999_9999_999a));
        e.set_global(UPDATE_ENABLED, 1u8);
        e.set_global(NODE_OWNER, 0x0011_0000u32);
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0043_b480, |_, _| 2u32.into_ret());
        e.register(0x0043_b4a0, |_, a| {
            if a[1] == 0 { 0x0022_0000u32 } else { 0 }.into_ret()
        });
        e.register(0x0084_d030, |_, _| 0.25f64.into_ret());
        e.register(0x004e_0320, |_, _| 0x77u32.into_ret());
        e.register(0x00a5_bdd0, |_, _| 0x0033_0000u32.into_ret());
        e.register(0x0040_3e20, |_, a| (a[0] + 4).into_ret());
        e.register(0x004b_3ab0, |_, _| 0.25f64.into_ret());
        for target in [
            0x0043_d410,
            0x00a5_9c60,
            0x004d_e8a0,
            0x004d_e8b0,
            0x004d_e8c0,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(0x004d_e8d0, |_, _| 0.5f64.into_ret());
        e.register(0x0040_ebd0, |_, a| {
            f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))).into_ret()
        });
        e
    }

    #[test]
    fn update_fades_a_running_timer() {
        let mut e = update_engine();
        let timer = e.mem.alloc(0x10);
        e.register_double(0x0048_d150, move |_, _| timer.into_ret());
        e.mem.set_f32(timer, 0.5);
        e.mem.set_f32(timer + 4, 1.0);
        e.mem.set_f32(FADE_SETTING + 4, 0.5);
        e.call_log = Some(vec![]);
        e.call(0x004e_0110, &[]);
        assert_eq!(e.mem.f32(timer), 0.75);
        assert_eq!(e.mem.f32(timer + 8), 0.75);
        assert_eq!(
            calls_to(&e, 0x004b_3ab0),
            vec![vec![
                0,
                FLOAT_ONE,
                0.5f32.to_bits(),
                1.0f32.to_bits(),
                0.75f32.to_bits()
            ]]
        );
        let built = calls_to(&e, 0x0043_d410);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1..], [0.75f32.to_bits(), 1, 0]);
        assert_eq!(calls_to(&e, 0x00a5_9c60)[0][0], 0x0022_0000);
        // The level grows by 0.25 / 0.4 and is limited to 1.
        assert_eq!(calls_to(&e, 0x004d_e8c0), vec![vec![FLOAT_ONE]]);
        assert_eq!(calls_to(&e, 0x0043_b4a0).len(), 2);
    }

    #[test]
    fn update_removes_an_expired_timer() {
        let mut e = update_engine();
        let timer = e.mem.alloc(0x10);
        e.register_double(0x0048_d150, move |_, _| timer.into_ret());
        e.mem.set_f32(timer, 0.9);
        e.mem.set_f32(timer + 4, 1.0);
        e.mem.set_u32(timer + 0xc, 5);
        e.set_global(RUNNING_TOTAL, 12u32);
        e.put_vtable(0x0044_0000, &[0; 0x3d]);
        e.mem.set_u32(0x0044_0000 + 0xf0, 0x0045_0000);
        e.register(0x0045_0000, |_, _| Ret::default());
        e.mem.map(0x0011_0000, 4);
        e.mem.set_u32(0x0011_0000, 0x0044_0000);
        e.register(0x004d_e8d0, |_, _| 1.0f64.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x004e_0110, &[]);
        assert_eq!(e.global::<u32>(RUNNING_TOTAL), 7);
        assert_eq!(calls_to(&e, 0x0045_0000), vec![vec![0x0011_0000, 0]]);
        assert!(calls_to(&e, 0x0043_d410).is_empty());
        // A level of 1 is left alone.
        assert!(calls_to(&e, 0x004d_e8c0).is_empty());
        // Disabled: nothing.
        e.set_global(UPDATE_ENABLED, 0u8);
        e.call_log = Some(vec![]);
        e.call(0x004e_0110, &[]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_004e0320_answers_the_word() {
        let mut e = Engine::new();
        e.map(0x011c_7000, 0x1000);
        e.set_global(SPD_TAG, 0x00c0_ffeeu32);
        assert_eq!(e.call(0x004e_0320, &args![]).u32(), 0x00c0_ffee);
    }
}

#[cfg(test)]
mod splatter_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, callee: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn fn_004e04e0_builds_the_extra_data() {
        let mut e = Engine::new();
        e.map(0x011c_7000, 0x1000);
        e.set_global(SPD_TAG, 0x5d00u32);
        e.register(EXTRA_BASE_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x1c);
        let values = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(values + 4 * i, 0x100 + i);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004e_04e0, &args![this, values]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, EXTRA_BASE_CONSTRUCT), vec![vec![this, 0x5d00]]);
        assert_eq!(e.mem.u32(this), SPLATTER_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x100);
        assert_eq!(e.mem.u32(this + 0x18), 0x103);
    }

    fn engine(enabled: u8, granted: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_7000, 0x1000);
        e.map(0x011c_f000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(UPDATE_ENABLED, enabled);
        e.set_global(RUNNING_TOTAL, 3u32);
        e.mem.set_u32(LIMIT_SETTING + 4, 10);
        e.mem.set_u32(DURATION_SETTING + 4, 2.5f32.to_bits());
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(SETTING_FLOAT, |_, a| (a[0] + 4).into_ret());
        e.register_double(GRANT_0042F5A0, move |_, _| granted.into_ret());
        e.register(MAKE_NODE_004E08A0, |_, _| 0x7000u32.into_ret());
        e.register(ALLOCATE, |e, _| e.mem.alloc(0x1c).into_ret());
        e.register(EXTRA_BASE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(STRING_CONSTRUCT, |_, _| 0x7100u32.into_ret());
        e.register(HOLDER_OBJECT, |_, _| 0x7200u32.into_ret());
        e.register(DATA_WORD_007D6BB0, |_, _| 0x7300u32.into_ret());
        e.register(0x0060_00dc, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x38];
        slots[0xdc / 4] = 0x0060_00dc;
        e.put_vtable(0x0070_0000, &slots);
        e.map(0x7200, 4);
        e.mem.set_u32(0x7200, 0x0070_0000);
        for callee in [
            ADD_EXTRA_DATA,
            SET_NAME_00A5B950,
            STRING_DESTRUCT,
            ATTACH_00B81420,
            UPDATE_DATA_0043D410,
            UPDATE_00A59C60,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_004e0330_adds_a_splatter() {
        let mut e = engine(1, 4);
        e.call(0x004e_0330, &args![9u32, 1.5f32.to_bits()]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GRANT_0042F5A0), vec![vec![9, 7]]);
        assert_eq!(e.global::<u32>(RUNNING_TOTAL), 7);
        let added = calls(&log, ADD_EXTRA_DATA);
        let data = added[0][1];
        assert_eq!(added[0][0], 0x7000);
        assert_eq!(e.mem.u32(data + 0xc), 0);
        assert_eq!(e.mem.u32(data + 0x10), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(data + 0x14), 1.0f32.to_bits());
        assert_eq!(e.mem.u32(data + 0x18), 4);
        assert_eq!(calls(&log, SET_NAME_00A5B950), vec![vec![0x7000, 0x7100]]);
        assert_eq!(calls(&log, 0x0060_00dc), vec![vec![0x7200, 0x7000, 1]]);
        assert_eq!(calls(&log, ATTACH_00B81420), vec![vec![0x7000, 0x7300]]);
        let update = calls(&log, UPDATE_DATA_0043D410);
        assert_eq!(update[0][1..], [0, 0, 0]);
        assert_eq!(
            calls(&log, UPDATE_00A59C60),
            vec![vec![0x7200, update[0][0]]]
        );
    }

    #[test]
    fn fn_004e0330_uses_the_setting_for_no_positive_duration() {
        for duration in [0.0f32, -1.0, f32::NAN] {
            let mut e = engine(1, 1);
            e.call(0x004e_0330, &args![9u32, duration.to_bits()]);
            let log = e.call_log.take().unwrap();
            let data = calls(&log, ADD_EXTRA_DATA)[0][1];
            assert_eq!(e.mem.u32(data + 0x10), 2.5f32.to_bits());
        }
    }

    #[test]
    fn fn_004e0330_does_nothing_disabled_or_without_room() {
        let mut e = engine(0, 4);
        e.call(0x004e_0330, &args![9u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let mut e = engine(1, 0);
        e.call(0x004e_0330, &args![9u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, MAKE_NODE_004E08A0).is_empty());
        assert_eq!(e.global::<u32>(RUNNING_TOTAL), 3);
    }
}
