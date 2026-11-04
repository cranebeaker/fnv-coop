//! `(unplaced)/run_006271b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions sit inside `fallout shared/havok/loadedareabound.cpp`
//! (between `LoadedAreaBound`'s destructor `006270f0` and `WarpActor`
//! `006295e0`) but the Xbox PDB names no unit for them. They belong to the
//! `LoadedAreaBound` class: the layout is the one of
//! `units/fallout_shared_havok/loadedareabound.rs` (only `pCurrentCell` is
//! declared there), the other fields are read at their Xbox PDB offsets,
//! which the PC code confirms:
//! `+0x08` `spPhantomA[6]` (the six `bhkAabbPhantom` walls), `+0x20`
//! `pCurrentCell`, `+0x24` `HitMap` (an `NiTMap<bhkRigidBody *, unsigned
//! short>`), `+0x34` `fBoundOffset`, `+0x38` `fBoundDepth`, `+0x3c`
//! `fSideWallHeightOffset`, `+0x40` `fVelocityDampen`.
//!
//! Notes for the next session:
//! - This unit is complete after `006286a0`.
//! - The six phantoms are built around a box (an `hkAabb`: two 16-byte
//!   vectors, minimum then maximum) that comes from
//!   `006281d0`, the phantoms are rebuilt by `00627980` and the hit map is
//!   updated by `006271b0`.
//! - Locals the game keeps on its stack (arrays of the `00629xxx` family,
//!   the hit map, the construction info) are kept in one block per function
//!   (`with_stack`), at the offsets named in the constants below, and
//!   passed by address like the game does.
//! - The 16-byte vector operations (`00627920`, `006285e0`, `00628640`) are
//!   SSE (`MULPS`, `MINPS`, `MAXPS`): computed in `f32` per lane.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_havok::loadedareabound::LoadedAreaBound;

/// `LoadedAreaBound::spPhantomA` (Xbox PDB) `+0x08`: six `NiPointer`s.
const PHANTOMS: u32 = 0x08;
/// `LoadedAreaBound::HitMap` (Xbox PDB) `+0x24`.
const HIT_MAP: u32 = 0x24;
/// `LoadedAreaBound::fBoundOffset` (Xbox PDB) `+0x34`.
const BOUND_OFFSET: u32 = 0x34;
/// `LoadedAreaBound::fBoundDepth` (Xbox PDB) `+0x38`.
const BOUND_DEPTH: u32 = 0x38;
/// `LoadedAreaBound::fSideWallHeightOffset` (Xbox PDB) `+0x3c`.
const SIDE_WALL_HEIGHT_OFFSET: u32 = 0x3c;
/// `LoadedAreaBound::fVelocityDampen` (Xbox PDB) `+0x40`.
const VELOCITY_DAMPEN: u32 = 0x40;

/// `NiPointer::operator=(T *)` (`0066b0d0`, thiscall).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `*this` (`00559450`: the pointer an `NiPointer` holds), and the
/// identity function `this` (`006815c0`).
const NI_POINTER_GET: u32 = 0x0055_9450;
const IDENTITY: u32 = 0x0068_15c0;
/// Number of elements of an `hkArray`, `BSSimpleArray` or list: the word at
/// `+4` (`00726070`).
const WORD_AT_4: u32 = 0x0072_6070;
/// Is the cell an interior cell (`00425fd0`).
const IS_INTERIOR_CELL: u32 = 0x0042_5fd0;
/// `hkArray<..>` helpers of `loadedareabound.cpp`: constructor, `pushBack`
/// of the pointed-to word, `setSize`, element address, destructor.
const ARRAY_CONSTRUCT: u32 = 0x0062_99a0;
const ARRAY_PUSH_BACK: u32 = 0x0062_99c0;
const ARRAY_RESERVE: u32 = 0x0062_9a20;
const ARRAY_ELEMENT: u32 = 0x0062_9ab0;
const ARRAY_DESTRUCT: u32 = 0x0062_9b40;
const ARRAY_FIND: u32 = 0x0062_9f10;
/// `HitMap` helpers: constructor (hash size on the stack), insert, lookup
/// (key, `unsigned short *` result), next (iterator, key out, value out),
/// destructor, and `BSSimpleArray` helpers (constructor, `RemoveAt`).
const MAP_CONSTRUCT: u32 = 0x0062_99f0;
const MAP_SET: u32 = 0x0062_9c10;
const MAP_GET: u32 = 0x0062_9cf0;
const MAP_NEXT: u32 = 0x0062_9d70;
const MAP_DESTRUCT: u32 = 0x0062_9e50;
const MAP_FIRST: u32 = 0x004b_9ba0;
const MAP_REMOVE: u32 = 0x0040_5430;
const SIMPLE_ARRAY_CONSTRUCT: u32 = 0x0062_9fd0;
const SIMPLE_ARRAY_DESTRUCT: u32 = 0x0062_a000;
const SIMPLE_ARRAY_ADD: u32 = 0x007c_b2e0;
const SIMPLE_ARRAY_COUNT: u32 = 0x0044_ddc0;
const SIMPLE_ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// `LoadedAreaBound::WarpActor` (`006295e0`).
const WARP_ACTOR: u32 = 0x0062_95e0;
/// `hkVector4::set(x, y, z, w)` (`004b4d50`), `setAll(float)` (`004589c0`),
/// the address of component `index` (`00560d30`), the copy constructor of a
/// vector (`004a3f10`, and `004a3c90` from the address of a reference's
/// vector) and the constructor of an `hkAabb` (`00437c10`).
const VECTOR_SET: u32 = 0x004b_4d50;
const VECTOR_SET_ALL: u32 = 0x0045_89c0;
const VECTOR_COMPONENT: u32 = 0x0056_0d30;
const VECTOR_COPY: u32 = 0x004a_3f10;
const VECTOR_COPY_FROM_POINT: u32 = 0x004a_3c90;
const AABB_CONSTRUCT: u32 = 0x0043_7c10;

/// Constants of `006281d0`: the start values of the box (floats, set with
/// `setAll`) and the doubles its result compares them with; the 0.1
/// tolerance for a shape's box.
const MAX_START: u32 = 0x0104_e148;
const MIN_START: u32 = 0x0104_da48;
const MAX_UNCHANGED: u32 = 0x0104_e140;
const MIN_UNCHANGED: u32 = 0x0104_e138;
const SHAPE_TOLERANCE: u32 = 0x0101_e2bc;
/// The `double` `-1.0` the side walls scale their velocity with.
const MINUS_ONE: u32 = 0x0101_a6b0;

/// `hkAabb`: the minimum vector at `+0`, the maximum at `+0x10`.
const AABB_MAXIMUM: u32 = 0x10;

fn load_vector(e: &Engine, at: u32) -> [f32; 4] {
    [
        e.mem.f32(at),
        e.mem.f32(at + 4),
        e.mem.f32(at + 8),
        e.mem.f32(at + 12),
    ]
}

fn store_vector(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (lane, v) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * lane as u32, v);
    }
}

// Translation incomplete: 006271b0 - C++ exception unwinding frame (FS:[0]) and /GS cookie check (00ec408c) not translated
// Translated from 006271b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedAreaBound` update for a new current cell (called by
/// `UpdateCurrentGridCell`): `grid` is the exterior's `GridCellArray` (or
/// null). When `cell` is neither the current cell nor null the box of the
/// area is computed (`006281d0`); if that finds nothing the function ends,
/// otherwise the six phantoms are rebuilt (`00627980`), the cell becomes the
/// current one and `006286f0` runs. A null cell clears the current cell and
/// runs `00628da0`. Otherwise, for each of the six phantoms (bit `1 << i` of
/// a hit mask): the bodies it overlaps are split into those with a
/// collidable reference (`004b59f0`) and the others (`004b5950`); for the
/// first kind, when the cell is an interior or this is wall 0, actors
/// (form type `0x1e`) get `WarpActor`; every overlapped body that is not of
/// type 4, 5 or 7 is either given the hit bit (and, if it was not hit in
/// the last update, a velocity pushing it back in) or, for the interior
/// case, replaced by the bodies `00628f90` finds. Afterwards the new hit
/// map replaces the old one and bodies no longer hit are removed.
///
/// Locals (constructed on the game's stack): the `hkAabb` at `+0x00`, the
/// new hit map `+0x30`, the arrays at `+0x40` (bodies without reference),
/// `+0x50` (bodies with one), `+0x60` (already handled) and `+0x70` (found
/// by `00628f90`), the `BSSimpleArray` at `+0x80`, and the scratch words.
pub fn fn_006271b0(e: &mut Engine, this: Ptr<LoadedAreaBound>, cell: Ptr, grid: Ptr) {
    e.with_stack(0x100, |e, frame| {
        let f = frame.addr();
        let bounds = f;
        let new_hits = f + 0x30;
        let plain = f + 0x40;
        let referenced = f + 0x50;
        let handled = f + 0x60;
        let found = f + 0x70;
        let removed = f + 0x80;
        let handle = f + 0x90;
        let flag_byte = f + 0x94;
        let out_value = f + 0x98;
        let id_cell = f + 0xa0;
        let key_out = f + 0xc0;
        let value_out = f + 0xc4;
        let iterator = f + 0xc8;
        let body_cell = f + 0xd0;
        let vector_a = f + 0xe0;
        let vector_b = f + 0xf0;
        let base = this.addr();

        let current = e.get(this, LoadedAreaBound::pCurrentCell);
        if cell != current && !cell.is_null() {
            e.call(AABB_CONSTRUCT, &args![bounds]);
            if !fn_006281d0(e, this, cell, grid, Ptr::new(bounds)) {
                return;
            }
            fn_00627980(e, this, cell, Ptr::new(bounds));
            e.set(this, LoadedAreaBound::pCurrentCell, cell);
            e.call(0x0062_86f0, &args![this]);
        }
        if cell.is_null() {
            e.set(this, LoadedAreaBound::pCurrentCell, cell);
            e.call(0x0062_8da0, &args![this]);
            return;
        }

        let mut mask: u16 = 1;
        e.call(MAP_CONSTRUCT, &args![new_hits, 0x25u32]);
        for wall in 0..6u32 {
            let current = e.get(this, LoadedAreaBound::pCurrentCell);
            let interior = e.call(IS_INTERIOR_CELL, &args![current]).bool();
            let warp_actors = interior || wall == 0;
            let phantom = e
                .call(NI_POINTER_GET, &args![base + PHANTOMS + wall * 4])
                .u32();
            let listener = e.call(0x0062_0b80, &args![phantom]).u32();
            let overlaps = e.call(0x0045_c650, &args![listener]).u32();
            let query = e.call(0x0062_e120, &args![overlaps, flag_byte]).u32();
            if !e.call(0x0056_09f0, &args![query]).bool() {
                let count = e.call(WORD_AT_4, &args![overlaps]).i32();
                e.call(ARRAY_CONSTRUCT, &args![plain]);
                e.call(ARRAY_CONSTRUCT, &args![referenced]);
                let size = e.call(WORD_AT_4, &args![overlaps]).u32();
                e.call(ARRAY_RESERVE, &args![referenced, size]);
                for index in 0..count {
                    let element = e.call(ARRAY_ELEMENT, &args![overlaps, index]).u32();
                    let body = e.mem.u32(element);
                    let with_reference = e.call(0x004b_59f0, &args![body]).u32();
                    e.mem.set_u32(id_cell, with_reference);
                    if with_reference != 0 {
                        e.call(ARRAY_PUSH_BACK, &args![referenced, id_cell]);
                    } else {
                        let element = e.call(ARRAY_ELEMENT, &args![overlaps, index]).u32();
                        let body = e.mem.u32(element);
                        let other = e.call(0x004b_5950, &args![body]).u32();
                        e.mem.set_u32(id_cell + 4, other);
                        if other != 0 {
                            e.call(ARRAY_PUSH_BACK, &args![plain, id_cell + 4]);
                        }
                    }
                }

                let mut index = 0;
                while index < e.call(WORD_AT_4, &args![plain]).i32() {
                    if warp_actors {
                        let element = e.call(ARRAY_ELEMENT, &args![plain, index]).u32();
                        let reference = e.mem.u32(element);
                        let form = e.call(0x0046_0140, &args![reference]).u32();
                        let form_type_source = e.call(0x0043_b540, &args![form]).u32();
                        e.call(0x008c_71b0, &args![handle, form_type_source]);
                        let element = e.call(ARRAY_ELEMENT, &args![plain, index]).u32();
                        let reference = e.mem.u32(element);
                        let form = e.call(0x0046_0140, &args![reference]).u32();
                        let actor = e.call(0x0062_b4e0, &args![form]).u32();
                        let form_type = e.call(0x0043_b4d0, &args![handle]).i32();
                        if form_type == 0x1e && actor != 0 && e.vcall(actor, 0x100, &args![]).bool()
                        {
                            e.call(WARP_ACTOR, &args![this, actor]);
                        }
                    }
                    index += 1;
                }

                e.call(ARRAY_CONSTRUCT, &args![handled]);
                let mut index = 0;
                while index < e.call(WORD_AT_4, &args![referenced]).i32() {
                    let element = e.call(ARRAY_ELEMENT, &args![referenced, index]).u32();
                    let reference = e.mem.u32(element);
                    e.mem.set_u32(body_cell, reference);
                    let seen = e
                        .call(ARRAY_FIND, &args![handled, body_cell, 0u32, -1i32])
                        .i32();
                    if seen == -1 {
                        let form = e.call(0x0046_0140, &args![reference]).u32();
                        let _collidable = e.call(0x0062_b4e0, &args![form]).u32();
                        if reference != 0
                            && !matches!(e.call(0x0051_7670, &args![reference]).i32(), 4 | 5 | 7)
                        {
                            let body = e.call(0x004b_5a80, &args![reference]).u32();
                            if !warp_actors {
                                let found_hit = e
                                    .call(MAP_GET, &args![base + HIT_MAP, body, out_value])
                                    .bool();
                                let hit_before = e.mem.u16(out_value);
                                if !(found_hit && hit_before & mask != 0) {
                                    push_body_back(e, this, wall, body, vector_a, vector_b);
                                }
                                e.mem.set_u16(out_value, mask);
                                if e.call(MAP_GET, &args![new_hits, body, out_value]).bool() {
                                    let hits = e.mem.u16(out_value) | mask;
                                    e.mem.set_u16(out_value, hits);
                                }
                                let hits = e.mem.u16(out_value);
                                e.call(MAP_SET, &args![new_hits, body, hits as u32]);
                            } else {
                                e.call(ARRAY_CONSTRUCT, &args![found]);
                                e.call(0x0062_8f90, &args![this, body, found]);
                                let mut found_index = 0;
                                while found_index < e.call(WORD_AT_4, &args![found]).i32() {
                                    let item =
                                        e.call(ARRAY_ELEMENT, &args![found, found_index]).u32();
                                    e.call(ARRAY_PUSH_BACK, &args![handled, item]);
                                    found_index += 1;
                                }
                                e.call(ARRAY_DESTRUCT, &args![found]);
                            }
                        }
                    }
                    index += 1;
                }
                e.call(ARRAY_DESTRUCT, &args![handled]);
                e.call(ARRAY_DESTRUCT, &args![referenced]);
                e.call(ARRAY_DESTRUCT, &args![plain]);
            }
            mask <<= 1;
        }

        // The new hit map replaces the old one.
        let first = e.call(MAP_FIRST, &args![new_hits]).u32();
        e.mem.set_u32(iterator, first);
        while e.mem.u32(iterator) != 0 {
            e.call(MAP_NEXT, &args![new_hits, iterator, key_out, value_out]);
            let key = e.mem.u32(key_out);
            let value = e.mem.u16(value_out);
            e.call(MAP_SET, &args![base + HIT_MAP, key, value as u32]);
        }
        e.call(SIMPLE_ARRAY_CONSTRUCT, &args![removed]);
        let first = e.call(MAP_FIRST, &args![base + HIT_MAP]).u32();
        e.mem.set_u32(iterator, first);
        while e.mem.u32(iterator) != 0 {
            e.call(
                MAP_NEXT,
                &args![base + HIT_MAP, iterator, key_out, value_out],
            );
            let key = e.mem.u32(key_out);
            if !e.call(MAP_GET, &args![new_hits, key, value_out]).bool() {
                e.call(SIMPLE_ARRAY_ADD, &args![removed, key_out]);
            }
        }
        let mut index = 0u32;
        while index < e.call(SIMPLE_ARRAY_COUNT, &args![removed]).u32() {
            let element = e.call(SIMPLE_ARRAY_ELEMENT, &args![removed, index]).u32();
            let key = e.mem.u32(element);
            e.call(MAP_REMOVE, &args![base + HIT_MAP, key]);
            index += 1;
        }
        e.call(SIMPLE_ARRAY_DESTRUCT, &args![removed]);
        e.call(MAP_DESTRUCT, &args![new_hits]);
    })
}

/// The block of `006271b0` that gives a body touching a side wall a
/// velocity pushing it back: the body's linear velocity is scaled by the
/// wall's damping (negated, except for wall 1) and its vertical component
/// negated, then stored.
fn push_body_back(
    e: &mut Engine,
    this: Ptr<LoadedAreaBound>,
    wall: u32,
    body: u32,
    velocity: u32,
    scale: u32,
) {
    let velocity_source = e.call(0x0056_0d80, &args![body]).u32();
    e.call(VECTOR_COPY_FROM_POINT, &args![velocity, velocity_source]);
    let dampen = e.mem.f32(this.addr() + VELOCITY_DAMPEN);
    let minus_one: f64 = e.global(MINUS_ONE);
    let factor = if wall != 1 {
        dampen as f64 * minus_one
    } else {
        dampen as f64
    } as f32;
    e.call(0x005d_bf20, &args![scale, factor]);
    fn_00627920(e, Ptr::new(velocity), Ptr::new(scale));
    let component = e.call(VECTOR_COMPONENT, &args![velocity, 2u32]).u32();
    let z = e.mem.f32(component);
    e.mem.set_f32(component, (z as f64 * minus_one) as f32);
    e.call(0x0056_1690, &args![body, velocity]);
}

// Translation incomplete: 00627920 - /GS stack cookie check (00ec408c) not translated
// Translated from 00627920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Multiplies the 4-lane vector at `this` by the vector whose lanes all
/// equal the first lane of the vector at `other` (`00561240` builds that
/// vector; `MULPS`).
pub fn fn_00627920(e: &mut Engine, this: Ptr, other: Ptr) {
    let value = load_vector(e, this.addr());
    e.with_stack(0x20, |e, splat_at| {
        let splat = e.call(0x0056_1240, &args![other, splat_at]).u32();
        let factor = load_vector(e, splat);
        let product = [
            factor[0] * value[0],
            factor[1] * value[1],
            factor[2] * value[2],
            factor[3] * value[3],
        ];
        store_vector(e, this.addr(), product);
    });
}

// Translation incomplete: 00627980 - C++ exception unwinding frame (FS:[0]) and /GS cookie check (00ec408c) not translated
// Translated from 00627980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the six wall phantoms of the `LoadedAreaBound` around `bounds`
/// (the `hkAabb` found by `006281d0`: minimum at `+0`, maximum at `+0x10`).
/// For each wall `i`: the old `NiPointer` is released; two corners are
/// computed from the box, `fBoundOffset` (`W`), `fBoundDepth` (`D`) and the
/// side wall height offset `H` (`0` for an interior cell, else
/// `fSideWallHeightOffset`):
/// each wall gets its own pair of corners from the minimum (`min`) and
/// maximum (`max`) of the box, for example wall 0 spans `min.z - W` to
/// `min.z - D` in z and `min.xy - D` to `max.xy + D`; see the match below.
/// Each sum is x87 code, one rounding per `FADD`/`FSUB` at the running
/// thread's precision. The corners go into the construction info's `hkAabb`; the filter
/// word is `00622290(word, 0, 005dbe10(), 0)`; the `bhkAabbPhantom` (`0x18` bytes,
/// `0056e550`) is stored in the `NiPointer` and added to the world
/// (`0056d2c0`) the cell belongs to (`004543c0`).
pub fn fn_00627980(e: &mut Engine, this: Ptr<LoadedAreaBound>, cell: Ptr, bounds: Ptr) {
    let world = e.call(0x0045_43c0, &args![cell]).u32();
    let base = this.addr();
    e.with_stack(0x180, |e, frame| {
        let f = frame.addr();
        let info = f; // 0x60 bytes: construction info (aabb at +0x20)
        let aabb = f + 0x60;
        let corner_a = f + 0x80;
        let corner_b = f + 0x90;
        let filter = f + 0xa0;
        for wall in 0..6u32 {
            e.call(NI_POINTER_ASSIGN, &args![base + PHANTOMS + wall * 4, 0u32]);
            e.call(0x0056_e4e0, &args![info]);
            e.call(AABB_CONSTRUCT, &args![aabb]);
            e.call(IDENTITY, &args![corner_a]);
            e.call(IDENTITY, &args![corner_b]);
            let interior = e.call(IS_INTERIOR_CELL, &args![cell]).bool();
            // 00627a36 / 00627a40: the height local (`FLDZ`, or `FLD [this + 0x3c]`)
            // stored with `FSTP` and copied once more (00627a4c) before the walls read it.
            let height = if interior {
                0
            } else {
                x87_quiet_bits(e.mem.u32(base + SIDE_WALL_HEIGHT_OFFSET))
            };

            // 00627a7b..0062807d: each coordinate is `FLD` of a box component, then one
            // `FADD`/`FSUB` per term in the exe's order with the `float` operand
            // `[this + 0x34]` (`W`), `[this + 0x38]` (`D`) or the height (`H`), each
            // rounded at the running thread's x87 precision, then `FSTP` to a local
            // that is pushed as the `float` argument.
            let min = |e: &mut Engine, axis: u32| {
                let p = e.call(VECTOR_COMPONENT, &args![bounds, axis]).u32();
                x87_load32(e.mem.u32(p))
            };
            let max = |e: &mut Engine, axis: u32| {
                let p = e
                    .call(VECTOR_COMPONENT, &args![bounds.addr() + AABB_MAXIMUM, axis])
                    .u32();
                x87_load32(e.mem.u32(p))
            };
            let w = |e: &mut Engine| x87_m32(e.mem.u32(base + BOUND_OFFSET));
            let d = |e: &mut Engine| x87_m32(e.mem.u32(base + BOUND_DEPTH));
            let h = x87_m32(height);
            let set = |e: &mut Engine, corner: u32, x: f64, y: f64, z: f64| {
                let (x, y, z) = (x87_store32(x), x87_store32(y), x87_store32(z));
                e.call(VECTOR_SET, &args![corner, x, y, z, 0.0f32]);
            };
            // The calls read the box in the order the game does: z, y, x.
            match wall {
                0 => {
                    let z = x87_sub(min(e, 2), w(e));
                    let y = x87_add(max(e, 1), d(e));
                    let x = x87_add(max(e, 0), d(e));
                    set(e, corner_a, x, y, z);
                    let z = x87_sub(min(e, 2), d(e));
                    let y = x87_sub(min(e, 1), d(e));
                    let x = x87_sub(min(e, 0), d(e));
                    set(e, corner_b, x, y, z);
                }
                1 => {
                    let z = x87_add(x87_add(max(e, 2), d(e)), h);
                    let y = x87_add(max(e, 1), d(e));
                    let x = x87_add(max(e, 0), d(e));
                    set(e, corner_a, x, y, z);
                    let z = x87_add(x87_add(max(e, 2), h), w(e));
                    let y = x87_sub(min(e, 1), d(e));
                    let x = x87_sub(min(e, 0), d(e));
                    set(e, corner_b, x, y, z);
                }
                2 => {
                    let z = x87_add(x87_add(max(e, 2), h), w(e));
                    let y = x87_sub(min(e, 1), w(e));
                    let x = x87_add(max(e, 0), w(e));
                    set(e, corner_a, x, y, z);
                    let z = x87_sub(min(e, 2), w(e));
                    let y = x87_sub(min(e, 1), d(e));
                    let x = x87_sub(min(e, 0), w(e));
                    set(e, corner_b, x, y, z);
                }
                3 => {
                    let z = x87_add(x87_add(max(e, 2), h), w(e));
                    let y = x87_add(max(e, 1), d(e));
                    let x = x87_sub(min(e, 0), w(e));
                    set(e, corner_a, x, y, z);
                    let z = x87_sub(min(e, 2), w(e));
                    let y = x87_sub(min(e, 1), d(e));
                    let x = x87_sub(min(e, 0), d(e));
                    set(e, corner_b, x, y, z);
                }
                4 => {
                    let z = x87_add(x87_add(max(e, 2), h), w(e));
                    let y = x87_add(max(e, 1), d(e));
                    let x = x87_add(max(e, 0), d(e));
                    set(e, corner_a, x, y, z);
                    let z = x87_sub(min(e, 2), w(e));
                    let y = x87_sub(min(e, 1), d(e));
                    let x = x87_add(max(e, 0), w(e));
                    set(e, corner_b, x, y, z);
                }
                _ => {
                    let z = x87_add(x87_add(max(e, 2), h), w(e));
                    let y = x87_add(max(e, 1), d(e));
                    let x = x87_add(max(e, 0), w(e));
                    set(e, corner_a, x, y, z);
                    let z = x87_sub(min(e, 2), w(e));
                    let y = x87_add(max(e, 1), w(e));
                    let x = x87_sub(min(e, 0), w(e));
                    set(e, corner_b, x, y, z);
                }
            }

            // The construction info's box: maximum corner, then minimum.
            e.call(VECTOR_COPY, &args![aabb + AABB_MAXIMUM, corner_a]);
            e.call(VECTOR_COPY, &args![aabb, corner_b]);
            e.call(0x0062_81a0, &args![info + 0x20, aabb]);
            let seed = e.call(0x005d_be10, &args![]).u32();
            e.call(0x0062_2290, &args![filter, 0u32, seed, 0u32]);
            let first = e.call(NI_POINTER_GET, &args![filter]).u32();
            e.mem.set_u32(info, first);
            let memory = e.call(0x00aa_13e0, &args![0x18u32]).u32();
            let phantom = if memory == 0 {
                0
            } else {
                e.call(0x0056_e550, &args![memory, info]).u32()
            };
            e.call(
                NI_POINTER_ASSIGN,
                &args![base + PHANTOMS + wall * 4, phantom],
            );
            let stored = e
                .call(NI_POINTER_GET, &args![base + PHANTOMS + wall * 4])
                .u32();
            e.call(0x0056_d2c0, &args![world, stored]);
            e.call(0x0056_d730, &args![info]);
        }
    })
}

// Translated from 006281a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 32-byte `hkAabb` at `source` (two vectors) into `this` and
/// returns `this`.
pub fn fn_006281a0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(VECTOR_COPY, &args![this, source]);
    e.call(
        VECTOR_COPY,
        &args![this.addr() + AABB_MAXIMUM, source.addr() + AABB_MAXIMUM],
    );
    this
}

// Translation incomplete: 006281d0 - /GS cookie check (00ec408c) not translated
// Translated from 006281d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes the box (an `hkAabb` written to `out`: minimum, then maximum)
/// enclosing everything in the cell `cell`: starting from an empty box
/// (`+FLT_MAX` / `-FLT_MAX`), for an interior cell every reference of the
/// cell's list (`00624700`) with a loaded object contributes its bounds
/// (`007d6bb0`, the object + 0x14, handed with the box to the virtual at slot
/// `0x3c` of `006286d0`'s object); for an exterior cell (when `grid` is given) every loaded
/// object of every grid cell contributes, through the virtual
/// `0x94`-selected path (its own bounds, or its shape's box widened by the
/// `0.1` tolerance, `006286a0`). Returns false when the box stayed empty.
pub fn fn_006281d0(
    e: &mut Engine,
    this: Ptr<LoadedAreaBound>,
    cell: Ptr,
    grid: Ptr,
    out: Ptr,
) -> bool {
    let _ = this;
    let world = e.call(0x0045_43c0, &args![cell]).u32();
    let root = if world == 0 {
        0
    } else {
        e.call(0x0054_1c80, &args![world]).u32()
    };
    if root == 0 {
        return false;
    }
    e.with_stack(0xa0, |e, frame| {
        let f = frame.addr();
        let aabb = f;
        let min_total = f + 0x20;
        let max_total = f + 0x30;
        let transform = f + 0x40;
        e.call(AABB_CONSTRUCT, &args![aabb]);
        e.call(IDENTITY, &args![min_total]);
        e.call(IDENTITY, &args![max_total]);
        let max_start = e.global::<f32>(MAX_START);
        e.call(VECTOR_SET_ALL, &args![max_total, max_start]);
        let min_start = e.global::<f32>(MIN_START);
        e.call(VECTOR_SET_ALL, &args![min_total, min_start]);

        let widen = |e: &mut Engine| {
            fn_00628640(
                e,
                Ptr::new(max_total),
                Ptr::new(max_total),
                Ptr::new(aabb + AABB_MAXIMUM),
            );
            fn_006285e0(e, Ptr::new(min_total), Ptr::new(min_total), Ptr::new(aabb));
        };

        if e.call(IS_INTERIOR_CELL, &args![cell]).bool() {
            let owner = e.call(0x007a_f430, &args![root]).u32();
            let references = e.call(0x0062_4700, &args![owner]).u32();
            let mut index = 0;
            while index < e.call(WORD_AT_4, &args![references]).i32() {
                let element = e.call(ARRAY_ELEMENT, &args![references, index]).u32();
                let handle = e.mem.u32(element);
                let object = e.call(0x0046_0140, &args![handle]).u32();
                if e.call(NI_POINTER_GET, &args![object]).u32() != 0 {
                    let sink = e.call(0x0062_86d0, &args![root]).u32();
                    // 006282e1: `007d6bb0` takes no stack word (plain RET); the pushed
                    // box address is the second word of the sink's virtual +0x3c.
                    let bounds = e.call(0x007d_6bb0, &args![object]).u32();
                    e.vcall(sink, 0x3c, &args![bounds, aabb]);
                    widen(e);
                }
                index += 1;
            }
        } else if !grid.is_null() {
            let size = e.call(0x0084_e3a0, &args![grid]).u32();
            for row in 0..size {
                for column in 0..size {
                    let slot = e.call(0x004b_a490, &args![grid, row, column]).u32();
                    let grid_cell = e.mem.u32(slot);
                    if grid_cell == 0 {
                        continue;
                    }
                    let owner = e.call(0x0053_7b30, &args![grid_cell]).u32();
                    let objects = e.call(0x0041_3f40, &args![owner]).u32();
                    let mut index = 0u32;
                    while index < e.call(0x0065_8930, &args![objects]).u32() {
                        let element = e.call(SIMPLE_ARRAY_ELEMENT, &args![objects, index]).u32();
                        let pointer = e.call(NI_POINTER_GET, &args![element]).u32();
                        let object = e.call(0x0043_b560, &args![pointer]).u32();
                        if e.call(NI_POINTER_GET, &args![object]).u32() != 0 {
                            let element =
                                e.call(SIMPLE_ARRAY_ELEMENT, &args![objects, index]).u32();
                            let node = e.call(NI_POINTER_GET, &args![element]).u32();
                            if e.vcall(node, 0x94, &args![]).u32() != 0 {
                                let sink = e.call(0x0062_86d0, &args![root]).u32();
                                // 00628471: as above, the box address is the second
                                // word of the sink's +0x3c.
                                let bounds = e.call(0x007d_6bb0, &args![object]).u32();
                                e.vcall(sink, 0x3c, &args![bounds, aabb]);
                            } else {
                                e.call(0x0056_ded0, &args![transform]);
                                let element =
                                    e.call(SIMPLE_ARRAY_ELEMENT, &args![objects, index]).u32();
                                let node = e.call(NI_POINTER_GET, &args![element]).u32();
                                e.vcall(node, 0xf4, &args![transform]);
                                let element =
                                    e.call(SIMPLE_ARRAY_ELEMENT, &args![objects, index]).u32();
                                let node = e.call(NI_POINTER_GET, &args![element]).u32();
                                let shape_owner = e.call(0x004a_e6a0, &args![node]).u32();
                                let shape = e.call(0x0062_0b80, &args![shape_owner]).u32();
                                let tolerance = e.global::<f32>(SHAPE_TOLERANCE);
                                fn_006286a0(e, Ptr::new(shape), transform, tolerance, aabb);
                            }
                            widen(e);
                        }
                        index += 1;
                    }
                }
            }
        }

        e.call(VECTOR_COPY, &args![out, min_total]);
        e.call(VECTOR_COPY, &args![out.addr() + AABB_MAXIMUM, max_total]);
        let max_first = e.call(VECTOR_COMPONENT, &args![max_total, 0u32]).u32();
        let max_unchanged: f64 = e.global(MAX_UNCHANGED);
        if e.mem.f32(max_first) as f64 == max_unchanged {
            return false;
        }
        let min_first = e.call(VECTOR_COMPONENT, &args![min_total, 0u32]).u32();
        let min_unchanged: f64 = e.global(MIN_UNCHANGED);
        e.mem.f32(min_first) as f64 != min_unchanged
    })
}

// Translation incomplete: 006285e0 - /GS stack cookie check (00ec408c) not translated
// Translated from 006285e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per lane minimum of the vectors at `first` and `second` into `this`
/// (`MINPS`: `first` where it is smaller, otherwise `second`). Each address
/// goes through the identity function `006815c0` first.
pub fn fn_006285e0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let second_at = e.call(IDENTITY, &args![second]).u32();
    let second_value = load_vector(e, second_at);
    let first_at = e.call(IDENTITY, &args![first]).u32();
    let first_value = load_vector(e, first_at);
    let mut result = [0.0f32; 4];
    for lane in 0..4 {
        result[lane] = if first_value[lane] < second_value[lane] {
            first_value[lane]
        } else {
            second_value[lane]
        };
    }
    store_vector(e, this.addr(), result);
}

// Translation incomplete: 00628640 - /GS stack cookie check (00ec408c) not translated
// Translated from 00628640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per lane maximum of the vectors at `first` and `second` into `this`
/// (`MAXPS`: `first` where it is larger, otherwise `second`).
pub fn fn_00628640(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    let second_at = e.call(IDENTITY, &args![second]).u32();
    let second_value = load_vector(e, second_at);
    let first_at = e.call(IDENTITY, &args![first]).u32();
    let first_value = load_vector(e, first_at);
    let mut result = [0.0f32; 4];
    for lane in 0..4 {
        result[lane] = if first_value[lane] > second_value[lane] {
            first_value[lane]
        } else {
            second_value[lane]
        };
    }
    store_vector(e, this.addr(), result);
}

// Translated from 006286a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the virtual at slot `0x1c` of `this` with the three arguments
/// (the second is a `float`).
pub fn fn_006286a0(e: &mut Engine, this: Ptr, first: u32, second: f32, third: u32) {
    // 006286ac: the float is passed on through FLD dword / FSTP dword, which
    // quiets a signalling NaN.
    let second = x87_quiet(second);
    e.vcall(this.addr(), 0x1c, &args![first, second, third]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006271b0, fn_006271b0(Ptr<LoadedAreaBound>, Ptr, Ptr)),
        entry!(0x00627920, fn_00627920(Ptr, Ptr)),
        entry!(0x00627980, fn_00627980(Ptr<LoadedAreaBound>, Ptr, Ptr)),
        entry!(0x006281a0, fn_006281a0(Ptr, Ptr) -> Ptr),
        entry!(
            0x006281d0,
            fn_006281d0(Ptr<LoadedAreaBound>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x006285e0, fn_006285e0(Ptr, Ptr, Ptr)),
        entry!(0x00628640, fn_00628640(Ptr, Ptr, Ptr)),
        entry!(0x006286a0, fn_006286a0(Ptr, u32, f32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn recording(e: &mut Engine, addrs: &[u32]) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for a in addrs {
            let a = *a;
            let log = log.clone();
            e.register_double(a, move |_, w| {
                log.borrow_mut().push((a, w.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Doubles of the vector helpers every test needs (16-byte copy, splat,
    /// identity, component address, `hkVector4::set`).
    fn vector_doubles(e: &mut Engine) {
        e.register(IDENTITY, |_, a| ret(a[0]));
        e.register(VECTOR_COPY, |e, a| {
            let v = load_vector(e, a[1]);
            store_vector(e, a[0], v);
            ret(a[0])
        });
        e.register(VECTOR_COMPONENT, |_, a| ret(a[0] + 4 * a[1]));
        e.register(VECTOR_SET_ALL, |e, a| {
            store_vector(e, a[0], [f32::from_bits(a[1]); 4]);
            Ret::default()
        });
        e.register(VECTOR_SET, |e, a| {
            let v = [
                f32::from_bits(a[1]),
                f32::from_bits(a[2]),
                f32::from_bits(a[3]),
                f32::from_bits(a[4]),
            ];
            store_vector(e, a[0], v);
            Ret::default()
        });
        e.register(AABB_CONSTRUCT, |_, _| Ret::default());
    }

    #[test]
    fn splat_multiply_uses_the_first_lane_of_the_other_vector() {
        let mut e = Engine::new();
        e.register(0x0056_1240, |e, a| {
            let x = e.mem.f32(a[0]);
            store_vector(e, a[1], [x; 4]);
            ret(a[1])
        });
        let this = e.mem.alloc(16);
        let other = e.mem.alloc(16);
        store_vector(&mut e, this, [1.0, 2.0, 3.0, 4.0]);
        store_vector(&mut e, other, [-0.5, 9.0, 9.0, 9.0]);
        e.call(0x0062_7920, &args![this, other]);
        assert_eq!(load_vector(&e, this), [-0.5, -1.0, -1.5, -2.0]);
    }

    #[test]
    fn aabb_copy_copies_both_vectors_and_returns_this() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let source = e.mem.alloc(32);
        let this = e.mem.alloc(32);
        store_vector(&mut e, source, [1.0, 2.0, 3.0, 4.0]);
        store_vector(&mut e, source + 16, [5.0, 6.0, 7.0, 8.0]);
        let result = e.call(0x0062_81a0, &args![this, source]).u32();
        assert_eq!(result, this);
        assert_eq!(load_vector(&e, this), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vector(&e, this + 16), [5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn min_and_max_follow_the_sse_rules_per_lane() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let first = e.mem.alloc(16);
        let second = e.mem.alloc(16);
        let this = e.mem.alloc(16);
        store_vector(&mut e, first, [1.0, 2.0, f32::NAN, 0.0]);
        store_vector(&mut e, second, [2.0, 1.0, 5.0, -0.0]);
        e.call(0x0062_85e0, &args![this, first, second]);
        let min = load_vector(&e, this);
        assert_eq!(min[0], 1.0);
        assert_eq!(min[1], 1.0);
        // A NaN in the first operand gives the second.
        assert_eq!(min[2], 5.0);
        // Equal values (here 0.0 and -0.0) give the second operand.
        assert_eq!(min[3].to_bits(), (-0.0f32).to_bits());
        e.call(0x0062_8640, &args![this, first, second]);
        let max = load_vector(&e, this);
        assert_eq!(max[0], 2.0);
        assert_eq!(max[1], 2.0);
        assert_eq!(max[2], 5.0);
        assert_eq!(max[3].to_bits(), (-0.0f32).to_bits());
    }

    #[test]
    fn the_shape_box_call_goes_through_slot_0x1c() {
        let mut e = Engine::new();
        let log = recording(&mut e, &[0x0060_c200]);
        let mut slots = vec![0u32; 0x10];
        slots[0x1c / 4] = 0x0060_c200;
        e.put_vtable(0x0a00_0000, &slots);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0a00_0000);
        e.call(0x0062_86a0, &args![this, 0x11u32, 0.1f32, 0x33u32]);
        assert_eq!(
            calls_to(&log, 0x0060_c200),
            vec![vec![this, 0x11, 0.1f32.to_bits(), 0x33]]
        );
    }

    #[test]
    fn fn_006286a0_passes_a_signalling_nan_on_quieted() {
        let mut e = Engine::new();
        let log = recording(&mut e, &[0x0060_c200]);
        let mut slots = vec![0u32; 0x10];
        slots[0x1c / 4] = 0x0060_c200;
        e.put_vtable(0x0a00_0000, &slots);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x0a00_0000);
        // 006286ac: FLD dword / FSTP dword of the float argument.
        e.call(0x0062_86a0, &args![this, 0x11u32, 0x7f80_0001u32, 0x33u32]);
        assert_eq!(
            calls_to(&log, 0x0060_c200),
            vec![vec![this, 0x11, 0x7fc0_0001, 0x33]]
        );
    }

    /// Doubles for `006281d0`: the world's root, the box a reference or an
    /// object reports (written by `007d6bb0`), and the sink that receives it.
    fn box_engine() -> (Engine, Rc<RefCell<Vec<u32>>>) {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        e.register(0x0045_43c0, |_, a| ret(a[0]));
        e.register(0x0054_1c80, |_, _| ret(0x5000));
        e.register(0x0062_86d0, |_, _| ret(0x6000));
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(ARRAY_ELEMENT, |e, a| ret(e.mem.u32(a[0]) + 4 * a[1]));
        e.register(0x0046_0140, |_, a| ret(a[0]));
        e.register(0x007a_f430, |_, a| ret(a[0]));
        // The sink's virtual 0x3c: records the bounds it is given and, as a model,
        // copies them into the box (its second word).
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(0x0060_c300, move |e, a| {
            log.borrow_mut().push(a[1]);
            let bounds = e.mem.bytes(a[1], 32);
            e.mem.write(a[2], &bounds);
            Ret::default()
        });
        let mut slots = vec![0u32; 0x40];
        slots[0x3c / 4] = 0x0060_c300;
        e.put_vtable(0x0a00_1000, &slots);
        e.mem.map(0x6000, 8);
        e.mem.set_u32(0x6000, 0x0a00_1000);
        e.map(MAX_START, 8);
        e.map(MIN_START, 8);
        e.map(MAX_UNCHANGED, 8);
        e.map(MIN_UNCHANGED, 8);
        e.set_global(MAX_START, -3.4028235e38f32);
        e.set_global(MIN_START, 3.4028235e38f32);
        e.set_global(MAX_UNCHANGED, -3.4028235e38f32 as f64);
        e.set_global(MIN_UNCHANGED, 3.4028235e38f32 as f64);
        (e, seen)
    }

    #[test]
    fn interior_box_covers_every_loaded_reference() {
        let (mut e, seen) = box_engine();
        e.register(IS_INTERIOR_CELL, |_, _| ret(1));
        // The reference list: an `hkArray` of two handles; the second
        // reference has no loaded object.
        let references = e.mem.alloc(12);
        let data = e.mem.alloc(8);
        e.mem.set_u32(references, data);
        e.mem.set_u32(references + 4, 2);
        let first_object = e.mem.alloc(8);
        e.mem.set_u32(first_object, 0x7001);
        let second_object = e.mem.alloc(8);
        e.mem.set_u32(data, first_object);
        e.mem.set_u32(data + 4, second_object);
        e.register_double(0x0062_4700, move |_, _| ret(references));
        // Each loaded object reports a box (unit offsets by the object).
        e.register_double(0x007d_6bb0, move |e, a| {
            assert_eq!(a.len(), 1);
            let base = if a[0] == first_object { 0.0 } else { 100.0 };
            let bounds = e.mem.alloc(32);
            store_vector(e, bounds, [base - 1.0, base - 2.0, base - 3.0, 0.0]);
            store_vector(e, bounds + 16, [base + 1.0, base + 2.0, base + 3.0, 0.0]);
            ret(bounds)
        });
        // NI_POINTER_GET on the references' handles gives the object.
        e.register(0x0046_0140, |_, a| ret(a[0]));
        let cell = 0x4000;
        let this = e.new_object::<LoadedAreaBound>();
        let out = e.mem.alloc(32);
        let found = fn_006281d0(&mut e, this, Ptr::new(cell), Ptr::NULL, Ptr::new(out));
        assert!(found);
        // Only the first object is loaded: its box.
        assert_eq!(&load_vector(&e, out)[..3], &[-1.0, -2.0, -3.0]);
        assert_eq!(&load_vector(&e, out + 16)[..3], &[1.0, 2.0, 3.0]);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn an_empty_box_is_reported_as_nothing_found() {
        let (mut e, seen) = box_engine();
        e.register(IS_INTERIOR_CELL, |_, _| ret(0));
        let this = e.new_object::<LoadedAreaBound>();
        let out = e.mem.alloc(32);
        // Exterior cell without a grid: nothing contributes.
        assert!(!fn_006281d0(
            &mut e,
            this,
            Ptr::new(0x4000),
            Ptr::NULL,
            Ptr::new(out)
        ));
        assert_eq!(load_vector(&e, out)[0], 3.4028235e38);
        assert_eq!(load_vector(&e, out + 16)[0], -3.4028235e38);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn no_world_means_no_box() {
        let (mut e, _) = box_engine();
        e.register(0x0045_43c0, |_, _| Ret::default());
        let this = e.new_object::<LoadedAreaBound>();
        let out = e.mem.alloc(32);
        assert!(!fn_006281d0(
            &mut e,
            this,
            Ptr::new(0x4000),
            Ptr::NULL,
            Ptr::new(out)
        ));
        e.register(0x0045_43c0, |_, a| ret(a[0]));
        e.register(0x0054_1c80, |_, _| Ret::default());
        assert!(!fn_006281d0(
            &mut e,
            this,
            Ptr::new(0x4000),
            Ptr::NULL,
            Ptr::new(out)
        ));
    }

    #[test]
    fn exterior_box_takes_the_bounds_of_grid_objects() {
        let (mut e, seen) = box_engine();
        e.register(IS_INTERIOR_CELL, |_, _| ret(0));
        // A 1 x 1 grid whose slot holds a cell (0x7100) with one object.
        let grid = e.mem.alloc(0x20);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x7100);
        e.register(0x0084_e3a0, |_, _| ret(1));
        e.register_double(0x004b_a490, move |_, _| ret(slot));
        e.register(0x0053_7b30, |_, a| ret(a[0] + 1));
        e.register(0x0041_3f40, |_, a| ret(a[0] + 1));
        e.register(0x0065_8930, |_, _| ret(1));
        // The object list is one slot holding a handle; the handle and the
        // object are the same word here.
        let object_slot = e.mem.alloc(4);
        e.mem.set_u32(object_slot, object_slot);
        e.register_double(SIMPLE_ARRAY_ELEMENT, move |_, _| ret(object_slot));
        e.register_double(NI_POINTER_GET, move |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0043_b560, |_, a| ret(a[0]));
        // The object answers 1 from its virtual 0x94: use its own bounds.
        let mut slots = vec![0u32; 0x80];
        slots[0] = 1;
        slots[0x94 / 4] = 0x0060_c400;
        e.put_vtable(0x0a00_2000, &slots);
        e.register(0x0060_c400, |_, _| ret(1));
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0a00_2000);
        e.mem.set_u32(object_slot, object);
        e.register(0x007d_6bb0, |e, a| {
            assert_eq!(a.len(), 1);
            let bounds = e.mem.alloc(32);
            store_vector(e, bounds, [-4.0, -5.0, -6.0, 0.0]);
            store_vector(e, bounds + 16, [4.0, 5.0, 6.0, 0.0]);
            ret(bounds)
        });
        let this = e.new_object::<LoadedAreaBound>();
        let out = e.mem.alloc(32);
        assert!(fn_006281d0(
            &mut e,
            this,
            Ptr::new(0x4000),
            Ptr::new(grid),
            Ptr::new(out)
        ));
        assert_eq!(&load_vector(&e, out)[..3], &[-4.0, -5.0, -6.0]);
        assert_eq!(&load_vector(&e, out + 16)[..3], &[4.0, 5.0, 6.0]);
        assert_eq!(seen.borrow().len(), 1);
    }

    /// The six corner pairs of `00627980` for a box (-10, -20, -30) to
    /// (10, 20, 30), `W = 1`, `D = 2`, `H = 0.5`: ((x, y, z), (x, y, z)) per
    /// wall, first and second corner.
    #[test]
    fn six_walls_get_their_corners_and_phantoms() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let corners: Rc<RefCell<Vec<[f32; 4]>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = corners.clone();
        e.register_double(VECTOR_SET, move |_, a| {
            sink.borrow_mut().push([
                f32::from_bits(a[1]),
                f32::from_bits(a[2]),
                f32::from_bits(a[3]),
                f32::from_bits(a[4]),
            ]);
            Ret::default()
        });
        e.register(0x0045_43c0, |_, _| ret(0x9000));
        e.register(IS_INTERIOR_CELL, |_, _| ret(0));
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let log = recording(
            &mut e,
            &[
                0x0056_e4e0,
                0x0062_81a0,
                0x0056_d2c0,
                0x0056_d730,
                0x0062_2290,
                0x0056_e550,
            ],
        );
        e.register(0x005d_be10, |_, _| ret(0x2a));
        e.register(0x00aa_13e0, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(0x0056_e550, |_, a| ret(a[0]));
        let this = e.new_object::<LoadedAreaBound>();
        e.mem.set_f32(this.addr() + BOUND_OFFSET, 1.0);
        e.mem.set_f32(this.addr() + BOUND_DEPTH, 2.0);
        e.mem.set_f32(this.addr() + SIDE_WALL_HEIGHT_OFFSET, 0.5);
        let bounds = e.mem.alloc(32);
        store_vector(&mut e, bounds, [-10.0, -20.0, -30.0, 0.0]);
        store_vector(&mut e, bounds + 16, [10.0, 20.0, 30.0, 0.0]);
        fn_00627980(&mut e, this, Ptr::new(0x4000), Ptr::new(bounds));
        let c = corners.borrow().clone();
        let expected: [[f32; 3]; 12] = [
            [12.0, 22.0, -31.0],
            [-12.0, -22.0, -32.0],
            [12.0, 22.0, 32.5],
            [-12.0, -22.0, 31.5],
            [11.0, -21.0, 31.5],
            [-11.0, -22.0, -31.0],
            [-11.0, 22.0, 31.5],
            [-12.0, -22.0, -31.0],
            [12.0, 22.0, 31.5],
            [11.0, -22.0, -31.0],
            [11.0, 22.0, 31.5],
            [-11.0, 21.0, -31.0],
        ];
        assert_eq!(c.len(), 12);
        for (corner, want) in c.iter().zip(expected) {
            assert_eq!(&corner[..3], &want, "corner");
            assert_eq!(corner[3], 0.0);
        }
        // Every wall's phantom goes into the world and the info is destroyed.
        let adds = calls_to(&log, 0x0056_d2c0);
        assert_eq!(adds.len(), 6);
        assert!(adds.iter().all(|w| w[0] == 0x9000));
        assert_eq!(calls_to(&log, 0x0056_d730).len(), 6);
        for wall in 0..6u32 {
            assert_ne!(e.mem.u32(this.addr() + 8 + wall * 4), 0);
        }
        // The filter word is built from the next collision-layer seed.
        assert_eq!(calls_to(&log, 0x0062_2290)[0][1..], [0, 0x2a, 0]);
    }

    #[test]
    fn side_wall_height_is_dropped_inside() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let corners: Rc<RefCell<Vec<[f32; 4]>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = corners.clone();
        e.register_double(VECTOR_SET, move |_, a| {
            sink.borrow_mut().push([f32::from_bits(a[3]); 4]);
            Ret::default()
        });
        e.register(0x0045_43c0, |_, _| ret(0x9000));
        e.register(IS_INTERIOR_CELL, |_, _| ret(1));
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        for a in [
            0x0056_e4e0u32,
            0x0062_81a0,
            0x0056_d2c0,
            0x0056_d730,
            0x0062_2290,
            0x0056_e550,
        ] {
            e.register(a, |_, a| ret(a[0]));
        }
        e.register(0x005d_be10, |_, _| ret(0));
        e.register(0x00aa_13e0, |e, a| ret(e.mem.alloc(a[0])));
        let this = e.new_object::<LoadedAreaBound>();
        e.mem.set_f32(this.addr() + BOUND_OFFSET, 1.0);
        e.mem.set_f32(this.addr() + BOUND_DEPTH, 2.0);
        e.mem.set_f32(this.addr() + SIDE_WALL_HEIGHT_OFFSET, 0.5);
        let bounds = e.mem.alloc(32);
        store_vector(&mut e, bounds, [-10.0, -20.0, -30.0, 0.0]);
        store_vector(&mut e, bounds + 16, [10.0, 20.0, 30.0, 0.0]);
        fn_00627980(&mut e, this, Ptr::new(0x4000), Ptr::new(bounds));
        // Wall 1: 30 + 2 + 0 and 30 + 0 + 1.
        let c = corners.borrow().clone();
        assert_eq!(c[2][0], 32.0);
        assert_eq!(c[3][0], 31.0);
    }

    /// 00627b74..00627b95: wall 1's first z is `FLD max.z`, `FADD [this + 0x38]`,
    /// `FADD` the height, `FSTP`, each operation rounded at the thread's x87
    /// precision. With max.z 1 and `D = H = 2^-24`: at 24 bits each sum is a tie
    /// that rounds back to 1; at 53 bits the two halves make `1 + 2^-23`.
    #[test]
    fn wall_corners_round_each_x87_operation_at_the_thread_precision() {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let zs: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = zs.clone();
        e.register_double(VECTOR_SET, move |_, a| {
            sink.borrow_mut().push(a[3]);
            Ret::default()
        });
        e.register(0x0045_43c0, |_, _| ret(0x9000));
        e.register(IS_INTERIOR_CELL, |_, _| ret(0));
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        for a in [
            0x0056_e4e0u32,
            0x0062_81a0,
            0x0056_d2c0,
            0x0056_d730,
            0x0062_2290,
            0x0056_e550,
        ] {
            e.register(a, |_, a| ret(a[0]));
        }
        e.register(0x005d_be10, |_, _| ret(0));
        e.register(0x00aa_13e0, |e, a| ret(e.mem.alloc(a[0])));
        let this = e.new_object::<LoadedAreaBound>();
        let half_ulp = 2f32.powi(-24);
        e.mem.set_f32(this.addr() + BOUND_DEPTH, half_ulp);
        e.mem
            .set_f32(this.addr() + SIDE_WALL_HEIGHT_OFFSET, half_ulp);
        let bounds = e.mem.alloc(32);
        store_vector(&mut e, bounds, [0.0; 4]);
        store_vector(&mut e, bounds + 16, [1.0; 4]);
        crate::units::crt::set_x87_control_word(&mut e, 0x007f);
        fn_00627980(&mut e, this, Ptr::new(0x4000), Ptr::new(bounds));
        crate::units::crt::set_x87_control_word(&mut e, 0x027f);
        assert_eq!(zs.borrow().len(), 12);
        assert_eq!(zs.borrow()[2], 1.0f32.to_bits());
        zs.borrow_mut().clear();
        fn_00627980(&mut e, this, Ptr::new(0x4000), Ptr::new(bounds));
        assert_eq!(zs.borrow()[2], (1.0f32 + f32::EPSILON).to_bits());
    }

    /// Shared state for the `006271b0` doubles: arrays (`data, size`), the
    /// hit maps and the call log.
    #[derive(Default)]
    struct Sim {
        maps: BTreeMap<u32, BTreeMap<u32, u16>>,
        removed: Vec<(u32, u32)>,
        warped: Vec<u32>,
        pushed_back: Vec<(u32, [f32; 4])>,
        queries: u32,
    }

    /// An engine with every callee of `006271b0` doubled by a small working
    /// model: `hkArray`s in memory, hit maps in `Sim`.
    fn update_engine(interior: bool, wall_with_bodies: u32) -> (Engine, Rc<RefCell<Sim>>) {
        let mut e = Engine::new();
        vector_doubles(&mut e);
        let sim = Rc::new(RefCell::new(Sim::default()));
        e.register(
            IS_INTERIOR_CELL,
            if interior {
                |_, _| ret(1)
            } else {
                |_, _| ret(0)
            },
        );
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(ARRAY_CONSTRUCT, |e, a| {
            for off in [0, 4, 8] {
                e.mem.set_u32(a[0] + off, 0);
            }
            ret(a[0])
        });
        e.register(ARRAY_RESERVE, |_, _| Ret::default());
        e.register(ARRAY_DESTRUCT, |_, _| Ret::default());
        e.register(ARRAY_ELEMENT, |e, a| ret(e.mem.u32(a[0]) + 4 * a[1]));
        e.register(ARRAY_PUSH_BACK, |e, a| {
            let mut data = e.mem.u32(a[0]);
            let size = e.mem.u32(a[0] + 4);
            if data == 0 {
                data = e.mem.alloc(0x40);
                e.mem.set_u32(a[0], data);
            }
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(data + 4 * size, value);
            e.mem.set_u32(a[0] + 4, size + 1);
            Ret::default()
        });
        e.register(ARRAY_FIND, |e, a| {
            let needle = e.mem.u32(a[1]);
            let data = e.mem.u32(a[0]);
            let size = e.mem.u32(a[0] + 4);
            for i in 0..size {
                if e.mem.u32(data + 4 * i) == needle {
                    return ret(i);
                }
            }
            ret(-1i32 as u32)
        });
        e.register(MAP_CONSTRUCT, |_, _| Ret::default());
        e.register(MAP_DESTRUCT, |_, _| Ret::default());
        let state = sim.clone();
        e.register_double(MAP_GET, move |e, a| {
            let value = state
                .borrow()
                .maps
                .get(&a[0])
                .and_then(|m| m.get(&a[1]).copied());
            match value {
                Some(v) => {
                    e.mem.set_u16(a[2], v);
                    ret(1)
                }
                None => ret(0),
            }
        });
        let state = sim.clone();
        e.register_double(MAP_SET, move |_, a| {
            state
                .borrow_mut()
                .maps
                .entry(a[0])
                .or_default()
                .insert(a[1], a[2] as u16);
            Ret::default()
        });
        let state = sim.clone();
        e.register_double(MAP_FIRST, move |_, a| {
            ret(
                if state
                    .borrow()
                    .maps
                    .get(&a[0])
                    .is_some_and(|m| !m.is_empty())
                {
                    1
                } else {
                    0
                },
            )
        });
        let state = sim.clone();
        e.register_double(MAP_NEXT, move |e, a| {
            let index = e.mem.u32(a[1]) as usize - 1;
            let state = state.borrow();
            let entries: Vec<(u32, u16)> =
                state.maps[&a[0]].iter().map(|(k, v)| (*k, *v)).collect();
            e.mem.set_u32(a[2], entries[index].0);
            e.mem.set_u16(a[3], entries[index].1);
            e.mem.set_u32(
                a[1],
                if index + 1 < entries.len() {
                    index as u32 + 2
                } else {
                    0
                },
            );
            Ret::default()
        });
        let state = sim.clone();
        e.register_double(MAP_REMOVE, move |_, a| {
            state.borrow_mut().removed.push((a[0], a[1]));
            Ret::default()
        });
        // BSSimpleArray of removed keys: element i at data + 4 * i.
        e.register(SIMPLE_ARRAY_CONSTRUCT, |e, a| {
            for off in [0, 4, 8, 12] {
                e.mem.set_u32(a[0] + off, 0);
            }
            ret(a[0])
        });
        e.register(SIMPLE_ARRAY_DESTRUCT, |_, _| Ret::default());
        e.register(SIMPLE_ARRAY_ADD, |e, a| {
            let mut data = e.mem.u32(a[0]);
            let size = e.mem.u32(a[0] + 4);
            if data == 0 {
                data = e.mem.alloc(0x40);
                e.mem.set_u32(a[0], data);
            }
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(data + 4 * size, value);
            e.mem.set_u32(a[0] + 4, size + 1);
            Ret::default()
        });
        e.register(SIMPLE_ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SIMPLE_ARRAY_ELEMENT, |e, a| ret(e.mem.u32(a[0]) + 4 * a[1]));
        // One overlap list, belonging to `wall_with_bodies`: the walls'
        // phantoms are their own indices + 1.
        e.register_double(NI_POINTER_GET, move |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0062_0b80, |_, a| ret(a[0]));

        let list = e.mem.alloc(12);
        let data = e.mem.alloc(16);
        e.mem.set_u32(list, data);
        e.mem.set_u32(list + 4, 2);
        // Two bodies: 0xa1 has a reference (0x7a1), 0xa2 only a plain one.
        e.mem.set_u32(data, 0xa1);
        e.mem.set_u32(data + 4, 0xa2);
        e.register_double(0x0045_c650, move |_, _| ret(list));
        e.register(0x0062_e120, |_, _| ret(0));
        let state = sim.clone();
        e.register_double(0x0056_09f0, move |_, _| {
            let mut state = state.borrow_mut();
            let wall = state.queries;
            state.queries += 1;
            ret((wall != wall_with_bodies) as u32)
        });
        e.register(0x004b_59f0, |_, a| {
            ret(if a[0] == 0xa1 { 0x7a1 } else { 0 })
        });
        e.register(0x004b_5950, |_, a| {
            ret(if a[0] == 0xa2 { 0x7a2 } else { 0 })
        });
        e.register(0x0046_0140, |_, a| ret(a[0]));
        e.register(0x0043_b540, |_, a| ret(a[0]));
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        // Form type of the handle: 0x1e for the plain reference's form.
        e.register(0x0043_b4d0, |e, a| {
            ret(if e.mem.u32(a[0]) == 0x7a2 { 0x1e } else { 0x20 })
        });
        e.register(0x0062_b4e0, |_, a| ret(a[0] + 0x1000));
        // The actor (0x7a2 + 0x1000) answers `IsActor` true.
        let mut slots = vec![0u32; 0x80];
        slots[0x100 / 4] = 0x0060_c500;
        e.put_vtable(0x0a00_3000, &slots);
        e.mem.map(0x7a2 + 0x1000, 8);
        e.mem.set_u32(0x7a2 + 0x1000, 0x0a00_3000);
        e.register(0x0060_c500, |_, _| ret(1));
        let state = sim.clone();
        e.register_double(WARP_ACTOR, move |_, a| {
            state.borrow_mut().warped.push(a[1]);
            Ret::default()
        });
        e.register(0x0051_7670, |_, _| ret(1));
        e.register(0x004b_5a80, |_, a| ret(a[0] + 0x10000));
        e.register(0x0062_8f90, |_, _| Ret::default());
        e.register(0x0062_86f0, |_, _| Ret::default());
        e.register(0x0062_8da0, |_, _| Ret::default());
        // Push-back helpers.
        e.register(0x0056_0d80, |_, a| ret(a[0]));
        e.register(VECTOR_COPY_FROM_POINT, |e, a| {
            store_vector(e, a[0], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        e.register(0x005d_bf20, |e, a| {
            store_vector(e, a[0], [f32::from_bits(a[1]); 4]);
            Ret::default()
        });
        e.register(0x0056_1240, |e, a| {
            let x = e.mem.f32(a[0]);
            store_vector(e, a[1], [x; 4]);
            ret(a[1])
        });
        let state = sim.clone();
        e.register_double(0x0056_1690, move |e, a| {
            state
                .borrow_mut()
                .pushed_back
                .push((a[0], load_vector(e, a[1])));
            Ret::default()
        });
        e.map(MINUS_ONE, 8);
        e.set_global(MINUS_ONE, -1.0f64);
        (e, sim)
    }

    #[test]
    fn a_null_cell_clears_the_current_cell() {
        let (mut e, _) = update_engine(false, 99);
        let log = recording(&mut e, &[0x0062_8da0]);
        let this = e.new_object::<LoadedAreaBound>();
        e.set(this, LoadedAreaBound::pCurrentCell, Ptr::new(0x4000));
        fn_006271b0(&mut e, this, Ptr::NULL, Ptr::NULL);
        assert!(e.get(this, LoadedAreaBound::pCurrentCell).is_null());
        assert_eq!(calls_to(&log, 0x0062_8da0), vec![vec![this.addr()]]);
    }

    #[test]
    fn an_empty_box_leaves_the_current_cell_alone() {
        let (mut e, _) = update_engine(false, 99);
        // The box computation fails (no world).
        e.register(0x0045_43c0, |_, _| Ret::default());
        let log = recording(&mut e, &[0x0062_86f0]);
        let this = e.new_object::<LoadedAreaBound>();
        e.set(this, LoadedAreaBound::pCurrentCell, Ptr::new(0x4000));
        fn_006271b0(&mut e, this, Ptr::new(0x5000), Ptr::NULL);
        assert_eq!(e.get(this, LoadedAreaBound::pCurrentCell), Ptr::new(0x4000));
        assert!(calls_to(&log, 0x0062_86f0).is_empty());
    }

    #[test]
    fn interior_update_warps_actors_and_collects_nothing_into_the_hit_map() {
        let (mut e, sim) = update_engine(true, 0);
        let this = e.new_object::<LoadedAreaBound>();
        e.set(this, LoadedAreaBound::pCurrentCell, Ptr::new(0x4000));
        fn_006271b0(&mut e, this, Ptr::new(0x4000), Ptr::NULL);
        let sim = sim.borrow();
        // The plain body's reference form is an actor (type 0x1e): warped.
        assert_eq!(sim.warped, vec![0x7a2 + 0x1000]);
        // Interior: the referenced body goes through `00628f90`, not the
        // hit map.
        assert!(!sim.maps.contains_key(&(this.addr() + HIT_MAP)));
        assert_eq!(sim.queries, 6);
    }

    #[test]
    fn exterior_update_pushes_bodies_back_and_keeps_the_hit_map() {
        // Wall 2 (mask 4) has the bodies; the exterior case warps only for
        // wall 0.
        let (mut e, sim) = update_engine(false, 2);
        let this = e.new_object::<LoadedAreaBound>();
        e.set(this, LoadedAreaBound::pCurrentCell, Ptr::new(0x4000));
        e.mem.set_f32(this.addr() + VELOCITY_DAMPEN, 0.5);
        // An old hit entry that is not hit again, and one already hit by
        // this wall's mask.
        let map = this.addr() + HIT_MAP;
        sim.borrow_mut()
            .maps
            .entry(map)
            .or_default()
            .insert(0x77, 1);
        fn_006271b0(&mut e, this, Ptr::new(0x4000), Ptr::NULL);
        let sim = sim.borrow();
        assert!(sim.warped.is_empty());
        // Body 0x7a1 + 0x10000 is pushed: velocity (1, 2, 3, 4) times the
        // dampening negated (-0.5), then z negated again.
        assert_eq!(sim.pushed_back.len(), 1);
        assert_eq!(sim.pushed_back[0].0, 0x7a1 + 0x10000);
        assert_eq!(sim.pushed_back[0].1, [-0.5, -1.0, 1.5, -2.0]);
        // The new hit map records mask 4 for the body, and the old map now
        // holds it; the stale entry 0x77 is removed.
        assert_eq!(sim.maps[&map][&(0x7a1 + 0x10000)], 4);
        assert_eq!(sim.removed, vec![(map, 0x77)]);
    }

    #[test]
    fn a_body_already_hit_by_the_same_wall_is_not_pushed_again() {
        let (mut e, sim) = update_engine(false, 2);
        let this = e.new_object::<LoadedAreaBound>();
        e.set(this, LoadedAreaBound::pCurrentCell, Ptr::new(0x4000));
        let map = this.addr() + HIT_MAP;
        sim.borrow_mut()
            .maps
            .entry(map)
            .or_default()
            .insert(0x7a1 + 0x10000, 4 | 1);
        fn_006271b0(&mut e, this, Ptr::new(0x4000), Ptr::NULL);
        let sim = sim.borrow();
        assert!(sim.pushed_back.is_empty());
        assert_eq!(sim.maps[&map][&(0x7a1 + 0x10000)], 4);
    }

    #[test]
    fn a_new_cell_rebuilds_the_phantoms_first() {
        let (mut e, sim) = update_engine(true, 99);
        // Box: the world, its root and one loaded reference.
        e.register(0x0045_43c0, |_, a| ret(a[0]));
        e.register(0x0054_1c80, |_, _| ret(0x5000));
        e.register(0x007a_f430, |_, a| ret(a[0]));
        let references = e.mem.alloc(12);
        let data = e.mem.alloc(4);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 1);
        e.mem.set_u32(data, object);
        e.mem.set_u32(references, data);
        e.mem.set_u32(references + 4, 1);
        e.register_double(0x0062_4700, move |_, _| ret(references));
        e.register(0x0062_86d0, |_, _| ret(0x6000));
        // The sink's +0x3c (a model): copies the bounds into the box, its second word.
        e.register(0x0060_c300, |e, a| {
            let bounds = e.mem.bytes(a[1], 32);
            e.mem.write(a[2], &bounds);
            Ret::default()
        });
        let mut slots = vec![0u32; 0x40];
        slots[0x3c / 4] = 0x0060_c300;
        e.put_vtable(0x0a00_1000, &slots);
        e.mem.map(0x6000, 8);
        e.mem.set_u32(0x6000, 0x0a00_1000);
        e.register(0x007d_6bb0, |e, a| {
            assert_eq!(a.len(), 1);
            let bounds = e.mem.alloc(32);
            store_vector(e, bounds, [-1.0, -2.0, -3.0, 0.0]);
            store_vector(e, bounds + 16, [1.0, 2.0, 3.0, 0.0]);
            ret(bounds)
        });
        for (addr, value) in [(MAX_START, -3.4028235e38f32), (MIN_START, 3.4028235e38f32)] {
            e.map(addr, 8);
            e.set_global(addr, value);
        }
        for (addr, value) in [
            (MAX_UNCHANGED, -3.4028235e38f32 as f64),
            (MIN_UNCHANGED, 3.4028235e38f32 as f64),
        ] {
            e.map(addr, 8);
            e.set_global(addr, value);
        }
        // Phantom construction.
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        for addr in [0x0056_e4e0u32, 0x0056_d2c0, 0x0056_d730, 0x0062_2290] {
            e.register(addr, |_, a| ret(a[0]));
        }
        e.register(0x0056_e550, |_, a| ret(a[0]));
        e.register(0x005d_be10, |_, _| ret(0x11));
        e.register(0x00aa_13e0, |e, a| ret(e.mem.alloc(a[0])));
        let log = recording(&mut e, &[0x0062_86f0]);
        let this = e.new_object::<LoadedAreaBound>();
        e.mem.set_f32(this.addr() + BOUND_OFFSET, 1.0);
        e.mem.set_f32(this.addr() + BOUND_DEPTH, 2.0);
        fn_006271b0(&mut e, this, Ptr::new(0x4400), Ptr::NULL);
        assert_eq!(e.get(this, LoadedAreaBound::pCurrentCell), Ptr::new(0x4400));
        assert_eq!(calls_to(&log, 0x0062_86f0), vec![vec![this.addr()]]);
        for wall in 0..6u32 {
            assert_ne!(e.mem.u32(this.addr() + 8 + wall * 4), 0);
        }
        // The main loop then ran over all six walls.
        assert_eq!(sim.borrow().queries, 6);
    }
}
