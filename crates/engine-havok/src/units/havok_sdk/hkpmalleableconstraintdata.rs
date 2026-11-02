//! `Havok SDK/hkpmalleableconstraintdata.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMalleableConstraintData` (Xbox PDB, vtable `010c82a4`) wraps another constraint data:
//! the wrapped object is the pointer at +0x0C, a strength factor (float, `0.01` from the
//! constructor) is at +0x1C, and an embedded object of its own is at +0x10 (the member names
//! are the Havok SDK's; the exe has none).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable `010c82a4` of the class.
const VTABLE: u32 = 0x010c_82a4;
/// The vtable `0102e388` the destructor body leaves the object with.
const BASE_VTABLE: u32 = 0x0102_e388;
/// `float` `0.01` (`01013ea4`), the strength the constructor starts with.
const DEFAULT_STRENGTH: u32 = 0x0101_3ea4;
/// `00c90510` (thiscall, no arguments): takes a reference on the wrapped object.
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `00c905b0` (thiscall, no arguments): drops the reference on the wrapped object.
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `00d21ce0` (thiscall, `RET 4`): stores its argument at +8 of the embedded object at +0x10
/// and a function address at +4 (the engine map has no name).
const SET_OWNER: u32 = 0x00d2_1ce0;
/// `00d72190` (cdecl, `(atoms, parameters, flag byte, word)`): the jacobian builder for the
/// atom list that starts with type `0x16`.
const BUILD_JACOBIAN_ATOM_0X16: u32 = 0x00d7_2190;
/// `00d6f460` (cdecl, `(atoms, size, parameters, word)`): the generic jacobian builder.
const BUILD_JACOBIAN_ATOMS: u32 = 0x00d6_f460;

// Translated from 00cdf620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMalleableConstraintData::getConstraintInfo` (Xbox PDB), `RET 4`: lets the wrapped
/// object fill `info` (its vtable slot `0x3C`), then overwrites the atom pointer at `info+0x10`
/// with this object's embedded part (`this + 0x10`) and the size at `info+0x14` with 0xC.
pub fn hkp_malleable_constraint_data_get_constraint_info(e: &mut Engine, this: Ptr, info: Ptr) {
    let wrapped = e.mem.u32(this.addr() + 0xc);
    e.vcall(wrapped, 0x3c, &args![info]);
    e.mem.set_u32(info.addr() + 0x10, this.addr() + 0x10);
    e.mem.set_u32(info.addr() + 0x14, 0xc);
}

// Translated from 00cdf650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMalleableConstraintData::getRuntimeInfo` (Xbox PDB), `RET 8`: forwards both stack words,
/// in the same order, to slot `0x24` of the wrapped object.
pub fn hkp_malleable_constraint_data_get_runtime_info(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
) {
    let wrapped = e.mem.u32(this.addr() + 0xc);
    e.vcall(wrapped, 0x24, &args![first, second]);
}

// Translated from 00cdf690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMalleableConstraintData::isValid` (Xbox PDB), `RET 4`: calls slot `0x1C` of the wrapped
/// object with the stack word and returns that word itself.
pub fn hkp_malleable_constraint_data_is_valid(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    let wrapped = e.mem.u32(this.addr() + 0xc);
    e.vcall(wrapped, 0x1c, &args![word]);
    word
}

// Translated from 00cdf6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMalleableConstraintData::getType` (Xbox PDB): the constant 0xD.
pub fn hkp_malleable_constraint_data_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0xd
}

// Translated from 00cdf6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMalleableConstraintData::getWrappedConstraintData` (Xbox PDB): the wrapped object at
/// +0x0C.
pub fn hkp_malleable_constraint_data_get_wrapped_constraint_data(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xc))
}

// Translated from 00cdf6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A second accessor with the same body as `getWrappedConstraintData` (the engine map has no
/// name): the wrapped object at +0x0C.
pub fn fn_00cdf6d0(e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(this.addr() + 0xc))
}

// Translated from 00cdf710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor (no name in the engine map), `RET 4`: stores `wrapped` at +0xC, 1 as the
/// 16-bit words at +6 and +0x10, 0 at +8, the vtable `010c82a4`, the strength `0.01` at +0x1C
/// (through `FLD`/`FSTP`), takes a reference on the wrapped object when it is not null
/// (`00c90510`) and sets the owner of the embedded part at +0x10 to this object (`00d21ce0`).
/// Returns `this`.
pub fn fn_00cdf710(e: &mut Engine, this: Ptr, wrapped: Ptr) -> Ptr {
    let me = this.addr();
    let strength = x87_quiet(e.global::<f32>(DEFAULT_STRENGTH));
    e.mem.set_u32(me + 0xc, wrapped.addr());
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me, VTABLE);
    e.mem.set_u16(me + 0x10, 1);
    e.mem.set_f32(me + 0x1c, strength);
    let wrapped = e.mem.u32(me + 0xc);
    if wrapped != 0 {
        e.call(ADD_REFERENCE, &args![wrapped]);
    }
    e.call(SET_OWNER, &args![me + 0x10, this]);
    this
}

// Translated from 00cdf760 (decompiled, FalloutNV.exe 1.4.0.525)
/// A destructor-like body (no name in the engine map), `RET 4`: sets the vtable `010c82a4` and,
/// when `flag` is not 0, runs `00d21ce0` on the embedded part at +0x10 twice: with the word at
/// +0x18, then with this object. Returns `this`.
pub fn fn_00cdf760(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE);
    if flag != 0 {
        let word = e.mem.u32(me + 0x18);
        e.call(SET_OWNER, &args![me + 0x10, word]);
        e.call(SET_OWNER, &args![me + 0x10, this]);
    }
    this
}

// Translated from 00cdf7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body (no name in the engine map): sets the vtable `010c82a4`, drops the
/// reference on the wrapped object (`00c905b0`; the exe does not test it for null) and leaves
/// the object with the base vtable `0102e388`.
pub fn fn_00cdf7a0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let wrapped = e.mem.u32(me + 0xc);
    e.mem.set_u32(me, VTABLE);
    e.call(REMOVE_REFERENCE, &args![wrapped]);
    e.mem.set_u32(me, BASE_VTABLE);
}

// Translated from 00cdf7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMalleableConstraintData::buildJacobian` (Xbox PDB), `RET 8`: copies the 0x80-byte
/// parameter block `parameters` to the stack and multiplies its float at +0x24 by the
/// strength at +0x1C (x87, rounded to float), has the wrapped object fill a 0x18-byte info
/// record (slot `0x3C`) and, when the first atom it names (the pointer at +0x10) has type
/// `0x16`, runs `00d72190(atoms, copy, 1, word)`, otherwise `00d6f460(atoms, the size at +0x14,
/// copy, word)`. The exe passes the flag `1` as a dword whose upper three bytes are stack
/// garbage; the callee reads only the low byte.
pub fn hkp_malleable_constraint_data_build_jacobian(
    e: &mut Engine,
    this: Ptr,
    parameters: Ptr,
    word: u32,
) {
    let strength = x87_quiet(e.mem.f32(this.addr() + 0x1c));
    let wrapped = e.mem.u32(this.addr() + 0xc);
    e.with_stack(0xa8, |e, frame| {
        let info = frame.addr() + 0x10;
        let copy = frame.addr() + 0x28;
        for offset in 0..0x20 {
            let source = e.mem.u32(parameters.addr() + 4 * offset);
            e.mem.set_u32(copy + 4 * offset, source);
        }
        let scaled = (strength as f64) * (x87_quiet(e.mem.f32(copy + 0x24)) as f64);
        e.mem.set_f32(copy + 0x24, scaled as f32);
        e.vcall(wrapped, 0x3c, &args![info]);
        let atoms = e.mem.u32(info + 0x10);
        if e.mem.u16(atoms) == 0x16 {
            e.call(BUILD_JACOBIAN_ATOM_0X16, &args![atoms, copy, 1u32, word]);
        } else {
            let size = e.mem.u32(info + 0x14);
            e.call(BUILD_JACOBIAN_ATOMS, &args![atoms, size, copy, word]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cdf620,
            hkp_malleable_constraint_data_get_constraint_info(Ptr, Ptr)
        ),
        entry!(
            0x00cdf650,
            hkp_malleable_constraint_data_get_runtime_info(Ptr, u32, u32)
        ),
        entry!(
            0x00cdf690,
            hkp_malleable_constraint_data_is_valid(Ptr, u32) -> u32
        ),
        entry!(
            0x00cdf6b0,
            hkp_malleable_constraint_data_get_type(Ptr) -> u32
        ),
        entry!(
            0x00cdf6c0,
            hkp_malleable_constraint_data_get_wrapped_constraint_data(Ptr) -> Ptr
        ),
        entry!(0x00cdf6d0, fn_00cdf6d0(Ptr) -> Ptr),
        entry!(0x00cdf710, fn_00cdf710(Ptr, Ptr) -> Ptr),
        entry!(0x00cdf760, fn_00cdf760(Ptr, u32) -> Ptr),
        entry!(0x00cdf7a0, fn_00cdf7a0(Ptr)),
        entry!(
            0x00cdf7c0,
            hkp_malleable_constraint_data_build_jacobian(Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const SLOT_DOUBLE: u32 = 0x0900_0011;

    fn log_calls(e: &mut Engine, addr: u32, log: &Log, tag: u32) {
        let sink = Rc::clone(log);
        e.register_double(addr, move |_, a| {
            let mut row = vec![tag];
            row.extend_from_slice(a);
            sink.borrow_mut().push(row);
            Ret::default()
        });
    }

    /// An object whose vtable has `SLOT_DOUBLE` at `slot`.
    fn wrapped_with_slot(e: &mut Engine, slot: u32) -> u32 {
        let vtable = e.mem.alloc(0x80);
        e.mem.set_u32(vtable + slot, SLOT_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        object
    }

    fn malleable(e: &mut Engine, wrapped: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, wrapped);
        this
    }

    #[test]
    fn constraint_info_is_filled_by_the_wrapped_object_then_overwritten() {
        let mut e = Engine::new();
        e.register_double(SLOT_DOUBLE, |e, a| {
            e.mem.set_u32(a[1] + 0x10, 0x5555);
            e.mem.set_u32(a[1] + 0x14, 0x6666);
            e.mem.set_u32(a[1], 0x7777);
            Ret::default()
        });
        let wrapped = wrapped_with_slot(&mut e, 0x3c);
        let this = malleable(&mut e, wrapped);
        let info = e.mem.alloc(0x20);
        e.call(0x00cdf620, &args![this, info]);
        assert_eq!(e.mem.u32(info + 0x10), this + 0x10);
        assert_eq!(e.mem.u32(info + 0x14), 0xc);
        assert_eq!(e.mem.u32(info), 0x7777);
    }

    #[test]
    fn runtime_info_and_validity_go_to_the_wrapped_object() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        log_calls(&mut e, SLOT_DOUBLE, &log, 0);
        let wrapped = wrapped_with_slot(&mut e, 0x24);
        let vtable = e.mem.u32(wrapped);
        e.mem.set_u32(vtable + 0x1c, SLOT_DOUBLE);
        let this = malleable(&mut e, wrapped);
        e.call(0x00cdf650, &args![this, 7u32, 8u32]);
        let got = e.call(0x00cdf690, &args![this, 0x1234u32]).u32();
        assert_eq!(got, 0x1234);
        assert_eq!(
            *log.borrow(),
            vec![vec![0, wrapped, 7, 8], vec![0, wrapped, 0x1234]]
        );
    }

    #[test]
    fn the_small_accessors() {
        let mut e = Engine::new();
        let this = malleable(&mut e, 0x4321);
        assert_eq!(e.call(0x00cdf6b0, &args![this]).u32(), 0xd);
        assert_eq!(e.call(0x00cdf6c0, &args![this]).u32(), 0x4321);
        assert_eq!(e.call(0x00cdf6d0, &args![this]).u32(), 0x4321);
    }

    #[test]
    fn the_constructor_wraps_references_and_sets_the_owner() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(DEFAULT_STRENGTH, 0.01);
        let log: Log = Rc::default();
        log_calls(&mut e, ADD_REFERENCE, &log, 1);
        log_calls(&mut e, SET_OWNER, &log, 2);
        let this = e.mem.alloc(0x40);
        let result = e.call(0x00cdf710, &args![this, 0x4321u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this + 0xc), 0x4321);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u16(this + 0x10), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.f32(this + 0x1c), 0.01);
        assert_eq!(
            *log.borrow(),
            vec![vec![1, 0x4321], vec![2, this + 0x10, this]]
        );
        // A null wrapped object takes no reference.
        log.borrow_mut().clear();
        e.call(0x00cdf710, &args![this, 0u32]);
        assert_eq!(*log.borrow(), vec![vec![2, this + 0x10, this]]);
    }

    #[test]
    fn the_destructor_bodies() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        log_calls(&mut e, SET_OWNER, &log, 2);
        log_calls(&mut e, REMOVE_REFERENCE, &log, 3);
        let this = malleable(&mut e, 0x4321);
        e.mem.set_u32(this + 0x18, 0x99);
        assert_eq!(e.call(0x00cdf760, &args![this, 0u32]).u32(), this);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.call(0x00cdf760, &args![this, 1u32]).u32(), this);
        assert_eq!(
            *log.borrow(),
            vec![vec![2, this + 0x10, 0x99], vec![2, this + 0x10, this]]
        );
        log.borrow_mut().clear();
        e.call(0x00cdf7a0, &args![this]);
        assert_eq!(*log.borrow(), vec![vec![3, 0x4321]]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
    }

    #[test]
    fn build_jacobian_scales_the_copy_and_picks_the_builder_by_atom_type() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        for (addr, tag) in [(BUILD_JACOBIAN_ATOM_0X16, 1u32), (BUILD_JACOBIAN_ATOMS, 2)] {
            let sink = Rc::clone(&log);
            e.register_double(addr, move |e, a| {
                let copy = if tag == 1 { a[1] } else { a[2] };
                let mut row = vec![tag];
                row.extend_from_slice(a);
                row.push(e.mem.u32(copy));
                row.push(e.mem.u32(copy + 0x24));
                row.push(e.mem.u32(copy + 0x7c));
                sink.borrow_mut().push(row);
                Ret::default()
            });
        }
        let atoms = e.mem.alloc(0x20);
        e.register_double(SLOT_DOUBLE, move |e, a| {
            e.mem.set_u32(a[1] + 0x10, atoms);
            e.mem.set_u32(a[1] + 0x14, 0x30);
            Ret::default()
        });
        let wrapped = wrapped_with_slot(&mut e, 0x3c);
        let this = malleable(&mut e, wrapped);
        e.mem.set_f32(this + 0x1c, 0.5);
        let parameters = e.mem.alloc(0x80);
        e.mem.set_u32(parameters, 0xaaaa);
        e.mem.set_f32(parameters + 0x24, 8.0);
        e.mem.set_u32(parameters + 0x7c, 0xbbbb);
        e.mem.set_u16(atoms, 0x16);
        e.call(0x00cdf7c0, &args![this, parameters, 0x55u32]);
        e.mem.set_u16(atoms, 0x17);
        e.call(0x00cdf7c0, &args![this, parameters, 0x66u32]);
        let rows = log.borrow();
        let four = 4.0f32.to_bits();
        // The first builder gets (atoms, copy, 1, word); the copy is not the original.
        assert_eq!(rows[0][1], atoms);
        assert_ne!(rows[0][2], parameters);
        assert_eq!(rows[0][3..5], [1, 0x55]);
        assert_eq!(rows[0][5..], [0xaaaa, four, 0xbbbb]);
        // The second gets (atoms, size, copy, word).
        assert_eq!(rows[1][1..3], [atoms, 0x30]);
        assert_eq!(rows[1][4], 0x66);
        assert_eq!(rows[1][5..], [0xaaaa, four, 0xbbbb]);
        // The caller's block is untouched.
        assert_eq!(e.mem.f32(parameters + 0x24), 8.0);
    }
}
