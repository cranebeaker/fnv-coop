//! `NiAnimation/nilookatcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLookAtController::ms_RTTI` (the class's `NiRTTI` record).
const NI_LOOK_AT_CONTROLLER_RTTI: u32 = 0x011f_3f98;

// Translated from 00a4cd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_look_at_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LOOK_AT_CONTROLLER_RTTI)
}

// Translated from 00a4cd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a6d290`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x3c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_look_at_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a6_d290;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x3cu32]);
    }
    this
}

/// `NiLookAtController`'s vtable (the address its constructors store) and size.
const NI_LOOK_AT_CONTROLLER_VTABLE: u32 = 0x0109_8c14;
const NI_LOOK_AT_CONTROLLER_SIZE: u32 = 0x3c;
/// Fields (Xbox PDB names, PC offsets as the code uses them): `NiTimeController::m_uFlags`
/// +0x08, `m_pkTarget` +0x2c, this class's `m_uFlags` +0x34 (16 bits: bit 0 flip, bits 1-2
/// the axis) and `m_pkLookAt` +0x38.
const TIME_FLAGS: u32 = 0x08;
const TARGET: u32 = 0x2c;
const LOOK_AT_FLAGS: u32 = 0x34;
const LOOK_AT: u32 = 0x38;
/// `NiTimeController`'s constructor (`00a6d220`), `CopyMembers` (`00a6cfe0`), `LinkObject`
/// (`00a6d500`), `RegisterStreamables` (`00a6d0f0`), `IsEqual` (`00a6cc80`), `ProcessClone`
/// (`00a6d0c0`), `LoadBinary` (`00a6d5e0`), `SaveBinary` (`00a6d730`) and
/// `GetViewerStrings` (`00a6d830`), all thiscall.
const TIME_CONTROLLER_CONSTRUCT: u32 = 0x00a6_d220;
const TIME_CONTROLLER_COPY_MEMBERS: u32 = 0x00a6_cfe0;
const TIME_CONTROLLER_LINK_OBJECT: u32 = 0x00a6_d500;
const TIME_CONTROLLER_REGISTER_STREAMABLES: u32 = 0x00a6_d0f0;
const TIME_CONTROLLER_IS_EQUAL: u32 = 0x00a6_cc80;
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
const TIME_CONTROLLER_LOAD_BINARY: u32 = 0x00a6_d5e0;
const TIME_CONTROLLER_SAVE_BINARY: u32 = 0x00a6_d730;
const TIME_CONTROLLER_GET_VIEWER_STRINGS: u32 = 0x00a6_d830;
/// `NiStream::GetObjectFromLinkID` (`00a63430`) and `00a645b0` (thiscall on the stream); the
/// stream's `SaveLinkID` (virtual slot `0x30`); the clone map's lookup (`00a53c30`, `(key,
/// &value)`); `00aa13e0` (cdecl, one word): allocates from the Ni allocator.
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const SLOT_SAVE_LINK_ID: u32 = 0x30;
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// Virtual slots on other objects: `RegisterStreamables` (`0x54`) and `IsEqual` (`0x5c`).
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
/// `NiStream::m_uiNifFileVersion` +0xd8, `m_pkIstr` +0x24c and `m_pkOstr` +0x250 (Xbox PDB
/// names, PC offsets), the stream's 16-bit word +0x286 old files take the flags from, and the
/// binary stream's position (+4), read (+8) and write (+0xc).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const STREAM_OLD_FLAGS: u32 = 0x286;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// `NiGetViewerString` (`00a7d330`, cdecl) and the viewer-string array's resize (`0096ad30`)
/// and `SetAt` (`0096ae90`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `Update`'s constants: the smallest squared distance that turns the target (`01017d00`),
/// the bounds of a direction's z near +1 (`01098c00` float, `01098bf8` double), the up
/// vectors for that case (`011a9478`) and the others (`011a9484`), and the identity matrix
/// (`011a9448`).
const MIN_DISTANCE_SQUARED: u32 = 0x0101_7d00;
const POLE_ABOVE: u32 = 0x0109_8c00;
const POLE_BELOW: u32 = 0x0109_8bf8;
const POLE_UP: u32 = 0x011a_9478;
const UP: u32 = 0x011a_9484;
const IDENTITY_MATRIX: u32 = 0x011a_9448;
/// Math callees of `Update` (thiscall): the matrix times a point (`004b4500`, `(&result,
/// &point)`), `NiPoint3::Unitize` (`004a0c10`) and `NiMatrix3::TransposeTimes` (`00a588a0`,
/// `(&result, &matrix)`); each answers its result's address where it has one.
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
const POINT_UNITIZE: u32 = 0x004a_0c10;
const MATRIX_TRANSPOSE_TIMES: u32 = 0x00a5_88a0;

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

// Translated from 00a4c840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::LinkObject` (Xbox PDB): `NiTimeController::LinkObject`, then the
/// look-at object from the stream's next link (not counted).
pub fn ni_look_at_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TIME_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let look_at = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.mem.set_u32(this.addr().wrapping_add(LOOK_AT), look_at);
}

// Translated from 00a4c860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::RegisterStreamables` (Xbox PDB): false when
/// `NiTimeController::RegisterStreamables` is; otherwise the look-at object (when there is
/// one) registers too (its slot `0x54`; its answer unused). True.
pub fn ni_look_at_controller_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(TIME_CONTROLLER_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let look_at = e.mem.u32(this.addr().wrapping_add(LOOK_AT));
    if look_at != 0 {
        e.vcall(look_at, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    true
}

// Translated from 00a4c890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::IsEqual` (Xbox PDB): `NiTimeController::IsEqual`, then both or neither
/// look-at object, this one's `IsEqual` the other's (slot `0x5c`).
pub fn ni_look_at_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(TIME_CONTROLLER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let look_at = e.mem.u32(this.addr().wrapping_add(LOOK_AT));
    let other_look_at = e.mem.u32(other.addr().wrapping_add(LOOK_AT));
    if (look_at != 0) != (other_look_at != 0) {
        return false;
    }
    if look_at == 0 {
        return true;
    }
    let other_look_at = e.mem.u32(other.addr().wrapping_add(LOOK_AT));
    e.vcall(look_at, SLOT_IS_EQUAL, &args![other_look_at])
        .bool()
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

// Translated from 00a4c8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::Update` (Xbox PDB; the time argument is not read). With a look-at
/// object and flag bit 3 of the controller's flags byte (+8) set, the target's local rotation
/// (+0x34) turns toward the look-at object's world translation (+0x8c): the target's local
/// translation (+0x58) through its parent's (+0x18) world rotation (+0x68, `004b4500`),
/// scale (+0x98) and translation (+0x8c) (as it is, with the identity `011a9448`, without a
/// parent) gives the position; a direction whose squared length is below the float at
/// `01017d00` gives the identity rotation; otherwise it is unitized (`004a0c10`), the up
/// vector is `011a9478` when its z is near +1 (the bounds at `01098c00` and `01098bf8`),
/// `011a9484` otherwise, made perpendicular (minus the direction times its y or z) and
/// unitized; the direction is negated unless flag bit 0 (+0x34) is set; with the cross
/// product of up and direction the axis (bits 1-2) chooses the columns (0: direction, up,
/// -cross; 1: cross, direction, -up; 2: cross, up, direction; 3 leaves the rotation stack
/// words as they are, the frame's memory here), and the result is the parent's rotation
/// transposed times it (`00a588a0`).
pub fn ni_look_at_controller_update(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let object = this.addr();
    let look_at = e.mem.u32(object.wrapping_add(LOOK_AT));
    if look_at == 0 || (e.mem.u8(object.wrapping_add(TIME_FLAGS)) >> 3) & 1 == 0 {
        return;
    }
    let target = e.mem.u32(object.wrapping_add(TARGET));
    let parent = e.mem.u32(target.wrapping_add(0x18));
    // Frame (offsets from the stack pointer after the saved registers): the look-at
    // position (then the up vector's offset) +0x10, the world position (then the cross
    // product) +0x1c, the direction +0x28, the transformed position (then the up vector)
    // +0x34, the scale and factor float +0x40, the rotation +0x44, the parent's rotation
    // +0x6c and the matrix result +0x90.
    e.with_stack(0xb4, |e, frame| {
        let base = frame.addr();
        let look_at_position = base + 0x10;
        let world = base + 0x1c;
        let direction = base + 0x28;
        let up = base + 0x34;
        let scale_word = base + 0x40;
        let rotation = base + 0x44;
        let parent_rotation = base + 0x6c;
        let product = base + 0x90;
        for k in 0..3 {
            let word = e.mem.u32(look_at.wrapping_add(0x8c + 4 * k));
            e.mem.set_u32(look_at_position + 4 * k, word);
        }
        let position;
        if parent != 0 {
            for k in 0..9 {
                let word = e.mem.u32(parent.wrapping_add(0x68 + 4 * k));
                e.mem.set_u32(parent_rotation + 4 * k, word);
            }
            let scale = x87_quiet_bits(e.mem.u32(parent.wrapping_add(0x98)));
            e.mem.set_u32(scale_word, scale);
            let local = target.wrapping_add(0x58);
            let turned = e
                .call(MATRIX_TIMES_POINT, &args![parent_rotation, up, local])
                .u32();
            let s = x87_load32(e.mem.u32(scale_word));
            let tx = x87_load32(e.mem.u32(turned));
            e.mem.set_u32(world, x87_store32(x87_mul(tx, s)));
            let ty = x87_load32(e.mem.u32(turned.wrapping_add(4)));
            e.mem.set_u32(world + 4, x87_store32(x87_mul(ty, s)));
            let tz = x87_m32(e.mem.u32(turned.wrapping_add(8)));
            e.mem.set_u32(world + 8, x87_store32(x87_mul(s, tz)));
            let mut sums = [0u32; 3];
            for (k, sum) in sums.iter_mut().enumerate() {
                let k = k as u32;
                let translate = x87_load32(e.mem.u32(parent.wrapping_add(0x8c + 4 * k)));
                let scaled = x87_m32(e.mem.u32(world + 4 * k));
                *sum = x87_store32(x87_add(translate, scaled));
                e.mem.set_u32(up + 4 * k, *sum);
            }
            position = sums;
        } else {
            for k in 0..9 {
                let word = e.mem.u32(IDENTITY_MATRIX + 4 * k);
                e.mem.set_u32(parent_rotation + 4 * k, word);
            }
            position = [0u32, 4, 8].map(|k| e.mem.u32(target.wrapping_add(0x58 + k)));
        }
        store3(e, world, position);
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
        if length < x87_m32(e.mem.u32(MIN_DISTANCE_SQUARED)) {
            let identity = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
            for (k, value) in identity.iter().enumerate() {
                e.mem.set_f32(rotation + 4 * k as u32, *value);
            }
        } else {
            e.call(POINT_UNITIZE, &args![direction]);
            let z = x87_load32(e.mem.u32(direction + 8));
            let pole =
                z < x87_m32(e.mem.u32(POLE_ABOVE)) && z > f64::from_bits(e.mem.u64(POLE_BELOW));
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
        }
        for k in 0..9 {
            let word = e.mem.u32(rotation + 4 * k);
            e.mem.set_u32(target.wrapping_add(0x34 + 4 * k), word);
        }
    });
}

// Translated from 00a4ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::ProcessClone` (Xbox PDB): `NiTimeController::ProcessClone`; then the
/// clone of this controller is looked up in the cloning process's map (`00a53c30` on its
/// first word, into the argument's stack word set to 0) and, with a look-at object, the
/// clone's look-at object becomes the look-at object's clone (looked up into a stack word
/// that holds `this` until then) when the map has one, the same object otherwise.
pub fn ni_look_at_controller_process_clone(e: &mut Engine, this: Ptr, cloning: Ptr) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, cloning]);
    let map = e.mem.u32(cloning.addr());
    // Frame: the local word (holding `this`) +0, the argument word +4.
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument = local + 4;
        e.mem.set_u32(local, this.addr());
        e.mem.set_u32(argument, 0);
        e.call(CLONE_MAP_GET_AT, &args![map, this, argument]);
        let look_at = e.mem.u32(this.addr().wrapping_add(LOOK_AT));
        if look_at == 0 {
            return;
        }
        let map = e.mem.u32(cloning.addr());
        let found = e.call(CLONE_MAP_GET_AT, &args![map, look_at, local]).bool();
        let clone = e.mem.u32(argument);
        let value = if found {
            e.mem.u32(local)
        } else {
            e.mem.u32(this.addr().wrapping_add(LOOK_AT))
        };
        e.mem.set_u32(clone.wrapping_add(LOOK_AT), value);
    });
}

/// The flags copy `CreateClone` does: the whole word, then bit 0 and bits 1-2 as the
/// source's (the compiled bitfield copies).
fn copy_look_at_flags(e: &mut Engine, source: u32, copy: u32) {
    let at = copy.wrapping_add(LOOK_AT_FLAGS);
    let flags = e.mem.u16(source.wrapping_add(LOOK_AT_FLAGS));
    e.mem.set_u16(at, flags);
    let flags = if e.mem.u8(source.wrapping_add(LOOK_AT_FLAGS)) & 1 != 0 {
        flags | 1
    } else {
        flags & 0xfffe
    };
    e.mem.set_u16(at, flags);
    let change = (flags ^ e.mem.u16(source.wrapping_add(LOOK_AT_FLAGS))) & 6;
    e.mem.set_u16(at, e.mem.u16(at) ^ change);
}

// Translated from 00a4cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::CreateClone` (Xbox PDB): a new controller (0x3C bytes: the
/// `NiTimeController` constructor `00a6d220`, flag bit 6 of the time flags cleared, the
/// vtable, no look-at object, flags 0), `NiTimeController`'s member copy (`00a6cfe0`, also
/// for a null clone, as the exe does), the flags and the look-at object.
pub fn ni_look_at_controller_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> u32 {
    let object = e
        .call(NI_ALLOCATE, &args![NI_LOOK_AT_CONTROLLER_SIZE])
        .u32();
    if object != 0 {
        e.call(TIME_CONTROLLER_CONSTRUCT, &args![object]);
        let flags = e.mem.u16(object.wrapping_add(TIME_FLAGS));
        e.mem
            .set_u16(object.wrapping_add(TIME_FLAGS), flags & 0xffbf);
        e.mem.set_u32(object, NI_LOOK_AT_CONTROLLER_VTABLE);
        e.mem.set_u32(object.wrapping_add(LOOK_AT), 0);
        e.mem.set_u16(object.wrapping_add(LOOK_AT_FLAGS), 0);
    }
    e.call(TIME_CONTROLLER_COPY_MEMBERS, &args![this, object, cloning]);
    copy_look_at_flags(e, this.addr(), object);
    let look_at = e.mem.u32(this.addr().wrapping_add(LOOK_AT));
    e.mem.set_u32(object.wrapping_add(LOOK_AT), look_at);
    object
}

// Translated from 00a4cde0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::CreateObject` (registered as the `"NiLookAtController"` loader at
/// `00a2bde0`; Ghidra has no function here): allocates 0x3C bytes and, when that succeeds,
/// runs the `NiTimeController` constructor, clears the flags and flag bit 6 of the time
/// flags, stores the vtable and clears the look-at object. Returns it, or 0.
pub fn ni_look_at_controller_create_object(e: &mut Engine) -> u32 {
    let object = e
        .call(NI_ALLOCATE, &args![NI_LOOK_AT_CONTROLLER_SIZE])
        .u32();
    if object == 0 {
        return 0;
    }
    e.call(TIME_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u16(object.wrapping_add(LOOK_AT_FLAGS), 0);
    let flags = e.mem.u16(object.wrapping_add(TIME_FLAGS));
    e.mem
        .set_u16(object.wrapping_add(TIME_FLAGS), flags & 0xffbf);
    e.mem.set_u32(object, NI_LOOK_AT_CONTROLLER_VTABLE);
    e.mem.set_u32(object.wrapping_add(LOOK_AT), 0);
    object
}

// Translated from 00a4ce20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::LoadBinary` (Xbox PDB): `NiTimeController::LoadBinary`; files before
/// version `0x0a000102` take the flags from bits 5-12 of the stream's word at +0x286, newer
/// ones read them (2 bytes; the argument's own stack word is the component size word); then
/// the look-at object's link ID (`00a645b0`).
pub fn ni_look_at_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TIME_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    let object = this.addr();
    let stream = stream.addr();
    if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) < 0x0a00_0102 {
        let old = e.mem.u16(stream.wrapping_add(STREAM_OLD_FLAGS));
        e.mem
            .set_u16(object.wrapping_add(LOOK_AT_FLAGS), (old >> 5) & 0xff);
    } else {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), stream);
            let flags = object.wrapping_add(LOOK_AT_FLAGS);
            let input = (STREAM_INPUT, BINARY_STREAM_READ);
            stream_transfer(e, stream, input.0, input.1, flags, 2, slot.addr());
        });
    }
    e.call(STREAM_READ_LINK_ID, &args![stream]);
}

// Translated from 00a4cea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::SaveBinary` (Xbox PDB): `NiTimeController::SaveBinary`, the flags (2
/// bytes; the argument's own stack word is the component size word) and the look-at object's
/// link ID (the stream's slot `0x30`).
pub fn ni_look_at_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(TIME_CONTROLLER_SAVE_BINARY, &args![this, stream]);
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
}

// Translated from 00a4cef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLookAtController::GetViewerStrings` (Xbox PDB): `NiTimeController::GetViewerStrings`,
/// then the class name (the first word of the `NiRTTI` record at `011f3f98`, through
/// `NiGetViewerString`) appended to the array (grown by its step when full), held in the
/// argument's own stack word.
pub fn ni_look_at_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(TIME_CONTROLLER_GET_VIEWER_STRINGS, &args![this, strings]);
    let strings = strings.addr();
    let class_name = e.mem.u32(NI_LOOK_AT_CONTROLLER_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4cd30, ni_look_at_controller_get_rtti() -> Ptr),
        entry!(0x00a4cd40, ni_look_at_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_c840, ni_look_at_controller_link_object(Ptr, Ptr)),
        entry!(
            0x00a4_c860,
            ni_look_at_controller_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a4_c890, ni_look_at_controller_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a4_c8e0, ni_look_at_controller_update(Ptr, u32)),
        entry!(0x00a4_ccd0, ni_look_at_controller_process_clone(Ptr, Ptr)),
        entry!(
            0x00a4_cd60,
            ni_look_at_controller_create_clone(Ptr, Ptr) -> u32
        ),
        entry!(0x00a4_cde0, ni_look_at_controller_create_object() -> u32),
        entry!(0x00a4_ce20, ni_look_at_controller_load_binary(Ptr, Ptr)),
        entry!(0x00a4_cea0, ni_look_at_controller_save_binary(Ptr, Ptr)),
        entry!(
            0x00a4_cef0,
            ni_look_at_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_look_at_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4cd30, &args![]).u32(),
            NI_LOOK_AT_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_look_at_controller_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a6_d290, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a4cd40, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a4cd40, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x3c]]);
        assert_eq!(calls_to(&e, 0x00a6_d290), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

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

    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn link_object_reads_the_look_at_link() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, TIME_CONTROLLER_LINK_OBJECT, 0);
        logged(&mut e, &log, GET_OBJECT_FROM_LINK_ID, 0x5000);
        let this = e.mem.alloc(0x3c);
        e.call(0x00a4_c840, &args![this, 0x6000u32]);
        assert_eq!(e.mem.u32(this + LOOK_AT), 0x5000);
        assert_eq!(
            calls(&log, TIME_CONTROLLER_LINK_OBJECT),
            [vec![this, 0x6000]]
        );
        assert_eq!(calls(&log, GET_OBJECT_FROM_LINK_ID), [vec![0x6000]]);
    }

    #[test]
    fn register_streamables_registers_the_look_at_object() {
        let mut e = Engine::new();
        let log = Log::default();
        let base = Rc::new(Cell::new(0u32));
        answers(&mut e, TIME_CONTROLLER_REGISTER_STREAMABLES, &base);
        logged(&mut e, &log, 0x00ee_7000, 0);
        let look_at = object(&mut e, 8, &[(SLOT_REGISTER_STREAMABLES, 0x00ee_7000)]);
        let this = e.mem.alloc(0x3c);
        assert!(!e.call(0x00a4_c860, &args![this, 0x6000u32]).bool());
        base.set(1);
        assert!(e.call(0x00a4_c860, &args![this, 0x6000u32]).bool());
        e.mem.set_u32(this + LOOK_AT, look_at);
        assert!(e.call(0x00a4_c860, &args![this, 0x6000u32]).bool());
        assert_eq!(calls(&log, 0x00ee_7000), [vec![look_at, 0x6000]]);
    }

    #[test]
    fn is_equal_compares_the_look_at_objects() {
        let mut e = Engine::new();
        let base = Rc::new(Cell::new(1u32));
        answers(&mut e, TIME_CONTROLLER_IS_EQUAL, &base);
        let equal = Rc::new(Cell::new(1u32));
        answers(&mut e, 0x00ee_7100, &equal);
        let a = object(&mut e, 8, &[(SLOT_IS_EQUAL, 0x00ee_7100)]);
        let this = e.mem.alloc(0x3c);
        let other = e.mem.alloc(0x3c);
        assert!(e.call(0x00a4_c890, &args![this, other]).bool());
        e.mem.set_u32(other + LOOK_AT, a);
        assert!(!e.call(0x00a4_c890, &args![this, other]).bool());
        e.mem.set_u32(this + LOOK_AT, a);
        assert!(e.call(0x00a4_c890, &args![this, other]).bool());
        equal.set(0);
        assert!(!e.call(0x00a4_c890, &args![this, other]).bool());
        e.mem.set_u32(other + LOOK_AT, 0);
        assert!(!e.call(0x00a4_c890, &args![this, other]).bool());
        base.set(0);
        e.mem.set_u32(this + LOOK_AT, 0);
        assert!(!e.call(0x00a4_c890, &args![this, other]).bool());
    }

    type Matrices = Rc<RefCell<Vec<[f32; 9]>>>;

    /// The doubles of `Update`'s math callees, computing in f64 and storing floats; the
    /// rotation `TransposeTimes` receives is recorded.
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
        let record = rotations.clone();
        e.register_double(MATRIX_TRANSPOSE_TIMES, move |e, a| {
            let r = [0u32, 1, 2, 3, 4, 5, 6, 7, 8].map(|k| e.mem.f32(a[2] + 4 * k));
            record.borrow_mut().push(r);
            for k in 0..9u32 {
                e.mem.set_f32(a[1] + 4 * k, 10.0 + k as f32);
            }
            ret(a[1])
        });
    }

    fn update_constants(e: &mut Engine) {
        for (address, size) in [
            (MIN_DISTANCE_SQUARED, 4),
            (POLE_ABOVE, 4),
            (POLE_BELOW, 8),
            (POLE_UP, 12),
            (UP, 12),
            (IDENTITY_MATRIX, 36),
        ] {
            e.map(address, size);
        }
        e.mem.set_f32(MIN_DISTANCE_SQUARED, 0.001);
        e.mem.set_f32(POLE_ABOVE, 1.001);
        e.mem.set_f64(POLE_BELOW, 0.999);
        e.mem.set_f32(POLE_UP + 4, 1.0);
        e.mem.set_f32(UP + 8, 1.0);
        for k in [0u32, 4, 8] {
            e.mem.set_f32(IDENTITY_MATRIX + 4 * k, 1.0);
        }
    }

    /// The rotation `Update` builds for a direction `to - from` (no -1 pole case here).
    fn expected_rotation(to: [f32; 3], from: [f32; 3], flip: bool, axis: u32) -> [f32; 9] {
        let unit = |v: [f32; 3]| -> [f32; 3] {
            let p = v.map(f64::from);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            p.map(|c| (c / length) as f32)
        };
        let d = unit([0, 1, 2].map(|k| (f64::from(to[k]) - f64::from(from[k])) as f32));
        let pole = d[2] < 1.001 && f64::from(d[2]) > 0.999;
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

    /// A controller (flag bit 3 set) with a look-at object at `to` and a target with local
    /// translation `from` and no parent.
    fn setup(e: &mut Engine, to: [f32; 3], from: [f32; 3]) -> (u32, u32) {
        let look_at = e.mem.alloc(0xa0);
        let target = e.mem.alloc(0xa0);
        for k in 0..3u32 {
            e.mem.set_f32(look_at + 0x8c + 4 * k, to[k as usize]);
            e.mem.set_f32(target + 0x58 + 4 * k, from[k as usize]);
        }
        let this = e.mem.alloc(0x3c);
        e.mem.set_u8(this + TIME_FLAGS, 0x08);
        e.mem.set_u32(this + LOOK_AT, look_at);
        e.mem.set_u32(this + TARGET, target);
        (this, target)
    }

    #[test]
    fn update_turns_the_target_for_each_axis() {
        for (axis, flip) in [(0u16, false), (1, true), (2, false), (3, false)] {
            let mut e = Engine::new();
            update_constants(&mut e);
            let rotations = Matrices::default();
            math(&mut e, &rotations);
            let (this, target) = setup(&mut e, [0.0, 3.0, 10.0], [1.0, 0.0, 0.0]);
            e.mem
                .set_u16(this + LOOK_AT_FLAGS, (axis << 1) | u16::from(flip));
            e.call(0x00a4_c8e0, &args![this, 0u32]);
            if axis < 3 {
                let want =
                    expected_rotation([0.0, 3.0, 10.0], [1.0, 0.0, 0.0], flip, u32::from(axis));
                assert_eq!(*rotations.borrow(), [want], "axis {axis}");
            } else {
                // Axis 3 leaves the frame's rotation words (zero here) as they are.
                assert_eq!(*rotations.borrow(), [[0.0; 9]]);
            }
            // The target's local rotation is the transpose-times result.
            assert_eq!(e.mem.f32(target + 0x34), 10.0);
            assert_eq!(e.mem.f32(target + 0x54), 18.0);
        }
    }

    #[test]
    fn update_uses_the_parent_and_the_pole_up_vector() {
        let mut e = Engine::new();
        update_constants(&mut e);
        let rotations = Matrices::default();
        math(&mut e, &rotations);
        let (this, target) = setup(&mut e, [0.0, 0.0, 10.0], [0.0, 0.0, 2.0]);
        e.mem.set_u16(this + LOOK_AT_FLAGS, 2 << 1);
        let parent = e.mem.alloc(0xa0);
        for k in [0u32, 4, 8] {
            e.mem.set_f32(parent + 0x68 + 4 * k, 1.0);
        }
        e.mem.set_f32(parent + 0x98, 0.5);
        e.mem.set_f32(parent + 0x94, 2.0);
        e.mem.set_u32(target + 0x18, parent);
        e.call(0x00a4_c8e0, &args![this, 0u32]);
        let want = expected_rotation([0.0, 0.0, 10.0], [0.0, 0.0, 3.0], false, 2);
        assert_eq!(*rotations.borrow(), [want]);
        assert_eq!(want[4], 1.0);
    }

    #[test]
    fn update_skips_without_the_flag_and_sets_identity_when_close() {
        let mut e = Engine::new();
        update_constants(&mut e);
        let rotations = Matrices::default();
        math(&mut e, &rotations);
        let (this, target) = setup(&mut e, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]);
        e.mem.set_u8(this + TIME_FLAGS, 0xf7);
        e.mem.set_f32(target + 0x34, 7.0);
        e.call(0x00a4_c8e0, &args![this, 0u32]);
        assert_eq!(e.mem.f32(target + 0x34), 7.0);
        e.mem.set_u8(this + TIME_FLAGS, 0x08);
        e.call(0x00a4_c8e0, &args![this, 0u32]);
        let rotation = [0u32, 1, 2, 3, 4, 5, 6, 7, 8].map(|k| e.mem.f32(target + 0x34 + 4 * k));
        assert_eq!(rotation, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        assert!(rotations.borrow().is_empty());
        e.mem.set_u32(this + LOOK_AT, 0);
        e.mem.set_f32(target + 0x34, 7.0);
        e.call(0x00a4_c8e0, &args![this, 0u32]);
        assert_eq!(e.mem.f32(target + 0x34), 7.0);
    }

    #[test]
    fn process_clone_maps_the_look_at_object() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, TIME_CONTROLLER_PROCESS_CLONE, 0);
        let clone = e.mem.alloc(0x3c);
        let found = Rc::new(Cell::new(true));
        let answer = found.clone();
        let this = e.mem.alloc(0x3c);
        e.register_double(CLONE_MAP_GET_AT, move |e, a| {
            if a[1] == 0x7000 {
                // The look-up's stack word holds `this` until it is written.
                assert_eq!(e.mem.u32(a[2]), this);
                if answer.get() {
                    e.mem.set_u32(a[2], 0x7100);
                }
                return ret(u32::from(answer.get()));
            }
            assert_eq!(e.mem.u32(a[2]), 0);
            e.mem.set_u32(a[2], clone);
            ret(1)
        });
        let cloning = e.mem.alloc(8);
        e.mem.set_u32(cloning, 0x8000);
        // Without a look-at object only the clone is looked up.
        e.call(0x00a4_ccd0, &args![this, cloning]);
        assert_eq!(e.mem.u32(clone + LOOK_AT), 0);
        e.mem.set_u32(this + LOOK_AT, 0x7000);
        e.call(0x00a4_ccd0, &args![this, cloning]);
        assert_eq!(e.mem.u32(clone + LOOK_AT), 0x7100);
        found.set(false);
        e.call(0x00a4_ccd0, &args![this, cloning]);
        assert_eq!(e.mem.u32(clone + LOOK_AT), 0x7000);
        assert_eq!(calls(&log, TIME_CONTROLLER_PROCESS_CLONE).len(), 3);
    }

    #[test]
    fn create_clone_and_create_object_build_the_controller() {
        let mut e = Engine::new();
        let log = Log::default();
        e.register(TIME_CONTROLLER_CONSTRUCT, |e, a| {
            e.mem.set_u16(a[0] + TIME_FLAGS, 0xffff);
            Ret::default()
        });
        logged(&mut e, &log, TIME_CONTROLLER_COPY_MEMBERS, 0);
        let block = Rc::new(Cell::new(0u32));
        answers(&mut e, NI_ALLOCATE, &block);
        assert_eq!(e.call(0x00a4_cde0, &args![]).u32(), 0);
        let object = e.mem.alloc(0x3c);
        e.mem.set_u16(object + LOOK_AT_FLAGS, 0x55);
        e.mem.set_u32(object + LOOK_AT, 0x1234);
        block.set(object);
        assert_eq!(e.call(0x00a4_cde0, &args![]).u32(), object);
        assert_eq!(e.mem.u32(object), NI_LOOK_AT_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u16(object + TIME_FLAGS), 0xffbf);
        assert_eq!(e.mem.u16(object + LOOK_AT_FLAGS), 0);
        assert_eq!(e.mem.u32(object + LOOK_AT), 0);
        let this = e.mem.alloc(0x3c);
        e.mem.set_u16(this + LOOK_AT_FLAGS, 0x0105);
        e.mem.set_u32(this + LOOK_AT, 0x5000);
        assert_eq!(e.call(0x00a4_cd60, &args![this, 0x9000u32]).u32(), object);
        assert_eq!(
            calls(&log, TIME_CONTROLLER_COPY_MEMBERS),
            [vec![this, object, 0x9000]]
        );
        assert_eq!(e.mem.u16(object + LOOK_AT_FLAGS), 0x0105);
        assert_eq!(e.mem.u32(object + LOOK_AT), 0x5000);
    }

    #[test]
    fn load_and_save_binary_stream_the_flags_and_link() {
        let mut e = Engine::new();
        let log = Log::default();
        for address in [
            TIME_CONTROLLER_LOAD_BINARY,
            TIME_CONTROLLER_SAVE_BINARY,
            STREAM_READ_LINK_ID,
            0x00ee_7200,
        ] {
            logged(&mut e, &log, address, 0);
        }
        let transfers = Rc::new(RefCell::new(vec![]));
        let record = transfers.clone();
        e.register_double(0x00ee_7210, move |e, a| {
            record.borrow_mut().push((a[1], a[2], e.mem.u32(a[3])));
            e.mem.set_u16(a[1], 0x0003);
            ret(a[2])
        });
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + BINARY_STREAM_READ, 0x00ee_7210);
        e.mem.set_u32(binary + BINARY_STREAM_WRITE, 0x00ee_7210);
        let stream = object(&mut e, 0x290, &[(SLOT_SAVE_LINK_ID, 0x00ee_7200)]);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        e.mem.set_u32(stream + STREAM_OUTPUT, binary);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a00_0101);
        e.mem.set_u16(stream + STREAM_OLD_FLAGS, 0x1fe0);
        let this = e.mem.alloc(0x3c);
        e.call(0x00a4_ce20, &args![this, stream]);
        assert_eq!(e.mem.u16(this + LOOK_AT_FLAGS), 0xff);
        assert!(transfers.borrow().is_empty());
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a00_0102);
        e.call(0x00a4_ce20, &args![this, stream]);
        assert_eq!(e.mem.u16(this + LOOK_AT_FLAGS), 3);
        assert_eq!(
            calls(&log, STREAM_READ_LINK_ID),
            [vec![stream], vec![stream]]
        );
        e.mem.set_u32(this + LOOK_AT, 0x5000);
        e.call(0x00a4_cea0, &args![this, stream]);
        assert_eq!(
            calls(&log, TIME_CONTROLLER_SAVE_BINARY),
            [vec![this, stream]]
        );
        assert_eq!(calls(&log, 0x00ee_7200), [vec![stream, 0x5000]]);
        assert_eq!(
            *transfers.borrow(),
            [(this + LOOK_AT_FLAGS, 2, 2), (this + LOOK_AT_FLAGS, 2, 2)]
        );
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(NI_LOOK_AT_CONTROLLER_RTTI, 4);
        e.mem.set_u32(NI_LOOK_AT_CONTROLLER_RTTI, 0x0155_0000);
        logged(&mut e, &log, TIME_CONTROLLER_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x0166_0000);
        logged(&mut e, &log, VIEWER_STRINGS_GROW, 0);
        e.register(VIEWER_STRINGS_SET_AT, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x0166_0000);
            Ret::default()
        });
        let this = e.mem.alloc(0x3c);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a4_cef0, &args![this, strings]);
        assert_eq!(
            calls(&log, TIME_CONTROLLER_GET_VIEWER_STRINGS),
            [vec![this, strings]]
        );
        assert_eq!(calls(&log, NI_GET_VIEWER_STRING), [vec![0x0155_0000]]);
        assert_eq!(calls(&log, VIEWER_STRINGS_GROW), [vec![strings, 2]]);
    }
}
