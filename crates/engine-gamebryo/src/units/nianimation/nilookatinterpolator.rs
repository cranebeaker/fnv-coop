//! `NiAnimation/nilookatinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLookAtInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_LOOK_AT_INTERPOLATOR_RTTI: u32 = 0x011f_3f90;

// Translated from 00a4be10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_look_at_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LOOK_AT_INTERPOLATOR_RTTI)
}

// Translated from 00a4c5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a4be20`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x44 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_look_at_interpolator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_be20;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x44u32]);
    }
    this
}

/// `NiLookAtInterpolator`'s vtable (the address the constructor and destructor store) and
/// size.
const NI_LOOK_AT_INTERPOLATOR_VTABLE: u32 = 0x0109_8b0c;
const NI_LOOK_AT_INTERPOLATOR_SIZE: u32 = 0x44;
/// Fields (Xbox PDB names, PC offsets as the code uses them): `NiInterpolator::m_fLastTime`
/// +0x08, `m_uFlags` +0x0c (16 bits: bit 0 flip, bits 1-2 the axis), `m_pkLookAt` +0x10,
/// `m_kLookAtName` +0x14, `m_kTransformValue` +0x18 (translate +0x18, rotate +0x24, scale
/// +0x34) and `m_aspInterpolators` +0x38 (three pointers).
const LAST_TIME: u32 = 0x08;
const LOOK_AT_FLAGS: u32 = 0x0c;
const LOOK_AT: u32 = 0x10;
const LOOK_AT_NAME: u32 = 0x14;
const TRANSFORM: u32 = 0x18;
const ROTATE: u32 = 0x24;
const SCALE: u32 = 0x34;
const INTERPOLATORS: u32 = 0x38;
/// The empty fixed string's handle (`0109b220`) and the handle at `011f49e0` the name goes
/// back to without a look-at target.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
const NO_LOOK_AT_NAME: u32 = 0x011f_49e0;
/// The floats the active time range starts from (`01098b04`, `01015f5c`) and the doubles it
/// is compared with at the end (`010231b0`, `010241b0`); `01015f5c` is also the float an
/// invalid transform part is set to.
const RANGE_START_FLOAT: u32 = 0x0109_8b04;
const INVALID_FLOAT: u32 = 0x0101_5f5c;
const RANGE_START_DOUBLE: u32 = 0x0102_31b0;
const INVALID_DOUBLE: u32 = 0x0102_41b0;
/// The constructor's initial values: the translation (`011a8400`, three words), the rotation
/// (`011f3704`, four words) and the scale float (`01096b8c`).
const INITIAL_TRANSLATE: u32 = 0x011a_8400;
const INITIAL_ROTATE: u32 = 0x011f_3704;
const INITIAL_SCALE: u32 = 0x0109_6b8c;
/// `Update`'s constants: the smallest squared distance that turns the target (`01017d00`),
/// the bounds of a direction's z near +1 (`01098c00` float, `01098bf8` double) and near -1
/// (`01098bf0` float, `01098be8` double), the up vectors for those cases (`011a9478`) and
/// the others (`011a9484`), and the identity matrix (`011a9448`).
const MIN_DISTANCE_SQUARED: u32 = 0x0101_7d00;
const POLE_ABOVE: u32 = 0x0109_8c00;
const POLE_BELOW: u32 = 0x0109_8bf8;
const ANTIPOLE_BELOW: u32 = 0x0109_8bf0;
const ANTIPOLE_ABOVE: u32 = 0x0109_8be8;
const POLE_UP: u32 = 0x011a_9478;
const UP: u32 = 0x011a_9484;
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// Callees (thiscall unless noted): `NiInterpolator`'s constructor (`00a56ff0`), destructor
/// (`00a57010`), `GetSequenceInterpolator` (`00a57050`), `LoadBinary` (`00a88a40`) and
/// `GetViewerStrings` (`00a570b0`); `NiKeyBasedInterpolator::CopyMembers` (`00aa1000`); the
/// base `RegisterStreamables` (`00a81fe0`), `IsEqual` (`00ba87d0`) and `SaveBinary`
/// (`00a724d0`; the engine map names some after other classes, folded code);
/// `NiObject::ProcessClone` (`00a5d430`) and `NiObject::Clone` (`00a5d680`); the clone map's
/// lookup (`00a53c30`, `(key, &value)`); `NiQuatTransform`'s `operator==` (`00a391c0`),
/// `LoadBinary` (`00a38bb0`), `SaveBinary` (`00a38b60`) and `GetViewerStrings` (`00a38cb0`).
const NI_INTERPOLATOR_CONSTRUCT: u32 = 0x00a5_6ff0;
const NI_INTERPOLATOR_DESTRUCT: u32 = 0x00a5_7010;
const NI_INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR: u32 = 0x00a5_7050;
const NI_INTERPOLATOR_LOAD_BINARY: u32 = 0x00a8_8a40;
const NI_INTERPOLATOR_GET_VIEWER_STRINGS: u32 = 0x00a5_70b0;
const KEY_BASED_COPY_MEMBERS: u32 = 0x00aa_1000;
const BASE_REGISTER_STREAMABLES: u32 = 0x00a8_1fe0;
const BASE_IS_EQUAL: u32 = 0x00ba_87d0;
const BASE_SAVE_BINARY: u32 = 0x00a7_24d0;
const NI_OBJECT_PROCESS_CLONE: u32 = 0x00a5_d430;
const NI_OBJECT_CLONE: u32 = 0x00a5_d680;
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
const QUAT_TRANSFORM_EQUALS: u32 = 0x00a3_91c0;
const QUAT_TRANSFORM_LOAD_BINARY: u32 = 0x00a3_8bb0;
const QUAT_TRANSFORM_SAVE_BINARY: u32 = 0x00a3_8b60;
const QUAT_TRANSFORM_GET_VIEWER_STRINGS: u32 = 0x00a3_8cb0;
/// The stream: `ResolveLinkID` (`00a64620`), `LoadCStringAsFixedString` (`00a64ea0`),
/// `LoadFixedString` (`00a64df0`) and `SaveFixedString` (`00a641f0`); `SaveLinkID` (virtual
/// slot `0x30`) and `RegisterFixedString` (slot `0x20`).
const RESOLVE_LINK_ID: u32 = 0x00a6_4620;
const LOAD_CSTRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
const LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
const SAVE_FIXED_STRING: u32 = 0x00a6_41f0;
const SLOT_SAVE_LINK_ID: u32 = 0x30;
const SLOT_REGISTER_FIXED_STRING: u32 = 0x20;
/// `NiStream::m_uiNifFileVersion` +0xd8, `m_pkIstr` +0x24c and `m_pkOstr` +0x250 (Xbox PDB
/// names, PC offsets), and the binary stream's position (+4), read (+8) and write (+0xc).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// Viewer strings: `NiGetViewerString` (`00a7d330`) and its pointer form (`00a7d5a0`), cdecl;
/// the array's resize (`0096ad30`) and `SetAt` (`0096ae90`); the label `"m_pkLookAt"`.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;
const TEXT_LOOK_AT: u32 = 0x0109_8c04;
/// `00aa13e0` (cdecl, one word): allocates from the Ni allocator; `00a5b690` (cdecl): the
/// fixed-string handle for a C string.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const GET_FIXED_STRING: u32 = 0x00a5_b690;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Math callees of `Update` (thiscall): the matrix times a point (`004b4500`, `(&result,
/// &point)`), `NiPoint3::Unitize` (`004a0c10`), `NiMatrix3::TransposeTimes` (`00a588a0`,
/// `(&result, &matrix)`), the z rotation (`004a0c90`, the angle), the matrix product
/// (`0043f8d0`, `(&result, &matrix)`) and `NiQuaternion::FromRotation` (`00a6df40`, the
/// matrix); each answers its result's address where it has one.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
const POINT_UNITIZE: u32 = 0x004a_0c10;
const MATRIX_TRANSPOSE_TIMES: u32 = 0x00a5_88a0;
const MATRIX_MAKE_Z_ROTATION: u32 = 0x004a_0c90;
const MATRIX_TIMES_MATRIX: u32 = 0x0043_f8d0;
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
/// Virtual slots: `DeleteThis` (`0x04`), `IsEqual` (`0x5c`), `RegisterStreamables` (`0x54`),
/// `GetViewerStrings` (`0x60`); on interpolators the point3 update (`0x94`), the float update
/// (`0x9c`), `Collapse` (`0xbc`), `GetActiveTimeRange` (`0xc0`), `GuaranteeTimeRange`
/// (`0xc4`) and `GetSequenceInterpolator` (`0xc8`); the palette's object lookup (`0x8c`).
const SLOT_DELETE_THIS: u32 = 0x04;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_GET_VIEWER_STRINGS: u32 = 0x60;
const SLOT_UPDATE_POINT3: u32 = 0x94;
const SLOT_UPDATE_FLOAT: u32 = 0x9c;
const SLOT_COLLAPSE: u32 = 0xbc;
const SLOT_GET_ACTIVE_TIME_RANGE: u32 = 0xc0;
const SLOT_GUARANTEE_TIME_RANGE: u32 = 0xc4;
const SLOT_GET_SEQUENCE_INTERPOLATOR: u32 = 0xc8;
const SLOT_PALETTE_GET: u32 = 0x8c;

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at +4 and, when it
/// reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// The inline `NiPointer::operator=` for the pointer at `slot`.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
        }
    }
}

/// The inline `NiFixedString::operator=` of the handle at `slot` to `handle`: the new handle
/// counted and the old one uncounted (neither when it is the empty string's).
fn assign_fixed_string(e: &mut Engine, slot: u32, handle: u32) {
    if e.mem.u32(slot) == handle {
        return;
    }
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_INCREMENT, &args![handle.wrapping_sub(8)]);
    }
    let old = e.mem.u32(slot);
    if old != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
    }
    e.mem.set_u32(slot, handle);
}

/// The three interpolator slots' addresses.
fn interpolator_slots(this: u32) -> [u32; 3] {
    [0u32, 4, 8].map(|k| this.wrapping_add(INTERPOLATORS + k))
}

// Translated from 00a4b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::ResolveDependencies` (Xbox PDB): without a look-at object, the
/// palette finds the one named by the look-at name (its slot `0x8c`, the name's address);
/// false when it finds none. True otherwise.
pub fn ni_look_at_interpolator_resolve_dependencies(
    e: &mut Engine,
    this: Ptr,
    palette: Ptr,
) -> bool {
    let this = this.addr();
    if e.mem.u32(this.wrapping_add(LOOK_AT)) == 0 {
        let name = this.wrapping_add(LOOK_AT_NAME);
        let found = e
            .vcall(palette.addr(), SLOT_PALETTE_GET, &args![name])
            .u32();
        if found == 0 {
            return false;
        }
        e.mem.set_u32(this.wrapping_add(LOOK_AT), found);
    }
    true
}

// Translated from 00a4b900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::Collapse` (Xbox PDB): each of the three interpolators collapses
/// (slot `0xbc`).
pub fn ni_look_at_interpolator_collapse(e: &mut Engine, this: Ptr) {
    for slot in interpolator_slots(this.addr()) {
        let interpolator = e.mem.u32(slot);
        if interpolator != 0 {
            e.vcall(interpolator, SLOT_COLLAPSE, &args![]);
        }
    }
}

// Translated from 00a4b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::GetActiveTimeRange` (Xbox PDB): the range starts from the floats at
/// `01098b04` and `01015f5c`; each interpolator's active time range (slot `0xc0`, into the
/// two argument stack words) that is not empty (start equal to end; unordered counts as not
/// equal) lowers the start and raises the end (NaN changes nothing). A range still at both
/// starting values (compared with the doubles at `010231b0` and `010241b0`) becomes 0.0 to
/// 0.0.
pub fn ni_look_at_interpolator_get_active_time_range(
    e: &mut Engine,
    this: Ptr,
    start: Ptr,
    end: Ptr,
) {
    let start = start.addr();
    let end = end.addr();
    let value = x87_quiet_bits(e.mem.u32(RANGE_START_FLOAT));
    e.mem.set_u32(start, value);
    let value = x87_quiet_bits(e.mem.u32(INVALID_FLOAT));
    e.mem.set_u32(end, value);
    // Frame: the argument words, reused for each interpolator's range.
    e.with_stack(8, |e, frame| {
        let range_start = frame.addr();
        let range_end = range_start + 4;
        e.mem.set_u32(range_start, start);
        e.mem.set_u32(range_end, end);
        for slot in interpolator_slots(this.addr()) {
            let interpolator = e.mem.u32(slot);
            if interpolator == 0 {
                continue;
            }
            e.vcall(
                interpolator,
                SLOT_GET_ACTIVE_TIME_RANGE,
                &args![range_start, range_end],
            );
            let first_bits = e.mem.u32(range_start);
            let last_bits = e.mem.u32(range_end);
            let first = x87_load32(first_bits);
            let last = x87_load32(last_bits);
            if last == first {
                continue;
            }
            if x87_load32(e.mem.u32(start)) > first {
                e.mem.set_u32(start, x87_quiet_bits(first_bits));
            }
            if x87_load32(e.mem.u32(end)) < last {
                e.mem.set_u32(end, x87_quiet_bits(last_bits));
            }
        }
    });
    let unchanged_start = f64::from_bits(e.mem.u64(RANGE_START_DOUBLE));
    if x87_load32(e.mem.u32(start)) == unchanged_start {
        let unchanged_end = f64::from_bits(e.mem.u64(INVALID_DOUBLE));
        if x87_load32(e.mem.u32(end)) == unchanged_end {
            e.mem.set_f32(start, 0.0);
            e.mem.set_f32(end, 0.0);
        }
    }
}

// Translated from 00a4b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::GuaranteeTimeRange` (Xbox PDB): each interpolator gets
/// `GuaranteeTimeRange(start, end)` (slot `0xc4`).
pub fn ni_look_at_interpolator_guarantee_time_range(
    e: &mut Engine,
    this: Ptr,
    start: f32,
    end: f32,
) {
    for slot in interpolator_slots(this.addr()) {
        let interpolator = e.mem.u32(slot);
        if interpolator != 0 {
            let start = x87_quiet(start).to_bits();
            let end = x87_quiet(end).to_bits();
            e.vcall(interpolator, SLOT_GUARANTEE_TIME_RANGE, &args![start, end]);
        }
    }
}

// Translated from 00a4ba30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::GetSequenceInterpolator` (Xbox PDB): the clone
/// `NiInterpolator::GetSequenceInterpolator` makes, whose interpolators become this one's
/// interpolators' sequence interpolators (slot `0xc8` with the same range; counted, the
/// clone's old ones released).
pub fn ni_look_at_interpolator_get_sequence_interpolator(
    e: &mut Engine,
    this: Ptr,
    start: f32,
    end: f32,
) -> u32 {
    let start = x87_quiet(start).to_bits();
    let end = x87_quiet(end).to_bits();
    let clone = e
        .call(
            NI_INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR,
            &args![this, start, end],
        )
        .u32();
    let ours = interpolator_slots(this.addr());
    let theirs = interpolator_slots(clone);
    for (mine, slot) in ours.iter().zip(theirs) {
        let interpolator = e.mem.u32(*mine);
        if interpolator == 0 {
            continue;
        }
        let sequence = e
            .vcall(
                interpolator,
                SLOT_GET_SEQUENCE_INTERPOLATOR,
                &args![start, end],
            )
            .u32();
        assign_pointer(e, slot, sequence);
    }
    clone
}

// Translated from 00a4bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::SetUpDependencies` (Xbox PDB): with a name on the look-at object
/// (+8), the look-at name becomes that name's fixed string (`00a5b690`, the temporary
/// uncounted afterwards), the look-at object is cleared and true is returned; without one the
/// name becomes the handle at `011f49e0` and false is returned.
pub fn ni_look_at_interpolator_set_up_dependencies(e: &mut Engine, this: Ptr) -> bool {
    let this = this.addr();
    let look_at = e.mem.u32(this.wrapping_add(LOOK_AT));
    let name = e.mem.u32(look_at.wrapping_add(8));
    let slot = this.wrapping_add(LOOK_AT_NAME);
    if name == 0 {
        let handle = e.mem.u32(NO_LOOK_AT_NAME);
        assign_fixed_string(e, slot, handle);
        return false;
    }
    let handle = e.call(GET_FIXED_STRING, &args![name]).u32();
    assign_fixed_string(e, slot, handle);
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
    e.mem.set_u32(this.wrapping_add(LOOK_AT), 0);
    true
}

// Translated from 00a4bb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::ProcessClone` (Xbox PDB): `NiObject::ProcessClone`; then the clone
/// of this interpolator is looked up in the cloning process's map (`00a53c30` on its first
/// word, into the argument's stack word set to 0) and its look-at object becomes the look-at
/// object's clone when the map has one, the same object otherwise.
pub fn ni_look_at_interpolator_process_clone(e: &mut Engine, this: Ptr, cloning: Ptr) {
    e.call(NI_OBJECT_PROCESS_CLONE, &args![this, cloning]);
    let map = e.mem.u32(cloning.addr());
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, 0);
        e.call(CLONE_MAP_GET_AT, &args![map, this, slot]);
        let look_at = e.mem.u32(this.addr().wrapping_add(LOOK_AT));
        let clone = e.mem.u32(slot);
        let map = e.mem.u32(cloning.addr());
        let found = e.call(CLONE_MAP_GET_AT, &args![map, look_at, slot]).bool();
        let value = if found {
            e.mem.u32(slot)
        } else {
            e.mem.u32(this.addr().wrapping_add(LOOK_AT))
        };
        e.mem.set_u32(clone.wrapping_add(LOOK_AT), value);
    });
}

// Translated from 00a4bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::RegisterStreamables` (Xbox PDB): false when the base's
/// `RegisterStreamables` (`00a81fe0`) or one of the interpolators' (slot `0x54`) is; then the
/// look-at name registers with the stream (its slot `0x20`). True.
pub fn ni_look_at_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    for slot in interpolator_slots(this.addr()) {
        let interpolator = e.mem.u32(slot);
        if interpolator != 0
            && !e
                .vcall(interpolator, SLOT_REGISTER_STREAMABLES, &args![stream])
                .bool()
        {
            return false;
        }
    }
    let name = this.addr().wrapping_add(LOOK_AT_NAME);
    e.vcall(stream.addr(), SLOT_REGISTER_FIXED_STRING, &args![name]);
    true
}

// Translated from 00a4bc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::IsEqual` (Xbox PDB): the base's `IsEqual` (`00ba87d0`), the same
/// flags, both or neither look-at object (this one's `IsEqual` the other's, slot `0x5c`), the
/// same transform (`NiQuatTransform::operator==`), and per interpolator both or neither, this
/// one's `IsEqual` the other's.
pub fn ni_look_at_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let this = this.addr();
    let other = other.addr();
    if e.mem.u16(this.wrapping_add(LOOK_AT_FLAGS)) != e.mem.u16(other.wrapping_add(LOOK_AT_FLAGS)) {
        return false;
    }
    let look_at = e.mem.u32(this.wrapping_add(LOOK_AT));
    let other_look_at = e.mem.u32(other.wrapping_add(LOOK_AT));
    if (look_at != 0) != (other_look_at != 0) {
        return false;
    }
    if look_at != 0 {
        let other_look_at = e.mem.u32(other.wrapping_add(LOOK_AT));
        if other_look_at != 0 {
            let look_at = e.mem.u32(this.wrapping_add(LOOK_AT));
            if !e
                .vcall(look_at, SLOT_IS_EQUAL, &args![other_look_at])
                .bool()
            {
                return false;
            }
        }
    }
    if !e
        .call(
            QUAT_TRANSFORM_EQUALS,
            &args![this.wrapping_add(TRANSFORM), other.wrapping_add(TRANSFORM)],
        )
        .bool()
    {
        return false;
    }
    for (mine, theirs) in interpolator_slots(this)
        .iter()
        .zip(interpolator_slots(other))
    {
        let ours = e.mem.u32(*mine);
        let other_interpolator = e.mem.u32(theirs);
        if ours != 0 {
            if other_interpolator == 0 {
                return false;
            }
            if !e
                .vcall(ours, SLOT_IS_EQUAL, &args![other_interpolator])
                .bool()
            {
                return false;
            }
        } else if other_interpolator != 0 {
            return false;
        }
    }
    true
}

// Translated from 00a4bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::NiLookAtInterpolator` (Xbox PDB; `(look-at object, axis, flip)`):
/// the `NiInterpolator` constructor, the vtable, the look-at object, flags 0, the empty
/// name, the translation from `011a8400`, the rotation from `011f3704`, the scale from the
/// float at `01096b8c` and no interpolators; flag bit 0 is the low byte of `flip`, bits 1-2
/// the low 16 bits of `axis * 2` (or-ed into the cleared bits), and the (null) interpolators
/// are released as the member reset does. Returns `this`.
pub fn ni_look_at_interpolator_construct(
    e: &mut Engine,
    this: Ptr,
    look_at: u32,
    axis: u32,
    flip: u32,
) -> Ptr {
    e.call(NI_INTERPOLATOR_CONSTRUCT, &args![this]);
    let object = this.addr();
    e.mem.set_u32(object, NI_LOOK_AT_INTERPOLATOR_VTABLE);
    e.mem.set_u32(object.wrapping_add(LOOK_AT), look_at);
    e.mem.set_u16(object.wrapping_add(LOOK_AT_FLAGS), 0);
    let empty = e.mem.u32(EMPTY_FIXED_STRING);
    e.mem.set_u32(object.wrapping_add(LOOK_AT_NAME), empty);
    for k in 0..3 {
        let word = e.mem.u32(INITIAL_TRANSLATE + 4 * k);
        e.mem.set_u32(object.wrapping_add(TRANSFORM + 4 * k), word);
    }
    for k in 0..4 {
        let word = e.mem.u32(INITIAL_ROTATE + 4 * k);
        e.mem.set_u32(object.wrapping_add(ROTATE + 4 * k), word);
    }
    let scale = x87_quiet_bits(e.mem.u32(INITIAL_SCALE));
    e.mem.set_u32(object.wrapping_add(SCALE), scale);
    for slot in interpolator_slots(object) {
        e.mem.set_u32(slot, 0);
    }
    let at = object.wrapping_add(LOOK_AT_FLAGS);
    let flags = e.mem.u16(at);
    e.mem.set_u16(
        at,
        if flip & 0xff != 0 {
            flags | 1
        } else {
            flags & 0xfffe
        },
    );
    let flags = e.mem.u16(at);
    e.mem
        .set_u16(at, (flags & 0xfff9) | (axis.wrapping_mul(2) as u16));
    for slot in interpolator_slots(object) {
        let interpolator = e.mem.u32(slot);
        if interpolator != 0 {
            release(e, interpolator);
            e.mem.set_u32(slot, 0);
        }
    }
    this
}

// Translated from 00a4be20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiLookAtInterpolator` destructor body (the deleting
/// destructor `00a4c5c0` runs it). Stores the vtable, sets the look-at name to the handle at
/// `011f49e0`, releases the interpolators from the last, uncounts the name (unless it is the
/// empty string's) and ends in the `NiInterpolator` destructor (`00a57010`).
pub fn fn_00a4be20(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, NI_LOOK_AT_INTERPOLATOR_VTABLE);
    let handle = e.mem.u32(NO_LOOK_AT_NAME);
    assign_fixed_string(e, object.wrapping_add(LOOK_AT_NAME), handle);
    for slot in interpolator_slots(object).iter().rev() {
        let interpolator = e.mem.u32(*slot);
        if interpolator != 0 {
            release(e, interpolator);
        }
    }
    let name = e.mem.u32(object.wrapping_add(LOOK_AT_NAME));
    if name != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    e.call(NI_INTERPOLATOR_DESTRUCT, &args![this]);
}

/// The x87 values `FLD dword` gives for the floats at `address`, `address + 4` and
/// `address + 8`.
fn load3(e: &Engine, address: u32) -> [f64; 3] {
    [0u32, 4, 8].map(|k| x87_load32(e.mem.u32(address.wrapping_add(k))))
}

/// Stores the bits of three floats at `address`.
fn store3(e: &mut Engine, address: u32, bits: [u32; 3]) {
    for (k, word) in bits.iter().enumerate() {
        e.mem.set_u32(address.wrapping_add(4 * k as u32), *word);
    }
}

/// `FCHS` of a loaded float, stored back: its quiet bits with the sign flipped.
fn negated(bits: u32) -> u32 {
    x87_quiet_bits(bits) ^ 0x8000_0000
}

/// Sets the translation x (+0x18), the rotation's second word (+0x28) and the scale (+0x34)
/// to the float at `01015f5c`, as `Update` marks a transform invalid.
fn invalidate(e: &mut Engine, this: u32) {
    let invalid = x87_quiet_bits(e.mem.u32(INVALID_FLOAT));
    for offset in [TRANSFORM, 0x28, SCALE] {
        e.mem.set_u32(this.wrapping_add(offset), invalid);
    }
}

// Translated from 00a4bec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::Update` (Xbox PDB; the transform form: time, target, out
/// transform). Without a look-at object the out transform's translation x (+0), rotation x
/// (+0x10) and scale (+0x1c) and this one's matching fields become the float at `01015f5c`
/// (false). The position is the first interpolator's point3 update (slot `0x94`; false and
/// this transform invalidated when it fails, else it becomes this translation) or the
/// target's local translation (+0x58); with the target's parent (+0x18) it goes through the
/// parent's world rotation (+0x68, `004b4500`), scale (+0x98) and translation (+0x8c),
/// without one the rotation is the identity (`011a9448`). The direction to the look-at
/// object's world translation (+0x8c), when its squared length is at least the float at
/// `01017d00`, is unitized (`004a0c10`); the up vector is `011a9478` when its z is near +1 or
/// -1 (the bounds at `01098c00`/`01098bf8` and `01098bf0`/`01098be8`), `011a9484` otherwise,
/// made perpendicular (minus the direction times its y or z) and unitized; the direction is
/// negated unless flag bit 0 is set; with the cross product of up and direction the axis
/// (flag bits 1-2) chooses the columns (0: direction, up, -cross; 1: cross, direction, -up;
/// 2: cross, up, direction; 3 leaves the rotation stack words as they are, the frame's
/// memory here). The rotation is the parent's transposed times it (`00a588a0`), rolled when
/// the second interpolator's float update gives an angle (a z rotation of minus the angle,
/// `004a0c90`, applied after, `0043f8d0`; false and invalid when that fails). A short
/// direction gives the identity rotation without the roll. The rotation becomes this
/// rotation (`NiQuaternion::FromRotation`), the third interpolator's float update this scale
/// (false and invalid when it fails). When the scale, rotation x and translation x all equal
/// the double at `010241b0`, the out transform is marked invalid (false); otherwise this
/// transform is copied out, the time becomes the last time and the answer is true.
pub fn ni_look_at_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: u32,
    out: Ptr,
) -> bool {
    let object = this.addr();
    let out = out.addr();
    let look_at = e.mem.u32(object.wrapping_add(LOOK_AT));
    if look_at == 0 {
        let invalid = x87_quiet_bits(e.mem.u32(INVALID_FLOAT));
        for offset in [0u32, 0x10, 0x1c] {
            e.mem.set_u32(out.wrapping_add(offset), invalid);
        }
        invalidate(e, object);
        return false;
    }
    let time_bits = x87_quiet(time).to_bits();
    // Frame (offsets from the stack pointer after the 0xe0 bytes of locals): the scale and
    // update floats +0, the look-at position (then the up vector's offset) +4, the world
    // position (then the cross product) +0x10, the direction +0x1c, the transformed position
    // (then the up vector) +0x28, the rotation +0x34, the position +0x58, the quaternion
    // +0x64, the parent's rotation +0x74 and two matrix results +0x98 and +0xbc.
    e.with_stack(0xe0, |e, frame| {
        let base = frame.addr();
        let scale_word = base;
        let look_at_position = base + 4;
        let world = base + 0x10;
        let direction = base + 0x1c;
        let up = base + 0x28;
        let rotation = base + 0x34;
        let position = base + 0x58;
        let quaternion = base + 0x64;
        let parent_rotation = base + 0x74;
        let product = base + 0x98;
        let rolled = base + 0xbc;
        for k in 0..3 {
            let word = e.mem.u32(look_at.wrapping_add(0x8c + 4 * k));
            e.mem.set_u32(look_at_position + 4 * k, word);
        }
        let parent = e.mem.u32(target.wrapping_add(0x18));
        let first = e.mem.u32(object.wrapping_add(INTERPOLATORS));
        if first != 0 {
            let ok = e
                .vcall(
                    first,
                    SLOT_UPDATE_POINT3,
                    &args![time_bits, target, position],
                )
                .bool();
            if !ok {
                invalidate(e, object);
                return false;
            }
            for k in 0..3 {
                let word = e.mem.u32(position + 4 * k);
                e.mem.set_u32(object.wrapping_add(TRANSFORM + 4 * k), word);
            }
        } else {
            for k in 0..3 {
                let word = e.mem.u32(target.wrapping_add(0x58 + 4 * k));
                e.mem.set_u32(position + 4 * k, word);
            }
        }
        if parent != 0 {
            for k in 0..9 {
                let word = e.mem.u32(parent.wrapping_add(0x68 + 4 * k));
                e.mem.set_u32(parent_rotation + 4 * k, word);
            }
            let scale = x87_quiet_bits(e.mem.u32(parent.wrapping_add(0x98)));
            e.mem.set_u32(scale_word, scale);
            let turned = e
                .call(MATRIX_TIMES_POINT, &args![parent_rotation, up, position])
                .u32();
            let s = x87_load32(e.mem.u32(scale_word));
            let tx = x87_load32(e.mem.u32(turned));
            e.mem.set_u32(world, x87_store32(x87_mul(tx, s)));
            let ty = x87_load32(e.mem.u32(turned.wrapping_add(4)));
            e.mem.set_u32(world + 4, x87_store32(x87_mul(ty, s)));
            let tz = x87_m32(e.mem.u32(turned.wrapping_add(8)));
            e.mem.set_u32(world + 8, x87_store32(x87_mul(s, tz)));
            for k in 0..3 {
                let translate = x87_load32(e.mem.u32(parent.wrapping_add(0x8c + 4 * k)));
                let scaled = x87_m32(e.mem.u32(world + 4 * k));
                let sum = x87_store32(x87_add(translate, scaled));
                e.mem.set_u32(up + 4 * k, sum);
                e.mem.set_u32(world + 4 * k, sum);
            }
        } else {
            for k in 0..9 {
                let word = e.mem.u32(IDENTITY_MATRIX + 4 * k);
                e.mem.set_u32(parent_rotation + 4 * k, word);
            }
            for k in 0..3 {
                let word = e.mem.u32(position + 4 * k);
                e.mem.set_u32(world + 4 * k, word);
            }
        }
        for k in 0..3 {
            let to = x87_load32(e.mem.u32(look_at_position + 4 * k));
            let from = x87_m32(e.mem.u32(world + 4 * k));
            e.mem
                .set_u32(direction + 4 * k, x87_store32(x87_sub(to, from)));
        }
        let [dx, dy, dz] = load3(e, direction);
        let sum = x87_add(x87_mul(dx, dx), x87_mul(dy, dy));
        let length = x87_store32(x87_add(sum, x87_mul(dz, dz)));
        e.mem.set_u32(scale_word, length);
        let length = x87_load32(length);
        let threshold = x87_m32(e.mem.u32(MIN_DISTANCE_SQUARED));
        if length < threshold {
            let identity = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
            for (k, value) in identity.iter().enumerate() {
                e.mem.set_f32(rotation + 4 * k as u32, *value);
            }
        } else {
            e.call(POINT_UNITIZE, &args![direction]);
            let z = x87_load32(e.mem.u32(direction + 8));
            let pole = (z < x87_m32(e.mem.u32(POLE_ABOVE))
                && z > f64::from_bits(e.mem.u64(POLE_BELOW)))
                || (x87_m32(e.mem.u32(ANTIPOLE_BELOW)) < z
                    && z < f64::from_bits(e.mem.u64(ANTIPOLE_ABOVE)));
            let (factor, source) = if pole {
                (e.mem.u32(direction + 4), POLE_UP)
            } else {
                (e.mem.u32(direction + 8), UP)
            };
            e.mem.set_u32(scale_word, x87_quiet_bits(factor));
            for k in 0..3 {
                let word = e.mem.u32(source + 4 * k);
                e.mem.set_u32(up + 4 * k, word);
            }
            let [dx, dy, dz] = load3(e, direction);
            let factor = x87_load32(e.mem.u32(scale_word));
            let along = [
                x87_store32(x87_mul(dx, factor)),
                x87_store32(x87_mul(dy, factor)),
                x87_store32(x87_mul(dz, factor)),
            ];
            store3(e, look_at_position, along);
            for k in 0..3 {
                let u = x87_load32(e.mem.u32(up + 4 * k));
                let a = x87_m32(e.mem.u32(look_at_position + 4 * k));
                e.mem.set_u32(up + 4 * k, x87_store32(x87_sub(u, a)));
            }
            e.call(POINT_UNITIZE, &args![up]);
            if e.mem.u8(object.wrapping_add(LOOK_AT_FLAGS)) & 1 == 0 {
                for k in 0..3 {
                    let flipped = negated(e.mem.u32(direction + 4 * k));
                    e.mem.set_u32(look_at_position + 4 * k, flipped);
                    e.mem.set_u32(direction + 4 * k, flipped);
                }
            }
            let axis = (u32::from(e.mem.u16(object.wrapping_add(LOOK_AT_FLAGS))) >> 1) & 3;
            let dz = x87_load32(e.mem.u32(direction + 8));
            let dy = x87_load32(e.mem.u32(direction + 4));
            let [ux, uy, uz] = load3(e, up);
            let dx_operand = x87_m32(e.mem.u32(direction));
            let cross = [
                x87_store32(x87_sub(x87_mul(uy, dz), x87_mul(uz, dy))),
                x87_store32(x87_sub(x87_mul(uz, dx_operand), x87_mul(ux, dz))),
                x87_store32(x87_sub(x87_mul(ux, dy), x87_mul(uy, dx_operand))),
            ];
            store3(e, world, cross);
            let quiet = |e: &Engine, address: u32| -> [u32; 3] {
                [0u32, 4, 8].map(|k| x87_quiet_bits(e.mem.u32(address + k)))
            };
            let columns = match axis {
                0 => {
                    let d = quiet(e, direction);
                    let u = quiet(e, up);
                    let minus = quiet(e, world).map(negated);
                    store3(e, look_at_position, minus);
                    Some([d, u, minus])
                }
                1 => {
                    let c = quiet(e, world);
                    let d = quiet(e, direction);
                    let minus = quiet(e, up).map(negated);
                    store3(e, look_at_position, minus);
                    Some([c, d, minus])
                }
                2 => Some([quiet(e, world), quiet(e, up), quiet(e, direction)]),
                _ => None,
            };
            if let Some(columns) = columns {
                for (column, values) in columns.iter().enumerate() {
                    for (row, bits) in values.iter().enumerate() {
                        let at = rotation + 4 * (3 * row + column) as u32;
                        e.mem.set_u32(at, x87_quiet_bits(*bits));
                    }
                }
            }
            let result = e
                .call(
                    MATRIX_TRANSPOSE_TIMES,
                    &args![parent_rotation, product, rotation],
                )
                .u32();
            for k in 0..9 {
                let word = e.mem.u32(result.wrapping_add(4 * k));
                e.mem.set_u32(rotation + 4 * k, word);
            }
            let roll = e.mem.u32(object.wrapping_add(INTERPOLATORS + 4));
            if roll != 0 {
                let ok = e
                    .vcall(
                        roll,
                        SLOT_UPDATE_FLOAT,
                        &args![time_bits, target, scale_word],
                    )
                    .bool();
                if !ok {
                    invalidate(e, object);
                    return false;
                }
                let angle = negated(e.mem.u32(scale_word));
                e.call(MATRIX_MAKE_Z_ROTATION, &args![product, angle]);
                let result = e
                    .call(MATRIX_TIMES_MATRIX, &args![rotation, rolled, product])
                    .u32();
                for k in 0..9 {
                    let word = e.mem.u32(result.wrapping_add(4 * k));
                    e.mem.set_u32(rotation + 4 * k, word);
                }
            }
        }
        e.call(QUATERNION_FROM_ROTATION, &args![quaternion, rotation]);
        for k in 0..4 {
            let word = e.mem.u32(quaternion + 4 * k);
            e.mem.set_u32(object.wrapping_add(ROTATE + 4 * k), word);
        }
        let scaler = e.mem.u32(object.wrapping_add(INTERPOLATORS + 8));
        if scaler != 0 {
            let ok = e
                .vcall(
                    scaler,
                    SLOT_UPDATE_FLOAT,
                    &args![time_bits, target, scale_word],
                )
                .bool();
            if !ok {
                invalidate(e, object);
                return false;
            }
            let scale = x87_quiet_bits(e.mem.u32(scale_word));
            e.mem.set_u32(object.wrapping_add(SCALE), scale);
        }
        let invalid = f64::from_bits(e.mem.u64(INVALID_DOUBLE));
        if x87_load32(e.mem.u32(object.wrapping_add(SCALE))) == invalid
            && x87_load32(e.mem.u32(object.wrapping_add(0x28))) == invalid
            && x87_load32(e.mem.u32(object.wrapping_add(TRANSFORM))) == invalid
        {
            let value = x87_quiet_bits(e.mem.u32(INVALID_FLOAT));
            for offset in [0u32, 0x10, 0x1c] {
                e.mem.set_u32(out.wrapping_add(offset), value);
            }
            return false;
        }
        for k in 0..8 {
            let word = e.mem.u32(object.wrapping_add(TRANSFORM + 4 * k));
            e.mem.set_u32(out.wrapping_add(4 * k), word);
        }
        e.mem.set_u32(object.wrapping_add(LAST_TIME), time_bits);
        true
    })
}

// Translated from 00a4c4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::CopyMembers` (Xbox PDB): `NiKeyBasedInterpolator::CopyMembers`,
/// the flags (bit 0 and bits 1-2 as this one's), the look-at object, the look-at name set to
/// the handle at `011f49e0`, the transform (eight words) and a clone of each interpolator
/// (`NiObject::Clone`, counted into the destination; the old one released).
pub fn ni_look_at_interpolator_copy_members(
    e: &mut Engine,
    this: Ptr,
    destination: Ptr,
    cloning: u32,
) {
    e.call(KEY_BASED_COPY_MEMBERS, &args![this, destination, cloning]);
    let source = this.addr();
    let copy = destination.addr();
    let flags_at = copy.wrapping_add(LOOK_AT_FLAGS);
    let flags = e.mem.u16(source.wrapping_add(LOOK_AT_FLAGS));
    e.mem.set_u16(flags_at, flags);
    let flags = if e.mem.u8(source.wrapping_add(LOOK_AT_FLAGS)) & 1 != 0 {
        flags | 1
    } else {
        flags & 0xfffe
    };
    e.mem.set_u16(flags_at, flags);
    let change = (flags ^ e.mem.u16(source.wrapping_add(LOOK_AT_FLAGS))) & 6;
    e.mem.set_u16(flags_at, e.mem.u16(flags_at) ^ change);
    let look_at = e.mem.u32(source.wrapping_add(LOOK_AT));
    e.mem.set_u32(copy.wrapping_add(LOOK_AT), look_at);
    let handle = e.mem.u32(NO_LOOK_AT_NAME);
    assign_fixed_string(e, copy.wrapping_add(LOOK_AT_NAME), handle);
    for k in 0..8 {
        let word = e.mem.u32(source.wrapping_add(TRANSFORM + 4 * k));
        e.mem.set_u32(copy.wrapping_add(TRANSFORM + 4 * k), word);
    }
    for (mine, slot) in interpolator_slots(source)
        .iter()
        .zip(interpolator_slots(copy))
    {
        let interpolator = e.mem.u32(*mine);
        if interpolator == 0 {
            continue;
        }
        let clone = e.call(NI_OBJECT_CLONE, &args![interpolator]).u32();
        assign_pointer(e, slot, clone);
    }
}

// Translated from 00a4c5a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::CreateObject` (registered as the `"NiLookAtInterpolator"` loader at
/// `00a2bde0`; Ghidra has no function here): allocates 0x44 bytes and, when that succeeds,
/// constructs an interpolator in it with no look-at object, axis 0 and no flip. Returns it,
/// or 0.
pub fn ni_look_at_interpolator_create_object(e: &mut Engine) -> u32 {
    let block = e
        .call(NI_ALLOCATE, &args![NI_LOOK_AT_INTERPOLATOR_SIZE])
        .u32();
    if block == 0 {
        return 0;
    }
    ni_look_at_interpolator_construct(e, Ptr::new(block), 0, 0, 0).addr()
}

// Translated from 00a4c5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::CreateClone` (Xbox PDB): a new interpolator (0x44 bytes,
/// constructed with no look-at object, axis 0 and no flip) and the member copy into it; when
/// the allocation fails the member copy still runs, into address 0, and 0 is returned.
pub fn ni_look_at_interpolator_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> u32 {
    let block = e
        .call(NI_ALLOCATE, &args![NI_LOOK_AT_INTERPOLATOR_SIZE])
        .u32();
    if block == 0 {
        ni_look_at_interpolator_copy_members(e, this, Ptr::new(0), cloning);
        return 0;
    }
    let clone = ni_look_at_interpolator_construct(e, Ptr::new(block), 0, 0, 0);
    ni_look_at_interpolator_copy_members(e, this, clone, cloning);
    clone.addr()
}

/// One transfer of `size` bytes in one component through the binary stream at
/// `stream + side` with its function at `function`: the word at `slot` is set to `size` and
/// passed as the component size; the position advances by the bytes moved.
fn stream_transfer(
    e: &mut Engine,
    stream: u32,
    side: u32,
    function: u32,
    buffer: u32,
    size: u32,
    slot: u32,
) {
    let binary = e.mem.u32(stream.wrapping_add(side));
    let transfer = e.mem.u32(binary.wrapping_add(function));
    e.mem.set_u32(slot, size);
    let bytes = e
        .call(transfer, &args![binary, buffer, size, slot, 1u32])
        .u32();
    let position = e.mem.u32(binary.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        binary.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

// Translated from 00a4c630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::LoadBinary` (Xbox PDB): `NiInterpolator::LoadBinary`, the flags (2
/// bytes; the argument's own stack word is the component size word), the look-at object's
/// link (`ResolveLinkID`, not counted), the name (a C string made a fixed string before file
/// version `0x14010001`, a fixed string from then on), the transform
/// (`NiQuatTransform::LoadBinary`) and the three interpolators' links (counted; the old ones
/// released).
pub fn ni_look_at_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_INTERPOLATOR_LOAD_BINARY, &args![this, stream]);
    let object = this.addr();
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stream);
        let flags = object.wrapping_add(LOOK_AT_FLAGS);
        let input = (STREAM_INPUT, BINARY_STREAM_READ);
        stream_transfer(e, stream, input.0, input.1, flags, 2, slot.addr());
    });
    let look_at = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
    e.mem.set_u32(object.wrapping_add(LOOK_AT), look_at);
    let name = object.wrapping_add(LOOK_AT_NAME);
    if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) < 0x1401_0001 {
        e.call(LOAD_CSTRING_AS_FIXED_STRING, &args![stream, name]);
    } else {
        e.call(LOAD_FIXED_STRING, &args![stream, name]);
    }
    e.call(
        QUAT_TRANSFORM_LOAD_BINARY,
        &args![object.wrapping_add(TRANSFORM), stream],
    );
    for slot in interpolator_slots(object) {
        let interpolator = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
        assign_pointer(e, slot, interpolator);
    }
}

// Translated from 00a4c700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::SaveBinary` (Xbox PDB): the base's `SaveBinary` (`00a724d0`), the
/// flags (2 bytes; the argument's own stack word is the component size word), the look-at
/// object's link ID (the stream's slot `0x30`), the name (`SaveFixedString`), the transform
/// (`NiQuatTransform::SaveBinary`) and the three interpolators' link IDs.
pub fn ni_look_at_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let object = this.addr();
    let stream = stream.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), stream);
        let flags = object.wrapping_add(LOOK_AT_FLAGS);
        let output = (STREAM_OUTPUT, BINARY_STREAM_WRITE);
        stream_transfer(e, stream, output.0, output.1, flags, 2, slot.addr());
    });
    let look_at = e.mem.u32(object.wrapping_add(LOOK_AT));
    e.vcall(stream, SLOT_SAVE_LINK_ID, &args![look_at]);
    e.call(
        SAVE_FIXED_STRING,
        &args![stream, object.wrapping_add(LOOK_AT_NAME)],
    );
    e.call(
        QUAT_TRANSFORM_SAVE_BINARY,
        &args![object.wrapping_add(TRANSFORM), stream],
    );
    for slot in interpolator_slots(object) {
        let interpolator = e.mem.u32(slot);
        e.vcall(stream, SLOT_SAVE_LINK_ID, &args![interpolator]);
    }
}

/// The viewer-string array append the compiler inlined with the string in the stack word
/// `slot`: grows the array (16-bit size +0xA, capacity +8, grow-by +0xE) when full, then
/// `SetAt` at the old size with the slot's address.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a4c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtInterpolator::GetViewerStrings` (Xbox PDB): `NiInterpolator::GetViewerStrings`,
/// the class name (`011f3f90`) and `"m_pkLookAt"` with the look-at object (held in the
/// argument's own stack word), the transform's strings (`NiQuatTransform::GetViewerStrings`)
/// and each interpolator's (slot `0x60`).
pub fn ni_look_at_interpolator_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_INTERPOLATOR_GET_VIEWER_STRINGS, &args![this, strings]);
    let object = this.addr();
    let strings = strings.addr();
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, strings);
        let class_name = e.mem.u32(NI_LOOK_AT_INTERPOLATOR_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let look_at = e.mem.u32(object.wrapping_add(LOOK_AT));
        let text = e
            .call(NI_GET_VIEWER_STRING_POINTER, &args![TEXT_LOOK_AT, look_at])
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
    });
    e.call(
        QUAT_TRANSFORM_GET_VIEWER_STRINGS,
        &args![object.wrapping_add(TRANSFORM), strings],
    );
    for slot in interpolator_slots(object) {
        let interpolator = e.mem.u32(slot);
        if interpolator != 0 {
            e.vcall(interpolator, SLOT_GET_VIEWER_STRINGS, &args![strings]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4be10, ni_look_at_interpolator_get_rtti() -> Ptr),
        entry!(0x00a4c5c0, ni_look_at_interpolator_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00a4_b8d0,
            ni_look_at_interpolator_resolve_dependencies(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_b900, ni_look_at_interpolator_collapse(Ptr)),
        entry!(
            0x00a4_b930,
            ni_look_at_interpolator_get_active_time_range(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00a4_b9f0,
            ni_look_at_interpolator_guarantee_time_range(Ptr, f32, f32)
        ),
        entry!(
            0x00a4_ba30,
            ni_look_at_interpolator_get_sequence_interpolator(Ptr, f32, f32) -> u32
        ),
        entry!(
            0x00a4_bae0,
            ni_look_at_interpolator_set_up_dependencies(Ptr) -> bool
        ),
        entry!(0x00a4_bb80, ni_look_at_interpolator_process_clone(Ptr, Ptr)),
        entry!(
            0x00a4_bbe0,
            ni_look_at_interpolator_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_bc40, ni_look_at_interpolator_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_bcf0,
            ni_look_at_interpolator_construct(Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(0x00a4_be20, fn_00a4be20(Ptr)),
        entry!(
            0x00a4_bec0,
            ni_look_at_interpolator_update(Ptr, f32, u32, Ptr) -> bool
        ),
        entry!(
            0x00a4_c4b0,
            ni_look_at_interpolator_copy_members(Ptr, Ptr, u32)
        ),
        entry!(0x00a4_c5a0, ni_look_at_interpolator_create_object() -> u32),
        entry!(
            0x00a4_c5e0,
            ni_look_at_interpolator_create_clone(Ptr, u32) -> u32
        ),
        entry!(0x00a4_c630, ni_look_at_interpolator_load_binary(Ptr, Ptr)),
        entry!(0x00a4_c700, ni_look_at_interpolator_save_binary(Ptr, Ptr)),
        entry!(
            0x00a4_c780,
            ni_look_at_interpolator_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_look_at_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4be10, &args![]).u32(),
            NI_LOOK_AT_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn ni_look_at_interpolator_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_be20, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a4c5c0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a4c5c0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x44]]);
        assert_eq!(calls_to(&e, 0x00a4_be20), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Matrices = Rc<RefCell<Vec<[f32; 9]>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    fn answers(e: &mut Engine, address: u32, value: &Rc<Cell<u32>>) {
        let value = value.clone();
        e.register_double(address, move |_, _| ret(value.get()));
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object whose vtable has `slots` set to the given targets.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The interlocked import slots, logged and counting in memory.
    fn interlocked(e: &mut Engine, log: &Log) {
        for (address, step) in [
            (INTERLOCKED_INCREMENT, 1u32),
            (INTERLOCKED_DECREMENT, u32::MAX),
        ] {
            let log = log.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let count = e.mem.u32(a[0]).wrapping_add(step);
                e.mem.set_u32(a[0], count);
                ret(count)
            });
        }
    }

    /// The empty string's handle `0x09000008`, the handle at `011f49e0` `0x0a000008`, and
    /// the count words of the handles these tests use (`0x09..0x0c000000`).
    fn empty_string(e: &mut Engine) {
        e.map(EMPTY_FIXED_STRING, 4);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0900_0008);
        e.map(NO_LOOK_AT_NAME, 4);
        e.mem.set_u32(NO_LOOK_AT_NAME, 0x0a00_0008);
        for page in [0x0900_0000u32, 0x0a00_0000, 0x0b00_0000, 0x0c00_0000] {
            e.map(page, 0x10);
        }
    }

    #[test]
    fn resolve_dependencies_finds_the_named_object() {
        let mut e = Engine::new();
        let found = Rc::new(Cell::new(0u32));
        answers(&mut e, 0x00ee_6000, &found);
        let palette = object(&mut e, 8, &[(SLOT_PALETTE_GET, 0x00ee_6000)]);
        let this = e.mem.alloc(0x44);
        assert!(!e.call(0x00a4_b8d0, &args![this, palette]).bool());
        found.set(0x5000);
        assert!(e.call(0x00a4_b8d0, &args![this, palette]).bool());
        assert_eq!(e.mem.u32(this + LOOK_AT), 0x5000);
        found.set(0);
        assert!(e.call(0x00a4_b8d0, &args![this, palette]).bool());
    }

    #[test]
    fn collapse_and_guarantee_time_range_reach_each_interpolator() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, 0x00ee_6100, 0);
        logged(&mut e, &log, 0x00ee_6110, 0);
        let slots = [
            (SLOT_COLLAPSE, 0x00ee_6100),
            (SLOT_GUARANTEE_TIME_RANGE, 0x00ee_6110),
        ];
        let a = object(&mut e, 8, &slots);
        let b = object(&mut e, 8, &slots);
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + INTERPOLATORS, a);
        e.mem.set_u32(this + INTERPOLATORS + 8, b);
        e.call(0x00a4_b900, &args![this]);
        assert_eq!(calls(&log, 0x00ee_6100), [vec![a], vec![b]]);
        e.call(0x00a4_b9f0, &args![this, 1.0f32, 0x7f80_0001u32]);
        assert_eq!(
            calls(&log, 0x00ee_6110),
            [
                vec![a, 1.0f32.to_bits(), 0x7fc0_0001],
                vec![b, 1.0f32.to_bits(), 0x7fc0_0001]
            ]
        );
    }

    fn range_constants(e: &mut Engine) {
        for (address, size) in [
            (RANGE_START_FLOAT, 4),
            (INVALID_FLOAT, 4),
            (RANGE_START_DOUBLE, 8),
            (INVALID_DOUBLE, 8),
        ] {
            e.map(address, size);
        }
        e.mem.set_f32(RANGE_START_FLOAT, f32::MAX);
        e.mem.set_f32(INVALID_FLOAT, -f32::MAX);
        e.mem.set_f64(RANGE_START_DOUBLE, f64::from(f32::MAX));
        e.mem.set_f64(INVALID_DOUBLE, f64::from(-f32::MAX));
    }

    #[test]
    fn get_active_time_range_spans_the_non_empty_ranges() {
        let mut e = Engine::new();
        range_constants(&mut e);
        let ranges = [(2.0f32, 2.0f32), (1.0, 3.0), (0.5, 1.5)];
        let this = e.mem.alloc(0x44);
        for (k, (start, end)) in ranges.iter().enumerate() {
            let target = 0x00ee_6200 + 0x10 * k as u32;
            let (start, end) = (*start, *end);
            e.register_double(target, move |e, a| {
                e.mem.set_f32(a[1], start);
                e.mem.set_f32(a[2], end);
                Ret::default()
            });
            let interpolator = object(&mut e, 8, &[(SLOT_GET_ACTIVE_TIME_RANGE, target)]);
            e.mem
                .set_u32(this + INTERPOLATORS + 4 * k as u32, interpolator);
        }
        let range = e.mem.alloc(8);
        e.call(0x00a4_b930, &args![this, range, range + 4]);
        assert_eq!(e.mem.f32(range), 0.5);
        assert_eq!(e.mem.f32(range + 4), 3.0);
        // Only an empty range: the starting values become 0.0 to 0.0.
        e.mem.set_u32(this + INTERPOLATORS + 4, 0);
        e.mem.set_u32(this + INTERPOLATORS + 8, 0);
        e.call(0x00a4_b930, &args![this, range, range + 4]);
        assert_eq!(e.mem.u32(range), 0);
        assert_eq!(e.mem.u32(range + 4), 0);
    }

    #[test]
    fn get_sequence_interpolator_fills_the_clone() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        let clone = e.mem.alloc(0x44);
        logged(
            &mut e,
            &log,
            NI_INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR,
            clone,
        );
        let sequence = object(&mut e, 8, &[]);
        logged(&mut e, &log, 0x00ee_6300, sequence);
        logged(&mut e, &log, 0x00ee_6310, 0);
        let interpolator = object(&mut e, 8, &[(SLOT_GET_SEQUENCE_INTERPOLATOR, 0x00ee_6300)]);
        let old = object(&mut e, 8, &[(SLOT_DELETE_THIS, 0x00ee_6310)]);
        e.mem.set_u32(old + 4, 1);
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + INTERPOLATORS + 4, interpolator);
        e.mem.set_u32(clone + INTERPOLATORS + 4, old);
        let result = e.call(0x00a4_ba30, &args![this, 1.0f32, 2.0f32]).u32();
        assert_eq!(result, clone);
        let (one, two) = (1.0f32.to_bits(), 2.0f32.to_bits());
        assert_eq!(
            calls(&log, NI_INTERPOLATOR_GET_SEQUENCE_INTERPOLATOR),
            [vec![this, one, two]]
        );
        assert_eq!(calls(&log, 0x00ee_6300), [vec![interpolator, one, two]]);
        assert_eq!(e.mem.u32(clone + INTERPOLATORS + 4), sequence);
        assert_eq!(e.mem.u32(sequence + 4), 1);
        assert_eq!(calls(&log, 0x00ee_6310), [vec![old]]);
    }

    #[test]
    fn set_up_dependencies_names_the_look_at_object() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        empty_string(&mut e);
        logged(&mut e, &log, GET_FIXED_STRING, 0x0b00_0008);
        let look_at = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + LOOK_AT, look_at);
        e.mem.set_u32(this + LOOK_AT_NAME, 0x0900_0008);
        // No name on the object: the name goes to the handle at 011f49e0.
        assert!(!e.call(0x00a4_bae0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + LOOK_AT_NAME), 0x0a00_0008);
        assert_eq!(e.mem.u32(this + LOOK_AT), look_at);
        e.mem.set_u32(look_at + 8, 0x6000);
        assert!(e.call(0x00a4_bae0, &args![this]).bool());
        assert_eq!(e.mem.u32(this + LOOK_AT_NAME), 0x0b00_0008);
        assert_eq!(e.mem.u32(this + LOOK_AT), 0);
        assert_eq!(calls(&log, GET_FIXED_STRING), [vec![0x6000]]);
        assert_eq!(
            calls(&log, INTERLOCKED_INCREMENT),
            [vec![0x0a00_0000], vec![0x0b00_0000]]
        );
        assert_eq!(
            calls(&log, INTERLOCKED_DECREMENT),
            [vec![0x0a00_0000], vec![0x0b00_0000]]
        );
    }

    #[test]
    fn process_clone_maps_the_look_at_object() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, NI_OBJECT_PROCESS_CLONE, 0);
        let clone = e.mem.alloc(0x44);
        let has_look_at = Rc::new(Cell::new(true));
        let answer = has_look_at.clone();
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            assert_eq!(e.mem.u32(a[2]), if a[1] == 0x7000 { clone } else { 0 });
            if a[1] == 0x7000 {
                if answer.get() {
                    e.mem.set_u32(a[2], 0x7100);
                }
                return ret(u32::from(answer.get()));
            }
            e.mem.set_u32(a[2], clone);
            ret(1)
        });
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + LOOK_AT, 0x7000);
        let cloning = e.mem.alloc(8);
        e.mem.set_u32(cloning, 0x8000);
        e.call(0x00a4_bb80, &args![this, cloning]);
        assert_eq!(e.mem.u32(clone + LOOK_AT), 0x7100);
        has_look_at.set(false);
        e.call(0x00a4_bb80, &args![this, cloning]);
        assert_eq!(e.mem.u32(clone + LOOK_AT), 0x7000);
        assert_eq!(calls(&log, NI_OBJECT_PROCESS_CLONE).len(), 2);
    }

    #[test]
    fn register_streamables_stops_at_a_failing_interpolator() {
        let mut e = Engine::new();
        let log = Log::default();
        let base = Rc::new(Cell::new(1u32));
        answers(&mut e, BASE_REGISTER_STREAMABLES, &base);
        let registered = Rc::new(Cell::new(1u32));
        answers(&mut e, 0x00ee_6400, &registered);
        logged(&mut e, &log, 0x00ee_6410, 1);
        let stream = object(&mut e, 8, &[(SLOT_REGISTER_FIXED_STRING, 0x00ee_6410)]);
        let interpolator = object(&mut e, 8, &[(SLOT_REGISTER_STREAMABLES, 0x00ee_6400)]);
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + INTERPOLATORS + 8, interpolator);
        assert!(e.call(0x00a4_bbe0, &args![this, stream]).bool());
        assert_eq!(
            calls(&log, 0x00ee_6410),
            [vec![stream, this + LOOK_AT_NAME]]
        );
        registered.set(0);
        assert!(!e.call(0x00a4_bbe0, &args![this, stream]).bool());
        base.set(0);
        registered.set(1);
        assert!(!e.call(0x00a4_bbe0, &args![this, stream]).bool());
        assert_eq!(calls(&log, 0x00ee_6410).len(), 1);
    }

    #[test]
    fn is_equal_compares_flags_look_at_transform_and_interpolators() {
        let mut e = Engine::new();
        let base = Rc::new(Cell::new(1u32));
        answers(&mut e, BASE_IS_EQUAL, &base);
        let transform = Rc::new(Cell::new(1u32));
        answers(&mut e, QUAT_TRANSFORM_EQUALS, &transform);
        let equal = Rc::new(Cell::new(1u32));
        answers(&mut e, 0x00ee_6500, &equal);
        let a = object(&mut e, 8, &[(SLOT_IS_EQUAL, 0x00ee_6500)]);
        let b = object(&mut e, 8, &[(SLOT_IS_EQUAL, 0x00ee_6500)]);
        let this = e.mem.alloc(0x44);
        let other = e.mem.alloc(0x44);
        assert!(e.call(0x00a4_bc40, &args![this, other]).bool());
        e.mem.set_u32(this + LOOK_AT, a);
        assert!(!e.call(0x00a4_bc40, &args![this, other]).bool());
        e.mem.set_u32(other + LOOK_AT, b);
        assert!(e.call(0x00a4_bc40, &args![this, other]).bool());
        equal.set(0);
        assert!(!e.call(0x00a4_bc40, &args![this, other]).bool());
        equal.set(1);
        e.mem.set_u32(other + INTERPOLATORS + 4, b);
        assert!(!e.call(0x00a4_bc40, &args![this, other]).bool());
        e.mem.set_u32(this + INTERPOLATORS + 4, a);
        assert!(e.call(0x00a4_bc40, &args![this, other]).bool());
        transform.set(0);
        assert!(!e.call(0x00a4_bc40, &args![this, other]).bool());
        transform.set(1);
        e.mem.set_u16(other + LOOK_AT_FLAGS, 2);
        assert!(!e.call(0x00a4_bc40, &args![this, other]).bool());
        e.mem.set_u16(other + LOOK_AT_FLAGS, 0);
        base.set(0);
        assert!(!e.call(0x00a4_bc40, &args![this, other]).bool());
    }

    fn constructor_globals(e: &mut Engine) {
        empty_string(e);
        e.map(INITIAL_TRANSLATE, 12);
        e.map(INITIAL_ROTATE, 16);
        e.map(INITIAL_SCALE, 4);
        for k in 0..3 {
            e.mem.set_u32(INITIAL_TRANSLATE + 4 * k, 0x100 + k);
        }
        for k in 0..4 {
            e.mem.set_u32(INITIAL_ROTATE + 4 * k, 0x200 + k);
        }
        e.mem.set_f32(INITIAL_SCALE, -f32::MAX);
    }

    #[test]
    fn constructor_sets_the_members_and_flags() {
        let mut e = Engine::new();
        let log = Log::default();
        constructor_globals(&mut e);
        logged(&mut e, &log, NI_INTERPOLATOR_CONSTRUCT, 0);
        let this = e.mem.alloc(0x44);
        e.mem.write(this, &[0xaa; 0x44]);
        let result = e.call(0x00a4_bcf0, &args![this, 0x5000u32, 0x8003u32, 0x100u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), NI_LOOK_AT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + LOOK_AT), 0x5000);
        assert_eq!(e.mem.u32(this + LOOK_AT_NAME), 0x0900_0008);
        // The flip word's low byte is 0; the axis word times 2 keeps its low 16 bits.
        assert_eq!(e.mem.u16(this + LOOK_AT_FLAGS), 0x0006);
        assert_eq!(e.mem.u32(this + TRANSFORM + 8), 0x102);
        assert_eq!(e.mem.u32(this + ROTATE + 12), 0x203);
        assert_eq!(e.mem.f32(this + SCALE), -f32::MAX);
        assert_eq!(e.mem.bytes(this + INTERPOLATORS, 12), vec![0; 12]);
        e.call(0x00a4_bcf0, &args![this, 0u32, 1u32, 1u32]);
        assert_eq!(e.mem.u16(this + LOOK_AT_FLAGS), 0x0003);
        assert_eq!(
            calls(&log, NI_INTERPOLATOR_CONSTRUCT),
            [vec![this], vec![this]]
        );
    }

    #[test]
    fn destructor_body_resets_the_name_and_releases_from_the_last() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        empty_string(&mut e);
        logged(&mut e, &log, NI_INTERPOLATOR_DESTRUCT, 0);
        logged(&mut e, &log, 0x00ee_6600, 0);
        let a = object(&mut e, 8, &[(SLOT_DELETE_THIS, 0x00ee_6600)]);
        let b = object(&mut e, 8, &[(SLOT_DELETE_THIS, 0x00ee_6600)]);
        e.mem.set_u32(a + 4, 1);
        e.mem.set_u32(b + 4, 1);
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + LOOK_AT_NAME, 0x0c00_0008);
        e.mem.set_u32(this + INTERPOLATORS, a);
        e.mem.set_u32(this + INTERPOLATORS + 8, b);
        e.call(0x00a4_be20, &args![this]);
        assert_eq!(e.mem.u32(this), NI_LOOK_AT_INTERPOLATOR_VTABLE);
        assert_eq!(e.mem.u32(this + LOOK_AT_NAME), 0x0a00_0008);
        assert_eq!(calls(&log, 0x00ee_6600), [vec![b], vec![a]]);
        assert_eq!(
            calls(&log, INTERLOCKED_DECREMENT),
            [
                vec![0x0c00_0000],
                vec![b + 4],
                vec![a + 4],
                vec![0x0a00_0000]
            ]
        );
        assert_eq!(calls(&log, NI_INTERPOLATOR_DESTRUCT), [vec![this]]);
    }

    /// The doubles of `Update`'s math callees, computing in f64 and storing floats; the
    /// rotation `FromRotation` receives is recorded.
    fn math(e: &mut Engine, rotations: &Matrices) {
        e.register(POINT_UNITIZE, |e, a| {
            let p = [0u32, 4, 8].map(|k| f64::from(e.mem.f32(a[0] + k)));
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            for (k, v) in p.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * k as u32, (v / length) as f32);
            }
            Ret::default()
        });
        e.register(MATRIX_TIMES_POINT, |e, a| {
            for row in 0..3u32 {
                let mut sum = 0.0f64;
                for column in 0..3u32 {
                    let m = f64::from(e.mem.f32(a[0] + 4 * (3 * row + column)));
                    sum += m * f64::from(e.mem.f32(a[2] + 4 * column));
                }
                e.mem.set_f32(a[1] + 4 * row, sum as f32);
            }
            ret(a[1])
        });
        e.register(MATRIX_TRANSPOSE_TIMES, |e, a| {
            for row in 0..3u32 {
                for column in 0..3u32 {
                    let mut sum = 0.0f64;
                    for k in 0..3u32 {
                        let m = f64::from(e.mem.f32(a[0] + 4 * (3 * k + row)));
                        sum += m * f64::from(e.mem.f32(a[2] + 4 * (3 * k + column)));
                    }
                    e.mem.set_f32(a[1] + 4 * (3 * row + column), sum as f32);
                }
            }
            ret(a[1])
        });
        let record = rotations.clone();
        e.register_double(QUATERNION_FROM_ROTATION, move |e, a| {
            let r = [0u32, 1, 2, 3, 4, 5, 6, 7, 8].map(|k| e.mem.f32(a[1] + 4 * k));
            record.borrow_mut().push(r);
            for k in 0..4u32 {
                e.mem.set_f32(a[0] + 4 * k, k as f32 + 1.0);
            }
            ret(a[0])
        });
    }

    fn update_constants(e: &mut Engine) {
        range_constants(e);
        for (address, size) in [
            (MIN_DISTANCE_SQUARED, 4),
            (POLE_ABOVE, 4),
            (POLE_BELOW, 8),
            (ANTIPOLE_BELOW, 4),
            (ANTIPOLE_ABOVE, 8),
            (POLE_UP, 12),
            (UP, 12),
            (IDENTITY_MATRIX, 36),
        ] {
            e.map(address, size);
        }
        e.mem.set_f32(MIN_DISTANCE_SQUARED, 0.001);
        e.mem.set_f32(POLE_ABOVE, 1.001);
        e.mem.set_f64(POLE_BELOW, 0.999);
        e.mem.set_f32(ANTIPOLE_BELOW, -1.001);
        e.mem.set_f64(ANTIPOLE_ABOVE, -0.999);
        e.mem.set_f32(POLE_UP + 4, 1.0);
        e.mem.set_f32(UP + 8, 1.0);
        for k in [0u32, 4, 8] {
            e.mem.set_f32(IDENTITY_MATRIX + 4 * k, 1.0);
        }
    }

    /// The rotation `Update` builds for a direction `to - from`, flag bit 0 `flip` and
    /// `axis`, computed in f64 with float stores (the unitize double's rounding).
    fn expected_rotation(to: [f32; 3], from: [f32; 3], flip: bool, axis: u32) -> [f32; 9] {
        let unit = |v: [f32; 3]| -> [f32; 3] {
            let p = v.map(f64::from);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            p.map(|c| (c / length) as f32)
        };
        let d = unit([0, 1, 2].map(|k| (f64::from(to[k]) - f64::from(from[k])) as f32));
        let pole = (d[2] < 1.001 && f64::from(d[2]) > 0.999)
            || (-1.001 < d[2] && f64::from(d[2]) < -0.999);
        let (factor, up) = if pole {
            (d[1], [0.0f32, 1.0, 0.0])
        } else {
            (d[2], [0.0f32, 0.0, 1.0])
        };
        let along = d.map(|c| (f64::from(c) * f64::from(factor)) as f32);
        let u = unit([0, 1, 2].map(|k| (f64::from(up[k]) - f64::from(along[k])) as f32));
        let d = if flip { d } else { d.map(|c| -c) };
        let [ux, uy, uz] = u.map(f64::from);
        let [dx, dy, dz] = d.map(f64::from);
        let c = [
            (uy * dz - uz * dy) as f32,
            (uz * dx - ux * dz) as f32,
            (ux * dy - uy * dx) as f32,
        ];
        let columns = match axis {
            0 => [d, u, c.map(|v| -v)],
            1 => [c, d, u.map(|v| -v)],
            _ => [c, u, d],
        };
        let mut r = [0.0f32; 9];
        for (column, values) in columns.iter().enumerate() {
            for (row, value) in values.iter().enumerate() {
                r[3 * row + column] = *value;
            }
        }
        r
    }

    /// An interpolator with a look-at object at (0, 0, 10) and a target at `position`
    /// without a parent.
    fn look_at_setup(e: &mut Engine, position: [f32; 3]) -> (u32, u32, u32) {
        let look_at = e.mem.alloc(0xa0);
        e.mem.set_f32(look_at + 0x94, 10.0);
        let target = e.mem.alloc(0xa0);
        for (k, value) in position.iter().enumerate() {
            e.mem.set_f32(target + 0x58 + 4 * k as u32, *value);
        }
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + LOOK_AT, look_at);
        e.mem.set_f32(this + SCALE, 2.0);
        let out = e.mem.alloc(0x20);
        (this, target, out)
    }

    #[test]
    fn update_turns_toward_the_look_at_object_for_each_axis() {
        for (axis, flip) in [(0u16, false), (1, true), (2, false), (2, true)] {
            let mut e = Engine::new();
            update_constants(&mut e);
            let rotations = Matrices::default();
            math(&mut e, &rotations);
            let (this, target, out) = look_at_setup(&mut e, [1.0, 0.0, 0.0]);
            e.mem
                .set_u16(this + LOOK_AT_FLAGS, (axis << 1) | u16::from(flip));
            assert!(e
                .call(0x00a4_bec0, &args![this, 0.5f32, target, out])
                .bool());
            let want = expected_rotation([0.0, 0.0, 10.0], [1.0, 0.0, 0.0], flip, u32::from(axis));
            assert_eq!(*rotations.borrow(), [want], "axis {axis}");
            assert_eq!(e.mem.f32(this + ROTATE + 12), 4.0);
            assert_eq!(e.mem.f32(out + 0x10), 2.0);
            assert_eq!(e.mem.f32(out + 0x1c), 2.0);
            assert_eq!(e.mem.f32(this + LAST_TIME), 0.5);
        }
    }

    #[test]
    fn update_uses_the_pole_up_vector_and_the_parent_transform() {
        let mut e = Engine::new();
        update_constants(&mut e);
        let rotations = Matrices::default();
        math(&mut e, &rotations);
        let (this, target, out) = look_at_setup(&mut e, [0.0, 0.0, 0.0]);
        e.mem.set_u16(this + LOOK_AT_FLAGS, 2 << 1);
        // A parent at (0, 0, 2) with scale 0.5 and the identity rotation: the world
        // position of the local (0, 0, 2) is (0, 0, 3), straight below the look-at object.
        let parent = e.mem.alloc(0xa0);
        for k in [0u32, 4, 8] {
            e.mem.set_f32(parent + 0x68 + 4 * k, 1.0);
        }
        e.mem.set_f32(parent + 0x98, 0.5);
        e.mem.set_f32(parent + 0x94, 2.0);
        e.mem.set_u32(target + 0x18, parent);
        e.mem.set_f32(target + 0x60, 2.0);
        assert!(e
            .call(0x00a4_bec0, &args![this, 0.5f32, target, out])
            .bool());
        let want = expected_rotation([0.0, 0.0, 10.0], [0.0, 0.0, 3.0], false, 2);
        assert_eq!(*rotations.borrow(), [want]);
        // The direction is +z: the up vector is the pole one, y.
        assert_eq!(want[4], 1.0);
    }

    #[test]
    fn update_rolls_and_scales_through_the_other_interpolators() {
        let mut e = Engine::new();
        let log = Log::default();
        update_constants(&mut e);
        let rotations = Matrices::default();
        math(&mut e, &rotations);
        logged(&mut e, &log, MATRIX_MAKE_Z_ROTATION, 0);
        e.register(MATRIX_TIMES_MATRIX, |e, a| {
            // The roll double leaves the rotation as it is.
            for k in 0..9u32 {
                let word = e.mem.u32(a[0] + 4 * k);
                e.mem.set_u32(a[1] + 4 * k, word);
            }
            ret(a[1])
        });
        let outcome = Rc::new(Cell::new(1u32));
        let answer = outcome.clone();
        e.register_double(0x00ee_6700, move |e, a| {
            e.mem.set_f32(a[3], 0.25);
            ret(answer.get())
        });
        e.register(0x00ee_6710, |e, a| {
            e.mem.set_f32(a[3], 3.0);
            ret(1)
        });
        e.register(0x00ee_6720, |e, a| {
            for k in 0..3u32 {
                e.mem.set_f32(a[3] + 4 * k, 1.0);
            }
            ret(1)
        });
        let roll = object(&mut e, 8, &[(SLOT_UPDATE_FLOAT, 0x00ee_6700)]);
        let scale = object(&mut e, 8, &[(SLOT_UPDATE_FLOAT, 0x00ee_6710)]);
        let position = object(&mut e, 8, &[(SLOT_UPDATE_POINT3, 0x00ee_6720)]);
        let (this, target, out) = look_at_setup(&mut e, [5.0, 5.0, 5.0]);
        e.mem.set_u16(this + LOOK_AT_FLAGS, 2 << 1);
        e.mem.set_u32(this + INTERPOLATORS, position);
        e.mem.set_u32(this + INTERPOLATORS + 4, roll);
        e.mem.set_u32(this + INTERPOLATORS + 8, scale);
        assert!(e
            .call(0x00a4_bec0, &args![this, 0.5f32, target, out])
            .bool());
        // The first interpolator's position (1, 1, 1) replaced the target's.
        assert_eq!(e.mem.f32(this + TRANSFORM), 1.0);
        let want = expected_rotation([0.0, 0.0, 10.0], [1.0, 1.0, 1.0], false, 2);
        assert_eq!(*rotations.borrow(), [want]);
        let angle = (-0.25f32).to_bits();
        assert_eq!(calls(&log, MATRIX_MAKE_Z_ROTATION).len(), 1);
        assert_eq!(calls(&log, MATRIX_MAKE_Z_ROTATION)[0][1], angle);
        assert_eq!(e.mem.f32(this + SCALE), 3.0);
        assert_eq!(e.mem.f32(out + 0x1c), 3.0);
        // A failing roll marks the transform invalid.
        outcome.set(0);
        assert!(!e
            .call(0x00a4_bec0, &args![this, 0.5f32, target, out])
            .bool());
        assert_eq!(e.mem.f32(this + SCALE), -f32::MAX);
        assert_eq!(e.mem.f32(this + 0x28), -f32::MAX);
        assert_eq!(e.mem.f32(this + TRANSFORM), -f32::MAX);
    }

    #[test]
    fn update_handles_short_directions_and_invalid_transforms() {
        let mut e = Engine::new();
        update_constants(&mut e);
        let rotations = Matrices::default();
        math(&mut e, &rotations);
        // The look-at object sits on the target: the identity rotation.
        let (this, target, out) = look_at_setup(&mut e, [0.0, 0.0, 10.0]);
        assert!(e
            .call(0x00a4_bec0, &args![this, 0.5f32, target, out])
            .bool());
        assert_eq!(
            *rotations.borrow(),
            [[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]]
        );
        // All three parts invalid: the out transform is marked invalid.
        for offset in [SCALE, 0x28, TRANSFORM] {
            e.mem.set_f32(this + offset, -f32::MAX);
        }
        e.register(QUATERNION_FROM_ROTATION, |e, a| {
            for k in 0..4u32 {
                e.mem.set_f32(a[0] + 4 * k, -f32::MAX);
            }
            ret(a[0])
        });
        e.mem.set_f32(out, 1.0);
        assert!(!e
            .call(0x00a4_bec0, &args![this, 0.75f32, target, out])
            .bool());
        assert_eq!(e.mem.f32(out), -f32::MAX);
        assert_eq!(e.mem.f32(this + LAST_TIME), 0.5);
        // A failing position interpolator, or no look-at object.
        e.register(0x00ee_6800, |_, _| ret(0));
        let position = object(&mut e, 8, &[(SLOT_UPDATE_POINT3, 0x00ee_6800)]);
        e.mem.set_u32(this + INTERPOLATORS, position);
        e.mem.set_f32(this + SCALE, 1.0);
        assert!(!e
            .call(0x00a4_bec0, &args![this, 0.5f32, target, out])
            .bool());
        assert_eq!(e.mem.f32(this + SCALE), -f32::MAX);
        e.mem.set_u32(this + LOOK_AT, 0);
        e.mem.set_f32(out + 0x10, 1.0);
        assert!(!e
            .call(0x00a4_bec0, &args![this, 0.5f32, target, out])
            .bool());
        assert_eq!(e.mem.f32(out + 0x10), -f32::MAX);
    }

    #[test]
    fn copy_members_copies_flags_transform_and_cloned_interpolators() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        empty_string(&mut e);
        logged(&mut e, &log, KEY_BASED_COPY_MEMBERS, 0);
        let clone = object(&mut e, 8, &[]);
        logged(&mut e, &log, NI_OBJECT_CLONE, clone);
        let this = e.mem.alloc(0x44);
        let copy = e.mem.alloc(0x44);
        e.mem.set_u16(this + LOOK_AT_FLAGS, 0x0007);
        e.mem.set_u16(copy + LOOK_AT_FLAGS, 0x0010);
        e.mem.set_u32(this + LOOK_AT, 0x5000);
        e.mem.set_u32(copy + LOOK_AT_NAME, 0x0900_0008);
        for k in 0..8 {
            e.mem.set_u32(this + TRANSFORM + 4 * k, 0x7f80_0001 + k);
        }
        e.mem.set_u32(this + INTERPOLATORS + 8, 0x6000);
        e.call(0x00a4_c4b0, &args![this, copy, 0x9000u32]);
        assert_eq!(
            calls(&log, KEY_BASED_COPY_MEMBERS),
            [vec![this, copy, 0x9000]]
        );
        assert_eq!(e.mem.u16(copy + LOOK_AT_FLAGS), 0x0007);
        assert_eq!(e.mem.u32(copy + LOOK_AT), 0x5000);
        assert_eq!(e.mem.u32(copy + LOOK_AT_NAME), 0x0a00_0008);
        // The transform is copied word for word.
        assert_eq!(e.mem.u32(copy + TRANSFORM + 4), 0x7f80_0002);
        assert_eq!(calls(&log, NI_OBJECT_CLONE), [vec![0x6000]]);
        assert_eq!(e.mem.u32(copy + INTERPOLATORS + 8), clone);
        assert_eq!(e.mem.u32(clone + 4), 1);
    }

    #[test]
    fn create_object_and_create_clone_construct_a_default_interpolator() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        constructor_globals(&mut e);
        logged(&mut e, &log, NI_INTERPOLATOR_CONSTRUCT, 0);
        logged(&mut e, &log, KEY_BASED_COPY_MEMBERS, 0);
        let block = Rc::new(Cell::new(0u32));
        answers(&mut e, NI_ALLOCATE, &block);
        assert_eq!(e.call(0x00a4_c5a0, &args![]).u32(), 0);
        let this = e.mem.alloc(0x44);
        // A failed allocation still runs the member copy, into address 0 (mapped here).
        e.map(0, 0x100);
        e.mem.set_u32(LOOK_AT_NAME, 0x0900_0008);
        assert_eq!(e.call(0x00a4_c5e0, &args![this, 0x9000u32]).u32(), 0);
        assert_eq!(calls(&log, KEY_BASED_COPY_MEMBERS), [vec![this, 0, 0x9000]]);
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        constructor_globals(&mut e);
        logged(&mut e, &log, NI_INTERPOLATOR_CONSTRUCT, 0);
        logged(&mut e, &log, KEY_BASED_COPY_MEMBERS, 0);
        let first = e.mem.alloc(0x44);
        let second = e.mem.alloc(0x44);
        let blocks = RefCell::new(vec![first, second]);
        e.register_double(NI_ALLOCATE, move |_, a| {
            assert_eq!(a[0], 0x44);
            ret(blocks.borrow_mut().remove(0))
        });
        assert_eq!(e.call(0x00a4_c5a0, &args![]).u32(), first);
        assert_eq!(e.mem.u32(first), NI_LOOK_AT_INTERPOLATOR_VTABLE);
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + LOOK_AT, 0x5000);
        assert_eq!(e.call(0x00a4_c5e0, &args![this, 0x9000u32]).u32(), second);
        assert_eq!(e.mem.u32(second + LOOK_AT), 0x5000);
        assert_eq!(
            calls(&log, KEY_BASED_COPY_MEMBERS),
            [vec![this, second, 0x9000]]
        );
    }

    #[test]
    fn load_and_save_binary_stream_the_members() {
        let mut e = Engine::new();
        let log = Log::default();
        interlocked(&mut e, &log);
        for address in [
            NI_INTERPOLATOR_LOAD_BINARY,
            BASE_SAVE_BINARY,
            LOAD_CSTRING_AS_FIXED_STRING,
            LOAD_FIXED_STRING,
            SAVE_FIXED_STRING,
            QUAT_TRANSFORM_LOAD_BINARY,
            QUAT_TRANSFORM_SAVE_BINARY,
            0x00ee_6900,
        ] {
            logged(&mut e, &log, address, 0);
        }
        let interpolator = e.mem.alloc(8);
        let links = RefCell::new(vec![0x5000u32, 0, interpolator, 0]);
        e.register_double(RESOLVE_LINK_ID, move |_, _| {
            ret(links.borrow_mut().remove(0))
        });
        let transfers = Rc::new(RefCell::new(vec![]));
        let record = transfers.clone();
        e.register_double(0x00ee_6910, move |e, a| {
            record.borrow_mut().push((a[1], a[2], e.mem.u32(a[3])));
            e.mem.set_u16(a[1], 0x0005);
            ret(a[2])
        });
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + BINARY_STREAM_READ, 0x00ee_6910);
        e.mem.set_u32(binary + BINARY_STREAM_WRITE, 0x00ee_6910);
        let stream = object(&mut e, 0x260, &[(SLOT_SAVE_LINK_ID, 0x00ee_6900)]);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT, binary);
        e.mem.set_u32(stream + STREAM_VERSION, 0x1401_0000);
        let this = e.mem.alloc(0x44);
        e.call(0x00a4_c630, &args![this, stream]);
        assert_eq!(e.mem.u16(this + LOOK_AT_FLAGS), 5);
        assert_eq!(e.mem.u32(this + LOOK_AT), 0x5000);
        assert_eq!(
            calls(&log, LOAD_CSTRING_AS_FIXED_STRING),
            [vec![stream, this + LOOK_AT_NAME]]
        );
        assert_eq!(
            calls(&log, QUAT_TRANSFORM_LOAD_BINARY),
            [vec![this + TRANSFORM, stream]]
        );
        assert_eq!(e.mem.u32(this + INTERPOLATORS + 4), interpolator);
        assert_eq!(e.mem.u32(interpolator + 4), 1);
        e.call(0x00a4_c700, &args![this, stream]);
        assert_eq!(calls(&log, BASE_SAVE_BINARY), [vec![this, stream]]);
        assert_eq!(
            calls(&log, 0x00ee_6900),
            [
                vec![stream, 0x5000],
                vec![stream, 0],
                vec![stream, interpolator],
                vec![stream, 0]
            ]
        );
        assert_eq!(
            calls(&log, SAVE_FIXED_STRING),
            [vec![stream, this + LOOK_AT_NAME]]
        );
        assert_eq!(
            calls(&log, QUAT_TRANSFORM_SAVE_BINARY),
            [vec![this + TRANSFORM, stream]]
        );
        assert_eq!(
            *transfers.borrow(),
            [(this + LOOK_AT_FLAGS, 2, 2), (this + LOOK_AT_FLAGS, 2, 2)]
        );
        // Newer files load the name as a fixed string.
        e.mem.set_u32(stream + STREAM_VERSION, 0x1401_0001);
        let links = RefCell::new(vec![0u32; 4]);
        e.register_double(RESOLVE_LINK_ID, move |_, _| {
            ret(links.borrow_mut().remove(0))
        });
        e.mem.set_u32(this + INTERPOLATORS + 4, 0);
        e.call(0x00a4_c630, &args![this, stream]);
        assert_eq!(
            calls(&log, LOAD_FIXED_STRING),
            [vec![stream, this + LOOK_AT_NAME]]
        );
    }

    #[test]
    fn get_viewer_strings_adds_name_look_at_transform_and_interpolators() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(NI_LOOK_AT_INTERPOLATOR_RTTI, 4);
        e.mem.set_u32(NI_LOOK_AT_INTERPOLATOR_RTTI, 0x0155_0000);
        for (address, eax) in [
            (NI_INTERPOLATOR_GET_VIEWER_STRINGS, 0u32),
            (NI_GET_VIEWER_STRING, 0x0166_0000),
            (NI_GET_VIEWER_STRING_POINTER, 0x0177_0000),
            (VIEWER_STRINGS_GROW, 0),
            (QUAT_TRANSFORM_GET_VIEWER_STRINGS, 0),
            (0x00ee_6a00, 0),
        ] {
            logged(&mut e, &log, address, eax);
        }
        let seen = Rc::new(RefCell::new(vec![]));
        let record = seen.clone();
        e.register_double(VIEWER_STRINGS_SET_AT, move |e, a| {
            record.borrow_mut().push(e.mem.u32(a[2]));
            Ret::default()
        });
        let interpolator = object(&mut e, 8, &[(SLOT_GET_VIEWER_STRINGS, 0x00ee_6a00)]);
        let this = e.mem.alloc(0x44);
        e.mem.set_u32(this + LOOK_AT, 0x5000);
        e.mem.set_u32(this + INTERPOLATORS + 4, interpolator);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.call(0x00a4_c780, &args![this, strings]);
        assert_eq!(*seen.borrow(), [0x0166_0000, 0x0177_0000]);
        assert_eq!(
            calls(&log, NI_GET_VIEWER_STRING_POINTER),
            [vec![TEXT_LOOK_AT, 0x5000]]
        );
        assert_eq!(
            calls(&log, QUAT_TRANSFORM_GET_VIEWER_STRINGS),
            [vec![this + TRANSFORM, strings]]
        );
        assert_eq!(calls(&log, 0x00ee_6a00), [vec![interpolator, strings]]);
        assert!(calls(&log, VIEWER_STRINGS_GROW).is_empty());
    }
}
