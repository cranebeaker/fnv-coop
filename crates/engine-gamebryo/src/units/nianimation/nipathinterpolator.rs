//! `NiAnimation/nipathinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPosData::ms_RTTI` (the class's `NiRTTI` record).
const NI_POS_DATA_RTTI: u32 = 0x011f_3f68;

/// `NiPathInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_PATH_INTERPOLATOR_RTTI: u32 = 0x011f_3e48;

// Translated from 00a3dab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_pos_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_POS_DATA_RTTI)
}

// Translated from 00a3dde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_path_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PATH_INTERPOLATOR_RTTI)
}

// Translated from 00a3dac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a48fb0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x18 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_pos_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_8fb0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00a3e980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a3dd40`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x5c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_path_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_dd40;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x5cu32]);
    }
    this
}

/// `NiPathInterpolator`'s vtable (written by its constructor and destructor).
const NI_PATH_INTERPOLATOR_VTABLE: u32 = 0x0109_720c;
/// `NiKeyBasedInterpolator`'s constructor (`thiscall`, no arguments) and destructor body
/// (`thiscall`).
const KEY_BASED_INTERPOLATOR_CONSTRUCTOR: u32 = 0x00a3_ca10;
const KEY_BASED_INTERPOLATOR_DESTRUCTOR: u32 = 0x00a3_ca30;
/// `NiKeyBasedInterpolator::LoadBinary` (`thiscall`, `(stream)`), its `LinkObject`
/// (`00a95ca0`, `(stream)`; the map's name for that address is a folded one) and its
/// `CopyMembers` (`00aa0ab0`, `(destination, cloning)`).
const KEY_BASED_INTERPOLATOR_LOAD_BINARY: u32 = 0x00aa_0ac0;
const KEY_BASED_INTERPOLATOR_LINK_OBJECT: u32 = 0x00a9_5ca0;
const KEY_BASED_INTERPOLATOR_COPY_MEMBERS: u32 = 0x00aa_0ab0;
/// `NiObject::ProcessClone` (`thiscall`, `(cloning)`).
const NI_OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
/// `NiStream::GetObjectFromLinkID` (`thiscall` on the stream, no arguments) and the link-ID
/// read `00a645b0` (`thiscall` on the stream, no arguments).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiPathInterpolator::CalcRefFrame` (`thiscall`, `(first, second, ratio, &matrix)`) and
/// `NiQuaternion::FromRotation` (`thiscall` on the quaternion, `(&matrix)`).
const CALC_REF_FRAME: u32 = 0x00a3_de50;
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
/// `NiFloatKey::GenInterp` (`cdecl`, `(time, keys, key type, key count, &last index, key
/// size)`, the value in ST0).
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `NiPosKey`'s per-key-type function tables (indexed by `NiPosData`'s key type): the
/// interpolation (`011f3ca8`, `cdecl (ratio, key, next key, &point)`), the derivative
/// (`011f3c18`, same arguments) and the one `LinkObject` stores as the maximum curvature
/// (`011f3d38`, `cdecl (keys, key count)`, the value in ST0).
const POS_KEY_INTERPOLATIONS: u32 = 0x011f_3ca8;
const POS_KEY_DERIVATIVES: u32 = 0x011f_3c18;
const POS_KEY_CURVATURES: u32 = 0x011f_3d38;
/// The allocator's table allocation and free (`cdecl`, one word), `operator new` (`cdecl`,
/// `(size)`), `memcpy` (`cdecl`), the CRT square root `_CIsqrt` (argument and result in ST0)
/// and `_CIfmod` (the dividend in ST1, the divisor in ST0, both passed here as doubles).
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
const TABLE_FREE: u32 = 0x00aa_10f0;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const MEMCPY: u32 = 0x00ec_44d0;
const CRT_SQRT: u32 = 0x00ec_6040;
const CI_FMOD: u32 = 0x00ec_913a;
/// Import slots of `InterlockedIncrement` and `InterlockedDecrement`.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiQuaternion::IDENTITY` (four floats), `NiInterpolator::INVALID_POINT3` (three floats),
/// `-1.0f`, `-FLT_MAX` (`NiInterpolator::INVALID_FLOAT`'s bits) and the `double` tolerance
/// `0.0001` of the arc-length search.
const QUATERNION_IDENTITY: u32 = 0x011f_4208;
const INVALID_POINT3: u32 = 0x011a_9424;
const MINUS_ONE: u32 = 0x0101_2054;
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
const ARC_LENGTH_TOLERANCE: u32 = 0x0103_2980;
/// The five Gauss-Legendre abscissas (on [0, 1]) and weights of the arc-length integral.
const GAUSS_ABSCISSAS: u32 = 0x011a_854c;
const GAUSS_WEIGHTS: u32 = 0x011a_8538;

layout! {
    /// `NiPathInterpolator` (Xbox PDB), 0x5C bytes (offsets checked against the PC code).
    pub struct NiPathInterpolator: 0x5C {
        /// `NiInterpolator::m_fLastTime` (Xbox PDB), float bits.
        0x08 m_fInterpolatorLastTime: u32,
        /// `m_uFlags` (Xbox PDB): bit 0 "lengths stale", bit 4 constant speed, bit 5 follow.
        0x0C m_uFlags: u16,
        /// `m_uiLastPathIdx` (Xbox PDB).
        0x10 m_uiLastPathIdx: u32,
        /// `m_uiLastPctIdx` (Xbox PDB).
        0x14 m_uiLastPctIdx: u32,
        /// `m_spPathData` (Xbox PDB): `NiPosData*`.
        0x18 m_spPathData: u32,
        /// `m_spPctData` (Xbox PDB): `NiFloatData*`.
        0x1C m_spPctData: u32,
        /// `m_pfPartialLength` (Xbox PDB): one float per key.
        0x20 m_pfPartialLength: u32,
        /// `m_fTotalLength` (Xbox PDB), float bits.
        0x24 m_fTotalLength: u32,
        /// `m_fMaxBankAngle` (Xbox PDB), float bits.
        0x28 m_fMaxBankAngle: u32,
        /// `m_fSmoothing` (Xbox PDB), float bits.
        0x2C m_fSmoothing: u32,
        /// `m_sFollowAxis` (Xbox PDB).
        0x30 m_sFollowAxis: u16,
        /// `m_fMaxCurvature` (Xbox PDB), float bits.
        0x34 m_fMaxCurvature: u32,
        /// `m_eBankDir` (Xbox PDB).
        0x38 m_eBankDir: u32,
        /// `m_kRotation` (Xbox PDB): `NiQuaternion`, four floats.
        0x3C m_kRotation: u32,
        /// `m_kPosition` (Xbox PDB): `NiPoint3`, three floats.
        0x4C m_kPosition: u32,
        /// `m_fLastTime` (Xbox PDB), float bits.
        0x58 m_fLastTime: u32,
    }
}

/// `NiPosData` (Xbox PDB) fields read here: `m_uiNumKeys` +0x08, `m_pkKeys` +0x0C,
/// `m_eType` +0x10, `m_ucKeySize` +0x14. `NiFloatData` has the same four at the same offsets.
const KEY_DATA_NUM_KEYS: u32 = 0x08;
const KEY_DATA_KEYS: u32 = 0x0C;
const KEY_DATA_TYPE: u32 = 0x10;
const KEY_DATA_KEY_SIZE: u32 = 0x14;

/// Releases a counted object (count at +4): when the decrement reaches 0, its virtual slot
/// +4 (`DeleteThis`) runs.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The smart-pointer assignment of a counted object to the pointer at `slot`: nothing when
/// it is already there; otherwise the old one is released, `new` stored and counted.
fn assign_counted(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
    }
}

/// Reads `size` bytes from the stream's input (`NiStream::m_pkIstr` +0x24C) into
/// `destination`: the input's read function (`NiBinaryStream::m_pfnRead` +0x08) is called
/// as `function(input, destination, size, &count, 1)` with `count` set to `size`, and the
/// byte count it answers is added to the input's position (+0x04).
fn stream_read(e: &mut Engine, stream: Ptr, destination: u32, size: u32) {
    let input = e.mem.u32(stream.addr().wrapping_add(0x24c));
    let function = e.mem.u32(input.wrapping_add(8));
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), size);
        e.call(function, &args![input, destination, size, count, 1u32])
            .u32()
    });
    let position = e.mem.u32(input.wrapping_add(4));
    e.mem
        .set_u32(input.wrapping_add(4), position.wrapping_add(done));
}

/// The path data's key type, key size and keys (all 0 without path data).
fn path_keys(e: &Engine, this: Ptr<NiPathInterpolator>) -> (u32, u32, u32) {
    let data = e.get(this, NiPathInterpolator::m_spPathData);
    if data == 0 {
        (0, 0, 0)
    } else {
        (
            e.mem.u32(data.wrapping_add(KEY_DATA_TYPE)),
            u32::from(e.mem.u8(data.wrapping_add(KEY_DATA_KEY_SIZE))),
            e.mem.u32(data.wrapping_add(KEY_DATA_KEYS)),
        )
    }
}

/// The path data's key count (0 without path data).
fn path_key_count(e: &Engine, this: Ptr<NiPathInterpolator>) -> u32 {
    let data = e.get(this, NiPathInterpolator::m_spPathData);
    if data == 0 {
        0
    } else {
        e.mem.u32(data.wrapping_add(KEY_DATA_NUM_KEYS))
    }
}

/// The low byte of the flags word (the exe tests its bits through a byte read).
fn flags_byte(e: &Engine, this: Ptr<NiPathInterpolator>) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0xc))
}

// Translated from 00a3dd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator`'s destructor body (unnamed in the map): the class's vtable, then
/// the path and percentage data are released and cleared, the partial lengths freed
/// (`00aa10f0`), the two pointers' own destructors run (reading the now cleared words, so
/// they release nothing more), and `NiKeyBasedInterpolator`'s destructor body (`00a3ca30`).
pub fn fn_00a3dd40(e: &mut Engine, this: Ptr<NiPathInterpolator>) {
    e.mem.set_u32(this.addr(), NI_PATH_INTERPOLATOR_VTABLE);
    for field in [
        NiPathInterpolator::m_spPathData,
        NiPathInterpolator::m_spPctData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            release(e, object);
            e.set(this, field, 0);
        }
    }
    let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
    e.call(TABLE_FREE, &args![lengths]);
    for field in [
        NiPathInterpolator::m_spPctData,
        NiPathInterpolator::m_spPathData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            release(e, object);
        }
    }
    e.call(KEY_BASED_INTERPOLATOR_DESTRUCTOR, &args![this]);
}

// Translated from 00a3ddf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's point between the keys `first` and `second` (unnamed in the map): the path
/// data's key-type interpolation (`011f3ca8`) with `ratio`, into `point`. Without path data
/// the key type, size and keys are 0. Answers `point`.
pub fn fn_00a3ddf0(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    point: Ptr,
    first: u32,
    second: u32,
    ratio: f32,
) -> Ptr {
    let (key_type, size, keys) = path_keys(e, this);
    let key = size.wrapping_mul(first).wrapping_add(keys);
    let next_key = size.wrapping_mul(second).wrapping_add(keys);
    let function = e
        .mem
        .u32(POS_KEY_INTERPOLATIONS.wrapping_add(key_type.wrapping_mul(4)));
    e.call(
        function,
        &args![x87_quiet_bits(ratio.to_bits()), key, next_key, point],
    );
    point
}

// Translated from 00a3dc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::ProcessClone` (Xbox PDB): `NiObject::ProcessClone`, then the path
/// and percentage data's own `ProcessClone` (vtable +0x68) when present.
pub fn ni_path_interpolator_process_clone(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    cloning: Ptr,
) {
    e.call(NI_OBJECT_PROCESS_CLONE, &args![this, cloning]);
    for field in [
        NiPathInterpolator::m_spPathData,
        NiPathInterpolator::m_spPctData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            e.vcall(object, 0x68, &args![cloning]);
        }
    }
}

// Translated from 00a3e5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's speed between the keys `first` and `second` at `ratio` (unnamed in the map):
/// the key type's derivative (`011f3c18`) into a local vector, whose length (`x*x + y*y +
/// z*z` rounded to a float, through `_CIsqrt`) is answered as a float.
pub fn fn_00a3e5f0(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    first: u32,
    second: u32,
    ratio: f32,
) -> f32 {
    let (key_type, size, keys) = path_keys(e, this);
    let key = size.wrapping_mul(first).wrapping_add(keys);
    let next_key = size.wrapping_mul(second).wrapping_add(keys);
    let function = e
        .mem
        .u32(POS_KEY_DERIVATIVES.wrapping_add(key_type.wrapping_mul(4)));
    let [x, y, z] = e.with_stack(12, |e, vector| {
        e.call(
            function,
            &args![x87_quiet_bits(ratio.to_bits()), key, next_key, vector],
        );
        [0u32, 4, 8].map(|off| x87_load32(e.mem.u32(vector.addr() + off)))
    });
    let xx = x87_mul(x, x);
    let yy = x87_mul(y, y);
    let sum = x87_add(yy, xx);
    let zz = x87_mul(z, z);
    let squared = x87_store32(x87_add(sum, zz));
    let root = e.call(CRT_SQRT, &args![x87_load32(squared)]).f64();
    f32::from_bits(x87_quiet_bits(x87_store32(root)))
}

// Translated from 00a3e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's length from the key `first` towards `second` up to `ratio` (unnamed in the
/// map): five-point Gauss-Legendre quadrature of the speed (`fn_00a3e5f0`) at the
/// abscissas (`011a854c`) scaled by `ratio`, weighted (`011a8538`) and summed as a float,
/// times `ratio`.
pub fn fn_00a3e680(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    first: u32,
    second: u32,
    ratio: f32,
) -> f32 {
    let ratio_bits = ratio.to_bits();
    let mut sum = x87_store32(0.0);
    for offset in (0..0x14u32).step_by(4) {
        let abscissa = e.global::<u32>(GAUSS_ABSCISSAS + offset);
        let at = x87_store32(x87_mul(x87_load32(abscissa), x87_m32(ratio_bits)));
        let speed = fn_00a3e5f0(e, this, first, second, f32::from_bits(x87_quiet_bits(at)));
        let weight = e.global::<u32>(GAUSS_WEIGHTS + offset);
        let weighted = x87_mul(x87_load32(speed.to_bits()), x87_m32(weight));
        sum = x87_store32(x87_add(weighted, x87_m32(sum)));
    }
    let length = x87_store32(x87_mul(x87_load32(sum), x87_m32(ratio_bits)));
    f32::from_bits(x87_quiet_bits(length))
}

// Translated from 00a3e9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's total length (unnamed in the map): while `m_fTotalLength` is below 0 the
/// partial lengths are rebuilt (the old table freed, one float per key allocated, the first
/// 0, each next one the previous plus the length of that segment from `fn_00a3e680` with
/// ratio 1) and the last becomes the total. Answers the total. With no keys the loop runs
/// to the count's wrap-around and the total is read from the word before the table, as the
/// exe does.
pub fn fn_00a3e9a0(e: &mut Engine, this: Ptr<NiPathInterpolator>) -> f32 {
    let total = e.get(this, NiPathInterpolator::m_fTotalLength);
    if x87_load32(total) < 0.0 {
        let count = path_key_count(e, this);
        let old = e.get(this, NiPathInterpolator::m_pfPartialLength);
        e.call(TABLE_FREE, &args![old]);
        let bytes = count.wrapping_mul(4);
        let lengths = e.call(TABLE_ALLOCATE, &args![bytes]).u32();
        e.set(this, NiPathInterpolator::m_pfPartialLength, lengths);
        e.mem.set_u32(lengths, x87_store32(0.0));
        let last = count.wrapping_sub(1);
        if last != 0 {
            let mut index = 0u32;
            loop {
                let next = index.wrapping_add(1);
                let segment = fn_00a3e680(e, this, index, next, 1.0);
                let segment = x87_store32(x87_load32(segment.to_bits()));
                let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
                let at = lengths.wrapping_add(index.wrapping_mul(4));
                let sum = x87_add(x87_load32(e.mem.u32(at)), x87_m32(segment));
                e.mem.set_u32(at.wrapping_add(4), x87_store32(sum));
                index = next;
                if index >= last {
                    break;
                }
            }
        }
        let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
        let last_length = e.mem.u32(lengths.wrapping_add(bytes).wrapping_sub(4));
        e.set(
            this,
            NiPathInterpolator::m_fTotalLength,
            x87_quiet_bits(last_length),
        );
    }
    let total = e.get(this, NiPathInterpolator::m_fTotalLength);
    f32::from_bits(x87_quiet_bits(total))
}

// Translated from 00a3ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constant-speed segment search (unnamed in the map): the keys `first`/`second` and the
/// ratio between them at which the path has covered the fraction `time` of its total
/// length. Time 0 answers (0, 1, 0); a negative time is folded with `_CIfmod(time, 1) + 1`;
/// time 1 answers (count - 2, count - 1, 1); a time above 1 is folded with
/// `_CIfmod(time, 1)`. The partial lengths give the segment holding `time * total` and a
/// linear first guess (0 and the segment past the end when none holds it), refined by up
/// to 32 Newton steps (segment length `fn_00a3e680` against the distance into the segment,
/// over the speed `fn_00a3e5f0`) until the error is within 0.0001 (`01032980`).
pub fn fn_00a3ea60(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    time: f32,
    first_out: Ptr,
    second_out: Ptr,
    ratio_out: Ptr,
) {
    let count = path_key_count(e, this);
    let mut current = x87_load32(time.to_bits());
    if current == 0.0 {
        e.mem.set_u32(first_out.addr(), 0);
        e.mem.set_u32(second_out.addr(), 1);
        e.mem.set_u32(ratio_out.addr(), x87_store32(0.0));
        return;
    }
    if current < 0.0 {
        let folded = e.call(CI_FMOD, &args![current, 1.0f64]).f64();
        let folded = x87_load32(x87_store32(folded));
        current = x87_load32(x87_store32(x87_add(folded, 1.0)));
    }
    if current == 1.0 {
        e.mem.set_u32(first_out.addr(), count.wrapping_sub(2));
        e.mem.set_u32(second_out.addr(), count.wrapping_sub(1));
        e.mem.set_u32(ratio_out.addr(), x87_store32(1.0));
        return;
    }
    if 1.0 < current {
        let folded = e.call(CI_FMOD, &args![current, 1.0f64]).f64();
        current = x87_load32(x87_store32(folded));
    }
    let total = e.get(this, NiPathInterpolator::m_fTotalLength);
    let distance = x87_store32(x87_mul(current, x87_m32(total)));
    let mut into_segment = x87_store32(0.0);
    let mut ratio = into_segment;
    let mut first = 0u32;
    let mut second = 1u32;
    let last = count.wrapping_sub(1);
    if last != 0 {
        let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
        loop {
            let next = e
                .mem
                .u32(lengths.wrapping_add(first.wrapping_mul(4)).wrapping_add(4));
            if x87_load32(next) >= x87_load32(distance) {
                let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
                let start = e.mem.u32(lengths.wrapping_add(first.wrapping_mul(4)));
                let end = e.mem.u32(lengths.wrapping_add(second.wrapping_mul(4)));
                into_segment = x87_store32(x87_sub(x87_load32(distance), x87_m32(start)));
                let span = x87_sub(x87_load32(end), x87_m32(start));
                ratio = x87_store32(x87_div(x87_load32(into_segment), span));
                break;
            }
            first = first.wrapping_add(1);
            second = second.wrapping_add(1);
            if first >= last {
                break;
            }
        }
    }
    let tolerance = e.global::<f64>(ARC_LENGTH_TOLERANCE);
    let mut steps = 0u32;
    loop {
        let guess = f32::from_bits(x87_quiet_bits(ratio));
        let length = fn_00a3e680(e, this, first, second, guess);
        let error = x87_store32(x87_sub(x87_load32(length.to_bits()), x87_m32(into_segment)));
        let error = x87_load32(error);
        let magnitude = x87_store32(error.abs());
        if x87_load32(magnitude) <= tolerance {
            break;
        }
        let speed = fn_00a3e5f0(e, this, first, second, guess);
        let step = x87_div(error, x87_load32(speed.to_bits()));
        ratio = x87_store32(x87_sub(x87_load32(ratio), step));
        steps += 1;
        if steps >= 0x20 {
            break;
        }
    }
    e.mem.set_u32(first_out.addr(), first);
    e.mem.set_u32(second_out.addr(), second);
    e.mem.set_u32(ratio_out.addr(), x87_quiet_bits(ratio));
}

// Translated from 00a3ec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path segment for `time` (unnamed in the map): with constant speed (flag bit 4) the
/// arc-length search `fn_00a3ea60`, its first key kept as `m_uiLastPathIdx`. Otherwise, by
/// the key times (each key's first float): at or before the first key (0, 1, 0); at or
/// after the last (count - 2, count - 1, 1); else from `m_uiLastPathIdx` on (wrapping to 0
/// once) the first key equal to `time` or followed by a later key, kept as
/// `m_uiLastPathIdx`, and the ratio `(time - t[first]) / (t[second] - t[first])` (with
/// `second` 0 when no key matched). Without path data the keys are read at address 0, as
/// the exe does.
pub fn fn_00a3ec30(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    time: f32,
    first_out: Ptr,
    second_out: Ptr,
    ratio_out: Ptr,
) {
    if (flags_byte(e, this) >> 4) & 1 != 0 {
        fn_00a3ea60(e, this, time, first_out, second_out, ratio_out);
        let first = e.mem.u32(first_out.addr());
        e.set(this, NiPathInterpolator::m_uiLastPathIdx, first);
        return;
    }
    let count = path_key_count(e, this);
    let (_, size, keys) = path_keys(e, this);
    let time = x87_load32(time.to_bits());
    let key_time = |e: &Engine, index: u32| e.mem.u32(size.wrapping_mul(index).wrapping_add(keys));
    if x87_load32(e.mem.u32(keys)) >= time {
        e.mem.set_u32(first_out.addr(), 0);
        e.mem.set_u32(second_out.addr(), 1);
        e.mem.set_u32(ratio_out.addr(), x87_store32(0.0));
        return;
    }
    if x87_load32(key_time(e, count.wrapping_sub(1))) <= time {
        e.mem.set_u32(first_out.addr(), count.wrapping_sub(2));
        e.mem.set_u32(second_out.addr(), count.wrapping_sub(1));
        e.mem.set_u32(ratio_out.addr(), x87_store32(1.0));
        return;
    }
    let mut first = e.get(this, NiPathInterpolator::m_uiLastPathIdx);
    let mut second = 0u32;
    let mut first_pass = true;
    let last = count.wrapping_sub(1);
    if first < last {
        loop {
            let at = x87_load32(key_time(e, first));
            let found = if at == time {
                true
            } else if at < time {
                x87_load32(key_time(e, first.wrapping_add(1))) > time
            } else {
                false
            };
            if found {
                second = first.wrapping_add(1);
                e.set(this, NiPathInterpolator::m_uiLastPathIdx, first);
                break;
            }
            if first == count.wrapping_sub(2) && first_pass {
                first = 0;
                first_pass = false;
            } else {
                first = first.wrapping_add(1);
            }
            if first >= last {
                break;
            }
        }
    }
    let start = key_time(e, first);
    let end = key_time(e, second);
    let elapsed = x87_sub(time, x87_m32(start));
    let span = x87_sub(x87_load32(end), x87_m32(start));
    e.mem
        .set_u32(ratio_out.addr(), x87_store32(x87_div(elapsed, span)));
    e.mem.set_u32(first_out.addr(), first);
    e.mem.set_u32(second_out.addr(), second);
}

// Translated from 00a3ed90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::LoadBinary` (Xbox PDB): `NiKeyBasedInterpolator::LoadBinary`, then
/// from the stream the flags (2 bytes), the bank direction (4 bytes, through a local), the
/// maximum bank angle and smoothing (4 bytes each) and the follow axis (2 bytes), and the
/// stream's link IDs of the two data objects (`00a645b0`, twice).
pub fn ni_path_interpolator_load_binary(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    stream: Ptr,
) {
    e.call(KEY_BASED_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    stream_read(e, stream, this.addr() + 0xc, 2);
    let bank = e.with_stack(4, |e, local| {
        stream_read(e, stream, local.addr(), 4);
        e.mem.u32(local.addr())
    });
    e.set(this, NiPathInterpolator::m_eBankDir, bank);
    stream_read(e, stream, this.addr() + 0x28, 4);
    stream_read(e, stream, this.addr() + 0x2c, 4);
    stream_read(e, stream, this.addr() + 0x30, 2);
    e.call(STREAM_READ_LINK_ID, &args![stream]);
    e.call(STREAM_READ_LINK_ID, &args![stream]);
}

// Translated from 00a3ee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::LinkObject` (Xbox PDB): the base `LinkObject` (`00a95ca0`), the
/// path and percentage data from the stream's next linked objects (counted pointer
/// assignment), the "lengths stale" flag (bit 0) set; with constant speed (bit 4) the total
/// length is recomputed (`fn_00a3e9a0`) and bit 0 cleared. With path data holding keys, the
/// key type's function at `011f3d38` (`(keys, count)`) gives `m_fMaxCurvature`.
pub fn ni_path_interpolator_link_object(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    stream: Ptr,
) {
    e.call(KEY_BASED_INTERPOLATOR_LINK_OBJECT, &args![this, stream]);
    let path = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_counted(e, this.addr() + 0x18, path);
    let percentage = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_counted(e, this.addr() + 0x1c, percentage);
    let flags = e.get(this, NiPathInterpolator::m_uFlags) | 1;
    e.set(this, NiPathInterpolator::m_uFlags, flags);
    if (flags >> 4) & 1 != 0 && flags_byte(e, this) & 1 != 0 {
        let total = fn_00a3e9a0(e, this);
        e.set(this, NiPathInterpolator::m_fTotalLength, total.to_bits());
        let flags = e.get(this, NiPathInterpolator::m_uFlags) & 0xfffe;
        e.set(this, NiPathInterpolator::m_uFlags, flags);
    }
    let data = e.get(this, NiPathInterpolator::m_spPathData);
    if data != 0 {
        let count = e.mem.u32(data.wrapping_add(KEY_DATA_NUM_KEYS));
        let key_type = e.mem.u32(data.wrapping_add(KEY_DATA_TYPE));
        let keys = e.mem.u32(data.wrapping_add(KEY_DATA_KEYS));
        if count != 0 {
            let function = e
                .mem
                .u32(POS_KEY_CURVATURES.wrapping_add(key_type.wrapping_mul(4)));
            let curvature = e.call(function, &args![keys, count]).f64();
            e.set(
                this,
                NiPathInterpolator::m_fMaxCurvature,
                x87_store32(curvature),
            );
        }
    }
}

// Translated from 00a3f060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::NiPathInterpolator` (Xbox PDB), `(path data, percentage data)`:
/// `NiKeyBasedInterpolator`'s constructor, the class's vtable, both data pointers stored
/// and counted, the rotation the identity quaternion (`011f4208`), the position the invalid
/// point (`011a9424`), bank angle, smoothing and curvature 0, total length -1
/// (`01012054`), last time `-FLT_MAX` (`01015f5c`), indices 0, bank direction 1, follow
/// axis 0, no partial lengths, flags 3. Answers `this`.
pub fn ni_path_interpolator_constructor(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    path: Ptr,
    percentage: Ptr,
) -> Ptr<NiPathInterpolator> {
    e.call(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), NI_PATH_INTERPOLATOR_VTABLE);
    for (field, object) in [
        (NiPathInterpolator::m_spPathData, path.addr()),
        (NiPathInterpolator::m_spPctData, percentage.addr()),
    ] {
        e.set(this, field, object);
        if object != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
        }
    }
    for word in 0..4u32 {
        let value = e.global::<u32>(QUATERNION_IDENTITY + 4 * word);
        e.mem.set_u32(this.addr() + 0x3c + 4 * word, value);
    }
    for word in 0..3u32 {
        let value = e.global::<u32>(INVALID_POINT3 + 4 * word);
        e.mem.set_u32(this.addr() + 0x4c + 4 * word, value);
    }
    let zero = x87_store32(0.0);
    e.set(this, NiPathInterpolator::m_fMaxBankAngle, zero);
    e.set(this, NiPathInterpolator::m_fSmoothing, zero);
    e.set(this, NiPathInterpolator::m_fMaxCurvature, zero);
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    e.set(this, NiPathInterpolator::m_fTotalLength, minus_one);
    let lowest = x87_quiet_bits(e.global::<u32>(LOWEST_FLOAT));
    e.set(this, NiPathInterpolator::m_uiLastPathIdx, 0);
    e.set(this, NiPathInterpolator::m_fLastTime, lowest);
    e.set(this, NiPathInterpolator::m_uiLastPctIdx, 0);
    e.set(this, NiPathInterpolator::m_eBankDir, 1);
    e.set(this, NiPathInterpolator::m_sFollowAxis, 0);
    e.set(this, NiPathInterpolator::m_pfPartialLength, 0);
    e.set(this, NiPathInterpolator::m_uFlags, 3);
    this
}

/// `Update`'s copy of the kept position (three words) into the transform, and with flag
/// bit 5 the kept rotation (four words) after it.
fn copy_transform(e: &mut Engine, this: Ptr<NiPathInterpolator>, transform: Ptr) {
    for word in 0..3u32 {
        let value = e.mem.u32(this.addr() + 0x4c + 4 * word);
        e.mem.set_u32(transform.addr() + 4 * word, value);
    }
    if (flags_byte(e, this) >> 5) & 1 != 0 {
        for word in 0..4u32 {
            let value = e.mem.u32(this.addr() + 0x3c + 4 * word);
            e.mem.set_u32(transform.addr() + 0xc + 4 * word, value);
        }
    }
}

// Translated from 00a3f120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::Update` (Xbox PDB), the transform update `(time, interpolator
/// target, &transform)`: at the base's last time (+0x08) the kept position (and with flag
/// bit 5 the kept rotation) is copied out, unless the position is the invalid point, which
/// answers false with the transform's translation, rotation and scale marked invalid
/// (`-FLT_MAX` at +0x00, +0x10, +0x1C). Otherwise, with percentage data holding keys and
/// path data holding keys: the percentage at `time` (`NiFloatKey::GenInterp`), its path
/// segment (`fn_00a3ec30`), with bit 5 the frame's rotation (`CalcRefFrame`,
/// `NiQuaternion::FromRotation` into `m_kRotation`), the point (`fn_00a3ddf0`) kept and
/// copied out with the rotation as before, `m_fLastTime` set; answers true. Without the
/// data it answers false.
pub fn ni_path_interpolator_update(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    time: f32,
    _unused_2: u32,
    transform: Ptr,
) -> bool {
    let at = x87_load32(time.to_bits());
    let last = x87_load32(e.get(this, NiPathInterpolator::m_fInterpolatorLastTime));
    if at == last {
        let invalid = (0..3u32).all(|word| {
            x87_load32(e.mem.u32(INVALID_POINT3 + 4 * word))
                == x87_load32(e.mem.u32(this.addr() + 0x4c + 4 * word))
        });
        if invalid {
            let lowest = x87_quiet_bits(e.global::<u32>(LOWEST_FLOAT));
            for offset in [0u32, 0x10, 0x1c] {
                e.mem.set_u32(transform.addr() + offset, lowest);
            }
            return false;
        }
        copy_transform(e, this, transform);
        return true;
    }
    let percentage = e.get(this, NiPathInterpolator::m_spPctData);
    if percentage == 0 {
        return false;
    }
    let size = u32::from(e.mem.u8(percentage.wrapping_add(KEY_DATA_KEY_SIZE)));
    let count = e.mem.u32(percentage.wrapping_add(KEY_DATA_NUM_KEYS));
    let key_type = e.mem.u32(percentage.wrapping_add(KEY_DATA_TYPE));
    let keys = e.mem.u32(percentage.wrapping_add(KEY_DATA_KEYS));
    if keys == 0 {
        return false;
    }
    let path = e.get(this, NiPathInterpolator::m_spPathData);
    if path == 0 || e.mem.u32(path.wrapping_add(KEY_DATA_KEYS)) == 0 {
        return false;
    }
    e.with_stack(0x40, |e, locals| {
        let ratio_slot = locals;
        let second_slot = locals.byte_add(4);
        let first_slot = locals.byte_add(8);
        let point = locals.byte_add(0xc);
        let matrix = locals.byte_add(0x18);
        let percent = e
            .call(
                FLOAT_KEY_GEN_INTERP,
                &args![
                    x87_quiet_bits(time.to_bits()),
                    keys,
                    key_type,
                    count,
                    this.addr() + 0x14,
                    size
                ],
            )
            .f64();
        let percent = x87_quiet_bits(x87_store32(percent));
        fn_00a3ec30(
            e,
            this,
            f32::from_bits(percent),
            first_slot,
            second_slot,
            ratio_slot,
        );
        let second = e.mem.u32(second_slot.addr());
        let first = e.mem.u32(first_slot.addr());
        if (flags_byte(e, this) >> 5) & 1 != 0 {
            let ratio = x87_quiet_bits(e.mem.u32(ratio_slot.addr()));
            e.call(CALC_REF_FRAME, &args![this, first, second, ratio, matrix]);
            e.call(QUATERNION_FROM_ROTATION, &args![this.addr() + 0x3c, matrix]);
        }
        let ratio = x87_quiet_bits(e.mem.u32(ratio_slot.addr()));
        let result = fn_00a3ddf0(e, this, point, first, second, f32::from_bits(ratio));
        for word in 0..3u32 {
            let value = e.mem.u32(result.addr() + 4 * word);
            e.mem.set_u32(this.addr() + 0x4c + 4 * word, value);
        }
    });
    copy_transform(e, this, transform);
    e.set(
        this,
        NiPathInterpolator::m_fLastTime,
        x87_quiet_bits(time.to_bits()),
    );
    true
}

// Translated from 00a3f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::CopyMembers` (unnamed in the map), `(destination, cloning)`:
/// `NiKeyBasedInterpolator::CopyMembers`, the flags word, clones (vtable +0x48) of the path
/// and percentage data assigned as counted pointers, flag bits 2, 3, 4, 5, 6 and 1 copied
/// one by one (with bit 4 set and the destination's bit 0 set the total length is
/// recomputed and bit 0 cleared), the bank direction, bank angle, smoothing and follow axis
/// copied, the partial lengths duplicated (`00aa1070` and `memcpy`, one float per path key)
/// when present, and the total length copied.
pub fn fn_00a3f2f0(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    destination: Ptr<NiPathInterpolator>,
    cloning: Ptr,
) {
    e.call(
        KEY_BASED_INTERPOLATOR_COPY_MEMBERS,
        &args![this, destination, cloning],
    );
    let flags = e.get(this, NiPathInterpolator::m_uFlags);
    e.set(destination, NiPathInterpolator::m_uFlags, flags);
    for (offset, field) in [
        (0x18u32, NiPathInterpolator::m_spPathData),
        (0x1c, NiPathInterpolator::m_spPctData),
    ] {
        let object = e.get(this, field);
        if object != 0 {
            let clone = e.vcall(object, 0x48, &args![cloning]).u32();
            assign_counted(e, destination.addr() + offset, clone);
        }
    }
    let copy_bit = |e: &mut Engine, bit: u32| {
        let source = flags_byte(e, this);
        let flags = e.get(destination, NiPathInterpolator::m_uFlags);
        let flags = if (source >> bit) & 1 != 0 {
            flags | (1u16 << bit)
        } else {
            flags & !(1u16 << bit)
        };
        e.set(destination, NiPathInterpolator::m_uFlags, flags);
    };
    copy_bit(e, 2);
    copy_bit(e, 3);
    let bank = e.get(this, NiPathInterpolator::m_eBankDir);
    e.set(destination, NiPathInterpolator::m_eBankDir, bank);
    if (flags_byte(e, this) >> 4) & 1 != 0 {
        let flags = e.get(destination, NiPathInterpolator::m_uFlags) | 0x10;
        e.set(destination, NiPathInterpolator::m_uFlags, flags);
        if flags_byte(e, destination) & 1 != 0 {
            let total = fn_00a3e9a0(e, destination);
            e.set(
                destination,
                NiPathInterpolator::m_fTotalLength,
                total.to_bits(),
            );
            let flags = e.get(destination, NiPathInterpolator::m_uFlags) & 0xfffe;
            e.set(destination, NiPathInterpolator::m_uFlags, flags);
        }
    } else {
        let flags = e.get(destination, NiPathInterpolator::m_uFlags) & 0xffef;
        e.set(destination, NiPathInterpolator::m_uFlags, flags);
    }
    copy_bit(e, 5);
    for field in [
        NiPathInterpolator::m_fMaxBankAngle,
        NiPathInterpolator::m_fSmoothing,
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(destination, field, value);
    }
    let axis = e.get(this, NiPathInterpolator::m_sFollowAxis);
    e.set(destination, NiPathInterpolator::m_sFollowAxis, axis);
    copy_bit(e, 6);
    copy_bit(e, 1);
    if e.get(this, NiPathInterpolator::m_pfPartialLength) != 0 {
        let bytes = path_key_count(e, this).wrapping_mul(4);
        let copy = e.call(TABLE_ALLOCATE, &args![bytes]).u32();
        e.set(destination, NiPathInterpolator::m_pfPartialLength, copy);
        let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
        e.call(MEMCPY, &args![copy, lengths, bytes]);
    }
    let total = x87_quiet_bits(e.get(this, NiPathInterpolator::m_fTotalLength));
    e.set(destination, NiPathInterpolator::m_fTotalLength, total);
}

// Translated from 00a3f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::CreateClone` (Xbox PDB), `(cloning)`: a 0x5C-byte block
/// (`operator new`), constructed without data (`NiPathInterpolator(0, 0)`), and this
/// interpolator's members copied into it (`fn_00a3f2f0`, also run with a null destination
/// when the allocation fails). Answers the clone (null when the allocation failed).
pub fn ni_path_interpolator_create_clone(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    cloning: Ptr,
) -> Ptr<NiPathInterpolator> {
    let block = e.call(OPERATOR_NEW, &args![0x5cu32]).u32();
    let clone = if block != 0 {
        ni_path_interpolator_constructor(e, Ptr::new(block), Ptr::NULL, Ptr::NULL)
    } else {
        Ptr::NULL
    };
    fn_00a3f2f0(e, this, clone, cloning);
    clone
}

// Translated from 00a3f4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiPathInterpolator` factory (Ghidra made no function here; unnamed in the map):
/// a 0x5C-byte block (`operator new`) constructed without
/// data, or null when the allocation fails.
pub fn fn_00a3f4b0(e: &mut Engine) -> Ptr<NiPathInterpolator> {
    let block = e.call(OPERATOR_NEW, &args![0x5cu32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    ni_path_interpolator_constructor(e, Ptr::new(block), Ptr::NULL, Ptr::NULL)
}

/// `FLT_MAX` as a float (`01097170`), and the doubles `FLT_MAX` (`010231b0`) and `-FLT_MAX`
/// (`010241b0`) `GetActiveTimeRange` compares the range with.
const HIGHEST_FLOAT: u32 = 0x0109_7170;
const RANGE_UNSET_BEGIN: u32 = 0x0102_31b0;
const RANGE_UNSET_END: u32 = 0x0102_41b0;
/// `NiKeyBasedInterpolator`'s key at an index (`00a3ca80`, `thiscall`, (index, channel));
/// `NiKeyBasedInterpolator::RegisterStreamables` (`00aa0ad0`, `thiscall`, the stream; a
/// bool) and `GetViewerStrings` (`00a3cbf0`, `thiscall`, the array).
const KEY_BASED_INTERPOLATOR_GET_KEY_AT: u32 = 0x00a3_ca80;
const KEY_BASED_INTERPOLATOR_REGISTER_STREAMABLES: u32 = 0x00aa_0ad0;
const KEY_BASED_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a3_cbf0;
/// The base `SaveBinary` `00a95ca0` (the same address as the base `LinkObject` above; the
/// engine map's name for it is a folded one).
const KEY_BASED_INTERPOLATOR_SAVE_BINARY: u32 = 0x00a9_5ca0;
/// `NiFloatData::GuaranteeKeysAtStartAndEnd` (`00a3cc40`, `thiscall`, (start, end)) and
/// `NiFloatData::GetSequenceData` (`00a3ce20`, `thiscall`, (start, end, &data pointer)).
const FLOAT_DATA_GUARANTEE_KEYS: u32 = 0x00a3_cc40;
const FLOAT_DATA_GET_SEQUENCE_DATA: u32 = 0x00a3_ce20;
/// `NiInterpolator::GetSequenceInterpolator` (`00a57050`, `thiscall`, (start, end); answers
/// the new interpolator) and `NiObject::IsEqual` (`00c85800` as this class calls it,
/// `thiscall`, the other object; a bool).
const INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR: u32 = 0x00a5_7050;
const INTERPOLATOR_IS_EQUAL: u32 = 0x00c8_5800;
/// `NiGetViewerString` (`cdecl`, the text) and the viewer-string array's `SetSize`
/// (`0096ad30`) and `SetAt` (`0096ae90`, (index, address of the string)).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// Writes `size` bytes from `source` to the stream's output (`NiStream::m_pkOstr` +0x250):
/// its write function (`NiBinaryStream::m_pfnWrite` +0x0C) as `function(output, source, size,
/// &count, 1)` with the count word (the stack word at `count`) set to `size`, and the byte
/// count it answers added to the output's position (+0x04).
fn stream_write(e: &mut Engine, stream: Ptr, source: u32, size: u32, count: u32) {
    let output = e.mem.u32(stream.addr().wrapping_add(0x250));
    let function = e.mem.u32(output.wrapping_add(0xc));
    e.mem.set_u32(count, size);
    let done = e
        .call(function, &args![output, source, size, count, 1u32])
        .u32();
    let position = e.mem.u32(output.wrapping_add(4));
    e.mem
        .set_u32(output.wrapping_add(4), position.wrapping_add(done));
}

/// The data a channel names: 0 the path data, 1 the percentage data, any other none.
fn channel_data(e: &Engine, this: Ptr<NiPathInterpolator>, channel: u16) -> u32 {
    match channel {
        0 => e.get(this, NiPathInterpolator::m_spPathData),
        1 => e.get(this, NiPathInterpolator::m_spPctData),
        _ => 0,
    }
}

// Translated from 00a3da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetKeyContent` (Xbox PDB; `thiscall`, the channel, `RET 4`; `this`
/// is not read): 1 for channel 0, 0 for channel 1, 6 for any other.
pub fn ni_path_interpolator_get_key_content(_e: &mut Engine, _this: Ptr, channel: u16) -> u32 {
    if channel == 0 {
        return 1;
    }
    if channel.wrapping_sub(1) != 0 {
        6
    } else {
        0
    }
}

// Translated from 00a3dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetKeyCount` (Xbox PDB; `thiscall`, the channel): the key count
/// (+0x08) of the channel's data, 0 without it.
pub fn ni_path_interpolator_get_key_count(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    channel: u16,
) -> u32 {
    let data = channel_data(e, this, channel);
    if data == 0 {
        return 0;
    }
    e.mem.u32(data.wrapping_add(KEY_DATA_NUM_KEYS))
}

// Translated from 00a3db10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetKeyType` (Xbox PDB; `thiscall`, the channel): the key type
/// (+0x10) of the channel's data, 0 without it.
pub fn ni_path_interpolator_get_key_type(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    channel: u16,
) -> u32 {
    let data = channel_data(e, this, channel);
    if data == 0 {
        return 0;
    }
    e.mem.u32(data.wrapping_add(KEY_DATA_TYPE))
}

// Translated from 00a3db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetKeyArray` (Xbox PDB; `thiscall`, the channel): the keys (+0x0C)
/// of the channel's data, 0 without it.
pub fn ni_path_interpolator_get_key_array(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    channel: u16,
) -> u32 {
    let data = channel_data(e, this, channel);
    if data == 0 {
        return 0;
    }
    e.mem.u32(data.wrapping_add(KEY_DATA_KEYS))
}

// Translated from 00a3db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetKeyStride` (Xbox PDB; `thiscall`, the channel; the answer is the
/// low byte): the key size (+0x14) of the channel's data, 0 without it.
pub fn ni_path_interpolator_get_key_stride(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    channel: u16,
) -> u8 {
    let data = channel_data(e, this, channel);
    if data == 0 {
        return 0;
    }
    e.mem.u8(data.wrapping_add(KEY_DATA_KEY_SIZE))
}

// Translated from 00a3dba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetActiveTimeRange` (Xbox PDB; `thiscall`, the addresses of the
/// begin and end times, `RET 8`): the range starts as (`FLT_MAX`, `-FLT_MAX`); when this
/// interpolator's `GetKeyCount(1)` (its slot +0xE0) is not 0 and the first and last
/// percentage keys (`00a3ca80`, channel 1) both exist, the first key's time lowers the begin
/// and the last key's time raises the end. A range still at (`FLT_MAX`, `-FLT_MAX`) (the
/// doubles at `010231b0`, `010241b0`) becomes (0, 0).
pub fn ni_path_interpolator_get_active_time_range(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    begin: Ptr,
    end: Ptr,
) {
    let highest = x87_quiet_bits(e.mem.u32(HIGHEST_FLOAT));
    e.mem.set_u32(begin.addr(), highest);
    let lowest = x87_quiet_bits(e.mem.u32(LOWEST_FLOAT));
    e.mem.set_u32(end.addr(), lowest);
    if e.vcall(this.addr(), 0xe0, &args![1u32]).u32() != 0 {
        let first = e
            .call(KEY_BASED_INTERPOLATOR_GET_KEY_AT, &args![this, 0u32, 1u32])
            .u32();
        let count = e.vcall(this.addr(), 0xe0, &args![1u32]).u32();
        let last = e
            .call(
                KEY_BASED_INTERPOLATOR_GET_KEY_AT,
                &args![this, count.wrapping_sub(1), 1u32],
            )
            .u32();
        if first != 0 && last != 0 {
            if x87_load32(e.mem.u32(begin.addr())) > x87_load32(e.mem.u32(first)) {
                let time = x87_quiet_bits(e.mem.u32(first));
                e.mem.set_u32(begin.addr(), time);
            }
            if x87_load32(e.mem.u32(end.addr())) < x87_load32(e.mem.u32(last)) {
                let time = x87_quiet_bits(e.mem.u32(last));
                e.mem.set_u32(end.addr(), time);
            }
        }
    }
    let unset_begin = f64::from_bits(e.mem.u64(RANGE_UNSET_BEGIN));
    let unset_end = f64::from_bits(e.mem.u64(RANGE_UNSET_END));
    if x87_load32(e.mem.u32(begin.addr())) == unset_begin
        && x87_load32(e.mem.u32(end.addr())) == unset_end
    {
        e.mem.set_u32(begin.addr(), x87_store32(0.0));
        e.mem.set_u32(end.addr(), x87_store32(0.0));
    }
}

// Translated from 00a3dc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GuaranteeTimeRange` (Xbox PDB; `thiscall`, (start, end), `RET 8`):
/// with percentage data, its `GuaranteeKeysAtStartAndEnd` (`00a3cc40`) on the two times
/// (copied through the x87 stack).
pub fn ni_path_interpolator_guarantee_time_range(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    start: f32,
    end: f32,
) {
    let percentage = e.get(this, NiPathInterpolator::m_spPctData);
    if percentage != 0 {
        e.call(
            FLOAT_DATA_GUARANTEE_KEYS,
            &args![
                percentage,
                x87_quiet_bits(start.to_bits()),
                x87_quiet_bits(end.to_bits())
            ],
        );
    }
}

// Translated from 00a3dcc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::RegisterStreamables` (Xbox PDB; `thiscall`, the stream): false when
/// `NiKeyBasedInterpolator::RegisterStreamables` (`00aa0ad0`) says so; otherwise the path
/// and percentage data's own (slot +0x54) when present, and true.
pub fn ni_path_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    stream: Ptr,
) -> bool {
    if !e
        .call(
            KEY_BASED_INTERPOLATOR_REGISTER_STREAMABLES,
            &args![this, stream],
        )
        .bool()
    {
        return false;
    }
    for field in [
        NiPathInterpolator::m_spPathData,
        NiPathInterpolator::m_spPctData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            e.vcall(object, 0x54, &args![stream]);
        }
    }
    true
}

// Translated from 00a3dd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path data's keys (unnamed in the map; `thiscall`, (&count, &key type, &key size),
/// `RET 0xC`): the key count, type and size (one byte) written out and the keys answered;
/// without path data all three are 0 and so is the answer.
pub fn fn_00a3dd00(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    count: Ptr,
    key_type: Ptr,
    size: Ptr,
) -> u32 {
    let data = e.get(this, NiPathInterpolator::m_spPathData);
    if data == 0 {
        e.mem.set_u32(count.addr(), 0);
        e.mem.set_u32(key_type.addr(), 0);
        e.mem.set_u8(size.addr(), 0);
        return 0;
    }
    let value = e.mem.u32(data.wrapping_add(KEY_DATA_NUM_KEYS));
    e.mem.set_u32(count.addr(), value);
    let value = e.mem.u32(data.wrapping_add(KEY_DATA_TYPE));
    e.mem.set_u32(key_type.addr(), value);
    let value = e.mem.u8(data.wrapping_add(KEY_DATA_KEY_SIZE));
    e.mem.set_u8(size.addr(), value);
    e.mem.u32(data.wrapping_add(KEY_DATA_KEYS))
}

// Translated from 00a3e6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetSequenceInterpolator` (Xbox PDB; `thiscall`, (start, end), `RET
/// 8`): `NiInterpolator::GetSequenceInterpolator` (`00a57050`) makes the new interpolator;
/// with percentage data, that data's `GetSequenceData` (`00a3ce20`) for the times goes into
/// a local smart pointer, which is assigned (counted) to the new interpolator's percentage
/// data, its `m_uiLastPctIdx` cleared, and the local released. Answers the new one.
pub fn ni_path_interpolator_get_sequence_interpolator(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    start: f32,
    end: f32,
) -> u32 {
    let start = x87_quiet_bits(start.to_bits());
    let end = x87_quiet_bits(end.to_bits());
    let sequence = e
        .call(
            INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR,
            &args![this, start, end],
        )
        .u32();
    let percentage = e.get(this, NiPathInterpolator::m_spPctData);
    if percentage == 0 {
        return sequence;
    }
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        e.call(
            FLOAT_DATA_GET_SEQUENCE_DATA,
            &args![percentage, start, end, local],
        );
        let data = e.mem.u32(local.addr());
        assign_counted(e, sequence.wrapping_add(0x1c), data);
        e.mem.set_u32(sequence.wrapping_add(0x14), 0);
        let data = e.mem.u32(local.addr());
        if data != 0 {
            release(e, data);
        }
    });
    sequence
}

// Translated from 00a3e7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::IsEqual` (Xbox PDB; `thiscall`, the other object): the base
/// `IsEqual` (`00c85800`); the path and the percentage data each present in both or in
/// neither, and equal by their own `IsEqual` (slot +0x5C); the partial lengths present in
/// both or neither, and with both the first count * 4 bytes equal (the count from
/// `fn_00a3dd00`, compared a word at a time and the rest byte by byte); then the total
/// length, flag bits 0 and 1, the bank direction, flag bits 2 to 5, the bank angle, the
/// smoothing, the follow axis and flag bit 6 (floats by `FUCOMPP`: a NaN is never equal).
pub fn ni_path_interpolator_is_equal(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    other: Ptr<NiPathInterpolator>,
) -> bool {
    if !e.call(INTERPOLATOR_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    for field in [
        NiPathInterpolator::m_spPathData,
        NiPathInterpolator::m_spPctData,
    ] {
        let own = e.get(this, field);
        if (own == 0) != (e.get(other, field) == 0) {
            return false;
        }
        if own != 0 {
            let theirs = e.get(other, field);
            if !e.vcall(own, 0x5c, &args![theirs]).bool() {
                return false;
            }
        }
    }
    let own = e.get(this, NiPathInterpolator::m_pfPartialLength);
    if (own == 0) != (e.get(other, NiPathInterpolator::m_pfPartialLength) == 0) {
        return false;
    }
    if own != 0 {
        // Locals: the count, the key type, and the other-object argument's slot for the
        // key size.
        let count = e.with_stack(0xc, |e, locals| {
            let base = locals.addr();
            fn_00a3dd00(
                e,
                this,
                Ptr::new(base),
                Ptr::new(base + 4),
                Ptr::new(base + 8),
            );
            e.mem.u32(base)
        });
        let mut left = count.wrapping_mul(4);
        let mut mine = own;
        let mut theirs = e.get(other, NiPathInterpolator::m_pfPartialLength);
        while left >= 4 {
            if e.mem.u32(mine) != e.mem.u32(theirs) {
                return false;
            }
            left -= 4;
            theirs = theirs.wrapping_add(4);
            mine = mine.wrapping_add(4);
        }
        for byte in 0..left {
            if e.mem.u8(theirs.wrapping_add(byte)) != e.mem.u8(mine.wrapping_add(byte)) {
                return false;
            }
        }
    }
    let total = x87_load32(e.get(this, NiPathInterpolator::m_fTotalLength));
    if !(x87_load32(e.get(other, NiPathInterpolator::m_fTotalLength)) == total) {
        return false;
    }
    let theirs = flags_byte(e, other);
    let mine = flags_byte(e, this);
    let differs = |bit: u32| ((theirs >> bit) ^ (mine >> bit)) & 1 != 0;
    if differs(0) || differs(1) {
        return false;
    }
    if e.get(this, NiPathInterpolator::m_eBankDir) != e.get(other, NiPathInterpolator::m_eBankDir) {
        return false;
    }
    if differs(2) || differs(3) || differs(4) || differs(5) {
        return false;
    }
    for field in [
        NiPathInterpolator::m_fMaxBankAngle,
        NiPathInterpolator::m_fSmoothing,
    ] {
        let value = x87_load32(e.get(this, field));
        if !(x87_load32(e.get(other, field)) == value) {
            return false;
        }
    }
    if e.get(this, NiPathInterpolator::m_sFollowAxis)
        != e.get(other, NiPathInterpolator::m_sFollowAxis)
    {
        return false;
    }
    !differs(6)
}

// Translated from 00a3ea40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`): when the "lengths stale" flag (bit 0) is set, the total
/// length is recomputed (`fn_00a3e9a0`, stored as a float) and the flag cleared.
pub fn fn_00a3ea40(e: &mut Engine, this: Ptr<NiPathInterpolator>) {
    if flags_byte(e, this) & 1 == 0 {
        return;
    }
    let total = fn_00a3e9a0(e, this);
    e.set(
        this,
        NiPathInterpolator::m_fTotalLength,
        x87_store32(x87_load32(total.to_bits())),
    );
    let flags = e.get(this, NiPathInterpolator::m_uFlags) & 0xfffe;
    e.set(this, NiPathInterpolator::m_uFlags, flags);
}

// Translated from 00a3ef70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base
/// `SaveBinary` (`00a95ca0`), then the flags (2 bytes), the bank direction, the bank angle
/// and the smoothing (4 bytes each) and the follow axis (2 bytes), the count word being the
/// stream argument's own stack slot; then the stream's `SaveLinkID` (slot +0x30) of the path
/// and the percentage data.
pub fn ni_path_interpolator_save_binary(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    stream: Ptr,
) {
    let t = this.addr();
    e.call(KEY_BASED_INTERPOLATOR_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        let count = count.addr();
        stream_write(e, stream, t + 0xc, 2, count);
        stream_write(e, stream, t + 0x38, 4, count);
        stream_write(e, stream, t + 0x28, 4, count);
        stream_write(e, stream, t + 0x2c, 4, count);
        stream_write(e, stream, t + 0x30, 2, count);
    });
    for field in [
        NiPathInterpolator::m_spPathData,
        NiPathInterpolator::m_spPctData,
    ] {
        let object = e.get(this, field);
        e.vcall(stream.addr(), 0x30, &args![object]);
    }
}

// Translated from 00a3f4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::GetViewerStrings` (Xbox PDB; `thiscall`, the viewer-string array):
/// `NiKeyBasedInterpolator::GetViewerStrings` (`00a3cbf0`), then the class name (the first
/// word of the `NiRTTI` record at `011f3e48`, through `NiGetViewerString`) appended: the
/// array grown by its step when full, then `SetAt` at the old size with the address of the
/// string's stack word.
pub fn ni_path_interpolator_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    strings: Ptr,
) {
    e.call(
        KEY_BASED_INTERPOLATOR_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    let class_name = e.global::<u32>(NI_PATH_INTERPOLATOR_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_SET_SIZE, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// The fields `CalcRefFrame` reads, as offsets in the object. `NiPathInterpolator::
/// CalcRefFrame` (`00a3de50`) and `NiPathController::CalcRefFrame` (`00a49e70`) are the same
/// code instruction by instruction but for these offsets and the address of their
/// function-local `static` (the bank scale, `4 / pi` from `01097304`, set on first use).
pub(crate) struct RefFrameFields {
    pub(crate) path_data: u32,
    pub(crate) flags: u32,
    pub(crate) max_bank_angle: u32,
    pub(crate) smoothing: u32,
    pub(crate) follow_axis: u32,
    pub(crate) max_curvature: u32,
    pub(crate) bank_dir: u32,
    pub(crate) bank_scale: u32,
    pub(crate) bank_scale_guard: u32,
}

/// `NiPathInterpolator`'s offsets for [`calc_ref_frame`].
const INTERPOLATOR_REF_FRAME: RefFrameFields = RefFrameFields {
    path_data: 0x18,
    flags: 0xc,
    max_bank_angle: 0x28,
    smoothing: 0x2c,
    follow_axis: 0x30,
    max_curvature: 0x34,
    bank_dir: 0x38,
    bank_scale: 0x011f_3e40,
    bank_scale_guard: 0x011f_3e44,
};

/// The rotation matrix a frame starts from (`011a9448`, nine floats), the point
/// `NiPoint3::UnitCross` runs on for the unbanked frame (`011a9484`) and the float `4 / pi`
/// (`01097304`).
const REF_FRAME_DEFAULT_MATRIX: u32 = 0x011a_9448;
const REF_FRAME_UP: u32 = 0x011a_9484;
const FOUR_OVER_PI: u32 = 0x0109_7304;
/// `00a277c0` (unnamed in the map; `cdecl`, (ratio, key, next key, key type, &tangent,
/// &second vector, &third vector, &curvature)): the path's frame vectors and curvature
/// between two position keys.
const POS_KEY_FRAME: u32 = 0x00a2_77c0;
/// `NiPoint3::UnitCross` (`0053d1a0`, `thiscall` on a point, (result, other)).
const POINT_UNIT_CROSS: u32 = 0x0053_d1a0;
/// Unnamed in the map, by their bodies: a point times a float (`004a3760`, `cdecl`,
/// (result, factor, &point)), the sum of two points (`00439e90`, `thiscall`, (result,
/// &other)), the cross product (`004b3800`, `thiscall`, (result, &other)), all answering
/// the result; `004a0c10` (`thiscall` on a point): its unitizing; `004168a0` (`thiscall` on
/// a matrix, (angle, axis x, y, z)): the rotation about an axis.
const POINT_SCALE: u32 = 0x004a_3760;
const POINT_ADD: u32 = 0x0043_9e90;
const POINT_UNITIZE: u32 = 0x004a_0c10;
const POINT_CROSS: u32 = 0x004b_3800;
const MATRIX_MAKE_ROTATION: u32 = 0x0041_68a0;
/// `NiMatrix3::operator*` (`0043f8d0`, `thiscall` on the left matrix, (result, &right);
/// answers the result) and `_CIatan` (`00ec9050`, ST0).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
const CI_ATAN: u32 = 0x00ec_9050;

/// The shared body of the two `CalcRefFrame`s (see [`RefFrameFields`]), on the object at
/// `this`, into the nine floats at `matrix`. The frame starts as the matrix at `011a9448`;
/// with path data of at least two keys, the key size and type give the keys `first` and
/// `second`, and:
///
/// - without flag bit 2: `00a277c0` at `ratio` gives the tangent `t` (and the curvature);
///   the columns become `t`, `UnitCross(011a9484, t)` and `t` crossed with it (x87, each
///   component stored as a float);
/// - with flag bit 2: `00a277c0` at `ratio` gives the tangent, a second vector `n`, a third
///   `b` and the curvature; with a non-zero smoothing `s`, a ratio at or past `1 - s`
///   blends with the segment after (when there is one: `second`, or 0 for the last on a
///   closed path, flag bit 1 clear) and a ratio up to `s` with the segment before (`first`,
///   or the last for 0 on a closed path; none for 0): the frame at `1 - s` (or `s`) of this
///   segment and at `s` (or `1 - s`) of the other give `n` as the unitized weighted sum
///   (weights `(1 - ratio + s) / 2s` and its complement, or `(s - ratio) / 2s` and its
///   complement), the curvature the same blend, and `b = t x n`; the columns become `t`,
///   `-n` and `-b`.
///
/// Then flag bit 6 negates the first two columns; follow axis 1 makes the columns
/// (`-col1`, `col0`, `col2`) and axis 2 (`-col2`, `col1`, `col0`); with flag bit 3 the
/// frame is turned (`004168a0`, `NiMatrix3::operator*` on the left) about `t` by the bank
/// direction times the maximum bank angle, scaled when the curvature is below the maximum
/// by `atan(curvature / max) * 4 / pi`. The frame is copied to `matrix`.
pub(crate) fn calc_ref_frame(
    e: &mut Engine,
    this: u32,
    fields: &RefFrameFields,
    first: u32,
    second: u32,
    ratio: f32,
    matrix: u32,
) {
    // The function's stack frame: the 0xF8 bytes below the arguments, the offsets the
    // exe's own (relative to its stack pointer after the prologue).
    e.with_stack(0xf8, |e, frame| {
        let b = frame.addr();
        let l = |k: u32| b + k;
        let m = l(0x4c);
        for word in 0..9 {
            let value = e.mem.u32(REF_FRAME_DEFAULT_MATRIX + 4 * word);
            e.mem.set_u32(m + 4 * word, value);
        }
        let data = e.mem.u32(this + fields.path_data);
        if data != 0 {
            let count = e.mem.u32(data + 8);
            let key_type = e.mem.u32(data + 0x10);
            let size = u32::from(e.mem.u8(data + 0x14));
            let keys = e.mem.u32(data + 0xc);
            e.mem.set_u32(l(0x1c), count);
            if count >= 2 {
                build_frame(
                    e,
                    this,
                    fields,
                    (first, second, ratio),
                    (keys, size, key_type),
                    b,
                );
            }
        }
        for word in 0..9 {
            let value = e.mem.u32(m + 4 * word);
            e.mem.set_u32(matrix + 4 * word, value);
        }
    });
}

/// [`calc_ref_frame`] for path data of at least two keys: everything but the default
/// matrix and the final copy. `b` is the base of the exe's stack frame.
fn build_frame(
    e: &mut Engine,
    this: u32,
    fields: &RefFrameFields,
    (first, second, ratio): (u32, u32, f32),
    (keys, size, key_type): (u32, u32, u32),
    b: u32,
) {
    let l = |k: u32| b + k;
    let float = |e: &Engine, k: u32| x87_load32(e.mem.u32(b + k));
    let set_float = |e: &mut Engine, k: u32, value: f64| e.mem.set_u32(b + k, x87_store32(value));
    let quiet = |e: &Engine, k: u32| x87_quiet_bits(e.mem.u32(b + k));
    let copy = |e: &mut Engine, to: u32, from: u32| {
        let word = x87_quiet_bits(e.mem.u32(b + from));
        e.mem.set_u32(b + to, word);
    };
    let negate = |e: &mut Engine, to: u32, from: u32| {
        let value = -x87_load32(e.mem.u32(b + from));
        e.mem.set_u32(b + to, x87_store32(value));
    };
    let flags_now = e.mem.u8(this + fields.flags);
    let count = e.mem.u32(l(0x1c));
    let r = x87_load32(x87_quiet_bits(ratio.to_bits()));
    let key = |index: u32| size.wrapping_mul(index).wrapping_add(keys);
    // The matrix columns, by the offsets of their three entries.
    const COLUMN_0: [u32; 3] = [0x4c, 0x58, 0x64];
    const COLUMN_1: [u32; 3] = [0x50, 0x5c, 0x68];
    const COLUMN_2: [u32; 3] = [0x54, 0x60, 0x6c];
    if (flags_now >> 2) & 1 != 0 {
        let key1 = key(first);
        let key2 = key(second);
        e.mem.set_u32(l(0x88), key2);
        e.mem.set_u32(l(0x20), key1);
        e.call(
            POS_KEY_FRAME,
            &args![
                x87_store32(r),
                key1,
                key2,
                key_type,
                l(0x70),
                l(0x40),
                l(0x7c),
                l(0x24)
            ],
        );
        let s = x87_load32(e.mem.u32(this + fields.smoothing));
        if !(0.0 == x87_m32(e.mem.u32(this + fields.smoothing))) {
            let one_minus_s = x87_sub(1.0, s);
            if one_minus_s <= r {
                // The ratio is in the end zone: blend with the next segment.
                let wraps = (e.mem.u8(this + fields.flags) >> 1) & 1 == 0;
                let index = if wraps && second == count.wrapping_sub(1) {
                    0
                } else {
                    second
                };
                e.mem.set_u32(l(0x2c), index);
                if index < count.wrapping_sub(1) {
                    let one_minus_r = x87_sub(1.0, r);
                    let s = x87_load32(e.mem.u32(this + fields.smoothing));
                    let sum = x87_add(one_minus_r, s);
                    let s = x87_load32(e.mem.u32(this + fields.smoothing));
                    let weight = x87_div(sum, x87_add(s, s));
                    let key2 = e.mem.u32(l(0x88));
                    let key1 = e.mem.u32(l(0x20));
                    set_float(e, 0x1c, weight);
                    let other = x87_sub(1.0, float(e, 0x1c));
                    set_float(e, 0x30, other);
                    let start = x87_sub(1.0, x87_m32(e.mem.u32(this + fields.smoothing)));
                    set_float(e, 0x20, start);
                    e.call(
                        POS_KEY_FRAME,
                        &args![
                            quiet(e, 0x20),
                            key1,
                            key2,
                            key_type,
                            l(0x34),
                            l(0x40),
                            l(0x7c),
                            l(0x24)
                        ],
                    );
                    let smoothing = x87_quiet_bits(e.mem.u32(this + fields.smoothing));
                    let index = e.mem.u32(l(0x2c));
                    e.call(
                        POS_KEY_FRAME,
                        &args![
                            smoothing,
                            key(index),
                            key(index.wrapping_add(1)),
                            key_type,
                            l(0x34),
                            l(0x10),
                            l(0x7c),
                            l(0x28)
                        ],
                    );
                    let part = e
                        .call(POINT_SCALE, &args![l(0xa4), quiet(e, 0x30), l(0x10)])
                        .u32();
                    let whole = e
                        .call(POINT_SCALE, &args![l(0x8c), quiet(e, 0x1c), l(0x40)])
                        .u32();
                    let sum = e.call(POINT_ADD, &args![whole, l(0x98), part]).u32();
                    blend_finish(e, b, sum);
                }
            } else if s >= r {
                // The ratio is in the start zone: blend with the previous segment.
                let wraps = (e.mem.u8(this + fields.flags) >> 1) & 1 == 0;
                let index = if wraps && first == 0 {
                    count.wrapping_sub(1)
                } else {
                    first
                };
                e.mem.set_u32(l(0x2c), index);
                if index >= 1 {
                    let s = x87_load32(e.mem.u32(this + fields.smoothing));
                    let difference = x87_sub(s, r);
                    let key2 = e.mem.u32(l(0x88));
                    let s = x87_load32(e.mem.u32(this + fields.smoothing));
                    let weight = x87_div(difference, x87_add(s, s));
                    let key1 = e.mem.u32(l(0x20));
                    set_float(e, 0x30, weight);
                    let other = x87_sub(1.0, x87_m32(e.mem.u32(l(0x30))));
                    set_float(e, 0x1c, other);
                    let smoothing = x87_quiet_bits(e.mem.u32(this + fields.smoothing));
                    e.call(
                        POS_KEY_FRAME,
                        &args![
                            smoothing,
                            key1,
                            key2,
                            key_type,
                            l(0x34),
                            l(0x40),
                            l(0x7c),
                            l(0x24)
                        ],
                    );
                    let end = x87_sub(1.0, x87_load32(e.mem.u32(this + fields.smoothing)));
                    set_float(e, 0x20, end);
                    let index = e.mem.u32(l(0x2c));
                    e.call(
                        POS_KEY_FRAME,
                        &args![
                            quiet(e, 0x20),
                            key(index.wrapping_sub(1)),
                            key(index),
                            key_type,
                            l(0x34),
                            l(0x10),
                            l(0x7c),
                            l(0x28)
                        ],
                    );
                    let part = e
                        .call(POINT_SCALE, &args![l(0x8c), quiet(e, 0x30), l(0x10)])
                        .u32();
                    let whole = e
                        .call(POINT_SCALE, &args![l(0xa4), quiet(e, 0x1c), l(0x40)])
                        .u32();
                    let sum = e.call(POINT_ADD, &args![whole, l(0x98), part]).u32();
                    blend_finish(e, b, sum);
                }
            }
        }
        // Columns: the tangent, minus the second vector, minus the third.
        for (row, from) in [0x70u32, 0x74, 0x78].into_iter().enumerate() {
            copy(e, COLUMN_0[row], from);
        }
        for (row, from) in [0x40u32, 0x44, 0x48].into_iter().enumerate() {
            negate(e, 0x10 + 4 * row as u32, from);
        }
        for row in 0..3 {
            copy(e, COLUMN_1[row], 0x10 + 4 * row as u32);
        }
        for (row, from) in [0x7cu32, 0x80, 0x84].into_iter().enumerate() {
            negate(e, 0x10 + 4 * row as u32, from);
        }
        for row in 0..3 {
            copy(e, COLUMN_2[row], 0x10 + 4 * row as u32);
        }
    } else {
        e.call(
            POS_KEY_FRAME,
            &args![
                x87_store32(r),
                key(first),
                key(second),
                key_type,
                l(0x70),
                l(0x40),
                l(0x7c),
                l(0x24)
            ],
        );
        for (row, from) in [0x70u32, 0x74, 0x78].into_iter().enumerate() {
            copy(e, COLUMN_0[row], from);
        }
        e.call(POINT_UNIT_CROSS, &args![REF_FRAME_UP, l(0x34), l(0x70)]);
        for (row, from) in [0x34u32, 0x38, 0x3c].into_iter().enumerate() {
            copy(e, COLUMN_1[row], from);
        }
        let (cx, cy, cz) = (float(e, 0x34), float(e, 0x38), float(e, 0x3c));
        let (tx, ty, tz) = (float(e, 0x70), float(e, 0x74), float(e, 0x78));
        let x = x87_sub(x87_mul(cz, ty), x87_mul(cy, tz));
        set_float(e, 0x10, x);
        let y = x87_sub(x87_mul(tz, cx), x87_mul(cz, tx));
        set_float(e, 0x14, y);
        let z = x87_sub(x87_mul(cy, tx), x87_mul(cx, ty));
        set_float(e, 0x18, z);
        for row in 0..3 {
            copy(e, COLUMN_2[row], 0x10 + 4 * row as u32);
        }
    }
    // The tangent as the FPU still holds it (the floats loaded from it).
    let tangent = [quiet(e, 0x70), quiet(e, 0x74), quiet(e, 0x78)];
    let flags = e.mem.u8(this + fields.flags);
    if (flags >> 6) & 1 != 0 {
        for row in 0..3 {
            let value = -x87_load32(tangent[row]);
            e.mem.set_u32(l(0x10 + 4 * row as u32), x87_store32(value));
        }
        for row in 0..3 {
            copy(e, COLUMN_0[row], 0x10 + 4 * row as u32);
        }
        for row in 0..3 {
            negate(e, 0x10 + 4 * row as u32, COLUMN_1[row]);
        }
        for row in 0..3 {
            copy(e, COLUMN_1[row], 0x10 + 4 * row as u32);
        }
    }
    let axis = e.mem.u16(this + fields.follow_axis) as i16;
    let swapped = match axis {
        1 => Some(COLUMN_1),
        2 => Some(COLUMN_2),
        _ => None,
    };
    if let Some(column) = swapped {
        for row in 0..3 {
            copy(e, 0x10 + 4 * row as u32, COLUMN_0[row]);
        }
        for row in 0..3 {
            negate(e, 0x34 + 4 * row as u32, column[row]);
        }
        for row in 0..3 {
            copy(e, COLUMN_0[row], 0x34 + 4 * row as u32);
        }
        for row in 0..3 {
            copy(e, column[row], 0x10 + 4 * row as u32);
        }
    }
    if (flags >> 3) & 1 == 0 {
        return;
    }
    if e.mem.u8(fields.bank_scale_guard) & 1 == 0 {
        let scale = x87_quiet_bits(e.mem.u32(FOUR_OVER_PI));
        let guard = e.mem.u32(fields.bank_scale_guard);
        e.mem.set_u32(fields.bank_scale_guard, guard | 1);
        e.mem.set_u32(fields.bank_scale, scale);
    }
    let direction = f64::from(e.mem.u32(this + fields.bank_dir) as i32);
    let angle = x87_mul(direction, x87_m32(e.mem.u32(this + fields.max_bank_angle)));
    set_float(e, 0x28, angle);
    let curvature = float(e, 0x24);
    if x87_load32(e.mem.u32(this + fields.max_curvature)) > curvature {
        let part = x87_div(curvature, x87_m32(e.mem.u32(this + fields.max_curvature)));
        set_float(e, 0x20, part);
        let turned = e.call(CI_ATAN, &args![float(e, 0x20)]).f64();
        set_float(e, 0x20, turned);
        let scaled = x87_mul(float(e, 0x20), x87_m32(e.mem.u32(fields.bank_scale)));
        let scaled = x87_mul(scaled, x87_m32(e.mem.u32(l(0x28))));
        set_float(e, 0x28, scaled);
    }
    e.call(
        MATRIX_MAKE_ROTATION,
        &args![l(0xb0), quiet(e, 0x28), tangent[0], tangent[1], tangent[2]],
    );
    let product = e
        .call(MATRIX_MULTIPLY, &args![l(0xb0), l(0xd4), l(0x4c)])
        .u32();
    for word in 0..9 {
        let value = e.mem.u32(product + 4 * word);
        e.mem.set_u32(l(0x4c) + 4 * word, value);
    }
}

/// The end of either smoothing blend: the sum (at `sum`) becomes the second vector, which
/// is unitized (`004a0c10`); the curvature becomes the blend `d0 * c8 + dc * d4` (the
/// other segment's curvature and its weight, this segment's weight and curvature) and the
/// third vector the tangent crossed with the second (`004b3800`).
fn blend_finish(e: &mut Engine, b: u32, sum: u32) {
    for word in 0..3 {
        let value = e.mem.u32(sum + 4 * word);
        e.mem.set_u32(b + 0x40 + 4 * word, value);
    }
    e.call(POINT_UNITIZE, &args![b + 0x40]);
    let other = x87_mul(
        x87_load32(e.mem.u32(b + 0x28)),
        x87_m32(e.mem.u32(b + 0x30)),
    );
    let own = x87_mul(
        x87_load32(e.mem.u32(b + 0x1c)),
        x87_m32(e.mem.u32(b + 0x24)),
    );
    e.mem.set_u32(b + 0x24, x87_store32(x87_add(other, own)));
    let cross = e
        .call(POINT_CROSS, &args![b + 0x70, b + 0x8c, b + 0x40])
        .u32();
    for word in 0..3 {
        let value = e.mem.u32(cross + 4 * word);
        e.mem.set_u32(b + 0x7c + 4 * word, value);
    }
}

// Translated from 00a3de50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathInterpolator::CalcRefFrame` (Xbox PDB; `thiscall`, (first key, second key, ratio,
/// &matrix), `RET 0x10`): the path's frame at `ratio` between the two keys, as
/// [`calc_ref_frame`] describes it, with this class's fields (path data +0x18, flags
/// +0xC, bank angle +0x28, smoothing +0x2C, follow axis +0x30, maximum curvature +0x34,
/// bank direction +0x38; the bank scale `static` at `011f3e40`, guard `011f3e44`).
pub fn ni_path_interpolator_calc_ref_frame(
    e: &mut Engine,
    this: Ptr<NiPathInterpolator>,
    first: u32,
    second: u32,
    ratio: f32,
    matrix: Ptr,
) {
    calc_ref_frame(
        e,
        this.addr(),
        &INTERPOLATOR_REF_FRAME,
        first,
        second,
        ratio,
        matrix.addr(),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3dab0, ni_pos_data_get_rtti() -> Ptr),
        entry!(0x00a3dde0, ni_path_interpolator_get_rtti() -> Ptr),
        entry!(0x00a3dac0, ni_pos_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a3e980, ni_path_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a3dd40, fn_00a3dd40(Ptr<NiPathInterpolator>)),
        entry!(
            0x00a3ddf0,
            fn_00a3ddf0(Ptr<NiPathInterpolator>, Ptr, u32, u32, f32) -> Ptr
        ),
        entry!(
            0x00a3dc80,
            ni_path_interpolator_process_clone(Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3e5f0,
            fn_00a3e5f0(Ptr<NiPathInterpolator>, u32, u32, f32) -> f32
        ),
        entry!(
            0x00a3e680,
            fn_00a3e680(Ptr<NiPathInterpolator>, u32, u32, f32) -> f32
        ),
        entry!(0x00a3e9a0, fn_00a3e9a0(Ptr<NiPathInterpolator>) -> f32),
        entry!(
            0x00a3ea60,
            fn_00a3ea60(Ptr<NiPathInterpolator>, f32, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a3ec30,
            fn_00a3ec30(Ptr<NiPathInterpolator>, f32, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a3ed90,
            ni_path_interpolator_load_binary(Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3ee80,
            ni_path_interpolator_link_object(Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3f060,
            ni_path_interpolator_constructor(
                Ptr<NiPathInterpolator>,
                Ptr,
                Ptr,
            ) -> Ptr<NiPathInterpolator>
        ),
        entry!(
            0x00a3f120,
            ni_path_interpolator_update(Ptr<NiPathInterpolator>, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a3f2f0,
            fn_00a3f2f0(Ptr<NiPathInterpolator>, Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3f520,
            ni_path_interpolator_create_clone(
                Ptr<NiPathInterpolator>,
                Ptr,
            ) -> Ptr<NiPathInterpolator>
        ),
        entry!(0x00a3f4b0, fn_00a3f4b0() -> Ptr<NiPathInterpolator>),
        entry!(
            0x00a3_da80,
            ni_path_interpolator_get_key_content(Ptr, u16) -> u32
        ),
        entry!(
            0x00a3_dae0,
            ni_path_interpolator_get_key_count(Ptr<NiPathInterpolator>, u16) -> u32
        ),
        entry!(
            0x00a3_db10,
            ni_path_interpolator_get_key_type(Ptr<NiPathInterpolator>, u16) -> u32
        ),
        entry!(
            0x00a3_db40,
            ni_path_interpolator_get_key_array(Ptr<NiPathInterpolator>, u16) -> u32
        ),
        entry!(
            0x00a3_db70,
            ni_path_interpolator_get_key_stride(Ptr<NiPathInterpolator>, u16) -> u8
        ),
        entry!(
            0x00a3_dba0,
            ni_path_interpolator_get_active_time_range(Ptr<NiPathInterpolator>, Ptr, Ptr)
        ),
        entry!(
            0x00a3_dc50,
            ni_path_interpolator_guarantee_time_range(Ptr<NiPathInterpolator>, f32, f32)
        ),
        entry!(
            0x00a3_dcc0,
            ni_path_interpolator_register_streamables(Ptr<NiPathInterpolator>, Ptr) -> bool
        ),
        entry!(
            0x00a3_dd00,
            fn_00a3dd00(Ptr<NiPathInterpolator>, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00a3_e6f0,
            ni_path_interpolator_get_sequence_interpolator(
                Ptr<NiPathInterpolator>,
                f32,
                f32,
            ) -> u32
        ),
        entry!(
            0x00a3_e7b0,
            ni_path_interpolator_is_equal(Ptr<NiPathInterpolator>, Ptr<NiPathInterpolator>) -> bool
        ),
        entry!(0x00a3_ea40, fn_00a3ea40(Ptr<NiPathInterpolator>)),
        entry!(
            0x00a3_ef70,
            ni_path_interpolator_save_binary(Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3_f4d0,
            ni_path_interpolator_get_viewer_strings(Ptr<NiPathInterpolator>, Ptr)
        ),
        entry!(
            0x00a3_de50,
            ni_path_interpolator_calc_ref_frame(Ptr<NiPathInterpolator>, u32, u32, f32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_pos_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a3dab0, &args![]).u32(), NI_POS_DATA_RTTI);
    }

    #[test]
    fn ni_path_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3dde0, &args![]).u32(),
            NI_PATH_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_pos_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_8fb0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a3dac0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a3dac0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, 0x00a4_8fb0), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_path_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a3_dd40, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a3e980, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a3e980, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x5c]]);
        assert_eq!(calls_to(&e, 0x00a3_dd40), vec![vec![this]; 2]);
    }

    /// The argument words of every logged call to `addr`.
    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine logging calls, with the interlocked imports working on memory, the
    /// constants the constructor and the searches read, and doubles for the allocator.
    fn path_engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        e.register(TABLE_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0].max(4)),
            ..Ret::default()
        });
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(CRT_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e.register(CI_FMOD, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let y = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            Ret {
                st0: x % y,
                ..Ret::default()
            }
        });
        for page in [
            0x011a_8000u32,
            0x011a_9000,
            0x011f_3000,
            0x011f_4000,
            0x0101_2000,
            0x0101_5000,
            0x0103_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global::<f64>(ARC_LENGTH_TOLERANCE, f64::from_bits(0x3f1a_36e2_e000_0000));
        e.set_global::<f32>(MINUS_ONE, -1.0);
        e.set_global::<u32>(LOWEST_FLOAT, 0xff7f_ffff);
        for word in 0..3u32 {
            e.set_global::<u32>(INVALID_POINT3 + 4 * word, 0x7f7f_ffff);
        }
        e.set_global::<f32>(QUATERNION_IDENTITY, 1.0);
        // Gauss abscissas 0.25, 0.5, 0.5, 0.75, 1.0 and weights summing exactly to 1.
        for (i, (x, w)) in [
            (0.25f32, 0.125f32),
            (0.5, 0.25),
            (0.5, 0.25),
            (0.75, 0.25),
            (1.0, 0.125),
        ]
        .into_iter()
        .enumerate()
        {
            e.set_global::<f32>(GAUSS_ABSCISSAS + 4 * i as u32, x);
            e.set_global::<f32>(GAUSS_WEIGHTS + 4 * i as u32, w);
        }
        e
    }

    /// A key-data object (`NiPosData`/`NiFloatData` shape) with `times.len()` keys of
    /// `size` bytes, each key's first float its time.
    fn key_data(e: &mut Engine, key_type: u32, size: u8, times: &[f32]) -> u32 {
        let data = e.mem.alloc(0x18);
        let keys = e.mem.alloc((times.len() as u32 * u32::from(size)).max(4));
        for (i, t) in times.iter().enumerate() {
            e.mem.set_f32(keys + i as u32 * u32::from(size), *t);
        }
        e.mem.set_u32(data + KEY_DATA_NUM_KEYS, times.len() as u32);
        e.mem.set_u32(data + KEY_DATA_KEYS, keys);
        e.mem.set_u32(data + KEY_DATA_TYPE, key_type);
        e.mem.set_u8(data + KEY_DATA_KEY_SIZE, size);
        data
    }

    /// Key type 0's derivative writes `(speed, 0, 0)`: the path's speed is `speed` everywhere.
    fn constant_speed(e: &mut Engine, speed: f32) {
        e.mem.set_u32(POS_KEY_DERIVATIVES, 0x0090_0200);
        e.register_double(0x0090_0200, move |e, a| {
            e.mem.set_f32(a[3], speed);
            e.mem.set_f32(a[3] + 4, 0.0);
            e.mem.set_f32(a[3] + 8, 0.0);
            Ret::default()
        });
    }

    fn interpolator(e: &mut Engine) -> Ptr<NiPathInterpolator> {
        e.new_object::<NiPathInterpolator>()
    }

    #[test]
    fn destructor_body_releases_the_data_and_frees_the_lengths() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        e.register(0x0090_0004, |_, _| Ret::default());
        e.put_vtable(0x0090_1000, &[0, 0x0090_0004]);
        let this = interpolator(&mut e);
        let path = e.mem.alloc(0x18);
        let percentage = e.mem.alloc(0x18);
        for (object, count) in [(path, 1u32), (percentage, 2)] {
            e.mem.set_u32(object, 0x0090_1000);
            e.mem.set_u32(object + 4, count);
        }
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.set(this, NiPathInterpolator::m_spPctData, percentage);
        e.set(this, NiPathInterpolator::m_pfPartialLength, 0x1234);
        e.call(0x00a3dd40, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_PATH_INTERPOLATOR_VTABLE);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPathData), 0);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPctData), 0);
        assert_eq!(calls_at(&e, 0x0090_0004), vec![vec![path]]);
        assert_eq!(e.mem.u32(percentage + 4), 1);
        assert_eq!(calls_at(&e, TABLE_FREE), vec![vec![0x1234]]);
        assert_eq!(
            calls_at(&e, KEY_BASED_INTERPOLATOR_DESTRUCTOR),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn point_interpolates_with_the_key_types_function() {
        let mut e = path_engine();
        e.map(POS_KEY_INTERPOLATIONS, 0x10);
        e.mem.set_u32(POS_KEY_INTERPOLATIONS + 8, 0x0090_0100);
        e.mem.set_u32(POS_KEY_INTERPOLATIONS, 0x0090_0108);
        e.register(0x0090_0100, |_, _| Ret::default());
        e.register(0x0090_0108, |_, _| Ret::default());
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 2, 0x10, &[0.0, 1.0, 2.0, 3.0]);
        let keys = e.mem.u32(data + KEY_DATA_KEYS);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        let point = 0x7000u32;
        assert_eq!(
            e.call(0x00a3ddf0, &args![this, point, 1u32, 3u32, 0.25f32])
                .u32(),
            point
        );
        assert_eq!(
            calls_at(&e, 0x0090_0100),
            vec![vec![0.25f32.to_bits(), keys + 0x10, keys + 0x30, point]]
        );
        // Without path data: key type, size and keys 0.
        e.set(this, NiPathInterpolator::m_spPathData, 0);
        e.call(0x00a3ddf0, &args![this, point, 1u32, 3u32, 0.5f32]);
        assert_eq!(
            calls_at(&e, 0x0090_0108),
            vec![vec![0.5f32.to_bits(), 0, 0, point]]
        );
    }

    #[test]
    fn process_clone_reaches_both_data_objects() {
        let mut e = path_engine();
        e.register(NI_OBJECT_PROCESS_CLONE, |_, _| Ret::default());
        e.register(0x0090_0068, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x1b];
        slots[0x1a] = 0x0090_0068;
        e.put_vtable(0x0090_1000, &slots);
        let this = interpolator(&mut e);
        let path = e.mem.alloc(0x18);
        e.mem.set_u32(path, 0x0090_1000);
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.call(0x00a3dc80, &args![this, 0x5555u32]);
        assert_eq!(
            calls_at(&e, NI_OBJECT_PROCESS_CLONE),
            vec![vec![this.addr(), 0x5555]]
        );
        assert_eq!(calls_at(&e, 0x0090_0068), vec![vec![path, 0x5555]]);
        let percentage = e.mem.alloc(0x18);
        e.mem.set_u32(percentage, 0x0090_1000);
        e.set(this, NiPathInterpolator::m_spPctData, percentage);
        e.call(0x00a3dc80, &args![this, 0x5555u32]);
        assert_eq!(calls_at(&e, 0x0090_0068).len(), 3);
    }

    #[test]
    fn speed_is_the_length_of_the_derivative() {
        let mut e = path_engine();
        e.mem.set_u32(POS_KEY_DERIVATIVES, 0x0090_0200);
        e.register(0x0090_0200, |e, a| {
            e.mem.set_f32(a[3], 3.0);
            e.mem.set_f32(a[3] + 4, 4.0);
            e.mem.set_f32(a[3] + 8, 12.0);
            Ret::default()
        });
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 0, 8, &[0.0, 1.0]);
        let keys = e.mem.u32(data + KEY_DATA_KEYS);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        let speed = e.call(0x00a3e5f0, &args![this, 0u32, 1u32, 0.5f32]).f64();
        assert_eq!(speed, 13.0);
        let call = &calls_at(&e, 0x0090_0200)[0];
        assert_eq!(call[..3], [0.5f32.to_bits(), keys, keys + 8]);
        assert_eq!(calls_at(&e, CRT_SQRT), vec![args![169.0f64]]);
    }

    #[test]
    fn segment_length_integrates_the_speed() {
        let mut e = path_engine();
        constant_speed(&mut e, 3.0);
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 0, 8, &[0.0, 1.0]);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        let length = e.call(0x00a3e680, &args![this, 0u32, 1u32, 2.0f32]).f64();
        assert_eq!(length, 6.0);
        let ratios: Vec<u32> = calls_at(&e, 0x0090_0200).iter().map(|c| c[0]).collect();
        let expected: Vec<u32> = [0.5f32, 1.0, 1.0, 1.5, 2.0]
            .iter()
            .map(|r| r.to_bits())
            .collect();
        assert_eq!(ratios, expected);
    }

    #[test]
    fn total_length_is_rebuilt_only_while_negative() {
        let mut e = path_engine();
        constant_speed(&mut e, 1.0);
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 0, 8, &[0.0, 1.0, 2.0]);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        e.set(this, NiPathInterpolator::m_fTotalLength, 5.0f32.to_bits());
        assert_eq!(e.call(0x00a3e9a0, &args![this]).f64(), 5.0);
        assert!(calls_at(&e, TABLE_ALLOCATE).is_empty());
        e.set(
            this,
            NiPathInterpolator::m_fTotalLength,
            (-1.0f32).to_bits(),
        );
        e.set(this, NiPathInterpolator::m_pfPartialLength, 0x4444);
        assert_eq!(e.call(0x00a3e9a0, &args![this]).f64(), 2.0);
        assert_eq!(calls_at(&e, TABLE_FREE), vec![vec![0x4444]]);
        assert_eq!(calls_at(&e, TABLE_ALLOCATE), vec![vec![12]]);
        let lengths = e.get(this, NiPathInterpolator::m_pfPartialLength);
        assert_eq!(
            [0u32, 4, 8].map(|o| e.mem.f32(lengths + o)),
            [0.0, 1.0, 2.0]
        );
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fTotalLength),
            2.0f32.to_bits()
        );
    }

    /// Runs a segment search at `address` and answers (first, second, ratio).
    fn search(
        e: &mut Engine,
        address: u32,
        this: Ptr<NiPathInterpolator>,
        time: f32,
    ) -> (u32, u32, f32) {
        let out = e.mem.alloc(12);
        e.call(address, &args![this, time, out, out + 4, out + 8]);
        (e.mem.u32(out), e.mem.u32(out + 4), e.mem.f32(out + 8))
    }

    /// A constant-speed interpolator over three keys with partial lengths 0, 1, 2.
    fn arc_length_path(e: &mut Engine) -> Ptr<NiPathInterpolator> {
        let this = interpolator(e);
        let data = key_data(e, 0, 8, &[0.0, 1.0, 2.0]);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        let lengths = e.mem.alloc(12);
        for (i, l) in [0.0f32, 1.0, 2.0].into_iter().enumerate() {
            e.mem.set_f32(lengths + 4 * i as u32, l);
        }
        e.set(this, NiPathInterpolator::m_pfPartialLength, lengths);
        e.set(this, NiPathInterpolator::m_fTotalLength, 2.0f32.to_bits());
        this
    }

    #[test]
    fn arc_length_search_folds_the_time_and_refines_the_ratio() {
        let mut e = path_engine();
        constant_speed(&mut e, 1.0);
        let this = arc_length_path(&mut e);
        assert_eq!(search(&mut e, 0x00a3ea60, this, 0.0), (0, 1, 0.0));
        assert_eq!(search(&mut e, 0x00a3ea60, this, 1.0), (1, 2, 1.0));
        assert_eq!(search(&mut e, 0x00a3ea60, this, 0.25), (0, 1, 0.5));
        assert_eq!(search(&mut e, 0x00a3ea60, this, -0.75), (0, 1, 0.5));
        assert_eq!(search(&mut e, 0x00a3ea60, this, 1.25), (0, 1, 0.5));
        assert_eq!(search(&mut e, 0x00a3ea60, this, 0.75), (1, 2, 0.5));
        assert_eq!(calls_at(&e, CI_FMOD).len(), 2);
    }

    #[test]
    fn arc_length_search_takes_newton_steps_until_close() {
        let mut e = path_engine();
        // Speed 2 while the partial lengths say 1 per segment: the first guess 0.5 gives
        // length 1 against 0.5 into the segment; one step (0.5 / 2) gives 0.25.
        constant_speed(&mut e, 2.0);
        let this = arc_length_path(&mut e);
        assert_eq!(search(&mut e, 0x00a3ea60, this, 0.25), (0, 1, 0.25));
    }

    #[test]
    fn key_search_by_times() {
        let mut e = path_engine();
        let this = interpolator(&mut e);
        let data = key_data(&mut e, 0, 0x10, &[0.0, 1.0, 2.0, 3.0]);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        assert_eq!(search(&mut e, 0x00a3ec30, this, -1.0), (0, 1, 0.0));
        assert_eq!(search(&mut e, 0x00a3ec30, this, 3.0), (2, 3, 1.0));
        assert_eq!(search(&mut e, 0x00a3ec30, this, 1.5), (1, 2, 0.5));
        assert_eq!(e.get(this, NiPathInterpolator::m_uiLastPathIdx), 1);
        // From the last segment the search wraps to the start once.
        e.set(this, NiPathInterpolator::m_uiLastPathIdx, 2);
        assert_eq!(search(&mut e, 0x00a3ec30, this, 0.5), (0, 1, 0.5));
        assert_eq!(e.get(this, NiPathInterpolator::m_uiLastPathIdx), 0);
        // A key equal to the time.
        assert_eq!(search(&mut e, 0x00a3ec30, this, 2.0), (2, 3, 0.0));
        // Constant speed: the arc-length search, its first key kept.
        e.set(this, NiPathInterpolator::m_uFlags, 0x10);
        e.set(this, NiPathInterpolator::m_uiLastPathIdx, 7);
        assert_eq!(search(&mut e, 0x00a3ec30, this, 0.0), (0, 1, 0.0));
        assert_eq!(e.get(this, NiPathInterpolator::m_uiLastPathIdx), 0);
    }

    /// A stream whose input's read function fills each read with its 1-based call number.
    fn stream(e: &mut Engine) -> Ptr {
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0090_0300);
        let mut n = 0u8;
        e.register_double(0x0090_0300, move |e, a| {
            n += 1;
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, n);
            }
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        Ptr::new(stream)
    }

    #[test]
    fn load_binary_reads_the_members() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        let this = interpolator(&mut e);
        let stream = stream(&mut e);
        e.call(0x00a3ed90, &args![this, stream]);
        assert_eq!(e.get(this, NiPathInterpolator::m_uFlags), 0x0101);
        assert_eq!(e.get(this, NiPathInterpolator::m_eBankDir), 0x0202_0202);
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fMaxBankAngle),
            0x0303_0303
        );
        assert_eq!(e.get(this, NiPathInterpolator::m_fSmoothing), 0x0404_0404);
        assert_eq!(e.get(this, NiPathInterpolator::m_sFollowAxis), 0x0505);
        let input = e.mem.u32(stream.addr() + 0x24c);
        assert_eq!(e.mem.u32(input + 4), 16);
        let sizes: Vec<u32> = calls_at(&e, 0x0090_0300).iter().map(|c| c[2]).collect();
        assert_eq!(sizes, vec![2, 4, 4, 4, 2]);
        assert_eq!(
            calls_at(&e, STREAM_READ_LINK_ID),
            vec![vec![stream.addr()]; 2]
        );
        assert_eq!(
            calls_at(&e, KEY_BASED_INTERPOLATOR_LOAD_BINARY),
            vec![vec![this.addr(), stream.addr()]]
        );
    }

    #[test]
    fn link_object_links_the_data_and_its_curvature() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_LINK_OBJECT, |_, _| Ret::default());
        e.register(0x0090_0004, |_, _| Ret::default());
        e.put_vtable(0x0090_1000, &[0, 0x0090_0004]);
        let path = key_data(&mut e, 1, 8, &[0.0, 1.0]);
        let percentage = e.mem.alloc(0x18);
        let mut linked = vec![percentage, path];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| Ret {
            eax: linked.pop().unwrap_or(0),
            ..Ret::default()
        });
        e.map(POS_KEY_CURVATURES, 8);
        e.mem.set_u32(POS_KEY_CURVATURES + 4, 0x0090_0400);
        e.register(0x0090_0400, |_, _| Ret {
            st0: 0.75,
            ..Ret::default()
        });
        let this = interpolator(&mut e);
        let old = e.mem.alloc(0x18);
        e.mem.set_u32(old, 0x0090_1000);
        e.mem.set_u32(old + 4, 1);
        e.set(this, NiPathInterpolator::m_spPathData, old);
        e.set(this, NiPathInterpolator::m_uFlags, 0x10);
        e.set(this, NiPathInterpolator::m_fTotalLength, 5.0f32.to_bits());
        e.call(0x00a3ee80, &args![this, 0x6000u32]);
        assert_eq!(calls_at(&e, 0x0090_0004), vec![vec![old]]);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPathData), path);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPctData), percentage);
        assert_eq!(e.mem.u32(path + 4), 1);
        assert_eq!(e.mem.u32(percentage + 4), 1);
        assert_eq!(e.get(this, NiPathInterpolator::m_uFlags), 0x10);
        let keys = e.mem.u32(path + KEY_DATA_KEYS);
        assert_eq!(calls_at(&e, 0x0090_0400), vec![vec![keys, 2]]);
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fMaxCurvature),
            0.75f32.to_bits()
        );
        // Without constant speed the stale flag stays set.
        let other = interpolator(&mut e);
        e.call(0x00a3ee80, &args![other, 0x6000u32]);
        assert_eq!(e.get(other, NiPathInterpolator::m_uFlags), 1);
        assert_eq!(calls_at(&e, 0x0090_0400).len(), 1);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        let this = interpolator(&mut e);
        let path = e.mem.alloc(0x18);
        assert_eq!(
            e.call(0x00a3f060, &args![this, path, 0u32]).u32(),
            this.addr()
        );
        assert_eq!(e.mem.u32(this.addr()), NI_PATH_INTERPOLATOR_VTABLE);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPathData), path);
        assert_eq!(e.mem.u32(path + 4), 1);
        assert_eq!(e.get(this, NiPathInterpolator::m_spPctData), 0);
        assert_eq!(e.mem.f32(this.addr() + 0x3c), 1.0);
        assert_eq!(e.mem.u32(this.addr() + 0x50), 0x7f7f_ffff);
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fTotalLength),
            (-1.0f32).to_bits()
        );
        assert_eq!(e.get(this, NiPathInterpolator::m_fLastTime), 0xff7f_ffff);
        assert_eq!(e.get(this, NiPathInterpolator::m_eBankDir), 1);
        assert_eq!(e.get(this, NiPathInterpolator::m_uFlags), 3);
        assert_eq!(calls_at(&e, INTERLOCKED_INCREMENT).len(), 1);
    }

    #[test]
    fn update_uses_the_kept_point_at_the_last_time() {
        let mut e = path_engine();
        let this = interpolator(&mut e);
        let transform = e.mem.alloc(0x20);
        e.set(
            this,
            NiPathInterpolator::m_fInterpolatorLastTime,
            2.0f32.to_bits(),
        );
        for word in 0..3u32 {
            e.mem.set_u32(this.addr() + 0x4c + 4 * word, 0x7f7f_ffff);
        }
        assert!(!e
            .call(0x00a3f120, &args![this, 2.0f32, 0u32, transform])
            .bool());
        for offset in [0u32, 0x10, 0x1c] {
            assert_eq!(e.mem.u32(transform + offset), 0xff7f_ffff);
        }
        e.mem.set_f32(this.addr() + 0x50, 4.0);
        e.mem.set_f32(this.addr() + 0x3c, 9.0);
        e.set(this, NiPathInterpolator::m_uFlags, 0x20);
        assert!(e
            .call(0x00a3f120, &args![this, 2.0f32, 0u32, transform])
            .bool());
        assert_eq!(e.mem.f32(transform + 4), 4.0);
        assert_eq!(e.mem.f32(transform + 0xc), 9.0);
    }

    #[test]
    fn update_interpolates_the_path_at_the_percentage() {
        let mut e = path_engine();
        e.register(FLOAT_KEY_GEN_INTERP, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        e.register(CALC_REF_FRAME, |_, _| Ret::default());
        e.register(QUATERNION_FROM_ROTATION, |_, _| Ret::default());
        e.map(POS_KEY_INTERPOLATIONS, 4);
        e.mem.set_u32(POS_KEY_INTERPOLATIONS, 0x0090_0100);
        e.register(0x0090_0100, |e, a| {
            for (i, v) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * i as u32, v);
            }
            Ret::default()
        });
        let this = interpolator(&mut e);
        let transform = e.mem.alloc(0x20);
        assert!(!e
            .call(0x00a3f120, &args![this, 1.0f32, 0u32, transform])
            .bool());
        let percentage = key_data(&mut e, 3, 8, &[0.0, 1.0]);
        let path = key_data(&mut e, 0, 0x10, &[0.0, 1.0]);
        e.set(this, NiPathInterpolator::m_spPctData, percentage);
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.set(this, NiPathInterpolator::m_uFlags, 0x20);
        e.mem.set_f32(this.addr() + 0x48, 6.0);
        assert!(e
            .call(0x00a3f120, &args![this, 1.0f32, 0u32, transform])
            .bool());
        let pct_keys = e.mem.u32(percentage + KEY_DATA_KEYS);
        assert_eq!(
            calls_at(&e, FLOAT_KEY_GEN_INTERP),
            vec![vec![
                1.0f32.to_bits(),
                pct_keys,
                3,
                2,
                this.addr() + 0x14,
                8
            ]]
        );
        let frame = &calls_at(&e, CALC_REF_FRAME)[0];
        assert_eq!(frame[..4], [this.addr(), 0, 1, 0.5f32.to_bits()]);
        assert_eq!(
            calls_at(&e, QUATERNION_FROM_ROTATION)[0][0],
            this.addr() + 0x3c
        );
        assert_eq!(
            [0u32, 4, 8].map(|o| e.mem.f32(transform + o)),
            [7.0, 8.0, 9.0]
        );
        assert_eq!(e.mem.f32(this.addr() + 0x54), 9.0);
        assert_eq!(e.mem.f32(transform + 0x18), 6.0);
        assert_eq!(
            e.get(this, NiPathInterpolator::m_fLastTime),
            1.0f32.to_bits()
        );
    }

    #[test]
    fn copy_members_copies_flags_data_and_lengths() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        e.register(MEMCPY, |_, _| Ret::default());
        let clone = e.mem.alloc(0x18);
        e.register_double(0x0090_0048, move |_, _| Ret {
            eax: clone,
            ..Ret::default()
        });
        let mut slots = vec![0u32; 0x13];
        slots[0x12] = 0x0090_0048;
        e.put_vtable(0x0090_1000, &slots);
        let this = interpolator(&mut e);
        let destination = interpolator(&mut e);
        let path = key_data(&mut e, 0, 8, &[0.0, 1.0]);
        e.mem.set_u32(path, 0x0090_1000);
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.set(this, NiPathInterpolator::m_uFlags, 0x7e);
        e.set(this, NiPathInterpolator::m_eBankDir, 2);
        e.set(this, NiPathInterpolator::m_fSmoothing, 0.5f32.to_bits());
        e.set(this, NiPathInterpolator::m_sFollowAxis, 1);
        e.set(this, NiPathInterpolator::m_pfPartialLength, 0x4000);
        e.set(this, NiPathInterpolator::m_fTotalLength, 3.0f32.to_bits());
        e.call(0x00a3f2f0, &args![this, destination, 0x5555u32]);
        assert_eq!(
            calls_at(&e, KEY_BASED_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this.addr(), destination.addr(), 0x5555]]
        );
        assert_eq!(e.get(destination, NiPathInterpolator::m_uFlags), 0x7e);
        assert_eq!(e.get(destination, NiPathInterpolator::m_spPathData), clone);
        assert_eq!(e.mem.u32(clone + 4), 1);
        assert_eq!(e.get(destination, NiPathInterpolator::m_eBankDir), 2);
        assert_eq!(
            e.get(destination, NiPathInterpolator::m_fSmoothing),
            0.5f32.to_bits()
        );
        assert_eq!(e.get(destination, NiPathInterpolator::m_sFollowAxis), 1);
        let copy = e.get(destination, NiPathInterpolator::m_pfPartialLength);
        assert_eq!(calls_at(&e, MEMCPY), vec![vec![copy, 0x4000, 8]]);
        assert_eq!(
            e.get(destination, NiPathInterpolator::m_fTotalLength),
            3.0f32.to_bits()
        );
        // Cleared bits are cleared in the destination; bit 4 with a stale destination
        // recomputes its total length.
        e.set(this, NiPathInterpolator::m_uFlags, 0x10);
        e.set(this, NiPathInterpolator::m_pfPartialLength, 0);
        e.set(
            destination,
            NiPathInterpolator::m_fTotalLength,
            1.5f32.to_bits(),
        );
        e.call(0x00a3f2f0, &args![this, destination, 0x5555u32]);
        assert_eq!(e.get(destination, NiPathInterpolator::m_uFlags), 0x10);
        e.set(this, NiPathInterpolator::m_uFlags, 0x11);
        e.set(
            this,
            NiPathInterpolator::m_fTotalLength,
            (-1.0f32).to_bits(),
        );
        e.call(0x00a3f2f0, &args![this, destination, 0x5555u32]);
        // The recompute runs on the destination (its total 3 from the copy before, kept),
        // clears its bit 0, and the source's total is copied last.
        assert_eq!(e.get(destination, NiPathInterpolator::m_uFlags), 0x10);
        assert_eq!(
            e.get(destination, NiPathInterpolator::m_fTotalLength),
            (-1.0f32).to_bits()
        );
    }

    #[test]
    fn create_clone_and_factory_construct_a_new_interpolator() {
        let mut e = path_engine();
        e.register(KEY_BASED_INTERPOLATOR_CONSTRUCTOR, |_, _| Ret::default());
        e.register(KEY_BASED_INTERPOLATOR_COPY_MEMBERS, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let this = interpolator(&mut e);
        let clone = e.call(0x00a3f520, &args![this, 0x5555u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), NI_PATH_INTERPOLATOR_VTABLE);
        assert_eq!(calls_at(&e, OPERATOR_NEW), vec![vec![0x5c]]);
        assert_eq!(
            calls_at(&e, KEY_BASED_INTERPOLATOR_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x5555]]
        );
        let made = e.call(0x00a3f4b0, &args![]).u32();
        assert_eq!(e.mem.u32(made), NI_PATH_INTERPOLATOR_VTABLE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00a3f4b0, &args![]).u32(), 0);
    }
}

#[cfg(test)]
mod n005_tests {
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

    /// An object whose vtable sends every slot `0..0x100` to `0x0099_0000 + slot`.
    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        let table = e.mem.alloc(0x100);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(table + slot, 0x0099_0000 + slot);
        }
        e.mem.set_u32(object, table);
        object
    }

    /// Key data (`NiPosData`/`NiFloatData` layout) with the given count, type, keys and size.
    fn key_data(e: &mut Engine, count: u32, key_type: u32, keys: u32, size: u8) -> u32 {
        let data = object(e, 0x18);
        e.mem.set_u32(data + KEY_DATA_NUM_KEYS, count);
        e.mem.set_u32(data + KEY_DATA_TYPE, key_type);
        e.mem.set_u32(data + KEY_DATA_KEYS, keys);
        e.mem.set_u8(data + KEY_DATA_KEY_SIZE, size);
        data
    }

    fn interpolator(e: &mut Engine) -> Ptr<NiPathInterpolator> {
        Ptr::new(object(e, 0x5c))
    }

    #[test]
    fn key_content_by_channel() {
        let mut e = Engine::new();
        let this = 0x1000u32;
        assert_eq!(e.call(0x00a3_da80, &[this, 0]).u32(), 1);
        assert_eq!(e.call(0x00a3_da80, &[this, 0xffff_0001]).u32(), 0);
        assert_eq!(e.call(0x00a3_da80, &[this, 2]).u32(), 6);
    }

    #[test]
    fn key_count_type_array_and_stride_by_channel() {
        let mut e = Engine::new();
        let this = interpolator(&mut e);
        let path = key_data(&mut e, 5, 2, 0x4000, 0x10);
        let percentage = key_data(&mut e, 7, 3, 0x5000, 0x14);
        for (address, path_value, percentage_value) in [
            (0x00a3_dae0u32, 5u32, 7u32),
            (0x00a3_db10, 2, 3),
            (0x00a3_db40, 0x4000, 0x5000),
            (0x00a3_db70, 0x10, 0x14),
        ] {
            assert_eq!(e.call(address, &[this.addr(), 0]).u32(), 0);
            e.set(this, NiPathInterpolator::m_spPathData, path);
            e.set(this, NiPathInterpolator::m_spPctData, percentage);
            assert_eq!(e.call(address, &[this.addr(), 0]).u32(), path_value);
            assert_eq!(e.call(address, &[this.addr(), 1]).u32(), percentage_value);
            assert_eq!(e.call(address, &[this.addr(), 2]).u32(), 0);
            e.set(this, NiPathInterpolator::m_spPathData, 0);
            e.set(this, NiPathInterpolator::m_spPctData, 0);
        }
    }

    fn range_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0109_7000, 0x0101_5000, 0x0102_3000, 0x0102_4000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(HIGHEST_FLOAT, f32::MAX);
        e.mem.set_f32(LOWEST_FLOAT, f32::MIN);
        e.mem
            .set_u64(RANGE_UNSET_BEGIN, f64::from(f32::MAX).to_bits());
        e.mem
            .set_u64(RANGE_UNSET_END, f64::from(f32::MIN).to_bits());
        e
    }

    #[test]
    fn active_time_range_takes_the_first_and_last_percentage_keys() {
        let mut e = range_engine();
        let first = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        e.mem.set_f32(first, 0.5);
        e.mem.set_f32(last, 4.0);
        e.register(0x0099_00e0, |_, _| 3u32.into_ret());
        e.register_double(KEY_BASED_INTERPOLATOR_GET_KEY_AT, move |_, a| {
            assert_eq!(a[2], 1);
            if a[1] == 0 { first } else { last }.into_ret()
        });
        let this = interpolator(&mut e);
        let (begin, end) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        e.call(0x00a3_dba0, &[this.addr(), begin, end]);
        assert_eq!(e.mem.f32(begin), 0.5);
        assert_eq!(e.mem.f32(end), 4.0);
        let indices: Vec<u32> = calls_to(&e, KEY_BASED_INTERPOLATOR_GET_KEY_AT)
            .iter()
            .map(|a| a[1])
            .collect();
        assert_eq!(indices, [0, 2]);
    }

    #[test]
    fn active_time_range_without_keys_is_zero() {
        let mut e = range_engine();
        e.register(0x0099_00e0, |_, _| 0u32.into_ret());
        let this = interpolator(&mut e);
        let (begin, end) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call(0x00a3_dba0, &[this.addr(), begin, end]);
        assert_eq!(e.mem.u32(begin), 0);
        assert_eq!(e.mem.u32(end), 0);
        // Keys counted but one of them missing: also zero.
        e.register(0x0099_00e0, |_, _| 2u32.into_ret());
        e.register(KEY_BASED_INTERPOLATOR_GET_KEY_AT, |_, _| 0u32.into_ret());
        e.call(0x00a3_dba0, &[this.addr(), begin, end]);
        assert_eq!(e.mem.u32(begin), 0);
    }

    #[test]
    fn guarantee_time_range_forwards_to_the_percentage_data() {
        let mut e = Engine::new();
        e.register(FLOAT_DATA_GUARANTEE_KEYS, |_, _| Ret::default());
        let this = interpolator(&mut e);
        e.call_log = Some(vec![]);
        e.call(
            0x00a3_dc50,
            &[this.addr(), 1.0f32.to_bits(), 3.0f32.to_bits()],
        );
        assert!(calls_to(&e, FLOAT_DATA_GUARANTEE_KEYS).is_empty());
        e.set(this, NiPathInterpolator::m_spPctData, 0x6000);
        e.call(0x00a3_dc50, &[this.addr(), 1.0f32.to_bits(), 0xff80_0001]);
        assert_eq!(
            calls_to(&e, FLOAT_DATA_GUARANTEE_KEYS),
            vec![vec![0x6000, 1.0f32.to_bits(), 0xffc0_0001]]
        );
    }

    #[test]
    fn register_streamables_registers_both_data() {
        let mut e = Engine::new();
        e.register(KEY_BASED_INTERPOLATOR_REGISTER_STREAMABLES, |_, a| {
            (a[1] == 0x7000).into_ret()
        });
        e.register(0x0099_0054, |_, _| Ret::default());
        let this = interpolator(&mut e);
        let path = key_data(&mut e, 0, 0, 0, 0);
        let percentage = key_data(&mut e, 0, 0, 0, 0);
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a3_dcc0, &[this.addr(), 0x7001]).bool());
        assert!(calls_to(&e, 0x0099_0054).is_empty());
        assert!(e.call(0x00a3_dcc0, &[this.addr(), 0x7000]).bool());
        assert_eq!(calls_to(&e, 0x0099_0054), vec![vec![path, 0x7000]]);
        e.set(this, NiPathInterpolator::m_spPctData, percentage);
        e.call_log = Some(vec![]);
        e.call(0x00a3_dcc0, &[this.addr(), 0x7000]);
        assert_eq!(
            calls_to(&e, 0x0099_0054),
            vec![vec![path, 0x7000], vec![percentage, 0x7000]]
        );
    }

    #[test]
    fn path_keys_00a3dd00_writes_count_type_and_size() {
        let mut e = Engine::new();
        let this = interpolator(&mut e);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 8, 0xffff_ffff);
        assert_eq!(
            e.call(0x00a3_dd00, &[this.addr(), out, out + 4, out + 8])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u32(out + 4), 0);
        assert_eq!(e.mem.u32(out + 8), 0xffff_ff00);
        let path = key_data(&mut e, 9, 4, 0x8000, 0x1c);
        e.set(this, NiPathInterpolator::m_spPathData, path);
        assert_eq!(
            e.call(0x00a3_dd00, &[this.addr(), out, out + 4, out + 8])
                .u32(),
            0x8000
        );
        assert_eq!(e.mem.u32(out), 9);
        assert_eq!(e.mem.u32(out + 4), 4);
        assert_eq!(e.mem.u32(out + 8), 0xffff_ff1c);
    }

    fn counting(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x0099_0004, |_, _| Ret::default());
    }

    #[test]
    fn sequence_interpolator_takes_the_sequence_percentage_data() {
        let mut e = Engine::new();
        counting(&mut e);
        let sequence = interpolator(&mut e);
        let old = key_data(&mut e, 0, 0, 0, 0);
        e.mem.set_u32(old + 4, 1);
        e.set(sequence, NiPathInterpolator::m_spPctData, old);
        e.set(sequence, NiPathInterpolator::m_uiLastPctIdx, 5);
        let new = key_data(&mut e, 0, 0, 0, 0);
        e.register_double(INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR, move |_, _| {
            sequence.into_ret()
        });
        // The data's GetSequenceData hands out a counted reference.
        e.register_double(FLOAT_DATA_GET_SEQUENCE_DATA, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 0);
            e.mem.set_u32(a[3], new);
            e.mem.set_u32(new + 4, 1);
            Ret::default()
        });
        let this = interpolator(&mut e);
        e.call_log = Some(vec![]);
        // Without percentage data: only the base.
        assert_eq!(
            e.call(
                0x00a3_e6f0,
                &[this.addr(), 1.0f32.to_bits(), 2.0f32.to_bits()]
            )
            .u32(),
            sequence.addr()
        );
        assert!(calls_to(&e, FLOAT_DATA_GET_SEQUENCE_DATA).is_empty());
        e.set(this, NiPathInterpolator::m_spPctData, 0x6000);
        e.call(
            0x00a3_e6f0,
            &[this.addr(), 1.0f32.to_bits(), 2.0f32.to_bits()],
        );
        assert_eq!(
            calls_to(&e, FLOAT_DATA_GET_SEQUENCE_DATA)[0][..3],
            [0x6000, 1.0f32.to_bits(), 2.0f32.to_bits()]
        );
        assert_eq!(e.get(sequence, NiPathInterpolator::m_spPctData), new);
        assert_eq!(e.get(sequence, NiPathInterpolator::m_uiLastPctIdx), 0);
        // The old data was released (deleted), the new one is held once.
        assert_eq!(calls_to(&e, 0x0099_0004), vec![vec![old]]);
        assert_eq!(e.mem.u32(new + 4), 1);
    }

    /// Two equal interpolators with path and percentage data and partial lengths.
    fn equal_pair(e: &mut Engine) -> (Ptr<NiPathInterpolator>, Ptr<NiPathInterpolator>) {
        let mut pair = Vec::new();
        for _ in 0..2 {
            let this = interpolator(e);
            let path = key_data(e, 2, 1, 0, 0);
            let percentage = key_data(e, 2, 1, 0, 0);
            e.set(this, NiPathInterpolator::m_spPathData, path);
            e.set(this, NiPathInterpolator::m_spPctData, percentage);
            let lengths = e.mem.alloc(8);
            e.mem.set_f32(lengths + 4, 3.0);
            e.set(this, NiPathInterpolator::m_pfPartialLength, lengths);
            e.set(this, NiPathInterpolator::m_fTotalLength, 3.0f32.to_bits());
            e.set(this, NiPathInterpolator::m_uFlags, 0x35);
            e.set(this, NiPathInterpolator::m_eBankDir, 1);
            e.set(this, NiPathInterpolator::m_fMaxBankAngle, 0.5f32.to_bits());
            e.set(this, NiPathInterpolator::m_fSmoothing, 0.25f32.to_bits());
            e.set(this, NiPathInterpolator::m_sFollowAxis, 2);
            pair.push(this);
        }
        (pair[0], pair[1])
    }

    #[test]
    fn is_equal_compares_data_lengths_and_settings() {
        let mut e = Engine::new();
        e.register(INTERPOLATOR_IS_EQUAL, |_, _| true.into_ret());
        e.register(0x0099_005c, |_, _| true.into_ret());
        let (a, b) = equal_pair(&mut e);
        let equal = |e: &mut Engine| e.call(0x00a3_e7b0, &[a.addr(), b.addr()]).bool();
        assert!(equal(&mut e));
        // A partial length differs.
        let lengths = e.get(b, NiPathInterpolator::m_pfPartialLength);
        e.mem.set_f32(lengths + 4, 3.5);
        assert!(!equal(&mut e));
        e.mem.set_f32(lengths + 4, 3.0);
        // Only one has partial lengths.
        e.set(b, NiPathInterpolator::m_pfPartialLength, 0);
        assert!(!equal(&mut e));
        e.set(b, NiPathInterpolator::m_pfPartialLength, lengths);
        // A NaN total length is never equal.
        for this in [a, b] {
            e.set(this, NiPathInterpolator::m_fTotalLength, 0x7fc0_0000);
        }
        assert!(!equal(&mut e));
        for this in [a, b] {
            e.set(this, NiPathInterpolator::m_fTotalLength, 3.0f32.to_bits());
        }
        // Each compared flag bit, the bank direction, the floats and the axis.
        for bit in 0..7u16 {
            e.set(b, NiPathInterpolator::m_uFlags, 0x35 ^ (1 << bit));
            assert!(!equal(&mut e), "bit {bit}");
        }
        e.set(b, NiPathInterpolator::m_uFlags, 0x35 ^ 0x80);
        assert!(equal(&mut e));
        e.set(b, NiPathInterpolator::m_uFlags, 0x35);
        e.set(b, NiPathInterpolator::m_eBankDir, 0);
        assert!(!equal(&mut e));
        e.set(b, NiPathInterpolator::m_eBankDir, 1);
        e.set(b, NiPathInterpolator::m_fSmoothing, 0.0f32.to_bits());
        assert!(!equal(&mut e));
        e.set(b, NiPathInterpolator::m_fSmoothing, 0.25f32.to_bits());
        e.set(b, NiPathInterpolator::m_sFollowAxis, 1);
        assert!(!equal(&mut e));
        e.set(b, NiPathInterpolator::m_sFollowAxis, 2);
        // One side without percentage data.
        e.set(b, NiPathInterpolator::m_spPctData, 0);
        assert!(!equal(&mut e));
        // The data's own IsEqual says no.
        let (a, b) = equal_pair(&mut e);
        e.register(0x0099_005c, |_, _| false.into_ret());
        assert!(!e.call(0x00a3_e7b0, &[a.addr(), b.addr()]).bool());
        // The base says no.
        e.register(0x0099_005c, |_, _| true.into_ret());
        e.register(INTERPOLATOR_IS_EQUAL, |_, _| false.into_ret());
        assert!(!e.call(0x00a3_e7b0, &[a.addr(), b.addr()]).bool());
    }

    #[test]
    fn stale_lengths_00a3ea40_recompute_the_total() {
        let mut e = Engine::new();
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = interpolator(&mut e);
        let path = key_data(&mut e, 1, 0, 0, 0);
        e.set(this, NiPathInterpolator::m_spPathData, path);
        e.set(
            this,
            NiPathInterpolator::m_fTotalLength,
            (-1.0f32).to_bits(),
        );
        e.set(this, NiPathInterpolator::m_uFlags, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x00a3_ea40, &[this.addr()]);
        assert!(calls_to(&e, TABLE_ALLOCATE).is_empty());
        e.set(this, NiPathInterpolator::m_uFlags, 0x11);
        e.call(0x00a3_ea40, &[this.addr()]);
        assert_eq!(calls_to(&e, TABLE_ALLOCATE), vec![vec![4]]);
        assert_eq!(e.get(this, NiPathInterpolator::m_fTotalLength), 0);
        assert_eq!(e.get(this, NiPathInterpolator::m_uFlags), 0x10);
    }

    #[test]
    fn save_binary_writes_the_settings_and_the_data_links() {
        let mut e = Engine::new();
        e.register(KEY_BASED_INTERPOLATOR_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0099_0030, |_, _| Ret::default());
        e.register(0x0300_0000, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            a[2].into_ret()
        });
        let stream = object(&mut e, 0x300);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, 0x0300_0000);
        e.mem.set_u32(stream + 0x250, output);
        let this = interpolator(&mut e);
        e.set(this, NiPathInterpolator::m_spPathData, 0x1111);
        e.set(this, NiPathInterpolator::m_spPctData, 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x00a3_ef70, &[this.addr(), stream]);
        let t = this.addr();
        let writes: Vec<(u32, u32)> = calls_to(&e, 0x0300_0000)
            .iter()
            .map(|a| (a[1], a[2]))
            .collect();
        assert_eq!(
            writes,
            [
                (t + 0xc, 2),
                (t + 0x38, 4),
                (t + 0x28, 4),
                (t + 0x2c, 4),
                (t + 0x30, 2)
            ]
        );
        assert_eq!(e.mem.u32(output + 4), 16);
        assert_eq!(
            calls_to(&e, 0x0099_0030),
            vec![vec![stream, 0x1111], vec![stream, 0x2222]]
        );
        assert_eq!(
            calls_to(&e, KEY_BASED_INTERPOLATOR_SAVE_BINARY),
            vec![vec![t, stream]]
        );
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = Engine::new();
        e.map(NI_PATH_INTERPOLATOR_RTTI & !0xfff, 0x1000);
        e.set_global(NI_PATH_INTERPOLATOR_RTTI, 0x0444_0000u32);
        e.register(KEY_BASED_INTERPOLATOR_GET_VIEWER_STRINGS, |_, _| {
            Ret::default()
        });
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] | 1).into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0444_0001);
            Ret::default()
        });
        let this = interpolator(&mut e);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xe, 4);
        e.call_log = Some(vec![]);
        e.call(0x00a3_f4d0, &[this.addr(), strings]);
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0444_0000]]);
        assert_eq!(calls_to(&e, STRINGS_SET_SIZE), vec![vec![strings, 5]]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 1]);
    }
}

#[cfg(test)]
mod ref_frame_tests {
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

    fn set_point(e: &mut Engine, at: u32, point: [f32; 3]) {
        for (i, v) in point.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn matrix(e: &Engine, at: u32) -> Vec<f32> {
        (0..9).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// The tangent, second vector, third vector and curvature `00a277c0` gives, by call.
    type Frame = ([f32; 3], [f32; 3], [f32; 3], f32);

    /// An engine with the constants and doubles `calc_ref_frame` reaches: the frame
    /// function answers `frames` in order (the last one again after that), `UnitCross`
    /// gives (4, 5, 6), the point scale, sum and cross compute in `f32`, unitizing does
    /// nothing, the rotation about an axis writes its angle in every entry and the matrix
    /// product answers its right operand plus the left's first entry.
    fn frame_engine(frames: Vec<Frame>) -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.map(0x0109_7000, 0x1000);
        e.map(0x011f_3000, 0x1000);
        for i in 0..3 {
            e.mem.set_f32(REF_FRAME_DEFAULT_MATRIX + 16 * i, 1.0);
        }
        e.mem.set_f32(REF_FRAME_UP + 8, 1.0);
        e.mem.set_f32(FOUR_OVER_PI, 1.273_239_5);
        let mut frames = std::collections::VecDeque::from(frames);
        e.register_double(POS_KEY_FRAME, move |e, a| {
            let (t, n, b, c) = if frames.len() > 1 {
                frames.pop_front().unwrap()
            } else {
                frames[0]
            };
            set_point(e, a[4], t);
            set_point(e, a[5], n);
            set_point(e, a[6], b);
            e.mem.set_f32(a[7], c);
            Ret::default()
        });
        e.register(POINT_UNIT_CROSS, |e, a| {
            set_point(e, a[1], [4.0, 5.0, 6.0]);
            Ret::default()
        });
        e.register(POINT_SCALE, |e, a| {
            let factor = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[2] + 4 * i) * factor;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_UNITIZE, |_, _| Ret::default());
        e.register(POINT_CROSS, |e, a| {
            let p = |e: &Engine, at: u32, i: u32| e.mem.f32(at + 4 * i);
            let (l, r) = (a[0], a[2]);
            let x = p(e, l, 1) * p(e, r, 2) - p(e, l, 2) * p(e, r, 1);
            let y = p(e, l, 2) * p(e, r, 0) - p(e, l, 0) * p(e, r, 2);
            let z = p(e, l, 0) * p(e, r, 1) - p(e, l, 1) * p(e, r, 0);
            set_point(e, a[1], [x, y, z]);
            a[1].into_ret()
        });
        e.register(MATRIX_MAKE_ROTATION, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[0] + 4 * i, a[1]);
            }
            Ret::default()
        });
        e.register(MATRIX_MULTIPLY, |e, a| {
            let left = e.mem.f32(a[0]);
            for i in 0..9 {
                let v = e.mem.f32(a[2] + 4 * i) + left;
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(CI_ATAN, |_, a| {
            let v = f64::take(a, &mut 0);
            (v * 2.0).into_ret()
        });
        e
    }

    /// An interpolator with path data of `count` keys of 0x10 bytes (type 2, keys at
    /// 0x7000), the given flags and smoothing.
    fn path(e: &mut Engine, count: u32, flags: u16, smoothing: f32) -> Ptr<NiPathInterpolator> {
        let this = Ptr::<NiPathInterpolator>::new(e.mem.alloc(0x5c));
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data + 8, count);
        e.mem.set_u32(data + 0xc, 0x7000);
        e.mem.set_u32(data + 0x10, 2);
        e.mem.set_u8(data + 0x14, 0x10);
        e.set(this, NiPathInterpolator::m_spPathData, data);
        e.set(this, NiPathInterpolator::m_uFlags, flags);
        e.set(this, NiPathInterpolator::m_fSmoothing, smoothing.to_bits());
        this
    }

    fn run(
        e: &mut Engine,
        this: Ptr<NiPathInterpolator>,
        first: u32,
        second: u32,
        ratio: f32,
    ) -> u32 {
        let out = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(
            0x00a3_de50,
            &[this.addr(), first, second, ratio.to_bits(), out],
        );
        out
    }

    const PLAIN: Frame = ([1.0, 2.0, 3.0], [0.5, 0.25, 0.125], [7.0, 8.0, 9.0], 0.5);

    #[test]
    fn without_two_keys_the_frame_is_the_default() {
        let mut e = frame_engine(vec![PLAIN]);
        let this = path(&mut e, 1, 0, 0.0);
        let out = run(&mut e, this, 0, 1, 0.5);
        assert_eq!(
            matrix(&e, out),
            [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert!(calls_to(&e, POS_KEY_FRAME).is_empty());
        e.set(this, NiPathInterpolator::m_spPathData, 0);
        let out = run(&mut e, this, 0, 1, 0.5);
        assert_eq!(matrix(&e, out)[4], 1.0);
    }

    #[test]
    fn unbanked_frame_is_tangent_up_cross_and_their_cross() {
        let mut e = frame_engine(vec![PLAIN]);
        let this = path(&mut e, 3, 0, 0.0);
        let out = run(&mut e, this, 1, 2, 0.5);
        let frame = &calls_to(&e, POS_KEY_FRAME)[0];
        assert_eq!(frame[..4], [0.5f32.to_bits(), 0x7010, 0x7020, 2]);
        let cross = &calls_to(&e, POINT_UNIT_CROSS)[0];
        assert_eq!(cross[0], REF_FRAME_UP);
        // t = (1, 2, 3), c = (4, 5, 6), t x c = (-3, 6, -3), as columns.
        assert_eq!(
            matrix(&e, out),
            [1.0, 4.0, -3.0, 2.0, 5.0, 6.0, 3.0, 6.0, -3.0]
        );
    }

    #[test]
    fn frame_with_bit_2_and_no_smoothing_uses_the_negated_vectors() {
        let mut e = frame_engine(vec![PLAIN]);
        let this = path(&mut e, 3, 0x04, 0.0);
        let out = run(&mut e, this, 0, 1, 0.5);
        assert_eq!(calls_to(&e, POS_KEY_FRAME).len(), 1);
        assert_eq!(
            matrix(&e, out),
            [1.0, -0.5, -7.0, 2.0, -0.25, -8.0, 3.0, -0.125, -9.0]
        );
        // Flag bit 6 negates the first two columns.
        e.set(this, NiPathInterpolator::m_uFlags, 0x44);
        let out = run(&mut e, this, 0, 1, 0.5);
        assert_eq!(
            matrix(&e, out),
            [-1.0, 0.5, -7.0, -2.0, 0.25, -8.0, -3.0, 0.125, -9.0]
        );
    }

    #[test]
    fn follow_axes_swap_columns() {
        let mut e = frame_engine(vec![PLAIN]);
        let this = path(&mut e, 3, 0x04, 0.0);
        e.set(this, NiPathInterpolator::m_sFollowAxis, 1);
        let out = run(&mut e, this, 0, 1, 0.5);
        // (-col1, col0, col2).
        assert_eq!(
            matrix(&e, out),
            [0.5, 1.0, -7.0, 0.25, 2.0, -8.0, 0.125, 3.0, -9.0]
        );
        e.set(this, NiPathInterpolator::m_sFollowAxis, 2);
        let out = run(&mut e, this, 0, 1, 0.5);
        // (-col2, col1, col0).
        assert_eq!(
            matrix(&e, out),
            [7.0, -0.5, 1.0, 8.0, -0.25, 2.0, 9.0, -0.125, 3.0]
        );
        e.set(this, NiPathInterpolator::m_sFollowAxis, 3);
        let out = run(&mut e, this, 0, 1, 0.5);
        assert_eq!(matrix(&e, out)[0], 1.0);
    }

    #[test]
    fn banking_turns_the_frame_about_the_tangent() {
        let mut e = frame_engine(vec![PLAIN]);
        let this = path(&mut e, 3, 0x0c, 0.0);
        e.set(this, NiPathInterpolator::m_eBankDir, (-1i32) as u32);
        e.set(this, NiPathInterpolator::m_fMaxBankAngle, 2.0f32.to_bits());
        e.set(this, NiPathInterpolator::m_fMaxCurvature, 4.0f32.to_bits());
        let out = run(&mut e, this, 0, 1, 0.5);
        // The static scale is set on first use.
        assert_eq!(e.mem.u32(INTERPOLATOR_REF_FRAME.bank_scale_guard) & 1, 1);
        assert_eq!(e.mem.f32(INTERPOLATOR_REF_FRAME.bank_scale), 1.273_239_5);
        // Curvature 0.5 below 4: atan(0.125) (the double: 0.25) * 4 / pi * -1 * 2.
        let atan = x87_store32(0.25);
        let angle = x87_mul(x87_load32(atan), x87_m32(1.273_239_5f32.to_bits()));
        let angle = x87_store32(x87_mul(angle, x87_m32((-2.0f32).to_bits())));
        assert_eq!(calls_to(&e, CI_ATAN)[0], args![0.125f64]);
        let rotation = &calls_to(&e, MATRIX_MAKE_ROTATION)[0];
        assert_eq!(
            rotation[1..],
            [angle, 1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        // The product: the frame plus the rotation's first entry.
        let turn = f32::from_bits(angle);
        let expected: Vec<f32> = [1.0f32, -0.5, -7.0, 2.0, -0.25, -8.0, 3.0, -0.125, -9.0]
            .iter()
            .map(|v| v + turn)
            .collect();
        assert_eq!(matrix(&e, out), expected);
        // At or above the maximum curvature the full bank angle.
        e.set(this, NiPathInterpolator::m_fMaxCurvature, 0.5f32.to_bits());
        run(&mut e, this, 0, 1, 0.5);
        assert!(calls_to(&e, CI_ATAN).is_empty());
        assert_eq!(
            calls_to(&e, MATRIX_MAKE_ROTATION)[0][1],
            (-2.0f32).to_bits()
        );
    }

    #[test]
    fn smoothing_blends_with_the_next_segment_near_the_end() {
        let near: Frame = ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 2.0);
        let next: Frame = ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], 4.0);
        let mut e = frame_engine(vec![PLAIN, near, next]);
        let this = path(&mut e, 3, 0x04, 0.25);
        let out = run(&mut e, this, 0, 1, 0.875);
        let frames = calls_to(&e, POS_KEY_FRAME);
        assert_eq!(frames.len(), 3);
        // This segment at 1 - s, then the next one (keys 1 and 2) at s.
        assert_eq!(frames[1][..3], [0.75f32.to_bits(), 0x7000, 0x7010]);
        assert_eq!(frames[2][..3], [0.25f32.to_bits(), 0x7010, 0x7020]);
        // Weights (1 - 0.875 + 0.25) / 0.5 = 0.75 for this segment, 0.25 for the next.
        let scales = calls_to(&e, POINT_SCALE);
        assert_eq!(scales[0][1], 0.25f32.to_bits());
        assert_eq!(scales[1][1], 0.75f32.to_bits());
        // n = 0.75 (0, 1, 0) + 0.25 (0, 0, 1); b = t x n; curvature 4 * 0.25 + 0.75 * 2.
        let t = [1.0f32, 2.0, 3.0];
        let n = [0.0f32, 0.75, 0.25];
        let b = [
            t[1] * n[2] - t[2] * n[1],
            t[2] * n[0] - t[0] * n[2],
            t[0] * n[1] - t[1] * n[0],
        ];
        assert_eq!(
            matrix(&e, out),
            [t[0], -n[0], -b[0], t[1], -n[1], -b[1], t[2], -n[2], -b[2]]
        );
        assert_eq!(calls_to(&e, POINT_UNITIZE).len(), 1);
        // The last segment of an open path (bit 1) has no next one.
        let mut e2 = frame_engine(vec![PLAIN]);
        let open = path(&mut e2, 3, 0x06, 0.25);
        run(&mut e2, open, 1, 2, 0.875);
        assert_eq!(calls_to(&e2, POS_KEY_FRAME).len(), 1);
        // On a closed path the last segment blends with the first (index 0).
        let mut e3 = frame_engine(vec![PLAIN]);
        let closed = path(&mut e3, 3, 0x04, 0.25);
        run(&mut e3, closed, 1, 2, 0.875);
        let frames = calls_to(&e3, POS_KEY_FRAME);
        assert_eq!(frames[2][1..3], [0x7000, 0x7010]);
    }

    #[test]
    fn smoothing_blends_with_the_previous_segment_near_the_start() {
        let mut e = frame_engine(vec![PLAIN]);
        let this = path(&mut e, 3, 0x04, 0.25);
        let out = run(&mut e, this, 1, 2, 0.125);
        let frames = calls_to(&e, POS_KEY_FRAME);
        assert_eq!(frames.len(), 3);
        // This segment at s, then the previous one (keys 0 and 1) at 1 - s.
        assert_eq!(frames[1][..3], [0.25f32.to_bits(), 0x7010, 0x7020]);
        assert_eq!(frames[2][..3], [0.75f32.to_bits(), 0x7000, 0x7010]);
        // Weights (0.25 - 0.125) / 0.5 = 0.25 for the previous, 0.75 for this one.
        let scales = calls_to(&e, POINT_SCALE);
        assert_eq!(scales[0][1], 0.25f32.to_bits());
        assert_eq!(scales[1][1], 0.75f32.to_bits());
        assert_eq!(matrix(&e, out)[0], 1.0);
        // The first segment of an open path has no previous one; on a closed path it
        // blends with the last (keys 1 and 2).
        let mut e2 = frame_engine(vec![PLAIN]);
        let open = path(&mut e2, 3, 0x06, 0.25);
        run(&mut e2, open, 0, 1, 0.125);
        assert_eq!(calls_to(&e2, POS_KEY_FRAME).len(), 1);
        let mut e3 = frame_engine(vec![PLAIN]);
        let closed = path(&mut e3, 3, 0x04, 0.25);
        run(&mut e3, closed, 0, 1, 0.125);
        assert_eq!(calls_to(&e3, POS_KEY_FRAME)[2][1..3], [0x7010, 0x7020]);
        // In the middle of a segment nothing is blended.
        let mut e4 = frame_engine(vec![PLAIN]);
        let middle = path(&mut e4, 3, 0x04, 0.25);
        run(&mut e4, middle, 0, 1, 0.5);
        assert_eq!(calls_to(&e4, POS_KEY_FRAME).len(), 1);
    }
}
