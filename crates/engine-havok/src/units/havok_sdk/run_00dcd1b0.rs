//! `Havok SDK/run_00dcd1b0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dcd480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d4_bd60`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dcd480(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d4_bd60, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00dcd470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to `00dcd480`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dcd470(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00dc_d480, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// The functions below are members of the mesh shape whose vtable `010d7754` has the RTTI name
// `hkpMeshShape` (the Xbox PDB has no entry for the class, so the fields are described by what
// the code does with them). Its subparts are 0x38-byte records at the pointer at `+0x34` (count
// at `+0x38`, capacity word at `+0x3c`); the welding array is `u16`s at the pointer at `+0x40`
// (count `+0x44`, capacity word `+0x48`); the byte at `+0x4c` is the welding type, the float at
// `+0x50` the radius, the vector at `+0x20` the scale and the word at `+0x30` the number of
// bits of a triangle key that hold the subpart index.

/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl: allocator, array, capacity, element
/// size) and the function that makes room for one more element (`00c90940`, cdecl: allocator,
/// array, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkpShapeCollection::hkpShapeCollection` (Xbox PDB, `00d21410`, `thiscall`, `RET 8`).
const SHAPE_COLLECTION_CONSTRUCTOR: u32 = 0x00d2_1410;
/// Vtable pointers: the mesh shape's two (`010d7754` and the interface at `+0x10`, `010d7734`),
/// the collection's (`010cc364`, `010cc344`), the shape container's (`010c6fb0`) and the shape's
/// (`010305a8`).
const MESH_VTABLE: u32 = 0x010d_7754;
const MESH_INTERFACE_VTABLE: u32 = 0x010d_7734;
const COLLECTION_VTABLE: u32 = 0x010c_c364;
const COLLECTION_INTERFACE_VTABLE: u32 = 0x010c_c344;
const CONTAINER_INTERFACE_VTABLE: u32 = 0x010c_6fb0;
const SHAPE_VTABLE: u32 = 0x0103_05a8;
/// The `float` 1.0 (`01012068`), and the global zero vector (`01267e30`) a subpart without a
/// material table points at.
const FLOAT_ONE: u32 = 0x0101_2068;
const ZERO_VECTOR: u32 = 0x0126_7e30;
/// The capacity word of an array that owns nothing.
const NO_STORAGE: u32 = 0x8000_0000;
/// The size of the subpart records.
const SUBPART_SIZE: u32 = 0x38;

// Translated from 00dcd1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `mode`). Stores the low byte of
/// `mode` at `+0x4c`. Mode 6 releases the welding array: size 0, its storage freed through the
/// router's allocator (virtual function `+0x10`: block, capacity times 2 bytes, unless the
/// capacity word's sign bit is set), the pointer cleared and the capacity word set to
/// `0x80000000`. Any other mode lays the welding array out: each subpart's welding offset
/// (`+0x34`) becomes the sum of the triangle counts (`+0x1c`) before it, and the array is
/// reserved for that total (`00c908c0`, element size 2) when its capacity (low 30 bits of the
/// word at `+0x48`) is smaller, then once more (to the larger of twice the capacity and the
/// total) when it still is, as the code does; the size at `+0x44` becomes the total.
pub fn fn_00dcd1b0(e: &mut Engine, this: u32, mode: u32) {
    e.mem.set_u8(this + 0x4c, mode as u8);
    if mode == 6 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity_word = e.mem.u32(this + 0x48);
        e.mem.set_u32(this + 0x44, 0);
        if (capacity_word as i32) >= 0 {
            let storage = e.mem.u32(this + 0x40);
            e.vcall(
                allocator,
                0x10,
                &args![storage, (capacity_word & 0x3fff_ffff) * 2],
            );
        }
        e.mem.set_u32(this + 0x40, 0);
        e.mem.set_u32(this + 0x48, NO_STORAGE);
        return;
    }
    let mut total = 0i32;
    if e.mem.i32(this + 0x38) > 0 {
        let mut offset = 0u32;
        let mut index = 0i32;
        loop {
            let subparts = e.mem.u32(this + 0x34);
            e.mem.set_u32(
                offset.wrapping_add(subparts).wrapping_add(0x34),
                total as u32,
            );
            let subparts = e.mem.u32(this + 0x34);
            total = total.wrapping_add(e.mem.i32(offset.wrapping_add(subparts).wrapping_add(0x1c)));
            index += 1;
            offset = offset.wrapping_add(SUBPART_SIZE);
            if index >= e.mem.i32(this + 0x38) {
                break;
            }
        }
    }
    let welding = this + 0x40;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = (e.mem.u32(welding + 8) & 0x3fff_ffff) as i32;
    if capacity < total {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE, &args![allocator, welding, total, 2u32]);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(welding + 8) & 0x3fff_ffff) as i32;
    if capacity < total {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if total < doubled { doubled } else { total };
        e.call(ARRAY_RESERVE, &args![allocator, welding, wanted, 2u32]);
    }
    e.mem.set_u32(welding + 4, total as u32);
}

// Translated from 00dcd270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `subpart`). Appends the 0x38-byte
/// subpart record at `subpart` to the subpart array (`this + 0x34`, `hkArray` form: data, size,
/// capacity word): when the size equals the capacity it first makes room (`00c90940`, element
/// size 0x38), then initializes the new slot (byte `+0x11` 1, words `+0x18`, `+0x20`, `+0x24`,
/// `+0x28`, `+0x2c` 0, `+0x30` 1, `+0x34` all ones), counts it, and overwrites it with the
/// record. When the record's word at `+0x20` (the material array) is 0 the slot's `+0x20` and
/// `+0x28` become the global zero vector (`01267e30`) and `+0x30` 1.
pub fn fn_00dcd270(e: &mut Engine, this: u32, subpart: u32) {
    let array = this + 0x34;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, SUBPART_SIZE]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let slot = data.wrapping_add(size.wrapping_mul(SUBPART_SIZE));
    if slot != 0 {
        e.mem.set_u8(slot + 0x11, 1);
        e.mem.set_u32(slot + 0x24, 0);
        e.mem.set_u32(slot + 0x2c, 0);
        e.mem.set_u32(slot + 0x30, 1);
        e.mem.set_u32(slot + 0x28, 0);
        e.mem.set_u32(slot + 0x20, 0);
        e.mem.set_u32(slot + 0x34, 0xffff_ffff);
        e.mem.set_u32(slot + 0x18, 0);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let slot = data.wrapping_add(size.wrapping_mul(SUBPART_SIZE));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    for word in 0..0xe {
        let value = e.mem.u32(subpart + 4 * word);
        e.mem.set_u32(slot + 4 * word, value);
    }
    if e.mem.u32(slot + 0x20) == 0 {
        e.mem.set_u32(slot + 0x30, 1);
        e.mem.set_u32(slot + 0x28, ZERO_VECTOR);
        e.mem.set_u32(slot + 0x20, ZERO_VECTOR);
    }
}

// Translated from 00dcd310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: `this`, `radius`, `bits`): the constructor of
/// the mesh shape (`hkpMeshShape` by the vtable's RTTI). Runs the collection constructor
/// (`00d21410`, arguments 0x17 and 5), then sets the two vtables, the subpart array and the
/// welding array empty (data 0, size 0, capacity word `0x80000000`), the radius `float` at
/// `+0x50` (through the x87 stack, so quieted), the word `bits` at `+0x30`, the scale vector at
/// `+0x20` to the `float` at `01012068` in all four lanes and the byte at `+0x4c` to 6.
/// Answers `this`.
pub fn fn_00dcd310(e: &mut Engine, this: u32, radius: f32, bits: u32) -> u32 {
    e.call(SHAPE_COLLECTION_CONSTRUCTOR, &args![this, 0x17u32, 5u32]);
    let one = e.mem.u32(FLOAT_ONE);
    e.mem.set_u32(this, MESH_VTABLE);
    e.mem.set_u32(this + 0x10, MESH_INTERFACE_VTABLE);
    e.mem.set_u32(this + 0x34, 0);
    e.mem.set_u32(this + 0x38, 0);
    e.mem.set_u32(this + 0x3c, NO_STORAGE);
    e.mem.set_u32(this + 0x40, 0);
    e.mem.set_u32(this + 0x44, 0);
    e.mem.set_u32(this + 0x48, NO_STORAGE);
    e.mem.set_u32(this + 0x50, x87_quiet_bits(radius.to_bits()));
    e.mem.set_u32(this + 0x30, bits);
    for lane in 0..4 {
        e.mem.set_u32(this + 0x20 + 4 * lane, one);
    }
    e.mem.set_u8(this + 0x4c, 6);
    this
}

// Translated from 00dcd380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `flag`): the chain of base-class
/// vtable stores of the mesh shape's destructor (the stores are visible only through `this`).
/// It stores the shape vtable (`010305a8`), the container interface vtable (`010c6fb0` at
/// `+0x10`), the collection vtables (`010cc364`, `010cc344`) and finally the mesh shape's
/// (`010d7754`, `010d7734`). When `flag` is not 0 it also sets the word at `+0xc` to 0, 7 and
/// 0x17 in turn with the byte at `+0x15` 3 and 5 after the second and third steps, and sets every
/// subpart's byte at `+0x11` to 1 where it is 0. Answers `this`.
pub fn fn_00dcd380(e: &mut Engine, this: u32, flag: u32) -> u32 {
    e.mem.set_u32(this, SHAPE_VTABLE);
    if flag != 0 {
        e.mem.set_u32(this + 0xc, 0);
    }
    e.mem.set_u32(this + 0x10, CONTAINER_INTERFACE_VTABLE);
    e.mem.set_u32(this, COLLECTION_VTABLE);
    e.mem.set_u32(this + 0x10, COLLECTION_INTERFACE_VTABLE);
    if flag != 0 {
        e.mem.set_u32(this + 0xc, 7);
        e.mem.set_u8(this + 0x15, 3);
    }
    e.mem.set_u32(this, MESH_VTABLE);
    e.mem.set_u32(this + 0x10, MESH_INTERFACE_VTABLE);
    if flag != 0 {
        if e.mem.i32(this + 0x38) > 0 {
            let mut index = 0i32;
            let mut offset = 0u32;
            loop {
                let subparts = e.mem.u32(this + 0x34);
                let at = offset.wrapping_add(subparts).wrapping_add(0x11);
                if e.mem.u8(at) == 0 {
                    e.mem.set_u8(at, 1);
                }
                index += 1;
                offset = offset.wrapping_add(SUBPART_SIZE);
                if index >= e.mem.i32(this + 0x38) {
                    break;
                }
            }
        }
        e.mem.set_u32(this + 0xc, 0x17);
        e.mem.set_u8(this + 0x15, 5);
    }
    this
}

// Translated from 00dcd410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `key`). Looks up the subpart picked
/// by the top `[+0x30]` bits of the triangle key (subparts at the pointer at `+0x34`); answers
/// `0xffffffff` when its word at `+0x20` (a material array) is 0, else the `u8` (when the byte at
/// `+0x11` is 1) or `u16` at `[+0x20] + (triangle index in the subpart) * [+0x24]`.
pub fn fn_00dcd410(e: &mut Engine, this: u32, key: u32) -> u32 {
    let bits = e.mem.u32(this + 0x30);
    let subparts = e.mem.u32(this + 0x34);
    let subpart = key >> (0x20u32.wrapping_sub(bits) & 31);
    let element = subparts.wrapping_add(subpart.wrapping_mul(SUBPART_SIZE));
    let data = e.mem.u32(element + 0x20);
    if data == 0 {
        return 0xffff_ffff;
    }
    let triangle = (0xffff_ffffu32 >> (bits & 31)) & key;
    let at = triangle
        .wrapping_mul(e.mem.u32(element + 0x24))
        .wrapping_add(data);
    if e.mem.u8(element + 0x11) == 1 {
        u32::from(e.mem.u8(at))
    } else {
        u32::from(e.mem.u16(at))
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dc_d480, deleting_destructor_00dcd480(Ptr, u8) -> Ptr),
        entry!(0x00dc_d470, fn_00dcd470(Ptr, u32) -> u32),
        entry!(0x00dcd1b0, fn_00dcd1b0(u32, u32)),
        entry!(0x00dcd270, fn_00dcd270(u32, u32)),
        entry!(0x00dcd310, fn_00dcd310(u32, f32, u32) -> u32),
        entry!(0x00dcd380, fn_00dcd380(u32, u32) -> u32),
        entry!(0x00dcd410, fn_00dcd410(u32, u32) -> u32),
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
    fn deleting_destructor_00dcd480_frees_only_when_bit_zero_is_set() {
        check(0x00dc_d480, 0x00d4_bd60);
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
    fn thunk_00dcd470() {
        assert_eq!(check(0x00dc_d470, 0x00dc_d480, 0x10), 0xdead_beef);
    }
}

#[cfg(test)]
mod a07_mesh_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    const FREE_DOUBLE: u32 = 0x0074_0010;
    const ALLOCATOR: u32 = 0xa110;

    /// A router whose allocator frees through a double that logs the block and the size, and
    /// doubles for the two array functions: `reserve` logs its words (and sets the capacity
    /// word when `grow` is set), `reserve_more` logs and gives the array room for 4 more.
    fn environment(e: &mut Engine, grow: bool) -> Log {
        let log: Log = Rc::default();
        let allocator = e.mem.alloc(8);
        e.put_vtable(0x0074_1000, &[0, 0, 0, 0, FREE_DOUBLE]);
        e.mem.set_u32(allocator, 0x0074_1000);
        let router = e.mem.alloc(0x14);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_DOUBLE, move |_, a| {
            seen.borrow_mut().push(("free", vec![a[1], a[2]]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            // The allocator is shown as ALLOCATOR.
            seen.borrow_mut()
                .push(("reserve", vec![ALLOCATOR, a[1], a[2], a[3]]));
            if grow {
                e.mem.set_u32(a[1] + 8, a[2]);
            }
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            seen.borrow_mut()
                .push(("more", vec![ALLOCATOR, a[1], a[2]]));
            let size = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[1] + 8, size + 4);
            Ret::default()
        });
        log
    }

    /// A mesh object with subparts holding the given triangle counts.
    fn mesh_with_counts(e: &mut Engine, counts: &[u32]) -> (u32, u32) {
        let this = e.mem.alloc(0x60);
        let subparts = e.mem.alloc(0x38 * counts.len().max(1) as u32);
        for (i, &count) in counts.iter().enumerate() {
            e.mem.set_u32(subparts + 0x38 * i as u32 + 0x1c, count);
        }
        e.mem.set_u32(this + 0x34, subparts);
        e.mem.set_u32(this + 0x38, counts.len() as u32);
        (this, subparts)
    }

    #[test]
    fn mode_six_frees_the_welding_array() {
        let mut e = Engine::new();
        let log = environment(&mut e, false);
        let (this, _) = mesh_with_counts(&mut e, &[]);
        e.mem.set_u32(this + 0x40, 0x1234);
        e.mem.set_u32(this + 0x44, 9);
        e.mem.set_u32(this + 0x48, 0x4000_0005);
        e.call(0x00dc_d1b0, &args![this, 6u32]);
        assert_eq!(*log.borrow(), vec![("free", vec![0x1234, 10])]);
        assert_eq!(e.mem.u8(this + 0x4c), 6);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x40), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0x8000_0000);
        // A capacity word with its sign bit set is not freed.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x40, 0x1234);
        e.call(0x00dc_d1b0, &args![this, 6u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x40), 0);
    }

    #[test]
    fn the_welding_offsets_are_running_sums_and_the_array_is_reserved_twice_if_it_stays_small() {
        let mut e = Engine::new();
        let log = environment(&mut e, false);
        let (this, subparts) = mesh_with_counts(&mut e, &[3, 4]);
        e.mem.set_u32(this + 0x48, 2);
        e.call(0x00dc_d1b0, &args![this, 3u32]);
        assert_eq!(e.mem.u8(this + 0x4c), 3);
        assert_eq!(e.mem.u32(subparts + 0x34), 0);
        assert_eq!(e.mem.u32(subparts + 0x38 + 0x34), 3);
        assert_eq!(e.mem.u32(this + 0x44), 7);
        // Capacity 2, total 7: 2 * 2 is below 7, so the second request is for 7.
        assert_eq!(
            *log.borrow(),
            vec![
                ("reserve", vec![ALLOCATOR, this + 0x40, 7, 2]),
                ("reserve", vec![ALLOCATOR, this + 0x40, 7, 2]),
            ]
        );
    }

    #[test]
    fn the_second_request_is_twice_the_capacity_when_that_exceeds_the_total() {
        let mut e = Engine::new();
        let log = environment(&mut e, false);
        let (this, _) = mesh_with_counts(&mut e, &[2, 3]);
        e.mem.set_u32(this + 0x48, 4);
        e.call(0x00dc_d1b0, &args![this, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                ("reserve", vec![ALLOCATOR, this + 0x40, 5, 2]),
                ("reserve", vec![ALLOCATOR, this + 0x40, 8, 2]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x44), 5);
    }

    #[test]
    fn a_reserve_that_grows_the_array_is_not_repeated_and_a_big_enough_array_is_left_alone() {
        let mut e = Engine::new();
        let log = environment(&mut e, true);
        let (this, _) = mesh_with_counts(&mut e, &[2, 3]);
        e.mem.set_u32(this + 0x48, 4);
        e.call(0x00dc_d1b0, &args![this, 1u32]);
        assert_eq!(log.borrow().len(), 1);
        log.borrow_mut().clear();
        let (this, _) = mesh_with_counts(&mut e, &[1]);
        e.mem.set_u32(this + 0x48, 0x8000_0001);
        e.call(0x00dc_d1b0, &args![this, 1u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x44), 1);
        // No subparts: nothing to lay out, nothing to reserve.
        let (this, _) = mesh_with_counts(&mut e, &[]);
        e.call(0x00dc_d1b0, &args![this, 1u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    #[test]
    fn a_subpart_is_appended_with_room_made_when_full() {
        let mut e = Engine::new();
        let log = environment(&mut e, false);
        let this = e.mem.alloc(0x60);
        let data = e.mem.alloc(0x38 * 8);
        e.mem.set_u32(this + 0x34, data);
        e.mem.set_u32(this + 0x3c, 2);
        let source = e.mem.alloc(0x40);
        for word in 0..0xe {
            e.mem.set_u32(source + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(source + 0x20, 0);
        e.call(0x00dc_d270, &args![this, source]);
        assert!(log.borrow().is_empty());
        assert_eq!(e.mem.u32(this + 0x38), 1);
        assert_eq!(e.mem.u32(data), 0x100);
        assert_eq!(e.mem.u32(data + 0x34), 0x10d);
        // The material word was 0: the slot points at the zero vector and `+0x30` is 1.
        assert_eq!(e.mem.u32(data + 0x20), ZERO_VECTOR);
        assert_eq!(e.mem.u32(data + 0x28), ZERO_VECTOR);
        assert_eq!(e.mem.u32(data + 0x30), 1);
        // Full: size 1 equals the capacity 1.
        e.mem.set_u32(this + 0x3c, 1);
        e.mem.set_u32(source + 0x20, 0x77);
        e.call(0x00dc_d270, &args![this, source]);
        assert_eq!(
            *log.borrow(),
            vec![("more", vec![ALLOCATOR, this + 0x34, 0x38])]
        );
        assert_eq!(e.mem.u32(this + 0x38), 2);
        assert_eq!(e.mem.u32(data + 0x38 + 0x20), 0x77);
        assert_eq!(e.mem.u32(data + 0x38 + 0x28), 0x10a);
    }

    #[test]
    fn the_constructor_runs_the_collection_constructor_and_sets_the_members() {
        let mut e = Engine::new();
        e.map(FLOAT_ONE, 8);
        e.mem.set_f32(FLOAT_ONE, 1.0);
        let calls: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let seen = Rc::clone(&calls);
        e.register_double(SHAPE_COLLECTION_CONSTRUCTOR, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_u32(this + 4 * word, 0xdead_0000);
        }
        let r = e.call(
            0x00dc_d310,
            &args![this, f32::from_bits(0x7f80_0001), 0xaau32],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(*calls.borrow(), vec![vec![this, 0x17, 5]]);
        assert_eq!(e.mem.u32(this), MESH_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), MESH_INTERFACE_VTABLE);
        let words: Vec<u32> = (0x34..0x4c)
            .step_by(4)
            .map(|o| e.mem.u32(this + o))
            .collect();
        assert_eq!(words, vec![0, 0, 0x8000_0000, 0, 0, 0x8000_0000]);
        // The radius went through the x87 stack: the signalling NaN is quiet.
        assert_eq!(e.mem.u32(this + 0x50), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x30), 0xaa);
        assert_eq!(e.mem.u8(this + 0x4c), 6);
        let scale: Vec<f32> = (0..4).map(|i| e.mem.f32(this + 0x20 + 4 * i)).collect();
        assert_eq!(scale, vec![1.0; 4]);
    }

    #[test]
    fn the_destructor_chain_ends_with_the_mesh_vtables() {
        let mut e = Engine::new();
        let (this, subparts) = mesh_with_counts(&mut e, &[1, 1]);
        e.mem.set_u8(subparts + 0x11, 0);
        e.mem.set_u8(subparts + 0x38 + 0x11, 5);
        e.mem.set_u32(this + 0xc, 0x99);
        assert_eq!(e.call(0x00dc_d380, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), MESH_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), MESH_INTERFACE_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x99);
        assert_eq!(e.mem.u8(subparts + 0x11), 0);
        assert_eq!(e.call(0x00dc_d380, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this + 0xc), 0x17);
        assert_eq!(e.mem.u8(this + 0x15), 5);
        assert_eq!(e.mem.u8(subparts + 0x11), 1);
        assert_eq!(e.mem.u8(subparts + 0x38 + 0x11), 5);
    }

    #[test]
    fn a_material_is_read_as_a_byte_or_a_halfword() {
        let mut e = Engine::new();
        let (this, subparts) = mesh_with_counts(&mut e, &[2, 4]);
        let materials = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x30, 1);
        let second = subparts + 0x38;
        e.mem.set_u32(second + 0x20, materials);
        e.mem.set_u32(second + 0x24, 2);
        e.mem.set_u16(materials + 4, 0x1234);
        let key = 0x8000_0002u32;
        e.mem.set_u8(second + 0x11, 0);
        assert_eq!(e.call(0x00dc_d410, &args![this, key]).u32(), 0x1234);
        e.mem.set_u8(second + 0x11, 1);
        assert_eq!(e.call(0x00dc_d410, &args![this, key]).u32(), 0x34);
        // Subpart 0 has no material array.
        assert_eq!(e.call(0x00dc_d410, &args![this, 1u32]).u32(), 0xffff_ffff);
    }
}
