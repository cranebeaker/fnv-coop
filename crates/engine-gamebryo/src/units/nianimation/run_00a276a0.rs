//! `NiAnimation/run_00a276a0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a27b00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3656` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a27b00(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3656) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3656, 0x1);
    e.mem.set_u32(0x011f_3dc8, 0xe7cd40);
    e.mem.set_u32(0x011f_3a68, 0xa29680);
    e.mem.set_u32(0x011f_39d8, 0xa29680);
    e.mem.set_u32(0x011f_3948, 0xe7cd40);
    e.mem.set_u32(0x011f_38b8, 0xa29680);
    e.mem.set_u8(0x011f_375e, 0x10);
    e.call(0x00a2_7aa0, &args![0x1u32, 0x0u32]);
    1
}

// Translated from 00a28680 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3668` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a28680(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3668) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3668, 0x1);
    e.mem.set_u32(0x011f_3dd0, 0xa281e0);
    e.mem.set_u32(0x011f_3a70, 0xa28260);
    e.mem.set_u32(0x011f_39e0, 0xa27b60);
    e.mem.set_u32(0x011f_3950, 0xa27c40);
    e.mem.set_u32(0x011f_38c0, 0xa2a600);
    e.mem.set_u8(0x011f_3760, 0x40);
    e.call(0x00a2_8620, &args![0x1u32, 0x2u32]);
    1
}

// Translated from 00a27aa0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a27aa0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa27760);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0x0);
}

// Translated from 00a28620 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a28620(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa28060);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa27c80);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa27d60);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27e40);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa27bd0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa27ee0);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa282a0);
}

/// The Ni memory manager (`MemoryManager` singleton the key arrays are
/// allocated from).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `00aa3e40` (thiscall on the memory manager, byte count): allocates.
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `00a27430` (thiscall, stream): loads a key's time.
const KEY_LOAD_TIME: u32 = 0x00a2_7430;
/// `00a59be0` (`NiColor::LoadBinary` in the engine map, thiscall, stream):
/// reads three floats.
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `2.0`, `3.0` and `6.0` (`double`).
const TWO: u32 = 0x0101_1590;
const THREE: u32 = 0x0102_1928;
const SIX: u32 = 0x0103_4208;
/// The number of curvature samples per key (an `int` global).
const CURVATURE_SAMPLES: u32 = 0x011a_7ed4;
/// The time step between curvature samples (a `float` global).
const CURVATURE_STEP: u32 = 0x011f_3658;
/// `_CIsqrt` (`00ec6040`): square root of ST0.
const CI_SQRT: u32 = 0x00ec_6040;
/// The size of one of these position keys.
const KEY_SIZE: u32 = 0x40;

/// `FLD dword` of the float at `at`.
fn load(e: &Engine, at: u32) -> f64 {
    x87_load32(e.mem.u32(at))
}

/// A `float` memory operand at `at`.
fn operand(e: &Engine, at: u32) -> f64 {
    x87_m32(e.mem.u32(at))
}

/// `FSTP dword` of `value` to `at`.
fn store(e: &mut Engine, at: u32, value: f64) {
    e.mem.set_u32(at, x87_store32(value));
}

/// A float stored and loaded again (an `FSTP dword` into a temporary
/// followed by an `FLD dword` or a memory operand of it).
fn rounded(value: f64) -> f64 {
    x87_m32(x87_store32(value))
}

// Translated from 00a281e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// No function in Ghidra (read from the bytes); the key creator this unit's
/// registration stores for key type 2 (`011f3dd0`): allocates `count` keys
/// of 0x40 bytes behind a count word (the byte count saturates to
/// 0xFFFFFFFF on overflow, as the compiler's array new does) from the
/// memory manager, and loads each key's time (`00a27430`) and the points at
/// +0x10 and +0x1C (`00a59be0`) from `stream`. Returns the keys (null when
/// the allocation fails; the loads still run then, as in the exe).
pub fn fn_00a281e0(e: &mut Engine, stream: Ptr, count: u32) -> u32 {
    let bytes = count.saturating_mul(KEY_SIZE);
    let bytes = bytes.saturating_add(4);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    let keys = if block != 0 {
        e.mem.set_u32(block, count);
        block + 4
    } else {
        0
    };
    let mut key = keys;
    for _ in 0..count {
        e.call(KEY_LOAD_TIME, &args![key, stream]);
        e.call(POINT_LOAD_BINARY, &args![key.wrapping_add(0x10), stream]);
        e.call(POINT_LOAD_BINARY, &args![key.wrapping_add(0x1c), stream]);
        key = key.wrapping_add(KEY_SIZE);
    }
    keys
}

// Translated from 00a27c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: the key's cubic at `time`, per lane `value + (out +
/// (a + b * t) * t) * t` with value +4, out tangent +0x1C, a +0x28, b +0x34,
/// written to `out`. Every x87 operation and float temporary as the exe
/// does them. The third argument (the next key) is not read.
pub fn fn_00a27c80(e: &mut Engine, time: f32, key: Ptr, _unused_3: u32, out: Ptr) {
    let k = key.addr();
    let t = x87_load32(time.to_bits());
    // b * t, as floats.
    let b0 = rounded(x87_mul(load(e, k + 0x34), t));
    let b1 = rounded(x87_mul(load(e, k + 0x38), t));
    let b2 = rounded(x87_mul(load(e, k + 0x3c), t));
    let a0 = rounded(x87_add(load(e, k + 0x28), b0));
    let a1 = rounded(x87_add(load(e, k + 0x2c), b1));
    let a2 = rounded(x87_add(load(e, k + 0x30), b2));
    let c0 = rounded(x87_mul(a0, t));
    let c1 = rounded(x87_mul(a1, t));
    let c2 = rounded(x87_mul(a2, t));
    let d0 = rounded(x87_add(load(e, k + 0x1c), c0));
    let d1 = rounded(x87_add(load(e, k + 0x20), c1));
    let d2 = rounded(x87_add(load(e, k + 0x24), c2));
    let e0 = rounded(x87_mul(d0, t));
    let e1 = rounded(x87_mul(d1, t));
    let e2 = rounded(x87_mul(t, d2));
    let x = x87_add(load(e, k + 4), e0);
    let y = x87_add(load(e, k + 8), e1);
    let z = x87_add(load(e, k + 0xc), e2);
    store(e, out.addr(), x);
    store(e, out.addr() + 4, y);
    store(e, out.addr() + 8, z);
}

// Translated from 00a27d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: the cubic's first derivative at `time`, per lane `out
/// + (2a + 3t * b) * t` (fields as in `00a27c80`), written to `out`. The
/// third argument is not read.
pub fn fn_00a27d60(e: &mut Engine, time: f32, key: Ptr, _unused_3: u32, out: Ptr) {
    let k = key.addr();
    let t = x87_load32(time.to_bits());
    let three: f64 = e.global(THREE);
    let two: f64 = e.global(TWO);
    let three_t = rounded(x87_mul(x87_load64(three.to_bits()), t));
    let b0 = rounded(x87_mul(load(e, k + 0x34), three_t));
    let b1 = rounded(x87_mul(load(e, k + 0x38), three_t));
    let b2 = rounded(x87_mul(three_t, operand(e, k + 0x3c)));
    let two = x87_load64(two.to_bits());
    let a0 = rounded(x87_mul(load(e, k + 0x28), two));
    let a1 = rounded(x87_mul(load(e, k + 0x2c), two));
    let a2 = rounded(x87_mul(two, operand(e, k + 0x30)));
    let s0 = rounded(x87_add(a0, b0));
    let s1 = rounded(x87_add(a1, b1));
    let s2 = rounded(x87_add(a2, b2));
    let p0 = rounded(x87_mul(s0, t));
    let p1 = rounded(x87_mul(s1, t));
    let p2 = rounded(x87_mul(t, s2));
    let x = x87_add(load(e, k + 0x1c), p0);
    let y = x87_add(load(e, k + 0x20), p1);
    let z = x87_add(load(e, k + 0x24), p2);
    store(e, out.addr(), x);
    store(e, out.addr() + 4, y);
    store(e, out.addr() + 8, z);
}

// Translated from 00a27e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: the cubic's second derivative at `time`, per lane `2a
/// + 6t * b` (fields as in `00a27c80`), written to `out`. The third
/// argument is not read.
pub fn fn_00a27e40(e: &mut Engine, time: f32, key: Ptr, _unused_3: u32, out: Ptr) {
    let k = key.addr();
    let six: f64 = e.global(SIX);
    let two: f64 = e.global(TWO);
    let six_t = rounded(x87_mul(x87_load32(time.to_bits()), six));
    let b0 = rounded(x87_mul(load(e, k + 0x34), six_t));
    let b1 = rounded(x87_mul(load(e, k + 0x38), six_t));
    let b2 = rounded(x87_mul(six_t, operand(e, k + 0x3c)));
    let two = x87_load64(two.to_bits());
    let a0 = rounded(x87_mul(load(e, k + 0x28), two));
    let a1 = rounded(x87_mul(load(e, k + 0x2c), two));
    let a2 = rounded(x87_mul(two, operand(e, k + 0x30)));
    let x = x87_add(a0, b0);
    store(e, out.addr(), x);
    let y = x87_add(b1, a1);
    store(e, out.addr() + 4, y);
    let z = x87_add(a2, b2);
    store(e, out.addr() + 8, z);
}

// Translated from 00a27ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: fills the cubic coefficients of the first `count - 1`
/// keys from each key and the next: with `d = next.value - value`,
/// `a = 3d - (next.in + 2 * out)` (+0x28) and `b = (next.in + out) - 2d`
/// (+0x34), in-tangent +0x10, out-tangent +0x1C. Every x87 operation and
/// float temporary as the exe does them. `cdecl` with three words (the
/// key-fill table at `011f3780` holds it; the callers push the key size,
/// 0x40 from `00a282a0`, as the third), the third never read.
pub fn fn_00a27ee0(e: &mut Engine, keys: Ptr, count: u32, _unused_2: u32) {
    let mut remaining = count.wrapping_sub(1);
    if remaining == 0 {
        return;
    }
    let two = x87_load64(e.global::<f64>(TWO).to_bits());
    let three = x87_load64(e.global::<f64>(THREE).to_bits());
    let mut key = keys.addr();
    loop {
        let next = key.wrapping_add(KEY_SIZE);
        let mut out2 = [0f64; 3];
        let mut delta3 = [0f64; 3];
        for lane in 0..3u32 {
            out2[lane as usize] = rounded(x87_mul(load(e, key + 0x1c + 4 * lane), two));
        }
        let mut sum = [0f64; 3];
        for lane in 0..3u32 {
            sum[lane as usize] = rounded(x87_add(
                load(e, next + 0x10 + 4 * lane),
                out2[lane as usize],
            ));
        }
        let mut delta = [0f64; 3];
        for lane in 0..3u32 {
            delta[lane as usize] = rounded(x87_sub(
                load(e, next + 4 + 4 * lane),
                operand(e, key + 4 + 4 * lane),
            ));
        }
        for lane in 0..3 {
            delta3[lane] = rounded(x87_mul(delta[lane], three));
        }
        for lane in 0..3u32 {
            let a = x87_sub(delta3[lane as usize], sum[lane as usize]);
            store(e, key + 0x28 + 4 * lane, a);
        }
        let mut delta_again = [0f64; 3];
        for lane in 0..3u32 {
            delta_again[lane as usize] = rounded(x87_sub(
                load(e, next + 4 + 4 * lane),
                operand(e, key + 4 + 4 * lane),
            ));
        }
        let mut delta2 = [0f64; 3];
        for lane in 0..3 {
            delta2[lane] = rounded(x87_mul(delta_again[lane], two));
        }
        let mut tangents = [0f64; 3];
        for lane in 0..3u32 {
            tangents[lane as usize] = rounded(x87_add(
                load(e, next + 0x10 + 4 * lane),
                operand(e, key + 0x1c + 4 * lane),
            ));
        }
        for lane in 0..3u32 {
            let b = x87_sub(tangents[lane as usize], delta2[lane as usize]);
            store(e, key + 0x34 + 4 * lane, b);
        }
        key = next;
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            break;
        }
    }
}

// Translated from 00a28060 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name: the largest curvature measure over the first
/// `count - 1` keys: for each key, at the times `i * step` (`011f3658`) for
/// `i` in 0..=`011a7ed4`, the first (`00a27d60`) and second (`00a27e40`)
/// derivatives d1 and d2 give `|d1 x d2| / |d1|^2` (the cross product's
/// length through `_CIsqrt`); the maximum (0 to start) is returned in ST0.
/// Every x87 operation and float or double temporary as the exe does them.
pub fn fn_00a28060(e: &mut Engine, keys: Ptr, count: u32) -> f32 {
    let mut maximum = x87_store32(0.0);
    let mut remaining = count.wrapping_sub(1);
    if remaining != 0 {
        let mut key = keys.addr();
        e.with_stack(0x18, |e, locals| {
            let first = locals.addr();
            let second = first + 0xc;
            loop {
                let next = key.wrapping_add(KEY_SIZE);
                let mut index = 0i32;
                if e.global::<i32>(CURVATURE_SAMPLES) >= 0 {
                    loop {
                        let step = operand(e, CURVATURE_STEP);
                        let time = x87_store32(x87_mul(f64::from(index), step));
                        let time = x87_quiet_bits(time);
                        fn_00a27d60(
                            e,
                            f32::from_bits(time),
                            Ptr::new(key),
                            next,
                            Ptr::new(first),
                        );
                        fn_00a27e40(
                            e,
                            f32::from_bits(time),
                            Ptr::new(key),
                            next,
                            Ptr::new(second),
                        );
                        let (d1x, d1y, d1z) =
                            (load(e, first), load(e, first + 4), load(e, first + 8));
                        let (d2x, d2y, d2z) =
                            (load(e, second), load(e, second + 4), load(e, second + 8));
                        let cross_x = rounded(x87_sub(x87_mul(d1y, d2z), x87_mul(d1z, d2y)));
                        let cross_y = rounded(x87_sub(x87_mul(d1z, d2x), x87_mul(d2z, d1x)));
                        let cross_z = rounded(x87_sub(x87_mul(d2y, d1x), x87_mul(d2x, d1y)));
                        let length2 = x87_add(
                            x87_mul(cross_z, cross_z),
                            x87_add(x87_mul(cross_x, cross_x), x87_mul(cross_y, cross_y)),
                        );
                        let length2 = rounded(length2);
                        let root = e.call(CI_SQRT, &args![length2]).st0;
                        let root = rounded(root);
                        let speed2 = x87_add(
                            x87_add(x87_mul(d1x, d1x), x87_mul(d1y, d1y)),
                            x87_mul(d1z, d1z),
                        );
                        let speed2 = rounded(speed2);
                        let curvature = rounded(x87_div(root, speed2));
                        if x87_load32(maximum) < curvature {
                            maximum = x87_store32(curvature);
                        }
                        index += 1;
                        if index > e.global::<i32>(CURVATURE_SAMPLES) {
                            break;
                        }
                    }
                }
                remaining = remaining.wrapping_sub(1);
                key = next;
                if remaining == 0 {
                    break;
                }
            }
        });
    }
    f32::from_bits(x87_quiet_bits(maximum))
}

/// `NiPosKey::GenInterp` (`cdecl`, (result, time, keys, type, count, &last index, size);
/// answers the result).
const POS_KEY_GEN_INTERP: u32 = 0x00a2_7490;
/// The three words a position key list without keys answers (`011f365c`).
const NO_KEYS_POINT: u32 = 0x011f_365c;
/// `00a3d520` (`cdecl`, two keys): whether the keys' times are equal.
const KEY_TIMES_EQUAL: u32 = 0x00a3_d520;
/// The position-key row (row 1) of the first and second derivative tables, one function
/// per key type (`cdecl`, (time, key, next key, result)): `011f3c00 + 0x18` and
/// `011f3b70 + 0x18`.
const FIRST_DERIVATIVES: u32 = 0x011f_3c18;
const SECOND_DERIVATIVES: u32 = 0x011f_3b88;
/// The float `1e-6` the frame compares a curvature against (`010959f4`).
const FLAT_CURVATURE: u32 = 0x0109_59f4;
/// `004a0c10` (`thiscall` on a point, no stack words): makes it unit length (zero when its
/// length is at most `1e-6`).
const UNITIZE: u32 = 0x004a_0c10;
/// `004390c0` (`thiscall` on a point, the other point): whether the points are equal.
const POINTS_EQUAL: u32 = 0x0043_90c0;
/// `NiStreamSaveBinary<float>` (`00a3da50`, `cdecl`, (stream, key)): writes the key's time.
const WRITE_KEY_TIME: u32 = 0x00a3_da50;
/// `NiColor::SaveBinary` (`00a69780`, `thiscall`, the stream): writes three floats.
const WRITE_THREE_FLOATS: u32 = 0x00a6_9780;
/// `00a27450` (unit `run_00a26d70`, `cdecl`, (stream, key)): writes a key's time and the
/// three floats after it.
const WRITE_POINT_KEY: u32 = 0x00a2_7450;
/// `NiQuaternion::SaveBinary` (`00a6e4a0`, `thiscall`, the stream).
const QUATERNION_SAVE_BINARY: u32 = 0x00a6_e4a0;
/// `00a6db50` (`thiscall`, (w, x, y, z); answers `this`): sets a quaternion's four floats.
const QUATERNION_SET: u32 = 0x00a6_db50;
/// `00a3d5d0` (`cdecl`, (time, keys, count, &index, size)): finds where a key at `time`
/// goes (the index of the first key not before it); false when a key has that time.
const FIND_INSERT_INDEX: u32 = 0x00a3_d5d0;
/// `_memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `00aa4060` (`thiscall` on the memory manager, the block): frees.
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `00a420e0` (`cdecl`, ten words): the tangents around a new key, one lane at a time (the
/// previous key's value, time and out tangent pointer, the next key's value, time and in
/// tangent pointer, the new key's time and value, and its in and out tangent pointers).
const NEW_KEY_TANGENTS: u32 = 0x00a4_20e0;
/// The three words of the zero point (`011f426c`).
const ZERO_POINT: u32 = 0x011f_426c;

// Translated from 00a276a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (result, time, keys, type, count, size); answers
/// `result`): the position keys' value at `time` written to `result`. No keys: the three
/// words at `011f365c`; before the first key's time: the first key's value (+4); after the
/// time of the last key (at `(count - 1) * size`, the size's low byte): the last key's
/// value (all plain word copies); otherwise `NiPosKey::GenInterp` (`00a27490`), the count's
/// stack word (set to 0) as its last-index word.
pub fn fn_00a276a0(
    e: &mut Engine,
    result: Ptr,
    time: f32,
    keys: Ptr,
    kind: u32,
    count: u32,
    size: u32,
) -> Ptr {
    let copy3 = |e: &mut Engine, from: u32| {
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(from.wrapping_add(offset));
            e.mem.set_u32(result.addr() + offset, word);
        }
    };
    if count == 0 {
        copy3(e, NO_KEYS_POINT);
        return result;
    }
    let t = x87_load32(time.to_bits());
    if load(e, keys.addr()) > t {
        copy3(e, keys.addr() + 4);
        return result;
    }
    let last = count.wrapping_sub(1).wrapping_mul(size & 0xff);
    let last = keys.addr().wrapping_add(last);
    if load(e, last) < t {
        copy3(e, last.wrapping_add(4));
        return result;
    }
    e.with_stack(4, |e, last_index| {
        e.mem.set_u32(last_index.addr(), 0);
        e.call(
            POS_KEY_GEN_INTERP,
            &args![result, x87_store32(t), keys, kind, count, last_index, size],
        );
    });
    result
}

// Translated from 00a27760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two keys): whether the keys are equal: their times
/// (`00a3d520`), then the three floats at +4 (`==` as the x87 compares them; a NaN is
/// unequal).
pub fn fn_00a27760(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    if !e.call(KEY_TIMES_EQUAL, &args![first, second]).bool() {
        return false;
    }
    [4u32, 8, 0xc]
        .iter()
        .all(|&offset| load(e, first.addr() + offset) == load(e, second.addr() + offset))
}

// Translated from 00a277c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (time, key, next key, type, &tangent, &normal,
/// &binormal, &curvature)): the curve frame at `time`. The first and second derivatives d1
/// and d2 come from the position-key row of the derivative tables for `type` (`011f3c18`,
/// `011f3b88`). The tangent is `d1 / |d1|` (`_CIsqrt` of `|d1|^2`); the curvature `|d1 x
/// d2| / |d1|^2` (as `(|d1 x d2| * (1 / |d1|)) * (1 / |d1|)`). Above `1e-6` (`010959f4`)
/// the normal is `|d1|^2 d2 - (d1 . d2) d1`; otherwise the curvature is 0 and the normal
/// `(ty, -tx, 0)` when the tangent's x or y is above `1e-6` in size, else `(0, -tz, ty)`.
/// The normal is made unit length (`004a0c10`) and the binormal is `tangent x normal`
/// (`n.z * t.y - t.z * n.y`, ...). Every x87 operation and float temporary as the exe
/// does them.
pub fn fn_00a277c0(
    e: &mut Engine,
    time: f32,
    key: Ptr,
    next: Ptr,
    kind: u32,
    tangent: Ptr,
    normal: Ptr,
    binormal: Ptr,
    curvature: Ptr,
) {
    let first_derivative = e
        .mem
        .u32(FIRST_DERIVATIVES.wrapping_add(kind.wrapping_mul(4)));
    let second_derivative = e
        .mem
        .u32(SECOND_DERIVATIVES.wrapping_add(kind.wrapping_mul(4)));
    let t = x87_quiet_bits(time.to_bits());
    let (x, y, z, d2x, d2y, d2z) = e.with_stack(0x18, |e, locals| {
        let d1 = locals.addr();
        let d2 = d1 + 0xc;
        e.call(first_derivative, &args![t, key, next, d1]);
        e.call(second_derivative, &args![t, key, next, d2]);
        (
            load(e, d1),
            load(e, d1 + 4),
            load(e, d1 + 8),
            load(e, d2),
            load(e, d2 + 4),
            load(e, d2 + 8),
        )
    });
    // |d1|^2, its root and the root's inverse, each kept as a float.
    let speed2 = rounded(x87_add(
        x87_add(x87_mul(y, y), x87_mul(x, x)),
        x87_mul(z, z),
    ));
    let speed = rounded(e.call(CI_SQRT, &args![speed2]).st0);
    let inverse = rounded(x87_div(1.0, speed));
    let tx = x87_store32(x87_mul(x, inverse));
    let ty = x87_store32(x87_mul(y, inverse));
    let tz = x87_store32(x87_mul(inverse, z));
    let t_at = tangent.addr();
    e.mem.set_u32(t_at, tx);
    e.mem.set_u32(t_at + 4, ty);
    e.mem.set_u32(t_at + 8, tz);
    // d1 x d2.
    let cx = rounded(x87_sub(x87_mul(d2z, y), x87_mul(d2y, z)));
    let cy = rounded(x87_sub(x87_mul(z, d2x), x87_mul(x, d2z)));
    let cz = rounded(x87_sub(x87_mul(x, d2y), x87_mul(d2x, y)));
    let cross2 = rounded(x87_add(
        x87_add(x87_mul(cy, cy), x87_mul(cx, cx)),
        x87_mul(cz, cz),
    ));
    let cross = rounded(e.call(CI_SQRT, &args![cross2]).st0);
    let bend = x87_store32(x87_mul(x87_mul(cross, inverse), inverse));
    e.mem.set_u32(curvature.addr(), x87_quiet_bits(bend));
    let size = rounded(x87_load32(bend).abs());
    let flat = load(e, FLAT_CURVATURE);
    let n_at = normal.addr();
    if flat < size {
        let dot = rounded(x87_add(
            x87_mul(d2z, z),
            x87_add(x87_mul(d2x, x), x87_mul(d2y, y)),
        ));
        let px = x87_store32(x87_mul(x, dot));
        let py = x87_store32(x87_mul(y, dot));
        let pz = x87_store32(x87_mul(dot, z));
        let qx = rounded(x87_mul(d2x, speed2));
        let qy = rounded(x87_mul(d2y, speed2));
        let qz = rounded(x87_mul(speed2, d2z));
        e.mem.set_u32(n_at, x87_store32(x87_sub(qx, x87_m32(px))));
        e.mem
            .set_u32(n_at + 4, x87_store32(x87_sub(qy, x87_m32(py))));
        e.mem
            .set_u32(n_at + 8, x87_store32(x87_sub(qz, x87_m32(pz))));
    } else {
        e.mem.set_u32(curvature.addr(), 0);
        let size_x = rounded(load(e, t_at).abs());
        let use_xy = flat < size_x || {
            let size_y = rounded(load(e, t_at + 4).abs());
            flat < size_y
        };
        if use_xy {
            let word = x87_quiet_bits(e.mem.u32(t_at + 4));
            e.mem.set_u32(n_at, word);
            let word = x87_quiet_bits(e.mem.u32(t_at)) ^ 0x8000_0000;
            e.mem.set_u32(n_at + 4, word);
            e.mem.set_u32(n_at + 8, 0);
        } else {
            let word = x87_quiet_bits(e.mem.u32(t_at + 4));
            e.mem.set_u32(n_at + 8, word);
            let word = x87_quiet_bits(e.mem.u32(t_at + 8)) ^ 0x8000_0000;
            e.mem.set_u32(n_at + 4, word);
            e.mem.set_u32(n_at, 0);
        }
    }
    e.call(UNITIZE, &args![normal]);
    let lane = |e: &Engine, a: u32, b: u32, c: u32, d: u32| {
        x87_store32(x87_sub(
            x87_mul(load(e, a), operand(e, b)),
            x87_mul(load(e, c), operand(e, d)),
        ))
    };
    let bx = lane(e, n_at + 8, t_at + 4, t_at + 8, n_at + 4);
    let by = lane(e, n_at, t_at + 8, n_at + 8, t_at);
    let bz = lane(e, t_at, n_at + 4, n_at, t_at + 4);
    e.mem.set_u32(binormal.addr(), bx);
    e.mem.set_u32(binormal.addr() + 4, by);
    e.mem.set_u32(binormal.addr() + 8, bz);
}

// Translated from 00a27b60 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, (destination, source)): copies a 0x40-byte
/// position key, its time through the x87 stack (`FLD`/`FSTP`, which quiets a signalling
/// NaN) and the fifteen words after it as plain words.
pub fn fn_00a27b60(e: &mut Engine, destination: Ptr, source: Ptr) {
    let time = x87_quiet_bits(e.mem.u32(source.addr()));
    e.mem.set_u32(destination.addr(), time);
    for offset in (4..KEY_SIZE).step_by(4) {
        let word = e.mem.u32(source.addr() + offset);
        e.mem.set_u32(destination.addr() + offset, word);
    }
}

// Translated from 00a27bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two keys): whether two 0x40-byte position keys are
/// equal: time and value (`00a27760`), the three floats at +0x10 (`==`, a NaN unequal),
/// then the points at +0x1c (`004390c0`).
pub fn fn_00a27bd0(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    if !fn_00a27760(e, first, second) {
        return false;
    }
    let same = [0x10u32, 0x14, 0x18]
        .iter()
        .all(|&offset| load(e, first.addr() + offset) == load(e, second.addr() + offset));
    same && e
        .call(
            POINTS_EQUAL,
            &args![first.addr() + 0x1c, second.addr() + 0x1c],
        )
        .bool()
}

// Translated from 00a27c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the count): makes `count` uninitialised position keys
/// of 0x40 bytes from the memory manager (`count * 0x40 + 4` bytes, saturating, the count in
/// the first word). Answers the keys, or null when the allocation fails.
pub fn fn_00a27c40(e: &mut Engine, count: u32) -> u32 {
    let bytes = count
        .checked_mul(KEY_SIZE)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    if block != 0 {
        e.mem.set_u32(block, count);
        block + 4
    } else {
        0
    }
}

// Translated from 00a28260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (stream, keys, count)): writes `count` 0x40-byte
/// position keys: each one's time and value (`00a27450`), then the points at +0x10 and +0x1c
/// (`NiColor::SaveBinary`, `00a69780`).
pub fn fn_00a28260(e: &mut Engine, stream: Ptr, keys: Ptr, count: u32) {
    let mut key = keys.addr();
    for _ in 0..count {
        e.call(WRITE_POINT_KEY, &args![stream, key]);
        e.call(WRITE_THREE_FLOATS, &args![key.wrapping_add(0x10), stream]);
        e.call(WRITE_THREE_FLOATS, &args![key.wrapping_add(0x1c), stream]);
        key = key.wrapping_add(KEY_SIZE);
    }
}

// Translated from 00a282a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, (time, &keys, &count); a bool): inserts a key at
/// `time` into a list of 0x40-byte position keys (time, value +4, in tangent +0x10, out
/// tangent +0x1c). `00a3d5d0` finds the index; when a key already has that time nothing
/// changes (false). Otherwise a list one longer (from the memory manager, saturating size)
/// takes the keys before the index, the new key and the keys after it (`_memcpy`); the new
/// key's value is the old keys' value at `time` (`00a276a0`, type 2, size 0x40) and both its
/// tangents the zero point (`011f426c`). Between two keys, `00a420e0` runs once per lane
/// (x, y, z) on stack copies and gives the previous key's out tangent, the new key's tangents
/// and the next key's in tangent. The count grows by one, the old list is freed (`00aa4060`,
/// from its count word), the new one stored, and its cubic coefficients filled (`00a27ee0`).
/// Answers true.
pub fn fn_00a282a0(e: &mut Engine, time: f32, keys: Ptr, count: Ptr) -> bool {
    let old = e.mem.u32(keys.addr());
    let time_word = x87_quiet_bits(time.to_bits());
    // The stack locals from ESP-0x70 up (offsets into the block): the index (0), the next
    // key's time (4), the previous key's time (8), the old keys (0xc), the previous key's out
    // tangent (0x10), the next key's in tangent (0x1c), the new key's value (0x28), the next
    // key's value (0x34), the previous key's value (0x40), the new key's in (0x4c) and out
    // (0x58) tangents and the value at the time (0x64).
    e.with_stack(0x70, |e, locals| {
        let l = locals.addr();
        let index_word = l;
        let (next_time, previous_time) = (l + 4, l + 8);
        let (previous_out, next_in, value) = (l + 0x10, l + 0x1c, l + 0x28);
        let (next_value, previous_value) = (l + 0x34, l + 0x40);
        let (new_in, new_out, at_time) = (l + 0x4c, l + 0x58, l + 0x64);
        e.mem.set_u32(l + 0xc, old);
        let n = e.mem.u32(count.addr());
        let found = e
            .call(
                FIND_INSERT_INDEX,
                &args![time_word, old, n, index_word, KEY_SIZE],
            )
            .bool();
        if !found {
            return false;
        }
        let grown = e.mem.u32(count.addr()).wrapping_add(1);
        let new = fn_00a27c40(e, grown);
        let index = e.mem.u32(index_word);
        e.call(MEMCPY, &args![new, old, index << 6]);
        let n = e.mem.u32(count.addr());
        let index = e.mem.u32(index_word);
        if n > index {
            e.call(
                MEMCPY,
                &args![
                    (index << 6).wrapping_add(new).wrapping_add(KEY_SIZE),
                    (index << 6).wrapping_add(old),
                    (n - index) << 6
                ],
            );
        }
        let n = e.mem.u32(count.addr());
        fn_00a276a0(
            e,
            Ptr::new(at_time),
            f32::from_bits(time_word),
            Ptr::new(old),
            2,
            n,
            KEY_SIZE,
        );
        let key = (e.mem.u32(index_word) << 6).wrapping_add(new);
        let time_st = x87_load32(time.to_bits());
        e.mem.set_u32(key, x87_store32(time_st));
        let copy = |e: &mut Engine, to: u32, from: u32, words: u32| {
            for i in 0..words {
                let word = e.mem.u32(from.wrapping_add(4 * i));
                e.mem.set_u32(to.wrapping_add(4 * i), word);
            }
        };
        copy(e, key.wrapping_add(4), at_time, 3);
        copy(e, key.wrapping_add(0x10), ZERO_POINT, 3);
        copy(e, key.wrapping_add(0x1c), ZERO_POINT, 3);
        let index = e.mem.u32(index_word);
        if index == 0 || index == e.mem.u32(count.addr()) {
            return finish_position_insert(e, keys, count, old, new);
        }
        let at = (index << 6).wrapping_add(new);
        let previous = at.wrapping_sub(KEY_SIZE);
        let following = at.wrapping_add(KEY_SIZE);
        let quiet = |e: &Engine, from: u32| x87_quiet_bits(e.mem.u32(from));
        let word = quiet(e, previous);
        e.mem.set_u32(previous_time, word);
        copy(e, previous_value, previous.wrapping_add(4), 3);
        copy(e, previous_out, previous.wrapping_add(0x1c), 3);
        let word = quiet(e, following);
        e.mem.set_u32(next_time, word);
        copy(e, value, key.wrapping_add(4), 3);
        copy(e, next_value, following.wrapping_add(4), 3);
        copy(e, next_in, following.wrapping_add(0x10), 3);
        for lane in 0..3u32 {
            let words = [
                quiet(e, previous_value + 4 * lane),
                quiet(e, previous_time),
                previous_out + 4 * lane,
                quiet(e, next_value + 4 * lane),
                quiet(e, next_time),
                next_in + 4 * lane,
                x87_store32(time_st),
                quiet(e, value + 4 * lane),
                new_in + 4 * lane,
                new_out + 4 * lane,
            ];
            e.call(NEW_KEY_TANGENTS, &words);
        }
        copy(e, previous.wrapping_add(0x1c), previous_out, 3);
        copy(e, key.wrapping_add(0x10), new_in, 3);
        copy(e, key.wrapping_add(0x1c), new_out, 3);
        copy(e, following.wrapping_add(0x10), next_in, 3);
        finish_position_insert(e, keys, count, old, new)
    })
}

/// The end of `fn_00a282a0`: the count grows by one, the old list is freed, the new one
/// stored and its coefficients filled (`00a27ee0` on the new list and count). Answers true.
fn finish_position_insert(e: &mut Engine, keys: Ptr, count: Ptr, old: u32, new: u32) -> bool {
    let n = e.mem.u32(count.addr());
    e.mem.set_u32(count.addr(), n.wrapping_add(1));
    if old != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, old.wrapping_sub(4)]);
    }
    e.mem.set_u32(keys.addr(), new);
    let n = e.mem.u32(count.addr());
    fn_00a27ee0(e, Ptr::new(new), n, 0x40);
    true
}

// Translated from 00a286e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, four words, the shape of the interpolation
/// functions: (time, key, next key, result)): writes the identity quaternion: `00a6db50`
/// sets a stack quaternion to (1, 0, 0, 0) and its four words are copied to `result`; the
/// other words are not read.
pub fn fn_00a286e0(e: &mut Engine, _unused_0: u32, _unused_1: u32, _unused_2: u32, result: Ptr) {
    e.with_stack(0x10, |e, quaternion| {
        let one = x87_store32(1.0);
        let made = e
            .call(QUATERNION_SET, &args![quaternion, one, 0u32, 0u32, 0u32])
            .u32();
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(made.wrapping_add(offset));
            e.mem.set_u32(result.addr() + offset, word);
        }
    });
}

// Translated from 00a28720 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, (stream, key)): writes a rotation key's time
/// (`NiStreamSaveBinary<float>`, `00a3da50`), then its quaternion at +4
/// (`NiQuaternion::SaveBinary`, `00a6e4a0`).
pub fn fn_00a28720(e: &mut Engine, stream: Ptr, key: Ptr) {
    e.call(WRITE_KEY_TIME, &args![stream, key]);
    e.call(QUATERNION_SAVE_BINARY, &args![key.addr() + 4, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a27b00, fn_00a27b00() -> u32),
        entry!(0x00a28680, fn_00a28680() -> u32),
        entry!(0x00a27aa0, fn_00a27aa0(u32, u32)),
        entry!(0x00a28620, fn_00a28620(u32, u32)),
        entry!(0x00a2_81e0, fn_00a281e0(Ptr, u32) -> u32),
        entry!(0x00a2_7c80, fn_00a27c80(f32, Ptr, u32, Ptr)),
        entry!(0x00a2_7d60, fn_00a27d60(f32, Ptr, u32, Ptr)),
        entry!(0x00a2_7e40, fn_00a27e40(f32, Ptr, u32, Ptr)),
        entry!(0x00a2_7ee0, fn_00a27ee0(Ptr, u32, u32)),
        entry!(0x00a2_8060, fn_00a28060(Ptr, u32) -> f32),
        entry!(0x00a2_76a0, fn_00a276a0(Ptr, f32, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00a2_7760, fn_00a27760(Ptr, Ptr) -> bool),
        entry!(
            0x00a2_77c0,
            fn_00a277c0(f32, Ptr, Ptr, u32, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00a2_7b60, fn_00a27b60(Ptr, Ptr)),
        entry!(0x00a2_7bd0, fn_00a27bd0(Ptr, Ptr) -> bool),
        entry!(0x00a2_7c40, fn_00a27c40(u32) -> u32),
        entry!(0x00a2_8260, fn_00a28260(Ptr, Ptr, u32)),
        entry!(0x00a2_82a0, fn_00a282a0(f32, Ptr, Ptr) -> bool),
        entry!(0x00a2_86e0, fn_00a286e0(u32, u32, u32, Ptr)),
        entry!(0x00a2_8720, fn_00a28720(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_registration_runs_once_and_stores_through_its_helper() {
        let mut e = Engine::new();
        e.map(0x011f_3600, 0x900);
        assert_eq!(e.call(0x00a27b00, &[]).u32(), 1);
        assert_eq!(e.mem.u8(0x011f_3656), 1);
        assert_eq!(e.mem.u32(0x011f_3dc8), 0x00e7_cd40);
        assert_eq!(e.mem.u8(0x011f_375e), 0x10);
        // The helper with row 1, column 0: (1 * 6 + 0) * 4 = 0x18 into
        // each of its tables.
        assert_eq!(e.mem.u32(0x011f_3d20 + 0x18), 0x00a2_91e0);
        assert_eq!(e.mem.u32(0x011f_3810 + 0x18), 0);
        // Once only: the guard is set, later calls answer 0 and store nothing.
        e.mem.set_u32(0x011f_3dc8, 0);
        assert_eq!(e.call(0x00a27b00, &[]).u32(), 0);
        assert_eq!(e.mem.u32(0x011f_3dc8), 0);
    }

    #[test]
    fn the_helper_indexes_by_row_and_column() {
        let mut e = Engine::new();
        e.map(0x011f_3600, 0x900);
        e.mem.set_u32(0x011f_3810 + 0x2c, 0x5555);
        e.call(0x00a27aa0, &[1, 5]);
        // (1 * 6 + 5) * 4 = 0x2c.
        assert_eq!(e.mem.u32(0x011f_3c90 + 0x2c), 0x00a2_7470);
        assert_eq!(e.mem.u32(0x011f_3810 + 0x2c), 0);
    }

    fn constants(e: &mut Engine) {
        e.map(TWO, 8);
        e.map(THREE, 8);
        e.map(SIX, 8);
        e.set_global(TWO, 2.0f64);
        e.set_global(THREE, 3.0f64);
        e.set_global(SIX, 6.0f64);
    }

    /// A key with value v (+4), in-tangent i (+0x10), out-tangent o
    /// (+0x1C), a (+0x28), b (+0x34), each the same in all lanes but scaled
    /// by the lane number plus 1.
    fn key_at(e: &mut Engine, key: u32, v: f32, i: f32, o: f32, a: f32, b: f32) {
        for lane in 0..3u32 {
            let s = (lane + 1) as f32;
            e.mem.set_f32(key + 4 + 4 * lane, v * s);
            e.mem.set_f32(key + 0x10 + 4 * lane, i * s);
            e.mem.set_f32(key + 0x1c + 4 * lane, o * s);
            e.mem.set_f32(key + 0x28 + 4 * lane, a * s);
            e.mem.set_f32(key + 0x34 + 4 * lane, b * s);
        }
    }

    fn lanes(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    #[test]
    fn fn_00a281e0_allocates_and_loads_the_keys() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(KEY_LOAD_TIME, |_, _| Ret::default());
        e.register(POINT_LOAD_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_81e0, &args![0x5000u32, 2u32]).u32();
        assert_eq!(e.mem.u32(keys - 4), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MEMORY_ALLOCATE, vec![MEMORY_MANAGER, 0x84]));
        let loads: Vec<_> = log.into_iter().skip(2).collect();
        assert_eq!(
            loads,
            vec![
                (KEY_LOAD_TIME, vec![keys, 0x5000]),
                (POINT_LOAD_BINARY, vec![keys + 0x10, 0x5000]),
                (POINT_LOAD_BINARY, vec![keys + 0x1c, 0x5000]),
                (KEY_LOAD_TIME, vec![keys + 0x40, 0x5000]),
                (POINT_LOAD_BINARY, vec![keys + 0x50, 0x5000]),
                (POINT_LOAD_BINARY, vec![keys + 0x5c, 0x5000]),
            ]
        );
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00a2_81e0, &args![0x5000u32, 0u32]).u32(), 0);
    }

    #[test]
    fn the_cubic_and_its_derivatives() {
        let mut e = Engine::new();
        constants(&mut e);
        let key = e.mem.alloc(0x40);
        // value 1, out 2, a 3, b 4.
        key_at(&mut e, key, 1.0, 0.0, 2.0, 3.0, 4.0);
        let out = e.mem.alloc(0xc);
        e.call(0x00a2_7c80, &args![2.0f32, key, 0u32, out]);
        // 1 + (2 + (3 + 4t)t)t at t = 2: 1 + (2 + 22)2 = 49.
        assert_eq!(lanes(&e, out), [49.0, 98.0, 147.0]);
        e.call(0x00a2_7d60, &args![2.0f32, key, 0u32, out]);
        // 2 + (6 + 24)2 = 62.
        assert_eq!(lanes(&e, out), [62.0, 124.0, 186.0]);
        e.call(0x00a2_7e40, &args![2.0f32, key, 0u32, out]);
        // 6 + 48 = 54.
        assert_eq!(lanes(&e, out), [54.0, 108.0, 162.0]);
    }

    #[test]
    fn fn_00a27ee0_fills_the_coefficients_of_all_but_the_last_key() {
        let mut e = Engine::new();
        constants(&mut e);
        let keys = e.mem.alloc(0xc0);
        key_at(&mut e, keys, 1.0, 0.0, 2.0, 0.0, 0.0);
        key_at(&mut e, keys + 0x40, 5.0, 3.0, 0.0, 9.0, 9.0);
        key_at(&mut e, keys + 0x80, 0.0, 0.0, 0.0, 7.0, 7.0);
        e.call(0x00a2_7ee0, &args![keys, 1u32, 0x40u32]);
        assert_eq!(lanes(&e, keys + 0x28), [0.0; 3], "one key: nothing");
        e.call(0x00a2_7ee0, &args![keys, 2u32, 0x40u32]);
        // d = 4: a = 12 - (3 + 4) = 5, b = (3 + 2) - 8 = -3.
        assert_eq!(lanes(&e, keys + 0x28), [5.0, 10.0, 15.0]);
        assert_eq!(lanes(&e, keys + 0x34), [-3.0, -6.0, -9.0]);
        assert_eq!(lanes(&e, keys + 0x68), [9.0, 18.0, 27.0], "last key kept");
    }

    #[test]
    fn fn_00a28060_takes_the_largest_curvature() {
        let mut e = Engine::new();
        constants(&mut e);
        e.map(CURVATURE_SAMPLES, 4);
        e.map(CURVATURE_STEP, 4);
        e.set_global(CURVATURE_SAMPLES, 1i32);
        e.set_global(CURVATURE_STEP, 1.0f32);
        e.register(CI_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).sqrt(),
            ..Ret::default()
        });
        let keys = e.mem.alloc(0x80);
        // d1 = out + (2a + 3t b)t, d2 = 2a + 6t b with per-lane x, 0, 0 by
        // giving the y and z lanes separate values.
        for lane in 0..3u32 {
            e.mem
                .set_f32(keys + 0x1c + 4 * lane, [1.0, 0.0, 0.0][lane as usize]);
            e.mem
                .set_f32(keys + 0x28 + 4 * lane, [0.0, 1.0, 0.0][lane as usize]);
        }
        // t = 0: d1 = (1, 0, 0), d2 = (0, 2, 0): |d1 x d2| / |d1|^2 = 2.
        // t = 1: d1 = (1, 2, 0), d2 = (0, 2, 0): 2 / 5.
        assert_eq!(e.call(0x00a2_8060, &args![keys, 2u32]).f32(), 2.0);
        assert_eq!(e.call(0x00a2_8060, &args![keys, 1u32]).f32(), 0.0);
        e.set_global(CURVATURE_SAMPLES, -1i32);
        assert_eq!(e.call(0x00a2_8060, &args![keys, 2u32]).f32(), 0.0);
    }
}

#[cfg(test)]
mod n006_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    #[test]
    fn fn_00a276a0_copies_the_ends_or_interpolates() {
        let mut e = Engine::new();
        e.map(NO_KEYS_POINT, 12);
        floats(&mut e, NO_KEYS_POINT, &[7.0, 8.0, 9.0]);
        e.register(POS_KEY_GEN_INTERP, |e, a| {
            assert_eq!(e.mem.u32(a[5]), 0);
            e.mem.set_f32(a[0], 42.0);
            a[0].into_ret()
        });
        // Two 16-byte keys: time 1 value (1, 2, 3) and time 3 value (4, 5, 6).
        let keys = e.mem.alloc(0x20);
        floats(&mut e, keys, &[1.0, 1.0, 2.0, 3.0, 3.0, 4.0, 5.0, 6.0]);
        let result = e.mem.alloc(12);
        let call = |e: &mut Engine, t: f32, count: u32| {
            let answer = e
                .call(0x00a2_76a0, &args![result, t, keys, 2u32, count, 0x110u32])
                .u32();
            assert_eq!(answer, result);
            read(e, result, 3)
        };
        assert_eq!(call(&mut e, 2.0, 0), vec![7.0, 8.0, 9.0]);
        assert_eq!(call(&mut e, 0.5, 2), vec![1.0, 2.0, 3.0]);
        assert_eq!(call(&mut e, 4.0, 2), vec![4.0, 5.0, 6.0]);
        e.call_log = Some(vec![]);
        assert_eq!(call(&mut e, 2.0, 2)[0], 42.0);
        let interp = calls_to(&e, POS_KEY_GEN_INTERP);
        assert_eq!(interp[0][..5], [result, 2.0f32.to_bits(), keys, 2, 2]);
        assert_eq!(interp[0][6], 0x110);
    }

    #[test]
    fn fn_00a27760_compares_the_value_after_the_time() {
        let mut e = Engine::new();
        e.register(KEY_TIMES_EQUAL, |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        floats(&mut e, a, &[1.0, 2.0, 3.0, 4.0]);
        floats(&mut e, b, &[1.0, 2.0, 3.0, 4.0]);
        assert!(e.call(0x00a2_7760, &args![a, b]).bool());
        e.mem.set_f32(b + 0xc, 5.0);
        assert!(!e.call(0x00a2_7760, &args![a, b]).bool());
        e.mem.set_f32(b + 0xc, 4.0);
        e.mem.set_f32(b, 9.0);
        assert!(!e.call(0x00a2_7760, &args![a, b]).bool());
    }

    /// An engine for `fn_00a277c0`: derivative doubles (type 2) answering `d1` and `d2`, a
    /// square root, the flatness bound and a unitize double.
    fn frame_engine(d1: [f32; 3], d2: [f32; 3]) -> Engine {
        const FIRST: u32 = 0x0300_0100;
        const SECOND: u32 = 0x0300_0110;
        let mut e = Engine::new();
        e.map(FIRST_DERIVATIVES, 0x10);
        e.map(SECOND_DERIVATIVES, 0x10);
        e.mem.set_u32(FIRST_DERIVATIVES + 8, FIRST);
        e.mem.set_u32(SECOND_DERIVATIVES + 8, SECOND);
        e.map(FLAT_CURVATURE, 4);
        e.mem.set_f32(FLAT_CURVATURE, 1e-6);
        e.register_double(FIRST, move |e, a| {
            floats(e, a[3], &d1);
            Ret::default()
        });
        e.register_double(SECOND, move |e, a| {
            floats(e, a[3], &d2);
            Ret::default()
        });
        e.register(CI_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e.register(UNITIZE, |e, a| {
            let v = read(e, a[0], 3);
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            let unit: Vec<f32> = v.iter().map(|c| c / length).collect();
            floats(e, a[0], &unit);
            Ret::default()
        });
        e
    }

    fn frame(e: &mut Engine) -> (Vec<f32>, Vec<f32>, Vec<f32>, f32) {
        let out = e.mem.alloc(0x28);
        e.call(
            0x00a2_77c0,
            &args![
                0.5f32,
                0x7000u32,
                0x7040u32,
                2u32,
                out,
                out + 0xc,
                out + 0x18,
                out + 0x24
            ],
        );
        (
            read(e, out, 3),
            read(e, out + 0xc, 3),
            read(e, out + 0x18, 3),
            e.mem.f32(out + 0x24),
        )
    }

    #[test]
    fn fn_00a277c0_builds_the_frame_of_a_bent_curve() {
        let mut e = frame_engine([2.0, 0.0, 0.0], [0.0, 3.0, 0.0]);
        e.call_log = Some(vec![]);
        let (tangent, normal, binormal, curvature) = frame(&mut e);
        assert_eq!(tangent, vec![1.0, 0.0, 0.0]);
        // |d1 x d2| / |d1|^2 = 6 / 4; the normal |d1|^2 d2 - (d1 . d2) d1 = (0, 12, 0),
        // made unit length.
        assert_eq!(curvature, 1.5);
        assert_eq!(normal, vec![0.0, 1.0, 0.0]);
        assert_eq!(binormal, vec![0.0, 0.0, 1.0]);
        let first = calls_to(&e, 0x0300_0100);
        assert_eq!(first[0][..3], [0.5f32.to_bits(), 0x7000, 0x7040]);
    }

    #[test]
    fn fn_00a277c0_picks_a_normal_for_a_straight_curve() {
        // d2 along d1: no curvature, the normal from the tangent's x and y.
        let mut e = frame_engine([2.0, 0.0, 0.0], [4.0, 0.0, 0.0]);
        let (tangent, normal, binormal, curvature) = frame(&mut e);
        assert_eq!(tangent, vec![1.0, 0.0, 0.0]);
        assert_eq!(curvature, 0.0);
        assert_eq!(normal, vec![0.0, -1.0, 0.0]);
        assert_eq!(binormal, vec![0.0, 0.0, -1.0]);
        // Along z: the tangent's x and y are both small, so the normal is (0, -tz, ty).
        let mut e = frame_engine([0.0, 0.0, 2.0], [0.0, 0.0, 1.0]);
        let (tangent, normal, _, curvature) = frame(&mut e);
        assert_eq!(tangent, vec![0.0, 0.0, 1.0]);
        assert_eq!(curvature, 0.0);
        assert_eq!(normal, vec![0.0, -1.0, 0.0]);
    }

    #[test]
    fn fn_00a27b60_copies_a_position_key() {
        let mut e = Engine::new();
        let source = e.mem.alloc(KEY_SIZE);
        for i in 0..16 {
            e.mem.set_u32(source + 4 * i, 0x7f80_0001 + i);
        }
        let destination = e.mem.alloc(KEY_SIZE);
        e.call(0x00a2_7b60, &args![destination, source]);
        // The time through the x87 stack (quieted), the rest as words.
        assert_eq!(e.mem.u32(destination), 0x7fc0_0001);
        for i in 1..16 {
            assert_eq!(e.mem.u32(destination + 4 * i), 0x7f80_0001 + i);
        }
    }

    #[test]
    fn fn_00a27bd0_compares_tangents_and_the_last_point() {
        let mut e = Engine::new();
        e.register(KEY_TIMES_EQUAL, |_, _| 1u32.into_ret());
        e.register(POINTS_EQUAL, |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let a = e.mem.alloc(KEY_SIZE);
        let b = e.mem.alloc(KEY_SIZE);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_7bd0, &args![a, b]).bool());
        assert_eq!(calls_to(&e, POINTS_EQUAL), vec![vec![a + 0x1c, b + 0x1c]]);
        e.mem.set_f32(b + 0x1c, 1.0);
        assert!(!e.call(0x00a2_7bd0, &args![a, b]).bool());
        e.mem.set_f32(b + 0x1c, 0.0);
        e.mem.set_f32(b + 0x14, 1.0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a2_7bd0, &args![a, b]).bool());
        assert!(calls_to(&e, POINTS_EQUAL).is_empty());
    }

    #[test]
    fn fn_00a27c40_allocates_counted_position_keys() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_7c40, &args![2u32]).u32();
        assert_eq!(e.mem.u32(keys - 4), 2);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a2_7c40, &args![0x0400_0000u32]).u32(), 0);
        let sizes: Vec<u32> = calls_to(&e, MEMORY_ALLOCATE).iter().map(|c| c[1]).collect();
        assert_eq!(sizes, vec![0x84, 0xffff_ffff]);
    }

    #[test]
    fn fn_00a28260_writes_each_position_key() {
        let mut e = Engine::new();
        e.register(WRITE_POINT_KEY, |_, _| Ret::default());
        e.register(WRITE_THREE_FLOATS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_8260, &args![0x6000u32, 0x5000u32, 2u32]);
        assert_eq!(
            calls_to(&e, WRITE_POINT_KEY),
            vec![vec![0x6000, 0x5000], vec![0x6000, 0x5040]]
        );
        assert_eq!(
            calls_to(&e, WRITE_THREE_FLOATS),
            vec![
                vec![0x5010, 0x6000],
                vec![0x501c, 0x6000],
                vec![0x5050, 0x6000],
                vec![0x505c, 0x6000]
            ]
        );
    }

    /// Doubles for `fn_00a282a0`: allocator, copy, index finder (`found`), the value at the
    /// time (7, 8, 9) and tangents derived from the lane's previous value; the constants the
    /// coefficient fill reads.
    fn position_insert_engine(found: Option<u32>) -> Engine {
        let mut e = Engine::new();
        e.map(TWO, 8);
        e.map(THREE, 8);
        e.set_global(TWO, 2.0f64);
        e.set_global(THREE, 3.0f64);
        e.map(ZERO_POINT, 12);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            a[0].into_ret()
        });
        e.register_double(FIND_INSERT_INDEX, move |e, a| match found {
            Some(index) => {
                e.mem.set_u32(a[3], index);
                1u32.into_ret()
            }
            None => 0u32.into_ret(),
        });
        e.register(POS_KEY_GEN_INTERP, |e, a| {
            floats(e, a[0], &[7.0, 8.0, 9.0]);
            a[0].into_ret()
        });
        e.register(NEW_KEY_TANGENTS, |e, a| {
            let lane = f32::from_bits(a[0]);
            e.mem.set_f32(a[2], lane + 10.0);
            e.mem.set_f32(a[5], lane + 20.0);
            e.mem.set_f32(a[8], lane + 30.0);
            e.mem.set_f32(a[9], lane + 40.0);
            Ret::default()
        });
        e
    }

    #[test]
    fn fn_00a282a0_inserts_a_position_key_between_two() {
        let mut e = position_insert_engine(Some(1));
        let block = e.mem.alloc(4 + 2 * KEY_SIZE);
        e.mem.set_u32(block, 2);
        let old = block + 4;
        floats(&mut e, old, &[1.0, 1.0, 2.0, 3.0]);
        floats(&mut e, old + KEY_SIZE, &[3.0, 4.0, 5.0, 6.0]);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, old);
        e.mem.set_u32(slots + 4, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_82a0, &args![2.0f32, slots, slots + 4]).bool());
        assert_eq!(e.mem.u32(slots + 4), 3);
        let new = e.mem.u32(slots);
        let key = new + KEY_SIZE;
        assert_eq!(read(&e, key, 4), vec![2.0, 7.0, 8.0, 9.0]);
        assert_eq!(read(&e, key + 0x10, 3), vec![31.0, 32.0, 33.0]);
        assert_eq!(read(&e, key + 0x1c, 3), vec![41.0, 42.0, 43.0]);
        assert_eq!(read(&e, new + 0x1c, 3), vec![11.0, 12.0, 13.0]);
        assert_eq!(read(&e, key + KEY_SIZE + 0x10, 3), vec![21.0, 22.0, 23.0]);
        let tangents = calls_to(&e, NEW_KEY_TANGENTS);
        assert_eq!(tangents.len(), 3);
        for (lane, call) in tangents.iter().enumerate() {
            assert_eq!(f32::from_bits(call[0]), 1.0 + lane as f32);
            assert_eq!(f32::from_bits(call[1]), 1.0);
            assert_eq!(f32::from_bits(call[3]), 4.0 + lane as f32);
            assert_eq!(f32::from_bits(call[4]), 3.0);
            assert_eq!(f32::from_bits(call[6]), 2.0);
            assert_eq!(f32::from_bits(call[7]), 7.0 + lane as f32);
        }
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, block]]);
    }

    #[test]
    fn fn_00a282a0_appends_or_refuses() {
        let mut e = position_insert_engine(Some(1));
        let block = e.mem.alloc(4 + KEY_SIZE);
        e.mem.set_u32(block, 1);
        floats(&mut e, block + 4, &[1.0, 1.0, 2.0, 3.0]);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, block + 4);
        e.mem.set_u32(slots + 4, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_82a0, &args![2.0f32, slots, slots + 4]).bool());
        assert!(calls_to(&e, NEW_KEY_TANGENTS).is_empty());
        let key = e.mem.u32(slots) + KEY_SIZE;
        // After the last key's time the value is the last key's (no interpolation call).
        assert_eq!(read(&e, key, 4), vec![2.0, 1.0, 2.0, 3.0]);
        assert_eq!(read(&e, key + 0x10, 6), vec![0.0; 6]);
        assert!(calls_to(&e, POS_KEY_GEN_INTERP).is_empty());
        let mut e = position_insert_engine(None);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, 0x1234);
        e.mem.set_u32(slots + 4, 1);
        assert!(!e.call(0x00a2_82a0, &args![2.0f32, slots, slots + 4]).bool());
        assert_eq!((e.mem.u32(slots), e.mem.u32(slots + 4)), (0x1234, 1));
    }

    #[test]
    fn fn_00a286e0_writes_the_identity_quaternion() {
        let mut e = Engine::new();
        e.register(QUATERNION_SET, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        let result = e.mem.alloc(0x10);
        e.call(0x00a2_86e0, &args![0.5f32, 0u32, 0u32, result]);
        assert_eq!(read(&e, result, 4), vec![1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_00a28720_writes_the_time_then_the_quaternion() {
        let mut e = Engine::new();
        e.register(WRITE_KEY_TIME, |_, _| Ret::default());
        e.register(QUATERNION_SAVE_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_8720, &args![0x6000u32, 0x5000u32]);
        assert_eq!(calls_to(&e, WRITE_KEY_TIME), vec![vec![0x6000, 0x5000]]);
        assert_eq!(
            calls_to(&e, QUATERNION_SAVE_BINARY),
            vec![vec![0x5004, 0x6000]]
        );
    }
}
