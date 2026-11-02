//! `NiAnimation/nipathcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPathController::ms_RTTI` (the class's `NiRTTI` record).
const NI_PATH_CONTROLLER_RTTI: u32 = 0x011f_3f80;

// Translated from 00a49e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_path_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PATH_CONTROLLER_RTTI)
}

// Translated from 00a4a8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a49d60`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x64 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_path_controller_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_9d60;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x64u32]);
    }
    this
}

layout! {
    /// `NiPathController` (Xbox PDB), 0x64 bytes on PC (the same).
    pub struct NiPathController: 0x64 {
        /// `NiTimeController::m_uFlags` (Xbox PDB).
        0x08 m_uTimeControllerFlags: u16,
        /// `NiTimeController::m_fScaledTime` (Xbox PDB), float bits.
        0x28 m_fScaledTime: u32,
        /// `NiTimeController::m_pkTarget` (Xbox PDB).
        0x2C m_pkTarget: u32,
        /// `m_uFlags` (Xbox PDB), as this unit uses the bits: 0 the partial lengths are
        /// stale, 1 `CalcRefFrame` does not wrap around the path, 2 `CalcRefFrame` builds
        /// the frame from the key vectors, 3 `CalcRefFrame` banks, 4 constant speed (the
        /// arc-length search), 5 `Update` stores the frame, 6 `CalcRefFrame` negates the
        /// first two columns.
        0x34 m_uFlags: u16,
        /// `m_uiLastPathIdx` (Xbox PDB).
        0x38 m_uiLastPathIdx: u32,
        /// `m_uiLastPctIdx` (Xbox PDB).
        0x3C m_uiLastPctIdx: u32,
        /// `m_spPathData` (Xbox PDB): `NiPosData*`.
        0x40 m_spPathData: u32,
        /// `m_spPctData` (Xbox PDB): `NiFloatData*`.
        0x44 m_spPctData: u32,
        /// `m_pfPartialLength` (Xbox PDB): one float per key.
        0x48 m_pfPartialLength: u32,
        /// `m_fTotalLength` (Xbox PDB), float bits.
        0x4C m_fTotalLength: u32,
        /// `m_fMaxBankAngle` (Xbox PDB), float bits.
        0x50 m_fMaxBankAngle: u32,
        /// `m_fSmoothing` (Xbox PDB), float bits.
        0x54 m_fSmoothing: u32,
        /// `m_sFollowAxis` (Xbox PDB).
        0x58 m_sFollowAxis: u16,
        /// `m_fMaxCurvature` (Xbox PDB), float bits.
        0x5C m_fMaxCurvature: u32,
        /// `m_eBankDir` (Xbox PDB).
        0x60 m_eBankDir: u32,
    }
}

/// `NiPathController`'s vtable.
const NI_PATH_CONTROLLER_VTABLE: u32 = 0x0109_893c;
/// `NiTimeController`'s constructor (`00a6d220`), destructor body (`00a6d290`),
/// `ProcessClone` (`00a6d0c0`), `RegisterStreamables` (`00a6d0f0`; a bool), `IsEqual`
/// (`00a6cc80`; a bool), `LoadBinary` (`00a6d5e0`), `LinkObject` (`00a6d500`), `SaveBinary`
/// (`00a6d730`), member copy (`00a6cfe0`, (clone, cloning)), `GetViewerStrings`
/// (`00a6d830`) and `DontDoUpdate` (`00a36250`, the time; a bool): all `thiscall`.
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const TIME_CONTROLLER_REGISTER_STREAMABLES: u32 = 0x00a6_d0f0;
const TIME_CONTROLLER_IS_EQUAL: u32 = 0x00a6_cc80;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;
const TIME_CONTROLLER_SAVE_BINARY: u32 = 0x00a6_d730;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
const TIME_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a6_d830;
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// `NiStream`'s link-ID read (`00a645b0`) and `GetObjectFromLinkID` (`00a63430`), both
/// `thiscall` on the stream without arguments.
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiFloatKey::GenInterp` (`00a26b40`, `cdecl`, (time, keys, key type, key count, &last
/// index, key size); the value in ST0; it reads the low byte of the key size word).
const FLOAT_KEY_GEN_INTERP: u32 = 0x00a2_6b40;
/// `NiPosKey`'s per-key-type function tables: the interpolation (`011f3ca8`, `cdecl` (ratio,
/// key, next key, &point)), the derivative (`011f3c18`, the same) and the curvature
/// (`011f3d38`, `cdecl` (keys, count), the value in ST0).
const POS_KEY_INTERPOLATIONS: u32 = 0x011f_3ca8;
const POS_KEY_DERIVATIVES: u32 = 0x011f_3c18;
const POS_KEY_CURVATURES: u32 = 0x011f_3d38;
/// The five Gauss-Legendre abscissas (on [0, 1]) and weights of the arc length.
const GAUSS_ABSCISSAS: u32 = 0x011a_8b40;
const GAUSS_WEIGHTS: u32 = 0x011a_8b2c;
/// The `double` tolerance of the arc-length search (`01032980`, 0.0001) and `-1.0f`
/// (`01012054`).
const ARC_LENGTH_TOLERANCE: u32 = 0x0103_2980;
const MINUS_ONE: u32 = 0x0101_2054;
/// The table allocation (`00aa1070`) and free (`00aa10f0`), `operator new` (`00aa13e0`; all
/// `cdecl`, one word), `memcpy` (`00ec44d0`, `cdecl`), `_CIsqrt` (`00ec6040`, ST0) and
/// `_CIfmod` (`00ec913a`, the dividend and divisor passed as doubles).
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
const TABLE_FREE: u32 = 0x00aa_10f0;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const MEMCPY: u32 = 0x00ec_44d0;
const CRT_SQRT: u32 = 0x00ec_6040;
const CI_FMOD: u32 = 0x00ec_913a;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGetViewerString` (`cdecl`, the text) and the viewer-string array's `SetSize`
/// (`0096ad30`) and `SetAt` (`0096ae90`, (index, address of the string)).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;

/// `NiPosData` (Xbox PDB) fields read here: `m_uiNumKeys` +0x08, `m_pkKeys` +0x0C, `m_eType`
/// +0x10, `m_ucKeySize` +0x14 (`NiFloatData` has the same four).
const KEY_DATA_NUM_KEYS: u32 = 0x08;
const KEY_DATA_KEYS: u32 = 0x0C;
const KEY_DATA_TYPE: u32 = 0x10;
const KEY_DATA_KEY_SIZE: u32 = 0x14;

/// Releases a counted object (count at +4; `DeleteThis`, slot +4, at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The smart-pointer assignment of a counted object to the pointer at `slot`.
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

/// One transfer of `size` bytes through the stream's input (`NiStream::m_pkIstr` +0x24C,
/// `m_pfnRead` +0x8) or output (`m_pkOstr` +0x250, `m_pfnWrite` +0xC): the count word at
/// `count` set to `size`, the bytes moved added to the file's position.
fn transfer(
    e: &mut Engine,
    (file_field, function_field): (u32, u32),
    stream: u32,
    buffer: u32,
    (size, count): (u32, u32),
) {
    let file = e.mem.u32(stream.wrapping_add(file_field));
    let function = e.mem.u32(file.wrapping_add(function_field));
    e.mem.set_u32(count, size);
    let done = e
        .call(function, &args![file, buffer, size, count, 1u32])
        .u32();
    let position = e.mem.u32(file.wrapping_add(4));
    e.mem
        .set_u32(file.wrapping_add(4), position.wrapping_add(done));
}

const READ: (u32, u32) = (0x24c, 8);
const WRITE: (u32, u32) = (0x250, 0xc);

/// The path data's key type, key size and keys (all 0 without path data).
fn path_keys(e: &Engine, this: Ptr<NiPathController>) -> (u32, u32, u32) {
    let data = e.get(this, NiPathController::m_spPathData);
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
fn path_key_count(e: &Engine, this: Ptr<NiPathController>) -> u32 {
    let data = e.get(this, NiPathController::m_spPathData);
    if data == 0 {
        0
    } else {
        e.mem.u32(data.wrapping_add(KEY_DATA_NUM_KEYS))
    }
}

/// The low byte of the flags word (the exe tests its bits through a byte read).
fn flags_byte(e: &Engine, this: Ptr<NiPathController>) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x34))
}

// Translated from 00a49c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::ProcessClone` (Xbox PDB; `thiscall`, the cloning process):
/// `NiTimeController::ProcessClone`, then the path and percentage data's own (slot +0x68)
/// when present.
pub fn ni_path_controller_process_clone(e: &mut Engine, this: Ptr<NiPathController>, cloning: u32) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, cloning]);
    for field in [
        NiPathController::m_spPathData,
        NiPathController::m_spPctData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            e.vcall(object, 0x68, &args![cloning]);
        }
    }
}

// Translated from 00a49cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::RegisterStreamables` (Xbox PDB; `thiscall`, the stream): false when
/// `NiTimeController::RegisterStreamables` says so; otherwise the path and percentage data's
/// own (slot +0x54) when present, and true.
pub fn ni_path_controller_register_streamables(
    e: &mut Engine,
    this: Ptr<NiPathController>,
    stream: u32,
) -> bool {
    if !e
        .call(TIME_CONTROLLER_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    for field in [
        NiPathController::m_spPathData,
        NiPathController::m_spPctData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            e.vcall(object, 0x54, &args![stream]);
        }
    }
    true
}

// Translated from 00a49d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path data's keys (unnamed in the map; `thiscall`, (&keys, &count, &key type, &key
/// size), `RET 0x10`): the four written out, all 0 without path data.
pub fn fn_00a49d00(
    e: &mut Engine,
    this: Ptr<NiPathController>,
    keys: Ptr,
    count: Ptr,
    key_type: Ptr,
    size: Ptr,
) {
    let data = e.get(this, NiPathController::m_spPathData);
    if data == 0 {
        e.mem.set_u32(count.addr(), 0);
        e.mem.set_u32(key_type.addr(), 0);
        e.mem.set_u32(keys.addr(), 0);
        e.mem.set_u8(size.addr(), 0);
        return;
    }
    let value = e.mem.u32(data.wrapping_add(KEY_DATA_NUM_KEYS));
    e.mem.set_u32(count.addr(), value);
    let value = e.mem.u32(data.wrapping_add(KEY_DATA_TYPE));
    e.mem.set_u32(key_type.addr(), value);
    let value = e.mem.u8(data.wrapping_add(KEY_DATA_KEY_SIZE));
    e.mem.set_u8(size.addr(), value);
    let value = e.mem.u32(data.wrapping_add(KEY_DATA_KEYS));
    e.mem.set_u32(keys.addr(), value);
}

// Translated from 00a49d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController`'s destructor body (unnamed in the map; `thiscall`): the vtable, the
/// path and percentage data released and cleared, the partial lengths freed (`00aa10f0`),
/// the two pointers' own destructors (reading the cleared words), then the
/// `NiTimeController` destructor body (a tail jump; its `EAX` is answered).
pub fn fn_00a49d60(e: &mut Engine, this: Ptr<NiPathController>) -> u32 {
    e.mem.set_u32(this.addr(), NI_PATH_CONTROLLER_VTABLE);
    for field in [
        NiPathController::m_spPathData,
        NiPathController::m_spPctData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            release(e, object);
            e.set(this, field, 0);
        }
    }
    let lengths = e.get(this, NiPathController::m_pfPartialLength);
    e.call(TABLE_FREE, &args![lengths]);
    for field in [
        NiPathController::m_spPctData,
        NiPathController::m_spPathData,
    ] {
        let object = e.get(this, field);
        if object != 0 {
            release(e, object);
        }
    }
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]).u32()
}

// Translated from 00a49e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::GenPathInterp` (Xbox PDB; `thiscall`, (result, first, second, ratio),
/// `RET 0x10`): the path's point between the keys `first` and `second` at `ratio` (the key
/// type's interpolation, `011f3ca8`) into `result`; without path data the type, size and
/// keys are 0. Answers `result`.
pub fn ni_path_controller_gen_path_interp(
    e: &mut Engine,
    this: Ptr<NiPathController>,
    result: Ptr,
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
        &args![x87_quiet_bits(ratio.to_bits()), key, next_key, result],
    );
    result
}

// Translated from 00a4a610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's speed between the keys `first` and `second` at `ratio` (unnamed in the map;
/// `thiscall`, `RET 0xC`): the key type's derivative (`011f3c18`) into a local vector, whose
/// length (`x*x + y*y + z*z` rounded to a float, through `_CIsqrt`) is answered as a float.
pub fn fn_00a4a610(
    e: &mut Engine,
    this: Ptr<NiPathController>,
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

// Translated from 00a4a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's length from the key `first` towards `second` up to `ratio` (unnamed in the
/// map; `thiscall`, `RET 0xC`): five-point Gauss-Legendre quadrature of the speed
/// (`fn_00a4a610`) at the abscissas (`011a8b40`) scaled by `ratio`, weighted (`011a8b2c`)
/// and summed as a float, times `ratio`.
pub fn fn_00a4a6a0(
    e: &mut Engine,
    this: Ptr<NiPathController>,
    first: u32,
    second: u32,
    ratio: f32,
) -> f32 {
    let ratio_bits = ratio.to_bits();
    let mut sum = x87_store32(0.0);
    for offset in (0..0x14u32).step_by(4) {
        let abscissa = e.global::<u32>(GAUSS_ABSCISSAS + offset);
        let at = x87_store32(x87_mul(x87_load32(abscissa), x87_m32(ratio_bits)));
        let speed = fn_00a4a610(e, this, first, second, f32::from_bits(x87_quiet_bits(at)));
        let weight = e.global::<u32>(GAUSS_WEIGHTS + offset);
        let weighted = x87_mul(x87_load32(speed.to_bits()), x87_m32(weight));
        sum = x87_store32(x87_add(weighted, x87_m32(sum)));
    }
    let length = x87_store32(x87_mul(x87_load32(sum), x87_m32(ratio_bits)));
    f32::from_bits(x87_quiet_bits(length))
}

// Translated from 00a4a710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::IsEqual` (Xbox PDB; `thiscall`, the other object):
/// `NiTimeController::IsEqual`; the path and the percentage data equal by their own
/// `IsEqual` (slot +0x5C, run on this controller's data without a null check); the partial
/// lengths present in both or neither, and with both the first count * 4 bytes equal (the
/// count from `fn_00a49d00`, a word at a time and the rest byte by byte); then the total
/// length, flag bits 0 and 1, the bank direction, flag bits 2 to 5, the bank angle, the
/// smoothing, the follow axis and flag bit 6 (floats by `FUCOMPP`: a NaN is never equal).
pub fn ni_path_controller_is_equal(
    e: &mut Engine,
    this: Ptr<NiPathController>,
    other: Ptr<NiPathController>,
) -> bool {
    if !e.call(TIME_CONTROLLER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    for field in [
        NiPathController::m_spPathData,
        NiPathController::m_spPctData,
    ] {
        let own = e.get(this, field);
        let theirs = e.get(other, field);
        if !e.vcall(own, 0x5c, &args![theirs]).bool() {
            return false;
        }
    }
    let own = e.get(this, NiPathController::m_pfPartialLength);
    if (own == 0) != (e.get(other, NiPathController::m_pfPartialLength) == 0) {
        return false;
    }
    if own != 0 {
        // Locals: the count, the key type, the keys; the other-object argument's slot holds
        // the key size.
        let count = e.with_stack(0x10, |e, locals| {
            let base = locals.addr();
            fn_00a49d00(
                e,
                this,
                Ptr::new(base + 8),
                Ptr::new(base),
                Ptr::new(base + 4),
                Ptr::new(base + 0xc),
            );
            e.mem.u32(base)
        });
        let mut left = count.wrapping_mul(4);
        let mut mine = own;
        let mut theirs = e.get(other, NiPathController::m_pfPartialLength);
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
    let total = x87_load32(e.get(this, NiPathController::m_fTotalLength));
    if !(x87_load32(e.get(other, NiPathController::m_fTotalLength)) == total) {
        return false;
    }
    let theirs = flags_byte(e, other);
    let mine = flags_byte(e, this);
    let differs = |bit: u32| ((theirs >> bit) ^ (mine >> bit)) & 1 != 0;
    if differs(0) || differs(1) {
        return false;
    }
    if e.get(this, NiPathController::m_eBankDir) != e.get(other, NiPathController::m_eBankDir) {
        return false;
    }
    if differs(2) || differs(3) || differs(4) || differs(5) {
        return false;
    }
    for field in [
        NiPathController::m_fMaxBankAngle,
        NiPathController::m_fSmoothing,
    ] {
        let value = x87_load32(e.get(this, field));
        if !(x87_load32(e.get(other, field)) == value) {
            return false;
        }
    }
    if e.get(this, NiPathController::m_sFollowAxis) != e.get(other, NiPathController::m_sFollowAxis)
    {
        return false;
    }
    !differs(6)
}

// Translated from 00a4a8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path's total length (unnamed in the map; `thiscall`): while `m_fTotalLength` is below
/// 0 the partial lengths are rebuilt (the old table freed, one float per key allocated, the
/// first 0, each next one the previous plus the length of that segment from `fn_00a4a6a0`
/// with ratio 1) and the last becomes the total. Answers the total. With no keys the loop
/// runs to the count's wrap-around and the total is read from the word before the table,
/// as the exe does.
pub fn fn_00a4a8e0(e: &mut Engine, this: Ptr<NiPathController>) -> f32 {
    let total = e.get(this, NiPathController::m_fTotalLength);
    if 0.0 > x87_load32(total) {
        let count = path_key_count(e, this);
        let old = e.get(this, NiPathController::m_pfPartialLength);
        e.call(TABLE_FREE, &args![old]);
        let bytes = count.wrapping_mul(4);
        let lengths = e.call(TABLE_ALLOCATE, &args![bytes]).u32();
        e.set(this, NiPathController::m_pfPartialLength, lengths);
        e.mem.set_u32(lengths, x87_store32(0.0));
        let last = count.wrapping_sub(1);
        if last != 0 {
            let mut index = 0u32;
            loop {
                let next = index.wrapping_add(1);
                let segment = fn_00a4a6a0(e, this, index, next, 1.0);
                let segment = x87_store32(x87_load32(segment.to_bits()));
                let lengths = e.get(this, NiPathController::m_pfPartialLength);
                let at = lengths.wrapping_add(index.wrapping_mul(4));
                let sum = x87_add(x87_load32(e.mem.u32(at)), x87_m32(segment));
                e.mem.set_u32(at.wrapping_add(4), x87_store32(sum));
                index = next;
                if index >= last {
                    break;
                }
            }
        }
        let lengths = e.get(this, NiPathController::m_pfPartialLength);
        let last_length = e.mem.u32(lengths.wrapping_add(bytes).wrapping_sub(4));
        e.set(
            this,
            NiPathController::m_fTotalLength,
            x87_quiet_bits(last_length),
        );
    }
    let total = e.get(this, NiPathController::m_fTotalLength);
    f32::from_bits(x87_quiet_bits(total))
}

// Translated from 00a4a980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constant-speed segment search (unnamed in the map; `thiscall`, (time, &first,
/// &second, &ratio), `RET 0x10`): the keys and the ratio between them at which the path has
/// covered the fraction `time` of its total length. Time 0 answers (0, 1, 0); a negative
/// time is folded with `_CIfmod(time, 1) + 1`; time 1 answers (count - 2, count - 1, 1); a
/// time above 1 is folded with `_CIfmod(time, 1)`. The partial lengths give the segment
/// holding `time * total` and a linear first guess (0 and the segment past the end when
/// none holds it), refined by up to 32 Newton steps (segment length `fn_00a4a6a0` against
/// the distance into the segment, over the speed `fn_00a4a610`) until the error is within
/// 0.0001 (`01032980`).
pub fn fn_00a4a980(
    e: &mut Engine,
    this: Ptr<NiPathController>,
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
    let total = e.get(this, NiPathController::m_fTotalLength);
    let distance = x87_store32(x87_mul(current, x87_m32(total)));
    let mut into_segment = x87_store32(0.0);
    let mut ratio = into_segment;
    let mut first = 0u32;
    let mut second = 1u32;
    let last = count.wrapping_sub(1);
    if last != 0 {
        let lengths = e.get(this, NiPathController::m_pfPartialLength);
        loop {
            let next = e
                .mem
                .u32(lengths.wrapping_add(first.wrapping_mul(4)).wrapping_add(4));
            if x87_load32(next) >= x87_load32(distance) {
                let lengths = e.get(this, NiPathController::m_pfPartialLength);
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
        let length = fn_00a4a6a0(e, this, first, second, guess);
        let error = x87_store32(x87_sub(x87_load32(length.to_bits()), x87_m32(into_segment)));
        let error = x87_load32(error);
        let magnitude = x87_store32(error.abs());
        if x87_load32(magnitude) <= tolerance {
            break;
        }
        let speed = fn_00a4a610(e, this, first, second, guess);
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

// Translated from 00a4ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The path segment for `time` (unnamed in the map; `thiscall`, (time, &first, &second,
/// &ratio), `RET 0x10`): with constant speed (flag bit 4) the arc-length search
/// `fn_00a4a980`, its first key kept as `m_uiLastPathIdx`. Otherwise, by the key times
/// (each key's first float): at or before the first key (0, 1, 0); at or after the last
/// (count - 2, count - 1, 1); else from `m_uiLastPathIdx` on (wrapping to 0 once) the first
/// key equal to `time` or followed by a later key, kept as `m_uiLastPathIdx`, and the ratio
/// `(time - t[first]) / (t[second] - t[first])` (with `second` 0 when no key matched).
/// Without path data the keys are read at address 0, as the exe does.
pub fn fn_00a4ab50(
    e: &mut Engine,
    this: Ptr<NiPathController>,
    time: f32,
    first_out: Ptr,
    second_out: Ptr,
    ratio_out: Ptr,
) {
    if (flags_byte(e, this) >> 4) & 1 != 0 {
        fn_00a4a980(e, this, time, first_out, second_out, ratio_out);
        let first = e.mem.u32(first_out.addr());
        e.set(this, NiPathController::m_uiLastPathIdx, first);
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
    let mut first = e.get(this, NiPathController::m_uiLastPathIdx);
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
                e.set(this, NiPathController::m_uiLastPathIdx, first);
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

// Translated from 00a4acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::LoadBinary` (Xbox PDB; `thiscall`, the stream):
/// `NiTimeController::LoadBinary`; files older than version `0x0A000102` (`NiStream` +0xD8)
/// take the flags from the stream's `m_usNiTimeControllerFlags` (+0x286) shifted right by
/// 5, newer ones read them (2 bytes); then the bank direction (4 bytes, through a local),
/// the bank angle and smoothing (4 bytes each), the follow axis (2 bytes), and the link IDs
/// of the two data objects (`00a645b0`, twice). The count word is the stream argument's
/// own stack slot.
pub fn ni_path_controller_load_binary(e: &mut Engine, this: Ptr<NiPathController>, stream: u32) {
    let t = this.addr();
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    e.with_stack(8, |e, slots| {
        let local = slots.addr();
        let count = local + 4;
        if e.mem.u32(stream + 0xd8) < 0x0a00_0102 {
            let flags = e.mem.u16(stream + 0x286) >> 5;
            e.set(this, NiPathController::m_uFlags, flags);
        } else {
            transfer(e, READ, stream, t + 0x34, (2, count));
        }
        transfer(e, READ, stream, local, (4, count));
        let bank = e.mem.u32(local);
        e.set(this, NiPathController::m_eBankDir, bank);
        transfer(e, READ, stream, t + 0x50, (4, count));
        transfer(e, READ, stream, t + 0x54, (4, count));
        transfer(e, READ, stream, t + 0x58, (2, count));
    });
    e.call(STREAM_READ_LINK_ID, &args![stream]);
    e.call(STREAM_READ_LINK_ID, &args![stream]);
}

// Translated from 00a4adc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::LinkObject` (Xbox PDB; `thiscall`, the stream):
/// `NiTimeController::LinkObject`, the path and percentage data from the stream's next
/// linked objects (counted assignment), the "lengths stale" flag (bit 0) set; with constant
/// speed (bit 4) the total length is recomputed (`fn_00a4a8e0`) and bit 0 cleared. With path
/// data holding keys, the key type's function at `011f3d38` (`(keys, count)`) gives
/// `m_fMaxCurvature`.
pub fn ni_path_controller_link_object(e: &mut Engine, this: Ptr<NiPathController>, stream: u32) {
    let t = this.addr();
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let path = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_counted(e, t + 0x40, path);
    let percentage = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_counted(e, t + 0x44, percentage);
    let flags = e.get(this, NiPathController::m_uFlags) | 1;
    e.set(this, NiPathController::m_uFlags, flags);
    if (flags as u8 >> 4) & 1 != 0 && flags_byte(e, this) & 1 != 0 {
        let total = fn_00a4a8e0(e, this);
        e.set(
            this,
            NiPathController::m_fTotalLength,
            x87_store32(x87_load32(total.to_bits())),
        );
        let flags = e.get(this, NiPathController::m_uFlags) & 0xfffe;
        e.set(this, NiPathController::m_uFlags, flags);
    }
    let data = e.get(this, NiPathController::m_spPathData);
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
                NiPathController::m_fMaxCurvature,
                x87_store32(curvature),
            );
        }
    }
}

// Translated from 00a4aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::SaveBinary` (Xbox PDB; `thiscall`, the stream):
/// `NiTimeController::SaveBinary`, then the flags (2 bytes), the bank direction, the bank
/// angle and the smoothing (4 bytes each) and the follow axis (2 bytes), the count word
/// being the stream argument's own stack slot; then the stream's `SaveLinkID` (slot +0x30)
/// of the path and the percentage data.
pub fn ni_path_controller_save_binary(e: &mut Engine, this: Ptr<NiPathController>, stream: u32) {
    let t = this.addr();
    e.call(TIME_CONTROLLER_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        let count = count.addr();
        transfer(e, WRITE, stream, t + 0x34, (2, count));
        transfer(e, WRITE, stream, t + 0x60, (4, count));
        transfer(e, WRITE, stream, t + 0x50, (4, count));
        transfer(e, WRITE, stream, t + 0x54, (4, count));
        transfer(e, WRITE, stream, t + 0x58, (2, count));
    });
    for field in [
        NiPathController::m_spPathData,
        NiPathController::m_spPctData,
    ] {
        let object = e.get(this, field);
        e.vcall(stream, 0x30, &args![object]);
    }
}

// Translated from 00a4afa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::Update` (Xbox PDB; `thiscall`, the update data, whose first float is
/// the time): with percentage data holding keys and path data holding keys, and
/// `DontDoUpdate` saying to go on, the percentage at the scaled time
/// (`NiFloatKey::GenInterp` on the percentage keys, the last index at +0x3C, but the key
/// size passed is the path data's), its path segment (`fn_00a4ab50`, the ratio in the
/// update argument's own stack slot), with flag bit 5 the frame (`CalcRefFrame`) stored as
/// the target's rotation (+0x34, nine floats), and the point (`GenPathInterp`) as the
/// target's translation (+0x58).
pub fn ni_path_controller_update(e: &mut Engine, this: Ptr<NiPathController>, update: u32) {
    let t = this.addr();
    let percentage = e.get(this, NiPathController::m_spPctData);
    if percentage == 0 {
        return;
    }
    let key_type = e.mem.u32(percentage.wrapping_add(KEY_DATA_TYPE));
    let keys = e.mem.u32(percentage.wrapping_add(KEY_DATA_KEYS));
    let count = e.mem.u32(percentage.wrapping_add(KEY_DATA_NUM_KEYS));
    if keys == 0 {
        return;
    }
    let path = e.get(this, NiPathController::m_spPathData);
    if path == 0 {
        return;
    }
    let path_size = u32::from(e.mem.u8(path.wrapping_add(KEY_DATA_KEY_SIZE)));
    if e.mem.u32(path.wrapping_add(KEY_DATA_KEYS)) == 0 {
        return;
    }
    let time = x87_quiet_bits(e.mem.u32(update));
    if e.call(DONT_DO_UPDATE, &args![this, time]).bool() {
        return;
    }
    // Locals: the first key, the second key, the point, the matrix; then the update
    // argument's slot, which receives the ratio.
    e.with_stack(0x40, |e, locals| {
        let base = locals.addr();
        let first_slot = base;
        let second_slot = base + 4;
        let point = base + 8;
        let matrix = base + 0x14;
        let ratio_slot = base + 0x38;
        let scaled = x87_quiet_bits(e.get(this, NiPathController::m_fScaledTime));
        let value = e
            .call(
                FLOAT_KEY_GEN_INTERP,
                &args![scaled, keys, key_type, count, t + 0x3c, path_size],
            )
            .f64();
        e.mem.set_u32(ratio_slot, x87_store32(value));
        let percent = f32::from_bits(x87_quiet_bits(e.mem.u32(ratio_slot)));
        fn_00a4ab50(
            e,
            this,
            percent,
            Ptr::new(first_slot),
            Ptr::new(second_slot),
            Ptr::new(ratio_slot),
        );
        let second = e.mem.u32(second_slot);
        if (flags_byte(e, this) >> 5) & 1 != 0 {
            let ratio = f32::from_bits(x87_quiet_bits(e.mem.u32(ratio_slot)));
            let first = e.mem.u32(first_slot);
            ni_path_controller_calc_ref_frame(e, this, first, second, ratio, Ptr::new(matrix));
            let target = e.get(this, NiPathController::m_pkTarget);
            for word in 0..9u32 {
                let value = e.mem.u32(matrix + 4 * word);
                e.mem.set_u32(target + 0x34 + 4 * word, value);
            }
        }
        let ratio = f32::from_bits(x87_quiet_bits(e.mem.u32(ratio_slot)));
        let first = e.mem.u32(first_slot);
        ni_path_controller_gen_path_interp(e, this, Ptr::new(point), first, second, ratio);
        let target = e.get(this, NiPathController::m_pkTarget);
        for word in 0..3u32 {
            let value = e.mem.u32(point + 4 * word);
            e.mem.set_u32(target + 0x58 + 4 * word, value);
        }
    });
}

// Translated from 00a4b0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController`'s member copy (unnamed in the map; `thiscall`, (destination,
/// cloning), `RET 8`): `NiTimeController`'s copy, the flags word, clones (slot +0x48) of the
/// path and percentage data assigned as counted pointers, flag bits 2 and 3, the bank
/// direction, bit 4 (with it set and the destination's bit 0 set the total length is
/// recomputed and bit 0 cleared), bit 5, the bank angle and smoothing (through the x87
/// stack), the follow axis, bits 6 and 1, the partial lengths duplicated (`00aa1070` and
/// `memcpy`, one float per path key) when present, and the total length.
pub fn fn_00a4b0b0(
    e: &mut Engine,
    this: Ptr<NiPathController>,
    destination: Ptr<NiPathController>,
    cloning: u32,
) {
    e.call(
        TIME_CONTROLLER_COPY_MEMBERS,
        &args![this, destination, cloning],
    );
    let flags = e.get(this, NiPathController::m_uFlags);
    e.set(destination, NiPathController::m_uFlags, flags);
    for (offset, field) in [
        (0x40u32, NiPathController::m_spPathData),
        (0x44, NiPathController::m_spPctData),
    ] {
        let object = e.get(this, field);
        if object != 0 {
            let clone = e.vcall(object, 0x48, &args![cloning]).u32();
            assign_counted(e, destination.addr() + offset, clone);
        }
    }
    let copy_bit = |e: &mut Engine, bit: u32| {
        let source = flags_byte(e, this);
        let flags = e.get(destination, NiPathController::m_uFlags);
        let flags = if (source >> bit) & 1 != 0 {
            flags | (1u16 << bit)
        } else {
            flags & !(1u16 << bit)
        };
        e.set(destination, NiPathController::m_uFlags, flags);
    };
    copy_bit(e, 2);
    copy_bit(e, 3);
    let bank = e.get(this, NiPathController::m_eBankDir);
    e.set(destination, NiPathController::m_eBankDir, bank);
    if (flags_byte(e, this) >> 4) & 1 != 0 {
        let flags = e.get(destination, NiPathController::m_uFlags) | 0x10;
        e.set(destination, NiPathController::m_uFlags, flags);
        if flags_byte(e, destination) & 1 != 0 {
            let total = fn_00a4a8e0(e, destination);
            e.set(
                destination,
                NiPathController::m_fTotalLength,
                x87_store32(x87_load32(total.to_bits())),
            );
            let flags = e.get(destination, NiPathController::m_uFlags) & 0xfffe;
            e.set(destination, NiPathController::m_uFlags, flags);
        }
    } else {
        let flags = e.get(destination, NiPathController::m_uFlags) & 0xffef;
        e.set(destination, NiPathController::m_uFlags, flags);
    }
    copy_bit(e, 5);
    for field in [
        NiPathController::m_fMaxBankAngle,
        NiPathController::m_fSmoothing,
    ] {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(destination, field, value);
    }
    let axis = e.get(this, NiPathController::m_sFollowAxis);
    e.set(destination, NiPathController::m_sFollowAxis, axis);
    copy_bit(e, 6);
    copy_bit(e, 1);
    if e.get(this, NiPathController::m_pfPartialLength) != 0 {
        let bytes = path_key_count(e, this).wrapping_mul(4);
        let copy = e.call(TABLE_ALLOCATE, &args![bytes]).u32();
        e.set(destination, NiPathController::m_pfPartialLength, copy);
        let lengths = e.get(this, NiPathController::m_pfPartialLength);
        e.call(MEMCPY, &args![copy, lengths, bytes]);
    }
    let total = x87_quiet_bits(e.get(this, NiPathController::m_fTotalLength));
    e.set(destination, NiPathController::m_fTotalLength, total);
}

/// The members a new controller gets after the `NiTimeController` constructor (as
/// `CreateClone` and the creation function store them): the vtable, no data, bank angle,
/// smoothing and curvature 0, flags 3, total length -1 (`01012054`), indices 0, bank
/// direction 1, follow axis 0, no partial lengths.
fn init_path_controller(e: &mut Engine, object: u32) {
    let this = Ptr::<NiPathController>::new(object);
    e.call(TIME_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_PATH_CONTROLLER_VTABLE);
    e.set(this, NiPathController::m_spPathData, 0);
    e.set(this, NiPathController::m_spPctData, 0);
    let zero = x87_store32(0.0);
    e.set(this, NiPathController::m_fMaxBankAngle, zero);
    e.set(this, NiPathController::m_fSmoothing, zero);
    e.set(this, NiPathController::m_fMaxCurvature, zero);
    e.set(this, NiPathController::m_uFlags, 3);
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    e.set(this, NiPathController::m_fTotalLength, minus_one);
    e.set(this, NiPathController::m_uiLastPathIdx, 0);
    e.set(this, NiPathController::m_uiLastPctIdx, 0);
    e.set(this, NiPathController::m_eBankDir, 1);
    e.set(this, NiPathController::m_sFollowAxis, 0);
    e.set(this, NiPathController::m_pfPartialLength, 0);
}

// Translated from 00a4b270 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiPathController`): a 0x64-byte controller built as `CreateClone` builds
/// one, or null when the allocation fails.
pub fn fn_00a4b270(e: &mut Engine) -> Ptr<NiPathController> {
    let object = e.call(OPERATOR_NEW, &args![0x64u32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    init_path_controller(e, object);
    Ptr::new(object)
}

// Translated from 00a4b2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::GetViewerStrings` (Xbox PDB; `thiscall`, the viewer-string array):
/// `NiTimeController::GetViewerStrings`, then the class name (the first word of the
/// `NiRTTI` record at `011f3f80`) appended: the array grown by its step when full, then
/// `SetAt` at the old size with the address of the string's stack word.
pub fn ni_path_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: u32) {
    e.call(TIME_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(NI_PATH_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_SET_SIZE, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a4b330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::CreateClone` (Xbox PDB; `thiscall`, the cloning process): a new
/// 0x64-byte controller (`operator new`, the `NiTimeController` constructor and the members
/// a new one gets) given this one's members (`fn_00a4b0b0`, also run with a null clone when
/// the allocation fails). Answers the clone or null.
pub fn ni_path_controller_create_clone(
    e: &mut Engine,
    this: Ptr<NiPathController>,
    cloning: u32,
) -> Ptr<NiPathController> {
    let object = e.call(OPERATOR_NEW, &args![0x64u32]).u32();
    if object != 0 {
        init_path_controller(e, object);
    }
    fn_00a4b0b0(e, this, Ptr::new(object), cloning);
    Ptr::new(object)
}

use crate::units::nianimation::nipathinterpolator::{calc_ref_frame, RefFrameFields};

/// `NiPathController`'s offsets for the shared `CalcRefFrame` body: path data +0x40, flags
/// +0x34, bank angle +0x50, smoothing +0x54, follow axis +0x58, maximum curvature +0x5C,
/// bank direction +0x60; the bank scale `static` at `011f3f78`, guard `011f3f7c`.
const CONTROLLER_REF_FRAME: RefFrameFields = RefFrameFields {
    path_data: 0x40,
    flags: 0x34,
    max_bank_angle: 0x50,
    smoothing: 0x54,
    follow_axis: 0x58,
    max_curvature: 0x5c,
    bank_dir: 0x60,
    bank_scale: 0x011f_3f78,
    bank_scale_guard: 0x011f_3f7c,
};

// Translated from 00a49e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPathController::CalcRefFrame` (Xbox PDB; `thiscall`, (first key, second key, ratio,
/// &matrix), `RET 0x10`): the path's frame at `ratio` between the two keys. The exe's body
/// is `NiPathInterpolator::CalcRefFrame`'s (`00a3de50`) instruction by instruction with
/// this class's offsets (`CONTROLLER_REF_FRAME`), so it runs the shared translation
/// (`nipathinterpolator::calc_ref_frame`, which describes the frame).
pub fn ni_path_controller_calc_ref_frame(
    e: &mut Engine,
    this: Ptr<NiPathController>,
    first: u32,
    second: u32,
    ratio: f32,
    matrix: Ptr,
) {
    calc_ref_frame(
        e,
        this.addr(),
        &CONTROLLER_REF_FRAME,
        first,
        second,
        ratio,
        matrix.addr(),
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    type C = NiPathController;
    vec![
        entry!(0x00a49e00, ni_path_controller_get_rtti() -> Ptr),
        entry!(0x00a4a8c0, ni_path_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_9c80, ni_path_controller_process_clone(Ptr<C>, u32)),
        entry!(
            0x00a4_9cc0,
            ni_path_controller_register_streamables(Ptr<C>, u32) -> bool
        ),
        entry!(0x00a4_9d00, fn_00a49d00(Ptr<C>, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00a4_9d60, fn_00a49d60(Ptr<C>) -> u32),
        entry!(
            0x00a4_9e10,
            ni_path_controller_gen_path_interp(Ptr<C>, Ptr, u32, u32, f32) -> Ptr
        ),
        entry!(
            0x00a4_9e70,
            ni_path_controller_calc_ref_frame(Ptr<C>, u32, u32, f32, Ptr)
        ),
        entry!(0x00a4_a610, fn_00a4a610(Ptr<C>, u32, u32, f32) -> f32),
        entry!(0x00a4_a6a0, fn_00a4a6a0(Ptr<C>, u32, u32, f32) -> f32),
        entry!(
            0x00a4_a710,
            ni_path_controller_is_equal(Ptr<C>, Ptr<C>) -> bool
        ),
        entry!(0x00a4_a8e0, fn_00a4a8e0(Ptr<C>) -> f32),
        entry!(0x00a4_a980, fn_00a4a980(Ptr<C>, f32, Ptr, Ptr, Ptr)),
        entry!(0x00a4_ab50, fn_00a4ab50(Ptr<C>, f32, Ptr, Ptr, Ptr)),
        entry!(0x00a4_acb0, ni_path_controller_load_binary(Ptr<C>, u32)),
        entry!(0x00a4_adc0, ni_path_controller_link_object(Ptr<C>, u32)),
        entry!(0x00a4_aeb0, ni_path_controller_save_binary(Ptr<C>, u32)),
        entry!(0x00a4_afa0, ni_path_controller_update(Ptr<C>, u32)),
        entry!(0x00a4_b0b0, fn_00a4b0b0(Ptr<C>, Ptr<C>, u32)),
        entry!(0x00a4_b270, fn_00a4b270() -> Ptr<C>),
        entry!(0x00a4_b2e0, ni_path_controller_get_viewer_strings(Ptr, u32)),
        entry!(
            0x00a4_b330,
            ni_path_controller_create_clone(Ptr<C>, u32) -> Ptr<C>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_path_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a49e00, &args![]).u32(), NI_PATH_CONTROLLER_RTTI);
    }

    #[test]
    fn ni_path_controller_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_9d60, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a4a8c0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a4a8c0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x64]]);
        assert_eq!(calls_to(&e, 0x00a4_9d60), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n005_tests {
    use super::*;

    type C = NiPathController;

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

    fn controller(e: &mut Engine) -> Ptr<C> {
        Ptr::new(object(e, 0x64))
    }

    /// Key data (`NiPosData`/`NiFloatData` layout), counted once.
    fn key_data(e: &mut Engine, count: u32, key_type: u32, keys: u32, size: u8) -> u32 {
        let data = object(e, 0x18);
        e.mem.set_u32(data + 4, 1);
        e.mem.set_u32(data + KEY_DATA_NUM_KEYS, count);
        e.mem.set_u32(data + KEY_DATA_TYPE, key_type);
        e.mem.set_u32(data + KEY_DATA_KEYS, keys);
        e.mem.set_u8(data + KEY_DATA_KEY_SIZE, size);
        data
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
    fn process_clone_and_register_streamables_reach_both_data() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_PROCESS_CLONE, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_REGISTER_STREAMABLES, |_, a| {
            (a[1] == 0x77).into_ret()
        });
        e.register(0x0099_0068, |_, _| Ret::default());
        e.register(0x0099_0054, |_, _| Ret::default());
        let this = controller(&mut e);
        let path = key_data(&mut e, 0, 0, 0, 0);
        let percentage = key_data(&mut e, 0, 0, 0, 0);
        e.set(this, C::m_spPctData, percentage);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9c80, &[this.addr(), 0x66]);
        assert_eq!(calls_to(&e, 0x0099_0068), vec![vec![percentage, 0x66]]);
        e.set(this, C::m_spPathData, path);
        assert!(!e.call(0x00a4_9cc0, &[this.addr(), 0x78]).bool());
        assert!(calls_to(&e, 0x0099_0054).is_empty());
        assert!(e.call(0x00a4_9cc0, &[this.addr(), 0x77]).bool());
        assert_eq!(
            calls_to(&e, 0x0099_0054),
            vec![vec![path, 0x77], vec![percentage, 0x77]]
        );
    }

    #[test]
    fn path_keys_00a49d00_write_all_four() {
        let mut e = Engine::new();
        let this = controller(&mut e);
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 0xc, 0xffff_ffff);
        e.call(
            0x00a4_9d00,
            &[this.addr(), out, out + 4, out + 8, out + 0xc],
        );
        assert_eq!(
            (0..4).map(|i| e.mem.u32(out + 4 * i)).collect::<Vec<_>>(),
            [0, 0, 0, 0xffff_ff00]
        );
        let path = key_data(&mut e, 5, 3, 0x7000, 0x10);
        e.set(this, C::m_spPathData, path);
        e.call(
            0x00a4_9d00,
            &[this.addr(), out, out + 4, out + 8, out + 0xc],
        );
        assert_eq!(
            (0..4).map(|i| e.mem.u32(out + 4 * i)).collect::<Vec<_>>(),
            [0x7000, 5, 3, 0xffff_ff10]
        );
    }

    #[test]
    fn destructor_body_releases_the_data() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_DESTRUCT, |_, _| 0x31u32.into_ret());
        let this = controller(&mut e);
        let path = key_data(&mut e, 0, 0, 0, 0);
        let percentage = key_data(&mut e, 0, 0, 0, 0);
        e.mem.set_u32(percentage + 4, 2);
        e.set(this, C::m_spPathData, path);
        e.set(this, C::m_spPctData, percentage);
        e.set(this, C::m_pfPartialLength, 0x4000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_9d60, &[this.addr()]).u32(), 0x31);
        assert_eq!(e.mem.u32(this.addr()), NI_PATH_CONTROLLER_VTABLE);
        assert_eq!(calls_to(&e, 0x0099_0004), vec![vec![path]]);
        assert_eq!(e.mem.u32(percentage + 4), 1);
        assert_eq!(e.get(this, C::m_spPathData), 0);
        assert_eq!(e.get(this, C::m_spPctData), 0);
        assert_eq!(calls_to(&e, TABLE_FREE), vec![vec![0x4000]]);
    }

    /// The key-type tables mapped: each type's interpolation, derivative and curvature
    /// function `0x0098_0000 + table offset + 4 * type`.
    fn tables(e: &mut Engine) {
        e.map(0x011f_3000, 0x1000);
        for table in [
            POS_KEY_INTERPOLATIONS,
            POS_KEY_DERIVATIVES,
            POS_KEY_CURVATURES,
        ] {
            for key_type in 0..4u32 {
                e.mem.set_u32(
                    table + 4 * key_type,
                    0x0098_0000 + (table & 0xfff) + 4 * key_type,
                );
            }
        }
    }

    fn function(table: u32, key_type: u32) -> u32 {
        0x0098_0000 + (table & 0xfff) + 4 * key_type
    }

    #[test]
    fn gen_path_interp_runs_the_key_type_interpolation() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(function(POS_KEY_INTERPOLATIONS, 2), |_, _| Ret::default());
        let this = controller(&mut e);
        let path = key_data(&mut e, 4, 2, 0x7000, 0x10);
        e.set(this, C::m_spPathData, path);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00a4_9e10, &[this.addr(), 0x5000, 1, 2, 0x7f80_0001])
            .u32();
        assert_eq!(result, 0x5000);
        assert_eq!(
            calls_to(&e, function(POS_KEY_INTERPOLATIONS, 2)),
            vec![vec![0x7fc0_0001, 0x7010, 0x7020, 0x5000]]
        );
    }

    /// A controller whose path derivative is (3, 0, 4) * `ratio` (so the speed is 5 *
    /// ratio).
    fn speedy(e: &mut Engine) -> Ptr<C> {
        tables(e);
        e.register(function(POS_KEY_DERIVATIVES, 1), |e, a| {
            let ratio = f32::from_bits(a[0]);
            e.mem.set_f32(a[3], 3.0 * ratio);
            e.mem.set_f32(a[3] + 4, 0.0);
            e.mem.set_f32(a[3] + 8, 4.0 * ratio);
            Ret::default()
        });
        e.register(CRT_SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        let this = controller(e);
        let path = key_data(e, 3, 1, 0x7000, 0x10);
        e.set(this, C::m_spPathData, path);
        this
    }

    #[test]
    fn speed_is_the_derivative_length() {
        let mut e = Engine::new();
        let this = speedy(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00a4_a610, &[this.addr(), 0, 1, 2.0f32.to_bits()])
                .f32(),
            10.0
        );
        assert_eq!(
            calls_to(&e, function(POS_KEY_DERIVATIVES, 1))[0][..3],
            [2.0f32.to_bits(), 0x7000, 0x7010]
        );
        assert_eq!(calls_to(&e, CRT_SQRT), vec![args![100.0f64]]);
    }

    #[test]
    fn length_integrates_the_speed() {
        let mut e = Engine::new();
        e.map(0x011a_8000, 0x1000);
        // Weights 1 at abscissas 0.5: five samples of speed 5 * 0.5 * ratio.
        for i in 0..5 {
            e.mem.set_f32(GAUSS_ABSCISSAS + 4 * i, 0.5);
            e.mem.set_f32(GAUSS_WEIGHTS + 4 * i, 0.25);
        }
        let this = speedy(&mut e);
        let length = e
            .call(0x00a4_a6a0, &[this.addr(), 0, 1, 2.0f32.to_bits()])
            .f32();
        // Each sample: speed at 0.5 * 2 = 5, weighted 1.25; summed 6.25, times 2.
        assert_eq!(length, 12.5);
    }

    #[test]
    fn total_length_rebuilds_the_partial_lengths() {
        let mut e = Engine::new();
        e.map(0x011a_8000, 0x1000);
        for i in 0..5 {
            e.mem.set_f32(GAUSS_ABSCISSAS + 4 * i, 0.5);
            e.mem.set_f32(GAUSS_WEIGHTS + 4 * i, 0.2);
        }
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = speedy(&mut e);
        e.set(this, C::m_fTotalLength, (-1.0f32).to_bits());
        e.call_log = Some(vec![]);
        let total = e.call(0x00a4_a8e0, &[this.addr()]).f32();
        // Two segments of length 2.5 each (speed 2.5 at 0.5, weights 0.2 * 5).
        let lengths = e.get(this, C::m_pfPartialLength);
        assert_eq!(calls_to(&e, TABLE_ALLOCATE), vec![vec![12]]);
        assert_eq!(
            (0..3)
                .map(|i| e.mem.f32(lengths + 4 * i))
                .collect::<Vec<_>>(),
            [0.0, 2.5, 5.0]
        );
        assert_eq!(total, 5.0);
        // A length already known is answered as it is.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_a8e0, &[this.addr()]).f32(), 5.0);
        assert!(calls_to(&e, TABLE_ALLOCATE).is_empty());
    }

    /// Two equal controllers with data and partial lengths.
    fn equal_pair(e: &mut Engine) -> (Ptr<C>, Ptr<C>) {
        let mut pair = Vec::new();
        for _ in 0..2 {
            let this = controller(e);
            let path = key_data(e, 2, 1, 0, 0);
            let percentage = key_data(e, 2, 1, 0, 0);
            e.set(this, C::m_spPathData, path);
            e.set(this, C::m_spPctData, percentage);
            let lengths = e.mem.alloc(8);
            e.mem.set_f32(lengths + 4, 3.0);
            e.set(this, C::m_pfPartialLength, lengths);
            e.set(this, C::m_fTotalLength, 3.0f32.to_bits());
            e.set(this, C::m_uFlags, 0x35);
            e.set(this, C::m_eBankDir, 1);
            e.set(this, C::m_fMaxBankAngle, 0.5f32.to_bits());
            e.set(this, C::m_fSmoothing, 0.25f32.to_bits());
            e.set(this, C::m_sFollowAxis, 2);
            pair.push(this);
        }
        (pair[0], pair[1])
    }

    #[test]
    fn is_equal_compares_data_lengths_and_settings() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_IS_EQUAL, |_, _| true.into_ret());
        e.register(0x0099_005c, |_, _| true.into_ret());
        let (a, b) = equal_pair(&mut e);
        let equal = |e: &mut Engine| e.call(0x00a4_a710, &[a.addr(), b.addr()]).bool();
        assert!(equal(&mut e));
        let lengths = e.get(b, C::m_pfPartialLength);
        e.mem.set_f32(lengths + 4, 3.5);
        assert!(!equal(&mut e));
        e.mem.set_f32(lengths + 4, 3.0);
        e.set(b, C::m_pfPartialLength, 0);
        assert!(!equal(&mut e));
        e.set(b, C::m_pfPartialLength, lengths);
        for this in [a, b] {
            e.set(this, C::m_fTotalLength, 0x7fc0_0000);
        }
        assert!(!equal(&mut e));
        for this in [a, b] {
            e.set(this, C::m_fTotalLength, 3.0f32.to_bits());
        }
        for bit in 0..7u16 {
            e.set(b, C::m_uFlags, 0x35 ^ (1 << bit));
            assert!(!equal(&mut e), "bit {bit}");
        }
        e.set(b, C::m_uFlags, 0x35 ^ 0x80);
        assert!(equal(&mut e));
        e.set(b, C::m_uFlags, 0x35);
        e.set(b, C::m_eBankDir, 0);
        assert!(!equal(&mut e));
        e.set(b, C::m_eBankDir, 1);
        e.set(b, C::m_fMaxBankAngle, 0.75f32.to_bits());
        assert!(!equal(&mut e));
        e.set(b, C::m_fMaxBankAngle, 0.5f32.to_bits());
        e.set(b, C::m_sFollowAxis, 0);
        assert!(!equal(&mut e));
        e.set(b, C::m_sFollowAxis, 2);
        e.register(0x0099_005c, |_, _| false.into_ret());
        assert!(!equal(&mut e));
        e.register(0x0099_005c, |_, _| true.into_ret());
        e.register(TIME_CONTROLLER_IS_EQUAL, |_, _| false.into_ret());
        assert!(!equal(&mut e));
    }

    /// A controller with three path keys whose partial lengths are 0, 1 and 3 (total 3),
    /// and a speed of 2 everywhere with lengths `2 * ratio` per segment.
    fn arc(e: &mut Engine) -> Ptr<C> {
        e.map(0x0103_2000, 0x1000);
        e.mem.set_u64(ARC_LENGTH_TOLERANCE, 0.0001f64.to_bits());
        e.map(0x011a_8000, 0x1000);
        for i in 0..5 {
            e.mem.set_f32(GAUSS_ABSCISSAS + 4 * i, 0.5);
            e.mem.set_f32(GAUSS_WEIGHTS + 4 * i, 0.2);
        }
        tables(e);
        e.register(function(POS_KEY_DERIVATIVES, 1), |e, a| {
            e.mem.set_f32(a[3], 2.0);
            e.mem.set_f32(a[3] + 4, 0.0);
            e.mem.set_f32(a[3] + 8, 0.0);
            Ret::default()
        });
        e.register(CRT_SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        e.register(CI_FMOD, |_, a| {
            let x = f64::take(a, &mut 0);
            let y = f64::take(a, &mut 2);
            (x % y).into_ret()
        });
        let this = controller(e);
        let path = key_data(e, 3, 1, 0x7000, 0x10);
        e.set(this, C::m_spPathData, path);
        let lengths = e.mem.alloc(12);
        e.mem.set_f32(lengths + 4, 1.0);
        e.mem.set_f32(lengths + 8, 3.0);
        e.set(this, C::m_pfPartialLength, lengths);
        e.set(this, C::m_fTotalLength, 3.0f32.to_bits());
        this
    }

    fn search(e: &mut Engine, address: u32, this: Ptr<C>, time: f32) -> (u32, u32, f32) {
        let out = e.mem.alloc(12);
        e.call(
            address,
            &[this.addr(), time.to_bits(), out, out + 4, out + 8],
        );
        (e.mem.u32(out), e.mem.u32(out + 4), e.mem.f32(out + 8))
    }

    #[test]
    fn constant_speed_search_finds_the_segment_and_ratio() {
        let mut e = Engine::new();
        let this = arc(&mut e);
        assert_eq!(search(&mut e, 0x00a4_a980, this, 0.0), (0, 1, 0.0));
        assert_eq!(search(&mut e, 0x00a4_a980, this, 1.0), (1, 2, 1.0));
        // Half way: distance 1.5, in the second segment (1 to 3), 0.5 in: ratio 0.25.
        assert_eq!(search(&mut e, 0x00a4_a980, this, 0.5), (1, 2, 0.25));
        // Folded times: -0.5 is 0.5, 1.5 is 0.5.
        assert_eq!(search(&mut e, 0x00a4_a980, this, -0.5), (1, 2, 0.25));
        assert_eq!(search(&mut e, 0x00a4_a980, this, 1.5), (1, 2, 0.25));
    }

    #[test]
    fn segment_search_by_key_times() {
        let mut e = Engine::new();
        let this = arc(&mut e);
        // Keys at times 0, 1, 4 (0x10 bytes each).
        let keys = e.mem.alloc(0x30);
        for (i, t) in [0.0f32, 1.0, 4.0].iter().enumerate() {
            e.mem.set_f32(keys + 0x10 * i as u32, *t);
        }
        let path = e.get(this, C::m_spPathData);
        e.mem.set_u32(path + KEY_DATA_KEYS, keys);
        assert_eq!(search(&mut e, 0x00a4_ab50, this, -1.0), (0, 1, 0.0));
        assert_eq!(search(&mut e, 0x00a4_ab50, this, 5.0), (1, 2, 1.0));
        assert_eq!(search(&mut e, 0x00a4_ab50, this, 2.5), (1, 2, 0.5));
        assert_eq!(e.get(this, C::m_uiLastPathIdx), 1);
        // From the last index on, wrapping to 0 once.
        assert_eq!(search(&mut e, 0x00a4_ab50, this, 0.5), (0, 1, 0.5));
        assert_eq!(e.get(this, C::m_uiLastPathIdx), 0);
        // Constant speed: the arc-length search, its first key kept.
        e.set(this, C::m_uFlags, 0x10);
        assert_eq!(search(&mut e, 0x00a4_ab50, this, 0.5), (1, 2, 0.25));
        assert_eq!(e.get(this, C::m_uiLastPathIdx), 1);
    }

    /// A stream whose reads answer `words` (low bytes for 2-byte reads) and whose writes are
    /// recorded.
    fn test_stream(e: &mut Engine, words: Vec<u32>) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, 0x0300_0000);
        e.mem.set_u32(file + 0xc, 0x0300_0004);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 0x250, file);
        let mut words = words;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            let word = words.remove(0);
            e.mem.write(a[1], &word.to_le_bytes()[..a[2] as usize]);
            a[2].into_ret()
        });
        e.register(0x0300_0004, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            a[2].into_ret()
        });
        (stream, file)
    }

    #[test]
    fn load_binary_reads_the_settings() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        let (stream, file) = test_stream(
            &mut e,
            vec![0x15, 2, 0.5f32.to_bits(), 0.25f32.to_bits(), 1],
        );
        e.mem.set_u32(stream + 0xd8, 0x0a00_0102);
        let this = controller(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a4_acb0, &[this.addr(), stream]);
        assert_eq!(e.get(this, C::m_uFlags), 0x15);
        assert_eq!(e.get(this, C::m_eBankDir), 2);
        assert_eq!(e.get(this, C::m_fMaxBankAngle), 0.5f32.to_bits());
        assert_eq!(e.get(this, C::m_fSmoothing), 0.25f32.to_bits());
        assert_eq!(e.get(this, C::m_sFollowAxis), 1);
        assert_eq!(e.mem.u32(file + 4), 16);
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID).len(), 2);
        // An older file takes the flags from the stream's controller flags.
        let (stream, _) = test_stream(&mut e, vec![2, 0, 0, 1]);
        e.mem.set_u32(stream + 0xd8, 0x0a00_0101);
        e.mem.set_u16(stream + 0x286, 0x0260);
        e.call(0x00a4_acb0, &[this.addr(), stream]);
        assert_eq!(e.get(this, C::m_uFlags), 0x13);
    }

    #[test]
    fn link_object_links_data_and_computes_lengths_and_curvature() {
        let mut e = Engine::new();
        counting(&mut e);
        tables(&mut e);
        e.register(TIME_CONTROLLER_LINK_OBJECT, |_, _| Ret::default());
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(function(POS_KEY_CURVATURES, 1), |_, a| {
            f64::from(a[1] as f32 * 0.5).into_ret()
        });
        let path = key_data(&mut e, 1, 1, 0x7000, 0x10);
        let percentage = key_data(&mut e, 2, 0, 0, 4);
        let mut linked = vec![path, percentage];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            linked.remove(0).into_ret()
        });
        let this = controller(&mut e);
        e.set(this, C::m_uFlags, 0x10);
        e.set(this, C::m_fTotalLength, (-1.0f32).to_bits());
        e.call(0x00a4_adc0, &[this.addr(), 0x9000]);
        assert_eq!(e.get(this, C::m_spPathData), path);
        assert_eq!(e.get(this, C::m_spPctData), percentage);
        assert_eq!(e.mem.u32(path + 4), 2);
        // Constant speed: the total recomputed (one key: 0) and bit 0 cleared again.
        assert_eq!(e.get(this, C::m_uFlags), 0x10);
        assert_eq!(e.get(this, C::m_fTotalLength), 0);
        assert_eq!(e.get(this, C::m_fMaxCurvature), 0.5f32.to_bits());
    }

    #[test]
    fn save_binary_writes_the_settings_and_links() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0099_0030, |_, _| Ret::default());
        let (stream, file) = test_stream(&mut e, vec![]);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 0x30, 0x0099_0030);
        e.mem.set_u32(stream, table);
        let this = controller(&mut e);
        e.set(this, C::m_spPathData, 0x1111);
        e.set(this, C::m_spPctData, 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x00a4_aeb0, &[this.addr(), stream]);
        let t = this.addr();
        let writes: Vec<(u32, u32)> = calls_to(&e, 0x0300_0004)
            .iter()
            .map(|a| (a[1], a[2]))
            .collect();
        assert_eq!(
            writes,
            [
                (t + 0x34, 2),
                (t + 0x60, 4),
                (t + 0x50, 4),
                (t + 0x54, 4),
                (t + 0x58, 2)
            ]
        );
        assert_eq!(e.mem.u32(file + 4), 16);
        assert_eq!(
            calls_to(&e, 0x0099_0030),
            vec![vec![stream, 0x1111], vec![stream, 0x2222]]
        );
    }

    #[test]
    fn update_moves_and_turns_the_target() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(DONT_DO_UPDATE, |_, a| (a[1] == 0x7f80_0000).into_ret());
        // The percentage: 0.5; the segment search by key times (keys at 0 and 1): (0, 1, 0.5).
        e.register(FLOAT_KEY_GEN_INTERP, |_, _| 0.5f64.into_ret());
        e.register(function(POS_KEY_INTERPOLATIONS, 1), |e, a| {
            let ratio = f32::from_bits(a[0]);
            for i in 0..3 {
                e.mem.set_f32(a[3] + 4 * i, ratio + i as f32);
            }
            Ret::default()
        });
        let this = controller(&mut e);
        let keys = e.mem.alloc(0x20);
        e.mem.set_f32(keys + 0x10, 1.0);
        let path = key_data(&mut e, 2, 1, keys, 0x10);
        let percentage = key_data(&mut e, 4, 3, 0x6000, 0x08);
        e.set(this, C::m_spPathData, path);
        e.set(this, C::m_spPctData, percentage);
        e.set(this, C::m_fScaledTime, 2.0f32.to_bits());
        let target = e.mem.alloc(0x70);
        e.set(this, C::m_pkTarget, target);
        let update = e.mem.alloc(4);
        e.mem.set_f32(update, 3.0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_afa0, &[this.addr(), update]);
        assert_eq!(
            calls_to(&e, DONT_DO_UPDATE),
            vec![vec![this.addr(), 3.0f32.to_bits()]]
        );
        // The percentage keys, with the path data's key size.
        assert_eq!(
            calls_to(&e, FLOAT_KEY_GEN_INTERP),
            vec![vec![
                2.0f32.to_bits(),
                0x6000,
                3,
                4,
                this.addr() + 0x3c,
                0x10
            ]]
        );
        let translation: Vec<f32> = (0..3).map(|i| e.mem.f32(target + 0x58 + 4 * i)).collect();
        assert_eq!(translation, [0.5, 1.5, 2.5]);
        // Without bit 5 the rotation is left alone.
        assert_eq!(e.mem.u32(target + 0x34), 0);
        // Told not to update: nothing.
        e.mem.set_u32(update, 0x7f80_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a4_afa0, &[this.addr(), update]);
        assert!(calls_to(&e, FLOAT_KEY_GEN_INTERP).is_empty());
        // Without percentage keys: nothing either.
        e.mem.set_u32(percentage + KEY_DATA_KEYS, 0);
        e.mem.set_f32(update, 3.0);
        e.call(0x00a4_afa0, &[this.addr(), update]);
        assert_eq!(calls_to(&e, DONT_DO_UPDATE).len(), 1);
    }

    #[test]
    fn update_with_bit_5_stores_the_frame() {
        let mut e = Engine::new();
        tables(&mut e);
        e.map(0x011a_9000, 0x1000);
        for i in 0..3 {
            e.mem.set_f32(0x011a_9448 + 16 * i, 1.0);
        }
        e.register(DONT_DO_UPDATE, |_, _| false.into_ret());
        e.register(FLOAT_KEY_GEN_INTERP, |_, _| 0.5f64.into_ret());
        e.register(function(POS_KEY_INTERPOLATIONS, 1), |_, _| Ret::default());
        let this = controller(&mut e);
        let keys = e.mem.alloc(0x20);
        e.mem.set_f32(keys + 0x10, 1.0);
        // One key short of a frame: CalcRefFrame answers the default matrix.
        let path = key_data(&mut e, 2, 1, keys, 0x10);
        e.mem.set_u32(path + KEY_DATA_NUM_KEYS, 1);
        let percentage = key_data(&mut e, 4, 3, 0x6000, 0x08);
        e.set(this, C::m_spPathData, path);
        e.set(this, C::m_spPctData, percentage);
        e.set(this, C::m_uFlags, 0x20);
        let target = e.mem.alloc(0x70);
        e.set(this, C::m_pkTarget, target);
        let update = e.mem.alloc(4);
        e.call(0x00a4_afa0, &[this.addr(), update]);
        let rotation: Vec<f32> = (0..9).map(|i| e.mem.f32(target + 0x34 + 4 * i)).collect();
        assert_eq!(rotation, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn copy_members_copies_settings_and_lengths() {
        let mut e = Engine::new();
        counting(&mut e);
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let clone = key_data(&mut e, 0, 0, 0, 0);
        e.register_double(0x0099_0048, move |_, _| clone.into_ret());
        let this = controller(&mut e);
        let path = key_data(&mut e, 2, 1, 0, 0);
        e.set(this, C::m_spPathData, path);
        let lengths = e.mem.alloc(8);
        e.mem.set_f32(lengths + 4, 2.0);
        e.set(this, C::m_pfPartialLength, lengths);
        e.set(this, C::m_uFlags, 0x6c);
        e.set(this, C::m_eBankDir, 2);
        e.set(this, C::m_fMaxBankAngle, 0x7f80_0001);
        e.set(this, C::m_fSmoothing, 0.25f32.to_bits());
        e.set(this, C::m_sFollowAxis, 1);
        e.set(this, C::m_fTotalLength, 2.0f32.to_bits());
        let destination = controller(&mut e);
        e.set(destination, C::m_uFlags, 0x93);
        e.call_log = Some(vec![]);
        e.call(0x00a4_b0b0, &[this.addr(), destination.addr(), 0x5555]);
        assert_eq!(calls_to(&e, 0x0099_0048), vec![vec![path, 0x5555]]);
        assert_eq!(e.get(destination, C::m_spPathData), clone);
        assert_eq!(e.mem.u32(clone + 4), 2);
        assert_eq!(e.get(destination, C::m_spPctData), 0);
        // The flags word is copied first, then each bit again.
        assert_eq!(e.get(destination, C::m_uFlags), 0x6c);
        assert_eq!(e.get(destination, C::m_eBankDir), 2);
        // The bank angle went through the x87 stack.
        assert_eq!(e.get(destination, C::m_fMaxBankAngle), 0x7fc0_0001);
        assert_eq!(e.get(destination, C::m_sFollowAxis), 1);
        let copied = e.get(destination, C::m_pfPartialLength);
        assert_eq!(e.mem.f32(copied + 4), 2.0);
        assert_eq!(calls_to(&e, TABLE_ALLOCATE), vec![vec![8]]);
        assert_eq!(e.get(destination, C::m_fTotalLength), 2.0f32.to_bits());
    }

    #[test]
    fn copy_members_with_constant_speed_and_stale_lengths_recomputes_the_total() {
        let mut e = Engine::new();
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = controller(&mut e);
        e.set(this, C::m_uFlags, 0x11);
        let destination = controller(&mut e);
        let path = key_data(&mut e, 1, 0, 0, 0);
        e.set(destination, C::m_spPathData, path);
        e.set(destination, C::m_fTotalLength, (-1.0f32).to_bits());
        e.call(0x00a4_b0b0, &[this.addr(), destination.addr(), 0]);
        assert_eq!(e.get(destination, C::m_uFlags), 0x10);
        // The destination's own path (one key) gives 0, then the source's total is copied.
        assert_eq!(e.get(destination, C::m_fTotalLength), 0);
    }

    #[test]
    fn creation_and_clone_build_a_default_controller() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(block + offset, 0xffff_ffff);
            }
            block.into_ret()
        });
        e.register(TIME_CONTROLLER_CONSTRUCT, |_, _| Ret::default());
        e.register(TIME_CONTROLLER_COPY_MEMBERS, |_, _| Ret::default());
        let made = Ptr::<C>::new(e.call(0x00a4_b270, &[]).u32());
        assert_eq!(e.mem.u32(made.addr()), NI_PATH_CONTROLLER_VTABLE);
        assert_eq!(e.get(made, C::m_uFlags), 3);
        assert_eq!(e.get(made, C::m_fTotalLength), (-1.0f32).to_bits());
        assert_eq!(e.get(made, C::m_eBankDir), 1);
        for field in [
            C::m_spPathData,
            C::m_spPctData,
            C::m_pfPartialLength,
            C::m_fMaxCurvature,
        ] {
            assert_eq!(e.get(made, field), 0);
        }
        let source = controller(&mut e);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a4_b330, &[source.addr(), 0x5555]).u32();
        assert_eq!(e.mem.u32(clone), NI_PATH_CONTROLLER_VTABLE);
        assert_eq!(
            calls_to(&e, TIME_CONTROLLER_COPY_MEMBERS),
            vec![vec![source.addr(), clone, 0x5555]]
        );
        // A failed allocation: no object. (`CreateClone` would then run the member copy on
        // a null clone, which writes to it, as the exe does.)
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_b270, &[]).u32(), 0);
        assert!(calls_to(&e, TIME_CONTROLLER_CONSTRUCT).is_empty());
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = Engine::new();
        e.map(NI_PATH_CONTROLLER_RTTI & !0xfff, 0x1000);
        e.set_global(NI_PATH_CONTROLLER_RTTI, 0x0444_0000u32);
        e.register(TIME_CONTROLLER_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, a| (a[0] + 1).into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0444_0001);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xa, 3);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xe, 1);
        e.call_log = Some(vec![]);
        e.call(0x00a4_b2e0, &[0x5000, strings]);
        assert_eq!(calls_to(&e, STRINGS_SET_SIZE), vec![vec![strings, 4]]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 3]);
    }

    #[test]
    fn calc_ref_frame_uses_the_controller_offsets() {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.map(0x0109_7000, 0x1000);
        e.map(0x011f_3000, 0x1000);
        for i in 0..3 {
            e.mem.set_f32(0x011a_9448 + 16 * i, 1.0);
        }
        e.mem.set_f32(0x0109_7304, 1.0);
        // 00a277c0: tangent (1, 2, 3), second (0.5, 0.25, 0.125), third (7, 8, 9).
        e.register(0x00a2_77c0, |e, a| {
            for (at, point) in [
                (a[4], [1.0f32, 2.0, 3.0]),
                (a[5], [0.5, 0.25, 0.125]),
                (a[6], [7.0, 8.0, 9.0]),
            ] {
                for (i, v) in point.iter().enumerate() {
                    e.mem.set_f32(at + 4 * i as u32, *v);
                }
            }
            e.mem.set_f32(a[7], 0.5);
            Ret::default()
        });
        e.register(0x0041_68a0, |_, _| Ret::default());
        e.register(0x0043_f8d0, |_, a| a[2].into_ret());
        let this = controller(&mut e);
        let path = key_data(&mut e, 3, 2, 0x7000, 0x10);
        e.set(this, C::m_spPathData, path);
        // Bits 2 (vectors), 6 (flip) and 3 (bank); follow axis 1; no smoothing.
        e.set(this, C::m_uFlags, 0x4c);
        e.set(this, C::m_sFollowAxis, 1);
        e.set(this, C::m_eBankDir, 1);
        e.set(this, C::m_fMaxBankAngle, 0.5f32.to_bits());
        let out = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9e70, &[this.addr(), 0, 1, 0.5f32.to_bits(), out]);
        // Columns (t, -n, -b), flipped (-t, n, -b), axis 1 (-n, -t, -b).
        let rows: Vec<f32> = (0..9).map(|i| e.mem.f32(out + 4 * i)).collect();
        assert_eq!(
            rows,
            [-0.5, -1.0, -7.0, -0.25, -2.0, -8.0, -0.125, -3.0, -9.0]
        );
        // The bank: the controller's static set up, the full angle (curvature 0.5 is not
        // below the maximum 0).
        assert_eq!(e.mem.u32(0x011f_3f7c) & 1, 1);
        assert_eq!(calls_to(&e, 0x0041_68a0)[0][1], 0.5f32.to_bits());
    }
}
