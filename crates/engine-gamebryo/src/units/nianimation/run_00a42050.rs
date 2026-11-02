//! `NiAnimation/run_00a42050` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::set_literal_fixed_string;
use crate::units::nimain::nimainsdm::{release_fixed_string, GET_FIXED_STRING};

// Translated from 00a421c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (range inside the NiAnimation subsystem; reached by the jump
/// that ends `fn_00a2d8a0`; no arguments): sets the static fixed strings at `011f3ec0` and
/// `011f3ec4` from the literals at `010977f4` and `010977ec`, each only when the word does
/// not hold the literal's address already.
pub fn fn_00a421c0(e: &mut Engine) {
    set_literal_fixed_string(e, 0x011f_3ec0, 0x0109_77f4);
    set_literal_fixed_string(e, 0x011f_3ec4, 0x0109_77ec);
}

/// `2.0` (`double`, `01011590`) and `3.0` (`double`, `01021928`).
const TWO: u32 = 0x0101_1590;
const THREE: u32 = 0x0102_1928;

// Translated from 00a42050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three floats, result in ST0): the
/// linear blend `t * b + (1 - t) * a` of `a` and `b` by `t`, in x87 order
/// (`1 - t`, times `a`; `t * b`; added), rounded to a float.
pub fn fn_00a42050(_e: &mut Engine, t: f32, a: f32, b: f32) -> f64 {
    let t = x87_load32(t.to_bits());
    let one_minus_t = x87_sub(1.0, t);
    let from_a = x87_mul(one_minus_t, x87_m32(a.to_bits()));
    let from_b = x87_mul(t, x87_load32(b.to_bits()));
    x87_load32(x87_store32(x87_add(from_b, from_a)))
}

// Translated from 00a42070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, five floats, result in ST0): the
/// cubic Hermite value at `t` of the segment from `start` (tangent
/// `start_tangent`) to `end` (tangent `end_tangent`):
/// `start + t * (start_tangent + t * (c2 + t * c3))` with
/// `c3 = (start_tangent + end_tangent) - (end - start) * 2` and
/// `c2 = (end - start) * 3 - (end_tangent + 2 * start_tangent)`, each
/// coefficient rounded to a float (the exe keeps them in the `end` argument's
/// stack word), in the exe's x87 operation order.
pub fn fn_00a42070(
    e: &mut Engine,
    t: f32,
    start: f32,
    start_tangent: f32,
    end: f32,
    end_tangent: f32,
) -> f64 {
    let two = f64::from_bits(e.global::<u64>(TWO));
    let three = f64::from_bits(e.global::<u64>(THREE));
    let end_value = x87_load32(end.to_bits());
    let start_value = x87_load32(start.to_bits());
    let difference = x87_sub(end_value, start_value);
    let start_tangent = x87_load32(start_tangent.to_bits());
    let end_tangent = x87_load32(end_tangent.to_bits());
    let tangent_sum = x87_add(start_tangent, end_tangent);
    let loaded_two = x87_load64(two.to_bits());
    let twice_difference = x87_mul(difference, loaded_two);
    let cubic = x87_store32(x87_sub(tangent_sum, twice_difference));
    let cubic = x87_load32(cubic);
    let t = x87_load32(t.to_bits());
    let cubic_t = x87_mul(cubic, t);
    let thrice_difference = x87_mul(difference, three);
    let twice_start_tangent = x87_mul(loaded_two, start_tangent);
    let tangents = x87_add(end_tangent, twice_start_tangent);
    let quadratic = x87_store32(x87_sub(thrice_difference, tangents));
    let inner = x87_mul(x87_add(cubic_t, x87_m32(quadratic)), t);
    let linear = x87_add(start_tangent, inner);
    let scaled = x87_mul(t, linear);
    x87_load32(x87_store32(x87_add(scaled, start_value)))
}

// Translated from 00a42230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments; the counterpart of
/// `fn_00a421c0`): for each of the static fixed strings at `011f3ec0` and
/// `011f3ec4` that is set, stores the handle `00a5b690` gives for a null
/// string and releases the old handle (`InterlockedDecrement` of its count
/// at -8, unless it is the empty string's handle).
pub fn fn_00a42230(e: &mut Engine) {
    for slot in [0x011f_3ec0u32, 0x011f_3ec4] {
        let old = e.mem.u32(slot);
        if old != 0 {
            let handle = e.call(GET_FIXED_STRING, &args![0u32]).u32();
            e.mem.set_u32(slot, handle);
            release_fixed_string(e, old);
        }
    }
}

/// `InterlockedIncrement` and `InterlockedDecrement` (KERNEL32.DLL import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00aa13e0` (`cdecl`, the size): the Gamebryo allocation the objects are made with.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiObject`'s constructor (`00a5d3a0`, `thiscall`, no stack words).
const OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The vtables of `NiFloatData` (`0102dabc`) and `NiPosData` (`0109717c`), by their RTTI type
/// descriptors.
const FLOAT_DATA_VTABLE: u32 = 0x0102_dabc;
const POS_DATA_VTABLE: u32 = 0x0109_717c;
/// The smart-pointer assignments of the two data pointers (`0066b0d0` and `00a5f860`,
/// `thiscall` on the pointer, the object) and the data's key replacement (`00a3ccb0`,
/// unit `nifloatdata.cpp`, and `00a48fe0`, unit `niposdata.cpp`; `thiscall`, (keys, count,
/// type)).
const FLOAT_DATA_ASSIGN: u32 = 0x0066_b0d0;
const POS_DATA_ASSIGN: u32 = 0x00a5_f860;
const FLOAT_DATA_REPLACE: u32 = 0x00a3_ccb0;
const POS_DATA_REPLACE: u32 = 0x00a4_8fe0;
/// The `NiRTTI` records (static, built at start-up) of `NiTransformController` (`011f36ec`),
/// `NiTransformInterpolator` (`011f3e50`) and `NiLookAtInterpolator` (`011f3f90`): their
/// initializers name them.
const TRANSFORM_CONTROLLER_RTTI: u32 = 0x011f_36ec;
const TRANSFORM_INTERPOLATOR_RTTI: u32 = 0x011f_3e50;
const LOOK_AT_INTERPOLATOR_RTTI: u32 = 0x011f_3f90;
/// `NiObjectNET::GetController` (`thiscall`, the RTTI) and `NiObjectNET::RemoveController`
/// (`thiscall`, the controller), engine map.
const GET_CONTROLLER: u32 = 0x00a5_c570;
const REMOVE_CONTROLLER: u32 = 0x00a5_c480;
/// The constructors `00a42570` and `00a427e0` build with (`thiscall`): `00a36560` (unit
/// `nitransformcontroller.cpp`, no stack words), `NiLookAtInterpolator::NiLookAtInterpolator`
/// (`00a4bcf0`, three words), `00a495f0` (unit `nipoint3interpolator.cpp`, one word) and
/// `00a3c340` (one word; the float interpolator its object is stored as, by its use).
const TRANSFORM_CONTROLLER_CONSTRUCT: u32 = 0x00a3_6560;
const LOOK_AT_INTERPOLATOR_CONSTRUCT: u32 = 0x00a4_bcf0;
const POINT3_INTERPOLATOR_CONSTRUCT: u32 = 0x00a4_95f0;
const FLOAT_INTERPOLATOR_CONSTRUCT: u32 = 0x00a3_c340;
/// `00eea6d0` (`thiscall`, one word), run on the new look-at interpolator with the address
/// `011f49e0`.
const LOOK_AT_SETUP: u32 = 0x00ee_a6d0;
const LOOK_AT_SETUP_ARGUMENT: u32 = 0x011f_49e0;
/// The transform interpolator's key getters (`thiscall`, three out pointers; answer the
/// keys): `00a3f990` (count, type, size byte; position keys by its use) and `00a3f9e0`
/// (count, type, size byte; scale keys by its use), and the transform data's clears
/// (`00a4e0b0`, `00a4e0f0`, unit `nitransformdata.cpp`; `thiscall`, three zero words).
const POSITION_KEYS: u32 = 0x00a3_f990;
const SCALE_KEYS: u32 = 0x00a3_f9e0;
const CLEAR_POSITION_KEYS: u32 = 0x00a4_e0b0;
const CLEAR_SCALE_KEYS: u32 = 0x00a4_e0f0;

/// Whether `object`'s class is `rtti`'s or derives from it: its `GetRTTI` record (vtable +8)
/// or one of that record's bases (`NiRTTI` +4).
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, 8, &[]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record + 4);
    }
    false
}

/// The smart-pointer store at `this + offset`: a null `object` releases the old one (its
/// count at +4 down, vtable +4 at zero) and clears the pointer; another object releases the
/// old one first, then stores it and counts it up; the same object changes nothing.
fn assign_counted(e: &mut Engine, this: u32, offset: u32, object: u32) {
    let old = e.mem.u32(this + offset);
    if object == 0 {
        if old != 0 {
            if e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
                e.vcall(old, 4, &[]);
            }
            e.mem.set_u32(this + offset, 0);
        }
        return;
    }
    if old == object {
        return;
    }
    if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
        e.vcall(old, 4, &[]);
    }
    e.mem.set_u32(this + offset, object);
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
}

/// The key setters' shared body: with keys (`count` not 0), a data object made first when
/// the pointer at `this + offset` is null (0x18 bytes from `00aa13e0`, `NiObject`'s
/// constructor, `vtable`, +8, +0xc and +0x10 cleared and the byte +0x14; stored by
/// `assign`), the data's keys replaced (`replace`), and the word after the pointer cleared.
/// Without keys, an existing data object's keys are replaced by none.
#[allow(clippy::too_many_arguments)]
fn set_keys(
    e: &mut Engine,
    this: u32,
    offset: u32,
    vtable: u32,
    assign: u32,
    replace: u32,
    keys: u32,
    count: u32,
    kind: u32,
) {
    if count == 0 {
        let data = e.mem.u32(this + offset);
        if data != 0 {
            e.call(replace, &args![data, 0u32, 0u32, 0u32]);
        }
        return;
    }
    if e.mem.u32(this + offset) == 0 {
        let object = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
        let data = if object != 0 {
            e.call(OBJECT_CONSTRUCT, &args![object]);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(object + 8, 0);
            e.mem.set_u32(object + 0xc, 0);
            e.mem.set_u32(object + 0x10, 0);
            e.mem.set_u8(object + 0x14, 0);
            object
        } else {
            0
        };
        e.call(assign, &args![this + offset, data]);
    }
    let data = e.mem.u32(this + offset);
    e.call(replace, &args![data, keys, count, kind]);
    e.mem.set_u32(this + offset + 4, 0);
}

// Translated from 00a420e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, ten words: start value, start time, &start tangent,
/// end value, end time, &end tangent, a time, a word it does not read, &first result,
/// &second result): splits a Hermite segment at `time`, by its arithmetic: with `d = end -
/// start`, `w = end time - start time`, `a = time - start time`, `b = end time - time` and
/// `u = a / w`, the slope `v = ((2 (3 d - 2 s - t) + 3 (s - 2 d + t) u) u + s) / w` from the
/// tangents `s`, `t`; then `first = a v`, `second = b v`, the start tangent scaled by `a / w`
/// (the unrounded quotient) and the end tangent by `b / w`. Every `float` store and reload,
/// and every read of the pointed-to floats, in the exe's order.
#[allow(clippy::too_many_arguments)]
pub fn fn_00a420e0(
    e: &mut Engine,
    start_value: f32,
    start_time: f32,
    start_tangent: Ptr,
    end_value: f32,
    end_time: f32,
    end_tangent: Ptr,
    time: f32,
    _unused_7: u32,
    first: Ptr,
    second: Ptr,
) {
    let st = x87_store32;
    let ld = x87_load32;
    let (s, t) = (start_tangent.addr(), end_tangent.addr());
    let difference = st(x87_sub(
        ld(end_value.to_bits()),
        x87_m32(start_value.to_bits()),
    ));
    let end = ld(end_time.to_bits());
    let start = ld(start_time.to_bits());
    let span = st(x87_sub(end, start));
    let at = ld(time.to_bits());
    let before = st(x87_sub(at, start));
    let after = st(x87_sub(end, at));
    let before_value = ld(before);
    let span_value = ld(span);
    let quotient = x87_div(before_value, span_value);
    let u = st(quotient);
    let d = ld(difference);
    let two = x87_load64(e.mem.u64(TWO));
    let twice_d = x87_mul(d, two);
    let cubic = st(x87_add(
        x87_sub(x87_m32(e.mem.u32(s)), twice_d),
        x87_m32(e.mem.u32(t)),
    ));
    let three = x87_load64(e.mem.u64(THREE));
    let thrice_cubic = x87_mul(ld(cubic), three);
    let thrice_cubic_u = x87_mul(thrice_cubic, ld(u));
    let thrice_d = x87_mul(d, three);
    let twice_s = x87_mul(ld(e.mem.u32(s)), two);
    let quadratic = st(x87_sub(x87_sub(thrice_d, twice_s), ld(e.mem.u32(t))));
    let sum = x87_add(x87_mul(two, ld(quadratic)), thrice_cubic_u);
    let slope = x87_add(x87_mul(sum, x87_m32(u)), x87_m32(e.mem.u32(s)));
    let slope = ld(st(x87_div(slope, span_value)));
    e.mem.set_u32(first.addr(), st(slope));
    let after_value = ld(after);
    e.mem
        .set_u32(second.addr(), st(x87_mul(slope, after_value)));
    let first_value = ld(e.mem.u32(first.addr()));
    e.mem
        .set_u32(first.addr(), st(x87_mul(before_value, first_value)));
    let scaled = x87_mul(quotient, x87_m32(e.mem.u32(s)));
    e.mem.set_u32(s, st(scaled));
    let ratio = x87_div(after_value, span_value);
    let scaled = x87_mul(ratio, x87_m32(e.mem.u32(t)));
    e.mem.set_u32(t, st(scaled));
}

// Translated from 00a42290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (keys, count, type), `RET 0xc`): the float keys'
/// setter of the object whose `NiFloatData` pointer is at +0x10 (the shared body above;
/// `0066b0d0` stores a new data object, `00a3ccb0` replaces the keys, +0x14 cleared).
pub fn fn_00a42290(e: &mut Engine, this: Ptr, keys: u32, count: u32, kind: u32) {
    set_keys(
        e,
        this.addr(),
        0x10,
        FLOAT_DATA_VTABLE,
        FLOAT_DATA_ASSIGN,
        FLOAT_DATA_REPLACE,
        keys,
        count,
        kind,
    );
}

// Translated from 00a42310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the object, `RET 4`): the smart-pointer store at
/// +0x38 (the old object released before the new one is counted up).
pub fn fn_00a42310(e: &mut Engine, this: Ptr, object: u32) {
    assign_counted(e, this.addr(), 0x38, object);
}

// Translated from 00a42380 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the object, `RET 4`): the smart-pointer store at
/// +0x3c, as `00a42310`.
pub fn fn_00a42380(e: &mut Engine, this: Ptr, object: u32) {
    assign_counted(e, this.addr(), 0x3c, object);
}

// Translated from 00a423f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the object, `RET 4`): the smart-pointer store at
/// +0x40, as `00a42310`.
pub fn fn_00a423f0(e: &mut Engine, this: Ptr, object: u32) {
    assign_counted(e, this.addr(), 0x40, object);
}

// Translated from 00a42460 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (keys, count, type), `RET 0xc`): the position keys'
/// setter of the object whose `NiPosData` pointer is at +0x18 (the shared body above;
/// `00a5f860` stores a new data object, `00a48fe0` replaces the keys, +0x1c cleared).
pub fn fn_00a42460(e: &mut Engine, this: Ptr, keys: u32, count: u32, kind: u32) {
    set_keys(
        e,
        this.addr(),
        0x18,
        POS_DATA_VTABLE,
        POS_DATA_ASSIGN,
        POS_DATA_REPLACE,
        keys,
        count,
        kind,
    );
}

// Translated from 00a424e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (source controller, destination controller)): copies
/// the time controller settings: of the 16-bit flags (+8) bit 0 (bit 3 set when it is clear,
/// cleared when set), bits 1-2 and bit 4, then the floats +0xc, +0x14 and +0x18 through the
/// x87 stack, and last bit 3 from the source.
pub fn fn_00a424e0(e: &mut Engine, source: Ptr, dest: Ptr) {
    let (s, d) = (source.addr(), dest.addr());
    let mut flags = e.mem.u16(d + 8);
    let bit0 = e.mem.u16(s + 8) & 1;
    flags = flags & 0xfffe | bit0;
    e.mem.set_u16(d + 8, flags);
    flags = if bit0 == 0 { flags | 8 } else { flags & 0xfff7 };
    e.mem.set_u16(d + 8, flags);
    let mask = (flags ^ e.mem.u16(s + 8)) & 6;
    let flags = e.mem.u16(d + 8) ^ mask;
    e.mem.set_u16(d + 8, flags);
    let flags = if (e.mem.u8(s + 8) >> 4) & 1 != 0 {
        e.mem.u16(d + 8) | 0x10
    } else {
        e.mem.u16(d + 8) & 0xffef
    };
    e.mem.set_u16(d + 8, flags);
    for offset in [0xcu32, 0x14, 0x18] {
        let value = x87_quiet_bits(e.mem.u32(s + offset));
        e.mem.set_u32(d + offset, value);
    }
    let flags = if (e.mem.u8(s + 8) >> 3) & 1 != 0 {
        e.mem.u16(d + 8) | 8
    } else {
        e.mem.u16(d + 8) & 0xfff7
    };
    e.mem.set_u16(d + 8, flags);
}

// Translated from 00a42570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, an old look-at controller; one of the conversions
/// `00a42b70` runs): on its target (+0x2c), the `NiTransformController` (`GetController`)
/// is kept when its interpolator (vtable +0xc4, index 0) is a `NiTransformInterpolator`;
/// otherwise a new one is made (0x38 bytes, `00a36560`) with the old controller's settings
/// (`00a424e0`). A new `NiLookAtInterpolator` (0x44 bytes, `00a4bcf0` with three zeros) gets
/// the old +0x38 at +0x10, `00eea6d0` with `011f49e0`, and flag bits 0-2 (+0xc) from the old
/// +0x34. When the controller's interpolator (counted up meanwhile) is a transform
/// interpolator whose data (+0x2c) has position keys (count +0xa), a new interpolator (0x20
/// bytes, `00a495f0`) takes them (`00a3f990`, `00a42460`), the data's are cleared
/// (`00a4e0b0`) and it is stored at the look-at +0x38; with scale keys (+0xc) likewise
/// (0x18 bytes, `00a3c340`; `00a3f9e0`, `00a42290`, `00a4e0f0`; +0x40). Then the controller
/// takes the look-at interpolator (vtable +0xc8, (it, 0)), the interpolator runs vtable
/// +0xbc, a new controller gets the target (vtable +0x98), the target drops the old
/// controller (`RemoveController`) and the counted interpolator is released.
pub fn fn_00a42570(e: &mut Engine, old: Ptr) {
    let o = old.addr();
    let target = e.mem.u32(o + 0x2c);
    e.with_stack(0x10, |e, local| {
        let l = local.addr();
        let (created_at, size_at, word_8, word_c) = (l + 2, l + 3, l + 8, l + 0xc);
        e.mem.set_u32(l + 4, target);
        e.mem.set_u8(created_at, 0);
        let mut controller = e
            .call(GET_CONTROLLER, &args![target, TRANSFORM_CONTROLLER_RTTI])
            .u32();
        let mut keep = false;
        if controller != 0 {
            let interpolator = e.vcall(controller, 0xc4, &args![0u32]).u32();
            if interpolator != 0 {
                keep = is_kind_of(e, interpolator, TRANSFORM_INTERPOLATOR_RTTI);
            }
        }
        if !keep {
            e.mem.set_u8(created_at, 1);
            let object = e.call(NI_ALLOCATE, &args![0x38u32]).u32();
            controller = if object != 0 {
                e.call(TRANSFORM_CONTROLLER_CONSTRUCT, &args![object]).u32()
            } else {
                0
            };
            fn_00a424e0(e, old, Ptr::new(controller));
        }
        let object = e.call(NI_ALLOCATE, &args![0x44u32]).u32();
        let look_at = if object != 0 {
            e.call(
                LOOK_AT_INTERPOLATOR_CONSTRUCT,
                &args![object, 0u32, 0u32, 0u32],
            )
            .u32()
        } else {
            0
        };
        let look_at_target = e.mem.u32(o + 0x38);
        e.mem.set_u32(look_at + 0x10, look_at_target);
        e.call(LOOK_AT_SETUP, &args![look_at, LOOK_AT_SETUP_ARGUMENT]);
        let flags = if e.mem.u8(o + 0x34) & 1 != 0 {
            e.mem.u16(look_at + 0xc) | 1
        } else {
            e.mem.u16(look_at + 0xc) & 0xfffe
        };
        e.mem.set_u16(look_at + 0xc, flags);
        let mask = (e.mem.u16(look_at + 0xc) ^ e.mem.u16(o + 0x34)) & 6;
        let flags = e.mem.u16(look_at + 0xc) ^ mask;
        e.mem.set_u16(look_at + 0xc, flags);
        let interpolator = e.vcall(controller, 0xc4, &args![0u32]).u32();
        if interpolator != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![interpolator + 4]);
            if is_kind_of(e, interpolator, TRANSFORM_INTERPOLATOR_RTTI) {
                let data = e.mem.u32(interpolator + 0x2c);
                if data != 0 && e.mem.u16(data + 0xa) != 0 {
                    let object = e.call(NI_ALLOCATE, &args![0x20u32]).u32();
                    let point = if object != 0 {
                        e.call(POINT3_INTERPOLATOR_CONSTRUCT, &args![object, 0u32])
                            .u32()
                    } else {
                        0
                    };
                    let keys = e
                        .call(POSITION_KEYS, &args![interpolator, word_c, word_8, size_at])
                        .u32();
                    let (count, kind) = (e.mem.u32(word_c), e.mem.u32(word_8));
                    fn_00a42460(e, Ptr::new(point), keys, count, kind);
                    let data = e.mem.u32(interpolator + 0x2c);
                    e.call(CLEAR_POSITION_KEYS, &args![data, 0u32, 0u32, 0u32]);
                    fn_00a42310(e, Ptr::new(look_at), point);
                }
                let data = e.mem.u32(interpolator + 0x2c);
                if data != 0 && e.mem.u16(data + 0xc) != 0 {
                    let object = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
                    let float = if object != 0 {
                        e.call(FLOAT_INTERPOLATOR_CONSTRUCT, &args![object, 0u32])
                            .u32()
                    } else {
                        0
                    };
                    let keys = e
                        .call(SCALE_KEYS, &args![interpolator, word_8, word_c, size_at])
                        .u32();
                    let (count, kind) = (e.mem.u32(word_8), e.mem.u32(word_c));
                    fn_00a42290(e, Ptr::new(float), keys, count, kind);
                    let data = e.mem.u32(interpolator + 0x2c);
                    e.call(CLEAR_SCALE_KEYS, &args![data, 0u32, 0u32, 0u32]);
                    fn_00a423f0(e, Ptr::new(look_at), float);
                }
            }
        }
        e.vcall(controller, 0xc8, &args![look_at, 0u32]);
        e.vcall(look_at, 0xbc, &[]);
        let target = e.mem.u32(l + 4);
        if e.mem.u8(created_at) != 0 {
            e.vcall(controller, 0x98, &args![target]);
        }
        if target != 0 {
            e.call(REMOVE_CONTROLLER, &args![target, old]);
        }
        if interpolator != 0
            && e.call(INTERLOCKED_DECREMENT, &args![interpolator + 4])
                .u32()
                == 0
        {
            e.vcall(interpolator, 4, &[]);
        }
    });
}

// Translated from 00a427e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, an old roll controller; one of the conversions
/// `00a42b70` runs): among its target's controllers (+0xc, next +0x30), the first
/// `NiTransformController` whose interpolator (vtable +0xc4, index 0) is a
/// `NiLookAtInterpolator` gets a new interpolator (0x18 bytes, `00a3c340` with the old
/// +0x38) at the look-at +0x3c and the old controller's settings (`00a424e0`); the look-at
/// interpolator's two floats from vtable +0xc0 are passed back to its vtable +0xc4, the
/// controller runs vtable +0xcc and the interpolator +0xbc. In every case the target drops
/// the old controller (`RemoveController`). The second float's slot is the argument's own
/// stack word.
pub fn fn_00a427e0(e: &mut Engine, old: Ptr) {
    let o = old.addr();
    let target = e.mem.u32(o + 0x2c);
    let mut controller = e.mem.u32(target + 0xc);
    while controller != 0 {
        if is_kind_of(e, controller, TRANSFORM_CONTROLLER_RTTI) {
            let interpolator = e.vcall(controller, 0xc4, &args![0u32]).u32();
            if interpolator != 0 && is_kind_of(e, interpolator, LOOK_AT_INTERPOLATOR_RTTI) {
                let roll = e.mem.u32(o + 0x38);
                let object = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
                let float = if object != 0 {
                    e.call(FLOAT_INTERPOLATOR_CONSTRUCT, &args![object, roll])
                        .u32()
                } else {
                    0
                };
                fn_00a42380(e, Ptr::new(interpolator), float);
                fn_00a424e0(e, old, Ptr::new(controller));
                e.with_stack(8, |e, local| {
                    let begin_at = local.addr();
                    let end_at = begin_at + 4;
                    e.mem.set_u32(end_at, o);
                    e.vcall(interpolator, 0xc0, &args![begin_at, end_at]);
                    let end = x87_quiet_bits(e.mem.u32(end_at));
                    let begin = x87_quiet_bits(e.mem.u32(begin_at));
                    e.vcall(interpolator, 0xc4, &args![begin, end]);
                });
                e.vcall(controller, 0xcc, &[]);
                e.vcall(interpolator, 0xbc, &[]);
                e.call(REMOVE_CONTROLLER, &args![target, old]);
                return;
            }
        }
        controller = e.mem.u32(controller + 0x30);
    }
    e.call(REMOVE_CONTROLLER, &args![target, old]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a4_20e0,
            fn_00a420e0(f32, f32, Ptr, f32, f32, Ptr, f32, u32, Ptr, Ptr)
        ),
        entry!(0x00a4_2290, fn_00a42290(Ptr, u32, u32, u32)),
        entry!(0x00a4_2310, fn_00a42310(Ptr, u32)),
        entry!(0x00a4_2380, fn_00a42380(Ptr, u32)),
        entry!(0x00a4_23f0, fn_00a423f0(Ptr, u32)),
        entry!(0x00a4_2460, fn_00a42460(Ptr, u32, u32, u32)),
        entry!(0x00a4_24e0, fn_00a424e0(Ptr, Ptr)),
        entry!(0x00a4_2570, fn_00a42570(Ptr)),
        entry!(0x00a4_27e0, fn_00a427e0(Ptr)),
        entry!(0x00a4_21c0, fn_00a421c0()),
        entry!(0x00a42050, fn_00a42050(f32, f32, f32) -> f64),
        entry!(0x00a42070, fn_00a42070(f32, f32, f32, f32, f32) -> f64),
        entry!(0x00a42230, fn_00a42230()),
    ]
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn sets_both_strings_unless_already_set() {
        let mut e = engine_with_counting();
        e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u32(0x011f_3ec0, empty);
        e.mem.set_u32(0x011f_3ec4, 0x0109_77ec);
        e.call_log = Some(vec![]);
        e.call(0x00a4_21c0, &args![]);
        assert_eq!(e.mem.u32(0x011f_3ec0), 0x0109_77f4 + 0x1000);
        assert_eq!(e.mem.u32(0x011f_3ec4), 0x0109_77ec);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0x0109_77f4]]);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn linear_blend() {
        let mut e = Engine::new();
        let r = e.call(0x00a42050, &args![0.25f32, 4.0f32, 8.0f32]).f64();
        assert_eq!(r, 5.0);
        let r = e.call(0x00a42050, &args![1.0f32, 4.0f32, 8.0f32]).f64();
        assert_eq!(r, 8.0);
    }

    #[test]
    fn hermite_value() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0102_1000, 0x1000);
        e.set_global(TWO, 2.0f64);
        e.set_global(THREE, 3.0f64);
        let r = e
            .call(0x00a42070, &args![0.5f32, 1.0f32, 2.0f32, 3.0f32, 4.0f32])
            .f64();
        assert_eq!(r, 1.75);
        // t = 1 lands on the end point.
        let r = e
            .call(0x00a42070, &args![1.0f32, 1.0f32, 2.0f32, 3.0f32, 4.0f32])
            .f64();
        assert_eq!(r, 3.0);
    }

    #[test]
    fn releases_both_static_strings_when_set() {
        let mut e = engine_with_counting();
        e.register(GET_FIXED_STRING, |_, _| Ret {
            eax: 0x0109_b228,
            ..Ret::default()
        });
        e.map(0x011f_3000, 0x1000);
        let counted = e.mem.alloc(0x10);
        e.mem.set_u32(counted, 5);
        e.mem.set_u32(0x011f_3ec0, counted + 8);
        e.mem.set_u32(0x011f_3ec4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a42230, &args![]);
        assert_eq!(e.mem.u32(0x011f_3ec0), 0x0109_b228);
        assert_eq!(e.mem.u32(0x011f_3ec4), 0);
        assert_eq!(e.mem.u32(counted), 4);
        assert_eq!(calls_to(&e, GET_FIXED_STRING), vec![vec![0]]);
        // The empty string handle is not counted down.
        e.mem.set_u32(0x011f_3ec4, 0x0109_b228);
        let empty_count = e.mem.u32(0x0109_b220);
        e.call(0x00a42230, &args![]);
        assert_eq!(e.mem.u32(0x0109_b220), empty_count);
    }
}

#[cfg(test)]
mod n007_tests {
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

    /// An engine with the double constants, counting interlocked doubles, a heap allocator
    /// and a call log.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0102_1000, 0x1000);
        e.set_global(TWO, 2.0f64);
        e.set_global(THREE, 3.0f64);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.i32(a[0]).wrapping_add(1);
            e.mem.set_i32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.i32(a[0]).wrapping_sub(1);
            e.mem.set_i32(a[0], count);
            count.into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// An object of a class whose `GetRTTI` (vtable +8, the double at `get_rtti`) answers
    /// `rtti`, with the other vtable slots given as (offset, function).
    fn object(e: &mut Engine, get_rtti: u32, rtti: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 8, get_rtti);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        e.register_double(get_rtti, move |_, _| rtti.into_ret());
        let object = e.mem.alloc(0x50);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn hermite_split_scales_the_tangents() {
        let mut e = engine();
        let (s, t, first, second) = (
            e.mem.alloc(4),
            e.mem.alloc(4),
            e.mem.alloc(4),
            e.mem.alloc(4),
        );
        e.mem.set_f32(s, 3.0);
        e.mem.set_f32(t, 1.0);
        e.call(
            0x00a4_20e0,
            &args![0.0f32, 0.0f32, s, 1.0f32, 2.0f32, t, 0.5f32, 0u32, first, second],
        );
        // u = 1/4, C3 = 2, C2 = -4: slope ((2 C2 + 3 C3 u) u + s) / 2 = 0.6875.
        assert_eq!(e.mem.f32(first), 0.34375);
        assert_eq!(e.mem.f32(second), 1.03125);
        assert_eq!(e.mem.f32(s), 0.75);
        assert_eq!(e.mem.f32(t), 0.75);
    }

    #[test]
    fn key_setters_make_data_and_replace_keys() {
        let mut e = engine();
        e.register(OBJECT_CONSTRUCT, |_, a| a[0].into_ret());
        for assign in [FLOAT_DATA_ASSIGN, POS_DATA_ASSIGN] {
            e.register(assign, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
        }
        e.register(FLOAT_DATA_REPLACE, |_, _| Ret::default());
        e.register(POS_DATA_REPLACE, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, 9);
        // No keys and no data: nothing.
        e.call(0x00a4_2290, &args![this, 0x700u32, 0u32, 1u32]);
        assert!(calls_to(&e, FLOAT_DATA_REPLACE).is_empty());
        e.call(0x00a4_2290, &args![this, 0x700u32, 3u32, 1u32]);
        let data = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.u32(data), FLOAT_DATA_VTABLE);
        assert_eq!(
            calls_to(&e, FLOAT_DATA_ASSIGN),
            vec![vec![this + 0x10, data]]
        );
        assert_eq!(
            calls_to(&e, FLOAT_DATA_REPLACE),
            vec![vec![data, 0x700, 3, 1]]
        );
        assert_eq!(e.mem.u32(this + 0x14), 0);
        // No keys with data: replaced by none; with data and keys: no new data.
        e.call(0x00a4_2290, &args![this, 0x700u32, 0u32, 1u32]);
        e.call(0x00a4_2290, &args![this, 0x800u32, 2u32, 2u32]);
        assert_eq!(
            calls_to(&e, FLOAT_DATA_REPLACE)[1..],
            [vec![data, 0, 0, 0], vec![data, 0x800, 2, 2]]
        );
        assert_eq!(calls_to(&e, FLOAT_DATA_ASSIGN).len(), 1);
        // The position keys: at +0x18.
        e.call(0x00a4_2460, &args![this, 0x900u32, 1u32, 5u32]);
        let data = e.mem.u32(this + 0x18);
        assert_eq!(e.mem.u32(data), POS_DATA_VTABLE);
        assert_eq!(
            calls_to(&e, POS_DATA_REPLACE),
            vec![vec![data, 0x900, 1, 5]]
        );
    }

    #[test]
    fn counted_stores_release_then_count_up() {
        let mut e = engine();
        e.register(0x0344_0000, |_, _| Ret::default());
        let counted = |e: &mut Engine| {
            let vtable = e.mem.alloc(8);
            e.mem.set_u32(vtable + 4, 0x0344_0000);
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, vtable);
            object
        };
        let (a, b) = (counted(&mut e), counted(&mut e));
        let this = e.mem.alloc(0x44);
        for (address, offset) in [
            (0x00a4_2310u32, 0x38u32),
            (0x00a4_2380, 0x3c),
            (0x00a4_23f0, 0x40),
        ] {
            e.call(address, &args![this, a]);
            assert_eq!((e.mem.u32(this + offset), e.mem.i32(a + 4)), (a, 1));
            e.call(address, &args![this, a]);
            assert_eq!(e.mem.i32(a + 4), 1);
            e.call(address, &args![this, b]);
            assert_eq!((e.mem.u32(this + offset), e.mem.i32(b + 4)), (b, 1));
            e.call(address, &args![this, 0u32]);
            assert_eq!((e.mem.u32(this + offset), e.mem.i32(b + 4)), (0, 0));
        }
        // Each object's last release deleted it.
        assert_eq!(calls_to(&e, 0x0344_0000).len(), 6);
    }

    #[test]
    fn controller_settings_copy() {
        let mut e = engine();
        let (source, dest) = (e.mem.alloc(0x1c), e.mem.alloc(0x1c));
        e.mem.set_u16(source + 8, 0x0017);
        e.mem.set_u16(dest + 8, 0xff28);
        e.mem.set_u32(source + 0xc, 0x7f80_0001);
        e.mem.set_f32(source + 0x14, 2.0);
        e.mem.set_f32(source + 0x18, 3.0);
        e.mem.set_u32(source + 0x10, 7);
        e.call(0x00a4_24e0, &args![source, dest]);
        // Bits 0-2 and 4 from the source, bit 3 from the source last.
        assert_eq!(e.mem.u16(dest + 8), 0xff37 & !0x8);
        assert_eq!(e.mem.u32(dest + 0xc), 0x7fc0_0001);
        assert_eq!((e.mem.f32(dest + 0x14), e.mem.f32(dest + 0x18)), (2.0, 3.0));
        assert_eq!(e.mem.u32(dest + 0x10), 0);
        e.mem.set_u16(source + 8, 0x0008);
        e.call(0x00a4_24e0, &args![source, dest]);
        assert_eq!(e.mem.u16(dest + 8) & 0x1f, 0x08);
    }

    #[test]
    fn look_at_conversion_makes_a_controller_and_interpolator() {
        let mut e = engine();
        for address in [
            LOOK_AT_SETUP,
            REMOVE_CONTROLLER,
            CLEAR_POSITION_KEYS,
            CLEAR_SCALE_KEYS,
            FLOAT_DATA_REPLACE,
            POS_DATA_REPLACE,
            0x0345_0001,
            0x0345_0002,
            0x0345_0003,
            0x0345_0004,
            0x0345_0005,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(GET_CONTROLLER, |_, _| 0u32.into_ret());
        e.register(0x0345_0010, |_, _| 0u32.into_ret());
        let controller = object(
            &mut e,
            0x0345_0020,
            TRANSFORM_CONTROLLER_RTTI,
            &[
                (0xc4, 0x0345_0010),
                (0xc8, 0x0345_0001),
                (0x98, 0x0345_0002),
            ],
        );
        e.register_double(TRANSFORM_CONTROLLER_CONSTRUCT, move |_, _| {
            controller.into_ret()
        });
        let look_at = object(
            &mut e,
            0x0345_0021,
            LOOK_AT_INTERPOLATOR_RTTI,
            &[(0xbc, 0x0345_0003)],
        );
        e.register_double(LOOK_AT_INTERPOLATOR_CONSTRUCT, move |_, _| {
            look_at.into_ret()
        });
        let target = e.mem.alloc(0x10);
        let old = e.mem.alloc(0x40);
        e.mem.set_u32(old + 0x2c, target);
        e.mem.set_u32(old + 0x38, 0x5555);
        e.mem.set_u16(old + 0x34, 0x7);
        e.call(0x00a4_2570, &args![old]);
        assert_eq!(e.mem.u32(look_at + 0x10), 0x5555);
        assert_eq!(e.mem.u16(look_at + 0xc) & 7, 7);
        assert_eq!(
            calls_to(&e, LOOK_AT_SETUP),
            vec![vec![look_at, LOOK_AT_SETUP_ARGUMENT]]
        );
        assert_eq!(
            calls_to(&e, 0x0345_0001),
            vec![vec![controller, look_at, 0]]
        );
        assert_eq!(calls_to(&e, 0x0345_0003), vec![vec![look_at]]);
        // A new controller gets the target; the target drops the old controller.
        assert_eq!(calls_to(&e, 0x0345_0002), vec![vec![controller, target]]);
        assert_eq!(calls_to(&e, REMOVE_CONTROLLER), vec![vec![target, old]]);

        // An existing controller with a transform interpolator holding both kinds of keys.
        e.call_log = Some(vec![]);
        let data = e.mem.alloc(0x10);
        e.mem.set_u16(data + 0xa, 2);
        e.mem.set_u16(data + 0xc, 1);
        let interpolator = object(&mut e, 0x0345_0022, TRANSFORM_INTERPOLATOR_RTTI, &[]);
        e.mem.set_u32(interpolator + 0x2c, data);
        e.mem.set_i32(interpolator + 4, 1);
        let existing = object(
            &mut e,
            0x0345_0023,
            TRANSFORM_CONTROLLER_RTTI,
            &[
                (0xc4, 0x0345_0011),
                (0xc8, 0x0345_0001),
                (0x98, 0x0345_0002),
            ],
        );
        e.register_double(0x0345_0011, move |_, _| interpolator.into_ret());
        e.register_double(GET_CONTROLLER, move |_, _| existing.into_ret());
        e.register(POINT3_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FLOAT_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        for getter in [POSITION_KEYS, SCALE_KEYS] {
            e.register(getter, |e, a| {
                e.mem.set_u32(a[1], 4);
                e.mem.set_u32(a[2], 2);
                0x6000u32.into_ret()
            });
        }
        e.register(OBJECT_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(FLOAT_DATA_ASSIGN, |_, _| Ret::default());
        e.register(POS_DATA_ASSIGN, |_, _| Ret::default());
        e.call(0x00a4_2570, &args![old]);
        // Position keys: count from the first word, type from the second.
        assert_eq!(calls_to(&e, POS_DATA_REPLACE)[0][1..], [0x6000, 4, 2]);
        assert_eq!(calls_to(&e, FLOAT_DATA_REPLACE)[0][1..], [0x6000, 4, 2]);
        assert_eq!(calls_to(&e, CLEAR_POSITION_KEYS), vec![vec![data, 0, 0, 0]]);
        assert_eq!(calls_to(&e, CLEAR_SCALE_KEYS), vec![vec![data, 0, 0, 0]]);
        assert_ne!(e.mem.u32(look_at + 0x38), 0);
        assert_ne!(e.mem.u32(look_at + 0x40), 0);
        // The kept controller is not given the target; the counted interpolator is back to 1.
        assert!(calls_to(&e, 0x0345_0002).is_empty());
        assert_eq!(e.mem.i32(interpolator + 4), 1);
        assert!(calls_to(&e, TRANSFORM_CONTROLLER_CONSTRUCT).is_empty());
    }

    #[test]
    fn roll_conversion_finds_the_look_at_interpolator() {
        let mut e = engine();
        e.register(REMOVE_CONTROLLER, |_, _| Ret::default());
        let target = e.mem.alloc(0x10);
        let old = e.mem.alloc(0x40);
        e.mem.set_u32(old + 0x2c, target);
        e.mem.set_u32(old + 0x38, 0x4444);
        // No controllers: only the removal.
        e.call(0x00a4_27e0, &args![old]);
        assert_eq!(calls_to(&e, REMOVE_CONTROLLER), vec![vec![target, old]]);

        for address in [0x0346_0001u32, 0x0346_0003, 0x0346_0004] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(0x0346_0002, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[2], 2.0);
            Ret::default()
        });
        e.register(FLOAT_INTERPOLATOR_CONSTRUCT, |_, a| a[0].into_ret());
        let look_at = object(
            &mut e,
            0x0346_0020,
            LOOK_AT_INTERPOLATOR_RTTI,
            &[
                (0xc0, 0x0346_0002),
                (0xc4, 0x0346_0003),
                (0xbc, 0x0346_0004),
            ],
        );
        let unrelated = e.mem.alloc(8);
        let other = object(&mut e, 0x0346_0021, unrelated, &[]);
        let controller = object(
            &mut e,
            0x0346_0022,
            TRANSFORM_CONTROLLER_RTTI,
            &[(0xc4, 0x0346_0010), (0xcc, 0x0346_0001)],
        );
        e.register_double(0x0346_0010, move |_, _| look_at.into_ret());
        e.mem.set_u32(target + 0xc, other);
        e.mem.set_u32(other + 0x30, controller);
        e.call(0x00a4_27e0, &args![old]);
        let float = e.mem.u32(look_at + 0x3c);
        assert_ne!(float, 0);
        assert_eq!(
            calls_to(&e, FLOAT_INTERPOLATOR_CONSTRUCT),
            vec![vec![float, 0x4444]]
        );
        assert_eq!(
            calls_to(&e, 0x0346_0003),
            vec![vec![look_at, 1.0f32.to_bits(), 2.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, 0x0346_0001), vec![vec![controller]]);
        assert_eq!(calls_to(&e, 0x0346_0004), vec![vec![look_at]]);
        assert_eq!(calls_to(&e, REMOVE_CONTROLLER).len(), 2);
    }
}
