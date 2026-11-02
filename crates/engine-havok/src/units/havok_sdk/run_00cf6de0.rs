//! `Havok SDK/run_00cf6de0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four of these are entries of a vtable (the one at `010c63bc`, the one `00cf6f90` installs
//! at `+0x108`; the class is unconfirmed) that take a pair of objects `(A, B)` in a two-word
//! block. Each object has a type byte at `+4` and a signed byte at `+5`, the distance to its
//! "main" part; that part has a second signed byte at `+0x10`, the distance to the sub-object
//! whose vtable is called. A call goes to the function at the given slot of the sub-object of
//! one object, with the main part of the other as its stack argument.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable word of `hkBaseObject` (`0102e388`) written by `00cf7060`.
const BASE_VTABLE: u32 = 0x0102_e388;

/// The signed byte at `at`.
fn signed_byte(e: &Engine, at: u32) -> u32 {
    e.mem.u8(at) as i8 as i32 as u32
}

/// The main part of `object`: `object + (signed byte at +5)`.
fn main_part(e: &Engine, object: u32) -> u32 {
    object.wrapping_add(signed_byte(e, object.wrapping_add(5)))
}

/// Calls virtual method `slot` of the sub-object of `object` (`main + (signed byte at
/// main + 0x10)` with `main = main_part(object)`) with `argument`.
fn call_sub_object(e: &mut Engine, object: u32, slot: u32, argument: u32) {
    let main = main_part(e, object);
    let sub_object = main.wrapping_add(signed_byte(e, main.wrapping_add(0x10)));
    e.vcall(sub_object, slot, &args![argument]);
}

/// The body of `00cf6de0` (`slot` `0x1c`) and `00cf6e40` (`slot` `0x24`): for each of the two
/// objects of the pair whose type byte is 2, calls the slot of its sub-object with the main part
/// of the other. The first call happens first (and its `B` is read from the pair before the
/// call), the second is the exe's tail call.
fn dispatch_type_2(e: &mut Engine, pair: Ptr, slot: u32) {
    let first = e.mem.u32(pair.addr());
    if e.mem.u8(first + 4) == 2 {
        let second = e.mem.u32(pair.addr() + 4);
        let argument = main_part(e, second);
        call_sub_object(e, first, slot, argument);
    }
    let second = e.mem.u32(pair.addr() + 4);
    if e.mem.u8(second + 4) == 2 {
        let first = e.mem.u32(pair.addr());
        let argument = main_part(e, first);
        call_sub_object(e, second, slot, argument);
    }
}

/// The body of `00cf6ea0` (`slot` `0x1c`) and `00cf6f10` (`slot` `0x24`): like `dispatch_type_2`
/// for type 3, but when both objects have type 3 nothing is called. (The exe compares the
/// register holding the first type with itself before the first call; that jump is never
/// taken.)
fn dispatch_type_3(e: &mut Engine, pair: Ptr, slot: u32) {
    let first = e.mem.u32(pair.addr());
    let kind = e.mem.u8(first + 4);
    if kind == 3 {
        let second = e.mem.u32(pair.addr() + 4);
        if e.mem.u8(second + 4) == kind {
            return;
        }
        // `CMP AL,AL` / `JNZ`: the same register against itself, never different.
        let argument = main_part(e, second);
        call_sub_object(e, first, slot, argument);
    }
    let second = e.mem.u32(pair.addr() + 4);
    if e.mem.u8(second + 4) == 3 {
        let first = e.mem.u32(pair.addr());
        let argument = main_part(e, first);
        call_sub_object(e, second, slot, argument);
    }
}

// Translated from 00cf6de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable entry (data reference at `010c63e4`, `this` unused): for each object of the pair whose
/// type is 2, calls virtual method `+0x1c` of its sub-object with the other's main part (see
/// the module note).
pub fn fn_00cf6de0(e: &mut Engine, _this: Ptr, pair: Ptr) {
    dispatch_type_2(e, pair, 0x1c);
}

// Translated from 00cf6e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same for virtual method `+0x24` (data reference at `010c63e8`).
pub fn fn_00cf6e40(e: &mut Engine, _this: Ptr, pair: Ptr) {
    dispatch_type_2(e, pair, 0x24);
}

// Translated from 00cf6ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable entry (data reference at `010c6404`, `this` unused): like `00cf6de0` for objects of type
/// 3, virtual method `+0x1c`; nothing at all when both have type 3.
pub fn fn_00cf6ea0(e: &mut Engine, _this: Ptr, pair: Ptr) {
    dispatch_type_3(e, pair, 0x1c);
}

// Translated from 00cf6f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same for virtual method `+0x24` (data reference at `010c6408`).
pub fn fn_00cf6f10(e: &mut Engine, _this: Ptr, pair: Ptr) {
    dispatch_type_3(e, pair, 0x24);
}

// Translated from 00cf6f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises a block (the class is unconfirmed; `this` is returned): the 16-bit word at
/// `+0x106` becomes 1, the words at `+0x108` and `+0x100` the final tables (`010cc4d0` and
/// `010cc4e0`; `010c63bc` is written to `+0x108` first), and all 64 words from `+0` to `+0xfc`
/// are set to `this + 0x108` (0 when `this + 0x100` is null).
pub fn fn_00cf6f90(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let inner = base.wrapping_add(0x100);
    e.mem.set_u16(inner + 6, 1);
    e.mem.set_u32(inner + 8, 0x010c_63bc);
    e.mem.set_u32(inner, 0x010c_c4e0);
    e.mem.set_u32(inner + 8, 0x010c_c4d0);
    let target = if inner != 0 {
        base.wrapping_add(0x108)
    } else {
        0
    };
    for word in 0..64 {
        e.mem.set_u32(base + 4 * word, target);
    }
    this
}

// Translated from 00cf7060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the vtable words `010c63bc` at `+0x108` and `0102e388` at `+0x100`.
pub fn fn_00cf7060(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x108, 0x010c_63bc);
    e.mem.set_u32(this.addr() + 0x100, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_6de0, fn_00cf6de0(Ptr, Ptr)),
        entry!(0x00cf_6e40, fn_00cf6e40(Ptr, Ptr)),
        entry!(0x00cf_6ea0, fn_00cf6ea0(Ptr, Ptr)),
        entry!(0x00cf_6f10, fn_00cf6f10(Ptr, Ptr)),
        entry!(0x00cf_6f90, fn_00cf6f90(Ptr) -> Ptr),
        entry!(0x00cf_7060, fn_00cf7060(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SLOT_1C_DOUBLE: u32 = 0x0900_0071;
    const SLOT_24_DOUBLE: u32 = 0x0900_0091;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// An object of the given type whose main part is `main_delta` bytes from it (a negative
    /// delta means the object sits after its main part) and whose sub-object is `sub_delta`
    /// from the main part. Returns the object, its main part and its sub-object.
    fn object(
        e: &mut Engine,
        block: u32,
        kind: u8,
        main_delta: i8,
        sub_delta: i8,
    ) -> (u32, u32, u32) {
        let at = block + 0x40;
        e.mem.set_u8(at + 4, kind);
        e.mem.set_u8(at + 5, main_delta as u8);
        let main = at.wrapping_add(main_delta as i32 as u32);
        e.mem.set_u8(main + 0x10, sub_delta as u8);
        let sub = main.wrapping_add(sub_delta as i32 as u32);
        (at, main, sub)
    }

    /// Builds a pair of objects (their blocks do not overlap) with `kinds`, the sub-objects
    /// sharing a vtable whose slots `0x1c` and `0x24` log their `this` and argument.
    fn rig(kinds: [u8; 2]) -> (Engine, Log, u32, [(u32, u32, u32); 2]) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(SLOT_1C_DOUBLE, move |_, a| {
            sink.borrow_mut().push(("slot1c", a.to_vec()));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SLOT_24_DOUBLE, move |_, a| {
            sink.borrow_mut().push(("slot24", a.to_vec()));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x1c, SLOT_1C_DOUBLE);
        e.mem.set_u32(vtable + 0x24, SLOT_24_DOUBLE);
        let first_block = e.mem.alloc(0x100);
        let second_block = e.mem.alloc(0x100);
        let first = object(&mut e, first_block, kinds[0], -4, 12);
        let second = object(&mut e, second_block, kinds[1], 8, 8);
        e.mem.set_u32(first.2, vtable);
        e.mem.set_u32(second.2, vtable);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, first.0);
        e.mem.set_u32(pair + 4, second.0);
        (e, log, pair, [first, second])
    }

    /// Type-2 objects call their own sub-object with the other's main part: the first object
    /// first, then the second; objects of another type do not call.
    #[test]
    fn type_two_objects_call_their_slots() {
        for (address, name) in [(0x00cf_6de0, "slot1c"), (0x00cf_6e40, "slot24")] {
            let (mut e, log, pair, objects) = rig([2, 2]);
            e.call(address, &args![0x1234u32, pair]);
            assert_eq!(
                *log.borrow(),
                vec![
                    (name, vec![objects[0].2, objects[1].1]),
                    (name, vec![objects[1].2, objects[0].1])
                ]
            );
            let (mut e, log, pair, objects) = rig([2, 7]);
            e.call(address, &args![0x1234u32, pair]);
            assert_eq!(
                *log.borrow(),
                vec![(name, vec![objects[0].2, objects[1].1])]
            );
            let (mut e, log, pair, objects) = rig([7, 2]);
            e.call(address, &args![0x1234u32, pair]);
            assert_eq!(
                *log.borrow(),
                vec![(name, vec![objects[1].2, objects[0].1])]
            );
            let (mut e, log, pair, _) = rig([1, 3]);
            e.call(address, &args![0x1234u32, pair]);
            assert!(log.borrow().is_empty());
        }
    }

    /// Exactly one type-3 object calls; two of them (or none) do nothing.
    #[test]
    fn type_three_objects_call_unless_both_are() {
        for (address, name) in [(0x00cf_6ea0, "slot1c"), (0x00cf_6f10, "slot24")] {
            let (mut e, log, pair, objects) = rig([3, 2]);
            e.call(address, &args![0x1234u32, pair]);
            assert_eq!(
                *log.borrow(),
                vec![(name, vec![objects[0].2, objects[1].1])]
            );
            let (mut e, log, pair, objects) = rig([2, 3]);
            e.call(address, &args![0x1234u32, pair]);
            assert_eq!(
                *log.borrow(),
                vec![(name, vec![objects[1].2, objects[0].1])]
            );
            let (mut e, log, pair, _) = rig([3, 3]);
            e.call(address, &args![0x1234u32, pair]);
            assert!(log.borrow().is_empty());
            let (mut e, log, pair, _) = rig([1, 2]);
            e.call(address, &args![0x1234u32, pair]);
            assert!(log.borrow().is_empty());
        }
    }

    /// The initialiser fills 64 words and the tables; the destructor part writes two words.
    #[test]
    fn initialiser_fills_the_pointer_table_and_the_tables() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x120);
        e.mem.write(this, &[0xaa; 0x120]);
        let got = e.call(0x00cf_6f90, &args![this]).u32();
        assert_eq!(got, this);
        for word in 0..64 {
            assert_eq!(e.mem.u32(this + 4 * word), this + 0x108);
        }
        assert_eq!(e.mem.u16(this + 0x106), 1);
        assert_eq!(e.mem.u32(this + 0x100), 0x010c_c4e0);
        assert_eq!(e.mem.u32(this + 0x108), 0x010c_c4d0);
        assert_eq!(e.mem.u32(this + 0x10c), 0xaaaa_aaaa);
        e.call(0x00cf_7060, &args![this]);
        assert_eq!(e.mem.u32(this + 0x108), 0x010c_63bc);
        assert_eq!(e.mem.u32(this + 0x100), BASE_VTABLE);
    }
}
