//! `Havok SDK/run_00deefd0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpPulleyConstraintData` (class confirmed by the RTTI of the vtable `010da04c` its
//! constructor stores) and the thunks of `hkpPairCollisionFilter`'s secondary vtables
//! (`010da09c`, `010da0a8`: RTTI `hkpPairCollisionFilter`) that forward a collision query to
//! the filter the collision filter holds as its child.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `hkpPulleyConstraintData` (RTTI).
const PULLEY_VTABLE: u32 = 0x010d_a04c;

// Translated from 00deefd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPulleyConstraintData::hkpPulleyConstraintData` (constructor; class confirmed by the RTTI
/// of the vtable it stores), thiscall without words: the 16-bit word at `+6` is 1, `+8` is 0, the
/// vtable goes to `+0`, the 16-bit words at `+0x10` and `+0x40` are 3 and 0x14, the floats at
/// `+0x70` and `+0x74` are `0.0` and `1.0` and the vectors at `+0x20`, `+0x30`, `+0x50` and
/// `+0x60` are zero. Returns `this`.
pub fn fn_00deefd0(e: &mut Engine, this: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me.wrapping_add(6), 1);
    e.mem.set_u32(me.wrapping_add(8), 0);
    e.mem.set_u32(me, PULLEY_VTABLE);
    e.mem.set_u16(me.wrapping_add(0x10), 3);
    e.mem.set_u16(me.wrapping_add(0x40), 0x14);
    e.mem.set_f32(me.wrapping_add(0x70), 0.0);
    e.mem.set_f32(me.wrapping_add(0x74), 1.0);
    for vector in [0x20u32, 0x30, 0x50, 0x60] {
        for lane in 0..4 {
            e.mem.set_u32(me.wrapping_add(vector + 4 * lane), 0);
        }
    }
    this
}

/// The common part of the three collision query thunks: asks `target` (the object at `child +
/// offset`, its vtable slot `slot`) with `words`, the first of which is the address of a
/// temporary byte, and answers whether the byte the callee names is non-zero. When there is
/// no child the answer is yes.
fn forward_query(e: &mut Engine, child: u32, offset: u32, slot: u32, words: &[u32]) -> bool {
    if child == 0 {
        return true;
    }
    // The exe uses the caller's last argument slot as the temporary byte (the word was
    // already pushed on): a block of its own does the same.
    e.with_stack(8, |e, scratch| {
        let mut all = vec![scratch.addr()];
        all.extend_from_slice(words);
        let result = e.vcall(child.wrapping_add(offset), slot, &all).u32();
        e.mem.u8(result) != 0
    })
}

// Translated from 00def020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of `hkpPairCollisionFilter`'s secondary vtable (slot of `010da0ac`; the engine map has
/// no name), thiscall with six words `(out, a2, a3, a4, a5, a6)`: when the child filter at
/// `this + 0x30` is set, asks its sub-object at `+0xc` (vtable slot `+4`) with a pointer to a
/// temporary byte and `a2` to `a6`; writes 1 to the byte at `out` when there is no child or the
/// answer's byte is non-zero, 0 otherwise. Returns `out`.
#[allow(clippy::too_many_arguments)] // the exe takes these six words and `this`
pub fn fn_00def020(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
) -> Ptr {
    let child = e.mem.u32(this.addr().wrapping_add(0x30));
    let allowed = forward_query(e, child, 0xc, 4, &[a2, a3, a4, a5, a6]);
    e.mem.set_u8(out.addr(), u8::from(allowed));
    out
}

// Translated from 00def070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of `hkpPairCollisionFilter`'s secondary vtable (slot of `010da0a8`; the engine map has
/// no name), thiscall with eight words `(out, a2 ..= a8)`: as `00def020` with the child's
/// sub-object at `+0xc`, vtable slot `0`, and seven words `a2` to `a8` after the temporary
/// byte. Returns `out`.
#[allow(clippy::too_many_arguments)] // the exe takes these eight words and `this`
pub fn fn_00def070(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
) -> Ptr {
    let child = e.mem.u32(this.addr().wrapping_add(0x30));
    let allowed = forward_query(e, child, 0xc, 0, &[a2, a3, a4, a5, a6, a7, a8]);
    e.mem.set_u8(out.addr(), u8::from(allowed));
    out
}

// Translated from 00def0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of `hkpPairCollisionFilter`'s secondary vtable (slot of `010da09c`; the engine map has
/// no name), thiscall with five words `(out, a2, a3, a4, a5)`: when the child filter at `this
/// + 0x2c` is set, asks its sub-object at `+0x10` (vtable slot `0`) with a pointer to a
/// temporary byte and `a2` to `a5`; writes 1 to the byte at `out` when there is no child or the
/// answer's byte is non-zero, 0 otherwise. Returns `out`.
pub fn fn_00def0d0(e: &mut Engine, this: Ptr, out: Ptr, a2: u32, a3: u32, a4: u32, a5: u32) -> Ptr {
    let child = e.mem.u32(this.addr().wrapping_add(0x2c));
    let allowed = forward_query(e, child, 0x10, 0, &[a2, a3, a4, a5]);
    e.mem.set_u8(out.addr(), u8::from(allowed));
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de_efd0, fn_00deefd0(Ptr) -> Ptr),
        entry!(0x00de_f020, fn_00def020(Ptr, Ptr, u32, u32, u32, u32, u32) -> Ptr),
        entry!(
            0x00de_f070,
            fn_00def070(Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00de_f0d0, fn_00def0d0(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn the_pulley_constructor_stores_the_defaults() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x80);
        for word in 0..0x20 {
            e.mem.set_u32(this + 4 * word, 0xdead_0000 + word);
        }
        let result = e.call(0x00de_efd0, &args![this]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), 0x010d_a04c);
        assert_eq!(e.mem.u16(this + 0x10), 3);
        assert_eq!(e.mem.u16(this + 0x40), 0x14);
        assert_eq!(e.mem.f32(this + 0x70), 0.0);
        assert_eq!(e.mem.f32(this + 0x74), 1.0);
        for vector in [0x20u32, 0x30, 0x50, 0x60] {
            for lane in 0..4 {
                assert_eq!(e.mem.u32(this + vector + 4 * lane), 0);
            }
        }
        // words the constructor does not store keep their value
        assert_eq!(e.mem.u32(this + 4), 0x0001_0001);
        assert_eq!(e.mem.u32(this + 0x14), 0xdead_0005);
    }

    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A filter object whose sub-object at `offset` has a vtable with the double at `slot`;
    /// the double logs its words, sets the temporary byte it was given to `byte` and answers a
    /// pointer to it.
    fn filter(e: &mut Engine, offset: u32, slot: u32, byte: u8) -> (u32, Seen) {
        let child = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(child + offset, vtable);
        e.mem.set_u32(vtable + slot, 0x0900_0000);
        let seen: Seen = Rc::default();
        let log = seen.clone();
        e.register_double(0x0900_0000, move |e, words| {
            log.borrow_mut().push(words.to_vec());
            e.mem.set_u8(words[1], byte);
            Ret {
                eax: words[1],
                ..Ret::default()
            }
        });
        (child, seen)
    }

    #[test]
    fn the_five_word_query_allows_without_a_child() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        let result = e.call(0x00de_f020, &args![this, out, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(result.u32(), out);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn the_five_word_query_forwards_to_slot_one_of_the_child() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        let (child, seen) = filter(&mut e, 0xc, 4, 0);
        e.mem.set_u32(this + 0x30, child);
        e.call(0x00de_f020, &args![this, out, 1u32, 2u32, 3u32, 4u32, 5u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][0], child + 0xc);
        assert_eq!(seen[0][2..], [1, 2, 3, 4, 5]);
        // the byte the callee named was 0: the answer is 0
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn the_five_word_query_allows_when_the_childs_byte_is_set() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        let (child, _) = filter(&mut e, 0xc, 4, 7);
        e.mem.set_u32(this + 0x30, child);
        e.call(0x00de_f020, &args![this, out, 1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn the_seven_word_query_forwards_to_slot_zero_of_the_child() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        let (child, seen) = filter(&mut e, 0xc, 0, 0);
        e.mem.set_u32(this + 0x30, child);
        let result = e.call(
            0x00de_f070,
            &args![this, out, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
        );
        assert_eq!(result.u32(), out);
        let seen = seen.borrow();
        assert_eq!(seen[0][0], child + 0xc);
        assert_eq!(seen[0][2..], [1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn the_seven_word_query_allows_without_a_child() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        e.call(
            0x00de_f070,
            &args![this, out, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32],
        );
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn the_four_word_query_forwards_to_the_sub_object_at_0x10() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        let (child, seen) = filter(&mut e, 0x10, 0, 0);
        e.mem.set_u32(this + 0x2c, child);
        let result = e.call(0x00de_f0d0, &args![this, out, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(result.u32(), out);
        let seen = seen.borrow();
        assert_eq!(seen[0][0], child + 0x10);
        assert_eq!(seen[0][2..], [1, 2, 3, 4]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn the_four_word_query_allows_without_a_child() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let out = e.mem.alloc(8);
        e.call(0x00de_f0d0, &args![this, out, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(e.mem.u8(out), 1);
    }
}
