//! `BSHavok/bhkfixedconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `bhkFixedConstraint` is a 0x10-byte constraint object built on `bhkGenericConstraint`
//! (see `bhkhingeconstraint`). The exception-unwinding frames (`FS:[0]` registrations) and
//! the `/GS` cookie check of the functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{enter_scope, HEAP_ALLOCATE, SERIALIZABLE_KILL};
use super::bhkgenericconstraint::{
    clone_generic_data, GENERIC_CINFO_BLOCK_VTABLE, GENERIC_CLASSES, GENERIC_CLONE_DATA,
    GENERIC_COUNT, GENERIC_VTABLE,
};
use super::bhkhingeconstraint::{
    bump, clone_constraint, construct_constraint, create_cinfo_block, new_constraint,
};

/// The fixed constraint's vtable and live counter, its cinfo block's vtable, and the
/// constraint classes up to it for `construct_constraint`.
const FIXED_VTABLE: u32 = 0x010c_9d7c;
const FIXED_COUNT: u32 = 0x0126_8624;
const FIXED_CINFO_BLOCK_VTABLE: u32 = 0x010c_9f54;
const FIXED_CLASSES: &[(u32, u32)] =
    &[(GENERIC_VTABLE, GENERIC_COUNT), (FIXED_VTABLE, FIXED_COUNT)];
/// The generic constraint's cinfo `LoadBinary` (`00d50000`), `SaveBinary` (`00d50010`) and
/// the destructor body (`00d50020`).
const GENERIC_LOAD_BINARY: u32 = 0x00d5_0000;
const GENERIC_SAVE_BINARY: u32 = 0x00d5_0010;
const GENERIC_DESTRUCT: u32 = 0x00d5_0020;
/// `bhkConstraint` routine that takes the cinfo holder a constraint was made from
/// (`00c89dd0`, thiscall on the constraint with the holder's address plus 4).
const CONSTRAINT_ATTACH_CINFO: u32 = 0x00c8_9dd0;
/// Teardown of a temporary cinfo holder (`00c89ac0`, thiscall on the holder, one argument,
/// always 0 here) and the base vtable it is given first (`0108ffb4`).
const HOLDER_TEARDOWN: u32 = 0x00c8_9ac0;
const HOLDER_BASE_VTABLE: u32 = 0x0108_ffb4;
/// The fixed cinfo constructor-like routine this unit's `FixConstraint` calls (`00cc7dc0`),
/// the `Fix` routine (`00cc7fe0`) and the identity transform it falls back on (`01268370`).
const FIXED_CONSTRUCT: u32 = 0x00cc_7dc0;
const FIX: u32 = 0x00cc_7fe0;
const DEFAULT_TRANSFORM: u32 = 0x0126_8370;
/// Routines `Fix` uses on its 0x1C-byte working object (`00d51720` builds it from the data
/// block, thiscall with one argument) and on `hkTransform`s (`00cb22f0`, thiscall on the
/// transform with `(source, argument)`).
const KIT_CONSTRUCT: u32 = 0x00d5_1720;
const TRANSFORM_CONSTRUCT: u32 = 0x00cb_22f0;
/// `Fix`'s further steps, thiscall on the working object: `00d52000` and `00d51c30` take
/// the first transform's translation / the first transform, `00d520a0` and `00d51d40` the
/// second's (their answers are kept at `+0x14` / `+0x18` of the holder), `00d51bc0` and
/// `00d51f90` take nothing, `00d52ae0` takes a float and `00d52ba0` and `00d52ca0` nothing.
const KIT_FIRST_TRANSLATION: u32 = 0x00d5_2000;
const KIT_FIRST_ROTATION: u32 = 0x00d5_1c30;
const KIT_SECOND_TRANSLATION: u32 = 0x00d5_20a0;
const KIT_SECOND_ROTATION: u32 = 0x00d5_1d40;
const KIT_STEP_A: u32 = 0x00d5_1bc0;
const KIT_STEP_B: u32 = 0x00d5_1f90;
const KIT_APPLY_FLOAT: u32 = 0x00d5_2ae0;
const KIT_STEP_C: u32 = 0x00d5_2ba0;
const KIT_STEP_D: u32 = 0x00d5_2ca0;
/// The float `Fix` hands to `00d52ae0`.
const KIT_FLOAT: u32 = 0x0101_e2bc;
/// The temporary wrapper `FixConstraint` builds for a scale below 1: its vtable
/// (`010c8288`), creation (`00ca9da0`), data setter (`00ca9e50`), scale setter (`00ca9840`)
/// and the constraint constructor that takes it (`00cb9a10`, thiscall on a 0x10-byte block).
const WRAPPER_VTABLE: u32 = 0x010c_8288;
const WRAPPER_CREATE_DATA: u32 = 0x00ca_9da0;
const WRAPPER_SET_DATA: u32 = 0x00ca_9e50;
const WRAPPER_SET_SCALE: u32 = 0x00ca_9840;
const WRAPPED_CONSTRUCT: u32 = 0x00cb_9a10;

// Translated from 00cc7ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFixedConstraintCinfo::LoadBinary` (Xbox PDB): a tail jump to the generic constraint's
/// `LoadBinary` (`00d50000`) with the same argument; answers its `EAX`.
pub fn hk_fixed_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(GENERIC_LOAD_BINARY, &args![this, stream]).u32()
}

// Translated from 00cc7cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkFixedConstraintCinfo::SaveBinary` (Xbox PDB): a tail jump to the generic constraint's
/// `SaveBinary` (`00d50010`) with the same argument; answers its `EAX`.
pub fn hk_fixed_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(GENERIC_SAVE_BINARY, &args![this, stream]).u32()
}

// Translated from 00cc7d00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7d00 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkFixedConstraint::~bhkFixedConstraint` (Xbox PDB): installs the `bhkFixedConstraint`
/// vtable, runs `bhkSerializable::Kill`, decrements the live counter of this class and runs
/// the generic constraint's destructor body (`00d50020`).
pub fn bhk_fixed_constraint_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FIXED_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(FIXED_COUNT);
    e.mem.set_u32(FIXED_COUNT, count.wrapping_sub(1));
    e.call(GENERIC_DESTRUCT, &args![this]);
}

// Translated from 00cc7dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7dc0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkFixedConstraint` constructor (no PDB name): builds the generic constraint in `this`
/// (`construct_constraint` up to the generic class), installs the fixed vtable, hands the
/// holder's address plus 4 (0 for no holder) to `00c89dd0` and counts the fixed constraint.
/// Answers `this`.
pub fn fn_00cc7dc0(e: &mut Engine, this: Ptr, holder: u32) -> Ptr {
    construct_constraint(e, this.addr(), GENERIC_CLASSES);
    e.mem.set_u32(this.addr(), FIXED_VTABLE);
    let adjusted = if holder == 0 { 0 } else { holder + 4 };
    e.call(CONSTRAINT_ATTACH_CINFO, &args![this, adjusted]);
    bump(e, FIXED_COUNT);
    this
}

// Translated from 00cc7e70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7e70 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkFixedConstraint` factory (no PDB name): allocates 0x10 bytes from the game heap and
/// constructs a fixed constraint in them (the constructor's steps are inline here, without
/// the call to `00c89dd0`). Answers the object, or 0 when the allocation failed (takes no
/// arguments).
pub fn fn_00cc7e70(e: &mut Engine) -> u32 {
    new_constraint(e, FIXED_CLASSES)
}

// Translated from 00cc7f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises a 0x14-byte cinfo block in `this` (no PDB name): zero at `+4`, reference count
/// 1 at `+8`, zeros at `+0xC` and `+0x10` and the generic cinfo block vtable `010c9f38`.
/// Answers `this`.
pub fn fn_00cc7f10(e: &mut Engine, this: Ptr) -> Ptr {
    let block = this.addr();
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 1);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 0);
    e.mem.set_u32(block, GENERIC_CINFO_BLOCK_VTABLE);
    this
}

// Translated from 00cc7f30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7f30 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkFixedConstraintCinfo::CloneData` (Xbox PDB): `clone_generic_data` finishing with the
/// generic constraint's `CloneData` (`00d50090`).
pub fn hk_fixed_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    context: u32,
) -> u32 {
    clone_generic_data(e, this, data, context, GENERIC_CLONE_DATA)
}

// Translated from 00cc7fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc7fe0 - the /GS cookie check (00ec408c) is not modeled.
/// `hkFixedConstraintCinfo::Fix` (Xbox PDB): has the holder build its data (virtual slot
/// `0x10`), then, on a 0xB4-byte frame holding a 0x1C-byte working object at `+8` (built
/// from the data block at `+4`) and two `hkTransform`s at `+0x24` and `+0x64`, builds the
/// transforms from the two bodies (`+0xC` and `+0x10`, each plus `0xF0`) and `transform`,
/// feeds the working object the first transform's translation (`00d52000`) and the
/// transform (`00d51c30`), keeps the answers of the second's (`00d520a0`, `00d51d40`) at
/// `+0x14` and `+0x18` of the holder and finishes with `00d51bc0`, `00d51f90`, the float
/// at `0101e2bc` (`00d52ae0`), `00d52ba0` and `00d52ca0`.
pub fn hk_fixed_constraint_cinfo_fix(e: &mut Engine, this: Ptr, transform: u32) {
    let holder = this.addr();
    e.with_stack(0xb4, |e, frame| {
        let frame = frame.addr();
        let kit = frame + 8;
        let first = frame + 0x24;
        let second = frame + 0x64;
        e.vcall(holder, 0x10, &args![]);
        let data = e.mem.u32(holder + 4);
        e.call(KIT_CONSTRUCT, &args![kit, data]);
        let body_a = e.mem.u32(holder + 0xc);
        let body_b = e.mem.u32(holder + 0x10);
        e.call(
            TRANSFORM_CONSTRUCT,
            &args![first, body_a.wrapping_add(0xf0), transform],
        );
        e.call(
            TRANSFORM_CONSTRUCT,
            &args![second, body_b.wrapping_add(0xf0), transform],
        );
        e.call(KIT_FIRST_TRANSLATION, &args![kit, first + 0x30]);
        e.call(KIT_FIRST_ROTATION, &args![kit, first]);
        let answer = e
            .call(KIT_SECOND_TRANSLATION, &args![kit, second + 0x30])
            .u32();
        e.mem.set_u32(holder + 0x14, answer);
        let answer = e.call(KIT_SECOND_ROTATION, &args![kit, second]).u32();
        e.mem.set_u32(holder + 0x18, answer);
        e.call(KIT_STEP_A, &args![kit]);
        e.call(KIT_STEP_B, &args![kit]);
        let float = x87_quiet_bits(e.global::<u32>(KIT_FLOAT));
        e.call(KIT_APPLY_FLOAT, &args![kit, float]);
        e.call(KIT_STEP_C, &args![kit]);
        e.call(KIT_STEP_D, &args![kit]);
    });
}

// Translated from 00cc80d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc80d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkFixedConstraint::CreateClone` (Xbox PDB): a new fixed constraint filled from `this`
/// (`clone_constraint`). Answers the clone.
pub fn bhk_fixed_constraint_create_clone(e: &mut Engine, this: Ptr, context: u32) -> u32 {
    clone_constraint(e, this, context, FIXED_CLASSES)
}

// Translated from 00cc81c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc81c0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkFixedConstraint::FixConstraint` (Xbox PDB): with the allocation-scope word at 0x10,
/// fills a temporary 0x1C-byte cinfo holder (vtable `010c9f54`, zero at `+4`, count 1 at
/// `+8`) from `object`'s virtual slots `0xCC` and `0xD0`: `+0xC` is `[[slot 0xCC result + 0xC] + 8]`
/// and `+0x10` is `[[slot 0xD0 result + 0xC] + 8]`, each 0 where a link is missing. Runs `Fix`
/// (`00cc7fe0`) on it with slot `0xCC`'s result plus `0xF0` (or the default transform
/// `01268370` when it has none). When `1.0 > scale` it then builds a 0x14-byte wrapper on
/// the stack (vtable `010c8288`, `00ca9da0`, `00ca9e50` with the holder's data, the
/// holder's `+0xC` / `+0x10` copied over, `00ca9840` with `scale`'s bit pattern) and makes a
/// 0x10-byte object from it through `00cb9a10`; otherwise it makes a fixed constraint from
/// the holder (`00cc7dc0`). Either object stays 0 when the allocation failed. The wrapper and
/// the holder are then given the base vtable `0108ffb4` and torn down (`00c89ac0` with 0).
/// Answers the new object. The wrapper's and the holder's last words (`+0x14`, `+0x18` of the
/// holder) are never set by the exe.
pub fn bhk_fixed_constraint_fix_constraint(e: &mut Engine, object: Ptr, scale: f32) -> u32 {
    let (word, saved) = enter_scope(e);
    let this = object.addr();
    // The wrapper and, right after it, the holder, as laid out in the exe's frame.
    let made = e.with_stack(0x30, |e, frame| {
        let wrapper = frame.addr();
        let holder = wrapper + 0x14;
        e.mem.set_u32(holder + 4, 0);
        e.mem.set_u32(holder + 8, 1);
        e.mem.set_u32(holder + 0xc, 0);
        e.mem.set_u32(holder + 0x10, 0);
        e.mem.set_u32(holder, FIXED_CINFO_BLOCK_VTABLE);
        let first = e.vcall(this, 0xd0, &args![]).u32();
        let first_body = if first != 0 {
            e.mem.u32(first + 0xc)
        } else {
            0
        };
        let second = e.vcall(this, 0xcc, &args![]).u32();
        let second_value = if second == 0 {
            0
        } else {
            let second_body = e.mem.u32(second + 0xc);
            if second_body == 0 {
                0
            } else {
                e.mem.u32(second_body + 8)
            }
        };
        e.mem.set_u32(holder + 0xc, second_value);
        let first_value = if first_body == 0 {
            0
        } else {
            e.mem.u32(first_body + 8)
        };
        e.mem.set_u32(holder + 0x10, first_value);
        let third = e.vcall(this, 0xcc, &args![]).u32();
        let transform = if third == 0 {
            DEFAULT_TRANSFORM
        } else {
            third + 0xf0
        };
        e.call(FIX, &args![holder, transform]);
        let below_one = 1.0 > scale;
        let made = if below_one {
            e.mem.set_u32(wrapper + 4, 0);
            e.mem.set_u32(wrapper + 8, 1);
            e.mem.set_u32(wrapper + 0xc, 0);
            e.mem.set_u32(wrapper + 0x10, 0);
            e.mem.set_u32(wrapper, WRAPPER_VTABLE);
            e.call(WRAPPER_CREATE_DATA, &args![wrapper]);
            let data = e.mem.u32(holder + 4);
            e.call(WRAPPER_SET_DATA, &args![wrapper, data]);
            let scale_bits = x87_quiet_bits(scale.to_bits());
            let copied_a = e.mem.u32(holder + 0xc);
            let copied_b = e.mem.u32(holder + 0x10);
            e.mem.set_u32(wrapper + 0xc, copied_a);
            e.mem.set_u32(wrapper + 0x10, copied_b);
            e.call(WRAPPER_SET_SCALE, &args![wrapper, scale_bits]);
            let block = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
            let made = if block != 0 {
                e.call(WRAPPED_CONSTRUCT, &args![block, wrapper]).u32()
            } else {
                0
            };
            e.mem.set_u32(wrapper, HOLDER_BASE_VTABLE);
            e.call(HOLDER_TEARDOWN, &args![wrapper, 0u32]);
            made
        } else {
            let block = e.call(HEAP_ALLOCATE, &args![0x10u32]).u32();
            if block != 0 {
                e.call(FIXED_CONSTRUCT, &args![block, holder]).u32()
            } else {
                0
            }
        };
        e.mem.set_u32(holder, HOLDER_BASE_VTABLE);
        e.call(HOLDER_TEARDOWN, &args![holder, 0u32]);
        made
    });
    e.mem.set_u32(word, saved);
    made
}

// Translated from 00cc83b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cc83b0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkFixedConstraint::CreateCinfo` (Xbox PDB): `create_cinfo_block` with 0x1C bytes and the
/// cinfo block vtable `010c9f54`. `created` is the address of the byte that receives whether
/// a new block was made.
pub fn bhk_fixed_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    create_cinfo_block(e, this, created, 0x1c, FIXED_CINFO_BLOCK_VTABLE)
}

// Translated from 00cc7da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkGenericConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00d50020`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_generic_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00d5_0020;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc7e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkFixedConstraint::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00cc7d00`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x10 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn bhk_fixed_constraint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00cc_7d00;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00cc7d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkFixedConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8628.
pub fn bhk_fixed_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8628
}

// Translated from 00cc7d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkGenericConstraint::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_8f78.
pub fn bhk_generic_constraint_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_8f78
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cc7ce0, hk_fixed_constraint_cinfo_load_binary(Ptr, u32) -> u32),
        entry!(0x00cc7cf0, hk_fixed_constraint_cinfo_save_binary(Ptr, u32) -> u32),
        entry!(0x00cc7d00, bhk_fixed_constraint_destruct(Ptr)),
        entry!(0x00cc7d70, bhk_fixed_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cc7d90, bhk_generic_constraint_get_r_t_t_i() -> u32),
        entry!(0x00cc7da0, bhk_generic_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc7dc0, fn_00cc7dc0(Ptr, u32) -> Ptr),
        entry!(0x00cc7e50, bhk_fixed_constraint_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00cc7e70, fn_00cc7e70() -> u32),
        entry!(0x00cc7f10, fn_00cc7f10(Ptr) -> Ptr),
        entry!(0x00cc7f30, hk_fixed_constraint_cinfo_clone_data(Ptr, u32, u32) -> u32),
        entry!(0x00cc7fe0, hk_fixed_constraint_cinfo_fix(Ptr, u32)),
        entry!(0x00cc80d0, bhk_fixed_constraint_create_clone(Ptr, u32) -> u32),
        entry!(0x00cc81c0, bhk_fixed_constraint_fix_constraint(Ptr, f32) -> u32),
        entry!(0x00cc83b0, bhk_fixed_constraint_create_cinfo(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::{MEMORY_MANAGER, MEMORY_MANAGER_ALLOCATE, SCOPE_WORD};
    use super::super::bhkhingeconstraint::testing::*;
    use super::super::bhkhingeconstraint::{
        CONSTRAINT_ADOPT_CINFO, CONSTRAINT_CONSTRUCT, CONSTRAINT_COPY_MEMBERS, CONSTRAINT_COUNT,
    };
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn bhk_generic_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00d5_0020, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc7da0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc7da0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00d5_0020), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_fixed_constraint_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00cc_7d00, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00cc7e50, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00cc7e50, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x10]]);
        assert_eq!(calls_to(&e, 0x00cc_7d00), vec![vec![this]; 2]);
    }

    #[test]
    fn bhk_fixed_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_7d70, &args![]).u32(), 0x0126_8628);
    }

    #[test]
    fn bhk_generic_constraint_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00cc_7d90, &args![]).u32(), 0x0126_8f78);
    }

    #[test]
    fn load_and_save_pass_straight_through_to_the_generic_cinfo() {
        let mut e = logging_engine();
        e.register(GENERIC_LOAD_BINARY, |_, _| Ret {
            eax: 0x11,
            ..Ret::default()
        });
        e.register(GENERIC_SAVE_BINARY, |_, _| Ret {
            eax: 0x22,
            ..Ret::default()
        });
        assert_eq!(
            e.call(0x00cc_7ce0, &args![0x1000u32, 0x2000u32]).u32(),
            0x11
        );
        assert_eq!(
            e.call(0x00cc_7cf0, &args![0x1000u32, 0x2000u32]).u32(),
            0x22
        );
        assert_eq!(
            calls_to(&e, GENERIC_LOAD_BINARY),
            vec![vec![0x1000, 0x2000]]
        );
        assert_eq!(
            calls_to(&e, GENERIC_SAVE_BINARY),
            vec![vec![0x1000, 0x2000]]
        );
    }

    #[test]
    fn destructor_installs_the_vtable_and_runs_the_generic_destructor() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[FIXED_COUNT]);
        e.register(GENERIC_DESTRUCT, |_, _| Ret::default());
        e.mem.set_u32(FIXED_COUNT, 2);
        let this = e.mem.alloc(0x10);
        e.call(0x00cc_7d00, &args![this]);
        assert_eq!(e.mem.u32(this), FIXED_VTABLE);
        assert_eq!(e.mem.u32(FIXED_COUNT), 1);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, GENERIC_DESTRUCT), vec![vec![this]]);
        assert!(calls_to(&e, super::super::bhkhingeconstraint::CONSTRAINT_DESTRUCT).is_empty());
    }

    #[test]
    fn constructor_builds_the_generic_part_then_attaches_the_holder() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[GENERIC_COUNT, FIXED_COUNT]);
        e.register(CONSTRAINT_ATTACH_CINFO, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00cc_7dc0, &args![this, 0x4000u32]).u32(), this);
        assert_eq!(e.mem.u32(this), FIXED_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 1);
        assert_eq!(e.mem.u32(GENERIC_COUNT), 1);
        assert_eq!(e.mem.u32(FIXED_COUNT), 1);
        assert_eq!(calls_to(&e, CONSTRAINT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(
            calls_to(&e, CONSTRAINT_ATTACH_CINFO),
            vec![vec![this, 0x4004]]
        );
        // No holder: null goes on.
        e.call(0x00cc_7dc0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, CONSTRAINT_ATTACH_CINFO)[1], vec![this, 0]);
        assert_eq!(e.mem.u32(FIXED_COUNT), 2);
    }

    #[test]
    fn factory_builds_a_fixed_constraint_or_nothing() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[GENERIC_COUNT, FIXED_COUNT]);
        allocator_doubles(&mut e);
        let object = e.call(0x00cc_7e70, &args![]).u32();
        assert_ne!(object, 0);
        assert_eq!(e.mem.u32(object), FIXED_VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(CONSTRAINT_COUNT), 1);
        assert_eq!(e.mem.u32(GENERIC_COUNT), 1);
        assert_eq!(e.mem.u32(FIXED_COUNT), 1);
        assert!(calls_to(&e, CONSTRAINT_ATTACH_CINFO).is_empty());
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_7e70, &args![]).u32(), 0);
        assert_eq!(e.mem.u32(FIXED_COUNT), 1);
    }

    #[test]
    fn block_initialiser_sets_the_generic_cinfo_block() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block + 4, 0xdead);
        assert_eq!(e.call(0x00cc_7f10, &args![block]).u32(), block);
        assert_eq!(e.mem.u32(block), GENERIC_CINFO_BLOCK_VTABLE);
        assert_eq!(e.mem.u32(block + 4), 0);
        assert_eq!(e.mem.u32(block + 8), 1);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(e.mem.u32(block + 0x10), 0);
    }

    #[test]
    fn clone_data_finishes_with_the_generic_clone_data() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        e.register(
            super::super::bhkgenericconstraint::GENERIC_DATA_CONSTRUCT,
            |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            },
        );
        e.register(GENERIC_CLONE_DATA, |_, _| Ret {
            eax: 0x99,
            ..Ret::default()
        });
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        assert_eq!(
            e.call(0x00cc_7f30, &args![0x1000u32, 0x3000u32, 0x4000u32])
                .u32(),
            0x99
        );
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(
            calls_to(&e, GENERIC_CLONE_DATA),
            vec![vec![0x1000, 0x3000, 0x4000]]
        );
        e.call(0x00cc_7f30, &args![0x1000u32, 0u32, 0x4000u32]);
        let made = calls_to(&e, GENERIC_CLONE_DATA)[1][1];
        assert_ne!(made, 0);
        assert_eq!(e.mem.u16(made + 4), 0x58);
    }

    #[test]
    fn fix_drives_the_working_object_in_order() {
        let mut e = logging_engine();
        // Slot 0x10 of the holder builds its data.
        let holder = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(holder, vtable);
        e.mem.set_u32(vtable + 0x10, 0x0900_0910);
        e.register(0x0900_0910, |_, _| Ret::default());
        e.mem.set_u32(holder + 4, 0x6000);
        e.mem.set_u32(holder + 0xc, 0x7000);
        e.mem.set_u32(holder + 0x10, 0x8000);
        for addr in [
            KIT_CONSTRUCT,
            TRANSFORM_CONSTRUCT,
            KIT_FIRST_TRANSLATION,
            KIT_FIRST_ROTATION,
            KIT_STEP_A,
            KIT_STEP_B,
            KIT_APPLY_FLOAT,
            KIT_STEP_C,
            KIT_STEP_D,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(KIT_SECOND_TRANSLATION, |_, _| Ret {
            eax: 0xaa,
            ..Ret::default()
        });
        e.register(KIT_SECOND_ROTATION, |_, _| Ret {
            eax: 0xbb,
            ..Ret::default()
        });
        e.map(KIT_FLOAT, 4);
        e.mem.set_u32(KIT_FLOAT, 0x7f80_0001);
        e.call(0x00cc_7fe0, &args![holder, 0x1234u32]);
        assert_eq!(e.mem.u32(holder + 0x14), 0xaa);
        assert_eq!(e.mem.u32(holder + 0x18), 0xbb);
        let kit = calls_to(&e, KIT_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&e, KIT_CONSTRUCT), vec![vec![kit, 0x6000]]);
        let transforms = calls_to(&e, TRANSFORM_CONSTRUCT);
        let (first, second) = (transforms[0][0], transforms[1][0]);
        assert_eq!(first, kit + 0x1c);
        assert_eq!(second, first + 0x40);
        assert_eq!(transforms[0][1..], [0x70f0, 0x1234]);
        assert_eq!(transforms[1][1..], [0x80f0, 0x1234]);
        assert_eq!(
            calls_to(&e, KIT_FIRST_TRANSLATION),
            vec![vec![kit, first + 0x30]]
        );
        assert_eq!(calls_to(&e, KIT_FIRST_ROTATION), vec![vec![kit, first]]);
        assert_eq!(
            calls_to(&e, KIT_SECOND_TRANSLATION),
            vec![vec![kit, second + 0x30]]
        );
        assert_eq!(calls_to(&e, KIT_SECOND_ROTATION), vec![vec![kit, second]]);
        for addr in [KIT_STEP_A, KIT_STEP_B, KIT_STEP_C, KIT_STEP_D] {
            assert_eq!(calls_to(&e, addr), vec![vec![kit]]);
        }
        // The float goes on quiet, as a bit pattern.
        assert_eq!(calls_to(&e, KIT_APPLY_FLOAT), vec![vec![kit, 0x7fc0_0001]]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x00cc_7fe0,
                0x0900_0910,
                KIT_CONSTRUCT,
                TRANSFORM_CONSTRUCT,
                TRANSFORM_CONSTRUCT,
                KIT_FIRST_TRANSLATION,
                KIT_FIRST_ROTATION,
                KIT_SECOND_TRANSLATION,
                KIT_SECOND_ROTATION,
                KIT_STEP_A,
                KIT_STEP_B,
                KIT_APPLY_FLOAT,
                KIT_STEP_C,
                KIT_STEP_D
            ]
        );
    }

    #[test]
    fn create_clone_builds_a_fixed_constraint_with_both_counters() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[GENERIC_COUNT, FIXED_COUNT]);
        allocator_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        let clone = e.call(0x00cc_80d0, &args![this, 0x1234u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), FIXED_VTABLE);
        assert_eq!(e.mem.u32(GENERIC_COUNT), 1);
        assert_eq!(e.mem.u32(FIXED_COUNT), 1);
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert_eq!(
            calls_to(&e, CONSTRAINT_COPY_MEMBERS),
            vec![vec![this, clone, 0x1234]]
        );
    }

    /// An object whose virtual slots `0xD0` and `0xCC` answer `link_d0` and `link_cc`, which
    /// is what `fix_constraint` asks.
    fn linked_object(e: &mut Engine, link_d0: u32, link_cc: u32) -> u32 {
        let object = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(vtable + 0xd0, 0x0900_0d00);
        e.mem.set_u32(vtable + 0xcc, 0x0900_0cc0);
        e.register_double(0x0900_0d00, move |_, _| Ret {
            eax: link_d0,
            ..Ret::default()
        });
        e.register_double(0x0900_0cc0, move |_, _| Ret {
            eax: link_cc,
            ..Ret::default()
        });
        object
    }

    /// A link record whose `+0xC` points to a body whose `+8` holds `value`.
    fn link_with_value(e: &mut Engine, value: u32) -> u32 {
        let link = e.mem.alloc(0x100);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(link + 0xc, body);
        e.mem.set_u32(body + 8, value);
        link
    }

    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Doubles that record, for `Fix` and the holder teardown, the holder's words as they
    /// were at the time of the call.
    fn fix_doubles(e: &mut Engine, seen: &Seen) {
        let log = Rc::clone(seen);
        e.register_double(FIX, move |e, a| {
            let mut words = vec![a[0], a[1]];
            words.extend((0..5).map(|i| e.mem.u32(a[0] + 4 * i)));
            log.borrow_mut().push(words);
            Ret::default()
        });
        let log = Rc::clone(seen);
        e.register_double(HOLDER_TEARDOWN, move |e, a| {
            log.borrow_mut().push(vec![a[0], a[1], e.mem.u32(a[0])]);
            Ret::default()
        });
        e.register(FIXED_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 0x1000,
            ..Ret::default()
        });
    }

    #[test]
    fn fix_constraint_makes_a_fixed_constraint_for_a_scale_of_one_or_more() {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        let seen: Seen = Rc::default();
        fix_doubles(&mut e, &seen);
        let first = link_with_value(&mut e, 0x1111);
        let second = link_with_value(&mut e, 0x2222);
        let object = linked_object(&mut e, first, second);
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        let made = e.call(0x00cc_81c0, &args![object, 1.0f32]).u32();
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        let seen = seen.borrow().clone();
        // Fix: the holder (vtable, zero, count 1, the two values) and the second link's transform.
        let holder = seen[0][0];
        assert_eq!(seen[0][1], second + 0xf0);
        assert_eq!(
            seen[0][2..],
            [FIXED_CINFO_BLOCK_VTABLE, 0, 1, 0x2222, 0x1111]
        );
        let built = calls_to(&e, FIXED_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1], holder);
        assert_eq!(made, built[0][0] + 0x1000);
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x10]]);
        // The holder is torn down with the base vtable and a 0 argument.
        assert_eq!(seen[1], vec![holder, 0, HOLDER_BASE_VTABLE]);
        assert_eq!(seen.len(), 2);
        assert!(calls_to(&e, WRAPPER_CREATE_DATA).is_empty());
    }

    #[test]
    fn fix_constraint_without_links_uses_the_default_transform_and_zero_values() {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        let seen: Seen = Rc::default();
        fix_doubles(&mut e, &seen);
        let object = linked_object(&mut e, 0, 0);
        // A NaN scale is not below 1.
        e.call(0x00cc_81c0, &args![object, f32::NAN]);
        let seen = seen.borrow().clone();
        assert_eq!(seen[0][1], DEFAULT_TRANSFORM);
        assert_eq!(seen[0][2..], [FIXED_CINFO_BLOCK_VTABLE, 0, 1, 0, 0]);
        assert_eq!(calls_to(&e, FIXED_CONSTRUCT).len(), 1);
        // An allocation that fails leaves the result null (the holder is still torn down).
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        assert_eq!(e.call(0x00cc_81c0, &args![object, 2.0f32]).u32(), 0);
        assert_eq!(calls_to(&e, FIXED_CONSTRUCT).len(), 1);
        assert_eq!(seen.len(), 2);
        assert_eq!(calls_to(&e, HOLDER_TEARDOWN).len(), 2);
    }

    #[test]
    fn fix_constraint_wraps_the_holder_for_a_scale_below_one() {
        let mut e = logging_engine();
        allocator_doubles(&mut e);
        let seen: Seen = Rc::default();
        fix_doubles(&mut e, &seen);
        let log = Rc::clone(&seen);
        e.register_double(WRAPPER_SET_SCALE, move |e, a| {
            // The wrapper's words at the time: vtable, +4, +8, and the copied pair.
            let words: Vec<u32> = [0, 4, 8, 0xc, 0x10]
                .iter()
                .map(|&offset| e.mem.u32(a[0] + offset))
                .collect();
            log.borrow_mut().push([vec![a[0], a[1]], words].concat());
            Ret::default()
        });
        e.register(WRAPPER_CREATE_DATA, |_, _| Ret::default());
        e.register(WRAPPER_SET_DATA, |_, _| Ret::default());
        e.register(WRAPPED_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 0x2000,
            ..Ret::default()
        });
        let first = link_with_value(&mut e, 0x1111);
        let second = link_with_value(&mut e, 0x2222);
        let object = linked_object(&mut e, first, second);
        let made = e.call(0x00cc_81c0, &args![object, 0.5f32]).u32();
        let seen = seen.borrow().clone();
        let holder = seen[0][0];
        let wrapper = holder - 0x14;
        assert_eq!(calls_to(&e, WRAPPER_CREATE_DATA), vec![vec![wrapper]]);
        assert_eq!(calls_to(&e, WRAPPER_SET_DATA), vec![vec![wrapper, 0]]);
        // The scale goes on as its bit pattern, with the pair copied from the holder.
        assert_eq!(
            seen[1],
            vec![
                wrapper,
                0.5f32.to_bits(),
                WRAPPER_VTABLE,
                0,
                1,
                0x2222,
                0x1111
            ]
        );
        let built = calls_to(&e, WRAPPED_CONSTRUCT);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1], wrapper);
        assert_eq!(made, built[0][0] + 0x2000);
        assert!(calls_to(&e, FIXED_CONSTRUCT).is_empty());
        // The wrapper is torn down first, then the holder, each with the base vtable.
        assert_eq!(seen[2], vec![wrapper, 0, HOLDER_BASE_VTABLE]);
        assert_eq!(seen[3], vec![holder, 0, HOLDER_BASE_VTABLE]);
    }

    #[test]
    fn create_cinfo_makes_a_larger_block_only_when_missing() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        let created = e.mem.alloc(4);
        e.mem.set_u32(this + 8, 0x77);
        let cinfo = e.call(0x00cc_83b0, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(cinfo - 4), FIXED_CINFO_BLOCK_VTABLE);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1c]]
        );
        assert_eq!(
            calls_to(&e, CONSTRAINT_ADOPT_CINFO),
            vec![vec![this, cinfo - 4]]
        );
        assert_eq!(e.call(0x00cc_83b0, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
    }
}
