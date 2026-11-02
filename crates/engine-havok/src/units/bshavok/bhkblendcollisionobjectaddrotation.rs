//! `BSHavok/bhkblendcollisionobjectaddrotation.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkBlendCollisionObjectAddRotation` (Xbox PDB) objects are 0x50 bytes built on
//! `bhkBlendCollisionObject` (`BSHavok/bhkblendcollisionobject.obj`): `+0x2c` holds an extra
//! 3x3 matrix of nine floats that `UpdateWorldData` multiplies with two other
//! matrices and stores at `+0x68` of the object at `+8`.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame
//! that is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c818f0`: the jump target of `00c8ff80`.
const JUMP_TARGET_00C8FF80: u32 = 0x00c8_18f0;

// Translated from 00c8ff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObjectAddRotation::LoadBinary` (Xbox PDB): a 5-byte jump to `00c818f0` (engine map: `bhkBlendCollisionObject::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@bhkBlendCollisionObjectAddRotation@@UAAXAAVNiStream@@@Z`.
pub fn bhk_blend_collision_object_add_rotation_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C8FF80, &args![this, stream]);
}

// Translated from 00c8ff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObjectAddRotation::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_81cc.
pub fn bhk_blend_collision_object_add_rotation_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_81cc
}

/// Per-thread word (`TLS + 0x2b4`) that the allocating functions set to 0x10 around their work.
const SCOPE_WORD: u32 = 0x2b4;
/// The game heap's allocator (`00aa13e0`, cdecl `(size)`) and the sized delete (`00aa1460`,
/// `(pointer, size)`).
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `bhkBlendCollisionObject` functions this class builds on: the constructor (`00c81600`), the
/// destructor body (`00c81550`), `UpdateWorldData` (`00c817d0`, `(this, word)`), the copy step
/// (`00c81580`, `(this, destination, process)`), `AddViewerStrings` (`00c825b0`), and the
/// targets of the thunks: `LinkObject`, `SaveBinary` and `IsEqual`, plus
/// `bhkSPCollisionObject::RegisterStreamables` (`00cb7610`).
const BLEND_CONSTRUCT: u32 = 0x00c8_1600;
const BLEND_DESTRUCTOR: u32 = 0x00c8_1550;
const BLEND_UPDATE_WORLD_DATA: u32 = 0x00c8_17d0;
const BLEND_COPY: u32 = 0x00c8_1580;
const BLEND_ADD_VIEWER_STRINGS: u32 = 0x00c8_25b0;
const BLEND_LINK_OBJECT: u32 = 0x00c8_19a0;
const BLEND_SAVE_BINARY: u32 = 0x00c8_19f0;
const BLEND_IS_EQUAL: u32 = 0x00c8_15b0;
const BLEND_REGISTER_STREAMABLES: u32 = 0x00cb_7610;
/// Unnamed (`00a68bf0`, `this` only): refreshes a node's world data.
const REFRESH_WORLD_DATA: u32 = 0x00a6_8bf0;
/// `bhkNiCollisionObject::SetWorldObject` (Xbox PDB, `00c6b980`, `(this, object)`).
const SET_WORLD_OBJECT: u32 = 0x00c6_b980;
/// The matrix product at `0043f8d0` (`this` = the left operand, `(result, right)`; answers
/// the result).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `NiMatrix3::GetViewerString` (Xbox PDB, `00a598f0`, `this` = the matrix, `(prefix)`).
const MATRIX_GET_VIEWER_STRING: u32 = 0x00a5_98f0;
/// `NiGetViewerString` and the two array helpers of the viewer-string array.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The word the class passes to `NiGetViewerString`, and the text `GetViewerString` of the
/// rotation matrix gets as its prefix.
const CLASS_NAME_POINTER: u32 = 0x0126_81cc;
const MATRIX_PREFIX: u32 = 0x010c_62f8;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Class vtable and the nine floats (a 3x3 matrix) the constructor copies to `+0x2c`.
const VTABLE_CLASS: u32 = 0x010c_6234;
const DEFAULT_MATRIX: u32 = 0x011a_9448;

/// Sets the allocation-scope word to 0x10 and returns its address and old value.
fn enter_scope(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + SCOPE_WORD;
    let saved = e.mem.u32(word);
    e.mem.set_u32(word, 0x10);
    (word, saved)
}

/// Copies the nine words (`REP MOVSD`) of a matrix.
fn copy_matrix(e: &mut Engine, from: u32, to: u32) {
    for index in 0..9u32 {
        let value = e.mem.u32(from + 4 * index);
        e.mem.set_u32(to + 4 * index, value);
    }
}

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

// Translated from 00c8ff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObjectAddRotation::LinkObject` (Xbox PDB): a jump to `00c819a0`,
/// passing on `this` and its one argument.
pub fn bhk_blend_collision_object_add_rotation_link_object(e: &mut Engine, this: Ptr, source: u32) {
    e.call(BLEND_LINK_OBJECT, &args![this, source]);
}

// Translated from 00c8ffa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObjectAddRotation::RegisterStreamables` (Xbox PDB): a jump to
/// `00cb7610`; `(this, stream)` and the answer pass through.
pub fn bhk_blend_collision_object_add_rotation_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> u32 {
    e.call(BLEND_REGISTER_STREAMABLES, &args![this, stream])
        .u32()
}

// Translated from 00c8ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObjectAddRotation::SaveBinary` (Xbox PDB): a jump to `00c819f0`,
/// passing on `this` and its one argument.
pub fn bhk_blend_collision_object_add_rotation_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BLEND_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00c8ffc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObjectAddRotation::IsEqual` (Xbox PDB): a jump to `00c815b0`; `(this,
/// other)` and the byte answer pass through.
pub fn bhk_blend_collision_object_add_rotation_is_equal(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    e.call(BLEND_IS_EQUAL, &args![this, other]).bool()
}

// Translated from 00c8ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the constructor, `this` only): runs the `bhkBlendCollisionObject` constructor
/// (`00c81600`), stores the class vtable and copies the nine words at `011a9448` to `+0x2c`.
/// Answers `this`.
pub fn fn_00c8ffd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BLEND_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    copy_matrix(e, DEFAULT_MATRIX, this.addr() + 0x2c);
    this
}

// Translated from 00c90000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObjectAddRotation::_scalar_deleting_destructor_` (Xbox PDB): stores the
/// class vtable, runs the `bhkBlendCollisionObject` destructor body (`00c81550`) and, when
/// bit 0 of `flags` is set, the sized delete (`00aa1460`, `(this, 0x50)`). Answers `this`.
pub fn bhk_blend_collision_object_add_rotation_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_CLASS);
    e.call(BLEND_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x50u32]);
    }
    this
}

// Translated from 00c90030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObjectAddRotation::UpdateWorldData` (Xbox PDB, one stack word): with a
/// Havok object at `+0x10` it runs `bhkBlendCollisionObject::UpdateWorldData` (`00c817d0`)
/// with that word, else refreshes the world data of the object at `+8` (`00a68bf0`) when
/// there is one. Then, with `node` = `[this + 8]` (not tested) and `parent` = `[node + 0x18]`,
/// it multiplies the matrices at `parent + 0x68` and `node + 0x34` (`0043f8d0`), multiplies
/// the answer by the matrix at `this + 0x2c` and stores the nine words of that product at
/// `node + 0x68`.
pub fn bhk_blend_collision_object_add_rotation_update_world_data(
    e: &mut Engine,
    this: Ptr,
    word: u32,
) {
    let me = this.addr();
    if e.mem.u32(me + 0x10) != 0 {
        e.call(BLEND_UPDATE_WORLD_DATA, &args![this, word]);
    } else {
        let target = e.mem.u32(me + 8);
        if target != 0 {
            e.call(REFRESH_WORLD_DATA, &args![target]);
        }
    }
    let node = e.mem.u32(me + 8);
    let parent = e.mem.u32(node + 0x18);
    e.with_stack(0x48, |e, area| {
        let second = area.addr() + 0x24;
        let first = e
            .call(MATRIX_MULTIPLY, &args![parent + 0x68, area, node + 0x34])
            .u32();
        let product = e
            .call(MATRIX_MULTIPLY, &args![first, second, me + 0x2c])
            .u32();
        copy_matrix(e, product, node + 0x68);
    });
}

// Translated from 00c90090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c90090 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBlendCollisionObjectAddRotation::CopyFrom` (Xbox PDB, `(this, other)`): holds a
/// reference on the object at `other + 0x10` (`InterlockedIncrement` of its `+4`), clears
/// that field of `other` and sets it on `this` (`bhkNiCollisionObject::SetWorldObject`,
/// `00c6b980`, with 0 and with the object), copies the floats at `+0x1C` and `+0x18`
/// (`FLD`/`FSTP` copies) and the word at `+0x24`, runs virtual slot 0x8C of `this` with
/// `other`'s `+8`, then drops the reference (`InterlockedDecrement`; when it reaches 0 the
/// function at offset 4 of the object's vtable runs).
pub fn bhk_blend_collision_object_add_rotation_copy_from(e: &mut Engine, this: Ptr, other: Ptr) {
    let me = this.addr();
    let other = other.addr();
    let held = e.mem.u32(other + 0x10);
    if held != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![held + 4]);
    }
    e.call(SET_WORLD_OBJECT, &args![other, 0u32]);
    e.call(SET_WORLD_OBJECT, &args![this, held]);
    for offset in [0x1cu32, 0x18] {
        let bits = x87_quiet_bits(e.mem.u32(other + offset));
        e.mem.set_u32(me + offset, bits);
    }
    let word = e.mem.u32(other + 0x24);
    e.mem.set_u32(me + 0x24, word);
    let world = e.mem.u32(other + 8);
    e.vcall(me, 0x8c, &args![world]);
    if held != 0 && e.call(INTERLOCKED_DECREMENT, &args![held + 4]).u32() == 0 {
        e.vcall(held, 4, &args![]);
    }
}

// Translated from 00c90160 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c90160 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBlendCollisionObjectAddRotation::CreateClone` (Xbox PDB), with the allocation scope
/// word set to 0x10: builds a new object (a 0x50-byte block from the game heap, 0 when the
/// heap answers 0; the `bhkBlendCollisionObject` constructor, the class vtable and the
/// default matrix at `+0x2c` as in `00c8ffd0`), runs the blend copy step (`00c81580`) on
/// `(this, clone, process)` and copies the matrix at `+0x2c` of `this` into the clone.
/// Answers the clone.
pub fn bhk_blend_collision_object_add_rotation_create_clone(
    e: &mut Engine,
    this: Ptr,
    process: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let clone = e.call(HEAP_ALLOCATE, &args![0x50u32]).u32();
    if clone != 0 {
        e.call(BLEND_CONSTRUCT, &args![clone]);
        copy_matrix(e, DEFAULT_MATRIX, clone + 0x2c);
        e.mem.set_u32(clone, VTABLE_CLASS);
    }
    e.call(BLEND_COPY, &args![this, clone, process]);
    copy_matrix(e, this.addr() + 0x2c, clone + 0x2c);
    e.mem.set_u32(word, saved);
    clone
}

// Translated from 00c90240 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c90240 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkBlendCollisionObjectAddRotation::AddViewerStrings` (Xbox PDB), with the allocation
/// scope word set to 0x10: runs `bhkBlendCollisionObject::AddViewerStrings` (`00c825b0`) on
/// `(this, strings)`, then appends the string `NiGetViewerString` makes of the word at
/// `012681cc` and the string `NiMatrix3::GetViewerString` (`00a598f0`) makes of the matrix at
/// `this + 0x2c` with the prefix text at `010c62f8` (each growing the array by the step at
/// `+0xE` plus the size when the 16-bit size at `+0xA` has reached the capacity at `+0x8`).
pub fn bhk_blend_collision_object_add_rotation_add_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr,
) {
    let (word, saved) = enter_scope(e);
    e.call(BLEND_ADD_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    append_viewer_string(e, strings.addr(), text);
    let matrix_text = e
        .call(
            MATRIX_GET_VIEWER_STRING,
            &args![this.addr() + 0x2c, MATRIX_PREFIX],
        )
        .u32();
    append_viewer_string(e, strings.addr(), matrix_text);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c8ff70, bhk_blend_collision_object_add_rotation_get_r_t_t_i() -> u32),
        entry!(
            0x00c8_ff80,
            bhk_blend_collision_object_add_rotation_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00c8ff90,
            bhk_blend_collision_object_add_rotation_link_object(Ptr, u32)
        ),
        entry!(0x00c8ffa0, bhk_blend_collision_object_add_rotation_register_streamables(Ptr, Ptr) -> u32),
        entry!(
            0x00c8ffb0,
            bhk_blend_collision_object_add_rotation_save_binary(Ptr, Ptr)
        ),
        entry!(0x00c8ffc0, bhk_blend_collision_object_add_rotation_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c8ffd0, fn_00c8ffd0(Ptr) -> Ptr),
        entry!(0x00c90000, bhk_blend_collision_object_add_rotation_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00c90030,
            bhk_blend_collision_object_add_rotation_update_world_data(Ptr, u32)
        ),
        entry!(
            0x00c90090,
            bhk_blend_collision_object_add_rotation_copy_from(Ptr, Ptr)
        ),
        entry!(0x00c90160, bhk_blend_collision_object_add_rotation_create_clone(Ptr, u32) -> u32),
        entry!(
            0x00c90240,
            bhk_blend_collision_object_add_rotation_add_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c8ff80_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c8_ff80;
        const TARGET: u32 = 0x00c8_18f0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn bhk_blend_collision_object_add_rotation_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c8_ff70, &args![]).u32(), 0x0126_81cc);
    }

    use std::cell::{Cell, RefCell};
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

    /// Maps the default matrix: words 0x100 + index.
    fn default_matrix(e: &mut Engine) {
        e.map(DEFAULT_MATRIX, 0x24);
        for index in 0..9u32 {
            e.mem.set_u32(DEFAULT_MATRIX + 4 * index, 0x100 + index);
        }
    }

    #[test]
    fn thunks_forward_their_words_and_answers() {
        let mut e = Engine::new();
        e.register(BLEND_LINK_OBJECT, nothing);
        e.register(BLEND_SAVE_BINARY, nothing);
        e.register(BLEND_REGISTER_STREAMABLES, |_, a| answer(a[0] + a[1]));
        e.register(BLEND_IS_EQUAL, |_, a| answer(u32::from(a[0] == a[1])));
        e.call_log = Some(vec![]);
        e.call(0x00c8ff90, &args![0x100u32, 0x20u32]);
        assert_eq!(calls(&e, BLEND_LINK_OBJECT), [[0x100, 0x20]]);
        assert_eq!(e.call(0x00c8ffa0, &args![0x100u32, 0x20u32]).u32(), 0x120);
        e.call(0x00c8ffb0, &args![0x100u32, 0x20u32]);
        assert_eq!(calls(&e, BLEND_SAVE_BINARY), [[0x100, 0x20]]);
        assert_eq!(e.call(0x00c8ffc0, &args![5u32, 5u32]).u32(), 1);
        assert_eq!(e.call(0x00c8ffc0, &args![5u32, 6u32]).u32(), 0);
    }

    #[test]
    fn constructor_stores_the_vtable_and_the_default_matrix() {
        let mut e = Engine::new();
        default_matrix(&mut e);
        e.register(BLEND_CONSTRUCT, nothing);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c8ffd0, &args![this]).u32(), this);
        assert_eq!(calls(&e, BLEND_CONSTRUCT), [[this]]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
        for index in 0..9u32 {
            assert_eq!(e.mem.u32(this + 0x2c + 4 * index), 0x100 + index);
        }
        assert_eq!(e.mem.u32(this + 0x2c + 36), 0);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.register(BLEND_DESTRUCTOR, nothing);
        e.register(SIZED_DELETE, nothing);
        let this = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c90000, &args![this, 0xfffeu32]).u32(), this);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00c90000, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, SIZED_DELETE), [[this, 0x50]]);
        assert_eq!(calls(&e, BLEND_DESTRUCTOR), [[this], [this]]);
        assert_eq!(e.mem.u32(this), VTABLE_CLASS);
    }

    /// A rotation object whose world object (`+8`) has a parent (`+0x18`); the matrix
    /// multiplication is a double that records its call and writes a recognisable result.
    fn update_setup(e: &mut Engine, with_object: bool) -> (u32, u32, u32) {
        e.register(BLEND_UPDATE_WORLD_DATA, nothing);
        e.register(REFRESH_WORLD_DATA, nothing);
        let this = object(e, &[]);
        let node = e.mem.alloc(0x100);
        let parent = e.mem.alloc(0x100);
        e.mem.set_u32(this + 8, node);
        e.mem.set_u32(node + 0x18, parent);
        if with_object {
            let hk = e.mem.alloc(0x40);
            e.mem.set_u32(this + 0x10, hk);
        }
        let counter = Rc::new(Cell::new(0u32));
        e.register_double(MATRIX_MULTIPLY, move |e, a| {
            counter.set(counter.get() + 1);
            for index in 0..9u32 {
                e.mem
                    .set_u32(a[1] + 4 * index, 0x1000 * counter.get() + index);
            }
            answer(a[1])
        });
        e.call_log = Some(vec![]);
        (this, node, parent)
    }

    #[test]
    fn update_world_data_multiplies_the_matrices_into_the_node() {
        for with_object in [true, false] {
            let mut e = Engine::new();
            let (this, node, parent) = update_setup(&mut e, with_object);
            e.call(0x00c90030, &args![this, 0x42u32]);
            if with_object {
                assert_eq!(calls(&e, BLEND_UPDATE_WORLD_DATA), [[this, 0x42]]);
                assert!(calls(&e, REFRESH_WORLD_DATA).is_empty());
            } else {
                assert!(calls(&e, BLEND_UPDATE_WORLD_DATA).is_empty());
                assert_eq!(calls(&e, REFRESH_WORLD_DATA), [[node]]);
            }
            let products = calls(&e, MATRIX_MULTIPLY);
            assert_eq!(products.len(), 2);
            assert_eq!(products[0][0], parent + 0x68);
            assert_eq!(products[0][2], node + 0x34);
            assert_eq!(products[1][0], products[0][1]);
            assert_eq!(products[1][2], this + 0x2c);
            assert_ne!(products[0][1], products[1][1]);
            for index in 0..9u32 {
                assert_eq!(e.mem.u32(node + 0x68 + 4 * index), 0x2000 + index);
            }
        }
    }

    #[test]
    fn copy_from_moves_the_world_object_reference() {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            answer(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            answer(value)
        });
        e.register(SET_WORLD_OBJECT, nothing);
        e.register(0x0900_008c, nothing);
        e.register(0x0900_0004, nothing);
        let held = object(&mut e, &[(4, 0x0900_0004)]);
        e.mem.set_u32(held + 4, 1);
        let this = object(&mut e, &[(0x8c, 0x0900_008c)]);
        let other = e.mem.alloc(0x100);
        e.mem.set_u32(other + 0x10, held);
        e.mem.set_u32(other + 0x1c, 0x7f80_0001);
        e.mem.set_f32(other + 0x18, 0.25);
        e.mem.set_u32(other + 0x24, 0x99);
        e.mem.set_u32(other + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00c90090, &args![this, other]);
        assert_eq!(calls(&e, SET_WORLD_OBJECT), [[other, 0], [this, held]]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x18), 0.25);
        assert_eq!(e.mem.u32(this + 0x24), 0x99);
        assert_eq!(calls(&e, 0x0900_008c), [[this, 0x4444]]);
        // One reference was taken and dropped; the count is back at 1 and nothing is destroyed.
        assert_eq!(e.mem.u32(held + 4), 1);
        assert!(calls(&e, 0x0900_0004).is_empty());
        // Dropping the last reference runs the function at offset 4.
        e.mem.set_u32(held + 4, 0);
        e.call(0x00c90090, &args![this, other]);
        assert_eq!(calls(&e, 0x0900_0004), [[held]]);
        // No held object: no reference counting at all.
        e.mem.set_u32(other + 0x10, 0);
        e.call(0x00c90090, &args![this, other]);
        assert_eq!(calls(&e, INTERLOCKED_INCREMENT).len(), 2);
        assert_eq!(calls(&e, SET_WORLD_OBJECT).len(), 6);
    }

    #[test]
    fn create_clone_builds_the_object_and_copies_the_matrix() {
        let mut e = Engine::new();
        default_matrix(&mut e);
        let word = e.tls() + SCOPE_WORD;
        e.mem.set_u32(word, 0x55);
        let block = e.mem.alloc(0x60);
        e.register_double(HEAP_ALLOCATE, move |_, _| answer(block));
        e.register(BLEND_CONSTRUCT, nothing);
        e.register(BLEND_COPY, |e, _| {
            let scope = e.tls() + SCOPE_WORD;
            assert_eq!(e.mem.u32(scope), 0x10);
            Ret::default()
        });
        let this = e.mem.alloc(0x60);
        for index in 0..9u32 {
            e.mem.set_u32(this + 0x2c + 4 * index, 0x500 + index);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c90160, &args![this, 0x7700u32]).u32(), block);
        assert_eq!(calls(&e, HEAP_ALLOCATE), [[0x50]]);
        assert_eq!(calls(&e, BLEND_CONSTRUCT), [[block]]);
        assert_eq!(calls(&e, BLEND_COPY), [[this, block, 0x7700]]);
        assert_eq!(e.mem.u32(block), VTABLE_CLASS);
        for index in 0..9u32 {
            assert_eq!(e.mem.u32(block + 0x2c + 4 * index), 0x500 + index);
        }
        assert_eq!(e.mem.u32(word), 0x55);
    }

    #[test]
    fn add_viewer_strings_appends_the_class_name_and_the_matrix_text() {
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(CLASS_NAME_POINTER, 4);
            e.set_global(CLASS_NAME_POINTER, 0x5151_0001u32);
            let word = e.tls() + SCOPE_WORD;
            e.mem.set_u32(word, 0x66);
            e.register(BLEND_ADD_VIEWER_STRINGS, nothing);
            e.register(NI_GET_VIEWER_STRING, |_, _| answer(0x7777_0001));
            e.register(MATRIX_GET_VIEWER_STRING, |_, _| answer(0x7777_0002));
            e.register(STRINGS_GROW, nothing);
            let stored = Rc::new(RefCell::new(Vec::new()));
            let seen = stored.clone();
            e.register_double(STRINGS_SET_AT, move |e, a| {
                seen.borrow_mut().push((a[1], e.mem.u32(a[2])));
                Ret::default()
            });
            let this = object(&mut e, &[]);
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.call_log = Some(vec![]);
            e.call(0x00c90240, &args![this, strings]);
            assert_eq!(calls(&e, BLEND_ADD_VIEWER_STRINGS), [[this, strings]]);
            assert_eq!(calls(&e, NI_GET_VIEWER_STRING), [[0x5151_0001]]);
            assert_eq!(
                calls(&e, MATRIX_GET_VIEWER_STRING),
                [[this + 0x2c, MATRIX_PREFIX]]
            );
            let grown = calls(&e, STRINGS_GROW);
            let size = u32::from(size);
            if capacity > 3 {
                assert!(grown.is_empty());
            } else {
                // The mock does not advance the size, so both appends grow a full array.
                assert_eq!(grown, [[strings, 5 + size], [strings, 5 + size]]);
            }
            assert_eq!(*stored.borrow(), [(size, 0x7777_0001), (size, 0x7777_0002)]);
            assert_eq!(e.mem.u32(word), 0x66);
        }
    }
}
