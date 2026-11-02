//! `Havok SDK/hkpallcdbodypaircollector.cpp` (Xbox PDB source unit), subsystem `Havok SDK`:
//! its functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bshavok::bhkentity::MEMORY_ROUTER;
use crate::units::bshavok::run_00cb9810::chain_end;

/// The array growth routine (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_GROW_ONE: u32 = 0x00c9_0940;
/// Size of one pair record.
const RECORD_SIZE: u32 = 0x10;

// Translated from 00cb7550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAllCdBodyPairCollector::addCdBodyPair` (Xbox PDB): takes two bodies. The collector's
/// array of 16-byte records is at `+8` (data pointer, size at `+0xC`, capacity in the low 30
/// bits of `+0x10`). Looks up the memory router (`00c85750`), grows the array (`00c90940`,
/// with the router's allocator at `+0x10`) when the size equals the capacity, and appends a
/// record: the last link of the chain of words at `+0xC` from the first body, that body's word
/// at `+4`, and the same two for the second body.
pub fn hkp_all_cd_body_pair_collector_add_cd_body_pair(
    e: &mut Engine,
    this: u32,
    first: u32,
    second: u32,
) {
    let array = this + 8;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW_ONE, &args![allocator, array, RECORD_SIZE]);
    }
    let size = e.mem.u32(array + 4);
    let record = size
        .wrapping_mul(RECORD_SIZE)
        .wrapping_add(e.mem.u32(array));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    let first_root = chain_end(e, first);
    e.mem.set_u32(record, first_root);
    let first_word = e.mem.u32(first + 4);
    e.mem.set_u32(record + 4, first_word);
    let second_root = chain_end(e, second);
    e.mem.set_u32(record + 8, second_root);
    let second_word = e.mem.u32(second + 4);
    e.mem.set_u32(record + 0xc, second_word);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00cb7550,
        hkp_all_cd_body_pair_collector_add_cd_body_pair(u32, u32, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{calls_to, logging_engine};

    /// A collector whose array has room for `capacity` records and holds `size`; the router
    /// double answers an object with an allocator word at `+0x10`.
    fn setup(size: u32, capacity: u32) -> (Engine, u32, u32) {
        let mut e = logging_engine();
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0xa110c);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(ARRAY_GROW_ONE, |e, a| {
            // Moves the data to a larger block and raises the capacity by 4.
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let fresh = e.mem.alloc(16 * (size + 4));
            for word in 0..size * 4 {
                let value = e.mem.u32(old + 4 * word);
                e.mem.set_u32(fresh + 4 * word, value);
            }
            e.mem.set_u32(a[1], fresh);
            e.mem.set_u32(a[1] + 8, size + 4);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        let data = e.mem.alloc(16 * capacity.max(1));
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, size);
        // The flag bit above the capacity must not matter.
        e.mem.set_u32(this + 0x10, capacity | 0x8000_0000);
        (e, this, router)
    }

    /// A body whose `+0xC` chain has `depth` links above it; answers the body and its root.
    fn body(e: &mut Engine, depth: u32, marker: u32) -> (u32, u32) {
        let bottom = e.mem.alloc(0x20);
        e.mem.set_u32(bottom + 4, marker);
        let mut top = bottom;
        for _ in 0..depth {
            let parent = e.mem.alloc(0x20);
            e.mem.set_u32(top + 0xc, parent);
            top = parent;
        }
        (bottom, top)
    }

    #[test]
    fn appends_a_record_with_both_roots() {
        let (mut e, this, _) = setup(1, 4);
        let (first, first_root) = body(&mut e, 2, 0x11);
        let (second, second_root) = body(&mut e, 0, 0x22);
        e.call(0x00cb_7550, &args![this, first, second]);
        let data = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(this + 0xc), 2);
        let record = data + 16;
        assert_eq!(e.mem.u32(record), first_root);
        assert_eq!(e.mem.u32(record + 4), 0x11);
        assert_eq!(e.mem.u32(record + 8), second_root);
        assert_eq!(e.mem.u32(record + 0xc), 0x22);
        assert!(calls_to(&e, ARRAY_GROW_ONE).is_empty());
        assert_eq!(calls_to(&e, MEMORY_ROUTER).len(), 1);
    }

    #[test]
    fn a_full_array_grows_first_and_keeps_the_old_records() {
        let (mut e, this, _) = setup(2, 2);
        let data = e.mem.u32(this + 8);
        e.mem.set_u32(data + 16, 0x9999);
        let (first, first_root) = body(&mut e, 1, 0x33);
        let (second, second_root) = body(&mut e, 1, 0x44);
        e.call(0x00cb_7550, &args![this, first, second]);
        assert_eq!(calls_to(&e, ARRAY_GROW_ONE), [[0xa110c, this + 8, 16]]);
        let grown = e.mem.u32(this + 8);
        assert_ne!(grown, data);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.u32(grown + 16), 0x9999);
        assert_eq!(e.mem.u32(grown + 32), first_root);
        assert_eq!(e.mem.u32(grown + 40), second_root);
        assert_eq!(e.mem.u32(grown + 44), 0x44);
    }
}
