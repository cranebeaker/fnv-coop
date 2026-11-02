//! `Havok SDK/hkxanimatedmatrixclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The calling thread's memory router (`00c85750`); its word at `+0x10` is the allocator whose
/// vtable slot `+8` frees `(block, size)` and slot `+0x10` frees `(array storage, bytes)`.
const GET_MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The base object's vtable (`.?AVhkBaseObject@@`) the destructor leaves.
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;
/// The size bits of an `hkArray`'s capacity word; bit 31 means it does not own its storage.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// The capacity word of an array with no storage that must not be deallocated.
const EMPTY_ARRAY_CAPACITY: u32 = 0x8000_0000;

layout! {
    /// `hkxAnimatedMatrix` (engine map name; not in the Xbox PDB): a referenced object (vtable,
    /// the 16-bit size-and-flags at +4 and the reference count at +6) followed by an array of
    /// `0x40`-byte elements (the matrices). The array's field names follow `hkArray` and are
    /// unconfirmed.
    pub struct HkxAnimatedMatrix: 0x14 {
        /// `m_memSizeAndFlags` (Xbox PDB `hkReferencedObject`).
        0x04 m_memSizeAndFlags: u16,
        /// `m_referenceCount` (Xbox PDB `hkReferencedObject`).
        0x06 m_referenceCount: i16,
        /// The matrices' storage.
        0x08 m_matrices_data: Ptr,
        /// The number of matrices.
        0x0C m_matrices_size: i32,
        /// The capacity word (matrix count in the low 30 bits, bit 31 for borrowed storage).
        0x10 m_matrices_capacityAndFlags: u32,
    }
}

// Translated from 00e02650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkxAnimatedMatrix::_scalar_deleting_destructor_` (engine map name), thiscall, one stack
/// word `flags`: empties the matrix array (the size becomes 0; storage the array owns is
/// freed through the allocator's slot `+0x10` with `capacity << 6` bytes; the data pointer
/// becomes null and the capacity word `0x80000000`), sets the base object's vtable and, when
/// bit 0 of `flags` is set, frees the object itself through the allocator's slot `+8` with
/// the 16-bit size at `+4`. Returns `this`.
pub fn hkx_animated_matrix_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HkxAnimatedMatrix>,
    flags: u8,
) -> Ptr<HkxAnimatedMatrix> {
    let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity_word = e.get(this, HkxAnimatedMatrix::m_matrices_capacityAndFlags);
    e.set(this, HkxAnimatedMatrix::m_matrices_size, 0);
    if (capacity_word as i32) >= 0 {
        let data = e.get(this, HkxAnimatedMatrix::m_matrices_data);
        let bytes = (capacity_word & CAPACITY_MASK) << 6;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.set(this, HkxAnimatedMatrix::m_matrices_data, Ptr::NULL);
    e.set(
        this,
        HkxAnimatedMatrix::m_matrices_capacityAndFlags,
        EMPTY_ARRAY_CAPACITY,
    );
    e.mem.set_u32(this.addr(), BASE_OBJECT_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(GET_MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.get(this, HkxAnimatedMatrix::m_memSizeAndFlags));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00e0_2650,
        hkx_animated_matrix_scalar_deleting_destructor(
            Ptr<HkxAnimatedMatrix>,
            u8,
        ) -> Ptr<HkxAnimatedMatrix>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_BLOCK_SLOT: u32 = 0x0900_0090;
    const FREE_ARRAY_SLOT: u32 = 0x0900_0091;

    type Frees = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> (Engine, Frees) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_BLOCK_SLOT);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x24);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(GET_MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let frees: Frees = Rc::new(RefCell::new(Vec::new()));
        for slot in [FREE_BLOCK_SLOT, FREE_ARRAY_SLOT] {
            let log = Rc::clone(&frees);
            e.register_double(slot, move |_, a| {
                log.borrow_mut().push((slot, a[1..].to_vec()));
                Ret::default()
            });
        }
        (e, frees)
    }

    fn matrix(e: &mut Engine, data: u32, size: u32, capacity: u32) -> u32 {
        let this = e.mem.alloc(0x14);
        e.mem.set_u16(this + 4, 0x1c);
        e.mem.set_u32(this, 0x1111);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, size);
        e.mem.set_u32(this + 0x10, capacity);
        this
    }

    #[test]
    fn destructor_frees_owned_storage_in_matrix_sized_units_and_the_object() {
        let (mut e, frees) = engine();
        let this = matrix(&mut e, 0x5000, 3, 5);
        assert_eq!(e.call(0x00e0_2650, &args![this, 1u8]).u32(), this);
        // Storage: capacity 5 matrices of 0x40 bytes; the object: its 16-bit size.
        assert_eq!(
            *frees.borrow(),
            vec![
                (FREE_ARRAY_SLOT, vec![0x5000, 5 << 6]),
                (FREE_BLOCK_SLOT, vec![this, 0x1c])
            ]
        );
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.i32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
    }

    #[test]
    fn destructor_keeps_the_object_without_bit_zero_and_borrowed_storage() {
        let (mut e, frees) = engine();
        // Borrowed storage (bit 31 set): the array is not freed; flags without bit 0: neither
        // is the object.
        let this = matrix(&mut e, 0x5000, 3, 0x8000_0005);
        e.call(0x00e0_2650, &args![this, 0xfeu8]);
        assert!(frees.borrow().is_empty());
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        // Only the low byte of the flags word is the flags: 0x03 has bit 0.
        let other = matrix(&mut e, 0, 0, 0x8000_0000);
        e.call(0x00e0_2650, &args![other, 0x03u8]);
        assert_eq!(*frees.borrow(), vec![(FREE_BLOCK_SLOT, vec![other, 0x1c])]);
    }
}
