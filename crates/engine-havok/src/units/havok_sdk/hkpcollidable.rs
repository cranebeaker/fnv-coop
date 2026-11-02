//! `Havok SDK/hkpcollidable.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): word +0x10 is the heap allocator; its vtable slot 4 is
/// `allocate(bytes)` and slot 8 is `deallocate(pointer, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d01a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidable::BoundingVolumeData::deallocate` (Xbox PDB): frees the two blocks of the
/// bounding volume data, the 32-byte entries at +0x24 (`count` of them, `count` being the
/// word at +0x22) and the words at +0x28 (`count` rounded up to a multiple of 4, times 4
/// bytes), and clears both pointers and both words (+0x20, +0x22).
pub fn hkp_collidable_bounding_volume_data_deallocate(e: &mut Engine, this: Ptr) {
    let entries = e.mem.u32(this.addr() + 0x24);
    let count = u32::from(e.mem.u16(this.addr() + 0x22));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 8, &args![entries, count << 5]);
    let count = u32::from(e.mem.u16(this.addr() + 0x22));
    let words = e.mem.u32(this.addr() + 0x28);
    let rounded = count.wrapping_add(3) & 0xffff_fffc;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 8, &args![words, rounded.wrapping_mul(4)]);
    e.mem.set_u32(this.addr() + 0x24, 0);
    e.mem.set_u32(this.addr() + 0x28, 0);
    e.mem.set_u16(this.addr() + 0x20, 0);
    e.mem.set_u16(this.addr() + 0x22, 0);
}

// Translated from 00d01ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidable::BoundingVolumeData::allocate` (Xbox PDB): stores the low 16 bits of
/// `count` in the words at +0x20 and +0x22; when `count` is non-zero it allocates the
/// 32-byte entries (+0x24) and the words (+0x28, `count` rounded up to a multiple of 4, times
/// 4 bytes) from the heap allocator.
pub fn hkp_collidable_bounding_volume_data_allocate(e: &mut Engine, this: Ptr, count: u32) {
    e.mem.set_u16(this.addr() + 0x20, count as u16);
    e.mem.set_u16(this.addr() + 0x22, count as u16);
    if count != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let entries = e
            .vcall(allocator, 4, &args![u32::from(count as u16) << 5])
            .u32();
        let rounded = (u32::from(e.mem.u16(this.addr() + 0x22)) + 3) & 0xffff_fffc;
        e.mem.set_u32(this.addr() + 0x24, entries);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let words = e.vcall(allocator, 4, &args![rounded.wrapping_mul(4)]).u32();
        e.mem.set_u32(this.addr() + 0x28, words);
    }
}

// Translated from 00d01a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCollidable::BoundingVolumeData::BoundingVolumeData` (Xbox PDB; `this` is ECX): clears
/// the two half-words at +0x20 and +0x22 and the two pointers at +0x24 and +0x28, sets the
/// word at +0 to 1 and clears the word at +0x10. Returns `this` (the exe leaves it in `EAX`).
pub fn hkp_collidable_bounding_volume_data_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let address = this.addr();
    e.mem.set_u16(address + 0x20, 0);
    e.mem.set_u16(address + 0x22, 0);
    e.mem.set_u32(address + 0x24, 0);
    e.mem.set_u32(address + 0x28, 0);
    e.mem.set_u32(address, 1);
    e.mem.set_u32(address + 0x10, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d0_1a80,
            hkp_collidable_bounding_volume_data_deallocate(Ptr)
        ),
        entry!(
            0x00d0_1ae0,
            hkp_collidable_bounding_volume_data_allocate(Ptr, u32)
        ),
        entry!(
            0x00d0_1a60,
            hkp_collidable_bounding_volume_data_constructor(Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Log = Rc<RefCell<Vec<(&'static str, u32, u32)>>>;

    /// A memory router whose allocator logs `(what, bytes or pointer, bytes)`.
    fn rig() -> (Engine, Log) {
        let mut e = test_engine();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0004, move |e, a| {
            sink.borrow_mut().push(("allocate", a[1], 0));
            Ret {
                eax: e.mem.alloc(a[1].max(1)),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0008, move |_, a| {
            sink.borrow_mut().push(("free", a[1], a[2]));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (e, log)
    }

    #[test]
    fn allocate_makes_both_blocks_and_stores_the_count() {
        let (mut e, log) = rig();
        let this = e.mem.alloc(0x40);
        e.call(0x00d0_1ae0, &args![this, 5u32]);
        // 5 entries of 32 bytes; 5 rounded up to 8 words of 4 bytes.
        assert_eq!(
            *log.borrow(),
            vec![("allocate", 5 * 32, 0), ("allocate", 8 * 4, 0)]
        );
        assert_eq!(e.mem.u16(this + 0x20), 5);
        assert_eq!(e.mem.u16(this + 0x22), 5);
        assert_ne!(e.mem.u32(this + 0x24), 0);
        assert_ne!(e.mem.u32(this + 0x28), 0);
    }

    #[test]
    fn allocating_zero_leaves_the_pointers_alone() {
        let (mut e, log) = rig();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, 0x1234);
        e.call(0x00d0_1ae0, &args![this, 0u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x24), 0x1234);
        // A count whose low 16 bits are zero still allocates (the test is on the whole word).
        e.call(0x00d0_1ae0, &args![this, 0x1_0000u32]);
        assert_eq!(*log.borrow(), vec![("allocate", 0, 0), ("allocate", 0, 0)]);
        assert_eq!(e.mem.u16(this + 0x20), 0);
    }

    #[test]
    fn deallocate_frees_both_blocks_and_clears_the_fields() {
        let (mut e, log) = rig();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, 0xaaa0);
        e.mem.set_u32(this + 0x28, 0xbbb0);
        e.mem.set_u16(this + 0x20, 6);
        e.mem.set_u16(this + 0x22, 6);
        e.call(0x00d0_1a80, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![("free", 0xaaa0, 6 * 32), ("free", 0xbbb0, 8 * 4)]
        );
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u16(this + 0x20), 0);
        assert_eq!(e.mem.u16(this + 0x22), 0);
    }
}

#[cfg(test)]
mod havok_constructor_tests {
    use super::*;

    #[test]
    fn the_constructor_clears_the_counts_and_pointers_and_returns_this() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        for offset in (0..0x30).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        let got = e.call(0x00d0_1a60, &args![this]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u16(this + 0x20), 0);
        assert_eq!(e.mem.u16(this + 0x22), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        // The other words are left alone.
        assert_eq!(e.mem.u32(this + 4), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x14), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x2c), 0xdead_beef);
    }
}
