//! `Havok SDK/run_00e61720` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `1e-7` as a `double` (`.rdata`).
const EPSILON_0109A3A0: u32 = 0x0109_a3a0;

// Translated from 00e63af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Thiscall, three words (`RET 0xc`). `this + 0` is a plane
/// normal (four floats) and `[this + 0x10]` points at a point on the plane;
/// `dist = (point - plane point) . normal` (SSE, `p2 + (p1 + p0)`) is
/// clamped into the interval `[*low, *high]` with the x87 compares of the
/// code (NaN included, the compares are ordered):
/// - if `*low < dist`: result 1, and `*high` becomes `dist` when
///   `dist < *high`;
/// - otherwise result 2, and `*low` becomes `dist + 1e-7` (a `double`
///   sum, rounded to `float`) when `*low < dist + 1e-7`;
/// - then the result is 4 whenever `*high <= *low`.
pub fn fn_00e63af0(e: &mut Engine, this: Ptr, point: Ptr, low: Ptr, high: Ptr) -> u32 {
    let plane_point = e.mem.u32(this.addr() + 0x10);
    let terms = [0u32, 4, 8].map(|offset| {
        (e.mem.f32(point.addr() + offset) - e.mem.f32(plane_point + offset))
            * e.mem.f32(this.addr() + offset)
    });
    let dist = terms[2] + (terms[1] + terms[0]);
    let result = if e.mem.f32(low.addr()) < dist {
        if dist < e.mem.f32(high.addr()) {
            e.mem.set_f32(high.addr(), dist);
        }
        1
    } else {
        let epsilon = e.mem.f64(EPSILON_0109A3A0);
        let widened = dist as f64 + epsilon;
        if (e.mem.f32(low.addr()) as f64) < widened {
            e.mem.set_f32(low.addr(), widened as f32);
        }
        2
    };
    if e.mem.f32(high.addr()) <= e.mem.f32(low.addr()) {
        4
    } else {
        result
    }
}

// The functions below are the other Havok routines of this range of the exe: the copy
// constructor and assignment of `hkxMeshSection` (the RTTI of the vtable at `010e9b7c`
// names the class; the engine map has no names for them) and triangle and quad geometry
// helpers whose names the map does not give either. They are described by what they do.

/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`),
/// both thiscall on the object.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The memory router (`00c85750`); its word at +0x10 is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl `(allocator, array, capacity,
/// elementSize)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The vtable of `hkxMeshSection` (`010e9b7c`).
const MESH_SECTION_VTABLE: u32 = 0x010e_9b7c;

use crate::units::havok_sdk::hkgeometryprocessing::{
    add4, compare_flags, load_vec4, movhlps, movlhps, mul4, shufps, store_vec4, sub4, sum3,
    unpckhps, unpcklps, Vec4,
};

/// The dot product of the first three lanes, built `z + (y + x)` as the exe does, with the
/// SSE NaN rules (`MULPS` with `a` as the destination, `ADDSS` with the higher lane).
fn dot3(a: Vec4, b: Vec4) -> f32 {
    sum3_sse(mul4_sse(a, b))
}

/// `SUBPS a, b` with the SSE NaN rules.
fn sub4_sse(a: Vec4, b: Vec4) -> Vec4 {
    crate::sse::subps(a, b)
}

/// `MULPS a, b` with the SSE NaN rules (`a` is the destination).
fn mul4_sse(a: Vec4, b: Vec4) -> Vec4 {
    crate::sse::mulps(a, b)
}

/// `z + (y + x)` with `ADDSS` (the higher lane the destination, the SSE NaN rules).
fn sum3_sse(v: Vec4) -> f32 {
    crate::sse::add(v[2], crate::sse::add(v[1], v[0]))
}

/// The x87 register stack, one `f64` per register (the code computes in extended
/// precision and stores `float`s: docs/ENGINE_CRATE.md). `ST0` is the last element. Every
/// method is one x87 instruction of the exe, named after it, with its register operands
/// as the exe writes them (`ST(i)`); arithmetic goes through the prelude helpers so a NaN
/// is chosen as the hardware chooses it, and the `_m` forms take their memory operand as
/// an `f64` (`x87_m32(bits)` for a `float` operand, the value itself for a `double`).
#[derive(Debug, Default)]
pub(crate) struct X87 {
    stack: Vec<f64>,
}

#[allow(dead_code)]
impl X87 {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// `ST(i)`.
    pub(crate) fn st(&self, i: usize) -> f64 {
        self.stack[self.stack.len() - 1 - i]
    }

    fn set(&mut self, i: usize, value: f64) {
        let at = self.stack.len() - 1 - i;
        self.stack[at] = value;
    }

    /// True when no register is in use.
    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Replaces `ST0` (a library call that takes `ST0` and answers in `ST0`).
    pub(crate) fn set_st0(&mut self, value: f64) {
        self.set(0, value);
    }

    /// `FLD m64` (or any value already in extended form).
    pub(crate) fn fld(&mut self, value: f64) {
        assert!(self.stack.len() < 8, "x87 stack overflow");
        self.stack.push(value);
    }

    /// `FLD m32` of the `float` with these bits (a signalling NaN is quieted).
    pub(crate) fn fld_m32(&mut self, bits: u32) {
        self.fld(x87_load32(bits));
    }

    /// `FLD ST(i)`.
    pub(crate) fn fld_st(&mut self, i: usize) {
        let value = self.st(i);
        self.fld(value);
    }

    pub(crate) fn fld1(&mut self) {
        self.fld(1.0);
    }

    pub(crate) fn fldz(&mut self) {
        self.fld(0.0);
    }

    /// `FILD m32`.
    pub(crate) fn fild(&mut self, value: i32) {
        self.fld(f64::from(value));
    }

    /// `FSTP ST0`.
    pub(crate) fn pop(&mut self) {
        self.stack.pop().expect("x87 stack underflow");
    }

    /// `FXCH ST(i)`.
    pub(crate) fn fxch(&mut self, i: usize) {
        let top = self.st(0);
        let other = self.st(i);
        self.set(0, other);
        self.set(i, top);
    }

    /// `FST m32`: the bits of the `float`.
    pub(crate) fn fst_m32(&self) -> u32 {
        x87_store32(self.st(0))
    }

    /// `FSTP m32`.
    pub(crate) fn fstp_m32(&mut self) -> u32 {
        let bits = self.fst_m32();
        self.pop();
        bits
    }

    /// `FSTP ST(i)`: copies `ST0` to `ST(i)` and pops.
    pub(crate) fn fstp_st(&mut self, i: usize) {
        let value = self.st(0);
        self.set(i, value);
        self.pop();
    }

    pub(crate) fn fchs(&mut self) {
        let value = self.st(0);
        self.set(0, -value);
    }

    /// `ST(dst) = ST(dst) + ST(src)`.
    pub(crate) fn fadd(&mut self, dst: usize, src: usize) {
        let value = x87_add(self.st(dst), self.st(src));
        self.set(dst, value);
    }

    /// `ST(dst) = ST(dst) - ST(src)`.
    pub(crate) fn fsub(&mut self, dst: usize, src: usize) {
        let value = x87_sub(self.st(dst), self.st(src));
        self.set(dst, value);
    }

    /// `ST(dst) = ST(src) - ST(dst)`.
    pub(crate) fn fsubr(&mut self, dst: usize, src: usize) {
        let value = x87_sub(self.st(src), self.st(dst));
        self.set(dst, value);
    }

    /// `ST(dst) = ST(dst) * ST(src)`.
    pub(crate) fn fmul(&mut self, dst: usize, src: usize) {
        let value = x87_mul(self.st(dst), self.st(src));
        self.set(dst, value);
    }

    /// `ST(dst) = ST(dst) / ST(src)`.
    pub(crate) fn fdiv(&mut self, dst: usize, src: usize) {
        let value = x87_div(self.st(dst), self.st(src));
        self.set(dst, value);
    }

    /// `ST(dst) = ST(src) / ST(dst)`.
    pub(crate) fn fdivr(&mut self, dst: usize, src: usize) {
        let value = x87_div(self.st(src), self.st(dst));
        self.set(dst, value);
    }

    /// `FADDP ST(i), ST0`.
    pub(crate) fn faddp(&mut self, i: usize) {
        self.fadd(i, 0);
        self.pop();
    }

    /// `FSUBP ST(i), ST0`: `ST(i) = ST(i) - ST0`.
    pub(crate) fn fsubp(&mut self, i: usize) {
        self.fsub(i, 0);
        self.pop();
    }

    /// `FSUBRP ST(i), ST0`: `ST(i) = ST0 - ST(i)`.
    pub(crate) fn fsubrp(&mut self, i: usize) {
        self.fsubr(i, 0);
        self.pop();
    }

    /// `FMULP ST(i), ST0`.
    pub(crate) fn fmulp(&mut self, i: usize) {
        self.fmul(i, 0);
        self.pop();
    }

    /// `FDIVP ST(i), ST0`: `ST(i) = ST(i) / ST0`.
    pub(crate) fn fdivp(&mut self, i: usize) {
        self.fdiv(i, 0);
        self.pop();
    }

    /// `FDIVRP ST(i), ST0`: `ST(i) = ST0 / ST(i)`.
    pub(crate) fn fdivrp(&mut self, i: usize) {
        self.fdivr(i, 0);
        self.pop();
    }

    /// `FADD m`: `ST0 = ST0 + operand`.
    pub(crate) fn fadd_m(&mut self, operand: f64) {
        let value = x87_add(self.st(0), operand);
        self.set(0, value);
    }

    /// `FSUB m`: `ST0 = ST0 - operand`.
    pub(crate) fn fsub_m(&mut self, operand: f64) {
        let value = x87_sub(self.st(0), operand);
        self.set(0, value);
    }

    /// `FSUBR m`: `ST0 = operand - ST0`.
    pub(crate) fn fsubr_m(&mut self, operand: f64) {
        let value = x87_sub(operand, self.st(0));
        self.set(0, value);
    }

    /// `FMUL m`: `ST0 = ST0 * operand`.
    pub(crate) fn fmul_m(&mut self, operand: f64) {
        let value = x87_mul(self.st(0), operand);
        self.set(0, value);
    }

    /// `FDIV m`: `ST0 = ST0 / operand`.
    pub(crate) fn fdiv_m(&mut self, operand: f64) {
        let value = x87_div(self.st(0), operand);
        self.set(0, value);
    }

    /// `FDIVR m`: `ST0 = operand / ST0`.
    pub(crate) fn fdivr_m(&mut self, operand: f64) {
        let value = x87_div(operand, self.st(0));
        self.set(0, value);
    }

    /// `FCOM ST(i)` (or `FUCOM`) and `FNSTSW AX`: the high byte of the status word.
    pub(crate) fn fcom(&self, i: usize) -> u8 {
        compare_flags(self.st(0), self.st(i))
    }

    /// `FCOM m` and `FNSTSW AX` (the caller pops for `FCOMP`).
    pub(crate) fn fcom_m(&self, operand: f64) -> u8 {
        compare_flags(self.st(0), operand)
    }

    /// `FCOMP ST(i)` and `FNSTSW AX`: compares and pops once.
    pub(crate) fn fcomp(&mut self, i: usize) -> u8 {
        let flags = self.fcom(i);
        self.pop();
        flags
    }

    /// `FCOMP m` and `FNSTSW AX`.
    pub(crate) fn fcomp_m(&mut self, operand: f64) -> u8 {
        let flags = self.fcom_m(operand);
        self.pop();
        flags
    }

    /// `FCOMPP` and `FNSTSW AX`: compares `ST0` with `ST1` and pops both.
    pub(crate) fn fcompp(&mut self) -> u8 {
        let flags = self.fcom(1);
        self.pop();
        self.pop();
        flags
    }

    /// `FABS`.
    pub(crate) fn fabs(&mut self) {
        let value = self.st(0).abs();
        self.set(0, value);
    }
}

/// Reads a reference-counted pointer slot's object and calls `addReference` on it when it
/// is not null.
fn add_reference(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
}

fn remove_reference(e: &mut Engine, object: u32) {
    if object != 0 {
        e.call(REMOVE_REFERENCE, &args![object]);
    }
}

/// Assigns the array of reference-counted pointers at `source_array` to the one at
/// `this_array` (data, size, capacity word with flags): reserves room when the capacity is
/// smaller than the source size, releases and clears the elements past the new size, copies
/// the common ones (each destination element released, each source element referenced),
/// copies the rest into the new slots, and sets the size. `skip_same` is the second
/// array's loop, which leaves an element alone when the two data pointers are one block.
fn assign_reference_array(e: &mut Engine, this_array: u32, source_array: u32, skip_same: bool) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let old_size = e.mem.i32(this_array + 4);
    let new_size = e.mem.i32(source_array + 4);
    let allocator = e.mem.u32(router + 0x10);
    let common = if new_size > old_size {
        old_size
    } else {
        new_size
    };
    let capacity = (e.mem.u32(this_array + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, this_array, wanted, 4u32]);
    }
    let data = e.mem.u32(this_array);
    let extra = old_size.wrapping_sub(new_size);
    let tail = data.wrapping_add((new_size as u32).wrapping_mul(4));
    for i in 0..extra.max(0) as u32 {
        let slot = tail.wrapping_add(i * 4);
        let element = e.mem.u32(slot);
        remove_reference(e, element);
        e.mem.set_u32(slot, 0);
    }
    let source_data = e.mem.u32(source_array);
    let this_data = e.mem.u32(this_array);
    for i in 0..common.max(0) as u32 {
        let destination = this_data.wrapping_add(i * 4);
        let source = source_data.wrapping_add(i * 4);
        if skip_same {
            if destination != source {
                let element = e.mem.u32(source);
                add_reference(e, element);
                let old = e.mem.u32(destination);
                remove_reference(e, old);
                e.mem.set_u32(destination, element);
            }
        } else {
            let element = e.mem.u32(source);
            add_reference(e, element);
            let old = e.mem.u32(destination);
            remove_reference(e, old);
            let element = e.mem.u32(source);
            e.mem.set_u32(destination, element);
        }
    }
    let source_data = e.mem.u32(source_array);
    let this_data = e.mem.u32(this_array);
    let skip = (common as u32).wrapping_mul(4);
    for i in 0..new_size.wrapping_sub(common).max(0) as u32 {
        let source = source_data.wrapping_add(skip).wrapping_add(i * 4);
        let destination = this_data.wrapping_add(skip).wrapping_add(i * 4);
        if destination != 0 {
            let element = e.mem.u32(source);
            add_reference(e, element);
            let element = e.mem.u32(source);
            e.mem.set_u32(destination, element);
        }
    }
    e.mem.set_i32(this_array + 4, new_size);
}

// Translated from 00e61770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assignment operator of `hkxMeshSection` (RTTI of the vtable at `010e9b7c`; no name in
/// the engine map), thiscall `(this, source)`, `RET 4`; returns `this`. Copies the two
/// words at +4 and +6 (size and flags, reference count), the reference-counted pointer at
/// +8 (the source's referenced, the old one released) and the array of reference-counted
/// pointers at +0xc (data, size at +0x10, capacity word at +0x14) with
/// `assign_reference_array`, then the pointer at +0x18 and the array at +0x1c (size +0x20,
/// capacity +0x24) the same way, the second array's copy loop skipping an element whose
/// destination is its source.
pub fn fn_00e61770(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let t = this.addr();
    let s = source.addr();
    let word = e.mem.u16(s + 4);
    e.mem.set_u16(t + 4, word);
    let word = e.mem.u16(s + 6);
    e.mem.set_u16(t + 6, word);
    for (pointer, array, skip_same) in [(8u32, 0xcu32, false), (0x18, 0x1c, true)] {
        let incoming = e.mem.u32(s + pointer);
        add_reference(e, incoming);
        let old = e.mem.u32(t + pointer);
        remove_reference(e, old);
        let incoming = e.mem.u32(s + pointer);
        e.mem.set_u32(t + pointer, incoming);
        assign_reference_array(e, t + array, s + array, skip_same);
    }
    this
}

// Translated from 00e61720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy constructor of `hkxMeshSection` (RTTI of the vtable at `010e9b7c`; no name in
/// the engine map), thiscall `(this, source)`, `RET 4`; returns `this`. Installs the vtable,
/// sets the word at +6 to 1 and both arrays (+0xc, +0x1c) empty and not owned (capacity
/// word 0x80000000), clears the two pointers (+8, +0x18), then runs the assignment
/// `fn_00e61770` with `source`.
pub fn fn_00e61720(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, MESH_SECTION_VTABLE);
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0xc, 0);
    e.mem.set_u32(t + 0x10, 0);
    e.mem.set_u32(t + 0x14, 0x8000_0000);
    e.mem.set_u32(t + 0x18, 0);
    e.mem.set_u32(t + 0x24, 0x8000_0000);
    e.mem.set_u32(t + 0x1c, 0);
    e.mem.set_u32(t + 0x20, 0);
    fn_00e61770(e, this, source)
}

// Translated from 00e61a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl `(point, a, base, c, out)`): the barycentric coordinates of the
/// point `point` in the triangle `(base, a, c)`, three floats at `out`, or its closest
/// point when it falls outside. Let `d1 = a - base`, `d2 = c - base`, `b = d1.d1`,
/// `cc = d2.d2`, `m = d1.d2` (dots built `z + (y + x)`), `det = float(float(b * cc) -
/// float(m * m))`. When `det > 0` (x87 compare, so not for NaN), with `f1 = (base - point).d1`
/// and `f2 = (base - point).d2` and `inv = float(1 / det)`:
/// `out = (X * inv, (det - X - Y) * inv, Y * inv)` with `X = float(f2 * m - cc * f1)` and
/// `Y = float(f1 * m - b * f2)`. Otherwise (a degenerate triangle) the point is projected on
/// one edge, chosen by `b < cc` and the squared length `e3` of `a - c`: `b >= cc` and
/// `e3 < b` gives `(t, 1 - t, 0)` with `t = ((point - base).d1) / b`; `b >= cc` and `e3 >=
/// b` gives `(t, 0, 1 - t)` with `t = ((point - c).(a - c)) / e3` and `(0, 0, 0)` when
/// `e3` is not positive; `b < cc` and `e3 < cc` gives `(0, 1 - t, t)` with `t = ((point -
/// base).d2) / cc`; `b < cc` and `e3 >= cc` gives `(t, 0, 1 - t)` with `t = ((point -
/// c).(a - c)) / e3`. The x87 stack code is followed instruction by instruction.
pub fn fn_00e61a10(e: &mut Engine, point: Ptr, a: Ptr, base: Ptr, c: Ptr, out: Ptr) {
    let a_vec = load_vec4(e, a.addr());
    let base_vec = load_vec4(e, base.addr());
    let c_vec = load_vec4(e, c.addr());
    let d1 = sub4_sse(a_vec, base_vec);
    let slot_8 = dot3(d1, d1).to_bits();
    let d2 = sub4_sse(c_vec, base_vec);
    let slot_c = dot3(d2, d2).to_bits();
    let slot_4 = dot3(d2, d1).to_bits();
    let o = out.addr();
    let mut x = X87::new();
    x.fld_m32(slot_4);
    x.fld_st(0);
    x.fld_m32(slot_8);
    x.fld_st(0);
    x.fld_m32(slot_c);
    x.fld_st(0);
    x.fmulp(2);
    x.fxch(1);
    let slot_c = x.fstp_m32();
    x.fld_m32(slot_c);
    x.fld_st(3);
    x.fmulp(4);
    x.fxch(3);
    let slot_c = x.fstp_m32();
    x.fld_m32(slot_c);
    x.fsubp(3);
    x.fxch(2);
    let slot_c = x.fstp_m32();
    x.fldz();
    x.fld_m32(slot_c);
    let flags = x.fcom(1);
    if flags & 0x41 == 0 {
        x.fstp_st(1);
        let point_vec = load_vec4(e, point.addr());
        x.fld_st(0);
        x.fld1();
        let shifted = sub4_sse(base_vec, point_vec);
        x.fdivrp(1);
        let f1 = dot3(shifted, d1);
        let f2 = dot3(shifted, d2);
        let slot_c = f2.to_bits();
        let slot_8 = f1.to_bits();
        let slot_4 = x.fstp_m32();
        x.fld_m32(slot_c);
        x.fld_st(0);
        x.fmul(0, 5);
        x.fld_m32(slot_8);
        x.fld_st(0);
        x.fmulp(6);
        x.fxch(1);
        x.fsubrp(5);
        x.fxch(4);
        let slot_c = x.fstp_m32();
        x.fxch(4);
        x.fmulp(3);
        x.fxch(3);
        x.fmulp(1);
        x.fsubp(1);
        let slot_8 = x.fstp_m32();
        x.fld_m32(slot_c);
        x.fld_st(0);
        x.fld_m32(slot_4);
        x.fld_st(0);
        x.fmulp(2);
        x.fxch(1);
        e.mem.set_u32(o, x.fstp_m32());
        x.fxch(2);
        x.fsubrp(1);
        x.fld_m32(slot_8);
        x.fld_st(0);
        x.fsubp(2);
        x.fld_st(2);
        x.fmulp(2);
        x.fxch(1);
        e.mem.set_u32(o + 4, x.fstp_m32());
        x.fmulp(1);
        e.mem.set_u32(o + 8, x.fstp_m32());
        debug_assert!(x.is_empty());
        return;
    }
    // A degenerate triangle: the stack holds [det, 0, b, cc, m]; keep [0, b, cc, det],
    // then [b, cc, 0].
    x.fstp_st(4);
    let d3 = sub4_sse(a_vec, c_vec);
    x.fstp_st(3);
    let e3 = dot3(d3, d3).to_bits();
    let flags = x.fcom(1);
    if flags & 0x01 == 0 {
        // b >= cc (or unordered).
        x.fstp_st(1);
        x.fld_m32(e3);
        let flags = x.fcom(1);
        if flags & 0x01 == 0 {
            x.fstp_st(1);
            x.fxch(1);
            e.mem.set_u32(o + 4, x.fst_m32());
            let flags = x.fcom(1);
            if (flags & 0x05).count_ones() % 2 == 1 {
                // 0 < e3.
                let c_vec = load_vec4(e, c.addr());
                x.pop();
                let point_vec = load_vec4(e, point.addr());
                let slot_c = dot3(sub4_sse(point_vec, c_vec), d3).to_bits();
                x.fdivr_m(x87_m32(slot_c));
                let slot_c = x.fstp_m32();
                x.fld_m32(slot_c);
                e.mem.set_u32(o, x.fst_m32());
                x.fld1();
                x.fsubrp(1);
                e.mem.set_u32(o + 8, x.fstp_m32());
            } else {
                x.fstp_st(1);
                e.mem.set_u32(o, x.fst_m32());
                e.mem.set_u32(o + 8, x.fstp_m32());
            }
        } else {
            // e3 < b.
            x.pop();
            let point_vec = load_vec4(e, point.addr());
            let slot_c = dot3(sub4_sse(point_vec, base_vec), d1).to_bits();
            x.fdivr_m(x87_m32(slot_c));
            let slot_c = x.fstp_m32();
            x.fld_m32(slot_c);
            e.mem.set_u32(o, x.fst_m32());
            x.fld1();
            x.fsubrp(1);
            e.mem.set_u32(o + 4, x.fstp_m32());
            e.mem.set_u32(o + 8, x.fstp_m32());
        }
    } else {
        // b < cc.
        x.pop();
        x.fld_m32(e3);
        let flags = x.fcom(1);
        if flags & 0x01 == 0 {
            x.fstp_st(1);
            let point_vec = load_vec4(e, point.addr());
            let slot_c = dot3(sub4_sse(point_vec, c_vec), d3).to_bits();
            x.fdivr_m(x87_m32(slot_c));
            let slot_c = x.fstp_m32();
            x.fld_m32(slot_c);
            e.mem.set_u32(o, x.fst_m32());
            x.fxch(1);
            e.mem.set_u32(o + 4, x.fstp_m32());
            x.fld1();
            x.fsubrp(1);
            e.mem.set_u32(o + 8, x.fstp_m32());
        } else {
            x.pop();
            let point_vec = load_vec4(e, point.addr());
            x.fxch(1);
            e.mem.set_u32(o, x.fstp_m32());
            let slot_c = dot3(sub4_sse(point_vec, base_vec), d2).to_bits();
            x.fdivr_m(x87_m32(slot_c));
            let slot_c = x.fstp_m32();
            x.fld_m32(slot_c);
            e.mem.set_u32(o + 8, x.fst_m32());
            x.fld1();
            x.fsubrp(1);
            e.mem.set_u32(o + 4, x.fstp_m32());
        }
    }
}

// Translated from 00e61d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl `(triangle, out)`): the reciprocal Gram matrix of a triangle and the
/// inverse length of its normal. `triangle` is three 16-byte points `p0`, `p1`, `p2`;
/// `d1 = p0 - p1`, `d2 = p2 - p1`; with `n11 = d1.d1`, `n22 = d2.d2`, `m = d2.d1` (dots
/// summed `z + (y + x)`), `det = float(float(n11 * n22) - float(m * m))` and `inv =
/// float(1 / det)`, `out` gets `(float(n11 * inv), float(n22 * inv), float(m * inv),
/// float(1 / sqrt(|d1 x d2|^2)))` (the length squared summed `z + (y + x)` too, `SQRTSS`,
/// then an x87 division). The x87 stack code is followed instruction by instruction.
pub fn fn_00e61d30(e: &mut Engine, triangle: Ptr, out: Ptr) {
    let t = triangle.addr();
    let p1 = load_vec4(e, t + 0x10);
    let p0 = load_vec4(e, t);
    let p2 = load_vec4(e, t + 0x20);
    let d1 = sub4_sse(p0, p1);
    let d2 = sub4_sse(p2, p1);
    let slot_8 = dot3(d1, d1).to_bits();
    let slot_c = dot3(d2, d2).to_bits();
    let slot_4 = dot3(d2, d1).to_bits();
    let o = out.addr();
    let mut x = X87::new();
    x.fld_m32(slot_4);
    x.fld_st(0);
    x.fld_m32(slot_8);
    x.fld_st(0);
    x.fld_m32(slot_c);
    x.fld_st(0);
    x.fmulp(2);
    x.fxch(1);
    let slot_c = x.fstp_m32();
    x.fld_m32(slot_c);
    x.fld_st(3);
    x.fmulp(4);
    x.fxch(3);
    let slot_c = x.fstp_m32();
    x.fld_m32(slot_c);
    x.fsubp(3);
    x.fxch(2);
    let slot_c = x.fstp_m32();
    x.fld_m32(slot_c);
    x.fld1();
    x.fld_st(0);
    x.fdivrp(2);
    x.fxch(1);
    let slot_c = x.fstp_m32();
    x.fld_m32(slot_c);
    x.fld_st(0);
    x.fmulp(3);
    x.fxch(2);
    e.mem.set_u32(o, x.fstp_m32());
    x.fld_st(1);
    x.fmulp(3);
    x.fxch(2);
    e.mem.set_u32(o + 4, x.fstp_m32());
    x.fmulp(2);
    x.fxch(1);
    e.mem.set_u32(o + 8, x.fstp_m32());
    // The normal d1 x d2 by the exe's shuffles: (d2.zxy * d1.yzx) - (d2.yzx * d1.zxy).
    let zxy = |v: Vec4| [v[2], v[0], v[1], v[3]];
    let yzx = |v: Vec4| [v[1], v[2], v[0], v[3]];
    let normal = sub4_sse(mul4_sse(zxy(d2), yzx(d1)), mul4_sse(yzx(d2), zxy(d1)));
    let squares = mul4_sse(normal, normal);
    let length_squared = sum3_sse(squares);
    let slot_c = crate::sse::sqrt(length_squared).to_bits();
    x.fdiv_m(x87_m32(slot_c));
    e.mem.set_u32(o + 0xc, x.fstp_m32());
    debug_assert!(x.is_empty());
}

use crate::x87::rsqrt_estimate;

/// The four-lane sign mask `0x80000000` (`010c44d0`) that `XORPS` negates with.
const SIGN_MASK: u32 = 0x010c_44d0;
/// The four-lane `3.0` (`010c71c0`) and `0.5` (`010c71d0`) of the refined `RSQRTPS`.
const THREES: u32 = 0x010c_71c0;
const HALVES: u32 = 0x010c_71d0;

/// `XORPS v, [mask]`: the lanes' bits exclusive-or the four words at `mask`.
fn xor_mask(e: &Engine, v: Vec4, mask: u32) -> Vec4 {
    let mut result = v;
    for (lane, value) in result.iter_mut().enumerate() {
        *value = f32::from_bits(value.to_bits() ^ e.mem.u32(mask + 4 * lane as u32));
    }
    result
}

/// `MAXPS dst, src`: each lane is `dst` when `dst > src`, otherwise `src` (so a NaN or two
/// equal zeros give `src`).
fn max_ps(dst: Vec4, src: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| if dst[i] > src[i] { dst[i] } else { src[i] })
}

/// `SHUFPS v, v, 0x55 * lane`: lane `lane` of `v` in every lane.
fn splat(v: Vec4, lane: usize) -> Vec4 {
    [v[lane]; 4]
}

/// `ADDSS dst, src`: lane 0 is the sum, the other lanes stay `dst`'s.
fn addss(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0] + src[0], dst[1], dst[2], dst[3]]
}

/// The matrix `this` holds (three rows of a 3 by 3 rotation at +0, +0x10, +0x20 and the
/// translation at +0x30) applied the way the exe builds it from lane shuffles: the columns
/// `[r0x, r1x, r2x, r2y]`, `[r0y, r1y, r2y, r2w]` and `[r0z, r1z, r2z, r2w]` are made with
/// `UNPCKLPS`, `UNPCKHPS`, `MOVLHPS`, `MOVHLPS` and `SHUFPS`, and `v` is multiplied as
/// `y * column_y + x * column_x + z * column_z`. Returns the sum without the translation.
fn rotate_by_columns(rows: [Vec4; 3], v: Vec4) -> Vec4 {
    let low = unpcklps(rows[0], rows[1]);
    let high = unpckhps(rows[0], rows[1]);
    let column_x = movlhps(low, rows[2]);
    let moved = movhlps(column_x, low);
    let column_y = shufps(moved, rows[2], 0xd4);
    let column_z = shufps(high, rows[2], 0xe4);
    let by_y = mul4(splat(v, 1), column_y);
    let by_x = mul4(splat(v, 0), column_x);
    let by_z = mul4(splat(v, 2), column_z);
    add4(add4(by_y, by_x), by_z)
}

// Translated from 00e62ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (register convention: the seed in `EAX`, the answer in `ST0`, and `EAX`
/// left as `seed * 0x3e39b193` by the `IMUL`): a linear congruential step.
/// `(0x3039 - seed * 0x3e39b193) & 0x7fffffff` converted exactly to a double, times the
/// double at `010229b8` (2^-31), stored as a `float`. In the uniform form the one word is
/// the seed; the result carries `eax` and `st0`.
pub fn fn_00e62ac0(e: &mut Engine, seed: u32) -> Ret {
    let scrambled = seed.wrapping_mul(0x3e39_b193);
    let value = (0x3039u32.wrapping_sub(scrambled) & 0x7fff_ffff) as i32;
    let factor = e.mem.f64(0x0102_29b8);
    let product = x87_mul(f64::from(value), factor);
    Ret {
        eax: scrambled,
        st0: x87_load32(x87_store32(product)),
        ..Ret::default()
    }
}

fn fn_00e62ac0_entry(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00e62ac0(e, a[0])
}

// Translated from 00e62c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`): appends three 16-byte vectors, the 48 bytes at `rows`,
/// to three parallel arrays of vectors: the vector at `rows + 0` goes to the array whose
/// pointer is at `this + 0x68`, `rows + 0x10` to the one at `this + 0x60`, `rows + 0x20`
/// to the one at `this + 0x64`, each at index `[this + 0x6c]`, which then grows by one.
pub fn fn_00e62c10(e: &mut Engine, this: Ptr, rows: Ptr) {
    let t = this.addr();
    let index = e.mem.u32(t + 0x6c);
    let offset = index << 4;
    let first = load_vec4(e, rows.addr());
    let array = e.mem.u32(t + 0x68);
    store_vec4(e, array.wrapping_add(offset), first);
    let second = load_vec4(e, rows.addr() + 0x10);
    let array = e.mem.u32(t + 0x60);
    store_vec4(e, array.wrapping_add(offset), second);
    let array = e.mem.u32(t + 0x64);
    let third = load_vec4(e, rows.addr() + 0x20);
    store_vec4(e, array.wrapping_add(offset), third);
    e.mem.set_u32(t + 0x6c, index.wrapping_add(1));
}

// Translated from 00e63ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0xc`: `(this, a, b, c)`): sets the simplex of the machine up
/// from three 64-byte blocks of four vectors each. `this + 8` becomes 4; block `a`'s
/// vectors go to `this + 0x30, 0x70, 0xb0, 0xf0`, block `b`'s to `this + 0x40, 0x80, 0xc0,
/// 0x100`, block `c`'s to `this + 0x20, 0x60, 0xa0, 0xe0`, and the words at `this + 0x50,
/// 0x90, 0xd0, 0x110` become 0; then `00e679d0` runs on `this` and `this + 0xc` becomes 0.
pub fn fn_00e63ba0(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr, c: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t + 8, 4);
    for row in 0..4u32 {
        let base = 0x40 * row;
        let vector = load_vec4(e, a.addr() + 0x10 * row);
        store_vec4(e, t + 0x30 + base, vector);
        let vector = load_vec4(e, b.addr() + 0x10 * row);
        store_vec4(e, t + 0x40 + base, vector);
        let vector = load_vec4(e, c.addr() + 0x10 * row);
        store_vec4(e, t + 0x20 + base, vector);
        e.mem.set_u32(t + 0x50 + base, 0);
    }
    e.call(0x00e6_79d0, &args![this]);
    e.mem.set_u32(t + 0xc, 0);
}

// Translated from 00e62b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 8`: `(this, direction, out)`): the support point of the
/// difference of two convex objects. `this` holds a transform (rows at +0, +0x10, +0x20,
/// translation +0x30) and the two objects at `this + 0x54` and `this + 0x58`, each with a
/// virtual function at vtable offset 4 that takes `(direction, out)`. The first object is
/// asked for the support point of `direction` itself, written at `out + 0x10`; the second
/// for the support point of `-direction` carried into its own frame by the transform's
/// columns (`rotate_by_columns`), kept in a local; `out + 0` becomes the first support
/// point minus the second one moved by the transform (`y * row1 + x * row0 + (z * row2 +
/// row3)`) and `out + 0x20` the second object's raw support point.
pub fn fn_00e62b20(e: &mut Engine, this: Ptr, direction: Ptr, out: Ptr) {
    let t = this.addr();
    let o = out.addr();
    let negated = xor_mask(e, load_vec4(e, direction.addr()), SIGN_MASK);
    let rows = [
        load_vec4(e, t),
        load_vec4(e, t + 0x10),
        load_vec4(e, t + 0x20),
    ];
    let first_object = e.mem.u32(t + 0x54);
    let local = rotate_by_columns(rows, negated);
    e.with_stack(0x24, |e, frame| {
        let f = frame.addr();
        store_vec4(e, f + 4, local);
        e.vcall(first_object, 4, &args![direction, o + 0x10]);
        let second_object = e.mem.u32(t + 0x58);
        e.vcall(second_object, 4, &args![f + 4, f + 0x14]);
        let support = load_vec4(e, f + 0x14);
        let row1 = load_vec4(e, t + 0x10);
        let row0 = load_vec4(e, t);
        let by_y = mul4(splat(support, 1), row1);
        let by_x = mul4(splat(support, 0), row0);
        let row2 = load_vec4(e, t + 0x20);
        let sum = add4(by_y, by_x);
        let by_z = mul4(splat(support, 2), row2);
        let row3 = load_vec4(e, t + 0x30);
        let moved = add4(sum, add4(by_z, row3));
        let first = load_vec4(e, o + 0x10);
        store_vec4(e, o, sub4(first, moved));
        store_vec4(e, o + 0x20, support);
    });
}

// Translated from 00e62c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x10`: `(this, best, direction, origin, vertex)`): asks
/// `fn_00e62b20` for the support points of `direction` (into a local record of three vectors
/// and a word) and of `-direction` (into `vertex`), then compares how far each lies along its
/// own direction from `origin`: `d1 = (local.vector0 - origin) . direction` and `d2 =
/// (vertex.vector0 - origin) . -direction`, each summed `z + (y + x)`. `best` gets the
/// four-lane `MAXPS` of the two registers (lane 0 the larger of `d1` and `d2`, the other
/// lanes the exe's leftovers) and when `d2 - d1` has its sign bit set (`d2 < d1`) the local
/// record (the vectors at +0, +0x10, +0x20 and the word at +0x30, the last never written by
/// the call: the frame's own contents) is copied over `vertex`.
pub fn fn_00e62c60(e: &mut Engine, this: Ptr, best: Ptr, direction: Ptr, origin: Ptr, vertex: Ptr) {
    e.with_stack(0x54, |e, frame| {
        let f = frame.addr();
        let negated = xor_mask(e, load_vec4(e, direction.addr()), SIGN_MASK);
        store_vec4(e, f + 4, negated);
        fn_00e62b20(e, this, direction, Ptr::new(f + 0x14));
        fn_00e62b20(e, this, Ptr::new(f + 4), vertex);
        let origin_vec = load_vec4(e, origin.addr());
        let near = load_vec4(e, f + 0x14);
        let direction_vec = load_vec4(e, direction.addr());
        let product = mul4(sub4(near, origin_vec), direction_vec);
        let low = addss(splat(product, 1), product);
        let high = splat(product, 2);
        let far = load_vec4(e, vertex.addr());
        let negated_vec = load_vec4(e, f + 4);
        let product = mul4(sub4(far, origin_vec), negated_vec);
        let low_far = addss(splat(product, 1), product);
        let d2 = addss(splat(product, 2), low_far);
        let d1 = addss(high, low);
        let larger = max_ps(d1, d2);
        let difference = d2[0] - d1[0];
        store_vec4(e, best.addr(), larger);
        if difference.to_bits() >> 31 != 0 {
            let supporting = load_vec4(e, f + 0x24);
            let word = e.mem.u32(f + 0x44);
            store_vec4(e, vertex.addr() + 0x10, supporting);
            store_vec4(e, vertex.addr(), near);
            let second = load_vec4(e, f + 0x34);
            store_vec4(e, vertex.addr() + 0x20, second);
            e.mem.set_u32(vertex.addr() + 0x30, word);
        }
    });
}

// Translated from 00e625c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. Register convention: `ECX` the first convex object, `ESI` a transform
/// (rows +0, +0x10, +0x20, translation +0x30), `EDI` a direction vector, and two stack
/// words (`RET` is plain: the caller cleans them), the second convex object and the record
/// `out`; the uniform form takes the words `(ecx, esi, edi, second, out)`. Both objects have a
/// virtual function at vtable offset 4 that takes `(direction, out)`. The first is asked for
/// the support point of `-direction` (into a local), the second for the support point of
/// `direction` carried into its frame by the transform's columns (`rotate_by_columns`,
/// into a second local). With `t = y * row1 + x * row0 + (z * row2 + row3)` of the second
/// point and `w = first - t`, `dist = (w . direction)` summed `z + (y + x)`: when `out + 0x3c
/// - dist` has its sign bit set (so `dist > out.w` or a NaN with the sign), `out` gets the
/// first point at +0, `t` at +0x10, the second point at +0x20, and at +0x30 the vector
/// `(direction.x, direction.y, direction.z, dist)` (the y, z and `dist` through the x87
/// stack, which quiets a signalling NaN).
pub fn fn_00e625c0(
    e: &mut Engine,
    first: Ptr,
    transform: Ptr,
    direction: Ptr,
    second: Ptr,
    out: Ptr,
) {
    let s = transform.addr();
    let dir = load_vec4(e, direction.addr());
    let rows = [
        load_vec4(e, s),
        load_vec4(e, s + 0x10),
        load_vec4(e, s + 0x20),
    ];
    let negated = xor_mask(e, dir, SIGN_MASK);
    let rotated = rotate_by_columns(rows, dir);
    e.with_stack(0x4c, |e, frame| {
        let f = frame.addr();
        store_vec4(e, f + 0xc, negated);
        store_vec4(e, f + 0x1c, rotated);
        e.vcall(first.addr(), 4, &args![f + 0xc, f + 0x3c]);
        e.vcall(second.addr(), 4, &args![f + 0x1c, f + 0x2c]);
        let second_point = load_vec4(e, f + 0x2c);
        let row1 = load_vec4(e, s + 0x10);
        let row0 = load_vec4(e, s);
        let first_point = load_vec4(e, f + 0x3c);
        let dir = load_vec4(e, direction.addr());
        let by_y = mul4(splat(second_point, 1), row1);
        let by_x = mul4(splat(second_point, 0), row0);
        let row2 = load_vec4(e, s + 0x20);
        let sum = add4(by_y, by_x);
        let by_z = mul4(splat(second_point, 2), row2);
        let row3 = load_vec4(e, s + 0x30);
        let moved = add4(sum, add4(by_z, row3));
        let o = out.addr();
        let product = mul4(sub4(first_point, moved), dir);
        let low = addss(splat(product, 1), product);
        let distance = splat(product, 2)[0] + low[0];
        let limit = e.mem.f32(o + 0x3c);
        if (limit - distance).to_bits() >> 31 != 0 {
            let quiet = |value: f32| f32::from_bits(x87_quiet_bits(value.to_bits()));
            store_vec4(e, o, first_point);
            store_vec4(e, o + 0x20, second_point);
            store_vec4(e, o + 0x10, moved);
            let record = [dir[0], quiet(dir[1]), quiet(dir[2]), quiet(distance)];
            store_vec4(e, o + 0x30, record);
        }
    });
}

// Translated from 00e61e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl `(triangle, out)`): inverse lengths of a triangle's edges and of its
/// normal. `triangle` is three 16-byte points `p0`, `p1`, `p2`; with `a = p0 - p2`, `b = p2 -
/// p1`, `c = p1 - p0`, `n = a.zxy * b.yzx - a.yzx * b.zxy` the squared lengths (summed
/// `z + (y + x)`) of `b`, `a`, `c` and `n` are put through `RSQRTPS` and one Newton step
/// `(3 - x r^2) * (0.5 r)` (constants `010c71c0`, `010c71d0`), giving `out[0..4]` = the
/// inverse lengths of `b`, `a`, `c`, `n` (each stored through the x87 stack, which quiets a
/// signalling NaN) and `out[4] = float(|n|^2 * out[3])`.
pub fn fn_00e61e70(e: &mut Engine, triangle: Ptr, out: Ptr) {
    let t = triangle.addr();
    let p2 = load_vec4(e, t + 0x20);
    let p0 = load_vec4(e, t);
    let p1 = load_vec4(e, t + 0x10);
    let a = sub4_sse(p0, p2);
    let b = sub4_sse(p2, p1);
    let c = sub4_sse(p1, p0);
    let first = mul4_sse(shufps(a, a, 0xc9), shufps(b, b, 0xd2));
    let second = mul4_sse(shufps(a, a, 0xd2), shufps(b, b, 0xc9));
    let normal = sub4_sse(second, first);
    let squares = [
        sum3_sse(mul4_sse(b, b)),
        sum3_sse(mul4_sse(a, a)),
        sum3_sse(mul4_sse(c, c)),
        sum3_sse(mul4_sse(normal, normal)),
    ];
    let slots = squares.map(|value| x87_quiet_bits(value.to_bits()));
    let lengths = slots.map(f32::from_bits);
    let threes = load_vec4(e, THREES);
    let halves = load_vec4(e, HALVES);
    let estimate = lengths.map(rsqrt_estimate);
    let times = mul4_sse(mul4_sse(estimate, lengths), estimate);
    let refined = mul4_sse(sub4_sse(threes, times), mul4_sse(halves, estimate));
    let o = out.addr();
    for (lane, value) in refined.iter().enumerate() {
        e.mem
            .set_u32(o + 4 * lane as u32, x87_quiet_bits(value.to_bits()));
    }
    let product = x87_mul(x87_load32(slots[3]), x87_load32(refined[3].to_bits()));
    e.mem.set_u32(o + 0x10, x87_store32(product));
}

// Translated from 00e61fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl `(triangle, factors, matrix, normal)`): builds a 4 by 4 matrix and a
/// normal from a triangle. `triangle` is three 16-byte points `p0`, `p1`, `p2`; `factors` is
/// five floats `s0..s4`. With `a = p0 - p2`, `b = p2 - p1`, `c = p1 - p0` and `x.yzx` meaning
/// the lanes `(y, z, x, w)`: the normal `N = (a.zxy * b.yzx - a.yzx * b.zxy) * s3` is stored
/// at `normal`; the rows `r0 = cross(s0 * b, N)`, `r1 = cross(s1 * a, N)`, `r2 = cross(s2 * c,
/// N)` (each `u.yzx * N.zxy - u.zxy * N.yzx`) go to `matrix + 0, 0x10, 0x20`; then the
/// 3 by 3 part is transposed (`r0[1]` with `r1[0]`, `r0[2]` with `r2[0]`, `r1[2]` with
/// `r2[1]`, each through the x87 stack, quieting signalling NaNs), column 3 of the rows
/// becomes `N.x, N.y, N.z` (read back from `normal`), row 3 becomes zero and then its first
/// element `float(s4 * -s0)`.
pub fn fn_00e61fb0(e: &mut Engine, triangle: Ptr, factors: Ptr, matrix: Ptr, normal: Ptr) {
    let t = triangle.addr();
    let f = factors.addr();
    let m = matrix.addr();
    let n = normal.addr();
    let p2 = load_vec4(e, t + 0x20);
    let p1 = load_vec4(e, t + 0x10);
    let p0 = load_vec4(e, t);
    let b = sub4_sse(p2, p1);
    let c = sub4_sse(p1, p0);
    let a = sub4_sse(p0, p2);
    let yzx = |v: Vec4| shufps(v, v, 0xc9);
    let zxy = |v: Vec4| shufps(v, v, 0xd2);
    let cross_of = |u: Vec4, v: Vec4| sub4_sse(mul4_sse(zxy(u), yzx(v)), mul4_sse(yzx(u), zxy(v)));
    let normal_vec = mul4_sse(cross_of(a, b), [e.mem.f32(f + 0xc); 4]);
    store_vec4(e, n, normal_vec);
    let scaled_b = mul4_sse([e.mem.f32(f); 4], b);
    let scaled_a = mul4_sse([e.mem.f32(f + 4); 4], a);
    let scaled_c = mul4_sse([e.mem.f32(f + 8); 4], c);
    let row = |u: Vec4| {
        sub4_sse(
            mul4_sse(yzx(u), zxy(normal_vec)),
            mul4_sse(zxy(u), yzx(normal_vec)),
        )
    };
    store_vec4(e, m, row(scaled_b));
    store_vec4(e, m + 0x10, row(scaled_a));
    store_vec4(e, m + 0x20, row(scaled_c));
    for (first, second) in [(0x10u32, 4u32), (0x20, 8), (0x24, 0x18)] {
        let held = x87_quiet_bits(e.mem.u32(m + first));
        let other = x87_quiet_bits(e.mem.u32(m + second));
        e.mem.set_u32(m + first, other);
        e.mem.set_u32(m + second, x87_quiet_bits(held));
    }
    for (column, source) in [(0xcu32, 0u32), (0x1c, 4), (0x2c, 8)] {
        let bits = x87_quiet_bits(e.mem.u32(n + source));
        e.mem.set_u32(m + column, bits);
    }
    store_vec4(e, m + 0x30, [0.0; 4]);
    let scale4 = x87_load32(e.mem.u32(f + 0x10));
    let negated_first = -x87_load32(e.mem.u32(f));
    e.mem
        .set_u32(m + 0x30, x87_store32(x87_mul(scale4, negated_first)));
}

/// `hkLineSegmentUtil::closestPointLineSeg` (`00d230e0`, cdecl `(point, a, b, out)`): the
/// closest point of the segment to the point goes to `out`; the answer says which end it is
/// (4 and 8 the ends, anything else the inside).
const CLOSEST_POINT_LINE_SEG: u32 = 0x00d2_30e0;
/// Square root of ST0 (`00ec6040`).
const X87_SQRT: u32 = 0x00ec_6040;
/// The four thresholds the three barycentric tests compare against (`01267e30`), the doubles
/// `0.001` (`01016978`) and `1.001` (`010cb448`) added to them and the `float` in the fourth
/// lane (`01017d00`).
const THRESHOLDS: u32 = 0x0126_7e30;
const SHIFT_SMALL_DOUBLE: u32 = 0x0101_6978;
const SHIFT_ONE_DOUBLE: u32 = 0x010c_b448;
const FOURTH_LANE_FLOAT: u32 = 0x0101_7d00;
/// The tables of signed bytes: the feature for each outcome of the three tests (`010ececc`),
/// and the vertex neighbours / edge ends (`010eced4`, `010eced6`, `010ecedc`).
const OUTCOME_TABLE: u32 = 0x010e_cecc;
const NEIGHBOUR_TABLE_NEXT: u32 = 0x010e_ced6;
const NEIGHBOUR_TABLE_PREVIOUS: u32 = 0x010e_ced4;
const EDGE_TABLE_FIRST: u32 = 0x010e_cedc;
/// `float` zero (`01011d78`), 3.0 (`01017718`) and 0.5 (`01016248`) of the refined `RSQRTSS`.
const ZERO_FLOAT: u32 = 0x0101_1d78;
const THREE_FLOAT: u32 = 0x0101_7718;
const HALF_FLOAT: u32 = 0x0101_6248;

fn signed_table_byte(e: &Engine, base: u32, index: i32) -> i32 {
    i32::from(e.mem.i8(base.wrapping_add(index as u32)))
}

/// What `fn_00e62100` writes into the feature record at `info` (a byte count, then 16-bit
/// vertex indexes at +2, +4 and +6) when it is not null.
fn write_feature(e: &mut Engine, info: u32, count: u8, first: i32, second: i32) {
    if info == 0 {
        return;
    }
    e.mem.set_u16(info + 2, first as u16);
    if count == 2 {
        e.mem.set_u16(info + 4, second as u16);
    }
    e.mem.set_u8(info, count);
}

/// Records which end or edge a `closestPointLineSeg` result (4, 8 or other) stands for.
fn record_segment_feature(e: &mut Engine, info: u32, result: u32, first: i32, second: i32) {
    if info == 0 {
        return;
    }
    if result == 4 {
        write_feature(e, info, 1, first, 0);
    } else if result == 8 {
        write_feature(e, info, 1, second, 0);
    } else {
        write_feature(e, info, 2, first, second);
    }
}

/// The normalized `point - closest` and its length, written to `out` (the refined `RSQRTSS`
/// with a zero squared length giving a zero factor), for the edge case.
fn finish_from_difference(e: &mut Engine, point: u32, closest: Vec4, out: u32) {
    let difference = sub4(load_vec4(e, point), closest);
    let squared = mul4(difference, difference);
    let length = sum3(squared);
    let zero = e.mem.f32(ZERO_FLOAT);
    let three = e.mem.f32(THREE_FLOAT);
    let half = e.mem.f32(HALF_FLOAT);
    let estimate = rsqrt_estimate(length);
    let refined = (half * estimate) * (three - (length * estimate) * estimate);
    let factor = if length == zero { 0.0 } else { refined };
    store_vec4(e, out, mul4(difference, [factor; 4]));
    let distance = sum3(squared) * factor;
    e.mem
        .set_u32(out + 0x10, x87_quiet_bits(distance.to_bits()));
}

/// `1 / float(sqrt(x))` of a `float` through the CRT's square root and the x87 stack, as the
/// two vertex paths of `fn_00e62100` make it.
fn inverse_root(e: &mut Engine, bits: u32) -> u32 {
    let root = x87_store32(e.call(X87_SQRT, &args![x87_load32(bits)]).f64());
    x87_store32(x87_div(1.0, x87_load32(root)))
}

// Translated from 00e62100 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl `(point, triangle, plane, out, info)`): the closest feature of a
/// triangle to a point. `triangle` is three 16-byte points; `plane` (read at +0, +4, +8 and
/// +0xc) is the triangle's normal with a scale. `info`, when not null, receives the feature
/// found as a byte count (`3` all three vertices, `2` an edge, `1` a vertex) followed by 16-bit
/// vertex indexes (+2, +4, +6). With `v = p1 - point`, `e0 = p0 - p1`, `e1 = p2 - p1`, `da =
/// e0.v`, `db = e1.v` (dots summed `z + (y + x)`) the exe forms the barycentric-like numbers
/// `A = pz * db - py * da`, `B = pz * da - px * db` through the x87 stack and the vector
/// `(A + 0.001, 1.001 - A - B, B + 0.001, k)`; each of its first three lanes is compared with
/// the thresholds at `01267e30` (`CMPLTPS`, `MOVMSKPS`). When all three pass, the point is
/// over the face: `out` gets `plane.w * (e1.zxy * e0.yzx - e1.yzx * e0.zxy)` (negated when
/// `(v . that) > 0`, else the distance word is negated instead) and the squared distance
/// `v . that` at `out + 0x10`, `info` says `3, 0, 1, 2`, and the answer is 0. Otherwise the
/// byte table at `010ececc` turns the outcome of the tests into a feature: a negative entry
/// is an edge, whose closest point `00d230e0` finds, and `out` gets the normalized vector
/// from it to the point and the distance; a non-negative entry is a vertex, for which the two
/// edges to its neighbours are tried and the nearer one used. The answer is 1.
pub fn fn_00e62100(
    e: &mut Engine,
    point: Ptr,
    triangle: Ptr,
    plane: Ptr,
    out: Ptr,
    info: Ptr,
) -> u32 {
    let info = info.addr();
    if info != 0 {
        e.mem.set_u8(info, 0);
    }
    let t = triangle.addr();
    let p = plane.addr();
    let o = out.addr();
    let p1 = load_vec4(e, t + 0x10);
    let point_vec = load_vec4(e, point.addr());
    let mut x = X87::new();
    x.fld_m32(e.mem.u32(p + 8));
    let v = sub4(p1, point_vec);
    let e0 = sub4(load_vec4(e, t), p1);
    let slot_10 = sum3(mul4(e0, v)).to_bits();
    let e1 = sub4(load_vec4(e, t + 0x20), p1);
    let slot_14 = sum3(mul4(e1, v)).to_bits();
    x.fld_m32(slot_14);
    x.fld_st(0);
    x.fmulp(2);
    x.fld_m32(e.mem.u32(p + 4));
    x.fld_m32(slot_10);
    x.fld_st(0);
    x.fmulp(2);
    x.fxch(3);
    x.fsubrp(1);
    let slot_14 = x.fstp_m32();
    x.fld_m32(e.mem.u32(p + 8));
    x.fmulp(2);
    x.fmul_m(x87_m32(e.mem.u32(p)));
    x.fsubp(1);
    let slot_10 = x.fstp_m32();
    x.fld_m32(slot_14);
    x.fld_st(0);
    x.fld(e.mem.f64(SHIFT_SMALL_DOUBLE));
    x.fadd(1, 0);
    x.fxch(1);
    let lane_0 = x.fstp_m32();
    x.fld(e.mem.f64(SHIFT_ONE_DOUBLE));
    x.fsubrp(2);
    x.fld_m32(slot_10);
    x.fld_st(0);
    x.fsubp(3);
    x.fxch(2);
    let lane_1 = x.fstp_m32();
    x.faddp(1);
    let lane_2 = x.fstp_m32();
    let lane_3 = x87_quiet_bits(e.mem.u32(FOURTH_LANE_FLOAT));
    debug_assert!(x.is_empty());
    let lanes = [lane_0, lane_1, lane_2, lane_3].map(f32::from_bits);
    let thresholds = load_vec4(e, THRESHOLDS);
    let mask = (0..3).fold(0u32, |mask, lane| {
        mask | (u32::from(thresholds[lane] < lanes[lane]) << lane)
    });
    if mask == 7 {
        let yzx = |v: Vec4| shufps(v, v, 0xc9);
        let zxy = |v: Vec4| shufps(v, v, 0xd2);
        let normal = sub4(mul4(zxy(e1), yzx(e0)), mul4(yzx(e1), zxy(e0)));
        store_vec4(e, o, normal);
        let scaled = mul4([e.mem.f32(p + 0xc); 4], normal);
        let distance = sum3(mul4(v, scaled)).to_bits();
        let mut x = X87::new();
        x.fld_m32(distance);
        e.mem.set_u32(o + 0x10, x.fst_m32());
        store_vec4(e, o, scaled);
        x.fldz();
        let flags = x.fcom(1);
        x.pop();
        if (flags & 0x05).count_ones() % 2 == 1 {
            x.pop();
            let negated = [0, 1, 2, 3].map(|lane| {
                f32::from_bits(scaled[lane].to_bits() ^ e.mem.u32(SIGN_MASK + 4 * lane as u32))
            });
            store_vec4(e, o, negated);
        } else {
            x.fchs();
            e.mem.set_u32(o + 0x10, x.fstp_m32());
        }
        if info != 0 {
            e.mem.set_u8(info, 3);
            e.mem.set_u16(info + 2, 0);
            e.mem.set_u16(info + 4, 1);
            e.mem.set_u16(info + 6, 2);
        }
        return 0;
    }
    let feature = signed_table_byte(e, OUTCOME_TABLE, mask as i32);
    if feature < 0 {
        let first = signed_table_byte(e, EDGE_TABLE_FIRST, feature);
        let second = signed_table_byte(e, EDGE_TABLE_FIRST + 2, feature);
        let result = e.with_stack(0x10, |e, closest| {
            let result = e.call(
                CLOSEST_POINT_LINE_SEG,
                &args![
                    point,
                    Ptr::<()>::new(t.wrapping_add((second as u32) << 4)),
                    Ptr::<()>::new(t.wrapping_add((first as u32) << 4)),
                    closest
                ],
            );
            let closest_vec = load_vec4(e, closest.addr());
            (result.u32(), closest_vec)
        });
        record_segment_feature(e, info, result.0, first, second);
        finish_from_difference(e, point.addr(), result.1, o);
        return 1;
    }
    // A vertex: the closest point of each of its two edges.
    let vertex = t.wrapping_add((feature as u32) << 4);
    let next = signed_table_byte(e, NEIGHBOUR_TABLE_NEXT, feature);
    let previous = signed_table_byte(e, NEIGHBOUR_TABLE_PREVIOUS, feature);
    let (first_result, second_result, first_closest, second_closest) =
        e.with_stack(0x20, |e, frame| {
            let first_out = frame.addr();
            let second_out = frame.addr() + 0x10;
            let first_result = e
                .call(
                    CLOSEST_POINT_LINE_SEG,
                    &args![
                        point,
                        Ptr::<()>::new(vertex),
                        Ptr::<()>::new(t.wrapping_add((next as u32) << 4)),
                        Ptr::<()>::new(first_out)
                    ],
                )
                .u32();
            let second_result = e
                .call(
                    CLOSEST_POINT_LINE_SEG,
                    &args![
                        point,
                        Ptr::<()>::new(t.wrapping_add((previous as u32) << 4)),
                        Ptr::<()>::new(vertex),
                        Ptr::<()>::new(second_out)
                    ],
                )
                .u32();
            (
                first_result,
                second_result,
                load_vec4(e, first_out),
                load_vec4(e, second_out),
            )
        });
    let point_vec = load_vec4(e, point.addr());
    let first_difference = sub4(point_vec, first_closest);
    let second_difference = sub4(point_vec, second_closest);
    let first_length = sum3(mul4(first_difference, first_difference)).to_bits();
    let second_length = sum3(mul4(second_difference, second_difference)).to_bits();
    let mut x = X87::new();
    x.fld_m32(first_length);
    x.fld_m32(second_length);
    let flags = x.fcom(1);
    if flags & 0x41 == 0 {
        // The second edge is farther: the first one is the answer.
        let inverse = inverse_root(e, first_length);
        if info != 0 {
            if first_result == 4 {
                write_feature(e, info, 1, feature, 0);
            } else if first_result == 8 {
                write_feature(e, info, 1, next, 0);
            } else {
                write_feature(e, info, 2, feature, next);
            }
        }
        let distance = x87_mul(x87_load32(first_length), x87_m32(inverse));
        e.mem.set_u32(o + 0x10, x87_store32(distance));
        store_vec4(e, o, mul4([f32::from_bits(inverse); 4], first_difference));
    } else {
        let inverse = inverse_root(e, second_length);
        if info != 0 {
            if second_result == 4 {
                write_feature(e, info, 1, feature, 0);
            } else if second_result == 8 {
                write_feature(e, info, 1, previous, 0);
            } else {
                write_feature(e, info, 2, feature, previous);
            }
        }
        let distance = x87_mul(x87_load32(second_length), x87_m32(inverse));
        e.mem.set_u32(o + 0x10, x87_store32(distance));
        store_vec4(e, o, mul4([f32::from_bits(inverse); 4], second_difference));
    }
    1
}

/// The three 16-byte directions the search starts from (`01268370`, `+0x10`, `+0x20`).
const START_DIRECTIONS: u32 = 0x0126_8370;
/// The `float` the best distance starts at (`0104e148`, about -3.4e38).
const WORST_DISTANCE: u32 = 0x0104_e148;
/// The factor the step is scaled by on each round (`0106d508`, a `double`, 0.99) and
/// the one it is halved by (`01011588`, 0.5).
const STEP_SCALE_DOUBLE: u32 = 0x0106_d508;
const HALF_DOUBLE: u32 = 0x0101_1588;
/// The step below which the search stops (`01017d00`, a `float`).
const MINIMUM_STEP: u32 = 0x0101_7d00;

/// A vector refined as the exe's normalizer does for the direction search: `RSQRTSS`, then
/// `(0.5 r) * (3 - (x r) r)`; a squared length equal to the zero scalar gives 0.
fn refined_inverse(length: f32, zero: f32, three: f32, half: f32) -> f32 {
    let estimate = rsqrt_estimate(length);
    let refined = (half * estimate) * (three - (length * estimate) * estimate);
    if length == zero {
        0.0
    } else {
        refined
    }
}

// Translated from 00e62740 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl `(first, second, transform, out0, out1, out2)`, six words, plain
/// `RET`): the direction of deepest penetration of two convex objects, found by repeated
/// calls of `fn_00e625c0` (the support search of two objects under a transform) on a local
/// record of four vectors (the first point, its transform, the second point, and the best
/// direction with its distance in the last lane, `-3.4e38` at the start).
///
/// First every direction of the three the global array holds (`01268370` and the next two,
/// 16 bytes each) is negated and offered twice to `fn_00e625c0`. Then, with the best
/// direction `d` and a step `u` starting at `float(1.0 * 0.99)`, rounds follow: the smaller
/// of the three absolute components is left out and the other two make a vector `p` with
/// `p[a] = d[b]`, `p[b] = -d[a]` (perpendicular to `d`), normalized to `n` (the refined
/// `RSQRTSS`; a zero length gives a zero vector); with `q = u * n` and `c = u * (d x n)`
/// (`d.yzx * n.zxy - d.zxy * n.yzx`), the four vectors `d + q`, `d - q`, `d + c`, `d - c`
/// are each scaled by one factor, the normalizing factor of `d + q` (the others have the same
/// length when `d` is a unit vector), and offered to `fn_00e625c0` in the order `d + q`,
/// `d - q`, `d + c`, `d - c`. The step is then halved (through a `float`) and the rounds go on
/// while it stays above the `float` at `01017d00`; a round multiplies the carried value by
/// 0.99 to make the step it starts with. Finally `out0` gets the record's first point,
/// `out1` its second point, `out2` the best direction with distance at +0 and the first
/// point at +0x10.
///
/// The record lives in the translation's frame (the exe's stack, whose unwritten words read
/// as zero here, as `fn_00e625c0`'s record words do).
pub fn fn_00e62740(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    transform: Ptr,
    out0: Ptr,
    out1: Ptr,
    out2: Ptr,
) {
    e.with_stack(0x100, |e, frame| {
        let f = frame.addr();
        let record = Ptr::new(f + 0x40);
        let worst = x87_quiet_bits(e.mem.u32(WORST_DISTANCE));
        e.mem.set_u32(f + 0x7c, worst);
        for step in 0..3u32 {
            let source = START_DIRECTIONS + 0x10 * step;
            for word in 0..4 {
                let bits = e.mem.u32(source + 4 * word);
                e.mem.set_u32(f + 0x20 + 4 * word, bits);
            }
            let direction = load_vec4(e, f + 0x20);
            let negated = xor_mask(e, direction, SIGN_MASK);
            store_vec4(e, f + 0x20, negated);
            for _ in 0..2 {
                fn_00e625c0(e, first, transform, Ptr::new(f + 0x20), second, record);
            }
        }
        let zero = e.mem.f32(ZERO_FLOAT);
        let three = e.mem.f32(THREE_FLOAT);
        let half = e.mem.f32(HALF_FLOAT);
        store_vec4(e, f + 0xf0, [0.0; 4]);
        store_vec4(e, f + 0xe0, [zero, 0.0, 0.0, 0.0]);
        store_vec4(e, f + 0x90, [three, 0.0, 0.0, 0.0]);
        store_vec4(e, f + 0x80, [half, 0.0, 0.0, 0.0]);
        let mut carried = 1.0f64;
        loop {
            let step_bits = x87_store32(x87_mul(carried, e.mem.f64(STEP_SCALE_DOUBLE)));
            e.mem.set_u32(f + 0x18, step_bits);
            // The two axes the perpendicular is made from: the smaller absolute component of
            // the best direction is left out.
            let component = |e: &Engine, lane: u32| {
                x87_store32(x87_load32(e.mem.u32(f + 0x70 + 4 * lane)).abs())
            };
            let abs_x = component(e, 0);
            let abs_y = component(e, 1);
            let abs_z = component(e, 2);
            let (mut ecx, mut edi, mut edx) = (1usize, 0usize, 2usize);
            let mut smallest = abs_x;
            if compare_flags(x87_load32(abs_x), x87_load32(abs_y)) & 0x41 == 0 {
                smallest = abs_y;
                ecx = 0;
                edi = edx - 1;
            }
            if compare_flags(x87_load32(smallest), x87_load32(abs_z)) & 0x41 == 0 {
                edx = edi;
            }
            store_vec4(e, f + 0x20, [0.0; 4]);
            let kept = x87_quiet_bits(e.mem.u32(f + 0x70 + 4 * edx as u32));
            e.mem.set_u32(f + 0x20 + 4 * ecx as u32, kept);
            let swapped = -x87_load32(e.mem.u32(f + 0x70 + 4 * ecx as u32));
            e.mem
                .set_u32(f + 0x20 + 4 * edx as u32, x87_store32(swapped));
            let perpendicular = load_vec4(e, f + 0x20);
            let length = sum3(mul4(perpendicular, perpendicular));
            let zero_vector = load_vec4(e, f + 0xe0);
            let three_vector = load_vec4(e, f + 0x90);
            let half_vector = load_vec4(e, f + 0x80);
            let inverse = refined_inverse(length, zero_vector[0], three_vector[0], half_vector[0]);
            let unit = mul4([inverse; 4], perpendicular);
            let step = e.mem.f32(f + 0x18);
            let q = mul4([step; 4], unit);
            let unit_zxy = shufps(unit, unit, 0xd2);
            let unit_yzx = shufps(unit, unit, 0xc9);
            let d = load_vec4(e, f + 0x70);
            let across = mul4(shufps(d, d, 0xd2), unit_yzx);
            let along = mul4(shufps(d, d, 0xc9), unit_zxy);
            let c = mul4(sub4(along, across), [step; 4]);
            let d_plus_q = add4(d, q);
            let long_inverse = refined_inverse(
                sum3(mul4(d_plus_q, d_plus_q)),
                zero_vector[0],
                load_vec4(e, f + 0x90)[0],
                load_vec4(e, f + 0x80)[0],
            );
            let factor = [long_inverse; 4];
            store_vec4(e, f + 0xc0, mul4(factor, d_plus_q));
            store_vec4(e, f + 0xa0, mul4(sub4(d, q), factor));
            store_vec4(e, f + 0xb0, mul4(add4(c, d), factor));
            store_vec4(e, f + 0xd0, mul4(sub4(d, c), factor));
            for offset in [0xc0u32, 0xa0, 0xb0, 0xd0] {
                fn_00e625c0(e, first, transform, Ptr::new(f + offset), second, record);
            }
            let halved = x87_store32(x87_mul(
                x87_load32(e.mem.u32(f + 0x18)),
                e.mem.f64(HALF_DOUBLE),
            ));
            e.mem.set_u32(f + 0x18, halved);
            let limit = x87_load32(e.mem.u32(MINIMUM_STEP));
            let flags = compare_flags(x87_load32(halved), limit);
            carried = x87_load32(halved);
            if flags & 0x41 != 0 {
                break;
            }
        }
        let point = load_vec4(e, f + 0x40);
        let second_point = load_vec4(e, f + 0x60);
        store_vec4(e, out0.addr(), point);
        store_vec4(e, out1.addr(), second_point);
        let best = load_vec4(e, f + 0x70);
        store_vec4(e, out2.addr(), best);
        store_vec4(e, out2.addr() + 0x10, point);
    });
}

/// The four-lane `0xffffffff, 0xffffffff, 0xffffffff, 0` mask (`010c4c10`), the absolute-value
/// mask (`01017d10`), the vector of `2^-23` (`010c7200`) and the one of `1.0` (`010ec7f0`).
const XYZ_MASK: u32 = 0x010c_4c10;
const ABS_MASK_VECTOR: u32 = 0x0101_7d10;
const EPSILON_VECTOR: u32 = 0x010c_7200;
const ONES_VECTOR: u32 = 0x010e_c7f0;
/// The vector the first direction falls back to (`01268380`) and the origin the barycentric
/// coordinates are taken of (`01267e30`).
const FALLBACK_DIRECTION: u32 = 0x0126_8380;
const ORIGIN: u32 = 0x0126_7e30;
const THREE_SCALAR: u32 = 0x0101_7718;
const HALF_SCALAR: u32 = 0x0101_6248;
const ZERO_SCALAR: u32 = 0x0101_1d78;

/// `MOVSS xmm, [mem]`: the float in lane 0, the other lanes zero.
fn movss_load(e: &Engine, at: u32) -> Vec4 {
    [e.mem.f32(at), 0.0, 0.0, 0.0]
}

fn subss(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0] - src[0], dst[1], dst[2], dst[3]]
}

fn mulss(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0] * src[0], dst[1], dst[2], dst[3]]
}

fn divss(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0] / src[0], dst[1], dst[2], dst[3]]
}

/// `CMPEQSS dst, src`: lane 0 becomes all ones when equal, else zero; the others stay.
fn cmpeqss(dst: Vec4, src: Vec4) -> Vec4 {
    let mask = if dst[0] == src[0] { u32::MAX } else { 0 };
    [f32::from_bits(mask), dst[1], dst[2], dst[3]]
}

/// `ANDNPS dst, src`: `!dst & src` on the bits of every lane.
fn andnps(dst: Vec4, src: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(!dst[i].to_bits() & src[i].to_bits()))
}

/// `ANDPS dst, src`.
fn andps(dst: Vec4, src: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(dst[i].to_bits() & src[i].to_bits()))
}

/// `RSQRTSS dst, dst`: lane 0 becomes the hardware estimate, the others stay.
fn rsqrtss(v: Vec4) -> Vec4 {
    [rsqrt_estimate(v[0]), v[1], v[2], v[3]]
}

/// `SQRTSS dst, dst`.
fn sqrtss(v: Vec4) -> Vec4 {
    [v[0].sqrt(), v[1], v[2], v[3]]
}

/// `UNPCKLPS`-free helper: lane 0 of `v` stored as a dword.
fn lane0_bits(v: Vec4) -> u32 {
    v[0].to_bits()
}

/// `MOVSS [mem], xmm`.
fn movss_store(e: &mut Engine, at: u32, v: Vec4) {
    e.mem.set_u32(at, lane0_bits(v));
}

/// `JP` after `TEST AH, mask`: taken when the masked byte has an even number of bits.
fn parity_even(flags: u8, mask: u8) -> bool {
    (flags & mask).count_ones() % 2 == 0
}

// Translated from 00e62d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 8`: `(this, output, code)`; the answer 0 or 1 in `EAX`): one
/// step of the simplex search of the distance routine, the object being the machine
/// `00e63ba0` sets up. `this + 0x6c` is the number of simplex points (0 to 4), `this + 0x68`
/// the array of the points (16-byte vectors), `this + 0x60` and `this + 0x64` the arrays of the
/// supporting points of the two objects, `this + 0x50` a threshold, `this + 0x30` a vector the
/// origin has been moved by, `this + 0x40` four thresholds. `output` receives the closest
/// feature (a normal at +0, the point at +0x10) and `code` one of 0 (a feature was found),
/// 5 (the search went wrong: more than three restarts of the same size, or a size outside
/// 0 to 4) or 0 with an answer of 1 (a point found).
///
/// The function is the exe's state machine: for each size it computes the squared extent of
/// the simplex (a vector length, the normal of a triangle, the volume of a tetrahedron) and
/// compares it with the threshold; when it is degenerate the size drops (`this + 0x6c`),
/// when not a new point is asked from `fn_00e62b20` (support points of a direction through
/// the two objects), the better of the two supports (`fn_00e62c60` and its comparison) is
/// appended to the simplex with `fn_00e62c10`, and the size grows. `EBX` (kept here as
/// `visited`) remembers the largest size already tried so the search cannot loop; hitting
/// the sizes again gives answer 1 with `code` 5. Written as a transcription of the exe's
/// basic blocks (`pc` is the address of a block), with the SSE registers as `xmm[0..8]`, the
/// x87 stack as `x`, and the stack frame as a block of memory (`f` is the exe's `ESP` after
/// the prologue), since the code leaves vectors in frame slots that later blocks read.
#[allow(unused_assignments)]
pub fn fn_00e62d30(e: &mut Engine, this: Ptr, output: Ptr, code: Ptr) -> u32 {
    let t = this.addr();
    let out = output.addr();
    let code_ptr = code.addr();
    e.with_stack(0x180, |e, frame| {
        let f = frame.addr();
        let mut esi = t;
        let mut visited: i32 = -1;
        let mut xmm: [Vec4; 8] = [[0.0; 4]; 8];
        let mut x = X87::new();
        let (mut edi, mut ecx, mut edx) = (0u32, 0u32, 0u32);
        let state = e.mem.u32(esi + 0x6c);
        if state > 4 {
            e.mem.set_u32(code_ptr, 5);
            return 1;
        }
        let mut pc: u32 = match state {
            0 => 0x00e6_2f25,
            1 => 0x00e6_2edd,
            2 => 0x00e6_2e8d,
            3 => 0x00e6_2e0f,
            _ => 0x00e6_2d57,
        };
        loop {
            match pc {
                // Four points: the volume of the tetrahedron against the threshold.
                0x00e6_2d57 => {
                    let eax = e.mem.u32(esi + 0x68);
                    xmm[1] = load_vec4(e, eax + 0x10);
                    xmm[0] = load_vec4(e, eax);
                    xmm[2] = load_vec4(e, eax + 0x20);
                    xmm[0] = sub4(xmm[0], xmm[1]);
                    xmm[1] = sub4(xmm[1], xmm[2]);
                    xmm[2] = shufps(xmm[1], xmm[1], 0xc9);
                    xmm[3] = shufps(xmm[0], xmm[0], 0xd2);
                    xmm[2] = mul4(xmm[2], xmm[3]);
                    xmm[0] = shufps(xmm[0], xmm[0], 0xc9);
                    xmm[1] = shufps(xmm[1], xmm[1], 0xd2);
                    xmm[1] = mul4(xmm[1], xmm[0]);
                    xmm[1] = sub4(xmm[1], xmm[2]);
                    xmm[0] = mul4(xmm[1], xmm[1]);
                    xmm[2] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[3] = xmm[0];
                    store_vec4(e, f + 0x80, xmm[1]);
                    xmm[3] = shufps(xmm[3], xmm[0], 0xaa);
                    xmm[0] = load_vec4(e, eax + 0x30);
                    xmm[3] = addss(xmm[3], xmm[2]);
                    xmm[2] = load_vec4(e, eax);
                    xmm[0] = sub4(xmm[0], xmm[2]);
                    xmm[0] = mul4(xmm[0], xmm[1]);
                    xmm[1] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[2] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[2] = addss(xmm[2], xmm[1]);
                    movss_store(e, f + 0x6c, xmm[2]);
                    x.fld_m32(e.mem.u32(f + 0x6c));
                    x.fmul(0, 0);
                    movss_store(e, f + 0x78, xmm[3]);
                    x.fld_m32(e.mem.u32(esi + 0x50));
                    x.fmul_m(x87_m32(e.mem.u32(f + 0x78)));
                    let product = x.fstp_m32();
                    e.mem.set_u32(f + 0x18, product);
                    x.fld_m32(e.mem.u32(f + 0x18));
                    let flags = x.fcompp();
                    if flags & 0x01 != 0 {
                        pc = 0x00e6_3723;
                    } else if visited >= 3 {
                        pc = 0x00e6_370c;
                    } else {
                        e.mem.set_u32(esi + 0x6c, 3);
                        pc = 0x00e6_2e0f;
                    }
                }
                // Three points: the squared length of the triangle's normal.
                0x00e6_2e0f => {
                    let eax = e.mem.u32(esi + 0x68);
                    x.fldz();
                    xmm[0] = load_vec4(e, eax + 0x10);
                    xmm[1] = load_vec4(e, eax);
                    xmm[2] = load_vec4(e, eax + 0x20);
                    xmm[1] = sub4(xmm[1], xmm[0]);
                    xmm[0] = sub4(xmm[0], xmm[2]);
                    xmm[2] = shufps(xmm[0], xmm[0], 0xc9);
                    xmm[3] = shufps(xmm[1], xmm[1], 0xd2);
                    xmm[2] = mul4(xmm[2], xmm[3]);
                    xmm[1] = shufps(xmm[1], xmm[1], 0xc9);
                    xmm[0] = shufps(xmm[0], xmm[0], 0xd2);
                    xmm[0] = mul4(xmm[0], xmm[1]);
                    xmm[0] = sub4(xmm[0], xmm[2]);
                    xmm[1] = mul4(xmm[0], xmm[0]);
                    xmm[2] = addss(shufps(xmm[1], xmm[1], 0x55), xmm[1]);
                    xmm[3] = shufps(xmm[1], xmm[1], 0xaa);
                    xmm[3] = addss(xmm[3], xmm[2]);
                    movss_store(e, f + 0x4c, xmm[3]);
                    let flags = x.fcomp_m(x87_m32(e.mem.u32(f + 0x4c)));
                    store_vec4(e, f + 0x80, xmm[0]);
                    if parity_even(flags, 0x44) {
                        pc = 0x00e6_32db;
                    } else {
                        edx = 2;
                        if visited >= edx as i32 {
                            pc = 0x00e6_34d6;
                        } else {
                            e.mem.set_u32(esi + 0x6c, edx);
                            pc = 0x00e6_2e8d;
                        }
                    }
                }
                // Two points: the squared length of the segment.
                0x00e6_2e8d => {
                    let eax = e.mem.u32(esi + 0x68);
                    xmm[1] = load_vec4(e, eax + 0x10);
                    xmm[0] = load_vec4(e, eax);
                    xmm[0] = sub4(xmm[0], xmm[1]);
                    xmm[0] = mul4(xmm[0], xmm[0]);
                    xmm[1] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[2] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[2] = addss(xmm[2], xmm[1]);
                    movss_store(e, f + 0x64, xmm[2]);
                    x.fld_m32(e.mem.u32(f + 0x64));
                    x.fld_m32(e.mem.u32(esi + 0x50));
                    let flags = x.fcompp();
                    if flags & 0x01 != 0 {
                        pc = 0x00e6_30b5;
                    } else if visited >= 1 {
                        pc = 0x00e6_34c4;
                    } else {
                        e.mem.set_u32(esi + 0x6c, 1);
                        pc = 0x00e6_2edd;
                    }
                }
                // One point: its squared length.
                0x00e6_2edd => {
                    let eax = e.mem.u32(esi + 0x68);
                    xmm[0] = load_vec4(e, eax);
                    xmm[0] = mul4(xmm[0], xmm[0]);
                    xmm[1] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[2] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[2] = addss(xmm[2], xmm[1]);
                    movss_store(e, f + 0x74, xmm[2]);
                    x.fld_m32(e.mem.u32(f + 0x74));
                    x.fld_m32(e.mem.u32(esi + 0x50));
                    let flags = x.fcompp();
                    if flags & 0x01 != 0 {
                        pc = 0x00e6_2fda;
                    } else if visited >= 0 {
                        pc = 0x00e6_34ad;
                    } else {
                        e.mem.set_u32(esi + 0x6c, 0);
                        pc = 0x00e6_2f25;
                    }
                }
                // No point: start from the negated vector at +0x30 (or the fallback).
                0x00e6_2f25 => {
                    xmm[0] = load_vec4(e, esi + 0x30);
                    xmm[0] = mul4(xmm[0], xmm[0]);
                    xmm[1] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[2] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[0] = load_vec4(e, esi + 0x30);
                    xmm[2] = addss(xmm[2], xmm[1]);
                    xmm[1] = load_vec4(e, SIGN_MASK);
                    movss_store(e, f + 0x7c, xmm[2]);
                    x.fld_m32(e.mem.u32(f + 0x7c));
                    x.fld_m32(e.mem.u32(esi + 0x50));
                    xmm[0] = xorps(xmm[0], xmm[1]);
                    let flags = x.fcompp();
                    store_vec4(e, f + 0x50, xmm[0]);
                    if flags & 0x01 == 0 {
                        xmm[0] = load_vec4(e, FALLBACK_DIRECTION);
                        store_vec4(e, f + 0x50, xmm[0]);
                    }
                    fn_00e62b20(e, Ptr::new(esi), Ptr::new(f + 0x50), Ptr::new(f + 0x90));
                    fn_00e62c10(e, Ptr::new(esi), Ptr::new(f + 0x90));
                    let eax = e.mem.u32(esi + 0x68);
                    xmm[0] = load_vec4(e, eax);
                    xmm[0] = mul4(xmm[0], xmm[0]);
                    xmm[1] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[2] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[2] = addss(xmm[2], xmm[1]);
                    movss_store(e, f + 0x70, xmm[2]);
                    x.fld_m32(e.mem.u32(f + 0x70));
                    x.fld_m32(e.mem.u32(esi + 0x50));
                    let flags = x.fcompp();
                    if flags & 0x01 == 0 {
                        pc = 0x00e6_33a2;
                    } else {
                        visited = 0;
                        pc = 0x00e6_2edd;
                    }
                }
                // One point, not at the origin: the support point of its direction.
                0x00e6_2fda => {
                    edi = e.mem.u32(esi + 0x68);
                    visited = 1;
                    fn_00e62b20(e, Ptr::new(esi), Ptr::new(edi), Ptr::new(f + 0x130));
                    ecx = e.mem.u32(esi + 0x68);
                    xmm[1] = load_vec4(e, ecx);
                    xmm[0] = load_vec4(e, f + 0x130);
                    xmm[0] = sub4(xmm[0], xmm[1]);
                    xmm[0] = mul4(xmm[0], xmm[0]);
                    xmm[1] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[2] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[0] = load_vec4(e, edi);
                    xmm[2] = addss(xmm[2], xmm[1]);
                    xmm[1] = load_vec4(e, SIGN_MASK);
                    xmm[0] = xorps(xmm[0], xmm[1]);
                    movss_store(e, f + 0x68, xmm[2]);
                    store_vec4(e, f + 0x100, xmm[0]);
                    fn_00e62b20(e, Ptr::new(esi), Ptr::new(f + 0x100), Ptr::new(f + 0x90));
                    x.fld_m32(e.mem.u32(f + 0x68));
                    ecx = e.mem.u32(esi + 0x68);
                    xmm[1] = load_vec4(e, ecx);
                    xmm[0] = load_vec4(e, f + 0x90);
                    xmm[0] = sub4(xmm[0], xmm[1]);
                    xmm[0] = mul4(xmm[0], xmm[0]);
                    xmm[1] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[2] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[2] = addss(xmm[2], xmm[1]);
                    movss_store(e, f + 0x1c, xmm[2]);
                    x.fld_m32(e.mem.u32(f + 0x1c));
                    let flags = x.fcompp();
                    if parity_even(flags, 0x41) {
                        fn_00e62c10(e, Ptr::new(esi), Ptr::new(f + 0x90));
                    } else {
                        fn_00e62c10(e, Ptr::new(esi), Ptr::new(f + 0x130));
                    }
                    pc = 0x00e6_2e8d;
                }
                // Two points, a segment: the perpendicular directions to search.
                0x00e6_30b5 => {
                    ecx = e.mem.u32(esi + 0x68);
                    xmm[1] = load_vec4(e, ecx + 0x10);
                    xmm[0] = load_vec4(e, ecx);
                    xmm[0] = sub4(xmm[0], xmm[1]);
                    store_vec4(e, f + 0x30, xmm[0]);
                    x.fld_m32(e.mem.u32(f + 0x30));
                    x.fabs();
                    visited = 2;
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x10, slot);
                    e.mem.set_u32(f + 0x18, 0);
                    x.fld_m32(e.mem.u32(f + 0x10));
                    edx = (visited - 1) as u32;
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x10, slot);
                    edi = visited as u32;
                    x.fld_m32(e.mem.u32(f + 0x34));
                    x.fabs();
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x14, slot);
                    x.fld_m32(e.mem.u32(f + 0x14));
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0xc, slot);
                    x.fld_m32(e.mem.u32(f + 0x38));
                    x.fabs();
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x14, slot);
                    x.fld_m32(e.mem.u32(f + 0x14));
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x14, slot);
                    x.fld_m32(e.mem.u32(f + 0xc));
                    x.fld_m32(e.mem.u32(f + 0x10));
                    let flags = x.fcomp(1);
                    if flags & 0x41 == 0 {
                        let slot = x.fstp_m32();
                        e.mem.set_u32(f + 0x10, slot);
                        edx = 0;
                        e.mem.set_u32(f + 0x18, 1);
                    } else {
                        x.pop();
                    }
                    x.fld_m32(e.mem.u32(f + 0x14));
                    x.fld_m32(e.mem.u32(f + 0x10));
                    let flags = x.fcompp();
                    if flags & 0x41 == 0 {
                        edi = e.mem.u32(f + 0x18);
                    }
                    x.fld_m32(e.mem.u32(f + 0x30 + 4 * edi));
                    xmm[7] = movss_load(e, ZERO_SCALAR);
                    xmm[5] = movss_load(e, THREE_SCALAR);
                    xmm[1] = [0.0; 4];
                    store_vec4(e, f + 0x20, xmm[1]);
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x20 + 4 * edx, slot);
                    x.fld_m32(e.mem.u32(f + 0x30 + 4 * edx));
                    xmm[1] = shufps(xmm[0], xmm[0], 0xd2);
                    x.fchs();
                    xmm[0] = shufps(xmm[0], xmm[0], 0xc9);
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x20 + 4 * edi, slot);
                    xmm[2] = load_vec4(e, f + 0x20);
                    xmm[3] = shufps(xmm[2], xmm[2], 0xc9);
                    xmm[3] = mul4(xmm[3], xmm[1]);
                    xmm[1] = shufps(xmm[2], xmm[2], 0xd2);
                    xmm[1] = mul4(xmm[1], xmm[0]);
                    xmm[1] = sub4(xmm[1], xmm[3]);
                    xmm[0] = mul4(xmm[2], xmm[2]);
                    xmm[3] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[0] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[0] = addss(xmm[0], xmm[3]);
                    xmm[3] = xmm[0];
                    xmm[4] = xmm[0];
                    xmm[3] = rsqrtss(xmm[3]);
                    xmm[6] = xmm[3];
                    xmm[3] = movss_load(e, HALF_SCALAR);
                    xmm[0] = mulss(xmm[0], xmm[6]);
                    xmm[0] = mulss(xmm[0], xmm[6]);
                    xmm[4] = cmpeqss(xmm[4], xmm[7]);
                    store_vec4(e, f + 0x110, xmm[7]);
                    xmm[7] = xmm[5];
                    xmm[7] = subss(xmm[7], xmm[0]);
                    xmm[0] = xmm[3];
                    xmm[0] = mulss(xmm[0], xmm[6]);
                    xmm[0] = mulss(xmm[0], xmm[7]);
                    xmm[4] = andnps(xmm[4], xmm[0]);
                    xmm[0] = xmm[4];
                    xmm[0] = shufps(xmm[0], xmm[4], 0);
                    xmm[0] = mul4(xmm[0], xmm[2]);
                    store_vec4(e, f + 0x20, xmm[0]);
                    xmm[0] = xmm[1];
                    xmm[0] = mul4(xmm[0], xmm[1]);
                    xmm[2] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[0] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[0] = addss(xmm[0], xmm[2]);
                    xmm[2] = xmm[0];
                    xmm[2] = rsqrtss(xmm[2]);
                    xmm[4] = xmm[2];
                    xmm[2] = xmm[0];
                    let zero_slot = movss_load(e, f + 0x110);
                    xmm[2] = cmpeqss(xmm[2], zero_slot);
                    xmm[0] = mulss(xmm[0], xmm[4]);
                    xmm[0] = mulss(xmm[0], xmm[4]);
                    xmm[5] = subss(xmm[5], xmm[0]);
                    xmm[3] = mulss(xmm[3], xmm[4]);
                    xmm[3] = mulss(xmm[3], xmm[5]);
                    xmm[2] = andnps(xmm[2], xmm[3]);
                    xmm[0] = xmm[2];
                    xmm[0] = shufps(xmm[0], xmm[2], 0);
                    xmm[0] = mul4(xmm[0], xmm[1]);
                    store_vec4(e, f + 0x120, xmm[0]);
                    fn_00e62c60(
                        e,
                        Ptr::new(esi),
                        Ptr::new(f + 0xe0),
                        Ptr::new(f + 0x20),
                        Ptr::new(ecx),
                        Ptr::new(f + 0x90),
                    );
                    let eax = e.mem.u32(esi + 0x68);
                    fn_00e62c60(
                        e,
                        Ptr::new(esi),
                        Ptr::new(f + 0xd0),
                        Ptr::new(f + 0x120),
                        Ptr::new(eax),
                        Ptr::new(f + 0x130),
                    );
                    xmm[0] = load_vec4(e, f + 0xd0);
                    xmm[1] = load_vec4(e, f + 0xe0);
                    xmm[0] = subss(xmm[0], xmm[1]);
                    if xmm[0][0].to_bits() >> 31 != 0 {
                        fn_00e62c10(e, Ptr::new(esi), Ptr::new(f + 0x90));
                    } else {
                        fn_00e62c10(e, Ptr::new(esi), Ptr::new(f + 0x130));
                    }
                    pc = 0x00e6_2e0f;
                }
                // Three points, a real triangle: the normal and the support point along it.
                0x00e6_32db => {
                    ecx = e.mem.u32(esi + 0x68);
                    xmm[1] = mul4(xmm[0], xmm[0]);
                    xmm[2] = addss(shufps(xmm[1], xmm[1], 0x55), xmm[1]);
                    xmm[1] = shufps(xmm[1], xmm[1], 0xaa);
                    xmm[1] = addss(xmm[1], xmm[2]);
                    xmm[2] = xmm[1];
                    xmm[2] = rsqrtss(xmm[2]);
                    xmm[3] = xmm[2];
                    xmm[2] = movss_load(e, ZERO_SCALAR);
                    xmm[4] = xmm[2];
                    xmm[2] = xmm[1];
                    xmm[1] = mulss(xmm[1], xmm[3]);
                    xmm[1] = mulss(xmm[1], xmm[3]);
                    xmm[2] = cmpeqss(xmm[2], xmm[4]);
                    xmm[4] = movss_load(e, THREE_SCALAR);
                    xmm[4] = subss(xmm[4], xmm[1]);
                    xmm[1] = movss_load(e, HALF_SCALAR);
                    xmm[1] = mulss(xmm[1], xmm[3]);
                    xmm[1] = mulss(xmm[1], xmm[4]);
                    xmm[2] = andnps(xmm[2], xmm[1]);
                    xmm[1] = xmm[2];
                    xmm[1] = shufps(xmm[1], xmm[2], 0);
                    xmm[1] = mul4(xmm[1], xmm[0]);
                    visited = 3;
                    store_vec4(e, f + 0x80, xmm[1]);
                    fn_00e62c60(
                        e,
                        Ptr::new(esi),
                        Ptr::new(f + 0xf0),
                        Ptr::new(f + 0x80),
                        Ptr::new(ecx),
                        Ptr::new(f + 0x90),
                    );
                    xmm[0] = load_vec4(e, esi + 0x40);
                    xmm[1] = load_vec4(e, f + 0xf0);
                    xmm[1] = subss(xmm[1], xmm[0]);
                    if xmm[1][0].to_bits() >> 31 != 0 {
                        pc = 0x00e6_365f;
                    } else {
                        fn_00e62c10(e, Ptr::new(esi), Ptr::new(f + 0x90));
                        pc = 0x00e6_2d57;
                    }
                }
                // The origin is inside the triangle: the normal and the closest point.
                0x00e6_365f => {
                    xmm[1] = load_vec4(e, XYZ_MASK);
                    xmm[0] = load_vec4(e, f + 0x80);
                    xmm[0] = andps(xmm[0], xmm[1]);
                    store_vec4(e, out, xmm[0]);
                    let eax = e.mem.u32(esi + 0x68);
                    fn_00e61a10(
                        e,
                        Ptr::new(ORIGIN),
                        Ptr::new(eax),
                        Ptr::new(eax + 0x10),
                        Ptr::new(eax + 0x20),
                        Ptr::new(f + 0x30),
                    );
                    x.fld_m32(e.mem.u32(f + 0x30));
                    esi = e.mem.u32(esi + 0x60);
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0xc, slot);
                    xmm[0] = movss_load(e, f + 0xc);
                    x.fld_m32(e.mem.u32(f + 0x34));
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0xc, slot);
                    xmm[1] = movss_load(e, f + 0xc);
                    x.fld_m32(e.mem.u32(f + 0x38));
                    xmm[3] = xmm[1];
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0xc, slot);
                    xmm[2] = movss_load(e, f + 0xc);
                    xmm[3] = shufps(xmm[3], xmm[1], 0);
                    xmm[1] = load_vec4(e, esi + 0x10);
                    xmm[3] = mul4(xmm[3], xmm[1]);
                    xmm[1] = xmm[0];
                    xmm[1] = shufps(xmm[1], xmm[0], 0);
                    xmm[0] = load_vec4(e, esi);
                    xmm[1] = mul4(xmm[1], xmm[0]);
                    xmm[3] = add4(xmm[3], xmm[1]);
                    xmm[1] = load_vec4(e, esi + 0x20);
                    xmm[0] = xmm[2];
                    xmm[0] = shufps(xmm[0], xmm[2], 0);
                    xmm[0] = mul4(xmm[0], xmm[1]);
                    xmm[3] = add4(xmm[3], xmm[0]);
                    store_vec4(e, out + 0x10, xmm[3]);
                    e.mem.set_u32(code_ptr, 0);
                    return 1;
                }
                // The origin is the closest point (the simplex is the point at the origin).
                0x00e6_33a2 => {
                    esi = e.mem.u32(esi + 0x68);
                    xmm[1] = load_vec4(e, esi);
                    xmm[0] = load_vec4(e, f + 0xa0);
                    xmm[4] = load_vec4(e, f + 0x50);
                    store_vec4(e, out + 0x10, xmm[0]);
                    xmm[0] = xmm[4];
                    xmm[0] = mul4(xmm[0], xmm[4]);
                    xmm[2] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[0] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[0] = addss(xmm[0], xmm[2]);
                    xmm[2] = xmm[0];
                    xmm[2] = rsqrtss(xmm[2]);
                    xmm[3] = xmm[2];
                    xmm[2] = movss_load(e, ZERO_SCALAR);
                    xmm[5] = xmm[2];
                    xmm[2] = xmm[0];
                    xmm[0] = mulss(xmm[0], xmm[3]);
                    xmm[0] = mulss(xmm[0], xmm[3]);
                    xmm[2] = cmpeqss(xmm[2], xmm[5]);
                    xmm[5] = movss_load(e, THREE_SCALAR);
                    xmm[5] = subss(xmm[5], xmm[0]);
                    xmm[0] = movss_load(e, HALF_SCALAR);
                    xmm[0] = mulss(xmm[0], xmm[3]);
                    xmm[1] = mul4(xmm[1], xmm[1]);
                    xmm[0] = mulss(xmm[0], xmm[5]);
                    xmm[2] = andnps(xmm[2], xmm[0]);
                    xmm[0] = xmm[2];
                    xmm[0] = shufps(xmm[0], xmm[2], 0);
                    xmm[2] = addss(shufps(xmm[1], xmm[1], 0x55), xmm[1]);
                    xmm[3] = shufps(xmm[1], xmm[1], 0xaa);
                    xmm[0] = mul4(xmm[0], xmm[4]);
                    xmm[3] = addss(xmm[3], xmm[2]);
                    xmm[3] = sqrtss(xmm[3]);
                    movss_store(e, f + 0xc, xmm[3]);
                    x.fld_m32(e.mem.u32(f + 0xc));
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x1c, slot);
                    xmm[3] = movss_load(e, f + 0x1c);
                    xmm[1] = shufps(xmm[0], xmm[0], 0xaa);
                    movss_store(e, f + 0xc, xmm[1]);
                    x.fld_m32(e.mem.u32(f + 0xc));
                    xmm[1] = xmm[0];
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x14, slot);
                    xmm[2] = movss_load(e, f + 0x14);
                    xmm[1] = shufps(xmm[1], xmm[0], 0x55);
                    movss_store(e, f + 0xc, xmm[1]);
                    x.fld_m32(e.mem.u32(f + 0xc));
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0xc, slot);
                    xmm[1] = movss_load(e, f + 0xc);
                    xmm[0] = unpcklps(xmm[0], xmm[2]);
                    xmm[1] = unpcklps(xmm[1], xmm[3]);
                    xmm[0] = unpcklps(xmm[0], xmm[1]);
                    store_vec4(e, out, xmm[0]);
                    e.mem.set_u32(code_ptr, 0);
                    return 1;
                }
                // A degenerate triangle with a visited count of 2 or more: a perpendicular to
                // the first edge, and a point on the first segment.
                0x00e6_34d6 => {
                    let eax = e.mem.u32(esi + 0x68);
                    xmm[0] = load_vec4(e, eax + 0x10);
                    xmm[1] = load_vec4(e, eax);
                    xmm[1] = sub4(xmm[1], xmm[0]);
                    store_vec4(e, f + 0x20, xmm[1]);
                    x.fld_m32(e.mem.u32(f + 0x20));
                    x.fabs();
                    edi = 0;
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0xc, slot);
                    ecx = edi + 1;
                    x.fld_m32(e.mem.u32(f + 0xc));
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x10, slot);
                    x.fld_m32(e.mem.u32(f + 0x24));
                    x.fabs();
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0xc, slot);
                    x.fld_m32(e.mem.u32(f + 0xc));
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x14, slot);
                    x.fld_m32(e.mem.u32(f + 0x28));
                    x.fabs();
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0xc, slot);
                    x.fld_m32(e.mem.u32(f + 0xc));
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0xc, slot);
                    x.fld_m32(e.mem.u32(f + 0x14));
                    x.fld_m32(e.mem.u32(f + 0x10));
                    let flags = x.fcomp(1);
                    if flags & 0x41 == 0 {
                        ecx = 0;
                        let slot = x.fstp_m32();
                        e.mem.set_u32(f + 0x10, slot);
                        edi = ecx + 1;
                    } else {
                        x.pop();
                    }
                    x.fld_m32(e.mem.u32(f + 0xc));
                    x.fld_m32(e.mem.u32(f + 0x10));
                    let flags = x.fcompp();
                    if flags & 0x41 == 0 {
                        edx = edi;
                    }
                    x.fld_m32(e.mem.u32(f + 0x20 + 4 * edx));
                    xmm[0] = [0.0; 4];
                    store_vec4(e, f + 0x50, xmm[0]);
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x50 + 4 * ecx, slot);
                    x.fld_m32(e.mem.u32(f + 0x20 + 4 * ecx));
                    x.fchs();
                    let slot = x.fstp_m32();
                    e.mem.set_u32(f + 0x50 + 4 * edx, slot);
                    xmm[3] = load_vec4(e, f + 0x50);
                    xmm[0] = xmm[3];
                    xmm[0] = mul4(xmm[0], xmm[3]);
                    xmm[1] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[0] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[0] = addss(xmm[0], xmm[1]);
                    xmm[1] = xmm[0];
                    xmm[1] = rsqrtss(xmm[1]);
                    xmm[2] = xmm[1];
                    xmm[1] = movss_load(e, ZERO_SCALAR);
                    xmm[4] = xmm[1];
                    xmm[1] = xmm[0];
                    xmm[0] = mulss(xmm[0], xmm[2]);
                    xmm[0] = mulss(xmm[0], xmm[2]);
                    xmm[1] = cmpeqss(xmm[1], xmm[4]);
                    xmm[4] = movss_load(e, THREE_SCALAR);
                    xmm[4] = subss(xmm[4], xmm[0]);
                    xmm[0] = movss_load(e, HALF_SCALAR);
                    xmm[0] = mulss(xmm[0], xmm[2]);
                    xmm[0] = mulss(xmm[0], xmm[4]);
                    xmm[1] = andnps(xmm[1], xmm[0]);
                    xmm[0] = xmm[1];
                    xmm[0] = shufps(xmm[0], xmm[1], 0);
                    xmm[1] = load_vec4(e, XYZ_MASK);
                    xmm[0] = mul4(xmm[0], xmm[3]);
                    xmm[0] = andps(xmm[0], xmm[1]);
                    store_vec4(e, out, xmm[0]);
                    ecx = e.mem.u32(esi + 0x68);
                    xmm[1] = load_vec4(e, ABS_MASK_VECTOR);
                    xmm[0] = load_vec4(e, ecx);
                    xmm[2] = load_vec4(e, EPSILON_VECTOR);
                    esi = e.mem.u32(esi + 0x60);
                    xmm[0] = andps(xmm[0], xmm[1]);
                    xmm[1] = addss(shufps(xmm[0], xmm[0], 0x55), xmm[0]);
                    xmm[0] = shufps(xmm[0], xmm[0], 0xaa);
                    xmm[0] = addss(xmm[0], xmm[1]);
                    xmm[1] = xmm[0];
                    xmm[1] = addss(xmm[1], xmm[0]);
                    xmm[1] = addss(xmm[1], xmm[2]);
                    xmm[2] = load_vec4(e, esi);
                    xmm[0] = divss(xmm[0], xmm[1]);
                    xmm[1] = load_vec4(e, ONES_VECTOR);
                    xmm[0] = shufps(xmm[0], xmm[0], 0);
                    xmm[1] = sub4(xmm[1], xmm[0]);
                    xmm[1] = mul4(xmm[1], xmm[2]);
                    xmm[2] = load_vec4(e, esi + 0x10);
                    xmm[2] = mul4(xmm[2], xmm[0]);
                    xmm[1] = add4(xmm[1], xmm[2]);
                    store_vec4(e, out + 0x10, xmm[1]);
                    e.mem.set_u32(code_ptr, 0);
                    return 1;
                }
                // Returns 0 with the code 0.
                0x00e6_3723 => {
                    e.mem.set_u32(code_ptr, 0);
                    return 0;
                }
                // Returns 1 with the code 5 (the three jumps to the same exit).
                0x00e6_370c | 0x00e6_34ad | 0x00e6_34c4 => {
                    e.mem.set_u32(code_ptr, 5);
                    return 1;
                }
                _ => unreachable!("block {pc:08x}"),
            }
        }
    })
}

/// `XORPS dst, src`.
fn xorps(dst: Vec4, src: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(dst[i].to_bits() ^ src[i].to_bits()))
}

/// The routines the search calls on the simplex object it keeps in its frame: the one that
/// answers the current face (`00dfb0b0`, thiscall), the one that updates the simplex with a
/// new point (`00e67660`, thiscall, four words), the one that drops points when it has
/// grown (`00e67740`, thiscall, three words) and the one that advances to the next face
/// (`00e67ae0`, thiscall, one word).
const CURRENT_FACE: u32 = 0x00df_b0b0;
const UPDATE_SIMPLEX: u32 = 0x00e6_7660;
const PRUNE_SIMPLEX: u32 = 0x00e6_7740;
const NEXT_FACE: u32 = 0x00e6_7ae0;
/// The floats `1e-7` (`010ecedc`) and the largest float (`0104da48`) the plane interval
/// starts from, and the doubles the random offset is made with (`01032980` subtracted,
/// `010dbb40` multiplied).
const INTERVAL_LOW_FLOAT: u32 = 0x010e_cedc;
const INTERVAL_HIGH_FLOAT: u32 = 0x0104_da48;
const OFFSET_SUBTRACT_DOUBLE: u32 = 0x0103_2980;
const OFFSET_SCALE_DOUBLE: u32 = 0x010d_bb40;

// Translated from 00e63770 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`: `(this, output)`; the answer in `EAX`): the distance search
/// of two convex objects, the driver of `fn_00e62d30`. `output` receives the closest feature.
///
/// First `fn_00e62d30` is run until it gives an answer other than "keep going"; an answer
/// 1 with a code other than 5 is returned as it is. A code 5 (or a failure further on)
/// restarts the search with a random nudge: `this + 0x5c` counts the restarts (the
/// first sets `this + 0x6c` to 1, a restart at 20 or more returns 5, the others clear
/// `this + 0x6c` and add `(rand - c1) * c2` (`fn_00e62ac0` seeded with the count) to the
/// three lanes of `this + 0x30`, and the fourth lane once more of whatever the frame's
/// word beside it held) and runs again.
///
/// When `fn_00e62d30` does not answer 1 the search proper begins: a simplex object kept in
/// the stack frame (`fn_00e63ba0` fills it from the three arrays at `this + 0x60`, `0x64` and
/// `0x68`; its entries are 0x40 bytes from frame offset 0x50) is grown by one entry per step:
/// the current face comes from `00dfb0b0`, its support point from `fn_00e62b20`, and the
/// point is tested against the face (a distance along the face's normal against the four
/// thresholds at `this + 0x40`); a point behind goes straight to the last stage with code 0.
/// Otherwise the face's plane interval is clipped (`fn_00e63af0`; the face's field +0x44 gets
/// its answer), and when that gives 1 the simplex is updated (`00e67660`) and pruned
/// (`00e67740`) until no more than 0x37 entries remain, an update says stop, or the
/// entries outgrow the budget (then the code is 2). The last stage (up to ten faces,
/// advanced by `00e67ae0`) copies the face's three vertices to the simplex arrays,
/// takes the barycentric coordinates of the origin (`fn_00e61a10`) and stops at the first face
/// in which all three are non-negative; there the output gets the point (the coordinates
/// applied to the first support arrays), the negated face normal at +0 and the negated face
/// distance as its last lane, `this + 0x6c` is 3 and the code is returned.
///
/// The frame (0x2ed4 bytes in the exe, whose unwritten words read as zero here) is
/// a block of memory: the scratch words beside the simplex's header are used as the exe uses
/// them (the interval ends at frame offsets 0x30 and 0x34).
pub fn fn_00e63770(e: &mut Engine, this: Ptr, output: Ptr) -> u32 {
    let object = this.addr();
    let out = output.addr();
    e.with_stack(0x2ee0, |e, frame| {
        let f = frame.addr();
        let simplex = f + 0x30;
        let mut face = 0u32;
        let mut entry = 0u32;
        let mut pc: u32 = 0x00e6_3785;
        loop {
            match pc {
                // Run the step function until it answers something but "keep going".
                0x00e6_3785 => {
                    e.mem.set_u32(f + 0x1c, 0);
                    let answer =
                        fn_00e62d30(e, Ptr::new(object), Ptr::new(out), Ptr::new(f + 0x1c));
                    if answer == 1 {
                        let code = e.mem.u32(f + 0x1c);
                        if code != 5 {
                            return code;
                        }
                        pc = 0x00e6_37af;
                    } else {
                        pc = 0x00e6_37c7;
                    }
                }
                // A restart.
                0x00e6_37af => {
                    let count = e.mem.u32(object + 0x5c).wrapping_add(1);
                    e.mem.set_u32(object + 0x5c, count);
                    if count == 1 {
                        e.mem.set_u32(object + 0x6c, 1);
                        pc = 0x00e6_3785;
                    } else if (count as i32) >= 0x14 {
                        return 5;
                    } else {
                        e.mem.set_u32(object + 0x6c, 0);
                        let random = fn_00e62ac0(e, count).st0;
                        let current = load_vec4(e, object + 0x30);
                        let nudge = x87_mul(
                            x87_sub(random, e.mem.f64(OFFSET_SUBTRACT_DOUBLE)),
                            e.mem.f64(OFFSET_SCALE_DOUBLE),
                        );
                        let bits = x87_store32(nudge);
                        e.mem.set_u32(f + 0x18, bits);
                        let quiet = x87_quiet_bits(bits);
                        for slot in [0x20u32, 0x24, 0x28] {
                            e.mem.set_u32(f + slot, quiet);
                        }
                        let moved = add4(load_vec4(e, f + 0x20), current);
                        store_vec4(e, f + 0x20, moved);
                        store_vec4(e, object + 0x30, moved);
                        pc = 0x00e6_3785;
                    }
                }
                // Set the simplex up and start the face loop.
                0x00e6_37c7 => {
                    let first = e.mem.u32(object + 0x60);
                    let second = e.mem.u32(object + 0x64);
                    let third = e.mem.u32(object + 0x68);
                    fn_00e63ba0(
                        e,
                        Ptr::new(simplex),
                        Ptr::new(first),
                        Ptr::new(second),
                        Ptr::new(third),
                    );
                    pc = 0x00e6_37e0;
                }
                // One step: the current face, a support point along its normal.
                0x00e6_37e0 => {
                    face = e.call(CURRENT_FACE, &args![simplex]).u32();
                    let count = e.mem.u32(f + 0x38);
                    entry = f.wrapping_add(0x50).wrapping_add(count << 6);
                    e.mem.set_u32(f + 0x38, count.wrapping_add(1));
                    e.mem.set_u32(entry + 0x30, 0);
                    fn_00e62b20(e, Ptr::new(object), Ptr::new(face), Ptr::new(entry));
                    let point = load_vec4(e, entry);
                    let vertices = [0x10u32, 0x20, 0x30].map(|offset| {
                        let vertex = e.mem.u32(face + offset);
                        load_vec4(e, vertex)
                    });
                    let rows = vertices.map(|vertex| sub4(point, vertex));
                    let normal = load_vec4(e, face);
                    let distances = rotate_by_columns(rows, normal);
                    let thresholds = load_vec4(e, object + 0x40);
                    let outside = (0..3).any(|lane| distances[lane] < thresholds[lane]);
                    if outside {
                        e.mem.set_u32(f + 0x1c, 0);
                        entry = 0;
                        pc = 0x00e6_3921;
                    } else {
                        e.mem
                            .set_u32(f + 0x30, x87_quiet_bits(e.mem.u32(INTERVAL_LOW_FLOAT)));
                        e.mem
                            .set_u32(f + 0x34, x87_quiet_bits(e.mem.u32(INTERVAL_HIGH_FLOAT)));
                        e.mem.set_u32(f + 0x18, 0);
                        e.mem.set_u32(f + 0x1c, 0);
                        let clipped = fn_00e63af0(
                            e,
                            Ptr::new(face),
                            Ptr::new(entry),
                            Ptr::new(f + 0x30),
                            Ptr::new(f + 0x34),
                        );
                        e.mem.set_u32(face + 0x44, clipped);
                        if clipped != 1 {
                            pc = 0x00e6_37af;
                        } else {
                            let updated = e
                                .call(
                                    UPDATE_SIMPLEX,
                                    &args![simplex, face, entry, f + 0x1c, f + 0x18],
                                )
                                .u32();
                            if updated != 0 {
                                pc = 0x00e6_37af;
                            } else {
                                let used = e.mem.i32(f + 0x3c).wrapping_sub(e.mem.i32(f + 0x40));
                                let reported = e.mem.i32(f + 0x1c);
                                if used.wrapping_add(0x64) < reported {
                                    pc = 0x00e6_3917;
                                } else {
                                    let pruned = e
                                        .call(
                                            PRUNE_SIMPLEX,
                                            &args![
                                                simplex,
                                                reported as u32,
                                                e.mem.u32(f + 0x18),
                                                entry
                                            ],
                                        )
                                        .u32();
                                    if pruned != 0 {
                                        pc = 0x00e6_37af;
                                    } else if e.mem.i32(f + 0x38) < 0x37 {
                                        pc = 0x00e6_37e0;
                                    } else {
                                        pc = 0x00e6_3917;
                                    }
                                }
                            }
                        }
                    }
                }
                0x00e6_3917 => {
                    e.mem.set_u32(f + 0x1c, 2);
                    entry = 0;
                    pc = 0x00e6_3921;
                }
                // The face loop of the last stage: the barycentric coordinates of the origin.
                0x00e6_3921 => {
                    for (offset, target) in [(0x10u32, 0u32), (0x20, 0x10), (0x30, 0x20)] {
                        let vertex = e.mem.u32(face + offset);
                        let vector = load_vec4(e, vertex);
                        let array = e.mem.u32(object + 0x68);
                        store_vec4(e, array + target, vector);
                    }
                    let array = e.mem.u32(object + 0x68);
                    fn_00e61a10(
                        e,
                        Ptr::new(ORIGIN),
                        Ptr::new(array),
                        Ptr::new(array + 0x10),
                        Ptr::new(array + 0x20),
                        Ptr::new(f + 0x20),
                    );
                    let mut x = X87::new();
                    x.fldz();
                    let negative_a = x.fcom_m(x87_m32(e.mem.u32(f + 0x20))) & 0x41 == 0;
                    // The exe tests each coordinate in turn and stops at the first that passes (no further FCOM).
                    #[allow(clippy::if_same_then_else)]
                    let advance = if negative_a {
                        true
                    } else if x.fcom_m(x87_m32(e.mem.u32(f + 0x24))) & 0x41 == 0 {
                        true
                    } else {
                        x.fcomp_m(x87_m32(e.mem.u32(f + 0x28))) & 0x41 == 0
                    };
                    if advance {
                        face = e.call(NEXT_FACE, &args![simplex, face]).u32();
                        entry = entry.wrapping_add(1);
                        pc = if (entry as i32) < 10 {
                            0x00e6_3921
                        } else {
                            0x00e6_39fe
                        };
                    } else {
                        pc = 0x00e6_39fe;
                    }
                }
                // Done: copy the face's supports and make the output.
                0x00e6_39fe => {
                    for (offset, target) in [(0x10u32, 0u32), (0x20, 0x10), (0x30, 0x20)] {
                        let vertex = e.mem.u32(face + offset);
                        let first_support = load_vec4(e, vertex + 0x10);
                        let array = e.mem.u32(object + 0x60);
                        store_vec4(e, array + target, first_support);
                        let second_support = load_vec4(e, vertex + 0x20);
                        let array = e.mem.u32(object + 0x64);
                        store_vec4(e, array + target, second_support);
                    }
                    let mut slots = [0u32; 3];
                    for (index, slot) in [0x20u32, 0x24, 0x28].into_iter().enumerate() {
                        slots[index] = x87_quiet_bits(e.mem.u32(f + slot));
                        e.mem.set_u32(f + 0x18, slots[index]);
                    }
                    e.mem.set_u32(object + 0x6c, 3);
                    let supports = e.mem.u32(object + 0x60);
                    let weights = slots.map(f32::from_bits);
                    let first = mul4([weights[1]; 4], load_vec4(e, supports + 0x10));
                    let second = mul4([weights[0]; 4], load_vec4(e, supports));
                    let third = mul4([weights[2]; 4], load_vec4(e, supports + 0x20));
                    store_vec4(e, out + 0x10, add4(add4(first, second), third));
                    let normal = xor_mask(e, load_vec4(e, face), SIGN_MASK);
                    store_vec4(e, out, normal);
                    let distance = -x87_load32(e.mem.u32(face + 0x40));
                    e.mem.set_u32(out + 0xc, x87_store32(distance));
                    return e.mem.u32(f + 0x1c);
                }
                _ => unreachable!("block {pc:08x}"),
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e61720, fn_00e61720(Ptr, Ptr) -> Ptr),
        entry!(0x00e61770, fn_00e61770(Ptr, Ptr) -> Ptr),
        entry!(0x00e61a10, fn_00e61a10(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e61d30, fn_00e61d30(Ptr, Ptr)),
        entry!(0x00e61e70, fn_00e61e70(Ptr, Ptr)),
        entry!(0x00e61fb0, fn_00e61fb0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e62100, fn_00e62100(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e625c0, fn_00e625c0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e62740, fn_00e62740(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e62d30, fn_00e62d30(Ptr, Ptr, Ptr) -> u32),
        (0x00e62ac0, fn_00e62ac0_entry as AbiFn),
        entry!(0x00e62b20, fn_00e62b20(Ptr, Ptr, Ptr)),
        entry!(0x00e62c10, fn_00e62c10(Ptr, Ptr)),
        entry!(0x00e62c60, fn_00e62c60(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e63770, fn_00e63770(Ptr, Ptr) -> u32),
        entry!(0x00e63af0, fn_00e63af0(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e63ba0, fn_00e63ba0(Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A plane with normal (0, 0, 1) through the origin and the point
    /// (0, 0, z); returns the result and the interval afterwards.
    fn clip_00e63af0(z: f32, low: f32, high: f32) -> (u32, f32, f32) {
        let mut e = Engine::new();
        e.map(0x0109_a000, 0x1000);
        e.mem.set_f64(EPSILON_0109A3A0, 1.0e-7);
        let origin = e.mem.alloc(16);
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 8, 1.0);
        e.mem.set_u32(this + 0x10, origin);
        let point = e.mem.alloc(16);
        e.mem.set_f32(point + 8, z);
        let interval = e.mem.alloc(8);
        e.mem.set_f32(interval, low);
        e.mem.set_f32(interval + 4, high);
        let result = e
            .call(
                0x00e6_3af0,
                &args![
                    Ptr::<()>::new(this),
                    Ptr::<()>::new(point),
                    Ptr::<()>::new(interval),
                    Ptr::<()>::new(interval + 4)
                ],
            )
            .u32();
        (result, e.mem.f32(interval), e.mem.f32(interval + 4))
    }

    #[test]
    fn fn_00e63af0_narrows_the_high_end_when_the_plane_is_inside() {
        assert_eq!(clip_00e63af0(2.0, 0.0, 5.0), (1, 0.0, 2.0));
        assert_eq!(clip_00e63af0(7.0, 0.0, 5.0), (1, 0.0, 5.0));
    }

    #[test]
    fn fn_00e63af0_raises_the_low_end_a_little_past_the_plane() {
        // dist == low: the low end moves by the epsilon.
        let (result, low, high) = clip_00e63af0(1.0, 1.0, 5.0);
        assert_eq!(result, 2);
        assert_eq!(low, (1.0f64 + 1.0e-7) as f32);
        assert_eq!(high, 5.0);
        // Far below the low end: nothing moves.
        assert_eq!(clip_00e63af0(-3.0, 1.0, 5.0), (2, 1.0, 5.0));
    }

    #[test]
    fn fn_00e63af0_reports_an_empty_interval_as_4() {
        assert_eq!(clip_00e63af0(2.0, 3.0, 1.0).0, 4);
        assert_eq!(clip_00e63af0(2.0, 1.0, 1.0).0, 4);
        // A NaN distance takes the second branch and never compares as empty.
        assert_eq!(clip_00e63af0(f32::NAN, 0.0, 5.0).0, 2);
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn alloc_ptr(e: &mut Engine, size: u32) -> Ptr {
        Ptr::new(e.mem.alloc(size))
    }

    fn put_vec4(e: &mut Engine, values: [f32; 4]) -> Ptr {
        let at = alloc_ptr(e, 16);
        store_vec4(e, at.addr(), values);
        at
    }

    fn barycentric(point: [f32; 3], a: [f32; 3], base: [f32; 3], c: [f32; 3]) -> [f32; 3] {
        let mut e = Engine::new();
        let lift = |v: [f32; 3]| [v[0], v[1], v[2], 0.0];
        let (point, a, base, c) = (
            put_vec4(&mut e, lift(point)),
            put_vec4(&mut e, lift(a)),
            put_vec4(&mut e, lift(base)),
            put_vec4(&mut e, lift(c)),
        );
        let out = alloc_ptr(&mut e, 16);
        e.call(0x00e6_1a10, &args![point, a, base, c, out]);
        [0, 4, 8].map(|offset| e.mem.f32(out.addr() + offset))
    }

    #[test]
    fn fn_00e61a10_gives_barycentric_coordinates_inside_a_triangle() {
        // base (0,0,0), a (2,0,0), c (0,2,0), the point (0.5,0.5,0): X = Y = 4, det = 16.
        let weights = barycentric(
            [0.5, 0.5, 0.0],
            [2.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0],
        );
        assert_eq!(weights, [0.25, 0.5, 0.25]);
        // The point at `a` itself.
        let weights = barycentric(
            [2.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0],
        );
        assert_eq!(weights, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_00e61a10_projects_onto_an_edge_of_a_degenerate_triangle() {
        let base = [0.0, 0.0, 0.0];
        // Collinear: base, a = (1,0,0), c = (2,0,0): b = 1 < cc = 4, e3 = 1 < cc, so
        // (0, 1 - t, t) with t = ((point - base).d2) / cc.
        assert_eq!(
            barycentric([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], base, [2.0, 0.0, 0.0]),
            [0.0, 0.5, 0.5]
        );
        // a = (4,0,0), c = (1,0,0): b = 16 >= cc = 1, e3 = 9 < b: (t, 1 - t, 0) with
        // t = ((point - base).d1) / b.
        assert_eq!(
            barycentric([2.0, 0.0, 0.0], [4.0, 0.0, 0.0], base, [1.0, 0.0, 0.0]),
            [0.5, 0.5, 0.0]
        );
        // a = (1,0,0), c = (-1,0,0): b = cc = 1, e3 = 4 >= b: (t, 0, 1 - t) with
        // t = ((point - c).(a - c)) / e3 = 2 / 4.
        assert_eq!(
            barycentric([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], base, [-1.0, 0.0, 0.0]),
            [0.5, 0.0, 0.5]
        );
        // a = (1,0,0), c = (-2,0,0): b = 1 < cc = 4, e3 = 9 >= cc: (t, 0, 1 - t) with
        // t = ((point - c).(a - c)) / e3 = 6 / 9.
        let weights = barycentric([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], base, [-2.0, 0.0, 0.0]);
        assert_eq!(weights[1], 0.0);
        assert_eq!(weights[0], 6.0f32 / 9.0);
        assert_eq!(weights[2], 1.0 - 6.0f32 / 9.0);
    }

    #[test]
    fn fn_00e61a10_returns_zeros_when_all_points_coincide() {
        // b = cc = 0 and e3 = 0: not b < cc, not e3 < b, not 0 < e3.
        let weights = barycentric([1.0, 2.0, 3.0], [0.0; 3], [0.0; 3], [0.0; 3]);
        assert_eq!(weights, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_00e61a10_treats_a_nan_determinant_as_degenerate() {
        // A NaN coordinate makes the determinant NaN: the compare is unordered, so the
        // degenerate branch runs, and every later compare is unordered too, which the exe
        // reads as "less than" (C0 is set): the `b < cc` branch, then `e3 < cc`: (0, 1 - t, t).
        let weights = barycentric(
            [0.0, 0.0, 0.0],
            [f32::NAN, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        );
        assert_eq!(weights, [0.0, 1.0, 0.0]);
    }

    fn gram(points: [[f32; 3]; 3]) -> [f32; 4] {
        let mut e = Engine::new();
        let at = alloc_ptr(&mut e, 0x30);
        for (index, point) in points.iter().enumerate() {
            store_vec4(
                &mut e,
                at.addr() + 0x10 * index as u32,
                [point[0], point[1], point[2], 0.0],
            );
        }
        let out = alloc_ptr(&mut e, 16);
        e.call(0x00e6_1d30, &args![at, out]);
        [0, 4, 8, 12].map(|offset| e.mem.f32(out.addr() + offset))
    }

    #[test]
    fn fn_00e61d30_makes_the_reciprocal_gram_matrix() {
        // p1 at the origin, d1 = (2,0,0), d2 = (0,2,0): n11 = n22 = 4, m = 0, det = 16,
        // the normal (0,0,4).
        assert_eq!(
            gram([[2.0, 0.0, 0.0], [0.0; 3], [0.0, 2.0, 0.0]]),
            [0.25, 0.25, 0.0, 0.25]
        );
        // d1 = (1,0,0), d2 = (1,1,0): n11 = 1, n22 = 2, m = 1, det = 1, the normal (0,0,1).
        assert_eq!(
            gram([[1.0, 0.0, 0.0], [0.0; 3], [1.0, 1.0, 0.0]]),
            [1.0, 2.0, 1.0, 1.0]
        );
    }

    #[test]
    fn fn_00e61d30_divides_by_zero_like_the_x87_does() {
        // Collinear points: det = 0, so 1 / det is infinite and n11 * inf, 0 * inf is NaN.
        let out = gram([[1.0, 0.0, 0.0], [0.0; 3], [2.0, 0.0, 0.0]]);
        assert_eq!(out[0], f32::INFINITY);
        assert_eq!(out[1], f32::INFINITY);
        assert!(out[2].is_infinite());
        assert_eq!(out[3], f32::INFINITY);
    }

    /// The doubles a mesh section's reference counting calls, recording into the log.
    fn section_world() -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0xa110c);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [ADD_REFERENCE, REMOVE_REFERENCE] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let (array, wanted) = (a[1], a[2]);
            let size = e.mem.u32(array + 4);
            let data = e.mem.alloc(wanted * 4);
            for i in 0..size {
                let word = e.mem.u32(e.mem.u32(array) + 4 * i);
                e.mem.set_u32(data + 4 * i, word);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, wanted);
            Ret::default()
        });
        (e, log)
    }

    /// An array at `at` (data, size, capacity word) holding `items`, room for `capacity`.
    fn put_array(e: &mut Engine, at: u32, items: &[u32], capacity: u32) {
        let data = e.mem.alloc(capacity.max(1) * 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        e.mem.set_u32(at, data);
        e.mem.set_u32(at + 4, items.len() as u32);
        e.mem.set_u32(at + 8, capacity);
    }

    fn read_array(e: &Engine, at: u32) -> Vec<u32> {
        let data = e.mem.u32(at);
        (0..e.mem.u32(at + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn make_source(e: &mut Engine) -> Ptr {
        let source = alloc_ptr(e, 0x28);
        let s = source.addr();
        e.mem.set_u16(s + 4, 0x1234);
        e.mem.set_u16(s + 6, 7);
        e.mem.set_u32(s + 8, 0x1000);
        put_array(e, s + 0xc, &[0x2000, 0x2001, 0x2002], 4);
        e.mem.set_u32(s + 0x18, 0x3000);
        put_array(e, s + 0x1c, &[0x4000, 0x4001], 2);
        source
    }

    #[test]
    fn fn_00e61770_assigns_pointers_and_arrays_with_reference_counts() {
        let (mut e, log) = section_world();
        let source = make_source(&mut e);
        let this = alloc_ptr(&mut e, 0x28);
        let t = this.addr();
        e.mem.set_u16(t + 4, 1);
        e.mem.set_u16(t + 6, 2);
        e.mem.set_u32(t + 8, 0x1111);
        put_array(
            &mut e,
            t + 0xc,
            &[0x5000, 0x5001, 0x5002, 0x5003, 0x5004],
            8,
        );
        e.mem.set_u32(t + 0x18, 0);
        e.mem.set_u32(t + 0x1c, 0);
        e.mem.set_u32(t + 0x20, 0);
        e.mem.set_u32(t + 0x24, 0x8000_0000);
        let result = e.call(0x00e6_1770, &args![this, source]);
        assert_eq!(result.u32(), t);
        assert_eq!(e.mem.u16(t + 4), 0x1234);
        assert_eq!(e.mem.u16(t + 6), 7);
        assert_eq!(e.mem.u32(t + 8), 0x1000);
        assert_eq!(read_array(&e, t + 0xc), vec![0x2000, 0x2001, 0x2002]);
        assert_eq!(e.mem.u32(t + 0x18), 0x3000);
        assert_eq!(read_array(&e, t + 0x1c), vec![0x4000, 0x4001]);
        // The order of the reference calls: the pointer first, the old pointer released;
        // the array's tail released, then every common element referenced and the old
        // element released; the second pair likewise (no old material), the array
        // reserved to 2 and its two new elements referenced.
        let expected: Vec<(u32, Vec<u32>)> = vec![
            (ADD_REFERENCE, vec![0x1000]),
            (REMOVE_REFERENCE, vec![0x1111]),
            (REMOVE_REFERENCE, vec![0x5003]),
            (REMOVE_REFERENCE, vec![0x5004]),
            (ADD_REFERENCE, vec![0x2000]),
            (REMOVE_REFERENCE, vec![0x5000]),
            (ADD_REFERENCE, vec![0x2001]),
            (REMOVE_REFERENCE, vec![0x5001]),
            (ADD_REFERENCE, vec![0x2002]),
            (REMOVE_REFERENCE, vec![0x5002]),
            (ADD_REFERENCE, vec![0x3000]),
            (ARRAY_RESERVE, vec![0xa110c, t + 0x1c, 2, 4]),
            (ADD_REFERENCE, vec![0x4000]),
            (ADD_REFERENCE, vec![0x4001]),
        ];
        assert_eq!(*log.borrow(), expected);
        // The destination array's data stayed (capacity 8 was enough), the tail is cleared.
        let data = e.mem.u32(t + 0xc);
        assert_eq!(e.mem.u32(data + 12), 0);
        assert_eq!(e.mem.u32(data + 16), 0);
    }

    #[test]
    fn fn_00e61770_grows_by_doubling_and_skips_shared_elements() {
        let (mut e, log) = section_world();
        let source = alloc_ptr(&mut e, 0x28);
        let s = source.addr();
        put_array(&mut e, s + 0xc, &[1, 2, 3, 4, 5, 6, 7], 8);
        put_array(&mut e, s + 0x1c, &[0x50, 0x51], 2);
        let this = alloc_ptr(&mut e, 0x28);
        let t = this.addr();
        put_array(&mut e, t + 0xc, &[9, 9, 9], 3);
        // The second array of the destination shares its data with the source's.
        e.mem.set_u32(t + 0x1c, e.mem.u32(s + 0x1c));
        e.mem.set_u32(t + 0x20, 2);
        e.mem.set_u32(t + 0x24, 2);
        e.call(0x00e6_1770, &args![this, source]);
        // Capacity 3 < 7: the new capacity is max(2 * 3, 7) = 7.
        assert_eq!(
            calls(&log, ARRAY_RESERVE),
            vec![vec![0xa110c, t + 0xc, 7, 4]]
        );
        assert_eq!(read_array(&e, t + 0xc), vec![1, 2, 3, 4, 5, 6, 7]);
        // The shared second array: no reference traffic for its elements.
        assert!(!calls(&log, ADD_REFERENCE).iter().any(|c| c[0] == 0x50));
        assert!(!calls(&log, REMOVE_REFERENCE).iter().any(|c| c[0] == 0x50));
        assert_eq!(read_array(&e, t + 0x1c), vec![0x50, 0x51]);
        // A larger need than double the capacity takes the need: capacity 1, need 7 -> 7;
        // capacity 5, need 7 -> 10.
        let (mut e, log) = section_world();
        let source = alloc_ptr(&mut e, 0x28);
        put_array(&mut e, source.addr() + 0xc, &[1, 2, 3, 4, 5, 6, 7], 8);
        put_array(&mut e, source.addr() + 0x1c, &[], 1);
        let this = alloc_ptr(&mut e, 0x28);
        put_array(&mut e, this.addr() + 0xc, &[1, 1, 1, 1, 1], 5);
        put_array(&mut e, this.addr() + 0x1c, &[], 1);
        e.call(0x00e6_1770, &args![this, source]);
        assert_eq!(
            calls(&log, ARRAY_RESERVE),
            vec![vec![0xa110c, this.addr() + 0xc, 10, 4]]
        );
    }

    #[test]
    fn fn_00e61720_makes_a_copy_with_the_vtable() {
        let (mut e, log) = section_world();
        let source = make_source(&mut e);
        let this = alloc_ptr(&mut e, 0x28);
        let t = this.addr();
        // Garbage that the constructor must not rely on in the array words.
        e.mem.set_u32(t + 0x14, 0x1234_5678);
        let result = e.call(0x00e6_1720, &args![this, source]);
        assert_eq!(result.u32(), t);
        assert_eq!(e.mem.u32(t), MESH_SECTION_VTABLE);
        // The assignment copies the words at +4 and +6 over the constructor's 1.
        assert_eq!(e.mem.u16(t + 4), 0x1234);
        assert_eq!(e.mem.u16(t + 6), 7);
        assert_eq!(e.mem.u32(t + 8), 0x1000);
        assert_eq!(read_array(&e, t + 0xc), vec![0x2000, 0x2001, 0x2002]);
        assert_eq!(e.mem.u32(t + 0x18), 0x3000);
        assert_eq!(read_array(&e, t + 0x1c), vec![0x4000, 0x4001]);
        // Both arrays started empty and not owned (capacity word 0x80000000): reserves of
        // 3 and 2, no object to release.
        assert_eq!(
            calls(&log, ARRAY_RESERVE),
            vec![vec![0xa110c, t + 0xc, 3, 4], vec![0xa110c, t + 0x1c, 2, 4]]
        );
        assert!(calls(&log, REMOVE_REFERENCE).is_empty());
        assert_eq!(calls(&log, ADD_REFERENCE).len(), 7);
    }

    const SHAPE_A_SUPPORT: u32 = 0x00aa_1000;
    const SHAPE_B_SUPPORT: u32 = 0x00aa_1100;

    fn vec_at(e: &Engine, at: u32) -> [f32; 4] {
        load_vec4(e, at)
    }

    /// An object whose vtable slot 1 (offset 4) is a double writing `direction * scale` (all
    /// four lanes) to its `out` argument.
    fn make_shape(e: &mut Engine, support: u32, vtable: u32, scale: f32) -> u32 {
        e.put_vtable(vtable, &[0, support]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.register_double(support, move |e, a| {
            let direction = load_vec4(e, a[1]);
            store_vec4(e, a[2], direction.map(|lane| lane * scale));
            Ret::default()
        });
        object
    }

    /// A transform object: identity rows, the translation `(1, 2, 3)` and the two shapes
    /// (scales 10 and 3) at +0x54 and +0x58.
    fn make_machine(e: &mut Engine) -> Ptr {
        e.map(0x0126_9000, 0x1000);
        let machine = alloc_ptr(e, 0x80);
        let m = machine.addr();
        store_vec4(e, m, [1.0, 0.0, 0.0, 0.0]);
        store_vec4(e, m + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec4(e, m + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec4(e, m + 0x30, [1.0, 2.0, 3.0, 0.0]);
        let shape_a = make_shape(e, SHAPE_A_SUPPORT, 0x0126_9000, 10.0);
        let shape_b = make_shape(e, SHAPE_B_SUPPORT, 0x0126_9100, 3.0);
        e.mem.set_u32(m + 0x54, shape_a);
        e.mem.set_u32(m + 0x58, shape_b);
        machine
    }

    fn with_constants(e: &mut Engine) {
        e.map(0x010c_4000, 0x4000);
        e.map(0x0102_2000, 0x1000);
        e.map(0x0101_1000, 0x1000);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_f32(THREES + 4 * lane, 3.0);
            e.mem.set_f32(HALVES + 4 * lane, 0.5);
        }
        e.mem.set_f64(0x0102_29b8, 1.0 / 2_147_483_648.0);
    }

    #[test]
    fn fn_00e62ac0_steps_the_generator_and_scales_it() {
        let mut e = Engine::new();
        with_constants(&mut e);
        let step = |e: &mut Engine, seed: u32| e.call(0x00e6_2ac0, &args![seed]);
        let result = step(&mut e, 0);
        assert_eq!(result.u32(), 0);
        assert_eq!(result.f32(), 12345.0 / 2_147_483_648.0);
        let seed = 0x1234_5678u32;
        let scrambled = seed.wrapping_mul(0x3e39_b193);
        let result = step(&mut e, seed);
        assert_eq!(result.u32(), scrambled);
        let expected = (0x3039u32.wrapping_sub(scrambled) & 0x7fff_ffff) as f32 / 2_147_483_648.0;
        assert_eq!(result.f32(), expected);
        assert!((0.0..1.0).contains(&result.f32()));
    }

    #[test]
    fn fn_00e62c10_appends_three_vectors_to_three_arrays() {
        let mut e = Engine::new();
        let this = alloc_ptr(&mut e, 0x80);
        let t = this.addr();
        let arrays = [0x68u32, 0x60, 0x64].map(|offset| {
            let array = e.mem.alloc(0x40);
            e.mem.set_u32(t + offset, array);
            array
        });
        e.mem.set_u32(t + 0x6c, 2);
        let rows = alloc_ptr(&mut e, 0x30);
        for (index, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            store_vec4(&mut e, rows.addr() + 0x10 * index as u32, [*value; 4]);
        }
        e.call(0x00e6_2c10, &args![this, rows]);
        assert_eq!(e.mem.u32(t + 0x6c), 3);
        // Row 0 to the array at +0x68, row 1 to +0x60, row 2 to +0x64, at index 2.
        assert_eq!(vec_at(&e, arrays[0] + 0x20), [1.0; 4]);
        assert_eq!(vec_at(&e, arrays[1] + 0x20), [2.0; 4]);
        assert_eq!(vec_at(&e, arrays[2] + 0x20), [3.0; 4]);
        assert_eq!(vec_at(&e, arrays[0] + 0x10), [0.0; 4]);
    }

    #[test]
    fn fn_00e63ba0_copies_the_three_blocks_and_starts_the_machine() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(0x00e6_79d0, move |e, a| {
            // Seen at the call: the flag is set, the field after the copies still clear.
            sink.borrow_mut()
                .push((0x00e6_79d0, vec![a[0], e.mem.u32(a[0] + 8)]));
            Ret::default()
        });
        let this = alloc_ptr(&mut e, 0x120);
        let t = this.addr();
        e.mem.set_u32(t + 0xc, 0xdead);
        e.mem.set_u32(t + 0x50, 0xdead);
        let block = |e: &mut Engine, base: f32| {
            let at = alloc_ptr(e, 0x40);
            for row in 0..4u32 {
                store_vec4(e, at.addr() + 0x10 * row, [base + row as f32; 4]);
            }
            at
        };
        let (a, b, c) = (
            block(&mut e, 10.0),
            block(&mut e, 20.0),
            block(&mut e, 30.0),
        );
        e.call(0x00e6_3ba0, &args![this, a, b, c]);
        assert_eq!(e.mem.u32(t + 8), 4);
        for row in 0..4u32 {
            assert_eq!(vec_at(&e, t + 0x30 + 0x40 * row), [10.0 + row as f32; 4]);
            assert_eq!(vec_at(&e, t + 0x40 + 0x40 * row), [20.0 + row as f32; 4]);
            assert_eq!(vec_at(&e, t + 0x20 + 0x40 * row), [30.0 + row as f32; 4]);
            assert_eq!(e.mem.u32(t + 0x50 + 0x40 * row), 0);
        }
        assert_eq!(e.mem.u32(t + 0xc), 0);
        assert_eq!(*log.borrow(), vec![(0x00e6_79d0, vec![t, 4])]);
    }

    #[test]
    fn fn_00e62b20_makes_the_support_point_of_the_difference() {
        let mut e = Engine::new();
        with_constants(&mut e);
        let machine = make_machine(&mut e);
        let direction = put_vec4(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let out = alloc_ptr(&mut e, 0x30);
        e.call(0x00e6_2b20, &args![machine, direction, out]);
        let o = out.addr();
        // Shape A sees the direction itself, shape B its negation (identity rotation): the
        // transformed point is (-3, 0, 0) + (1, 2, 3).
        assert_eq!(vec_at(&e, o + 0x10), [10.0, 0.0, 0.0, 0.0]);
        assert_eq!(vec_at(&e, o + 0x20)[..3], [-3.0, -0.0, -0.0]);
        assert_eq!(vec_at(&e, o)[..3], [12.0, -2.0, -3.0]);
    }

    #[test]
    fn fn_00e62b20_carries_the_direction_through_the_transform_columns() {
        let mut e = Engine::new();
        with_constants(&mut e);
        let machine = make_machine(&mut e);
        let m = machine.addr();
        // A rotation of a quarter turn about z: rows (0,1,0), (-1,0,0), (0,0,1).
        store_vec4(&mut e, m, [0.0, 1.0, 0.0, 0.0]);
        store_vec4(&mut e, m + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(SHAPE_B_SUPPORT, move |e, a| {
            let direction = load_vec4(e, a[1]);
            sink.borrow_mut().push(direction);
            store_vec4(e, a[2], [direction[0], direction[1], direction[2], 0.0]);
            Ret::default()
        });
        let direction = put_vec4(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let out = alloc_ptr(&mut e, 0x30);
        e.call(0x00e6_2b20, &args![machine, direction, out]);
        // The columns are (0, -1, 0), (1, 0, 0), (0, 0, 1) and the vector is -d =
        // (-1, -2, -3): y * column_y + x * column_x + z * column_z = (-2, 1, -3) (the exe's
        // `y` column is the first column of the x*column_x pairing; the fourth lane is whatever
        // the shuffles leave).
        let direction_for_b = seen.borrow()[0];
        let columns_x = [0.0f32, -1.0, 0.0];
        let columns_y = [1.0f32, 0.0, 0.0];
        let columns_z = [0.0f32, 0.0, 1.0];
        for lane in 0..3 {
            let expected = -2.0 * columns_y[lane] + -1.0 * columns_x[lane] + -3.0 * columns_z[lane];
            assert_eq!(direction_for_b[lane], expected, "lane {lane}");
        }
    }

    #[test]
    fn fn_00e62c60_keeps_the_farther_support_point() {
        for (origin_x, expect_copy) in [(-100.0f32, true), (100.0, false)] {
            let mut e = Engine::new();
            with_constants(&mut e);
            let machine = make_machine(&mut e);
            let direction = put_vec4(&mut e, [1.0, 0.0, 0.0, 0.0]);
            let origin = put_vec4(&mut e, [origin_x, 0.0, 0.0, 0.0]);
            let best = alloc_ptr(&mut e, 16);
            let vertex = alloc_ptr(&mut e, 0x40);
            {
                let offset = 0x30u32;
                e.mem.set_u32(vertex.addr() + offset, 0x7777);
            }
            // What the two calls make, by calling the first function directly.
            let near = alloc_ptr(&mut e, 0x30);
            e.call(0x00e6_2b20, &args![machine, direction, near]);
            let near_point = vec_at(&e, near.addr());
            let negated = put_vec4(&mut e, [-1.0, -0.0, -0.0, -0.0]);
            let far = alloc_ptr(&mut e, 0x30);
            e.call(0x00e6_2b20, &args![machine, negated, far]);
            let far_point = vec_at(&e, far.addr());
            let d1 = near_point[0] - origin_x;
            let d2 = (far_point[0] - origin_x) * -1.0;
            e.call(
                0x00e6_2c60,
                &args![machine, best, direction, origin, vertex],
            );
            assert_eq!(e.mem.f32(best.addr()), if d1 > d2 { d1 } else { d2 });
            let copied = vec_at(&e, vertex.addr()) == near_point;
            assert_eq!(copied, expect_copy, "d1 {d1}, d2 {d2}");
            if expect_copy {
                assert_eq!(vec_at(&e, vertex.addr() + 0x10), [10.0, 0.0, 0.0, 0.0]);
                // The word at +0x30 comes from the call's frame, which nothing wrote.
                assert_eq!(e.mem.u32(vertex.addr() + 0x30), 0);
            } else {
                assert_eq!(e.mem.u32(vertex.addr() + 0x30), 0x7777);
            }
        }
    }

    #[test]
    fn fn_00e625c0_updates_the_best_direction_only_when_it_is_farther() {
        for (limit, expect_update) in [(-100.0f32, true), (1000.0, false)] {
            let mut e = Engine::new();
            with_constants(&mut e);
            let machine = make_machine(&mut e);
            let m = machine.addr();
            let shape_a = e.mem.u32(m + 0x54);
            let shape_b = e.mem.u32(m + 0x58);
            let direction = put_vec4(&mut e, [1.0, 0.5, 0.0, 0.0]);
            let out = alloc_ptr(&mut e, 0x40);
            e.mem.set_f32(out.addr() + 0x3c, limit);
            e.mem.set_f32(out.addr() + 0x30, 9.0);
            e.call(
                0x00e6_25c0,
                &args![
                    Ptr::<()>::new(shape_a),
                    machine,
                    direction,
                    Ptr::<()>::new(shape_b),
                    out
                ],
            );
            let o = out.addr();
            if expect_update {
                // A sees -direction = (-1, -0.5, 0): point (-10, -5, -0); B sees the direction
                // rotated (identity): point (3, 1.5, 0); moved by the translation (1, 2, 3)
                // = (4, 3.5, 3). w = first - moved; dist = w . direction.
                assert_eq!(vec_at(&e, o)[..2], [-10.0, -5.0]);
                assert_eq!(vec_at(&e, o + 0x20)[..3], [3.0, 1.5, 0.0]);
                assert_eq!(vec_at(&e, o + 0x10)[..3], [4.0, 3.5, 3.0]);
                let w = [-10.0f32 - 4.0, -5.0 - 3.5, -0.0 - 3.0];
                let distance = w[2] * 0.0 + (w[1] * 0.5 + w[0] * 1.0);
                assert_eq!(vec_at(&e, o + 0x30), [1.0, 0.5, 0.0, distance]);
            } else {
                assert_eq!(e.mem.f32(o + 0x30), 9.0);
                assert_eq!(vec_at(&e, o), [0.0; 4]);
            }
        }
    }

    fn relative_close(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() <= expected.abs() * 6e-7
    }

    #[test]
    fn fn_00e61e70_gives_inverse_edge_lengths_and_the_normal_length() {
        let mut e = Engine::new();
        with_constants(&mut e);
        let at = alloc_ptr(&mut e, 0x30);
        for (index, point) in [[0.0f32, 0.0, 0.0], [3.0, 0.0, 0.0], [0.0, 4.0, 0.0]]
            .iter()
            .enumerate()
        {
            store_vec4(
                &mut e,
                at.addr() + 0x10 * index as u32,
                [point[0], point[1], point[2], 0.0],
            );
        }
        let out = alloc_ptr(&mut e, 0x20);
        e.call(0x00e6_1e70, &args![at, out]);
        // |p2 - p1| = 5, |p0 - p2| = 4, |p1 - p0| = 3, |normal| = 12 (twice the area).
        let lengths = [0, 4, 8, 12, 16].map(|offset| e.mem.f32(out.addr() + offset));
        let expected = [1.0 / 5.0, 1.0 / 4.0, 1.0 / 3.0, 1.0 / 12.0, 12.0];
        for lane in 0..5 {
            assert!(
                relative_close(lengths[lane], expected[lane]),
                "{lane}: {} vs {}",
                lengths[lane],
                expected[lane]
            );
        }
    }

    #[test]
    fn fn_00e61fb0_builds_the_matrix_and_the_normal() {
        let mut e = Engine::new();
        let at = alloc_ptr(&mut e, 0x30);
        for (index, point) in [[1.0f32, 0.0, 0.0], [0.0; 3], [0.0, 1.0, 0.0]]
            .iter()
            .enumerate()
        {
            store_vec4(
                &mut e,
                at.addr() + 0x10 * index as u32,
                [point[0], point[1], point[2], 0.0],
            );
        }
        let factors = alloc_ptr(&mut e, 0x20);
        for (index, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0].iter().enumerate() {
            e.mem.set_f32(factors.addr() + 4 * index as u32, *value);
        }
        let matrix = alloc_ptr(&mut e, 0x40);
        let normal = alloc_ptr(&mut e, 0x10);
        e.call(0x00e6_1fb0, &args![at, factors, matrix, normal]);
        // a = (1,-1,0), b = (0,1,0), c = (-1,0,0): N = (0, 0, -1) * 4; the rows are the
        // cross products of (s0 b, s1 a, s2 c) = ((0,1,0), (2,-2,0), (-3,0,0)) with N, then
        // transposed in the 3 by 3 part, with N in column 3 and (-s0 * s4, 0, 0, 0) below.
        assert_eq!(vec_at(&e, normal.addr())[..3], [0.0, 0.0, -4.0]);
        let m = matrix.addr();
        assert_eq!(vec_at(&e, m), [-4.0, 8.0, 0.0, 0.0]);
        assert_eq!(vec_at(&e, m + 0x10), [0.0, 8.0, -12.0, 0.0]);
        assert_eq!(vec_at(&e, m + 0x20), [0.0, 0.0, 0.0, -4.0]);
        assert_eq!(vec_at(&e, m + 0x30), [-5.0, 0.0, 0.0, 0.0]);
    }

    type Segments = Rc<RefCell<Vec<(u32, [f32; 4])>>>;

    /// The world of `fn_00e62100`: constants, the tables, a triangle with p0 = (1,0,0),
    /// p1 = (0,0,0), p2 = (0,1,0), a plane (0, 0, -1) with scale 2, and a double for
    /// `closestPointLineSeg` that answers from `segments` (code, point) in call order and
    /// records its arguments.
    struct Closest {
        e: Engine,
        triangle: Ptr,
        plane: Ptr,
        out: Ptr,
        info: Ptr,
        calls: Log,
    }

    fn closest_world(segments: Vec<(u32, [f32; 4])>) -> Closest {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x010c_4000, 0x8000);
        e.map(0x010e_c000, 0x1000);
        e.map(0x0126_7000, 0x1000);
        e.mem.set_f64(SHIFT_SMALL_DOUBLE, 0.001);
        e.mem.set_f64(SHIFT_ONE_DOUBLE, 1.001);
        e.mem.set_f32(FOURTH_LANE_FLOAT, 0.001);
        e.mem.set_f32(ZERO_FLOAT, 0.0);
        e.mem.set_f32(THREE_FLOAT, 3.0);
        e.mem.set_f32(HALF_FLOAT, 0.5);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        for (index, byte) in [254u8, 0, 1, 250, 2, 249, 248, 254].iter().enumerate() {
            e.mem.set_u8(OUTCOME_TABLE + index as u32, *byte);
        }
        for (index, byte) in [2u8, 0, 1, 2, 0, 0, 0, 0, 0x95].iter().enumerate() {
            e.mem.set_u8(NEIGHBOUR_TABLE_PREVIOUS + index as u32, *byte);
        }
        e.register(X87_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        let calls: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&calls);
        let queue: Segments = Rc::new(RefCell::new(segments));
        e.register_double(CLOSEST_POINT_LINE_SEG, move |e, a| {
            sink.borrow_mut().push((CLOSEST_POINT_LINE_SEG, a.to_vec()));
            let (code, vector) = queue.borrow_mut().remove(0);
            store_vec4(e, a[3], vector);
            Ret {
                eax: code,
                ..Ret::default()
            }
        });
        let triangle = alloc_ptr(&mut e, 0x30);
        store_vec4(&mut e, triangle.addr(), [1.0, 0.0, 0.0, 0.0]);
        store_vec4(&mut e, triangle.addr() + 0x10, [0.0; 4]);
        store_vec4(&mut e, triangle.addr() + 0x20, [0.0, 1.0, 0.0, 0.0]);
        let plane = put_vec4(&mut e, [0.0, 0.0, -1.0, 2.0]);
        let out = alloc_ptr(&mut e, 0x20);
        let info = alloc_ptr(&mut e, 8);
        e.mem.set_u8(info.addr(), 0x55);
        Closest {
            e,
            triangle,
            plane,
            out,
            info,
            calls,
        }
    }

    impl Closest {
        fn run(&mut self, point: [f32; 4], with_info: bool) -> u32 {
            let point = put_vec4(&mut self.e, point);
            let info = if with_info { self.info } else { Ptr::NULL };
            self.e
                .call(
                    0x00e6_2100,
                    &args![point, self.triangle, self.plane, self.out, info],
                )
                .u32()
        }

        fn info_bytes(&self) -> (u8, u16, u16, u16) {
            let i = self.info.addr();
            (
                self.e.mem.u8(i),
                self.e.mem.u16(i + 2),
                self.e.mem.u16(i + 4),
                self.e.mem.u16(i + 6),
            )
        }

        fn out_vec(&self) -> ([f32; 4], f32) {
            (
                load_vec4(&self.e, self.out.addr()),
                self.e.mem.f32(self.out.addr() + 0x10),
            )
        }
    }

    #[test]
    fn fn_00e62100_finds_the_face_when_all_three_tests_pass() {
        let mut world = closest_world(vec![]);
        // v = p1 - point = (-0.25, -0.25, -2); A = -db = 0.25, B = -da = 0.25.
        let answer = world.run([0.25, 0.25, 2.0, 0.0], true);
        assert_eq!(answer, 0);
        assert_eq!(world.info_bytes(), (3, 0, 1, 2));
        // plane.w * normal = 2 * (0, 0, 1); d = v . that = -4 is not positive, so the
        // distance word is negated and the vector stays.
        let (vector, distance) = world.out_vec();
        assert_eq!(vector[..3], [0.0, 0.0, 2.0]);
        assert_eq!(distance, 4.0);
        // The point below the plane: d = +4, the vector is negated and the distance stays.
        let mut world = closest_world(vec![]);
        let answer = world.run([0.25, 0.25, -2.0, 0.0], false);
        assert_eq!(answer, 0);
        let (vector, distance) = world.out_vec();
        assert_eq!(vector[..3], [-0.0, -0.0, -2.0]);
        assert!(vector[0].is_sign_negative());
        assert_eq!(distance, 4.0);
        // No record was asked for: the byte of `info` is untouched.
        assert_eq!(world.info_bytes().0, 0x55);
        assert!(world.calls.borrow().is_empty());
    }

    fn near(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() <= expected.abs() * 1e-6 + 1e-7
    }

    #[test]
    fn fn_00e62100_projects_onto_an_edge_when_the_tests_name_one() {
        // v = (-0.25, 0.5, -1): da = -0.25, db = 0.5: A = -0.5 (lane 0 fails), the other
        // two pass: outcome 6, table entry -8: the edge from vertex 2 (A) to vertex 1 (B).
        let point = [0.25, -0.5, 1.0, 0.0];
        for (code, expected_info) in [(4u32, (1u8, 2u16, 0u16)), (8, (1, 1, 0)), (0, (2, 2, 1))] {
            let mut world = closest_world(vec![(code, [0.0; 4])]);
            let answer = world.run(point, true);
            assert_eq!(answer, 1);
            let info = world.info_bytes();
            assert_eq!((info.0, info.1), (expected_info.0, expected_info.1));
            if expected_info.0 == 2 {
                assert_eq!(info.2, expected_info.2);
            }
            // The call: (point, vertex 1, vertex 2, out); the words after the point.
            let calls = world.calls.borrow();
            let call = &calls[0].1;
            let t = world.triangle.addr();
            assert_eq!(call[1], t + 0x10);
            assert_eq!(call[2], t + 0x20);
            drop(calls);
            // The closest point is the origin: out = point / |point|, distance |point|.
            let length = (0.25f32 * 0.25 + 0.25 + 1.0).sqrt();
            let (vector, distance) = world.out_vec();
            assert!(near(vector[0], 0.25 / length), "{vector:?}");
            assert!(near(vector[1], -0.5 / length));
            assert!(near(vector[2], 1.0 / length));
            assert!(near(distance, length));
        }
    }

    #[test]
    fn fn_00e62100_leaves_a_zero_vector_when_the_point_is_on_the_closest_point() {
        let mut world = closest_world(vec![(4, [0.25, -0.5, 1.0, 0.0])]);
        world.run([0.25, -0.5, 1.0, 0.0], false);
        let (vector, distance) = world.out_vec();
        assert_eq!(vector[..3], [0.0, 0.0, 0.0]);
        assert_eq!(distance, 0.0);
    }

    #[test]
    fn fn_00e62100_tries_both_edges_of_a_vertex_and_takes_the_nearer() {
        // v = (0.5, -2, z): da = 0.5, db = -2: A = 2 passes, B = -0.5 and the sum fail:
        // outcome 1, vertex 0, neighbours: next 1, previous 2.
        let point = [-0.5, 2.0, 1.0, 0.0];
        let t_of = |world: &Closest| world.triangle.addr();
        // The first edge's closest point is near the point: L1 < L2 takes the first.
        let mut world = closest_world(vec![(8, [-0.5, 2.0, 0.0, 0.0]), (4, [3.0, 3.0, 3.0, 0.0])]);
        assert_eq!(world.run(point, true), 1);
        let t = t_of(&world);
        {
            let calls = world.calls.borrow();
            assert_eq!(calls.len(), 2);
            // (point, vertex 0, next (1), out) and (point, previous (2), vertex 0, out).
            assert_eq!(calls[0].1[1], t);
            assert_eq!(calls[0].1[2], t + 0x10);
            assert_eq!(calls[1].1[1], t + 0x20);
            assert_eq!(calls[1].1[2], t);
        }
        // Result 8 on the first edge: the index of its other end (next = 1).
        assert_eq!(world.info_bytes().0, 1);
        assert_eq!(world.info_bytes().1, 1);
        let (vector, distance) = world.out_vec();
        assert_eq!(vector[..3], [0.0, 0.0, 1.0]);
        assert!(near(distance, 1.0));
        // The second edge nearer: the first is farther (L2 <= L1 or equal takes the second).
        let mut world = closest_world(vec![(4, [3.0, 3.0, 3.0, 0.0]), (0, [-0.5, 2.0, -1.0, 0.0])]);
        assert_eq!(world.run(point, true), 1);
        let info = world.info_bytes();
        // An inner point: count 2, the vertex then the previous neighbour (2).
        assert_eq!((info.0, info.1, info.2), (2, 0, 2));
        let (vector, distance) = world.out_vec();
        assert_eq!(vector[..3], [0.0, 0.0, 2.0 / 2.0]);
        assert!(near(distance, 2.0));
        // Equal distances: the second edge (the compare is "L2 > L1").
        let mut world = closest_world(vec![(4, [-0.5, 2.0, 0.0, 0.0]), (4, [-0.5, 2.0, 2.0, 0.0])]);
        world.run(point, true);
        assert_eq!(world.info_bytes().1, 0);
        assert_eq!(world.info_bytes().0, 1);
    }

    /// A shape like `make_shape` that also records the direction it was asked about.
    fn make_recording_shape(
        e: &mut Engine,
        support: u32,
        vtable: u32,
        scale: f32,
        seen: Rc<RefCell<Vec<[f32; 4]>>>,
    ) -> u32 {
        e.put_vtable(vtable, &[0, support]);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        e.register_double(support, move |e, a| {
            let direction = load_vec4(e, a[1]);
            seen.borrow_mut().push(direction);
            store_vec4(e, a[2], direction.map(|lane| lane * scale));
            Ret::default()
        });
        object
    }

    #[test]
    fn fn_00e62740_searches_the_penetration_direction_by_halving_steps() {
        let mut e = Engine::new();
        with_constants(&mut e);
        e.map(0x0126_8000, 0x1000);
        e.map(0x0104_e000, 0x1000);
        e.map(0x0106_d000, 0x1000);
        e.map(0x0101_6000, 0x2000);
        e.mem.set_f32(WORST_DISTANCE, -3.0e38);
        e.mem.set_f64(STEP_SCALE_DOUBLE, f64::from(0.99f32));
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f32(MINIMUM_STEP, 0.001);
        e.mem.set_f32(ZERO_FLOAT, 0.0);
        e.mem.set_f32(THREE_FLOAT, 3.0);
        e.mem.set_f32(HALF_FLOAT, 0.5);
        for (index, direction) in [[1.0f32, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
            .iter()
            .enumerate()
        {
            store_vec4(
                &mut e,
                START_DIRECTIONS + 0x10 * index as u32,
                [direction[0], direction[1], direction[2], 0.0],
            );
        }
        let seen_a = Rc::new(RefCell::new(Vec::new()));
        let machine = make_machine(&mut e);
        let m = machine.addr();
        let shape_a = make_recording_shape(
            &mut e,
            SHAPE_A_SUPPORT,
            0x0126_9400,
            10.0,
            Rc::clone(&seen_a),
        );
        let shape_b = e.mem.u32(m + 0x58);
        let out0 = alloc_ptr(&mut e, 16);
        let out1 = alloc_ptr(&mut e, 16);
        let out2 = alloc_ptr(&mut e, 0x20);
        e.call(
            0x00e6_2740,
            &args![
                Ptr::<()>::new(shape_a),
                Ptr::<()>::new(shape_b),
                machine,
                out0,
                out1,
                out2
            ],
        );
        // The rounds, from the step recurrence: u = float(carried * 0.99), then float(u * 0.5)
        // against 0.001.
        let mut rounds = 0;
        let mut carried = 1.0f32;
        loop {
            rounds += 1;
            let step = (f64::from(carried) * f64::from(0.99f32)) as f32;
            carried = (f64::from(step) * 0.5) as f32;
            if carried <= 0.001 {
                break;
            }
        }
        let seen = seen_a.borrow();
        // Six calls for the start directions (each twice), four per round after.
        assert_eq!(seen.len(), 6 + 4 * rounds);
        // The first object is asked about the negation of what the search passes in, and the
        // search passes in the negated start directions: so it sees the start directions.
        for (pair, direction) in [[1.0f32, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
            .iter()
            .enumerate()
        {
            for copy in 0..2 {
                assert_eq!(seen[2 * pair + copy][..3], direction[..]);
            }
        }
        // The best start direction: dist = -13 + t_k for t = (1, 2, 3): the last, so
        // d = (0, 0, -1) as passed (negated). The first round's four directions come out
        // of d and the unit perpendicular (0, -1, 0): u = 0.99 off to each side along n
        // and along d x n, all scaled by 1 / sqrt(1 + u^2).
        let scale = 1.0 / (1.0f32 + 0.99 * 0.99).sqrt();
        let expected = [
            [0.0, 0.99, 1.0],
            [0.0, -0.99, 1.0],
            [0.99, 0.0, 1.0],
            [-0.99, 0.0, 1.0],
        ];
        for (index, want) in expected.iter().enumerate() {
            let got = seen[6 + index];
            for lane in 0..3 {
                assert!(
                    (got[lane] - want[lane] * scale).abs() < 2e-6,
                    "round 1, vector {index}: {got:?} vs {want:?}"
                );
            }
        }
        // The outputs: the first point twice, the second point, the best direction and its
        // distance, which can only have grown from -10.
        let best = load_vec4(&e, out2.addr());
        assert!(best[3] > -10.0, "{best:?}");
        assert_eq!(
            load_vec4(&e, out2.addr() + 0x10),
            load_vec4(&e, out0.addr())
        );
    }

    /// The search machine of `fn_00e62d30`: the transform machine of `make_machine` (rows,
    /// translation at +0x30, shapes at +0x54 and +0x58) with the three arrays of vectors
    /// (+0x60 first supports, +0x64 second supports, +0x68 simplex points; room for eight
    /// each), the size at +0x6c, the threshold at +0x50 and the four thresholds at +0x40.
    fn make_search_machine(e: &mut Engine) -> Ptr {
        with_constants(e);
        e.map(0x0126_7000, 0x2000);
        e.map(0x010c_7000, 0x1000);
        e.map(0x010e_c000, 0x1000);
        e.map(0x0101_0000, 0x2_0000);
        e.mem.set_f32(ZERO_FLOAT, 0.0);
        e.mem.set_f32(THREE_FLOAT, 3.0);
        e.mem.set_f32(HALF_FLOAT, 0.5);
        for lane in 0..4 {
            e.mem.set_u32(0x0101_7d10 + 4 * lane, 0x7fff_ffff);
            e.mem.set_f32(0x010c_7200 + 4 * lane, 1.1920929e-7);
            e.mem.set_f32(0x010e_c7f0 + 4 * lane, 1.0);
            e.mem
                .set_u32(XYZ_MASK + 4 * lane, if lane < 3 { u32::MAX } else { 0 });
        }
        e.register(0x00ec_6040, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        let machine = make_machine(e);
        let m = machine.addr();
        for offset in [0x60u32, 0x64, 0x68] {
            let array = e.mem.alloc(0x80);
            e.mem.set_u32(m + offset, array);
        }
        e.mem.set_f32(m + 0x50, 0.001);
        e.mem.set_u32(m + 0x6c, 0);
        store_vec4(e, m + 0x40, [0.0; 4]);
        machine
    }

    fn put_points(e: &mut Engine, machine: Ptr, offset: u32, points: &[[f32; 3]]) {
        let array = e.mem.u32(machine.addr() + offset);
        for (index, point) in points.iter().enumerate() {
            store_vec4(
                e,
                array + 0x10 * index as u32,
                [point[0], point[1], point[2], 0.0],
            );
        }
    }

    fn run_search(e: &mut Engine, machine: Ptr) -> (u32, u32, Ptr) {
        let out = alloc_ptr(e, 0x20);
        let code = alloc_ptr(e, 4);
        e.mem.set_u32(code.addr(), 0xdead_beef);
        let answer = e.call(0x00e6_2d30, &args![machine, out, code]).u32();
        (answer, e.mem.u32(code.addr()), out)
    }

    /// Shapes whose support points never change: `first` and `second`.
    fn fixed_shapes(e: &mut Engine, first: [f32; 4], second: [f32; 4]) {
        for (support, point) in [(SHAPE_A_SUPPORT, first), (SHAPE_B_SUPPORT, second)] {
            e.register_double(support, move |e, a| {
                store_vec4(e, a[2], point);
                Ret::default()
            });
        }
    }

    #[test]
    fn fn_00e62d30_rejects_a_size_beyond_four() {
        let mut e = Engine::new();
        let machine = make_search_machine(&mut e);
        e.mem.set_u32(machine.addr() + 0x6c, 5);
        let (answer, code, _) = run_search(&mut e, machine);
        assert_eq!((answer, code), (1, 5));
    }

    #[test]
    fn fn_00e62d30_finds_the_origin_when_the_first_support_point_is_the_origin() {
        let mut e = Engine::new();
        let machine = make_search_machine(&mut e);
        let m = machine.addr();
        // The translation (1,0,0): the first support (1,0,0) - (1,0,0) = 0, so the origin.
        store_vec4(&mut e, m + 0x30, [1.0, 0.0, 0.0, 0.0]);
        fixed_shapes(&mut e, [1.0, 0.0, 0.0, 0.0], [0.0; 4]);
        let (answer, code, out) = run_search(&mut e, machine);
        assert_eq!((answer, code), (1, 0));
        assert_eq!(e.mem.u32(m + 0x6c), 1);
        let simplex = e.mem.u32(m + 0x68);
        assert_eq!(vec_at(&e, simplex), [0.0; 4]);
        // out = (negated start direction normalized, the length of the point = 0) and the
        // the first object's support point (the record's second vector) at +0x10.
        let result = vec_at(&e, out.addr());
        assert!(near(result[0], -1.0), "{result:?}");
        assert_eq!(result[1..3], [0.0, 0.0]);
        assert_eq!(result[3], 0.0);
        assert_eq!(vec_at(&e, out.addr() + 0x10), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_00e62d30_gives_up_when_the_support_point_never_moves() {
        let mut e = Engine::new();
        let machine = make_search_machine(&mut e);
        let m = machine.addr();
        store_vec4(&mut e, m + 0x30, [1.0, 2.0, 3.0, 0.0]);
        fixed_shapes(&mut e, [4.0, 0.0, 0.0, 0.0], [0.0; 4]);
        let (answer, code, _) = run_search(&mut e, machine);
        // D = (4,0,0) - (1,2,3) twice: a segment of length 0 at size 2 with the visited
        // count already 1: code 5.
        assert_eq!((answer, code), (1, 5));
        assert_eq!(e.mem.u32(m + 0x6c), 2);
        let simplex = e.mem.u32(m + 0x68);
        assert_eq!(vec_at(&e, simplex)[..3], [3.0, -2.0, -3.0]);
        assert_eq!(vec_at(&e, simplex + 0x10)[..3], [3.0, -2.0, -3.0]);
    }

    #[test]
    fn fn_00e62d30_reports_an_enclosing_tetrahedron() {
        let mut e = Engine::new();
        let machine = make_search_machine(&mut e);
        let m = machine.addr();
        put_points(
            &mut e,
            machine,
            0x68,
            &[
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
                [-1.0, -1.0, -1.0],
            ],
        );
        e.mem.set_u32(m + 0x6c, 4);
        let (answer, code, _) = run_search(&mut e, machine);
        assert_eq!((answer, code), (0, 0));
        assert_eq!(e.mem.u32(m + 0x6c), 4);
    }

    #[test]
    fn fn_00e62d30_finishes_inside_a_triangle_with_barycentric_weights() {
        let mut e = Engine::new();
        let machine = make_search_machine(&mut e);
        let m = machine.addr();
        put_points(
            &mut e,
            machine,
            0x68,
            &[[2.0, 0.0, 1.0], [-1.0, 1.0, 1.0], [-1.0, -1.0, 1.0]],
        );
        put_points(
            &mut e,
            machine,
            0x60,
            &[[3.0, 0.0, 0.0], [0.0, 6.0, 0.0], [0.0, 0.0, 9.0]],
        );
        e.mem.set_u32(m + 0x6c, 3);
        store_vec4(&mut e, m + 0x40, [100.0; 4]);
        let (answer, code, out) = run_search(&mut e, machine);
        assert_eq!((answer, code), (1, 0));
        let normal = vec_at(&e, out.addr());
        assert!(near(normal[2], 1.0), "{normal:?}");
        assert_eq!(normal[3], 0.0);
        // The origin is the centroid: weights 1/3 each; the point is (A0 + A1 + A2) / 3.
        let point = vec_at(&e, out.addr() + 0x10);
        for (lane, expected) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            assert!(near(point[lane], *expected), "{point:?}");
        }
    }

    #[test]
    fn fn_00e62d30_searches_perpendicular_to_a_segment() {
        let mut e = Engine::new();
        let machine = make_search_machine(&mut e);
        let m = machine.addr();
        put_points(&mut e, machine, 0x68, &[[1.0, 0.0, 0.5], [-1.0, 0.0, 0.5]]);
        e.mem.set_u32(m + 0x6c, 2);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let shape_a =
            make_recording_shape(&mut e, SHAPE_A_SUPPORT, 0x0126_9400, 10.0, Rc::clone(&seen));
        e.mem.set_u32(m + 0x54, shape_a);
        let _ = run_search(&mut e, machine);
        // The edge is (2, 0, 0): the smallest absolute components are y and z, the vector
        // (0, 0, -2) is perpendicular and normalizes to (0, 0, -1); the other direction is
        // (0, 1, 0). Each is offered as it is and then negated.
        let seen = seen.borrow();
        let expected = [
            [0.0f32, 0.0, -1.0],
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
        ];
        for (index, want) in expected.iter().enumerate() {
            for lane in 0..3 {
                assert!(
                    (seen[index][lane] - want[lane]).abs() < 2e-6,
                    "call {index}: {:?} vs {want:?}",
                    seen[index]
                );
            }
        }
    }

    #[test]
    fn fn_00e63770_returns_the_code_of_a_finished_step() {
        let mut e = Engine::new();
        let machine = make_search_machine(&mut e);
        let m = machine.addr();
        store_vec4(&mut e, m + 0x30, [1.0, 0.0, 0.0, 0.0]);
        fixed_shapes(&mut e, [1.0, 0.0, 0.0, 0.0], [0.0; 4]);
        let out = alloc_ptr(&mut e, 0x20);
        let code = e.call(0x00e6_3770, &args![machine, out]).u32();
        // The origin is the first support point: code 0, the restart count untouched.
        assert_eq!(code, 0);
        assert_eq!(e.mem.u32(m + 0x5c), 0);
        assert!(near(vec_at(&e, out.addr())[0], -1.0));
    }

    #[test]
    fn fn_00e63770_restarts_with_random_nudges_and_gives_up_after_twenty() {
        let mut e = Engine::new();
        let machine = make_search_machine(&mut e);
        let m = machine.addr();
        e.map(0x0103_2000, 0x1000);
        e.map(0x010d_b000, 0x1000);
        e.mem.set_f64(OFFSET_SUBTRACT_DOUBLE, 0.5);
        e.mem.set_f64(OFFSET_SCALE_DOUBLE, 0.002);
        store_vec4(&mut e, m + 0x30, [1.0, 2.0, 3.0, 0.0]);
        fixed_shapes(&mut e, [4.0, 0.0, 0.0, 0.0], [0.0; 4]);
        let out = alloc_ptr(&mut e, 0x20);
        let code = e.call(0x00e6_3770, &args![machine, out]).u32();
        assert_eq!(code, 5);
        assert_eq!(e.mem.u32(m + 0x5c), 0x14);
        // The first restart only sets the size; restarts 2 to 19 add
        // float((random - 0.5) * 0.002) to x, y and z.
        let mut expected = [1.0f32, 2.0, 3.0];
        for count in 2..0x14u32 {
            let scrambled = count.wrapping_mul(0x3e39_b193);
            let value = (0x3039u32.wrapping_sub(scrambled) & 0x7fff_ffff) as i32;
            let random = (f64::from(value) * (1.0 / 2_147_483_648.0)) as f32;
            let nudge = ((f64::from(random) - 0.5) * 0.002) as f32;
            for lane in expected.iter_mut() {
                *lane += nudge;
            }
        }
        let translation = vec_at(&e, m + 0x30);
        for lane in 0..3 {
            assert!(
                (translation[lane] - expected[lane]).abs() < 1e-6,
                "{translation:?} vs {expected:?}"
            );
        }
        assert_ne!(translation[0], 1.0);
    }

    #[test]
    fn fn_00e63770_walks_faces_and_ends_on_the_first_one_holding_the_origin() {
        let mut e = Engine::new();
        let machine = make_search_machine(&mut e);
        let m = machine.addr();
        e.map(0x0109_a000, 0x1000);
        e.mem.set_f64(0x0109_a3a0, 1.0e-7);
        e.map(0x0104_d000, 0x1000);
        e.mem.set_f32(INTERVAL_HIGH_FLOAT, f32::MAX);
        e.mem.set_f32(INTERVAL_LOW_FLOAT, 1.0e-7);
        // The enclosing tetrahedron: the step function answers 0 and the search proper runs.
        put_points(
            &mut e,
            machine,
            0x68,
            &[
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
                [-1.0, -1.0, -1.0],
            ],
        );
        e.mem.set_u32(m + 0x6c, 4);
        // The face: normal (0, 0, 1), three vertex records, a distance of 2.
        let face = e.mem.alloc(0x50);
        store_vec4(&mut e, face, [0.0, 0.0, 1.0, 0.0]);
        let records = [
            ([2.0f32, 0.0, 1.0], [3.0f32, 0.0, 0.0]),
            ([-1.0, 1.0, 1.0], [0.0, 6.0, 0.0]),
            ([-1.0, -1.0, 1.0], [0.0, 0.0, 9.0]),
        ];
        for (index, (point, support)) in records.iter().enumerate() {
            let record = e.mem.alloc(0x30);
            store_vec4(&mut e, record, [point[0], point[1], point[2], 0.0]);
            store_vec4(
                &mut e,
                record + 0x10,
                [support[0], support[1], support[2], 0.0],
            );
            store_vec4(&mut e, record + 0x20, [0.5; 4]);
            e.mem.set_u32(face + 0x10 * (index as u32 + 1), record);
        }
        e.mem.set_f32(face + 0x40, 2.0);
        let calls: Log = Rc::new(RefCell::new(Vec::new()));
        for address in [
            0x00e6_79d0u32,
            CURRENT_FACE,
            UPDATE_SIMPLEX,
            PRUNE_SIMPLEX,
            NEXT_FACE,
        ] {
            let sink = Rc::clone(&calls);
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                match address {
                    CURRENT_FACE | NEXT_FACE => Ret {
                        eax: face,
                        ..Ret::default()
                    },
                    UPDATE_SIMPLEX => {
                        // The count of entries reported, and the float.
                        e.mem.set_u32(a[3], 1);
                        e.mem.set_u32(a[4], 0);
                        Ret::default()
                    }
                    _ => Ret::default(),
                }
            });
        }
        let out = alloc_ptr(&mut e, 0x20);
        let code = e.call(0x00e6_3770, &args![machine, out]).u32();
        // The simplex grows from 4 entries to 0x37 (51 steps), then the entries outgrow the
        // budget: code 2. The first face holds the origin (weights 1/3 each).
        assert_eq!(code, 2);
        let log = calls.borrow();
        let count = |address: u32| log.iter().filter(|c| c.0 == address).count();
        assert_eq!(count(0x00e6_79d0), 1);
        assert_eq!(count(CURRENT_FACE), 0x37 - 4);
        assert_eq!(count(UPDATE_SIMPLEX), 0x37 - 4);
        assert_eq!(count(PRUNE_SIMPLEX), 0x37 - 4);
        assert_eq!(count(NEXT_FACE), 0);
        let result = vec_at(&e, out.addr());
        assert_eq!(result[..3], [-0.0, -0.0, -1.0]);
        assert_eq!(result[3], -2.0);
        let point = vec_at(&e, out.addr() + 0x10);
        for (lane, expected) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            assert!(near(point[lane], *expected), "{point:?}");
        }
        assert_eq!(e.mem.u32(m + 0x6c), 3);
        // The clip function stored its answer in the face.
        assert_eq!(e.mem.u32(face + 0x44), 1);
    }
}
