//! `fallout shared/pathfinding/pathingrequestcover.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006e41e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestCover` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `006dad70` and frees 0xec bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn pathing_request_cover_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x006d_ad70, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0xecu32]);
    }
    this
}

// ---- Batch n032: the cover request and the cover location -------------------

layout! {
    /// `PathingRequestCover` (Xbox PDB), 0xEC bytes: a `PathingRequest`
    /// (0xB0 bytes) and the cover search's parameters.
    pub struct PathingRequestCover: 0xEC {
        /// `Threat` (Xbox PDB): an `NiPoint3`.
        0xB0 Threat: Inline<crate::types::NiPoint3>,
        /// `fMaxCoverDistance` (Xbox PDB).
        0xBC fMaxCoverDistance: f32,
        /// `fActorCrouchHeight` (Xbox PDB).
        0xC0 fActorCrouchHeight: f32,
        /// `fMaxHeightChange` (Xbox PDB).
        0xC4 fMaxHeightChange: f32,
        /// `iMaxResults` (Xbox PDB).
        0xC8 iMaxResults: u32,
        /// `bCanCrouch` (Xbox PDB).
        0xCC bCanCrouch: u8,
        /// `bFindBothSides` (Xbox PDB).
        0xCD bFindBothSides: u8,
        /// `bFindBehindTarget` (Xbox PDB).
        0xCE bFindBehindTarget: u8,
        /// `bSortFarthestFirst` (Xbox PDB).
        0xCF bSortFarthestFirst: u8,
        /// `bSortDistanceFromTarget` (Xbox PDB).
        0xD0 bSortDistanceFromTarget: u8,
        /// `bHideCover` (Xbox PDB).
        0xD1 bHideCover: u8,
        /// `DistanceProjectionVector` (Xbox PDB): an `NiPoint3`.
        0xD4 DistanceProjectionVector: Inline<crate::types::NiPoint3>,
        /// `ActorLocation` (Xbox PDB): an `NiPoint3`.
        0xE0 ActorLocation: Inline<crate::types::NiPoint3>,
    }

    /// `PathingCoverLocation` (Xbox PDB), 0x54 bytes: a `PathingLocation`
    /// (0x28 bytes) and the cover edge.
    pub struct PathingCoverLocation: 0x54 {
        /// `LeftVertex` (Xbox PDB): an `NiPoint3`.
        0x28 LeftVertex: Inline<crate::types::NiPoint3>,
        /// `RightVertex` (Xbox PDB): an `NiPoint3`.
        0x34 RightVertex: Inline<crate::types::NiPoint3>,
        /// `EdgeDirection` (Xbox PDB): an `NiPoint3`.
        0x40 EdgeDirection: Inline<crate::types::NiPoint3>,
        /// `cCoverHeight` (Xbox PDB).
        0x4C cCoverHeight: u8,
        /// `bStandCover` (Xbox PDB).
        0x4D bStandCover: u8,
        /// `bHigherThanCrouch` (Xbox PDB).
        0x4E bHigherThanCrouch: u8,
        /// `bLeftEdgeOpen` (Xbox PDB).
        0x4F bLeftEdgeOpen: u8,
        /// `bRightEdgeOpen` (Xbox PDB).
        0x50 bRightEdgeOpen: u8,
        /// `cEdge` (Xbox PDB).
        0x51 cEdge: u8,
        /// `cState` (Xbox PDB).
        0x52 cState: u8,
    }
}

/// The `PathingRequestCover` vtable (by its RTTI).
const COVER_VTABLE: u32 = 0x0106_d29c;
/// `PathingRequest::PathingRequest` (Xbox PDB) and `PathingRequest::CopyTo`
/// (Xbox PDB), in `pathingrequest.cpp`.
const REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const REQUEST_COPY_TO: u32 = 0x006e_2750;
/// `006815c0` (`thiscall`): the `NiPoint3` constructor.
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// The constructor's defaults (`float`s, through the x87 stack): actor
/// radius `0104e0e8`, actor height `0101e704`, crouch height `0101e6ec`,
/// cover distance `010236e0`, height change `0106b158`; and the zero
/// point (`NiPoint3`, `011f426c`).
const DEFAULT_ACTOR_RADIUS: u32 = 0x0104_e0e8;
const DEFAULT_ACTOR_HEIGHT: u32 = 0x0101_e704;
const DEFAULT_CROUCH_HEIGHT: u32 = 0x0101_e6ec;
const DEFAULT_COVER_DISTANCE: u32 = 0x0102_36e0;
const DEFAULT_HEIGHT_CHANGE: u32 = 0x0106_b158;
const ZERO_POINT: u32 = 0x011f_426c;
/// The setters `CopyTo` calls on the other request outside this unit:
/// the threat (`006dad40`, `RET 4`: a point), the crouch height
/// (`005076c0`, `RET 4`: a float) and the result count (`004030d0`,
/// `RET 4`).
const SET_THREAT: u32 = 0x006d_ad40;
const SET_CROUCH_HEIGHT: u32 = 0x0050_76c0;
const SET_MAX_RESULTS: u32 = 0x0040_30d0;
/// `PathingLocation::operator=` (engine map), `thiscall`, `RET 4`.
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// `PathingLocation::SaveGame` / `LoadGame` (engine map), `RET 4`.
const LOCATION_SAVE_GAME: u32 = 0x006d_ef40;
const LOCATION_LOAD_GAME: u32 = 0x006d_f010;
/// `PathingLocation::GetNavMesh` (engine map) and
/// `PathingLocation::GetNavMeshAndTriangle` (engine map), `thiscall`.
const LOCATION_GET_NAV_MESH: u32 = 0x006d_d610;
const LOCATION_GET_NAV_MESH_AND_TRIANGLE: u32 = 0x006d_d640;
/// `006df760` (`thiscall`): a location's triangle (16 bits).
const LOCATION_TRIANGLE: u32 = 0x006d_f760;
/// The `NiPointer` constructors (`0042fb00` null, `00633c90` from a
/// pointer) and destructors (`0042fa40`, `0045cec0`); `00559450`: its target.
const NI_POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const NI_POINTER_FROM: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT: u32 = 0x0042_fa40;
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
const WORD_AT: u32 = 0x0055_9450;
/// `0084e3a0` (`thiscall`): a navmesh's key.
const NAV_MESH_KEY: u32 = 0x0084_e3a0;
/// `006997b0` (`thiscall`, `RET 8`): a navmesh triangle's cover object
/// into an `NiPointer` (triangle, out); answers whether there is one.
const TRIANGLE_COVER: u32 = 0x0069_97b0;
/// `00439ef0` (`thiscall`, `RET 8`): `NiPoint3` difference into an out
/// point (out, other); `004a7290` (`thiscall`): a point's length in ST0.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_LENGTH: u32 = 0x004a_7290;
/// `BGSSaveGameBuffer` raw save (`00865e50`: data, size, 0) and
/// `BGSLoadGameBuffer` raw load (`00864980`: data, size).
const SAVE_DATA: u32 = 0x0086_5e50;
const LOAD_DATA: u32 = 0x0086_4980;

/// `FLD float [address]; FSTP float [..]`: the bits (a signalling NaN
/// goes quiet).
fn float_copy(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

// Translated from 006e4080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e4080 - the C++ exception frame (FS:[0]) that destroys the base on unwinding is not translated
/// `PathingRequestCover::PathingRequestCover` (Xbox PDB): the
/// `PathingRequest` base, the vtable, the three points, then the defaults:
/// actor radius and height, crouch height, cover distance and height
/// change from the constants, 50 results, `bCanCrouch` set and the other
/// flags clear, the projection vector and the actor location zero.
/// Returns `this`.
pub fn pathing_request_cover_pathing_request_cover(
    e: &mut Engine,
    this: Ptr<PathingRequestCover>,
) -> Ptr<PathingRequestCover> {
    let base = this.addr();
    e.call(REQUEST_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, COVER_VTABLE);
    e.call(POINT_CONSTRUCT, &args![base + 0xb0]);
    e.call(POINT_CONSTRUCT, &args![base + 0xd4]);
    e.call(POINT_CONSTRUCT, &args![base + 0xe0]);
    for (offset, constant) in [
        (0x68u32, DEFAULT_ACTOR_RADIUS),
        (0x6c, DEFAULT_ACTOR_HEIGHT),
        (0xc0, DEFAULT_CROUCH_HEIGHT),
        (0xbc, DEFAULT_COVER_DISTANCE),
        (0xc4, DEFAULT_HEIGHT_CHANGE),
    ] {
        let value = float_copy(e, constant);
        e.mem.set_u32(base + offset, value);
    }
    e.set(this, PathingRequestCover::iMaxResults, 0x32);
    e.set(this, PathingRequestCover::bCanCrouch, 1);
    e.set(this, PathingRequestCover::bFindBothSides, 0);
    e.set(this, PathingRequestCover::bFindBehindTarget, 0);
    e.set(this, PathingRequestCover::bSortFarthestFirst, 0);
    e.set(this, PathingRequestCover::bSortDistanceFromTarget, 0);
    e.set(this, PathingRequestCover::bHideCover, 0);
    for point in [0xd4u32, 0xe0] {
        for i in 0..3 {
            let word = e.mem.u32(ZERO_POINT + 4 * i);
            e.mem.set_u32(base + point + 4 * i, word);
        }
    }
    this
}

// Translated from 006e4220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestCover::CopyTo` (Xbox PDB): the `PathingRequest` part
/// (`PathingRequest::CopyTo`), then the threat, crouch height, cover
/// distance, height change, result count, the flags from
/// `bFindBothSides` to `bHideCover` and the projection vector through the
/// other request's setters (`bCanCrouch` and the actor location are not
/// copied; floats go through the x87 stack).
pub fn pathing_request_cover_copy_to(e: &mut Engine, this: Ptr<PathingRequestCover>, other: Ptr) {
    let base = this.addr();
    e.call(REQUEST_COPY_TO, &args![this, other]);
    e.call(SET_THREAT, &args![other, base + 0xb0]);
    let crouch = float_copy(e, base + 0xc0);
    e.call(SET_CROUCH_HEIGHT, &args![other, crouch]);
    let distance = f32::from_bits(float_copy(e, base + 0xbc));
    fn_006e4320(e, other.cast(), distance);
    let change = f32::from_bits(float_copy(e, base + 0xc4));
    fn_006e4340(e, other.cast(), change);
    let results = e.get(this, PathingRequestCover::iMaxResults);
    e.call(SET_MAX_RESULTS, &args![other, results]);
    let other = other.cast::<PathingRequestCover>();
    let flag = e.get(this, PathingRequestCover::bFindBothSides);
    fn_006e4360(e, other, flag);
    let flag = e.get(this, PathingRequestCover::bFindBehindTarget);
    fn_006e4380(e, other, flag);
    let flag = e.get(this, PathingRequestCover::bSortFarthestFirst);
    fn_006e43a0(e, other, flag);
    let flag = e.get(this, PathingRequestCover::bSortDistanceFromTarget);
    fn_006e43c0(e, other, flag);
    let flag = e.get(this, PathingRequestCover::bHideCover);
    fn_006e43e0(e, other, flag);
    fn_006e4400(e, other, base + 0xd4);
}

// Translated from 006e4320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fMaxCoverDistance` (+0xBC; through the x87 stack).
pub fn fn_006e4320(e: &mut Engine, this: Ptr<PathingRequestCover>, value: f32) {
    e.mem
        .set_u32(this.addr() + 0xbc, x87_quiet_bits(value.to_bits()));
}

// Translated from 006e4340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fMaxHeightChange` (+0xC4; through the x87 stack).
pub fn fn_006e4340(e: &mut Engine, this: Ptr<PathingRequestCover>, value: f32) {
    e.mem
        .set_u32(this.addr() + 0xc4, x87_quiet_bits(value.to_bits()));
}

// Translated from 006e4360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bFindBothSides` (+0xCD).
pub fn fn_006e4360(e: &mut Engine, this: Ptr<PathingRequestCover>, value: u8) {
    e.set(this, PathingRequestCover::bFindBothSides, value);
}

// Translated from 006e4380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bFindBehindTarget` (+0xCE).
pub fn fn_006e4380(e: &mut Engine, this: Ptr<PathingRequestCover>, value: u8) {
    e.set(this, PathingRequestCover::bFindBehindTarget, value);
}

// Translated from 006e43a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bSortFarthestFirst` (+0xCF).
pub fn fn_006e43a0(e: &mut Engine, this: Ptr<PathingRequestCover>, value: u8) {
    e.set(this, PathingRequestCover::bSortFarthestFirst, value);
}

// Translated from 006e43c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bSortDistanceFromTarget` (+0xD0).
pub fn fn_006e43c0(e: &mut Engine, this: Ptr<PathingRequestCover>, value: u8) {
    e.set(this, PathingRequestCover::bSortDistanceFromTarget, value);
}

// Translated from 006e43e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bHideCover` (+0xD1).
pub fn fn_006e43e0(e: &mut Engine, this: Ptr<PathingRequestCover>, value: u8) {
    e.set(this, PathingRequestCover::bHideCover, value);
}

// Translated from 006e4400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `DistanceProjectionVector` (+0xD4) from three words at `vector`.
pub fn fn_006e4400(e: &mut Engine, this: Ptr<PathingRequestCover>, vector: u32) {
    for i in 0..3 {
        let word = e.mem.u32(vector + 4 * i);
        e.mem.set_u32(this.addr() + 0xd4 + 4 * i, word);
    }
}

// Translated from 006e4430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `PathingCoverLocation` assignment (by its body): the location part
/// (`PathingLocation::operator=`), the three edge points (word copies) and
/// the bytes from `cCoverHeight` to `cState`. Returns `this`.
pub fn fn_006e4430(
    e: &mut Engine,
    this: Ptr<PathingCoverLocation>,
    other: Ptr<PathingCoverLocation>,
) -> Ptr<PathingCoverLocation> {
    e.call(LOCATION_ASSIGN, &args![this, other]);
    for offset in [0x28u32, 0x34, 0x40] {
        for i in 0..3 {
            let word = e.mem.u32(other.addr() + offset + 4 * i);
            e.mem.set_u32(this.addr() + offset + 4 * i, word);
        }
    }
    for offset in 0x4cu32..=0x52 {
        let byte = e.mem.u8(other.addr() + offset);
        e.mem.set_u8(this.addr() + offset, byte);
    }
    this
}

// Translated from 006e4500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingCoverLocation`, `RET 0x18`): `cCoverHeight` is the low
/// byte of `height`; `bStandCover` whether `height >= stand_height` and
/// `bHigherThanCrouch` whether `height > crouch_height` (16-bit, unsigned);
/// the two edge flags and `cEdge` as given; `cState` 0.
pub fn fn_006e4500(
    e: &mut Engine,
    this: Ptr<PathingCoverLocation>,
    height: u16,
    stand_height: u16,
    crouch_height: u16,
    left_edge_open: u8,
    right_edge_open: u8,
    edge: u8,
) {
    e.set(this, PathingCoverLocation::cCoverHeight, height as u8);
    e.set(
        this,
        PathingCoverLocation::bStandCover,
        u8::from(height >= stand_height),
    );
    e.set(
        this,
        PathingCoverLocation::bHigherThanCrouch,
        u8::from(height > crouch_height),
    );
    e.set(this, PathingCoverLocation::bLeftEdgeOpen, left_edge_open);
    e.set(this, PathingCoverLocation::bRightEdgeOpen, right_edge_open);
    e.set(this, PathingCoverLocation::cEdge, edge);
    e.set(this, PathingCoverLocation::cState, 0);
}

// Translated from 006e4570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e4570 - the C++ exception frame (FS:[0]) that destroys the NiPointer on unwinding is not translated
/// Unnamed (`PathingCoverLocation`): the cover's key ([`fn_006e4630`] of
/// the navmesh's key, the triangle and `cEdge`) when the location has a
/// navmesh and triangle (`PathingLocation::GetNavMeshAndTriangle`), else 0.
pub fn fn_006e4570(e: &mut Engine, this: Ptr<PathingCoverLocation>) -> u32 {
    e.with_stack(8, |e, locals| {
        let (nav_mesh, triangle) = (locals, locals.addr() + 4);
        e.call(NI_POINTER_CONSTRUCT, &args![nav_mesh]);
        e.mem.set_u16(triangle, 0);
        let key = if e
            .call(
                LOCATION_GET_NAV_MESH_AND_TRIANGLE,
                &args![this, nav_mesh, triangle],
            )
            .bool()
        {
            let edge = e.get(this, PathingCoverLocation::cEdge);
            let triangle = e.mem.u16(triangle);
            let mesh = e.call(WORD_AT, &args![nav_mesh]).u32();
            let mesh_key = e.call(NAV_MESH_KEY, &args![mesh]).u32();
            fn_006e4630(e, mesh_key, triangle, edge)
        } else {
            0
        };
        e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
        key
    })
}

// Translated from 006e4630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`): a cover key, `(nav_mesh_key << 16) + 4 * triangle +
/// edge` (wrapping).
pub fn fn_006e4630(_e: &mut Engine, nav_mesh_key: u32, triangle: u16, edge: u8) -> u32 {
    (nav_mesh_key << 16)
        .wrapping_add(u32::from(triangle) * 4)
        .wrapping_add(u32::from(edge))
}

/// The cover object of the location's triangle into `cover` (the
/// location's navmesh, `006df760`, `006997b0`); `None` without a navmesh.
fn triangle_cover(e: &mut Engine, this: Ptr, nav_mesh: Ptr, cover: Ptr) -> Option<bool> {
    if !e.call(LOCATION_GET_NAV_MESH, &args![this, nav_mesh]).bool() {
        return None;
    }
    e.call(NI_POINTER_FROM, &args![cover, 0u32]);
    let triangle = e.call(LOCATION_TRIANGLE, &args![this]).u16();
    let mesh = e.call(WORD_AT, &args![nav_mesh]).u32();
    Some(
        e.call(TRIANGLE_COVER, &args![mesh, u32::from(triangle), cover])
            .bool(),
    )
}

// Translated from 006e4650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e4650 - the C++ exception frame (FS:[0]) that releases the NiPointers on unwinding is not translated
/// Unnamed (`PathingCoverLocation`): whether the cover object of the
/// location's triangle has `owner` at its +0x70 (false without a navmesh
/// or a cover object).
pub fn fn_006e4650(e: &mut Engine, this: Ptr<PathingCoverLocation>, owner: u32) -> bool {
    e.with_stack(8, |e, locals| {
        let (nav_mesh, cover) = (locals, locals.byte_add(4));
        e.call(NI_POINTER_CONSTRUCT, &args![nav_mesh]);
        let result = match triangle_cover(e, this.cast(), nav_mesh, cover) {
            None => false,
            Some(found) => {
                let result = if found {
                    let object = e.call(WORD_AT, &args![cover]).u32();
                    e.mem.u32(object + 0x70) == owner
                } else {
                    false
                };
                e.call(NI_POINTER_RELEASE, &args![cover]);
                result
            }
        };
        e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
        result
    })
}

// Translated from 006e4740 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006e4740 - the C++ exception frame (FS:[0]) that releases the NiPointers on unwinding is not translated
/// Unnamed (`PathingCoverLocation`): the word at +0x70 of the cover object
/// of the location's triangle, `0xFFFFFFFF` without a navmesh or a cover
/// object.
pub fn fn_006e4740(e: &mut Engine, this: Ptr<PathingCoverLocation>) -> u32 {
    e.with_stack(8, |e, locals| {
        let (nav_mesh, cover) = (locals, locals.byte_add(4));
        e.call(NI_POINTER_CONSTRUCT, &args![nav_mesh]);
        let result = match triangle_cover(e, this.cast(), nav_mesh, cover) {
            None => 0xffff_ffff,
            Some(found) => {
                let result = if found {
                    let object = e.call(WORD_AT, &args![cover]).u32();
                    e.mem.u32(object + 0x70)
                } else {
                    0xffff_ffff
                };
                e.call(NI_POINTER_RELEASE, &args![cover]);
                result
            }
        };
        e.call(NI_POINTER_DESTRUCT, &args![nav_mesh]);
        result
    })
}

// Translated from 006e4820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingCoverLocation`): whether `cCoverHeight` is 2.
pub fn fn_006e4820(e: &mut Engine, this: Ptr<PathingCoverLocation>) -> bool {
    e.get(this, PathingCoverLocation::cCoverHeight) == 2
}

// Translated from 006e4840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`PathingCoverLocation`): whether the right vertex is farther
/// from `point` than the left one (`004a7290` lengths of the differences,
/// the first stored as a `double`; false when equal or NaN).
pub fn fn_006e4840(e: &mut Engine, this: Ptr<PathingCoverLocation>, point: u32) -> bool {
    e.with_stack(0x18, |e, locals| {
        let (left, right) = (locals.addr(), locals.addr() + 0xc);
        e.call(POINT_SUBTRACT, &args![this.addr() + 0x28, left, point]);
        e.call(POINT_SUBTRACT, &args![this.addr() + 0x34, right, point]);
        let left_length = e.call(POINT_LENGTH, &args![left]).f64();
        let right_length = e.call(POINT_LENGTH, &args![right]).f64();
        // FCOMP; TEST AH,0x41; JNZ: true only when ST0 > the stored value.
        right_length > left_length
    })
}

// Translated from 006e48a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingCoverLocation::SaveGame` (Xbox PDB): the location part
/// (`PathingLocation::SaveGame`), the three edge points (0xC bytes each)
/// and the bytes from `cCoverHeight` to `cState`.
pub fn pathing_cover_location_save_game(
    e: &mut Engine,
    this: Ptr<PathingCoverLocation>,
    buffer: u32,
) {
    let base = this.addr();
    e.call(LOCATION_SAVE_GAME, &args![this, buffer]);
    for offset in [0x28u32, 0x34, 0x40] {
        e.call(SAVE_DATA, &args![buffer, base + offset, 0xcu32, 0u32]);
    }
    for offset in 0x4cu32..=0x52 {
        e.call(SAVE_DATA, &args![buffer, base + offset, 1u32, 0u32]);
    }
}

// Translated from 006e4980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingCoverLocation::LoadGame` (Xbox PDB): the reverse of
/// [`pathing_cover_location_save_game`].
pub fn pathing_cover_location_load_game(
    e: &mut Engine,
    this: Ptr<PathingCoverLocation>,
    buffer: u32,
) {
    let base = this.addr();
    e.call(LOCATION_LOAD_GAME, &args![this, buffer]);
    for offset in [0x28u32, 0x34, 0x40] {
        e.call(LOAD_DATA, &args![buffer, base + offset, 0xcu32]);
    }
    for offset in 0x4cu32..=0x52 {
        e.call(LOAD_DATA, &args![buffer, base + offset, 1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006e_41e0, pathing_request_cover_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x006e_4080,
            pathing_request_cover_pathing_request_cover(
                Ptr<PathingRequestCover>,
            ) -> Ptr<PathingRequestCover>
        ),
        entry!(
            0x006e_4220,
            pathing_request_cover_copy_to(Ptr<PathingRequestCover>, Ptr)
        ),
        entry!(0x006e_4320, fn_006e4320(Ptr<PathingRequestCover>, f32)),
        entry!(0x006e_4340, fn_006e4340(Ptr<PathingRequestCover>, f32)),
        entry!(0x006e_4360, fn_006e4360(Ptr<PathingRequestCover>, u8)),
        entry!(0x006e_4380, fn_006e4380(Ptr<PathingRequestCover>, u8)),
        entry!(0x006e_43a0, fn_006e43a0(Ptr<PathingRequestCover>, u8)),
        entry!(0x006e_43c0, fn_006e43c0(Ptr<PathingRequestCover>, u8)),
        entry!(0x006e_43e0, fn_006e43e0(Ptr<PathingRequestCover>, u8)),
        entry!(0x006e_4400, fn_006e4400(Ptr<PathingRequestCover>, u32)),
        entry!(
            0x006e_4430,
            fn_006e4430(
                Ptr<PathingCoverLocation>,
                Ptr<PathingCoverLocation>,
            ) -> Ptr<PathingCoverLocation>
        ),
        entry!(
            0x006e_4500,
            fn_006e4500(Ptr<PathingCoverLocation>, u16, u16, u16, u8, u8, u8)
        ),
        entry!(0x006e_4570, fn_006e4570(Ptr<PathingCoverLocation>) -> u32),
        entry!(0x006e_4630, fn_006e4630(u32, u16, u8) -> u32),
        entry!(
            0x006e_4650,
            fn_006e4650(Ptr<PathingCoverLocation>, u32) -> bool
        ),
        entry!(0x006e_4740, fn_006e4740(Ptr<PathingCoverLocation>) -> u32),
        entry!(0x006e_4820, fn_006e4820(Ptr<PathingCoverLocation>) -> bool),
        entry!(
            0x006e_4840,
            fn_006e4840(Ptr<PathingCoverLocation>, u32) -> bool
        ),
        entry!(
            0x006e_48a0,
            pathing_cover_location_save_game(Ptr<PathingCoverLocation>, u32)
        ),
        entry!(
            0x006e_4980,
            pathing_cover_location_load_game(Ptr<PathingCoverLocation>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn pathing_request_cover_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x006e_41e0, 0x006d_ad70, 0xec);
    }
}

#[cfg(test)]
mod n032_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn take(e: &mut Engine) -> Log {
        e.call_log.take().unwrap()
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = Engine::new();
        stub(&mut e, &[REQUEST_CONSTRUCT, POINT_CONSTRUCT]);
        for (address, value) in [
            (DEFAULT_ACTOR_RADIUS, 24.0f32.to_bits()),
            (DEFAULT_ACTOR_HEIGHT, 128.0f32.to_bits()),
            (DEFAULT_CROUCH_HEIGHT, 64.0f32.to_bits()),
            (DEFAULT_COVER_DISTANCE, 0x7f80_0001),
            (DEFAULT_HEIGHT_CHANGE, 180.0f32.to_bits()),
        ] {
            e.map(address, 4);
            e.mem.set_u32(address, value);
        }
        e.map(ZERO_POINT, 0xc);
        e.mem.set_u32(ZERO_POINT + 8, 0x8000_0000);
        let this = e.mem.alloc(0xec);
        e.mem.write(this, &[0xaa; 0xec]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_4080, &args![this]).u32(), this);
        let log = take(&mut e);
        assert_eq!(calls(&log, REQUEST_CONSTRUCT), vec![vec![this]]);
        assert_eq!(
            calls(&log, POINT_CONSTRUCT),
            vec![vec![this + 0xb0], vec![this + 0xd4], vec![this + 0xe0]]
        );
        assert_eq!(e.mem.u32(this), COVER_VTABLE);
        assert_eq!(e.mem.f32(this + 0x68), 24.0);
        assert_eq!(e.mem.f32(this + 0x6c), 128.0);
        assert_eq!(e.mem.f32(this + 0xc0), 64.0);
        assert_eq!(e.mem.u32(this + 0xbc), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0xc4), 180.0);
        assert_eq!(e.mem.u32(this + 0xc8), 50);
        assert_eq!(e.mem.bytes(this + 0xcc, 6), [1, 0, 0, 0, 0, 0]);
        assert_eq!(e.mem.u32(this + 0xdc), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0xe8), 0x8000_0000);
        assert_eq!(e.mem.u32(this + 0xd4), 0);
    }

    #[test]
    fn copy_to_copies_the_cover_parameters() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[
                REQUEST_COPY_TO,
                SET_THREAT,
                SET_CROUCH_HEIGHT,
                SET_MAX_RESULTS,
            ],
        );
        let from = e.mem.alloc(0xec);
        let to = e.mem.alloc(0xec);
        e.mem.set_f32(from + 0xc0, 64.0);
        e.mem.set_u32(from + 0xbc, 0x7f80_0001);
        e.mem.set_f32(from + 0xc4, 3.0);
        e.mem.set_u32(from + 0xc8, 7);
        e.mem.write(from + 0xcc, &[9, 1, 2, 3, 4, 5]);
        e.mem.set_u32(from + 0xd4, 11);
        e.mem.set_u32(from + 0xd8, 12);
        e.mem.set_u32(from + 0xdc, 13);
        e.call_log = Some(vec![]);
        e.call(0x006e_4220, &args![from, to]);
        let log = take(&mut e);
        assert_eq!(calls(&log, REQUEST_COPY_TO), vec![vec![from, to]]);
        assert_eq!(calls(&log, SET_THREAT), vec![vec![to, from + 0xb0]]);
        assert_eq!(
            calls(&log, SET_CROUCH_HEIGHT),
            vec![vec![to, 64.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, SET_MAX_RESULTS), vec![vec![to, 7]]);
        assert_eq!(e.mem.u32(to + 0xbc), 0x7fc0_0001);
        assert_eq!(e.mem.f32(to + 0xc4), 3.0);
        // bCanCrouch is not copied.
        assert_eq!(e.mem.bytes(to + 0xcc, 6), [0, 1, 2, 3, 4, 5]);
        assert_eq!(
            [
                e.mem.u32(to + 0xd4),
                e.mem.u32(to + 0xd8),
                e.mem.u32(to + 0xdc)
            ],
            [11, 12, 13]
        );
    }

    #[test]
    fn setters_store_their_fields() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xec);
        e.call(0x006e_4320, &args![this, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(this + 0xbc), 0x7fc0_0001);
        e.call(0x006e_4340, &args![this, 2.5f32]);
        assert_eq!(e.mem.f32(this + 0xc4), 2.5);
        for (address, offset) in [
            (0x006e_4360u32, 0xcdu32),
            (0x006e_4380, 0xce),
            (0x006e_43a0, 0xcf),
            (0x006e_43c0, 0xd0),
            (0x006e_43e0, 0xd1),
        ] {
            e.call(address, &args![this, 0x1_0107u32]);
            assert_eq!(e.mem.u8(this + offset), 7);
        }
        let vector = e.mem.alloc(0xc);
        e.mem.write(vector, &[1; 0xc]);
        e.call(0x006e_4400, &args![this, vector]);
        assert_eq!(e.mem.bytes(this + 0xd4, 0xc), [1; 0xc]);
    }

    #[test]
    fn cover_location_assignment_copies_the_edge() {
        let mut e = Engine::new();
        stub(&mut e, &[LOCATION_ASSIGN]);
        let from = e.mem.alloc(0x54);
        let to = e.mem.alloc(0x54);
        let bytes: Vec<u8> = (0..0x54).map(|i| i as u8).collect();
        e.mem.write(from, &bytes);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_4430, &args![to, from]).u32(), to);
        assert_eq!(calls(&take(&mut e), LOCATION_ASSIGN), vec![vec![to, from]]);
        assert_eq!(e.mem.bytes(to + 0x28, 0x2b), &bytes[0x28..0x53]);
        assert_eq!(e.mem.bytes(to, 0x28), [0; 0x28]);
        assert_eq!(e.mem.u8(to + 0x53), 0);
    }

    #[test]
    fn cover_location_set_up_compares_heights_unsigned() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x54);
        e.mem.set_u8(this + 0x52, 9);
        e.call(
            0x006e_4500,
            &args![this, 0x0102u32, 0x0102u32, 0x0103u32, 1u32, 0u32, 3u32],
        );
        assert_eq!(e.mem.bytes(this + 0x4c, 7), [2, 1, 0, 1, 0, 3, 0]);
        // 0x8000 is above 0x7fff (unsigned 16-bit).
        e.call(
            0x006e_4500,
            &args![this, 0x8000u32, 0x8001u32, 0x7fffu32, 0u32, 1u32, 0u32],
        );
        assert_eq!(e.mem.bytes(this + 0x4c, 7), [0, 0, 1, 0, 1, 0, 0]);
    }

    #[test]
    fn cover_key_uses_the_nav_mesh_triangle_and_edge() {
        for found in [false, true] {
            let mut e = Engine::new();
            stub(&mut e, &[NI_POINTER_CONSTRUCT, NI_POINTER_DESTRUCT]);
            e.register_double(LOCATION_GET_NAV_MESH_AND_TRIANGLE, move |e, a| {
                assert_eq!(e.mem.u16(a[2]), 0);
                e.mem.set_u32(a[1], 0x4444);
                e.mem.set_u16(a[2], 0x10);
                found.into_ret()
            });
            e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
            e.register(NAV_MESH_KEY, |_, a| (a[0] + 1).into_ret());
            let this = e.mem.alloc(0x54);
            e.mem.set_u8(this + 0x51, 3);
            e.call_log = Some(vec![]);
            let key = e.call(0x006e_4570, &args![this]).u32();
            let expected = if found { (0x4445 << 16) + 0x40 + 3 } else { 0 };
            assert_eq!(key, expected);
            assert_eq!(calls(&take(&mut e), NI_POINTER_DESTRUCT).len(), 1);
        }
        let mut e = Engine::new();
        let key = e
            .call(0x006e_4630, &args![0x1_0001u32, 0x1_ffffu32, 0x1ffu32])
            .u32();
        assert_eq!(
            key,
            0x0001_0000u32.wrapping_add(0xffff * 4).wrapping_add(0xff)
        );
    }

    fn cover_object_doubles(e: &mut Engine, has_mesh: bool, has_cover: bool) -> u32 {
        stub(
            e,
            &[
                NI_POINTER_CONSTRUCT,
                NI_POINTER_DESTRUCT,
                NI_POINTER_RELEASE,
            ],
        );
        e.register_double(LOCATION_GET_NAV_MESH, move |e, a| {
            e.mem.set_u32(a[1], 0x4444);
            has_mesh.into_ret()
        });
        e.register(NI_POINTER_FROM, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(LOCATION_TRIANGLE, |_, _| 0x1_0005u32.into_ret());
        e.register(WORD_AT, |e, a| e.mem.u32(a[0]).into_ret());
        let object = e.mem.alloc(0x74);
        e.mem.set_u32(object + 0x70, 0x1234);
        e.register_double(TRIANGLE_COVER, move |e, a| {
            assert_eq!(a[..2], [0x4444, 5]);
            e.mem.set_u32(a[2], object);
            has_cover.into_ret()
        });
        object
    }

    #[test]
    fn cover_owner_test_and_value() {
        for (has_mesh, has_cover, owner, expected, value) in [
            (false, true, 0x1234u32, false, 0xffff_ffffu32),
            (true, false, 0x1234, false, 0xffff_ffff),
            (true, true, 0x1235, false, 0x1234),
            (true, true, 0x1234, true, 0x1234),
        ] {
            let mut e = Engine::new();
            cover_object_doubles(&mut e, has_mesh, has_cover);
            let this = e.mem.alloc(0x54);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e_4650, &args![this, owner]).bool(), expected);
            let log = take(&mut e);
            assert_eq!(calls(&log, NI_POINTER_DESTRUCT).len(), 1);
            assert_eq!(calls(&log, NI_POINTER_RELEASE).len(), usize::from(has_mesh));
            assert_eq!(e.call(0x006e_4740, &args![this]).u32(), value);
        }
    }

    #[test]
    fn crouch_cover_is_height_two() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x54);
        assert!(!e.call(0x006e_4820, &args![this]).bool());
        e.mem.set_u8(this + 0x4c, 2);
        assert!(e.call(0x006e_4820, &args![this]).bool());
    }

    #[test]
    fn right_vertex_farther_compares_the_lengths() {
        for (left, right, expected) in [
            (1.0f64, 2.0f64, true),
            (2.0, 1.0, false),
            (2.0, 2.0, false),
            (f64::NAN, 2.0, false),
        ] {
            let mut e = Engine::new();
            e.register(POINT_SUBTRACT, |e, a| {
                // The out point remembers which vertex it came from.
                e.mem.set_u32(a[1], a[0]);
                a[1].into_ret()
            });
            let this = e.mem.alloc(0x54);
            e.register_double(POINT_LENGTH, move |e, a| {
                let vertex = e.mem.u32(a[0]);
                let length = if vertex == this + 0x28 { left } else { right };
                Ret {
                    st0: length,
                    ..Ret::default()
                }
            });
            assert_eq!(e.call(0x006e_4840, &args![this, 0x900u32]).bool(), expected);
        }
    }

    #[test]
    fn cover_location_save_and_load() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[SAVE_DATA, LOAD_DATA, LOCATION_SAVE_GAME, LOCATION_LOAD_GAME],
        );
        let this = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        e.call(0x006e_48a0, &args![this, 0x77u32]);
        e.call(0x006e_4980, &args![this, 0x77u32]);
        let log = take(&mut e);
        assert_eq!(calls(&log, LOCATION_SAVE_GAME), vec![vec![this, 0x77]]);
        assert_eq!(calls(&log, LOCATION_LOAD_GAME), vec![vec![this, 0x77]]);
        let saved: Vec<(u32, u32)> = calls(&log, SAVE_DATA)
            .iter()
            .map(|c| (c[1] - this, c[2]))
            .collect();
        let loaded: Vec<(u32, u32)> = calls(&log, LOAD_DATA)
            .iter()
            .map(|c| (c[1] - this, c[2]))
            .collect();
        let expected: Vec<(u32, u32)> = vec![
            (0x28, 0xc),
            (0x34, 0xc),
            (0x40, 0xc),
            (0x4c, 1),
            (0x4d, 1),
            (0x4e, 1),
            (0x4f, 1),
            (0x50, 1),
            (0x51, 1),
            (0x52, 1),
        ];
        assert_eq!(saved, expected);
        assert_eq!(loaded, expected);
    }
}
