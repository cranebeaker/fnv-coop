//! `memset`, `memcpy` and `memmove` with their SSE2 paths, `memcmp`, `memchr`, `memcpy_s` and the
//! game's two wrappers. The exe's jump tables and unrolled moves become runs of bytes in the same
//! direction (see the family's notes); the paths, their tests and every call stay the exe's.

use super::{
    chunk, fill, low_byte, move_down, move_up, overlaps_above, pair_room, signed, small, unsigned,
    FAMILY, SCAN, SSE2_AVAILABLE,
};
use engine_core::{args, typed_call, Engine, Ptr};

/// `errno` (`_errno`, in this family) and the invalid-parameter report (`00ec7c30`, five words).
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
/// `EINVAL` and `ERANGE`.
const EINVAL: u32 = 0x16;
const ERANGE: u32 = 0x22;

#[inline(always)]
fn sse2_available(e: &Engine) -> bool {
    e.mem.u32(SSE2_AVAILABLE) != 0
}

/// `x % 16` as the code computes it (`CDQ` and friends): the signed remainder of the address.
#[inline(always)]
fn signed_rem16(x: u32) -> i32 {
    signed(x).wrapping_rem(16)
}

/// `REP MOVSB` forward: byte by byte where the destination overlaps the source from above (each
/// byte may be one just written), a run otherwise.
fn movsb(e: &Engine, dst: u32, src: u32, count: u32) {
    if !overlaps_above(dst, src, count) {
        move_up(e, dst, src, count);
        return;
    }
    for i in 0..count {
        let b = e.mem.u8(src.wrapping_add(i));
        e.mem.set_u8(dst.wrapping_add(i), b);
    }
}

/// `REP MOVSD` forward (`count` dwords), as [`movsb`] with dword steps.
fn movsd(e: &Engine, dst: u32, src: u32, count: u32) {
    let bytes = count.wrapping_mul(4);
    if !overlaps_above(dst, src, bytes) {
        move_up(e, dst, src, bytes);
        return;
    }
    for i in 0..count {
        let w = e.mem.u32(src.wrapping_add(4 * i));
        e.mem.set_u32(dst.wrapping_add(4 * i), w);
    }
}

// Typed from 00ec61c0
/// `memset`, cdecl: destination, value, count; answers the destination. A zero value with 0x100
/// bytes or more and SSE2 goes to `__VEC_memzero` (a jump: its answer); otherwise the bytes are
/// stored (heads, dwords and tails in the exe).
pub fn memset(e: &mut Engine, dst: u32, value: u32, count: u32) -> u32 {
    if count == 0 {
        return dst;
    }
    let byte = low_byte(value);
    if byte == 0 && count >= 0x100 && sse2_available(e) {
        return typed_call!(FAMILY, 0x00ed_2cd5, vec_memzero(e, dst, value, count));
    }
    fill(e, dst, byte, count);
    dst
}

// Typed from 00ed2c7e
/// `fastzero_I`, cdecl: destination, size: `size >> 7` blocks of 128 zero bytes; the loop runs at
/// least once (`DEC; JNZ`, so 0 blocks run 2^32 times until a fault).
pub fn fastzero_i(e: &mut Engine, dst: u32, size: u32) {
    let blocks = size >> 7;
    if blocks != 0 {
        e.mem.zero(dst, blocks << 7);
        return;
    }
    let mut at = dst;
    loop {
        e.mem.zero(at, 0x80);
        at = at.wrapping_add(0x80);
    }
}

// Typed from 00ed2cd5
/// `__VEC_memzero`, cdecl: destination, value (unread), size; answers the destination. 16-aligned (the
/// signed remainder 0): the whole 128-byte blocks through `fastzero_I`, the rest cleared here;
/// otherwise the bytes up to the next 16-aligned address, then the rest by a call to itself.
pub fn vec_memzero(e: &mut Engine, dst: u32, _value: u32, size: u32) -> u32 {
    let misaligned = signed_rem16(dst);
    if misaligned == 0 {
        let rest = size & 0x7f;
        if size != rest {
            typed_call!(
                FAMILY,
                0x00ed_2c7e,
                fastzero_i(e, dst, size.wrapping_sub(rest))
            );
        }
        if rest != 0 {
            e.mem.zero(dst.wrapping_add(size).wrapping_sub(rest), rest);
        }
        return dst;
    }
    let head = unsigned(16i32.wrapping_sub(misaligned));
    e.mem.zero(dst, head);
    let _: u32 = typed_call!(
        FAMILY,
        0x00ed_2cd5,
        vec_memzero(e, dst.wrapping_add(head), 0u32, size.wrapping_sub(head))
    );
    dst
}

/// The shared body of `memcpy` and `memmove`: a destination above the source inside its range copies
/// from the top down; otherwise 0x100 bytes or more with SSE2 and the same alignment within 16 bytes
/// go to `__VEC_memcpy` (a jump: its answer); the rest copies from the bottom up.
fn copy(e: &mut Engine, dst: u32, src: u32, count: u32) -> u32 {
    if overlaps_above(dst, src, count) {
        move_down(e, dst, src, count);
        return dst;
    }
    if count >= 0x100 && sse2_available(e) && dst & 0xf == src & 0xf {
        return typed_call!(FAMILY, 0x00ed_17f4, vec_memcpy(e, dst, src, count));
    }
    move_up(e, dst, src, count);
    dst
}

// Typed from 00ec44d0
/// `memcpy`, cdecl: destination, source, count; answers the destination. It handles overlap as
/// `memmove` does.
pub fn memcpy(e: &mut Engine, dst: u32, src: u32, count: u32) -> u32 {
    copy(e, dst, src, count)
}

// Typed from 00ec7230
/// `memmove`: the same code as `memcpy` with its own jump tables.
pub fn memmove(e: &mut Engine, dst: u32, src: u32, count: u32) -> u32 {
    copy(e, dst, src, count)
}

// Typed from 00ed17f4
/// `__VEC_memcpy`, cdecl: destination, source, count; answers the destination. Both 16-aligned: the
/// whole 128-byte blocks through `00ed176d`, the rest with `REP MOVSB`. The same misalignment: the
/// bytes up to 16-alignment, then the rest by a call to itself. Different ones: `REP MOVSD` and the
/// last 0 to 3 bytes.
pub fn vec_memcpy(e: &mut Engine, dst: u32, src: u32, count: u32) -> u32 {
    let src_rem = signed_rem16(src);
    let dst_rem = signed_rem16(dst);
    if src_rem | dst_rem == 0 {
        let rest = count & 0x7f;
        if count != rest {
            typed_call!(
                FAMILY,
                0x00ed_176d,
                vec_memcpy_blocks(e, dst, src, count.wrapping_sub(rest))
            );
        }
        if rest != 0 {
            let from = src.wrapping_add(count).wrapping_sub(rest);
            let to = dst.wrapping_add(count).wrapping_sub(rest);
            movsb(e, to, from, rest);
        }
        return dst;
    }
    if src_rem == dst_rem {
        let head = unsigned(16i32.wrapping_sub(src_rem));
        movsb(e, dst, src, head);
        let _: u32 = typed_call!(
            FAMILY,
            0x00ed_17f4,
            vec_memcpy(
                e,
                dst.wrapping_add(head),
                src.wrapping_add(head),
                count.wrapping_sub(head)
            )
        );
        return dst;
    }
    movsd(e, dst, src, count >> 2);
    movsb(
        e,
        dst.wrapping_add(count & !3),
        src.wrapping_add(count & !3),
        count & 3,
    );
    dst
}

// Typed from 00ed176d
/// `__VEC_memcpy`'s block loop, cdecl: destination, source, size: `size >> 7` blocks of 128 bytes,
/// each moved as two groups of four 16-byte loads and four stores; at least once (`DEC; JNZ`).
pub fn vec_memcpy_blocks(e: &mut Engine, dst: u32, src: u32, size: u32) {
    let blocks = size >> 7;
    // Groups of 64 bytes read before they are written: a run gives the same unless the destination
    // is above the source by less than a group's length.
    if blocks != 0 && !overlaps_above(dst, src, blocks << 7) {
        move_up(e, dst, src, blocks << 7);
        return;
    }
    let mut remaining = blocks;
    let (mut d, mut s) = (dst, src);
    let mut group = [0u8; 0x40];
    loop {
        for half in 0..2u32 {
            e.mem.read(s.wrapping_add(half * 0x40), &mut group);
            e.mem.write(d.wrapping_add(half * 0x40), &group);
        }
        s = s.wrapping_add(0x80);
        d = d.wrapping_add(0x80);
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            break;
        }
    }
}

// Typed from 00ec4835
/// `memcmp`, cdecl: two buffers and a count; 0, 1 or -1 by the first differing byte (unsigned).
pub fn memcmp(e: &mut Engine, a: u32, b: u32, count: u32) -> i32 {
    let (mut x, mut y) = ([0u8; SCAN], [0u8; SCAN]);
    let mut done = 0u32;
    while done < count {
        let (pa, pb) = (a.wrapping_add(done), b.wrapping_add(done));
        let n = pair_room(pa, pb, count - done);
        e.mem.read(pa, &mut x[..n]);
        e.mem.read(pb, &mut y[..n]);
        if let Some(i) = (0..n).find(|&i| x[i] != y[i]) {
            return if x[i] > y[i] { 1 } else { -1 };
        }
        done += small(n);
    }
    0
}

// Typed from 00eca2e0
/// `memchr`, cdecl: buffer, value, count; the address of the first byte equal to the value's low
/// byte within the count, or null.
pub fn memchr(e: &mut Engine, buffer: u32, value: u32, count: u32) -> u32 {
    let byte = low_byte(value);
    let mut buf = [0u8; SCAN];
    let mut done = 0u32;
    while done < count {
        let at = buffer.wrapping_add(done);
        let run = chunk(e, at, count - done, &mut buf);
        if let Some(i) = run.iter().position(|&c| c == byte) {
            return at.wrapping_add(small(i));
        }
        done += small(run.len());
    }
    0
}

// Typed from 00401460
/// The game's `memcpy` wrapper: the runtime's `memcpy`.
pub fn memcpy_wrapper(e: &mut Engine, dst: Ptr, src: Ptr, n: u32) -> Ptr {
    let to: u32 = typed_call!(FAMILY, 0x00ec_44d0, memcpy(e, dst.addr(), src.addr(), n));
    Ptr::new(to)
}

// Typed from 00403d30
/// The game's `memset` wrapper: the runtime's `memset`.
pub fn memset_wrapper(e: &mut Engine, dst: Ptr, c: i32, n: u32) -> Ptr {
    let to: u32 = typed_call!(FAMILY, 0x00ec_61c0, memset(e, dst.addr(), unsigned(c), n));
    Ptr::new(to)
}

/// `errno` set, then the invalid-parameter report with five zeros.
fn invalid(e: &mut Engine, code: u32) -> u32 {
    let slot: Ptr = typed_call!(FAMILY, 0x00ec_85e3, super::math::errno(e));
    e.mem.set_u32(slot.addr(), code);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
    code
}

// Typed from 00ec7c66
/// `memcpy_s`, cdecl: (destination, its size, source, count). A count of 0 answers 0; a null
/// destination is `EINVAL` with the report. A source with room is `memcpy` and 0; otherwise the
/// destination is zeroed (`memset` of its size), and a null source is `EINVAL` with the report, too
/// little room `ERANGE` with it (the branch for a source with room answers `EINVAL` without it).
pub fn memcpy_s(e: &mut Engine, destination: Ptr, size: u32, source: Ptr, count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    if destination.addr() == 0 {
        return invalid(e, EINVAL);
    }
    if source.addr() != 0 && size >= count {
        let _: u32 = typed_call!(
            FAMILY,
            0x00ec_44d0,
            memcpy(e, destination.addr(), source.addr(), count)
        );
        return 0;
    }
    let _: u32 = typed_call!(
        FAMILY,
        0x00ec_61c0,
        memset(e, destination.addr(), 0u32, size)
    );
    if source.addr() == 0 {
        return invalid(e, EINVAL);
    }
    if size >= count {
        return EINVAL;
    }
    invalid(e, ERANGE)
}
