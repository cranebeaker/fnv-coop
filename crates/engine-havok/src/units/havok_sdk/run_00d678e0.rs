//! `Havok SDK/run_00d678e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d67930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d6_7250`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d67930(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d6_7250, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00d678f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x2c (`SUB ECX,0x2c`) and jumps to `00d67930`. `00d67930` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d678f0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_7930, &args![this.addr().wrapping_sub(0x2c), flags])
        .u32()
}

// Translated from 00d67900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x20 (`SUB ECX,0x20`) and jumps to `00d67930`. `00d67930` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d67900(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_7930, &args![this.addr().wrapping_sub(0x20), flags])
        .u32()
}

// Translated from 00d67910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00d67930`. `00d67930` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d67910(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_7930, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00d67920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x28 (`SUB ECX,0x28`) and jumps to `00d67930`. `00d67930` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00d67920(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00d6_7930, &args![this.addr().wrapping_sub(0x28), flags])
        .u32()
}

use super::run_00d51760::{load_lanes, mulps, sse_add, sse_mul, store_lanes, subps, Lanes};

/// `hkpWorld` methods of `hkpworld.cpp` (thiscall on the world, one argument, the listener):
/// `00c93920` (Xbox PDB name `hkpWorld::addWorldPostSimulationListener`) pushes it on the array
/// at world `+0x164`, `00c93a20` on the one at `+0x1a0`.
const WORLD_ADD_POST_SIMULATION_LISTENER: u32 = 0x00c9_3920;
const WORLD_ADD_ISLAND_ACTIVATION_LISTENER: u32 = 0x00c9_3a20;
/// `hkpEntity` methods of `hkpentity.cpp` (thiscall on the entity, one argument, the listener):
/// `00c9c5b0` adds the listener to the entity's listener array, `00c9c670` removes it.
const ENTITY_ADD_LISTENER: u32 = 0x00c9_c5b0;
const ENTITY_REMOVE_LISTENER: u32 = 0x00c9_c670;
/// `hkMapBase<unsigned long, unsigned long, hkMapOperations<unsigned long>>` methods
/// (`hkmap.cpp`, thiscall on the map): `findKey` (Xbox PDB, `00cb18f0`, one argument, the key;
/// answers the slot index, above the hash mask when the key is missing), `00cb1870` (three
/// arguments: the allocator, the key, the value; inserts) and `00cb1fb0` (one argument, the
/// key: removes it).
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
const MAP_INSERT: u32 = 0x00cb_1870;
const MAP_REMOVE: u32 = 0x00cb_1fb0;
/// The id the rigid body inertia viewer's process was registered under (`01268ffc`) and the
/// one of the TOI count viewer (`01269000`).
const INERTIA_PROCESS_ID: u32 = 0x0126_8ffc;
const TOI_COUNT_PROCESS_ID: u32 = 0x0126_9000;
/// The mask `ANDPS` takes the absolute values with (`01017d10`, four words `7fffffff`), the float
/// half (`01016248`) and the four-float constant `01267e30` the arrow drawing starts from.
const ABS_MASK: u32 = 0x0101_7d10;
const HALF: u32 = 0x0101_6248;
const ARROW_DIRECTIONS: u32 = 0x0126_7e30;

// Translated from 00d678e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id kept at `01268ffc`: slot 1 of the vtable `010d1d60` (RTTI:
/// `hkpRigidBodyInertiaViewer`); `ECX` unused.
pub fn fn_00d678e0(e: &mut Engine) -> u32 {
    e.global(INERTIA_PROCESS_ID)
}

// Translated from 00d67960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers the process id kept at `01269000`: slot 1 of the vtable `010d1e4c` (RTTI:
/// `hkpToiCountViewer`); `ECX` unused.
pub fn fn_00d67960(e: &mut Engine) -> u32 {
    e.global(TOI_COUNT_PROCESS_ID)
}

// Translated from 00d67970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of a sub-object at `+0x20` (`this - 0x20` is the whole object) taking a world: adds
/// the listener `this + 0xc` (0 when the whole object would be null) to the world's
/// post-simulation listeners (`00c93920`), then `this + 8` (or 0) to the array at `+0x1a0`
/// (`00c93a20`). The world comes as the one stack word.
pub fn fn_00d67970(e: &mut Engine, this: Ptr, world: Ptr) {
    let at = this.addr();
    let whole = at.wrapping_sub(0x20);
    let first = if whole != 0 { at.wrapping_add(0xc) } else { 0 };
    e.call(WORLD_ADD_POST_SIMULATION_LISTENER, &args![world, first]);
    let second = if whole != 0 { at.wrapping_add(8) } else { 0 };
    e.call(WORLD_ADD_ISLAND_ACTIVATION_LISTENER, &args![world, second]);
}

// Translated from 00d679c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index of the largest component of the vector (cdecl, one pointer): squares the lanes
/// (`MULPS`), takes the absolute values (`ANDPS` with the mask at `01017d10`) and compares the
/// first three as x87 floats in the exe's order (`b > a` decides between the first two, `c`
/// against the winner decides the rest; ties and unordered values keep the lower index). The
/// answer is that index, plus 3 when the component itself (read from the vector again) is
/// above zero (an ordered compare).
pub fn fn_00d679c0(e: &mut Engine, vector: Ptr) -> i32 {
    let lanes = load_lanes(e, vector.addr());
    let squares = mulps(lanes, lanes);
    let mask = load_lanes(e, ABS_MASK);
    let value = |lane: usize| x87_load32(squares[lane] & mask[lane]);
    let (a, b, c) = (value(0), value(1), value(2));
    // `x` is not above `y` (the `FCOM` flags C0 or C3: below, equal or unordered).
    let not_above = |x: f64, y: f64| x <= y || x.is_nan() || y.is_nan();
    let index: u32 = if not_above(b, a) {
        if not_above(c, a) {
            0
        } else {
            2
        }
    } else if not_above(c, b) {
        1
    } else {
        2
    };
    let component = e.mem.u32(vector.addr().wrapping_add(index.wrapping_mul(4)));
    if 0.0 < x87_m32(component) {
        (index + 3) as i32
    } else {
        index as i32
    }
}

// Translated from 00d67a50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d67a50 - for a face remainder of -2 the exe writes over its own saved EBX (stack frame), which the engine does not model.
/// Draws an arrow for one face of an entity's bounding box (cdecl, eight stack words): asks the
/// entity's shape (the object at `entity + 0x10`, virtual method at `+0x1c`: the transform at
/// `entity + 0xf0`, tolerance 0.0, a local box of two vectors) for its box, takes the centre
/// (`0.5 * (max + min)`, `SSE`) and the half extents (`max - centre`), and builds the
/// direction `L` from the four floats at `01267e30` with lane `face % 3` replaced by `1 - 2 *
/// (face / 3)` (a signed remainder; the lane is addressed from the start of `L`, so a negative
/// face writes below it in the local frame, and for a remainder of -2 onto the saved `EBX` of the
/// exe's own frame, a register effect the engine does not model). The distance is `|sum of half * L|` (x87 `FABS`
/// of the stored float), the arrow starts at `p0 = centre + distance * L` and ends at
/// `p1 = p0 + length * L`. Then the method at vtable offset `0x30` of `target` is called with
/// `(fifth, &p1, sixth, entity + 0x10, second)`, and when `both` is not zero the one at `0x24`
/// with `(&p0, entity + 0x140, sixth, entity + 0x10, second)`. The argument roles are not
/// confirmed (the callees take a colour-like word, points and an id).
#[allow(clippy::too_many_arguments)]
pub fn fn_00d67a50(
    e: &mut Engine,
    target: Ptr,
    second: u32,
    entity: Ptr,
    face: i32,
    fifth: u32,
    sixth: u32,
    length: f32,
    both: u8,
) {
    e.with_stack(0x80, |e, frame| {
        // `a` stands for the aligned stack pointer of the exe's frame; its locals are the
        // ones at these offsets from it: the box (-0x20), the direction (-0x50), the second
        // point (-0x40), the first point (-0x30) and the scratch float (-0x54).
        let a = frame.addr().wrapping_add(0x60);
        let (boxed, direction, p1, p0, scratch) = (
            a.wrapping_sub(0x20),
            a.wrapping_sub(0x50),
            a.wrapping_sub(0x40),
            a.wrapping_sub(0x30),
            a.wrapping_sub(0x54),
        );
        let entity_at = entity.addr();
        let shape = e.mem.u32(entity_at.wrapping_add(0x10));
        e.vcall(
            shape,
            0x1c,
            &args![entity_at.wrapping_add(0xf0), 0u32, boxed],
        );
        let max = load_lanes(e, a.wrapping_sub(0x10));
        let min = load_lanes(e, boxed);
        let half = e.mem.u32(HALF);
        // face / 3 and face % 3 the way the exe does them (`IMUL` by 0x55555556).
        let product = i64::from(face) * 0x5555_5556;
        let high = (product >> 32) as i32;
        let quotient = high.wrapping_add(((high as u32) >> 31) as i32);
        let remainder = face.wrapping_sub(quotient.wrapping_mul(3));
        let doubled = x87_add(f64::from(quotient), f64::from(quotient));
        let lane = x87_store32(x87_sub(1.0, doubled));
        let centre_sum: Lanes = std::array::from_fn(|i| sse_add(max[i], min[i]));
        let centre: Lanes = std::array::from_fn(|i| sse_mul(half, centre_sum[i]));
        let extents = subps(max, centre);
        let directions = load_lanes(e, ARROW_DIRECTIONS);
        store_lanes(e, direction, directions);
        e.mem.set_u32(
            direction.wrapping_add((remainder as u32).wrapping_mul(4)),
            lane,
        );
        let l = load_lanes(e, direction);
        let products = mulps(extents, l);
        let partial = sse_add(products[1], products[0]);
        let sum = sse_add(products[2], partial);
        e.mem.set_u32(scratch, sum);
        let distance = x87_quiet_bits(x87_store32(x87_load32(e.mem.u32(scratch)).abs()));
        let first: Lanes = std::array::from_fn(|i| sse_add(sse_mul(distance, l[i]), centre[i]));
        let second_point: Lanes =
            std::array::from_fn(|i| sse_add(sse_mul(length.to_bits(), l[i]), first[i]));
        store_lanes(e, p0, first);
        store_lanes(e, p1, second_point);
        e.vcall(
            target.addr(),
            0x30,
            &args![fifth, p1, sixth, entity_at.wrapping_add(0x10), second],
        );
        if both != 0 {
            e.vcall(
                target.addr(),
                0x24,
                &args![
                    p0,
                    entity_at.wrapping_add(0x140),
                    sixth,
                    entity_at.wrapping_add(0x10),
                    second
                ],
            );
        }
    });
}

/// The shared body of `00d67b90` and `00d67bc0` (the same code at two addresses): removes the
/// entity from the entity listener array through `00c9c670` (the listener `this`, or 0 when
/// `this` is `0x30`) and removes it from the map at `this + 4` (`00cb1fb0`).
fn entity_removed(e: &mut Engine, this: Ptr, entity: u32) {
    let at = this.addr();
    let listener = if at != 0x30 { at } else { 0 };
    e.call(ENTITY_REMOVE_LISTENER, &args![entity, listener]);
    e.call(MAP_REMOVE, &args![at.wrapping_add(4), entity]);
}

// Translated from 00d67b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of a sub-object at `+0x30` taking an entity: takes the listener (`this`, or 0 when
/// `this` is exactly `0x30`, the null object's sub-object) away from the entity (`00c9c670` on
/// `entity`) and removes the entity from the map at `this + 4` (`00cb1fb0`).
pub fn fn_00d67b90(e: &mut Engine, this: Ptr, entity: u32) {
    entity_removed(e, this, entity);
}

// Translated from 00d67bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same code as `00d67b90` at another address (another vtable's slot).
pub fn fn_00d67bc0(e: &mut Engine, this: Ptr, entity: u32) {
    entity_removed(e, this, entity);
}

// Translated from 00d67bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the method at vtable offset 4 of the sub-object at `this + 0x18` once for each world
/// of the container at `[this + 0x1c]` (a count at `+0x70`, an array of world pointers at
/// `+0x6c`); the container is read again after every call and the loop ends when it has gone
/// null or the index reaches the count.
pub fn fn_00d67bf0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    let mut container = e.mem.u32(at.wrapping_add(0x1c));
    let mut index = 0i32;
    while container != 0 && index < e.mem.i32(container.wrapping_add(0x70)) {
        let worlds = e.mem.u32(container.wrapping_add(0x6c));
        let world = e
            .mem
            .u32(worlds.wrapping_add((index as u32).wrapping_mul(4)));
        e.vcall(at.wrapping_add(0x18), 4, &args![world]);
        container = e.mem.u32(at.wrapping_add(0x1c));
        index = index.wrapping_add(1);
    }
}

// Translated from 00d67c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts a pair of entities for the TOI count viewer (`this` has the map at `+0xc`, its hash
/// mask at `+0x14`): when the word at `event + 0x10` is not above zero, does the following for
/// the two entities at `event + 4` and `event + 8`. A key the map does not know (`00cb18f0`
/// answers an index above the mask) is inserted with the value `0x1000001` (the router's
/// allocator, `00cb1870`) and the entity gets the listener `this + 8` (0 when `this - 0x28`
/// is null; `00c9c5b0`). A known key has the two counters of its value raised by one, the top
/// byte and the low 24 bits, each at most `0xff` and `0xffffff`.
pub fn fn_00d67c30(e: &mut Engine, this: Ptr, event: Ptr) {
    let at = this.addr();
    if e.mem.i32(event.addr().wrapping_add(0x10)) > 0 {
        return;
    }
    let map = at.wrapping_add(0xc);
    let mut cursor = event.addr().wrapping_add(4);
    for _ in 0..2 {
        let key = e.mem.u32(cursor);
        let index = e.call(MAP_FIND_KEY, &args![map, key]).i32();
        if index > e.mem.i32(at.wrapping_add(0x14)) {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router.wrapping_add(0x10));
            e.call(MAP_INSERT, &args![map, allocator, key, 0x0100_0001u32]);
            let listener = if at.wrapping_sub(0x28) != 0 {
                at.wrapping_add(8)
            } else {
                0
            };
            e.call(ENTITY_ADD_LISTENER, &args![key, listener]);
        } else {
            let data = e.mem.u32(map);
            let slot = data
                .wrapping_add((index as u32).wrapping_mul(8))
                .wrapping_add(4);
            let word = e.mem.u32(slot);
            let high = (word >> 0x18).wrapping_add(1).min(0xff);
            let low = (word & 0x00ff_ffff).wrapping_add(1).min(0x00ff_ffff);
            e.mem.set_u32(slot, (high << 0x18) | low);
        }
        cursor = cursor.wrapping_add(4);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_78e0, fn_00d678e0() -> u32),
        entry!(0x00d6_7960, fn_00d67960() -> u32),
        entry!(0x00d6_7970, fn_00d67970(Ptr, Ptr)),
        entry!(0x00d6_79c0, fn_00d679c0(Ptr) -> i32),
        entry!(
            0x00d6_7a50,
            fn_00d67a50(Ptr, u32, Ptr, i32, u32, u32, f32, u8)
        ),
        entry!(0x00d6_7b90, fn_00d67b90(Ptr, u32)),
        entry!(0x00d6_7bc0, fn_00d67bc0(Ptr, u32)),
        entry!(0x00d6_7bf0, fn_00d67bf0(Ptr)),
        entry!(0x00d6_7c30, fn_00d67c30(Ptr, Ptr)),
        entry!(0x00d6_7930, deleting_destructor_00d67930(Ptr, u8) -> Ptr),
        entry!(0x00d6_78f0, fn_00d678f0(Ptr, u32) -> u32),
        entry!(0x00d6_7900, fn_00d67900(Ptr, u32) -> u32),
        entry!(0x00d6_7910, fn_00d67910(Ptr, u32) -> u32),
        entry!(0x00d6_7920, fn_00d67920(Ptr, u32) -> u32),
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
    fn deleting_destructor_00d67930_frees_only_when_bit_zero_is_set() {
        check(0x00d6_7930, 0x00d6_7250);
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
    fn le81_thunk_00d678f0() {
        assert_eq!(
            check(0x00d6_78f0, 0x00d6_7930, 0x2c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d67900() {
        assert_eq!(
            check(0x00d6_7900, 0x00d6_7930, 0x20, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d67910() {
        assert_eq!(
            check(0x00d6_7910, 0x00d6_7930, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00d67920() {
        assert_eq!(
            check(0x00d6_7920, 0x00d6_7930, 0x28, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

#[cfg(test)]
mod toi_viewer_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn record(e: &mut Engine, log: &Log, address: u32, answer: u32) {
        let seen = Rc::clone(log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x0126_8000, 0x2000);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        e.mem.set_u32(HALF, 0.5f32.to_bits());
        e
    }

    #[test]
    fn fn_00d678e0_and_00d67960_answer_the_registered_process_ids() {
        let mut e = engine();
        e.set_global(INERTIA_PROCESS_ID, 0x11u32);
        e.set_global(TOI_COUNT_PROCESS_ID, 0x22u32);
        assert_eq!(e.call(0x00d6_78e0, &args![]).u32(), 0x11);
        assert_eq!(e.call(0x00d6_7960, &args![]).u32(), 0x22);
    }

    #[test]
    fn fn_00d67970_adds_the_adjusted_listeners() {
        let mut e = engine();
        let log: Log = Rc::default();
        record(&mut e, &log, WORLD_ADD_POST_SIMULATION_LISTENER, 0);
        record(&mut e, &log, WORLD_ADD_ISLAND_ACTIVATION_LISTENER, 0);
        e.call(0x00d6_7970, &args![0x2020u32, 0x5000u32]);
        e.call(0x00d6_7970, &args![0x20u32, 0x5000u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![0x5000, 0x202c]),
                (WORLD_ADD_ISLAND_ACTIVATION_LISTENER, vec![0x5000, 0x2028]),
                (WORLD_ADD_POST_SIMULATION_LISTENER, vec![0x5000, 0]),
                (WORLD_ADD_ISLAND_ACTIVATION_LISTENER, vec![0x5000, 0]),
            ]
        );
    }

    fn largest(e: &mut Engine, values: [f32; 4]) -> i32 {
        let at = e.mem.alloc(16);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
        e.call(0x00d6_79c0, &args![at]).i32()
    }

    #[test]
    fn fn_00d679c0_finds_the_largest_component_and_adds_3_when_it_is_positive() {
        let mut e = engine();
        assert_eq!(largest(&mut e, [3.0, 1.0, 2.0, 0.0]), 3);
        assert_eq!(largest(&mut e, [-3.0, 1.0, 2.0, 0.0]), 0);
        assert_eq!(largest(&mut e, [1.0, -5.0, 2.0, 0.0]), 1);
        assert_eq!(largest(&mut e, [1.0, 5.0, 2.0, 0.0]), 4);
        assert_eq!(largest(&mut e, [1.0, 2.0, -3.0, 0.0]), 2);
        assert_eq!(largest(&mut e, [1.0, 2.0, 3.0, 0.0]), 5);
        // Ties keep the lower index.
        assert_eq!(largest(&mut e, [1.0, 1.0, 1.0, 0.0]), 3);
        assert_eq!(largest(&mut e, [0.0, 2.0, 2.0, 0.0]), 4);
        // Zero is not above zero.
        assert_eq!(largest(&mut e, [0.0, 0.0, 0.0, 9.0]), 0);
        assert_eq!(largest(&mut e, [-0.0, 0.0, 0.0, 9.0]), 0);
    }

    #[test]
    fn fn_00d679c0_treats_a_nan_as_unordered() {
        let mut e = engine();
        // `a` NaN: the comparisons are unordered, so index 0, and NaN is not above zero.
        assert_eq!(largest(&mut e, [f32::NAN, 1.0, 2.0, 0.0]), 0);
        // `b` NaN: unordered keeps the first branch, then `c` beats `a`.
        assert_eq!(largest(&mut e, [1.0, f32::NAN, 2.0, 0.0]), 5);
        // `c` NaN: unordered, the lower index wins.
        assert_eq!(largest(&mut e, [1.0, 2.0, f32::NAN, 0.0]), 4);
    }

    const TARGET: u32 = 0x0900_0100;
    const TARGET_VTABLE: u32 = 0x0900_0120;
    const SHAPE: u32 = 0x0900_0140;
    const SHAPE_VTABLE: u32 = 0x0900_0160;
    const DRAW_FIRST: u32 = 0x0900_0201;
    const DRAW_SECOND: u32 = 0x0900_0202;
    const GET_BOX: u32 = 0x0900_0203;
    type Points = Rc<RefCell<Vec<(u32, Vec<u32>, [f32; 4])>>>;

    /// Doubles for the arrow drawing: a shape whose box is `[-1, -2, -3]` to `[3, 4, 5]`, the
    /// four constant floats `directions`, and two draw methods that note their arguments and
    /// the vector their second (first for the second method) argument points at.
    fn arrows(directions: [f32; 4]) -> (Engine, Points, u32) {
        let mut e = engine();
        e.map(0x0900_0000, 0x1000);
        for (i, v) in directions.iter().enumerate() {
            e.mem.set_f32(ARROW_DIRECTIONS + 4 * i as u32, *v);
        }
        e.mem.set_u32(TARGET, TARGET_VTABLE);
        e.mem.set_u32(TARGET_VTABLE + 0x30, DRAW_FIRST);
        e.mem.set_u32(TARGET_VTABLE + 0x24, DRAW_SECOND);
        e.mem.set_u32(SHAPE, SHAPE_VTABLE);
        e.mem.set_u32(SHAPE_VTABLE + 0x1c, GET_BOX);
        let points: Points = Rc::default();
        let seen = Rc::clone(&points);
        e.register_double(DRAW_FIRST, move |e, a| {
            let at = a[2];
            let lanes = std::array::from_fn(|i| e.mem.f32(at + 4 * i as u32));
            seen.borrow_mut().push((DRAW_FIRST, a.to_vec(), lanes));
            Ret::default()
        });
        let seen = Rc::clone(&points);
        e.register_double(DRAW_SECOND, move |e, a| {
            let at = a[1];
            let lanes = std::array::from_fn(|i| e.mem.f32(at + 4 * i as u32));
            seen.borrow_mut().push((DRAW_SECOND, a.to_vec(), lanes));
            Ret::default()
        });
        e.register_double(GET_BOX, move |e, a| {
            // The transform, the tolerance and where the box goes.
            assert_eq!(a[2], 0);
            for (i, v) in [-1.0f32, -2.0, -3.0, 0.0, 3.0, 4.0, 5.0, 0.0]
                .iter()
                .enumerate()
            {
                e.mem.set_f32(a[3] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        let entity = e.mem.alloc(0x200);
        e.mem.set_u32(entity + 0x10, SHAPE);
        (e, points, entity)
    }

    #[test]
    fn fn_00d67a50_draws_an_arrow_from_the_centre_of_the_face() {
        let (mut e, points, entity) = arrows([0.0, 0.0, 0.0, 7.0]);
        // Face 4: face / 3 = 1 and face % 3 = 1: the direction is [0, 1 - 2, 0, 7].
        e.call(
            0x00d6_7a50,
            &args![TARGET, 0x22u32, entity, 4i32, 0x55u32, 0x66u32, 2.0f32, 0u8],
        );
        let seen = points.borrow();
        assert_eq!(seen.len(), 1);
        // Centre [1, 1, 1, 0], half extents [2, 3, 4, 0]: the distance is |3 * -1| = 3, so the
        // arrow starts at centre + 3 * L = [1, -2, 1, 21] and ends 2 * L further.
        assert_eq!(seen[0].0, DRAW_FIRST);
        assert_eq!(seen[0].1[0], TARGET);
        assert_eq!(seen[0].1[1], 0x55);
        assert_eq!(seen[0].1[3], 0x66);
        assert_eq!(seen[0].1[4], entity + 0x10);
        assert_eq!(seen[0].1[5], 0x22);
        assert_eq!(seen[0].2, [1.0, -4.0, 1.0, 35.0]);
    }

    #[test]
    fn fn_00d67a50_with_the_flag_also_draws_the_start_point() {
        let (mut e, points, entity) = arrows([0.0, 0.0, 0.0, 7.0]);
        // Face 0: the direction is [1, 0, 0, 7]; distance |2| = 2; start [3, 1, 1, 14].
        e.call(
            0x00d6_7a50,
            &args![TARGET, 0x22u32, entity, 0i32, 0x55u32, 0x66u32, 1.0f32, 1u8],
        );
        let seen = points.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].2, [4.0, 1.0, 1.0, 21.0]);
        assert_eq!(seen[1].0, DRAW_SECOND);
        assert_eq!(seen[1].1[0], TARGET);
        assert_eq!(seen[1].1[2], entity + 0x140);
        assert_eq!(seen[1].1[3], 0x66);
        assert_eq!(seen[1].1[4], entity + 0x10);
        assert_eq!(seen[1].1[5], 0x22);
        assert_eq!(seen[1].2, [3.0, 1.0, 1.0, 14.0]);
    }

    #[test]
    fn fn_00d67a50_a_negative_face_writes_below_the_direction_and_leaves_it_alone() {
        let (mut e, points, entity) = arrows([0.0, 0.0, 0.0, 7.0]);
        // Face -1: face / 3 = 0 and face % 3 = -1, so the lane written is before L: L is the
        // constant [0, 0, 0, 7] and the distance 0; the start is the centre [1, 1, 1, 0].
        e.call(
            0x00d6_7a50,
            &args![TARGET, 0u32, entity, -1i32, 0u32, 0u32, 0.0f32, 1u8],
        );
        let seen = points.borrow();
        assert_eq!(seen[1].2, [1.0, 1.0, 1.0, 0.0]);
    }

    #[test]
    fn fn_00d67a50_a_nan_sum_gives_a_positive_distance() {
        let (mut e, points, entity) = arrows([f32::NAN, 0.0, 0.0, 0.0]);
        e.call(
            0x00d6_7a50,
            &args![TARGET, 0u32, entity, 4i32, 0u32, 0u32, 0.0f32, 1u8],
        );
        let seen = points.borrow();
        // The NaN is the first lane of the direction: the sum is NaN (sign cleared by FABS),
        // and every lane of the start point that touches it is NaN.
        assert!(seen[1].2[0].is_nan());
        assert!(seen[1].2[1].is_nan());
    }

    #[test]
    fn fn_00d67b90_and_00d67bc0_take_the_entity_out_of_the_listener_and_the_map() {
        let mut e = engine();
        let log: Log = Rc::default();
        record(&mut e, &log, ENTITY_REMOVE_LISTENER, 0);
        record(&mut e, &log, MAP_REMOVE, 0);
        e.call(0x00d6_7b90, &args![0x1000u32, 0x77u32]);
        e.call(0x00d6_7bc0, &args![0x30u32, 0x78u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                (ENTITY_REMOVE_LISTENER, vec![0x77, 0x1000]),
                (MAP_REMOVE, vec![0x1004, 0x77]),
                (ENTITY_REMOVE_LISTENER, vec![0x78, 0]),
                (MAP_REMOVE, vec![0x34, 0x78]),
            ]
        );
    }

    #[test]
    fn fn_00d67bf0_calls_slot_4_per_world_and_stops_when_the_container_goes() {
        let mut e = engine();
        e.map(0x0900_0000, 0x1000);
        let log: Log = Rc::default();
        // The sub-object at this + 0x18 has the vtable at 0x09000008 (slot 4 at 0x0900000c).
        e.mem.set_u32(0x0900_000c, 0x0900_0011);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 0x0900_0008);
        let container = e.mem.alloc(0x80);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, 0x1111);
        e.mem.set_u32(list + 4, 0x2222);
        e.mem.set_u32(container + 0x70, 2);
        e.mem.set_u32(container + 0x6c, list);
        e.mem.set_u32(this + 0x1c, container);
        record(&mut e, &log, 0x0900_0011, 0);
        e.call(0x00d6_7bf0, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (0x0900_0011, vec![this + 0x18, 0x1111]),
                (0x0900_0011, vec![this + 0x18, 0x2222]),
            ]
        );
        // A callback that drops the container ends the loop after its first call.
        log.borrow_mut().clear();
        let seen = Rc::clone(&log);
        e.register_double(0x0900_0011, move |e, a| {
            seen.borrow_mut().push((0x0900_0011, a.to_vec()));
            e.mem.set_u32(a[0] - 0x18 + 0x1c, 0);
            Ret::default()
        });
        e.call(0x00d6_7bf0, &args![this]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn fn_00d67c30_counts_known_entities_and_registers_new_ones() {
        let mut e = engine();
        let log: Log = Rc::default();
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0x5555);
        record(&mut e, &log, MEMORY_ROUTER, router);
        record(&mut e, &log, MAP_INSERT, 0);
        record(&mut e, &log, ENTITY_ADD_LISTENER, 0);
        let this = e.mem.alloc(0x40);
        // The map: data at +0xc (slots of 8 bytes: key, value), hash mask 3 at +0x14.
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, data);
        e.mem.set_u32(this + 0x14, 3);
        e.mem.set_u32(data + 8 + 4, 0x0100_0001);
        e.mem.set_u32(data + 16 + 4, 0xfeff_ffff);
        let seen = Rc::clone(&log);
        e.register_double(MAP_FIND_KEY, move |_, a| {
            seen.borrow_mut().push((MAP_FIND_KEY, a.to_vec()));
            let index = match a[1] {
                0xa1 => 4,
                0xa2 => 1,
                _ => 2,
            };
            Ret {
                eax: index,
                ..Ret::default()
            }
        });
        let event = e.mem.alloc(0x20);
        e.mem.set_u32(event + 4, 0xa1);
        e.mem.set_u32(event + 8, 0xa2);
        e.call(0x00d6_7c30, &args![this, event]);
        let map = this + 0xc;
        assert_eq!(
            *log.borrow(),
            vec![
                (MAP_FIND_KEY, vec![map, 0xa1]),
                (MEMORY_ROUTER, vec![]),
                (MAP_INSERT, vec![map, 0x5555, 0xa1, 0x0100_0001]),
                (ENTITY_ADD_LISTENER, vec![0xa1, this + 8]),
                (MAP_FIND_KEY, vec![map, 0xa2]),
            ]
        );
        // The known key's two counters went up by one.
        assert_eq!(e.mem.u32(data + 8 + 4), 0x0200_0002);
        // Saturation: the top byte stops at 0xff and the low 24 bits at 0xffffff.
        log.borrow_mut().clear();
        e.mem.set_u32(event + 4, 0xa3);
        e.mem.set_u32(event + 8, 0xa3);
        e.call(0x00d6_7c30, &args![this, event]);
        assert_eq!(e.mem.u32(data + 16 + 4), 0xffff_ffff);
        // A positive count at +0x10 skips everything.
        log.borrow_mut().clear();
        e.mem.set_u32(event + 0x10, 1);
        e.call(0x00d6_7c30, &args![this, event]);
        assert!(log.borrow().is_empty());
    }
}
