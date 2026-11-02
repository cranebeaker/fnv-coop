//! `NiAnimation/run_00a289b0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a28cb0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3669` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a28cb0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3669) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d50, 0x0);
    e.mem.set_u32(0x011f_3c30, 0x0);
    e.mem.set_u32(0x011f_3ba0, 0x0);
    e.mem.set_u32(0x011f_3840, 0x0);
    e.mem.set_u8(0x011f_3669, 0x1);
    e.mem.set_u32(0x011f_3de0, 0xe7cd40);
    e.mem.set_u32(0x011f_3a80, 0xa29680);
    e.mem.set_u32(0x011f_39f0, 0xa29680);
    e.mem.set_u32(0x011f_3960, 0xe7cd40);
    e.mem.set_u32(0x011f_38d0, 0xa29680);
    e.mem.set_u8(0x011f_3764, 0x14);
    e.mem.set_u32(0x011f_3cc0, 0xa286e0);
    e.mem.set_u32(0x011f_3b10, 0xa28a80);
    e.mem.set_u32(0x011f_37b0, 0xa28ab0);
    1
}

// Translated from 00a29110 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f367c` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a29110(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_367c) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d58, 0x0);
    e.mem.set_u32(0x011f_3c38, 0x0);
    e.mem.set_u32(0x011f_3ba8, 0x0);
    e.mem.set_u8(0x011f_367c, 0x1);
    e.mem.set_u32(0x011f_3de8, 0xa290a0);
    e.mem.set_u32(0x011f_3a88, 0xa28ed0);
    e.mem.set_u32(0x011f_39f8, 0xa28d40);
    e.mem.set_u32(0x011f_3968, 0xa28f00);
    e.mem.set_u32(0x011f_38d8, 0xa2a600);
    e.mem.set_u8(0x011f_3766, 0x24);
    e.mem.set_u32(0x011f_3cc8, 0xa28d90);
    e.mem.set_u32(0x011f_3b18, 0xa28ea0);
    e.mem.set_u32(0x011f_37b8, 0xa28dc0);
    e.mem.set_u32(0x011f_3848, 0xa28f50);
    1
}

// Translated from 00a291f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a291f0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa291d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa291d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa291d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa291a0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0x0);
}

/// `NiQuaternion::operator-` (Xbox PDB, unary, returned by value), `00a6dbd0`.
const QUATERNION_NEGATE: u32 = 0x00a6_dbd0;
/// `float` 0.0 (`01011d78`), the sign test of the dot product.
const ZERO_FLOAT: u32 = 0x0101_1d78;
/// `float` -1.0 (`01012054`), the lower clamp of `W`.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;

/// Clamps the `float` at `at` as the exe's clamp blocks do: the value
/// (through `FLD`/`FSTP`/`FLD` of a local) below `low` stores `low`,
/// otherwise above 1 stores 1; NaN stores nothing.
fn clamp_w(e: &mut Engine, at: u32, low: f64) {
    let v = x87_load32(x87_store32(x87_load32(e.mem.u32(at))));
    if v < low {
        e.mem.set_u32(at, x87_store32(low));
    } else if v > 1.0 {
        e.mem.set_u32(at, x87_store32(1.0));
    }
}

// Translated from 00a28ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Prepares an array of `count` rotation keys of `stride` bytes (`cdecl`;
/// no Xbox PDB name; the quaternion at +4 of each key): first each key's
/// quaternion whose dot product with the previous key's is negative is
/// negated (through `NiQuaternion::operator-`, `00a6dbd0`), so neighbours
/// lie in one hemisphere; then every key's `W` is clamped to `[-1, 1]`,
/// four keys at a time and then the rest. As in the exe, the first loop
/// runs `count - 1` times without a guard for `count == 0`.
pub fn fn_00a28ab0(e: &mut Engine, keys: Ptr, count: u32, stride: u8) {
    let base = keys.addr();
    let stride = u32::from(stride);
    let mut remaining = count.wrapping_sub(1);
    let mut next = base.wrapping_add(stride).wrapping_add(4);
    let mut prev = base.wrapping_add(0xc);
    let zero = x87_m32(e.mem.u32(ZERO_FLOAT));
    while remaining != 0 {
        let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
        let m = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
        let a = x87_mul(ld(e, next.wrapping_add(4)), m(e, prev.wrapping_sub(4)));
        let b = x87_mul(ld(e, prev.wrapping_sub(8)), m(e, next));
        let sum = x87_add(a, b);
        let c = x87_mul(ld(e, next.wrapping_add(8)), m(e, prev));
        let sum = x87_add(sum, c);
        let d = x87_mul(ld(e, next.wrapping_add(0xc)), m(e, prev.wrapping_add(4)));
        let dot = x87_load32(x87_store32(x87_add(sum, d)));
        // FCOMP 0.0; TEST AH,0x5; JP: only "below" negates.
        if dot < zero {
            e.with_stack(0x10, |e, tmp| {
                let r = e.call(QUATERNION_NEGATE, &args![next, tmp]).u32();
                for i in 0..4 {
                    let w = e.mem.u32(r.wrapping_add(4 * i));
                    e.mem.set_u32(next.wrapping_add(4 * i), w);
                }
            });
        }
        next = next.wrapping_add(stride);
        prev = prev.wrapping_add(stride);
        remaining = remaining.wrapping_sub(1);
    }
    let low = x87_load32(e.mem.u32(MINUS_ONE_FLOAT));
    let mut done = 0u32;
    if count as i32 >= 4 {
        let mut groups = (count.wrapping_sub(4) >> 2).wrapping_add(1);
        done = groups.wrapping_mul(4);
        let step = stride.wrapping_mul(4);
        let mut p0 = base.wrapping_add(4);
        let mut p1 = base.wrapping_add(stride).wrapping_add(4);
        let mut p2 = base.wrapping_add(stride.wrapping_mul(2)).wrapping_add(4);
        let mut p3 = base.wrapping_add(stride.wrapping_mul(3)).wrapping_add(4);
        while groups != 0 {
            clamp_w(e, p0, low);
            clamp_w(e, p1, low);
            clamp_w(e, p2, low);
            clamp_w(e, p3, low);
            p0 = p0.wrapping_add(step);
            p1 = p1.wrapping_add(step);
            p2 = p2.wrapping_add(step);
            p3 = p3.wrapping_add(step);
            groups = groups.wrapping_sub(1);
        }
    }
    if done < count {
        let mut p = base.wrapping_add(stride.wrapping_mul(done)).wrapping_add(4);
        let mut left = count - done;
        while left != 0 {
            clamp_w(e, p, low);
            p = p.wrapping_add(stride);
            left -= 1;
        }
    }
}

/// `NiRotKey::GenInterp` (`cdecl`, (result, time, keys, type, count, &last index, size);
/// answers the result).
const ROT_KEY_GEN_INTERP: u32 = 0x00a2_8740;
/// The four words a rotation key list without keys answers (`011f366c`).
const NO_KEYS_QUATERNION: u32 = 0x011f_366c;
/// `00a3d520` (`cdecl`, two keys): whether the keys' times are equal.
const KEY_TIMES_EQUAL: u32 = 0x00a3_d520;
/// `00a92660` (`thiscall` on a quaternion, the other one): whether their four floats are
/// equal.
const QUATERNIONS_EQUAL: u32 = 0x00a9_2660;
/// `00a6e430` (`thiscall` on the result, (time, p, a, b, q)): the spherical quadrangle
/// interpolation of two keys' quaternions and intermediates.
const QUATERNION_SQUAD: u32 = 0x00a6_e430;
/// `00a6dd50` (`cdecl`, (result, previous, quaternion, next); answers the result): the
/// intermediate quaternion between three.
const QUATERNION_INTERMEDIATE: u32 = 0x00a6_dd50;
/// `00a28720` (unit `run_00a276a0`, `cdecl`, (stream, key)): writes a rotation key's time
/// and quaternion.
const WRITE_ROTATION_KEY: u32 = 0x00a2_8720;
/// The memory manager singleton and its `Allocate` and `Free` (`thiscall`, one word).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// The compiler's array constructor iterator (`stdcall`, (array, element size, count,
/// constructor)) and the constructor these 0x24-byte keys are given (`00a2bdb0`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const ROTATION_KEY_CONSTRUCT: u32 = 0x00a2_bdb0;
/// `00a29640` (`thiscall` on a key, the stream): reads a key's time and its four floats.
const READ_ROTATION_KEY: u32 = 0x00a2_9640;
/// `00a3d5d0` (`cdecl`, (time, keys, count, &index, size)): finds where a key at `time`
/// goes (the index of the first key not before it); false when a key has that time.
const FIND_INSERT_INDEX: u32 = 0x00a3_d5d0;
/// `_memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// The size of these rotation keys (time, quaternion, intermediate quaternion).
const ROTATION_KEY_SIZE: u32 = 0x24;
/// The byte `00a291d0` copies (`011f367d`).
const STEP_VALUE: u32 = 0x011f_367d;

// Translated from 00a289b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (result, time, keys, type, count, size); answers
/// `result`): the rotation keys' quaternion at `time` written to `result`. No keys: the four
/// words at `011f366c`; before the first key's time: the first key's quaternion (+4); after
/// the time of the last key (at `(count - 1) * size`, the size's low byte): the last key's
/// (plain word copies); otherwise `NiRotKey::GenInterp` (`00a28740`), the count's stack word
/// (set to 0) as its last-index word.
pub fn fn_00a289b0(
    e: &mut Engine,
    result: Ptr,
    time: f32,
    keys: Ptr,
    kind: u32,
    count: u32,
    size: u32,
) -> Ptr {
    let copy4 = |e: &mut Engine, from: u32| {
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(from.wrapping_add(offset));
            e.mem.set_u32(result.addr() + offset, word);
        }
    };
    if count == 0 {
        copy4(e, NO_KEYS_QUATERNION);
        return result;
    }
    let t = x87_load32(time.to_bits());
    if x87_load32(e.mem.u32(keys.addr())) > t {
        copy4(e, keys.addr() + 4);
        return result;
    }
    let last = count.wrapping_sub(1).wrapping_mul(size & 0xff);
    let last = keys.addr().wrapping_add(last);
    if x87_load32(e.mem.u32(last)) < t {
        copy4(e, last.wrapping_add(4));
        return result;
    }
    e.with_stack(4, |e, last_index| {
        e.mem.set_u32(last_index.addr(), 0);
        e.call(
            ROT_KEY_GEN_INTERP,
            &args![result, x87_store32(t), keys, kind, count, last_index, size],
        );
    });
    result
}

// Translated from 00a28a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two keys): whether two rotation keys are equal: their
/// times (`00a3d520`), then their quaternions at +4 (`00a92660`).
pub fn fn_00a28a80(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    if !e.call(KEY_TIMES_EQUAL, &args![first, second]).bool() {
        return false;
    }
    e.call(
        QUATERNIONS_EQUAL,
        &args![first.addr() + 4, second.addr() + 4],
    )
    .bool()
}

// Translated from 00a28d40 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, (destination, source)): copies a 0x24-byte
/// rotation key, its time through the x87 stack (`FLD`/`FSTP`, which quiets a signalling
/// NaN) and the eight words after it as plain words.
pub fn fn_00a28d40(e: &mut Engine, destination: Ptr, source: Ptr) {
    let time = x87_quiet_bits(e.mem.u32(source.addr()));
    e.mem.set_u32(destination.addr(), time);
    for offset in (4..ROTATION_KEY_SIZE).step_by(4) {
        let word = e.mem.u32(source.addr() + offset);
        e.mem.set_u32(destination.addr() + offset, word);
    }
}

// Translated from 00a28d90 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, (time, key, next key, result), the shape of the
/// interpolation functions): the spherical quadrangle interpolation (`00a6e430`, on `result`)
/// at `time` of the key's quaternion (+4) and intermediate (+0x14) and the next key's
/// intermediate and quaternion.
pub fn fn_00a28d90(e: &mut Engine, time: f32, key: Ptr, next: Ptr, result: Ptr) {
    let t = x87_quiet_bits(time.to_bits());
    e.call(
        QUATERNION_SQUAD,
        &args![
            result,
            t,
            key.addr().wrapping_add(4),
            key.addr().wrapping_add(0x14),
            next.addr().wrapping_add(0x14),
            next.addr().wrapping_add(4)
        ],
    );
}

// Translated from 00a28dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (keys, count, size)): prepares `count` 0x24-byte
/// rotation keys for squad interpolation when there are at least two: `00a28ab0` (one
/// hemisphere, clamped `W`), then each key's intermediate quaternion at +0x14 from its
/// neighbours (`00a6dd50` into a stack quaternion, copied from the pointer it answers): the
/// first key from (itself, itself, the second), the middle keys from (previous, itself,
/// next), the last from (the one before, itself, itself).
pub fn fn_00a28dc0(e: &mut Engine, keys: Ptr, count: u32, size: u32) {
    if count < 2 {
        return;
    }
    let base = keys.addr();
    fn_00a28ab0(e, keys, count, size as u8);
    e.with_stack(0x10, |e, quaternion| {
        let intermediate = |e: &mut Engine, previous: u32, current: u32, next: u32, to: u32| {
            let made = e
                .call(
                    QUATERNION_INTERMEDIATE,
                    &args![quaternion, previous, current, next],
                )
                .u32();
            for offset in [0u32, 4, 8, 0xc] {
                let word = e.mem.u32(made.wrapping_add(offset));
                e.mem.set_u32(to.wrapping_add(offset), word);
            }
        };
        let second = base.wrapping_add(0x28);
        intermediate(e, base + 4, base + 4, second, base + 0x14);
        let last = count - 1;
        if last > 1 {
            let mut current = second;
            for _ in 0..last - 1 {
                let next = current.wrapping_add(ROTATION_KEY_SIZE);
                intermediate(
                    e,
                    current.wrapping_sub(ROTATION_KEY_SIZE),
                    current,
                    next,
                    current.wrapping_add(0x10),
                );
                current = next;
            }
        }
        let final_key = base.wrapping_add(last.wrapping_mul(ROTATION_KEY_SIZE));
        let before = base
            .wrapping_add(count.wrapping_mul(ROTATION_KEY_SIZE))
            .wrapping_sub(0x44);
        intermediate(
            e,
            before,
            final_key.wrapping_add(4),
            final_key.wrapping_add(4),
            final_key.wrapping_add(0x14),
        );
    });
}

// Translated from 00a28ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two keys): whether two 0x24-byte rotation keys are
/// equal: time and quaternion (`00a28a80`), then the intermediates at +0x14 (`00a92660`).
pub fn fn_00a28ea0(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    if !fn_00a28a80(e, first, second) {
        return false;
    }
    e.call(
        QUATERNIONS_EQUAL,
        &args![first.addr() + 0x14, second.addr() + 0x14],
    )
    .bool()
}

// Translated from 00a28ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, (stream, keys, count)): writes `count` 0x24-byte
/// rotation keys, each one's time and quaternion (`00a28720`).
pub fn fn_00a28ed0(e: &mut Engine, stream: Ptr, keys: Ptr, count: u32) {
    let mut key = keys.addr();
    for _ in 0..count {
        e.call(WRITE_ROTATION_KEY, &args![stream, key]);
        key = key.wrapping_add(ROTATION_KEY_SIZE);
    }
}

/// The compiler's array new of `count` 0x24-byte rotation keys from the memory manager:
/// `count * 0x24 + 4` bytes (each step all ones on overflow), the count in the first word,
/// each key constructed (`00401050` with `00a2bdb0`). Answers the keys, or null.
fn new_rotation_keys(e: &mut Engine, count: u32) -> u32 {
    let bytes = count
        .checked_mul(ROTATION_KEY_SIZE)
        .unwrap_or(u32::MAX)
        .checked_add(4)
        .unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    if block == 0 {
        return 0;
    }
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT,
        &args![block + 4, ROTATION_KEY_SIZE, count, ROTATION_KEY_CONSTRUCT],
    );
    block + 4
}

// Translated from 00a28f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the count): makes `count` constructed 0x24-byte
/// rotation keys from the memory manager (`count * 0x24 + 4` bytes, saturating, the count in
/// the first word, each key through `00a2bdb0`). Answers the keys, or null when the
/// allocation fails.
pub fn fn_00a28f00(e: &mut Engine, count: u32) -> u32 {
    new_rotation_keys(e, count)
}

// Translated from 00a28f50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, (time, &keys, &count); a bool): inserts a key at
/// `time` into a list of 0x24-byte rotation keys. `00a3d5d0` finds the index; when a key
/// already has that time nothing changes (false). Otherwise a constructed list one longer
/// (`00401050`/`00a2bdb0`) takes the keys before the index, the new key and the keys after it
/// (`_memcpy`); the new key's time is `time` and its quaternion the old keys' value there
/// (`00a289b0`, type 2, size 0x24). The count grows by one, the old list is freed
/// (`00aa4060`, from its count word), the new one stored and its intermediates computed
/// (`00a28dc0`). Answers true.
pub fn fn_00a28f50(e: &mut Engine, time: f32, keys: Ptr, count: Ptr) -> bool {
    let old = e.mem.u32(keys.addr());
    let time_word = x87_quiet_bits(time.to_bits());
    // The stack locals from ESP-0x14 up: the index (+0) and the quaternion at the time (+4).
    e.with_stack(0x14, |e, locals| {
        let index_word = locals.addr();
        let at_time = index_word + 4;
        let n = e.mem.u32(count.addr());
        let found = e
            .call(
                FIND_INSERT_INDEX,
                &args![time_word, old, n, index_word, ROTATION_KEY_SIZE],
            )
            .bool();
        if !found {
            return false;
        }
        let grown = e.mem.u32(count.addr()).wrapping_add(1);
        let new = new_rotation_keys(e, grown);
        let index = e.mem.u32(index_word);
        e.call(
            MEMCPY,
            &args![new, old, index.wrapping_mul(ROTATION_KEY_SIZE)],
        );
        let n = e.mem.u32(count.addr());
        let index = e.mem.u32(index_word);
        if n > index {
            let offset = index.wrapping_mul(ROTATION_KEY_SIZE);
            e.call(
                MEMCPY,
                &args![
                    offset.wrapping_add(new).wrapping_add(ROTATION_KEY_SIZE),
                    offset.wrapping_add(old),
                    (n - index).wrapping_mul(ROTATION_KEY_SIZE)
                ],
            );
        }
        let n = e.mem.u32(count.addr());
        fn_00a289b0(
            e,
            Ptr::new(at_time),
            f32::from_bits(time_word),
            Ptr::new(old),
            2,
            n,
            ROTATION_KEY_SIZE,
        );
        let key = e
            .mem
            .u32(index_word)
            .wrapping_mul(ROTATION_KEY_SIZE)
            .wrapping_add(new);
        e.mem.set_u32(key, x87_store32(x87_load32(time.to_bits())));
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(at_time + offset);
            e.mem.set_u32(key.wrapping_add(4 + offset), word);
        }
        let n = e.mem.u32(count.addr());
        e.mem.set_u32(count.addr(), n.wrapping_add(1));
        if old != 0 {
            e.call(MEMORY_FREE, &args![MEMORY_MANAGER, old.wrapping_sub(4)]);
        }
        e.mem.set_u32(keys.addr(), new);
        let n = e.mem.u32(count.addr());
        fn_00a28dc0(e, Ptr::new(new), n, ROTATION_KEY_SIZE);
        true
    })
}

// Translated from 00a290a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, (stream, count)): makes `count` constructed
/// 0x24-byte rotation keys (as `00a28f00`) and reads each one's time and quaternion
/// (`00a29640`, on the key) from `stream`; the reads run even when the allocation failed (on
/// keys from null, as in the exe). Answers the keys, or null.
pub fn fn_00a290a0(e: &mut Engine, stream: Ptr, count: u32) -> u32 {
    let keys = new_rotation_keys(e, count);
    let mut key = keys;
    for _ in 0..count {
        e.call(READ_ROTATION_KEY, &args![key, stream]);
        key = key.wrapping_add(ROTATION_KEY_SIZE);
    }
    keys
}

// Translated from 00a291a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two keys): whether two step keys are equal: their
/// times (`00a3d520`), then their bytes at +4.
pub fn fn_00a291a0(e: &mut Engine, first: Ptr, second: Ptr) -> bool {
    if !e.call(KEY_TIMES_EQUAL, &args![first, second]).bool() {
        return false;
    }
    e.mem.u8(first.addr() + 4) == e.mem.u8(second.addr() + 4)
}

// Translated from 00a291d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, four words, the shape of the interpolation
/// functions: (time, key, next key, result)): stores the byte at `011f367d` at `result`; the
/// other words are not read.
pub fn fn_00a291d0(e: &mut Engine, _unused_0: u32, _unused_1: u32, _unused_2: u32, result: Ptr) {
    let value = e.mem.u8(STEP_VALUE);
    e.mem.set_u8(result.addr(), value);
}

// Translated from 00a291e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not create (`cdecl`, two words (keys, count); answers in `ST0`):
/// answers 0 (`FLDZ`); the words are not read. It is a curvature function of the tables at
/// `011f3d20` (next to `00a28060`); the two words are confirmed by the caller through that
/// table, `NiPathInterpolator`'s `00a3ee80` (`PUSH count; PUSH keys; CALL [..]; ADD ESP,8`).
pub fn fn_00a291e0(_e: &mut Engine, _unused_0: u32, _unused_1: u32) -> f32 {
    0.0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a28cb0, fn_00a28cb0() -> u32),
        entry!(0x00a29110, fn_00a29110() -> u32),
        entry!(0x00a291f0, fn_00a291f0(u32, u32)),
        entry!(0x00a28ab0, fn_00a28ab0(Ptr, u32, u8)),
        entry!(0x00a2_89b0, fn_00a289b0(Ptr, f32, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00a2_8a80, fn_00a28a80(Ptr, Ptr) -> bool),
        entry!(0x00a2_8d40, fn_00a28d40(Ptr, Ptr)),
        entry!(0x00a2_8d90, fn_00a28d90(f32, Ptr, Ptr, Ptr)),
        entry!(0x00a2_8dc0, fn_00a28dc0(Ptr, u32, u32)),
        entry!(0x00a2_8ea0, fn_00a28ea0(Ptr, Ptr) -> bool),
        entry!(0x00a2_8ed0, fn_00a28ed0(Ptr, Ptr, u32)),
        entry!(0x00a2_8f00, fn_00a28f00(u32) -> u32),
        entry!(0x00a2_8f50, fn_00a28f50(f32, Ptr, Ptr) -> bool),
        entry!(0x00a2_90a0, fn_00a290a0(Ptr, u32) -> u32),
        entry!(0x00a2_91a0, fn_00a291a0(Ptr, Ptr) -> bool),
        entry!(0x00a2_91d0, fn_00a291d0(u32, u32, u32, Ptr)),
        entry!(0x00a2_91e0, fn_00a291e0(u32, u32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `count` keys of `stride` bytes, each with the given quaternion at +4.
    fn keys(e: &mut Engine, stride: u32, quats: &[[f32; 4]]) -> Ptr {
        let base = e.mem.alloc(stride * quats.len() as u32 + 0x10);
        for (i, q) in quats.iter().enumerate() {
            for (j, v) in q.iter().enumerate() {
                e.mem
                    .set_f32(base + stride * i as u32 + 4 + 4 * j as u32, *v);
            }
        }
        Ptr::new(base)
    }

    fn quat(e: &Engine, keys: Ptr, stride: u32, i: u32) -> [f32; 4] {
        [0, 4, 8, 12].map(|o| e.mem.f32(keys.addr() + stride * i + 4 + o))
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x2000);
        e.set_global(ZERO_FLOAT, 0.0f32);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        e.register(QUATERNION_NEGATE, |e, a| {
            for i in 0..4 {
                let v = e.mem.f32(a[0] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, -v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e
    }

    #[test]
    fn keys_opposite_their_neighbour_are_negated() {
        let mut e = engine();
        let k = keys(
            &mut e,
            0x14,
            &[
                [1.0, 0.0, 0.0, 0.0],
                [-0.5, 0.5, 0.0, 0.0],
                [0.5, 0.5, 0.0, 0.0],
            ],
        );
        e.call(0x00a2_8ab0, &args![k, 3u32, 0x14u32]);
        assert_eq!(quat(&e, k, 0x14, 1), [0.5, -0.5, -0.0, -0.0]);
        // The third is compared with the negated second: dot 0, kept.
        assert_eq!(quat(&e, k, 0x14, 2), [0.5, 0.5, 0.0, 0.0]);
    }

    #[test]
    fn every_w_is_clamped_in_groups_of_four_and_then_the_rest() {
        let mut e = engine();
        let w = [2.0f32, -3.0, 0.5, 1.5, -1.5, 0.25];
        // X = 10 keeps every dot product positive: nothing is negated.
        let quats: Vec<[f32; 4]> = w.iter().map(|w| [*w, 10.0, 0.0, 0.0]).collect();
        let k = keys(&mut e, 0x10, &quats);
        e.mem.set_u32(k.addr() + 0x10 * 5 + 4, 0x7fc0_0000);
        e.register(QUATERNION_NEGATE, |_, _| panic!("no key is negated"));
        e.call(0x00a2_8ab0, &args![k, 6u32, 0x10u32]);
        let got: Vec<u32> = (0..6).map(|i| e.mem.u32(k.addr() + 0x10 * i + 4)).collect();
        assert_eq!(
            got,
            vec![
                1.0f32.to_bits(),
                (-1.0f32).to_bits(),
                0.5f32.to_bits(),
                1.0f32.to_bits(),
                (-1.0f32).to_bits(),
                0x7fc0_0000,
            ]
        );
    }

    #[test]
    fn fewer_than_four_keys_go_through_the_tail_loop() {
        let mut e = engine();
        let k = keys(&mut e, 0x10, &[[5.0, 0.0, 0.0, 0.0]]);
        e.call(0x00a2_8ab0, &args![k, 1u32, 0x10u32]);
        assert_eq!(quat(&e, k, 0x10, 0)[0], 1.0);
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

    fn equal_words(e: &mut Engine, a: &[u32]) -> Ret {
        let same = (0..4).all(|i| e.mem.u32(a[0] + 4 * i) == e.mem.u32(a[1] + 4 * i));
        same.into_ret()
    }

    #[test]
    fn fn_00a289b0_copies_the_ends_or_interpolates() {
        let mut e = Engine::new();
        e.map(NO_KEYS_QUATERNION, 0x10);
        floats(&mut e, NO_KEYS_QUATERNION, &[1.0, 0.0, 0.0, 0.0]);
        e.register(ROT_KEY_GEN_INTERP, |e, a| {
            assert_eq!(e.mem.u32(a[5]), 0);
            e.mem.set_f32(a[0], 42.0);
            a[0].into_ret()
        });
        // Two 0x24-byte keys: time 1 quaternion (1, 2, 3, 4), time 3 (5, 6, 7, 8).
        let keys = e.mem.alloc(0x48);
        floats(&mut e, keys, &[1.0, 1.0, 2.0, 3.0, 4.0]);
        floats(&mut e, keys + 0x24, &[3.0, 5.0, 6.0, 7.0, 8.0]);
        let result = e.mem.alloc(0x10);
        let call = |e: &mut Engine, t: f32, count: u32| {
            let answer = e
                .call(0x00a2_89b0, &args![result, t, keys, 2u32, count, 0x24u32])
                .u32();
            assert_eq!(answer, result);
            read(e, result, 4)
        };
        assert_eq!(call(&mut e, 2.0, 0), vec![1.0, 0.0, 0.0, 0.0]);
        assert_eq!(call(&mut e, 0.5, 2), vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(call(&mut e, 4.0, 2), vec![5.0, 6.0, 7.0, 8.0]);
        e.call_log = Some(vec![]);
        assert_eq!(call(&mut e, 2.0, 2)[0], 42.0);
        let interp = calls_to(&e, ROT_KEY_GEN_INTERP);
        assert_eq!(interp[0][..5], [result, 2.0f32.to_bits(), keys, 2, 2]);
        assert_eq!(interp[0][6], 0x24);
    }

    #[test]
    fn fn_00a28a80_and_fn_00a28ea0_compare_times_and_quaternions() {
        let mut e = Engine::new();
        e.register(KEY_TIMES_EQUAL, |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        e.register(QUATERNIONS_EQUAL, equal_words);
        let a = e.mem.alloc(0x24);
        let b = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_8a80, &args![a, b]).bool());
        assert!(e.call(0x00a2_8ea0, &args![a, b]).bool());
        assert_eq!(
            calls_to(&e, QUATERNIONS_EQUAL),
            vec![
                vec![a + 4, b + 4],
                vec![a + 4, b + 4],
                vec![a + 0x14, b + 0x14]
            ]
        );
        e.mem.set_f32(b + 0x18, 1.0);
        assert!(e.call(0x00a2_8a80, &args![a, b]).bool());
        assert!(!e.call(0x00a2_8ea0, &args![a, b]).bool());
        e.mem.set_f32(b, 1.0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a2_8a80, &args![a, b]).bool());
        assert!(!e.call(0x00a2_8ea0, &args![a, b]).bool());
        assert!(calls_to(&e, QUATERNIONS_EQUAL).is_empty());
    }

    #[test]
    fn fn_00a28d40_copies_a_rotation_key() {
        let mut e = Engine::new();
        let source = e.mem.alloc(ROTATION_KEY_SIZE);
        for i in 0..9 {
            e.mem.set_u32(source + 4 * i, 0x7f80_0001 + i);
        }
        let destination = e.mem.alloc(ROTATION_KEY_SIZE);
        e.call(0x00a2_8d40, &args![destination, source]);
        assert_eq!(e.mem.u32(destination), 0x7fc0_0001);
        for i in 1..9 {
            assert_eq!(e.mem.u32(destination + 4 * i), 0x7f80_0001 + i);
        }
    }

    #[test]
    fn fn_00a28d90_squads_the_two_keys() {
        let mut e = Engine::new();
        e.register(QUATERNION_SQUAD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x00a2_8d90,
            &args![0.25f32, 0x5000u32, 0x5024u32, 0x6000u32],
        );
        assert_eq!(
            calls_to(&e, QUATERNION_SQUAD),
            vec![vec![
                0x6000,
                0.25f32.to_bits(),
                0x5004,
                0x5014,
                0x5038,
                0x5028
            ]]
        );
    }

    fn rotation_constants(e: &mut Engine) {
        e.map(ZERO_FLOAT, 4);
        e.map(MINUS_ONE_FLOAT, 4);
        e.mem.set_f32(MINUS_ONE_FLOAT, -1.0);
    }

    #[test]
    fn fn_00a28dc0_computes_each_intermediate() {
        let mut e = Engine::new();
        rotation_constants(&mut e);
        e.register(QUATERNION_INTERMEDIATE, |e, a| {
            // Marks the result with the three inputs' addresses.
            for i in 0..3 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + i as usize]);
            }
            e.mem.set_u32(a[0] + 0xc, 0x77);
            a[0].into_ret()
        });
        let keys = e.mem.alloc(3 * ROTATION_KEY_SIZE);
        for i in 0..3 {
            floats(
                &mut e,
                keys + i * ROTATION_KEY_SIZE + 4,
                &[1.0, 0.0, 0.0, 0.0],
            );
        }
        e.call_log = Some(vec![]);
        e.call(0x00a2_8dc0, &args![keys, 3u32, ROTATION_KEY_SIZE]);
        let q = |i: u32| keys + i * ROTATION_KEY_SIZE + 4;
        let words = |e: &Engine, i: u32| -> Vec<u32> {
            (0..4)
                .map(|w| e.mem.u32(keys + i * ROTATION_KEY_SIZE + 0x14 + 4 * w))
                .collect()
        };
        assert_eq!(words(&e, 0), vec![q(0), q(0), q(1), 0x77]);
        assert_eq!(words(&e, 1), vec![q(0), q(1), q(2), 0x77]);
        assert_eq!(words(&e, 2), vec![q(1), q(2), q(2), 0x77]);
        // Fewer than two keys: nothing.
        e.call_log = Some(vec![]);
        e.call(0x00a2_8dc0, &args![keys, 1u32, ROTATION_KEY_SIZE]);
        assert!(calls_to(&e, QUATERNION_INTERMEDIATE).is_empty());
    }

    #[test]
    fn fn_00a28ed0_writes_each_rotation_key() {
        let mut e = Engine::new();
        e.register(WRITE_ROTATION_KEY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00a2_8ed0, &args![0x6000u32, 0x5000u32, 2u32]);
        assert_eq!(
            calls_to(&e, WRITE_ROTATION_KEY),
            vec![vec![0x6000, 0x5000], vec![0x6000, 0x5024]]
        );
    }

    #[test]
    fn fn_00a28f00_makes_constructed_keys() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_8f00, &args![2u32]).u32();
        assert_eq!(e.mem.u32(keys - 4), 2);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![keys, 0x24, 2, ROTATION_KEY_CONSTRUCT]]
        );
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE)[0][1], 0x4c);
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_8f00, &args![0x1000_0000u32]).u32(), 0);
        assert_eq!(calls_to(&e, MEMORY_ALLOCATE)[0][1], 0xffff_ffff);
        assert!(calls_to(&e, VECTOR_CONSTRUCT).is_empty());
    }

    fn rotation_insert_engine(found: Option<u32>) -> Engine {
        let mut e = Engine::new();
        rotation_constants(&mut e);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(MEMORY_FREE, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
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
        e.register(ROT_KEY_GEN_INTERP, |e, a| {
            floats(e, a[0], &[0.5, 0.5, 0.5, 0.5]);
            a[0].into_ret()
        });
        e.register(QUATERNION_INTERMEDIATE, |e, a| {
            floats(e, a[0], &[9.0, 9.0, 9.0, 9.0]);
            a[0].into_ret()
        });
        e
    }

    #[test]
    fn fn_00a28f50_inserts_a_rotation_key() {
        let mut e = rotation_insert_engine(Some(1));
        let block = e.mem.alloc(4 + 2 * ROTATION_KEY_SIZE);
        e.mem.set_u32(block, 2);
        let old = block + 4;
        floats(&mut e, old, &[1.0, 1.0, 0.0, 0.0, 0.0]);
        floats(&mut e, old + ROTATION_KEY_SIZE, &[3.0, 1.0, 0.0, 0.0, 0.0]);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, old);
        e.mem.set_u32(slots + 4, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a2_8f50, &args![2.0f32, slots, slots + 4]).bool());
        assert_eq!(e.mem.u32(slots + 4), 3);
        let new = e.mem.u32(slots);
        assert_eq!(e.mem.u32(new - 4), 3);
        let times: Vec<f32> = (0..3)
            .map(|i| e.mem.f32(new + i * ROTATION_KEY_SIZE))
            .collect();
        assert_eq!(times, vec![1.0, 2.0, 3.0]);
        assert_eq!(
            read(&e, new + ROTATION_KEY_SIZE + 4, 4),
            vec![0.5, 0.5, 0.5, 0.5]
        );
        // The intermediates are recomputed for the new list.
        assert_eq!(calls_to(&e, QUATERNION_INTERMEDIATE).len(), 3);
        assert_eq!(read(&e, new + 0x14, 4), vec![9.0; 4]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, block]]);
        // A key with that time already: nothing changes.
        let mut e = rotation_insert_engine(None);
        let slots = e.mem.alloc(8);
        e.mem.set_u32(slots, 0x1234);
        e.mem.set_u32(slots + 4, 1);
        assert!(!e.call(0x00a2_8f50, &args![2.0f32, slots, slots + 4]).bool());
        assert_eq!((e.mem.u32(slots), e.mem.u32(slots + 4)), (0x1234, 1));
    }

    #[test]
    fn fn_00a290a0_makes_and_reads_rotation_keys() {
        let mut e = Engine::new();
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(READ_ROTATION_KEY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let keys = e.call(0x00a2_90a0, &args![0x6000u32, 2u32]).u32();
        assert_eq!(
            calls_to(&e, READ_ROTATION_KEY),
            vec![vec![keys, 0x6000], vec![keys + 0x24, 0x6000]]
        );
        // A failed allocation still reads, on keys from null.
        e.register(MEMORY_ALLOCATE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_90a0, &args![0x6000u32, 1u32]).u32(), 0);
        assert_eq!(calls_to(&e, READ_ROTATION_KEY), vec![vec![0, 0x6000]]);
    }

    #[test]
    fn fn_00a291a0_compares_the_step_byte() {
        let mut e = Engine::new();
        e.register(KEY_TIMES_EQUAL, |_, _| 1u32.into_ret());
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_u8(a + 4, 1);
        e.mem.set_u8(b + 4, 1);
        assert!(e.call(0x00a2_91a0, &args![a, b]).bool());
        e.mem.set_u8(b + 4, 0);
        assert!(!e.call(0x00a2_91a0, &args![a, b]).bool());
        e.register(KEY_TIMES_EQUAL, |_, _| 0u32.into_ret());
        e.mem.set_u8(b + 4, 1);
        assert!(!e.call(0x00a2_91a0, &args![a, b]).bool());
    }

    #[test]
    fn fn_00a291d0_and_fn_00a291e0_answer_constants() {
        let mut e = Engine::new();
        e.map(STEP_VALUE, 1);
        e.mem.set_u8(STEP_VALUE, 1);
        let result = e.mem.alloc(4);
        e.mem.set_u32(result, 0xaaaa_aa00);
        e.call(0x00a2_91d0, &args![0.5f32, 0u32, 0u32, result]);
        assert_eq!(e.mem.u32(result), 0xaaaa_aa01);
        let r = e.call(0x00a2_91e0, &args![0x5000u32, 3u32]);
        assert_eq!(r.st0.to_bits(), 0);
    }
}
