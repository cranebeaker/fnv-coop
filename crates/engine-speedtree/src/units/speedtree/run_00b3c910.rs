//! `SpeedTree/run_00b3c910` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The small classes of the tree code, identified from the code (the engine map gives them no
//! names): the extents box (twelve words: a minimum point at `+0` and a maximum point at `+0x18`,
//! each a `CVec`; its constructor, reset, merge with another box and growth by one point), a
//! 4x4 identity matrix and its constructor, a vtable-only object, and one constructor and
//! destructor of a class that keeps a `std::vector` at `+0x10` and three arrays.
//!
//! x87 note: a float copied through the x87 stack (`FLD dword`, `FSTP dword`) goes through
//! [`x87_quiet_bits`]; the box comparisons are the exe's `FCOMPP` flag tests (an unordered
//! compare never updates).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::OPERATOR_DELETE;

/// The vtable `00b3c910` stores (`010a31fc`, the "scalar deleting destructor" table of the
/// object's class).
const SCALAR_DELETING_VTABLE: u32 = 0x010a_31fc;
/// The `CVec` default constructor (`00b3c470`, `fastcall`).
const CVEC_CONSTRUCT: u32 = 0x00b3_c470;
/// The empty destructor of the plain-data members (`00aa7830`, `thiscall`, `this` only).
const TRIVIAL_DESTRUCTOR: u32 = 0x00aa_7830;
/// The float constants the box reset reads: the initial minimum (`01016970`) and the initial
/// maximum (`01015f5c`).
const EXTENTS_INITIAL_MINIMUM: u32 = 0x0101_6970;
const EXTENTS_INITIAL_MAXIMUM: u32 = 0x0101_5f5c;
/// The float constants the constructor `00b3cd60` copies to `+0x28`, `+0x2C` and `+0x30`.
const DEFAULT_FIELD_28: u32 = 0x0101_6248;
const DEFAULT_FIELD_2C: u32 = 0x0101_6264;
const DEFAULT_FIELD_30: u32 = 0x0103_0ff0;
/// The vector constructor/destructor of the member at `+0x10` of the class `00b3cd60` builds
/// (`00b3d830`, `00b3db50`, `fastcall`).
const MEMBER_VECTOR_CONSTRUCT: u32 = 0x00b3_d830;
const MEMBER_VECTOR_DESTRUCT: u32 = 0x00b3_db50;

/// Copies the twelve words of an extents box from `source` to `dest` (`REP MOVSD`).
fn copy_extents(e: &mut Engine, dest: u32, source: u32) {
    for word in 0..12u32 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

/// `FLD dword [from]; FSTP dword [to]`.
fn quiet_copy(e: &mut Engine, to: u32, from: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

// Translated from 00b3c910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `010a31fc` vtable at the start of the object (identified from the code;
/// `fastcall`; the class is unconfirmed).
pub fn fn_00b3c910(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SCALAR_DELETING_VTABLE);
}

// Translated from 00b3c930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3c930 - the FS:[0] exception frame is not translated.
/// The extents box constructor (identified from the code; `fastcall`): constructs the minimum
/// point (`+0`) and the maximum point (`+0x18`) as `CVec`s (`00b3c470`) and resets the box
/// (`00b3cc30`). Returns `this`.
pub fn fn_00b3c930(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CVEC_CONSTRUCT, &args![this]);
    e.call(CVEC_CONSTRUCT, &args![this.addr() + 0x18]);
    fn_00b3cc30(e, this);
    this
}

// Translated from 00b3c9a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3c9a0 - the FS:[0] exception frame is not translated.
/// The union of two extents boxes (identified from the code; `thiscall`: a result box and the
/// other box): copies `this` to a local, lowers its minimum (`+0`, `+4`, `+8`) to the other's
/// where the other's is smaller and raises its maximum (`+0x18`, `+0x1C`, `+0x20`) where the
/// other's is larger (ordered compares; a NaN never updates; the values copied through the
/// x87 stack), copies the local over the result box and runs the empty destructors of its two
/// points. Returns the result box.
pub fn fn_00b3c9a0(e: &mut Engine, this: Ptr, out: Ptr, other: Ptr) -> Ptr {
    e.with_stack(0x30, |e, local| {
        copy_extents(e, local.addr(), this.addr());
        for axis in 0..3u32 {
            let other_minimum = x87_load32(e.mem.u32(other.addr() + 4 * axis));
            let own_minimum = x87_load32(e.mem.u32(this.addr() + 4 * axis));
            if own_minimum > other_minimum {
                quiet_copy(e, local.addr() + 4 * axis, other.addr() + 4 * axis);
            }
            let other_maximum = x87_load32(e.mem.u32(other.addr() + 0x18 + 4 * axis));
            let own_maximum = x87_load32(e.mem.u32(this.addr() + 0x18 + 4 * axis));
            if own_maximum < other_maximum {
                quiet_copy(
                    e,
                    local.addr() + 0x18 + 4 * axis,
                    other.addr() + 0x18 + 4 * axis,
                );
            }
        }
        copy_extents(e, out.addr(), local.addr());
        e.call(TRIVIAL_DESTRUCTOR, &args![local.addr() + 0x18]);
        e.call(TRIVIAL_DESTRUCTOR, &args![local]);
    });
    out
}

// Translated from 00b3caa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3caa0 - the FS:[0] exception frame is not translated.
/// The growth of an extents box by one point (identified from the code; `thiscall`: a result
/// box and the point's three floats): the union of `00b3c9a0` with a box that is just the
/// point, in the exe's order (the three minimum compares, then the three maximum compares); the
/// result is copied over the result box and the empty destructors of the local's two points
/// run. Returns the result box.
pub fn fn_00b3caa0(e: &mut Engine, this: Ptr, out: Ptr, point: Ptr) -> Ptr {
    e.with_stack(0x30, |e, local| {
        copy_extents(e, local.addr(), this.addr());
        for axis in 0..3u32 {
            let value = x87_load32(e.mem.u32(point.addr() + 4 * axis));
            let minimum = x87_load32(e.mem.u32(local.addr() + 4 * axis));
            if minimum > value {
                quiet_copy(e, local.addr() + 4 * axis, point.addr() + 4 * axis);
            }
        }
        for axis in 0..3u32 {
            let value = x87_load32(e.mem.u32(point.addr() + 4 * axis));
            let maximum = x87_load32(e.mem.u32(local.addr() + 0x18 + 4 * axis));
            if maximum < value {
                quiet_copy(e, local.addr() + 0x18 + 4 * axis, point.addr() + 4 * axis);
            }
        }
        copy_extents(e, out.addr(), local.addr());
        e.call(TRIVIAL_DESTRUCTOR, &args![local.addr() + 0x18]);
        e.call(TRIVIAL_DESTRUCTOR, &args![local]);
    });
    out
}

// Translated from 00b3cc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets an extents box (identified from the code; `fastcall`): the minimum's three floats
/// (`+0`, `+4`, `+8`) get the float at `01016970` and the maximum's (`+0x18`, `+0x1C`, `+0x20`)
/// the float at `01015f5c`, each copied through the x87 stack (stored `+8`, `+4`, `+0`, then
/// `+0x20`, `+0x1C`, `+0x18`).
pub fn fn_00b3cc30(e: &mut Engine, this: Ptr) {
    for offset in [8u32, 4, 0] {
        quiet_copy(e, this.addr() + offset, EXTENTS_INITIAL_MINIMUM);
    }
    for offset in [8u32, 4, 0] {
        quiet_copy(e, this.addr() + 0x18 + offset, EXTENTS_INITIAL_MAXIMUM);
    }
}

// Translated from 00b3ccb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the 4x4 matrix (identified from the code; `fastcall`): makes it the identity
/// (`00b3ccd0`). Returns `this`.
pub fn fn_00b3ccb0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00b3ccd0(e, this);
    this
}

// Translated from 00b3ccd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the 4x4 matrix at `this` to the identity (identified from the code; `fastcall`): sixteen
/// floats, 1.0 on the diagonal and 0.0 elsewhere.
pub fn fn_00b3ccd0(e: &mut Engine, this: Ptr) {
    for index in 0..16u32 {
        let value: f32 = if index % 5 == 0 { 1.0 } else { 0.0 };
        e.mem.set_f32(this.addr() + 4 * index, value);
    }
}

// Translated from 00b3cd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of a class with a `std::vector` at `+0x10` (identified from the code; class
/// unconfirmed; `fastcall`): sets the byte at `+0` to 1, the float at `+4` to 1.0, `+0xC` to 2,
/// constructs the vector at `+0x10` (`00b3d830`), copies the floats at `01016248`, `01016264`
/// and `01030ff0` (through the x87 stack) to `+0x28`, `+0x2C` and `+0x30`, sets `+0x34` to 1,
/// `+0x40` to 3, `+0x44` to 4 and clears `+0x48`, `+0x4C`, `+0x50` and `+0x54`. Returns `this`.
pub fn fn_00b3cd60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u8(base, 1);
    e.mem.set_f32(base + 4, 1.0);
    e.mem.set_u32(base + 0x0C, 2);
    e.call(MEMBER_VECTOR_CONSTRUCT, &args![base + 0x10]);
    quiet_copy(e, base + 0x28, DEFAULT_FIELD_28);
    quiet_copy(e, base + 0x2C, DEFAULT_FIELD_2C);
    quiet_copy(e, base + 0x30, DEFAULT_FIELD_30);
    e.mem.set_u32(base + 0x34, 1);
    e.mem.set_u32(base + 0x40, 3);
    e.mem.set_u32(base + 0x44, 4);
    for offset in [0x48u32, 0x4C, 0x50, 0x54] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

// Translated from 00b3ce30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3ce30 - the FS:[0] exception frame is not translated.
/// The destructor of the class `00b3cd60` builds (identified from the code; `fastcall`): frees
/// the array at `+0x54` (and clears it), when `+0x4C` is set frees each of its `+0x44` entries
/// and the array itself (and clears it), frees the array at `+0x50` (and clears it), destructs
/// the vector at `+0x10` (`00b3db50`) and frees its first word's block.
pub fn fn_00b3ce30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let block = e.mem.u32(base + 0x54);
    e.call(OPERATOR_DELETE, &args![block]);
    e.mem.set_u32(base + 0x54, 0);
    if e.mem.u32(base + 0x4C) != 0 {
        let mut index: i32 = 0;
        while index < e.mem.i32(base + 0x44) {
            let entry = e.mem.u32(
                e.mem
                    .u32(base + 0x4C)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            e.call(OPERATOR_DELETE, &args![entry]);
            index += 1;
        }
        let array = e.mem.u32(base + 0x4C);
        e.call(OPERATOR_DELETE, &args![array]);
        e.mem.set_u32(base + 0x4C, 0);
    }
    let block = e.mem.u32(base + 0x50);
    e.call(OPERATOR_DELETE, &args![block]);
    e.mem.set_u32(base + 0x50, 0);
    e.call(MEMBER_VECTOR_DESTRUCT, &args![base + 0x10]);
    let proxy = e.mem.u32(base + 0x10);
    e.call(OPERATOR_DELETE, &args![proxy]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b3c910, fn_00b3c910(Ptr)),
        entry!(0x00b3c930, fn_00b3c930(Ptr) -> Ptr),
        entry!(0x00b3c9a0, fn_00b3c9a0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00b3caa0, fn_00b3caa0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00b3cc30, fn_00b3cc30(Ptr)),
        entry!(0x00b3ccb0, fn_00b3ccb0(Ptr) -> Ptr),
        entry!(0x00b3ccd0, fn_00b3ccd0(Ptr)),
        entry!(0x00b3cd60, fn_00b3cd60(Ptr) -> Ptr),
        entry!(0x00b3ce30, fn_00b3ce30(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e
    }

    fn spy(e: &mut Engine, addr: u32) -> Words {
        let log: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log
    }

    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    /// A box: minimum at `+0`, maximum at `+0x18`, the other words marked.
    fn extents(e: &mut Engine, min: [f32; 3], max: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0x30);
        for word in 0..12 {
            e.mem.set_u32(at + 4 * word, 0xAAAA_0000 + word);
        }
        put(e, at, &min);
        put(e, at + 0x18, &max);
        at
    }

    fn corners(e: &Engine, at: u32) -> ([f32; 3], [f32; 3]) {
        let read = |base: u32| [0, 1, 2].map(|i| e.mem.f32(at + base + 4 * i));
        (read(0), read(0x18))
    }

    #[test]
    fn vtable_is_stored() {
        let mut e = engine();
        let object = e.mem.alloc(8);
        e.call(0x00b3_c910, &args![Ptr::<()>::new(object)]);
        assert_eq!(e.mem.u32(object), SCALAR_DELETING_VTABLE);
    }

    #[test]
    fn extents_constructor_builds_the_points_and_resets() {
        let mut e = engine();
        e.mem.set_u32(EXTENTS_INITIAL_MINIMUM, 0x7F7F_FFFF);
        e.mem.set_u32(EXTENTS_INITIAL_MAXIMUM, 0xFF7F_FFFF);
        let constructed = spy(&mut e, CVEC_CONSTRUCT);
        let at = e.mem.alloc(0x30);
        for word in 0..12 {
            e.mem.set_u32(at + 4 * word, 0xAAAA_0000);
        }
        let r = e.call(0x00b3_c930, &args![Ptr::<()>::new(at)]);
        assert_eq!(r.u32(), at);
        assert_eq!(*constructed.borrow(), vec![vec![at], vec![at + 0x18]]);
        let (min, max) = corners(&e, at);
        assert_eq!(min, [f32::MAX; 3]);
        assert_eq!(max, [-f32::MAX; 3]);
        // The words between the points are left alone.
        assert_eq!(e.mem.u32(at + 0x0C), 0xAAAA_0000);
    }

    #[test]
    fn reset_quiets_signalling_constants() {
        let mut e = engine();
        e.mem.set_u32(EXTENTS_INITIAL_MINIMUM, 0x7F80_0001);
        e.mem.set_u32(EXTENTS_INITIAL_MAXIMUM, 0x3F80_0000);
        let at = e.mem.alloc(0x30);
        e.call(0x00b3_cc30, &args![Ptr::<()>::new(at)]);
        assert_eq!(e.mem.u32(at), 0x7FC0_0001);
        assert_eq!(e.mem.u32(at + 8), 0x7FC0_0001);
        assert_eq!(e.mem.f32(at + 0x20), 1.0);
    }

    #[test]
    fn union_takes_the_smaller_minimum_and_larger_maximum() {
        let mut e = engine();
        let destructed = spy(&mut e, TRIVIAL_DESTRUCTOR);
        let a = extents(&mut e, [0.0, 5.0, -3.0], [1.0, 6.0, 2.0]);
        let b = extents(&mut e, [-1.0, 5.0, 0.0], [0.5, 9.0, 2.0]);
        let out = e.mem.alloc(0x30);
        let r = e.call(
            0x00b3_c9a0,
            &args![Ptr::<()>::new(a), Ptr::<()>::new(out), Ptr::<()>::new(b)],
        );
        assert_eq!(r.u32(), out);
        let (min, max) = corners(&e, out);
        assert_eq!(min, [-1.0, 5.0, -3.0]);
        assert_eq!(max, [1.0, 9.0, 2.0]);
        // The words between the points are copied from `this`; `this` is unchanged.
        assert_eq!(e.mem.u32(out + 0x0C), 0xAAAA_0003);
        assert_eq!(corners(&e, a).0, [0.0, 5.0, -3.0]);
        assert_eq!(destructed.borrow().len(), 2);
    }

    #[test]
    fn union_ignores_nan_and_quiets_what_it_copies() {
        let mut e = engine();
        spy(&mut e, TRIVIAL_DESTRUCTOR);
        let a = extents(&mut e, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let b = extents(&mut e, [f32::NAN, -2.0, -1.0], [f32::NAN, 3.0, 4.0]);
        // A signalling NaN is unordered too: no update.
        e.mem.set_u32(b + 8, 0x7F80_0001);
        let out = e.mem.alloc(0x30);
        e.call(
            0x00b3_c9a0,
            &args![Ptr::<()>::new(a), Ptr::<()>::new(out), Ptr::<()>::new(b)],
        );
        let (min, max) = corners(&e, out);
        assert_eq!(min, [0.0, -2.0, 0.0]);
        assert_eq!(max, [1.0, 3.0, 4.0]);
    }

    #[test]
    fn point_growth_updates_in_the_exe_order() {
        let mut e = engine();
        let destructed = spy(&mut e, TRIVIAL_DESTRUCTOR);
        let a = extents(&mut e, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let point = e.mem.alloc(12);
        put(&mut e, point, &[-4.0, 0.5, 7.0]);
        let out = e.mem.alloc(0x30);
        let r = e.call(
            0x00b3_caa0,
            &args![
                Ptr::<()>::new(a),
                Ptr::<()>::new(out),
                Ptr::<()>::new(point)
            ],
        );
        assert_eq!(r.u32(), out);
        let (min, max) = corners(&e, out);
        assert_eq!(min, [-4.0, 0.0, 0.0]);
        assert_eq!(max, [1.0, 1.0, 7.0]);
        assert_eq!(destructed.borrow().len(), 2);
        // A NaN coordinate updates nothing; a signalling one is not copied either.
        put(&mut e, point, &[f32::NAN, 0.5, 0.5]);
        e.mem.set_u32(point + 8, 0x7F80_0001);
        e.call(
            0x00b3_caa0,
            &args![
                Ptr::<()>::new(a),
                Ptr::<()>::new(out),
                Ptr::<()>::new(point)
            ],
        );
        let (min, max) = corners(&e, out);
        assert_eq!(min, [0.0, 0.0, 0.0]);
        assert_eq!(max, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn matrix_constructors_make_the_identity() {
        let mut e = engine();
        let at = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_u32(at + 4 * word, 0xDEAD_BEEF);
        }
        let r = e.call(0x00b3_ccb0, &args![Ptr::<()>::new(at)]);
        assert_eq!(r.u32(), at);
        let values: Vec<f32> = (0..16).map(|i| e.mem.f32(at + 4 * i)).collect();
        for (i, v) in values.iter().enumerate() {
            assert_eq!(*v, if i % 5 == 0 { 1.0 } else { 0.0 }, "word {i}");
        }
        for word in 0..16 {
            e.mem.set_u32(at + 4 * word, 0xDEAD_BEEF);
        }
        e.call(0x00b3_ccd0, &args![Ptr::<()>::new(at)]);
        assert_eq!(e.mem.f32(at + 0x3C), 1.0);
        assert_eq!(e.mem.f32(at + 0x38), 0.0);
    }

    #[test]
    fn class_constructor_sets_the_defaults() {
        let mut e = engine();
        let built = spy(&mut e, MEMBER_VECTOR_CONSTRUCT);
        e.mem.set_f32(DEFAULT_FIELD_28, 0.5);
        e.mem.set_f32(DEFAULT_FIELD_2C, 0.75);
        e.mem.set_u32(DEFAULT_FIELD_30, 0x7F80_0001);
        let at = e.mem.alloc(0x58);
        for word in 0..22 {
            e.mem.set_u32(at + 4 * word, 0xDEAD_BEEF);
        }
        let r = e.call(0x00b3_cd60, &args![Ptr::<()>::new(at)]);
        assert_eq!(r.u32(), at);
        assert_eq!(*built.borrow(), vec![vec![at + 0x10]]);
        assert_eq!(e.mem.u8(at), 1);
        assert_eq!(e.mem.f32(at + 4), 1.0);
        assert_eq!(e.mem.u32(at + 0x0C), 2);
        assert_eq!(e.mem.f32(at + 0x28), 0.5);
        assert_eq!(e.mem.f32(at + 0x2C), 0.75);
        assert_eq!(e.mem.u32(at + 0x30), 0x7FC0_0001);
        for (offset, value) in [
            (0x34u32, 1u32),
            (0x40, 3),
            (0x44, 4),
            (0x48, 0),
            (0x4C, 0),
            (0x50, 0),
            (0x54, 0),
        ] {
            assert_eq!(e.mem.u32(at + offset), value, "offset {offset:#x}");
        }
        // The words the constructor does not touch stay.
        assert_eq!(e.mem.u32(at + 8), 0xDEAD_BEEF);
    }

    #[test]
    fn class_destructor_frees_the_arrays_in_order() {
        let mut e = engine();
        let log: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        for address in [OPERATOR_DELETE, MEMBER_VECTOR_DESTRUCT] {
            let sink = log.clone();
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a[0]));
                Ret::default()
            });
        }
        let at = e.mem.alloc(0x58);
        let entries = e.mem.alloc(16);
        for i in 0..3 {
            e.mem.set_u32(entries + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(at + 0x10, 0x7777);
        e.mem.set_u32(at + 0x44, 3);
        e.mem.set_u32(at + 0x4C, entries);
        e.mem.set_u32(at + 0x50, 0x5000);
        e.mem.set_u32(at + 0x54, 0x5400);
        e.call(0x00b3_ce30, &args![Ptr::<()>::new(at)]);
        assert_eq!(
            *log.borrow(),
            vec![
                (OPERATOR_DELETE, 0x5400),
                (OPERATOR_DELETE, 0x100),
                (OPERATOR_DELETE, 0x101),
                (OPERATOR_DELETE, 0x102),
                (OPERATOR_DELETE, entries),
                (OPERATOR_DELETE, 0x5000),
                (MEMBER_VECTOR_DESTRUCT, at + 0x10),
                (OPERATOR_DELETE, 0x7777),
            ]
        );
        assert_eq!(
            (
                e.mem.u32(at + 0x4C),
                e.mem.u32(at + 0x50),
                e.mem.u32(at + 0x54)
            ),
            (0, 0, 0)
        );
        // Without the entry array only the others are freed.
        log.borrow_mut().clear();
        e.mem.set_u32(at + 0x50, 0x5000);
        e.call(0x00b3_ce30, &args![Ptr::<()>::new(at)]);
        let calls: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            calls,
            vec![
                OPERATOR_DELETE,
                OPERATOR_DELETE,
                MEMBER_VECTOR_DESTRUCT,
                OPERATOR_DELETE
            ]
        );
    }
}
