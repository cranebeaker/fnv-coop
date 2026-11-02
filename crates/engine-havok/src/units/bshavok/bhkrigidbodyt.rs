//! `BSHavok/bhkrigidbodyt.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkRigidBodyT` (Xbox PDB, RTTI-confirmed) is a `bhkRigidBody` that keeps a local offset
//! of the Havok body: a rotation quaternion at `+0x20` and a translation vector at `+0x30`
//! (16-byte vectors, in the order `x y z w`). Its position and rotation accessors apply that
//! offset to what the Havok body reports. The vector code follows the SSE instruction order
//! of the exe lane by lane.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame
//! or a `/GS` stack-cookie check (`00ec408c`), neither of which is modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// Per-thread word (`TLS + 0x2b4`) that the allocating functions set to 0x10 around their work.
const SCOPE_WORD: u32 = 0x2b4;
/// The game heap's allocator (`00aa13e0`, cdecl `(size)`).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// The memory manager (`011f6238`) with its allocate (`00aa3e40`) and free (`00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `kernel32` `InterlockedIncrement` / `InterlockedDecrement` import slots.
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `bhkRefObject` constructor (`00c87c30`).
const REF_OBJECT_CONSTRUCT: u32 = 0x00c8_7c30;
/// `bhkSerializable::PostInit` (`00c876d0`) and `bhkSerializable::SethkObject` (`00c87760`),
/// the clone lock (`00c876f0`) and its unlock (`00c87700`).
const SERIALIZABLE_POST_INIT: u32 = 0x00c8_76d0;
const SERIALIZABLE_SET_HK_OBJECT: u32 = 0x00c8_7760;
const CLONE_LOCK: u32 = 0x00c8_76f0;
const CLONE_UNLOCK: u32 = 0x00c8_7700;
/// `bhkRigidBody` methods this unit's overrides call (Xbox PDB names).
const RIGID_BODY_GET_POSITION: u32 = 0x00c8_d0f0;
const RIGID_BODY_GET_ROTATION: u32 = 0x00c8_d130;
const RIGID_BODY_SET_POSITION: u32 = 0x00c8_d170;
const RIGID_BODY_SET_ROTATION: u32 = 0x00c8_d1a0;
const RIGID_BODY_SET_POS_ROT: u32 = 0x00c8_d1d0;
const RIGID_BODY_GET_CENTER_OF_MASS_IN_WORLD: u32 = 0x00c8_d240;
const RIGID_BODY_CREATE_DISPLAY_GEOMETRY: u32 = 0x00c8_d2f0;
const RIGID_BODY_SAVE_BINARY: u32 = 0x00c8_d960;
const RIGID_BODY_KILL: u32 = 0x00c8_da40;
const RIGID_BODY_DESTRUCT: u32 = 0x00c8_e9c0;
const RIGID_BODY_LOAD_BINARY: u32 = 0x00c8_ea30;
const RIGID_BODY_COPY_MEMBERS: u32 = 0x00c8_f2a0;
const RIGID_BODY_GET_VIEWER_STRINGS: u32 = 0x00c8_f870;
/// Reference counting of Havok objects: add (`00c90510`) and remove (`00c905b0`).
const HK_ADD_REFERENCE: u32 = 0x00c9_0510;
const HK_REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The empty lock call (`00a29680`) the PC build kept around the Havok accesses.
const EMPTY_LOCK: u32 = 0x00a2_9680;
/// `hkQuaternion` helpers: product (`00c66320`, `(out, left, right)`), normalise (`005611c0`)
/// and `a * inverse(b)` (`00c74ac0`, `(out, a, b)`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const QUATERNION_NORMALIZE: u32 = 0x0056_11c0;
const QUATERNION_DIFFERENCE: u32 = 0x00c7_4ac0;
/// `hkVector4::setMul3` (`00c7f750`, `(out, matrix, vector)`).
const VECTOR_SET_MUL3: u32 = 0x00c7_f750;
/// `hkRotation` from a quaternion (`00cb2d90`, `(out, quaternion)`), the `NiMatrix3` from an
/// `hkRotation` (`00624b10`, cdecl `(out, rotation)`) and the `NiPoint3` from a vector
/// (`00458620`, cdecl `(out, vector)`).
const ROTATION_FROM_QUATERNION: u32 = 0x00cb_2d90;
const NI_MATRIX_FROM_ROTATION: u32 = 0x0062_4b10;
const NI_POINT3_FROM_VECTOR: u32 = 0x0045_8620;
/// Scene graph: the `NiNode` constructor (`00a5ecb0`, thiscall, one argument), its update
/// (`00a59c60`, `(update data)`), `NiObjectNET::SetName` (`00a5b950`, `(fixed string)`), and
/// the fixed-string handle of a text (`00a5b690`, cdecl) with the default handle's global.
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NODE_UPDATE: u32 = 0x00a5_9c60;
const OBJECT_SET_NAME: u32 = 0x00a5_b950;
const FIXED_STRING_ASSIGN: u32 = 0x00a5_b690;
const FIXED_STRING_DEFAULT: u32 = 0x0109_b220;
/// The text `"bhkRigidBodyT"` the display node is named with.
const NAME_TEXT: u32 = 0x010c_3ccc;
/// Viewer strings: `NiGetViewerString` for a class name (`00a7d330`, cdecl), for a
/// quaternion (`00a6e190`, `(this, name)`) and for a point (`00a59af0`, `(this, name)`);
/// the array's grow (`0096ad30`) and set-at (`0096ae90`) methods.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_QUATERNION: u32 = 0x00a6_e190;
const VIEWER_STRING_POINT: u32 = 0x00a5_9af0;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"Local Pos"` and `"Local Rot"`, the names of the viewer strings.
const NAME_LOCAL_POSITION: u32 = 0x010c_889c;
const NAME_LOCAL_ROTATION: u32 = 0x010c_88a8;
/// The default vector (`01267e30`, all zero), the identity quaternion (`010c71b0`) and the
/// vector (`010c71e0`) added to `w * w` in the rotation of the offset (-0.5 in each lane).
const DEFAULT_VECTOR: u32 = 0x0126_7e30;
const IDENTITY_QUATERNION: u32 = 0x010c_71b0;
const ROTATION_BIAS: u32 = 0x010c_71e0;
/// `bhkRigidBodyT::GetRTTI`'s constant (the `NiRTTI` of the class).
const CLASS_RTTI: u32 = 0x0126_83c8;
/// Vtables and object counters of the class chain the constructors store: `bhkWorldObject`,
/// `bhkEntity`, `bhkRigidBody`, `bhkRigidBodyT` (RTTI-confirmed).
const VTABLE_WORLD_OBJECT: u32 = 0x0103_0394;
const COUNT_WORLD_OBJECT: u32 = 0x0126_810c;
const VTABLE_ENTITY: u32 = 0x0103_02bc;
const COUNT_ENTITY: u32 = 0x0126_81fc;
const VTABLE_RIGID_BODY: u32 = 0x0103_01b4;
const COUNT_RIGID_BODY: u32 = 0x0126_81bc;
const VTABLE_CLASS: u32 = 0x010c_879c;
const COUNT_CLASS: u32 = 0x0126_83c0;
/// Virtual slots: `SethkObject` (`0x8c`), the virtual at `0xa4` (the function the exe calls
/// with the flag word `CreateCinfo` answered; engine map: `00c9f320`), `CreateCinfo` (`0xb4`),
/// `GetPosition` (`0xd4`), `GetRotation` (`0xd8`), `AttachChild` (`0xdc`) and
/// `SetHavokAssertState` (`0xfc`).
const VSLOT_SET_HK_OBJECT: u32 = 0x8c;
const VSLOT_RELEASE_CINFO: u32 = 0xa4;
const VSLOT_CREATE_CINFO: u32 = 0xb4;
const VSLOT_GET_POSITION: u32 = 0xd4;
const VSLOT_GET_ROTATION: u32 = 0xd8;
const VSLOT_ATTACH_CHILD: u32 = 0xdc;
const VSLOT_SET_HAVOK_ASSERT_STATE: u32 = 0xfc;

/// Four single-precision lanes (an `hkVector4` / `hkQuaternion` in an XMM register).
type Vector = [f32; 4];

fn load_vector(e: &Engine, address: u32) -> Vector {
    [0, 4, 8, 12].map(|lane| e.mem.f32(address + lane))
}

fn store_vector(e: &mut Engine, address: u32, vector: Vector) {
    for (lane, value) in vector.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, value);
    }
}

/// Copies 16 bytes (one `MOVAPS`).
fn copy_vector(e: &mut Engine, destination: u32, source: u32) {
    for word in 0..4 {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(destination + 4 * word, value);
    }
}

fn multiply(a: Vector, b: Vector) -> Vector {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

fn add_vectors(a: Vector, b: Vector) -> Vector {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

fn subtract(a: Vector, b: Vector) -> Vector {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// Half the rotation of the local offset (`this + 0x30`) by the quaternion `q` (`xyzw`),
/// exactly as the SSE code in `00cb3370`, `00cb3480` and `00cb3570` computes it:
/// `cross(q.xyz, offset) * w + (w * w + bias) * offset + dot(offset, q) * q`, the dot product
/// summed as `z + (y + x)`. Callers double it.
fn rotated_offset_half(e: &Engine, this: u32, q: Vector) -> Vector {
    let offset = load_vector(e, this + 0x30);
    let bias = load_vector(e, ROTATION_BIAS);
    let product = multiply(offset, q);
    let dot = product[2] + (product[1] + product[0]);
    let backward = multiply(
        [offset[1], offset[2], offset[0], offset[3]],
        [q[2], q[0], q[1], q[3]],
    );
    let forward = multiply(
        [offset[2], offset[0], offset[1], offset[3]],
        [q[1], q[2], q[0], q[3]],
    );
    let scalar = [q[3]; 4];
    let cross = multiply(subtract(forward, backward), scalar);
    let along = multiply(add_vectors(multiply(scalar, scalar), bias), offset);
    let toward = multiply([dot; 4], q);
    add_vectors(cross, add_vectors(along, toward))
}

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

fn count_up(e: &mut Engine, counter: u32) {
    let value: u32 = e.global(counter);
    e.set_global(counter, value.wrapping_add(1));
}

/// A 0x50-byte block from the memory manager (`00aa3e40`) aligned to 16 bytes: the shift
/// (1 to 16) is stored in the byte before the object.
fn allocate_aligned(e: &mut Engine) -> u32 {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x50u32])
        .u32();
    let shift = 0x10u8.wrapping_sub(block as u8 & 0xf);
    let object = block.wrapping_add(u32::from(shift));
    e.mem.set_u8(object.wrapping_sub(1), shift);
    object
}

/// Appends `text` to the viewer-string array: grows it by its grow-by (`0096ad30`) when the
/// size (`+0xa`) has reached the capacity (`+8`), then stores through `0096ae90` (array,
/// index = the old size, address of the pointer).
fn push_viewer_string(e: &mut Engine, array: u32, text: u32) {
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRINGS_GROW, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![array, size, slot]);
    });
}

// Translated from 00cb2ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb2ff0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRigidBodyT::~bhkRigidBodyT` (Xbox PDB): stores the class vtable, runs
/// `bhkRigidBody::Kill` (`00c8da40`), lowers the class counter (`012683c0`) and runs the
/// `bhkRigidBody` destructor body (`00c8e9c0`).
pub fn bhk_rigid_body_t_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    e.call(RIGID_BODY_KILL, &args![this]);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_sub(1));
    e.call(RIGID_BODY_DESTRUCT, &args![this]);
}

// Translated from 00cb3060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBodyT::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_83c8.
pub fn bhk_rigid_body_t_get_r_t_t_i(_e: &mut Engine) -> u32 {
    CLASS_RTTI
}

// Translated from 00cb3070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBodyT::LoadBinary` (Xbox PDB), `(stream)`: `bhkRigidBody::LoadBinary`
/// (`00c8ea30`), then the virtual `CreateCinfo` (slot `0xb4`) with a flag out-parameter whose
/// value is not used (the exe passes the address of its own stack argument).
pub fn bhk_rigid_body_t_load_binary(e: &mut Engine, this: Ptr, stream: u32) {
    e.call(RIGID_BODY_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, created| {
        e.vcall(this.addr(), VSLOT_CREATE_CINFO, &args![created]);
    });
}

// Translated from 00cb30a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBodyT::_scalar_deleting_destructor_` (Xbox PDB): the destructor body (`00cb2ff0`),
/// then, when bit 0 of `flags` is set and `this` is not null, the memory manager's
/// deallocation (`00aa4060` on `011f6238`) of the block that starts as many bytes before
/// `this` as the byte at `this - 1` says (the alignment shift). Answers `this`.
pub fn bhk_rigid_body_t_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    bhk_rigid_body_t_destructor(e, this);
    if flags & 1 != 0 && !this.is_null() {
        let shift = u32::from(e.mem.u8(this.addr().wrapping_sub(1)));
        e.call(
            MEMORY_DEALLOCATE,
            &args![MEMORY_MANAGER, this.addr().wrapping_sub(shift)],
        );
    }
    this
}

// Translated from 00cb30d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBodyT::CopyMembers` (Xbox PDB), `(clone, process)`: takes the cinfo (`CreateCinfo`,
/// virtual slot `0xb4`), gives it the local rotation (`+0x40`) and translation (`+0x30`) of
/// the body and, when the cloning scale (`process + 0x10`, copied through the x87 stack) is not
/// exactly 1.0 (a NaN is not), multiplies the cinfo's translation by it in all four lanes;
/// then `bhkRigidBody::CopyMembers` (`00c8f2a0`) on `(this, clone, process)`.
pub fn bhk_rigid_body_t_copy_members(e: &mut Engine, this: Ptr, clone: u32, process: u32) {
    let me = this.addr();
    let cinfo = e
        .with_stack(4, |e, created| {
            e.vcall(me, VSLOT_CREATE_CINFO, &args![created])
        })
        .u32();
    copy_vector(e, cinfo + 0x40, me + 0x20);
    copy_vector(e, cinfo + 0x30, me + 0x30);
    let scale = f32::from_bits(x87_quiet_bits(e.mem.u32(process + 0x10)));
    if scale != 1.0 {
        let translation = load_vector(e, cinfo + 0x30);
        store_vector(e, cinfo + 0x30, multiply([scale; 4], translation));
    }
    e.call(RIGID_BODY_COPY_MEMBERS, &args![this, clone, process]);
}

// Translated from 00cb3150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBodyT::SaveBinary` (Xbox PDB), `(stream)`: takes the cinfo (`CreateCinfo`, virtual
/// slot `0xb4`), gives it the local translation (`+0x30`) and rotation (`+0x40`) of the body,
/// runs `bhkRigidBody::SaveBinary` (`00c8d960`) and then the virtual at slot `0xa4` with the
/// word `CreateCinfo` answered through its out-parameter (the exe reads a whole dword of
/// which only the flag byte was written; the other bytes are zero here).
pub fn bhk_rigid_body_t_save_binary(e: &mut Engine, this: Ptr, stream: u32) {
    let me = this.addr();
    e.with_stack(4, |e, created| {
        let cinfo = e.vcall(me, VSLOT_CREATE_CINFO, &args![created]).u32();
        copy_vector(e, cinfo + 0x30, me + 0x30);
        copy_vector(e, cinfo + 0x40, me + 0x20);
        e.call(RIGID_BODY_SAVE_BINARY, &args![this, stream]);
        let flag = e.mem.u32(created.addr());
        e.vcall(me, VSLOT_RELEASE_CINFO, &args![flag]);
    });
}

// Translated from 00cb31a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBodyT::PostInit` (Xbox PDB), `(cinfo)`: takes the cinfo's translation (`+0x30`) and
/// rotation (`+0x40`) as the local offset (`this + 0x30`, `this + 0x20`), resets them in the
/// cinfo to the default vector (`01267e30`) and the identity quaternion (`010c71b0`), sets
/// the Havok body's position and rotation from them (`bhkRigidBody::SetPosition` and
/// `SetRotation`, `00c8d170` and `00c8d1a0`) and runs `bhkSerializable::PostInit` (`00c876d0`).
pub fn bhk_rigid_body_t_post_init(e: &mut Engine, this: Ptr, cinfo: u32) {
    let me = this.addr();
    copy_vector(e, me + 0x30, cinfo + 0x30);
    copy_vector(e, me + 0x20, cinfo + 0x40);
    copy_vector(e, cinfo + 0x30, DEFAULT_VECTOR);
    copy_vector(e, cinfo + 0x40, IDENTITY_QUATERNION);
    e.call(RIGID_BODY_SET_POSITION, &args![this, cinfo + 0x30]);
    e.call(RIGID_BODY_SET_ROTATION, &args![this, cinfo + 0x40]);
    e.call(SERIALIZABLE_POST_INIT, &args![this, cinfo]);
}

// Translated from 00cb3200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb3200 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRigidBodyT::GetRotation` (Xbox PDB), `(out)`: the body's rotation
/// (`bhkRigidBody::GetRotation_ov2`, `00c8d130`) times the inverse of the local rotation
/// (`+0x20`) through `00c74ac0`, into `out`. Answers `out`.
pub fn bhk_rigid_body_t_get_rotation(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.with_stack(0x10, |e, body_rotation| {
        e.call(RIGID_BODY_GET_ROTATION, &args![this, body_rotation]);
        e.call(
            QUATERNION_DIFFERENCE,
            &args![out, body_rotation, this.addr() + 0x20],
        );
    });
    out
}

// Translated from 00cb3250 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb3250 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRigidBodyT::SetRotation` (Xbox PDB), `(rotation)`: multiplies `rotation` by the local
/// rotation (`+0x20`) with `00c66320` and sets the result as the Havok body's rotation
/// (`bhkRigidBody::SetRotation`, `00c8d1a0`).
pub fn bhk_rigid_body_t_set_rotation(e: &mut Engine, this: Ptr, rotation: Ptr) {
    e.with_stack(0x10, |e, product| {
        e.call(
            QUATERNION_SET_MUL,
            &args![product, rotation, this.addr() + 0x20],
        );
        e.call(RIGID_BODY_SET_ROTATION, &args![this, product]);
    });
}

// Translated from 00cb32a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb32a0 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRigidBodyT::GetCenterOfMassInWorld` (Xbox PDB), `(out)`: `bhkRigidBody::
/// GetCenterOfMassInWorld` (`00c8d240`) into `out` (also called for a null `this`); with a
/// Havok body (`+8`) the local translation (`+0x30`) transformed by the body's transform
/// (`body + 0xf0`, `hkVector4::setMul3`, `00c7f750`) is subtracted from `out`. Answers `out`.
pub fn bhk_rigid_body_t_get_center_of_mass_in_world(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.call(RIGID_BODY_GET_CENTER_OF_MASS_IN_WORLD, &args![this, out]);
    if this.is_null() {
        return out;
    }
    let body = e.mem.u32(this.addr() + 8);
    if body == 0 {
        return out;
    }
    e.with_stack(0x10, |e, moved| {
        e.call(
            VECTOR_SET_MUL3,
            &args![moved, body + 0xf0, this.addr() + 0x30],
        );
        let centre = load_vector(e, out.addr());
        let offset = load_vector(e, moved.addr());
        store_vector(e, out.addr(), subtract(centre, offset));
    });
    out
}

// Translated from 00cb3310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb3310 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRigidBodyT::GetTransform` (Xbox PDB), `(out)`: the virtual `GetPosition` (slot `0xd4`)
/// into `out + 0x30` and the virtual `GetRotation` (slot `0xd8`) into a local quaternion,
/// which `00cb2d90` turns into the rotation columns at `out`. Answers `out`.
pub fn bhk_rigid_body_t_get_transform(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    e.vcall(this.addr(), VSLOT_GET_POSITION, &args![out.addr() + 0x30]);
    e.with_stack(0x10, |e, rotation| {
        e.vcall(this.addr(), VSLOT_GET_ROTATION, &args![rotation]);
        e.call(ROTATION_FROM_QUATERNION, &args![out, rotation]);
    });
    out
}

// Translated from 00cb3370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb3370 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRigidBodyT::GetPosition` (Xbox PDB), `(out)`: without a Havok body (`+8`) or `this`
/// the default vector (`01267e30`); otherwise the body's position
/// (`bhkRigidBody::GetPosition_ov2`, `00c8d0f0`) minus the local translation (`+0x30`)
/// rotated by the virtual `GetRotation` (slot `0xd8`) quaternion
/// ([`rotated_offset_half`] doubled). Answers `out`.
pub fn bhk_rigid_body_t_get_position(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    if this.is_null() || e.mem.u32(this.addr() + 8) == 0 {
        copy_vector(e, out.addr(), DEFAULT_VECTOR);
        return out;
    }
    let half = e.with_stack(0x10, |e, rotation| {
        e.vcall(this.addr(), VSLOT_GET_ROTATION, &args![rotation]);
        let q = load_vector(e, rotation.addr());
        rotated_offset_half(e, this.addr(), q)
    });
    e.call(RIGID_BODY_GET_POSITION, &args![this, out]);
    let position = load_vector(e, out.addr());
    store_vector(e, out.addr(), subtract(position, add_vectors(half, half)));
    out
}

// Translated from 00cb3480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb3480 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRigidBodyT::SetPosition` (Xbox PDB), `(position)`: nothing without a Havok body (`+8`)
/// or `this`; otherwise the given position plus the local translation (`+0x30`) rotated by
/// the virtual `GetRotation` (slot `0xd8`) quaternion ([`rotated_offset_half`] doubled) is set
/// with `bhkRigidBody::SetPosition` (`00c8d170`).
pub fn bhk_rigid_body_t_set_position(e: &mut Engine, this: Ptr, position: Ptr) {
    if this.is_null() || e.mem.u32(this.addr() + 8) == 0 {
        return;
    }
    e.with_stack(0x10, |e, rotation| {
        e.vcall(this.addr(), VSLOT_GET_ROTATION, &args![rotation]);
        let q = load_vector(e, rotation.addr());
        let half = rotated_offset_half(e, this.addr(), q);
        let target = add_vectors(add_vectors(half, half), load_vector(e, position.addr()));
        e.with_stack(0x10, |e, moved| {
            store_vector(e, moved.addr(), target);
            e.call(RIGID_BODY_SET_POSITION, &args![this, moved]);
        });
    });
}

// Translated from 00cb3570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb3570 - the /GS stack-cookie check (00ec408c) is not modeled.
/// `bhkRigidBodyT::SetPosRot` (Xbox PDB), `(position, rotation)`: the position plus the local
/// translation (`+0x30`) rotated by `rotation` ([`rotated_offset_half`] doubled), and the
/// product of `rotation` and the local rotation (`+0x20`, `00c66320`) normalised
/// (`005611c0`), are set with `bhkRigidBody::SetPosRot` (`00c8d1d0`).
pub fn bhk_rigid_body_t_set_pos_rot(e: &mut Engine, this: Ptr, position: Ptr, rotation: Ptr) {
    let q = load_vector(e, rotation.addr());
    let half = rotated_offset_half(e, this.addr(), q);
    let target = add_vectors(add_vectors(half, half), load_vector(e, position.addr()));
    e.with_stack(0x20, |e, scratch| {
        let moved = scratch.addr();
        let product = moved + 0x10;
        store_vector(e, moved, target);
        e.call(
            QUATERNION_SET_MUL,
            &args![product, rotation, this.addr() + 0x20],
        );
        e.call(QUATERNION_NORMALIZE, &args![product]);
        e.call(RIGID_BODY_SET_POS_ROT, &args![this, moved, product]);
    });
}

// Translated from 00cb3650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb3650 - the FS:[0] frame and the /GS stack-cookie check (00ec408c) are not modeled.
/// `bhkRigidBodyT::CreateDisplayGeometry` (Xbox PDB), `(node)`, with the allocation-scope word
/// at 0x10. The display node is `node` itself when its word at `+8` is 0; a null `node` gets a
/// new `NiNode` (0xac bytes from the game heap, constructor `00a5ecb0` with 0); any other
/// `node` gets such a new node attached as a child (virtual slot `0xdc`, with 1) and updated
/// (`00a59c60`, with a zeroed time and flags). The display node is named `"bhkRigidBodyT"`
/// (fixed string `00a5b690`, `00a5b950`; the handle is released unless it is the default
/// handle `0109b220`), gets the local rotation (`+0x20` through `00cb2d90` and `00624b10`)
/// as its `NiMatrix3` (`+0x34`) and the local translation (`+0x30` through `00458620`) as its
/// `NiPoint3` (`+0x58`), and is handed to `bhkRigidBody::CreateDisplayGeometry`
/// (`00c8d2f0`). The exe does not use that call's result but leaves it in `EAX`; it is
/// answered here.
pub fn bhk_rigid_body_t_create_display_geometry(e: &mut Engine, this: Ptr, parent: u32) -> u32 {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    let node = if parent == 0 {
        let block = e.call(HEAP_ALLOCATE, &args![0xacu32]).u32();
        if block == 0 {
            0
        } else {
            e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
        }
    } else if e.mem.u32(parent + 8) == 0 {
        parent
    } else {
        let block = e.call(HEAP_ALLOCATE, &args![0xacu32]).u32();
        let node = if block == 0 {
            0
        } else {
            e.call(NODE_CONSTRUCT, &args![block, 0u32]).u32()
        };
        e.vcall(parent, VSLOT_ATTACH_CHILD, &args![node, 1u32]);
        e.with_stack(0x10, |e, update| {
            e.call(NODE_UPDATE, &args![node, update]);
        });
        node
    };
    let name = e.call(FIXED_STRING_ASSIGN, &args![NAME_TEXT]).u32();
    e.with_stack(4, |e, handle| {
        e.mem.set_u32(handle.addr(), name);
        e.call(OBJECT_SET_NAME, &args![node, handle]);
        let handle = e.mem.u32(handle.addr());
        if handle != e.global::<u32>(FIXED_STRING_DEFAULT) {
            e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
        }
    });
    e.with_stack(0x30, |e, rotation| {
        e.call(ROTATION_FROM_QUATERNION, &args![rotation, me + 0x20]);
        e.with_stack(0x10, |e, vector| {
            copy_vector(e, vector.addr(), me + 0x30);
            e.with_stack(0x24, |e, matrix| {
                e.call(NI_MATRIX_FROM_ROTATION, &args![matrix, rotation]);
                e.with_stack(0xc, |e, point| {
                    e.call(NI_POINT3_FROM_VECTOR, &args![point, vector]);
                    for word in 0..9 {
                        let value = e.mem.u32(matrix.addr() + 4 * word);
                        e.mem.set_u32(node + 0x34 + 4 * word, value);
                    }
                    for axis in 0..3 {
                        let value = e.mem.u32(point.addr() + 4 * axis);
                        e.mem.set_u32(node + 0x58 + 4 * axis, value);
                    }
                });
            });
        });
    });
    let result = e
        .call(RIGID_BODY_CREATE_DISPLAY_GEOMETRY, &args![this, node])
        .u32();
    e.mem.set_u32(word, saved);
    result
}

// Translated from 00cb3860 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `bhkrigidbodyt.obj`). The `bhkRigidBodyT` constructor
/// without a body: the `bhkRefObject` constructor (`00c87c30`), then the vtable and object
/// counter of each class in turn (`bhkWorldObject` `01030394`/`0126810c`, `bhkEntity`
/// `010302bc`/`012681fc`, `bhkRigidBody` `010301b4`/`012681bc`, `bhkRigidBodyT`
/// `010c879c`/`012683c0`, RTTI-confirmed), clearing `+0xc`, `+0x10`, `+0x14`, `+0x18` (and
/// `+0x10` again). Answers `this`.
pub fn fn_00cb3860(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(me, VTABLE_WORLD_OBJECT);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me + 0xc, 0);
    count_up(e, COUNT_WORLD_OBJECT);
    e.mem.set_u32(me, VTABLE_ENTITY);
    count_up(e, COUNT_ENTITY);
    e.mem.set_u32(me, VTABLE_RIGID_BODY);
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x18, 0);
    count_up(e, COUNT_RIGID_BODY);
    e.mem.set_u32(me + 0x10, 0);
    e.mem.set_u32(me, VTABLE_CLASS);
    count_up(e, COUNT_CLASS);
    this
}

// Translated from 00cb38c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb38c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRigidBodyT::bhkRigidBodyT` (Xbox PDB), `(body)`: constructs the object as `00cb3860`
/// does (the vtable chain and counters), then takes over the Havok object of the
/// `bhkRigidBody` `body`: a reference is added to it (`00c90510`); the shape wrapper of its
/// Havok shape (`hkBody + 0x10` then `+8`, between empty locks) gets a reference
/// (`InterlockedIncrement`; none for no shape); `body`'s virtual `SethkObject` (slot `0x8c`)
/// runs with 0 and `this` gets the Havok object (`bhkSerializable::SethkObject`, `00c87760`);
/// the shape wrapper's reference is lowered again (`InterlockedDecrement`, virtual slot 4 at
/// 0; the exe does this also without a shape, which faults on address 4) and the Havok
/// object's added reference is removed (`00c905b0`). Answers `this`.
pub fn bhk_rigid_body_t_constructor(e: &mut Engine, this: Ptr, body: Ptr) -> Ptr {
    let me = this.addr();
    e.call(REF_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(me, VTABLE_WORLD_OBJECT);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0);
    count_up(e, COUNT_WORLD_OBJECT);
    e.mem.set_u32(me, VTABLE_ENTITY);
    count_up(e, COUNT_ENTITY);
    e.mem.set_u32(me, VTABLE_RIGID_BODY);
    e.mem.set_u32(me + 0x14, 0);
    e.mem.set_u32(me + 0x18, 0);
    count_up(e, COUNT_RIGID_BODY);
    e.mem.set_u32(me, VTABLE_CLASS);
    count_up(e, COUNT_CLASS);
    let havok_body = e.mem.u32(body.addr() + 8);
    e.call(HK_ADD_REFERENCE, &args![havok_body]);
    let mut shape = 0;
    if havok_body != 0 {
        e.call(EMPTY_LOCK, &args![body]);
        e.call(EMPTY_LOCK, &args![body]);
        let shape_slot = havok_body + 0x10;
        let havok_shape = e.mem.u32(shape_slot);
        if havok_shape != 0 {
            shape = e.mem.u32(havok_shape + 8);
            if shape != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![shape + 4]);
            }
        }
    }
    e.vcall(body.addr(), VSLOT_SET_HK_OBJECT, &args![0u32]);
    e.call(SERIALIZABLE_SET_HK_OBJECT, &args![this, havok_body]);
    if e.call(INTERLOCKED_DECREMENT, &args![shape + 4]).u32() == 0 {
        e.vcall(shape, 4, &args![]);
    }
    e.call(HK_REMOVE_REFERENCE, &args![havok_body]);
    this
}

// Translated from 00cb39d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb39d0 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name. Allocates a `bhkRigidBodyT` (0x50 bytes from the memory manager, aligned to
/// 16 bytes with the shift stored in the byte before the object) and constructs it with
/// `00cb3860`. Answers the object (the allocation result is not checked).
pub fn fn_00cb39d0(e: &mut Engine) -> Ptr {
    let object = allocate_aligned(e);
    fn_00cb3860(e, Ptr::new(object))
}

// Translated from 00cb3a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb3a40 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRigidBodyT::CreateClone` (Xbox PDB), `(process)`: with the allocation-scope word at
/// 0x10 and the clone lock (`00c876f0`, `00c87700`) held, calls `SetHavokAssertState(0)`
/// (virtual slot `0xfc`), builds an empty `bhkRigidBodyT` (as `00cb39d0`), runs
/// [`bhk_rigid_body_t_copy_members`] on `(clone, process)` and `SetHavokAssertState(1)`.
/// Answers the clone.
pub fn bhk_rigid_body_t_create_clone(e: &mut Engine, this: Ptr, process: u32) -> Ptr {
    let (word, saved) = enter_scope(e);
    e.call(CLONE_LOCK, &args![]);
    e.vcall(this.addr(), VSLOT_SET_HAVOK_ASSERT_STATE, &args![0u32]);
    let clone = fn_00cb39d0(e);
    bhk_rigid_body_t_copy_members(e, this, clone.addr(), process);
    e.vcall(this.addr(), VSLOT_SET_HAVOK_ASSERT_STATE, &args![1u32]);
    e.call(CLONE_UNLOCK, &args![]);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00cb3b20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb3b20 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkRigidBodyT::GetViewerStrings` (Xbox PDB), `(strings)`, with the allocation-scope word at
/// 0x10: `bhkRigidBody::GetViewerStrings` (`00c8f870`), then three more strings appended to
/// the array (`push_viewer_string`): the class name (`NiGetViewerString` on the word read from `012683c8`, the start of the class's
/// RTTI object), `"Local Rot"` for the local rotation as a `NiQuaternion` (`w` from `+0x2c`,
/// then `x`, `y`, `z` from `+0x20` .. `+0x28`, each copied through the x87 stack) and
/// `"Local Pos"` for the local translation as a `NiPoint3` (`00458620`).
pub fn bhk_rigid_body_t_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let me = this.addr();
    let (word, saved) = enter_scope(e);
    e.call(RIGID_BODY_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI);
    let class_text = e.call(VIEWER_STRING_CLASS, &args![class_name]).u32();
    push_viewer_string(e, strings.addr(), class_text);
    e.with_stack(0x1c, |e, scratch| {
        let point = scratch.addr();
        let quaternion = point + 0xc;
        // `NiQuaternion` order is w, x, y, z: the local rotation's x, y, z, w (`+0x20` ..
        // `+0x2c`) go to `+4`, `+8`, `+0xc` and `+0`.
        for (source, target) in [(0x20u32, 4u32), (0x24, 8), (0x28, 0xc), (0x2c, 0)] {
            let bits = x87_quiet_bits(e.mem.u32(me + source));
            e.mem.set_u32(quaternion + target, bits);
        }
        e.call(NI_POINT3_FROM_VECTOR, &args![point, me + 0x30]);
        let text = e
            .call(
                VIEWER_STRING_QUATERNION,
                &args![quaternion, NAME_LOCAL_ROTATION],
            )
            .u32();
        push_viewer_string(e, strings.addr(), text);
        let text = e
            .call(VIEWER_STRING_POINT, &args![point, NAME_LOCAL_POSITION])
            .u32();
        push_viewer_string(e, strings.addr(), text);
    });
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb2ff0, bhk_rigid_body_t_destructor(Ptr)),
        entry!(0x00cb3060, bhk_rigid_body_t_get_r_t_t_i() -> u32),
        entry!(0x00cb3070, bhk_rigid_body_t_load_binary(Ptr, u32)),
        entry!(
            0x00cb30a0,
            bhk_rigid_body_t_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00cb30d0, bhk_rigid_body_t_copy_members(Ptr, u32, u32)),
        entry!(0x00cb3150, bhk_rigid_body_t_save_binary(Ptr, u32)),
        entry!(0x00cb31a0, bhk_rigid_body_t_post_init(Ptr, u32)),
        entry!(0x00cb3200, bhk_rigid_body_t_get_rotation(Ptr, Ptr) -> Ptr),
        entry!(0x00cb3250, bhk_rigid_body_t_set_rotation(Ptr, Ptr)),
        entry!(
            0x00cb32a0,
            bhk_rigid_body_t_get_center_of_mass_in_world(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00cb3310, bhk_rigid_body_t_get_transform(Ptr, Ptr) -> Ptr),
        entry!(0x00cb3370, bhk_rigid_body_t_get_position(Ptr, Ptr) -> Ptr),
        entry!(0x00cb3480, bhk_rigid_body_t_set_position(Ptr, Ptr)),
        entry!(0x00cb3570, bhk_rigid_body_t_set_pos_rot(Ptr, Ptr, Ptr)),
        entry!(
            0x00cb3650,
            bhk_rigid_body_t_create_display_geometry(Ptr, u32) -> u32
        ),
        entry!(0x00cb3860, fn_00cb3860(Ptr) -> Ptr),
        entry!(0x00cb38c0, bhk_rigid_body_t_constructor(Ptr, Ptr) -> Ptr),
        entry!(0x00cb39d0, fn_00cb39d0() -> Ptr),
        entry!(0x00cb3a40, bhk_rigid_body_t_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00cb3b20, bhk_rigid_body_t_get_viewer_strings(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const DELETE: u32 = 0x7fff_0004;
    const SET_HK_OBJECT_SLOT: u32 = 0x7fff_008c;
    const RELEASE_CINFO_SLOT: u32 = 0x7fff_00a4;
    const CREATE_CINFO_SLOT: u32 = 0x7fff_00b4;
    const GET_POSITION_SLOT: u32 = 0x7fff_00d4;
    const GET_ROTATION_SLOT: u32 = 0x7fff_00d8;
    const ATTACH_CHILD_SLOT: u32 = 0x7fff_00dc;
    const ASSERT_STATE_SLOT: u32 = 0x7fff_00fc;

    /// The arguments of each call a recording double saw, with the 16-byte vectors found at
    /// the argument indices it was asked to read.
    type Seen = Rc<RefCell<Vec<(Vec<u32>, Vec<Vector>)>>>;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn vtable_with(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        table
    }

    fn object_with(e: &mut Engine, table: u32) -> u32 {
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, table);
        object
    }

    /// Registers a double at `address` that answers `answer` and records the arguments and the
    /// vectors at the argument indices `vectors`.
    fn record_vectors(e: &mut Engine, address: u32, vectors: Vec<usize>, answer: u32) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(vec![]));
        let sink = Rc::clone(&seen);
        e.register_double(address, move |e, a| {
            let read = vectors
                .iter()
                .map(|index| load_vector(e, a[*index]))
                .collect();
            sink.borrow_mut().push((a.to_vec(), read));
            answer.into_ret()
        });
        seen
    }

    /// A `bhkRigidBodyT`-shaped object (vtable with the given virtual slots) with a Havok body
    /// at `+8`, the local rotation at `+0x20` and the local translation at `+0x30`.
    fn body_t(e: &mut Engine, slots: &[(u32, u32)], rotation: Vector, offset: Vector) -> u32 {
        let table = vtable_with(e, slots);
        let this = object_with(e, table);
        let havok = e.mem.alloc(0x200);
        e.mem.set_u32(this + 8, havok);
        store_vector(e, this + 0x20, rotation);
        store_vector(e, this + 0x30, offset);
        this
    }

    /// The vector the rotated-offset code adds to `w * w` (-0.5 in every lane in the exe).
    fn set_bias(e: &mut Engine) {
        store_vector(e, ROTATION_BIAS, [-0.5; 4]);
    }

    fn interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            value.into_ret()
        });
    }

    #[test]
    fn bhk_rigid_body_t_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cb_3060, &args![]).u32(), 0x0126_83c8);
    }

    #[test]
    fn bhk_rigid_body_t_destructor_kills_counts_down_and_destructs() {
        let mut e = logged_engine();
        e.register(RIGID_BODY_KILL, |_, _| Ret::default());
        e.register(RIGID_BODY_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        e.set_global(COUNT_CLASS, 5u32);
        e.call(0x00cb2ff0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 4);
        assert_eq!(calls_to(&e, RIGID_BODY_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, RIGID_BODY_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn bhk_rigid_body_t_load_binary_loads_the_body_then_creates_the_cinfo() {
        let mut e = logged_engine();
        e.register(RIGID_BODY_LOAD_BINARY, |_, _| Ret::default());
        e.register(CREATE_CINFO_SLOT, |_, _| Ret::default());
        let this = body_t(&mut e, &[(0xb4, CREATE_CINFO_SLOT)], [0.0; 4], [0.0; 4]);
        e.call(0x00cb3070, &args![this, 0x9000u32]);
        assert_eq!(
            calls_to(&e, RIGID_BODY_LOAD_BINARY),
            vec![vec![this, 0x9000]]
        );
        let created = calls_to(&e, CREATE_CINFO_SLOT);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][0], this);
    }

    #[test]
    fn bhk_rigid_body_t_scalar_deleting_destructor_frees_the_aligned_block_on_bit_0() {
        let mut e = logged_engine();
        e.register(RIGID_BODY_KILL, |_, _| Ret::default());
        e.register(RIGID_BODY_DESTRUCT, |_, _| Ret::default());
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let block = e.mem.alloc(0x60);
        let this = block + 5;
        e.mem.set_u8(this - 1, 5);
        assert_eq!(e.call(0x00cb30a0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        assert_eq!(e.call(0x00cb30a0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block]]
        );
        assert_eq!(calls_to(&e, RIGID_BODY_DESTRUCT).len(), 2);
    }

    #[test]
    fn bhk_rigid_body_t_copy_members_gives_the_cinfo_the_offset_and_scales_the_translation() {
        let mut e = logged_engine();
        e.register(RIGID_BODY_COPY_MEMBERS, |_, _| Ret::default());
        let cinfo = e.mem.alloc(0x60);
        e.register_double(CREATE_CINFO_SLOT, move |_, _| cinfo.into_ret());
        let this = body_t(
            &mut e,
            &[(0xb4, CREATE_CINFO_SLOT)],
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
        );
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 1.0);
        e.call(0x00cb30d0, &args![this, 0x7000u32, process]);
        assert_eq!(load_vector(&e, cinfo + 0x40), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vector(&e, cinfo + 0x30), [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_f32(process + 0x10, 2.0);
        e.call(0x00cb30d0, &args![this, 0x7000u32, process]);
        assert_eq!(load_vector(&e, cinfo + 0x40), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vector(&e, cinfo + 0x30), [10.0, 12.0, 14.0, 16.0]);
        // A NaN scale is not 1.0 either.
        e.mem.set_u32(process + 0x10, 0x7fa0_0001);
        e.call(0x00cb30d0, &args![this, 0x7000u32, process]);
        assert!(load_vector(&e, cinfo + 0x30)
            .iter()
            .all(|lane| lane.is_nan()));
        assert_eq!(
            calls_to(&e, RIGID_BODY_COPY_MEMBERS),
            vec![vec![this, 0x7000, process]; 3]
        );
    }

    #[test]
    fn bhk_rigid_body_t_save_binary_gives_the_cinfo_the_offset_and_releases_it() {
        let mut e = logged_engine();
        e.register(RIGID_BODY_SAVE_BINARY, |_, _| Ret::default());
        e.register(RELEASE_CINFO_SLOT, |_, _| Ret::default());
        let cinfo = e.mem.alloc(0x60);
        e.register_double(CREATE_CINFO_SLOT, move |e, a| {
            e.mem.set_u8(a[1], 1);
            cinfo.into_ret()
        });
        let this = body_t(
            &mut e,
            &[(0xb4, CREATE_CINFO_SLOT), (0xa4, RELEASE_CINFO_SLOT)],
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
        );
        e.call(0x00cb3150, &args![this, 0x9000u32]);
        assert_eq!(load_vector(&e, cinfo + 0x40), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vector(&e, cinfo + 0x30), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(
            calls_to(&e, RIGID_BODY_SAVE_BINARY),
            vec![vec![this, 0x9000]]
        );
        assert_eq!(calls_to(&e, RELEASE_CINFO_SLOT), vec![vec![this, 1]]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| {
                [
                    CREATE_CINFO_SLOT,
                    RIGID_BODY_SAVE_BINARY,
                    RELEASE_CINFO_SLOT,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                CREATE_CINFO_SLOT,
                RIGID_BODY_SAVE_BINARY,
                RELEASE_CINFO_SLOT
            ]
        );
    }

    #[test]
    fn bhk_rigid_body_t_post_init_takes_the_offset_from_the_cinfo() {
        let mut e = logged_engine();
        e.register(RIGID_BODY_SET_POSITION, |_, _| Ret::default());
        e.register(RIGID_BODY_SET_ROTATION, |_, _| Ret::default());
        e.register(SERIALIZABLE_POST_INIT, |_, _| Ret::default());
        store_vector(&mut e, DEFAULT_VECTOR, [0.0; 4]);
        store_vector(&mut e, IDENTITY_QUATERNION, [0.0, 0.0, 0.0, 1.0]);
        let this = body_t(&mut e, &[], [9.0; 4], [9.0; 4]);
        let cinfo = e.mem.alloc(0x60);
        store_vector(&mut e, cinfo + 0x30, [5.0, 6.0, 7.0, 8.0]);
        store_vector(&mut e, cinfo + 0x40, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x00cb31a0, &args![this, cinfo]);
        assert_eq!(load_vector(&e, this + 0x30), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(load_vector(&e, this + 0x20), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vector(&e, cinfo + 0x30), [0.0; 4]);
        assert_eq!(load_vector(&e, cinfo + 0x40), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(
            calls_to(&e, RIGID_BODY_SET_POSITION),
            vec![vec![this, cinfo + 0x30]]
        );
        assert_eq!(
            calls_to(&e, RIGID_BODY_SET_ROTATION),
            vec![vec![this, cinfo + 0x40]]
        );
        assert_eq!(
            calls_to(&e, SERIALIZABLE_POST_INIT),
            vec![vec![this, cinfo]]
        );
    }

    #[test]
    fn bhk_rigid_body_t_get_rotation_removes_the_local_rotation_from_the_body_rotation() {
        let mut e = logged_engine();
        let rotation = record_vectors(&mut e, RIGID_BODY_GET_ROTATION, vec![], 0);
        e.register(QUATERNION_DIFFERENCE, |_, _| Ret::default());
        let this = body_t(&mut e, &[], [1.0, 2.0, 3.0, 4.0], [0.0; 4]);
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00cb3200, &args![this, out]).u32(), out);
        let body_rotation = rotation.borrow()[0].0[1];
        assert_eq!(rotation.borrow()[0].0[0], this);
        assert_eq!(
            calls_to(&e, QUATERNION_DIFFERENCE),
            vec![vec![out, body_rotation, this + 0x20]]
        );
    }

    #[test]
    fn bhk_rigid_body_t_set_rotation_multiplies_by_the_local_rotation() {
        let mut e = logged_engine();
        e.register(QUATERNION_SET_MUL, |_, _| Ret::default());
        e.register(RIGID_BODY_SET_ROTATION, |_, _| Ret::default());
        let this = body_t(&mut e, &[], [1.0, 2.0, 3.0, 4.0], [0.0; 4]);
        e.call(0x00cb3250, &args![this, 0x6000u32]);
        let products = calls_to(&e, QUATERNION_SET_MUL);
        assert_eq!(products.len(), 1);
        let product = products[0][0];
        assert_eq!(products[0][1..], [0x6000, this + 0x20]);
        assert_eq!(
            calls_to(&e, RIGID_BODY_SET_ROTATION),
            vec![vec![this, product]]
        );
    }

    #[test]
    fn bhk_rigid_body_t_get_center_of_mass_in_world_subtracts_the_transformed_offset() {
        let mut e = logged_engine();
        e.register(RIGID_BODY_GET_CENTER_OF_MASS_IN_WORLD, |e, a| {
            store_vector(e, a[1], [10.0, 20.0, 30.0, 0.0]);
            Ret::default()
        });
        e.register(VECTOR_SET_MUL3, |e, a| {
            store_vector(e, a[0], [1.0, 2.0, 3.0, 0.0]);
            Ret::default()
        });
        let this = body_t(&mut e, &[], [0.0; 4], [4.0, 5.0, 6.0, 0.0]);
        let havok = e.mem.u32(this + 8);
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00cb32a0, &args![this, out]).u32(), out);
        assert_eq!(load_vector(&e, out), [9.0, 18.0, 27.0, 0.0]);
        let moved = calls_to(&e, VECTOR_SET_MUL3);
        assert_eq!(moved.len(), 1);
        assert_eq!(moved[0][1..], [havok + 0xf0, this + 0x30]);
        // Without a Havok body nothing is subtracted.
        e.mem.set_u32(this + 8, 0);
        assert_eq!(e.call(0x00cb32a0, &args![this, out]).u32(), out);
        assert_eq!(load_vector(&e, out), [10.0, 20.0, 30.0, 0.0]);
        assert_eq!(calls_to(&e, VECTOR_SET_MUL3).len(), 1);
    }

    #[test]
    fn bhk_rigid_body_t_get_transform_builds_the_rotation_and_the_position() {
        let mut e = logged_engine();
        e.register(GET_POSITION_SLOT, |_, _| Ret::default());
        e.register(GET_ROTATION_SLOT, |_, _| Ret::default());
        e.register(ROTATION_FROM_QUATERNION, |_, _| Ret::default());
        let this = body_t(
            &mut e,
            &[(0xd4, GET_POSITION_SLOT), (0xd8, GET_ROTATION_SLOT)],
            [0.0; 4],
            [0.0; 4],
        );
        let out = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00cb3310, &args![this, out]).u32(), out);
        assert_eq!(
            calls_to(&e, GET_POSITION_SLOT),
            vec![vec![this, out + 0x30]]
        );
        let rotation = calls_to(&e, GET_ROTATION_SLOT);
        assert_eq!(rotation.len(), 1);
        assert_eq!(
            calls_to(&e, ROTATION_FROM_QUATERNION),
            vec![vec![out, rotation[0][1]]]
        );
    }

    /// A body whose rotation is the half turn about z `(0, 0, 1, 0)` and whose local
    /// translation is `(1, 2, 3, 0)`: the rotated offset is `(-1, -2, 3, 0)`.
    fn half_turn_body(e: &mut Engine) -> u32 {
        set_bias(e);
        e.register(GET_ROTATION_SLOT, |e, a| {
            store_vector(e, a[1], [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        body_t(
            e,
            &[(0xd8, GET_ROTATION_SLOT)],
            [0.0; 4],
            [1.0, 2.0, 3.0, 0.0],
        )
    }

    #[test]
    fn bhk_rigid_body_t_get_position_subtracts_the_rotated_offset() {
        let mut e = logged_engine();
        e.register(RIGID_BODY_GET_POSITION, |e, a| {
            store_vector(e, a[1], [10.0, 20.0, 30.0, 0.0]);
            Ret::default()
        });
        let this = half_turn_body(&mut e);
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00cb3370, &args![this, out]).u32(), out);
        assert_eq!(load_vector(&e, out), [11.0, 22.0, 27.0, 0.0]);
        assert_eq!(calls_to(&e, RIGID_BODY_GET_POSITION), vec![vec![this, out]]);
        // The identity rotation subtracts the offset itself.
        e.register(GET_ROTATION_SLOT, |e, a| {
            store_vector(e, a[1], [0.0, 0.0, 0.0, 1.0]);
            Ret::default()
        });
        store_vector(&mut e, this + 0x30, [1.0, 2.0, 3.0, 4.0]);
        e.register(RIGID_BODY_GET_POSITION, |e, a| {
            store_vector(e, a[1], [10.0, 20.0, 30.0, 20.0]);
            Ret::default()
        });
        e.call(0x00cb3370, &args![this, out]);
        assert_eq!(load_vector(&e, out), [9.0, 18.0, 27.0, 16.0]);
        // Without a Havok body or `this`: the default vector.
        store_vector(&mut e, DEFAULT_VECTOR, [7.0, 8.0, 9.0, 10.0]);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00cb3370, &args![this, out]);
        assert_eq!(load_vector(&e, out), [7.0, 8.0, 9.0, 10.0]);
        store_vector(&mut e, out, [0.0; 4]);
        assert_eq!(e.call(0x00cb3370, &args![0u32, out]).u32(), out);
        assert_eq!(load_vector(&e, out), [7.0, 8.0, 9.0, 10.0]);
        assert_eq!(calls_to(&e, RIGID_BODY_GET_POSITION).len(), 2);
    }

    #[test]
    fn bhk_rigid_body_t_set_position_adds_the_rotated_offset() {
        let mut e = logged_engine();
        let set = record_vectors(&mut e, RIGID_BODY_SET_POSITION, vec![1], 0);
        let this = half_turn_body(&mut e);
        let position = e.mem.alloc(0x10);
        store_vector(&mut e, position, [10.0, 20.0, 30.0, 0.0]);
        e.call(0x00cb3480, &args![this, position]);
        let seen = set.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[0], this);
        assert_eq!(seen[0].1[0], [9.0, 18.0, 33.0, 0.0]);
        drop(seen);
        // Nothing happens without a Havok body or `this`.
        e.mem.set_u32(this + 8, 0);
        e.call(0x00cb3480, &args![this, position]);
        e.call(0x00cb3480, &args![0u32, position]);
        assert_eq!(set.borrow().len(), 1);
    }

    #[test]
    fn bhk_rigid_body_t_set_pos_rot_moves_by_the_rotated_offset_and_normalises() {
        let mut e = logged_engine();
        set_bias(&mut e);
        e.register(QUATERNION_SET_MUL, |e, a| {
            store_vector(e, a[0], [0.0, 0.0, 3.0, 4.0]);
            Ret::default()
        });
        e.register(QUATERNION_NORMALIZE, |_, _| Ret::default());
        let set = record_vectors(&mut e, RIGID_BODY_SET_POS_ROT, vec![1, 2], 0);
        let this = body_t(&mut e, &[], [1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 0.0]);
        let position = e.mem.alloc(0x10);
        store_vector(&mut e, position, [10.0, 20.0, 30.0, 0.0]);
        let rotation = e.mem.alloc(0x10);
        store_vector(&mut e, rotation, [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00cb3570, &args![this, position, rotation]);
        let seen = set.borrow();
        assert_eq!(seen.len(), 1);
        let product = seen[0].0[2];
        assert_eq!(seen[0].0[0], this);
        assert_eq!(seen[0].1[0], [9.0, 18.0, 33.0, 0.0]);
        assert_eq!(seen[0].1[1], [0.0, 0.0, 3.0, 4.0]);
        assert_eq!(
            calls_to(&e, QUATERNION_SET_MUL),
            vec![vec![product, rotation, this + 0x20]]
        );
        assert_eq!(calls_to(&e, QUATERNION_NORMALIZE), vec![vec![product]]);
    }

    /// Doubles for everything `00cb3650` calls outside the file; the display node is given
    /// the matrix words 1 to 9 and the translation words of the vector.
    fn display_doubles(e: &mut Engine) {
        const MATRIX_WORDS: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        interlocked(e);
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NODE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(NODE_UPDATE, |_, _| Ret::default());
        e.register(FIXED_STRING_ASSIGN, |e, _| {
            let block = e.mem.alloc(0x10);
            e.mem.set_u32(block, 5);
            (block + 8).into_ret()
        });
        e.register(OBJECT_SET_NAME, |_, _| Ret::default());
        e.register(ROTATION_FROM_QUATERNION, |_, _| Ret::default());
        e.register(NI_MATRIX_FROM_ROTATION, |e, a| {
            for (index, word) in MATRIX_WORDS.into_iter().enumerate() {
                e.mem.set_u32(a[0] + 4 * index as u32, word);
            }
            Ret::default()
        });
        e.register(NI_POINT3_FROM_VECTOR, |e, a| {
            for axis in 0..3 {
                let value = e.mem.u32(a[1] + 4 * axis);
                e.mem.set_u32(a[0] + 4 * axis, value);
            }
            Ret::default()
        });
        e.register(RIGID_BODY_CREATE_DISPLAY_GEOMETRY, |e, _| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            0x1234u32.into_ret()
        });
    }

    #[test]
    fn bhk_rigid_body_t_create_display_geometry_builds_or_reuses_the_display_node() {
        let mut e = logged_engine();
        display_doubles(&mut e);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 7);
        let this = body_t(&mut e, &[], [0.0; 4], [1.5, 2.5, 3.5, 4.5]);
        // No node given: a new one.
        assert_eq!(e.call(0x00cb3650, &args![this, 0u32]).u32(), 0x1234);
        assert_eq!(e.mem.u32(word), 7);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0xac]]);
        let constructed = calls_to(&e, NODE_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        assert_eq!(constructed[0][1], 0);
        let node = constructed[0][0];
        assert_eq!(
            calls_to(&e, RIGID_BODY_CREATE_DISPLAY_GEOMETRY),
            vec![vec![this, node]]
        );
        let words: Vec<u32> = (0..9)
            .map(|index| e.mem.u32(node + 0x34 + 4 * index))
            .collect();
        assert_eq!(words, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(e.mem.f32(node + 0x58), 1.5);
        assert_eq!(e.mem.f32(node + 0x5c), 2.5);
        assert_eq!(e.mem.f32(node + 0x60), 3.5);
        // The name handle goes through `SetName` and is released.
        let names = calls_to(&e, OBJECT_SET_NAME);
        assert_eq!(names.len(), 1);
        assert_eq!(names[0][0], node);
        assert_eq!(calls_to(&e, FIXED_STRING_ASSIGN), vec![vec![NAME_TEXT]]);
        let released = calls_to(&e, INTERLOCKED_DECREMENT);
        assert_eq!(released.len(), 1);
        assert_eq!(e.mem.u32(released[0][0]), 4);
        // A node without a name word is reused as it is.
        let given = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cb3650, &args![this, given]).u32(), 0x1234);
        assert!(calls_to(&e, HEAP_ALLOCATE).is_empty());
        assert_eq!(
            calls_to(&e, RIGID_BODY_CREATE_DISPLAY_GEOMETRY),
            vec![vec![this, given]]
        );
        assert_eq!(e.mem.u32(given + 0x34), 1);
        // The default name handle is not released.
        e.call_log = Some(vec![]);
        e.register(FIXED_STRING_ASSIGN, |e, _| {
            let block = e.mem.alloc(0x10);
            e.set_global(FIXED_STRING_DEFAULT, block + 8);
            (block + 8).into_ret()
        });
        e.call(0x00cb3650, &args![this, given]);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn bhk_rigid_body_t_create_display_geometry_attaches_a_new_node_to_a_named_one() {
        let mut e = logged_engine();
        display_doubles(&mut e);
        e.register(ATTACH_CHILD_SLOT, |_, _| Ret::default());
        let this = body_t(&mut e, &[], [0.0; 4], [1.5, 2.5, 3.5, 4.5]);
        let table = vtable_with(&mut e, &[(0xdc, ATTACH_CHILD_SLOT)]);
        let parent = object_with(&mut e, table);
        e.mem.set_u32(parent + 8, 0x4000);
        e.call(0x00cb3650, &args![this, parent]);
        let constructed = calls_to(&e, NODE_CONSTRUCT);
        assert_eq!(constructed.len(), 1);
        let node = constructed[0][0];
        assert_eq!(calls_to(&e, ATTACH_CHILD_SLOT), vec![vec![parent, node, 1]]);
        let updates = calls_to(&e, NODE_UPDATE);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0][0], node);
        assert_eq!(
            calls_to(&e, RIGID_BODY_CREATE_DISPLAY_GEOMETRY),
            vec![vec![this, node]]
        );
    }

    #[test]
    fn fn_00cb3860_constructs_a_rigid_body_t() {
        let mut e = logged_engine();
        e.register(REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        let counters = [
            COUNT_WORLD_OBJECT,
            COUNT_ENTITY,
            COUNT_RIGID_BODY,
            COUNT_CLASS,
        ];
        for counter in counters {
            e.set_global(counter, 10u32);
        }
        let this = e.mem.alloc(0x60);
        for offset in [0xc, 0x10, 0x14, 0x18] {
            e.mem.set_u32(this + offset, 0xdead);
        }
        assert_eq!(e.call(0x00cb3860, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        for offset in [0xc, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for counter in counters {
            assert_eq!(e.global::<u32>(counter), 11);
        }
        assert_eq!(calls_to(&e, REF_OBJECT_CONSTRUCT), vec![vec![this]]);
    }

    /// A body with a Havok body that has a shape whose wrapper object has `references`
    /// references and the virtual slot 4 [`DELETE`].
    fn body_with_shape(e: &mut Engine, references: u32) -> (u32, u32, u32) {
        let table = vtable_with(e, &[(0x8c, SET_HK_OBJECT_SLOT)]);
        let body = object_with(e, table);
        let havok = e.mem.alloc(0x40);
        let havok_shape = e.mem.alloc(0x20);
        let wrapper_table = vtable_with(e, &[(4, DELETE)]);
        let wrapper = object_with(e, wrapper_table);
        e.mem.set_u32(wrapper + 4, references);
        e.mem.set_u32(havok_shape + 8, wrapper);
        e.mem.set_u32(havok + 0x10, havok_shape);
        e.mem.set_u32(body + 8, havok);
        (body, havok, wrapper)
    }

    fn constructor_doubles(e: &mut Engine) {
        interlocked(e);
        e.register(REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(HK_ADD_REFERENCE, |_, _| Ret::default());
        e.register(HK_REMOVE_REFERENCE, |_, _| Ret::default());
        e.register(EMPTY_LOCK, |_, _| Ret::default());
        e.register(SET_HK_OBJECT_SLOT, |_, _| Ret::default());
        e.register(SERIALIZABLE_SET_HK_OBJECT, |_, _| Ret::default());
        e.register(DELETE, |_, _| Ret::default());
    }

    #[test]
    fn bhk_rigid_body_t_constructor_takes_over_the_havok_body_of_another_body() {
        let mut e = logged_engine();
        constructor_doubles(&mut e);
        let (body, havok, wrapper) = body_with_shape(&mut e, 2);
        let this = e.mem.alloc(0x60);
        e.set_global(COUNT_CLASS, 3u32);
        assert_eq!(e.call(0x00cb38c0, &args![this, body]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 4);
        assert_eq!(calls_to(&e, HK_ADD_REFERENCE), vec![vec![havok]]);
        assert_eq!(calls_to(&e, EMPTY_LOCK), vec![vec![body]; 2]);
        assert_eq!(calls_to(&e, SET_HK_OBJECT_SLOT), vec![vec![body, 0]]);
        assert_eq!(
            calls_to(&e, SERIALIZABLE_SET_HK_OBJECT),
            vec![vec![this, havok]]
        );
        assert_eq!(calls_to(&e, HK_REMOVE_REFERENCE), vec![vec![havok]]);
        // The wrapper got a reference and gave it back.
        assert_eq!(e.mem.u32(wrapper + 4), 2);
        assert!(calls_to(&e, DELETE).is_empty());
        // The last reference of the wrapper deletes it.
        let (body, _, wrapper) = body_with_shape(&mut e, 0);
        e.call(0x00cb38c0, &args![this, body]);
        assert_eq!(calls_to(&e, DELETE), vec![vec![wrapper]]);
    }

    #[test]
    #[should_panic(expected = "read of unmapped memory at 00000004")]
    fn bhk_rigid_body_t_constructor_faults_on_a_body_without_a_shape() {
        let mut e = logged_engine();
        constructor_doubles(&mut e);
        let (body, havok, _) = body_with_shape(&mut e, 2);
        e.mem.set_u32(havok + 0x10, 0);
        let this = e.mem.alloc(0x60);
        e.call(0x00cb38c0, &args![this, body]);
    }

    #[test]
    fn fn_00cb39d0_allocates_an_aligned_block_and_constructs_it() {
        let mut e = logged_engine();
        e.register(REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        let area = e.mem.alloc(0x200);
        let aligned = (area + 0xf) & !0xf;
        e.register_double(MEMORY_ALLOCATE, move |_, _| (aligned + 5).into_ret());
        let object = e.call(0x00cb39d0, &args![]).u32();
        assert_eq!(object, aligned + 5 + 11);
        assert_eq!(e.mem.u8(object - 1), 11);
        assert_eq!(e.mem.u32(object), VTABLE_CLASS);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x50]]
        );
        // An allocation that is aligned already is shifted by 16.
        e.register_double(MEMORY_ALLOCATE, move |_, _| aligned.into_ret());
        let object = e.call(0x00cb39d0, &args![]).u32();
        assert_eq!(object, aligned + 0x10);
        assert_eq!(e.mem.u8(object - 1), 0x10);
    }

    #[test]
    fn bhk_rigid_body_t_create_clone_copies_the_members_into_a_new_body_under_the_lock() {
        let mut e = logged_engine();
        e.register(CLONE_LOCK, |_, _| Ret::default());
        e.register(CLONE_UNLOCK, |_, _| Ret::default());
        e.register(REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(RIGID_BODY_COPY_MEMBERS, |_, _| Ret::default());
        e.register(CREATE_CINFO_SLOT, |e, _| e.mem.alloc(0x60).into_ret());
        e.register(ASSERT_STATE_SLOT, |e, _| {
            let word = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(word), 0x10);
            Ret::default()
        });
        let area = e.mem.alloc(0x200);
        let aligned = (area + 0xf) & !0xf;
        e.register_double(MEMORY_ALLOCATE, move |_, _| aligned.into_ret());
        let this = body_t(
            &mut e,
            &[(0xb4, CREATE_CINFO_SLOT), (0xfc, ASSERT_STATE_SLOT)],
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
        );
        let process = e.mem.alloc(0x20);
        e.mem.set_f32(process + 0x10, 1.0);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 3);
        let clone = e.call(0x00cb3a40, &args![this, process]).u32();
        assert_eq!(clone, aligned + 0x10);
        assert_eq!(e.mem.u32(word), 3);
        assert_eq!(e.mem.u32(clone), VTABLE_CLASS);
        assert_eq!(
            calls_to(&e, RIGID_BODY_COPY_MEMBERS),
            vec![vec![this, clone, process]]
        );
        assert_eq!(
            calls_to(&e, ASSERT_STATE_SLOT),
            vec![vec![this, 0], vec![this, 1]]
        );
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| {
                [
                    CLONE_LOCK,
                    ASSERT_STATE_SLOT,
                    MEMORY_ALLOCATE,
                    RIGID_BODY_COPY_MEMBERS,
                    CLONE_UNLOCK,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                CLONE_LOCK,
                ASSERT_STATE_SLOT,
                MEMORY_ALLOCATE,
                RIGID_BODY_COPY_MEMBERS,
                ASSERT_STATE_SLOT,
                CLONE_UNLOCK
            ]
        );
    }

    #[test]
    fn bhk_rigid_body_t_get_viewer_strings_appends_the_class_rotation_and_position() {
        let mut e = logged_engine();
        e.register(RIGID_BODY_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_CLASS, |_, _| 0xaaau32.into_ret());
        e.mem.set_u32(CLASS_RTTI, 0x4242);
        e.register(NI_POINT3_FROM_VECTOR, |e, a| {
            for axis in 0..3 {
                let value = e.mem.u32(a[1] + 4 * axis);
                e.mem.set_u32(a[0] + 4 * axis, value);
            }
            Ret::default()
        });
        let rotation = record_vectors(&mut e, VIEWER_STRING_QUATERNION, vec![0], 0xbbb);
        let position = record_vectors(&mut e, VIEWER_STRING_POINT, vec![0], 0xccc);
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 8, 5);
            Ret::default()
        });
        let stored = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&stored);
        e.register_double(STRINGS_SET_AT, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.u32(a[2])));
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let this = body_t(&mut e, &[], [0.0; 4], [1.5, 2.5, 3.5, 4.5]);
        e.mem.set_u32(this + 0x20, 0x7fa0_0001);
        e.mem.set_f32(this + 0x24, 2.0);
        e.mem.set_f32(this + 0x28, 3.0);
        e.mem.set_f32(this + 0x2c, 4.0);
        let strings = e.mem.alloc(0x20);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 4);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 9);
        e.call(0x00cb3b20, &args![this, strings]);
        assert_eq!(e.mem.u32(word), 9);
        assert_eq!(
            calls_to(&e, RIGID_BODY_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING_CLASS), vec![vec![0x4242]]);
        // The quaternion is in `w x y z` order, the signalling NaN of `x` quieted.
        let seen = rotation.borrow();
        assert_eq!(seen[0].0[1], NAME_LOCAL_ROTATION);
        let quaternion = seen[0].1[0];
        assert_eq!(quaternion[0], 4.0);
        assert_eq!(quaternion[1].to_bits(), 0x7fe0_0001);
        assert_eq!(quaternion[2], 2.0);
        assert_eq!(quaternion[3], 3.0);
        let seen = position.borrow();
        assert_eq!(seen[0].0[1], NAME_LOCAL_POSITION);
        assert_eq!(seen[0].1[0][..3], [1.5, 2.5, 3.5]);
        // The array grew once (the capacity of 1 was reached by the second string).
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 5]]);
        assert_eq!(*stored.borrow(), vec![(0, 0xaaa), (1, 0xbbb), (2, 0xccc)]);
    }
}
