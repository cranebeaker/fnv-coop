//! `Havok SDK/run_00df2510` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_00df06a0::{
    addps, addss, maxps, movaps, movss_bits, movss_mem, mulps, parity_even, rcpps, shufps, splat0,
    store_xmm, subps, unpckhps, Xmm, X87,
};
use crate::x87::rsqrt_estimate;

/// Unnamed (cdecl, `(a, b)`): called first by `00df25d0`.
const PREPARE_BODY: u32 = 0x00cb_3c80;
/// Unnamed (cdecl, `(a, b)`): called by `00df25f0` for the records of type 1.
const APPLY_RECORD: u32 = 0x00cb_3da0;
/// The memory router (`00c85750`): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;

/// The floats `00df2750` normalises with: `0.0`, `3.0` and `0.5`.
const FLOAT_ZERO: u32 = 0x0101_1d78;
const FLOAT_THREE: u32 = 0x0101_7718;
const FLOAT_HALF: u32 = 0x0101_6248;
/// Four floats of 1.0, and the mask whose first three lanes are all ones.
const ONES_VECTOR: u32 = 0x010c_7170;
const MASK_XYZ: u32 = 0x010c_4c10;
/// The float -3.4028233e38 (`0xff7fffee`), 1.0e16 and 3.4028233e38 (`0x7f7fffee`).
const NEGATIVE_FLOAT_MAX: u32 = 0x0104_e148;
const BIG_FLOAT: u32 = 0x010c_d644;
const FLOAT_MAX: u32 = 0x0104_da48;
/// The `double`s 65535.0 and 0.0001.
const WIDE_SCALE: u32 = 0x0102_17d0;
const TOLERANCE: u32 = 0x0103_2980;
/// The bit of each axis (1, 2, 4, 8) and four floats of 1 / 65535.
const AXIS_BITS: u32 = 0x010c_92e0;
const BOX_SCALE: u32 = 0x011b_9490;

// Translated from 00df2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Cdecl, five pointers `a1..a5`, answers a float
/// in `ST0`: with `d1 = [a5] - [a3]`, `d2 = [a5] - [a4]`, `m3 = [a1 + 0x1c0]`, `m4 = [a2 +
/// 0x1c0]` and the cross product written as `ru(m) * rd(d) - rd(m) * ru(d)` (`SHUFPS` 0xc9 and
/// 0xd2): the dot product (lane sum `x2 + (x1 + x0)`) of `(cross(m3, d1) + [a1 + 0x1b0]) -
/// (cross(m4, d2) + [a2 + 0x1b0])` with the vector at `a5 + 0x10`; quieted by the `FLD`.
pub fn fn_00df2510(e: &mut Engine, a1: Ptr, a2: Ptr, a3: Ptr, a4: Ptr, a5: Ptr) -> f32 {
    // SSE arithmetic with the SSE choice of NaN (the shared `crate::sse` helpers): a NaN in
    // the destination operand wins (quieted), else a NaN in the source (quieted); an invalid
    // operation on two numbers gives the default NaN 0xffc00000.
    use crate::sse::{addps as add, mulps as mul, subps as sub};
    // MOVAPS XMM2,[a5]; MOVAPS XMM0,[a3]; MOVAPS XMM3,[a1+0x1c0]; SUBPS XMM1(=XMM2),XMM0.
    let point = movaps(e, a5.addr());
    let first = movaps(e, a3.addr());
    let m3 = movaps(e, a1.addr().wrapping_add(0x1c0));
    let d1 = sub(point, first);
    let second = movaps(e, a4.addr());
    let m4 = movaps(e, a2.addr().wrapping_add(0x1c0));
    let d2 = sub(point, second);
    let down = shufps(d1, d1, 0xd2);
    let up = shufps(d1, d1, 0xc9);
    let forward = mul(shufps(m3, m3, 0xc9), down);
    let back = mul(shufps(m3, m3, 0xd2), up);
    let offset = movaps(e, a1.addr().wrapping_add(0x1b0));
    let down2 = shufps(d2, d2, 0xd2);
    let cross = add(sub(forward, back), offset);
    let forward = mul(shufps(m4, m4, 0xc9), down2);
    let up2 = shufps(d2, d2, 0xc9);
    let back = mul(shufps(m4, m4, 0xd2), up2);
    let offset = movaps(e, a2.addr().wrapping_add(0x1b0));
    let cross2 = add(sub(forward, back), offset);
    let diff = sub(cross, cross2);
    let normal = movaps(e, a5.addr().wrapping_add(0x10));
    let product = mul(diff, normal);
    // ADDSS XMM1(lane 1),XMM0(lane 0); ADDSS XMM2(lane 2),XMM1; MOVSS; FLD (quiets).
    let first_two = crate::sse::add(product[1], product[0]);
    let sum = crate::sse::add(product[2], first_two);
    x87_quiet(sum)
}

// Translated from 00df25d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Register-passed inputs first: `a1` (`EAX`) and
/// `body` (`ESI`), then one stack word `flags` (only its low byte is read): runs `00cb3c80`
/// with `(a1, body)` (cdecl), then ORs the low byte of `flags` into the byte at offset 0x12 of
/// the object `body + 0x30` points to.
pub fn fn_00df25d0(e: &mut Engine, a1: u32, body: Ptr, flags: u32) {
    e.call(PREPARE_BODY, &args![a1, body]);
    let object = e.mem.u32(body.addr().wrapping_add(0x30));
    let at = object.wrapping_add(0x12);
    let merged = e.mem.u8(at) | flags as u8;
    e.mem.set_u8(at, merged);
}

// Translated from 00df25f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Cdecl, three words: walks the 16-byte records in
/// `[data, data + size)` (pointer arithmetic wraps; nothing is walked when `data >= data +
/// size`). A record of type 0 (its first byte) runs `00df25d0` with `a1`, the word at `+4` and
/// the word at `+8`; one of type 1 runs `00cb3da0` with `(a1, word at +4)`; any other type ends
/// the walk.
pub fn fn_00df25f0(e: &mut Engine, a1: u32, data: Ptr, size: u32) {
    let end = data.addr().wrapping_add(size);
    let mut record = data.addr();
    if record >= end {
        return;
    }
    loop {
        match e.mem.u8(record) {
            0 => {
                let flags = e.mem.u32(record.wrapping_add(8));
                let body = e.mem.u32(record.wrapping_add(4));
                e.call(0x00df_25d0, &args![a1, body, flags]);
            }
            1 => {
                let body = e.mem.u32(record.wrapping_add(4));
                e.call(APPLY_RECORD, &args![a1, body]);
            }
            _ => return,
        }
        record = record.wrapping_add(0x10);
        if record >= end {
            return;
        }
    }
}

// Translated from 00df2640 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Thiscall with two stack words `a1` and `a2`,
/// answers 0 or 1.
///
/// With the object at `*(this + 0x18)` (`owner`) and its count `*(owner + 0x60)` both non-null:
/// adds the count to the words at `owner + 0xc` and `owner + 0x10`, stores the 16-bit word at
/// `a1 + 4` at `this + 0x10` and the low half of the count at `this + 0x20`, then pushes each of
/// the `count` pointers at `*(owner + 0x5c)` (the signed count: none for zero or negative) onto
/// the pointer array at `*(*(a2 + 0x14) + 0x18) + 0x28` (data, size, capacity word), first
/// storing the array's size (16 bit) at `item + 0x20`; a full array (size equal to the capacity
/// without its two flag bits) grows through `00c90940` with the allocator at the router's
/// `+0x10` and element size 4. The owner's pointer list is then freed (virtual slot 0x10 of the
/// allocator, `(pointer, capacity * 4)`, unless the capacity word has its top bit set), and its
/// pointer set to 0, its count to 0 and its capacity word to `0x80000000`; answers 1.
///
/// Without an owner or a count: when the 16-bit word at `this + 0x20` is more than 1, subtracts
/// 1 from it, adds 1 to the word at `this + 0x10`, stores 1 at `a2 + 0x20` (16 bit) and answers
/// 1; otherwise answers 0.
pub fn fn_00df2640(e: &mut Engine, this: Ptr, a1: Ptr, a2: Ptr) -> u32 {
    let me = this.addr();
    let owner = e.mem.u32(me.wrapping_add(0x18));
    let count = if owner != 0 {
        e.mem.u32(owner.wrapping_add(0x60))
    } else {
        0
    };
    if count != 0 {
        let first = e.mem.u32(owner.wrapping_add(0xc)).wrapping_add(count);
        e.mem.set_u32(owner.wrapping_add(0xc), first);
        let second = e.mem.u32(owner.wrapping_add(0x10)).wrapping_add(count);
        e.mem.set_u32(owner.wrapping_add(0x10), second);
        let word = e.mem.u16(a1.addr().wrapping_add(4));
        e.mem.set_u16(me.wrapping_add(0x10), word);
        e.mem.set_u16(me.wrapping_add(0x20), count as u16);
        let scene = e.mem.u32(a2.addr().wrapping_add(0x14));
        let array = e.mem.u32(scene.wrapping_add(0x18)).wrapping_add(0x28);
        let mut index = 0u32;
        if (count as i32) > 0 {
            loop {
                let list = e.mem.u32(owner.wrapping_add(0x5c));
                let item = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
                let size16 = e.mem.u16(array.wrapping_add(4));
                e.mem.set_u16(item.wrapping_add(0x20), size16);
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let capacity = e.mem.u32(array.wrapping_add(8)) & 0x3fff_ffff;
                if e.mem.u32(array.wrapping_add(4)) == capacity {
                    let allocator = e.mem.u32(router.wrapping_add(0x10));
                    e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
                }
                let size = e.mem.u32(array.wrapping_add(4));
                let data = e.mem.u32(array);
                e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), item);
                e.mem.set_u32(array.wrapping_add(4), size.wrapping_add(1));
                index = index.wrapping_add(1);
                if (index as i32) >= (count as i32) {
                    break;
                }
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router.wrapping_add(0x10));
        let capacity = e.mem.u32(owner.wrapping_add(0x64));
        e.mem.set_u32(owner.wrapping_add(0x60), 0);
        if (capacity as i32) >= 0 {
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            let list = e.mem.u32(owner.wrapping_add(0x5c));
            e.vcall(allocator, 0x10, &args![list, bytes]);
        }
        e.mem.set_u32(owner.wrapping_add(0x5c), 0);
        e.mem.set_u32(owner.wrapping_add(0x64), 0x8000_0000);
        return 1;
    }
    let left = e.mem.u16(me.wrapping_add(0x20));
    if left <= 1 {
        return 0;
    }
    let left = left - 1;
    let step = e.mem.u16(me.wrapping_add(0x10)).wrapping_add(1);
    e.mem.set_u16(me.wrapping_add(0x10), step);
    e.mem.set_u16(me.wrapping_add(0x20), left);
    e.mem.set_u16(a2.addr().wrapping_add(0x20), 1);
    1
}

// Translated from 00df30f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (class unconfirmed). Register-passed inputs first: `vector` (`ECX`)
/// and `index` (`EAX`), then one stack word `base`: stores the 16 bytes at `vector` at `base +
/// (index << 4)`.
pub fn fn_00df30f0(e: &mut Engine, vector: Ptr, index: u32, base: Ptr) {
    let value = movaps(e, vector.addr());
    store_xmm(e, base.addr().wrapping_add(index << 4), value);
}

/// What `00df2750` does next; the labels are the exe's addresses.
#[derive(Clone, Copy, PartialEq)]
enum Step {
    /// `00df2a0b`: the top of the walk, a node header in `EDI`.
    Head,
    /// `00df2a13`: an inner node.
    Inner,
    /// `00df2a2c`: the slab test of the node's axis.
    Slab,
    /// `00df2a5e`: the ray is parallel to the slab.
    Parallel,
    /// `00df2b37`: after the parallel test, the next node.
    NextNode,
    /// `00df2c70`: the ray crosses the slab.
    Crossing,
    /// `00df2df4`: continue with the node in `ESI`.
    Continue,
    /// `00df2b44`: a leaf.
    Leaf,
    /// `00df2fd7`: the x87 stack is trimmed, then an entry is taken from the saved ones.
    Trim,
    /// `00df2fe1`: takes the last saved entry.
    Pop,
    /// `00df3078`: done.
    Done,
}

/// `MOVMSKPS`: the sign bits of the four lanes.
fn movmskps(v: Xmm) -> u32 {
    v.iter().enumerate().fold(0, |bits, (lane, value)| {
        bits | ((value.to_bits() >> 31) << lane)
    })
}

/// A lane mask made by a packed compare: all ones where `test` holds.
fn compare_lanes(a: Xmm, b: Xmm, test: fn(f32, f32) -> bool) -> Xmm {
    std::array::from_fn(|lane| f32::from_bits(if test(a[lane], b[lane]) { u32::MAX } else { 0 }))
}

/// `ANDPS dest, src`.
fn andps(dest: Xmm, src: Xmm) -> Xmm {
    std::array::from_fn(|lane| f32::from_bits(dest[lane].to_bits() & src[lane].to_bits()))
}

/// `ANDNPS dest, src`: `!dest & src`.
fn andnps(dest: Xmm, src: Xmm) -> Xmm {
    std::array::from_fn(|lane| f32::from_bits(!dest[lane].to_bits() & src[lane].to_bits()))
}

/// `ORPS dest, src`.
fn orps(dest: Xmm, src: Xmm) -> Xmm {
    std::array::from_fn(|lane| f32::from_bits(dest[lane].to_bits() | src[lane].to_bits()))
}

/// `MINPS dest, src`: `dest < src ? dest : src` per lane.
fn minps(dest: Xmm, src: Xmm) -> Xmm {
    std::array::from_fn(|lane| {
        if dest[lane] < src[lane] {
            dest[lane]
        } else {
            src[lane]
        }
    })
}

/// The eight 16-bit words at `at` as the two groups of four dwords `PUNPCKLWD` and `PUNPCKHWD`
/// against a zero register make of them.
fn unpack_words(e: &Engine, at: u32) -> ([u32; 4], [u32; 4]) {
    let word = |index: u32| u32::from(e.mem.u16(at.wrapping_add(2 * index)));
    (
        [word(0), word(1), word(2), word(3)],
        [word(4), word(5), word(6), word(7)],
    )
}

/// `PSRLD v, 16` then `CVTDQ2PS`.
fn high_halves(v: [u32; 4]) -> Xmm {
    v.map(|lane| (lane >> 16) as i32 as f32)
}

/// `PSLLD v, 16`, `PSRLD v, 16` then `CVTDQ2PS`.
fn low_halves(v: [u32; 4]) -> Xmm {
    v.map(|lane| ((lane << 16) >> 16) as i32 as f32)
}

// Translated from 00df2750 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a ray sweep through a bounding-volume tree of 16-byte nodes with
/// 16-bit boxes, calling a listener for the hit leaves; class unconfirmed). Cdecl, five words:
/// `result` (the address of an output byte, answered), `tree` (the root pointer at `+0`, a
/// capacity at `+0xc`, the node block at `+0x1c`, the origin vector at `+0x30` and the vector
/// at `+0x40` whose difference to it scales the 16-bit boxes), `sweep` (the start at `+0`, the
/// end at `+0x10` and the extent at `+0x20`, 16 bytes each), `listener` (virtual slot `4` takes a
/// byte address and a leaf's 32-bit user data, slot `8` takes `result` and ends the walk, slot
/// `0xc` answers a float in `ST0` that is scaled by the ray length) and `radius` (a float added
/// to the extent).
///
/// It takes `max(capacity, 0x80)` 16-byte entries from the stack allocator of the memory router
/// (`00c85750`: the allocator's word is advanced by `(n * 16 + 0x7f) & !0x7f` and put back at the
/// end) for a stack of saved far children, derives the direction of the sweep (`RSQRTSS` and a
/// Newton step, 0 for a length of zero), its reciprocal per lane (`RCPPS` and a Newton step, the
/// lanes of a zero direction made 0) and its length, and walks the tree. An inner node's header
/// has its axis in the low two bits and the offset of its two children (8 bytes apart) in the
/// rest; per axis it computes the entry and exit of the 16-bit minimum and maximum (divided by
/// 65535.0, scaled by the extent, offset by the origin, grown by the radius) and orders the
/// children by the sign of the direction. A leaf (header other than `0x7fffffff`, with a
/// positive count) holds that many 16-byte boxes, tested in SSE; each hit calls the listener's
/// slot 4. The x87 stack is kept in the same shape as the exe's.
#[allow(unused_assignments)] // the exe's registers are locals; some start unread
pub fn fn_00df2750(
    e: &mut Engine,
    result: Ptr,
    tree: Ptr,
    sweep: Ptr,
    listener: Ptr,
    radius: f32,
) -> Ptr {
    e.with_stack(0x1a0, |e, frame| {
        let b = frame.addr();
        let at = |offset: u32| b.wrapping_add(offset);
        let tr = tree.addr();
        let sw = sweep.addr();
        let l = listener.addr();
        for lane in 0..4u32 {
            let word = e.mem.u32(tr.wrapping_add(0x30 + 4 * lane));
            e.mem.set_u32(at(0x80 + 4 * lane), word);
        }
        let extent = subps(
            movaps(e, tr.wrapping_add(0x40)),
            movaps(e, tr.wrapping_add(0x30)),
        );
        store_xmm(e, at(0x100), extent);
        let node_base = e.mem.u32(tr.wrapping_add(0x1c));
        e.mem.set_u32(at(0x98), node_base);
        let count = e.mem.u32(tr.wrapping_add(0xc));
        let capacity = if (count as i32) > 0 { count } else { 0x80 };
        e.mem.set_u32(at(0x18), capacity);
        let root = e.mem.u32(tr);
        let mut edi = e.mem.u32(root);
        let mut ebx = e.mem.u32(root.wrapping_add(4));
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let cursor = e.mem.u32(router);
        let advance = capacity.wrapping_shl(4).wrapping_add(0x7f) & 0xffff_ff80;
        e.mem.set_u32(router, advance.wrapping_add(cursor));
        e.mem.set_u32(at(0x6c), cursor);
        e.mem.set_u32(at(0x2c), cursor);
        for lane in 0..4u32 {
            let word = e.mem.u32(sw.wrapping_add(4 * lane));
            e.mem.set_u32(at(0x40 + 4 * lane), word);
        }
        let p0 = movaps(e, sw);
        let p1 = movaps(e, sw.wrapping_add(0x10));
        let delta = subps(p1, p0);
        let squares = mulps(delta, delta);
        let partial = addss(shufps(squares, squares, 0x55), squares);
        let total = addss(shufps(squares, squares, 0xaa), partial);
        let mut estimate = total;
        estimate[0] = rsqrt_estimate(total[0]);
        let zero_scalar = movss_bits(e.mem.u32(FLOAT_ZERO));
        let mut product = total;
        product[0] *= estimate[0];
        product[0] *= estimate[0];
        let mut is_zero = total;
        is_zero[0] = f32::from_bits(if total[0] == zero_scalar[0] {
            u32::MAX
        } else {
            0
        });
        let three = f32::from_bits(e.mem.u32(FLOAT_THREE));
        let half = f32::from_bits(e.mem.u32(FLOAT_HALF));
        let mut scale = [half, 0.0, 0.0, 0.0];
        scale[0] *= estimate[0];
        scale[0] *= three - product[0];
        let scale = splat0(andnps(is_zero, scale));
        let direction = mulps(scale, delta);
        let length = total[0] * scale[0];
        store_xmm(e, at(0x120), direction);
        e.mem.set_f32(at(0x38), length);
        for lane in 0..4u32 {
            let word = e.mem.u32(sw.wrapping_add(0x20 + 4 * lane));
            e.mem.set_u32(at(0x50 + 4 * lane), word);
        }
        e.mem.set_u32(at(0x28), 0);
        e.mem.set_u32(at(0x30), x87_quiet_bits(length.to_bits()));
        let reach = addps(splat0(movss_bits(radius.to_bits())), movaps(e, at(0x50)));
        store_xmm(e, at(0x180), reach);
        let zero = [0.0f32; 4];
        let ones = movaps(e, ONES_VECTOR);
        let flat = compare_lanes(zero, direction, |a, b| a == b);
        let kept = orps(andnps(flat, direction), andps(flat, ones));
        let high = unpckhps(kept, ones);
        let lanes = shufps(kept, high, 0xc4);
        let inverse = rcpps(lanes);
        let newton = mulps(mulps(inverse, lanes), inverse);
        let doubled = addps(inverse, inverse);
        let newton = andps(subps(doubled, newton), movaps(e, MASK_XYZ));
        store_xmm(e, at(0xa0), flat);
        let reciprocal = orps(andnps(flat, newton), andps(flat, zero));
        store_xmm(e, at(0xb0), zero);
        store_xmm(e, at(0x70), reciprocal);
        let mut x = X87::new();
        let returned = e.vcall(l, 0xc, &args![]).st0;
        x.push(returned);
        x.fld32(e.mem.u32(at(0x38)));
        x.fld_st(0);
        x.fmulp(2);
        x.fxch(1);
        e.mem.set_u32(at(0x34), x.fstp32());
        x.fld32(e.mem.u32(at(0x34)));
        let status = x.fcom(1);
        if parity_even(status, 5) {
            x.fpop();
        } else {
            x.fstp_st(1);
        }
        let flat_lanes = movaps(e, at(0xa0));
        e.mem.set_u32(at(0x20), x.fstp32());
        let mut xmm0 = flat_lanes;
        let mut xmm6 = [0.0f32; 4];
        let mut xmm7 = [0.0f32; 4];
        let (mut eax, mut ecx, mut edx) = (0u32, 0u32, 0u32);
        let mut esi = root;
        let mut step = if movmskps(flat_lanes) & 7 == 7 {
            Step::Done
        } else {
            x.fld64(e.mem.f64(WIDE_SCALE));
            xmm7 = movaps(e, at(0x40));
            x.fld32(e.mem.u32(BIG_FLOAT));
            xmm6 = movaps(e, at(0xb0));
            x.fld32(e.mem.u32(NEGATIVE_FLOAT_MAX));
            x.fld32(e.mem.u32(at(0x28)));
            x.fld32(e.mem.u32(at(0x30)));
            Step::Head
        };
        while step != Step::Done {
            match step {
                Step::Head => {
                    step = if (edi as i32) >= 0 {
                        Step::Leaf
                    } else {
                        Step::Inner
                    };
                }
                Step::Inner => {
                    xmm0 = movaps(e, at(0x120));
                    let equal = compare_lanes(xmm6, xmm0, |a, b| a == b);
                    e.mem.set_u32(at(0x9c), movmskps(equal));
                    step = Step::Slab;
                }
                Step::Slab => {
                    eax = e.mem.u32(esi) & 0x7fff_fffc;
                    edi &= 3;
                    edx = eax.wrapping_add(esi);
                    eax = edi;
                    ecx = e.mem.u32(AXIS_BITS.wrapping_add(eax.wrapping_mul(4)));
                    edi = e.mem.u32(at(0x9c)) & ecx;
                    esi = edx.wrapping_add(8);
                    e.mem.set_u32(at(0x18), edx);
                    e.mem.set_u32(at(0x24), esi);
                    step = if edi != ecx {
                        Step::Crossing
                    } else {
                        Step::Parallel
                    };
                }
                Step::Parallel => {
                    let lane = eax.wrapping_mul(4);
                    edx = ebx & 0xffff;
                    e.mem.set_u32(at(0x1c), edx);
                    ecx = at(0x100).wrapping_add(lane);
                    edi = at(0x80).wrapping_add(lane);
                    x.fild(e.mem.i32(at(0x1c)));
                    esi = at(0x50).wrapping_add(lane);
                    ebx >>= 16;
                    edx = at(0x40).wrapping_add(lane);
                    x.fdiv_st0(5);
                    x.fmul_m32(e.mem.u32(ecx));
                    x.fadd_m32(e.mem.u32(edi));
                    e.mem.set_u32(at(0x1c), x.fstp32());
                    x.fld32(e.mem.u32(at(0x1c)));
                    e.mem.set_u32(at(0x1c), ebx);
                    x.fsub_m32(e.mem.u32(esi));
                    e.mem.set_u32(at(0x14), x.fstp32());
                    x.fld32(e.mem.u32(edx));
                    x.fild(e.mem.i32(at(0x1c)));
                    x.fdiv_st0(6);
                    x.fmul_m32(e.mem.u32(ecx));
                    x.fadd_m32(e.mem.u32(edi));
                    e.mem.set_u32(at(0x1c), x.fstp32());
                    x.fld32(e.mem.u32(at(0x1c)));
                    x.fadd_m32(e.mem.u32(esi));
                    e.mem.set_u32(at(0x1c), x.fstp32());
                    x.fld32(e.mem.u32(at(0x1c)));
                    x.fld64(e.mem.f64(TOLERANCE));
                    x.fadd_to(1);
                    x.fxch(1);
                    e.mem.set_u32(at(0x1c), x.fstp32());
                    x.fld32(e.mem.u32(at(0x1c)));
                    let status = x.fcomp(2);
                    x.fstp_st(1);
                    ecx = if status & 1 != 0 { 0 } else { 1 };
                    x.fld32(e.mem.u32(edx));
                    x.fld32(e.mem.u32(at(0x14)));
                    x.fsubrp(2);
                    x.fxch(1);
                    e.mem.set_u32(at(0x14), x.fstp32());
                    x.fld32(e.mem.u32(at(0x14)));
                    let status = x.fcompp();
                    eax = if parity_even(status, 0x41) { 0 } else { 1 };
                    if ecx == 0 {
                        if eax == 0 {
                            step = Step::Trim;
                        } else {
                            esi = e.mem.u32(at(0x24));
                            step = Step::NextNode;
                        }
                    } else {
                        esi = e.mem.u32(at(0x18));
                        if eax != 0 {
                            eax = e.mem.u32(at(0x2c));
                            x.fxch(1);
                            ecx = e.mem.u32(at(0x24));
                            e.mem.set_u32(eax.wrapping_add(4), x.fst32());
                            x.fxch(1);
                            e.mem.set_u32(eax, ecx);
                            e.mem.set_u32(eax.wrapping_add(8), x.fst32());
                            eax = eax.wrapping_add(0x10);
                            e.mem.set_u32(at(0x2c), eax);
                        }
                        step = Step::NextNode;
                    }
                }
                Step::NextNode => {
                    edi = e.mem.u32(esi);
                    ebx = e.mem.u32(esi.wrapping_add(4));
                    step = if (edi as i32) < 0 {
                        Step::Slab
                    } else {
                        Step::Leaf
                    };
                }
                Step::Crossing => {
                    ecx = ebx & 0xffff;
                    e.mem.set_u32(at(0x14), ecx);
                    ebx >>= 16;
                    let nearer = compare_lanes(xmm6, xmm0, |a, b| a < b);
                    x.fild(e.mem.i32(at(0x14)));
                    ecx = movmskps(nearer);
                    let lane = eax.wrapping_mul(4);
                    x.fdiv_st0(5);
                    x.fmul_m32(e.mem.u32(at(0x100).wrapping_add(lane)));
                    x.fadd_m32(e.mem.u32(at(0x80).wrapping_add(lane)));
                    e.mem.set_u32(at(0x14), x.fstp32());
                    x.fld32(e.mem.u32(at(0x14)));
                    x.fsub_m32(e.mem.u32(at(0x50).wrapping_add(lane)));
                    e.mem.set_u32(at(0x14), x.fstp32());
                    x.fld32(e.mem.u32(at(0x14)));
                    e.mem.set_u32(at(0x14), ebx);
                    x.fsub_m32(e.mem.u32(at(0x40).wrapping_add(lane)));
                    x.fmul_m32(e.mem.u32(at(0x70).wrapping_add(lane)));
                    e.mem.set_u32(at(0x18), x.fstp32());
                    x.fild(e.mem.i32(at(0x14)));
                    x.fdiv_st0(5);
                    x.fmul_m32(e.mem.u32(at(0x100).wrapping_add(lane)));
                    x.fadd_m32(e.mem.u32(at(0x80).wrapping_add(lane)));
                    e.mem.set_u32(at(0x14), x.fstp32());
                    x.fld32(e.mem.u32(at(0x14)));
                    x.fadd_m32(e.mem.u32(at(0x50).wrapping_add(lane)));
                    e.mem.set_u32(at(0x14), x.fstp32());
                    x.fld32(e.mem.u32(at(0x14)));
                    x.fsub_m32(e.mem.u32(at(0x40).wrapping_add(lane)));
                    x.fmul_m32(e.mem.u32(at(0x70).wrapping_add(lane)));
                    eax = e.mem.u32(AXIS_BITS.wrapping_add(lane));
                    ecx &= eax;
                    e.mem.set_u32(at(0x24), x.fstp32());
                    if ecx == eax {
                        ecx = edx;
                    } else {
                        x.fld32(e.mem.u32(at(0x18)));
                        ecx = esi;
                        e.mem.set_u32(at(0x14), x.fstp32());
                        esi = edx;
                        x.fld32(e.mem.u32(at(0x24)));
                        e.mem.set_u32(at(0x18), x.fstp32());
                        x.fld32(e.mem.u32(at(0x14)));
                        e.mem.set_u32(at(0x24), x.fstp32());
                    }
                    x.fld32(e.mem.u32(at(0x24)));
                    let status = x.fcomp(2);
                    if !parity_even(status, 5) {
                        step = Step::Continue;
                        continue;
                    }
                    x.fld32(e.mem.u32(at(0x18)));
                    x.fld32(e.mem.u32(at(0x20)));
                    let status = x.fcomp(1);
                    if !parity_even(status, 5) {
                        esi = ecx;
                        x.fpop();
                        step = Step::Continue;
                        continue;
                    }
                    x.fstp_st(4);
                    x.fstp_st(4);
                    x.fstp_st(1);
                    let status = x.fcom(1);
                    if parity_even(status, 5) {
                        x.fstp_st(1);
                    } else {
                        x.fpop();
                    }
                    eax = e.mem.u32(at(0x2c));
                    e.mem.set_u32(at(0x18), x.fstp32());
                    x.fld32(e.mem.u32(at(0x18)));
                    e.mem.set_u32(eax, esi);
                    e.mem.set_u32(eax.wrapping_add(4), x.fstp32());
                    eax = eax.wrapping_add(0x10);
                    e.mem.set_u32(at(0x2c), eax);
                    e.mem.set_u32(eax.wrapping_sub(8), x.fst32());
                    esi = ecx;
                    x.fld32(e.mem.u32(at(0x24)));
                    let status = x.fcom(1);
                    x.fstp_st(1);
                    if parity_even(status, 5) {
                        x.fpop();
                    } else {
                        e.mem.set_u32(at(0x30), x.fstp32());
                    }
                    let returned = e.vcall(l, 0xc, &args![]).st0;
                    x.push(returned);
                    x.fmul_m32(e.mem.u32(at(0x38)));
                    xmm6 = movaps(e, at(0xb0));
                    xmm7 = movaps(e, at(0x40));
                    e.mem.set_u32(at(0x14), x.fstp32());
                    x.fld32(e.mem.u32(at(0x14)));
                    x.fld32(e.mem.u32(at(0x30)));
                    let status = x.fcom(1);
                    if status & 0x41 != 0 {
                        x.fstp_st(1);
                        e.mem.set_u32(at(0x20), x.fst32());
                    } else {
                        x.fxch(1);
                        e.mem.set_u32(at(0x20), x.fstp32());
                    }
                    x.fld32(e.mem.u32(NEGATIVE_FLOAT_MAX));
                    x.fld32(e.mem.u32(BIG_FLOAT));
                    x.fld64(e.mem.f64(WIDE_SCALE));
                    x.fld32(e.mem.u32(at(0x28)));
                    x.fxch(1);
                    x.fxch(4);
                    x.fxch(2);
                    x.fxch(3);
                    x.fxch(2);
                    step = Step::Continue;
                }
                Step::Continue => {
                    edi = e.mem.u32(esi);
                    ebx = e.mem.u32(esi.wrapping_add(4));
                    step = Step::Head;
                }
                Step::Leaf => {
                    x.fpop();
                    ecx = edi ^ 0x7fff_ffff;
                    x.fpop();
                    x.fstp_st(2);
                    if ecx == 0 || (ebx as i32) <= 0 {
                        x.fpop();
                        x.fpop();
                        step = Step::Pop;
                        continue;
                    }
                    let inverse = movaps(e, at(0x70));
                    let big = x.fst32();
                    for offset in [0xc0u32, 0xc4, 0xc8, 0xcc, 0xd0, 0xd4, 0xd8] {
                        e.mem.set_u32(at(offset), big);
                    }
                    edx = e.mem.u32(at(0x98));
                    let not_behind = compare_lanes(xmm6, inverse, |a, b| a <= b);
                    store_xmm(e, at(0xa0), not_behind);
                    e.mem.set_u32(at(0xdc), x.fstp32());
                    xmm6 = compare_lanes(xmm6, inverse, |a, b| a == b);
                    let lowest = x.fst32();
                    for offset in [0xf0u32, 0xf4, 0xf8] {
                        e.mem.set_u32(at(offset), lowest);
                    }
                    edi &= 0x7fff_ffff;
                    edi <<= 4;
                    store_xmm(e, at(0x140), [0.0; 4]);
                    e.mem.set_u32(at(0xfc), x.fstp32());
                    store_xmm(e, at(0x190), xmm6);
                    x.fld32(e.mem.u32(FLOAT_MAX));
                    let lowest_vector = movaps(e, at(0xf0));
                    let flat_low = andps(xmm6, lowest_vector);
                    let top = x.fst32();
                    e.mem.set_u32(at(0x110), top);
                    e.mem.set_u32(at(0x114), top);
                    store_xmm(e, at(0x160), flat_low);
                    e.mem.set_u32(at(0x118), top);
                    e.mem.set_u32(at(0x11c), x.fstp32());
                    edi = edi.wrapping_add(edx).wrapping_add(0xe);
                    let top_vector = movaps(e, at(0x110));
                    let flat_high = andps(xmm6, top_vector);
                    store_xmm(e, at(0x130), flat_high);
                    let near_limit = splat0(movss_mem(e, at(0x28)));
                    store_xmm(e, at(0x170), near_limit);
                    let far_limit = splat0(movss_mem(e, at(0x20)));
                    store_xmm(e, at(0x150), far_limit);
                    loop {
                        let user_low = e.mem.u16(edi.wrapping_sub(8));
                        let user_high = e.mem.u16(edi);
                        let dwords = [
                            e.mem.u32(edi.wrapping_sub(0xe)),
                            e.mem.u32(edi.wrapping_sub(0xa)),
                            e.mem.u32(edi.wrapping_sub(6)),
                            e.mem.u32(edi.wrapping_sub(2)),
                        ];
                        let scaled = mulps(movaps(e, at(0x100)), movaps(e, BOX_SCALE));
                        for (lane, word) in dwords.iter().enumerate() {
                            e.mem.set_u32(at(0xe0 + 4 * lane as u32), *word);
                        }
                        e.mem.set_u16(at(0x34), user_low);
                        e.mem.set_u16(at(0x36), user_high);
                        let (low_words, high_words) = unpack_words(e, at(0xe0));
                        let inverse = movaps(e, at(0x70));
                        let minimum = addps(
                            mulps(high_halves(low_words), movaps(e, at(0xc0))),
                            low_halves(low_words),
                        );
                        let origin = movaps(e, at(0x80));
                        let minimum = addps(mulps(minimum, scaled), origin);
                        let maximum = addps(
                            mulps(high_halves(high_words), movaps(e, at(0xd0))),
                            low_halves(high_words),
                        );
                        let maximum = addps(mulps(maximum, scaled), origin);
                        let reach = movaps(e, at(0x180));
                        let minimum = subps(minimum, reach);
                        let maximum = addps(maximum, reach);
                        let sign = movaps(e, at(0xa0));
                        let enter_a = mulps(subps(minimum, xmm7), inverse);
                        let enter_b = mulps(subps(maximum, xmm7), movaps(e, at(0x70)));
                        let near = orps(andnps(sign, enter_b), andps(sign, enter_a));
                        let near = orps(andnps(xmm6, near), movaps(e, at(0x160)));
                        let far = orps(andps(sign, enter_b), andnps(sign, enter_a));
                        let far = orps(andnps(xmm6, far), movaps(e, at(0x130)));
                        let lowest = minps(
                            minps(splat0(far), shufps(far, far, 0x55)),
                            shufps(far, far, 0xaa),
                        );
                        let lowest = minps(movaps(e, at(0x150)), lowest);
                        let highest = maxps(
                            maxps(splat0(near), shufps(near, near, 0x55)),
                            shufps(near, near, 0xaa),
                        );
                        let highest = maxps(movaps(e, at(0x170)), highest);
                        xmm7 = movaps(e, at(0x40));
                        let missed = andnps(xmm6, compare_lanes(highest, lowest, |a, b| a < b));
                        let inside = andps(
                            compare_lanes(minimum, xmm7, |a, b| a <= b),
                            compare_lanes(xmm7, maximum, |a, b| a <= b),
                        );
                        let inside = andps(inside, xmm6);
                        if movmskps(orps(missed, inside)) & 7 == 7 {
                            let data = e.mem.u32(at(0x34));
                            e.vcall(l, 4, &args![at(0x3f), data]);
                            xmm6 = movaps(e, at(0x190));
                            xmm7 = movaps(e, at(0x40));
                        }
                        edi = edi.wrapping_add(0x10);
                        ebx = ebx.wrapping_sub(1);
                        if ebx == 0 {
                            break;
                        }
                    }
                    step = Step::Pop;
                }
                Step::Trim => {
                    x.fstp_st(3);
                    x.fstp_st(2);
                    x.fstp_st(1);
                    x.fstp_st(1);
                    x.fpop();
                    step = Step::Pop;
                }
                Step::Pop => {
                    eax = e.mem.u32(at(0x2c));
                    if eax == e.mem.u32(at(0x6c)) {
                        step = Step::Done;
                        continue;
                    }
                    x.fld32(e.mem.u32(eax.wrapping_sub(8)));
                    esi = e.mem.u32(eax.wrapping_sub(0x10));
                    e.mem.set_u32(at(0x30), x.fstp32());
                    x.fld32(e.mem.u32(eax.wrapping_sub(0xc)));
                    edi = e.mem.u32(esi);
                    ebx = e.mem.u32(esi.wrapping_add(4));
                    e.mem.set_u32(at(0x28), x.fstp32());
                    eax = eax.wrapping_sub(0x10);
                    e.mem.set_u32(at(0x2c), eax);
                    let returned = e.vcall(l, 0xc, &args![]).st0;
                    x.push(returned);
                    x.fmul_m32(e.mem.u32(at(0x38)));
                    e.mem.set_u32(at(0x14), x.fstp32());
                    x.fld32(e.mem.u32(at(0x14)));
                    x.fld32(e.mem.u32(at(0x30)));
                    let status = x.fcom(1);
                    if status & 0x41 != 0 {
                        x.fstp_st(1);
                        e.mem.set_u32(at(0x20), x.fst32());
                    } else {
                        x.fxch(1);
                        e.mem.set_u32(at(0x20), x.fstp32());
                    }
                    x.fld32(e.mem.u32(at(0x20)));
                    x.fld32(e.mem.u32(at(0x28)));
                    let status = x.fcom(1);
                    x.fstp_st(1);
                    if status & 1 == 0 {
                        x.fstp_st(1);
                        x.fpop();
                    } else {
                        x.fld32(e.mem.u32(NEGATIVE_FLOAT_MAX));
                        xmm6 = movaps(e, at(0xb0));
                        x.fld32(e.mem.u32(BIG_FLOAT));
                        xmm7 = movaps(e, at(0x40));
                        x.fld64(e.mem.f64(WIDE_SCALE));
                        x.fxch(4);
                        x.fxch(1);
                        x.fxch(3);
                        x.fxch(1);
                        step = Step::Head;
                    }
                }
                Step::Done => {}
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let base = e.mem.u32(at(0x6c));
        e.mem.set_u32(router, base);
        e.vcall(l, 8, &args![result]);
        result
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_2510, fn_00df2510(Ptr, Ptr, Ptr, Ptr, Ptr) -> f32),
        entry!(0x00df_25d0, fn_00df25d0(u32, Ptr, u32)),
        entry!(0x00df_25f0, fn_00df25f0(u32, Ptr, u32)),
        entry!(0x00df_2640, fn_00df2640(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00df_2750, fn_00df2750(Ptr, Ptr, Ptr, Ptr, f32) -> Ptr),
        entry!(0x00df_30f0, fn_00df30f0(Ptr, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn put(e: &mut Engine, at: u32, lanes: [f32; 4]) {
        for (lane, value) in lanes.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
    }

    fn logging(e: &mut Engine, address: u32, eax: u32) -> CallLog {
        let log: CallLog = Rc::default();
        let seen = log.clone();
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                eax,
                ..Ret::default()
            }
        });
        log
    }

    #[test]
    fn the_drift_along_the_normal_is_a_difference_of_two_cross_products() {
        let mut e = Engine::new();
        let a1 = e.mem.alloc(0x1d0 + 0x10);
        let a2 = e.mem.alloc(0x1d0 + 0x10);
        let a3 = e.mem.alloc(0x10);
        let a4 = e.mem.alloc(0x10);
        let a5 = e.mem.alloc(0x20);
        put(&mut e, a5, [5.0, 3.0, 2.0, 0.0]);
        put(&mut e, a5 + 0x10, [1.0, 2.0, 3.0, 0.0]);
        put(&mut e, a3, [1.0, 1.0, 1.0, 0.0]);
        put(&mut e, a4, [2.0, 0.0, 1.0, 0.0]);
        put(&mut e, a1 + 0x1c0, [1.0, 2.0, 3.0, 0.0]);
        put(&mut e, a1 + 0x1b0, [1.0, 0.0, 0.0, 0.0]);
        put(&mut e, a2 + 0x1c0, [0.0, 1.0, 2.0, 0.0]);
        put(&mut e, a2 + 0x1b0, [0.0, 1.0, 0.0, 0.0]);
        // cross((1,2,3), (4,2,1)) + (1,0,0) = (-3, 11, -6);
        // cross((0,1,2), (3,3,1)) + (0,1,0) = (-5, 7, -3); the difference (2, 4, -3) . (1, 2, 3).
        let answer = e.call(0x00df_2510, &args![a1, a2, a3, a4, a5]).f32();
        assert_eq!(answer, 1.0);
    }

    type Words<'a> = &'a [(u32, u32)];

    fn words(e: &mut Engine, size: u32, words: Words) -> u32 {
        let at = e.mem.alloc(size);
        for offset in (0..size).step_by(4) {
            e.mem.set_u32(at + offset, 0);
        }
        for &(offset, word) in words {
            e.mem.set_u32(at + offset, word);
        }
        at
    }

    #[test]
    fn the_drift_keeps_the_sse_choice_of_nan() {
        // Oracle vectors 00df2510_4 and 00df2510_12 (the words not listed are zero): the
        // NaN of the destination operand wins at each SSE step; the exe answers 0xffffffca
        // and 0x7fc00000.
        let cases: [(Words, Words, Words, Words, Words, u32); 2] = [
            (
                &[
                    (0x1b4, 1),
                    (0x1b8, 2),
                    (0x1c0, 3),
                    (0x1c4, 0xffff_ffff),
                    (0x1c8, 0x8000_0000),
                ],
                &[
                    (0x1b0, 0x7fff_ffff),
                    (0x1b4, 0x3f80_0000),
                    (0x1b8, 0xbf80_0000),
                    (0x1c0, 0x3fc0_0000),
                    (0x1c4, 0x3f00_0000),
                    (0x1c8, 0x7fc0_0000),
                ],
                &[(0, 0x7f80_0000), (4, 1), (8, 0xffff_ffd1)],
                &[(0, 0xffff_ffd2), (4, 0xffff_ffd3), (8, 0xffff_ffc8)],
                &[
                    (0, 0xffff_ffc9),
                    (4, 0xffff_ffca),
                    (8, 0x54),
                    (0x10, 0x55),
                    (0x14, 0x56),
                    (0x18, 0xffff_ffa9),
                ],
                0xffff_ffca,
            ),
            (
                &[
                    (0x1b0, 0xbf80_0000),
                    (0x1b4, 0x3fc0_0000),
                    (0x1b8, 0x3f00_0000),
                    (0x1c0, 0x7fc0_0000),
                    (0x1c4, 0x7f80_0000),
                    (0x1c8, 1),
                ],
                &[
                    (0x1b0, 0xffff_ffd1),
                    (0x1b4, 0xffff_ffd2),
                    (0x1b8, 0xffff_ffd3),
                    (0x1c0, 0xffff_ffc8),
                    (0x1c4, 0xffff_ffc9),
                    (0x1c8, 0xffff_ffca),
                ],
                &[(0, 0x54), (4, 0x55), (8, 0x56)],
                &[(0, 0xffff_ffa9), (4, 0xffff_ffaa), (8, 0xffff_ffab)],
                &[
                    (0, 0x3000_0000),
                    (4, 0x3000_01e0),
                    (8, 0x3000_03c0),
                    (0x10, 0x3000_03e0),
                    (0x14, 0x3000_0400),
                ],
                0x7fc0_0000,
            ),
        ];
        for (w1, w2, w3, w4, w5, expected) in cases {
            let mut e = Engine::new();
            let a1 = words(&mut e, 0x1e0, w1);
            let a2 = words(&mut e, 0x1e0, w2);
            let a3 = words(&mut e, 0x20, w3);
            let a4 = words(&mut e, 0x20, w4);
            let a5 = words(&mut e, 0x30, w5);
            let answer = e.call(0x00df_2510, &args![a1, a2, a3, a4, a5]).f32();
            assert_eq!(answer.to_bits(), expected);
        }
    }

    #[test]
    fn the_flags_are_merged_into_the_object_behind_the_body() {
        let mut e = Engine::new();
        let prepare = logging(&mut e, PREPARE_BODY, 0);
        let body = e.mem.alloc(0x40);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(body + 0x30, object);
        e.mem.set_u8(object + 0x12, 0x01);
        e.call(0x00df_25d0, &args![0x5000u32, body, 0xffff_ff06u32]);
        assert_eq!(e.mem.u8(object + 0x12), 0x07);
        assert_eq!(*prepare.borrow(), vec![vec![0x5000, body]]);
    }

    fn record(e: &mut Engine, at: u32, kind: u8, word: u32, flags: u32) {
        e.mem.set_u8(at, kind);
        e.mem.set_u32(at + 4, word);
        e.mem.set_u32(at + 8, flags);
    }

    #[test]
    fn the_records_run_until_one_of_another_type() {
        let mut e = Engine::new();
        let prepare = logging(&mut e, PREPARE_BODY, 0);
        let apply = logging(&mut e, APPLY_RECORD, 0);
        let body = e.mem.alloc(0x40);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(body + 0x30, object);
        let data = e.mem.alloc(0x50);
        record(&mut e, data, 0, body, 0x04);
        record(&mut e, data + 0x10, 1, 0x7777, 0);
        record(&mut e, data + 0x20, 2, 0x8888, 0);
        record(&mut e, data + 0x30, 1, 0x9999, 0);
        e.call(0x00df_25f0, &args![0x5000u32, data, 0x40u32]);
        assert_eq!(*prepare.borrow(), vec![vec![0x5000, body]]);
        assert_eq!(*apply.borrow(), vec![vec![0x5000, 0x7777]]);
        assert_eq!(e.mem.u8(object + 0x12), 0x04);
    }

    #[test]
    fn an_empty_range_runs_nothing() {
        let mut e = Engine::new();
        let prepare = logging(&mut e, PREPARE_BODY, 0);
        let apply = logging(&mut e, APPLY_RECORD, 0);
        let data = e.mem.alloc(0x20);
        record(&mut e, data, 1, 0x7777, 0);
        e.call(0x00df_25f0, &args![0x5000u32, data, 0u32]);
        e.call(0x00df_25f0, &args![0x5000u32, data, 0xffff_ffffu32]);
        assert!(prepare.borrow().is_empty());
        assert!(apply.borrow().is_empty());
    }

    #[test]
    fn a_vector_is_stored_at_the_indexed_slot() {
        let mut e = Engine::new();
        let vector = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x100);
        put(&mut e, vector, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x00df_30f0, &args![vector, 3u32, base]);
        for lane in 0..4u32 {
            assert_eq!(e.mem.f32(base + 0x30 + 4 * lane), 1.0 + lane as f32);
        }
        assert_eq!(e.mem.u32(base + 0x2c), 0);
        assert_eq!(e.mem.u32(base + 0x40), 0);
    }

    const ALLOCATOR_FREE: u32 = 0x0b00_0010;

    struct Rig {
        e: Engine,
        this: u32,
        owner: u32,
        a1: u32,
        a2: u32,
        array: u32,
        items: Vec<u32>,
        frees: CallLog,
    }

    /// An owner with `items` pointers, a scene whose pointer array has the given size and
    /// capacity word, and a router whose allocator frees through slot `0x10`.
    fn rig(items: usize, size: u32, capacity: u32) -> Rig {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let owner = e.mem.alloc(0x70);
        let a1 = e.mem.alloc(0x10);
        let a2 = e.mem.alloc(0x30);
        let scene = e.mem.alloc(0x40);
        let holder = e.mem.alloc(0x40);
        let array = holder + 0x28;
        let list = e.mem.alloc(4 * 8);
        let data = e.mem.alloc(4 * 16);
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x20);
        let mut item_ptrs = Vec::new();
        for index in 0..items as u32 {
            let item = e.mem.alloc(0x30);
            e.mem.set_u32(list + 4 * index, item);
            item_ptrs.push(item);
        }
        e.mem.set_u32(this + 0x18, owner);
        e.mem.set_u32(owner + 0x5c, list);
        e.mem.set_u32(owner + 0x60, items as u32);
        e.mem.set_u32(owner + 0x64, 8);
        e.mem.set_u32(owner + 0xc, 100);
        e.mem.set_u32(owner + 0x10, 200);
        e.mem.set_u16(a1 + 4, 0x1234);
        e.mem.set_u32(a2 + 0x14, scene);
        e.mem.set_u32(scene + 0x18, holder);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(vtable + 0x10, ALLOCATOR_FREE);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees = logging(&mut e, ALLOCATOR_FREE, 0);
        Rig {
            e,
            this,
            owner,
            a1,
            a2,
            array,
            items: item_ptrs,
            frees,
        }
    }

    #[test]
    fn the_owners_pointers_move_into_the_scene_array_and_the_list_is_freed() {
        let mut r = rig(2, 3, 16);
        let list = r.e.mem.u32(r.owner + 0x5c);
        let result = r.e.call(0x00df_2640, &args![r.this, r.a1, r.a2]).u32();
        assert_eq!(result, 1);
        assert_eq!(r.e.mem.u32(r.owner + 0xc), 102);
        assert_eq!(r.e.mem.u32(r.owner + 0x10), 202);
        assert_eq!(r.e.mem.u16(r.this + 0x10), 0x1234);
        assert_eq!(r.e.mem.u16(r.this + 0x20), 2);
        let data = r.e.mem.u32(r.array);
        assert_eq!(r.e.mem.u32(r.array + 4), 5);
        assert_eq!(r.e.mem.u32(data + 12), r.items[0]);
        assert_eq!(r.e.mem.u32(data + 16), r.items[1]);
        assert_eq!(r.e.mem.u16(r.items[0] + 0x20), 3);
        assert_eq!(r.e.mem.u16(r.items[1] + 0x20), 4);
        assert_eq!(r.e.mem.u32(r.owner + 0x5c), 0);
        assert_eq!(r.e.mem.u32(r.owner + 0x60), 0);
        assert_eq!(r.e.mem.u32(r.owner + 0x64), 0x8000_0000);
        let frees = r.frees.borrow();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1..], [list, 32]);
    }

    #[test]
    fn a_list_without_memory_is_not_freed() {
        let mut r = rig(1, 0, 16);
        r.e.mem.set_u32(r.owner + 0x64, 0x8000_0000);
        assert_eq!(r.e.call(0x00df_2640, &args![r.this, r.a1, r.a2]).u32(), 1);
        assert!(r.frees.borrow().is_empty());
        assert_eq!(r.e.mem.u32(r.array + 4), 1);
    }

    #[test]
    fn a_full_array_grows_first() {
        let mut r = rig(1, 4, 0x4000_0004);
        let array = r.array;
        let growth = Rc::new(RefCell::new(Vec::new()));
        {
            let growth = growth.clone();
            r.e.register_double(ARRAY_GROW, move |e, a| {
                growth.borrow_mut().push(a.to_vec());
                let bigger = e.mem.alloc(0x100);
                let old = e.mem.u32(a[1]);
                for k in 0..4 {
                    let word = e.mem.u32(old + 4 * k);
                    e.mem.set_u32(bigger + 4 * k, word);
                }
                e.mem.set_u32(a[1], bigger);
                e.mem.set_u32(a[1] + 8, 0x40);
                Ret::default()
            });
        }
        assert_eq!(r.e.call(0x00df_2640, &args![r.this, r.a1, r.a2]).u32(), 1);
        let growth = growth.borrow();
        assert_eq!(growth.len(), 1);
        assert_eq!(growth[0][1..], [array, 4]);
        let data = r.e.mem.u32(array);
        assert_eq!(r.e.mem.u32(data + 16), r.items[0]);
        assert_eq!(r.e.mem.u32(array + 4), 5);
    }

    #[test]
    fn without_an_owner_the_counts_step() {
        let mut r = rig(0, 0, 16);
        r.e.mem.set_u32(r.owner + 0x60, 0);
        r.e.mem.set_u16(r.this + 0x20, 3);
        r.e.mem.set_u16(r.this + 0x10, 0xffff);
        assert_eq!(r.e.call(0x00df_2640, &args![r.this, r.a1, r.a2]).u32(), 1);
        assert_eq!(r.e.mem.u16(r.this + 0x20), 2);
        assert_eq!(r.e.mem.u16(r.this + 0x10), 0);
        assert_eq!(r.e.mem.u16(r.a2 + 0x20), 1);
        r.e.mem.set_u32(r.this + 0x18, 0);
        r.e.mem.set_u16(r.this + 0x20, 1);
        r.e.mem.set_u16(r.a2 + 0x20, 9);
        assert_eq!(r.e.call(0x00df_2640, &args![r.this, r.a1, r.a2]).u32(), 0);
        assert_eq!(r.e.mem.u16(r.this + 0x20), 1);
        assert_eq!(r.e.mem.u16(r.a2 + 0x20), 9);
    }
}

#[cfg(test)]
mod ray_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const HIT: u32 = 0x0a00_0304;
    const FINISH: u32 = 0x0a00_0308;
    const REACH: u32 = 0x0a00_030c;

    struct Ray {
        e: Engine,
        result: u32,
        tree: u32,
        sweep: u32,
        listener: u32,
        router: u32,
        base: u32,
        nodes: u32,
        boxes: u32,
        hits: Rc<RefCell<Vec<(u32, u32)>>>,
        finished: Rc<Cell<u32>>,
        reached: Rc<Cell<u32>>,
        cell_in_hit: Rc<Cell<u32>>,
    }

    /// The constants of the exe the walk reads.
    fn constants(e: &mut Engine) {
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(FLOAT_ZERO, 0.0);
        e.mem.set_f32(FLOAT_THREE, 3.0);
        e.mem.set_f32(FLOAT_HALF, 0.5);
        for lane in 0..4 {
            e.mem.set_f32(ONES_VECTOR + 4 * lane, 1.0);
            e.mem
                .set_u32(MASK_XYZ + 4 * lane, if lane < 3 { u32::MAX } else { 0 });
            e.mem.set_u32(BOX_SCALE + 4 * lane, 0x3780_0080);
            e.mem.set_u32(AXIS_BITS + 4 * lane, 1 << lane);
        }
        e.mem.set_u32(NEGATIVE_FLOAT_MAX, 0xff7f_ffee);
        e.mem.set_u32(BIG_FLOAT, 0x5a0e_1bca);
        e.mem.set_u32(FLOAT_MAX, 0x7f7f_ffee);
        e.mem.set_f64(WIDE_SCALE, 65535.0);
        e.mem.set_f64(TOLERANCE, 0.0001);
    }

    /// A tree whose boxes span 0..65535 on every axis (so a quantised word is a coordinate), a
    /// sweep from `from` to `to`, and a listener whose slot 0xc answers 1.0.
    fn ray(from: [f32; 3], to: [f32; 3]) -> Ray {
        let mut e = Engine::new();
        constants(&mut e);
        let result = e.mem.alloc(0x10);
        let tree = e.mem.alloc(0x50);
        let sweep = e.mem.alloc(0x30);
        let listener = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        let base = e.mem.alloc(0x1000);
        let nodes = e.mem.alloc(0x100);
        let boxes = e.mem.alloc(0x100);
        e.mem.set_u32(router, base);
        e.mem.set_u32(tree + 0x1c, boxes);
        for lane in 0..3u32 {
            e.mem.set_f32(tree + 0x40 + 4 * lane, 65535.0);
            e.mem.set_f32(sweep + 4 * lane, from[lane as usize]);
            e.mem.set_f32(sweep + 0x10 + 4 * lane, to[lane as usize]);
        }
        e.mem.set_u32(listener, vtable);
        e.mem.set_u32(vtable + 4, HIT);
        e.mem.set_u32(vtable + 8, FINISH);
        e.mem.set_u32(vtable + 0xc, REACH);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let hits: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        let finished = Rc::new(Cell::new(0));
        let reached = Rc::new(Cell::new(0));
        let cell_in_hit = Rc::new(Cell::new(0));
        {
            let hits = hits.clone();
            let cell = cell_in_hit.clone();
            e.register_double(HIT, move |e, a| {
                hits.borrow_mut().push((a[1], a[2]));
                cell.set(e.mem.u32(router));
                Ret::default()
            });
        }
        {
            let finished = finished.clone();
            e.register_double(FINISH, move |_, a| {
                assert_eq!(a.len(), 2);
                finished.set(a[1]);
                Ret::default()
            });
        }
        {
            let reached = reached.clone();
            e.register_double(REACH, move |_, _| {
                reached.set(reached.get() + 1);
                Ret {
                    st0: 1.0,
                    ..Ret::default()
                }
            });
        }
        Ray {
            e,
            result,
            tree,
            sweep,
            listener,
            router,
            base,
            nodes,
            boxes,
            hits,
            finished,
            reached,
            cell_in_hit,
        }
    }

    impl Ray {
        /// The two dwords of node `at`.
        fn node(&mut self, at: u32, header: u32, aux: u32) {
            self.e.mem.set_u32(at, header);
            self.e.mem.set_u32(at + 4, aux);
        }

        /// Box `index` of the leaf block: minimum, maximum, and the two user words.
        fn leaf(&mut self, index: u32, min: [u16; 3], max: [u16; 3], low: u16, high: u16) {
            let at = self.boxes + 0x10 * index;
            for axis in 0..3u32 {
                self.e.mem.set_u16(at + 2 * axis, min[axis as usize]);
                self.e.mem.set_u16(at + 8 + 2 * axis, max[axis as usize]);
            }
            self.e.mem.set_u16(at + 6, low);
            self.e.mem.set_u16(at + 0xe, high);
        }

        /// Makes `root` the root node and runs the walk with radius 1.
        fn run(&mut self, root: u32) -> u32 {
            self.e.mem.set_u32(self.tree, root);
            let (result, tree, sweep, listener) =
                (self.result, self.tree, self.sweep, self.listener);
            self.e
                .call(0x00df_2750, &args![result, tree, sweep, listener, 1.0f32])
                .u32()
        }

        fn data(&self) -> Vec<u32> {
            self.hits.borrow().iter().map(|hit| hit.1).collect()
        }
    }

    #[test]
    fn a_leaf_box_on_the_ray_reaches_the_listener_once() {
        let mut r = ray([0.0; 3], [10.0, 0.0, 0.0]);
        let root = r.nodes;
        r.node(root, 0, 1);
        r.leaf(0, [5, 0, 0], [6, 2, 2], 0x1111, 0x2222);
        assert_eq!(r.run(root), r.result);
        assert_eq!(r.data(), vec![0x2222_1111]);
        assert_eq!(r.finished.get(), r.result);
        assert_eq!(r.reached.get(), 1);
        // The stack allocator's word was advanced for the walk and put back after it.
        assert_eq!(r.cell_in_hit.get(), r.base + 0x800);
        assert_eq!(r.e.mem.u32(r.router), r.base);
    }

    #[test]
    fn a_box_beyond_the_end_of_the_ray_is_not_reported() {
        let mut r = ray([0.0; 3], [10.0, 0.0, 0.0]);
        let root = r.nodes;
        r.node(root, 0, 1);
        r.leaf(0, [20, 0, 0], [21, 2, 2], 1, 2);
        r.run(root);
        assert!(r.data().is_empty());
        assert_eq!(r.finished.get(), r.result);
    }

    #[test]
    fn a_box_beside_the_ray_is_not_reported() {
        let mut r = ray([0.0; 3], [10.0, 0.0, 0.0]);
        let root = r.nodes;
        r.node(root, 0, 1);
        r.leaf(0, [5, 5, 0], [6, 7, 2], 1, 2);
        r.run(root);
        assert!(r.data().is_empty());
    }

    #[test]
    fn two_leaves_under_an_inner_node_are_visited_nearest_first() {
        let mut r = ray([0.0; 3], [1000.0, 0.0, 0.0]);
        let root = r.nodes;
        // An inner node on axis 0 (header with the top bit, the children 0x10 bytes on) whose
        // two children are leaves 0 and 1; its slab is 100..200.
        r.node(root, 0x8000_0010, 100 | (200 << 16));
        r.node(root + 0x10, 0, 1);
        r.node(root + 0x18, 1, 1);
        r.leaf(0, [10, 0, 0], [20, 2, 2], 0xa1, 0xb1);
        r.leaf(1, [300, 0, 0], [310, 2, 2], 0xa2, 0xb2);
        r.run(root);
        assert_eq!(r.data(), vec![0x00b1_00a1, 0x00b2_00a2]);
        assert_eq!(r.finished.get(), r.result);
        assert_eq!(r.e.mem.u32(r.router), r.base);
        // The listener's reach is asked for at the start, at the split and when the saved
        // child is taken back.
        assert_eq!(r.reached.get(), 3);
    }

    #[test]
    fn a_ray_parallel_to_the_split_axis_inside_its_slab_visits_both_children() {
        let mut r = ray([150.0, 0.0, 0.0], [150.0, 1000.0, 0.0]);
        let root = r.nodes;
        r.node(root, 0x8000_0010, 100 | (200 << 16));
        r.node(root + 0x10, 0, 1);
        r.node(root + 0x18, 1, 1);
        r.leaf(0, [140, 10, 0], [160, 20, 2], 0xa1, 0xb1);
        r.leaf(1, [140, 300, 0], [160, 310, 2], 0xa2, 0xb2);
        r.run(root);
        assert_eq!(r.data(), vec![0x00b1_00a1, 0x00b2_00a2]);
    }

    #[test]
    fn a_ray_parallel_to_the_split_axis_outside_its_slab_visits_one_child() {
        let mut r = ray([50.0, 0.0, 0.0], [50.0, 1000.0, 0.0]);
        let root = r.nodes;
        r.node(root, 0x8000_0010, 100 | (200 << 16));
        r.node(root + 0x10, 0, 1);
        r.node(root + 0x18, 1, 1);
        r.leaf(0, [40, 10, 0], [60, 20, 2], 0xa1, 0xb1);
        r.leaf(1, [40, 300, 0], [60, 310, 2], 0xa2, 0xb2);
        r.run(root);
        assert_eq!(r.data(), vec![0x00b1_00a1]);
    }

    #[test]
    fn an_empty_leaf_header_is_skipped() {
        let mut r = ray([0.0; 3], [10.0, 0.0, 0.0]);
        let root = r.nodes;
        r.node(root, 0x7fff_ffff, 1);
        r.run(root);
        assert!(r.data().is_empty());
        assert_eq!(r.finished.get(), r.result);
        r.node(root, 0, 0);
        r.run(root);
        assert!(r.data().is_empty());
    }

    #[test]
    fn a_zero_length_sweep_ends_at_once() {
        let mut r = ray([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]);
        let root = r.nodes;
        r.node(root, 0, 1);
        r.leaf(0, [0, 0, 0], [9, 9, 9], 1, 2);
        assert_eq!(r.run(root), r.result);
        assert!(r.data().is_empty());
        assert_eq!(r.finished.get(), r.result);
        assert_eq!(r.reached.get(), 1);
        assert_eq!(r.e.mem.u32(r.router), r.base);
    }
}
