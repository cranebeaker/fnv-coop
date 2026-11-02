//! `BSHavok/bhkblendcollisionobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkBlendCollisionObject` (Xbox PDB) objects are 0x2c bytes built on `bhkNiCollisionObject`:
//! `+0x8` is the object whose world data `00a68bf0` refreshes, `+0x10` a Havok object whose
//! body word at `+0x2c` selects a pair of floats of the table at `011b06d8`, `+0x18` and
//! `+0x1C` are the two blend floats stored from it, `+0x20` is the motion mode and `+0x28` a
//! counter.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame
//! or a `/GS` cookie check that is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c81570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_7e64.
pub fn bhk_blend_collision_object_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_7e64
}

/// Per-thread words: the allocation-scope word (`TLS + 0x2b4`, set to 0x10 around allocating
/// work) and the word at `TLS + 0x2c0`, which the update functions compare with 2 and 3.
const SCOPE_WORD: u32 = 0x2b4;
const TLS_UPDATE_STATE: u32 = 0x2c0;
/// The game heap's allocator (`00aa13e0`, cdecl `(size)`) and the sized delete (`00aa1460`,
/// `(pointer, size)`).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `bhkNiCollisionObject` constructor (`00c6b910`) and destructor body (`00c6c290`).
const BASE_CONSTRUCT: u32 = 0x00c6_b910;
const BASE_DESTRUCTOR: u32 = 0x00c6_c290;
/// `bhkCollisionObject` functions (Xbox PDB where named): the copy (`00c657f0`, `(this,
/// destination, process)`), `IsEqual` (`00c87fb0`), `LoadBinary`, `LinkObject`, `SaveBinary`
/// (`(this, stream or link source)`) and `NitohkTransform` (`(this)`).
const COLLISION_COPY: u32 = 0x00c6_57f0;
const COLLISION_IS_EQUAL: u32 = 0x00c8_7fb0;
const COLLISION_LOAD_BINARY: u32 = 0x00c6_5800;
const COLLISION_LINK_OBJECT: u32 = 0x00c6_5810;
const COLLISION_SAVE_BINARY: u32 = 0x00c6_5820;
const NI_TO_HK_TRANSFORM: u32 = 0x00c6_5b00;
/// `bhkNiCollisionObject::RegisterStreamables` through its thunk (`00c87fa0`).
const REGISTER_STREAMABLES_THUNK: u32 = 0x00c8_7fa0;
/// Unnamed (`00a68bf0`, `this` only): refreshes a node's world data.
const REFRESH_WORLD_DATA: u32 = 0x00a6_8bf0;
/// RTTI cast (`00653290`, `this` = the object, `(rtti)`): the object or 0.
const RTTI_CAST: u32 = 0x0065_3290;
/// The RTTI structure of this class (next to its object counter) and of the other types the
/// functions cast to.
const RTTI_CLASS: u32 = 0x0126_7e64;
const RTTI_CHILD_OWNER: u32 = 0x011f_4428;
const RTTI_UPDATE_TARGET: u32 = 0x0120_43f8;
const RTTI_MOTION_OBJECT: u32 = 0x0126_81c0;
const RTTI_CONTROLLER: u32 = 0x0126_81f0;
/// `BSUtilities::GetObjectByName` (Xbox PDB; cdecl `(root, &name, 0)`), the string interner
/// (`00a5b690`, cdecl `(text)`), the word `0109b220` its empty string is compared with and
/// `InterlockedDecrement` (import slot `00fdf1c4`).
const GET_OBJECT_BY_NAME: u32 = 0x00c4_b470;
const INTERN_STRING: u32 = 0x00a5_b690;
const EMPTY_STRING: u32 = 0x0109_b220;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Rigid-body predicate on a motion mode (`00c8cce0`, cdecl `(&mode)`), the one-byte
/// function `00a29680` (`this` only), `hkpEntity::activate` (Xbox PDB, `00c9c1d0`) and
/// `NiObjectNET::GetController` (Xbox PDB, `00a5c570`, `(this, rtti)`).
const MODE_PREDICATE: u32 = 0x00c8_cce0;
const EMPTY_MARKER: u32 = 0x00a2_9680;
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
const GET_CONTROLLER: u32 = 0x00a5_c570;
/// The table of float pairs indexed by bits 8 to 12 of the Havok object's word at `+0x2c`.
const FLOAT_PAIR_TABLE: u32 = 0x011b_06d8;
/// Class vtable and object counter.
const VTABLE_CLASS: u32 = 0x010c_53dc;
const COUNT_CLASS: u32 = 0x0126_7e60;

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// The word at `+0x2c` of the Havok object at `object + 8` (`[[object + 8] + 0x2c]`), 0 when
/// there is no such object.
fn havok_word(e: &Engine, object: u32) -> u32 {
    let body = e.mem.u32(object + 8);
    if body == 0 {
        return 0;
    }
    let member = body.wrapping_add(0x10);
    if member == 0 {
        0
    } else {
        e.mem.u32(member + 0x1c)
    }
}

/// An `FLD dword` / `FSTP dword` copy of a float.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// The state the constructor leaves: base constructor, vtable and counter, the two floats
/// 0.0 and 1.0 at `+0x18` and `+0x1C`, bit 8 of the word at `+0xC` cleared, `+0x20` = 8 and
/// the rest zero.
fn construct_blend(e: &mut Engine, object: u32) {
    e.call(BASE_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, VTABLE_CLASS);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_add(1));
    e.mem.set_f32(object + 0x18, 0.0);
    e.mem.set_f32(object + 0x1c, 1.0);
    let flags = e.mem.u16(object + 0xc);
    e.mem.set_u16(object + 0xc, flags & 0xfeff);
    e.mem.set_u32(object + 0x20, 8);
    e.mem.set_u32(object + 0x24, 0);
    e.mem.set_u32(object + 0x28, 0);
    e.mem.set_u8(object + 0x14, 0);
}

/// One stream transfer: calls the function at `+function` of the file at `[stream + file_slot]`
/// (`(file, buffer, size, &size, 1)`) and adds the count it answers to the file position at
/// `+4`.
fn transfer(e: &mut Engine, stream: u32, file_slot: u32, function: u32, buffer: u32, size: u32) {
    let file = e.mem.u32(stream + file_slot);
    let routine = e.mem.u32(file + function);
    let count = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), size);
        e.call(routine, &args![file, buffer, size, slot, 1u32])
            .u32()
    });
    let position = e.mem.u32(file + 4);
    e.mem.set_u32(file + 4, position.wrapping_add(count));
}

/// The notification the motion functions end with: when the Havok object (`this + 0x10`)
/// exists and its word's low seven bits are 8 or 0x1D, runs its virtual slot 0x9C on `+0x24`
/// when that is not 0.
fn notify_object(e: &mut Engine, this: u32) {
    let object = e.mem.u32(this + 0x10);
    if object == 0 {
        return;
    }
    let kind = havok_word(e, object) & 0x7f;
    if kind == 8 || kind == 0x1d {
        let argument = e.mem.u32(this + 0x24);
        if argument != 0 {
            e.vcall(object, 0x9c, &args![argument]);
        }
    }
}

// Translated from 00c81580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the copy step of the class, `(this, destination, process)`): runs the
/// `bhkCollisionObject` copy (`00c657f0`), then copies the floats at `+0x1C` and `+0x18`
/// (`FLD`/`FSTP` copies) and the word at `+0x24` into `destination`.
pub fn fn_00c81580(e: &mut Engine, this: Ptr, destination: Ptr, process: u32) {
    e.call(COLLISION_COPY, &args![this, destination, process]);
    copy_float(e, this.addr() + 0x1c, destination.addr() + 0x1c);
    copy_float(e, this.addr() + 0x18, destination.addr() + 0x18);
    let word = e.mem.u32(this.addr() + 0x24);
    e.mem.set_u32(destination.addr() + 0x24, word);
}

// Translated from 00c815b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::IsEqual` (Xbox PDB): false when the base test (`00c87fb0`) says
/// so, else whether the floats at `+0x1C` and at `+0x18` of both objects are equal (a NaN is
/// never equal).
pub fn bhk_blend_collision_object_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(COLLISION_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let same_1c = e.mem.f32(other.addr() + 0x1c) == e.mem.f32(this.addr() + 0x1c);
    let same_18 = e.mem.f32(other.addr() + 0x18) == e.mem.f32(this.addr() + 0x18);
    same_1c && same_18
}

// Translated from 00c81600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the constructor, `this` only): the `bhkNiCollisionObject` constructor
/// (`00c6b910`), then the class vtable and counter, `+0x18` = 0.0, `+0x1C` = 1.0, bit 8 of
/// the 16-bit word at `+0xC` cleared, `+0x20` = 8, `+0x24` and `+0x28` = 0 and the byte at
/// `+0x14` = 0. Answers `this`.
pub fn fn_00c81600(e: &mut Engine, this: Ptr) -> Ptr {
    construct_blend(e, this.addr());
    this
}

// Translated from 00c81640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::_scalar_deleting_destructor_` (Xbox PDB): stores the class
/// vtable, decrements the class counter, runs the `bhkNiCollisionObject` destructor body
/// (`00c6c290`) and, when bit 0 of `flags` is set, the sized delete (`00aa1460`,
/// `(this, 0x2c)`). Answers `this`.
pub fn bhk_blend_collision_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    let count: u32 = e.global(COUNT_CLASS);
    e.set_global(COUNT_CLASS, count.wrapping_sub(1));
    e.call(BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x2cu32]);
    }
    this
}

// Translated from 00c81670 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c81670 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBlendCollisionObject::CreateClone` (Xbox PDB): builds a new object (a 0x2c-byte block
/// from the game heap, constructed as `00c81600` does; 0 when the heap answers 0) and runs the
/// copy step (`00c81580`'s work) on `(this, clone, process)`. Answers the clone.
pub fn bhk_blend_collision_object_create_clone(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    let clone = e.call(HEAP_ALLOCATE, &args![0x2cu32]).u32();
    if clone != 0 {
        construct_blend(e, clone);
    }
    fn_00c81580(e, this, Ptr::new(clone), process);
    clone
}

// Translated from 00c81730 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c81730 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (cdecl, no arguments): `new bhkBlendCollisionObject`, 0 when the heap returns 0.
pub fn fn_00c81730(e: &mut Engine) -> u32 {
    let object = e.call(HEAP_ALLOCATE, &args![0x2cu32]).u32();
    if object == 0 {
        return 0;
    }
    construct_blend(e, object);
    object
}

// Translated from 00c817d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::UpdateWorldData` (Xbox PDB; the stack word is unused): does
/// nothing when the word at `TLS + 0x2C0` is 3. A set byte at `+0x14` is cleared after
/// refreshing the world data of the object at `+8` (`00a68bf0`, when there is one) and ends
/// it. Otherwise, unless bit 6 of the dword at `+0x10` of the Havok object (`+0x10`) is set:
/// bit 6 of the byte at `+0xC` makes it convert the transform (`00c65b00`) and clear that bit
/// in the 16-bit word (refreshing the world data instead when the TLS word is 2); with bit 0
/// of that byte set it runs virtual slot 0xA4 when there is a Havok object, the float at
/// `+0x18` is below 1.0 or `+0x20` is 1, and ends; every other case refreshes the world data
/// of the object at `+8`.
pub fn bhk_blend_collision_object_update_world_data(e: &mut Engine, this: Ptr, _unused: u32) {
    let me = this.addr();
    let slot = e.tls() + TLS_UPDATE_STATE;
    let state = e.mem.u32(slot);
    if state == 3 {
        return;
    }
    if e.mem.u8(me + 0x14) != 0 {
        let target = e.mem.u32(me + 8);
        if target != 0 {
            e.call(REFRESH_WORLD_DATA, &args![target]);
        }
        e.mem.set_u8(me + 0x14, 0);
        return;
    }
    let object = e.mem.u32(me + 0x10);
    let object_flag = object != 0 && (e.mem.u32(object + 0x10) >> 6) & 1 != 0;
    if !object_flag {
        let flags = e.mem.u8(me + 0xc);
        if (flags >> 6) & 1 != 0 {
            if state != 2 {
                e.call(NI_TO_HK_TRANSFORM, &args![this]);
                let word = e.mem.u16(me + 0xc);
                e.mem.set_u16(me + 0xc, word & 0xffbf);
                return;
            }
        } else if flags & 1 != 0 {
            let scale = e.mem.f32(me + 0x18);
            if object != 0 || 1.0 > scale || e.mem.u32(me + 0x20) == 1 {
                e.vcall(me, 0xa4, &args![]);
            }
            return;
        }
    }
    let target = e.mem.u32(me + 8);
    if target != 0 {
        e.call(REFRESH_WORLD_DATA, &args![target]);
    }
}

// Translated from 00c81890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::ForceUpdateWorldData` (Xbox PDB, `this` only): does nothing when
/// the word at `TLS + 0x2C0` is 3. With bit 6 of the byte at `+0xC` set it converts the
/// transform (`00c65b00`) and clears that bit in the 16-bit word, or refreshes the world data
/// of the object at `+8` (`00a68bf0`, when there is one) if the TLS word is 2. Without it,
/// bit 0 of the byte runs virtual slot 0xA4, else the world data is refreshed.
pub fn bhk_blend_collision_object_force_update_world_data(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let slot = e.tls() + TLS_UPDATE_STATE;
    let state = e.mem.u32(slot);
    if state == 3 {
        return;
    }
    let flags = e.mem.u8(me + 0xc);
    if (flags >> 6) & 1 != 0 {
        if state != 2 {
            e.call(NI_TO_HK_TRANSFORM, &args![this]);
            let word = e.mem.u16(me + 0xc);
            e.mem.set_u16(me + 0xc, word & 0xffbf);
            return;
        }
    } else if flags & 1 != 0 {
        e.vcall(me, 0xa4, &args![]);
        return;
    }
    let target = e.mem.u32(me + 8);
    if target != 0 {
        e.call(REFRESH_WORLD_DATA, &args![target]);
    }
}

// Translated from 00c818f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::LoadBinary` (Xbox PDB): `bhkCollisionObject::LoadBinary`
/// (`00c65800`) on `(this, stream)`, then reads the floats at `+0x18` and `+0x1C` (4 bytes
/// each) through the read function of the stream's file (`[stream + 0x24C]`, function at
/// `+8`, `(file, destination, 4, &4, 1)`, the count added to the file position at `+4`);
/// when the dword at `stream + 4` is below 8 it reads two more words into a scratch area.
pub fn bhk_blend_collision_object_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLISION_LOAD_BINARY, &args![this, stream]);
    transfer(e, stream.addr(), 0x24c, 8, this.addr() + 0x18, 4);
    transfer(e, stream.addr(), 0x24c, 8, this.addr() + 0x1c, 4);
    if e.mem.u32(stream.addr() + 4) < 8 {
        e.with_stack(8, |e, scratch| {
            transfer(e, stream.addr(), 0x24c, 8, scratch.addr(), 4);
            transfer(e, stream.addr(), 0x24c, 8, scratch.addr() + 4, 4);
        });
    }
}

// Translated from 00c819a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::LinkObject` (Xbox PDB): `bhkCollisionObject::LinkObject`
/// (`00c65810`) on `(this, link source)`, sets bits 3 and 5 of the 16-bit word at `+0xC`, and
/// takes the pair of floats at index `(word >> 8) & 0x1F` of the table at `011b06d8`
/// (8 bytes per index, `FLD`/`FSTP` copies; the word is the Havok object word at `+0x2c` of
/// the object at `+8` of `[this + 0x10]`, 0 when there is none) into `+0x18` and `+0x1C`.
/// The exe does not test `[this + 0x10]`.
pub fn bhk_blend_collision_object_link_object(e: &mut Engine, this: Ptr, source: u32) {
    let me = this.addr();
    e.call(COLLISION_LINK_OBJECT, &args![this, source]);
    let flags = e.mem.u16(me + 0xc);
    e.mem.set_u16(me + 0xc, flags | 0x28);
    let object = e.mem.u32(me + 0x10);
    let index = (havok_word(e, object) >> 8) & 0x1f;
    copy_float(e, FLOAT_PAIR_TABLE + index * 8, me + 0x18);
    copy_float(e, FLOAT_PAIR_TABLE + index * 8 + 4, me + 0x1c);
}

// Translated from 00c819f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::SaveBinary` (Xbox PDB): `bhkCollisionObject::SaveBinary`
/// (`00c65820`) on `(this, stream)`, then writes the floats at `+0x18` and `+0x1C` (4 bytes
/// each) through the write function of the stream's file (`[stream + 0x250]`, function at
/// `+0xC`, `(file, source, 4, &4, 1)`, the count added to the file position at `+4`).
pub fn bhk_blend_collision_object_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLISION_SAVE_BINARY, &args![this, stream]);
    transfer(e, stream.addr(), 0x250, 0xc, this.addr() + 0x18, 4);
    transfer(e, stream.addr(), 0x250, 0xc, this.addr() + 0x1c, 4);
}

// Translated from 00c81a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::SynchronizeMotionType` (Xbox PDB, `this` only): with a Havok
/// object at `+0x10` and the mode at `+0x20`, runs the object's virtual slot 0xE4 with the
/// mode when the mode is 4 and the byte at `+0xE8` of the object's `+8` is missing or not
/// 4, or when the mode is 1 and that byte is missing, 4 or 5.
pub fn bhk_blend_collision_object_synchronize_motion_type(e: &mut Engine, this: Ptr) {
    let object = e.mem.u32(this.addr() + 0x10);
    if object == 0 {
        return;
    }
    let mode = e.mem.u32(this.addr() + 0x20);
    let body = e.mem.u32(object + 8);
    let byte = if body == 0 {
        None
    } else {
        Some(e.mem.u8(body + 0xe8))
    };
    let call = match mode {
        4 => byte != Some(4),
        1 => matches!(byte, None | Some(4) | Some(5)),
        _ => false,
    };
    if call {
        e.vcall(object, 0xe4, &args![mode]);
    }
}

// Translated from 00c81aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`this` only): the motion notification: with a Havok object at `+0x10` whose word
/// has the low seven bits 8 or 0x1D, runs the object's virtual slot 0x9C with the word at
/// `+0x24` when that is not 0.
pub fn fn_00c81aa0(e: &mut Engine, this: Ptr) {
    notify_object(e, this.addr());
}

// Translated from 00c81ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`this` only): increments the counter at `+0x28`; when it was 0, forces the world
/// data update ([`bhk_blend_collision_object_force_update_world_data`]) and gives the motion
/// notification ([`fn_00c81aa0`]).
pub fn fn_00c81ae0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let counter = e.mem.u32(me + 0x28);
    e.mem.set_u32(me + 0x28, counter.wrapping_add(1));
    if counter == 0 {
        bhk_blend_collision_object_force_update_world_data(e, this);
        notify_object(e, me);
    }
}

// Translated from 00c81b90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c81b90 - the FS:[0] exception-unwinding frame is not modeled.
/// Unnamed (cdecl `(root, name)`, answers a float): 1.0 when `root` is 0. Otherwise looks the
/// named object up under `root` (the name is interned, `00a5b690`, and passed by address to
/// `BSUtilities::GetObjectByName`, `00c4b470`; the interned string is released with
/// `InterlockedDecrement` of the word 8 bytes before it unless it is the word at
/// `0109b220`); `root` itself is used when there is no name or no such object. When the
/// found object's `+0x1C` casts (`00653290`) to the RTTI at `01267e64`, the answer is the
/// float at `+0x18` of the cast result (an `FLD`/`FSTP` copy), else 1.0.
pub fn fn_00c81b90(e: &mut Engine, root: Ptr, name: u32) -> f32 {
    let mut value = 1.0f32;
    if root.addr() == 0 {
        return value;
    }
    let mut found = root.addr();
    if name != 0 {
        let handle = e.call(INTERN_STRING, &args![name]).u32();
        let (object, released) = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), handle);
            let object = e.call(GET_OBJECT_BY_NAME, &args![root, slot, 0u32]).u32();
            (object, e.mem.u32(slot.addr()))
        });
        if released != e.global::<u32>(EMPTY_STRING) {
            e.call(INTERLOCKED_DECREMENT, &args![released.wrapping_sub(8)]);
        }
        if object != 0 {
            found = object;
        }
    }
    let field = e.mem.u32(found + 0x1c);
    if field != 0 {
        let cast = e.call(RTTI_CAST, &args![field, RTTI_CLASS]).u32();
        if cast != 0 {
            value = x87_quiet(e.mem.f32(cast + 0x18));
        }
    }
    value
}

/// `[object + 0x1C]` cast to this class's RTTI (`00653290`), 0 when the field is 0.
fn cast_field(e: &mut Engine, object: u32) -> u32 {
    let field = e.mem.u32(object + 0x1c);
    if field == 0 {
        0
    } else {
        e.call(RTTI_CAST, &args![field, RTTI_CLASS]).u32()
    }
}

// Translated from 00c81c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::GetFirstCollisionObject` (Xbox PDB, cdecl `(node)`): searches a
/// node for the first object (its `+0x1C` cast to this class's RTTI, `00653290`) of this
/// class: the node's own, the first child's (the child array is at `+0xA0`, its 16-bit count
/// at `+0xA6`, each child cast to the RTTI at `011f4428`), then, in the node or that child,
/// the child at index 1 (index 0 when the 16-bit word at `+0xA8` is 1) and the children
/// returned by that one's virtual slot 0xC. 0 when nothing is found.
pub fn bhk_blend_collision_object_get_first_collision_object(e: &mut Engine, node: Ptr) -> u32 {
    let node = node.addr();
    if node == 0 {
        return 0;
    }
    let own = cast_field(e, node);
    if own != 0 {
        return own;
    }
    let mut result = 0;
    let mut child = 0;
    if e.mem.u16(node + 0xa6) != 0 {
        let first = e.mem.u32(e.mem.u32(node + 0xa0));
        if first != 0 {
            child = e.call(RTTI_CAST, &args![first, RTTI_CHILD_OWNER]).u32();
            if child != 0 {
                result = cast_field(e, child);
            }
        }
    }
    if result == 0 {
        child = 0;
        if e.mem.u16(node + 0xa6) != 0 {
            let first = e.mem.u32(e.mem.u32(node + 0xa0));
            if first != 0 {
                child = e.call(RTTI_CAST, &args![first, RTTI_CHILD_OWNER]).u32();
                if child != 0 {
                    result = cast_field(e, child);
                }
            }
        }
    }
    let cursor = if child == 0 { node } else { child };
    if result != 0 {
        return result;
    }
    let count = u32::from(e.mem.u16(cursor + 0xa6));
    let index = u32::from(e.mem.u16(cursor + 0xa8) != 1);
    if count <= index {
        return 0;
    }
    let candidate = e.mem.u32(e.mem.u32(cursor + 0xa0) + index * 4);
    if candidate == 0 {
        return 0;
    }
    let found = cast_field(e, candidate);
    if found != 0 {
        return found;
    }
    let list = e.vcall(candidate, 0xc, &args![]).u32();
    if list == 0 || e.mem.u16(list + 0xa6) == 0 {
        return 0;
    }
    let mut position = 0u32;
    loop {
        let element = e.mem.u32(e.mem.u32(list + 0xa0) + position * 4);
        let found = cast_field(e, element);
        if found != 0 {
            return found;
        }
        position += 1;
        if position >= u32::from(e.mem.u16(list + 0xa6)) {
            return 0;
        }
    }
}

// Translated from 00c81db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl `(node, value)`): for `node` and, recursively, all the children its
/// virtual slot 0xC answers (array at `+0xA0`, 16-bit count at `+0xA6`): when the node's
/// `+0x1C` casts (`00653290`) to the RTTI at `012043f8`, runs that object's virtual slot
/// 0x3C and stores `value` at `+0x24` of its answer when it is not 0. Does nothing for a
/// null node.
pub fn fn_00c81db0(e: &mut Engine, node: Ptr, value: u32) {
    let node = node.addr();
    if node == 0 {
        return;
    }
    let field = e.mem.u32(node + 0x1c);
    if field != 0 {
        let cast = e.call(RTTI_CAST, &args![field, RTTI_UPDATE_TARGET]).u32();
        if cast != 0 {
            let target = e.vcall(cast, 0x3c, &args![]).u32();
            if target != 0 {
                e.mem.set_u32(target + 0x24, value);
            }
        }
    }
    let list = e.vcall(node, 0xc, &args![]).u32();
    if list == 0 || e.mem.u16(list + 0xa6) == 0 {
        return;
    }
    let mut position = 0u32;
    loop {
        let child = e.mem.u32(e.mem.u32(list + 0xa0) + position * 4);
        fn_00c81db0(e, Ptr::new(child), value);
        position += 1;
        if position >= u32::from(e.mem.u16(list + 0xa6)) {
            return;
        }
    }
}

// Translated from 00c82580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::GetBipBlendValue` (Xbox PDB, cdecl `(node)`, answers a float):
/// the float at `+0x18` of the first collision object under `node`
/// (`00c81c50`; an `FLD`/`FSTP` copy), 1.0 when there is none.
pub fn bhk_blend_collision_object_get_bip_blend_value(e: &mut Engine, node: Ptr) -> f32 {
    let found = bhk_blend_collision_object_get_first_collision_object(e, node);
    if found == 0 {
        1.0
    } else {
        x87_quiet(e.mem.f32(found + 0x18))
    }
}

// Translated from 00c823b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::SetMotionType` (Xbox PDB, `(this, mode, object, flag)`, the last
/// a byte). The mode lives in a stack word that is passed by address to the rigid-body
/// predicate (`00c8cce0`), which is true for modes 1 to 3 and 6. `object` is cast
/// (`00653290`) to the RTTI at `012681c0`; without a cast the Havok object at `+0x10` is used
/// (with none, nothing below runs). Both floats start as 0.0 for such a mode and 1.0
/// otherwise. When the target's Havok word has the low seven bits 8 or 0x1D, the pair of
/// floats of the table at `011b06d8` at index `(word >> 8) & 0x1F` (`t`, `u`) sets
/// `+0x18` = `t * start` and `+0x1C` = `u * start`, or both to 1.0 when `t` is above 1.0;
/// bit 9 of the 16-bit word at `+0xC` is cleared and, for such a mode, a set `flag`
/// activates the target's entity (`00c9c1d0`, between two `00a29680` calls) and the
/// controller (`00a5c570`, RTTI `012681f0`) of the object at `+8` has bit 3 of its 16-bit
/// word at `+8` cleared. Otherwise the target's virtual slot 0xE4 runs with the mode; if it
/// answers false and the mode satisfies the predicate both floats become 1.0. The floats are
/// stored (`FLD`/`FSTP` copies); if the mode differs from the word at `+0x20`, bit 6 of the
/// byte at `+0xC` is clear and bit 6 of the dword at `+0x10` of the target is clear, the
/// world data update is forced (`00c81890`). It ends with
/// [`bhk_blend_collision_object_synchronize_motion_type`] and the motion notification.
pub fn bhk_blend_collision_object_set_motion_type(
    e: &mut Engine,
    this: Ptr,
    mode: u32,
    object: u32,
    flag: bool,
) {
    let me = this.addr();
    let mut target = if object == 0 {
        0
    } else {
        e.call(RTTI_CAST, &args![object, RTTI_MOTION_OBJECT]).u32()
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), mode);
        let special = e.call(MODE_PREDICATE, &args![slot]).bool();
        let start = if special { 0.0f32 } else { 1.0 };
        // The floats for `+0x18` (`low`) and `+0x1C` (`high`).
        let mut low = start;
        let mut high = start;
        if target == 0 {
            target = e.mem.u32(me + 0x10);
        }
        if target != 0 {
            let word = havok_word(e, target);
            let kind = word & 0x7f;
            if kind == 8 || kind == 0x1d {
                let index = (word >> 8) & 0x1f;
                let first = x87_quiet(f32::from_bits(
                    e.global::<u32>(FLOAT_PAIR_TABLE + index * 8),
                ));
                if first > 1.0 {
                    low = 1.0;
                    high = 1.0;
                } else {
                    let second = x87_quiet(f32::from_bits(
                        e.global::<u32>(FLOAT_PAIR_TABLE + index * 8 + 4),
                    ));
                    high = second * start;
                    low = first * start;
                }
                let flags = e.mem.u16(me + 0xc);
                e.mem.set_u16(me + 0xc, flags & 0xfdff);
                if e.call(MODE_PREDICATE, &args![slot]).bool() {
                    if flag {
                        let entity = e.mem.u32(target + 8);
                        if entity != 0 {
                            e.call(EMPTY_MARKER, &args![target]);
                            e.call(ENTITY_ACTIVATE, &args![entity]);
                            e.call(EMPTY_MARKER, &args![target]);
                        }
                    }
                    let owner = e.mem.u32(me + 8);
                    if owner != 0 {
                        let controller =
                            e.call(GET_CONTROLLER, &args![owner, RTTI_CONTROLLER]).u32();
                        if controller != 0 {
                            let bits = e.mem.u16(controller + 8);
                            e.mem.set_u16(controller + 8, bits & 0xfff7);
                        }
                    }
                }
            } else {
                let current = e.mem.u32(slot.addr());
                let accepted = e.vcall(target, 0xe4, &args![current]).bool();
                if !accepted && e.call(MODE_PREDICATE, &args![slot]).bool() {
                    low = 1.0;
                    high = 1.0;
                }
            }
        }
        let previous = e.mem.u32(me + 0x20);
        e.mem.set_u32(me + 0x1c, x87_quiet(high).to_bits());
        e.mem.set_u32(me + 0x18, x87_quiet(low).to_bits());
        let current = e.mem.u32(slot.addr());
        if previous != current
            && (e.mem.u8(me + 0xc) >> 6) & 1 == 0
            && (e.mem.u32(target + 0x10) >> 6) & 1 == 0
        {
            bhk_blend_collision_object_force_update_world_data(e, this);
        }
    });
    bhk_blend_collision_object_synchronize_motion_type(e, this);
    notify_object(e, me);
}

// Translated from 00cb7610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSPCollisionObject::RegisterStreamables` (Xbox PDB): a jump to `00c87fa0` (which jumps
/// on to `00c6b730`); `(this, stream)` and the answer pass through.
pub fn bhk_s_p_collision_object_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> u32 {
    e.call(REGISTER_STREAMABLES_THUNK, &args![this, stream])
        .u32()
}

/// `NiGetViewerString` and its overloads for a float, a bool and a string, `sprintf`-style
/// formatting (`00406d00`, cdecl `(buffer, size, format, value)`), the base
/// `bhkCollisionObject::AddViewerStrings` (`00c66060`, `(this, strings)`) and the array
/// helpers of the viewer-string array.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const FORMAT_TEXT: u32 = 0x0040_6d00;
const COLLISION_ADD_VIEWER_STRINGS: u32 = 0x00c6_6060;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The prefix texts of the five lines (`010c54e0`, `010c54d4`, `010c54c8`, `010c54b8`,
/// `010c54a0`) and the format of the sixth (`010c54b0`).
const PREFIX_FIRST_FLOAT: u32 = 0x010c_54e0;
const PREFIX_SECOND_FLOAT: u32 = 0x010c_54d4;
const PREFIX_FIRST_BOOL: u32 = 0x010c_54c8;
const PREFIX_SECOND_BOOL: u32 = 0x010c_54b8;
const PREFIX_TEXT: u32 = 0x010c_54a0;
const FORMAT_WORD: u32 = 0x010c_54b0;

/// Appends `text` to the viewer-string array: grown by the step at `+0xE` plus the size when
/// the 16-bit size at `+0xA` has reached the capacity at `+0x8`, then stored at the old size.
fn append_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    let capacity = u32::from(e.mem.u16(strings + 8));
    if size >= capacity {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00c825b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c825b0 - the FS:[0] exception-unwinding frame and the /GS cookie check (00ec408c) are not modeled.
/// `bhkBlendCollisionObject::AddViewerStrings` (Xbox PDB), with the allocation scope word
/// set to 0x10: runs `bhkCollisionObject::AddViewerStrings` (`00c66060`) on `(this,
/// strings)`, then appends six strings (each growing the array by the step at `+0xE` plus
/// the size when the 16-bit size at `+0xA` has reached the capacity at `+0x8`):
/// `NiGetViewerString` of the word at `01267e64`; the float overload (`00a7d550`, `(prefix,
/// float)`) of the floats at `+0x18` (prefix `010c54e0`) and `+0x1C` (prefix `010c54d4`);
/// the bool overload (`00a7d370`, `(prefix, value)`) of bit 0 of the byte at `+0xD` (prefix
/// `010c54c8`) and of bit 9 of the 16-bit word at `+0xC` (prefix `010c54b8`); and the text
/// overload (`00a7d5e0`, `(prefix 010c54a0, text)`) of the word at `+0x24` formatted
/// (`00406d00`, 64-byte buffer) with `010c54b0`. The bool arguments are the stack dword
/// whose low byte was replaced: its other three bytes are those of the string pointer the
/// preceding append stored there.
pub fn bhk_blend_collision_object_add_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let me = this.addr();
    let list = strings.addr();
    let (word, saved) = enter_scope(e);
    e.call(COLLISION_ADD_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(RTTI_CLASS);
    let mut text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    append_viewer_string(e, list, text);
    for (offset, prefix) in [(0x18u32, PREFIX_FIRST_FLOAT), (0x1c, PREFIX_SECOND_FLOAT)] {
        let value = x87_quiet_bits(e.mem.u32(me + offset));
        text = e
            .call(NI_GET_VIEWER_STRING_FLOAT, &args![prefix, value])
            .u32();
        append_viewer_string(e, list, text);
    }
    let first_bit = u32::from(e.mem.u8(me + 0xd) & 1);
    let second_bit = u32::from((e.mem.u16(me + 0xc) >> 9) & 1);
    for (bit, prefix) in [
        (first_bit, PREFIX_FIRST_BOOL),
        (second_bit, PREFIX_SECOND_BOOL),
    ] {
        let argument = (text & 0xffff_ff00) | bit;
        text = e
            .call(NI_GET_VIEWER_STRING_BOOL, &args![prefix, argument])
            .u32();
        append_viewer_string(e, list, text);
    }
    let value = e.mem.u32(me + 0x24);
    let text = e.with_stack(0x40, |e, buffer| {
        e.call(FORMAT_TEXT, &args![buffer, 0x40u32, FORMAT_WORD, value]);
        e.call(NI_GET_VIEWER_STRING_TEXT, &args![PREFIX_TEXT, buffer])
            .u32()
    });
    append_viewer_string(e, list, text);
    e.mem.set_u32(word, saved);
}

/// `bhkRigidBody::GetPosition` and `GetRotation` (Xbox PDB, `00560bc0` and `00560c20`,
/// `this` = the Havok object, `(out)`), the predicate `008cffa0` (`this` = the Havok object,
/// answers a byte), `NiQuaternion::ToRotation` (Xbox PDB, `004f0180`, `this` = the
/// quaternion, `(matrix out)`), `NiQuaternion::FromRotation` (Xbox PDB, `00a6df40`, `this` =
/// the quaternion out, `(matrix)`) and the vector subtraction `004578c0` (`this -= argument`
/// for three floats).
const RIGID_BODY_GET_POSITION: u32 = 0x0056_0bc0;
const RIGID_BODY_GET_ROTATION: u32 = 0x0056_0c20;
const BODY_PREDICATE: u32 = 0x008c_ffa0;
const QUATERNION_TO_ROTATION: u32 = 0x004f_0180;
const QUATERNION_FROM_ROTATION: u32 = 0x00a6_df40;
const POINT_SUBTRACT: u32 = 0x0045_78c0;
/// `bhkBlendCollisionObject::GetFirstCollisionObject` (Xbox PDB, `00c81c50`, cdecl `(node)`).
const GET_FIRST_COLLISION_OBJECT: u32 = 0x00c8_1c50;

// Translated from 00c821b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::SetBipTransform` (Xbox PDB), cdecl `(node, position, update)`
/// (`update` is a byte). Goes down the node's first children (virtual slot 0xC; each needs a
/// non-zero 16-bit count at `+0xA6` and a first entry of the array at `+0xA0`) to three nodes
/// `a`, `b` and `c`, takes `b`'s first collision object (`00c81c50`) and does nothing more
/// unless `c`'s word at `+0x1C` is non-zero or the float at `+0x18` of that object is 0.0.
/// The rotation (nine words) and position (three words) come from the object's Havok body
/// (`+0x10`) when the predicate `008cffa0` accepts it (`GetPosition`, `GetRotation`,
/// `ToRotation`); else, when the float is 0.0, from `a`'s own matrix at `+0x34` and position
/// at `+0x58` (`FromRotation`, `ToRotation`); else the thirteen words at `c + 0x68`. The
/// third position word is `a`'s own (`+0x60`) unless the float is 0.0 and `update` is 0.
/// Unless the position came from `a`, it is stored at `position` less `a`'s old position
/// (`004578c0`). `a`'s position (`+0x58`) is set and, when the float is 0.0, so is its
/// matrix. With `update`, `a`'s world data is refreshed (`00a68bf0`) and its virtual slot
/// 0xB8 runs with a nine-byte zero record.
pub fn bhk_blend_collision_object_set_bip_transform(
    e: &mut Engine,
    node: Ptr,
    position: Ptr,
    update: u32,
) {
    let update = update & 0xff != 0;
    if node.addr() == 0 {
        return;
    }
    let a = e.vcall(node.addr(), 0xc, &args![]).u32();
    if a == 0 || e.mem.u16(a + 0xa6) == 0 {
        return;
    }
    let list = e.mem.u32(a + 0xa0);
    let child = e.mem.u32(list);
    if child == 0 {
        return;
    }
    let b = e.vcall(child, 0xc, &args![]).u32();
    if b == 0 || e.mem.u16(b + 0xa6) == 0 {
        return;
    }
    let list = e.mem.u32(b + 0xa0);
    let child = e.mem.u32(list);
    if child == 0 {
        return;
    }
    let c = e.vcall(child, 0xc, &args![]).u32();
    if c == 0 {
        return;
    }
    let object = e.call(GET_FIRST_COLLISION_OBJECT, &args![b]).u32();
    if object == 0 {
        return;
    }
    let zero_blend = 0.0f32 == e.mem.f32(object + 0x18);
    if e.mem.u32(c + 0x1c) == 0 && !zero_blend {
        return;
    }
    let body = e.mem.u32(object + 0x10);
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let quaternion = f;
        let matrix = f + 0x10;
        let new_position = f + 0x34;
        let copy = f + 0x44;
        let mut rebuilt = false;
        if body != 0 && e.call(BODY_PREDICATE, &args![body]).bool() {
            e.call(RIGID_BODY_GET_POSITION, &args![body, new_position]);
            e.call(RIGID_BODY_GET_ROTATION, &args![body, quaternion]);
            e.call(QUATERNION_TO_ROTATION, &args![quaternion, matrix]);
        } else if zero_blend {
            for lane in 0..3u32 {
                let value = e.mem.u32(a + 0x58 + 4 * lane);
                e.mem.set_u32(new_position + 4 * lane, value);
            }
            rebuilt = true;
            e.call(QUATERNION_FROM_ROTATION, &args![quaternion, a + 0x34]);
            e.call(QUATERNION_TO_ROTATION, &args![quaternion, matrix]);
        } else {
            for lane in 0..13u32 {
                let value = e.mem.u32(c + 0x68 + 4 * lane);
                e.mem.set_u32(matrix + 4 * lane, value);
            }
        }
        for lane in 0..13u32 {
            let value = e.mem.u32(a + 0x34 + 4 * lane);
            e.mem.set_u32(copy + 4 * lane, value);
        }
        if update || !zero_blend {
            let value = x87_quiet_bits(e.mem.u32(copy + 0x2c));
            e.mem.set_u32(new_position + 8, value);
        }
        let x = e.mem.u32(new_position);
        let y = e.mem.u32(new_position + 4);
        let z = e.mem.u32(new_position + 8);
        if !rebuilt {
            e.mem.set_u32(position.addr(), x);
            e.mem.set_u32(position.addr() + 4, y);
            e.mem.set_u32(position.addr() + 8, z);
            e.call(POINT_SUBTRACT, &args![position, a + 0x58]);
        }
        e.mem.set_u32(a + 0x58, x);
        e.mem.set_u32(a + 0x5c, y);
        e.mem.set_u32(a + 0x60, z);
        if zero_blend {
            for lane in 0..9u32 {
                let value = e.mem.u32(matrix + 4 * lane);
                e.mem.set_u32(a + 0x34 + 4 * lane, value);
            }
        }
    });
    if update {
        e.call(REFRESH_WORLD_DATA, &args![a]);
        e.with_stack(0x10, |e, record| {
            e.mem.set_f32(record.addr(), 0.0);
            for offset in 4..9u32 {
                e.mem.set_u8(record.addr() + offset, 0);
            }
            e.vcall(a, 0xb8, &args![record]);
        });
    }
}

/// `bhkCollisionObject::SetMotionType` (Xbox PDB, `00c65f40`, `(this, mode, Havok object,
/// 0)`) and `bhkCollisionObject::hktoNiTransform` (Xbox PDB, `00c65a80`, `this` only).
const COLLISION_SET_MOTION_TYPE: u32 = 0x00c6_5f40;
const COLLISION_HK_TO_NI_TRANSFORM: u32 = 0x00c6_5a80;
/// Unnamed: set the linear and angular velocity of a Havok object through its character
/// proxy (`00561690`, `005617c0`, `this` = the object, `(vector)`), and the vector they get
/// (`01267e30`).
const SET_LINEAR_VELOCITY: u32 = 0x0056_1690;
const SET_ANGULAR_VELOCITY: u32 = 0x0056_17c0;
const SHARED_VECTOR: u32 = 0x0126_7e30;
/// The lock around the motion type change: acquired by `0040fbf0` (`this` = `01267b80`, one
/// zero argument); released inline by decrementing the count at `01267b84` and clearing the
/// owner word at `01267b80` when it reaches 0.
const LOCK_ACQUIRE: u32 = 0x0040_fbf0;
const LOCK_OBJECT: u32 = 0x0126_7b80;
/// Floats and the vector in the exe image: the value whose reciprocal is given to
/// `00c8de20` (`011afe64`), the factor of the node position (`01267e6c`) and the vector
/// (1, 1, 1, 1) (`010ec7f0`).
const RECIPROCAL_SOURCE: u32 = 0x011a_fe64;
const POSITION_SCALE: u32 = 0x0126_7e6c;
const ONES_VECTOR: u32 = 0x010e_c7f0;
/// Unnamed: `0056dff0` (cdecl `(out, transform)`) fills `out` from the rotation of a
/// transform; `hkQuaternion::set` (Xbox PDB, `00cb26e0`, `this` = the quaternion,
/// `(out)`); `hkQuaternion::setSlerp` (Xbox PDB, `00cb2850`, `this` = the result, `(from,
/// to, float)`); `005611c0` (`this` = the quaternion); `00cb2d90` (`this` = a 0x40-byte
/// record, `(quaternion)`); `00624b10` (cdecl `(transform, record)`) writes the rotation of
/// the record into a transform; `00458620` (cdecl `(out, vector)`) writes three floats; and
/// `00c8de20` (`this` = the Havok object, `(vector, quaternion, float, float)`).
const TRANSFORM_ROTATION_OUT: u32 = 0x0056_dff0;
const QUATERNION_SET: u32 = 0x00cb_26e0;
const QUATERNION_SET_SLERP: u32 = 0x00cb_2850;
const QUATERNION_STEP: u32 = 0x0056_11c0;
const RECORD_FROM_QUATERNION: u32 = 0x00cb_2d90;
const TRANSFORM_FROM_RECORD: u32 = 0x0062_4b10;
const VECTOR_TO_FLOATS: u32 = 0x0045_8620;
const UPDATE_FROM_BLEND: u32 = 0x00c8_de20;
/// Unnamed (`00c81aa0`, `this` only; the motion notification).
const MOTION_NOTIFICATION: u32 = 0x00c8_1aa0;

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// The part of `hktoNiTransform` that runs when the blend mode is 0: refreshes the node's
/// world data, blends the Havok object's position and rotation with the node's, writes the
/// result into the node's transform at `+0x68` and runs `00c8de20` on the Havok object.
fn blend_into_node(e: &mut Engine, this: u32, node: u32, hk: u32, flag: bool, source: f32) {
    e.call(REFRESH_WORLD_DATA, &args![node]);
    let transform = node + 0x68;
    let translation = transform + 0x24;
    let scale = x87_quiet(e.global::<f32>(POSITION_SCALE));
    let state_word = e.tls() + TLS_UPDATE_STATE;
    e.with_stack(0xb0, |e, frame| {
        let f = frame.addr();
        // Frame: 0 scaled position, 0x10 and 0x20 quaternion and position blend, 0x30 and
        // 0x40 two more quaternions, 0x50 the Havok position, 0x60 a 0x40-byte record whose
        // last vector (0x90) is the position, 0xa0 the rotation buffer.
        let scaled = f;
        let blended_quaternion = f + 0x10;
        let blended_position = f + 0x20;
        let node_quaternion = f + 0x30;
        let havok_quaternion = f + 0x40;
        let havok_position = f + 0x50;
        let record = f + 0x60;
        let buffer = f + 0xa0;
        for lane in 0..3u32 {
            let value = x87_quiet(e.mem.f32(translation + 4 * lane));
            let product = f64::from(value) * f64::from(scale);
            e.mem.set_f32(scaled + 4 * lane, product as f32);
        }
        e.mem.set_f32(scaled + 0xc, 0.0);
        e.call(TRANSFORM_ROTATION_OUT, &args![buffer, transform]);
        e.call(QUATERNION_SET, &args![node_quaternion, buffer]);
        e.vcall(hk, 0xd4, &args![havok_position]);
        e.vcall(hk, 0xd8, &args![havok_quaternion]);
        let blend = x87_quiet(e.mem.f32(this + 0x18));
        let ones = lanes(e, ONES_VECTOR);
        let position = lanes(e, havok_position);
        let own = lanes(e, scaled);
        let mixed = [0, 1, 2, 3].map(|i| (ones[i] - blend) * position[i] + own[i] * blend);
        set_lanes(e, blended_position, mixed);
        let blend_bits = x87_quiet_bits(e.mem.u32(this + 0x18));
        e.call(
            QUATERNION_SET_SLERP,
            &args![
                blended_quaternion,
                havok_quaternion,
                node_quaternion,
                blend_bits
            ],
        );
        e.call(QUATERNION_STEP, &args![blended_quaternion]);
        let chosen = if flag { blended_position } else { scaled };
        for lane in 0..4u32 {
            let value = e.mem.u32(chosen + 4 * lane);
            e.mem.set_u32(record + 0x30 + 4 * lane, value);
        }
        e.call(RECORD_FROM_QUATERNION, &args![record, blended_quaternion]);
        e.call(TRANSFORM_FROM_RECORD, &args![transform, record]);
        e.call(VECTOR_TO_FLOATS, &args![translation, record + 0x30]);
        if e.mem.u32(state_word) == 0 {
            e.vcall(this, 0xb8, &args![transform]);
        }
        // `FLD1; FDIVRP` in double precision, or 1.0 when the value is 0.0.
        let reciprocal = if source == 0.0 {
            1.0f32
        } else {
            (1.0f64 / f64::from(source)) as f32
        };
        let second = if flag {
            1.0f32
        } else {
            x87_quiet(e.mem.f32(this + 0x1c))
        };
        e.call(
            UPDATE_FROM_BLEND,
            &args![
                hk,
                blended_position,
                blended_quaternion,
                reciprocal.to_bits(),
                second.to_bits()
            ],
        );
    });
}

// Translated from 00c81e30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c81e30 - the /GS cookie check (00ec408c) is not modeled.
/// `bhkBlendCollisionObject::hktoNiTransform` (Xbox PDB), `this` only. Picks a target motion
/// mode and a blend mode from the float at `+0x18` (`a`), the float at `+0x1C` (`b`), bit 0
/// of the byte at `+0xD` (`flag`) and the word at `TLS + 0x2c0` (`state`): `a` = 0.0 gives
/// target 1 (when `state` is 1 only the node's (`+8`) world data is refreshed, `00a68bf0`,
/// and the function ends), blend mode 1 when `b` = 0.0 or `flag`, else 0; `a` = 1.0 gives
/// target 4 and blend mode 1 when `state` is 2, else 2; any other `a` gives target 1 and
/// blend mode 1 when `state` is 2, else 0. When the motion mode at `+0x20` is not the
/// target, it is changed (virtual slot 0xA8 when it was 4, else the Havok object's (`+0x10`)
/// linear and angular velocity are set from the vector at `01267e30` if it exists), under
/// the lock at `01267b80` (`0040fbf0`), by `bhkCollisionObject::SetMotionType` (`00c65f40`,
/// `(this, target, Havok object, 0)`), and remembered. Then blend mode 0 blends the Havok
/// object's position and rotation (virtual slots 0xD4, 0xD8) with the node's transform at
/// `+0x68` of the node (virtual slot 0xB8 runs when `state` is 0),
/// mode 1 runs `bhkCollisionObject::hktoNiTransform` (`00c65a80`) and mode 2 virtual slot
/// 0xA8. When the motion mode was changed and the object at `+0x24` has a zero word at
/// `+0x18`, the motion notification `00c81aa0` runs.
pub fn bhk_blend_collision_object_hkto_ni_transform(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let source = x87_quiet(e.global::<f32>(RECIPROCAL_SOURCE));
    let node = e.mem.u32(me + 8);
    let flag = e.mem.u8(me + 0xd) & 1 != 0;
    let hk = e.mem.u32(me + 0x10);
    let state_word = e.tls() + TLS_UPDATE_STATE;
    let first = e.mem.f32(me + 0x18);
    let (target, mode) = if first == 0.0 {
        if e.mem.u32(state_word) == 1 {
            e.call(REFRESH_WORLD_DATA, &args![node]);
            return;
        }
        let second = e.mem.f32(me + 0x1c);
        (1u32, u32::from(second == 0.0 || flag))
    } else if 1.0 == first {
        let state = e.mem.u32(state_word);
        (4, if state == 2 { 1 } else { 2 })
    } else {
        let state = e.mem.u32(state_word);
        (1, u32::from(state == 2))
    };
    let current = e.mem.u32(me + 0x20);
    let mut changed = false;
    if current != target {
        if current == 4 {
            e.vcall(me, 0xa8, &args![]);
        } else if hk != 0 {
            e.call(SET_LINEAR_VELOCITY, &args![hk, SHARED_VECTOR]);
            e.call(SET_ANGULAR_VELOCITY, &args![hk, SHARED_VECTOR]);
        }
        e.call(LOCK_ACQUIRE, &args![LOCK_OBJECT, 0u32]);
        e.call(COLLISION_SET_MOTION_TYPE, &args![this, target, hk, 0u32]);
        let count = e.global::<u32>(LOCK_OBJECT + 4).wrapping_sub(1);
        e.set_global(LOCK_OBJECT + 4, count);
        if count == 0 {
            e.set_global(LOCK_OBJECT, 0u32);
        }
        e.mem.set_u32(me + 0x20, target);
        changed = true;
    }
    match mode {
        0 => blend_into_node(e, me, node, hk, flag, source),
        1 => {
            e.call(COLLISION_HK_TO_NI_TRANSFORM, &args![this]);
        }
        2 => {
            e.vcall(me, 0xa8, &args![]);
        }
        _ => {}
    }
    if changed {
        let owner = e.mem.u32(me + 0x24);
        if owner != 0 && e.mem.u32(owner + 0x18) == 0 {
            e.call(MOTION_NOTIFICATION, &args![this]);
        }
    }
}

// Translated from 00c81b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the class is unconfirmed): `this` in `ECX`, no stack words. Counts
/// the word at `+0x28` down; while it is still positive that is all. Otherwise it is reset to 0
/// and, when the object at `+0x10` exists and its kind is 8 or 0x1d, the virtual function at +0x9c
/// of that object is called with the word at `+0x24` (when it is not null). The kind is the low
/// seven bits of the word at `+0x1c` of the block `0x10` bytes into the object the owner's word at
/// `+8` points to (0 when that word is 0). Ghidra made no function here; the address is reached
/// by a jump at 00c662ef.
pub fn fn_00c81b40(e: &mut Engine, this: u32) {
    let count = (e.mem.u32(this + 0x28) as i32).wrapping_sub(1);
    e.mem.set_u32(this + 0x28, count as u32);
    if count > 0 {
        return;
    }
    let owner = e.mem.u32(this + 0x10);
    e.mem.set_u32(this + 0x28, 0);
    if owner == 0 {
        return;
    }
    let inner = e.mem.u32(owner + 8);
    let kind = if inner != 0 {
        // `ADD EDX, 0x10` followed by `JZ`: a sum that wraps to zero counts as no object.
        let block = inner.wrapping_add(0x10);
        if block != 0 {
            e.mem.u32(block.wrapping_add(0x1c))
        } else {
            0
        }
    } else {
        0
    } & 0x7f;
    if kind == 8 || kind == 0x1d {
        let target = e.mem.u32(this + 0x24);
        if target != 0 {
            e.vcall(owner, 0x9c, &args![target]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c81570, bhk_blend_collision_object_get_r_t_t_i() -> u32),
        entry!(0x00c81580, fn_00c81580(Ptr, Ptr, u32)),
        entry!(0x00c815b0, bhk_blend_collision_object_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c81600, fn_00c81600(Ptr) -> Ptr),
        entry!(0x00c81640, bhk_blend_collision_object_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00c81670, bhk_blend_collision_object_create_clone(Ptr, u32) -> u32),
        entry!(0x00c81730, fn_00c81730() -> u32),
        entry!(
            0x00c817d0,
            bhk_blend_collision_object_update_world_data(Ptr, u32)
        ),
        entry!(
            0x00c81890,
            bhk_blend_collision_object_force_update_world_data(Ptr)
        ),
        entry!(0x00c818f0, bhk_blend_collision_object_load_binary(Ptr, Ptr)),
        entry!(0x00c819a0, bhk_blend_collision_object_link_object(Ptr, u32)),
        entry!(0x00c819f0, bhk_blend_collision_object_save_binary(Ptr, Ptr)),
        entry!(
            0x00c81a50,
            bhk_blend_collision_object_synchronize_motion_type(Ptr)
        ),
        entry!(0x00c81aa0, fn_00c81aa0(Ptr)),
        entry!(0x00c81ae0, fn_00c81ae0(Ptr)),
        entry!(0x00c81b90, fn_00c81b90(Ptr, u32) -> f32),
        entry!(0x00c81c50, bhk_blend_collision_object_get_first_collision_object(Ptr) -> u32),
        entry!(0x00c81db0, fn_00c81db0(Ptr, u32)),
        entry!(0x00c82580, bhk_blend_collision_object_get_bip_blend_value(Ptr) -> f32),
        entry!(
            0x00c823b0,
            bhk_blend_collision_object_set_motion_type(Ptr, u32, u32, bool)
        ),
        entry!(0x00cb7610, bhk_s_p_collision_object_register_streamables(Ptr, Ptr) -> u32),
        entry!(
            0x00c81e30,
            bhk_blend_collision_object_hkto_ni_transform(Ptr)
        ),
        entry!(
            0x00c821b0,
            bhk_blend_collision_object_set_bip_transform(Ptr, Ptr, u32)
        ),
        entry!(
            0x00c825b0,
            bhk_blend_collision_object_add_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00c8_1b40, fn_00c81b40(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_blend_collision_object_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c8_1570, &args![]).u32(), 0x0126_7e64);
    }

    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn map_counter(e: &mut Engine) {
        e.map(COUNT_CLASS, 4);
        e.set_global(COUNT_CLASS, 10u32);
    }

    /// A vtable of `slots` (byte offset, target) in front of a 0x100-byte object.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, table);
        object
    }

    /// Sets the word at `TLS + 0x2C0`.
    fn set_state(e: &mut Engine, value: u32) {
        let word = e.tls() + TLS_UPDATE_STATE;
        e.mem.set_u32(word, value);
    }

    /// The cast double: succeeds (answering the object) for the `(object, rtti)` pairs given.
    fn cast_double(e: &mut Engine, ok: Vec<(u32, u32)>) {
        e.register_double(RTTI_CAST, move |_, a| {
            answer(if ok.contains(&(a[0], a[1])) { a[0] } else { 0 })
        });
    }

    /// The rigid-body predicate: modes 1 to 3 and 6.
    fn mode_predicate(e: &mut Engine, a: &[u32]) -> Ret {
        let mode = e.mem.u32(a[0]) as i32;
        answer(u32::from(mode > 0 && (mode < 4 || mode == 6)))
    }

    /// The Havok object word table with the pair `(first, second)` at index 3, and a Havok
    /// object of the class `kind` and index 3 (`word`) whose body holds the word.
    fn havok_object(e: &mut Engine, slots: &[(u32, u32)], word: u32) -> u32 {
        let object = object(e, slots);
        let body = e.mem.alloc(0x100);
        e.mem.set_u32(object + 8, body);
        e.mem.set_u32(body + 0x2c, word);
        object
    }

    fn pair_table(e: &mut Engine, first: f32, second: f32) {
        e.map(FLOAT_PAIR_TABLE, 0x100);
        e.mem.set_f32(FLOAT_PAIR_TABLE + 3 * 8, first);
        e.mem.set_f32(FLOAT_PAIR_TABLE + 3 * 8 + 4, second);
    }

    #[test]
    fn fn_00c81580_copies_the_floats_quietly_and_the_word() {
        let mut e = Engine::new();
        e.register(COLLISION_COPY, nothing);
        let this = e.mem.alloc(0x40);
        let destination = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x1c, 0x7f80_0001);
        e.mem.set_f32(this + 0x18, 0.25);
        e.mem.set_u32(this + 0x24, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00c81580, &args![this, destination, 0x77u32]);
        assert_eq!(calls(&e, COLLISION_COPY), [[this, destination, 0x77]]);
        assert_eq!(e.mem.u32(destination + 0x1c), 0x7fc0_0001);
        assert_eq!(e.mem.f32(destination + 0x18), 0.25);
        assert_eq!(e.mem.u32(destination + 0x24), 0x1234);
    }

    #[test]
    fn is_equal_needs_the_base_and_both_floats() {
        let mut e = Engine::new();
        e.register(COLLISION_IS_EQUAL, |_, _| answer(1));
        let this = e.mem.alloc(0x40);
        let other = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x18, 0.5);
        e.mem.set_f32(other + 0x18, 0.5);
        e.mem.set_f32(this + 0x1c, 2.0);
        e.mem.set_f32(other + 0x1c, 2.0);
        assert_eq!(e.call(0x00c815b0, &args![this, other]).u32(), 1);
        e.mem.set_f32(other + 0x18, 0.75);
        assert_eq!(e.call(0x00c815b0, &args![this, other]).u32(), 0);
        e.mem.set_f32(other + 0x18, 0.5);
        e.mem.set_f32(other + 0x1c, 3.0);
        assert_eq!(e.call(0x00c815b0, &args![this, other]).u32(), 0);
        // NaN is never equal.
        e.mem.set_f32(this + 0x1c, f32::NAN);
        e.mem.set_f32(other + 0x1c, f32::NAN);
        assert_eq!(e.call(0x00c815b0, &args![this, other]).u32(), 0);
        e.mem.set_f32(other + 0x1c, 3.0);
        e.mem.set_f32(this + 0x1c, 3.0);
        assert_eq!(e.call(0x00c815b0, &args![this, other]).u32(), 1);
        e.register(COLLISION_IS_EQUAL, |_, _| answer(0));
        assert_eq!(e.call(0x00c815b0, &args![this, other]).u32(), 0);
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = Engine::new();
        map_counter(&mut e);
        e.register(BASE_CONSTRUCT, nothing);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 0xc, 0xffff);
        e.mem.set_u32(this + 0x24, 5);
        e.mem.set_u32(this + 0x28, 5);
        e.mem.set_u8(this + 0x14, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c81600, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_CONSTRUCT), [[this]]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
        assert_eq!(e.mem.f32(this + 0x18), 0.0);
        assert_eq!(e.mem.f32(this + 0x1c), 1.0);
        assert_eq!(e.mem.u16(this + 0xc), 0xfeff);
        assert_eq!(e.mem.u32(this + 0x20), 8);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u8(this + 0x14), 0);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        map_counter(&mut e);
        e.register(BASE_DESTRUCTOR, nothing);
        e.register(SIZED_DELETE, nothing);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c81640, &args![this, 0xfffeu32]).u32(), this);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00c81640, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, SIZED_DELETE), [[this, 0x2c]]);
        assert_eq!(calls(&e, BASE_DESTRUCTOR), [[this], [this]]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 8);
    }

    #[test]
    fn create_clone_and_new_build_the_object() {
        let mut e = Engine::new();
        map_counter(&mut e);
        let block = e.mem.alloc(0x40);
        e.register_double(HEAP_ALLOCATE, move |_, _| answer(block));
        e.register(BASE_CONSTRUCT, nothing);
        e.register(COLLISION_COPY, nothing);
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x18, 0.5);
        e.mem.set_f32(this + 0x1c, 0.75);
        e.mem.set_u32(this + 0x24, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c81670, &args![this, 0x7700u32]).u32(), block);
        assert_eq!(calls(&e, HEAP_ALLOCATE), [[0x2c]]);
        assert_eq!(calls(&e, COLLISION_COPY), [[this, block, 0x7700]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        assert_eq!(e.mem.f32(block + 0x18), 0.5);
        assert_eq!(e.mem.f32(block + 0x1c), 0.75);
        assert_eq!(e.mem.u32(block + 0x24), 9);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 11);
        // new
        let second = e.mem.alloc(0x40);
        e.register_double(HEAP_ALLOCATE, move |_, _| answer(second));
        assert_eq!(e.call(0x00c81730, &args![]).u32(), second);
        assert_eq!(e.mem.f32(second + 0x1c), 1.0);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 12);
        e.register(HEAP_ALLOCATE, nothing);
        assert_eq!(e.call(0x00c81730, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(COUNT_CLASS), 12);
    }

    /// An object for the update functions: a world object at `+8`, a Havok object at `+0x10`
    /// (when `with_object`) and the vtable double 0xA4.
    fn update_setup(e: &mut Engine, with_object: bool) -> (u32, u32) {
        e.register(NI_TO_HK_TRANSFORM, nothing);
        e.register(REFRESH_WORLD_DATA, nothing);
        e.register(0x0900_00a4, nothing);
        let this = object(e, &[(0xa4, 0x0900_00a4)]);
        let world = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, world);
        if with_object {
            let hk = e.mem.alloc(0x40);
            e.mem.set_u32(this + 0x10, hk);
        }
        e.call_log = Some(vec![]);
        (this, world)
    }

    #[test]
    fn update_world_data_does_nothing_in_state_3() {
        let mut e = Engine::new();
        let (this, _) = update_setup(&mut e, false);
        set_state(&mut e, 3);
        e.mem.set_u8(this + 0x14, 1);
        e.call(0x00c817d0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x14), 1);
        assert!(calls(&e, REFRESH_WORLD_DATA).is_empty());
    }

    #[test]
    fn update_world_data_clears_the_pending_byte_after_refreshing() {
        let mut e = Engine::new();
        let (this, world) = update_setup(&mut e, false);
        e.mem.set_u8(this + 0x14, 1);
        e.call(0x00c817d0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(calls(&e, REFRESH_WORLD_DATA), [[world]]);
        e.mem.set_u8(this + 0x14, 1);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c817d0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x14), 0);
        assert_eq!(calls(&e, REFRESH_WORLD_DATA).len(), 1);
    }

    #[test]
    fn update_world_data_converts_the_transform_for_bit_6() {
        for (state, converts) in [(0u32, true), (1, true), (2, false)] {
            let mut e = Engine::new();
            let (this, world) = update_setup(&mut e, false);
            set_state(&mut e, state);
            e.mem.set_u16(this + 0xc, 0x0041);
            e.call(0x00c817d0, &args![this, 0u32]);
            if converts {
                assert_eq!(calls(&e, NI_TO_HK_TRANSFORM), [[this]]);
                assert_eq!(e.mem.u16(this + 0xc), 0x0001);
                assert!(calls(&e, REFRESH_WORLD_DATA).is_empty());
            } else {
                assert!(calls(&e, NI_TO_HK_TRANSFORM).is_empty());
                assert_eq!(calls(&e, REFRESH_WORLD_DATA), [[world]]);
            }
        }
    }

    #[test]
    fn update_world_data_with_bit_0_runs_slot_0xa4_under_the_conditions() {
        // (with a Havok object, float at +0x18, word at +0x20, runs)
        let cases = [
            (true, 2.0f32, 0u32, true),
            (false, 0.5, 0, true),
            (false, 1.0, 1, true),
            (false, 1.0, 2, false),
            (false, f32::NAN, 0, false),
            (false, 2.0, 1, true),
        ];
        for (with_object, scale, word, runs) in cases {
            let mut e = Engine::new();
            let (this, _) = update_setup(&mut e, with_object);
            e.mem.set_u8(this + 0xc, 1);
            e.mem.set_f32(this + 0x18, scale);
            e.mem.set_u32(this + 0x20, word);
            e.call(0x00c817d0, &args![this, 0u32]);
            assert_eq!(calls(&e, 0x0900_00a4).len(), usize::from(runs));
            assert!(calls(&e, REFRESH_WORLD_DATA).is_empty());
        }
    }

    #[test]
    fn update_world_data_refreshes_otherwise() {
        let mut e = Engine::new();
        let (this, world) = update_setup(&mut e, true);
        // Bit 6 of the Havok object's dword at +0x10 forces the refresh.
        let hk = e.mem.u32(this + 0x10);
        e.mem.set_u32(hk + 0x10, 0x40);
        e.mem.set_u8(this + 0xc, 0x41);
        e.call(0x00c817d0, &args![this, 0u32]);
        assert_eq!(calls(&e, REFRESH_WORLD_DATA), [[world]]);
        assert!(calls(&e, NI_TO_HK_TRANSFORM).is_empty());
        // Neither bit set.
        e.mem.set_u32(hk + 0x10, 0);
        e.mem.set_u8(this + 0xc, 0);
        e.call(0x00c817d0, &args![this, 0u32]);
        assert_eq!(calls(&e, REFRESH_WORLD_DATA).len(), 2);
    }

    #[test]
    fn force_update_world_data_follows_the_flag_bits() {
        let mut e = Engine::new();
        let (this, world) = update_setup(&mut e, false);
        set_state(&mut e, 3);
        e.mem.set_u8(this + 0xc, 0x41);
        e.call(0x00c81890, &args![this]);
        assert!(calls(&e, NI_TO_HK_TRANSFORM).is_empty());
        set_state(&mut e, 0);
        e.mem.set_u16(this + 0xc, 0x0041);
        e.call(0x00c81890, &args![this]);
        assert_eq!(calls(&e, NI_TO_HK_TRANSFORM), [[this]]);
        assert_eq!(e.mem.u16(this + 0xc), 0x0001);
        // Bit 6 with state 2 refreshes.
        set_state(&mut e, 2);
        e.mem.set_u16(this + 0xc, 0x0041);
        e.call(0x00c81890, &args![this]);
        assert_eq!(calls(&e, REFRESH_WORLD_DATA), [[world]]);
        // Bit 0 alone runs slot 0xA4; no bits refreshes.
        set_state(&mut e, 0);
        e.mem.set_u16(this + 0xc, 1);
        e.call(0x00c81890, &args![this]);
        assert_eq!(calls(&e, 0x0900_00a4).len(), 1);
        e.mem.set_u16(this + 0xc, 0);
        e.call(0x00c81890, &args![this]);
        assert_eq!(calls(&e, REFRESH_WORLD_DATA).len(), 2);
        e.mem.set_u32(this + 8, 0);
        e.call(0x00c81890, &args![this]);
        assert_eq!(calls(&e, REFRESH_WORLD_DATA).len(), 2);
    }

    /// A stream whose file (read at `+0x24C`, write at `+0x250`) has the read function
    /// 0x0900_0040 and the write function 0x0900_0044, position 100.
    fn stream_setup(e: &mut Engine, version: u32) -> (u32, u32) {
        let file = object(e, &[]);
        e.mem.set_u32(file + 8, 0x0900_0040);
        e.mem.set_u32(file + 0xc, 0x0900_0044);
        e.mem.set_u32(file + 4, 100);
        let stream = e.mem.alloc(0x300);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 0x250, file);
        e.mem.set_u32(stream + 4, version);
        (stream, file)
    }

    #[test]
    fn load_binary_reads_two_floats_and_two_more_for_old_streams() {
        for (version, extra) in [(8u32, 0usize), (7, 2), (0x20, 0)] {
            let mut e = Engine::new();
            let (stream, file) = stream_setup(&mut e, version);
            e.register(COLLISION_LOAD_BINARY, nothing);
            e.register(0x0900_0040, |e, a| {
                assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
                e.mem.set_u32(a[1], 0x3f00_0000);
                answer(4)
            });
            let this = e.mem.alloc(0x40);
            e.call_log = Some(vec![]);
            e.call(0x00c818f0, &args![this, stream]);
            assert_eq!(calls(&e, COLLISION_LOAD_BINARY), [[this, stream]]);
            let reads = calls(&e, 0x0900_0040);
            assert_eq!(reads.len(), 2 + extra);
            assert_eq!(reads[0][1], this + 0x18);
            assert_eq!(reads[1][1], this + 0x1c);
            assert_eq!(e.mem.f32(this + 0x18), 0.5);
            assert_eq!(e.mem.u32(file + 4), 100 + 4 * (2 + extra as u32));
        }
    }

    #[test]
    fn save_binary_writes_the_two_floats() {
        let mut e = Engine::new();
        let (stream, file) = stream_setup(&mut e, 8);
        e.register(COLLISION_SAVE_BINARY, nothing);
        e.register(0x0900_0044, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            answer(4)
        });
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00c819f0, &args![this, stream]);
        assert_eq!(calls(&e, COLLISION_SAVE_BINARY), [[this, stream]]);
        let writes: Vec<u32> = calls(&e, 0x0900_0044).iter().map(|c| c[1]).collect();
        assert_eq!(writes, [this + 0x18, this + 0x1c]);
        assert_eq!(e.mem.u32(file + 4), 108);
    }

    #[test]
    fn link_object_takes_the_float_pair_of_the_filter_word() {
        let mut e = Engine::new();
        pair_table(&mut e, 0.5, 0.25);
        e.mem.set_u32(FLOAT_PAIR_TABLE + 3 * 8, 0x7f80_0001);
        e.register(COLLISION_LINK_OBJECT, nothing);
        let this = e.mem.alloc(0x40);
        let hk = havok_object(&mut e, &[], 0x0000_0308);
        e.mem.set_u32(this + 0x10, hk);
        e.mem.set_u16(this + 0xc, 0x0100);
        e.call_log = Some(vec![]);
        e.call(0x00c819a0, &args![this, 0x5000u32]);
        assert_eq!(calls(&e, COLLISION_LINK_OBJECT), [[this, 0x5000]]);
        assert_eq!(e.mem.u16(this + 0xc), 0x0128);
        assert_eq!(e.mem.u32(this + 0x18), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x1c), 0.25);
        // No body: index 0.
        let bare = object(&mut e, &[]);
        e.mem.set_u32(this + 0x10, bare);
        e.mem.set_f32(FLOAT_PAIR_TABLE, 4.0);
        e.mem.set_f32(FLOAT_PAIR_TABLE + 4, 8.0);
        e.call(0x00c819a0, &args![this, 0x5000u32]);
        assert_eq!(e.mem.f32(this + 0x18), 4.0);
        assert_eq!(e.mem.f32(this + 0x1c), 8.0);
    }

    #[test]
    fn synchronize_motion_type_runs_slot_0xe4_for_the_mismatching_bytes() {
        // (mode, byte at +0xE8 of the body or none, runs)
        let cases = [
            (4u32, None, true),
            (4, Some(4u8), false),
            (4, Some(5), true),
            (1, None, true),
            (1, Some(4), true),
            (1, Some(5), true),
            (1, Some(3), false),
            (2, Some(4), false),
            (2, None, false),
        ];
        for (mode, byte, runs) in cases {
            let mut e = Engine::new();
            e.register(0x0900_00e4, nothing);
            let this = object(&mut e, &[]);
            let hk = object(&mut e, &[(0xe4, 0x0900_00e4)]);
            if let Some(byte) = byte {
                let body = e.mem.alloc(0x100);
                e.mem.set_u8(body + 0xe8, byte);
                e.mem.set_u32(hk + 8, body);
            }
            e.mem.set_u32(this + 0x10, hk);
            e.mem.set_u32(this + 0x20, mode);
            e.call_log = Some(vec![]);
            e.call(0x00c81a50, &args![this]);
            assert_eq!(
                calls(&e, 0x0900_00e4),
                if runs { vec![vec![hk, mode]] } else { vec![] }
            );
        }
        // No Havok object.
        let mut e = Engine::new();
        let this = object(&mut e, &[]);
        e.call(0x00c81a50, &args![this]);
    }

    #[test]
    fn the_motion_notification_needs_the_class_bits_and_a_value() {
        for (word, runs) in [(8u32, true), (0x1d, true), (0x88, true), (0x1, false)] {
            let mut e = Engine::new();
            e.register(0x0900_009c, nothing);
            let hk = havok_object(&mut e, &[(0x9c, 0x0900_009c)], word);
            let this = object(&mut e, &[]);
            e.mem.set_u32(this + 0x10, hk);
            e.mem.set_u32(this + 0x24, 0x77);
            e.call_log = Some(vec![]);
            e.call(0x00c81aa0, &args![this]);
            assert_eq!(
                calls(&e, 0x0900_009c),
                if runs { vec![vec![hk, 0x77]] } else { vec![] }
            );
            e.mem.set_u32(this + 0x24, 0);
            e.call(0x00c81aa0, &args![this]);
            assert_eq!(calls(&e, 0x0900_009c).len(), usize::from(runs));
        }
        let mut e = Engine::new();
        let this = object(&mut e, &[]);
        e.call(0x00c81aa0, &args![this]);
    }

    #[test]
    fn fn_00c81ae0_forces_the_update_only_on_the_first_increment() {
        let mut e = Engine::new();
        e.register(0x0900_009c, nothing);
        e.register(NI_TO_HK_TRANSFORM, nothing);
        let hk = havok_object(&mut e, &[(0x9c, 0x0900_009c)], 8);
        let this = object(&mut e, &[]);
        e.mem.set_u32(this + 0x10, hk);
        e.mem.set_u32(this + 0x24, 0x77);
        e.mem.set_u16(this + 0xc, 0x0040);
        e.call_log = Some(vec![]);
        e.call(0x00c81ae0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 1);
        assert_eq!(calls(&e, NI_TO_HK_TRANSFORM), [[this]]);
        assert_eq!(calls(&e, 0x0900_009c), [[hk, 0x77]]);
        e.call(0x00c81ae0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 2);
        assert_eq!(calls(&e, 0x0900_009c).len(), 1);
        // No Havok object: only the forced update.
        let bare = object(&mut e, &[]);
        e.mem.set_u16(bare + 0xc, 0x0040);
        e.call(0x00c81ae0, &args![bare]);
        assert_eq!(calls(&e, NI_TO_HK_TRANSFORM).len(), 2);
    }

    #[test]
    fn fn_00c81b90_looks_the_named_object_up_and_reads_its_float() {
        let mut e = Engine::new();
        e.map(EMPTY_STRING, 4);
        e.set_global(EMPTY_STRING, 0x9999u32);
        let root = e.mem.alloc(0x40);
        let named = e.mem.alloc(0x40);
        let root_field = e.mem.alloc(0x40);
        let named_field = e.mem.alloc(0x40);
        e.mem.set_u32(root + 0x1c, root_field);
        e.mem.set_u32(named + 0x1c, named_field);
        e.mem.set_f32(root_field + 0x18, 0.5);
        e.mem.set_u32(named_field + 0x18, 0x7f80_0001);
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(handle - 8, 5);
        e.register_double(INTERN_STRING, move |_, _| answer(handle));
        let found = Rc::new(RefCell::new(named));
        let lookup = found.clone();
        e.register_double(GET_OBJECT_BY_NAME, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), handle);
            assert_eq!(a[2], 0);
            answer(*lookup.borrow())
        });
        e.register_double(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            answer(value)
        });
        cast_double(
            &mut e,
            vec![(named_field, RTTI_CLASS), (root_field, RTTI_CLASS)],
        );
        e.call_log = Some(vec![]);
        // Named object found: its field is cast, the NaN is quieted, the string released.
        let value = e.call(0x00c81b90, &args![root, 0x4040u32]);
        assert_eq!(value.f32().to_bits(), 0x7fc0_0001);
        assert_eq!(calls(&e, INTERN_STRING), [[0x4040]]);
        assert_eq!(calls(&e, GET_OBJECT_BY_NAME)[0][0], root);
        assert_eq!(e.mem.u32(handle - 8), 4);
        // Not found: the root is used.
        *found.borrow_mut() = 0;
        assert_eq!(e.call(0x00c81b90, &args![root, 0x4040u32]).f32(), 0.5);
        // Released string equal to the empty-string word is not decremented.
        e.set_global(EMPTY_STRING, handle);
        e.call(0x00c81b90, &args![root, 0x4040u32]);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT).len(), 2);
        // No name: the root is used without lookups.
        assert_eq!(e.call(0x00c81b90, &args![root, 0u32]).f32(), 0.5);
        assert_eq!(calls(&e, INTERN_STRING).len(), 3);
        // No root, no field, or a failed cast: 1.0.
        assert_eq!(e.call(0x00c81b90, &args![0u32, 0x4040u32]).f32(), 1.0);
        e.mem.set_u32(root + 0x1c, 0);
        assert_eq!(e.call(0x00c81b90, &args![root, 0u32]).f32(), 1.0);
        e.mem.set_u32(root + 0x1c, root_field);
        cast_double(&mut e, vec![]);
        assert_eq!(e.call(0x00c81b90, &args![root, 0u32]).f32(), 1.0);
    }

    /// A node with the child array at `+0xA0` (count `+0xA6`, word `+0xA8`), a field at
    /// `+0x1C` and virtual slot 0xC answering `list`.
    fn node(e: &mut Engine, field: u32, children: &[u32], word: u16, list: u32) -> u32 {
        let slot = 0x0900_0000 + list;
        e.register_double(slot, move |_, _| answer(list));
        let node = object(e, &[(0xc, slot)]);
        let array = e.mem.alloc(0x40);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *child);
        }
        e.mem.set_u32(node + 0x1c, field);
        e.mem.set_u32(node + 0xa0, array);
        e.mem.set_u16(node + 0xa6, children.len() as u16);
        e.mem.set_u16(node + 0xa8, word);
        node
    }

    #[test]
    fn get_first_collision_object_searches_the_node_then_its_children() {
        let mut e = Engine::new();
        let own = e.mem.alloc(0x40);
        // The node's own field.
        let root = node(&mut e, own, &[], 0, 0);
        cast_double(&mut e, vec![(own, RTTI_CLASS)]);
        assert_eq!(e.call(0x00c81c50, &args![root]).u32(), own);
        assert_eq!(e.call(0x00c81c50, &args![0u32]).u32(), 0);
        // The first child's field.
        let field = e.mem.alloc(0x40);
        let first = node(&mut e, 0x1111, &[], 0, 0);
        let child = node(&mut e, field, &[], 0, 0);
        let root = node(&mut e, 0x2222, &[child, first], 1, 0);
        cast_double(&mut e, vec![(child, RTTI_CHILD_OWNER), (field, RTTI_CLASS)]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c81c50, &args![root]).u32(), field);
        let lookups = |e: &Engine| {
            calls(e, RTTI_CAST)
                .iter()
                .filter(|c| c[1] == RTTI_CHILD_OWNER)
                .count()
        };
        assert_eq!(lookups(&e), 1);
    }

    #[test]
    fn get_first_collision_object_falls_back_to_the_second_child_and_its_list() {
        let mut e = Engine::new();
        // The first child does not cast; the child at index 1 holds the field.
        let field = e.mem.alloc(0x40);
        let first = node(&mut e, 0x1111, &[], 0, 0);
        let second = node(&mut e, field, &[], 0, 0);
        let root = node(&mut e, 0x2222, &[first, second], 0, 0);
        cast_double(&mut e, vec![(field, RTTI_CLASS)]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c81c50, &args![root]).u32(), field);
        // A child that does not cast is looked up twice (the exe repeats the lookup).
        let lookups = calls(&e, RTTI_CAST)
            .iter()
            .filter(|c| c[1] == RTTI_CHILD_OWNER)
            .count();
        assert_eq!(lookups, 2);
        // The word 1 at +0xA8 selects index 0.
        let root = node(&mut e, 0x2222, &[second, first], 1, 0);
        assert_eq!(e.call(0x00c81c50, &args![root]).u32(), field);
        // Index out of range.
        let root = node(&mut e, 0x2222, &[first], 0, 0);
        assert_eq!(e.call(0x00c81c50, &args![root]).u32(), 0);
        // The candidate has no field of its own: its list's elements are searched.
        let inner = e.mem.alloc(0x40);
        let element_a = node(&mut e, 0x3333, &[], 0, 0);
        let element_b = node(&mut e, inner, &[], 0, 0);
        let list = node(&mut e, 0, &[element_a, element_b], 0, 0);
        let candidate = node(&mut e, 0, &[], 0, list);
        let root = node(&mut e, 0x2222, &[first, candidate], 0, 0);
        cast_double(&mut e, vec![(inner, RTTI_CLASS)]);
        assert_eq!(e.call(0x00c81c50, &args![root]).u32(), inner);
        // A list without a match, an empty list and no list.
        cast_double(&mut e, vec![]);
        assert_eq!(e.call(0x00c81c50, &args![root]).u32(), 0);
        let empty = node(&mut e, 0, &[], 0, 0);
        let candidate = node(&mut e, 0, &[], 0, empty);
        let root = node(&mut e, 0x2222, &[first, candidate], 0, 0);
        assert_eq!(e.call(0x00c81c50, &args![root]).u32(), 0);
        let candidate = node(&mut e, 0, &[], 0, 0);
        let root = node(&mut e, 0x2222, &[first, candidate], 0, 0);
        assert_eq!(e.call(0x00c81c50, &args![root]).u32(), 0);
    }

    #[test]
    fn get_bip_blend_value_reads_the_first_collision_object() {
        let mut e = Engine::new();
        let own = e.mem.alloc(0x40);
        e.mem.set_u32(own + 0x18, 0x7f80_0001);
        let root = node(&mut e, own, &[], 0, 0);
        cast_double(&mut e, vec![(own, RTTI_CLASS)]);
        let value = e.call(0x00c82580, &args![root]);
        assert_eq!(value.f32().to_bits(), 0x7fc0_0001);
        e.mem.set_f32(own + 0x18, 0.375);
        assert_eq!(e.call(0x00c82580, &args![root]).f32(), 0.375);
        cast_double(&mut e, vec![]);
        assert_eq!(e.call(0x00c82580, &args![root]).f32(), 1.0);
    }

    #[test]
    fn fn_00c81db0_stores_the_value_in_every_matching_node() {
        let mut e = Engine::new();
        let target_a = e.mem.alloc(0x40);
        let target_b = e.mem.alloc(0x40);
        e.register_double(0x0900_0a3c, move |_, _| answer(target_a));
        e.register_double(0x0900_0b3c, move |_, _| answer(target_b));
        let cast_a = object(&mut e, &[(0x3c, 0x0900_0a3c)]);
        let cast_b = object(&mut e, &[(0x3c, 0x0900_0b3c)]);
        let leaf = node(&mut e, cast_b, &[], 0, 0);
        let plain = node(&mut e, 0, &[], 0, 0);
        let list = node(&mut e, 0, &[leaf, plain], 0, 0);
        let root = node(&mut e, cast_a, &[], 0, list);
        cast_double(
            &mut e,
            vec![(cast_a, RTTI_UPDATE_TARGET), (cast_b, RTTI_UPDATE_TARGET)],
        );
        e.call(0x00c81db0, &args![root, 0x55u32]);
        assert_eq!(e.mem.u32(target_a + 0x24), 0x55);
        assert_eq!(e.mem.u32(target_b + 0x24), 0x55);
        e.call(0x00c81db0, &args![0u32, 0x66u32]);
        // A node whose cast answers 0 is skipped but its list still runs.
        cast_double(&mut e, vec![(cast_b, RTTI_UPDATE_TARGET)]);
        e.call(0x00c81db0, &args![root, 0x77u32]);
        assert_eq!(e.mem.u32(target_a + 0x24), 0x55);
        assert_eq!(e.mem.u32(target_b + 0x24), 0x77);
    }

    /// An object for the motion type tests: the Havok object (slots 0xE4 and 0x9C) at `+0x10`
    /// whose body word is `word`, the old mode at `+0x20`, a world object at `+8`.
    fn motion_setup(e: &mut Engine, word: u32, old_mode: u32) -> (u32, u32) {
        e.register(MODE_PREDICATE, mode_predicate);
        e.register(RTTI_CAST, |_, _| answer(0));
        e.register(REFRESH_WORLD_DATA, nothing);
        e.register(0x0900_009c, nothing);
        e.register(0x0900_00e4, |_, _| answer(1));
        pair_table(e, 0.5, 0.25);
        let hk = havok_object(e, &[(0x9c, 0x0900_009c), (0xe4, 0x0900_00e4)], word);
        let this = object(e, &[]);
        e.mem.set_u32(this + 0x10, hk);
        e.mem.set_u32(this + 0x20, old_mode);
        e.mem.set_u32(this + 0x24, 0x77);
        let world = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, world);
        e.call_log = Some(vec![]);
        (this, hk)
    }

    #[test]
    fn set_motion_type_scales_the_floats_by_the_table_pair() {
        // (mode, table pair, expected +0x18, expected +0x1C)
        let cases = [
            (5u32, (0.5f32, 0.25f32), 0.5f32, 0.25f32),
            (1, (0.5, 0.25), 0.0, 0.0),
            (5, (2.0, 0.25), 1.0, 1.0),
            (1, (2.0, 0.25), 1.0, 1.0),
            (5, (1.0, 0.25), 1.0, 0.25),
        ];
        for (mode, (first, second), low, high) in cases {
            let mut e = Engine::new();
            let (this, hk) = motion_setup(&mut e, 0x0308, mode);
            pair_table(&mut e, first, second);
            e.mem.set_u16(this + 0xc, 0x0200);
            e.call(0x00c823b0, &args![this, mode, 0u32, false]);
            assert_eq!(e.mem.f32(this + 0x18), low);
            assert_eq!(e.mem.f32(this + 0x1c), high);
            assert_eq!(e.mem.u16(this + 0xc), 0);
            // The mode is unchanged, so no forced update; the notification runs.
            assert!(calls(&e, REFRESH_WORLD_DATA).is_empty());
            assert_eq!(calls(&e, 0x0900_009c), [[hk, 0x77]]);
        }
    }

    #[test]
    fn set_motion_type_activates_the_entity_for_a_special_mode_with_the_flag() {
        let mut e = Engine::new();
        let (this, hk) = motion_setup(&mut e, 0x0308, 1);
        e.register(EMPTY_MARKER, nothing);
        e.register(ENTITY_ACTIVATE, nothing);
        let controller = e.mem.alloc(0x40);
        e.mem.set_u16(controller + 8, 0xffff);
        e.register_double(GET_CONTROLLER, move |_, _| answer(controller));
        let body = e.mem.u32(hk + 8);
        e.call(0x00c823b0, &args![this, 2u32, 0u32, true]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| [EMPTY_MARKER, ENTITY_ACTIVATE, GET_CONTROLLER].contains(a))
            .collect();
        assert_eq!(
            order,
            [EMPTY_MARKER, ENTITY_ACTIVATE, EMPTY_MARKER, GET_CONTROLLER]
        );
        assert_eq!(calls(&e, ENTITY_ACTIVATE), [[body]]);
        assert_eq!(calls(&e, GET_CONTROLLER)[0][1], RTTI_CONTROLLER);
        assert_eq!(e.mem.u16(controller + 8), 0xfff7);
        // Without the flag, only the controller bit.
        e.mem.set_u16(controller + 8, 0xffff);
        e.call(0x00c823b0, &args![this, 2u32, 0u32, false]);
        assert_eq!(calls(&e, ENTITY_ACTIVATE).len(), 1);
        assert_eq!(e.mem.u16(controller + 8), 0xfff7);
        // A non-special mode does neither.
        e.mem.set_u16(controller + 8, 0xffff);
        e.call(0x00c823b0, &args![this, 5u32, 0u32, true]);
        assert_eq!(calls(&e, GET_CONTROLLER).len(), 2);
        assert_eq!(e.mem.u16(controller + 8), 0xffff);
    }

    #[test]
    fn set_motion_type_asks_other_objects_through_slot_0xe4() {
        for (accepts, mode, expected) in
            [(1u32, 2u32, 0.0f32), (0, 2, 1.0), (0, 5, 1.0), (1, 5, 1.0)]
        {
            let mut e = Engine::new();
            let (this, hk) = motion_setup(&mut e, 0x0003, mode);
            e.register_double(0x0900_00e4, move |_, _| answer(accepts));
            e.call(0x00c823b0, &args![this, mode, 0u32, false]);
            assert_eq!(calls(&e, 0x0900_00e4)[0], [hk, mode]);
            assert_eq!(e.mem.f32(this + 0x18), expected);
            assert_eq!(e.mem.f32(this + 0x1c), expected);
        }
    }

    #[test]
    fn set_motion_type_uses_the_cast_object_and_forces_the_update_on_a_mode_change() {
        let mut e = Engine::new();
        let (this, hk) = motion_setup(&mut e, 0x0308, 4);
        let world = e.mem.u32(this + 8);
        let given = havok_object(&mut e, &[(0xe4, 0x0900_00e4)], 0x0003);
        cast_double(&mut e, vec![(given, RTTI_MOTION_OBJECT)]);
        // The mode changes from 4 to 5 with clear flag bits: the update is forced.
        e.call(0x00c823b0, &args![this, 5u32, given, false]);
        assert_eq!(calls(&e, 0x0900_00e4)[0], [given, 5]);
        assert_eq!(calls(&e, REFRESH_WORLD_DATA), [[world]]);
        // Synchronize and the notification then use the object's own Havok object.
        assert_eq!(calls(&e, 0x0900_00e4)[1], [hk, 4]);
        assert_eq!(calls(&e, 0x0900_009c), [[hk, 0x77]]);
        // Bit 6 of the flag byte, or an unchanged mode, prevents the forced update.
        e.mem.set_u8(this + 0xc, 0x40);
        e.call(0x00c823b0, &args![this, 5u32, given, false]);
        e.mem.set_u8(this + 0xc, 0);
        e.mem.set_u32(this + 0x20, 5);
        e.call(0x00c823b0, &args![this, 5u32, given, false]);
        assert_eq!(calls(&e, REFRESH_WORLD_DATA).len(), 1);
    }

    #[test]
    fn set_motion_type_without_any_object_only_stores_the_floats() {
        let mut e = Engine::new();
        let (this, _) = motion_setup(&mut e, 0x0308, 5);
        e.mem.set_u32(this + 0x10, 0);
        e.call(0x00c823b0, &args![this, 5u32, 0u32, false]);
        assert_eq!(e.mem.f32(this + 0x18), 1.0);
        assert_eq!(e.mem.f32(this + 0x1c), 1.0);
        e.mem.set_u32(this + 0x20, 1);
        e.call(0x00c823b0, &args![this, 1u32, 0u32, false]);
        assert_eq!(e.mem.f32(this + 0x18), 0.0);
        assert_eq!(e.mem.f32(this + 0x1c), 0.0);
    }

    #[test]
    fn register_streamables_goes_through_the_thunk() {
        let mut e = Engine::new();
        e.register(REGISTER_STREAMABLES_THUNK, |_, a| answer(a[0] + a[1]));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00cb7610, &args![0x100u32, 0x20u32]).u32(), 0x120);
        assert_eq!(calls(&e, REGISTER_STREAMABLES_THUNK), [[0x100, 0x20]]);
    }

    type Pairs = Rc<RefCell<Vec<(u32, u32)>>>;

    #[test]
    fn add_viewer_strings_appends_six_lines_in_order() {
        for (word, first_bit, second_bit) in [(0x0200u16, 0u32, 1u32), (0x0100, 1, 0)] {
            let mut e = Engine::new();
            let scope = e.tls() + SCOPE_WORD;
            e.mem.set_u32(scope, 0x66);
            e.map(RTTI_CLASS, 4);
            e.set_global(RTTI_CLASS, 0x5151_0001u32);
            e.register(COLLISION_ADD_VIEWER_STRINGS, nothing);
            e.register(NI_GET_VIEWER_STRING, |_, _| answer(0x1111_1101));
            let floats: Pairs = Rc::new(RefCell::new(vec![]));
            let floats_in = floats.clone();
            e.register_double(NI_GET_VIEWER_STRING_FLOAT, move |_, a| {
                floats_in.borrow_mut().push((a[0], a[1]));
                answer(0xaaaa_0000 + (a[0] & 0xffff))
            });
            let bools: Pairs = Rc::new(RefCell::new(vec![]));
            let bools_in = bools.clone();
            e.register_double(NI_GET_VIEWER_STRING_BOOL, move |_, a| {
                bools_in.borrow_mut().push((a[0], a[1]));
                answer(0xbbbb_0000 + (a[0] & 0xffff))
            });
            e.register(FORMAT_TEXT, nothing);
            e.register(NI_GET_VIEWER_STRING_TEXT, |_, _| answer(0xcccc_0001));
            e.register(STRINGS_GROW, nothing);
            let stored: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
            let stored_in = stored.clone();
            e.register_double(STRINGS_SET_AT, move |e, a| {
                stored_in.borrow_mut().push(e.mem.u32(a[2]));
                let size = e.mem.u16(a[0] + 0xa);
                e.mem.set_u16(a[0] + 0xa, size + 1);
                Ret::default()
            });
            let this = e.mem.alloc(0x40);
            e.mem.set_f32(this + 0x18, 1.5);
            e.mem.set_f32(this + 0x1c, -2.0);
            e.mem.set_u16(this + 0xc, word);
            e.mem.set_u32(this + 0x24, 0x1234);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, 16);
            e.mem.set_u16(strings + 0xa, 0);
            e.call_log = Some(vec![]);
            e.call(0x00c825b0, &args![this, strings]);
            assert_eq!(calls(&e, COLLISION_ADD_VIEWER_STRINGS), [[this, strings]]);
            assert_eq!(calls(&e, NI_GET_VIEWER_STRING), [[0x5151_0001]]);
            assert_eq!(
                *floats.borrow(),
                [
                    (PREFIX_FIRST_FLOAT, 1.5f32.to_bits()),
                    (PREFIX_SECOND_FLOAT, (-2.0f32).to_bits())
                ]
            );
            // The bool arguments keep the upper three bytes of the previous string pointer.
            assert_eq!(
                *bools.borrow(),
                [
                    (PREFIX_FIRST_BOOL, 0xaaaa_5400 | first_bit),
                    (PREFIX_SECOND_BOOL, 0xbbbb_5400 | second_bit)
                ]
            );
            let format = calls(&e, FORMAT_TEXT);
            assert_eq!(format.len(), 1);
            assert_eq!(format[0][1..], [0x40, FORMAT_WORD, 0x1234]);
            assert_eq!(
                calls(&e, NI_GET_VIEWER_STRING_TEXT),
                [[PREFIX_TEXT, format[0][0]]]
            );
            assert_eq!(
                *stored.borrow(),
                [
                    0x1111_1101,
                    0xaaaa_54e0,
                    0xaaaa_54d4,
                    0xbbbb_54c8,
                    0xbbbb_54b8,
                    0xcccc_0001
                ]
            );
            assert!(calls(&e, STRINGS_GROW).is_empty());
            assert_eq!(e.mem.u32(scope), 0x66);
        }
    }

    #[test]
    fn add_viewer_strings_grows_a_full_array() {
        let mut e = Engine::new();
        e.map(RTTI_CLASS, 4);
        e.register(COLLISION_ADD_VIEWER_STRINGS, nothing);
        e.register(NI_GET_VIEWER_STRING, |_, _| answer(0x1111_1101));
        e.register(NI_GET_VIEWER_STRING_FLOAT, |_, _| answer(0x2222_0002));
        e.register(NI_GET_VIEWER_STRING_BOOL, |_, _| answer(0x3333_0003));
        e.register(FORMAT_TEXT, nothing);
        e.register(NI_GET_VIEWER_STRING_TEXT, |_, _| answer(0x4444_0004));
        e.register(STRINGS_GROW, nothing);
        e.register_double(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 5);
        e.mem.set_u16(strings + 0xa, 4);
        e.mem.set_u16(strings + 0xe, 3);
        e.call_log = Some(vec![]);
        e.call(0x00c825b0, &args![this, strings]);
        // Capacity 5: the second append (size 5) grows it, and so does every later one.
        let grown = calls(&e, STRINGS_GROW);
        assert_eq!(
            grown,
            [
                [strings, 3 + 5],
                [strings, 3 + 6],
                [strings, 3 + 7],
                [strings, 3 + 8],
                [strings, 3 + 9]
            ]
        );
        assert_eq!(calls(&e, STRINGS_SET_AT).len(), 6);
    }

    /// The chain of `SetBipTransform`: the node answers `a` through slot 0xC, whose first
    /// child answers `b`, whose first child answers `c`; `00c81c50` answers an object with
    /// the float `blend` at `+0x18` and the Havok body `body` at `+0x10`.
    struct Bip {
        e: Engine,
        node: u32,
        a: u32,
        c: u32,
        position: u32,
    }

    fn bip_setup(blend: f32, body: u32, predicate: bool) -> Bip {
        let mut e = Engine::new();
        let a = object(&mut e, &[(0xb8, 0x0900_00b8)]);
        let b = e.mem.alloc(0x100);
        let c = e.mem.alloc(0x100);
        let child_a = object(&mut e, &[(0xc, 0x0900_001c)]);
        let child_b = object(&mut e, &[(0xc, 0x0900_002c)]);
        let node = object(&mut e, &[(0xc, 0x0900_000c)]);
        for (holder, child) in [(a, child_a), (b, child_b)] {
            let list = e.mem.alloc(8);
            e.mem.set_u32(list, child);
            e.mem.set_u32(holder + 0xa0, list);
            e.mem.set_u16(holder + 0xa6, 1);
        }
        e.register_double(0x0900_000c, move |_, _| answer(a));
        e.register_double(0x0900_001c, move |_, _| answer(b));
        e.register_double(0x0900_002c, move |_, _| answer(c));
        let object_blend = e.mem.alloc(0x40);
        e.mem.set_f32(object_blend + 0x18, blend);
        e.mem.set_u32(object_blend + 0x10, body);
        e.register_double(GET_FIRST_COLLISION_OBJECT, move |_, args| {
            assert_eq!(args[0], b);
            answer(object_blend)
        });
        e.register_double(BODY_PREDICATE, move |_, _| answer(u32::from(predicate)));
        e.register(RIGID_BODY_GET_POSITION, |e, a| {
            for (lane, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e.register(RIGID_BODY_GET_ROTATION, nothing);
        e.register(QUATERNION_TO_ROTATION, |e, a| {
            for lane in 0..9u32 {
                e.mem.set_u32(a[1] + 4 * lane, 0x100 + lane);
            }
            Ret::default()
        });
        e.register(QUATERNION_FROM_ROTATION, nothing);
        e.register(POINT_SUBTRACT, nothing);
        e.register(REFRESH_WORLD_DATA, nothing);
        e.register(0x0900_00b8, nothing);
        // The node a: matrix words at +0x34, position (10, 20, 30) at +0x58.
        for lane in 0..9u32 {
            e.mem.set_u32(a + 0x34 + 4 * lane, 0x500 + lane);
        }
        for (lane, value) in [10.0f32, 20.0, 30.0].into_iter().enumerate() {
            e.mem.set_f32(a + 0x58 + 4 * lane as u32, value);
        }
        e.mem.set_u32(c + 0x1c, 1);
        let position = e.mem.alloc(0x10);
        for lane in 0..3u32 {
            e.mem.set_u32(position + 4 * lane, 0xdead_0000);
        }
        e.call_log = Some(vec![]);
        Bip {
            e,
            node,
            a,
            c,
            position,
        }
    }

    fn words3(e: &Engine, at: u32) -> [u32; 3] {
        [0, 1, 2].map(|i| e.mem.u32(at + 4 * i))
    }

    #[test]
    fn set_bip_transform_stops_at_the_first_missing_piece() {
        let mut s = bip_setup(0.5, 0, false);
        s.e.call(0x00c821b0, &args![0u32, s.position, 0u32]);
        assert!(calls(&s.e, 0x0900_000c).is_empty());
        // No child: the count word is 0.
        s.e.mem.set_u16(s.a + 0xa6, 0);
        s.e.call(0x00c821b0, &args![s.node, s.position, 0u32]);
        assert!(calls(&s.e, GET_FIRST_COLLISION_OBJECT).is_empty());
        s.e.mem.set_u16(s.a + 0xa6, 1);
        // A non-zero float and a zero word at c + 0x1C end it before the predicate.
        s.e.mem.set_u32(s.c + 0x1c, 0);
        s.e.call(0x00c821b0, &args![s.node, s.position, 0u32]);
        assert_eq!(calls(&s.e, GET_FIRST_COLLISION_OBJECT).len(), 1);
        assert!(calls(&s.e, BODY_PREDICATE).is_empty());
        assert_eq!(words3(&s.e, s.position), [0xdead_0000; 3]);
    }

    #[test]
    fn set_bip_transform_takes_the_havok_position_and_keeps_the_nodes_third_word() {
        let mut s = bip_setup(0.5, 0x7700, true);
        s.e.call(0x00c821b0, &args![s.node, s.position, 0u32]);
        let own_z = 30.0f32.to_bits();
        let wanted = [1.0f32.to_bits(), 2.0f32.to_bits(), own_z];
        assert_eq!(words3(&s.e, s.position), wanted);
        assert_eq!(words3(&s.e, s.a + 0x58), wanted);
        assert_eq!(calls(&s.e, POINT_SUBTRACT), [[s.position, s.a + 0x58]]);
        // The float is not 0.0: the matrix is not stored, nothing is refreshed.
        assert_eq!(s.e.mem.u32(s.a + 0x34), 0x500);
        assert!(calls(&s.e, REFRESH_WORLD_DATA).is_empty());
        assert_eq!(calls(&s.e, BODY_PREDICATE), [[0x7700]]);
    }

    #[test]
    fn set_bip_transform_copies_thirteen_words_without_a_body() {
        let mut s = bip_setup(0.5, 0, false);
        for lane in 0..13u32 {
            s.e.mem.set_f32(s.c + 0x68 + 4 * lane, 1.0 + lane as f32);
        }
        s.e.call(0x00c821b0, &args![s.node, s.position, 0u32]);
        let wanted = [10.0f32.to_bits(), 11.0f32.to_bits(), 30.0f32.to_bits()];
        assert_eq!(words3(&s.e, s.position), wanted);
        assert_eq!(words3(&s.e, s.a + 0x58), wanted);
        assert_eq!(calls(&s.e, POINT_SUBTRACT).len(), 1);
        assert!(calls(&s.e, BODY_PREDICATE).is_empty());
    }

    #[test]
    fn set_bip_transform_rebuilds_the_rotation_for_a_zero_float_and_updates() {
        let mut s = bip_setup(0.0, 0, false);
        s.e.mem.set_u32(s.c + 0x1c, 0);
        s.e.register_double(0x0900_00b8, |e, a| {
            // The nine-byte record is a zero float and five zero bytes.
            let bytes: Vec<u8> = (0..9u32).map(|i| e.mem.u8(a[1] + i)).collect();
            assert_eq!(bytes, [0u8; 9]);
            Ret::default()
        });
        s.e.call(0x00c821b0, &args![s.node, s.position, 1u32]);
        let from = calls(&s.e, QUATERNION_FROM_ROTATION);
        assert_eq!(from.len(), 1);
        assert_eq!(from[0][1], s.a + 0x34);
        let to = calls(&s.e, QUATERNION_TO_ROTATION);
        assert_eq!(to, [[from[0][0], from[0][0] + 0x10]]);
        // The position is the node's own and is not given back; the matrix is stored.
        assert_eq!(words3(&s.e, s.position), [0xdead_0000; 3]);
        assert!(calls(&s.e, POINT_SUBTRACT).is_empty());
        let wanted = [10.0f32.to_bits(), 20.0f32.to_bits(), 30.0f32.to_bits()];
        assert_eq!(words3(&s.e, s.a + 0x58), wanted);
        for lane in 0..9u32 {
            assert_eq!(s.e.mem.u32(s.a + 0x34 + 4 * lane), 0x100 + lane);
        }
        assert_eq!(calls(&s.e, REFRESH_WORLD_DATA), [[s.a]]);
        assert_eq!(calls(&s.e, 0x0900_00b8).len(), 1);
        assert!(calls(&s.e, BODY_PREDICATE).is_empty());
    }

    /// A blend object: a node with a translation at `+0x8c`, a Havok object answering the
    /// slots 0xD4 and 0xD8, the globals and every callee of `hktoNiTransform` replaced.
    struct Blend {
        e: Engine,
        this: u32,
        node: u32,
        hk: u32,
        owner: u32,
    }

    fn blend_setup(first: f32, second: f32, flag: u8, mode: u32, state: u32) -> Blend {
        let mut e = Engine::new();
        for (address, size) in [
            (RECIPROCAL_SOURCE, 4u32),
            (POSITION_SCALE, 4),
            (ONES_VECTOR, 16),
            (LOCK_OBJECT, 8),
        ] {
            e.map(address, size);
        }
        e.set_global(RECIPROCAL_SOURCE, 0.5f32.to_bits());
        e.set_global(POSITION_SCALE, 2.0f32.to_bits());
        for lane in 0..4u32 {
            e.set_global(ONES_VECTOR + 4 * lane, 1.0f32.to_bits());
        }
        e.set_global(LOCK_OBJECT, 7u32);
        e.set_global(LOCK_OBJECT + 4, 1u32);
        let this = object(&mut e, &[(0xa8, 0x0900_00a8), (0xb8, 0x0900_00b8)]);
        let node = e.mem.alloc(0x100);
        let hk = object(&mut e, &[(0xd4, 0x0900_00d4), (0xd8, 0x0900_00d8)]);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, node);
        e.mem.set_u8(this + 0xd, flag);
        e.mem.set_u32(this + 0x10, hk);
        e.mem.set_f32(this + 0x18, first);
        e.mem.set_f32(this + 0x1c, second);
        e.mem.set_u32(this + 0x20, mode);
        e.mem.set_u32(this + 0x24, owner);
        for (lane, value) in [4.0f32, 8.0, 12.0].into_iter().enumerate() {
            e.mem.set_f32(node + 0x8c + 4 * lane as u32, value);
        }
        set_state(&mut e, state);
        for address in [
            REFRESH_WORLD_DATA,
            SET_LINEAR_VELOCITY,
            SET_ANGULAR_VELOCITY,
            LOCK_ACQUIRE,
            COLLISION_SET_MOTION_TYPE,
            COLLISION_HK_TO_NI_TRANSFORM,
            MOTION_NOTIFICATION,
            TRANSFORM_ROTATION_OUT,
            QUATERNION_SET,
            QUATERNION_SET_SLERP,
            QUATERNION_STEP,
            RECORD_FROM_QUATERNION,
            TRANSFORM_FROM_RECORD,
            VECTOR_TO_FLOATS,
            UPDATE_FROM_BLEND,
            0x0900_00a8,
            0x0900_00b8,
            0x0900_00d8,
        ] {
            e.register(address, nothing);
        }
        e.register(0x0900_00d4, |e, a| {
            for (lane, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        Blend {
            e,
            this,
            node,
            hk,
            owner,
        }
    }

    #[test]
    fn hkto_ni_transform_only_refreshes_for_a_zero_float_in_state_one() {
        let mut s = blend_setup(0.0, 5.0, 0, 8, 1);
        s.e.call(0x00c81e30, &args![s.this]);
        let order: Vec<u32> = s.e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00c81e30, REFRESH_WORLD_DATA]);
        assert_eq!(calls(&s.e, REFRESH_WORLD_DATA), [[s.node]]);
        assert_eq!(s.e.mem.u32(s.this + 0x20), 8);
    }

    #[test]
    fn hkto_ni_transform_blend_mode_one_runs_the_base_conversion() {
        // First float 0.0, second float 0.0: target 1 and blend mode 1.
        let mut s = blend_setup(0.0, 0.0, 0, 1, 0);
        s.e.call(0x00c81e30, &args![s.this]);
        assert_eq!(calls(&s.e, COLLISION_HK_TO_NI_TRANSFORM), [[s.this]]);
        assert!(calls(&s.e, COLLISION_SET_MOTION_TYPE).is_empty());
        assert!(calls(&s.e, MOTION_NOTIFICATION).is_empty());
        // The second float 5.0 and no flag: blend mode 0 in state 0.
        let mut s = blend_setup(0.0, 5.0, 0, 1, 0);
        s.e.call(0x00c81e30, &args![s.this]);
        assert!(calls(&s.e, COLLISION_HK_TO_NI_TRANSFORM).is_empty());
        assert_eq!(calls(&s.e, REFRESH_WORLD_DATA), [[s.node]]);
        // With the flag it is blend mode 1 again.
        let mut s = blend_setup(0.0, 5.0, 1, 1, 0);
        s.e.call(0x00c81e30, &args![s.this]);
        assert_eq!(calls(&s.e, COLLISION_HK_TO_NI_TRANSFORM).len(), 1);
    }

    #[test]
    fn hkto_ni_transform_changes_the_motion_mode_under_the_lock() {
        // First float 1.0 in state 0: target 4, blend mode 2 (virtual slot 0xA8).
        let mut s = blend_setup(1.0, 0.0, 0, 1, 0);
        s.e.call(0x00c81e30, &args![s.this]);
        let order: Vec<u32> = s.e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00c81e30,
                SET_LINEAR_VELOCITY,
                SET_ANGULAR_VELOCITY,
                LOCK_ACQUIRE,
                COLLISION_SET_MOTION_TYPE,
                0x0900_00a8,
                MOTION_NOTIFICATION
            ]
        );
        assert_eq!(calls(&s.e, SET_LINEAR_VELOCITY), [[s.hk, SHARED_VECTOR]]);
        assert_eq!(calls(&s.e, SET_ANGULAR_VELOCITY), [[s.hk, SHARED_VECTOR]]);
        assert_eq!(calls(&s.e, LOCK_ACQUIRE), [[LOCK_OBJECT, 0]]);
        assert_eq!(
            calls(&s.e, COLLISION_SET_MOTION_TYPE),
            [[s.this, 4, s.hk, 0]]
        );
        assert_eq!(calls(&s.e, MOTION_NOTIFICATION), [[s.this]]);
        assert_eq!(s.e.mem.u32(s.this + 0x20), 4);
        // The lock count went from 1 to 0 and the owner word was cleared.
        assert_eq!(s.e.global::<u32>(LOCK_OBJECT + 4), 0);
        assert_eq!(s.e.global::<u32>(LOCK_OBJECT), 0);
    }

    #[test]
    fn hkto_ni_transform_leaves_motion_mode_four_through_slot_0xa8() {
        // First float 0.25 in state 2: target 1, blend mode 1; the old mode is 4.
        let mut s = blend_setup(0.25, 0.0, 0, 4, 2);
        s.e.mem.set_u32(s.owner + 0x18, 9);
        s.e.set_global(LOCK_OBJECT + 4, 2u32);
        s.e.call(0x00c81e30, &args![s.this]);
        let order: Vec<u32> = s.e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00c81e30,
                0x0900_00a8,
                LOCK_ACQUIRE,
                COLLISION_SET_MOTION_TYPE,
                COLLISION_HK_TO_NI_TRANSFORM
            ]
        );
        assert_eq!(
            calls(&s.e, COLLISION_SET_MOTION_TYPE),
            [[s.this, 1, s.hk, 0]]
        );
        // Still held by someone else; the owner of the object at +0x24 has a non-zero word.
        assert_eq!(s.e.global::<u32>(LOCK_OBJECT + 4), 1);
        assert_eq!(s.e.global::<u32>(LOCK_OBJECT), 7);
        assert!(calls(&s.e, MOTION_NOTIFICATION).is_empty());
        // First float 1.0 in state 2: target 4, blend mode 1.
        let mut s = blend_setup(1.0, 0.0, 0, 4, 2);
        s.e.call(0x00c81e30, &args![s.this]);
        assert_eq!(calls(&s.e, COLLISION_HK_TO_NI_TRANSFORM).len(), 1);
        assert!(calls(&s.e, COLLISION_SET_MOTION_TYPE).is_empty());
    }

    #[test]
    fn hkto_ni_transform_blends_the_havok_position_and_rotation() {
        let mut s = blend_setup(0.25, 5.0, 0, 1, 0);
        let lanes_seen: Rc<RefCell<Vec<[f32; 4]>>> = Rc::new(RefCell::new(vec![]));
        let seen = lanes_seen.clone();
        s.e.register_double(VECTOR_TO_FLOATS, move |e, a| {
            seen.borrow_mut()
                .push([0, 1, 2, 3].map(|i| e.mem.f32(a[1] + 4 * i)));
            Ret::default()
        });
        let blended = lanes_seen.clone();
        s.e.register_double(UPDATE_FROM_BLEND, move |e, a| {
            blended
                .borrow_mut()
                .push([0, 1, 2, 3].map(|i| e.mem.f32(a[1] + 4 * i)));
            Ret::default()
        });
        s.e.call(0x00c81e30, &args![s.this]);
        let transform = s.node + 0x68;
        assert_eq!(calls(&s.e, REFRESH_WORLD_DATA), [[s.node]]);
        let rotation = calls(&s.e, TRANSFORM_ROTATION_OUT);
        assert_eq!(rotation[0][1], transform);
        let set = calls(&s.e, QUATERNION_SET);
        assert_eq!(set[0][1], rotation[0][0]);
        let slerp = calls(&s.e, QUATERNION_SET_SLERP);
        assert_eq!(slerp[0][1..3], [slerp[0][0] + 0x30, set[0][0]]);
        assert_eq!(slerp[0][3], 0.25f32.to_bits());
        assert_eq!(calls(&s.e, QUATERNION_STEP), [[slerp[0][0]]]);
        let record = calls(&s.e, RECORD_FROM_QUATERNION);
        assert_eq!(record[0][1], slerp[0][0]);
        assert_eq!(
            calls(&s.e, TRANSFORM_FROM_RECORD),
            [[transform, record[0][0]]]
        );
        // The position (4, 8, 12) times 2.0 and a zero lane, without the flag.
        let converted = calls(&s.e, VECTOR_TO_FLOATS);
        assert_eq!(converted, [[transform + 0x24, record[0][0] + 0x30]]);
        // The blended position: 0.75 * (1, 2, 3, 4) + 0.25 * (8, 16, 24, 0).
        let update = calls(&s.e, UPDATE_FROM_BLEND);
        assert_eq!(update[0][2], slerp[0][0]);
        assert_eq!(update[0][3], 2.0f32.to_bits());
        assert_eq!(update[0][4], 5.0f32.to_bits());
        assert_eq!(update[0][0], s.hk);
        let seen = lanes_seen.borrow();
        assert_eq!(seen[0], [8.0, 16.0, 24.0, 0.0]);
        assert_eq!(seen[1], [2.75, 5.5, 8.25, 3.0]);
        // State 0: virtual slot 0xB8 gets the transform.
        assert_eq!(calls(&s.e, 0x0900_00b8), [[s.this, transform]]);
    }

    #[test]
    fn hkto_ni_transform_with_the_flag_uses_the_blended_position() {
        let mut s = blend_setup(0.25, 5.0, 1, 1, 3);
        let lanes_seen: Rc<RefCell<Vec<[f32; 4]>>> = Rc::new(RefCell::new(vec![]));
        let seen = lanes_seen.clone();
        s.e.register_double(VECTOR_TO_FLOATS, move |e, a| {
            seen.borrow_mut()
                .push([0, 1, 2, 3].map(|i| e.mem.f32(a[1] + 4 * i)));
            Ret::default()
        });
        s.e.call(0x00c81e30, &args![s.this]);
        assert_eq!(lanes_seen.borrow()[0], [2.75, 5.5, 8.25, 3.0]);
        // The state is not 0: no virtual slot 0xB8; the last float is 1.0 for the flag.
        assert!(calls(&s.e, 0x0900_00b8).is_empty());
        assert_eq!(calls(&s.e, UPDATE_FROM_BLEND)[0][4], 1.0f32.to_bits());
    }

    #[test]
    fn hkto_ni_transform_uses_one_for_the_reciprocal_of_zero() {
        let mut s = blend_setup(0.25, 5.0, 0, 1, 3);
        s.e.set_global(RECIPROCAL_SOURCE, 0.0f32.to_bits());
        s.e.call(0x00c81e30, &args![s.this]);
        assert_eq!(calls(&s.e, UPDATE_FROM_BLEND)[0][3], 1.0f32.to_bits());
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type VirtualCalls = Rc<RefCell<Vec<Vec<u32>>>>;

    /// An object with counter `counter`, owner at +0x10 (its virtual function at +0x9c is a double
    /// that logs its words), target `target` at +0x24 and an inner object of kind word `kind`.
    fn blend_object(
        e: &mut Engine,
        counter: u32,
        kind: u32,
        target: u32,
    ) -> (u32, u32, VirtualCalls) {
        let this = e.mem.alloc(0x40);
        let owner = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0xc0);
        let inner = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x9c, 0x0900_0003);
        e.mem.set_u32(owner, vtable);
        e.mem.set_u32(owner + 8, inner);
        e.mem.set_u32(inner + 0x2c, kind);
        e.mem.set_u32(this + 0x10, owner);
        e.mem.set_u32(this + 0x24, target);
        e.mem.set_u32(this + 0x28, counter);
        let log: VirtualCalls = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0003, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (this, owner, log)
    }

    /// A counter that is still positive after the decrement changes nothing else.
    #[test]
    fn fn_00c81b40_only_counts_down_while_positive() {
        let mut e = Engine::new();
        let (this, _, log) = blend_object(&mut e, 3, 8, 0x55);
        e.call(0x00c8_1b40, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 2);
        assert!(log.borrow().is_empty());
    }

    /// At zero the counter stays 0 and kinds 8 and 0x1d (high bit ignored) call the owner.
    #[test]
    fn fn_00c81b40_calls_the_owner_for_kinds_8_and_1d() {
        for kind in [8, 0x1d, 0x88, 0x9d] {
            let mut e = Engine::new();
            let (this, owner, log) = blend_object(&mut e, 1, kind, 0x55);
            e.call(0x00c8_1b40, &args![this]);
            assert_eq!(e.mem.u32(this + 0x28), 0);
            assert_eq!(*log.borrow(), vec![vec![owner, 0x55]]);
        }
        // A counter that was already 0 goes negative first, which is not positive either.
        let mut e = Engine::new();
        let (this, owner, log) = blend_object(&mut e, 0, 8, 0x66);
        e.call(0x00c8_1b40, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(*log.borrow(), vec![vec![owner, 0x66]]);
    }

    /// Other kinds, a null target, a null owner and a null inner object call nothing.
    #[test]
    fn fn_00c81b40_calls_nothing_for_other_cases() {
        let mut e = Engine::new();
        let (this, _, log) = blend_object(&mut e, 1, 7, 0x55);
        e.call(0x00c8_1b40, &args![this]);
        assert!(log.borrow().is_empty());

        let (this, _, log) = blend_object(&mut e, 1, 8, 0);
        e.call(0x00c8_1b40, &args![this]);
        assert!(log.borrow().is_empty());

        let (this, owner, log) = blend_object(&mut e, 1, 8, 0x55);
        e.mem.set_u32(owner + 8, 0);
        e.call(0x00c8_1b40, &args![this]);
        assert!(log.borrow().is_empty());

        let (this, _, log) = blend_object(&mut e, 5, 8, 0x55);
        e.mem.set_u32(this + 0x28, 1);
        e.mem.set_u32(this + 0x10, 0);
        e.call(0x00c8_1b40, &args![this]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert!(log.borrow().is_empty());
    }
}
