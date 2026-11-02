//! `fallout shared/run_00531720` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `00531720` and `00531d20` push the values of an image-space modifier's interpolators
//! (objects inside `this`: 21 pairs at `+0x114`, stride 0x30 and 0x18, then single ones from
//! `+0x504`) into the image-space manager (`004e3270`); the class names are not confirmed.
//! The others are small helpers of four-float values and clamps.

#[allow(unused_imports)]
use crate::prelude::*;

/// `004e3270` (`cdecl`, no argument; engine-map unit `teswater.cpp`): the manager the values go to.
const MANAGER: u32 = 0x004e_3270;
/// The float the modifier's time is measured from (`011c96fc`).
const CURRENT_TIME: u32 = 0x011c_96fc;
/// The upper clamp of the first single value in `00531720` (`01018f98`), the float both
/// halves of the point `00452dc0` builds (`01016248`).
const FIRST_VALUE_LIMIT: u32 = 0x0101_8f98;
const POINT_HALF: u32 = 0x0101_6248;
/// `00414430` (`thiscall`, `RET 0x10`): sets the four floats of a colour.
const COLOR_SET: u32 = 0x0041_4430;
/// `00452dc0` (`thiscall`, `RET 8`): sets a two-float point, answers it.
const POINT_SET: u32 = 0x0045_2dc0;
/// `NiAVObject::GetWorldBound` (engine map, `0043d450`, `thiscall`).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// `004a4f40` (`thiscall`, `RET 8`: the answer object and a float; engine-map unit
/// `bgsprimitive.cpp`): scales the object `this` holds into the answer.
const SCALE_INTO: u32 = 0x004a_4f40;
/// `004f5d90` (`thiscall`, `RET 4`, a float) and the float `005330a0` passes (`01021d84`).
const OBJECT_004F5D90: u32 = 0x004f_5d90;
const ARGUMENT_01021D84: u32 = 0x0102_1d84;
/// The address `00532ff0` answers (`011c9750`).
const OBJECT_011C9750: u32 = 0x011c_9750;
/// The manager calls of `00531720` (all `thiscall` on the manager, unit
/// `imagespacemanager.cpp`; the floats as their bits).
const SET_TABLE_VALUE: u32 = 0x00b8_cb30;
const SET_COLOR_A: u32 = 0x00b8_b540;
const SET_COLOR_B: u32 = 0x00b8_b610;
const SET_VALUE_A: u32 = 0x00b8_af10;
const SET_VALUE_B: u32 = 0x00b8_afb0;
const SET_POINT_GROUP: u32 = 0x00b8_b000;
const SET_BOUND_GROUP: u32 = 0x00b8_b760;
const SET_PAIR_GROUP: u32 = 0x00b8_b0d0;
const SET_PAIR_BOUND_GROUP: u32 = 0x00b8_b8d0;
const SET_VALUE_C: u32 = 0x00b8_af60;
/// The manager calls of `00531d20`.
const ADD_TABLE_VALUE: u32 = 0x00b8_cc20;
const ADD_COLOR_A: u32 = 0x00b8_b5c0;
const ADD_COLOR_B: u32 = 0x00b8_b690;
const ADD_VALUE_A: u32 = 0x00b8_af40;
const ADD_VALUE_B: u32 = 0x00b8_afe0;
const ADD_POINT_GROUP: u32 = 0x00b8_b6e0;
const ADD_PAIR_GROUP: u32 = 0x00b8_b130;
const ADD_VALUE_C: u32 = 0x00b8_af90;

/// The single interpolators after the pairs, and the frame slot (offset below `EBP`) each
/// one's value goes to, in the order the exe asks them.
const SINGLES: [(u32, u32); 11] = [
    (0x504, 0x04),
    (0x51c, 0x30),
    (0x57c, 0x2c),
    (0x594, 0x38),
    (0x5ac, 0x40),
    (0x5c4, 0x44),
    (0x5dc, 0x24),
    (0x5f4, 0x18),
    (0x60c, 0x28),
    (0x624, 0x34),
    (0x63c, 0x48),
];

/// The game's frame of the two pushers: `slot(n)` is `EBP - n`.
struct Frame(u32);

impl Frame {
    const SIZE: u32 = 0x90;
    fn slot(&self, below: u32) -> u32 {
        self.0 + Self::SIZE - below
    }
}

/// `value * scale` (`FLD; FMUL dword`), stored as a float and passed on (`FSTP; FLD; FSTP`).
fn scaled(e: &Engine, at: u32, scale: u32) -> u32 {
    x87_quiet_bits(x87_store32(x87_mul(
        x87_load32(e.mem.u32(at)),
        x87_m32(scale),
    )))
}

/// The modifier's position in its time: `(time(011c96fc) - start) / length(+0x24)` when the
/// byte at `+0x20` is set, else 0, stored as a float.
fn position(e: &Engine, this: u32, start: u32) -> u32 {
    if e.mem.u8(this + 0x20) == 0 {
        return x87_store32(0.0);
    }
    let elapsed = x87_sub(x87_load32(e.mem.u32(CURRENT_TIME)), x87_m32(start));
    x87_store32(x87_div(elapsed, x87_m32(e.mem.u32(this + 0x24))))
}

/// The 21 pairs: both of a pair are asked (slot `0x9c` with `(t, 0, &buffer[j])`), then the
/// buffer goes to the manager's `table` call by index range: 0-13 as kind 1, 14-16 as kind
/// 0 (index - 14), 17-20 as kind 3 (index - 17), with `scale`.
fn push_pairs(
    e: &mut Engine,
    this: u32,
    frame: &Frame,
    t: u32,
    manager: u32,
    scale: u32,
    table: u32,
) {
    let buffer = frame.slot(0x20);
    for index in 0..0x15u32 {
        for half in 0..2u32 {
            let object = this + 0x114 + index * 0x30 + half * 0x18;
            let t = x87_quiet_bits(t);
            e.vcall(object, 0x9c, &args![t, 0u32, buffer + 4 * half]);
        }
        let scale = x87_quiet_bits(scale);
        if index <= 0xd {
            e.call(table, &args![manager, 1u32, index, buffer, scale]);
        }
        if (0xe..=0x10).contains(&index) {
            e.call(table, &args![manager, 0u32, index - 0xe, buffer, scale]);
        }
        if (0x11..=0x14).contains(&index) {
            e.call(table, &args![manager, 3u32, index - 0x11, buffer, scale]);
        }
    }
}

// Translated from 005330e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the address of a float, then `low` and `high` as
/// their bits): clamps the float: set to `high` when `high` is below it, else to `low` when
/// `low` is above it (ordered compares; copies through the x87 stack).
pub fn fn_005330e0(e: &mut Engine, value: Ptr, low: u32, high: u32) {
    let at = value.addr();
    // FLD *v; FLD high; FCOMPP; TEST AH,5; JP: set when high < *v.
    if x87_load32(high) < x87_load32(e.mem.u32(at)) {
        e.mem.set_u32(at, x87_quiet_bits(high));
    } else if x87_load32(low) > x87_load32(e.mem.u32(at)) {
        // FLD *v; FLD low; FCOMPP; TEST AH,0x41; JNZ: set when low > *v.
        e.mem.set_u32(at, x87_quiet_bits(low));
    }
}

// Translated from 005321d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`, a float as its bits): multiplies the four
/// floats of `this` by `scale` in place (`FMUL dword`). Answers `this`.
pub fn fn_005321d0(e: &mut Engine, this: Ptr, scale: u32) -> Ptr {
    for word in 0..4 {
        let at = this.addr() + 4 * word;
        let product = x87_mul(x87_load32(e.mem.u32(at)), x87_m32(scale));
        e.mem.set_u32(at, x87_store32(product));
    }
    this
}

// Translated from 00532e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `answer = this + other` for four
/// floats (`this` copied first, each sum through the x87 stack). Answers `answer`.
pub fn fn_00532e40(e: &mut Engine, this: Ptr, answer: Ptr, other: Ptr) -> Ptr {
    let copy: [u32; 4] = std::array::from_fn(|word| e.mem.u32(this.addr() + 4 * word as u32));
    let sums: Vec<u32> = (0..4u32)
        .map(|word| {
            let other_word = e.mem.u32(other.addr() + 4 * word);
            x87_store32(x87_add(
                x87_load32(copy[word as usize]),
                x87_m32(other_word),
            ))
        })
        .collect();
    for (word, value) in sums.iter().enumerate() {
        e.mem.set_u32(answer.addr() + 4 * word as u32, *value);
    }
    answer
}

// Translated from 00532ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): `answer = this - other` for four
/// floats, as `00532e40` does the sum. Answers `answer`.
pub fn fn_00532ec0(e: &mut Engine, this: Ptr, answer: Ptr, other: Ptr) -> Ptr {
    let copy: [u32; 4] = std::array::from_fn(|word| e.mem.u32(this.addr() + 4 * word as u32));
    let differences: Vec<u32> = (0..4u32)
        .map(|word| {
            let other_word = e.mem.u32(other.addr() + 4 * word);
            x87_store32(x87_sub(
                x87_load32(copy[word as usize]),
                x87_m32(other_word),
            ))
        })
        .collect();
    for (word, value) in differences.iter().enumerate() {
        e.mem.set_u32(answer.addr() + 4 * word as u32, *value);
    }
    answer
}

// Translated from 00532f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: the answer and a float as its bits):
/// `answer = this * scale` for four floats (`this` copied first). Answers `answer`.
pub fn fn_00532f60(e: &mut Engine, this: Ptr, answer: Ptr, scale: u32) -> Ptr {
    let copy: [u32; 4] = std::array::from_fn(|word| e.mem.u32(this.addr() + 4 * word as u32));
    for (word, value) in copy.iter().enumerate() {
        let product = x87_store32(x87_mul(x87_load32(*value), x87_m32(scale)));
        e.mem.set_u32(answer.addr() + 4 * word as u32, product);
    }
    answer
}

// Translated from 00532f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the answer, a float as its bits, the value): the
/// `scale * value` operator, `00532f60` on `value` (the float copied through the x87
/// stack). Answers `answer`.
pub fn fn_00532f40(e: &mut Engine, answer: Ptr, scale: u32, value: Ptr) -> Ptr {
    fn_00532f60(e, value, answer, x87_quiet_bits(scale));
    answer
}

// Translated from 00532fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the answer, a float as its bits, the value): the
/// same operator for another class, `004a4f40` on `value`. Answers `answer`.
pub fn fn_00532fd0(e: &mut Engine, answer: Ptr, scale: u32, value: Ptr) -> Ptr {
    e.call(SCALE_INTO, &args![value, answer, x87_quiet_bits(scale)]);
    answer
}

// Translated from 00532ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no argument): the address `011c9750`.
pub fn fn_00532ff0(_e: &mut Engine) -> u32 {
    OBJECT_011C9750
}

// Translated from 005330a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the object's own slot `0x14`, the byte at
/// `+0x20` set, then `004f5d90` on the object at `+0x700` with the float at `01021d84`.
pub fn fn_005330a0(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x14, &[]);
    e.mem.set_u8(this.addr() + 0x20, 1);
    let value = x87_quiet_bits(e.mem.u32(ARGUMENT_01021D84));
    let target = e.mem.u32(this.addr() + 0x700);
    e.call(OBJECT_004F5D90, &args![target, value]);
}

// Translated from 00531720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: `start` and `scale` as float bits, an
/// object): sets the manager's values from the interpolators at position `t` (see
/// `position`, clamped to 0..1 with `005330e0`): the 21 pairs (`00b8cb30`), the colours at
/// `+0x534` and `+0x558` (slot `0x90`; alpha times `scale`, clamped to 0..1;
/// `00b8b540`/`00b8b610`), then the eleven single values. The first, times `scale` and
/// clamped to 0..`[01018f98]`, goes to `00b8af10`, the second times `scale` to `00b8afb0`.
/// The next group (times `scale` for the first only) goes with the object's world bound
/// (`00b8b760`, after the object's slot `0xbc`) when the byte at `+0xe8` is set and there
/// is an object, else with a point (`00452dc0` of the float at `01016248` twice; byte set)
/// or the point at `+0xec` (`00b8b000`). The pair group: with the byte at `+0x100` and an
/// object, `00b8b8d0` with the world bound; with the byte only, `00b8b0d0` with 0 and 1;
/// else `00b8b0d0` with two of the values; each with the byte at `+0x101`. The last value
/// goes to `00b8af60`.
pub fn fn_00531720(e: &mut Engine, this: Ptr, start: u32, scale: u32, object: Ptr) {
    let node = this.addr();
    e.with_stack(Frame::SIZE, |e, base| {
        let frame = Frame(base.addr());
        let t_at = frame.slot(0x3c);
        let t = position(e, node, start);
        e.mem.set_u32(t_at, t);
        fn_005330e0(e, Ptr::new(t_at), x87_store32(0.0), x87_store32(1.0));
        let manager = e.call(MANAGER, &args![]).u32();
        let t = e.mem.u32(t_at);
        push_pairs(e, node, &frame, t, manager, scale, SET_TABLE_VALUE);
        let color = frame.slot(0x14);
        let zero = x87_store32(0.0);
        e.call(COLOR_SET, &args![color, zero, zero, zero, zero]);
        for (offset, setter) in [(0x534, SET_COLOR_A), (0x558, SET_COLOR_B)] {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(node + offset, 0x90, &args![t, 0u32, color]);
            let alpha = frame.slot(8);
            let product = x87_mul(x87_load32(e.mem.u32(alpha)), x87_m32(scale));
            e.mem.set_u32(alpha, x87_store32(product));
            fn_005330e0(e, Ptr::new(alpha), x87_store32(0.0), x87_store32(1.0));
            e.call(setter, &args![manager, color]);
        }
        for (offset, slot) in SINGLES {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(node + offset, 0x9c, &args![t, 0u32, frame.slot(slot)]);
        }
        let value = |e: &Engine, slot: u32| x87_quiet_bits(e.mem.u32(frame.slot(slot)));
        let first = frame.slot(4);
        let product = x87_mul(x87_load32(e.mem.u32(first)), x87_m32(scale));
        e.mem.set_u32(first, x87_store32(product));
        let limit = x87_quiet_bits(e.mem.u32(FIRST_VALUE_LIMIT));
        fn_005330e0(e, Ptr::new(first), x87_store32(0.0), limit);
        let first = value(e, 4);
        e.call(SET_VALUE_A, &args![manager, first]);
        let second = scaled(e, frame.slot(0x30), scale);
        e.call(SET_VALUE_B, &args![manager, second]);
        let use_bound = e.mem.u8(node + 0xe8) != 0;
        if use_bound && !object.is_null() {
            e.vcall(object.addr(), 0xbc, &[]);
            let bound = e.call(GET_WORLD_BOUND, &args![object]).u32();
            let group = [
                scaled(e, frame.slot(0x2c), scale),
                value(e, 0x38),
                value(e, 0x40),
                value(e, 0x44),
                value(e, 0x24),
            ];
            e.call(
                SET_BOUND_GROUP,
                &args![manager, group[0], group[1], group[2], group[3], group[4], bound],
            );
        } else {
            let point = if use_bound {
                let half = x87_quiet_bits(e.mem.u32(POINT_HALF));
                e.call(POINT_SET, &args![frame.slot(0x5c), half, half])
                    .u32()
            } else {
                node + 0xec
            };
            let group = [
                scaled(e, frame.slot(0x2c), scale),
                value(e, 0x38),
                value(e, 0x40),
                value(e, 0x44),
                value(e, 0x24),
            ];
            e.call(
                SET_POINT_GROUP,
                &args![manager, group[0], group[1], group[2], group[3], group[4], point],
            );
        }
        let use_pair_bound = e.mem.u8(node + 0x100) != 0;
        let flag = u32::from(e.mem.u8(node + 0x101));
        if use_pair_bound && !object.is_null() {
            e.vcall(object.addr(), 0xbc, &[]);
            let bound = e.call(GET_WORLD_BOUND, &args![object]).u32();
            let pair = scaled(e, frame.slot(0x18), scale);
            e.call(
                SET_PAIR_BOUND_GROUP,
                &args![manager, pair, flag, zero, bound],
            );
        } else if use_pair_bound {
            let pair = scaled(e, frame.slot(0x18), scale);
            e.call(
                SET_PAIR_GROUP,
                &args![manager, pair, flag, zero, zero, x87_store32(1.0)],
            );
        } else {
            let (low, high) = (value(e, 0x28), value(e, 0x34));
            let pair = scaled(e, frame.slot(0x18), scale);
            e.call(SET_PAIR_GROUP, &args![manager, pair, flag, zero, low, high]);
        }
        let last = value(e, 0x48);
        e.call(SET_VALUE_C, &args![manager, last]);
    });
}

// Translated from 00531d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `start` and `scale` as float bits): adds
/// the interpolators' values at position `t` (not clamped) to the manager: the 21 pairs
/// (`00b8cc20`), the colours scaled by `scale` (`005321d0`; `00b8b5c0`/`00b8b690`), and the
/// eleven single values each times `scale`: `00b8af40`, `00b8afe0`, a group of five with
/// the point at `+0xec` scaled into a local (`004a4f40`; `00b8b6e0`), the pair group with
/// the byte at `+0x101` and 0 (`00b8b130`) and the last value (`00b8af90`).
pub fn fn_00531d20(e: &mut Engine, this: Ptr, start: u32, scale: u32) {
    let node = this.addr();
    e.with_stack(Frame::SIZE, |e, base| {
        let frame = Frame(base.addr());
        let t_at = frame.slot(0x3c);
        let t = position(e, node, start);
        e.mem.set_u32(t_at, t);
        let manager = e.call(MANAGER, &args![]).u32();
        push_pairs(e, node, &frame, t, manager, scale, ADD_TABLE_VALUE);
        let color = frame.slot(0x14);
        let zero = x87_store32(0.0);
        e.call(COLOR_SET, &args![color, zero, zero, zero, zero]);
        for (offset, adder) in [(0x534, ADD_COLOR_A), (0x558, ADD_COLOR_B)] {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(node + offset, 0x90, &args![t, 0u32, color]);
            fn_005321d0(e, Ptr::new(color), x87_quiet_bits(scale));
            e.call(adder, &args![manager, color]);
        }
        for (offset, slot) in SINGLES {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(node + offset, 0x9c, &args![t, 0u32, frame.slot(slot)]);
        }
        let first = scaled(e, frame.slot(4), scale);
        e.call(ADD_VALUE_A, &args![manager, first]);
        let second = scaled(e, frame.slot(0x30), scale);
        e.call(ADD_VALUE_B, &args![manager, second]);
        let point = e
            .call(
                SCALE_INTO,
                &args![node + 0xec, frame.slot(0x5c), x87_quiet_bits(scale)],
            )
            .u32();
        let group: Vec<u32> = [0x24u32, 0x44, 0x40, 0x38, 0x2c]
            .iter()
            .map(|slot| scaled(e, frame.slot(*slot), scale))
            .collect();
        e.call(
            ADD_POINT_GROUP,
            &args![manager, group[4], group[3], group[2], group[1], group[0], point],
        );
        let high = scaled(e, frame.slot(0x34), scale);
        let low = scaled(e, frame.slot(0x28), scale);
        let flag = u32::from(e.mem.u8(node + 0x101));
        let pair = scaled(e, frame.slot(0x18), scale);
        e.call(ADD_PAIR_GROUP, &args![manager, pair, flag, zero, low, high]);
        let last = scaled(e, frame.slot(0x48), scale);
        e.call(ADD_VALUE_C, &args![manager, last]);
    });
}

/// `00559450` (`thiscall` on the member at +0x10, no stack words): the
/// object `00533000` reads, or 0.
const MEMBER_00559450: u32 = 0x0055_9450;

// Translated from 00533000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three out pointers, `RET 0xc`):
/// when `00559450` on the member at +0x10 answers an object, asks it again
/// and answers `00533060` on it with the three pointers; otherwise clears
/// the two words and the byte and answers 0.
pub fn fn_00533000(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, flag: Ptr) -> u32 {
    let member = this.addr() + 0x10;
    if e.call(MEMBER_00559450, &args![member]).u32() == 0 {
        e.mem.set_u32(first.addr(), 0);
        e.mem.set_u32(second.addr(), 0);
        e.mem.set_u8(flag.addr(), 0);
        return 0;
    }
    let object = e.call(MEMBER_00559450, &args![member]).u32();
    fn_00533060(e, Ptr::new(object), first, second, flag)
}

// Translated from 00533060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three out pointers, `RET 0xc`):
/// stores the word at +8 in `first`, the word at +0x10 in `second` and the
/// byte at +0x14 in `flag`; answers the word at +0xC.
pub fn fn_00533060(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, flag: Ptr) -> u32 {
    let at = this.addr();
    let word = e.mem.u32(at + 8);
    e.mem.set_u32(first.addr(), word);
    let word = e.mem.u32(at + 0x10);
    e.mem.set_u32(second.addr(), word);
    let byte = e.mem.u8(at + 0x14);
    e.mem.set_u8(flag.addr(), byte);
    e.mem.u32(at + 0xc)
}

/// The manager calls only `00532220` makes (`thiscall` on the manager,
/// `imagespacemanager.cpp`, `RET 8`: the world bound and the address of a
/// point).
const SET_BOUND_POINT: u32 = 0x00b8_b1c0;
const SET_PAIR_BOUND_POINT: u32 = 0x00b8_bb70;
/// `006815c0` (`thiscall`, no stack word), run on the result point before
/// it is filled.
const POINT_PREPARE: u32 = 0x0068_15c0;
/// The two-float point operators (`thiscall`, `RET 8`: the answer and the
/// other point): `004e8880` (`this - other`, `teswater.cpp`) and `004dfe30`
/// (`this + other`).
const POINT2_SUBTRACT: u32 = 0x004e_8880;
const POINT2_ADD: u32 = 0x004d_fe30;
/// The `double` 0.5 (`01011588`) the position is compared with.
const HALF_DOUBLE: u32 = 0x0101_1588;
/// The single interpolators `00532220` blends, and the frame slot (below
/// `EBP`) each blended value goes to, in the exe's order.
const BLENDED_SINGLES: [(u32, u32); 8] = [
    (0x504, 0x0c),
    (0x51c, 0x68),
    (0x57c, 0x64),
    (0x594, 0x70),
    (0x5ac, 0x84),
    (0x5c4, 0x90),
    (0x5dc, 0x60),
    (0x5f4, 0x28),
];

/// `(theirs - mine) * position + mine` (`FLD; FSUB; FMUL; FADD; FSTP`, all
/// `float` memory operands), stored as a float.
fn blend(theirs: u32, mine: u32, position: u32) -> u32 {
    let difference = x87_sub(x87_load32(theirs), x87_m32(mine));
    x87_store32(x87_add(
        x87_mul(difference, x87_m32(position)),
        x87_m32(mine),
    ))
}

// Translated from 00532220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, seven words: the position, two
/// modifiers, the start and end strength as float bits, two objects): sets
/// the image-space manager's values (`004e3270`) to a blend of the two
/// modifiers' interpolators (each asked at time 0; a null modifier is the
/// object `00532ff0` answers). The strength is
/// `(end - start) * position + start`. The 21 pairs, blended, go to
/// `00b8cb30` with the strength; the colours at `+0x534` and `+0x558` are
/// blended (`00532ec0`, `00532f40`, `00532e40`), their alpha times the
/// strength clamped to 0..1, to `00b8b540`/`00b8b610`. Of the singles the
/// first eight and the last are blended, those at `+0x60c` and `+0x624`
/// are read from both. The first, times the strength and clamped to
/// 0..`[01018f98]`, goes to `00b8af10`; the second times the strength to
/// `00b8afb0`. Each modifier's point is its object's world bound
/// (`00b8b1c0`, after slot `0xbc` of the first object, also for the
/// second) when its byte at `+0xe8` is set and it has an object, else its
/// point at `+0xec` (a point of the float at `01016248` twice when the byte
/// is set without an object); the blended point goes to `00b8b000` with the
/// third single times the strength and four more. The pair point likewise
/// (`+0x100`, `00b8bb70`, slot `0xbc` of each object; else the
/// `+0x60c`/`+0x624` values of that modifier, or zero) goes to `00b8b0d0`
/// with the eighth single times the strength, the byte at `+0x101` of the
/// first modifier when the position is below 0.5 (else of the second,
/// also for a NaN position) and 0. The last value goes to `00b8af60`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00532220(
    e: &mut Engine,
    position: u32,
    first: u32,
    second: u32,
    start: u32,
    end: u32,
    first_object: Ptr,
    second_object: Ptr,
) {
    let mut first = first;
    if first == 0 {
        first = fn_00532ff0(e);
    }
    let mut second = second;
    if second == 0 {
        second = fn_00532ff0(e);
    }
    e.with_stack(0x170, |e, base| {
        let base = base.addr();
        let slot = |below: u32| base + 0x170 - below;
        let t_at = slot(0x74);
        e.mem.set_u32(t_at, x87_store32(0.0));
        let manager = e.call(MANAGER, &args![]).u32();
        let strength = x87_store32(x87_add(
            x87_mul(x87_sub(x87_load32(end), x87_m32(start)), x87_m32(position)),
            x87_m32(start),
        ));
        e.mem.set_u32(slot(0x6c), strength);
        let mine_at = slot(0xc0);
        let theirs_at = slot(0xbc);
        let buffer = slot(0x30);
        for index in 0..0x15u32 {
            for half in 0..2u32 {
                let offset = 0x114 + index * 0x30 + half * 0x18;
                let t = x87_quiet_bits(e.mem.u32(t_at));
                e.vcall(first + offset, 0x9c, &args![t, 0u32, mine_at]);
                let t = x87_quiet_bits(e.mem.u32(t_at));
                e.vcall(second + offset, 0x9c, &args![t, 0u32, theirs_at]);
                let value = blend(e.mem.u32(theirs_at), e.mem.u32(mine_at), position);
                e.mem.set_u32(buffer + 4 * half, value);
            }
            if index <= 0xd {
                let strength = x87_quiet_bits(e.mem.u32(slot(0x6c)));
                e.call(
                    SET_TABLE_VALUE,
                    &args![manager, 1u32, index, buffer, strength],
                );
            }
            if (0xe..=0x10).contains(&index) {
                let strength = x87_quiet_bits(e.mem.u32(slot(0x6c)));
                e.call(
                    SET_TABLE_VALUE,
                    &args![manager, 0u32, index - 0xe, buffer, strength],
                );
            }
            if (0x11..=0x14).contains(&index) {
                let strength = x87_quiet_bits(e.mem.u32(slot(0x6c)));
                e.call(
                    SET_TABLE_VALUE,
                    &args![manager, 3u32, index - 0x11, buffer, strength],
                );
            }
        }
        let zero = x87_store32(0.0);
        let color = slot(0x20);
        let mine_color = slot(0x54);
        let their_color = slot(0xb0);
        for at in [color, mine_color, their_color] {
            e.call(COLOR_SET, &args![at, zero, zero, zero, zero]);
        }
        for (offset, setter, temporaries) in [
            (0x534u32, SET_COLOR_A, [0xd0u32, 0xe0, 0xf0]),
            (0x558, SET_COLOR_B, [0x100, 0x110, 0x120]),
        ] {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(first + offset, 0x90, &args![t, 0u32, mine_color]);
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(second + offset, 0x90, &args![t, 0u32, their_color]);
            let difference = fn_00532ec0(
                e,
                Ptr::new(their_color),
                Ptr::new(slot(temporaries[0])),
                Ptr::new(mine_color),
            );
            let scaled_difference = fn_00532f40(
                e,
                Ptr::new(slot(temporaries[1])),
                x87_quiet_bits(position),
                difference,
            );
            let sum = fn_00532e40(
                e,
                Ptr::new(mine_color),
                Ptr::new(slot(temporaries[2])),
                scaled_difference,
            );
            for word in 0..4 {
                let value = e.mem.u32(sum.addr() + 4 * word);
                e.mem.set_u32(color + 4 * word, value);
            }
            let alpha = slot(0x14);
            let product = x87_mul(x87_load32(e.mem.u32(alpha)), x87_m32(e.mem.u32(slot(0x6c))));
            e.mem.set_u32(alpha, x87_store32(product));
            fn_005330e0(e, Ptr::new(alpha), x87_store32(0.0), x87_store32(1.0));
            e.call(setter, &args![manager, color]);
        }
        let mine_single = slot(0x94);
        let their_single = slot(0x10);
        let single = |e: &mut Engine, offset: u32, mine: u32, theirs: u32| {
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(first + offset, 0x9c, &args![t, 0u32, mine]);
            let t = x87_quiet_bits(e.mem.u32(t_at));
            e.vcall(second + offset, 0x9c, &args![t, 0u32, theirs]);
        };
        for (offset, target) in BLENDED_SINGLES {
            single(e, offset, mine_single, their_single);
            let value = blend(e.mem.u32(their_single), e.mem.u32(mine_single), position);
            e.mem.set_u32(slot(target), value);
        }
        single(e, 0x60c, slot(0x24), slot(0x80));
        single(e, 0x624, slot(0x44), slot(0x9c));
        single(e, 0x63c, mine_single, their_single);
        let value = blend(e.mem.u32(their_single), e.mem.u32(mine_single), position);
        e.mem.set_u32(slot(0xa0), value);
        let first_value = slot(0xc);
        let product = x87_mul(
            x87_load32(e.mem.u32(first_value)),
            x87_m32(e.mem.u32(slot(0x6c))),
        );
        e.mem.set_u32(first_value, x87_store32(product));
        let limit = x87_quiet_bits(e.mem.u32(FIRST_VALUE_LIMIT));
        fn_005330e0(e, Ptr::new(first_value), x87_store32(0.0), limit);
        let value = x87_quiet_bits(e.mem.u32(first_value));
        e.call(SET_VALUE_A, &args![manager, value]);
        let value = scaled(e, slot(0x68), e.mem.u32(slot(0x6c)));
        e.call(SET_VALUE_B, &args![manager, value]);
        // The blended point.
        let mine_point = slot(0x40);
        let their_point = slot(0x5c);
        for at in [mine_point, their_point] {
            let half = x87_quiet_bits(e.mem.u32(POINT_HALF));
            e.call(POINT_SET, &args![at, half, half]);
        }
        let point = slot(0x8c);
        e.call(POINT_PREPARE, &args![point]);
        if e.mem.u8(first + 0xe8) != 0 && !first_object.is_null() {
            e.vcall(first_object.addr(), 0xbc, &args![]);
            let bound = e.call(GET_WORLD_BOUND, &args![first_object]).u32();
            e.call(SET_BOUND_POINT, &args![manager, bound, mine_point]);
        }
        if e.mem.u8(second + 0xe8) != 0 && !second_object.is_null() {
            // The exe calls slot 0xbc of the first object here.
            e.vcall(first_object.addr(), 0xbc, &args![]);
            let bound = e.call(GET_WORLD_BOUND, &args![second_object]).u32();
            e.call(SET_BOUND_POINT, &args![manager, bound, their_point]);
        }
        for (modifier, at) in [(first, mine_point), (second, their_point)] {
            if e.mem.u8(modifier + 0xe8) == 0 {
                let x = e.mem.u32(modifier + 0xec);
                let y = e.mem.u32(modifier + 0xf0);
                e.mem.set_u32(at, x);
                e.mem.set_u32(at + 4, y);
            }
        }
        let difference = e
            .call(
                POINT2_SUBTRACT,
                &args![their_point, slot(0x128), mine_point],
            )
            .u32();
        let scaled_difference = fn_00532fd0(
            e,
            Ptr::new(slot(0x130)),
            x87_quiet_bits(position),
            Ptr::new(difference),
        );
        let sum = e
            .call(
                POINT2_ADD,
                &args![mine_point, slot(0x138), scaled_difference],
            )
            .u32();
        let x = e.mem.u32(sum);
        let y = e.mem.u32(sum + 4);
        e.mem.set_u32(point, x);
        e.mem.set_u32(point + 4, y);
        let group = [
            scaled(e, slot(0x64), e.mem.u32(slot(0x6c))),
            x87_quiet_bits(e.mem.u32(slot(0x70))),
            x87_quiet_bits(e.mem.u32(slot(0x84))),
            x87_quiet_bits(e.mem.u32(slot(0x90))),
            x87_quiet_bits(e.mem.u32(slot(0x60))),
        ];
        e.call(
            SET_POINT_GROUP,
            &args![manager, group[0], group[1], group[2], group[3], group[4], point],
        );
        // The blended pair point.
        let mine_pair = slot(0x38);
        let their_pair = slot(0x8);
        for at in [mine_pair, their_pair] {
            e.call(POINT_SET, &args![at, zero, zero]);
        }
        let pair = slot(0x7c);
        e.call(POINT_PREPARE, &args![pair]);
        if e.mem.u8(first + 0x100) != 0 && !first_object.is_null() {
            e.vcall(first_object.addr(), 0xbc, &args![]);
            let bound = e.call(GET_WORLD_BOUND, &args![first_object]).u32();
            e.call(SET_PAIR_BOUND_POINT, &args![manager, bound, mine_pair]);
        }
        if e.mem.u8(second + 0x100) != 0 && !second_object.is_null() {
            e.vcall(second_object.addr(), 0xbc, &args![]);
            let bound = e.call(GET_WORLD_BOUND, &args![second_object]).u32();
            e.call(SET_PAIR_BOUND_POINT, &args![manager, bound, their_pair]);
        }
        for (modifier, at, x_slot, y_slot, temporary) in [
            (first, mine_pair, 0x24u32, 0x44u32, 0x140u32),
            (second, their_pair, 0x80, 0x9c, 0x148),
        ] {
            if e.mem.u8(modifier + 0x100) == 0 {
                let y = x87_quiet_bits(e.mem.u32(slot(y_slot)));
                let x = x87_quiet_bits(e.mem.u32(slot(x_slot)));
                let made = e.call(POINT_SET, &args![slot(temporary), x, y]).u32();
                let x = e.mem.u32(made);
                let y = e.mem.u32(made + 4);
                e.mem.set_u32(at, x);
                e.mem.set_u32(at + 4, y);
            }
        }
        let difference = e
            .call(POINT2_SUBTRACT, &args![their_pair, slot(0x150), mine_pair])
            .u32();
        let scaled_difference = fn_00532fd0(
            e,
            Ptr::new(slot(0x158)),
            x87_quiet_bits(position),
            Ptr::new(difference),
        );
        let sum = e
            .call(
                POINT2_ADD,
                &args![mine_pair, slot(0x160), scaled_difference],
            )
            .u32();
        let x = e.mem.u32(sum);
        let y = e.mem.u32(sum + 4);
        e.mem.set_u32(pair, x);
        e.mem.set_u32(pair + 4, y);
        // FCOMP double [01011588]; TEST AH,5; JP: the second modifier's
        // byte unless the position is below 0.5 (ordered).
        let flag = if x87_load32(position) < e.global::<f64>(HALF_DOUBLE) {
            e.mem.u8(first + 0x101)
        } else {
            e.mem.u8(second + 0x101)
        };
        e.mem.set_u8(slot(0x169), flag);
        let value = scaled(e, slot(0x28), e.mem.u32(slot(0x6c)));
        let low = x87_quiet_bits(e.mem.u32(pair));
        let high = x87_quiet_bits(e.mem.u32(pair + 4));
        e.call(
            SET_PAIR_GROUP,
            &args![manager, value, u32::from(flag), zero, low, high],
        );
        let last = x87_quiet_bits(e.mem.u32(slot(0xa0)));
        e.call(SET_VALUE_C, &args![manager, last]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0053_30e0, fn_005330e0(Ptr, u32, u32)),
        entry!(0x0053_21d0, fn_005321d0(Ptr, u32) -> Ptr),
        entry!(0x0053_2e40, fn_00532e40(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0053_2ec0, fn_00532ec0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x0053_2f60, fn_00532f60(Ptr, Ptr, u32) -> Ptr),
        entry!(0x0053_2f40, fn_00532f40(Ptr, u32, Ptr) -> Ptr),
        entry!(0x0053_2fd0, fn_00532fd0(Ptr, u32, Ptr) -> Ptr),
        entry!(0x0053_2ff0, fn_00532ff0() -> u32),
        entry!(0x0053_30a0, fn_005330a0(Ptr)),
        entry!(0x0053_1720, fn_00531720(Ptr, u32, u32, Ptr)),
        entry!(0x0053_1d20, fn_00531d20(Ptr, u32, u32)),
        entry!(0x0053_3000, fn_00533000(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x0053_3060, fn_00533060(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x0053_2220, fn_00532220(u32, u32, u32, u32, u32, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len() as u32);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
        at
    }

    fn read(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn clamping() {
        let mut e = Engine::new();
        let at = floats(&mut e, &[5.0]);
        let (low, high) = (1.0f32.to_bits(), 2.0f32.to_bits());
        e.call(0x0053_30e0, &args![at, low, high]);
        assert_eq!(e.mem.f32(at), 2.0);
        e.mem.set_f32(at, -1.0);
        e.call(0x0053_30e0, &args![at, low, high]);
        assert_eq!(e.mem.f32(at), 1.0);
        e.mem.set_f32(at, 1.5);
        e.call(0x0053_30e0, &args![at, low, high]);
        assert_eq!(e.mem.f32(at), 1.5);
        e.mem.set_f32(at, f32::NAN);
        e.call(0x0053_30e0, &args![at, low, high]);
        assert!(e.mem.f32(at).is_nan());
    }

    #[test]
    fn four_float_arithmetic() {
        let mut e = Engine::new();
        let a = floats(&mut e, &[1.0, 2.0, 3.0, 4.0]);
        let b = floats(&mut e, &[0.5, 0.5, 1.0, 2.0]);
        let out = floats(&mut e, &[0.0; 4]);
        assert_eq!(e.call(0x0053_2e40, &args![a, out, b]).u32(), out);
        assert_eq!(read(&e, out, 4), vec![1.5, 2.5, 4.0, 6.0]);
        e.call(0x0053_2ec0, &args![a, out, b]);
        assert_eq!(read(&e, out, 4), vec![0.5, 1.5, 2.0, 2.0]);
        e.call(0x0053_2f60, &args![a, out, 2.0f32.to_bits()]);
        assert_eq!(read(&e, out, 4), vec![2.0, 4.0, 6.0, 8.0]);
        assert_eq!(
            e.call(0x0053_2f40, &args![out, 0.5f32.to_bits(), a]).u32(),
            out
        );
        assert_eq!(read(&e, out, 4), vec![0.5, 1.0, 1.5, 2.0]);
        assert_eq!(e.call(0x0053_21d0, &args![a, 3.0f32.to_bits()]).u32(), a);
        assert_eq!(read(&e, a, 4), vec![3.0, 6.0, 9.0, 12.0]);
        // The sum into the second operand itself.
        e.call(0x0053_2e40, &args![a, b, b]);
        assert_eq!(read(&e, b, 4), vec![3.5, 6.5, 10.0, 14.0]);
    }

    #[test]
    fn the_other_operator_and_the_small_ones() {
        let mut e = Engine::new();
        e.register(SCALE_INTO, |_, _| Ret::default());
        e.register(OBJECT_004F5D90, |_, _| Ret::default());
        e.register(0x0300_1000, |_, _| Ret::default());
        let mut slots = vec![0u32; 6];
        slots[5] = 0x0300_1000;
        e.put_vtable(0x0300_0000, &slots);
        e.map(0x0102_1000, 0x1000);
        e.mem.set_f32(ARGUMENT_01021D84, 0.75);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0053_2fd0, &args![0x6000u32, 2.0f32.to_bits(), 0x7000u32])
                .u32(),
            0x6000
        );
        assert_eq!(
            calls(&e, SCALE_INTO),
            vec![vec![0x7000, 0x6000, 2.0f32.to_bits()]]
        );
        assert_eq!(e.call(0x0053_2ff0, &args![]).u32(), OBJECT_011C9750);
        let this = e.mem.alloc(0x704);
        e.mem.set_u32(this, 0x0300_0000);
        e.mem.set_u32(this + 0x700, 0x8800);
        e.call(0x0053_30a0, &args![this]);
        assert_eq!(calls(&e, 0x0300_1000), vec![vec![this]]);
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(
            calls(&e, OBJECT_004F5D90),
            vec![vec![0x8800, 0.75f32.to_bits()]]
        );
    }

    const INTERPOLATOR_VTABLE: u32 = 0x0300_2000;
    const VALUE_SLOT: u32 = 0x0300_3000;
    const COLOR_SLOT: u32 = 0x0300_4000;
    const OBJECT_VTABLE: u32 = 0x0300_5000;
    const OBJECT_SLOT_BC: u32 = 0x0300_6000;

    /// A modifier whose interpolators all answer 2.0 (colours: alpha 2.0) and the engine
    /// with every manager call doubled.
    fn modifier(e: &mut Engine) -> u32 {
        e.map(0x011c_9000, 0x1000);
        e.map(0x0101_6000, 0x3000);
        e.map(0x0102_1000, 0x1000);
        e.mem.set_f32(CURRENT_TIME, 10.0);
        e.mem.set_f32(FIRST_VALUE_LIMIT, 7.0);
        e.mem.set_f32(POINT_HALF, 0.5);
        let mut slots = vec![0u32; 0x28];
        slots[0x90 / 4] = COLOR_SLOT;
        slots[0x9c / 4] = VALUE_SLOT;
        e.put_vtable(INTERPOLATOR_VTABLE, &slots);
        let mut object_slots = vec![0u32; 0x30];
        object_slots[0xbc / 4] = OBJECT_SLOT_BC;
        e.put_vtable(OBJECT_VTABLE, &object_slots);
        e.register(VALUE_SLOT, |e, a| {
            e.mem.set_f32(a[3], 2.0);
            Ret::default()
        });
        e.register(COLOR_SLOT, |e, a| {
            e.mem.set_f32(a[3] + 0xc, 2.0);
            Ret::default()
        });
        e.register(MANAGER, |_, _| 0x9900u32.into_ret());
        e.register(GET_WORLD_BOUND, |_, _| 0xb0b0u32.into_ret());
        e.register(POINT_SET, |_, a| a[0].into_ret());
        e.register(SCALE_INTO, |_, a| a[1].into_ret());
        for address in [
            COLOR_SET,
            OBJECT_SLOT_BC,
            SET_TABLE_VALUE,
            SET_COLOR_A,
            SET_COLOR_B,
            SET_VALUE_A,
            SET_VALUE_B,
            SET_POINT_GROUP,
            SET_BOUND_GROUP,
            SET_PAIR_GROUP,
            SET_PAIR_BOUND_GROUP,
            SET_VALUE_C,
            ADD_TABLE_VALUE,
            ADD_COLOR_A,
            ADD_COLOR_B,
            ADD_VALUE_A,
            ADD_VALUE_B,
            ADD_POINT_GROUP,
            ADD_PAIR_GROUP,
            ADD_VALUE_C,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x660);
        for index in 0..0x2a {
            e.mem
                .set_u32(this + 0x114 + 0x18 * index, INTERPOLATOR_VTABLE);
        }
        for offset in [0x534, 0x558] {
            e.mem.set_u32(this + offset, INTERPOLATOR_VTABLE);
        }
        for (offset, _) in SINGLES {
            e.mem.set_u32(this + offset, INTERPOLATOR_VTABLE);
        }
        e.mem.set_u8(this + 0x20, 1);
        e.mem.set_f32(this + 0x24, 4.0);
        e.mem.set_u8(this + 0x101, 1);
        e.call_log = Some(vec![]);
        this
    }

    /// Records the colour alpha each call of `address` sees.
    fn alpha_of(e: &mut Engine, address: u32) -> std::rc::Rc<std::cell::Cell<f32>> {
        let seen = std::rc::Rc::new(std::cell::Cell::new(0.0f32));
        let sink = seen.clone();
        e.register_double(address, move |e, a| {
            sink.set(e.mem.f32(a[1] + 0xc));
            Ret::default()
        });
        seen
    }

    #[test]
    fn setting_the_values() {
        let mut e = Engine::new();
        let this = modifier(&mut e);
        let alpha = alpha_of(&mut e, SET_COLOR_A);
        let (two, three, six) = (2.0f32.to_bits(), 3.0f32.to_bits(), 6.0f32.to_bits());
        // Position (10 - 8) / 4 = 0.5; scale 3.
        e.call(0x0053_1720, &args![this, 8.0f32.to_bits(), three, 0u32]);
        let t = 0.5f32.to_bits();
        let asked = calls(&e, VALUE_SLOT);
        assert_eq!(asked.len(), 42 + 11);
        assert!(asked.iter().all(|c| c[1] == t && c[2] == 0));
        let table = calls(&e, SET_TABLE_VALUE);
        assert_eq!(table.len(), 21);
        assert_eq!((table[0][1], table[0][2]), (1, 0));
        assert_eq!((table[14][1], table[14][2]), (0, 0));
        assert_eq!((table[20][1], table[20][2], table[20][4]), (3, 3, three));
        assert_eq!(calls(&e, SET_COLOR_A).len(), 1);
        // The colour alpha 2 * 3 clamped to 1.
        assert_eq!(alpha.get(), 1.0);
        // 2 * 3 = 6, below the limit 7.
        assert_eq!(calls(&e, SET_VALUE_A), vec![vec![0x9900, six]]);
        assert_eq!(calls(&e, SET_VALUE_B), vec![vec![0x9900, six]]);
        assert_eq!(
            calls(&e, SET_POINT_GROUP),
            vec![vec![0x9900, six, two, two, two, two, this + 0xec]]
        );
        assert_eq!(
            calls(&e, SET_PAIR_GROUP),
            vec![vec![0x9900, six, 1, 0, two, two]]
        );
        assert_eq!(calls(&e, SET_VALUE_C), vec![vec![0x9900, two]]);
    }

    #[test]
    fn setting_the_values_with_an_object_and_the_flags() {
        let mut e = Engine::new();
        let this = modifier(&mut e);
        e.mem.set_u8(this + 0xe8, 1);
        e.mem.set_u8(this + 0x100, 1);
        e.mem.set_u8(this + 0x20, 0);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, OBJECT_VTABLE);
        let (one, two, zero) = (1.0f32.to_bits(), 2.0f32.to_bits(), 0u32);
        e.call(0x0053_1720, &args![this, 8.0f32.to_bits(), one, object]);
        assert!(calls(&e, VALUE_SLOT).iter().all(|c| c[1] == zero));
        assert_eq!(calls(&e, OBJECT_SLOT_BC).len(), 2);
        assert_eq!(
            calls(&e, SET_BOUND_GROUP),
            vec![vec![0x9900, two, two, two, two, two, 0xb0b0]]
        );
        assert_eq!(
            calls(&e, SET_PAIR_BOUND_GROUP),
            vec![vec![0x9900, two, 1, zero, 0xb0b0]]
        );
        // Without the object: the point of two halves and the pair 0..1.
        e.call_log = Some(vec![]);
        e.call(0x0053_1720, &args![this, 8.0f32.to_bits(), one, 0u32]);
        let half = 0.5f32.to_bits();
        let point = calls(&e, POINT_SET);
        assert_eq!((point[0][1], point[0][2]), (half, half));
        assert_eq!(calls(&e, SET_POINT_GROUP)[0][6], point[0][0]);
        assert_eq!(
            calls(&e, SET_PAIR_GROUP),
            vec![vec![0x9900, two, 1, zero, zero, one]]
        );
    }

    #[test]
    fn adding_the_values() {
        let mut e = Engine::new();
        let this = modifier(&mut e);
        let alpha = alpha_of(&mut e, ADD_COLOR_A);
        let (four, half) = (4.0f32.to_bits(), 0.5f32.to_bits());
        e.call(
            0x0053_1d20,
            &args![this, 8.0f32.to_bits(), 2.0f32.to_bits()],
        );
        let asked = calls(&e, VALUE_SLOT);
        assert!(asked.iter().all(|c| c[1] == half));
        assert_eq!(calls(&e, ADD_TABLE_VALUE).len(), 21);
        assert_eq!(alpha.get(), 4.0);
        assert_eq!(calls(&e, ADD_VALUE_A), vec![vec![0x9900, four]]);
        assert_eq!(calls(&e, ADD_VALUE_B), vec![vec![0x9900, four]]);
        let scaled_point = calls(&e, SCALE_INTO);
        assert_eq!(scaled_point[0][0], this + 0xec);
        assert_eq!(
            calls(&e, ADD_POINT_GROUP),
            vec![vec![
                0x9900,
                four,
                four,
                four,
                four,
                four,
                scaled_point[0][1]
            ]]
        );
        assert_eq!(
            calls(&e, ADD_PAIR_GROUP),
            vec![vec![0x9900, four, 1, 0, four, four]]
        );
        assert_eq!(calls(&e, ADD_VALUE_C), vec![vec![0x9900, four]]);
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_00533000_reads_the_member_object_or_clears() {
        let mut e = Engine::new();
        e.register(MEMBER_00559450, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let outs = e.mem.alloc(0xc);
        e.mem.set_u32(outs, 0x1111);
        e.mem.set_u32(outs + 4, 0x2222);
        e.mem.set_u8(outs + 8, 0x33);
        assert_eq!(
            e.call(0x0053_3000, &[this, outs, outs + 4, outs + 8]).u32(),
            0
        );
        assert_eq!(e.mem.u32(outs), 0);
        assert_eq!(e.mem.u32(outs + 4), 0);
        assert_eq!(e.mem.u8(outs + 8), 0);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object + 8, 8);
        e.mem.set_u32(object + 0xc, 0xc);
        e.mem.set_u32(object + 0x10, 0x10);
        e.mem.set_u8(object + 0x14, 0x14);
        e.mem.set_u32(this + 0x10, object);
        assert_eq!(
            e.call(0x0053_3000, &[this, outs, outs + 4, outs + 8]).u32(),
            0xc
        );
        assert_eq!(e.mem.u32(outs), 8);
        assert_eq!(e.mem.u32(outs + 4), 0x10);
        assert_eq!(e.mem.u8(outs + 8), 0x14);
    }
}

#[cfg(test)]
mod n040_blend_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// The interpolators' vtable (`0x0300_2000`): slot 0x9c writes the
    /// float at +4 of the interpolator, slot 0x90 the four floats from +4.
    fn interpolators(e: &mut Engine) {
        let mut slots = vec![0u32; 0x28];
        slots[0x90 / 4] = 0x0300_2090;
        slots[0x9c / 4] = 0x0300_209c;
        e.put_vtable(0x0300_2000, &slots);
        e.register(0x0300_209c, |e, a| {
            assert_eq!(a[1..3], [0, 0]);
            let value = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[3], value);
            Ret::default()
        });
        e.register(0x0300_2090, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[0] + 4 + 4 * word);
                e.mem.set_u32(a[3] + 4 * word, value);
            }
            Ret::default()
        });
    }

    /// A modifier whose singles answer `single`, whose colours answer
    /// `color` and whose pairs answer `single` too.
    fn modifier(e: &mut Engine, single: f32, color: [f32; 4]) -> u32 {
        let this = e.mem.alloc(0x700);
        fill(e, this, single, color);
        this
    }

    /// Sets up the interpolators of the modifier at `this`.
    fn fill(e: &mut Engine, this: u32, single: f32, color: [f32; 4]) {
        let mut offsets: Vec<u32> = (0..0x2a).map(|i| 0x114 + 0x18 * i).collect();
        offsets.extend([
            0x504, 0x51c, 0x57c, 0x594, 0x5ac, 0x5c4, 0x5dc, 0x5f4, 0x60c, 0x624, 0x63c,
        ]);
        for offset in offsets {
            e.mem.set_u32(this + offset, 0x0300_2000);
            e.mem.set_f32(this + offset + 4, single);
        }
        for offset in [0x534, 0x558] {
            e.mem.set_u32(this + offset, 0x0300_2000);
            for (word, value) in color.iter().enumerate() {
                e.mem.set_f32(this + offset + 4 + 4 * word as u32, *value);
            }
        }
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    /// Doubles for everything outside the file; the manager calls record
    /// their words, with the floats they point at appended.
    fn setup(e: &mut Engine) -> Seen {
        interpolators(e);
        e.map(0x0101_1000, 0x1000);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_f32(POINT_HALF, 0.5);
        e.map(0x0101_8000, 0x1000);
        e.mem.set_f32(FIRST_VALUE_LIMIT, 3.0);
        e.register(MANAGER, |_, _| Ret {
            eax: 0x9000,
            ..Ret::default()
        });
        e.register(COLOR_SET, |e, a| {
            for word in 0..4 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            Ret::default()
        });
        e.register(POINT_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(POINT_PREPARE, |_, _| Ret::default());
        e.register(GET_WORLD_BOUND, |_, a| Ret {
            eax: a[0] + 0x40,
            ..Ret::default()
        });
        e.register(SCALE_INTO, |e, a| {
            let scale = f32::from_bits(a[2]);
            for word in 0..2 {
                let value = e.mem.f32(a[0] + 4 * word) * scale;
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            Ret::default()
        });
        for (address, sign) in [(POINT2_SUBTRACT, -1.0f32), (POINT2_ADD, 1.0)] {
            e.register_double(address, move |e, a| {
                for word in 0..2 {
                    let value = e.mem.f32(a[0] + 4 * word) + sign * e.mem.f32(a[2] + 4 * word);
                    e.mem.set_f32(a[1] + 4 * word, value);
                }
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
        }
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        for (address, pointer_word, count) in [
            (SET_TABLE_VALUE, 3usize, 2u32),
            (SET_COLOR_A, 1, 4),
            (SET_COLOR_B, 1, 4),
            (SET_VALUE_A, 9, 0),
            (SET_VALUE_B, 9, 0),
            (SET_POINT_GROUP, 6, 2),
            (SET_PAIR_GROUP, 9, 0),
            (SET_VALUE_C, 9, 0),
            (SET_BOUND_POINT, 9, 0),
            (SET_PAIR_BOUND_POINT, 9, 0),
        ] {
            let seen = seen.clone();
            e.register_double(address, move |e, a| {
                let mut words = a.to_vec();
                if let Some(at) = a.get(pointer_word) {
                    words.extend(floats(e, *at, count));
                }
                seen.borrow_mut().push((address, words));
                Ret::default()
            });
        }
        seen
    }

    #[test]
    fn fn_00532220_blends_two_modifiers() {
        let mut e = Engine::new();
        let seen = setup(&mut e);
        let first = modifier(&mut e, 1.0, [1.0, 1.0, 1.0, 0.25]);
        let second = modifier(&mut e, 3.0, [3.0, 3.0, 3.0, 0.75]);
        // The points at +0xec and the flags at +0x101.
        e.mem.set_f32(first + 0xec, 1.0);
        e.mem.set_f32(first + 0xf0, 1.0);
        e.mem.set_f32(second + 0xec, 3.0);
        e.mem.set_f32(second + 0xf0, 5.0);
        e.mem.set_u8(first + 0x101, 7);
        e.mem.set_u8(second + 0x101, 9);
        let half = 0.5f32.to_bits();
        let words = [
            half,
            first,
            second,
            1.0f32.to_bits(),
            3.0f32.to_bits(),
            0,
            0,
        ];
        e.call(0x0053_2220, &words);
        let seen = seen.borrow();
        let table: Vec<_> = seen.iter().filter(|(a, _)| *a == SET_TABLE_VALUE).collect();
        assert_eq!(table.len(), 21);
        let two = 2.0f32.to_bits();
        let four = 4.0f32.to_bits();
        assert_eq!(table[0].1[..3], [0x9000, 1, 0]);
        assert_eq!(table[0].1[4..], [two, two, two]);
        assert_eq!(table[14].1[..3], [0x9000, 0, 0]);
        assert_eq!(table[20].1[..3], [0x9000, 3, 3]);
        let rest: Vec<_> = seen.iter().filter(|(a, _)| *a != SET_TABLE_VALUE).collect();
        assert_eq!(rest[0].0, SET_COLOR_A);
        assert_eq!(rest[0].1[2..], bits(&[2.0, 2.0, 2.0, 1.0]));
        assert_eq!(rest[1].0, SET_COLOR_B);
        assert_eq!(rest[2], &(SET_VALUE_A, vec![0x9000, 3.0f32.to_bits()]));
        assert_eq!(rest[3], &(SET_VALUE_B, vec![0x9000, four]));
        assert_eq!(rest[4].0, SET_POINT_GROUP);
        assert_eq!(rest[4].1[1..6], [four, two, two, two, two]);
        assert_eq!(rest[4].1[7..], bits(&[2.0, 3.0]));
        assert_eq!(
            rest[5],
            &(SET_PAIR_GROUP, vec![0x9000, four, 9, 0, two, two])
        );
        assert_eq!(rest[6], &(SET_VALUE_C, vec![0x9000, two]));
        assert_eq!(rest.len(), 7);
    }

    #[test]
    fn fn_00532220_uses_the_bounds_and_the_first_flag_below_half() {
        let mut e = Engine::new();
        let seen = setup(&mut e);
        let first = modifier(&mut e, 1.0, [0.0; 4]);
        let second = modifier(&mut e, 1.0, [0.0; 4]);
        for modifier in [first, second] {
            e.mem.set_u8(modifier + 0xe8, 1);
            e.mem.set_u8(modifier + 0x100, 1);
        }
        e.mem.set_u8(first + 0x101, 7);
        let objects: Vec<u32> = (0..2).map(|_| e.mem.alloc(0x50)).collect();
        e.put_vtable(0x0300_3000, &[0x0300_30bc; 0x30]);
        for object in &objects {
            e.mem.set_u32(*object, 0x0300_3000);
        }
        e.register(0x0300_30bc, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let words = [
            0.25f32.to_bits(),
            first,
            second,
            1.0f32.to_bits(),
            1.0f32.to_bits(),
            objects[0],
            objects[1],
        ];
        e.call(0x0053_2220, &words);
        let slot_calls: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0300_30bc)
            .map(|(_, w)| w[0])
            .collect();
        // The first object's slot is called for both point bounds.
        assert_eq!(slot_calls, [objects[0], objects[0], objects[0], objects[1]]);
        let seen = seen.borrow();
        let bounds: Vec<_> = seen
            .iter()
            .filter(|(a, _)| [SET_BOUND_POINT, SET_PAIR_BOUND_POINT].contains(a))
            .map(|(a, w)| (*a, w[1]))
            .collect();
        assert_eq!(
            bounds,
            [
                (SET_BOUND_POINT, objects[0] + 0x40),
                (SET_BOUND_POINT, objects[1] + 0x40),
                (SET_PAIR_BOUND_POINT, objects[0] + 0x40),
                (SET_PAIR_BOUND_POINT, objects[1] + 0x40),
            ]
        );
        let pair = seen.iter().find(|(a, _)| *a == SET_PAIR_GROUP).unwrap();
        assert_eq!(pair.1[2], 7);
        // Both points stay the halves `00452dc0` set (the bound calls are doubles).
        let point = seen.iter().find(|(a, _)| *a == SET_POINT_GROUP).unwrap();
        assert_eq!(point.1[7..], bits(&[0.5, 0.5]));
    }

    #[test]
    fn fn_00532220_takes_the_default_modifier_for_null() {
        let mut e = Engine::new();
        setup(&mut e);
        e.map(0x011c_9000, 0x1000);
        fill(&mut e, OBJECT_011C9750, 5.0, [0.0; 4]);
        let other = modifier(&mut e, 1.0, [0.0; 4]);
        e.call_log = Some(vec![]);
        e.call(0x0053_2220, &[0, 0, other, 0, 0, 0, 0]);
        let log = e.call_log.take().unwrap();
        let asked: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == 0x0300_209c)
            .map(|(_, w)| w[0])
            .take(2)
            .collect();
        assert_eq!(asked, [OBJECT_011C9750 + 0x114, other + 0x114]);
    }
}
