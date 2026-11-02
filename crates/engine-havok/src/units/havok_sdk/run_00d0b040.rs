//! `Havok SDK/run_00d0b040` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00d0b1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d0b1e0`. `00d0b1e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0b1c0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_b1e0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d0b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xc (`SUB ECX,0xc`) and jumps to `00d0b1e0`. `00d0b1e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d0b1d0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d0_b1e0, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator; its slot `4` allocates
/// `(size)`, slot `8` frees `(block, size)` and slot `0x10` frees array data `(block, bytes)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The calls `00d0b040` makes on the world: `00c93a60` and `00c937e0` (`this` = the world, one
/// word each) and `00d9e250` (`this` = the part at `+0x14` of the new block, the world).
const WORLD_CALL_FIRST: u32 = 0x00c9_3a60;
const WORLD_CALL_SECOND: u32 = 0x00c9_37e0;
const BLOCK_PART_CALL: u32 = 0x00d9_e250;
/// `00d9e860` (`this` = the part at `+0x14`, no arguments).
const BUILD_PART: u32 = 0x00d9_e860;
/// The mask of the capacity bits of an `hkArray`'s capacity word and the word of a not-owned one.
const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;

/// The memory router's allocator (`00c85750`, field `+0x10`).
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Empties the `hkArray` at `array` (data `+0`, size `+4`, capacity word `+8`) of `element_size`-byte
/// elements: the size becomes 0 and, unless bit 31 of the capacity word says the memory is not
/// owned, the data goes back to the memory router's allocator (slot `0x10`, `(capacity &
/// 0x3fffffff) * element_size` bytes); then the data pointer is cleared and the capacity word
/// set to `0x80000000`.
fn empty_array(e: &mut Engine, array: u32, element_size: u32) {
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & NOT_OWNED == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NOT_OWNED);
}

// Translated from 00d0b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(world)`: takes a reference on the helper object the world keeps at `+0x1c0` (the
/// class is unconfirmed): when it exists its count at `+0x10` is raised by one. Otherwise `0x4c`
/// bytes are asked of the memory router's allocator (slot 4), the 16-bit word `0x4c` is stored at
/// `+4` of the block, `00d0b180` constructs it and it becomes the helper; `00c93a60` runs on the
/// world with `block + 0xc`, `00c937e0` with `block + 8` and `00d9e250` on `block + 0x14` with the
/// world. (The exe also has the branches for a null result of the constructor; they are
/// translated and cannot run unless the allocation fails.)
pub fn fn_00d0b040(e: &mut Engine, world: Ptr) {
    let world = world.addr();
    let helper = e.mem.u32(world + 0x1c0);
    if helper != 0 {
        let count = e.mem.u32(helper + 0x10);
        e.mem.set_u32(helper + 0x10, count.wrapping_add(1));
        return;
    }
    let allocator = memory_allocator(e);
    let block = e.vcall(allocator, 4, &args![0x4cu32]).u32();
    e.mem.set_u16(block + 4, 0x4c);
    let made = fn_00d0b180(e, Ptr::new(block)).addr();
    e.mem.set_u32(world + 0x1c0, made);
    let part = if made != 0 { made.wrapping_add(0xc) } else { 0 };
    e.call(WORLD_CALL_FIRST, &args![world, part]);
    if made != 0 {
        e.call(WORLD_CALL_SECOND, &args![world, made.wrapping_add(8)]);
        e.call(BLOCK_PART_CALL, &args![made.wrapping_add(0x14), world]);
    } else {
        e.call(WORLD_CALL_SECOND, &args![world, 0u32]);
        e.call(BLOCK_PART_CALL, &args![0x14u32, world]);
    }
}

// Translated from 00d0b0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the helper `00d0b180` builds: empties the `hkArray`s of 12-byte elements
/// at `+0x2c`, of 16-byte elements at `+0x20` and of 12-byte elements at `+0x14` (in that order),
/// then writes the table words `010ccf7c` at `+8` and `0102e388` at `+0`.
pub fn fn_00d0b0d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    empty_array(e, base + 0x2c, 12);
    empty_array(e, base + 0x20, 16);
    empty_array(e, base + 0x14, 12);
    e.mem.set_u32(base + 8, 0x010c_cf7c);
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00d0b180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the helper (`this` is returned): the 16-bit word at `+6` becomes 1, the table
/// words at `+8` and `+0xc` first `010cb1e4` and `010cb1c4`, then the final tables (`010cd33c` at
/// `+0`, `010cd31c` at `+8`, `010cd30c` at `+0xc`), the word at `+0x10` becomes 1 and `00d9e860`
/// builds the part at `+0x14`.
pub fn fn_00d0b180(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base + 6, 1);
    e.mem.set_u32(base + 8, 0x010c_b1e4);
    e.mem.set_u32(base + 0xc, 0x010c_b1c4);
    e.mem.set_u32(base, 0x010c_d33c);
    e.mem.set_u32(base + 8, 0x010c_d31c);
    e.mem.set_u32(base + 0xc, 0x010c_d30c);
    e.mem.set_u32(base + 0x10, 1);
    e.call(BUILD_PART, &args![base + 0x14]);
    this
}

// Translated from 00d0b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the helper: runs `00d0b0d0` on `this + 0x14`, writes the table words
/// `010cb1c4` at `+0xc`, `010cb1e4` at `+8` and `0102e388` at `+0`, and when bit 0 of `flags` is set
/// frees the block through the memory router's allocator (slot `8`) with the 16-bit size kept at
/// `+4`. Returns `this`.
pub fn fn_00d0b1e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let base = this.addr();
    fn_00d0b0d0(e, Ptr::new(base + 0x14));
    e.mem.set_u32(base + 0xc, 0x010c_b1c4);
    e.mem.set_u32(base + 8, 0x010c_b1e4);
    e.mem.set_u32(base, 0x0102_e388);
    if flags & 1 != 0 {
        let allocator = memory_allocator(e);
        let size = u32::from(e.mem.u16(base + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_b040, fn_00d0b040(Ptr)),
        entry!(0x00d0_b0d0, fn_00d0b0d0(Ptr)),
        entry!(0x00d0_b180, fn_00d0b180(Ptr) -> Ptr),
        entry!(0x00d0_b1e0, fn_00d0b1e0(Ptr, u8) -> Ptr),
        entry!(0x00d0_b1c0, fn_00d0b1c0(Ptr, u32) -> u32),
        entry!(0x00d0_b1d0, fn_00d0b1d0(Ptr, u32) -> u32),
    ]
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
    fn le81_thunk_00d0b1c0() {
        assert_eq!(
            check(0x00d0_b1c0, 0x00d0_b1e0, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d0b1d0() {
        assert_eq!(
            check(0x00d0_b1d0, 0x00d0_b1e0, 0xc, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod tests_00d0b040 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ALLOCATE_DOUBLE: u32 = 0x0900_0601;
    const FREE_BLOCK_DOUBLE: u32 = 0x0900_0611;
    const FREE_ARRAY_DOUBLE: u32 = 0x0900_0621;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// A router whose allocator allocates (slot 4, returns `block`), frees a block (slot 8) and
    /// frees array data (slot 0x10) through doubles that log; doubles for the other callees.
    fn rig() -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let block = e.mem.alloc(0x60);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_DOUBLE);
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
        for (address, name) in [
            (ALLOCATE_DOUBLE, "allocate"),
            (FREE_BLOCK_DOUBLE, "free block"),
            (FREE_ARRAY_DOUBLE, "free array"),
            (WORLD_CALL_FIRST, "world first"),
            (WORLD_CALL_SECOND, "world second"),
            (BLOCK_PART_CALL, "block part"),
            (BUILD_PART, "build part"),
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((name, a.to_vec()));
                Ret {
                    eax: if name == "allocate" { block } else { 0 },
                    ..Ret::default()
                }
            });
        }
        (e, log, block, allocator)
    }

    /// With a helper present only its count at `+0x10` rises; without one a block is made,
    /// constructed, stored in the world and wired up.
    #[test]
    fn world_helper_is_counted_or_made() {
        let (mut e, log, block, allocator) = rig();
        let world = e.mem.alloc(0x200);
        let existing = e.mem.alloc(0x20);
        e.mem.set_u32(existing + 0x10, 4);
        e.mem.set_u32(world + 0x1c0, existing);
        e.call(0x00d0_b040, &args![world]);
        assert_eq!(e.mem.u32(existing + 0x10), 5);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(world + 0x1c0, 0);
        e.call(0x00d0_b040, &args![world]);
        assert_eq!(e.mem.u32(world + 0x1c0), block);
        assert_eq!(e.mem.u16(block + 4), 0x4c);
        assert_eq!(e.mem.u32(block + 0x10), 1);
        assert_eq!(e.mem.u32(block), 0x010c_d33c);
        assert_eq!(
            *log.borrow(),
            vec![
                ("allocate", vec![allocator, 0x4c]),
                ("build part", vec![block + 0x14]),
                ("world first", vec![world, block + 0xc]),
                ("world second", vec![world, block + 8]),
                ("block part", vec![block + 0x14, world])
            ]
        );
    }

    /// The constructor's table words, the destructor body and the deleting destructor.
    #[test]
    fn helper_constructor_and_destructors() {
        let (mut e, log, _, _) = rig();
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00d0_b180, &args![this]).u32(), this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), 0x010c_d33c);
        assert_eq!(e.mem.u32(this + 8), 0x010c_d31c);
        assert_eq!(e.mem.u32(this + 0xc), 0x010c_d30c);
        assert_eq!(e.mem.u32(this + 0x10), 1);
        assert_eq!(*log.borrow(), vec![("build part", vec![this + 0x14])]);
        // The body: owned arrays are freed with their element sizes, not-owned ones are not.
        log.borrow_mut().clear();
        for (array, data, capacity) in [
            (0x2cu32, 0x5000u32, 2u32),
            (0x20, 0x6000, 0x8000_0003),
            (0x14, 0x7000, 4),
        ] {
            e.mem.set_u32(this + array, data);
            e.mem.set_u32(this + array + 4, 1);
            e.mem.set_u32(this + array + 8, capacity);
        }
        e.call(0x00d0_b0d0, &args![this]);
        let frees: Vec<(&str, Vec<u32>)> = log
            .borrow()
            .iter()
            .map(|(name, words)| (*name, words[1..].to_vec()))
            .collect();
        assert_eq!(
            frees,
            vec![
                ("free array", vec![0x5000, 24]),
                ("free array", vec![0x7000, 48])
            ]
        );
        for array in [0x2cu32, 0x20, 0x14] {
            assert_eq!(e.mem.u32(this + array), 0);
            assert_eq!(e.mem.u32(this + array + 4), 0);
            assert_eq!(e.mem.u32(this + array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(this + 8), 0x010c_cf7c);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        // The deleting destructor.
        log.borrow_mut().clear();
        e.mem.set_u16(this + 4, 0x60);
        assert_eq!(e.call(0x00d0_b1e0, &args![this, 2u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 0xc), 0x010c_b1c4);
        assert_eq!(e.mem.u32(this + 8), 0x010c_b1e4);
        assert!(log.borrow().iter().all(|(name, _)| *name != "free block"));
        e.call(0x00d0_b1e0, &args![this, 1u32]);
        let last = log.borrow().last().cloned().unwrap();
        assert_eq!(last.0, "free block");
        assert_eq!(last.1[1..], [this, 0x60]);
    }
}
