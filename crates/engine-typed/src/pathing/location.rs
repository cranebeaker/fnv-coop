//! Pathing locations: a point with the navmesh triangle and the cell or world space it is in.

use super::layout::{NavMeshInfo, PathingLocation, PATHING_LOCATION_VTABLE};
use super::navmesh::{is_loaded, parent_world_space, resolve, NO_TRIANGLE};
use super::{copy_point, fld_global, fstp, FAMILY, FLOAT_MAX};
use crate::culling::layout::NiPoint3;
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::{args, typed_call, Engine};

/// Callees.
const CELL_AT: u32 = 0x0058_7630;
const SET_FROM: u32 = 0x006d_d280;
const INTERIOR_CELL: u32 = 0x006b_77b0;
const REFRESH: u32 = 0x006d_cfe0;
/// The cell key of a location in no exterior cell.
const NO_CELL_KEY: u32 = 0xdead_dead;

/// The vtable and `ReturnThis` on the point, as each constructor starts.
fn construct_base(e: &mut Engine, this: Ptr<PathingLocation>) {
    e.set(this, PathingLocation::vtable, PATHING_LOCATION_VTABLE);
    let point = this.at(PathingLocation::point);
    typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, point));
}

/// Every field after the vtable copied as words.
fn copy_fields(e: &mut Engine, this: Ptr<PathingLocation>, other: Ptr<PathingLocation>) {
    copy_point(
        e,
        other.at(PathingLocation::point),
        this.at(PathingLocation::point),
    );
    for f in [
        PathingLocation::info,
        PathingLocation::unknown_14,
        PathingLocation::cell,
        PathingLocation::world_space,
        PathingLocation::cell_key,
    ] {
        let w = e.get(other, f);
        e.set(this, f, w);
    }
    let t = e.get(other, PathingLocation::triangle);
    e.set(this, PathingLocation::triangle, t);
    let flags = e.get(other, PathingLocation::flags);
    e.set(this, PathingLocation::flags, flags);
}

// Typed from 00441110
/// `PathingLocation::GetWorldspace` (`thiscall(location)`).
pub fn world_space(e: &mut Engine, this: Ptr<PathingLocation>) -> u32 {
    e.get(this, PathingLocation::world_space)
}

// Typed from 006a9520
/// Whether the location's flag 2 is set (`thiscall(location)`).
pub fn is_valid(e: &mut Engine, this: Ptr<PathingLocation>) -> bool {
    e.get(this, PathingLocation::flags) & 2 != 0
}

// Typed from 006dd640
/// `PathingLocation::GetNavMeshAndTriangle` (`thiscall(location, out holder, out triangle)`): false
/// without an info or a triangle, or when the info's mesh doesn't resolve.
pub fn mesh_and_triangle(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    out_mesh: Ptr,
    out_triangle: Ptr,
) -> bool {
    let info = e.get(this, PathingLocation::info);
    if info == 0 {
        return false;
    }
    if e.get(this, PathingLocation::triangle) == NO_TRIANGLE {
        return false;
    }
    let info = Ptr::<NavMeshInfo>::new(e.get(this, PathingLocation::info));
    if !typed_call!(FAMILY, 0x0069_ad00, resolve(e, info, out_mesh)) {
        return false;
    }
    let t = e.get(this, PathingLocation::triangle);
    e.mem.set_u16(out_triangle.addr(), t);
    true
}

// Typed from 006dd610
/// `PathingLocation::GetNavMesh` (`thiscall(location, out holder)`).
pub fn mesh(e: &mut Engine, this: Ptr<PathingLocation>, out: Ptr) -> bool {
    let info = e.get(this, PathingLocation::info);
    if info == 0 {
        return false;
    }
    typed_call!(FAMILY, 0x0069_ad00, resolve(e, Ptr::new(info), out))
}

// Typed from 006dd6a0
/// `PathingLocation::HasValidTriangle` (`thiscall(location)`): an info whose mesh is loaded, and a
/// triangle.
pub fn has_triangle(e: &mut Engine, this: Ptr<PathingLocation>) -> bool {
    let info = e.get(this, PathingLocation::info);
    if info == 0 {
        return false;
    }
    if !typed_call!(FAMILY, 0x0069_dfb0, is_loaded(e, Ptr::new(info))) {
        return false;
    }
    e.get(this, PathingLocation::triangle) != NO_TRIANGLE
}

// Typed from 006dd4f0
/// `PathingLocation::GetCell` (`thiscall(location)`): the cell, else the world space's cell at the
/// key (`00587630`), else null.
pub fn cell(e: &mut Engine, this: Ptr<PathingLocation>) -> u32 {
    let c = e.get(this, PathingLocation::cell);
    if c != 0 {
        return c;
    }
    let w = e.get(this, PathingLocation::world_space);
    if w != 0 {
        let key = e.get(this, PathingLocation::cell_key);
        let c = e.call(CELL_AT, &args![w, key]).u32();
        if c != 0 {
            return c;
        }
    }
    0
}

// Typed from 006dcbb0
/// `PathingLocation::PathingLocation` (`thiscall(location)`): the point at `FLT_MAX` in each axis, no
/// info, cell, world space or triangle.
pub fn new(e: &mut Engine, this: Ptr<PathingLocation>) -> Ptr<PathingLocation> {
    construct_base(e, this);
    let point = this.at(PathingLocation::point);
    for f in [NiPoint3::x, NiPoint3::y, NiPoint3::z] {
        let v = fstp(fld_global(e, FLOAT_MAX));
        e.set(point, f, v);
    }
    e.set(this, PathingLocation::cell, 0);
    e.set(this, PathingLocation::world_space, 0);
    e.set(this, PathingLocation::triangle, NO_TRIANGLE);
    e.set(this, PathingLocation::cell_key, NO_CELL_KEY);
    e.set(this, PathingLocation::info, 0);
    e.set(this, PathingLocation::unknown_14, 0);
    e.set(this, PathingLocation::flags, 0);
    this
}

// Typed from 006dcee0
/// `PathingLocation::PathingLocation(point, cell, world space)` (`thiscall`): set by `006dd280`.
pub fn new_at(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: u32,
    cell: u32,
    world_space: u32,
) -> Ptr<PathingLocation> {
    construct_base(e, this);
    e.call(SET_FROM, &args![this, point, cell, world_space]);
    this
}

// Typed from 006dcf20
/// `PathingLocation::PathingLocation(point, info)` (`thiscall`): the point copied, the info, its
/// world space ([`parent_world_space`]) and cell key, or its interior cell (`006b77b0`); refreshed
/// (`006dcfe0`), no triangle.
pub fn new_in(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    point: Ptr<NiPoint3>,
    info: Ptr<NavMeshInfo>,
) -> Ptr<PathingLocation> {
    construct_base(e, this);
    copy_point(e, point, this.at(PathingLocation::point));
    e.set(this, PathingLocation::flags, 0);
    e.set(this, PathingLocation::info, info.addr());
    e.set(this, PathingLocation::unknown_14, 0);
    let w = typed_call!(FAMILY, 0x0069_0800, parent_world_space(e, info));
    e.set(this, PathingLocation::world_space, w);
    if e.get(this, PathingLocation::world_space) != 0 {
        e.set(this, PathingLocation::cell, 0);
        let key = e.get(info, NavMeshInfo::cell_key);
        e.set(this, PathingLocation::cell_key, key);
    } else {
        let c = e.call(INTERIOR_CELL, &args![info]).u32();
        e.set(this, PathingLocation::cell, c);
        e.set(this, PathingLocation::cell_key, NO_CELL_KEY);
    }
    e.call(REFRESH, &args![this, 1u32]);
    e.set(this, PathingLocation::triangle, NO_TRIANGLE);
    this
}

// Typed from 006dcc40
/// `PathingLocation`'s copy constructor (`thiscall(location, other)`).
pub fn copy(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    other: Ptr<PathingLocation>,
) -> Ptr<PathingLocation> {
    construct_base(e, this);
    copy_fields(e, this, other);
    this
}

// Typed from 006dcce0
/// `PathingLocation::operator=` (`thiscall(location, other)`).
pub fn assign(
    e: &mut Engine,
    this: Ptr<PathingLocation>,
    other: Ptr<PathingLocation>,
) -> Ptr<PathingLocation> {
    copy_fields(e, this, other);
    this
}

// Typed from 006d5320
/// `WORLD_LOCATION::WORLD_LOCATION` (`thiscall(this)`): `ReturnThis` on the point at +4.
pub fn world_location_new(e: &mut Engine, this: Ptr) -> Ptr {
    typed_call!(
        leaf::FAMILY,
        0x0068_15c0,
        leaf::same_word(e, this.byte_add(4))
    );
    this
}
