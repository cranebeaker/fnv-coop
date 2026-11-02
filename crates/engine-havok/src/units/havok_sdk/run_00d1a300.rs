//! `Havok SDK/run_00d1a300` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d1a300 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words `items`, `count`): for each of the first `count` pointers of the
/// array at `items` (nothing when `count` is not above 0, signed) sets the word at the object's
/// `+0x30` to 1 and the word at its `+0x40` to 0.
pub fn fn_00d1a300(e: &mut Engine, items: Ptr, count: i32) {
    for index in 0..count.max(0) as u32 {
        let item = e.mem.u32(items.addr().wrapping_add(index.wrapping_mul(4)));
        e.mem.set_u32(item.wrapping_add(0x30), 1);
        e.mem.set_u32(item.wrapping_add(0x40), 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d1_a300, fn_00d1a300(Ptr, i32))]
}

#[cfg(test)]
mod havok_a04_tests {
    use crate::prelude::*;

    #[test]
    fn fn_00d1a300_resets_two_words_of_each_item() {
        let mut e = Engine::new();
        let items = e.mem.alloc(0x10);
        let objects: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x50)).collect();
        for (index, object) in objects.iter().enumerate() {
            e.mem.set_u32(items + index as u32 * 4, *object);
            e.mem.set_u32(object + 0x30, 0xffff_ffff);
            e.mem.set_u32(object + 0x40, 0xffff_ffff);
            e.mem.set_u32(object + 0x34, 0x1234);
        }
        e.call(0x00d1_a300, &args![items, 2i32]);
        for object in &objects[..2] {
            assert_eq!(e.mem.u32(object + 0x30), 1);
            assert_eq!(e.mem.u32(object + 0x40), 0);
            assert_eq!(e.mem.u32(object + 0x34), 0x1234);
        }
        // The third is outside the count.
        assert_eq!(e.mem.u32(objects[2] + 0x30), 0xffff_ffff);
        assert_eq!(e.mem.u32(objects[2] + 0x40), 0xffff_ffff);
    }

    #[test]
    fn fn_00d1a300_does_nothing_for_a_count_that_is_not_positive() {
        let mut e = Engine::new();
        let items = e.mem.alloc(0x10);
        let object = e.mem.alloc(0x50);
        e.mem.set_u32(items, object);
        e.mem.set_u32(object + 0x30, 5);
        e.call(0x00d1_a300, &args![items, 0i32]);
        e.call(0x00d1_a300, &args![items, -4i32]);
        assert_eq!(e.mem.u32(object + 0x30), 5);
    }
}
