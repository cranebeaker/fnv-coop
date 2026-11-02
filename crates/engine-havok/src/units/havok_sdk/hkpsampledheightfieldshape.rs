//! `Havok SDK/hkpsampledheightfieldshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    inverse_length, load_vec, profile_record, shufps, store_bits, store_vec, sum3, zip_with, Vec4,
    PROFILE_END,
};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d262e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d2_4660`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d262e0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d2_4660, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// Mask clearing the sign bit of every lane (`.rdata`), used as `ANDPS`.
const ABS_MASK_01017D10: u32 = 0x0101_7d10;
/// Mask holding the sign bit of every lane (`.rdata`), used as `XORPS`.
const SIGN_MASK_010C44D0: u32 = 0x010c_44d0;

// Translated from 00d26230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAabbUtil::calcAabb_ov3` (map name). cdecl, five words, SSE (`f32` per
/// lane): the box of an oriented box with half extents `half` and centre
/// `centre` under the transform whose columns are the four vectors at
/// `transform` (`+0`, `+0x10`, `+0x20` and the translation at `+0x30`).
/// With `a0`, `a1`, `a2` the first three columns, per lane:
/// `extent = (|hx*a0| + |hy*a1|) + (|hz*a2| + tolerance)` and
/// `middle = (cx*a0 + cy*a1) + (cz*a2 + translation)`; `out[0..4]` gets
/// `-extent + middle` (the sign flipped by the mask at `010c44d0`) and
/// `out[4..8]` gets `middle + extent`.
pub fn hk_aabb_util_calc_aabb_ov3(
    e: &mut Engine,
    transform: Ptr,
    half: Ptr,
    centre: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let lane = |e: &Engine, base: u32, i: u32| e.mem.f32(base + 4 * i);
    let abs = |e: &Engine, value: f32, i: u32| {
        f32::from_bits(value.to_bits() & e.mem.u32(ABS_MASK_01017D10 + 4 * i))
    };
    let (half_x, half_y, half_z) = (
        lane(e, half.addr(), 0),
        lane(e, half.addr(), 1),
        lane(e, half.addr(), 2),
    );
    let (centre_x, centre_y, centre_z) = (
        lane(e, centre.addr(), 0),
        lane(e, centre.addr(), 1),
        lane(e, centre.addr(), 2),
    );
    let mut minimum = [0.0f32; 4];
    let mut maximum = [0.0f32; 4];
    for i in 0..4u32 {
        let (a0, a1, a2) = (
            lane(e, transform.addr(), i),
            lane(e, transform.addr() + 0x10, i),
            lane(e, transform.addr() + 0x20, i),
        );
        let translation = lane(e, transform.addr() + 0x30, i);
        // `MULPS`/`ADDPS` with the SSE NaN rules, the destination operand first.
        use crate::sse::{add, mul};
        let extent = add(
            add(abs(e, mul(half_x, a0), i), abs(e, mul(half_y, a1), i)),
            add(abs(e, mul(half_z, a2), i), tolerance),
        );
        let middle = add(
            add(mul(centre_x, a0), mul(centre_y, a1)),
            add(mul(centre_z, a2), translation),
        );
        let negated = f32::from_bits(extent.to_bits() ^ e.mem.u32(SIGN_MASK_010C44D0 + 4 * i));
        minimum[i as usize] = add(negated, middle);
        maximum[i as usize] = add(middle, extent);
    }
    for (i, value) in maximum.iter().enumerate() {
        e.mem.set_f32(out.addr() + 0x10 + 4 * i as u32, *value);
    }
    for (i, value) in minimum.iter().enumerate() {
        e.mem.set_f32(out.addr() + 4 * i as u32, *value);
    }
}

/// `a > b` is false: `a` is less, equal, or the pair is unordered. This is what the exe's
/// `FCOM` followed by `TEST AH,0x41` and a jump on zero decides.
fn not_greater(a: f32, b: f32) -> bool {
    a.partial_cmp(&b) != Some(std::cmp::Ordering::Greater)
}

/// Slot `0x34` of the shape's vtable: the height of the sample `(x, z)`, as an x87 value.
const HEIGHT_SLOT: u32 = 0x34;
/// The `double` `0.5` the middle of the heights is scaled by.
const HALF_F64: u32 = 0x0101_1588;
/// Four `0.5` lanes the half extents are scaled by.
const HALF_LANES: u32 = 0x010c_71d0;

// Translated from 00d249f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSampledHeightFieldShape::getAabbImpl` (map name), thiscall with three arguments
/// (`RET 0xC`): `ECX` is `this`. The shape keeps its sample counts at `+0x10` and `+0x14`, its
/// middle height at `+0x18`, its height scale at `+0x24`, and its extents at `+0x50`
/// (`+0x54` is the height range). While the range is negative (not yet computed) every sample
/// is asked of slot `0x34`, in a loop over the first count then the second; the smallest and
/// largest height, scaled and ordered, give the middle (stored at `+0x18`) and the range (stored
/// at `+0x54`). Then the box `(half x, half z)` is asked of `calcAabb_ov3` with the half extents
/// `(x, y, z, w)` and the centre `(half x, middle, half z, w)` built from the extents times `0.5`.
pub fn hk_sampled_height_field_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let this = this.addr();
    let sample =
        |e: &mut Engine, x: i32, z: i32| e.vcall(this, HEIGHT_SLOT, &args![x, z]).st0 as f32;
    let mut lowest = sample(e, 0, 0);
    let mut highest = lowest;
    if e.mem.f32(this + 0x54) < 0.0 {
        let mut x: i32 = 0;
        if x < e.mem.i32(this + 0x10) {
            loop {
                let mut z: i32 = 0;
                if z < e.mem.i32(this + 0x14) {
                    loop {
                        let height = sample(e, x, z);
                        if not_greater(height, lowest) {
                            lowest = height;
                        }
                        if not_greater(highest, height) {
                            highest = height;
                        }
                        z += 1;
                        if z >= e.mem.i32(this + 0x14) {
                            break;
                        }
                    }
                }
                x += 1;
                if x >= e.mem.i32(this + 0x10) {
                    break;
                }
            }
        }
        let scale = f64::from(e.mem.f32(this + 0x24));
        let lowest_scaled = (scale * f64::from(lowest)) as f32;
        let highest_scaled = (scale * f64::from(highest)) as f32;
        // Order the pair: the larger goes up, the smaller down; an equal or unordered pair
        // takes the second value twice.
        let (low, high) = if highest_scaled > lowest_scaled {
            (lowest_scaled, highest_scaled)
        } else if lowest_scaled > highest_scaled {
            (highest_scaled, lowest_scaled)
        } else {
            (highest_scaled, highest_scaled)
        };
        let middle = ((f64::from(high) + f64::from(low)) * e.mem.f64(HALF_F64)) as f32;
        e.mem.set_f32(this + 0x18, middle);
        e.mem
            .set_f32(this + 0x54, (f64::from(high) - f64::from(low)) as f32);
    }
    let extents = load_vec(e, this + 0x50);
    let half = zip_with(extents, load_vec(e, HALF_LANES), |a, b| a * b);
    let middle_bits = x87_quiet_bits(e.mem.u32(this + 0x18));
    let tolerance_bits = x87_quiet_bits(tolerance.to_bits());
    e.with_stack(0x20, |e, block| {
        let centre = block.addr();
        let half_extents = centre + 0x10;
        store_bits(
            e,
            centre,
            [
                half[0].to_bits(),
                middle_bits,
                half[2].to_bits(),
                half[3].to_bits(),
            ],
        );
        store_vec(e, half_extents, half);
        e.call(
            0x00d2_6230,
            &args![transform, half_extents, centre, tolerance_bits, out],
        );
    });
}

/// `hkpSampledHeightFieldShape::castRayInternal` (`00d24b50`, thiscall, `RET 0x14`):
/// `(ray, extra, flag, fraction, collector)`; the flag is read as a byte.
const CAST_RAY_INTERNAL: u32 = 0x00d2_4b50;
/// The vtable of the nearest-hit collector (`addRayHit` is `00d261b0`).
const NEAREST_HIT_COLLECTOR_VTABLE: u32 = 0x010c_e304;
/// The vtable of the collector that rotates the hit normal (`addRayHit` is `00d26310`).
const ROTATE_NORMAL_HIT_COLLECTOR_VTABLE: u32 = 0x010c_e310;
/// The vtable of the pure virtual base the nearest-hit collector ends as.
const PURE_VIRTUAL_VTABLE: u32 = 0x0101_e594;
/// The CRT `free` the scalar deleting destructor releases the collector with (cdecl, one word).
const FREE: u32 = 0x0040_1030;

// Translated from 00d26080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSampledHeightFieldShape::castRayImpl` (map name), thiscall with three arguments
/// (`RET 0xC`): `ECX` is `this`. Builds a nearest-hit collector on the stack (vtable
/// `010ce304`, `+4` the float at `input + 0x10`, `+8` the found flag cleared, `+0xC` the
/// `input` pointer) and casts `ray` through `castRayInternal` with extra word 0, flag 0 and
/// fraction `0.0`. The found flag of the collector is then stored in the byte at `found`.
/// Answers `found`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hk_sampled_height_field_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr,
    found: Ptr,
    ray: Ptr,
    input: Ptr,
) -> Ptr {
    let fraction = x87_quiet_bits(e.mem.u32(input.addr() + 0x10));
    let hit = e.with_stack(0x10, |e, collector| {
        let collector = collector.addr();
        e.mem.set_u32(collector, NEAREST_HIT_COLLECTOR_VTABLE);
        e.mem.set_u32(collector + 4, fraction);
        e.mem.set_u8(collector + 8, 0);
        e.mem.set_u32(collector + 0xc, input.addr());
        e.call(
            CAST_RAY_INTERNAL,
            &args![this, ray, 0u32, 0u32, 0u32, collector],
        );
        e.mem.u8(collector + 8)
    });
    e.mem.set_u8(found.addr(), hit);
    found
}

// Translated from 00d260d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSampledHeightFieldShape::castRayWithCollectorImpl` (map name), thiscall with three
/// arguments (`RET 0xC`): `ECX` is `this`. Builds a collector on the stack (vtable `010ce310`,
/// `+4` the float at `hits + 4`, `+8` the `hits` pointer) and casts `ray` through
/// `castRayInternal` with `extra` as the second word, flag 0 and fraction `0.0`.
pub fn hk_sampled_height_field_shape_cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr,
    ray: Ptr,
    extra: u32,
    hits: Ptr,
) {
    let fraction = x87_quiet_bits(e.mem.u32(hits.addr() + 4));
    e.with_stack(0xc, |e, collector| {
        let collector = collector.addr();
        e.mem.set_u32(collector, ROTATE_NORMAL_HIT_COLLECTOR_VTABLE);
        e.mem.set_u32(collector + 4, fraction);
        e.mem.set_u32(collector + 8, hits.addr());
        e.call(
            CAST_RAY_INTERNAL,
            &args![this, ray, extra, 0u32, 0u32, collector],
        );
    });
}

// Translated from 00d26120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSampledHeightFieldShape::castSphere` (map name), thiscall with three arguments
/// (`RET 0xC`): `ECX` is `this`. Copies the twelve words at `input` to the stack. When the
/// height scale at `this + 0x24` is positive the radius (`input + 0x30`) is negated. The
/// radius is added to the copy's second word and to its sixth (at `+0x14`), each rounded to a
/// `float` store, and the copy is cast through `castRayInternal` with `extra`, the flag 1, the
/// float at `input + 0x34` and `hits`.
pub fn hk_sampled_height_field_shape_cast_sphere(
    e: &mut Engine,
    this: Ptr,
    input: Ptr,
    extra: u32,
    hits: Ptr,
) {
    let mut radius = x87_quiet_bits(e.mem.u32(input.addr() + 0x30));
    if 0.0 < e.mem.f32(this.addr() + 0x24) {
        radius ^= 0x8000_0000;
    }
    let radius = f32::from_bits(radius);
    let fraction = x87_quiet_bits(e.mem.u32(input.addr() + 0x34));
    e.with_stack(0x30, |e, copy| {
        let copy = copy.addr();
        for word in 0..12 {
            let value = e.mem.u32(input.addr() + 4 * word);
            e.mem.set_u32(copy + 4 * word, value);
        }
        let second = (f64::from(radius) + f64::from(x87_quiet(e.mem.f32(copy + 4)))) as f32;
        e.mem.set_f32(copy + 4, second);
        let sixth = (f64::from(radius) + f64::from(e.mem.f32(copy + 0x14))) as f32;
        e.mem.set_f32(copy + 0x14, sixth);
        e.call(
            CAST_RAY_INTERNAL,
            &args![this, copy, extra, 1u32, fraction, hits],
        );
    });
}

// Translated from 00d261b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NearestHitCollector::addRayHit` (Xbox PDB, anonymous namespace), thiscall with two
/// arguments (`RET 8`): `ECX` is `this`, whose `+4` is the nearest fraction so far, `+8` the
/// found flag and `+0xC` the result record. When the fraction of the hit (`hit + 0x10`) is below
/// the nearest (a `NaN` is not), the nearest fraction, the first four words of the hit, the
/// fraction again (both as float stores) and the word at `hit + 0x14` go to the record, the
/// entry at `+0x20 + 4 * [record + 0x40]` of the record is set to -1, and the found flag is set.
pub fn nearest_hit_collector_add_ray_hit(e: &mut Engine, this: Ptr, _unused_1: u32, hit: Ptr) {
    let this = this.addr();
    let hit = hit.addr();
    let fraction = e.mem.f32(hit + 0x10);
    if fraction < e.mem.f32(this + 4) {
        e.mem.set_u32(this + 4, x87_quiet_bits(fraction.to_bits()));
        let record = e.mem.u32(this + 0xc);
        for word in 0..4 {
            let value = e.mem.u32(hit + 4 * word);
            e.mem.set_u32(record + 4 * word, value);
        }
        let record = e.mem.u32(this + 0xc);
        let quieted = x87_quiet_bits(e.mem.u32(hit + 0x10));
        e.mem.set_u32(record + 0x10, quieted);
        let record = e.mem.u32(this + 0xc);
        let extra = e.mem.u32(hit + 0x14);
        e.mem.set_u32(record + 0x14, extra);
        let record = e.mem.u32(this + 0xc);
        let index = e.mem.u32(record + 0x40);
        let entry = record
            .wrapping_add(index.wrapping_mul(4))
            .wrapping_add(0x20);
        e.mem.set_u32(entry, u32::MAX);
        e.mem.set_u8(this + 8, 1);
    }
}

// Translated from 00d26210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NearestHitCollector::scalar deleting destructor` (Xbox PDB, anonymous namespace), thiscall
/// with one argument (`RET 4`): `ECX` is `this`. Sets the vtable to the pure virtual one and,
/// when bit 0 of the flag byte is set, frees the block. Returns `this`.
pub fn nearest_hit_collector_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), PURE_VIRTUAL_VTABLE);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00d26310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RotateNormalHitCollector::addRayHit` (Xbox PDB, anonymous namespace), thiscall with two
/// arguments (`RET 8`): `ECX` is `this`, whose `+8` is the inner collector. Copies the eight
/// words at `hit` to the stack, then replaces the first four (the normal) with the normal
/// rotated by the three columns at `[context + 8]` (`+0`, `+0x10`, `+0x20`), per lane
/// `(y * column1 + x * column0) + z * column2` (single-precision SSE), and calls the first
/// virtual function of the inner collector with `(context, record)`.
pub fn rotate_normal_hit_collector_add_ray_hit(e: &mut Engine, this: Ptr, context: Ptr, hit: Ptr) {
    let normal = load_vec(e, hit.addr());
    let matrix = e.mem.u32(context.addr() + 8);
    let columns = [
        load_vec(e, matrix),
        load_vec(e, matrix + 0x10),
        load_vec(e, matrix + 0x20),
    ];
    let inner = e.mem.u32(this.addr() + 8);
    e.with_stack(0x20, |e, record| {
        let record = record.addr();
        for word in 0..8 {
            let value = e.mem.u32(hit.addr() + 4 * word);
            e.mem.set_u32(record + 4 * word, value);
        }
        let by_y = zip_with(shufps(normal, normal, 0x55), columns[1], |a, b| a * b);
        let by_x = zip_with(shufps(normal, normal, 0x00), columns[0], |a, b| a * b);
        let sum = zip_with(by_y, by_x, |a, b| a + b);
        let by_z = zip_with(shufps(normal, normal, 0xaa), columns[2], |a, b| a * b);
        let rotated = zip_with(sum, by_z, |a, b| a + b);
        store_vec(e, record, rotated);
        e.vcall(inner, 0, &args![context, record]);
    });
}

/// The x87 condition bits `FNSTSW` leaves after a compare: `c0` when the first operand is
/// below the second, `c3` when they are equal, all three when they are unordered.
#[derive(Clone, Copy)]
struct Compare {
    c0: bool,
    c2: bool,
    c3: bool,
}

impl Compare {
    fn new(a: f64, b: f64) -> Compare {
        if a.is_nan() || b.is_nan() {
            Compare {
                c0: true,
                c2: true,
                c3: true,
            }
        } else {
            Compare {
                c0: a < b,
                c2: false,
                c3: a == b,
            }
        }
    }

    /// `AH` after `FNSTSW AX`: `C0` is bit 0, `C2` bit 2 and `C3` bit 6.
    fn ah(self) -> u8 {
        u8::from(self.c0) | (u8::from(self.c2) << 2) | (u8::from(self.c3) << 6)
    }

    /// `TEST AH, mask` and the result's parity flag (`JP` jumps when it is even).
    fn parity(self, mask: u8) -> bool {
        (self.ah() & mask).count_ones() % 2 == 0
    }

    /// `TEST AH, mask` and the zero flag.
    fn zero(self, mask: u8) -> bool {
        self.ah() & mask == 0
    }
}

/// The x87 register stack (the last element is `ST0`). Values live in double precision; a
/// `float` load widens (a signalling NaN is quieted) and a `float` store rounds.
struct X87 {
    stack: Vec<f64>,
}

impl X87 {
    fn top(&self, depth: usize) -> usize {
        self.stack.len() - 1 - depth
    }

    fn pop(&mut self) -> f64 {
        self.stack.pop().expect("x87 stack")
    }

    fn st(&self, depth: usize) -> f64 {
        self.stack[self.top(depth)]
    }

    fn set(&mut self, depth: usize, value: f64) {
        let index = self.top(depth);
        self.stack[index] = value;
    }

    fn push(&mut self, value: f64) {
        assert!(self.stack.len() < 8, "x87 stack overflow");
        self.stack.push(value);
    }

    /// `FLD ST(depth)`.
    fn fld_st(&mut self, depth: usize) {
        self.push(self.st(depth));
    }

    fn fld1(&mut self) {
        self.push(1.0);
    }

    fn fldz(&mut self) {
        self.push(0.0);
    }

    /// `FXCH ST(depth)`.
    fn fxch(&mut self, depth: usize) {
        let (a, b) = (self.top(0), self.top(depth));
        self.stack.swap(a, b);
    }

    fn fchs(&mut self) {
        self.set(0, -self.st(0));
    }

    fn fabs(&mut self) {
        self.set(0, self.st(0).abs());
    }

    /// `FADD ST0, ST(depth)`.
    fn fadd0(&mut self, depth: usize) {
        self.set(0, self.st(0) + self.st(depth));
    }

    /// `FSUB ST0, ST(depth)`.
    fn fsub0(&mut self, depth: usize) {
        self.set(0, self.st(0) - self.st(depth));
    }

    /// `FSUBR ST0, ST(depth)`: `ST0 = ST(depth) - ST0`.
    fn fsubr0(&mut self, depth: usize) {
        self.set(0, self.st(depth) - self.st(0));
    }

    /// `FMUL ST0, ST(depth)`.
    fn fmul0(&mut self, depth: usize) {
        self.set(0, self.st(0) * self.st(depth));
    }

    /// `FDIVR ST0, ST(depth)`: `ST0 = ST(depth) / ST0`.
    fn fdivr0(&mut self, depth: usize) {
        self.set(0, self.st(depth) / self.st(0));
    }

    /// `FADDP ST(depth), ST0`.
    fn faddp(&mut self, depth: usize) {
        let value = self.st(0);
        self.set(depth, self.st(depth) + value);
        self.pop();
    }

    /// `FSUBP ST(depth), ST0`: `ST(depth) = ST(depth) - ST0`.
    fn fsubp(&mut self, depth: usize) {
        let value = self.st(0);
        self.set(depth, self.st(depth) - value);
        self.pop();
    }

    /// `FSUBRP ST(depth), ST0`: `ST(depth) = ST0 - ST(depth)`.
    fn fsubrp(&mut self, depth: usize) {
        let value = self.st(0);
        self.set(depth, value - self.st(depth));
        self.pop();
    }

    /// `FMULP ST(depth), ST0`.
    fn fmulp(&mut self, depth: usize) {
        let value = self.st(0);
        self.set(depth, self.st(depth) * value);
        self.pop();
    }

    /// `FDIVP ST(depth), ST0`: `ST(depth) = ST(depth) / ST0`.
    fn fdivp(&mut self, depth: usize) {
        let value = self.st(0);
        self.set(depth, self.st(depth) / value);
        self.pop();
    }

    /// `FDIVRP ST(depth), ST0`: `ST(depth) = ST0 / ST(depth)`.
    fn fdivrp(&mut self, depth: usize) {
        let value = self.st(0);
        self.set(depth, value / self.st(depth));
        self.pop();
    }

    /// `FSTP ST(depth)`: `ST(depth) = ST0`, then pop.
    fn fstp_st(&mut self, depth: usize) {
        let value = self.st(0);
        self.set(depth, value);
        self.pop();
    }

    /// `FCOM ST(depth)` and `FUCOM ST(depth)`.
    fn fcom(&self, depth: usize) -> Compare {
        Compare::new(self.st(0), self.st(depth))
    }

    /// `FCOMP ST(depth)` and `FUCOMP ST(depth)`.
    fn fcomp(&mut self, depth: usize) -> Compare {
        let result = self.fcom(depth);
        self.pop();
        result
    }

    /// `FCOMPP`.
    fn fcompp(&mut self) -> Compare {
        let result = self.fcomp(1);
        self.pop();
        result
    }
}

/// `FISTP dword`: the value rounded to nearest (even); an out-of-range value or a NaN gives the
/// integer indefinite `0x80000000`.
fn round_to_int(value: f64) -> i32 {
    crate::x87::fistp(value)
}

/// The registers, the stack and the x87 stack of `castRayInternal`, and its arguments.
struct Walk<'a> {
    e: &'a mut Engine,
    x: X87,
    /// The stack pointer; the frame's low end `L` is where it stands after the prologue.
    sp: u32,
    eax: u32,
    ecx: u32,
    edx: u32,
    ebx: u32,
    esi: u32,
    edi: u32,
    xmm: [Vec4; 5],
    /// The byte argument (`[EBP + 0x10]`; the function clears it after a hit is reported).
    flag: u8,
    ray: u32,
    extra: u32,
    /// The fraction argument (`[EBP + 0x14]`).
    fraction: f32,
    collector: u32,
}

impl Walk<'_> {
    fn at(&self, offset: u32) -> u32 {
        self.sp.wrapping_add(offset)
    }

    fn word(&self, offset: u32) -> u32 {
        self.e.mem.u32(self.at(offset))
    }

    fn set_word(&mut self, offset: u32, value: u32) {
        let address = self.at(offset);
        self.e.mem.set_u32(address, value);
    }

    fn push(&mut self, value: u32) {
        self.sp = self.sp.wrapping_sub(4);
        let address = self.sp;
        self.e.mem.set_u32(address, value);
    }

    /// `FLD dword ptr [ESP + offset]`.
    fn fld(&mut self, offset: u32) {
        let value = x87_quiet(self.e.mem.f32(self.at(offset)));
        self.x.push(f64::from(value));
    }

    /// `FLD dword ptr [address]`.
    fn fld_at(&mut self, address: u32) {
        let value = x87_quiet(self.e.mem.f32(address));
        self.x.push(f64::from(value));
    }

    /// `FILD dword ptr [ESP + offset]`.
    fn fild(&mut self, offset: u32) {
        let value = self.e.mem.i32(self.at(offset));
        self.x.push(f64::from(value));
    }

    /// `FSTP dword ptr [ESP + offset]`.
    fn fstp(&mut self, offset: u32) {
        let value = self.x.pop();
        let address = self.at(offset);
        self.e.mem.set_f32(address, value as f32);
    }

    /// `FST dword ptr [ESP + offset]`.
    fn fst(&mut self, offset: u32) {
        let value = self.x.st(0);
        let address = self.at(offset);
        self.e.mem.set_f32(address, value as f32);
    }

    /// `FADD`, `FSUB`, `FSUBR` and `FMUL` with a `float` in the stack frame.
    fn fadd(&mut self, offset: u32) {
        let value = f64::from(x87_quiet(self.e.mem.f32(self.at(offset))));
        self.x.set(0, self.x.st(0) + value);
    }

    fn fsub(&mut self, offset: u32) {
        let value = f64::from(x87_quiet(self.e.mem.f32(self.at(offset))));
        self.x.set(0, self.x.st(0) - value);
    }

    fn fmul(&mut self, offset: u32) {
        let value = f64::from(x87_quiet(self.e.mem.f32(self.at(offset))));
        self.x.set(0, self.x.st(0) * value);
    }

    /// `FISUB dword ptr [ESP + offset]`.
    fn fisub(&mut self, offset: u32) {
        let value = f64::from(self.e.mem.i32(self.at(offset)));
        self.x.set(0, self.x.st(0) - value);
    }

    /// `FIMUL dword ptr [ESP + offset]`.
    fn fimul(&mut self, offset: u32) {
        let value = f64::from(self.e.mem.i32(self.at(offset)));
        self.x.set(0, self.x.st(0) * value);
    }

    /// `FISTP dword ptr [ESP + offset]`.
    fn fistp(&mut self, offset: u32) {
        let value = self.x.pop();
        let address = self.at(offset);
        self.e.mem.set_i32(address, round_to_int(value));
    }

    /// `FCOM dword ptr [ESP + offset]` (and `FUCOM`).
    fn fcom_mem(&mut self, offset: u32) -> Compare {
        let value = f64::from(x87_quiet(self.e.mem.f32(self.at(offset))));
        Compare::new(self.x.st(0), value)
    }

    /// `FCOMP dword ptr [ESP + offset]`.
    fn fcomp_mem(&mut self, offset: u32) -> Compare {
        let result = self.fcom_mem(offset);
        self.x.pop();
        result
    }

    /// `FCOMP dword ptr [address]`.
    fn fcomp_at(&mut self, address: u32) -> Compare {
        let value = f64::from(x87_quiet(self.e.mem.f32(address)));
        let result = Compare::new(self.x.st(0), value);
        self.x.pop();
        result
    }

    /// `MOVSS xmm, dword ptr [ESP + offset]`: the word in the first lane, the others cleared.
    fn movss(&self, offset: u32) -> Vec4 {
        [f32::from_bits(self.word(offset)), 0.0, 0.0, 0.0]
    }

    /// `CALL target` with `this` in `ECX` and the `args` words on top of the stack (the callee
    /// pops them); `EAX` gets the answer.
    fn call_this(&mut self, target: u32, this: u32, args: usize) -> Ret {
        let mut words = vec![this];
        for index in 0..args {
            words.push(self.e.mem.u32(self.sp + 4 * index as u32));
        }
        let result = self.e.call(target, &words);
        self.sp = self.sp.wrapping_add(4 * args as u32);
        self.eax = result.eax;
        result
    }

    /// The call of the shape's height slot (`0x34`): `x` and `z` were pushed; the height is
    /// left in `ST0`.
    fn height(&mut self) {
        let vtable = self.e.mem.u32(self.esi);
        let target = self.e.mem.u32(vtable + HEIGHT_SLOT);
        let result = self.call_this(target, self.esi, 2);
        self.x.push(result.st0);
    }
}

/// The profile label of the cast (`"TtrcHeightFild"`, `010ce35c`), the vector added to the
/// scaled ray ends (`011b1bd0`, four times 196608.0: the magic number that leaves the integer
/// cell in the mantissa), the smallest direction component the walk steps along
/// (`010ce358`, 2^-15), the largest float (`0104da48`) and its negative (`0104e148`), the unit
/// vector (`010ec7f0`), `-1.0` (`01012054`), `1.0` as a float (`01012068`) and as a double
/// (`01012070`), and the helper `00c98210` (cdecl, two floats, answers `ST0`) the hit fraction
/// goes through when the ray starts below the surface.
const CAST_RAY_LABEL: u32 = 0x010c_e35c;
const CELL_MAGIC: u32 = 0x011b_1bd0;
const DIRECTION_LIMIT: u32 = 0x010c_e358;
const BIG_FLOAT: u32 = 0x0104_da48;
const NEGATIVE_BIG_FLOAT: u32 = 0x0104_e148;
const UNIT_VECTOR: u32 = 0x010e_c7f0;
const NEGATIVE_ONE_FLOAT: u32 = 0x0101_2054;
const ONE_FLOAT: u32 = 0x0101_2068;
const ONE_DOUBLE: u32 = 0x0101_2070;
const FRACTION_HELPER: u32 = 0x00c9_8210;
/// The shape's virtual slot that writes the byte telling which way a cell is split (it is
/// handed the address of the byte).
const DIAGONAL_SLOT: u32 = 0x38;

fn addps(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

fn subps(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

fn mulps(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

fn sign_extend_16(value: u32) -> u32 {
    i32::from(value as u16 as i16) as u32
}

impl Walk<'_> {
    fn flag_byte(&self, offset: u32) -> u8 {
        self.e.mem.u8(self.at(offset))
    }

    fn vec_at(&self, offset: u32) -> Vec4 {
        load_vec(self.e, self.at(offset))
    }

    fn put_vec(&mut self, offset: u32, value: Vec4) {
        let address = self.at(offset);
        store_vec(self.e, address, value);
    }

    /// `FSUBR dword ptr [ESP + offset]`: `ST0 = m32 - ST0`.
    fn fsubr(&mut self, offset: u32) {
        let value = f64::from(x87_quiet(self.e.mem.f32(self.at(offset))));
        self.x.set(0, value - self.x.st(0));
    }

    /// `FADD dword ptr [address]`.
    fn fadd_at(&mut self, address: u32) {
        let value = f64::from(x87_quiet(self.e.mem.f32(address)));
        self.x.set(0, self.x.st(0) + value);
    }

    /// `FSTP dword ptr [address]`.
    fn fstp_at(&mut self, address: u32) {
        let value = self.x.pop();
        self.e.mem.set_f32(address, value as f32);
    }

    /// The end of the function: the profile record of the cast's end; the x87 stack is empty.
    fn leave(&mut self) {
        debug_assert!(self.x.stack.is_empty(), "x87 stack not empty at the return");
        profile_record(self.e, PROFILE_END);
    }

    /// `FCOMP qword ptr [address]`.
    fn fcomp_double(&mut self, address: u32) -> Compare {
        let value = self.e.mem.f64(address);
        let result = Compare::new(self.x.st(0), value);
        self.x.pop();
        result
    }

    /// `FSUB qword ptr [address]`.
    fn fsub_double(&mut self, address: u32) {
        let value = self.e.mem.f64(address);
        self.x.set(0, self.x.st(0) - value);
    }

    /// The vector scaled to unit length the way the exe does it (`RSQRTSS` refined once; the
    /// vector stays zero when its first three lanes sum to zero).
    fn scaled_normal(&mut self, vector: Vec4) -> Vec4 {
        let squares = mulps(vector, vector);
        let sum = sum3(squares);
        let inverse = inverse_length(self.e, sum);
        mulps([inverse; 4], vector)
    }

    /// The call of the collector's `addRayHit` (slot 0, `RET 8`): the shape word the caller
    /// passed and a pointer to the hit record.
    fn report_hit(&mut self, hit: u32) {
        let vtable = self.e.mem.u32(self.collector);
        let target = self.e.mem.u32(vtable);
        let words = [self.collector, self.extra, hit];
        self.e.call(target, &words);
    }

    fn run(&mut self) {
        let mut pc: u32 = 0x00d2_4b90;
        loop {
            match pc {
                // The ray in cell coordinates and the shape's slot 0x38 (a byte at 0x13).
                0x00d2_4b90 => {
                    self.edx = self.ray;
                    self.xmm[1] = load_vec(self.e, self.edx);
                    self.xmm[0] = load_vec(self.e, self.esi + 0x30);
                    self.xmm[2] = load_vec(self.e, self.esi + 0x40);
                    self.xmm[3] = load_vec(self.e, CELL_MAGIC);
                    self.xmm[2] = addps(self.xmm[2], self.xmm[1]);
                    self.xmm[2] = mulps(self.xmm[2], self.xmm[0]);
                    self.xmm[2] = addps(self.xmm[2], self.xmm[3]);
                    self.put_vec(0x110, self.xmm[2]);
                    self.ecx = self.word(0x118) >> 6;
                    self.eax = self.word(0x110) >> 6;
                    self.ebx = sign_extend_16(self.ecx);
                    self.edi = sign_extend_16(self.eax);
                    self.eax = self.e.mem.u32(self.esi);
                    self.xmm[2] = mulps(self.xmm[0], self.xmm[1]);
                    self.xmm[1] = load_vec(self.e, self.edx + 0x10);
                    self.edx = self.e.mem.u32(self.eax + DIAGONAL_SLOT);
                    self.ecx = self.at(0x13);
                    self.push(self.ecx);
                    self.xmm[1] = mulps(self.xmm[1], self.xmm[0]);
                    self.ecx = self.esi;
                    self.set_word(0x38, self.edi);
                    self.set_word(0x40, self.ebx);
                    self.put_vec(0x94, self.xmm[2]);
                    self.put_vec(0xb4, self.xmm[1]);
                    self.call_this(self.edx, self.ecx, 1);
                    self.fld(0x94);
                    let byte = self.flag_byte(0x13);
                    self.fstp(0x9c);
                    self.fld(0xb4);
                    self.fstp(0xbc);
                    if byte == 0 {
                        self.fld(0x98);
                        self.x.fld_st(0);
                        self.fld(0x90);
                        self.x.fld_st(0);
                        self.x.faddp(2);
                        self.x.fxch(1);
                        self.fstp(0x94);
                        self.fld(0xb8);
                        self.fld(0xb0);
                        self.x.fld_st(0);
                        self.x.faddp(2);
                        self.x.fxch(1);
                        self.fstp(0xb4);
                    } else {
                        self.fld(0x90);
                        self.x.fld_st(0);
                        self.fld(0x98);
                        self.x.fld_st(0);
                        self.x.fsubp(2);
                        self.x.fxch(1);
                        self.fstp(0x94);
                        self.fld(0xb0);
                        self.x.fld_st(0);
                        self.fsub(0xb8);
                        self.fstp(0xb4);
                        self.x.fxch(1);
                        self.x.fxch(2);
                        self.x.fxch(1);
                    }
                    // The first axis: the step, its inverse and the first crossing.
                    self.x.fsub0(1);
                    self.edx = 1;
                    self.fstp(0x28);
                    self.fld(0x28);
                    self.x.fld_st(0);
                    self.x.fabs();
                    self.fstp(0x14);
                    self.fld(0x14);
                    self.fstp(0x28);
                    self.fld(0x28);
                    self.fst(0x120);
                    self.fld_at(DIRECTION_LIMIT);
                    let c = self.x.fcom(1);
                    self.x.fstp_st(1);
                    self.x.fld1();
                    if c.zero(0x41) {
                        self.x.fstp_st(2);
                        self.ecx = u32::MAX;
                        self.x.fldz();
                        self.set_word(0x48, self.ecx);
                        self.fst(0x70);
                        self.fld_at(BIG_FLOAT);
                        self.fstp(0x54);
                    } else {
                        self.x.fld_st(2);
                        self.ecx = u32::MAX;
                        self.x.fdivr0(1);
                        self.fstp(0x20);
                        self.fld(0x20);
                        self.fst(0x110);
                        self.x.fldz();
                        let c = self.x.fcom(4);
                        self.x.fstp_st(4);
                        if !c.zero(0x41) {
                            self.edi = self.edi.wrapping_add(self.edx);
                            self.fst(0x70);
                            self.set_word(0x48, self.edx);
                            self.set_word(0x34, self.edi);
                        } else {
                            self.x.fld_st(0);
                            self.set_word(0x48, self.ecx);
                            self.x.fchs();
                            self.fstp(0x70);
                        }
                        self.fild(0x34);
                        self.x.fsub0(5);
                        self.x.fmulp(1);
                        self.fstp(0x54);
                        self.x.fxch(2);
                    }
                    // The second axis (the diagonal, whose cell index is not known yet: the
                    // word at 0x38 is read before the exe has written it, and the values
                    // computed from it are overwritten before they are used).
                    self.fld(0xb4);
                    self.fsub(0x94);
                    self.fstp(0x28);
                    self.fld(0x28);
                    self.x.fld_st(0);
                    self.x.fabs();
                    self.fstp(0x14);
                    self.fld(0x14);
                    self.fstp(0x28);
                    self.x.fxch(2);
                    let c = self.fcom_mem(0x28);
                    if c.zero(0x41) {
                        self.x.fstp_st(2);
                        self.set_word(0x4c, self.ecx);
                        self.fst(0x74);
                        self.fld_at(BIG_FLOAT);
                    } else {
                        self.x.fld_st(2);
                        self.x.fdivr0(4);
                        self.fstp(0x20);
                        self.x.fxch(2);
                        let c = self.x.fcomp(1);
                        self.fld(0x20);
                        if c.parity(5) {
                            let index = self.word(0x38).wrapping_add(self.edx);
                            self.set_word(0x38, index);
                            self.fst(0x74);
                            self.set_word(0x4c, self.edx);
                        } else {
                            self.x.fld_st(0);
                            self.set_word(0x4c, self.ecx);
                            self.x.fchs();
                            self.fstp(0x74);
                        }
                        self.fild(0x38);
                        self.fsub(0x94);
                        self.x.fmulp(1);
                    }
                    self.fstp(0x58);
                    // The third axis.
                    self.fld(0xb8);
                    self.x.fsub0(5);
                    self.fstp(0x28);
                    self.fld(0x28);
                    self.x.fld_st(0);
                    self.x.fabs();
                    self.fstp(0x14);
                    self.fld(0x14);
                    self.fstp(0x28);
                    self.x.fxch(2);
                    let c = self.fcomp_mem(0x28);
                    if c.zero(0x41) {
                        self.x.fstp_st(1);
                        self.edx = self.ecx;
                        self.x.fstp_st(1);
                        self.fst(0x78);
                        self.fld_at(BIG_FLOAT);
                    } else {
                        self.x.fld_st(1);
                        self.x.fdivp(3);
                        self.x.fxch(2);
                        self.fstp(0x20);
                        self.fld(0x20);
                        self.fst(0x118);
                        self.x.fxch(1);
                        let c = self.x.fcomp(2);
                        if c.parity(5) {
                            self.ebx = self.ebx.wrapping_add(1);
                            self.fst(0x78);
                            self.set_word(0x3c, self.ebx);
                        } else {
                            self.x.fld_st(0);
                            self.edx = self.ecx;
                            self.x.fchs();
                            self.fstp(0x78);
                        }
                        self.fild(0x3c);
                        self.x.fsub0(4);
                        self.x.fmulp(1);
                    }
                    self.fstp(0x5c);
                    self.set_word(0x50, self.edx);
                    // Two of the three inverse steps adding up to 0 (the ray runs along a
                    // diagonal of the grid) is the special case.
                    self.fld(0x78);
                    self.fld(0x70);
                    self.x.fld_st(0);
                    self.x.faddp(2);
                    self.x.fldz();
                    let c = self.x.fcom(2);
                    self.x.fstp_st(2);
                    if !c.parity(0x44) {
                        pc = 0x00d2_5c3f;
                        continue;
                    }
                    self.fld(0x74);
                    self.x.fld_st(0);
                    self.x.fadd0(2);
                    let c = self.x.fcomp(3);
                    if !c.parity(0x44) {
                        pc = 0x00d2_5c3d;
                        continue;
                    }
                    self.x.fld_st(0);
                    self.fadd(0x78);
                    let c = self.x.fcomp(3);
                    self.x.fstp_st(2);
                    if !c.parity(0x44) {
                        pc = 0x00d2_5c3f;
                        continue;
                    }
                    // The general case: the walk starts where the ray enters the grid.
                    let step = self.word(0x48) as i32;
                    self.eax = self.word(0x90);
                    self.ecx = self.word(0x94);
                    self.xmm[1] = self.vec_at(0x90);
                    self.xmm[2] = self.vec_at(0xb0);
                    self.set_word(0x60, self.eax);
                    self.eax = self.word(0x98);
                    self.set_word(0x64, self.ecx);
                    self.ecx = self.word(0x9c);
                    self.set_word(0x68, self.eax);
                    self.set_word(0x6c, self.ecx);
                    self.set_word(0x44, self.edi);
                    if step <= 0 {
                        pc = 0x00d2_4f3d;
                        continue;
                    }
                    if (self.edi as i32) > 0 {
                        pc = 0x00d2_50f2;
                        continue;
                    }
                    self.x.fstp_st(1);
                    self.x.fstp_st(3);
                    let c = self.fcomp_mem(0xb0);
                    if !c.zero(0x41) {
                        self.x.fchs();
                        self.edi = 1;
                        pc = 0x00d2_4faa;
                        continue;
                    }
                    self.x.fstp_st(1);
                    self.x.pop();
                    self.leave();
                    return;
                }
                // The first axis' index is above the grid (it counts down).
                0x00d2_4f3d => {
                    self.eax = self.e.mem.u32(self.esi + 0x10);
                    self.ecx = self.eax.wrapping_sub(2);
                    if (self.edi as i32) <= (self.ecx as i32) {
                        pc = 0x00d2_50f2;
                        continue;
                    }
                    self.x.fstp_st(1);
                    self.eax = self.eax.wrapping_sub(1);
                    self.x.fstp_st(1);
                    self.set_word(0x14, self.eax);
                    self.x.fstp_st(2);
                    self.fild(0x14);
                    self.fstp(0x14);
                    self.fld(0xb0);
                    self.fld(0x14);
                    let c = self.x.fcom(1);
                    self.x.fstp_st(1);
                    if !c.parity(5) {
                        self.x.pop();
                        self.x.fstp_st(1);
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    self.x.fsubp(1);
                    self.edi = self.ecx;
                    pc = 0x00d2_4faa;
                }
                // Moves the walk's start to the grid's edge along the first axis.
                0x00d2_4faa => {
                    self.xmm[3] = load_vec(self.e, UNIT_VECTOR);
                    self.x.fmul0(1);
                    self.xmm[4] = self.xmm[2];
                    self.set_word(0x34, self.edi);
                    self.fstp(0x24);
                    self.xmm[0] = self.movss(0x24);
                    self.fld(0x24);
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0);
                    self.x.fld_st(0);
                    self.xmm[3] = subps(self.xmm[3], self.xmm[0]);
                    self.x.faddp(2);
                    self.xmm[3] = mulps(self.xmm[3], self.xmm[1]);
                    self.x.fxch(1);
                    self.xmm[4] = mulps(self.xmm[4], self.xmm[0]);
                    self.xmm[3] = addps(self.xmm[3], self.xmm[4]);
                    self.fstp(0x54);
                    self.put_vec(0x60, self.xmm[3]);
                    self.fmul(0x28);
                    self.fstp(0x24);
                    self.fld(0x24);
                    self.fistp(0x14);
                    self.fld(0x68);
                    self.eax = self.edx;
                    self.eax = self.eax.wrapping_mul(self.word(0x14));
                    self.ebx = self.ebx.wrapping_add(self.eax).wrapping_sub(2);
                    self.set_word(0x3c, self.ebx);
                    self.fild(0x3c);
                    let mut c = self.x.fcomp(1);
                    while !c.parity(0x41) {
                        self.ebx = self.ebx.wrapping_add(1);
                        self.set_word(0x3c, self.ebx);
                        self.fild(0x3c);
                        c = self.x.fcomp(1);
                    }
                    self.x.pop();
                    self.ecx = self.edx;
                    self.x.fldz();
                    self.ecx = ((self.ecx as i32) >> 1) as u32;
                    self.x.fld_st(0);
                    self.ebx = self.ebx.wrapping_add(self.ecx);
                    self.fld(0x78);
                    self.set_word(0x3c, self.ebx);
                    let c = self.x.fcom(1);
                    self.x.fstp_st(1);
                    if c.parity(0x44) {
                        self.fild(0x3c);
                        self.fld(0x98);
                        self.x.fld_st(0);
                        self.x.fsubp(2);
                        self.fld(0x118);
                        self.x.fmulp(2);
                        self.x.fxch(1);
                        self.fstp(0x5c);
                        self.fld(0x74);
                        self.fld(0x70);
                        self.x.fxch(2);
                        self.x.fxch(4);
                        self.x.fxch(2);
                        self.x.fxch(3);
                    } else {
                        self.fld(0x74);
                        self.fld(0x70);
                        self.fld(0x98);
                        self.x.fxch(4);
                        self.x.fxch(1);
                        self.x.fxch(3);
                        self.x.fxch(1);
                        self.x.fxch(2);
                        self.x.fxch(1);
                    }
                    pc = 0x00d2_5093;
                }
                0x00d2_50f2 => {
                    self.x.fstp_st(3);
                    self.fld(0x78);
                    pc = 0x00d2_5093;
                }
                0x00d2_5093 => {
                    if (self.edx as i32) <= 0 {
                        pc = 0x00d2_5107;
                        continue;
                    }
                    if (self.ebx as i32) > 0 {
                        pc = 0x00d2_527c;
                        continue;
                    }
                    self.x.fstp_st(1);
                    self.x.fstp_st(2);
                    let c = self.fcomp_mem(0xb8);
                    if !c.zero(0x41) {
                        self.x.fxch(1);
                        self.ebx = 1;
                        self.x.fchs();
                        self.x.fmul0(1);
                        pc = 0x00d2_517a;
                        continue;
                    }
                    self.x.fstp_st(1);
                    self.x.pop();
                    self.leave();
                    return;
                }
                0x00d2_5107 => {
                    self.eax = self.e.mem.u32(self.esi + 0x14);
                    self.ecx = self.eax.wrapping_sub(2);
                    if (self.ebx as i32) <= (self.ecx as i32) {
                        pc = 0x00d2_527c;
                        continue;
                    }
                    self.x.fstp_st(1);
                    self.eax = self.eax.wrapping_sub(1);
                    self.x.fstp_st(2);
                    self.set_word(0x14, self.eax);
                    self.x.pop();
                    self.fild(0x14);
                    self.fstp(0x14);
                    self.fld(0xb8);
                    self.fld(0x14);
                    let c = self.x.fcom(1);
                    self.x.fstp_st(1);
                    if !c.parity(5) {
                        self.x.pop();
                        self.x.fstp_st(1);
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    self.x.fsubp(2);
                    self.ebx = self.ecx;
                    self.x.fld_st(0);
                    self.x.fmulp(2);
                    self.x.fxch(1);
                    pc = 0x00d2_517a;
                }
                // Moves the walk's start to the grid's edge along the third axis.
                0x00d2_517a => {
                    self.xmm[3] = load_vec(self.e, UNIT_VECTOR);
                    self.fstp(0x24);
                    self.fld(0x24);
                    self.xmm[0] = self.movss(0x24);
                    self.x.fld_st(0);
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0);
                    self.x.faddp(2);
                    self.xmm[3] = subps(self.xmm[3], self.xmm[0]);
                    self.x.fxch(1);
                    self.xmm[4] = self.xmm[2];
                    self.xmm[3] = mulps(self.xmm[3], self.xmm[1]);
                    self.fstp(0x5c);
                    self.xmm[4] = mulps(self.xmm[4], self.xmm[0]);
                    self.xmm[3] = addps(self.xmm[3], self.xmm[4]);
                    self.fmul(0x120);
                    self.set_word(0x3c, self.ebx);
                    self.put_vec(0x60, self.xmm[3]);
                    self.fstp(0x24);
                    self.fld(0x24);
                    self.fistp(0x14);
                    self.ecx = self.word(0x48);
                    self.fld(0x60);
                    self.edi = self.word(0x44);
                    self.eax = self.ecx;
                    self.eax = self.eax.wrapping_mul(self.word(0x14));
                    self.edi = self.eax.wrapping_add(self.edi).wrapping_sub(2);
                    self.set_word(0x34, self.edi);
                    self.fild(0x34);
                    let mut c = self.x.fcomp(1);
                    while !c.parity(0x41) {
                        self.edi = self.edi.wrapping_add(1);
                        self.set_word(0x34, self.edi);
                        self.fild(0x34);
                        c = self.x.fcomp(1);
                    }
                    self.x.pop();
                    self.ecx = ((self.ecx as i32) >> 1) as u32;
                    self.x.fldz();
                    self.edi = self.edi.wrapping_add(self.ecx);
                    self.x.fld_st(0);
                    self.set_word(0x34, self.edi);
                    self.fld(0x70);
                    let c = self.x.fcom(1);
                    self.x.fstp_st(1);
                    if c.parity(0x44) {
                        self.fild(0x34);
                        self.fsub(0x90);
                        self.fmul(0x110);
                        self.fstp(0x54);
                    }
                    self.fld(0x74);
                    self.fld(0x78);
                    self.x.fxch(3);
                    self.x.fxch(1);
                    pc = 0x00d2_5241;
                }
                0x00d2_527c => {
                    self.x.fstp_st(4);
                    pc = 0x00d2_5241;
                }
                // The walk's start must lie inside the grid; the cells of the other axes
                // and the first step's axis are chosen.
                0x00d2_5241 => {
                    self.eax = self.e.mem.u32(self.esi + 0x10);
                    if self.edi >= self.eax {
                        self.x.pop();
                        self.x.fstp_st(1);
                        self.x.pop();
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    self.ecx = self.edi.wrapping_sub(self.word(0x48));
                    if self.ecx >= self.eax {
                        self.x.pop();
                        self.x.fstp_st(1);
                        self.x.pop();
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    self.edx ^= self.word(0x48);
                    let mut selector = u8::from((self.edx as i32) >= 0);
                    self.x.fld_st(1);
                    let c = self.x.fcomp(1);
                    let walks_down = u8::from(self.flag_byte(0x13) != 0);
                    selector ^= walks_down;
                    let address = self.at(0x1f);
                    self.e.mem.set_u8(address, selector);
                    if !c.parity(0x44) {
                        self.e.mem.set_u8(address, 0);
                        selector = 0;
                    }
                    self.edx = (self.edx & !0xff) | u32::from(selector);
                    let walks_down = self.flag_byte(0x13) != 0;
                    self.ecx = 2;
                    self.set_word(0x24, self.ecx);
                    if walks_down {
                        self.edi = self.edi.wrapping_sub(self.ebx);
                    } else {
                        self.edi = self.edi.wrapping_add(self.ebx);
                    }
                    self.fld(0x64);
                    self.set_word(0x38, self.edi);
                    self.fisub(0x38);
                    self.fstp(0x14);
                    if selector != 0 {
                        self.fld(0x14);
                        self.x.fabs();
                        self.fstp(0x14);
                        self.fld(0x14);
                        let c = self.fcomp_at(ONE_FLOAT);
                        if c.zero(0x41) {
                            self.edi = self.edi.wrapping_sub(self.word(0x4c));
                            self.set_word(0x38, self.edi);
                        } else {
                            self.ecx = 0;
                            self.set_word(0x24, self.ecx);
                        }
                    } else {
                        self.fild(0x4c);
                        self.fmul(0x14);
                        self.fstp(0x14);
                        self.fld(0x14);
                        let c = self.x.fcomp(2);
                        if c.zero(0x41) {
                            self.ecx = 0;
                            self.edi = self.edi.wrapping_add(self.word(0x4c));
                            self.set_word(0x24, self.ecx);
                            self.set_word(0x38, self.edi);
                        }
                    }
                    self.x.fld_st(1);
                    let c = self.x.fcomp(1);
                    if !c.parity(0x44) {
                        self.fld_at(BIG_FLOAT);
                    } else {
                        self.fild(0x38);
                        self.fsub(0x94);
                        self.x.fmul0(1);
                        self.fimul(0x4c);
                    }
                    self.fstp(0x58);
                    self.x.fld_st(1);
                    let c = self.x.fcomp(3);
                    self.fld_at(NEGATIVE_BIG_FLOAT);
                    if !c.parity(0x44) {
                        self.x.fstp_st(3);
                        self.x.fxch(2);
                        self.fst(0x60);
                    } else {
                        self.fld(0x54);
                        self.x.fsubrp(4);
                        self.x.fxch(3);
                        self.fstp(0x60);
                        self.x.fxch(2);
                    }
                    self.x.fld_st(1);
                    let c = self.x.fcomp(3);
                    if !c.parity(0x44) {
                        self.x.fstp_st(2);
                        self.x.fxch(1);
                        self.fst(0x64);
                    } else {
                        self.fld(0x58);
                        self.x.fsubrp(3);
                        self.x.fxch(2);
                        self.fstp(0x64);
                        self.x.fxch(1);
                    }
                    self.x.fld_st(1);
                    let c = self.x.fcomp(3);
                    if !c.parity(0x44) {
                        self.x.fstp_st(2);
                    } else {
                        self.x.pop();
                        self.fld(0x5c);
                        self.x.fsubrp(2);
                    }
                    self.x.fxch(1);
                    self.fstp(0x68);
                    // The axis of the first step.
                    let axis: u32 = if selector != 0 {
                        if self.ecx == 0 {
                            1
                        } else {
                            self.fld(0x60);
                            self.fld(0x68);
                            let c = self.x.fcompp();
                            if c.parity(5) {
                                2
                            } else {
                                0
                            }
                        }
                    } else {
                        self.fld(0x120);
                        self.fld(0x28);
                        let c = self.x.fcompp();
                        if c.zero(0x41) {
                            if self.ecx != 0 {
                                2
                            } else {
                                self.fld(0x60);
                                self.fld(0x64);
                                let c = self.x.fcompp();
                                if c.parity(5) {
                                    1
                                } else {
                                    0
                                }
                            }
                        } else if self.ecx != 0 {
                            0
                        } else {
                            self.fld(0x64);
                            self.fld(0x68);
                            let c = self.x.fcompp();
                            if !c.parity(5) {
                                1
                            } else {
                                2
                            }
                        }
                    };
                    self.eax = axis;
                    self.fld(0x54 + 4 * axis);
                    self.edx = self.word(0x48 + 4 * axis);
                    self.fsub(0x70 + 4 * axis);
                    let cell = self.word(0x34 + 4 * axis).wrapping_sub(self.edx);
                    self.set_word(0x34 + 4 * axis, cell);
                    self.set_word(0x20, axis);
                    self.fstp(0x54 + 4 * axis);
                    self.eax = self.e.mem.u32(self.esi + 0x10);
                    self.fld_at(BIG_FLOAT);
                    self.fstp(0x44);
                    self.fstp(0x88);
                    self.fld_at(NEGATIVE_ONE_FLOAT);
                    self.fstp(0x8c);
                    if self.word(0x34) < self.eax {
                        pc = 0x00d2_5500;
                    } else {
                        self.leave();
                        return;
                    }
                }
                // The top of the walk: the vectors are reloaded.
                0x00d2_54f0 => {
                    self.xmm[2] = self.vec_at(0xb0);
                    self.xmm[1] = self.vec_at(0x90);
                    pc = 0x00d2_5500;
                }
                0x00d2_5500 => {
                    self.edi = self.word(0x3c);
                    if self.edi >= self.e.mem.u32(self.esi + 0x14) {
                        self.leave();
                        return;
                    }
                    self.eax = self.collector;
                    self.fld(0x8c);
                    self.fld_at(self.eax + 4);
                    let c = self.x.fcompp();
                    if !c.parity(5) {
                        self.leave();
                        return;
                    }
                    // One step of the walk: the point where the ray crosses the next plane
                    // and the heights of the cell it is in.
                    self.edx = self.word(0x20);
                    self.fld(0x54 + 4 * self.edx);
                    self.ebx = self.word(0x34);
                    self.xmm[3] = load_vec(self.e, UNIT_VECTOR);
                    self.fstp(0x14);
                    self.xmm[0] = self.movss(0x14);
                    self.eax = self.at(0x54 + 4 * self.edx);
                    self.set_word(0x2c, self.eax);
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0);
                    self.xmm[3] = subps(self.xmm[3], self.xmm[0]);
                    self.xmm[3] = mulps(self.xmm[3], self.xmm[1]);
                    self.eax = self.ebx;
                    self.eax = self.eax.wrapping_sub(self.word(0x48));
                    self.xmm[1] = self.xmm[2];
                    self.xmm[1] = mulps(self.xmm[1], self.xmm[0]);
                    self.xmm[3] = addps(self.xmm[3], self.xmm[1]);
                    self.put_vec(0x60, self.xmm[3]);
                    self.set_word(0xac, self.eax);
                    self.ecx = self.edi;
                    if self.edx == 0 {
                        self.eax = self.edi.wrapping_sub(self.word(0x50));
                        self.ecx = self.esi;
                        self.push(self.eax);
                        self.push(self.ebx);
                        self.height();
                        self.fstp(0x30);
                        self.push(self.edi);
                        self.push(self.ebx);
                        self.height();
                        self.fstp(0x18);
                        self.fld(0x68);
                        self.fisub(0x3c);
                        self.fstp(0x40);
                        self.fld(0x40);
                        self.x.fabs();
                        self.fstp(0x40);
                        self.fld(0x30);
                        self.fld(0x18);
                        self.x.fld_st(0);
                        self.x.fsubp(2);
                        self.x.fxch(1);
                        self.fmul(0x40);
                        self.x.faddp(1);
                    } else if self.edx == 2 {
                        self.push(self.edi);
                        self.push(self.eax);
                        self.ecx = self.esi;
                        self.height();
                        self.fstp(0x40);
                        self.push(self.edi);
                        self.push(self.ebx);
                        self.ecx = self.esi;
                        self.height();
                        self.fstp(0x30);
                        self.fld(0x60);
                        self.fisub(0x34);
                        self.fstp(0x18);
                        self.fld(0x18);
                        self.x.fabs();
                        self.fstp(0x18);
                        self.fld(0x40);
                        self.fld(0x30);
                        self.x.fld_st(0);
                        self.x.fsubp(2);
                        self.x.fxch(1);
                        self.fmul(0x18);
                        self.x.faddp(1);
                    } else {
                        let along_diagonal = self.flag_byte(0x1f) == 0;
                        self.fld(0x60);
                        self.fisub(0x34);
                        self.set_word(0x40, self.edi);
                        self.fstp(0x18);
                        self.fld(0x18);
                        self.x.fabs();
                        self.fstp(0x18);
                        self.fld(0x18);
                        self.fstp(0x30);
                        if along_diagonal {
                            self.ecx = self.ecx.wrapping_sub(self.word(0x50));
                        } else {
                            self.edx = self.edi.wrapping_sub(self.word(0x50));
                            self.set_word(0x40, self.edx);
                        }
                        self.push(self.ecx);
                        self.push(self.eax);
                        self.ecx = self.esi;
                        self.height();
                        self.fstp(0x18);
                        self.eax = self.word(0x40);
                        self.push(self.eax);
                        self.push(self.ebx);
                        self.ecx = self.esi;
                        self.height();
                        self.fstp(0x40);
                        self.fld(0x18);
                        self.fld(0x40);
                        self.x.fld_st(0);
                        self.x.fsubp(2);
                        self.fld(0x30);
                        self.x.fmulp(2);
                        self.x.faddp(1);
                    }
                    pc = 0x00d2_5684;
                }
                // The height of the ray's plane at this step is on the x87 stack.
                0x00d2_5684 => {
                    self.ecx = (self.ecx & !0xff) | u32::from(self.flag);
                    let first = self.flag == 0;
                    self.fstp(0x30);
                    self.fld(0x14);
                    if first {
                        self.fld(0x6c);
                        self.fld(0x30);
                        let c = self.x.fcom(1);
                        self.x.fstp_st(1);
                        if !c.parity(0x41) {
                            pc = 0x00d2_5af9;
                            continue;
                        }
                        self.fld(0x88);
                        self.fld(0x44);
                        let c = self.x.fcom(1);
                        self.x.fstp_st(1);
                        if c.zero(0x41) {
                            pc = 0x00d2_5af7;
                            continue;
                        }
                        self.x.fldz();
                        self.fld(0x8c);
                        self.x.fxch(2);
                        self.x.fxch(3);
                        self.x.fxch(2);
                        self.x.fxch(1);
                        pc = 0x00d2_577a;
                        continue;
                    }
                    self.fld(0x6c);
                    self.fld(0x30);
                    let c = self.x.fcom(1);
                    if !c.parity(0x41) {
                        pc = 0x00d2_5af3;
                        continue;
                    }
                    self.fld(0x88);
                    self.fld(0x44);
                    self.x.fld_st(0);
                    self.x.fsubp(2);
                    self.x.fxch(1);
                    self.fstp(0x44);
                    self.x.fld_st(1);
                    self.x.fsubp(3);
                    self.x.fxch(2);
                    self.fstp(0x18);
                    self.fld(0x18);
                    self.fld(0x44);
                    self.x.fld_st(4);
                    self.fld(0x8c);
                    self.x.fld_st(0);
                    self.x.fsubp(2);
                    self.x.fxch(1);
                    self.fstp(0x18);
                    self.fld(0x18);
                    let fraction = f64::from(x87_quiet(self.fraction));
                    self.x.set(0, self.x.st(0) * fraction);
                    self.x.fsubp(2);
                    self.x.fxch(2);
                    let c = self.x.fcompp();
                    if c.zero(1) {
                        pc = 0x00d2_5af1;
                        continue;
                    }
                    self.x.fldz();
                    let c = self.fcom_mem(0x44);
                    if c.ah() & 1 != 0 {
                        pc = 0x00d2_577a;
                        continue;
                    }
                    self.x.fstp_st(2);
                    self.sp = self.sp.wrapping_sub(8);
                    self.x.fstp_st(3);
                    self.x.fstp_st(1);
                    self.x.fxch(1);
                    self.fstp(4);
                    self.fstp(0);
                    let words = [self.word(0), self.word(4)];
                    let result = self.e.call(FRACTION_HELPER, &words);
                    self.x.push(result.st0);
                    self.fstp(0x48);
                    self.sp = self.sp.wrapping_add(8);
                    self.x.fldz();
                    self.fld(0x30);
                    self.fld(0x14);
                    pc = 0x00d2_57c4;
                }
                // Where the ray crosses the surface inside the cell.
                0x00d2_577a => {
                    self.fld(0x6c);
                    self.x.fsub0(3);
                    self.fstp(0x44);
                    let c = self.x.fcom(1);
                    if c.parity(0x41) {
                        pc = 0x00d2_589f;
                        continue;
                    }
                    self.fld(0x88);
                    self.x.fsubrp(4);
                    self.x.fxch(3);
                    self.fstp(0x18);
                    self.fld(0x18);
                    self.fld(0x44);
                    self.x.fsubr0(1);
                    self.x.fdivp(1);
                    self.fstp(0x18);
                    self.fld(0x18);
                    self.x.fld_st(4);
                    self.x.fsub0(2);
                    self.x.fmulp(1);
                    self.x.faddp(1);
                    self.fstp(0x40);
                    self.x.fxch(1);
                    self.x.fxch(2);
                    pc = 0x00d2_57c4;
                }
                0x00d2_589f => {
                    self.x.fld_st(4);
                    self.x.fld_st(0);
                    self.x.fsubrp(3);
                    self.x.fdivrp(2);
                    self.x.fxch(1);
                    self.fstp(0x18);
                    self.fld(0x9c);
                    self.fld(0x18);
                    self.x.fld_st(0);
                    self.x.fld1();
                    self.x.fsubrp(1);
                    self.x.fmul0(4);
                    self.x.fxch(1);
                    self.x.fmulp(5);
                    self.x.faddp(4);
                    self.x.fxch(3);
                    self.fstp(0x18);
                    self.fld(0x18);
                    self.x.fsubp(3);
                    self.x.fxch(2);
                    self.fstp(0x14);
                    self.x.fxch(1);
                    let c = self.fcom_mem(0x14);
                    if c.zero(0x41) {
                        if (self.ecx & 0xff) == 0 {
                            pc = 0x00d2_5af7;
                            continue;
                        }
                        self.fst(0x14);
                    }
                    self.fld(0x14);
                    self.fld(0x44);
                    self.x.fsubr0(1);
                    self.x.fdivp(1);
                    self.fstp(0x18);
                    self.fld(0x18);
                    self.x.fmul0(3);
                    self.fstp(0x40);
                    self.x.fxch(2);
                    pc = 0x00d2_57c4;
                }
                // A hit candidate: the cell's triangle is built and reported.
                0x00d2_57c4 => {
                    self.eax = self.collector;
                    self.x.fxch(1);
                    self.fstp(0x44);
                    self.fld(0x6c);
                    self.fstp(0x88);
                    self.fstp(0x8c);
                    self.fld(0x40);
                    self.fld_at(self.eax + 4);
                    let c = self.x.fcompp();
                    if !c.parity(5) {
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    self.x.fld1();
                    self.edx = self.e.mem.u32(self.esi);
                    self.edx = self.e.mem.u32(self.edx + 0x34);
                    self.fst(0xd0);
                    self.x.fxch(1);
                    self.ebx = self.edi.wrapping_sub(self.word(0x50));
                    self.fstp(0xcc);
                    let diagonal_zero = self.flag_byte(0x1f) == 0;
                    self.eax = u32::MAX;
                    self.fstp(0xc4);
                    self.set_word(0xd4, self.eax);
                    self.set_word(0x100, 0);
                    self.set_word(0xe0, self.eax);
                    self.push(self.ebx);
                    self.ecx = self.esi;
                    let gt: bool;
                    if !diagonal_zero {
                        self.eax = self.word(0x38);
                        self.push(self.eax);
                        self.height();
                        self.fstp(0x30);
                        self.ecx = self.word(0xac);
                        self.push(self.edi);
                        self.push(self.ecx);
                        self.height();
                        self.fstp(0x14);
                        if self.word(0x20) != 1 {
                            self.eax = self.word(0x34);
                            self.push(self.edi);
                            self.push(self.eax);
                            self.height();
                            self.fstp(0x18);
                            self.fld(0x14);
                            self.fld(0x18);
                            self.x.fld_st(0);
                            gt = (self.edi as i32) > (self.ebx as i32);
                            self.x.fsubp(2);
                            self.x.fxch(1);
                            self.fstp(0xc0);
                            self.fsubr(0x30);
                        } else {
                            self.ecx = self.word(0xac);
                            self.push(self.ebx);
                            self.push(self.ecx);
                            self.height();
                            self.fstp(0x18);
                            self.fld(0x18);
                            self.x.fld_st(0);
                            self.fsub(0x30);
                            self.fstp(0xc0);
                            self.fsub(0x14);
                            gt = (self.ebx as i32) > (self.edi as i32);
                        }
                    } else {
                        self.eax = self.word(0xb0);
                        self.push(self.eax);
                        self.height();
                        self.fstp(0x14);
                        self.ecx = self.word(0x34);
                        self.push(self.edi);
                        self.push(self.ecx);
                        self.height();
                        self.fstp(0x30);
                        let axis = self.word(0x20);
                        let mut other_corner = axis == 0;
                        if axis == 1 {
                            self.fld(0x120);
                            self.fld(0x28);
                            let c = self.x.fcompp();
                            other_corner = c.zero(0x41);
                        }
                        if other_corner {
                            self.eax = self.word(0x34);
                            self.push(self.ebx);
                            self.push(self.eax);
                            self.height();
                            self.fstp(0x18);
                            self.fld(0x14);
                            self.fld(0x18);
                            self.x.fld_st(0);
                            self.x.fsubp(2);
                            self.x.fxch(1);
                            self.fstp(0xc0);
                            self.fsub(0x30);
                            gt = (self.ebx as i32) > (self.edi as i32);
                        } else {
                            self.ecx = self.word(0xac);
                            self.push(self.edi);
                            self.push(self.ecx);
                            self.height();
                            self.fstp(0x18);
                            self.fld(0x18);
                            self.x.fld_st(0);
                            gt = (self.edi as i32) > (self.ebx as i32);
                            self.fsub(0x30);
                            self.fstp(0xc0);
                            self.fsubr(0x14);
                        }
                    }
                    // The hit's normal from the cell's slopes, scaled and normalised.
                    self.xmm[1] = load_vec(self.e, self.esi + 0x30);
                    self.fstp(0xc8);
                    self.fild(0x48);
                    self.edx = self.word(0x50);
                    self.eax = u32::from(gt);
                    self.ecx = 0;
                    self.fmul(0xc0);
                    self.ecx = u32::from(self.flag_byte(0x13) != 0);
                    self.fstp(0xc0);
                    self.edx = ((self.edx as i32) >> 1) as u32;
                    self.fild(0x50);
                    self.edi = self.edi.wrapping_sub(self.edx);
                    self.edi <<= 0xf;
                    self.fmul(0xc8);
                    self.eax ^= self.ecx;
                    self.ecx = self.word(0x48);
                    self.fstp(0xc8);
                    self.xmm[0] = self.vec_at(0xc0);
                    self.fld(0x40);
                    self.xmm[1] = mulps(self.xmm[1], self.xmm[0]);
                    self.fstp(0xd0);
                    let normal = self.scaled_normal(self.xmm[1]);
                    self.ecx = ((self.ecx as i32) >> 1) as u32;
                    self.edi = self.edi.wrapping_sub(self.ecx);
                    self.edi = self.edi.wrapping_add(self.word(0x34));
                    self.ecx = self.collector;
                    self.edx = self
                        .eax
                        .wrapping_add(self.edi.wrapping_mul(2))
                        .wrapping_add(0xfffe_fffe);
                    self.set_word(0xd4, self.edx);
                    let hit = self.at(0xc0);
                    self.push(hit);
                    let extra = self.extra;
                    self.push(extra);
                    self.put_vec(0xc8, normal);
                    self.report_hit(hit);
                    self.sp = self.sp.wrapping_add(8);
                    self.flag = 0;
                    pc = 0x00d2_5b0f;
                }
                0x00d2_5af1 => {
                    self.x.pop();
                    pc = 0x00d2_5af3;
                }
                0x00d2_5af3 => {
                    self.x.fstp_st(1);
                    pc = 0x00d2_5af9;
                }
                0x00d2_5af7 => {
                    self.x.pop();
                    pc = 0x00d2_5af9;
                }
                0x00d2_5af9 => {
                    self.fstp(0x44);
                    self.fld(0x6c);
                    self.fstp(0x88);
                    self.fstp(0x8c);
                    pc = 0x00d2_5b0f;
                }
                // The walk steps along the chosen axis; the next axis is chosen.
                0x00d2_5b0f => {
                    self.eax = self.word(0x20);
                    self.edi = self.word(0x48 + 4 * self.eax);
                    self.fld(0x70 + 4 * self.eax);
                    let cell = self.word(0x34 + 4 * self.eax).wrapping_add(self.edi);
                    self.set_word(0x34 + 4 * self.eax, cell);
                    self.eax = self.word(0x2c);
                    self.ecx = self.word(0x24);
                    self.fadd_at(self.eax);
                    self.edx = 2;
                    self.ecx ^= self.edx;
                    let zero = self.flag_byte(0x1f) == 0;
                    self.fstp_at(self.eax);
                    self.set_word(0x24, self.ecx);
                    let next: u32 = if !zero {
                        if self.ecx == 0 {
                            1
                        } else {
                            self.fld(0x54);
                            self.fld(0x5c);
                            let c = self.x.fcompp();
                            if !c.zero(0x41) {
                                2
                            } else {
                                0
                            }
                        }
                    } else {
                        self.fld(0x120);
                        self.fld(0x28);
                        let c = self.x.fcompp();
                        if c.zero(0x41) {
                            if self.ecx != 0 {
                                2
                            } else {
                                self.fld(0x54);
                                self.fld(0x58);
                                let c = self.x.fcompp();
                                if !c.zero(0x41) {
                                    1
                                } else {
                                    self.ecx
                                }
                            }
                        } else if self.ecx == 0 {
                            self.fld(0x58);
                            self.fld(0x5c);
                            let c = self.x.fcompp();
                            if c.zero(0x41) {
                                1
                            } else {
                                2
                            }
                        } else {
                            0
                        }
                    };
                    self.set_word(0x20, next);
                    self.ecx = self.word(0x34);
                    if self.ecx < self.e.mem.u32(self.esi + 0x10) {
                        pc = 0x00d2_54f0;
                    } else {
                        self.leave();
                        return;
                    }
                }
                0x00d2_5c3d => {
                    self.x.fstp_st(2);
                    pc = 0x00d2_5c3f;
                }
                // The ray runs along a diagonal: a single cell is tested.
                0x00d2_5c3f => {
                    self.eax = self.e.mem.u32(self.esi + 0x10);
                    self.x.fstp_st(1);
                    self.eax = self.eax.wrapping_sub(1);
                    self.x.pop();
                    self.x.pop();
                    if self.edi >= self.eax {
                        self.x.fstp_st(1);
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    self.edx = self.e.mem.u32(self.esi + 0x14).wrapping_sub(1);
                    if self.ebx >= self.edx {
                        self.x.fstp_st(1);
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    self.x.fld1();
                    self.eax = self.e.mem.u32(self.esi);
                    self.xmm[0] = [f32::from_bits(self.e.mem.u32(ONE_FLOAT)), 0.0, 0.0, 0.0];
                    self.fstp(0xd0);
                    self.edx = self.e.mem.u32(self.eax + DIAGONAL_SLOT);
                    self.set_word(0xd4, self.ecx);
                    self.fisub(0x34);
                    self.set_word(0xe0, self.ecx);
                    self.ecx = self.at(0x1f);
                    self.push(self.ecx);
                    self.xmm[0] = shufps(self.xmm[0], self.xmm[0], 0);
                    self.ecx = self.esi;
                    self.set_word(0x104, 0);
                    self.put_vec(0xc4, self.xmm[0]);
                    self.fstp(0x24);
                    self.fisub(0x40);
                    self.fstp(0x2c);
                    self.call_this(self.edx, self.ecx, 1);
                    let diagonal_zero = self.flag_byte(0x1f) == 0;
                    self.push(self.ebx);
                    self.ecx = self.esi;
                    if !diagonal_zero {
                        self.edx = self.e.mem.u32(self.esi);
                        self.push(self.edi);
                        self.height();
                        self.fstp(0x24);
                        self.edx = self.e.mem.u32(self.esi);
                        self.eax = self.ebx.wrapping_add(1);
                        self.push(self.eax);
                        self.ecx = self.edi.wrapping_add(1);
                        self.push(self.ecx);
                        self.ecx = self.esi;
                        self.height();
                        self.fstp(0x14);
                        self.fld(0x20);
                        self.fld(0x28);
                        self.ecx = self.esi;
                        let c = self.x.fcompp();
                        if !c.parity(5) {
                            self.push(self.ebx);
                            self.eax = self.edi.wrapping_add(1);
                            self.push(self.eax);
                            self.height();
                            self.fstp(0x2c);
                            self.fld(0x2c);
                            self.edx = 1;
                            self.x.fld_st(0);
                            self.fld(0x24);
                            self.x.fld_st(0);
                            self.x.fsubp(2);
                            self.x.fxch(1);
                            self.fstp(0x18);
                            self.fld(0x14);
                            self.x.fsubrp(2);
                            self.x.fxch(1);
                            pc = 0x00d2_5e0d;
                        } else {
                            self.eax = self.ebx.wrapping_add(1);
                            self.push(self.eax);
                            self.push(self.edi);
                            self.height();
                            self.fstp(0x2c);
                            self.fld(0x14);
                            self.fld(0x2c);
                            self.x.fld_st(0);
                            self.x.fsubp(2);
                            self.x.fxch(1);
                            self.fstp(0x18);
                            self.fld(0x24);
                            self.x.fld_st(0);
                            self.x.fsubp(2);
                            self.x.fxch(1);
                            pc = 0x00d2_5e0b;
                        }
                    } else {
                        self.edx = self.e.mem.u32(self.esi);
                        self.eax = self.edi.wrapping_add(1);
                        self.push(self.eax);
                        self.height();
                        self.fstp(0x14);
                        self.edx = self.e.mem.u32(self.esi);
                        self.eax = self.ebx.wrapping_add(1);
                        self.push(self.eax);
                        self.push(self.edi);
                        self.ecx = self.esi;
                        self.height();
                        self.fstp(0x24);
                        self.fld(0x20);
                        self.edx = self.e.mem.u32(self.esi);
                        self.fadd(0x28);
                        self.ecx = self.esi;
                        let c = self.fcomp_double(ONE_DOUBLE);
                        if c.zero(0x41) {
                            self.eax = self.ebx.wrapping_add(1);
                            self.push(self.eax);
                            self.eax = self.edi.wrapping_add(1);
                            self.push(self.eax);
                            self.height();
                            self.fstp(0x2c);
                            self.fld(0x2c);
                            self.edx = 1;
                            self.x.fld_st(0);
                            self.fsub(0x24);
                            self.fstp(0x18);
                            self.fld(0x14);
                            self.x.fld_st(0);
                            self.x.fsubp(2);
                            self.x.fxch(1);
                            self.fstp(0x2c);
                            self.fld(0x2c);
                            self.x.fld_st(0);
                            self.fmul(0x28);
                            self.x.faddp(2);
                            self.fld(0x20);
                            self.fsub_double(ONE_DOUBLE);
                            self.fld(0x18);
                            self.x.fld_st(0);
                            self.x.fmulp(2);
                            self.x.fxch(3);
                            self.x.faddp(1);
                            pc = 0x00d2_5e2b;
                        } else {
                            self.push(self.ebx);
                            self.push(self.edi);
                            self.height();
                            self.fstp(0x2c);
                            self.fld(0x14);
                            self.fld(0x2c);
                            self.x.fld_st(0);
                            self.x.fsubp(2);
                            self.x.fxch(1);
                            self.fstp(0x18);
                            self.fld(0x24);
                            self.x.fsub0(1);
                            pc = 0x00d2_5e0b;
                        }
                    }
                }
                0x00d2_5e0b => {
                    self.edx = 0;
                    pc = 0x00d2_5e0d;
                }
                0x00d2_5e0d => {
                    self.fstp(0x2c);
                    self.fld(0x2c);
                    self.x.fld_st(0);
                    self.fmul(0x28);
                    self.x.faddp(2);
                    self.fld(0x18);
                    self.x.fld_st(0);
                    self.fmul(0x20);
                    self.x.faddp(3);
                    self.x.fxch(2);
                    pc = 0x00d2_5e2b;
                }
                0x00d2_5e2b => {
                    self.fstp(0x24);
                    self.x.fxch(1);
                    self.x.fchs();
                    self.fstp(0xc0);
                    self.x.fchs();
                    self.fstp(0xc8);
                    self.fld(0x9c);
                    self.fld(0x24);
                    self.x.fld_st(0);
                    self.x.fsubp(2);
                    self.x.fxch(1);
                    self.fstp(0x18);
                    self.fsubr(0xbc);
                    self.fstp(0x2c);
                    self.fld(0x2c);
                    self.fld(0x18);
                    let c = self.x.fcom(1);
                    if !c.parity(5) {
                        self.x.pop();
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    let flag_zero = self.flag == 0;
                    self.x.fldz();
                    let c = self.x.fcom(1);
                    if flag_zero {
                        if c.zero(0x41) {
                            self.x.fstp_st(1);
                            self.x.fstp_st(1);
                            self.x.pop();
                            self.leave();
                            return;
                        }
                    } else if c.zero(0x41) {
                        let fraction = f64::from(x87_quiet(self.fraction));
                        self.x.push(fraction);
                        self.x.fsubp(2);
                        self.x.fxch(2);
                        let c = self.x.fcompp();
                        if c.zero(0x41) {
                            self.x.pop();
                            self.leave();
                            return;
                        }
                        pc = 0x00d2_5f0e;
                        continue;
                    }
                    let c = self.x.fcomp(2);
                    if !c.parity(0x41) {
                        self.x.pop();
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    self.x.fld_st(0);
                    self.x.fsubrp(2);
                    self.x.fdivrp(1);
                    pc = 0x00d2_5f0e;
                }
                // The ray hits the surface in the single cell: the hit is reported.
                0x00d2_5f0e => {
                    self.ecx = self.collector;
                    self.fstp(0x14);
                    self.fld(0x14);
                    self.fld_at(self.ecx + 4);
                    let c = self.x.fcomp(1);
                    if !c.zero(0x41) {
                        self.x.pop();
                        self.leave();
                        return;
                    }
                    self.xmm[1] = self.vec_at(0xc0);
                    self.fstp(0xd0);
                    self.xmm[0] = load_vec(self.e, self.esi + 0x30);
                    self.xmm[1] = mulps(self.xmm[1], self.xmm[0]);
                    let normal = self.scaled_normal(self.xmm[1]);
                    self.ebx <<= 0xf;
                    self.ebx = self.ebx.wrapping_add(self.edi);
                    self.eax = self.edx.wrapping_add(self.ebx.wrapping_mul(2));
                    self.set_word(0xd4, self.eax);
                    let hit = self.at(0xc0);
                    self.push(hit);
                    let extra = self.extra;
                    self.push(extra);
                    self.put_vec(0xc8, normal);
                    self.report_hit(hit);
                    self.sp = self.sp.wrapping_add(8);
                    self.leave();
                    return;
                }
                other => panic!("castRayInternal: no block at {other:08x}"),
            }
        }
    }
}

// Translated from 00d24b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSampledHeightFieldShape::castRayInternal` (map name), thiscall with five stack words
/// (`RET 0x14`): `ECX` is `this`; `ray` points to two vectors (start at `+0`, end at `+0x10`),
/// `extra` is handed to the collector with every hit, `flag` (a byte, cleared once a hit has
/// been reported by the walk) selects the one-sided test, `fraction` is the slope allowance of
/// that test and `collector` holds the largest accepted hit fraction at `+4` and the hit
/// callback in slot 0 (thiscall `(extra, hit)`, `RET 8`).
///
/// The ray is scaled by the vector at `this + 0x30` and the first and third lane of the start
/// (offset by `this + 0x40`) give the starting cell; the shape's slot `0x38` writes the byte
/// that says which way the cells are split. The walk visits the cells the ray crosses (the
/// three axes are x, x +- z and z, each stepped by its inverse direction), asks the shape's
/// slot `0x34` for the heights at the cell corners and reports the first crossing of the
/// surface; a ray along a cell diagonal tests its single cell. The profile record
/// `"TtrcHeightFild"` (`010ce35c`) is started at the entry and every exit ends it.
///
/// The code is written the way the exe runs: a state machine over the exe's blocks with its
/// stack frame (the same offsets as the exe's `ESP`), its registers and the x87 stack. When
/// the second axis steps, the frame word at `+0x38` (the diagonal cell index) is read before
/// the exe has written it; the result only reaches the store at `+0x58`, which the walk
/// overwrites before it reads it (the single-cell path never reads it), so the zeroed frame
/// here changes no result. There is no exception frame and no cookie.
pub fn hkp_sampled_height_field_shape_cast_ray_internal(
    e: &mut Engine,
    this: Ptr,
    ray: Ptr,
    extra: u32,
    flag: u8,
    fraction: f32,
    collector: Ptr,
) {
    profile_record(e, CAST_RAY_LABEL);
    e.with_stack(0x200, |e, block| {
        let mut walk = Walk {
            e,
            x: X87 { stack: Vec::new() },
            sp: block.addr() + 0x40,
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esi: this.addr(),
            edi: 0,
            xmm: [[0.0; 4]; 5],
            flag,
            ray: ray.addr(),
            extra,
            fraction,
            collector: collector.addr(),
        };
        walk.run();
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d26230,
            hk_aabb_util_calc_aabb_ov3(Ptr, Ptr, Ptr, f32, Ptr)
        ),
        entry!(0x00d2_62e0, deleting_destructor_00d262e0(Ptr, u8) -> Ptr),
        entry!(
            0x00d2_49f0,
            hk_sampled_height_field_shape_get_aabb_impl(Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d2_6080,
            hk_sampled_height_field_shape_cast_ray_impl(Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d2_60d0,
            hk_sampled_height_field_shape_cast_ray_with_collector_impl(Ptr, Ptr, u32, Ptr)
        ),
        entry!(
            0x00d2_6120,
            hk_sampled_height_field_shape_cast_sphere(Ptr, Ptr, u32, Ptr)
        ),
        entry!(
            0x00d2_61b0,
            nearest_hit_collector_add_ray_hit(Ptr, u32, Ptr)
        ),
        entry!(
            0x00d2_6210,
            nearest_hit_collector_scalar_deleting_destructor(Ptr, u8) -> Ptr
        ),
        entry!(
            0x00d2_6310,
            rotate_normal_hit_collector_add_ray_hit(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d2_4b50,
            hkp_sampled_height_field_shape_cast_ray_internal(Ptr, Ptr, u32, u8, f32, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{monitor, profile_labels};
    use crate::units::havok_sdk::hkpsphereshape::{HALF_F32, THREE_F32};
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
    fn deleting_destructor_00d262e0_frees_only_when_bit_zero_is_set() {
        check(0x00d2_62e0, 0x00d2_4660);
    }

    #[test]
    fn hk_aabb_util_calc_aabb_ov3_boxes_a_rotated_box() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x010c_4000, 0x1000);
        for i in 0..4 {
            e.mem.set_u32(ABS_MASK_01017D10 + 4 * i, 0x7fff_ffff);
            e.mem.set_u32(SIGN_MASK_010C44D0 + 4 * i, 0x8000_0000);
        }
        // A quarter turn about z (x axis -> y, y axis -> -x) and a translation.
        let transform = e.mem.alloc(0x40);
        let columns: [[f32; 4]; 4] = [
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [10.0, 20.0, 30.0, 0.0],
        ];
        for (c, column) in columns.iter().enumerate() {
            for (i, value) in column.iter().enumerate() {
                e.mem
                    .set_f32(transform + 0x10 * c as u32 + 4 * i as u32, *value);
            }
        }
        let half = e.mem.alloc(16);
        let centre = e.mem.alloc(16);
        for (i, (h, c)) in [(1.0f32, 0.5f32), (2.0, 0.0), (3.0, 0.0)]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(half + 4 * i as u32, *h);
            e.mem.set_f32(centre + 4 * i as u32, *c);
        }
        let out = e.mem.alloc(0x20);
        e.call(
            0x00d2_6230,
            &args![
                Ptr::<()>::new(transform),
                Ptr::<()>::new(half),
                Ptr::<()>::new(centre),
                0.25f32,
                Ptr::<()>::new(out)
            ],
        );
        let read = |offset: u32| [0, 4, 8].map(|o| e.mem.f32(out + offset + o));
        // The x half extent 1 maps onto y, the y half extent 2 onto x.
        // extent = (2.25, 1.25, 3.25); middle = (10, 20.5, 30).
        assert_eq!(read(0), [7.75, 19.25, 26.75]);
        assert_eq!(read(0x10), [12.25, 21.75, 33.25]);
    }

    const HEIGHT_DOUBLE: u32 = 0x0900_0034;
    const INNER_COLLECTOR_DOUBLE: u32 = 0x0900_0050;
    const CALC_AABB: u32 = 0x00d2_6230;

    /// What a double saw, one entry per call.
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A shape (0x80 bytes) whose vtable slot `0x34` is a double answering `heights[x][z]`
    /// (0.0 outside the table); it logs `(this, x, z)` per call.
    fn height_shape(e: &mut Engine, heights: Vec<Vec<f64>>) -> (u32, Seen) {
        let this = e.mem.alloc(0x80);
        let table = e.mem.alloc(0x60);
        e.mem.set_u32(this, table);
        e.mem.set_u32(table + 0x34, HEIGHT_DOUBLE);
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(HEIGHT_DOUBLE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            let value = heights
                .get(a[1] as usize)
                .and_then(|row| row.get(a[2] as usize))
                .copied()
                .unwrap_or(0.0);
            Ret {
                st0: value,
                ..Ret::default()
            }
        });
        for lane in 0..4 {
            e.mem.set_f32(HALF_LANES + 4 * lane, 0.5);
        }
        e.mem.set_u32(HALF_F64, 0);
        e.mem.set_u32(HALF_F64 + 4, 0x3fe0_0000);
        (this, seen)
    }

    /// A double for `calcAabb_ov3` that logs its five words, then the four lanes of what the
    /// second and third words point at.
    fn calc_aabb_double(e: &mut Engine) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CALC_AABB, move |e, a| {
            let mut record = a.to_vec();
            for lane in 0..4 {
                record.push(e.mem.u32(a[1] + 4 * lane));
            }
            for lane in 0..4 {
                record.push(e.mem.u32(a[2] + 4 * lane));
            }
            log.borrow_mut().push(record);
            Ret::default()
        });
        seen
    }

    fn floats(words: &[u32]) -> Vec<f32> {
        words.iter().map(|word| f32::from_bits(*word)).collect()
    }

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
    }

    #[test]
    fn a_known_height_range_goes_straight_to_the_box_00d249f0() {
        let mut e = engine();
        let (this, heights) = height_shape(&mut e, vec![vec![9.0]]);
        let boxes = calc_aabb_double(&mut e);
        set_floats(&mut e, this + 0x50, &[10.0, 4.0, 6.0, 8.0]);
        e.mem.set_f32(this + 0x18, 1.5);
        e.mem.set_u32(this + 0x10, 3);
        e.mem.set_u32(this + 0x14, 3);
        e.call(0x00d2_49f0, &args![this, 0x7000u32, 0.25f32, 0x8000u32]);
        // Only the corner sample is asked; the range and the middle stay.
        assert_eq!(*heights.borrow(), vec![vec![this, 0, 0]]);
        assert_eq!(e.mem.f32(this + 0x54), 4.0);
        assert_eq!(e.mem.f32(this + 0x18), 1.5);
        let boxes = boxes.borrow();
        assert_eq!(boxes.len(), 1);
        let call = &boxes[0];
        assert_eq!(
            (call[0], call[3], call[4]),
            (0x7000, 0.25f32.to_bits(), 0x8000)
        );
        assert_eq!(floats(&call[5..9]), vec![5.0, 2.0, 3.0, 4.0]);
        assert_eq!(floats(&call[9..13]), vec![5.0, 1.5, 3.0, 4.0]);
    }

    #[test]
    fn a_range_that_is_not_negative_is_kept_whatever_it_is_00d249f0() {
        let mut e = engine();
        let (this, heights) = height_shape(&mut e, vec![]);
        let boxes = calc_aabb_double(&mut e);
        for range in [0.0f32, f32::NAN, -0.0] {
            set_floats(&mut e, this + 0x50, &[2.0, range, 2.0, 2.0]);
            e.call(0x00d2_49f0, &args![this, 0u32, 0.0f32, 0u32]);
            assert_eq!(e.mem.u32(this + 0x54), range.to_bits());
        }
        assert_eq!(heights.borrow().len(), 3);
        assert_eq!(boxes.borrow().len(), 3);
    }

    #[test]
    fn a_negative_range_scans_every_sample_00d249f0() {
        let mut e = engine();
        let table = vec![vec![1.0, 5.0, 2.0], vec![-3.0, 4.0, 0.0]];
        let (this, heights) = height_shape(&mut e, table);
        let boxes = calc_aabb_double(&mut e);
        set_floats(&mut e, this + 0x50, &[8.0, -1.0, 4.0, 2.0]);
        e.mem.set_f32(this + 0x24, 2.0);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 3);
        e.call(0x00d2_49f0, &args![this, 0x7000u32, 0.5f32, 0x8000u32]);
        let order: Vec<(u32, u32)> = heights.borrow().iter().map(|a| (a[1], a[2])).collect();
        assert_eq!(
            order,
            vec![(0, 0), (0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
        );
        // Scaled: lowest -6, highest 10.
        assert_eq!(e.mem.f32(this + 0x18), 2.0);
        assert_eq!(e.mem.f32(this + 0x54), 16.0);
        let boxes = boxes.borrow();
        assert_eq!(floats(&boxes[0][5..9]), vec![4.0, 8.0, 2.0, 1.0]);
        assert_eq!(floats(&boxes[0][9..13]), vec![4.0, 2.0, 2.0, 1.0]);
    }

    #[test]
    fn a_negative_scale_swaps_the_scaled_extremes_00d249f0() {
        let mut e = engine();
        let table = vec![vec![1.0, 5.0], vec![-3.0, 4.0]];
        let (this, _) = height_shape(&mut e, table);
        calc_aabb_double(&mut e);
        set_floats(&mut e, this + 0x50, &[2.0, -1.0, 2.0, 2.0]);
        e.mem.set_f32(this + 0x24, -1.0);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 2);
        e.call(0x00d2_49f0, &args![this, 0u32, 0.0f32, 0u32]);
        // Scaled: lowest 3, highest -5 -> low -5, high 3.
        assert_eq!(e.mem.f32(this + 0x18), -1.0);
        assert_eq!(e.mem.f32(this + 0x54), 8.0);
    }

    #[test]
    fn without_samples_the_corner_height_gives_a_flat_range_00d249f0() {
        let mut e = engine();
        let (this, heights) = height_shape(&mut e, vec![vec![3.0]]);
        calc_aabb_double(&mut e);
        set_floats(&mut e, this + 0x50, &[2.0, -1.0, 2.0, 2.0]);
        e.mem.set_f32(this + 0x24, 1.0);
        e.call(0x00d2_49f0, &args![this, 0u32, 0.0f32, 0u32]);
        assert_eq!(heights.borrow().len(), 1);
        assert_eq!(e.mem.f32(this + 0x18), 3.0);
        assert_eq!(e.mem.f32(this + 0x54), 0.0);
        // A count of zero in the other direction also skips the inner loop.
        e.mem.set_f32(this + 0x54, -1.0);
        e.mem.set_u32(this + 0x10, 2);
        e.mem.set_u32(this + 0x14, 0);
        e.call(0x00d2_49f0, &args![this, 0u32, 0.0f32, 0u32]);
        assert_eq!(heights.borrow().len(), 2);
        assert_eq!(e.mem.f32(this + 0x54), 0.0);
    }

    #[test]
    fn the_tolerance_and_the_middle_are_quieted_00d249f0() {
        let mut e = engine();
        let (this, _) = height_shape(&mut e, vec![]);
        let boxes = calc_aabb_double(&mut e);
        set_floats(&mut e, this + 0x50, &[2.0, 2.0, 2.0, 2.0]);
        e.mem.set_u32(this + 0x18, 0x7f80_0001);
        e.call(
            0x00d2_49f0,
            &args![this, 0u32, f32::from_bits(0xff80_0002), 0u32],
        );
        let boxes = boxes.borrow();
        assert_eq!(boxes[0][3], 0xffc0_0002);
        assert_eq!(boxes[0][10], 0x7fc0_0001);
    }

    /// `CAST_RAY_INTERNAL` double: logs its six words (`this` first) and, for each, the
    /// `words` words the last one (the collector) or the second (the ray) points at.
    fn cast_double(e: &mut Engine, ray_words: u32, collector_words: u32, found: u8) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CAST_RAY_INTERNAL, move |e, a| {
            let mut record = a.to_vec();
            for word in 0..ray_words {
                record.push(e.mem.u32(a[1] + 4 * word));
            }
            for word in 0..collector_words {
                record.push(e.mem.u32(a[5] + 4 * word));
            }
            log.borrow_mut().push(record);
            e.mem.set_u8(a[5] + 8, found);
            Ret::default()
        });
        seen
    }

    #[test]
    fn the_ray_cast_makes_a_nearest_hit_collector_00d26080() {
        for found in [1u8, 0] {
            let mut e = engine();
            let seen = cast_double(&mut e, 0, 4, found);
            let input = e.mem.alloc(0x20);
            e.mem.set_u32(input + 0x10, 0x7f80_0001);
            let out = e.mem.alloc(4);
            e.mem.set_u32(out, 0xcdcd_cdcd);
            e.call(0x00d2_6080, &args![0x5000u32, out, 0x6000u32, input]);
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            let call = &seen[0];
            assert_eq!(call[..5], [0x5000, 0x6000, 0, 0, 0]);
            // The collector: vtable, the quieted fraction, the flag, the input.
            assert_eq!(call[6], NEAREST_HIT_COLLECTOR_VTABLE);
            assert_eq!(call[7], 0x7fc0_0001);
            assert_eq!(call[8] & 0xff, 0);
            assert_eq!(call[9], input);
            assert_eq!(e.mem.u32(out), 0xcdcd_cd00 | u32::from(found));
        }
    }

    #[test]
    fn fn_00d26080_answers_its_result_pointer() {
        let mut e = engine();
        cast_double(&mut e, 0, 4, 1);
        let input = e.mem.alloc(0x20);
        let out = e.mem.alloc(4);
        // EAX is `out` at the RET (the hkBool result pointer the callers read through).
        let answered = e
            .call(0x00d2_6080, &args![0x5000u32, out, 0x6000u32, input])
            .u32();
        assert_eq!(answered, out);
    }

    #[test]
    fn the_ray_cast_with_a_collector_passes_the_extra_word_00d260d0() {
        let mut e = engine();
        let seen = cast_double(&mut e, 2, 3, 7);
        let hits = e.mem.alloc(0x20);
        e.mem.set_f32(hits + 4, 0.75);
        let ray = e.mem.alloc(8);
        e.mem.set_u32(ray, 0x1234);
        e.call(0x00d2_60d0, &args![0x5000u32, ray, 0x7000u32, hits]);
        let seen = seen.borrow();
        let call = &seen[0];
        assert_eq!(call[..5], [0x5000, ray, 0x7000, 0, 0]);
        assert_eq!(call[6], 0x1234);
        assert_eq!(call[8], ROTATE_NORMAL_HIT_COLLECTOR_VTABLE);
        assert_eq!(call[9], 0.75f32.to_bits());
        assert_eq!(call[10], hits);
    }

    #[test]
    fn the_sphere_cast_moves_the_ray_by_the_radius_00d26120() {
        for (scale, radius) in [(2.0f32, -0.5f32), (-2.0, 0.5), (0.0, 0.5), (f32::NAN, 0.5)] {
            let mut e = engine();
            let seen = cast_double(&mut e, 12, 0, 0);
            let shape = e.mem.alloc(0x40);
            e.mem.set_f32(shape + 0x24, scale);
            let input = e.mem.alloc(0x40);
            let words: Vec<f32> = (1..=12).map(|n| n as f32).collect();
            set_floats(&mut e, input, &words);
            e.mem.set_f32(input + 0x30, 0.5);
            e.mem.set_f32(input + 0x34, 0.125);
            let hits = e.mem.alloc(0x10);
            e.call(0x00d2_6120, &args![shape, input, 0x7000u32, hits]);
            let seen = seen.borrow();
            let call = &seen[0];
            assert_ne!(call[1], input);
            assert_eq!((call[0], call[2], call[3]), (shape, 0x7000, 1));
            assert_eq!((call[4], call[5]), (0.125f32.to_bits(), hits));
            let mut expected = words.clone();
            expected[1] += radius;
            expected[5] += radius;
            assert_eq!(floats(&call[6..18]), expected);
        }
    }

    #[test]
    fn the_sphere_cast_quiets_the_values_it_loads_00d26120() {
        let mut e = engine();
        let seen = cast_double(&mut e, 12, 0, 0);
        let shape = e.mem.alloc(0x40);
        let input = e.mem.alloc(0x40);
        e.mem.set_u32(input + 4, 0x7f80_0003);
        e.mem.set_u32(input + 0x30, 0x3f80_0000);
        e.mem.set_u32(input + 0x34, 0x7f80_0005);
        let hits = e.mem.alloc(0x10);
        e.call(0x00d2_6120, &args![shape, input, 0u32, hits]);
        let seen = seen.borrow();
        assert_eq!(seen[0][4], 0x7fc0_0005);
        assert_eq!(seen[0][7], 0x7fc0_0003);
    }

    /// A nearest-hit collector with the record `record` (0x50 bytes of `0xcd`).
    fn nearest_collector(e: &mut Engine, best: f32) -> (u32, u32) {
        let record = e.mem.alloc(0x50);
        e.mem.write(record, &[0xcd; 0x50]);
        e.mem.set_u32(record + 0x40, 2);
        let collector = e.mem.alloc(0x10);
        e.mem.write(collector, &[0; 0x10]);
        e.mem.set_f32(collector + 4, best);
        e.mem.set_u32(collector + 0xc, record);
        (collector, record)
    }

    #[test]
    fn a_nearer_hit_replaces_the_record_00d261b0() {
        let mut e = engine();
        let (collector, record) = nearest_collector(&mut e, 1.0);
        let hit = e.mem.alloc(0x20);
        set_floats(&mut e, hit, &[0.0, 1.0, 0.0, 0.5, 0.25]);
        e.mem.set_u32(hit + 0x14, 0xabcd);
        e.call(0x00d2_61b0, &args![collector, 0u32, hit]);
        assert_eq!(e.mem.f32(collector + 4), 0.25);
        assert_eq!(e.mem.u8(collector + 8), 1);
        assert_eq!(
            floats(&[e.mem.u32(record + 4), e.mem.u32(record + 0xc)]),
            [1.0, 0.5]
        );
        assert_eq!(e.mem.f32(record + 0x10), 0.25);
        assert_eq!(e.mem.u32(record + 0x14), 0xabcd);
        // The word at `+0x40` indexes the list at `+0x20`.
        assert_eq!(e.mem.u32(record + 0x28), u32::MAX);
        assert_eq!(e.mem.u32(record + 0x24), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(record + 0x18), 0xcdcd_cdcd);
        // A farther hit, an equal one and an unordered one change nothing.
        for fraction in [0.5f32, 0.25, f32::NAN] {
            e.mem.set_f32(hit + 0x10, fraction);
            e.mem.set_f32(hit + 4, 9.0);
            e.call(0x00d2_61b0, &args![collector, 0u32, hit]);
            assert_eq!(e.mem.f32(collector + 4), 0.25);
            assert_eq!(e.mem.f32(record + 4), 1.0);
        }
    }

    #[test]
    fn a_hit_leaves_the_found_flag_alone_when_it_is_not_nearer_00d261b0() {
        let mut e = engine();
        let (collector, _) = nearest_collector(&mut e, 0.5);
        let hit = e.mem.alloc(0x20);
        e.mem.set_f32(hit + 0x10, 0.75);
        e.call(0x00d2_61b0, &args![collector, 0u32, hit]);
        assert_eq!(e.mem.u8(collector + 8), 0);
    }

    #[test]
    fn the_collector_destructor_frees_when_bit_zero_is_set_00d26210() {
        for (flags, freed) in [
            (0u32, 0usize),
            (1, 1),
            (2, 0),
            (3, 1),
            (0xffff_ff01, 1),
            (0x100, 0),
        ] {
            let mut e = engine();
            let seen = Rc::new(RefCell::new(Vec::new()));
            let log = Rc::clone(&seen);
            e.register_double(FREE, move |_, a| {
                log.borrow_mut().push(a.to_vec());
                Ret::default()
            });
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this, 0x1111);
            let got = e.call(0x00d2_6210, &args![this, flags]).u32();
            assert_eq!(got, this);
            assert_eq!(e.mem.u32(this), PURE_VIRTUAL_VTABLE);
            assert_eq!(seen.borrow().len(), freed);
            if freed == 1 {
                assert_eq!(seen.borrow()[0], vec![this]);
            }
        }
    }

    #[test]
    fn the_normal_is_rotated_and_handed_to_the_inner_collector_00d26310() {
        let mut e = engine();
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(INNER_COLLECTOR_DOUBLE, move |e, a| {
            let mut record = a.to_vec();
            for word in 0..8 {
                record.push(e.mem.u32(a[2] + 4 * word));
            }
            log.borrow_mut().push(record);
            Ret::default()
        });
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, INNER_COLLECTOR_DOUBLE);
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(inner, table);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, inner);
        let matrix = e.mem.alloc(0x30);
        set_floats(
            &mut e,
            matrix,
            &[
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
            ],
        );
        let context = e.mem.alloc(0x10);
        e.mem.set_u32(context + 8, matrix);
        let hit = e.mem.alloc(0x20);
        set_floats(&mut e, hit, &[0.5, 0.25, 2.0, 1.0, 21.0, 22.0, 23.0, 24.0]);
        e.call(0x00d2_6310, &args![this, context, hit]);
        let seen = seen.borrow();
        let call = &seen[0];
        assert_eq!((call[0], call[1]), (inner, context));
        // (y * column1 + x * column0) + z * column2 per lane.
        let columns = [
            [1.0f32, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
        ];
        let expected: Vec<f32> = (0..4)
            .map(|lane| (0.25 * columns[1][lane] + 0.5 * columns[0][lane]) + 2.0 * columns[2][lane])
            .collect();
        assert_eq!(floats(&call[3..7]), expected);
        assert_eq!(floats(&call[7..11]), vec![21.0, 22.0, 23.0, 24.0]);
        // The hit itself is not touched.
        assert_eq!(e.mem.f32(hit), 0.5);
    }

    const FLAG_SLOT_DOUBLE: u32 = 0x0900_0038;
    const ADD_HIT_DOUBLE: u32 = 0x0900_0060;
    const EXTRA_WORD: u32 = 0x1234;

    /// A cast: the ray (start and end, the fourth lane zero), the flag byte, the fraction
    /// argument, the largest fraction the collector accepts and the bytes slot `0x38` of the
    /// shape writes (the first call gives the orientation of the diagonal axis, the second
    /// the split of the cell; the last byte repeats).
    struct Request {
        start: [f32; 3],
        end: [f32; 3],
        flag: u8,
        fraction: f32,
        max: f32,
        bytes: &'static [u8],
    }

    /// What a cast did.
    struct Outcome {
        /// The extra word and the 24 words at the hit pointer, per reported hit.
        hits: Vec<(u32, Vec<u32>)>,
        /// The `(x, z)` of every height call.
        heights: Vec<(u32, u32)>,
        /// The pointers handed to slot `0x38`.
        slot_calls: Vec<u32>,
        labels: Vec<u32>,
    }

    fn flat() -> Vec<Vec<f64>> {
        vec![vec![0.0; 8]; 8]
    }

    fn terrain(points: &[(usize, usize, f64)]) -> Vec<Vec<f64>> {
        let mut grid = flat();
        for &(x, z, height) in points {
            grid[x][z] = height;
        }
        grid
    }

    /// Runs `castRayInternal` on an 8 by 8 shape of unit scale whose heights are `grid[x][z]`,
    /// with a collector that keeps the fraction of every hit as its new limit.
    fn cast(grid: Vec<Vec<f64>>, request: &Request) -> Outcome {
        let mut e = engine();
        let (stream, records) = monitor(&mut e, 0x60);
        for lane in 0..4 {
            e.mem.set_f32(CELL_MAGIC + 4 * lane, 196608.0);
            e.mem.set_f32(UNIT_VECTOR + 4 * lane, 1.0);
        }
        e.mem.set_u32(DIRECTION_LIMIT, 0x3800_0000);
        e.mem.set_u32(BIG_FLOAT, 0x7f7f_ffee);
        e.mem.set_u32(NEGATIVE_BIG_FLOAT, 0xff7f_ffee);
        e.mem.set_f32(NEGATIVE_ONE_FLOAT, -1.0);
        e.mem.set_f32(ONE_FLOAT, 1.0);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        let (shape, heights) = height_shape(&mut e, grid);
        let table = e.mem.u32(shape);
        e.mem.set_u32(table + DIAGONAL_SLOT, FLAG_SLOT_DOUBLE);
        e.mem.set_u32(shape + 0x10, 8);
        e.mem.set_u32(shape + 0x14, 8);
        set_floats(&mut e, shape + 0x30, &[1.0; 4]);
        let slot_calls: Rc<RefCell<Vec<u32>>> = Rc::default();
        let sink = Rc::clone(&slot_calls);
        let bytes = request.bytes;
        e.register_double(FLAG_SLOT_DOUBLE, move |e, a| {
            let call = sink.borrow().len();
            sink.borrow_mut().push(a[1]);
            e.mem.set_u8(a[1], bytes[call.min(bytes.len() - 1)]);
            Ret::default()
        });
        let collector = e.mem.alloc(0x10);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(collector, vtable);
        e.mem.set_u32(vtable, ADD_HIT_DOUBLE);
        e.mem.set_f32(collector + 4, request.max);
        let hits: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::default();
        let sink = Rc::clone(&hits);
        e.register_double(ADD_HIT_DOUBLE, move |e, a| {
            let record: Vec<u32> = (0..0x18).map(|word| e.mem.u32(a[2] + 4 * word)).collect();
            e.mem.set_u32(a[0] + 4, record[4]);
            sink.borrow_mut().push((a[1], record));
            Ret::default()
        });
        let ray = e.mem.alloc(0x20);
        set_floats(&mut e, ray, &request.start);
        set_floats(&mut e, ray + 0x10, &request.end);
        e.call(
            CAST_RAY_INTERNAL,
            &args![
                shape,
                ray,
                EXTRA_WORD,
                u32::from(request.flag),
                request.fraction,
                collector
            ],
        );
        let outcome = Outcome {
            hits: hits.borrow().clone(),
            heights: heights.borrow().iter().map(|c| (c[1], c[2])).collect(),
            slot_calls: slot_calls.borrow().clone(),
            labels: profile_labels(&e, stream, records),
        };
        assert_eq!(outcome.labels, vec![CAST_RAY_LABEL, PROFILE_END]);
        outcome
    }

    fn assert_near(words: &[u32], expected: &[f32]) {
        for (word, want) in words.iter().zip(expected) {
            let got = f32::from_bits(*word);
            assert!((got - want).abs() <= 1e-5, "{got} is not {want}");
        }
    }

    fn unit(x: f32, y: f32, z: f32) -> [f32; 3] {
        let length = (x * x + y * y + z * z).sqrt();
        [x / length, y / length, z / length]
    }

    /// A ray straight down over the cell `(2, 3)` at `(x, 3 + z)`.
    fn vertical(x: f32, z: f32, bytes: &'static [u8]) -> Request {
        Request {
            start: [2.0 + x, 5.0, 3.0 + z],
            end: [2.0 + x, -5.0, 3.0 + z],
            flag: 0,
            fraction: 1.0,
            max: 1.0,
            bytes,
        }
    }

    #[test]
    fn a_vertical_ray_picks_the_triangle_of_its_cell_00d24b50() {
        // (split byte, x, z, corner heights, height calls, slopes along x and z, height).
        type Case = (
            u8,
            f32,
            f32,
            Vec<(usize, usize, f64)>,
            Vec<(u32, u32)>,
            f32,
            f32,
            f64,
        );
        let split_one = vec![(2, 4, 1.0), (3, 3, 2.0), (3, 4, 3.0)];
        let split_zero = vec![(3, 3, 2.0), (2, 4, 1.0), (3, 4, 4.0)];
        let cases: Vec<Case> = vec![
            // Split 1, the ray on or above the diagonal: corner (x, z + 1) is read last.
            (
                1,
                0.5,
                0.5,
                split_one.clone(),
                vec![(2, 3), (3, 4), (2, 4)],
                2.0,
                1.0,
                1.5,
            ),
            // Split 1, below the diagonal: corner (x + 1, z) is read last.
            (
                1,
                0.75,
                0.25,
                split_one,
                vec![(2, 3), (3, 4), (3, 3)],
                2.0,
                1.0,
                1.75,
            ),
            // Split 0, below the anti-diagonal (and on it).
            (
                0,
                0.25,
                0.5,
                split_zero.clone(),
                vec![(3, 3), (2, 4), (2, 3)],
                2.0,
                1.0,
                1.0,
            ),
            (
                0,
                0.5,
                0.5,
                split_zero.clone(),
                vec![(3, 3), (2, 4), (2, 3)],
                2.0,
                1.0,
                1.5,
            ),
            // Split 0, above the anti-diagonal.
            (
                0,
                0.75,
                0.5,
                split_zero,
                vec![(3, 3), (2, 4), (3, 4)],
                3.0,
                2.0,
                2.25,
            ),
        ];
        for (split, x, z, points, calls, slope_x, slope_z, height) in cases {
            for flag in [0u8, 1] {
                let mut request = vertical(x, z, if split == 1 { &[0, 1] } else { &[0, 0] });
                request.flag = flag;
                let out = cast(terrain(&points), &request);
                assert_eq!(out.heights, calls, "{split} {x} {z}");
                assert_eq!(out.slot_calls.len(), 2);
                assert_eq!(out.slot_calls[1] - out.slot_calls[0], 0xc);
                assert_eq!(out.hits.len(), 1, "{split} {x} {z}");
                let (extra, record) = &out.hits[0];
                assert_eq!(*extra, EXTRA_WORD);
                let n = unit(-slope_x, 1.0, -slope_z);
                let length = (slope_x * slope_x + 1.0 + slope_z * slope_z).sqrt();
                assert_near(&record[..4], &[n[0], n[1], n[2], 1.0 / length]);
                let fraction = ((5.0 - height) / 10.0) as f32;
                assert_eq!(record[4], fraction.to_bits(), "{split} {x} {z}");
                let triangle = u32::from((split == 1 && z < x) || (split == 0 && x + z > 1.0));
                assert_eq!(record[5], 196612 + triangle, "{split} {x} {z}");
                assert_eq!((record[8], record[16]), (u32::MAX, 0));
            }
        }
    }

    #[test]
    fn a_vertical_ray_that_does_not_cross_the_surface_downwards_reports_nothing_00d24b50() {
        let points = [(2, 4, 1.0), (3, 4, 3.0)];
        // The surface is 1.5 high where the ray runs. Starting below it, going up, staying
        // above it, and a collector that accepts less than the fraction 0.35 all give nothing.
        let ways: [(f32, f32, f32); 4] = [
            (1.0, -5.0, 1.0),
            (-5.0, 5.0, 1.0),
            (5.0, 3.0, 1.0),
            (5.0, -5.0, 0.2),
        ];
        for (start, end, max) in ways {
            let mut request = vertical(0.5, 0.5, &[0, 1]);
            request.start[1] = start;
            request.end[1] = end;
            request.max = max;
            let out = cast(terrain(&points), &request);
            assert!(out.hits.is_empty(), "{start} {end} {max}");
            assert_eq!(out.slot_calls.len(), 2);
        }
    }

    #[test]
    fn a_ray_that_starts_under_the_surface_hits_at_zero_with_the_flag_00d24b50() {
        let points = [(2, 4, 1.0), (3, 4, 3.0)];
        // d0 = -0.5; the end is 4.5 below the surface (hit) or 1.0 below it (no hit).
        for (end, hit) in [(-3.0f32, true), (0.5, false)] {
            let mut request = vertical(0.5, 0.5, &[0, 1]);
            request.start[1] = 1.0;
            request.end[1] = end;
            request.flag = 1;
            let out = cast(terrain(&points), &request);
            assert_eq!(out.hits.len(), usize::from(hit), "{end}");
            if hit {
                assert_eq!(out.hits[0].1[4], 0);
            }
            // Without the flag the ray is one-sided and nothing is found.
            request.flag = 0;
            assert!(cast(terrain(&points), &request).hits.is_empty());
        }
    }

    #[test]
    fn a_vertical_ray_outside_the_cells_is_not_tested_00d24b50() {
        for (x, z) in [(7.5f32, 3.5f32), (2.5, 7.5)] {
            let mut request = vertical(0.5, 0.5, &[0, 1]);
            request.start[0] = x;
            request.end[0] = x;
            request.start[2] = z;
            request.end[2] = z;
            let out = cast(flat(), &request);
            assert!(out.hits.is_empty() && out.heights.is_empty());
            assert_eq!(out.slot_calls.len(), 1);
        }
    }

    /// A ray over a flat zero surface from y = 5 to y = -5: the hit is at the fraction 0.5
    /// whatever the way the walk reaches it.
    fn slanted(start: [f32; 2], end: [f32; 2], flag: u8, bytes: &'static [u8]) -> Request {
        Request {
            start: [start[0], 5.0, start[1]],
            end: [end[0], -5.0, end[1]],
            flag,
            fraction: 0.0,
            max: 1.0,
            bytes,
        }
    }

    /// The hit of a walk over flat ground: fraction 0.5, normal up, in the cell `(x, z)`.
    fn assert_flat_hit(out: &Outcome, x: u32, z: u32) {
        assert_eq!(out.hits.len(), 1);
        let (extra, record) = &out.hits[0];
        assert_eq!(*extra, EXTRA_WORD);
        assert_near(&record[..3], &[0.0, 1.0, 0.0]);
        assert_near(&record[4..5], &[0.5]);
        assert_eq!(record[5] >> 1, (z << 15) + x, "{:#x}", record[5]);
        assert_eq!((record[8], record[16]), (u32::MAX, 0));
        assert_eq!(out.slot_calls.len(), 1);
    }

    #[test]
    fn a_slanted_ray_walks_the_cells_to_the_surface_00d24b50() {
        for flag in [0u8, 1] {
            let out = cast(flat(), &slanted([0.5, 0.3], [4.3, 2.15], flag, &[0]));
            assert_flat_hit(&out, 2, 1);
            assert!(out.heights.len() > 8);
        }
        // The diagonal axis the other way round and a negative step along z.
        let out = cast(flat(), &slanted([0.5, 3.3], [4.3, 1.15], 0, &[1]));
        assert_flat_hit(&out, 2, 2);
        // A ray going up never crosses the surface downwards.
        let mut request = slanted([0.5, 0.3], [4.3, 2.15], 0, &[0]);
        request.start[1] = -5.0;
        request.end[1] = 5.0;
        assert!(cast(flat(), &request).hits.is_empty());
    }

    #[test]
    fn a_walk_against_the_axes_and_from_outside_the_grid_00d24b50() {
        // Starting beyond the last cells, stepping down in x or in z.
        let out = cast(flat(), &slanted([9.3, 2.15], [0.5, 0.3], 0, &[0]));
        assert_flat_hit(&out, 4, 1);
        let out = cast(flat(), &slanted([2.15, 9.3], [0.3, 0.5], 0, &[0]));
        assert_flat_hit(&out, 1, 4);
        // Starting before the first cells, stepping up.
        let out = cast(flat(), &slanted([-1.5, 0.3], [4.3, 2.15], 0, &[0]));
        assert_flat_hit(&out, 1, 1);
        let out = cast(flat(), &slanted([0.3, -1.5], [2.15, 4.3], 0, &[0]));
        assert_flat_hit(&out, 1, 1);
        // Rays that end before they reach the cells.
        for (start, end) in [([9.3, 2.15], [8.5, 0.3]), ([2.15, 9.3], [0.3, 8.5])] {
            let out = cast(flat(), &slanted(start, end, 0, &[0]));
            assert!(out.hits.is_empty());
        }
        for (start, end) in [([-3.5, 0.3], [-0.5, 2.15]), ([0.3, -3.5], [2.15, -0.5])] {
            let out = cast(flat(), &slanted(start, end, 0, &[0]));
            assert!(out.hits.is_empty());
        }
    }
}
