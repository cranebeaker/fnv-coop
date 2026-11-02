//! `fallout shared/pathfinding/pathingrequestcoveredmove.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `PathingRequestCoveredMove` is a `PathingRequest` (0xB0 bytes) plus the
//! attacker data, the cover and target locations, two arrays (unreachable
//! cover locations, target locations) and two owned pointers; 0x10C bytes
//! on PC as in the Xbox PDB, and the PDB's offsets match the PC code.
//!
//! The class's layout `PathingRequestCoveredMove` is declared (partly) in
//! `pathfind.rs`; the fields it does not declare are read here at their
//! offsets, named after the Xbox PDB field.
//!
//! The compiler's exception-unwinding frames (`FS:[0]` chain and the cookie
//! word `011c16bc` stored in them) of the constructors and the destructor
//! are not translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;
use crate::units::fallout_shared_pathfinding::pathfind::PathingRequestCoveredMove;

/// `PathingRequestCoveredMove`'s vtable (slot 0 is [`pathing_request_covered_move_scalar_deleting_destructor`],
/// slot +8 [`pathing_request_covered_move_copy_to`], slot +0x10 [`fn_006e4ba0`]).
const VTABLE: u32 = 0x0106_d2d4;
/// A global three-word vector the constructor and [`fn_006e4f50`] copy into
/// `CoverLocation`/`TargetLocation` (copied as words; its value comes from
/// the exe).
const DEFAULT_POINT: u32 = 0x011f_426c;

// Offsets of `PathingRequestCoveredMove` fields (Xbox PDB names) that the
// shared layout does not declare.
/// `fAttackerRadius` (f32).
const ATTACKER_RADIUS: u32 = 0xb0;
/// `fAttackerSpeed` (f32).
const ATTACKER_SPEED: u32 = 0xb4;
/// `fAttackerHeight` (f32).
const ATTACKER_HEIGHT: u32 = 0xb8;
/// `fAttackerSneakHeight` (f32).
const ATTACKER_SNEAK_HEIGHT: u32 = 0xbc;
/// `fAttackerHealth` (f32).
const ATTACKER_HEALTH: u32 = 0xc0;
/// `iLocationIndex` (i32).
const LOCATION_INDEX: u32 = 0xc4;
/// `CoverLocation` (`NiPoint3`).
const COVER_LOCATION: u32 = 0xc8;
/// `TargetLocation` (`NiPoint3`).
const TARGET_LOCATION: u32 = 0xd4;
/// `bHasTargetLocation` (bool).
const HAS_TARGET_LOCATION: u32 = 0xe0;
/// `bFinished` (bool).
const FINISHED: u32 = 0xe1;
/// `UnreachableCoverLocations` (`BSSimpleArray<unsigned int,1024>`).
const UNREACHABLE_COVER_LOCATIONS: u32 = 0xe4;
/// `TargetLocations` (`BSSimpleArray<NiPoint3,1024>`).
const TARGET_LOCATIONS: u32 = 0xf4;

/// `PathingRequest::PathingRequest` (`006e2420`, Xbox PDB; thiscall).
const PATHING_REQUEST_CONSTRUCTOR: u32 = 0x006e_2420;
/// `PathingRequest::PathingRequest_ov2` (`006e26a0`, Xbox PDB; thiscall
/// `(this, source)`).
const PATHING_REQUEST_COPY_CONSTRUCTOR: u32 = 0x006e_26a0;
/// `006e2620` (thiscall): the base class teardown the destructor ends with
/// (it stores the vtable `0106d030`).
const PATHING_REQUEST_TEARDOWN: u32 = 0x006e_2620;
/// `PathingRequest::CopyTo` (`006e2750`, Xbox PDB; thiscall `(this, dest)`).
const PATHING_REQUEST_COPY_TO: u32 = 0x006e_2750;
/// `PathingRequest::SaveGame` (`006e3230`, Xbox PDB; thiscall `(this, buffer)`).
const PATHING_REQUEST_SAVE_GAME: u32 = 0x006e_3230;
/// `PathingRequest::LoadGame` (`006e34f0`, Xbox PDB; thiscall `(this, buffer)`).
const PATHING_REQUEST_LOAD_GAME: u32 = 0x006e_34f0;
/// `006815c0` (thiscall): the constructor run on both `NiPoint3` members.
const POINT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// `00438660` (thiscall): the constructor run on `UnreachableCoverLocations`.
const UNREACHABLE_ARRAY_CONSTRUCTOR: u32 = 0x0043_8660;
/// `0049f1c0` (thiscall): the constructor run on `TargetLocations`.
const TARGET_ARRAY_CONSTRUCTOR: u32 = 0x0049_f1c0;
/// `0049f1f0` (thiscall): the destructor run on `TargetLocations`.
const TARGET_ARRAY_DESTRUCTOR: u32 = 0x0049_f1f0;
/// `00438690` (thiscall): the destructor run on `UnreachableCoverLocations`.
const UNREACHABLE_ARRAY_DESTRUCTOR: u32 = 0x0043_8690;
/// `006d3370` (thiscall `(object, flags)`): the deleting destructor run on
/// `pCombatCoverLocation`.
const COVER_LOCATION_DELETING_DESTRUCTOR: u32 = 0x006d_3370;
/// `006dae90` (thiscall): run on `+0x20` of the object [`fn_006e4d00`]
/// tears down.
const COVER_SEARCH_INFO_MEMBER_TEARDOWN: u32 = 0x006d_ae90;
/// `00aa1460(block, size)` (cdecl): frees a block of `size` bytes through
/// the memory manager (nothing for a null block).
const DEALLOCATE: u32 = 0x00aa_1460;
/// `operator delete` (`00401030`, cdecl `(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Setters `CopyTo` calls on the destination (thiscall, one word each); the
// engine map gives them other units' names because of code folding.
const SET_ATTACKER_RADIUS: u32 = 0x0050_7610;
const SET_ATTACKER_SPEED: u32 = 0x006e_5ee0;
const SET_ATTACKER_SNEAK_HEIGHT: u32 = 0x006e_4320;
const SET_ATTACKER_HEALTH: u32 = 0x0050_76c0;
const SET_LOCATION_INDEX: u32 = 0x0070_5b10;
/// `006d30b0` (thiscall `(dest, point)`): takes the cover location.
const SET_COVER_LOCATION: u32 = 0x006d_30b0;
/// `00613fb0` (thiscall `(dest, byte)`): takes `bFinished`.
const SET_FINISHED: u32 = 0x0061_3fb0;
/// `BSSimpleArray<TESObjectWEAP*,1024>::SetSize` (`006f2290`, Xbox PDB name
/// of the folded body; thiscall `(array, size, keep)`).
const WORD_ARRAY_SET_SIZE: u32 = 0x006f_2290;
/// `BSSimpleArray<CombatSearchDoor,1024>::SetSize` (`0069cfe0`, Xbox PDB
/// name of the folded body; thiscall `(source, dest)`): copies the target
/// locations into the destination's array.
const POINT_ARRAY_COPY_TO: u32 = 0x0069_cfe0;
/// `0069cea0` (thiscall `(array, size, keep)`): sizes `TargetLocations` on
/// load.
const POINT_ARRAY_SET_SIZE: u32 = 0x0069_cea0;
/// `00877a30` (thiscall `(array, index)`): address of element `index` of a
/// 4-byte-element array.
const WORD_ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// `0044ddc0` (thiscall): an array's `iSize`.
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `006a7ad0` (thiscall `(array, index)`): address of a 4-byte element.
const UNREACHABLE_ELEMENT: u32 = 0x006a_7ad0;
/// `006a7af0` (thiscall `(array, index)`): address of a 12-byte element.
const TARGET_ELEMENT: u32 = 0x006a_7af0;
/// `00865e50` (thiscall `(buffer, data, size, 0)`): writes `size` bytes to
/// the save buffer.
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (`00865f60`, Xbox PDB).
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `00864980` (thiscall `(buffer, data, size)`): reads `size` bytes from
/// the load buffer.
const LOAD_DATA: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (`00864a60`, Xbox PDB).
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;

/// The member constructors both constructors run after the base.
fn construct_members(e: &mut Engine, t: u32) {
    e.mem.set_u32(t, VTABLE);
    e.call(POINT_CONSTRUCTOR, &args![t + COVER_LOCATION]);
    e.call(POINT_CONSTRUCTOR, &args![t + TARGET_LOCATION]);
    e.call(
        UNREACHABLE_ARRAY_CONSTRUCTOR,
        &args![t + UNREACHABLE_COVER_LOCATIONS],
    );
    e.call(TARGET_ARRAY_CONSTRUCTOR, &args![t + TARGET_LOCATIONS]);
}

/// Copies the three words of [`DEFAULT_POINT`] to `dest`.
fn copy_default_point(e: &mut Engine, dest: u32) {
    for word in 0..3 {
        let value: u32 = e.global(DEFAULT_POINT + 4 * word);
        e.mem.set_u32(dest + 4 * word, value);
    }
}

// Translated from 006e4a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestCoveredMove::PathingRequestCoveredMove` (Xbox PDB): runs
/// the `PathingRequest` constructor, stores the vtable, constructs the two
/// points and two arrays, then sets the attacker speed and health to 1.0,
/// radius, height and sneak height to 0.0, the location index to 0, both
/// locations to the vector at `011f426c`, both flags to false and both
/// pointers to null. Returns `this`.
// Translation incomplete: 006e4a50 - FS:[0] exception frame (unwinding of the base and members) is not translated.
pub fn pathing_request_covered_move_pathing_request_covered_move(
    e: &mut Engine,
    this: Ptr<PathingRequestCoveredMove>,
) -> Ptr<PathingRequestCoveredMove> {
    let t = this.addr();
    e.call(PATHING_REQUEST_CONSTRUCTOR, &args![this]);
    construct_members(e, t);
    // FLD1 / FLDZ stores.
    e.mem.set_u32(t + ATTACKER_SPEED, 1.0f32.to_bits());
    e.mem.set_u32(t + ATTACKER_RADIUS, 0.0f32.to_bits());
    e.mem.set_u32(t + ATTACKER_HEIGHT, 0.0f32.to_bits());
    e.mem.set_u32(t + ATTACKER_SNEAK_HEIGHT, 0.0f32.to_bits());
    e.mem.set_u32(t + ATTACKER_HEALTH, 1.0f32.to_bits());
    e.mem.set_u32(t + LOCATION_INDEX, 0);
    copy_default_point(e, t + COVER_LOCATION);
    copy_default_point(e, t + TARGET_LOCATION);
    e.mem.set_u8(t + HAS_TARGET_LOCATION, 0);
    e.mem.set_u8(t + FINISHED, 0);
    e.set(
        this,
        PathingRequestCoveredMove::pCombatCoverLocation,
        Ptr::NULL,
    );
    e.set(
        this,
        PathingRequestCoveredMove::pCombatCoverSearchInfo,
        Ptr::NULL,
    );
    this
}

// Translated from 006e4ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; slot +0x10 of the class's vtable, the Xbox
/// PDB's `GetType` slot: returns 8. `this` is not read.
pub fn fn_006e4ba0(_e: &mut Engine, _this: Ptr<PathingRequestCoveredMove>) -> u32 {
    8
}

// Translated from 006e4bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestCoveredMove::__vecDelDtor` (Xbox PDB): runs the
/// destructor and, when bit 0 of `flags` is set, frees the 0x10C bytes
/// (`00aa1460`). Returns `this`. `RET 4`.
pub fn pathing_request_covered_move_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PathingRequestCoveredMove>,
    flags: u32,
) -> Ptr<PathingRequestCoveredMove> {
    pathing_request_covered_move_destructor(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this, 0x10cu32]);
    }
    this
}

// Translated from 006e4bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestCoveredMove::~PathingRequestCoveredMove` (Xbox PDB):
/// stores the vtable, deletes `pCombatCoverLocation` (`006d3370(p, 1)`) and
/// `pCombatCoverSearchInfo` ([`fn_006e4cd0`]`(p, 1)`) when they are set
/// (the pointers are left as they are), destroys `TargetLocations` and
/// `UnreachableCoverLocations` and ends with the base teardown `006e2620`.
// Translation incomplete: 006e4bf0 - FS:[0] exception frame (unwinding of the members) is not translated.
pub fn pathing_request_covered_move_destructor(
    e: &mut Engine,
    this: Ptr<PathingRequestCoveredMove>,
) {
    let t = this.addr();
    e.mem.set_u32(t, VTABLE);
    let cover_location = e.get(this, PathingRequestCoveredMove::pCombatCoverLocation);
    if !cover_location.is_null() {
        e.call(
            COVER_LOCATION_DELETING_DESTRUCTOR,
            &args![cover_location, 1u32],
        );
    }
    let search_info = e.get(this, PathingRequestCoveredMove::pCombatCoverSearchInfo);
    if !search_info.is_null() {
        fn_006e4cd0(e, search_info, 1);
    }
    e.call(TARGET_ARRAY_DESTRUCTOR, &args![t + TARGET_LOCATIONS]);
    e.call(
        UNREACHABLE_ARRAY_DESTRUCTOR,
        &args![t + UNREACHABLE_COVER_LOCATIONS],
    );
    e.call(PATHING_REQUEST_TEARDOWN, &args![this]);
}

// Translated from 006e4cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor the
/// destructor runs on `pCombatCoverSearchInfo`: [`fn_006e4d00`], then
/// `operator delete` when bit 0 of `flags` is set. Returns `this`. `RET 4`.
pub fn fn_006e4cd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006e4d00(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e4d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `006dae90` on `this + 0x20`.
pub fn fn_006e4d00(e: &mut Engine, this: Ptr) {
    e.call(
        COVER_SEARCH_INFO_MEMBER_TEARDOWN,
        &args![this.addr().wrapping_add(0x20)],
    );
}

// Translated from 006e4d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestCoveredMove::PathingRequestCoveredMove_ov2` (Xbox PDB):
/// the copy constructor: `PathingRequest_ov2(this, source)`, the vtable and
/// member constructors, then `source`'s
/// [`pathing_request_covered_move_copy_to`] into `this`. Returns `this`.
/// `RET 4`.
// Translation incomplete: 006e4d20 - FS:[0] exception frame (unwinding of the base and members) is not translated.
pub fn pathing_request_covered_move_pathing_request_covered_move_ov2(
    e: &mut Engine,
    this: Ptr<PathingRequestCoveredMove>,
    source: Ptr<PathingRequestCoveredMove>,
) -> Ptr<PathingRequestCoveredMove> {
    e.call(PATHING_REQUEST_COPY_CONSTRUCTOR, &args![this, source]);
    construct_members(e, this.addr());
    pathing_request_covered_move_copy_to(e, source, this);
    this
}

// Translated from 006e4dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestCoveredMove::CopyTo` (Xbox PDB): copies the base
/// (`PathingRequest::CopyTo`), then every field into `dest` through its
/// setters: the five attacker floats (each loaded and stored through x87,
/// so a signalling NaN is passed quiet), the location index, the cover
/// location, the target location ([`fn_006e4f10`]) or its reset
/// ([`fn_006e4f50`]) by `bHasTargetLocation`, `bFinished`, and both arrays
/// ([`bs_simple_array_unsigned_int_copy_to`] and `0069cfe0`). `RET 4`.
pub fn pathing_request_covered_move_copy_to(
    e: &mut Engine,
    this: Ptr<PathingRequestCoveredMove>,
    dest: Ptr<PathingRequestCoveredMove>,
) {
    let t = this.addr();
    e.call(PATHING_REQUEST_COPY_TO, &args![this, dest]);
    let radius = x87_quiet_bits(e.mem.u32(t + ATTACKER_RADIUS));
    e.call(SET_ATTACKER_RADIUS, &args![dest, radius]);
    let speed = x87_quiet_bits(e.mem.u32(t + ATTACKER_SPEED));
    e.call(SET_ATTACKER_SPEED, &args![dest, speed]);
    let height = x87_quiet_bits(e.mem.u32(t + ATTACKER_HEIGHT));
    fn_006e4ef0(e, dest, f32::from_bits(height));
    let sneak_height = x87_quiet_bits(e.mem.u32(t + ATTACKER_SNEAK_HEIGHT));
    e.call(SET_ATTACKER_SNEAK_HEIGHT, &args![dest, sneak_height]);
    let health = x87_quiet_bits(e.mem.u32(t + ATTACKER_HEALTH));
    e.call(SET_ATTACKER_HEALTH, &args![dest, health]);
    let index = e.mem.u32(t + LOCATION_INDEX);
    e.call(SET_LOCATION_INDEX, &args![dest, index]);
    e.call(SET_COVER_LOCATION, &args![dest, t + COVER_LOCATION]);
    if e.mem.u8(t + HAS_TARGET_LOCATION) != 0 {
        fn_006e4f10(e, dest, Ptr::new(t + TARGET_LOCATION));
    } else {
        fn_006e4f50(e, dest);
    }
    let finished = e.mem.u8(t + FINISHED) as u32;
    e.call(SET_FINISHED, &args![dest, finished]);
    let unreachable = fn_006e4f90(e, dest);
    bs_simple_array_unsigned_int_copy_to(e, Ptr::new(t + UNREACHABLE_COVER_LOCATIONS), unreachable);
    let targets = fn_006e4fb0(e, dest);
    e.call(POINT_ARRAY_COPY_TO, &args![t + TARGET_LOCATIONS, targets]);
}

// Translated from 006e4ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stores `value` in `fAttackerHeight` (through
/// x87, so a signalling NaN is stored quiet). `RET 4`.
pub fn fn_006e4ef0(e: &mut Engine, this: Ptr<PathingRequestCoveredMove>, value: f32) {
    e.mem.set_u32(
        this.addr() + ATTACKER_HEIGHT,
        x87_quiet_bits(value.to_bits()),
    );
}

// Translated from 006e4f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the three words at `point` into
/// `TargetLocation` and sets `bHasTargetLocation`. `RET 4`.
pub fn fn_006e4f10(e: &mut Engine, this: Ptr<PathingRequestCoveredMove>, point: Ptr) {
    let t = this.addr();
    for word in 0..3 {
        let value = e.mem.u32(point.addr() + 4 * word);
        e.mem.set_u32(t + TARGET_LOCATION + 4 * word, value);
    }
    e.mem.set_u8(t + HAS_TARGET_LOCATION, 1);
}

// Translated from 006e4f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the vector at `011f426c` into
/// `TargetLocation` and clears `bHasTargetLocation`.
pub fn fn_006e4f50(e: &mut Engine, this: Ptr<PathingRequestCoveredMove>) {
    let t = this.addr();
    copy_default_point(e, t + TARGET_LOCATION);
    e.mem.set_u8(t + HAS_TARGET_LOCATION, 0);
}

// Translated from 006e4f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the address of `UnreachableCoverLocations`.
pub fn fn_006e4f90(_e: &mut Engine, this: Ptr<PathingRequestCoveredMove>) -> Ptr<BSSimpleArray> {
    Ptr::new(this.addr().wrapping_add(UNREACHABLE_COVER_LOCATIONS))
}

// Translated from 006e4fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the address of `TargetLocations`.
pub fn fn_006e4fb0(_e: &mut Engine, this: Ptr<PathingRequestCoveredMove>) -> Ptr<BSSimpleArray> {
    Ptr::new(this.addr().wrapping_add(TARGET_LOCATIONS))
}

// Translated from 006e4fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestCoveredMove::SaveGame` (Xbox PDB): saves the base, then
/// the attacker fields, location index, both locations (12 bytes each) and
/// `bHasTargetLocation` (1 byte), then each array as its size
/// (`SaveVariableSizedValue`; 0 when the array's address is 0) followed by
/// its elements (4 and 12 bytes). `bFinished` is not saved. `RET 4`.
pub fn pathing_request_covered_move_save_game(
    e: &mut Engine,
    this: Ptr<PathingRequestCoveredMove>,
    buffer: Ptr,
) {
    let t = this.addr();
    e.call(PATHING_REQUEST_SAVE_GAME, &args![this, buffer]);
    for (offset, size) in [
        (ATTACKER_RADIUS, 4u32),
        (ATTACKER_SPEED, 4),
        (ATTACKER_HEIGHT, 4),
        (ATTACKER_SNEAK_HEIGHT, 4),
        (ATTACKER_HEALTH, 4),
        (LOCATION_INDEX, 4),
        (COVER_LOCATION, 0xc),
        (TARGET_LOCATION, 0xc),
        (HAS_TARGET_LOCATION, 1),
    ] {
        e.call(
            SAVE_DATA,
            &args![buffer, t.wrapping_add(offset), size, 0u32],
        );
    }
    for (offset, element, size) in [
        (UNREACHABLE_COVER_LOCATIONS, UNREACHABLE_ELEMENT, 4u32),
        (TARGET_LOCATIONS, TARGET_ELEMENT, 0xc),
    ] {
        let array = t.wrapping_add(offset);
        let count = if array == 0 {
            0
        } else {
            e.call(ARRAY_SIZE, &args![array]).u32()
        };
        e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
        let mut index = 0u32;
        while index < count {
            let item = e.call(element, &args![array, index]).u32();
            e.call(SAVE_DATA, &args![buffer, item, size, 0u32]);
            index += 1;
        }
    }
}

// Translated from 006e51a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingRequestCoveredMove::LoadGame` (Xbox PDB): loads what
/// [`pathing_request_covered_move_save_game`] saves: the base, the fields,
/// then each array's size (`LoadVariableSizedValue`), sized with
/// `006f2290(array, n, 1)` / `0069cea0(array, n, 1)`, and its elements.
/// `RET 4`.
pub fn pathing_request_covered_move_load_game(
    e: &mut Engine,
    this: Ptr<PathingRequestCoveredMove>,
    buffer: Ptr,
) {
    let t = this.addr();
    e.call(PATHING_REQUEST_LOAD_GAME, &args![this, buffer]);
    for (offset, size) in [
        (ATTACKER_RADIUS, 4u32),
        (ATTACKER_SPEED, 4),
        (ATTACKER_HEIGHT, 4),
        (ATTACKER_SNEAK_HEIGHT, 4),
        (ATTACKER_HEALTH, 4),
        (LOCATION_INDEX, 4),
        (COVER_LOCATION, 0xc),
        (TARGET_LOCATION, 0xc),
        (HAS_TARGET_LOCATION, 1),
    ] {
        e.call(LOAD_DATA, &args![buffer, t.wrapping_add(offset), size]);
    }
    for (offset, set_size, element, size) in [
        (
            UNREACHABLE_COVER_LOCATIONS,
            WORD_ARRAY_SET_SIZE,
            UNREACHABLE_ELEMENT,
            4u32,
        ),
        (TARGET_LOCATIONS, POINT_ARRAY_SET_SIZE, TARGET_ELEMENT, 0xc),
    ] {
        let array = t.wrapping_add(offset);
        let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
        e.call(set_size, &args![array, count, 1u32]);
        let mut index = 0u32;
        while index < count {
            let item = e.call(element, &args![array, index]).u32();
            e.call(LOAD_DATA, &args![buffer, item, size]);
            index += 1;
        }
    }
}

// Translated from 006e5330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<unsigned int,1024>::CopyTo<1024>` (Xbox PDB): sizes
/// `dest` to this array's `iSize` (`006f2290(dest, iSize, 1)`) and copies
/// each word into `dest`'s element (`00877a30(dest, i)`). `RET 4`.
pub fn bs_simple_array_unsigned_int_copy_to(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    dest: Ptr<BSSimpleArray>,
) {
    let size = e.get(this, BSSimpleArray::iSize);
    e.call(WORD_ARRAY_SET_SIZE, &args![dest, size, 1u32]);
    let mut index = 0u32;
    while index < e.get(this, BSSimpleArray::iSize) {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let slot = e.call(WORD_ARRAY_ELEMENT, &args![dest, index]).u32();
        let value = e.mem.u32(buffer.wrapping_add(index.wrapping_mul(4)));
        e.mem.set_u32(slot, value);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006e4a50,
            pathing_request_covered_move_pathing_request_covered_move(
                Ptr<PathingRequestCoveredMove>,
            ) -> Ptr<
                PathingRequestCoveredMove,
            >
        ),
        entry!(
            0x006e4ba0,
            fn_006e4ba0(Ptr<PathingRequestCoveredMove>) -> u32
        ),
        entry!(
            0x006e4bb0,
            pathing_request_covered_move_scalar_deleting_destructor(
                Ptr<PathingRequestCoveredMove>,
                u32,
            ) -> Ptr<
                PathingRequestCoveredMove,
            >
        ),
        entry!(
            0x006e4bf0,
            pathing_request_covered_move_destructor(Ptr<PathingRequestCoveredMove>)
        ),
        entry!(0x006e4cd0, fn_006e4cd0(Ptr, u32) -> Ptr),
        entry!(0x006e4d00, fn_006e4d00(Ptr)),
        entry!(
            0x006e4d20,
            pathing_request_covered_move_pathing_request_covered_move_ov2(
                Ptr<PathingRequestCoveredMove>,
                Ptr<PathingRequestCoveredMove>,
            ) -> Ptr<
                PathingRequestCoveredMove,
            >
        ),
        entry!(
            0x006e4dd0,
            pathing_request_covered_move_copy_to(
                Ptr<PathingRequestCoveredMove>,
                Ptr<PathingRequestCoveredMove>,
            )
        ),
        entry!(0x006e4ef0, fn_006e4ef0(Ptr<PathingRequestCoveredMove>, f32)),
        entry!(0x006e4f10, fn_006e4f10(Ptr<PathingRequestCoveredMove>, Ptr)),
        entry!(0x006e4f50, fn_006e4f50(Ptr<PathingRequestCoveredMove>)),
        entry!(
            0x006e4f90,
            fn_006e4f90(Ptr<PathingRequestCoveredMove>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006e4fb0,
            fn_006e4fb0(Ptr<PathingRequestCoveredMove>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006e4fd0,
            pathing_request_covered_move_save_game(Ptr<PathingRequestCoveredMove>, Ptr)
        ),
        entry!(
            0x006e51a0,
            pathing_request_covered_move_load_game(Ptr<PathingRequestCoveredMove>, Ptr)
        ),
        entry!(
            0x006e5330,
            bs_simple_array_unsigned_int_copy_to(Ptr<BSSimpleArray>, Ptr<BSSimpleArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet_doubles(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    fn with_default_point(e: &mut Engine) {
        e.map(DEFAULT_POINT & !0xfff, 0x1000);
        e.set_global(DEFAULT_POINT, 1.5f32);
        e.set_global(DEFAULT_POINT + 4, 2.5f32);
        e.set_global(DEFAULT_POINT + 8, 3.5f32);
    }

    const MEMBER_CONSTRUCTORS: [u32; 3] = [
        POINT_CONSTRUCTOR,
        UNREACHABLE_ARRAY_CONSTRUCTOR,
        TARGET_ARRAY_CONSTRUCTOR,
    ];

    #[test]
    fn constructor_sets_defaults() {
        let mut e = Engine::new();
        with_default_point(&mut e);
        quiet_doubles(&mut e, &MEMBER_CONSTRUCTORS);
        quiet_doubles(&mut e, &[PATHING_REQUEST_CONSTRUCTOR]);
        let this: Ptr<PathingRequestCoveredMove> = e.new_object();
        let t = this.addr();
        for off in (0xb0..0x10c).step_by(4) {
            e.mem.set_u32(t + off, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e4a50, &args![this]).u32(), t);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, PATHING_REQUEST_CONSTRUCTOR), vec![vec![t]]);
        assert_eq!(
            calls(&log, POINT_CONSTRUCTOR),
            vec![vec![t + 0xc8], vec![t + 0xd4]]
        );
        assert_eq!(
            calls(&log, UNREACHABLE_ARRAY_CONSTRUCTOR),
            vec![vec![t + 0xe4]]
        );
        assert_eq!(calls(&log, TARGET_ARRAY_CONSTRUCTOR), vec![vec![t + 0xf4]]);
        assert_eq!(e.mem.u32(t), VTABLE);
        assert_eq!(e.mem.f32(t + 0xb0), 0.0);
        assert_eq!(e.mem.f32(t + 0xb4), 1.0);
        assert_eq!(e.mem.f32(t + 0xb8), 0.0);
        assert_eq!(e.mem.f32(t + 0xbc), 0.0);
        assert_eq!(e.mem.f32(t + 0xc0), 1.0);
        assert_eq!(e.mem.u32(t + 0xc4), 0);
        for base in [0xc8, 0xd4] {
            assert_eq!(e.mem.f32(t + base), 1.5);
            assert_eq!(e.mem.f32(t + base + 8), 3.5);
        }
        assert_eq!(e.mem.u8(t + 0xe0), 0);
        assert_eq!(e.mem.u8(t + 0xe1), 0);
        assert_eq!(e.mem.u32(t + 0x104), 0);
        assert_eq!(e.mem.u32(t + 0x108), 0);
    }

    #[test]
    fn get_type_is_eight() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x006e4ba0, &args![0x1234u32]).u32(), 8);
    }

    fn destructor_doubles(e: &mut Engine) {
        quiet_doubles(
            e,
            &[
                COVER_LOCATION_DELETING_DESTRUCTOR,
                COVER_SEARCH_INFO_MEMBER_TEARDOWN,
                OPERATOR_DELETE,
                TARGET_ARRAY_DESTRUCTOR,
                UNREACHABLE_ARRAY_DESTRUCTOR,
                PATHING_REQUEST_TEARDOWN,
                DEALLOCATE,
            ],
        );
    }

    #[test]
    fn destructor_deletes_set_pointers_and_members() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let this: Ptr<PathingRequestCoveredMove> = e.new_object();
        let t = this.addr();
        e.mem.set_u32(t + 0x104, 0x7100);
        e.mem.set_u32(t + 0x108, 0x7200);
        e.call_log = Some(vec![]);
        e.call(0x006e4bf0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(t), VTABLE);
        assert_eq!(
            calls(&log, COVER_LOCATION_DELETING_DESTRUCTOR),
            vec![vec![0x7100, 1]]
        );
        assert_eq!(
            calls(&log, COVER_SEARCH_INFO_MEMBER_TEARDOWN),
            vec![vec![0x7220]]
        );
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![0x7200]]);
        assert_eq!(calls(&log, TARGET_ARRAY_DESTRUCTOR), vec![vec![t + 0xf4]]);
        assert_eq!(
            calls(&log, UNREACHABLE_ARRAY_DESTRUCTOR),
            vec![vec![t + 0xe4]]
        );
        assert_eq!(calls(&log, PATHING_REQUEST_TEARDOWN), vec![vec![t]]);
        // The order: target array, unreachable array, then the base.
        let order: Vec<u32> = log
            .iter()
            .map(|c| c.0)
            .filter(|a| {
                [
                    TARGET_ARRAY_DESTRUCTOR,
                    UNREACHABLE_ARRAY_DESTRUCTOR,
                    PATHING_REQUEST_TEARDOWN,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                TARGET_ARRAY_DESTRUCTOR,
                UNREACHABLE_ARRAY_DESTRUCTOR,
                PATHING_REQUEST_TEARDOWN
            ]
        );

        // Null pointers are skipped.
        e.mem.set_u32(t + 0x104, 0);
        e.mem.set_u32(t + 0x108, 0);
        e.call_log = Some(vec![]);
        e.call(0x006e4bf0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, COVER_LOCATION_DELETING_DESTRUCTOR).is_empty());
        assert!(calls(&log, COVER_SEARCH_INFO_MEMBER_TEARDOWN).is_empty());
    }

    #[test]
    fn scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let this: Ptr<PathingRequestCoveredMove> = e.new_object();
        for (flags, frees) in [(1u32, 1usize), (0, 0), (3, 1), (2, 0)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e4bb0, &args![this, flags]).u32(), this.addr());
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, PATHING_REQUEST_TEARDOWN).len(), 1);
            let frees_logged = calls(&log, DEALLOCATE);
            assert_eq!(frees_logged.len(), frees);
            if frees == 1 {
                assert_eq!(frees_logged[0], vec![this.addr(), 0x10c]);
            }
        }
    }

    #[test]
    fn search_info_deleting_destructor() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        for (flags, frees) in [(1u32, 1usize), (0, 0)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006e4cd0, &args![0x7200u32, flags]).u32(), 0x7200);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls(&log, COVER_SEARCH_INFO_MEMBER_TEARDOWN),
                vec![vec![0x7220]]
            );
            assert_eq!(calls(&log, OPERATOR_DELETE).len(), frees);
        }
        e.call_log = Some(vec![]);
        e.call(0x006e4d00, &args![0x7300u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, COVER_SEARCH_INFO_MEMBER_TEARDOWN),
            vec![vec![0x7320]]
        );
    }

    fn copy_doubles(e: &mut Engine) {
        quiet_doubles(
            e,
            &[
                PATHING_REQUEST_COPY_TO,
                SET_ATTACKER_RADIUS,
                SET_ATTACKER_SPEED,
                SET_ATTACKER_SNEAK_HEIGHT,
                SET_ATTACKER_HEALTH,
                SET_LOCATION_INDEX,
                SET_COVER_LOCATION,
                SET_FINISHED,
                WORD_ARRAY_SET_SIZE,
                POINT_ARRAY_COPY_TO,
            ],
        );
    }

    /// A source request with distinct field values and an empty word array.
    fn source_request(e: &mut Engine) -> Ptr<PathingRequestCoveredMove> {
        let this: Ptr<PathingRequestCoveredMove> = e.new_object();
        let t = this.addr();
        e.mem.set_u32(t + 0xb0, 0x7f80_0001); // signalling NaN
        e.mem.set_f32(t + 0xb4, 2.0);
        e.mem.set_u32(t + 0xb8, 0x7f80_0002); // signalling NaN
        e.mem.set_f32(t + 0xbc, 4.0);
        e.mem.set_f32(t + 0xc0, 5.0);
        e.mem.set_u32(t + 0xc4, 7);
        for word in 0..3 {
            e.mem.set_u32(t + 0xd4 + 4 * word, 0x100 + word);
        }
        e.mem.set_u8(t + 0xe0, 1);
        e.mem.set_u8(t + 0xe1, 1);
        this
    }

    #[test]
    fn copy_to_passes_every_field() {
        let mut e = Engine::new();
        with_default_point(&mut e);
        copy_doubles(&mut e);
        let source = source_request(&mut e);
        let s = source.addr();
        let dest: Ptr<PathingRequestCoveredMove> = e.new_object();
        let d = dest.addr();
        e.call_log = Some(vec![]);
        e.call(0x006e4dd0, &args![source, dest]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, PATHING_REQUEST_COPY_TO), vec![vec![s, d]]);
        assert_eq!(calls(&log, SET_ATTACKER_RADIUS), vec![vec![d, 0x7fc0_0001]]);
        assert_eq!(
            calls(&log, SET_ATTACKER_SPEED),
            vec![vec![d, 2.0f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(d + 0xb8), 0x7fc0_0002);
        assert_eq!(
            calls(&log, SET_ATTACKER_SNEAK_HEIGHT),
            vec![vec![d, 4.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&log, SET_ATTACKER_HEALTH),
            vec![vec![d, 5.0f32.to_bits()]]
        );
        assert_eq!(calls(&log, SET_LOCATION_INDEX), vec![vec![d, 7]]);
        assert_eq!(calls(&log, SET_COVER_LOCATION), vec![vec![d, s + 0xc8]]);
        assert_eq!(e.mem.u32(d + 0xd8), 0x101);
        assert_eq!(e.mem.u8(d + 0xe0), 1);
        assert_eq!(calls(&log, SET_FINISHED), vec![vec![d, 1]]);
        assert_eq!(calls(&log, WORD_ARRAY_SET_SIZE), vec![vec![d + 0xe4, 0, 1]]);
        assert_eq!(
            calls(&log, POINT_ARRAY_COPY_TO),
            vec![vec![s + 0xf4, d + 0xf4]]
        );

        // Without a target location the destination's is reset.
        e.mem.set_u8(s + 0xe0, 0);
        e.call(0x006e4dd0, &args![source, dest]);
        assert_eq!(e.mem.f32(d + 0xd4), 1.5);
        assert_eq!(e.mem.u8(d + 0xe0), 0);
    }

    #[test]
    fn copy_constructor_builds_then_copies_from_source() {
        let mut e = Engine::new();
        with_default_point(&mut e);
        copy_doubles(&mut e);
        quiet_doubles(&mut e, &MEMBER_CONSTRUCTORS);
        quiet_doubles(&mut e, &[PATHING_REQUEST_COPY_CONSTRUCTOR]);
        let source = source_request(&mut e);
        let this: Ptr<PathingRequestCoveredMove> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e4d20, &args![this, source]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(
            calls(&log, PATHING_REQUEST_COPY_CONSTRUCTOR),
            vec![vec![this.addr(), source.addr()]]
        );
        assert_eq!(calls(&log, POINT_CONSTRUCTOR).len(), 2);
        assert_eq!(
            calls(&log, PATHING_REQUEST_COPY_TO),
            vec![vec![source.addr(), this.addr()]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0xdc), 0x102);
    }

    #[test]
    fn small_accessors() {
        let mut e = Engine::new();
        with_default_point(&mut e);
        let this: Ptr<PathingRequestCoveredMove> = e.new_object();
        let t = this.addr();
        e.call(0x006e4ef0, &args![this, 0x7f80_0003u32]);
        assert_eq!(e.mem.u32(t + 0xb8), 0x7fc0_0003);
        e.call(0x006e4ef0, &args![this, 6.25f32]);
        assert_eq!(e.mem.f32(t + 0xb8), 6.25);

        let point = e.mem.alloc(12);
        e.mem.set_f32(point, 9.0);
        e.mem.set_f32(point + 4, 8.0);
        e.mem.set_f32(point + 8, 7.0);
        e.call(0x006e4f10, &args![this, point]);
        assert_eq!(e.mem.f32(t + 0xd4), 9.0);
        assert_eq!(e.mem.f32(t + 0xdc), 7.0);
        assert_eq!(e.mem.u8(t + 0xe0), 1);
        e.call(0x006e4f50, &args![this]);
        assert_eq!(e.mem.f32(t + 0xd4), 1.5);
        assert_eq!(e.mem.f32(t + 0xd8), 2.5);
        assert_eq!(e.mem.f32(t + 0xdc), 3.5);
        assert_eq!(e.mem.u8(t + 0xe0), 0);

        assert_eq!(e.call(0x006e4f90, &args![this]).u32(), t + 0xe4);
        assert_eq!(e.call(0x006e4fb0, &args![this]).u32(), t + 0xf4);
    }

    #[test]
    fn save_game_writes_fields_and_arrays() {
        let mut e = Engine::new();
        quiet_doubles(
            &mut e,
            &[
                PATHING_REQUEST_SAVE_GAME,
                SAVE_DATA,
                SAVE_VARIABLE_SIZED_VALUE,
            ],
        );
        // The unreachable array has 2 elements, the target array 1.
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(UNREACHABLE_ELEMENT, |_, a| (0x9000 + 4 * a[1]).into_ret());
        e.register(TARGET_ELEMENT, |_, a| (0xa000 + 12 * a[1]).into_ret());
        let this: Ptr<PathingRequestCoveredMove> = e.new_object();
        let t = this.addr();
        e.mem.set_u32(t + 0xe4 + 8, 2);
        e.mem.set_u32(t + 0xf4 + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x006e4fd0, &args![this, 0x8000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PATHING_REQUEST_SAVE_GAME),
            vec![vec![t, 0x8000]]
        );
        assert_eq!(
            calls(&log, SAVE_DATA),
            vec![
                vec![0x8000, t + 0xb0, 4, 0],
                vec![0x8000, t + 0xb4, 4, 0],
                vec![0x8000, t + 0xb8, 4, 0],
                vec![0x8000, t + 0xbc, 4, 0],
                vec![0x8000, t + 0xc0, 4, 0],
                vec![0x8000, t + 0xc4, 4, 0],
                vec![0x8000, t + 0xc8, 12, 0],
                vec![0x8000, t + 0xd4, 12, 0],
                vec![0x8000, t + 0xe0, 1, 0],
                vec![0x8000, 0x9000, 4, 0],
                vec![0x8000, 0x9004, 4, 0],
                vec![0x8000, 0xa000, 12, 0],
            ]
        );
        assert_eq!(
            calls(&log, SAVE_VARIABLE_SIZED_VALUE),
            vec![vec![0x8000, 2], vec![0x8000, 1]]
        );
        assert_eq!(
            calls(&log, ARRAY_SIZE),
            vec![vec![t + 0xe4], vec![t + 0xf4]]
        );
    }

    #[test]
    fn load_game_reads_fields_and_sizes_arrays() {
        let mut e = Engine::new();
        quiet_doubles(
            &mut e,
            &[
                PATHING_REQUEST_LOAD_GAME,
                LOAD_DATA,
                WORD_ARRAY_SET_SIZE,
                POINT_ARRAY_SET_SIZE,
            ],
        );
        // Sizes 1, then 2.
        e.register_double(LOAD_VARIABLE_SIZED_VALUE, {
            let mut next = 0u32;
            move |_, _| {
                next += 1;
                next.into_ret()
            }
        });
        e.register(UNREACHABLE_ELEMENT, |_, a| (0x9000 + 4 * a[1]).into_ret());
        e.register(TARGET_ELEMENT, |_, a| (0xa000 + 12 * a[1]).into_ret());
        let this: Ptr<PathingRequestCoveredMove> = e.new_object();
        let t = this.addr();
        e.call_log = Some(vec![]);
        e.call(0x006e51a0, &args![this, 0x8000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, PATHING_REQUEST_LOAD_GAME),
            vec![vec![t, 0x8000]]
        );
        let loads = calls(&log, LOAD_DATA);
        assert_eq!(loads.len(), 9 + 1 + 2);
        assert_eq!(loads[6], vec![0x8000, t + 0xc8, 12]);
        assert_eq!(loads[8], vec![0x8000, t + 0xe0, 1]);
        assert_eq!(loads[9], vec![0x8000, 0x9000, 4]);
        assert_eq!(loads[11], vec![0x8000, 0xa00c, 12]);
        assert_eq!(calls(&log, WORD_ARRAY_SET_SIZE), vec![vec![t + 0xe4, 1, 1]]);
        assert_eq!(
            calls(&log, POINT_ARRAY_SET_SIZE),
            vec![vec![t + 0xf4, 2, 1]]
        );
    }

    #[test]
    fn word_array_copy_to_sizes_and_copies() {
        let mut e = Engine::new();
        quiet_doubles(&mut e, &[WORD_ARRAY_SET_SIZE]);
        let dest_buffer = e.mem.alloc(16);
        e.register_double(WORD_ARRAY_ELEMENT, move |_, a| {
            (dest_buffer + 4 * a[1]).into_ret()
        });
        let source: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(16);
        for i in 0..3 {
            e.mem.set_u32(buffer + 4 * i, 0x50 + i);
        }
        e.set(source, BSSimpleArray::pBuffer, buffer);
        e.set(source, BSSimpleArray::iSize, 3);
        let dest: Ptr<BSSimpleArray> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x006e5330, &args![source, dest]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, WORD_ARRAY_SET_SIZE),
            vec![vec![dest.addr(), 3, 1]]
        );
        assert_eq!(calls(&log, WORD_ARRAY_ELEMENT).len(), 3);
        for i in 0..3 {
            assert_eq!(e.mem.u32(dest_buffer + 4 * i), 0x50 + i);
        }
    }
}
