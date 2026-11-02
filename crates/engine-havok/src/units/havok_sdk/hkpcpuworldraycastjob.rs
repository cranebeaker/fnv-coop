//! `Havok SDK/hkpcpuworldraycastjob.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector code (`SUBPS`, `MULPS`, `ADDPS`, `RCPPS`) computes in `f32`; `RCPPS` is the host
//! CPU's estimate. Floats the code moves through `FLD` / `FSTP` go through `x87_quiet_bits`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpcpuagentsectorjob::JOB_DONE;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_vec, profile_record, store_vec, Vec4, PROFILE_END,
};

/// The memory router (`00c85750`): word `+0` is the stack allocator's top, `+0x10` the heap
/// allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (engine map; cdecl, four words: the allocator, an array, the wanted
/// capacity and the element size).
pub(crate) const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// Pointer to the function `addRayHit` of the collector calls: three words, cdecl.
pub(crate) const ADD_RAY_HIT_HOOK: u32 = 0x0126_ec04;
/// What the hook is set to when a collector is used (`00e2b9b0`, unnamed in the engine map).
pub(crate) const ADD_RAY_HIT_TARGET: u32 = 0x00e2_b9b0;
/// Unnamed in the engine map (cdecl, seven words: a job word, the array of ray pairs, the rays
/// of the job, the world, the first output structure, the scratch block and the ray count).
const CAST_BUNDLES_FIRST: u32 = 0x00e2_c8a0;
/// Unnamed in the engine map (cdecl, seven words, as `00e2c8a0`).
const CAST_BUNDLES_SECOND: u32 = 0x00e2_cc30;
/// Unnamed in the engine map (cdecl, eight words: a job word, a ray, the world, the collector and
/// four zeros).
const CAST_ONE_RAY: u32 = 0x00e2_abf0;
/// Unnamed in the engine map (cdecl, nine words: a job word, a block of four rays, the world,
/// a vector and five zeros).
const CAST_BLOCK_FIRST: u32 = 0x00e2_b070;
/// Unnamed in the engine map (cdecl, nine words, as `00e2b070`).
const CAST_BLOCK_SECOND: u32 = 0x00e2_b510;
/// `hkTreeRayCastJobUtil::castRayBundleKdTree` (engine map; cdecl, nine words, as `00e2b070`).
const CAST_BLOCK_KD_TREE: u32 = 0x00e2_acf0;

/// The vtable the output structures start with.
const OUTPUT_VTABLES: [u32; 4] = [0x010d_baf8, 0x010d_bae4, 0x010d_bad8, 0x010d_bacc];
/// The vtables the output structures are left with.
const OUTPUT_VTABLES_AFTER: [u32; 4] = [0x010c_3bb8, 0x010c_cf8c, 0x010c_cfa0, 0x010c_cfac];
/// The vtable of the collector object.
pub(crate) const COLLECTOR_VTABLE: u32 = 0x010d_bb04;
/// Four floats `0` (compared with the ray direction).
const ZERO_DIRECTION: u32 = 0x0126_7e30;
/// The 16-byte constant that replaces a zero direction before the reciprocal.
const REPLACEMENT_DIRECTION: u32 = 0x010c_7170;
/// The 16-byte constant stored as the inverse of a zero direction.
const ZERO_INVERSE: u32 = 0x010c_d670;
/// The float the maximum hit fractions start at.
const FRACTION_LIMIT: u32 = 0x0101_2068;

/// Profiling label `"TtCpuWorldRayCast"`.
const LABEL_WORLD_RAY_CAST: u32 = 0x010d_bb0c;

const CAPACITY_MASK: u32 = 0x3fff_ffff;
const NOT_OWNED: u32 = 0x8000_0000;
/// 1.0f.
pub(crate) const ONE: u32 = 0x3f80_0000;

/// `RCPPS` of one lane (the host CPU's estimate on x86).
#[cfg(target_arch = "x86_64")]
fn rcp_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
fn rcp_estimate(x: f32) -> f32 {
    1.0 / x
}

/// The four words at `addr`, bit for bit.
fn load_words(e: &Engine, addr: u32) -> [u32; 4] {
    std::array::from_fn(|lane| e.mem.u32(addr + 4 * lane as u32))
}

/// The four words of `words` stored at `addr`.
fn store_words(e: &mut Engine, addr: u32, words: [u32; 4]) {
    for (lane, word) in words.into_iter().enumerate() {
        e.mem.set_u32(addr + 4 * lane as u32, word);
    }
}

/// The point `v` in the frame of the transform at `matrix` (rows at `+0`, `+0x10`, `+0x20` and the
/// translation at `+0x30`): `v - translation`, then the lanes
/// `(r0y*y + r0x*x) + r0z*z`, `(r1y*y + r1x*x) + r1z*z`, `(r2y*y + r2x*x) + r2z*z` and
/// `(r2w*y + r2y*x) + r2w*z` the shuffles of the vector code give.
pub(crate) fn into_transform_frame(e: &Engine, matrix: u32, point: Vec4) -> Vec4 {
    let rows = [
        load_vec(e, matrix),
        load_vec(e, matrix + 0x10),
        load_vec(e, matrix + 0x20),
    ];
    let translation = load_vec(e, matrix + 0x30);
    let v: Vec4 = std::array::from_fn(|lane| point[lane] - translation[lane]);
    let by_y = [rows[0][1], rows[1][1], rows[2][1], rows[2][3]];
    let by_x = [rows[0][0], rows[1][0], rows[2][0], rows[2][1]];
    let by_z = [rows[0][2], rows[1][2], rows[2][2], rows[2][3]];
    std::array::from_fn(|lane| (by_y[lane] * v[1] + by_x[lane] * v[0]) + by_z[lane] * v[2])
}

// Translated from 00df8960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFixedBufferRayHitCollector::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable
/// `0101e594` in the object and, when bit 0 of `flags` is set, gives the 0x1c-byte block back to
/// the memory router's allocator (slot 8). Returns `this`.
pub fn deleting_destructor_00df8960(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_e594);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00df8990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpFixedBufferRayHitCollector::addRayHit` (Xbox PDB): calls the function the global pointer
/// `0126ec04` holds with the two stack words and `this` (cdecl) and returns what it returns in
/// `EAX`.
pub fn hkp_fixed_buffer_ray_hit_collector_add_ray_hit(
    e: &mut Engine,
    this: u32,
    first: u32,
    second: u32,
) -> u32 {
    let target: u32 = e.global(ADD_RAY_HIT_HOOK);
    e.call(target, &args![first, second, this]).u32()
}

// Translated from 00df89b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: cdecl, four stack words: `owner`, the ray job `request`, the
/// `state` structure and the `collector` (or null). Records the request in the state (the request
/// in `+0x1C` and `+0x20`, the request words at `+0x30` in `+0x24` and `+0x28`, the one at
/// `+0x24` in `+0x50`, `[state + 0x10] + 0x10` in `+0x54` when the byte at `request + 0x20` is
/// set and that pointer is not null, else 0), and the collector in `+0x60`: when the byte at
/// `request + 0x3C` is set and the collector is not null, the collector gets 1.0 at `+4`, the
/// vtable `010dbb04` and the request's words `+0x30` and `+0x34` (twice) in `+8`, `+0xC` and
/// `+0x10`; and the hook `0126ec04` is set to `00e2b9b0` whenever the byte is set. Then the
/// output cell `[request + 0x30]` gets 1.0 at `+0x10`, `0xFFFFFFFF` at `+0x14` and `+0x20`, 0 at
/// `+0x40` and `+0x50`, and the function runs slot `0x5C` of `owner` (a block on its stack
/// with the 16 bytes at `request`, the count 1, the pointer `request + 0x10`, the stride 0x10 and 0,
/// then `state` and 0) and returns its `EAX`.
pub fn fn_00df89b0(e: &mut Engine, owner: u32, request: Ptr, state: Ptr, collector: Ptr) -> u32 {
    let (request, state, collector) = (request.addr(), state.addr(), collector.addr());
    e.with_stack(0x20, |e, block| {
        let block = block.addr();
        copy_vec(e, request, block);
        e.mem.set_u32(state + 0x20, request);
        let value = e.mem.u32(request + 0x24);
        e.mem.set_u32(state + 0x50, value);
        let cell = e.mem.u32(request + 0x30);
        e.mem.set_u32(state + 0x24, cell);
        e.mem.set_u32(state + 0x28, cell);
        e.mem.set_u32(state + 0x1c, request);
        e.mem.set_u32(block + 0x14, request + 0x10);
        e.mem.set_u32(block + 0x18, 0x10);
        e.mem.set_u32(block + 0x1c, 0);
        e.mem.set_u32(block + 0x10, 1);
        let heap = e.mem.u32(state + 0x10);
        let neighbour = if e.mem.u8(request + 0x20) != 0 && heap != 0 {
            heap + 0x10
        } else {
            0
        };
        e.mem.set_u32(state + 0x54, neighbour);
        if e.mem.u8(request + 0x3c) == 0 {
            e.mem.set_u32(state + 0x60, 0);
        } else if collector == 0 {
            e.mem.set_u32(state + 0x60, 0);
            e.set_global(ADD_RAY_HIT_HOOK, ADD_RAY_HIT_TARGET);
        } else {
            let (first, second) = (e.mem.u32(request + 0x30), e.mem.u32(request + 0x34));
            e.mem.set_u32(collector + 4, ONE);
            e.mem.set_u32(collector, COLLECTOR_VTABLE);
            e.mem.set_u32(collector + 8, first);
            e.mem.set_u32(collector + 0xc, first);
            e.mem.set_u32(collector + 0x10, second);
            e.mem.set_u32(collector + 0x14, 0);
            e.mem.set_u32(collector + 0x18, 0);
            e.mem.set_u32(state + 0x60, collector);
            e.set_global(ADD_RAY_HIT_HOOK, ADD_RAY_HIT_TARGET);
        }
        let cell = e.mem.u32(request + 0x30);
        e.mem.set_u32(cell + 0x10, ONE);
        e.mem.set_u32(cell + 0x14, 0xffff_ffff);
        e.mem.set_u32(cell + 0x20, 0xffff_ffff);
        e.mem.set_u32(cell + 0x50, 0);
        e.mem.set_u32(cell + 0x40, 0);
        e.vcall(owner, 0x5c, &args![block, state, 0u32]).u32()
    })
}

// Translated from 00df8aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = `this` (the ray cast state), two stack words: the address
/// of the output byte `out` and the `hit` entry (`[hit]` is the object that is cast against,
/// `[hit + 8]` the transform of its frame); returns `out`. Clears the byte; with a null object
/// that is all. Otherwise it asks the filter `[this + 0x10] + 0x14` (slot 4: the byte cell, the
/// ray `[this + 0x20]`, the entry) and stops when the byte it answers with is 0. It puts the
/// ray's two end points into the frame of the entry (`this + 0x30` and `+0x40`, the vector code
/// of `into_transform_frame`), sets up an output block on its stack (the fraction `[this + 0x14]` at
/// `+0x10`, `0xFFFFFFFF` at `+0x14` and `+0x20`, 0 at `+0x40`) and, with a collector at
/// `[this + 0x60]`, runs slot `0x24` of the object with the local ray, the entry and the collector,
/// copies the collector's count `+0x14` to the word `+0x38` of `[this + 0x1c]` and splats the
/// collector's fraction (`+4`) into the four words at `this + 0x70`. Without a collector it runs
/// slot `0x20` of the object (the byte cell, the local ray, the output block), stores its answer
/// in `out` and, on a hit, takes a 0x60-byte entry of the list at `[this + 0x24]`: the next free
/// one (`[this + 0x28]`, advancing it by 0x60 and the count `[[this + 0x1c] + 0x38]`) while the
/// count is below `[[this + 0x1c] + 0x34]`, otherwise the one with the greatest fraction (`+0x10`),
/// but only when that is greater than the new fraction. The entry gets the output block (the
/// normal rotated back into the world frame by the rows of the entry's transform) and the entry
/// pointer at `+0x50`; the new limit is stored in `[this + 0x14]` (1.0 when a free entry was
/// taken, else the greatest fraction; with the byte `+0x3D` of the list owner set the new
/// fraction is taken as 0, and with a capacity of 1 it replaces the limit), the byte
/// `[this + 0x18]` is set to 1 and the limit is splatted into `this + 0x70`. The exe copies 0x60
/// bytes out of a 0x50-byte block on its stack, so the last twelve bytes of the entry come from
/// stack residue beyond its frame; here they are zero. The byte cell at `f + 0x13` and the rest
/// of the frame are zero where the exe leaves them unwritten.
pub fn fn_00df8aa0(e: &mut Engine, this: Ptr, out: Ptr, hit: Ptr) -> Ptr {
    let (this, hit) = (this.addr(), hit.addr());
    e.mem.set_u8(out.addr(), 0);
    let object = e.mem.u32(hit);
    if object == 0 {
        return out;
    }
    e.with_stack(0x90, |e, frame| {
        let f = frame.addr();
        let cell = f + 0x13;
        let filter = e.mem.u32(this + 0x10) + 0x14;
        let ray = e.mem.u32(this + 0x20);
        let verdict = e.vcall(filter, 4, &args![cell, ray, hit]).u32();
        if e.mem.u8(verdict) == 0 {
            return;
        }
        let matrix = e.mem.u32(hit + 8);
        let current = x87_quiet_bits(e.mem.u32(this + 0x14));
        let start = into_transform_frame(e, matrix, load_vec(e, ray));
        store_vec(e, this + 0x30, start);
        e.mem.set_u32(f + 0x30, current);
        let end = into_transform_frame(e, matrix, load_vec(e, ray + 0x10));
        e.mem.set_u32(f + 0x34, 0xffff_ffff);
        e.mem.set_u32(f + 0x40, 0xffff_ffff);
        store_vec(e, this + 0x40, end);
        e.mem.set_u32(f + 0x60, 0);
        let collector = e.mem.u32(this + 0x60);
        if collector != 0 {
            e.vcall(object, 0x24, &args![this + 0x30, hit, collector]);
            let owner = e.mem.u32(this + 0x1c);
            let count = e.mem.u32(collector + 0x14);
            e.mem.set_u32(owner + 0x38, count);
            let fraction = e.mem.u32(collector + 4);
            store_words(e, this + 0x70, [fraction; 4]);
            return;
        }
        e.vcall(object, 0x20, &args![cell, this + 0x30, f + 0x20]);
        let found = e.mem.u8(cell);
        e.mem.set_u8(out.addr(), found);
        if found == 0 {
            return;
        }
        let owner = e.mem.u32(this + 0x1c);
        let count = e.mem.i32(owner + 0x38);
        let capacity = e.mem.i32(owner + 0x34);
        let base = e.mem.u32(this + 0x24);
        let (slot, limit) = if count < capacity {
            let slot = e.mem.u32(this + 0x28);
            e.mem.set_i32(owner + 0x38, count.wrapping_add(1));
            e.mem
                .set_u32(this + 0x28, e.mem.u32(this + 0x28).wrapping_add(0x60));
            (slot, ONE)
        } else {
            let mut greatest = base;
            let mut next = base.wrapping_add(0x60);
            for _ in 1..count {
                if e.mem.f32(greatest + 0x10) < e.mem.f32(next + 0x10) {
                    greatest = next;
                }
                next = next.wrapping_add(0x60);
            }
            if e.mem.f32(greatest + 0x10) > e.mem.f32(f + 0x30) {
                (greatest, x87_quiet_bits(e.mem.u32(greatest + 0x10)))
            } else {
                return;
            }
        };
        e.mem.set_u32(f + 0x18, limit);
        e.mem.set_u32(f + 0x14, slot);
        if slot == 0 {
            return;
        }
        let normal = load_vec(e, f + 0x20);
        let rows = [
            load_vec(e, matrix),
            load_vec(e, matrix + 0x10),
            load_vec(e, matrix + 0x20),
        ];
        let rotated: Vec4 = std::array::from_fn(|lane| {
            (normal[1] * rows[1][lane] + normal[0] * rows[0][lane]) + normal[2] * rows[2][lane]
        });
        store_vec(e, f + 0x20, rotated);
        for offset in 0..0x18 {
            let word = e.mem.u32(f + 0x20 + 4 * offset);
            e.mem.set_u32(slot + 4 * offset, word);
        }
        e.mem.set_u32(slot + 0x50, hit);
        let owner = e.mem.u32(this + 0x1c);
        if e.mem.u8(owner + 0x3d) != 0 {
            e.mem.set_u32(f + 0x30, 0);
        }
        let chosen = if e.mem.i32(owner + 0x34) == 1 {
            e.mem.u32(f + 0x30)
        } else {
            e.mem.u32(f + 0x18)
        };
        let limit = x87_quiet_bits(x87_quiet_bits(chosen));
        e.mem.set_u32(f + 0x18, limit);
        e.mem.set_u32(this + 0x14, limit);
        e.mem.set_u8(this + 0x18, 1);
        store_words(e, this + 0x70, [limit; 4]);
    });
    out
}

// Translated from 00df8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = `this`, two stack words: an entry whose signed byte at
/// `+5` is the offset to the entry `00df8aa0` works on, and one the code does not read (the exe
/// uses its stack slot as the output byte of the call, a scratch byte here). Runs `00df8aa0`
/// and returns the float at `this + 0x14`.
pub fn fn_00df8d60(e: &mut Engine, this: Ptr, entry: Ptr, _unused_1: u32) -> f32 {
    let offset = i32::from(e.mem.u8(entry.addr() + 5) as i8);
    let hit = entry.addr().wrapping_add(offset as u32);
    e.with_stack(4, |e, cell| {
        fn_00df8aa0(e, this, cell, Ptr::new(hit));
    });
    x87_quiet(e.mem.f32(this.addr() + 0x14))
}

// Translated from 00df8d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = the address 4 bytes after the ray cast state, two stack
/// words: the output byte `out` and the `hit` entry. Runs `00df8aa0` on the state and returns
/// `out`.
pub fn fn_00df8d90(e: &mut Engine, this: Ptr, out: Ptr, hit: Ptr) -> Ptr {
    fn_00df8aa0(e, Ptr::new(this.addr().wrapping_sub(4)), out, hit);
    out
}

// Translated from 00df8db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = the address 8 bytes after the ray cast state, one stack
/// word: an object whose word at `+0x20` is the entry. Runs `00df8aa0` on the state (the exe
/// uses the stack slot as the output byte, a scratch byte here) and returns the float at
/// `state + 0x14`, which is `this + 0xC`.
pub fn fn_00df8db0(e: &mut Engine, this: Ptr, object: Ptr) -> f32 {
    let hit = e.mem.u32(object.addr() + 0x20);
    let state = Ptr::new(this.addr().wrapping_sub(8));
    e.with_stack(4, |e, cell| {
        fn_00df8aa0(e, state, cell, Ptr::new(hit));
    });
    x87_quiet(e.mem.f32(this.addr() + 0xc))
}

// Translated from 00df8dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: `ECX` = the address 0xC bytes after the ray cast state, one stack
/// word: an object whose word at `+0xC` is the entry. Runs `00df8aa0` on the state (the exe
/// uses the stack slot as the output byte, a scratch byte here) and returns the float at
/// `state + 0x14`, which is `this + 8`.
pub fn fn_00df8dd0(e: &mut Engine, this: Ptr, object: Ptr) -> f32 {
    let hit = e.mem.u32(object.addr() + 0xc);
    let state = Ptr::new(this.addr().wrapping_sub(0xc));
    e.with_stack(4, |e, cell| {
        fn_00df8aa0(e, state, cell, Ptr::new(hit));
    });
    x87_quiet(e.mem.f32(this.addr() + 8))
}

/// The inverse direction of the ray from `from` to `to` the vector code of the world ray cast
/// job computes: `d = to - from`; lanes where `d` equals the constant at `01267e30` take the one
/// at `010c7170` (lane 3 always does), then `r = RCPPS` of that and `(r + r) - (r * x) * r`; the
/// lanes that were equal take `010cd670` instead.
fn inverse_direction(e: &Engine, from: u32, to: u32) -> [u32; 4] {
    let (from, to) = (load_vec(e, from), load_vec(e, to));
    let zero = load_words(e, ZERO_DIRECTION);
    let replacement = load_words(e, REPLACEMENT_DIRECTION);
    let infinite = load_words(e, ZERO_INVERSE);
    let direction: Vec4 = std::array::from_fn(|lane| to[lane] - from[lane]);
    let equal: [bool; 4] =
        std::array::from_fn(|lane| direction[lane] == f32::from_bits(zero[lane]));
    let divisor: Vec4 = std::array::from_fn(|lane| {
        if lane == 3 || equal[lane] {
            f32::from_bits(replacement[lane])
        } else {
            direction[lane]
        }
    });
    let estimate = divisor.map(rcp_estimate);
    std::array::from_fn(|lane| {
        if equal[lane] {
            infinite[lane]
        } else {
            let product = (estimate[lane] * divisor[lane]) * estimate[lane];
            ((estimate[lane] + estimate[lane]) - product).to_bits()
        }
    })
}

// Translated from 00df8df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkCpuWorldRayCastJob` (Xbox PDB name per the engine map): cdecl, two stack words: a word that
/// becomes `ECX` of the final call `00d04e80`, and the job. Writes `TtCpuWorldRayCast`.
/// World of the job: `[[job + 0x1c] + 0xC]`. Rays: `[job + 0x24]`, 0x40 bytes each, `[job + 0x28]`
/// of them (signed).
///
/// With 0 at `job + 0x34` it makes room on the router's stack allocator for an array of
/// 0x20-byte elements (`from` and the inverse direction of each ray, see `inverse_direction`)
/// and a 0x80-byte scratch block, builds four 0x80-byte output structures on its stack (the
/// four vtables, 1.0 at `+0x14`, the world at `+0x10`, the splat of the float at `01012068` at
/// `+0x70`), runs `00e2c8a0` and `00e2cc30` (words `+0x38` and `+0x3C` of the job, the array
/// data, the rays, the world, the first output structure, the scratch block and the count),
/// gives the stack back, leaves the output structures with their other vtables, frees the array
/// storage when it is owned (allocator slot `0x10`: the data, 32 bytes per capacity), then makes
/// a collector (vtable `010dbb04`, 1.0 at `+4`, 1 at `+0x10`) and runs `00e2abf0` for each ray,
/// and `00e2b070`, `00e2b510` and `00e2acf0` (words `+0x38`, `+0x3C`, `+0x40`) for each of the
/// `[job + 0x30]` blocks of 0x100 bytes at `[job + 0x2C]`, each with the splat vector in the
/// frame (the stack words the vector overlaps are the array's, which is dead by then).
///
/// With a nonzero `job + 0x34` it clears one output structure and a collector and, for each ray,
/// runs `00df89b0` with the object `[job + 0x34]`; then for each block with a count `K` at
/// `+0xC0` it builds a ray job per element (0x30 bytes at `+0` of the block: the two vectors, the
/// byte at `+0x20` and the word at `+0x24`; the output cell is the word at `+0xC8` plus 4 per
/// element, the byte `+0xF0` is the collector flag) on its stack, runs `00df89b0` and stores the
/// word `+0x38` of the ray job in the result words at `+0xE0` of the block.
///
/// Ends with the end record and `00d04e80` and returns what that returns. The frame words the exe
/// leaves unwritten are zero here.
pub fn hk_cpu_world_ray_cast_job(e: &mut Engine, second: u32, job: Ptr) -> u32 {
    profile_record(e, LABEL_WORLD_RAY_CAST);
    let job = job.addr();
    e.with_stack(0x340, |e, frame| {
        let b = frame.addr();
        if e.mem.u32(job + 0x34) == 0 {
            cast_bundles(e, job, b);
        } else {
            cast_one_by_one(e, job, b);
        }
    });
    profile_record(e, PROFILE_END);
    e.call(JOB_DONE, &args![second, job, job, 0u32]).u32()
}

/// The `job + 0x34 == 0` path of `hk_cpu_world_ray_cast_job` on the frame `b`.
fn cast_bundles(e: &mut Engine, job: u32, b: u32) {
    let owner = e.mem.u32(job + 0x1c);
    let world = e.mem.u32(owner + 0xc);
    let rays = e.mem.u32(job + 0x24);
    let count = e.mem.i32(job + 0x28);
    // The array of ray pairs (data, size, capacity) at b+0x20 and the stack top it started at.
    e.mem.set_u32(b + 0x14, world);
    e.mem.set_u32(b + 0x1c, rays);
    e.mem.set_u32(b + 0x20, 0);
    e.mem.set_u32(b + 0x24, 0);
    e.mem.set_u32(b + 0x28, NOT_OWNED);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    let bytes = ((count as u32) << 5).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, top.wrapping_add(bytes));
    e.mem.set_u32(b + 0x20, top);
    e.mem.set_u32(b + 0x28, (count as u32) | NOT_OWNED);
    e.mem.set_u32(b + 0x2c, top);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(b + 0x28) & CAPACITY_MASK) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE, &args![allocator, b + 0x20, wanted, 0x20u32]);
    }
    e.mem.set_u32(b + 0x24, count as u32);
    for k in 0..4 {
        let output = b + 0x130 + 0x80 * k;
        for (index, vtable) in OUTPUT_VTABLES.into_iter().enumerate() {
            e.mem.set_u32(output + 4 * index as u32, vtable);
        }
        e.mem.set_u32(output + 0x14, ONE);
        e.mem.set_u32(output + 0x50, 0);
        e.mem.set_u32(output + 0x54, 0);
        e.mem.set_u32(output + 0x60, 0);
        e.mem.set_u8(output + 0x18, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let scratch = e.mem.u32(router);
    e.mem.set_u32(router, scratch.wrapping_add(0x80));
    if count > 0 {
        for i in 0..count as u32 {
            let data = e.mem.u32(b + 0x20);
            let ray = rays + 0x40 * i;
            copy_vec(e, ray, data + 0x20 * i);
            let inverse = inverse_direction(e, ray, ray + 0x10);
            store_words(e, data + 0x20 * i + 0x10, inverse);
        }
    }
    let splat = e.mem.u32(FRACTION_LIMIT);
    store_words(e, b + 0x30, [splat; 4]);
    for k in 0..4 {
        let output = b + 0x130 + 0x80 * k;
        store_words(e, output + 0x70, [splat; 4]);
        e.mem.set_u32(output + 0x10, world);
    }
    let data = e.mem.u32(b + 0x20);
    let args = args![
        e.mem.u32(job + 0x38),
        data,
        rays,
        world,
        b + 0x130,
        scratch,
        count
    ];
    e.call(CAST_BUNDLES_FIRST, &args);
    let data = e.mem.u32(b + 0x20);
    let args = args![
        e.mem.u32(job + 0x3c),
        data,
        rays,
        world,
        b + 0x130,
        scratch,
        count
    ];
    e.call(CAST_BUNDLES_SECOND, &args);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, scratch);
    for k in (0..4).rev() {
        let output = b + 0x130 + 0x80 * k;
        for (index, vtable) in OUTPUT_VTABLES_AFTER.into_iter().enumerate() {
            e.mem.set_u32(output + 4 * index as u32, vtable);
        }
    }
    let saved_top = e.mem.u32(b + 0x2c);
    if saved_top == e.mem.u32(b + 0x20) {
        e.mem.set_u32(b + 0x24, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, saved_top);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(b + 0x28);
    e.mem.set_u32(b + 0x24, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(b + 0x20);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & CAPACITY_MASK) << 5],
        );
    }
    // The collector and the loop over the rays.
    let first_word = e.mem.u32(job + 0x40);
    e.mem.set_u32(b + 0x58, ONE);
    e.mem.set_u32(b + 0x18, first_word);
    e.mem.set_u32(b + 0x54, COLLECTOR_VTABLE);
    e.mem.set_u32(b + 0x5c, 0);
    e.mem.set_u32(b + 0x60, 0);
    e.mem.set_u32(b + 0x64, 1);
    e.mem.set_u32(b + 0x68, 0);
    e.mem.set_u32(b + 0x6c, 0);
    let rays = e.mem.u32(job + 0x24);
    let count = e.mem.i32(job + 0x28);
    if count > 0 {
        for i in 0..count as u32 {
            let args = args![
                e.mem.u32(b + 0x18),
                rays + 0x40 * i,
                world,
                b + 0x54,
                0u32,
                0u32,
                0u32,
                0u32
            ];
            e.call(CAST_ONE_RAY, &args);
        }
    }
    let blocks = e.mem.u32(job + 0x2c);
    let block_count = e.mem.i32(job + 0x30);
    if block_count > 0 {
        for i in 0..block_count as u32 {
            let block = blocks + 0x100 * i;
            let vector = load_words(e, b + 0x30);
            store_words(e, b + 0x20, vector);
            let word = e.mem.u32(job + 0x38);
            cast_block(e, CAST_BLOCK_FIRST, word, block, world, b + 0x20);
            let word = e.mem.u32(job + 0x3c);
            cast_block(e, CAST_BLOCK_SECOND, word, block, world, b + 0x20);
            let word = e.mem.u32(b + 0x18);
            cast_block(e, CAST_BLOCK_KD_TREE, word, block, world, b + 0x20);
        }
    }
}

/// One of the three block casts: `target` with the job word, the block, the world, the vector
/// and five zeros (cdecl).
fn cast_block(e: &mut Engine, target: u32, word: u32, block: u32, world: u32, vector: u32) {
    let args = args![word, block, world, vector, 0u32, 0u32, 0u32, 0u32, 0u32];
    e.call(target, &args);
}

/// The `job + 0x34 != 0` path of `hk_cpu_world_ray_cast_job` on the frame `b`.
fn cast_one_by_one(e: &mut Engine, job: u32, b: u32) {
    let owner = e.mem.u32(job + 0x1c);
    let world = e.mem.u32(owner + 0xc);
    let rays = e.mem.u32(job + 0x24);
    let state = b + 0xb0;
    let collector = b + 0x30;
    for (index, vtable) in OUTPUT_VTABLES.into_iter().enumerate() {
        e.mem.set_u32(state + 4 * index as u32, vtable);
    }
    e.mem.set_u32(state + 0x14, ONE);
    e.mem.set_u32(state + 0x50, 0);
    e.mem.set_u32(state + 0x54, 0);
    e.mem.set_u32(state + 0x60, 0);
    e.mem.set_u8(state + 0x18, 0);
    e.mem.set_u32(state + 0x10, world);
    e.mem.set_u32(collector, COLLECTOR_VTABLE);
    e.mem.set_u32(collector + 4, ONE);
    for offset in [8, 0xc, 0x14, 0x18] {
        e.mem.set_u32(collector + offset, 0);
    }
    e.mem.set_u32(collector + 0x10, 1);
    if e.mem.i32(job + 0x28) > 0 {
        let mut index = 0;
        let mut ray = rays;
        loop {
            e.mem.set_u32(state + 0x14, ONE);
            e.mem.set_u8(state + 0x18, 0);
            let owner = e.mem.u32(job + 0x34);
            fn_00df89b0(
                e,
                owner,
                Ptr::new(ray),
                Ptr::new(state),
                Ptr::new(collector),
            );
            index += 1;
            ray += 0x40;
            if index >= e.mem.i32(job + 0x28) {
                break;
            }
        }
    }
    let blocks = e.mem.u32(job + 0x2c);
    let block_count = e.mem.i32(job + 0x30);
    if block_count <= 0 {
        return;
    }
    // The request on the stack (b+0x70): two vectors, a byte and a word, the output cell, the
    // count 1 and the result word at +0x38.
    let request = b + 0x70;
    for i in 0..block_count as u32 {
        let counter = blocks + 0x100 * i + 0xc0;
        if e.mem.i32(counter) <= 0 {
            continue;
        }
        let mut results = counter + 0x20;
        let mut element = counter - 0xa0;
        let mut j = 0;
        loop {
            copy_vec(e, element - 0x20, request);
            copy_vec(e, element - 0x10, request + 0x10);
            let flag = e.mem.u8(element);
            let word = e.mem.u32(element + 4);
            e.mem.set_u8(b + 0x90, flag);
            e.mem.set_u32(b + 0x94, word);
            let cell = e.mem.u32(results - 0x18);
            e.mem.set_u32(request + 0x30, cell);
            let collect = e.mem.u8(counter + 0x30);
            e.mem.set_u32(request + 0x34, 1);
            e.mem.set_u32(request + 0x38, 0);
            e.mem.set_u8(request + 0x3c, collect);
            e.mem.set_u8(request + 0x3d, 0);
            e.mem.set_u32(state + 0x14, ONE);
            e.mem.set_u8(state + 0x18, 0);
            let owner = e.mem.u32(job + 0x34);
            fn_00df89b0(
                e,
                owner,
                Ptr::new(request),
                Ptr::new(state),
                Ptr::new(collector),
            );
            let result = e.mem.u32(request + 0x38);
            e.mem.set_u32(results, result);
            results += 4;
            j += 1;
            element += 0x30;
            if j >= e.mem.i32(counter) {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df8960, deleting_destructor_00df8960(Ptr, u8) -> Ptr),
        entry!(
            0x00df8990,
            hkp_fixed_buffer_ray_hit_collector_add_ray_hit(u32, u32, u32) -> u32
        ),
        entry!(0x00df89b0, fn_00df89b0(u32, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00df8aa0, fn_00df8aa0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00df8d60, fn_00df8d60(Ptr, Ptr, u32) -> f32),
        entry!(0x00df8d90, fn_00df8d90(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00df8db0, fn_00df8db0(Ptr, Ptr) -> f32),
        entry!(0x00df8dd0, fn_00df8dd0(Ptr, Ptr) -> f32),
        entry!(0x00df8df0, hk_cpu_world_ray_cast_job(u32, Ptr) -> u32),
    ]
}

/// Helpers the tests of the ray cast job units share.
#[cfg(test)]
pub(crate) mod test_support {
    use crate::prelude::*;

    /// An object whose vtable has the given `(slot, target)` pairs.
    pub(crate) fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    pub(crate) fn set_floats(e: &mut Engine, addr: u32, values: [f32; 4]) {
        for (lane, value) in values.into_iter().enumerate() {
            e.mem.set_f32(addr + 4 * lane as u32, value);
        }
    }

    pub(crate) fn get_floats(e: &Engine, addr: u32) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::{get_floats, object_with_slots, set_floats};
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, logging_object, new_log, Log,
    };
    use crate::units::havok_sdk::hkpcpuagentsectorjob::test_support::{job_rig, JobRig};

    const ALLOCATOR: u32 = 0x0900_0000;
    const FILTER_SLOT: u32 = 0x0900_1004;
    const CAST_SLOT: u32 = 0x0900_1020;
    const COLLECT_SLOT: u32 = 0x0900_1024;
    const OWNER_SLOT: u32 = 0x0900_105c;

    #[test]
    fn the_deleting_destructor_stores_the_vtable_and_frees_only_with_bit_zero() {
        let mut e = engine();
        let log = new_log();
        let allocator = logging_object(&mut e, &log, ALLOCATOR);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00df_8960, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0101_e594);
        assert!(calls(&log, ALLOCATOR + 8).is_empty());
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00df_8960, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x0101_e594);
        assert_eq!(
            calls(&log, ALLOCATOR + 8),
            vec![vec![allocator, this, 0x1c]]
        );
    }

    #[test]
    fn add_ray_hit_calls_the_hook_with_its_words_and_this_last() {
        let mut e = engine();
        let log = new_log();
        log_double(&mut e, &log, 0x00e2_b9b0, 0x42);
        e.set_global(ADD_RAY_HIT_HOOK, 0x00e2_b9b0u32);
        let got = e
            .call(0x00df_8990, &args![0x1000u32, 0x11u32, 0x22u32])
            .u32();
        assert_eq!(got, 0x42);
        assert_eq!(calls(&log, 0x00e2_b9b0), vec![vec![0x11, 0x22, 0x1000]]);
    }

    /// The pieces `fn_00df89b0` works on: the owner whose slot `0x5c` is a logging double, the
    /// request with the output cell `cell`, the state with a heap pointer and the collector.
    struct Prepare {
        e: Engine,
        log: Log,
        owner: u32,
        request: u32,
        state: u32,
        collector: u32,
        cell: u32,
    }

    fn prepare(neighbour_flag: u8, collector_flag: u8) -> Prepare {
        let mut e = engine();
        let log = new_log();
        let sink = log.clone();
        e.register_double(OWNER_SLOT, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..8).map(|word| e.mem.u32(a[1] + 4 * word)));
            sink.borrow_mut().push((OWNER_SLOT, words));
            Ret {
                eax: 0x77,
                ..Ret::default()
            }
        });
        let owner = object_with_slots(&mut e, &[(0x5c, OWNER_SLOT)]);
        let request = e.mem.alloc(0x40);
        set_floats(&mut e, request, [1.0, 2.0, 3.0, 4.0]);
        let cell = e.mem.alloc(0x60);
        e.mem.set_u32(request + 0x24, 0x24);
        e.mem.set_u8(request + 0x20, neighbour_flag);
        e.mem.set_u32(request + 0x30, cell);
        e.mem.set_u32(request + 0x34, 0x34);
        e.mem.set_u8(request + 0x3c, collector_flag);
        let state = e.mem.alloc(0x80);
        e.mem.set_u32(state + 0x10, 0x5000);
        let collector = e.mem.alloc(0x20);
        Prepare {
            e,
            log,
            owner,
            request,
            state,
            collector,
            cell,
        }
    }

    #[test]
    fn preparing_a_ray_with_a_collector_fills_state_collector_and_cell() {
        let mut p = prepare(1, 1);
        let got =
            p.e.call(
                0x00df_89b0,
                &args![p.owner, p.request, p.state, p.collector],
            )
            .u32();
        assert_eq!(got, 0x77);
        let e = &p.e;
        assert_eq!(e.mem.u32(p.state + 0x1c), p.request);
        assert_eq!(e.mem.u32(p.state + 0x20), p.request);
        assert_eq!(e.mem.u32(p.state + 0x50), 0x24);
        assert_eq!(e.mem.u32(p.state + 0x24), p.cell);
        assert_eq!(e.mem.u32(p.state + 0x28), p.cell);
        assert_eq!(e.mem.u32(p.state + 0x54), 0x5010);
        assert_eq!(e.mem.u32(p.state + 0x60), p.collector);
        assert_eq!(e.mem.u32(p.collector), COLLECTOR_VTABLE);
        assert_eq!(e.mem.u32(p.collector + 4), ONE);
        assert_eq!(e.mem.u32(p.collector + 8), p.cell);
        assert_eq!(e.mem.u32(p.collector + 0xc), p.cell);
        assert_eq!(e.mem.u32(p.collector + 0x10), 0x34);
        assert_eq!(e.global::<u32>(ADD_RAY_HIT_HOOK), ADD_RAY_HIT_TARGET);
        assert_eq!(e.mem.u32(p.cell + 0x10), ONE);
        assert_eq!(e.mem.u32(p.cell + 0x14), 0xffff_ffff);
        assert_eq!(e.mem.u32(p.cell + 0x20), 0xffff_ffff);
        assert_eq!(e.mem.u32(p.cell + 0x40), 0);
        assert_eq!(e.mem.u32(p.cell + 0x50), 0);
        // The block the owner was called with: the ray, the count, the end pointer and stride.
        let owner_calls = calls(&p.log, OWNER_SLOT);
        assert_eq!(owner_calls.len(), 1);
        let call = &owner_calls[0];
        assert_eq!(call[0], p.owner);
        assert_eq!(call[2], p.state);
        assert_eq!(call[3], 0);
        let ray: Vec<u32> = [1.0f32, 2.0, 3.0, 4.0]
            .iter()
            .map(|v| v.to_bits())
            .collect();
        assert_eq!(call[4..8], ray[..]);
        assert_eq!(call[8..12], [1, p.request + 0x10, 0x10, 0]);
    }

    #[test]
    fn preparing_without_a_collector_leaves_the_state_free_and_sets_the_hook_only_when_asked() {
        let mut p = prepare(0, 1);
        p.e.call(0x00df_89b0, &args![p.owner, p.request, p.state, 0u32]);
        assert_eq!(p.e.mem.u32(p.state + 0x54), 0);
        assert_eq!(p.e.mem.u32(p.state + 0x60), 0);
        assert_eq!(p.e.global::<u32>(ADD_RAY_HIT_HOOK), ADD_RAY_HIT_TARGET);
        p.e.set_global(ADD_RAY_HIT_HOOK, 0u32);
        p.e.mem.set_u8(p.request + 0x3c, 0);
        p.e.mem.set_u32(p.state + 0x60, 0x99);
        p.e.call(
            0x00df_89b0,
            &args![p.owner, p.request, p.state, p.collector],
        );
        assert_eq!(p.e.mem.u32(p.state + 0x60), 0);
        assert_eq!(p.e.global::<u32>(ADD_RAY_HIT_HOOK), 0);
        assert_eq!(p.e.mem.u32(p.collector), 0);
    }

    /// The ray cast state `fn_00df8aa0` works on: a filter that answers `verdict`, a hit entry
    /// whose object has slots `0x20` and `0x24` at `CAST_SLOT` and `COLLECT_SLOT`, a transform
    /// scaling by 2, 3 and 4 with translation 1, the ray from `(3, 5, 7)` to `(5, 9, 13)`, and
    /// a list of three entries with capacity 2 and count 0.
    struct Cast {
        e: Engine,
        log: Log,
        this: u32,
        out: u32,
        hit: u32,
        list: u32,
        entries: u32,
        object: u32,
    }

    fn cast_rig(object_present: bool, verdict: u8) -> Cast {
        let mut e = engine();
        let log = new_log();
        let answer = e.mem.alloc(4);
        e.mem.set_u8(answer, verdict);
        log_double(&mut e, &log, FILTER_SLOT, answer);
        let filter_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(filter_vtable + 4, FILTER_SLOT);
        let filter = e.mem.alloc(0x40);
        e.mem.set_u32(filter + 0x14, filter_vtable);
        let list = e.mem.alloc(0x50);
        e.mem.set_u32(list + 0x34, 2);
        let entries = e.mem.alloc(0x180);
        let matrix = e.mem.alloc(0x40);
        set_floats(&mut e, matrix, [2.0, 0.0, 0.0, 0.0]);
        set_floats(&mut e, matrix + 0x10, [0.0, 3.0, 0.0, 0.0]);
        set_floats(&mut e, matrix + 0x20, [0.0, 0.0, 4.0, 0.0]);
        set_floats(&mut e, matrix + 0x30, [1.0, 1.0, 1.0, 0.0]);
        let ray = e.mem.alloc(0x20);
        set_floats(&mut e, ray, [3.0, 5.0, 7.0, 0.0]);
        set_floats(&mut e, ray + 0x10, [5.0, 9.0, 13.0, 0.0]);
        let object = object_with_slots(&mut e, &[(0x20, CAST_SLOT), (0x24, COLLECT_SLOT)]);
        let hit = e.mem.alloc(0x10);
        e.mem.set_u32(hit, if object_present { object } else { 0 });
        e.mem.set_u32(hit + 8, matrix);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x10, filter);
        e.mem.set_f32(this + 0x14, 1.0);
        e.mem.set_u32(this + 0x1c, list);
        e.mem.set_u32(this + 0x20, ray);
        e.mem.set_u32(this + 0x24, entries);
        e.mem.set_u32(this + 0x28, entries);
        let out = e.mem.alloc(4);
        e.mem.set_u8(out, 0xff);
        Cast {
            e,
            log,
            this,
            out,
            hit,
            list,
            entries,
            object,
        }
    }

    /// Makes the cast of the object (slot `0x20`) answer `found` with the normal `(1, 2, 3)` and
    /// the new fraction `fraction`.
    fn answer_cast(c: &mut Cast, found: u8, fraction: f32) {
        let sink = c.log.clone();
        c.e.register_double(CAST_SLOT, move |e, a| {
            sink.borrow_mut().push((CAST_SLOT, a.to_vec()));
            e.mem.set_u8(a[1], found);
            set_floats(e, a[3], [1.0, 2.0, 3.0, 0.0]);
            e.mem.set_f32(a[3] + 0x10, fraction);
            e.mem.set_u32(a[3] + 0x14, 0x1234);
            Ret::default()
        });
    }

    fn set_entry_fraction(c: &mut Cast, index: u32, fraction: f32) {
        c.e.mem.set_f32(c.entries + 0x60 * index + 0x10, fraction);
    }

    #[test]
    fn a_null_object_only_clears_the_output_byte() {
        let mut c = cast_rig(false, 1);
        let got = c.e.call(0x00df_8aa0, &args![c.this, c.out, c.hit]).u32();
        assert_eq!(got, c.out);
        assert_eq!(c.e.mem.u8(c.out), 0);
        assert!(c.log.borrow().is_empty());
    }

    #[test]
    fn a_filter_that_refuses_stops_before_any_transform() {
        let mut c = cast_rig(true, 0);
        c.e.call(0x00df_8aa0, &args![c.this, c.out, c.hit]);
        assert_eq!(c.e.mem.u8(c.out), 0);
        let filter_calls = calls(&c.log, FILTER_SLOT);
        assert_eq!(filter_calls.len(), 1);
        assert_eq!(filter_calls[0][2..], [c.e.mem.u32(c.this + 0x20), c.hit]);
        assert_eq!(c.log.borrow().len(), 1);
        assert_eq!(c.e.mem.u32(c.this + 0x30), 0);
    }

    #[test]
    fn a_collector_receives_the_local_ray_and_its_count_and_fraction_are_taken() {
        let mut c = cast_rig(true, 1);
        let collector = c.e.mem.alloc(0x20);
        c.e.mem.set_f32(collector + 4, 0.5);
        c.e.mem.set_u32(collector + 0x14, 7);
        c.e.mem.set_u32(c.this + 0x60, collector);
        log_double(&mut c.e, &c.log.clone(), COLLECT_SLOT, 0);
        c.e.call(0x00df_8aa0, &args![c.this, c.out, c.hit]);
        let e = &c.e;
        assert_eq!(get_floats(e, c.this + 0x30), [4.0, 12.0, 24.0, 0.0]);
        assert_eq!(get_floats(e, c.this + 0x40), [8.0, 24.0, 48.0, 0.0]);
        assert_eq!(
            calls(&c.log, COLLECT_SLOT),
            vec![vec![c.object, c.this + 0x30, c.hit, collector]]
        );
        assert_eq!(e.mem.u32(c.list + 0x38), 7);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(c.this + 0x70 + 4 * lane), 0.5);
        }
        assert_eq!(e.mem.u8(c.out), 0);
    }

    #[test]
    fn a_miss_leaves_the_list_alone() {
        let mut c = cast_rig(true, 1);
        answer_cast(&mut c, 0, 0.5);
        c.e.call(0x00df_8aa0, &args![c.this, c.out, c.hit]);
        assert_eq!(c.e.mem.u8(c.out), 0);
        assert_eq!(c.e.mem.u32(c.list + 0x38), 0);
        assert_eq!(c.e.mem.u8(c.this + 0x18), 0);
        let cast = calls(&c.log, CAST_SLOT);
        assert_eq!(cast.len(), 1);
        assert_eq!(cast[0][2], c.this + 0x30);
    }

    #[test]
    fn a_hit_takes_the_next_free_entry_and_rotates_the_normal_back() {
        let mut c = cast_rig(true, 1);
        answer_cast(&mut c, 1, 0.5);
        c.e.call(0x00df_8aa0, &args![c.this, c.out, c.hit]);
        let e = &c.e;
        assert_eq!(e.mem.u8(c.out), 1);
        assert_eq!(e.mem.u32(c.list + 0x38), 1);
        assert_eq!(e.mem.u32(c.this + 0x28), c.entries + 0x60);
        // The normal (1, 2, 3) through the rows: 2 * (0, 3, 0) + (2, 0, 0) + 3 * (0, 0, 4).
        assert_eq!(get_floats(e, c.entries), [2.0, 6.0, 12.0, 0.0]);
        assert_eq!(e.mem.f32(c.entries + 0x10), 0.5);
        assert_eq!(e.mem.u32(c.entries + 0x14), 0x1234);
        assert_eq!(e.mem.u32(c.entries + 0x20), 0xffff_ffff);
        assert_eq!(e.mem.u32(c.entries + 0x50), c.hit);
        assert_eq!(e.mem.u32(c.entries + 0x54), 0);
        // The limit becomes 1.0 and is splatted.
        assert_eq!(e.mem.u32(c.this + 0x14), ONE);
        assert_eq!(e.mem.u8(c.this + 0x18), 1);
        for lane in 0..4 {
            assert_eq!(e.mem.u32(c.this + 0x70 + 4 * lane), ONE);
        }
    }

    #[test]
    fn a_full_list_replaces_its_greatest_fraction_only_when_the_new_one_is_smaller() {
        let mut c = cast_rig(true, 1);
        c.e.mem.set_u32(c.list + 0x38, 2);
        set_entry_fraction(&mut c, 0, 0.25);
        set_entry_fraction(&mut c, 1, 0.75);
        answer_cast(&mut c, 1, 0.5);
        c.e.call(0x00df_8aa0, &args![c.this, c.out, c.hit]);
        let e = &c.e;
        assert_eq!(e.mem.f32(c.entries + 0x10), 0.25);
        assert_eq!(e.mem.f32(c.entries + 0x60 + 0x10), 0.5);
        assert_eq!(e.mem.u32(c.entries + 0x60 + 0x50), c.hit);
        assert_eq!(e.mem.u32(c.list + 0x38), 2);
        // The limit is the fraction the replaced entry had.
        assert_eq!(e.mem.f32(c.this + 0x14), 0.75);
        assert_eq!(e.mem.u8(c.this + 0x18), 1);
    }

    #[test]
    fn a_full_list_keeps_everything_when_the_new_fraction_is_not_smaller() {
        let mut c = cast_rig(true, 1);
        c.e.mem.set_u32(c.list + 0x38, 2);
        set_entry_fraction(&mut c, 0, 0.25);
        set_entry_fraction(&mut c, 1, 0.75);
        answer_cast(&mut c, 1, 0.75);
        c.e.call(0x00df_8aa0, &args![c.this, c.out, c.hit]);
        let e = &c.e;
        assert_eq!(e.mem.u8(c.out), 1);
        assert_eq!(e.mem.f32(c.entries + 0x60 + 0x10), 0.75);
        assert_eq!(e.mem.u32(c.entries + 0x60 + 0x50), 0);
        assert_eq!(e.mem.f32(c.this + 0x14), 1.0);
        assert_eq!(e.mem.u8(c.this + 0x18), 0);
    }

    #[test]
    fn a_list_of_one_takes_the_new_fraction_as_the_limit_or_zero_when_flagged() {
        let mut c = cast_rig(true, 1);
        c.e.mem.set_u32(c.list + 0x34, 1);
        c.e.mem.set_u32(c.list + 0x38, 1);
        set_entry_fraction(&mut c, 0, 0.9);
        answer_cast(&mut c, 1, 0.5);
        c.e.call(0x00df_8aa0, &args![c.this, c.out, c.hit]);
        assert_eq!(c.e.mem.f32(c.this + 0x14), 0.5);
        assert_eq!(c.e.mem.f32(c.entries + 0x10), 0.5);
        c.e.mem.set_f32(c.entries + 0x10, 0.9);
        c.e.mem.set_u8(c.list + 0x3d, 1);
        c.e.call(0x00df_8aa0, &args![c.this, c.out, c.hit]);
        assert_eq!(c.e.mem.f32(c.this + 0x14), 0.0);
    }

    /// Checks that a wrapper over `00df8aa0` reached it with the state `this`, the entry `hit`
    /// and the output cell the filter saw as the first word.
    fn filter_saw_hit(c: &Cast, hit: u32) {
        let filter_calls = calls(&c.log, FILTER_SLOT);
        assert_eq!(filter_calls.len(), 1);
        assert_eq!(filter_calls[0][3], hit);
    }

    #[test]
    fn wrapper_00df8d60_follows_the_signed_offset_and_returns_the_limit() {
        let mut c = cast_rig(true, 0);
        c.e.mem.set_f32(c.this + 0x14, 0.25);
        let entry = c.e.mem.alloc(0x30);
        let target = entry + 0x10;
        c.e.mem.set_u8(target + 5, 0xf0);
        let hit = target.wrapping_sub(0x10);
        assert_eq!(hit, entry);
        c.e.mem.set_u32(entry, c.object);
        let got = c.e.call(0x00df_8d60, &args![c.this, target, 0u32]);
        assert_eq!(got.f32(), 0.25);
        filter_saw_hit(&c, entry);
    }

    #[test]
    fn wrapper_00df8d90_moves_this_back_four_and_returns_the_output_cell() {
        let mut c = cast_rig(true, 0);
        let got =
            c.e.call(0x00df_8d90, &args![c.this + 4, c.out, c.hit])
                .u32();
        assert_eq!(got, c.out);
        assert_eq!(c.e.mem.u8(c.out), 0);
        filter_saw_hit(&c, c.hit);
    }

    #[test]
    fn wrapper_00df8db0_takes_the_entry_from_word_0x20() {
        let mut c = cast_rig(true, 0);
        let holder = c.e.mem.alloc(0x40);
        c.e.mem.set_u32(holder + 0x20, c.hit);
        c.e.mem.set_f32(c.this + 0x14, 0.125);
        let got = c.e.call(0x00df_8db0, &args![c.this + 8, holder]);
        assert_eq!(got.f32(), 0.125);
        filter_saw_hit(&c, c.hit);
    }

    #[test]
    fn wrapper_00df8dd0_takes_the_entry_from_word_0xc() {
        let mut c = cast_rig(true, 0);
        let holder = c.e.mem.alloc(0x40);
        c.e.mem.set_u32(holder + 0xc, c.hit);
        c.e.mem.set_f32(c.this + 0x14, 0.0625);
        let got = c.e.call(0x00df_8dd0, &args![c.this + 0xc, holder]);
        assert_eq!(got.f32(), 0.0625);
        filter_saw_hit(&c, c.hit);
    }

    /// The memory of the world ray cast job: two rays, two blocks, a stack allocator whose top is
    /// the start of `top_block`, and doubles for everything the job calls (except `00df89b0`).
    struct World {
        e: Engine,
        rig: JobRig,
        job: u32,
        top_block: u32,
        router: u32,
        rays: u32,
        blocks: u32,
        allocator: u32,
        world: u32,
    }

    fn world_rig() -> World {
        let (mut e, rig) = job_rig();
        for lane in 0..4 {
            e.mem.set_u32(ZERO_DIRECTION + 4 * lane, 0);
            e.mem.set_f32(REPLACEMENT_DIRECTION + 4 * lane, 1.0);
            e.mem.set_u32(ZERO_INVERSE + 4 * lane, 0x7100_0000 + lane);
        }
        e.mem.set_f32(FRACTION_LIMIT, 3.0);
        let allocator = logging_object(&mut e, &rig.log, ALLOCATOR);
        let top_block = e.mem.alloc(0x400);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, top_block);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let rays = e.mem.alloc(0x80);
        set_floats(&mut e, rays, [1.0, 2.0, 3.0, 0.0]);
        set_floats(&mut e, rays + 0x10, [3.0, 2.0, 7.0, 0.0]);
        set_floats(&mut e, rays + 0x40, [0.0, 0.0, 0.0, 0.0]);
        set_floats(&mut e, rays + 0x50, [1.0, 1.0, 1.0, 0.0]);
        let blocks = e.mem.alloc(0x200);
        let world = 0x1111_0000;
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 0xc, world);
        let job = e.mem.alloc(0x60);
        for (offset, value) in [
            (0x1c, owner),
            (0x24, rays),
            (0x28, 2),
            (0x2c, blocks),
            (0x30, 2),
            (0x38, 0x138),
            (0x3c, 0x13c),
            (0x40, 0x140),
        ] {
            e.mem.set_u32(job + offset, value);
        }
        World {
            e,
            rig,
            job,
            top_block,
            router,
            rays,
            blocks,
            allocator,
            world,
        }
    }

    #[test]
    fn the_bundled_path_builds_the_inverse_directions_and_walks_the_rays_and_blocks() {
        let mut w = world_rig();
        // The first bundle cast logs its words, the array it was given and the four output
        // structures (vtables, world, fraction, splat vector) as they are at the call.
        let sink = w.rig.log.clone();
        w.e.register_double(CAST_BUNDLES_FIRST, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..16).map(|word| e.mem.u32(a[1] + 4 * word)));
            for k in 0..4 {
                let output = a[4] + 0x80 * k;
                words.extend((0..8).map(|word| e.mem.u32(output + 4 * word)));
                words.extend((0..4).map(|word| e.mem.u32(output + 0x70 + 4 * word)));
            }
            sink.borrow_mut().push((CAST_BUNDLES_FIRST, words));
            Ret::default()
        });
        log_double(&mut w.e, &w.rig.log, CAST_BUNDLES_SECOND, 0);
        let sink = w.rig.log.clone();
        w.e.register_double(CAST_ONE_RAY, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..7).map(|word| e.mem.u32(a[3] + 4 * word)));
            sink.borrow_mut().push((CAST_ONE_RAY, words));
            Ret::default()
        });
        for target in [CAST_BLOCK_FIRST, CAST_BLOCK_SECOND, CAST_BLOCK_KD_TREE] {
            let sink = w.rig.log.clone();
            w.e.register_double(target, move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..4).map(|word| e.mem.u32(a[3] + 4 * word)));
                sink.borrow_mut().push((target, words));
                Ret::default()
            });
        }
        let returned = w.e.call(0x00df_8df0, &args![0x99u32, w.job]).u32();
        w.rig.assert_finished(returned, 0x99, w.job);
        let first = &calls(&w.rig.log, CAST_BUNDLES_FIRST)[0];
        // The job word, the array data (the old stack top), the rays, the world, the first
        // output structure, the scratch block after the 0x80-byte array and the ray count.
        assert_eq!(first[0], 0x138);
        assert_eq!(first[1], w.top_block);
        assert_eq!(first[2], w.rays);
        assert_eq!(first[3], w.world);
        assert_eq!(first[5], w.top_block + 0x80);
        assert_eq!(first[6], 2);
        let element = |index: usize| &first[7 + 8 * index..7 + 8 * index + 8];
        let near = |bits: u32, wanted: f32| (f32::from_bits(bits) - wanted).abs() < 1e-5;
        assert_eq!(element(0)[..4], [1.0f32, 2.0, 3.0, 0.0].map(f32::to_bits));
        assert!(near(element(0)[4], 0.5));
        assert_eq!(element(0)[5], 0x7100_0001);
        assert!(near(element(0)[6], 0.25));
        assert_eq!(element(0)[7], 0x7100_0003);
        assert!(near(element(1)[4], 1.0) && near(element(1)[5], 1.0) && near(element(1)[6], 1.0));
        assert_eq!(element(1)[7], 0x7100_0003);
        for k in 0..4 {
            let at = 7 + 16 + 12 * k;
            assert_eq!(first[at..at + 4], OUTPUT_VTABLES);
            assert_eq!(first[at + 4], w.world);
            assert_eq!(first[at + 5], ONE);
            assert_eq!(first[at + 8..at + 12], [3.0f32.to_bits(); 4]);
        }
        // One cast per ray with the collector, the job word and four zeros.
        let rays = calls(&w.rig.log, CAST_ONE_RAY);
        assert_eq!(rays.len(), 2);
        for (index, ray) in rays.iter().enumerate() {
            assert_eq!(ray[..3], [0x140, w.rays + 0x40 * index as u32, w.world]);
            assert_eq!(ray[4..8], [0, 0, 0, 0]);
            assert_eq!(ray[8..], [COLLECTOR_VTABLE, ONE, 0, 0, 1, 0, 0]);
        }
        // Three casts per block, in order, each with the splat vector and five zeros.
        for (index, target) in [CAST_BLOCK_FIRST, CAST_BLOCK_SECOND, CAST_BLOCK_KD_TREE]
            .into_iter()
            .enumerate()
        {
            let block_calls = calls(&w.rig.log, target);
            assert_eq!(block_calls.len(), 2);
            for (block_index, call) in block_calls.iter().enumerate() {
                assert_eq!(call[0], [0x138, 0x13c, 0x140][index]);
                assert_eq!(call[1], w.blocks + 0x100 * block_index as u32);
                assert_eq!(call[2], w.world);
                assert_eq!(call[4..9], [0; 5]);
                assert_eq!(call[9..], [3.0f32.to_bits(); 4]);
            }
        }
        let casts: Vec<u32> = w
            .rig
            .log
            .borrow()
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| {
                [CAST_BLOCK_FIRST, CAST_BLOCK_SECOND, CAST_BLOCK_KD_TREE].contains(address)
            })
            .collect();
        assert_eq!(
            casts,
            vec![
                CAST_BLOCK_FIRST,
                CAST_BLOCK_SECOND,
                CAST_BLOCK_KD_TREE,
                CAST_BLOCK_FIRST,
                CAST_BLOCK_SECOND,
                CAST_BLOCK_KD_TREE
            ]
        );
        // The stack allocator is back where it started and nothing was freed.
        assert_eq!(w.e.mem.u32(w.router), w.top_block);
        assert!(calls(&w.rig.log, ALLOCATOR + 0x10).is_empty());
        assert_eq!(
            w.rig.labels(&w.e, false),
            vec![LABEL_WORLD_RAY_CAST, PROFILE_END]
        );
    }

    #[test]
    fn owned_array_storage_is_given_back_to_the_allocator() {
        let mut w = world_rig();
        w.e.mem.set_u32(w.job + 0x30, 0);
        // The second bundle cast makes the array owned (capacity 2, bit 31 clear).
        w.e.register_double(CAST_BUNDLES_SECOND, |e, a| {
            e.mem.set_u32(a[4] - 0x130 + 0x28, 2);
            Ret::default()
        });
        log_double(&mut w.e, &w.rig.log, CAST_BUNDLES_FIRST, 0);
        log_double(&mut w.e, &w.rig.log, CAST_ONE_RAY, 0);
        let returned = w.e.call(0x00df_8df0, &args![0x99u32, w.job]).u32();
        w.rig.assert_finished(returned, 0x99, w.job);
        assert_eq!(
            calls(&w.rig.log, ALLOCATOR + 0x10),
            vec![vec![w.allocator, w.top_block, 0x40]]
        );
        assert_eq!(w.e.mem.u32(w.router), w.top_block);
    }

    #[test]
    fn the_single_ray_path_casts_every_ray_and_every_block_element_through_the_owner() {
        let mut w = world_rig();
        let sink = w.rig.log.clone();
        let mut served = 0u32;
        w.e.register_double(OWNER_SLOT, move |e, a| {
            let request = e.mem.u32(a[2] + 0x1c);
            let mut words = a.to_vec();
            words.extend([
                request,
                u32::from(e.mem.u8(request + 0x20)),
                e.mem.u32(request + 0x24),
                e.mem.u32(request + 0x30),
                e.mem.u32(request + 0x34),
                e.mem.u32(request + 0x38),
                u32::from(e.mem.u8(request + 0x3c)),
            ]);
            words.extend((0..8).map(|word| e.mem.u32(request + 4 * word)));
            sink.borrow_mut().push((OWNER_SLOT, words));
            e.mem.set_u32(request + 0x38, 0x70 + served);
            served += 1;
            Ret::default()
        });
        let owner = object_with_slots(&mut w.e, &[(0x5c, OWNER_SLOT)]);
        w.e.mem.set_u32(w.job + 0x34, owner);
        let ray_cells = [w.e.mem.alloc(0x60), w.e.mem.alloc(0x60)];
        for (index, cell) in ray_cells.iter().enumerate() {
            w.e.mem.set_u32(w.rays + 0x40 * index as u32 + 0x30, *cell);
        }
        // Block 0 has two elements, block 1 none.
        let block = w.blocks;
        let element_cells = [w.e.mem.alloc(0x60), w.e.mem.alloc(0x60)];
        for (index, cell) in element_cells.iter().enumerate() {
            let index = index as u32;
            let element = block + 0x30 * index;
            set_floats(&mut w.e, element, [index as f32, 1.0, 2.0, 0.0]);
            set_floats(
                &mut w.e,
                element + 0x10,
                [index as f32 + 5.0, 6.0, 7.0, 0.0],
            );
            w.e.mem.set_u8(element + 0x20, 5 + index as u8);
            w.e.mem.set_u32(element + 0x24, 0xa0 + index);
            w.e.mem.set_u32(block + 0xc8 + 4 * index, *cell);
        }
        w.e.mem.set_u32(block + 0xc0, 2);
        w.e.mem.set_u8(block + 0xf0, 1);
        let returned = w.e.call(0x00df_8df0, &args![0x99u32, w.job]).u32();
        w.rig.assert_finished(returned, 0x99, w.job);
        let served = calls(&w.rig.log, OWNER_SLOT);
        assert_eq!(served.len(), 4);
        // Each call: the owner, the ray block, the state and 0, then the request words.
        for call in &served {
            assert_eq!(call[0], owner);
            assert_eq!(call[3], 0);
            assert_eq!(call[9], 0);
        }
        assert_eq!(served[0][4], w.rays);
        assert_eq!(served[1][4], w.rays + 0x40);
        assert_eq!(served[0][7], ray_cells[0]);
        assert_eq!(served[1][7], ray_cells[1]);
        // The element requests are built on the stack: the flag byte and word of the element,
        // the output cell of its slot, the count 1, the result word 0 and the collector flag.
        for (index, call) in served[2..].iter().enumerate() {
            let k = index as u32;
            assert_eq!(
                call[5..11],
                [5 + k, 0xa0 + k, element_cells[index], 1, 0, 1]
            );
            assert_eq!(f32::from_bits(call[11]), k as f32);
            assert_eq!(f32::from_bits(call[12]), 1.0);
            assert_eq!(f32::from_bits(call[15]), k as f32 + 5.0);
        }
        // The results go to the words at +0xE0 of the block, and the cells were reset.
        assert_eq!(w.e.mem.u32(block + 0xe0), 0x72);
        assert_eq!(w.e.mem.u32(block + 0xe4), 0x73);
        for cell in ray_cells.iter().chain(element_cells.iter()) {
            assert_eq!(w.e.mem.u32(*cell + 0x14), 0xffff_ffff);
        }
    }
}
