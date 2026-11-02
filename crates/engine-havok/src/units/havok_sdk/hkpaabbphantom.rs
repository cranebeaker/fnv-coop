//! `Havok SDK/hkpaabbphantom.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpAabbPhantom` (Xbox PDB, 0xe0 bytes) is a phantom that keeps the collidables overlapping
//! an axis-aligned box. PC offsets (they agree with the Xbox PDB): `m_world` +8, `m_userData`
//! +0x2c, the collidable at +0x10, `m_overlapListeners` +0x8c and `m_phantomListeners` +0x98
//! (`hkArray`: data, size, capacity word), `m_aabb` +0xb0 (two vectors), the array
//! `m_overlappingCollidables` +0xd0 (data, size, capacity word whose low 30 bits are the
//! capacity and whose top bit means "storage not owned") and `m_orderDirty` +0xdc.
//!
//! The ray and cast functions write Havok's profiling records (`RDTSC` time stamps appended to
//! the thread's monitor stream, found with `TlsGetValue` of the index at `0126835c`); the engine
//! has no model of `RDTSC`, so the host clock stands in for the time stamps. SSE code computes
//! lane by lane in `f32`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtrcPhantom"`, `"TthkpAabbPhantom::linearCast"` and the end record.
const LABEL_RAY: u32 = 0x010c_9538;
const LABEL_LINEAR_CAST: u32 = 0x010c_bfd0;
const LABEL_END: u32 = 0x010c_41f8;
/// `"OvrlpCollPtr"`, the statistics name of the overlapping-collidable array.
const NAME_OVERLAPPING: u32 = 0x010c_bff0;
/// The class record `hkpPhantom::calcContentStatistics` receives (`0126902c`).
const PHANTOM_CLASS_RECORD: u32 = 0x0126_902c;
/// The vtable `hkpAabbPhantom` writes (`010cc004`).
const AABB_PHANTOM_VTABLE: u32 = 0x010c_c004;
/// `hkpWorldObject::hkpWorldObject` (`00cd2ce0`, `this`, `(0, 2)`).
const WORLD_OBJECT_CONSTRUCT: u32 = 0x00cd_2ce0;
/// `hkpWorldObject::copyProperties` (`00ca92a0`, `this`, `(source)`).
const COPY_PROPERTIES: u32 = 0x00ca_92a0;
/// `hkpPhantom::fireCollidableAdded` (`00ca94c0`, `this`, `(collidable)`) and
/// `fireCollidableRemoved` (`00ca9520`, `this`, `(collidable, flag word)`).
const FIRE_COLLIDABLE_ADDED: u32 = 0x00ca_94c0;
const FIRE_COLLIDABLE_REMOVED: u32 = 0x00ca_9520;
/// `hkpPhantom::updateBroadPhase` (`00d07ec0`, `this`, `(aabb)`).
const UPDATE_BROAD_PHASE: u32 = 0x00d0_7ec0;
/// `hkpPhantom::~hkpPhantom` (`00d08130`), `deallocateInternalArrays` (`00d07e20`) and
/// `calcContentStatistics` (`00d07cf0`, `this`, `(collector, class record)`).
const BASE_DESTRUCTOR: u32 = 0x00d0_8130;
const BASE_DEALLOCATE_INTERNAL_ARRAYS: u32 = 0x00d0_7e20;
const BASE_CALC_CONTENT_STATISTICS: u32 = 0x00d0_7cf0;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl `(pointer, bytes)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkVector4::setRotatedDir` (`00c7f700`, `this` = result, `(matrix, vector)`).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `quickSortRecursive<CollisionDetail, OrderByUid>` (`00cf2ed0`, cdecl `(data, low, high,
/// comparer)`).
const QUICK_SORT_RECURSIVE: u32 = 0x00cf_2ed0;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// The stand-in for `RDTSC`'s low word: the host clock.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// A ray point in the local space of the collidable whose transform (three columns, then the
/// translation at +0x30) is at `transform`: the point minus the translation, then
/// `(y * Y + x * X) + z * Z` per lane with the `SHUFPS` selection the code uses
/// (`X = [c0.x, c1.x, c2.x, c2.y]`, `Y = [c0.y, c1.y, c2.y, c2.w]`,
/// `Z = [c0.z, c1.z, c2.z, c2.w]`).
fn to_local_space(e: &Engine, transform: u32, point: Vec4) -> Vec4 {
    let c0 = load_vec(e, transform);
    let c1 = load_vec(e, transform + 0x10);
    let c2 = load_vec(e, transform + 0x20);
    let translation = load_vec(e, transform + 0x30);
    let d: Vec4 = std::array::from_fn(|lane| point[lane] - translation[lane]);
    let x_lane = [c0[0], c1[0], c2[0], c2[1]];
    let y_lane = [c0[1], c1[1], c2[1], c2[3]];
    let z_lane = [c0[2], c1[2], c2[2], c2[3]];
    std::array::from_fn(|lane| (d[1] * y_lane[lane] + d[0] * x_lane[lane]) + z_lane[lane] * d[2])
}

/// The ray input's filter word (+0x24) and the filter pointer the shapes receive: the world's
/// collision filter plus 0x10 when the input's flag byte (+0x20) is set and the world has a
/// filter, else 0.
fn ray_filter(e: &Engine, this: Ptr, input: Ptr) -> (u32, u32) {
    let filter_info = e.mem.u32(input.addr() + 0x24);
    let mut filter_pointer = 0;
    if e.mem.u8(input.addr() + 0x20) != 0 {
        let world = e.mem.u32(this.addr() + 8);
        let filter = e.mem.u32(world + 0x7c);
        if filter != 0 {
            filter_pointer = filter + 0x10;
        }
    }
    (filter_info, filter_pointer)
}

// Translated from 00cf2460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::getType` (Xbox PDB): the phantom type, 0.
pub fn hkp_aabb_phantom_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00cf2470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::calcAabb` (Xbox PDB): copies the box (`+0xb0`, two vectors) to `out`.
pub fn hkp_aabb_phantom_calc_aabb(e: &mut Engine, this: Ptr, out: Ptr) {
    copy_vec(e, this.addr() + 0xb0, out.addr());
    copy_vec(e, this.addr() + 0xc0, out.addr() + 0x10);
}

// Translated from 00cf24a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::setAabb` (Xbox PDB): copies `aabb` (two vectors) into the box at `+0xb0`,
/// then tells the broad phase (`hkpPhantom::updateBroadPhase`, `00d07ec0`, with the box's
/// address).
pub fn hkp_aabb_phantom_set_aabb(e: &mut Engine, this: Ptr, aabb: Ptr) {
    let own = this.addr() + 0xb0;
    copy_vec(e, aabb.addr(), own);
    copy_vec(e, aabb.addr() + 0x10, own + 0x10);
    e.call(UPDATE_BROAD_PHASE, &args![this, own]);
}

// Translated from 00cf24d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::isOverlappingCollidableAdded` (Xbox PDB): writes 1 to the byte at `out` when
/// `collidable` is in the overlapping array, else 0. Returns `out`.
pub fn hkp_aabb_phantom_is_overlapping_collidable_added(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    collidable: u32,
) -> Ptr {
    let size = e.mem.i32(this.addr() + 0xd4);
    let data = e.mem.u32(this.addr() + 0xd0);
    let mut found = 0u8;
    let mut index = 0i32;
    if size > 0 {
        loop {
            if e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4))) == collidable {
                found = 1;
                break;
            }
            index += 1;
            if index >= size {
                break;
            }
        }
    }
    e.mem.set_u8(out.addr(), found);
    out
}

// Translated from 00cf2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Casts a ray against the phantom's overlapping collidables and gives every hit to
/// `collector` (the `hkpRayHitCollector` variant of `castRay`; the Xbox PDB name is
/// unconfirmed). `input` is the world ray input: from (+0) and to (+0x10) vectors, a flag byte
/// at +0x20 (when set, the world's collision filter plus 0x10 is passed on) and the filter info
/// word at +0x24. For each collidable with a shape the two points are moved into its local space
/// ([`to_local_space`]) and the shape's slot `0x24` is called with `(local input, collidable,
/// collector)`; the local input is the two points, the filter info and the filter pointer.
pub fn hkp_aabb_phantom_cast_ray_collector(e: &mut Engine, this: Ptr, input: Ptr, collector: u32) {
    profile_record(e, LABEL_RAY);
    let (filter_info, filter_pointer) = ray_filter(e, this, input);
    let data = e.mem.u32(this.addr() + 0xd0);
    let size = e.mem.i32(this.addr() + 0xd4);
    let from = load_vec(e, input.addr());
    let to = load_vec(e, input.addr() + 0x10);
    let mut index = size.wrapping_sub(1);
    let mut cursor = data;
    while index >= 0 {
        let collidable = e.mem.u32(cursor);
        let shape = e.mem.u32(collidable);
        if shape != 0 {
            let transform = e.mem.u32(collidable + 8);
            let local_from = to_local_space(e, transform, from);
            let local_to = to_local_space(e, transform, to);
            e.with_stack(0x28, |e, local| {
                store_vec(e, local.addr(), local_from);
                store_vec(e, local.addr() + 0x10, local_to);
                e.mem.set_u32(local.addr() + 0x20, filter_info);
                e.mem.set_u32(local.addr() + 0x24, filter_pointer);
                e.vcall(shape, 0x24, &args![local, collidable, collector]);
            });
        }
        cursor = cursor.wrapping_add(4);
        index = index.wrapping_sub(1);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00cf26b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::castRay` (Xbox PDB): as the collector variant (`00cf2510`, which see), but
/// the shape's slot `0x20` is called with `(hit flag byte, local input, output)`; a collidable
/// whose call sets the flag byte becomes the output's hit collidable (`+0x50`). The flag byte
/// is one stack byte for the whole loop: a call that leaves it alone leaves the previous
/// answer. When the output has a hit collidable afterwards, `hkVector4::setRotatedDir`
/// (`00c7f700`) is called with `this` = the output, `(the collidable's transform, the output)`
/// (the normal into world space).
pub fn hkp_aabb_phantom_cast_ray(e: &mut Engine, this: Ptr, input: Ptr, output: Ptr) {
    profile_record(e, LABEL_RAY);
    let (filter_info, filter_pointer) = ray_filter(e, this, input);
    let data = e.mem.u32(this.addr() + 0xd0);
    let size = e.mem.i32(this.addr() + 0xd4);
    let from = load_vec(e, input.addr());
    let to = load_vec(e, input.addr() + 0x10);
    e.with_stack(0x50, |e, block| {
        // The exe's frame: the hit flag byte at +0x1b, the local input at +0x20.
        let flag = block.addr() + 0x1b;
        let local = block.addr() + 0x20;
        let mut index = size.wrapping_sub(1);
        let mut cursor = data;
        while index >= 0 {
            let collidable = e.mem.u32(cursor);
            let shape = e.mem.u32(collidable);
            if shape != 0 {
                let transform = e.mem.u32(collidable + 8);
                let local_from = to_local_space(e, transform, from);
                let local_to = to_local_space(e, transform, to);
                store_vec(e, local, local_from);
                store_vec(e, local + 0x10, local_to);
                e.mem.set_u32(local + 0x20, filter_info);
                e.mem.set_u32(local + 0x24, filter_pointer);
                e.vcall(shape, 0x20, &args![flag, local, output]);
                if e.mem.u8(flag) != 0 {
                    e.mem.set_u32(output.addr() + 0x50, collidable);
                }
            }
            cursor = cursor.wrapping_add(4);
            index = index.wrapping_sub(1);
        }
    });
    let hit_collidable = e.mem.u32(output.addr() + 0x50);
    if hit_collidable != 0 {
        let transform = e.mem.u32(hit_collidable + 8);
        e.call(SET_ROTATED_DIR, &args![output, transform, output]);
    }
    profile_record(e, LABEL_END);
}

// Translated from 00cf2880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::linearCast` (Xbox PDB): builds the linear-cast collision input on its stack
/// from the world's collision input (`world + 0x78`): words +0, +4, +0xc, +0x10, +0x14, the
/// vectors +0x20, +0x30, +0x40 are copied; then the word at +8 is `cast_input`'s word 5, +0x50
/// is `cast_input`'s point minus the collidable's translation (`transform + 0x30`), +0x60 is
/// `cast_input`'s word 4, +0x64 the length of that difference (`SQRTSS` of the lane sums
/// `z + (y + x)`) and +0x68 the world input's word +0x70. The floats go through the x87
/// (`FLD`/`FSTP`), so signalling NaNs are quieted. (The exe also stores a constant at the place
/// of +0x60 first; it is overwritten before anything reads it.) For each overlapping collidable
/// the world filter's `isCollisionEnabled` (second interface at `filter + 8`, slot 4, `(out
/// byte, collidable, other)`, returning the byte's address) is asked, and when the other has a
/// shape, the answer is non-zero and it is not `collidable` itself, the dispatcher's function
/// (`dispatcher + 0x9ac + 0x14 * (byte at dispatcher + (type + 0xd) * 0x20 + other type)`, the
/// types being the dword at shape `+0xc`) is called with `(collidable, other, input, cast
/// collector, start collector)`. The stack words the exe leaves unwritten (+0x18..+0x20,
/// +0x6c) stay zero here.
pub fn hkp_aabb_phantom_linear_cast(
    e: &mut Engine,
    this: Ptr,
    collidable: u32,
    cast_input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    profile_record(e, LABEL_LINEAR_CAST);
    let world = e.mem.u32(this.addr() + 8);
    let world_input = e.mem.u32(world + 0x78);
    let transform = e.mem.u32(collidable + 8);
    let point = load_vec(e, cast_input.addr());
    let translation = load_vec(e, transform + 0x30);
    let delta: Vec4 = std::array::from_fn(|lane| point[lane] - translation[lane]);
    let squares: Vec4 = std::array::from_fn(|lane| delta[lane] * delta[lane]);
    let length = (squares[2] + (squares[1] + squares[0])).sqrt();
    let filter = e.mem.u32(world + 0x7c);
    let dispatcher = e.mem.u32(world + 0x80);
    let count = e.mem.i32(this.addr() + 0xd4);
    e.with_stack(0x90, |e, block| {
        let flag = block.addr() + 0x13;
        let input = block.addr() + 0x20;
        for word in [0u32, 4, 0xc, 0x10, 0x14] {
            let value = e.mem.u32(world_input + word);
            e.mem.set_u32(input + word, value);
        }
        let to_y = x87_quiet_bits(e.mem.u32(cast_input.addr() + 0x14));
        e.mem.set_u32(input + 8, to_y);
        for offset in [0x20u32, 0x30, 0x40] {
            copy_vec(e, world_input + offset, input + offset);
        }
        store_vec(e, input + 0x50, delta);
        let to_x = x87_quiet_bits(e.mem.u32(cast_input.addr() + 0x10));
        e.mem.set_u32(input + 0x60, to_x);
        e.mem.set_f32(input + 0x64, x87_quiet(length));
        let world_word = e.mem.u32(world_input + 0x70);
        e.mem.set_u32(input + 0x68, world_word);
        let mut index = 0;
        while index < count {
            let data = e.mem.u32(this.addr() + 0xd0);
            let other = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            let enabled = e
                .vcall(filter + 8, 4, &args![flag, collidable, other])
                .u32();
            let other_shape = e.mem.u32(other);
            if other_shape != 0 && e.mem.u8(enabled) != 0 && other != collidable {
                let shape = e.mem.u32(collidable);
                let first_type = e.mem.u32(shape + 0xc);
                let second_type = e.mem.u32(other_shape + 0xc);
                let slot = dispatcher
                    .wrapping_add(first_type.wrapping_add(0xd).wrapping_mul(0x20))
                    .wrapping_add(second_type);
                let record = u32::from(e.mem.u8(slot));
                let function = e.mem.u32(dispatcher + 0x9ac + record * 0x14);
                e.call(
                    function,
                    &args![collidable, other, input, cast_collector, start_collector],
                );
            }
            index += 1;
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00cf2a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::removeOverlappingCollidable` (Xbox PDB): looks `collidable` up in the array;
/// `hkpPhantom::fireCollidableRemoved` (`00ca9520`) is told whether it was found (the second
/// argument word is the array size with its low byte replaced by the flag). When found the
/// array shrinks by one, the last element fills the hole (unless it was the last) and the
/// order-dirty byte (`+0xdc`) is set.
pub fn hkp_aabb_phantom_remove_overlapping_collidable(e: &mut Engine, this: Ptr, collidable: u32) {
    let size = e.mem.i32(this.addr() + 0xd4);
    let mut found = -1i32;
    let mut index = 0i32;
    if size > 0 {
        let data = e.mem.u32(this.addr() + 0xd0);
        loop {
            let at = data.wrapping_add((index as u32).wrapping_mul(4));
            if e.mem.u32(at) == collidable {
                found = index;
                break;
            }
            index += 1;
            if index >= size {
                break;
            }
        }
    }
    let flag = (size as u32 & 0xffff_ff00) | u32::from(found >= 0);
    e.call(FIRE_COLLIDABLE_REMOVED, &args![this, collidable, flag]);
    if found >= 0 {
        let new_size = e.mem.i32(this.addr() + 0xd4).wrapping_sub(1);
        e.mem.set_i32(this.addr() + 0xd4, new_size);
        if new_size != found {
            let data = e.mem.u32(this.addr() + 0xd0);
            let last = e
                .mem
                .u32(data.wrapping_add((new_size as u32).wrapping_mul(4)));
            e.mem
                .set_u32(data.wrapping_add((found as u32).wrapping_mul(4)), last);
        }
        e.mem.set_u8(this.addr() + 0xdc, 1);
    }
}

// Translated from 00cf2ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::calcContentStatistics` (Xbox PDB): when the array's capacity word is not
/// negative or the collector's byte at `+0x10` is set, reports the array to the collector (slot
/// `0x14`: `(3, "OvrlpCollPtr", data, size * 4, allocated size)`, the allocated size from
/// `hkStatisticsCollector::getAllocatedSize` of `(data, capacity * 4)`); then the base
/// `hkpPhantom::calcContentStatistics` (`00d07cf0`) gets `(collector, 0126902c)`.
/// The second stack word (`RET 8`; callers pass the class record) is not read.
pub fn hkp_aabb_phantom_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: Ptr,
    _unused_2: u32,
) {
    let capacity_word = e.mem.u32(this.addr() + 0xd8);
    if (capacity_word as i32) >= 0 || e.mem.u8(collector.addr() + 0x10) != 0 {
        let data = e.mem.u32(this.addr() + 0xd0);
        let bytes = (capacity_word & 0x3fff_ffff).wrapping_mul(4);
        let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, bytes]).u32();
        let data = e.mem.u32(this.addr() + 0xd0);
        let used = e.mem.u32(this.addr() + 0xd4).wrapping_mul(4);
        e.vcall(
            collector.addr(),
            0x14,
            &args![3u32, NAME_OVERLAPPING, data, used, allocated],
        );
    }
    e.call(
        BASE_CALC_CONTENT_STATISTICS,
        &args![this, collector, PHANTOM_CLASS_RECORD],
    );
}

// Translated from 00cf2b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::addOverlappingCollidable` (Xbox PDB): `hkpPhantom::fireCollidableAdded`
/// (`00ca94c0`) is asked first; when it returns 0 the collidable is appended to the array
/// (grown with `hkArrayUtil::_reserveMore` of the memory router's allocator when size equals
/// capacity) and the order-dirty byte (`+0xdc`) is set.
pub fn hkp_aabb_phantom_add_overlapping_collidable(e: &mut Engine, this: Ptr, collidable: u32) {
    if e.call(FIRE_COLLIDABLE_ADDED, &args![this, collidable])
        .u32()
        == 0
    {
        let array = this.addr() + 0xd0;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
        }
        let size = e.mem.u32(array + 4);
        let data = e.mem.u32(array);
        e.mem
            .set_u32(data.wrapping_add(size.wrapping_mul(4)), collidable);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
        e.mem.set_u8(this.addr() + 0xdc, 1);
    }
}

// Translated from 00cf2bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::ensureDeterministicOrder` (Xbox PDB, `__fastcall`): when the order-dirty
/// byte (`+0xdc`) is set and the array has more than one element, sorts it with
/// `quickSortRecursive<CollisionDetail, OrderByUid>` (`00cf2ed0`, `(data, 0, size - 1,
/// comparer)`; the comparer word is `this` with its low byte cleared, as the exe passes its
/// stack slot); then clears the byte.
pub fn hkp_aabb_phantom_ensure_deterministic_order(e: &mut Engine, this: Ptr) {
    if e.mem.u8(this.addr() + 0xdc) != 0 {
        let size = e.mem.i32(this.addr() + 0xd4);
        if size > 1 {
            let data = e.mem.u32(this.addr() + 0xd0);
            e.call(
                QUICK_SORT_RECURSIVE,
                &args![data, 0u32, size - 1, this.addr() & 0xffff_ff00],
            );
        }
        e.mem.set_u8(this.addr() + 0xdc, 0);
    }
}

/// Releases the overlapping-collidable array through the memory router's allocator (slot
/// `0x10`, `(data, capacity * 4)`) when its capacity word is not negative, and resets it to
/// empty and not owned. Shared by the destructor and `deallocateInternalArrays`.
fn release_overlapping_array(e: &mut Engine, this: Ptr) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity_word = e.mem.u32(this.addr() + 0xd8);
    e.mem.set_u32(this.addr() + 0xd4, 0);
    if (capacity_word as i32) >= 0 {
        let data = e.mem.u32(this.addr() + 0xd0);
        let bytes = (capacity_word & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(this.addr() + 0xd0, 0);
    e.mem.set_u32(this.addr() + 0xd8, 0x8000_0000);
}

// Translated from 00cf2bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::~hkpAabbPhantom` (Xbox PDB, `__fastcall`): writes the phantom's vtable
/// (`010cc004`), releases the overlapping-collidable array and tail-calls `~hkpPhantom`
/// (`00d08130`); returns that call's `EAX`.
pub fn hkp_aabb_phantom_destruct(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.set_u32(this.addr(), AABB_PHANTOM_VTABLE);
    release_overlapping_array(e, this);
    e.call(BASE_DESTRUCTOR, &args![this]).u32()
}

// Translated from 00cf2c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::deallocateInternalArrays` (Xbox PDB, `__fastcall`): when the overlapping
/// array is empty, releases it as the destructor does; then tail-calls
/// `hkpPhantom::deallocateInternalArrays` (`00d07e20`) and returns its `EAX`.
pub fn hkp_aabb_phantom_deallocate_internal_arrays(e: &mut Engine, this: Ptr) -> u32 {
    if e.mem.u32(this.addr() + 0xd4) == 0 {
        release_overlapping_array(e, this);
    }
    e.call(BASE_DEALLOCATE_INTERNAL_ARRAYS, &args![this]).u32()
}

// Translated from 00cf2cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::hkpAabbPhantom` (Xbox PDB): runs `hkpWorldObject::hkpWorldObject`
/// (`00cd2ce0`, `(0, 2)`), empties the two listener arrays (`+0x8c`, `+0x98`: data 0, size 0,
/// capacity word `0x80000000`), writes the byte `0xf0` at `+0x20` (the offset from the
/// collidable to the object, as a signed byte), the vtable (`010cc004`), the empty overlapping
/// array (`+0xd0`) with the order-dirty byte 0, copies the box from `aabb` and stores
/// `user_data` at `+0x2c`. Returns `this`.
pub fn hkp_aabb_phantom_construct(e: &mut Engine, this: Ptr, aabb: Ptr, user_data: u32) -> Ptr {
    e.call(WORLD_OBJECT_CONSTRUCT, &args![this, 0u32, 2u32]);
    let base = this.addr();
    for array in [0x8cu32, 0x98] {
        e.mem.set_u32(base + array, 0);
        e.mem.set_u32(base + array + 4, 0);
        e.mem.set_u32(base + array + 8, 0x8000_0000);
    }
    let collidable = base + 0x10;
    e.mem.set_u8(
        collidable + 0x10,
        (base as u8).wrapping_sub(collidable as u8),
    );
    e.mem.set_u32(base, AABB_PHANTOM_VTABLE);
    e.mem.set_u32(base + 0xd0, 0);
    e.mem.set_u32(base + 0xd4, 0);
    e.mem.set_u32(base + 0xd8, 0x8000_0000);
    e.mem.set_u8(base + 0xdc, 0);
    copy_vec(e, aabb.addr(), base + 0xb0);
    copy_vec(e, aabb.addr() + 0x10, base + 0xc0);
    e.mem.set_u32(base + 0x2c, user_data);
    this
}

/// Copies the `hkArray` of words at `source` into the array at `target` the way `clone` does:
/// when the target's capacity (low 30 bits) is smaller than the source's size, its storage is
/// freed if owned (capacity word not negative; allocator slot `0x10`, `(data, capacity * 4)`)
/// and a block of `size * 4` bytes is taken (slot `0xc`, with the byte count in a stack word
/// the allocator updates to the granted size); the capacity is the granted size divided by 4,
/// truncating toward zero. Then the size is copied and the words.
fn clone_word_array(e: &mut Engine, allocator: u32, target: u32, source: u32) {
    let capacity_word = e.mem.u32(target + 8);
    let capacity = (capacity_word & 0x3fff_ffff) as i32;
    let source_size = e.mem.i32(source + 4);
    if capacity < source_size {
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(target);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity as u32).wrapping_mul(4)],
            );
        }
        let wanted = (source_size as u32).wrapping_mul(4);
        let (block, granted) = e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), wanted);
            let block = e.vcall(allocator, 0xc, &args![size]).u32();
            (block, e.mem.i32(size.addr()))
        });
        e.mem.set_u32(target, block);
        e.mem.set_i32(target + 8, granted / 4);
    }
    let source_size = e.mem.i32(source + 4);
    let data = e.mem.u32(target);
    e.mem.set_i32(target + 4, source_size);
    let from = e.mem.u32(source);
    let mut remaining = source_size;
    let mut cursor = data;
    if remaining > 0 {
        loop {
            let word = e.mem.u32(cursor.wrapping_add(from.wrapping_sub(data)));
            e.mem.set_u32(cursor, word);
            cursor = cursor.wrapping_add(4);
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
}

// Translated from 00cf2d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAabbPhantom::clone` (Xbox PDB, `__fastcall`): takes `0xe0` bytes from the memory router's
/// allocator (slot 4), stores the size in the 16-bit word at `+4`, constructs the copy with this
/// phantom's box and user data, then copies the two listener arrays (`+0x8c` and `+0x98`, see
/// [`clone_word_array`]; the router is asked again before each) and the properties
/// (`hkpWorldObject::copyProperties`, `00ca92a0`). Returns the copy.
pub fn hkp_aabb_phantom_clone(e: &mut Engine, this: Ptr) -> Ptr {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0xe0u32]).u32();
    e.mem.set_u16(block + 4, 0xe0);
    let user_data = e.mem.u32(this.addr() + 0x2c);
    let copy =
        hkp_aabb_phantom_construct(e, Ptr::new(block), Ptr::new(this.addr() + 0xb0), user_data);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    clone_word_array(e, allocator, copy.addr() + 0x8c, this.addr() + 0x8c);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    clone_word_array(e, allocator, copy.addr() + 0x98, this.addr() + 0x98);
    e.call(COPY_PROPERTIES, &args![copy, this]);
    copy
}

// Translated from 00cf2f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00cf_2bf0`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00cf2f60(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00cf_2bf0, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_2460, hkp_aabb_phantom_get_type(Ptr) -> u32),
        entry!(0x00cf_2470, hkp_aabb_phantom_calc_aabb(Ptr, Ptr)),
        entry!(0x00cf_24a0, hkp_aabb_phantom_set_aabb(Ptr, Ptr)),
        entry!(
            0x00cf_24d0,
            hkp_aabb_phantom_is_overlapping_collidable_added(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00cf_2510,
            hkp_aabb_phantom_cast_ray_collector(Ptr, Ptr, u32)
        ),
        entry!(0x00cf_26b0, hkp_aabb_phantom_cast_ray(Ptr, Ptr, Ptr)),
        entry!(
            0x00cf_2880,
            hkp_aabb_phantom_linear_cast(Ptr, u32, Ptr, u32, u32)
        ),
        entry!(
            0x00cf_2a60,
            hkp_aabb_phantom_remove_overlapping_collidable(Ptr, u32)
        ),
        entry!(
            0x00cf_2ae0,
            hkp_aabb_phantom_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(
            0x00cf_2b50,
            hkp_aabb_phantom_add_overlapping_collidable(Ptr, u32)
        ),
        entry!(
            0x00cf_2bb0,
            hkp_aabb_phantom_ensure_deterministic_order(Ptr)
        ),
        entry!(0x00cf_2bf0, hkp_aabb_phantom_destruct(Ptr) -> u32),
        entry!(
            0x00cf_2c50,
            hkp_aabb_phantom_deallocate_internal_arrays(Ptr) -> u32
        ),
        entry!(0x00cf_2cb0, hkp_aabb_phantom_construct(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cf_2d50, hkp_aabb_phantom_clone(Ptr) -> Ptr),
        entry!(0x00cf_2f60, deleting_destructor_00cf2f60(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;
    /// Doubles for the heap allocator's slots 4 (take), 0xc (take with size word), 0x10 (free).
    const TAKE_SLOT_DOUBLE: u32 = 0x0900_0021;
    const TAKE_SIZED_SLOT_DOUBLE: u32 = 0x0900_0022;
    const FREE_SIZED_SLOT_DOUBLE: u32 = 0x0900_0023;
    /// Doubles for the shapes' ray slots, the filter's slot and the collector's slot.
    const SHAPE_RAY_COLLECTOR_DOUBLE: u32 = 0x0900_0031;
    const SHAPE_RAY_DOUBLE: u32 = 0x0900_0032;
    const FILTER_DOUBLE: u32 = 0x0900_0033;
    const STATISTICS_DOUBLE: u32 = 0x0900_0034;
    /// Dispatcher functions of the linear cast.
    const DISPATCH_FIRST: u32 = 0x0900_0041;
    const DISPATCH_SECOND: u32 = 0x0900_0042;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.call_log = Some(Vec::new());
        e
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Writes a vtable with the given `(byte offset, target)` slots and returns its address.
    fn vtable(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x40);
        for (offset, target) in slots {
            e.mem.set_u32(table + offset, *target);
        }
        table
    }

    /// An object of `size` bytes whose first word is `vt`.
    fn object(e: &mut Engine, size: u32, vt: u32) -> u32 {
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, vt);
        o
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
    fn deleting_destructor_00cf2f60_frees_only_when_bit_zero_is_set() {
        check(0x00cf_2f60, 0x00cf_2bf0);
    }

    /// The monitor stream (cursor at `+4`, limit at `+0xc`) and the TLS slot.
    fn monitor_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0xf0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 5);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    /// The labels of the profiling records written since the stream was made.
    fn record_labels(e: &Engine, stream: u32) -> Vec<u32> {
        let end = e.mem.u32(stream + 4);
        (stream + 0x40..end)
            .step_by(12)
            .map(|at| e.mem.u32(at))
            .collect()
    }

    /// A phantom whose array holds `collidables`, in a world with a collision filter.
    fn phantom_with(e: &mut Engine, collidables: &[u32]) -> (u32, u32, u32) {
        let phantom = e.mem.alloc(0xe0);
        let world = e.mem.alloc(0x100);
        let filter = e.mem.alloc(0x40);
        e.mem.set_u32(world + 0x7c, filter);
        e.mem.set_u32(phantom + 8, world);
        let data = e.mem.alloc(4 * collidables.len() as u32 + 4);
        for (i, collidable) in collidables.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *collidable);
        }
        e.mem.set_u32(phantom + 0xd0, data);
        e.mem.set_u32(phantom + 0xd4, collidables.len() as u32);
        e.mem.set_u32(phantom + 0xd8, collidables.len() as u32);
        (phantom, world, filter)
    }

    /// A transform: columns `(1,2,3,0)`, `(4,5,6,0)`, `(7,8,9,10)`, translation `(1,1,1,0)`.
    fn transform(e: &mut Engine) -> u32 {
        let t = e.mem.alloc(0x40);
        let values: [[f32; 4]; 4] = [
            [1.0, 2.0, 3.0, 0.0],
            [4.0, 5.0, 6.0, 0.0],
            [7.0, 8.0, 9.0, 10.0],
            [1.0, 1.0, 1.0, 0.0],
        ];
        for (column, lanes) in values.iter().enumerate() {
            for (lane, value) in lanes.iter().enumerate() {
                e.mem
                    .set_f32(t + 16 * column as u32 + 4 * lane as u32, *value);
            }
        }
        t
    }

    /// A ray input: from `(2,3,4,0)`, to `(3,4,5,0)`, the flag byte and the filter info.
    fn ray_input(e: &mut Engine, flag: u8) -> u32 {
        let input = e.mem.alloc(0x30);
        for (i, v) in [2.0f32, 3.0, 4.0, 0.0, 3.0, 4.0, 5.0, 0.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(input + 4 * i as u32, *v);
        }
        e.mem.set_u8(input + 0x20, flag);
        e.mem.set_u32(input + 0x24, 0xabcd);
        input
    }

    /// `from = (2,3,4)` through [`transform`]: `d = (1,2,3)`; lane 0 `2*2 + 1*1 + 3*3`, lane 1
    /// `2*5 + 1*4 + 6*3`, lane 2 `2*8 + 1*7 + 9*3`, lane 3 `2*10 + 1*8 + 10*3` (lane 3 uses
    /// `c2.y` and `c2.w`).
    const LOCAL_FROM: [f32; 4] = [14.0, 32.0, 50.0, 58.0];
    /// `to = (3,4,5)`: `d = (2,3,4)`; lane 0 `3*2 + 2*1 + 3*4`, lane 1 `3*5 + 2*4 + 6*4`, lane 2
    /// `3*8 + 2*7 + 9*4`, lane 3 `3*10 + 2*8 + 10*4`.
    const LOCAL_TO: [f32; 4] = [20.0, 47.0, 74.0, 86.0];

    fn words_to_f32(words: &[u32]) -> Vec<f32> {
        words.iter().map(|w| f32::from_bits(*w)).collect()
    }

    #[test]
    fn get_type_is_zero() {
        let mut e = engine();
        assert_eq!(e.call(0x00cf_2460, &args![0x1234u32]).u32(), 0);
    }

    #[test]
    fn calc_aabb_copies_both_vectors() {
        let mut e = engine();
        let phantom = e.mem.alloc(0xe0);
        let out = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(phantom + 0xb0 + 4 * i, 0x100 + i);
        }
        e.call(0x00cf_2470, &args![phantom, out]);
        for i in 0..8u32 {
            assert_eq!(e.mem.u32(out + 4 * i), 0x100 + i);
        }
    }

    #[test]
    fn set_aabb_copies_the_box_then_updates_the_broad_phase() {
        let mut e = engine();
        e.register_double(UPDATE_BROAD_PHASE, |_, _| Ret::default());
        let phantom = e.mem.alloc(0xe0);
        let aabb = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(aabb + 4 * i, 0x200 + i);
        }
        e.call(0x00cf_24a0, &args![phantom, aabb]);
        for i in 0..8u32 {
            assert_eq!(e.mem.u32(phantom + 0xb0 + 4 * i), 0x200 + i);
        }
        assert_eq!(
            calls(&e, UPDATE_BROAD_PHASE),
            vec![vec![phantom, phantom + 0xb0]]
        );
    }

    #[test]
    fn is_overlapping_collidable_added_searches_the_array() {
        let mut e = engine();
        let (phantom, _, _) = phantom_with(&mut e, &[0x11, 0x22, 0x33]);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 9);
        let got = e.call(0x00cf_24d0, &args![phantom, out, 0x33u32]).u32();
        assert_eq!((got, e.mem.u8(out)), (out, 1));
        let got = e.call(0x00cf_24d0, &args![phantom, out, 0x44u32]).u32();
        assert_eq!((got, e.mem.u8(out)), (out, 0));
        let (empty, _, _) = phantom_with(&mut e, &[]);
        e.mem.set_u8(out, 9);
        e.call(0x00cf_24d0, &args![empty, out, 0u32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn ray_with_a_collector_moves_the_ray_into_each_shape_space() {
        let mut e = engine();
        let stream = monitor_stream(&mut e);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(SHAPE_RAY_COLLECTOR_DOUBLE, move |e, a| {
            let local: Vec<u32> = (0..10).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            log.borrow_mut()
                .push([vec![a[0], a[2], a[3]], local].concat());
            Ret::default()
        });
        let shape_vtable = vtable(&mut e, &[(0x24, SHAPE_RAY_COLLECTOR_DOUBLE)]);
        let shape = object(&mut e, 0x10, shape_vtable);
        let shapeless = e.mem.alloc(0x20);
        let with_shape = e.mem.alloc(0x20);
        let t = transform(&mut e);
        e.mem.set_u32(with_shape, shape);
        e.mem.set_u32(with_shape + 8, t);
        let (phantom, _, filter) = phantom_with(&mut e, &[shapeless, with_shape]);
        // Flag clear: no filter pointer.
        let input = ray_input(&mut e, 0);
        e.call(0x00cf_2510, &args![phantom, input, 0x7777u32]);
        // Flag set: the world's filter plus 0x10.
        let input = ray_input(&mut e, 1);
        e.call(0x00cf_2510, &args![phantom, input, 0x7777u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        for (call, pointer) in seen.iter().zip([0, filter + 0x10]) {
            assert_eq!(call[0], shape);
            assert_eq!(call[1], with_shape);
            assert_eq!(call[2], 0x7777);
            assert_eq!(words_to_f32(&call[3..7]), LOCAL_FROM);
            assert_eq!(words_to_f32(&call[7..11]), LOCAL_TO);
            assert_eq!(call[11..13], [0xabcd, pointer]);
        }
        assert_eq!(
            record_labels(&e, stream),
            vec![LABEL_RAY, LABEL_END, LABEL_RAY, LABEL_END]
        );
    }

    #[test]
    fn ray_without_a_world_filter_passes_a_null_filter() {
        let mut e = engine();
        monitor_stream(&mut e);
        let seen: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(SHAPE_RAY_COLLECTOR_DOUBLE, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[1] + 0x24));
            Ret::default()
        });
        let shape_vtable = vtable(&mut e, &[(0x24, SHAPE_RAY_COLLECTOR_DOUBLE)]);
        let shape = object(&mut e, 0x10, shape_vtable);
        let collidable = e.mem.alloc(0x20);
        let t = transform(&mut e);
        e.mem.set_u32(collidable, shape);
        e.mem.set_u32(collidable + 8, t);
        let (phantom, world, _) = phantom_with(&mut e, &[collidable]);
        e.mem.set_u32(world + 0x7c, 0);
        let input = ray_input(&mut e, 1);
        e.call(0x00cf_2510, &args![phantom, input, 1u32]);
        assert_eq!(*seen.borrow(), vec![0]);
    }

    #[test]
    fn cast_ray_records_the_hit_collidable_and_rotates_the_normal() {
        let mut e = engine();
        let stream = monitor_stream(&mut e);
        e.register_double(SET_ROTATED_DIR, |_, _| Ret::default());
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        // Hits on the second call only; the flag byte is left alone on the third.
        let count = Rc::new(RefCell::new(0u32));
        e.register_double(SHAPE_RAY_DOUBLE, move |e, a| {
            let local: Vec<u32> = (0..10).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            log.borrow_mut().push([vec![a[0], a[3]], local].concat());
            let mut n = count.borrow_mut();
            *n += 1;
            if *n == 2 {
                e.mem.set_u8(a[1], 1);
            } else if *n == 1 {
                e.mem.set_u8(a[1], 0);
            }
            Ret::default()
        });
        let shape_vtable = vtable(&mut e, &[(0x20, SHAPE_RAY_DOUBLE)]);
        let shape = object(&mut e, 0x10, shape_vtable);
        let t = transform(&mut e);
        let mut collidables = Vec::new();
        for _ in 0..3 {
            let c = e.mem.alloc(0x20);
            e.mem.set_u32(c, shape);
            e.mem.set_u32(c + 8, t);
            collidables.push(c);
        }
        let (phantom, _, filter) = phantom_with(&mut e, &collidables);
        let input = ray_input(&mut e, 1);
        let output = e.mem.alloc(0x60);
        e.call(0x00cf_26b0, &args![phantom, input, output]);
        // The array is walked from its first element: the second call (the middle collidable)
        // sets the flag and the third leaves it set, so the last collidable is the hit.
        assert_eq!(e.mem.u32(output + 0x50), collidables[2]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 3);
        for call in seen.iter() {
            assert_eq!(call[1], output);
            assert_eq!(words_to_f32(&call[2..6]), LOCAL_FROM);
            assert_eq!(words_to_f32(&call[6..10]), LOCAL_TO);
            assert_eq!(call[10..12], [0xabcd, filter + 0x10]);
        }
        assert_eq!(calls(&e, SET_ROTATED_DIR), vec![vec![output, t, output]]);
        assert_eq!(record_labels(&e, stream), vec![LABEL_RAY, LABEL_END]);
    }

    #[test]
    fn cast_ray_without_a_hit_leaves_the_normal_alone() {
        let mut e = engine();
        monitor_stream(&mut e);
        e.register_double(SET_ROTATED_DIR, |_, _| Ret::default());
        e.register_double(SHAPE_RAY_DOUBLE, |_, _| Ret::default());
        let shape_vtable = vtable(&mut e, &[(0x20, SHAPE_RAY_DOUBLE)]);
        let shape = object(&mut e, 0x10, shape_vtable);
        let t = transform(&mut e);
        let collidable = e.mem.alloc(0x20);
        e.mem.set_u32(collidable, shape);
        e.mem.set_u32(collidable + 8, t);
        let (phantom, _, _) = phantom_with(&mut e, &[collidable]);
        let input = ray_input(&mut e, 0);
        let output = e.mem.alloc(0x60);
        e.call(0x00cf_26b0, &args![phantom, input, output]);
        assert_eq!(e.mem.u32(output + 0x50), 0);
        assert!(calls(&e, SET_ROTATED_DIR).is_empty());
        // A hit collidable the output already holds is rotated even without a new hit.
        e.mem.set_u32(output + 0x50, collidable);
        e.call(0x00cf_26b0, &args![phantom, input, output]);
        assert_eq!(calls(&e, SET_ROTATED_DIR), vec![vec![output, t, output]]);
    }

    #[test]
    fn linear_cast_dispatches_to_enabled_overlapping_collidables() {
        let mut e = engine();
        let stream = monitor_stream(&mut e);
        // The world input, filter (interface at +8) and dispatcher.
        let world_input = e.mem.alloc(0x80);
        for i in 0..0x20u32 {
            e.mem.set_u32(world_input + 4 * i, 0x5000 + i);
        }
        let dispatcher = e.mem.alloc(0x1000);
        let filter = e.mem.alloc(0x40);
        let disabled = e.mem.alloc(0x20);
        let filter_vtable = vtable(&mut e, &[(4, FILTER_DOUBLE)]);
        e.mem.set_u32(filter + 8, filter_vtable);
        e.register_double(FILTER_DOUBLE, move |e, a| {
            e.mem.set_u8(a[1], u8::from(a[3] != disabled));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        // Shapes of types 1 (the cast collidable), 2 and 3.
        let casting_shape = e.mem.alloc(0x20);
        e.mem.set_u32(casting_shape + 0xc, 1);
        let shape_two = e.mem.alloc(0x20);
        e.mem.set_u32(shape_two + 0xc, 2);
        let shape_three = e.mem.alloc(0x20);
        e.mem.set_u32(shape_three + 0xc, 3);
        let t = transform(&mut e);
        let cast_collidable = e.mem.alloc(0x20);
        e.mem.set_u32(cast_collidable, casting_shape);
        e.mem.set_u32(cast_collidable + 8, t);
        let shapeless = e.mem.alloc(0x20);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first, shape_two);
        let second = e.mem.alloc(0x20);
        e.mem.set_u32(second, shape_three);
        let (phantom, world, _) = phantom_with(
            &mut e,
            &[shapeless, cast_collidable, first, disabled, second],
        );
        e.mem.set_u32(disabled, shape_two);
        e.mem.set_u32(world + 0x78, world_input);
        e.mem.set_u32(world + 0x7c, filter);
        e.mem.set_u32(world + 0x80, dispatcher);
        // Type pairs (1, 2) and (1, 3) select records 1 and 2.
        e.mem.set_u8(dispatcher + (1 + 0xd) * 0x20 + 2, 1);
        e.mem.set_u8(dispatcher + (1 + 0xd) * 0x20 + 3, 2);
        e.mem.set_u32(dispatcher + 0x9ac + 0x14, DISPATCH_FIRST);
        e.mem.set_u32(dispatcher + 0x9ac + 0x28, DISPATCH_SECOND);
        let inputs: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        for double in [DISPATCH_FIRST, DISPATCH_SECOND] {
            let log = Rc::clone(&inputs);
            e.register_double(double, move |e, a| {
                let input: Vec<u32> = (0..0x1c).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
                log.borrow_mut().push([a.to_vec(), input].concat());
                Ret::default()
            });
        }
        let cast_input = e.mem.alloc(0x20);
        for (i, v) in [5.0f32, 6.0, 7.0, 8.0, 2.5, 9.5].iter().enumerate() {
            e.mem.set_f32(cast_input + 4 * i as u32, *v);
        }
        e.call(
            0x00cf_2880,
            &args![phantom, cast_collidable, cast_input, 0x111u32, 0x222u32],
        );
        let inputs = inputs.borrow();
        assert_eq!(inputs.len(), 2);
        let calls_made: Vec<&[u32]> = inputs.iter().map(|c| &c[..5]).collect();
        let input_address = inputs[0][2];
        assert_eq!(
            calls_made[0],
            [cast_collidable, first, input_address, 0x111, 0x222]
        );
        assert_eq!(
            calls_made[1],
            [cast_collidable, second, input_address, 0x111, 0x222]
        );
        let words = &inputs[0][5..];
        // Words copied from the world input, then the cast's words.
        assert_eq!(words[0], 0x5000);
        assert_eq!(words[1], 0x5001);
        assert_eq!(f32::from_bits(words[2]), 9.5);
        assert_eq!(words[3], 0x5003);
        assert_eq!(words[4], 0x5004);
        assert_eq!(words[5], 0x5005);
        // Vectors +0x20..+0x50 are the world input's.
        for i in 0..12 {
            assert_eq!(words[8 + i], 0x5008 + i as u32);
        }
        // Delta (5,6,7,8) - (1,1,1,0), its length, the cast's word 4 and the world's word +0x70.
        assert_eq!(words_to_f32(&words[20..24]), [4.0, 5.0, 6.0, 8.0]);
        assert_eq!(f32::from_bits(words[24]), 2.5);
        assert_eq!(f32::from_bits(words[25]), 77.0f32.sqrt());
        assert_eq!(words[26], 0x501c);
        assert_eq!(
            record_labels(&e, stream),
            vec![LABEL_LINEAR_CAST, LABEL_END]
        );
    }

    #[test]
    fn remove_overlapping_collidable_fills_the_hole_with_the_last_element() {
        let mut e = engine();
        e.register_double(FIRE_COLLIDABLE_REMOVED, |_, _| Ret::default());
        let (phantom, _, _) = phantom_with(&mut e, &[0x11, 0x22, 0x33]);
        e.call(0x00cf_2a60, &args![phantom, 0x11u32]);
        let data = e.mem.u32(phantom + 0xd0);
        assert_eq!(e.mem.u32(phantom + 0xd4), 2);
        assert_eq!((e.mem.u32(data), e.mem.u32(data + 4)), (0x33, 0x22));
        assert_eq!(e.mem.u8(phantom + 0xdc), 1);
        // The last element leaves the array without moving anything.
        e.mem.set_u8(phantom + 0xdc, 0);
        e.call(0x00cf_2a60, &args![phantom, 0x22u32]);
        assert_eq!(e.mem.u32(phantom + 0xd4), 1);
        assert_eq!(e.mem.u32(data), 0x33);
        assert_eq!(e.mem.u8(phantom + 0xdc), 1);
        // The event words: the size with its low byte replaced by the found flag.
        assert_eq!(
            calls(&e, FIRE_COLLIDABLE_REMOVED),
            vec![vec![phantom, 0x11, 1], vec![phantom, 0x22, 1]]
        );
    }

    #[test]
    fn remove_overlapping_collidable_reports_a_missing_one() {
        let mut e = engine();
        e.register_double(FIRE_COLLIDABLE_REMOVED, |_, _| Ret::default());
        let values: Vec<u32> = (0..0x102).map(|i| 0x1000 + i).collect();
        let (phantom, _, _) = phantom_with(&mut e, &values);
        e.call(0x00cf_2a60, &args![phantom, 0x99u32]);
        assert_eq!(e.mem.u32(phantom + 0xd4), 0x102);
        assert_eq!(e.mem.u8(phantom + 0xdc), 0);
        let (empty, _, _) = phantom_with(&mut e, &[]);
        e.call(0x00cf_2a60, &args![empty, 0x99u32]);
        let logged = calls(&e, FIRE_COLLIDABLE_REMOVED);
        assert_eq!(logged[0], vec![phantom, 0x99, 0x100]);
        assert_eq!(logged[1], vec![empty, 0x99, 0]);
    }

    #[test]
    fn calc_content_statistics_reports_the_array_unless_unowned() {
        let mut e = engine();
        e.register_double(GET_ALLOCATED_SIZE, |_, _| Ret {
            eax: 0x40,
            ..Ret::default()
        });
        e.register_double(BASE_CALC_CONTENT_STATISTICS, |_, _| Ret::default());
        e.register_double(STATISTICS_DOUBLE, |_, _| Ret::default());
        let collector_vtable = vtable(&mut e, &[(0x14, STATISTICS_DOUBLE)]);
        let collector = object(&mut e, 0x20, collector_vtable);
        let (phantom, _, _) = phantom_with(&mut e, &[1, 2, 3]);
        e.mem.set_u32(phantom + 0xd8, 0x4000_0008);
        let data = e.mem.u32(phantom + 0xd0);
        e.call(0x00cf_2ae0, &args![phantom, collector, 0u32]);
        assert_eq!(calls(&e, GET_ALLOCATED_SIZE), vec![vec![data, 0x20]]);
        assert_eq!(
            calls(&e, STATISTICS_DOUBLE),
            vec![vec![collector, 3, NAME_OVERLAPPING, data, 12, 0x40]]
        );
        assert_eq!(
            calls(&e, BASE_CALC_CONTENT_STATISTICS),
            vec![vec![phantom, collector, PHANTOM_CLASS_RECORD]]
        );
        // Unowned storage and no collector flag: only the base class reports.
        e.mem.set_u32(phantom + 0xd8, 0x8000_0003);
        e.call(0x00cf_2ae0, &args![phantom, collector, 0u32]);
        assert_eq!(calls(&e, STATISTICS_DOUBLE).len(), 1);
        assert_eq!(calls(&e, BASE_CALC_CONTENT_STATISTICS).len(), 2);
        // With the collector's flag byte set the array is reported anyway.
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00cf_2ae0, &args![phantom, collector, 0u32]);
        assert_eq!(calls(&e, STATISTICS_DOUBLE).len(), 2);
        assert_eq!(calls(&e, GET_ALLOCATED_SIZE)[1], vec![data, 12]);
    }

    #[test]
    fn add_overlapping_collidable_appends_and_grows_when_full() {
        let mut e = engine();
        install(&mut e, 0x00cf_2bf0);
        e.register_double(FIRE_COLLIDABLE_ADDED, |_, a| Ret {
            eax: u32::from(a[1] == 0xdead),
            ..Ret::default()
        });
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            // Grows the array at a[1] to room for 4 elements.
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        let phantom = e.mem.alloc(0xe0);
        e.mem.set_u32(phantom + 0xd8, 0x8000_0000);
        // Full (size 0 == capacity 0): grown, then appended.
        e.call(0x00cf_2b50, &args![phantom, 0x11u32]);
        let data = e.mem.u32(phantom + 0xd0);
        assert_eq!(e.mem.u32(data), 0x11);
        assert_eq!(e.mem.u32(phantom + 0xd4), 1);
        assert_eq!(e.mem.u8(phantom + 0xdc), 1);
        assert_eq!(calls(&e, ARRAY_RESERVE_MORE).len(), 1);
        assert_eq!(calls(&e, ARRAY_RESERVE_MORE)[0][1..], [phantom + 0xd0, 4]);
        // Room left: no growth.
        e.call(0x00cf_2b50, &args![phantom, 0x22u32]);
        assert_eq!(e.mem.u32(data + 4), 0x22);
        assert_eq!(e.mem.u32(phantom + 0xd4), 2);
        assert_eq!(calls(&e, ARRAY_RESERVE_MORE).len(), 1);
        // A listener that vetoes the collidable stops the addition.
        e.mem.set_u8(phantom + 0xdc, 0);
        e.call(0x00cf_2b50, &args![phantom, 0xdeadu32]);
        assert_eq!(e.mem.u32(phantom + 0xd4), 2);
        assert_eq!(e.mem.u8(phantom + 0xdc), 0);
    }

    #[test]
    fn ensure_deterministic_order_sorts_only_a_dirty_array_of_two_or_more() {
        let mut e = engine();
        e.register_double(QUICK_SORT_RECURSIVE, |_, _| Ret::default());
        let (phantom, _, _) = phantom_with(&mut e, &[1, 2, 3]);
        let data = e.mem.u32(phantom + 0xd0);
        // Clean: nothing happens.
        e.call(0x00cf_2bb0, &args![phantom]);
        assert!(calls(&e, QUICK_SORT_RECURSIVE).is_empty());
        e.mem.set_u8(phantom + 0xdc, 1);
        e.call(0x00cf_2bb0, &args![phantom]);
        assert_eq!(
            calls(&e, QUICK_SORT_RECURSIVE),
            vec![vec![data, 0, 2, phantom & 0xffff_ff00]]
        );
        assert_eq!(e.mem.u8(phantom + 0xdc), 0);
        // One element: the flag is cleared without a sort.
        e.mem.set_u32(phantom + 0xd4, 1);
        e.mem.set_u8(phantom + 0xdc, 1);
        e.call(0x00cf_2bb0, &args![phantom]);
        assert_eq!(calls(&e, QUICK_SORT_RECURSIVE).len(), 1);
        assert_eq!(e.mem.u8(phantom + 0xdc), 0);
    }

    #[test]
    fn destructor_frees_owned_storage_then_runs_the_base_destructor() {
        let mut e = engine();
        let log = install(&mut e, BASE_DESTRUCTOR);
        e.register_double(FREE_SIZED_SLOT_DOUBLE, |_, _| Ret::default());
        let allocator_vtable = vtable(
            &mut e,
            &[(8, FREE_SLOT_DOUBLE), (0x10, FREE_SIZED_SLOT_DOUBLE)],
        );
        let router = e.mem.alloc(0x20);
        let allocator = object(&mut e, 0x10, allocator_vtable);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let (phantom, _, _) = phantom_with(&mut e, &[1, 2]);
        e.mem.set_u32(phantom + 0xd8, 0x0000_0010);
        let data = e.mem.u32(phantom + 0xd0);
        let got = e.call(0x00cf_2bf0, &args![phantom]).u32();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(phantom), AABB_PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(phantom + 0xd4), 0);
        assert_eq!(e.mem.u32(phantom + 0xd0), 0);
        assert_eq!(e.mem.u32(phantom + 0xd8), 0x8000_0000);
        assert_eq!(
            calls(&e, FREE_SIZED_SLOT_DOUBLE),
            vec![vec![allocator, data, 0x40]]
        );
        assert_eq!(*log.borrow(), vec![1, phantom]);
        // Unowned storage (capacity word negative): nothing is freed.
        e.mem.set_u32(phantom + 0xd8, 0x8000_0002);
        e.call(0x00cf_2bf0, &args![phantom]);
        assert_eq!(calls(&e, FREE_SIZED_SLOT_DOUBLE).len(), 1);
    }

    #[test]
    fn deallocate_internal_arrays_releases_only_an_empty_array() {
        let mut e = engine();
        e.register_double(BASE_DEALLOCATE_INTERNAL_ARRAYS, |_, _| Ret {
            eax: 0x77,
            ..Ret::default()
        });
        e.register_double(FREE_SIZED_SLOT_DOUBLE, |_, _| Ret::default());
        let allocator_vtable = vtable(&mut e, &[(0x10, FREE_SIZED_SLOT_DOUBLE)]);
        let router = e.mem.alloc(0x20);
        let allocator = object(&mut e, 0x10, allocator_vtable);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let (phantom, _, _) = phantom_with(&mut e, &[1, 2]);
        let data = e.mem.u32(phantom + 0xd0);
        // Not empty: the array stays.
        assert_eq!(e.call(0x00cf_2c50, &args![phantom]).u32(), 0x77);
        assert_eq!(e.mem.u32(phantom + 0xd0), data);
        assert!(calls(&e, FREE_SIZED_SLOT_DOUBLE).is_empty());
        // Empty: freed with capacity * 4 bytes.
        e.mem.set_u32(phantom + 0xd4, 0);
        assert_eq!(e.call(0x00cf_2c50, &args![phantom]).u32(), 0x77);
        assert_eq!(e.mem.u32(phantom + 0xd0), 0);
        assert_eq!(e.mem.u32(phantom + 0xd8), 0x8000_0000);
        assert_eq!(
            calls(&e, FREE_SIZED_SLOT_DOUBLE),
            vec![vec![allocator, data, 8]]
        );
    }

    #[test]
    fn construct_sets_up_the_phantom() {
        let mut e = engine();
        e.register_double(WORLD_OBJECT_CONSTRUCT, |_, _| Ret::default());
        let phantom = e.mem.alloc(0xe0);
        // Garbage the constructor must overwrite.
        for offset in [
            0x8cu32, 0x90, 0x94, 0x98, 0x9c, 0xa0, 0xd0, 0xd4, 0xd8, 0xdc,
        ] {
            e.mem.set_u32(phantom + offset, 0x5a5a_5a5a);
        }
        let aabb = e.mem.alloc(0x20);
        for i in 0..8u32 {
            e.mem.set_u32(aabb + 4 * i, 0x300 + i);
        }
        let got = e.call(0x00cf_2cb0, &args![phantom, aabb, 0x4242u32]).u32();
        assert_eq!(got, phantom);
        assert_eq!(calls(&e, WORLD_OBJECT_CONSTRUCT), vec![vec![phantom, 0, 2]]);
        for array in [0x8cu32, 0x98] {
            assert_eq!(
                [
                    e.mem.u32(phantom + array),
                    e.mem.u32(phantom + array + 4),
                    e.mem.u32(phantom + array + 8)
                ],
                [0, 0, 0x8000_0000]
            );
        }
        assert_eq!(e.mem.u8(phantom + 0x20), 0xf0);
        assert_eq!(e.mem.u32(phantom), AABB_PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(phantom + 0xd0), 0);
        assert_eq!(e.mem.u32(phantom + 0xd4), 0);
        assert_eq!(e.mem.u32(phantom + 0xd8), 0x8000_0000);
        assert_eq!(e.mem.u8(phantom + 0xdc), 0);
        for i in 0..8u32 {
            assert_eq!(e.mem.u32(phantom + 0xb0 + 4 * i), 0x300 + i);
        }
        assert_eq!(e.mem.u32(phantom + 0x2c), 0x4242);
    }

    #[test]
    fn clone_copies_the_box_listeners_and_properties() {
        let mut e = engine();
        e.register_double(WORLD_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register_double(COPY_PROPERTIES, |_, _| Ret::default());
        e.register_double(TAKE_SLOT_DOUBLE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        // Takes the requested bytes and grants 6 more (not a multiple of 4).
        e.register_double(TAKE_SIZED_SLOT_DOUBLE, |e, a| {
            let wanted = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], wanted + 6);
            Ret {
                eax: e.mem.alloc(wanted + 6),
                ..Ret::default()
            }
        });
        e.register_double(FREE_SIZED_SLOT_DOUBLE, |_, _| Ret::default());
        let allocator_vtable = vtable(
            &mut e,
            &[
                (4, TAKE_SLOT_DOUBLE),
                (0xc, TAKE_SIZED_SLOT_DOUBLE),
                (0x10, FREE_SIZED_SLOT_DOUBLE),
            ],
        );
        let router = e.mem.alloc(0x20);
        let allocator = object(&mut e, 0x10, allocator_vtable);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let phantom = e.mem.alloc(0xe0);
        e.mem.set_u32(phantom + 0x2c, 0x4242);
        for i in 0..8u32 {
            e.mem.set_u32(phantom + 0xb0 + 4 * i, 0x300 + i);
        }
        // Overlap listeners: two words; phantom listeners: three words.
        let overlap = e.mem.alloc(8);
        e.mem.set_u32(overlap, 0xaa);
        e.mem.set_u32(overlap + 4, 0xbb);
        let listeners = e.mem.alloc(12);
        for i in 0..3u32 {
            e.mem.set_u32(listeners + 4 * i, 0xc0 + i);
        }
        e.mem.set_u32(phantom + 0x8c, overlap);
        e.mem.set_u32(phantom + 0x90, 2);
        e.mem.set_u32(phantom + 0x94, 2);
        e.mem.set_u32(phantom + 0x98, listeners);
        e.mem.set_u32(phantom + 0x9c, 3);
        e.mem.set_u32(phantom + 0xa0, 3);
        let copy = e.call(0x00cf_2d50, &args![phantom]).u32();
        assert_eq!(e.mem.u16(copy + 4), 0xe0);
        assert_eq!(calls(&e, TAKE_SLOT_DOUBLE), vec![vec![allocator, 0xe0]]);
        assert_eq!(calls(&e, WORLD_OBJECT_CONSTRUCT), vec![vec![copy, 0, 2]]);
        assert_eq!(e.mem.u32(copy + 0x2c), 0x4242);
        for i in 0..8u32 {
            assert_eq!(e.mem.u32(copy + 0xb0 + 4 * i), 0x300 + i);
        }
        // Overlap listeners: 8 bytes asked, 14 granted, capacity 3 (truncated).
        let data = e.mem.u32(copy + 0x8c);
        assert_ne!(data, overlap);
        assert_eq!((e.mem.u32(copy + 0x90), e.mem.u32(copy + 0x94)), (2, 3));
        assert_eq!((e.mem.u32(data), e.mem.u32(data + 4)), (0xaa, 0xbb));
        // Phantom listeners: 12 asked, 18 granted, capacity 4.
        let data = e.mem.u32(copy + 0x98);
        assert_eq!((e.mem.u32(copy + 0x9c), e.mem.u32(copy + 0xa0)), (3, 4));
        assert_eq!(
            [e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)],
            [0xc0, 0xc1, 0xc2]
        );
        // The freshly constructed arrays are unowned: nothing was freed.
        assert!(calls(&e, FREE_SIZED_SLOT_DOUBLE).is_empty());
        assert_eq!(calls(&e, COPY_PROPERTIES), vec![vec![copy, phantom]]);
        // The router is asked three times (block, then once per array).
        assert_eq!(calls(&e, MEMORY_ROUTER).len(), 3);
    }
}
