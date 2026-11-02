//! `NiAnimation/nibsplinedata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBSplineData::ms_RTTI` (the class's `NiRTTI` record).
const NI_B_SPLINE_DATA_RTTI: u32 = 0x011f_4034;

// Translated from 00a52500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_b_spline_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_B_SPLINE_DATA_RTTI)
}

/// `NiObject::LoadBinary` (`00a5d610`; `thiscall`: stream).
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// The `NiObject` destructor (`00a5d3d0`; `thiscall`, no stack word).
const OBJECT_DESTRUCTOR: u32 = 0x00a5_d3d0;
/// `00aa1070` and `00aa10f0`: the `cdecl` allocate (size) and free (block).
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;
/// `00aa1460`: the sized delete (`cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The `NiBSplineData` virtual table (`01099e54`).
const B_SPLINE_DATA_VTABLE: u32 = 0x0109_9e54;
/// `NiBSplineBasisData::GetDegree3Basis` (`00a62660`; `thiscall`, no stack word): the
/// address of the basis data's `NiBSplineBasis<float,3>`.
const GET_DEGREE3_BASIS: u32 = 0x00a6_2660;
/// `00a89650` (`cdecl`, six words): expands `count` 16-bit compact control values at
/// `source` into floats at `destination` with the offset and half range (source, count,
/// offset, half range, destination, count).
const EXPAND_COMPACT_VALUES: u32 = 0x00a8_9650;
/// `_ftol2` (`00ec62c0`): the value in ST0 truncated to an integer in EAX; passed here as
/// a leading `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// Constants the basis computation reads: the floats 0.5 (`01016248`) and 1/3
/// (`0106c2e0`), the doubles 1.0 (`01012070`), 2.0 (`01011590`), 3.0 (`01021928`), 0.5
/// (`01011588`) and 1/3 (`01099ee0`). Read from the exe's data at run time.
const HALF_FLOAT: u32 = 0x0101_6248;
const THIRD_FLOAT: u32 = 0x0106_c2e0;
const ONE: u32 = 0x0101_2070;
const TWO: u32 = 0x0101_1590;
const THREE: u32 = 0x0102_1928;
const HALF: u32 = 0x0101_1588;
const THIRD: u32 = 0x0109_9ee0;

layout! {
    /// `NiBSplineBasis<float,3>` (Xbox PDB), 0x20 bytes: the four basis values of the last
    /// time asked for and the control point range they apply to.
    pub struct NiBSplineBasisFloat3: 0x20 {
        /// `m_iQuantity` (Xbox PDB): number of control points.
        0x00 m_iQuantity: u32,
        /// `m_afValue` (Xbox PDB): the first of the four basis values.
        0x04 m_afValue: u32,
        /// `m_fLastTime` (Xbox PDB): compared as a 32-bit word.
        0x14 m_fLastTime: u32,
        /// `m_iMin` (Xbox PDB).
        0x18 m_iMin: u32,
        /// `m_iMax` (Xbox PDB).
        0x1C m_iMax: u32,
    }

    /// `NiBSplineData` (Xbox PDB), 0x18 bytes.
    pub struct NiBSplineData: 0x18 {
        /// `m_pafControlPoints` (Xbox PDB).
        0x08 m_pafControlPoints: u32,
        /// `m_pasCompactControlPoints` (Xbox PDB): 16-bit values.
        0x0C m_pasCompactControlPoints: u32,
        /// `m_uiControlPointCount` (Xbox PDB).
        0x10 m_uiControlPointCount: u32,
        /// `m_uiCompactControlPointCount` (Xbox PDB).
        0x14 m_uiCompactControlPointCount: u32,
    }
}

/// One read of `bytes` bytes into `destination` through the stream's input object
/// (`NiStream` `+0x24C`; its read function at `+8`, `cdecl` (input, destination, bytes,
/// address of the component size, 1)), the answer added to the input's word at `+4`.
fn stream_read(e: &mut Engine, stream: Ptr, destination: u32, bytes: u32, component: u32) {
    let input = e.mem.u32(stream.addr() + 0x24c);
    let read = e.mem.u32(input + 8);
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), component);
        e.call(read, &args![input, destination, bytes, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(input + 4).wrapping_add(answer);
    e.mem.set_u32(input + 4, total);
}

// Translated from 00a52870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineData::LoadBinary` (Xbox PDB; `thiscall`, `RET 4`: stream). After
/// `NiObject::LoadBinary` (`00a5d610`) reads the control point count (4 bytes) and, when
/// it is not zero, that many floats into a new block (`00aa1070`, count * 4 bytes); then
/// the compact count and, when not zero, that many 16-bit values (component size 2) into
/// a new block of count * 2 bytes.
pub fn ni_b_spline_data_load_binary(e: &mut Engine, this: Ptr<NiBSplineData>, stream: Ptr) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    stream_read(e, stream, this.addr() + 0x10, 4, 4);
    let count = e.get(this, NiBSplineData::m_uiControlPointCount);
    if count != 0 {
        let block = e.call(ALLOCATE, &args![count.wrapping_mul(4)]).u32();
        let bytes = e
            .get(this, NiBSplineData::m_uiControlPointCount)
            .wrapping_mul(4);
        e.set(this, NiBSplineData::m_pafControlPoints, block);
        stream_read(e, stream, block, bytes, 4);
    }
    stream_read(e, stream, this.addr() + 0x14, 4, 4);
    let count = e.get(this, NiBSplineData::m_uiCompactControlPointCount);
    if count != 0 {
        let block = e.call(ALLOCATE, &args![count.wrapping_mul(2)]).u32();
        let bytes = e
            .get(this, NiBSplineData::m_uiCompactControlPointCount)
            .wrapping_mul(2);
        e.set(this, NiBSplineData::m_pasCompactControlPoints, block);
        stream_read(e, stream, block, bytes, 2);
    }
}

// Translated from 00a52510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineData`'s scalar deleting destructor (Xbox PDB; `thiscall`, `RET 4`): stores the
/// class's virtual table, frees both control point blocks (`00aa10f0`, also when null),
/// runs the `NiObject` destructor (`00a5d3d0`) and, when bit 0 of `flags` is set, frees the
/// 0x18 bytes (`00aa1460`). Answers `this`.
pub fn ni_b_spline_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiBSplineData>,
    flags: u32,
) -> Ptr<NiBSplineData> {
    let points = e.get(this, NiBSplineData::m_pafControlPoints);
    e.mem.set_u32(this.addr(), B_SPLINE_DATA_VTABLE);
    e.call(FREE, &args![points]);
    let compact = e.get(this, NiBSplineData::m_pasCompactControlPoints);
    e.call(FREE, &args![compact]);
    e.call(OBJECT_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00a51e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineBasis<float,3>::Compute` (Xbox PDB; `thiscall`, `RET 0xC`: time, address of
/// the first index, address of the last index). When `time` has the bit pattern of the
/// last time computed, only the stored range is answered. Otherwise the time is kept, the
/// last index is `_ftol2((quantity - 3) * time) + 3` for a time below 1 (`n - 1` else,
/// NaN included), the first index three less, both stored and answered, and the four basis
/// values for the scaled time `(quantity - 3) * time` (rounded through floats) are computed
/// on the x87 stack instruction by instruction: the general uniform-knot formula for seven
/// control points or more, the special cases of six, five and four, nothing for fewer.
pub fn ni_b_spline_basis_float3_compute(
    e: &mut Engine,
    this: Ptr<NiBSplineBasisFloat3>,
    time: f32,
    first_out: Ptr,
    last_out: Ptr,
) {
    let time_bits = time.to_bits();
    if time_bits == e.get(this, NiBSplineBasisFloat3::m_fLastTime) {
        let first = e.get(this, NiBSplineBasisFloat3::m_iMin);
        e.mem.set_u32(first_out.addr(), first);
        let last = e.get(this, NiBSplineBasisFloat3::m_iMax);
        e.mem.set_u32(last_out.addr(), last);
        return;
    }
    let quantity = e.get(this, NiBSplineBasisFloat3::m_iQuantity) as i32;
    let t = x87_load32(time_bits);
    e.set(this, NiBSplineBasisFloat3::m_fLastTime, time_bits);
    let span = quantity.wrapping_sub(3);
    // FCOM of the time with 1.0: below 1 (ordered) takes the truncated product.
    let last = if t < 1.0 {
        let product = x87_mul(f64::from(span), t);
        e.call(FTOL, &args![product]).i32().wrapping_add(3)
    } else {
        quantity.wrapping_sub(1)
    };
    let first = last.wrapping_sub(3);
    e.mem.set_u32(first_out.addr(), first as u32);
    e.set(this, NiBSplineBasisFloat3::m_iMin, first as u32);
    let span_float = x87_load32(x87_store32(f64::from(span)));
    e.mem.set_u32(last_out.addr(), last as u32);
    let u = x87_store32(x87_mul(t, span_float));
    e.set(this, NiBSplineBasisFloat3::m_iMax, last as u32);
    // The quantity is read again for the choice of formula; the general case keeps the
    // first reading's `n - 3`, `n - 2` and `n - 1` for its comparisons.
    let quantity_again = e.get(this, NiBSplineBasisFloat3::m_iQuantity) as i32;
    if quantity_again >= 7 {
        compute_general(e, this, u, first, last, span, span_float);
        return;
    }
    match quantity_again {
        6 => match last {
            3 => {
                let x1 = x87_load32(u);
                let x2 = x87_sub(1.0, x1);
                let s1 = x87_store32(x2);
                let x3 = x87_sub(f64::from_bits(e.mem.u64(TWO)), x1);
                let s2 = x87_store32(x3);
                let x4 = x87_load64(e.mem.u64(HALF));
                let x5 = x87_mul(x1, x4);
                let s3 = x87_store32(x5);
                let x6 = x87_load32(s1);
                let x7 = x87_mul(x6, x6);
                let s4 = x87_store32(x7);
                let x8 = x87_load32(s3);
                let x9 = x87_load32(s2);
                let x10 = x87_mul(x8, x9);
                let x11 = x87_mul(x6, x1);
                let x12 = x87_add(x10, x11);
                let x13 = x87_mul(x4, x12);
                let s5 = x87_store32(x13);
                let x14 = x87_mul(x1, f64::from_bits(e.mem.u64(THIRD)));
                let x15 = x87_mul(x8, x14);
                let s6 = x87_store32(x15);
                let x16 = x87_load32(s4);
                let x17 = x87_mul(x6, x16);
                set_value(e, this, 0, x17);
                let x18 = x87_load32(s5);
                let x19 = x87_mul(x9, x18);
                let x20 = x87_mul(x16, x1);
                let x21 = x87_add(x20, x19);
                set_value(e, this, 1, x21);
                let x22 = x87_sub(f64::from_bits(e.mem.u64(THREE)), x1);
                let s7 = x87_store32(x22);
                let x23 = x87_load32(s7);
                let x24 = x87_load32(s6);
                let x25 = x87_mul(x23, x24);
                let x26 = x87_mul(x18, x1);
                let x27 = x87_add(x26, x25);
                set_value(e, this, 2, x27);
                let x28 = x87_mul(x1, x24);
                set_value(e, this, 3, x28);
            }
            4 => {
                let x1 = x87_load32(u);
                let x2 = x87_sub(x1, 1.0);
                let s1 = x87_store32(x2);
                let x3 = x87_sub(f64::from_bits(e.mem.u64(THREE)), x1);
                let s2 = x87_store32(x3);
                let x4 = x87_load64(e.mem.u64(HALF));
                let x5 = x87_mul(x1, x4);
                let s3 = x87_store32(x5);
                let x6 = x87_load32(s3);
                let x7 = x87_sub(1.0, x6);
                let s4 = x87_store32(x7);
                let x8 = x87_load32(s1);
                let x9 = x87_mul(x4, x8);
                let s5 = x87_store32(x9);
                let x10 = x87_load32(s4);
                let x11 = x87_mul(x10, x10);
                let s6 = x87_store32(x11);
                let x12 = x87_load32(s5);
                let x13 = x87_load32(s2);
                let x14 = x87_mul(x12, x13);
                let x15 = x87_mul(x10, x1);
                let x16 = x87_add(x15, x14);
                let x17 = x87_mul(x16, f64::from_bits(e.mem.u64(THIRD)));
                let s7 = x87_store32(x17);
                let x18 = x87_mul(x12, x12);
                let s8 = x87_store32(x18);
                let x19 = x87_sub(f64::from_bits(e.mem.u64(TWO)), x1);
                let s9 = x87_store32(x19);
                let x20 = x87_load32(s9);
                let x21 = x87_load32(s6);
                let x22 = x87_mul(x20, x21);
                set_value(e, this, 0, x22);
                let x23 = x87_load32(s7);
                let x24 = x87_mul(x23, x13);
                let x25 = x87_mul(x21, x1);
                let x26 = x87_add(x25, x24);
                set_value(e, this, 1, x26);
                let x27 = x87_load32(s8);
                let x28 = x87_mul(x13, x27);
                let x29 = x87_mul(x1, x23);
                let x30 = x87_add(x29, x28);
                set_value(e, this, 2, x30);
                let x31 = x87_mul(x27, x8);
                set_value(e, this, 3, x31);
            }
            _ => {
                let x1 = x87_load32(u);
                let x2 = x87_load64(e.mem.u64(ONE));
                let x3 = x87_sub(x1, x2);
                let s1 = x87_store32(x3);
                let x4 = x87_load64(e.mem.u64(TWO));
                let x5 = x87_sub(x1, x4);
                let s2 = x87_store32(x5);
                let x6 = x87_sub(f64::from_bits(e.mem.u64(THREE)), x1);
                let s3 = x87_store32(x6);
                let x7 = x87_load32(s3);
                let x8 = x87_load64(e.mem.u64(HALF));
                let x9 = x87_mul(x7, x8);
                let s4 = x87_store32(x9);
                let x10 = x87_load32(s2);
                let x11 = x87_mul(x10, x10);
                let s5 = x87_store32(x11);
                let x12 = x87_mul(x7, f64::from_bits(e.mem.u64(THIRD)));
                let x13 = x87_load32(s4);
                let x14 = x87_mul(x12, x13);
                let s6 = x87_store32(x14);
                let x15 = x87_mul(x7, x10);
                let x16 = x87_load32(s1);
                let x17 = x87_mul(x13, x16);
                let x18 = x87_add(x17, x15);
                let x19 = x87_mul(x8, x18);
                let s7 = x87_store32(x19);
                let x20 = x87_load32(s6);
                let x21 = x87_mul(x20, x7);
                set_value(e, this, 0, x21);
                let x22 = x87_load32(s7);
                let x23 = x87_mul(x22, x7);
                let x24 = x87_mul(x1, x20);
                let x25 = x87_add(x24, x23);
                set_value(e, this, 1, x25);
                let x26 = x87_load32(s5);
                let x27 = x87_mul(x7, x26);
                let x28 = x87_mul(x16, x22);
                let x29 = x87_add(x28, x27);
                set_value(e, this, 2, x29);
                let x30 = x87_mul(x26, x10);
                set_value(e, this, 3, x30);
            }
        },
        5 => {
            if last == 3 {
                let x1 = x87_load32(u);
                let x2 = x87_sub(1.0, x1);
                let s1 = x87_store32(x2);
                let x3 = x87_sub(f64::from_bits(e.mem.u64(TWO)), x1);
                let s2 = x87_store32(x3);
                let x4 = x87_load64(e.mem.u64(HALF));
                let x5 = x87_mul(x1, x4);
                let s3 = x87_store32(x5);
                let x6 = x87_load32(s1);
                let x7 = x87_mul(x6, x6);
                let s4 = x87_store32(x7);
                let x8 = x87_load32(s3);
                let x9 = x87_load32(s2);
                let x10 = x87_mul(x8, x9);
                let x11 = x87_mul(x6, x1);
                let x12 = x87_add(x10, x11);
                let x13 = x87_mul(x4, x12);
                let s5 = x87_store32(x13);
                let x14 = x87_mul(x8, x8);
                let s6 = x87_store32(x14);
                let x15 = x87_load32(s4);
                let x16 = x87_mul(x6, x15);
                set_value(e, this, 0, x16);
                let x17 = x87_load32(s5);
                let x18 = x87_mul(x17, x9);
                let x19 = x87_mul(x15, x1);
                let x20 = x87_add(x19, x18);
                set_value(e, this, 1, x20);
                let x21 = x87_load32(s6);
                let x22 = x87_mul(x9, x21);
                let x23 = x87_mul(x17, x1);
                let x24 = x87_add(x23, x22);
                set_value(e, this, 2, x24);
                let x25 = x87_mul(x1, x21);
                set_value(e, this, 3, x25);
            } else {
                let x1 = x87_load32(u);
                let x2 = x87_sub(x1, 1.0);
                let s1 = x87_store32(x2);
                let x3 = x87_sub(f64::from_bits(e.mem.u64(TWO)), x1);
                let s2 = x87_store32(x3);
                let x4 = x87_mul(x1, f64::from_bits(e.mem.u64(HALF)));
                let s3 = x87_store32(x4);
                let x5 = x87_load32(s1);
                let x6 = x87_mul(x5, x5);
                let s4 = x87_store32(x6);
                let x7 = x87_load32(s3);
                let x8 = x87_sub(1.0, x7);
                let s5 = x87_store32(x8);
                let x9 = x87_load32(s5);
                let x10 = x87_mul(x9, x9);
                let s6 = x87_store32(x10);
                let x11 = x87_add(x7, x5);
                let x12 = x87_mul(x11, x9);
                let s7 = x87_store32(x12);
                let x13 = x87_load32(s6);
                let x14 = x87_load32(s2);
                let x15 = x87_mul(x13, x14);
                set_value(e, this, 0, x15);
                let x16 = x87_load32(s7);
                let x17 = x87_mul(x16, x14);
                let x18 = x87_mul(x13, x1);
                let x19 = x87_add(x18, x17);
                set_value(e, this, 1, x19);
                let x20 = x87_load32(s4);
                let x21 = x87_mul(x14, x20);
                let x22 = x87_mul(x1, x16);
                let x23 = x87_add(x22, x21);
                set_value(e, this, 2, x23);
                let x24 = x87_mul(x20, x5);
                set_value(e, this, 3, x24);
            }
        }
        4 => {
            let x1 = x87_load32(u);
            let x2 = x87_sub(1.0, x1);
            let s1 = x87_store32(x2);
            let x3 = x87_mul(x1, x1);
            let s2 = x87_store32(x3);
            let x4 = x87_load32(s1);
            let x5 = x87_mul(x4, x4);
            let s3 = x87_store32(x5);
            let x6 = x87_load32(s3);
            let x7 = x87_mul(x6, x4);
            set_value(e, this, 0, x7);
            let x8 = x87_load64(e.mem.u64(THREE));
            let x9 = x87_mul(x1, x8);
            let x10 = x87_mul(x6, x9);
            set_value(e, this, 1, x10);
            let x11 = x87_load32(s2);
            let x12 = x87_mul(x8, x11);
            let x13 = x87_mul(x4, x12);
            set_value(e, this, 2, x13);
            let x14 = x87_mul(x1, x11);
            set_value(e, this, 3, x14);
        }
        // Fewer than four control points (or a negative count): no basis values.
        _ => {}
    }
}

/// `FSTP dword` of `value` into basis value `index` (`+4 + 4 * index`).
fn set_value(e: &mut Engine, this: Ptr<NiBSplineBasisFloat3>, index: u32, value: f64) {
    e.mem
        .set_u32(this.addr() + 4 + 4 * index, x87_store32(value));
}

/// The general case of [`ni_b_spline_basis_float3_compute`] (seven control points or
/// more), `u` the scaled time's float bits. The selections the exe makes on the x87 stack
/// (the knot distances near the ends and the weights 1, 0.5 and 1/3) are stored as floats
/// first, then combined in the exe's order.
fn compute_general(
    e: &mut Engine,
    this: Ptr<NiBSplineBasisFloat3>,
    u: u32,
    first: i32,
    last: i32,
    span: i32,
    span_float: f64,
) {
    let second_last = span.wrapping_add(1);
    let final_index = second_last.wrapping_add(1);
    let half = x87_load32(e.global::<u32>(HALF_FLOAT));
    let third = x87_load32(e.global::<u32>(THIRD_FLOAT));
    let from_five = x87_store32(if last > 5 {
        f64::from(last.wrapping_sub(5))
    } else {
        0.0
    });
    let from_four = x87_store32(if last > 4 {
        f64::from(last.wrapping_sub(4))
    } else {
        0.0
    });
    let from_min = x87_store32(f64::from(first));
    let from_max_less_two = x87_store32(f64::from(last.wrapping_sub(2)));
    let upper = x87_store32(if last >= second_last {
        span_float
    } else {
        f64::from(last.wrapping_sub(1))
    });
    let upper_end = x87_store32(if last >= span {
        span_float
    } else {
        f64::from(last)
    });
    let weight_low = x87_store32(if last == 3 { 1.0 } else { half });
    let weight_high = x87_store32(if last == final_index { 1.0 } else { half });
    let weight_first = x87_store32(match last {
        3 => 1.0,
        4 => half,
        _ => third,
    });
    let weight_middle = x87_store32(if last == 3 || last == final_index {
        half
    } else {
        third
    });
    let weight_last = x87_store32(if last == final_index {
        1.0
    } else if last == second_last {
        half
    } else {
        third
    });
    let x1 = x87_load32(u);
    let x2 = x87_sub(x1, x87_m32(from_four));
    let s1 = x87_store32(x2);
    let x3 = x87_sub(x1, x87_m32(from_min));
    let s2 = x87_store32(x3);
    let x4 = x87_load32(from_max_less_two);
    let x5 = x87_sub(x4, x1);
    let s3 = x87_store32(x5);
    let x6 = x87_load32(upper);
    let x7 = x87_sub(x6, x1);
    let s4 = x87_store32(x7);
    let x8 = x87_load32(s3);
    let x9 = x87_mul(x8, x87_m32(weight_low));
    let s5 = x87_store32(x9);
    let x10 = x87_load32(s2);
    let x11 = x87_mul(x10, x87_m32(weight_high));
    let s6 = x87_store32(x11);
    let x12 = x87_mul(x8, x87_m32(weight_first));
    let x13 = x87_load32(s5);
    let x14 = x87_mul(x12, x13);
    let s7 = x87_store32(x14);
    let x15 = x87_load32(s6);
    let x16 = x87_load32(s4);
    let x17 = x87_mul(x15, x16);
    let x18 = x87_load32(s1);
    let x19 = x87_mul(x13, x18);
    let x20 = x87_add(x19, x17);
    let x21 = x87_load32(weight_middle);
    let x22 = x87_mul(x20, x21);
    let s8 = x87_store32(x22);
    let x23 = x87_mul(x10, x87_m32(weight_last));
    let x24 = x87_mul(x15, x23);
    let s9 = x87_store32(x24);
    let x25 = x87_load32(s7);
    let x26 = x87_mul(x8, x25);
    set_value(e, this, 0, x26);
    let x27 = x87_sub(x1, x87_m32(from_five));
    let s10 = x87_store32(x27);
    let x28 = x87_load32(s10);
    let x29 = x87_mul(x25, x28);
    let x30 = x87_load32(s8);
    let x31 = x87_mul(x16, x30);
    let x32 = x87_add(x31, x29);
    set_value(e, this, 1, x32);
    let x33 = x87_load32(upper_end);
    let x34 = x87_sub(x33, x1);
    let s11 = x87_store32(x34);
    let x35 = x87_load32(s11);
    let x36 = x87_load32(s9);
    let x37 = x87_mul(x35, x36);
    let x38 = x87_load32(s1);
    let x39 = x87_mul(x30, x38);
    let x40 = x87_add(x39, x37);
    set_value(e, this, 2, x40);
    let x41 = x87_mul(x10, x36);
    set_value(e, this, 3, x41);
}

// Translated from 00a526b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineData::GetCompactedValueDegree3` (Xbox PDB; `thiscall`, `RET 0x1C`: time,
/// output, dimension, basis data, control point handle, offset, half range). Computes the
/// degree-3 basis of the basis data (`00a62660`, then
/// [`ni_b_spline_basis_float3_compute`], the first index written into the basis-data
/// argument's own word), expands the four control points from the compact array
/// (`00a89650`, from `compact + 2 * (first * dimension + handle)`, `4 * dimension` values,
/// into a 0x40-byte stack buffer) and writes `dimension` floats: the first basis value
/// times the first point, then each further basis value times the next point added on
/// (x87, one rounding to float per output word and step).
pub fn ni_b_spline_data_get_compacted_value_degree3(
    e: &mut Engine,
    this: Ptr<NiBSplineData>,
    time: f32,
    output: Ptr,
    dimension: u32,
    basis_data: Ptr,
    handle: u32,
    offset: f32,
    half_range: f32,
) {
    let basis = e.call(GET_DEGREE3_BASIS, &args![basis_data]).u32();
    e.with_stack(0x48, |e, frame| {
        // frame + 0: the basis-data argument's word (reused for the first index),
        // frame + 4: the last index, frame + 8: the expanded control values.
        let first_slot = frame.addr();
        let last_slot = first_slot + 4;
        let buffer = first_slot + 8;
        e.mem.set_u32(first_slot, basis_data.addr());
        ni_b_spline_basis_float3_compute(
            e,
            Ptr::new(basis),
            f32::from_bits(x87_quiet_bits(time.to_bits())),
            Ptr::new(first_slot),
            Ptr::new(last_slot),
        );
        let compact = e.get(this, NiBSplineData::m_pasCompactControlPoints);
        let first = e.mem.u32(first_slot);
        let index = first.wrapping_mul(dimension).wrapping_add(handle);
        let source = compact.wrapping_add(index.wrapping_mul(2));
        let count = dimension.wrapping_mul(4);
        e.call(
            EXPAND_COMPACT_VALUES,
            &args![
                source,
                count,
                x87_quiet_bits(offset.to_bits()),
                x87_quiet_bits(half_range.to_bits()),
                buffer,
                count
            ],
        );
        let out = output.addr();
        let weight = x87_load32(e.mem.u32(basis + 4));
        let mut point = 0u32;
        for item in 0..dimension {
            let value = x87_load32(e.mem.u32(buffer.wrapping_add(point.wrapping_mul(4))));
            let product = x87_mul(value, weight);
            e.mem
                .set_u32(out.wrapping_add(item.wrapping_mul(4)), x87_store32(product));
            point = point.wrapping_add(1);
        }
        let first = e.mem.u32(first_slot) as i32;
        let last = e.mem.u32(last_slot) as i32;
        let next = first.wrapping_add(1);
        if next > last {
            return;
        }
        let mut remaining = (last as u32).wrapping_sub(next as u32).wrapping_add(1);
        let mut weight_address = basis
            .wrapping_add((next.wrapping_sub(first) as u32).wrapping_mul(4))
            .wrapping_add(4);
        loop {
            let weight = x87_load32(e.mem.u32(weight_address));
            for item in 0..dimension {
                let value = x87_load32(e.mem.u32(buffer.wrapping_add(point.wrapping_mul(4))));
                let target = out.wrapping_add(item.wrapping_mul(4));
                let sum = x87_add(x87_mul(value, weight), x87_m32(e.mem.u32(target)));
                e.mem.set_u32(target, x87_store32(sum));
                point = point.wrapping_add(1);
            }
            weight_address = weight_address.wrapping_add(4);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    });
}

/// `NiObject::IsEqual` (`00a5d470`, engine map; `thiscall`, one word), the
/// base `SaveBinary` (`00a89af0`, `thiscall`, one word) and
/// `NiObject::GetViewerStrings` (`00a5d6e0`, engine map; `thiscall`, one
/// word).
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`) and its unsigned
/// overload (`00a7d510`, `cdecl (name, value)`); the viewer-string array's
/// grow (`0096ad30`, `thiscall (array; new capacity)`) and store
/// (`0096ae90`, `thiscall (array; index, &string)`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_uiControlPointCount"` and `"m_uiCompactControlPointCount"`.
const NAME_CONTROL_POINT_COUNT: u32 = 0x0109_9f08;
const NAME_COMPACT_CONTROL_POINT_COUNT: u32 = 0x0109_9ee8;

/// One write of `bytes` bytes from `source` through the stream's output
/// object (`NiStream` +0x250; its write function at +0xC, `cdecl (output,
/// source, bytes, address of the component size, 1)`), the answer added to
/// the output's word at +4.
fn stream_write(e: &mut Engine, stream: Ptr, source: u32, bytes: u32, component: u32) {
    let output = e.mem.u32(stream.addr() + 0x250);
    let write = e.mem.u32(output + 0xc);
    let answer = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), component);
        e.call(write, &args![output, source, bytes, size, 1u32])
            .u32()
    });
    let total = e.mem.u32(output + 4).wrapping_add(answer);
    e.mem.set_u32(output + 4, total);
}

/// Appends `text` to a viewer-string array as the exe's inlined `Add`
/// does: when the 16-bit size (+0xA) has reached the capacity (+0x8) the
/// array grows to the step (+0xE) plus the size, then the string is stored
/// at the old size (through a stack slot holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a51e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineData::IsEqual` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the other object): `NiObject::IsEqual` and the
/// same control point and compact control point counts.
pub fn ni_b_spline_data_is_equal(
    e: &mut Engine,
    this: Ptr<NiBSplineData>,
    other: Ptr<NiBSplineData>,
) -> bool {
    e.call(OBJECT_IS_EQUAL, &args![this, other]).bool()
        && e.get(other, NiBSplineData::m_uiControlPointCount)
            == e.get(this, NiBSplineData::m_uiControlPointCount)
        && e.get(other, NiBSplineData::m_uiCompactControlPointCount)
            == e.get(this, NiBSplineData::m_uiCompactControlPointCount)
}

// Translated from 00a52550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineData::GetValueDegree3` (Xbox PDB; `thiscall`, `RET 0x14`:
/// time, output, dimension, basis data, control point handle). Computes the
/// degree-3 basis of the basis data (`00a62660`, then
/// [`ni_b_spline_basis_float3_compute`], the first index written into the
/// time argument's own word and the last into the word the entry `PUSH
/// ECX` made) and writes `dimension` floats from the control points at
/// `points + 4 * (first * dimension + handle)`: the first basis value times
/// the first point, then each further basis value times the next point
/// added on (x87 `FMUL ST0,ST1` and `FADD`, one rounding to float per
/// output word and step; the exe's four-at-a-time loop writes the same
/// words in the same order).
pub fn ni_b_spline_data_get_value_degree3(
    e: &mut Engine,
    this: Ptr<NiBSplineData>,
    time: f32,
    output: Ptr,
    dimension: u32,
    basis_data: Ptr,
    handle: u32,
) {
    let basis = e.call(GET_DEGREE3_BASIS, &args![basis_data]).u32();
    e.with_stack(8, |e, frame| {
        // frame + 0: the time argument's word (reused for the first
        // index), frame + 4: the entry's `PUSH ECX` word (the last index).
        let first_slot = frame.addr();
        let last_slot = first_slot + 4;
        e.mem.set_u32(first_slot, time.to_bits());
        e.mem.set_u32(last_slot, this.addr());
        ni_b_spline_basis_float3_compute(
            e,
            Ptr::new(basis),
            f32::from_bits(x87_quiet_bits(time.to_bits())),
            Ptr::new(first_slot),
            Ptr::new(last_slot),
        );
        let weight = x87_load32(e.mem.u32(basis + 4));
        let first = e.mem.u32(first_slot);
        let points = e.get(this, NiBSplineData::m_pafControlPoints);
        let index = first.wrapping_mul(dimension).wrapping_add(handle);
        let mut source = points.wrapping_add(index.wrapping_mul(4));
        let out = output.addr();
        for item in 0..dimension {
            let value = x87_load32(e.mem.u32(source));
            let product = x87_mul(value, weight);
            e.mem
                .set_u32(out.wrapping_add(item.wrapping_mul(4)), x87_store32(product));
            source = source.wrapping_add(4);
        }
        let last = e.mem.u32(last_slot) as i32;
        let first = first as i32;
        let next = first.wrapping_add(1);
        if next > last {
            return;
        }
        let mut remaining = (last as u32).wrapping_sub(next as u32).wrapping_add(1);
        let mut weight_address = basis
            .wrapping_add((next.wrapping_sub(first) as u32).wrapping_mul(4))
            .wrapping_add(4);
        loop {
            let weight = x87_load32(e.mem.u32(weight_address));
            for item in 0..dimension {
                let value = x87_load32(e.mem.u32(source));
                let target = out.wrapping_add(item.wrapping_mul(4));
                let sum = x87_add(x87_mul(value, weight), x87_m32(e.mem.u32(target)));
                e.mem.set_u32(target, x87_store32(sum));
                source = source.wrapping_add(4);
            }
            weight_address = weight_address.wrapping_add(4);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    });
}

// Translated from 00a52950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineData::SaveBinary` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the stream): the base `SaveBinary` (`00a89af0`),
/// the control point count (4 bytes) and, when not zero, the control
/// points (count * 4 bytes, component size 4); then the compact count and,
/// when not zero, the compact points (count * 2 bytes, component size 2).
pub fn ni_b_spline_data_save_binary(e: &mut Engine, this: Ptr<NiBSplineData>, stream: Ptr) {
    e.call(OBJECT_SAVE_BINARY, &args![this, stream]);
    stream_write(e, stream, this.addr() + 0x10, 4, 4);
    let count = e.get(this, NiBSplineData::m_uiControlPointCount);
    if count != 0 {
        let points = e.get(this, NiBSplineData::m_pafControlPoints);
        stream_write(e, stream, points, count.wrapping_mul(4), 4);
    }
    stream_write(e, stream, this.addr() + 0x14, 4, 4);
    let count = e.get(this, NiBSplineData::m_uiCompactControlPointCount);
    if count != 0 {
        let compact = e.get(this, NiBSplineData::m_pasCompactControlPoints);
        stream_write(e, stream, compact, count.wrapping_add(count), 2);
    }
}

// Translated from 00a52a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBSplineData::GetViewerStrings` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the strings array): `NiObject::GetViewerStrings`
/// (`00a5d6e0`), then the class name (the first word of the `NiRTTI`
/// record), `m_uiControlPointCount` and `m_uiCompactControlPointCount`,
/// each appended.
pub fn ni_b_spline_data_get_viewer_strings(e: &mut Engine, this: Ptr<NiBSplineData>, strings: Ptr) {
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_B_SPLINE_DATA_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let count = e.get(this, NiBSplineData::m_uiControlPointCount);
    let text = e
        .call(
            NI_GET_VIEWER_STRING_UNSIGNED,
            &args![NAME_CONTROL_POINT_COUNT, count],
        )
        .u32();
    push_viewer_string(e, strings, text);
    let count = e.get(this, NiBSplineData::m_uiCompactControlPointCount);
    let text = e
        .call(
            NI_GET_VIEWER_STRING_UNSIGNED,
            &args![NAME_COMPACT_CONTROL_POINT_COUNT, count],
        )
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_1e50,
            ni_b_spline_data_is_equal(Ptr<NiBSplineData>, Ptr<NiBSplineData>) -> bool
        ),
        entry!(
            0x00a5_2550,
            ni_b_spline_data_get_value_degree3(Ptr<NiBSplineData>, f32, Ptr, u32, Ptr, u32)
        ),
        entry!(
            0x00a5_2950,
            ni_b_spline_data_save_binary(Ptr<NiBSplineData>, Ptr)
        ),
        entry!(
            0x00a5_2a20,
            ni_b_spline_data_get_viewer_strings(Ptr<NiBSplineData>, Ptr)
        ),
        entry!(0x00a52500, ni_b_spline_data_get_rtti() -> Ptr),
        entry!(
            0x00a5_2870,
            ni_b_spline_data_load_binary(Ptr<NiBSplineData>, Ptr)
        ),
        entry!(
            0x00a5_2510,
            ni_b_spline_data_scalar_deleting_destructor(
                Ptr<NiBSplineData>,
                u32,
            ) -> Ptr<NiBSplineData>
        ),
        entry!(
            0x00a5_1e80,
            ni_b_spline_basis_float3_compute(Ptr<NiBSplineBasisFloat3>, f32, Ptr, Ptr)
        ),
        entry!(
            0x00a5_26b0,
            ni_b_spline_data_get_compacted_value_degree3(
                Ptr<NiBSplineData>,
                f32,
                Ptr,
                u32,
                Ptr,
                u32,
                f32,
                f32,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_b_spline_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a52500, &args![]).u32(), NI_B_SPLINE_DATA_RTTI);
    }
}

#[cfg(test)]
mod b037_tests {
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

    /// The exe's constants the basis computation reads.
    fn constants(e: &mut Engine) {
        for (address, value) in [
            (ONE, 1.0f64),
            (TWO, 2.0),
            (THREE, 3.0),
            (HALF, 0.5),
            (THIRD, 1.0 / 3.0),
        ] {
            e.map(address, 8);
            e.mem.set_u64(address, value.to_bits());
        }
        e.map(HALF_FLOAT, 4);
        e.set_global(HALF_FLOAT, 0.5f32.to_bits());
        e.map(THIRD_FLOAT, 4);
        e.set_global(THIRD_FLOAT, 0x3eaa_aaabu32);
    }

    fn basis(e: &mut Engine, quantity: u32) -> u32 {
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, quantity);
        e.mem.set_u32(this + 0x14, 0xffff_ffff);
        this
    }

    fn compute(e: &mut Engine, this: u32, time: f32) -> (i32, i32, [f32; 4]) {
        let slots = e.mem.alloc(8);
        e.call(0x00a5_1e80, &args![this, time.to_bits(), slots, slots + 4]);
        let values = [0, 1, 2, 3].map(|i| e.mem.f32(this + 4 + 4 * i));
        (e.mem.u32(slots) as i32, e.mem.u32(slots + 4) as i32, values)
    }

    #[test]
    fn compute_four_points_gives_the_bezier_weights() {
        let mut e = Engine::new();
        constants(&mut e);
        let this = basis(&mut e, 4);
        let (first, last, values) = compute(&mut e, this, 0.25);
        assert_eq!((first, last), (0, 3));
        let u = 0.25f64;
        let expected = [
            (1.0 - u).powi(3),
            3.0 * u * (1.0 - u).powi(2),
            3.0 * u * u * (1.0 - u),
            u.powi(3),
        ];
        for (value, wanted) in values.iter().zip(expected) {
            assert!(
                (f64::from(*value) - wanted).abs() < 1e-6,
                "{value} {wanted}"
            );
        }
        assert_eq!(e.mem.u32(this + 0x14), 0.25f32.to_bits());
        assert_eq!((e.mem.u32(this + 0x18), e.mem.u32(this + 0x1c)), (0, 3));
    }

    #[test]
    fn compute_repeats_the_cached_range_for_the_same_time() {
        let mut e = Engine::new();
        constants(&mut e);
        let this = basis(&mut e, 4);
        e.mem.set_u32(this + 0x14, 0.5f32.to_bits());
        e.mem.set_u32(this + 0x18, 7);
        e.mem.set_u32(this + 0x1c, 9);
        e.mem.set_u32(this + 4, 0x1234);
        let (first, last, _) = compute(&mut e, this, 0.5);
        assert_eq!((first, last), (7, 9));
        assert_eq!(e.mem.u32(this + 4), 0x1234);
    }

    #[test]
    fn compute_weights_sum_to_one_for_every_point_count() {
        for quantity in [5u32, 6, 7, 9, 12] {
            for time in [0.0f32, 0.1, 0.37, 0.5, 0.81, 0.999, 1.0] {
                let mut e = Engine::new();
                constants(&mut e);
                let this = basis(&mut e, quantity);
                let (first, last, values) = compute(&mut e, this, time);
                assert_eq!(first, last - 3);
                assert!(last >= 3 && last < quantity as i32, "{quantity} {time}");
                let sum: f64 = values.iter().map(|v| f64::from(*v)).sum();
                assert!((sum - 1.0).abs() < 1e-5, "{quantity} {time} {values:?}");
                assert!(values.iter().all(|v| *v >= -1e-6));
            }
        }
    }

    #[test]
    fn compute_at_time_one_and_nan_takes_the_last_span() {
        let mut e = Engine::new();
        constants(&mut e);
        let this = basis(&mut e, 8);
        let (first, last, values) = compute(&mut e, this, 1.0);
        assert_eq!((first, last), (4, 7));
        assert_eq!(values[3], 1.0);
        let this = basis(&mut e, 8);
        let (_, last, _) = compute(&mut e, this, f32::NAN);
        assert_eq!(last, 7);
    }

    #[test]
    fn compute_with_three_points_leaves_the_values() {
        let mut e = Engine::new();
        constants(&mut e);
        let this = basis(&mut e, 3);
        e.mem.set_u32(this + 4, 0x4444);
        let (first, last, _) = compute(&mut e, this, 0.5);
        assert_eq!((first, last), (0, 3));
        assert_eq!(e.mem.u32(this + 4), 0x4444);
    }

    #[test]
    fn compacted_value_blends_the_expanded_points() {
        let mut e = Engine::new();
        constants(&mut e);
        let quantity = 4u32;
        let basis_object = basis(&mut e, quantity);
        e.register_double(GET_DEGREE3_BASIS, move |_, _| basis_object.into_ret());
        // The expansion double writes point p, component c as 10 * p + c.
        e.register(EXPAND_COMPACT_VALUES, |e, a| {
            let dimension = a[1] / 4;
            for point in 0..4 {
                for component in 0..dimension {
                    let value = (10 * point + component) as f32;
                    e.mem
                        .set_u32(a[4] + 4 * (point * dimension + component), value.to_bits());
                }
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x0050_0000);
        let output = e.mem.alloc(16);
        e.call_log = Some(vec![]);
        e.call(
            0x00a5_26b0,
            &args![
                this,
                0.5f32.to_bits(),
                output,
                3u32,
                0x9000u32,
                6u32,
                2.0f32.to_bits(),
                4.0f32.to_bits()
            ],
        );
        let expand = calls(&e, EXPAND_COMPACT_VALUES);
        assert_eq!(
            expand[0][..4],
            [0x0050_0000 + 2 * 6, 12, 2.0f32.to_bits(), 4.0f32.to_bits()]
        );
        assert_eq!(expand[0][5], 12);
        assert_eq!(calls(&e, GET_DEGREE3_BASIS), vec![vec![0x9000]]);
        let weights = [0, 1, 2, 3].map(|i| f64::from(e.mem.f32(basis_object + 4 + 4 * i)));
        for component in 0..3u32 {
            let wanted: f64 = (0..4)
                .map(|point| weights[point as usize] * f64::from(10 * point + component))
                .sum();
            let value = f64::from(e.mem.f32(output + 4 * component));
            assert!((value - wanted).abs() < 1e-4, "{value} {wanted}");
        }
    }

    #[test]
    fn load_binary_reads_both_point_arrays() {
        let mut e = Engine::new();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        const READ: u32 = 0x0300_0000;
        e.register(READ, |e, a| {
            if a[2] == 4 && a[4] == 1 && e.mem.u32(a[3]) == 4 {
                // The two counts: 2 floats, 3 compact values.
                let count = if e.mem.u32(0x4000_0000) == 0 { 2 } else { 3 };
                e.mem.set_u32(0x4000_0000, 1);
                e.mem.set_u32(a[1], count);
            }
            a[2].into_ret()
        });
        e.map(0x4000_0000, 4);
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00a5_2870, &args![this, stream]);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        assert_eq!(e.mem.u32(this + 0x14), 3);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8], vec![6]]);
        let reads = calls(&e, READ);
        assert_eq!(reads.len(), 4);
        assert_eq!((reads[1][1], reads[1][2]), (e.mem.u32(this + 8), 8));
        assert_eq!((reads[3][1], reads[3][2]), (e.mem.u32(this + 0xc), 6));
        assert_eq!(e.mem.u32(input + 4), 4 + 8 + 4 + 6);
    }

    #[test]
    fn load_binary_skips_empty_arrays() {
        let mut e = Engine::new();
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        const READ: u32 = 0x0300_0000;
        e.register(READ, |_, a| a[2].into_ret());
        let input = e.mem.alloc(0xc);
        e.mem.set_u32(input + 8, READ);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00a5_2870, &args![this, stream]);
        assert!(calls(&e, ALLOCATE).is_empty());
        assert_eq!(calls(&e, READ).len(), 2);
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_points() {
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        e.register(OBJECT_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 8, 0x100);
        e.mem.set_u32(this + 0xc, 0x200);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a5_2510, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), B_SPLINE_DATA_VTABLE);
        assert_eq!(calls(&e, FREE), vec![vec![0x100], vec![0x200]]);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a5_2510, &args![this, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0x18]]);
        assert_eq!(calls(&e, OBJECT_DESTRUCTOR).len(), 2);
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    type D = NiBSplineData;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn is_equal_compares_both_counts() {
        let mut e = Engine::new();
        e.register(OBJECT_IS_EQUAL, |_, _| word(1));
        e.call_log = Some(vec![]);
        let this = e.new_object::<D>();
        let other = e.new_object::<D>();
        for target in [this, other] {
            e.set(target, D::m_uiControlPointCount, 4);
            e.set(target, D::m_uiCompactControlPointCount, 6);
        }
        assert!(e.call(0x00a5_1e50, &args![this, other]).bool());
        assert_eq!(
            calls(&e, OBJECT_IS_EQUAL),
            vec![vec![this.addr(), other.addr()]]
        );
        e.set(other, D::m_uiCompactControlPointCount, 7);
        assert!(!e.call(0x00a5_1e50, &args![this, other]).bool());
        e.set(other, D::m_uiCompactControlPointCount, 6);
        e.set(other, D::m_uiControlPointCount, 5);
        assert!(!e.call(0x00a5_1e50, &args![this, other]).bool());
        e.set(other, D::m_uiControlPointCount, 4);
        e.register(OBJECT_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_1e50, &args![this, other]).bool());
    }

    #[test]
    fn get_value_degree3_sums_the_weighted_control_points() {
        let mut e = Engine::new();
        e.register(GET_DEGREE3_BASIS, |_, a| word(a[0] + 8));
        e.call_log = Some(vec![]);
        // A basis already computed for time 0.5: indices 2..=5, weights.
        let basis_data = e.mem.alloc(0x28);
        let basis = basis_data + 8;
        e.mem.set_u32(basis, 9);
        for (i, w) in [1.0f32, 2.0, 0.5, 0.25].iter().enumerate() {
            e.mem.set_f32(basis + 4 + 4 * i as u32, *w);
        }
        e.mem.set_u32(basis + 0x14, 0.5f32.to_bits());
        e.mem.set_u32(basis + 0x18, 2);
        e.mem.set_u32(basis + 0x1c, 5);
        let points = e.mem.alloc(4 * 40);
        for i in 0..40u32 {
            e.mem.set_f32(points + 4 * i, i as f32);
        }
        let this = e.new_object::<D>();
        e.set(this, D::m_pafControlPoints, points);
        let output = e.mem.alloc(4 * 5);
        e.call(
            0x00a5_2550,
            &args![this, 0.5f32, output, 5u32, basis_data, 1u32],
        );
        assert_eq!(calls(&e, GET_DEGREE3_BASIS), vec![vec![basis_data]]);
        for k in 0..5u32 {
            let p = |j: u32| (11 + 5 * j + k) as f32;
            let expected = p(0) + 2.0 * p(1) + 0.5 * p(2) + 0.25 * p(3);
            assert_eq!(e.mem.f32(output + 4 * k), expected);
        }
        // A one-point range: only the first weight.
        e.mem.set_u32(basis + 0x1c, 2);
        e.call(
            0x00a5_2550,
            &args![this, 0.5f32, output, 3u32, basis_data, 0u32],
        );
        for k in 0..3u32 {
            assert_eq!(e.mem.f32(output + 4 * k), (6 + k) as f32);
        }
    }

    fn output(e: &mut Engine, write: u32) -> u32 {
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, write);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + 0x250, output);
        stream
    }

    #[test]
    fn save_binary_writes_counts_and_points() {
        const WRITE: u32 = 0x0300_0100;
        let mut e = Engine::new();
        e.register(OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register_double(WRITE, |e, a| {
            let component = e.mem.u32(a[3]);
            word(a[2] + component)
        });
        e.call_log = Some(vec![]);
        let stream = output(&mut e, WRITE);
        let this = e.new_object::<D>();
        e.set(this, D::m_pafControlPoints, 0x5000);
        e.set(this, D::m_pasCompactControlPoints, 0x6000);
        e.set(this, D::m_uiControlPointCount, 3);
        e.set(this, D::m_uiCompactControlPointCount, 5);
        e.call(0x00a5_2950, &args![this, stream]);
        assert_eq!(
            calls(&e, OBJECT_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
        let writes: Vec<_> = calls(&e, WRITE)
            .iter()
            .map(|w| (w[1], w[2], w[4]))
            .collect();
        assert_eq!(
            writes,
            vec![
                (this.addr() + 0x10, 4, 1),
                (0x5000, 12, 1),
                (this.addr() + 0x14, 4, 1),
                (0x6000, 10, 1)
            ]
        );
        let output = e.mem.u32(stream + 0x250);
        assert_eq!(e.mem.u32(output + 4), 8 + 16 + 8 + 12);
        // Empty arrays: only the counts.
        e.set(this, D::m_uiControlPointCount, 0);
        e.set(this, D::m_uiCompactControlPointCount, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_2950, &args![this, stream]);
        assert_eq!(calls(&e, WRITE).len(), 2);
    }

    #[test]
    fn get_viewer_strings_appends_name_and_counts() {
        let mut e = Engine::new();
        e.map(NI_B_SPLINE_DATA_RTTI, 4);
        e.set_global(NI_B_SPLINE_DATA_RTTI, 0x0109_8888u32);
        e.register(OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0x21));
        e.register(NI_GET_VIEWER_STRING_UNSIGNED, |_, _| word(0x22));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.new_object::<D>();
        e.set(this, D::m_uiControlPointCount, 3);
        e.set(this, D::m_uiCompactControlPointCount, 5);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 4);
        e.call(0x00a5_2a20, &args![this, strings]);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_8888]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_UNSIGNED),
            vec![
                vec![NAME_CONTROL_POINT_COUNT, 3],
                vec![NAME_COMPACT_CONTROL_POINT_COUNT, 5]
            ]
        );
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 6]]);
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 3);
    }
}
