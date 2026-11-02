//! `BSHavok/bhkgenericconstraint.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `bhkGenericConstraint` is a 0x10-byte constraint object (see `bhkhingeconstraint`) whose
//! cinfo holder keeps a 0x58-byte data block at `+4`. The exception-unwinding frames
//! (`FS:[0]` registrations) of the functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{enter_scope, havok_allocate, SERIALIZABLE_KILL};
use super::bhkhingeconstraint::{
    clone_constraint, create_cinfo_block, create_data_block, CINFO_CLONE_DATA, CINFO_LOAD_BINARY,
    CINFO_SAVE_BINARY, CONSTRAINT_DESTRUCT,
};

/// The generic constraint's vtable and live counter, its cinfo block's vtable and the data
/// block's constructor (`00d50c90`, thiscall on the block).
pub(crate) const GENERIC_VTABLE: u32 = 0x010c_9e5c;
pub(crate) const GENERIC_COUNT: u32 = 0x0126_8f74;
pub(crate) const GENERIC_CINFO_BLOCK_VTABLE: u32 = 0x010c_9f38;
pub(crate) const GENERIC_DATA_CONSTRUCT: u32 = 0x00d5_0c90;
/// Size of the data block.
pub(crate) const GENERIC_DATA_SIZE: u32 = 0x58;
/// The constraint classes up to this one, for `construct_constraint`.
pub(crate) const GENERIC_CLASSES: &[(u32, u32)] = &[(GENERIC_VTABLE, GENERIC_COUNT)];
/// The generic cinfo `CloneData` (`00d50090`).
pub(crate) const GENERIC_CLONE_DATA: u32 = 0x00d5_0090;

/// The body of the generic and fixed `CloneData`: with the allocation-scope word at 0x10,
/// uses `data` as the destination or, when it is 0, builds a new data block (0x58 bytes from
/// the current Havok allocator and its constructor `00d50c90`), then answers what `finish`
/// (`(this, data, context)`) answers.
pub(crate) fn clone_generic_data(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    context: u32,
    finish: u32,
) -> u32 {
    let (word, saved) = enter_scope(e);
    let data = if data == 0 {
        let block = havok_allocate(e, GENERIC_DATA_SIZE);
        e.call(GENERIC_DATA_CONSTRUCT, &args![block]).u32()
    } else {
        data
    };
    let result = e.call(finish, &args![this, data, context]).u32();
    e.mem.set_u32(word, saved);
    result
}

// Translated from 00d50000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGenericConstraintCinfo::LoadBinary` (Xbox PDB): a tail jump to the base cinfo load
/// (`00c8a230`) with the same argument; answers its `EAX`.
pub fn hk_generic_constraint_cinfo_load_binary(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(CINFO_LOAD_BINARY, &args![this, stream]).u32()
}

// Translated from 00d50010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGenericConstraintCinfo::SaveBinary` (Xbox PDB): a tail jump to the base cinfo save
/// (`00c8a280`) with the same argument; answers its `EAX`.
pub fn hk_generic_constraint_cinfo_save_binary(e: &mut Engine, this: Ptr, stream: u32) -> u32 {
    e.call(CINFO_SAVE_BINARY, &args![this, stream]).u32()
}

// Translated from 00d50020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d50020 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkGenericConstraint::~bhkGenericConstraint` (Xbox PDB): installs the
/// `bhkGenericConstraint` vtable, runs `bhkSerializable::Kill`, decrements the live counter of
/// this class and runs the `bhkConstraint` destructor body.
pub fn bhk_generic_constraint_destruct(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), GENERIC_VTABLE);
    e.call(SERIALIZABLE_KILL, &args![this]);
    let count = e.mem.u32(GENERIC_COUNT);
    e.mem.set_u32(GENERIC_COUNT, count.wrapping_sub(1));
    e.call(CONSTRAINT_DESTRUCT, &args![this]);
}

// Translated from 00d50090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d50090 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkGenericConstraintCinfo::CloneData` (Xbox PDB): `clone_generic_data` finishing with the
/// base `CloneData` (`00c8a1b0`).
pub fn hk_generic_constraint_cinfo_clone_data(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    context: u32,
) -> u32 {
    clone_generic_data(e, this, data, context, CINFO_CLONE_DATA)
}

// Translated from 00d50140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d50140 - the FS:[0] exception-unwinding frame is not modeled.
/// `hkGenericConstraintCinfo::CreateData` (no PDB name): builds the holder's data block when
/// it has none (`create_data_block`: 0x58 bytes and the constructor `00d50c90`). Answers the
/// size, 0x58.
pub fn fn_00d50140(e: &mut Engine, this: Ptr) -> u32 {
    create_data_block(e, this, GENERIC_DATA_SIZE, GENERIC_DATA_CONSTRUCT);
    GENERIC_DATA_SIZE
}

// Translated from 00d501f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d501f0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkGenericConstraint::CreateClone` (Xbox PDB): a new generic constraint filled from `this`
/// (`clone_constraint`). Answers the clone.
pub fn bhk_generic_constraint_create_clone(e: &mut Engine, this: Ptr, context: u32) -> u32 {
    clone_constraint(e, this, context, GENERIC_CLASSES)
}

// Translated from 00d502d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d502d0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkGenericConstraint::CreateCinfo` (Xbox PDB): `create_cinfo_block` with 0x14 bytes and the
/// cinfo block vtable `010c9f38`. `created` is the address of the byte that receives whether a
/// new block was made.
pub fn bhk_generic_constraint_create_cinfo(e: &mut Engine, this: Ptr, created: u32) -> u32 {
    create_cinfo_block(e, this, created, 0x14, GENERIC_CINFO_BLOCK_VTABLE)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5_0000,
            hk_generic_constraint_cinfo_load_binary(Ptr, u32) -> u32
        ),
        entry!(
            0x00d5_0010,
            hk_generic_constraint_cinfo_save_binary(Ptr, u32) -> u32
        ),
        entry!(0x00d5_0020, bhk_generic_constraint_destruct(Ptr)),
        entry!(
            0x00d5_0090,
            hk_generic_constraint_cinfo_clone_data(Ptr, u32, u32) -> u32
        ),
        entry!(0x00d5_0140, fn_00d50140(Ptr) -> u32),
        entry!(
            0x00d5_01f0,
            bhk_generic_constraint_create_clone(Ptr, u32) -> u32
        ),
        entry!(
            0x00d5_02d0,
            bhk_generic_constraint_create_cinfo(Ptr, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::bhkentity::testing::*;
    use super::super::bhkentity::{MEMORY_MANAGER, MEMORY_MANAGER_ALLOCATE, SCOPE_WORD};
    use super::super::bhkhingeconstraint::testing::*;
    use super::super::bhkhingeconstraint::{CONSTRAINT_ADOPT_CINFO, CONSTRAINT_COPY_MEMBERS};
    use super::*;

    #[test]
    fn load_and_save_pass_straight_through_to_the_base_cinfo() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        e.register(CINFO_LOAD_BINARY, |_, _| Ret {
            eax: 0x11,
            ..Ret::default()
        });
        e.register(CINFO_SAVE_BINARY, |_, _| Ret {
            eax: 0x22,
            ..Ret::default()
        });
        assert_eq!(
            e.call(0x00d5_0000, &args![0x1000u32, 0x2000u32]).u32(),
            0x11
        );
        assert_eq!(
            e.call(0x00d5_0010, &args![0x1000u32, 0x2000u32]).u32(),
            0x22
        );
        assert_eq!(calls_to(&e, CINFO_LOAD_BINARY), vec![vec![0x1000, 0x2000]]);
        assert_eq!(calls_to(&e, CINFO_SAVE_BINARY), vec![vec![0x1000, 0x2000]]);
    }

    #[test]
    fn destructor_installs_the_vtable_and_counts_the_constraint_down() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[GENERIC_COUNT]);
        e.mem.set_u32(GENERIC_COUNT, 2);
        let this = e.mem.alloc(0x10);
        e.call(0x00d5_0020, &args![this]);
        assert_eq!(e.mem.u32(this), GENERIC_VTABLE);
        assert_eq!(e.mem.u32(GENERIC_COUNT), 1);
        assert_eq!(calls_to(&e, SERIALIZABLE_KILL), vec![vec![this]]);
        assert_eq!(calls_to(&e, CONSTRAINT_DESTRUCT), vec![vec![this]]);
    }

    #[test]
    fn clone_data_uses_the_destination_or_builds_one() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        e.register(GENERIC_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(CINFO_CLONE_DATA, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        let tls = e.tls();
        e.mem.set_u32(tls + SCOPE_WORD, 6);
        assert_eq!(
            e.call(0x00d5_0090, &args![0x1000u32, 0x3000u32, 0x4000u32])
                .u32(),
            0x77
        );
        assert_eq!(e.mem.u32(tls + SCOPE_WORD), 6);
        assert!(calls_to(&e, GENERIC_DATA_CONSTRUCT).is_empty());
        assert_eq!(
            calls_to(&e, CINFO_CLONE_DATA),
            vec![vec![0x1000, 0x3000, 0x4000]]
        );
        e.call(0x00d5_0090, &args![0x1000u32, 0u32, 0x4000u32]);
        let made = calls_to(&e, GENERIC_DATA_CONSTRUCT)[0][0];
        assert_ne!(made, 0);
        assert_eq!(e.mem.u16(made + 4), 0x58);
        assert_eq!(
            calls_to(&e, CINFO_CLONE_DATA)[1],
            vec![0x1000, made, 0x4000]
        );
    }

    #[test]
    fn create_data_builds_the_block_once() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        e.register(GENERIC_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0] + 0x10,
            ..Ret::default()
        });
        let holder = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00d5_0140, &args![holder]).u32(), 0x58);
        let block = calls_to(&e, GENERIC_DATA_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(holder + 4), block + 0x10);
        assert_eq!(e.mem.u16(block + 4), 0x58);
        assert_eq!(e.call(0x00d5_0140, &args![holder]).u32(), 0x58);
        assert_eq!(calls_to(&e, GENERIC_DATA_CONSTRUCT).len(), 1);
    }

    #[test]
    fn create_clone_builds_a_generic_constraint() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[GENERIC_COUNT]);
        allocator_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        let clone = e.call(0x00d5_01f0, &args![this, 0x1234u32]).u32();
        assert_ne!(clone, 0);
        assert_eq!(e.mem.u32(clone), GENERIC_VTABLE);
        assert_eq!(e.mem.u32(GENERIC_COUNT), 1);
        assert_eq!(
            calls_to(&e, CONSTRAINT_COPY_MEMBERS),
            vec![vec![this, clone, 0x1234]]
        );
    }

    #[test]
    fn create_cinfo_makes_the_block_only_when_missing() {
        let mut e = logging_engine();
        constraint_doubles(&mut e, &[]);
        allocator_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        let created = e.mem.alloc(4);
        e.mem.set_u32(this + 8, 0x77);
        let cinfo = e.call(0x00d5_02d0, &args![this, created]).u32();
        assert_eq!(e.mem.u8(created), 1);
        assert_eq!(e.mem.u32(cinfo - 4), GENERIC_CINFO_BLOCK_VTABLE);
        assert_eq!(
            calls_to(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x14]]
        );
        assert_eq!(
            calls_to(&e, CONSTRAINT_ADOPT_CINFO),
            vec![vec![this, cinfo - 4]]
        );
        assert_eq!(e.call(0x00d5_02d0, &args![this, created]).u32(), cinfo);
        assert_eq!(e.mem.u8(created), 0);
    }
}
