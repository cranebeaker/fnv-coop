//! `Havok SDK/run_00df06a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unnamed routines `hkSolver_solvePoweredChain` (`hkpsolver.rs`) calls, and the function
//! that serialises a chain block (`00df21c0`). The context block they take is twelve words:
//! `[0]` the solver info, `[1]` the link count `n` (signed), `[2]` and `[3]` two floats, `[4]`
//! the link records (0x90 bytes each: nine vectors), `[5]` the second records (0x60 bytes),
//! `[6]` the table of body offsets (`n + 1` words), `[7]` the bodies, `[8]` the matrix blocks
//! (0x3c0 bytes each), `[9]` the flag records (0x40 bytes each; the first byte holds three
//! two-bit modes), `[10]` the vectors and `[11]` the rows (0x20 bytes per link).
//!
//! The SSE code is written instruction by instruction on [`Xmm`] registers (four `f32` lanes,
//! moves keep the bits) and the x87 code on the [`X87`] stack (`f64` values, rounded to `float`
//! at each store, the x87 NaN rules of the prelude). Helpers shared with the other units of this
//! batch are `pub(crate)`.

#[allow(unused_imports)]
use crate::prelude::*;

/// An SSE register: four `f32` lanes.
pub(crate) type Xmm = [f32; 4];

/// `MOVAPS xmm, [at]`.
pub(crate) fn movaps(e: &Engine, at: u32) -> Xmm {
    std::array::from_fn(|lane| e.mem.f32(at.wrapping_add(4 * lane as u32)))
}

/// `MOVAPS [at], xmm`.
pub(crate) fn store_xmm(e: &mut Engine, at: u32, v: Xmm) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(at.wrapping_add(4 * lane as u32), *value);
    }
}

/// `MOVSS xmm, [at]`: lane 0 loaded, the other lanes zero.
pub(crate) fn movss_mem(e: &Engine, at: u32) -> Xmm {
    [e.mem.f32(at), 0.0, 0.0, 0.0]
}

/// `MOVSS xmm, [stack word]` for a float held as bits.
pub(crate) fn movss_bits(bits: u32) -> Xmm {
    [f32::from_bits(bits), 0.0, 0.0, 0.0]
}

/// `MULPS dest, src` (the SSE NaN rules, [`crate::sse::mulps`]).
pub(crate) fn mulps(a: Xmm, b: Xmm) -> Xmm {
    crate::sse::mulps(a, b)
}

/// `ADDPS dest, src` (the SSE NaN rules, [`crate::sse::addps`]).
pub(crate) fn addps(a: Xmm, b: Xmm) -> Xmm {
    crate::sse::addps(a, b)
}

/// `SUBPS dest, src` (the SSE NaN rules, [`crate::sse::subps`]).
pub(crate) fn subps(a: Xmm, b: Xmm) -> Xmm {
    crate::sse::subps(a, b)
}

/// `ADDSS dest, src`: lane 0 summed, the other lanes of `dest` (the SSE NaN rules,
/// [`crate::sse::addss`]).
pub(crate) fn addss(dest: Xmm, src: Xmm) -> Xmm {
    crate::sse::addss(dest, src)
}

/// `SHUFPS dest, src, imm`: two lanes of `dest` then two of `src`.
pub(crate) fn shufps(dest: Xmm, src: Xmm, imm: u8) -> Xmm {
    [
        dest[usize::from(imm & 3)],
        dest[usize::from((imm >> 2) & 3)],
        src[usize::from((imm >> 4) & 3)],
        src[usize::from((imm >> 6) & 3)],
    ]
}

/// `SHUFPS v, v, 0`: lane 0 in all four.
pub(crate) fn splat0(v: Xmm) -> Xmm {
    shufps(v, v, 0)
}

/// `MAXPS dest, src`: per lane `dest > src ? dest : src` (a NaN in either gives `src`;
/// [`crate::sse::maxps`]).
pub(crate) fn maxps(dest: Xmm, src: Xmm) -> Xmm {
    crate::sse::maxps(dest, src)
}

/// `RCPPS`: the per-lane reciprocal estimate of the host CPU (x86).
pub(crate) fn rcpps(v: Xmm) -> Xmm {
    v.map(super::hkaabbtree::rcp_estimate)
}

/// `UNPCKLPS dest, src`: `dest0, src0, dest1, src1`.
pub(crate) fn unpcklps(dest: Xmm, src: Xmm) -> Xmm {
    [dest[0], src[0], dest[1], src[1]]
}

/// `UNPCKHPS dest, src`: `dest2, src2, dest3, src3`.
pub(crate) fn unpckhps(dest: Xmm, src: Xmm) -> Xmm {
    [dest[2], src[2], dest[3], src[3]]
}

/// `MOVLHPS dest, src`: the low half of `dest`, then the low half of `src`.
pub(crate) fn movlhps(dest: Xmm, src: Xmm) -> Xmm {
    [dest[0], dest[1], src[0], src[1]]
}

/// `MOVHLPS dest, src`: the high half of `src`, then the high half of `dest`.
pub(crate) fn movhlps(dest: Xmm, src: Xmm) -> Xmm {
    [src[2], src[3], dest[2], dest[3]]
}

/// The lane sum the code builds with two shuffles and two `ADDSS`: `XMM1 = shuf(v, 0x55) +
/// v` (lane 0: `v1 + v0`), `XMM2 = shuf(v, 0xaa) + XMM1`: `v2 + (v1 + v0)`.
pub(crate) fn sum3(v: Xmm) -> f32 {
    let one = addss(shufps(v, v, 0x55), v);
    addss(shufps(v, v, 0xaa), one)[0]
}

/// The x87 register stack (`ST(0)` is the last element). Values are `f64`; a stored float goes
/// through [`x87_store32`], a loaded one through [`x87_load32`].
pub(crate) struct X87 {
    stack: Vec<f64>,
}

impl X87 {
    pub(crate) fn new() -> Self {
        X87 { stack: Vec::new() }
    }

    /// `ST(i)`.
    fn st(&self, i: usize) -> f64 {
        self.stack[self.stack.len() - 1 - i]
    }

    fn set(&mut self, i: usize, value: f64) {
        let at = self.stack.len() - 1 - i;
        self.stack[at] = value;
    }

    /// Pushes a value (a float answered in `ST0` by a call, for instance).
    pub(crate) fn push(&mut self, value: f64) {
        assert!(self.stack.len() < 8, "x87 stack overflow");
        self.stack.push(value);
    }

    fn pop(&mut self) -> f64 {
        self.stack.pop().expect("x87 stack underflow")
    }

    /// `FLD dword`.
    pub(crate) fn fld32(&mut self, bits: u32) {
        self.push(x87_load32(bits));
    }

    /// `FLD qword`.
    pub(crate) fn fld64(&mut self, value: f64) {
        self.push(value);
    }

    /// `FLD ST(i)`.
    pub(crate) fn fld_st(&mut self, i: usize) {
        let value = self.st(i);
        self.push(value);
    }

    /// `FILD dword`.
    pub(crate) fn fild(&mut self, value: i32) {
        self.push(f64::from(value));
    }

    /// `FXCH ST(i)`.
    pub(crate) fn fxch(&mut self, i: usize) {
        let last = self.stack.len() - 1;
        self.stack.swap(last, last - i);
    }

    /// `FSTP dword`: pops, answers the float's bits.
    pub(crate) fn fstp32(&mut self) -> u32 {
        x87_store32(self.pop())
    }

    /// `FST dword`: the float's bits, nothing popped.
    pub(crate) fn fst32(&self) -> u32 {
        x87_store32(self.st(0))
    }

    /// `FSTP ST0`: drops the top.
    pub(crate) fn fpop(&mut self) {
        self.pop();
    }

    /// `FSTP ST(i)`: `ST(i) = ST(0)`, then pops.
    pub(crate) fn fstp_st(&mut self, i: usize) {
        let value = self.st(0);
        self.set(i, value);
        self.pop();
    }

    /// `FADD dword`: `ST0 += float`.
    pub(crate) fn fadd_m32(&mut self, bits: u32) {
        let value = x87_add(self.st(0), x87_m32(bits));
        self.set(0, value);
    }

    /// `FSUB dword`: `ST0 -= float`.
    pub(crate) fn fsub_m32(&mut self, bits: u32) {
        let value = x87_sub(self.st(0), x87_m32(bits));
        self.set(0, value);
    }

    /// `FMUL dword`: `ST0 *= float`.
    pub(crate) fn fmul_m32(&mut self, bits: u32) {
        let value = x87_mul(self.st(0), x87_m32(bits));
        self.set(0, value);
    }

    /// `FDIV ST0, ST(i)`.
    pub(crate) fn fdiv_st0(&mut self, i: usize) {
        let value = x87_div(self.st(0), self.st(i));
        self.set(0, value);
    }

    /// `FADD ST(i), ST0`.
    pub(crate) fn fadd_to(&mut self, i: usize) {
        let value = x87_add(self.st(i), self.st(0));
        self.set(i, value);
    }

    /// `FMULP ST(i), ST0`: `ST(i) *= ST0`, then pops.
    pub(crate) fn fmulp(&mut self, i: usize) {
        let value = x87_mul(self.st(i), self.st(0));
        self.set(i, value);
        self.pop();
    }

    /// `FSUBRP ST(i), ST0`: `ST(i) = ST0 - ST(i)`, then pops.
    pub(crate) fn fsubrp(&mut self, i: usize) {
        let value = x87_sub(self.st(0), self.st(i));
        self.set(i, value);
        self.pop();
    }

    /// `FCOM ST(i)`: the high byte of the status word (`AH` after `FNSTSW AX`), nothing popped.
    pub(crate) fn fcom(&self, i: usize) -> u8 {
        compare_ah(self.st(0), self.st(i))
    }

    /// `FCOMP ST(i)`: the status byte, then pops.
    pub(crate) fn fcomp(&mut self, i: usize) -> u8 {
        let status = compare_ah(self.st(0), self.st(i));
        self.pop();
        status
    }

    /// `FCOMPP`: compares `ST0` with `ST1`, pops both.
    pub(crate) fn fcompp(&mut self) -> u8 {
        let status = compare_ah(self.st(0), self.st(1));
        self.pop();
        self.pop();
        status
    }
}

/// `FCOM`'s condition bits in `AH` for `left` against `right`: `0x00` when greater, `0x01` when
/// less, `0x40` when equal, `0x45` when unordered.
pub(crate) fn compare_ah(left: f64, right: f64) -> u8 {
    if left.is_nan() || right.is_nan() {
        0x45
    } else if left > right {
        0x00
    } else if left < right {
        0x01
    } else {
        0x40
    }
}

/// `TEST AH, mask` then `JP`: whether the parity flag is set, an even number of the masked
/// bits being 1.
pub(crate) fn parity_even(ah: u8, mask: u8) -> bool {
    (ah & mask).count_ones() % 2 == 0
}

/// Unnamed (cdecl, `(block, out)`): the first of the helpers `00df0870` runs on a matrix block.
const BLOCK_PREPARE: u32 = 0x00e2_9250;
/// Unnamed (cdecl, `(out, block, cursor)`): builds the vectors of one block from the cursor.
const BLOCK_BUILD: u32 = 0x00e2_9470;
/// Unnamed (cdecl, `(block + 0xc0, vectors)`): finishes the block's rows from the built vectors.
const BLOCK_FINISH: u32 = 0x00e2_9380;
/// Unnamed (cdecl, `(block + 0xc0, copy of the vectors)`).
const BLOCK_REFINE: u32 = 0x00e2_98f0;
/// The float at `0104da4c`: the smallest value `00df0870` leaves in six block entries.
const MINIMUM: u32 = 0x0104_da4c;
/// The `double` at `010c9300` (1.1920929e-07) `00df21c0` adds to the first float of the output.
const OUTPUT_BIAS: u32 = 0x010c_9300;
/// The routine that fills the jacobian rows of a chain (`00df19e0`, nine words).
const BUILD_CHAIN_ROWS: u32 = 0x00df_19e0;

// Translated from 00df06a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`, `hkpsolver.rs`).
/// Cdecl, four words: `flags` (a record whose first byte holds three two-bit modes), `second`
/// (only its low byte is read), `block_a` and `block_b` (two 0xc0-byte groups of rows).
///
/// Nothing happens when the flag byte is 0. For each axis `k` (0, 1, 2) whose mode bits are not
/// zero (`3 << 2k` of the byte: the first test uses the byte read at the start, the others read
/// it again) the six floats at `block_a + 0x60 + 4k + 0x10 * j` (`j` = 0..5) are set to 0.0 and
/// the one at `j = 3 + k` to 1.0; when the low byte of `second` is not zero the six floats of
/// `block_b` are zeroed as well (no 1.0 there).
pub fn fn_00df06a0(e: &mut Engine, flags: Ptr, second: u32, block_a: Ptr, block_b: Ptr) {
    let first = e.mem.u8(flags.addr());
    if first == 0 {
        return;
    }
    let both = second as u8 != 0;
    for axis in 0..3u32 {
        let byte = if axis == 0 {
            first
        } else {
            e.mem.u8(flags.addr())
        };
        if byte & (3 << (2 * axis)) == 0 {
            continue;
        }
        let column = 0x60 + 4 * axis;
        for j in 0..6u32 {
            e.mem
                .set_f32(block_a.addr().wrapping_add(column + 0x10 * j), 0.0);
        }
        e.mem
            .set_f32(block_a.addr().wrapping_add(column + 0x10 * (3 + axis)), 1.0);
        if both {
            for j in 0..6u32 {
                e.mem
                    .set_f32(block_b.addr().wrapping_add(column + 0x10 * j), 0.0);
            }
        }
    }
}

// Translated from 00df07c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`). Cdecl, two words:
/// `flags` (a record whose first byte holds three two-bit modes) and `block`. Nothing happens
/// when the flag byte is 0. For each axis `k` (0, 1, 2) whose mode bits (the byte shifted right
/// by `2k`, masked with 3) are not zero, the 16-byte vectors at `block + 0x30 + 0x10k` and
/// `block + 0x90 + 0x10k` are zeroed.
pub fn fn_00df07c0(e: &mut Engine, flags: Ptr, block: Ptr) {
    if e.mem.u8(flags.addr()) == 0 {
        return;
    }
    let mut shift = 0u32;
    let mut cursor = block.addr().wrapping_add(0x90);
    while shift < 6 {
        let byte = e.mem.u8(flags.addr());
        if (byte >> shift) & 3 != 0 {
            store_xmm(e, cursor.wrapping_sub(0x60), [0.0; 4]);
            store_xmm(e, cursor, [0.0; 4]);
        }
        shift += 2;
        cursor = cursor.wrapping_add(0x10);
    }
}

// Translated from 00df0810 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`). Cdecl, four words:
/// copies one float of a flag record into the rows. With `record = flags_base + (first << 6)`
/// and `mode = (byte at record >> (2 * second)) & 3` (an 8-bit shift: a count of 8 or more
/// gives 0), the float at `record + 20 * second + 4` (`mode == 1`) or `+ 8` (otherwise) goes
/// through the x87 stack (a signalling NaN comes back quieted) to `rows + (first << 5) + 4 *
/// second + 0x10`.
pub fn fn_00df0810(e: &mut Engine, first: u32, second: u32, flags_base: Ptr, rows: Ptr) {
    let record = flags_base.addr().wrapping_add(first << 6);
    let row = rows.addr().wrapping_add(first << 5);
    let count = (second as u8).wrapping_add(second as u8) & 0x1f;
    let byte = e.mem.u8(record);
    let mode = if count >= 8 { 0 } else { (byte >> count) & 3 };
    let offset = if mode == 1 { 4 } else { 8 };
    let at = record
        .wrapping_add(second.wrapping_mul(5).wrapping_mul(4))
        .wrapping_add(offset);
    let value = x87_quiet_bits(e.mem.u32(at));
    e.mem.set_u32(
        row.wrapping_add(second.wrapping_mul(4)).wrapping_add(0x10),
        value,
    );
}

// Translated from 00df0870 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`: one step of the chain
/// per matrix block). Cdecl, five words: `flag` (low byte used by `00df06a0`), `block` (a 0x3c0
/// byte matrix block), `flagrec` (the flag record), `out` (twelve vectors) and `cursor` (a pointer
/// to a pointer).
///
/// Copies the 24 vectors at `block + 0x240` into a local array `copy`, then runs `00df06a0`
/// (`(flagrec, flag, copy, copy + 0xc0)`), `00df07c0` (`(flagrec, out)`), `00e29250` (`(block,
/// out)`) and `00e29470` (`(tmp, block, *cursor)`, `tmp` a local block), copies the first twelve
/// vectors of `copy` to `block + 0xc0` and runs `00e29380` (`(block + 0xc0, tmp)`). Then the six
/// floats at `block + 0xc0`, `0x150`, `0xd4`, `0x164`, `0xe8` and `0x178` are each set to
/// themselves when above the float at `0104da4c` (the x87 `FCOM` test: a NaN is not above), else
/// to that float. The twelve vectors at `block + 0xc0` are copied to a second local array,
/// `00e298f0` runs on `(block + 0xc0, that copy)` and `00e29470` on `(block + 0x180, block +
/// 0xc0, copy + 0xc0)`; the twelve vectors at `block + 0x300` go to `out` and `*cursor` is set to
/// `block + 0x180`.
pub fn fn_00df0870(e: &mut Engine, flag: u32, block: Ptr, flagrec: Ptr, out: Ptr, cursor: Ptr) {
    e.with_stack(0x310, |e, frame| {
        let base = frame.addr();
        let copy = base.wrapping_add(0x10);
        let tmp = base.wrapping_add(0x250);
        let second_copy = base.wrapping_add(0x190);
        let me = block.addr();
        for i in 0..24u32 {
            let vector = movaps(e, me.wrapping_add(0x240 + 0x10 * i));
            store_xmm(e, copy.wrapping_add(0x10 * i), vector);
        }
        e.call(
            0x00df_06a0,
            &args![flagrec, flag, copy, copy.wrapping_add(0xc0)],
        );
        e.call(0x00df_07c0, &args![flagrec, out]);
        e.call(BLOCK_PREPARE, &args![block, out]);
        let held = e.mem.u32(cursor.addr());
        e.call(BLOCK_BUILD, &args![tmp, block, held]);
        let rows = me.wrapping_add(0xc0);
        for i in 0..12u32 {
            let vector = movaps(e, copy.wrapping_add(0x10 * i));
            store_xmm(e, rows.wrapping_add(0x10 * i), vector);
        }
        e.call(BLOCK_FINISH, &args![rows, tmp]);
        let minimum = x87_quiet_bits(e.mem.u32(MINIMUM));
        for offset in [0xc0u32, 0x150, 0xd4, 0x164, 0xe8, 0x178] {
            let at = me.wrapping_add(offset);
            let value = x87_quiet_bits(e.mem.u32(at));
            let above = compare_ah(x87_load32(value), x87_load32(minimum)) & 0x41 == 0;
            e.mem.set_u32(at, if above { value } else { minimum });
        }
        for i in 0..12u32 {
            let vector = movaps(e, rows.wrapping_add(0x10 * i));
            store_xmm(e, second_copy.wrapping_add(0x10 * i), vector);
        }
        e.call(BLOCK_REFINE, &args![rows, second_copy]);
        let next = me.wrapping_add(0x180);
        e.call(BLOCK_BUILD, &args![next, rows, copy.wrapping_add(0xc0)]);
        for i in 0..12u32 {
            let vector = movaps(e, me.wrapping_add(0x300 + 0x10 * i));
            store_xmm(e, out.addr().wrapping_add(0x10 * i), vector);
        }
        e.mem.set_u32(cursor.addr(), next);
    });
}

// Translated from 00df2040 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`: runs `00df0870` over the
/// matrix blocks from the one before `found`). Cdecl, four words: `found`, `count` (both
/// signed), `flags` (0x40-byte records) and `matrices` (0x3c0-byte blocks).
///
/// With `k = max(found - 1, 0)` (signed): a local array of twelve vectors holds the twelve
/// vectors at `matrices + k * 0x3c0 - 0xc0` (the output of the previous block), or zeros when
/// `k` is 0, and a pointer variable the address `matrices + k * 0x3c0 - 0x240` (the previous
/// block's `+0x180`), or the local array when `k` is 0. Then for each block from `k` up to
/// `count` it runs `00df0870` with the flag `remaining - 1 != 0` (where `remaining` counts
/// the blocks left including this one; the flag word's upper bytes are whatever the stack held,
/// here zero), the block, the flag record `flags + (index << 6)`, the local array and the
/// pointer variable's address.
pub fn fn_00df2040(e: &mut Engine, found: i32, count: i32, flags: Ptr, matrices: Ptr) {
    e.with_stack(0xf0, |e, frame| {
        let base = frame.addr();
        let locals = base.wrapping_add(0x20);
        let pointer_slot = base.wrapping_add(0x18);
        let flag_slot = base.wrapping_add(0x1c);
        let counter_slot = base.wrapping_add(0x14);
        let last = found.wrapping_sub(1);
        let k = if last < 0 { 0 } else { last };
        let cursor = if k != 0 {
            let at = (k as u32).wrapping_mul(0x3c0).wrapping_add(matrices.addr());
            for i in 0..12u32 {
                let vector = movaps(e, at.wrapping_sub(0xc0).wrapping_add(0x10 * i));
                store_xmm(e, locals.wrapping_add(0x10 * i), vector);
            }
            at.wrapping_add(0xffff_fdc0)
        } else {
            for i in 0..12u32 {
                store_xmm(e, locals.wrapping_add(0x10 * i), [0.0; 4]);
            }
            locals
        };
        e.mem.set_u32(pointer_slot, cursor);
        if k >= count {
            return;
        }
        let mut block = (k as u32).wrapping_mul(0x3c0).wrapping_add(matrices.addr());
        let mut record = ((k as u32) << 6).wrapping_add(flags.addr());
        let remaining = count.wrapping_sub(k);
        let mut after = remaining.wrapping_sub(1);
        e.mem.set_u32(counter_slot, remaining as u32);
        loop {
            e.mem.set_u8(flag_slot, u8::from(after != 0));
            let flag = e.mem.u32(flag_slot);
            e.call(
                0x00df_0870,
                &args![flag, block, record, locals, pointer_slot],
            );
            after = after.wrapping_sub(1);
            record = record.wrapping_add(0x40);
            block = block.wrapping_add(0x3c0);
            let left = e.mem.u32(counter_slot).wrapping_sub(1);
            e.mem.set_u32(counter_slot, left);
            if left == 0 {
                break;
            }
        }
    });
}

// Translated from 00df21c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (it writes a chain block into the output buffer of
/// `hkSolver_solvePoweredChain`). Cdecl, three words, the second unread: `source` (a record: `n`
/// at `+0`, floats at `+4`, `+8`, `+0x28`, words at `+0x1c` (an array of `n + 1` dwords), `+0x20`,
/// `+0x24` (`n` records of 0x40 bytes) and `+0x2c`) and `cursor` (a pointer to the output
/// pointer `out`).
///
/// Writes the header: `out[0]` (a float) plus the `double` at `010c9300`, then its low byte set to
/// 0x1f, the word `n` (16 bit) at `+4`, the word at `source + 0x2c` at `+8` (written twice), the
/// floats `source + 4` at `+0x18`, `source + 8` at `+0x1c` and `source + 0x28` at `+0x14`, and
/// `(n16 * 0x534 + 0x53) & !0xf` at `+0x10`. Copies the `n + 1` dwords of `+0x1c` to `out + 0x40 +
/// n16 * 0x4f0` and the `n` records of `+0x24` to `out + 0x44 + n16 * 0x4f4`, then runs `00df19e0`
/// (nine words: `n`, `source + 0xc`, the word at `+0x24`, `out + 0x20`, `out + n * 0x90 + 0x20`,
/// `out + 0x40 + n16 * 0x4f0`, the word at `+0x20`, `out + n16 * 0xf0 + 0x20`, `n * 0x3c0`) and
/// advances `*cursor` by the word at `out + 0x10`.
pub fn fn_00df21c0(e: &mut Engine, source: Ptr, _unused_2: u32, cursor: Ptr) {
    let src = source.addr();
    let count = e.mem.u32(src);
    let mut records = e.mem.u32(src.wrapping_add(0x24));
    let first_float = x87_quiet_bits(e.mem.u32(src.wrapping_add(4)));
    let second_float = x87_quiet_bits(e.mem.u32(src.wrapping_add(8)));
    let mut array = e.mem.u32(src.wrapping_add(0x1c));
    let tail = e.mem.u32(src.wrapping_add(0x2c));
    let out = e.mem.u32(cursor.addr());
    let bias = x87_load32(e.mem.u32(out));
    e.mem.set_u32(out.wrapping_add(8), tail);
    let biased = x87_add(bias, e.mem.f64(OUTPUT_BIAS));
    let count16 = count & 0xffff;
    e.mem.set_u32(out, x87_store32(biased));
    e.mem.set_u32(out.wrapping_add(0x18), first_float);
    e.mem.set_u32(out.wrapping_add(0x1c), second_float);
    let size = (count16.wrapping_mul(0x534).wrapping_add(0x53)) & 0xffff_fff0;
    e.mem.set_u32(out.wrapping_add(0x10), size);
    let plus_one = count.wrapping_add(1);
    e.mem.set_u8(out, 0x1f);
    e.mem.set_u16(out.wrapping_add(4), count as u16);
    let mut to = count16
        .wrapping_mul(0x4f0)
        .wrapping_add(out)
        .wrapping_add(0x40);
    if (plus_one as i32) > 0 {
        let mut left = plus_one;
        loop {
            let word = e.mem.u32(array);
            e.mem.set_u32(to, word);
            to = to.wrapping_add(4);
            array = array.wrapping_add(4);
            left = left.wrapping_sub(1);
            if left == 0 {
                break;
            }
        }
    }
    let count16 = u32::from(e.mem.u16(out.wrapping_add(4)));
    let mut to = count16
        .wrapping_mul(0x4f4)
        .wrapping_add(out)
        .wrapping_add(0x44);
    if (count as i32) > 0 {
        let mut left = count;
        loop {
            let from = records;
            records = records.wrapping_add(0x40);
            for word in 0..16u32 {
                let value = e.mem.u32(from.wrapping_add(4 * word));
                e.mem.set_u32(to.wrapping_add(4 * word), value);
            }
            to = to.wrapping_add(0x40);
            left = left.wrapping_sub(1);
            if left == 0 {
                break;
            }
        }
    }
    let float = x87_quiet_bits(e.mem.u32(src.wrapping_add(0x28)));
    e.mem.set_u32(out.wrapping_add(0x14), float);
    let tail = e.mem.u32(src.wrapping_add(0x2c));
    e.mem.set_u32(out.wrapping_add(8), tail);
    let count16 = u32::from(e.mem.u16(out.wrapping_add(4)));
    let out = e.mem.u32(cursor.addr());
    let rows_a = out
        .wrapping_add(count16.wrapping_mul(0xf0))
        .wrapping_add(0x20);
    let rows_b = out
        .wrapping_add(count16.wrapping_mul(0x4f0))
        .wrapping_add(0x40);
    let words_b = out
        .wrapping_add(count.wrapping_mul(0x90))
        .wrapping_add(0x20);
    let chain = e.mem.u32(src.wrapping_add(0x20));
    let links = e.mem.u32(src.wrapping_add(0x24));
    e.call(
        BUILD_CHAIN_ROWS,
        &args![
            count,
            src.wrapping_add(0xc),
            links,
            out.wrapping_add(0x20),
            words_b,
            rows_b,
            chain,
            rows_a,
            count.wrapping_mul(0x3c0)
        ],
    );
    let size = e.mem.u32(out.wrapping_add(0x10));
    let advanced = e.mem.u32(cursor.addr()).wrapping_add(size);
    e.mem.set_u32(cursor.addr(), advanced);
}

/// Unnamed (cdecl, `(out, matrix, vectors)`): maps a pair of vectors through a matrix block
/// into the two vectors at `out`.
const MATRIX_APPLY: u32 = 0x00e2_9510;
/// The `double` at `010d5288` the search routines scale the limits with.
const LIMIT_SCALE: u32 = 0x010d_5288;

/// An 8-bit `SHL`: the count is masked to five bits, bits shifted past the byte are lost.
fn shl8(value: u8, count: u32) -> u8 {
    let count = count & 0x1f;
    if count >= 8 {
        0
    } else {
        value << count
    }
}

/// The row value both `00df1060` and the search `00df1430` compute for link `first`, axis
/// `second` of the chain context `ctx`, as the bits of the float they store (`FSTP`):
/// `m * f - s`, where `m` is the float at `flags + (first << 6) + 20 * second + 0x10`, `f` the
/// float at `+0x1c` of the second record `S = seconds + (3 * first + second) * 0x20`, `c` the
/// float at `+0x14` of the flag record entry and `s` the lane sum of `((ga * S0 + fb * S1) *
/// c) + (((S0 * a) + (S1 * b)) * u)`, with `ga = A20 - A50`, `fb = B20 - B50` (the bodies at
/// the offsets `table[first]` and `table[first + 1]` in the body block, vectors at `+0x20` and
/// `+0x50`), `S0` and `S1` the vectors at `S` and `S + 0x10`, `a` and `b` the vectors `A50` and
/// `B50` and `u` the float `solver_info[0x40] * m` (rounded to a float).
fn chain_row(e: &Engine, ctx: u32, first: u32, second: u32) -> u32 {
    let table = e.mem.u32(ctx.wrapping_add(0x18));
    let bodies = e.mem.u32(ctx.wrapping_add(0x1c));
    let entry = table.wrapping_add(first.wrapping_mul(4));
    let body_a = e.mem.u32(entry).wrapping_add(bodies);
    let body_b = e.mem.u32(entry.wrapping_add(4)).wrapping_add(bodies);
    let record = first.wrapping_mul(3).wrapping_add(second);
    let a50 = movaps(e, body_a.wrapping_add(0x50));
    let b50 = movaps(e, body_b.wrapping_add(0x50));
    let flags = (first << 6).wrapping_add(e.mem.u32(ctx.wrapping_add(0x24)));
    let entry_at = flags.wrapping_add(second.wrapping_mul(5).wrapping_mul(4));
    let m = x87_quiet_bits(e.mem.u32(entry_at.wrapping_add(0x10)));
    let c = movss_mem(e, entry_at.wrapping_add(0x14));
    let seconds = (record << 5).wrapping_add(e.mem.u32(ctx.wrapping_add(0x14)));
    let info = e.mem.u32(ctx);
    let u = x87_store32(x87_mul(
        x87_load32(e.mem.u32(info.wrapping_add(0x40))),
        x87_load32(m),
    ));
    let s1 = movaps(e, seconds.wrapping_add(0x10));
    let s0 = movaps(e, seconds);
    let tail = x87_mul(
        x87_load32(m),
        x87_m32(e.mem.u32(seconds.wrapping_add(0x1c))),
    );
    let ga = subps(movaps(e, body_a.wrapping_add(0x20)), a50);
    let fb = subps(movaps(e, body_b.wrapping_add(0x20)), b50);
    let part = mulps(ga, s0);
    let part = addps(part, mulps(fb, s1));
    let part = mulps(part, splat0(c));
    let rest = mulps(s0, a50);
    let rest = addps(rest, mulps(s1, b50));
    let rest = mulps(rest, splat0(movss_bits(u)));
    let sum = sum3(addps(part, rest));
    x87_store32(x87_sub(tail, x87_m32(sum.to_bits())))
}

// Translated from 00df0d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`: the two rows per axis
/// of every link of the chain). Cdecl, two words: `context` (the twelve-word block described at
/// the top of this file) and `rows`.
///
/// For each of the `n` links (the word at `+4`, signed; nothing for zero or less) with the body
/// pair `table[i]`, `table[i + 1]` of the body block (offsets in the word at `+0x18`, base the
/// word at `+0x1c`) it builds, with `ga = A20 - A50`, `fb = B20 - B50`, `d = (A10 - A40) - (B10 -
/// B40)` and `e = A40 - B40` (vectors of the two bodies at those offsets), and for each of the
/// three axes `j` the link vectors `L = links + 0x10 + 0x90 * i + 0x30 * j` (`L - 0x10`, `L`,
/// `L + 0x10`) and the second vectors `S = seconds + 0x10 + 0x60 * i + 0x20 * j` (`S - 0x10`,
/// `S`), the floats `f2` (`+8`), `f3` (`+0xc`) and `sv` (the float at `+0x40` of the word at
/// `+0`): `rows[4 * i ...]` at `rows + 0x20 * i + 4 * j` gets `f2 * L[-1] - s1` where `s1` is the
/// lane sum of `(ga * L + fb * L1 + d * L[-0x10]) * f3 + ((L1 * B50 + A50 * L + e * L[-0x10])
/// * (sv * f2))`, and `rows + 0x20 * i + 0x10 + 4 * j` gets `m * S[0xc] - s2` for the flag record
/// `flags + 0x40 * i` entry `j` (float `m` at `+0x10 + 0x14 * j`, `c` at `+0x14 + 0x14 * j`)
/// with `s2` the lane sum of `((ga * (S - 0x10) + fb * S) * c) + (((A50 * (S - 0x10) + B50 * S)
/// * (sv * m))`. The two words at `+0xc` and `+0x1c` of each rows record are set to 0.0. The
/// x87 products and differences are rounded to a float at each store (`FSTP`), the SSE sums
/// as `SSE` does.
pub fn fn_00df0d30(e: &mut Engine, context: Ptr, rows: Ptr) {
    let ctx = context.addr();
    let count = e.mem.i32(ctx.wrapping_add(4));
    if count <= 0 {
        return;
    }
    let mut link = e.mem.u32(ctx.wrapping_add(0x10)).wrapping_add(0x10);
    let mut second = e.mem.u32(ctx.wrapping_add(0x14)).wrapping_add(0x10);
    let mut flag_offset = 0u32;
    let mut row = rows.addr();
    let mut index = 0u32;
    loop {
        let table = e.mem.u32(ctx.wrapping_add(0x18));
        let bodies = e.mem.u32(ctx.wrapping_add(0x1c));
        let entry = table.wrapping_add(index.wrapping_mul(4));
        let body_a = e.mem.u32(entry).wrapping_add(bodies);
        let body_b = e.mem.u32(entry.wrapping_add(4)).wrapping_add(bodies);
        let a40 = movaps(e, body_a.wrapping_add(0x40));
        let a50 = movaps(e, body_a.wrapping_add(0x50));
        let a10 = movaps(e, body_a.wrapping_add(0x10));
        let a20 = movaps(e, body_a.wrapping_add(0x20));
        let b40 = movaps(e, body_b.wrapping_add(0x40));
        let b50 = movaps(e, body_b.wrapping_add(0x50));
        let b10 = movaps(e, body_b.wrapping_add(0x10));
        let b20 = movaps(e, body_b.wrapping_add(0x20));
        let from_b = subps(b10, b40);
        let d = subps(subps(a10, a40), from_b);
        let diff = subps(a40, b40);
        let fb = subps(b20, b50);
        let ga = subps(a20, a50);
        let flags = e.mem.u32(ctx.wrapping_add(0x24)).wrapping_add(flag_offset);
        let mut entry_at = flags.wrapping_add(0x14);
        for axis in 0..3u32 {
            let f3 = x87_quiet_bits(e.mem.u32(ctx.wrapping_add(0xc)));
            let f2 = x87_quiet_bits(e.mem.u32(ctx.wrapping_add(8)));
            let info = e.mem.u32(ctx);
            let lm1 = movaps(e, link.wrapping_sub(0x10));
            let l0 = movaps(e, link);
            let l1 = movaps(e, link.wrapping_add(0x10));
            let f3_splat = splat0(movss_bits(f3));
            let f2_value = x87_load32(f2);
            let scale = x87_store32(x87_mul(
                x87_load32(e.mem.u32(info.wrapping_add(0x40))),
                f2_value,
            ));
            let head = x87_mul(f2_value, x87_m32(e.mem.u32(link.wrapping_sub(4))));
            let part = mulps(ga, l0);
            let part = addps(part, mulps(fb, l1));
            let part = addps(part, mulps(d, lm1));
            let part = mulps(part, f3_splat);
            let rest = mulps(l1, b50);
            let rest = addps(rest, mulps(a50, l0));
            let rest = addps(rest, mulps(diff, lm1));
            let rest = mulps(rest, splat0(movss_bits(scale)));
            let sum = sum3(addps(part, rest));
            let value = x87_store32(x87_sub(head, x87_m32(sum.to_bits())));
            e.mem
                .set_u32(row.wrapping_add(4 * axis), x87_quiet_bits(value));

            let info = e.mem.u32(ctx);
            let m = x87_quiet_bits(e.mem.u32(entry_at.wrapping_sub(4)));
            let c = movss_mem(e, entry_at);
            let s_prev = movaps(e, second.wrapping_sub(0x10));
            let s_here = movaps(e, second);
            let m_value = x87_load32(m);
            let scale = x87_store32(x87_mul(
                x87_load32(e.mem.u32(info.wrapping_add(0x40))),
                m_value,
            ));
            let part = mulps(a50, s_prev);
            let part = addps(part, mulps(b50, s_here));
            let part = mulps(part, splat0(movss_bits(scale)));
            let other = mulps(ga, s_prev);
            let other = addps(other, mulps(fb, s_here));
            let other = mulps(other, splat0(c));
            let tail = x87_mul(m_value, x87_m32(e.mem.u32(second.wrapping_add(0xc))));
            let sum = sum3(addps(part, other));
            let value = x87_store32(x87_sub(tail, x87_m32(sum.to_bits())));
            e.mem
                .set_u32(row.wrapping_add(0x10 + 4 * axis), x87_quiet_bits(value));
            link = link.wrapping_add(0x30);
            second = second.wrapping_add(0x20);
            entry_at = entry_at.wrapping_add(0x14);
        }
        flag_offset = flag_offset.wrapping_add(0x40);
        e.mem.set_u32(row.wrapping_add(0xc), 0);
        e.mem.set_u32(row.wrapping_add(0x1c), 0);
        index = index.wrapping_add(1);
        row = row.wrapping_add(0x20);
        if (index as i32) >= e.mem.i32(ctx.wrapping_add(4)) {
            break;
        }
    }
}

// Translated from 00df1060 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`: one row value of the
/// chain). Cdecl, four words: `first` (the link), `second` (the axis), `context` and `rows`.
/// Stores the row value of `chain_row` (see there: `m * f - s` for link `first`, axis `second`)
/// at `rows + 0x20 * first + 4 * second + 0x10`, through the x87 stack (a signalling NaN comes
/// back quieted).
pub fn fn_00df1060(e: &mut Engine, first: u32, second: u32, context: Ptr, rows: Ptr) {
    let value = chain_row(e, context.addr(), first, second);
    let at = rows
        .addr()
        .wrapping_add(first << 5)
        .wrapping_add(second.wrapping_mul(4))
        .wrapping_add(0x10);
    e.mem.set_u32(at, x87_quiet_bits(value));
}

// Translated from 00df11a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`: the forward pass of the
/// chain). Cdecl, two words: `context` and `out` (`n + 1` pairs of vectors, 0x20 bytes each).
/// Zeroes the first pair; then for each of the `n` links `i` (the word at `+4`, signed; none for
/// zero or less), with `matrix = matrices + 0x3c0 * i` (the word at `+0x20`) and `rows = *(ctx
/// + 0x2c) + 0x20 * i`: `T = 00e29510(matrix, out_i)`, `T0 = rows0 - T0`, `T1 = rows1 - T1`, then
/// `out_{i+1} = 00e29510(matrix + 0xc0, T)`.
pub fn fn_00df11a0(e: &mut Engine, context: Ptr, out: Ptr) {
    e.with_stack(0x40, |e, frame| {
        let temp = frame.addr().wrapping_add(0x20);
        let ctx = context.addr();
        store_xmm(e, out.addr(), [0.0; 4]);
        store_xmm(e, out.addr().wrapping_add(0x10), [0.0; 4]);
        if e.mem.i32(ctx.wrapping_add(4)) <= 0 {
            return;
        }
        let mut matrix_offset = 0u32;
        let mut rows_offset = 0u32;
        let mut index = 0u32;
        let mut target = out.addr().wrapping_add(0x20);
        loop {
            let rows = e.mem.u32(ctx.wrapping_add(0x2c)).wrapping_add(rows_offset);
            let matrix = e
                .mem
                .u32(ctx.wrapping_add(0x20))
                .wrapping_add(matrix_offset);
            e.call(
                MATRIX_APPLY,
                &args![temp, matrix, target.wrapping_sub(0x20)],
            );
            let low = subps(movaps(e, rows), movaps(e, temp));
            let matrices = e.mem.u32(ctx.wrapping_add(0x20));
            let temp_high = movaps(e, temp.wrapping_add(0x10));
            store_xmm(e, temp, low);
            let high = subps(movaps(e, rows.wrapping_add(0x10)), temp_high);
            store_xmm(e, temp.wrapping_add(0x10), high);
            e.call(
                MATRIX_APPLY,
                &args![
                    target,
                    matrices.wrapping_add(matrix_offset).wrapping_add(0xc0),
                    temp
                ],
            );
            rows_offset = rows_offset.wrapping_add(0x20);
            index = index.wrapping_add(1);
            target = target.wrapping_add(0x20);
            matrix_offset = matrix_offset.wrapping_add(0x3c0);
            if (index as i32) >= e.mem.i32(ctx.wrapping_add(4)) {
                break;
            }
        }
    });
}

// Translated from 00df1260 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`: finds the chain axis
/// that breaks its limits the most). Cdecl, four words: `context`, `link_out`, `axis_out` and
/// `value_out`.
///
/// Walks the links `k = n - 1` down to 0 (`n` the signed word at `+4`; nothing for zero or less)
/// carrying a pair of vectors `V` (zero at first): `T = 00e29510(matrix_k + 0x180, V)` (the
/// block `matrices + 0x3c0 * k`, `V` held in the frame) and `V = rows_k - T` where `rows_k` is
/// the pair at `*(ctx + 0x28) + 0x20 * (k + 1)`. For each axis `a` whose two mode bits in the
/// flag byte at `flags + (k << 6)` are 0 (the byte is read again for each axis) it takes
/// `y = V1[a]` (quieted) and with `c` the double at `010d5288` the floats `lo = y - f1 * c` and
/// `hi = f2 * c - y` (`f1`, `f2` the floats at `+4` and `+8` of the entry at `flags + (k << 6) +
/// 0x14 * a`, each rounded to a float), keeps `m = hi` unless `hi < lo` (then `lo`: an `FCOM`
/// with the `JP` test of its `C0` and `C2` bits, so a NaN keeps `hi`), and when `m` is above the
/// best value so far (0.0 at first; the same test, a NaN does not win) it stores `y` at
/// `value_out` and keeps `(k, a)`.
///
/// When some pair won, the entry's float `f1` (at `+4` of the entry of the winning flag record)
/// is compared with the stored `y` (`FCOMPP`: 1 when `f1 < y`, else 3) and that value replaces
/// the winner's two mode bits; `*link_out` and `*axis_out` get the pair.
pub fn fn_00df1260(e: &mut Engine, context: Ptr, link_out: Ptr, axis_out: Ptr, value_out: Ptr) {
    e.with_stack(0x80, |e, frame| {
        let b = frame.addr();
        let at = |offset: u32| b.wrapping_add(offset);
        let ctx = context.addr();
        let mut best = 0u32;
        let mut best_link = u32::MAX;
        let mut best_axis = u32::MAX;
        let vectors = e.mem.u32(ctx.wrapping_add(0x28)).wrapping_add(0x20);
        let mut k = e.mem.u32(ctx.wrapping_add(4)).wrapping_sub(1) as i32;
        store_xmm(e, at(0x40), [0.0; 4]);
        store_xmm(e, at(0x50), [0.0; 4]);
        if k >= 0 {
            let mut block_offset = (k as u32).wrapping_mul(0x3c0);
            let mut row = ((k as u32) << 5).wrapping_add(vectors);
            loop {
                let matrices = e.mem.u32(ctx.wrapping_add(0x20));
                e.call(
                    MATRIX_APPLY,
                    &args![
                        at(0x60),
                        matrices.wrapping_add(block_offset).wrapping_add(0x180),
                        at(0x40)
                    ],
                );
                let low = subps(movaps(e, row), movaps(e, at(0x60)));
                store_xmm(e, at(0x40), low);
                let high = subps(movaps(e, row.wrapping_add(0x10)), movaps(e, at(0x70)));
                store_xmm(e, at(0x50), high);
                let flag_offset = (k as u32) << 6;
                for axis in 0..3u32 {
                    let flags = e.mem.u32(ctx.wrapping_add(0x24));
                    let byte = e.mem.u8(flag_offset.wrapping_add(flags));
                    let shift = (axis as u8).wrapping_add(axis as u8);
                    let mode = if shift >= 8 { 0 } else { (byte >> shift) & 3 };
                    if mode != 0 {
                        continue;
                    }
                    let y_bits = x87_quiet_bits(e.mem.u32(at(0x50 + 4 * axis)));
                    let y = x87_load32(y_bits);
                    let entry = e
                        .mem
                        .u32(ctx.wrapping_add(0x24))
                        .wrapping_add(0x14 * axis)
                        .wrapping_add(flag_offset);
                    let scale = e.mem.f64(LIMIT_SCALE);
                    let f1 = x87_load32(e.mem.u32(entry.wrapping_add(4)));
                    let lo = x87_store32(x87_sub(y, x87_mul(f1, scale)));
                    let f2 = x87_load32(e.mem.u32(entry.wrapping_add(8)));
                    let hi = x87_store32(x87_sub(x87_mul(f2, scale), y));
                    let status = compare_ah(x87_load32(hi), x87_load32(lo));
                    let kept = if parity_even(status, 5) { hi } else { lo };
                    let candidate = x87_load32(x87_quiet_bits(kept));
                    let status = compare_ah(x87_load32(best), candidate);
                    if !parity_even(status, 5) {
                        best = x87_store32(candidate);
                        best_link = k as u32;
                        e.mem.set_u32(value_out.addr(), x87_store32(y));
                        best_axis = axis;
                    }
                }
                block_offset = block_offset.wrapping_sub(0x3c0);
                row = row.wrapping_sub(0x20);
                k -= 1;
                if k < 0 {
                    break;
                }
            }
            if (best_link as i32) >= 0 {
                let flags = e.mem.u32(ctx.wrapping_add(0x24));
                let value = x87_load32(e.mem.u32(value_out.addr()));
                let record = (best_link << 6).wrapping_add(flags);
                let entry = record.wrapping_add(best_axis.wrapping_mul(5).wrapping_mul(4));
                let flag = x87_load32(e.mem.u32(entry.wrapping_add(4)));
                let status = compare_ah(flag, value);
                let new_mode: u8 = if parity_even(status, 5) { 3 } else { 1 };
                let shift = best_axis.wrapping_add(best_axis);
                let mask = shl8(3, shift);
                let bits = shl8(new_mode & 3, shift);
                let byte = e.mem.u8(record);
                e.mem.set_u8(record, (!mask & byte) | bits);
                e.mem.set_u32(link_out.addr(), best_link);
                e.mem.set_u32(axis_out.addr(), best_axis);
            }
        }
    });
}

// Translated from 00df1430 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a helper of `hkSolver_solvePoweredChain`: picks the chain axis
/// with the largest remaining error and clears its mode bits). Cdecl, three words: `context`,
/// `link_out`, `axis_out`.
///
/// Like `00df1260` it walks the links from `n - 1` down to 0 carrying a vector pair `V` (zero
/// at first) in the frame: before the loop `T = 00e29510(matrix_{n-1} + 0x180, V)` (the pointer
/// computed as `matrices + n * 0x3c0 - 0x240`, so it also runs for a count of zero) and `V =
/// rows_n - T`; each link `k` first shifts the frame's three pair slots (`V` of the two
/// previous links, `P1` and `P2`), then, with `prev = matrices + (sign extended 16-bit `k - 1`)
/// * 0x3c0`, updates `V = rows_k - 00e29510(prev + 0x180, V)` (zero pair for `k == 0`), builds a 12-vector
/// block with `00e29250` from `prev + 0x300` (an identity-shaped block for `k == 0`: the vectors
/// at frame `+0x140`, `0x150`, `0x160`, `0x1d0`, `0x1e0`, `0x1f0` are unit vectors, the rest
/// zero) and sums `Q = 00e29510(block, V) + 00e29510(prev + 0x600, P1) + 00e29510(prev + 0x6c0,
/// P2)` (pairs). The flag byte at `flags + (k << 6)` is read once; for each axis `a` whose mode
/// bits are 1 or 3 it takes the chain row value `r` (`chain_row`, link `k`, axis `a`) and the float
/// `q = Q1[a]` (the second vector of the sum, quieted) and with mode 3 goes on when `r >= q`
/// (C0 clear), with mode 1 when `r <= q` (the `FCOM` with the `JP` test of `C0` and `C3`); it
/// then takes `d = |q - r|` and keeps `(k, a)` when `d` is above the best so far (0.0 at first;
/// a NaN never wins). When a pair won, its two mode bits are cleared and `*link_out` and
/// `*axis_out` get the pair.
pub fn fn_00df1430(e: &mut Engine, context: Ptr, link_out: Ptr, axis_out: Ptr) {
    e.with_stack(0x210, |e, frame| {
        let b = frame.addr();
        let at = |offset: u32| b.wrapping_add(offset);
        let ctx = context.addr();
        let mut best = 0u32;
        let mut best_link = u32::MAX;
        let mut best_axis = u32::MAX;
        let matrices = e.mem.u32(ctx.wrapping_add(0x20));
        let count = e.mem.u32(ctx.wrapping_add(4));
        let vectors = e.mem.u32(ctx.wrapping_add(0x28)).wrapping_add(0x20);
        for offset in [0x110u32, 0x120, 0x130, 0xf0, 0x100, 0x70, 0x80] {
            store_xmm(e, at(offset), [0.0; 4]);
        }
        e.call(
            MATRIX_APPLY,
            &args![
                at(0x90),
                count
                    .wrapping_mul(0x3c0)
                    .wrapping_add(matrices)
                    .wrapping_sub(0x240),
                at(0x70)
            ],
        );
        let count = e.mem.u32(ctx.wrapping_add(4));
        let t_low = movaps(e, at(0x90));
        let t_high = movaps(e, at(0xa0));
        let row = (count << 5).wrapping_add(vectors).wrapping_sub(0x20);
        let low = subps(movaps(e, row), t_low);
        store_xmm(e, at(0x70), low);
        let high = subps(movaps(e, row.wrapping_add(0x10)), t_high);
        store_xmm(e, at(0x80), high);
        let mut k = count.wrapping_sub(1) as i32;
        if k < 0 {
            return;
        }
        let mut row = ((k as u32) << 5).wrapping_add(vectors).wrapping_sub(0x20);
        let mut previous = k.wrapping_sub(1);
        loop {
            let old_low = movaps(e, at(0xf0));
            let old_high = movaps(e, at(0x100));
            let v_low = movaps(e, at(0x70));
            let v_high = movaps(e, at(0x80));
            let block = ((previous as i16 as i32) as u32)
                .wrapping_mul(0x3c0)
                .wrapping_add(e.mem.u32(ctx.wrapping_add(0x20)));
            store_xmm(e, at(0x120), old_low);
            store_xmm(e, at(0x130), old_high);
            store_xmm(e, at(0xf0), v_low);
            store_xmm(e, at(0x100), v_high);
            let zero = movaps(e, at(0x110));
            if previous >= 0 {
                e.call(
                    MATRIX_APPLY,
                    &args![at(0x90), block.wrapping_add(0x180), at(0x70)],
                );
                let t_low = movaps(e, at(0x90));
                let low = subps(movaps(e, row), t_low);
                store_xmm(e, at(0x70), low);
                let t_high = movaps(e, at(0xa0));
                let high = subps(movaps(e, row.wrapping_add(0x10)), t_high);
                store_xmm(e, at(0x80), high);
            } else {
                store_xmm(e, at(0x70), zero);
                store_xmm(e, at(0x80), zero);
            }
            if previous >= 0 {
                e.call(BLOCK_PREPARE, &args![at(0x140), block.wrapping_add(0x300)]);
            } else {
                for (vector, unit) in [
                    (0x140u32, 0x140u32),
                    (0x150, 0x154),
                    (0x160, 0x168),
                    (0x1d0, 0x1d0),
                    (0x1e0, 0x1e4),
                    (0x1f0, 0x1f8),
                ] {
                    store_xmm(e, at(vector), zero);
                    e.mem.set_f32(at(unit), 1.0);
                }
                for vector in [0x170u32, 0x180, 0x190, 0x1a0, 0x1b0, 0x1c0] {
                    store_xmm(e, at(vector), zero);
                }
            }
            e.call(MATRIX_APPLY, &args![at(0xb0), at(0x140), at(0x70)]);
            e.call(
                MATRIX_APPLY,
                &args![at(0x90), block.wrapping_add(0x600), at(0xf0)],
            );
            let first = addps(movaps(e, at(0xb0)), movaps(e, at(0x90)));
            let t_high = movaps(e, at(0xa0));
            store_xmm(e, at(0xb0), first);
            let second = addps(movaps(e, at(0xc0)), t_high);
            store_xmm(e, at(0xc0), second);
            e.call(
                MATRIX_APPLY,
                &args![at(0x90), block.wrapping_add(0x6c0), at(0x120)],
            );
            let first = addps(movaps(e, at(0xb0)), movaps(e, at(0x90)));
            let t_high = movaps(e, at(0xa0));
            store_xmm(e, at(0xb0), first);
            let second = addps(movaps(e, at(0xc0)), t_high);
            store_xmm(e, at(0xc0), second);
            let record = e
                .mem
                .u32(ctx.wrapping_add(0x24))
                .wrapping_add((k as u32) << 6);
            let byte = u32::from(e.mem.u8(record));
            for axis in 0..3u32 {
                let shift = (axis as u8).wrapping_add(axis as u8);
                let mode = (byte >> shift) & 3;
                if mode != 3 && mode != 1 {
                    continue;
                }
                let r_bits = chain_row(e, ctx, k as u32, axis);
                let q_bits = x87_quiet_bits(e.mem.u32(at(0xc0 + 4 * axis)));
                let r = x87_load32(r_bits);
                let q = x87_load32(q_bits);
                let status = compare_ah(r, q);
                let go_on = if mode == 3 {
                    status & 1 == 0
                } else {
                    !parity_even(status, 0x41)
                };
                if !go_on {
                    continue;
                }
                let difference = x87_store32(x87_sub(q, r));
                let magnitude = x87_store32(x87_load32(difference).abs());
                let magnitude = x87_load32(magnitude);
                let status = compare_ah(x87_load32(best), magnitude);
                if !parity_even(status, 5) {
                    best = x87_store32(magnitude);
                    best_link = k as u32;
                    best_axis = axis;
                }
            }
            row = row.wrapping_sub(0x20);
            previous = previous.wrapping_sub(1);
            k -= 1;
            if k < 0 {
                break;
            }
        }
        if (best_link as i32) >= 0 {
            let record = (best_link << 6).wrapping_add(e.mem.u32(ctx.wrapping_add(0x24)));
            let shift = best_axis.wrapping_add(best_axis);
            let mask = shl8(3, shift);
            let byte = e.mem.u8(record);
            e.mem.set_u8(record, !mask & byte);
            e.mem.set_u32(link_out.addr(), best_link);
            e.mem.set_u32(axis_out.addr(), best_axis);
        }
    });
}

// Translated from 00df19e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (it sets up the matrix blocks of the chain solver
/// `hkSolver_solvePoweredChain`, then runs `00df0870` over them). Cdecl, nine words, the last
/// not read: `count` (`n`, signed), `floats` (four floats `f0..f3`), `flags` (0x40-byte flag
/// records), `links` (0x90 bytes per link), `seconds` (0x60 bytes per link), `table` (`n + 1`
/// body offsets), `bodies` and `matrices` (0x3c0-byte blocks).
///
/// Nothing happens for a count of zero or less. For each link `i`, with `A` and `B` the bodies
/// at the table offsets `table[i]` and `table[i + 1]` and `M = matrices + 0x3c0 * i + 0x350`:
///
/// 1. For each axis `j` (0..2) and each `m` from `j` to 2, with the link vectors `Lj0, Lj1, Lj2` =
///    `links + 0x10 + 0x90 * i + 0x30 * j + (-0x10, 0, 0x10)` and the second-record vectors `sj0`,
///    `sj1` at `seconds + 0x60 * i + 0x20 * j` and `+0x10` (the same for `m`), `A30` and `B30`
///    the vectors at `+0x30` of the two bodies: the lane sums `s1 = sum(splat(A30.w) * t +
///    splat(B30.w) * t + (Lm2 * Lj2) * B30 + (Lm1 * Lj1) * A30)` with `t = Lm0 * Lj0`, and `s2 =
///    sum((sm1 * sj1) * B30 + (sj0 * sm0) * A30)` go (through the x87 stack, quieted) to `E + 0x10
///    * m` and `E + 4 * m` (`s1`) and `E + 0x90 + 0x10 * m` and `E + 0x90 + 4 * m` (`s2`), with `E =
///    M - 0x110 + 0x14 * j`. Then `E[0] = f1 * E[0] + f0` and `E[0x90] = E[0x90] * f3 + f2` (each
///    product and sum rounded to a float, the memory operand a float of the x87 stack's kind).
/// 2. A 3 by 3 table at `M - 0xb0 + 4 * o + 0x10 * m` (`o`, `m` in 0..2): the lane sum of
///    `(Lm2 * so1) * B30 + (so0 * Lm1) * A30` with the link vectors at `links + 0x90 * i + 0x10 +
///    0x30 * m` and the second records at `seconds + 0x60 * i + 0x20 * o`.
/// 3. The table is moved to `M - 0xe0 ...` (ten of its floats go to `M - 0xe0`, `-0xcc`, `-0xb8`,
///    `-0xdc`, `-0xd0`, `-0xd8`, `-0xc0`, `-0xc8`, `-0xbc` and zeros to `-0xd4`, `-0xc4`, `-0xb4`).
/// 4. For a link before the last, a block of 4 x 9 lane sums at `M - 0x20 + 4 * o + 0x10 * m`
///    (stores at `-0x30`, `+0x60`, `0`, `+0x30` of that) from the next link's vectors, the next
///    second records and `B30`/`A30`; for the last link, the twelve vectors from `M - 0x50`
///    are zeroed.
///
/// Finally for each link (the flag `remaining - 1 != 0`, upper bytes as the stack left them) it
/// runs `00df0870` on the block, the flag record `flags + 0x40 * i`, a local zero-initialised
/// array of twelve vectors and a local pointer set to that array.
#[allow(clippy::too_many_arguments)]
pub fn fn_00df19e0(
    e: &mut Engine,
    count: i32,
    floats: Ptr,
    flags: Ptr,
    links: Ptr,
    seconds: Ptr,
    table: Ptr,
    bodies: Ptr,
    matrices: Ptr,
    _unused_9: u32,
) {
    e.with_stack(0x1a0, |e, frame| {
        let b = frame.addr();
        let at = |offset: u32| b.wrapping_add(offset);
        for offset in (0xd0u32..=0x190).step_by(0x10) {
            store_xmm(e, at(offset), [0.0; 4]);
        }
        e.mem.set_u32(at(0xb0), at(0xe0));
        if count <= 0 {
            return;
        }
        let f = floats.addr();
        let mut tptr = table.addr().wrapping_add(4);
        let mut sec = seconds.addr().wrapping_add(0x10);
        let mut lnk = links.addr().wrapping_add(0xa0);
        let mut m = matrices.addr().wrapping_add(0x350);
        let mut index = 0i32;
        loop {
            let mut edi = sec.wrapping_sub(0x10);
            let mut esi = lnk.wrapping_sub(0x90);
            let mut edx = m.wrapping_sub(0x110);
            let ebx = m;
            for j in 0..3u32 {
                let first = e.mem.u32(tptr.wrapping_sub(4));
                let second = e.mem.u32(tptr);
                let base = bodies.addr();
                let body_a = first.wrapping_add(base);
                let body_b = second.wrapping_add(base);
                let sj0 = movaps(e, edi);
                let lj0 = movaps(e, esi.wrapping_sub(0x10));
                let lj2 = movaps(e, esi.wrapping_add(0x10));
                let lj1 = movaps(e, esi);
                let sj1 = movaps(e, edi.wrapping_add(0x10));
                store_xmm(e, at(0xc0), sj0);
                let mut cur84 = edx;
                let mut cur6c = edx.wrapping_add(0x90);
                let mut esi_m = esi;
                let mut edi_m = edi;
                for _ in 0..(3 - j) {
                    let b30 = movaps(e, body_b.wrapping_add(0x30));
                    let a30 = movaps(e, body_a.wrapping_add(0x30));
                    let lm0 = movaps(e, esi_m.wrapping_sub(0x10));
                    let t = mulps(lm0, lj0);
                    let by_b = mulps(shufps(b30, b30, 0xff), t);
                    let by_a = mulps(shufps(a30, a30, 0xff), t);
                    let lm2 = movaps(e, esi_m.wrapping_add(0x10));
                    let cross2 = mulps(mulps(lm2, lj2), b30);
                    let lm1 = movaps(e, esi_m);
                    let cross1 = mulps(lm1, lj1);
                    let total = addps(by_a, by_b);
                    let cross1 = mulps(cross1, movaps(e, body_a.wrapping_add(0x30)));
                    let cross2 = addps(cross2, cross1);
                    let total = addps(total, cross2);
                    let s1 = sum3(total);
                    let sm1 = movaps(e, edi_m.wrapping_add(0x10));
                    e.mem.set_f32(at(0xa8), s1);
                    let quiet = x87_quiet_bits(s1.to_bits());
                    let sm0 = movaps(e, edi_m);
                    e.mem.set_u32(cur84, quiet);
                    e.mem.set_u32(cur6c.wrapping_sub(0x90), quiet);
                    let b30 = movaps(e, body_b.wrapping_add(0x30));
                    let part = mulps(mulps(sm1, sj1), b30);
                    let copy = movaps(e, at(0xc0));
                    let part2 = mulps(copy, sm0);
                    let a30 = movaps(e, body_a.wrapping_add(0x30));
                    let part2 = mulps(part2, a30);
                    let total = addps(part, part2);
                    let s2 = sum3(total);
                    e.mem.set_f32(at(0xb4), s2);
                    let quiet = x87_quiet_bits(s2.to_bits());
                    e.mem.set_u32(cur84.wrapping_add(0x90), quiet);
                    e.mem.set_u32(cur6c, quiet);
                    cur84 = cur84.wrapping_add(0x10);
                    cur6c = cur6c.wrapping_add(4);
                    esi_m = esi_m.wrapping_add(0x30);
                    edi_m = edi_m.wrapping_add(0x20);
                }
                // The diagonal: E[0] = f1 * E[0] + f0 and E[0x90] = E[0x90] * f3 + f2.
                let product = x87_mul(
                    x87_load32(e.mem.u32(f.wrapping_add(4))),
                    x87_m32(e.mem.u32(edx)),
                );
                e.mem.set_u32(edx, x87_store32(product));
                let high = x87_mul(
                    x87_load32(e.mem.u32(edx.wrapping_add(0x90))),
                    x87_m32(e.mem.u32(f.wrapping_add(0xc))),
                );
                e.mem.set_u32(edx.wrapping_add(0x90), x87_store32(high));
                let sum = x87_add(x87_load32(e.mem.u32(edx)), x87_m32(e.mem.u32(f)));
                e.mem.set_u32(edx, x87_store32(sum));
                let sum = x87_add(
                    x87_load32(e.mem.u32(edx.wrapping_add(0x90))),
                    x87_m32(e.mem.u32(f.wrapping_add(8))),
                );
                e.mem.set_u32(edx.wrapping_add(0x90), x87_store32(sum));
                edi = edi.wrapping_add(0x20);
                esi = esi.wrapping_add(0x30);
                edx = edx.wrapping_add(0x14);
            }
            // Second part: the 3 by 3 table at M - 0xb0.
            let first = e.mem.u32(tptr.wrapping_sub(4));
            let second = e.mem.u32(tptr);
            let base = bodies.addr();
            let body_b = second.wrapping_add(base);
            let a30_at = first.wrapping_add(base).wrapping_add(0x30);
            let link_base = lnk.wrapping_sub(0xa0).wrapping_add(0x10);
            let table_at = ebx.wrapping_sub(0x110).wrapping_add(0x60);
            let mut sec_o = sec.wrapping_sub(0x10);
            let mut out = table_at;
            for _ in 0..3u32 {
                let s1 = movaps(e, sec_o.wrapping_add(0x10));
                let s0 = movaps(e, sec_o);
                let mut link_m = link_base;
                let mut cell = out;
                for _ in 0..3u32 {
                    let lm2 = movaps(e, link_m.wrapping_add(0x10));
                    let b30 = movaps(e, body_b.wrapping_add(0x30));
                    let lm1 = movaps(e, link_m);
                    let part = mulps(mulps(lm2, s1), b30);
                    let part2 = mulps(s0, lm1);
                    let a30 = movaps(e, a30_at);
                    let part2 = mulps(part2, a30);
                    let total = addps(part, part2);
                    let s = sum3(total);
                    e.mem.set_f32(at(0xac), s);
                    e.mem.set_u32(cell, x87_quiet_bits(s.to_bits()));
                    link_m = link_m.wrapping_add(0x30);
                    cell = cell.wrapping_add(0x10);
                }
                out = out.wrapping_add(4);
                sec_o = sec_o.wrapping_add(0x20);
            }
            // Third part: the table moved into place.
            let table_at = ebx.wrapping_sub(0x110).wrapping_add(0x60);
            for (from, to) in [
                (0x00u32, 0xffff_ff20u32),
                (0x14, 0xffff_ff34),
                (0x28, 0xffff_ff48),
            ] {
                let bits = x87_quiet_bits(e.mem.u32(table_at.wrapping_add(from)));
                e.mem.set_u32(ebx.wrapping_add(to), bits);
            }
            for to in [0xffff_ff2cu32, 0xffff_ff3c, 0xffff_ff4c] {
                e.mem.set_u32(ebx.wrapping_add(to), 0);
            }
            for (from, to) in [
                (0x10u32, 0xffff_ff24u32),
                (0x04, 0xffff_ff30),
                (0x20, 0xffff_ff28),
                (0x08, 0xffff_ff40),
                (0x24, 0xffff_ff38),
                (0x18, 0xffff_ff44),
            ] {
                let bits = x87_quiet_bits(e.mem.u32(table_at.wrapping_add(from)));
                e.mem.set_u32(ebx.wrapping_add(to), bits);
            }
            // Fourth part.
            let n = count;
            if index < n.wrapping_sub(1) {
                let mut edi = lnk.wrapping_sub(0xa0);
                let mut sec_cur = sec;
                let mut out = ebx.wrapping_sub(0x20);
                for _ in 0..3u32 {
                    let cur = movaps(e, sec_cur);
                    let l2 = movaps(e, edi.wrapping_add(0x20));
                    let mut edx_m = lnk;
                    let mut esi_m = sec.wrapping_add(0x50);
                    let mut cell = out;
                    for _ in 0..3u32 {
                        let b30 = movaps(e, body_b.wrapping_add(0x30));
                        let next0 = movaps(e, edx_m.wrapping_sub(0x10));
                        let own0 = movaps(e, edi);
                        let next1 = movaps(e, edx_m);
                        let part = mulps(next0, own0);
                        let b30b = movaps(e, body_b.wrapping_add(0x30));
                        let splat = shufps(b30, b30, 0xff);
                        let part = mulps(part, splat);
                        let diff = subps(mulps(mulps(next1, l2), b30b), part);
                        let s = sum3(diff);
                        let next_sec = movaps(e, esi_m);
                        e.mem.set_f32(at(0xb8), s);
                        e.mem
                            .set_u32(cell.wrapping_sub(0x30), x87_quiet_bits(s.to_bits()));
                        let b30 = movaps(e, body_b.wrapping_add(0x30));
                        let v = mulps(mulps(next_sec, cur), b30);
                        let s2 = sum3(v);
                        e.mem.set_f32(at(0x9c), s2);
                        e.mem
                            .set_u32(cell.wrapping_add(0x60), x87_quiet_bits(s2.to_bits()));
                        let b30 = movaps(e, body_b.wrapping_add(0x30));
                        let v = mulps(mulps(next_sec, l2), b30);
                        let s3 = sum3(v);
                        e.mem.set_f32(at(0xa0), s3);
                        e.mem.set_u32(cell, x87_quiet_bits(s3.to_bits()));
                        let b30 = movaps(e, body_b.wrapping_add(0x30));
                        let v = mulps(mulps(next1, cur), b30);
                        let s4 = sum3(v);
                        e.mem.set_f32(at(0x90), s4);
                        e.mem
                            .set_u32(cell.wrapping_add(0x30), x87_quiet_bits(s4.to_bits()));
                        esi_m = esi_m.wrapping_add(0x20);
                        edx_m = edx_m.wrapping_add(0x30);
                        cell = cell.wrapping_add(0x10);
                    }
                    out = out.wrapping_add(4);
                    sec_cur = sec_cur.wrapping_add(0x20);
                    edi = edi.wrapping_add(0x30);
                }
            } else {
                for k in 0..12u32 {
                    store_xmm(e, ebx.wrapping_sub(0x50).wrapping_add(0x10 * k), [0.0; 4]);
                }
            }
            index += 1;
            tptr = tptr.wrapping_add(4);
            lnk = lnk.wrapping_add(0x90);
            sec = sec.wrapping_add(0x60);
            m = m.wrapping_add(0x3c0);
            if index >= count {
                break;
            }
        }
        let mut record = flags.addr();
        let mut block = matrices.addr();
        let mut after = count.wrapping_sub(1);
        let mut left = count;
        loop {
            e.mem.set_u8(at(0x90), u8::from(after != 0));
            let flag = e.mem.u32(at(0x90));
            e.call(0x00df_0870, &args![flag, block, record, at(0xe0), at(0xb0)]);
            after = after.wrapping_sub(1);
            record = record.wrapping_add(0x40);
            block = block.wrapping_add(0x3c0);
            left -= 1;
            if left == 0 {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_06a0, fn_00df06a0(Ptr, u32, Ptr, Ptr)),
        entry!(0x00df_07c0, fn_00df07c0(Ptr, Ptr)),
        entry!(0x00df_0810, fn_00df0810(u32, u32, Ptr, Ptr)),
        entry!(0x00df_0870, fn_00df0870(u32, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00df_0d30, fn_00df0d30(Ptr, Ptr)),
        entry!(0x00df_1060, fn_00df1060(u32, u32, Ptr, Ptr)),
        entry!(0x00df_11a0, fn_00df11a0(Ptr, Ptr)),
        entry!(0x00df_1260, fn_00df1260(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00df_1430, fn_00df1430(Ptr, Ptr, Ptr)),
        entry!(
            0x00df_19e0,
            fn_00df19e0(i32, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u32)
        ),
        entry!(0x00df_2040, fn_00df2040(i32, i32, Ptr, Ptr)),
        entry!(0x00df_21c0, fn_00df21c0(Ptr, u32, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn put(e: &mut Engine, at: u32, lanes: [f32; 4]) {
        store_xmm(e, at, lanes);
    }

    /// A double that logs its argument words, then runs `body` on them.
    fn recording(
        e: &mut Engine,
        address: u32,
        body: impl Fn(&mut Engine, &[u32]) + 'static,
    ) -> CallLog {
        let log: CallLog = Rc::default();
        let seen = log.clone();
        e.register_double(address, move |e, a| {
            seen.borrow_mut().push(a.to_vec());
            body(e, a);
            Ret::default()
        });
        log
    }

    /// Small integers in -3..=3, a repeatable sequence: every product and sum in the chain
    /// formulas stays exact in a float.
    fn small(seed: &mut u32) -> f32 {
        *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        ((*seed >> 16) % 7) as f32 - 3.0
    }

    /// Fills `words` floats at `at` with small integers.
    fn fill(e: &mut Engine, at: u32, words: u32, seed: &mut u32) {
        for word in 0..words {
            let value = small(seed);
            e.mem.set_f32(at + 4 * word, value);
        }
    }

    fn dot(a: [f32; 4], b: [f32; 4]) -> f32 {
        a[2] * b[2] + (a[1] * b[1] + a[0] * b[0])
    }

    fn vector(e: &Engine, at: u32) -> [f32; 4] {
        movaps(e, at)
    }

    fn sub(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
        [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
    }

    fn scaled(a: [f32; 4], f: f32) -> [f32; 4] {
        [a[0] * f, a[1] * f, a[2] * f, a[3] * f]
    }

    fn sum(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
    }

    fn times(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
        [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
    }

    const SENTINEL: f32 = 7.0;

    fn filled(e: &mut Engine, size: u32) -> u32 {
        let at = e.mem.alloc(size);
        for word in 0..size / 4 {
            e.mem.set_f32(at + 4 * word, SENTINEL);
        }
        at
    }

    #[test]
    fn the_first_block_marks_each_axis_with_a_unit_entry() {
        let mut e = Engine::new();
        let flags = e.mem.alloc(0x40);
        let a = filled(&mut e, 0xc0);
        let b = filled(&mut e, 0xc0);
        e.mem.set_u8(flags, 0x05);
        e.call(0x00df_06a0, &args![flags, 1u32, a, b]);
        for axis in 0..2u32 {
            for j in 0..6u32 {
                let at = 0x60 + 4 * axis + 0x10 * j;
                let unit = if j == 3 + axis { 1.0 } else { 0.0 };
                assert_eq!(e.mem.f32(a + at), unit, "a axis {axis} row {j}");
                assert_eq!(e.mem.f32(b + at), 0.0, "b axis {axis} row {j}");
            }
        }
        for j in 0..6u32 {
            assert_eq!(e.mem.f32(a + 0x68 + 0x10 * j), SENTINEL);
            assert_eq!(e.mem.f32(b + 0x68 + 0x10 * j), SENTINEL);
        }
        assert_eq!(e.mem.f32(a), SENTINEL);
    }

    #[test]
    fn the_second_block_is_left_alone_when_the_low_byte_of_the_flag_is_zero() {
        let mut e = Engine::new();
        let flags = e.mem.alloc(0x40);
        let a = filled(&mut e, 0xc0);
        let b = filled(&mut e, 0xc0);
        e.mem.set_u8(flags, 0x30);
        e.call(0x00df_06a0, &args![flags, 0x100u32, a, b]);
        for j in 0..6u32 {
            let at = 0x68 + 0x10 * j;
            assert_eq!(e.mem.f32(a + at), if j == 5 { 1.0 } else { 0.0 });
            assert_eq!(e.mem.f32(b + at), SENTINEL);
        }
        assert_eq!(e.mem.f32(a + 0x60), SENTINEL);
        e.mem.set_u8(flags, 0);
        let c = filled(&mut e, 0xc0);
        e.call(0x00df_06a0, &args![flags, 1u32, c, b]);
        assert_eq!(e.mem.f32(c + 0x68), SENTINEL);
    }

    #[test]
    fn the_vector_clearer_takes_the_axes_of_the_flag_byte() {
        let mut e = Engine::new();
        let flags = e.mem.alloc(0x40);
        let block = filled(&mut e, 0xc0 + 0x30);
        e.mem.set_u8(flags, 0x04);
        e.call(0x00df_07c0, &args![flags, block]);
        for k in 0..3u32 {
            for at in [0x30 + 0x10 * k, 0x90 + 0x10 * k] {
                let expected = if k == 1 { 0.0 } else { SENTINEL };
                for lane in 0..4u32 {
                    assert_eq!(e.mem.f32(block + at + 4 * lane), expected);
                }
            }
        }
        e.mem.set_u8(flags, 0);
        let other = filled(&mut e, 0xc0 + 0x30);
        e.call(0x00df_07c0, &args![flags, other]);
        assert_eq!(e.mem.f32(other + 0x40), SENTINEL);
    }

    #[test]
    fn the_flag_entry_copy_picks_its_float_by_the_mode() {
        let mut e = Engine::new();
        let flags = e.mem.alloc(0x80);
        let rows = e.mem.alloc(0x80);
        e.mem.set_u8(flags + 0x40, 0x10);
        e.mem.set_u32(flags + 0x40 + 0x28 + 4, 0x7f80_0001);
        e.mem.set_f32(flags + 0x40 + 0x28 + 8, 5.0);
        e.call(0x00df_0810, &args![1u32, 2u32, flags, rows]);
        assert_eq!(e.mem.u32(rows + 0x20 + 8 + 0x10), 0x7fc0_0001);
        e.mem.set_u8(flags + 0x40, 0x20);
        e.call(0x00df_0810, &args![1u32, 2u32, flags, rows]);
        assert_eq!(e.mem.f32(rows + 0x20 + 8 + 0x10), 5.0);
        e.mem.set_u8(flags + 0x40, 0x30);
        e.call(0x00df_0810, &args![1u32, 2u32, flags, rows]);
        assert_eq!(e.mem.f32(rows + 0x20 + 8 + 0x10), 5.0);
    }

    #[test]
    fn a_flag_entry_with_a_high_shift_reads_the_second_float() {
        let mut e = Engine::new();
        let flags = e.mem.alloc(0x80);
        let rows = e.mem.alloc(0x80);
        e.mem.set_u8(flags, 0xff);
        e.mem.set_f32(flags + 0x14 * 4 + 4, 1.0);
        e.mem.set_f32(flags + 0x14 * 4 + 8, 2.0);
        // second = 4: 8-bit shift by 8 gives 0, so the mode is not 1.
        e.call(0x00df_0810, &args![0u32, 4u32, flags, rows]);
        assert_eq!(e.mem.f32(rows + 16 + 0x10), 2.0);
    }

    type OrderedLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    struct Block {
        e: Engine,
        block: u32,
        flagrec: u32,
        out: u32,
        cursor: u32,
        log: OrderedLog,
    }

    const E29250: u32 = BLOCK_PREPARE;
    const E29470: u32 = BLOCK_BUILD;
    const E29380: u32 = BLOCK_FINISH;
    const E298F0: u32 = BLOCK_REFINE;

    /// A block of 0x3c0 bytes whose words are their own index, doubles for the six helpers (each
    /// logs its argument words; `00df06a0` also the 24 vectors at its third word and `00e298f0`
    /// the 12 vectors at its second word).
    fn block_rig() -> Block {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let block = e.mem.alloc(0x3c0 + 0x40);
        for word in 0..0x3c0 / 4 {
            e.mem.set_f32(block + 4 * word, word as f32);
        }
        let flagrec = e.mem.alloc(0x40);
        let out = e.mem.alloc(0xc0);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, 0x1234);
        e.mem.set_f32(MINIMUM, 0.25);
        let log: OrderedLog = Rc::default();
        for (address, words) in [
            (0x00df_06a0u32, 4usize),
            (0x00df_07c0, 2),
            (E29250, 2),
            (E29470, 3),
            (E29380, 2),
            (E298F0, 2),
        ] {
            let seen = log.clone();
            e.register_double(address, move |e, a| {
                assert_eq!(a.len(), words);
                let mut copy = a.to_vec();
                if address == 0x00df_06a0 {
                    for lane in 0..(24 * 4) {
                        copy.push(e.mem.u32(a[2] + 4 * lane));
                    }
                }
                if address == E298F0 {
                    for lane in 0..(12 * 4) {
                        copy.push(e.mem.u32(a[1] + 4 * lane));
                    }
                }
                seen.borrow_mut().push((address, copy));
                Ret::default()
            });
        }
        Block {
            e,
            block,
            flagrec,
            out,
            cursor,
            log,
        }
    }

    #[test]
    fn a_block_step_runs_the_helpers_in_order_and_clamps_six_floats() {
        let mut r = block_rig();
        // The clamp reads vectors 0, 1, 2, 9, 10 and 11 of the twelve that were copied from
        // the working copy (the vectors at +0x240) to +0xc0.
        let source = r.block + 0x240;
        r.e.mem.set_f32(source, -1.0);
        r.e.mem.set_f32(source + 0x10 + 4, 0.25);
        r.e.mem.set_f32(source + 0x20 + 8, f32::NAN);
        r.e.mem.set_f32(source + 0x90, 5.0);
        r.e.mem.set_f32(source + 0xa0 + 4, 0.2);
        r.e.mem.set_f32(source + 0xb0 + 8, 1.0);
        r.e.call(
            0x00df_0870,
            &args![1u32, r.block, r.flagrec, r.out, r.cursor],
        );
        let log = r.log.borrow();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x00df_06a0,
                0x00df_07c0,
                E29250,
                E29470,
                E29380,
                E298F0,
                E29470
            ]
        );
        // 00df06a0: (flag record, flag, copy, copy + 0xc0), the copy holding the vectors at +0x240.
        let first = &log[0].1;
        assert_eq!(first[..2], [r.flagrec, 1]);
        assert_eq!(first[3], first[2] + 0xc0);
        assert_eq!(f32::from_bits(first[4]), -1.0);
        assert_eq!(f32::from_bits(first[4 + 1]), 145.0);
        assert_eq!(f32::from_bits(first[4 + 24 * 4 - 1]), 239.0);
        assert_eq!(log[1].1, vec![r.flagrec, r.out]);
        assert_eq!(log[2].1, vec![r.block, r.out]);
        // 00e29470 gets a local block, the block and the cursor's old target.
        assert_eq!(log[3].1[1..], [r.block, 0x1234]);
        // 00e29380: (block + 0xc0, the local block of 00e29470).
        assert_eq!(log[4].1, vec![r.block + 0xc0, log[3].1[0]]);
        // 00e298f0: (block + 0xc0, a copy) with the clamped twelve vectors.
        assert_eq!(log[5].1[0], r.block + 0xc0);
        let clamped = &log[5].1[2..];
        assert_eq!(f32::from_bits(clamped[0]), 0.25);
        assert_eq!(f32::from_bits(clamped[4 + 1]), 0.25);
        assert_eq!(f32::from_bits(clamped[8 + 2]), 0.25);
        assert_eq!(f32::from_bits(clamped[9 * 4]), 5.0);
        assert_eq!(f32::from_bits(clamped[10 * 4 + 1]), 0.25);
        assert_eq!(f32::from_bits(clamped[11 * 4 + 2]), 1.0);
        assert_eq!(f32::from_bits(clamped[3]), 147.0);
        // The last helper builds the next block at +0x180 from the first block and the second
        // half of the copy.
        assert_eq!(log[6].1[..2], [r.block + 0x180, r.block + 0xc0]);
        assert_eq!(log[6].1[2], first[2] + 0xc0);
        assert_eq!(r.e.mem.u32(r.cursor), r.block + 0x180);
        for vector in 0..12u32 {
            for lane in 0..4u32 {
                assert_eq!(
                    r.e.mem.u32(r.out + 0x10 * vector + 4 * lane),
                    r.e.mem.u32(r.block + 0x300 + 0x10 * vector + 4 * lane)
                );
            }
        }
    }

    /// Runs `00df2040` over six blocks with a double for `00df0870` that logs its words, the
    /// twelve vectors at its fourth word and the pointer its fifth word points at.
    fn walk(found: i32, count: i32) -> (u32, u32, Vec<Vec<u32>>) {
        let mut e = Engine::new();
        let matrices = e.mem.alloc(0x3c0 * 6);
        let flags = e.mem.alloc(0x40 * 6);
        for word in 0..(0x3c0 * 6 / 4) {
            e.mem.set_f32(matrices + 4 * word, word as f32);
        }
        let log: CallLog = Rc::default();
        let seen = log.clone();
        e.register_double(0x00df_0870, move |e, a| {
            let mut words = a.to_vec();
            for word in 0..48 {
                words.push(e.mem.u32(a[3] + 4 * word));
            }
            words.push(e.mem.u32(a[4]));
            seen.borrow_mut().push(words);
            Ret::default()
        });
        e.call(0x00df_2040, &args![found, count, flags, matrices]);
        let calls = log.borrow().clone();
        (matrices, flags, calls)
    }

    #[test]
    fn the_chain_walk_starts_at_the_block_before_the_found_one() {
        let (matrices, flags, log) = walk(3, 5);
        assert_eq!(log.len(), 3);
        for (i, call) in log.iter().enumerate() {
            let k = 2 + i as u32;
            assert_eq!(call[0], u32::from(i != 2));
            assert_eq!(call[1], matrices + 0x3c0 * k);
            assert_eq!(call[2], flags + 0x40 * k);
            assert_eq!(call[3], log[0][3]);
            assert_eq!(call[4], log[0][4]);
        }
        // The twelve vectors before block 2, and the pointer variable at block 2 - 0x240.
        for word in 0..48u32 {
            let expected = (2 * 0x3c0 - 0xc0) / 4 + word;
            assert_eq!(f32::from_bits(log[0][5 + word as usize]), expected as f32);
        }
        assert_eq!(log[0][53], matrices + 2 * 0x3c0 - 0x240);
    }

    #[test]
    fn the_chain_walk_from_the_start_runs_every_block_on_a_zero_array() {
        let (matrices, flags, log) = walk(0, 5);
        assert_eq!(log.len(), 5);
        let flag_words: Vec<u32> = log.iter().map(|call| call[0]).collect();
        assert_eq!(flag_words, vec![1, 1, 1, 1, 0]);
        for (k, call) in log.iter().enumerate() {
            assert_eq!(call[1], matrices + 0x3c0 * k as u32);
            assert_eq!(call[2], flags + 0x40 * k as u32);
        }
        assert!(log[0][5..53].iter().all(|word| *word == 0));
        assert_eq!(log[0][53], log[0][3]);
        // A found index of 1 starts at block 0 as well.
        assert_eq!(walk(1, 2).2.len(), 2);
    }

    #[test]
    fn the_chain_walk_does_nothing_past_the_end() {
        assert!(walk(4, 3).2.is_empty());
        assert!(walk(2, 0).2.is_empty());
        assert!(walk(-5, 0).2.is_empty());
        assert_eq!(walk(-5, 1).2.len(), 1);
    }

    #[test]
    fn a_chain_block_is_written_into_the_output_buffer() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(OUTPUT_BIAS, 1.192_092_895_507_812_5e-7);
        let source = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x100);
        let out = e.mem.alloc(0x1000);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, out);
        e.mem.set_f32(out, 1.5);
        e.mem.set_u32(source, 2);
        e.mem.set_f32(source + 4, 1.25);
        e.mem.set_f32(source + 8, 2.5);
        e.mem.set_u32(source + 0x1c, array);
        e.mem.set_u32(source + 0x20, 0x7777);
        e.mem.set_u32(source + 0x24, records);
        e.mem.set_f32(source + 0x28, 3.5);
        e.mem.set_u32(source + 0x2c, 0xaabb);
        for word in 0..3u32 {
            e.mem.set_u32(array + 4 * word, 0x100 + word);
        }
        for word in 0..32u32 {
            e.mem.set_u32(records + 4 * word, 0x200 + word);
        }
        let log = recording(&mut e, BUILD_CHAIN_ROWS, |_, _| {});
        e.call(0x00df_21c0, &args![source, 0x99u32, cursor]);
        assert_eq!(e.mem.u32(out), 0x3fc0_001f);
        assert_eq!(e.mem.u16(out + 4), 2);
        assert_eq!(e.mem.u32(out + 8), 0xaabb);
        assert_eq!(e.mem.u32(out + 0x10), 0xab0);
        assert_eq!(e.mem.f32(out + 0x14), 3.5);
        assert_eq!(e.mem.f32(out + 0x18), 1.25);
        assert_eq!(e.mem.f32(out + 0x1c), 2.5);
        for word in 0..3u32 {
            assert_eq!(e.mem.u32(out + 0xa20 + 4 * word), 0x100 + word);
        }
        for word in 0..32u32 {
            assert_eq!(e.mem.u32(out + 0xa2c + 4 * word), 0x200 + word);
        }
        assert_eq!(
            *log.borrow(),
            vec![vec![
                2,
                source + 0xc,
                records,
                out + 0x20,
                out + 0x20 + 2 * 0x90,
                out + 0xa20,
                0x7777,
                out + 0x1e0 + 0x20,
                2 * 0x3c0
            ]]
        );
        assert_eq!(e.mem.u32(cursor), out + 0xab0);
    }

    #[test]
    fn an_empty_chain_block_copies_nothing() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(OUTPUT_BIAS, 0.0);
        let source = e.mem.alloc(0x40);
        let array = e.mem.alloc(0x20);
        let out = e.mem.alloc(0x200);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, out);
        e.mem.set_u32(source + 0x1c, array);
        e.mem.set_u32(array, 0xfeed);
        let log = recording(&mut e, BUILD_CHAIN_ROWS, |_, _| {});
        e.call(0x00df_21c0, &args![source, 0u32, cursor]);
        // One dword is copied (n + 1 = 1), no records.
        assert_eq!(e.mem.u32(out + 0x40), 0xfeed);
        assert_eq!(e.mem.u16(out + 4), 0);
        assert_eq!(e.mem.u32(out + 0x10), 0x50);
        assert_eq!(log.borrow()[0][0], 0);
        assert_eq!(e.mem.u32(cursor), out + 0x50);
    }

    struct Chain {
        e: Engine,
        ctx: u32,
        rows: u32,
        count: u32,
    }

    /// A chain context of `count` links with every table filled with small integers.
    fn chain(count: u32, seed: u32) -> Chain {
        let mut e = Engine::new();
        let mut seed = seed;
        let info = e.mem.alloc(0x80);
        let ctx = e.mem.alloc(0x30);
        let links = e.mem.alloc(0x90 * count);
        let seconds = e.mem.alloc(0x60 * count);
        let table = e.mem.alloc(4 * (count + 1));
        let bodies = e.mem.alloc(0x100 * (count + 1));
        let matrices = e.mem.alloc(0x3c0 * count);
        let flags = e.mem.alloc(0x40 * count);
        let vectors = e.mem.alloc(0x20 * (count + 1));
        let rows = e.mem.alloc(0x20 * count);
        e.mem.set_f32(info + 0x40, 2.0);
        fill(&mut e, links, 0x90 / 4 * count, &mut seed);
        fill(&mut e, seconds, 0x60 / 4 * count, &mut seed);
        fill(&mut e, bodies, 0x100 / 4 * (count + 1), &mut seed);
        fill(&mut e, flags, 0x40 / 4 * count, &mut seed);
        for index in 0..=count {
            e.mem.set_u32(table + 4 * index, 0x100 * index);
        }
        for word in 0..0x20 / 4 * count {
            e.mem.set_f32(rows + 4 * word, 99.0);
        }
        for (word, value) in [
            (0, info),
            (1, count),
            (2, 3.0f32.to_bits()),
            (3, 2.0f32.to_bits()),
            (4, links),
            (5, seconds),
            (6, table),
            (7, bodies),
            (8, matrices),
            (9, flags),
            (10, vectors),
            (11, rows),
        ] {
            e.mem.set_u32(ctx + 4 * word, value);
        }
        Chain {
            e,
            ctx,
            rows,
            count,
        }
    }

    fn lanes(a: [f32; 4]) -> f32 {
        dot(a, [1.0; 4])
    }

    /// The two row values of link `i`, axis `j` by the formulas of the decompilation: the
    /// first (stored at `rows + 0x20 * i + 4 * j`) and the second (`... + 0x10`).
    fn expected(e: &Engine, ctx: u32, i: u32, j: u32) -> (f32, f32) {
        let word = |at: u32| e.mem.u32(at);
        let float = |at: u32| e.mem.f32(at);
        let info = word(ctx);
        let sv = float(info + 0x40);
        let (f2, f3) = (float(ctx + 8), float(ctx + 0xc));
        let table = word(ctx + 0x18);
        let bodies = word(ctx + 0x1c);
        let a = word(table + 4 * i) + bodies;
        let b = word(table + 4 * i + 4) + bodies;
        let links = word(ctx + 0x10) + 0x90 * i + 0x30 * j;
        let seconds = word(ctx + 0x14) + 0x60 * i + 0x20 * j;
        let flags = word(ctx + 0x24) + 0x40 * i + 0x14 * j;
        let v = |at: u32| vector(e, at);
        let (a10, a20, a40, a50) = (v(a + 0x10), v(a + 0x20), v(a + 0x40), v(a + 0x50));
        let (b10, b20, b40, b50) = (v(b + 0x10), v(b + 0x20), v(b + 0x40), v(b + 0x50));
        let ga = sub(a20, a50);
        let fb = sub(b20, b50);
        let d = sub(sub(a10, a40), sub(b10, b40));
        let diff = sub(a40, b40);
        let (lm1, l0, l1) = (v(links), v(links + 0x10), v(links + 0x20));
        let part = scaled(sum(sum(times(ga, l0), times(fb, l1)), times(d, lm1)), f3);
        let rest = scaled(
            sum(sum(times(l1, b50), times(a50, l0)), times(diff, lm1)),
            sv * f2,
        );
        let first = f2 * float(links + 0x0c) - lanes(sum(part, rest));
        let (s_prev, s_here) = (v(seconds), v(seconds + 0x10));
        let (m, c) = (float(flags + 0x10), float(flags + 0x14));
        let part = scaled(sum(times(ga, s_prev), times(fb, s_here)), c);
        let rest = scaled(sum(times(a50, s_prev), times(b50, s_here)), sv * m);
        let second = m * float(seconds + 0x1c) - lanes(sum(part, rest));
        (first, second)
    }

    #[test]
    fn the_two_rows_of_every_axis_follow_the_formulas() {
        let mut c = chain(2, 12345);
        c.e.call(0x00df_0d30, &args![c.ctx, c.rows]);
        for i in 0..c.count {
            for j in 0..3u32 {
                let (first, second) = expected(&c.e, c.ctx, i, j);
                let at = c.rows + 0x20 * i + 4 * j;
                assert_eq!(c.e.mem.f32(at), first, "link {i} axis {j} first");
                assert_eq!(c.e.mem.f32(at + 0x10), second, "link {i} axis {j} second");
            }
            assert_eq!(c.e.mem.f32(c.rows + 0x20 * i + 0xc), 0.0);
            assert_eq!(c.e.mem.f32(c.rows + 0x20 * i + 0x1c), 0.0);
        }
    }

    #[test]
    fn no_links_leave_the_rows_alone() {
        let mut c = chain(1, 7);
        c.e.mem.set_u32(c.ctx + 4, 0);
        c.e.call(0x00df_0d30, &args![c.ctx, c.rows]);
        assert_eq!(c.e.mem.f32(c.rows), 99.0);
        c.e.mem.set_u32(c.ctx + 4, 0xffff_ffff);
        c.e.call(0x00df_0d30, &args![c.ctx, c.rows]);
        assert_eq!(c.e.mem.f32(c.rows), 99.0);
    }

    #[test]
    fn a_single_row_value_matches_the_second_row_of_the_full_pass() {
        let mut c = chain(2, 99);
        for i in 0..2u32 {
            for j in 0..3u32 {
                c.e.call(0x00df_1060, &args![i, j, c.ctx, c.rows]);
                let (_, second) = expected(&c.e, c.ctx, i, j);
                assert_eq!(c.e.mem.f32(c.rows + 0x20 * i + 4 * j + 0x10), second);
                assert_eq!(c.e.mem.f32(c.rows + 0x20 * i + 4 * j), 99.0);
            }
        }
    }

    #[test]
    fn a_signalling_row_input_comes_out_quieted() {
        let mut c = chain(1, 5);
        let flags = c.e.mem.u32(c.ctx + 0x24);
        c.e.mem.set_u32(flags + 0x10, 0x7f80_0001);
        c.e.call(0x00df_1060, &args![0u32, 0u32, c.ctx, c.rows]);
        assert_eq!(c.e.mem.u32(c.rows + 0x10) & 0x7fc0_0000, 0x7fc0_0000);
    }

    #[test]
    fn the_forward_pass_alternates_the_apply_helper_with_the_row_subtraction() {
        let mut c = chain(2, 31);
        let out = c.e.mem.alloc(0x20 * 3);
        let rows = c.e.mem.u32(c.ctx + 0x2c);
        put(&mut c.e, rows, [10.0, 20.0, 30.0, 40.0]);
        put(&mut c.e, rows + 0x10, [50.0, 60.0, 70.0, 80.0]);
        put(&mut c.e, rows + 0x20, [1.0, 2.0, 3.0, 4.0]);
        put(&mut c.e, rows + 0x30, [5.0, 6.0, 7.0, 8.0]);
        let log = recording(&mut c.e, MATRIX_APPLY, |e, a| {
            let first = sum(vector(e, a[2]), [1.0, 2.0, 3.0, 4.0]);
            let second = sum(vector(e, a[2] + 0x10), [5.0, 6.0, 7.0, 8.0]);
            put(e, a[0], first);
            put(e, a[0] + 0x10, second);
        });
        c.e.call(0x00df_11a0, &args![c.ctx, out]);
        let matrices = c.e.mem.u32(c.ctx + 0x20);
        let log = log.borrow();
        assert_eq!(log.len(), 4);
        let temp = log[0][0];
        assert_eq!(log[0], vec![temp, matrices, out]);
        assert_eq!(log[1], vec![out + 0x20, matrices + 0xc0, temp]);
        assert_eq!(log[2], vec![temp, matrices + 0x3c0, out + 0x20]);
        assert_eq!(log[3], vec![out + 0x40, matrices + 0x3c0 + 0xc0, temp]);
        assert_eq!(vector(&c.e, out), [0.0; 4]);
        assert_eq!(vector(&c.e, out + 0x10), [0.0; 4]);
        assert_eq!(vector(&c.e, out + 0x20), [10.0, 20.0, 30.0, 40.0]);
        assert_eq!(vector(&c.e, out + 0x30), [50.0, 60.0, 70.0, 80.0]);
        assert_eq!(vector(&c.e, out + 0x40), [-9.0, -18.0, -27.0, -36.0]);
        assert_eq!(vector(&c.e, out + 0x50), [-45.0, -54.0, -63.0, -72.0]);
    }

    #[test]
    fn a_forward_pass_without_links_only_clears_the_first_pair() {
        let mut c = chain(1, 3);
        c.e.mem.set_u32(c.ctx + 4, 0);
        let out = c.e.mem.alloc(0x40);
        put(&mut c.e, out, [1.0; 4]);
        put(&mut c.e, out + 0x10, [2.0; 4]);
        put(&mut c.e, out + 0x20, [3.0; 4]);
        let log = recording(&mut c.e, MATRIX_APPLY, |_, _| {});
        c.e.call(0x00df_11a0, &args![c.ctx, out]);
        assert!(log.borrow().is_empty());
        assert_eq!(vector(&c.e, out), [0.0; 4]);
        assert_eq!(vector(&c.e, out + 0x10), [0.0; 4]);
        assert_eq!(vector(&c.e, out + 0x20), [3.0; 4]);
    }

    struct Search {
        c: Chain,
        link: u32,
        axis: u32,
        value: u32,
        log: CallLog,
    }

    /// A one-link chain for the limit searches: the pair of vectors after the zero walk is
    /// `(?, (1, 2, 3))`, the helper answers zeros, the limits scale is 2.0 and the flag entries
    /// hold `f1 = (0, 0.5, 0.25)` and `f2 = (0.25, 0, 0)`.
    fn search(modes: u8) -> Search {
        let mut c = chain(1, 17);
        c.e.map(0x0100_0000, 0x0030_0000);
        c.e.mem.set_f64(LIMIT_SCALE, 2.0);
        let vectors = c.e.mem.u32(c.ctx + 0x28);
        put(&mut c.e, vectors + 0x20, [0.0; 4]);
        put(&mut c.e, vectors + 0x30, [1.0, 2.0, 3.0, 0.0]);
        let flags = c.e.mem.u32(c.ctx + 0x24);
        c.e.mem.set_u8(flags, modes);
        for (axis, f1, f2) in [(0u32, 0.0f32, 0.25f32), (1, 0.5, 0.0), (2, 0.25, 0.0)] {
            c.e.mem.set_f32(flags + 0x14 * axis + 4, f1);
            c.e.mem.set_f32(flags + 0x14 * axis + 8, f2);
        }
        let link = c.e.mem.alloc(4);
        let axis = c.e.mem.alloc(4);
        let value = c.e.mem.alloc(4);
        c.e.mem.set_u32(link, 0xdead);
        c.e.mem.set_u32(axis, 0xbeef);
        c.e.mem.set_u32(value, 0xcafe);
        let log = recording(&mut c.e, MATRIX_APPLY, |e, a| {
            put(e, a[0], [0.0; 4]);
            put(e, a[0] + 0x10, [0.0; 4]);
        });
        Search {
            c,
            link,
            axis,
            value,
            log,
        }
    }

    #[test]
    fn the_limit_search_keeps_the_axis_that_overshoots_most_and_sets_its_mode() {
        let mut s = search(0);
        let (ctx, link, axis, value) = (s.c.ctx, s.link, s.axis, s.value);
        s.c.e.call(0x00df_1260, &args![ctx, link, axis, value]);
        assert_eq!(s.c.e.mem.u32(link), 0);
        assert_eq!(s.c.e.mem.u32(axis), 2);
        assert_eq!(s.c.e.mem.f32(value), 3.0);
        let flags = s.c.e.mem.u32(ctx + 0x24);
        assert_eq!(s.c.e.mem.u8(flags), 0x10);
        let matrices = s.c.e.mem.u32(ctx + 0x20);
        let log = s.log.borrow();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0][1], matrices + 0x180);
        assert_eq!(log[0][2] + 0x20, log[0][0]);
    }

    #[test]
    fn the_limit_search_skips_axes_that_have_a_mode() {
        let mut s = search(0x30);
        let (ctx, link, axis, value) = (s.c.ctx, s.link, s.axis, s.value);
        s.c.e.call(0x00df_1260, &args![ctx, link, axis, value]);
        assert_eq!(s.c.e.mem.u32(link), 0);
        assert_eq!(s.c.e.mem.u32(axis), 0);
        assert_eq!(s.c.e.mem.f32(value), 1.0);
        let flags = s.c.e.mem.u32(ctx + 0x24);
        // Axis 0 moves to mode 1 (its `f1` is below the value), axis 2 keeps its mode 3.
        assert_eq!(s.c.e.mem.u8(flags), 0x31);
    }

    #[test]
    fn the_limit_search_reports_nothing_when_every_axis_has_a_mode() {
        let mut s = search(0x3f);
        let (ctx, link, axis, value) = (s.c.ctx, s.link, s.axis, s.value);
        s.c.e.call(0x00df_1260, &args![ctx, link, axis, value]);
        assert_eq!(s.c.e.mem.u32(link), 0xdead);
        assert_eq!(s.c.e.mem.u32(axis), 0xbeef);
        assert_eq!(s.c.e.mem.u32(value), 0xcafe);
        let flags = s.c.e.mem.u32(ctx + 0x24);
        assert_eq!(s.c.e.mem.u8(flags), 0x3f);
        s.c.e.mem.set_u32(ctx + 4, 0);
        s.c.e.call(0x00df_1260, &args![ctx, link, axis, value]);
        assert_eq!(s.c.e.mem.u32(link), 0xdead);
    }

    /// A one-link chain whose rows are all zero but for the float at `+0x1c` of the second
    /// records, `(2, -3, 5)`: the row values are those floats, the helpers answer zeros.
    fn error_search(modes: u8) -> Search {
        let mut c = chain(1, 23);
        let seconds = c.e.mem.u32(c.ctx + 0x14);
        let bodies = c.e.mem.u32(c.ctx + 0x1c);
        let flags = c.e.mem.u32(c.ctx + 0x24);
        for word in 0..0x60 / 4 {
            c.e.mem.set_f32(seconds + 4 * word, 0.0);
        }
        for word in 0..0x200 / 4 {
            c.e.mem.set_f32(bodies + 4 * word, 0.0);
        }
        for (axis, error) in [(0u32, 2.0f32), (1, -3.0), (2, 5.0)] {
            c.e.mem.set_f32(seconds + 0x20 * axis + 0x1c, error);
            c.e.mem.set_f32(flags + 0x14 * axis + 0x10, 1.0);
            c.e.mem.set_f32(flags + 0x14 * axis + 0x14, 0.0);
        }
        c.e.mem.set_u8(flags, modes);
        let link = c.e.mem.alloc(4);
        let axis = c.e.mem.alloc(4);
        c.e.mem.set_u32(link, 0xdead);
        c.e.mem.set_u32(axis, 0xbeef);
        let log: CallLog = Rc::default();
        {
            let seen = log.clone();
            c.e.register_double(MATRIX_APPLY, move |e, a| {
                let mut words = a.to_vec();
                if a[1] == a[0] + 0x90 {
                    for word in 0..48 {
                        words.push(e.mem.u32(a[1] + 4 * word));
                    }
                }
                seen.borrow_mut().push(words);
                put(e, a[0], [0.0; 4]);
                put(e, a[0] + 0x10, [0.0; 4]);
                Ret::default()
            });
        }
        c.e.register_double(BLOCK_PREPARE, |_, _| Ret::default());
        Search {
            c,
            link,
            axis,
            value: 0,
            log,
        }
    }

    #[test]
    fn the_error_search_clears_the_mode_of_the_largest_error() {
        let mut s = error_search(0x37);
        let (ctx, link, axis) = (s.c.ctx, s.link, s.axis);
        s.c.e.call(0x00df_1430, &args![ctx, link, axis]);
        assert_eq!(s.c.e.mem.u32(link), 0);
        assert_eq!(s.c.e.mem.u32(axis), 2);
        let flags = s.c.e.mem.u32(ctx + 0x24);
        assert_eq!(s.c.e.mem.u8(flags), 0x07);
    }

    #[test]
    fn the_error_search_ignores_errors_on_the_wrong_side() {
        // Axis 0 has mode 1 and a positive error: skipped; axis 1 has mode 1 and a negative one.
        let mut s = error_search(0x05);
        let (ctx, link, axis) = (s.c.ctx, s.link, s.axis);
        s.c.e.call(0x00df_1430, &args![ctx, link, axis]);
        assert_eq!(s.c.e.mem.u32(link), 0);
        assert_eq!(s.c.e.mem.u32(axis), 1);
        let flags = s.c.e.mem.u32(ctx + 0x24);
        assert_eq!(s.c.e.mem.u8(flags), 0x01);
        // Only modes 0 and 2: nothing is eligible.
        let mut s = error_search(0x28);
        let (ctx, link, axis) = (s.c.ctx, s.link, s.axis);
        s.c.e.call(0x00df_1430, &args![ctx, link, axis]);
        assert_eq!(s.c.e.mem.u32(link), 0xdead);
        assert_eq!(s.c.e.mem.u32(axis), 0xbeef);
    }

    #[test]
    fn the_error_search_runs_its_helpers_on_the_expected_blocks() {
        let mut s = error_search(0x37);
        let (ctx, link, axis) = (s.c.ctx, s.link, s.axis);
        s.c.e.call(0x00df_1430, &args![ctx, link, axis]);
        let matrices = s.c.e.mem.u32(ctx + 0x20);
        let log = s.log.borrow();
        assert_eq!(log.len(), 4);
        let frame = log[0][0] - 0x90;
        assert_eq!(log[0], vec![frame + 0x90, matrices + 0x180, frame + 0x70]);
        assert_eq!(log[1][..3], [frame + 0xb0, frame + 0x140, frame + 0x70]);
        assert_eq!(log[2], vec![frame + 0x90, matrices + 0x240, frame + 0xf0]);
        assert_eq!(log[3], vec![frame + 0x90, matrices + 0x300, frame + 0x120]);
        // The block that stands in for the missing previous one: three unit vectors twice.
        let block = &log[1][3..];
        let expected_units = [(0usize, 0usize), (1, 1), (2, 2), (9, 0), (10, 1), (11, 2)];
        for vector in 0..12usize {
            for lane in 0..4usize {
                let unit = expected_units.contains(&(vector, lane));
                assert_eq!(
                    f32::from_bits(block[vector * 4 + lane]),
                    if unit { 1.0 } else { 0.0 },
                    "vector {vector} lane {lane}"
                );
            }
        }
    }

    #[test]
    fn the_error_search_with_no_links_makes_one_call_and_reports_nothing() {
        let mut s = error_search(0x37);
        let (ctx, link, axis) = (s.c.ctx, s.link, s.axis);
        s.c.e.mem.set_u32(ctx + 4, 0);
        s.c.e.call(0x00df_1430, &args![ctx, link, axis]);
        let matrices = s.c.e.mem.u32(ctx + 0x20);
        let log = s.log.borrow();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0][1], matrices.wrapping_sub(0x240));
        assert_eq!(s.c.e.mem.u32(link), 0xdead);
    }

    struct Setup {
        e: Engine,
        floats: u32,
        flags: u32,
        links: u32,
        seconds: u32,
        table: u32,
        bodies: u32,
        matrices: u32,
        log: CallLog,
    }

    /// The inputs of `00df19e0` for `count` links: floats `(0.5, 2, 1.5, 4)`, the matrix blocks
    /// full of 9.0, and the links, second records and bodies full of small integers (or zero).
    /// A double of `00df0870` logs its five words, the pointer's target and the twelve vectors.
    fn setup(count: u32, seed: u32, zero: bool) -> Setup {
        let mut e = Engine::new();
        let mut seed = seed;
        let floats = e.mem.alloc(0x10);
        let flags = e.mem.alloc(0x40 * count);
        let links = e.mem.alloc(0x90 * count);
        let seconds = e.mem.alloc(0x60 * count);
        let table = e.mem.alloc(4 * (count + 1));
        let bodies = e.mem.alloc(0x100 * (count + 1));
        let matrices = e.mem.alloc(0x3c0 * count);
        for (word, value) in [0.5f32, 2.0, 1.5, 4.0].iter().enumerate() {
            e.mem.set_f32(floats + 4 * word as u32, *value);
        }
        for word in 0..0x3c0 / 4 * count {
            e.mem.set_f32(matrices + 4 * word, 9.0);
        }
        if !zero {
            fill(&mut e, links, 0x90 / 4 * count, &mut seed);
            fill(&mut e, seconds, 0x60 / 4 * count, &mut seed);
            fill(&mut e, bodies, 0x100 / 4 * (count + 1), &mut seed);
        }
        for index in 0..=count {
            e.mem.set_u32(table + 4 * index, 0x100 * index);
        }
        let log: CallLog = Rc::default();
        let seen = log.clone();
        e.register_double(0x00df_0870, move |e, a| {
            let mut words = a.to_vec();
            words.push(e.mem.u32(a[4]));
            for word in 0..48 {
                words.push(e.mem.u32(a[3] + 4 * word));
            }
            seen.borrow_mut().push(words);
            Ret::default()
        });
        Setup {
            e,
            floats,
            flags,
            links,
            seconds,
            table,
            bodies,
            matrices,
            log,
        }
    }

    fn run_setup(s: &mut Setup, count: i32) {
        let (floats, flags, links, seconds) = (s.floats, s.flags, s.links, s.seconds);
        let (table, bodies, matrices) = (s.table, s.bodies, s.matrices);
        s.e.call(
            0x00df_19e0,
            &args![count, floats, flags, links, seconds, table, bodies, matrices, 0xabcdu32],
        );
    }

    #[test]
    fn a_single_zero_link_leaves_only_the_diagonal_offsets() {
        let mut s = setup(1, 0, true);
        run_setup(&mut s, 1);
        let m = s.matrices + 0x350;
        let base = m - 0x110;
        for j in 0..3u32 {
            assert_eq!(s.e.mem.f32(base + 0x14 * j), 0.5, "axis {j}");
            assert_eq!(s.e.mem.f32(base + 0x14 * j + 0x90), 1.5, "axis {j}");
        }
        for offset in [
            0x10u32,
            0x04,
            0x24,
            0x18,
            0x10 + 0x90,
            0x04 + 0x90,
            0x08,
            0x20,
        ] {
            assert_eq!(s.e.mem.f32(base + offset), 0.0, "cell {offset:#x}");
        }
        // The table moved in place, and the zero block for the last link.
        for offset in [
            0xe0u32, 0xdc, 0xd8, 0xd4, 0xd0, 0xcc, 0xc8, 0xc4, 0xc0, 0xbc, 0xb8, 0xb4,
        ] {
            assert_eq!(s.e.mem.f32(m - offset), 0.0, "table cell -{offset:#x}");
        }
        for vector in 0..12u32 {
            for lane in 0..4u32 {
                assert_eq!(s.e.mem.f32(m - 0x50 + 0x10 * vector + 4 * lane), 0.0);
            }
        }
        let log = s.log.borrow();
        assert_eq!(log.len(), 1);
        let call = &log[0];
        assert_eq!(call[0], 0);
        assert_eq!(call[1..3], [s.matrices, s.flags]);
        assert_eq!(call[5], call[3]);
        assert!(call[6..].iter().all(|word| *word == 0));
    }

    #[test]
    fn the_matrix_entries_are_products_of_the_link_vectors() {
        let mut s = setup(1, 777, false);
        run_setup(&mut s, 1);
        let e = &s.e;
        let m = s.matrices + 0x350;
        let base = m - 0x110;
        let table = s.table;
        let a = s.bodies + e.mem.u32(table);
        let b = s.bodies + e.mem.u32(table + 4);
        let (a30, b30) = (vector(e, a + 0x30), vector(e, b + 0x30));
        let (aw, bw) = (a30[3], b30[3]);
        let link = |j: u32| {
            (
                vector(e, s.links + 0x30 * j),
                vector(e, s.links + 0x30 * j + 0x10),
                vector(e, s.links + 0x30 * j + 0x20),
            )
        };
        let second = |j: u32| {
            (
                vector(e, s.seconds + 0x20 * j),
                vector(e, s.seconds + 0x20 * j + 0x10),
            )
        };
        let s1 = |j: u32, k: u32| {
            let (j0, j1, j2) = link(j);
            let (k0, k1, k2) = link(k);
            let t = times(k0, j0);
            let first = sum(scaled(t, aw), scaled(t, bw));
            let rest = sum(times(times(k2, j2), b30), times(times(k1, j1), a30));
            lanes(sum(first, rest))
        };
        let s2 = |j: u32, k: u32| {
            let (j0, j1) = second(j);
            let (k0, k1) = second(k);
            lanes(sum(times(times(k1, j1), b30), times(times(j0, k0), a30)))
        };
        for j in 0..3u32 {
            for k in j..3u32 {
                let (mut first, mut second_value) = (s1(j, k), s2(j, k));
                if j == k {
                    first = 2.0 * first + 0.5;
                    second_value = second_value * 4.0 + 1.5;
                }
                let upper = base + 0x10 * j + 4 * k;
                let lower = base + 0x10 * k + 4 * j;
                assert_eq!(e.mem.f32(upper), first, "first ({j}, {k})");
                assert_eq!(e.mem.f32(lower), first, "first ({k}, {j})");
                assert_eq!(e.mem.f32(upper + 0x90), second_value, "second ({j}, {k})");
                assert_eq!(e.mem.f32(lower + 0x90), second_value, "second ({k}, {j})");
            }
        }
        // The 3 by 3 table at M - 0xb0 and its transposed copy.
        let link_second = |m_index: u32| {
            (
                vector(e, s.links + 0x10 + 0x30 * m_index),
                vector(e, s.links + 0x20 + 0x30 * m_index),
            )
        };
        let table_value = |o: u32, m_index: u32| {
            let (s0, s1v) = second(o);
            let (l1, l2) = link_second(m_index);
            lanes(sum(times(times(l2, s1v), b30), times(times(s0, l1), a30)))
        };
        for o in 0..3u32 {
            for m_index in 0..3u32 {
                assert_eq!(
                    e.mem.f32(m - 0xb0 + 4 * o + 0x10 * m_index),
                    table_value(o, m_index)
                );
            }
        }
        for (cell, o, m_index) in [
            (0xe0u32, 0u32, 0u32),
            (0xcc, 1, 1),
            (0xb8, 2, 2),
            (0xdc, 0, 1),
            (0xd0, 1, 0),
            (0xd8, 0, 2),
            (0xc0, 2, 0),
            (0xc8, 1, 2),
            (0xbc, 2, 1),
        ] {
            assert_eq!(
                e.mem.f32(m - cell),
                table_value(o, m_index),
                "cell -{cell:#x}"
            );
        }
    }

    #[test]
    fn a_link_before_the_last_gets_the_cross_terms_of_the_next_one() {
        let mut s = setup(2, 4242, false);
        run_setup(&mut s, 2);
        let e = &s.e;
        let m = s.matrices + 0x350;
        let b30 = vector(e, s.bodies + e.mem.u32(s.table + 4) + 0x30);
        let bw = [b30[3]; 4];
        let l = |at: u32| vector(e, s.links + at);
        for o in 0..3u32 {
            for m_index in 0..3u32 {
                let next0 = l(0x90 + 0x30 * m_index);
                let next1 = l(0x90 + 0x30 * m_index + 0x10);
                let own0 = l(0x30 * o);
                let own2 = l(0x30 * o + 0x20);
                let current = vector(e, s.seconds + 0x10 + 0x20 * o);
                let next_second = vector(e, s.seconds + 0x60 + 0x20 * m_index);
                let first = lanes(sub(
                    times(times(next1, own2), b30),
                    times(times(next0, own0), bw),
                ));
                let second = lanes(times(times(next_second, current), b30));
                let third = lanes(times(times(next_second, own2), b30));
                let fourth = lanes(times(times(next1, current), b30));
                let cell = m - 0x20 + 4 * o + 0x10 * m_index;
                assert_eq!(e.mem.f32(cell - 0x30), first, "a ({o}, {m_index})");
                assert_eq!(e.mem.f32(cell + 0x60), second, "b ({o}, {m_index})");
                assert_eq!(e.mem.f32(cell), third, "c ({o}, {m_index})");
                assert_eq!(e.mem.f32(cell + 0x30), fourth, "d ({o}, {m_index})");
            }
        }
        // The last link gets the zero block instead.
        let last = s.matrices + 0x3c0 + 0x350;
        for vector_index in 0..12u32 {
            assert_eq!(e.mem.f32(last - 0x50 + 0x10 * vector_index), 0.0);
        }
        let log = s.log.borrow();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0][0] & 0xff, 1);
        assert_eq!(log[0][1..3], [s.matrices, s.flags]);
        assert_eq!(log[1][0] & 0xff, 0);
        assert_eq!(log[1][1..3], [s.matrices + 0x3c0, s.flags + 0x40]);
        assert_eq!(log[1][3], log[0][3]);
        assert_eq!(log[1][4], log[0][4]);
    }

    #[test]
    fn no_links_set_nothing_up() {
        let mut s = setup(1, 1, true);
        run_setup(&mut s, 0);
        assert!(s.log.borrow().is_empty());
        assert_eq!(s.e.mem.f32(s.matrices + 0x350), 9.0);
        run_setup(&mut s, -2);
        assert!(s.log.borrow().is_empty());
    }
}
