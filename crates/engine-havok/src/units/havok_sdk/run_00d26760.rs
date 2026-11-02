//! `Havok SDK/run_00d26760` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d26880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_6760`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d26880(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_6760, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d26870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `00d26880`. `00d26880` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d26870(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d2_6880, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The pointer-sized object at `00c905b0` the destructor releases the word at `+0x18` with
/// (thiscall on that word, no stack words).
const RELEASE_HELD: u32 = 0x00c9_05b0;

/// Frees the `hkArray` of 16-bit elements (data at `+0x24`, size `+0x28`, capacity word `+0x2c`)
/// of `base`: after `00c85750`, sets the size to 0 and, when the capacity word's sign bit is
/// clear, frees the data (`(capacity & 0x3fffffff) * 2` bytes) through slot `0x10` of the
/// router's allocator (`+0x10`); then clears the data pointer and makes the capacity word
/// `0x80000000`.
fn free_words_00d26760(e: &mut Engine, base: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x2c);
    e.mem.set_u32(base + 0x28, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 0x24);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(2);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x2c, 0x8000_0000);
}

// Translated from 00d26760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the class at the vtable `010ce390` (thiscall, no stack words; the engine map has
/// no name, the class is the one `00d26880` deletes): writes the vtable words `+0` =
/// `0x010c_e390` and `+0x10` = `0x010c_e370`, calls `00c905b0` on the word at `+0x18`, frees the
/// 16-bit array at `+0x24` and writes the base vtable words (`+0x10` = `0x010c_6fb0`, `+0` =
/// `0x0102_e388`).
pub fn fn_00d26760(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let held = e.mem.u32(base + 0x18);
    e.mem.set_u32(base, 0x010c_e390);
    e.mem.set_u32(base + 0x10, 0x010c_e370);
    e.call(RELEASE_HELD, &args![held]);
    free_words_00d26760(e, base);
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00d267c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x2c` of the vtable `010ce390` (thiscall, one stack word `mode`; the class is the one
/// `00d26760` destroys). With `mode == 6` the 16-bit array at `+0x24` is freed as in the
/// destructor. Otherwise `00d26450` (called on `this`) answers a shape whose `+0x10` and `+0x14`
/// are the width `w` and depth `d`; the array is sized to `(2 * d - 2) * (w - 1)` elements
/// (signed product): reserved through `00c908c0` with exactly that count when its capacity is
/// below it, then again with `max(count, 2 * capacity)` when it still is, and the size at `+0x28`
/// is set to the count (the elements are not initialised).
pub fn fn_00d267c0(e: &mut Engine, this: Ptr, mode: u32) {
    let me = this.addr();
    if mode == 6 {
        free_words_00d26760(e, me);
        return;
    }
    let shape = e.call(0x00d2_6450, &args![this]).u32();
    let depth = e.mem.u32(shape + 0x14);
    let width = e.mem.u32(shape + 0x10);
    let count = (depth.wrapping_add(depth).wrapping_sub(2) as i32)
        .wrapping_mul(width.wrapping_sub(1) as i32);
    let array = me + 0x24;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE, &args![allocator, array, count, 2u32]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 2u32]);
    }
    e.mem.set_i32(array + 4, count);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_6880, deleting_destructor_00d26880(Ptr, u8) -> Ptr),
        entry!(0x00d2_6870, fn_00d26870(Ptr, u32) -> u32),
        entry!(0x00d2_6760, fn_00d26760(Ptr)),
        entry!(0x00d2_67c0, fn_00d267c0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d26880_frees_only_when_bit_zero_is_set() {
        check(0x00d2_6880, 0x00d2_6760);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00d26870() {
        assert_eq!(
            check(0x00d2_6870, 0x00d2_6880, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    /// A router whose allocator frees through a double at `0x0900_0051`.
    fn install_router(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, 0x0900_0051);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(0x0900_0051, |_, _| Ret::default());
        allocator
    }

    #[test]
    fn fn_00d26760_releases_and_frees_the_array() {
        for (capacity, freed) in [(0x4000_0005u32, true), (0x8000_0005, false)] {
            let mut e = Engine::new();
            let allocator = install_router(&mut e);
            e.register(RELEASE_HELD, |_, _| Ret::default());
            let this = e.mem.alloc(0x40);
            e.mem.set_u32(this + 0x18, 0x1111);
            e.mem.set_u32(this + 0x24, 0x7000);
            e.mem.set_u32(this + 0x28, 9);
            e.mem.set_u32(this + 0x2c, capacity);
            e.call_log = Some(vec![]);
            e.call(0x00d2_6760, &args![this]);
            let log = e.call_log.take().unwrap();
            let mut expected = vec![(RELEASE_HELD, vec![0x1111]), (MEMORY_ROUTER, vec![])];
            if freed {
                expected.push((0x0900_0051, vec![allocator, 0x7000, 10]));
            }
            assert_eq!(log[1..], expected[..]);
            assert_eq!(e.mem.u32(this + 0x24), 0);
            assert_eq!(e.mem.u32(this + 0x28), 0);
            assert_eq!(e.mem.u32(this + 0x2c), 0x8000_0000);
            assert_eq!(e.mem.u32(this + 0x10), 0x010c_6fb0);
            assert_eq!(e.mem.u32(this), 0x0102_e388);
        }
    }

    /// A shape of the given width and depth, and a reserve double that records its calls and
    /// gives the array the wanted capacity.
    fn install_shape(e: &mut Engine, width: u32, depth: u32) {
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0x10, width);
        e.mem.set_u32(shape + 0x14, depth);
        e.register_double(0x00d2_6450, move |_, _| Ret {
            eax: shape,
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            e.mem.set_u32(a[1] + 8, a[2] | 0x8000_0000);
            Ret::default()
        });
    }

    #[test]
    fn fn_00d267c0_sizes_the_array_to_the_cell_triangles() {
        let mut e = Engine::new();
        let allocator = install_router(&mut e);
        install_shape(&mut e, 5, 4);
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d2_67c0, &args![this, 3u32]);
        let log = e.call_log.take().unwrap();
        // (2 * 4 - 2) * (5 - 1) = 24 elements of 2 bytes.
        let reserves: Vec<_> = log.iter().filter(|(a, _)| *a == ARRAY_RESERVE).collect();
        assert_eq!(reserves.len(), 1);
        assert_eq!(reserves[0].1, vec![allocator, this + 0x24, 24, 2]);
        assert_eq!(e.mem.u32(this + 0x28), 24);
        assert_eq!(e.mem.u32(this + 0x2c) & 0x3fff_ffff, 24);
        // Enough capacity: no reserve.
        e.call_log = Some(vec![]);
        e.call(0x00d2_67c0, &args![this, 3u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != ARRAY_RESERVE));
    }

    #[test]
    fn fn_00d267c0_reserves_again_when_the_first_did_not_grow_the_array() {
        let mut e = Engine::new();
        let allocator = install_router(&mut e);
        install_shape(&mut e, 3, 3);
        // A reserve that gives only 2 words, so the second check still asks for max(8, 2 * 2).
        e.register(ARRAY_RESERVE, |e, a| {
            e.mem.set_u32(a[1] + 8, 2);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00d2_67c0, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        let reserves: Vec<_> = log.iter().filter(|(a, _)| *a == ARRAY_RESERVE).collect();
        // (2 * 3 - 2) * (3 - 1) = 8 elements, then max(8, 4) = 8 again.
        assert_eq!(reserves.len(), 2);
        assert_eq!(reserves[0].1, vec![allocator, this + 0x24, 8, 2]);
        assert_eq!(reserves[1].1, vec![allocator, this + 0x24, 8, 2]);
        assert_eq!(e.mem.u32(this + 0x28), 8);
    }

    #[test]
    fn fn_00d267c0_with_mode_6_frees_the_array() {
        let mut e = Engine::new();
        let allocator = install_router(&mut e);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, 0x7000);
        e.mem.set_u32(this + 0x28, 9);
        e.mem.set_u32(this + 0x2c, 6);
        e.call_log = Some(vec![]);
        e.call(0x00d2_67c0, &args![this, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (MEMORY_ROUTER, vec![]),
                (0x0900_0051, vec![allocator, 0x7000, 12])
            ]
        );
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0x8000_0000);
    }
}
