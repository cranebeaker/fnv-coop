//! `Havok SDK/run_00cf56f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00cf5890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_5750`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cf5890(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_5750, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00cf5830 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (slot of the vtable at `010cc3b8`, no map name): the number at +0xC
/// minus the unsigned 16-bit number at +0x16.
pub fn fn_00cf5830(e: &mut Engine, this: Ptr) -> i32 {
    let count = e.mem.i32(this.addr() + 0xc);
    count.wrapping_sub(i32::from(e.mem.u16(this.addr() + 0x16)))
}

// Translated from 00cf5840 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (no map name): the index of the next set bit after `index` in the
/// 256-bit set at +0x40, or `-1` when none is below the count at +0xC. An index of 256 or
/// more (or a negative one, compared as unsigned) that is still below the count is
/// returned as it is.
pub fn fn_00cf5840(e: &mut Engine, this: Ptr, index: i32) -> i32 {
    let count = e.mem.i32(this.addr() + 0xc);
    let mut next = index.wrapping_add(1);
    while next < count {
        if next as u32 >= 0x100 {
            return next;
        }
        let word = e.mem.u32(this.addr() + 0x40 + (next as u32 >> 5) * 4);
        if word & (1u32 << (next & 0x1f)) != 0 {
            return next;
        }
        next += 1;
    }
    -1
}

// Translated from 00cf5880 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual method (no map name): `00cf5840(this, -1)`, the first set bit.
pub fn fn_00cf5880(e: &mut Engine, this: Ptr) -> i32 {
    fn_00cf5840(e, this, -1)
}

// Translated from 00cf5820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to `00cf5890`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00cf5820(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00cf_5890, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

/// `00c905b0` (`this` only): run on the first word of every child entry by `00cf5750`.
const RELEASE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00cf56f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor body of the list shape (vtable chain: `010305a8`, then `010cc364`/`010cc344`, then
/// the final `010cc3d8`/`010cc3b8`; the word at `+0x10` first holds `010c6fb0`). The one stack
/// word is a flag: when it is not 0 the number at `+0xc` is set to 0, then 7 (with the byte at
/// `+0x15` set to 3), then 9 (the byte cleared). Returns `this`. The class is unconfirmed.
pub fn fn_00cf56f0(e: &mut Engine, this: Ptr, flag: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0x0103_05a8);
    if flag != 0 {
        e.mem.set_u32(base + 0xc, 0);
    }
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x010c_c364);
    e.mem.set_u32(base + 0x10, 0x010c_c344);
    if flag != 0 {
        e.mem.set_u32(base + 0xc, 7);
        e.mem.set_u8(base + 0x15, 3);
    }
    e.mem.set_u32(base, 0x010c_c3d8);
    e.mem.set_u32(base + 0x10, 0x010c_c3b8);
    if flag != 0 {
        e.mem.set_u32(base + 0xc, 9);
        e.mem.set_u8(base + 0x15, 0);
    }
    this
}

// Translated from 00cf5750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the list shape: writes the final vtables (`010cc3d8` at `+0`, `010cc3b8`
/// at `+0x10`), runs `00c905b0` on the first word of each of the `+0x1c` (signed) child entries of
/// 16 bytes at `+0x18` (the count is read again for every entry; a null word is passed on too),
/// then frees the child array (the memory router allocator, slot `0x10`, `(capacity &
/// 0x3fffffff) * 16` bytes, unless the capacity word at `+0x20` says the memory is not owned),
/// clears the data pointer, sets the capacity word to `0x80000000`, and writes the base tables
/// (`010c6fb0` at `+0x10`, `0102e388` at `+0`).
pub fn fn_00cf5750(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010c_c3d8);
    e.mem.set_u32(base + 0x10, 0x010c_c3b8);
    let mut index = 0;
    if e.mem.i32(base + 0x1c) > 0 {
        let mut offset = 0u32;
        loop {
            let data = e.mem.u32(base + 0x18);
            let shape = e.mem.u32(data.wrapping_add(offset));
            e.call(RELEASE_REFERENCE, &args![shape]);
            index += 1;
            offset = offset.wrapping_add(0x10);
            if index >= e.mem.i32(base + 0x1c) {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x20);
    e.mem.set_u32(base + 0x1c, 0);
    if capacity as i32 >= 0 {
        let data = e.mem.u32(base + 0x18);
        e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 4]);
    }
    e.mem.set_u32(base + 0x18, 0);
    e.mem.set_u32(base + 0x20, 0x8000_0000);
    e.mem.set_u32(base + 0x10, 0x010c_6fb0);
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00cf57d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The deleting-destructor thunk of the container table (`this` is the container at `+0x10` of
/// the shape): with `shape = this - 0x10` it writes the vtable words `010c6fb0` at `shape +
/// 0x10` and `0102e388` at `shape`, and when bit 0 of `flags` is set frees the block through the
/// memory router allocator (slot `8`) with the 16-bit size kept at `shape + 4`. Returns `shape`.
pub fn fn_00cf57d0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let shape = Ptr::new(this.addr().wrapping_sub(0x10));
    e.mem.set_u32(shape.addr() + 0x10, 0x010c_6fb0);
    e.mem.set_u32(shape.addr(), 0x0102_e388);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = u32::from(e.mem.u16(shape.addr() + 4));
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![shape, size]);
    }
    shape
}

// Translated from 00cf57e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; 00cf57e0 to the `RET 4` at 00cf580f, inside the extent the
/// map gives `00cf57d0`): the deleting destructor of the shape itself (vtable `0102e388`), the
/// body [`fn_00cf57d0`] reaches after its `SUB ECX, 0x10`. `thiscall` with one stack word:
/// writes `010c6fb0` at `+0x10` and `0102e388` at `+0`, and with bit 0 of the word set frees the
/// block through the memory router's allocator (slot `8`) with the 16-bit size at `+4`. Answers
/// `this`.
pub fn fn_00cf57e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr() + 0x10, 0x010c_6fb0);
    e.mem.set_u32(this.addr(), 0x0102_e388);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let size = u32::from(e.mem.u16(this.addr() + 4));
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_5830, fn_00cf5830(Ptr) -> i32),
        entry!(0x00cf_5840, fn_00cf5840(Ptr, i32) -> i32),
        entry!(0x00cf_5880, fn_00cf5880(Ptr) -> i32),
        entry!(0x00cf_5890, deleting_destructor_00cf5890(Ptr, u8) -> Ptr),
        entry!(0x00cf_5820, fn_00cf5820(Ptr, u32) -> u32),
        entry!(0x00cf_56f0, fn_00cf56f0(Ptr, u32) -> Ptr),
        entry!(0x00cf_5750, fn_00cf5750(Ptr)),
        entry!(0x00cf_57d0, fn_00cf57d0(Ptr, u8) -> Ptr),
        entry!(0x00cf_57e0, fn_00cf57e0(Ptr, u8) -> Ptr),
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

    fn bit_set(e: &mut Engine, count: i32, bits: &[u32]) -> u32 {
        let this = e.mem.alloc(0x80);
        e.mem.set_i32(this + 0xc, count);
        for bit in bits {
            let at = this + 0x40 + (bit >> 5) * 4;
            let word = e.mem.u32(at);
            e.mem.set_u32(at, word | (1 << (bit & 31)));
        }
        this
    }

    #[test]
    fn count_minus_the_16_bit_number() {
        let mut e = Engine::new();
        let this = bit_set(&mut e, 10, &[]);
        e.mem.set_u16(this + 0x16, 0xffff);
        assert_eq!(e.call(0x00cf_5830, &args![this]).i32(), 10 - 0xffff);
    }

    #[test]
    fn next_set_bit_walks_up_to_the_count() {
        let mut e = Engine::new();
        let this = bit_set(&mut e, 100, &[3, 40, 99]);
        let next = |e: &mut Engine, from: i32| e.call(0x00cf_5840, &args![this, from]).i32();
        assert_eq!(next(&mut e, -1), 3);
        assert_eq!(next(&mut e, 3), 40);
        assert_eq!(next(&mut e, 40), 99);
        assert_eq!(next(&mut e, 99), -1);
        assert_eq!(next(&mut e, 5000), -1);
        // a bit at or past the count is not looked at
        let this = bit_set(&mut e, 50, &[60]);
        assert_eq!(e.call(0x00cf_5840, &args![this, 2i32]).i32(), -1);
    }

    #[test]
    fn next_set_bit_returns_indices_from_256_up_unexamined() {
        let mut e = Engine::new();
        let this = bit_set(&mut e, 300, &[]);
        assert_eq!(e.call(0x00cf_5840, &args![this, 253i32]).i32(), 256);
        // 255 is below 256 and its bit is clear: the walk reaches 256 and returns it
        let this = bit_set(&mut e, 300, &[]);
        assert_eq!(e.call(0x00cf_5840, &args![this, 255i32]).i32(), 256);
        // a negative start: -3 + 1 = -2 is below the count and compares as >= 256 unsigned
        assert_eq!(e.call(0x00cf_5840, &args![this, -3i32]).i32(), -2);
    }

    #[test]
    fn first_set_bit_starts_from_minus_one() {
        let mut e = Engine::new();
        let this = bit_set(&mut e, 64, &[0, 7]);
        assert_eq!(e.call(0x00cf_5880, &args![this]).i32(), 0);
        let this = bit_set(&mut e, 64, &[7]);
        assert_eq!(e.call(0x00cf_5880, &args![this]).i32(), 7);
        let this = bit_set(&mut e, 0, &[7]);
        assert_eq!(e.call(0x00cf_5880, &args![this]).i32(), -1);
    }

    #[test]
    fn deleting_destructor_00cf5890_frees_only_when_bit_zero_is_set() {
        check(0x00cf_5890, 0x00cf_5750);
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
    fn thunk_00cf5820() {
        assert_eq!(check(0x00cf_5820, 0x00cf_5890, 0x10), 0xdead_beef);
    }
}

#[cfg(test)]
mod tests_00cf56f0 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_BLOCK_DOUBLE: u32 = 0x0900_0511;
    const FREE_ARRAY_DOUBLE: u32 = 0x0900_0521;

    /// A router whose allocator frees through two doubles (slot 8: a block, slot `0x10`: array
    /// data), both logging tagged words (1: array data, 2: block).
    fn rig() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_BLOCK_DOUBLE);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(FREE_ARRAY_DOUBLE, move |_, a| {
            sink.borrow_mut().extend([1, a[1], a[2]]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(FREE_BLOCK_DOUBLE, move |_, a| {
            sink.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        (e, log)
    }

    /// Without the flag only the vtables are written; with it the number and byte go through
    /// 0, 7 (byte 3) and 9 (byte 0), and `this` is returned.
    #[test]
    fn constructor_writes_the_final_tables_and_numbers() {
        let (mut e, _) = rig();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x55);
        e.mem.set_u8(this + 0x15, 0x66);
        assert_eq!(e.call(0x00cf_56f0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_c3d8);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_c3b8);
        assert_eq!(e.mem.u32(this + 0xc), 0x55);
        assert_eq!(e.mem.u8(this + 0x15), 0x66);
        assert_eq!(e.call(0x00cf_56f0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 0xc), 9);
        assert_eq!(e.mem.u8(this + 0x15), 0);
    }

    /// Each child entry first word is released (also a null one), an owned array is freed with
    /// 16 bytes per element of capacity, and the tables end as the base ones.
    #[test]
    fn destructor_releases_children_and_frees_the_array() {
        let (mut e, log) = rig();
        let released = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&released);
        e.register_double(RELEASE_REFERENCE, move |_, a| {
            sink.borrow_mut().push(a[0]);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(0x30);
        for (i, shape) in [0x111u32, 0, 0x333].into_iter().enumerate() {
            e.mem.set_u32(data + 0x10 * i as u32, shape);
        }
        e.mem.set_u32(this + 0x18, data);
        e.mem.set_u32(this + 0x1c, 3);
        e.mem.set_u32(this + 0x20, 5);
        e.call(0x00cf_5750, &args![this]);
        assert_eq!(*released.borrow(), vec![0x111, 0, 0x333]);
        assert_eq!(*log.borrow(), vec![1, data, 0x50]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x20), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_6fb0);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        // A not-owned array is not freed and a count of zero releases nothing.
        log.borrow_mut().clear();
        released.borrow_mut().clear();
        e.call(0x00cf_5750, &args![this]);
        assert!(log.borrow().is_empty());
        assert!(released.borrow().is_empty());
    }

    /// The thunk works on the shape 0x10 below the container and returns it.
    #[test]
    fn container_thunk_resets_the_tables_and_frees_the_shape_block() {
        let (mut e, log) = rig();
        let shape = e.mem.alloc(0x40);
        e.mem.set_u16(shape + 4, 0x38);
        let got = e
            .call(0x00cf_57d0, &args![shape + 0x10, 0xffff_fffeu32])
            .u32();
        assert_eq!(got, shape);
        assert_eq!(e.mem.u32(shape), 0x0102_e388);
        assert_eq!(e.mem.u32(shape + 0x10), 0x010c_6fb0);
        assert!(log.borrow().is_empty());
        e.call(0x00cf_57d0, &args![shape + 0x10, 1u32]);
        assert_eq!(*log.borrow(), vec![2, shape, 0x38]);
    }
}

#[cfg(test)]
mod gap1_tests {
    use super::*;

    #[test]
    fn fn_00cf57e0_stores_the_vtables_and_frees_with_the_size() {
        let mut e = Engine::new();
        let mut slots = [0u32; 4];
        slots[2] = 0x0f00_0008;
        e.put_vtable(0x0f00_1000, &slots);
        e.register(0x0f00_0008, |_, _| Ret::default());
        let allocator = e.mem.alloc(4);
        e.mem.set_u32(allocator, 0x0f00_1000);
        let router = e.mem.alloc(0x14);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u16(this + 4, 0x60);
        assert_eq!(e.call(0x00cf_57e0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_6fb0);
        assert_eq!(e.call(0x00cf_57e0, &args![this, 1u32]).u32(), this);
        let freed: Vec<Vec<u32>> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0f00_0008)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(freed, vec![vec![allocator, this, 0x60]]);
    }
}
