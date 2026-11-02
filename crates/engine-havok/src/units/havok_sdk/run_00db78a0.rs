//! `Havok SDK/run_00db78a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00db78a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall `(this, array, values)` (`RET 8`): walks the zero-terminated list of object pointers
/// `array` next to the list `values`; for every object whose word at `+0xc` is not zero it calls
/// `this`'s virtual slot `+0xc` with that word and the matching value. The walk ends at the
/// first zero value as well as at the first null object pointer. The exe's name for it is
/// unknown.
pub fn fn_00db78a0(e: &mut Engine, this: u32, array: u32, values: u32) {
    if e.mem.u32(array) == 0 {
        return;
    }
    let mut at = array;
    let offset = values.wrapping_sub(array);
    loop {
        let value = e.mem.u32(at.wrapping_add(offset));
        if value == 0 {
            return;
        }
        let first = e.mem.u32(e.mem.u32(at) + 0xc);
        if first != 0 {
            e.vcall(this, 0xc, &args![first, value]);
        }
        at += 4;
        if e.mem.u32(at) == 0 {
            return;
        }
    }
}

/// Allocates `size` bytes through the thread's memory router (`00c85750`): the allocator at
/// `+0x10`, whose virtual slot `4` hands out the block; the size goes into the 16-bit word at
/// `+4`.
fn allocate_sized_block(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    let block = e.vcall(allocator, 4, &args![size]).u32();
    e.mem.set_u16(block.wrapping_add(4), size as u16);
    block
}

// Translated from 00db7960 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, no words; no direct caller; descriptions, not confirmed): allocates a
/// 0x14-byte object (`allocate_sized_block`), sets the 16-bit word at `+6` to 1 and the vtable
/// `010cf844`, makes the `hkArray` at `+8` empty (data 0, size 0, capacity word `0xffffffff`) and
/// runs `00cb1fe0` on it (`thiscall`, `(allocator, 0)`: the memory router's allocator and 0).
/// Answers the object.
pub fn fn_00db7960(e: &mut Engine) -> u32 {
    let object = allocate_sized_block(e, 0x14);
    e.mem.set_u16(object.wrapping_add(6), 1);
    e.mem.set_u32(object, 0x010c_f844);
    let array = object.wrapping_add(8);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array.wrapping_add(4), 0);
    e.mem.set_u32(array.wrapping_add(8), 0xffff_ffff);
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(0x10));
    e.call(0x00cb_1fe0, &args![array, allocator, 0u32]);
    object
}

// Translated from 00db79c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, no words; no direct caller; descriptions, not confirmed): allocates a
/// 0x20-byte object (`allocate_sized_block`), sets the 16-bit word at `+6` to 1, the vtable
/// `010cf7d4`, the word at `+8` to `010cf830`, runs the constructor `00d87790` on the member at
/// `+0xc` (`thiscall`, one stack word the callee never reads: the exe passes the low byte 0 of a
/// stack slot, here 0) and then sets the vtable `010cf804`. Answers the object.
pub fn fn_00db79c0(e: &mut Engine) -> u32 {
    let object = allocate_sized_block(e, 0x20);
    e.mem.set_u16(object.wrapping_add(6), 1);
    e.mem.set_u32(object, 0x010c_f7d4);
    e.mem.set_u32(object.wrapping_add(8), 0x010c_f830);
    e.call(0x00d8_7790, &args![object.wrapping_add(0xc), 0u32]);
    e.mem.set_u32(object, 0x010c_f804);
    object
}

// Translated from 00db7a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, no words; no direct caller; descriptions, not confirmed): allocates a
/// 0x1c-byte object (`allocate_sized_block`), sets the 16-bit word at `+6` to 1, the vtable
/// `010cf7b0`, runs the constructor `00d87790` on the member at `+8` (`thiscall`, one stack word
/// the callee never reads: the exe passes the low byte 0 of a stack slot, here 0) and sets the
/// word at `+0x18` to 1. Answers the object.
pub fn fn_00db7a20(e: &mut Engine) -> u32 {
    let object = allocate_sized_block(e, 0x1c);
    e.mem.set_u16(object.wrapping_add(6), 1);
    e.mem.set_u32(object, 0x010c_f7b0);
    e.call(0x00d8_7790, &args![object.wrapping_add(8), 0u32]);
    e.mem.set_u32(object.wrapping_add(0x18), 1);
    object
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db7960, fn_00db7960() -> u32),
        entry!(0x00db79c0, fn_00db79c0() -> u32),
        entry!(0x00db7a20, fn_00db7a20() -> u32),
        entry!(0x00db_78a0, fn_00db78a0(u32, u32, u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;

    /// The memory router (`00c85750`) answers an object whose `+0x10` is an allocator; the
    /// allocator's slot 4 hands out `block` and records the size it was asked for. Returns the
    /// sizes and the allocator.
    fn install_allocator(
        e: &mut Engine,
        block: u32,
    ) -> (std::rc::Rc<std::cell::RefCell<Vec<u32>>>, u32) {
        let sizes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = std::rc::Rc::clone(&sizes);
        e.map(block, 0x100);
        e.put_vtable(0x0900_0000, &[0, 0x0900_0100]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x0900_0000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(0x0900_0100, move |_, a| {
            seen.borrow_mut().push(a[1]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        (sizes, allocator)
    }

    fn calls_of_00db7960(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00db7960_builds_the_object_with_an_empty_array() {
        let mut e = Engine::new();
        let (sizes, allocator) = install_allocator(&mut e, 0x0800_0000);
        e.register(0x00cb_1fe0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let object = e.call(0x00db_7960, &args![]).u32();
        assert_eq!(object, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x14]);
        assert_eq!(e.mem.u16(object + 4), 0x14);
        assert_eq!(e.mem.u16(object + 6), 1);
        assert_eq!(e.mem.u32(object), 0x010c_f844);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 0x10), 0xffff_ffff);
        assert_eq!(
            calls_of_00db7960(&e, 0x00cb_1fe0),
            vec![vec![object + 8, allocator, 0]]
        );
    }

    #[test]
    fn fn_00db79c0_builds_the_object_and_constructs_its_member() {
        let mut e = Engine::new();
        let (sizes, _) = install_allocator(&mut e, 0x0800_0000);
        // The member constructor sees the first vtable stores already made.
        e.register(0x00d8_7790, |e, a| {
            let object = a[0] - 0xc;
            assert_eq!(e.mem.u32(object), 0x010c_f7d4);
            assert_eq!(e.mem.u32(object + 8), 0x010c_f830);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let object = e.call(0x00db_79c0, &args![]).u32();
        assert_eq!(object, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x20]);
        assert_eq!(e.mem.u16(object + 4), 0x20);
        assert_eq!(e.mem.u16(object + 6), 1);
        assert_eq!(e.mem.u32(object), 0x010c_f804);
        assert_eq!(
            calls_of_00db7960(&e, 0x00d8_7790),
            vec![vec![object + 0xc, 0]]
        );
    }

    #[test]
    fn fn_00db7a20_builds_the_object_and_constructs_its_member() {
        let mut e = Engine::new();
        let (sizes, _) = install_allocator(&mut e, 0x0800_0000);
        e.register(0x00d8_7790, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let object = e.call(0x00db_7a20, &args![]).u32();
        assert_eq!(object, 0x0800_0000);
        assert_eq!(*sizes.borrow(), vec![0x1c]);
        assert_eq!(e.mem.u16(object + 4), 0x1c);
        assert_eq!(e.mem.u16(object + 6), 1);
        assert_eq!(e.mem.u32(object), 0x010c_f7b0);
        assert_eq!(e.mem.u32(object + 0x18), 1);
        assert_eq!(
            calls_of_00db7960(&e, 0x00d8_7790),
            vec![vec![object + 8, 0]]
        );
    }

    const SLOT: u32 = 0x0900_000c;

    fn target(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SLOT);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, vtable);
        e.register(SLOT, |_, _| Ret::default());
        this
    }

    /// Objects with the given words at `+0xc`, as a zero-terminated pointer list.
    fn objects(e: &mut Engine, words: &[u32]) -> u32 {
        let array = e.mem.alloc(4 * (words.len() as u32 + 1));
        for (i, word) in words.iter().enumerate() {
            let object = e.mem.alloc(0x20);
            e.mem.set_u32(object + 0xc, *word);
            e.mem.set_u32(array + 4 * i as u32, object);
        }
        array
    }

    fn values(e: &mut Engine, items: &[u32]) -> u32 {
        let at = e.mem.alloc(4 * (items.len() as u32 + 1));
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, *item);
        }
        at
    }

    fn slot_calls(e: &Engine) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == SLOT)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn every_object_with_a_word_is_told_its_value() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = target(&mut e);
        let array = objects(&mut e, &[0xa1, 0, 0xa3]);
        let list = values(&mut e, &[1, 2, 3]);
        e.call(0x00db_78a0, &args![this, array, list]);
        assert_eq!(
            slot_calls(&e),
            vec![vec![this, 0xa1, 1], vec![this, 0xa3, 3]]
        );
    }

    #[test]
    fn a_zero_value_ends_the_walk() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = target(&mut e);
        let array = objects(&mut e, &[0xa1, 0xa2, 0xa3]);
        let list = values(&mut e, &[1, 0, 3]);
        e.call(0x00db_78a0, &args![this, array, list]);
        assert_eq!(slot_calls(&e), vec![vec![this, 0xa1, 1]]);
    }

    #[test]
    fn an_empty_list_calls_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let this = target(&mut e);
        let array = objects(&mut e, &[]);
        let list = values(&mut e, &[1]);
        e.call(0x00db_78a0, &args![this, array, list]);
        assert!(slot_calls(&e).is_empty());
    }

    /// A memory router at `0x7000` whose `+0x10` allocator hands out `block` for `size`.
    fn creator_engine(block: u32, size: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x0000_7000, 0x1000);
        e.mem.set_u32(0x7010, 0x7100);
        e.mem.set_u32(0x7100, 0x7200);
        e.mem.set_u32(0x7204, 0x00ab_0004);
        e.register(0x00c8_5750, |_, _| 0x7000u32.into_ret());
        e.register_double(0x00ab_0004, move |_, a| {
            assert_eq!(a, [0x7100, size]);
            block.into_ret()
        });
        e
    }

    #[test]
    fn the_class_registries_are_built_with_their_tables_and_maps() {
        let mut e = creator_engine(0x7800, 0x14);
        e.register(0x00cb_1fe0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00db_7960, &args![]).u32(), 0x7800);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(0x7800), 0x010c_f844);
        assert_eq!(e.mem.u16(0x7804), 0x14);
        assert_eq!(e.mem.u32(0x7810), 0xffff_ffff);
        let reserve: Vec<_> = log.iter().filter(|(a, _)| *a == 0x00cb_1fe0).collect();
        assert_eq!(reserve[0].1, [0x7808, 0x7100, 0]);

        let mut e = creator_engine(0x7800, 0x20);
        e.register(0x00d8_7790, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x00db_79c0, &args![]).u32(), 0x7800);
        assert_eq!(e.mem.u32(0x7800), 0x010c_f804, "the final table");
        assert_eq!(e.mem.u32(0x7808), 0x010c_f830);
        assert_eq!(e.mem.u16(0x7804), 0x20);

        let mut e = creator_engine(0x7800, 0x1c);
        e.register(0x00d8_7790, |_, a| a[0].into_ret());
        assert_eq!(e.call(0x00db_7a20, &args![]).u32(), 0x7800);
        assert_eq!(e.mem.u32(0x7800), 0x010c_f7b0);
        assert_eq!(e.mem.u32(0x7818), 1);
    }
}
