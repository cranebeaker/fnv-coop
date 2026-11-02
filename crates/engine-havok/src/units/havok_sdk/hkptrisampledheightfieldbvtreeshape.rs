//! `Havok SDK/hkptrisampledheightfieldbvtreeshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpTriSampledHeightFieldBvTreeShape` (vtable `010ce404`, 0x30 bytes) is the bounding-volume
//! tree shape over an `hkpTriSampledHeightFieldCollection`: its `hkpSingleShapeContainer`
//! (vtable `010c6fd0`) is at `+0x14` and holds the collection at `+0x18`; `+0x1C` is the child
//! size `calcSizeForSpu` recorded and `+0x20` the `m_wantAabbRejectionTest` byte. A shape key
//! of the collection is `(z * 0x8000 + x) * 2`, plus 1 for the second triangle of the cell.
//!
//! `getExtentsForQueryAabb` turns a query box into a range of cells (`x_min..=x_max`,
//! `z_min..=z_max`). With the rejection test on, `queryAabb` and `queryAabbImpl` also sample the
//! height field's heights (vtable slot `0x34`, multiplied by the scale at `+0x24`) over those
//! cells and one more row and column, and throw the keys away when every sample lies at or
//! below the box's bottom (`all_below`) or at or above its top (`all_above`).
//!
//! Float comparisons of the samples are x87 `FCOM`s: a NaN never clears a flag. SSE `MINPS` and
//! `MAXPS` return the second operand when the compare is false (also for NaN).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkptrisampledheightfieldcollection::HkpTriSampledHeightFieldCollection;

layout! {
    /// `hkpTriSampledHeightFieldBvTreeShape` (Xbox PDB), 0x30 bytes; offsets checked against
    /// the code below.
    pub struct HkpTriSampledHeightFieldBvTreeShape: 0x30 {
        /// `m_childContainer.m_childShape` (Xbox PDB): the `hkpTriSampledHeightFieldCollection`.
        0x18 m_child_shape: Ptr,
        /// `m_childSize` (Xbox PDB).
        0x1c m_child_size: u32,
        /// `m_wantAabbRejectionTest` (Xbox PDB).
        0x20 m_want_aabb_rejection_test: u8,
    }
}

/// `hkpTriSampledHeightFieldCollection::getHeightFieldShape` (Xbox PDB).
const GET_HEIGHT_FIELD_SHAPE: u32 = 0x00d2_6450;
/// `getExtentsForQueryAabb` (Xbox PDB), cdecl with seven stack words.
const GET_EXTENTS_FOR_QUERY_AABB: u32 = 0x00d2_6940;
/// `hkpBoxShape`'s box helper (`00c9e120`, cdecl `(transform, extents, tolerance, aabb)`).
const CALC_AABB: u32 = 0x00c9_e120;
/// The memory router (`00c85750`, no arguments): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, cdecl `(allocator, array, capacity, element size)`) and
/// the `hkarrayutil.cpp` function that makes room for one more element (`00c90940`, cdecl
/// `(allocator, array, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkReferencedObject` reference counting on the collection: add (`00c90510`) and remove
/// (`00c905b0`), `this` = the object.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// The transform `getExtentsForQueryAabb` hands to the height field's `getAabbImpl` (`01268370`,
/// zero in the file, set up at start-up).
const AABB_TRANSFORM: u32 = 0x0126_8370;
/// The lane mask `getExtentsForQueryAabb` ANDs its compare results with (`012684b0`, set up at
/// start-up), the float at `01011d78` (0.0 in the file) and the four floats 65536.0 it adds to
/// make the integer part sit in the mantissa (`011b3ce0`).
const LANE_MASK: u32 = 0x0126_84b0;
const LIMIT_LANE: u32 = 0x0101_1d78;
const MANTISSA_BIAS: u32 = 0x011b_3ce0;
/// The vtables the constructor and destructor write: the shape's (`010ce404`), the container's
/// (`010c6fd0`, and `010c6fb0` once the collection is released) and the base class's
/// (`0102e388`).
const SHAPE_VTABLE: u32 = 0x010c_e404;
const CONTAINER_VTABLE: u32 = 0x010c_6fd0;
const CONTAINER_BASE_VTABLE: u32 = 0x010c_6fb0;
const SHAPE_BASE_VTABLE: u32 = 0x0102_e388;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

// Translated from 00d268b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldBvTreeShape::calcSizeForSpu` (Xbox PDB): asks the collection (slot
/// `0x18`) for its size with `param` and the limit `0x100`. An answer above `0x100` (unsigned)
/// gives -1; otherwise the shape records it at `+0x1C` and the size is `0x30`. The function
/// ends `RET 8`: the second stack word is not read.
pub fn calc_size_for_spu(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldBvTreeShape>,
    param: u32,
    _unused_1: u32,
) -> i32 {
    let collection = e.get(this, HkpTriSampledHeightFieldBvTreeShape::m_child_shape);
    let size = e
        .vcall(collection.addr(), 0x18, &args![param, 0x100u32])
        .u32();
    if size > 0x100 {
        return -1;
    }
    e.set(
        this,
        HkpTriSampledHeightFieldBvTreeShape::m_child_size,
        size,
    );
    0x30
}

// Translated from 00d268f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldBvTreeShape::queryObb` (Xbox PDB), `(this, transform, extents,
/// tolerance, hits)`: turns the oriented box into an axis-aligned one in a 0x20-byte local
/// (`00c9e120`, cdecl with `transform`, `extents`, the tolerance as an `FLD`/`FSTP` copy and
/// the local; the `this` the code pushes first is overwritten by the float) and passes it with
/// `hits` to the shape's slot `0x30` (`queryAabb`).
pub fn query_obb(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldBvTreeShape>,
    transform: Ptr,
    extents: Ptr,
    tolerance: f32,
    hits: Ptr,
) {
    e.with_stack(0x20, |e, aabb| {
        e.call(
            CALC_AABB,
            &args![transform, extents, x87_quiet(tolerance), aabb],
        );
        e.vcall(this.addr(), 0x30, &args![aabb, hits]);
    });
}

// Translated from 00d26940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldBvTreeShape::getExtentsForQueryAabb` (Xbox PDB), cdecl,
/// `(found, aabb, collection, x_min, x_max, z_min, z_max)` (the last four are `u32` outputs).
/// Widens `aabb` (min at `+0`, max at `+0x10`) by the collection's radius, asks the height
/// field (slot `0x1C`, with the transform `01268370` and a tolerance of 0.0) for its own box and
/// clamps `aabb` to it lane by lane (`CMPLTPS` masks ANDed with the lane mask `012684b0`).
/// When the clamped max is not above the min in all of lanes 0 to 2 (`CMPLEPS`) `found` is 0.
/// Otherwise both corners are taken to grid cells, `(corner + offset(+0x40)) * scale(+0x30)`
/// clamped between 0 and `(xRes - 2, c, zRes - 2, c)` (`c` the float at `01011d78`), plus
/// 65536.0; the mantissa bits shifted right by 7 and masked to 16 bits are the cell, lane 0
/// for x and lane 2 for z. `found` is then 1.
/// Answers `found`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn get_extents_for_query_aabb(
    e: &mut Engine,
    found: Ptr,
    aabb: Ptr,
    collection: Ptr<HkpTriSampledHeightFieldCollection>,
    x_min: Ptr,
    x_max: Ptr,
    z_min: Ptr,
    z_max: Ptr,
) -> Ptr {
    let height_field = e.call(GET_HEIGHT_FIELD_SHAPE, &args![collection]).u32();
    let radius = x87_quiet(e.get(collection, HkpTriSampledHeightFieldCollection::m_radius));
    let mut max = load_vec(e, aabb.addr() + 0x10);
    max = max.map(|lane| lane + radius);
    store_vec(e, aabb.addr() + 0x10, max);
    let mut min = load_vec(e, aabb.addr());
    min = min.map(|lane| lane - radius);
    store_vec(e, aabb.addr(), min);
    let (box_min, box_max) = e.with_stack(0x20, |e, local| {
        e.vcall(height_field, 0x1c, &args![AABB_TRANSFORM, 0.0f32, local]);
        (load_vec(e, local.addr()), load_vec(e, local.addr() + 0x10))
    });
    let mask: [u32; 4] = std::array::from_fn(|lane| e.mem.u32(LANE_MASK + 4 * lane as u32));
    // Lane masks of the compares, ANDed with the lane mask.
    let raised: [u32; 4] = std::array::from_fn(|lane| {
        let compare = if min[lane] < box_min[lane] {
            u32::MAX
        } else {
            0
        };
        compare & mask[lane]
    });
    let new_min: Vec4 = std::array::from_fn(|lane| {
        f32::from_bits(
            (!raised[lane] & min[lane].to_bits()) | (raised[lane] & box_min[lane].to_bits()),
        )
    });
    let lowered: [u32; 4] = std::array::from_fn(|lane| {
        let compare = if box_max[lane] < max[lane] {
            u32::MAX
        } else {
            0
        };
        compare & mask[lane]
    });
    let new_max: Vec4 = std::array::from_fn(|lane| {
        f32::from_bits(
            (lowered[lane] & box_max[lane].to_bits()) | (!lowered[lane] & max[lane].to_bits()),
        )
    });
    store_vec(e, aabb.addr() + 0x10, new_max);
    store_vec(e, aabb.addr(), new_min);
    if (0..3).any(|lane| new_max[lane] <= new_min[lane]) {
        e.mem.set_u8(found.addr(), 0);
        return found;
    }
    let x_res = e.mem.u32(height_field + 0x10).wrapping_sub(2) as i32 as f32;
    let z_res = e.mem.u32(height_field + 0x14).wrapping_sub(2) as i32 as f32;
    let lane_c: f32 = e.global(LIMIT_LANE);
    let limit = [x_res, lane_c, z_res, lane_c];
    let offset = load_vec(e, height_field + 0x40);
    let scale = load_vec(e, height_field + 0x30);
    let bias = load_vec(e, MANTISSA_BIAS);
    let cell = |corner: Vec4, lane: usize| -> u32 {
        let scaled = (offset[lane] + corner[lane]) * scale[lane];
        let below = if scaled < limit[lane] {
            scaled
        } else {
            limit[lane]
        };
        let clamped = if below > 0.0 { below } else { 0.0 };
        (((clamped + bias[lane]).to_bits() >> 7) & 0xffff) as u32
    };
    e.mem.set_u32(x_min.addr(), cell(new_min, 0));
    e.mem.set_u32(z_min.addr(), cell(new_min, 2));
    e.mem.set_u32(x_max.addr(), cell(new_max, 0));
    e.mem.set_u32(z_max.addr(), cell(new_max, 2));
    e.mem.set_u8(found.addr(), 1);
    found
}

// Translated from 00d26ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldBvTreeShape::getAabbImpl` (Xbox PDB), `(this, transform, tolerance,
/// aabb)`: passes everything on to the collection's slot `0x1C` (the tolerance as an
/// `FLD`/`FSTP` copy).
pub fn get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldBvTreeShape>,
    transform: Ptr,
    tolerance: f32,
    aabb: Ptr,
) {
    let collection = e.get(this, HkpTriSampledHeightFieldBvTreeShape::m_child_shape);
    e.vcall(
        collection.addr(),
        0x1c,
        &args![transform, x87_quiet(tolerance), aabb],
    );
}

// Translated from 00d26b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldBvTreeShape::castRayImpl` (Xbox PDB), `(this, a, b, c)`: calls the
/// slot `0x20` of the collection's height field (`getHeightFieldShape` of the collection at
/// `+0x18`) with the three words and returns the first one (`ESI`), not the callee's answer.
pub fn cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldBvTreeShape>,
    first: u32,
    second: u32,
    third: u32,
) -> u32 {
    let collection = e.get(this, HkpTriSampledHeightFieldBvTreeShape::m_child_shape);
    let height_field = e.call(GET_HEIGHT_FIELD_SHAPE, &args![collection]).u32();
    e.vcall(height_field, 0x20, &args![first, second, third]);
    first
}

// Translated from 00d26b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldBvTreeShape::castRayWithCollectorImpl` (Xbox PDB), `(this, a, b,
/// c)`: a tail jump to the slot `0x24` of the collection's height field with the caller's own
/// stack words (three, as that slot's `RET 0xC` takes).
pub fn cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldBvTreeShape>,
    first: u32,
    second: u32,
    third: u32,
) {
    let collection = e.get(this, HkpTriSampledHeightFieldBvTreeShape::m_child_shape);
    let height_field = e.call(GET_HEIGHT_FIELD_SHAPE, &args![collection]).u32();
    e.vcall(height_field, 0x24, &args![first, second, third]);
}

// Translated from 00d26b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shape's constructor (the map gives `00d26b50` no name; that it is
/// `hkpTriSampledHeightFieldBvTreeShape::hkpTriSampledHeightFieldBvTreeShape` is unconfirmed),
/// `(this, collection, want rejection test)`: reference count `+6` = 1, user data `+8` = 0,
/// type `+0xC` = 8, bv tree type `+0x10` = 1, vtables `010ce404` at `+0` and `010c6fd0` at
/// `+0x14`, the collection at `+0x18` with a reference added (`00c90510`) and the flag byte at
/// `+0x20`. Returns `this`.
pub fn fn_00d26b50(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldBvTreeShape>,
    collection: Ptr,
    want_rejection_test: u8,
) -> Ptr<HkpTriSampledHeightFieldBvTreeShape> {
    e.mem.set_u16(this.addr() + 6, 1);
    e.mem.set_u32(this.addr() + 8, 0);
    e.mem.set_u32(this.addr() + 0xc, 8);
    e.mem.set_u8(this.addr() + 0x10, 1);
    e.mem.set_u32(this.addr(), SHAPE_VTABLE);
    e.mem.set_u32(this.addr() + 0x14, CONTAINER_VTABLE);
    e.set(
        this,
        HkpTriSampledHeightFieldBvTreeShape::m_child_shape,
        collection,
    );
    e.call(ADD_REFERENCE, &args![collection]);
    e.set(
        this,
        HkpTriSampledHeightFieldBvTreeShape::m_want_aabb_rejection_test,
        want_rejection_test,
    );
    this
}

/// A range of grid cells of a query.
#[derive(Clone, Copy)]
struct Cells {
    x_min: u32,
    x_max: u32,
    z_min: u32,
    z_max: u32,
}

/// The two flags of the rejection test: `all_below` stays set while no sample is above the
/// box's bottom (`min_y`), `all_above` while no sample is below its top (`max_y`).
struct Rejection {
    min_y: f32,
    max_y: f32,
    all_below: bool,
    all_above: bool,
}

impl Rejection {
    /// The height at cell corner `(x, z)` times the height field's scale (`+0x24`); then the
    /// two `FCOM`s clear the flags on a strict, ordered compare.
    fn sample(&mut self, e: &mut Engine, height_field: u32, x: u32, z: u32) {
        let height = e.vcall(height_field, 0x34, &args![x, z]).f32();
        let scaled = e.mem.f32(height_field + 0x24) * height;
        if scaled > self.min_y {
            self.all_below = false;
        }
        if self.max_y > scaled {
            self.all_above = false;
        }
    }

    fn rejects(&self) -> bool {
        self.all_below || self.all_above
    }
}

/// Visits the keys of `cells` without sampling heights: for every x, for every z, the key
/// `(z * 0x8000 + x) * 2`; `emit` is called with it (and is to add the second triangle's key).
fn walk_cells(e: &mut Engine, cells: Cells, emit: &mut dyn FnMut(&mut Engine, u32)) {
    let mut x = cells.x_min;
    if x > cells.x_max {
        return;
    }
    loop {
        if cells.z_min <= cells.z_max {
            let mut key = ((cells.z_min << 15).wrapping_add(x)).wrapping_mul(2);
            let mut left = cells.z_max.wrapping_sub(cells.z_min).wrapping_add(1);
            loop {
                emit(e, key);
                key = key.wrapping_add(0x10000);
                left = left.wrapping_sub(1);
                if left == 0 {
                    break;
                }
            }
        }
        x = x.wrapping_add(1);
        if x > cells.x_max {
            break;
        }
    }
}

/// The rejection-test walk of `queryAabbImpl` and `queryAabb`: emits the keys like
/// [`walk_cells`], sampling the height of each cell's corner while a flag is still set; when
/// both flags are cleared after the cells the extra row and column are not sampled. Otherwise
/// the heights along `z_max + 1` (x from `x_min` to `x_max + 1`) and along `x_max + 1` (z from
/// `z_min` to `z_max + 1`) are sampled too. Returns whether the box is accepted (both flags
/// cleared).
fn walk_cells_sampling(
    e: &mut Engine,
    height_field: u32,
    cells: Cells,
    rejection: &mut Rejection,
    emit: &mut dyn FnMut(&mut Engine, u32),
) -> bool {
    if cells.x_min <= cells.x_max {
        let mut x = cells.x_min;
        loop {
            if cells.z_min <= cells.z_max {
                let mut key = ((cells.z_min << 15).wrapping_add(x)).wrapping_mul(2);
                let mut z = cells.z_min;
                loop {
                    emit(e, key);
                    if rejection.rejects() {
                        rejection.sample(e, height_field, x, z);
                    }
                    z = z.wrapping_add(1);
                    key = key.wrapping_add(0x10000);
                    if z > cells.z_max {
                        break;
                    }
                }
            }
            x = x.wrapping_add(1);
            if x > cells.x_max {
                break;
            }
        }
        if !rejection.rejects() {
            return true;
        }
    }
    let x_edge = cells.x_max.wrapping_add(1);
    let z_edge = cells.z_max.wrapping_add(1);
    if cells.x_min <= x_edge {
        let mut x = cells.x_min;
        loop {
            rejection.sample(e, height_field, x, z_edge);
            x = x.wrapping_add(1);
            if x > x_edge {
                break;
            }
        }
    }
    if cells.z_min <= z_edge {
        let mut z = cells.z_min;
        loop {
            rejection.sample(e, height_field, x_edge, z);
            z = z.wrapping_add(1);
            if z > z_edge {
                break;
            }
        }
    }
    !rejection.rejects()
}

/// Runs `getExtentsForQueryAabb` on a copy of `aabb` (made in a local, as the function clamps
/// it). Returns the clamped copy's y range and the cells, or `None` when nothing was found.
fn query_cells(e: &mut Engine, collection: Ptr, aabb: Ptr) -> Option<(f32, f32, Cells)> {
    e.with_stack(0x40, |e, frame| {
        let bytes = e.mem.bytes(aabb.addr(), 0x20);
        e.mem.write(frame.addr(), &bytes);
        let found = frame.addr() + 0x20;
        let words = frame.addr() + 0x24;
        e.call(
            GET_EXTENTS_FOR_QUERY_AABB,
            &args![
                found,
                frame,
                collection,
                words,
                words + 4,
                words + 8,
                words + 12
            ],
        );
        if e.mem.u8(found) == 0 {
            return None;
        }
        Some((
            e.mem.f32(frame.addr() + 4),
            e.mem.f32(frame.addr() + 0x14),
            Cells {
                x_min: e.mem.u32(words),
                x_max: e.mem.u32(words + 4),
                z_min: e.mem.u32(words + 8),
                z_max: e.mem.u32(words + 12),
            },
        ))
    })
}

// Translated from 00d26ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldBvTreeShape::queryAabbImpl` (Xbox PDB), `(this, aabb, keys,
/// max keys)`: fills the `keys` array (at most `max keys`, a signed count) with the keys of the
/// cells under `aabb` and returns how many were written, or 0 when `getExtentsForQueryAabb`
/// finds nothing. The extents come from a copy of the box in a local (the function clamps it).
/// Every cell gives the key and then the key with bit 0 set, each only while there is room.
/// With the rejection test off (`+0x20` = 0) that is all; with it on the heights are sampled
/// (see the unit comment) and the answer is 0 unless both flags were cleared.
pub fn query_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldBvTreeShape>,
    aabb: Ptr,
    keys: Ptr,
    max_keys: i32,
) -> i32 {
    let collection = e.get(this, HkpTriSampledHeightFieldBvTreeShape::m_child_shape);
    let Some((min_y, max_y, cells)) = query_cells(e, collection, aabb) else {
        return 0;
    };
    let mut count = 0i32;
    let height_field = e.call(GET_HEIGHT_FIELD_SHAPE, &args![collection]).u32();
    let want_test = e.get(
        this,
        HkpTriSampledHeightFieldBvTreeShape::m_want_aabb_rejection_test,
    );
    let mut emit = |e: &mut Engine, key: u32| {
        if count < max_keys {
            e.mem.set_u32(
                keys.addr().wrapping_add((count as u32).wrapping_mul(4)),
                key,
            );
            count += 1;
            if count < max_keys {
                e.mem.set_u32(
                    keys.addr().wrapping_add((count as u32).wrapping_mul(4)),
                    key | 1,
                );
                count += 1;
            }
        }
    };
    if want_test == 0 {
        walk_cells(e, cells, &mut emit);
        return count;
    }
    let mut rejection = Rejection {
        min_y,
        max_y,
        all_below: true,
        all_above: true,
    };
    if walk_cells_sampling(e, height_field, cells, &mut rejection, &mut emit) {
        count
    } else {
        0
    }
}

/// Appends `key` to the `hkArray<u32>` at `array` (data `+0`, size `+4`, capacity and flags
/// `+8`): asks the memory router first, makes room with `00c90940` when the size equals the
/// capacity (`+8` masked with `3FFFFFFF`).
fn append_key(e: &mut Engine, array: u32, key: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data.wrapping_add(size.wrapping_mul(4)), key);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00d26e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldBvTreeShape::queryAabb` (Xbox PDB), `(this, aabb, hits)`: like
/// `queryAabbImpl`, but appends the keys (the key, then the key with bit 0 set) to the
/// `hkArray<u32>` `hits`, growing it through `00c90940`. When the rejection test throws the keys
/// away, the array's size goes back to what it was (after asking `00c908c0` for the capacity
/// `max(2 * capacity, size)` if the capacity is below that size). Returns nothing.
pub fn query_aabb(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldBvTreeShape>,
    aabb: Ptr,
    hits: Ptr,
) {
    let collection = e.get(this, HkpTriSampledHeightFieldBvTreeShape::m_child_shape);
    let Some((min_y, max_y, cells)) = query_cells(e, collection, aabb) else {
        return;
    };
    let height_field = e.call(GET_HEIGHT_FIELD_SHAPE, &args![collection]).u32();
    let original_size = e.mem.u32(hits.addr() + 4);
    let want_test = e.get(
        this,
        HkpTriSampledHeightFieldBvTreeShape::m_want_aabb_rejection_test,
    );
    let array = hits.addr();
    let mut emit = |e: &mut Engine, key: u32| {
        append_key(e, array, key);
        append_key(e, array, key | 1);
    };
    if want_test == 0 {
        walk_cells(e, cells, &mut emit);
        return;
    }
    let mut rejection = Rejection {
        min_y,
        max_y,
        all_below: true,
        all_above: true,
    };
    if walk_cells_sampling(e, height_field, cells, &mut rejection, &mut emit) {
        return;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    let wanted = original_size as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = if wanted < doubled { doubled } else { wanted };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity as u32, 4u32],
        );
    }
    e.mem.set_u32(array + 4, original_size);
}

// Translated from 00d5e070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriSampledHeightFieldBvTreeShape::~hkpTriSampledHeightFieldBvTreeShape` as its scalar
/// deleting destructor (Xbox PDB): puts the container vtable back, removes a reference from the
/// collection when there is one (`00c905b0`), sets the base vtables (`010c6fb0` at `+0x14`,
/// `0102e388` at `+0`) and, when bit 0 of `flags` is set, returns the block to the memory
/// router's allocator (slot 8) with the 16-bit size kept at `+4`. Returns `this`.
pub fn hkp_tri_sampled_height_field_bv_tree_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HkpTriSampledHeightFieldBvTreeShape>,
    flags: u8,
) -> Ptr<HkpTriSampledHeightFieldBvTreeShape> {
    let collection = e.get(this, HkpTriSampledHeightFieldBvTreeShape::m_child_shape);
    e.mem.set_u32(this.addr() + 0x14, CONTAINER_VTABLE);
    if collection.addr() != 0 {
        e.call(REMOVE_REFERENCE, &args![collection]);
    }
    e.mem.set_u32(this.addr() + 0x14, CONTAINER_BASE_VTABLE);
    e.mem.set_u32(this.addr(), SHAPE_BASE_VTABLE);
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
        entry!(
            0x00d2_68b0,
            calc_size_for_spu(Ptr<HkpTriSampledHeightFieldBvTreeShape>, u32, u32) -> i32
        ),
        entry!(
            0x00d2_68f0,
            query_obb(Ptr<HkpTriSampledHeightFieldBvTreeShape>, Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d2_6940,
            get_extents_for_query_aabb(
                Ptr,
                Ptr,
                Ptr<HkpTriSampledHeightFieldCollection>,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            ) -> Ptr
        ),
        entry!(
            0x00d2_6ae0,
            get_aabb_impl(Ptr<HkpTriSampledHeightFieldBvTreeShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d2_6b00,
            cast_ray_impl(Ptr<HkpTriSampledHeightFieldBvTreeShape>, u32, u32, u32) -> u32
        ),
        entry!(
            0x00d2_6b30,
            cast_ray_with_collector_impl(Ptr<HkpTriSampledHeightFieldBvTreeShape>, u32, u32, u32)
        ),
        entry!(
            0x00d2_6b50,
            fn_00d26b50(
                Ptr<HkpTriSampledHeightFieldBvTreeShape>,
                Ptr,
                u8,
            ) -> Ptr<HkpTriSampledHeightFieldBvTreeShape>
        ),
        entry!(
            0x00d2_6ba0,
            query_aabb_impl(Ptr<HkpTriSampledHeightFieldBvTreeShape>, Ptr, Ptr, i32) -> i32
        ),
        entry!(
            0x00d2_6e50,
            query_aabb(Ptr<HkpTriSampledHeightFieldBvTreeShape>, Ptr, Ptr)
        ),
        entry!(
            0x00d5_e070,
            hkp_tri_sampled_height_field_bv_tree_shape_scalar_deleting_destructor(
                Ptr<HkpTriSampledHeightFieldBvTreeShape>,
                u8,
            )
                -> Ptr<
                HkpTriSampledHeightFieldBvTreeShape,
            >
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{calls, engine, log_double, new_log};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Slot doubles of the height field and of the collection.
    const AABB_SLOT: u32 = 0x0900_001c;
    const HEIGHT_SLOT: u32 = 0x0900_0034;
    const COLLECTION_SIZE_SLOT: u32 = 0x0900_0118;
    const RAY_SLOT: u32 = 0x0900_0020;
    const RAY_COLLECTOR_SLOT: u32 = 0x0900_0024;
    const FREE_SLOT: u32 = 0x0900_0208;

    /// A shape over a collection over a height field. The height field has a 12 by 14 grid,
    /// floatToIntScale `(2, 1, 2, 1)`, offset `(1, 0, 1, 0)` and y scale 1.0 (`+0x24`). Returns
    /// `(shape, collection, height field)`.
    fn shape(e: &mut Engine, rejection_test: u8) -> (u32, u32, u32) {
        let hf_vtable = e.mem.alloc(0x40);
        for slot in [0x1c, 0x20, 0x24, 0x34] {
            e.mem.set_u32(hf_vtable + slot, 0x0900_0000 + slot);
        }
        let height_field = e.mem.alloc(0x60);
        e.mem.set_u32(height_field, hf_vtable);
        e.mem.set_u32(height_field + 0x10, 12);
        e.mem.set_u32(height_field + 0x14, 14);
        e.mem.set_f32(height_field + 0x24, 1.0);
        for (lane, value) in [2.0f32, 1.0, 2.0, 1.0].into_iter().enumerate() {
            e.mem.set_f32(height_field + 0x30 + 4 * lane as u32, value);
        }
        for (lane, value) in [1.0f32, 0.0, 1.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(height_field + 0x40 + 4 * lane as u32, value);
        }
        let collection_vtable = e.mem.alloc(0x40);
        e.mem
            .set_u32(collection_vtable + 0x18, COLLECTION_SIZE_SLOT);
        e.mem.set_u32(collection_vtable + 0x1c, AABB_SLOT + 0x100);
        let collection = e.mem.alloc(0x40);
        e.mem.set_u32(collection, collection_vtable);
        e.mem.set_u32(collection + 0x18, height_field);
        let shape = e.mem.alloc(0x30);
        e.mem.set_u32(shape + 0x18, collection);
        e.mem.set_u8(shape + 0x20, rejection_test);
        (shape, collection, height_field)
    }

    #[test]
    fn calc_size_for_spu_records_sizes_up_to_the_limit() {
        for (answer, result, recorded) in
            [(0x80u32, 0x30, 0x80), (0x100, 0x30, 0x100), (0x101, -1, 0)]
        {
            let mut e = engine();
            let (shape, collection, _) = shape(&mut e, 0);
            let seen = Rc::new(RefCell::new(Vec::new()));
            let sink = Rc::clone(&seen);
            e.register_double(COLLECTION_SIZE_SLOT, move |_, a| {
                sink.borrow_mut().push(a.to_vec());
                Ret {
                    eax: answer,
                    ..Ret::default()
                }
            });
            let got = e.call(0x00d2_68b0, &args![shape, 0x77u32, 0x99u32]).i32();
            assert_eq!(got, result);
            assert_eq!(e.mem.u32(shape + 0x1c), recorded);
            assert_eq!(*seen.borrow(), vec![vec![collection, 0x77, 0x100]]);
        }
    }

    #[test]
    fn query_obb_builds_the_box_and_queries_it() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, 0x00c9_e120, 0);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0x30, 0x0900_0030);
        log_double(&mut e, &log, 0x0900_0030, 0x55);
        let shape = e.mem.alloc(0x30);
        e.mem.set_u32(shape, vtable);
        e.call(
            0x00d2_68f0,
            &args![shape, 0x1000u32, 0x2000u32, 0.5f32, 0x3000u32],
        );
        let built = calls(&log, 0x00c9_e120);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][..3], [0x1000, 0x2000, 0.5f32.to_bits()]);
        let query = calls(&log, 0x0900_0030);
        assert_eq!(query, vec![vec![shape, built[0][3], 0x3000]]);
    }

    /// Runs `getExtentsForQueryAabb` for a box over the grid of [`shape`] with the radius 0.5
    /// and the height field answering the box `(box_min, box_max)`. Returns `(found, box after,
    /// x_min, x_max, z_min, z_max)`.
    fn run_extents(
        min: Vec4,
        max: Vec4,
        box_min: Vec4,
        box_max: Vec4,
    ) -> (u8, [f32; 8], [u32; 4], Vec<u32>) {
        let mut e = engine();
        let (_, collection, _) = shape(&mut e, 0);
        e.mem.set_f32(collection + 0x20, 0.5);
        for lane in 0..4 {
            e.mem.set_u32(LANE_MASK + 4 * lane, u32::MAX);
            e.mem.set_f32(MANTISSA_BIAS + 4 * lane, 65536.0);
        }
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        let answer: Vec<f32> = box_min.into_iter().chain(box_max).collect();
        e.register_double(AABB_SLOT, move |e, a| {
            sink.borrow_mut().extend_from_slice(&a[..3]);
            for (i, value) in answer.iter().enumerate() {
                e.mem.set_f32(a[3] + 4 * i as u32, *value);
            }
            Ret::default()
        });
        let aabb = e.mem.alloc(0x20);
        store_vec(&mut e, aabb, min);
        store_vec(&mut e, aabb + 0x10, max);
        let out = e.mem.alloc(0x20);
        e.mem.write(out, &[0xaa; 0x20]);
        e.call(
            0x00d2_6940,
            &args![out, aabb, collection, out + 4, out + 8, out + 12, out + 16],
        );
        let written = std::array::from_fn(|i| e.mem.f32(aabb + 4 * i as u32));
        let cells = std::array::from_fn(|i| e.mem.u32(out + 4 + 4 * i as u32));
        let seen = seen.borrow().clone();
        (e.mem.u8(out), written, cells, seen)
    }

    const HUGE: f32 = 1000.0;

    #[test]
    fn extents_convert_the_clamped_box_to_cells() {
        // Box (1.5 .. 4.5, 2.5 .. 5.5) widened by 0.5 to (1 .. 5, 2 .. 6); offset 1, scale 2:
        // x (1 + 1) * 2 = 4 .. (5 + 1) * 2 = 12 but the limit is 12 - 2 = 10 for x and
        // 14 - 2 = 12 for z, z: (2 + 1) * 2 = 6 .. (6 + 1) * 2 = 14, clamped to 12.
        let (found, written, cells, seen) = run_extents(
            [1.5, 0.0, 2.5, 0.0],
            [4.5, 1.0, 5.5, 1.0],
            [-HUGE; 4],
            [HUGE; 4],
        );
        assert_eq!(found, 1);
        assert_eq!(cells, [4, 10, 6, 12]);
        // The box was widened by the radius and written back (min - 0.5, max + 0.5).
        assert_eq!(written, [1.0, -0.5, 2.0, -0.5, 5.0, 1.5, 6.0, 1.5]);
        // The height field was asked with the transform and a tolerance of 0.0.
        assert_eq!(seen[1..], [AABB_TRANSFORM, 0]);
    }

    #[test]
    fn extents_clamp_the_box_to_the_height_field_box() {
        // The height field's own box is (3 .. 4) in x and (0 .. 100) in z: the widened box
        // (1 .. 5) is cut to it, and z (2 .. 6) stays.
        let (found, written, cells, _) = run_extents(
            [1.5, 0.0, 2.5, 0.0],
            [4.5, 1.0, 5.5, 1.0],
            [3.0, -HUGE, -HUGE, -HUGE],
            [4.0, HUGE, 100.0, HUGE],
        );
        assert_eq!(found, 1);
        assert_eq!(written[0], 3.0);
        assert_eq!(written[4], 4.0);
        // x: (3 + 1) * 2 = 8 .. (4 + 1) * 2 = 10.
        assert_eq!(cells, [8, 10, 6, 12]);
    }

    #[test]
    fn extents_find_nothing_when_the_box_is_outside() {
        let (found, written, cells, _) = run_extents(
            [20.0, 0.0, 2.5, 0.0],
            [21.0, 1.0, 5.5, 1.0],
            [0.0, -HUGE, -HUGE, -HUGE],
            [10.0, HUGE, HUGE, HUGE],
        );
        assert_eq!(found, 0);
        // The clamped box is still written back; the cells are not.
        assert_eq!(written[0], 19.5);
        assert_eq!(written[4], 10.0);
        assert_eq!(cells, [0xaaaa_aaaa; 4]);
    }

    #[test]
    fn extents_clamp_negative_cells_to_zero() {
        let (found, _, cells, _) = run_extents(
            [-20.0, 0.0, -20.0, 0.0],
            [-10.0, 1.0, 1.0, 1.0],
            [-HUGE; 4],
            [HUGE; 4],
        );
        assert_eq!(found, 1);
        assert_eq!(cells, [0, 0, 0, 5]);
    }

    #[test]
    fn get_aabb_impl_forwards_to_the_collection() {
        let mut e = engine();
        let (shape, collection, _) = shape(&mut e, 0);
        let log = new_log();
        log_double(&mut e, &log, AABB_SLOT + 0x100, 0);
        e.call(0x00d2_6ae0, &args![shape, 0x10u32, 1.25f32, 0x30u32]);
        assert_eq!(
            calls(&log, AABB_SLOT + 0x100),
            vec![vec![collection, 0x10, 1.25f32.to_bits(), 0x30]]
        );
    }

    #[test]
    fn cast_ray_impl_calls_the_height_field_and_returns_the_first_word() {
        let mut e = engine();
        let (shape, _, height_field) = shape(&mut e, 0);
        let log = new_log();
        log_double(&mut e, &log, RAY_SLOT, 0x1234);
        let result = e.call(0x00d2_6b00, &args![shape, 7u32, 8u32, 9u32]).u32();
        assert_eq!(result, 7);
        assert_eq!(calls(&log, RAY_SLOT), vec![vec![height_field, 7, 8, 9]]);
    }

    #[test]
    fn cast_ray_with_collector_impl_calls_the_height_field() {
        let mut e = engine();
        let (shape, _, height_field) = shape(&mut e, 0);
        let log = new_log();
        log_double(&mut e, &log, RAY_COLLECTOR_SLOT, 0);
        e.call(0x00d2_6b30, &args![shape, 7u32, 8u32, 9u32]);
        assert_eq!(
            calls(&log, RAY_COLLECTOR_SLOT),
            vec![vec![height_field, 7, 8, 9]]
        );
    }

    #[test]
    fn construction_fills_the_header_and_references_the_collection() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, ADD_REFERENCE, 0);
        let this = e.mem.alloc(0x30);
        e.mem.write(this, &[0xaa; 0x30]);
        let result = e
            .call(0x00d2_6b50, &args![this, 0x4242u32, 0x1234_5601u32])
            .u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        assert_eq!(e.mem.u32(this), 0x010c_e404);
        assert_eq!(e.mem.u32(this + 0x14), 0x010c_6fd0);
        assert_eq!(e.mem.u32(this + 0x18), 0x4242);
        // Only the low byte of the flag word is stored.
        assert_eq!(e.mem.u8(this + 0x20), 1);
        assert_eq!(e.mem.u8(this + 0x21), 0xaa);
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![0x4242]]);
    }

    /// A double for `getExtentsForQueryAabb` that answers `cells` (`None`: not found) and
    /// writes `(min_y, max_y)` into the copy of the box it was given.
    fn extents_double(e: &mut Engine, cells: Option<[u32; 4]>, min_y: f32, max_y: f32) {
        e.register_double(GET_EXTENTS_FOR_QUERY_AABB, move |e, a| {
            e.mem.set_f32(a[1] + 4, min_y);
            e.mem.set_f32(a[1] + 0x14, max_y);
            match cells {
                None => e.mem.set_u8(a[0], 0),
                Some(cells) => {
                    e.mem.set_u8(a[0], 1);
                    for (i, cell) in cells.into_iter().enumerate() {
                        e.mem.set_u32(a[3 + i], cell);
                    }
                }
            }
            Ret::default()
        });
    }

    #[test]
    fn get_extents_answers_its_found_pointer() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let field = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(field, vtable);
        e.mem.set_u32(vtable + 0x1c, 0x0900_0d00);
        e.register_double(0x0900_0d00, |_, _| Ret::default());
        e.register_double(GET_HEIGHT_FIELD_SHAPE, move |_, _| Ret {
            eax: field,
            ..Ret::default()
        });
        let collection = e.mem.alloc(0x80);
        let aabb = e.mem.alloc(0x20);
        let found = e.mem.alloc(4);
        let cells = e.mem.alloc(0x10);
        // An empty box: nothing is found, and EAX is `found` at the RET (the hkBool result
        // pointer the callers read through).
        let answered = e
            .call(
                0x00d2_6940,
                &args![
                    found,
                    aabb,
                    collection,
                    cells,
                    cells + 4,
                    cells + 8,
                    cells + 12
                ],
            )
            .u32();
        assert_eq!(answered, found);
        assert_eq!(e.mem.u8(found), 0);
    }

    /// A height double answering `height(x, z)` and recording the calls.
    fn height_double(
        e: &mut Engine,
        height: impl Fn(u32, u32) -> f32 + 'static,
    ) -> Rc<RefCell<Vec<(u32, u32)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(HEIGHT_SLOT, move |_, a| {
            sink.borrow_mut().push((a[1], a[2]));
            Ret {
                st0: f64::from(height(a[1], a[2])),
                ..Ret::default()
            }
        });
        seen
    }

    fn key(x: u32, z: u32) -> u32 {
        (z * 0x8000 + x) * 2
    }

    /// Runs `queryAabbImpl` with the extents `cells` (x 1..=2, z 3..=4 unless given), the box
    /// y range `0..10`, and the given room; returns the answer and the written words.
    fn run_impl(
        rejection_test: u8,
        cells: Option<[u32; 4]>,
        room: i32,
        height: impl Fn(u32, u32) -> f32 + 'static,
    ) -> (i32, Vec<u32>, Vec<(u32, u32)>) {
        let mut e = engine();
        let (shape, _, _) = shape(&mut e, rejection_test);
        extents_double(&mut e, cells, 0.0, 10.0);
        let samples = height_double(&mut e, height);
        let aabb = e.mem.alloc(0x20);
        let keys = e.mem.alloc(0x100);
        let count = e.call(0x00d2_6ba0, &args![shape, aabb, keys, room]).i32();
        let words = (0..16).map(|i| e.mem.u32(keys + 4 * i)).collect();
        let samples = samples.borrow().clone();
        (count, words, samples)
    }

    #[test]
    fn query_aabb_impl_returns_nothing_when_no_extents_are_found() {
        let (count, words, samples) = run_impl(1, None, 8, |_, _| 5.0);
        assert_eq!(count, 0);
        assert_eq!(words, vec![0; 16]);
        assert!(samples.is_empty());
    }

    #[test]
    fn query_aabb_impl_lists_both_triangles_of_every_cell() {
        let (count, words, samples) = run_impl(0, Some([1, 2, 3, 4]), 16, |_, _| 5.0);
        assert_eq!(count, 8);
        let expected: Vec<u32> = [(1, 3), (1, 4), (2, 3), (2, 4)]
            .into_iter()
            .flat_map(|(x, z)| [key(x, z), key(x, z) | 1])
            .collect();
        assert_eq!(words[..8], expected[..]);
        // No heights are sampled without the rejection test.
        assert!(samples.is_empty());
    }

    #[test]
    fn query_aabb_impl_stops_at_the_room_it_is_given() {
        let (count, words, _) = run_impl(0, Some([1, 2, 3, 4]), 3, |_, _| 5.0);
        assert_eq!(count, 3);
        assert_eq!(words[..4], [key(1, 3), key(1, 3) | 1, key(1, 4), 0]);
        let (count, _, _) = run_impl(0, Some([1, 2, 3, 4]), 0, |_, _| 5.0);
        assert_eq!(count, 0);
        // A negative room (signed compare) writes nothing either.
        let (count, words, _) = run_impl(0, Some([1, 2, 3, 4]), -1, |_, _| 5.0);
        assert_eq!((count, words[0]), (0, 0));
    }

    #[test]
    fn query_aabb_impl_does_nothing_for_an_empty_cell_range() {
        let (count, _, _) = run_impl(0, Some([3, 2, 3, 4]), 16, |_, _| 5.0);
        assert_eq!(count, 0);
        let (count, _, _) = run_impl(0, Some([1, 2, 4, 3]), 16, |_, _| 5.0);
        assert_eq!(count, 0);
    }

    #[test]
    fn query_aabb_impl_accepts_when_the_heights_straddle_the_box() {
        // Every sample is inside (0, 10): both flags clear on the first cell and the rest of
        // the cells and the extra row and column are not sampled.
        let (count, words, samples) = run_impl(1, Some([1, 2, 3, 4]), 16, |_, _| 5.0);
        assert_eq!(count, 8);
        assert_eq!(words[0], key(1, 3));
        // The first cell clears both flags; the other cells are not sampled.
        assert_eq!(samples, vec![(1, 3)]);
    }

    #[test]
    fn query_aabb_impl_rejects_terrain_above_or_below_the_box() {
        // All above the top: the extra row and column are sampled too, and the answer is 0.
        let (count, _, samples) = run_impl(1, Some([1, 2, 3, 4]), 16, |_, _| 20.0);
        assert_eq!(count, 0);
        // Cells (4), then row z = 5 for x = 1..=3 and column x = 3 for z = 3..=5.
        let mut expected = vec![(1, 3), (1, 4), (2, 3), (2, 4)];
        expected.extend([(1, 5), (2, 5), (3, 5), (3, 3), (3, 4), (3, 5)]);
        assert_eq!(samples, expected);
        // All below the bottom.
        let (count, _, _) = run_impl(1, Some([1, 2, 3, 4]), 16, |_, _| -5.0);
        assert_eq!(count, 0);
        // A sample exactly at the bottom is not above it, a NaN is never above anything.
        let (count, _, _) = run_impl(1, Some([1, 2, 3, 4]), 16, |_, _| 0.0);
        assert_eq!(count, 0);
        let (count, _, _) = run_impl(1, Some([1, 2, 3, 4]), 16, |_, _| f32::NAN);
        assert_eq!(count, 0);
    }

    #[test]
    fn query_aabb_impl_accepts_through_the_extra_row() {
        // The cells are all above the top; the corner sample (3, 5) is inside.
        let (count, _, samples) = run_impl(1, Some([1, 2, 3, 4]), 16, |x, z| {
            if (x, z) == (3, 5) {
                5.0
            } else {
                20.0
            }
        });
        assert_eq!(count, 8);
        assert_eq!(samples.len(), 10);
    }

    /// The array double: `hkArray<u32>` growing through the `00c90940` double.
    struct ArrayRig {
        e: Engine,
        shape: u32,
        array: u32,
        reserve_more: Rc<RefCell<Vec<Vec<u32>>>>,
        reserve: Rc<RefCell<Vec<Vec<u32>>>>,
        allocator: u32,
    }

    /// A rig for `queryAabb`: a router double whose allocator is a marker, an array of
    /// `size` entries with capacity `capacity`, and doubles for both reserve functions (the
    /// first doubles the room in a new block).
    fn array_rig(rejection_test: u8, size: u32, capacity: u32) -> ArrayRig {
        let mut e = engine();
        let (shape, _, _) = shape(&mut e, rejection_test);
        let router = e.mem.alloc(0x20);
        let allocator = 0x7777_0000;
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x400);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity | 0x8000_0000);
        let reserve_more = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&reserve_more);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            let old = e.mem.u32(a[1]);
            let capacity = e.mem.u32(a[1] + 8) & 0x3fff_ffff;
            let block = e.mem.alloc(0x400);
            let bytes = e.mem.bytes(old, 4 * capacity);
            e.mem.write(block, &bytes);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, capacity * 2);
            Ret::default()
        });
        let reserve = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&reserve);
        e.register_double(ARRAY_RESERVE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        ArrayRig {
            e,
            shape,
            array,
            reserve_more,
            reserve,
            allocator,
        }
    }

    fn array_keys(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    #[test]
    fn query_aabb_appends_both_triangles_and_grows_the_array() {
        let mut rig = array_rig(0, 1, 2);
        let data = rig.e.mem.u32(rig.array);
        rig.e.mem.set_u32(data, 0xfeed);
        extents_double(&mut rig.e, Some([1, 2, 3, 4]), 0.0, 10.0);
        let aabb = rig.e.mem.alloc(0x20);
        rig.e.call(0x00d2_6e50, &args![rig.shape, aabb, rig.array]);
        let keys = array_keys(&rig.e, rig.array);
        let mut expected = vec![0xfeed];
        for (x, z) in [(1, 3), (1, 4), (2, 3), (2, 4)] {
            expected.extend([key(x, z), key(x, z) | 1]);
        }
        assert_eq!(keys, expected);
        // Growth was asked for each time the size reached the capacity (2, then 4, then 8).
        let asked = rig.reserve_more.borrow();
        assert_eq!(asked.len(), 3);
        assert_eq!(asked[0], vec![rig.allocator, rig.array, 4]);
    }

    #[test]
    fn query_aabb_leaves_the_array_alone_when_nothing_is_found() {
        let mut rig = array_rig(1, 1, 2);
        extents_double(&mut rig.e, None, 0.0, 10.0);
        let aabb = rig.e.mem.alloc(0x20);
        rig.e.call(0x00d2_6e50, &args![rig.shape, aabb, rig.array]);
        assert_eq!(rig.e.mem.u32(rig.array + 4), 1);
        assert!(rig.reserve_more.borrow().is_empty());
    }

    #[test]
    fn query_aabb_with_the_rejection_test_keeps_accepted_keys() {
        let mut rig = array_rig(1, 0, 16);
        extents_double(&mut rig.e, Some([1, 1, 3, 3]), 0.0, 10.0);
        let samples = height_double(&mut rig.e, |_, _| 5.0);
        let aabb = rig.e.mem.alloc(0x20);
        rig.e.call(0x00d2_6e50, &args![rig.shape, aabb, rig.array]);
        assert_eq!(
            array_keys(&rig.e, rig.array),
            vec![key(1, 3), key(1, 3) | 1]
        );
        assert_eq!(*samples.borrow(), vec![(1, 3)]);
    }

    #[test]
    fn query_aabb_with_the_rejection_test_restores_the_size_when_rejecting() {
        let mut rig = array_rig(1, 1, 16);
        extents_double(&mut rig.e, Some([1, 1, 3, 3]), 0.0, 10.0);
        let samples = height_double(&mut rig.e, |_, _| 20.0);
        let aabb = rig.e.mem.alloc(0x20);
        rig.e.call(0x00d2_6e50, &args![rig.shape, aabb, rig.array]);
        assert_eq!(rig.e.mem.u32(rig.array + 4), 1);
        // The cell, the extra row (x 1..=2 at z 4) and column (x 2, z 3..=4).
        assert_eq!(
            *samples.borrow(),
            vec![(1, 3), (1, 4), (2, 4), (2, 3), (2, 4)]
        );
        // The capacity (16) is not below the size, so nothing is reserved.
        assert!(rig.reserve.borrow().is_empty());
    }

    #[test]
    fn query_aabb_reserves_when_the_capacity_is_below_the_restored_size() {
        // An array whose size is above its capacity (the capacity is only compared).
        for (size, capacity, wanted) in [(3u32, 2u32, 4u32), (7, 2, 7)] {
            let mut rig = array_rig(1, size, capacity);
            extents_double(&mut rig.e, Some([1, 1, 3, 3]), 0.0, 10.0);
            height_double(&mut rig.e, |_, _| 20.0);
            let aabb = rig.e.mem.alloc(0x20);
            rig.e.call(0x00d2_6e50, &args![rig.shape, aabb, rig.array]);
            assert_eq!(
                *rig.reserve.borrow(),
                vec![vec![rig.allocator, rig.array, wanted, 4]]
            );
            assert_eq!(rig.e.mem.u32(rig.array + 4), size);
        }
    }

    #[test]
    fn destruction_releases_the_collection_and_frees_on_bit_zero() {
        for (collection, flags, freed) in [
            (0x4242u32, 0xfeu8, false),
            (0, 1, true),
            (0x4242, 0x03, true),
        ] {
            let mut e = engine();
            let log = new_log();
            log_double(&mut e, &log, REMOVE_REFERENCE, 0);
            log_double(&mut e, &log, FREE_SLOT, 0);
            let vtable = e.mem.alloc(0x20);
            e.mem.set_u32(vtable + 8, FREE_SLOT);
            let allocator = e.mem.alloc(0x10);
            e.mem.set_u32(allocator, vtable);
            let router = e.mem.alloc(0x20);
            e.mem.set_u32(router + 0x10, allocator);
            e.register_double(MEMORY_ROUTER, move |_, _| Ret {
                eax: router,
                ..Ret::default()
            });
            let this = e.mem.alloc(0x30);
            e.mem.set_u16(this + 4, 0x30);
            e.mem.set_u32(this + 0x18, collection);
            let result = e
                .call(0x00d5_e070, &args![this, u32::from(flags) | 0x100])
                .u32();
            assert_eq!(result, this);
            assert_eq!(e.mem.u32(this), 0x0102_e388);
            assert_eq!(e.mem.u32(this + 0x14), 0x010c_6fb0);
            let removed = calls(&log, REMOVE_REFERENCE);
            let expected: Vec<Vec<u32>> = if collection == 0 {
                vec![]
            } else {
                vec![vec![collection]]
            };
            assert_eq!(removed, expected);
            let expected: Vec<Vec<u32>> = if freed {
                vec![vec![allocator, this, 0x30]]
            } else {
                vec![]
            };
            assert_eq!(calls(&log, FREE_SLOT), expected);
        }
    }
}
