//! `Havok SDK/hkpmultirayshapeclass.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the allocator the array and the object
/// are given back to.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtable the destructor leaves in the object (`0102e388`, the `hkReferencedObject` one,
/// as the other Havok destructors do).
const BASE_VTABLE: u32 = 0x0102_e388;
/// `m_rays` (Xbox PDB), an `hkArray` of 32-byte rays: data +0x10, size +0x14, capacity and
/// flags +0x18.
const RAYS: u32 = 0x10;
/// Bit 31 of the capacity word: the array does not own its storage.
const DONT_DEALLOCATE_FLAG: u32 = 0x8000_0000;
/// The capacity bits of that word.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00dfd130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiRayShape::_scalar_deleting_destructor_` (Xbox PDB): `ECX` = shape and one stack word,
/// `flags`, of which bit 0 asks for the object itself to be freed. Empties `m_rays` (size 0;
/// the storage is returned to the allocator, slot `0x10`, as `(data, capacity * 32)` unless
/// bit 31 of the capacity word says it is not owned), leaves data null and the capacity word
/// at `0x80000000`, sets the vtable `0102e388` and, with bit 0 of `flags`, frees the object
/// through the allocator's slot `8` with the 16-bit size kept at `+4`. Returns `this`.
pub fn hkp_multi_ray_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let me = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + RAYS + 8);
    e.mem.set_u32(me + RAYS + 4, 0);
    if capacity & DONT_DEALLOCATE_FLAG == 0 {
        let data = e.mem.u32(me + RAYS);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK) << 5],
        );
    }
    e.mem.set_u32(me + RAYS, 0);
    e.mem.set_u32(me + RAYS + 8, DONT_DEALLOCATE_FLAG);
    e.mem.set_u32(me, BASE_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(me + 4));
        e.vcall(allocator, 8, &args![me, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00dfd130,
        hkp_multi_ray_shape_scalar_deleting_destructor(Ptr, u8) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    const ROUTER: u32 = 0x0030_1000;
    const ALLOCATOR: u32 = 0x0030_2000;
    const ALLOCATOR_VTABLE: u32 = 0x0030_3000;
    const FAKE_DELETE: u32 = 0x7000_0008;
    const FAKE_FREE: u32 = 0x7000_0010;

    fn setup() -> Engine {
        let mut e = Engine::new();
        e.map(ROUTER, 0x20);
        e.mem.set_u32(ROUTER + 0x10, ALLOCATOR);
        e.map(ALLOCATOR, 8);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.put_vtable(ALLOCATOR_VTABLE, &[0, 0, FAKE_DELETE, 0, FAKE_FREE]);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: ROUTER,
            ..Ret::default()
        });
        e.register(FAKE_DELETE, |_, _| Ret::default());
        e.register(FAKE_FREE, |_, _| Ret::default());
        e
    }

    fn shape(e: &mut Engine, capacity: u32) -> u32 {
        let me = e.mem.alloc(0x30);
        e.mem.set_u32(me, 0x1234);
        e.mem.set_u16(me + 4, 0x30);
        e.mem.set_u32(me + RAYS, 0x00c0_0000);
        e.mem.set_u32(me + RAYS + 4, 3);
        e.mem.set_u32(me + RAYS + 8, capacity);
        me
    }

    #[test]
    fn owned_rays_are_freed_and_the_object_kept_without_the_flag() {
        let mut e = setup();
        let me = shape(&mut e, 8);
        e.call_log = Some(vec![]);
        let result = e.call(0x00df_d130, &args![me, 0u32]);
        let log: Log = e.call_log.take().unwrap();
        assert_eq!(result.u32(), me);
        let frees: Vec<_> = log.iter().filter(|c| c.0 == FAKE_FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, args![ALLOCATOR, 0x00c0_0000u32, 8u32 << 5]);
        assert!(log.iter().all(|c| c.0 != FAKE_DELETE));
        assert_eq!(e.mem.u32(me), BASE_VTABLE);
        assert_eq!(e.mem.u32(me + RAYS), 0);
        assert_eq!(e.mem.u32(me + RAYS + 4), 0);
        assert_eq!(e.mem.u32(me + RAYS + 8), 0x8000_0000);
    }

    #[test]
    fn unowned_rays_are_not_freed_and_the_flag_frees_the_object() {
        let mut e = setup();
        let me = shape(&mut e, 0x8000_0002);
        e.call_log = Some(vec![]);
        e.call(0x00df_d130, &args![me, 1u32]);
        let log: Log = e.call_log.take().unwrap();
        assert!(log.iter().all(|c| c.0 != FAKE_FREE));
        let deletes: Vec<_> = log.iter().filter(|c| c.0 == FAKE_DELETE).collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, args![ALLOCATOR, me, 0x30u32]);
    }

    #[test]
    fn the_capacity_flag_bits_are_masked_off_the_size() {
        let mut e = setup();
        let me = shape(&mut e, 0x4000_0005);
        e.call_log = Some(vec![]);
        e.call(0x00df_d130, &args![me, 0u32]);
        let log: Log = e.call_log.take().unwrap();
        let frees: Vec<_> = log.iter().filter(|c| c.0 == FAKE_FREE).collect();
        assert_eq!(frees[0].1[2], 5 << 5);
    }
}
