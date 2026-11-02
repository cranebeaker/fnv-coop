//! `SpeedTree/branchinfo.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `SIdvBranchInfo` is one level of a tree's branch description: segment
//! counts, tiling, flaring and radial/length parameters, and nine optional
//! `CIdvBezierSpline` curves that it owns (it deletes a curve it replaces
//! and all of them when destroyed).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new(size)` (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(pointer)` (`cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `std::basic_string<char>::basic_string(const char*)` (`thiscall`).
const STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
/// The exception constructor `Parse` throws with (`thiscall`: the string
/// and a `bool`; unnamed in the engine map, `SpeedTree/speedtreert.cpp`).
const PARSE_ERROR_FROM_STRING: u32 = 0x00b0_9480;
/// `_CxxThrowException(object, throw_info)`.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `CIdvBezierSpline::CIdvBezierSpline()` (`thiscall`, `00b37590`; the
/// default constructor, in its own run of the engine map).
const SPLINE_CONSTRUCTOR: u32 = 0x00b3_7590;
/// `CIdvBezierSpline::~CIdvBezierSpline` (Xbox PDB, `thiscall`).
const SPLINE_DESTRUCTOR: u32 = 0x00b3_7880;
/// `CTreeFileAccess::ParseInt` (Xbox PDB, `thiscall`).
const PARSE_INT: u32 = 0x00b3_ed80;
/// `CTreeFileAccess::ParseFloat` (Xbox PDB, `thiscall`; `float` in ST0).
const PARSE_FLOAT: u32 = 0x00b3_ed20;
/// `CTreeFileAccess::ParseBranchParameter` (Xbox PDB, `thiscall`): reads a
/// `CIdvBezierSpline` and returns it.
const PARSE_BRANCH_PARAMETER: u32 = 0x00b1_3970;
/// The checked element access of the tree file's byte vector (`thiscall`:
/// the vector at the file plus 4, the index; returns the element's
/// address; unnamed in the engine map).
const BYTE_VECTOR_ELEMENT: u32 = 0x00b1_5170;

/// `0.3f`: the default of the first/last branch positions and radial
/// distance.
const DEFAULT_THIRTY_PERCENT: u32 = 0x0101_7870;
/// `30.0f`: the default radial influence.
const DEFAULT_RADIAL_INFLUENCE: u32 = 0x0101_8f5c;
/// `10.0f`: the default radial influence variance.
const DEFAULT_RADIAL_INFLUENCE_VARIANCE: u32 = 0x0101_7b78;
/// `0.5f`: the default radial distance.
const DEFAULT_HALF: u32 = 0x0101_6248;
/// `0.25f`: the default radial variance.
const DEFAULT_QUARTER: u32 = 0x0101_622c;
/// `0.1f`: the default length variance.
const DEFAULT_TENTH: u32 = 0x0101_e2bc;

/// `"malformed branch data"`.
const MESSAGE_MALFORMED_BRANCH: u32 = 0x010a_54cc;
/// `"malformed general branch information"`.
const MESSAGE_MALFORMED_GENERAL: u32 = 0x010a_54a4;
/// The throw information of the parse exceptions.
const PARSE_THROW_INFO: u32 = 0x0117_0384;

layout! {
    /// `SIdvBranchInfo` (Xbox PDB), 0x74 bytes; the PC layout is the same.
    pub struct SIdvBranchInfo: 0x74 {
        /// `m_nCrossSectionSegments` (Xbox PDB).
        0x00 m_nCrossSectionSegments: i32,
        /// `m_nSegments` (Xbox PDB).
        0x04 m_nSegments: i32,
        /// `m_fFirstBranch` (Xbox PDB).
        0x08 m_fFirstBranch: f32,
        /// `m_fLastBranch` (Xbox PDB).
        0x0C m_fLastBranch: f32,
        /// `m_fFrequency` (Xbox PDB).
        0x10 m_fFrequency: f32,
        /// `m_fSTile` (Xbox PDB).
        0x14 m_fSTile: f32,
        /// `m_fTTile` (Xbox PDB).
        0x18 m_fTTile: f32,
        /// `m_bSTileAbsolute` (Xbox PDB).
        0x1C m_bSTileAbsolute: bool,
        /// `m_bTTileAbsolute` (Xbox PDB).
        0x1D m_bTTileAbsolute: bool,
        /// `m_bRandomTCoordOffset` (Xbox PDB).
        0x1E m_bRandomTCoordOffset: bool,
        /// `m_fTwist` (Xbox PDB).
        0x20 m_fTwist: f32,
        /// `m_fSegmentPackingExponent` (Xbox PDB).
        0x24 m_fSegmentPackingExponent: f32,
        /// `m_nNumFlares` (Xbox PDB).
        0x28 m_nNumFlares: i32,
        /// `m_fFlareBalance` (Xbox PDB).
        0x2C m_fFlareBalance: f32,
        /// `m_fRadialInfluence` (Xbox PDB).
        0x30 m_fRadialInfluence: f32,
        /// `m_fRadialInfluenceVariance` (Xbox PDB).
        0x34 m_fRadialInfluenceVariance: f32,
        /// `m_fRadialExponent` (Xbox PDB).
        0x38 m_fRadialExponent: f32,
        /// `m_fRadialDistance` (Xbox PDB).
        0x3C m_fRadialDistance: f32,
        /// `m_fRadialVariance` (Xbox PDB).
        0x40 m_fRadialVariance: f32,
        /// `m_fLengthDistance` (Xbox PDB).
        0x44 m_fLengthDistance: f32,
        /// `m_fLengthVariance` (Xbox PDB).
        0x48 m_fLengthVariance: f32,
        /// `m_fLengthExponent` (Xbox PDB).
        0x4C m_fLengthExponent: f32,
        /// `m_pDisturbance` (Xbox PDB): `CIdvBezierSpline*`.
        0x50 m_pDisturbance: Ptr,
        /// `m_pGravity` (Xbox PDB): `CIdvBezierSpline*`.
        0x54 m_pGravity: Ptr,
        /// `m_pFlexibility` (Xbox PDB): `CIdvBezierSpline*`.
        0x58 m_pFlexibility: Ptr,
        /// `m_pFlexibilityScale` (Xbox PDB): `CIdvBezierSpline*`.
        0x5C m_pFlexibilityScale: Ptr,
        /// `m_pLength` (Xbox PDB): `CIdvBezierSpline*`.
        0x60 m_pLength: Ptr,
        /// `m_pRadius` (Xbox PDB): `CIdvBezierSpline*`.
        0x64 m_pRadius: Ptr,
        /// `m_pRadiusScale` (Xbox PDB): `CIdvBezierSpline*`.
        0x68 m_pRadiusScale: Ptr,
        /// `m_pStartAngle` (Xbox PDB): `CIdvBezierSpline*`.
        0x6C m_pStartAngle: Ptr,
        /// `m_pAngleProfile` (Xbox PDB): `CIdvBezierSpline*`.
        0x70 m_pAngleProfile: Ptr,
    }
}

/// The tree file as `Parse` reads it: the read position (`m_nIndex`, Xbox
/// PDB) at +0; the byte vector starts at +4.
const FILE_INDEX: u32 = 0;

// Translated from 00b3f310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3f310 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// The `SIdvBranchInfo` constructor (identified from the Xbox PDB layout it
/// fills; the engine map has no name): 6 cross-section segments, 3
/// segments, the defaults of every parameter, the three flags and the nine
/// curve pointers cleared; then six curves are allocated (0x84 bytes each,
/// `CIdvBezierSpline` default constructor) in the order flexibility,
/// gravity, radius, start angle, radius scale, length. The disturbance,
/// flexibility scale and angle profile curves stay null. Returns `this`.
pub fn fn_00b3f310(e: &mut Engine, this: Ptr<SIdvBranchInfo>) -> Ptr<SIdvBranchInfo> {
    e.set(this, SIdvBranchInfo::m_nCrossSectionSegments, 6);
    e.set(this, SIdvBranchInfo::m_nSegments, 3);
    let thirty_percent = x87_quiet(e.global::<f32>(DEFAULT_THIRTY_PERCENT));
    e.set(this, SIdvBranchInfo::m_fFirstBranch, thirty_percent);
    e.set(this, SIdvBranchInfo::m_fLastBranch, 1.0);
    e.set(this, SIdvBranchInfo::m_fFrequency, thirty_percent);
    e.set(this, SIdvBranchInfo::m_fSTile, 1.0);
    e.set(this, SIdvBranchInfo::m_fTTile, 1.0);
    e.set(this, SIdvBranchInfo::m_bSTileAbsolute, true);
    e.set(this, SIdvBranchInfo::m_bTTileAbsolute, false);
    e.set(this, SIdvBranchInfo::m_bRandomTCoordOffset, false);
    e.set(this, SIdvBranchInfo::m_fTwist, 0.0);
    e.set(this, SIdvBranchInfo::m_fSegmentPackingExponent, 1.0);
    e.set(this, SIdvBranchInfo::m_nNumFlares, 0);
    e.set(this, SIdvBranchInfo::m_fFlareBalance, 1.0);
    let radial_influence = x87_quiet(e.global::<f32>(DEFAULT_RADIAL_INFLUENCE));
    e.set(this, SIdvBranchInfo::m_fRadialInfluence, radial_influence);
    let radial_variance = x87_quiet(e.global::<f32>(DEFAULT_RADIAL_INFLUENCE_VARIANCE));
    e.set(
        this,
        SIdvBranchInfo::m_fRadialInfluenceVariance,
        radial_variance,
    );
    e.set(this, SIdvBranchInfo::m_fRadialExponent, 1.0);
    let half = x87_quiet(e.global::<f32>(DEFAULT_HALF));
    e.set(this, SIdvBranchInfo::m_fRadialDistance, half);
    let quarter = x87_quiet(e.global::<f32>(DEFAULT_QUARTER));
    e.set(this, SIdvBranchInfo::m_fRadialVariance, quarter);
    e.set(this, SIdvBranchInfo::m_fLengthDistance, thirty_percent);
    let tenth = x87_quiet(e.global::<f32>(DEFAULT_TENTH));
    e.set(this, SIdvBranchInfo::m_fLengthVariance, tenth);
    e.set(this, SIdvBranchInfo::m_fLengthExponent, 1.0);
    e.set(this, SIdvBranchInfo::m_pDisturbance, Ptr::NULL);
    e.set(this, SIdvBranchInfo::m_pGravity, Ptr::NULL);
    e.set(this, SIdvBranchInfo::m_pFlexibility, Ptr::NULL);
    e.set(this, SIdvBranchInfo::m_pFlexibilityScale, Ptr::NULL);
    e.set(this, SIdvBranchInfo::m_pLength, Ptr::NULL);
    e.set(this, SIdvBranchInfo::m_pRadius, Ptr::NULL);
    e.set(this, SIdvBranchInfo::m_pRadiusScale, Ptr::NULL);
    e.set(this, SIdvBranchInfo::m_pStartAngle, Ptr::NULL);
    e.set(this, SIdvBranchInfo::m_pAngleProfile, Ptr::NULL);

    for field in [
        SIdvBranchInfo::m_pFlexibility,
        SIdvBranchInfo::m_pGravity,
        SIdvBranchInfo::m_pRadius,
        SIdvBranchInfo::m_pStartAngle,
        SIdvBranchInfo::m_pRadiusScale,
        SIdvBranchInfo::m_pLength,
    ] {
        let memory = e.call(OPERATOR_NEW, &args![0x84u32]).ptr::<()>();
        let spline = if memory.is_null() {
            Ptr::NULL
        } else {
            e.call(SPLINE_CONSTRUCTOR, &args![memory]).ptr::<()>()
        };
        e.set(this, field, spline);
    }
    this
}

/// Deletes a curve the way every owner of one does: the destructor body,
/// then `operator delete` (the compiler folded the "delete too" flag to 1,
/// so the other branch of its `flags & 1` test never runs).
fn delete_spline(e: &mut Engine, spline: Ptr) {
    e.call(SPLINE_DESTRUCTOR, &args![spline]);
    e.call(OPERATOR_DELETE, &args![spline]);
}

// Translated from 00b3f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::~SIdvBranchInfo` (Xbox PDB): deletes the nine curves
/// that are set (flexibility, flexibility scale, gravity, disturbance, start
/// angle, radius, radius scale, length, angle profile), then clears all nine
/// pointers.
pub fn sidv_branch_info_destructor(e: &mut Engine, this: Ptr<SIdvBranchInfo>) {
    for field in [
        SIdvBranchInfo::m_pFlexibility,
        SIdvBranchInfo::m_pFlexibilityScale,
        SIdvBranchInfo::m_pGravity,
        SIdvBranchInfo::m_pDisturbance,
        SIdvBranchInfo::m_pStartAngle,
        SIdvBranchInfo::m_pRadius,
        SIdvBranchInfo::m_pRadiusScale,
        SIdvBranchInfo::m_pLength,
        SIdvBranchInfo::m_pAngleProfile,
    ] {
        let spline = e.get(this, field);
        if !spline.is_null() {
            delete_spline(e, spline);
        }
    }
    for field in [
        SIdvBranchInfo::m_pFlexibility,
        SIdvBranchInfo::m_pFlexibilityScale,
        SIdvBranchInfo::m_pGravity,
        SIdvBranchInfo::m_pDisturbance,
        SIdvBranchInfo::m_pStartAngle,
        SIdvBranchInfo::m_pRadius,
        SIdvBranchInfo::m_pRadiusScale,
        SIdvBranchInfo::m_pLength,
        SIdvBranchInfo::m_pAngleProfile,
    ] {
        e.set(this, field, Ptr::NULL);
    }
}

/// The body of the nine curve setters: when `spline` is not the curve
/// already held, deletes the old one (if any) and keeps `spline`.
fn set_spline(
    e: &mut Engine,
    this: Ptr<SIdvBranchInfo>,
    field: Field<SIdvBranchInfo, Ptr>,
    spline: Ptr,
) {
    if e.get(this, field) != spline {
        let old = e.get(this, field);
        if !old.is_null() {
            delete_spline(e, old);
        }
        e.set(this, field, spline);
    }
}

// Translated from 00b3fc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::SetDisturbance` (Xbox PDB): replaces the disturbance
/// curve (+0x50), deleting the old one.
pub fn sidv_branch_info_set_disturbance(e: &mut Engine, this: Ptr<SIdvBranchInfo>, spline: Ptr) {
    set_spline(e, this, SIdvBranchInfo::m_pDisturbance, spline);
}

// Translated from 00b3fcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::SetGravity` (Xbox PDB): replaces the gravity curve
/// (+0x54), deleting the old one.
pub fn sidv_branch_info_set_gravity(e: &mut Engine, this: Ptr<SIdvBranchInfo>, spline: Ptr) {
    set_spline(e, this, SIdvBranchInfo::m_pGravity, spline);
}

// Translated from 00b3fd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::SetFlexibility` (Xbox PDB): replaces the flexibility
/// curve (+0x58), deleting the old one.
pub fn sidv_branch_info_set_flexibility(e: &mut Engine, this: Ptr<SIdvBranchInfo>, spline: Ptr) {
    set_spline(e, this, SIdvBranchInfo::m_pFlexibility, spline);
}

// Translated from 00b3fdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::SetFlexibilityScale` (Xbox PDB): replaces the
/// flexibility scale curve (+0x5c), deleting the old one.
pub fn sidv_branch_info_set_flexibility_scale(
    e: &mut Engine,
    this: Ptr<SIdvBranchInfo>,
    spline: Ptr,
) {
    set_spline(e, this, SIdvBranchInfo::m_pFlexibilityScale, spline);
}

// Translated from 00b3fe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::SetLength` (Xbox PDB): replaces the length curve
/// (+0x60), deleting the old one.
pub fn sidv_branch_info_set_length(e: &mut Engine, this: Ptr<SIdvBranchInfo>, spline: Ptr) {
    set_spline(e, this, SIdvBranchInfo::m_pLength, spline);
}

// Translated from 00b3feb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::SetRadius` (Xbox PDB): replaces the radius curve
/// (+0x64), deleting the old one.
pub fn sidv_branch_info_set_radius(e: &mut Engine, this: Ptr<SIdvBranchInfo>, spline: Ptr) {
    set_spline(e, this, SIdvBranchInfo::m_pRadius, spline);
}

// Translated from 00b3ff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::SetRadiusScale` (Xbox PDB): replaces the radius scale
/// curve (+0x68), deleting the old one.
pub fn sidv_branch_info_set_radius_scale(e: &mut Engine, this: Ptr<SIdvBranchInfo>, spline: Ptr) {
    set_spline(e, this, SIdvBranchInfo::m_pRadiusScale, spline);
}

// Translated from 00b3ff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::SetStartAngle` (Xbox PDB): replaces the start angle
/// curve (+0x6c), deleting the old one.
pub fn sidv_branch_info_set_start_angle(e: &mut Engine, this: Ptr<SIdvBranchInfo>, spline: Ptr) {
    set_spline(e, this, SIdvBranchInfo::m_pStartAngle, spline);
}

// Translated from 00b40000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchInfo::SetAngleProfile` (Xbox PDB): replaces the angle profile
/// curve (+0x70), deleting the old one.
pub fn sidv_branch_info_set_angle_profile(e: &mut Engine, this: Ptr<SIdvBranchInfo>, spline: Ptr) {
    set_spline(e, this, SIdvBranchInfo::m_pAngleProfile, spline);
}

/// Throws the parse exception for `message` (the string, an exception built
/// from it and a `false` flag, `_CxxThrowException`). Does not return.
fn throw_malformed(e: &mut Engine, message: u32) -> ! {
    e.with_stack(0x1c, |e, text| {
        e.call(STRING_FROM_C_STRING, &args![text, message]);
        e.with_stack(0x28, |e, exception| {
            e.call(PARSE_ERROR_FROM_STRING, &args![exception, text, 0u32]);
            e.call(CXX_THROW_EXCEPTION, &args![exception, PARSE_THROW_INFO]);
        });
    });
    unreachable!("__CxxThrowException does not return (00ec7894)")
}

/// A `bool` field read from the tree file's byte vector: the position is
/// taken and advanced, the byte fetched with the checked element access.
fn parse_flag(e: &mut Engine, file: Ptr) -> bool {
    let index = e.mem.u32(file.addr() + FILE_INDEX);
    e.mem
        .set_u32(file.addr() + FILE_INDEX, index.wrapping_add(1));
    let element = e
        .call(BYTE_VECTOR_ELEMENT, &args![file.byte_add(4), index])
        .u32();
    e.mem.u8(element) != 0
}

// Translated from 00b3f8e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3f8e0 - exception unwinding frame (FS:[0], /GS cookie) is not translated; the throws are.
/// `SIdvBranchInfo::Parse` (Xbox PDB, `cdecl`): reads one branch level from
/// the tree file into a new `SIdvBranchInfo`. The data must start with the
/// tag 0x3f8 (else `"malformed branch data"` is thrown) and run to the tag
/// 0x3f9. Tags 0x1770..=0x1777 and 0x1781 read a curve
/// (`ParseBranchParameter`) for the disturbance, gravity, flexibility,
/// flexibility scale, length, radius, radius scale, start angle and angle
/// profile; 0x1778 and 0x1779 read the two segment counts; 0x177a..=0x177e
/// the five floats (first branch, last branch, frequency, S tile, T tile);
/// 0x177f and 0x1780 the two tile flags (one byte each from the file's byte
/// vector). Any other tag, an end tag as the first one included, throws
/// `"malformed general branch information"`. Returns the new object.
pub fn sidv_branch_info_parse(e: &mut Engine, file: Ptr) -> Ptr<SIdvBranchInfo> {
    let memory = e
        .call(OPERATOR_NEW, &args![0x74u32])
        .ptr::<SIdvBranchInfo>();
    let info = if memory.is_null() {
        Ptr::NULL
    } else {
        fn_00b3f310(e, memory)
    };
    let tag = e.call(PARSE_INT, &args![file]).u32();
    if tag != 0x3f8 {
        throw_malformed(e, MESSAGE_MALFORMED_BRANCH);
    }
    let mut tag = e.call(PARSE_INT, &args![file]).u32();
    loop {
        // The compiler's switch: `tag - 0x1770` indexes a table of 18
        // entries (`00b3fc34`).
        match tag.wrapping_sub(0x1770) {
            index @ (0..=7 | 17) => {
                let spline = e.call(PARSE_BRANCH_PARAMETER, &args![file]).ptr::<()>();
                match index {
                    0 => sidv_branch_info_set_disturbance(e, info, spline),
                    1 => sidv_branch_info_set_gravity(e, info, spline),
                    2 => sidv_branch_info_set_flexibility(e, info, spline),
                    3 => sidv_branch_info_set_flexibility_scale(e, info, spline),
                    4 => sidv_branch_info_set_length(e, info, spline),
                    5 => sidv_branch_info_set_radius(e, info, spline),
                    6 => sidv_branch_info_set_radius_scale(e, info, spline),
                    7 => sidv_branch_info_set_start_angle(e, info, spline),
                    _ => sidv_branch_info_set_angle_profile(e, info, spline),
                }
            }
            8 => {
                let value = e.call(PARSE_INT, &args![file]).i32();
                e.set(info, SIdvBranchInfo::m_nCrossSectionSegments, value);
            }
            9 => {
                let value = e.call(PARSE_INT, &args![file]).i32();
                e.set(info, SIdvBranchInfo::m_nSegments, value);
            }
            index @ 10..=14 => {
                let value = e.call(PARSE_FLOAT, &args![file]).f32();
                let field = match index {
                    10 => SIdvBranchInfo::m_fFirstBranch,
                    11 => SIdvBranchInfo::m_fLastBranch,
                    12 => SIdvBranchInfo::m_fFrequency,
                    13 => SIdvBranchInfo::m_fSTile,
                    _ => SIdvBranchInfo::m_fTTile,
                };
                e.set(info, field, value);
            }
            15 => {
                let flag = parse_flag(e, file);
                e.set(info, SIdvBranchInfo::m_bSTileAbsolute, flag);
            }
            16 => {
                let flag = parse_flag(e, file);
                e.set(info, SIdvBranchInfo::m_bTTileAbsolute, flag);
            }
            _ => throw_malformed(e, MESSAGE_MALFORMED_GENERAL),
        }
        tag = e.call(PARSE_INT, &args![file]).u32();
        if tag == 0x3f9 {
            return info;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b3f310,
            fn_00b3f310(Ptr<SIdvBranchInfo>) -> Ptr<SIdvBranchInfo>
        ),
        entry!(0x00b3f620, sidv_branch_info_destructor(Ptr<SIdvBranchInfo>)),
        entry!(0x00b3f8e0, sidv_branch_info_parse(Ptr) -> Ptr<SIdvBranchInfo>),
        entry!(
            0x00b3fc80,
            sidv_branch_info_set_disturbance(Ptr<SIdvBranchInfo>, Ptr)
        ),
        entry!(
            0x00b3fcf0,
            sidv_branch_info_set_gravity(Ptr<SIdvBranchInfo>, Ptr)
        ),
        entry!(
            0x00b3fd60,
            sidv_branch_info_set_flexibility(Ptr<SIdvBranchInfo>, Ptr)
        ),
        entry!(
            0x00b3fdd0,
            sidv_branch_info_set_flexibility_scale(Ptr<SIdvBranchInfo>, Ptr)
        ),
        entry!(
            0x00b3fe40,
            sidv_branch_info_set_length(Ptr<SIdvBranchInfo>, Ptr)
        ),
        entry!(
            0x00b3feb0,
            sidv_branch_info_set_radius(Ptr<SIdvBranchInfo>, Ptr)
        ),
        entry!(
            0x00b3ff20,
            sidv_branch_info_set_radius_scale(Ptr<SIdvBranchInfo>, Ptr)
        ),
        entry!(
            0x00b3ff90,
            sidv_branch_info_set_start_angle(Ptr<SIdvBranchInfo>, Ptr)
        ),
        entry!(
            0x00b40000,
            sidv_branch_info_set_angle_profile(Ptr<SIdvBranchInfo>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Records the calls to `addresses`; each double answers `Ret::default()`.
    fn recorded(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for &address in addresses {
            let record = calls.clone();
            e.register_double(address, move |_, a| {
                record.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    fn branch_info(e: &mut Engine) -> Ptr<SIdvBranchInfo> {
        e.new_object()
    }

    #[test]
    fn constructor_sets_the_defaults_and_builds_six_curves() {
        let mut e = Engine::new();
        for page in [0x0101_6000, 0x0101_7000, 0x0101_8000, 0x0101_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEFAULT_THIRTY_PERCENT, 0.3f32);
        e.set_global(DEFAULT_RADIAL_INFLUENCE, 30.0f32);
        e.set_global(DEFAULT_RADIAL_INFLUENCE_VARIANCE, 10.0f32);
        e.set_global(DEFAULT_HALF, 0.5f32);
        e.set_global(DEFAULT_QUARTER, 0.25f32);
        e.set_global(DEFAULT_TENTH, 0.1f32);
        let sizes: Calls = Rc::new(RefCell::new(Vec::new()));
        let record = sizes.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            record.borrow_mut().push((OPERATOR_NEW, a.to_vec()));
            Ret {
                eax: e.mem.alloc(a[0]),
                ..Ret::default()
            }
        });
        let record = sizes.clone();
        e.register_double(SPLINE_CONSTRUCTOR, move |_, a| {
            record.borrow_mut().push((SPLINE_CONSTRUCTOR, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let this = branch_info(&mut e);
        for offset in (0..0x74).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xdead_beef);
        }
        assert_eq!(
            e.call(0x00b3_f310, &args![this]).ptr::<SIdvBranchInfo>(),
            this
        );
        assert_eq!(e.get(this, SIdvBranchInfo::m_nCrossSectionSegments), 6);
        assert_eq!(e.get(this, SIdvBranchInfo::m_nSegments), 3);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fFirstBranch), 0.3);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fLastBranch), 1.0);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fFrequency), 0.3);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fSTile), 1.0);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fTTile), 1.0);
        assert!(e.get(this, SIdvBranchInfo::m_bSTileAbsolute));
        assert!(!e.get(this, SIdvBranchInfo::m_bTTileAbsolute));
        assert!(!e.get(this, SIdvBranchInfo::m_bRandomTCoordOffset));
        assert_eq!(e.get(this, SIdvBranchInfo::m_fTwist), 0.0);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fSegmentPackingExponent), 1.0);
        assert_eq!(e.get(this, SIdvBranchInfo::m_nNumFlares), 0);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fFlareBalance), 1.0);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fRadialInfluence), 30.0);
        assert_eq!(
            e.get(this, SIdvBranchInfo::m_fRadialInfluenceVariance),
            10.0
        );
        assert_eq!(e.get(this, SIdvBranchInfo::m_fRadialExponent), 1.0);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fRadialDistance), 0.5);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fRadialVariance), 0.25);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fLengthDistance), 0.3);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fLengthVariance), 0.1);
        assert_eq!(e.get(this, SIdvBranchInfo::m_fLengthExponent), 1.0);
        // Six 0x84-byte curves, in the exe's order, three pointers left null.
        let log = sizes.borrow();
        assert_eq!(
            log.iter()
                .filter(|c| c.0 == OPERATOR_NEW && c.1 == [0x84])
                .count(),
            6
        );
        let built: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == SPLINE_CONSTRUCTOR)
            .map(|c| c.1[0])
            .collect();
        let expected = [
            SIdvBranchInfo::m_pFlexibility,
            SIdvBranchInfo::m_pGravity,
            SIdvBranchInfo::m_pRadius,
            SIdvBranchInfo::m_pStartAngle,
            SIdvBranchInfo::m_pRadiusScale,
            SIdvBranchInfo::m_pLength,
        ]
        .map(|field| e.get(this, field).addr());
        assert_eq!(built, expected);
        assert!(expected.iter().all(|&p| p != 0));
        assert!(e.get(this, SIdvBranchInfo::m_pDisturbance).is_null());
        assert!(e.get(this, SIdvBranchInfo::m_pFlexibilityScale).is_null());
        assert!(e.get(this, SIdvBranchInfo::m_pAngleProfile).is_null());
    }

    #[test]
    fn constructor_keeps_null_curves_when_allocation_fails() {
        let mut e = Engine::new();
        for page in [0x0101_6000, 0x0101_7000, 0x0101_8000, 0x0101_e000] {
            e.map(page, 0x1000);
        }
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let calls = recorded(&mut e, &[SPLINE_CONSTRUCTOR]);
        let this = branch_info(&mut e);
        e.mem.set_u32(this.addr() + 0x58, 0x1234);
        e.call(0x00b3_f310, &args![this]);
        assert!(calls.borrow().is_empty());
        assert!(e.get(this, SIdvBranchInfo::m_pFlexibility).is_null());
        assert!(e.get(this, SIdvBranchInfo::m_pLength).is_null());
    }

    const SETTERS: [(u32, u32); 9] = [
        (0x00b3_fc80, 0x50),
        (0x00b3_fcf0, 0x54),
        (0x00b3_fd60, 0x58),
        (0x00b3_fdd0, 0x5c),
        (0x00b3_fe40, 0x60),
        (0x00b3_feb0, 0x64),
        (0x00b3_ff20, 0x68),
        (0x00b3_ff90, 0x6c),
        (0x00b4_0000, 0x70),
    ];

    #[test]
    fn the_setters_replace_their_curve_and_delete_the_old_one() {
        for (address, offset) in SETTERS {
            let mut e = Engine::new();
            let calls = recorded(&mut e, &[SPLINE_DESTRUCTOR, OPERATOR_DELETE]);
            let this = branch_info(&mut e);
            // No old curve: just stored.
            e.call(address, &args![this, 0x1000u32]);
            assert_eq!(e.mem.u32(this.addr() + offset), 0x1000);
            assert!(calls.borrow().is_empty());
            // The same curve again: nothing happens.
            e.call(address, &args![this, 0x1000u32]);
            assert!(calls.borrow().is_empty());
            // A different one: the old curve is destroyed and deleted.
            e.call(address, &args![this, 0x2000u32]);
            assert_eq!(e.mem.u32(this.addr() + offset), 0x2000);
            assert_eq!(
                *calls.borrow(),
                vec![
                    (SPLINE_DESTRUCTOR, vec![0x1000]),
                    (OPERATOR_DELETE, vec![0x1000])
                ]
            );
            // Setting null deletes the current one and clears the field.
            calls.borrow_mut().clear();
            e.call(address, &args![this, 0u32]);
            assert_eq!(e.mem.u32(this.addr() + offset), 0);
            assert_eq!(calls.borrow().len(), 2);
        }
    }

    #[test]
    fn the_destructor_deletes_every_curve_in_the_exes_order_and_clears_them() {
        let mut e = Engine::new();
        let calls = recorded(&mut e, &[SPLINE_DESTRUCTOR, OPERATOR_DELETE]);
        let this = branch_info(&mut e);
        // Curves at every offset except the gravity one (0x54).
        for offset in [0x50, 0x58, 0x5c, 0x60, 0x64, 0x68, 0x6c, 0x70] {
            e.mem.set_u32(this.addr() + offset, 0x1000 + offset);
        }
        e.call(0x00b3_f620, &args![this]);
        let order: Vec<u32> = calls
            .borrow()
            .iter()
            .filter(|c| c.0 == SPLINE_DESTRUCTOR)
            .map(|c| c.1[0])
            .collect();
        // 0x58, 0x5c, (0x54 is null), 0x50, 0x6c, 0x64, 0x68, 0x60, 0x70.
        assert_eq!(
            order,
            vec![0x1058, 0x105c, 0x1050, 0x106c, 0x1064, 0x1068, 0x1060, 0x1070]
        );
        assert_eq!(calls.borrow().len(), 16);
        for offset in (0x50..0x74).step_by(4) {
            assert_eq!(e.mem.u32(this.addr() + offset), 0);
        }
    }

    /// An engine for `Parse`: doubles for the file reads, the allocation and
    /// the curve destructor, and the defaults the constructor loads.
    /// `ints` are the values `ParseInt` answers in order, `floats` those of
    /// `ParseFloat`, `bytes` the tree file's byte vector.
    fn parse_engine(ints: Vec<u32>, floats: Vec<f32>, bytes: Vec<u8>) -> (Engine, Ptr, Calls) {
        let mut e = Engine::new();
        for page in [0x0101_6000, 0x0101_7000, 0x0101_8000, 0x0101_e000] {
            e.map(page, 0x1000);
        }
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(SPLINE_CONSTRUCTOR, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let calls = recorded(
            &mut e,
            &[
                SPLINE_DESTRUCTOR,
                OPERATOR_DELETE,
                STRING_FROM_C_STRING,
                PARSE_ERROR_FROM_STRING,
            ],
        );
        let mut next = 0;
        e.register_double(PARSE_INT, move |_, _| {
            next += 1;
            Ret {
                eax: ints[next - 1],
                ..Ret::default()
            }
        });
        let mut next_float = 0;
        e.register_double(PARSE_FLOAT, move |_, _| {
            next_float += 1;
            Ret {
                st0: floats[next_float - 1] as f64,
                ..Ret::default()
            }
        });
        let mut curves = 0;
        e.register_double(PARSE_BRANCH_PARAMETER, move |_, _| {
            curves += 1;
            Ret {
                eax: 0x5000 + curves,
                ..Ret::default()
            }
        });
        let memory = e.mem.alloc(bytes.len().max(1) as u32);
        e.mem.write(memory, &bytes);
        e.register_double(BYTE_VECTOR_ELEMENT, move |_, a| Ret {
            eax: memory + a[1],
            ..Ret::default()
        });
        let file = Ptr::<()>::new(e.mem.alloc(0x20));
        (e, file, calls)
    }

    #[test]
    fn parse_reads_every_kind_of_field() {
        let tags = vec![
            0x3f8, 0x1770, 0x1771, 0x1772, 0x1773, 0x1774, 0x1775, 0x1776, 0x1777, 0x1781, 0x1778,
            11, 0x1779, 22, 0x177a, 0x177b, 0x177c, 0x177d, 0x177e, 0x177f, 0x1780, 0x3f9,
        ];
        let floats = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let (mut e, file, calls) = parse_engine(tags, floats, vec![1, 0]);
        let info = e.call(0x00b3_f8e0, &args![file]).ptr::<SIdvBranchInfo>();
        // Disturbance .. start angle take curves 1..=8, the angle profile 9.
        assert_eq!(e.get(info, SIdvBranchInfo::m_pDisturbance).addr(), 0x5001);
        assert_eq!(e.get(info, SIdvBranchInfo::m_pGravity).addr(), 0x5002);
        assert_eq!(e.get(info, SIdvBranchInfo::m_pFlexibility).addr(), 0x5003);
        assert_eq!(
            e.get(info, SIdvBranchInfo::m_pFlexibilityScale).addr(),
            0x5004
        );
        assert_eq!(e.get(info, SIdvBranchInfo::m_pLength).addr(), 0x5005);
        assert_eq!(e.get(info, SIdvBranchInfo::m_pRadius).addr(), 0x5006);
        assert_eq!(e.get(info, SIdvBranchInfo::m_pRadiusScale).addr(), 0x5007);
        assert_eq!(e.get(info, SIdvBranchInfo::m_pStartAngle).addr(), 0x5008);
        assert_eq!(e.get(info, SIdvBranchInfo::m_pAngleProfile).addr(), 0x5009);
        assert_eq!(e.get(info, SIdvBranchInfo::m_nCrossSectionSegments), 11);
        assert_eq!(e.get(info, SIdvBranchInfo::m_nSegments), 22);
        assert_eq!(e.get(info, SIdvBranchInfo::m_fFirstBranch), 0.1);
        assert_eq!(e.get(info, SIdvBranchInfo::m_fLastBranch), 0.2);
        assert_eq!(e.get(info, SIdvBranchInfo::m_fFrequency), 0.3);
        assert_eq!(e.get(info, SIdvBranchInfo::m_fSTile), 0.4);
        assert_eq!(e.get(info, SIdvBranchInfo::m_fTTile), 0.5);
        // The tile flags come from bytes 1 and 0 (the constructor's S flag
        // was true, the T flag false).
        assert!(e.get(info, SIdvBranchInfo::m_bSTileAbsolute));
        assert!(!e.get(info, SIdvBranchInfo::m_bTTileAbsolute));
        assert_eq!(e.mem.u32(file.addr()), 2);
        // The six constructor curves the setters replaced were deleted: the
        // disturbance, flexibility scale and angle profile were null.
        let deleted = calls
            .borrow()
            .iter()
            .filter(|c| c.0 == SPLINE_DESTRUCTOR)
            .count();
        assert_eq!(deleted, 6);
    }

    #[test]
    fn parse_flags_follow_the_bytes() {
        let tags = vec![0x3f8, 0x177f, 0x1780, 0x3f9];
        let (mut e, file, _) = parse_engine(tags, vec![], vec![0, 7]);
        let info = e.call(0x00b3_f8e0, &args![file]).ptr::<SIdvBranchInfo>();
        assert!(!e.get(info, SIdvBranchInfo::m_bSTileAbsolute));
        assert!(e.get(info, SIdvBranchInfo::m_bTTileAbsolute));
    }

    #[test]
    fn parse_throws_without_the_start_tag() {
        let (mut e, _, calls) = parse_engine(vec![0x1234], vec![], vec![]);
        e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
        let file = Ptr::<()>::new(e.mem.alloc(0x20));
        let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00b3_f8e0, &args![file]);
        }));
        assert!(thrown.is_err());
        let log = calls.borrow();
        let string = log.iter().find(|c| c.0 == STRING_FROM_C_STRING).unwrap();
        assert_eq!(string.1[1], MESSAGE_MALFORMED_BRANCH);
        let error = log.iter().find(|c| c.0 == PARSE_ERROR_FROM_STRING).unwrap();
        assert_eq!(error.1[1..], [string.1[0], 0]);
    }

    #[test]
    fn parse_throws_on_an_unknown_tag() {
        for tag in [0x176f, 0x1782, 0x3f9, 0] {
            let (mut e, _, calls) = parse_engine(vec![0x3f8, tag], vec![], vec![]);
            e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
            let file = Ptr::<()>::new(e.mem.alloc(0x20));
            let thrown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                e.call(0x00b3_f8e0, &args![file]);
            }));
            assert!(thrown.is_err());
            let log = calls.borrow();
            let string = log.iter().find(|c| c.0 == STRING_FROM_C_STRING).unwrap();
            assert_eq!(string.1[1], MESSAGE_MALFORMED_GENERAL);
        }
    }
}
