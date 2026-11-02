//! `Havok SDK/hkpmoppcachedshapemediator.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00dd5de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00dd_5d90`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00dd5de0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00dd_5d90, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkArrayUtil::_reserve` (`00c90940`, cdecl `(allocator, array, element size)`): grows the
/// array by the allocator's policy.
const ARRAY_RESERVE: u32 = 0x00c9_0940;
/// The vtable written by the constructor and destructor (`010d7be4`) and the base one.
const VTABLE_MEDIATOR: u32 = 0x010d_7be4;
const VTABLE_BASE: u32 = 0x0102_e388;
/// Bytes of one record of the extents array at `+8` (a word and 13 min/max pairs), and of one
/// primitive in the arrays handed to the mediator.
const RECORD_SIZE: u32 = 0x6c;
const PRIMITIVE_SIZE: u32 = 0x14;

// Translated from 00dd55d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCachedShapeMediator::getPrimitiveProperties` (Xbox PDB), `RET 8` with the primitive
/// unread: stores 0 in the word at `properties` and returns 1.
pub fn hkp_mopp_cached_shape_mediator_get_primitive_properties(
    e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    properties: Ptr,
) -> u32 {
    e.mem.set_u32(properties.addr(), 0);
    1
}

// Translated from 00dd55f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCachedShapeMediator::getPrimitives` (Xbox PDB), `RET 4`: walks the keys of the
/// source at `+0x14` (first key from slot `8`, next from slot `0xC`) and writes, for each of
/// the `+0x18` primitives, the key and its index to the first two words of a 20-byte entry
/// of `output`.
pub fn hkp_mopp_cached_shape_mediator_get_primitives(e: &mut Engine, this: Ptr, output: Ptr) {
    let me = this.addr();
    let source = e.mem.u32(me + 0x14);
    let count = e.mem.u32(me + 0x18) as i32;
    let mut key = e.vcall(source, 8, &args![]).u32();
    let mut entry = output.addr();
    for index in 0..count {
        e.mem.set_u32(entry, key);
        e.mem.set_u32(entry + 4, index as u32);
        entry = entry.wrapping_add(PRIMITIVE_SIZE);
        let source = e.mem.u32(me + 0x14);
        key = e.vcall(source, 0xc, &args![key]).u32();
    }
}

// Translated from 00dd5630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCachedShapeMediator::splitPrimitive` (Xbox PDB), `RET 0x14` with only the first and
/// the last of its five words read: copies the 20-byte primitive at `primitive` to `output`.
pub fn hkp_mopp_cached_shape_mediator_split_primitive(
    e: &mut Engine,
    _this: Ptr,
    primitive: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    output: Ptr,
) {
    for word in 0..5 {
        let value = e.mem.u32(primitive.addr() + 4 * word);
        e.mem.set_u32(output.addr() + 4 * word, value);
    }
}

// Translated from 00dd5660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCachedShapeMediator::getNumPrimitives` (Xbox PDB), `__fastcall`: the count at
/// `+0x18`.
pub fn hkp_mopp_cached_shape_mediator_get_num_primitives(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x18)
}

/// The two floats of the record `key` of the extents array at `this + 8`, for the axis whose
/// byte offset is `axis_offset`, each through `FLD`/`FSTP`.
fn extents_pair(e: &Engine, this: u32, axis_offset: u32, key: u32) -> (f32, f32) {
    let base = e.mem.u32(this + 8);
    let at = key
        .wrapping_mul(RECORD_SIZE)
        .wrapping_add(axis_offset)
        .wrapping_add(base);
    (x87_quiet(e.mem.f32(at + 4)), x87_quiet(e.mem.f32(at + 8)))
}

/// Folds a pair into the running minimum and maximum the way the compares of the exe do: the
/// minimum is replaced unless it is below the value, the maximum unless it is above, so a NaN
/// value always replaces both.
fn fold_extents(low: &mut f32, high: &mut f32, value_low: f32, value_high: f32) {
    if !(*low < value_low) {
        *low = value_low;
    }
    if !(*high > value_high) {
        *high = value_high;
    }
}

// Translated from 00dd5670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCachedShapeMediator::projectPrimitives` (Xbox PDB), `RET 0x18` with
/// `(_, axis, primitives, count, out_min, out_max)`: for each of `count` primitives (20 bytes,
/// the first starting at `primitives + 4`, its first word the key of a record of the extents
/// array at `this + 8`), reads the pair of floats at `+4` and `+8` of the record, offset by
/// `axis * 8`, stores them in words 1 and 2 of the primitive, and folds them into the
/// minimum of the first and the maximum of the second, written to `out_min` and `out_max`.
/// The first primitive is always read, even when `count` is not above 0.
pub fn hkp_mopp_cached_shape_mediator_project_primitives(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    axis: i32,
    primitives: Ptr,
    count: i32,
    out_min: Ptr,
    out_max: Ptr,
) {
    let me = this.addr();
    let axis_offset = (axis as u32).wrapping_mul(8);
    let start = primitives.addr();
    let key = e.mem.u32(start + 4);
    let (mut low, mut high) = extents_pair(e, me, axis_offset, key);
    e.mem.set_f32(start + 8, low);
    e.mem.set_f32(start + 0xc, high);
    for index in 1..count.max(1) as u32 {
        let entry = start + 4 + PRIMITIVE_SIZE * index;
        let key = e.mem.u32(entry);
        let (value_low, value_high) = extents_pair(e, me, axis_offset, key);
        e.mem.set_f32(entry + 4, value_low);
        e.mem.set_f32(entry + 8, value_high);
        fold_extents(&mut low, &mut high, value_low, value_high);
    }
    e.mem.set_f32(out_min.addr(), low);
    e.mem.set_f32(out_max.addr(), high);
}

// Translated from 00dd58d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCachedShapeMediator::findExtents` (Xbox PDB), `RET 0x18` with
/// `(_, axis, primitives, count, out_min, out_max)`: the same walk as `projectPrimitives`
/// without storing the pairs in the primitives. The first primitive is always read.
pub fn hkp_mopp_cached_shape_mediator_find_extents(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    axis: i32,
    primitives: Ptr,
    count: i32,
    out_min: Ptr,
    out_max: Ptr,
) {
    let me = this.addr();
    let axis_offset = (axis as u32).wrapping_mul(8);
    let start = primitives.addr();
    let key = e.mem.u32(start + 4);
    let (mut low, mut high) = extents_pair(e, me, axis_offset, key);
    for index in 1..count.max(1) as u32 {
        let key = e.mem.u32(start + 4 + PRIMITIVE_SIZE * index);
        let (value_low, value_high) = extents_pair(e, me, axis_offset, key);
        fold_extents(&mut low, &mut high, value_low, value_high);
    }
    e.mem.set_f32(out_min.addr(), low);
    e.mem.set_f32(out_max.addr(), high);
}

// Translated from 00dd5b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCachedShapeMediator::addConvexShape` (Xbox PDB), `RET 0x10`
/// `(shape, value, directions, direction_count)`: asks the shape for its vertex count (slot
/// `0x2C`), takes room for the vertices from the memory router's bump pointer (its first
/// word, in steps of 128 bytes), lets the shape fill them (slot `0x30`), appends a record to
/// the array at `this + 8` (size at `+0xC`, capacity word at `+0x10`, 0x6C bytes per record)
/// and writes the value to its first word. For each direction (32 bytes apart, four
/// floats) the record gets the minimum and maximum over the vertices of `dot - w` and
/// `dot + w` (`dot` over three lanes with the `l2 + (l1 + l0)` order, `w` the fourth lane
/// of the vertex), as a pair after the pair of the previous direction; with no vertices the
/// pair is `(0, 0)`. Finally gives the bump pointer back.
pub fn hkp_mopp_cached_shape_mediator_add_convex_shape(
    e: &mut Engine,
    this: Ptr,
    shape: Ptr,
    value: u32,
    directions: Ptr,
    direction_count: i32,
) {
    let me = this.addr();
    let s = shape.addr();
    let vertex_count = e.vcall(s, 0x2c, &args![]).u32() as i32;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let vertices = e.mem.u32(router);
    let rounded = (vertex_count as u32).wrapping_shl(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, rounded.wrapping_add(vertices));
    e.vcall(s, 0x30, &args![vertices]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let array = me + 8;
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE, &args![allocator, array, RECORD_SIZE]);
    }
    let size = e.mem.u32(array + 4);
    let record = size
        .wrapping_mul(RECORD_SIZE)
        .wrapping_add(e.mem.u32(array));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.mem.set_u32(record, value);
    if direction_count > 0 {
        let mut direction = directions.addr();
        let mut pair = record + 4;
        for _ in 0..direction_count {
            let (low, high) = if vertex_count > 0 {
                direction_extents(e, vertices, vertex_count, direction)
            } else {
                (0.0, 0.0)
            };
            e.mem.set_f32(pair, low);
            e.mem.set_f32(pair + 4, high);
            direction = direction.wrapping_add(0x20);
            pair += 8;
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, vertices);
}

/// The minimum of `dot - w` and the maximum of `dot + w` over the `count` vertices at
/// `vertices` (16 bytes each) for the direction at `direction`, with the SSE dot and the x87
/// sum and difference rounded to `f32` at the stores.
fn direction_extents(e: &Engine, vertices: u32, count: i32, direction: u32) -> (f32, f32) {
    let dir: [f32; 4] = std::array::from_fn(|lane| e.mem.f32(direction + 4 * lane as u32));
    let bounds = |vertex: u32| {
        let products: [f32; 4] =
            std::array::from_fn(|lane| e.mem.f32(vertex + 4 * lane as u32) * dir[lane]);
        let dot = products[2] + (products[1] + products[0]);
        let w = x87_quiet(e.mem.f32(vertex + 0xc));
        (
            (f64::from(dot) - f64::from(w)) as f32,
            (f64::from(dot) + f64::from(w)) as f32,
        )
    };
    let (mut low, mut high) = bounds(vertices);
    for index in 1..count as u32 {
        let (candidate_low, candidate_high) = bounds(vertices + 16 * index);
        if low > candidate_low {
            low = candidate_low;
        }
        if high < candidate_high {
            high = candidate_high;
        }
    }
    (low, high)
}

// Translated from 00dd5cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMoppCachedShapeMediator::setSplittingPlaneDirections` (Xbox PDB), `RET 8`
/// `(directions, direction_count)`: walks the keys of the source at `+0x14` (count from slot
/// 4, first key from slot 8, shape of a key from slot `0x14` with a 512-byte buffer, next
/// key from slot `0xC`) and for each shape whose slot `0x14` answers true adds it with
/// `addConvexShape(shape, key, directions, direction_count)`.
pub fn hkp_mopp_cached_shape_mediator_set_splitting_plane_directions(
    e: &mut Engine,
    this: Ptr,
    directions: Ptr,
    direction_count: u32,
) {
    let me = this.addr();
    e.with_stack(0x200, |e, buffer| {
        let source = e.mem.u32(me + 0x14);
        let mut remaining = e.vcall(source, 4, &args![]).u32() as i32;
        let source = e.mem.u32(me + 0x14);
        let mut key = e.vcall(source, 8, &args![]).u32();
        if remaining > 0 {
            loop {
                let source = e.mem.u32(me + 0x14);
                let shape = e.vcall(source, 0x14, &args![key, buffer]).u32();
                if e.vcall(shape, 0x14, &args![]).bool() {
                    e.call(
                        0x00dd_5b00,
                        &args![this, shape, key, directions, direction_count],
                    );
                }
                let source = e.mem.u32(me + 0x14);
                key = e.vcall(source, 0xc, &args![key]).u32();
                remaining -= 1;
                if remaining == 0 {
                    break;
                }
            }
        }
    });
}

// Translated from 00dd5d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor (the engine map has no name), `RET 4`: sets the 16-bit word at `+6` to 1, the
/// vtable `010d7be4`, an empty array at `+8` (data 0, size 0, capacity word `0x80000000`),
/// the source at `+0x14` and the count from the source's slot 4 at `+0x18`. Returns `this`.
pub fn fn_00dd5d50(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me, VTABLE_MEDIATOR);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 0);
    e.mem.set_u32(me + 0x10, 0x8000_0000);
    e.mem.set_u32(me + 0x14, source.addr());
    let count = e.vcall(source.addr(), 4, &args![]).u32();
    e.mem.set_u32(me + 0x18, count);
    this
}

// Translated from 00dd5d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor (the engine map has no name), `__fastcall`: clears the source at `+0x14` and the
/// size at `+0xC`, frees the extents array unless it is not owned (0x6C bytes per
/// element), and sets the base vtable.
pub fn fn_00dd5d90(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE_MEDIATOR);
    e.mem.set_u32(me + 0x14, 0);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(me + 0x10);
    e.mem.set_u32(me + 0xc, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(me + 8);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(RECORD_SIZE)],
        );
    }
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0x10, 0x8000_0000);
    e.mem.set_u32(me, VTABLE_BASE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00dd_5de0, deleting_destructor_00dd5de0(Ptr, u8) -> Ptr),
        entry!(
            0x00dd_55d0,
            hkp_mopp_cached_shape_mediator_get_primitive_properties(Ptr, u32, Ptr) -> u32
        ),
        entry!(
            0x00dd_55f0,
            hkp_mopp_cached_shape_mediator_get_primitives(Ptr, Ptr)
        ),
        entry!(
            0x00dd_5630,
            hkp_mopp_cached_shape_mediator_split_primitive(Ptr, Ptr, u32, u32, u32, Ptr)
        ),
        entry!(
            0x00dd_5660,
            hkp_mopp_cached_shape_mediator_get_num_primitives(Ptr) -> u32
        ),
        entry!(
            0x00dd_5670,
            hkp_mopp_cached_shape_mediator_project_primitives(Ptr, u32, i32, Ptr, i32, Ptr, Ptr)
        ),
        entry!(
            0x00dd_58d0,
            hkp_mopp_cached_shape_mediator_find_extents(Ptr, u32, i32, Ptr, i32, Ptr, Ptr)
        ),
        entry!(
            0x00dd_5b00,
            hkp_mopp_cached_shape_mediator_add_convex_shape(Ptr, Ptr, u32, Ptr, i32)
        ),
        entry!(
            0x00dd_5cd0,
            hkp_mopp_cached_shape_mediator_set_splitting_plane_directions(Ptr, Ptr, u32)
        ),
        entry!(0x00dd_5d50, fn_00dd5d50(Ptr, Ptr) -> Ptr),
        entry!(0x00dd_5d90, fn_00dd5d90(Ptr)),
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
    fn deleting_destructor_00dd5de0_frees_only_when_bit_zero_is_set() {
        check(0x00dd_5de0, 0x00dd_5d90);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    const SOURCE_COUNT: u32 = 0x0900_0401;
    const SOURCE_FIRST: u32 = 0x0900_0402;
    const SOURCE_NEXT: u32 = 0x0900_0403;
    const SOURCE_SHAPE: u32 = 0x0900_0404;
    const SHAPE_COUNT: u32 = 0x0900_0405;
    const SHAPE_FILL: u32 = 0x0900_0406;
    const SHAPE_FLAG: u32 = 0x0900_0407;
    const FREE_SLOT: u32 = 0x0900_0408;

    fn push(log: &Log, row: Vec<u32>) {
        log.borrow_mut().push(row);
    }

    fn log_double(e: &mut Engine, addr: u32, log: &Log, tag: u32) {
        let sink = Rc::clone(log);
        e.register_double(addr, move |_, a| {
            let mut row = vec![tag];
            row.extend_from_slice(a);
            push(&sink, row);
            Ret::default()
        });
    }

    /// An object whose first word points at a vtable of 32 slots filled from `slots`.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn the_primitive_properties_are_cleared_and_the_answer_is_one() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let properties = e.mem.alloc(8);
        e.mem.set_u32(properties, 0xffff);
        let got = e
            .call(0x00dd55d0, &args![this, 0x1234u32, properties])
            .u32();
        assert_eq!(got, 1);
        assert_eq!(e.mem.u32(properties), 0);
    }

    #[test]
    fn get_primitives_numbers_the_keys_of_the_source() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        e.register_double(SOURCE_FIRST, |_, _| Ret {
            eax: 100,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SOURCE_NEXT, move |_, a| {
            push(&sink, a.to_vec());
            Ret {
                eax: a[1] + 1,
                ..Ret::default()
            }
        });
        let source = object_with_slots(&mut e, &[(8, SOURCE_FIRST), (0xc, SOURCE_NEXT)]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, source);
        e.mem.set_u32(this + 0x18, 3);
        let output = e.mem.alloc(0x80);
        e.mem.set_u32(output + 8, 0x7777);
        e.call(0x00dd55f0, &args![this, output]);
        let entries: Vec<(u32, u32)> = (0..3)
            .map(|i| {
                (
                    e.mem.u32(output + 0x14 * i),
                    e.mem.u32(output + 0x14 * i + 4),
                )
            })
            .collect();
        assert_eq!(entries, vec![(100, 0), (101, 1), (102, 2)]);
        // The words in between are left alone.
        assert_eq!(e.mem.u32(output + 8), 0x7777);
        // Next is asked for after every entry, the last one too.
        assert_eq!(
            *log.borrow(),
            vec![vec![source, 100], vec![source, 101], vec![source, 102],]
        );
        // With no primitives only the first key is asked for.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x00dd55f0, &args![this, output]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn split_primitive_copies_twenty_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let primitive = e.mem.alloc(0x20);
        for word in 0..5 {
            e.mem.set_u32(primitive + 4 * word, 0x10 + word);
        }
        let output = e.mem.alloc(0x20);
        e.mem.set_u32(output + 0x14, 0x99);
        e.call(
            0x00dd5630,
            &args![this, primitive, 1u32, 2u32, 3u32, output],
        );
        let copied: Vec<u32> = (0..6).map(|i| e.mem.u32(output + 4 * i)).collect();
        assert_eq!(copied, vec![0x10, 0x11, 0x12, 0x13, 0x14, 0x99]);
    }

    #[test]
    fn the_number_of_primitives_is_the_word_at_0x18() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 42);
        assert_eq!(e.call(0x00dd5660, &args![this]).u32(), 42);
    }

    /// A mediator whose extents table has 8 records; record `k` holds `(k, 10 + k)` for
    /// axis 0 and the given pair for axis 1. The primitives array has one entry per key given.
    fn extents_rig(pairs: &[(f32, f32)]) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let table = e.mem.alloc(RECORD_SIZE * 8);
        for (k, (low, high)) in pairs.iter().enumerate() {
            let record = table + RECORD_SIZE * k as u32;
            e.mem.set_f32(record + 4, k as f32);
            e.mem.set_f32(record + 8, 10.0 + k as f32);
            e.mem.set_f32(record + 12, *low);
            e.mem.set_f32(record + 16, *high);
        }
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, table);
        let primitives = e.mem.alloc(4 + PRIMITIVE_SIZE * 8);
        for k in 0..pairs.len() as u32 {
            let entry = primitives + 4 + PRIMITIVE_SIZE * k;
            e.mem.set_u32(entry, k);
            e.mem.set_u32(entry + 4, 0xdead);
            e.mem.set_u32(entry + 8, 0xbeef);
        }
        (e, this, primitives)
    }

    fn run_extents(
        e: &mut Engine,
        address: u32,
        this: u32,
        axis: i32,
        primitives: u32,
        count: i32,
    ) -> (f32, f32) {
        let out = e.mem.alloc(8);
        e.call(
            address,
            &args![this, 0x55u32, axis, primitives, count, out, out + 4],
        );
        (e.mem.f32(out), e.mem.f32(out + 4))
    }

    #[test]
    fn projecting_stores_each_pair_and_returns_the_extremes() {
        let pairs = [
            (3.0, 10.0),
            (1.0, 12.0),
            (4.0, 9.0),
            (1.5, 11.0),
            (5.0, 20.0),
            (0.5, 2.0),
        ];
        let (mut e, this, primitives) = extents_rig(&pairs);
        let got = run_extents(&mut e, 0x00dd5670, this, 1, primitives, 6);
        assert_eq!(got, (0.5, 20.0));
        for (k, (low, high)) in pairs.iter().enumerate() {
            let entry = primitives + 4 + PRIMITIVE_SIZE * k as u32;
            assert_eq!(e.mem.f32(entry + 4), *low);
            assert_eq!(e.mem.f32(entry + 8), *high);
        }
        // Axis 0 reads the first pair of every record.
        let got = run_extents(&mut e, 0x00dd5670, this, 0, primitives, 4);
        assert_eq!(got, (0.0, 13.0));
        assert_eq!(e.mem.f32(primitives + 4 + PRIMITIVE_SIZE + 4), 1.0);
        // Fewer than two primitives: the first one is read and stored regardless.
        for count in [1, 0, -3] {
            let (mut e, this, primitives) = extents_rig(&pairs);
            let got = run_extents(&mut e, 0x00dd5670, this, 1, primitives, count);
            assert_eq!(got, (3.0, 10.0));
            assert_eq!(e.mem.f32(primitives + 8), 3.0);
            assert_eq!(e.mem.f32(primitives + 0xc), 10.0);
            assert_eq!(e.mem.u32(primitives + 4 + PRIMITIVE_SIZE + 4), 0xdead);
        }
    }

    #[test]
    fn finding_extents_leaves_the_primitives_alone() {
        let pairs = [
            (3.0, 10.0),
            (1.0, 12.0),
            (4.0, 9.0),
            (-2.0, 11.0),
            (5.0, 20.0),
        ];
        let (mut e, this, primitives) = extents_rig(&pairs);
        let got = run_extents(&mut e, 0x00dd58d0, this, 1, primitives, 5);
        assert_eq!(got, (-2.0, 20.0));
        assert_eq!(e.mem.u32(primitives + 8), 0xdead);
        assert_eq!(e.mem.u32(primitives + 4 + PRIMITIVE_SIZE + 4), 0xdead);
        let got = run_extents(&mut e, 0x00dd58d0, this, 1, primitives, 2);
        assert_eq!(got, (1.0, 12.0));
    }

    #[test]
    fn a_nan_in_the_last_pair_replaces_the_running_extreme() {
        let pairs = [(3.0, 10.0), (f32::NAN, f32::NAN)];
        for address in [0x00dd5670u32, 0x00dd58d0] {
            let (mut e, this, primitives) = extents_rig(&pairs);
            let (low, high) = run_extents(&mut e, address, this, 1, primitives, 2);
            assert!(low.is_nan() && high.is_nan());
        }
    }

    /// A mediator whose vertex buffer comes from the memory router's bump pointer; the
    /// shape has the vertices `(1, 0, 0, w 0.5)` and `(0, 1, 0, w 0.25)`.
    struct Convex {
        e: Engine,
        this: u32,
        shape: u32,
        router: u32,
        cursor: u32,
        log: Log,
    }

    fn convex_rig(vertex_count: u32) -> Convex {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let cursor = e.mem.alloc(0x400);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, cursor);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register_double(SHAPE_COUNT, move |_, _| Ret {
            eax: vertex_count,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SHAPE_FILL, move |e, a| {
            push(&sink, vec![1, a[1], e.mem.u32(router)]);
            for (vertex, lanes) in [[1.0f32, 0.0, 0.0, 0.5], [0.0, 1.0, 0.0, 0.25]]
                .iter()
                .enumerate()
            {
                for (lane, v) in lanes.iter().enumerate() {
                    e.mem
                        .set_f32(a[1] + 16 * vertex as u32 + 4 * lane as u32, *v);
                }
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            push(&sink, vec![2, a[0], a[1], a[2]]);
            let data = e.mem.alloc(4 * RECORD_SIZE);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let shape = object_with_slots(&mut e, &[(0x2c, SHAPE_COUNT), (0x30, SHAPE_FILL)]);
        let this = e.mem.alloc(0x40);
        Convex {
            e,
            this,
            shape,
            router,
            cursor,
            log,
        }
    }

    fn directions(e: &mut Engine) -> u32 {
        let block = e.mem.alloc(0x40);
        for (i, d) in [[1.0f32, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]]
            .iter()
            .enumerate()
        {
            for (lane, v) in d.iter().enumerate() {
                e.mem.set_f32(block + 0x20 * i as u32 + 4 * lane as u32, *v);
            }
        }
        block
    }

    #[test]
    fn a_convex_shape_adds_a_record_of_projected_extents() {
        let mut c = convex_rig(2);
        let dirs = directions(&mut c.e);
        c.e.call(0x00dd5b00, &args![c.this, c.shape, 0xabcu32, dirs, 2i32]);
        let data = c.e.mem.u32(c.this + 8);
        assert_eq!(c.e.mem.u32(c.this + 0xc), 1);
        assert_eq!(c.e.mem.u32(data), 0xabc);
        let pairs: Vec<f32> = (0..4).map(|i| c.e.mem.f32(data + 4 + 4 * i)).collect();
        assert_eq!(pairs, vec![-0.25, 1.5, -0.5, 1.25]);
        // The empty array is made to grow first; the vertices come from the bump pointer,
        // which has moved on by 2 * 16 rounded up to 128 while the shape fills them, and is
        // given back afterwards.
        let log = c.log.borrow();
        assert_eq!(log[0], vec![1, c.cursor, c.cursor + 0x80]);
        assert_eq!(log[1][0], 2);
        assert_eq!(log[1][2..], [c.this + 8, RECORD_SIZE]);
        assert_eq!(c.e.mem.u32(c.router), c.cursor);
        drop(log);
        // The second shape fits: the array has room, no reserve.
        c.log.borrow_mut().clear();
        c.e.call(0x00dd5b00, &args![c.this, c.shape, 0xdefu32, dirs, 1i32]);
        assert_eq!(c.e.mem.u32(c.this + 0xc), 2);
        assert_eq!(c.log.borrow().len(), 1);
        assert_eq!(c.e.mem.u32(data + RECORD_SIZE), 0xdef);
        assert_eq!(c.e.mem.f32(data + RECORD_SIZE + 4), -0.25);
        assert_eq!(c.e.mem.f32(data + RECORD_SIZE + 8), 1.5);
    }

    #[test]
    fn a_shape_without_vertices_gets_empty_extents_and_no_directions_none() {
        let mut c = convex_rig(0);
        let dirs = directions(&mut c.e);
        c.e.call(0x00dd5b00, &args![c.this, c.shape, 1u32, dirs, 2i32]);
        let data = c.e.mem.u32(c.this + 8);
        let pairs: Vec<u32> = (0..4).map(|i| c.e.mem.u32(data + 4 + 4 * i)).collect();
        assert_eq!(pairs, vec![0, 0, 0, 0]);
        // The bump pointer does not move for zero vertices.
        assert_eq!(c.log.borrow()[0], vec![1, c.cursor, c.cursor]);
        // A direction count of 0 leaves the record with only the value.
        c.e.call(0x00dd5b00, &args![c.this, c.shape, 9u32, dirs, 0i32]);
        assert_eq!(c.e.mem.u32(data + RECORD_SIZE), 9);
        assert_eq!(c.e.mem.u32(data + RECORD_SIZE + 4), 0);
    }

    #[test]
    fn a_single_vertex_gives_its_own_bounds() {
        let mut c = convex_rig(1);
        let dirs = directions(&mut c.e);
        c.e.call(0x00dd5b00, &args![c.this, c.shape, 3u32, dirs, 2i32]);
        let data = c.e.mem.u32(c.this + 8);
        let pairs: Vec<f32> = (0..4).map(|i| c.e.mem.f32(data + 4 + 4 * i)).collect();
        // Vertex (1, 0, 0, w 0.5): direction x gives 1 -/+ 0.5, direction y gives 0 -/+ 0.5.
        assert_eq!(pairs, vec![0.5, 1.5, -0.5, 0.5]);
        assert_eq!(c.e.mem.u32(c.router), c.cursor);
    }

    #[test]
    fn the_splitting_directions_go_to_the_convex_shapes_of_the_source() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        e.register_double(SOURCE_COUNT, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register_double(SOURCE_FIRST, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.register_double(SOURCE_NEXT, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        // Key 8 is not convex.
        let convex = object_with_slots(&mut e, &[(0x14, SHAPE_FLAG)]);
        let flat = object_with_slots(&mut e, &[(0x14, SHAPE_FLAG + 1)]);
        e.register_double(SHAPE_FLAG, |_, _| Ret {
            eax: 0xffff_ff01,
            ..Ret::default()
        });
        e.register_double(SHAPE_FLAG + 1, |_, _| Ret::default());
        let sink = Rc::clone(&log);
        e.register_double(SOURCE_SHAPE, move |_, a| {
            push(&sink, vec![1, a[1]]);
            Ret {
                eax: if a[1] == 8 { flat } else { convex },
                ..Ret::default()
            }
        });
        log_double(&mut e, 0x00dd5b00, &log, 2);
        let source = object_with_slots(
            &mut e,
            &[
                (4, SOURCE_COUNT),
                (8, SOURCE_FIRST),
                (0xc, SOURCE_NEXT),
                (0x14, SOURCE_SHAPE),
            ],
        );
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, source);
        e.call(0x00dd5cd0, &args![this, 0x3000u32, 5u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![1, 7],
                vec![2, this, convex, 7, 0x3000, 5],
                vec![1, 8],
                vec![1, 9],
                vec![2, this, convex, 9, 0x3000, 5],
            ]
        );
        // No shapes: only the count and the first key are asked for.
        log.borrow_mut().clear();
        e.register_double(SOURCE_COUNT, |_, _| Ret::default());
        e.call(0x00dd5cd0, &args![this, 0u32, 0u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_constructor_empties_the_array_and_reads_the_count_from_the_source() {
        let mut e = Engine::new();
        e.register_double(SOURCE_COUNT, |_, _| Ret {
            eax: 12,
            ..Ret::default()
        });
        let source = object_with_slots(&mut e, &[(4, SOURCE_COUNT)]);
        let this = e.mem.alloc(0x40);
        let got = e.call(0x00dd5d50, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this), VTABLE_MEDIATOR);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x14), source);
        assert_eq!(e.mem.u32(this + 0x18), 12);
    }

    #[test]
    fn the_destructor_frees_the_records_unless_they_are_not_owned() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, FREE_SLOT, &log, 1);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, 0x5000);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 4);
        e.mem.set_u32(this + 0x14, 0x77);
        e.call(0x00dd5d90, &args![this]);
        assert_eq!(*log.borrow(), vec![vec![1, allocator, 0x5000, 4 * 0x6c]]);
        assert_eq!(e.mem.u32(this), VTABLE_BASE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        log.borrow_mut().clear();
        e.mem.set_u32(this + 8, 0x5000);
        e.mem.set_u32(this + 0x10, 0x8000_0004);
        e.call(0x00dd5d90, &args![this]);
        assert!(log.borrow().is_empty());
    }
}
