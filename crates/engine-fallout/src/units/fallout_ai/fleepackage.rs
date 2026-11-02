//! `fallout/ai/fleepackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `FleePackage` fields on PC (the Xbox PDB's offsets less 0x10:
//! `TESPackage` is 0x80 bytes on PC): `bEvaluatePoint` +0x80, `bCombatMode`
//! +0x81, `pFleePoint` (`NiPoint3`) +0x84, `fFleeTimer` +0x90,
//! `bFleeSucceeded` +0x94, `pAvoidRefList` (`BSSimpleList<TESObjectREFR*>`)
//! +0x98, `pTeleportDoor` +0xA0, `pRefFleeTo` +0xA4, `bknowstarget` +0xA8,
//! `busehorse` +0xA9.

#[allow(unused_imports)]
use crate::prelude::*;

const EVALUATE_POINT: u32 = 0x80;
const COMBAT_MODE: u32 = 0x81;
const FLEE_POINT: u32 = 0x84;
const FLEE_TIMER: u32 = 0x90;
const FLEE_SUCCEEDED: u32 = 0x94;
const AVOID_LIST: u32 = 0x98;
const TELEPORT_DOOR: u32 = 0xa0;
const REF_FLEE_TO: u32 = 0xa4;
const KNOWS_TARGET: u32 = 0xa8;
const USE_HORSE: u32 = 0xa9;

/// `FleePackage`'s vtable (stored by the constructor and the destructor).
const FLEE_PACKAGE_VTABLE: u32 = 0x0109_31e4;
/// The player character pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// The global `NiPoint3` the constructor copies into the flee point.
const ZERO_POINT: u32 = 0x011f_426c;
/// The teleport-door search's globals: the fleeing actor (`011f2f1c`) and
/// the door found (`011f2f10`).
const DOOR_SEARCH_ACTOR: u32 = 0x011f_2f1c;
const DOOR_FOUND: u32 = 0x011f_2f10;
/// The game's allocator (`00401000`) and release (`00401030`), cdecl.
const ALLOCATE: u32 = 0x0040_1000;
const DEALLOCATE: u32 = 0x0040_1030;
/// `BSSimpleList` steps: the node test `008256d0` (no item), the item
/// getter `006815c0` (the node's own address), the next node `00726070`,
/// the head removal `0063f7b0`, the value removal `00905330`
/// (thiscall(&value)), the membership test `005f65d0` (thiscall(&value),
/// bool), the append `005ae3d0` (thiscall(&value)), the item set
/// `00726c60` (thiscall(&value)) and the count `005ae380`.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_ITEM: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const LIST_REMOVE_VALUE: u32 = 0x0090_5330;
const LIST_CONTAINS: u32 = 0x005f_65d0;
const LIST_APPEND: u32 = 0x005a_e3d0;
const LIST_SET_ITEM: u32 = 0x0072_6c60;
const LIST_COUNT: u32 = 0x005a_e380;
/// `TESPackage`'s target (`00671d10`) and location (`0055b980`) getters;
/// `PackageTarget::SetTargReference` (`00680110`),
/// `PackageTarget::GetTargReference` (`00680020`) and
/// `PackageLocation::SetLocReference` (`0067f3c0`).
const PACKAGE_TARGET: u32 = 0x0067_1d10;
const PACKAGE_LOCATION: u32 = 0x0055_b980;
const SET_TARGET_REFERENCE: u32 = 0x0068_0110;
const GET_TARGET_REFERENCE: u32 = 0x0068_0020;
const SET_LOCATION_REFERENCE: u32 = 0x0067_f3c0;
/// `TESObjectREFR::GetRefPersists` (`005653d0`), `GetDistanceFromReference`
/// (`005723b0`, thiscall(other, 0, 0), ST0) and two state tests
/// (`00440da0`, `00440d80`; no Xbox PDB names).
const REF_PERSISTS: u32 = 0x0056_53d0;
const DISTANCE_TO: u32 = 0x0057_23b0;
const REF_TEST_00440DA0: u32 = 0x0044_0da0;
const REF_TEST_00440D80: u32 = 0x0044_0d80;
/// `__RTDynamicCast` and the RTTI type descriptors `.?AVTESForm@@`,
/// `.?AVTESObjectREFR@@` and `.?AVActor@@`; `004839c0` gives the form of a
/// saved id.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
const FORM_FOR_ID: u32 = 0x0048_39c0;
/// The save/load manager and its calls (see the dialogue package's).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const BUFFER_POSITION: u32 = 0x0082_5c00;
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
const SAVE_VERSION: u32 = 0x008d_f040;
const RAW_WRITE: u32 = 0x0085_79b0;
const RAW_READ: u32 = 0x0085_79e0;
const SAVED_FORM: u32 = 0x004f_d3e0;
const LOADED_FORM: u32 = 0x004f_d3c0;
const SAVE_LOGGING_SETTING: u32 = 0x011d_e4e8;
const BOOL_SETTING_VALUE: u32 = 0x0040_8d60;
const SAVE_DATA_OLD: u32 = 0x0048_4ce0;
const LOAD_DATA_OLD: u32 = 0x0048_4d00;
const SAVE_NUMERIC_ID: u32 = 0x0048_4d20;
const LOAD_NUMERIC_ID: u32 = 0x0048_4d40;
const PRINT: u32 = 0x005b_5e40;
const ERROR: u32 = 0x0040_fbe0;
/// `"D:\_Fallout3\...\FleePackage.cpp"`.
const SOURCE_FILE: u32 = 0x0109_3344;
/// `"KOLB"` as a little-endian word: the save-game block header.
const BLOCK_HEADER: u32 = 0x424c_4f4b;
/// The form id getter (`0084e3a0`).
const FORM_ID: u32 = 0x0084_e3a0;

fn field(this: Ptr, offset: u32) -> u32 {
    this.addr().wrapping_add(offset)
}

fn save_load_game(e: &Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME)
}

fn logging(e: &mut Engine) -> bool {
    let value = e
        .call(BOOL_SETTING_VALUE, &args![SAVE_LOGGING_SETTING])
        .u32();
    e.mem.u8(value) != 0
}

/// `__RTDynamicCast(004839c0(id), 0, TESForm, to, 0)`.
fn cast_loaded(e: &mut Engine, id: u32, to: u32) -> u32 {
    let form = e.call(FORM_FOR_ID, &args![id]).u32();
    e.call(RT_DYNAMIC_CAST, &args![form, 0u32, TYPE_TES_FORM, to, 0u32])
        .u32()
}

/// The form record of the save logs: its id, name (virtual +0x130 of the
/// form `004839c0` gives), version byte (+9) and flags (+5).
fn logged_form(e: &mut Engine, record: u32) -> (u32, u32, u32, u32) {
    let id = e.mem.u32(record);
    let form = e.call(FORM_FOR_ID, &args![id]).u32();
    let name = e.vcall(form, 0x130, &args![]).u32();
    let id = e.mem.u32(record);
    let version = u32::from(e.mem.u8(record.wrapping_add(9)));
    let flags = e.mem.u32(record.wrapping_add(5));
    (id, name, version, flags)
}

// Translated from 009f0e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f0e00 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `FleePackage::FleePackage` (Xbox PDB; thiscall(`reference`, `cell`,
/// `location`), `RET 0xC`): the `TESPackage` constructor, the vtable, the
/// flee point (`006815c0`) and avoid list (`0096a2d0`) built, package type
/// 0x16, `00826b40(1)` and `00826b90(1)`, `bEvaluatePoint` set. With a
/// reference: a location (0xC bytes) at the reference (no cell or location
/// given; `bEvaluatePoint` stays set) or at the location reference or cell
/// (`bEvaluatePoint` cleared), set and deleted; a target (0x10 bytes) set
/// and deleted; `00984f60(0x13)`, the target's type 0 and reference. Then
/// created, the flee point from `011f426c`, the timer -1, nothing found
/// yet, combat mode on. Returns `this`.
pub fn flee_package_flee_package(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    cell: u32,
    location: u32,
) -> Ptr {
    e.call(0x0067_07c0, &args![this]);
    e.mem.set_u32(this.addr(), FLEE_PACKAGE_VTABLE);
    e.call(LIST_ITEM, &args![field(this, FLEE_POINT)]);
    e.call(0x0096_a2d0, &args![field(this, AVOID_LIST)]);
    e.call(0x0067_0fc0, &args![this, 0x16u32]);
    e.call(0x0082_6b40, &args![this, 1u32]);
    e.call(0x0082_6b90, &args![this, 1u32]);
    e.mem.set_u8(field(this, EVALUATE_POINT), 1);
    if reference != 0 {
        let block = e.call(ALLOCATE, &args![0xcu32]).u32();
        let place = if block != 0 {
            e.call(0x0067_f030, &args![block]).u32()
        } else {
            0
        };
        if cell == 0 && location == 0 {
            e.call(SET_LOCATION_REFERENCE, &args![place, reference]);
            e.mem.set_u8(field(this, EVALUATE_POINT), 1);
        } else {
            if location != 0 {
                e.call(SET_LOCATION_REFERENCE, &args![place, location]);
            } else if cell != 0 {
                e.call(0x0067_f410, &args![place, cell]);
            }
            e.mem.set_u8(field(this, EVALUATE_POINT), 0);
        }
        e.call(0x0067_1d30, &args![this, place]);
        if place != 0 {
            e.call(0x0067_0b30, &args![place, 1u32]);
        }
        let block = e.call(ALLOCATE, &args![0x10u32]).u32();
        let target = if block != 0 {
            e.call(0x0067_ff70, &args![block]).u32()
        } else {
            0
        };
        e.call(0x0067_2fc0, &args![this, target]);
        if target != 0 {
            e.call(0x007b_3fa0, &args![target, 1u32]);
        }
        e.call(0x0098_4f60, &args![this, 0x13u32]);
        let target = e.call(PACKAGE_TARGET, &args![this]).u32();
        e.call(0x0068_00b0, &args![target, 0u32]);
        let target = e.call(PACKAGE_TARGET, &args![this]).u32();
        e.call(SET_TARGET_REFERENCE, &args![target, reference]);
    }
    e.call(0x0067_4d70, &args![this, 1u32]);
    for word in 0..3 {
        let value = e.mem.u32(ZERO_POINT + 4 * word);
        e.mem.set_u32(field(this, FLEE_POINT) + 4 * word, value);
    }
    let timer = x87_quiet_bits(e.global::<u32>(0x0101_2054));
    e.mem.set_u32(field(this, FLEE_TIMER), timer);
    e.mem.set_u8(field(this, FLEE_SUCCEEDED), 0);
    e.mem.set_u32(field(this, TELEPORT_DOOR), 0);
    e.mem.set_u32(field(this, REF_FLEE_TO), 0);
    e.mem.set_u8(field(this, KNOWS_TARGET), 0);
    e.mem.set_u8(field(this, USE_HORSE), 0);
    e.mem.set_u8(field(this, COMBAT_MODE), 1);
    this
}

// Translated from 009f1090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `FleePackage` scalar deleting destructor (thiscall(`flags`), `RET
/// 4`): [`fn_009f10c0`], then `operator delete(this)` with bit 0 of
/// `flags`. Returns `this`.
pub fn fn_009f1090(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009f10c0(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 009f10c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f10c0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `FleePackage` destructor (thiscall): the vtable, the avoid list
/// cleared (`00470470`) and destroyed (`0046ffb0`), then the `TESPackage`
/// destructor (`006708b0`).
pub fn fn_009f10c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FLEE_PACKAGE_VTABLE);
    e.call(0x0047_0470, &args![field(this, AVOID_LIST)]);
    e.call(0x0046_ffb0, &args![field(this, AVOID_LIST)]);
    e.call(0x0067_08b0, &args![this]);
}

// Translated from 009f1140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage::FindTeleportDoor` (Xbox PDB; thiscall(`reference`,
/// `actor`), `RET 8`, bool): false without `bEvaluatePoint` or an actor.
/// With the actor in `011f2f1c`, a persistent reference (`GetRefPersists`)
/// has the data handler (`011c3f2c`) enumerate the references close to it
/// (`TESDataHandler::EnumReferencesCloseToPoint(cell, point, radius,
/// point, radius, 009f1600, reference)`, the radius the `float` at
/// `0104fabc`, the points its virtual +0x1F4). The door found
/// (`011f2f10`) becomes `pTeleportDoor`; both globals are cleared. Answers
/// whether there is a door.
pub fn flee_package_find_teleport_door(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    actor: u32,
) -> bool {
    if e.mem.u8(field(this, EVALUATE_POINT)) == 0 {
        return false;
    }
    if actor == 0 {
        return false;
    }
    e.set_global(DOOR_SEARCH_ACTOR, actor);
    if e.call(REF_PERSISTS, &args![reference]).bool() {
        let radius = x87_quiet_bits(e.global::<u32>(0x0104_fabc));
        let near = e.vcall(reference, 0x1f4, &args![]).u32();
        let radius_near = x87_quiet_bits(e.global::<u32>(0x0104_fabc));
        let point = e.vcall(reference, 0x1f4, &args![]).u32();
        let cell = e.call(0x008d_6f30, &args![reference]).u32();
        let handler = e.global::<u32>(0x011c_3f2c);
        e.call(
            0x0046_f280,
            &args![
                handler,
                cell,
                point,
                radius_near,
                near,
                radius,
                0x009f_1600u32,
                reference
            ],
        );
    }
    let door = e.global::<u32>(DOOR_FOUND);
    e.mem.set_u32(field(this, TELEPORT_DOOR), door);
    e.set_global(DOOR_SEARCH_ACTOR, 0u32);
    e.set_global(DOOR_FOUND, 0u32);
    e.mem.u32(field(this, TELEPORT_DOOR)) != 0
}

// Translated from 009f1210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage::GetClosestAvoidedRef` (Xbox PDB; thiscall(`reference`),
/// `RET 4`): 0 without `bEvaluatePoint` or for a reference that does not
/// persist. Otherwise the avoided reference closest to `reference`
/// (`GetDistanceFromReference`, starting from `FLT_MAX`; only an ordered
/// "closer" wins); one that is gone (`00440da0`, or virtual +0x22C(0)) is
/// removed ([`fn_009f1350`]) and the walk starts over.
pub fn flee_package_get_closest_avoided_ref(e: &mut Engine, this: Ptr, reference: u32) -> u32 {
    if e.mem.u8(field(this, EVALUATE_POINT)) == 0 || !e.call(REF_PERSISTS, &args![reference]).bool()
    {
        return 0;
    }
    let mut node = field(this, AVOID_LIST);
    let mut best = x87_quiet_bits(e.global::<u32>(0x0101_6970));
    let mut closest = 0;
    while node != 0 {
        let item = e.call(LIST_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(LIST_ITEM, &args![node]).u32();
        let candidate = e.mem.u32(item);
        node = e.call(LIST_NEXT, &args![node]).u32();
        let distance = x87_store32(
            e.call(DISTANCE_TO, &args![candidate, reference, 0u32, 0u32])
                .f64(),
        );
        let gone = e.call(REF_TEST_00440DA0, &args![candidate]).bool()
            || e.vcall(candidate, 0x22c, &args![0u32]).bool();
        if gone {
            fn_009f1350(e, this, candidate);
            node = field(this, AVOID_LIST);
        } else if x87_load32(distance) < x87_load32(best) {
            // `FCOMPP`/`TEST AH,5`/`JP`: only an ordered "below" wins.
            best = x87_quiet_bits(distance);
            closest = candidate;
        }
    }
    closest
}

// Translated from 009f1310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage::AddAvoidedRef` (Xbox PDB; thiscall(`reference`), `RET 4`):
/// appends the reference to the avoid list (`005ae3d0`) unless it is there
/// (`005f65d0`); both get the address of the stack argument.
pub fn flee_package_add_avoided_ref(e: &mut Engine, this: Ptr, reference: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), reference);
        if !e
            .call(LIST_CONTAINS, &args![field(this, AVOID_LIST), slot])
            .bool()
        {
            e.call(LIST_APPEND, &args![field(this, AVOID_LIST), slot]);
        }
    });
}

// Translated from 009f1350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drops an avoided reference (thiscall(`reference`), `RET 4`; no Xbox PDB
/// name): removed from the avoid list (`00905330`); when it was the
/// package target's reference, the target and location move to the next
/// avoided reference (the list head, `00680110`, `0067f3c0` on
/// `0055b980`) or to none; `pRefFleeTo` is cleared when it was it.
pub fn fn_009f1350(e: &mut Engine, this: Ptr, reference: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), reference);
        e.call(LIST_REMOVE_VALUE, &args![field(this, AVOID_LIST), slot]);
    });
    let mut current = 0;
    if e.call(PACKAGE_TARGET, &args![this]).u32() != 0 {
        let target = e.call(PACKAGE_TARGET, &args![this]).u32();
        current = e.call(GET_TARGET_REFERENCE, &args![target]).u32();
    }
    if current != 0 && current == reference {
        if !e
            .call(LIST_IS_EMPTY, &args![field(this, AVOID_LIST)])
            .bool()
        {
            let item = e.call(LIST_ITEM, &args![field(this, AVOID_LIST)]).u32();
            let next = e.mem.u32(item);
            let target = e.call(PACKAGE_TARGET, &args![this]).u32();
            e.call(SET_TARGET_REFERENCE, &args![target, next]);
            let item = e.call(LIST_ITEM, &args![field(this, AVOID_LIST)]).u32();
            let next = e.mem.u32(item);
            let place = e.call(PACKAGE_LOCATION, &args![this]).u32();
            e.call(SET_LOCATION_REFERENCE, &args![place, next]);
        } else {
            let target = e.call(PACKAGE_TARGET, &args![this]).u32();
            e.call(SET_TARGET_REFERENCE, &args![target, 0u32]);
            let place = e.call(PACKAGE_LOCATION, &args![this]).u32();
            e.call(SET_LOCATION_REFERENCE, &args![place, 0u32]);
        }
    }
    if reference == e.mem.u32(field(this, REF_FLEE_TO)) {
        e.mem.set_u32(field(this, REF_FLEE_TO), 0);
    }
}

// Translated from 009f1440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f1440 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// Builds the array of the spheres to avoid (thiscall; no Xbox PDB name):
/// a 0x18-byte array (`00aa13e0`, `006e3850`; null when the allocation
/// failed); the radius is the package target's integer (`0044ddc0`, as a
/// `float`) or the `float` at `010231a0`, through `00404010` with the
/// `float` at `01032adc`. For each avoided reference still there (not
/// `00440da0`, not virtual +0x22C(0)) the array grows by one
/// (`006e3ad0(count + 1, 1)`) and its new 0x24-byte slot (`006a1400`) gets
/// the sphere `006915d0(point, radius, 00404010(radius * radius, 100))` at
/// the reference's virtual +0x1F4. Returns the array.
pub fn fn_009f1440(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(0x00aa_13e0, &args![0x18u32]).u32();
    let array = if block != 0 {
        e.call(0x006e_3850, &args![block]).u32()
    } else {
        0
    };
    let base_radius = if e.call(PACKAGE_TARGET, &args![this]).u32() != 0 {
        let target = e.call(PACKAGE_TARGET, &args![this]).u32();
        let value = e.call(0x0044_ddc0, &args![target]).i32();
        x87_store32(f64::from(value))
    } else {
        x87_quiet_bits(e.global::<u32>(0x0102_31a0))
    };
    let base_radius = x87_quiet_bits(base_radius);
    let floor = x87_quiet_bits(e.global::<u32>(0x0103_2adc));
    let radius = x87_store32(
        e.call(0x0040_4010, &args![x87_quiet_bits(base_radius), floor])
            .f64(),
    );
    e.with_stack(0x58, |e, frame| {
        let ebp = frame.addr() + 0x58;
        let mut node = field(this, AVOID_LIST);
        while node != 0 {
            let item = e.call(LIST_ITEM, &args![node]).u32();
            let reference = e.mem.u32(item);
            let skip = reference == 0
                || e.call(REF_TEST_00440DA0, &args![reference]).bool()
                || e.vcall(reference, 0x22c, &args![0u32]).bool();
            if !skip {
                let count = e.call(0x0044_ddc0, &args![array]).u32();
                e.call(0x006e_3ad0, &args![array, count.wrapping_add(1), 1u32]);
                let hundred = x87_quiet_bits(e.global::<u32>(0x0101_6410));
                let square = x87_store32(x87_mul(x87_load32(radius), x87_m32(radius)));
                let height = x87_store32(
                    e.call(0x0040_4010, &args![x87_quiet_bits(square), hundred])
                        .f64(),
                );
                let point = e.vcall(reference, 0x1f4, &args![]).u32();
                let sphere = e
                    .call(
                        0x0069_15d0,
                        &args![
                            ebp - 0x50,
                            point,
                            x87_quiet_bits(radius),
                            x87_quiet_bits(height)
                        ],
                    )
                    .u32();
                let slot = e.call(0x006a_1400, &args![array, count]).u32();
                // `MOVSD.REP` with ECX = 9: the 0x24-byte sphere.
                for word in 0..9 {
                    let value = e.mem.u32(sphere.wrapping_add(4 * word));
                    e.mem.set_u32(slot.wrapping_add(4 * word), value);
                }
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
    });
    array
}

// Translated from 009f1600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f1600 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The teleport-door search's callback (cdecl(`reference`, `actor`), a
/// byte; no Xbox PDB name): false for no reference, one that fails
/// `00440d80`, `004077c0` or passes `00440da0`, no actor, a reference
/// whose base (`007af430`) is the form at `011ca258`, or one without a
/// linked door (`00568e50`). A door within the radius (the `float` at
/// `0101e704` for an interior searcher `011f2f1c` (`GetInterior`), else
/// `01022454`) of the searcher, or closer to the actor than the searcher
/// is, qualifies unless it is locked (`GetLock`, `005021a0`) for a
/// non-owner (`IsAnOwner(actor, 1)`) without its key (`Actor::HasObjects`);
/// then it is found (`011f2f10`) when a path leads from the actor to its
/// teleport position (`006dcd70`, `006dce10`, `006d62c0`; the pathing
/// locations destroyed by `004ff7e0`).
pub fn fn_009f1600(e: &mut Engine, reference: u32, actor: u32) -> bool {
    if reference == 0
        || e.call(REF_TEST_00440D80, &args![reference]).bool()
        || e.call(0x0040_77c0, &args![reference]).bool()
        || e.call(REF_TEST_00440DA0, &args![reference]).bool()
    {
        return false;
    }
    if actor == 0 {
        return false;
    }
    if e.call(0x007a_f430, &args![reference]).u32() == e.global::<u32>(0x011c_a258) {
        return false;
    }
    if e.call(0x0056_8e50, &args![reference]).u32() == 0 {
        return false;
    }
    let searcher = e.global::<u32>(DOOR_SEARCH_ACTOR);
    let radius = if searcher != 0 && e.call(0x0057_5d10, &args![searcher]).bool() {
        x87_quiet_bits(e.global::<u32>(0x0101_e704))
    } else {
        x87_quiet_bits(e.global::<u32>(0x0102_2454))
    };
    let radius = x87_quiet_bits(radius);
    let to_actor = x87_store32(
        e.call(DISTANCE_TO, &args![reference, actor, 0u32, 0u32])
            .f64(),
    );
    let searcher = e.global::<u32>(DOOR_SEARCH_ACTOR);
    let to_searcher = x87_store32(
        e.call(DISTANCE_TO, &args![reference, searcher, 0u32, 0u32])
            .f64(),
    );
    // Two `FCOMPP`/`TEST AH,5` tests: the radius below the searcher's
    // distance (`JNP`), else the actor's distance below it (`JP` out).
    let near = x87_load32(radius) < x87_load32(to_searcher)
        || x87_load32(to_actor) < x87_load32(to_searcher);
    if !near {
        return false;
    }
    e.with_stack(0x80, |e, frame| {
        let ebp = frame.addr() + 0x80;
        let lock = e.call(0x0056_9160, &args![reference]).u32();
        if lock != 0
            && e.call(0x0050_21a0, &args![lock]).bool()
            && !e.call(0x0057_85e0, &args![reference, actor, 1u32]).bool()
        {
            let key = e.mem.u32(lock.wrapping_add(4));
            let count = ebp - 0x20;
            e.mem.set_u32(count, 0);
            if key == 0
                || !e
                    .call(0x0089_1db0, &args![actor, key, 0u32, 1u32, 0u32, count])
                    .bool()
            {
                return false;
            }
        }
        let from = ebp - 0x48;
        let to = ebp - 0x74;
        e.call(0x006d_cd70, &args![from, actor]);
        let position = e.call(0x0056_8fa0, &args![reference]).u32();
        e.call(0x006d_ce10, &args![to, position, reference]);
        let found = e.call(0x006d_62c0, &args![from, to, 0u32]).bool();
        if found {
            e.set_global(DOOR_FOUND, reference);
        }
        e.call(0x004f_f7e0, &args![to]);
        e.call(0x004f_f7e0, &args![from]);
        found
    })
}

// Translated from 009f1830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Prunes the avoid list (thiscall(`combat`), `RET 4`; no Xbox PDB name):
/// a reference that is gone (virtual +0x22C(0), `00440d80`) is dropped
/// (`0063f7b0`, the walk restarting); in combat mode an actor reference
/// (virtual +0x100) that `00493bb0` accepts or is the player is dropped
/// unless `combat` (virtual +0x20(actor, 0)) last saw it within the setting
/// at `011cd348` of now (`00435dd0`); other actors are dropped.
pub fn fn_009f1830(e: &mut Engine, this: Ptr, combat: u32) {
    let mut node = field(this, AVOID_LIST);
    let setting = e.call(0x0040_3e20, &args![0x011c_d348u32]).u32();
    let limit = x87_quiet_bits(e.mem.u32(setting));
    while node != 0 {
        let item = e.call(LIST_ITEM, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(LIST_ITEM, &args![node]).u32();
        let reference = e.mem.u32(item);
        let actor = if e.vcall(reference, 0x100, &args![]).bool() {
            reference
        } else {
            0
        };
        let mut drop = e.vcall(reference, 0x22c, &args![0u32]).bool()
            || e.call(REF_TEST_00440D80, &args![reference]).bool();
        if !drop && actor != 0 && e.mem.u8(field(this, COMBAT_MODE)) != 0 {
            if e.call(0x0049_3bb0, &args![actor]).bool() || actor == e.global::<u32>(PLAYER) {
                drop = if combat == 0 {
                    false
                } else {
                    let now = e.call(0x0043_5dd0, &[]).f64();
                    let seen = e.vcall(combat, 0x20, &args![actor, 0u32]).f64();
                    let since = x87_sub(now, seen);
                    // `FLD` the limit, `FCOMPP`, `TEST AH,5`/`JNP`: an
                    // ordered "limit below the time since" keeps it dropped.
                    x87_load32(limit) < since
                };
            } else {
                drop = true;
            }
        }
        if drop {
            e.call(LIST_REMOVE_HEAD, &args![node]);
            node = field(this, AVOID_LIST);
        } else {
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
    }
}

// Translated from 009f1990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage::ShouldShutDown` (Xbox PDB): true when the only avoided
/// reference is the player and the player is not in combat
/// (`PlayerCharacter::IsPlayerCharacterInCombat(&flag)`).
pub fn flee_package_should_shut_down(e: &mut Engine, this: Ptr) -> bool {
    if e.call(LIST_COUNT, &args![field(this, AVOID_LIST)]).u32() != 1 {
        return false;
    }
    e.with_stack(4, |e, flag| {
        e.mem.set_u8(flag.addr(), 0);
        let item = e.call(LIST_ITEM, &args![field(this, AVOID_LIST)]).u32();
        let only = e.mem.u32(item);
        let player = e.global::<u32>(PLAYER);
        if only != player {
            return false;
        }
        let player = e.global::<u32>(PLAYER);
        !e.call(0x0095_3c50, &args![player, flag]).bool()
    })
}

// Translated from 009f19f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage`'s old-format save size (thiscall, a 16-bit size; no Xbox
/// PDB name): the `TESPackage` part (`00678fd0`), 4 + 2 more with
/// save-game blocks, 4 per avoided reference + 2, then 4 + 4 + 1 + 1 + 1 +
/// 4 + 0xC + 1 + 1. With save-size logging the difference is printed
/// (`0040fbe0`). Every step is a 16-bit addition.
pub fn fn_009f19f0(e: &mut Engine, this: Ptr) -> u16 {
    let mut size = 0u16;
    size = size.wrapping_add(e.call(0x0067_8fd0, &args![this]).u16());
    let start = size;
    let manager = save_load_game(e);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    let count = e.call(LIST_COUNT, &args![field(this, AVOID_LIST)]).u32();
    size = u32::from(size)
        .wrapping_add(count.wrapping_mul(4))
        .wrapping_add(2) as u16;
    for step in [4u16, 4, 1, 1, 1, 4, 0xc, 1, 1] {
        size = size.wrapping_add(step);
    }
    if logging(e) {
        let difference = (i32::from(size) - i32::from(start)) as u32;
        let manager = save_load_game(e);
        let record = e.call(SAVED_FORM, &args![manager]).u32();
        if record != 0 {
            let (id, name, _, flags) = logged_form(e, record);
            e.call(
                ERROR,
                &args![
                    0x0101_2cb0u32,
                    difference,
                    id,
                    name,
                    flags,
                    0x237u32,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR,
                &args![0x0101_2c78u32, difference, 0x237u32, SOURCE_FILE],
            );
        }
    }
    size
}

// Translated from 009f1b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage`'s old-format save (thiscall; no Xbox PDB name): the
/// `TESPackage` part (`00679120`); with save-game blocks the `"KOLB"`
/// header and a 16-bit length placeholder; a 16-bit count placeholder
/// (filled after the loop) and each avoided reference's numeric id; the
/// ids of `pRefFleeTo` and `pTeleportDoor` (0 for none); `bCombatMode`,
/// `bknowstarget`, `bFleeSucceeded`, the timer, the flee point,
/// `bEvaluatePoint` and `busehorse`. With save-size logging the size is
/// printed; with blocks the length is written (a warning past 0xFFFF).
pub fn fn_009f1b70(e: &mut Engine, this: Ptr) {
    e.call(0x0067_9120, &args![this]);
    e.with_stack(0x48, |e, frame| {
        let ebp = frame.addr() + 0x48;
        let length = ebp - 0xc;
        e.mem.set_u16(length, 0);
        let mut block = 0u32;
        let manager = save_load_game(e);
        let mut start = e.call(BUFFER_POSITION, &args![manager]).u32();
        if logging(e) {
            let manager = save_load_game(e);
            start = e.call(BUFFER_POSITION, &args![manager]).u32();
        }
        let manager = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
            let header = ebp - 0x24;
            e.mem.set_u32(header, BLOCK_HEADER);
            let manager = save_load_game(e);
            e.call(RAW_WRITE, &args![manager, header, 4u32]);
            let manager = save_load_game(e);
            block = e.call(BUFFER_POSITION, &args![manager]).u32();
            let manager = save_load_game(e);
            e.call(RAW_WRITE, &args![manager, length, 2u32]);
        }
        let count = ebp - 0x20;
        e.mem.set_u16(count, 0);
        let manager = save_load_game(e);
        let count_at = e.call(BUFFER_POSITION, &args![manager]).u32();
        let manager = save_load_game(e);
        e.call(RAW_WRITE, &args![manager, count, 2u32]);
        let mut node = field(this, AVOID_LIST);
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let item = e.call(LIST_ITEM, &args![node]).u32();
            let reference = e.mem.u32(item);
            let id = e.call(FORM_ID, &args![reference]).u32();
            let slot = ebp - 0x28;
            e.mem.set_u32(slot, id);
            e.call(SAVE_NUMERIC_ID, &args![this, slot, 4u32]);
            let counted = e.mem.u16(count).wrapping_add(1);
            e.mem.set_u16(count, counted);
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        let counted = e.mem.u16(count);
        e.mem.set_u16(count_at, counted);
        for (offset, slot) in [(REF_FLEE_TO, ebp - 0x14), (TELEPORT_DOOR, ebp - 0x18)] {
            e.mem.set_u32(slot, 0);
            let form = e.mem.u32(field(this, offset));
            if form != 0 {
                let id = e.call(FORM_ID, &args![form]).u32();
                e.mem.set_u32(slot, id);
            }
            e.call(SAVE_NUMERIC_ID, &args![this, slot, 4u32]);
        }
        for (offset, size) in [
            (COMBAT_MODE, 1u32),
            (KNOWS_TARGET, 1),
            (FLEE_SUCCEEDED, 1),
            (FLEE_TIMER, 4),
            (FLEE_POINT, 0xc),
            (EVALUATE_POINT, 1),
            (USE_HORSE, 1),
        ] {
            e.call(SAVE_DATA_OLD, &args![this, field(this, offset), size]);
        }
        if logging(e) {
            let manager = save_load_game(e);
            let now = e.call(BUFFER_POSITION, &args![manager]).u32();
            let manager = save_load_game(e);
            let record = e.call(SAVED_FORM, &args![manager]).u32();
            let difference = now.wrapping_sub(start);
            if record != 0 {
                let (id, name, _, flags) = logged_form(e, record);
                e.call(
                    ERROR,
                    &args![
                        0x0101_53a0u32,
                        difference,
                        id,
                        name,
                        flags,
                        0x26au32,
                        SOURCE_FILE
                    ],
                );
            } else {
                e.call(
                    ERROR,
                    &args![0x0101_536cu32, difference, 0x26au32, SOURCE_FILE],
                );
            }
        }
        let manager = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
            let manager = save_load_game(e);
            let now = e.call(BUFFER_POSITION, &args![manager]).u32();
            if now > block.wrapping_add(0xffff) {
                e.call(PRINT, &args![0x0101_5318u32, SOURCE_FILE, 0x26au32]);
            }
            e.mem.set_u16(block, now.wrapping_sub(block) as u16);
        }
    });
}

/// The buffer check at the end of the old-format load (line `0x29a`).
fn check_load_block(e: &mut Engine, start: u32, length: u16) {
    let manager = save_load_game(e);
    let now = e.call(BUFFER_POSITION, &args![manager]).u32();
    let manager = save_load_game(e);
    let record = e.call(LOADED_FORM, &args![manager]).u32();
    let end = u32::from(length).wrapping_add(start);
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(FORM_FOR_ID, &args![id]).u32();
        let (format, amount) = if now > end {
            (0x0101_5588u32, now.wrapping_sub(end))
        } else if now < end {
            (0x0101_5500, end.wrapping_sub(now))
        } else {
            return;
        };
        let flags = e.mem.u32(record.wrapping_add(5));
        let version = u32::from(e.mem.u8(record.wrapping_add(9)));
        let name = e.vcall(form, 0x130, &args![]).u32();
        let id = e.mem.u32(record);
        e.call(
            PRINT,
            &args![
                format,
                amount,
                SOURCE_FILE,
                0x29au32,
                id,
                name,
                version,
                flags
            ],
        );
    } else {
        let (format, amount) = if now > end {
            (0x0101_54a0u32, now.wrapping_sub(end))
        } else if now < end {
            (0x0101_5440, end.wrapping_sub(now))
        } else {
            return;
        };
        let manager = save_load_game(e);
        let version = u32::from(e.call(SAVE_VERSION, &args![manager]).u8());
        e.call(
            PRINT,
            &args![format, amount, SOURCE_FILE, 0x29au32, version],
        );
    }
}

// Translated from 009f1e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage::LoadGame` (Xbox PDB; the old format; thiscall): the
/// `TESPackage` part (`00679340`); with save-game blocks the header (a
/// wrong one printed) and the block length; the avoided references' ids
/// (a 16-bit count, each appended as read), the `pRefFleeTo` and
/// `pTeleportDoor` ids, the fields [`fn_009f1b70`] writes (`busehorse`
/// from version 0x71); with blocks the buffer position is checked.
pub fn flee_package_load_game(e: &mut Engine, this: Ptr) {
    e.call(0x0067_9340, &args![this]);
    e.with_stack(0x40, |e, frame| {
        let ebp = frame.addr() + 0x40;
        let length = ebp - 8;
        e.mem.set_u16(length, 0);
        let mut start = 0u32;
        let manager = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
            let header = ebp - 0x1c;
            let manager = save_load_game(e);
            e.call(RAW_READ, &args![manager, header, 4u32]);
            if e.mem.u32(header) != BLOCK_HEADER {
                let manager = save_load_game(e);
                let record = e.call(LOADED_FORM, &args![manager]).u32();
                if record != 0 {
                    let (_, name, version, flags) = logged_form(e, record);
                    let id = e.mem.u32(record);
                    e.call(
                        PRINT,
                        &args![
                            0x0101_5718u32,
                            SOURCE_FILE,
                            0x272u32,
                            id,
                            name,
                            version,
                            flags
                        ],
                    );
                } else {
                    let manager = save_load_game(e);
                    let version = u32::from(e.call(SAVE_VERSION, &args![manager]).u8());
                    e.call(
                        PRINT,
                        &args![0x0101_56a8u32, SOURCE_FILE, 0x272u32, version],
                    );
                }
            }
            let manager = save_load_game(e);
            start = e.call(BUFFER_POSITION, &args![manager]).u32();
            let manager = save_load_game(e);
            e.call(RAW_READ, &args![manager, length, 2u32]);
        }
        let count = ebp - 0x18;
        e.call(LOAD_DATA_OLD, &args![this, count, 2u32]);
        let mut index = 0i32;
        while index < i32::from(e.mem.u16(count)) {
            let read = ebp - 0x2c;
            e.call(LOAD_NUMERIC_ID, &args![this, read, 4u32]);
            let id = e.mem.u32(read);
            let slot = ebp - 0x3c;
            e.mem.set_u32(slot, id);
            e.call(LIST_APPEND, &args![field(this, AVOID_LIST), slot]);
            index += 1;
        }
        for (slot, offset) in [(ebp - 0x10, REF_FLEE_TO), (ebp - 0x14, TELEPORT_DOOR)] {
            e.call(LOAD_NUMERIC_ID, &args![this, slot, 4u32]);
            let id = e.mem.u32(slot);
            e.mem.set_u32(field(this, offset), id);
        }
        for (offset, size) in [
            (COMBAT_MODE, 1u32),
            (KNOWS_TARGET, 1),
            (FLEE_SUCCEEDED, 1),
            (FLEE_TIMER, 4),
            (FLEE_POINT, 0xc),
            (EVALUATE_POINT, 1),
        ] {
            e.call(LOAD_DATA_OLD, &args![this, field(this, offset), size]);
        }
        let manager = save_load_game(e);
        if e.call(SAVE_VERSION, &args![manager]).u8() >= 0x71 {
            e.call(LOAD_DATA_OLD, &args![this, field(this, USE_HORSE), 1u32]);
        }
        let manager = save_load_game(e);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![manager]).bool() {
            let length = e.mem.u16(length);
            check_load_block(e, start, length);
        }
    });
}

// Translated from 009f2220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage`'s load fix-up (thiscall; no Xbox PDB name): the
/// `TESPackage` part (`006796c0`); each avoided id is resolved to a
/// reference (`004839c0`, `TESObjectREFR` cast) and stored (`00726c60`),
/// or, unresolved, removed (the head by `0063f7b0`, a later one through
/// the previous node's `00905330`); `pRefFleeTo` is cast to `Actor` and
/// `pTeleportDoor` to `TESObjectREFR` when set.
pub fn fn_009f2220(e: &mut Engine, this: Ptr) {
    e.call(0x0067_96c0, &args![this]);
    e.with_stack(0x20, |e, frame| {
        let ebp = frame.addr() + 0x20;
        let mut node = field(this, AVOID_LIST);
        let mut previous = 0u32;
        while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            let item = e.call(LIST_ITEM, &args![node]).u32();
            let id = e.mem.u32(item);
            let resolved = ebp - 0x14;
            e.mem.set_u32(resolved, id);
            if id != 0 {
                let reference = cast_loaded(e, id, TYPE_TES_OBJECT_REFR);
                e.mem.set_u32(resolved, reference);
            }
            if e.mem.u32(resolved) == 0 {
                if previous == 0 {
                    e.call(LIST_REMOVE_HEAD, &args![node]);
                } else {
                    let value = ebp - 0x1c;
                    e.mem.set_u32(value, id);
                    e.call(LIST_REMOVE_VALUE, &args![previous, value]);
                    node = e.call(LIST_NEXT, &args![previous]).u32();
                }
            } else {
                e.call(LIST_SET_ITEM, &args![node, resolved]);
                previous = node;
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
        }
    });
    let flee_to = e.mem.u32(field(this, REF_FLEE_TO));
    if flee_to != 0 {
        let actor = cast_loaded(e, flee_to, TYPE_ACTOR);
        e.mem.set_u32(field(this, REF_FLEE_TO), actor);
    }
    let door = e.mem.u32(field(this, TELEPORT_DOOR));
    if door != 0 {
        let reference = cast_loaded(e, door, TYPE_TES_OBJECT_REFR);
        e.mem.set_u32(field(this, TELEPORT_DOOR), reference);
    }
}

// Translated from 009f2380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage::SaveGame` (Xbox PDB; thiscall(`buffer`), `RET 4`): the
/// `TESPackage` part (`006797c0`), `bEvaluatePoint`, `bCombatMode`, the
/// flee point, the timer, `bFleeSucceeded`, `bknowstarget`, `busehorse`
/// (`00865e50`), the door and flee-to form ids (`00865df0`), and the
/// avoided references' ids as a counted block (`00865f20`, `00865ff0`).
pub fn flee_package_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0067_97c0, &args![this, buffer]);
    for (offset, size) in [
        (EVALUATE_POINT, 1u32),
        (COMBAT_MODE, 1),
        (FLEE_POINT, 0xc),
        (FLEE_TIMER, 4),
        (FLEE_SUCCEEDED, 1),
        (KNOWS_TARGET, 1),
        (USE_HORSE, 1),
    ] {
        e.call(0x0086_5e50, &args![buffer, field(this, offset), size, 0u32]);
    }
    for offset in [TELEPORT_DOOR, REF_FLEE_TO] {
        let form = e.mem.u32(field(this, offset));
        e.call(0x0086_5df0, &args![buffer, form, 0u32]);
    }
    let mut count = 0u32;
    let block = e.call(0x0086_5f20, &args![buffer]).u32();
    let mut node = field(this, AVOID_LIST);
    while node != 0 {
        let item = e.call(LIST_ITEM, &args![node]).u32();
        let reference = e.mem.u32(item);
        if reference != 0 {
            e.call(0x0086_5df0, &args![buffer, reference, 0u32]);
            count = count.wrapping_add(1);
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    e.call(0x0086_5ff0, &args![buffer, count, block]);
}

// Translated from 009f24d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FleePackage::LoadGame_ov2` (Xbox PDB; thiscall(`buffer`), `RET 4`):
/// what [`flee_package_save_game`] writes, in its order: the `TESPackage`
/// part (`006798c0`), the fields (`00864980`), the door and flee-to ids
/// (`008648e0`), and the avoided ids (`00864a60` of them, each read into a
/// cleared stack word and appended).
pub fn flee_package_load_game_ov2(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(0x0067_98c0, &args![this, buffer]);
    for (offset, size) in [
        (EVALUATE_POINT, 1u32),
        (COMBAT_MODE, 1),
        (FLEE_POINT, 0xc),
        (FLEE_TIMER, 4),
        (FLEE_SUCCEEDED, 1),
        (KNOWS_TARGET, 1),
        (USE_HORSE, 1),
    ] {
        e.call(0x0086_4980, &args![buffer, field(this, offset), size]);
    }
    for offset in [TELEPORT_DOOR, REF_FLEE_TO] {
        e.call(0x0086_48e0, &args![buffer, field(this, offset)]);
    }
    let count = e.call(0x0086_4a60, &args![buffer]).u32();
    e.with_stack(4, |e, slot| {
        let mut index = 0u32;
        while index < count {
            e.mem.set_u32(slot.addr(), 0);
            e.call(0x0086_48e0, &args![buffer, slot]);
            e.call(LIST_APPEND, &args![field(this, AVOID_LIST), slot]);
            index += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x009f_0e00,
            flee_package_flee_package(Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(0x009f_1090, fn_009f1090(Ptr, u32) -> Ptr),
        entry!(0x009f_10c0, fn_009f10c0(Ptr)),
        entry!(
            0x009f_1140,
            flee_package_find_teleport_door(Ptr, u32, u32) -> bool
        ),
        entry!(
            0x009f_1210,
            flee_package_get_closest_avoided_ref(Ptr, u32) -> u32
        ),
        entry!(0x009f_1310, flee_package_add_avoided_ref(Ptr, u32)),
        entry!(0x009f_1350, fn_009f1350(Ptr, u32)),
        entry!(0x009f_1440, fn_009f1440(Ptr) -> u32),
        entry!(0x009f_1600, fn_009f1600(u32, u32) -> bool),
        entry!(0x009f_1830, fn_009f1830(Ptr, u32)),
        entry!(0x009f_1990, flee_package_should_shut_down(Ptr) -> bool),
        entry!(0x009f_19f0, fn_009f19f0(Ptr) -> u16),
        entry!(0x009f_1b70, fn_009f1b70(Ptr)),
        entry!(0x009f_1e80, flee_package_load_game(Ptr)),
        entry!(0x009f_2220, fn_009f2220(Ptr)),
        entry!(0x009f_2380, flee_package_save_game(Ptr, Ptr)),
        entry!(0x009f_24d0, flee_package_load_game_ov2(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn word(value: u32) -> Ret {
        value.into_ret()
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An object whose vtable slots answer the given results (each slot a
    /// double at `vtable + 0x10_0000 + slot`).
    fn object(e: &mut Engine, vtable: u32, slots: &[(u32, Ret)]) -> u32 {
        let size = slots.iter().map(|(s, _)| *s).max().unwrap_or(0) / 4 + 1;
        let mut words = vec![0u32; size as usize];
        for (slot, ret) in slots {
            let target = vtable + 0x10_0000 + slot;
            words[*slot as usize / 4] = target;
            let ret = *ret;
            e.register_double(target, move |_, _| ret);
        }
        e.put_vtable(vtable, &words);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// `BSSimpleList` doubles over `{item, next}` nodes: empty when both
    /// words are 0; the head removal moves the next node into the head;
    /// the append fills the first empty item or adds a node.
    fn lists(e: &mut Engine) {
        e.register(LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next != 0 {
                (e.mem.u32(next), e.mem.u32(next + 4))
            } else {
                (0, 0)
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(LIST_APPEND, |e, a| {
            let value = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) != 0 {
                while e.mem.u32(node + 4) != 0 {
                    node = e.mem.u32(node + 4);
                }
                let fresh = e.mem.alloc(8);
                e.mem.set_u32(node + 4, fresh);
                node = fresh;
            }
            e.mem.set_u32(node, value);
            Ret::default()
        });
        e.register(LIST_COUNT, |e, a| {
            let mut count = 0u32;
            let mut node = a[0];
            while node != 0 && e.mem.u32(node) != 0 {
                count += 1;
                node = e.mem.u32(node + 4);
            }
            count.into_ret()
        });
    }

    fn package(e: &mut Engine, avoided: &[u32]) -> Ptr {
        let this: Ptr = Ptr::new(e.mem.alloc(0xb0));
        for reference in avoided {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), *reference);
                e.call(LIST_APPEND, &args![field(this, AVOID_LIST), slot]);
            });
        }
        this
    }

    fn avoided(e: &Engine, this: Ptr) -> Vec<u32> {
        let mut items = Vec::new();
        let mut node = field(this, AVOID_LIST);
        while node != 0 && e.mem.u32(node) != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    #[test]
    fn constructor_with_and_without_a_reference() {
        for (reference, cell, location) in [
            (0u32, 0u32, 0u32),
            (0x11, 0, 0),
            (0x11, 0x22, 0),
            (0x11, 0x22, 0x33),
        ] {
            let mut e = Engine::new();
            e.map(0x0101_2000, 0x1000);
            e.mem.set_f32(0x0101_2054, -1.0);
            e.map(0x011f_4000, 0x1000);
            e.mem.set_u32(ZERO_POINT, 0x0abc);
            quiet(
                &mut e,
                &[
                    0x0067_07c0,
                    LIST_ITEM,
                    0x0096_a2d0,
                    0x0067_0fc0,
                    0x0082_6b40,
                    0x0082_6b90,
                    SET_LOCATION_REFERENCE,
                    0x0067_f410,
                    0x0067_1d30,
                    0x0067_0b30,
                    0x0067_2fc0,
                    0x007b_3fa0,
                    0x0098_4f60,
                    0x0068_00b0,
                    SET_TARGET_REFERENCE,
                    0x0067_4d70,
                ],
            );
            e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(0x0067_f030, |_, a| a[0].into_ret());
            e.register(0x0067_ff70, |_, a| a[0].into_ret());
            e.register(PACKAGE_TARGET, |_, _| word(0x0c00_0000));
            let this: Ptr = Ptr::new(e.mem.alloc(0xb0));
            e.call_log = Some(vec![]);
            let back = e
                .call(0x009f_0e00, &args![this, reference, cell, location])
                .u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(back, this.addr());
            assert_eq!(e.mem.u32(this.addr()), FLEE_PACKAGE_VTABLE);
            assert_eq!(e.mem.f32(field(this, FLEE_TIMER)), -1.0);
            assert_eq!(e.mem.u32(field(this, FLEE_POINT)), 0x0abc);
            assert_eq!(e.mem.u8(field(this, COMBAT_MODE)), 1);
            assert_eq!(calls(&log, 0x0067_0fc0), vec![vec![this.addr(), 0x16]]);
            if reference == 0 {
                assert!(calls(&log, ALLOCATE).is_empty());
                assert_eq!(e.mem.u8(field(this, EVALUATE_POINT)), 1);
                continue;
            }
            let located = calls(&log, SET_LOCATION_REFERENCE);
            match (cell, location) {
                (0, 0) => assert_eq!(located[0][1], reference),
                (_, 0) => assert_eq!(calls(&log, 0x0067_f410)[0][1], cell),
                _ => assert_eq!(located[0][1], location),
            }
            assert_eq!(
                e.mem.u8(field(this, EVALUATE_POINT)),
                u8::from(cell == 0 && location == 0)
            );
            assert_eq!(calls(&log, 0x0068_00b0), vec![vec![0x0c00_0000, 0]]);
            assert_eq!(
                calls(&log, SET_TARGET_REFERENCE),
                vec![vec![0x0c00_0000, reference]]
            );
            assert_eq!(calls(&log, 0x0098_4f60), vec![vec![this.addr(), 0x13]]);
        }
    }

    #[test]
    fn destructors() {
        let mut e = Engine::new();
        quiet(&mut e, &[0x0047_0470, 0x0046_ffb0, 0x0067_08b0, DEALLOCATE]);
        let this: Ptr = Ptr::new(e.mem.alloc(0xb0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_1090, &args![this, 1u32]).u32(), this.addr());
        e.call(0x009f_1090, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, DEALLOCATE), vec![vec![this.addr()]]);
        assert_eq!(calls(&log, 0x0046_ffb0).len(), 2);
        assert_eq!(e.mem.u32(this.addr()), FLEE_PACKAGE_VTABLE);
    }

    #[test]
    fn avoided_references() {
        let mut e = Engine::new();
        lists(&mut e);
        e.register(LIST_CONTAINS, |e, a| {
            let value = e.mem.u32(a[1]);
            let mut node = a[0];
            let mut found = false;
            while node != 0 {
                found |= e.mem.u32(node) == value;
                node = e.mem.u32(node + 4);
            }
            u32::from(found).into_ret()
        });
        let this = package(&mut e, &[]);
        e.call(0x009f_1310, &args![this, 0x11u32]);
        e.call(0x009f_1310, &args![this, 0x22u32]);
        e.call(0x009f_1310, &args![this, 0x11u32]);
        assert_eq!(avoided(&e, this), vec![0x11, 0x22]);

        // Dropping the target's reference moves the target to the head.
        quiet(
            &mut e,
            &[
                LIST_REMOVE_VALUE,
                SET_TARGET_REFERENCE,
                SET_LOCATION_REFERENCE,
            ],
        );
        e.register(PACKAGE_TARGET, |_, _| word(0x0c00_0000));
        e.register(GET_TARGET_REFERENCE, |_, _| word(0x22));
        e.register(PACKAGE_LOCATION, |_, _| word(0x0c10_0000));
        e.mem.set_u32(field(this, REF_FLEE_TO), 0x22);
        e.call_log = Some(vec![]);
        e.call(0x009f_1350, &args![this, 0x22u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SET_TARGET_REFERENCE),
            vec![vec![0x0c00_0000, 0x11]]
        );
        assert_eq!(
            calls(&log, SET_LOCATION_REFERENCE),
            vec![vec![0x0c10_0000, 0x11]]
        );
        assert_eq!(e.mem.u32(field(this, REF_FLEE_TO)), 0);
        // Another reference: the target stays.
        e.call_log = Some(vec![]);
        e.call(0x009f_1350, &args![this, 0x33u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, SET_TARGET_REFERENCE).is_empty());
    }

    #[test]
    fn closest_avoided_reference() {
        let mut e = Engine::new();
        lists(&mut e);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_f32(0x0101_6970, f32::MAX);
        let near = object(&mut e, 0x0a00_0000, &[(0x22c, word(0))]);
        let far = object(&mut e, 0x0a20_0000, &[(0x22c, word(0))]);
        let dead = object(&mut e, 0x0a40_0000, &[(0x22c, word(1))]);
        let this = package(&mut e, &[far, dead, near]);
        e.mem.set_u8(field(this, EVALUATE_POINT), 1);
        e.register(REF_PERSISTS, |_, _| word(1));
        e.register_double(DISTANCE_TO, move |_, a| {
            st0(if a[0] == near { 5.0 } else { 50.0 })
        });
        e.register(REF_TEST_00440DA0, |_, _| word(0));
        // The removal (009f1350) and its callees.
        e.register(LIST_REMOVE_VALUE, |e, a| {
            let value = e.mem.u32(a[1]);
            let mut node = a[0];
            let mut previous = 0;
            while node != 0 && e.mem.u32(node) != value {
                previous = node;
                node = e.mem.u32(node + 4);
            }
            if node != 0 && previous != 0 {
                let next = e.mem.u32(node + 4);
                e.mem.set_u32(previous + 4, next);
            }
            Ret::default()
        });
        e.register(PACKAGE_TARGET, |_, _| word(0));
        assert_eq!(e.call(0x009f_1210, &args![this, 0x99u32]).u32(), near);
        assert_eq!(avoided(&e, this), vec![far, near]);
        // Without `bEvaluatePoint`: none.
        e.mem.set_u8(field(this, EVALUATE_POINT), 0);
        assert_eq!(e.call(0x009f_1210, &args![this, 0x99u32]).u32(), 0);
    }

    #[test]
    fn teleport_door_search() {
        let mut e = Engine::new();
        e.map(0x011f_2000, 0x1000);
        e.map(0x0104_f000, 0x1000);
        e.mem.set_f32(0x0104_fabc, 512.0);
        e.map(0x011c_3000, 0x1000);
        e.set_global(0x011c_3f2c, 0x0e00_0000u32);
        let reference = object(&mut e, 0x0a00_0000, &[(0x1f4, word(0x0c00_0000))]);
        e.register(REF_PERSISTS, |_, _| word(1));
        e.register(0x008d_6f30, |_, _| word(0x0d00_0000));
        e.register(0x0046_f280, |e, a| {
            // The callback finds a door.
            assert_eq!(e.global::<u32>(DOOR_SEARCH_ACTOR), 0x77);
            assert_eq!(a[6], 0x009f_1600);
            e.set_global(DOOR_FOUND, 0x5555u32);
            Ret::default()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(0xb0));
        assert!(!e.call(0x009f_1140, &args![this, reference, 0x77u32]).bool());
        e.mem.set_u8(field(this, EVALUATE_POINT), 1);
        assert!(!e.call(0x009f_1140, &args![this, reference, 0u32]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x009f_1140, &args![this, reference, 0x77u32]).bool());
        let log = e.call_log.take().unwrap();
        let enumerated = calls(&log, 0x0046_f280);
        assert_eq!(enumerated[0][..3], [0x0e00_0000, 0x0d00_0000, 0x0c00_0000]);
        assert_eq!(f32::from_bits(enumerated[0][3]), 512.0);
        assert_eq!(enumerated[0][7], reference);
        assert_eq!(e.mem.u32(field(this, TELEPORT_DOOR)), 0x5555);
        assert_eq!(e.global::<u32>(DOOR_FOUND), 0);
        assert_eq!(e.global::<u32>(DOOR_SEARCH_ACTOR), 0);
    }

    fn door(locked: u32, has_key: u32, path: u32, to_actor: f64) -> (bool, Vec<(u32, Vec<u32>)>) {
        let mut e = Engine::new();
        e.map(0x011f_2000, 0x1000);
        e.set_global(DOOR_SEARCH_ACTOR, 0x0e00_0000u32);
        e.map(0x011c_a000, 0x1000);
        e.set_global(0x011c_a258, 0x0f00_0000u32);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_f32(0x0101_e704, 128.0);
        e.map(0x0102_2000, 0x1000);
        e.mem.set_f32(0x0102_2454, 800.0);
        for address in [
            REF_TEST_00440D80,
            0x0040_77c0,
            REF_TEST_00440DA0,
            0x0057_5d10,
        ] {
            e.register(address, |_, _| word(0));
        }
        e.register(0x007a_f430, |_, _| word(0x1234));
        e.register(0x0056_8e50, |_, _| word(1));
        e.register_double(DISTANCE_TO, move |_, a| {
            st0(if a[1] == 0x0e00_0000 { 500.0 } else { to_actor })
        });
        let lock = e.mem.alloc(8);
        e.mem.set_u32(lock + 4, 0x4444);
        e.register_double(0x0056_9160, move |_, _| word(lock));
        e.register_double(0x0050_21a0, move |_, _| word(locked));
        e.register(0x0057_85e0, |_, _| word(0));
        e.register_double(0x0089_1db0, move |_, _| word(has_key));
        quiet(&mut e, &[0x006d_cd70, 0x006d_ce10, 0x004f_f7e0]);
        e.register(0x0056_8fa0, |_, _| word(0x0c00_0000));
        e.register_double(0x006d_62c0, move |_, _| word(path));
        e.call_log = Some(vec![]);
        let found = e.call(0x009f_1600, &args![0x3333u32, 0x7777u32]).bool();
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u32>(DOOR_FOUND), if found { 0x3333 } else { 0 });
        (found, log)
    }

    #[test]
    fn door_callback() {
        // Within 800 of the searcher (500 away): no, but closer to the
        // actor (10) than the searcher: yes; unlocked with a path.
        let (found, log) = door(0, 0, 1, 10.0);
        assert!(found);
        assert_eq!(calls(&log, 0x006d_ce10)[0][1..], [0x0c00_0000, 0x3333]);
        assert_eq!(calls(&log, 0x004f_f7e0).len(), 2);
        // Locked without the key.
        let (found, log) = door(1, 0, 1, 10.0);
        assert!(!found);
        assert_eq!(calls(&log, 0x0089_1db0)[0][1..5], [0x4444, 0, 1, 0]);
        // Locked with the key, but no path.
        let (found, _) = door(1, 1, 0, 10.0);
        assert!(!found);
        // Neither near: farther from the actor than from the searcher.
        let (found, log) = door(0, 0, 1, 2000.0);
        assert!(!found);
        assert!(calls(&log, 0x0056_9160).is_empty());
    }

    #[test]
    fn avoid_spheres() {
        let mut e = Engine::new();
        lists(&mut e);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_f32(0x0101_6410, 100.0);
        e.map(0x0102_3000, 0x1000);
        e.mem.set_f32(0x0102_31a0, 300.0);
        e.map(0x0103_2000, 0x1000);
        e.mem.set_f32(0x0103_2adc, 256.0);
        let live = object(
            &mut e,
            0x0a00_0000,
            &[(0x22c, word(0)), (0x1f4, word(0x0c00_0000))],
        );
        let gone = object(&mut e, 0x0a20_0000, &[(0x22c, word(1))]);
        let this = package(&mut e, &[live, gone, live]);
        e.register(0x00aa_13e0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x006e_3850, |_, a| a[0].into_ret());
        e.register(PACKAGE_TARGET, |_, _| word(0));
        e.register(0x0040_4010, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(REF_TEST_00440DA0, |_, _| word(0));
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x006e_3ad0, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(0x0069_15d0, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[0] + 4 * i, a[1 + (i as usize % 3)]);
            }
            a[0].into_ret()
        });
        let slots = e.mem.alloc(0x100);
        e.register_double(0x006a_1400, move |_, a| word(slots + 0x24 * a[1]));
        e.call_log = Some(vec![]);
        let array = e.call(0x009f_1440, &args![this]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(array + 8), 2);
        let spheres = calls(&log, 0x0069_15d0);
        assert_eq!(spheres.len(), 2);
        // The radius: max(300, 256) = 300; the height max(300 * 300, 100).
        assert_eq!(spheres[0][2], 300.0f32.to_bits());
        assert_eq!(spheres[0][3], 90000.0f32.to_bits());
        assert_eq!(e.mem.u32(slots + 0x24), 0x0c00_0000);
    }

    #[test]
    fn pruning_and_shut_down() {
        let mut e = Engine::new();
        lists(&mut e);
        e.map(0x011c_d000, 0x1000);
        e.mem.set_f32(0x011c_d348 + 4, 30.0);
        e.register(0x0040_3e20, |_, a| (a[0] + 4).into_ret());
        e.map(0x011d_e000, 0x1000);
        let player = object(&mut e, 0x0a00_0000, &[(0x100, word(1)), (0x22c, word(0))]);
        e.set_global(PLAYER, player);
        let stranger = object(&mut e, 0x0a20_0000, &[(0x100, word(1)), (0x22c, word(0))]);
        let thing = object(&mut e, 0x0a40_0000, &[(0x100, word(0)), (0x22c, word(0))]);
        let this = package(&mut e, &[stranger, thing, player]);
        e.mem.set_u8(field(this, COMBAT_MODE), 1);
        e.register(REF_TEST_00440D80, |_, _| word(0));
        e.register(0x0049_3bb0, |_, _| word(0));
        e.register(0x0043_5dd0, |_, _| st0(100.0));
        let combat = object(&mut e, 0x0a60_0000, &[(0x20, st0(80.0))]);
        e.call_log = Some(vec![]);
        e.call(0x009f_1830, &args![this, combat]);
        let log = e.call_log.take().unwrap();
        // The stranger (not 00493bb0, not the player) goes; the player was
        // seen 20 ago (within 30) and stays.
        assert_eq!(avoided(&e, this), vec![thing, player]);
        assert_eq!(calls(&log, 0x0a70_0020), vec![vec![combat, player, 0]]);
        // Not shut down with two references; with only the player out of
        // combat, it is.
        e.register(0x0095_3c50, |_, _| word(0));
        assert!(!e.call(0x009f_1990, &args![this]).bool());
        e.register(0x0043_5dd0, |_, _| st0(200.0));
        // The thing goes once it is gone.
        e.register(0x0a50_022c, |_, _| word(1));
        e.call(0x009f_1830, &args![this, combat]);
        assert_eq!(avoided(&e, this), vec![]);
        let this = package(&mut e, &[player]);
        assert!(e.call(0x009f_1990, &args![this]).bool());
        e.register(0x0095_3c50, |_, _| word(1));
        assert!(!e.call(0x009f_1990, &args![this]).bool());
    }

    /// The save/load manager with `blocks` and `version`, no logging, and
    /// a buffer position at the returned block.
    fn manager(e: &mut Engine, blocks: u32, version: u32) -> u32 {
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x0b00_0000u32);
        e.register_double(USE_SAVE_GAME_BLOCKS, move |_, _| word(blocks));
        e.register_double(SAVE_VERSION, move |_, _| word(version));
        e.register(BOOL_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(SAVED_FORM, |_, _| word(0));
        e.register(LOADED_FORM, |_, _| word(0));
        let position = e.mem.alloc(0x100);
        let cursor = std::rc::Rc::new(std::cell::Cell::new(position));
        let at = cursor.clone();
        e.register_double(BUFFER_POSITION, move |_, _| word(at.get()));
        let step = cursor;
        e.register_double(RAW_WRITE, move |_, a| {
            step.set(step.get() + a[2]);
            Ret::default()
        });
        quiet(e, &[ERROR, PRINT]);
        position
    }

    #[test]
    fn old_save_size_save_and_load() {
        let mut e = Engine::new();
        lists(&mut e);
        let base = manager(&mut e, 1, 0x71);
        let this = package(&mut e, &[0x0c00_0000, 0x0c10_0000]);
        e.register(0x0067_8fd0, |_, _| word(0x10));
        // 0x10 + 6 + 2 * 4 + 2 + 0x1d.
        assert_eq!(
            e.call(0x009f_19f0, &args![this]).u16(),
            0x10 + 6 + 8 + 2 + 0x1d
        );

        quiet(&mut e, &[0x0067_9120, SAVE_DATA_OLD]);
        e.register(FORM_ID, |_, a| (a[0] >> 16).into_ret());
        let ids = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let list = ids.clone();
        e.register_double(SAVE_NUMERIC_ID, move |e, a| {
            list.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.mem.set_u32(field(this, REF_FLEE_TO), 0x0d00_0000);
        e.call(0x009f_1b70, &args![this]);
        assert_eq!(ids.borrow().clone(), vec![0x0c00, 0x0c10, 0x0d00, 0]);
        // The header (4 bytes) then the length at base + 4, the count at
        // base + 6.
        assert_eq!(e.mem.u16(base + 6), 2);
        assert_eq!(e.mem.u16(base + 4), 4);

        // Load it back (the old format).
        let mut e = Engine::new();
        lists(&mut e);
        manager(&mut e, 0, 0x71);
        quiet(&mut e, &[0x0067_9340]);
        let this = package(&mut e, &[]);
        e.register(LOAD_DATA_OLD, |e, a| {
            if a[2] == 2 {
                e.mem.set_u16(a[1], 2);
            }
            Ret::default()
        });
        let next = std::rc::Rc::new(std::cell::Cell::new(0x100u32));
        let counter = next.clone();
        e.register_double(LOAD_NUMERIC_ID, move |e, a| {
            e.mem.set_u32(a[1], counter.get());
            counter.set(counter.get() + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009f_1e80, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(avoided(&e, this), vec![0x100, 0x101]);
        assert_eq!(e.mem.u32(field(this, REF_FLEE_TO)), 0x102);
        assert_eq!(e.mem.u32(field(this, TELEPORT_DOOR)), 0x103);
        let loaded: Vec<_> = calls(&log, LOAD_DATA_OLD)
            .iter()
            .map(|w| w[1].wrapping_sub(this.addr()))
            .collect();
        assert!(loaded.contains(&(USE_HORSE)));
    }

    #[test]
    fn load_fix_up_resolves_or_drops() {
        let mut e = Engine::new();
        lists(&mut e);
        let this = package(&mut e, &[0x10, 0x20, 0x30]);
        e.mem.set_u32(field(this, REF_FLEE_TO), 0x40);
        quiet(&mut e, &[0x0067_96c0]);
        e.register(FORM_FOR_ID, |_, a| (a[0] + 1).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| {
            // 0x21 (from 0x20) does not resolve.
            let value = if a[0] == 0x21 { 0 } else { a[0] + 0x1000 };
            value.into_ret()
        });
        e.register(LIST_SET_ITEM, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(LIST_REMOVE_VALUE, |e, a| {
            // Unlinks the node after `a[0]`.
            let node = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(node + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009f_2220, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(avoided(&e, this), vec![0x1011, 0x1031]);
        assert_eq!(calls(&log, LIST_REMOVE_VALUE).len(), 1);
        assert_eq!(e.mem.u32(field(this, REF_FLEE_TO)), 0x1041);
        assert_eq!(e.mem.u32(field(this, TELEPORT_DOOR)), 0);
    }

    #[test]
    fn new_save_and_load() {
        let mut e = Engine::new();
        lists(&mut e);
        let this = package(&mut e, &[0x10, 0x20]);
        e.mem.set_u32(field(this, TELEPORT_DOOR), 0x30);
        quiet(
            &mut e,
            &[0x0067_97c0, 0x0086_5e50, 0x0086_5df0, 0x0086_5ff0],
        );
        e.register(0x0086_5f20, |_, _| word(0x99));
        e.call_log = Some(vec![]);
        e.call(0x009f_2380, &args![this, 0x0b00_0000u32]);
        let log = e.call_log.take().unwrap();
        let forms: Vec<_> = calls(&log, 0x0086_5df0).iter().map(|w| w[1]).collect();
        assert_eq!(forms, [0x30, 0, 0x10, 0x20]);
        assert_eq!(calls(&log, 0x0086_5ff0), vec![vec![0x0b00_0000, 2, 0x99]]);
        let sizes: Vec<_> = calls(&log, 0x0086_5e50).iter().map(|w| w[2]).collect();
        assert_eq!(sizes, [1, 1, 0xc, 4, 1, 1, 1]);

        let mut e = Engine::new();
        lists(&mut e);
        let this = package(&mut e, &[]);
        quiet(&mut e, &[0x0067_98c0, 0x0086_4980]);
        let next = std::rc::Rc::new(std::cell::Cell::new(0x50u32));
        let counter = next.clone();
        e.register_double(0x0086_48e0, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0);
            e.mem.set_u32(a[1], counter.get());
            counter.set(counter.get() + 1);
            Ret::default()
        });
        e.register(0x0086_4a60, |_, _| word(2));
        e.call(0x009f_24d0, &args![this, 0x0b00_0000u32]);
        assert_eq!(e.mem.u32(field(this, TELEPORT_DOOR)), 0x50);
        assert_eq!(e.mem.u32(field(this, REF_FLEE_TO)), 0x51);
        assert_eq!(avoided(&e, this), vec![0x52, 0x53]);
    }
}
