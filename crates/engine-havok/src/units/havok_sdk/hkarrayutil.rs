//! `Havok SDK/hkarrayutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Havok's out-of-line array growth: the templated `hkArray` calls these with its allocator
//! (called through its vtable: slot `0x0C` allocates `(size pointer)`, slot `0x10` frees
//! `(pointer, bytes)`, slot `0x14` reallocates `(pointer, old bytes, size pointer)`; the size
//! pointers are in/out: the allocator may round the request up) and the array (data, size,
//! and capacity with the flag bits in the top two bits; bit 31 is "does not own its
//! storage").
//!
//! All three are cdecl (the caller cleans the stack) and divide the allocated byte count by
//! the element size with a signed `IDIV`, as the exe does.

#[allow(unused_imports)]
use crate::prelude::*;

/// `memcpy` (`00ec44d0`, cdecl `(destination, source, bytes)`).
const MEMCPY: u32 = 0x00ec_44d0;
/// The capacity field's size bits; the top two bits are flags.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
/// Capacity flag: the array does not own its storage.
const DONT_DEALLOCATE_FLAG: u32 = 0x8000_0000;

layout! {
    /// `hkArrayBase` (Xbox PDB) in the PC build: data pointer, element count, capacity.
    pub struct HkArrayBase: 0xC {
        /// `m_data` (Xbox PDB).
        0x00 m_data: Ptr,
        /// `m_size` (Xbox PDB).
        0x04 m_size: i32,
        /// `m_capacityAndFlags` (Xbox PDB).
        0x08 m_capacityAndFlags: u32,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c908c0,
            hk_array_util_reserve(Ptr, Ptr<HkArrayBase>, i32, i32)
        ),
        entry!(0x00c90940, fn_00c90940(Ptr, Ptr<HkArrayBase>, i32)),
        entry!(
            0x00c909d0,
            hk_array_util_reduce(Ptr, Ptr<HkArrayBase>, i32, Ptr, i32)
        ),
    ]
}

/// The tail shared by `_reserve` and `00c90940`: gives `array` storage for `bytes` bytes.
/// An array that owns its storage is reallocated (slot `0x14`); one that does not gets a new
/// block (slot `0x0C`) with the old elements copied in. The capacity becomes the byte count
/// the allocator reports, divided by `element_size`.
fn regrow(e: &mut Engine, allocator: Ptr, array: Ptr<HkArrayBase>, bytes: i32, element_size: i32) {
    let capacity = e.get(array, HkArrayBase::m_capacityAndFlags);
    e.with_stack(4, |e, request| {
        e.mem.set_i32(request.addr(), bytes);
        if (capacity as i32) >= 0 {
            let data = e.get(array, HkArrayBase::m_data);
            let old_bytes = ((capacity & CAPACITY_MASK) as i32).wrapping_mul(element_size);
            let storage = e
                .vcall(allocator.addr(), 0x14, &args![data, old_bytes, request])
                .ptr::<()>();
            e.set(array, HkArrayBase::m_data, storage);
        } else {
            let storage = e.vcall(allocator.addr(), 0xc, &args![request]).u32();
            let size = e.get(array, HkArrayBase::m_size);
            let data = e.get(array, HkArrayBase::m_data);
            e.call(
                MEMCPY,
                &args![storage, data, size.wrapping_mul(element_size)],
            );
            e.set(array, HkArrayBase::m_data, Ptr::new(storage));
        }
        let granted = e.mem.i32(request.addr());
        e.set(
            array,
            HkArrayBase::m_capacityAndFlags,
            (granted / element_size) as u32,
        );
    });
}

// Translated from 00c908c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayUtil::_reserve` (Xbox PDB), cdecl `(allocator, array, count, element size)`: makes
/// room for `count` elements.
pub fn hk_array_util_reserve(
    e: &mut Engine,
    allocator: Ptr,
    array: Ptr<HkArrayBase>,
    count: i32,
    element_size: i32,
) {
    regrow(
        e,
        allocator,
        array,
        count.wrapping_mul(element_size),
        element_size,
    );
}

// Translated from 00c90940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; cdecl `(allocator, array, element size)`: the growth step of
/// an array that is full. Asks for room for one element when the array is empty, otherwise
/// for twice its current element count.
pub fn fn_00c90940(e: &mut Engine, allocator: Ptr, array: Ptr<HkArrayBase>, element_size: i32) {
    let size = e.get(array, HkArrayBase::m_size);
    let count = if size == 0 {
        1
    } else {
        size.wrapping_add(size)
    };
    regrow(
        e,
        allocator,
        array,
        count.wrapping_mul(element_size),
        element_size,
    );
}

// Translated from 00c909d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayUtil::_reduce` (Xbox PDB), cdecl `(allocator, array, element size, buffer,
/// minimum capacity)`: when a `buffer` is given and the array holds fewer than `minimum
/// capacity` elements, moves the elements into the buffer (which the array then does not
/// own) and frees the old storage (slot `0x10`); otherwise reallocates (slot `0x14`) to
/// `minimum capacity` elements.
pub fn hk_array_util_reduce(
    e: &mut Engine,
    allocator: Ptr,
    array: Ptr<HkArrayBase>,
    element_size: i32,
    buffer: Ptr,
    minimum_capacity: i32,
) {
    let size = e.get(array, HkArrayBase::m_size);
    let data = e.get(array, HkArrayBase::m_data);
    let capacity = e.get(array, HkArrayBase::m_capacityAndFlags);
    if !buffer.is_null() && size < minimum_capacity {
        e.call(
            MEMCPY,
            &args![buffer, data, size.wrapping_mul(element_size)],
        );
        let old_bytes = ((capacity & CAPACITY_MASK) as i32).wrapping_mul(element_size);
        e.vcall(allocator.addr(), 0x10, &args![data, old_bytes]);
        e.set(
            array,
            HkArrayBase::m_capacityAndFlags,
            minimum_capacity as u32 | DONT_DEALLOCATE_FLAG,
        );
        e.set(array, HkArrayBase::m_data, buffer);
    } else {
        e.with_stack(4, |e, request| {
            e.mem
                .set_i32(request.addr(), element_size.wrapping_mul(minimum_capacity));
            let old_bytes = (capacity as i32).wrapping_mul(element_size);
            let storage = e
                .vcall(allocator.addr(), 0x14, &args![data, old_bytes, request])
                .ptr::<()>();
            e.set(array, HkArrayBase::m_data, storage);
            let granted = e.mem.i32(request.addr());
            e.set(
                array,
                HkArrayBase::m_capacityAndFlags,
                (granted / element_size) as u32,
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATOR_VTABLE: u32 = 0x0200_0000;
    const ALLOCATE: u32 = 0x0200_0100;
    const FREE: u32 = 0x0200_0104;
    const REALLOCATE: u32 = 0x0200_0108;

    /// An allocator whose slots hand out fresh blocks and round every request up by 8.
    fn rig() -> (Engine, Ptr, Ptr<HkArrayBase>) {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.put_vtable(ALLOCATOR_VTABLE, &[0, 0, 0, ALLOCATE, FREE, REALLOCATE]);
        e.register(ALLOCATE, |e, a| {
            let bytes = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], bytes + 8);
            e.mem.alloc(bytes + 8).into_ret()
        });
        e.register(FREE, |_, _| Ret::default());
        e.register(REALLOCATE, |e, a| {
            let bytes = e.mem.u32(a[3]);
            e.mem.set_u32(a[3], bytes + 8);
            e.mem.alloc(bytes + 8).into_ret()
        });
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let array = e.new_object::<HkArrayBase>();
        (e, Ptr::new(allocator), array)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn reserve_reallocates_an_owning_array() {
        let (mut e, allocator, array) = rig();
        e.set(array, HkArrayBase::m_data, Ptr::new(0x5000));
        e.set(array, HkArrayBase::m_capacityAndFlags, 3);
        e.call(0x00c9_08c0, &args![allocator, array, 5i32, 16i32]);
        // The old block of 3 * 16 bytes is passed; 5 * 16 bytes are asked for, 8 more granted.
        let realloc = calls(&e, REALLOCATE);
        assert_eq!(realloc.len(), 1);
        assert_eq!(realloc[0][..3], [allocator.addr(), 0x5000, 48]);
        assert_eq!(e.get(array, HkArrayBase::m_capacityAndFlags), 88 / 16);
        assert_ne!(e.get(array, HkArrayBase::m_data).addr(), 0x5000);
        assert!(calls(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn reserve_copies_out_of_borrowed_storage_and_drops_the_flag() {
        let (mut e, allocator, array) = rig();
        let borrowed = e.mem.alloc(32);
        e.mem.set_u32(borrowed, 0xabcd_1234);
        e.set(array, HkArrayBase::m_data, Ptr::new(borrowed));
        e.set(array, HkArrayBase::m_size, 2);
        e.set(array, HkArrayBase::m_capacityAndFlags, 0x8000_0002);
        e.call(0x00c9_08c0, &args![allocator, array, 6i32, 8i32]);
        assert_eq!(calls(&e, ALLOCATE).len(), 1);
        assert!(calls(&e, REALLOCATE).is_empty());
        let copied = e.get(array, HkArrayBase::m_data);
        assert_eq!(e.mem.u32(copied.addr()), 0xabcd_1234);
        assert_eq!(e.get(array, HkArrayBase::m_capacityAndFlags), (48 + 8) / 8);
        let copies = calls(&e, MEMCPY);
        assert_eq!(copies[0][1..], [borrowed, 16]);
    }

    #[test]
    fn reserve_more_asks_for_one_then_double() {
        let (mut e, allocator, array) = rig();
        e.call(0x00c9_0940, &args![allocator, array, 0x70i32]);
        // One element of 0x70 bytes plus the 8 bytes of rounding.
        assert_eq!(
            e.get(array, HkArrayBase::m_capacityAndFlags),
            (0x70 + 8) / 0x70
        );
        e.set(array, HkArrayBase::m_size, 4);
        e.call(0x00c9_0940, &args![allocator, array, 8i32]);
        // Eight elements of eight bytes plus the rounding.
        assert_eq!(e.get(array, HkArrayBase::m_capacityAndFlags), (64 + 8) / 8);
    }

    #[test]
    fn reduce_moves_into_the_buffer_only_when_it_is_larger() {
        let (mut e, allocator, array) = rig();
        let old = e.mem.alloc(64);
        let buffer = e.mem.alloc(64);
        e.mem.set_u32(old, 77);
        e.set(array, HkArrayBase::m_data, Ptr::new(old));
        e.set(array, HkArrayBase::m_size, 2);
        e.set(array, HkArrayBase::m_capacityAndFlags, 4);
        e.call(
            0x00c9_09d0,
            &args![allocator, array, 8i32, Ptr::<()>::new(buffer), 6i32],
        );
        assert_eq!(e.get(array, HkArrayBase::m_data).addr(), buffer);
        assert_eq!(e.mem.u32(buffer), 77);
        assert_eq!(e.get(array, HkArrayBase::m_capacityAndFlags), 0x8000_0006);
        assert_eq!(calls(&e, FREE)[0][1..], [old, 32]);

        // The array already holds enough: it is reallocated to the minimum capacity.
        let (mut e, allocator, array) = rig();
        e.set(array, HkArrayBase::m_data, Ptr::new(0x6000));
        e.set(array, HkArrayBase::m_size, 5);
        e.set(array, HkArrayBase::m_capacityAndFlags, 9);
        e.call(
            0x00c9_09d0,
            &args![allocator, array, 8i32, Ptr::<()>::new(0x7000), 3i32],
        );
        let realloc = calls(&e, REALLOCATE);
        assert_eq!(realloc[0][1..3], [0x6000, 72]);
        assert_eq!(e.get(array, HkArrayBase::m_capacityAndFlags), (24 + 8) / 8);
        assert!(calls(&e, FREE).is_empty());
    }
}
