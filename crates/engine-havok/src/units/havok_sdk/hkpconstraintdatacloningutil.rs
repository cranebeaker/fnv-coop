//! `Havok SDK/hkpconstraintdatacloningutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Every `copyConstraintAtoms` instance has the same shape: allocate a constraint-data object of
//! a given size through the allocator (slot 4), store the size in the word at +4, run the class's
//! constructor on it and copy a range of bytes (`hkString::memCpy`, `00cb1260`) from the
//! original; they differ in size, constructor and range, and are cdecl with the original as the
//! only argument.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkString::memCpy(dst, src, size)` (Xbox PDB).
const MEM_COPY: u32 = 0x00cb_1260;
/// Removes a reference from the object in `ECX`.
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `00d85300(this, source)`: the base copy of an atoms block (RET 4).
const COPY_ATOMS_BASE: u32 = 0x00d8_5300;

/// A block of `size` bytes from the allocator (slot 4) with its size stored at +4.
fn allocate_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block + 4, size as u16);
    block
}

/// The body of every `copyConstraintAtoms` instance: the block, the constructor's answer (`this`)
/// and the copy of `length` bytes at `offset` from `source`.
fn copy_constraint_atoms(
    e: &mut Engine,
    source: Ptr,
    size: u32,
    construct: impl FnOnce(&mut Engine, Ptr) -> Ptr,
    offset: u32,
    length: u32,
) -> Ptr {
    let block = allocate_block(e, size);
    let object = construct(e, Ptr::new(block));
    e.call(
        MEM_COPY,
        &args![object.addr() + offset, source.addr() + offset, length],
    );
    object
}

/// A constructor of the exe, run on the block (thiscall, no arguments, answering `this`).
fn construct_at(address: u32) -> impl FnOnce(&mut Engine, Ptr) -> Ptr {
    move |e, block| e.call(address, &args![block]).ptr()
}

// Translated from 00d84bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a constraint-data class, unconfirmed; the constructor of the
/// object `00d855c0` copies): `this` is the block (ECX). Sets the vtable `010d37e4`, the word at
/// +6 to 1, the word at +8 to 0, the word at +0x10 to 2, the word at +0xa0 to 7, clears
/// 0x20..0x9f and puts 1.0 on the diagonals of the two 3x3 blocks (+0x20, +0x34, +0x48 and
/// +0x60, +0x74, +0x88); the byte at +0xa2 is 0. Returns `this`.
pub fn fn_00d84bd0(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, 0x010d_37e4);
    e.mem.set_u16(at + 6, 1);
    e.mem.set_u32(at + 8, 0);
    e.mem.set_u16(at + 0x10, 2);
    e.mem.set_u16(at + 0xa0, 7);
    for offset in (0x20..0xa0).step_by(4) {
        e.mem.set_u32(at + offset, 0);
    }
    for offset in [0x20, 0x34, 0x48, 0x60, 0x74, 0x88] {
        e.mem.set_f32(at + offset, 1.0);
    }
    e.mem.set_u8(at + 0xa2, 0);
    this
}

// Translated from 00d85360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpBallAndSocketConstraintData>` (Xbox PDB): a 0x50-byte object built by
/// the constructor `00d54290`, with the 0x38 bytes at +0x10 copied from `source`.
pub fn copy_constraint_atoms_ball_and_socket(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0x50, construct_at(0x00d5_4290), 0x10, 0x38)
}

// Translated from 00d853a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpHingeConstraintData>` (Xbox PDB): 0xb0 bytes, constructor
/// `00cbe7b0`, 0x9c bytes at +0x10.
pub fn copy_constraint_atoms_hinge(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0xb0, construct_at(0x00cb_e7b0), 0x10, 0x9c)
}

// Translated from 00d853f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpLimitedHingeConstraintData>` (Xbox PDB): 0xe0 bytes, constructor
/// `00cb9090`, 0xcc bytes at +0x10.
pub fn copy_constraint_atoms_limited_hinge(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0xe0, construct_at(0x00cb_9090), 0x10, 0xcc)
}

// Translated from 00d85440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpPointToPathConstraintData>` (Xbox PDB): 0xb0 bytes, constructor
/// `00dee750`, only the 0xc bytes at +0xc.
pub fn copy_constraint_atoms_point_to_path(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0xb0, construct_at(0x00de_e750), 0xc, 0xc)
}

// Translated from 00d85490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpPrismaticConstraintData>` (Xbox PDB): 0xd0 bytes, constructor
/// `00d4fe80`, 0xc0 bytes at +0x10.
pub fn copy_constraint_atoms_prismatic(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0xd0, construct_at(0x00d4_fe80), 0x10, 0xc0)
}

// Translated from 00d854e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpRagdollConstraintData>` (Xbox PDB): 0x140 bytes, constructor
/// `00cdf2f0`, 0x130 bytes at +0x10.
pub fn copy_constraint_atoms_ragdoll(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0x140, construct_at(0x00cd_f2f0), 0x10, 0x130)
}

// Translated from 00d85530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpStiffSpringConstraintData>` (Xbox PDB): 0x50 bytes, constructor
/// `00d4f0f0`, 0x38 bytes at +0x10.
pub fn copy_constraint_atoms_stiff_spring(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0x50, construct_at(0x00d4_f0f0), 0x10, 0x38)
}

// Translated from 00d85570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpWheelConstraintData>` (Xbox PDB): 0x160 bytes, constructor
/// `00d4edc0`, 0x124 bytes at +0x10.
pub fn copy_constraint_atoms_wheel(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0x160, construct_at(0x00d4_edc0), 0x10, 0x124)
}

// Translated from 00d855c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the same shape as `copyConstraintAtoms`): 0xb0 bytes, the
/// constructor `00d84bd0` of this unit, 0x94 bytes at +0x10.
pub fn fn_00d855c0(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0xb0, fn_00d84bd0, 0x10, 0x94)
}

// Translated from 00d85610 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the same shape as `copyConstraintAtoms`): 0x80 bytes,
/// constructor `00deefd0`, 0x70 bytes at +0x10.
pub fn fn_00d85610(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0x80, construct_at(0x00de_efd0), 0x10, 0x70)
}

// Translated from 00d85660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpHingeLimitsData>` (Xbox PDB): 0xa0 bytes, constructor `00d52fc0`,
/// 0x84 bytes at +0x10.
pub fn copy_constraint_atoms_hinge_limits(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0xa0, construct_at(0x00d5_2fc0), 0x10, 0x84)
}

// Translated from 00d856b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `copyConstraintAtoms<hkpRagdollLimitsData>` (Xbox PDB): 0xc0 bytes, constructor `00d4f2a0`,
/// 0xac bytes at +0x10.
pub fn copy_constraint_atoms_ragdoll_limits(e: &mut Engine, source: Ptr) -> Ptr {
    copy_constraint_atoms(e, source, 0xc0, construct_at(0x00d4_f2a0), 0x10, 0xac)
}

// Translated from 00d85700 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (it clones any constraint data by type; cdecl, `data` the original).
/// The type is the answer of the virtual function at `+0x20`; types 0 to 3, 6 to 9, 14, 15, 18
/// and 19 copy the object with the matching function of this unit (0 ball and socket, 1 hinge,
/// 2 limited hinge, 3 point to path, 6 prismatic, 7 ragdoll, 8 stiff spring, 9 wheel, 14
/// `00d855c0`, 15 `00d85610`, 18 hinge limits, 19 ragdoll limits). For type 3 the original's
/// object at +0x18 is asked for its virtual function at `+0x34` (answer kept), the copy gets
/// the original object at +0x18 through `00dee6f0(copy, object)` and a reference is removed from
/// the kept answer. Types 12 and 13 wrap a clone of the inner data (the word at +0x18 for 12,
/// the answer of `00cdf6d0(data)` for 13), made by this function, in a new 0x28-byte
/// (constructor `00d4df30`) or 0x20-byte (constructor `00cdf710`) object, with a reference
/// removed from the clone; the wrapper takes the float at +0x20 (12) or +0x1c (13) through the
/// x87 stack, for 12 also the bytes at +0x24 and +0x25, and the word at +8. If the inner clone
/// is null, or for any other type, the answer is null.
pub fn fn_00d85700(e: &mut Engine, data: Ptr) -> Ptr {
    let at = data.addr();
    let kind = e.vcall(at, 0x20, &args![]).u32();
    match kind {
        0 => copy_constraint_atoms_ball_and_socket(e, data),
        1 => copy_constraint_atoms_hinge(e, data),
        2 => copy_constraint_atoms_limited_hinge(e, data),
        3 => {
            let copy = copy_constraint_atoms_point_to_path(e, data);
            let path = e.mem.u32(at + 0x18);
            let answer = e.vcall(path, 0x34, &args![]).u32();
            e.call(0x00de_e6f0, &args![copy, path]);
            e.call(REMOVE_REFERENCE, &args![answer]);
            copy
        }
        6 => copy_constraint_atoms_prismatic(e, data),
        7 => copy_constraint_atoms_ragdoll(e, data),
        8 => copy_constraint_atoms_stiff_spring(e, data),
        9 => copy_constraint_atoms_wheel(e, data),
        12 => {
            let inner = e.mem.u32(at + 0x18);
            let child = fn_00d85700(e, Ptr::new(inner));
            if child.is_null() {
                return Ptr::NULL;
            }
            let block = allocate_block(e, 0x28);
            let copy: Ptr = e.call(0x00d4_df30, &args![block, child]).ptr();
            e.call(REMOVE_REFERENCE, &args![child]);
            let strength = x87_quiet(e.mem.f32(at + 0x20));
            e.mem.set_f32(copy.addr() + 0x20, strength);
            let flag = e.mem.u8(at + 0x24);
            e.mem.set_u8(copy.addr() + 0x24, flag);
            let flag = e.mem.u8(at + 0x25);
            e.mem.set_u8(copy.addr() + 0x25, flag);
            let word = e.mem.u32(at + 8);
            e.mem.set_u32(copy.addr() + 8, word);
            copy
        }
        13 => {
            let inner = e.call(0x00cd_f6d0, &args![data]).u32();
            let child = fn_00d85700(e, Ptr::new(inner));
            if child.is_null() {
                return Ptr::NULL;
            }
            let block = allocate_block(e, 0x20);
            let copy: Ptr = e.call(0x00cd_f710, &args![block, child]).ptr();
            e.call(REMOVE_REFERENCE, &args![child]);
            let value = x87_quiet(e.mem.f32(at + 0x1c));
            e.mem.set_f32(copy.addr() + 0x1c, value);
            let word = e.mem.u32(at + 8);
            e.mem.set_u32(copy.addr() + 8, word);
            copy
        }
        14 => fn_00d855c0(e, data),
        15 => fn_00d85610(e, data),
        18 => copy_constraint_atoms_hinge_limits(e, data),
        19 => copy_constraint_atoms_ragdoll_limits(e, data),
        _ => Ptr::NULL,
    }
}

// Translated from 00d858f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintDataCloningUtil::cloneIfCanHaveMotors` (Xbox PDB), cdecl `(data)`: by the type
/// the virtual function at `+0x20` answers: 2 (limited hinge, 0xe0 bytes, constructor
/// `00cb9090`, atoms copied by `00d85a20`, motor at +0xb0), 6 (prismatic, 0xd0 bytes,
/// `00d4fe80`, `00d85af0`, motor at +0xac) and 7 (ragdoll, 0x140 bytes, `00cdf2f0`, `00d85ba0`,
/// the three motors at +0xe0, +0xe4, +0xe8) make an object, copy the atoms from `data + 0x10`
/// into `object + 0x10`, copy the word at +8 and replace each motor that is not null by the
/// answer of its virtual function at `+0xc`; any other type answers null.
pub fn hkp_constraint_data_cloning_util_clone_if_can_have_motors(e: &mut Engine, data: Ptr) -> Ptr {
    let at = data.addr();
    let kind = e.vcall(at, 0x20, &args![]).u32();
    let (size, construct, motors): (u32, u32, &[u32]) = match kind {
        2 => (0xe0, 0x00cb_9090, &[0xb0][..]),
        6 => (0xd0, 0x00d4_fe80, &[0xac][..]),
        7 => (0x140, 0x00cd_f2f0, &[0xe0, 0xe4, 0xe8][..]),
        _ => return Ptr::NULL,
    };
    let block = allocate_block(e, size);
    let copy: Ptr = e.call(construct, &args![block]).ptr();
    let atoms = copy.addr() + 0x10;
    match kind {
        2 => {
            hkp_limited_hinge_constraint_data_atoms_assign(e, Ptr::new(atoms), Ptr::new(at + 0x10))
        }
        6 => fn_00d85af0(e, Ptr::new(atoms), Ptr::new(at + 0x10)),
        _ => fn_00d85ba0(e, Ptr::new(atoms), Ptr::new(at + 0x10)),
    };
    let word = e.mem.u32(at + 8);
    e.mem.set_u32(copy.addr() + 8, word);
    for motor_offset in motors {
        let motor = e.mem.u32(copy.addr() + motor_offset);
        if motor != 0 {
            let cloned = e.vcall(motor, 0xc, &args![]).u32();
            e.mem.set_u32(copy.addr() + motor_offset, cloned);
        }
    }
    copy
}

// Translated from 00d85a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpLimitedHingeConstraintData::Atoms::operator_` (Xbox PDB), thiscall with one argument:
/// the base copy `00d85300(this, source)`, then the 15 words at +0x90 to +0xc8 copied from
/// `source`. Returns `this`.
pub fn hkp_limited_hinge_constraint_data_atoms_assign(
    e: &mut Engine,
    this: Ptr,
    source: Ptr,
) -> Ptr {
    e.call(COPY_ATOMS_BASE, &args![this, source]);
    for offset in (0x90..=0xc8).step_by(4) {
        let word = e.mem.u32(source.addr() + offset);
        e.mem.set_u32(this.addr() + offset, word);
    }
    this
}

// Translated from 00d85af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the atoms assignment of the prismatic data, thiscall with one
/// argument): the base copy `00d85300(this, source)`, then the 12 words at +0x90 to +0xbc.
/// Returns `this`.
pub fn fn_00d85af0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(COPY_ATOMS_BASE, &args![this, source]);
    for offset in (0x90..=0xbc).step_by(4) {
        let word = e.mem.u32(source.addr() + offset);
        e.mem.set_u32(this.addr() + offset, word);
    }
    this
}

// Translated from 00d85ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (the atoms assignment of the ragdoll data, thiscall with one
/// argument): the base copy `00d85300(this, source)`, then the word at +0x90, the byte at +0x92,
/// the words at +0x94 and +0x96, the three 16-byte vectors at +0xa0, +0xb0 and +0xc0, the
/// dwords at +0xd0 to +0xd8 and the 20 dwords at +0xe0 to +0x12c. Returns `this`.
pub fn fn_00d85ba0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(COPY_ATOMS_BASE, &args![this, source]);
    let (to, from) = (this.addr(), source.addr());
    let word = e.mem.u16(from + 0x90);
    e.mem.set_u16(to + 0x90, word);
    let byte = e.mem.u8(from + 0x92);
    e.mem.set_u8(to + 0x92, byte);
    let word = e.mem.u16(from + 0x94);
    e.mem.set_u16(to + 0x94, word);
    let word = e.mem.u16(from + 0x96);
    e.mem.set_u16(to + 0x96, word);
    for offset in (0xa0..0xd0).step_by(4) {
        let value = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, value);
    }
    for offset in [0xd0, 0xd4, 0xd8] {
        let value = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, value);
    }
    for offset in (0xe0..=0x12c).step_by(4) {
        let value = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, value);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d8_4bd0, fn_00d84bd0(Ptr) -> Ptr),
        entry!(0x00d8_5360, copy_constraint_atoms_ball_and_socket(Ptr) -> Ptr),
        entry!(0x00d8_53a0, copy_constraint_atoms_hinge(Ptr) -> Ptr),
        entry!(0x00d8_53f0, copy_constraint_atoms_limited_hinge(Ptr) -> Ptr),
        entry!(0x00d8_5440, copy_constraint_atoms_point_to_path(Ptr) -> Ptr),
        entry!(0x00d8_5490, copy_constraint_atoms_prismatic(Ptr) -> Ptr),
        entry!(0x00d8_54e0, copy_constraint_atoms_ragdoll(Ptr) -> Ptr),
        entry!(0x00d8_5530, copy_constraint_atoms_stiff_spring(Ptr) -> Ptr),
        entry!(0x00d8_5570, copy_constraint_atoms_wheel(Ptr) -> Ptr),
        entry!(0x00d8_55c0, fn_00d855c0(Ptr) -> Ptr),
        entry!(0x00d8_5610, fn_00d85610(Ptr) -> Ptr),
        entry!(0x00d8_5660, copy_constraint_atoms_hinge_limits(Ptr) -> Ptr),
        entry!(0x00d8_56b0, copy_constraint_atoms_ragdoll_limits(Ptr) -> Ptr),
        entry!(0x00d8_5700, fn_00d85700(Ptr) -> Ptr),
        entry!(0x00d8_58f0, hkp_constraint_data_cloning_util_clone_if_can_have_motors(Ptr) -> Ptr),
        entry!(0x00d8_5a20, hkp_limited_hinge_constraint_data_atoms_assign(Ptr, Ptr) -> Ptr),
        entry!(0x00d8_5af0, fn_00d85af0(Ptr, Ptr) -> Ptr),
        entry!(0x00d8_5ba0, fn_00d85ba0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod havok_a03_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATE_SLOT: u32 = 0x0900_0004;
    const TYPE_SLOT: u32 = 0x0900_0020;
    const PATH_SLOT: u32 = 0x0900_0034;
    const MOTOR_CLONE_SLOT: u32 = 0x0900_000c;
    const PATH_ANSWER: u32 = 0x7a7a_0001;
    const MOTOR_CLONE: u32 = 0xc10e_0001;

    type CallLog = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// The constructors of the exe the `copyConstraintAtoms` instances run.
    const CONSTRUCTORS: [u32; 11] = [
        0x00d5_4290,
        0x00cb_e7b0,
        0x00cb_9090,
        0x00de_e750,
        0x00d4_fe80,
        0x00cd_f2f0,
        0x00d4_f0f0,
        0x00d4_edc0,
        0x00de_efd0,
        0x00d5_2fc0,
        0x00d4_f2a0,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    /// A memory router whose allocator hands out blocks filled with `0xcc` (logging the size);
    /// doubles over every callee: constructors that store their own address at +0 and answer
    /// `this`, `memCpy` that copies, and loggers for the rest.
    fn install(e: &mut Engine) -> CallLog {
        let log: CallLog = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            sink.borrow_mut().push((ALLOCATE_SLOT, a.to_vec()));
            let block = e.mem.alloc(a[1]);
            for offset in 0..a[1] {
                e.mem.set_u8(block + offset, 0xcc);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        for address in CONSTRUCTORS.into_iter().chain([0x00d4_df30, 0x00cd_f710]) {
            let sink = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                e.mem.set_u32(a[0], address);
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
        let sink = Rc::clone(&log);
        e.register_double(MEM_COPY, move |e, a| {
            sink.borrow_mut().push((MEM_COPY, a.to_vec()));
            for offset in 0..a[2] {
                let byte = e.mem.u8(a[1] + offset);
                e.mem.set_u8(a[0] + offset, byte);
            }
            Ret::default()
        });
        for address in [REMOVE_REFERENCE, 0x00de_e6f0, COPY_ATOMS_BASE] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        e.register_double(TYPE_SLOT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x30),
            ..Ret::default()
        });
        e.register_double(PATH_SLOT, |_, _| Ret {
            eax: PATH_ANSWER,
            ..Ret::default()
        });
        e.register_double(MOTOR_CLONE_SLOT, |_, _| Ret {
            eax: MOTOR_CLONE,
            ..Ret::default()
        });
        e.register_double(0x00cd_f6d0, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x38),
            ..Ret::default()
        });
        log
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A data object of the given type with the virtual functions the tests double.
    fn make_data(e: &mut Engine, kind: u32) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x20, TYPE_SLOT);
        e.mem.set_u32(vtable + 0x34, PATH_SLOT);
        e.mem.set_u32(vtable + 0xc, MOTOR_CLONE_SLOT);
        let data = e.mem.alloc(0x200);
        e.mem.set_u32(data, vtable);
        e.mem.set_u32(data + 0x30, kind);
        for offset in (0x40..0x1f0).step_by(4) {
            e.mem.set_u32(data + offset, 0x1000_0000 + offset);
        }
        for offset in (4..0x30).step_by(4) {
            e.mem.set_u32(data + offset, 0x2000_0000 + offset);
        }
        data
    }

    /// A motor object (answers the clone from its virtual function at +0xc).
    fn make_motor(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0xc, MOTOR_CLONE_SLOT);
        let motor = e.mem.alloc(8);
        e.mem.set_u32(motor, vtable);
        motor
    }

    #[test]
    fn the_constructor_sets_the_identity_blocks() {
        let mut e = engine();
        let object = e.mem.alloc(0xb0);
        for offset in (0..0xb0).step_by(4) {
            e.mem.set_u32(object + offset, 0xcccc_cccc);
        }
        let got = e.call(0x00d8_4bd0, &args![object]).u32();
        assert_eq!(got, object);
        assert_eq!(e.mem.u32(object), 0x010d_37e4);
        assert_eq!(e.mem.u16(object + 6), 1);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u16(object + 0x10), 2);
        assert_eq!(e.mem.u16(object + 0xa0), 7);
        assert_eq!(e.mem.u8(object + 0xa2), 0);
        for offset in (0x20..0xa0).step_by(4) {
            let expected = if [0x20, 0x34, 0x48, 0x60, 0x74, 0x88].contains(&offset) {
                1.0
            } else {
                0.0
            };
            assert_eq!(e.mem.f32(object + offset), expected, "offset {offset:#x}");
        }
        // Bytes the constructor does not set are left alone.
        assert_eq!(e.mem.u16(object + 4), 0xcccc);
        assert_eq!(e.mem.u32(object + 0xc), 0xcccc_cccc);
        assert_eq!(e.mem.u8(object + 0xa3), 0xcc);
    }

    #[test]
    fn each_copy_function_builds_its_object_and_copies_its_range() {
        // (function, size, constructor, offset, length)
        let cases = [
            (0x00d8_5360, 0x50, 0x00d5_4290, 0x10, 0x38),
            (0x00d8_53a0, 0xb0, 0x00cb_e7b0, 0x10, 0x9c),
            (0x00d8_53f0, 0xe0, 0x00cb_9090, 0x10, 0xcc),
            (0x00d8_5440, 0xb0, 0x00de_e750, 0xc, 0xc),
            (0x00d8_5490, 0xd0, 0x00d4_fe80, 0x10, 0xc0),
            (0x00d8_54e0, 0x140, 0x00cd_f2f0, 0x10, 0x130),
            (0x00d8_5530, 0x50, 0x00d4_f0f0, 0x10, 0x38),
            (0x00d8_5570, 0x160, 0x00d4_edc0, 0x10, 0x124),
            (0x00d8_5610, 0x80, 0x00de_efd0, 0x10, 0x70),
            (0x00d8_5660, 0xa0, 0x00d5_2fc0, 0x10, 0x84),
            (0x00d8_56b0, 0xc0, 0x00d4_f2a0, 0x10, 0xac),
        ];
        for (function, size, constructor, offset, length) in cases {
            let mut e = engine();
            let log = install(&mut e);
            let source = make_data(&mut e, 0);
            let object = e.call(function, &args![source]).u32();
            assert_eq!(calls_to(&log, ALLOCATE_SLOT).len(), 1, "{function:08x}");
            assert_eq!(calls_to(&log, ALLOCATE_SLOT)[0][1], size);
            assert_eq!(e.mem.u32(object), constructor, "{function:08x}");
            assert_eq!(calls_to(&log, constructor), [[object]]);
            assert_eq!(e.mem.u16(object + 4), size as u16);
            assert_eq!(
                calls_to(&log, MEM_COPY),
                [[object + offset, source + offset, length]]
            );
            for at in offset..offset + length {
                assert_eq!(e.mem.u8(object + at), e.mem.u8(source + at));
            }
            // The byte before the range is untouched.
            assert_eq!(e.mem.u8(object + offset - 1), 0xcc);
        }
    }

    #[test]
    fn the_unnamed_copy_runs_this_units_constructor() {
        let mut e = engine();
        let log = install(&mut e);
        let source = make_data(&mut e, 0);
        let object = e.call(0x00d8_55c0, &args![source]).u32();
        assert_eq!(calls_to(&log, ALLOCATE_SLOT)[0][1], 0xb0);
        assert_eq!(e.mem.u16(object + 4), 0xb0);
        assert_eq!(
            calls_to(&log, MEM_COPY),
            [[object + 0x10, source + 0x10, 0x94]]
        );
        // The constructor ran for real: its vtable, then the copy over 0x10..0xa4.
        assert_eq!(e.mem.u32(object), 0x010d_37e4);
        assert_eq!(e.mem.u32(object + 0xa0), e.mem.u32(source + 0xa0));
        assert_eq!(e.mem.u32(object + 0x20), e.mem.u32(source + 0x20));
        assert_eq!(e.mem.u8(object + 0xa5), 0xcc);
    }

    #[test]
    fn cloning_by_type_picks_the_matching_copy() {
        // (type, allocated size) for the types that copy directly.
        let sizes = [
            (0, 0x50),
            (1, 0xb0),
            (2, 0xe0),
            (3, 0xb0),
            (6, 0xd0),
            (7, 0x140),
            (8, 0x50),
            (9, 0x160),
            (14, 0xb0),
            (15, 0x80),
            (18, 0xa0),
            (19, 0xc0),
        ];
        for (kind, size) in sizes {
            let mut e = engine();
            let log = install(&mut e);
            let data = make_data(&mut e, kind);
            let path = make_data(&mut e, 0);
            e.mem.set_u32(data + 0x18, path);
            let got = e.call(0x00d8_5700, &args![data]).u32();
            assert_ne!(got, 0, "type {kind}");
            let allocations = calls_to(&log, ALLOCATE_SLOT);
            assert_eq!(allocations.len(), 1, "type {kind}");
            assert_eq!(allocations[0][1], size, "type {kind}");
            assert_eq!(e.mem.u16(got + 4), size as u16);
        }
        for kind in [4, 5, 10, 11, 16, 17, 20, 0x40, 0xffff_ffff] {
            let mut e = engine();
            let log = install(&mut e);
            let data = make_data(&mut e, kind);
            assert_eq!(e.call(0x00d8_5700, &args![data]).u32(), 0, "type {kind}");
            assert!(log.borrow().is_empty());
        }
    }

    #[test]
    fn a_point_to_path_copy_takes_the_original_path_and_releases_the_answer() {
        let mut e = engine();
        let log = install(&mut e);
        let data = make_data(&mut e, 3);
        let path = make_data(&mut e, 0);
        e.mem.set_u32(data + 0x18, path);
        let copy = e.call(0x00d8_5700, &args![data]).u32();
        assert_eq!(calls_to(&log, 0x00de_e6f0), [[copy, path]]);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[PATH_ANSWER]]);
        assert_eq!(calls_to(&log, MEM_COPY), [[copy + 0xc, data + 0xc, 0xc]]);
    }

    #[test]
    fn a_breakable_wraps_a_clone_of_its_inner_data() {
        let mut e = engine();
        let log = install(&mut e);
        let data = make_data(&mut e, 12);
        let inner = make_data(&mut e, 1);
        e.mem.set_u32(data + 0x18, inner);
        // A signalling NaN is quieted by the x87 copy.
        e.mem.set_u32(data + 0x20, 0x7f80_0001);
        e.mem.set_u8(data + 0x24, 0x11);
        e.mem.set_u8(data + 0x25, 0x22);
        let got = e.call(0x00d8_5700, &args![data]).u32();
        let sizes: Vec<u32> = calls_to(&log, ALLOCATE_SLOT).iter().map(|a| a[1]).collect();
        // The inner hinge, then the 0x28-byte wrapper.
        assert_eq!(sizes, [0xb0, 0x28]);
        let wrapper_ctor = calls_to(&log, 0x00d4_df30);
        assert_eq!(wrapper_ctor.len(), 1);
        assert_eq!(wrapper_ctor[0][0], got);
        let child = wrapper_ctor[0][1];
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[child]]);
        assert_eq!(e.mem.u16(got + 4), 0x28);
        assert_eq!(e.mem.u32(got + 0x20), 0x7fc0_0001);
        assert_eq!(e.mem.u8(got + 0x24), 0x11);
        assert_eq!(e.mem.u8(got + 0x25), 0x22);
        assert_eq!(e.mem.u32(got + 8), e.mem.u32(data + 8));
    }

    #[test]
    fn a_wrapper_without_a_clone_answers_null() {
        let mut e = engine();
        let log = install(&mut e);
        let data = make_data(&mut e, 12);
        let inner = make_data(&mut e, 4);
        e.mem.set_u32(data + 0x18, inner);
        assert_eq!(e.call(0x00d8_5700, &args![data]).u32(), 0);
        assert!(calls_to(&log, 0x00d4_df30).is_empty());
        let data = make_data(&mut e, 13);
        let inner = make_data(&mut e, 5);
        e.mem.set_u32(data + 0x38, inner);
        assert_eq!(e.call(0x00d8_5700, &args![data]).u32(), 0);
        assert!(calls_to(&log, 0x00cd_f710).is_empty());
    }

    #[test]
    fn the_other_wrapper_takes_its_inner_data_from_the_getter() {
        let mut e = engine();
        let log = install(&mut e);
        let data = make_data(&mut e, 13);
        let inner = make_data(&mut e, 8);
        e.mem.set_u32(data + 0x38, inner);
        e.mem.set_u32(data + 0x1c, 0x3fc0_0000);
        let got = e.call(0x00d8_5700, &args![data]).u32();
        let sizes: Vec<u32> = calls_to(&log, ALLOCATE_SLOT).iter().map(|a| a[1]).collect();
        assert_eq!(sizes, [0x50, 0x20]);
        let wrapper_ctor = calls_to(&log, 0x00cd_f710);
        assert_eq!(wrapper_ctor.len(), 1);
        assert_eq!(wrapper_ctor[0][0], got);
        assert_eq!(calls_to(&log, REMOVE_REFERENCE), [[wrapper_ctor[0][1]]]);
        assert_eq!(e.mem.u16(got + 4), 0x20);
        assert_eq!(e.mem.u32(got + 0x1c), 0x3fc0_0000);
        assert_eq!(e.mem.u32(got + 8), e.mem.u32(data + 8));
    }

    #[test]
    fn motor_cloning_covers_the_three_motor_types() {
        // (type, size, constructor, motor offsets in the original)
        let cases: [(u32, u32, u32, &[u32]); 3] = [
            (2, 0xe0, 0x00cb_9090, &[0xb0][..]),
            (6, 0xd0, 0x00d4_fe80, &[0xac][..]),
            (7, 0x140, 0x00cd_f2f0, &[0xe0, 0xe4, 0xe8][..]),
        ];
        for (kind, size, constructor, motors) in cases {
            let mut e = engine();
            let log = install(&mut e);
            let data = make_data(&mut e, kind);
            let motor = make_motor(&mut e);
            // Every motor but the last is set; the last is null.
            for (index, offset) in motors.iter().enumerate() {
                let value = if index + 1 == motors.len() && motors.len() > 1 {
                    0
                } else {
                    motor
                };
                e.mem.set_u32(data + offset, value);
            }
            let got = e.call(0x00d8_58f0, &args![data]).u32();
            assert_eq!(calls_to(&log, ALLOCATE_SLOT)[0][1], size);
            assert_eq!(calls_to(&log, constructor), [[got]]);
            assert_eq!(e.mem.u32(got + 8), e.mem.u32(data + 8));
            assert_eq!(calls_to(&log, COPY_ATOMS_BASE), [[got + 0x10, data + 0x10]]);
            for (index, offset) in motors.iter().enumerate() {
                let expected = if index + 1 == motors.len() && motors.len() > 1 {
                    0
                } else {
                    MOTOR_CLONE
                };
                assert_eq!(
                    e.mem.u32(got + offset),
                    expected,
                    "type {kind} motor {index}"
                );
            }
        }
        let mut e = engine();
        let log = install(&mut e);
        let data = make_data(&mut e, 5);
        assert_eq!(e.call(0x00d8_58f0, &args![data]).u32(), 0);
        assert!(log.borrow().is_empty());
    }

    /// Fills `len` bytes at `at` with a pattern.
    fn fill_pattern(e: &mut Engine, at: u32, len: u32) {
        for offset in 0..len {
            e.mem.set_u8(at + offset, (offset * 7 + 3) as u8);
        }
    }

    /// The bytes of `[to, to + len)` equal those of `from` at the same offsets.
    fn copied(e: &Engine, from: u32, to: u32, range: std::ops::Range<u32>) -> bool {
        range
            .into_iter()
            .all(|at| e.mem.u8(from + at) == e.mem.u8(to + at))
    }

    #[test]
    #[allow(clippy::single_range_in_vec_init)]
    fn the_atom_assignments_copy_their_own_tails() {
        for (function, copied_ranges, untouched) in [
            (0x00d8_5a20, vec![0x90..0xcc], vec![0x8f, 0xcc, 0xcd]),
            (0x00d8_5af0, vec![0x90..0xc0], vec![0x8f, 0xc0, 0xc1]),
            (
                0x00d8_5ba0,
                vec![0x90..0x93, 0x94..0x98, 0xa0..0xdc, 0xe0..0x130],
                vec![0x93, 0x98, 0x9f, 0xdc, 0xdf, 0x130],
            ),
        ] {
            let mut e = engine();
            let log = install(&mut e);
            let source = e.mem.alloc(0x140);
            let target = e.mem.alloc(0x140);
            fill_pattern(&mut e, source, 0x140);
            for offset in 0..0x140 {
                e.mem.set_u8(target + offset, 0x99);
            }
            let got = e.call(function, &args![target, source]).u32();
            assert_eq!(got, target);
            assert_eq!(calls_to(&log, COPY_ATOMS_BASE), [[target, source]]);
            for range in copied_ranges {
                assert!(
                    copied(&e, source, target, range.clone()),
                    "{function:08x} {range:?}"
                );
            }
            for offset in untouched {
                assert_eq!(
                    e.mem.u8(target + offset),
                    0x99,
                    "{function:08x} +{offset:#x}"
                );
            }
        }
    }
}
