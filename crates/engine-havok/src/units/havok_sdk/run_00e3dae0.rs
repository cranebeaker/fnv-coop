//! `Havok SDK/run_00e3dae0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{allocator, release_array, CAPACITY_MASK};

/// `hkpShapeCollection::hkpShapeCollection` (`00d21410`, `this`, two stack words: `0x10` and 6).
const SHAPE_COLLECTION_CONSTRUCT: u32 = 0x00d2_1410;
/// `hkStringPtr` destructor (`00cb0c00`, `this` = the member).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
/// `1.0f` in the exe's data.
const ONE: u32 = 0x0101_2068;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e3deb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e3_dc50`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e3deb0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e3_dc50, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e3dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to `00e3deb0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00e3dc40(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00e3_deb0, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 00e3dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor of a shape collection (`this` in `ECX`, one float
/// stack word `value`, `RET 4`). It runs `hkpShapeCollection::hkpShapeCollection` (`00d21410`,
/// `this`, `0x10`, 6), stores `value` through the x87 stack at `+0x28`, sets the vtable words
/// `010eba5c` at `+0` and `010eba3c` at `+0x10`, the words `0x11`, `0x12`, `0x3ffff` and
/// `0x1ffff` at `+0x18..+0x24`, empties eight `hkArray`-shaped triples (data 0, size 0,
/// capacity and flags `0x80000000`) at `+0x30`, `+0x3c`, `+0x48`, `+0x54`, `+0x60`, `+0x6c`,
/// `+0x78` and `+0x84`, clears `+0xc0`, the 16-bit words at `+0xc4` and `+0xc6`, `+0xc8` and
/// `+0xcc` and sets `+0xd0` to `0x80000000` (a ninth, empty triple at `+0xc8`), the byte at
/// `+0x2c` to 6 and the vector at `+0xe0` to `1.0` in all four lanes. Returns `this` (`EAX`).
/// The class is unconfirmed.
pub fn fn_00e3dae0(e: &mut Engine, this: Ptr, value: f32) -> Ptr {
    let base = this.addr();
    e.call(SHAPE_COLLECTION_CONSTRUCT, &args![this, 0x10u32, 6u32]);
    e.mem.set_u32(base + 0x28, x87_quiet_bits(value.to_bits()));
    e.mem.set_u32(base, 0x010e_ba5c);
    e.mem.set_u32(base + 0x10, 0x010e_ba3c);
    e.mem.set_u32(base + 0x18, 0x11);
    e.mem.set_u32(base + 0x1c, 0x12);
    e.mem.set_u32(base + 0x20, 0x3ffff);
    e.mem.set_u32(base + 0x24, 0x1ffff);
    let one: u32 = e.global(ONE);
    for array in [0x30u32, 0x3c, 0x48, 0x54, 0x60, 0x6c, 0x78, 0x84] {
        e.mem.set_u32(base + array, 0);
        e.mem.set_u32(base + array + 4, 0);
        e.mem.set_u32(base + array + 8, 0x8000_0000);
    }
    e.mem.set_u32(base + 0xc0, 0);
    e.mem.set_u16(base + 0xc4, 0);
    e.mem.set_u16(base + 0xc6, 0);
    e.mem.set_u32(base + 0xc8, 0);
    e.mem.set_u32(base + 0xcc, 0);
    e.mem.set_u32(base + 0xd0, 0x8000_0000);
    e.mem.set_u8(base + 0x2c, 6);
    for lane in 0..4 {
        e.mem.set_u32(base + 0xe0 + 4 * lane, one);
    }
    this
}

// Translated from 00e3dbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a constructor chain that the compiler collapsed (`this` in `ECX`,
/// one stack word `mode`). The vtable word at `+0` takes three values in turn (`010305a8`,
/// `010cc364`, `010eba5c`) and the word at `+0x10` three (`010c6fb0`, `010cc344`, `010eba3c`);
/// when `mode` is not zero the stages also write `+0xc` (0, 7, `0x10`) and the byte at `+0x15`
/// (3, 6), and the last stage copies the word at `+0xc8` to `+0xc0` when the word at `+0xcc` is
/// not zero. Returns `this` (`EAX`). The class is unconfirmed.
pub fn fn_00e3dbd0(e: &mut Engine, this: Ptr, mode: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x010c_c364);
    e.mem.set_u32(base + 0x10, 0x010c_c344);
    if mode != 0 {
        e.mem.set_u32(base + 0xc, 7);
        e.mem.set_u8(base + 0x15, 3);
    }
    e.mem.set_u32(base, 0x010e_ba5c);
    e.mem.set_u32(base + 0x10, 0x010e_ba3c);
    if mode != 0 {
        let has_extra = e.mem.u32(base + 0xcc) != 0;
        e.mem.set_u32(base + 0xc, 0x10);
        e.mem.set_u8(base + 0x15, 6);
        if has_extra {
            let value = e.mem.u32(base + 0xc8);
            e.mem.set_u32(base + 0xc0, value);
        }
    }
    this
}

/// The second half of an inlined `hkArray::clearAndDeallocate`, with the allocator `heap`
/// fetched earlier: the size is cleared; unless the capacity word is negative the storage goes
/// back through the allocator's slot `0x10` with `(capacity & 0x3fffffff) * element_size`
/// bytes; then data is cleared and the capacity word becomes `0x80000000`.
fn clear_with(e: &mut Engine, heap: u32, array: u32, element_size: u32) {
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            heap,
            0x10,
            &args![data, (capacity & CAPACITY_MASK).wrapping_mul(element_size)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00e3dc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a destructor (`this` in `ECX`; the body of the deleting
/// destructor `00e3deb0`). Nine `hkArray`-shaped triples are emptied in this order: `+0xc8`
/// (8-byte elements: first every element's `hkStringPtr` at `+4` is destroyed with `00cb0c00`),
/// `+0x84` (0x40-byte elements, each destroyed with `00dfd440`), `+0x78` (0x50-byte elements,
/// each destroyed with `00dfd380`), then `+0x6c` (16 bytes), `+0x60` (16), `+0x54` (64), `+0x48`
/// (1), `+0x3c` (2) and `+0x30` (4). For the first three the allocator is fetched before the
/// element destructors run, and the element count and data pointer are read once. Then the
/// vtable word `+0x10` is set to `010c6fb0` and `+0` to `0102e388`. The class is unconfirmed.
pub fn fn_00e3dc50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let heap = allocator(e);
    let count = e.mem.u32(base + 0xcc) as i32;
    let data = e.mem.u32(base + 0xc8);
    if count > 0 {
        let mut member = data.wrapping_add(4);
        for _ in 0..count {
            e.call(STRING_PTR_DESTRUCT, &args![member]);
            member = member.wrapping_add(8);
        }
    }
    clear_with(e, heap, base + 0xc8, 8);
    let heap = allocator(e);
    let count = e.mem.u32(base + 0x88) as i32;
    let data = e.mem.u32(base + 0x84);
    if count > 0 {
        let mut element = data;
        for _ in 0..count {
            e.call(0x00df_d440, &args![element]);
            element = element.wrapping_add(0x40);
        }
    }
    clear_with(e, heap, base + 0x84, 0x40);
    let heap = allocator(e);
    let count = e.mem.u32(base + 0x7c) as i32;
    let data = e.mem.u32(base + 0x78);
    if count > 0 {
        let mut element = data;
        for _ in 0..count {
            e.call(0x00df_d380, &args![element]);
            element = element.wrapping_add(0x50);
        }
    }
    clear_with(e, heap, base + 0x78, 0x50);
    release_array(e, base + 0x6c, 16);
    release_array(e, base + 0x60, 16);
    release_array(e, base + 0x54, 64);
    release_array(e, base + 0x48, 1);
    release_array(e, base + 0x3c, 2);
    release_array(e, base + 0x30, 4);
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_deb0, deleting_destructor_00e3deb0(Ptr, u8) -> Ptr),
        entry!(0x00e3_dc40, fn_00e3dc40(Ptr, u32) -> u32),
        entry!(0x00e3_dae0, fn_00e3dae0(Ptr, f32) -> Ptr),
        entry!(0x00e3_dbd0, fn_00e3dbd0(Ptr, u32) -> Ptr),
        entry!(0x00e3_dc50, fn_00e3dc50(Ptr)),
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
    fn deleting_destructor_00e3deb0_frees_only_when_bit_zero_is_set() {
        check(0x00e3_deb0, 0x00e3_dc50);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00e3dc40() {
        assert_eq!(check(0x00e3_dc40, 0x00e3_deb0, 0x10), 0xdead_beef);
    }
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT: u32 = 0x0900_0011;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// The base constructor double sees nothing written yet; the other fields and the final
    /// values are checked afterwards.
    #[test]
    fn fn_00e3dae0_constructs_the_collection() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(0x0101_2068, 0x3f80_0000u32);
        let this = e.mem.alloc(0x100);
        e.mem.write(this, &[0x55; 0x100]);
        e.register_double(0x00d2_1410, move |e, a| {
            assert_eq!(a, [this, 0x10, 6]);
            assert_eq!(e.mem.u32(this + 0x28), 0x5555_5555);
            Ret::default()
        });
        assert_eq!(
            e.call(0x00e3_dae0, &args![this, f32::from_bits(0x7f80_0001)])
                .u32(),
            this
        );
        assert_eq!(e.mem.u32(this + 0x28), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this), 0x010e_ba5c);
        assert_eq!(e.mem.u32(this + 0x10), 0x010e_ba3c);
        assert_eq!(e.mem.u32(this + 0x18), 0x11);
        assert_eq!(e.mem.u32(this + 0x1c), 0x12);
        assert_eq!(e.mem.u32(this + 0x20), 0x3ffff);
        assert_eq!(e.mem.u32(this + 0x24), 0x1ffff);
        assert_eq!(e.mem.u8(this + 0x2c), 6);
        assert_eq!(e.mem.u8(this + 0x2d), 0x55);
        for array in [0x30u32, 0x3c, 0x48, 0x54, 0x60, 0x6c, 0x78, 0x84, 0xc8] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0xc0), 0);
        assert_eq!(e.mem.u16(this + 0xc4), 0);
        assert_eq!(e.mem.u16(this + 0xc6), 0);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(this + 0xe0 + 4 * lane), 1.0);
        }
        assert_eq!(e.mem.u32(this + 0x90), 0x5555_5555);
        assert_eq!(e.mem.u32(this + 0xdc), 0x5555_5555);
    }

    /// Mode 0 leaves the stage values out; a mode adds them, and +0xc0 follows +0xc8 only when
    /// +0xcc is set.
    #[test]
    fn fn_00e3dbd0_stages_depend_on_the_mode() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xe0);
        e.mem.write(this, &[0x66; 0xe0]);
        assert_eq!(e.call(0x00e3_dbd0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010e_ba5c);
        assert_eq!(e.mem.u32(this + 0x10), 0x010e_ba3c);
        assert_eq!(e.mem.u32(this + 0xc), 0x6666_6666);
        assert_eq!(e.mem.u8(this + 0x15), 0x66);
        e.mem.set_u32(this + 0xcc, 0);
        e.mem.set_u32(this + 0xc8, 0x1234);
        e.call(0x00e3_dbd0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x10);
        assert_eq!(e.mem.u8(this + 0x15), 6);
        assert_eq!(e.mem.u32(this + 0xc0), 0x6666_6666);
        e.mem.set_u32(this + 0xcc, 1);
        e.call(0x00e3_dbd0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this + 0xc0), 0x1234);
    }

    fn set_array(e: &mut Engine, array: u32, data: u32, size: u32, capacity: u32) {
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity);
    }

    /// Nine arrays: the three with element destructors (two elements of 8 bytes, one of 0x40
    /// and two of 0x50), then six plain ones, one of them not owned.
    #[test]
    fn fn_00e3dc50_destroys_the_elements_and_frees_every_array() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [FREE_SLOT, 0x00cb_0c00, 0x00df_d440, 0x00df_d380] {
            let calls = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                calls.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0xe0);
        set_array(&mut e, this + 0xc8, 0xa000, 2, 3);
        set_array(&mut e, this + 0x84, 0xb000, 1, 2);
        set_array(&mut e, this + 0x78, 0xc000, 2, 1);
        set_array(&mut e, this + 0x6c, 0xd000, 1, 2);
        set_array(&mut e, this + 0x60, 0xe000, 1, 0x8000_0001);
        set_array(&mut e, this + 0x54, 0xf000, 1, 1);
        set_array(&mut e, this + 0x48, 0x1100, 1, 5);
        set_array(&mut e, this + 0x3c, 0x1200, 1, 3);
        set_array(&mut e, this + 0x30, 0x1300, 1, 2);
        e.call(0x00e3_dc50, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x00cb_0c00, vec![0xa004]),
                (0x00cb_0c00, vec![0xa00c]),
                (FREE_SLOT, vec![allocator, 0xa000, 24]),
                (0x00df_d440, vec![0xb000]),
                (FREE_SLOT, vec![allocator, 0xb000, 128]),
                (0x00df_d380, vec![0xc000]),
                (0x00df_d380, vec![0xc050]),
                (FREE_SLOT, vec![allocator, 0xc000, 0x50]),
                (FREE_SLOT, vec![allocator, 0xd000, 32]),
                (FREE_SLOT, vec![allocator, 0xf000, 64]),
                (FREE_SLOT, vec![allocator, 0x1100, 5]),
                (FREE_SLOT, vec![allocator, 0x1200, 6]),
                (FREE_SLOT, vec![allocator, 0x1300, 8]),
            ]
        );
        for array in [0xc8u32, 0x84, 0x78, 0x6c, 0x60, 0x54, 0x48, 0x3c, 0x30] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_6fb0);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }
}
