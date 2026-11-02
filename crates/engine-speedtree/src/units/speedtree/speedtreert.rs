//! `SpeedTree/speedtreert.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SpeedTreeRT's public class `CSpeedTreeRT` (the tree object the game holds): the instance
//! maker, `Compute`, the file loaders and their chunk parsers, the lighting, wind and level of
//! detail set-up, the camera and the geometry getters.
//!
//! x87 note: the exe computes in extended precision and stores `float` results. The
//! translations compute in `f64` through the shared x87 helpers (`x87_add`, `x87_mul`, ...),
//! in the exe's order of operations, and round to `f32` at each `float` store (`FSTP dword`).
//! A float copied through the x87 stack (`FLD dword`, `FSTP dword`) goes through
//! [`x87_quiet_bits`].
//!
//! Functions that carry an `FS:[0]` exception frame are marked `Translation incomplete`
//! (the unwinding handler is not translated).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{
    checked_iterator, copy_float, copy_words, ftol, vector_at, vector_len, CVec3,
    INVALID_PARAMETER, MEMCPY,
};

/// `MemoryManager::Allocate` (`00aa3e40`, `this` = the manager, a size) and `Deallocate`
/// (`00aa4060`, `this` = the manager, the block); the manager singleton is at `011f6238`.
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `_CxxThrowException(object, throw_info)`.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// The empty destructor of the `CVec3`-like members (`00aa7830`, `this` only).
const TRIVIAL_DESTRUCTOR: u32 = 0x00aa_7830;
/// `std::string::_Tidy(built, new_size)` (`0044a730`, `this` = the string, a flag and a size).
const STRING_TIDY: u32 = 0x0044_a730;

/// `[011f8b7c]`: the count of every `CSpeedTreeRT` ever made or copied (Xbox PDB
/// `m_uiAllRefCount`; the address is matched by its uses).
const ALL_REF_COUNT: u32 = 0x011f_8b7c;

/// The static camera the leaf and billboard code reads (position, then direction: three floats
/// each; the addresses are matched by their uses, the Xbox PDB keeps such a camera in statics of
/// `CIdvCamera`, unconfirmed for these).
const CAMERA_POSITION: u32 = 0x011f_8e9c;
const CAMERA_DIRECTION: u32 = 0x011f_8eac;
/// `011f8b78`: the byte that selects discrete leaf levels (one more level than the leaf info
/// has); the other statics: the critical section guarding the list of unique trees
/// (`011f8bc4`), that list (a `std::vector` of tree pointers, `011f8bdc`) and the static error
/// string `SetError` fills (`011f8ba4`).
const DISCRETE_LEAF_LOD_FLAG: u32 = 0x011f_8b78;
const UNIQUE_TREES_LOCK: u32 = 0x011f_8bc4;
const UNIQUE_TREES: u32 = 0x011f_8bdc;
const ERROR_STRING: u32 = 0x011f_8ba4;
/// The `EnterCriticalSection` and `LeaveCriticalSection` import slots.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `CTreeEngine` +0x21: `m_bTransientDataIntact` (Xbox PDB; the offset agrees on the PC).
const ENGINE_TRANSIENT_DATA_INTACT: u32 = 0x21;
/// The CRT helpers `00ec6cde` (`cdecl`, one `double`; Ghidra names it `_fabs`) and `pow`
/// (`00ec9ac0`, `cdecl`, base then exponent).
const ABSOLUTE_VALUE: u32 = 0x00ec_6cde;
const POW: u32 = 0x00ec_9ac0;

layout! {
    /// `CSpeedTreeRT` (Xbox PDB), 0xA0 bytes on the PC as on the Xbox (no `std::vector` member).
    pub struct CSpeedTreeRT: 0xA0 {
        /// `m_pEngine` (Xbox PDB): the `CTreeEngine`.
        0x00 m_pEngine: Ptr,
        /// `m_pBranchGeometry` (Xbox PDB): a `CIndexedGeometry`.
        0x04 m_pBranchGeometry: Ptr,
        /// `m_pLeafGeometry` (Xbox PDB): a `CLeafGeometry`.
        0x08 m_pLeafGeometry: Ptr,
        /// `m_pLightingEngine` (Xbox PDB): a `CLightingEngine`.
        0x0C m_pLightingEngine: Ptr,
        /// `m_pWindEngine` (Xbox PDB): a `CWindEngine`.
        0x10 m_pWindEngine: Ptr,
        /// `m_pSimpleBillboard` (Xbox PDB).
        0x14 m_pSimpleBillboard: Ptr,
        /// `m_eLeafLodMethod` (Xbox PDB).
        0x18 m_eLeafLodMethod: u32,
        /// `m_fLeafLodTransitionRadius` (Xbox PDB).
        0x1C m_fLeafLodTransitionRadius: f32,
        /// `m_fLeafLodCurveExponent` (Xbox PDB).
        0x20 m_fLeafLodCurveExponent: f32,
        /// `m_fLeafSizeIncreaseFactor` (Xbox PDB).
        0x24 m_fLeafSizeIncreaseFactor: f32,
        /// `m_fLeafTransitionFactor` (Xbox PDB).
        0x28 m_fLeafTransitionFactor: f32,
        /// `m_pLeafLodSizeFactors` (Xbox PDB).
        0x2C m_pLeafLodSizeFactors: Ptr,
        /// `m_pInstanceRefCount` (Xbox PDB): the count shared by every instance of a tree.
        0x30 m_pInstanceRefCount: Ptr,
        /// `m_pInstanceData` (Xbox PDB): this instance's `STreeInstanceData`.
        0x34 m_pInstanceData: Ptr<STreeInstanceData>,
        /// `m_pInstanceList` (Xbox PDB): the `SInstanceList` shared by the instances.
        0x38 m_pInstanceList: Ptr,
        /// `m_nFrondLevel` (Xbox PDB).
        0x3C m_nFrondLevel: i32,
        /// `m_pTreeSizes` (Xbox PDB): the bounding box, six floats, and the size at `+0x18`.
        0x40 m_pTreeSizes: Ptr,
        /// `m_ucTargetAlphaValue` (Xbox PDB).
        0x44 m_ucTargetAlphaValue: u8,
        /// `m_bTreeComputed` (Xbox PDB).
        0x45 m_bTreeComputed: u8,
        /// `m_nBranchWindLevel` (Xbox PDB).
        0x48 m_nBranchWindLevel: i32,
        /// `m_pEmbeddedTexCoords` (Xbox PDB): an `SEmbeddedTexCoords`.
        0x4C m_pEmbeddedTexCoords: Ptr,
        /// `m_pProjectedShadow` (Xbox PDB): a `CProjectedShadow`.
        0x50 m_pProjectedShadow: Ptr,
        /// `m_usNum360Billboards` (Xbox PDB).
        0x54 m_usNum360Billboards: u16,
        /// `m_pCollisionObjects` (Xbox PDB): an `SCollisionObjects`.
        0x58 m_pCollisionObjects: Ptr,
        /// `m_pFrondEngine` (Xbox PDB): a `CFrondEngine`.
        0x5C m_pFrondEngine: Ptr,
        /// `m_pFrondGeometry` (Xbox PDB): a `CIndexedGeometry`.
        0x60 m_pFrondGeometry: Ptr,
        /// `m_usNumFrondLodLevels` (Xbox PDB).
        0x64 m_usNumFrondLodLevels: u16,
        /// `m_pUserData` (Xbox PDB).
        0x68 m_pUserData: Ptr,
        /// `m_b360Billboard` (Xbox PDB).
        0x6C m_b360Billboard: u8,
        /// `m_bHorizontalBillboard` (Xbox PDB).
        0x6D m_bHorizontalBillboard: u8,
        /// `m_afHorizontalCoords` (Xbox PDB): twelve floats.
        0x70 m_afHorizontalCoords: f32,
    }

    /// `STreeInstanceData` (Xbox PDB), 0x14 bytes.
    pub struct STreeInstanceData: 0x14 {
        /// `m_pParent` (Xbox PDB): the tree this instance was made from.
        0x00 m_pParent: Ptr<CSpeedTreeRT>,
        /// `m_cPosition` (Xbox PDB): three floats.
        0x04 m_cPosition: f32,
        /// `m_fLodLevel` (Xbox PDB).
        0x10 m_fLodLevel: f32,
    }
}

/// `FLD dword`: the x87 value after loading the float at `addr`.
pub(crate) fn fld(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// A `float` memory operand of an x87 arithmetic instruction (`FMUL dword ptr [addr]`).
pub(crate) fn operand(e: &Engine, addr: u32) -> f64 {
    x87_m32(e.mem.u32(addr))
}

/// `FSTP dword`: the x87 value rounded to the float at `addr`.
pub(crate) fn fstp(e: &mut Engine, addr: u32, value: f64) {
    e.mem.set_u32(addr, x87_store32(value));
}

/// The bits an argument push of the float at `addr` (`FLD`, `FSTP [ESP]`) leaves.
pub(crate) fn pushed_float(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// The checked-iterator constructor `00b10430` (`this` = the iterator, a position, the vector).
const ITERATOR_CONSTRUCT: u32 = 0x00b1_0430;
/// The iterator comparison `00b0eca0` (`this` = the iterator, the other one); returns a byte.
const ITERATOR_EQUAL: u32 = 0x00b0_eca0;
/// The static `Compute` sets to the frond engine (`011f8c00`).
const CURRENT_FROND_ENGINE: u32 = 0x011f_8c00;
/// `011f8b79`: the byte `Compute` hands to the frond texture set-up.
const FROND_TEXTURE_FLAG: u32 = 0x011f_8b79;
/// `011f8c04`: the byte the chunk `22000` of a tree file sets.
const TREE_FILE_FLAG: u32 = 0x011f_8c04;
/// `011f8d80`: the time `SetTime` stores; `011f8d84`: the word the loaders hand to the wind
/// matrices' set-up.
const WIND_TIME: u32 = 0x011f_8d80;
const WIND_MATRIX_SPAN: u32 = 0x011f_8d84;

// Translated from 00b036d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b036d0 - the FS:[0] exception frame (releasing the instance data
// if a later step throws) is not translated.
/// The `CSpeedTreeRT` instance constructor (a copy of `source`, unnamed in the engine map): copies
/// every member except the instance-data pointer (`+0x34`), registers `this` in the instance
/// list (`00b0ddc0`), copies the horizontal-billboard coordinates (0x30 bytes), counts one more
/// instance in the shared count, allocates a fresh `STreeInstanceData` (0x14 bytes: the parent
/// `source`, the source engine's position, the source's level of detail) and counts one more
/// tree in the all-trees static.
pub fn fn_00b036d0(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    source: Ptr<CSpeedTreeRT>,
) -> Ptr<CSpeedTreeRT> {
    let (to, from) = (this.addr(), source.addr());
    for offset in [0x00u32, 0x04, 0x08, 0x0C, 0x10, 0x14, 0x18] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    // The four floats are copied through the x87 stack.
    for offset in [0x1Cu32, 0x20, 0x24, 0x28] {
        copy_float(e, to + offset, from + offset);
    }
    for offset in [0x2Cu32, 0x30, 0x38, 0x3C, 0x40] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0x44u32, 0x45] {
        let byte = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, byte);
    }
    for offset in [0x48u32, 0x4C, 0x50] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    let word = e.mem.u16(from + 0x54);
    e.mem.set_u16(to + 0x54, word);
    for offset in [0x58u32, 0x5C, 0x60] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    let word = e.mem.u16(from + 0x64);
    e.mem.set_u16(to + 0x64, word);
    let word = e.mem.u32(from + 0x68);
    e.mem.set_u32(to + 0x68, word);
    for offset in [0x6Cu32, 0x6D] {
        let byte = e.mem.u8(from + offset);
        e.mem.set_u8(to + offset, byte);
    }
    // The instance list gets the address of a local holding `this`.
    let list = e.get(this, CSpeedTreeRT::m_pInstanceList);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), to);
        e.call(0x00b0_ddc0, &args![list, slot]);
    });
    e.call(MEMCPY, &args![to + 0x70, from + 0x70, 0x30u32]);
    let count = e.get(this, CSpeedTreeRT::m_pInstanceRefCount);
    let total = e.mem.u32(count.addr()).wrapping_add(1);
    e.mem.set_u32(count.addr(), total);
    let block: Ptr<STreeInstanceData> = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x14u32]).ptr();
    if !block.is_null() {
        e.mem.set_u32(block.addr(), 0);
        e.mem.set_f32(block.addr() + 0xC, 0.0);
        e.mem.set_f32(block.addr() + 8, 0.0);
        e.mem.set_f32(block.addr() + 4, 0.0);
        e.mem.set_f32(block.addr() + 0x10, 1.0);
    }
    e.set(this, CSpeedTreeRT::m_pInstanceData, block);
    let data = e.get(this, CSpeedTreeRT::m_pInstanceData);
    e.set(data, STreeInstanceData::m_pParent, source);
    // The position is copied word by word from the source engine's `CIdvCamera` position.
    let engine = e.mem.u32(from);
    copy_words(e, data.addr() + 4, engine + 4, 3);
    let level = c_speed_tree_rt_get_lod_level(e, source);
    let data = e.get(this, CSpeedTreeRT::m_pInstanceData);
    e.mem
        .set_u32(data.addr() + 0x10, x87_quiet_bits(level.to_bits()));
    let all = e.global::<u32>(ALL_REF_COUNT).wrapping_add(1);
    e.set_global(ALL_REF_COUNT, all);
    this
}

// Translated from 00b044a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b044a0 - the FS:[0] exception frame (destroying the local extents
// if a step throws) is not translated.
/// `CSpeedTreeRT::Compute` (Xbox PDB): once only (a second call reports through `SetError`).
/// Sets the static frond engine, seeds and computes the tree engine (the leaf-lod float from
/// `+0x24`), the frond engine, copies the frond lod count, builds the leaf tables, the wind
/// matrices and the leaf geometry, hands the embedded texture coordinates to the leaf and frond
/// geometry, optionally runs the geometry's `00b2ed70` pass, applies `matrix` (when not null)
/// to the geometries and collision objects, takes the extents of the geometries into the tree
/// sizes (the largest distance of the box corners from the origin, doubled, at `+0x18`),
/// computes the projected shadow's texture coordinates when there is one, and marks the tree
/// computed. Returns the computed flag.
pub fn c_speed_tree_rt_compute(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    matrix: Ptr,
    seed: u32,
    flag: bool,
) -> u8 {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) == 0 {
        let frond_engine = e.get(this, CSpeedTreeRT::m_pFrondEngine);
        e.set_global(CURRENT_FROND_ENGINE, frond_engine.addr());
        let engine = e.get(this, CSpeedTreeRT::m_pEngine);
        e.call(0x00b2_c0c0, &args![engine, seed]);
        let engine = e.get(this, CSpeedTreeRT::m_pEngine);
        let leaf_float = pushed_float(e, this.addr() + 0x24);
        e.call(0x00b2_af80, &args![engine, leaf_float]);
        let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
        let frond_geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
        let frond_engine = e.get(this, CSpeedTreeRT::m_pFrondEngine);
        e.call(0x00b1_8fd0, &args![frond_engine, frond_geometry, lighting]);
        let frond_geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
        let levels = e.mem.u16(frond_geometry.addr() + 0x20);
        e.set(this, CSpeedTreeRT::m_usNumFrondLodLevels, levels);
        // The leaf info (`CTreeEngine` +0x94, `SIdvLeafInfo`) keeps a vector of 0x54-byte entries
        // at `+0x10` of it.
        let engine = e.get(this, CSpeedTreeRT::m_pEngine).addr();
        let entries = engine + 0xA4;
        let bytes = e
            .mem
            .u32(entries + 0x10)
            .wrapping_sub(e.mem.u32(entries + 0x0C)) as i32;
        e.call(0x00b3_cf40, &args![engine + 0x94, bytes / 0x54]);
        let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
        // `CLightingEngine` +0x38 is `m_eLeafLightingMethod` (Xbox PDB).
        if e.mem.u32(lighting.addr() + 0x38) == 1 {
            fn_00b08cc0(e, this);
        }
        let engine = e.get(this, CSpeedTreeRT::m_pEngine).addr();
        let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
        e.call(0x00b3_7510, &args![wind, engine + 0x10C]);
        let engine = e.get(this, CSpeedTreeRT::m_pEngine).addr();
        let leaf_info = engine + 0x94;
        let large_percent = e.mem.u32(engine + 0xEC);
        let leaf_count = e.mem.u32(engine + 0xD8);
        let leaf_geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
        e.call(
            0x00b2_8480,
            &args![leaf_geometry, leaf_count as u16, large_percent, leaf_info],
        );
        let embedded = e.get(this, CSpeedTreeRT::m_pEmbeddedTexCoords);
        if !embedded.is_null() {
            let mut index = 0i32;
            while index < e.mem.i32(embedded.addr()) {
                let entries = e.mem.u32(embedded.addr() + 4);
                let leaf_geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
                e.call(
                    0x00b2_8720,
                    &args![
                        leaf_geometry,
                        index,
                        entries.wrapping_add((index as u32) << 5)
                    ],
                );
                index += 1;
            }
            let mut index = 0i32;
            while index < e.mem.i32(embedded.addr() + 0x10) {
                let entries = e.mem.u32(embedded.addr() + 0x14);
                let flag_byte = e.global::<u8>(FROND_TEXTURE_FLAG);
                let frond_geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
                let frond_engine = e.get(this, CSpeedTreeRT::m_pFrondEngine);
                e.call(
                    0x00b1_c6d0,
                    &args![
                        frond_engine,
                        frond_geometry,
                        index,
                        entries.wrapping_add((index as u32) << 5),
                        flag_byte
                    ],
                );
                index += 1;
            }
        }
        if flag {
            let geometry = e.get(this, CSpeedTreeRT::m_pBranchGeometry);
            e.call(0x00b2_ed70, &args![geometry, 0u32]);
            let geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
            e.call(0x00b2_ed70, &args![geometry, 0u32]);
        }
        if !matrix.is_null() {
            e.with_stack(0x40, |e, local| {
                e.call(0x00b3_ccb0, &args![local]);
                e.call(MEMCPY, &args![local, matrix, 0x40u32]);
                let geometry = e.get(this, CSpeedTreeRT::m_pBranchGeometry);
                e.call(0x00b2_fc50, &args![geometry, local]);
                let leaf_geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
                e.call(0x00b2_8280, &args![leaf_geometry, local]);
                let geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
                e.call(0x00b2_fc50, &args![geometry, local]);
                let collision = e.get(this, CSpeedTreeRT::m_pCollisionObjects);
                if !collision.is_null() {
                    fn_00b04db0(e, collision, local);
                }
            });
        }
        // The two extents: the minimum at `+0`, the maximum at `+0x18`, 0x18 bytes each.
        e.with_stack(0x30, |e, extents| {
            e.call(0x00b3_c930, &args![extents]);
            let geometry = e.get(this, CSpeedTreeRT::m_pBranchGeometry);
            e.call(0x00b2_fa30, &args![geometry, extents]);
            let leaf_geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
            e.call(0x00b2_7910, &args![leaf_geometry, extents]);
            let geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
            e.call(0x00b2_fa30, &args![geometry, extents]);
            let sizes = e.get(this, CSpeedTreeRT::m_pTreeSizes);
            e.call(MEMCPY, &args![sizes, extents, 0xCu32]);
            let sizes = e.get(this, CSpeedTreeRT::m_pTreeSizes).addr();
            e.call(MEMCPY, &args![sizes + 0xC, extents.addr() + 0x18, 0xCu32]);
            // The farthest of the four corners of the box's x/y rectangle, `(+0|+0xC, +4|+0x10)`,
            // from the origin; the exe measures a corner again when it keeps it.
            let best = e.with_stack(0x18, |e, scratch| {
                let origin = scratch.addr();
                let corner = origin + 0xC;
                let measure = |e: &mut Engine, x: u32, y: u32| {
                    let sizes = e.get(this, CSpeedTreeRT::m_pTreeSizes).addr();
                    copy_float(e, corner, sizes + x);
                    copy_float(e, corner + 4, sizes + y);
                    e.mem.set_f32(corner + 8, 0.0);
                    c_vec3_distance(e, Ptr::new(origin), Ptr::new(corner))
                };
                let mut best = x87_quiet(measure(e, 0, 4));
                for (x, y) in [(0u32, 0x10u32), (0xC, 4), (0xC, 0x10)] {
                    let distance = measure(e, x, y);
                    // `FCOMPP`: the new corner replaces the best unless best > distance.
                    best = if best > distance {
                        x87_quiet(best)
                    } else {
                        x87_quiet(measure(e, x, y))
                    };
                }
                best
            });
            let sizes = e.get(this, CSpeedTreeRT::m_pTreeSizes).addr();
            let best = f64::from(best);
            fstp(e, sizes + 0x18, x87_add(best, best));
            e.call(TRIVIAL_DESTRUCTOR, &args![extents.addr() + 0x18]);
            e.call(TRIVIAL_DESTRUCTOR, &args![extents]);
        });
        if !e.get(this, CSpeedTreeRT::m_pProjectedShadow).is_null() {
            c_speed_tree_rt_compute_self_shadow_tex_coords(e, this);
        }
        fn_00b0d5f0(e, this);
        e.set(this, CSpeedTreeRT::m_bTreeComputed, 1);
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4998);
    }
    e.get(this, CSpeedTreeRT::m_bTreeComputed)
}

// Translated from 00b04d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec3::Distance` (Xbox PDB): the distance from `this` to `other`, by the fast square-root
/// approximation: the sum of the squared differences is stored as a float, its bits are
/// shifted right (arithmetically) and `0x1fc00000` is added, and the result is read back as a
/// float.
pub fn c_vec3_distance(e: &mut Engine, this: Ptr<CVec3>, other: Ptr<CVec3>) -> f32 {
    let (a, b) = (this.addr(), other.addr());
    // `FLD this.axis; FSUBR other.axis` twice, then `FMULP`: the square of `other - this`.
    let mut squares = [0.0f64; 3];
    for (slot, axis) in [8u32, 4, 0].into_iter().enumerate() {
        let first = x87_sub(operand(e, b + axis), fld(e, a + axis));
        let second = x87_sub(operand(e, b + axis), fld(e, a + axis));
        squares[slot] = x87_mul(first, second);
    }
    // The stack holds z*z, y*y, x*x: the first `FADDP` adds x*x to y*y, the second z*z.
    let sum = x87_add(squares[0], x87_add(squares[1], squares[2]));
    let bits = x87_store32(sum);
    let approximate = (((bits as i32) >> 1) as u32).wrapping_add(0x1fc0_0000);
    f32::from_bits(x87_quiet_bits(approximate))
}

layout! {
    /// `SShape` (Xbox PDB), 0x1C bytes: a collision shape.
    pub struct SShape: 0x1C {
        /// `m_eType` (Xbox PDB): the collision object type (0, 1 or 2 are scaled below).
        0x00 m_eType: u32,
        /// `m_afPosition` (Xbox PDB): three floats.
        0x04 m_afPosition: f32,
        /// `m_afDimensions` (Xbox PDB): three floats.
        0x10 m_afDimensions: f32,
    }
}

/// `std::vector<SShape>` iterator helpers of this unit: `operator++` (`00b0eb30`, 0x1C bytes
/// per step) and `operator*` (`00b0ebb0`) share the checked-iterator test; see
/// [`check_iterator_in_range`].
///
/// An iterator is two words: the vector's proxy (`+0`, whose first word is the vector) and
/// the element pointer (`+4`). The test calls the invalid-parameter path when the proxy is
/// null or the pointer is not below the vector's end pointer (`+0x10`).
fn check_iterator_in_range(e: &mut Engine, iterator: u32) {
    let proxy = e.mem.u32(iterator);
    if proxy == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    // A null proxy reads the vector through address 0, as the exe does.
    let vector = if e.mem.u32(iterator) != 0 {
        e.mem.u32(proxy)
    } else {
        0
    };
    if e.mem.u32(iterator + 4) >= e.mem.u32(vector + 0x10) {
        e.call(INVALID_PARAMETER, &args![]);
    }
}

// Translated from 00b0eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checked `operator++` of the `std::vector<SShape>` iterator at `this`: the range check, then
/// the element pointer moves on by 0x1C bytes. Returns `this`.
pub fn fn_00b0eb30(e: &mut Engine, this: Ptr) -> Ptr {
    check_iterator_in_range(e, this.addr());
    let position = e.mem.u32(this.addr() + 4).wrapping_add(0x1C);
    e.mem.set_u32(this.addr() + 4, position);
    this
}

// Translated from 00b0ebb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checked `operator*` of the `std::vector` iterator at `this`: the range check, then the
/// element pointer.
pub fn fn_00b0ebb0(e: &mut Engine, this: Ptr) -> u32 {
    check_iterator_in_range(e, this.addr());
    e.mem.u32(this.addr() + 4)
}

// Translated from 00b0ec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checked `operator++` of the iterator of a vector of 4-byte elements at `this`: the range
/// check, then the element pointer moves on by 4. Returns `this`.
pub fn fn_00b0ec20(e: &mut Engine, this: Ptr) -> Ptr {
    check_iterator_in_range(e, this.addr());
    let position = e.mem.u32(this.addr() + 4).wrapping_add(4);
    e.mem.set_u32(this.addr() + 4, position);
    this
}

// Translated from 00b04db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The transform of the tree's collision shapes by `matrix` (a 16-float matrix; the unit's own
/// function, unnamed in the engine map): `this` is the `SCollisionObjects` (a vector of
/// `SShape` at `+0`). Each shape's position goes through the vector-times-matrix `00b05000`;
/// the dimensions are scaled by the matrix diagonal: type 0 scales the first by `m[0]`, type 1
/// the first two by `m[0]`, type 2 the three by `m[0]`, `m[5]` and `m[10]`.
pub fn fn_00b04db0(e: &mut Engine, this: Ptr, matrix: Ptr) {
    let vector = this.addr();
    let m = matrix.addr();
    // The two iterators (8 bytes each), the position (12) and the vector-times-matrix result slot
    // (12) are locals of the exe.
    e.with_stack(0x28, |e, frame| {
        let begin = Ptr::new(frame.addr());
        let end = frame.addr() + 8;
        let position = frame.addr() + 0x10;
        let slot = frame.addr() + 0x1C;
        let start = e.mem.u32(vector + 0x0C);
        e.call(ITERATOR_CONSTRUCT, &args![begin, start, this]);
        loop {
            let last = e.mem.u32(vector + 0x10);
            e.call(ITERATOR_CONSTRUCT, &args![end, last, this]);
            if e.call(ITERATOR_EQUAL, &args![begin, end]).bool() {
                break;
            }
            let for_z = fn_00b0ebb0(e, begin);
            let for_y = fn_00b0ebb0(e, begin);
            let for_x = fn_00b0ebb0(e, begin);
            copy_float(e, position, for_x + 4);
            copy_float(e, position + 4, for_y + 8);
            copy_float(e, position + 8, for_z + 0xC);
            let moved = fn_00b05000(e, Ptr::new(position), Ptr::new(slot), matrix);
            copy_words(e, position, moved.addr(), 3);
            for axis in 0..3u32 {
                let shape = fn_00b0ebb0(e, begin);
                copy_float(e, shape + 4 + 4 * axis, position + 4 * axis);
            }
            let shape = fn_00b0ebb0(e, begin);
            if e.mem.u32(shape) == 0 {
                let shape = fn_00b0ebb0(e, begin);
                scale_dimension(e, shape + 0x10, m);
            } else {
                let shape = fn_00b0ebb0(e, begin);
                if e.mem.u32(shape) == 1 {
                    let shape = fn_00b0ebb0(e, begin);
                    scale_dimension(e, shape + 0x10, m);
                    let shape = fn_00b0ebb0(e, begin);
                    scale_dimension(e, shape + 0x14, m);
                } else {
                    let shape = fn_00b0ebb0(e, begin);
                    if e.mem.u32(shape) == 2 {
                        let shape = fn_00b0ebb0(e, begin);
                        scale_dimension(e, shape + 0x10, m);
                        let shape = fn_00b0ebb0(e, begin);
                        scale_dimension(e, shape + 0x14, m + 0x14);
                        let shape = fn_00b0ebb0(e, begin);
                        scale_dimension(e, shape + 0x18, m + 0x28);
                    }
                }
            }
            fn_00b0eb30(e, begin);
        }
    });
}

/// `FLD dword [dimension]; FMUL dword [factor]; FSTP dword [dimension]`.
fn scale_dimension(e: &mut Engine, dimension: u32, factor: u32) {
    let scaled = x87_mul(fld(e, dimension), operand(e, factor));
    fstp(e, dimension, scaled);
}

// Translated from 00b05000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The vector `this` times the 4x4 matrix at `matrix` (the translation in the matrix's `+0x30`
/// column): `x = m[12] + (m[8]*z + (m[4]*y + m[0]*x))`, and the same with `m[13], m[9], m[5],
/// m[1]` for `y` and `m[14], m[10], m[6], m[2]` for `z` (indices in floats). The three results
/// are stored to `result`, which is returned.
pub fn fn_00b05000(e: &mut Engine, this: Ptr, result: Ptr, matrix: Ptr) -> Ptr {
    let (v, m) = (this.addr(), matrix.addr());
    let mut out = [0u32; 3];
    for (axis, slot) in out.iter_mut().enumerate() {
        let offset = 4 * axis as u32;
        let z = x87_mul(fld(e, m + 0x20 + offset), operand(e, v + 8));
        let y = x87_mul(fld(e, m + 0x10 + offset), operand(e, v + 4));
        let x = x87_mul(fld(e, m + offset), operand(e, v));
        let sum = x87_add(fld(e, m + 0x30 + offset), x87_add(z, x87_add(y, x)));
        *slot = x87_store32(sum);
    }
    for (axis, bits) in out.into_iter().enumerate() {
        e.mem
            .set_u32(result.addr() + 4 * axis as u32, x87_quiet_bits(bits));
    }
    result
}

// Translated from 00b050d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b050d0 - the FS:[0] exception frame is not translated.
/// Destructor of the 0x54-byte record `00b05130` constructs (unnamed in the engine map): both of
/// its leading 0x18-byte members have the empty destructor, the second (at `+0x18`) first.
pub fn fn_00b050d0(e: &mut Engine, this: Ptr) {
    e.call(TRIVIAL_DESTRUCTOR, &args![this.addr() + 0x18]);
    e.call(TRIVIAL_DESTRUCTOR, &args![this]);
}

// Translated from 00b05130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x54-byte record (unnamed in the engine map): zeroes the first six words,
/// has the `std::string` tidy (`0044a730`, no buffer, size 0) build the empty member at
/// `+0x18`, then stores the floats `1, 1, 0, 1, 0, 0, 1, 0` from `+0x34` on. Returns `this`.
pub fn fn_00b05130(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [0u32, 4, 8, 0xC, 0x10, 0x14] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(STRING_TIDY, &args![base + 0x18, 0u32, 0u32]);
    for (offset, value) in [
        (0x34u32, 1.0f32),
        (0x38, 1.0),
        (0x3C, 0.0),
        (0x40, 1.0),
        (0x44, 0.0),
        (0x48, 0.0),
        (0x4C, 1.0),
        (0x50, 0.0),
    ] {
        e.mem.set_f32(base + offset, value);
    }
    this
}

// Translated from 00b051e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree an instance was made from: the parent pointer of the instance data (`+0x34`), or
/// null when the tree has no instance data (unnamed in the engine map).
pub fn fn_00b051e0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> Ptr<CSpeedTreeRT> {
    let data = e.get(this, CSpeedTreeRT::m_pInstanceData);
    if data.is_null() {
        Ptr::NULL
    } else {
        e.get(data, STreeInstanceData::m_pParent)
    }
}

// Translated from 00b05210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b05210 - the FS:[0] exception frame (releasing the block if the
// constructor throws) is not translated.
/// Makes a new instance of the tree (unnamed in the engine map): allocates 0xA0 bytes and
/// builds the `CSpeedTreeRT` in it from `this` (`00b036d0`); null when the allocation fails.
pub fn fn_00b05210(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> Ptr<CSpeedTreeRT> {
    let block: Ptr<CSpeedTreeRT> = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0xA0u32]).ptr();
    if block.is_null() {
        Ptr::NULL
    } else {
        fn_00b036d0(e, block, this)
    }
}

// Translated from 00b053e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::DeleteTransientData` (Xbox PDB): frees the engine's transient data
/// (`00b2b940`) while it is intact (`CTreeEngine` +0x21); otherwise reports through `SetError`.
pub fn c_speed_tree_rt_delete_transient_data(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
    if e.mem.u8(engine.addr() + ENGINE_TRANSIENT_DATA_INTACT) != 0 {
        e.call(0x00b2_b940, &args![engine]);
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_49f4);
    }
}

/// `IdvFormatString` (`00b18860`, `cdecl`, variadic: the `std::string` to fill, the format, the
/// arguments), `std::exception::exception(const char *const &)` (`00ec78f1`), the CRT's `_errno`
/// (`00ec85e3`) and `strerror` (`00ecdb42`), the throw info of the thrown `std::exception`
/// (`0116f904`).
const FORMAT_STRING: u32 = 0x00b1_8860;
const EXCEPTION_CONSTRUCT: u32 = 0x00ec_78f1;
const ERRNO: u32 = 0x00ec_85e3;
const STRERROR: u32 = 0x00ec_db42;
const EXCEPTION_THROW_INFO: u32 = 0x0116_f904;
/// `FileFinder::GetFile_ov2` (`00afdf20`, `cdecl`: the name, 0, 0x2800, 1).
const FILE_FINDER_GET_FILE: u32 = 0x00af_df20;

/// Formats the message `format` with the words that follow it into a `std::string`
/// (`IdvFormatString`) and throws it as a `std::exception`; the string's text is its inline
/// buffer (`+4`) or, from a reserve of 0x10 on, the heap pointer there.
fn throw_formatted(e: &mut Engine, format: &[u32]) -> ! {
    e.with_stack(0x1C, |e, text| {
        let mut words = vec![text.addr()];
        words.extend_from_slice(format);
        e.call(FORMAT_STRING, &words);
        let reserve = e.mem.u32(text.addr() + 0x18);
        let characters = if reserve >= 0x10 {
            e.mem.u32(text.addr() + 4)
        } else {
            text.addr() + 4
        };
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), characters);
            e.with_stack(0xC, |e, exception| {
                e.call(EXCEPTION_CONSTRUCT, &args![exception, slot]);
                e.call(CXX_THROW_EXCEPTION, &args![exception, EXCEPTION_THROW_INFO]);
            });
        });
    });
    unreachable!("_CxxThrowException does not return (00ec7894)")
}

/// `strerror(errno)`: the message text of the last error.
fn last_error_text(e: &mut Engine) -> u32 {
    let code = e.call(ERRNO, &args![]).u32();
    let value = e.mem.u32(code);
    e.call(STRERROR, &args![value]).u32()
}

// Translated from 00b05420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b05420 - the FS:[0] exception frame (releasing the file and the
// buffer when an error is thrown) is not translated.
/// `CSpeedTreeRT::LoadTree` (Xbox PDB), the file form: opens `filename` through the file finder
/// (`00afdf20`), asks the file object for its size (virtual `+0x28`), reads the whole file into
/// a buffer through the stream's own read word at `+8` of the object, not a virtual slot
/// (`cdecl`, `00b0551f`..`00b05527`: the file, the buffer, the size, the address of a word
/// holding 1, and 1; the file's position at `+4` then moves on by the bytes read), loads the tree from the buffer (`00b059e0`), frees the buffer and deletes the file
/// (virtual `+0`, flag 1). A file that does not open, is empty or reads short throws a
/// `std::exception` with a formatted message. A null name loads nothing. Returns whether the
/// tree loaded.
pub fn c_speed_tree_rt_load_tree(e: &mut Engine, this: Ptr<CSpeedTreeRT>, filename: u32) -> u8 {
    let mut loaded = 0u8;
    if filename != 0 {
        let file = e
            .call(
                FILE_FINDER_GET_FILE,
                &args![filename, 0u32, 0x2800u32, 1u32],
            )
            .u32();
        if file != 0 {
            let size = e.vcall(file, 0x28, &args![]).i32();
            // The exe also tests `0 >= 0` here; it always holds.
            if size > 0 {
                let buffer = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
                let read = e.with_stack(4, |e, flag| {
                    e.mem.set_u32(flag.addr(), 1);
                    // 00b0551f..00b05525: the stream's own read word at `+8` (not a virtual
                    // slot), called cdecl with the file first (`ADD ESP,0x14` at 00b05527).
                    let read_word = e.mem.u32(file + 8);
                    e.call(read_word, &args![file, buffer, size, flag, 1u32])
                        .u32()
                });
                let position = e.mem.u32(file + 4).wrapping_add(read);
                e.mem.set_u32(file + 4, position);
                if read as i32 == size {
                    loaded = fn_00b059e0(e, this, buffer, size as u32);
                    e.call(DEALLOCATE, &args![MEMORY_MANAGER, buffer]);
                } else {
                    let reason = last_error_text(e);
                    throw_formatted(e, &[0x010a_4acc, read, size as u32, filename, reason]);
                }
            } else {
                let reason = last_error_text(e);
                throw_formatted(e, &[0x010a_4aac, filename, reason]);
            }
            e.vcall(file, 0, &args![1u32]);
        } else {
            let reason = last_error_text(e);
            throw_formatted(e, &[0x010a_4a8c, filename, reason]);
        }
    }
    loaded
}

// Translated from 00b059e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b059e0 - the FS:[0] exception frame (destroying the file-access
// buffer when a parser throws) is not translated.
/// `CSpeedTreeRT::LoadTree` (Xbox PDB), the memory form (unnamed in the engine map): wraps
/// `size` bytes at `data` in a `CTreeFileAccess` (`00b06090`), lets the engine check the header
/// (`CTreeEngine::Parse`, `00b2b550`), then reads chunk tags until the data ends or a tag is
/// unknown, handing each to its parser: 8000 the lighting engine (`00b2a940`), 9000 lod info,
/// 10000, 11000 wind info, 12000 collision objects, 13000 the frond engine (`00b18340`), 15000,
/// 16000 and 16013 the tree engine (`00b2d090`, `00b2d2e0`, `00b2daf0`), 16014 a float into
/// `+0x28`, 18000, 19000 user data, 20000, 21000 and 21001 floats into the wind engine's
/// `+0x3C` and `+0x40`, and 22000 a byte flag. Then the three lighting methods are set again
/// from the lighting engine (`SetBranchLightingMethod` and its two siblings), the branch wind
/// level and the frond level are refreshed, a leaf lod method of 2 becomes 1 with the transition
/// factor from `01016248`, and the wind matrices are set up. Returns 1 when the engine accepted
/// the header.
pub fn fn_00b059e0(e: &mut Engine, this: Ptr<CSpeedTreeRT>, data: u32, size: u32) -> u8 {
    let mut loaded = 0u8;
    // The `CTreeFileAccess` (0x1c bytes: the index and the 0x18-byte byte vector, whose
    // constructor 00657ee0 clears its words up to +0x14; EBP-0x30 to EBP-0x14 in the exe).
    e.with_stack(0x1c, |e, access| {
        fn_00b06090(e, access, data, size as i32);
        let engine = e.get(this, CSpeedTreeRT::m_pEngine);
        if e.call(0x00b2_b550, &args![engine, access]).bool() {
            let at_end = |e: &Engine| {
                let vector = access.addr() + 4;
                let length = e
                    .mem
                    .u32(vector + 0x10)
                    .wrapping_sub(e.mem.u32(vector + 0x0C));
                e.mem.u32(access.addr()) >= length
            };
            if !at_end(e) {
                let mut done = false;
                let mut tag = e.call(0x00b3_ed80, &args![access]).i32();
                while !done {
                    let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
                    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
                    match tag {
                        8000 => {
                            e.call(0x00b2_a940, &args![lighting, access]);
                        }
                        9000 => c_speed_tree_rt_parse_lod_info(e, this, access),
                        11000 => c_speed_tree_rt_parse_wind_info(e, this, access),
                        10000 => fn_00b09960(e, this, access),
                        12000 => c_speed_tree_rt_parse_collision_objects(e, this, access),
                        13000 => {
                            let frond_engine = e.get(this, CSpeedTreeRT::m_pFrondEngine);
                            e.call(0x00b1_8340, &args![frond_engine, access]);
                        }
                        15000 => {
                            e.call(0x00b2_d090, &args![engine, access]);
                        }
                        16000 => {
                            e.call(0x00b2_d2e0, &args![engine, access]);
                        }
                        16013 => {
                            e.call(0x00b2_daf0, &args![engine, access]);
                        }
                        16014 => {
                            let value = e.call(0x00b3_ed20, &args![access]).f32();
                            e.mem
                                .set_u32(this.addr() + 0x28, x87_quiet_bits(value.to_bits()));
                        }
                        18000 => fn_00b0cd40(e, this, access),
                        19000 => c_speed_tree_rt_parse_user_data(e, this, access),
                        20000 => fn_00b0d810(e, this, access),
                        21000 => {
                            let value = e.call(0x00b3_ed20, &args![access]).f32();
                            let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
                            e.mem
                                .set_u32(wind.addr() + 0x3C, x87_quiet_bits(value.to_bits()));
                        }
                        21001 => {
                            let value = e.call(0x00b3_ed20, &args![access]).f32();
                            let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
                            e.mem
                                .set_u32(wind.addr() + 0x40, x87_quiet_bits(value.to_bits()));
                        }
                        22000 => {
                            let flag = fn_00b06180(e, access);
                            e.set_global(TREE_FILE_FLAG, flag as u8);
                        }
                        _ => done = true,
                    }
                    if at_end(e) {
                        done = true;
                    } else {
                        tag = e.call(0x00b3_ed80, &args![access]).i32();
                    }
                }
            }
            let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
            let method = e.mem.u32(lighting.addr());
            c_speed_tree_rt_set_branch_lighting_method(e, this, method);
            let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
            let method = e.mem.u32(lighting.addr() + 0x38);
            c_speed_tree_rt_set_leaf_lighting_method(e, this, method);
            let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
            let method = e.mem.u32(lighting.addr() + 0x78);
            c_speed_tree_rt_set_frond_lighting_method(e, this, method);
            let engine = e.get(this, CSpeedTreeRT::m_pEngine);
            let level = e.mem.i32(engine.addr() + 0x108);
            e.set(this, CSpeedTreeRT::m_nBranchWindLevel, level);
            let frond_engine = e.get(this, CSpeedTreeRT::m_pFrondEngine);
            let frond_level = e.call(0x00b1_8300, &args![frond_engine]).i32();
            e.set(this, CSpeedTreeRT::m_nFrondLevel, frond_level);
            if e.get(this, CSpeedTreeRT::m_eLeafLodMethod) == 2 {
                e.set(this, CSpeedTreeRT::m_eLeafLodMethod, 1);
                let factor = x87_quiet_bits(e.global::<u32>(0x0101_6248));
                e.mem.set_u32(this.addr() + 0x28, factor);
            }
            let span = e.global::<u16>(WIND_MATRIX_SPAN);
            let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
            e.call(0x00b3_7570, &args![wind, 0u32, span]);
            loaded = 1;
        }
        // The access object's destructor, inlined: the vector's clear (`00657f60`), then its
        // block is freed.
        e.call(0x0065_7f60, &args![access.addr() + 4]);
        let block = e.mem.u32(access.addr() + 4);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
    });
    loaded
}

// Translated from 00b06090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b06090 - the FS:[0] exception frame is not translated.
/// `CTreeFileAccess::CTreeFileAccess` over memory (unnamed in the engine map): the index is
/// zero, the byte vector (`00657b50`) is grown to `size` zeroes (`00b0e200`) and filled from
/// `data` through the checked index. Returns `this`.
pub fn fn_00b06090(e: &mut Engine, this: Ptr, data: u32, size: i32) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.call(0x0065_7b50, &args![this.addr() + 4]);
    e.call(0x00b0_e200, &args![this.addr() + 4, size, 0u32]);
    let vector = this.addr() + 4;
    let mut index = 0i32;
    while index < size {
        let length = e
            .mem
            .u32(vector + 0x10)
            .wrapping_sub(e.mem.u32(vector + 0x0C));
        if (index as u32) >= length {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let first = e.mem.u32(vector + 0x0C);
        let byte = e.mem.u8(data.wrapping_add(index as u32));
        e.mem.set_u8(first.wrapping_add(index as u32), byte);
        index += 1;
    }
    this
}

// Translated from 00b06180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CTreeFileAccess::ParseBool` (unnamed in the engine map): the byte at the file index (a
/// checked read of the byte vector), the index moving on; whether it is not zero.
pub fn fn_00b06180(e: &mut Engine, this: Ptr) -> bool {
    let index = e.mem.u32(this.addr());
    e.mem.set_u32(this.addr(), index.wrapping_add(1));
    let vector = this.addr() + 4;
    let length = e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C));
    if index >= length {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let first = e.mem.u32(vector + 0x0C);
    e.mem.u8(first.wrapping_add(index)) != 0
}

// Translated from 00b061e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b061e0 - the FS:[0] exception frame is not translated.
/// `CTreeFileAccess::~CTreeFileAccess` (unnamed in the engine map): clears the byte vector
/// (`00657f60`) and frees its block.
pub fn fn_00b061e0(e: &mut Engine, this: Ptr) {
    e.call(0x0065_7f60, &args![this.addr() + 4]);
    let block = e.mem.u32(this.addr() + 4);
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

// Translated from 00b06260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands two output addresses to the engine (unnamed in the engine map): `00b2bcf0` stores the
/// engine's `+0x4C` and `+0x50` floats through them (`00b2bd20`, Ghidra's `CTreeEngine::SetSize`,
/// sets the same pair).
pub fn fn_00b06260(e: &mut Engine, this: Ptr<CSpeedTreeRT>, size: u32, variance: u32) {
    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
    e.call(0x00b2_bcf0, &args![engine, size, variance]);
}

// Translated from 00b06280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b06280 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetTreeSize` (Xbox PDB): with the engine's transient data intact and a size
/// above zero (a NaN is rejected), hands both floats (through the x87 stack) to the engine
/// (`00b2bd20`); a size not above zero and intact data missing are reported through `SetError`.
pub fn c_speed_tree_rt_set_tree_size(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    size: f32,
    variance: f32,
) {
    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
    if e.mem.u8(engine.addr() + ENGINE_TRANSIENT_DATA_INTACT) != 0 {
        let zero: f64 = e.global(0x0101_2060);
        if x87_load32(size.to_bits()) > zero {
            e.call(
                0x00b2_bd20,
                &args![
                    engine,
                    x87_quiet_bits(size.to_bits()),
                    x87_quiet_bits(variance.to_bits())
                ],
            );
        } else {
            c_speed_tree_rt_set_error(e, 0x010a_4b68);
        }
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4b20);
    }
}

// Translated from 00b06440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b06440 - the FS:[0] exception frame is not translated.
/// The engine's `+0x48` word (the branch wind level the engine holds), when the engine's
/// transient data is intact; otherwise `SetError` is told and 0 returned (unnamed in the engine
/// map).
pub fn fn_00b06440(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> u32 {
    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
    if e.mem.u8(engine.addr() + ENGINE_TRANSIENT_DATA_INTACT) != 0 {
        e.mem.u32(engine.addr() + 0x48)
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4b20);
        0
    }
}

// Translated from 00b065e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a position (three floats, copied through the x87 stack): into the instance data's
/// `+4` when the tree has instance data, else into the engine's `+4` (`CIdvCamera::m_cPosition`
/// in the Xbox PDB). Unnamed in the engine map.
pub fn fn_00b065e0(e: &mut Engine, this: Ptr<CSpeedTreeRT>, x: f32, y: f32, z: f32) {
    let (x, y, z) = (
        x87_quiet_bits(x.to_bits()),
        x87_quiet_bits(y.to_bits()),
        x87_quiet_bits(z.to_bits()),
    );
    let data = e.get(this, CSpeedTreeRT::m_pInstanceData);
    let target = if !data.is_null() {
        data.addr() + 4
    } else {
        e.mem.u32(this.addr()) + 4
    };
    e.mem.set_u32(target, x);
    e.mem.set_u32(target + 4, y);
    e.mem.set_u32(target + 8, z);
}

// Translated from 00b06660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b06660 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetBranchLightingMethod` (Xbox PDB): before `Compute`, sets the branch
/// geometry's manual-lighting flag (`+9`: the method is 1) and the lighting engine's branch
/// method (`+0`); afterwards reports through `SetError`.
pub fn c_speed_tree_rt_set_branch_lighting_method(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    method: u32,
) {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) == 0 {
        let geometry = e.get(this, CSpeedTreeRT::m_pBranchGeometry);
        e.mem.set_u8(geometry.addr() + 9, (method == 1) as u8);
        let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
        e.mem.set_u32(lighting.addr(), method);
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4bd0);
    }
}

// Translated from 00b06810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b06810 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetLeafLightingMethod` (Xbox PDB): before `Compute`, sets the lighting
/// engine's leaf method (`+0x38`) and the leaf geometry's manual-lighting flag (`+0`: the
/// method is 1); afterwards reports through `SetError`.
pub fn c_speed_tree_rt_set_leaf_lighting_method(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    method: u32,
) {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) == 0 {
        let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
        e.mem.set_u32(lighting.addr() + 0x38, method);
        let geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
        e.mem.set_u8(geometry.addr(), (method == 1) as u8);
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4c40);
    }
}

// Translated from 00b069c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b069c0 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetFrondLightingMethod` (Xbox PDB): before `Compute`, sets the frond
/// geometry's manual-lighting flag (`+9`: the method is 1) and the lighting engine's frond
/// method (`+0x78`); afterwards reports through `SetError`.
pub fn c_speed_tree_rt_set_frond_lighting_method(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    method: u32,
) {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) == 0 {
        let geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
        e.mem.set_u8(geometry.addr() + 9, (method == 1) as u8);
        let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
        e.mem.set_u32(lighting.addr() + 0x78, method);
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4cb0);
    }
}

// Translated from 00b06b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::GetCamera` (Xbox PDB), a static function (`cdecl`): copies the stored camera
/// position (`011f8e9c`) and direction (`011f8eac`), three floats each, to the two output
/// addresses; either output null reports through `SetError`.
pub fn c_speed_tree_rt_get_camera(e: &mut Engine, position: u32, direction: u32) {
    if position != 0 && direction != 0 {
        e.with_stack(0x18, |e, locals| {
            let (stored_position, stored_direction) = (locals.addr(), locals.addr() + 0xC);
            for axis in 0..3 {
                e.mem.set_f32(stored_position + 4 * axis, 0.0);
                e.mem.set_f32(stored_direction + 4 * axis, 0.0);
            }
            copy_words(e, stored_position, CAMERA_POSITION, 3);
            copy_words(e, stored_direction, CAMERA_DIRECTION, 3);
            e.call(MEMCPY, &args![position, stored_position, 0xCu32]);
            e.call(MEMCPY, &args![direction, stored_direction, 0xCu32]);
        });
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4cf8);
    }
}

// Translated from 00b06c10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b06c10 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetCamera` (Xbox PDB), a static function (`cdecl`): `position` and `direction`
/// are each three floats (either null reports through `SetError`). When either differs from the
/// stored camera (`011f8e9c`, `011f8eac`; a NaN never equals) the new values are stored, the
/// camera-dependent state is refreshed (`00b08ec0(1)`, the direction handed to `00b27080`,
/// `00b08ec0(1)` again) and the blend `011ad124` is recomputed: the direction's z through the
/// math function `00ec9610` (unconfirmed which: Ghidra's name for the stub is ambiguous) times
/// the double at `010a4d90` is a pitch angle, `t = 1 - (angle - [011ad11c]) / ([011ad120] -
/// [011ad11c])` is clamped to 0..1 (a NaN is kept), and the blend is `t * ([01023cd8] -
/// [010a4d88]) + [010a4d88]`.
pub fn c_speed_tree_rt_set_camera(e: &mut Engine, position: u32, direction: u32) {
    if position != 0 && direction != 0 {
        // The locals of the exe: the stored position and direction (12 bytes each), the new
        // position and direction, and a scratch copy of the direction.
        e.with_stack(0x3C, |e, frame| {
            let base = frame.addr();
            let (old_position, old_direction) = (base, base + 0xC);
            let (new_position, new_direction) = (base + 0x18, base + 0x24);
            let scratch = base + 0x30;
            copy_words(e, old_position, CAMERA_POSITION, 3);
            copy_words(e, old_direction, CAMERA_DIRECTION, 3);
            for axis in 0..3 {
                copy_float(e, new_position + 4 * axis, position + 4 * axis);
                copy_float(e, new_direction + 4 * axis, direction + 4 * axis);
            }
            let differs = |e: &Engine, new: u32, old: u32| {
                (0..3u32).any(|axis| fld(e, new + 4 * axis) != fld(e, old + 4 * axis))
            };
            if differs(e, new_position, old_position) || differs(e, new_direction, old_direction) {
                copy_words(e, CAMERA_POSITION, new_position, 3);
                copy_words(e, CAMERA_DIRECTION, new_direction, 3);
                fn_00b08ec0(e, 1);
                e.call(0x00b2_7080, &args![new_direction]);
                fn_00b08ec0(e, 1);
                copy_float(e, scratch, new_direction);
                copy_float(e, scratch + 4, new_direction + 4);
                copy_float(e, scratch + 8, new_direction + 8);
                // The `[2]` element of the copy: the direction's z through the math function.
                let z = fld(e, scratch + 8);
                let result = e.call(0x00ec_9610, &args![z]).f64();
                let rounded = f64::from(x87_quiet(f32::from_bits(x87_store32(result))));
                let divisor: f64 = e.global(0x010a_4d90);
                let angle = x87_store32(x87_mul(rounded, divisor));
                let low = fld(e, 0x011a_d11c);
                let span = x87_sub(operand(e, 0x011a_d120), low);
                let offset = x87_sub(x87_m32(angle), low);
                let mut blend = f64::from(f32::from_bits(x87_store32(x87_sub(
                    1.0,
                    x87_div(offset, span),
                ))));
                let zero: f64 = e.global(0x0101_2060);
                // `FCOMP`, then a parity test of C0|C2: only a value below zero is replaced.
                if blend < zero {
                    blend = 0.0;
                }
                let one: f64 = e.global(0x0101_2070);
                if blend > one {
                    blend = 1.0;
                }
                let spread = x87_sub(operand(e, 0x0102_3cd8), fld(e, 0x010a_4d88));
                let scaled = x87_add(x87_mul(blend, spread), operand(e, 0x010a_4d88));
                fstp(e, 0x011a_d124, scaled);
            }
        });
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4d4c);
    }
}

// Translated from 00b07060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b07060 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetTime` (unnamed in the engine map): stores the time float (through the x87
/// stack) at `011f8d80` and calls `00b08ec0(0)`.
pub fn fn_00b07060(e: &mut Engine, time: f32) {
    e.set_global(WIND_TIME, x87_quiet_bits(time.to_bits()));
    fn_00b08ec0(e, 0);
}

// Translated from 00b071c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a flag byte at the wind engine's `+0x14` (`m_bRockingLeaves` in the Xbox PDB).
pub fn fn_00b071c0(e: &mut Engine, this: Ptr<CSpeedTreeRT>, flag: u8) {
    let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
    e.mem.set_u8(wind.addr() + 0x14, flag);
}

// Translated from 00b071f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b071f0 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetNumLeafRockingGroups` (Xbox PDB): before `Compute`, stores the count (a
/// zero becomes one) in the engine's `+0xD4`; afterwards reports through `SetError`.
pub fn c_speed_tree_rt_set_num_leaf_rocking_groups(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    count: u32,
) {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) == 0 {
        let count = if count == 0 { 1 } else { count };
        let engine = e.get(this, CSpeedTreeRT::m_pEngine);
        e.mem.set_u32(engine.addr() + 0xD4, count);
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4dd8);
    }
}

// Translated from 00b07390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float (through the x87 stack) at the engine's `+0x9C`; nothing happens when the
/// tree has no engine (unnamed in the engine map).
pub fn fn_00b07390(e: &mut Engine, this: Ptr<CSpeedTreeRT>, value: f32) {
    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
    if !engine.is_null() {
        e.mem
            .set_u32(engine.addr() + 0x9C, x87_quiet_bits(value.to_bits()));
    }
}

// Translated from 00b073c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float (through the x87 stack) at the engine's `+0x98`; nothing happens when the
/// tree has no engine (unnamed in the engine map).
pub fn fn_00b073c0(e: &mut Engine, this: Ptr<CSpeedTreeRT>, value: f32) {
    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
    if !engine.is_null() {
        e.mem
            .set_u32(engine.addr() + 0x98, x87_quiet_bits(value.to_bits()));
    }
}

/// The float at `offset` of the engine's branch geometry (`CTreeEngine` `+0x5C`, a
/// `CIndexedGeometry`) through the x87 stack, or zero when the tree has no engine or the engine
/// no geometry.
fn engine_branch_geometry_float(e: &Engine, this: Ptr<CSpeedTreeRT>, offset: u32) -> f32 {
    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
    if !engine.is_null() {
        let geometry = e.mem.u32(engine.addr() + 0x5C);
        if geometry != 0 {
            return f32::from_bits(x87_quiet_bits(e.mem.u32(geometry + offset)));
        }
    }
    0.0
}

// Translated from 00b073f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The trunk width: `m_fTrunkWidth` (Xbox PDB, `+0x18`) of the engine's branch geometry, or
/// zero without an engine or geometry (unnamed in the engine map).
pub fn fn_00b073f0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> f32 {
    engine_branch_geometry_float(e, this, 0x18)
}

// Translated from 00b07440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The trunk length: `m_fTrunkLength` (Xbox PDB, `+0x1C`) of the engine's branch geometry, or
/// zero without an engine or geometry (unnamed in the engine map).
pub fn fn_00b07440(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> f32 {
    engine_branch_geometry_float(e, this, 0x1C)
}

// Translated from 00b07490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float (through the x87 stack) at the engine's `+0xCC`; nothing happens when the
/// tree has no engine (unnamed in the engine map).
pub fn fn_00b07490(e: &mut Engine, this: Ptr<CSpeedTreeRT>, value: f32) {
    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
    if !engine.is_null() {
        e.mem
            .set_u32(engine.addr() + 0xCC, x87_quiet_bits(value.to_bits()));
    }
}

// Translated from 00b074c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float (through the x87 stack) at the engine's `+0xD0`; nothing happens when the
/// tree has no engine (unnamed in the engine map).
pub fn fn_00b074c0(e: &mut Engine, this: Ptr<CSpeedTreeRT>, value: f32) {
    let engine = e.get(this, CSpeedTreeRT::m_pEngine);
    if !engine.is_null() {
        e.mem
            .set_u32(engine.addr() + 0xD0, x87_quiet_bits(value.to_bits()));
    }
}

// Translated from 00b074f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The leaf wind method: `m_eLeafMethod` (Xbox PDB, `+0x10`) of the wind engine.
pub fn fn_00b074f0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> u32 {
    let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
    e.mem.u32(wind.addr() + 0x10)
}

// Translated from 00b07510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b07510 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetLeafWindMethod` (Xbox PDB): before `Compute`, stores the method in the
/// wind engine (`+0x10`) and sets the leaf geometry's vertex-weighting flag (`+1`: the method
/// is not 2); afterwards reports through `SetError`.
pub fn c_speed_tree_rt_set_leaf_wind_method(e: &mut Engine, this: Ptr<CSpeedTreeRT>, method: u32) {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) == 0 {
        let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
        e.mem.set_u32(wind.addr() + 0x10, method);
        let geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
        e.mem.set_u8(geometry.addr() + 1, (method != 2) as u8);
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4e40);
    }
}

// Translated from 00b076c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The branch wind method: `m_eBranchMethod` (Xbox PDB, `+8`) of the wind engine.
pub fn fn_00b076c0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> u32 {
    let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
    e.mem.u32(wind.addr() + 8)
}

/// The shared part of `SetBranchWindMethod` and `SetFrondWindMethod`: before `Compute`, a
/// method of 2 while the wind engine's method at `method_offset` is 1 clears the geometry's
/// valid flag (`+0x12`) and runs its wind computation (`00b30800`, level 0); then the method is
/// stored in the wind engine, the geometry's `+8` flag becomes "the method is not 2" and its
/// wind method (`+0x14`) the method. Afterwards `SetError` gets `message`.
fn set_geometry_wind_method(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    geometry_field: Field<CSpeedTreeRT, Ptr>,
    method_offset: u32,
    method: u32,
    message: u32,
) {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) == 0 {
        if method == 2 {
            let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
            if e.mem.u32(wind.addr() + method_offset) == 1 {
                let geometry = e.get(this, geometry_field);
                e.mem.set_u8(geometry.addr() + 0x12, 0);
                let geometry = e.get(this, geometry_field);
                e.call(0x00b3_0800, &args![geometry, 0u32]);
            }
        }
        let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
        e.mem.set_u32(wind.addr() + method_offset, method);
        let geometry = e.get(this, geometry_field);
        e.mem.set_u8(geometry.addr() + 8, (method != 2) as u8);
        let geometry = e.get(this, geometry_field);
        e.mem.set_u32(geometry.addr() + 0x14, method);
    } else {
        c_speed_tree_rt_set_error(e, message);
    }
}

// Translated from 00b076e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b076e0 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetBranchWindMethod` (Xbox PDB): see [`set_geometry_wind_method`] (the branch
/// geometry, the wind engine's `+8`; the error message at `010a4ea8`).
pub fn c_speed_tree_rt_set_branch_wind_method(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    method: u32,
) {
    set_geometry_wind_method(
        e,
        this,
        CSpeedTreeRT::m_pBranchGeometry,
        8,
        method,
        0x010a_4ea8,
    );
}

// Translated from 00b078e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The frond wind method: `m_eFrondMethod` (Xbox PDB, `+0xC`) of the wind engine.
pub fn fn_00b078e0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> u32 {
    let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
    e.mem.u32(wind.addr() + 0xC)
}

// Translated from 00b07900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b07900 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetFrondWindMethod` (Xbox PDB): see [`set_geometry_wind_method`] (the frond
/// geometry, the wind engine's `+0xC`; the error message at `010a4f10`).
pub fn c_speed_tree_rt_set_frond_wind_method(e: &mut Engine, this: Ptr<CSpeedTreeRT>, method: u32) {
    set_geometry_wind_method(
        e,
        this,
        CSpeedTreeRT::m_pFrondGeometry,
        0xC,
        method,
        0x010a_4f10,
    );
}

// Translated from 00b07b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b07b00 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetWindStrength` (Xbox PDB): with a strength of zero or more (a NaN is
/// rejected through `SetError`), a second and a third value equal to the double at `0101a6b0`
/// are replaced by the wind engine's `+4` and `+0` floats, the three go to the wind engine's
/// `00b37470` (the strength is the first), whose float result is kept; a branch or frond wind
/// method of 1 invalidates that geometry (`+0x12`), and the leaf geometry's `00b27820` runs
/// unless the leaf wind method is not 1 and the wind engine's rocking-leaves flag (`+0x14`)
/// is clear. Returns the engine's result (0 after an error).
pub fn c_speed_tree_rt_set_wind_strength(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    strength: f32,
    second: f32,
    third: f32,
) -> f32 {
    let mut result = 0.0f32.to_bits();
    let zero: f64 = e.global(0x0101_2060);
    if x87_load32(strength.to_bits()) >= zero {
        let sentinel: f64 = e.global(0x0101_a6b0);
        let mut second = second.to_bits();
        let mut third = third.to_bits();
        if x87_load32(second) == sentinel {
            let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
            second = pushed_float(e, wind.addr() + 4);
        }
        if x87_load32(third) == sentinel {
            let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
            third = pushed_float(e, wind.addr());
        }
        let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
        let value = e
            .call(
                0x00b3_7470,
                &args![
                    wind,
                    x87_quiet_bits(strength.to_bits()),
                    x87_quiet_bits(second),
                    x87_quiet_bits(third)
                ],
            )
            .f32();
        result = x87_quiet_bits(value.to_bits());
        if fn_00b076c0(e, this) == 1 {
            let geometry = e.get(this, CSpeedTreeRT::m_pBranchGeometry);
            if !geometry.is_null() {
                e.mem.set_u8(geometry.addr() + 0x12, 0);
            }
        }
        if fn_00b078e0(e, this) == 1 {
            let geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
            if !geometry.is_null() {
                e.mem.set_u8(geometry.addr() + 0x12, 0);
            }
        }
        let rocking = {
            let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
            e.mem.u8(wind.addr() + 0x14) != 0
        };
        if fn_00b074f0(e, this) == 1 || rocking {
            let leaf_geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
            e.call(0x00b2_7820, &args![leaf_geometry]);
        }
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4f74);
    }
    f32::from_bits(result)
}

// Translated from 00b07db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b07db0 - the FS:[0] exception frame is not translated.
/// A static function (`cdecl`, one word): `00b07f20` on the static at `011f8d84` with the low
/// 16 bits of the argument (unnamed in the engine map).
pub fn fn_00b07db0(e: &mut Engine, value: u16) {
    fn_00b07f20(e, Ptr::new(WIND_MATRIX_SPAN), value);
}

// Translated from 00b07f20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b07f20 - the FS:[0] exception frame (freeing the array when an
// element constructor throws) is not translated.
/// Sets the number of wind matrices of the table `this` (the static at `011f8d84`: the count
/// word at `+0`, the array of 0x40-byte matrices at `+4`; unnamed in the engine map). A changed
/// count frees the old array, stores the count and allocates `count * 0x40` bytes (a product
/// that overflows becomes 0xFFFFFFFF), constructing each matrix with `00b3ccb0`; then every
/// matrix is reset with `00b3ccd0`.
pub fn fn_00b07f20(e: &mut Engine, this: Ptr, count: u16) {
    let base = this.addr();
    if e.mem.u16(base) != count {
        let old = e.mem.u32(base + 4);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, old]);
        e.mem.set_u16(base, count);
        let size = u32::from(count).checked_mul(0x40).unwrap_or(u32::MAX);
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        let array = if block != 0 {
            let mut remaining = i32::from(count);
            let mut element = block;
            loop {
                remaining -= 1;
                if remaining < 0 {
                    break;
                }
                e.call(0x00b3_ccb0, &args![element]);
                element = element.wrapping_add(0x40);
            }
            block
        } else {
            0
        };
        e.mem.set_u32(base + 4, array);
    }
    let mut index = 0i32;
    while index < i32::from(count) {
        let element = ((index as u32) << 6).wrapping_add(e.mem.u32(base + 4));
        e.call(0x00b3_ccd0, &args![element]);
        index += 1;
    }
}

// Translated from 00b08050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b08050 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::SetLocalMatrices` (Xbox PDB): before `Compute`, when `first + count - 1` is
/// below the matrix count word at `011f8d84`, hands both to the wind engine's
/// `SetLocalMatrices` (`00b37570`); otherwise reports through `SetError` (a range error, or
/// that the tree is already computed).
pub fn c_speed_tree_rt_set_local_matrices(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    first: u32,
    count: u32,
) {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) == 0 {
        let limit = u32::from(e.global::<u16>(WIND_MATRIX_SPAN));
        if first.wrapping_add(count).wrapping_sub(1) < limit {
            let wind = e.get(this, CSpeedTreeRT::m_pWindEngine);
            e.call(0x00b3_7570, &args![wind, first, count]);
        } else {
            c_speed_tree_rt_set_error(e, 0x010a_5034);
        }
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_4ff0);
    }
}

// Translated from 00b08200 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b08200 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::ComputeLodLevel` (Xbox PDB): with instance data, lends the instance's
/// position to the engine (saving the engine's position words and its level-of-detail float at
/// `+0x14`), lets the engine compute the level (`CTreeEngine::ComputeLod`, `00b2bd40`), keeps
/// it in the instance data (`+0x10`) and restores the engine's float and position; without
/// instance data the engine computes it and the result is dropped.
pub fn c_speed_tree_rt_compute_lod_level(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    if !e.get(this, CSpeedTreeRT::m_pInstanceData).is_null() {
        let engine = e.mem.u32(this.addr());
        let mut saved_position = [0u32; 3];
        for (axis, word) in saved_position.iter_mut().enumerate() {
            *word = e.mem.u32(engine + 4 + 4 * axis as u32);
        }
        let saved_level = pushed_float(e, engine + 0x14);
        let data = e.get(this, CSpeedTreeRT::m_pInstanceData);
        let engine = e.mem.u32(this.addr());
        copy_words(e, engine + 4, data.addr() + 4, 3);
        let engine = e.mem.u32(this.addr());
        let level = e.call(0x00b2_bd40, &args![engine]).f32();
        let data = e.get(this, CSpeedTreeRT::m_pInstanceData);
        e.mem
            .set_u32(data.addr() + 0x10, x87_quiet_bits(level.to_bits()));
        let engine = e.mem.u32(this.addr());
        e.mem.set_u32(engine + 0x14, x87_quiet_bits(saved_level));
        let engine = e.mem.u32(this.addr());
        for (axis, word) in saved_position.into_iter().enumerate() {
            e.mem.set_u32(engine + 4 + 4 * axis as u32, word);
        }
    } else {
        let engine = e.mem.u32(this.addr());
        e.call(0x00b2_bd40, &args![engine]);
    }
}

// Translated from 00b08440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::GetLodLevel` (Xbox PDB): the instance's level of detail (`+0x10` of the
/// instance data) when the tree has instance data, else the engine's (`+0x14`); the float at
/// `01012054` is loaded first and replaced.
pub fn c_speed_tree_rt_get_lod_level(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> f32 {
    let _initial = pushed_float(e, 0x0101_2054);
    let data = e.get(this, CSpeedTreeRT::m_pInstanceData);
    let level = if !data.is_null() {
        pushed_float(e, data.addr() + 0x10)
    } else {
        let engine = e.mem.u32(this.addr());
        pushed_float(e, engine + 0x14)
    };
    f32::from_bits(x87_quiet_bits(level))
}

// Translated from 00b08480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::SetLodLevel` (Xbox PDB): a level from 0 to 1 (both ends included; a NaN is
/// rejected) is stored (through the x87 stack) in the instance data's `+0x10` when the tree has
/// instance data, else in the engine's `+0x14`; any other level reports through `SetError`.
pub fn c_speed_tree_rt_set_lod_level(e: &mut Engine, this: Ptr<CSpeedTreeRT>, level: f32) {
    let zero: f64 = e.global(0x0101_2060);
    let one: f64 = e.global(0x0101_2070);
    let value = x87_load32(level.to_bits());
    if (zero..=one).contains(&value) {
        let data = e.get(this, CSpeedTreeRT::m_pInstanceData);
        if !data.is_null() {
            e.mem
                .set_u32(data.addr() + 0x10, x87_quiet_bits(level.to_bits()));
        } else {
            let engine = e.mem.u32(this.addr());
            e.mem
                .set_u32(engine + 0x14, x87_quiet_bits(level.to_bits()));
        }
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_5094);
    }
}

// Translated from 00b084f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A static function (`cdecl`, one word): stores the low byte of the argument at `011f8b78`
/// (unnamed in the engine map).
pub fn fn_00b084f0(e: &mut Engine, value: u32) {
    e.set_global(DISCRETE_LEAF_LOD_FLAG, value as u8);
}

// Translated from 00b08500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands two floats (through the x87 stack) to the engine's `00b2bdc0` (unnamed in the engine
/// map).
pub fn fn_00b08500(e: &mut Engine, this: Ptr<CSpeedTreeRT>, first: f32, second: f32) {
    let engine = e.mem.u32(this.addr());
    e.call(
        0x00b2_bdc0,
        &args![
            engine,
            x87_quiet_bits(first.to_bits()),
            x87_quiet_bits(second.to_bits())
        ],
    );
}

/// `FLD [1 - level]; FIMUL levels; _ftol2`: the discrete level of a level-of-detail float
/// that is `-1` for "the tree's own level" (`GetLodLevel`), out of `levels` levels; the top
/// of the range is pulled back by one (the exe compares in 32 bits after a sign extension).
fn discrete_level_through_ftol(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    level: f32,
    levels: impl Fn(&mut Engine) -> u32,
) -> u16 {
    let mut level = level.to_bits();
    let sentinel: f64 = e.global(0x0101_a6b0);
    if x87_load32(level) == sentinel {
        level = c_speed_tree_rt_get_lod_level(e, this).to_bits();
    }
    let count = levels(e);
    let scaled = x87_mul(x87_sub(1.0, x87_load32(level)), f64::from(count));
    let mut discrete = ftol(e, scaled) as u16;
    if i32::from(discrete as i16) == count as i32 {
        discrete = discrete.wrapping_sub(1);
    }
    discrete
}

// Translated from 00b08530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::GetDiscreteBranchLodLevel` (Xbox PDB): see [`discrete_level_through_ftol`]
/// with the branch level count of `GetNumBranchLodLevels`.
pub fn c_speed_tree_rt_get_discrete_branch_lod_level(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    level: f32,
) -> u16 {
    discrete_level_through_ftol(e, this, level, |e| {
        u32::from(c_speed_tree_rt_get_num_branch_lod_levels(e, this))
    })
}

// Translated from 00b085a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::GetDiscreteFrondLodLevel` (Xbox PDB): see [`discrete_level_through_ftol`]
/// with the frond level count (`+0x64`).
pub fn c_speed_tree_rt_get_discrete_frond_lod_level(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    level: f32,
) -> u16 {
    discrete_level_through_ftol(e, this, level, |e| u32::from(fn_00b08730(e, this)))
}

/// `FISTP dword` with the rounding control forced to truncate (`FNSTCW`, `OR 0xC00`,
/// `FLDCW`, `FISTP`, `FLDCW`): the value cut towards zero; a NaN or an out-of-range value gives
/// the integer indefinite `0x80000000`.
fn fistp_truncate(value: f64) -> i32 {
    if value.is_nan() || value >= 2147483648.0 || value <= -2147483649.0 {
        i32::MIN
    } else {
        value as i32
    }
}

// Translated from 00b08610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The discrete leaf level of a level-of-detail float (unnamed in the engine map; `-1` means the
/// tree's own level): `(1 - level) * n`, truncated by an inline `FISTP`, where `n` is the leaf
/// level count from `00b08710` (plus one while the byte at `011f8b78` is set); a result equal to
/// `n` is pulled back by one.
pub fn fn_00b08610(e: &mut Engine, this: Ptr<CSpeedTreeRT>, level: f32) -> u16 {
    let mut level = level.to_bits();
    let sentinel: f64 = e.global(0x0101_a6b0);
    if x87_load32(level) == sentinel {
        level = c_speed_tree_rt_get_lod_level(e, this).to_bits();
    }
    let count = if e.global::<u8>(DISCRETE_LEAF_LOD_FLAG) != 0 {
        u32::from(fn_00b08710(e, this)) + 1
    } else {
        u32::from(fn_00b08710(e, this))
    };
    let scaled = x87_mul(x87_sub(1.0, x87_load32(level)), f64::from(count as i32));
    let mut discrete = fistp_truncate(scaled) as u16;
    if u32::from(discrete) == count {
        discrete = discrete.wrapping_sub(1);
    }
    discrete
}

// Translated from 00b086f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::GetNumBranchLodLevels` (Xbox PDB): the low word of the engine's `+0x78`.
pub fn c_speed_tree_rt_get_num_branch_lod_levels(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> u16 {
    let engine = e.mem.u32(this.addr());
    e.mem.u32(engine + 0x78) as u16
}

// Translated from 00b08710 (decompiled, FalloutNV.exe 1.4.0.525)
/// The low word of the engine's `+0xD8` (the leaf level count; unnamed in the engine map).
pub fn fn_00b08710(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> u16 {
    let engine = e.mem.u32(this.addr());
    e.mem.u32(engine + 0xD8) as u16
}

// Translated from 00b08730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `m_usNumFrondLodLevels` (Xbox PDB, `+0x64`) of the tree (unnamed in the engine map).
pub fn fn_00b08730(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> u16 {
    e.get(this, CSpeedTreeRT::m_usNumFrondLodLevels)
}

// Translated from 00b08750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears a vector of the branch geometry when the tree has one (unnamed in the engine map):
/// `00ac2f70` on the geometry's `+0x70` (`m_vColors` in the Xbox PDB).
pub fn fn_00b08750(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    let geometry = e.get(this, CSpeedTreeRT::m_pBranchGeometry);
    if !geometry.is_null() {
        e.call(0x00ac_2f70, &args![geometry.addr() + 0x70]);
    }
}

// Translated from 00b08780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the wind vectors of the branch geometry when the tree has one (unnamed in the engine
/// map): `0044a0a0` on its `+0x160` (`m_vWindWeights` in the Xbox PDB), then the clear of
/// `00b0dc10` on its `+0x178` (`m_vWindMatrixIndices`).
pub fn fn_00b08780(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    let geometry = e.get(this, CSpeedTreeRT::m_pBranchGeometry);
    if !geometry.is_null() {
        e.call(0x0044_a0a0, &args![geometry.addr() + 0x160]);
        fn_00b0dc10(e, Ptr::new(geometry.addr() + 0x178));
    }
}

// Translated from 00b087d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the transient parts of a computed tree (unnamed in the engine map): the projected
/// shadow (`+0x50`: its self-shadow map string at `+0x24` is tidied away, the block is freed
/// and the pointer cleared) and the texture-coordinate vector (`0044a0a0` on `+0x148`) of the
/// branch geometry and of the frond geometry (`+0x60`) when there are any. The exe's delete
/// flag is the compile-time constant 1, so the shadow's block is always freed.
pub fn fn_00b087d0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    let shadow = e.get(this, CSpeedTreeRT::m_pProjectedShadow);
    if !shadow.is_null() {
        e.call(STRING_TIDY, &args![shadow.addr() + 0x24, 1u32, 0u32]);
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, shadow]);
        e.set(this, CSpeedTreeRT::m_pProjectedShadow, Ptr::NULL);
    }
    let geometry = e.get(this, CSpeedTreeRT::m_pBranchGeometry);
    if !geometry.is_null() {
        e.call(0x0044_a0a0, &args![geometry.addr() + 0x148]);
    }
    let geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
    if !geometry.is_null() {
        e.call(0x0044_a0a0, &args![geometry.addr() + 0x148]);
    }
}

// Translated from 00b088b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `00b28ed0` on the leaf geometry when the tree has one (unnamed in the engine map).
pub fn fn_00b088b0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    let geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
    if !geometry.is_null() {
        e.call(0x00b2_8ed0, &args![geometry]);
    }
}

// Translated from 00b088d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b088d0 - the FS:[0] exception frame is not translated.
/// Deletes the branch geometry of a computed tree that has one, has no instance data and is the
/// only user of its shared count (unnamed in the engine map): the geometry's destructor
/// (`00b2e9c0`), its block freed (the exe's delete flag is the constant 1), the tree's pointer
/// cleared and the engine's branch-geometry pointer (`+0x5C`) cleared.
pub fn fn_00b088d0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) != 0
        && !e.get(this, CSpeedTreeRT::m_pBranchGeometry).is_null()
        && e.get(this, CSpeedTreeRT::m_pInstanceData).is_null()
    {
        let count = e.get(this, CSpeedTreeRT::m_pInstanceRefCount);
        if e.mem.u32(count.addr()) == 1 {
            let geometry = e.get(this, CSpeedTreeRT::m_pBranchGeometry);
            if !geometry.is_null() {
                e.call(0x00b2_e9c0, &args![geometry]);
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, geometry]);
            }
            e.set(this, CSpeedTreeRT::m_pBranchGeometry, Ptr::NULL);
            let engine = e.get(this, CSpeedTreeRT::m_pEngine);
            if !engine.is_null() {
                e.mem.set_u32(engine.addr() + 0x5C, 0);
            }
        }
    }
}

// Translated from 00b08af0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b08af0 - the FS:[0] exception frame is not translated.
/// Hands its argument to the leaf geometry's `00b27870` and returns the result (unnamed in
/// the engine map).
pub fn fn_00b08af0(e: &mut Engine, this: Ptr<CSpeedTreeRT>, argument: u32) -> u32 {
    let geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
    e.call(0x00b2_7870, &args![geometry, argument]).u32()
}

// Translated from 00b08c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::GetBoundingBox` (Xbox PDB): copies the 0x18 bytes of the tree sizes
/// (`+0x40`: the box's six floats) to `box_out`; a null address reports through `SetError`.
pub fn c_speed_tree_rt_get_bounding_box(e: &mut Engine, this: Ptr<CSpeedTreeRT>, box_out: u32) {
    if box_out != 0 {
        let sizes = e.get(this, CSpeedTreeRT::m_pTreeSizes);
        e.call(MEMCPY, &args![box_out, sizes, 0x18u32]);
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_5118);
    }
}

// Translated from 00b08cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b08cc0 - the FS:[0] exception frame is not translated.
/// The static lighting of the leaves (unnamed in the engine map): takes the bounding box,
/// forms its centre (`(max + min) * [01011588]` per axis, the double), and runs the lighting
/// engine's `ComputeLeafStaticLighting` (`00b2a210`) with the centre, the engine's word at `+0xEC`
/// and the leaf level count (`00b08710`).
pub fn fn_00b08cc0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    e.with_stack(0x24, |e, frame| {
        let bounds = frame.addr();
        let centre = frame.addr() + 0x18;
        c_speed_tree_rt_get_bounding_box(e, this, bounds);
        let half: f64 = e.global(0x0101_1588);
        let mut sums = [0u32; 3];
        for (axis, slot) in sums.iter_mut().enumerate() {
            let axis = 4 * axis as u32;
            let sum = x87_add(fld(e, bounds + 0xC + axis), operand(e, bounds + axis));
            *slot = x87_store32(x87_mul(sum, half));
        }
        for (axis, bits) in sums.into_iter().enumerate() {
            e.mem
                .set_u32(centre + 4 * axis as u32, x87_quiet_bits(bits));
        }
        let engine = e.mem.u32(this.addr());
        let large_percent = e.mem.u32(engine + 0xEC);
        let levels = fn_00b08710(e, this);
        let lighting = e.get(this, CSpeedTreeRT::m_pLightingEngine);
        e.call(0x00b2_a210, &args![lighting, centre, large_percent, levels]);
    });
}

// Translated from 00b08ec0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b08ec0 - the FS:[0] exception frame (leaving the critical section
// if a step throws) is not translated.
/// A static function (`cdecl`, one word; unnamed in the engine map) over every tree in the
/// shared list of unique trees (the vector at `011f8bdc`, under the critical section at
/// `011f8bc4`, entered and left through `EnterCriticalSection` and `LeaveCriticalSection`):
/// with 0, a tree whose branch wind method is 1 gets the branch geometry invalidated (`+0x12`),
/// one whose frond wind method is 1 the frond geometry, and the leaf geometry's `00b27820`
/// runs unless the leaf wind method is not 1 and the rocking-leaves flag is clear; with 1,
/// `00b27820` runs on the leaf geometry and the simple billboard's valid byte (`+0x30`) is
/// cleared; any other value does nothing.
pub fn fn_00b08ec0(e: &mut Engine, mode: i32) {
    e.call(ENTER_CRITICAL_SECTION, &args![UNIQUE_TREES_LOCK]);
    e.with_stack(0x18, |e, frame| {
        let begin = Ptr::new(frame.addr());
        let end = frame.addr() + 8;
        let first = e.global::<u32>(UNIQUE_TREES + 0x0C);
        e.call(ITERATOR_CONSTRUCT, &args![begin, first, UNIQUE_TREES]);
        loop {
            let last = e.global::<u32>(UNIQUE_TREES + 0x10);
            e.call(ITERATOR_CONSTRUCT, &args![end, last, UNIQUE_TREES]);
            if e.call(ITERATOR_EQUAL, &args![begin, end]).bool() {
                break;
            }
            if mode == 0 {
                let slot = fn_00b0ebb0(e, begin);
                let tree = Ptr::<CSpeedTreeRT>::new(e.mem.u32(slot));
                if fn_00b076c0(e, tree) == 1 {
                    let slot = fn_00b0ebb0(e, begin);
                    let tree = e.mem.u32(slot);
                    let geometry = e.mem.u32(tree + 4);
                    e.mem.set_u8(geometry + 0x12, 0);
                }
                let slot = fn_00b0ebb0(e, begin);
                let tree = Ptr::<CSpeedTreeRT>::new(e.mem.u32(slot));
                if fn_00b078e0(e, tree) == 1 {
                    let slot = fn_00b0ebb0(e, begin);
                    let tree = e.mem.u32(slot);
                    let geometry = e.mem.u32(tree + 0x60);
                    e.mem.set_u8(geometry + 0x12, 0);
                }
                let slot = fn_00b0ebb0(e, begin);
                let tree = Ptr::<CSpeedTreeRT>::new(e.mem.u32(slot));
                let mut run = fn_00b074f0(e, tree) == 1;
                if !run {
                    let slot = fn_00b0ebb0(e, begin);
                    let tree = e.mem.u32(slot);
                    let wind = e.mem.u32(tree + 0x10);
                    run = e.mem.u8(wind + 0x14) != 0;
                }
                if run {
                    let slot = fn_00b0ebb0(e, begin);
                    let tree = e.mem.u32(slot);
                    let leaf_geometry = e.mem.u32(tree + 8);
                    e.call(0x00b2_7820, &args![leaf_geometry]);
                }
            } else if mode == 1 {
                let slot = fn_00b0ebb0(e, begin);
                let tree = e.mem.u32(slot);
                let leaf_geometry = e.mem.u32(tree + 8);
                e.call(0x00b2_7820, &args![leaf_geometry]);
                let slot = fn_00b0ebb0(e, begin);
                let tree = e.mem.u32(slot);
                let billboard = e.mem.u32(tree + 0x14);
                e.mem.set_u8(billboard + 0x30, 0);
            }
            fn_00b0ec20(e, begin);
        }
    });
    e.call(LEAVE_CRITICAL_SECTION, &args![UNIQUE_TREES_LOCK]);
}

// Translated from 00b09270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::SetError` (Xbox PDB), `cdecl`: stores the message, a C string, in the static
/// error string at `011f8ba4` (`std::string::assign(char *, length)`, `0044a7f0`, with the
/// length from `strlen`, `00ec6130`).
pub fn c_speed_tree_rt_set_error(e: &mut Engine, message: u32) {
    let length = e.call(STRLEN, &args![message]).u32();
    e.call(STRING_ASSIGN_TEXT, &args![ERROR_STRING, message, length]);
}

/// `std::string::string(const char *)` (`0044a4c0`, `this` = the string, the text),
/// `std::string::assign(const string &, pos, count)` (`0044a580`), the string append helpers
/// `operator+(string, const char *)` (`00ad4210`: the result string, the left string, the text)
/// and `operator+(string, string)` (`00ad42e0`: the result, the left, the right), the CRT's
/// `strcpy` (`00ec6370`), `strlen` (`00ec6130`), the `std::exception` constructor
/// (`00ec78e0`), destructor (`00ec79a1`) and copy constructor (`00aca270`), and the throw info
/// of the parse errors (`01170384`).
const STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
const STRING_ASSIGN: u32 = 0x0044_a580;
/// `std::string::assign(const char *, count)` (`0044a7f0`, `this` = the string, the text, the count).
const STRING_ASSIGN_TEXT: u32 = 0x0044_a7f0;
const STRING_PLUS_TEXT: u32 = 0x00ad_4210;
const STRING_PLUS_STRING: u32 = 0x00ad_42e0;
const STRCPY: u32 = 0x00ec_6370;
const STRLEN: u32 = 0x00ec_6130;
const EXCEPTION_DEFAULT_CONSTRUCT: u32 = 0x00ec_78e0;
const EXCEPTION_DESTRUCT: u32 = 0x00ec_79a1;
const EXCEPTION_COPY_CONSTRUCT: u32 = 0x00ac_a270;
const PARSE_THROW_INFO: u32 = 0x0117_0384;
/// The `std::string` npos word the assigns take as their count (`010173f0`).
const STRING_NPOS: u32 = 0x0101_73f0;
/// `CTreeFileAccess::ParseInt` (`00b3ed80`), `ParseFloat` (`00b3ed20`, the float in `ST0`) and
/// `ParseString` (`00b3ee40`: the file access, the string to fill).
const PARSE_INT: u32 = 0x00b3_ed80;
pub(crate) const PARSE_FLOAT: u32 = 0x00b3_ed20;
const PARSE_STRING: u32 = 0x00b3_ee40;

/// The next integer of the tree file.
pub(crate) fn parse_int(e: &mut Engine, access: Ptr) -> i32 {
    e.call(PARSE_INT, &args![access]).i32()
}

/// Whether the tree file's index has reached the end of its byte vector (the vector's end
/// minus start, compared unsigned).
pub(crate) fn file_ended(e: &Engine, access: Ptr) -> bool {
    let vector = access.addr() + 4;
    let length = e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C));
    e.mem.u32(access.addr()) >= length
}

/// Throws the parse error `message` (a C string address): the text in a `std::string`, the
/// error object (`00b09480`, errno not used) built from it, and `_CxxThrowException` with the
/// parse-error throw info.
pub(crate) fn throw_parse_error(e: &mut Engine, message: u32) -> ! {
    e.with_stack(0x1C, |e, text| {
        e.call(STRING_FROM_C_STRING, &args![text, message]);
        e.with_stack(0x28, |e, error| {
            fn_00b09480(e, error, text, 0);
            e.call(CXX_THROW_EXCEPTION, &args![error, PARSE_THROW_INFO]);
        });
    });
    unreachable!("_CxxThrowException does not return (00ec7894)")
}

// Translated from 00b092a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b092a0 - the FS:[0] exception frame (destroying the message
// string when a parse error is thrown) is not translated.
/// `CSpeedTreeRT::ParseLodInfo` (Xbox PDB): reads tagged entries of the leaf-lod chunk until
/// the tag 9001: 9002 the leaf lod method (`+0x18`, an int), 9003, 9004 and 9009 the floats
/// at `+0x1C`, `+0x20` and `+0x24`, 9005 the tree engine's own parser (`00b2ceb0`); any other tag
/// throws "malformed lod info", and running out of data throws "premature end of file".
pub fn c_speed_tree_rt_parse_lod_info(e: &mut Engine, this: Ptr<CSpeedTreeRT>, access: Ptr) {
    let mut tag = parse_int(e, access);
    loop {
        match tag.wrapping_sub(0x232A) as u32 {
            0 => {
                let method = e.call(PARSE_INT, &args![access]).u32();
                e.set(this, CSpeedTreeRT::m_eLeafLodMethod, method);
            }
            1 => {
                let value = e.call(PARSE_FLOAT, &args![access]).f64();
                fstp(e, this.addr() + 0x1C, value);
            }
            2 => {
                let value = e.call(PARSE_FLOAT, &args![access]).f64();
                fstp(e, this.addr() + 0x20, value);
            }
            7 => {
                let value = e.call(PARSE_FLOAT, &args![access]).f64();
                fstp(e, this.addr() + 0x24, value);
            }
            3 => {
                let engine = e.get(this, CSpeedTreeRT::m_pEngine);
                e.call(0x00b2_ceb0, &args![engine, access]);
            }
            _ => throw_parse_error(e, 0x010a_51c8),
        }
        if file_ended(e, access) {
            throw_parse_error(e, 0x010a_5194);
        }
        tag = parse_int(e, access);
        if tag == 0x2329 {
            break;
        }
    }
}

// Translated from 00b09480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b09480 - the FS:[0] exception frame (destroying the temporary
// strings when a step throws) is not translated.
/// The parse-error exception constructor (unnamed in the engine map; `this` is 0x28 bytes: the
/// `std::exception` base and a `std::string` at `+0xC`): the message is `message + " [" + reason +
/// "]"`, the reason `strerror(errno)` when `use_errno` is set, else the empty string (`01011584`);
/// it is handed to the base constructor `00b096a0`, the temporaries are destroyed and the
/// vtable `010a51e0` is stored. Returns `this`.
pub fn fn_00b09480(e: &mut Engine, this: Ptr, message: Ptr, use_errno: u32) -> Ptr {
    // Five `std::string` temporaries (0x1C bytes each): the errno text, the empty text and
    // three concatenation results.
    e.with_stack(0x8C, |e, frame| {
        let base = frame.addr();
        let (errno_text, empty_text) = (base, base + 0x1C);
        let (first, second, third) = (base + 0x38, base + 0x54, base + 0x70);
        let used_errno = (use_errno & 0xFF) != 0;
        let reason = if used_errno {
            let text = last_error_text(e);
            e.call(STRING_FROM_C_STRING, &args![errno_text, text]);
            errno_text
        } else {
            e.call(STRING_FROM_C_STRING, &args![empty_text, 0x0101_1584u32]);
            empty_text
        };
        let left = e
            .call(STRING_PLUS_TEXT, &args![first, message, 0x010a_51e8u32])
            .u32();
        let middle = e
            .call(STRING_PLUS_STRING, &args![second, left, reason])
            .u32();
        let whole = e
            .call(STRING_PLUS_TEXT, &args![third, middle, 0x0106_3e18u32])
            .u32();
        fn_00b096a0(e, this, Ptr::new(whole));
        e.call(STRING_TIDY, &args![third, 1u32, 0u32]);
        e.call(STRING_TIDY, &args![second, 1u32, 0u32]);
        e.call(STRING_TIDY, &args![first, 1u32, 0u32]);
        // The exe keeps a bit per temporary that was built (2: the empty text, 1: the errno text).
        if !used_errno {
            e.call(STRING_TIDY, &args![empty_text, 1u32, 0u32]);
        }
        if used_errno {
            e.call(STRING_TIDY, &args![errno_text, 1u32, 0u32]);
        }
    });
    e.mem.set_u32(this.addr(), 0x010a_51e0);
    this
}

// Translated from 00b096a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b096a0 - the FS:[0] exception frame is not translated.
/// The base constructor of the parse-error exception (unnamed in the engine map): the
/// `std::exception` constructor (`00ec78e0`), the vtable `010a307c`, the message string at
/// `+0xC` built empty and assigned from `message`. Returns `this`.
pub fn fn_00b096a0(e: &mut Engine, this: Ptr, message: Ptr) -> Ptr {
    e.call(EXCEPTION_DEFAULT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), 0x010a_307c);
    let text = this.addr() + 0xC;
    e.call(STRING_TIDY, &args![text, 0u32, 0u32]);
    let npos = e.global::<u32>(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![text, message, 0u32, npos]);
    this
}

// Translated from 00b09730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of the parse-error base (unnamed in the engine map): the
/// vtable `010a307c`, the message string destroyed, `std::exception`'s destructor (`00ec79a1`),
/// and the block freed when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00b09730(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x010a_307c);
    e.call(STRING_TIDY, &args![this.addr() + 0xC, 1u32, 0u32]);
    e.call(EXCEPTION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b09780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the parse-error base without the delete (unnamed in the engine map): as
/// [`fn_00b09730`] with the flag clear.
pub fn fn_00b09780(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010a_307c);
    e.call(STRING_TIDY, &args![this.addr() + 0xC, 1u32, 0u32]);
    e.call(EXCEPTION_DESTRUCT, &args![this]);
}

// Translated from 00b097b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy constructor of the parse-error exception (unnamed in the engine map): the
/// base's copy constructor (`00aca270`) and the vtable `010a51e0`. Returns `this`.
pub fn fn_00b097b0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(EXCEPTION_COPY_CONSTRUCT, &args![this, other]);
    e.mem.set_u32(this.addr(), 0x010a_51e0);
    this
}

// Translated from 00b097e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b097e0 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::ParseWindInfo` (Xbox PDB): reads tagged entries of the wind chunk until the
/// tag 11001: 11002 the engine's wind level (`+0x108`, an int); any other tag throws "malformed
/// wind info", and running out of data throws "premature end of file".
pub fn c_speed_tree_rt_parse_wind_info(e: &mut Engine, this: Ptr<CSpeedTreeRT>, access: Ptr) {
    let mut tag = parse_int(e, access);
    loop {
        if tag == 0x2AFA {
            let level = e.call(PARSE_INT, &args![access]).u32();
            let engine = e.get(this, CSpeedTreeRT::m_pEngine);
            e.mem.set_u32(engine.addr() + 0x108, level);
        } else {
            throw_parse_error(e, 0x010a_5220);
        }
        if file_ended(e, access) {
            throw_parse_error(e, 0x010a_51ec);
        }
        tag = parse_int(e, access);
        if tag == 0x2AF9 {
            break;
        }
    }
}

layout! {
    /// `SEmbeddedTexCoords` (Xbox PDB), 0x54 bytes; the offsets agree with this unit's code
    /// (the string at `+0x18` is the same size on the PC).
    pub struct SEmbeddedTexCoords: 0x54 {
        /// `m_nNumLeafMaps` (Xbox PDB).
        0x00 m_nNumLeafMaps: i32,
        /// `m_pLeafTexCoords` (Xbox PDB).
        0x04 m_pLeafTexCoords: Ptr,
        /// `m_nNumBillboards` (Xbox PDB).
        0x08 m_nNumBillboards: i32,
        /// `m_pBillboardTexCoords` (Xbox PDB).
        0x0C m_pBillboardTexCoords: Ptr,
        /// `m_nNumFrondMaps` (Xbox PDB).
        0x10 m_nNumFrondMaps: i32,
        /// `m_pFrondTexCoords` (Xbox PDB).
        0x14 m_pFrondTexCoords: Ptr,
        /// `m_strCompositeFilename` (Xbox PDB): a `std::string`.
        0x18 m_strCompositeFilename: Inline<()>,
        /// `m_afShadowTexCoords` (Xbox PDB): eight floats.
        0x34 m_afShadowTexCoords: f32,
    }
}

/// `new float[count * 8]` of an embedded-coordinates array, then `count * 8` floats read from
/// the file into it, `negate_odd` negating the odd ones when the byte at `011f8b79` is set.
/// `count_offset` is the offset of the record's count; its array pointer follows it.
fn parse_embedded_floats(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    access: Ptr,
    count_offset: u32,
    negate_odd: bool,
) {
    let record = |e: &Engine| e.mem.u32(this.addr() + 0x4C);
    let count = e.call(PARSE_INT, &args![access]).i32();
    let coords = record(e);
    e.mem.set_i32(coords + count_offset, count);
    if e.mem.i32(record(e) + count_offset) > 0 {
        let entries = e.mem.i32(record(e) + count_offset);
        let size = ((entries as u32) << 3).checked_mul(4).unwrap_or(u32::MAX);
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        let coords = record(e);
        e.mem.set_u32(coords + count_offset + 4, block);
        let mut row = 0i32;
        while row < e.mem.i32(record(e) + count_offset) {
            let mut column = 0i32;
            while column < 8 {
                let value = e.call(PARSE_FLOAT, &args![access]).f64();
                let array = e.mem.u32(record(e) + count_offset + 4);
                let slot = array.wrapping_add(((column + row * 8) as u32) << 2);
                fstp(e, slot, value);
                if negate_odd && (column & 1) != 0 && e.global::<u8>(FROND_TEXTURE_FLAG) != 0 {
                    let array = e.mem.u32(record(e) + count_offset + 4);
                    let slot = array.wrapping_add(((column + row * 8) as u32) << 2);
                    let flipped = -fld(e, slot);
                    fstp(e, slot, flipped);
                }
                column += 1;
            }
            row += 1;
        }
    }
}

// Translated from 00b09960 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b09960 - the FS:[0] exception frame and the /GS cookie check
// (00ec408c) are not translated.
/// Reads the embedded texture coordinates chunk (unnamed in the engine map): makes the
/// `SEmbeddedTexCoords` (0x54 bytes, `00b05130`) at `+0x4C`, then reads tagged entries until
/// the tag 10001: 10002 the leaf maps (a count and eight floats each), 10003 the billboards
/// (the same, the odd floats negated when the byte at `011f8b79` is set), 10004 the frond
/// maps, 10005 the composite file name (a string reduced to its file name by `NoPath`),
/// 10006 and 10007 the bytes at `+0x6D` and `+0x6C`; any other tag throws "malformed ..."
/// and running out of data throws "premature end of file".
pub fn fn_00b09960(e: &mut Engine, this: Ptr<CSpeedTreeRT>, access: Ptr) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x54u32]).u32();
    let record = if block != 0 {
        fn_00b05130(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.set(this, CSpeedTreeRT::m_pEmbeddedTexCoords, Ptr::new(record));
    let mut tag = parse_int(e, access);
    loop {
        match tag.wrapping_sub(0x2712) as u32 {
            0 => parse_embedded_floats(e, this, access, 0x00, false),
            1 => parse_embedded_floats(e, this, access, 0x08, true),
            2 => parse_embedded_floats(e, this, access, 0x10, false),
            3 => {
                // The by-value string argument, the file name and its reduced form.
                e.with_stack(0x54, |e, frame| {
                    let (argument, name, reduced) =
                        (frame.addr(), frame.addr() + 0x1C, frame.addr() + 0x38);
                    e.call(PARSE_STRING, &args![access, argument]);
                    let words: [u32; 7] =
                        std::array::from_fn(|i| e.mem.u32(argument + 4 * i as u32));
                    fn_00b09ef0(
                        e,
                        Ptr::new(name),
                        words[0],
                        words[1],
                        words[2],
                        words[3],
                        words[4],
                        words[5],
                        words[6],
                    );
                    let reduced = c_idv_filename_no_path(e, Ptr::new(name), Ptr::new(reduced));
                    let target = e.get(this, CSpeedTreeRT::m_pEmbeddedTexCoords).addr() + 0x18;
                    let npos = e.global::<u32>(STRING_NPOS);
                    e.call(STRING_ASSIGN, &args![target, reduced, 0u32, npos]);
                    e.call(STRING_TIDY, &args![reduced, 1u32, 0u32]);
                    e.call(STRING_TIDY, &args![name, 1u32, 0u32]);
                });
            }
            4 => {
                let flag = fn_00b06180(e, access);
                e.mem.set_u8(this.addr() + 0x6D, flag as u8);
            }
            5 => {
                let flag = fn_00b06180(e, access);
                e.mem.set_u8(this.addr() + 0x6C, flag as u8);
            }
            _ => throw_parse_error(e, 0x010a_5278),
        }
        if file_ended(e, access) {
            throw_parse_error(e, 0x010a_5238);
        }
        tag = parse_int(e, access);
        if tag == 0x2711 {
            break;
        }
    }
}

// Translated from 00b09ef0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b09ef0 - the FS:[0] exception frame is not translated.
/// The `std::string` constructor from a string passed by value (unnamed in the engine map):
/// `this` is built empty (`0044a730`), assigned the whole of the by-value argument (seven
/// words) and the argument destroyed. Returns `this`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00b09ef0(
    e: &mut Engine,
    this: Ptr,
    word0: u32,
    word1: u32,
    word2: u32,
    word3: u32,
    word4: u32,
    word5: u32,
    word6: u32,
) -> Ptr {
    e.call(STRING_TIDY, &args![this, 0u32, 0u32]);
    e.with_stack(0x1C, |e, argument| {
        for (i, word) in [word0, word1, word2, word3, word4, word5, word6]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(argument.addr() + 4 * i as u32, word);
        }
        let npos = e.global::<u32>(STRING_NPOS);
        e.call(STRING_ASSIGN, &args![this, argument, 0u32, npos]);
        e.call(STRING_TIDY, &args![argument, 1u32, 0u32]);
    });
    this
}

// Translated from 00b09f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `std::string` destructor (unnamed in the engine map): `00b09f70` tidies `this` away
/// (`0044a730`, free the buffer, size 0).
pub fn fn_00b09f70(e: &mut Engine, this: Ptr) {
    e.call(STRING_TIDY, &args![this, 1u32, 0u32]);
}

/// The text of a `std::string`: its inline buffer (`+4`) or, from a reserve of 0x10 on, the
/// heap pointer there.
fn string_text(e: &Engine, string: u32) -> u32 {
    if e.mem.u32(string + 0x18) >= 0x10 {
        e.mem.u32(string + 4)
    } else {
        string + 4
    }
}

/// `string[index]`'s range test: an index above the size takes the invalid-parameter path.
fn check_string_index(e: &mut Engine, string: u32, index: u32) {
    if index > e.mem.u32(string + 0x14) {
        e.call(INVALID_PARAMETER, &args![]);
    }
}

// Translated from 00b09f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b09f90 - the FS:[0] exception frame and the /GS cookie check
// (00ec408c) are not translated.
/// `CIdvFilename::NoPath` (Xbox PDB): copies the file name `this` (a `std::string`; the size is
/// at `+0x14`) to a 0x400-byte buffer (`strcpy`, `00ec6370`), scans back from its last
/// character for a `/` or `\` and, when one is found, moves the text after it to the start of
/// the buffer; the buffer's text is built into `result` (a fresh string, `0044a4c0`,
/// `0044a730`, `0044a580`). Returns `result`.
pub fn c_idv_filename_no_path(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    let name = this.addr();
    e.with_stack(0x400, |e, buffer| {
        let buffer = buffer.addr();
        let text = string_text(e, name);
        e.call(STRCPY, &args![buffer, text]);
        let size = e.mem.u32(name + 0x14) as i32;
        let mut index = size.wrapping_sub(1);
        while index >= 0 {
            check_string_index(e, name, index as u32);
            let character = e.mem.i8(string_text(e, name).wrapping_add(index as u32));
            let mut found = character == 0x2F;
            if !found {
                check_string_index(e, name, index as u32);
                let character = e.mem.i8(string_text(e, name).wrapping_add(index as u32));
                found = character == 0x5C;
            }
            if found {
                let mut from = index + 1;
                let mut to = 0u32;
                while from < size {
                    check_string_index(e, name, from as u32);
                    let byte = e.mem.u8(string_text(e, name).wrapping_add(from as u32));
                    e.mem.set_u8(buffer + to, byte);
                    from += 1;
                    to += 1;
                }
                e.mem.set_u8(buffer + to, 0);
                break;
            }
            index -= 1;
        }
        e.with_stack(0x1C, |e, text| {
            e.call(STRING_FROM_C_STRING, &args![text, buffer]);
            e.call(STRING_TIDY, &args![result, 0u32, 0u32]);
            let npos = e.global::<u32>(STRING_NPOS);
            e.call(STRING_ASSIGN, &args![result, text, 0u32, npos]);
            e.call(STRING_TIDY, &args![text, 1u32, 0u32]);
        });
    });
    result
}

// Translated from 00b0a260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of collision objects of the tree (unnamed in the engine map): the element count
/// (0x1C bytes each) of the vector of `+0x58`, or 0 without collision objects.
pub fn fn_00b0a260(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> i32 {
    let objects = e.get(this, CSpeedTreeRT::m_pCollisionObjects);
    if objects.is_null() {
        0
    } else {
        let bytes = e
            .mem
            .u32(objects.addr() + 0x10)
            .wrapping_sub(e.mem.u32(objects.addr() + 0x0C)) as i32;
        bytes / 0x1C
    }
}

// Translated from 00b0a2a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0a2a0 - the FS:[0] exception frame (destroying the message string
// if `SetError` throws) is not translated.
/// `CSpeedTreeRT::GetCollisionObject` (Xbox PDB): with collision objects, an index below their
/// count copies the shape's type word to `type_out`, its three position floats (`+4`) to
/// `position_out` and its three dimension floats (`+0x10`) to `dimensions_out` (each access
/// through the checked index); an index at or beyond the count is reported through `SetError`
/// with the message formatted from the index and the count; without collision objects the
/// error is a fixed message.
pub fn c_speed_tree_rt_get_collision_object(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    index: u32,
    type_out: u32,
    position_out: u32,
    dimensions_out: u32,
) {
    let objects = e.get(this, CSpeedTreeRT::m_pCollisionObjects);
    if objects.is_null() {
        c_speed_tree_rt_set_error(e, 0x010a_5298);
        return;
    }
    let vector = objects.addr();
    if index >= vector_len(e, vector, 0x1C) {
        let count = vector_len(e, vector, 0x1C);
        e.with_stack(0x1C, |e, text| {
            e.call(FORMAT_STRING, &args![text, 0x010a_52c8u32, index, count]);
            let characters = string_text(e, text.addr());
            c_speed_tree_rt_set_error(e, characters);
            e.call(STRING_TIDY, &args![text, 1u32, 0u32]);
        });
        return;
    }
    let shape = vector_at(e, vector, index, 0x1C);
    let word = e.mem.u32(shape);
    e.mem.set_u32(type_out, word);
    let shape = vector_at(e, vector, index, 0x1C);
    e.call(MEMCPY, &args![position_out, shape + 4, 0xCu32]);
    let shape = vector_at(e, vector, index, 0x1C);
    e.call(MEMCPY, &args![dimensions_out, shape + 0x10, 0xCu32]);
}

// Translated from 00b0a490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0a490 - the FS:[0] exception frame (destroying the message
// string when a parse error is thrown) is not translated.
/// `CSpeedTreeRT::ParseCollisionObjects` (Xbox PDB): makes the `SCollisionObjects` (0x18 bytes,
/// built by `00b0dfb0`) at `+0x58`, then reads shapes until the tag 12001: each starts with a
/// tag (12002, 12003, 12004: the shape type 0, 1, 2; anything else throws "malformed texture
/// coord info"), three position floats and, by type, one, two or three dimension floats, and
/// is appended to the objects (`00b0e090`); running out of data throws "premature end of
/// file".
pub fn c_speed_tree_rt_parse_collision_objects(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    access: Ptr,
) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x18u32]).u32();
    let objects = if block != 0 {
        fn_00b0dfb0(e, Ptr::new(block));
        block
    } else {
        0
    };
    e.set(this, CSpeedTreeRT::m_pCollisionObjects, Ptr::new(objects));
    let mut tag = parse_int(e, access);
    loop {
        // The `SShape` (0x1C bytes) with the three floats read for its position behind it.
        e.with_stack(0x24, |e, frame| {
            let shape = frame.addr();
            let temporaries = shape + 0x1C;
            let kind = match tag {
                0x2EE2 => 0,
                0x2EE3 => 1,
                0x2EE4 => 2,
                _ => throw_parse_error(e, 0x010a_535c),
            };
            for axis in 0..3 {
                let value = e.call(PARSE_FLOAT, &args![access]).f64();
                fstp(e, temporaries + 4 * axis, value);
            }
            for axis in 0..3 {
                copy_float(e, shape + 4 + 4 * axis, temporaries + 4 * axis);
            }
            let dimensions = match kind {
                0 => 1,
                1 => 2,
                2 => 3,
                _ => throw_parse_error(e, 0x010a_533c),
            };
            e.mem.set_u32(shape, kind);
            for axis in 0..dimensions {
                let value = e.call(PARSE_FLOAT, &args![access]).f64();
                fstp(e, shape + 0x10 + 4 * axis, value);
            }
            let objects = e.get(this, CSpeedTreeRT::m_pCollisionObjects);
            fn_00b0e090(e, objects, frame);
        });
        if file_ended(e, access) {
            throw_parse_error(e, 0x010a_5300);
        }
        tag = parse_int(e, access);
        if tag == 0x2EE1 {
            break;
        }
    }
}

// Translated from 00b0a7a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0a7a0 - the FS:[0] exception frame is not translated.
/// Deletes the frond geometry of a computed tree that has one, has no instance data and is the
/// only user of its shared count (unnamed in the engine map): the geometry's destructor
/// (`00b2e9c0`), its block freed (the exe's delete flag is the constant 1) and the tree's
/// pointer (`+0x60`) cleared.
pub fn fn_00b0a7a0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    if e.get(this, CSpeedTreeRT::m_bTreeComputed) != 0
        && !e.get(this, CSpeedTreeRT::m_pFrondGeometry).is_null()
        && e.get(this, CSpeedTreeRT::m_pInstanceData).is_null()
        && !e.get(this, CSpeedTreeRT::m_pInstanceRefCount).is_null()
    {
        let count = e.get(this, CSpeedTreeRT::m_pInstanceRefCount);
        if e.mem.u32(count.addr()) == 1 {
            let geometry = e.get(this, CSpeedTreeRT::m_pFrondGeometry);
            if !geometry.is_null() {
                e.call(0x00b2_e9c0, &args![geometry]);
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, geometry]);
            }
            e.set(this, CSpeedTreeRT::m_pFrondGeometry, Ptr::NULL);
        }
    }
}

// Translated from 00b0a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of strips of a detail level of an indexed geometry (`this`; unnamed in the
/// engine map): 0 for a negative level, else the pointer count of the level's entry of the
/// strip vector (`+0x58`, 0x18-byte entries; the checked index), the entry's byte size shifted
/// right by 2.
pub fn fn_00b0a9a0(e: &mut Engine, this: Ptr, level: i16) -> u16 {
    if i32::from(level) > -1 {
        let strips = this.addr() + 0x58;
        let entry = vector_at(e, strips, i32::from(level) as u32, 0x18);
        let bytes = e
            .mem
            .u32(entry + 0x10)
            .wrapping_sub(e.mem.u32(entry + 0x0C)) as i32;
        (bytes >> 2) as u16
    } else {
        0
    }
}

// Translated from 00b0aa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first strip length of a detail level of an indexed geometry (`this`; unnamed in the
/// engine map): null unless the level is below the geometry's level count (`+0x20`) and its
/// entry of the strip-length vector (`+0x40`, 0x18-byte entries) has elements; then the
/// address of the entry's first element.
pub fn fn_00b0aa20(e: &mut Engine, this: Ptr, level: i16) -> u32 {
    if i32::from(level) >= 0 && i32::from(level) < i32::from(e.mem.u16(this.addr() + 0x20)) {
        let lengths = this.addr() + 0x40;
        let index = i32::from(level) as u32;
        let entry = vector_at(e, lengths, index, 0x18);
        let count = (e
            .mem
            .u32(entry + 0x10)
            .wrapping_sub(e.mem.u32(entry + 0x0C)) as i32)
            >> 1;
        if count == 0 {
            return 0;
        }
        let entry = vector_at(e, lengths, index, 0x18);
        let count = (e
            .mem
            .u32(entry + 0x10)
            .wrapping_sub(e.mem.u32(entry + 0x0C)) as i32)
            >> 1;
        if count == 0 {
            e.call(INVALID_PARAMETER, &args![]);
        }
        e.mem.u32(entry + 0x0C)
    } else {
        0
    }
}

// Translated from 00b0ab20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0ab20 - the FS:[0] exception frame is not translated.
/// `CSpeedTreeRT::GetGeometry` (unnamed in the engine map): fills the geometry record `out`
/// for what `flags` ask: bit 0 the branches (`00b0b1d0`, with the low word of `branch_lod`),
/// bit 1 the fronds (`00b0b920`), bit 2 the leaves (`00b0bc00`), bit 3 the billboards: with the
/// 360-degree billboard flag (`+0x6C`) set and bit 4 of `flags` clear `00b0c050` (the flags),
/// else `00b0c4f0`.
pub fn fn_00b0ab20(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    out: u32,
    flags: u32,
    branch_lod: u16,
    frond_lod: u16,
    leaf_lod: u16,
) {
    if flags & 1 != 0 {
        c_speed_tree_rt_get_branch_geometry(e, this, out, branch_lod as i16);
    }
    if flags & 2 != 0 {
        c_speed_tree_rt_get_frond_geometry(e, this, out, frond_lod as i16);
    }
    if flags & 4 != 0 {
        fn_00b0bc00(e, this, out, leaf_lod as i16);
    }
    if flags & 8 != 0 {
        if e.get(this, CSpeedTreeRT::m_b360Billboard) != 0 && flags & 0x10 == 0 {
            fn_00b0c050(e, this, out, flags);
        } else {
            fn_00b0c4f0(e, this, out);
        }
    }
}

// Translated from 00b0dfb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0dfb0 - the FS:[0] exception frame is not translated.
/// The constructor of the `std::vector<SShape>` of `SCollisionObjects` (unnamed in the engine
/// map): a proxy block from `0044d670` (1, 0) whose first word is `this`, stored at `+0`,
/// then the vector's initialising `00b0e9c0(this, 0)`. Returns `this`.
pub fn fn_00b0dfb0(e: &mut Engine, this: Ptr) -> Ptr {
    let proxy = e.call(0x0044_d670, &args![1u32, 0u32]).u32();
    if proxy != 0 {
        e.mem.set_u32(proxy, this.addr());
    }
    e.mem.set_u32(this.addr(), proxy);
    e.call(0x00b0_e9c0, &args![this, 0u32]);
    this
}

// Translated from 00b0dc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `vector::clear` as `erase(begin, end)` (unnamed in the engine map): both iterators are made
/// through the checked constructor (`this` null or an end/begin outside the vector takes the
/// invalid-parameter path) and handed to `00b0e510` with a result slot.
pub fn fn_00b0dc10(e: &mut Engine, this: Ptr) {
    let vector = this.addr();
    let last = e.mem.u32(vector + 0x10);
    let end_proxy = checked_iterator(e, vector, last);
    let first = e.mem.u32(vector + 0x0C);
    let begin_proxy = checked_iterator(e, vector, first);
    e.with_stack(8, |e, slot| {
        e.call(
            0x00b0_e510,
            &args![this, slot, begin_proxy, first, end_proxy, last],
        );
    });
}

// Translated from 00b0e090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<SShape>::push_back` (unnamed in the engine map): with room (the size is
/// below the capacity, a vector that holds nothing has capacity 0) the element is constructed
/// at the end by `00b10990` (the position, a count of 1, `value`, the allocator at `+8` and two
/// byte flags; the second is an uninitialised stack byte in the exe and passed as 0 here) and
/// the end moves on by 0x1C; otherwise a checked end iterator and `value` go to the insert
/// `00b0e880`.
pub fn fn_00b0e090(e: &mut Engine, this: Ptr, value: Ptr) {
    let vector = this.addr();
    let size = vector_len(e, vector, 0x1C);
    let capacity = if e.mem.u32(vector + 0x0C) == 0 {
        0
    } else {
        let bytes = e
            .mem
            .u32(vector + 0x14)
            .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32;
        (bytes / 0x1C) as u32
    };
    if size < capacity {
        let position = e.mem.u32(vector + 0x10);
        e.call(
            0x00b1_0990,
            &args![position, 1u32, value, vector + 8, 0u32, 0u32],
        );
        e.mem.set_u32(vector + 0x10, position.wrapping_add(0x1C));
    } else {
        let last = e.mem.u32(vector + 0x10);
        let proxy = checked_iterator(e, vector, last);
        e.with_stack(8, |e, slot| {
            e.call(0x00b0_e880, &args![this, slot, proxy, last, value]);
        });
    }
}

// Translated from 00b0cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec3::operator+` (the sum `other + this`, unnamed in the engine map): the three sums
/// are formed first, then stored to `result` through the x87 stack. Returns `result`.
pub fn fn_00b0cf90(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    let (a, b) = (this.addr(), other.addr());
    let mut sums = [0u32; 3];
    for (axis, slot) in sums.iter_mut().enumerate() {
        let offset = 4 * axis as u32;
        *slot = x87_store32(x87_add(fld(e, b + offset), operand(e, a + offset)));
    }
    for (axis, bits) in sums.into_iter().enumerate() {
        e.mem
            .set_u32(result.addr() + 4 * axis as u32, x87_quiet_bits(bits));
    }
    result
}

// Translated from 00b0cff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec3::operator*` by a scalar (`this * scalar`, unnamed in the engine map): the three
/// products are formed first, then stored to `result` through the x87 stack. Returns `result`.
pub fn fn_00b0cff0(e: &mut Engine, this: Ptr, result: Ptr, scalar: f32) -> Ptr {
    let a = this.addr();
    let mut products = [0u32; 3];
    for (axis, slot) in products.iter_mut().enumerate() {
        let offset = 4 * axis as u32;
        *slot = x87_store32(x87_mul(
            x87_load32(scalar.to_bits()),
            operand(e, a + offset),
        ));
    }
    for (axis, bits) in products.into_iter().enumerate() {
        e.mem
            .set_u32(result.addr() + 4 * axis as u32, x87_quiet_bits(bits));
    }
    result
}

/// `vector.empty() ? 0 : &vector[0]` for the vector of `1 << shift`-byte elements at `vector`
/// (the element count is the byte size shifted right, arithmetically; the exe tests it for zero,
/// then again through the checked `operator[](0)`).
fn vector_front_or_null(e: &mut Engine, vector: u32, shift: u32) -> u32 {
    let count = |e: &Engine| {
        (e.mem
            .u32(vector + 0x10)
            .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32)
            >> shift
    };
    if count(e) == 0 {
        return 0;
    }
    if count(e) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(vector + 0x0C)
}

// Translated from 00b0b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first colour of an indexed geometry (`this`; unnamed in the engine map): the address of
/// the first element of its vector at `+0x70` (`m_vColors` in the Xbox PDB; 4-byte elements), or
/// null when it is empty.
pub fn fn_00b0b4b0(e: &mut Engine, this: Ptr) -> u32 {
    vector_front_or_null(e, this.addr() + 0x70, 2)
}

// Translated from 00b0b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first coordinate of an indexed geometry (`this`; unnamed in the engine map): the first
/// element of its vector at `+0x88` (`m_vCoords`), or null when it is empty.
pub fn fn_00b0b530(e: &mut Engine, this: Ptr) -> u32 {
    vector_front_or_null(e, this.addr() + 0x88, 2)
}

// Translated from 00b0b5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first normal of an indexed geometry (`this`; unnamed in the engine map): the first
/// element of its vector at `+0xB8` (`m_vNormals`), or null when it is empty.
pub fn fn_00b0b5b0(e: &mut Engine, this: Ptr) -> u32 {
    vector_front_or_null(e, this.addr() + 0xB8, 2)
}

// Translated from 00b0b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first binormal of an indexed geometry (`this`; unnamed in the engine map): the first
/// element of its vector at `+0xD0` (`m_vBinormals`), or null when it is empty.
pub fn fn_00b0b630(e: &mut Engine, this: Ptr) -> u32 {
    vector_front_or_null(e, this.addr() + 0xD0, 2)
}

// Translated from 00b0b6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first tangent of an indexed geometry (`this`; unnamed in the engine map): the first
/// element of its vector at `+0xE8` (`m_vTangents`), or null when it is empty.
pub fn fn_00b0b6b0(e: &mut Engine, this: Ptr) -> u32 {
    vector_front_or_null(e, this.addr() + 0xE8, 2)
}

// Translated from 00b0b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first texture coordinate of an indexed geometry (`this`; unnamed in the engine map): the
/// first element of its vector at `+0x100` (`m_vTexCoords0`), or null when it is empty.
pub fn fn_00b0b730(e: &mut Engine, this: Ptr) -> u32 {
    vector_front_or_null(e, this.addr() + 0x100, 2)
}

// Translated from 00b0b7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first second-set texture coordinate of an indexed geometry (`this`; unnamed in the
/// engine map): the first element of its vector at `+0x148` (`m_vTexCoords1`), or null when it
/// is empty.
pub fn fn_00b0b7b0(e: &mut Engine, this: Ptr) -> u32 {
    vector_front_or_null(e, this.addr() + 0x148, 2)
}

// Translated from 00b0b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first wind weight of an indexed geometry (`this`; unnamed in the engine map): the first
/// element of its vector at `+0x160` (`m_vWindWeights`), or null when it is empty.
pub fn fn_00b0b830(e: &mut Engine, this: Ptr) -> u32 {
    vector_front_or_null(e, this.addr() + 0x160, 2)
}

// Translated from 00b0b8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first wind matrix index of an indexed geometry (`this`; unnamed in the engine map): the
/// first element of its byte vector at `+0x178` (`m_vWindMatrixIndices`), or null when it is
/// empty.
pub fn fn_00b0b8b0(e: &mut Engine, this: Ptr) -> u32 {
    vector_front_or_null(e, this.addr() + 0x178, 0)
}

/// The leaf, billboard and level-count value `GetTransitionValues` (`00b0c8d0`) computes its
/// blends from, as the geometry getters call it: `lod` is the level-of-detail float, `count`
/// the number of discrete levels (plus one with the discrete flag at `011f8b78`), `radius`,
/// `factor` and `exponent` the transition settings, `alpha` the tree's target alpha; the
/// outputs are two floats and two level words at the given addresses.
#[allow(clippy::too_many_arguments)]
fn transition_values(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    lod: f32,
    count: u16,
    radius: f32,
    outputs: [u32; 4],
) {
    let factor = f32::from_bits(pushed_float(e, this.addr() + 0x28));
    let exponent = f32::from_bits(pushed_float(e, this.addr() + 0x20));
    let alpha = f32::from(e.get(this, CSpeedTreeRT::m_ucTargetAlphaValue));
    c_speed_tree_rt_get_transition_values(
        e,
        lod,
        u32::from(count),
        radius,
        factor,
        exponent,
        alpha,
        outputs[0],
        outputs[1],
        outputs[2],
        outputs[3],
    );
}

/// The shared body of `GetBranchGeometry` and `GetFrondGeometry` (`00b0b1d0`, `00b0b920`):
/// `base` is the geometry record to fill (the frond's sits `0x3C` bytes into the caller's),
/// `geometry` the tree's geometry field and `discrete` the discrete-level function for a
/// level of `-1`. The record gets the vertex count (the coordinate vector's floats over 3), the
/// nine pointers of the geometry's front-of-vector getters, the discrete level, the strip count,
/// strip lengths and first strip of that level, and a blend alpha at `+0x38`: the tree's target
/// alpha, or with the discrete flag the transition value of the top two levels.
fn get_geometry_record(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    base: u32,
    geometry: Field<CSpeedTreeRT, Ptr>,
    lod: i16,
    discrete: fn(&mut Engine, Ptr<CSpeedTreeRT>, f32) -> u16,
) {
    let coords = e.get(this, geometry).addr() + 0x88;
    let floats = (e
        .mem
        .u32(coords + 0x10)
        .wrapping_sub(e.mem.u32(coords + 0x0C)) as i32)
        >> 2;
    e.mem.set_u16(base + 0x10, ((floats as u32) / 3) as u16);
    type Getter = fn(&mut Engine, Ptr) -> u32;
    let getters: [(u32, Getter); 9] = [
        (0x14, fn_00b0b4b0),
        (0x24, fn_00b0b530),
        (0x18, fn_00b0b5b0),
        (0x1C, fn_00b0b630),
        (0x20, fn_00b0b6b0),
        (0x28, fn_00b0b730),
        (0x2C, fn_00b0b7b0),
        (0x30, fn_00b0b830),
        (0x34, fn_00b0b8b0),
    ];
    for (offset, getter) in getters {
        let geometry_pointer = e.get(this, geometry);
        let value = getter(e, geometry_pointer);
        e.mem.set_u32(base + offset, value);
    }
    let level = if lod == -1 {
        let sentinel = f32::from_bits(pushed_float(e, 0x0101_2054));
        discrete(e, this, sentinel) as i16
    } else {
        lod
    };
    e.mem.set_i32(base, i32::from(level));
    let geometry_pointer = e.get(this, geometry);
    let strips = fn_00b0a9a0(e, geometry_pointer, level);
    e.mem.set_u16(base + 4, strips);
    let geometry_pointer = e.get(this, geometry);
    let lengths = fn_00b0aa20(e, geometry_pointer, level);
    e.mem.set_u32(base + 8, lengths);
    let geometry_pointer = e.get(this, geometry);
    let first = e
        .call(0x00b3_0730, &args![geometry_pointer, level as u16])
        .u32();
    e.mem.set_u32(base + 0xC, first);
    let alpha = e.get(this, CSpeedTreeRT::m_ucTargetAlphaValue);
    if e.global::<u8>(DISCRETE_LEAF_LOD_FLAG) != 0 {
        let count = (u32::from(fn_00b08710(e, this)) + 1) as u16;
        e.with_stack(0x10, |e, slots| {
            let (first_value, second_value) = (slots.addr(), slots.addr() + 4);
            let (first_level, second_level) = (slots.addr() + 8, slots.addr() + 10);
            e.mem.set_u32(first_value, pushed_float(e, 0x0101_2054));
            e.mem.set_u32(second_value, pushed_float(e, 0x0101_2054));
            e.mem.set_u16(first_level, 0xFFFF);
            e.mem.set_u16(second_level, 0xFFFF);
            let current = c_speed_tree_rt_get_lod_level(e, this);
            let radius = f32::from_bits(pushed_float(e, this.addr() + 0x1C));
            transition_values(
                e,
                this,
                current,
                count,
                radius,
                [first_value, second_value, first_level, second_level],
            );
            let transition_level = i32::from(e.mem.i16(first_level));
            if transition_level == i32::from(count) - 2 {
                copy_float(e, base + 0x38, first_value);
            } else if transition_level == i32::from(count) - 1 {
                e.mem.set_u32(base + 0x38, pushed_float(e, 0x0102_3cd8));
                e.mem.set_i32(base, -1);
            } else {
                e.mem.set_u32(base + 0x38, x87_store32(f64::from(alpha)));
            }
        });
    } else {
        e.mem.set_u32(base + 0x38, x87_store32(f64::from(alpha)));
    }
}

// Translated from 00b0b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::GetBranchGeometry` (Xbox PDB): fills the branch part of the geometry record
/// `out` for the level `lod` (`-1`: the discrete branch level of the current level of detail;
/// see [`get_geometry_record`]); without a branch geometry reports through `SetError`.
pub fn c_speed_tree_rt_get_branch_geometry(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    out: u32,
    lod: i16,
) {
    if !e.get(this, CSpeedTreeRT::m_pBranchGeometry).is_null() {
        get_geometry_record(
            e,
            this,
            out,
            CSpeedTreeRT::m_pBranchGeometry,
            lod,
            c_speed_tree_rt_get_discrete_branch_lod_level,
        );
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_53c0);
    }
}

// Translated from 00b0b920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::GetFrondGeometry` (Xbox PDB): fills the frond part (`+0x3C` into the record
/// `out`) for the level `lod` (`-1`: the discrete frond level of the current level of detail;
/// see [`get_geometry_record`]); without a frond geometry reports through `SetError`.
pub fn c_speed_tree_rt_get_frond_geometry(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    out: u32,
    lod: i16,
) {
    if !e.get(this, CSpeedTreeRT::m_pFrondGeometry).is_null() {
        get_geometry_record(
            e,
            this,
            out + 0x3C,
            CSpeedTreeRT::m_pFrondGeometry,
            lod,
            c_speed_tree_rt_get_discrete_frond_lod_level,
        );
    } else {
        c_speed_tree_rt_set_error(e, 0x010a_5408);
    }
}

/// `atan2` (`00ec9760`, `cdecl`, y then x) and the unconfirmed math function `00ec9610` (one
/// `double`; Ghidra's name for the stub is ambiguous), and the double the angles are scaled by
/// (`010a4d90`).
const ATAN2: u32 = 0x00ec_9760;
const ANGLE_MATH_FUNCTION: u32 = 0x00ec_9610;
const ANGLE_SCALE: u32 = 0x010a_4d90;

/// The two angles of the stored camera direction (`011f8eac`, copied through the x87 stack)
/// the leaf geometry calls take: `atan2(y, x)` and the unconfirmed function of the z component,
/// each rounded to a float, times the scale double and rounded to a float; the second is
/// negated. Returns the bits of both floats.
fn camera_angles(e: &mut Engine) -> (u32, u32) {
    let scale: f64 = e.global(ANGLE_SCALE);
    let direction: [f32; 3] =
        std::array::from_fn(|i| f32::from_bits(pushed_float(e, CAMERA_DIRECTION + 4 * i as u32)));
    let first = e
        .call(
            ATAN2,
            &args![f64::from(direction[1]), f64::from(direction[0])],
        )
        .f64();
    let first = x87_quiet(f32::from_bits(x87_store32(first)));
    let azimuth = x87_store32(x87_mul(f64::from(first), scale));
    let second = e
        .call(ANGLE_MATH_FUNCTION, &args![f64::from(direction[2])])
        .f64();
    let second = x87_quiet(f32::from_bits(x87_store32(second)));
    let scaled = f32::from_bits(x87_store32(x87_mul(f64::from(second), scale)));
    let elevation = x87_store32(-f64::from(scaled));
    (azimuth, elevation)
}

/// A call of the leaf geometry's `00b28e40` or `00b288f0` (`this` = the leaf geometry at
/// `+8`): the record `target`, a level, the two camera angles and the tree's `+0x24` float.
fn call_leaf_geometry(
    e: &mut Engine,
    this: Ptr<CSpeedTreeRT>,
    function: u32,
    target: u32,
    level: u32,
    angles: (u32, u32),
) {
    let leaf_geometry = e.get(this, CSpeedTreeRT::m_pLeafGeometry);
    let extra = pushed_float(e, this.addr() + 0x24);
    e.call(
        function,
        &args![
            leaf_geometry,
            target,
            level,
            x87_quiet_bits(angles.0),
            x87_quiet_bits(angles.1),
            extra
        ],
    );
}

/// `level != -1 && level < count` for a level word and the leaf level count.
fn level_in_range(level: i16, count: u16) -> bool {
    i32::from(level) != -1 && i32::from(level) < i32::from(count)
}

// Translated from 00b0ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The leaf geometry of the tree for a level-of-detail float (unnamed in the engine map; `lod`
/// below zero ends the function after the first part). `GetTransitionValues` picks the two
/// neighbouring leaf levels and their blend alphas; with the discrete flag (`011f8b78`) the
/// record's `+0xFC` alpha is the blend of the top level and `+0xF0` says whether it is not
/// `-1`; `+0x38` is the tree's target alpha or the top levels' blend. Then, by the leaf lod
/// method (`+0x18`): 1 asks the leaf geometry (`00b28e40`) for both levels (records at `+0x78`
/// and `+0xB4`, their alphas at `+0x7C` and `+0xB8`, their valid bytes), 3 for level 0 only,
/// anything else for the discrete level of `00b08610`.
pub fn fn_00b0ad00(e: &mut Engine, this: Ptr<CSpeedTreeRT>, out: u32, lod: f32) {
    let levels = fn_00b08710(e, this);
    let flag = e.global::<u8>(DISCRETE_LEAF_LOD_FLAG) != 0;
    let total = (u32::from(levels) + u32::from(flag)) as u16;
    e.with_stack(0x10, |e, slots| {
        let (first_value, second_value) = (slots.addr(), slots.addr() + 4);
        let (first_level, second_level) = (slots.addr() + 8, slots.addr() + 10);
        e.mem.set_u32(first_value, pushed_float(e, 0x0101_2054));
        e.mem.set_u32(second_value, pushed_float(e, 0x0101_2054));
        e.mem.set_u16(first_level, 0xFFFF);
        e.mem.set_u16(second_level, 0xFFFF);
        let radius = f32::from_bits(pushed_float(e, this.addr() + 0x1C));
        transition_values(
            e,
            this,
            f32::from_bits(x87_quiet_bits(lod.to_bits())),
            total,
            radius,
            [first_value, second_value, first_level, second_level],
        );
        let top = i32::from(total) - 1;
        if flag {
            e.mem.set_u32(out + 0xFC, pushed_float(e, 0x0101_2054));
            if i32::from(e.mem.i16(first_level)) == top {
                copy_float(e, out + 0xFC, first_value);
            } else if i32::from(e.mem.i16(second_level)) == top {
                copy_float(e, out + 0xFC, second_value);
            }
            let sentinel: f64 = e.global(0x0101_a6b0);
            let different = fld(e, out + 0xFC) != sentinel;
            e.mem.set_u8(out + 0xF0, different as u8);
        } else {
            e.mem.set_u8(out + 0xF0, 0);
        }
        let zero: f64 = e.global(0x0101_2060);
        let below_zero = x87_load32(lod.to_bits()) < zero;
        if below_zero {
            return;
        }
        let alpha = f64::from(e.get(this, CSpeedTreeRT::m_ucTargetAlphaValue));
        if flag {
            if i32::from(e.mem.i16(first_level)) == top - 1 {
                copy_float(e, out + 0x38, first_value);
            } else if i32::from(e.mem.i16(first_level)) == top {
                e.mem.set_u32(out + 0x38, pushed_float(e, 0x0102_3cd8));
            } else {
                e.mem.set_u32(out + 0x38, x87_store32(alpha));
            }
        } else {
            e.mem.set_u32(out + 0x38, x87_store32(alpha));
        }
        let angles = camera_angles(e);
        let method = e.get(this, CSpeedTreeRT::m_eLeafLodMethod);
        let alpha_bits = x87_store32(alpha);
        if method == 1 {
            call_leaf_geometry(e, this, 0x00b2_8e40, out + 0x78, 0, angles);
            let in_range = level_in_range(e.mem.i16(first_level), levels);
            e.mem.set_u8(out + 0x78, in_range as u8);
            if e.mem.u8(out + 0x78) != 0 {
                let level = u32::from(e.mem.u16(first_level));
                call_leaf_geometry(e, this, 0x00b2_8e40, out + 0x78, level, angles);
                copy_float(e, out + 0x7C, first_value);
            }
            let in_range = level_in_range(e.mem.i16(second_level), levels);
            e.mem.set_u8(out + 0xB4, in_range as u8);
            if e.mem.u8(out + 0xB4) != 0 {
                let level = u32::from(e.mem.u16(second_level));
                call_leaf_geometry(e, this, 0x00b2_8e40, out + 0xB4, level, angles);
                copy_float(e, out + 0xB8, second_value);
            }
        } else if method == 3 {
            call_leaf_geometry(e, this, 0x00b2_8e40, out + 0x78, 0, angles);
            e.mem.set_u8(out + 0x78, 1);
            e.mem.set_u32(out + 0x7C, alpha_bits);
            e.mem.set_u8(out + 0xB4, 0);
        } else {
            let sentinel = f32::from_bits(pushed_float(e, 0x0101_2054));
            let level = fn_00b08610(e, this, sentinel);
            if u32::from(level) < u32::from(fn_00b08710(e, this)) {
                call_leaf_geometry(e, this, 0x00b2_8e40, out + 0x78, u32::from(level), angles);
                e.mem.set_u8(out + 0x78, 1);
                e.mem.set_u32(out + 0x7C, alpha_bits);
            }
            e.mem.set_u8(out + 0xB4, 0);
        }
    });
}

// Translated from 00b0bc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The leaf geometry of the tree for a leaf level (unnamed in the engine map): asks the leaf
/// geometry (`00b288f0`, the record `out + 0x78`, the camera angles) for every leaf level in
/// turn, then, for a level of 0 or more, `00b28e40` for that level (the record valid, its alpha
/// at `+0x7C` the tree's target alpha, the second record `+0xB4` invalid); for a level of `-1`
/// by the leaf lod method (`+0x18`): 1 the two blended levels `GetTransitionValues` picks
/// (records `+0x78` and `+0xB4`, alphas `+0x7C` and `+0xB8`), 3 level 0 only, anything else
/// the discrete level of `00b08610`.
pub fn fn_00b0bc00(e: &mut Engine, this: Ptr<CSpeedTreeRT>, out: u32, level: i16) {
    let levels = fn_00b08710(e, this);
    let flag = e.global::<u8>(DISCRETE_LEAF_LOD_FLAG) != 0;
    let total = (u32::from(levels) + u32::from(flag)) as u16;
    let angles = camera_angles(e);
    let mut index = 0u16;
    while u32::from(index) < u32::from(fn_00b08710(e, this)) {
        call_leaf_geometry(e, this, 0x00b2_88f0, out + 0x78, u32::from(index), angles);
        index = index.wrapping_add(1);
    }
    let alpha = f64::from(e.get(this, CSpeedTreeRT::m_ucTargetAlphaValue));
    if i32::from(level) > -1 {
        call_leaf_geometry(
            e,
            this,
            0x00b2_8e40,
            out + 0x78,
            u32::from(level as u16),
            angles,
        );
        e.mem.set_u8(out + 0x78, 1);
        e.mem.set_u32(out + 0x7C, x87_store32(alpha));
        e.mem.set_u8(out + 0xB4, 0);
        return;
    }
    let method = e.get(this, CSpeedTreeRT::m_eLeafLodMethod);
    if method == 1 {
        e.with_stack(0x10, |e, slots| {
            let (first_value, second_value) = (slots.addr(), slots.addr() + 4);
            let (first_level, second_level) = (slots.addr() + 8, slots.addr() + 10);
            e.mem.set_u32(first_value, pushed_float(e, 0x0101_2054));
            e.mem.set_u32(second_value, pushed_float(e, 0x0101_2054));
            e.mem.set_u16(first_level, 0xFFFF);
            e.mem.set_u16(second_level, 0xFFFF);
            let current = c_speed_tree_rt_get_lod_level(e, this);
            let radius = f32::from_bits(pushed_float(e, this.addr() + 0x1C));
            transition_values(
                e,
                this,
                current,
                total,
                radius,
                [first_value, second_value, first_level, second_level],
            );
            let in_range = level_in_range(e.mem.i16(first_level), levels);
            e.mem.set_u8(out + 0x78, in_range as u8);
            if e.mem.u8(out + 0x78) != 0 {
                let target = u32::from(e.mem.u16(first_level));
                call_leaf_geometry(e, this, 0x00b2_8e40, out + 0x78, target, angles);
                copy_float(e, out + 0x7C, first_value);
            }
            let in_range = level_in_range(e.mem.i16(second_level), levels);
            e.mem.set_u8(out + 0xB4, in_range as u8);
            if e.mem.u8(out + 0xB4) != 0 {
                let target = u32::from(e.mem.u16(second_level));
                call_leaf_geometry(e, this, 0x00b2_8e40, out + 0xB4, target, angles);
                copy_float(e, out + 0xB8, second_value);
            }
        });
    } else if method == 3 {
        call_leaf_geometry(e, this, 0x00b2_8e40, out + 0x78, 0, angles);
        e.mem.set_u8(out + 0x78, 1);
        e.mem.set_u32(out + 0x7C, x87_store32(alpha));
        e.mem.set_u8(out + 0xB4, 0);
    } else {
        let sentinel = f32::from_bits(pushed_float(e, 0x0101_2054));
        let discrete = fn_00b08610(e, this, sentinel);
        if u32::from(discrete) < u32::from(fn_00b08710(e, this)) {
            call_leaf_geometry(
                e,
                this,
                0x00b2_8e40,
                out + 0x78,
                u32::from(discrete),
                angles,
            );
            e.mem.set_u8(out + 0x78, 1);
            e.mem.set_u32(out + 0x7C, x87_store32(alpha));
        }
        e.mem.set_u8(out + 0xB4, 0);
    }
}

// Translated from 00b0c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The azimuth of the `CVec3` at `vector` in degrees (`cdecl`; unnamed in the engine map):
/// `atan2(y, x)` (`00ec9760`) rounded to a float, times the double at `010a4d90`, rounded to a
/// float.
pub fn fn_00b0c470(e: &mut Engine, vector: Ptr) -> f32 {
    let x = f32::from_bits(pushed_float(e, vector.addr()));
    let y = f32::from_bits(pushed_float(e, vector.addr() + 4));
    let angle = e.call(ATAN2, &args![f64::from(y), f64::from(x)]).f64();
    let angle = x87_quiet(f32::from_bits(x87_store32(angle)));
    let scale: f64 = e.global(ANGLE_SCALE);
    f32::from_bits(x87_quiet_bits(x87_store32(x87_mul(
        f64::from(angle),
        scale,
    ))))
}

// Translated from 00b0c4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The billboard part of the geometry record `out` (unnamed in the engine map). Without the
/// discrete flag (`011f8b78`) the record's valid bytes (`+0xF0`, `+0x104`, `+0x118`) are cleared.
/// With it, `GetTransitionValues` gives the blend of the top level (`+0xFC`; `+0xF0` says it is
/// not `-1`), `+0xF8` the simple billboard's vertices (`00b0c780` with the tree sizes' `+0x18`
/// and `+0x14` floats) and `+0xF4` the embedded billboard texture coordinates; `+0x118` and
/// `+0x11C` to `+0x124` describe the horizontal billboard when the tree has one (`+0x6D`).
pub fn fn_00b0c4f0(e: &mut Engine, this: Ptr<CSpeedTreeRT>, out: u32) {
    if e.global::<u8>(DISCRETE_LEAF_LOD_FLAG) == 0 {
        e.mem.set_u8(out + 0xF0, 0);
        e.mem.set_u8(out + 0x104, 0);
        e.mem.set_u8(out + 0x118, 0);
        return;
    }
    let total = (u32::from(fn_00b08710(e, this)) + 1) as u16;
    e.with_stack(0x10, |e, slots| {
        let (first_value, second_value) = (slots.addr(), slots.addr() + 4);
        let (first_level, second_level) = (slots.addr() + 8, slots.addr() + 10);
        e.mem.set_u32(first_value, pushed_float(e, 0x0101_2054));
        e.mem.set_u32(second_value, pushed_float(e, 0x0101_2054));
        e.mem.set_u16(first_level, 0xFFFF);
        e.mem.set_u16(second_level, 0xFFFF);
        let current = c_speed_tree_rt_get_lod_level(e, this);
        let radius = f32::from_bits(pushed_float(e, this.addr() + 0x1C));
        transition_values(
            e,
            this,
            current,
            total,
            radius,
            [first_value, second_value, first_level, second_level],
        );
        let top = i32::from(total) - 1;
        e.mem.set_u32(out + 0xFC, pushed_float(e, 0x0101_2054));
        if i32::from(e.mem.i16(first_level)) == top {
            copy_float(e, out + 0xFC, first_value);
        } else if i32::from(e.mem.i16(second_level)) == top {
            copy_float(e, out + 0xFC, second_value);
        }
        let sentinel: f64 = e.global(0x0101_a6b0);
        let different = fld(e, out + 0xFC) != sentinel;
        e.mem.set_u8(out + 0xF0, different as u8);
        e.mem.set_u16(out + 0x100, 0);
        if e.mem.u8(out + 0xF0) != 0 {
            let sizes = e.get(this, CSpeedTreeRT::m_pTreeSizes).addr();
            let (width, length) = (
                f32::from_bits(pushed_float(e, sizes + 0x14)),
                f32::from_bits(pushed_float(e, sizes + 0x18)),
            );
            let billboard = e.get(this, CSpeedTreeRT::m_pSimpleBillboard);
            let vertices = fn_00b0c780(e, billboard, length, width);
            e.mem.set_u32(out + 0xF8, vertices.addr());
            let embedded = e.get(this, CSpeedTreeRT::m_pEmbeddedTexCoords);
            if !embedded.is_null() {
                let coords = e.mem.u32(embedded.addr() + 0xC);
                e.mem.set_u32(out + 0xF4, coords);
            } else {
                e.mem.set_u32(out + 0xF4, 0);
            }
        }
        e.mem.set_u8(out + 0x104, 0);
        e.mem.set_u16(out + 0x114, 0);
        if e.get(this, CSpeedTreeRT::m_bHorizontalBillboard) != 0 {
            e.mem.set_u32(out + 0x120, this.addr() + 0x70);
            let embedded = e.get(this, CSpeedTreeRT::m_pEmbeddedTexCoords);
            if !embedded.is_null() {
                let coords = e.mem.u32(embedded.addr() + 0xC);
                let billboards = e.mem.u32(embedded.addr() + 8);
                let last = billboards
                    .wrapping_mul(8)
                    .wrapping_sub(8)
                    .wrapping_mul(4)
                    .wrapping_add(coords);
                e.mem.set_u32(out + 0x11C, last);
            } else {
                e.mem.set_u32(out + 0x11C, 0);
            }
            copy_float(e, out + 0x124, out + 0xFC);
            let zero: f64 = e.global(0x0101_2060);
            let valid = fld(e, out + 0x124) >= zero;
            e.mem.set_u8(out + 0x118, valid as u8);
            copy_float(e, out + 0x124, 0x011a_d124);
        } else {
            e.mem.set_u8(out + 0x118, 0);
        }
        e.mem.set_u16(out + 0x128, 0);
    });
}

// Translated from 00b0c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the vertices of the simple billboard `this` (a `CSimpleBillboard`; unnamed in the
/// engine map) once: unless its valid byte (`+0x30`) is set, each of the four corners (12-byte
/// entries of the unit table at `011f8d08`) is scaled by `(first, first, second)` into the
/// billboard's 48 floats, and the byte is set. Returns `this`.
pub fn fn_00b0c780(e: &mut Engine, this: Ptr, first: f32, second: f32) -> Ptr {
    if e.mem.u8(this.addr() + 0x30) == 0 {
        let scales = [first, first, second];
        for (corner, base) in [0x011f_8d08u32, 0x011f_8d14, 0x011f_8d20, 0x011f_8d2c]
            .into_iter()
            .enumerate()
        {
            for (axis, scale) in scales.into_iter().enumerate() {
                let offset = 4 * axis as u32;
                let value = x87_mul(x87_load32(scale.to_bits()), operand(e, base + offset));
                fstp(e, this.addr() + 12 * corner as u32 + offset, value);
            }
        }
        e.mem.set_u8(this.addr() + 0x30, 1);
    }
    this
}

// Translated from 00b0c050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 360-degree billboard part of the geometry record `out` (unnamed in the engine map): the
/// plain billboard part first (`00b0c4f0`); when that is valid and the tree has 360-degree
/// billboards (`+0x54`), the camera's azimuth (`00b0c470` of the negated direction, wrapped by
/// adding `[0103c348]` when below zero) picks the two neighbouring billboard images
/// (`GetTransitionValues` with the blend `a * (1 - 2h) + h`, `h = [01011588] / count`), the
/// levels are reversed and wrapped, and the record gets their texture coordinates (`+0xF4`,
/// `+0x108` from `+0xC` of the embedded coordinates), blend alphas (`+0xFC`, `+0x110`),
/// levels (`+0x100`, `+0x114`) and valid bytes (`+0x104`); with bit 5 of `flags` only the
/// nearer image is kept.
pub fn fn_00b0c050(e: &mut Engine, this: Ptr<CSpeedTreeRT>, out: u32, flags: u32) {
    fn_00b0c4f0(e, this, out);
    if e.mem.u8(out + 0xF0) == 0 {
        return;
    }
    let count = e.get(this, CSpeedTreeRT::m_usNum360Billboards);
    if count == 0 {
        return;
    }
    let alpha = f64::from(e.get(this, CSpeedTreeRT::m_ucTargetAlphaValue));
    let numerator = x87_sub(operand(e, out + 0xFC), alpha);
    let denominator = x87_sub(e.global::<f64>(0x0101_e568), alpha);
    let blend = x87_store32(x87_div(numerator, denominator));
    // The camera direction, negated, as a vector (the position copy beside it is unused).
    let negated: [u32; 3] = std::array::from_fn(|i| {
        x87_store32(-x87_load32(
            e.global::<u32>(CAMERA_DIRECTION + 4 * i as u32),
        ))
    });
    let mut azimuth = e.with_stack(0xC, |e, vector| {
        for (axis, bits) in negated.into_iter().enumerate() {
            e.mem
                .set_u32(vector.addr() + 4 * axis as u32, x87_quiet_bits(bits));
        }
        fn_00b0c470(e, vector)
    });
    let zero: f64 = e.global(0x0101_2060);
    if f64::from(azimuth) < zero {
        let full_turn: f64 = e.global(0x0103_c348);
        azimuth = f32::from_bits(x87_store32(x87_add(f64::from(azimuth), full_turn)));
    }
    let half: f64 = e.global(0x0101_1588);
    let full_turn: f64 = e.global(0x0103_c348);
    let step = x87_store32(x87_div(half, f64::from(count)));
    let fraction = x87_store32(x87_div(f64::from(azimuth), full_turn));
    let complement = x87_store32(x87_sub(1.0, x87_load32(step)));
    let blended = x87_store32(x87_add(
        x87_mul(
            x87_load32(fraction),
            x87_sub(x87_m32(complement), x87_load32(step)),
        ),
        x87_m32(step),
    ));
    e.with_stack(0x10, |e, slots| {
        let (first_value, second_value) = (slots.addr(), slots.addr() + 4);
        let (first_level, second_level) = (slots.addr() + 8, slots.addr() + 10);
        e.mem.set_u32(first_value, pushed_float(e, 0x0101_2054));
        e.mem.set_u32(second_value, pushed_float(e, 0x0101_2054));
        e.mem.set_u16(first_level, 0xFFFF);
        e.mem.set_u16(second_level, 0xFFFF);
        let radius = f32::from_bits(x87_quiet_bits(step));
        transition_values(
            e,
            this,
            f32::from_bits(blended),
            count,
            radius,
            [first_value, second_value, first_level, second_level],
        );
        // The levels count down from the other end and wrap into 0..count.
        let wrap = |e: &mut Engine, slot: u32| {
            let reversed = (i32::from(count) - 1 - i32::from(e.mem.i16(slot))) as u16;
            e.mem.set_u16(slot, reversed);
            while i32::from(e.mem.i16(slot)) >= i32::from(count) {
                let reduced = (i32::from(e.mem.i16(slot)) - i32::from(count)) as u16;
                e.mem.set_u16(slot, reduced);
            }
        };
        wrap(e, first_level);
        wrap(e, second_level);
        let alpha_float = f64::from(e.get(this, CSpeedTreeRT::m_ucTargetAlphaValue));
        let table = |e: &Engine, level: u32| {
            let embedded = e.mem.u32(this.addr() + 0x4C);
            let coords = e.mem.u32(embedded + 0xC);
            coords.wrapping_add((i32::from(e.mem.i16(level)) << 3).wrapping_mul(4) as u32)
        };
        // `blend * (255 - base) + base` for a float base.
        let mix = |e: &Engine, base: f64| {
            x87_store32(x87_add(
                x87_mul(x87_load32(blend), x87_sub(operand(e, 0x0102_3cd8), base)),
                base,
            ))
        };
        if flags & 0x20 != 0 {
            e.mem.set_u8(out + 0xF0, 1);
            let (first_alpha, second_alpha) = (fld(e, first_value), fld(e, second_value));
            let level = if second_alpha > first_alpha {
                first_level
            } else {
                second_level
            };
            let address = table(e, level);
            e.mem.set_u32(out + 0xF4, address);
            let base = x87_load32(x87_store32(alpha_float));
            e.mem.set_u32(out + 0xFC, mix(e, base));
            e.mem.set_u8(out + 0x104, 0);
        } else {
            if e.mem.u32(this.addr() + 0x4C) != 0 {
                let address = table(e, first_level);
                e.mem.set_u32(out + 0xF4, address);
            } else {
                e.mem.set_u32(out + 0xF4, 0);
            }
            let base = fld(e, first_value);
            e.mem.set_u32(out + 0xFC, mix(e, base));
            let level = e.mem.u16(first_level);
            e.mem.set_u16(out + 0x100, level);
            let word = e.mem.u32(out + 0xF8);
            e.mem.set_u32(out + 0x10C, word);
            if e.mem.u32(this.addr() + 0x4C) != 0 {
                let address = table(e, second_level);
                e.mem.set_u32(out + 0x108, address);
            } else {
                e.mem.set_u32(out + 0x108, 0);
            }
            let level = e.mem.u16(second_level);
            e.mem.set_u16(out + 0x114, level);
            e.mem.set_u8(out + 0x104, 1);
            let base = fld(e, second_value);
            e.mem.set_u32(out + 0x110, mix(e, base));
        }
    });
}

// Translated from 00b0c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::GetTransitionValues` (Xbox PDB), a static function (`cdecl`): from the
/// level-of-detail float `lod` (0 to 1, `1 - lod` counting up through the levels), the number of
/// levels `count`, the transition `radius`, `factor` and `exponent` and the tree's `alpha`
/// it finds the two levels to show and their blend alphas. `h = 1 / count`; `r = (1 - lod) / h`
/// is rounded to the nearest level `k`; `d = (1 - lod) - k * h` is its offset. At either end of
/// the range (`k` of 0 or `count`) or beyond the radius (`|d|` above it) the nearest level
/// alone is shown (`first_level` the level, capped at `count - 1`, `first_value` the alpha,
/// `second_value` 255 and `second_level` `-1`); otherwise two levels blend: `u = 1 - (radius -
/// d) / (radius * [01011590])`, `w = 1 - (u - factor) / (1 - factor)` and `x = u / (1 - factor)`
/// (both capped at 1 from above, a NaN becoming 1) give the levels `k - 1` and `k` the alphas
/// `pow(1 - w, exponent) * (255 - alpha) + alpha` and `pow(1 - x, exponent) * (255 - alpha) +
/// alpha`.
#[allow(clippy::too_many_arguments)]
pub fn c_speed_tree_rt_get_transition_values(
    e: &mut Engine,
    lod: f32,
    count: u32,
    radius: f32,
    factor: f32,
    exponent: f32,
    alpha: f32,
    first_value: u32,
    second_value: u32,
    first_level: u32,
    second_level: u32,
) {
    let levels = u32::from(count as u16);
    let lod_value = x87_load32(lod.to_bits());
    let step = x87_store32(x87_div(1.0, f64::from(levels)));
    let ratio = x87_store32(x87_div(x87_sub(1.0, lod_value), x87_load32(step)));
    let mut nearest = ftol(e, x87_load32(ratio));
    let fraction = x87_store32(x87_sub(x87_m32(ratio), f64::from(nearest)));
    let half: f64 = e.global(0x0101_1588);
    if x87_load32(fraction) >= half {
        nearest = nearest.wrapping_add(1);
    }
    let level = nearest as u16;
    let offset = x87_store32(x87_sub(
        x87_sub(1.0, lod_value),
        x87_mul(f64::from(level), x87_m32(step)),
    ));
    if level != 0 && u32::from(level) != levels {
        let distance = e
            .call(ABSOLUTE_VALUE, &args![f64::from(f32::from_bits(offset))])
            .f64();
        let distance = x87_store32(distance);
        // `FCOMPP`: the single-level path is taken when the radius is below the distance.
        let beyond_radius = x87_load32(radius.to_bits()) < x87_load32(distance);
        if !beyond_radius {
            // Two levels blend.
            let numerator = x87_sub(x87_load32(radius.to_bits()), x87_m32(offset));
            let denominator = x87_mul(x87_load32(radius.to_bits()), e.global::<f64>(0x0101_1590));
            let u = x87_store32(x87_sub(1.0, x87_div(numerator, denominator)));
            let one_minus_factor = x87_sub(1.0, x87_load32(factor.to_bits()));
            let w = x87_store32(x87_sub(
                1.0,
                x87_div(
                    x87_sub(x87_load32(u), x87_m32(factor.to_bits())),
                    one_minus_factor,
                ),
            ));
            let w = if x87_load32(w) < 1.0 {
                w
            } else {
                1.0f32.to_bits()
            };
            e.mem
                .set_u16(first_level, (u32::from(level).wrapping_sub(1)) as u16);
            let blend_first = blend_alpha(e, w, exponent, alpha);
            e.mem.set_u32(first_value, blend_first);
            let x = x87_store32(x87_div(
                x87_load32(u),
                x87_sub(1.0, x87_load32(factor.to_bits())),
            ));
            let x = if x87_load32(x) < 1.0 {
                x
            } else {
                1.0f32.to_bits()
            };
            e.mem.set_u16(second_level, level);
            let blend_second = blend_alpha(e, x, exponent, alpha);
            e.mem.set_u32(second_value, blend_second);
            return;
        }
    }
    // One level alone.
    e.mem.set_u32(first_value, x87_quiet_bits(alpha.to_bits()));
    let scaled = x87_mul(x87_sub(1.0, lod_value), f64::from(levels as i32));
    let chosen = ftol(e, scaled) as u16;
    let last = (levels.wrapping_sub(1)) as u16;
    let capped = if i32::from(chosen as i16) < i32::from(last as i16) {
        chosen
    } else {
        last
    };
    e.mem.set_u16(first_level, capped);
    e.mem.set_u32(second_value, pushed_float(e, 0x0102_3cd8));
    e.mem.set_u16(second_level, 0xFFFF);
}

/// `pow(1 - t, exponent) * (255 - alpha) + alpha` as `GetTransitionValues` forms a blend alpha
/// (`t` the float's bits; the power is rounded to a float, `255` is the float at `01023cd8`);
/// returns the float's bits.
fn blend_alpha(e: &mut Engine, t: u32, exponent: f32, alpha: f32) -> u32 {
    let base = x87_store32(x87_sub(1.0, x87_load32(t)));
    let power = e
        .call(
            POW,
            &args![
                f64::from(f32::from_bits(base)),
                x87_load32(exponent.to_bits())
            ],
        )
        .f64();
    let power = x87_store32(power);
    let full = x87_sub(operand(e, 0x0102_3cd8), x87_load32(alpha.to_bits()));
    x87_store32(x87_add(
        x87_mul(x87_load32(power), full),
        x87_m32(alpha.to_bits()),
    ))
}

// Translated from 00b0cb10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0cb10 - the FS:[0] exception frame is not translated.
/// The leaf lod size factors (unnamed in the engine map): made on first use, one float per leaf
/// level (`00b08710`) in a block of `levels * 4` bytes (a product that overflows becomes
/// 0xFFFFFFFF) at `+0x2C`, entry `i` being `i * [this+0x24] + 1` (computed in a `double`, stored
/// as a float). Returns the table.
pub fn fn_00b0cb10(e: &mut Engine, this: Ptr<CSpeedTreeRT>) -> u32 {
    if e.get(this, CSpeedTreeRT::m_pLeafLodSizeFactors).is_null() {
        let levels = i32::from(fn_00b08710(e, this));
        let size = (levels as u32).checked_mul(4).unwrap_or(u32::MAX);
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        e.set(this, CSpeedTreeRT::m_pLeafLodSizeFactors, Ptr::new(block));
        let mut index = 0i32;
        while index < levels {
            let factor = if levels > 0 {
                x87_add(
                    x87_mul(f64::from(index), operand(e, this.addr() + 0x24)),
                    1.0,
                )
            } else {
                1.0
            };
            let table = e.get(this, CSpeedTreeRT::m_pLeafLodSizeFactors).addr();
            e.mem
                .set_u32(table.wrapping_add((index as u32) << 2), x87_store32(factor));
            index += 1;
        }
    }
    e.get(this, CSpeedTreeRT::m_pLeafLodSizeFactors).addr()
}

// Translated from 00b0cd40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0cd40 - the FS:[0] exception frame is not translated.
/// Reads the projected shadow chunk (unnamed in the engine map): makes the `CProjectedShadow`
/// (0x40 bytes, constructor `00b3e6c0`) at `+0x50` and lets it parse `access` (`00b3e750`).
pub fn fn_00b0cd40(e: &mut Engine, this: Ptr<CSpeedTreeRT>, access: Ptr) {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x40u32]).u32();
    let shadow = if block != 0 {
        e.call(0x00b3_e6c0, &args![block]).u32()
    } else {
        0
    };
    e.set(this, CSpeedTreeRT::m_pProjectedShadow, Ptr::new(shadow));
    let shadow = e.get(this, CSpeedTreeRT::m_pProjectedShadow);
    e.call(0x00b3_e750, &args![shadow, access]);
}

// Translated from 00b0cdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CSpeedTreeRT::ComputeSelfShadowTexCoords` (Xbox PDB): with branch geometry, frond geometry
/// and a projected shadow, takes the box of the tree sizes (`+0x40`: minimum, then maximum),
/// the radius `Distance(minimum, maximum) * [01011588]` and the centre `(minimum + maximum) *
/// [01016248]`, and lets the projected shadow (`00b3ea10`) compute the shadow texture
/// coordinates for the branch geometry and for the frond geometry from the centre and the
/// radius into the embedded coordinates' `+0x34` floats (null when the tree has none).
pub fn c_speed_tree_rt_compute_self_shadow_tex_coords(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    if e.get(this, CSpeedTreeRT::m_pBranchGeometry).is_null()
        || e.get(this, CSpeedTreeRT::m_pFrondGeometry).is_null()
        || e.get(this, CSpeedTreeRT::m_pProjectedShadow).is_null()
    {
        return;
    }
    // minimum, maximum, the sum and the centre: four vectors of 12 bytes.
    e.with_stack(0x30, |e, frame| {
        let (minimum, maximum) = (frame.addr(), frame.addr() + 0xC);
        let (sum, centre) = (frame.addr() + 0x18, frame.addr() + 0x24);
        let sizes = e.get(this, CSpeedTreeRT::m_pTreeSizes).addr();
        for axis in 0..3u32 {
            copy_float(e, minimum + 4 * axis, sizes + 4 * axis);
            copy_float(e, maximum + 4 * axis, sizes + 0xC + 4 * axis);
        }
        let distance = c_vec3_distance(e, Ptr::new(minimum), Ptr::new(maximum));
        let half: f64 = e.global(0x0101_1588);
        let radius = x87_store32(x87_mul(f64::from(distance), half));
        let factor = f32::from_bits(pushed_float(e, 0x0101_6248));
        let total = fn_00b0cf90(e, Ptr::new(minimum), Ptr::new(sum), Ptr::new(maximum));
        fn_00b0cff0(e, total, Ptr::new(centre), factor);
        let target = {
            let embedded = e.get(this, CSpeedTreeRT::m_pEmbeddedTexCoords);
            if !embedded.is_null() {
                embedded.addr() + 0x34
            } else {
                0
            }
        };
        for geometry in [
            CSpeedTreeRT::m_pBranchGeometry,
            CSpeedTreeRT::m_pFrondGeometry,
        ] {
            let shadow = e.get(this, CSpeedTreeRT::m_pProjectedShadow);
            let geometry = e.get(this, geometry);
            let centre_words: [u32; 3] = std::array::from_fn(|i| e.mem.u32(centre + 4 * i as u32));
            e.call(
                0x00b3_ea10,
                &args![
                    shadow,
                    geometry,
                    centre_words[0],
                    centre_words[1],
                    centre_words[2],
                    radius,
                    target
                ],
            );
        }
    });
}

// Translated from 00b0d040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0d040 - the FS:[0] exception frame is not translated.
/// Fills the texture names record `out` (unnamed in the engine map): `+0` the engine's name
/// string's text (`CTreeEngine` `+0x24`), `+4` the number of leaf textures (the 0x54-byte
/// entries of the leaf info's vector at `CTreeEngine` `+0xA4`) and `+8` an array of their file
/// name texts (`entry +0x14`; null with none), `+0xC` the number of frond textures (the
/// 0x2C-byte entries of the frond engine's vector at `+0x50`) and `+0x10` an array of their
/// names (`00b0d490`), `+0x18` the projected shadow's map name (`+0x24` of it) and `+0x14` the
/// composite file name (`+0x18` of the embedded coordinates), the last two only when the
/// tree has those.
pub fn fn_00b0d040(e: &mut Engine, this: Ptr<CSpeedTreeRT>, out: u32) {
    let engine = e.mem.u32(this.addr());
    let name = string_text(e, engine + 0x24);
    e.mem.set_u32(out, name);
    let leaf_vector = engine + 0xA4;
    let leaf_count = vector_len(e, leaf_vector, 0x54);
    e.mem.set_u32(out + 4, leaf_count);
    if e.mem.u32(out + 4) > 0 {
        let size = e.mem.u32(out + 4).checked_mul(4).unwrap_or(u32::MAX);
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        e.mem.set_u32(out + 8, block);
        let mut index = 0u32;
        while index < vector_len(e, leaf_vector, 0x54) {
            let entry = vector_at(e, leaf_vector, index, 0x54);
            let text = string_text(e, entry + 0x14);
            let names = e.mem.u32(out + 8);
            e.mem.set_u32(names.wrapping_add(index << 2), text);
            index += 1;
        }
    } else {
        e.mem.set_u32(out + 8, 0);
    }
    let frond_engine = e.get(this, CSpeedTreeRT::m_pFrondEngine).addr();
    let frond_count = vector_len(e, frond_engine + 0x50, 0x2C);
    e.mem.set_u32(out + 0xC, frond_count);
    if e.mem.u32(out + 0xC) > 0 {
        let size = e.mem.u32(out + 0xC).checked_mul(4).unwrap_or(u32::MAX);
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        e.mem.set_u32(out + 0x10, block);
        let mut index = 0u32;
        while index < e.mem.u32(out + 0xC) {
            let text = fn_00b0d490(e, Ptr::new(frond_engine), index);
            let names = e.mem.u32(out + 0x10);
            e.mem.set_u32(names.wrapping_add(index << 2), text);
            index += 1;
        }
    } else {
        e.mem.set_u32(out + 0x10, 0);
    }
    let shadow = e.get(this, CSpeedTreeRT::m_pProjectedShadow);
    if !shadow.is_null() {
        let text = string_text(e, shadow.addr() + 0x24);
        e.mem.set_u32(out + 0x18, text);
    }
    let embedded = e.get(this, CSpeedTreeRT::m_pEmbeddedTexCoords);
    if !embedded.is_null() {
        let text = string_text(e, embedded.addr() + 0x18);
        e.mem.set_u32(out + 0x14, text);
    }
}

// Translated from 00b0d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The file name text of frond texture `index` (unnamed in the engine map; `this` the frond
/// engine): the string at the start of the entry (0x2C bytes) of the vector at `+0x50`, through
/// the checked index.
pub fn fn_00b0d490(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let entry = vector_at(e, this.addr() + 0x50, index, 0x2C);
    string_text(e, entry)
}

// Translated from 00b0d500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0d500 - the FS:[0] exception frame (destroying the string when
// a step throws) is not translated.
/// `CSpeedTreeRT::ParseUserData` (Xbox PDB): reads entries until the tag 19001; the tag 19002
/// is a string whose text is copied to a new block (`00b0d7c0`) kept at `+0x68`; any other tag
/// is skipped.
pub fn c_speed_tree_rt_parse_user_data(e: &mut Engine, this: Ptr<CSpeedTreeRT>, access: Ptr) {
    let mut tag = parse_int(e, access);
    loop {
        if tag == 0x4A3A {
            e.with_stack(0x1C, |e, text| {
                e.call(PARSE_STRING, &args![access, text]);
                let characters = string_text(e, text.addr());
                let copy = fn_00b0d7c0(e, characters);
                e.set(this, CSpeedTreeRT::m_pUserData, Ptr::new(copy));
                e.call(STRING_TIDY, &args![text, 1u32, 0u32]);
            });
        }
        tag = parse_int(e, access);
        if tag == 0x4A39 {
            break;
        }
    }
}

// Translated from 00b0d5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The horizontal billboard's corners (unnamed in the engine map): for a tree with a horizontal
/// billboard (`+0x6D`) the four corners of the tree sizes' x/y rectangle, at the height
/// `(z0 + z1) * [01011588]`, go to the 12 floats at `+0x70` in the order `(x1, y1)`, `(x0, y1)`,
/// `(x0, y0)`, `(x1, y0)` (each float copied through the x87 stack).
pub fn fn_00b0d5f0(e: &mut Engine, this: Ptr<CSpeedTreeRT>) {
    if e.get(this, CSpeedTreeRT::m_bHorizontalBillboard) == 0 {
        return;
    }
    let sizes = e.get(this, CSpeedTreeRT::m_pTreeSizes).addr();
    let quiet = |e: &Engine, offset: u32| pushed_float(e, sizes + offset);
    let (x0, y0, z0) = (quiet(e, 0), quiet(e, 4), quiet(e, 8));
    let (x1, y1, z1) = (quiet(e, 0xC), quiet(e, 0x10), quiet(e, 0x14));
    let half: f64 = e.global(0x0101_1588);
    let height = x87_store32(x87_mul(x87_add(x87_load32(z0), x87_m32(z1)), half));
    let corners = [(x1, y1), (x0, y1), (x0, y0), (x1, y0)];
    for (corner, (x, y)) in corners.into_iter().enumerate() {
        let base = this.addr() + 0x70 + 12 * corner as u32;
        e.mem.set_u32(base, x87_quiet_bits(x));
        e.mem.set_u32(base + 4, x87_quiet_bits(y));
        e.mem.set_u32(base + 8, x87_quiet_bits(height));
    }
}

// Translated from 00b0d7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A copy of the C string `text` in a new block (`cdecl`; unnamed in the engine map): the length
/// from `strlen`, a block of length + 1 bytes from the memory manager, `memcpy` of the
/// terminator too. Returns the block.
pub fn fn_00b0d7c0(e: &mut Engine, text: u32) -> u32 {
    let length = e.call(STRLEN, &args![text]).u32();
    let block = e
        .call(ALLOCATE, &args![MEMORY_MANAGER, length.wrapping_add(1)])
        .u32();
    e.call(MEMCPY, &args![block, text, length.wrapping_add(1)]);
    block
}

/// `std::string::string(const string &)` (`0044a500`, `this` = the new string, the source).
const STRING_COPY: u32 = 0x0044_a500;

// Translated from 00b0d810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0d810 - the FS:[0] exception frame and the /GS cookie check
// (00ec408c) are not translated.
/// Reads the composite-map chunk (unnamed in the engine map) into the embedded texture
/// coordinates (`+0x4C`; nothing is read when the tree has none) until the tag 20001: 20002
/// the composite file name (a string copied, reduced to its file name by `NoPath` and assigned
/// to the string at `+0x18` of the embedded coordinates), 20003 and 20004 the bytes at `+0x6D`
/// and `+0x6C`, 20005 the eight shadow coordinates (`+0x34`); any other tag throws "malformed
/// texture coord info" and running out of data throws "premature end of file".
pub fn fn_00b0d810(e: &mut Engine, this: Ptr<CSpeedTreeRT>, access: Ptr) {
    if e.get(this, CSpeedTreeRT::m_pEmbeddedTexCoords).is_null() {
        return;
    }
    let mut tag = parse_int(e, access);
    loop {
        match tag.wrapping_sub(0x4E22) as u32 {
            0 => {
                // The by-value string argument, its copy and the reduced name.
                e.with_stack(0x54, |e, frame| {
                    let (argument, copy, reduced) =
                        (frame.addr(), frame.addr() + 0x1C, frame.addr() + 0x38);
                    e.call(PARSE_STRING, &args![access, argument]);
                    e.call(STRING_COPY, &args![copy, argument]);
                    e.call(STRING_TIDY, &args![argument, 1u32, 0u32]);
                    let reduced = c_idv_filename_no_path(e, Ptr::new(copy), Ptr::new(reduced));
                    let target = e.get(this, CSpeedTreeRT::m_pEmbeddedTexCoords).addr() + 0x18;
                    let npos = e.global::<u32>(STRING_NPOS);
                    e.call(STRING_ASSIGN, &args![target, reduced, 0u32, npos]);
                    e.call(STRING_TIDY, &args![reduced, 1u32, 0u32]);
                    e.call(STRING_TIDY, &args![copy, 1u32, 0u32]);
                });
            }
            1 => {
                let flag = fn_00b06180(e, access);
                e.mem.set_u8(this.addr() + 0x6D, flag as u8);
            }
            2 => {
                let flag = fn_00b06180(e, access);
                e.mem.set_u8(this.addr() + 0x6C, flag as u8);
            }
            3 => {
                for index in 0..8u32 {
                    let value = e.call(PARSE_FLOAT, &args![access]).f64();
                    let embedded = e.get(this, CSpeedTreeRT::m_pEmbeddedTexCoords).addr();
                    fstp(e, embedded + 0x34 + 4 * index, value);
                }
            }
            _ => throw_parse_error(e, 0x010a_5278),
        }
        if file_ended(e, access) {
            throw_parse_error(e, 0x010a_5238);
        }
        tag = parse_int(e, access);
        if tag == 0x4E21 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b036d0,
            fn_00b036d0(Ptr<CSpeedTreeRT>, Ptr<CSpeedTreeRT>) -> Ptr<CSpeedTreeRT>
        ),
        entry!(
            0x00b044a0,
            c_speed_tree_rt_compute(Ptr<CSpeedTreeRT>, Ptr, u32, bool) -> u8
        ),
        entry!(0x00b04d40, c_vec3_distance(Ptr<CVec3>, Ptr<CVec3>) -> f32),
        entry!(0x00b04db0, fn_00b04db0(Ptr, Ptr)),
        entry!(0x00b05000, fn_00b05000(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00b050d0, fn_00b050d0(Ptr)),
        entry!(0x00b05130, fn_00b05130(Ptr) -> Ptr),
        entry!(
            0x00b051e0,
            fn_00b051e0(Ptr<CSpeedTreeRT>) -> Ptr<CSpeedTreeRT>
        ),
        entry!(
            0x00b05210,
            fn_00b05210(Ptr<CSpeedTreeRT>) -> Ptr<CSpeedTreeRT>
        ),
        entry!(
            0x00b053e0,
            c_speed_tree_rt_delete_transient_data(Ptr<CSpeedTreeRT>)
        ),
        entry!(
            0x00b05420,
            c_speed_tree_rt_load_tree(Ptr<CSpeedTreeRT>, u32) -> u8
        ),
        entry!(0x00b059e0, fn_00b059e0(Ptr<CSpeedTreeRT>, u32, u32) -> u8),
        entry!(0x00b06090, fn_00b06090(Ptr, u32, i32) -> Ptr),
        entry!(0x00b06180, fn_00b06180(Ptr) -> bool),
        entry!(0x00b061e0, fn_00b061e0(Ptr)),
        entry!(0x00b06260, fn_00b06260(Ptr<CSpeedTreeRT>, u32, u32)),
        entry!(
            0x00b06280,
            c_speed_tree_rt_set_tree_size(Ptr<CSpeedTreeRT>, f32, f32)
        ),
        entry!(0x00b06440, fn_00b06440(Ptr<CSpeedTreeRT>) -> u32),
        entry!(0x00b065e0, fn_00b065e0(Ptr<CSpeedTreeRT>, f32, f32, f32)),
        entry!(
            0x00b06660,
            c_speed_tree_rt_set_branch_lighting_method(Ptr<CSpeedTreeRT>, u32)
        ),
        entry!(
            0x00b06810,
            c_speed_tree_rt_set_leaf_lighting_method(Ptr<CSpeedTreeRT>, u32)
        ),
        entry!(
            0x00b069c0,
            c_speed_tree_rt_set_frond_lighting_method(Ptr<CSpeedTreeRT>, u32)
        ),
        entry!(0x00b06b70, c_speed_tree_rt_get_camera(u32, u32)),
        entry!(0x00b06c10, c_speed_tree_rt_set_camera(u32, u32)),
        entry!(0x00b07060, fn_00b07060(f32)),
        entry!(0x00b071c0, fn_00b071c0(Ptr<CSpeedTreeRT>, u8)),
        entry!(
            0x00b071f0,
            c_speed_tree_rt_set_num_leaf_rocking_groups(Ptr<CSpeedTreeRT>, u32)
        ),
        entry!(0x00b07390, fn_00b07390(Ptr<CSpeedTreeRT>, f32)),
        entry!(0x00b073c0, fn_00b073c0(Ptr<CSpeedTreeRT>, f32)),
        entry!(0x00b073f0, fn_00b073f0(Ptr<CSpeedTreeRT>) -> f32),
        entry!(0x00b07440, fn_00b07440(Ptr<CSpeedTreeRT>) -> f32),
        entry!(0x00b07490, fn_00b07490(Ptr<CSpeedTreeRT>, f32)),
        entry!(0x00b074c0, fn_00b074c0(Ptr<CSpeedTreeRT>, f32)),
        entry!(0x00b074f0, fn_00b074f0(Ptr<CSpeedTreeRT>) -> u32),
        entry!(
            0x00b07510,
            c_speed_tree_rt_set_leaf_wind_method(Ptr<CSpeedTreeRT>, u32)
        ),
        entry!(0x00b076c0, fn_00b076c0(Ptr<CSpeedTreeRT>) -> u32),
        entry!(
            0x00b076e0,
            c_speed_tree_rt_set_branch_wind_method(Ptr<CSpeedTreeRT>, u32)
        ),
        entry!(0x00b078e0, fn_00b078e0(Ptr<CSpeedTreeRT>) -> u32),
        entry!(
            0x00b07900,
            c_speed_tree_rt_set_frond_wind_method(Ptr<CSpeedTreeRT>, u32)
        ),
        entry!(
            0x00b07b00,
            c_speed_tree_rt_set_wind_strength(Ptr<CSpeedTreeRT>, f32, f32, f32) -> f32
        ),
        entry!(0x00b07db0, fn_00b07db0(u16)),
        entry!(0x00b07f20, fn_00b07f20(Ptr, u16)),
        entry!(
            0x00b08050,
            c_speed_tree_rt_set_local_matrices(Ptr<CSpeedTreeRT>, u32, u32)
        ),
        entry!(
            0x00b08200,
            c_speed_tree_rt_compute_lod_level(Ptr<CSpeedTreeRT>)
        ),
        entry!(
            0x00b08440,
            c_speed_tree_rt_get_lod_level(Ptr<CSpeedTreeRT>) -> f32
        ),
        entry!(
            0x00b08480,
            c_speed_tree_rt_set_lod_level(Ptr<CSpeedTreeRT>, f32)
        ),
        entry!(0x00b084f0, fn_00b084f0(u32)),
        entry!(0x00b08500, fn_00b08500(Ptr<CSpeedTreeRT>, f32, f32)),
        entry!(
            0x00b08530,
            c_speed_tree_rt_get_discrete_branch_lod_level(Ptr<CSpeedTreeRT>, f32) -> u16
        ),
        entry!(
            0x00b085a0,
            c_speed_tree_rt_get_discrete_frond_lod_level(Ptr<CSpeedTreeRT>, f32) -> u16
        ),
        entry!(0x00b08610, fn_00b08610(Ptr<CSpeedTreeRT>, f32) -> u16),
        entry!(
            0x00b086f0,
            c_speed_tree_rt_get_num_branch_lod_levels(Ptr<CSpeedTreeRT>) -> u16
        ),
        entry!(0x00b08710, fn_00b08710(Ptr<CSpeedTreeRT>) -> u16),
        entry!(0x00b08730, fn_00b08730(Ptr<CSpeedTreeRT>) -> u16),
        entry!(0x00b08750, fn_00b08750(Ptr<CSpeedTreeRT>)),
        entry!(0x00b08780, fn_00b08780(Ptr<CSpeedTreeRT>)),
        entry!(0x00b087d0, fn_00b087d0(Ptr<CSpeedTreeRT>)),
        entry!(0x00b088b0, fn_00b088b0(Ptr<CSpeedTreeRT>)),
        entry!(0x00b088d0, fn_00b088d0(Ptr<CSpeedTreeRT>)),
        entry!(0x00b08af0, fn_00b08af0(Ptr<CSpeedTreeRT>, u32) -> u32),
        entry!(
            0x00b08c80,
            c_speed_tree_rt_get_bounding_box(Ptr<CSpeedTreeRT>, u32)
        ),
        entry!(0x00b08cc0, fn_00b08cc0(Ptr<CSpeedTreeRT>)),
        entry!(0x00b08ec0, fn_00b08ec0(i32)),
        entry!(0x00b09270, c_speed_tree_rt_set_error(u32)),
        entry!(
            0x00b092a0,
            c_speed_tree_rt_parse_lod_info(Ptr<CSpeedTreeRT>, Ptr)
        ),
        entry!(0x00b09480, fn_00b09480(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00b096a0, fn_00b096a0(Ptr, Ptr) -> Ptr),
        entry!(0x00b09730, fn_00b09730(Ptr, u32) -> Ptr),
        entry!(0x00b09780, fn_00b09780(Ptr)),
        entry!(0x00b097b0, fn_00b097b0(Ptr, Ptr) -> Ptr),
        entry!(
            0x00b097e0,
            c_speed_tree_rt_parse_wind_info(Ptr<CSpeedTreeRT>, Ptr)
        ),
        entry!(0x00b09960, fn_00b09960(Ptr<CSpeedTreeRT>, Ptr)),
        entry!(0x00b09ef0, fn_00b09ef0(Ptr, u32, u32, u32, u32, u32, u32, u32) -> Ptr),
        entry!(0x00b09f70, fn_00b09f70(Ptr)),
        entry!(0x00b09f90, c_idv_filename_no_path(Ptr, Ptr) -> Ptr),
        entry!(0x00b0a260, fn_00b0a260(Ptr<CSpeedTreeRT>) -> i32),
        entry!(
            0x00b0a2a0,
            c_speed_tree_rt_get_collision_object(Ptr<CSpeedTreeRT>, u32, u32, u32, u32)
        ),
        entry!(
            0x00b0a490,
            c_speed_tree_rt_parse_collision_objects(Ptr<CSpeedTreeRT>, Ptr)
        ),
        entry!(0x00b0a7a0, fn_00b0a7a0(Ptr<CSpeedTreeRT>)),
        entry!(0x00b0a9a0, fn_00b0a9a0(Ptr, i16) -> u16),
        entry!(0x00b0aa20, fn_00b0aa20(Ptr, i16) -> u32),
        entry!(
            0x00b0ab20,
            fn_00b0ab20(Ptr<CSpeedTreeRT>, u32, u32, u16, u16, u16)
        ),
        entry!(0x00b0ad00, fn_00b0ad00(Ptr<CSpeedTreeRT>, u32, f32)),
        entry!(
            0x00b0b1d0,
            c_speed_tree_rt_get_branch_geometry(Ptr<CSpeedTreeRT>, u32, i16)
        ),
        entry!(0x00b0b4b0, fn_00b0b4b0(Ptr) -> u32),
        entry!(0x00b0b530, fn_00b0b530(Ptr) -> u32),
        entry!(0x00b0b5b0, fn_00b0b5b0(Ptr) -> u32),
        entry!(0x00b0b630, fn_00b0b630(Ptr) -> u32),
        entry!(0x00b0b6b0, fn_00b0b6b0(Ptr) -> u32),
        entry!(0x00b0b730, fn_00b0b730(Ptr) -> u32),
        entry!(0x00b0b7b0, fn_00b0b7b0(Ptr) -> u32),
        entry!(0x00b0b830, fn_00b0b830(Ptr) -> u32),
        entry!(0x00b0b8b0, fn_00b0b8b0(Ptr) -> u32),
        entry!(
            0x00b0b920,
            c_speed_tree_rt_get_frond_geometry(Ptr<CSpeedTreeRT>, u32, i16)
        ),
        entry!(0x00b0bc00, fn_00b0bc00(Ptr<CSpeedTreeRT>, u32, i16)),
        entry!(0x00b0c050, fn_00b0c050(Ptr<CSpeedTreeRT>, u32, u32)),
        entry!(0x00b0c470, fn_00b0c470(Ptr) -> f32),
        entry!(0x00b0c4f0, fn_00b0c4f0(Ptr<CSpeedTreeRT>, u32)),
        entry!(0x00b0c780, fn_00b0c780(Ptr, f32, f32) -> Ptr),
        entry!(
            0x00b0c8d0,
            c_speed_tree_rt_get_transition_values(f32, u32, f32, f32, f32, f32, u32, u32, u32, u32)
        ),
        entry!(0x00b0cb10, fn_00b0cb10(Ptr<CSpeedTreeRT>) -> u32),
        entry!(0x00b0cd40, fn_00b0cd40(Ptr<CSpeedTreeRT>, Ptr)),
        entry!(
            0x00b0cdd0,
            c_speed_tree_rt_compute_self_shadow_tex_coords(Ptr<CSpeedTreeRT>)
        ),
        entry!(0x00b0cf90, fn_00b0cf90(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00b0cff0, fn_00b0cff0(Ptr, Ptr, f32) -> Ptr),
        entry!(0x00b0d040, fn_00b0d040(Ptr<CSpeedTreeRT>, u32)),
        entry!(0x00b0d490, fn_00b0d490(Ptr, u32) -> u32),
        entry!(
            0x00b0d500,
            c_speed_tree_rt_parse_user_data(Ptr<CSpeedTreeRT>, Ptr)
        ),
        entry!(0x00b0d5f0, fn_00b0d5f0(Ptr<CSpeedTreeRT>)),
        entry!(0x00b0d7c0, fn_00b0d7c0(u32) -> u32),
        entry!(0x00b0d810, fn_00b0d810(Ptr<CSpeedTreeRT>, Ptr)),
        entry!(0x00b0dc10, fn_00b0dc10(Ptr)),
        entry!(0x00b0dfb0, fn_00b0dfb0(Ptr) -> Ptr),
        entry!(0x00b0e090, fn_00b0e090(Ptr, Ptr)),
        entry!(0x00b0eb30, fn_00b0eb30(Ptr) -> Ptr),
        entry!(0x00b0ebb0, fn_00b0ebb0(Ptr) -> u32),
        entry!(0x00b0ec20, fn_00b0ec20(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with a recording double (returning 0) at each of `addresses`.
    fn recording(addresses: &[u32]) -> (Engine, Calls) {
        let mut e = Engine::new();
        let calls: Calls = Rc::default();
        for &address in addresses {
            stub(&mut e, &calls, address, 0);
        }
        (e, calls)
    }

    /// A recording double at `address` that returns `eax`.
    fn stub(e: &mut Engine, calls: &Calls, address: u32, eax: u32) {
        let log = calls.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    /// A recording double at `address` that returns the float `st0`.
    fn stub_float(e: &mut Engine, calls: &Calls, address: u32, st0: f64) {
        let log = calls.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                st0,
                ..Ret::default()
            }
        });
    }

    /// The argument words of every call recorded for `address`.
    fn calls_to(calls: &Calls, address: u32) -> Vec<Vec<u32>> {
        calls
            .borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// `memcpy(destination, source, count)`.
    fn memcpy_double(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.bytes(a[1], a[2]);
        e.mem.write(a[0], &bytes);
        Ret::default()
    }

    /// A global word at `address`.
    fn global(e: &mut Engine, address: u32, bits: u32) {
        e.map(address, 4);
        e.mem.set_u32(address, bits);
    }

    fn global_f32(e: &mut Engine, address: u32, value: f32) {
        global(e, address, value.to_bits());
    }

    fn global_f64(e: &mut Engine, address: u32, value: f64) {
        e.map(address, 8);
        e.mem.set_f64(address, value);
    }

    /// A tree whose parts are allocated (zeroed) and linked: the engine, the branch, leaf and
    /// frond geometries, the lighting engine, the wind engine, the frond engine, the tree
    /// sizes, the shared count and the instance list.
    fn linked_tree(e: &mut Engine) -> Ptr<CSpeedTreeRT> {
        let tree = e.new_object::<CSpeedTreeRT>();
        for (field, size) in [
            (CSpeedTreeRT::m_pEngine, 0x200),
            (CSpeedTreeRT::m_pBranchGeometry, 0x200),
            (CSpeedTreeRT::m_pLeafGeometry, 0x80),
            (CSpeedTreeRT::m_pLightingEngine, 0xB0),
            (CSpeedTreeRT::m_pWindEngine, 0x44),
            (CSpeedTreeRT::m_pFrondEngine, 0x80),
            (CSpeedTreeRT::m_pFrondGeometry, 0x200),
            (CSpeedTreeRT::m_pTreeSizes, 0x20),
            (CSpeedTreeRT::m_pInstanceRefCount, 4),
            (CSpeedTreeRT::m_pInstanceList, 0x10),
        ] {
            let block = e.mem.alloc(size);
            e.set(tree, field, Ptr::new(block));
        }
        tree
    }

    /// The address a pointer field of the tree holds.
    fn part(e: &Engine, tree: Ptr<CSpeedTreeRT>, field: Field<CSpeedTreeRT, Ptr>) -> u32 {
        e.get(tree, field).addr()
    }

    /// A `std::vector` (0x18 bytes: proxy, first, last, end) at `vector` over `bytes` bytes of
    /// fresh storage; the proxy's first word is the vector. Returns the storage.
    fn make_vector(e: &mut Engine, vector: u32, bytes: u32) -> u32 {
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, vector);
        let storage = e.mem.alloc(bytes.max(8));
        e.mem.set_u32(vector, proxy);
        e.mem.set_u32(vector + 0x0C, storage);
        e.mem.set_u32(vector + 0x10, storage + bytes);
        e.mem.set_u32(vector + 0x14, storage + bytes);
        storage
    }

    /// The checked-iterator constructor and comparison as doubles: an iterator is the vector's
    /// proxy and the position.
    fn iterator_doubles(e: &mut Engine) {
        e.register_double(ITERATOR_CONSTRUCT, |e, a| {
            let proxy = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], proxy);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register_double(ITERATOR_EQUAL, |e, a| {
            let equal = e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4);
            Ret {
                eax: equal as u32,
                ..Ret::default()
            }
        });
    }

    /// The doubles `SetError` calls (`strlen` of 5 and the string assign), recorded.
    fn error_doubles(e: &mut Engine, calls: &Calls) {
        stub(e, calls, STRLEN, 5);
        stub(e, calls, STRING_ASSIGN_TEXT, 0);
    }

    /// Whether `SetError` was told `message`.
    fn error_was(calls: &Calls, message: u32) -> bool {
        calls_to(calls, STRING_ASSIGN_TEXT)
            .iter()
            .any(|a| a[0] == ERROR_STRING && a[1] == message)
    }

    #[test]
    fn instance_copies_a_tree_and_counts_it() {
        let (mut e, calls) = recording(&[]);
        e.register(MEMCPY, memcpy_double);
        let log = calls.clone();
        e.register_double(0x00b0_ddc0, move |e, a| {
            log.borrow_mut()
                .push((0x00b0_ddc0, vec![a[0], e.mem.u32(a[1])]));
            Ret::default()
        });
        global(&mut e, ALL_REF_COUNT, 7);
        global_f32(&mut e, 0x0101_2054, -1.0);
        let source = linked_tree(&mut e);
        e.set(source, CSpeedTreeRT::m_eLeafLodMethod, 3);
        e.set(source, CSpeedTreeRT::m_fLeafLodTransitionRadius, 1.5);
        e.set(source, CSpeedTreeRT::m_usNum360Billboards, 6);
        e.set(source, CSpeedTreeRT::m_b360Billboard, 1);
        e.set(source, CSpeedTreeRT::m_nBranchWindLevel, -2);
        e.mem.set_f32(source.addr() + 0x78, 4.5);
        let count = part(&e, source, CSpeedTreeRT::m_pInstanceRefCount);
        e.mem.set_u32(count, 1);
        let engine = part(&e, source, CSpeedTreeRT::m_pEngine);
        for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(engine + 4 + 4 * i as u32, value);
        }
        e.mem.set_f32(engine + 0x14, 0.25);
        let copy = e.new_object::<CSpeedTreeRT>();
        assert_eq!(e.call(0x00b0_36d0, &args![copy, source]).u32(), copy.addr());
        for field in [
            CSpeedTreeRT::m_pEngine,
            CSpeedTreeRT::m_pBranchGeometry,
            CSpeedTreeRT::m_pFrondGeometry,
            CSpeedTreeRT::m_pTreeSizes,
            CSpeedTreeRT::m_pInstanceRefCount,
            CSpeedTreeRT::m_pInstanceList,
        ] {
            assert_eq!(e.get(copy, field), e.get(source, field));
        }
        assert_eq!(e.get(copy, CSpeedTreeRT::m_eLeafLodMethod), 3);
        assert_eq!(e.get(copy, CSpeedTreeRT::m_fLeafLodTransitionRadius), 1.5);
        assert_eq!(e.get(copy, CSpeedTreeRT::m_usNum360Billboards), 6);
        assert_eq!(e.get(copy, CSpeedTreeRT::m_b360Billboard), 1);
        assert_eq!(e.get(copy, CSpeedTreeRT::m_nBranchWindLevel), -2);
        assert_eq!(e.mem.f32(copy.addr() + 0x78), 4.5);
        assert_eq!(e.mem.u32(count), 2);
        assert_eq!(e.global::<u32>(ALL_REF_COUNT), 8);
        let list = part(&e, source, CSpeedTreeRT::m_pInstanceList);
        assert_eq!(calls_to(&calls, 0x00b0_ddc0), vec![vec![list, copy.addr()]]);
        let data = e.get(copy, CSpeedTreeRT::m_pInstanceData);
        assert!(!data.is_null());
        assert_eq!(e.get(data, STreeInstanceData::m_pParent), source);
        assert_eq!(
            [4, 8, 12].map(|offset| e.mem.f32(data.addr() + offset)),
            [1.0, 2.0, 3.0]
        );
        assert_eq!(e.mem.f32(data.addr() + 0x10), 0.25);
    }

    #[test]
    fn instance_takes_the_source_instance_level() {
        let (mut e, _) = recording(&[0x00b0_ddc0]);
        e.register(MEMCPY, memcpy_double);
        global(&mut e, ALL_REF_COUNT, 0);
        global_f32(&mut e, 0x0101_2054, -1.0);
        let source = linked_tree(&mut e);
        let data = e.mem.alloc(0x14);
        e.mem.set_f32(data + 0x10, 0.75);
        e.set(source, CSpeedTreeRT::m_pInstanceData, Ptr::new(data));
        let copy = e.new_object::<CSpeedTreeRT>();
        e.call(0x00b0_36d0, &args![copy, source]);
        let made = e.get(copy, CSpeedTreeRT::m_pInstanceData);
        assert_eq!(e.mem.f32(made.addr() + 0x10), 0.75);
    }

    /// Every callee of `Compute`, as a recording double; the extents the geometries fill are
    /// the box (1, 2, 3) to (4, 5, 6).
    fn compute_doubles(e: &mut Engine, calls: &Calls) {
        for address in [
            0x00b2_c0c0,
            0x00b2_af80,
            0x00b1_8fd0,
            0x00b3_cf40,
            0x00b3_7510,
            0x00b2_8480,
            0x00b2_8720,
            0x00b1_c6d0,
            0x00b2_ed70,
            0x00b3_ccb0,
            0x00b2_fc50,
            0x00b2_8280,
            0x00b3_c930,
            0x00b2_7910,
            TRIVIAL_DESTRUCTOR,
        ] {
            stub(e, calls, address, 0);
        }
        let log = calls.clone();
        e.register_double(0x00b2_fa30, move |e, a| {
            log.borrow_mut().push((0x00b2_fa30, a.to_vec()));
            for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, value);
            }
            for (i, value) in [4.0f32, 5.0, 6.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 0x18 + 4 * i as u32, value);
            }
            Ret::default()
        });
        e.register(MEMCPY, memcpy_double);
        global(e, CURRENT_FROND_ENGINE, 0);
        global(e, FROND_TEXTURE_FLAG, 1);
        error_doubles(e, calls);
    }

    #[test]
    fn compute_builds_the_tree_and_measures_it() {
        let (mut e, calls) = recording(&[]);
        compute_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        let frond_engine = part(&e, tree, CSpeedTreeRT::m_pFrondEngine);
        let frond_geometry = part(&e, tree, CSpeedTreeRT::m_pFrondGeometry);
        let lighting = part(&e, tree, CSpeedTreeRT::m_pLightingEngine);
        let leaf_geometry = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        e.mem.set_f32(tree.addr() + 0x24, 0.5);
        e.mem.set_u16(frond_geometry + 0x20, 5);
        // The leaf info's vector of 0x54-byte entries holds two.
        e.mem.set_u32(engine + 0xB0, 0x1000);
        e.mem.set_u32(engine + 0xB4, 0x1000 + 2 * 0x54);
        e.mem.set_u32(engine + 0xD8, 0x0003_0007);
        e.mem.set_u32(engine + 0xEC, 0x1234);
        assert_eq!(
            e.call(0x00b0_44a0, &args![tree, 0u32, 99u32, false]).u32(),
            1
        );
        assert_eq!(e.get(tree, CSpeedTreeRT::m_bTreeComputed), 1);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_usNumFrondLodLevels), 5);
        assert_eq!(
            e.global::<u32>(CURRENT_FROND_ENGINE),
            part(&e, tree, CSpeedTreeRT::m_pFrondEngine)
        );
        assert_eq!(calls_to(&calls, 0x00b2_c0c0), vec![vec![engine, 99]]);
        assert_eq!(
            calls_to(&calls, 0x00b2_af80),
            vec![vec![engine, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&calls, 0x00b1_8fd0),
            vec![vec![frond_engine, frond_geometry, lighting]]
        );
        assert_eq!(calls_to(&calls, 0x00b3_cf40), vec![vec![engine + 0x94, 2]]);
        assert_eq!(
            calls_to(&calls, 0x00b3_7510),
            vec![vec![wind, engine + 0x10C]]
        );
        assert_eq!(
            calls_to(&calls, 0x00b2_8480),
            vec![vec![leaf_geometry, 7, 0x1234, engine + 0x94]]
        );
        // No matrix, no flag, no embedded texture coordinates.
        assert!(calls_to(&calls, 0x00b2_fc50).is_empty());
        assert!(calls_to(&calls, 0x00b2_ed70).is_empty());
        assert!(calls_to(&calls, 0x00b2_8720).is_empty());
        // The sizes are the box and twice the largest corner distance from the origin.
        let sizes = part(&e, tree, CSpeedTreeRT::m_pTreeSizes);
        assert_eq!(
            [0, 4, 8, 12, 16, 20].map(|offset| e.mem.f32(sizes + offset)),
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        );
        let corner = e.mem.alloc(0x18);
        e.mem.set_f32(corner + 0xC, 4.0);
        e.mem.set_f32(corner + 0x10, 5.0);
        let farthest = c_vec3_distance(&mut e, Ptr::new(corner), Ptr::new(corner + 0xC));
        assert_eq!(e.mem.f32(sizes + 0x18), farthest + farthest);
        assert_eq!(calls_to(&calls, TRIVIAL_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn compute_applies_the_matrix_and_the_texture_sets() {
        let (mut e, calls) = recording(&[]);
        compute_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        let branch = part(&e, tree, CSpeedTreeRT::m_pBranchGeometry);
        let frond = part(&e, tree, CSpeedTreeRT::m_pFrondGeometry);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        let frond_engine = part(&e, tree, CSpeedTreeRT::m_pFrondEngine);
        let embedded = e.mem.alloc(0x54);
        let leaf_table = e.mem.alloc(0x40);
        let frond_table = e.mem.alloc(0x20);
        e.mem.set_i32(embedded, 2);
        e.mem.set_u32(embedded + 4, leaf_table);
        e.mem.set_i32(embedded + 0x10, 1);
        e.mem.set_u32(embedded + 0x14, frond_table);
        e.set(tree, CSpeedTreeRT::m_pEmbeddedTexCoords, Ptr::new(embedded));
        let matrix = e.mem.alloc(0x40);
        e.mem.set_f32(matrix, 2.0);
        assert_eq!(
            e.call(0x00b0_44a0, &args![tree, matrix, 5u32, true]).u32(),
            1
        );
        assert_eq!(
            calls_to(&calls, 0x00b2_8720),
            vec![vec![leaf, 0, leaf_table], vec![leaf, 1, leaf_table + 0x20]]
        );
        assert_eq!(
            calls_to(&calls, 0x00b1_c6d0),
            vec![vec![frond_engine, frond, 0, frond_table, 1]]
        );
        assert_eq!(
            calls_to(&calls, 0x00b2_ed70),
            vec![vec![branch, 0], vec![frond, 0]]
        );
        let matrix_calls = calls_to(&calls, 0x00b2_fc50);
        assert_eq!(matrix_calls.len(), 2);
        assert_eq!((matrix_calls[0][0], matrix_calls[1][0]), (branch, frond));
        // The geometries get a copy of the matrix; the leaf geometry too.
        assert_ne!(matrix_calls[0][1], matrix);
        assert_eq!(calls_to(&calls, 0x00b2_8280)[0][0], leaf);
        assert_eq!(calls_to(&calls, 0x00b3_ccb0).len(), 1);
    }

    #[test]
    fn compute_twice_reports_an_error() {
        let (mut e, calls) = recording(&[]);
        compute_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        e.set(tree, CSpeedTreeRT::m_bTreeComputed, 1);
        assert_eq!(
            e.call(0x00b0_44a0, &args![tree, 0u32, 1u32, false]).u32(),
            1
        );
        assert!(error_was(&calls, 0x010a_4998));
        assert!(calls_to(&calls, 0x00b2_c0c0).is_empty());
    }

    #[test]
    fn distance_uses_the_square_root_approximation() {
        let mut e = Engine::new();
        let vectors = e.mem.alloc(0x18);
        e.mem.set_f32(vectors + 0xC, 3.0);
        e.mem.set_f32(vectors + 0x10, 4.0);
        // 25 as float bits 0x41c80000: shifted right, plus 0x1fc00000 = 0x40a40000.
        let distance = e.call(0x00b0_4d40, &args![vectors, vectors + 0xC]);
        assert_eq!(distance.f32().to_bits(), 0x40a4_0000);
        // The order of the operands does not matter for the squares.
        let reversed = e.call(0x00b0_4d40, &args![vectors + 0xC, vectors]);
        assert_eq!(reversed.f32().to_bits(), 0x40a4_0000);
    }

    /// A vector of `kinds.len()` collision shapes at a fresh vector object; every shape has the
    /// position (1, 1, 1) and the dimensions (1, 1, 1).
    fn shapes(e: &mut Engine, kinds: &[u32]) -> (u32, u32) {
        let vector = e.mem.alloc(0x18);
        let storage = make_vector(e, vector, 0x1C * kinds.len() as u32);
        for (i, kind) in kinds.iter().enumerate() {
            let shape = storage + 0x1C * i as u32;
            e.mem.set_u32(shape, *kind);
            for axis in 0..6 {
                e.mem.set_f32(shape + 4 + 4 * axis, 1.0);
            }
        }
        (vector, storage)
    }

    #[test]
    fn collision_shapes_are_moved_and_scaled_by_the_matrix() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        let (vector, storage) = shapes(&mut e, &[0, 1, 2]);
        let matrix = e.mem.alloc(0x40);
        for (offset, value) in [
            (0x00, 2.0f32),
            (0x14, 3.0),
            (0x28, 5.0),
            (0x30, 10.0),
            (0x34, 20.0),
            (0x38, 30.0),
        ] {
            e.mem.set_f32(matrix + offset, value);
        }
        e.call(0x00b0_4db0, &args![vector, matrix]);
        let read = |e: &Engine, shape: u32| -> [f32; 6] {
            [4, 8, 12, 0x10, 0x14, 0x18].map(|offset| e.mem.f32(shape + offset))
        };
        assert_eq!(read(&e, storage), [12.0, 23.0, 35.0, 2.0, 1.0, 1.0]);
        assert_eq!(read(&e, storage + 0x1C), [12.0, 23.0, 35.0, 2.0, 2.0, 1.0]);
        assert_eq!(read(&e, storage + 0x38), [12.0, 23.0, 35.0, 2.0, 3.0, 5.0]);
    }

    #[test]
    fn collision_shapes_of_an_unknown_type_keep_their_dimensions() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        let (vector, storage) = shapes(&mut e, &[7]);
        let matrix = e.mem.alloc(0x40);
        e.mem.set_f32(matrix, 2.0);
        e.call(0x00b0_4db0, &args![vector, matrix]);
        assert_eq!(e.mem.f32(storage + 0x10), 1.0);
        assert_eq!(
            [4, 8, 12].map(|offset| e.mem.f32(storage + offset)),
            [2.0, 0.0, 0.0]
        );
    }

    #[test]
    fn a_vector_times_a_matrix() {
        let mut e = Engine::new();
        let matrix = e.mem.alloc(0x40);
        for i in 0..16u32 {
            e.mem.set_f32(matrix + 4 * i, (i + 1) as f32);
        }
        let vector = e.mem.alloc(0xC);
        for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(vector + 4 * i as u32, value);
        }
        let result = e.mem.alloc(0xC);
        assert_eq!(
            e.call(0x00b0_5000, &args![vector, result, matrix]).u32(),
            result
        );
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(result + offset)),
            [51.0, 58.0, 65.0]
        );
    }

    #[test]
    fn the_collision_record_destroys_its_members() {
        let (mut e, calls) = recording(&[TRIVIAL_DESTRUCTOR]);
        let record = e.mem.alloc(0x54);
        e.call(0x00b0_50d0, &args![record]);
        assert_eq!(
            calls_to(&calls, TRIVIAL_DESTRUCTOR),
            vec![vec![record + 0x18], vec![record]]
        );
    }

    #[test]
    fn the_embedded_coordinates_record_starts_empty() {
        let (mut e, calls) = recording(&[STRING_TIDY]);
        let record = e.mem.alloc(0x54);
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(record + offset, 0xFFFF_FFFF);
        }
        assert_eq!(e.call(0x00b0_5130, &args![record]).u32(), record);
        assert_eq!(
            calls_to(&calls, STRING_TIDY),
            vec![vec![record + 0x18, 0, 0]]
        );
        assert_eq!(e.mem.u32(record), 0);
        assert_eq!(e.mem.u32(record + 0x14), 0);
        assert_eq!(
            [0x34, 0x38, 0x3C, 0x40, 0x44, 0x48, 0x4C, 0x50].map(|o| e.mem.f32(record + o)),
            [1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]
        );
    }

    #[test]
    fn the_parent_of_an_instance() {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        assert_eq!(e.call(0x00b0_51e0, &args![tree]).u32(), 0);
        let data = e.mem.alloc(0x14);
        e.mem.set_u32(data, 0x1234_5678);
        e.set(tree, CSpeedTreeRT::m_pInstanceData, Ptr::new(data));
        assert_eq!(e.call(0x00b0_51e0, &args![tree]).u32(), 0x1234_5678);
    }

    #[test]
    fn making_an_instance_allocates_and_builds_it() {
        let (mut e, _) = recording(&[0x00b0_ddc0]);
        e.register(MEMCPY, memcpy_double);
        global(&mut e, ALL_REF_COUNT, 0);
        global_f32(&mut e, 0x0101_2054, -1.0);
        let tree = linked_tree(&mut e);
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 2);
        let made = e.call(0x00b0_5210, &args![tree]).u32();
        assert_ne!(made, 0);
        assert_ne!(made, tree.addr());
        let instance = Ptr::<CSpeedTreeRT>::new(made);
        assert_eq!(e.get(instance, CSpeedTreeRT::m_eLeafLodMethod), 2);
        let data = e.get(instance, CSpeedTreeRT::m_pInstanceData);
        assert_eq!(e.get(data, STreeInstanceData::m_pParent), tree);
        assert_eq!(e.global::<u32>(ALL_REF_COUNT), 1);
    }

    #[test]
    fn transient_data_is_freed_only_while_intact() {
        let (mut e, calls) = recording(&[0x00b2_b940]);
        error_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.call(0x00b0_53e0, &args![tree]);
        assert!(calls_to(&calls, 0x00b2_b940).is_empty());
        assert!(error_was(&calls, 0x010a_49f4));
        e.mem.set_u8(engine + ENGINE_TRANSIENT_DATA_INTACT, 1);
        e.call(0x00b0_53e0, &args![tree]);
        assert_eq!(calls_to(&calls, 0x00b2_b940), vec![vec![engine]]);
    }

    /// A `CTreeFileAccess` (index 0 and a byte vector of `length` zero bytes).
    fn access(e: &mut Engine, length: u32) -> u32 {
        let object = e.mem.alloc(0x14);
        make_vector(e, object + 4, length);
        object
    }

    /// The doubles of a `CTreeFileAccess`: its vector's constructor and growth (fresh storage
    /// of the requested size), `ParseInt` giving `tags` in turn (then 0) and `ParseFloat` giving
    /// `floats` in turn; both advance the index as the real parsers do.
    fn file_doubles(e: &mut Engine, tags: Vec<i32>, floats: Vec<f64>) {
        e.register_double(0x0065_7b50, |_, _| Ret::default());
        e.register_double(0x00b0_e200, |e, a| {
            let storage = e.mem.alloc(a[1].max(8));
            let proxy = e.mem.alloc(8);
            e.mem.set_u32(a[0], proxy);
            e.mem.set_u32(a[0] + 0x0C, storage);
            e.mem.set_u32(a[0] + 0x10, storage + a[1]);
            e.mem.set_u32(a[0] + 0x14, storage + a[1]);
            Ret::default()
        });
        e.register_double(0x0065_7f60, |_, _| Ret::default());
        let mut queue = tags.into_iter();
        e.register_double(PARSE_INT, move |e, a| {
            let index = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], index + 1);
            Ret {
                eax: queue.next().unwrap_or(0) as u32,
                ..Ret::default()
            }
        });
        let mut values = floats.into_iter();
        e.register_double(PARSE_FLOAT, move |e, a| {
            let index = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], index + 1);
            Ret {
                st0: values.next().unwrap_or(0.0),
                ..Ret::default()
            }
        });
    }

    /// The doubles `throw_formatted` and `throw_parse_error` call; the throw itself panics with
    /// "thrown".
    fn throw_doubles(e: &mut Engine, calls: &Calls) {
        let errno = e.mem.alloc(4);
        e.mem.set_u32(errno, 2);
        stub(e, calls, ERRNO, errno);
        stub(e, calls, STRERROR, 0x1234_0000);
        for address in [FORMAT_STRING, EXCEPTION_CONSTRUCT] {
            stub(e, calls, address, 0);
        }
        e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
    }

    /// Whether `action` throws (the panicking `_CxxThrowException` double).
    fn throws(action: impl FnOnce()) -> bool {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(action)) {
            Ok(()) => false,
            Err(payload) => payload.downcast_ref::<&str>() == Some(&"thrown"),
        }
    }

    /// A file object as `LoadTree` uses it: its vtable's deleting destructor (`+0`) and size
    /// (`+0x28`), and the stream's own read word at `+8` of the object (filling the buffer with
    /// 7s and returning `read`). Its vtable slot `+8` is the seek, which must not be called.
    fn file_object(e: &mut Engine, calls: &Calls, size: u32, read: u32) -> u32 {
        let (destroy, read_slot, size_slot) = (0x0f10_0000, 0x0f10_0010, 0x0f10_0020);
        stub(e, calls, destroy, 0);
        stub(e, calls, size_slot, size);
        let log = calls.clone();
        e.register_double(read_slot, move |e, a| {
            log.borrow_mut()
                .push((read_slot, vec![a[0], a[2], e.mem.u32(a[3]), a[4]]));
            for offset in 0..a[2] {
                e.mem.set_u8(a[1] + offset, 7);
            }
            Ret {
                eax: read,
                ..Ret::default()
            }
        });
        let seek_slot = 0x0f10_0030;
        e.register_double(seek_slot, |_, _| {
            panic!("the seek slot was called for the read")
        });
        let vtable = 0x0f10_0100;
        let mut slots = [0u32; 11];
        slots[0] = destroy;
        slots[2] = seek_slot;
        slots[10] = size_slot;
        e.put_vtable(vtable, &slots);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file, vtable);
        e.mem.set_u32(file + 8, read_slot);
        file
    }

    #[test]
    fn load_tree_reads_the_file_and_loads_it() {
        let (mut e, calls) = recording(&[]);
        file_doubles(&mut e, vec![], vec![]);
        let file = file_object(&mut e, &calls, 4, 4);
        stub(&mut e, &calls, FILE_FINDER_GET_FILE, file);
        stub(&mut e, &calls, 0x00b2_b550, 0);
        let tree = linked_tree(&mut e);
        assert_eq!(e.call(0x00b0_5420, &args![tree, 0x5555u32]).u32(), 0);
        assert_eq!(
            calls_to(&calls, FILE_FINDER_GET_FILE),
            vec![vec![0x5555, 0, 0x2800, 1]]
        );
        // The file, the count read, the flag word (1 when read) and 1.
        assert_eq!(calls_to(&calls, 0x0f10_0010), vec![vec![file, 4, 1, 1]]);
        assert_eq!(e.mem.u32(file + 4), 4);
        assert_eq!(calls_to(&calls, 0x0f10_0000), vec![vec![file, 1]]);
        // The memory form ran: the engine was asked to parse the access object.
        assert_eq!(calls_to(&calls, 0x00b2_b550).len(), 1);
    }

    #[test]
    fn load_tree_without_a_name_does_nothing() {
        let (mut e, calls) = recording(&[FILE_FINDER_GET_FILE]);
        let tree = linked_tree(&mut e);
        assert_eq!(e.call(0x00b0_5420, &args![tree, 0u32]).u32(), 0);
        assert!(calls_to(&calls, FILE_FINDER_GET_FILE).is_empty());
    }

    #[test]
    fn load_tree_throws_when_the_file_will_not_open() {
        let (mut e, calls) = recording(&[]);
        throw_doubles(&mut e, &calls);
        stub(&mut e, &calls, FILE_FINDER_GET_FILE, 0);
        let tree = linked_tree(&mut e);
        assert!(throws(|| {
            e.call(0x00b0_5420, &args![tree, 0x5555u32]);
        }));
        let format = calls_to(&calls, FORMAT_STRING);
        assert_eq!(format[0][1..], [0x010a_4a8c, 0x5555, 0x1234_0000]);
        assert_eq!(calls_to(&calls, EXCEPTION_CONSTRUCT).len(), 1);
    }

    #[test]
    fn load_tree_throws_on_an_empty_file() {
        let (mut e, calls) = recording(&[]);
        throw_doubles(&mut e, &calls);
        let file = file_object(&mut e, &calls, 0, 0);
        stub(&mut e, &calls, FILE_FINDER_GET_FILE, file);
        let tree = linked_tree(&mut e);
        assert!(throws(|| {
            e.call(0x00b0_5420, &args![tree, 0x5555u32]);
        }));
        let format = calls_to(&calls, FORMAT_STRING);
        assert_eq!(format[0][1..], [0x010a_4aac, 0x5555, 0x1234_0000]);
        // The file is not deleted on the error path.
        assert!(calls_to(&calls, 0x0f10_0000).is_empty());
    }

    #[test]
    fn load_tree_throws_on_a_short_read() {
        let (mut e, calls) = recording(&[]);
        throw_doubles(&mut e, &calls);
        let file = file_object(&mut e, &calls, 8, 3);
        stub(&mut e, &calls, FILE_FINDER_GET_FILE, file);
        let tree = linked_tree(&mut e);
        assert!(throws(|| {
            e.call(0x00b0_5420, &args![tree, 0x5555u32]);
        }));
        let format = calls_to(&calls, FORMAT_STRING);
        assert_eq!(format[0][1..], [0x010a_4acc, 3, 8, 0x5555, 0x1234_0000]);
        assert_eq!(e.mem.u32(file + 4), 3);
    }

    #[test]
    fn loading_from_memory_applies_the_lighting_and_wind_settings() {
        let (mut e, calls) = recording(&[]);
        file_doubles(&mut e, vec![], vec![]);
        stub(&mut e, &calls, 0x00b2_b550, 1);
        stub(&mut e, &calls, 0x00b1_8300, 4);
        stub(&mut e, &calls, 0x00b3_7570, 0);
        e.map(WIND_MATRIX_SPAN, 4);
        e.mem.set_u16(WIND_MATRIX_SPAN, 12);
        global_f32(&mut e, 0x0101_6248, 0.75);
        let tree = linked_tree(&mut e);
        let lighting = part(&e, tree, CSpeedTreeRT::m_pLightingEngine);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        e.mem.set_u32(lighting, 1);
        e.mem.set_u32(lighting + 0x38, 1);
        e.mem.set_u32(engine + 0x108, 9);
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 2);
        let data = e.mem.alloc(8);
        assert_eq!(e.call(0x00b0_59e0, &args![tree, data, 0u32]).u32(), 1);
        assert_eq!(calls_to(&calls, 0x00b2_b550)[0][0], engine);
        let branch = part(&e, tree, CSpeedTreeRT::m_pBranchGeometry);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        let frond = part(&e, tree, CSpeedTreeRT::m_pFrondGeometry);
        assert_eq!(
            (e.mem.u8(branch + 9), e.mem.u8(leaf), e.mem.u8(frond + 9)),
            (1, 1, 0)
        );
        assert_eq!(e.get(tree, CSpeedTreeRT::m_nBranchWindLevel), 9);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_nFrondLevel), 4);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_eLeafLodMethod), 1);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_fLeafTransitionFactor), 0.75);
        assert_eq!(calls_to(&calls, 0x00b3_7570), vec![vec![wind, 0, 12]]);
    }

    #[test]
    fn loading_from_memory_stops_when_the_header_is_refused() {
        let (mut e, calls) = recording(&[]);
        file_doubles(&mut e, vec![], vec![]);
        stub(&mut e, &calls, 0x00b2_b550, 0);
        let tree = linked_tree(&mut e);
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 2);
        let data = e.mem.alloc(8);
        assert_eq!(e.call(0x00b0_59e0, &args![tree, data, 4u32]).u32(), 0);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_eLeafLodMethod), 2);
    }

    #[test]
    fn loading_from_memory_dispatches_each_chunk_to_its_parser() {
        let (mut e, calls) = recording(&[]);
        let tags = vec![
            8000, 13000, 15000, 16000, 16013, 16014, 21000, 21001, 22000, 0,
        ];
        file_doubles(&mut e, tags, vec![0.5, 1.5, 2.5]);
        for address in [
            0x00b2_b550,
            0x00b2_a940,
            0x00b1_8340,
            0x00b2_d090,
            0x00b2_d2e0,
            0x00b2_daf0,
            0x00b1_8300,
            0x00b3_7570,
        ] {
            stub(&mut e, &calls, address, 1);
        }
        e.map(WIND_MATRIX_SPAN, 4);
        global(&mut e, TREE_FILE_FLAG, 0);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        let lighting = part(&e, tree, CSpeedTreeRT::m_pLightingEngine);
        let frond_engine = part(&e, tree, CSpeedTreeRT::m_pFrondEngine);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        // Twenty bytes of data; 22000 reads its bool at index 12 (nine tags and the three floats
        // the doubles count before it).
        let data = e.mem.alloc(0x14);
        e.mem.set_u8(data + 12, 1);
        assert_eq!(e.call(0x00b0_59e0, &args![tree, data, 0x14u32]).u32(), 1);
        let first_argument = |address: u32| {
            calls_to(&calls, address)
                .iter()
                .map(|a| a[0])
                .collect::<Vec<_>>()
        };
        assert_eq!(first_argument(0x00b2_a940), vec![lighting]);
        assert_eq!(first_argument(0x00b1_8340), vec![frond_engine]);
        assert_eq!(first_argument(0x00b2_d090), vec![engine]);
        assert_eq!(first_argument(0x00b2_d2e0), vec![engine]);
        assert_eq!(first_argument(0x00b2_daf0), vec![engine]);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_fLeafTransitionFactor), 0.5);
        assert_eq!(e.mem.f32(wind + 0x3C), 1.5);
        assert_eq!(e.mem.f32(wind + 0x40), 2.5);
        assert_eq!(e.global::<u8>(TREE_FILE_FLAG), 1);
    }

    #[test]
    fn file_access_is_filled_from_memory() {
        let (mut e, calls) = recording(&[0x0065_7b50]);
        e.register_double(0x00b0_e200, |e, a| {
            let storage = e.mem.alloc(a[1].max(8));
            e.mem.set_u32(a[0] + 0x0C, storage);
            e.mem.set_u32(a[0] + 0x10, storage + a[1]);
            Ret::default()
        });
        let data = e.mem.alloc(8);
        for (i, byte) in [9u8, 8, 7].into_iter().enumerate() {
            e.mem.set_u8(data + i as u32, byte);
        }
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object, 0xFFFF);
        assert_eq!(
            e.call(0x00b0_6090, &args![object, data, 3i32]).u32(),
            object
        );
        assert_eq!(e.mem.u32(object), 0);
        assert_eq!(calls_to(&calls, 0x0065_7b50), vec![vec![object + 4]]);
        let first = e.mem.u32(object + 0x10);
        assert_eq!(e.mem.bytes(first, 3), vec![9, 8, 7]);
    }

    #[test]
    fn parsing_a_bool_advances_the_index() {
        let mut e = Engine::new();
        let object = access(&mut e, 2);
        let first = e.mem.u32(object + 0x10);
        e.mem.set_u8(first + 1, 5);
        assert_eq!(e.call(0x00b0_6180, &args![object]).u32(), 0);
        assert_eq!(e.mem.u32(object), 1);
        assert_eq!(e.call(0x00b0_6180, &args![object]).u32(), 1);
        assert_eq!(e.mem.u32(object), 2);
    }

    #[test]
    fn parsing_a_bool_past_the_end_takes_the_invalid_parameter_path() {
        let (mut e, calls) = recording(&[INVALID_PARAMETER]);
        let object = access(&mut e, 1);
        e.mem.set_u32(object, 1);
        e.call(0x00b0_6180, &args![object]);
        assert_eq!(calls_to(&calls, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn file_access_destructor_clears_and_frees_the_vector() {
        let (mut e, calls) = recording(&[0x0065_7f60]);
        let object = access(&mut e, 4);
        e.call(0x00b0_61e0, &args![object]);
        assert_eq!(calls_to(&calls, 0x0065_7f60), vec![vec![object + 4]]);
    }

    #[test]
    fn the_tree_size_getter_passes_both_addresses_on() {
        let (mut e, calls) = recording(&[0x00b2_bcf0]);
        let tree = linked_tree(&mut e);
        e.call(0x00b0_6260, &args![tree, 0x100u32, 0x200u32]);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        assert_eq!(
            calls_to(&calls, 0x00b2_bcf0),
            vec![vec![engine, 0x100, 0x200]]
        );
    }

    #[test]
    fn setting_the_tree_size_checks_the_data_and_the_size() {
        let (mut e, calls) = recording(&[0x00b2_bd20]);
        error_doubles(&mut e, &calls);
        global_f64(&mut e, 0x0101_2060, 0.0);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        // No intact transient data.
        e.call(0x00b0_6280, &args![tree, 2.0f32, 0.5f32]);
        assert!(error_was(&calls, 0x010a_4b20));
        e.mem.set_u8(engine + ENGINE_TRANSIENT_DATA_INTACT, 1);
        // A size of zero or less, and a NaN, are rejected.
        e.call(0x00b0_6280, &args![tree, 0.0f32, 0.5f32]);
        e.call(0x00b0_6280, &args![tree, f32::NAN, 0.5f32]);
        assert_eq!(
            calls_to(&calls, STRING_ASSIGN_TEXT)
                .iter()
                .filter(|a| a[1] == 0x010a_4b68)
                .count(),
            2
        );
        assert!(calls_to(&calls, 0x00b2_bd20).is_empty());
        e.call(0x00b0_6280, &args![tree, 2.0f32, 0.5f32]);
        assert_eq!(
            calls_to(&calls, 0x00b2_bd20),
            vec![vec![engine, 2.0f32.to_bits(), 0.5f32.to_bits()]]
        );
    }

    #[test]
    fn the_engine_wind_level_needs_intact_data() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_u32(engine + 0x48, 77);
        assert_eq!(e.call(0x00b0_6440, &args![tree]).u32(), 0);
        assert!(error_was(&calls, 0x010a_4b20));
        e.mem.set_u8(engine + ENGINE_TRANSIENT_DATA_INTACT, 1);
        assert_eq!(e.call(0x00b0_6440, &args![tree]).u32(), 77);
    }

    #[test]
    fn a_position_goes_to_the_instance_data_or_the_engine() {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.call(0x00b0_65e0, &args![tree, 1.0f32, 2.0f32, 3.0f32]);
        assert_eq!(
            [4, 8, 12].map(|offset| e.mem.f32(engine + offset)),
            [1.0, 2.0, 3.0]
        );
        let data = e.mem.alloc(0x14);
        e.set(tree, CSpeedTreeRT::m_pInstanceData, Ptr::new(data));
        e.call(0x00b0_65e0, &args![tree, 4.0f32, 5.0f32, 6.0f32]);
        assert_eq!(
            [4, 8, 12].map(|offset| e.mem.f32(data + offset)),
            [4.0, 5.0, 6.0]
        );
        // The engine's position is untouched the second time.
        assert_eq!(e.mem.f32(engine + 4), 1.0);
    }

    #[test]
    fn the_lighting_methods_are_set_before_compute_only() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        let lighting = part(&e, tree, CSpeedTreeRT::m_pLightingEngine);
        let branch = part(&e, tree, CSpeedTreeRT::m_pBranchGeometry);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        let frond = part(&e, tree, CSpeedTreeRT::m_pFrondGeometry);
        e.call(0x00b0_6660, &args![tree, 1u32]);
        e.call(0x00b0_6810, &args![tree, 1u32]);
        e.call(0x00b0_69c0, &args![tree, 3u32]);
        assert_eq!(
            (
                e.mem.u32(lighting),
                e.mem.u32(lighting + 0x38),
                e.mem.u32(lighting + 0x78)
            ),
            (1, 1, 3)
        );
        assert_eq!(
            (e.mem.u8(branch + 9), e.mem.u8(leaf), e.mem.u8(frond + 9)),
            (1, 1, 0)
        );
        e.call(0x00b0_6660, &args![tree, 0u32]);
        assert_eq!((e.mem.u32(lighting), e.mem.u8(branch + 9)), (0, 0));
        e.set(tree, CSpeedTreeRT::m_bTreeComputed, 1);
        e.call(0x00b0_6660, &args![tree, 1u32]);
        e.call(0x00b0_6810, &args![tree, 0u32]);
        e.call(0x00b0_69c0, &args![tree, 0u32]);
        assert!(error_was(&calls, 0x010a_4bd0));
        assert!(error_was(&calls, 0x010a_4c40));
        assert!(error_was(&calls, 0x010a_4cb0));
        assert_eq!(e.mem.u32(lighting), 0);
    }

    #[test]
    fn the_camera_is_read_back_and_checked() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        e.register(MEMCPY, memcpy_double);
        e.map(CAMERA_POSITION, 0x20);
        for (i, value) in [1.0f32, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(CAMERA_POSITION + 4 * i as u32, value);
        }
        let (position, direction) = (e.mem.alloc(0xC), e.mem.alloc(0xC));
        e.call(0x00b0_6b70, &args![position, direction]);
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(position + offset)),
            [1.0, 2.0, 3.0]
        );
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(direction + offset)),
            [4.0, 5.0, 6.0]
        );
        e.call(0x00b0_6b70, &args![0u32, direction]);
        assert!(error_was(&calls, 0x010a_4cf8));
    }

    /// The statics and doubles `SetCamera` needs: the math function returns 1.0, the angle
    /// scale is 57.5, the blend limits 40 and 80, and the table 255 and 84.
    fn camera_fixture(e: &mut Engine, calls: &Calls) {
        error_doubles(e, calls);
        for address in [ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION, 0x00b2_7080] {
            stub(e, calls, address, 0);
        }
        stub_float(e, calls, 0x00ec_9610, 1.0);
        iterator_doubles(e);
        e.register(MEMCPY, memcpy_double);
        e.map(CAMERA_POSITION, 0x20);
        e.map(UNIQUE_TREES, 0x20);
        global_f64(e, ANGLE_SCALE, 57.5);
        global_f32(e, 0x011a_d11c, 40.0);
        global_f32(e, 0x011a_d120, 80.0);
        global_f32(e, 0x011a_d124, 0.0);
        global_f32(e, 0x0102_3cd8, 255.0);
        global_f32(e, 0x010a_4d88, 84.0);
        global_f64(e, 0x0101_2060, 0.0);
        global_f64(e, 0x0101_2070, 1.0);
        make_vector(e, UNIQUE_TREES, 0);
    }

    #[test]
    fn a_new_camera_is_stored_and_blends_the_lighting() {
        let (mut e, calls) = recording(&[]);
        camera_fixture(&mut e, &calls);
        let (position, direction) = (e.mem.alloc(0xC), e.mem.alloc(0xC));
        for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, value);
        }
        for (i, value) in [0.0f32, 0.0, 1.0].into_iter().enumerate() {
            e.mem.set_f32(direction + 4 * i as u32, value);
        }
        e.call(0x00b0_6c10, &args![position, direction]);
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(CAMERA_POSITION + offset)),
            [1.0, 2.0, 3.0]
        );
        assert_eq!(e.mem.f32(CAMERA_DIRECTION + 8), 1.0);
        assert_eq!(calls_to(&calls, 0x00b2_7080).len(), 1);
        // The pitch angle is 1 * 57.5 and t = 1 - (57.5 - 40) / 40 = 0.5625.
        assert_eq!(e.mem.f32(0x011a_d124), 0.5625 * 171.0 + 84.0);
        // The same camera again changes nothing.
        e.mem.set_f32(0x011a_d124, 0.0);
        e.call(0x00b0_6c10, &args![position, direction]);
        assert_eq!(e.mem.f32(0x011a_d124), 0.0);
        assert_eq!(calls_to(&calls, 0x00b2_7080).len(), 1);
        // A changed direction recomputes it.
        e.mem.set_f32(direction, 0.5);
        e.call(0x00b0_6c10, &args![position, direction]);
        assert_eq!(calls_to(&calls, 0x00b2_7080).len(), 2);
    }

    #[test]
    fn the_camera_blend_is_clamped() {
        let (mut e, calls) = recording(&[]);
        camera_fixture(&mut e, &calls);
        let (position, direction) = (e.mem.alloc(0xC), e.mem.alloc(0xC));
        e.mem.set_f32(direction + 8, 1.0);
        // An angle of 57.5 against the limits 100 and 200: t = 1 - (-42.5 / 100) > 1.
        global_f32(&mut e, 0x011a_d11c, 100.0);
        global_f32(&mut e, 0x011a_d120, 200.0);
        e.call(0x00b0_6c10, &args![position, direction]);
        assert_eq!(e.mem.f32(0x011a_d124), 255.0);
        // The limits 0 and 20: t = 1 - 57.5 / 20 < 0.
        e.mem.set_f32(position, 1.0);
        global_f32(&mut e, 0x011a_d11c, 0.0);
        global_f32(&mut e, 0x011a_d120, 20.0);
        e.call(0x00b0_6c10, &args![position, direction]);
        assert_eq!(e.mem.f32(0x011a_d124), 84.0);
    }

    #[test]
    fn a_null_camera_vector_is_an_error() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        e.call(0x00b0_6c10, &args![0u32, 0u32]);
        assert!(error_was(&calls, 0x010a_4d4c));
    }

    /// A shared tree list (`011f8bdc`) holding `trees`, with the critical section stubs and the
    /// iterator doubles `00b08ec0` runs through.
    fn tree_list(e: &mut Engine, calls: &Calls, trees: &[Ptr<CSpeedTreeRT>]) {
        for address in [ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION, 0x00b2_7820] {
            stub(e, calls, address, 0);
        }
        iterator_doubles(e);
        e.map(UNIQUE_TREES, 0x20);
        let storage = make_vector(e, UNIQUE_TREES, 4 * trees.len() as u32);
        for (i, tree) in trees.iter().enumerate() {
            e.mem.set_u32(storage + 4 * i as u32, tree.addr());
        }
    }

    #[test]
    fn setting_the_time_stores_it_and_refreshes_the_trees() {
        let (mut e, calls) = recording(&[]);
        tree_list(&mut e, &calls, &[]);
        e.map(WIND_TIME, 4);
        e.call(0x00b0_7060, &args![1.25f32]);
        assert_eq!(e.mem.f32(WIND_TIME), 1.25);
        assert_eq!(calls_to(&calls, ENTER_CRITICAL_SECTION).len(), 1);
        assert_eq!(calls_to(&calls, LEAVE_CRITICAL_SECTION).len(), 1);
    }

    #[test]
    fn the_shared_tree_loop_with_mode_zero_invalidates_wind_geometry() {
        let (mut e, calls) = recording(&[]);
        let tree = linked_tree(&mut e);
        tree_list(&mut e, &calls, &[tree]);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        let branch = part(&e, tree, CSpeedTreeRT::m_pBranchGeometry);
        let frond = part(&e, tree, CSpeedTreeRT::m_pFrondGeometry);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        e.mem.set_u32(wind + 8, 1);
        e.mem.set_u32(wind + 0xC, 1);
        e.mem.set_u32(wind + 0x10, 0);
        e.mem.set_u8(wind + 0x14, 1);
        e.mem.set_u8(branch + 0x12, 1);
        e.mem.set_u8(frond + 0x12, 1);
        e.call(0x00b0_8ec0, &args![0i32]);
        assert_eq!((e.mem.u8(branch + 0x12), e.mem.u8(frond + 0x12)), (0, 0));
        assert_eq!(calls_to(&calls, 0x00b2_7820), vec![vec![leaf]]);
        // With the leaf method 2 and the rocking flag clear nothing runs on the leaves.
        e.mem.set_u32(wind + 0x10, 2);
        e.mem.set_u8(wind + 0x14, 0);
        e.call(0x00b0_8ec0, &args![0i32]);
        assert_eq!(calls_to(&calls, 0x00b2_7820).len(), 1);
        // The leaf method 1 runs them regardless.
        e.mem.set_u32(wind + 0x10, 1);
        e.call(0x00b0_8ec0, &args![0i32]);
        assert_eq!(calls_to(&calls, 0x00b2_7820).len(), 2);
    }

    #[test]
    fn the_shared_tree_loop_with_mode_one_clears_the_billboard() {
        let (mut e, calls) = recording(&[]);
        let tree = linked_tree(&mut e);
        tree_list(&mut e, &calls, &[tree]);
        let billboard = e.mem.alloc(0x34);
        e.mem.set_u8(billboard + 0x30, 1);
        e.set(tree, CSpeedTreeRT::m_pSimpleBillboard, Ptr::new(billboard));
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        e.call(0x00b0_8ec0, &args![1i32]);
        assert_eq!(e.mem.u8(billboard + 0x30), 0);
        assert_eq!(calls_to(&calls, 0x00b2_7820), vec![vec![leaf]]);
        // Another mode does nothing.
        e.mem.set_u8(billboard + 0x30, 1);
        e.call(0x00b0_8ec0, &args![5i32]);
        assert_eq!(e.mem.u8(billboard + 0x30), 1);
        assert_eq!(calls_to(&calls, 0x00b2_7820).len(), 1);
    }

    #[test]
    fn the_rocking_flag_is_stored_in_the_wind_engine() {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        e.call(0x00b0_71c0, &args![tree, 1u8]);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        assert_eq!(e.mem.u8(wind + 0x14), 1);
    }

    #[test]
    fn the_rocking_group_count_is_at_least_one_and_set_before_compute() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.call(0x00b0_71f0, &args![tree, 0u32]);
        assert_eq!(e.mem.u32(engine + 0xD4), 1);
        e.call(0x00b0_71f0, &args![tree, 6u32]);
        assert_eq!(e.mem.u32(engine + 0xD4), 6);
        e.set(tree, CSpeedTreeRT::m_bTreeComputed, 1);
        e.call(0x00b0_71f0, &args![tree, 9u32]);
        assert_eq!(e.mem.u32(engine + 0xD4), 6);
        assert!(error_was(&calls, 0x010a_4dd8));
    }

    /// A float setter that stores through the engine at `offset`, and does nothing without one.
    fn check_engine_float_setter(address: u32, offset: u32) {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.call(address, &args![tree, 2.5f32]);
        assert_eq!(e.mem.f32(engine + offset), 2.5);
        let bare = e.new_object::<CSpeedTreeRT>();
        e.call(address, &args![bare, 3.5f32]);
        assert_eq!(e.mem.f32(engine + offset), 2.5);
    }

    #[test]
    fn engine_float_setter_at_0x9c() {
        check_engine_float_setter(0x00b0_7390, 0x9C);
    }

    #[test]
    fn engine_float_setter_at_0x98() {
        check_engine_float_setter(0x00b0_73c0, 0x98);
    }

    #[test]
    fn engine_float_setter_at_0xcc() {
        check_engine_float_setter(0x00b0_7490, 0xCC);
    }

    #[test]
    fn engine_float_setter_at_0xd0() {
        check_engine_float_setter(0x00b0_74c0, 0xD0);
    }

    /// A getter of a float of the engine's branch geometry at `offset`: zero without an engine
    /// or without the geometry.
    fn check_trunk_getter(address: u32, offset: u32) {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        assert_eq!(e.call(address, &args![tree]).f32(), 0.0);
        let geometry = e.mem.alloc(0x40);
        e.mem.set_f32(geometry + offset, 6.5);
        e.mem.set_u32(engine + 0x5C, geometry);
        assert_eq!(e.call(address, &args![tree]).f32(), 6.5);
        let bare = e.new_object::<CSpeedTreeRT>();
        assert_eq!(e.call(address, &args![bare]).f32(), 0.0);
    }

    #[test]
    fn the_trunk_width_is_read_from_the_branch_geometry() {
        check_trunk_getter(0x00b0_73f0, 0x18);
    }

    #[test]
    fn the_trunk_length_is_read_from_the_branch_geometry() {
        check_trunk_getter(0x00b0_7440, 0x1C);
    }

    #[test]
    fn the_wind_methods_are_read_from_the_wind_engine() {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        e.mem.set_u32(wind + 8, 11);
        e.mem.set_u32(wind + 0xC, 12);
        e.mem.set_u32(wind + 0x10, 13);
        assert_eq!(e.call(0x00b0_76c0, &args![tree]).u32(), 11);
        assert_eq!(e.call(0x00b0_78e0, &args![tree]).u32(), 12);
        assert_eq!(e.call(0x00b0_74f0, &args![tree]).u32(), 13);
    }

    #[test]
    fn the_leaf_wind_method_sets_the_vertex_weighting_flag() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        e.call(0x00b0_7510, &args![tree, 1u32]);
        assert_eq!((e.mem.u32(wind + 0x10), e.mem.u8(leaf + 1)), (1, 1));
        e.call(0x00b0_7510, &args![tree, 2u32]);
        assert_eq!((e.mem.u32(wind + 0x10), e.mem.u8(leaf + 1)), (2, 0));
        e.set(tree, CSpeedTreeRT::m_bTreeComputed, 1);
        e.call(0x00b0_7510, &args![tree, 3u32]);
        assert_eq!(e.mem.u32(wind + 0x10), 2);
        assert!(error_was(&calls, 0x010a_4e40));
    }

    /// The common behaviour of the branch and frond wind-method setters: `address` sets the
    /// method at `method_offset` of the wind engine and the geometry's `+8` and `+0x14`; a
    /// method of 2 on a geometry whose wind method was 1 also invalidates it and runs its wind
    /// computation.
    fn check_geometry_wind_method(
        address: u32,
        geometry: Field<CSpeedTreeRT, Ptr>,
        method_offset: u32,
        message: u32,
    ) {
        let (mut e, calls) = recording(&[0x00b3_0800]);
        error_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        let target = part(&e, tree, geometry);
        e.mem.set_u8(target + 0x12, 1);
        e.call(address, &args![tree, 1u32]);
        assert_eq!(e.mem.u32(wind + method_offset), 1);
        assert_eq!((e.mem.u8(target + 8), e.mem.u32(target + 0x14)), (1, 1));
        // Method 2 with the old method 1: the geometry is invalidated and recomputed.
        e.call(address, &args![tree, 2u32]);
        assert_eq!(e.mem.u32(wind + method_offset), 2);
        assert_eq!((e.mem.u8(target + 8), e.mem.u32(target + 0x14)), (0, 2));
        assert_eq!(e.mem.u8(target + 0x12), 0);
        assert_eq!(calls_to(&calls, 0x00b3_0800), vec![vec![target, 0]]);
        // The old method is now 2: no second computation.
        e.call(address, &args![tree, 2u32]);
        assert_eq!(calls_to(&calls, 0x00b3_0800).len(), 1);
        e.set(tree, CSpeedTreeRT::m_bTreeComputed, 1);
        e.call(address, &args![tree, 0u32]);
        assert!(error_was(&calls, message));
        assert_eq!(e.mem.u32(wind + method_offset), 2);
    }

    #[test]
    fn the_branch_wind_method_updates_the_branch_geometry() {
        check_geometry_wind_method(0x00b0_76e0, CSpeedTreeRT::m_pBranchGeometry, 8, 0x010a_4ea8);
    }

    #[test]
    fn the_frond_wind_method_updates_the_frond_geometry() {
        check_geometry_wind_method(
            0x00b0_7900,
            CSpeedTreeRT::m_pFrondGeometry,
            0xC,
            0x010a_4f10,
        );
    }

    #[test]
    fn wind_strength_fills_in_defaults_and_invalidates_the_geometry() {
        let (mut e, calls) = recording(&[0x00b2_7820]);
        error_doubles(&mut e, &calls);
        stub_float(&mut e, &calls, 0x00b3_7470, 8.5);
        global_f64(&mut e, 0x0101_2060, 0.0);
        global_f64(&mut e, 0x0101_a6b0, -1.0);
        let tree = linked_tree(&mut e);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        let branch = part(&e, tree, CSpeedTreeRT::m_pBranchGeometry);
        let frond = part(&e, tree, CSpeedTreeRT::m_pFrondGeometry);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        e.mem.set_f32(wind, 0.75);
        e.mem.set_f32(wind + 4, 0.25);
        e.mem.set_u32(wind + 8, 1);
        e.mem.set_u32(wind + 0xC, 1);
        e.mem.set_u32(wind + 0x10, 2);
        e.mem.set_u8(branch + 0x12, 1);
        e.mem.set_u8(frond + 0x12, 1);
        let result = e.call(0x00b0_7b00, &args![tree, 2.0f32, -1.0f32, -1.0f32]);
        assert_eq!(result.f32(), 8.5);
        // The two defaults come from the wind engine's `+4` and `+0`.
        assert_eq!(
            calls_to(&calls, 0x00b3_7470),
            vec![vec![
                wind,
                2.0f32.to_bits(),
                0.25f32.to_bits(),
                0.75f32.to_bits()
            ]]
        );
        assert_eq!((e.mem.u8(branch + 0x12), e.mem.u8(frond + 0x12)), (0, 0));
        // The leaf method is 2 and the rocking flag is clear: the leaf geometry stays.
        assert!(calls_to(&calls, 0x00b2_7820).is_empty());
        e.mem.set_u8(wind + 0x14, 1);
        e.call(0x00b0_7b00, &args![tree, 2.0f32, 0.5f32, 0.5f32]);
        assert_eq!(calls_to(&calls, 0x00b2_7820), vec![vec![leaf]]);
        assert_eq!(
            calls_to(&calls, 0x00b3_7470)[1],
            vec![wind, 2.0f32.to_bits(), 0.5f32.to_bits(), 0.5f32.to_bits()]
        );
        // The leaf method 1 runs it regardless of the flag.
        e.mem.set_u8(wind + 0x14, 0);
        e.mem.set_u32(wind + 0x10, 1);
        e.call(0x00b0_7b00, &args![tree, 0.0f32, 0.5f32, 0.5f32]);
        assert_eq!(calls_to(&calls, 0x00b2_7820).len(), 2);
    }

    #[test]
    fn a_negative_wind_strength_is_an_error() {
        let (mut e, calls) = recording(&[0x00b3_7470]);
        error_doubles(&mut e, &calls);
        global_f64(&mut e, 0x0101_2060, 0.0);
        global_f64(&mut e, 0x0101_a6b0, -1.0);
        let tree = linked_tree(&mut e);
        for strength in [-0.5f32, f32::NAN] {
            let result = e.call(0x00b0_7b00, &args![tree, strength, 1.0f32, 1.0f32]);
            assert_eq!(result.f32(), 0.0);
        }
        assert!(calls_to(&calls, 0x00b3_7470).is_empty());
        assert_eq!(
            calls_to(&calls, STRING_ASSIGN_TEXT)
                .iter()
                .filter(|a| a[1] == 0x010a_4f74)
                .count(),
            2
        );
    }

    #[test]
    fn the_wind_matrix_table_is_resized_and_reset() {
        let (mut e, calls) = recording(&[0x00b3_ccb0, 0x00b3_ccd0]);
        let table = e.mem.alloc(8);
        e.call(0x00b0_7f20, &args![table, 3u16]);
        assert_eq!(e.mem.u16(table), 3);
        let block = e.mem.u32(table + 4);
        assert_ne!(block, 0);
        assert_eq!(
            calls_to(&calls, 0x00b3_ccb0),
            vec![vec![block], vec![block + 0x40], vec![block + 0x80]]
        );
        assert_eq!(
            calls_to(&calls, 0x00b3_ccd0),
            vec![vec![block], vec![block + 0x40], vec![block + 0x80]]
        );
        // The same count only resets the matrices.
        e.call(0x00b0_7f20, &args![table, 3u16]);
        assert_eq!(calls_to(&calls, 0x00b3_ccb0).len(), 3);
        assert_eq!(calls_to(&calls, 0x00b3_ccd0).len(), 6);
        assert_eq!(e.mem.u32(table + 4), block);
        // A new count frees the old table and builds another.
        e.call(0x00b0_7f20, &args![table, 1u16]);
        assert_eq!(e.mem.u16(table), 1);
        assert_eq!(calls_to(&calls, 0x00b3_ccb0).len(), 4);
    }

    #[test]
    fn the_static_matrix_table_takes_the_argument_word() {
        let (mut e, calls) = recording(&[0x00b3_ccb0, 0x00b3_ccd0]);
        e.map(WIND_MATRIX_SPAN, 8);
        e.call(0x00b0_7db0, &args![2u32]);
        assert_eq!(e.mem.u16(WIND_MATRIX_SPAN), 2);
        assert_eq!(calls_to(&calls, 0x00b3_ccb0).len(), 2);
    }

    #[test]
    fn local_matrices_must_fit_the_table() {
        let (mut e, calls) = recording(&[0x00b3_7570]);
        error_doubles(&mut e, &calls);
        e.map(WIND_MATRIX_SPAN, 4);
        e.mem.set_u16(WIND_MATRIX_SPAN, 10);
        let tree = linked_tree(&mut e);
        let wind = part(&e, tree, CSpeedTreeRT::m_pWindEngine);
        e.call(0x00b0_8050, &args![tree, 4u32, 6u32]);
        assert_eq!(calls_to(&calls, 0x00b3_7570), vec![vec![wind, 4, 6]]);
        e.call(0x00b0_8050, &args![tree, 5u32, 6u32]);
        assert_eq!(calls_to(&calls, 0x00b3_7570).len(), 1);
        assert!(error_was(&calls, 0x010a_5034));
        e.set(tree, CSpeedTreeRT::m_bTreeComputed, 1);
        e.call(0x00b0_8050, &args![tree, 0u32, 1u32]);
        assert!(error_was(&calls, 0x010a_4ff0));
        assert_eq!(calls_to(&calls, 0x00b3_7570).len(), 1);
    }

    #[test]
    fn computing_the_lod_level_borrows_the_instance_position() {
        let (mut e, calls) = recording(&[]);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        for (i, value) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(engine + 4 + 4 * i as u32, value);
        }
        e.mem.set_f32(engine + 0x14, 0.5);
        // The engine's computation sees the instance position and returns 0.75.
        let log = calls.clone();
        e.register_double(0x00b2_bd40, move |e, a| {
            let seen = [4, 8, 12, 0x14].map(|offset| e.mem.u32(a[0] + offset));
            log.borrow_mut().push((0x00b2_bd40, seen.to_vec()));
            Ret {
                st0: 0.75,
                ..Ret::default()
            }
        });
        let data = e.mem.alloc(0x14);
        for (i, value) in [7.0f32, 8.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(data + 4 + 4 * i as u32, value);
        }
        e.set(tree, CSpeedTreeRT::m_pInstanceData, Ptr::new(data));
        e.call(0x00b0_8200, &args![tree]);
        assert_eq!(
            calls_to(&calls, 0x00b2_bd40),
            vec![vec![
                7.0f32.to_bits(),
                8.0f32.to_bits(),
                9.0f32.to_bits(),
                0.5f32.to_bits()
            ]]
        );
        assert_eq!(e.mem.f32(data + 0x10), 0.75);
        // The engine's position and float are restored.
        assert_eq!(
            [4, 8, 12, 0x14].map(|offset| e.mem.f32(engine + offset)),
            [1.0, 2.0, 3.0, 0.5]
        );
    }

    #[test]
    fn computing_the_lod_level_without_instance_data_drops_the_result() {
        let (mut e, calls) = recording(&[]);
        stub_float(&mut e, &calls, 0x00b2_bd40, 0.75);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_f32(engine + 0x14, 0.5);
        e.call(0x00b0_8200, &args![tree]);
        assert_eq!(calls_to(&calls, 0x00b2_bd40), vec![vec![engine]]);
        assert_eq!(e.mem.f32(engine + 0x14), 0.5);
    }

    #[test]
    fn the_lod_level_comes_from_the_instance_or_the_engine() {
        let mut e = Engine::new();
        global_f32(&mut e, 0x0101_2054, -1.0);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_f32(engine + 0x14, 0.5);
        assert_eq!(e.call(0x00b0_8440, &args![tree]).f32(), 0.5);
        let data = e.mem.alloc(0x14);
        e.mem.set_f32(data + 0x10, 0.125);
        e.set(tree, CSpeedTreeRT::m_pInstanceData, Ptr::new(data));
        assert_eq!(e.call(0x00b0_8440, &args![tree]).f32(), 0.125);
    }

    #[test]
    fn the_lod_level_is_set_between_zero_and_one() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        global_f64(&mut e, 0x0101_2060, 0.0);
        global_f64(&mut e, 0x0101_2070, 1.0);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.call(0x00b0_8480, &args![tree, 0.25f32]);
        assert_eq!(e.mem.f32(engine + 0x14), 0.25);
        e.call(0x00b0_8480, &args![tree, 1.0f32]);
        assert_eq!(e.mem.f32(engine + 0x14), 1.0);
        e.call(0x00b0_8480, &args![tree, 0.0f32]);
        assert_eq!(e.mem.f32(engine + 0x14), 0.0);
        e.mem.set_f32(engine + 0x14, 0.5);
        for bad in [1.5f32, -0.5, f32::NAN] {
            e.call(0x00b0_8480, &args![tree, bad]);
        }
        assert_eq!(e.mem.f32(engine + 0x14), 0.5);
        assert_eq!(
            calls_to(&calls, STRING_ASSIGN_TEXT)
                .iter()
                .filter(|a| a[1] == 0x010a_5094)
                .count(),
            3
        );
        let data = e.mem.alloc(0x14);
        e.set(tree, CSpeedTreeRT::m_pInstanceData, Ptr::new(data));
        e.call(0x00b0_8480, &args![tree, 0.75f32]);
        assert_eq!(e.mem.f32(data + 0x10), 0.75);
        assert_eq!(e.mem.f32(engine + 0x14), 0.5);
    }

    #[test]
    fn the_discrete_leaf_flag_is_a_static_byte() {
        let mut e = Engine::new();
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        e.call(0x00b0_84f0, &args![0x1234_5601u32]);
        assert_eq!(e.mem.u8(DISCRETE_LEAF_LOD_FLAG), 1);
    }

    #[test]
    fn two_floats_go_to_the_engine() {
        let (mut e, calls) = recording(&[0x00b2_bdc0]);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.call(0x00b0_8500, &args![tree, 1.5f32, 2.5f32]);
        assert_eq!(
            calls_to(&calls, 0x00b2_bdc0),
            vec![vec![engine, 1.5f32.to_bits(), 2.5f32.to_bits()]]
        );
    }

    #[test]
    fn discrete_branch_level_scales_the_lod_level() {
        let mut e = Engine::new();
        global_f64(&mut e, 0x0101_a6b0, -1.0);
        global_f32(&mut e, 0x0101_2054, -1.0);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_u32(engine + 0x78, 4);
        // (1 - 0.25) * 4 = 3; the top of the range is pulled back by one.
        assert_eq!(e.call(0x00b0_8530, &args![tree, 0.25f32]).u16(), 3);
        assert_eq!(e.call(0x00b0_8530, &args![tree, 0.0f32]).u16(), 3);
        assert_eq!(e.call(0x00b0_8530, &args![tree, 1.0f32]).u16(), 0);
        // A level of -1 takes the tree's own (0.5: (1 - 0.5) * 4 = 2).
        e.mem.set_f32(engine + 0x14, 0.5);
        assert_eq!(e.call(0x00b0_8530, &args![tree, -1.0f32]).u16(), 2);
    }

    #[test]
    fn discrete_frond_level_uses_the_frond_level_count() {
        let mut e = Engine::new();
        global_f64(&mut e, 0x0101_a6b0, -1.0);
        global_f32(&mut e, 0x0101_2054, -1.0);
        let tree = linked_tree(&mut e);
        e.set(tree, CSpeedTreeRT::m_usNumFrondLodLevels, 2);
        assert_eq!(e.call(0x00b0_85a0, &args![tree, 0.25f32]).u16(), 1);
        assert_eq!(e.call(0x00b0_85a0, &args![tree, 0.0f32]).u16(), 1);
        assert_eq!(e.call(0x00b0_85a0, &args![tree, 1.0f32]).u16(), 0);
    }

    #[test]
    fn discrete_leaf_level_counts_one_more_with_the_flag() {
        let mut e = Engine::new();
        global_f64(&mut e, 0x0101_a6b0, -1.0);
        global_f32(&mut e, 0x0101_2054, -1.0);
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_u32(engine + 0xD8, 3);
        // Without the flag: (1 - 0.5) * 3 = 1.5 -> 1; the top is pulled back.
        assert_eq!(e.call(0x00b0_8610, &args![tree, 0.5f32]).u16(), 1);
        assert_eq!(e.call(0x00b0_8610, &args![tree, 0.0f32]).u16(), 2);
        // With the flag the count is 4: (1 - 0.5) * 4 = 2.
        e.mem.set_u8(DISCRETE_LEAF_LOD_FLAG, 1);
        assert_eq!(e.call(0x00b0_8610, &args![tree, 0.5f32]).u16(), 2);
        assert_eq!(e.call(0x00b0_8610, &args![tree, 0.0f32]).u16(), 3);
        // A level of -1 takes the tree's own (0.75: (1 - 0.75) * 4 = 1).
        e.mem.set_f32(engine + 0x14, 0.75);
        assert_eq!(e.call(0x00b0_8610, &args![tree, -1.0f32]).u16(), 1);
        // A NaN level gives the integer indefinite, whose low word is 0.
        assert_eq!(e.call(0x00b0_8610, &args![tree, f32::NAN]).u16(), 0);
    }

    #[test]
    fn the_level_counts_are_words_of_the_engine_and_the_tree() {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_u32(engine + 0x78, 0x0001_0005);
        e.mem.set_u32(engine + 0xD8, 0x0002_0007);
        e.set(tree, CSpeedTreeRT::m_usNumFrondLodLevels, 9);
        assert_eq!(e.call(0x00b0_86f0, &args![tree]).u16(), 5);
        assert_eq!(e.call(0x00b0_8710, &args![tree]).u16(), 7);
        assert_eq!(e.call(0x00b0_8730, &args![tree]).u16(), 9);
    }

    #[test]
    fn the_branch_geometry_vectors_are_cleared_when_there_is_one() {
        let (mut e, calls) = recording(&[0x00ac_2f70, 0x0044_a0a0, 0x00b0_e510]);
        let tree = linked_tree(&mut e);
        let branch = part(&e, tree, CSpeedTreeRT::m_pBranchGeometry);
        e.call(0x00b0_8750, &args![tree]);
        assert_eq!(calls_to(&calls, 0x00ac_2f70), vec![vec![branch + 0x70]]);
        // The wind vectors: the weights, then the matrix indices' clear.
        make_vector(&mut e, branch + 0x178, 0);
        e.call(0x00b0_8780, &args![tree]);
        assert_eq!(calls_to(&calls, 0x0044_a0a0), vec![vec![branch + 0x160]]);
        assert_eq!(calls_to(&calls, 0x00b0_e510).len(), 1);
        // Without a branch geometry nothing is touched.
        let bare = e.new_object::<CSpeedTreeRT>();
        e.call(0x00b0_8750, &args![bare]);
        e.call(0x00b0_8780, &args![bare]);
        assert_eq!(calls_to(&calls, 0x00ac_2f70).len(), 1);
    }

    #[test]
    fn transient_geometry_data_and_the_shadow_are_released() {
        let (mut e, calls) = recording(&[STRING_TIDY, 0x0044_a0a0]);
        let tree = linked_tree(&mut e);
        let branch = part(&e, tree, CSpeedTreeRT::m_pBranchGeometry);
        let frond = part(&e, tree, CSpeedTreeRT::m_pFrondGeometry);
        let shadow = e.mem.alloc(0x40);
        e.set(tree, CSpeedTreeRT::m_pProjectedShadow, Ptr::new(shadow));
        e.call(0x00b0_87d0, &args![tree]);
        assert_eq!(
            calls_to(&calls, STRING_TIDY),
            vec![vec![shadow + 0x24, 1, 0]]
        );
        assert!(e.get(tree, CSpeedTreeRT::m_pProjectedShadow).is_null());
        assert_eq!(
            calls_to(&calls, 0x0044_a0a0),
            vec![vec![branch + 0x148], vec![frond + 0x148]]
        );
        // Run again without the shadow: nothing more is tidied.
        e.call(0x00b0_87d0, &args![tree]);
        assert_eq!(calls_to(&calls, STRING_TIDY).len(), 1);
    }

    #[test]
    fn the_leaf_geometry_hooks_run_only_with_a_leaf_geometry() {
        let (mut e, calls) = recording(&[0x00b2_8ed0, 0x00b2_7870]);
        let tree = linked_tree(&mut e);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        e.call(0x00b0_88b0, &args![tree]);
        assert_eq!(calls_to(&calls, 0x00b2_8ed0), vec![vec![leaf]]);
        let bare = e.new_object::<CSpeedTreeRT>();
        e.call(0x00b0_88b0, &args![bare]);
        assert_eq!(calls_to(&calls, 0x00b2_8ed0).len(), 1);
    }

    #[test]
    fn the_branch_geometry_is_deleted_for_a_lone_computed_tree() {
        let (mut e, calls) = recording(&[0x00b2_e9c0]);
        let tree = linked_tree(&mut e);
        let branch = part(&e, tree, CSpeedTreeRT::m_pBranchGeometry);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        let count = part(&e, tree, CSpeedTreeRT::m_pInstanceRefCount);
        e.mem.set_u32(engine + 0x5C, branch);
        e.mem.set_u32(count, 1);
        // Not computed yet: nothing happens.
        e.call(0x00b0_88d0, &args![tree]);
        assert!(calls_to(&calls, 0x00b2_e9c0).is_empty());
        e.set(tree, CSpeedTreeRT::m_bTreeComputed, 1);
        // Shared by two: nothing happens.
        e.mem.set_u32(count, 2);
        e.call(0x00b0_88d0, &args![tree]);
        assert!(calls_to(&calls, 0x00b2_e9c0).is_empty());
        e.mem.set_u32(count, 1);
        e.call(0x00b0_88d0, &args![tree]);
        assert_eq!(calls_to(&calls, 0x00b2_e9c0), vec![vec![branch]]);
        assert!(e.get(tree, CSpeedTreeRT::m_pBranchGeometry).is_null());
        assert_eq!(e.mem.u32(engine + 0x5C), 0);
    }

    #[test]
    fn the_leaf_geometry_call_returns_its_result() {
        let (mut e, calls) = recording(&[]);
        stub(&mut e, &calls, 0x00b2_7870, 0x77);
        let tree = linked_tree(&mut e);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        assert_eq!(e.call(0x00b0_8af0, &args![tree, 5u32]).u32(), 0x77);
        assert_eq!(calls_to(&calls, 0x00b2_7870), vec![vec![leaf, 5]]);
    }

    #[test]
    fn the_bounding_box_is_copied_out() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        e.register(MEMCPY, memcpy_double);
        let tree = linked_tree(&mut e);
        let sizes = part(&e, tree, CSpeedTreeRT::m_pTreeSizes);
        for i in 0..6u32 {
            e.mem.set_f32(sizes + 4 * i, i as f32);
        }
        let out = e.mem.alloc(0x18);
        e.call(0x00b0_8c80, &args![tree, out]);
        assert_eq!(e.mem.f32(out + 0x14), 5.0);
        e.call(0x00b0_8c80, &args![tree, 0u32]);
        assert!(error_was(&calls, 0x010a_5118));
    }

    #[test]
    fn the_leaf_static_lighting_uses_the_box_centre() {
        let (mut e, calls) = recording(&[]);
        e.register(MEMCPY, memcpy_double);
        let log = calls.clone();
        e.register_double(0x00b2_a210, move |e, a| {
            let centre = [0, 4, 8].map(|offset| e.mem.u32(a[1] + offset));
            log.borrow_mut().push((
                0x00b2_a210,
                vec![a[0], centre[0], centre[1], centre[2], a[2], a[3]],
            ));
            Ret::default()
        });
        global_f64(&mut e, 0x0101_1588, 0.5);
        let tree = linked_tree(&mut e);
        let sizes = part(&e, tree, CSpeedTreeRT::m_pTreeSizes);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        let lighting = part(&e, tree, CSpeedTreeRT::m_pLightingEngine);
        for (i, value) in [1.0f32, 2.0, 3.0, 5.0, 8.0, 11.0].into_iter().enumerate() {
            e.mem.set_f32(sizes + 4 * i as u32, value);
        }
        e.mem.set_u32(engine + 0xEC, 0x5000);
        e.mem.set_u32(engine + 0xD8, 3);
        e.call(0x00b0_8cc0, &args![tree]);
        assert_eq!(
            calls_to(&calls, 0x00b2_a210),
            vec![vec![
                lighting,
                3.0f32.to_bits(),
                5.0f32.to_bits(),
                7.0f32.to_bits(),
                0x5000,
                3
            ]]
        );
    }

    #[test]
    fn set_error_stores_the_message() {
        let (mut e, calls) = recording(&[STRING_ASSIGN_TEXT]);
        stub(&mut e, &calls, STRLEN, 7);
        e.call(0x00b0_9270, &args![0x010a_5000u32]);
        assert_eq!(calls_to(&calls, STRLEN), vec![vec![0x010a_5000]]);
        assert_eq!(
            calls_to(&calls, STRING_ASSIGN_TEXT),
            vec![vec![ERROR_STRING, 0x010a_5000, 7]]
        );
    }

    /// The string helpers as doubles that move whole 0x1C-byte string objects: `assign(this,
    /// source, ..)` and the copy constructor copy the source object, `string(this, text)` is
    /// recorded with the text it was given; the others are recorded.
    fn string_doubles(e: &mut Engine, calls: &Calls) {
        e.map(STRING_NPOS, 4);
        e.mem.set_u32(STRING_NPOS, 0xFFFF_FFFF);
        for address in [STRING_TIDY, EXCEPTION_DEFAULT_CONSTRUCT, EXCEPTION_DESTRUCT] {
            stub(e, calls, address, 0);
        }
        for address in [STRING_ASSIGN, STRING_COPY] {
            let log = calls.clone();
            e.register_double(address, move |e, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let bytes = e.mem.bytes(a[1], 0x1C);
                e.mem.write(a[0], &bytes);
                Ret::default()
            });
        }
        let log = calls.clone();
        e.register_double(STRING_FROM_C_STRING, move |e, a| {
            let mut words = a.to_vec();
            let length = if e.mem.is_mapped(a[1]) {
                e.mem.cstr(a[1]).len()
            } else {
                0
            };
            words.push(length as u32);
            log.borrow_mut().push((STRING_FROM_C_STRING, words));
            Ret::default()
        });
    }

    /// `strcpy(destination, source)` over C strings.
    fn strcpy_double(e: &mut Engine, a: &[u32]) -> Ret {
        let mut text = e.mem.cstr(a[1]);
        text.push(0);
        e.mem.write(a[0], &text);
        Ret::default()
    }

    /// Writes `text` (under 16 bytes) as the inline text of the `std::string` at `string`.
    fn small_string(e: &mut Engine, string: u32, text: &str) {
        e.mem.write(string + 4, text.as_bytes());
        e.mem.set_u32(string + 0x14, text.len() as u32);
        e.mem.set_u32(string + 0x18, 0xF);
    }

    #[test]
    fn lod_info_chunk_sets_the_leaf_lod_settings() {
        let (mut e, calls) = recording(&[0x00b2_ceb0]);
        let tags = vec![9002, 3, 9003, 9004, 9009, 9005, 9001];
        file_doubles(&mut e, tags, vec![1.5, 2.5, 3.5]);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        let file = access(&mut e, 64);
        e.call(0x00b0_92a0, &args![tree, file]);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_eLeafLodMethod), 3);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_fLeafLodTransitionRadius), 1.5);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_fLeafLodCurveExponent), 2.5);
        assert_eq!(e.get(tree, CSpeedTreeRT::m_fLeafSizeIncreaseFactor), 3.5);
        assert_eq!(calls_to(&calls, 0x00b2_ceb0), vec![vec![engine, file]]);
    }

    #[test]
    fn lod_info_chunk_throws_on_an_unknown_tag_and_on_a_short_file() {
        let (mut e, calls) = recording(&[]);
        parse_error_doubles(&mut e, &calls);
        file_doubles(&mut e, vec![9006], vec![]);
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 64);
        assert!(throws(|| {
            e.call(0x00b0_92a0, &args![tree, file]);
        }));
        let made = calls_to(&calls, STRING_FROM_C_STRING);
        assert_eq!(made[0][1], 0x010a_51c8);
        // A file that ends after the first entry.
        let (mut e, calls) = recording(&[]);
        parse_error_doubles(&mut e, &calls);
        file_doubles(&mut e, vec![9002, 1], vec![]);
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 2);
        assert!(throws(|| {
            e.call(0x00b0_92a0, &args![tree, file]);
        }));
        assert_eq!(calls_to(&calls, STRING_FROM_C_STRING)[0][1], 0x010a_5194);
    }

    /// `operator+` of the string helpers as doubles returning their result slot.
    fn plus_doubles(e: &mut Engine, calls: &Calls) {
        for address in [STRING_PLUS_TEXT, STRING_PLUS_STRING] {
            let log = calls.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret {
                    eax: a[0],
                    ..Ret::default()
                }
            });
        }
    }

    /// The doubles of throwing a parse error: the string helpers, the concatenations and the
    /// throw.
    fn parse_error_doubles(e: &mut Engine, calls: &Calls) {
        string_doubles(e, calls);
        plus_doubles(e, calls);
        throw_doubles(e, calls);
    }

    #[test]
    fn the_parse_error_message_joins_the_text_and_the_reason() {
        let (mut e, calls) = recording(&[]);
        parse_error_doubles(&mut e, &calls);
        let reason = e.mem.alloc(8);
        e.mem.write(reason, b"oops");
        stub(&mut e, &calls, STRERROR, reason);
        let object = e.mem.alloc(0x28);
        let message = e.mem.alloc(0x1C);
        assert_eq!(
            e.call(0x00b0_9480, &args![object, message, 1u32]).u32(),
            object
        );
        assert_eq!(e.mem.u32(object), 0x010a_51e0);
        // The errno text is built first, then message + " [" + reason + "]".
        let from = calls_to(&calls, STRING_FROM_C_STRING);
        assert_eq!(from.len(), 1);
        assert_eq!(from[0][1], reason);
        let errno_text = from[0][0];
        let first = calls_to(&calls, STRING_PLUS_TEXT);
        assert_eq!(first.len(), 2);
        assert_eq!((first[0][1], first[0][2]), (message, 0x010a_51e8));
        let second = calls_to(&calls, STRING_PLUS_STRING);
        assert_eq!((second[0][1], second[0][2]), (first[0][0], errno_text));
        assert_eq!((first[1][1], first[1][2]), (second[0][0], 0x0106_3e18));
        // The base constructor got the final string; the four temporaries are tidied away.
        let base = calls_to(&calls, STRING_ASSIGN);
        assert_eq!(base[0][..2], [object + 0xC, first[1][0]]);
        let tidy: Vec<Vec<u32>> = calls_to(&calls, STRING_TIDY)
            .into_iter()
            .filter(|a| a[1] == 1)
            .collect();
        assert_eq!(
            tidy.iter().map(|a| a[0]).collect::<Vec<_>>(),
            vec![first[1][0], second[0][0], first[0][0], errno_text]
        );
    }

    #[test]
    fn the_parse_error_without_errno_uses_the_empty_reason() {
        let (mut e, calls) = recording(&[]);
        parse_error_doubles(&mut e, &calls);
        e.map(0x0101_1584, 4);
        let object = e.mem.alloc(0x28);
        let message = e.mem.alloc(0x1C);
        e.call(0x00b0_9480, &args![object, message, 0u32]);
        let from = calls_to(&calls, STRING_FROM_C_STRING);
        assert_eq!((from.len(), from[0][1]), (1, 0x0101_1584));
        assert!(calls_to(&calls, ERRNO).is_empty());
        let second = calls_to(&calls, STRING_PLUS_STRING);
        assert_eq!(second[0][2], from[0][0]);
    }

    #[test]
    fn the_parse_error_base_holds_its_message() {
        let (mut e, calls) = recording(&[]);
        string_doubles(&mut e, &calls);
        let object = e.mem.alloc(0x28);
        let message = e.mem.alloc(0x1C);
        assert_eq!(e.call(0x00b0_96a0, &args![object, message]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x010a_307c);
        assert_eq!(
            calls_to(&calls, EXCEPTION_DEFAULT_CONSTRUCT),
            vec![vec![object]]
        );
        assert_eq!(
            calls_to(&calls, STRING_TIDY),
            vec![vec![object + 0xC, 0, 0]]
        );
        assert_eq!(
            calls_to(&calls, STRING_ASSIGN),
            vec![vec![object + 0xC, message, 0, 0xFFFF_FFFF]]
        );
    }

    #[test]
    fn the_parse_error_base_destructors() {
        let (mut e, calls) = recording(&[]);
        string_doubles(&mut e, &calls);
        let object = e.mem.alloc(0x28);
        assert_eq!(e.call(0x00b0_9730, &args![object, 0u32]).u32(), object);
        assert_eq!(e.mem.u32(object), 0x010a_307c);
        assert_eq!(
            calls_to(&calls, STRING_TIDY),
            vec![vec![object + 0xC, 1, 0]]
        );
        assert_eq!(calls_to(&calls, EXCEPTION_DESTRUCT), vec![vec![object]]);
        assert!(e.mem.block_size(object).is_some());
        // Bit 0 of the flags frees the block as well.
        e.call(0x00b0_9730, &args![object, 1u32]);
        assert!(e.mem.block_size(object).is_none());
        // The plain destructor does not.
        let second = e.mem.alloc(0x28);
        e.call(0x00b0_9780, &args![second]);
        assert!(e.mem.block_size(second).is_some());
        assert_eq!(calls_to(&calls, EXCEPTION_DESTRUCT).len(), 3);
    }

    #[test]
    fn the_parse_error_copy_sets_the_derived_vtable() {
        let (mut e, calls) = recording(&[EXCEPTION_COPY_CONSTRUCT]);
        let (object, other) = (e.mem.alloc(0x28), e.mem.alloc(0x28));
        assert_eq!(e.call(0x00b0_97b0, &args![object, other]).u32(), object);
        assert_eq!(
            calls_to(&calls, EXCEPTION_COPY_CONSTRUCT),
            vec![vec![object, other]]
        );
        assert_eq!(e.mem.u32(object), 0x010a_51e0);
    }

    #[test]
    fn wind_info_chunk_sets_the_wind_level() {
        let (mut e, _) = recording(&[]);
        file_doubles(&mut e, vec![11002, 5, 11001], vec![]);
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        let file = access(&mut e, 64);
        e.call(0x00b0_97e0, &args![tree, file]);
        assert_eq!(e.mem.u32(engine + 0x108), 5);
    }

    #[test]
    fn wind_info_chunk_throws_on_an_unknown_tag_and_a_short_file() {
        for (tags, length, message) in [
            (vec![11003], 64, 0x010a_5220),
            (vec![11002, 5], 2, 0x010a_51ec),
        ] {
            let (mut e, calls) = recording(&[]);
            parse_error_doubles(&mut e, &calls);
            string_doubles(&mut e, &calls);
            file_doubles(&mut e, tags, vec![]);
            let tree = linked_tree(&mut e);
            let file = access(&mut e, length);
            assert!(throws(|| {
                e.call(0x00b0_97e0, &args![tree, file]);
            }));
            assert_eq!(calls_to(&calls, STRING_FROM_C_STRING)[0][1], message);
        }
    }

    #[test]
    fn embedded_coordinates_chunk_reads_maps_and_flags() {
        let (mut e, calls) = recording(&[STRING_TIDY]);
        let tags = vec![10002, 2, 10003, 1, 10004, 1, 10006, 10007, 10001];
        let floats = (1..=32).map(f64::from).collect();
        file_doubles(&mut e, tags, floats);
        global(&mut e, FROND_TEXTURE_FLAG, 1);
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 80);
        let bytes = e.mem.u32(file + 0x10);
        for offset in 0..80 {
            e.mem.set_u8(bytes + offset, 1);
        }
        e.call(0x00b0_9960, &args![tree, file]);
        let record = e.get(tree, CSpeedTreeRT::m_pEmbeddedTexCoords).addr();
        assert_ne!(record, 0);
        assert_eq!(
            calls_to(&calls, STRING_TIDY),
            vec![vec![record + 0x18, 0, 0]]
        );
        assert_eq!(
            (
                e.mem.i32(record),
                e.mem.i32(record + 8),
                e.mem.i32(record + 0x10)
            ),
            (2, 1, 1)
        );
        let read = |e: &Engine, array: u32, count: u32| -> Vec<f32> {
            (0..count).map(|i| e.mem.f32(array + 4 * i)).collect()
        };
        let leaf = e.mem.u32(record + 4);
        assert_eq!(
            read(&e, leaf, 16),
            (1..=16).map(|i| i as f32).collect::<Vec<_>>()
        );
        // The billboard maps negate their odd entries while the flag is set.
        let billboards = e.mem.u32(record + 0xC);
        assert_eq!(
            read(&e, billboards, 8),
            vec![17.0, -18.0, 19.0, -20.0, 21.0, -22.0, 23.0, -24.0]
        );
        let fronds = e.mem.u32(record + 0x14);
        assert_eq!(
            read(&e, fronds, 8),
            (25..=32).map(|i| i as f32).collect::<Vec<_>>()
        );
        assert_eq!(
            (e.mem.u8(tree.addr() + 0x6D), e.mem.u8(tree.addr() + 0x6C)),
            (1, 1)
        );
    }

    #[test]
    fn embedded_coordinates_keep_the_billboards_when_the_flag_is_clear() {
        let (mut e, _) = recording(&[STRING_TIDY]);
        file_doubles(
            &mut e,
            vec![10003, 1, 10001],
            (1..=8).map(f64::from).collect(),
        );
        global(&mut e, FROND_TEXTURE_FLAG, 0);
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 80);
        e.call(0x00b0_9960, &args![tree, file]);
        let record = e.get(tree, CSpeedTreeRT::m_pEmbeddedTexCoords).addr();
        let billboards = e.mem.u32(record + 0xC);
        assert_eq!(e.mem.f32(billboards + 4), 2.0);
    }

    #[test]
    fn embedded_coordinates_chunk_reads_the_composite_name_without_its_path() {
        let (mut e, calls) = recording(&[]);
        string_doubles(&mut e, &calls);
        e.register(STRCPY, strcpy_double);
        file_doubles(&mut e, vec![10005, 10001], vec![]);
        global(&mut e, FROND_TEXTURE_FLAG, 0);
        e.register_double(PARSE_STRING, |e, a| {
            small_string(e, a[1], "dir/x.dds");
            Ret::default()
        });
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 80);
        e.call(0x00b0_9960, &args![tree, file]);
        let record = e.get(tree, CSpeedTreeRT::m_pEmbeddedTexCoords).addr();
        // The reduced name is built from the copy's text with the path cut off.
        let made = calls_to(&calls, STRING_FROM_C_STRING);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][2], 5);
        let assigned = calls_to(&calls, STRING_ASSIGN);
        // Into the record's string last: (record + 0x18, reduced name, 0, npos).
        let last = assigned.last().unwrap();
        assert_eq!((last[0], last[2], last[3]), (record + 0x18, 0, 0xFFFF_FFFF));
    }

    #[test]
    fn embedded_coordinates_chunk_throws_on_an_unknown_tag() {
        let (mut e, calls) = recording(&[STRING_TIDY]);
        parse_error_doubles(&mut e, &calls);
        file_doubles(&mut e, vec![10008], vec![]);
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 80);
        assert!(throws(|| {
            e.call(0x00b0_9960, &args![tree, file]);
        }));
        assert_eq!(calls_to(&calls, STRING_FROM_C_STRING)[0][1], 0x010a_5278);
    }

    #[test]
    fn embedded_coordinates_chunk_throws_when_the_file_ends() {
        let (mut e, calls) = recording(&[STRING_TIDY]);
        parse_error_doubles(&mut e, &calls);
        file_doubles(&mut e, vec![10006], vec![]);
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 2);
        assert!(throws(|| {
            e.call(0x00b0_9960, &args![tree, file]);
        }));
        assert_eq!(calls_to(&calls, STRING_FROM_C_STRING)[0][1], 0x010a_5238);
    }

    #[test]
    fn a_string_is_built_from_a_by_value_argument() {
        let (mut e, calls) = recording(&[]);
        string_doubles(&mut e, &calls);
        let this = e.mem.alloc(0x1C);
        let words = [1u32, 2, 3, 4, 5, 6, 7];
        let copy = e.call(
            0x00b0_9ef0,
            &args![this, words[0], words[1], words[2], words[3], words[4], words[5], words[6]],
        );
        assert_eq!(copy.u32(), this);
        assert_eq!(
            [0, 4, 8, 12, 16, 20, 24].map(|offset| e.mem.u32(this + offset)),
            words
        );
        let tidy = calls_to(&calls, STRING_TIDY);
        assert_eq!(tidy[0], vec![this, 0, 0]);
        assert_eq!((tidy[1][1], tidy[1][2]), (1, 0));
        let assigned = calls_to(&calls, STRING_ASSIGN);
        assert_eq!(
            (assigned[0][0], assigned[0][2], assigned[0][3]),
            (this, 0, 0xFFFF_FFFF)
        );
        assert_eq!(assigned[0][1], tidy[1][0]);
    }

    #[test]
    fn a_string_destructor_tidies_the_string() {
        let (mut e, calls) = recording(&[STRING_TIDY]);
        e.call(0x00b0_9f70, &args![0x1234u32]);
        assert_eq!(calls_to(&calls, STRING_TIDY), vec![vec![0x1234, 1, 0]]);
    }

    /// Runs `NoPath` on `name` and returns the text it handed to the string constructor.
    fn reduced_name(name: &str) -> String {
        let (mut e, calls) = recording(&[]);
        string_doubles(&mut e, &calls);
        e.register(STRCPY, strcpy_double);
        let log = calls.clone();
        e.register_double(STRING_FROM_C_STRING, move |e, a| {
            log.borrow_mut()
                .push((STRING_FROM_C_STRING, vec![a[0], a[1], 0]));
            let text = e.mem.cstr(a[1]);
            e.mem.write(a[0] + 4, &text);
            Ret::default()
        });
        let string = e.mem.alloc(0x1C);
        small_string(&mut e, string, name);
        let result = e.mem.alloc(0x1C);
        assert_eq!(e.call(0x00b0_9f90, &args![string, result]).u32(), result);
        let tidy = calls_to(&calls, STRING_TIDY);
        assert_eq!(tidy[0], vec![result, 0, 0]);
        let assign = calls_to(&calls, STRING_ASSIGN);
        assert_eq!(
            (assign[0][0], assign[0][2], assign[0][3]),
            (result, 0, 0xFFFF_FFFF)
        );
        String::from_utf8(e.mem.cstr(result + 4)).unwrap()
    }

    #[test]
    fn a_file_name_loses_its_directory() {
        assert_eq!(reduced_name("dir/x.dds"), "x.dds");
        assert_eq!(reduced_name("a\\b.dds"), "b.dds");
        assert_eq!(reduced_name("plain.dds"), "plain.dds");
    }

    #[test]
    fn an_index_past_the_string_takes_the_invalid_parameter_path() {
        let (mut e, calls) = recording(&[INVALID_PARAMETER]);
        let string = e.mem.alloc(0x1C);
        e.mem.set_u32(string + 0x14, 2);
        check_string_index(&mut e, string, 2);
        assert!(calls_to(&calls, INVALID_PARAMETER).is_empty());
        check_string_index(&mut e, string, 3);
        assert_eq!(calls_to(&calls, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn the_collision_object_count_is_the_shape_vector_size() {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        assert_eq!(e.call(0x00b0_a260, &args![tree]).u32(), 0);
        let (vector, _) = shapes(&mut e, &[0, 1, 2]);
        e.set(tree, CSpeedTreeRT::m_pCollisionObjects, Ptr::new(vector));
        assert_eq!(e.call(0x00b0_a260, &args![tree]).u32(), 3);
    }

    #[test]
    fn a_collision_object_is_copied_out() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        e.register(MEMCPY, memcpy_double);
        let tree = linked_tree(&mut e);
        let (vector, storage) = shapes(&mut e, &[0, 2]);
        e.set(tree, CSpeedTreeRT::m_pCollisionObjects, Ptr::new(vector));
        for (i, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].into_iter().enumerate() {
            e.mem.set_f32(storage + 0x1C + 4 + 4 * i as u32, value);
        }
        let (kind, position, dimensions) = (e.mem.alloc(4), e.mem.alloc(0xC), e.mem.alloc(0xC));
        e.call(0x00b0_a2a0, &args![tree, 1u32, kind, position, dimensions]);
        assert_eq!(e.mem.u32(kind), 2);
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(position + offset)),
            [1.0, 2.0, 3.0]
        );
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(dimensions + offset)),
            [4.0, 5.0, 6.0]
        );
    }

    #[test]
    fn a_collision_object_index_out_of_range_is_reported_with_the_count() {
        let (mut e, calls) = recording(&[STRING_TIDY]);
        error_doubles(&mut e, &calls);
        stub(&mut e, &calls, FORMAT_STRING, 0);
        let tree = linked_tree(&mut e);
        let (vector, _) = shapes(&mut e, &[0, 2]);
        e.set(tree, CSpeedTreeRT::m_pCollisionObjects, Ptr::new(vector));
        let out = e.mem.alloc(4);
        e.call(0x00b0_a2a0, &args![tree, 2u32, out, out, out]);
        let format = calls_to(&calls, FORMAT_STRING);
        assert_eq!(format[0][1..], [0x010a_52c8, 2, 2]);
        // The message text is the formatted string's inline buffer.
        assert_eq!(calls_to(&calls, STRLEN)[0][0], format[0][0] + 4);
        assert_eq!(calls_to(&calls, STRING_TIDY).len(), 1);
        assert!(calls_to(&calls, STRING_ASSIGN_TEXT)
            .iter()
            .all(|a| a[0] == ERROR_STRING));
    }

    #[test]
    fn a_collision_object_without_objects_is_an_error() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        e.call(0x00b0_a2a0, &args![tree, 0u32, 0u32, 0u32, 0u32]);
        assert!(error_was(&calls, 0x010a_5298));
    }

    /// The doubles reading collision objects needs: the vector's proxy allocation and
    /// initialisation, and the insert that records the shape it was given.
    fn collision_doubles(e: &mut Engine, calls: &Calls) {
        e.register_double(0x0044_d670, |e, _| Ret {
            eax: e.mem.alloc(8),
            ..Ret::default()
        });
        stub(e, calls, 0x00b0_e9c0, 0);
        let log = calls.clone();
        e.register_double(0x00b0_e880, move |e, a| {
            let mut words = vec![a[0]];
            words.extend((0..7).map(|i| e.mem.u32(a[4] + 4 * i)));
            log.borrow_mut().push((0x00b0_e880, words));
            Ret::default()
        });
    }

    #[test]
    fn collision_objects_are_read_shape_by_shape() {
        let (mut e, calls) = recording(&[]);
        collision_doubles(&mut e, &calls);
        let tags = vec![0x2EE2, 0x2EE4, 0x2EE1];
        let floats = (1..=10).map(f64::from).collect();
        file_doubles(&mut e, tags, floats);
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 64);
        e.call(0x00b0_a490, &args![tree, file]);
        let objects = e.get(tree, CSpeedTreeRT::m_pCollisionObjects);
        assert!(!objects.is_null());
        let inserted = calls_to(&calls, 0x00b0_e880);
        assert_eq!(inserted.len(), 2);
        let word = |value: f32| value.to_bits();
        // A sphere: the position, one dimension; the others stay 0.
        assert_eq!(
            inserted[0][1..],
            [0, word(1.0), word(2.0), word(3.0), word(4.0), 0, 0]
        );
        // A box: all three.
        assert_eq!(
            inserted[1][1..],
            [
                2,
                word(5.0),
                word(6.0),
                word(7.0),
                word(8.0),
                word(9.0),
                word(10.0)
            ]
        );
        assert_eq!(inserted[0][0], objects.addr());
    }

    #[test]
    fn collision_object_chunk_throws_on_a_bad_shape_or_a_short_file() {
        for (tags, length, message) in [
            (vec![0x2EE5], 64, 0x010a_535c),
            (vec![0x2EE2], 4, 0x010a_5300),
        ] {
            let (mut e, calls) = recording(&[]);
            collision_doubles(&mut e, &calls);
            parse_error_doubles(&mut e, &calls);
            file_doubles(&mut e, tags, vec![]);
            let tree = linked_tree(&mut e);
            let file = access(&mut e, length);
            assert!(throws(|| {
                e.call(0x00b0_a490, &args![tree, file]);
            }));
            assert_eq!(calls_to(&calls, STRING_FROM_C_STRING)[0][1], message);
        }
    }

    #[test]
    fn the_frond_geometry_is_deleted_for_a_lone_computed_tree() {
        let (mut e, calls) = recording(&[0x00b2_e9c0]);
        let tree = linked_tree(&mut e);
        let frond = part(&e, tree, CSpeedTreeRT::m_pFrondGeometry);
        let count = part(&e, tree, CSpeedTreeRT::m_pInstanceRefCount);
        e.mem.set_u32(count, 1);
        e.call(0x00b0_a7a0, &args![tree]);
        assert!(calls_to(&calls, 0x00b2_e9c0).is_empty());
        e.set(tree, CSpeedTreeRT::m_bTreeComputed, 1);
        e.mem.set_u32(count, 3);
        e.call(0x00b0_a7a0, &args![tree]);
        assert!(calls_to(&calls, 0x00b2_e9c0).is_empty());
        e.mem.set_u32(count, 1);
        e.call(0x00b0_a7a0, &args![tree]);
        assert_eq!(calls_to(&calls, 0x00b2_e9c0), vec![vec![frond]]);
        assert!(e.get(tree, CSpeedTreeRT::m_pFrondGeometry).is_null());
        // Without a shared count nothing is tested or freed.
        let other = linked_tree(&mut e);
        e.set(other, CSpeedTreeRT::m_bTreeComputed, 1);
        e.set(other, CSpeedTreeRT::m_pInstanceRefCount, Ptr::NULL);
        e.call(0x00b0_a7a0, &args![other]);
        assert!(!e.get(other, CSpeedTreeRT::m_pFrondGeometry).is_null());
    }

    /// A geometry with two vector entries at `offset`: entry 0 holds `first` bytes of
    /// elements, entry 1 `second` bytes.
    fn geometry_with_vectors(e: &mut Engine, offset: u32, first: u32, second: u32) -> u32 {
        let geometry = e.mem.alloc(0x200);
        let storage = make_vector(e, geometry + offset, 0x30);
        for (i, bytes) in [first, second].into_iter().enumerate() {
            let entry = storage + 0x18 * i as u32;
            let elements = e.mem.alloc(bytes.max(8));
            e.mem.set_u32(entry + 0x0C, elements);
            e.mem.set_u32(entry + 0x10, elements + bytes);
        }
        geometry
    }

    #[test]
    fn the_strip_count_of_a_level() {
        let (mut e, calls) = recording(&[INVALID_PARAMETER]);
        let geometry = geometry_with_vectors(&mut e, 0x58, 8, 12);
        // Mapped memory after the vectors for the out-of-range read of the last case.
        let _guard = e.mem.alloc(0x100);
        assert_eq!(e.call(0x00b0_a9a0, &args![geometry, 1i16]).u16(), 3);
        assert_eq!(e.call(0x00b0_a9a0, &args![geometry, 0i16]).u16(), 2);
        assert_eq!(e.call(0x00b0_a9a0, &args![geometry, -1i16]).u16(), 0);
        assert!(calls_to(&calls, INVALID_PARAMETER).is_empty());
        // A level past the vector takes the invalid-parameter path.
        e.call(0x00b0_a9a0, &args![geometry, 2i16]);
        assert_eq!(calls_to(&calls, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn the_strip_lengths_of_a_level() {
        let (mut e, calls) = recording(&[INVALID_PARAMETER]);
        let geometry = geometry_with_vectors(&mut e, 0x40, 6, 0);
        e.mem.set_u16(geometry + 0x20, 2);
        let first = e.mem.u32(e.mem.u32(geometry + 0x40 + 0x0C) + 0x0C);
        assert_eq!(e.call(0x00b0_aa20, &args![geometry, 0i16]).u32(), first);
        // An empty level, a level past the count and a negative level give null.
        assert_eq!(e.call(0x00b0_aa20, &args![geometry, 1i16]).u32(), 0);
        assert_eq!(e.call(0x00b0_aa20, &args![geometry, 2i16]).u32(), 0);
        assert_eq!(e.call(0x00b0_aa20, &args![geometry, -1i16]).u32(), 0);
        assert!(calls_to(&calls, INVALID_PARAMETER).is_empty());
    }

    #[test]
    fn the_geometry_record_is_filled_for_the_chosen_parts() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        let tree = linked_tree(&mut e);
        e.set(tree, CSpeedTreeRT::m_pBranchGeometry, Ptr::NULL);
        e.set(tree, CSpeedTreeRT::m_pFrondGeometry, Ptr::NULL);
        e.set(tree, CSpeedTreeRT::m_b360Billboard, 1);
        let out = e.mem.alloc(0x130);
        for offset in [0xF0, 0x104, 0x118] {
            e.mem.set_u8(out + offset, 0xFF);
        }
        // Branches and fronds with no geometry report errors; the 360-degree billboard of an
        // engine without the discrete flag is empty.
        e.call(0x00b0_ab20, &args![tree, out, 0x0Bu32, 0u16, 0u16, 0u16]);
        assert!(error_was(&calls, 0x010a_53c0));
        assert!(error_was(&calls, 0x010a_5408));
        assert_eq!(
            [0xF0, 0x104, 0x118].map(|offset| e.mem.u8(out + offset)),
            [0, 0, 0]
        );
        // Bit 4 sends the billboard to the plain form too.
        for offset in [0xF0, 0x104, 0x118] {
            e.mem.set_u8(out + offset, 0xFF);
        }
        e.call(0x00b0_ab20, &args![tree, out, 0x18u32, 0u16, 0u16, 0u16]);
        assert_eq!(e.mem.u8(out + 0xF0), 0);
        // Nothing requested, nothing touched.
        e.mem.set_u8(out + 0xF0, 0x55);
        e.call(0x00b0_ab20, &args![tree, out, 0u32, 0u16, 0u16, 0u16]);
        assert_eq!(e.mem.u8(out + 0xF0), 0x55);
    }

    #[test]
    fn a_vector_of_collision_shapes_is_constructed_with_a_proxy() {
        let (mut e, calls) = recording(&[0x00b0_e9c0]);
        e.register_double(0x0044_d670, |e, _| Ret {
            eax: e.mem.alloc(8),
            ..Ret::default()
        });
        let vector = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00b0_dfb0, &args![vector]).u32(), vector);
        let proxy = e.mem.u32(vector);
        assert_ne!(proxy, 0);
        assert_eq!(e.mem.u32(proxy), vector);
        assert_eq!(calls_to(&calls, 0x00b0_e9c0), vec![vec![vector, 0]]);
    }

    #[test]
    fn a_failed_proxy_allocation_leaves_the_vector_without_one() {
        let (mut e, _) = recording(&[0x00b0_e9c0, 0x0044_d670]);
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector, 0xFFFF);
        e.call(0x00b0_dfb0, &args![vector]);
        assert_eq!(e.mem.u32(vector), 0);
    }

    #[test]
    fn clearing_a_vector_erases_its_whole_range() {
        let (mut e, calls) = recording(&[0x00b0_e510]);
        let vector = e.mem.alloc(0x18);
        let storage = make_vector(&mut e, vector, 8);
        let proxy = e.mem.u32(vector);
        e.call(0x00b0_dc10, &args![vector]);
        let seen = calls_to(&calls, 0x00b0_e510);
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][0], vector);
        assert_eq!(seen[0][2..], [proxy, storage, proxy, storage + 8]);
    }

    #[test]
    fn a_collision_shape_is_pushed_in_place_or_inserted() {
        let (mut e, calls) = recording(&[0x00b1_0990, 0x00b0_e880]);
        let vector = e.mem.alloc(0x18);
        let storage = make_vector(&mut e, vector, 0x1C);
        let proxy = e.mem.u32(vector);
        let value = e.mem.alloc(0x1C);
        // Full: the insert gets a checked end iterator.
        e.call(0x00b0_e090, &args![vector, value]);
        let insert = calls_to(&calls, 0x00b0_e880);
        assert_eq!(insert.len(), 1);
        assert_eq!(insert[0][0], vector);
        assert_eq!(insert[0][2..], [proxy, storage + 0x1C, value]);
        assert!(calls_to(&calls, 0x00b1_0990).is_empty());
        // Room for two more: the shape is constructed at the end, which moves on by 0x1C.
        e.mem.set_u32(vector + 0x14, storage + 3 * 0x1C);
        e.call(0x00b0_e090, &args![vector, value]);
        assert_eq!(
            calls_to(&calls, 0x00b1_0990),
            vec![vec![storage + 0x1C, 1, value, vector + 8, 0, 0]]
        );
        assert_eq!(e.mem.u32(vector + 0x10), storage + 0x38);
        assert_eq!(calls_to(&calls, 0x00b0_e880).len(), 1);
        // An empty vector without storage has no capacity either.
        let bare = e.mem.alloc(0x18);
        let bare_proxy = e.mem.alloc(8);
        e.mem.set_u32(bare, bare_proxy);
        e.call(0x00b0_e090, &args![bare, value]);
        assert_eq!(calls_to(&calls, 0x00b0_e880).len(), 2);
    }

    #[test]
    fn vectors_are_added_and_scaled_componentwise() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xC);
        let other = e.mem.alloc(0xC);
        let result = e.mem.alloc(0xC);
        for (i, (a, b)) in [(1.0f32, 10.0f32), (2.0, 20.0), (3.0, 30.0)]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(this + 4 * i as u32, a);
            e.mem.set_f32(other + 4 * i as u32, b);
        }
        assert_eq!(
            e.call(0x00b0_cf90, &args![this, result, other]).u32(),
            result
        );
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(result + offset)),
            [11.0, 22.0, 33.0]
        );
        assert_eq!(
            e.call(0x00b0_cff0, &args![this, result, 4.0f32]).u32(),
            result
        );
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(result + offset)),
            [4.0, 8.0, 12.0]
        );
        // The result may be one of the operands: all sums are formed before any is stored.
        e.call(0x00b0_cf90, &args![this, this, other]);
        assert_eq!(e.mem.f32(this + 8), 33.0);
    }

    /// Checks a front-of-vector getter: the address of the first element of the vector at
    /// `offset` (of `1 << shift` byte elements) of a geometry, null when it is empty.
    fn check_front_getter(address: u32, offset: u32) {
        let (mut e, calls) = recording(&[INVALID_PARAMETER]);
        let geometry = e.mem.alloc(0x200);
        assert_eq!(e.call(address, &args![geometry]).u32(), 0);
        let storage = make_vector(&mut e, geometry + offset, 0x10);
        assert_eq!(e.call(address, &args![geometry]).u32(), storage);
        make_vector(&mut e, geometry + offset, 0);
        assert_eq!(e.call(address, &args![geometry]).u32(), 0);
        assert!(calls_to(&calls, INVALID_PARAMETER).is_empty());
    }

    #[test]
    fn the_first_colour_of_a_geometry() {
        check_front_getter(0x00b0_b4b0, 0x70);
    }

    #[test]
    fn the_first_coordinate_of_a_geometry() {
        check_front_getter(0x00b0_b530, 0x88);
    }

    #[test]
    fn the_first_normal_of_a_geometry() {
        check_front_getter(0x00b0_b5b0, 0xB8);
    }

    #[test]
    fn the_first_binormal_of_a_geometry() {
        check_front_getter(0x00b0_b630, 0xD0);
    }

    #[test]
    fn the_first_tangent_of_a_geometry() {
        check_front_getter(0x00b0_b6b0, 0xE8);
    }

    #[test]
    fn the_first_texture_coordinate_of_a_geometry() {
        check_front_getter(0x00b0_b730, 0x100);
    }

    #[test]
    fn the_first_second_set_coordinate_of_a_geometry() {
        check_front_getter(0x00b0_b7b0, 0x148);
    }

    #[test]
    fn the_first_wind_weight_of_a_geometry() {
        check_front_getter(0x00b0_b830, 0x160);
    }

    #[test]
    fn the_first_wind_matrix_index_of_a_geometry() {
        check_front_getter(0x00b0_b8b0, 0x178);
    }

    /// A geometry with every vector the getters read: colours (+0x70), coordinates (+0x88, four
    /// vertices), normals, binormals, tangents, texture coordinates, the second set, wind
    /// weights and the byte vector of matrix indices, two levels of strips (+0x58: 2 and 3
    /// strips) and strip lengths (+0x40: 3 and 2 entries) with a level count of 2. Returns the
    /// geometry and the storage of its coordinates.
    fn full_geometry(e: &mut Engine) -> (u32, u32) {
        let geometry = geometry_with_vectors(e, 0x58, 8, 12);
        let lengths = make_vector(e, geometry + 0x40, 0x30);
        for (i, bytes) in [6u32, 4].into_iter().enumerate() {
            let entry = lengths + 0x18 * i as u32;
            let elements = e.mem.alloc(8);
            e.mem.set_u32(entry + 0x0C, elements);
            e.mem.set_u32(entry + 0x10, elements + bytes);
        }
        e.mem.set_u16(geometry + 0x20, 2);
        for (offset, bytes) in [
            (0x70, 0x10),
            (0xB8, 0x30),
            (0xD0, 0x30),
            (0xE8, 0x30),
            (0x100, 0x20),
            (0x148, 0x20),
            (0x160, 0x10),
            (0x178, 4),
        ] {
            make_vector(e, geometry + offset, bytes);
        }
        let coords = make_vector(e, geometry + 0x88, 0x30);
        (geometry, coords)
    }

    /// The pointers a geometry record gets from the getters, in the record's field order.
    fn record_pointers(e: &Engine, geometry: u32, record: u32) -> [(u32, u32); 9] {
        [
            (0x14, 0x70),
            (0x24, 0x88),
            (0x18, 0xB8),
            (0x1C, 0xD0),
            (0x20, 0xE8),
            (0x28, 0x100),
            (0x2C, 0x148),
            (0x30, 0x160),
            (0x34, 0x178),
        ]
        .map(|(field, vector)| {
            (
                e.mem.u32(record + field),
                e.mem.u32(geometry + vector + 0x0C),
            )
        })
    }

    #[test]
    fn the_branch_geometry_record_for_an_explicit_level() {
        let (mut e, calls) = recording(&[]);
        stub(&mut e, &calls, 0x00b3_0730, 0x77);
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        let tree = linked_tree(&mut e);
        let (geometry, _) = full_geometry(&mut e);
        e.set(tree, CSpeedTreeRT::m_pBranchGeometry, Ptr::new(geometry));
        e.set(tree, CSpeedTreeRT::m_ucTargetAlphaValue, 200);
        let record = e.mem.alloc(0x40);
        e.call(0x00b0_b1d0, &args![tree, record, 1i16]);
        assert_eq!(e.mem.u16(record + 0x10), 4);
        for (stored, expected) in record_pointers(&e, geometry, record) {
            assert_eq!(stored, expected);
            assert_ne!(stored, 0);
        }
        assert_eq!(e.mem.i32(record), 1);
        assert_eq!(e.mem.u16(record + 4), 3);
        let lengths = e.mem.u32(geometry + 0x40 + 0x0C);
        assert_eq!(e.mem.u32(record + 8), e.mem.u32(lengths + 0x18 + 0x0C));
        assert_eq!(e.mem.u32(record + 0xC), 0x77);
        assert_eq!(calls_to(&calls, 0x00b3_0730), vec![vec![geometry, 1]]);
        assert_eq!(e.mem.f32(record + 0x38), 200.0);
    }

    #[test]
    fn the_branch_geometry_record_picks_the_discrete_level_for_minus_one() {
        let (mut e, calls) = recording(&[]);
        stub(&mut e, &calls, 0x00b3_0730, 0);
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        global_f32(&mut e, 0x0101_2054, -1.0);
        global_f64(&mut e, 0x0101_a6b0, -1.0);
        let tree = linked_tree(&mut e);
        let (geometry, _) = full_geometry(&mut e);
        e.set(tree, CSpeedTreeRT::m_pBranchGeometry, Ptr::new(geometry));
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        // Two branch levels and the tree's own level 0.25: (1 - 0.25) * 2 = 1.5 -> level 1.
        e.mem.set_u32(engine + 0x78, 2);
        e.mem.set_f32(engine + 0x14, 0.25);
        let record = e.mem.alloc(0x40);
        e.call(0x00b0_b1d0, &args![tree, record, -1i16]);
        assert_eq!(e.mem.i32(record), 1);
        assert_eq!(e.mem.u16(record + 4), 3);
    }

    #[test]
    fn the_branch_geometry_record_blends_the_top_levels_with_the_discrete_flag() {
        let (mut e, calls) = recording(&[]);
        stub(&mut e, &calls, 0x00b3_0730, 0);
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        e.mem.set_u8(DISCRETE_LEAF_LOD_FLAG, 1);
        global_f32(&mut e, 0x0101_2054, -1.0);
        global_f32(&mut e, 0x0102_3cd8, 255.0);
        global_f64(&mut e, 0x0101_1588, 0.5);
        let tree = linked_tree(&mut e);
        let (geometry, _) = full_geometry(&mut e);
        e.set(tree, CSpeedTreeRT::m_pBranchGeometry, Ptr::new(geometry));
        e.set(tree, CSpeedTreeRT::m_ucTargetAlphaValue, 200);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        // One leaf level plus one makes two levels. At level of detail 0 the top one shows
        // alone: its alpha 255 and the record's level -1.
        e.mem.set_u32(engine + 0xD8, 1);
        e.mem.set_f32(engine + 0x14, 0.0);
        let record = e.mem.alloc(0x40);
        e.call(0x00b0_b1d0, &args![tree, record, 0i16]);
        assert_eq!(e.mem.f32(record + 0x38), 255.0);
        assert_eq!(e.mem.i32(record), -1);
        // At level of detail 1 the first level shows alone with the tree's own alpha.
        e.mem.set_f32(engine + 0x14, 1.0);
        e.call(0x00b0_b1d0, &args![tree, record, 0i16]);
        assert_eq!(e.mem.f32(record + 0x38), 200.0);
        assert_eq!(e.mem.i32(record), 0);
    }

    #[test]
    fn the_frond_geometry_record_is_the_branch_one_at_an_offset() {
        let (mut e, calls) = recording(&[]);
        stub(&mut e, &calls, 0x00b3_0730, 0x55);
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        global_f32(&mut e, 0x0101_2054, -1.0);
        let tree = linked_tree(&mut e);
        let (geometry, _) = full_geometry(&mut e);
        e.set(tree, CSpeedTreeRT::m_pFrondGeometry, Ptr::new(geometry));
        e.set(tree, CSpeedTreeRT::m_ucTargetAlphaValue, 100);
        let record = e.mem.alloc(0x80);
        e.call(0x00b0_b920, &args![tree, record, 0i16]);
        assert_eq!(e.mem.u16(record + 0x3C + 0x10), 4);
        assert_eq!(e.mem.u32(record + 0x3C + 0xC), 0x55);
        assert_eq!(e.mem.f32(record + 0x74), 100.0);
        assert_eq!(e.mem.i32(record + 0x3C), 0);
        // The branch part of the record is untouched.
        assert_eq!(e.mem.u16(record + 0x10), 0);
    }

    #[test]
    fn a_geometry_record_without_geometry_reports_an_error() {
        let (mut e, calls) = recording(&[]);
        error_doubles(&mut e, &calls);
        let tree = linked_tree(&mut e);
        e.set(tree, CSpeedTreeRT::m_pBranchGeometry, Ptr::NULL);
        e.set(tree, CSpeedTreeRT::m_pFrondGeometry, Ptr::NULL);
        e.call(0x00b0_b1d0, &args![tree, 0u32, 0i16]);
        e.call(0x00b0_b920, &args![tree, 0u32, 0i16]);
        assert!(error_was(&calls, 0x010a_53c0));
        assert!(error_was(&calls, 0x010a_5408));
    }

    /// The statics the leaf geometry functions read: the camera direction (0, 0, 0), the
    /// angle scale 2, the sentinel -1, the 255, and the doubles of the angle functions (both
    /// return 0); `00b28e40` and `00b288f0` are recorded.
    fn leaf_fixture(e: &mut Engine, calls: &Calls) {
        error_doubles(e, calls);
        for address in [0x00b2_8e40, 0x00b2_88f0] {
            stub(e, calls, address, 0);
        }
        stub_float(e, calls, ATAN2, 0.0);
        stub_float(e, calls, ANGLE_MATH_FUNCTION, 0.0);
        e.map(CAMERA_POSITION, 0x20);
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        global_f64(e, ANGLE_SCALE, 2.0);
        global_f64(e, 0x0101_a6b0, -1.0);
        global_f64(e, 0x0101_2060, 0.0);
        global_f64(e, 0x0101_1588, 0.5);
        global_f64(e, 0x0101_1590, 2.0);
        global_f32(e, 0x0101_2054, -1.0);
        global_f32(e, 0x0102_3cd8, 255.0);
    }

    /// A tree set up for the leaf geometry functions: `levels` leaf levels, the target alpha 200,
    /// transition radius 1, factor 0, exponent 3, the leaf size factor `+0x24` of 1.5.
    fn leaf_tree(e: &mut Engine, levels: u32) -> Ptr<CSpeedTreeRT> {
        let tree = linked_tree(e);
        let engine = part(e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_u32(engine + 0xD8, levels);
        e.set(tree, CSpeedTreeRT::m_ucTargetAlphaValue, 200);
        e.set(tree, CSpeedTreeRT::m_fLeafLodTransitionRadius, 1.0);
        e.set(tree, CSpeedTreeRT::m_fLeafTransitionFactor, 0.0);
        e.set(tree, CSpeedTreeRT::m_fLeafLodCurveExponent, 3.0);
        e.set(tree, CSpeedTreeRT::m_fLeafSizeIncreaseFactor, 1.5);
        tree
    }

    #[test]
    fn leaf_geometry_for_a_float_level_blends_two_levels() {
        let (mut e, calls) = recording(&[]);
        leaf_fixture(&mut e, &calls);
        // `pow` gives 0.25 whatever its arguments; the blend alphas are 0.25 * 255 + 0.
        stub_float(&mut e, &calls, POW, 0.25);
        stub_float(&mut e, &calls, ABSOLUTE_VALUE, 0.0);
        let tree = leaf_tree(&mut e, 4);
        e.set(tree, CSpeedTreeRT::m_ucTargetAlphaValue, 0);
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 1);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        let out = e.mem.alloc(0x140);
        e.call(0x00b0_ad00, &args![tree, out, 0.5f32]);
        // Four levels: 0.5 is exactly level 2; the blend is of levels 1 and 2.
        let extra = 1.5f32.to_bits();
        let a = 0u32;
        let b = 0x8000_0000u32;
        assert_eq!(
            calls_to(&calls, 0x00b2_8e40),
            vec![
                vec![leaf, out + 0x78, 0, a, b, extra],
                vec![leaf, out + 0x78, 1, a, b, extra],
                vec![leaf, out + 0xB4, 2, a, b, extra],
            ]
        );
        assert_eq!((e.mem.u8(out + 0x78), e.mem.u8(out + 0xB4)), (1, 1));
        assert_eq!(e.mem.f32(out + 0x7C), 63.75);
        assert_eq!(e.mem.f32(out + 0xB8), 63.75);
        assert_eq!(e.mem.u8(out + 0xF0), 0);
        assert_eq!(e.mem.f32(out + 0x38), 0.0);
    }

    #[test]
    fn leaf_geometry_for_a_float_level_uses_one_level_for_methods_other_than_one() {
        let (mut e, calls) = recording(&[]);
        leaf_fixture(&mut e, &calls);
        stub_float(&mut e, &calls, POW, 0.25);
        stub_float(&mut e, &calls, ABSOLUTE_VALUE, 0.0);
        let tree = leaf_tree(&mut e, 4);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_f32(engine + 0x14, 0.5);
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 3);
        let out = e.mem.alloc(0x140);
        e.mem.set_u8(out + 0xB4, 9);
        e.call(0x00b0_ad00, &args![tree, out, 0.5f32]);
        let seen = calls_to(&calls, 0x00b2_8e40);
        assert_eq!(seen.len(), 1);
        assert_eq!((seen[0][1], seen[0][2]), (out + 0x78, 0));
        assert_eq!(
            (
                e.mem.u8(out + 0x78),
                e.mem.f32(out + 0x7C),
                e.mem.u8(out + 0xB4)
            ),
            (1, 200.0, 0)
        );
        // Another method asks for the discrete level of the tree's own level of detail:
        // (1 - 0.5) * 4 = level 2.
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 0);
        e.mem.set_u8(out + 0x78, 0);
        e.call(0x00b0_ad00, &args![tree, out, 0.5f32]);
        let seen = calls_to(&calls, 0x00b2_8e40);
        assert_eq!(seen.len(), 2);
        assert_eq!((seen[1][1], seen[1][2]), (out + 0x78, 2));
        assert_eq!((e.mem.u8(out + 0x78), e.mem.f32(out + 0x7C)), (1, 200.0));
        assert_eq!(e.mem.u8(out + 0xB4), 0);
    }

    #[test]
    fn leaf_geometry_for_a_negative_level_stops_early() {
        let (mut e, calls) = recording(&[]);
        leaf_fixture(&mut e, &calls);
        stub_float(&mut e, &calls, POW, 0.25);
        stub_float(&mut e, &calls, ABSOLUTE_VALUE, 0.0);
        let tree = leaf_tree(&mut e, 4);
        let out = e.mem.alloc(0x140);
        e.mem.set_f32(out + 0x38, 7.0);
        e.call(0x00b0_ad00, &args![tree, out, -0.5f32]);
        assert!(calls_to(&calls, 0x00b2_8e40).is_empty());
        assert_eq!(e.mem.f32(out + 0x38), 7.0);
    }

    #[test]
    fn leaf_geometry_with_the_discrete_flag_adds_a_level() {
        let (mut e, calls) = recording(&[]);
        leaf_fixture(&mut e, &calls);
        stub_float(&mut e, &calls, ABSOLUTE_VALUE, 0.0);
        e.mem.set_u8(DISCRETE_LEAF_LOD_FLAG, 1);
        // One leaf level plus the discrete one: two levels. At lod 0 the top level shows alone.
        let tree = leaf_tree(&mut e, 1);
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 3);
        let out = e.mem.alloc(0x140);
        e.call(0x00b0_ad00, &args![tree, out, 0.0f32]);
        assert_eq!(e.mem.f32(out + 0xFC), 200.0);
        assert_eq!(e.mem.u8(out + 0xF0), 1);
        assert_eq!(e.mem.f32(out + 0x38), 255.0);
        // At lod 1 the first level shows alone; the record's blend is -1 ("none").
        let again = e.mem.alloc(0x140);
        e.call(0x00b0_ad00, &args![tree, again, 1.0f32]);
        assert_eq!(e.mem.f32(again + 0x38), 200.0);
    }

    #[test]
    fn leaf_geometry_for_a_level_asks_for_every_level_first() {
        let (mut e, calls) = recording(&[]);
        leaf_fixture(&mut e, &calls);
        let tree = leaf_tree(&mut e, 3);
        let leaf = part(&e, tree, CSpeedTreeRT::m_pLeafGeometry);
        let out = e.mem.alloc(0x140);
        e.call(0x00b0_bc00, &args![tree, out, 2i16]);
        let all: Vec<u32> = calls_to(&calls, 0x00b2_88f0).iter().map(|a| a[2]).collect();
        assert_eq!(all, vec![0, 1, 2]);
        let extra = 1.5f32.to_bits();
        assert_eq!(
            calls_to(&calls, 0x00b2_8e40),
            vec![vec![leaf, out + 0x78, 2, 0, 0x8000_0000, extra]]
        );
        assert_eq!(
            (
                e.mem.u8(out + 0x78),
                e.mem.f32(out + 0x7C),
                e.mem.u8(out + 0xB4)
            ),
            (1, 200.0, 0)
        );
    }

    #[test]
    fn leaf_geometry_for_minus_one_follows_the_leaf_lod_method() {
        let (mut e, calls) = recording(&[]);
        leaf_fixture(&mut e, &calls);
        stub_float(&mut e, &calls, POW, 0.25);
        stub_float(&mut e, &calls, ABSOLUTE_VALUE, 0.0);
        let tree = leaf_tree(&mut e, 4);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_f32(engine + 0x14, 0.5);
        e.set(tree, CSpeedTreeRT::m_ucTargetAlphaValue, 0);
        let out = e.mem.alloc(0x140);
        // Method 1: the two blended levels (1 and 2) of lod 0.5.
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 1);
        e.call(0x00b0_bc00, &args![tree, out, -1i16]);
        let seen = calls_to(&calls, 0x00b2_8e40);
        let levels: Vec<(u32, u32)> = seen.iter().map(|a| (a[1], a[2])).collect();
        assert_eq!(levels, vec![(out + 0x78, 1), (out + 0xB4, 2)]);
        assert_eq!(
            (e.mem.f32(out + 0x7C), e.mem.f32(out + 0xB8)),
            (63.75, 63.75)
        );
        assert_eq!((e.mem.u8(out + 0x78), e.mem.u8(out + 0xB4)), (1, 1));
        // Method 3: level 0 alone.
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 3);
        e.call(0x00b0_bc00, &args![tree, out, -1i16]);
        assert_eq!(calls_to(&calls, 0x00b2_8e40).len(), 3);
        assert_eq!((e.mem.u8(out + 0x78), e.mem.u8(out + 0xB4)), (1, 0));
        // Another method: the discrete level 2 of (1 - 0.5) * 4.
        e.set(tree, CSpeedTreeRT::m_eLeafLodMethod, 0);
        e.call(0x00b0_bc00, &args![tree, out, -1i16]);
        let seen = calls_to(&calls, 0x00b2_8e40);
        assert_eq!(seen.len(), 4);
        assert_eq!(seen[3][2], 2);
    }

    /// The doubles of the angle functions of `GetTransitionValues` and the statics it reads:
    /// `|x|` returns `distance`, `pow` returns `power`.
    fn transition_fixture(e: &mut Engine, calls: &Calls, distance: f64, power: f64) {
        stub_float(e, calls, ABSOLUTE_VALUE, distance);
        stub_float(e, calls, POW, power);
        global_f64(e, 0x0101_1588, 0.5);
        global_f64(e, 0x0101_1590, 2.0);
        global_f32(e, 0x0102_3cd8, 255.0);
    }

    /// Calls `GetTransitionValues`; returns (first value, second value, first level, second
    /// level).
    fn transitions(
        e: &mut Engine,
        lod: f32,
        count: u32,
        radius: f32,
        factor: f32,
        alpha: f32,
    ) -> (f32, f32, i16, i16) {
        let outputs = e.mem.alloc(0x10);
        e.call(
            0x00b0_c8d0,
            &args![
                lod,
                count,
                radius,
                factor,
                3.0f32,
                alpha,
                outputs,
                outputs + 4,
                outputs + 8,
                outputs + 10
            ],
        );
        (
            e.mem.f32(outputs),
            e.mem.f32(outputs + 4),
            e.mem.i16(outputs + 8),
            e.mem.i16(outputs + 10),
        )
    }

    #[test]
    fn transition_values_blend_two_levels_inside_the_radius() {
        let (mut e, calls) = recording(&[]);
        transition_fixture(&mut e, &calls, 0.0, 0.25);
        // Four levels, lod 0.5: exactly level 2, offset 0; both neighbours blend. The blend
        // alphas are pow * (255 - alpha) + alpha = 0.25 * 255.
        let result = transitions(&mut e, 0.5, 4, 1.0, 0.0, 0.0);
        assert_eq!(result, (63.75, 63.75, 1, 2));
        // pow is called with 1 - 0.5 and the exponent 3.
        let pow = calls_to(&calls, POW);
        assert_eq!(pow.len(), 2);
        let double =
            |words: &[u32]| f64::from_bits(u64::from(words[0]) | u64::from(words[1]) << 32);
        assert_eq!((double(&pow[0][..2]), double(&pow[0][2..])), (0.5, 3.0));
    }

    #[test]
    fn transition_values_cap_the_blend_at_one() {
        let (mut e, calls) = recording(&[]);
        transition_fixture(&mut e, &calls, 0.0, 0.5);
        // A factor of 0.75 pushes both blends past 1: pow is called with 1 - 1 = 0.
        let (first, second, first_level, second_level) =
            transitions(&mut e, 0.5, 4, 1.0, 0.75, 0.0);
        assert_eq!((first, second), (127.5, 127.5));
        assert_eq!((first_level, second_level), (1, 2));
        for call in calls_to(&calls, POW) {
            assert_eq!(
                f64::from_bits(u64::from(call[0]) | u64::from(call[1]) << 32),
                0.0
            );
        }
    }

    #[test]
    fn transition_values_show_one_level_beyond_the_radius() {
        let (mut e, calls) = recording(&[]);
        // |offset| is 0.1, the radius 0.05: the nearest level shows alone.
        transition_fixture(&mut e, &calls, 0.1, 0.25);
        let (first, second, first_level, second_level) =
            transitions(&mut e, 0.6, 4, 0.05, 0.0, 77.0);
        assert_eq!((first, second), (77.0, 255.0));
        assert_eq!((first_level, second_level), (1, -1));
        assert!(calls_to(&calls, POW).is_empty());
    }

    #[test]
    fn transition_values_at_the_ends_show_one_level() {
        let (mut e, calls) = recording(&[]);
        transition_fixture(&mut e, &calls, 0.0, 0.25);
        // lod 1: level 0; lod 0: level 4 = the count, capped at 3.
        assert_eq!(
            transitions(&mut e, 1.0, 4, 1.0, 0.0, 9.0),
            (9.0, 255.0, 0, -1)
        );
        assert_eq!(
            transitions(&mut e, 0.0, 4, 1.0, 0.0, 9.0),
            (9.0, 255.0, 3, -1)
        );
        assert!(calls_to(&calls, ABSOLUTE_VALUE).is_empty());
    }

    #[test]
    fn the_leaf_lod_size_factors_are_made_once() {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_u32(engine + 0xD8, 3);
        e.set(tree, CSpeedTreeRT::m_fLeafSizeIncreaseFactor, 0.5);
        let table = e.call(0x00b0_cb10, &args![tree]).u32();
        assert_ne!(table, 0);
        assert_eq!(
            e.get(tree, CSpeedTreeRT::m_pLeafLodSizeFactors).addr(),
            table
        );
        assert_eq!(
            [0, 4, 8].map(|offset| e.mem.f32(table + offset)),
            [1.0, 1.5, 2.0]
        );
        // A second call returns the same table without rebuilding it.
        e.mem.set_f32(table, 9.0);
        assert_eq!(e.call(0x00b0_cb10, &args![tree]).u32(), table);
        assert_eq!(e.mem.f32(table), 9.0);
    }

    #[test]
    fn the_projected_shadow_is_made_and_parses_the_file() {
        let (mut e, calls) = recording(&[0x00b3_e750]);
        e.register_double(0x00b3_e6c0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 8);
        e.call(0x00b0_cd40, &args![tree, file]);
        let shadow = e.get(tree, CSpeedTreeRT::m_pProjectedShadow);
        assert!(!shadow.is_null());
        assert_eq!(e.mem.block_size(shadow.addr()), Some(0x40));
        assert_eq!(
            calls_to(&calls, 0x00b3_e750),
            vec![vec![shadow.addr(), file]]
        );
    }

    /// A tree with a projected shadow and the box (0, 0, 0)-(3, 4, 0) as its sizes, and the
    /// statics `ComputeSelfShadowTexCoords` reads.
    fn shadow_tree(e: &mut Engine) -> (Ptr<CSpeedTreeRT>, u32) {
        global_f64(e, 0x0101_1588, 0.5);
        global_f32(e, 0x0101_6248, 0.5);
        let tree = linked_tree(e);
        let shadow = e.mem.alloc(0x40);
        e.set(tree, CSpeedTreeRT::m_pProjectedShadow, Ptr::new(shadow));
        let sizes = part(e, tree, CSpeedTreeRT::m_pTreeSizes);
        e.mem.set_f32(sizes + 0xC, 3.0);
        e.mem.set_f32(sizes + 0x10, 4.0);
        (tree, shadow)
    }

    #[test]
    fn the_self_shadow_coordinates_are_computed_for_both_geometries() {
        let (mut e, calls) = recording(&[0x00b3_ea10]);
        let (tree, shadow) = shadow_tree(&mut e);
        let embedded = e.mem.alloc(0x54);
        e.set(tree, CSpeedTreeRT::m_pEmbeddedTexCoords, Ptr::new(embedded));
        let branch = part(&e, tree, CSpeedTreeRT::m_pBranchGeometry);
        let frond = part(&e, tree, CSpeedTreeRT::m_pFrondGeometry);
        e.call(0x00b0_cdd0, &args![tree]);
        // The radius: Distance((0,0,0), (3,4,0)) = 5.125 by the approximation, halved; the
        // centre: (min + max) * 0.5.
        let radius = 2.5625f32.to_bits();
        let centre = [1.5f32.to_bits(), 2.0f32.to_bits(), 0];
        let expected = |geometry: u32| {
            vec![
                shadow,
                geometry,
                centre[0],
                centre[1],
                centre[2],
                radius,
                embedded + 0x34,
            ]
        };
        assert_eq!(
            calls_to(&calls, 0x00b3_ea10),
            vec![expected(branch), expected(frond)]
        );
    }

    #[test]
    fn the_self_shadow_coordinates_are_computed_without_embedded_coordinates() {
        let (mut e, calls) = recording(&[0x00b3_ea10]);
        let (tree, _) = shadow_tree(&mut e);
        e.call(0x00b0_cdd0, &args![tree]);
        let seen = calls_to(&calls, 0x00b3_ea10);
        assert_eq!(seen.len(), 2);
        assert_eq!((seen[0][6], seen[1][6]), (0, 0));
        // Without a shadow, or without a geometry, nothing is computed.
        e.set(tree, CSpeedTreeRT::m_pProjectedShadow, Ptr::NULL);
        e.call(0x00b0_cdd0, &args![tree]);
        assert_eq!(calls_to(&calls, 0x00b3_ea10).len(), 2);
    }

    #[test]
    fn the_texture_names_are_collected() {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        small_string(&mut e, engine + 0x24, "tree");
        let leaves = make_vector(&mut e, engine + 0xA4, 2 * 0x54);
        small_string(&mut e, leaves + 0x14, "a.dds");
        small_string(&mut e, leaves + 0x54 + 0x14, "b.dds");
        let frond_engine = part(&e, tree, CSpeedTreeRT::m_pFrondEngine);
        let fronds = make_vector(&mut e, frond_engine + 0x50, 0x2C);
        small_string(&mut e, fronds, "f.dds");
        let shadow = e.mem.alloc(0x40);
        small_string(&mut e, shadow + 0x24, "sh");
        e.set(tree, CSpeedTreeRT::m_pProjectedShadow, Ptr::new(shadow));
        let embedded = e.mem.alloc(0x54);
        small_string(&mut e, embedded + 0x18, "emb");
        e.set(tree, CSpeedTreeRT::m_pEmbeddedTexCoords, Ptr::new(embedded));
        let out = e.mem.alloc(0x20);
        e.call(0x00b0_d040, &args![tree, out]);
        assert_eq!(e.mem.u32(out), engine + 0x24 + 4);
        assert_eq!(e.mem.u32(out + 4), 2);
        let names = e.mem.u32(out + 8);
        assert_eq!(e.mem.u32(names), leaves + 0x14 + 4);
        assert_eq!(e.mem.u32(names + 4), leaves + 0x54 + 0x14 + 4);
        assert_eq!(e.mem.u32(out + 0xC), 1);
        assert_eq!(e.mem.u32(e.mem.u32(out + 0x10)), fronds + 4);
        assert_eq!(e.mem.u32(out + 0x18), shadow + 0x24 + 4);
        assert_eq!(e.mem.u32(out + 0x14), embedded + 0x18 + 4);
    }

    #[test]
    fn the_texture_names_of_a_tree_without_textures_are_null() {
        let mut e = Engine::new();
        let tree = linked_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        make_vector(&mut e, engine + 0xA4, 0);
        let frond_engine = part(&e, tree, CSpeedTreeRT::m_pFrondEngine);
        make_vector(&mut e, frond_engine + 0x50, 0);
        let out = e.mem.alloc(0x20);
        for offset in [0x14, 0x18] {
            e.mem.set_u32(out + offset, 0x55);
        }
        e.call(0x00b0_d040, &args![tree, out]);
        assert_eq!((e.mem.u32(out + 4), e.mem.u32(out + 8)), (0, 0));
        assert_eq!((e.mem.u32(out + 0xC), e.mem.u32(out + 0x10)), (0, 0));
        // The shadow's and the composite's names are only stored when there are any.
        assert_eq!((e.mem.u32(out + 0x14), e.mem.u32(out + 0x18)), (0x55, 0x55));
    }

    #[test]
    fn a_frond_texture_name_comes_from_the_checked_vector() {
        let (mut e, calls) = recording(&[INVALID_PARAMETER]);
        let engine = e.mem.alloc(0x80);
        let storage = make_vector(&mut e, engine + 0x50, 2 * 0x2C);
        small_string(&mut e, storage, "x");
        small_string(&mut e, storage + 0x2C, "y");
        // Memory after the vector, so that the out-of-range read of the last case is mapped.
        let _guard = e.mem.alloc(0x100);
        assert_eq!(
            e.call(0x00b0_d490, &args![engine, 1u32]).u32(),
            storage + 0x2C + 4
        );
        assert!(calls_to(&calls, INVALID_PARAMETER).is_empty());
        e.call(0x00b0_d490, &args![engine, 2u32]);
        assert_eq!(calls_to(&calls, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn the_user_data_string_is_copied_into_the_tree() {
        let (mut e, calls) = recording(&[STRING_TIDY]);
        file_doubles(&mut e, vec![0x4A3A, 0x4A39], vec![]);
        e.register(MEMCPY, memcpy_double);
        e.register_double(STRLEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register_double(PARSE_STRING, |e, a| {
            small_string(e, a[1], "hello");
            Ret::default()
        });
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 16);
        e.call(0x00b0_d500, &args![tree, file]);
        let data = e.get(tree, CSpeedTreeRT::m_pUserData).addr();
        assert_eq!(e.mem.cstr(data), b"hello".to_vec());
        assert_eq!(calls_to(&calls, STRING_TIDY).len(), 1);
    }

    #[test]
    fn other_user_data_tags_are_skipped() {
        let mut e = Engine::new();
        file_doubles(&mut e, vec![5, 0x4A39], vec![]);
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 16);
        e.call(0x00b0_d500, &args![tree, file]);
        assert!(e.get(tree, CSpeedTreeRT::m_pUserData).is_null());
        assert_eq!(e.mem.u32(file), 2);
    }

    #[test]
    fn the_horizontal_billboard_corners_follow_the_box() {
        let mut e = Engine::new();
        global_f64(&mut e, 0x0101_1588, 0.5);
        let tree = linked_tree(&mut e);
        let sizes = part(&e, tree, CSpeedTreeRT::m_pTreeSizes);
        for (i, value) in [1.0f32, 2.0, 3.0, 4.0, 6.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(sizes + 4 * i as u32, value);
        }
        e.call(0x00b0_d5f0, &args![tree]);
        assert_eq!(e.mem.f32(tree.addr() + 0x70), 0.0);
        e.set(tree, CSpeedTreeRT::m_bHorizontalBillboard, 1);
        e.call(0x00b0_d5f0, &args![tree]);
        let corners: Vec<[f32; 3]> = (0..4)
            .map(|k| [0, 4, 8].map(|offset| e.mem.f32(tree.addr() + 0x70 + 12 * k + offset)))
            .collect();
        assert_eq!(
            corners,
            vec![
                [4.0, 6.0, 6.0],
                [1.0, 6.0, 6.0],
                [1.0, 2.0, 6.0],
                [4.0, 2.0, 6.0]
            ]
        );
    }

    #[test]
    fn a_string_is_duplicated() {
        let mut e = Engine::new();
        e.register(MEMCPY, memcpy_double);
        e.register_double(STRLEN, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        let text = e.mem.alloc(8);
        e.mem.write(text, b"abc");
        let copy = e.call(0x00b0_d7c0, &args![text]).u32();
        assert_ne!(copy, text);
        assert_eq!(e.mem.bytes(copy, 4), b"abc\0".to_vec());
        assert_eq!(e.mem.block_size(copy), Some(8));
    }

    #[test]
    fn the_composite_chunk_sets_flags_and_shadow_coordinates() {
        let (mut e, _) = recording(&[]);
        let tags = vec![20003, 20004, 20005, 20001];
        file_doubles(&mut e, tags, (1..=8).map(f64::from).collect());
        let tree = linked_tree(&mut e);
        let embedded = e.mem.alloc(0x54);
        e.set(tree, CSpeedTreeRT::m_pEmbeddedTexCoords, Ptr::new(embedded));
        let file = access(&mut e, 64);
        let bytes = e.mem.u32(file + 0x10);
        for offset in 0..64 {
            e.mem.set_u8(bytes + offset, 1);
        }
        e.call(0x00b0_d810, &args![tree, file]);
        assert_eq!(
            (e.mem.u8(tree.addr() + 0x6D), e.mem.u8(tree.addr() + 0x6C)),
            (1, 1)
        );
        assert_eq!(
            (0..8)
                .map(|i| e.mem.f32(embedded + 0x34 + 4 * i))
                .collect::<Vec<_>>(),
            (1..=8).map(|i| i as f32).collect::<Vec<_>>()
        );
    }

    #[test]
    fn the_composite_chunk_does_nothing_without_embedded_coordinates() {
        let (mut e, calls) = recording(&[PARSE_INT]);
        let tree = linked_tree(&mut e);
        let file = access(&mut e, 8);
        e.call(0x00b0_d810, &args![tree, file]);
        assert!(calls_to(&calls, PARSE_INT).is_empty());
    }

    #[test]
    fn the_composite_chunk_keeps_only_the_file_name_of_the_map() {
        let (mut e, calls) = recording(&[]);
        string_doubles(&mut e, &calls);
        e.register(STRCPY, strcpy_double);
        file_doubles(&mut e, vec![20002, 20001], vec![]);
        e.register_double(PARSE_STRING, |e, a| {
            small_string(e, a[1], "d/e.dds");
            Ret::default()
        });
        let tree = linked_tree(&mut e);
        let embedded = e.mem.alloc(0x54);
        e.set(tree, CSpeedTreeRT::m_pEmbeddedTexCoords, Ptr::new(embedded));
        let file = access(&mut e, 64);
        e.call(0x00b0_d810, &args![tree, file]);
        // The reduced name "e.dds" was built into a string and assigned to the record's.
        let made = calls_to(&calls, STRING_FROM_C_STRING);
        assert_eq!(made[0][2], 5);
        let assigned = calls_to(&calls, STRING_ASSIGN);
        assert_eq!(assigned.last().unwrap()[0], embedded + 0x18);
        assert_eq!(calls_to(&calls, STRING_COPY).len(), 1);
    }

    #[test]
    fn the_composite_chunk_throws_on_an_unknown_tag_or_a_short_file() {
        for (tags, length, message) in [
            (vec![20009], 64, 0x010a_5278),
            (vec![20003], 2, 0x010a_5238),
        ] {
            let (mut e, calls) = recording(&[]);
            parse_error_doubles(&mut e, &calls);
            file_doubles(&mut e, tags, vec![]);
            let tree = linked_tree(&mut e);
            let embedded = e.mem.alloc(0x54);
            e.set(tree, CSpeedTreeRT::m_pEmbeddedTexCoords, Ptr::new(embedded));
            let file = access(&mut e, length);
            assert!(throws(|| {
                e.call(0x00b0_d810, &args![tree, file]);
            }));
            assert_eq!(calls_to(&calls, STRING_FROM_C_STRING)[0][1], message);
        }
    }

    #[test]
    fn the_simple_billboard_is_scaled_once() {
        let mut e = Engine::new();
        e.map(0x011f_8d00, 0x40);
        for k in 0..4u32 {
            for axis in 0..3u32 {
                e.mem
                    .set_f32(0x011f_8d08 + 12 * k + 4 * axis, (1 + 3 * k + axis) as f32);
            }
        }
        let billboard = e.mem.alloc(0x34);
        assert_eq!(
            e.call(0x00b0_c780, &args![billboard, 2.0f32, 3.0f32]).u32(),
            billboard
        );
        // Corner k is (first * u0, first * u1, second * u2).
        let corner =
            |e: &Engine, k: u32| [0, 4, 8].map(|offset| e.mem.f32(billboard + 12 * k + offset));
        assert_eq!(corner(&e, 0), [2.0, 4.0, 9.0]);
        assert_eq!(corner(&e, 3), [20.0, 22.0, 36.0]);
        assert_eq!(e.mem.u8(billboard + 0x30), 1);
        // Once valid it is not computed again.
        e.call(0x00b0_c780, &args![billboard, 5.0f32, 5.0f32]);
        assert_eq!(corner(&e, 0), [2.0, 4.0, 9.0]);
    }

    #[test]
    fn the_azimuth_of_a_vector_is_scaled_to_degrees() {
        let (mut e, calls) = recording(&[]);
        stub_float(&mut e, &calls, ATAN2, 0.5);
        global_f64(&mut e, ANGLE_SCALE, 2.0);
        let vector = e.mem.alloc(0xC);
        e.mem.set_f32(vector, 3.0);
        e.mem.set_f32(vector + 4, 4.0);
        assert_eq!(e.call(0x00b0_c470, &args![vector]).f32(), 1.0);
        // atan2 takes y first, then x.
        let seen = &calls_to(&calls, ATAN2)[0];
        let double =
            |words: &[u32]| f64::from_bits(u64::from(words[0]) | u64::from(words[1]) << 32);
        assert_eq!((double(&seen[..2]), double(&seen[2..])), (4.0, 3.0));
    }

    /// The statics and doubles the billboard functions need.
    fn billboard_fixture(e: &mut Engine, calls: &Calls) {
        transition_fixture(e, calls, 0.0, 0.25);
        stub_float(e, calls, ATAN2, 0.25);
        e.map(CAMERA_POSITION, 0x20);
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        e.mem.set_u8(DISCRETE_LEAF_LOD_FLAG, 1);
        e.map(0x011f_8d00, 0x40);
        global_f64(e, ANGLE_SCALE, 2.0);
        global_f64(e, 0x0101_a6b0, -1.0);
        global_f64(e, 0x0101_2060, 0.0);
        global_f64(e, 0x0103_c348, 1.0);
        global_f64(e, 0x0101_e568, 255.0);
        global_f32(e, 0x0101_2054, -1.0);
        global_f32(e, 0x011a_d124, 33.0);
    }

    /// A tree for the billboard functions: three leaf levels (four with the discrete one), a
    /// simple billboard and tree sizes, the target alpha 100, transition radius 1 and
    /// exponent 3.
    fn billboard_tree(e: &mut Engine) -> Ptr<CSpeedTreeRT> {
        let tree = linked_tree(e);
        let engine = part(e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_u32(engine + 0xD8, 3);
        e.set(tree, CSpeedTreeRT::m_ucTargetAlphaValue, 100);
        e.set(tree, CSpeedTreeRT::m_fLeafLodTransitionRadius, 1.0);
        e.set(tree, CSpeedTreeRT::m_fLeafLodCurveExponent, 3.0);
        let billboard = e.mem.alloc(0x34);
        e.set(tree, CSpeedTreeRT::m_pSimpleBillboard, Ptr::new(billboard));
        let sizes = part(e, tree, CSpeedTreeRT::m_pTreeSizes);
        e.mem.set_f32(sizes + 0x14, 1.5);
        e.mem.set_f32(sizes + 0x18, 2.5);
        tree
    }

    #[test]
    fn the_billboard_record_without_the_discrete_flag_is_empty() {
        let mut e = Engine::new();
        e.map(DISCRETE_LEAF_LOD_FLAG, 4);
        let tree = linked_tree(&mut e);
        let out = e.mem.alloc(0x130);
        for offset in [0xF0, 0x104, 0x118] {
            e.mem.set_u8(out + offset, 0xFF);
        }
        e.call(0x00b0_c4f0, &args![tree, out]);
        assert_eq!(
            [0xF0, 0x104, 0x118].map(|offset| e.mem.u8(out + offset)),
            [0, 0, 0]
        );
    }

    #[test]
    fn the_billboard_record_shows_the_top_level() {
        let (mut e, calls) = recording(&[]);
        billboard_fixture(&mut e, &calls);
        let tree = billboard_tree(&mut e);
        let billboard = e.get(tree, CSpeedTreeRT::m_pSimpleBillboard).addr();
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        // Three leaf levels, four with the discrete one; lod 0 shows the top level alone
        // (levels 3 = count - 1): the record's alpha is the tree's own.
        e.mem.set_f32(engine + 0x14, 0.0);
        let out = e.mem.alloc(0x130);
        e.call(0x00b0_c4f0, &args![tree, out]);
        assert_eq!(e.mem.f32(out + 0xFC), 100.0);
        assert_eq!(e.mem.u8(out + 0xF0), 1);
        assert_eq!(e.mem.u32(out + 0xF8), billboard);
        assert_eq!(e.mem.u32(out + 0xF4), 0);
        assert_eq!(
            (
                e.mem.u8(out + 0x104),
                e.mem.u8(out + 0x118),
                e.mem.u16(out + 0x100)
            ),
            (0, 0, 0)
        );
        assert_eq!(e.mem.u8(billboard + 0x30), 1);
    }

    #[test]
    fn the_billboard_record_describes_the_horizontal_billboard() {
        let (mut e, calls) = recording(&[]);
        billboard_fixture(&mut e, &calls);
        let tree = billboard_tree(&mut e);
        let engine = part(&e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_f32(engine + 0x14, 0.0);
        e.set(tree, CSpeedTreeRT::m_bHorizontalBillboard, 1);
        let embedded = e.mem.alloc(0x54);
        let coords = e.mem.alloc(0x100);
        e.mem.set_u32(embedded + 8, 3);
        e.mem.set_u32(embedded + 0xC, coords);
        e.set(tree, CSpeedTreeRT::m_pEmbeddedTexCoords, Ptr::new(embedded));
        let out = e.mem.alloc(0x130);
        e.call(0x00b0_c4f0, &args![tree, out]);
        assert_eq!(e.mem.u32(out + 0xF4), coords);
        assert_eq!(e.mem.u32(out + 0x120), tree.addr() + 0x70);
        // The last billboard's coordinates: (3 * 8 - 8) floats in.
        assert_eq!(e.mem.u32(out + 0x11C), coords + 64);
        assert_eq!(e.mem.u8(out + 0x118), 1);
        assert_eq!(e.mem.f32(out + 0x124), 33.0);
    }

    /// Sets the tree up so that the billboard record is valid with a blend of the top two
    /// levels: lod 0.25 of four levels, the pow double returning 0.25 (so the top level's alpha
    /// is 0.25 * 155 + 100 = 138.75).
    fn blended_billboard(e: &mut Engine) -> (Ptr<CSpeedTreeRT>, u32, u32) {
        let tree = billboard_tree(e);
        let engine = part(e, tree, CSpeedTreeRT::m_pEngine);
        e.mem.set_f32(engine + 0x14, 0.25);
        e.set(tree, CSpeedTreeRT::m_usNum360Billboards, 4);
        let embedded = e.mem.alloc(0x54);
        let coords = e.mem.alloc(0x100);
        e.mem.set_u32(embedded + 0xC, coords);
        e.set(tree, CSpeedTreeRT::m_pEmbeddedTexCoords, Ptr::new(embedded));
        (tree, coords, e.mem.alloc(0x130))
    }

    #[test]
    fn the_360_billboard_record_blends_two_images() {
        let (mut e, calls) = recording(&[]);
        billboard_fixture(&mut e, &calls);
        let (tree, coords, out) = blended_billboard(&mut e);
        e.call(0x00b0_c050, &args![tree, out, 0u32]);
        assert_eq!(e.mem.u8(out + 0xF0), 1);
        // The azimuth gives the blend 0.25; the images 2 and 1 (reversed from 1 and 2).
        assert_eq!(e.mem.u32(out + 0xF4), coords + 2 * 32);
        assert_eq!(e.mem.u32(out + 0x108), coords + 32);
        assert_eq!((e.mem.u16(out + 0x100), e.mem.u16(out + 0x114)), (2, 1));
        assert_eq!(e.mem.f32(out + 0xFC), 167.8125);
        assert_eq!(e.mem.f32(out + 0x110), 167.8125);
        assert_eq!(e.mem.u8(out + 0x104), 1);
        assert_eq!(e.mem.u32(out + 0x10C), e.mem.u32(out + 0xF8));
    }

    #[test]
    fn the_360_billboard_record_with_bit_5_keeps_one_image() {
        let (mut e, calls) = recording(&[]);
        billboard_fixture(&mut e, &calls);
        let (tree, coords, out) = blended_billboard(&mut e);
        e.call(0x00b0_c050, &args![tree, out, 0x20u32]);
        // Equal alphas pick the second image; the alpha is the plain blend of the tree's.
        assert_eq!(e.mem.u32(out + 0xF4), coords + 32);
        assert_eq!(e.mem.f32(out + 0xFC), 138.75);
        assert_eq!(e.mem.u8(out + 0x104), 0);
        assert_eq!(e.mem.u8(out + 0xF0), 1);
    }

    #[test]
    fn the_360_billboard_record_stops_without_billboards() {
        let (mut e, calls) = recording(&[]);
        billboard_fixture(&mut e, &calls);
        let (tree, coords, out) = blended_billboard(&mut e);
        e.set(tree, CSpeedTreeRT::m_usNum360Billboards, 0);
        e.call(0x00b0_c050, &args![tree, out, 0u32]);
        assert_eq!(e.mem.u8(out + 0xF0), 1);
        assert_eq!(e.mem.f32(out + 0xFC), 138.75);
        assert_eq!(e.mem.u32(out + 0xF4), coords);
        assert_eq!(e.mem.u8(out + 0x104), 0);
    }

    #[test]
    fn checked_iterators_step_and_dereference_inside_the_vector() {
        let (mut e, calls) = recording(&[INVALID_PARAMETER]);
        let vector = e.mem.alloc(0x18);
        let storage = make_vector(&mut e, vector, 0x38);
        let iterator = e.mem.alloc(8);
        let proxy = e.mem.u32(vector);
        e.mem.set_u32(iterator, proxy);
        e.mem.set_u32(iterator + 4, storage);
        assert_eq!(e.call(0x00b0_ebb0, &args![iterator]).u32(), storage);
        // The shape vector's step is 0x1C bytes, the pointer vector's 4.
        assert_eq!(e.call(0x00b0_eb30, &args![iterator]).u32(), iterator);
        assert_eq!(e.mem.u32(iterator + 4), storage + 0x1C);
        assert_eq!(e.call(0x00b0_ec20, &args![iterator]).u32(), iterator);
        assert_eq!(e.mem.u32(iterator + 4), storage + 0x20);
        assert!(calls_to(&calls, INVALID_PARAMETER).is_empty());
    }

    #[test]
    fn checked_iterators_at_the_end_take_the_invalid_parameter_path() {
        let (mut e, calls) = recording(&[INVALID_PARAMETER]);
        let vector = e.mem.alloc(0x18);
        let storage = make_vector(&mut e, vector, 8);
        let iterator = e.mem.alloc(8);
        let proxy = e.mem.u32(vector);
        e.mem.set_u32(iterator, proxy);
        e.mem.set_u32(iterator + 4, storage + 8);
        e.call(0x00b0_ebb0, &args![iterator]);
        e.call(0x00b0_eb30, &args![iterator]);
        e.call(0x00b0_ec20, &args![iterator]);
        assert_eq!(calls_to(&calls, INVALID_PARAMETER).len(), 3);
        // An iterator without a vector fails both tests (the exe then reads through address 0).
        e.map(0, 0x1000);
        e.mem.set_u32(iterator, 0);
        let before = calls_to(&calls, INVALID_PARAMETER).len();
        e.call(0x00b0_ebb0, &args![iterator]);
        assert_eq!(calls_to(&calls, INVALID_PARAMETER).len(), before + 2);
    }
}
