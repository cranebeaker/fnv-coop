//! `fallout shared/havok/cameracaster.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CameraCaster` (Xbox PDB, 8 bytes) holds two `NiPointer`s to
//! `bhkSimpleShapePhantom`s (a sphere for the player, one for the horse)
//! and casts the player's sphere along a line (`Pick`) to find what the
//! camera would hit. The unit also holds the Havok collector code the cast
//! uses: `hkpCdPointCollector` (vtable `0104da3c`, RTTI) and
//! `hkpAllCdPointCollector` (vtable `0104da28`, RTTI; 0x3A0 bytes, hits at
//! +0x10 as an `hkInplaceArray<hkpRootCdPoint, 8>`), and a few helpers the
//! linker placed here.
//!
//! The compiler's exception frames (`FS:[0]`) and `/GS` cookie checks are
//! not translated; the functions that have one are marked `Translation
//! incomplete`. A float copied through `FLD`/`FSTP` goes through
//! [`x87_quiet`]; floats pushed as arguments are passed as their bit
//! pattern.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `CameraCaster` (Xbox PDB), 8 bytes.
    pub struct CameraCaster: 0x08 {
        /// `spPlayerShapePhantom` (Xbox PDB): `NiPointer<bhkSimpleShapePhantom>`.
        0x00 spPlayerShapePhantom: Ptr,
        /// `spHorseShapePhantom` (Xbox PDB): `NiPointer<bhkSimpleShapePhantom>`.
        0x04 spHorseShapePhantom: Ptr,
    }
}

/// `hkpCdPointCollector`'s vtable (RTTI `.?AVhkpCdPointCollector@@`).
const CD_POINT_COLLECTOR_VTABLE: u32 = 0x0104_da3c;
/// `hkpAllCdPointCollector`'s vtable (RTTI `.?AVhkpAllCdPointCollector@@`).
const ALL_CD_POINT_COLLECTOR_VTABLE: u32 = 0x0104_da28;
/// The early-out distance a reset collector gets (`float`, `0x7F7FFFEE`).
const EARLY_OUT: u32 = 0x0104_da48;
/// `FLT_EPSILON` (`float`): the two tolerances of a linear cast input.
const CAST_TOLERANCE: u32 = 0x0104_da4c;
/// The VATS singleton (`ECX` of `VATS::GetCurrentAction`).
const VATS: u32 = 0x011f_2250;
/// A global reference `Pick` falls back on in VATS without an action.
const VATS_FALLBACK_TARGET: u32 = 0x011f_21cc;
/// The player character pointer.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// The critical section `00c9eba0` holds (`EnterCriticalSection` /
/// `LeaveCriticalSection` import slots).
const WORLD_LOCK: u32 = 0x0126_820c;
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;

/// `NiPointer` constructor from a raw pointer (`00633c90`).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer` destructor (`0045cec0`).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `NiPointer` assignment (`0066b0d0`).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// The pointer a `NiPointer` holds (`00559450`: `*this`).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// The game heap allocation (`00aa13e0`, cdecl `(size)`).
const ALLOCATE: u32 = 0x00aa_13e0;
/// The memory router (`00c85750`) and its allocator getter (`0044edb0`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const ROUTER_ALLOCATOR: u32 = 0x0044_edb0;
/// `hkVector4` construction (`006815c0`, returns `this`).
const VECTOR_CONSTRUCT: u32 = 0x0068_15c0;

fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 00620850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00620850 - the FS:[0] exception frame (its unwind destroys the two NiPointers) is not translated
/// `CameraCaster::CameraCaster` (Xbox PDB): two null `NiPointer`s, then the
/// phantoms (`00620930` with `radius` and `filter`). Returns `this`.
pub fn camera_caster_ctor(
    e: &mut Engine,
    this: Ptr<CameraCaster>,
    radius: f32,
    filter: u32,
) -> Ptr<CameraCaster> {
    e.call(NI_POINTER_CONSTRUCT, &args![this, 0u32]);
    e.call(NI_POINTER_CONSTRUCT, &args![this.addr() + 4, 0u32]);
    fn_00620930(e, this, x87_quiet(radius), filter);
    this
}

// Translated from 006208d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006208d0 - the FS:[0] exception frame is not translated
/// `CameraCaster::~CameraCaster` (Xbox PDB): releases the horse phantom,
/// then the player's.
pub fn camera_caster_dtor(e: &mut Engine, this: Ptr<CameraCaster>) {
    e.call(NI_POINTER_DESTRUCT, &args![this.addr() + 4]);
    e.call(NI_POINTER_DESTRUCT, &args![this]);
}

/// One phantom of `00620930`: fills the construction info at `info`
/// (0x60 bytes: shape `info + 0`, the sphere's `004ae750` at `info + 4`,
/// `0056e120` on `info + 0x20`), allocates 0x18 bytes, builds the phantom
/// (`bhkSimpleShapePhantom::bhkSimpleShapePhantom`, null when the
/// allocation failed) and assigns it to the `NiPointer` at `slot`.
fn make_phantom(e: &mut Engine, filter_info: u32, sphere: u32, info: u32, slot: u32) {
    let shape_filter = e.call(NI_POINTER_GET, &args![filter_info]).u32();
    e.mem.set_u32(info, shape_filter);
    // `bhkCharacterProxy::operatorP` (Xbox PDB name of `004ae750`).
    let shape = e.call(0x004a_e750, &args![sphere]).u32();
    e.mem.set_u32(info + 4, shape);
    e.call(0x0056_e120, &args![info + 0x20]);
    let block = e.call(ALLOCATE, &args![0x18u32]).u32();
    let phantom = if block == 0 {
        0
    } else {
        e.call(0x0056_e2d0, &args![block, info]).u32()
    };
    e.call(NI_POINTER_ASSIGN, &args![slot, phantom]);
}

// Translated from 00620930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00620930 - the FS:[0] exception frame (its unwind frees the shape and destroys the construction infos) is not translated
/// Makes the caster's phantoms: a sphere shape of `radius` (0x14 bytes,
/// `bhkSphereShape::bhkSphereShape(radius, 1)`), and two
/// `bhkSimpleShapePhantom`s on it: the player's with collision filter
/// layer 0x23 and `filter`, the horse's with layer 0x23 and `005dbe10()`.
/// The two construction infos are built with `0056e090` and destroyed with
/// `0056ea90`; the filter words with `008c71b0(0)`, `004a39f0(0x23)` and
/// `0059ce80`.
pub fn fn_00620930(e: &mut Engine, this: Ptr<CameraCaster>, radius: f32, filter: u32) {
    let block = e.call(ALLOCATE, &args![0x14u32]).u32();
    let sphere = if block == 0 {
        0
    } else {
        // `bhkSphereShape::bhkSphereShape` (Xbox PDB).
        e.call(
            0x0056_e950,
            &args![block, x87_quiet_bits(radius.to_bits()), 1u32],
        )
        .u32()
    };
    e.with_stack(0xc8, |e, frame| {
        let player_info = frame.addr();
        let horse_info = frame.addr() + 0x60;
        let player_filter = frame.addr() + 0xc0;
        let horse_filter = frame.addr() + 0xc4;
        e.call(0x0056_e090, &args![player_info]);
        e.call(0x0056_e090, &args![horse_info]);
        e.call(0x008c_71b0, &args![player_filter, 0u32]);
        e.call(0x008c_71b0, &args![horse_filter, 0u32]);
        e.call(0x004a_39f0, &args![player_filter, 0x23u32]);
        e.call(0x0059_ce80, &args![player_filter, filter]);
        e.call(0x004a_39f0, &args![horse_filter, 0x23u32]);
        let horse = e.call(0x005d_be10, &[]).u32();
        e.call(0x0059_ce80, &args![horse_filter, horse]);
        make_phantom(e, player_filter, sphere, player_info, this.addr());
        make_phantom(e, horse_filter, sphere, horse_info, this.addr() + 4);
        e.call(0x0056_ea90, &args![horse_info]);
        e.call(0x0056_ea90, &args![player_info]);
    });
}

// Translated from 00620b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `0044ddc0(this)` (the word at +0x08 of `this`).
pub fn fn_00620b80(e: &mut Engine, this: u32) -> u32 {
    e.call(0x0044_ddc0, &args![this]).u32()
}

// Translated from 00620ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `value` to `00c85ce0` (`bhkworldobject.obj`) on the object the
/// `NiPointer` at `this` holds.
pub fn fn_00620ba0(e: &mut Engine, this: u32, value: u32) {
    let object = e.call(NI_POINTER_GET, &args![this]).u32();
    e.call(0x00c8_5ce0, &args![object, value]);
}

// Translated from 00620bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00620bc0 - the FS:[0] exception frame (its unwind destroys the collector) and the /GS cookie check 00ec408c are not translated
/// `CameraCaster::Pick` (Xbox PDB): casts the player phantom from `from`
/// to `to` (`00c9eba0` with an `hkpAllCdPointCollector`) and takes the
/// first hit (after `sortHits`) that counts; returns whether there was
/// one. On success the hit is copied to `hit` (`hkpRootCdPoint`
/// assignment) and its position written back to `to` (`00458620`).
///
/// A hit counts if its collision layer (`0043b540` / `008c71b0` /
/// `0043b4d0` on root collidable B) is 0x11, or its `NiAVObject`
/// (`bhkUtilFunctions::GetNiAVObject`) exists and fails `00456610`. Of
/// those, layers 0x13, 0x14, 0x19, 0x1E and 0x20 (and 5, 8 and 0x1D in
/// VATS, `0044ddc0(VATS) == 4`) are skipped when the hit's reference
/// (`TESObjectREFR::FindReferenceFor3D`) is the VATS target (the current
/// action's +0x0C, else `011f21cc`, else the player), or when its slot
/// 0xFC is true and its character controller passes `00621240`.
pub fn camera_caster_pick(
    e: &mut Engine,
    this: Ptr<CameraCaster>,
    from: u32,
    to: u32,
    hit: u32,
) -> bool {
    let mut result = false;
    if e.call(NI_POINTER_GET, &args![this]).u32() == 0 {
        return result;
    }
    e.with_stack(0x400, |e, frame| {
        let collector = frame.addr();
        let input = frame.addr() + 0x3a0;
        let start = frame.addr() + 0x3c0;
        let has_hit = frame.addr() + 0x3d0;
        let filter = frame.addr() + 0x3d4;
        e.call(VECTOR_CONSTRUCT, &args![start]);
        hkp_all_cd_point_collector_ctor(e, collector);
        fn_00621290(e, input);
        e.call(0x004a_3e00, &args![start, from]);
        e.call(0x004a_3e00, &args![input, to]);
        let phantom = e.call(NI_POINTER_GET, &args![this]).u32();
        fn_00c9eba0(e, phantom, start, input, collector, 0);
        let flag = fn_00621210(e, collector, has_hit);
        result = e.call(0x0056_09f0, &args![flag]).bool();
        if result {
            // `hkpAllCdPointCollector::sortHits` (Xbox PDB).
            e.call(0x00ca_bad0, &args![collector]);
            let mut index = 0i32;
            let mut found = false;
            let in_vats = e.call(0x0044_ddc0, &args![VATS]).u32() == 4;
            let mut target = 0;
            if in_vats {
                // `VATS::GetCurrentAction` (Xbox PDB).
                let action = e.call(0x009c_71c0, &args![VATS]).u32();
                target = if action != 0 {
                    e.mem.u32(action + 0xc)
                } else {
                    e.global::<u32>(VATS_FALLBACK_TARGET)
                };
                if target == 0 {
                    target = e.global::<u32>(PLAYER_CHARACTER);
                }
            }
            while !found {
                let hits = e.call(0x0046_0140, &args![collector]).u32();
                let count = e.call(0x0072_6070, &args![hits]).i32();
                if index >= count {
                    break;
                }
                let hits = e.call(0x0046_0140, &args![collector]).u32();
                let point = e.call(0x0080_7770, &args![hits, index]).u32();
                let collidable = e.mem.u32(point + 0x48);
                let info = e.call(0x0043_b540, &args![collidable]).u32();
                e.call(0x008c_71b0, &args![filter, info]);
                let layer = e.call(0x0043_b4d0, &args![filter]).u32();
                // `bhkUtilFunctions::GetNiAVObject` (Xbox PDB), cdecl.
                let object = e.call(0x00c7_fa90, &args![collidable]).u32();
                let counts =
                    layer == 0x11 || (object != 0 && !e.call(0x0045_6610, &args![object]).bool());
                if !counts {
                    index += 1;
                    continue;
                }
                found = true;
                // The jump table at `00620fc8` with its index bytes at
                // `00620fd4`.
                let check = match layer {
                    5 | 8 | 0x1d => in_vats,
                    0x13 | 0x14 | 0x19 | 0x1e | 0x20 => true,
                    _ => false,
                };
                if check && object != 0 {
                    // `TESObjectREFR::FindReferenceFor3D` (Xbox PDB), cdecl.
                    let reference = e.call(0x0056_f930, &args![object]).u32();
                    if reference != 0 {
                        if in_vats && reference == target {
                            found = false;
                        } else if e.vcall(reference, 0xfc, &[]).bool() {
                            // `MobileObject::GetCharController` (Xbox PDB).
                            let controller = e.call(0x0093_06d0, &args![reference]).u32();
                            if controller != 0 && fn_00621240(e, controller) {
                                found = false;
                            }
                        }
                    }
                }
                if !found {
                    index += 1;
                } else if in_vats {
                    // The code reads the hit's root collidable B into a
                    // local it does not use.
                    let hits = e.call(0x0046_0140, &args![collector]).u32();
                    e.call(0x0080_7770, &args![hits, index]);
                }
            }
            if found {
                let hits = e.call(0x0046_0140, &args![collector]).u32();
                let point = e.call(0x0080_7770, &args![hits, index]).u32();
                hkp_root_cd_point_assign(e, hit, point);
                let position = e.call(VECTOR_CONSTRUCT, &args![hit]).u32();
                e.call(0x0045_8620, &args![to, position]);
            } else {
                result = false;
            }
        }
        hkp_all_cd_point_collector_dtor(e, collector);
    });
    result
}

// Translated from 00620ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00620ff0 - the FS:[0] exception frame is not translated
/// The `hkpAllCdPointCollector` constructor (its vtable `0104da28`, RTTI):
/// the base collector, the in-place hit array (`00621bc0` with 0), then
/// `reset`. Returns `this`.
pub fn hkp_all_cd_point_collector_ctor(e: &mut Engine, this: u32) -> u32 {
    hkp_cd_point_collector_ctor(e, this);
    e.mem.set_u32(this, ALL_CD_POINT_COLLECTOR_VTABLE);
    fn_00621bc0(e, this + 0x10, 0);
    hkp_all_cd_point_collector_reset(e, this);
    this
}

// Translated from 00621060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpCdPointCollector` constructor (vtable `0104da3c`, RTTI), then
/// `reset`. Returns `this`.
pub fn hkp_cd_point_collector_ctor(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, CD_POINT_COLLECTOR_VTABLE);
    hkp_cd_point_collector_reset(e, this);
    this
}

// Translated from 00621080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCdPointCollector::reset` (Xbox PDB): `m_earlyOutDistance` (+0x04)
/// back to `0104da48`.
pub fn hkp_cd_point_collector_reset(e: &mut Engine, this: u32) {
    let distance = x87_quiet(e.global::<f32>(EARLY_OUT));
    e.mem.set_f32(this + 4, distance);
}

// Translated from 006210a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCdPointCollector`'s scalar deleting destructor: the destructor,
/// then, with bit 0 of `flags`, `006210d0`. Returns `this`.
pub fn fn_006210a0(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_00621100(e, this);
    if flags & 1 != 0 {
        fn_006210d0(e, this);
    }
    this
}

/// Frees `block` of `size` bytes through the router's allocator (slot 8).
fn free_sized(e: &mut Engine, block: u32, size: u32) {
    if block == 0 {
        return;
    }
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    let allocator = e.call(ROUTER_ALLOCATOR, &args![router]).u32();
    e.vcall(allocator, 8, &args![block, size]);
}

// Translated from 006210d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees an 8-byte `hkpCdPointCollector` (cdecl): the memory router's
/// allocator (`0044edb0`), slot 8 `(block, 8)`; nothing for null.
pub fn fn_006210d0(e: &mut Engine, block: u32) {
    free_sized(e, block, 8);
}

// Translated from 00621100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpCdPointCollector` destructor: puts back its vtable.
pub fn fn_00621100(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, CD_POINT_COLLECTOR_VTABLE);
}

// Translated from 00621120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAllCdPointCollector::reset` (Xbox PDB): empties the hits
/// (`00626870` on +0x10) and resets the base.
pub fn hkp_all_cd_point_collector_reset(e: &mut Engine, this: u32) {
    e.call(0x0062_6870, &args![this + 0x10]);
    hkp_cd_point_collector_reset(e, this);
}

// Translated from 00621140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpAllCdPointCollector::`scalar deleting destructor'` (Xbox PDB): the
/// destructor, then, with bit 0 of `flags`, `00621170`. Returns `this`.
pub fn hkp_all_cd_point_collector_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    hkp_all_cd_point_collector_dtor(e, this);
    if flags & 1 != 0 {
        fn_00621170(e, this);
    }
    this
}

// Translated from 00621170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees a 0x3A0-byte `hkpAllCdPointCollector` (cdecl), like `006210d0`.
pub fn fn_00621170(e: &mut Engine, block: u32) {
    free_sized(e, block, 0x3a0);
}

// Translated from 006211b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006211b0 - the FS:[0] exception frame is not translated
/// `hkpAllCdPointCollector::~hkpAllCdPointCollector` (Xbox PDB): its
/// vtable, the hit array's destructor (`00621b40`), the base destructor.
pub fn hkp_all_cd_point_collector_dtor(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, ALL_CD_POINT_COLLECTOR_VTABLE);
    fn_00621b40(e, this + 0x10);
    fn_00621100(e, this);
}

// Translated from 00621210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes into the `hkBool` at `result` whether the collector has hits
/// (`00726070` on +0x10 greater than 0, through `00622570`); returns
/// `result`.
pub fn fn_00621210(e: &mut Engine, this: u32, result: u32) -> u32 {
    let count = e.call(0x0072_6070, &args![this + 0x10]).i32();
    e.call(0x0062_2570, &args![result, (count > 0) as u32]);
    result
}

// Translated from 00621240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x800 is set in the flags of the controller part at +0x410
/// (`00621270`).
pub fn fn_00621240(e: &mut Engine, this: u32) -> bool {
    fn_00621270(e, this.wrapping_add(0x410), 0x800) != 0
}

// Translated from 00621270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x04 masked with `mask`.
pub fn fn_00621270(e: &mut Engine, this: u32, mask: u32) -> u32 {
    e.mem.u32(this + 4) & mask
}

// Translated from 00621290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes a linear cast input: the vector (`006815c0`) and both
/// tolerances at +0x10 and +0x14 to `FLT_EPSILON` (`0104da4c`). Returns
/// `this`.
pub fn fn_00621290(e: &mut Engine, this: u32) -> u32 {
    e.call(VECTOR_CONSTRUCT, &args![this]);
    let tolerance = x87_quiet(e.global::<f32>(CAST_TOLERANCE));
    e.mem.set_f32(this + 0x10, tolerance);
    e.mem.set_f32(this + 0x14, tolerance);
    this
}

// Translated from 006212c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpRootCdPoint::operator=` (Xbox PDB): the contact (`006281a0`), then
/// the root collidables, shape keys and key paths word by word. Returns
/// `this`.
pub fn hkp_root_cd_point_assign(e: &mut Engine, this: u32, other: u32) -> u32 {
    e.call(0x0062_81a0, &args![this, other]);
    copy_words(e, other + 0x20, this + 0x20, 2);
    copy_words(e, other + 0x28, this + 0x28, 8);
    copy_words(e, other + 0x48, this + 0x48, 2);
    copy_words(e, other + 0x50, this + 0x50, 8);
    this
}

// Translated from 00621a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCharacterProxy::GethkPosition` (Xbox PDB): the Havok proxy's
/// position (`hkpCharacterProxy::getPosition`) between two calls of the
/// proxy's slot 0x94 (`00c89720`), or `00458b20()` when there is no
/// Havok proxy (`00620b80`).
pub fn bhk_character_proxy_get_hk_position(e: &mut Engine, this: u32) -> u32 {
    let proxy = fn_00620b80(e, this);
    let mut position = e.call(0x0045_8b20, &[]).u32();
    if proxy != 0 {
        e.call(0x00c8_9720, &args![this]);
        position = e.call(0x00ca_c910, &args![proxy]).u32();
        e.call(0x00c8_9720, &args![this]);
    }
    position
}

// Translated from 00621b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The hit array's destructor: `00621b60`.
pub fn fn_00621b40(e: &mut Engine, this: u32) {
    e.call(0x0062_1b60, &args![this]);
}

// Translated from 00621bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00621bc0 - the FS:[0] exception frame is not translated
/// The in-place hit array's constructor: the array over its own storage
/// at +0x10 for 8 elements (`0062e230(this + 0x10, value, 8)`), then the
/// eight 0x70-byte points constructed with `00621c40` (`00401050`, the
/// vector constructor iterator). Returns `this`.
pub fn fn_00621bc0(e: &mut Engine, this: u32, value: u32) -> u32 {
    e.call(0x0062_e230, &args![this, this + 0x10, value, 8u32]);
    e.call(
        0x0040_1050,
        &args![this + 0x10, 0x70u32, 8u32, 0x0062_1c40u32],
    );
    this
}

// Translated from 00c9eba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the world object's linear cast: with `this` non-null and its world
/// (+0x08) set, holds the world lock (`0126820c`), calls `00a29680` on
/// `this`, the world's slot 0x38 with the four arguments, `00a29680`
/// again, and releases the lock. (The map names this address
/// `hkpAllCdPointCollector::hkpAllCdPointCollector`; the body is this.)
pub fn fn_00c9eba0(e: &mut Engine, this: u32, start: u32, input: u32, collector: u32, extra: u32) {
    if this == 0 {
        return;
    }
    let world = e.mem.u32(this + 8);
    if world == 0 {
        return;
    }
    e.call(ENTER_CRITICAL_SECTION, &args![WORLD_LOCK]);
    e.call(0x00a2_9680, &args![this]);
    e.vcall(world, 0x38, &args![start, input, collector, extra]);
    e.call(0x00a2_9680, &args![this]);
    e.call(LEAVE_CRITICAL_SECTION, &args![WORLD_LOCK]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00620850,
            camera_caster_ctor(Ptr<CameraCaster>, f32, u32) -> Ptr<CameraCaster>
        ),
        entry!(0x006208d0, camera_caster_dtor(Ptr<CameraCaster>)),
        entry!(0x00620930, fn_00620930(Ptr<CameraCaster>, f32, u32)),
        entry!(0x00620b80, fn_00620b80(u32) -> u32),
        entry!(0x00620ba0, fn_00620ba0(u32, u32)),
        entry!(
            0x00620bc0,
            camera_caster_pick(Ptr<CameraCaster>, u32, u32, u32) -> bool
        ),
        entry!(0x00620ff0, hkp_all_cd_point_collector_ctor(u32) -> u32),
        entry!(0x00621060, hkp_cd_point_collector_ctor(u32) -> u32),
        entry!(0x00621080, hkp_cd_point_collector_reset(u32)),
        entry!(0x006210a0, fn_006210a0(u32, u32) -> u32),
        entry!(0x006210d0, fn_006210d0(u32)),
        entry!(0x00621100, fn_00621100(u32)),
        entry!(0x00621120, hkp_all_cd_point_collector_reset(u32)),
        entry!(0x00621140, hkp_all_cd_point_collector_scalar_deleting_destructor(u32, u32) -> u32),
        entry!(0x00621170, fn_00621170(u32)),
        entry!(0x006211b0, hkp_all_cd_point_collector_dtor(u32)),
        entry!(0x00621210, fn_00621210(u32, u32) -> u32),
        entry!(0x00621240, fn_00621240(u32) -> bool),
        entry!(0x00621270, fn_00621270(u32, u32) -> u32),
        entry!(0x00621290, fn_00621290(u32) -> u32),
        entry!(0x006212c0, hkp_root_cd_point_assign(u32, u32) -> u32),
        entry!(0x00621a40, bhk_character_proxy_get_hk_position(u32) -> u32),
        entry!(0x00621b40, fn_00621b40(u32)),
        entry!(0x00621bc0, fn_00621bc0(u32, u32) -> u32),
        entry!(0x00c9eba0, fn_00c9eba0(u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCRATCH: u32 = 0x0600_0000;
    const ROUTER: u32 = 0x0610_0000;
    const ALLOCATOR: u32 = ROUTER + 0x100;
    const ALLOCATOR_TABLE: u32 = ROUTER + 0x200;
    const ALLOCATOR_FREE: u32 = 0x0700_0008;
    const WORLD: u32 = 0x0620_0000;
    const WORLD_TABLE: u32 = 0x0620_0100;
    const WORLD_CAST: u32 = 0x0700_0038;
    const REFERENCE: u32 = 0x0630_0000;
    const REFERENCE_TABLE: u32 = 0x0630_0100;
    const REFERENCE_SLOT_FC: u32 = 0x0700_00fc;

    const CALLEES: &[u32] = &[
        NI_POINTER_CONSTRUCT,
        NI_POINTER_DESTRUCT,
        NI_POINTER_ASSIGN,
        NI_POINTER_GET,
        ALLOCATE,
        MEMORY_ROUTER,
        ROUTER_ALLOCATOR,
        VECTOR_CONSTRUCT,
        ENTER_CRITICAL_SECTION,
        LEAVE_CRITICAL_SECTION,
        ALLOCATOR_FREE,
        WORLD_CAST,
        REFERENCE_SLOT_FC,
        0x004a_e750,
        0x0056_e120,
        0x0056_e2d0,
        0x0056_e950,
        0x0056_e090,
        0x008c_71b0,
        0x004a_39f0,
        0x0059_ce80,
        0x005d_be10,
        0x0056_ea90,
        0x0044_ddc0,
        0x00c8_5ce0,
        0x004a_3e00,
        0x0056_09f0,
        0x00ca_bad0,
        0x009c_71c0,
        0x0046_0140,
        0x0072_6070,
        0x0080_7770,
        0x0043_b540,
        0x0043_b4d0,
        0x00c7_fa90,
        0x0045_6610,
        0x0056_f930,
        0x0093_06d0,
        0x0045_8620,
        0x0062_6870,
        0x0062_2570,
        0x0062_81a0,
        0x0045_8b20,
        0x00c8_9720,
        0x00ca_c910,
        0x0062_1b60,
        0x0062_e230,
        0x0040_1050,
        0x00a2_9680,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &a in CALLEES {
            e.register(a, |_, _| Ret::default());
        }
        for page in [0x0104_d000, 0x011f_2000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
        e.map(SCRATCH, 0x10000);
        e.map(ROUTER, 0x1000);
        e.map(WORLD, 0x1000);
        e.map(REFERENCE, 0x1000);
        e.mem.set_u32(EARLY_OUT, 0x7f7f_ffee);
        e.mem.set_u32(CAST_TOLERANCE, 0x3400_0000);
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MEMORY_ROUTER, |_, _| ROUTER.into_ret());
        e.register(ROUTER_ALLOCATOR, |_, a| {
            assert_eq!(a[0], ROUTER);
            ALLOCATOR.into_ret()
        });
        for slot in 0..0x40 {
            e.mem
                .set_u32(ALLOCATOR_TABLE + 4 * slot, 0x0700_0000 + 4 * slot);
            e.mem
                .set_u32(WORLD_TABLE + 4 * slot, 0x0700_0000 + 4 * slot);
            e.mem
                .set_u32(REFERENCE_TABLE + 4 * slot, 0x0700_0000 + 4 * slot);
        }
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_TABLE);
        e.mem.set_u32(WORLD, WORLD_TABLE);
        e.mem.set_u32(REFERENCE, REFERENCE_TABLE);
        e
    }

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn order(e: &Engine, addrs: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addrs.contains(a))
            .map(|(a, _)| *a)
            .collect()
    }

    #[test]
    fn constructor_builds_both_phantoms() {
        let mut e = engine();
        let caster = e.new_object::<CameraCaster>();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0056_e950, |_, _| 0x5000u32.into_ret());
        e.register(0x0056_e2d0, |_, a| (a[0] + 1).into_ret());
        e.register(0x005d_be10, |_, _| 0x77u32.into_ret());
        e.register(0x004a_e750, |_, a| (a[0] + 2).into_ret());
        e.call_log = Some(vec![]);
        let r = e
            .call(0x0062_0850, &args![caster, 0x7f80_0001u32, 0x42u32])
            .u32();
        assert_eq!(r, caster.addr());
        let t = caster.addr();
        assert_eq!(
            logged(&e, NI_POINTER_CONSTRUCT),
            vec![vec![t, 0], vec![t + 4, 0]]
        );
        let sphere = logged(&e, 0x0056_e950);
        assert_eq!(&sphere[0][1..], &[0x7fc0_0001, 1]);
        let filters = logged(&e, 0x0059_ce80);
        assert_eq!(filters[0][1], 0x42);
        assert_eq!(filters[1][1], 0x77);
        assert_eq!(logged(&e, 0x004a_39f0).len(), 2);
        let infos = logged(&e, 0x0056_e2d0);
        let assigned = logged(&e, NI_POINTER_ASSIGN);
        assert_eq!(assigned[0], vec![t, infos[0][0] + 1]);
        assert_eq!(assigned[1], vec![t + 4, infos[1][0] + 1]);
        assert_eq!(e.mem.u32(infos[0][1] + 4), 0x5002);
        assert_eq!(logged(&e, 0x0056_e120)[0][0], infos[0][1] + 0x20);
        let destroyed = logged(&e, 0x0056_ea90);
        assert_eq!(destroyed, vec![vec![infos[1][1]], vec![infos[0][1]]]);
    }

    #[test]
    fn phantoms_are_null_when_allocation_fails() {
        let mut e = engine();
        let caster = e.new_object::<CameraCaster>();
        e.call_log = Some(vec![]);
        e.call(0x0062_0930, &args![caster, 1.0f32, 0x42u32]);
        assert!(logged(&e, 0x0056_e950).is_empty());
        assert!(logged(&e, 0x0056_e2d0).is_empty());
        let t = caster.addr();
        assert_eq!(
            logged(&e, NI_POINTER_ASSIGN),
            vec![vec![t, 0], vec![t + 4, 0]]
        );
        assert_eq!(logged(&e, 0x004a_e750), vec![vec![0], vec![0]]);
    }

    #[test]
    fn destructor_releases_horse_then_player() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0062_08d0, &args![SCRATCH]);
        assert_eq!(
            logged(&e, NI_POINTER_DESTRUCT),
            vec![vec![SCRATCH + 4], vec![SCRATCH]]
        );
    }

    #[test]
    fn small_getters_pass_through() {
        let mut e = engine();
        e.register(0x0044_ddc0, |_, a| (a[0] + 8).into_ret());
        assert_eq!(e.call(0x0062_0b80, &args![SCRATCH]).u32(), SCRATCH + 8);
        e.mem.set_u32(SCRATCH + 4, 0x0f0f);
        assert_eq!(
            e.call(0x0062_1270, &args![SCRATCH, 0x0ff0u32]).u32(),
            0x0f00
        );
    }

    #[test]
    fn world_object_value_goes_to_the_held_object() {
        let mut e = engine();
        e.mem.set_u32(SCRATCH, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0062_0ba0, &args![SCRATCH, 9u32]);
        assert_eq!(logged(&e, 0x00c8_5ce0), vec![vec![0x1234, 9]]);
    }

    #[test]
    fn controller_flag_0x800() {
        let mut e = engine();
        e.mem.set_u32(SCRATCH + 0x414, 0x800);
        assert!(e.call(0x0062_1240, &args![SCRATCH]).bool());
        e.mem.set_u32(SCRATCH + 0x414, 0x7ff);
        assert_eq!(e.call(0x0062_1240, &args![SCRATCH]).u32(), 0);
    }

    #[test]
    fn collectors_construct_reset_and_destroy() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let r = e.call(0x0062_0ff0, &args![SCRATCH]).u32();
        assert_eq!(r, SCRATCH);
        assert_eq!(e.mem.u32(SCRATCH), ALL_CD_POINT_COLLECTOR_VTABLE);
        assert_eq!(e.mem.u32(SCRATCH + 4), 0x7f7f_ffee);
        assert_eq!(
            logged(&e, 0x0062_e230),
            vec![vec![SCRATCH + 0x10, SCRATCH + 0x20, 0, 8]]
        );
        assert_eq!(
            logged(&e, 0x0040_1050),
            vec![vec![SCRATCH + 0x20, 0x70, 8, 0x0062_1c40]]
        );
        assert_eq!(logged(&e, 0x0062_6870), vec![vec![SCRATCH + 0x10]]);

        e.mem.set_u32(SCRATCH + 4, 0);
        e.call(0x0062_1080, &args![SCRATCH]);
        assert_eq!(e.mem.u32(SCRATCH + 4), 0x7f7f_ffee);
        e.mem.set_u32(SCRATCH + 4, 0);
        e.call(0x0062_1120, &args![SCRATCH]);
        assert_eq!(e.mem.u32(SCRATCH + 4), 0x7f7f_ffee);

        let r = e.call(0x0062_1060, &args![SCRATCH + 0x800]).u32();
        assert_eq!(r, SCRATCH + 0x800);
        assert_eq!(e.mem.u32(SCRATCH + 0x800), CD_POINT_COLLECTOR_VTABLE);

        e.call_log = Some(vec![]);
        e.call(0x0062_11b0, &args![SCRATCH]);
        assert_eq!(logged(&e, 0x0062_1b60), vec![vec![SCRATCH + 0x10]]);
        assert_eq!(e.mem.u32(SCRATCH), CD_POINT_COLLECTOR_VTABLE);
    }

    #[test]
    fn deleting_destructors_free_their_sizes() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_10a0, &args![SCRATCH, 0u32]).u32(), SCRATCH);
        assert!(logged(&e, ALLOCATOR_FREE).is_empty());
        e.call(0x0062_10a0, &args![SCRATCH, 1u32]);
        assert_eq!(
            logged(&e, ALLOCATOR_FREE),
            vec![vec![ALLOCATOR, SCRATCH, 8]]
        );
        assert_eq!(e.mem.u32(SCRATCH), CD_POINT_COLLECTOR_VTABLE);

        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_1140, &args![SCRATCH, 3u32]).u32(), SCRATCH);
        assert_eq!(
            logged(&e, ALLOCATOR_FREE),
            vec![vec![ALLOCATOR, SCRATCH, 0x3a0]]
        );
        assert_eq!(logged(&e, 0x0062_1b60).len(), 1);

        e.call_log = Some(vec![]);
        e.call(0x0062_10d0, &args![0u32]);
        e.call(0x0062_1170, &args![0u32]);
        assert!(logged(&e, MEMORY_ROUTER).is_empty());
    }

    #[test]
    fn has_hit_flag_and_cast_input() {
        let mut e = engine();
        e.register(0x0072_6070, |_, _| 2u32.into_ret());
        e.call_log = Some(vec![]);
        let r = e.call(0x0062_1210, &args![SCRATCH, SCRATCH + 0x100]).u32();
        assert_eq!(r, SCRATCH + 0x100);
        assert_eq!(logged(&e, 0x0072_6070), vec![vec![SCRATCH + 0x10]]);
        assert_eq!(logged(&e, 0x0062_2570), vec![vec![SCRATCH + 0x100, 1]]);
        e.register(0x0072_6070, |_, _| u32::MAX.into_ret());
        e.call(0x0062_1210, &args![SCRATCH, SCRATCH + 0x100]);
        assert_eq!(logged(&e, 0x0062_2570)[1][1], 0);

        assert_eq!(e.call(0x0062_1290, &args![SCRATCH]).u32(), SCRATCH);
        assert_eq!(e.mem.u32(SCRATCH + 0x10), 0x3400_0000);
        assert_eq!(e.mem.u32(SCRATCH + 0x14), 0x3400_0000);
    }

    #[test]
    fn root_point_assignment_copies_the_fields() {
        let mut e = engine();
        let (to, from) = (SCRATCH, SCRATCH + 0x100);
        for i in 0..0x1c {
            e.mem.set_u32(from + 4 * i, 0x100 + i);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_12c0, &args![to, from]).u32(), to);
        assert_eq!(logged(&e, 0x0062_81a0), vec![vec![to, from]]);
        assert_eq!(e.mem.u32(to), 0);
        for i in 8..0x1c {
            assert_eq!(e.mem.u32(to + 4 * i), 0x100 + i);
        }
    }

    #[test]
    fn proxy_position_reads_the_havok_proxy_under_the_marks() {
        let mut e = engine();
        e.register(0x0045_8b20, |_, _| 0x0126_7e30u32.into_ret());
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x00ca_c910, |_, a| (a[0] + 0x40).into_ret());
        assert_eq!(e.call(0x0062_1a40, &args![SCRATCH]).u32(), 0x0126_7e30);
        e.mem.set_u32(SCRATCH + 8, 0x9000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0062_1a40, &args![SCRATCH]).u32(), 0x9040);
        assert_eq!(logged(&e, 0x00c8_9720), vec![vec![SCRATCH], vec![SCRATCH]]);
        assert_eq!(
            order(&e, &[0x00c8_9720, 0x00ca_c910]),
            vec![0x00c8_9720, 0x00ca_c910, 0x00c8_9720]
        );
    }

    #[test]
    fn linear_cast_runs_under_the_world_lock() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x00c9_eba0, &args![0u32, 1u32, 2u32, 3u32, 4u32]);
        e.call(0x00c9_eba0, &args![SCRATCH, 1u32, 2u32, 3u32, 4u32]);
        assert!(logged(&e, ENTER_CRITICAL_SECTION).is_empty());
        e.mem.set_u32(SCRATCH + 8, WORLD);
        e.call(0x00c9_eba0, &args![SCRATCH, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(logged(&e, WORLD_CAST), vec![vec![WORLD, 1, 2, 3, 4]]);
        assert_eq!(
            order(
                &e,
                &[
                    ENTER_CRITICAL_SECTION,
                    0x00a2_9680,
                    WORLD_CAST,
                    LEAVE_CRITICAL_SECTION
                ]
            ),
            vec![
                ENTER_CRITICAL_SECTION,
                0x00a2_9680,
                WORLD_CAST,
                0x00a2_9680,
                LEAVE_CRITICAL_SECTION
            ]
        );
        assert_eq!(logged(&e, ENTER_CRITICAL_SECTION), vec![vec![WORLD_LOCK]]);
    }

    #[test]
    fn hit_array_destructor_and_constructor() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0062_1b40, &args![SCRATCH]);
        assert_eq!(logged(&e, 0x0062_1b60), vec![vec![SCRATCH]]);
        assert_eq!(e.call(0x0062_1bc0, &args![SCRATCH, 5u32]).u32(), SCRATCH);
        assert_eq!(
            logged(&e, 0x0062_e230),
            vec![vec![SCRATCH, SCRATCH + 0x10, 5, 8]]
        );
    }

    /// A `Pick` set-up: the player phantom set, three hits whose layers are
    /// `layers`, each with an `NiAVObject` and a reference.
    fn pick_engine(layers: [u32; 3]) -> (Engine, Ptr<CameraCaster>) {
        let mut e = engine();
        let caster = e.new_object::<CameraCaster>();
        e.map(0x0640_0000, 0x1000);
        e.mem.set_u32(caster.addr(), 0x0640_0000);
        e.register(0x0056_09f0, |_, _| true.into_ret());
        e.register(0x0046_0140, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0072_6070, |_, _| 3u32.into_ret());
        e.register(0x0080_7770, |_, a| {
            (SCRATCH + 0x1000 + 0x70 * a[1]).into_ret()
        });
        for i in 0..3 {
            e.mem.set_u32(SCRATCH + 0x1000 + 0x70 * i + 0x48, 0x10 + i);
        }
        e.register(0x0043_b540, |_, a| a[0].into_ret());
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register_double(0x0043_b4d0, move |e, a| {
            layers[(e.mem.u32(a[0]) - 0x10) as usize].into_ret()
        });
        e.register(0x00c7_fa90, |_, a| (0x200 + a[0]).into_ret());
        e.register(0x0056_f930, |_, _| REFERENCE.into_ret());
        e.register(VECTOR_CONSTRUCT, |_, a| a[0].into_ret());
        (e, caster)
    }

    #[test]
    fn pick_takes_the_first_counting_hit() {
        let (mut e, caster) = pick_engine([0x13, 0x11, 0x5]);
        // The first hit's reference is a controller with flag 0x800: skipped.
        e.register(REFERENCE_SLOT_FC, |_, _| true.into_ret());
        e.register(0x0093_06d0, |_, _| SCRATCH.into_ret());
        e.mem.set_u32(SCRATCH + 0x414, 0x800);
        e.call_log = Some(vec![]);
        let found = e
            .call(
                0x0062_0bc0,
                &args![caster, 0xaaa0u32, 0xbbb0u32, SCRATCH + 0x3000],
            )
            .bool();
        assert!(found);
        let cast = logged(&e, 0x00a2_9680);
        assert_eq!(cast.len(), 0, "no world on the phantom");
        assert_eq!(logged(&e, 0x00ca_bad0).len(), 1);
        let assigned = logged(&e, 0x0062_81a0);
        assert_eq!(assigned, vec![vec![SCRATCH + 0x3000, SCRATCH + 0x1070]]);
        assert_eq!(
            logged(&e, 0x0045_8620),
            vec![vec![0xbbb0, SCRATCH + 0x3000]]
        );
        let inputs = logged(&e, 0x004a_3e00);
        assert_eq!(inputs[0][1], 0xaaa0);
        assert_eq!(inputs[1][1], 0xbbb0);
        assert_eq!(logged(&e, 0x0062_1b60).len(), 1);
    }

    #[test]
    fn pick_in_vats_skips_the_target_and_fails_without_hits() {
        let (mut e, caster) = pick_engine([0x5, 0x5, 0x5]);
        e.register(0x0044_ddc0, |_, a| {
            assert_eq!(a[0], VATS);
            4u32.into_ret()
        });
        e.set_global(VATS_FALLBACK_TARGET, 0u32);
        e.set_global(PLAYER_CHARACTER, REFERENCE);
        // Every hit fails `00456610` false: each counts, and each is the
        // VATS target (the player), so nothing is found.
        e.call_log = Some(vec![]);
        let found = e
            .call(0x0062_0bc0, &args![caster, 1u32, 2u32, SCRATCH + 0x3000])
            .bool();
        assert!(!found);
        assert!(logged(&e, 0x0062_81a0).is_empty());
        assert_eq!(logged(&e, 0x0056_f930).len(), 3);

        // Without a hit flag nothing is sorted.
        e.register(0x0056_09f0, |_, _| false.into_ret());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0062_0bc0, &args![caster, 1u32, 2u32, 3u32]).bool());
        assert!(logged(&e, 0x00ca_bad0).is_empty());

        // Without a phantom nothing is built.
        e.mem.set_u32(caster.addr(), 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0062_0bc0, &args![caster, 1u32, 2u32, 3u32]).bool());
        assert!(logged(&e, 0x004a_3e00).is_empty());
    }
}
