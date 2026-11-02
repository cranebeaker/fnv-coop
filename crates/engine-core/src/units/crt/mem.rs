//! The C runtime's memory routines (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe
//! 1.4.0.525), translated from the exe: `memset` (`00ec61c0`) with the SSE2 zeroing
//! `__VEC_memzero` (`00ed2cd5`) and `fastzero_I` (`00ed2c7e`), `memcpy` (`00ec44d0`) and
//! `memmove` (`00ec7230`, the same code with its own jump tables) with the SSE2 copy
//! `__VEC_memcpy` (`00ed17f4`) and its aligned block loop (`00ed176d`), `memcmp` (`00ec4835`)
//! and `memchr` (`00eca2e0`). The names are Ghidra's function-ID labels.
//!
//! The copy routines are written in the exe as jump tables into unrolled moves; here each
//! path is the same sequence of moves (bytes, dwords, 16-byte blocks) in the same order and
//! direction, so overlapping copies give the exe's result. The SSE2 paths are taken only when
//! `__sse2_available` (`01270a6c`) is set, as the CRT start-up's CPU check leaves it.

use super::*;

/// `__sse2_available` (`01270a6c`): nonzero when the CRT may use SSE2 (set at start-up).
const SSE2_AVAILABLE: u32 = 0x0127_0a6c;
/// The functions of this file the exe calls (`CALL`/`JMP`).
const VEC_MEMZERO: u32 = 0x00ed_2cd5;
const FASTZERO_I: u32 = 0x00ed_2c7e;
const VEC_MEMCPY: u32 = 0x00ed_17f4;
const VEC_MEMCPY_BLOCKS: u32 = 0x00ed_176d;

fn sse2_available(e: &Engine) -> bool {
    e.mem.u32(SSE2_AVAILABLE) != 0
}

/// `REP MOVSB` forward.
fn movsb(e: &mut Engine, dst: u32, src: u32, count: u32) {
    for i in 0..count {
        let b = e.mem.u8(src.wrapping_add(i));
        e.mem.set_u8(dst.wrapping_add(i), b);
    }
}

/// `REP MOVSD` forward (dwords).
fn movsd(e: &mut Engine, dst: u32, src: u32, count: u32) {
    for i in 0..count {
        let w = e.mem.u32(src.wrapping_add(4 * i));
        e.mem.set_u32(dst.wrapping_add(4 * i), w);
    }
}

/// `STD; REP MOVSD; CLD`: dwords from the given last ones down.
fn movsd_down(e: &mut Engine, last_dst: u32, last_src: u32, count: u32) {
    for i in 0..count {
        let w = e.mem.u32(last_src.wrapping_sub(4 * i));
        e.mem.set_u32(last_dst.wrapping_sub(4 * i), w);
    }
}

/// `x % 16` as the code computes it (`CDQ`, `XOR`, `SUB`, `AND 0xf`, `XOR`, `SUB`: the signed
/// remainder of the address).
fn signed_rem16(x: u32) -> i32 {
    (x as i32).wrapping_rem(16)
}

// Translated from 00ec61c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `memset` (Ghidra's function ID), cdecl: destination, value, count; answers the
/// destination. A count of 0 does nothing. A zero value with a count of 0x100 or more and SSE2
/// goes to `__VEC_memzero` (a jump: its answer). Otherwise fewer than 4 bytes are stored one by
/// one; else the bytes up to the next 4-aligned address, then dwords of the value repeated
/// (`REP STOSD`), then the remaining 0 to 3 bytes.
pub fn memset(e: &mut Engine, dst: u32, value: u32, count: u32) -> u32 {
    if count == 0 {
        return dst;
    }
    let byte = value as u8;
    if byte == 0 && count >= 0x100 && sse2_available(e) {
        return e.call(VEC_MEMZERO, &args![dst, value, count]).u32();
    }
    let mut at = dst;
    let mut left = count;
    if left >= 4 {
        let head = dst.wrapping_neg() & 3;
        left -= head;
        for _ in 0..head {
            e.mem.set_u8(at, byte);
            at = at.wrapping_add(1);
        }
        let word = u32::from(byte) * 0x0101_0101;
        let dwords = left >> 2;
        left &= 3;
        for _ in 0..dwords {
            e.mem.set_u32(at, word);
            at = at.wrapping_add(4);
        }
    }
    for _ in 0..left {
        e.mem.set_u8(at, byte);
        at = at.wrapping_add(1);
    }
    dst
}

// Translated from 00ed2c7e (disassembled, FalloutNV.exe 1.4.0.525)
/// `fastzero_I` (Ghidra's function ID), cdecl: destination, size: `size >> 7` blocks of 128
/// zero bytes (`MOVDQA` of a cleared register, eight to a block; the loop runs at least once,
/// `DEC; JNZ`).
pub fn fastzero_i(e: &mut Engine, dst: u32, size: u32) {
    let mut blocks = size >> 7;
    let mut at = dst;
    loop {
        for i in 0..0x20u32 {
            e.mem.set_u32(at.wrapping_add(4 * i), 0);
        }
        at = at.wrapping_add(0x80);
        blocks = blocks.wrapping_sub(1);
        if blocks == 0 {
            break;
        }
    }
}

// Translated from 00ed2cd5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__VEC_memzero` (Ghidra's function ID), cdecl: destination, value (not read), size; answers
/// the destination. A 16-aligned destination (the signed remainder of the address by 16 is 0):
/// the whole 128-byte blocks go to `fastzero_I` and the rest is cleared with `REP STOSB`.
/// Otherwise the bytes up to the next 16-aligned address are cleared and the rest is done by a
/// call to itself.
pub fn vec_memzero(e: &mut Engine, dst: u32, _value: u32, size: u32) -> u32 {
    let misaligned = signed_rem16(dst);
    if misaligned == 0 {
        let rest = size & 0x7f;
        if size != rest {
            e.call(FASTZERO_I, &args![dst, size.wrapping_sub(rest)]);
        }
        if rest != 0 {
            let start = dst.wrapping_add(size).wrapping_sub(rest);
            for i in 0..rest {
                e.mem.set_u8(start.wrapping_add(i), 0);
            }
        }
        return dst;
    }
    let head = (16i32.wrapping_sub(misaligned)) as u32;
    for i in 0..head {
        e.mem.set_u8(dst.wrapping_add(i), 0);
    }
    e.call(
        VEC_MEMZERO,
        &args![dst.wrapping_add(head), 0u32, size.wrapping_sub(head)],
    );
    dst
}

/// `memcpy`/`memmove`'s copy without SSE2 (the jump-table paths), forward or backward.
fn copy_x86(e: &mut Engine, dst: u32, src: u32, count: u32) {
    let overlaps_above = dst > src && dst < count.wrapping_add(src);
    if !overlaps_above {
        // Forward: 0 to 3 bytes up to a 4-aligned destination (only with 4 or more bytes),
        // the dwords, then the last 0 to 3 bytes.
        let (mut d, mut s, mut n) = (dst, src, count);
        if d & 3 != 0 && n >= 4 {
            let head = 4 - (d & 3);
            movsb(e, d, s, head);
            d = d.wrapping_add(head);
            s = s.wrapping_add(head);
            n -= head;
        }
        if d & 3 == 0 || n >= 4 {
            let dwords = n >> 2;
            movsd(e, d, s, dwords);
            d = d.wrapping_add(4 * dwords);
            s = s.wrapping_add(4 * dwords);
            n &= 3;
        }
        movsb(e, d, s, n);
        return;
    }
    // Backward (00ec4694): from the last dword down. The bytes above a 4-aligned last
    // destination dword first (only with 4 or more bytes), then the dwords, then the rest,
    // each from the top down.
    let mut last_s = count.wrapping_add(src).wrapping_sub(4);
    let mut last_d = count.wrapping_add(dst).wrapping_sub(4);
    let mut n = count;
    if last_d & 3 != 0 && n >= 4 {
        let top = last_d & 3;
        for k in (0..top).rev() {
            let b = e
                .mem
                .u8(last_s.wrapping_add(4).wrapping_sub(top).wrapping_add(k));
            e.mem
                .set_u8(last_d.wrapping_add(4).wrapping_sub(top).wrapping_add(k), b);
        }
        last_s = last_s.wrapping_sub(top);
        last_d = last_d.wrapping_sub(top);
        n -= top;
    }
    if last_d & 3 == 0 || n >= 4 {
        let dwords = n >> 2;
        movsd_down(e, last_d, last_s, dwords);
        last_s = last_s.wrapping_sub(4 * dwords);
        last_d = last_d.wrapping_sub(4 * dwords);
        n &= 3;
    }
    for k in (0..n).rev() {
        let b = e
            .mem
            .u8(last_s.wrapping_add(4).wrapping_sub(n).wrapping_add(k));
        e.mem
            .set_u8(last_d.wrapping_add(4).wrapping_sub(n).wrapping_add(k), b);
    }
}

/// The shared body of `memcpy` and `memmove`: an overlapping copy to a higher address goes
/// backward; otherwise 0x100 bytes or more with SSE2 and the same alignment within 16 bytes go
/// to `__VEC_memcpy` (a jump: its answer); the rest copies forward. Answers the destination.
fn copy(e: &mut Engine, dst: u32, src: u32, count: u32) -> u32 {
    let overlaps_above = dst > src && dst < count.wrapping_add(src);
    if !overlaps_above && count >= 0x100 && sse2_available(e) && dst & 0xf == src & 0xf {
        return e.call(VEC_MEMCPY, &args![dst, src, count]).u32();
    }
    copy_x86(e, dst, src, count);
    dst
}

// Translated from 00ec44d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `memcpy` (Ghidra's function ID), cdecl: destination, source, count; answers the
/// destination. The code handles overlap like `memmove`: a destination above the source and
/// inside the source range copies from the top down (dwords, with the 0 to 3 odd bytes at each
/// end moved one by one); otherwise 0x100 bytes or more with SSE2 available and the two
/// addresses equal modulo 16 go to `__VEC_memcpy`, and everything else copies up: the bytes up to
/// a 4-aligned destination, the dwords (`REP MOVSD`, or the unrolled moves for fewer than 8),
/// then the last bytes.
pub fn memcpy(e: &mut Engine, dst: u32, src: u32, count: u32) -> u32 {
    copy(e, dst, src, count)
}

// Translated from 00ec7230 (disassembled, FalloutNV.exe 1.4.0.525)
/// `memmove` (Ghidra's function ID): the same code as [`memcpy`] with its own jump tables.
pub fn memmove(e: &mut Engine, dst: u32, src: u32, count: u32) -> u32 {
    copy(e, dst, src, count)
}

// Translated from 00ed17f4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__VEC_memcpy` (Ghidra's function ID), cdecl: destination, source, count; answers the
/// destination. Both 16-aligned (signed remainders 0): the whole 128-byte blocks through
/// `00ed176d`, the rest with `REP MOVSB`. The same misalignment: the bytes up to 16-alignment
/// with `REP MOVSB`, then the rest by a call to itself. Different misalignments: `REP MOVSD` of
/// the dwords and `REP MOVSB` of the last 0 to 3 bytes.
pub fn vec_memcpy(e: &mut Engine, dst: u32, src: u32, count: u32) -> u32 {
    let src_rem = signed_rem16(src);
    let dst_rem = signed_rem16(dst);
    if src_rem | dst_rem == 0 {
        let rest = count & 0x7f;
        if count != rest {
            e.call(
                VEC_MEMCPY_BLOCKS,
                &args![dst, src, count.wrapping_sub(rest)],
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
        let head = (16i32.wrapping_sub(src_rem)) as u32;
        movsb(e, dst, src, head);
        e.call(
            VEC_MEMCPY,
            &args![
                dst.wrapping_add(head),
                src.wrapping_add(head),
                count.wrapping_sub(head)
            ],
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

// Translated from 00ed176d (disassembled, FalloutNV.exe 1.4.0.525)
/// `__VEC_memcpy`'s block loop (no label), cdecl: destination, source, size: `size >> 7` blocks
/// of 128 bytes, each read as eight 16-byte loads (`MOVDQA`, two groups of four) and written
/// after each group of four (the loop runs at least once, `DEC; JNZ`).
pub fn vec_memcpy_blocks(e: &mut Engine, dst: u32, src: u32, size: u32) {
    let mut blocks = size >> 7;
    let (mut d, mut s) = (dst, src);
    loop {
        for half in 0..2u32 {
            let base = half * 0x40;
            let words: Vec<u32> = (0..16u32)
                .map(|i| e.mem.u32(s.wrapping_add(base + 4 * i)))
                .collect();
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(d.wrapping_add(base + 4 * i as u32), *w);
            }
        }
        s = s.wrapping_add(0x80);
        d = d.wrapping_add(0x80);
        blocks = blocks.wrapping_sub(1);
        if blocks == 0 {
            break;
        }
    }
}

/// `memcmp`'s byte step: 0 when equal, else 1 or -1 (`SETG`, `LEA [EBX*2-1]`).
fn byte_order(a: u8, b: u8) -> i32 {
    match i32::from(a) - i32::from(b) {
        0 => 0,
        d if d > 0 => 1,
        _ => -1,
    }
}

// Translated from 00ec4835 (disassembled, FalloutNV.exe 1.4.0.525)
/// `memcmp` (Ghidra's function ID), cdecl: two buffers and a count; answers 0, 1 or -1 by the
/// first differing byte (unsigned). Counts 0 to 4 compare byte by byte. Larger counts go in
/// blocks of 32 bytes: each dword compared whole first and, when it differs, byte by byte; the
/// last 0 to 31 bytes through the jump table the same way, dwords then bytes, in ascending
/// order. (The exe's unrolled code; the comparisons and their order are these.)
pub fn memcmp(e: &mut Engine, a: u32, b: u32, count: u32) -> i32 {
    let compare_bytes = |e: &Engine, at: u32, n: u32| -> i32 {
        for i in 0..n {
            let r = byte_order(
                e.mem.u8(a.wrapping_add(at + i)),
                e.mem.u8(b.wrapping_add(at + i)),
            );
            if r != 0 {
                return r;
            }
        }
        0
    };
    if count <= 4 {
        return compare_bytes(e, 0, count);
    }
    let mut at = 0u32;
    while count - at >= 4 {
        let (wa, wb) = (e.mem.u32(a.wrapping_add(at)), e.mem.u32(b.wrapping_add(at)));
        if wa != wb {
            return compare_bytes(e, at, 4);
        }
        at += 4;
    }
    compare_bytes(e, at, count - at)
}

// Translated from 00eca2e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `memchr` (Ghidra's function ID), cdecl: buffer, value, count; answers the address of the
/// first byte equal to the value's low byte, or null. Byte by byte up to a 4-aligned address;
/// then, while 4 or more bytes are left, a dword at a time: only when the zero-byte test of the
/// dword XOR the repeated value (`((x ^ ~0) ^ (x + 0x7efefeff)) & 0x81010100`) is nonzero are
/// its four bytes checked in order; then the last bytes one by one.
pub fn memchr(e: &mut Engine, buffer: u32, value: u32, count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    let byte = value as u8;
    let mut at = buffer;
    let mut left = count;
    while at & 3 != 0 {
        let c = e.mem.u8(at);
        at = at.wrapping_add(1);
        if c == byte {
            return at.wrapping_sub(1);
        }
        left -= 1;
        if left == 0 {
            return 0;
        }
    }
    let mut rest = left.wrapping_sub(4);
    if left > 3 {
        let pattern = u32::from(byte) * 0x0101_0101;
        loop {
            let word = e.mem.u32(at);
            let x = word ^ pattern;
            if ((x ^ 0xffff_ffff) ^ x.wrapping_add(0x7efe_feff)) & 0x8101_0100 != 0 {
                for k in 0..4u32 {
                    if (word >> (8 * k)) as u8 == byte {
                        return at.wrapping_add(k);
                    }
                }
            }
            at = at.wrapping_add(4);
            let more = rest > 3;
            rest = rest.wrapping_sub(4);
            if !more {
                break;
            }
        }
    }
    let mut n = rest.wrapping_add(4);
    while n != 0 {
        let c = e.mem.u8(at);
        at = at.wrapping_add(1);
        if c == byte {
            return at.wrapping_sub(1);
        }
        n -= 1;
    }
    0
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_61c0, memset(u32, u32, u32) -> u32),
        entry!(0x00ed_2c7e, fastzero_i(u32, u32)),
        entry!(0x00ed_2cd5, vec_memzero(u32, u32, u32) -> u32),
        entry!(0x00ec_44d0, memcpy(u32, u32, u32) -> u32),
        entry!(0x00ec_7230, memmove(u32, u32, u32) -> u32),
        entry!(0x00ed_17f4, vec_memcpy(u32, u32, u32) -> u32),
        entry!(0x00ed_176d, vec_memcpy_blocks(u32, u32, u32)),
        entry!(0x00ec_4835, memcmp(u32, u32, u32) -> i32),
        entry!(0x00ec_a2e0, memchr(u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// A 0x1000-byte block of the pattern `i * 7 + 1`, 16-aligned, and its address.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let at = 0x0800_0000;
        e.map(at, 0x1000);
        for i in 0..0x1000u32 {
            e.mem.set_u8(at + i, (i * 7 + 1) as u8);
        }
        e.call_log = Some(vec![]);
        (e, at)
    }

    fn pattern(i: u32) -> u8 {
        (i * 7 + 1) as u8
    }

    #[test]
    fn copies_match_memmove_for_every_overlap_and_alignment() {
        for &(d, s, n) in &[
            (0x100u32, 0x200u32, 0u32),
            (0x101, 0x203, 3),
            (0x101, 0x203, 37),
            (0x100, 0x104, 64),
            (0x104, 0x100, 64),
            (0x103, 0x100, 61),
            (0x100, 0x102, 9),
            (0x102, 0x100, 9),
            (0x101, 0x100, 300),
            (0x100, 0x101, 300),
            (0x110, 0x210, 0x300),
            (0x111, 0x222, 0x205),
        ] {
            for sse in [0u32, 1] {
                for entry in [0x00ec_44d0u32, 0x00ec_7230] {
                    let (mut e, at) = engine();
                    e.mem.set_u32(SSE2_AVAILABLE, sse);
                    let mut want: Vec<u8> = (0..0x1000).map(pattern).collect();
                    want.copy_within((s as usize)..((s + n) as usize), d as usize);
                    let r = e.call(entry, &args![at + d, at + s, n]).u32();
                    assert_eq!(r, at + d);
                    assert_eq!(e.mem.bytes(at, 0x1000), want, "{d:x} {s:x} {n:x} sse {sse}");
                }
            }
        }
    }

    #[test]
    fn sse2_copies_go_to_vec_memcpy_only_when_aligned_alike() {
        let (mut e, at) = engine();
        e.mem.set_u32(SSE2_AVAILABLE, 1);
        memcpy(&mut e, at + 0x810, at + 0x10, 0x200);
        assert_eq!(calls(&e, VEC_MEMCPY).len(), 1);
        assert_eq!(
            calls(&e, VEC_MEMCPY_BLOCKS),
            vec![vec![at + 0x810, at + 0x10, 0x200]]
        );
        e.call_log = Some(vec![]);
        memcpy(&mut e, at + 0x811, at + 0x10, 0x200);
        memcpy(&mut e, at + 0x810, at + 0x10, 0xff);
        e.mem.set_u32(SSE2_AVAILABLE, 0);
        memcpy(&mut e, at + 0x810, at + 0x10, 0x200);
        assert!(calls(&e, VEC_MEMCPY).is_empty());
    }

    #[test]
    fn memset_fills_and_sse2_zeroing_takes_its_own_path() {
        for &(d, n, v) in &[
            (0x101u32, 0u32, 5u32),
            (0x101, 3, 0x1aa),
            (0x101, 300, 7),
            (0x100, 0x300, 0),
            (0x105, 0x283, 0),
        ] {
            for sse in [0u32, 1] {
                let (mut e, at) = engine();
                e.mem.set_u32(SSE2_AVAILABLE, sse);
                let mut want: Vec<u8> = (0..0x1000).map(pattern).collect();
                for b in &mut want[d as usize..(d + n) as usize] {
                    *b = v as u8;
                }
                assert_eq!(e.call(0x00ec_61c0, &args![at + d, v, n]).u32(), at + d);
                assert_eq!(e.mem.bytes(at, 0x1000), want, "{d:x} {n:x} {v:x} sse {sse}");
                let zeroing = !calls(&e, VEC_MEMZERO).is_empty();
                assert_eq!(zeroing, sse == 1 && v as u8 == 0 && n >= 0x100);
            }
        }
    }

    #[test]
    fn memcmp_answers_the_sign_of_the_first_difference() {
        let mut e = Engine::new();
        let a = e.mem.alloc(64);
        let b = e.mem.alloc(64);
        for i in 0..64 {
            e.mem.set_u8(a + i, i as u8);
            e.mem.set_u8(b + i, i as u8);
        }
        for n in 0..64u32 {
            assert_eq!(memcmp(&mut e, a, b, n), 0);
        }
        for k in [0u32, 1, 3, 4, 5, 31, 32, 33, 50, 63] {
            e.mem.set_u8(b + k, 0x80);
            assert_eq!(memcmp(&mut e, a, b, 64), -1, "{k}");
            assert_eq!(memcmp(&mut e, b, a, 64), 1, "{k}");
            assert_eq!(memcmp(&mut e, a, b, k), 0);
            assert_eq!(e.call(0x00ec_4835, &args![b, a, k + 1]).i32(), 1);
            e.mem.set_u8(b + k, k as u8);
        }
    }

    #[test]
    fn memchr_finds_the_first_match_within_the_count() {
        let mut e = Engine::new();
        let a = e.mem.alloc(64) + 1;
        for i in 0..60 {
            e.mem.set_u8(a + i, 0x41 + (i % 26) as u8);
        }
        assert_eq!(memchr(&mut e, a, 0x41 + 3, 60), a + 3);
        assert_eq!(memchr(&mut e, a, 0x141 + 30 % 26, 60), a + 4);
        assert_eq!(memchr(&mut e, a, 0x41 + 3, 3), 0);
        assert_eq!(memchr(&mut e, a, 0x41 + 25, 60), a + 25);
        assert_eq!(memchr(&mut e, a, 0x41 + 25, 25), 0);
        assert_eq!(memchr(&mut e, a, 0x41 + 7, 60), a + 7);
        assert_eq!(memchr(&mut e, a, 0x20, 60), 0);
        assert_eq!(memchr(&mut e, a, 0x41, 0), 0);
        // A match in the last byte of the last whole dword and in the tail.
        for target in [a + 54, a + 58, a + 59] {
            e.mem.set_u8(target, 0x80);
            assert_eq!(memchr(&mut e, a, 0x80, 60), target);
            e.mem.set_u8(target, 0x41);
        }
    }

    #[test]
    fn fastzero_and_vec_memzero_clear_whole_blocks_and_the_rest() {
        let (mut e, at) = engine();
        vec_memzero(&mut e, at + 0x13, 0, 0x1f0);
        let bytes = e.mem.bytes(at, 0x400);
        for (i, b) in bytes.iter().enumerate() {
            let inside = (0x13..0x13 + 0x1f0).contains(&i);
            assert_eq!(*b == 0, inside || pattern(i as u32) == 0, "{i:x}");
        }
        assert_eq!(calls(&e, FASTZERO_I).len(), 1);
    }
}
