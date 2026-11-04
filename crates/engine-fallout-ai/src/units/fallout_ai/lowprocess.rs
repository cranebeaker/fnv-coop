//! `fallout/ai/lowprocess.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_ai::highprocess::LowProcess;

/// The base constructor `00913810` (`this` plus one stack word, `RET 4`).
const BASE_CONSTRUCTOR_00913810: u32 = 0x0091_3810;
/// The vtable stored at `+0` (RTTI class `NiTMap<LowProcess*,LP_LOCK_DATA>`).
const VTABLE_009137B0: u32 = 0x0108_8fd4;

/// The destructor body `00906fe0` (`thiscall`, `this` only).
const DESTRUCTOR_BODY: u32 = 0x0090_6fe0;
/// `00401030` (`cdecl`, one word): frees an object.
const FREE_OBJECT: u32 = 0x0040_1030;
/// `007df1f0` (`thiscall`): the float at `+0x20` (`fHourLastProcessed`, Xbox PDB), in ST0.
const PROCESS_GET_HOUR: u32 = 0x007d_f1f0;
/// `00693d50` (`thiscall`, `RET 4`): stores a float at `+0x20`.
const PROCESS_SET_HOUR: u32 = 0x0069_3d50;
/// `ActorPackage::MoveTo` (Xbox PDB, `008c6130`; `thiscall`, `RET 4`).
const ACTOR_PACKAGE_MOVE_TO: u32 = 0x008c_6130;
/// `0059bb30` (`thiscall`): the word at `+0x24` (`iDateLastProcessed`, Xbox PDB).
const PROCESS_GET_DATE: u32 = 0x0059_bb30;
/// `00937160` (`thiscall`, `RET 4`, unit `modifierlist.cpp`, unnamed): takes the
/// `ModifierList` (Xbox PDB `pDamageModifiers`) at its argument.
const MODIFIER_LIST_COPY: u32 = 0x0093_7160;

/// The vtable `010886e4` (`_scalar_deleting_destructor_`'s table, the `LowProcess` class)
/// that the destructor body `00906fe0` stores at `+0`.
const LOW_PROCESS_VTABLE: u32 = 0x0108_86e4;
/// `BaseProcess::ClearCurrentPackage` (Xbox PDB, `008d03a0`; `thiscall`, plain `RET`).
const CLEAR_CURRENT_PACKAGE: u32 = 0x008d_03a0;
/// `009070a0` (`thiscall`, plain `RET`, unit `lowprocess.cpp`, unnamed): a 439-byte function
/// the destructor body runs on `this` right after `ClearCurrentPackage`.
const LOW_PROCESS_RELEASE: u32 = 0x0090_70a0;
/// `00937030` (`thiscall`, plain `RET`): the destructor of the `ModifierList` at `+0x94`.
const MODIFIER_LIST_DESTRUCT: u32 = 0x0093_7030;
/// `0046ffb0` (`thiscall`, plain `RET`): the base list destructor, run on the four lists at
/// `+0x7c`, `+0x74`, `+0x6c` and `+0x5c`.
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// The base destructor `008d0640` (`thiscall`, plain `RET`), last in the body.
const BASE_PROCESS_DESTRUCT: u32 = 0x008d_0640;

/// `CurrentPackage` (Xbox PDB): the embedded `ActorPackage` at `+0x04`.
const PACKAGE: u32 = 0x04;
/// `iDateLastProcessed` (Xbox PDB), `+0x24`.
const DATE_LAST_PROCESSED: u32 = 0x24;
/// `fCombatDelayTimer` (Xbox PDB `CombatTimer`), `+0x38`: two words.
const COMBAT_DELAY_TIMER: u32 = 0x38;
/// `pTarget` (Xbox PDB), `+0x40`.
const TARGET: u32 = 0x40;
/// `iNumberItemsActivate` (Xbox PDB), `+0x58`.
const NUMBER_ITEMS_ACTIVATE: u32 = 0x58;
/// `pAcquireObject` (Xbox PDB), `+0x64`.
const ACQUIRE_OBJECT: u32 = 0x64;
/// `pSavedAcquireObject` (Xbox PDB), `+0x68`.
const SAVED_ACQUIRE_OBJECT: u32 = 0x68;
/// `pDamageModifiers` (Xbox PDB `ModifierList`), `+0x94`.
const DAMAGE_MODIFIERS: u32 = 0x94;
/// `fEssentialDownTimer` (Xbox PDB), `+0xa4`.
const ESSENTIAL_DOWN_TIMER: u32 = 0xa4;
/// `fDeathTime` (Xbox PDB), `+0xa8`.
const DEATH_TIME: u32 = 0xa8;
/// `fTrackedDamage` (Xbox PDB), `+0xac`.
const TRACKED_DAMAGE: u32 = 0xac;
/// Vtable slot `+0x12c`, `SetTarget` (Xbox PDB name; PC slot used by this copy).
const SET_TARGET_SLOT: u32 = 0x12c;

// Translated from 009137b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lowprocess.cpp`): runs `00913810(this, argument)`, then
/// stores the vtable `01088fd4` (RTTI class `NiTMap<LowProcess*,LP_LOCK_DATA>`) at `+0`.
/// Returns `this`.
pub fn fn_009137b0(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR_00913810, &args![this, argument]);
    e.mem.set_u32(this.addr(), VTABLE_009137B0);
    this
}

// Translated from 00906fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, `RET 4`): runs the
/// destructor body `00906fe0` on `this`, then, when bit 0 of `flags` is set, frees the
/// object with `00401030` (`cdecl`, one word). Returns `this`.
pub fn low_process_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 00907510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lowprocess.cpp`; the level copy that
/// `BaseProcess::Copy` `008d0660` calls for the `LowProcess` level, `thiscall`, `RET 4`):
/// moves the low-process state of `source` into `this`. Takes the hour at `+0x20` through
/// `007df1f0`/`00693d50`, moves the embedded package (`ActorPackage::MoveTo`), copies the
/// flags byte, the item pointer, the combat delay timer and the activated-item count,
/// moves the acquire object (`+0x64`, and `+0x68` when it is set; the source's are
/// cleared), takes the date (`0059bb30`), sets the target through vtable slot `+0x12c`,
/// copies the damage modifiers (`00937160`) and the three floats at `+0xa4`, `+0xa8`,
/// `+0xac` (each as an x87 `FLD`/`FSTP` copy, which quiets a signalling NaN).
pub fn fn_00907510(e: &mut Engine, this: Ptr<LowProcess>, source: Ptr<LowProcess>) {
    let to = this.addr();
    let from = source.addr();
    // `FLD`/`FSTP` of the hour the callee returns in ST0, pushed as the argument.
    let hour = e.call(PROCESS_GET_HOUR, &args![from]).f32();
    e.call(PROCESS_SET_HOUR, &args![to, x87_quiet_bits(hour.to_bits())]);
    // CurrentPackage (Xbox PDB) +0x04: `source`'s package moves into `this`'s.
    e.call(
        ACTOR_PACKAGE_MOVE_TO,
        &args![from.wrapping_add(PACKAGE), to.wrapping_add(PACKAGE)],
    );
    let flags = e.get(source, LowProcess::m_uFlags);
    e.set(this, LowProcess::m_uFlags, flags);
    // pAcquireObject (Xbox PDB) +0x64: moved.
    let acquire = e.mem.u32(from.wrapping_add(ACQUIRE_OBJECT));
    e.mem.set_u32(to.wrapping_add(ACQUIRE_OBJECT), acquire);
    e.mem.set_u32(from.wrapping_add(ACQUIRE_OBJECT), 0);
    // fCombatDelayTimer (Xbox PDB) +0x38, two words.
    let timer_start = e.mem.u32(from.wrapping_add(COMBAT_DELAY_TIMER));
    let timer_delay = e.mem.u32(from.wrapping_add(COMBAT_DELAY_TIMER + 4));
    e.mem
        .set_u32(to.wrapping_add(COMBAT_DELAY_TIMER), timer_start);
    e.mem
        .set_u32(to.wrapping_add(COMBAT_DELAY_TIMER + 4), timer_delay);
    let item = e.get(source, LowProcess::pItemBeingUsed);
    e.set(this, LowProcess::pItemBeingUsed, item);
    // pSavedAcquireObject (Xbox PDB) +0x68: moved only when set.
    let saved_acquire = e.mem.u32(from.wrapping_add(SAVED_ACQUIRE_OBJECT));
    if saved_acquire != 0 {
        e.mem
            .set_u32(to.wrapping_add(SAVED_ACQUIRE_OBJECT), saved_acquire);
        e.mem.set_u32(from.wrapping_add(SAVED_ACQUIRE_OBJECT), 0);
    }
    // iDateLastProcessed (Xbox PDB) +0x24, through the getter `0059bb30`.
    let date = e.call(PROCESS_GET_DATE, &args![from]).u32();
    e.mem.set_u32(to.wrapping_add(DATE_LAST_PROCESSED), date);
    // SetTarget (Xbox PDB vtable slot +0x12c) with pTarget (+0x40).
    let target = e.mem.u32(from.wrapping_add(TARGET));
    e.vcall(to, SET_TARGET_SLOT, &args![target]);
    let items_activated = e.mem.u32(from.wrapping_add(NUMBER_ITEMS_ACTIVATE));
    e.mem
        .set_u32(to.wrapping_add(NUMBER_ITEMS_ACTIVATE), items_activated);
    // pDamageModifiers (Xbox PDB) +0x94: `00937160(this + 0x94, source + 0x94)`.
    e.call(
        MODIFIER_LIST_COPY,
        &args![
            to.wrapping_add(DAMAGE_MODIFIERS),
            from.wrapping_add(DAMAGE_MODIFIERS)
        ],
    );
    // fEssentialDownTimer, fDeathTime, fTrackedDamage (Xbox PDB) +0xa4, +0xa8, +0xac.
    for offset in [ESSENTIAL_DOWN_TIMER, DEATH_TIME, TRACKED_DAMAGE] {
        let bits = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), x87_quiet_bits(bits));
    }
}

// Translated from 00906fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00906fe0 - the C++ exception unwinding states (FS:[0] frame, handler 00f16c52) are not translated
/// Unnamed in the engine map (unit `lowprocess.cpp`; the destructor body that
/// `LowProcess::_scalar_deleting_destructor_` `00906fb0` runs; `thiscall`, plain `RET`).
/// Stores the vtable `010886e4` at `+0`, then runs `BaseProcess::ClearCurrentPackage`
/// (`008d03a0`) and `009070a0` on `this`, the `ModifierList` destructor `00937030` on
/// `+0x94`, the list destructor `0046ffb0` on `+0x7c`, `+0x74`, `+0x6c` and `+0x5c`, and
/// last the base destructor `008d0640` on `this`. The disassembly passes `this` in `ECX` to
/// every call (the decompiler shows `ClearCurrentPackage` with a cookie value instead).
pub fn fn_00906fe0(e: &mut Engine, this: Ptr<LowProcess>) {
    let base = this.addr();
    e.mem.set_u32(base, LOW_PROCESS_VTABLE);
    e.call(CLEAR_CURRENT_PACKAGE, &args![base]);
    e.call(LOW_PROCESS_RELEASE, &args![base]);
    // pDamageModifiers (Xbox PDB `ModifierList`) +0x94.
    e.call(
        MODIFIER_LIST_DESTRUCT,
        &args![base.wrapping_add(DAMAGE_MODIFIERS)],
    );
    for offset in [0x7c_u32, 0x74, 0x6c, 0x5c] {
        e.call(LIST_DESTRUCT, &args![base.wrapping_add(offset)]);
    }
    e.call(BASE_PROCESS_DESTRUCT, &args![base]);
}

/// `007b3fa0` (`thiscall`, `RET 4`): a scalar deleting destructor whose destructor is empty
/// (identical-code folding: many classes share the body); frees the object when bit 0 of
/// its argument is set.
const DELETE_EMPTY_DESTRUCTOR: u32 = 0x007b_3fa0;
/// `008256d0` (`thiscall`): whether both words at `+0` and `+4` of a list head are zero.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0` (`thiscall`): returns `this` (here the list head, whose first word is the
/// head node's item).
const LIST_HEAD_ITEM: u32 = 0x0068_15c0;
/// `00905330` (`thiscall`, `RET 4`): removes from a `BSSimpleList` the item held in the word
/// its argument points to.
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// `00470470` (`thiscall`, plain `RET`): empties a list (frees its chained nodes, zeroes the
/// head item).
const LIST_CLEAR: u32 = 0x0047_0470;

/// `pGenericLocation` (Xbox PDB), `+0x44`.
const GENERIC_LOCATION: u32 = 0x44;
/// `ObjectList` (Xbox PDB `BSSimpleList<ObjectstoAcquire *>`), `+0x5c`.
const OBJECT_LIST: u32 = 0x5c;
/// `GenericLocationsList` (Xbox PDB `BSSimpleList<TESObjectREFR *>`), `+0x6c`.
const GENERIC_LOCATIONS_LIST: u32 = 0x6c;
/// `TempObjectList` (Xbox PDB `BSSimpleList<ObjectstoAcquire *>`), `+0x74`.
const TEMP_OBJECT_LIST: u32 = 0x74;
/// `TempLocList` (Xbox PDB `BSSimpleList<TESObjectREFR *>`), `+0x7c`.
const TEMP_LOC_LIST: u32 = 0x7c;
/// `pObjecttoAcquire` (Xbox PDB, typed `TESBoundObject*` there), `+0x84`.
const OBJECT_TO_ACQUIRE: u32 = 0x84;
/// The vtable slot `+0x10` the code calls with the argument 1 on the object at `+0x84`.
const OBJECT_TO_ACQUIRE_DELETE_SLOT: u32 = 0x10;

/// Takes every item off the list at `list` (`008256d0` until empty): reads the head item
/// through `006815c0` into a stack word, removes it with `00905330(list, &word)`, and frees
/// the word's value afterwards, when non-zero, with `007b3fa0(value, 1)`.
fn free_list_items(e: &mut Engine, list: u32) {
    while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let head = e.call(LIST_HEAD_ITEM, &args![list]).u32();
        let item = e.mem.u32(head);
        let removed = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            e.call(LIST_REMOVE_ITEM, &args![list, slot]);
            // The code reads the stack word again after the call.
            e.mem.u32(slot.addr())
        });
        if removed != 0 {
            e.call(DELETE_EMPTY_DESTRUCTOR, &args![removed, 1u32]);
        }
    }
}

// Translated from 009070a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `lowprocess.cpp`; `thiscall`, plain `RET`; the destructor
/// body `00906fe0` runs it after `ClearCurrentPackage`): releases the low process's acquire
/// state. When `pAcquireObject` (`+0x64`) is set it clears `pSavedAcquireObject` (`+0x68`)
/// if that is the same object, frees the object (`007b3fa0`, flag 1) and clears `+0x64`;
/// then, when `+0x68` is still set, frees and clears it the same way. Every item of
/// `ObjectList` (`+0x5c`) is removed and freed, `GenericLocationsList` (`+0x6c`) is emptied
/// (`00470470`) and `pGenericLocation` (`+0x44`) cleared; every item of `TempObjectList`
/// (`+0x74`) is removed and freed and `TempLocList` (`+0x7c`) emptied. Last, the object at
/// `+0x84` (when set) is called through its vtable slot `+0x10` with the argument 1, and
/// `+0x84` is cleared. Field names are the Xbox PDB's; the code's own offsets match them.
pub fn fn_009070a0(e: &mut Engine, this: Ptr<LowProcess>) {
    let base = this.addr();
    let acquire = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
    if acquire != 0 {
        if e.mem.u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT)) == acquire {
            e.mem.set_u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT), 0);
        }
        // The code reads +0x64 again for the delete.
        let acquire = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
        if acquire != 0 {
            e.call(DELETE_EMPTY_DESTRUCTOR, &args![acquire, 1u32]);
        }
        e.mem.set_u32(base.wrapping_add(ACQUIRE_OBJECT), 0);
    }
    let saved = e.mem.u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT));
    if saved != 0 {
        e.call(DELETE_EMPTY_DESTRUCTOR, &args![saved, 1u32]);
        e.mem.set_u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT), 0);
    }
    free_list_items(e, base.wrapping_add(OBJECT_LIST));
    e.call(
        LIST_CLEAR,
        &args![base.wrapping_add(GENERIC_LOCATIONS_LIST)],
    );
    e.mem.set_u32(base.wrapping_add(GENERIC_LOCATION), 0);
    free_list_items(e, base.wrapping_add(TEMP_OBJECT_LIST));
    e.call(LIST_CLEAR, &args![base.wrapping_add(TEMP_LOC_LIST)]);
    let object = e.mem.u32(base.wrapping_add(OBJECT_TO_ACQUIRE));
    if object != 0 {
        e.vcall(object, OBJECT_TO_ACQUIRE_DELETE_SLOT, &args![1u32]);
    }
    e.mem.set_u32(base.wrapping_add(OBJECT_TO_ACQUIRE), 0);
}

// ---- Play build, recorded batch b003 ----
//
// Vtable slots of the process (`this`) used below, named as the Xbox PDB's
// `BaseProcess` table names them (the PC `LowProcess` vtable `010886e4`
// holds the same functions at these offsets up to `+0x7b0`; the slots from
// `+0x7b4` on are PC-only and named after the function the `LowProcess`
// table holds there, engine map).
/// `+0x14` `UpdateMissedPackages` (actor, flag).
const UPDATE_MISSED_PACKAGES_SLOT: u32 = 0x14;
/// `+0x8c` `SetTargetForPackage` (actor).
const SET_TARGET_FOR_PACKAGE_SLOT: u32 = 0x8c;
/// `+0x90` `SetLocationForPackage` (actor, flag).
const SET_LOCATION_FOR_PACKAGE_SLOT: u32 = 0x90;
/// `+0x22c` `GetCurrentPackage`.
const GET_CURRENT_PACKAGE_SLOT: u32 = 0x22c;
/// `+0x244` `SetupNewPackage` (empty in `LowProcess`).
const SETUP_NEW_PACKAGE_SLOT: u32 = 0x244;
/// `+0x278` `GetActorPackageThatIsRunning`.
const GET_ACTOR_PACKAGE_RUNNING_SLOT: u32 = 0x278;
/// `+0x284` `SetProcedureIndexRunning` (index).
const SET_PROCEDURE_INDEX_RUNNING_SLOT: u32 = 0x284;
/// `+0x2a4` `ProcessGreet` (6 words).
const PROCESS_GREET_SLOT: u32 = 0x2a4;
/// `+0x33c` `EnterCombat` (13 words).
const ENTER_COMBAT_SLOT: u32 = 0x33c;
/// `+0x810` `LowProcess::GetTargetForPackage` (actor, package).
const GET_TARGET_FOR_PACKAGE_SLOT: u32 = 0x810;
/// `+0x814` `LowProcess::GetLocationForPackage` (actor, package, flag).
const GET_LOCATION_FOR_PACKAGE_SLOT: u32 = 0x814;
/// `+0x25c` of a `MiddleHighProcess` (`SetAvoidWaitTimer` in the Xbox PDB's
/// table): the float `Update_ov2` clears.
const SET_AVOID_WAIT_TIMER_SLOT: u32 = 0x25c;

/// `m_uFlags` (Xbox PDB) +0x30 bit set and cleared through `008d7e00`
/// (`thiscall`, `RET 8`: set flag, mask).
const SET_FLAG_BITS: u32 = 0x008d_7e00;
/// `ModifierList::GetModifier` (engine map, `00937730`, `RET 4`; ST0).
const MODIFIER_LIST_GET_MODIFIER: u32 = 0x0093_7730;
/// `NiTMapBase<DFALL<..LowProcess*, LP_LOCK_DATA..>>` vtable (RTTI
/// `.?AV?$NiTMapBase@V?$DFALL@V?$NiTMapItem@PAVLowProcess@@ULP_LOCK_...`).
const LOCK_MAP_BASE_VTABLE: u32 = 0x0108_8ff4;
/// `00438af0` (`thiscall`, plain `RET`): removes every item of a map.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// `00aa1070` (`cdecl`, one word) / `00aa10f0` (`cdecl`, one word): Ni
/// allocator allocate / free.
const NI_ALLOCATE: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
/// `00403d30` (`cdecl`, three words: block, byte, size; the `memset`
/// shape; Ghidra's name `_Fiopen` does not fit the body).
const MEMORY_SET: u32 = 0x0040_3d30;
/// The calendar's statics `LowProcess::ComputeLastTimeProcessed` packs
/// (`fCurrentGameHour`, `iCurrentGameYear`, `cCurrentGameDay`, Xbox PDB
/// `LowProcess` statics) and the month word it reads at `011a39c4`.
const CURRENT_GAME_HOUR: u32 = 0x011e_0358;
const CURRENT_GAME_YEAR: u32 = 0x011e_035c;
const CURRENT_GAME_DAY: u32 = 0x011e_0360;
const CURRENT_GAME_MONTH: u32 = 0x011a_39c4;
/// `BGSMenuPacker::RecomputePacking` (engine map, `007037c0`, `RET 4`; on a
/// process: stores the packed date at `+0x24`).
const STORE_PACKED_DATE: u32 = 0x0070_37c0;
/// `TESObjectREFR::SetTargeted` (engine map, `00564db0`, `RET 4`).
const REFERENCE_SET_TARGETED: u32 = 0x0056_4db0;
/// `00403550` (`thiscall` on an `ActorPackage`, `RET 4`): sets its target.
const ACTOR_PACKAGE_SET_TARGET: u32 = 0x0040_3550;
/// `PatrolActorPackageData::Init` (engine map, `009f2ba0`, `RET 8`).
const PATROL_DATA_INIT: u32 = 0x009f_2ba0;
/// `004839c0` (`cdecl`, one word): the form with that id (0 when there is
/// no data handler or no such form).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// The form id `GetTimeDelta` looks up (its value goes through the
/// `TESGlobal` value getter `00526ac0`).
const TIME_SCALE_FORM_ID: u32 = 0x3a;
/// `Calendar::GetHour` (engine map, `00867da0`, ST0) on the calendar
/// singleton `011de7b8`.
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
const CALENDAR: u32 = 0x011d_e7b8;
/// The `TESGlobal` value getter (`00526ac0`, ST0).
const GLOBAL_VALUE: u32 = 0x0052_6ac0;
/// `24.0` and `3600.0` (`double`s).
const HOURS_PER_DAY: u32 = 0x0103_56d8;
const SECONDS_PER_HOUR: u32 = 0x0101_2640;
/// `0041ca90` (`thiscall` on a package, plain `RET`): its procedure type.
const PACKAGE_GET_TYPE: u32 = 0x0041_ca90;
/// `00441110` (`thiscall` on a package, plain `RET`; engine map
/// `PathingLocation::GetWorldspace` by folding): its flags word.
const PACKAGE_GET_FLAGS: u32 = 0x0044_1110;
/// `0045cd60` (`thiscall` on the process, plain `RET`): the word
/// `ShouldUseCombatControllerForPackage` compares with 1.
const PROCESS_GET_0045CD60: u32 = 0x0045_cd60;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, 5 words) and the type
/// descriptors `Update_ov2` casts between (`MobileObject` to `Actor`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const MOBILE_OBJECT_TYPE: u32 = 0x0118_4920;
const ACTOR_TYPE: u32 = 0x0118_46d4;
/// `0055b980` (`thiscall`, plain `RET`): read by `SetLocationForPackage`
/// and `ChangePackage` (its value is only stored or tested).
const GET_0055B980: u32 = 0x0055_b980;
/// `00406d70` (`cdecl`, two words: actor value, mask): whether the actor
/// value's info (table `011d61c8`) has a flag of the mask at `+0x44`.
const ACTOR_VALUE_HAS_FLAG: u32 = 0x0040_6d70;
/// `_ftol2` (`00ec62c0`): ST0 to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// The actor's `ActorValueOwner` (at `+0xa4`; vtable slot 0 the integer
/// value, `+4` the float value) and its modifier list at `+0xd0`.
const ACTOR_VALUE_OWNER: u32 = 0xa4;
const ACTOR_MODIFIERS: u32 = 0xd0;
/// `005d43c0` (`thiscall` on a reference, plain `RET`): its extra data list
/// (by its use with `ExtraDataList::GetScriptLocals`).
const REFERENCE_GET_EXTRA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetScriptLocals` (engine map, `00418830`).
const EXTRA_GET_SCRIPT_LOCALS: u32 = 0x0041_8830;
/// `Script::Run` (engine map, `005ac1e0`, `RET 0x10`: reference, locals, 0, 1).
const SCRIPT_RUN: u32 = 0x005a_c1e0;
/// The package's script and topic getters (`thiscall`, plain `RET`,
/// `highprocess.cpp`): on start `00903bd0`/`00903bb0`, on change
/// `00903cf0`/`00903cd0`, on done `00903e20`/`00903e00`.
const PACKAGE_START_SCRIPT: u32 = 0x0090_3bd0;
const PACKAGE_START_TOPIC: u32 = 0x0090_3bb0;
const PACKAGE_CHANGE_SCRIPT: u32 = 0x0090_3cf0;
const PACKAGE_CHANGE_TOPIC: u32 = 0x0090_3cd0;
const PACKAGE_DONE_SCRIPT: u32 = 0x0090_3e20;
const PACKAGE_DONE_TOPIC: u32 = 0x0090_3e00;
/// `00671d10` (`thiscall`, plain `RET`): tested on the change script.
const SCRIPT_GET_00671D10: u32 = 0x0067_1d10;
/// `00884f80` (`thiscall` on the actor, plain `RET`, `actor.cpp`): run at
/// the end of `StartNewPackage`.
const ACTOR_AFTER_NEW_PACKAGE: u32 = 0x0088_4f80;

// Translated from 009076a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetProcessedDay` (Xbox PDB): bits 0-8 of
/// `iDateLastProcessed` (+0x24).
pub fn low_process_get_processed_day(e: &mut Engine, this: Ptr<LowProcess>) -> u32 {
    e.mem.u32(this.addr().wrapping_add(DATE_LAST_PROCESSED)) & 0x1ff
}

// Translated from 009076c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetProcessedMonth` (Xbox PDB): bits 9-12 of
/// `iDateLastProcessed`.
pub fn low_process_get_processed_month(e: &mut Engine, this: Ptr<LowProcess>) -> u32 {
    (e.mem.u32(this.addr().wrapping_add(DATE_LAST_PROCESSED)) & 0x1e00) >> 9
}

// Translated from 009076e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetProcessedYear` (Xbox PDB): bits 13-26 of
/// `iDateLastProcessed`.
pub fn low_process_get_processed_year(e: &mut Engine, this: Ptr<LowProcess>) -> u32 {
    (e.mem.u32(this.addr().wrapping_add(DATE_LAST_PROCESSED)) & 0x07ff_e000) >> 0xd
}

// Translated from 0090ac20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, `RET 4`): sets (non-zero `set`) or clears bit 0x40
/// of `m_uFlags` through `008d7e00`.
pub fn fn_0090ac20(e: &mut Engine, this: Ptr<LowProcess>, set: u8) {
    e.call(SET_FLAG_BITS, &args![this, u32::from(set), 0x40u32]);
}

// Translated from 009074f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetDamageModifier` (Xbox PDB): `pDamageModifiers`'
/// `GetModifier` for the actor value (its ST0, unchanged).
pub fn low_process_get_damage_modifier(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor_value: u8,
) -> f64 {
    e.call(
        MODIFIER_LIST_GET_MODIFIER,
        &args![
            this.addr().wrapping_add(DAMAGE_MODIFIERS),
            u32::from(actor_value)
        ],
    )
    .f64()
}

// Translated from 00913810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<.., LowProcess*, LP_LOCK_DATA>` constructor (by its
/// vtable; `thiscall`, `RET 4`): stores the vtable and the bucket count,
/// clears the item count, allocates `buckets * 4` bytes of bucket table
/// (`00aa1070`) and zeroes them (`00403d30`). Returns `this`.
pub fn fn_00913810(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, LOCK_MAP_BASE_VTABLE);
    e.mem.set_u32(base.wrapping_add(4), buckets);
    e.mem.set_u32(base.wrapping_add(0xc), 0);
    let size = e.mem.u32(base.wrapping_add(4)) << 2;
    let table = e.call(NI_ALLOCATE, &args![size]).u32();
    e.mem.set_u32(base.wrapping_add(8), table);
    let size = e.mem.u32(base.wrapping_add(4)) << 2;
    let table = e.mem.u32(base.wrapping_add(8));
    e.call(MEMORY_SET, &args![table, 0u32, size]);
    this
}

// Translated from 00913880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00913880 - the compiler's exception frame (FS:[0] chain, handler 00f16e98) is not translated
/// The `NiTMap<LowProcess*, LP_LOCK_DATA>` destructor (by its vtable
/// `01088fd4`): stores the vtable, removes every item (`00438af0`), then
/// the base destructor `009138e0`.
pub fn fn_00913880(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_009137B0);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_009138e0(e, this);
}

// Translated from 009138e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase<.., LowProcess*, LP_LOCK_DATA>` destructor (by its
/// vtable): stores the vtable, removes every item (`00438af0`) and frees
/// the bucket table (`00aa10f0`).
pub fn fn_009138e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), LOCK_MAP_BASE_VTABLE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.mem.u32(this.addr().wrapping_add(8));
    e.call(NI_FREE, &args![table]);
}

// Translated from 00912960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessPatrolStartAtRefBehavior` (Xbox PDB; `RET 8`, the
/// first word unread): `SetupNewPackage` (vtable `+0x244`), then the patrol
/// data's float at `+8` cleared and its state word at `+4` set to 2.
pub fn low_process_process_patrol_start_at_ref_behavior(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    _unused_1: u32,
    data: Ptr,
) {
    e.vcall(this.addr(), SETUP_NEW_PACKAGE_SLOT, &args![]);
    e.mem.set_u32(data.addr().wrapping_add(8), 0);
    e.mem.set_u32(data.addr().wrapping_add(4), 2);
}

// Translated from 00907650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ComputeLastTimeProcessed` (Xbox PDB): the hour last
/// processed becomes `fCurrentGameHour` (`00693d50`, an x87 copy), and the
/// date the packed `month << 9 | day | year << 13` (`007037c0`; the day byte
/// is sign-extended before the `OR`).
pub fn low_process_compute_last_time_processed(e: &mut Engine, this: Ptr<LowProcess>) {
    let hour = e.global::<u32>(CURRENT_GAME_HOUR);
    e.call(PROCESS_SET_HOUR, &args![this, x87_quiet_bits(hour)]);
    let month = e.global::<u32>(CURRENT_GAME_MONTH);
    let day = e.global::<i8>(CURRENT_GAME_DAY);
    let year = e.global::<u32>(CURRENT_GAME_YEAR);
    let packed = (month << 9) | (i32::from(day) as u32) | (year << 0xd);
    e.call(STORE_PACKED_DATE, &args![this, packed]);
}

// Translated from 00911e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetTarget` (Xbox PDB): marks a non-null target targeted
/// (`TESObjectREFR::SetTargeted(1)`), stores it as `pTarget` (+0x40) and
/// gives it to the running package (`GetActorPackageThatIsRunning`, vtable
/// `+0x278`, then `00403550`).
pub fn low_process_set_target(e: &mut Engine, this: Ptr<LowProcess>, target: Ptr) {
    if target.addr() != 0 {
        e.call(REFERENCE_SET_TARGETED, &args![target, 1u32]);
    }
    e.mem
        .set_u32(this.addr().wrapping_add(TARGET), target.addr());
    let package = e
        .vcall(this.addr(), GET_ACTOR_PACKAGE_RUNNING_SLOT, &args![])
        .u32();
    e.call(ACTOR_PACKAGE_SET_TARGET, &args![package, target]);
}

// Translated from 009122e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessPatrolInit` (Xbox PDB; `RET 8`): initializes the
/// patrol data for the actor and the current package
/// (`PatrolActorPackageData::Init`); when that fails the running procedure
/// index becomes -1 (vtable `+0x284`) and it returns false, otherwise
/// `ProcessPatrolStartTravelToNextRef` runs and it returns true.
pub fn low_process_process_patrol_init(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    data: Ptr,
) -> bool {
    let package = e
        .vcall(this.addr(), GET_CURRENT_PACKAGE_SLOT, &args![])
        .u32();
    if !e
        .call(PATROL_DATA_INIT, &args![data, actor, package])
        .bool()
    {
        e.vcall(
            this.addr(),
            SET_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![u32::MAX],
        );
        return false;
    }
    low_process_process_patrol_start_travel_to_next_ref(e, this, actor, data);
    true
}

// Translated from 00908bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetTargetForPackage` (Xbox PDB): `SetTarget` (vtable
/// `+0x12c`) with `GetTargetForPackage` (`+0x810`) for the actor and the
/// current package; returns whether `pTarget` is set.
pub fn low_process_set_target_for_package(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
) -> bool {
    let package = e
        .vcall(this.addr(), GET_CURRENT_PACKAGE_SLOT, &args![])
        .u32();
    let target = e
        .vcall(
            this.addr(),
            GET_TARGET_FOR_PACKAGE_SLOT,
            &args![actor, package],
        )
        .u32();
    e.vcall(this.addr(), SET_TARGET_SLOT, &args![target]);
    e.mem.u32(this.addr().wrapping_add(TARGET)) != 0
}

// Translated from 00908c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SetLocationForPackage` of `LowProcess` (vtable `+0x90`; unnamed in
/// the engine map; `RET 8`): without a current package 0; otherwise the
/// location `GetLocationForPackage` (`+0x814`) gives for the actor, package
/// and flag becomes `pGenericLocation` (+0x44, flag 0) or
/// `pGenericSecondLocation` (+0x48) and is returned (`0055b980` is read on
/// the package in between, its value unused).
pub fn fn_00908c50(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr, second: u8) -> u32 {
    let package = e
        .vcall(this.addr(), GET_CURRENT_PACKAGE_SLOT, &args![])
        .u32();
    if package == 0 {
        return 0;
    }
    let location = e
        .vcall(
            this.addr(),
            GET_LOCATION_FOR_PACKAGE_SLOT,
            &args![actor, package, u32::from(second)],
        )
        .u32();
    e.call(GET_0055B980, &args![package]);
    let field = if second == 0 { 0x44 } else { 0x48 };
    e.mem.set_u32(this.addr().wrapping_add(field), location);
    location
}

// Translated from 00907260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetTimeDelta` (Xbox PDB; `RET 4`, the word unread): the
/// game hours since the hour last processed (the calendar's hour, plus 24
/// when it is below the last hour), as seconds: `(hour - last) * (3600 /
/// value of the form with id 0x3a)`. Each x87 step and `float` store as the
/// exe does them.
pub fn low_process_get_time_delta(e: &mut Engine, this: Ptr<LowProcess>, _unused_1: u32) -> f32 {
    let scale_form = e.call(LOOKUP_FORM_BY_ID, &args![TIME_SCALE_FORM_ID]).u32();
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    let mut hour_bits = x87_store32(hour);
    let last = e.call(PROCESS_GET_HOUR, &args![this]).f64();
    if x87_load32(hour_bits) < last {
        let day: f64 = e.global(HOURS_PER_DAY);
        hour_bits = x87_store32(x87_add(x87_load32(hour_bits), day));
    }
    let last = e.call(PROCESS_GET_HOUR, &args![this]).f64();
    let delta = x87_store32(x87_sub(x87_m32(hour_bits), last));
    let scale = e.call(GLOBAL_VALUE, &args![scale_form]).f64();
    let seconds: f64 = e.global(SECONDS_PER_HOUR);
    let per_hour = x87_div(seconds, scale);
    let result = x87_store32(x87_mul(per_hour, x87_m32(delta)));
    f32::from_bits(result)
}

// Translated from 009078d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ShouldUseCombatControllerForPackage` (Xbox PDB; `RET 8`):
/// for packages of procedure type 0, 1, 6, 7, 0xd, 0xe or 0x11 (the
/// `switch` through the tables at `00907944`/`0090794c`), true when the
/// package's flags (`00441110`) have 0x2000000 (or 0x4000000 too, with
/// `flag`) and `0045cd60` on the process is at most 1; false otherwise.
pub fn low_process_should_use_combat_controller_for_package(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    package: Ptr,
    flag: u8,
) -> bool {
    let kind = e.call(PACKAGE_GET_TYPE, &args![package]).u32();
    match kind {
        0 | 1 | 6 | 7 | 0xd | 0xe | 0x11 => {
            let mut mask = 0x0200_0000u32;
            if flag != 0 {
                mask |= 0x0400_0000;
            }
            if e.call(PACKAGE_GET_FLAGS, &args![package]).u32() & mask == 0 {
                return false;
            }
            e.call(PROCESS_GET_0045CD60, &args![this]).i32() <= 1
        }
        _ => false,
    }
}

// Translated from 0090c360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::Update_ov2` (Xbox PDB; `RET 8`): for an actor that casts
/// (`__RTDynamicCast`, `MobileObject` to `Actor`) to non-null: when `time`
/// is above 3600.0 (an x87 `FCOMP`: not when equal, lower or NaN)
/// `UpdateMissedPackages(actor, 1)` (vtable `+0x14`), then the cast
/// object's slot `+0x25c` with 0.0.
pub fn low_process_update_ov2(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr, time: f32) {
    if actor.addr() == 0 {
        return;
    }
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![actor, 0u32, MOBILE_OBJECT_TYPE, ACTOR_TYPE, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    let limit: f64 = e.global(SECONDS_PER_HOUR);
    if x87_load32(time.to_bits()) > limit {
        e.vcall(
            this.addr(),
            UPDATE_MISSED_PACKAGES_SLOT,
            &args![actor, 1u32],
        );
    }
    e.vcall(cast, SET_AVOID_WAIT_TIMER_SLOT, &args![0u32]);
}

/// The actor value's modifier sum both getters share: the actor's own
/// modifier (`+0xd0`, 0.0 without an actor; stored as a `float`) added to
/// `value`, then the process's damage modifier added (`double` steps).
fn modified_actor_value(
    e: &mut Engine,
    this: u32,
    actor_value: u32,
    actor: u32,
    value: impl FnOnce(&mut Engine) -> f64,
) -> f64 {
    let mut own = 0u32;
    if actor != 0 {
        let modifier = e
            .call(
                MODIFIER_LIST_GET_MODIFIER,
                &args![actor.wrapping_add(ACTOR_MODIFIERS), actor_value & 0xff],
            )
            .f64();
        own = x87_store32(modifier);
    }
    let base = value(e);
    let sum = x87_add(base, x87_m32(own));
    let damage = e
        .call(
            MODIFIER_LIST_GET_MODIFIER,
            &args![this.wrapping_add(DAMAGE_MODIFIERS), actor_value & 0xff],
        )
        .f64();
    x87_add(damage, sum)
}

// Translated from 009072d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetActorValue` (Xbox PDB; `RET 0xc`, the first word
/// unread): for actor values whose info has flag 0x100 (`00406d70`) the
/// actor's own integer value (its `ActorValueOwner` at `+0xa4`, slot 0);
/// otherwise that value plus the actor's modifier plus the damage modifier,
/// truncated (`_ftol2`).
pub fn low_process_get_actor_value(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    _unused_1: u32,
    actor_value: u32,
    actor: Ptr,
) -> i32 {
    let owner = actor.addr().wrapping_add(ACTOR_VALUE_OWNER);
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![actor_value, 0x100u32])
        .bool()
    {
        return e.vcall(owner, 0, &args![actor_value]).i32();
    }
    let total = modified_actor_value(e, this.addr(), actor_value, actor.addr(), |e| {
        f64::from(e.vcall(owner, 0, &args![actor_value]).i32())
    });
    e.call(FTOL, &args![total]).i32()
}

// Translated from 00907380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetActorFloatValue` (Xbox PDB; `RET 0xc`, the first word
/// unread): for actor values whose info has flag 0x100 the actor's own
/// float value (`ActorValueOwner` slot `+4`, its ST0 unchanged); otherwise
/// that value plus the actor's modifier plus the damage modifier, stored
/// as a `float`.
pub fn low_process_get_actor_float_value(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    _unused_1: u32,
    actor_value: u32,
    actor: Ptr,
) -> f64 {
    let owner = actor.addr().wrapping_add(ACTOR_VALUE_OWNER);
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![actor_value, 0x100u32])
        .bool()
    {
        return e.vcall(owner, 4, &args![actor_value]).f64();
    }
    let total = modified_actor_value(e, this.addr(), actor_value, actor.addr(), |e| {
        e.vcall(owner, 4, &args![actor_value]).f64()
    });
    f64::from(f32::from_bits(x87_store32(total)))
}

// Translated from 00907840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::CheckAndStartCombatControllerForPackage` (Xbox PDB; `RET
/// 0xc`): for a package that should use the combat controller
/// (`ShouldUseCombatControllerForPackage`) it sets the package's target
/// and location (vtable `+0x8c`, `+0x90` with 0) and enters combat
/// (`+0x33c`: the actor, eleven zeros, the package); returns whether it did.
pub fn low_process_check_and_start_combat_controller_for_package(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    package: Ptr,
    flag: u8,
) -> bool {
    if package.addr() == 0
        || !low_process_should_use_combat_controller_for_package(e, this, package, flag)
    {
        return false;
    }
    e.vcall(this.addr(), SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
    e.vcall(
        this.addr(),
        SET_LOCATION_FOR_PACKAGE_SLOT,
        &args![actor, 0u32],
    );
    let mut words = vec![actor.addr()];
    words.extend([0u32; 11]);
    words.push(package.addr());
    e.vcall(this.addr(), ENTER_COMBAT_SLOT, &words);
    true
}

/// Runs a package's script on the actor (`Script::Run(actor, locals, 0, 1)`
/// with the locals of the actor's extra data list).
fn run_package_script(e: &mut Engine, script: u32, actor: u32) {
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    let locals = e.call(EXTRA_GET_SCRIPT_LOCALS, &args![extra]).u32();
    e.call(SCRIPT_RUN, &args![script, actor, locals, 0u32, 1u32]);
}

/// Greets with a package's topic: `ProcessGreet(actor, topic, 0, 0, 1, 0)`
/// (vtable `+0x2a4`).
fn greet_with_topic(e: &mut Engine, this: u32, actor: u32, topic: u32) {
    e.vcall(
        this,
        PROCESS_GREET_SLOT,
        &args![actor, topic, 0u32, 0u32, 1u32, 0u32],
    );
}

// Translated from 00913510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::StartNewPackage` (Xbox PDB; `RET 8`): with a package, its
/// start script (`00903bd0`) runs on the actor and its start topic
/// (`00903bb0`) is greeted with; then, with an actor, `00884f80` on it.
pub fn low_process_start_new_package(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    package: Ptr,
) {
    if package.addr() != 0 {
        let script = e.call(PACKAGE_START_SCRIPT, &args![package]).u32();
        if script != 0 {
            run_package_script(e, script, actor.addr());
        }
        let topic = e.call(PACKAGE_START_TOPIC, &args![package]).u32();
        if topic != 0 {
            greet_with_topic(e, this.addr(), actor.addr(), topic);
        }
    }
    if actor.addr() != 0 {
        e.call(ACTOR_AFTER_NEW_PACKAGE, &args![actor]);
    }
}

// Translated from 009135a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ChangePackage` (Xbox PDB; `RET 8`): with a package, its
/// change script (`00903cf0`) runs on the actor when `00671d10` or
/// `0055b980` on it is non-zero, and its change topic (`00903cd0`) is
/// greeted with.
pub fn low_process_change_package(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr, package: Ptr) {
    if package.addr() == 0 {
        return;
    }
    let script = e.call(PACKAGE_CHANGE_SCRIPT, &args![package]).u32();
    if script != 0
        && (e.call(SCRIPT_GET_00671D10, &args![script]).u32() != 0
            || e.call(GET_0055B980, &args![script]).u32() != 0)
    {
        run_package_script(e, script, actor.addr());
    }
    let topic = e.call(PACKAGE_CHANGE_TOPIC, &args![package]).u32();
    if topic != 0 {
        greet_with_topic(e, this.addr(), actor.addr(), topic);
    }
}

// Translated from 00913630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::PackageDone` (Xbox PDB; `RET 8`): with a package, its done
/// script (`00903e20`) runs on the actor and its done topic (`00903e00`) is
/// greeted with.
pub fn low_process_package_done(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr, package: Ptr) {
    if package.addr() == 0 {
        return;
    }
    let script = e.call(PACKAGE_DONE_SCRIPT, &args![package]).u32();
    if script != 0 {
        run_package_script(e, script, actor.addr());
    }
    let topic = e.call(PACKAGE_DONE_TOPIC, &args![package]).u32();
    if topic != 0 {
        greet_with_topic(e, this.addr(), actor.addr(), topic);
    }
}

/// More process vtable slots (Xbox PDB `BaseProcess` names, see above).
/// `+0xb0` `CheckAndStartCombatControllerForPackage` (actor, package, flag).
const CHECK_AND_START_COMBAT_SLOT: u32 = 0xb0;
/// `+0x118` `SetCurrentActionComplete` (flag).
const SET_CURRENT_ACTION_COMPLETE_SLOT: u32 = 0x118;
/// `+0x230` `SetCurrentPackage` (package, actor).
const SET_CURRENT_PACKAGE_SLOT: u32 = 0x230;
/// `+0x238` `SetCurrentProcedureIndex` (index).
const SET_CURRENT_PROCEDURE_INDEX_SLOT: u32 = 0x238;
/// `+0x23c` `GetCurrentProcedureIndex`.
const GET_CURRENT_PROCEDURE_INDEX_SLOT: u32 = 0x23c;
/// `+0x240` `AddToCurrentProcedureIndex` (delta).
const ADD_TO_CURRENT_PROCEDURE_INDEX_SLOT: u32 = 0x240;
/// `+0x27c` `GetPackageThatIsRunning`.
const GET_PACKAGE_RUNNING_SLOT: u32 = 0x27c;
/// `+0x288` `AddToProcedureIndexRunning` (actor, delta).
const ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT: u32 = 0x288;
/// `+0x4ec` `SetTargetActivated` (flag).
const SET_TARGET_ACTIVATED_SLOT: u32 = 0x4ec;
/// `+0x5a0` `PackageDone` (actor, package).
const PACKAGE_DONE_SLOT: u32 = 0x5a0;
/// `+0x5a4` `IsPackageDoneOnce`.
const IS_PACKAGE_DONE_ONCE_SLOT: u32 = 0x5a4;
/// `+0x5a8` `SetPackageDoneOnce` (flag).
const SET_PACKAGE_DONE_ONCE_SLOT: u32 = 0x5a8;

/// A package's vtable slot `+0x13c` (4 words: actor, 0, a `float`, 0; its
/// `AL` tested): asked by `ProcessUseItemAt` and `ProcessFleeNonCombat`
/// with -1.0 (`01012054`).
const PACKAGE_SLOT_13C: u32 = 0x13c;
const MINUS_ONE: u32 = 0x0101_2054;
/// `0067efd0` (`thiscall` on a package, plain `RET`): tested by
/// `ProcessUseItemAt`.
const PACKAGE_TEST_0067EFD0: u32 = 0x0067_efd0;
/// `00671d10` (`thiscall` on a package, plain `RET`): its package target
/// (`PackageTarget`).
const PACKAGE_GET_TARGET: u32 = 0x0067_1d10;
/// `PackageTarget::GetTargReference` (engine map, `00680020`).
const PACKAGE_TARGET_GET_REFERENCE: u32 = 0x0068_0020;
/// `00440d80` (`thiscall`, plain `RET`): tested on the target reference.
const REFERENCE_TEST_00440D80: u32 = 0x0044_0d80;
/// `0044ddc0` (`thiscall` on a package target, plain `RET`): an integer
/// (the flee distance for `ProcessFleeNonCombat`).
const PACKAGE_TARGET_GET_VALUE: u32 = 0x0044_ddc0;
/// `TESObjectREFR::GetDistanceFromReference` (engine map, `005723b0`,
/// `RET 0xc`; ST0).
const REFERENCE_GET_DISTANCE: u32 = 0x0057_23b0;
/// `ProcessGreet`'s callees: `004fd380` (`thiscall` on the actor, plain
/// `RET`), `0057bd60` (`RET 4`), `00508d70` (on the topic, plain `RET`),
/// `TESTopic::CreateDialogueItem` (`0061b320`, `RET 0x14`),
/// `DialogueItem::FirstResponse` / `GetCurrentResponse` / `RunResult`
/// (`0083c7b0`, `0083c820`, `0083c850` `RET 4`), `0084e3a0` (the word at
/// `+0xc`, plain `RET`), `TESObjectREFR::SetSayToTopic` /
/// `SetSayToTopicInfo` (`0057ad20`, `0057ace0`, `RET 4`) and
/// `MobileObject::SayToCallBack` (`00936a20`, `cdecl`, two words).
const ACTOR_GET_004FD380: u32 = 0x004f_d380;
const ACTOR_SET_0057BD60: u32 = 0x0057_bd60;
const TOPIC_TEST_00508D70: u32 = 0x0050_8d70;
const TOPIC_CREATE_DIALOGUE_ITEM: u32 = 0x0061_b320;
const DIALOGUE_FIRST_RESPONSE: u32 = 0x0083_c7b0;
const DIALOGUE_GET_CURRENT_RESPONSE: u32 = 0x0083_c820;
const DIALOGUE_RUN_RESULT: u32 = 0x0083_c850;
const GET_WORD_0C: u32 = 0x0084_e3a0;
const REFERENCE_SET_SAY_TO_TOPIC: u32 = 0x0057_ad20;
const REFERENCE_SET_SAY_TO_TOPIC_INFO: u32 = 0x0057_ace0;
const SAY_TO_CALLBACK: u32 = 0x0093_6a20;
/// The actor's vtable slot `+0x140` (`MobileObject`, `00935f60` in the
/// `Character` table, no stack words): the last call of `ProcessGreet`.
const ACTOR_SLOT_140: u32 = 0x140;
/// `009611e0` (`thiscall` on a package, plain `RET`): its type (the word
/// at `+0x18`).
const PACKAGE_GET_PROCEDURE_TYPE: u32 = 0x0096_11e0;
/// `Procedure::GetProcedureCount` (engine map, `0096a820`, `cdecl`).
const PROCEDURE_COUNT: u32 = 0x0096_a820;
/// The per-type procedure lists (`011a3ff0`, a pointer per type to an array
/// of procedure numbers) and the number `AddToProcedureIndexRunning` treats
/// as the end of the package (0x36).
const PROCEDURE_LISTS: u32 = 0x011a_3ff0;
const PROCEDURE_DONE: u32 = 0x36;
/// `Script::SetActionFlag` (engine map, `005ac750`, `cdecl`, three words:
/// the package, the actor's extra data list, 0x400).
const SET_ACTION_FLAG: u32 = 0x005a_c750;
/// `Actor::GetPackage` (engine map, `00881450`) and
/// `Actor::ClearLookAtTarget` (`008b3d30`).
const ACTOR_GET_PACKAGE: u32 = 0x0088_1450;
const ACTOR_CLEAR_LOOK_AT_TARGET: u32 = 0x008b_3d30;
/// `PackageLocation::GetLocType` (engine map, `00678ca0`).
const PACKAGE_LOCATION_GET_TYPE: u32 = 0x0067_8ca0;
/// `00430830` (`thiscall` on the actor, plain `RET`): the object whose
/// `float` at `+8` is the start location's angle argument.
const ACTOR_GET_00430830: u32 = 0x0043_0830;
/// The actor's vtable slot `+0x1f4` (`Actor::GetLocationOnReference` in the
/// `Character` table) and its combat controller getter `+0x428` (`008a02d0`).
const ACTOR_GET_LOCATION_SLOT: u32 = 0x1f4;
const ACTOR_GET_COMBAT_CONTROLLER_SLOT: u32 = 0x428;
/// `008d6f30` (`thiscall`, plain `RET`) and `TESObjectREFR::GetWorldSpace`
/// (`00575d70`).
const ACTOR_GET_008D6F30: u32 = 0x008d_6f30;
const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// `TESObjectREFR::SetPackageStartLocation` (engine map, `0056ae10`, `RET
/// 0x10`).
const REFERENCE_SET_PACKAGE_START_LOCATION: u32 = 0x0056_ae10;
/// `CombatController::UpdatePackageData` (engine map, `00980ae0`, `RET 4`).
const COMBAT_UPDATE_PACKAGE_DATA: u32 = 0x0098_0ae0;

// Translated from 00908e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessUseItemAt` (Xbox PDB; `RET 4`): with a current
/// package whose slot `+0x13c` (actor, 0, -1.0, 0) fails, the running
/// procedure index moves by -1 (vtable `+0x288`). Otherwise (or without a
/// package), when `0067efd0` on the current package holds, it moves by 1.
/// Returns false.
pub fn low_process_process_use_item_at(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr) -> bool {
    let base = this.addr();
    if e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32() != 0 {
        let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        if !e
            .vcall(
                package,
                PACKAGE_SLOT_13C,
                &args![actor, 0u32, minus_one, 0u32],
            )
            .bool()
        {
            e.vcall(
                base,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, u32::MAX],
            );
            return false;
        }
    }
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    if e.call(PACKAGE_TEST_0067EFD0, &args![package]).bool() {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 1u32],
        );
    }
    false
}

// Translated from 0090a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ResetTarget` (Xbox PDB): when the current package has a
/// package target with a reference, that reference (unless `00440d80`
/// holds for it or it already is `pTarget`) becomes the target (vtable
/// `+0x12c`). Every step asks for the current package again, as the exe
/// does.
pub fn low_process_reset_target(e: &mut Engine, this: Ptr<LowProcess>) {
    let base = this.addr();
    let target_reference = |e: &mut Engine| {
        let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
        let target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
        e.call(PACKAGE_TARGET_GET_REFERENCE, &args![target]).u32()
    };
    if e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32() == 0 {
        return;
    }
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    if e.call(PACKAGE_GET_TARGET, &args![package]).u32() == 0 {
        return;
    }
    if target_reference(e) == 0 {
        return;
    }
    let reference = target_reference(e);
    if e.call(REFERENCE_TEST_00440D80, &args![reference]).bool() {
        return;
    }
    if reference == e.mem.u32(base.wrapping_add(TARGET)) {
        return;
    }
    e.vcall(base, SET_TARGET_SLOT, &args![reference]);
}

// Translated from 00909d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessFleeNonCombat` (Xbox PDB; `RET 4`; nothing is
/// returned in `EAX`): for the running package (vtable `+0x27c`) sets the
/// target when there is none (`+0x8c`) and the location (`+0x90` with 0).
/// The flee distance is the package target's integer (`0044ddc0`, 0.0
/// without a target). With a target no closer than that distance (the x87
/// `FCOMPP`: distance below the target's distance) it stops; without a
/// target the package's slot `+0x13c` runs (actor, 0, -1.0, 0).
pub fn low_process_process_flee_non_combat(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr) {
    let base = this.addr();
    let package = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
    if e.mem.u32(base.wrapping_add(TARGET)) == 0 {
        e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
    }
    e.vcall(base, SET_LOCATION_FOR_PACKAGE_SLOT, &args![actor, 0u32]);
    let mut flee_distance = 0u32;
    if e.call(PACKAGE_GET_TARGET, &args![package]).u32() != 0 {
        let target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
        let value = e.call(PACKAGE_TARGET_GET_VALUE, &args![target]).i32();
        flee_distance = x87_store32(f64::from(value));
    }
    let target = e.mem.u32(base.wrapping_add(TARGET));
    if target != 0 {
        let distance = e
            .call(REFERENCE_GET_DISTANCE, &args![actor, target, 0u32, 0u32])
            .f64();
        if x87_load32(flee_distance) < distance {
            return;
        }
    }
    if e.mem.u32(base.wrapping_add(TARGET)) == 0 {
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        e.vcall(
            package,
            PACKAGE_SLOT_13C,
            &args![actor, 0u32, minus_one, 0u32],
        );
    }
}

// Translated from 009136b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessGreet` (Xbox PDB; `RET 0x18`; the third to fifth
/// words are unread and nothing is returned in `EAX`). Asks for the current
/// package (unused), takes the speaker from `004fd380` on the actor (asked
/// again when 0), runs `0057bd60(actor, 0)`; for a topic `00508d70` accepts
/// it creates the dialogue item (`CreateDialogueItem(actor, speaker, 0, 0,
/// 1)`), moves to its first response, and when there is a current one runs
/// its result (`RunResult(0)`). With `say_to` set the actor then says the
/// topic: `SetSayToTopic`, `SetSayToTopicInfo` with the item's info
/// (`0084e3a0`), `SayToCallBack(0084e3a0(actor), 1)` and the actor's slot
/// `+0x140`.
pub fn low_process_process_greet(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    topic: Ptr,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    say_to: u8,
) {
    e.vcall(this.addr(), GET_CURRENT_PACKAGE_SLOT, &args![]);
    let mut speaker = e.call(ACTOR_GET_004FD380, &args![actor]).u32();
    e.call(ACTOR_SET_0057BD60, &args![actor, 0u32]);
    if speaker == 0 {
        speaker = e.call(ACTOR_GET_004FD380, &args![actor]).u32();
    }
    if !e.call(TOPIC_TEST_00508D70, &args![topic]).bool() {
        return;
    }
    let item = e
        .call(
            TOPIC_CREATE_DIALOGUE_ITEM,
            &args![topic, actor, speaker, 0u32, 0u32, 1u32],
        )
        .u32();
    if item == 0 {
        return;
    }
    e.call(DIALOGUE_FIRST_RESPONSE, &args![item]);
    if e.call(DIALOGUE_GET_CURRENT_RESPONSE, &args![item]).u32() == 0 {
        return;
    }
    e.call(DIALOGUE_RUN_RESULT, &args![item, 0u32]);
    if say_to == 0 {
        return;
    }
    e.call(REFERENCE_SET_SAY_TO_TOPIC, &args![actor, topic]);
    let info = e.call(GET_WORD_0C, &args![item]).u32();
    e.call(REFERENCE_SET_SAY_TO_TOPIC_INFO, &args![actor, info]);
    let speaker_object = e.call(GET_WORD_0C, &args![actor]).u32();
    e.call(SAY_TO_CALLBACK, &args![speaker_object, 1u32]);
    e.vcall(actor.addr(), ACTOR_SLOT_140, &args![]);
}

// Translated from 009119b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::AddToProcedureIndexRunning` (Xbox PDB; `RET 8`): moves the
/// current procedure index by `delta` (vtable `+0x240`), not below 0
/// (`+0x238`). With a current package the index is kept below the type's
/// procedure count (`Procedure::GetProcedureCount`); when the procedure at
/// the index is 0x36 and the package is not done once (`+0x5a4`) it sets
/// the script action flag 0x400 (`005ac750`), runs `PackageDone` (`+0x5a0`)
/// and marks the package done once (`+0x5a8`).
pub fn low_process_add_to_procedure_index_running(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    delta: u32,
) {
    let base = this.addr();
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    e.vcall(base, ADD_TO_CURRENT_PROCEDURE_INDEX_SLOT, &args![delta]);
    if e.vcall(base, GET_CURRENT_PROCEDURE_INDEX_SLOT, &args![])
        .i32()
        < 0
    {
        e.vcall(base, SET_CURRENT_PROCEDURE_INDEX_SLOT, &args![0u32]);
    }
    if package == 0 {
        return;
    }
    let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
    let count = e.call(PROCEDURE_COUNT, &args![kind]).i32();
    if e.vcall(base, GET_CURRENT_PROCEDURE_INDEX_SLOT, &args![])
        .i32()
        >= count
    {
        e.vcall(
            base,
            SET_CURRENT_PROCEDURE_INDEX_SLOT,
            &args![count.wrapping_sub(1)],
        );
    }
    let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
    let index = e
        .vcall(base, GET_CURRENT_PROCEDURE_INDEX_SLOT, &args![])
        .u32();
    let list = e
        .mem
        .u32(PROCEDURE_LISTS.wrapping_add(kind.wrapping_mul(4)));
    if e.mem.u32(list.wrapping_add(index.wrapping_mul(4))) != PROCEDURE_DONE {
        return;
    }
    if e.vcall(base, IS_PACKAGE_DONE_ONCE_SLOT, &args![]).bool() {
        return;
    }
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    e.call(SET_ACTION_FLAG, &args![package, extra, 0x400u32]);
    e.vcall(base, PACKAGE_DONE_SLOT, &args![actor, package]);
    e.vcall(base, SET_PACKAGE_DONE_ONCE_SLOT, &args![1u32]);
}

// Translated from 00907700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SetCurrentPackageFromSchedule` (Xbox PDB; `RET 4`): false
/// when the actor's package (`Actor::GetPackage`) is already the current
/// one. Otherwise it becomes current (vtable `+0x230`), `SetupNewPackage`
/// (`+0x244`) and `Actor::ClearLookAtTarget` run; for a current package
/// without a location (`0055b980`) or with one of type 2 the actor's
/// package start location is set (`SetPackageStartLocation(world space,
/// 008d6f30, GetLocationOnReference, the float at +8 of 00430830)`). The
/// actor's combat controller (slot `+0x428`) updates its package data, and
/// `CheckAndStartCombatControllerForPackage(actor, package, 0)` runs.
/// Returns true.
pub fn low_process_set_current_package_from_schedule(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
) -> bool {
    let base = this.addr();
    let package = e.call(ACTOR_GET_PACKAGE, &args![actor]).u32();
    if package == e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32() {
        return false;
    }
    e.vcall(base, SET_CURRENT_PACKAGE_SLOT, &args![package, actor]);
    e.vcall(base, SETUP_NEW_PACKAGE_SLOT, &args![]);
    e.call(ACTOR_CLEAR_LOOK_AT_TARGET, &args![actor]);
    if e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32() != 0 {
        let current = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
        let set_start = if e.call(GET_0055B980, &args![current]).u32() == 0 {
            true
        } else {
            let current = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
            let location = e.call(GET_0055B980, &args![current]).u32();
            e.call(PACKAGE_LOCATION_GET_TYPE, &args![location]).u32() == 2
        };
        if set_start {
            let object = e.call(ACTOR_GET_00430830, &args![actor]).u32();
            let angle = x87_quiet_bits(e.mem.u32(object.wrapping_add(8)));
            let location = e
                .vcall(actor.addr(), ACTOR_GET_LOCATION_SLOT, &args![])
                .u32();
            let value = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
            let world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
            e.call(
                REFERENCE_SET_PACKAGE_START_LOCATION,
                &args![actor, world_space, value, location, angle],
            );
        }
    }
    let controller = e
        .vcall(actor.addr(), ACTOR_GET_COMBAT_CONTROLLER_SLOT, &args![])
        .u32();
    if controller != 0 {
        e.call(COMBAT_UPDATE_PACKAGE_DATA, &args![controller, package]);
    }
    e.vcall(
        base,
        CHECK_AND_START_COMBAT_SLOT,
        &args![actor, package, 0u32],
    );
    true
}

// Translated from 00910b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ClearCurrentDataforProcess` (Xbox PDB; `RET 4`, the word
/// unread): clears the action-complete flag (vtable `+0x118`), the flag
/// bit 0x40 (`0090ac20`), the target-activated flag (`+0x4ec`), the
/// target (`+0x12c`) and `pGenericLocation`. `pAcquireObject` (+0x64) is
/// freed (`007b3fa0`, flag 1) and cleared; `pSavedAcquireObject` (+0x68)
/// is taken off `ObjectList` (`00905330` with the field's address), freed
/// and cleared. Every item left on `ObjectList` is freed, then taken off
/// (through a stack word holding it), and `GenericLocationsList` is
/// emptied (`00470470`).
pub fn low_process_clear_current_data_for_process(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    _unused_1: u32,
) {
    let base = this.addr();
    e.vcall(base, SET_CURRENT_ACTION_COMPLETE_SLOT, &args![0u32]);
    fn_0090ac20(e, this, 0);
    e.vcall(base, SET_TARGET_ACTIVATED_SLOT, &args![0u32]);
    e.vcall(base, SET_TARGET_SLOT, &args![0u32]);
    e.mem.set_u32(base.wrapping_add(GENERIC_LOCATION), 0);
    let acquire = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
    if acquire != 0 {
        e.call(DELETE_EMPTY_DESTRUCTOR, &args![acquire, 1u32]);
        e.mem.set_u32(base.wrapping_add(ACQUIRE_OBJECT), 0);
    }
    if e.mem.u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT)) != 0 {
        e.call(
            LIST_REMOVE_ITEM,
            &args![
                base.wrapping_add(OBJECT_LIST),
                base.wrapping_add(SAVED_ACQUIRE_OBJECT)
            ],
        );
        let saved = e.mem.u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT));
        if saved != 0 {
            e.call(DELETE_EMPTY_DESTRUCTOR, &args![saved, 1u32]);
        }
        e.mem.set_u32(base.wrapping_add(SAVED_ACQUIRE_OBJECT), 0);
    }
    let list = base.wrapping_add(OBJECT_LIST);
    while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let head = e.call(LIST_HEAD_ITEM, &args![list]).u32();
        let item = e.mem.u32(head);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), item);
            if item != 0 {
                e.call(DELETE_EMPTY_DESTRUCTOR, &args![item, 1u32]);
            }
            e.call(LIST_REMOVE_ITEM, &args![list, slot]);
        });
    }
    e.call(
        LIST_CLEAR,
        &args![base.wrapping_add(GENERIC_LOCATIONS_LIST)],
    );
}

/// The actor's vtable slot `+0x22c` (`008844f0` in the `Character` table,
/// one word; its `AL` tested), slot `+0x214` (`008d8800`, no words; its
/// `EAX` tested) and slot `+0x418` (`008a75a0`, no words).
const ACTOR_SLOT_22C: u32 = 0x22c;
const ACTOR_SLOT_214: u32 = 0x214;
const ACTOR_SLOT_418: u32 = 0x418;
/// The reference's base form (`007af430`, the word at `+0x20`) and the
/// form's type byte (`00401170`, zero-extended).
const REFERENCE_GET_BASE_FORM: u32 = 0x007a_f430;
const FORM_GET_TYPE: u32 = 0x0040_1170;
/// The form type the door helpers check for (0x1c).
const DOOR_FORM_TYPE: u32 = 0x1c;
/// `00568e50` (`thiscall` on a reference, plain `RET`): its teleport data
/// (by its use with `DoorTeleportData::SetConnectedCellsPublic`).
const REFERENCE_GET_TELEPORT_DATA: u32 = 0x0056_8e50;
/// `TESObjectREFR::IsAnOwner` (engine map, `005785e0`, `RET 8`),
/// `GetLock` (`00569160`), `Lock` (`00569250`), `UnLock` (`005692a0`).
const REFERENCE_IS_AN_OWNER: u32 = 0x0057_85e0;
const REFERENCE_GET_LOCK: u32 = 0x0056_9160;
const REFERENCE_LOCK: u32 = 0x0056_9250;
const REFERENCE_UNLOCK: u32 = 0x0056_92a0;
/// `DoorTeleportData::SetConnectedCellsPublic` (engine map, `0043a210`,
/// `RET 8`) and `GetTeleportWorldSpace` (`0043a320`).
const TELEPORT_SET_CONNECTED_CELLS_PUBLIC: u32 = 0x0043_a210;
const TELEPORT_GET_WORLD_SPACE: u32 = 0x0043_a320;
/// `00425fd0` (`thiscall`, plain `RET`): tested on `008d6f30`'s object.
const TEST_00425FD0: u32 = 0x0042_5fd0;
/// `008d0430` (`thiscall` on the process, plain `RET`; the
/// `GetInstanceDataThatIsRunning` slot's function): the running package's
/// instance data.
const PROCESS_GET_INSTANCE_DATA: u32 = 0x008d_0430;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `PatrolActorPackageData::GetCurrentReference` (engine map, `009f3390`)
/// and `SetNextReferenceForPatrol` (`009f2ea0`, `RET 4`).
const PATROL_GET_CURRENT_REFERENCE: u32 = 0x009f_3390;
const PATROL_SET_NEXT_REFERENCE: u32 = 0x009f_2ea0;
/// `0041fe90` (`thiscall` on an extra data list, plain `RET`): the extra
/// whose float (`006a7f50`, ST0) is the patrol wait time.
const EXTRA_GET_0041FE90: u32 = 0x0041_fe90;
const EXTRA_WAIT_TIME_006A7F50: u32 = 0x006a_7f50;
/// `00404010` (`cdecl`, two `float`s; ST0): the larger of the two by its
/// use here (not confirmed).
const LARGER_OF_00404010: u32 = 0x0040_4010;
/// `004578c0` (`thiscall` on a point, `RET 4`) and `004a7290` (`thiscall` on
/// a point, ST0): the point minus another, and its length (by their use).
const POINT_SUBTRACT_004578C0: u32 = 0x0045_78c0;
const POINT_LENGTH_004A7290: u32 = 0x004a_7290;
/// `0.001` (`double`) and `FLT_MAX` (`double`).
const PATROL_NEAR: u32 = 0x0101_6978;
const FLOAT_MAX: u32 = 0x0102_31b0;
/// `00440da0` / `00440d80` (`thiscall` on a reference, plain `RET`): tested
/// on the ambush target.
const REFERENCE_TEST_00440DA0: u32 = 0x0044_0da0;
/// `TESPackage::SetNeverRun` (engine map, `00674e70`, `RET 8`).
const PACKAGE_SET_NEVER_RUN: u32 = 0x0067_4e70;
/// A package's vtable slot `+0x148` (actor, 0; its `AL` tested).
const PACKAGE_SLOT_148: u32 = 0x148;
/// `00726070` (`thiscall` on a list node, plain `RET`): the next node.
const LIST_NEXT_NODE: u32 = 0x0072_6070;
/// `0063f7b0` (`thiscall` on a list, plain `RET`): removes its head item.
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;

// Translated from 0090d570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two words; unoptimized code; `AL` is 0 on return): an
/// actor unlocking a door. With an actor whose slot `+0x22c` (0) answers
/// false and a reference whose base form has type 0x1c: when the actor is
/// an owner (`IsAnOwner(actor, has teleport data)`) and the reference is
/// locked (`GetLock`), it is unlocked and its teleport data (when there is
/// some) made public (`SetConnectedCellsPublic(reference, 1)`).
pub fn fn_0090d570(e: &mut Engine, door: Ptr, actor: Ptr) -> bool {
    if actor.addr() == 0 || e.vcall(actor.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        return false;
    }
    if door.addr() == 0 {
        return false;
    }
    let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![door]).u32();
    if e.call(FORM_GET_TYPE, &args![base_form]).u32() != DOOR_FORM_TYPE {
        return false;
    }
    let teleport = e.call(REFERENCE_GET_TELEPORT_DATA, &args![door]).u32();
    let has_teleport = u32::from(teleport != 0);
    if e.call(REFERENCE_IS_AN_OWNER, &args![door, actor, has_teleport])
        .bool()
        && e.call(REFERENCE_GET_LOCK, &args![door]).u32() != 0
    {
        e.call(REFERENCE_UNLOCK, &args![door]);
        if teleport != 0 {
            e.call(
                TELEPORT_SET_CONNECTED_CELLS_PUBLIC,
                &args![teleport, door, 1u32],
            );
        }
    }
    false
}

// Translated from 0090d480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, two words; unoptimized code; `AL` is 0 on return): an
/// actor locking a door. With an actor whose slot `+0x22c` (0) answers
/// false and a reference whose base form has type 0x1c: the owner flag is
/// set when the reference has teleport data and either that leads to a
/// world space (`GetTeleportWorldSpace`), the actor's `008d6f30` is null or
/// `00425fd0` on it fails. When the actor is an owner (`IsAnOwner`) and the
/// reference has a lock it is locked (`Lock`) and its teleport data (asked
/// again) made private (`SetConnectedCellsPublic(reference, 0)`).
pub fn fn_0090d480(e: &mut Engine, door: Ptr, actor: Ptr) -> bool {
    if actor.addr() == 0 || e.vcall(actor.addr(), ACTOR_SLOT_22C, &args![0u32]).bool() {
        return false;
    }
    if door.addr() == 0 {
        return false;
    }
    let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![door]).u32();
    if e.call(FORM_GET_TYPE, &args![base_form]).u32() != DOOR_FORM_TYPE {
        return false;
    }
    let mut flag = 0u32;
    let teleport = e.call(REFERENCE_GET_TELEPORT_DATA, &args![door]).u32();
    if teleport != 0 {
        flag = if e.call(TELEPORT_GET_WORLD_SPACE, &args![teleport]).u32() != 0
            || e.call(ACTOR_GET_008D6F30, &args![actor]).u32() == 0
        {
            1
        } else {
            let object = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
            u32::from(!e.call(TEST_00425FD0, &args![object]).bool())
        };
    }
    if e.call(REFERENCE_IS_AN_OWNER, &args![door, actor, flag])
        .bool()
        && e.call(REFERENCE_GET_LOCK, &args![door]).u32() != 0
    {
        e.call(REFERENCE_LOCK, &args![door]);
        let teleport = e.call(REFERENCE_GET_TELEPORT_DATA, &args![door]).u32();
        if teleport != 0 {
            e.call(
                TELEPORT_SET_CONNECTED_CELLS_PUBLIC,
                &args![teleport, door, 0u32],
            );
        }
    }
    false
}

// Translated from 009121e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessPatrol` (Xbox PDB; `RET 4`; nothing in `EAX`):
/// without running instance data (`008d0430`) the running procedure index
/// becomes -1 (vtable `+0x284`); for an actor whose slot `+0x214` is
/// non-zero its slot `+0x418` runs instead. Otherwise the time since the
/// last update (`GetTimeDelta`, stored as a `float`) is spent while it is
/// above 0.0 (an x87 `FCOMP`), by the patrol data's state (+4): 0
/// `ProcessPatrolInit` (on failure the time becomes 0.0; on success it is
/// kept and the loop goes on), 1 the travel step `009125b0`, 2
/// `ProcessPatrolAtRef` (each returns the time left), any other 0.0.
pub fn low_process_process_patrol(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr) {
    let data = e.call(PROCESS_GET_INSTANCE_DATA, &args![this]).u32();
    if data == 0 {
        e.vcall(
            this.addr(),
            SET_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![u32::MAX],
        );
        return;
    }
    if e.vcall(actor.addr(), ACTOR_SLOT_214, &args![]).u32() != 0 {
        e.vcall(actor.addr(), ACTOR_SLOT_418, &args![]);
        return;
    }
    let mut time = low_process_get_time_delta(e, this, actor.addr()).to_bits();
    let zero: f64 = e.global(ZERO);
    while x87_load32(time) > zero {
        time = match e.mem.u32(data.wrapping_add(4)) {
            0 => {
                if low_process_process_patrol_init(e, this, actor, Ptr::new(data)) {
                    time
                } else {
                    0
                }
            }
            1 => {
                let left = fn_009125b0(
                    e,
                    this,
                    actor,
                    Ptr::new(data),
                    f32::from_bits(x87_quiet_bits(time)),
                );
                x87_store32(f64::from(left))
            }
            2 => {
                let left = low_process_process_patrol_at_ref(
                    e,
                    this,
                    actor,
                    Ptr::new(data),
                    f32::from_bits(x87_quiet_bits(time)),
                );
                x87_store32(f64::from(left))
            }
            _ => 0,
        };
    }
}

// Translated from 00912990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessPatrolAtRef` (Xbox PDB; `RET 0xc`): waiting at the
/// patrol's current reference. Without one the running procedure index
/// becomes -1 and it returns 0.0. The wait is the float of the reference's
/// extra `0041fe90` (0.0 without it) less the time already waited (`data +
/// 8`), at least 0 (`00404010(0.0, wait)`); the time left is `time` minus
/// that. With time left (above 0.0) it moves to the next reference
/// (`SetNextReferenceForPatrol(1)`; on failure the index moves by 1 and it
/// returns 0.0); the time left becomes 0.0 when the next reference is the
/// same one or closer than 0.001 to it (`004578c0`, `004a7290` on a stack
/// copy of its position, slot `+0x1f4`), and the travel to it starts
/// (`ProcessPatrolStartTravelToNextRef`). Without time left the whole
/// `time` is added to the time waited. Returns the time left.
pub fn low_process_process_patrol_at_ref(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    data: Ptr,
    time: f32,
) -> f32 {
    let reference = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
    if reference == 0 {
        e.vcall(
            this.addr(),
            SET_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![u32::MAX],
        );
        return 0.0;
    }
    let waited_at = data.addr().wrapping_add(8);
    let mut wait = 0u32;
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![reference]).u32();
    if e.call(EXTRA_GET_0041FE90, &args![extra]).u32() != 0 {
        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![reference]).u32();
        let object = e.call(EXTRA_GET_0041FE90, &args![extra]).u32();
        wait = x87_store32(e.call(EXTRA_WAIT_TIME_006A7F50, &args![object]).f64());
    }
    let remaining = x87_store32(x87_sub(x87_load32(wait), x87_m32(e.mem.u32(waited_at))));
    let larger = e
        .call(LARGER_OF_00404010, &args![0u32, x87_quiet_bits(remaining)])
        .f64();
    wait = x87_store32(larger);
    let mut left = x87_store32(x87_sub(x87_load32(time.to_bits()), x87_m32(wait)));
    let zero: f64 = e.global(ZERO);
    if x87_load32(left) > zero {
        if !e.call(PATROL_SET_NEXT_REFERENCE, &args![data, 1u32]).bool() {
            e.vcall(
                this.addr(),
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, 1u32],
            );
            return 0.0;
        }
        if e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32() == reference {
            left = 0;
        } else {
            let next = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
            let position = e.vcall(next, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
            let near = e.with_stack(12, |e, point| {
                for offset in [0u32, 4, 8] {
                    let word = e.mem.u32(position.wrapping_add(offset));
                    e.mem.set_u32(point.addr() + offset, word);
                }
                let other = e.vcall(reference, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
                e.call(POINT_SUBTRACT_004578C0, &args![point, other]);
                let length = e.call(POINT_LENGTH_004A7290, &args![point]).f64();
                let near: f64 = e.global(PATROL_NEAR);
                length < near
            });
            if near {
                left = 0;
            }
        }
        low_process_process_patrol_start_travel_to_next_ref(e, this, actor, data);
    } else {
        let waited = x87_store32(x87_add(
            x87_load32(e.mem.u32(waited_at)),
            x87_m32(time.to_bits()),
        ));
        e.mem.set_u32(waited_at, waited);
    }
    f32::from_bits(left)
}

// Translated from 00910c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessAmbushWait` (Xbox PDB; `RET 4`; nothing in `EAX`):
/// for the running package (`+0x27c`). Without a target, or with one
/// `00440da0` holds for, the running index moves by 2. A target `00440d80`
/// holds for marks the package never to run for it (`SetNeverRun(target,
/// 1)`, after asking `00440d80` again) and moves the index by 2. A target
/// whose slot `+0x22c` (1) holds while there is no acquire object also marks
/// the package. Otherwise, when the package's slot `+0x13c` (actor, 0,
/// -1.0, 0) fails the index moves by -1; else, unless the actor's distance
/// to the target is exactly `FLT_MAX`, the package's slot `+0x148` (actor,
/// 0) moving the index by 1 when it holds.
pub fn low_process_process_ambush_wait(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr) {
    let base = this.addr();
    let package = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
    let target = e.mem.u32(base.wrapping_add(TARGET));
    if target == 0 || e.call(REFERENCE_TEST_00440DA0, &args![target]).bool() {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 2u32],
        );
        return;
    }
    let target = e.mem.u32(base.wrapping_add(TARGET));
    if e.call(REFERENCE_TEST_00440D80, &args![target]).bool() {
        let target = e.mem.u32(base.wrapping_add(TARGET));
        if e.call(REFERENCE_TEST_00440D80, &args![target]).bool() {
            let target = e.mem.u32(base.wrapping_add(TARGET));
            e.call(PACKAGE_SET_NEVER_RUN, &args![package, target, 1u32]);
        }
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 2u32],
        );
        return;
    }
    let target = e.mem.u32(base.wrapping_add(TARGET));
    if e.vcall(target, ACTOR_SLOT_22C, &args![1u32]).bool()
        && e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT)) == 0
    {
        let target = e.mem.u32(base.wrapping_add(TARGET));
        e.call(PACKAGE_SET_NEVER_RUN, &args![package, target, 1u32]);
        return;
    }
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    if !e
        .vcall(
            package,
            PACKAGE_SLOT_13C,
            &args![actor, 0u32, minus_one, 0u32],
        )
        .bool()
    {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, u32::MAX],
        );
        return;
    }
    let target = e.mem.u32(base.wrapping_add(TARGET));
    let distance = e
        .call(REFERENCE_GET_DISTANCE, &args![actor, target, 0u32, 0u32])
        .f64();
    let distance = x87_store32(distance);
    let limit: f64 = e.global(FLOAT_MAX);
    if x87_load32(distance) == limit {
        return;
    }
    if e.vcall(package, PACKAGE_SLOT_148, &args![actor, 0u32])
        .bool()
    {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 1u32],
        );
    }
}

/// Frees the acquire object in the field at `field` (and clears the field)
/// when it is set and `matches` says so.
fn clear_matching_acquire(
    e: &mut Engine,
    field: u32,
    matches: impl FnOnce(&mut Engine, u32) -> bool,
) {
    let object = e.mem.u32(field);
    if object == 0 || !matches(e, object) {
        return;
    }
    let object = e.mem.u32(field);
    if object != 0 {
        e.call(DELETE_EMPTY_DESTRUCTOR, &args![object, 1u32]);
    }
    e.mem.set_u32(field, 0);
}

/// `ObjectList` (+0x5c): every item for which `matches` holds is taken off
/// (`00905330` through a stack word holding it) and freed, the walk starting
/// over at the head each time; others are stepped past (`00726070`).
fn clear_matching_objects(e: &mut Engine, this: u32, matches: impl Fn(&mut Engine, u32) -> bool) {
    let list = this.wrapping_add(OBJECT_LIST);
    let mut node = list;
    while node != 0 {
        let first = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        if e.mem.u32(first) == 0 {
            break;
        }
        let head = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        let item = e.mem.u32(head);
        if matches(e, item) {
            let removed = e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), item);
                e.call(LIST_REMOVE_ITEM, &args![list, slot]);
                e.mem.u32(slot.addr())
            });
            if removed != 0 {
                e.call(DELETE_EMPTY_DESTRUCTOR, &args![removed, 1u32]);
            }
            node = list;
        } else {
            node = e.call(LIST_NEXT_NODE, &args![node]).u32();
        }
    }
}

/// `GenericLocationsList` (+0x6c): every location for which `matches`
/// holds is taken off: after the head through the node before it
/// (`00905330` on that node, then on to its next node), at the head through
/// `0063f7b0` (the walk stays on the head).
fn clear_matching_locations(e: &mut Engine, this: u32, matches: impl Fn(&mut Engine, u32) -> bool) {
    let mut node = this.wrapping_add(GENERIC_LOCATIONS_LIST);
    let mut previous = 0u32;
    while node != 0 {
        let first = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        if e.mem.u32(first) == 0 {
            break;
        }
        let head = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        let location = e.mem.u32(head);
        if matches(e, location) {
            if previous != 0 {
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), location);
                    e.call(LIST_REMOVE_ITEM, &args![previous, slot]);
                });
                node = e.call(LIST_NEXT_NODE, &args![previous]).u32();
            } else {
                e.call(LIST_REMOVE_HEAD, &args![node]);
            }
        } else {
            previous = node;
            node = e.call(LIST_NEXT_NODE, &args![node]).u32();
        }
    }
}

// Translated from 0090cc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ClearObjectFromAcquireList` (Xbox PDB; `RET 4`): drops a
/// reference from the acquire state: the acquire object (+0x64) and the
/// saved one (+0x68) when their reference (first word) is `reference`
/// (freed, `007b3fa0`), every `ObjectList` item for it, `pGenericLocation`
/// when it is it, and every `GenericLocationsList` entry that is it.
pub fn low_process_clear_object_from_acquire_list(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    reference: u32,
) {
    let base = this.addr();
    clear_matching_acquire(e, base.wrapping_add(ACQUIRE_OBJECT), |e, object| {
        e.mem.u32(object) == reference
    });
    clear_matching_acquire(e, base.wrapping_add(SAVED_ACQUIRE_OBJECT), |e, object| {
        e.mem.u32(object) == reference
    });
    clear_matching_objects(e, base, |e, item| e.mem.u32(item) == reference);
    if e.mem.u32(base.wrapping_add(GENERIC_LOCATION)) == reference {
        e.mem.set_u32(base.wrapping_add(GENERIC_LOCATION), 0);
    }
    clear_matching_locations(e, base, |_, location| location == reference);
}

// Translated from 0090cdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ClearObjectFromAcquireList_ov2` (Xbox PDB; `RET 4`): as
/// `ClearObjectFromAcquireList`, comparing the word at `+0xc` (`0084e3a0`)
/// of each reference with `form` (`pGenericLocation` only when set).
pub fn low_process_clear_object_from_acquire_list_ov2(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    form: u32,
) {
    let base = this.addr();
    let matches =
        |e: &mut Engine, reference: u32| e.call(GET_WORD_0C, &args![reference]).u32() == form;
    clear_matching_acquire(e, base.wrapping_add(ACQUIRE_OBJECT), |e, object| {
        let reference = e.mem.u32(object);
        matches(e, reference)
    });
    clear_matching_acquire(e, base.wrapping_add(SAVED_ACQUIRE_OBJECT), |e, object| {
        let reference = e.mem.u32(object);
        matches(e, reference)
    });
    clear_matching_objects(e, base, |e, item| {
        let reference = e.mem.u32(item);
        matches(e, reference)
    });
    let location = e.mem.u32(base.wrapping_add(GENERIC_LOCATION));
    if location != 0 && matches(e, location) {
        e.mem.set_u32(base.wrapping_add(GENERIC_LOCATION), 0);
    }
    clear_matching_locations(e, base, matches);
}

/// The player (`011dea3c`, `PlayerCharacter*`) and
/// `PlayerCharacter::IsSleepingorResting` (engine map, `0094df60`).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_IS_SLEEPING_OR_RESTING: u32 = 0x0094_df60;
/// The `TES` singleton (`011dea10`) and `TES::IsCellLoaded` (engine map,
/// `004511e0`, `RET 8`: cell, 0).
const TES: u32 = 0x011d_ea10;
const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
/// The actor's vtable slot `+0x1d0` (no words; its `EAX` tested).
const ACTOR_SLOT_1D0: u32 = 0x1d0;
/// `006e25d0` (no words; ST0): the `float` at `0119e4bc`.
const PATROL_TRAVEL_TIME: u32 = 0x006e_25d0;
/// `00676280` (`thiscall` on a package, `RET 4`): an unsigned count
/// (loaded as a 64-bit integer with a zero high word).
const PACKAGE_GET_00676280: u32 = 0x0067_6280;
/// `PathingRequest::PathingRequest` (engine map, `006e2420`) and its
/// destructor (`006e2620`), on a 0xb4-byte stack request; the request's
/// setters `008e7c50` and `008e7c20` (`RET 4`, a flag and a `float`).
const PATHING_REQUEST_CONSTRUCT: u32 = 0x006e_2420;
const PATHING_REQUEST_DESTRUCT: u32 = 0x006e_2620;
const PATHING_REQUEST_SIZE: u32 = 0xb4;
const PATHING_REQUEST_SET_008E7C50: u32 = 0x008e_7c50;
const PATHING_REQUEST_SET_008E7C20: u32 = 0x008e_7c20;
/// `Actor::BuildRequest` (engine map, `008b3880`, `RET 0x18`) and
/// `Actor::SetPathfindingGoal` (`008b3630`, `RET 4`).
const ACTOR_BUILD_REQUEST: u32 = 0x008b_3880;
const ACTOR_SET_PATHFINDING_GOAL: u32 = 0x008b_3630;
/// The base-form type (0x30) and the form at `011ca248` for which the
/// travel request also gets the angle (`008e7c20`).
const ANGLED_FORM_TYPE: u32 = 0x30;
const ANGLED_FORM: u32 = 0x011c_a248;
/// `008b3b90` and `Actor::IsPathingComplete` (`008b3bb0`): `thiscall` on
/// the actor, plain `RET`.
const ACTOR_TEST_008B3B90: u32 = 0x008b_3b90;
const ACTOR_IS_PATHING_COMPLETE: u32 = 0x008b_3bb0;
/// `0067a480` (`thiscall` on a package, plain `RET`; its `AL`).
const PACKAGE_TEST_0067A480: u32 = 0x0067_a480;
/// `64.0` (`float`): the distance beyond which the patrol's move mode is
/// computed.
const PATROL_MOVE_DISTANCE: u32 = 0x0101_e6ec;
/// `TESPackage::GetPackageRadiusActorToLocation` (engine map, `00678670`,
/// `RET 8`; ST0).
const PACKAGE_RADIUS_ACTOR_TO_LOCATION: u32 = 0x0067_8670;
/// Process vtable slots `+0x34c` `CalculateMoveMode` (6 words) and `+0x350`
/// `SetActorsAnimation` (3 words) (Xbox PDB `BaseProcess` names).
const CALCULATE_MOVE_MODE_SLOT: u32 = 0x34c;
const SET_ACTORS_ANIMATION_SLOT: u32 = 0x350;
/// `Actor::GetCurrentSpeed` (engine map, `008a0b10`; ST0).
const ACTOR_GET_CURRENT_SPEED: u32 = 0x008a_0b10;
/// `00457990` (`thiscall` on a point; ST0): its length (by its use).
const POINT_LENGTH_00457990: u32 = 0x0045_7990;
/// `PathingLocation::PathingLocation_ov3` (engine map, `006dcd70`, `RET 4`)
/// and the destructor `004ff7e0`, on 0x28-byte stack locations;
/// `Pathing::ComputeTeleportDoorPathLength` (`006d4eb0`, `cdecl`, 5 words;
/// ST0) with the limit 4096.0 (`01017a3c`).
const PATHING_LOCATION_CONSTRUCT: u32 = 0x006d_cd70;
const PATHING_LOCATION_DESTRUCT: u32 = 0x004f_f7e0;
const PATHING_LOCATION_SIZE: u32 = 0x28;
const TELEPORT_DOOR_PATH_LENGTH: u32 = 0x006d_4eb0;
const DOOR_PATH_LIMIT: u32 = 0x0101_7a3c;
/// `008ad910` (`thiscall` on the actor, `RET 0xc`: the reference, 1, 0).
const ACTOR_MOVE_TO_008AD910: u32 = 0x008a_d910;

/// Whether neither cell (when set) is loaded (`TES::IsCellLoaded(cell, 0)`).
fn cells_unloaded(e: &mut Engine, first: u32, second: u32) -> bool {
    for cell in [first, second] {
        if cell != 0 {
            let tes = e.global::<u32>(TES);
            if e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool() {
                return false;
            }
        }
    }
    true
}

// Translated from 00912340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00912340 - the compiler's exception frame (FS:[0] chain, handler 00f16deb; it destroys the stack PathingRequest on unwinding) is not translated
/// `LowProcess::ProcessPatrolStartTravelToNextRef` (Xbox PDB; `RET 8`):
/// without a current patrol reference the running procedure index becomes
/// -1. Otherwise the reference becomes the target (vtable `+0x12c`). Unless
/// the player sleeps or rests, or (with the actor's slot `+0x1d0` 0)
/// neither the actor's nor the reference's cell (`008d6f30`) is loaded,
/// the actor gets a pathing request to the reference: a stack
/// `PathingRequest`, its time the larger of `006e25d0()` and the running
/// package's count `00676280(actor)`, `BuildRequest(request, position (slot
/// +0x1f4), cell, world space, time, 0)`, `008e7c50(0)`, the reference's
/// angle (`008e7c20` with the float at +8 of `00430830`) for base forms of
/// type 0x30 or the form `011ca248`, `SetPathfindingGoal`, the request's
/// destructor. Then the patrol data's state (+4) becomes 1.
pub fn low_process_process_patrol_start_travel_to_next_ref(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    data: Ptr,
) {
    let base = this.addr();
    let reference = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
    if reference == 0 {
        e.vcall(base, SET_PROCEDURE_INDEX_RUNNING_SLOT, &args![u32::MAX]);
        return;
    }
    e.vcall(base, SET_TARGET_SLOT, &args![reference]);
    let player = e.global::<u32>(PLAYER);
    let skip = if e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool() {
        true
    } else if e.vcall(actor.addr(), ACTOR_SLOT_1D0, &args![]).u32() != 0 {
        false
    } else {
        let actor_cell = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
        let reference_cell = e.call(ACTOR_GET_008D6F30, &args![reference]).u32();
        cells_unloaded(e, actor_cell, reference_cell)
    };
    if !skip {
        let mut time = x87_store32(e.call(PATROL_TRAVEL_TIME, &args![]).f64());
        let package = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
        if package != 0 {
            let count = e.call(PACKAGE_GET_00676280, &args![package, actor]).u32();
            let count = x87_store32(f64::from(count));
            let larger = e
                .call(LARGER_OF_00404010, &args![x87_quiet_bits(time), count])
                .f64();
            time = x87_store32(larger);
        }
        e.with_stack(PATHING_REQUEST_SIZE, |e, request| {
            e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
            let world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).u32();
            let cell = e.call(ACTOR_GET_008D6F30, &args![reference]).u32();
            let position = e.vcall(reference, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
            e.call(
                ACTOR_BUILD_REQUEST,
                &args![
                    actor,
                    request,
                    position,
                    cell,
                    world_space,
                    x87_quiet_bits(time),
                    0u32
                ],
            );
            e.call(PATHING_REQUEST_SET_008E7C50, &args![request, 0u32]);
            let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
            if e.call(FORM_GET_TYPE, &args![base_form]).u32() == ANGLED_FORM_TYPE
                || base_form == e.global::<u32>(ANGLED_FORM)
            {
                let object = e.call(ACTOR_GET_00430830, &args![reference]).u32();
                let angle = x87_quiet_bits(e.mem.u32(object.wrapping_add(8)));
                e.call(PATHING_REQUEST_SET_008E7C20, &args![request, angle]);
            }
            e.call(ACTOR_SET_PATHFINDING_GOAL, &args![actor, request]);
            e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
        });
    }
    e.mem.set_u32(data.addr().wrapping_add(4), 1);
}

// Translated from 009125b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009125b0 - the compiler's exception frame (FS:[0] chain, handler 00f16e23; it destroys the stack PathingLocations on unwinding) is not translated
/// The patrol's travel step (unnamed in the engine map; `lowprocess.cpp`,
/// `RET 0xc`; `ProcessPatrol`'s state 1): returns the time left of `time`.
/// Without a current patrol reference the running index becomes -1 and it
/// returns 0.0. The travel is skipped ("fast") when the player sleeps or
/// rests or neither cell is loaded. When not fast and `008b3b90` holds for
/// the actor, the travel restarts (`ProcessPatrolStartTravelToNextRef`) and
/// all the time is left. Otherwise the move mode is 0x101, or for a
/// package `0067a480` holds for with the reference farther than 64.0,
/// `CalculateMoveMode(actor, distance, radius, 2 * radius, 0, 0)` (radius
/// `GetPackageRadiusActorToLocation`); `SetActorsAnimation(actor, mode,
/// farther)`. When fast the actor arrives: the distance (straight when both
/// share a cell, else the teleport-door path length, limit 4096.0) divided
/// by the speed (at least 1.0) is taken off the time, the actor is placed
/// (`008ad910(reference, 1, 0)`) and `ProcessPatrolStartAtRefBehavior`
/// runs. When not fast it runs once pathing is complete, else no time is
/// left.
pub fn fn_009125b0(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr, data: Ptr, time: f32) -> f32 {
    let base = this.addr();
    let reference = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
    if reference == 0 {
        e.vcall(base, SET_PROCEDURE_INDEX_RUNNING_SLOT, &args![u32::MAX]);
        return 0.0;
    }
    let mut left = x87_quiet_bits(time.to_bits());
    let actor_cell = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
    let reference_cell = e.call(ACTOR_GET_008D6F30, &args![reference]).u32();
    let same_cell = actor_cell != 0 && actor_cell == reference_cell;
    let player = e.global::<u32>(PLAYER);
    let fast = if e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool() {
        true
    } else {
        cells_unloaded(e, actor_cell, reference_cell)
    };
    if !fast && e.call(ACTOR_TEST_008B3B90, &args![actor]).bool() {
        low_process_process_patrol_start_travel_to_next_ref(e, this, actor, data);
        return f32::from_bits(left);
    }
    let mut mode = 0x101u32;
    let package = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
    let mut farther = false;
    if e.call(PACKAGE_TEST_0067A480, &args![package]).bool() {
        let distance = e
            .call(REFERENCE_GET_DISTANCE, &args![actor, reference, 0u32, 0u32])
            .f64();
        let distance = x87_store32(distance);
        let limit = x87_quiet_bits(e.global::<u32>(PATROL_MOVE_DISTANCE));
        farther = x87_load32(limit) < x87_load32(distance);
        if farther {
            let radius = e
                .call(
                    PACKAGE_RADIUS_ACTOR_TO_LOCATION,
                    &args![package, actor, 0u32],
                )
                .f64();
            let radius = x87_store32(radius);
            let twice = x87_store32(x87_add(x87_load32(radius), x87_load32(radius)));
            mode = e
                .vcall(
                    base,
                    CALCULATE_MOVE_MODE_SLOT,
                    &args![
                        actor,
                        distance,
                        x87_quiet_bits(radius),
                        x87_quiet_bits(twice),
                        0u32,
                        0u32
                    ],
                )
                .u32();
        }
    }
    e.vcall(
        base,
        SET_ACTORS_ANIMATION_SLOT,
        &args![actor, mode, u32::from(farther)],
    );
    if fast {
        let speed = x87_store32(e.call(ACTOR_GET_CURRENT_SPEED, &args![actor]).f64());
        let one = 1.0f32.to_bits();
        let speed = x87_store32(e.call(LARGER_OF_00404010, &args![speed, one]).f64());
        let distance = if same_cell {
            let position = e.vcall(reference, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
            e.with_stack(12, |e, point| {
                for offset in [0u32, 4, 8] {
                    let word = e.mem.u32(position.wrapping_add(offset));
                    e.mem.set_u32(point.addr() + offset, word);
                }
                let other = e
                    .vcall(actor.addr(), ACTOR_GET_LOCATION_SLOT, &args![])
                    .u32();
                e.call(POINT_SUBTRACT_004578C0, &args![point, other]);
                x87_store32(e.call(POINT_LENGTH_00457990, &args![point]).f64())
            })
        } else {
            e.with_stack(2 * PATHING_LOCATION_SIZE, |e, locations| {
                let from = locations.addr();
                let to = from + PATHING_LOCATION_SIZE;
                e.call(PATHING_LOCATION_CONSTRUCT, &args![from, actor]);
                e.call(PATHING_LOCATION_CONSTRUCT, &args![to, reference]);
                let limit = x87_quiet_bits(e.global::<u32>(DOOR_PATH_LIMIT));
                let length = e
                    .call(
                        TELEPORT_DOOR_PATH_LENGTH,
                        &args![from, to, limit, 0u32, 0u32],
                    )
                    .f64();
                let length = x87_store32(length);
                e.call(PATHING_LOCATION_DESTRUCT, &args![to]);
                e.call(PATHING_LOCATION_DESTRUCT, &args![from]);
                length
            })
        };
        let spent = x87_div(x87_load32(distance), x87_m32(speed));
        left = x87_store32(x87_sub(x87_m32(left), spent));
        e.call(ACTOR_MOVE_TO_008AD910, &args![actor, reference, 1u32, 0u32]);
        low_process_process_patrol_start_at_ref_behavior(e, this, actor.addr(), data);
    } else if e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
        low_process_process_patrol_start_at_ref_behavior(e, this, actor.addr(), data);
    } else {
        left = 0;
    }
    f32::from_bits(left)
}

/// More process vtable slots (Xbox PDB `BaseProcess` names).
/// `+0x11c` `GetCurrentActionComplete`, `+0x234` `ClearCurrentPackage`,
/// `+0x4e8` `IsTargetActivated`, `+0x524` `ClearCurrentDataforProcess`.
const GET_CURRENT_ACTION_COMPLETE_SLOT: u32 = 0x11c;
const CLEAR_CURRENT_PACKAGE_SLOT: u32 = 0x234;
const IS_TARGET_ACTIVATED_SLOT: u32 = 0x4e8;
const CLEAR_CURRENT_DATA_SLOT: u32 = 0x524;
/// The actor's vtable slots `+0x48` (`00484b60` in the `Character` table,
/// one word: here 0x80000000) and `+0x100` (`008d0360`, no words; its `AL`).
const ACTOR_SLOT_48: u32 = 0x48;
const ACTOR_SLOT_100: u32 = 0x100;
/// `TESPackage::GetIsCreated` (engine map, `00674d40`).
const PACKAGE_GET_IS_CREATED: u32 = 0x0067_4d40;
/// `ExtraDataList::SetPackageExtra` (engine map, `0041c930`, `RET 0x18`) and
/// `SetActorPackageData` (`004211d0`, `RET 4`).
const EXTRA_SET_PACKAGE: u32 = 0x0041_c930;
const EXTRA_SET_ACTOR_PACKAGE_DATA: u32 = 0x0042_11d0;
/// `008d8520` (`thiscall`, plain `RET`; the word at `+0x68`, engine map
/// `MiddleHighProcess::GetSavedAcquireObject` by folding): the actor's
/// process here.
const ACTOR_GET_PROCESS: u32 = 0x008d_8520;
/// `ActorPackage::NullInstanceData` (engine map, `008c6060`) and
/// `ActorPackage::InitFromPackage` (`008c6080`, `RET 8`).
const ACTOR_PACKAGE_NULL_INSTANCE_DATA: u32 = 0x008c_6060;
const ACTOR_PACKAGE_INIT_FROM_PACKAGE: u32 = 0x008c_6080;
/// `006ecd40` (`thiscall`, `RET 4`; engine map
/// `CombatProcedureAttackMelee::Initialize` by folding): stores its one
/// word (on the `ActorPackage`: the instance data).
const SET_INSTANCE_DATA_006ECD40: u32 = 0x006e_cd40;
/// The extra data list's package getters (`thiscall`, plain `RET`):
/// `GetPackageExtra` (`0041cb10`), `GetPackageExtraIndex` (`0041cb40`),
/// `GetPackageExtraTarget` (`0041cb70`), the three flags `0041cba0`,
/// `0041cc30`, `0041cc00` (bytes), `RemovePackageExtra` (`0041cc60`),
/// `GetActorPackageData` (`004212b0`), `RemoveActorPackageData`
/// (`004212e0`).
const EXTRA_GET_PACKAGE: u32 = 0x0041_cb10;
const EXTRA_GET_PACKAGE_INDEX: u32 = 0x0041_cb40;
const EXTRA_GET_PACKAGE_TARGET: u32 = 0x0041_cb70;
const EXTRA_GET_ACTION_COMPLETE: u32 = 0x0041_cba0;
const EXTRA_GET_DONE_ONCE: u32 = 0x0041_cc30;
const EXTRA_GET_TARGET_ACTIVATED: u32 = 0x0041_cc00;
const EXTRA_REMOVE_PACKAGE: u32 = 0x0041_cc60;
const EXTRA_GET_ACTOR_PACKAGE_DATA: u32 = 0x0042_12b0;
const EXTRA_REMOVE_ACTOR_PACKAGE_DATA: u32 = 0x0042_12e0;
/// `004534f0` (`thiscall` on the actor, `RET 4`; empty in `LowProcess`'s
/// table): given the current package.
const ACTOR_TAKE_PACKAGE_004534F0: u32 = 0x0045_34f0;
/// `00559450` (`thiscall`, plain `RET`): the `ActorPackage`'s package.
const ACTOR_PACKAGE_GET_PACKAGE: u32 = 0x0055_9450;
/// `BaseProcess::SaveGame_ov2` (engine map, `008d0f30`, `RET 4`).
const BASE_PROCESS_SAVE_GAME: u32 = 0x008d_0f30;
/// The save buffer's writers (`thiscall`): `00865e50` (`RET 0xc`: address,
/// size, 0), `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`, `RET 8`: form,
/// 0), `StartVariableSizedValue` (`00865f20`) and
/// `SaveVariableSizedValue_ov2` (`00865ff0`, `RET 8`: count, mark).
const SAVE_DATA: u32 = 0x0086_5e50;
const SAVE_FORM_ID: u32 = 0x0086_5df0;
const SAVE_START_VARIABLE_SIZE: u32 = 0x0086_5f20;
const SAVE_VARIABLE_SIZE: u32 = 0x0086_5ff0;
/// `CombatTimer::SaveGame` (engine map, `009a5eb0`, `RET 4`) and
/// `ModifierList::SaveGame_ov2` (`00937b50`, `RET 4`).
const COMBAT_TIMER_SAVE: u32 = 0x009a_5eb0;
const MODIFIER_LIST_SAVE: u32 = 0x0093_7b50;
/// `00428110` (`thiscall` on the save buffer, `RET 4`: an out word) and
/// `004280f0` (`thiscall` on what it returns, `RET 4`: a mask; its `AL`):
/// whether the modifier list is saved (mask 0x200000).
const SAVE_FLAGS_00428110: u32 = 0x0042_8110;
const FLAGS_HAVE_004280F0: u32 = 0x0042_80f0;
const SAVE_MODIFIERS_FLAG: u32 = 0x0020_0000;

// Translated from 009130f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SavePackageToExtraData` (Xbox PDB; `RET 4`): with a current
/// package that was not created at run time (`TESPackage::GetIsCreated`),
/// stores it in the actor's extra data (`SetPackageExtra(package,
/// procedure index, target, action complete, target activated, done once)`),
/// runs the actor's slot `+0x48` with 0x80000000, and with running instance
/// data stores that too (`SetActorPackageData`) and clears the package's
/// (`NullInstanceData`). On the way it reads `0044ddc0` of the actor's
/// process's running package (its value unused).
pub fn low_process_save_package_to_extra_data(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr) {
    let base = this.addr();
    if e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32() == 0 {
        return;
    }
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    if e.call(PACKAGE_GET_IS_CREATED, &args![package]).bool() {
        return;
    }
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    let done_once = e.vcall(base, IS_PACKAGE_DONE_ONCE_SLOT, &args![]).u8();
    let activated = e.vcall(base, IS_TARGET_ACTIVATED_SLOT, &args![]).u8();
    let complete = e
        .vcall(base, GET_CURRENT_ACTION_COMPLETE_SLOT, &args![])
        .u8();
    let target = e.mem.u32(base.wrapping_add(TARGET));
    let index = e
        .vcall(base, GET_CURRENT_PROCEDURE_INDEX_SLOT, &args![])
        .u32();
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    e.call(
        EXTRA_SET_PACKAGE,
        &args![
            extra,
            package,
            index,
            target,
            u32::from(complete),
            u32::from(activated),
            u32::from(done_once)
        ],
    );
    e.vcall(actor.addr(), ACTOR_SLOT_48, &args![0x8000_0000u32]);
    let data = e.call(PROCESS_GET_INSTANCE_DATA, &args![this]).u32();
    if data == 0 {
        return;
    }
    if e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0 {
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        if e.vcall(process, GET_ACTOR_PACKAGE_RUNNING_SLOT, &args![])
            .u32()
            != 0
        {
            let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
            let running = e
                .vcall(process, GET_ACTOR_PACKAGE_RUNNING_SLOT, &args![])
                .u32();
            e.call(PACKAGE_TARGET_GET_VALUE, &args![running]);
        }
    }
    e.call(EXTRA_SET_ACTOR_PACKAGE_DATA, &args![extra, data]);
    e.call(
        ACTOR_PACKAGE_NULL_INSTANCE_DATA,
        &args![base.wrapping_add(PACKAGE)],
    );
}

// Translated from 00913250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::LoadPackageFromExtraData` (Xbox PDB; `RET 4`). With a
/// package in the actor's extra data: the process's current data is
/// cleared first when the actor's slot `+0x100` holds (vtable `+0x524`),
/// the package and its target become the `ActorPackage`'s
/// (`InitFromPackage`), the actor's `004534f0` gets the current package,
/// the saved procedure index (0 when not below the type's procedure count,
/// unless the type is -1), target, action-complete, done-once and
/// target-activated flags are restored, the package extra is removed, and
/// saved instance data is handed to the package (`006ecd40`) and removed.
/// Without one: the current data cleared as above, the current package
/// cleared (`+0x234`), and the target, action-complete, flag 0x40,
/// done-once and target-activated cleared.
pub fn low_process_load_package_from_extra_data(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr) {
    let base = this.addr();
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    if e.call(EXTRA_GET_PACKAGE, &args![extra]).u32() != 0 {
        if e.vcall(actor.addr(), ACTOR_SLOT_100, &args![]).bool() {
            e.vcall(base, CLEAR_CURRENT_DATA_SLOT, &args![actor]);
        }
        let target = e.call(EXTRA_GET_PACKAGE_TARGET, &args![extra]).u32();
        let package = e.call(EXTRA_GET_PACKAGE, &args![extra]).u32();
        e.call(
            ACTOR_PACKAGE_INIT_FROM_PACKAGE,
            &args![base.wrapping_add(PACKAGE), package, target],
        );
        let current = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
        e.call(ACTOR_TAKE_PACKAGE_004534F0, &args![actor, current]);
        let mut index = e.call(EXTRA_GET_PACKAGE_INDEX, &args![extra]).i32();
        let package = e
            .call(
                ACTOR_PACKAGE_GET_PACKAGE,
                &args![base.wrapping_add(PACKAGE)],
            )
            .u32();
        let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
        if kind != u32::MAX && index >= e.call(PROCEDURE_COUNT, &args![kind]).i32() {
            index = 0;
        }
        e.vcall(base, SET_CURRENT_PROCEDURE_INDEX_SLOT, &args![index]);
        let target = e.call(EXTRA_GET_PACKAGE_TARGET, &args![extra]).u32();
        e.vcall(base, SET_TARGET_SLOT, &args![target]);
        let complete = e.call(EXTRA_GET_ACTION_COMPLETE, &args![extra]).u8();
        e.vcall(
            base,
            SET_CURRENT_ACTION_COMPLETE_SLOT,
            &args![u32::from(complete)],
        );
        let done_once = e.call(EXTRA_GET_DONE_ONCE, &args![extra]).u8();
        e.vcall(
            base,
            SET_PACKAGE_DONE_ONCE_SLOT,
            &args![u32::from(done_once)],
        );
        let activated = e.call(EXTRA_GET_TARGET_ACTIVATED, &args![extra]).u8();
        e.vcall(
            base,
            SET_TARGET_ACTIVATED_SLOT,
            &args![u32::from(activated)],
        );
        e.call(EXTRA_REMOVE_PACKAGE, &args![extra]);
        let data = e.call(EXTRA_GET_ACTOR_PACKAGE_DATA, &args![extra]).u32();
        if data != 0 {
            e.call(
                SET_INSTANCE_DATA_006ECD40,
                &args![base.wrapping_add(PACKAGE), data],
            );
            e.call(EXTRA_REMOVE_ACTOR_PACKAGE_DATA, &args![extra]);
        }
    } else {
        if e.vcall(actor.addr(), ACTOR_SLOT_100, &args![]).bool() {
            e.vcall(base, CLEAR_CURRENT_DATA_SLOT, &args![actor]);
        }
        e.vcall(base, CLEAR_CURRENT_PACKAGE_SLOT, &args![]);
        e.vcall(base, SET_TARGET_SLOT, &args![0u32]);
        e.vcall(base, SET_CURRENT_ACTION_COMPLETE_SLOT, &args![0u32]);
        fn_0090ac20(e, this, 0);
        e.vcall(base, SET_PACKAGE_DONE_ONCE_SLOT, &args![0u32]);
        e.vcall(base, SET_TARGET_ACTIVATED_SLOT, &args![0u32]);
    }
}

// Translated from 00910450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SaveGame_ov2` (Xbox PDB; `RET 4`): the base process
/// (`BaseProcess::SaveGame_ov2`), then `m_uFlags` (1 byte),
/// `fEssentialDownTimer` (4), `pItemBeingUsed` (form id),
/// `iNumberItemsActivate` (4), `fDeathTime` (4), `fTrackedDamage` (4),
/// `bSkippedTimeStampForPathing` (1), `sFormListNumb` (2), the combat delay
/// timer (`CombatTimer::SaveGame`), the target (form id; 0 while there is an
/// acquire object), `pGenericLocation`, `pGenericSecondLocation`,
/// `pTargetFormList` and `pPatrolLocation` (form ids), the non-null entries
/// of `GenericLocationsList` (form ids, counted into a variable-sized value)
/// and, when the save flags have 0x200000 (`00428110`, `004280f0`), the
/// damage modifiers (`ModifierList::SaveGame_ov2`).
pub fn low_process_save_game_ov2(e: &mut Engine, this: Ptr<LowProcess>, buffer: Ptr) {
    let base = this.addr();
    e.call(BASE_PROCESS_SAVE_GAME, &args![this, buffer]);
    for (offset, size) in [(0x30u32, 1u32), (0xa4, 4)] {
        e.call(
            SAVE_DATA,
            &args![buffer, base.wrapping_add(offset), size, 0u32],
        );
    }
    let item = e.mem.u32(base.wrapping_add(0x34));
    e.call(SAVE_FORM_ID, &args![buffer, item, 0u32]);
    for (offset, size) in [(0x58u32, 4u32), (0xa8, 4), (0xac, 4), (0xb0, 1), (0x50, 2)] {
        e.call(
            SAVE_DATA,
            &args![buffer, base.wrapping_add(offset), size, 0u32],
        );
    }
    e.call(
        COMBAT_TIMER_SAVE,
        &args![base.wrapping_add(COMBAT_DELAY_TIMER), buffer],
    );
    let target = if e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT)) == 0 {
        e.mem.u32(base.wrapping_add(TARGET))
    } else {
        0
    };
    e.call(SAVE_FORM_ID, &args![buffer, target, 0u32]);
    for offset in [0x44u32, 0x48, 0x4c, 0x54] {
        let form = e.mem.u32(base.wrapping_add(offset));
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
    let mut count = 0u32;
    let mark = e.call(SAVE_START_VARIABLE_SIZE, &args![buffer]).u32();
    let mut node = base.wrapping_add(GENERIC_LOCATIONS_LIST);
    while node != 0 {
        let head = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        let location = e.mem.u32(head);
        if location != 0 {
            e.call(SAVE_FORM_ID, &args![buffer, location, 0u32]);
            count = count.wrapping_add(1);
        }
        node = e.call(LIST_NEXT_NODE, &args![node]).u32();
    }
    e.call(SAVE_VARIABLE_SIZE, &args![buffer, count, mark]);
    let save_modifiers = e.with_stack(4, |e, out| {
        let flags = e.call(SAVE_FLAGS_00428110, &args![buffer, out]).u32();
        e.call(FLAGS_HAVE_004280F0, &args![flags, SAVE_MODIFIERS_FLAG])
            .bool()
    });
    if save_modifiers {
        e.call(
            MODIFIER_LIST_SAVE,
            &args![base.wrapping_add(DAMAGE_MODIFIERS), buffer],
        );
    }
}

/// More process vtable slots (Xbox PDB `BaseProcess` names; `+0x7f8` is
/// PC-only, `LowProcess::ProcessGuardStartTravelToLocation` in the
/// `LowProcess` table).
const CHECK_FOR_NEW_PACKAGE_SLOT: u32 = 0x24;
const GET_TARGET_SLOT: u32 = 0x128;
const GET_RUN_ONCE_PACKAGE_SLOT: u32 = 0x20c;
const GET_RUN_ONCE_INSTANCE_DATA_SLOT: u32 = 0x270;
const IS_RUNNING_RUN_ONCE_SLOT: u32 = 0x35c;
const PROCESS_GUARD_START_TRAVEL_SLOT: u32 = 0x7f8;
/// The actor's vtable slots `+0x2a8` (`00931620` in the `Character` table,
/// one word: a position) and `+0x4b4` (`0087d750`, no words; its `AL`).
const ACTOR_SLOT_2A8: u32 = 0x2a8;
const ACTOR_SLOT_4B4: u32 = 0x4b4;
/// `008a6290` (`thiscall` on a package, plain `RET`): bit 0x10000 of its
/// flags word at `+0x1c`.
const PACKAGE_TEST_008A6290: u32 = 0x008a_6290;
/// `00962720` (`thiscall` on the player, `RET 4`: the actor) and
/// `00962620` (on the player, plain `RET`): tested by `ProcessFollow`.
const PLAYER_TEST_00962720: u32 = 0x0096_2720;
const PLAYER_COUNT_00962620: u32 = 0x0096_2620;
/// `0043d4d0` (`thiscall`, plain `RET`) on the object at `011cdad0`: the
/// address of its value (`+4`; a shared zero word when the object is null).
const SETTING_VALUE_0043D4D0: u32 = 0x0043_d4d0;
const FOLLOW_LIMIT_SETTING: u32 = 0x011c_dad0;
/// `00674f30` (`thiscall` on a package, `RET 4`).
const PACKAGE_SET_00674F30: u32 = 0x0067_4f30;
/// `TESPackage::GetPackageRadiusActorToRefTarget` (engine map, `006787e0`,
/// `RET 8`; ST0).
const PACKAGE_RADIUS_ACTOR_TO_TARGET: u32 = 0x0067_87e0;
/// `Actor::SetPathfindingGoal_ov2` (engine map, `008b3690`, `RET 0x14`) and
/// `_ov3` (`008b36f0`, `RET 0xc`).
const ACTOR_SET_PATHFINDING_GOAL_TO_LOCATION: u32 = 0x008b_3690;
const ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE: u32 = 0x008b_36f0;
/// The sandbox data's helpers: `009f3af0` (`thiscall`, `RET 8`),
/// `SandBoxActorPackageData::MoveToNextLocation` (`009f3c10`; its `AL`) and
/// `0044edb0` (plain `RET`; engine map
/// `BaseProcess::GetCurrentProcedureIndex` by folding): the current
/// location reference.
const SANDBOX_START_009F3AF0: u32 = 0x009f_3af0;
const SANDBOX_MOVE_TO_NEXT_LOCATION: u32 = 0x009f_3c10;
const SANDBOX_GET_LOCATION: u32 = 0x0044_edb0;
/// `TESObjectREFR::MoveRefToNewSpace` (engine map, `00573800`, `cdecl`,
/// three words).
const REFERENCE_MOVE_TO_NEW_SPACE: u32 = 0x0057_3800;
/// `10.0` (`float`): the least sandbox travel time.
const SANDBOX_LEAST_TIME: u32 = 0x0101_7b78;
/// `TESPackage::ComputeInitialTarget` (engine map, `006759e0`, `RET 4`),
/// `ExtraDataList::GetReferencePointer` (`0041c8d0`) and `AddGuard`
/// (`00421310`, `RET 4`).
const PACKAGE_COMPUTE_INITIAL_TARGET: u32 = 0x0067_59e0;
const EXTRA_GET_REFERENCE_POINTER: u32 = 0x0041_c8d0;
const EXTRA_ADD_GUARD: u32 = 0x0042_1310;
/// `30.0` (`float`): the guard radius without a package target.
const GUARD_DEFAULT_RADIUS: u32 = 0x0101_8f5c;

// Translated from 00909de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessFollow` (Xbox PDB; `RET 0x10`, three words unread;
/// nothing in `EAX`). For a current package with flag 0x10000
/// (`008a6290`): when the player's `00962720(actor)` holds it stops; when
/// the player's `00962620` is below the value of the setting at
/// `011cdad0` the package gets `00674f30(0)` and it goes on, otherwise it
/// stops. Without a target it sets one (vtable `+0x8c`). Without a target
/// still, or with a located package whose slot `+0x13c` (actor, 0, -1.0, 0)
/// holds and whose type is 1, the procedure index moves by 1 and a created
/// package is dropped (`+0x524`, `+0x234`, `CheckforNewPackage(actor, 0)`).
/// Otherwise the actor follows: once pathing is complete and the target
/// is farther than the package radius (`006787e0`; compared only within
/// one world space), `SetPathfindingGoal_ov2(target position, cell, world
/// space, radius, 0)`.
pub fn low_process_process_follow(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) {
    let base = this.addr();
    let current = |e: &mut Engine| e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    if current(e) != 0 {
        let package = current(e);
        if e.call(PACKAGE_TEST_008A6290, &args![package]).bool() {
            let player = e.global::<u32>(PLAYER);
            if e.call(PLAYER_TEST_00962720, &args![player, actor]).bool() {
                return;
            }
            let player = e.global::<u32>(PLAYER);
            let count = e.call(PLAYER_COUNT_00962620, &args![player]).i32();
            let value = e
                .call(SETTING_VALUE_0043D4D0, &args![FOLLOW_LIMIT_SETTING])
                .u32();
            if count >= e.mem.i32(value) {
                return;
            }
            let package = current(e);
            e.call(PACKAGE_SET_00674F30, &args![package, 0u32]);
        }
    }
    if e.mem.u32(base.wrapping_add(TARGET)) == 0 {
        e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
    }
    let finished = if e.mem.u32(base.wrapping_add(TARGET)) == 0 {
        true
    } else {
        let package = current(e);
        if e.call(GET_0055B980, &args![package]).u32() == 0 {
            false
        } else {
            let package = current(e);
            let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
            if !e
                .vcall(
                    package,
                    PACKAGE_SLOT_13C,
                    &args![actor, 0u32, minus_one, 0u32],
                )
                .bool()
            {
                false
            } else {
                let package = current(e);
                e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 1
            }
        }
    };
    if finished {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 1u32],
        );
        let package = current(e);
        if e.call(PACKAGE_GET_IS_CREATED, &args![package]).bool() {
            e.vcall(base, CLEAR_CURRENT_DATA_SLOT, &args![actor]);
            e.vcall(base, CLEAR_CURRENT_PACKAGE_SLOT, &args![]);
            e.vcall(base, CHECK_FOR_NEW_PACKAGE_SLOT, &args![actor, 0u32]);
        }
        return;
    }
    // The time delta is computed and stored, its value unused.
    low_process_get_time_delta(e, this, actor.addr());
    let package = current(e);
    let radius = e
        .call(PACKAGE_RADIUS_ACTOR_TO_TARGET, &args![package, actor, 0u32])
        .f64();
    let radius = x87_store32(radius);
    let actor_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
    let target = e.mem.u32(base.wrapping_add(TARGET));
    let target_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![target]).u32();
    let same_space = actor_space == target_space;
    if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
        || e.mem.u32(base.wrapping_add(TARGET)) == 0
    {
        return;
    }
    if same_space {
        let target = e.mem.u32(base.wrapping_add(TARGET));
        let distance = e
            .call(REFERENCE_GET_DISTANCE, &args![actor, target, 0u32, 0u32])
            .f64();
        if !(x87_load32(radius) < distance) {
            return;
        }
    }
    let target = e.mem.u32(base.wrapping_add(TARGET));
    let world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![target]).u32();
    let target = e.mem.u32(base.wrapping_add(TARGET));
    let cell = e.call(ACTOR_GET_008D6F30, &args![target]).u32();
    let target = e.mem.u32(base.wrapping_add(TARGET));
    let position = e.vcall(target, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
    e.call(
        ACTOR_SET_PATHFINDING_GOAL_TO_LOCATION,
        &args![
            actor,
            position,
            cell,
            world_space,
            x87_quiet_bits(radius),
            0u32
        ],
    );
}

// Translated from 00911f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessSandBox` (Xbox PDB; `RET 4`; the state `switch`
/// through the table at `00912140` translated as a `match`). Without
/// running instance data the running index moves by 1. By the data's state
/// (+4): 0 sets state 2 and starts the sandbox at the package location
/// (`009f3af0(actor, location)`); 1 and 2 set state 3 and go on as 3; 3,
/// once pathing is complete, moves to the next location
/// (`MoveToNextLocation`) and, for a new location other than the old one:
/// when neither location's cell is loaded the actor is placed there (slot
/// `+0x2a8` with its position, and `MoveRefToNewSpace` to its cell and
/// world space when the cells differ); otherwise it paths there
/// (`SetPathfindingGoal_ov3(location, time, 0)`, the time the package's
/// count `00676280`, at least 10.0). Other states do nothing.
pub fn low_process_process_sandbox(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr) {
    let base = this.addr();
    let data = e.call(PROCESS_GET_INSTANCE_DATA, &args![this]).u32();
    if data == 0 {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 1u32],
        );
        return;
    }
    let state = e.mem.u32(data.wrapping_add(4));
    match state {
        0 => {
            let package = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
            e.mem.set_u32(data.wrapping_add(4), 2);
            let location = e.call(GET_0055B980, &args![package]).u32();
            e.call(SANDBOX_START_009F3AF0, &args![data, actor, location]);
            return;
        }
        1 | 2 => e.mem.set_u32(data.wrapping_add(4), 3),
        3 => {}
        _ => return,
    }
    if !e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
        return;
    }
    let old = e.call(SANDBOX_GET_LOCATION, &args![data]).u32();
    if !e.call(SANDBOX_MOVE_TO_NEXT_LOCATION, &args![data]).bool() {
        return;
    }
    let next = e.call(SANDBOX_GET_LOCATION, &args![data]).u32();
    if next == 0 || next == old {
        return;
    }
    let old_cell = e.call(ACTOR_GET_008D6F30, &args![old]).u32();
    let next_cell = e.call(ACTOR_GET_008D6F30, &args![next]).u32();
    let unloaded = cells_unloaded(e, old_cell, next_cell);
    let same_cell = old_cell != 0 && old_cell == next_cell;
    if unloaded {
        let position = e.vcall(next, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
        e.vcall(actor.addr(), ACTOR_SLOT_2A8, &args![position]);
        if !same_cell {
            let world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![next]).u32();
            e.call(
                REFERENCE_MOVE_TO_NEW_SPACE,
                &args![actor, next_cell, world_space],
            );
        }
        return;
    }
    let package = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
    let count = e.call(PACKAGE_GET_00676280, &args![package, actor]).u32();
    let time = x87_store32(f64::from(count));
    let least = x87_quiet_bits(e.global::<u32>(SANDBOX_LEAST_TIME));
    let time = x87_store32(
        e.call(LARGER_OF_00404010, &args![x87_quiet_bits(time), least])
            .f64(),
    );
    e.call(
        ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE,
        &args![actor, next, x87_quiet_bits(time), 0u32],
    );
}

// Translated from 00912b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessGuard` (Xbox PDB; `RET 4`). The instance data is the
/// running one, or the run-once package's (vtable `+0x270`) when that
/// package (`+0x20c`) has type 0xe; without any the running index becomes
/// -1. The guarded reference is the package's initial target (the actor
/// without one), or its extra data's reference pointer; when it is not the
/// target it becomes the target, the actor is added as its guard
/// (`AddGuard`) and its slot `+0x48` gets 0x80000000. Not running a
/// run-once package (`+0x35c`) while the actor's slot `+0x214` is 4 or 9,
/// the data gets `006ecd40(0)` and the actor's slot `+0x418` runs. Unless
/// the actor's slot `+0x4b4` holds: "near" is the package's slot `+0x13c`
/// (actor, 0, -1.0, 0) for a located package, else whether the radius (the
/// package target's integer, or 30.0) exceeds the distance to the guarded
/// reference; when pathing is complete and not near, or `008b3b90` holds,
/// `ProcessGuardStartTravelToLocation(actor, data)` (`+0x7f8`) runs.
pub fn low_process_process_guard(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr) {
    let base = this.addr();
    let mut data = e.call(PROCESS_GET_INSTANCE_DATA, &args![this]).u32();
    if e.vcall(base, GET_RUN_ONCE_PACKAGE_SLOT, &args![]).u32() != 0 {
        let run_once = e.vcall(base, GET_RUN_ONCE_PACKAGE_SLOT, &args![]).u32();
        if e.call(PACKAGE_GET_TYPE, &args![run_once]).u32() == 0xe {
            data = e
                .vcall(base, GET_RUN_ONCE_INSTANCE_DATA_SLOT, &args![])
                .u32();
        }
    }
    if data == 0 {
        e.vcall(base, SET_PROCEDURE_INDEX_RUNNING_SLOT, &args![u32::MAX]);
        return;
    }
    let package = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
    let initial = e
        .call(PACKAGE_COMPUTE_INITIAL_TARGET, &args![package, actor])
        .u32();
    let mut guarded = if initial != 0 { initial } else { actor.addr() };
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![guarded]).u32();
    let pointer = e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32();
    if pointer != 0 {
        guarded = pointer;
    }
    if e.vcall(base, GET_TARGET_SLOT, &args![]).u32() != guarded {
        e.vcall(base, SET_TARGET_SLOT, &args![guarded]);
        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![guarded]).u32();
        e.call(EXTRA_ADD_GUARD, &args![extra, actor]);
        e.vcall(guarded, ACTOR_SLOT_48, &args![0x8000_0000u32]);
    }
    if !e.vcall(base, IS_RUNNING_RUN_ONCE_SLOT, &args![]).bool()
        && (e.vcall(actor.addr(), ACTOR_SLOT_214, &args![]).u32() == 4
            || e.vcall(actor.addr(), ACTOR_SLOT_214, &args![]).u32() == 9)
    {
        e.call(SET_INSTANCE_DATA_006ECD40, &args![data, 0u32]);
        e.vcall(actor.addr(), ACTOR_SLOT_418, &args![]);
        return;
    }
    if e.vcall(actor.addr(), ACTOR_SLOT_4B4, &args![]).bool() {
        return;
    }
    let near = if e.call(GET_0055B980, &args![package]).u32() != 0 {
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        let near = e
            .vcall(
                package,
                PACKAGE_SLOT_13C,
                &args![actor, 0u32, minus_one, 0u32],
            )
            .bool();
        // The package's count is loaded and stored as the radius, unused
        // after this branch.
        e.call(PACKAGE_GET_00676280, &args![package, actor]);
        near
    } else {
        let target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
        let radius = if target != 0 {
            let value = e.call(PACKAGE_TARGET_GET_VALUE, &args![target]).i32();
            x87_store32(f64::from(value))
        } else {
            x87_quiet_bits(e.global::<u32>(GUARD_DEFAULT_RADIUS))
        };
        let distance = e
            .call(REFERENCE_GET_DISTANCE, &args![actor, guarded, 0u32, 0u32])
            .f64();
        x87_load32(radius) > distance
    };
    let start = (e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() && !near)
        || e.call(ACTOR_TEST_008B3B90, &args![actor]).bool();
    if start {
        e.vcall(base, PROCESS_GUARD_START_TRAVEL_SLOT, &args![actor, data]);
    }
}

/// `+0x280` `GetProcedureIndexRunning` (Xbox PDB `BaseProcess` name).
const GET_PROCEDURE_INDEX_RUNNING_SLOT: u32 = 0x280;
/// `00672dd0` (`thiscall` on a package, plain `RET`): its second location.
const PACKAGE_GET_SECOND_LOCATION: u32 = 0x0067_2dd0;
/// `PackageLocation::GetLocReference` / `GetLocObject` /
/// `GetLocObjectType` / `IsLocationGeneric` (engine map, `0067f390`,
/// `0067f430`, `0067f480`, `0067f970`).
const LOCATION_GET_REFERENCE: u32 = 0x0067_f390;
const LOCATION_GET_OBJECT: u32 = 0x0067_f430;
const LOCATION_GET_OBJECT_TYPE: u32 = 0x0067_f480;
const LOCATION_IS_GENERIC: u32 = 0x0067_f970;
/// `00569b80` (`thiscall` on the actor, plain `RET`): its location for a
/// package location of type 6.
const ACTOR_GET_00569B80: u32 = 0x0056_9b80;
/// `pObjectForLocation` (Xbox PDB) +0x88 and `eFormType` +0x8c.
const OBJECT_FOR_LOCATION: u32 = 0x88;
const FORM_TYPE_FOR_LOCATION: u32 = 0x8c;
/// `TESPackage::GetAcquireRadius` (engine map, `006728a0`, `RET 8`: actor,
/// the address of a `float`; its `AL`).
const PACKAGE_GET_ACQUIRE_RADIUS: u32 = 0x0067_28a0;
/// `00403e20` (`thiscall`, plain `RET`) on the setting at `011cdebc`: the
/// address of its `float` value (the radius without an acquire radius).
const SETTING_VALUE_00403E20: u32 = 0x0040_3e20;
const ACQUIRE_RADIUS_SETTING: u32 = 0x011c_debc;
/// The data handler (`011c3f2c`) and
/// `TESDataHandler::EnumReferencesCloseToPoint` (engine map, `0046f280`,
/// `RET 0x1c`) with the callback `0090e660` (this unit) that fills
/// `TempLocList`.
const DATA_HANDLER: u32 = 0x011c_3f2c;
const ENUM_REFERENCES_CLOSE_TO_POINT: u32 = 0x0046_f280;
const COLLECT_LOCATION_CALLBACK: u32 = 0x0090_e660;
/// `005ae3d0` (`thiscall` on a list, `RET 4`: the address of the item):
/// adds an item.
const LIST_ADD_ITEM: u32 = 0x005a_e3d0;
/// `TESPackage::GetLocationCell` / `GetLocationWorld` (engine map,
/// `00675c20`, `00675a50`, `RET 4`: the actor) and `GetLocationCoord`
/// (`00675de0`, `RET 8`: the address of a result point, the actor).
const PACKAGE_GET_LOCATION_CELL: u32 = 0x0067_5c20;
const PACKAGE_GET_LOCATION_WORLD: u32 = 0x0067_5a50;
const PACKAGE_GET_LOCATION_COORD: u32 = 0x0067_5de0;
/// `00901c50` (`thiscall` on a package, plain `RET`; its `AL`).
const PACKAGE_TEST_00901C50: u32 = 0x0090_1c50;
/// `00639b40` (`thiscall` on instance data, plain `RET`): the reference it
/// holds.
const INSTANCE_DATA_GET_REFERENCE: u32 = 0x0063_9b40;
/// `TESPackage::IsInterruptPackage` (engine map, `00678610`).
const PACKAGE_IS_INTERRUPT: u32 = 0x0067_8610;
/// `MobileObject::GetCurrentPackage` (engine map, `009344a0`).
const MOBILE_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// `TESObjectREFR::GetPackageStartLocationCoord` (engine map, `0056add0`):
/// the address of a point.
const REFERENCE_GET_START_LOCATION_COORD: u32 = 0x0056_add0;
/// The point `GetCurrentDestinationCoordinate` starts from (`011f426c`,
/// three words).
const DESTINATION_DEFAULT_POINT: u32 = 0x011f_426c;

// Translated from 009089c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetLocationForPackage` (Xbox PDB; `RET 0xc`): the package's
/// location (its second one with `second`) as a reference. A location of
/// type 0 gives its reference; type 6 the actor's `00569b80` at once. For a
/// generic location of a package `0067efd0` holds: with
/// `GenericLocationsList` empty the candidates are collected first (the
/// object or object type for the search into `+0x88`/`+0x8c` for types 4
/// and 5; `EnumReferencesCloseToPoint(cell, position, radius, position,
/// radius, 0090e660, actor)` with the package's acquire radius or the
/// setting at `011cdebc`; then `+0x88`/`+0x8c` cleared and every
/// `TempLocList` item moved over), and the first one is taken off the list
/// and returned. Otherwise the reference of type 0, else 0.
pub fn low_process_get_location_for_package(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    package: Ptr,
    second: u8,
) -> u32 {
    let base = this.addr();
    let mut result = 0u32;
    let location = if second != 0 {
        e.call(PACKAGE_GET_SECOND_LOCATION, &args![package]).u32()
    } else {
        e.call(GET_0055B980, &args![package]).u32()
    };
    if location == 0 {
        return result;
    }
    if e.call(PACKAGE_LOCATION_GET_TYPE, &args![location]).u32() == 0 {
        result = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
    }
    if e.call(PACKAGE_LOCATION_GET_TYPE, &args![location]).u32() == 6 {
        return e.call(ACTOR_GET_00569B80, &args![actor]).u32();
    }
    if !e.call(LOCATION_IS_GENERIC, &args![location]).bool()
        || !e.call(PACKAGE_TEST_0067EFD0, &args![package]).bool()
    {
        return result;
    }
    let locations = base.wrapping_add(GENERIC_LOCATIONS_LIST);
    if e.call(LIST_IS_EMPTY, &args![locations]).bool() {
        let cell = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
        let position = e
            .vcall(actor.addr(), ACTOR_GET_LOCATION_SLOT, &args![])
            .u32();
        e.with_stack(16, |e, frame| {
            let point = frame.addr();
            let radius = point + 12;
            for offset in [0u32, 4, 8] {
                let word = e.mem.u32(position.wrapping_add(offset));
                e.mem.set_u32(point + offset, word);
            }
            match e.call(PACKAGE_LOCATION_GET_TYPE, &args![location]).u32() {
                4 => {
                    let object = e.call(LOCATION_GET_OBJECT, &args![location]).u32();
                    e.mem
                        .set_u32(base.wrapping_add(OBJECT_FOR_LOCATION), object);
                    e.mem.set_u32(base.wrapping_add(FORM_TYPE_FOR_LOCATION), 0);
                }
                5 => {
                    e.mem.set_u32(base.wrapping_add(OBJECT_FOR_LOCATION), 0);
                    let kind = e.call(LOCATION_GET_OBJECT_TYPE, &args![location]).u32();
                    e.mem
                        .set_u32(base.wrapping_add(FORM_TYPE_FOR_LOCATION), kind);
                }
                _ => {}
            }
            e.mem.set_u32(radius, 0);
            if package.addr() == 0
                || !e
                    .call(PACKAGE_GET_ACQUIRE_RADIUS, &args![package, actor, radius])
                    .bool()
            {
                let value = e
                    .call(SETTING_VALUE_00403E20, &args![ACQUIRE_RADIUS_SETTING])
                    .u32();
                let bits = x87_quiet_bits(e.mem.u32(value));
                e.mem.set_u32(radius, bits);
            }
            let radius_bits = x87_quiet_bits(e.mem.u32(radius));
            let position = e
                .vcall(actor.addr(), ACTOR_GET_LOCATION_SLOT, &args![])
                .u32();
            let handler = e.global::<u32>(DATA_HANDLER);
            e.call(
                ENUM_REFERENCES_CLOSE_TO_POINT,
                &args![
                    handler,
                    cell,
                    point,
                    radius_bits,
                    position,
                    radius_bits,
                    COLLECT_LOCATION_CALLBACK,
                    actor
                ],
            );
        });
        e.mem.set_u32(base.wrapping_add(FORM_TYPE_FOR_LOCATION), 0);
        e.mem.set_u32(base.wrapping_add(OBJECT_FOR_LOCATION), 0);
        let temp = base.wrapping_add(TEMP_LOC_LIST);
        loop {
            let head = e.call(LIST_HEAD_ITEM, &args![temp]).u32();
            if e.mem.u32(head) == 0 {
                break;
            }
            let head = e.call(LIST_HEAD_ITEM, &args![temp]).u32();
            e.call(LIST_ADD_ITEM, &args![locations, head]);
            e.call(LIST_REMOVE_HEAD, &args![temp]);
        }
    }
    if !e.call(LIST_IS_EMPTY, &args![locations]).bool() {
        let head = e.call(LIST_HEAD_ITEM, &args![locations]).u32();
        result = e.mem.u32(head);
        e.call(LIST_REMOVE_HEAD, &args![locations]);
    }
    result
}

/// The procedure number the running package is at: its type's procedure
/// list (`011a3ff0`) at the running procedure index (vtable `+0x280`).
fn running_procedure(e: &mut Engine, this: u32, package: u32) -> u32 {
    let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
    let index = e
        .vcall(this, GET_PROCEDURE_INDEX_RUNNING_SLOT, &args![])
        .u32();
    let list = e
        .mem
        .u32(PROCEDURE_LISTS.wrapping_add(kind.wrapping_mul(4)));
    e.mem.u32(list.wrapping_add(index.wrapping_mul(4)))
}

/// How `GetCurrentDestinationCell` / `GetCurrentDestinationWorldSpace`
/// treat a procedure number (the byte maps `0090f3b4` / `0090f630` and the
/// jump tables after them, identical for both).
#[derive(Clone, Copy)]
enum DestinationCase {
    /// The package's location (when it has one), else the actor's.
    PackageLocation,
    /// The target's, else the actor's.
    TargetOrActor,
    /// The actor's.
    Actor,
    /// The running instance data's reference's (when there is one).
    InstanceData,
    /// The package location when `00901c50` holds and it has one, else
    /// the target's (`GetTarget`, vtable `+0x128`), else the actor's.
    LocationOrTarget,
    /// Unchanged.
    Keep,
}

fn destination_case(procedure: u32) -> DestinationCase {
    match procedure {
        0 | 4 | 5 | 7 | 50 => DestinationCase::PackageLocation,
        1 | 2 | 3 | 6 | 8 | 13 | 14 | 15 | 30 => DestinationCase::TargetOrActor,
        27 | 54 => DestinationCase::Actor,
        42 => DestinationCase::InstanceData,
        47 => DestinationCase::LocationOrTarget,
        _ => DestinationCase::Keep,
    }
}

// Translated from 0090f0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetCurrentDestinationCell` (Xbox PDB; `RET 4`; the `switch`
/// on the running procedure number through the byte map `0090f3b4` and the
/// table `0090f38c`): the actor's cell (`008d6f30`; 0 without an actor),
/// or without a current package that; otherwise by the procedure: the
/// package location's cell (`GetLocationCell(actor)`), the target's or the
/// actor's, the actor's, the instance data reference's, or the package
/// location / `GetTarget` / actor cascade (see `DestinationCase`); other
/// numbers keep the actor's cell. The current package is asked for again at
/// each use.
pub fn low_process_get_current_destination_cell(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
) -> u32 {
    let base = this.addr();
    let current = |e: &mut Engine| e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let cell_of =
        |e: &mut Engine, reference: u32| e.call(ACTOR_GET_008D6F30, &args![reference]).u32();
    let mut result = 0u32;
    if actor.addr() != 0 {
        result = cell_of(e, actor.addr());
    }
    if current(e) == 0 {
        return result;
    }
    let package = current(e);
    let procedure = running_procedure(e, base, package);
    let location_cell = |e: &mut Engine| {
        let package = current(e);
        e.call(PACKAGE_GET_LOCATION_CELL, &args![package, actor])
            .u32()
    };
    match destination_case(procedure) {
        DestinationCase::PackageLocation => {
            let package = current(e);
            result = if e.call(GET_0055B980, &args![package]).u32() != 0 {
                location_cell(e)
            } else {
                cell_of(e, actor.addr())
            };
        }
        DestinationCase::TargetOrActor => {
            let target = e.mem.u32(base.wrapping_add(TARGET));
            result = if target != 0 {
                let target = e.mem.u32(base.wrapping_add(TARGET));
                cell_of(e, target)
            } else {
                cell_of(e, actor.addr())
            };
        }
        DestinationCase::Actor => result = cell_of(e, actor.addr()),
        DestinationCase::InstanceData => {
            let data = e.call(PROCESS_GET_INSTANCE_DATA, &args![this]).u32();
            if data != 0 {
                let reference = e.call(INSTANCE_DATA_GET_REFERENCE, &args![data]).u32();
                if reference != 0 {
                    result = cell_of(e, reference);
                }
            }
        }
        DestinationCase::LocationOrTarget => {
            let package = current(e);
            let located = e.call(PACKAGE_TEST_00901C50, &args![package]).bool() && {
                let package = current(e);
                e.call(GET_0055B980, &args![package]).u32() != 0
            };
            result = if located {
                location_cell(e)
            } else if e.vcall(base, GET_TARGET_SLOT, &args![]).u32() != 0 {
                let target = e.vcall(base, GET_TARGET_SLOT, &args![]).u32();
                cell_of(e, target)
            } else {
                cell_of(e, actor.addr())
            };
        }
        DestinationCase::Keep => {}
    }
    result
}

// Translated from 0090f3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetCurrentDestinationWorldSpace` (Xbox PDB; `RET 4`; the
/// `switch` through the byte map `0090f630` and the table `0090f608`): as
/// `GetCurrentDestinationCell` with world spaces (`GetWorldSpace`,
/// `GetLocationWorld`), the current package asked for once.
pub fn low_process_get_current_destination_world_space(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
) -> u32 {
    let base = this.addr();
    let space_of =
        |e: &mut Engine, reference: u32| e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).u32();
    let mut result = 0u32;
    if actor.addr() != 0 {
        result = space_of(e, actor.addr());
    }
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    if package == 0 {
        return result;
    }
    let procedure = running_procedure(e, base, package);
    let location_world = |e: &mut Engine| {
        e.call(PACKAGE_GET_LOCATION_WORLD, &args![package, actor])
            .u32()
    };
    match destination_case(procedure) {
        DestinationCase::PackageLocation => {
            result = if e.call(GET_0055B980, &args![package]).u32() != 0 {
                location_world(e)
            } else {
                space_of(e, actor.addr())
            };
        }
        DestinationCase::TargetOrActor => {
            let target = e.mem.u32(base.wrapping_add(TARGET));
            result = if target != 0 {
                let target = e.mem.u32(base.wrapping_add(TARGET));
                space_of(e, target)
            } else {
                space_of(e, actor.addr())
            };
        }
        DestinationCase::Actor => result = space_of(e, actor.addr()),
        DestinationCase::InstanceData => {
            let data = e.call(PROCESS_GET_INSTANCE_DATA, &args![this]).u32();
            if data != 0 {
                let reference = e.call(INSTANCE_DATA_GET_REFERENCE, &args![data]).u32();
                if reference != 0 {
                    result = space_of(e, reference);
                }
            }
        }
        DestinationCase::LocationOrTarget => {
            let located = e.call(PACKAGE_TEST_00901C50, &args![package]).bool()
                && e.call(GET_0055B980, &args![package]).u32() != 0;
            result = if located {
                location_world(e)
            } else if e.vcall(base, GET_TARGET_SLOT, &args![]).u32() != 0 {
                let target = e.vcall(base, GET_TARGET_SLOT, &args![]).u32();
                space_of(e, target)
            } else {
                space_of(e, actor.addr())
            };
        }
        DestinationCase::Keep => {}
    }
    result
}

/// Copies the three words of a point.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(from.wrapping_add(offset));
        e.mem.set_u32(to.wrapping_add(offset), word);
    }
}

// Translated from 0090ec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetCurrentDestinationCoordinate` (Xbox PDB; `RET 0xc`; the
/// point returned at `result`, which is returned; the `switch` through the
/// byte map `0090f0a0` and the table `0090f088`). Starting from the point
/// at `011f426c`, with the current package, target and running procedure
/// index, which (unless `flag`) for an interrupt package come from the
/// actor's extra data instead (`GetPackageExtra`, `GetPackageExtraIndex`,
/// `GetPackageExtraTarget`). Without a package or for a package type -1 the
/// start point. Otherwise by the procedure number: the package location's
/// point (`GetLocationCoord`) or the actor's position; the target's or the
/// actor's position; the actor's position, or for an actor target (its
/// slot `+0x100`) with a current package (`MobileObject::GetCurrentPackage`)
/// that package's location point; the location (when `00901c50` holds),
/// the package target's reference's position, or the actor's; for package
/// type 6 (not `flag`) the location point or the actor's package start
/// location (`GetPackageStartLocationCoord`), else the target's or the
/// actor's position. Positions are the references' slot `+0x1f4` points,
/// copied as words.
pub fn low_process_get_current_destination_coordinate(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    result: Ptr,
    actor: Ptr,
    flag: u8,
) -> Ptr {
    let base = this.addr();
    e.with_stack(12, |e, point| {
        let point = point.addr();
        copy_point(e, point, DESTINATION_DEFAULT_POINT);
        let mut package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
        let mut target = e.mem.u32(base.wrapping_add(TARGET));
        let mut index = e
            .vcall(base, GET_PROCEDURE_INDEX_RUNNING_SLOT, &args![])
            .u32();
        if flag == 0 && package != 0 && e.call(PACKAGE_IS_INTERRUPT, &args![package]).bool() {
            let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
            package = e.call(EXTRA_GET_PACKAGE, &args![extra]).u32();
            let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
            index = e.call(EXTRA_GET_PACKAGE_INDEX, &args![extra]).u32();
            let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
            target = e.call(EXTRA_GET_PACKAGE_TARGET, &args![extra]).u32();
        }
        let done =
            package == 0 || e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32() == u32::MAX;
        if !done {
            let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
            let list = e
                .mem
                .u32(PROCEDURE_LISTS.wrapping_add(kind.wrapping_mul(4)));
            let procedure = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
            let position_of = |e: &mut Engine, reference: u32| {
                let position = e.vcall(reference, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
                copy_point(e, point, position);
            };
            let location_coord = |e: &mut Engine, package: u32, reference: u32| {
                e.with_stack(12, |e, temp| {
                    let coord = e
                        .call(PACKAGE_GET_LOCATION_COORD, &args![package, temp, reference])
                        .u32();
                    copy_point(e, point, coord);
                });
            };
            let target_or_actor = |e: &mut Engine| {
                if target != 0 {
                    position_of(e, target);
                } else {
                    position_of(e, actor.addr());
                }
            };
            match procedure {
                0 | 4 | 5 | 7 | 50 => {
                    if e.call(GET_0055B980, &args![package]).u32() != 0 {
                        location_coord(e, package, actor.addr());
                    } else {
                        position_of(e, actor.addr());
                    }
                }
                1 | 2 | 3 | 6 | 8 | 9 | 13 | 14 | 15 | 28 | 30 | 38 | 42 | 44 => {
                    target_or_actor(e);
                }
                27 => {
                    position_of(e, actor.addr());
                    if target != 0 && e.vcall(target, ACTOR_SLOT_100, &args![]).bool() {
                        let current = e.call(MOBILE_GET_CURRENT_PACKAGE, &args![target]).u32();
                        if current != 0 {
                            location_coord(e, current, target);
                        }
                    }
                }
                47 => {
                    let located = e.call(PACKAGE_TEST_00901C50, &args![package]).bool()
                        && e.call(GET_0055B980, &args![package]).u32() != 0;
                    if located {
                        location_coord(e, package, actor.addr());
                    } else if e.call(PACKAGE_GET_TARGET, &args![package]).u32() != 0 && {
                        let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
                        e.call(PACKAGE_TARGET_GET_REFERENCE, &args![package_target])
                            .u32()
                            != 0
                    } {
                        let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
                        let reference = e
                            .call(PACKAGE_TARGET_GET_REFERENCE, &args![package_target])
                            .u32();
                        position_of(e, reference);
                    } else {
                        position_of(e, actor.addr());
                    }
                }
                54 => {
                    if flag == 0 && e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 6 {
                        if e.call(GET_0055B980, &args![package]).u32() != 0 {
                            location_coord(e, package, actor.addr());
                        } else {
                            let start = e
                                .call(REFERENCE_GET_START_LOCATION_COORD, &args![actor])
                                .u32();
                            copy_point(e, point, start);
                        }
                    } else {
                        target_or_actor(e);
                    }
                }
                _ => {}
            }
        }
        copy_point(e, result.addr(), point);
    });
    result
}

/// More process vtable slots: `+0x74` `ProcessActivateOneHour` (Xbox PDB
/// name; actor, a `float`), and the PC-only `+0x7b4`
/// `LowProcess::ProcessEscort`, `+0x7c8` `LowProcess::ProcessActivate`,
/// `+0x80c` `LowProcess::CheckIfHasObject`, `+0x818`
/// `LowProcess::EvaluateOrderAcquireList` (the `LowProcess` table's
/// functions).
const PROCESS_ACTIVATE_ONE_HOUR_SLOT: u32 = 0x74;
const PROCESS_ESCORT_SLOT: u32 = 0x7b4;
const PROCESS_ACTIVATE_SLOT: u32 = 0x7c8;
const CHECK_IF_HAS_OBJECT_SLOT: u32 = 0x80c;
const EVALUATE_ORDER_ACQUIRE_LIST_SLOT: u32 = 0x818;
/// The type descriptors `ProcessAcquire` casts a light's base form with
/// (`TESBoundObject` to `TESObjectLIGH`) and `0046f070` (`thiscall` on the
/// light, plain `RET`; its `AL`).
const BOUND_OBJECT_TYPE: u32 = 0x0118_3108;
const LIGHT_TYPE: u32 = 0x0118_3938;
const LIGHT_TEST_0046F070: u32 = 0x0046_f070;
/// `TESPackage::GetInitialTargetCount` (engine map, `00672930`).
const PACKAGE_GET_INITIAL_TARGET_COUNT: u32 = 0x0067_2930;
/// `0041d8a0` (`thiscall` on a package, plain `RET`): a list node whose
/// next node (`00726070`) `ProcessAcquire` tests.
const PACKAGE_GET_LIST_0041D8A0: u32 = 0x0041_d8a0;
/// `MobileObject::GetCurrentProcessType` (engine map, `00931850`).
const MOBILE_GET_PROCESS_TYPE: u32 = 0x0093_1850;
/// `3600.0` (`float`): the time `ProcessAcquire` gives
/// `ProcessActivateOneHour`.
const ONE_HOUR_SECONDS: u32 = 0x0108_4838;
/// `PackageTarget::GetTargType` / `GetTargObject` / `GetTargObjectType`
/// (engine map, `00519b00`, `00680050`, `00680080`).
const PACKAGE_TARGET_GET_TYPE: u32 = 0x0051_9b00;
const PACKAGE_TARGET_GET_OBJECT: u32 = 0x0068_0050;
const PACKAGE_TARGET_GET_OBJECT_TYPE: u32 = 0x0068_0080;
/// `TESPackage::GetLocationReference` (engine map, `00676140`, `RET 4`) and
/// `TESObjectREFR::IsFurniture` (`00568680`).
const PACKAGE_GET_LOCATION_REFERENCE: u32 = 0x0067_6140;
const REFERENCE_IS_FURNITURE: u32 = 0x0056_8680;
/// The two base forms (`011ca248`, `011ca244`) for which activating a
/// furniture location succeeds at once.
const ACTIVATE_DONE_FORM: u32 = 0x011c_a244;
/// `300.0` (`float`): the `float` `ProcessActivate` gives the package's slot
/// `+0x13c` for packages of type 9.
const ACTIVATE_WAIT: u32 = 0x0101_3dc0;
/// A package's vtable slot `+0x144` (actor, 0; its `AL`): whether the
/// activation is done.
const PACKAGE_SLOT_144: u32 = 0x144;
/// A reference's vtable slot `+0x218` (no words; its `AL`).
const REFERENCE_SLOT_218: u32 = 0x218;
/// `TESObjectREFR::Activate` (engine map, `00573170`, `RET 0x10`) and
/// `RunScript` (`00565870`).
const REFERENCE_ACTIVATE: u32 = 0x0057_3170;
const REFERENCE_RUN_SCRIPT: u32 = 0x0056_5870;
/// `005c1a00` (`thiscall` on the player, `RET 8`) and the player's byte at
/// `+0x20c`.
const PLAYER_005C1A00: u32 = 0x005c_1a00;
const PLAYER_BYTE_20C: u32 = 0x20c;
/// `TESPackage::IsTargetAnActor` (engine map, `00678ec0`, `RET 4`) and
/// `006780e0` (`RET 8`: actor, 0).
const PACKAGE_IS_TARGET_AN_ACTOR: u32 = 0x0067_8ec0;
const PACKAGE_GET_ACTOR_TARGET: u32 = 0x0067_80e0;
/// `PatrolActorPackageData::GetInitialReferenceForSetup` (engine map,
/// `009f2d40`, `cdecl`: actor, package).
const PATROL_INITIAL_REFERENCE: u32 = 0x009f_2d40;
/// `pTargetFormList` (Xbox PDB) +0x4c, `sFormListNumb` +0x50,
/// `pObjecttoAcquire` +0x84, `pFactiontoAquire` +0x90.
const TARGET_FORM_LIST: u32 = 0x4c;
const FORM_LIST_NUMBER: u32 = 0x50;
const FACTION_TO_ACQUIRE: u32 = 0x90;
/// `00500940` (`thiscall` on a form list, plain `RET`): its list (`+0x18`).
const FORM_LIST_GET_LIST: u32 = 0x0050_0940;
/// `TESPackage::GetPackageSearchLocation`, `GetSearchLocationRadius`,
/// `GetSearchLocationCell`, `GetSearchLocationCoord` (engine map,
/// `00672f20`, `006758a0`, `006757e0`, `00675830`).
const PACKAGE_GET_SEARCH_LOCATION: u32 = 0x0067_2f20;
const PACKAGE_GET_SEARCH_RADIUS: u32 = 0x0067_58a0;
const PACKAGE_GET_SEARCH_CELL: u32 = 0x0067_57e0;
const PACKAGE_GET_SEARCH_COORD: u32 = 0x0067_5830;
/// `FLT_MAX` (`float`, `01016970`): the search radius in cells where
/// `00425fd0` holds.
const SEARCH_RADIUS_MAX: u32 = 0x0101_6970;
/// The callback `0090dd80` (this unit) `GetTargetForPackage` collects
/// acquire candidates with.
const COLLECT_TARGET_CALLBACK: u32 = 0x0090_dd80;
/// The type descriptors the acquire object's owner is cast with (`TESForm`
/// to `TESNPC`), `TESObjectREFR::GetOwner` (`00567790`), the process lists
/// (`011e0e80`) and `ProcessLists::GetActorRefInHigh` (`00970a20`, `RET 8`).
const FORM_TYPE: u32 = 0x0118_3028;
const NPC_TYPE: u32 = 0x0118_3a1c;
const REFERENCE_GET_OWNER: u32 = 0x0056_7790;
const PROCESS_LISTS: u32 = 0x011e_0e80;
const PROCESS_LISTS_ACTOR_IN_HIGH: u32 = 0x0097_0a20;

// Translated from 00908f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessAcquire` (Xbox PDB; `RET 4`; nothing in `EAX`).
/// Sets the target when there is none (vtable `+0x8c`). For packages of type
/// 8 or 0x10 with a target, the target's base form type decides (the
/// `switch` through `009092f8`/`009092ec` on type - 0x15): types 0x15, 0x1b,
/// 0x1c, 0x20, 0x21, 0x24-0x27, 0x2a-0x2d, 0x3b, 0x3c finish the step
/// (`AddToProcedureIndexRunning(1)`); 0x1e (a light) finishes it unless
/// `0046f070` holds for it; the others go on. Then, with a package target
/// reference or a package `0067efd0` holds for: when `CheckIfHasObject(actor,
/// count)` (`+0x80c`, the initial target count or 1) holds, the index is
/// reset to 0 for packages of type 3 whose slot `+0x13c` fails, else moved
/// by 2 (procedure type 0x1a with no second list node) or 1, and packages of
/// type 2 escort (`+0x7b4`); without the object, actors of process type
/// below 2 get `ProcessActivateOneHour(3600.0)` (`+0x74`), others
/// `ProcessActivate(1)` (`+0x7c8`). Otherwise the index moves by 2
/// (procedure type 0x1a) or 1.
pub fn low_process_process_acquire(e: &mut Engine, this: Ptr<LowProcess>, actor: Ptr) {
    let base = this.addr();
    let current = |e: &mut Engine| e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let step = |e: &mut Engine, delta: u32| {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, delta],
        );
    };
    let mut package_target = 0u32;
    if e.mem.u32(base.wrapping_add(TARGET)) == 0 {
        e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
    }
    if e.mem.u32(base.wrapping_add(TARGET)) != 0 {
        let package = current(e);
        package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    }
    if e.mem.u32(base.wrapping_add(TARGET)) != 0 {
        let package = current(e);
        let typed = e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 8 || {
            let package = current(e);
            e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 0x10
        };
        if typed {
            let target = e.mem.u32(base.wrapping_add(TARGET));
            let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![target]).u32();
            let form_type = e.call(FORM_GET_TYPE, &args![base_form]).u32();
            match form_type {
                0x15 | 0x1b | 0x1c | 0x20 | 0x21 | 0x24..=0x27 | 0x2a..=0x2d | 0x3b | 0x3c => {
                    step(e, 1);
                    return;
                }
                0x1e => {
                    let target = e.mem.u32(base.wrapping_add(TARGET));
                    let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![target]).u32();
                    let light = e
                        .call(
                            DYNAMIC_CAST,
                            &args![base_form, 0u32, BOUND_OBJECT_TYPE, LIGHT_TYPE, 0u32],
                        )
                        .u32();
                    if light != 0 && !e.call(LIGHT_TEST_0046F070, &args![light]).bool() {
                        step(e, 1);
                    }
                    return;
                }
                _ => {}
            }
        }
    }
    let has_reference = package_target != 0
        && e.call(PACKAGE_TARGET_GET_REFERENCE, &args![package_target])
            .u32()
            != 0;
    if !has_reference && {
        let package = current(e);
        !e.call(PACKAGE_TEST_0067EFD0, &args![package]).bool()
    } {
        let package = current(e);
        let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
        step(e, if kind == 0x1a { 2 } else { 1 });
        return;
    }
    let mut count = 1u32;
    let package = current(e);
    if e.call(PACKAGE_GET_TARGET, &args![package]).u32() != 0 {
        let package = current(e);
        count = e
            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![package])
            .u32();
    }
    if e.vcall(base, CHECK_IF_HAS_OBJECT_SLOT, &args![actor, count])
        .bool()
    {
        let package = current(e);
        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
        let accepted = e
            .vcall(
                package,
                PACKAGE_SLOT_13C,
                &args![actor, 0u32, minus_one, 0u32],
            )
            .bool();
        if !accepted && {
            let package = current(e);
            e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 3
        } {
            e.vcall(base, SET_PROCEDURE_INDEX_RUNNING_SLOT, &args![0u32]);
        } else {
            let package = current(e);
            let last = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32() == 0x1a && {
                let package = current(e);
                let node = e.call(PACKAGE_GET_LIST_0041D8A0, &args![package]).u32();
                e.call(LIST_NEXT_NODE, &args![node]).u32() == 0
            };
            step(e, if last { 2 } else { 1 });
        }
        let package = current(e);
        if e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 2 {
            e.vcall(base, PROCESS_ESCORT_SLOT, &args![actor]);
        }
    } else if e.call(MOBILE_GET_PROCESS_TYPE, &args![actor]).i32() < 2 {
        let hour = x87_quiet_bits(e.global::<u32>(ONE_HOUR_SECONDS));
        e.vcall(base, PROCESS_ACTIVATE_ONE_HOUR_SLOT, &args![actor, hour]);
    } else {
        e.vcall(base, PROCESS_ACTIVATE_SLOT, &args![actor, 1u32]);
    }
}

/// Whether the player is the reference, sleeps or rests, and has the byte at
/// `+0x20c` clear.
fn player_resting(e: &mut Engine, reference: u32) -> bool {
    let player = e.global::<u32>(PLAYER);
    if reference != player {
        return false;
    }
    if !e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool() {
        return false;
    }
    let player = e.global::<u32>(PLAYER);
    e.mem.u8(player.wrapping_add(PLAYER_BYTE_20C)) == 0
}

/// Frees the acquire object (`007b3fa0`, flag 1) and clears it and the
/// target (vtable `+0x12c` with 0).
fn drop_acquire_object(e: &mut Engine, this: u32) {
    let object = e.mem.u32(this.wrapping_add(ACQUIRE_OBJECT));
    if object != 0 {
        e.call(DELETE_EMPTY_DESTRUCTOR, &args![object, 1u32]);
    }
    e.mem.set_u32(this.wrapping_add(ACQUIRE_OBJECT), 0);
    e.vcall(this, SET_TARGET_SLOT, &args![0u32]);
}

// Translated from 0090d620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessActivate` (Xbox PDB; `RET 8`; `acquire` set when it
/// activates for an acquire step). Finds a target (vtable `+0x8c`) when
/// there is none or `00440d80` holds for it; without one it moves the index
/// by 2 (with `acquire`) and returns false. Package targets of type 0 or 3
/// clear the items-activated count. A furniture location reference with the
/// target's base form `011ca248` or `011ca244` returns true. Packages of type
/// 9 whose slot `+0x13c` (300.0) fails move the index by -2. The actor
/// paths to a target farther than the package radius once pathing is
/// complete (`SetPathfindingGoal_ov3(target, -1.0, 0)`: false when it fails,
/// true when the process type changed). Then the package's slot `+0x144`
/// says whether the activation happens (false when not). With `acquire`:
/// the acquire object's count is taken off the items to activate (unless
/// its reference is an actor of slot `+0x22c` 0 whose slot `+0x218` holds),
/// the step ends when none are left; a non-actor target (or an actor one
/// flagged above) is activated with the object's amount and count and the
/// acquire object dropped; an actor target makes a resting player wake
/// (`005c1a00(0, 0)`, false), enters combat for an acquire object with its
/// byte +9, or activates for its byte +8 and drops it; without an acquire
/// object a non-player target moves the index by 2 and returns false.
/// Without `acquire`: non-actor targets are activated (`(actor, 0, 0, 1)`),
/// packages of type 2 move the actor's process's index by 2, a resting
/// player is woken (`005c1a00(0, 1)`, false), otherwise true. Each finished
/// activation runs the target's script (`RunScript`) and returns true.
pub fn low_process_process_activate(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    acquire: u8,
) -> bool {
    let base = this.addr();
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let target_of = |e: &Engine| e.mem.u32(base.wrapping_add(TARGET));
    if target_of(e) == 0 || e.call(REFERENCE_TEST_00440D80, &args![target_of(e)]).bool() {
        e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
    }
    if target_of(e) == 0 {
        if acquire != 0 {
            e.vcall(
                base,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, 2u32],
            );
        }
        return false;
    }
    let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    if e.call(PACKAGE_TARGET_GET_TYPE, &args![package_target])
        .u32()
        == 0
        || e.call(PACKAGE_TARGET_GET_TYPE, &args![package_target])
            .u32()
            == 3
    {
        e.mem.set_u32(base.wrapping_add(NUMBER_ITEMS_ACTIVATE), 0);
    }
    let location = e
        .call(PACKAGE_GET_LOCATION_REFERENCE, &args![package, actor])
        .u32();
    if location != 0 && e.call(REFERENCE_IS_FURNITURE, &args![location]).bool() {
        let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![target_of(e)]).u32();
        let matches = base_form == e.global::<u32>(ANGLED_FORM) || {
            let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![target_of(e)]).u32();
            base_form == e.global::<u32>(ACTIVATE_DONE_FORM)
        };
        if matches {
            return true;
        }
    }
    if e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 9 {
        let wait = x87_quiet_bits(e.global::<u32>(ACTIVATE_WAIT));
        if !e
            .vcall(package, PACKAGE_SLOT_13C, &args![actor, 0u32, wait, 0u32])
            .bool()
        {
            e.vcall(
                base,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, 0xffff_fffeu32],
            );
        }
    }
    // The time delta is computed and stored, its value unused.
    low_process_get_time_delta(e, this, actor.addr());
    let current = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let radius = e
        .call(PACKAGE_RADIUS_ACTOR_TO_TARGET, &args![current, actor, 0u32])
        .f64();
    let radius = x87_store32(radius);
    if target_of(e) != 0 {
        let distance = e
            .call(
                REFERENCE_GET_DISTANCE,
                &args![actor, target_of(e), 0u32, 0u32],
            )
            .f64();
        if x87_load32(radius) < distance && e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool()
        {
            let kind = e.call(MOBILE_GET_PROCESS_TYPE, &args![actor]).u32();
            let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
            if !e
                .call(
                    ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE,
                    &args![actor, target_of(e), minus_one, 0u32],
                )
                .bool()
            {
                return false;
            }
            if e.call(MOBILE_GET_PROCESS_TYPE, &args![actor]).u32() != kind {
                return true;
            }
        }
    }
    if !e
        .vcall(package, PACKAGE_SLOT_144, &args![actor, 0u32])
        .bool()
    {
        return false;
    }
    let activated = target_of(e);
    if acquire != 0 {
        let mut amount = 0u32;
        let mut count = 1u32;
        let mut actor_object = false;
        let object = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
        if object != 0 {
            let reference = e.mem.u32(object);
            let kept = reference != 0
                && !e.vcall(reference, ACTOR_SLOT_22C, &args![0u32]).bool()
                && e.vcall(reference, REFERENCE_SLOT_218, &args![]).bool();
            if !kept {
                let object = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
                amount = e.mem.u32(object.wrapping_add(4));
                count = e.mem.u32(object.wrapping_add(0xc));
                let left = e.mem.u32(base.wrapping_add(NUMBER_ITEMS_ACTIVATE));
                e.mem.set_u32(
                    base.wrapping_add(NUMBER_ITEMS_ACTIVATE),
                    left.wrapping_sub(count),
                );
                let reference = e.mem.u32(object);
                if e.vcall(reference, ACTOR_SLOT_22C, &args![0u32]).bool() {
                    actor_object = true;
                }
            }
            if e.mem.i32(base.wrapping_add(NUMBER_ITEMS_ACTIVATE)) <= 0 {
                e.vcall(
                    base,
                    ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                    &args![actor, 2u32],
                );
            }
        }
        let target = target_of(e);
        if !e.vcall(target, ACTOR_SLOT_100, &args![]).bool() || actor_object {
            let target = target_of(e);
            e.call(
                REFERENCE_ACTIVATE,
                &args![target, actor, 1u32, amount, count],
            );
            drop_acquire_object(e, base);
            e.vcall(
                base,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, 2u32],
            );
        } else {
            let target = target_of(e);
            if e.vcall(target, ACTOR_SLOT_100, &args![]).bool() {
                if player_resting(e, target_of(e)) {
                    let player = e.global::<u32>(PLAYER);
                    e.call(PLAYER_005C1A00, &args![player, 0u32, 0u32]);
                    return false;
                }
                let object = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
                if object != 0 {
                    if e.mem.u8(object.wrapping_add(9)) != 0 {
                        let mut words = vec![actor.addr(), target_of(e)];
                        words.extend([0u32, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0]);
                        e.vcall(base, ENTER_COMBAT_SLOT, &words);
                    } else if e.mem.u8(object.wrapping_add(8)) != 0 {
                        let object = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
                        let object_amount = e.mem.u32(object.wrapping_add(4));
                        let target = target_of(e);
                        e.call(
                            REFERENCE_ACTIVATE,
                            &args![target, actor, 1u32, object_amount, count],
                        );
                        drop_acquire_object(e, base);
                    }
                } else if target_of(e) != e.global::<u32>(PLAYER) {
                    e.vcall(
                        base,
                        ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                        &args![actor, 2u32],
                    );
                    return false;
                }
            }
        }
    } else {
        let target = target_of(e);
        if !e.vcall(target, ACTOR_SLOT_100, &args![]).bool() {
            let target = target_of(e);
            e.call(REFERENCE_ACTIVATE, &args![target, actor, 0u32, 0u32, 1u32]);
        } else if e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 2 {
            let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
            e.vcall(
                process,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, 2u32],
            );
        } else if player_resting(e, target_of(e)) {
            let player = e.global::<u32>(PLAYER);
            e.call(PLAYER_005C1A00, &args![player, 0u32, 1u32]);
            return false;
        } else {
            return true;
        }
    }
    if activated != 0 {
        e.call(REFERENCE_RUN_SCRIPT, &args![activated]);
    }
    true
}

/// The reference pointer of a reference's extra data (`0041c8d0`), taking
/// its place when `00440d80` holds for the reference and it has one.
fn reference_pointer(e: &mut Engine, reference: u32) -> u32 {
    if reference == 0 || !e.call(REFERENCE_TEST_00440D80, &args![reference]).bool() {
        return reference;
    }
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![reference]).u32();
    if e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32() == 0 {
        return reference;
    }
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![reference]).u32();
    e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32()
}

/// How `GetTargetForPackage` treats a package type (the byte maps
/// `00908990` / `009089ac` on type - 1).
#[derive(Clone, Copy, PartialEq)]
enum TargetCase {
    /// Types 1, 2 and 7: an actor target (`IsTargetAnActor`), else as the
    /// patrol case.
    Actor,
    /// Type 0xd: the patrol's reference.
    Patrol,
    /// Every other type.
    Other,
}

fn target_case(kind: u32) -> TargetCase {
    match kind {
        1 | 2 | 7 => TargetCase::Actor,
        0xd => TargetCase::Patrol,
        _ => TargetCase::Other,
    }
}

// Translated from 009082d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetTargetForPackage` (Xbox PDB; `RET 8`): the target for
/// the actor and package (0 without a package). With a package target of
/// type 0 (a reference): none without its reference; a reference whose
/// slot `+0x22c` (1) holds makes the package never run for it
/// (`SetNeverRun(reference, 1)`) and gives none; by the package type an
/// actor target (`006780e0`), the patrol's current or initial reference, or
/// the target reference, followed through its extra data's reference
/// pointer for non-actor references `00440d80` holds for. Type 3 gives the
/// actor's `00569b80`. Other types, for packages `0067efd0` holds for: with
/// `ObjectList` empty the candidates are found (an actor target, the
/// patrol's references, or a search: the object, form list entry, faction
/// or object type to look for into `+0x84`/`+0x88`/`+0x8c`/`+0x90`, then
/// `EnumReferencesCloseToPoint` around the search location or the actor
/// with the search, acquire or default radius and the callback `0090dd80`,
/// then `EvaluateOrderAcquireList`, and the initial target count into
/// `+0x58`); the first `ObjectList` entry becomes the acquire object and
/// gives the target (for entries of kind 2 the owner's high-process actor
/// when the reference is not an actor) and is taken off the list. Last, a
/// reference `00440d80` holds for is replaced by its reference pointer.
pub fn low_process_get_target_for_package(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    package: Ptr,
) -> u32 {
    let base = this.addr();
    let package = package.addr();
    if package == 0 {
        return 0;
    }
    let mut result = 0u32;
    let is_current = package == e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    if e.call(PACKAGE_GET_TARGET, &args![package]).u32() != 0 {
        let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
        let kind = e
            .call(PACKAGE_TARGET_GET_TYPE, &args![package_target])
            .u32();
        if kind == 0 {
            if e.call(PACKAGE_TARGET_GET_REFERENCE, &args![package_target])
                .u32()
                == 0
            {
                return result;
            }
            let reference = e
                .call(PACKAGE_TARGET_GET_REFERENCE, &args![package_target])
                .u32();
            if e.vcall(reference, ACTOR_SLOT_22C, &args![1u32]).bool() {
                let reference = e
                    .call(PACKAGE_TARGET_GET_REFERENCE, &args![package_target])
                    .u32();
                e.call(PACKAGE_SET_NEVER_RUN, &args![package, reference, 1u32]);
                return result;
            }
            let case = target_case(e.call(PACKAGE_GET_TYPE, &args![package]).u32());
            let mut patrol = case == TargetCase::Patrol;
            if case == TargetCase::Actor {
                if e.call(PACKAGE_IS_TARGET_AN_ACTOR, &args![package, actor])
                    .bool()
                {
                    result = e
                        .call(PACKAGE_GET_ACTOR_TARGET, &args![package, actor, 0u32])
                        .u32();
                } else {
                    patrol = true;
                }
            } else if case == TargetCase::Other {
                result = e
                    .call(PACKAGE_TARGET_GET_REFERENCE, &args![package_target])
                    .u32();
            }
            if patrol {
                let data = e.call(PROCESS_GET_INSTANCE_DATA, &args![this]).u32();
                if data != 0 {
                    result = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
                }
                if e.mem.u32(base.wrapping_add(TARGET)) == 0 {
                    result = e
                        .call(PATROL_INITIAL_REFERENCE, &args![actor, package])
                        .u32();
                }
            }
            if result != 0
                && e.call(REFERENCE_TEST_00440D80, &args![result]).bool()
                && !e.vcall(result, ACTOR_SLOT_100, &args![]).bool()
                && e.call(REFERENCE_GET_EXTRA_LIST, &args![result]).u32() != 0
            {
                let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![result]).u32();
                result = e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32();
            }
        } else if e
            .call(PACKAGE_TARGET_GET_TYPE, &args![package_target])
            .u32()
            == 3
        {
            result = e.call(ACTOR_GET_00569B80, &args![actor]).u32();
        } else if e.call(PACKAGE_TEST_0067EFD0, &args![package]).bool() {
            let objects = base.wrapping_add(OBJECT_LIST);
            if e.call(LIST_IS_EMPTY, &args![objects]).bool() {
                result = find_acquire_candidates(e, this, actor, package, is_current);
            }
            if !e.call(LIST_IS_EMPTY, &args![objects]).bool() {
                let head = e.call(LIST_HEAD_ITEM, &args![objects]).u32();
                let first = e.mem.u32(head);
                e.mem.set_u32(base.wrapping_add(ACQUIRE_OBJECT), first);
                let object = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
                if e.mem.u32(object.wrapping_add(0x18)) == 2 {
                    let reference = e.mem.u32(object);
                    if e.vcall(reference, ACTOR_SLOT_100, &args![]).bool() {
                        let object = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
                        result = e.mem.u32(object);
                    } else {
                        let object = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
                        let reference = e.mem.u32(object);
                        let owner = e.call(REFERENCE_GET_OWNER, &args![reference]).u32();
                        let npc = e
                            .call(DYNAMIC_CAST, &args![owner, 0u32, FORM_TYPE, NPC_TYPE, 0u32])
                            .u32();
                        if npc != 0 {
                            result = e
                                .call(
                                    PROCESS_LISTS_ACTOR_IN_HIGH,
                                    &args![PROCESS_LISTS, npc, 0u32],
                                )
                                .u32();
                        }
                    }
                } else {
                    let object = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
                    result = e.mem.u32(object);
                }
                e.call(
                    LIST_REMOVE_ITEM,
                    &args![objects, base.wrapping_add(ACQUIRE_OBJECT)],
                );
            }
        }
    }
    reference_pointer(e, result)
}

/// `GetTargetForPackage` with `ObjectList` empty: the candidates by package
/// type (see there); returns the target found directly (0 for a search).
fn find_acquire_candidates(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    package: u32,
    is_current: bool,
) -> u32 {
    let base = this.addr();
    let case = target_case(e.call(PACKAGE_GET_TYPE, &args![package]).u32());
    if case == TargetCase::Actor
        && e.call(PACKAGE_IS_TARGET_AN_ACTOR, &args![package, actor])
            .bool()
    {
        return e
            .call(PACKAGE_GET_ACTOR_TARGET, &args![package, actor, 0u32])
            .u32();
    }
    if case != TargetCase::Other {
        let mut result = 0u32;
        if is_current {
            let data = e.call(PROCESS_GET_INSTANCE_DATA, &args![this]).u32();
            if data != 0 {
                result = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
            }
        }
        if result == 0 {
            result = e
                .call(PATROL_INITIAL_REFERENCE, &args![actor, package])
                .u32();
        }
        return result;
    }
    let cell = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
    let position = e
        .vcall(actor.addr(), ACTOR_GET_LOCATION_SLOT, &args![])
        .u32();
    e.with_stack(0x1c, |e, frame| {
        let point = frame.addr();
        let radius = point + 12;
        let coord = point + 16;
        copy_point(e, point, position);
        let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
        let kind = e
            .call(PACKAGE_TARGET_GET_TYPE, &args![package_target])
            .u32();
        if kind == 1 {
            let object = e
                .call(PACKAGE_TARGET_GET_OBJECT, &args![package_target])
                .u32();
            if e.call(FORM_GET_TYPE, &args![object]).u32() == 0x55 {
                if e.mem.u32(base.wrapping_add(TARGET_FORM_LIST)) == 0 {
                    e.mem.set_u32(base.wrapping_add(TARGET_FORM_LIST), object);
                    e.mem.set_u16(base.wrapping_add(FORM_LIST_NUMBER), 0);
                } else {
                    let number = e.mem.u16(base.wrapping_add(FORM_LIST_NUMBER));
                    e.mem
                        .set_u16(base.wrapping_add(FORM_LIST_NUMBER), number.wrapping_add(1));
                }
                let form_list = e.mem.u32(base.wrapping_add(TARGET_FORM_LIST));
                let mut node = e.call(FORM_LIST_GET_LIST, &args![form_list]).u32();
                if !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                    // The walk reads each entry into a local nothing uses
                    // later; it runs while the list number is not above
                    // the step count.
                    let mut steps = 0i32;
                    while node != 0
                        && !e.call(LIST_IS_EMPTY, &args![node]).bool()
                        && i32::from(e.mem.i16(base.wrapping_add(FORM_LIST_NUMBER))) <= steps
                    {
                        e.call(LIST_HEAD_ITEM, &args![node]);
                        node = e.call(LIST_NEXT_NODE, &args![node]).u32();
                        steps = steps.wrapping_add(1);
                    }
                }
                if object != 0 {
                    e.mem.set_u32(base.wrapping_add(OBJECT_TO_ACQUIRE), object);
                }
                e.mem.set_u32(base.wrapping_add(FORM_TYPE_FOR_LOCATION), 0);
            } else if e.call(FORM_GET_TYPE, &args![object]).u32() == 8 {
                e.mem.set_u32(base.wrapping_add(FACTION_TO_ACQUIRE), object);
                e.mem
                    .set_u32(base.wrapping_add(FORM_TYPE_FOR_LOCATION), 0x1c);
            } else {
                let object = e
                    .call(PACKAGE_TARGET_GET_OBJECT, &args![package_target])
                    .u32();
                e.mem.set_u32(base.wrapping_add(OBJECT_TO_ACQUIRE), object);
                e.mem.set_u32(base.wrapping_add(FORM_TYPE_FOR_LOCATION), 0);
            }
        } else if kind == 2 {
            e.mem.set_u32(base.wrapping_add(OBJECT_TO_ACQUIRE), 0);
            let object_type = e
                .call(PACKAGE_TARGET_GET_OBJECT_TYPE, &args![package_target])
                .u32();
            e.mem
                .set_u32(base.wrapping_add(FORM_TYPE_FOR_LOCATION), object_type);
        }
        e.mem.set_u32(radius, 0);
        let handler = e.global::<u32>(DATA_HANDLER);
        if e.call(PACKAGE_GET_SEARCH_LOCATION, &args![package]).u32() != 0 {
            let search_radius = e
                .call(PACKAGE_GET_SEARCH_RADIUS, &args![package, actor])
                .u32();
            let search_cell = e
                .call(PACKAGE_GET_SEARCH_CELL, &args![package, actor])
                .u32();
            let coord_at = e
                .call(PACKAGE_GET_SEARCH_COORD, &args![package, coord, actor])
                .u32();
            copy_point(e, point, coord_at);
            let bits = if search_radius != 0 {
                x87_store32(f64::from(search_radius))
            } else if e.call(TEST_00425FD0, &args![search_cell]).bool() {
                x87_quiet_bits(e.global::<u32>(SEARCH_RADIUS_MAX))
            } else {
                let value = e
                    .call(SETTING_VALUE_00403E20, &args![ACQUIRE_RADIUS_SETTING])
                    .u32();
                x87_quiet_bits(e.mem.u32(value))
            };
            e.mem.set_u32(radius, bits);
            let bits = x87_quiet_bits(e.mem.u32(radius));
            e.call(
                ENUM_REFERENCES_CLOSE_TO_POINT,
                &args![
                    handler,
                    search_cell,
                    point,
                    bits,
                    point,
                    bits,
                    COLLECT_TARGET_CALLBACK,
                    actor
                ],
            );
        } else {
            if package == 0
                || !e
                    .call(PACKAGE_GET_ACQUIRE_RADIUS, &args![package, actor, radius])
                    .bool()
            {
                let value = e
                    .call(SETTING_VALUE_00403E20, &args![ACQUIRE_RADIUS_SETTING])
                    .u32();
                let bits = x87_quiet_bits(e.mem.u32(value));
                e.mem.set_u32(radius, bits);
            }
            let bits = x87_quiet_bits(e.mem.u32(radius));
            let position = e
                .vcall(actor.addr(), ACTOR_GET_LOCATION_SLOT, &args![])
                .u32();
            e.call(
                ENUM_REFERENCES_CLOSE_TO_POINT,
                &args![
                    handler,
                    cell,
                    point,
                    bits,
                    position,
                    bits,
                    COLLECT_TARGET_CALLBACK,
                    actor
                ],
            );
        }
    });
    e.mem.set_u32(base.wrapping_add(FORM_TYPE_FOR_LOCATION), 0);
    e.mem.set_u32(base.wrapping_add(OBJECT_TO_ACQUIRE), 0);
    e.vcall(base, EVALUATE_ORDER_ACQUIRE_LIST_SLOT, &args![actor]);
    let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    if package_target != 0
        && e.call(PACKAGE_TARGET_GET_TYPE, &args![package_target])
            .u32()
            != 0
    {
        let count = e
            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![package])
            .u32();
        e.mem
            .set_u32(base.wrapping_add(NUMBER_ITEMS_ACTIVATE), count);
    }
    0
}

/// More process vtable slots (Xbox PDB `BaseProcess` names; `+0x7f4` is
/// PC-only, `LowProcess::ProcessGuard` in the `LowProcess` table).
const SET_LOCKED_LOCATION_SLOT: u32 = 0x1c0;
const CLEAR_RUN_ONCE_PACKAGE_SLOT: u32 = 0x214;
const SET_CONTINUING_PACKAGE_SLOT: u32 = 0x4e4;
const START_NEW_PACKAGE_SLOT: u32 = 0x598;
const PROCESS_GUARD_SLOT: u32 = 0x7f4;
/// The actor's vtable slots `+0x26c` (`0087d770` in the `Character` table,
/// no words; its `AL`), `+0x28c` (`008bc750`, two words: package, day) and
/// `+0x2c8` (`008815a0`, no words; its `EAX`).
const ACTOR_SLOT_26C: u32 = 0x26c;
const ACTOR_SLOT_28C: u32 = 0x28c;
const ACTOR_SLOT_2C8: u32 = 0x2c8;
/// `00401000` (`cdecl`, one word): allocates; `0096a2d0` (`thiscall`,
/// plain `RET`) constructs the 8-byte package list `GetMissedPacks` fills;
/// `00470470` empties it and `004702f0` (`RET 4`, flag 1) deletes it.
const ALLOCATE: u32 = 0x0040_1000;
const PACKAGE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DELETE: u32 = 0x0047_02f0;
/// `Actor::GetMissedPacks` (engine map, `00881470`, `RET 8`: the list, the
/// date last processed as a `float`).
const ACTOR_GET_MISSED_PACKS: u32 = 0x0088_1470;
/// `0087eaa0` (`thiscall` on a package, plain `RET`): its conditions;
/// `TESCondition::IsTrue` (`00680c30`, `RET 8`: actor, target).
const PACKAGE_GET_CONDITIONS: u32 = 0x0087_eaa0;
const CONDITION_IS_TRUE: u32 = 0x0068_0c30;
/// `Actor::UnlockLockDoorsProcedure` (engine map, `008a6a40`).
const ACTOR_UNLOCK_LOCK_DOORS: u32 = 0x008a_6a40;
/// `TESObjectREFR::SetLocationOnReference` (engine map, `00575830`, `RET 4`:
/// a point) and `00575770` (`RET 4`: a `float`, here `FLT_MAX`).
const REFERENCE_SET_LOCATION: u32 = 0x0057_5830;
const REFERENCE_SET_ANGLE_00575770: u32 = 0x0057_5770;
/// `TESPackage::GetOncePerDay` (`00670f90`), `00670ef0`, `00670f10`,
/// `00670ed0` and `005394a0` (package flag tests; their `AL`),
/// `TESPackage::IsScriptPackage` (`00674dd0`), `008b1ff0` (`thiscall` on a
/// package; its `AL`).
const PACKAGE_GET_ONCE_PER_DAY: u32 = 0x0067_0f90;
const PACKAGE_TEST_00670EF0: u32 = 0x0067_0ef0;
const PACKAGE_TEST_00670F10: u32 = 0x0067_0f10;
const PACKAGE_TEST_00670ED0: u32 = 0x0067_0ed0;
const PACKAGE_TEST_005394A0: u32 = 0x0053_94a0;
const PACKAGE_IS_SCRIPT: u32 = 0x0067_4dd0;
const PACKAGE_TEST_008B1FF0: u32 = 0x008b_1ff0;
/// `Calendar::GetDay` (engine map, `00867d60`; its `AL`).
const CALENDAR_GET_DAY: u32 = 0x0086_7d60;
/// `5000.0` (`01030020`, a `float`): the door search radius; and the door
/// callbacks `0090d480` (lock) and `0090d570` (unlock) of this unit.
const DOOR_SEARCH_RADIUS: u32 = 0x0103_0020;
const LOCK_DOORS_CALLBACK: u32 = 0x0090_d480;
const UNLOCK_DOORS_CALLBACK: u32 = 0x0090_d570;
/// `Actor::IsAlarmed` (engine map, `008a61b0`), `00493bb0` (`thiscall` on
/// the actor; its `AL`), `Actor::EndInterruptPackage` (`00881680`, `RET 4`)
/// and `008840f0` (`thiscall` on a package; its `AL`).
const ACTOR_IS_ALARMED: u32 = 0x008a_61b0;
const ACTOR_TEST_00493BB0: u32 = 0x0049_3bb0;
const ACTOR_END_INTERRUPT_PACKAGE: u32 = 0x0088_1680;
const PACKAGE_TEST_008840F0: u32 = 0x0088_40f0;
/// `0041d8a0` (`thiscall` on a package, plain `RET`): its schedule
/// (`+0x38`), whose hour byte `0086a460` reads (signed; -1 for any hour)
/// and whose duration is the word at `+4` (`00726070`).
const PACKAGE_GET_SCHEDULE: u32 = 0x0041_d8a0;
const SCHEDULE_GET_HOUR: u32 = 0x0086_a460;
const WORD_04: u32 = 0x0072_6070;
/// `00621b00` (`thiscall` on the `ActorPackage`; ST0): the `float` at
/// `+0x10` (its start hour by its use).
const ACTOR_PACKAGE_GET_START_HOUR: u32 = 0x0062_1b00;

/// `EnumReferencesCloseToPoint` around the actor with the door search
/// radius and a door callback (the call `UpdateMissedPackages` and
/// `CheckforNewPackage` make for packages that lock or unlock doors): the
/// actor's position is asked for twice, the second answer first in the
/// argument list, as the exe pushes them.
fn enumerate_doors(e: &mut Engine, actor: u32, callback: u32) {
    let radius = x87_quiet_bits(e.global::<u32>(DOOR_SEARCH_RADIUS));
    let first = e.vcall(actor, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
    let second = e.vcall(actor, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
    let cell = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
    let handler = e.global::<u32>(DATA_HANDLER);
    e.call(
        ENUM_REFERENCES_CLOSE_TO_POINT,
        &args![handler, cell, second, radius, first, radius, callback, actor],
    );
}

/// Moves the actor to the package location's point (`GetLocationCoord`,
/// `SetLocationOnReference`) and returns its cell and world space.
fn move_to_package_location(e: &mut Engine, actor: u32, package: u32) -> (u32, u32) {
    e.with_stack(12, |e, temp| {
        let coord = e
            .call(PACKAGE_GET_LOCATION_COORD, &args![package, temp, actor])
            .u32();
        e.call(REFERENCE_SET_LOCATION, &args![actor, coord]);
    });
    let cell = e
        .call(PACKAGE_GET_LOCATION_CELL, &args![package, actor])
        .u32();
    let space = e
        .call(PACKAGE_GET_LOCATION_WORLD, &args![package, actor])
        .u32();
    (cell, space)
}

// Translated from 0090c3e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0090c3e0 - the compiler's exception frame (FS:[0] chain, handler 00f16cbb, around the new package list) is not translated
/// `LowProcess::UpdateMissedPackages` (Xbox PDB; `RET 8`; nothing in
/// `EAX`): plays out the packages the actor missed (`Actor::GetMissedPacks`
/// into a new list, from the date last processed). Unless a current
/// package is `0067efd0`, a script package or `008b1ff0` (which skips to the
/// end), each missed package whose conditions hold (`TESCondition::IsTrue`
/// with its target reference) becomes current: with `flag` the script
/// action flags 0x200 (the missed package) and 0x400 (the current one) are
/// set; `SetCurrentPackage`, the doors procedure, `SetLocationForPackage(0)`;
/// by type (the `switch` through `0090cbfc`/`0090cbe0`) the actor is placed
/// at the package location or its target (0, 1, 3, 4, 5, 6; a target
/// reference may be activated) or `ProcessGuard` runs (0xe); the doors
/// procedure again, the once-per-day mark (`+0x28c` with the calendar day),
/// and the door callbacks for lock/unlock packages. Then the list is
/// emptied and deleted; with a cell or world space found the actor's angle
/// is set (`FLT_MAX`) when asked, it moves there (`MoveRefToNewSpace`), and
/// the current package and data are cleared. Last the actor's process's
/// locked location is cleared (`SetLockedLocation(0)`).
pub fn low_process_update_missed_packages(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    mobile: Ptr,
    flag: u8,
) {
    let base = this.addr();
    let actor = e
        .call(
            DYNAMIC_CAST,
            &args![mobile, 0u32, MOBILE_OBJECT_TYPE, ACTOR_TYPE, 0u32],
        )
        .u32();
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let skip = (actor == 0 || package != 0)
        && (e.call(PACKAGE_TEST_0067EFD0, &args![package]).bool()
            || e.call(PACKAGE_IS_SCRIPT, &args![package]).bool()
            || e.call(PACKAGE_TEST_008B1FF0, &args![package]).bool());
    if !skip {
        let block = e.call(ALLOCATE, &args![8u32]).u32();
        let list = if block != 0 {
            e.call(PACKAGE_LIST_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        let date = e.call(PROCESS_GET_DATE, &args![this]).u32();
        let date = x87_store32(f64::from(date));
        e.call(ACTOR_GET_MISSED_PACKS, &args![actor, list, date]);
        let mut node = list;
        let mut cell = 0u32;
        let mut space = 0u32;
        let mut set_angle = false;
        if !e.call(LIST_IS_EMPTY, &args![node]).bool() {
            while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
                let head = e.call(LIST_HEAD_ITEM, &args![node]).u32();
                let missed = e.mem.u32(head);
                let mut target = 0u32;
                let package_target = e.call(PACKAGE_GET_TARGET, &args![missed]).u32();
                if package_target != 0 {
                    target = e
                        .call(PACKAGE_TARGET_GET_REFERENCE, &args![package_target])
                        .u32();
                }
                e.vcall(base, SET_LOCKED_LOCATION_SLOT, &args![0u32]);
                let conditions = e.call(PACKAGE_GET_CONDITIONS, &args![missed]).u32();
                if e.call(CONDITION_IS_TRUE, &args![conditions, actor, target])
                    .bool()
                {
                    if flag != 0 {
                        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
                        e.call(SET_ACTION_FLAG, &args![missed, extra, 0x200u32]);
                        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
                        let current = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
                        e.call(SET_ACTION_FLAG, &args![current, extra, 0x400u32]);
                    }
                    e.vcall(base, SET_CURRENT_PACKAGE_SLOT, &args![missed, actor]);
                    e.call(ACTOR_UNLOCK_LOCK_DOORS, &args![actor]);
                    e.vcall(base, SET_LOCATION_FOR_PACKAGE_SLOT, &args![actor, 0u32]);
                    match e.call(PACKAGE_GET_TYPE, &args![missed]).u32() {
                        5 | 6 => {
                            (cell, space) = move_to_package_location(e, actor, missed);
                            let location = e.call(GET_0055B980, &args![missed]).u32();
                            set_angle = location != 0 && {
                                let location = e.call(GET_0055B980, &args![missed]).u32();
                                e.call(PACKAGE_LOCATION_GET_TYPE, &args![location]).u32() == 1
                            };
                        }
                        0 => {
                            let reference = if e.call(PACKAGE_GET_TARGET, &args![missed]).u32() == 0
                            {
                                None
                            } else {
                                let package_target =
                                    e.call(PACKAGE_GET_TARGET, &args![missed]).u32();
                                let reference = e
                                    .call(PACKAGE_TARGET_GET_REFERENCE, &args![package_target])
                                    .u32();
                                (reference != 0
                                    && !e.call(REFERENCE_TEST_00440D80, &args![reference]).bool())
                                .then_some(reference)
                            };
                            match reference {
                                Some(reference) => {
                                    let position =
                                        e.vcall(reference, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
                                    e.call(REFERENCE_SET_LOCATION, &args![actor, position]);
                                    set_angle = true;
                                    if !e.vcall(reference, ACTOR_SLOT_100, &args![]).bool()
                                        && !e
                                            .call(REFERENCE_TEST_00440DA0, &args![reference])
                                            .bool()
                                        && !e
                                            .call(REFERENCE_TEST_00440D80, &args![reference])
                                            .bool()
                                    {
                                        e.call(
                                            REFERENCE_ACTIVATE,
                                            &args![reference, actor, 0u32, 0u32, 1u32],
                                        );
                                    }
                                    cell = e.call(ACTOR_GET_008D6F30, &args![reference]).u32();
                                    space =
                                        e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).u32();
                                }
                                None => {
                                    (cell, space) = move_to_package_location(e, actor, missed);
                                    set_angle = false;
                                }
                            }
                        }
                        3 | 4 => {
                            (cell, space) = move_to_package_location(e, actor, missed);
                            let location = e.call(GET_0055B980, &args![missed]).u32();
                            set_angle = location != 0 && {
                                let location = e.call(GET_0055B980, &args![missed]).u32();
                                e.call(LOCATION_GET_REFERENCE, &args![location]).u32() != 0
                            };
                        }
                        1 => {
                            if e.call(PACKAGE_GET_TARGET, &args![missed]).u32() != 0 {
                                let known = {
                                    let pt = e.call(PACKAGE_GET_TARGET, &args![missed]).u32();
                                    e.call(PACKAGE_TARGET_GET_REFERENCE, &args![pt]).u32() != 0
                                } || {
                                    let pt = e.call(PACKAGE_GET_TARGET, &args![missed]).u32();
                                    e.call(PACKAGE_TARGET_GET_OBJECT, &args![pt]).u32() != 0
                                } || {
                                    let pt = e.call(PACKAGE_GET_TARGET, &args![missed]).u32();
                                    e.call(PACKAGE_TARGET_GET_OBJECT_TYPE, &args![pt]).u32() != 0
                                };
                                if known {
                                    let pt = e.call(PACKAGE_GET_TARGET, &args![missed]).u32();
                                    let reference =
                                        e.call(PACKAGE_TARGET_GET_REFERENCE, &args![pt]).u32();
                                    if reference != 0
                                        && !e
                                            .call(REFERENCE_TEST_00440D80, &args![reference])
                                            .bool()
                                        && reference != e.global::<u32>(PLAYER)
                                    {
                                        let position = e
                                            .vcall(reference, ACTOR_GET_LOCATION_SLOT, &args![])
                                            .u32();
                                        e.call(REFERENCE_SET_LOCATION, &args![actor, position]);
                                        cell = e.call(ACTOR_GET_008D6F30, &args![reference]).u32();
                                        space = e
                                            .call(REFERENCE_GET_WORLD_SPACE, &args![reference])
                                            .u32();
                                        set_angle = false;
                                    }
                                }
                            }
                        }
                        0xe => {
                            e.vcall(base, PROCESS_GUARD_SLOT, &args![actor]);
                        }
                        _ => {}
                    }
                    e.call(ACTOR_UNLOCK_LOCK_DOORS, &args![actor]);
                    if e.call(PACKAGE_GET_ONCE_PER_DAY, &args![missed]).bool() {
                        let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8();
                        e.vcall(actor, ACTOR_SLOT_28C, &args![missed, u32::from(day)]);
                    }
                    if e.call(PACKAGE_TEST_00670EF0, &args![missed]).bool() {
                        enumerate_doors(e, actor, LOCK_DOORS_CALLBACK);
                    } else if e.call(PACKAGE_TEST_00670F10, &args![missed]).bool() {
                        enumerate_doors(e, actor, UNLOCK_DOORS_CALLBACK);
                    }
                }
                node = e.call(WORD_04, &args![node]).u32();
            }
            if list != 0 {
                e.call(LIST_CLEAR, &args![list]);
            }
            if list != 0 {
                e.call(LIST_DELETE, &args![list, 1u32]);
            }
            if cell != 0 || space != 0 {
                if set_angle {
                    let angle = x87_quiet_bits(e.global::<u32>(SEARCH_RADIUS_MAX));
                    e.call(REFERENCE_SET_ANGLE_00575770, &args![actor, angle]);
                }
                e.call(REFERENCE_MOVE_TO_NEW_SPACE, &args![actor, cell, space]);
                e.vcall(base, CLEAR_CURRENT_PACKAGE_SLOT, &args![]);
                e.vcall(base, CLEAR_CURRENT_DATA_SLOT, &args![actor]);
            }
        }
        if node != 0 {
            e.call(LIST_DELETE, &args![node, 1u32]);
        }
    }
    let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
    e.vcall(process, SET_LOCKED_LOCATION_SLOT, &args![0u32]);
}

// Translated from 0090a1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::CheckforNewPackage` (Xbox PDB; `RET 8`): whether the actor
/// changed package. Without `flag` an alarmed actor (or one `00493bb0`
/// holds for) keeps its package (false). Without a current package the
/// schedule's (`SetCurrentPackageFromSchedule`) is taken, the target and
/// target-activated flag cleared. With one, whether it ended: a timed
/// `0067efd0` package once its action is complete, its procedure is 0x36
/// or its type is 3, 4, 5 or 0xc (those also set the script action flag
/// 0x400 and the once-per-day mark) and the hour is outside its schedule
/// (the end hour wraps by 23, as the exe does); an untimed `0067efd0`
/// package once complete or at 0x36 (else `flag` is dropped); an
/// `008b1ff0` package once the actor's slot `+0x26c` holds or it finished
/// (slot `+0x13c`, or 0x36); any other package always. An ended package
/// (or with `flag` a non-script one) is replaced from the schedule (a
/// created package ends its interrupt first and is dropped); getting none
/// returns false, and an `008840f0` package the player shares a cell with
/// is kept (continuing, false) when the new one is another non-interrupt
/// package. On a change: door callbacks for the old and new packages, a
/// target for actor-target packages, the doors procedure, the locked
/// location and package start location reset, the continuing flag,
/// target, run-once package (unless of type 0x1a), action complete, flag
/// 0x40 and target activated cleared, `ObjectList` emptied,
/// `StartNewPackage` and the done-once flag cleared.
pub fn low_process_check_for_new_package(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: Ptr,
    flag: u8,
) -> bool {
    let base = this.addr();
    let current = |e: &mut Engine| e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let mut flag = flag;
    let hour = x87_store32(e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64());
    let mut package = current(e);
    if flag == 0
        && (e.call(ACTOR_IS_ALARMED, &args![actor]).bool()
            || e.call(ACTOR_TEST_00493BB0, &args![actor]).bool())
    {
        return false;
    }
    if current(e) != 0 {
        let doors = {
            let p = current(e);
            e.call(PACKAGE_TEST_00670EF0, &args![p]).bool()
        } || {
            let p = current(e);
            e.call(PACKAGE_TEST_00670F10, &args![p]).bool()
        };
        if doors {
            // The package is kept in a local the function never reads.
            current(e);
        }
    }
    let once_per_day = |e: &mut Engine, package: u32| {
        if package != 0 && e.call(PACKAGE_GET_ONCE_PER_DAY, &args![package]).bool() {
            let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8();
            e.vcall(
                actor.addr(),
                ACTOR_SLOT_28C,
                &args![package, u32::from(day)],
            );
        }
    };
    let mut changed;
    let mut replace = false;
    if package == 0 {
        changed = true;
        low_process_set_current_package_from_schedule(e, this, actor);
        if current(e) == 0 {
            changed = false;
        }
        package = current(e);
        e.vcall(base, SET_TARGET_SLOT, &args![0u32]);
        e.vcall(base, SET_TARGET_ACTIVATED_SLOT, &args![0u32]);
    } else {
        changed = false;
        let timed = {
            let schedule = e.call(PACKAGE_GET_SCHEDULE, &args![package]).u32();
            e.call(WORD_04, &args![schedule]).u32() != 0
        } && e.call(PACKAGE_TEST_0067EFD0, &args![package]).bool();
        if timed {
            let mut done = false;
            let mut run_script = false;
            let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
            if e.vcall(process, GET_CURRENT_ACTION_COMPLETE_SLOT, &args![])
                .bool()
                || running_procedure(e, base, package) == PROCEDURE_DONE
            {
                done = true;
            } else if [5u32, 4, 3, 0xc]
                .iter()
                .any(|&kind| e.call(PACKAGE_GET_TYPE, &args![package]).u32() == kind)
            {
                done = true;
                run_script = true;
            }
            if done {
                let schedule = e.call(PACKAGE_GET_SCHEDULE, &args![package]).u32();
                let start = e.call(SCHEDULE_GET_HOUR, &args![schedule]).u8() as i8;
                let mut end = if start == -1 {
                    let start = e
                        .call(
                            ACTOR_PACKAGE_GET_START_HOUR,
                            &args![base.wrapping_add(PACKAGE)],
                        )
                        .f64();
                    let start = e.call(FTOL, &args![start]).i32();
                    let schedule = e.call(PACKAGE_GET_SCHEDULE, &args![package]).u32();
                    start.wrapping_add(e.call(WORD_04, &args![schedule]).i32())
                } else {
                    let schedule = e.call(PACKAGE_GET_SCHEDULE, &args![package]).u32();
                    let start = i32::from(e.call(SCHEDULE_GET_HOUR, &args![schedule]).u8() as i8);
                    let schedule = e.call(PACKAGE_GET_SCHEDULE, &args![package]).u32();
                    start.wrapping_add(e.call(WORD_04, &args![schedule]).i32())
                };
                if end > 0x17 {
                    end = end.wrapping_sub(0x17);
                } else {
                    let schedule = e.call(PACKAGE_GET_SCHEDULE, &args![package]).u32();
                    let start = i32::from(e.call(SCHEDULE_GET_HOUR, &args![schedule]).u8() as i8);
                    if x87_load32(hour) < f64::from(start) {
                        changed = true;
                    }
                }
                if e.call(FTOL, &args![x87_load32(hour)]).i32() >= end {
                    changed = true;
                }
                if !changed && flag != 0 && !e.vcall(actor.addr(), ACTOR_SLOT_26C, &args![]).bool()
                {
                    flag = 0;
                }
                if run_script {
                    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
                    e.call(SET_ACTION_FLAG, &args![package, extra, 0x400u32]);
                    once_per_day(e, package);
                }
            }
        } else if e.call(PACKAGE_TEST_0067EFD0, &args![package]).bool() {
            let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
            let finished = e
                .vcall(process, GET_CURRENT_ACTION_COMPLETE_SLOT, &args![])
                .bool()
                || (e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32() != u32::MAX
                    && running_procedure(e, base, package) == PROCEDURE_DONE);
            if finished {
                once_per_day(e, package);
                changed = true;
            } else {
                changed = false;
                flag = 0;
            }
        } else if e.call(PACKAGE_TEST_008B1FF0, &args![package]).bool() {
            if e.vcall(actor.addr(), ACTOR_SLOT_26C, &args![]).bool() {
                changed = true;
            } else if e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32() == u32::MAX {
                flag = 0;
            } else {
                let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                let finished = e
                    .vcall(
                        package,
                        PACKAGE_SLOT_13C,
                        &args![actor, 0u32, minus_one, 0u32],
                    )
                    .bool()
                    || running_procedure(e, base, package) == PROCEDURE_DONE;
                if finished {
                    if running_procedure(e, base, package) == PROCEDURE_DONE {
                        once_per_day(e, package);
                    }
                    changed = true;
                } else {
                    flag = 0;
                }
            }
        } else {
            changed = true;
        }
        replace = changed || (flag != 0 && !e.call(PACKAGE_IS_SCRIPT, &args![package]).bool());
    }
    if replace {
        if e.call(PACKAGE_GET_IS_CREATED, &args![package]).bool() {
            e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
            if current(e) != 0 && current(e) == package {
                e.vcall(base, CLEAR_CURRENT_DATA_SLOT, &args![actor]);
                e.vcall(base, CLEAR_CURRENT_PACKAGE_SLOT, &args![]);
            }
            package = 0;
        }
        if !low_process_set_current_package_from_schedule(e, this, actor) {
            return false;
        }
        let new_package = current(e);
        if package != 0
            && package != new_package
            && (new_package == 0 || !e.call(PACKAGE_IS_INTERRUPT, &args![new_package]).bool())
        {
            if e.call(PACKAGE_TEST_008840F0, &args![package]).bool() && {
                let player = e.global::<u32>(PLAYER);
                let player_cell = e.call(ACTOR_GET_008D6F30, &args![player]).u32();
                player_cell == e.call(ACTOR_GET_008D6F30, &args![actor]).u32()
            } {
                if e.vcall(base, GET_PROCEDURE_INDEX_RUNNING_SLOT, &args![])
                    .u32()
                    != 0
                {
                    e.vcall(base, SET_CONTINUING_PACKAGE_SLOT, &args![1u32]);
                }
                e.vcall(base, SET_CURRENT_PACKAGE_SLOT, &args![package, actor]);
                return false;
            }
        } else {
            changed = false;
        }
    }
    if changed {
        if package != 0 && current(e) != package {
            if e.call(PACKAGE_TEST_00670EF0, &args![package]).bool() {
                enumerate_doors(e, actor.addr(), LOCK_DOORS_CALLBACK);
            } else if e.call(PACKAGE_TEST_00670F10, &args![package]).bool() {
                enumerate_doors(e, actor.addr(), UNLOCK_DOORS_CALLBACK);
            }
        }
        if current(e) != 0 && current(e) != package {
            let p = current(e);
            if e.call(PACKAGE_TEST_005394A0, &args![p]).bool() {
                enumerate_doors(e, actor.addr(), LOCK_DOORS_CALLBACK);
            } else {
                let p = current(e);
                if e.call(PACKAGE_TEST_00670ED0, &args![p]).bool() {
                    enumerate_doors(e, actor.addr(), UNLOCK_DOORS_CALLBACK);
                }
            }
        }
        if current(e) != 0 {
            let keep =
                package == current(e) && e.vcall(actor.addr(), ACTOR_SLOT_2C8, &args![]).u32() != 0;
            if !keep {
                let p = current(e);
                if e.call(PACKAGE_IS_TARGET_AN_ACTOR, &args![p, actor]).bool() && {
                    let p = current(e);
                    e.call(PACKAGE_GET_TYPE, &args![p]).u32() != 0
                } {
                    let p = current(e);
                    let target = e
                        .call(PACKAGE_GET_ACTOR_TARGET, &args![p, actor, 0u32])
                        .u32();
                    e.vcall(base, SET_TARGET_SLOT, &args![target]);
                }
            }
        }
        e.call(ACTOR_UNLOCK_LOCK_DOORS, &args![actor]);
        e.vcall(base, SET_LOCKED_LOCATION_SLOT, &args![0u32]);
        let object = e.call(ACTOR_GET_00430830, &args![actor]).u32();
        let angle = x87_quiet_bits(e.mem.u32(object.wrapping_add(8)));
        let position = e
            .vcall(actor.addr(), ACTOR_GET_LOCATION_SLOT, &args![])
            .u32();
        let cell = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
        let world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
        e.call(
            REFERENCE_SET_PACKAGE_START_LOCATION,
            &args![actor, world_space, cell, position, angle],
        );
        e.vcall(base, SET_CONTINUING_PACKAGE_SLOT, &args![0u32]);
        e.vcall(base, SET_TARGET_SLOT, &args![0u32]);
        let run_once = e.vcall(base, GET_RUN_ONCE_PACKAGE_SLOT, &args![]).u32();
        if run_once != 0 && e.call(PACKAGE_GET_TYPE, &args![run_once]).u32() != 0x1a {
            e.vcall(base, CLEAR_RUN_ONCE_PACKAGE_SLOT, &args![]);
        }
        e.vcall(base, SET_CURRENT_ACTION_COMPLETE_SLOT, &args![0u32]);
        fn_0090ac20(e, this, 0);
        e.vcall(base, SET_TARGET_ACTIVATED_SLOT, &args![0u32]);
        let list = base.wrapping_add(OBJECT_LIST);
        while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
            let head = e.call(LIST_HEAD_ITEM, &args![list]).u32();
            let item = e.mem.u32(head);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), item);
                if item != 0 {
                    e.call(DELETE_EMPTY_DESTRUCTOR, &args![item, 1u32]);
                }
                e.call(LIST_REMOVE_ITEM, &args![list, slot]);
            });
        }
    }
    if changed {
        let p = current(e);
        e.vcall(base, START_NEW_PACKAGE_SLOT, &args![actor, p]);
        e.vcall(base, SET_PACKAGE_DONE_ONCE_SLOT, &args![0u32]);
    }
    changed
}

/// More process vtable slots used by `Update` (Xbox PDB `BaseProcess`
/// names below `+0x7b4`; the PC-only ones after the function the
/// `LowProcess` table holds there).
const COMPUTE_LAST_TIME_PROCESSED_SLOT: u32 = 0x28;
const PROCESS_REMOVE_WORN_SLOT: u32 = 0xc4;
const CREATE_FOLLOW_FOR_ESCORT_SLOT: u32 = 0xc8;
const GET_LOCKED_LOCATION_SLOT: u32 = 0x1bc;
const SET_IS_AGGRESSOR_SLOT: u32 = 0x1c8;
const PROCESS_FOLLOW_SLOT: u32 = 0x298;
const PROCESS_PURSUE_SLOT: u32 = 0x29c;
const PROCESS_FLEE_SLOT: u32 = 0x2a0;
const PROCESS_ALARM_SLOT: u32 = 0x7c4;
const PROCESS_ACCOMPANY_SLOT: u32 = 0x7cc;
const PROCESS_ACQUIRE_SLOT: u32 = 0x7d0;
const PROCESS_AMBUSH_WAIT_SLOT: u32 = 0x7d4;
const PROCESS_USE_ITEM_AT_SLOT: u32 = 0x7d8;
const PROCESS_USE_ITEM_AT_2_SLOT: u32 = 0x7dc;
/// `+0x7e0`: empty (`004534f0`) in the `LowProcess` table.
const PROCESS_SLOT_7E0: u32 = 0x7e0;
const PROCESS_FLEE_NON_COMBAT_SLOT: u32 = 0x7e4;
const PROCESS_SANDBOX_SLOT: u32 = 0x7ec;
const PROCESS_MOVEMENT_BLOCKED_SLOT: u32 = 0x7f0;
const PROCESS_PATROL_SLOT: u32 = 0x7fc;
const PROCESS_SUMMON_CREATURE_DEFEND_SLOT: u32 = 0x800;
const PROCESS_AVOID_AREA_SLOT: u32 = 0x808;
/// The actor's vtable slots used by `Update` (the `Character` table's
/// functions): `+0x274` (`008a0270`, one word), `+0x434`
/// (`Actor::StopCombat`, one word), `+0x16c` (`008a8290`, the address of a
/// result), `+0x2c4` (`008a6a00`, a `float`: the angle).
const ACTOR_SLOT_274: u32 = 0x274;
const ACTOR_STOP_COMBAT_SLOT: u32 = 0x434;
const ACTOR_SLOT_16C: u32 = 0x16c;
const ACTOR_SET_ANGLE_SLOT: u32 = 0x2c4;
/// `00483710` (`thiscall`, plain `RET`): an empty function.
const EMPTY_00483710: u32 = 0x0048_3710;
/// The object `Update` calls the empty `00483710` on at its start
/// and end.
const UPDATE_MARKER: u32 = 0x011e_0361;
/// `008a8f60` (`thiscall` on the actor, plain `RET`): run after a new
/// package.
const ACTOR_AFTER_CHECK_008A8F60: u32 = 0x008a_8f60;
/// `00670f40` / `00670f60` (`thiscall` on a package; their `AL`): its
/// lock / unlock door flags at a location.
const PACKAGE_TEST_00670F40: u32 = 0x0067_0f40;
const PACKAGE_TEST_00670F60: u32 = 0x0067_0f60;
/// `Actor::IsWaitingOnPath` (engine map, `008b3bf0`).
const ACTOR_IS_WAITING_ON_PATH: u32 = 0x008b_3bf0;
/// `bSkippedTimeStampForPathing` (Xbox PDB) +0xb0.
const SKIPPED_TIME_STAMP: u32 = 0xb0;
/// The debug messages for the console target: `Interface::GetTargetREFR`
/// (`00703180`), the procedure-type name table (`0119bcb0`) through
/// `00464f30` (`cdecl`, two words), the reference's name (`0055d520`),
/// `sprintf` (`00ec623a`) into a stack buffer, and the console print
/// `00703c00` (`cdecl`); the formats "%s is sleeping with %s"
/// (`01088f2c`), "%s is eating with %s" (`01088f14`) and "%s is done with
/// %s" (`01088f00`).
const INTERFACE_GET_TARGET: u32 = 0x0070_3180;
const PROCEDURE_TYPE_NAMES: u32 = 0x0119_bcb0;
const NAME_00464F30: u32 = 0x0046_4f30;
const REFERENCE_GET_NAME: u32 = 0x0055_d520;
const SPRINTF: u32 = 0x00ec_623a;
const CONSOLE_PRINT: u32 = 0x0070_3c00;
const SLEEPING_FORMAT: u32 = 0x0108_8f2c;
const EATING_FORMAT: u32 = 0x0108_8f14;
const DONE_FORMAT: u32 = 0x0108_8f00;
/// The size of the stack buffer the messages are printed into.
const MESSAGE_BUFFER_SIZE: u32 = 0x100;
/// The settings `Update` reads for the wander radius (`011cdbac` in cells
/// where `00425fd0` holds, else `011cd82c` times the package target's
/// integer) and the default count 200.
const WANDER_RADIUS_SETTING: u32 = 0x011c_dbac;
const WANDER_SCALE_SETTING: u32 = 0x011c_d82c;
const WANDER_DEFAULT_COUNT: i32 = 200;

/// The console message for the console target: `format` with the
/// actor's name and the current package's procedure-type name.
fn print_procedure_message(e: &mut Engine, this: u32, actor: u32, format: u32) {
    if e.call(INTERFACE_GET_TARGET, &args![]).u32() != actor {
        return;
    }
    let package = e.vcall(this, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let kind = e.call(PACKAGE_GET_TYPE, &args![package]).u32();
    let entry = e
        .mem
        .u32(PROCEDURE_TYPE_NAMES.wrapping_add(kind.wrapping_mul(4)));
    let type_name = e.call(NAME_00464F30, &args![entry, 0u32]).u32();
    let name = e.call(REFERENCE_GET_NAME, &args![actor]).u32();
    e.with_stack(MESSAGE_BUFFER_SIZE, |e, buffer| {
        e.call(SPRINTF, &args![buffer, format, name, type_name]);
        e.call(CONSOLE_PRINT, &args![buffer]);
    });
}

/// `Update`'s door and angle steps of procedure 0 (see there).
fn update_location_doors(e: &mut Engine, this: u32, actor: u32) {
    let current = |e: &mut Engine| e.vcall(this, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    if !e.vcall(this, GET_LOCKED_LOCATION_SLOT, &args![]).bool() {
        let package = current(e);
        if e.call(PACKAGE_TEST_00670F40, &args![package]).bool() {
            enumerate_doors(e, actor, LOCK_DOORS_CALLBACK);
        }
        e.vcall(this, SET_LOCKED_LOCATION_SLOT, &args![1u32]);
    }
    let package = current(e);
    if e.call(PACKAGE_TEST_00670F60, &args![package]).bool() {
        enumerate_doors(e, actor, UNLOCK_DOORS_CALLBACK);
    }
}

/// `Update`'s angle step of procedure 0: for a generic location whose base
/// form is `011ca248`, or a package location of type 3, the actor takes the
/// angle of the location (the float at +8 of `00430830`) or, without a
/// location, of its own slot `+0x16c` result (slot `+0x2c4`).
fn update_location_angle(e: &mut Engine, this: u32, actor: u32, location: u32) {
    let current = |e: &mut Engine| e.vcall(this, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let furniture = location != 0
        && e.call(REFERENCE_GET_BASE_FORM, &args![location]).u32() == e.global::<u32>(ANGLED_FORM);
    let angled = furniture || {
        let package = current(e);
        e.call(GET_0055B980, &args![package]).u32() != 0 && {
            let package = current(e);
            let package_location = e.call(GET_0055B980, &args![package]).u32();
            e.call(PACKAGE_LOCATION_GET_TYPE, &args![package_location])
                .u32()
                == 3
        }
    };
    if !angled {
        return;
    }
    let angle = if location != 0 {
        let object = e.call(ACTOR_GET_00430830, &args![location]).u32();
        x87_quiet_bits(e.mem.u32(object.wrapping_add(8)))
    } else {
        e.with_stack(12, |e, result| {
            let point = e.vcall(actor, ACTOR_SLOT_16C, &args![result]).u32();
            x87_quiet_bits(e.mem.u32(point.wrapping_add(8)))
        })
    };
    e.vcall(actor, ACTOR_SET_ANGLE_SLOT, &args![angle]);
}

/// `Update`'s procedure 13 (a timed wander step; see there).
fn update_wander(e: &mut Engine, this: Ptr<LowProcess>, actor: u32, package: u32) {
    let base = this.addr();
    let step = |e: &mut Engine| {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 1u32],
        );
    };
    if e.mem.u32(base.wrapping_add(TARGET)) == 0 {
        e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
    }
    let target = e.mem.u32(base.wrapping_add(TARGET));
    if target == 0 || e.call(REFERENCE_TEST_00440D80, &args![target]).bool() {
        step(e);
        return;
    }
    if e.call(PACKAGE_TEST_008A6290, &args![package]).bool() {
        let player = e.global::<u32>(PLAYER);
        if e.call(PLAYER_TEST_00962720, &args![player, actor]).bool() {
            return;
        }
        let player = e.global::<u32>(PLAYER);
        let count = e.call(PLAYER_COUNT_00962620, &args![player]).i32();
        let value = e
            .call(SETTING_VALUE_0043D4D0, &args![FOLLOW_LIMIT_SETTING])
            .u32();
        if count < e.mem.i32(value) {
            let current = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
            e.call(PACKAGE_SET_00674F30, &args![current, 0u32]);
        }
        return;
    }
    if e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 0x1c
        || e.call(PACKAGE_TEST_008A6290, &args![package]).bool()
    {
        return;
    }
    if e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 1 {
        let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
        let value = e
            .call(PACKAGE_TARGET_GET_VALUE, &args![package_target])
            .i32();
        let reach = x87_store32(f64::from(value));
        let target = e.mem.u32(base.wrapping_add(TARGET));
        let distance = e
            .call(REFERENCE_GET_DISTANCE, &args![actor, target, 0u32, 0u32])
            .f64();
        if x87_load32(reach) >= distance {
            step(e);
        }
        return;
    }
    // A radius is worked out here and never used (the exe's dead store).
    let mut radius = 0u32;
    let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    let mut count = e
        .call(PACKAGE_TARGET_GET_VALUE, &args![package_target])
        .i32();
    if count <= 0 {
        count = WANDER_DEFAULT_COUNT;
    }
    let zero: f64 = e.global(ZERO);
    let mut found = false;
    if x87_load32(radius) == zero {
        let cell = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
        if cell != 0 {
            let cell = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
            if e.call(TEST_00425FD0, &args![cell]).bool() {
                let value = e
                    .call(SETTING_VALUE_00403E20, &args![WANDER_RADIUS_SETTING])
                    .u32();
                radius = x87_quiet_bits(e.mem.u32(value));
                found = true;
            }
        }
    }
    if !found && x87_load32(radius) == zero {
        let scale = f64::from(count);
        let value = e
            .call(SETTING_VALUE_00403E20, &args![WANDER_SCALE_SETTING])
            .u32();
        radius = x87_store32(x87_mul(x87_load32(e.mem.u32(value)), scale));
    }
    let _ = radius;
    step(e);
}

// Translated from 0090ad40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0090ad40 - the /GS cookie check (00ec408c) on the stack frame is not translated
/// `LowProcess::Update` (Xbox PDB; `RET 4`): one update of the low process
/// for the mobile object. Without an actor (`mobile`'s slot `+0x100`) only
/// the process's `ComputeLastTimeProcessed` runs. With one: a resting
/// player's combat with it ends (`StopCombat(0)`, when its slot `+0x274`
/// (1) holds); `CheckforNewPackage(actor, 0)` (vtable `+0x24`) and, on a
/// change, the current data cleared, `008a8f60`, the package location set
/// and the items to activate counted. For a current package with a
/// procedure type and without flag 0x10000 (`008a6290`) the target is
/// reset and the running procedure number (`0`-`0x33`; the `switch` through
/// `0090c308`/`0090c28c`, its case for 7 through `0090c344`/`0090c33c`)
/// runs its step: 0 travels to and uses the package location (doors,
/// angle), 1 for procedure type 0x1a steps back, 2 and 49 activate (49 for
/// an actor target whose package's slot `+0x140` holds or with no second
/// location), 3 acquires, 4 and 5 print the sleeping/eating messages for
/// the console target, 6 follows, 7 escorts (package types 1, 2, 7, 0x16),
/// 8 alarms, 10 flees, 12 ends an interrupt package, 13 wanders, 14
/// activates without acquiring, 15 pursues, 17 escorts a target actor, and
/// the others their process slots (accompany, use item at, ambush, flee
/// non-combat, remove worn, summon, sandbox, patrol, guard, movement
/// blocked, avoid area; 41 steps by 2). After the step, for actors of
/// process type 3 at the done procedure 0x36: the package is finished
/// (script flag, `PackageDone`, done once), the aggressor flag cleared, a
/// type-0 package that fails its slot `+0x13c` steps back and returns, the
/// once-per-day mark set, an interrupt package ended, the "done" message
/// printed, the acquire state freed, and an untimed package replaced from
/// the schedule (the running index reset when it changed). Last, unless the
/// actor waits on its path with no skipped time stamp (which sets the
/// flag), the actor's process computes its last time processed and the
/// flag is cleared.
pub fn low_process_update(e: &mut Engine, this: Ptr<LowProcess>, mobile: Ptr) {
    let base = this.addr();
    let current = |e: &mut Engine| e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    e.call(EMPTY_00483710, &args![UPDATE_MARKER]);
    if mobile.addr() == 0 {
        return;
    }
    // The calendar hour is read and stored, its value unused.
    let _hour = x87_store32(e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64());
    let actor = if e.vcall(mobile.addr(), ACTOR_SLOT_100, &args![]).bool() {
        mobile.addr()
    } else {
        0
    };
    if actor == 0 {
        let process = e.call(ACTOR_GET_PROCESS, &args![mobile]).u32();
        e.vcall(process, COMPUTE_LAST_TIME_PROCESSED_SLOT, &args![]);
        e.call(EMPTY_00483710, &args![UPDATE_MARKER]);
        return;
    }
    if e.vcall(actor, ACTOR_SLOT_274, &args![1u32]).bool() {
        let player = e.global::<u32>(PLAYER);
        if e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool() {
            e.vcall(actor, ACTOR_STOP_COMBAT_SLOT, &args![0u32]);
        }
    }
    let mut package = current(e);
    if e.vcall(base, CHECK_FOR_NEW_PACKAGE_SLOT, &args![actor, 0u32])
        .bool()
    {
        e.vcall(base, CLEAR_CURRENT_DATA_SLOT, &args![actor]);
        package = current(e);
        e.call(ACTOR_AFTER_CHECK_008A8F60, &args![actor]);
        e.vcall(base, SET_LOCATION_FOR_PACKAGE_SLOT, &args![actor, 0u32]);
        let new_package = current(e);
        if new_package != 0 {
            let package_target = e.call(PACKAGE_GET_TARGET, &args![new_package]).u32();
            if package_target != 0
                && e.call(PACKAGE_TARGET_GET_TYPE, &args![package_target])
                    .u32()
                    != 0
                && e.call(PACKAGE_TARGET_GET_TYPE, &args![package_target])
                    .u32()
                    != 3
            {
                let count = e
                    .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![new_package])
                    .u32();
                e.mem
                    .set_u32(base.wrapping_add(NUMBER_ITEMS_ACTIVATE), count);
            }
        }
    }
    let skipped = e.mem.u8(base.wrapping_add(SKIPPED_TIME_STAMP)) != 0
        && e.call(ACTOR_TEST_008B3B90, &args![actor]).bool();
    let mut runs = current(e) != 0;
    if runs {
        let p = current(e);
        runs = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![p]).u32() != u32::MAX;
    }
    if runs {
        let p = current(e);
        runs = !e.call(PACKAGE_TEST_008A6290, &args![p]).bool();
    }
    if runs {
        low_process_reset_target(e, this);
        let p = current(e);
        let procedure = running_procedure(e, base, p);
        let process_slot = |e: &mut Engine, slot: u32| {
            e.vcall(base, slot, &args![actor]);
        };
        let step = |e: &mut Engine, delta: u32| {
            e.vcall(
                base,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, delta],
            );
        };
        match procedure {
            0 => {
                let location = e.mem.u32(base.wrapping_add(GENERIC_LOCATION));
                let p = current(e);
                let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                if e.vcall(p, PACKAGE_SLOT_13C, &args![actor, 0u32, minus_one, 0u32])
                    .bool()
                {
                    update_location_doors(e, base, actor);
                    update_location_angle(e, base, actor, location);
                    step(e, 1);
                } else {
                    if e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
                        let p = current(e);
                        let world = e.call(PACKAGE_GET_LOCATION_WORLD, &args![p, actor]).u32();
                        let p = current(e);
                        let cell = e.call(PACKAGE_GET_LOCATION_CELL, &args![p, actor]).u32();
                        let reached = e.with_stack(12, |e, temp| {
                            let p = current(e);
                            let coord = e
                                .call(PACKAGE_GET_LOCATION_COORD, &args![p, temp, actor])
                                .u32();
                            e.call(
                                ACTOR_SET_PATHFINDING_GOAL_TO_LOCATION,
                                &args![actor, coord, cell, world, minus_one, 0u32],
                            )
                            .bool()
                        });
                        if !reached {
                            return;
                        }
                    }
                    // The time delta is computed and stored, its value unused.
                    low_process_get_time_delta(e, this, actor);
                    let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
                    if e.call(PROCESS_GET_0045CD60, &args![process]).u32() == 3 {
                        let p = current(e);
                        if e.vcall(p, PACKAGE_SLOT_13C, &args![actor, 0u32, minus_one, 0u32])
                            .bool()
                        {
                            update_location_doors(e, base, actor);
                            step(e, 1);
                            update_location_angle(e, base, actor, location);
                        }
                    }
                }
            }
            1 => {
                let p = current(e);
                if e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![p]).u32() == 0x1a {
                    step(e, u32::MAX);
                }
            }
            2 => {
                e.vcall(base, PROCESS_ACTIVATE_SLOT, &args![actor, 1u32]);
            }
            3 => process_slot(e, PROCESS_ACQUIRE_SLOT),
            4 => print_procedure_message(e, base, actor, SLEEPING_FORMAT),
            5 => print_procedure_message(e, base, actor, EATING_FORMAT),
            6 => {
                e.vcall(
                    base,
                    PROCESS_FOLLOW_SLOT,
                    &args![actor, 1u32, 0x101u32, 0u32],
                );
            }
            7 => {
                let p = current(e);
                if matches!(e.call(PACKAGE_GET_TYPE, &args![p]).u32(), 1 | 2 | 7 | 0x16) {
                    process_slot(e, PROCESS_ESCORT_SLOT);
                }
            }
            8 => process_slot(e, PROCESS_ALARM_SLOT),
            10 => process_slot(e, PROCESS_FLEE_SLOT),
            12 => {
                e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
            }
            13 => update_wander(e, this, actor, package),
            14 => {
                e.vcall(base, PROCESS_ACTIVATE_SLOT, &args![actor, 0u32]);
            }
            15 => {
                e.vcall(
                    base,
                    PROCESS_PURSUE_SLOT,
                    &args![actor, 0u32, 1u32, 0x101u32],
                );
            }
            17 => {
                if e.mem.u32(base.wrapping_add(TARGET)) == 0 {
                    e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
                }
                let target = e.mem.u32(base.wrapping_add(TARGET));
                if target != 0
                    && target != e.global::<u32>(PLAYER)
                    && e.vcall(target, ACTOR_SLOT_100, &args![]).bool()
                {
                    let p = current(e);
                    e.vcall(base, CREATE_FOLLOW_FOR_ESCORT_SLOT, &args![actor, p, 1u32]);
                }
                step(e, 1);
            }
            26 => process_slot(e, PROCESS_AVOID_AREA_SLOT),
            27 => process_slot(e, PROCESS_ACCOMPANY_SLOT),
            28 => process_slot(e, PROCESS_USE_ITEM_AT_SLOT),
            30 => process_slot(e, PROCESS_AMBUSH_WAIT_SLOT),
            33 => process_slot(e, PROCESS_SLOT_7E0),
            34 => process_slot(e, PROCESS_FLEE_NON_COMBAT_SLOT),
            35 => {
                e.vcall(base, PROCESS_REMOVE_WORN_SLOT, &args![actor, 1u32]);
            }
            38 => process_slot(e, PROCESS_SUMMON_CREATURE_DEFEND_SLOT),
            41 => step(e, 2),
            42 => process_slot(e, PROCESS_SANDBOX_SLOT),
            44 => process_slot(e, PROCESS_PATROL_SLOT),
            47 => process_slot(e, PROCESS_GUARD_SLOT),
            49 => {
                let target = e.mem.u32(base.wrapping_add(TARGET));
                let actor_target =
                    if target != 0 && e.vcall(target, ACTOR_SLOT_100, &args![]).bool() {
                        e.mem.u32(base.wrapping_add(TARGET))
                    } else {
                        0
                    };
                if actor_target != 0 {
                    let p = current(e);
                    let activate = e.call(PACKAGE_GET_SECOND_LOCATION, &args![p]).u32() == 0 || {
                        let p = current(e);
                        let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                        e.vcall(p, 0x140, &args![actor_target, actor, 0u32, minus_one, 1u32])
                            .bool()
                    };
                    if activate {
                        e.vcall(base, PROCESS_ACTIVATE_SLOT, &args![actor, 1u32]);
                    }
                }
            }
            50 => process_slot(e, PROCESS_USE_ITEM_AT_2_SLOT),
            51 => process_slot(e, PROCESS_MOVEMENT_BLOCKED_SLOT),
            _ => {}
        }
        if e.call(MOBILE_GET_PROCESS_TYPE, &args![actor]).u32() == 3 && current(e) != 0 && {
            let p = current(e);
            running_procedure(e, base, p) == PROCEDURE_DONE
        } {
            if !e.vcall(base, IS_PACKAGE_DONE_ONCE_SLOT, &args![]).bool() {
                let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
                let p = current(e);
                e.call(SET_ACTION_FLAG, &args![p, extra, 0x400u32]);
                let p = current(e);
                e.vcall(base, PACKAGE_DONE_SLOT, &args![actor, p]);
                e.vcall(base, SET_PACKAGE_DONE_ONCE_SLOT, &args![1u32]);
            }
            e.vcall(base, SET_IS_AGGRESSOR_SLOT, &args![0u32]);
            if current(e) != 0 && {
                let p = current(e);
                e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![p]).u32() == 0
            } {
                let p = current(e);
                let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                if !e
                    .vcall(p, PACKAGE_SLOT_13C, &args![actor, 0u32, minus_one, 0u32])
                    .bool()
                {
                    step(e, u32::MAX);
                    return;
                }
            }
            if current(e) != 0 && {
                let p = current(e);
                e.call(PACKAGE_GET_ONCE_PER_DAY, &args![p]).bool()
            } {
                let day = e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8();
                let p = current(e);
                e.vcall(actor, ACTOR_SLOT_28C, &args![p, u32::from(day)]);
            }
            if current(e) != 0 && {
                let p = current(e);
                e.call(PACKAGE_GET_IS_CREATED, &args![p]).bool()
            } {
                e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
            }
            print_procedure_message(e, base, actor, DONE_FORMAT);
            let object = e.mem.u32(base.wrapping_add(ACQUIRE_OBJECT));
            if object != 0 {
                e.call(DELETE_EMPTY_DESTRUCTOR, &args![object, 1u32]);
            }
            e.mem.set_u32(base.wrapping_add(ACQUIRE_OBJECT), 0);
            let list = base.wrapping_add(OBJECT_LIST);
            while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
                let head = e.call(LIST_HEAD_ITEM, &args![list]).u32();
                let item = e.mem.u32(head);
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), item);
                    if item != 0 {
                        e.call(DELETE_EMPTY_DESTRUCTOR, &args![item, 1u32]);
                    }
                    e.call(LIST_REMOVE_ITEM, &args![list, slot]);
                });
            }
            if current(e) != 0 && {
                let p = current(e);
                let schedule = e.call(PACKAGE_GET_SCHEDULE, &args![p]).u32();
                e.call(WORD_04, &args![schedule]).u32() == 0
            } {
                let old = current(e);
                low_process_set_current_package_from_schedule(e, this, Ptr::new(actor));
                package = current(e);
                if old != current(e) {
                    e.vcall(base, SET_PROCEDURE_INDEX_RUNNING_SLOT, &args![0u32]);
                }
            }
        }
    }
    let _ = package;
    if !e.call(ACTOR_IS_WAITING_ON_PATH, &args![actor]).bool() || skipped {
        let process = e.call(ACTOR_GET_PROCESS, &args![mobile]).u32();
        e.vcall(process, COMPUTE_LAST_TIME_PROCESSED_SLOT, &args![]);
        e.mem.set_u8(base.wrapping_add(SKIPPED_TIME_STAMP), 0);
    } else {
        e.mem.set_u8(base.wrapping_add(SKIPPED_TIME_STAMP), 1);
    }
    e.call(EMPTY_00483710, &args![UPDATE_MARKER]);
}

/// `ActorValue::CheckClampDamageModifier` (engine map, `0066eea0`; `cdecl`: owner, actor
/// value, `float`; in `ST0`) and `ModifierList::AdjustModifier` (`00937480`; `thiscall`:
/// actor value byte, `float`, kind).
const CHECK_CLAMP_DAMAGE_MODIFIER: u32 = 0x0066_eea0;
const MODIFIER_LIST_ADJUST_MODIFIER: u32 = 0x0093_7480;
/// The actor value `DamageModActorValue` adjusts (0x10).
const DAMAGED_ACTOR_VALUE: u32 = 0x10;
/// `0084d030` (unnamed, engine map unit `bgssaveloadglobaldata.cpp`; `thiscall` on
/// `011f6394`, no stack words; in `ST0`): the time step the essential down timer loses.
const GLOBAL_TIME_STEP: u32 = 0x0084_d030;
const GLOBAL_TIME_OBJECT: u32 = 0x011f_6394;
/// `ExtraDataList::SetPackageExtraTarget` (engine map, `0041cab0`; `thiscall`, one stack
/// word).
const EXTRA_SET_PACKAGE_TARGET: u32 = 0x0041_cab0;
/// The package type `UpdatePackageTargetForCombat` handles (0xd, patrol by the code it
/// runs).
const PATROL_PACKAGE_TYPE: u32 = 0xd;
/// The actor's vtable slot `+0x424` (eight stack words): starts combat.
const ACTOR_START_COMBAT_SLOT: u32 = 0x424;

// Translated from 00907460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::DamageModActorValue` (engine map; `thiscall`, three stack words: actor, actor
/// value, amount, `RET 0xc`): only for actor value 0x10 and unless it has flag 0x100, the
/// damage modifiers (`+0x94`) are adjusted (`ModifierList::AdjustModifier`, kind 0) by
/// `ActorValue::CheckClampDamageModifier` of the actor's value owner (`actor + 0xa4`, 0 for
/// no actor), the value and `amount` (stored as a `float`).
pub fn low_process_damage_mod_actor_value(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    actor_value: u32,
    amount: f32,
) {
    if e.call(ACTOR_VALUE_HAS_FLAG, &args![actor_value, 0x100u32])
        .bool()
    {
        return;
    }
    if actor_value != DAMAGED_ACTOR_VALUE {
        return;
    }
    let owner = if actor != 0 {
        actor + ACTOR_VALUE_OWNER
    } else {
        0
    };
    let clamped = e
        .call(
            CHECK_CLAMP_DAMAGE_MODIFIER,
            &args![owner, actor_value, x87_quiet_bits(amount.to_bits())],
        )
        .f64();
    let clamped = x87_store32(clamped);
    e.call(
        MODIFIER_LIST_ADJUST_MODIFIER,
        &args![
            this.addr() + DAMAGE_MODIFIERS,
            actor_value & 0xff,
            x87_quiet_bits(clamped),
            0u32
        ],
    );
}

// Translated from 00907620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ModEssentialDownTimer` (engine map; `thiscall`, no stack words):
/// `fEssentialDownTimer` loses the time step of `0084d030(011f6394)` (`FSUBR`, stored as a
/// `float`).
pub fn low_process_mod_essential_down_timer(e: &mut Engine, this: Ptr<LowProcess>) {
    let step = e.call(GLOBAL_TIME_STEP, &args![GLOBAL_TIME_OBJECT]).f64();
    let timer = e.mem.u32(this.addr() + ESSENTIAL_DOWN_TIMER);
    let left = x87_store32(x87_sub(x87_m32(timer), step));
    e.mem.set_u32(this.addr() + ESSENTIAL_DOWN_TIMER, left);
}

// Translated from 00907960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ShouldStopCombatControllerToFinishPackage` (engine map; `thiscall`, two stack
/// words: the package and a procedure index, `RET 8`). By the package's type (`0041ca90`;
/// the exe's byte map `009079d0` and jump table `009079c8`): types 0, 1, 6, 7, 13, 14 and 17
/// answer `false`; the others (and any type above 0x11) answer `false` only when the package
/// has a procedure type (`009611e0`, not -1) whose list (`011a3ff0`) holds the "done"
/// procedure 0x36 at `index`, `true` otherwise.
pub fn low_process_should_stop_combat_controller_to_finish_package(
    e: &mut Engine,
    _unused_0: Ptr<LowProcess>,
    package: u32,
    index: u32,
) -> bool {
    let kind = e.call(PACKAGE_GET_TYPE, &args![package]).u32();
    if matches!(kind, 0 | 1 | 6 | 7 | 13 | 14 | 17) {
        return false;
    }
    if e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).i32() == -1 {
        return true;
    }
    let procedure_type = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
    let list = e
        .mem
        .u32(PROCEDURE_LISTS.wrapping_add(procedure_type.wrapping_mul(4)));
    let procedure = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
    procedure != PROCEDURE_DONE
}

/// Hands the patrol's current reference (`009f3390`) to the actor's extra data list as the
/// package's extra target (`ExtraDataList::SetPackageExtraTarget`).
fn set_patrol_extra_target(e: &mut Engine, data: u32, actor: u32) {
    let reference = e.call(PATROL_GET_CURRENT_REFERENCE, &args![data]).u32();
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    e.call(EXTRA_SET_PACKAGE_TARGET, &args![extra, reference]);
}

// Translated from 009079f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::UpdatePackageTargetForCombat` (engine map; `thiscall`, two stack words: actor
/// and package, `RET 8`): only for package type 0xd. The actor's package data
/// (`004212b0` of its extra data list) must exist; when its word `+4` is 0 the patrol data is
/// initialised (`009f2ba0(actor, package)`; on success `+4` becomes 1), otherwise it moves to
/// the next reference (`009f2ea0(1)`); on success the current reference becomes the actor's
/// extra package target and the answer is `true`, else `false`.
pub fn low_process_update_package_target_for_combat(
    e: &mut Engine,
    _unused_0: Ptr<LowProcess>,
    actor: u32,
    package: u32,
) -> bool {
    if e.call(PACKAGE_GET_TYPE, &args![package]).u32() != PATROL_PACKAGE_TYPE {
        return false;
    }
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    let data = e.call(EXTRA_GET_ACTOR_PACKAGE_DATA, &args![extra]).u32();
    if data == 0 {
        return false;
    }
    if e.mem.u32(data + 4) == 0 {
        if !e
            .call(PATROL_DATA_INIT, &args![data, actor, package])
            .bool()
        {
            return false;
        }
        e.mem.set_u32(data + 4, 1);
        set_patrol_extra_target(e, data, actor);
        true
    } else {
        if !e.call(PATROL_SET_NEXT_REFERENCE, &args![data, 1u32]).bool() {
            return false;
        }
        set_patrol_extra_target(e, data, actor);
        true
    }
}

// Translated from 00908f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessAvoidArea` (engine map; `thiscall`, one stack word, `RET 4`): moves
/// the running procedure index on by one (vtable `+0x288`, actor and 1).
pub fn low_process_process_avoid_area(e: &mut Engine, this: Ptr<LowProcess>, actor: u32) {
    e.vcall(
        this.addr(),
        ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
        &args![actor, 1u32],
    );
}

// Translated from 00909c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessFlee` (engine map; `thiscall`, one stack word, `RET 4`): when the
/// current package's slot `+0x13c` (actor, 0, -1.0, 0) fails, [`low_process_get_time_delta`]
/// runs (its answer is stored and not used); when it succeeds, the actor ends its interrupt
/// package (`Actor::EndInterruptPackage(0)`).
pub fn low_process_process_flee(e: &mut Engine, this: Ptr<LowProcess>, actor: u32) {
    let package = e
        .vcall(this.addr(), GET_CURRENT_PACKAGE_SLOT, &args![])
        .u32();
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    if !e
        .vcall(
            package,
            PACKAGE_SLOT_13C,
            &args![actor, 0u32, minus_one, 0u32],
        )
        .bool()
    {
        low_process_get_time_delta(e, this, actor);
    } else {
        e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
    }
}

// Translated from 0090a130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetCurrentPackageLocation` (engine map; `thiscall`, two stack words: the out
/// point and the actor, `RET 8`): the current package's location point
/// (`TESPackage::GetLocationCoord(out, actor)`), or the zero point (`011f426c`) without a
/// package. Answers `out`.
pub fn low_process_get_current_package_location(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    out: u32,
    actor: u32,
) -> u32 {
    if e.vcall(this.addr(), GET_CURRENT_PACKAGE_SLOT, &args![])
        .u32()
        != 0
    {
        let package = e
            .vcall(this.addr(), GET_CURRENT_PACKAGE_SLOT, &args![])
            .u32();
        e.call(PACKAGE_GET_LOCATION_COORD, &args![package, out, actor]);
        return out;
    }
    copy_point(e, out, DESTINATION_DEFAULT_POINT);
    out
}

// Translated from 0090cfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::IsWandering` (engine map; `thiscall`, no stack words): with a current
/// package, whether its type's procedure list (`011a3ff0`) holds procedure 1 at the running
/// procedure index (vtable `+0x280`).
pub fn low_process_is_wandering(e: &mut Engine, this: Ptr<LowProcess>) -> bool {
    let base = this.addr();
    if e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32() == 0 {
        return false;
    }
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
    let index = e
        .vcall(base, GET_PROCEDURE_INDEX_RUNNING_SLOT, &args![])
        .u32();
    let list = e
        .mem
        .u32(PROCEDURE_LISTS.wrapping_add(kind.wrapping_mul(4)));
    e.mem.u32(list.wrapping_add(index.wrapping_mul(4))) == 1
}

// Translated from 0090dc10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::EnterCombat` (engine map; `thiscall`, thirteen stack words, `RET 0x34`;
/// answers `true`): the actor's slot `+0x424` with words 1 and 4, the low bytes of words 9,
/// 7 and 5, 0, the low byte of word 11 and word 12 (counting the actor as word 0); words 2,
/// 3, 6, 8 and 10 are not read.
pub fn low_process_enter_combat(
    e: &mut Engine,
    _unused_0: Ptr<LowProcess>,
    actor: u32,
    word_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    word_4: u32,
    word_5: u8,
    _unused_6: u32,
    word_7: u8,
    _unused_8: u32,
    word_9: u8,
    _unused_10: u32,
    word_11: u8,
    word_12: u32,
) -> bool {
    e.vcall(
        actor,
        ACTOR_START_COMBAT_SLOT,
        &args![
            word_1,
            word_4,
            u32::from(word_9),
            u32::from(word_7),
            u32::from(word_5),
            0u32,
            u32::from(word_11),
            word_12
        ],
    );
    true
}

// Translated from 0090e600 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (engine map unit `lowprocess.cpp`; `thiscall`, no stack words): a constructor
/// of a 0x1c-byte record: the words `+0`, `+4`, `+0xc` and `+0x14` and the bytes `+8` and
/// `+9` cleared, `+0x10` set to 0x7fffffff and `+0x18` to -1. Returns `this`.
pub fn fn_0090e600(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u8(base + 8, 0);
    e.mem.set_u8(base + 9, 0);
    e.mem.set_u32(base + 0x18, 0xffff_ffff);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u32(base + 0x10, 0x7fff_ffff);
    e.mem.set_u32(base + 0x14, 0);
    this
}

/// The process's vtable slot `+0x298` (four stack words).
const PROCESS_SLOT_298: u32 = 0x298;
/// `00408840` (unnamed, engine map unit `effectsetting.cpp`; `cdecl`, one `float`; in `ST0`).
const BLOCKED_TIME_00408840: u32 = 0x0040_8840;
/// The `double` the blocked-movement time is compared with (`0101ffa0`).
const BLOCKED_TIME_LIMIT: u32 = 0x0101_ffa0;
/// `00559450` (`thiscall`, no stack words): the word at `this` (here the current package of
/// the embedded `ActorPackage` at `+4`).
const WORD_AT_00559450: u32 = 0x0055_9450;
/// `TESPackage::SetIsCreated` (engine map, `00674d70`; `thiscall`, one stack word) and
/// `Actor::StopMoving` (engine map, `008b3ab0`; `thiscall`, no stack words).
const PACKAGE_SET_IS_CREATED: u32 = 0x0067_4d70;
const ACTOR_STOP_MOVING: u32 = 0x008b_3ab0;
/// The actor's vtable slot `+0x360` (no stack words; a `bool` in `AL`) and `00566950`
/// (`thiscall` on the actor, no stack words; a `bool` in `AL`; the engine map's name
/// `MiddleHighProcess::GetForceNextUpdate` is folded code).
const ACTOR_SLOT_360: u32 = 0x360;
const ACTOR_TEST_00566950: u32 = 0x0056_6950;
/// `006b8310` (unnamed, `thiscall` on the map's allocator `+0xc`, one stack word: the item).
const MAP_ALLOCATOR_DEALLOCATE: u32 = 0x006b_8310;
/// `BaseProcess::Revert_ov2` (`008d1070`, one stack word) and `BaseProcess::InitLoadGame`
/// (`008d0e60`, three stack words) (engine map; `thiscall`).
const BASE_PROCESS_REVERT_OV2: u32 = 0x008d_1070;
const BASE_PROCESS_INIT_LOAD_GAME: u32 = 0x008d_0e60;
/// `00910b10` (`thiscall` on the combat timer `+0x38`, no stack words; the engine map's name
/// `TESWeightForm::InitializeDataComponent` is folded code).
const COMBAT_TIMER_RESET_00910B10: u32 = 0x0091_0b10;
/// `0042ce30` (`thiscall` on the buffer, one stack word: the out word): the change flags
/// `Revert_ov2` tests; `004280f0` (`FLAGS_HAVE_004280F0`) tests them.
const REVERT_CHANGE_FLAGS: u32 = 0x0042_ce30;
/// `ModifierList`'s revert forwarder (`00937b30`, `thiscall`, no stack words).
const MODIFIER_LIST_REVERT: u32 = 0x0093_7b30;
/// `__RTDynamicCast`'s type descriptors of `TESForm` (`01183028`) and `TESObjectREFR`
/// (`011841cc`).
const FORM_TYPE_DESCRIPTOR: u32 = 0x0118_3028;
const REFERENCE_TYPE_DESCRIPTOR: u32 = 0x0118_41cc;

/// The form of a saved form id cast to `TESObjectREFR` (`004839c0`, `__RTDynamicCast`).
fn reference_of_form_id(e: &mut Engine, id: u32) -> u32 {
    let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![
            form,
            0u32,
            FORM_TYPE_DESCRIPTOR,
            REFERENCE_TYPE_DESCRIPTOR,
            0u32
        ],
    )
    .u32()
}

// Translated from 00911ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (engine map unit `lowprocess.cpp`; `thiscall`, two stack words, `RET 8`;
/// neither is read): answers 0.0 (`FLDZ`, through a `float` local).
pub fn fn_00911ae0(
    _e: &mut Engine,
    _unused_0: Ptr<LowProcess>,
    _unused_1: u32,
    _unused_2: u32,
) -> f32 {
    f32::from_bits(x87_store32(0.0))
}

// Translated from 00911ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessSummonCreatureDefend` (engine map; `thiscall`, one stack word,
/// `RET 4`): the process's slot `+0x298(actor, 0, 0x101, 0)`.
pub fn low_process_process_summon_creature_defend(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
) {
    e.vcall(
        this.addr(),
        PROCESS_SLOT_298,
        &args![actor, 0u32, 0x101u32, 0u32],
    );
}

// Translated from 00912150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessMovementBlocked` (engine map; `thiscall`, one stack word, `RET 4`):
/// the calendar's hour (kept as a `double`) minus the package's start hour (`00621b00` on the
/// `ActorPackage` `+4`), stored as a `float`, through `00408840`; when that is at least the
/// `double` at `0101ffa0` (`FCOMP`, `TEST AH,1`: below and unordered stop) the running
/// procedure moves on (slot `+0x288(actor, 1)`) and the package is marked created
/// (`TESPackage::SetIsCreated(1)`), otherwise the actor stops moving.
pub fn low_process_process_movement_blocked(e: &mut Engine, this: Ptr<LowProcess>, actor: u32) {
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    let start = e
        .call(ACTOR_PACKAGE_GET_START_HOUR, &args![this.addr() + PACKAGE])
        .f64();
    let elapsed = x87_store32(x87_sub(hour, start));
    let value = e
        .call(BLOCKED_TIME_00408840, &args![x87_quiet_bits(elapsed)])
        .f64();
    let value = x87_load32(x87_store32(value));
    let limit: f64 = e.global(BLOCKED_TIME_LIMIT);
    let below = value < limit || value.is_nan() || limit.is_nan();
    if below {
        e.call(ACTOR_STOP_MOVING, &args![actor]);
        return;
    }
    e.vcall(
        this.addr(),
        ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
        &args![actor, 1u32],
    );
    let package = e
        .call(WORD_AT_00559450, &args![this.addr() + PACKAGE])
        .u32();
    e.call(PACKAGE_SET_IS_CREATED, &args![package, 1u32]);
}

// Translated from 00913460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::IsPackageDone` (engine map; `thiscall`, one stack word not read, `RET 4`):
/// `true` without a current package; otherwise whether its type's procedure list holds the
/// "done" procedure 0x36 at the running procedure index (vtable `+0x280`).
pub fn low_process_is_package_done(e: &mut Engine, this: Ptr<LowProcess>, _unused_1: u32) -> bool {
    let base = this.addr();
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    if package == 0 {
        return true;
    }
    let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
    let index = e
        .vcall(base, GET_PROCEDURE_INDEX_RUNNING_SLOT, &args![])
        .u32();
    let list = e
        .mem
        .u32(PROCEDURE_LISTS.wrapping_add(kind.wrapping_mul(4)));
    e.mem.u32(list.wrapping_add(index.wrapping_mul(4))) == PROCEDURE_DONE
}

// Translated from 009134c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ModTrackedDamage` (engine map; `thiscall`, two stack words: actor and
/// amount, `RET 8`): for an actor whose slot `+0x360` or `00566950` answers true,
/// `fTrackedDamage` grows by `amount` (`FADD`, stored as a `float`).
pub fn low_process_mod_tracked_damage(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    amount: f32,
) {
    if actor == 0 {
        return;
    }
    if !e.vcall(actor, ACTOR_SLOT_360, &args![]).bool()
        && !e.call(ACTOR_TEST_00566950, &args![actor]).bool()
    {
        return;
    }
    let tracked = e.mem.u32(this.addr() + TRACKED_DAMAGE);
    let sum = x87_store32(x87_add(x87_load32(tracked), x87_m32(amount.to_bits())));
    e.mem.set_u32(this.addr() + TRACKED_DAMAGE, sum);
}

// Translated from 009137e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<LowProcess *,LP_LOCK_DATA>::_scalar_deleting_destructor_` (engine map;
/// `thiscall`, one stack word, `RET 4`): the destructor [`fn_00913880`], then `this` freed
/// (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn fn_009137e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00913880(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 00913950 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (engine map unit `lowprocess.cpp`; `thiscall`, one stack word not read,
/// `RET 4`): the `LP_LOCK_DATA` value constructor by its only caller here: both words
/// cleared. Returns `this`.
pub fn fn_00913950(e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

// Translated from 00913910 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (engine map unit `lowprocess.cpp`; `thiscall`, one stack word: an item,
/// `RET 4`; the `NiTMap<LowProcess *,LP_LOCK_DATA>` item delete by its shape): a local value
/// made by [`fn_00913950`] is copied into the item's value (`+8`, `+0xc`), then the item goes
/// to `006b8310` on the allocator `+0xc`.
pub fn fn_00913910(e: &mut Engine, this: Ptr, item: u32) {
    e.with_stack(8, |e, value| {
        fn_00913950(e, value, 0);
        let first = e.mem.u32(value.addr());
        e.mem.set_u32(item + 8, first);
        let second = e.mem.u32(value.addr() + 4);
        e.mem.set_u32(item + 0xc, second);
    });
    e.call(MAP_ALLOCATOR_DEALLOCATE, &args![this.addr() + 0xc, item]);
}

// Translated from 00913980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<..>,LowProcess *,LP_LOCK_DATA>::_scalar_deleting_destructor_` (engine
/// map; `thiscall`, one stack word, `RET 4`): the destructor [`fn_009138e0`], then `this`
/// freed (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00913980(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_009138e0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 00910a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::Revert_ov2` (engine map; `thiscall`, one stack word: the buffer, `RET 4`):
/// `BaseProcess::Revert_ov2`, the release [`fn_009070a0`], then `iNumberItemsActivate`
/// cleared, the target cleared (slot `+0x12c(0)`), `fEssentialDownTimer` 0.0,
/// `pItemBeingUsed` 0, the combat timer reset (`00910b10`), `fDeathTime` 0.0, `m_uFlags` 0,
/// `fTrackedDamage` 0.0; with the buffer's change flag 0x200000 (`0042ce30`) the damage
/// modifiers' revert forwarder (`00937b30`).
pub fn low_process_revert_ov2(e: &mut Engine, this: Ptr<LowProcess>, buffer: u32) {
    let base = this.addr();
    e.call(BASE_PROCESS_REVERT_OV2, &args![this, buffer]);
    fn_009070a0(e, this);
    e.mem.set_u32(base + NUMBER_ITEMS_ACTIVATE, 0);
    e.vcall(base, SET_TARGET_SLOT, &args![0u32]);
    e.mem.set_u32(base + ESSENTIAL_DOWN_TIMER, x87_store32(0.0));
    e.mem.set_u32(base + 0x34, 0);
    e.call(
        COMBAT_TIMER_RESET_00910B10,
        &args![base + COMBAT_DELAY_TIMER],
    );
    e.mem.set_u32(base + DEATH_TIME, x87_store32(0.0));
    e.mem.set_u8(base + 0x30, 0);
    e.mem.set_u32(base + TRACKED_DAMAGE, x87_store32(0.0));
    let has = e.with_stack(4, |e, flags| {
        let flags = e.call(REVERT_CHANGE_FLAGS, &args![buffer, flags]).u32();
        e.call(FLAGS_HAVE_004280F0, &args![flags, SAVE_MODIFIERS_FLAG])
            .bool()
    });
    if has {
        e.call(MODIFIER_LIST_REVERT, &args![base + DAMAGE_MODIFIERS]);
    }
}

// Translated from 00910250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::InitLoadGame` (engine map; `thiscall`, three stack words, `RET 0xc`):
/// `BaseProcess::InitLoadGame`; then the saved form ids of the target (`+0x40`) and of the
/// generic location (`+0x44`) become their references (`004839c0`, cast to
/// `TESObjectREFR`): the target through `SetTarget` (slot `+0x12c`) and, when one is then
/// set, marked targeted (`TESObjectREFR::SetTargeted(1)`); the location stored directly.
pub fn low_process_init_load_game(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    first: u32,
    second: u32,
    third: u32,
) {
    let base = this.addr();
    e.call(
        BASE_PROCESS_INIT_LOAD_GAME,
        &args![this, first, second, third],
    );
    let target_id = e.mem.u32(base + TARGET);
    if target_id != 0 {
        let target = reference_of_form_id(e, target_id);
        e.vcall(base, SET_TARGET_SLOT, &args![target]);
        if e.mem.u32(base + TARGET) != 0 {
            let target = e.mem.u32(base + TARGET);
            e.call(REFERENCE_SET_TARGETED, &args![target, 1u32]);
        }
    }
    let location_id = e.mem.u32(base + GENERIC_LOCATION);
    if location_id != 0 {
        let location = reference_of_form_id(e, location_id);
        e.mem.set_u32(base + GENERIC_LOCATION, location);
    }
}

/// `__RTDynamicCast`'s type descriptor of `Actor` (`011846d4`).
const ACTOR_TYPE_DESCRIPTOR: u32 = 0x0118_46d4;
/// The actor's vtable slots `+0x214` (no stack words), `+0x418` (no stack words) and
/// `+0x2f4` (three stack words), the target process's slots `+0x28` (no stack words),
/// `+0x710` (one) and `+0x12c` (one) (each counted from the `Character` and `LowProcess`
/// tables).
const ACTOR_SLOT_214_ESCORT: u32 = 0x214;
const ACTOR_SLOT_418_ESCORT: u32 = 0x418;
const ACTOR_SLOT_2F4: u32 = 0x2f4;
const PROCESS_SAVE_PACKAGE_SLOT: u32 = 0x710;
/// `TESPackage::CreatePackage` (`00670b90`, `cdecl`, one word), `TESPackage::SetPackType`
/// (`00670fc0`), `00826b40` and `00826b90` (unnamed, engine map unit `reanimateeffect.cpp`),
/// `TESPackage::SetPackageLocation` (`00671d30`), `TESPackage::GetEscortFollowDistance`
/// (`006784c0`, no stack words), `0067f1f0` (unnamed, `packagelocation.cpp`),
/// `TESPackage::SetPackageTarget` (`00672fc0`), `00984f60` (unnamed),
/// `PackageTarget::SetTargType` (`006800b0`), `PackageTarget::SetTargReference`
/// (`00680110`), `0067a1b0` (unnamed) and `Actor::AddFollower` (`008bc790`) (engine map;
/// `thiscall` with one stack word unless said).
const PACKAGE_CREATE: u32 = 0x0067_0b90;
const PACKAGE_SET_PACK_TYPE: u32 = 0x0067_0fc0;
const PACKAGE_00826B40: u32 = 0x0082_6b40;
const PACKAGE_00826B90: u32 = 0x0082_6b90;
const PACKAGE_SET_LOCATION: u32 = 0x0067_1d30;
const PACKAGE_GET_ESCORT_FOLLOW_DISTANCE: u32 = 0x0067_84c0;
const LOCATION_0067F1F0: u32 = 0x0067_f1f0;
const PACKAGE_SET_TARGET: u32 = 0x0067_2fc0;
const PACKAGE_00984F60: u32 = 0x0098_4f60;
const PACKAGE_TARGET_SET_TYPE: u32 = 0x0068_00b0;
const PACKAGE_TARGET_SET_REFERENCE: u32 = 0x0068_0110;
const PACKAGE_0067A1B0: u32 = 0x0067_a1b0;
const ACTOR_ADD_FOLLOWER: u32 = 0x008b_c790;
/// The package location of a package (`0055b980`, no stack words) and the word at `+8`
/// (`0044ddc0`, no stack words).
const PACKAGE_LOCATION_0055B980: u32 = 0x0055_b980;
const WORD_08_0044DDC0: u32 = 0x0044_ddc0;

// Translated from 00908cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::CreateFollowforEscort` (engine map; `thiscall`, three stack words: actor,
/// escort package, a word not read, `RET 0xc`). The target (`+0x40` cast to `Actor`) must be
/// set, be another actor than `actor` and not the player. When the actor's slot `+0x214`
/// answers something, its slot `+0x418` runs. Then a new package (`CreatePackage(1)`) gets
/// type 1 and the two `reanimateeffect.cpp` calls with 1; its location is the current
/// package's (`0055b980`), its location's distance the escort package's follow distance; its
/// target the current package's (`00671d10`); `00984f60(6)`; its target's type 0 and
/// reference `actor`; its target's word the escort package target's word `+8`
/// (`00403550`); `0067a1b0(escort package)`. The target's process (`008d8520`) runs its slot
/// `+0x28`, saves the package (slot `+0x710(target)`); `actor` follows (`AddFollower`); the
/// target's slot `+0x2f4(package, 0, 1)` and its process's `SetTarget(actor)` (slot
/// `+0x12c`).
pub fn low_process_create_followfor_escort(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    escort_package: u32,
    _unused_3: u32,
) {
    let base = this.addr();
    let target_reference = e.mem.u32(base + TARGET);
    let target = e
        .call(
            DYNAMIC_CAST,
            &args![
                target_reference,
                0u32,
                REFERENCE_TYPE_DESCRIPTOR,
                ACTOR_TYPE_DESCRIPTOR,
                0u32
            ],
        )
        .u32();
    if target == actor || target == 0 || target == e.global::<u32>(PLAYER) {
        return;
    }
    if e.vcall(actor, ACTOR_SLOT_214_ESCORT, &args![]).u32() != 0 {
        e.vcall(actor, ACTOR_SLOT_418_ESCORT, &args![]);
    }
    let package = e.call(PACKAGE_CREATE, &args![1u32]).u32();
    e.call(PACKAGE_SET_PACK_TYPE, &args![package, 1u32]);
    e.call(PACKAGE_00826B40, &args![package, 1u32]);
    e.call(PACKAGE_00826B90, &args![package, 1u32]);
    let current = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let location = e.call(PACKAGE_LOCATION_0055B980, &args![current]).u32();
    e.call(PACKAGE_SET_LOCATION, &args![package, location]);
    let distance = e
        .call(PACKAGE_GET_ESCORT_FOLLOW_DISTANCE, &args![escort_package])
        .u32();
    let new_location = e.call(PACKAGE_LOCATION_0055B980, &args![package]).u32();
    e.call(LOCATION_0067F1F0, &args![new_location, distance]);
    let current = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let current_target = e.call(PACKAGE_GET_TARGET, &args![current]).u32();
    e.call(PACKAGE_SET_TARGET, &args![package, current_target]);
    e.call(PACKAGE_00984F60, &args![package, 6u32]);
    let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    e.call(PACKAGE_TARGET_SET_TYPE, &args![package_target, 0u32]);
    let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    e.call(PACKAGE_TARGET_SET_REFERENCE, &args![package_target, actor]);
    let escort_target = e.call(PACKAGE_GET_TARGET, &args![escort_package]).u32();
    let word = e.call(WORD_08_0044DDC0, &args![escort_target]).u32();
    let package_target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    e.call(ACTOR_PACKAGE_SET_TARGET, &args![package_target, word]);
    e.call(PACKAGE_0067A1B0, &args![package, escort_package]);
    let process = e.call(ACTOR_GET_PROCESS, &args![target]).u32();
    e.vcall(process, COMPUTE_LAST_TIME_PROCESSED_SLOT, &args![]);
    e.vcall(process, PROCESS_SAVE_PACKAGE_SLOT, &args![target]);
    e.call(ACTOR_ADD_FOLLOWER, &args![actor, target]);
    e.vcall(target, ACTOR_SLOT_2F4, &args![package, 0u32, 1u32]);
    e.vcall(process, SET_TARGET_SLOT, &args![actor]);
}

/// `__RTDynamicCast`'s type descriptors of `TESBoundObject` (`01183108`), `TESNPC`
/// (`01183a1c`) and `TESCreature` (`01183a00`).
const BOUND_OBJECT_TYPE_DESCRIPTOR: u32 = 0x0118_3108;
const NPC_TYPE_DESCRIPTOR: u32 = 0x0118_3a1c;
const CREATURE_TYPE_DESCRIPTOR: u32 = 0x0118_3a00;
/// The actor's vtable slot `+0x1e8` (no stack words, from the `Character` table).
const ACTOR_SLOT_1E8: u32 = 0x1e8;
/// `00606540` (unnamed, engine map unit `tesnpc.cpp`; `thiscall` on the `TESNPC`, three stack
/// words) and `TESCreature::InitDefaultWorn` (engine map, `005f9e00`; four stack words).
const NPC_INIT_DEFAULT_WORN: u32 = 0x0060_6540;
const CREATURE_INIT_DEFAULT_WORN: u32 = 0x005f_9e00;
/// `00441b00` (unnamed, engine map unit `modelloader.cpp`; `thiscall` on the package, no
/// stack words; a `bool` in `AL`).
const PACKAGE_TEST_00441B00: u32 = 0x0044_1b00;

// Translated from 00909320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessRemoveWorn` (engine map; `thiscall`, two stack words: actor and a
/// flag, `RET 8`). The actor's base form (`007af430`) cast to `TESNPC` and to `TESCreature`.
/// Two flags start true: a package of type 4 clears both, otherwise a package for which
/// `00441b00` holds clears the second. An NPC gets `00606540(actor, slot +0x1e8 of the actor,
/// 1)`; else a creature `TESCreature::InitDefaultWorn(actor, first, second, 1)`. With the
/// flag the running procedure moves on (slot `+0x288(actor, 1)`).
pub fn low_process_process_remove_worn(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    advance: u8,
) {
    let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![actor]).u32();
    let npc = e
        .call(
            DYNAMIC_CAST,
            &args![
                base_form,
                0u32,
                BOUND_OBJECT_TYPE_DESCRIPTOR,
                NPC_TYPE_DESCRIPTOR,
                0u32
            ],
        )
        .u32();
    let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![actor]).u32();
    let creature = e
        .call(
            DYNAMIC_CAST,
            &args![
                base_form,
                0u32,
                BOUND_OBJECT_TYPE_DESCRIPTOR,
                CREATURE_TYPE_DESCRIPTOR,
                0u32
            ],
        )
        .u32();
    let package = e
        .vcall(this.addr(), GET_CURRENT_PACKAGE_SLOT, &args![])
        .u32();
    let mut first = true;
    let mut second = true;
    if package != 0 {
        if e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 4 {
            first = false;
            second = false;
        } else if e.call(PACKAGE_TEST_00441B00, &args![package]).bool() {
            second = false;
        }
    }
    if npc != 0 {
        let worn = e.vcall(actor, ACTOR_SLOT_1E8, &args![]).u32();
        e.call(NPC_INIT_DEFAULT_WORN, &args![npc, actor, worn, 1u32]);
    } else if creature != 0 {
        e.call(
            CREATURE_INIT_DEFAULT_WORN,
            &args![creature, actor, u32::from(first), u32::from(second), 1u32],
        );
    }
    if advance != 0 {
        e.vcall(
            this.addr(),
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 1u32],
        );
    }
}

/// `Actor::HasObjects` (engine map, `00891db0`; `thiscall`, five stack words; a `bool` in
/// `AL`).
const ACTOR_HAS_OBJECTS: u32 = 0x0089_1db0;

// Translated from 0090ac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::CheckIfHasObject` (engine map; `thiscall`, two stack words: actor and count,
/// `RET 8`): `false` without a target on the current package (`00671d10`). By the target's
/// type (`PackageTarget::GetTargType`; the exe's jump table `0090ad30`): 0 and 3 ask
/// `Actor::HasObjects(base form, 0, 1, 0084e3a0, local)` of the reference the
/// `ActorPackage` (`+4`) holds (`0044ddc0`; `false` without one), 1 asks it with
/// `(GetTargObject, 0, count, 0, local)`, 2 with `(0, GetTargObjectType, count, 0, local)`;
/// any other type answers `false`.
pub fn low_process_check_if_has_object(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    count: u32,
) -> bool {
    let package = e
        .vcall(this.addr(), GET_CURRENT_PACKAGE_SLOT, &args![])
        .u32();
    let target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
    if target == 0 {
        return false;
    }
    let kind = e.call(PACKAGE_TARGET_GET_TYPE, &args![target]).u32();
    e.with_stack(4, |e, local| match kind {
        0 | 3 => {
            let reference = e
                .call(WORD_08_0044DDC0, &args![this.addr() + PACKAGE])
                .u32();
            if reference == 0 {
                return false;
            }
            let word = e.call(GET_WORD_0C, &args![reference]).u32();
            let form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
            e.call(
                ACTOR_HAS_OBJECTS,
                &args![actor, form, 0u32, 1u32, word, local],
            )
            .bool()
        }
        1 => {
            let object = e.call(PACKAGE_TARGET_GET_OBJECT, &args![target]).u32();
            e.call(
                ACTOR_HAS_OBJECTS,
                &args![actor, object, 0u32, count, 0u32, local],
            )
            .bool()
        }
        2 => {
            let object_type = e.call(PACKAGE_TARGET_GET_OBJECT_TYPE, &args![target]).u32();
            e.call(
                ACTOR_HAS_OBJECTS,
                &args![actor, 0u32, object_type, count, 0u32, local],
            )
            .bool()
        }
        _ => false,
    })
}

/// The setting whose `float` sets the combat rounds (`011cd6cc`) and the `double` the
/// low-process combat multiplies it by (`0101db80`).
const COMBAT_ROUNDS_SETTING: u32 = 0x011c_d6cc;
const COMBAT_ROUNDS_FACTOR: u32 = 0x0101_db80;
/// The actor's vtable slot `+0x438` (no stack words): one combat round.
const ACTOR_COMBAT_ROUND_SLOT: u32 = 0x438;
/// `Actor::SetCurrentTarget` (engine map, `00881620`; `thiscall`, one stack word).
const ACTOR_SET_CURRENT_TARGET: u32 = 0x0088_1620;

/// Whether the actor's slot `+0x22c(0)` answers true (the combat loops' stop test).
fn actor_slot_22c(e: &mut Engine, actor: u32) -> bool {
    e.vcall(actor, ACTOR_SLOT_22C, &args![0u32]).bool()
}

/// The combat target (`+0x40`) cast to `Actor` (0 when it is not one).
fn combat_target_actor(e: &mut Engine, this: Ptr<LowProcess>) -> u32 {
    let target = e.mem.u32(this.addr() + TARGET);
    e.call(
        DYNAMIC_CAST,
        &args![
            target,
            0u32,
            REFERENCE_TYPE_DESCRIPTOR,
            ACTOR_TYPE_DESCRIPTOR,
            0u32
        ],
    )
    .u32()
}

/// The process's `EnterCombat` slot (`+0x33c`, 13 words) with the target, the actor, nine
/// zero words, 1 and 0.
fn enter_combat_call(e: &mut Engine, process: u32, first: u32, second: u32) {
    e.vcall(
        process,
        ENTER_COMBAT_SLOT,
        &args![first, second, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
}

// Translated from 0090d000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessCombat` (engine map; `thiscall`, one stack word: the actor, `RET 4`).
/// Without a target the package's target is set (slot `+0x8c`). With one whose package slot
/// `+0x144(actor, 0)` fails, the process hands over (slot `+0x298(actor, 0, 0x101, 0)`).
/// When the target is the sleeping or resting player (`byte +0x20c` clear), `005c1a00(0, 1)`
/// on the player and the running procedure moves back (slot `+0x288(actor, -1)`). Otherwise
/// up to `setting 011cd6cc * double 0101db80` rounds (`_ftol2`) run while the actor's slot
/// `+0x22c(0)` answers false: its slot `+0x438`; when `00493bb0` of the actor holds, the
/// target cast to `Actor` (when set) gets the actor as its current target and, unless its
/// slot `+0x22c(0)` answers true, enters combat with the actor (its process's slot `+0x33c`)
/// unless `00493bb0` holds for it, and runs its slot `+0x438` (remembered); when it does not
/// hold the rounds end. A remembered round makes the target's process run its slot `+0x28`.
pub fn low_process_process_combat(e: &mut Engine, this: Ptr<LowProcess>, actor: u32) {
    let base = this.addr();
    if e.mem.u32(base + TARGET) == 0 {
        e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
    }
    if e.mem.u32(base + TARGET) != 0 {
        let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
        if !e
            .vcall(package, PACKAGE_SLOT_144, &args![actor, 0u32])
            .bool()
        {
            e.vcall(base, PROCESS_SLOT_298, &args![actor, 0u32, 0x101u32, 0u32]);
            return;
        }
    }
    let player = e.global::<u32>(PLAYER);
    if e.mem.u32(base + TARGET) == player
        && e.call(PLAYER_IS_SLEEPING_OR_RESTING, &args![player]).bool()
        && e.mem.u8(e.global::<u32>(PLAYER) + PLAYER_BYTE_20C) == 0
    {
        let player = e.global::<u32>(PLAYER);
        e.call(PLAYER_005C1A00, &args![player, 0u32, 1u32]);
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, u32::MAX],
        );
        return;
    }
    let setting = e
        .call(SETTING_VALUE_00403E20, &args![COMBAT_ROUNDS_SETTING])
        .u32();
    let factor: f64 = e.global(COMBAT_ROUNDS_FACTOR);
    let rounds_value = x87_mul(x87_load32(e.mem.u32(setting)), factor);
    let mut rounds = e.call(FTOL, &args![rounds_value]).u32();
    let mut target = 0u32;
    let mut fought = false;
    while rounds != 0 {
        if actor_slot_22c(e, actor) {
            break;
        }
        e.vcall(actor, ACTOR_COMBAT_ROUND_SLOT, &args![]);
        if !e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
            break;
        }
        target = combat_target_actor(e, this);
        if target != 0 {
            e.call(ACTOR_SET_CURRENT_TARGET, &args![target, actor]);
            if !actor_slot_22c(e, target) {
                if !e.call(ACTOR_TEST_00493BB0, &args![target]).bool() {
                    let process = e.call(ACTOR_GET_PROCESS, &args![target]).u32();
                    enter_combat_call(e, process, target, actor);
                }
                e.vcall(target, ACTOR_COMBAT_ROUND_SLOT, &args![]);
                fought = true;
            }
        }
        rounds = rounds.wrapping_sub(1);
    }
    if fought {
        let process = e.call(ACTOR_GET_PROCESS, &args![target]).u32();
        e.vcall(process, COMPUTE_LAST_TIME_PROCESSED_SLOT, &args![]);
    }
}

/// `__RTDynamicCast`'s type descriptors of `TESPackage` (`011846a0`) and `AlarmPackage`
/// (`011a39c8`).
const PACKAGE_TYPE_DESCRIPTOR: u32 = 0x0118_46a0;
const ALARM_PACKAGE_TYPE_DESCRIPTOR: u32 = 0x011a_39c8;
/// `Actor::GetClass` (`00884350`), `TESClass::IsGuard` (`005f6e60`), `006130e0` (unnamed,
/// engine map unit `tesrace.cpp`: the alarm package's list) and
/// `Actor::GetShouldAttackActor` (`008b06d0`, four stack words) (engine map; `thiscall`).
const ACTOR_GET_CLASS: u32 = 0x0088_4350;
const CLASS_IS_GUARD: u32 = 0x005f_6e60;
const ALARM_PACKAGE_LIST: u32 = 0x0061_30e0;
const ACTOR_GET_SHOULD_ATTACK_ACTOR: u32 = 0x008b_06d0;
/// The distance (`double` 3500.0, `01088f48`) beyond which a guard's alarm ends when the
/// player is in its world space.
const ALARM_GUARD_DISTANCE: u32 = 0x0108_8f48;

// Translated from 0090d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessAlarm` (engine map; `thiscall`, one stack word: the actor, `RET 4`):
/// only for a current package that is an `AlarmPackage`. A non-guard (`TESClass::IsGuard` of
/// `Actor::GetClass`) walks the package's list (`006130e0`) until a node without an item:
/// for an item whose word `+4` (`00726070`) is at most 2, `Actor::GetShouldAttackActor` of
/// the item's `0084e3a0` reference (with 0, a local, 0) ends the walk as found (for larger
/// words the exe tests a local that is always 0 first, so that call never happens). Found:
/// the process hands over (slot `+0x298(actor, 0, 0x101, 0)`) and enters combat (slot
/// `+0x33c(actor, 0, ..., 1, 0)`); not found: the running procedure moves by 3. A guard moves
/// it by 1, pursues (slot `+0x29c(actor, 0, 1, 0x101)`), and when farther than 3500 from the
/// player (`FCOMP`, `TEST AH,0x41`) in the player's (non-null) world space ends its interrupt
/// package.
pub fn low_process_process_alarm(e: &mut Engine, this: Ptr<LowProcess>, actor: u32) {
    let base = this.addr();
    let current = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let package = e
        .call(
            DYNAMIC_CAST,
            &args![
                current,
                0u32,
                PACKAGE_TYPE_DESCRIPTOR,
                ALARM_PACKAGE_TYPE_DESCRIPTOR,
                0u32
            ],
        )
        .u32();
    // A local the exe passes to `EnterCombat` (never written: 0).
    let combat_word = 0u32;
    if package == 0 {
        return;
    }
    let class = e.call(ACTOR_GET_CLASS, &args![actor]).u32();
    if !e.call(CLASS_IS_GUARD, &args![class]).bool() {
        let mut node = e.call(ALARM_PACKAGE_LIST, &args![package]).u32();
        let mut found = false;
        e.with_stack(4, |e, attack| {
            while node != 0 {
                let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
                if e.mem.u32(item_address) == 0 || found {
                    break;
                }
                let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
                let item = e.mem.u32(item_address);
                let reference = e.call(GET_WORD_0C, &args![item]).u32();
                e.call(WORD_08_0044DDC0, &args![item]);
                // A local the exe sets to 0 and tests before the call for larger words.
                let tested = 0u32;
                e.mem.set_u32(attack.addr(), 0);
                if e.call(LIST_NEXT_NODE, &args![item]).i32() > 2 {
                    if tested != 0
                        && e.call(
                            ACTOR_GET_SHOULD_ATTACK_ACTOR,
                            &args![actor, reference, 0u32, attack, 0u32],
                        )
                        .bool()
                    {
                        found = true;
                    }
                } else if e
                    .call(
                        ACTOR_GET_SHOULD_ATTACK_ACTOR,
                        &args![actor, reference, 0u32, attack, 0u32],
                    )
                    .bool()
                {
                    found = true;
                }
                node = e.call(LIST_NEXT_NODE, &args![node]).u32();
            }
        });
        if found {
            e.vcall(base, PROCESS_SLOT_298, &args![actor, 0u32, 0x101u32, 0u32]);
            enter_combat_call(e, base, actor, combat_word);
        } else {
            e.vcall(
                base,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, 3u32],
            );
        }
        return;
    }
    e.vcall(
        base,
        ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
        &args![actor, 1u32],
    );
    e.vcall(
        base,
        PROCESS_PURSUE_SLOT,
        &args![actor, 0u32, 1u32, 0x101u32],
    );
    let player = e.global::<u32>(PLAYER);
    let distance = e
        .call(REFERENCE_GET_DISTANCE, &args![actor, player, 0u32, 0u32])
        .f64();
    let limit: f64 = e.global(ALARM_GUARD_DISTANCE);
    if !(distance > limit) {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    if e.call(REFERENCE_GET_WORLD_SPACE, &args![player]).u32() == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let player_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![player]).u32();
    let actor_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
    if player_space == actor_space {
        e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
    }
}

/// `Actor::RemoveFleeTarget` (engine map, `008a6730`; `thiscall`, one stack word) and the
/// actor's vtable slot `+0x304` (no stack words; a `bool` in `AL`).
const ACTOR_REMOVE_FLEE_TARGET: u32 = 0x008a_6730;
const ACTOR_SLOT_304: u32 = 0x304;
/// The pursuit distance (`double` 200.0, `010179e0`).
const PURSUE_DISTANCE: u32 = 0x0101_79e0;

// Translated from 0090dc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ProcessPursue` (engine map; `thiscall`, four stack words: the actor, two
/// words not read and a word passed on, `RET 0x10`). Without a target the package's target is
/// set (slot `+0x8c`); still without one the procedure moves on (slot `+0x288(actor, 1)`).
/// A target without a process (`MobileObject::GetCurrentProcessType` below 1) pursued by an
/// actor that is not alarmed moves the procedure by 2 and stops fleeing it
/// (`Actor::RemoveFleeTarget`). Farther than 200 from the target (`FCOMP`, `TEST AH,0x41`) the
/// process hands over (slot `+0x298(actor, 0, word, 1)`); nearer, the procedure moves on
/// unless the actor's slot `+0x304` answers true and the package type is not 0x13.
pub fn low_process_process_pursue(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    _unused_2: u32,
    _unused_3: u32,
    word: u32,
) {
    let base = this.addr();
    if e.mem.u32(base + TARGET) == 0 {
        e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
    }
    if e.mem.u32(base + TARGET) == 0 {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 1u32],
        );
        return;
    }
    let target = e.mem.u32(base + TARGET);
    if e.call(MOBILE_GET_PROCESS_TYPE, &args![target]).i32() < 1
        && !e.call(ACTOR_IS_ALARMED, &args![actor]).bool()
    {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 2u32],
        );
        e.call(ACTOR_REMOVE_FLEE_TARGET, &args![target, actor]);
    }
    let target = e.mem.u32(base + TARGET);
    let distance = e
        .call(REFERENCE_GET_DISTANCE, &args![actor, target, 0u32, 0u32])
        .f64();
    let limit: f64 = e.global(PURSUE_DISTANCE);
    if distance > limit {
        e.vcall(base, PROCESS_SLOT_298, &args![actor, 0u32, word, 1u32]);
        return;
    }
    if e.vcall(actor, ACTOR_SLOT_304, &args![]).bool() {
        let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
        if e.call(PACKAGE_GET_TYPE, &args![package]).u32() != 0x13 {
            return;
        }
    }
    e.vcall(
        base,
        ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
        &args![actor, 1u32],
    );
}

/// `BaseProcess::Revert` (engine map, `008d0f10`; `thiscall`, two stack words) and
/// `0055f5b0` (unnamed, engine map unit `tesobjectrefr.cpp`; `thiscall` on the save/load game
/// object `011de45c`, no stack words; a `bool` in `AL`).
const BASE_PROCESS_REVERT: u32 = 0x008d_0f10;
const SAVE_LOAD_TEST_0055F5B0: u32 = 0x0055_f5b0;
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;

// Translated from 00910310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::Revert` (engine map; `thiscall`, two stack words: the change flags and a
/// form, `RET 8`): `BaseProcess::Revert`; when `0055f5b0` of the save/load game object holds,
/// `iNumberItemsActivate` is cleared, every object of `ObjectList` (`+0x5c`) removed and
/// deleted, `GenericLocationsList` (`+0x6c`) emptied, the acquire objects (`+0x64`, `+0x68`)
/// cleared, the target cleared (slot `+0x12c(0)`), the generic location cleared,
/// `fEssentialDownTimer`, `fDeathTime` and `fTrackedDamage` 0.0, `pItemBeingUsed` 0 and
/// `m_uFlags` 0. When `00440d80` of the form fails, `0055f5b0` runs again (its answer
/// unused). With flag 0x200000 the damage modifiers' revert forwarder (`00937b30`).
pub fn low_process_revert(e: &mut Engine, this: Ptr<LowProcess>, flags: u32, form: u32) {
    let base = this.addr();
    e.call(BASE_PROCESS_REVERT, &args![this, flags, form]);
    let save = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_TEST_0055F5B0, &args![save]).bool() {
        e.mem.set_u32(base + NUMBER_ITEMS_ACTIVATE, 0);
        free_list_items(e, base + OBJECT_LIST);
        e.call(LIST_CLEAR, &args![base + GENERIC_LOCATIONS_LIST]);
        e.mem.set_u32(base + ACQUIRE_OBJECT, 0);
        e.mem.set_u32(base + SAVED_ACQUIRE_OBJECT, 0);
        e.vcall(base, SET_TARGET_SLOT, &args![0u32]);
        e.mem.set_u32(base + GENERIC_LOCATION, 0);
        e.mem.set_u32(base + ESSENTIAL_DOWN_TIMER, x87_store32(0.0));
        e.mem.set_u32(base + 0x34, 0);
        e.mem.set_u32(base + DEATH_TIME, x87_store32(0.0));
        e.mem.set_u8(base + 0x30, 0);
        e.mem.set_u32(base + TRACKED_DAMAGE, x87_store32(0.0));
    }
    if !e.call(REFERENCE_TEST_00440D80, &args![form]).bool() {
        let save = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_TEST_0055F5B0, &args![save]);
    }
    if flags & SAVE_MODIFIERS_FLAG != 0 {
        e.call(MODIFIER_LIST_REVERT, &args![base + DAMAGE_MODIFIERS]);
    }
}

/// The save/load game object's calls (all `thiscall` on `011de45c`):
/// `TESSaveLoadGame::UseSaveGameBlocks` (`00862110`; a `bool` in `AL`), the position
/// (`00825c00`), the writer and reader (`008579b0`, `008579e0`; address, size),
/// `TESSaveLoadGame::SaveNumericID` and `LoadNumericID` (`00857a10`, `00857aa0`; address,
/// size), the skip (`00857bd0`, one word: the byte count), the version byte (`008df040`), the
/// records of the form being saved (`004fd3e0`, named `TES::GetWorldSpace` in the engine map,
/// folded) and loaded (`004fd3c0`).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
const SAVE_LOAD_POSITION: u32 = 0x0082_5c00;
const SAVE_LOAD_WRITE: u32 = 0x0085_79b0;
const SAVE_LOAD_READ: u32 = 0x0085_79e0;
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
const SAVE_LOAD_SKIP: u32 = 0x0085_7bd0;
const SAVE_LOAD_VERSION: u32 = 0x008d_f040;
const SAVED_FORM_RECORD: u32 = 0x004f_d3e0;
const LOADED_FORM_RECORD: u32 = 0x004f_d3c0;
/// The save-game block marker (`"KOLB"` as a little-endian word).
const BLOCK_MARKER: u32 = 0x424c_4f4b;
/// The setting-like object at `011de4e8` whose byte (through `00408d60`) turns on the size
/// reports.
const SIZE_REPORT_SETTING: u32 = 0x011d_e4e8;
const SETTING_BYTE_ADDRESS: u32 = 0x0040_8d60;
/// `Error` (engine map, `0040fbe0`; `cdecl`, varargs) and the debug print (`DEBUG_PRINT`,
/// `005b5e40`).
const ERROR_REPORT: u32 = 0x0040_fbe0;
const SAVE_DEBUG_PRINT: u32 = 0x005b_5e40;
/// A form's vtable slot `+0x130` (no stack words): its name.
const FORM_NAME_SLOT: u32 = 0x130;
/// The reports' source file (`"D:\_Fallout3\...\LowProcess.cpp"`) and line numbers.
const LOW_PROCESS_SOURCE_FILE: u32 = 0x0108_8f50;
const GET_SAVE_SIZE_LINE: u32 = 0xeda;
const SAVE_GAME_LINE: u32 = 0xf20;
const LOAD_HEADER_LINE: u32 = 0xf2d;
const LOAD_SIZE_LINE: u32 = 0xf77;
/// The report formats (shared with `MiddleLowProcess`): `GetSaveSize()` with and without the
/// form (`01012cb0`, `01012c78`), `SaveGame()` (`010153a0`, `0101536c`), the block size
/// warning (`01015318`), the bad block header (`01015718`, `010156a8`), the overrun
/// (`01015588`, `010154a0`) and underrun (`01015500`, `01015440`).
const GET_SAVE_SIZE_FORM_FORMAT: u32 = 0x0101_2cb0;
const GET_SAVE_SIZE_FORMAT: u32 = 0x0101_2c78;
const SAVE_GAME_FORM_FORMAT: u32 = 0x0101_53a0;
const SAVE_GAME_FORMAT: u32 = 0x0101_536c;
const BLOCK_TOO_BIG_FORMAT: u32 = 0x0101_5318;
const BAD_HEADER_FORM_FORMAT: u32 = 0x0101_5718;
const BAD_HEADER_FORMAT: u32 = 0x0101_56a8;
const OVERRUN_FORM_FORMAT: u32 = 0x0101_5588;
const OVERRUN_FORMAT: u32 = 0x0101_54a0;
const UNDERRUN_FORM_FORMAT: u32 = 0x0101_5500;
const UNDERRUN_FORMAT: u32 = 0x0101_5440;
/// `BaseProcess::GetSaveSize` (`008d0900`), `SaveGame` (`008d0a10`), `LoadGame`
/// (`008d0bc0`), `LoadGame_ov2` (`008d0f90`) and `InitLoadGame_ov2` (`008d1000`) (engine
/// map; `thiscall`).
const BASE_PROCESS_GET_SAVE_SIZE: u32 = 0x008d_0900;
const BASE_PROCESS_SAVE_GAME_V1: u32 = 0x008d_0a10;
const BASE_PROCESS_LOAD_GAME: u32 = 0x008d_0bc0;
const BASE_PROCESS_LOAD_GAME_OV2: u32 = 0x008d_0f90;
const BASE_PROCESS_INIT_LOAD_GAME_OV2: u32 = 0x008d_1000;
/// `ModifierList` functions on the damage modifiers: `GetSaveSize` (`00937ab0`), `SaveGame`
/// (`009378d0`), `LoadGame` (`009379c0`) and `LoadGame_ov2` (`00937c60`).
const MODIFIER_LIST_GET_SAVE_SIZE: u32 = 0x0093_7ab0;
const MODIFIER_LIST_SAVE_GAME_V1: u32 = 0x0093_78d0;
const MODIFIER_LIST_LOAD_GAME: u32 = 0x0093_79c0;
const MODIFIER_LIST_LOAD_GAME_OV2: u32 = 0x0093_7c60;
/// `004b1500` (unnamed, engine map unit `globalfunc.cpp`; `cdecl`, two `float`s; in `ST0`) and
/// the two `float`s the old date conversion uses with it (100.0 at `01016410`, 10000.0 at
/// `01022958`).
const FLOAT_PAIR_004B1500: u32 = 0x004b_1500;
const DATE_HUNDRED: u32 = 0x0101_6410;
const DATE_TEN_THOUSAND: u32 = 0x0102_2958;
/// `__RTDynamicCast`'s type descriptors of `TESBoundObject` (`01183108`) and `BGSListForm`
/// (`011863d4`).
const BOUND_OBJECT_DESCRIPTOR: u32 = 0x0118_3108;
const LIST_FORM_DESCRIPTOR: u32 = 0x0118_63d4;
/// `BGSLoadGameBuffer` raw load (`00864980`; data, size), `LoadFormID` (`008648a0`, no stack
/// words), `LoadFormID_ov2` (`008648e0`; the address), `LoadVariableSizedValue`
/// (`00864a60`) (engine map; `thiscall` on the buffer), and the buffer's vtable slot `+0`
/// (its version, a byte).
const BUFFER_LOAD_BYTES: u32 = 0x0086_4980;
const BUFFER_LOAD_FORM_ID: u32 = 0x0086_48a0;
const BUFFER_LOAD_FORM_ID_INTO: u32 = 0x0086_48e0;
const BUFFER_LOAD_VARIABLE_SIZE: u32 = 0x0086_4a60;
/// `CombatTimer::LoadGame` (engine map, `009a5f20`; `thiscall`, one stack word).
const COMBAT_TIMER_LOAD_GAME: u32 = 0x009a_5f20;
/// `00726c60` (unnamed, `thiscall` on a list node, one stack word: the address of the new
/// item): stores the node's item.
const LIST_SET_ITEM_00726C60: u32 = 0x0072_6c60;
/// `FormListNumber` (`+0x50`, 16 bits), the word `+0x48` and `+0x54` the `_ov2` load reads
/// as form ids.
const FORM_ID_48: u32 = 0x48;
const FORM_ID_54: u32 = 0x54;
/// The change flag of the old saves' extra block (`0x800000`).
const OLD_EXTRA_BLOCK_FLAG: u32 = 0x0080_0000;

fn save_load_object(e: &Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME)
}

fn uses_save_game_blocks(e: &mut Engine) -> bool {
    let save = save_load_object(e);
    e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool()
}

fn save_load_position(e: &mut Engine) -> u32 {
    let save = save_load_object(e);
    e.call(SAVE_LOAD_POSITION, &args![save]).u32()
}

fn save_load_version(e: &mut Engine) -> u32 {
    let save = save_load_object(e);
    u32::from(e.call(SAVE_LOAD_VERSION, &args![save]).u8())
}

fn save_load_write(e: &mut Engine, address: u32, size: u32) {
    let save = save_load_object(e);
    e.call(SAVE_LOAD_WRITE, &args![save, address, size]);
}

fn save_load_read(e: &mut Engine, address: u32, size: u32) {
    let save = save_load_object(e);
    e.call(SAVE_LOAD_READ, &args![save, address, size]);
}

/// Whether the size reports are on (the byte at `00408d60(011de4e8)`).
fn size_reports_on(e: &mut Engine) -> bool {
    let at = e
        .call(SETTING_BYTE_ADDRESS, &args![SIZE_REPORT_SETTING])
        .u32();
    e.mem.u8(at) != 0
}

/// The save reports through `Error`: with the record of the form being saved (`004fd3e0`)
/// `form_format` with the size, the form's id, name and the word at `+5`, else `format` with
/// the size; both with `line` and the source file.
fn report_saved_size(e: &mut Engine, (form_format, format): (u32, u32), size: u32, line: u32) {
    let save = save_load_object(e);
    let record = e.call(SAVED_FORM_RECORD, &args![save]).u32();
    if record != 0 {
        let id = e.mem.u32(record);
        let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
        let word = e.mem.u32(record + 5);
        let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
        let id = e.mem.u32(record);
        e.call(
            ERROR_REPORT,
            &args![
                form_format,
                size,
                id,
                name,
                word,
                line,
                LOW_PROCESS_SOURCE_FILE
            ],
        );
    } else {
        e.call(
            ERROR_REPORT,
            &args![format, size, line, LOW_PROCESS_SOURCE_FILE],
        );
    }
}

/// The loaded form's report words, read in the exe's order (the word `+5`, the version byte
/// `+9`, the name, the id).
fn loaded_form_words(e: &mut Engine, record: u32, form: u32) -> (u32, u32, u32, u32) {
    let word = e.mem.u32(record + 5);
    let version = u32::from(e.mem.u8(record + 9));
    let name = e.vcall(form, FORM_NAME_SLOT, &args![]).u32();
    let id = e.mem.u32(record);
    (id, name, version, word)
}

/// The cast of the form of a saved id to the type `to` (`004839c0`, `__RTDynamicCast` from
/// `TESForm`).
fn form_of_id_as(e: &mut Engine, id: u32, to: u32) -> u32 {
    let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, FORM_TYPE_DESCRIPTOR, to, 0u32],
    )
    .u32()
}

/// A saved form id word of `this` replaced by its reference (0 for a zero id).
fn resolve_reference_word(e: &mut Engine, at: u32) {
    let id = e.mem.u32(at);
    let reference = if id != 0 {
        form_of_id_as(e, id, REFERENCE_TYPE_DESCRIPTOR)
    } else {
        0
    };
    e.mem.set_u32(at, reference);
}

// Translated from 0090f8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetSaveSize` (engine map; `thiscall`, two stack words: change flags and a
/// form, `RET 8`; a 16-bit answer): `BaseProcess::GetSaveSize`, 6 more with save-game blocks,
/// the value of the byte `m_uFlags`, 4 each from versions 0x4b, 0x4f and 0x56, 8, the damage
/// modifiers' size with flag 0x200000, 4 from version 0x74. With the size reports on, `Error`
/// reports this part's size (line 0xeda).
pub fn low_process_get_save_size(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    flags: u32,
    form: u32,
) -> u16 {
    let base = this.addr();
    let mut size: u16 = 0;
    let process = e
        .call(BASE_PROCESS_GET_SAVE_SIZE, &args![this, flags, form])
        .u16();
    size = size.wrapping_add(process);
    let before = size;
    if uses_save_game_blocks(e) {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(u16::from(e.mem.u8(base + 0x30)));
    for version in [0x4bu32, 0x4f, 0x56] {
        if save_load_version(e) >= version {
            size = size.wrapping_add(4);
        }
    }
    size = size.wrapping_add(8);
    if flags & SAVE_MODIFIERS_FLAG != 0 {
        let modifiers = e
            .call(MODIFIER_LIST_GET_SAVE_SIZE, &args![base + DAMAGE_MODIFIERS])
            .u16();
        size = size.wrapping_add(modifiers);
    }
    if save_load_version(e) >= 0x74 {
        size = size.wrapping_add(4);
    }
    if size_reports_on(e) {
        let part = i32::from(size).wrapping_sub(i32::from(before)) as u32;
        report_saved_size(
            e,
            (GET_SAVE_SIZE_FORM_FORMAT, GET_SAVE_SIZE_FORMAT),
            part,
            GET_SAVE_SIZE_LINE,
        );
    }
    size
}

/// The 4-byte numeric id of `reference` (`0084e3a0`, 0 for none) written through
/// `TESSaveLoadGame::SaveNumericID` from a local.
fn save_numeric_id(e: &mut Engine, reference: u32) {
    e.with_stack(4, |e, local| {
        let id = if reference != 0 {
            e.call(GET_WORD_0C, &args![reference]).u32()
        } else {
            0
        };
        e.mem.set_u32(local.addr(), id);
        let save = save_load_object(e);
        e.call(SAVE_NUMERIC_ID, &args![save, local, 4u32]);
    });
}

// Translated from 0090fa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::SaveGame` (engine map; `thiscall`, two stack words: change flags and a form,
/// `RET 8`): `BaseProcess::SaveGame`; with save-game blocks the marker `"KOLB"` and a 16-bit
/// size placeholder; `m_uFlags` (1 byte); from version 0x4b `fEssentialDownTimer`; from 0x4f
/// the numeric id of `pItemBeingUsed` (0 for none); from 0x56 `iNumberItemsActivate`; the
/// numeric ids of the target (0 while an acquire object is set) and the generic location;
/// the damage modifiers with flag 0x200000; from 0x74 `fDeathTime`; `fTrackedDamage`. With
/// the size reports on, `Error` reports the bytes written (line 0xf20). With blocks, the block
/// size goes into the placeholder (16 bits) after a debug print when it is over 0xffff.
pub fn low_process_save_game(e: &mut Engine, this: Ptr<LowProcess>, flags: u32, form: u32) {
    let base = this.addr();
    e.call(BASE_PROCESS_SAVE_GAME_V1, &args![this, flags, form]);
    e.with_stack(8, |e, locals| {
        let size_at = locals.addr();
        let marker_at = locals.addr() + 4;
        e.mem.set_u16(size_at, 0);
        let mut size_slot = 0u32;
        let mut start = save_load_position(e);
        if size_reports_on(e) {
            start = save_load_position(e);
        }
        if uses_save_game_blocks(e) {
            e.mem.set_u32(marker_at, BLOCK_MARKER);
            save_load_write(e, marker_at, 4);
            size_slot = save_load_position(e);
            save_load_write(e, size_at, 2);
        }
        save_load_write(e, base + 0x30, 1);
        if save_load_version(e) >= 0x4b {
            save_load_write(e, base + ESSENTIAL_DOWN_TIMER, 4);
        }
        if save_load_version(e) >= 0x4f {
            let item = e.mem.u32(base + 0x34);
            save_numeric_id(e, item);
        }
        if save_load_version(e) >= 0x56 {
            save_load_write(e, base + NUMBER_ITEMS_ACTIVATE, 4);
        }
        let target = if e.mem.u32(base + TARGET) != 0 && e.mem.u32(base + ACQUIRE_OBJECT) == 0 {
            e.mem.u32(base + TARGET)
        } else {
            0
        };
        save_numeric_id(e, target);
        let location = e.mem.u32(base + GENERIC_LOCATION);
        save_numeric_id(e, location);
        if flags & SAVE_MODIFIERS_FLAG != 0 {
            e.call(MODIFIER_LIST_SAVE_GAME_V1, &args![base + DAMAGE_MODIFIERS]);
        }
        if save_load_version(e) >= 0x74 {
            save_load_write(e, base + DEATH_TIME, 4);
        }
        save_load_write(e, base + TRACKED_DAMAGE, 4);
        if size_reports_on(e) {
            let end = save_load_position(e);
            report_saved_size(
                e,
                (SAVE_GAME_FORM_FORMAT, SAVE_GAME_FORMAT),
                end.wrapping_sub(start),
                SAVE_GAME_LINE,
            );
        }
        if uses_save_game_blocks(e) {
            let end = save_load_position(e);
            if end > size_slot.wrapping_add(0xffff) {
                e.call(
                    SAVE_DEBUG_PRINT,
                    &args![
                        BLOCK_TOO_BIG_FORMAT,
                        LOW_PROCESS_SOURCE_FILE,
                        SAVE_GAME_LINE
                    ],
                );
            }
            e.mem.set_u16(size_slot, end.wrapping_sub(size_slot) as u16);
        }
    });
}

/// The old saves' last processed date (before version 0x36): the `float` read goes through
/// `004b1500` with 100.0 (truncated by `_ftol2`: the day, bits 0-8 of `iDateLastProcessed`)
/// and with 10000.0 (truncated, divided by 100: the month, bits 9-12), and its own truncation
/// divided by 10000 is the year (from bit 13); `month << 9 | day | year << 13` is stored.
fn load_old_date(e: &mut Engine, base: u32) {
    e.with_stack(4, |e, local| {
        save_load_read(e, local.addr(), 4);
        let value = e.mem.u32(local.addr());
        let hundred = x87_quiet_bits(e.global::<u32>(DATE_HUNDRED));
        let day_value = e
            .call(FLOAT_PAIR_004B1500, &args![x87_quiet_bits(value), hundred])
            .f64();
        let day = e.call(FTOL, &args![day_value]).i32();
        let ten_thousand = x87_quiet_bits(e.global::<u32>(DATE_TEN_THOUSAND));
        let month_value = e
            .call(
                FLOAT_PAIR_004B1500,
                &args![x87_quiet_bits(value), ten_thousand],
            )
            .f64();
        let month = e.call(FTOL, &args![month_value]).i32().wrapping_div(100);
        let year = e
            .call(FTOL, &args![x87_load32(value)])
            .i32()
            .wrapping_div(10000);
        let date = (month << 9) | day | (year << 13);
        e.mem.set_i32(base + DATE_LAST_PROCESSED, date);
    });
}

// Translated from 0090fd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::LoadGame` (engine map; `thiscall`, three stack words: change flags and two
/// words passed on, `RET 0xc`): `BaseProcess::LoadGame`; with save-game blocks the block
/// header (a debug print when it is not `"KOLB"`, line 0xf2d), the block start and its 16-bit
/// size; before version 0x36 the old date ([`load_old_date`]); `m_uFlags`; from 0x4b
/// `fEssentialDownTimer`; from 0x4f `pItemBeingUsed` (numeric id, its form cast to
/// `TESBoundObject`); from 0x56 `iNumberItemsActivate`; the target id through `SetTarget`
/// (slot `+0x12c`) and the generic location id stored (resolved by `InitLoadGame`); the damage
/// modifiers with flag 0x200000; before 0x33 with flag 0x800000 a 16-bit count whose
/// `count * 8` bytes are skipped; from 0x74 `fDeathTime`; `fTrackedDamage`. With blocks a
/// position past or before the block's end prints an overrun or underrun (line 0xf77).
pub fn low_process_load_game(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    flags: u32,
    second: u32,
    third: u32,
) {
    let base = this.addr();
    e.call(BASE_PROCESS_LOAD_GAME, &args![this, flags, second, third]);
    e.with_stack(0x10, |e, locals| {
        let size_at = locals.addr();
        let marker_at = locals.addr() + 4;
        let word_at = locals.addr() + 8;
        e.mem.set_u16(size_at, 0);
        let mut start = 0u32;
        if uses_save_game_blocks(e) {
            save_load_read(e, marker_at, 4);
            if e.mem.u32(marker_at) != BLOCK_MARKER {
                let save = save_load_object(e);
                let record = e.call(LOADED_FORM_RECORD, &args![save]).u32();
                if record != 0 {
                    let id = e.mem.u32(record);
                    let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
                    let (id, name, version, word) = loaded_form_words(e, record, form);
                    e.call(
                        SAVE_DEBUG_PRINT,
                        &args![
                            BAD_HEADER_FORM_FORMAT,
                            LOW_PROCESS_SOURCE_FILE,
                            LOAD_HEADER_LINE,
                            id,
                            name,
                            version,
                            word
                        ],
                    );
                } else {
                    let version = save_load_version(e);
                    e.call(
                        SAVE_DEBUG_PRINT,
                        &args![
                            BAD_HEADER_FORMAT,
                            LOW_PROCESS_SOURCE_FILE,
                            LOAD_HEADER_LINE,
                            version
                        ],
                    );
                }
            }
            start = save_load_position(e);
            save_load_read(e, size_at, 2);
        }
        if save_load_version(e) < 0x36 {
            load_old_date(e, base);
        }
        save_load_read(e, base + 0x30, 1);
        if save_load_version(e) >= 0x4b {
            save_load_read(e, base + ESSENTIAL_DOWN_TIMER, 4);
        }
        if save_load_version(e) >= 0x4f {
            let save = save_load_object(e);
            e.call(LOAD_NUMERIC_ID, &args![save, word_at, 4u32]);
            let id = e.mem.u32(word_at);
            let item = form_of_id_as(e, id, BOUND_OBJECT_DESCRIPTOR);
            e.mem.set_u32(base + 0x34, item);
        }
        if save_load_version(e) >= 0x56 {
            save_load_read(e, base + NUMBER_ITEMS_ACTIVATE, 4);
        }
        let save = save_load_object(e);
        e.call(LOAD_NUMERIC_ID, &args![save, word_at, 4u32]);
        let target = e.mem.u32(word_at);
        e.vcall(base, SET_TARGET_SLOT, &args![target]);
        let save = save_load_object(e);
        e.call(LOAD_NUMERIC_ID, &args![save, word_at, 4u32]);
        let location = e.mem.u32(word_at);
        e.mem.set_u32(base + GENERIC_LOCATION, location);
        if flags & SAVE_MODIFIERS_FLAG != 0 {
            e.call(MODIFIER_LIST_LOAD_GAME, &args![base + DAMAGE_MODIFIERS]);
        }
        if save_load_version(e) < 0x33 && flags & OLD_EXTRA_BLOCK_FLAG != 0 {
            save_load_read(e, word_at, 2);
            let count = u32::from(e.mem.u16(word_at));
            if count != 0 {
                let save = save_load_object(e);
                e.call(SAVE_LOAD_SKIP, &args![save, count << 3]);
            }
        }
        if save_load_version(e) >= 0x74 {
            save_load_read(e, base + DEATH_TIME, 4);
        }
        save_load_read(e, base + TRACKED_DAMAGE, 4);
        if !uses_save_game_blocks(e) {
            return;
        }
        let end = save_load_position(e);
        let save = save_load_object(e);
        let record = e.call(LOADED_FORM_RECORD, &args![save]).u32();
        let expected = |e: &Engine| u32::from(e.mem.u16(size_at)).wrapping_add(start);
        if record != 0 {
            let id = e.mem.u32(record);
            let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
            let printed = if end > expected(e) {
                Some(OVERRUN_FORM_FORMAT)
            } else if end < expected(e) {
                Some(UNDERRUN_FORM_FORMAT)
            } else {
                None
            };
            if let Some(format) = printed {
                let (id, name, version, word) = loaded_form_words(e, record, form);
                let by = if format == OVERRUN_FORM_FORMAT {
                    end.wrapping_sub(expected(e))
                } else {
                    expected(e).wrapping_sub(end)
                };
                e.call(
                    SAVE_DEBUG_PRINT,
                    &args![
                        format,
                        by,
                        LOW_PROCESS_SOURCE_FILE,
                        LOAD_SIZE_LINE,
                        id,
                        name,
                        version,
                        word
                    ],
                );
            }
        } else if end > expected(e) {
            let version = save_load_version(e);
            let over = end.wrapping_sub(expected(e));
            e.call(
                SAVE_DEBUG_PRINT,
                &args![
                    OVERRUN_FORMAT,
                    over,
                    LOW_PROCESS_SOURCE_FILE,
                    LOAD_SIZE_LINE,
                    version
                ],
            );
        } else if end < expected(e) {
            let version = save_load_version(e);
            let under = expected(e).wrapping_sub(end);
            e.call(
                SAVE_DEBUG_PRINT,
                &args![
                    UNDERRUN_FORMAT,
                    under,
                    LOW_PROCESS_SOURCE_FILE,
                    LOAD_SIZE_LINE,
                    version
                ],
            );
        }
    });
}

// Translated from 00910630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::LoadGame_ov2` (engine map; `thiscall`, one stack word: the load buffer,
/// `RET 4`): `BaseProcess::LoadGame_ov2`; for a buffer version (its slot `+0`) below 6 two
/// words read and dropped; then `m_uFlags` (1 byte), `fEssentialDownTimer`, `pItemBeingUsed`
/// (a form id cast to `TESBoundObject`), `iNumberItemsActivate`, `fDeathTime`,
/// `fTrackedDamage`, the byte `+0xb0`, the 16-bit `+0x50`, the combat timer
/// (`CombatTimer::LoadGame`), the form ids `+0x40`, `+0x44`, `+0x48`, `+0x4c` (cast to
/// `BGSListForm`) and `+0x54`, then the generic locations: a count and each form id added to
/// `GenericLocationsList` (`005ae3d0`, from a local); the damage modifiers
/// (`ModifierList::LoadGame_ov2`) when the buffer's change flags have 0x200000.
pub fn low_process_load_game_ov2(e: &mut Engine, this: Ptr<LowProcess>, buffer: u32) {
    let base = this.addr();
    e.call(BASE_PROCESS_LOAD_GAME_OV2, &args![this, buffer]);
    e.with_stack(0x10, |e, locals| {
        if u32::from(e.vcall(buffer, 0, &args![]).u8()) < 6 {
            e.call(BUFFER_LOAD_BYTES, &args![buffer, locals.addr(), 4u32]);
            e.call(BUFFER_LOAD_BYTES, &args![buffer, locals.addr() + 4, 4u32]);
        }
        e.call(BUFFER_LOAD_BYTES, &args![buffer, base + 0x30, 1u32]);
        e.call(
            BUFFER_LOAD_BYTES,
            &args![buffer, base + ESSENTIAL_DOWN_TIMER, 4u32],
        );
        let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
        let item = form_of_id_as(e, id, BOUND_OBJECT_DESCRIPTOR);
        e.mem.set_u32(base + 0x34, item);
        for (offset, size) in [
            (NUMBER_ITEMS_ACTIVATE, 4u32),
            (DEATH_TIME, 4),
            (TRACKED_DAMAGE, 4),
            (SKIPPED_TIME_STAMP, 1),
            (FORM_LIST_NUMBER, 2),
        ] {
            e.call(BUFFER_LOAD_BYTES, &args![buffer, base + offset, size]);
        }
        e.call(
            COMBAT_TIMER_LOAD_GAME,
            &args![base + COMBAT_DELAY_TIMER, buffer],
        );
        for offset in [TARGET, GENERIC_LOCATION, FORM_ID_48] {
            e.call(BUFFER_LOAD_FORM_ID_INTO, &args![buffer, base + offset]);
        }
        let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
        let list = form_of_id_as(e, id, LIST_FORM_DESCRIPTOR);
        e.mem.set_u32(base + TARGET_FORM_LIST, list);
        e.call(BUFFER_LOAD_FORM_ID_INTO, &args![buffer, base + FORM_ID_54]);
        let count = e.call(BUFFER_LOAD_VARIABLE_SIZE, &args![buffer]).u32();
        let location = locals.addr() + 8;
        let mut index = 0u32;
        while index < count {
            e.mem.set_u32(location, 0);
            e.call(BUFFER_LOAD_FORM_ID_INTO, &args![buffer, location]);
            e.call(
                LIST_ADD_ITEM,
                &args![base + GENERIC_LOCATIONS_LIST, location],
            );
            index += 1;
        }
        let flags = e
            .call(SAVE_FLAGS_00428110, &args![buffer, locals.addr() + 0xc])
            .u32();
        if e.call(FLAGS_HAVE_004280F0, &args![flags, SAVE_MODIFIERS_FLAG])
            .bool()
        {
            e.call(
                MODIFIER_LIST_LOAD_GAME_OV2,
                &args![base + DAMAGE_MODIFIERS, buffer],
            );
        }
    });
}

// Translated from 00910820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::InitLoadGame_ov2` (engine map; `thiscall`, one stack word, `RET 4`):
/// `BaseProcess::InitLoadGame_ov2`; the target id (`+0x40`) becomes its reference through
/// `SetTarget` (slot `+0x12c`, after `+0x40` is cleared); the ids `+0x44`, `+0x48` and
/// `+0x54` become their references (0 for a zero id). Then `GenericLocationsList` is walked
/// from its head: an item whose id gives a reference keeps it (`00726c60` stores it); an
/// item without one is removed (through the previous node, `00905330`, or as the head,
/// `0063f7b0`, which leaves the walk on the same node).
pub fn low_process_init_load_game_ov2(e: &mut Engine, this: Ptr<LowProcess>, buffer: u32) {
    let base = this.addr();
    e.call(BASE_PROCESS_INIT_LOAD_GAME_OV2, &args![this, buffer]);
    let target_id = e.mem.u32(base + TARGET);
    let target = if target_id != 0 {
        form_of_id_as(e, target_id, REFERENCE_TYPE_DESCRIPTOR)
    } else {
        0
    };
    e.mem.set_u32(base + TARGET, 0);
    e.vcall(base, SET_TARGET_SLOT, &args![target]);
    for offset in [GENERIC_LOCATION, FORM_ID_48, FORM_ID_54] {
        resolve_reference_word(e, base + offset);
    }
    let mut node = base + GENERIC_LOCATIONS_LIST;
    let mut previous = 0u32;
    e.with_stack(4, |e, item_local| {
        while node != 0 {
            if e.call(LIST_IS_EMPTY, &args![node]).bool() {
                break;
            }
            let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
            let id = e.mem.u32(item_address);
            e.mem.set_u32(item_local.addr(), id);
            let mut remove = false;
            if id != 0 {
                let reference = if id != 0 {
                    form_of_id_as(e, id, REFERENCE_TYPE_DESCRIPTOR)
                } else {
                    0
                };
                e.mem.set_u32(item_local.addr(), reference);
                if reference != 0 {
                    e.call(LIST_SET_ITEM_00726C60, &args![node, item_local]);
                } else {
                    remove = true;
                }
            } else {
                remove = true;
            }
            if remove {
                if previous != 0 {
                    let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
                    e.call(LIST_REMOVE_ITEM, &args![previous, item_address]);
                    node = e.call(LIST_NEXT_NODE, &args![previous]).u32();
                } else {
                    e.call(LIST_REMOVE_HEAD, &args![node]);
                }
            } else {
                previous = node;
                node = e.call(LIST_NEXT_NODE, &args![node]).u32();
            }
        }
    });
}

/// `ExtraDataList::SetPackageExtraIndex` (`0041cae0`) and
/// `ExtraDataList::SetPackageExtraActionComplete` (`0041cbd0`) (engine map; `thiscall`, one
/// stack word), `ActorPackageData::CreateActorPackageData` (engine map, `008c6ca0`; `cdecl`,
/// one word: the package type) and the process's slot `+0x328` (one stack word).
const EXTRA_SET_PACKAGE_INDEX: u32 = 0x0041_cae0;
const EXTRA_SET_ACTION_COMPLETE: u32 = 0x0041_cbd0;
const CREATE_ACTOR_PACKAGE_DATA: u32 = 0x008c_6ca0;
const PROCESS_SLOT_328: u32 = 0x328;

// Translated from 00907ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::ShouldUpdateCombatControllerOrEndCombat` (engine map; `thiscall`, two stack
/// words: the actor and the address the package answer goes to, `RET 8`). Without a package
/// in the actor's extra data the answer word is 0 and `false`. When
/// [`low_process_should_stop_combat_controller_to_finish_package`] holds for it (and its
/// extra index), slot `+0x328(1)` runs, the word is 0 and the answer `true`. When
/// [`low_process_update_package_target_for_combat`] holds, the word is that package. Otherwise
/// the package is finished: its extra index becomes the last procedure
/// (`Procedure::GetProcedureCount - 1`), `Script::SetActionFlag(package, extra, 0x400)`,
/// `PackageDone(actor, package)` (slot `+0x5a0`), done-once, target-activated and
/// action-complete set (slots `+0x5a8`, `+0x4ec`, `+0x118` with 1) and the extra action
/// complete. Then the actor's package (`Actor::GetPackage`), when it is another one that
/// should use the combat controller ([`low_process_should_use_combat_controller_for_package`]
/// with 1), is started: its target (slot `+0x810`, else `+0x814(.., 0)`), the three flags
/// cleared, the actor's extra package set (`SetPackageExtra(package, 0, target,
/// action complete, target activated, done once)`), new actor package data when the type
/// makes one, `StartNewPackage` (slot `+0x598`), the combat target update, the actor's slot
/// `+0x48(0x80000000)`; the word is the new package. Otherwise the word is 0. Both answer
/// `false`.
pub fn low_process_should_update_combat_controller_or_end_combat(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    out: u32,
) -> bool {
    let base = this.addr();
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    let package = e.call(EXTRA_GET_PACKAGE, &args![extra]).u32();
    if package == 0 {
        e.mem.set_u32(out, 0);
        return false;
    }
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    let index = e.call(EXTRA_GET_PACKAGE_INDEX, &args![extra]).u32();
    if low_process_should_stop_combat_controller_to_finish_package(e, this, package, index) {
        e.vcall(base, PROCESS_SLOT_328, &args![1u32]);
        e.mem.set_u32(out, 0);
        return true;
    }
    if low_process_update_package_target_for_combat(e, this, actor, package) {
        e.mem.set_u32(out, package);
        return false;
    }
    let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
    let count = e.call(PROCEDURE_COUNT, &args![kind]).u32();
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    e.call(
        EXTRA_SET_PACKAGE_INDEX,
        &args![extra, count.wrapping_sub(1)],
    );
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    e.call(SET_ACTION_FLAG, &args![package, extra, 0x400u32]);
    e.vcall(base, PACKAGE_DONE_SLOT, &args![actor, package]);
    e.vcall(base, SET_PACKAGE_DONE_ONCE_SLOT, &args![1u32]);
    e.vcall(base, SET_TARGET_ACTIVATED_SLOT, &args![1u32]);
    e.vcall(base, SET_CURRENT_ACTION_COMPLETE_SLOT, &args![1u32]);
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    e.call(EXTRA_SET_ACTION_COMPLETE, &args![extra, 1u32]);
    let next = e.call(ACTOR_GET_PACKAGE, &args![actor]).u32();
    if next == 0
        || next == package
        || !low_process_should_use_combat_controller_for_package(e, this, Ptr::new(next), 1)
    {
        e.mem.set_u32(out, 0);
        return false;
    }
    let mut target = e
        .vcall(base, GET_TARGET_FOR_PACKAGE_SLOT, &args![actor, next])
        .u32();
    if target == 0 {
        target = e
            .vcall(
                base,
                GET_LOCATION_FOR_PACKAGE_SLOT,
                &args![actor, next, 0u32],
            )
            .u32();
    }
    e.vcall(base, SET_PACKAGE_DONE_ONCE_SLOT, &args![0u32]);
    e.vcall(base, SET_TARGET_ACTIVATED_SLOT, &args![0u32]);
    e.vcall(base, SET_CURRENT_ACTION_COMPLETE_SLOT, &args![0u32]);
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    let done_once = u32::from(e.vcall(base, IS_PACKAGE_DONE_ONCE_SLOT, &args![]).u8());
    let target_activated = u32::from(e.vcall(base, IS_TARGET_ACTIVATED_SLOT, &args![]).u8());
    let action_complete = u32::from(
        e.vcall(base, GET_CURRENT_ACTION_COMPLETE_SLOT, &args![])
            .u8(),
    );
    e.call(
        EXTRA_SET_PACKAGE,
        &args![
            extra,
            next,
            0u32,
            target,
            action_complete,
            target_activated,
            done_once
        ],
    );
    let kind = e.call(PACKAGE_GET_TYPE, &args![next]).u32();
    let data = e.call(CREATE_ACTOR_PACKAGE_DATA, &args![kind]).u32();
    if data != 0 {
        e.call(EXTRA_SET_ACTOR_PACKAGE_DATA, &args![extra, data]);
    }
    e.vcall(base, START_NEW_PACKAGE_SLOT, &args![actor, next]);
    low_process_update_package_target_for_combat(e, this, actor, next);
    e.vcall(actor, ACTOR_SLOT_48, &args![0x8000_0000u32]);
    e.mem.set_u32(out, next);
    false
}

/// The running package's tests (`008e30e0`, `008e3120`, `008e3100`; unnamed, engine map unit
/// `highprocess.cpp`; `thiscall`, no stack words; `bool`s in `AL`).
const RUNNING_TEST_008E30E0: u32 = 0x008e_30e0;
const RUNNING_TEST_008E3120: u32 = 0x008e_3120;
const RUNNING_TEST_008E3100: u32 = 0x008e_3100;
/// The reference's vtable slots `+0x100` (is an actor), `+0x218` and the actor's `+0x21c`
/// (no stack words; `bool`s in `AL`).
const REFERENCE_SLOT_100: u32 = 0x100;
const ACTOR_SLOT_21C: u32 = 0x21c;
/// `TESValueForm::GetFormValue` (`0048e8a0`, `cdecl`), `Actor::GetGoldAmount` (`00891d70`),
/// `004181e0` (unnamed, `extradatalist.cpp`), `TESAIForm::SellBuysItem` (`0047f600`, one stack
/// word; on the `TESAIForm` at `+0x90` of `004181e0`'s answer) and `TESObjectREFR::GetLock`
/// (`00569160`) (engine map).
const FORM_VALUE: u32 = 0x0048_e8a0;
const ACTOR_GET_GOLD_AMOUNT: u32 = 0x0089_1d70;
const BASE_OBJECT_004181E0: u32 = 0x0041_81e0;
const AI_FORM_SELL_BUYS_ITEM: u32 = 0x0047_f600;
/// The `TESAIForm` inside the base object `004181e0` answers.
const AI_FORM_OFFSET: u32 = 0x90;
/// The form whose objects the actor needs for a locked item (`011ca268`, a pointer).
const KEY_FORM_011CA268: u32 = 0x011c_a268;
/// `00905820` (unnamed, engine map unit `highprocess.cpp`; `thiscall` on `ObjectList`, one
/// stack word: the address of the entry).
const OBJECT_LIST_ADD_00905820: u32 = 0x0090_5820;
/// `ObjectstoAcquire` words: the reference `+0`, the form `+4`, the bytes `+8` and `+9`, the
/// distance `+0x10` and the state `+0x18`.
const ACQUIRE_REFERENCE: u32 = 0x0;
const ACQUIRE_FORM: u32 = 0x4;
const ACQUIRE_FLAG_8: u32 = 0x8;
const ACQUIRE_FLAG_9: u32 = 0x9;
const ACQUIRE_DISTANCE: u32 = 0x10;
const ACQUIRE_STATE: u32 = 0x18;

/// `TESObjectREFR::GetDistanceFromReference(actor, reference, 0, 0)` truncated (`_ftol2`).
fn whole_distance(e: &mut Engine, actor: u32, reference: u32) -> u32 {
    let distance = e
        .call(REFERENCE_GET_DISTANCE, &args![actor, reference, 0u32, 0u32])
        .f64();
    e.call(FTOL, &args![distance]).u32()
}

/// Whether `actor` is set and `00566950` holds for it.
fn flagged_actor(e: &mut Engine, actor: u32) -> bool {
    actor != 0 && e.call(ACTOR_TEST_00566950, &args![actor]).bool()
}

/// One entry of `EvaluateOrderAcquireList` from the point where the exe tests the owner
/// (`00907ec4` on): answers whether the entry is kept.
fn evaluate_acquire_entry(
    e: &mut Engine,
    actor: u32,
    entry: u32,
    owner_actor: u32,
    (sells, wanted, dangerous): (bool, bool, bool),
    mut keep: bool,
) -> bool {
    let object = e.mem.u32(entry + ACQUIRE_REFERENCE);
    let form = e.mem.u32(entry + ACQUIRE_FORM);
    let mut want = false;
    let mut steal = false;
    let mut handled = false;
    let price = e.call(FORM_VALUE, &args![form]).i32();
    if !e.vcall(object, ACTOR_SLOT_22C, &args![0u32]).bool()
        && e.call(ACTOR_GET_GOLD_AMOUNT, &args![actor]).i32() >= price
    {
        let seller = if e.vcall(object, REFERENCE_SLOT_218, &args![]).bool() {
            object
        } else {
            let owner = e.call(REFERENCE_GET_OWNER, &args![object]).u32();
            let npc = e
                .call(
                    DYNAMIC_CAST,
                    &args![owner, 0u32, FORM_TYPE_DESCRIPTOR, NPC_TYPE_DESCRIPTOR, 0u32],
                )
                .u32();
            if npc != 0 {
                e.call(
                    PROCESS_LISTS_ACTOR_IN_HIGH,
                    &args![PROCESS_LISTS, npc, 0u32],
                )
                .u32()
            } else {
                0
            }
        };
        if seller != 0 && sells {
            let base_object = e.call(BASE_OBJECT_004181E0, &args![seller]).u32();
            if e.call(
                AI_FORM_SELL_BUYS_ITEM,
                &args![base_object.wrapping_add(AI_FORM_OFFSET), form],
            )
            .bool()
            {
                if !e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
                    e.mem.set_u32(entry + ACQUIRE_STATE, 2);
                    let distance = whole_distance(e, actor, object);
                    e.mem.set_u32(entry + ACQUIRE_DISTANCE, distance);
                    handled = true;
                } else {
                    keep = false;
                }
            }
        }
    }
    if handled || flagged_actor(e, owner_actor) {
        return keep;
    }
    if !e.vcall(object, REFERENCE_SLOT_100, &args![]).bool() {
        if wanted {
            want = true;
        }
    } else if e.vcall(object, ACTOR_SLOT_22C, &args![0u32]).bool() {
        keep = true;
    } else if e.vcall(object, REFERENCE_SLOT_218, &args![]).bool() {
        if wanted {
            want = true;
        }
        if dangerous {
            steal = true;
        }
    }
    if (want || steal) && !flagged_actor(e, owner_actor) {
        if !e.vcall(object, REFERENCE_SLOT_100, &args![]).bool() {
            if e.mem.u32(entry + ACQUIRE_STATE) == 1 {
                e.mem.set_u32(entry + ACQUIRE_STATE, 4);
                let has_key = e.with_stack(4, |e, local| {
                    let key = e.global::<u32>(KEY_FORM_011CA268);
                    e.call(
                        ACTOR_HAS_OBJECTS,
                        &args![actor, key, 0u32, 1u32, 0u32, local],
                    )
                    .bool()
                });
                if !has_key && e.call(REFERENCE_GET_LOCK, &args![object]).u32() != 0 {
                    keep = false;
                }
            } else {
                e.mem.set_u32(entry + ACQUIRE_STATE, 3);
            }
        } else if want {
            if !e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
                e.mem.set_u32(entry + ACQUIRE_STATE, 4);
            } else {
                keep = false;
            }
        } else if steal {
            if !e.call(ACTOR_TEST_00493BB0, &args![actor]).bool()
                && !e.call(ACTOR_TEST_00566950, &args![actor]).bool()
            {
                if owner_actor == 0 || !e.call(ACTOR_TEST_00566950, &args![owner_actor]).bool() {
                    e.mem.set_u32(entry + ACQUIRE_STATE, 5);
                }
            } else {
                keep = false;
            }
        }
        e.mem.set_u8(entry + ACQUIRE_FLAG_8, u8::from(want));
        e.mem.set_u8(entry + ACQUIRE_FLAG_9, u8::from(steal));
        return keep;
    }
    if e.vcall(actor, ACTOR_SLOT_21C, &args![]).bool() {
        return false;
    }
    if !e.vcall(object, REFERENCE_SLOT_100, &args![]).bool()
        || e.vcall(object, REFERENCE_SLOT_218, &args![]).bool()
    {
        return false;
    }
    let owner = e.call(REFERENCE_GET_OWNER, &args![object]).u32();
    if owner == e.call(REFERENCE_GET_BASE_FORM, &args![actor]).u32()
        || e.call(ACTOR_TEST_00566950, &args![actor]).bool()
    {
        return false;
    }
    if owner_actor != 0 && !e.call(ACTOR_TEST_00566950, &args![owner_actor]).bool() {
        e.mem.set_u32(entry + ACQUIRE_STATE, 5);
        keep = true;
    }
    keep
}

// Translated from 00907d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::EvaluateOrderAcquireList` (engine map; `thiscall`, one stack word: the actor,
/// `RET 4`). The running package (slot `+0x27c`) gives three tests (`008e30e0`, `008e3120`,
/// `008e3100`; false without a package). Each entry of `TempObjectList` (`+0x74`, an
/// `ObjectstoAcquire`: reference, form, flags `+8`/`+9`, distance `+0x10`, state `+0x18`) is
/// judged ([`evaluate_acquire_entry`]): kept by default, dropped for a flagged owner actor
/// (`00566950`); an unowned or own (the actor's base form) non-actor reference only gets its
/// distance (`_ftol2` of `GetDistanceFromReference`). Kept entries are marked targeted
/// (`SetTargeted(1)`) and moved to `ObjectList` (`00905820`); the others deleted
/// (`007b3fa0(1)`). The temporary list is emptied at the end.
pub fn low_process_evaluate_order_acquire_list(e: &mut Engine, this: Ptr<LowProcess>, actor: u32) {
    let base = this.addr();
    let mut node = base + TEMP_OBJECT_LIST;
    let running = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
    let test =
        |e: &mut Engine, address: u32| running != 0 && e.call(address, &args![running]).bool();
    let sells = test(e, RUNNING_TEST_008E30E0);
    let wanted = test(e, RUNNING_TEST_008E3120);
    let dangerous = test(e, RUNNING_TEST_008E3100);
    while node != 0 {
        let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        let entry = e.mem.u32(item_address);
        let object = e.mem.u32(entry + ACQUIRE_REFERENCE);
        let owner_actor = if e.vcall(object, REFERENCE_SLOT_100, &args![]).bool() {
            object
        } else {
            0
        };
        let mut keep = true;
        if flagged_actor(e, owner_actor) {
            keep = false;
        }
        let owner = e.call(REFERENCE_GET_OWNER, &args![object]).u32();
        let own = owner == 0 || {
            let owner = e.call(REFERENCE_GET_OWNER, &args![object]).u32();
            owner == e.call(REFERENCE_GET_BASE_FORM, &args![actor]).u32()
        };
        if own && !e.vcall(object, REFERENCE_SLOT_100, &args![]).bool() {
            let distance = whole_distance(e, actor, object);
            e.mem.set_u32(entry + ACQUIRE_DISTANCE, distance);
        } else {
            keep = evaluate_acquire_entry(
                e,
                actor,
                entry,
                owner_actor,
                (sells, wanted, dangerous),
                keep,
            );
        }
        if keep {
            let reference = e.mem.u32(entry + ACQUIRE_REFERENCE);
            e.call(REFERENCE_SET_TARGETED, &args![reference, 1u32]);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), entry);
                e.call(OBJECT_LIST_ADD_00905820, &args![base + OBJECT_LIST, slot]);
            });
        } else if entry != 0 {
            e.call(DELETE_EMPTY_DESTRUCTOR, &args![entry, 1u32]);
        }
        node = e.call(LIST_NEXT_NODE, &args![node]).u32();
    }
    e.call(LIST_CLEAR, &args![base + TEMP_OBJECT_LIST]);
}

/// `004077c0` (unnamed, `thiscall` on the reference, no stack words; a `bool` in `AL`), the
/// actor's vtable slot `+0x1a0` (one stack word) and `008859e0` (unnamed, `actor.cpp`;
/// `thiscall` on the actor, one stack word: the reference).
const REFERENCE_TEST_004077C0: u32 = 0x0040_77c0;
const ACTOR_SLOT_1A0: u32 = 0x1a0;
const ACTOR_TEST_008859E0: u32 = 0x0088_59e0;
/// `__RTDynamicCast`'s type descriptors of `BaseProcess` (`0118a6b8`) and `LowProcess`
/// (`011a364c`).
const BASE_PROCESS_DESCRIPTOR: u32 = 0x0118_a6b8;
const LOW_PROCESS_DESCRIPTOR: u32 = 0x011a_364c;
/// `TESPackage::FormMatchesPackageObjectType` (engine map, `00679e00`; `cdecl`: form, type).
const FORM_MATCHES_PACKAGE_OBJECT_TYPE: u32 = 0x0067_9e00;
/// The inventory walk of a reference: the count (`00575590`, two stack words) and the item
/// at an index (`005754a0`, two stack words), `ItemChange::GetWorn` (`004bddd0`, one stack
/// word) and the item's delete (`004459e0`, one stack word) (`thiscall`).
const REFERENCE_ITEM_COUNT: u32 = 0x0057_5590;
const REFERENCE_ITEM_AT: u32 = 0x0057_54a0;
const ITEM_GET_WORN: u32 = 0x004b_ddd0;
const ITEM_DELETE: u32 = 0x0044_59e0;
/// The form type whose references are searched like actors (0x1b).
const CONTAINER_LIKE_FORM_TYPE: u32 = 0x1b;

/// Marks the reference targeted and adds it (through the address of the word holding it) to
/// the process's `TempLocList` (`00905820`).
fn add_temp_location(e: &mut Engine, process: u32, reference: u32) {
    e.call(REFERENCE_SET_TARGETED, &args![reference, 1u32]);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), reference);
        e.call(
            OBJECT_LIST_ADD_00905820,
            &args![process + TEMP_LOC_LIST, slot],
        );
    });
}

// Translated from 0090e660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference callback `GetLocationForPackage` hands to
/// `TESDataHandler::EnumReferencesCloseToPoint` (no map name; `cdecl`: reference, actor;
/// answers `false` to go on, `true` only for a null actor or one without a `LowProcess`).
/// Skips null, `00440d80`, `004077c0` and `00440da0` references and, when the actor's slot
/// `+0x1a0(0)` holds, references `008859e0` refuses. The actor's process cast to
/// `LowProcess` then takes the reference into `TempLocList` (targeted) when its base form
/// matches `eFormType` (`+0x8c`, and the reference's slot `+0x22c(0)` is false), or is
/// `pObjectForLocation` (`+0x88`, the same test), or when neither is set. Actor references
/// other than the player, and references whose base form type is 0x1b, also offer each
/// not-worn inventory item (`00575590`/`005754a0`): an item whose object (`0044ddc0`)
/// matches the type, is the object, or any item when the type, `pObjecttoAcquire` and the
/// object are all unset, takes the reference in again; each item is deleted (`004459e0(1)`).
pub fn fn_0090e660(e: &mut Engine, reference: u32, actor: u32) -> bool {
    if reference == 0
        || e.call(REFERENCE_TEST_00440D80, &args![reference]).bool()
        || e.call(REFERENCE_TEST_004077C0, &args![reference]).bool()
        || e.call(REFERENCE_TEST_00440DA0, &args![reference]).bool()
    {
        return false;
    }
    if e.vcall(actor, ACTOR_SLOT_1A0, &args![0u32]).bool()
        && !e.call(ACTOR_TEST_008859E0, &args![actor, reference]).bool()
    {
        return false;
    }
    if actor == 0 {
        return true;
    }
    let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
    let process = e
        .call(
            DYNAMIC_CAST,
            &args![
                process,
                0u32,
                BASE_PROCESS_DESCRIPTOR,
                LOW_PROCESS_DESCRIPTOR,
                0u32
            ],
        )
        .u32();
    if process == 0 {
        return true;
    }
    let is_actor = e.vcall(reference, REFERENCE_SLOT_100, &args![]).bool();
    let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
    let form_type = e.call(FORM_GET_TYPE, &args![base_form]).u32();
    let wanted_type = e.mem.u32(process + FORM_TYPE_FOR_LOCATION);
    let matches_type = wanted_type != 0 && {
        let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
        e.call(
            FORM_MATCHES_PACKAGE_OBJECT_TYPE,
            &args![base_form, wanted_type],
        )
        .bool()
    };
    // The exe has the same block for the type match and the object match.
    let takes = matches_type
        || (e.mem.u32(process + OBJECT_FOR_LOCATION) != 0
            && e.mem.u32(process + OBJECT_FOR_LOCATION)
                == e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32());
    if takes {
        if !e.vcall(reference, ACTOR_SLOT_22C, &args![0u32]).bool() {
            add_temp_location(e, process, reference);
        }
    } else if e.mem.u32(process + FORM_TYPE_FOR_LOCATION) == 0
        && e.mem.u32(process + OBJECT_FOR_LOCATION) == 0
    {
        add_temp_location(e, process, reference);
    }
    let searches =
        (is_actor && reference != e.global::<u32>(PLAYER)) || form_type == CONTAINER_LIKE_FORM_TYPE;
    if !searches {
        return false;
    }
    let count = e
        .call(REFERENCE_ITEM_COUNT, &args![reference, 0u32, 1u32])
        .i32();
    let mut index = 0i32;
    while index < count {
        let item = e
            .call(REFERENCE_ITEM_AT, &args![reference, index, 0u32])
            .u32();
        if item != 0 {
            if !e.call(ITEM_GET_WORN, &args![item, 0u32]).bool() {
                let wanted_type = e.mem.u32(process + FORM_TYPE_FOR_LOCATION);
                let matches_type = wanted_type != 0 && {
                    let object = e.call(WORD_08_0044DDC0, &args![item]).u32();
                    e.call(
                        FORM_MATCHES_PACKAGE_OBJECT_TYPE,
                        &args![object, wanted_type],
                    )
                    .bool()
                };
                let matched = matches_type
                    || e.call(WORD_08_0044DDC0, &args![item]).u32()
                        == e.mem.u32(process + OBJECT_FOR_LOCATION)
                    || (e.mem.u32(process + FORM_TYPE_FOR_LOCATION) == 0
                        && e.mem.u32(process + OBJECT_TO_ACQUIRE) == 0
                        && e.mem.u32(process + OBJECT_FOR_LOCATION) == 0);
                if matched {
                    add_temp_location(e, process, reference);
                }
            }
            e.call(ITEM_DELETE, &args![item, 1u32]);
        }
        index += 1;
    }
    false
}

/// `TESPackage::IsInterruptPackage` (`00678610`), `ExtraDataList::GetPackageExtraTarget`
/// (`0041cb70`), `MobileObject::GetCurrentPackage` (`009344a0`) and
/// `TESPackage::GetLocationReference` (`00676140`, one stack word) (engine map; `thiscall`)
/// and the process's slot `+0x514` (no stack words).
const PACKAGE_IS_INTERRUPT_PACKAGE: u32 = 0x0067_8610;
const EXTRA_GET_PACKAGE_EXTRA_TARGET: u32 = 0x0041_cb70;
const MOBILE_GET_CURRENT_PACKAGE_009344A0: u32 = 0x0093_44a0;
const PACKAGE_GET_LOCATION_REFERENCE_00676140: u32 = 0x0067_6140;
const PROCESS_SLOT_514: u32 = 0x514;

/// How `GetCurrentDestinationReference` treats a procedure number (the exe's byte map
/// `0090ec2c` and jump table `0090ec14`).
fn reference_case(procedure: u32) -> u32 {
    match procedure {
        0 | 4 | 5 | 7 | 50 => 0,
        1 | 2 | 3 | 6 | 8 | 9 | 13 | 14 | 15 | 28 | 30 | 38 | 42 | 44 => 1,
        27 => 2,
        47 => 3,
        54 => 4,
        _ => 5,
    }
}

/// The package location's reference (`0055b980`, `PackageLocation::GetLocReference`), 0
/// without a location.
fn location_reference(e: &mut Engine, package: u32) -> u32 {
    if e.call(PACKAGE_LOCATION_0055B980, &args![package]).u32() == 0 {
        return 0;
    }
    let location = e.call(PACKAGE_LOCATION_0055B980, &args![package]).u32();
    e.call(LOCATION_GET_REFERENCE, &args![location]).u32()
}

/// The actor's process's slot `+0x514`, 0 without a process.
fn process_slot_514(e: &mut Engine, actor: u32) -> u32 {
    if e.call(ACTOR_GET_PROCESS, &args![actor]).u32() == 0 {
        return 0;
    }
    let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
    e.vcall(process, PROCESS_SLOT_514, &args![]).u32()
}

// Translated from 0090e970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetCurrentDestinationReference` (engine map; `thiscall`, two stack words:
/// the actor and a flag, `RET 8`). The current package, target (`+0x40`) and running
/// procedure index; without the flag, a missing or interrupt package is replaced by the
/// actor's extra package, index and target. 0 without a package or procedure type. By the
/// procedure number ([`reference_case`]; above 0x36 the target): the package location's
/// reference, else the actor's process's slot `+0x514`, else the actor (case 0); the target,
/// else the actor (1); for an actor target its current package's location reference (2);
/// with `00901c50` and a location its reference, else (with a process) the process's target
/// (slot `+0x128`), its slot `+0x514`, the actor (3); the location's reference (kept as the
/// target without one), else the process's slot `+0x514`, else the actor (4); the target
/// (5).
pub fn low_process_get_current_destination_reference(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    keep_package: u8,
) -> u32 {
    let base = this.addr();
    let mut package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let mut target = e.mem.u32(base + TARGET);
    let mut index = e
        .vcall(base, GET_PROCEDURE_INDEX_RUNNING_SLOT, &args![])
        .u32();
    if keep_package == 0
        && (package == 0 || e.call(PACKAGE_IS_INTERRUPT_PACKAGE, &args![package]).bool())
    {
        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
        package = e.call(EXTRA_GET_PACKAGE, &args![extra]).u32();
        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
        index = e.call(EXTRA_GET_PACKAGE_INDEX, &args![extra]).u32();
        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
        target = e.call(EXTRA_GET_PACKAGE_EXTRA_TARGET, &args![extra]).u32();
    }
    if package == 0 {
        return 0;
    }
    if e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).i32() == -1 {
        return 0;
    }
    let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
    let list = e
        .mem
        .u32(PROCEDURE_LISTS.wrapping_add(kind.wrapping_mul(4)));
    let procedure = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
    if procedure > PROCEDURE_DONE {
        return target;
    }
    match reference_case(procedure) {
        0 => {
            target = location_reference(e, package);
            if target == 0 {
                target = process_slot_514(e, actor);
            }
            if target == 0 {
                target = actor;
            }
        }
        1 => {
            if target == 0 {
                target = actor;
            }
        }
        2 => {
            if target != 0 && e.vcall(target, REFERENCE_SLOT_100, &args![]).bool() {
                let mobile = target;
                let current = e
                    .call(MOBILE_GET_CURRENT_PACKAGE_009344A0, &args![mobile])
                    .u32();
                if current != 0 {
                    target = e
                        .call(
                            PACKAGE_GET_LOCATION_REFERENCE_00676140,
                            &args![current, mobile],
                        )
                        .u32();
                }
            }
        }
        3 => {
            if e.call(PACKAGE_TEST_00901C50, &args![package]).bool()
                && e.call(PACKAGE_LOCATION_0055B980, &args![package]).u32() != 0
            {
                let location = e.call(PACKAGE_LOCATION_0055B980, &args![package]).u32();
                target = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
            } else {
                let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
                if process != 0 {
                    target = e.vcall(process, GET_TARGET_SLOT, &args![]).u32();
                    if target == 0 {
                        target = e.vcall(process, PROCESS_SLOT_514, &args![]).u32();
                    }
                    if target == 0 {
                        target = actor;
                    }
                }
            }
        }
        4 => {
            if e.call(PACKAGE_LOCATION_0055B980, &args![package]).u32() != 0 {
                let location = e.call(PACKAGE_LOCATION_0055B980, &args![package]).u32();
                target = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
            }
            if target == 0 {
                target = process_slot_514(e, actor);
            }
            if target == 0 {
                target = actor;
            }
        }
        _ => {}
    }
    target
}

/// The default destination radius (`float` 128.0, `0101e704`) and the default the exe uses
/// for the discarded radius of case 3 (`float` 30.0, `01018f5c`).
const DEFAULT_DESTINATION_RADIUS: u32 = 0x0101_e704;
const DEFAULT_TARGET_RADIUS: u32 = 0x0101_8f5c;
/// `TESPackage::GetPackageRadiusActorToRefTarget` (engine map, `006787e0`; `thiscall`, two
/// stack words; in `ST0`).
const PACKAGE_RADIUS_ACTOR_TO_REF_TARGET: u32 = 0x0067_87e0;

/// How `GetCurrentDestinationRadius` treats a procedure number (the exe's byte map
/// `0090f87c` and jump table `0090f868`).
fn radius_case(procedure: u32) -> u32 {
    match procedure {
        0 | 50 => 0,
        1..=3 | 8 | 14 | 15 => 1,
        6 | 7 | 13 | 31 => 2,
        47 => 3,
        _ => 4,
    }
}

// Translated from 0090f670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LowProcess::GetCurrentDestinationRadius` (engine map; `thiscall`, one stack word: the
/// actor, `RET 4`; in `ST0`). 128.0 (`0101e704`) without a package or for a procedure number
/// above 0x32; otherwise by the number ([`radius_case`]): with a package location
/// `TESPackage::GetPackageRadiusActorToLocation(actor, 0)` (case 0); with a package target
/// `GetPackageRadiusActorToRefTarget(actor, 0)` (cases 1 and 2, the exe has the same code
/// twice); case 3 computes a radius (`00676280` with `00901c50` and a location, else the
/// target's `0044ddc0` word, else 30.0) into a local the function never reads, and keeps the
/// default. Each result is stored as a `float`.
pub fn low_process_get_current_destination_radius(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
) -> f32 {
    let base = this.addr();
    let mut radius = x87_quiet_bits(e.global::<u32>(DEFAULT_DESTINATION_RADIUS));
    if e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32() == 0 {
        return f32::from_bits(radius);
    }
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let kind = e.call(PACKAGE_GET_PROCEDURE_TYPE, &args![package]).u32();
    let index = e
        .vcall(base, GET_PROCEDURE_INDEX_RUNNING_SLOT, &args![])
        .u32();
    let list = e
        .mem
        .u32(PROCEDURE_LISTS.wrapping_add(kind.wrapping_mul(4)));
    let procedure = e.mem.u32(list.wrapping_add(index.wrapping_mul(4)));
    if procedure > 0x32 {
        return x87_quiet(f32::from_bits(radius));
    }
    let current = |e: &mut Engine| e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    match radius_case(procedure) {
        0 => {
            let package = current(e);
            if e.call(PACKAGE_LOCATION_0055B980, &args![package]).u32() != 0 {
                let package = current(e);
                let value = e
                    .call(
                        PACKAGE_RADIUS_ACTOR_TO_LOCATION,
                        &args![package, actor, 0u32],
                    )
                    .f64();
                radius = x87_store32(value);
            }
        }
        1 | 2 => {
            let package = current(e);
            if e.call(PACKAGE_GET_TARGET, &args![package]).u32() != 0 {
                let package = current(e);
                let value = e
                    .call(
                        PACKAGE_RADIUS_ACTOR_TO_REF_TARGET,
                        &args![package, actor, 0u32],
                    )
                    .f64();
                radius = x87_store32(value);
            }
        }
        3 => {
            let package = current(e);
            let target = e.call(PACKAGE_GET_TARGET, &args![package]).u32();
            let package = current(e);
            let uses_location = e.call(PACKAGE_TEST_00901C50, &args![package]).bool() && {
                let package = current(e);
                e.call(PACKAGE_LOCATION_0055B980, &args![package]).u32() != 0
            };
            // The exe stores this into a local it never reads again (`[EBP-8]`).
            let _unread = if uses_location {
                let package = current(e);
                let value = e.call(PACKAGE_GET_00676280, &args![package, actor]).u32();
                x87_store32(f64::from(value))
            } else if target != 0 {
                let word = e.call(WORD_08_0044DDC0, &args![target]).i32();
                x87_store32(f64::from(word))
            } else {
                x87_quiet_bits(e.global::<u32>(DEFAULT_TARGET_RADIUS))
            };
        }
        _ => {}
    }
    x87_quiet(f32::from_bits(radius))
}

/// `00422700` (unnamed, `extradatalist.cpp`: the actor's followers extra),
/// `ExtraDataList::RemoveFollower` (`00422690`, one stack word) and
/// `ExtraDataList::RemoveFollowerExtra` (`00422720`) (engine map; `thiscall` on the extra
/// list), and `Actor::GetPackageSetAsPcurrent` (engine map, `00881510`; `thiscall`).
const EXTRA_GET_FOLLOWERS: u32 = 0x0042_2700;
const EXTRA_REMOVE_FOLLOWER: u32 = 0x0042_2690;
const EXTRA_REMOVE_FOLLOWER_EXTRA: u32 = 0x0042_2720;
const ACTOR_GET_PACKAGE_SET_AS_CURRENT: u32 = 0x0088_1510;
/// The follower list in the followers extra (`+0xc`).
const FOLLOWER_LIST: u32 = 0xc;

/// A new 8-byte list (`00401000`, `0096a2d0`), 0 when the allocation fails.
fn new_list(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    if block != 0 {
        e.call(PACKAGE_LIST_CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

/// A list emptied (`00470470`) and, when set, deleted (`004702f0(1)`).
fn delete_list(e: &mut Engine, list: u32) {
    e.call(LIST_CLEAR, &args![list]);
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
}

/// The package that counts for an actor: its current one (`Actor::GetPackageSetAsPcurrent`),
/// replaced by its extra package (`ExtraDataList::GetPackageExtra`) as `replace` says.
fn counted_package(
    e: &mut Engine,
    actor: u32,
    replace: impl FnOnce(&mut Engine, u32, u32) -> u32,
) -> u32 {
    let package = e
        .call(ACTOR_GET_PACKAGE_SET_AS_CURRENT, &args![actor])
        .u32();
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    let extra_package = e.call(EXTRA_GET_PACKAGE, &args![extra]).u32();
    replace(e, package, extra_package)
}

// Translated from 00911b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00911b00 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two list allocations) is not translated
/// `LowProcess::UpdateFollowers` (engine map; `thiscall`, two stack words: the actor and a
/// `float` time, `RET 8`). With the actor's followers extra (`00422700`): the follower list
/// (`+0xc`) is copied into a new list; for each follower a dead actor (its slot `+0x22c(0)` or
/// `00440da0`) drops; otherwise a follower other than the player is kept updating while the
/// actor's counted package (current, or the extra one for a missing or interrupt package) is
/// of type 2 or the follower's (current, or its extra one) is of type 1 or 7: a follower with
/// a process (`MobileObject::GetCurrentProcessType`) gets its slot `+0x25c(time)` and
/// `TESObjectREFR::RunScript`; the others drop. Dropped followers are removed
/// (`ExtraDataList::RemoveFollower`); both lists are emptied and deleted; an emptied follower
/// list removes the extra (`RemoveFollowerExtra`).
pub fn low_process_update_followers(
    e: &mut Engine,
    _unused_0: Ptr<LowProcess>,
    actor: u32,
    time: f32,
) {
    let dead = e.vcall(actor, ACTOR_SLOT_22C, &args![0u32]).bool()
        || e.call(REFERENCE_TEST_00440DA0, &args![actor]).bool();
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
    let followers = e.call(EXTRA_GET_FOLLOWERS, &args![extra]).u32();
    if followers == 0 {
        return;
    }
    let removed = new_list(e);
    let copy = new_list(e);
    let mut node = e.mem.u32(followers + FOLLOWER_LIST);
    while node != 0 {
        let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        e.call(OBJECT_LIST_ADD_00905820, &args![copy, item_address]);
        node = e.call(LIST_NEXT_NODE, &args![node]).u32();
    }
    e.with_stack(4, |e, follower_slot| {
        let mut node = copy;
        while node != 0 {
            let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
            if e.mem.u32(item_address) == 0 {
                break;
            }
            let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
            let follower = e.mem.u32(item_address);
            e.mem.set_u32(follower_slot.addr(), follower);
            if dead {
                e.call(LIST_ADD_ITEM, &args![removed, follower_slot]);
            } else if follower != 0 && follower != e.global::<u32>(PLAYER) {
                let package = counted_package(e, actor, |e, package, extra_package| {
                    let replace = package == 0
                        || e.call(PACKAGE_IS_INTERRUPT_PACKAGE, &args![package]).bool();
                    if replace && extra_package != 0 {
                        extra_package
                    } else {
                        package
                    }
                });
                let guarding = package != 0 && e.call(PACKAGE_GET_TYPE, &args![package]).u32() == 2;
                let follower_package = counted_package(e, follower, |e, package, extra_package| {
                    if package == 0 {
                        return extra_package;
                    }
                    if e.call(PACKAGE_IS_INTERRUPT_PACKAGE, &args![package]).bool()
                        && extra_package != 0
                    {
                        extra_package
                    } else {
                        package
                    }
                });
                let follows = guarding
                    || (follower_package != 0
                        && (e.call(PACKAGE_GET_TYPE, &args![follower_package]).u32() == 1
                            || e.call(PACKAGE_GET_TYPE, &args![follower_package]).u32() == 7));
                if !follows {
                    e.call(LIST_ADD_ITEM, &args![removed, follower_slot]);
                } else if e.call(MOBILE_GET_PROCESS_TYPE, &args![follower]).u32() != 0 {
                    e.vcall(
                        follower,
                        SET_AVOID_WAIT_TIMER_SLOT,
                        &args![x87_quiet_bits(time.to_bits())],
                    );
                    e.call(REFERENCE_RUN_SCRIPT, &args![follower]);
                }
            }
            node = e.call(LIST_NEXT_NODE, &args![node]).u32();
        }
    });
    let mut node = removed;
    while node != 0 {
        let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        if e.mem.u32(item_address) == 0 {
            break;
        }
        let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
        let follower = e.mem.u32(item_address);
        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
        e.call(EXTRA_REMOVE_FOLLOWER, &args![extra, follower]);
        node = e.call(LIST_NEXT_NODE, &args![node]).u32();
    }
    delete_list(e, copy);
    delete_list(e, removed);
    let list = e.mem.u32(followers + FOLLOWER_LIST);
    if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![actor]).u32();
        e.call(EXTRA_REMOVE_FOLLOWER_EXTRA, &args![extra]);
    }
}

/// The actor's vtable slots `+0x29c` (one stack word: the point written; `0087d600` in the
/// `Character` table copies `+0x160`), `+0x298` and `+0x294` (no stack words), and the
/// process's `+0x20c` (`GetRunOncePackage`).
const ACTOR_SLOT_29C: u32 = 0x29c;
const ACTOR_SLOT_298: u32 = 0x298;
const ACTOR_SLOT_294: u32 = 0x294;
/// The guard radius factor (`double` 0.5, `01011588`) and its lower bound (`float` 15.0,
/// `0101e580`).
const GUARD_RADIUS_FACTOR: u32 = 0x0101_1588;
const GUARD_RADIUS_MINIMUM: u32 = 0x0101_e580;

/// Builds and sets a pathing request for the actor (`006e2420`, then the goal, `008b3880(request,
/// point, first, second, radius, 0)`, `008e7c50(0)`, `Actor::SetPathfindingGoal`), marks the data
/// `006ecd40(1)` and destroys the request.
fn start_guard_travel(
    e: &mut Engine,
    actor: u32,
    data: u32,
    goal: impl FnOnce(&mut Engine) -> (u32, u32, u32),
    radius: u32,
) {
    e.with_stack(PATHING_REQUEST_SIZE, |e, request| {
        e.call(PATHING_REQUEST_CONSTRUCT, &args![request]);
        let (point, first, second) = goal(e);
        e.call(
            ACTOR_BUILD_REQUEST,
            &args![
                actor,
                request,
                point,
                first,
                second,
                x87_quiet_bits(radius),
                0u32
            ],
        );
        e.call(PATHING_REQUEST_SET_008E7C50, &args![request, 0u32]);
        e.call(ACTOR_SET_PATHFINDING_GOAL, &args![actor, request]);
        e.call(SET_INSTANCE_DATA_006ECD40, &args![data, 1u32]);
        e.call(PATHING_REQUEST_DESTRUCT, &args![request]);
    });
}

// Translated from 00912db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00912db0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two pathing requests) is not translated
/// `LowProcess::ProcessGuardStartTravelToLocation` (engine map; `thiscall`, two stack words:
/// the actor and the package data, `RET 8`). From the running package's location (`0055b980`)
/// by its type (`PackageLocation::GetLocType`): 3 takes a point (the actor's slot `+0x29c`)
/// and the actor's slots `+0x298` and `+0x294`; 2 the actor as the reference; 6
/// `00569b80` of the actor; other types the location's reference, else (without a run-once
/// package, slot `+0x20c`) `pGenericLocation`. Without a reference (and not type 3) the
/// target (slot `+0x128`). The radius is `00676280` with a location, else the package
/// target's `0044ddc0` word, else 30.0; then the larger of half of it and 15.0 (`00404010`).
/// A reference gets a pathing request to its position (slot `+0x1f4`), cell (`008d6f30`) and
/// world space; else the point with the two slot answers when either is set.
pub fn low_process_process_guard_start_travel_to_location(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    data: u32,
) {
    let base = this.addr();
    e.with_stack(0x18, |e, locals| {
        let point = locals.addr();
        let temp = locals.addr() + 0xc;
        e.call(LIST_HEAD_ITEM, &args![point]);
        let mut reference = 0u32;
        let mut first = 0u32;
        let mut second = 0u32;
        let running = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
        let location = e.call(PACKAGE_LOCATION_0055B980, &args![running]).u32();
        let target = e.call(PACKAGE_GET_TARGET, &args![running]).u32();
        let mut use_target = true;
        if location != 0 {
            if e.call(PACKAGE_LOCATION_GET_TYPE, &args![location]).u32() == 3 {
                let at = e.vcall(actor, ACTOR_SLOT_29C, &args![temp]).u32();
                for i in 0..3 {
                    let word = e.mem.u32(at + 4 * i);
                    e.mem.set_u32(point + 4 * i, word);
                }
                first = e.vcall(actor, ACTOR_SLOT_298, &args![]).u32();
                second = e.vcall(actor, ACTOR_SLOT_294, &args![]).u32();
                use_target = false;
            } else if e.call(PACKAGE_LOCATION_GET_TYPE, &args![location]).u32() == 2 {
                reference = actor;
            } else if e.call(PACKAGE_LOCATION_GET_TYPE, &args![location]).u32() == 6 {
                reference = e.call(ACTOR_GET_00569B80, &args![actor]).u32();
            } else {
                reference = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
                if reference == 0 && e.vcall(base, GET_RUN_ONCE_PACKAGE_SLOT, &args![]).u32() == 0 {
                    reference = e.mem.u32(base + GENERIC_LOCATION);
                }
            }
        }
        if reference == 0 && use_target {
            reference = e.vcall(base, GET_TARGET_SLOT, &args![]).u32();
        }
        let radius = if location != 0 {
            let value = e.call(PACKAGE_GET_00676280, &args![running, actor]).u32();
            x87_store32(f64::from(value))
        } else if target != 0 {
            let word = e.call(WORD_08_0044DDC0, &args![target]).i32();
            x87_store32(f64::from(word))
        } else {
            x87_quiet_bits(e.global::<u32>(DEFAULT_TARGET_RADIUS))
        };
        let factor: f64 = e.global(GUARD_RADIUS_FACTOR);
        let half = x87_store32(x87_mul(x87_load32(radius), factor));
        let minimum = x87_quiet_bits(e.global::<u32>(GUARD_RADIUS_MINIMUM));
        let larger = e
            .call(LARGER_OF_00404010, &args![x87_quiet_bits(half), minimum])
            .f64();
        let radius = x87_store32(larger);
        if reference != 0 {
            let goal = move |e: &mut Engine| {
                let world_space = e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).u32();
                let cell = e.call(ACTOR_GET_008D6F30, &args![reference]).u32();
                let position = e.vcall(reference, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
                (position, cell, world_space)
            };
            start_guard_travel(e, actor, data, goal, radius);
        } else if first != 0 || second != 0 {
            start_guard_travel(e, actor, data, move |_| (point, first, second), radius);
        }
    });
}

/// `008a2d40` (unnamed, `actor.cpp`; `thiscall` on the actor, no stack words): the escorted
/// item; `00575450` (unnamed; `thiscall` on the actor, one stack word: a numeric id; a `bool`
/// in `AL`); `TESObjectREFR::GetInventoryItem` (`00576260`, two stack words);
/// `TESObjectREFR::HasContainer` (`0055d310`); `00892e90` (unnamed, `actor.cpp`; five stack
/// words) and the actor's vtable slot `+0x3cc` (five stack words); the item delete
/// (`004459e0(1)`).
const ACTOR_ESCORT_ITEM_008A2D40: u32 = 0x008a_2d40;
const ACTOR_TEST_00575450: u32 = 0x0057_5450;
const REFERENCE_GET_INVENTORY_ITEM: u32 = 0x0057_6260;
const REFERENCE_HAS_CONTAINER: u32 = 0x0055_d310;
const ACTOR_GIVE_ITEM_00892E90: u32 = 0x0089_2e90;
const ACTOR_SLOT_3CC: u32 = 0x3cc;
/// `PathingLocation::PathingLocation_ov7` (`006dcee0`, three stack words: point, cell, world
/// space) and `Pathing::ComputeLowPathDistance` (`006d6200`, `cdecl`: from, to, the address of
/// the `float` distance) (engine map).
const PATHING_LOCATION_CONSTRUCT_AT: u32 = 0x006d_cee0;
const COMPUTE_LOW_PATH_DISTANCE: u32 = 0x006d_6200;

/// Whether the target (`+0x40`) is set and is an actor (slot `+0x100`).
fn target_is_actor(e: &mut Engine, base: u32) -> bool {
    let target = e.mem.u32(base + TARGET);
    target != 0 && e.vcall(target, REFERENCE_SLOT_100, &args![]).bool()
}

/// Whether the target is set and is an actor, or the actor accepts its numeric id
/// (`00575450`).
fn target_is_actor_or_known(e: &mut Engine, base: u32, actor: u32) -> bool {
    if e.mem.u32(base + TARGET) == 0 {
        return false;
    }
    if target_is_actor(e, base) {
        return true;
    }
    let target = e.mem.u32(base + TARGET);
    let id = e.call(GET_WORD_0C, &args![target]).u32();
    e.call(ACTOR_TEST_00575450, &args![actor, id]).bool()
}

/// The first word of the item's extra list (`00559450`, `006815c0`), `fallback` without one.
fn item_extra(e: &mut Engine, item: u32, fallback: u32) -> u32 {
    if e.call(WORD_AT_00559450, &args![item]).u32() == 0 {
        return fallback;
    }
    let list = e.call(WORD_AT_00559450, &args![item]).u32();
    let head = e.call(LIST_HEAD_ITEM, &args![list]).u32();
    e.mem.u32(head)
}

/// The two path distances of `ProcessEscort`: from the actor and from the target to the
/// package location (`TESPackage::GetLocationCoord`, `GetLocationCell`, `GetLocationWorld`),
/// through `PathingLocation`s and `Pathing::ComputeLowPathDistance`.
fn escort_path_distances(e: &mut Engine, base: u32, actor: u32, running: u32) -> (u32, u32) {
    let mut actor_distance = x87_store32(0.0);
    let mut target_distance = x87_store32(0.0);
    let size = 0xc + 3 * PATHING_LOCATION_SIZE + 8;
    e.with_stack(size, |e, locals| {
        let point = locals.addr();
        let from_actor = point + 0xc;
        let to_location = from_actor + PATHING_LOCATION_SIZE;
        let from_target = to_location + PATHING_LOCATION_SIZE;
        let distances = from_target + PATHING_LOCATION_SIZE;
        e.mem.set_u32(distances, actor_distance);
        e.mem.set_u32(distances + 4, target_distance);
        e.call(PACKAGE_GET_LOCATION_COORD, &args![running, point, actor]);
        let cell = e
            .call(PACKAGE_GET_LOCATION_CELL, &args![running, actor])
            .u32();
        let world = e
            .call(PACKAGE_GET_LOCATION_WORLD, &args![running, actor])
            .u32();
        e.call(PATHING_LOCATION_CONSTRUCT, &args![from_actor, actor]);
        e.call(
            PATHING_LOCATION_CONSTRUCT_AT,
            &args![to_location, point, cell, world],
        );
        e.call(
            COMPUTE_LOW_PATH_DISTANCE,
            &args![from_actor, to_location, distances],
        );
        let target = e.mem.u32(base + TARGET);
        e.call(PATHING_LOCATION_CONSTRUCT, &args![from_target, target]);
        e.call(
            COMPUTE_LOW_PATH_DISTANCE,
            &args![from_target, to_location, distances + 4],
        );
        e.call(PATHING_LOCATION_DESTRUCT, &args![from_target]);
        e.call(PATHING_LOCATION_DESTRUCT, &args![to_location]);
        e.call(PATHING_LOCATION_DESTRUCT, &args![from_actor]);
        actor_distance = e.mem.u32(distances);
        target_distance = e.mem.u32(distances + 4);
    });
    (actor_distance, target_distance)
}

/// `ProcessEscort`'s hand-over of the escorted item when the package's slot `+0x13c` is done
/// (the target is not an actor): to the destination's container (`00892e90`) or through the
/// actor's slot `+0x3cc`.
fn escort_give_item(e: &mut Engine, base: u32, actor: u32, running: u32, item: u32) {
    let mut extra = item_extra(e, item, 0);
    let mut id = 0u32;
    if extra != 0 && e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32() != 0 {
        let pointer = e.call(EXTRA_GET_REFERENCE_POINTER, &args![extra]).u32();
        id = e.call(GET_WORD_0C, &args![pointer]).u32();
    }
    let location = e.call(PACKAGE_LOCATION_0055B980, &args![running]).u32();
    let mut destination = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
    if destination == 0 {
        destination = e.mem.u32(base + GENERIC_LOCATION);
    }
    if destination != 0 && e.call(REFERENCE_HAS_CONTAINER, &args![destination]).u32() != 0 {
        let count = e
            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![running])
            .u32();
        let object = e.call(WORD_08_0044DDC0, &args![item]).u32();
        e.call(
            ACTOR_GIVE_ITEM_00892E90,
            &args![actor, object, extra, destination, count, id],
        );
        return;
    }
    extra = item_extra(e, item, extra);
    let mut position = 0u32;
    let location = e.call(PACKAGE_LOCATION_0055B980, &args![running]).u32();
    let target = e.call(PACKAGE_GET_TARGET, &args![running]).u32();
    if location != 0 {
        let mut reference = e.call(LOCATION_GET_REFERENCE, &args![location]).u32();
        if reference == 0 {
            reference = e.mem.u32(base + GENERIC_LOCATION);
        }
        if reference != 0 {
            let marker = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32()
                == e.global::<u32>(ACTIVATE_DONE_FORM)
                || e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32()
                    == e.global::<u32>(ANGLED_FORM);
            if marker {
                position = e.vcall(reference, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
            }
        }
    }
    let mut count = 1u32;
    if e.call(PACKAGE_TARGET_GET_REFERENCE, &args![target]).u32() == 0 {
        count = e
            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![running])
            .u32();
    }
    let object = e.call(WORD_08_0044DDC0, &args![item]).u32();
    e.vcall(
        actor,
        ACTOR_SLOT_3CC,
        &args![object, extra, count, position, 0u32],
    );
}

// Translated from 00909430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00909430 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the three pathing locations) is not translated
/// `LowProcess::ProcessEscort` (engine map; `thiscall`, one stack word: the actor, `RET 4`).
/// Without a target the package's target is set (slot `+0x8c`). Without an escorted item
/// (`008a2d40`) and with no actor or known target (after setting the package's target again)
/// the procedure moves on. An acquire object held by the actor gives the item
/// (`GetInventoryItem`). With the package target's count (`GetInitialTargetCount`), a target
/// that is not an actor or known and for which `CheckIfHasObject` (slot `+0x80c`) fails moves
/// the procedure on. Otherwise ([`low_process_get_time_delta`] runs): when the current
/// package's slot `+0x13c(actor, 0, -1.0, 0)` holds, an actor target moves on once the
/// package's slot `+0x13c` holds for it (or there is no target), and any other target gets the
/// item ([`escort_give_item`]) and moves on; when it fails, the actor walks to the package
/// location (`Actor::SetPathfindingGoal_ov2` with -1.0, once its pathing is complete) unless
/// an actor target is nearer than the package's radius (`_ftol2`, `FCOMPP`, `TEST AH,0x41`)
/// and not farther along the path than the actor (`TEST AH,5`, `JP`). The item is deleted at
/// the end.
pub fn low_process_process_escort(e: &mut Engine, this: Ptr<LowProcess>, actor: u32) {
    let base = this.addr();
    let running = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
    if e.mem.u32(base + TARGET) == 0 {
        e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
    }
    let mut count = 0u32;
    let mut item = e.call(ACTOR_ESCORT_ITEM_008A2D40, &args![actor]).u32();
    if item == 0 && !target_is_actor_or_known(e, base, actor) {
        e.vcall(base, SET_TARGET_FOR_PACKAGE_SLOT, &args![actor]);
        if !target_is_actor_or_known(e, base, actor) {
            e.vcall(
                base,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, 1u32],
            );
            return;
        }
    }
    if item == 0 {
        let acquire = e.mem.u32(base + ACQUIRE_OBJECT);
        if acquire != 0 && e.mem.u32(acquire) == actor {
            let object = e.mem.u32(acquire + 4);
            item = e
                .call(REFERENCE_GET_INVENTORY_ITEM, &args![actor, object, 0u32])
                .u32();
        }
    }
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    if e.call(PACKAGE_GET_TARGET, &args![package]).u32() != 0 {
        let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
        count = e
            .call(PACKAGE_GET_INITIAL_TARGET_COUNT, &args![package])
            .u32();
    }
    if !target_is_actor_or_known(e, base, actor)
        && !e
            .vcall(base, CHECK_IF_HAS_OBJECT_SLOT, &args![actor, count])
            .bool()
    {
        e.vcall(
            base,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            &args![actor, 1u32],
        );
        if item != 0 {
            e.call(ITEM_DELETE, &args![item, 1u32]);
        }
        return;
    }
    low_process_get_time_delta(e, this, actor);
    let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    if e.vcall(
        package,
        PACKAGE_SLOT_13C,
        &args![actor, 0u32, minus_one, 0u32],
    )
    .bool()
    {
        let actor_target = e.mem.u32(base + TARGET) != 0 && target_is_actor(e, base);
        if !actor_target {
            if item != 0 {
                escort_give_item(e, base, actor, running, item);
            }
            e.vcall(
                base,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, 1u32],
            );
        } else if e.mem.u32(base + TARGET) != 0 {
            let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
            let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
            let target = e.mem.u32(base + TARGET);
            if e.vcall(
                package,
                PACKAGE_SLOT_13C,
                &args![target, 0u32, minus_one, 0u32],
            )
            .bool()
            {
                e.vcall(
                    base,
                    ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                    &args![actor, 1u32],
                );
            }
        } else {
            e.vcall(
                base,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                &args![actor, 1u32],
            );
        }
    } else {
        let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
        let radius = e
            .call(
                PACKAGE_RADIUS_ACTOR_TO_REF_TARGET,
                &args![package, actor, 0u32],
            )
            .f64();
        let radius = e.call(FTOL, &args![radius]).i32();
        let (mut actor_distance, mut target_distance) = (x87_store32(0.0), x87_store32(0.0));
        if target_is_actor(e, base) {
            (actor_distance, target_distance) = escort_path_distances(e, base, actor, running);
        }
        let mut walks = true;
        if target_is_actor(e, base) {
            let target = e.mem.u32(base + TARGET);
            let distance = e
                .call(REFERENCE_GET_DISTANCE, &args![actor, target, 0u32, 0u32])
                .f64();
            // FILD radius; FCOMPP (radius against the distance); TEST AH,0x41; JZ walks.
            let radius_beyond = f64::from(radius) > distance;
            if !radius_beyond {
                // FLD actor; FLD target; FCOMPP; TEST AH,5; JP skips the walk.
                let target_nearer = x87_load32(target_distance) < x87_load32(actor_distance);
                walks = target_nearer;
            }
        }
        if walks && e.call(ACTOR_IS_PATHING_COMPLETE, &args![actor]).bool() {
            e.with_stack(0xc, |e, coord| {
                let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
                let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
                let world = e
                    .call(PACKAGE_GET_LOCATION_WORLD, &args![package, actor])
                    .u32();
                let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
                let cell = e
                    .call(PACKAGE_GET_LOCATION_CELL, &args![package, actor])
                    .u32();
                let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
                let point = e
                    .call(PACKAGE_GET_LOCATION_COORD, &args![package, coord, actor])
                    .u32();
                e.call(
                    ACTOR_SET_PATHFINDING_GOAL_TO_LOCATION,
                    &args![actor, point, cell, world, minus_one, 0u32],
                );
            });
        }
    }
    if item != 0 {
        e.call(ITEM_DELETE, &args![item, 1u32]);
    }
}

/// `Actor::IsInFaction` (`008b8e90`, one stack word), `ExtraDataList::GetLevCreaOriginalBase`
/// (`004216f0`), `0073b460` (unnamed, `bookmenu.cpp`), `00568ad0` (unnamed,
/// `tesobjectrefr.cpp`; in `ST0`), `ExtraDataList::GetCount` (`00418770`; a 16-bit answer),
/// `ExtraDataList::GetContainerChanges` (`00418520`), `004bffb0` (unnamed,
/// `inventorychanges.cpp`: the changes' list), `InventoryChanges::GetInventoryItem`
/// (`004d0650`, two stack words), `TESObjectREFR::GetSpace` (`00575ca0`),
/// `InventoryChanges::GetInventoryChanges` (`004bf220`, `cdecl`),
/// `InventoryChanges::GetObjectCount` (`004c8f30`, one stack word) and
/// `ItemChange::GetItemHealth` (`004bcdb0`, one stack word; in `ST0`) (engine map; `thiscall`
/// unless said).
const ACTOR_IS_IN_FACTION: u32 = 0x008b_8e90;
const EXTRA_GET_LEVELED_ORIGINAL_BASE: u32 = 0x0042_16f0;
const BOOK_TEST_0073B460: u32 = 0x0073_b460;
const REFERENCE_VALUE_00568AD0: u32 = 0x0056_8ad0;
const EXTRA_GET_COUNT: u32 = 0x0041_8770;
const EXTRA_GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
const CHANGES_LIST_004BFFB0: u32 = 0x004b_ffb0;
const CHANGES_GET_INVENTORY_ITEM: u32 = 0x004d_0650;
const REFERENCE_GET_SPACE: u32 = 0x0057_5ca0;
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
const CHANGES_GET_OBJECT_COUNT: u32 = 0x004c_8f30;
const ITEM_GET_HEALTH: u32 = 0x004b_cdb0;
/// `BaseProcess::GetActorPackageThatIsRunning` in the engine map (`00717e50`; folded code, here
/// on the container changes' list: its first node).
const NODE_PART_00717E50: u32 = 0x0071_7e50;
/// The `ObjectstoAcquire` entry's count (`+0xc`) and extra list (`+0x14`).
const ACQUIRE_COUNT: u32 = 0xc;
const ACQUIRE_EXTRA: u32 = 0x14;

/// Whether an object `form` is wanted by the process's acquire settings: it matches the type
/// (`eFormType`, `+0x8c`), is `pObjecttoAcquire` (`+0x84`), or neither is set.
fn acquire_settings_match(e: &mut Engine, process: u32, form: u32) -> bool {
    let wanted_type = e.mem.u32(process + FORM_TYPE_FOR_LOCATION);
    (wanted_type != 0
        && e.call(FORM_MATCHES_PACKAGE_OBJECT_TYPE, &args![form, wanted_type])
            .bool())
        || (e.mem.u32(process + OBJECT_TO_ACQUIRE) != 0
            && e.mem.u32(process + OBJECT_TO_ACQUIRE) == form)
        || (e.mem.u32(process + FORM_TYPE_FOR_LOCATION) == 0
            && e.mem.u32(process + OBJECT_TO_ACQUIRE) == 0)
}

/// A new `ObjectstoAcquire` (`00401000(0x1c)`, [`fn_0090e600`]; 0 when the allocation fails).
fn new_acquire_entry(e: &mut Engine) -> u32 {
    let block = e.call(ALLOCATE, &args![0x1cu32]).u32();
    if block != 0 {
        fn_0090e600(e, Ptr::new(block)).addr()
    } else {
        0
    }
}

/// Adds the entry (through the address of a word holding it) to `TempObjectList` (`+0x74`).
fn add_temp_object(e: &mut Engine, process: u32, entry: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        e.call(
            OBJECT_LIST_ADD_00905820,
            &args![process + TEMP_OBJECT_LIST, slot],
        );
    });
}

/// An entry for an inventory object of the container reference: state 1 and the count
/// (`GetObjectCount` of its inventory changes when it has a container, else 1).
fn new_inventory_entry(e: &mut Engine, reference: u32, object: u32) -> u32 {
    let entry = new_acquire_entry(e);
    e.mem.set_u32(entry + ACQUIRE_FORM, object);
    e.mem.set_u32(entry + ACQUIRE_REFERENCE, reference);
    e.mem.set_u32(entry + ACQUIRE_STATE, 1);
    if e.call(REFERENCE_HAS_CONTAINER, &args![reference]).u32() != 0 {
        let changes = e.call(GET_INVENTORY_CHANGES, &args![reference]).u32();
        if changes != 0 {
            let object = e.mem.u32(entry + ACQUIRE_FORM);
            let count = e
                .call(CHANGES_GET_OBJECT_COUNT, &args![changes, object])
                .u32();
            e.mem.set_u32(entry + ACQUIRE_COUNT, count);
        }
    } else {
        e.mem.set_u32(entry + ACQUIRE_COUNT, 1);
    }
    entry
}

/// How the acquire callback treats `eFormType` for the inventory search (the exe's byte map
/// `0090e5e0` and jump table `0090e5d8`): types whose search is on.
fn acquire_searches_type(kind: u32) -> bool {
    matches!(
        kind,
        0 | 2 | 3 | 4 | 7 | 8 | 9 | 10 | 12 | 13 | 16 | 17 | 18 | 19 | 20 | 22 | 23
    )
}

// Translated from 0090dd80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0090dd80 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the entry allocations) is not translated
/// The reference callback `ProcessAcquire` hands to `TESDataHandler::EnumReferencesCloseToPoint`
/// (no map name; `cdecl`: reference, actor; answers `false` to go on, `true` only without an
/// actor or a process in acquire states 0 to 3). Skips null, the actor's own, `00440d80`,
/// `004077c0` and `00440da0` references. The reference itself is wanted when its base form
/// matches `eFormType` (and, for an actor with `pFactiontoAcquire`, it is in that faction),
/// is `pObjecttoAcquire` (or, for an actor, its levelled original base is), or neither is
/// set; a book (`0x19`) for which `0073b460` holds, or a light (`0x1e`) for which `0046f070`
/// does not, is not; type 0x28 is not when the actor `00493bb0` holds and `00568ad0` of the
/// reference is at most 0.0. A wanted reference becomes a new `ObjectstoAcquire` in
/// `TempObjectList` (its base, count from `ExtraDataList::GetCount`, state 0, extra list).
/// For a searched type ([`acquire_searches_type`]) on an actor other than the player, or for
/// form type 0x1b, the container changes' items not already in the inventory and the
/// inventory items not worn that match the settings in the same space as the actor become
/// entries of state 1 too; the walked inventory items are deleted.
pub fn fn_0090dd80(e: &mut Engine, reference: u32, actor: u32) -> bool {
    if reference == 0
        || reference == actor
        || e.call(REFERENCE_TEST_00440D80, &args![reference]).bool()
        || e.call(REFERENCE_TEST_004077C0, &args![reference]).bool()
        || e.call(REFERENCE_TEST_00440DA0, &args![reference]).bool()
    {
        return false;
    }
    if actor == 0 {
        return true;
    }
    let mut wanted = false;
    let mut process = 0u32;
    let is_actor = e.vcall(reference, REFERENCE_SLOT_100, &args![]).bool();
    let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
    let form_type = e.call(FORM_GET_TYPE, &args![base_form]).u32();
    if e.call(ACTOR_GET_PROCESS, &args![actor]).u32() != 0 {
        let candidate = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        let state = e.call(PROCESS_GET_0045CD60, &args![candidate]).i32();
        if (0..=3).contains(&state) {
            process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        }
    }
    if process == 0 {
        return true;
    }
    let wanted_type = e.mem.u32(process + FORM_TYPE_FOR_LOCATION);
    if wanted_type != 0
        && e.call(
            FORM_MATCHES_PACKAGE_OBJECT_TYPE,
            &args![base_form, wanted_type],
        )
        .bool()
    {
        let faction = e.mem.u32(process + FACTION_TO_ACQUIRE);
        // The exe sets the flag in two places: no faction or not an actor, or in the faction.
        if faction == 0
            || !is_actor
            || e.call(ACTOR_IS_IN_FACTION, &args![reference, faction])
                .bool()
        {
            wanted = true;
        }
    } else if e.mem.u32(process + OBJECT_TO_ACQUIRE) != 0 {
        if e.mem.u32(process + OBJECT_TO_ACQUIRE) == base_form {
            wanted = true;
        } else if is_actor {
            let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![reference]).u32();
            let original = e.call(EXTRA_GET_LEVELED_ORIGINAL_BASE, &args![extra]).u32();
            if original == e.mem.u32(process + OBJECT_TO_ACQUIRE) {
                wanted = true;
            }
        }
    } else if e.mem.u32(process + FORM_TYPE_FOR_LOCATION) == 0
        && e.mem.u32(process + OBJECT_TO_ACQUIRE) == 0
    {
        wanted = true;
    }
    match form_type {
        0x19 => {
            if e.call(BOOK_TEST_0073B460, &args![base_form]).bool() {
                wanted = false;
            }
        }
        0x1e => {
            if !e.call(LIGHT_TEST_0046F070, &args![base_form]).bool() {
                wanted = false;
            }
        }
        _ => {}
    }
    // A local the exe sets to 0 and only tests in the item loop.
    let health_test = 0u32;
    let mut busy = false;
    if form_type == 0x28 {
        let owner = if base_form != 0 { base_form + 0x94 } else { 0 };
        if e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
            busy = true;
        }
        if owner != 0 {
            let value = e.call(REFERENCE_VALUE_00568AD0, &args![reference]).f64();
            let zero: f64 = e.global(ZERO);
            // FCOMP 0.0; TEST AH,0x41; JP skips: only below or equal go on.
            if value <= zero && busy {
                wanted = false;
            }
        }
    }
    if wanted {
        let entry = new_acquire_entry(e);
        let base_form = e.call(REFERENCE_GET_BASE_FORM, &args![reference]).u32();
        e.mem.set_u32(entry + ACQUIRE_FORM, base_form);
        e.mem.set_u32(entry + ACQUIRE_REFERENCE, reference);
        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![reference]).u32();
        let count = e.call(EXTRA_GET_COUNT, &args![extra]).u16() as i16;
        e.mem.set_i32(entry + ACQUIRE_COUNT, i32::from(count));
        e.mem.set_u32(entry + ACQUIRE_STATE, 0);
        let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![reference]).u32();
        e.mem.set_u32(entry + ACQUIRE_EXTRA, extra);
        add_temp_object(e, process, entry);
    }
    let searched = acquire_searches_type(e.mem.u32(process + FORM_TYPE_FOR_LOCATION))
        && is_actor
        && reference != e.global::<u32>(PLAYER);
    if !searched && form_type != CONTAINER_LIKE_FORM_TYPE {
        return false;
    }
    let extra = e.call(REFERENCE_GET_EXTRA_LIST, &args![reference]).u32();
    let changes = e.call(EXTRA_GET_CONTAINER_CHANGES, &args![extra]).u32();
    if changes != 0 {
        let mut node = if e.call(CHANGES_LIST_004BFFB0, &args![changes]).u32() != 0 {
            let list = e.call(CHANGES_LIST_004BFFB0, &args![changes]).u32();
            e.call(NODE_PART_00717E50, &args![list]).u32()
        } else {
            0
        };
        while node != 0 {
            let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
            if e.mem.u32(item_address) == 0 {
                break;
            }
            let item_address = e.call(LIST_HEAD_ITEM, &args![node]).u32();
            let change = e.mem.u32(item_address);
            if change != 0 {
                let object = e.mem.u32(change + 4);
                let held = e
                    .call(CHANGES_GET_INVENTORY_ITEM, &args![changes, object, 0u32])
                    .u32();
                if held == 0 {
                    let object = e.mem.u32(change + 4);
                    if acquire_settings_match(e, process, object)
                        && e.call(REFERENCE_GET_SPACE, &args![reference]).u32()
                            == e.call(REFERENCE_GET_SPACE, &args![actor]).u32()
                    {
                        let object = e.mem.u32(change + 4);
                        let entry = new_inventory_entry(e, reference, object);
                        add_temp_object(e, process, entry);
                    }
                } else {
                    e.call(ITEM_DELETE, &args![held, 1u32]);
                }
            }
            node = e.call(LIST_NEXT_NODE, &args![node]).u32();
        }
    }
    let count = e
        .call(REFERENCE_ITEM_COUNT, &args![reference, 0u32, 1u32])
        .i32();
    let mut index = 0i32;
    while index < count {
        let item = e
            .call(REFERENCE_ITEM_AT, &args![reference, index, 0u32])
            .u32();
        if item != 0 {
            if !e.call(ITEM_GET_WORN, &args![item, 0u32]).bool() {
                let mut skip = false;
                if health_test != 0 {
                    let health = e.call(ITEM_GET_HEALTH, &args![item, 0u32]).f64();
                    let zero: f64 = e.global(ZERO);
                    if health <= zero && busy {
                        skip = true;
                    }
                }
                let matched = !skip && {
                    let object = e.call(WORD_08_0044DDC0, &args![item]).u32();
                    let wanted_type = e.mem.u32(process + FORM_TYPE_FOR_LOCATION);
                    (wanted_type != 0
                        && e.call(
                            FORM_MATCHES_PACKAGE_OBJECT_TYPE,
                            &args![object, wanted_type],
                        )
                        .bool())
                        || (e.mem.u32(process + OBJECT_TO_ACQUIRE) != 0
                            && e.mem.u32(process + OBJECT_TO_ACQUIRE)
                                == e.call(WORD_08_0044DDC0, &args![item]).u32())
                        || (e.mem.u32(process + FORM_TYPE_FOR_LOCATION) == 0
                            && e.mem.u32(process + OBJECT_TO_ACQUIRE) == 0)
                };
                if matched
                    && e.call(REFERENCE_GET_SPACE, &args![reference]).u32()
                        == e.call(REFERENCE_GET_SPACE, &args![actor]).u32()
                {
                    let object = e.call(WORD_08_0044DDC0, &args![item]).u32();
                    let entry = new_inventory_entry(e, reference, object);
                    add_temp_object(e, process, entry);
                    if e.call(WORD_AT_00559450, &args![item]).u32() != 0 {
                        let list = e.call(WORD_AT_00559450, &args![item]).u32();
                        let head = e.call(LIST_HEAD_ITEM, &args![list]).u32();
                        let first = e.mem.u32(head);
                        e.mem.set_u32(entry + ACQUIRE_EXTRA, first);
                    }
                }
            }
            e.call(ITEM_DELETE, &args![item, 1u32]);
        }
        index += 1;
    }
    false
}

/// The inventory changes' item count (`004c8fd0`, one stack word) and the item at an index
/// (`004c94f0`, one stack word), the weapon tests `006450c0` (`combatformulas.cpp`) and
/// `004c0c30` (`inventorychanges.cpp`) (unnamed; `thiscall`).
const CHANGES_ITEM_COUNT: u32 = 0x004c_8fd0;
const CHANGES_ITEM_AT: u32 = 0x004c_94f0;
const WEAPON_TEST_006450C0: u32 = 0x0064_50c0;
const WEAPON_TEST_004C0C30: u32 = 0x004c_0c30;
/// `__RTDynamicCast`'s type descriptors of `TESObjectARMO` (`01183a34`), `TESObjectWEAP`
/// (`01183998`), `TESAmmo` (`011840c4`) and `TESObjectCLOT` (`01186fcc`).
const ARMOR_DESCRIPTOR: u32 = 0x0118_3a34;
const WEAPON_DESCRIPTOR: u32 = 0x0118_3998;
const AMMO_DESCRIPTOR: u32 = 0x0118_40c4;
const CLOTHING_DESCRIPTOR: u32 = 0x0118_6fcc;
/// `"Package on %s is using an invalid Search Location"`.
const INVALID_SEARCH_LOCATION_FORMAT: u32 = 0x0108_8f90;

/// The item's object cast from `TESBoundObject` to `to`.
fn item_object_as(e: &mut Engine, item: u32, to: u32) -> u32 {
    let object = e.call(WORD_08_0044DDC0, &args![item]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![object, 0u32, BOUND_OBJECT_TYPE_DESCRIPTOR, to, 0u32],
    )
    .u32()
}

/// Walks the container's inventory changes: every item that `wanted` accepts becomes an
/// `ObjectstoAcquire` (its object, count `00726070`, the container, state 1) added to
/// `ObjectList` (`005ae3d0`); every walked item is deleted (`004459e0(1)`).
fn collect_container_items(
    e: &mut Engine,
    base: u32,
    container: u32,
    changes: u32,
    wanted: impl Fn(&mut Engine, u32) -> bool,
) {
    let count = e.call(CHANGES_ITEM_COUNT, &args![changes, 1u32]).i32();
    let mut index = 0i32;
    while index < count {
        let item = e.call(CHANGES_ITEM_AT, &args![changes, index]).u32();
        if item != 0 {
            if wanted(e, item) {
                let entry = new_acquire_entry(e);
                let object = e.call(WORD_08_0044DDC0, &args![item]).u32();
                e.mem.set_u32(entry + ACQUIRE_FORM, object);
                let item_count = e.call(LIST_NEXT_NODE, &args![item]).u32();
                e.mem.set_u32(entry + ACQUIRE_COUNT, item_count);
                e.mem.set_u32(entry + ACQUIRE_REFERENCE, container);
                e.mem.set_u32(entry + ACQUIRE_STATE, 1);
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), entry);
                    e.call(LIST_ADD_ITEM, &args![base + OBJECT_LIST, slot]);
                });
            }
            e.call(ITEM_DELETE, &args![item, 1u32]);
        }
        index += 1;
    }
}

// Translated from 00910e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00910e20 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the entry allocations) is not translated
/// `LowProcess::BuildMulitTargetObjectList` (engine map; `thiscall`, three stack words: the
/// actor, a container and an object type, `RET 0xc`). With a container that has inventory
/// changes (`HasContainer`, `GetInventoryChanges`), by the type (the exe's byte map `0091199c`
/// and jump table `00911984`): 0xc takes its weapons, 0x13 its armor, weapons and ammo, 0x14
/// those and clothing, 0x16 the weapons `006450c0` accepts, 0x17 those `004c0c30` accepts
/// ([`collect_container_items`]). Without one, `eFormType` becomes the type and the references
/// near the actor go through the acquire callback (`0090dd80`, by
/// `TESDataHandler::EnumReferencesCloseToPoint`): around the running package's search location
/// (its radius as a `float`, else `SEARCH_RADIUS_MAX` for an interior cell or the acquire
/// radius setting; a debug print for a search location without a cell) when the actor has a
/// cell and the current package one, else around the actor with the package's acquire radius
/// (`GetAcquireRadius`, else the setting). Then `eFormType`, `pObjecttoAcquire` and
/// `pFactiontoAcquire` are cleared and the list is ordered (slot `+0x818`).
pub fn low_process_build_mulit_target_object_list(
    e: &mut Engine,
    this: Ptr<LowProcess>,
    actor: u32,
    container: u32,
    kind: u32,
) {
    let base = this.addr();
    let has_container = if container != 0 {
        e.call(REFERENCE_HAS_CONTAINER, &args![container]).u32()
    } else {
        0
    };
    let changes = if has_container != 0 {
        e.call(GET_INVENTORY_CHANGES, &args![container]).u32()
    } else {
        0
    };
    if changes != 0 {
        match kind.wrapping_sub(0xc) {
            0 => collect_container_items(e, base, container, changes, |e, item| {
                item_object_as(e, item, WEAPON_DESCRIPTOR) != 0
            }),
            7 => collect_container_items(e, base, container, changes, |e, item| {
                let armor = item_object_as(e, item, ARMOR_DESCRIPTOR);
                let weapon = item_object_as(e, item, WEAPON_DESCRIPTOR);
                let ammo = item_object_as(e, item, AMMO_DESCRIPTOR);
                armor != 0 || weapon != 0 || ammo != 0
            }),
            8 => collect_container_items(e, base, container, changes, |e, item| {
                let armor = item_object_as(e, item, ARMOR_DESCRIPTOR);
                let weapon = item_object_as(e, item, WEAPON_DESCRIPTOR);
                let ammo = item_object_as(e, item, AMMO_DESCRIPTOR);
                let clothing = item_object_as(e, item, CLOTHING_DESCRIPTOR);
                armor != 0 || weapon != 0 || ammo != 0 || clothing != 0
            }),
            10 => collect_container_items(e, base, container, changes, |e, item| {
                let weapon = item_object_as(e, item, WEAPON_DESCRIPTOR);
                weapon != 0 && e.call(WEAPON_TEST_006450C0, &args![weapon]).bool()
            }),
            11 => collect_container_items(e, base, container, changes, |e, item| {
                let weapon = item_object_as(e, item, WEAPON_DESCRIPTOR);
                weapon != 0 && e.call(WEAPON_TEST_004C0C30, &args![weapon]).bool()
            }),
            _ => {}
        }
        return;
    }
    let mut cell = e.call(ACTOR_GET_008D6F30, &args![actor]).u32();
    e.with_stack(0x20, |e, locals| {
        let point = locals.addr();
        let temp = locals.addr() + 0xc;
        let radius_at = locals.addr() + 0x18;
        let position = e.vcall(actor, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
        for i in 0..3 {
            let word = e.mem.u32(position + 4 * i);
            e.mem.set_u32(point + 4 * i, word);
        }
        e.mem.set_u32(base + FORM_TYPE_FOR_LOCATION, kind);
        e.mem.set_u32(radius_at, x87_store32(0.0));
        let running = e.vcall(base, GET_PACKAGE_RUNNING_SLOT, &args![]).u32();
        let searches = cell != 0 && {
            let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
            e.call(PACKAGE_GET_SEARCH_LOCATION, &args![package]).u32() != 0
        };
        if searches {
            let search_radius = e
                .call(PACKAGE_GET_SEARCH_RADIUS, &args![running, actor])
                .u32();
            cell = e
                .call(PACKAGE_GET_SEARCH_CELL, &args![running, actor])
                .u32();
            let coord = e
                .call(PACKAGE_GET_SEARCH_COORD, &args![running, temp, actor])
                .u32();
            for i in 0..3 {
                let word = e.mem.u32(coord + 4 * i);
                e.mem.set_u32(point + 4 * i, word);
            }
            if cell != 0 {
                let radius = if search_radius > 0 {
                    x87_store32(f64::from(search_radius))
                } else if e.call(TEST_00425FD0, &args![cell]).bool() {
                    x87_quiet_bits(e.global::<u32>(SEARCH_RADIUS_MAX))
                } else {
                    let at = e
                        .call(SETTING_VALUE_00403E20, &args![ACQUIRE_RADIUS_SETTING])
                        .u32();
                    x87_quiet_bits(e.mem.u32(at))
                };
                e.mem.set_u32(radius_at, radius);
            } else {
                let name = e.call(REFERENCE_GET_NAME, &args![actor]).u32();
                e.call(
                    SAVE_DEBUG_PRINT,
                    &args![INVALID_SEARCH_LOCATION_FORMAT, name],
                );
            }
            let radius = x87_quiet_bits(e.mem.u32(radius_at));
            let handler = e.global::<u32>(DATA_HANDLER);
            e.call(
                ENUM_REFERENCES_CLOSE_TO_POINT,
                &args![
                    handler,
                    cell,
                    point,
                    radius,
                    point,
                    radius,
                    COLLECT_TARGET_CALLBACK,
                    actor
                ],
            );
        } else {
            let has_radius = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32() != 0 && {
                let package = e.vcall(base, GET_CURRENT_PACKAGE_SLOT, &args![]).u32();
                e.call(
                    PACKAGE_GET_ACQUIRE_RADIUS,
                    &args![package, actor, radius_at],
                )
                .bool()
            };
            if !has_radius {
                let at = e
                    .call(SETTING_VALUE_00403E20, &args![ACQUIRE_RADIUS_SETTING])
                    .u32();
                let value = x87_quiet_bits(e.mem.u32(at));
                e.mem.set_u32(radius_at, value);
            }
            let radius = x87_quiet_bits(e.mem.u32(radius_at));
            let position = e.vcall(actor, ACTOR_GET_LOCATION_SLOT, &args![]).u32();
            let handler = e.global::<u32>(DATA_HANDLER);
            e.call(
                ENUM_REFERENCES_CLOSE_TO_POINT,
                &args![
                    handler,
                    cell,
                    point,
                    radius,
                    position,
                    radius,
                    COLLECT_TARGET_CALLBACK,
                    actor
                ],
            );
        }
    });
    e.mem.set_u32(base + FORM_TYPE_FOR_LOCATION, 0);
    e.mem.set_u32(base + OBJECT_TO_ACQUIRE, 0);
    e.mem.set_u32(base + FACTION_TO_ACQUIRE, 0);
    e.vcall(base, EVALUATE_ORDER_ACQUIRE_LIST_SLOT, &args![actor]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0090_9430,
            low_process_process_escort(Ptr<LowProcess>, u32)
        ),
        entry!(0x0090_dd80, fn_0090dd80(u32, u32) -> bool),
        entry!(
            0x0091_0e20,
            low_process_build_mulit_target_object_list(Ptr<LowProcess>, u32, u32, u32)
        ),
        entry!(
            0x0091_1b00,
            low_process_update_followers(Ptr<LowProcess>, u32, f32)
        ),
        entry!(
            0x0091_2db0,
            low_process_process_guard_start_travel_to_location(Ptr<LowProcess>, u32, u32)
        ),
        entry!(0x0090_e660, fn_0090e660(u32, u32) -> bool),
        entry!(
            0x0090_e970,
            low_process_get_current_destination_reference(Ptr<LowProcess>, u32, u8) -> u32
        ),
        entry!(
            0x0090_f670,
            low_process_get_current_destination_radius(Ptr<LowProcess>, u32) -> f32
        ),
        entry!(
            0x0090_7ab0,
            low_process_should_update_combat_controller_or_end_combat(
                Ptr<LowProcess>,
                u32,
                u32,
            ) -> bool
        ),
        entry!(
            0x0090_7d90,
            low_process_evaluate_order_acquire_list(Ptr<LowProcess>, u32)
        ),
        entry!(
            0x0090_f8b0,
            low_process_get_save_size(Ptr<LowProcess>, u32, u32) -> u16
        ),
        entry!(
            0x0090_fa70,
            low_process_save_game(Ptr<LowProcess>, u32, u32)
        ),
        entry!(
            0x0090_fd80,
            low_process_load_game(Ptr<LowProcess>, u32, u32, u32)
        ),
        entry!(0x0091_0630, low_process_load_game_ov2(Ptr<LowProcess>, u32)),
        entry!(
            0x0091_0820,
            low_process_init_load_game_ov2(Ptr<LowProcess>, u32)
        ),
        entry!(0x0091_1ae0, fn_00911ae0(Ptr<LowProcess>, u32, u32) -> f32),
        entry!(
            0x0091_1ed0,
            low_process_process_summon_creature_defend(Ptr<LowProcess>, u32)
        ),
        entry!(
            0x0091_2150,
            low_process_process_movement_blocked(Ptr<LowProcess>, u32)
        ),
        entry!(
            0x0091_3460,
            low_process_is_package_done(Ptr<LowProcess>, u32) -> bool
        ),
        entry!(
            0x0091_34c0,
            low_process_mod_tracked_damage(Ptr<LowProcess>, u32, f32)
        ),
        entry!(0x0091_37e0, fn_009137e0(Ptr, u32) -> Ptr),
        entry!(0x0091_3950, fn_00913950(Ptr, u32) -> Ptr),
        entry!(0x0091_3910, fn_00913910(Ptr, u32)),
        entry!(0x0091_3980, fn_00913980(Ptr, u32) -> Ptr),
        entry!(0x0091_0a60, low_process_revert_ov2(Ptr<LowProcess>, u32)),
        entry!(
            0x0091_0250,
            low_process_init_load_game(Ptr<LowProcess>, u32, u32, u32)
        ),
        entry!(
            0x0090_8cd0,
            low_process_create_followfor_escort(Ptr<LowProcess>, u32, u32, u32)
        ),
        entry!(
            0x0090_9320,
            low_process_process_remove_worn(Ptr<LowProcess>, u32, u8)
        ),
        entry!(
            0x0090_ac40,
            low_process_check_if_has_object(Ptr<LowProcess>, u32, u32) -> bool
        ),
        entry!(
            0x0090_d000,
            low_process_process_combat(Ptr<LowProcess>, u32)
        ),
        entry!(0x0090_d240, low_process_process_alarm(Ptr<LowProcess>, u32)),
        entry!(
            0x0090_dc60,
            low_process_process_pursue(Ptr<LowProcess>, u32, u32, u32, u32)
        ),
        entry!(0x0091_0310, low_process_revert(Ptr<LowProcess>, u32, u32)),
        entry!(
            0x0090_7460,
            low_process_damage_mod_actor_value(Ptr<LowProcess>, u32, u32, f32)
        ),
        entry!(
            0x0090_7620,
            low_process_mod_essential_down_timer(Ptr<LowProcess>)
        ),
        entry!(
            0x0090_7960,
            low_process_should_stop_combat_controller_to_finish_package(
                Ptr<LowProcess>,
                u32,
                u32,
            ) -> bool
        ),
        entry!(
            0x0090_79f0,
            low_process_update_package_target_for_combat(Ptr<LowProcess>, u32, u32) -> bool
        ),
        entry!(
            0x0090_8f40,
            low_process_process_avoid_area(Ptr<LowProcess>, u32)
        ),
        entry!(0x0090_9c90, low_process_process_flee(Ptr<LowProcess>, u32)),
        entry!(
            0x0090_a130,
            low_process_get_current_package_location(Ptr<LowProcess>, u32, u32) -> u32
        ),
        entry!(
            0x0090_cfa0,
            low_process_is_wandering(Ptr<LowProcess>) -> bool
        ),
        entry!(
            0x0090_dc10,
            low_process_enter_combat(
                Ptr<LowProcess>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u8,
                u32,
                u8,
                u32,
                u8,
                u32,
                u8,
                u32,
            ) -> bool
        ),
        entry!(0x0090_e600, fn_0090e600(Ptr) -> Ptr),
        entry!(0x0090_70a0, fn_009070a0(Ptr<LowProcess>)),
        entry!(0x0090_6fe0, fn_00906fe0(Ptr<LowProcess>)),
        entry!(0x0090_6fb0, low_process_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0090_7510, fn_00907510(Ptr<LowProcess>, Ptr<LowProcess>)),
        entry!(0x0091_37b0, fn_009137b0(Ptr, u32) -> Ptr),
        entry!(
            0x0090_76a0,
            low_process_get_processed_day(Ptr<LowProcess>) -> u32
        ),
        entry!(
            0x0090_76c0,
            low_process_get_processed_month(Ptr<LowProcess>) -> u32
        ),
        entry!(
            0x0090_76e0,
            low_process_get_processed_year(Ptr<LowProcess>) -> u32
        ),
        entry!(0x0090_ac20, fn_0090ac20(Ptr<LowProcess>, u8)),
        entry!(
            0x0090_74f0,
            low_process_get_damage_modifier(Ptr<LowProcess>, u8) -> f64
        ),
        entry!(0x0091_3810, fn_00913810(Ptr, u32) -> Ptr),
        entry!(0x0091_3880, fn_00913880(Ptr)),
        entry!(0x0091_38e0, fn_009138e0(Ptr)),
        entry!(
            0x0091_2960,
            low_process_process_patrol_start_at_ref_behavior(Ptr<LowProcess>, u32, Ptr)
        ),
        entry!(
            0x0090_7650,
            low_process_compute_last_time_processed(Ptr<LowProcess>)
        ),
        entry!(0x0091_1e80, low_process_set_target(Ptr<LowProcess>, Ptr)),
        entry!(
            0x0091_22e0,
            low_process_process_patrol_init(Ptr<LowProcess>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x0090_8bf0,
            low_process_set_target_for_package(Ptr<LowProcess>, Ptr) -> bool
        ),
        entry!(0x0090_8c50, fn_00908c50(Ptr<LowProcess>, Ptr, u8) -> u32),
        entry!(
            0x0090_7260,
            low_process_get_time_delta(Ptr<LowProcess>, u32) -> f32
        ),
        entry!(
            0x0090_78d0,
            low_process_should_use_combat_controller_for_package(Ptr<LowProcess>, Ptr, u8) -> bool
        ),
        entry!(
            0x0090_c360,
            low_process_update_ov2(Ptr<LowProcess>, Ptr, f32)
        ),
        entry!(
            0x0090_72d0,
            low_process_get_actor_value(Ptr<LowProcess>, u32, u32, Ptr) -> i32
        ),
        entry!(
            0x0090_7380,
            low_process_get_actor_float_value(Ptr<LowProcess>, u32, u32, Ptr) -> f64
        ),
        entry!(
            0x0090_7840,
            low_process_check_and_start_combat_controller_for_package(
                Ptr<LowProcess>,
                Ptr,
                Ptr,
                u8,
            ) -> bool
        ),
        entry!(
            0x0091_3510,
            low_process_start_new_package(Ptr<LowProcess>, Ptr, Ptr)
        ),
        entry!(
            0x0091_35a0,
            low_process_change_package(Ptr<LowProcess>, Ptr, Ptr)
        ),
        entry!(
            0x0091_3630,
            low_process_package_done(Ptr<LowProcess>, Ptr, Ptr)
        ),
        entry!(
            0x0090_8e90,
            low_process_process_use_item_at(Ptr<LowProcess>, Ptr) -> bool
        ),
        entry!(0x0090_a070, low_process_reset_target(Ptr<LowProcess>)),
        entry!(
            0x0090_9d00,
            low_process_process_flee_non_combat(Ptr<LowProcess>, Ptr)
        ),
        entry!(
            0x0091_36b0,
            low_process_process_greet(Ptr<LowProcess>, Ptr, Ptr, u32, u32, u32, u8)
        ),
        entry!(
            0x0091_19b0,
            low_process_add_to_procedure_index_running(Ptr<LowProcess>, Ptr, u32)
        ),
        entry!(
            0x0090_7700,
            low_process_set_current_package_from_schedule(Ptr<LowProcess>, Ptr) -> bool
        ),
        entry!(
            0x0091_0b30,
            low_process_clear_current_data_for_process(Ptr<LowProcess>, u32)
        ),
        entry!(0x0090_d570, fn_0090d570(Ptr, Ptr) -> bool),
        entry!(0x0090_d480, fn_0090d480(Ptr, Ptr) -> bool),
        entry!(
            0x0091_21e0,
            low_process_process_patrol(Ptr<LowProcess>, Ptr)
        ),
        entry!(
            0x0091_2990,
            low_process_process_patrol_at_ref(Ptr<LowProcess>, Ptr, Ptr, f32) -> f32
        ),
        entry!(
            0x0091_0c80,
            low_process_process_ambush_wait(Ptr<LowProcess>, Ptr)
        ),
        entry!(
            0x0090_cc10,
            low_process_clear_object_from_acquire_list(Ptr<LowProcess>, u32)
        ),
        entry!(
            0x0090_cdc0,
            low_process_clear_object_from_acquire_list_ov2(Ptr<LowProcess>, u32)
        ),
        entry!(
            0x0091_2340,
            low_process_process_patrol_start_travel_to_next_ref(Ptr<LowProcess>, Ptr, Ptr)
        ),
        entry!(
            0x0091_25b0,
            fn_009125b0(Ptr<LowProcess>, Ptr, Ptr, f32) -> f32
        ),
        entry!(
            0x0091_30f0,
            low_process_save_package_to_extra_data(Ptr<LowProcess>, Ptr)
        ),
        entry!(
            0x0091_3250,
            low_process_load_package_from_extra_data(Ptr<LowProcess>, Ptr)
        ),
        entry!(0x0091_0450, low_process_save_game_ov2(Ptr<LowProcess>, Ptr)),
        entry!(
            0x0090_9de0,
            low_process_process_follow(Ptr<LowProcess>, Ptr, u32, u32, u32)
        ),
        entry!(
            0x0091_1f00,
            low_process_process_sandbox(Ptr<LowProcess>, Ptr)
        ),
        entry!(0x0091_2b00, low_process_process_guard(Ptr<LowProcess>, Ptr)),
        entry!(
            0x0090_89c0,
            low_process_get_location_for_package(Ptr<LowProcess>, Ptr, Ptr, u8) -> u32
        ),
        entry!(
            0x0090_f0e0,
            low_process_get_current_destination_cell(Ptr<LowProcess>, Ptr) -> u32
        ),
        entry!(
            0x0090_f3f0,
            low_process_get_current_destination_world_space(Ptr<LowProcess>, Ptr) -> u32
        ),
        entry!(
            0x0090_ec70,
            low_process_get_current_destination_coordinate(Ptr<LowProcess>, Ptr, Ptr, u8) -> Ptr
        ),
        entry!(
            0x0090_8f70,
            low_process_process_acquire(Ptr<LowProcess>, Ptr)
        ),
        entry!(
            0x0090_d620,
            low_process_process_activate(Ptr<LowProcess>, Ptr, u8) -> bool
        ),
        entry!(
            0x0090_82d0,
            low_process_get_target_for_package(Ptr<LowProcess>, Ptr, Ptr) -> u32
        ),
        entry!(
            0x0090_c3e0,
            low_process_update_missed_packages(Ptr<LowProcess>, Ptr, u8)
        ),
        entry!(
            0x0090_a1a0,
            low_process_check_for_new_package(Ptr<LowProcess>, Ptr, u8) -> bool
        ),
        entry!(0x0090_ad40, low_process_update(Ptr<LowProcess>, Ptr)),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn runs_the_base_constructor_then_stores_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR_00913810, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0091_37b0, &args![this, 0x27u32]).u32(), this);
        assert_eq!(
            calls_to(&mut e, BASE_CONSTRUCTOR_00913810),
            vec![vec![this, 0x27]]
        );
        assert_eq!(e.mem.u32(this), VTABLE_009137B0);
    }
}

#[cfg(test)]
mod low_process_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_bit_zero_is_set() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_BODY, |_, _| Ret::default());
        e.register(FREE_OBJECT, |_, _| Ret::default());
        let this = e.mem.alloc(0xb4);
        for (flags, frees) in [(0u32, 0usize), (1, 1), (2, 0), (3, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0090_6fb0, &args![this, flags]).u32(), this);
            assert_eq!(calls_to(&e, DESTRUCTOR_BODY), vec![vec![this]]);
            assert_eq!(calls_to(&e, FREE_OBJECT).len(), frees);
            if frees == 1 {
                assert_eq!(calls_to(&e, FREE_OBJECT), vec![vec![this]]);
            }
        }
    }

    const SET_TARGET: u32 = 0x00ee_0001;

    /// A source and a target process with doubles for every callee outside this file.
    fn copy_fixture() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.register(PROCESS_GET_HOUR, |_, _| 7.5f32.into_ret());
        e.register(PROCESS_SET_HOUR, |_, _| Ret::default());
        e.register(ACTOR_PACKAGE_MOVE_TO, |_, _| Ret::default());
        e.register(PROCESS_GET_DATE, |_, _| 0x0000_0abcu32.into_ret());
        e.register(MODIFIER_LIST_COPY, |_, _| Ret::default());
        e.register(SET_TARGET, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x12c / 4 + 1];
        slots[0x12c / 4] = SET_TARGET;
        e.put_vtable(0x0300_0000, &slots);
        let to = e.mem.alloc(0xb4);
        let from = e.mem.alloc(0xb4);
        e.mem.set_u32(to, 0x0300_0000);
        e.mem.set_u32(from, 0x0300_0000);
        (e, to, from)
    }

    #[test]
    fn copy_takes_the_state_of_the_other_process() {
        let (mut e, to, from) = copy_fixture();
        e.mem.set_u8(from + 0x30, 0x5a);
        e.mem.set_u32(from + 0x34, 0x1111);
        e.mem.set_u32(from + 0x38, 0x2222);
        e.mem.set_u32(from + 0x3c, 0x3333);
        e.mem.set_u32(from + 0x40, 0x4444);
        e.mem.set_u32(from + 0x58, 9);
        e.mem.set_u32(from + 0x64, 0x6464);
        e.mem.set_u32(from + 0x68, 0x6868);
        e.mem.set_u32(from + 0xa4, 1.5f32.to_bits());
        e.mem.set_u32(from + 0xa8, 2.5f32.to_bits());
        e.mem.set_u32(from + 0xac, 3.5f32.to_bits());
        e.mem.set_u32(to + 0x64, 0x9999);
        e.call_log = Some(vec![]);
        e.call(0x0090_7510, &args![to, from]);
        assert_eq!(calls_to(&e, PROCESS_GET_HOUR), vec![vec![from]]);
        assert_eq!(
            calls_to(&e, PROCESS_SET_HOUR),
            vec![vec![to, 7.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, ACTOR_PACKAGE_MOVE_TO),
            vec![vec![from + 4, to + 4]]
        );
        assert_eq!(calls_to(&e, PROCESS_GET_DATE), vec![vec![from]]);
        assert_eq!(calls_to(&e, SET_TARGET), vec![vec![to, 0x4444]]);
        assert_eq!(
            calls_to(&e, MODIFIER_LIST_COPY),
            vec![vec![to + 0x94, from + 0x94]]
        );
        assert_eq!(e.mem.u8(to + 0x30), 0x5a);
        assert_eq!(e.mem.u32(to + 0x24), 0xabc);
        assert_eq!(e.mem.u32(to + 0x34), 0x1111);
        assert_eq!(e.mem.u32(to + 0x38), 0x2222);
        assert_eq!(e.mem.u32(to + 0x3c), 0x3333);
        assert_eq!(e.mem.u32(to + 0x58), 9);
        assert_eq!(e.mem.u32(to + 0x64), 0x6464);
        assert_eq!(e.mem.u32(to + 0x68), 0x6868);
        assert_eq!(e.mem.u32(from + 0x64), 0);
        assert_eq!(e.mem.u32(from + 0x68), 0);
        assert_eq!(f32::from_bits(e.mem.u32(to + 0xa4)), 1.5);
        assert_eq!(f32::from_bits(e.mem.u32(to + 0xa8)), 2.5);
        assert_eq!(f32::from_bits(e.mem.u32(to + 0xac)), 3.5);
    }

    #[test]
    fn copy_keeps_the_saved_acquire_object_when_the_other_has_none() {
        let (mut e, to, from) = copy_fixture();
        e.mem.set_u32(to + 0x68, 0x7777);
        e.mem.set_u32(from + 0x64, 0);
        e.call(0x0090_7510, &args![to, from]);
        assert_eq!(e.mem.u32(to + 0x68), 0x7777);
        assert_eq!(e.mem.u32(to + 0x64), 0);
    }

    #[test]
    fn copy_quiets_signalling_nans_in_the_three_floats() {
        let (mut e, to, from) = copy_fixture();
        e.mem.set_u32(from + 0xa4, 0x7fa0_0001);
        e.mem.set_u32(from + 0xa8, 0xffa0_0002);
        e.mem.set_u32(from + 0xac, 0x7fc0_0003);
        e.call(0x0090_7510, &args![to, from]);
        assert_eq!(e.mem.u32(to + 0xa4), 0x7fe0_0001);
        assert_eq!(e.mem.u32(to + 0xa8), 0xffe0_0002);
        assert_eq!(e.mem.u32(to + 0xac), 0x7fc0_0003);
    }
}

#[cfg(test)]
mod destructor_body_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    #[test]
    fn stores_the_vtable_and_runs_the_destructors_in_order() {
        let mut e = Engine::new();
        let log: Log = Log::default();
        for address in [
            CLEAR_CURRENT_PACKAGE,
            LOW_PROCESS_RELEASE,
            MODIFIER_LIST_DESTRUCT,
            LIST_DESTRUCT,
            BASE_PROCESS_DESTRUCT,
        ] {
            let log = log.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let this = e.mem.alloc(0xb4);
        e.mem.set_u32(this, 0x1234_5678);
        e.call(0x0090_6fe0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0108_86e4);
        assert_eq!(
            *log.borrow(),
            [
                (CLEAR_CURRENT_PACKAGE, vec![this]),
                (LOW_PROCESS_RELEASE, vec![this]),
                (MODIFIER_LIST_DESTRUCT, vec![this + 0x94]),
                (LIST_DESTRUCT, vec![this + 0x7c]),
                (LIST_DESTRUCT, vec![this + 0x74]),
                (LIST_DESTRUCT, vec![this + 0x6c]),
                (LIST_DESTRUCT, vec![this + 0x5c]),
                (BASE_PROCESS_DESTRUCT, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod release_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const OBJECT_DELETE: u32 = 0x00ee_0010;

    /// Doubles for every callee outside this file. A list is modelled as its two head words:
    /// empty when both are zero, removal shifts the second word into the first.
    fn fixture() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Log::default();
        let logged = |log: &Log, address: u32, words: &[u32]| {
            log.borrow_mut().push((address, words.to_vec()));
        };
        let l = log.clone();
        e.register_double(DELETE_EMPTY_DESTRUCTOR, move |_, a| {
            logged(&l, DELETE_EMPTY_DESTRUCTOR, a);
            a[0].into_ret()
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_HEAD_ITEM, |_, a| a[0].into_ret());
        let l = log.clone();
        e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            logged(&l, LIST_REMOVE_ITEM, &[a[0], e.mem.u32(a[1])]);
            let next = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0], next);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        let l = log.clone();
        e.register_double(LIST_CLEAR, move |_, a| {
            logged(&l, LIST_CLEAR, a);
            Ret::default()
        });
        let l = log.clone();
        e.register_double(OBJECT_DELETE, move |_, a| {
            logged(&l, OBJECT_DELETE, a);
            Ret::default()
        });
        let mut slots = vec![0u32; 5];
        slots[4] = OBJECT_DELETE;
        e.put_vtable(0x0300_0000, &slots);
        let this = e.mem.alloc(0xb4);
        (e, log, this)
    }

    #[test]
    fn frees_the_shared_acquire_object_once_and_empties_the_lists() {
        let (mut e, log, this) = fixture();
        e.mem.set_u32(this + 0x44, 0x4444);
        e.mem.set_u32(this + 0x64, 0x6400);
        e.mem.set_u32(this + 0x68, 0x6400);
        // ObjectList: a null item first, then 0x5c01.
        e.mem.set_u32(this + 0x5c, 0);
        e.mem.set_u32(this + 0x60, 0x5c01);
        // TempObjectList: two items.
        e.mem.set_u32(this + 0x74, 0x7401);
        e.mem.set_u32(this + 0x78, 0x7402);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0300_0000);
        e.mem.set_u32(this + 0x84, object);
        e.call(0x0090_70a0, &args![this]);
        assert_eq!(
            *log.borrow(),
            [
                (DELETE_EMPTY_DESTRUCTOR, vec![0x6400, 1]),
                (LIST_REMOVE_ITEM, vec![this + 0x5c, 0]),
                (LIST_REMOVE_ITEM, vec![this + 0x5c, 0x5c01]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x5c01, 1]),
                (LIST_CLEAR, vec![this + 0x6c]),
                (LIST_REMOVE_ITEM, vec![this + 0x74, 0x7401]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x7401, 1]),
                (LIST_REMOVE_ITEM, vec![this + 0x74, 0x7402]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x7402, 1]),
                (LIST_CLEAR, vec![this + 0x7c]),
                (OBJECT_DELETE, vec![object, 1]),
            ]
        );
        for offset in [0x44, 0x64, 0x68, 0x84] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
    }

    #[test]
    fn frees_both_acquire_objects_when_they_differ() {
        let (mut e, log, this) = fixture();
        e.mem.set_u32(this + 0x64, 0x6400);
        e.mem.set_u32(this + 0x68, 0x6800);
        e.call(0x0090_70a0, &args![this]);
        assert_eq!(
            *log.borrow(),
            [
                (DELETE_EMPTY_DESTRUCTOR, vec![0x6400, 1]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x6800, 1]),
                (LIST_CLEAR, vec![this + 0x6c]),
                (LIST_CLEAR, vec![this + 0x7c]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.u32(this + 0x68), 0);
    }

    #[test]
    fn frees_only_the_saved_acquire_object_when_the_current_is_null() {
        let (mut e, log, this) = fixture();
        e.mem.set_u32(this + 0x68, 0x6800);
        e.call(0x0090_70a0, &args![this]);
        assert_eq!(
            log.borrow()
                .iter()
                .filter(|(a, _)| *a == DELETE_EMPTY_DESTRUCTOR)
                .cloned()
                .collect::<Vec<_>>(),
            [(DELETE_EMPTY_DESTRUCTOR, vec![0x6800, 1])]
        );
        assert_eq!(e.mem.u32(this + 0x68), 0);
    }
}

#[cfg(test)]
mod b003_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub(super) type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    pub(super) const PROCESS_VTABLE: u32 = 0x0300_0000;
    pub(super) const ACTOR_VTABLE: u32 = 0x0301_0000;
    pub(super) const PACKAGE_VTABLE: u32 = 0x0302_0000;
    pub(super) const OBJECT_VTABLE: u32 = 0x0303_0000;

    /// The targets the test vtables give their slot `offset`.
    pub(super) fn process_slot(offset: u32) -> u32 {
        0x00e0_0000 + offset
    }
    pub(super) fn actor_slot(offset: u32) -> u32 {
        0x00e1_0000 + offset
    }
    pub(super) fn package_slot(offset: u32) -> u32 {
        0x00e2_0000 + offset
    }
    pub(super) fn object_slot(offset: u32) -> u32 {
        0x00e3_0000 + offset
    }

    pub(super) fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    pub(super) fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// A process, an actor and a package (each with its own test vtable);
    /// every double registered through `on`/`answer` logs its words.
    pub(super) struct World {
        pub e: Engine,
        pub log: Log,
        pub process: u32,
        pub actor: u32,
        pub package: u32,
    }

    pub(super) fn world() -> World {
        let mut e = Engine::new();
        for (vtable, slot) in [
            (PROCESS_VTABLE, process_slot as fn(u32) -> u32),
            (ACTOR_VTABLE, actor_slot),
            (PACKAGE_VTABLE, package_slot),
            (OBJECT_VTABLE, object_slot),
        ] {
            let slots: Vec<u32> = (0..0x220).map(|i| slot(i * 4)).collect();
            e.put_vtable(vtable, &slots);
        }
        let make = |e: &mut Engine, vtable: u32, size: u32| {
            let p = e.mem.alloc(size);
            e.mem.set_u32(p, vtable);
            p
        };
        let process = make(&mut e, PROCESS_VTABLE, 0xb4);
        let actor = make(&mut e, ACTOR_VTABLE, 0x200);
        let package = make(&mut e, PACKAGE_VTABLE, 0x80);
        World {
            e,
            log: Log::default(),
            process,
            actor,
            package,
        }
    }

    impl World {
        /// A logged double at `address` answering `answer`.
        pub fn answer(&mut self, address: u32, answer: Ret) {
            let log = self.log.clone();
            self.e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                answer
            });
        }
        pub fn on(&mut self, address: u32, eax: u32) {
            self.answer(address, ret(eax));
        }
        /// A logged double answering `answers` in turn (the last one again
        /// once they run out).
        pub fn on_seq(&mut self, address: u32, answers: Vec<u32>) {
            let log = self.log.clone();
            let mut next = 0usize;
            self.e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let value = answers[next.min(answers.len() - 1)];
                next += 1;
                ret(value)
            });
        }
        /// A zeroed object with the test object vtable.
        pub fn object(&mut self) -> u32 {
            let p = self.e.mem.alloc(0x40);
            self.e.mem.set_u32(p, OBJECT_VTABLE);
            p
        }
        pub fn calls(&self) -> Vec<(u32, Vec<u32>)> {
            self.log.borrow().clone()
        }
        pub fn addresses(&self) -> Vec<u32> {
            self.log.borrow().iter().map(|(a, _)| *a).collect()
        }
        pub fn calls_to(&self, address: u32) -> Vec<Vec<u32>> {
            self.log
                .borrow()
                .iter()
                .filter(|(a, _)| *a == address)
                .map(|(_, words)| words.clone())
                .collect()
        }
        pub fn clear(&self) {
            self.log.borrow_mut().clear();
        }
    }

    #[test]
    fn processed_date_fields_unpack_the_packed_date() {
        let mut w = world();
        let packed = (5u32 << 13) | (11 << 9) | 0x1ab;
        w.e.mem.set_u32(w.process + 0x24, packed | 0xf800_0000);
        assert_eq!(w.e.call(0x0090_76a0, &args![w.process]).u32(), 0x1ab);
        assert_eq!(w.e.call(0x0090_76c0, &args![w.process]).u32(), 11);
        assert_eq!(w.e.call(0x0090_76e0, &args![w.process]).u32(), 5);
    }

    #[test]
    fn flag_0x40_and_damage_modifier_forward_their_words() {
        let mut w = world();
        w.on(SET_FLAG_BITS, 0);
        w.answer(MODIFIER_LIST_GET_MODIFIER, st0(2.5));
        w.e.call(0x0090_ac20, &args![w.process, 0x0000_0101u32]);
        let value =
            w.e.call(0x0090_74f0, &args![w.process, 0x0000_0123u32])
                .f64();
        assert_eq!(value, 2.5);
        assert_eq!(
            w.calls(),
            [
                (SET_FLAG_BITS, vec![w.process, 1, 0x40]),
                (MODIFIER_LIST_GET_MODIFIER, vec![w.process + 0x94, 0x23]),
            ]
        );
    }

    #[test]
    fn lock_map_constructor_and_destructors() {
        let mut w = world();
        w.on(NI_ALLOCATE, 0x7000);
        w.on(MEMORY_SET, 0);
        w.on(MAP_REMOVE_ALL, 0);
        w.on(NI_FREE, 0);
        let map = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(map + 0xc, 9);
        assert_eq!(w.e.call(0x0091_3810, &args![map, 0x25u32]).u32(), map);
        assert_eq!(w.e.mem.u32(map), LOCK_MAP_BASE_VTABLE);
        assert_eq!(w.e.mem.u32(map + 4), 0x25);
        assert_eq!(w.e.mem.u32(map + 8), 0x7000);
        assert_eq!(w.e.mem.u32(map + 0xc), 0);
        assert_eq!(
            w.calls(),
            [
                (NI_ALLOCATE, vec![0x94]),
                (MEMORY_SET, vec![0x7000, 0, 0x94]),
            ]
        );
        w.clear();
        w.e.call(0x0091_3880, &args![map]);
        assert_eq!(w.e.mem.u32(map), LOCK_MAP_BASE_VTABLE);
        assert_eq!(
            w.calls(),
            [
                (MAP_REMOVE_ALL, vec![map]),
                (MAP_REMOVE_ALL, vec![map]),
                (NI_FREE, vec![0x7000]),
            ]
        );
    }

    #[test]
    fn patrol_start_at_ref_behavior_sets_the_waiting_state() {
        let mut w = world();
        w.on(process_slot(SETUP_NEW_PACKAGE_SLOT), 0);
        let data = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(data + 8, 7.0f32.to_bits());
        w.e.call(0x0091_2960, &args![w.process, w.actor, data]);
        assert_eq!(
            w.calls(),
            [(process_slot(SETUP_NEW_PACKAGE_SLOT), vec![w.process])]
        );
        assert_eq!(w.e.mem.u32(data + 8), 0);
        assert_eq!(w.e.mem.u32(data + 4), 2);
    }

    #[test]
    fn compute_last_time_processed_packs_the_calendar() {
        let mut w = world();
        w.on(PROCESS_SET_HOUR, 0);
        w.on(STORE_PACKED_DATE, 0);
        w.e.map(CURRENT_GAME_HOUR, 0x10);
        w.e.map(CURRENT_GAME_MONTH, 4);
        w.e.mem.set_u32(CURRENT_GAME_HOUR, 0x7fa0_0000);
        w.e.mem.set_u32(CURRENT_GAME_YEAR, 281);
        w.e.mem.set_u8(CURRENT_GAME_DAY, 17);
        w.e.mem.set_u32(CURRENT_GAME_MONTH, 9);
        w.e.call(0x0090_7650, &args![w.process]);
        let packed = (9 << 9) | 17 | (281 << 13);
        assert_eq!(
            w.calls(),
            [
                (PROCESS_SET_HOUR, vec![w.process, 0x7fe0_0000]),
                (STORE_PACKED_DATE, vec![w.process, packed]),
            ]
        );
        // The day byte is sign-extended.
        w.clear();
        w.e.mem.set_u8(CURRENT_GAME_DAY, 0x80);
        w.e.call(0x0090_7650, &args![w.process]);
        assert_eq!(w.calls_to(STORE_PACKED_DATE)[0][1], 0xffff_ff80);
    }

    #[test]
    fn set_target_marks_stores_and_hands_it_to_the_package() {
        let mut w = world();
        w.on(REFERENCE_SET_TARGETED, 0);
        w.on(process_slot(GET_ACTOR_PACKAGE_RUNNING_SLOT), 0x5100);
        w.on(ACTOR_PACKAGE_SET_TARGET, 0);
        w.e.call(0x0091_1e80, &args![w.process, 0x6000u32]);
        assert_eq!(w.e.mem.u32(w.process + 0x40), 0x6000);
        assert_eq!(
            w.calls(),
            [
                (REFERENCE_SET_TARGETED, vec![0x6000, 1]),
                (
                    process_slot(GET_ACTOR_PACKAGE_RUNNING_SLOT),
                    vec![w.process]
                ),
                (ACTOR_PACKAGE_SET_TARGET, vec![0x5100, 0x6000]),
            ]
        );
        w.clear();
        w.e.call(0x0091_1e80, &args![w.process, 0u32]);
        assert_eq!(w.calls_to(REFERENCE_SET_TARGETED).len(), 0);
        assert_eq!(w.calls_to(ACTOR_PACKAGE_SET_TARGET), [vec![0x5100, 0]]);
    }

    #[test]
    fn patrol_init_fails_or_starts_the_travel() {
        let mut w = world();
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(PATROL_DATA_INIT, 0x100);
        w.on(process_slot(SET_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        let data = w.e.mem.alloc(0x10);
        assert!(!w
            .e
            .call(0x0091_22e0, &args![w.process, w.actor, data])
            .bool());
        assert_eq!(
            w.calls_to(PATROL_DATA_INIT),
            [vec![data, w.actor, w.package]]
        );
        assert_eq!(
            w.calls_to(process_slot(SET_PROCEDURE_INDEX_RUNNING_SLOT)),
            [vec![w.process, u32::MAX]]
        );
        // Success: the travel starts; without a current reference it sets -1.
        w.on(PATROL_DATA_INIT, 1);
        w.on(PATROL_GET_CURRENT_REFERENCE, 0);
        w.clear();
        assert!(w
            .e
            .call(0x0091_22e0, &args![w.process, w.actor, data])
            .bool());
        assert_eq!(w.calls_to(PATROL_GET_CURRENT_REFERENCE), [vec![data]]);
    }

    #[test]
    fn set_target_for_package_uses_the_package_target() {
        let mut w = world();
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(process_slot(GET_TARGET_FOR_PACKAGE_SLOT), 0x6000);
        let process = w.process;
        let log = w.log.clone();
        w.e.register_double(process_slot(SET_TARGET_SLOT), move |e, a| {
            log.borrow_mut()
                .push((process_slot(SET_TARGET_SLOT), a.to_vec()));
            e.mem.set_u32(process + 0x40, a[1]);
            ret(0)
        });
        assert!(w.e.call(0x0090_8bf0, &args![w.process, w.actor]).bool());
        assert_eq!(
            w.calls_to(process_slot(GET_TARGET_FOR_PACKAGE_SLOT)),
            [vec![w.process, w.actor, w.package]]
        );
        w.on(process_slot(GET_TARGET_FOR_PACKAGE_SLOT), 0);
        assert!(!w.e.call(0x0090_8bf0, &args![w.process, w.actor]).bool());
    }

    #[test]
    fn set_location_for_package_stores_the_first_or_second_location() {
        let mut w = world();
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(process_slot(GET_LOCATION_FOR_PACKAGE_SLOT), 0x6600);
        w.on(GET_0055B980, 0);
        assert_eq!(
            w.e.call(0x0090_8c50, &args![w.process, w.actor, 0u32])
                .u32(),
            0x6600
        );
        assert_eq!(w.e.mem.u32(w.process + 0x44), 0x6600);
        assert_eq!(
            w.e.call(0x0090_8c50, &args![w.process, w.actor, 1u32])
                .u32(),
            0x6600
        );
        assert_eq!(w.e.mem.u32(w.process + 0x48), 0x6600);
        assert_eq!(
            w.calls_to(process_slot(GET_LOCATION_FOR_PACKAGE_SLOT)),
            [
                vec![w.process, w.actor, w.package, 0],
                vec![w.process, w.actor, w.package, 1]
            ]
        );
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), 0);
        w.clear();
        assert_eq!(
            w.e.call(0x0090_8c50, &args![w.process, w.actor, 0u32])
                .u32(),
            0
        );
        assert_eq!(w.addresses(), [process_slot(GET_CURRENT_PACKAGE_SLOT)]);
    }

    /// `GetTimeDelta` with the calendar's hour, the last hour and the
    /// time-scale value.
    fn time_delta(hour: f32, last: f32, scale: f32) -> f32 {
        let mut w = world();
        w.on(LOOKUP_FORM_BY_ID, 0x4400);
        w.answer(CALENDAR_GET_HOUR, st0(f64::from(hour)));
        w.answer(PROCESS_GET_HOUR, st0(f64::from(last)));
        w.answer(GLOBAL_VALUE, st0(f64::from(scale)));
        w.e.map(HOURS_PER_DAY, 8);
        w.e.map(SECONDS_PER_HOUR, 8);
        w.e.mem.set_f64(HOURS_PER_DAY, 24.0);
        w.e.mem.set_f64(SECONDS_PER_HOUR, 3600.0);
        let value = w.e.call(0x0090_7260, &args![w.process, 0u32]).f32();
        assert_eq!(w.calls_to(LOOKUP_FORM_BY_ID), [vec![0x3a]]);
        assert_eq!(w.calls_to(GLOBAL_VALUE), [vec![0x4400]]);
        value
    }

    #[test]
    fn time_delta_converts_game_hours_to_seconds() {
        assert_eq!(time_delta(10.5, 10.0, 30.0), 60.0);
        // Past midnight the hour gains 24.
        assert_eq!(time_delta(1.0, 23.0, 30.0), 240.0);
        // Equal hours: nothing elapsed.
        assert_eq!(time_delta(5.0, 5.0, 30.0), 0.0);
    }

    #[test]
    fn combat_controller_packages_by_type_flags_and_count() {
        for (kind, flags, flag, count, expected) in [
            (0u32, 0x0200_0000u32, 0u8, 1i32, true),
            (0x11, 0x0400_0000, 1, 0, true),
            (6, 0x0400_0000, 0, 0, false),
            (7, 0x0200_0000, 0, 2, false),
            (2, 0x0600_0000, 1, 0, false),
            (0x12, 0x0600_0000, 1, 0, false),
        ] {
            let mut w = world();
            w.on(PACKAGE_GET_TYPE, kind);
            w.on(PACKAGE_GET_FLAGS, flags);
            w.on(PROCESS_GET_0045CD60, count as u32);
            let result =
                w.e.call(0x0090_78d0, &args![w.process, w.package, u32::from(flag)])
                    .bool();
            assert_eq!(result, expected, "type {kind:#x}");
        }
    }

    #[test]
    fn update_ov2_runs_missed_packages_after_an_hour() {
        let mut w = world();
        let cast = w.object();
        w.on(DYNAMIC_CAST, cast);
        w.on(process_slot(UPDATE_MISSED_PACKAGES_SLOT), 0);
        w.on(object_slot(SET_AVOID_WAIT_TIMER_SLOT), 0);
        w.e.map(SECONDS_PER_HOUR, 8);
        w.e.mem.set_f64(SECONDS_PER_HOUR, 3600.0);
        w.e.call(0x0090_c360, &args![w.process, w.actor, 3601.0f32]);
        assert_eq!(
            w.calls(),
            [
                (
                    DYNAMIC_CAST,
                    vec![w.actor, 0, MOBILE_OBJECT_TYPE, ACTOR_TYPE, 0]
                ),
                (
                    process_slot(UPDATE_MISSED_PACKAGES_SLOT),
                    vec![w.process, w.actor, 1]
                ),
                (object_slot(SET_AVOID_WAIT_TIMER_SLOT), vec![cast, 0]),
            ]
        );
        for time in [3600.0f32, 1.0, f32::NAN] {
            w.clear();
            w.e.call(0x0090_c360, &args![w.process, w.actor, time]);
            assert!(w
                .calls_to(process_slot(UPDATE_MISSED_PACKAGES_SLOT))
                .is_empty());
        }
        w.on(DYNAMIC_CAST, 0);
        w.clear();
        w.e.call(0x0090_c360, &args![w.process, w.actor, 5000.0f32]);
        assert_eq!(w.addresses(), [DYNAMIC_CAST]);
        w.clear();
        w.e.call(0x0090_c360, &args![w.process, 0u32, 5000.0f32]);
        assert!(w.calls().is_empty());
    }

    /// An actor whose value owner (+0xa4) answers through the object
    /// vtable.
    fn actor_values(w: &mut World, flagged: bool) {
        let owner = w.actor + ACTOR_VALUE_OWNER;
        w.e.mem.set_u32(owner, OBJECT_VTABLE);
        w.on(ACTOR_VALUE_HAS_FLAG, u32::from(flagged));
        w.on(object_slot(0), 40);
        w.answer(object_slot(4), st0(40.25));
        let actor = w.actor;
        let log = w.log.clone();
        w.e.register_double(MODIFIER_LIST_GET_MODIFIER, move |_, a| {
            log.borrow_mut()
                .push((MODIFIER_LIST_GET_MODIFIER, a.to_vec()));
            st0(if a[0] == actor + ACTOR_MODIFIERS {
                2.5
            } else {
                -1.0
            })
        });
        let log = w.log.clone();
        w.e.register_double(FTOL, move |_, a| {
            log.borrow_mut().push((FTOL, a.to_vec()));
            ret(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
        });
    }

    #[test]
    fn actor_values_add_the_modifiers_unless_flagged() {
        let mut w = world();
        actor_values(&mut w, false);
        let value =
            w.e.call(
                0x0090_72d0,
                &args![w.process, 0u32, 0x0000_0105u32, w.actor],
            )
            .i32();
        assert_eq!(value, 41);
        assert_eq!(w.calls_to(FTOL), [args![41.5f64]]);
        assert_eq!(
            w.calls_to(MODIFIER_LIST_GET_MODIFIER),
            [vec![w.actor + 0xd0, 5], vec![w.process + 0x94, 5]]
        );
        let value =
            w.e.call(0x0090_7380, &args![w.process, 0u32, 5u32, w.actor])
                .f64();
        assert_eq!(value, 41.75);
        let mut w = world();
        actor_values(&mut w, true);
        let value =
            w.e.call(0x0090_72d0, &args![w.process, 0u32, 5u32, w.actor])
                .i32();
        assert_eq!(value, 40);
        let value =
            w.e.call(0x0090_7380, &args![w.process, 0u32, 5u32, w.actor])
                .f64();
        assert_eq!(value, 40.25);
        assert!(w.calls_to(MODIFIER_LIST_GET_MODIFIER).is_empty());
        assert_eq!(
            w.calls_to(ACTOR_VALUE_HAS_FLAG),
            [vec![5, 0x100], vec![5, 0x100]]
        );
    }

    #[test]
    fn check_and_start_combat_controller_enters_combat() {
        let mut w = world();
        w.on(PACKAGE_GET_TYPE, 0);
        w.on(PACKAGE_GET_FLAGS, 0x0200_0000);
        w.on(PROCESS_GET_0045CD60, 0);
        w.on(process_slot(SET_TARGET_FOR_PACKAGE_SLOT), 0);
        w.on(process_slot(SET_LOCATION_FOR_PACKAGE_SLOT), 0);
        w.on(process_slot(ENTER_COMBAT_SLOT), 0);
        assert!(w
            .e
            .call(0x0090_7840, &args![w.process, w.actor, w.package, 0u32])
            .bool());
        let mut combat = vec![w.process, w.actor];
        combat.extend([0u32; 11]);
        combat.push(w.package);
        assert_eq!(w.calls_to(process_slot(ENTER_COMBAT_SLOT)), [combat]);
        assert_eq!(
            w.calls_to(process_slot(SET_LOCATION_FOR_PACKAGE_SLOT)),
            [vec![w.process, w.actor, 0]]
        );
        w.clear();
        assert!(!w
            .e
            .call(0x0090_7840, &args![w.process, w.actor, 0u32, 0u32])
            .bool());
        assert!(w.calls().is_empty());
        w.on(PACKAGE_GET_TYPE, 3);
        assert!(!w
            .e
            .call(0x0090_7840, &args![w.process, w.actor, w.package, 0u32])
            .bool());
        assert!(w.calls_to(process_slot(ENTER_COMBAT_SLOT)).is_empty());
    }

    /// Doubles for the package script and topic getters and the greet.
    fn package_events(w: &mut World, script: u32, topic: u32) {
        for address in [
            PACKAGE_START_SCRIPT,
            PACKAGE_CHANGE_SCRIPT,
            PACKAGE_DONE_SCRIPT,
        ] {
            w.on(address, script);
        }
        for address in [
            PACKAGE_START_TOPIC,
            PACKAGE_CHANGE_TOPIC,
            PACKAGE_DONE_TOPIC,
        ] {
            w.on(address, topic);
        }
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(EXTRA_GET_SCRIPT_LOCALS, 0x5555);
        w.on(SCRIPT_RUN, 0);
        w.on(process_slot(PROCESS_GREET_SLOT), 0);
        w.on(ACTOR_AFTER_NEW_PACKAGE, 0);
    }

    #[test]
    fn package_start_change_and_done_run_scripts_and_greet() {
        let mut w = world();
        package_events(&mut w, 0x7100, 0x7200);
        w.e.call(0x0091_3510, &args![w.process, w.actor, w.package]);
        assert_eq!(
            w.calls(),
            [
                (PACKAGE_START_SCRIPT, vec![w.package]),
                (REFERENCE_GET_EXTRA_LIST, vec![w.actor]),
                (EXTRA_GET_SCRIPT_LOCALS, vec![0x4444]),
                (SCRIPT_RUN, vec![0x7100, w.actor, 0x5555, 0, 1]),
                (PACKAGE_START_TOPIC, vec![w.package]),
                (
                    process_slot(PROCESS_GREET_SLOT),
                    vec![w.process, w.actor, 0x7200, 0, 0, 1, 0]
                ),
                (ACTOR_AFTER_NEW_PACKAGE, vec![w.actor]),
            ]
        );
        w.clear();
        w.e.call(0x0091_3630, &args![w.process, w.actor, w.package]);
        assert_eq!(
            w.addresses(),
            [
                PACKAGE_DONE_SCRIPT,
                REFERENCE_GET_EXTRA_LIST,
                EXTRA_GET_SCRIPT_LOCALS,
                SCRIPT_RUN,
                PACKAGE_DONE_TOPIC,
                process_slot(PROCESS_GREET_SLOT),
            ]
        );
        // The change script runs only when 00671d10 or 0055b980 holds.
        w.on(SCRIPT_GET_00671D10, 0);
        w.on(GET_0055B980, 0);
        w.clear();
        w.e.call(0x0091_35a0, &args![w.process, w.actor, w.package]);
        assert_eq!(
            w.addresses(),
            [
                PACKAGE_CHANGE_SCRIPT,
                SCRIPT_GET_00671D10,
                GET_0055B980,
                PACKAGE_CHANGE_TOPIC,
                process_slot(PROCESS_GREET_SLOT),
            ]
        );
        w.on(GET_0055B980, 1);
        w.clear();
        w.e.call(0x0091_35a0, &args![w.process, w.actor, w.package]);
        assert_eq!(w.calls_to(SCRIPT_RUN).len(), 1);
        // Without a package only StartNewPackage's actor call runs.
        w.clear();
        w.e.call(0x0091_3510, &args![w.process, w.actor, 0u32]);
        w.e.call(0x0091_35a0, &args![w.process, w.actor, 0u32]);
        w.e.call(0x0091_3630, &args![w.process, w.actor, 0u32]);
        assert_eq!(w.addresses(), [ACTOR_AFTER_NEW_PACKAGE]);
        // No script and no topic.
        let mut w = world();
        package_events(&mut w, 0, 0);
        w.e.call(0x0091_3630, &args![w.process, w.actor, w.package]);
        assert_eq!(w.addresses(), [PACKAGE_DONE_SCRIPT, PACKAGE_DONE_TOPIC]);
    }
}

#[cfg(test)]
mod b003_tests_2 {
    use super::b003_tests::*;
    use super::*;

    fn minus_one(w: &mut World) {
        w.e.map(MINUS_ONE, 4);
        w.e.mem.set_u32(MINUS_ONE, (-1.0f32).to_bits());
    }

    #[test]
    fn use_item_at_moves_the_procedure_index() {
        let mut w = world();
        minus_one(&mut w);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(package_slot(PACKAGE_SLOT_13C), 0);
        w.on(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        assert!(!w.e.call(0x0090_8e90, &args![w.process, w.actor]).bool());
        assert_eq!(
            w.calls_to(package_slot(PACKAGE_SLOT_13C)),
            [vec![w.package, w.actor, 0, (-1.0f32).to_bits(), 0]]
        );
        assert_eq!(
            w.calls_to(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)),
            [vec![w.process, w.actor, u32::MAX]]
        );
        // The package accepts: 0067efd0 decides the step of 1.
        w.on(package_slot(PACKAGE_SLOT_13C), 1);
        w.on(PACKAGE_TEST_0067EFD0, 1);
        w.clear();
        w.e.call(0x0090_8e90, &args![w.process, w.actor]);
        assert_eq!(
            w.calls_to(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)),
            [vec![w.process, w.actor, 1]]
        );
        w.on(PACKAGE_TEST_0067EFD0, 0x100);
        w.clear();
        w.e.call(0x0090_8e90, &args![w.process, w.actor]);
        assert!(w
            .calls_to(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT))
            .is_empty());
    }

    #[test]
    fn reset_target_takes_the_package_target_reference() {
        let mut w = world();
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(PACKAGE_GET_TARGET, 0x5100);
        w.on(PACKAGE_TARGET_GET_REFERENCE, 0x6000);
        w.on(REFERENCE_TEST_00440D80, 0);
        w.on(process_slot(SET_TARGET_SLOT), 0);
        w.e.call(0x0090_a070, &args![w.process]);
        assert_eq!(
            w.calls_to(process_slot(SET_TARGET_SLOT)),
            [vec![w.process, 0x6000]]
        );
        assert_eq!(w.calls_to(process_slot(GET_CURRENT_PACKAGE_SLOT)).len(), 4);
        // Already the target, or 00440d80 holds: nothing.
        w.e.mem.set_u32(w.process + 0x40, 0x6000);
        w.clear();
        w.e.call(0x0090_a070, &args![w.process]);
        assert!(w.calls_to(process_slot(SET_TARGET_SLOT)).is_empty());
        w.e.mem.set_u32(w.process + 0x40, 0);
        w.on(REFERENCE_TEST_00440D80, 1);
        w.clear();
        w.e.call(0x0090_a070, &args![w.process]);
        assert!(w.calls_to(process_slot(SET_TARGET_SLOT)).is_empty());
        // No reference.
        w.on(PACKAGE_TARGET_GET_REFERENCE, 0);
        w.clear();
        w.e.call(0x0090_a070, &args![w.process]);
        assert!(w.calls_to(REFERENCE_TEST_00440D80).is_empty());
    }

    #[test]
    fn flee_non_combat_runs_the_package_without_a_target() {
        let mut w = world();
        minus_one(&mut w);
        w.on(process_slot(GET_PACKAGE_RUNNING_SLOT), w.package);
        w.on(process_slot(SET_TARGET_FOR_PACKAGE_SLOT), 0);
        w.on(process_slot(SET_LOCATION_FOR_PACKAGE_SLOT), 0);
        w.on(PACKAGE_GET_TARGET, 0x5100);
        w.on(PACKAGE_TARGET_GET_VALUE, 500);
        w.on(package_slot(PACKAGE_SLOT_13C), 0);
        w.e.call(0x0090_9d00, &args![w.process, w.actor]);
        assert_eq!(
            w.addresses(),
            [
                process_slot(GET_PACKAGE_RUNNING_SLOT),
                process_slot(SET_TARGET_FOR_PACKAGE_SLOT),
                process_slot(SET_LOCATION_FOR_PACKAGE_SLOT),
                PACKAGE_GET_TARGET,
                PACKAGE_GET_TARGET,
                PACKAGE_TARGET_GET_VALUE,
                package_slot(PACKAGE_SLOT_13C),
            ]
        );
        // With a target: the distance is measured, the package not run.
        w.e.mem.set_u32(w.process + 0x40, 0x6000);
        w.answer(REFERENCE_GET_DISTANCE, st0(600.0));
        w.clear();
        w.e.call(0x0090_9d00, &args![w.process, w.actor]);
        assert_eq!(
            w.calls_to(REFERENCE_GET_DISTANCE),
            [vec![w.actor, 0x6000, 0, 0]]
        );
        assert!(w.calls_to(package_slot(PACKAGE_SLOT_13C)).is_empty());
        assert!(w
            .calls_to(process_slot(SET_TARGET_FOR_PACKAGE_SLOT))
            .is_empty());
    }

    fn greet(w: &mut World, topic_ok: u32, item: u32, response: u32, say_to: u32) {
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on_seq(ACTOR_GET_004FD380, vec![0, 0x7700]);
        w.on(ACTOR_SET_0057BD60, 0);
        w.on(TOPIC_TEST_00508D70, topic_ok);
        w.on(TOPIC_CREATE_DIALOGUE_ITEM, item);
        w.on(DIALOGUE_FIRST_RESPONSE, 0);
        w.on(DIALOGUE_GET_CURRENT_RESPONSE, response);
        w.on(DIALOGUE_RUN_RESULT, 0);
        w.on(REFERENCE_SET_SAY_TO_TOPIC, 0);
        w.on(GET_WORD_0C, 0x8800);
        w.on(REFERENCE_SET_SAY_TO_TOPIC_INFO, 0);
        w.on(SAY_TO_CALLBACK, 0);
        w.on(actor_slot(ACTOR_SLOT_140), 0);
        w.e.call(
            0x0091_36b0,
            &args![w.process, w.actor, 0x9900u32, 1u32, 2u32, 3u32, say_to],
        );
    }

    #[test]
    fn greet_creates_the_dialogue_item_and_says_the_topic() {
        let mut w = world();
        greet(&mut w, 1, 0xa000, 0xa100, 1);
        assert_eq!(
            w.calls(),
            [
                (process_slot(GET_CURRENT_PACKAGE_SLOT), vec![w.process]),
                (ACTOR_GET_004FD380, vec![w.actor]),
                (ACTOR_SET_0057BD60, vec![w.actor, 0]),
                (ACTOR_GET_004FD380, vec![w.actor]),
                (TOPIC_TEST_00508D70, vec![0x9900]),
                (
                    TOPIC_CREATE_DIALOGUE_ITEM,
                    vec![0x9900, w.actor, 0x7700, 0, 0, 1]
                ),
                (DIALOGUE_FIRST_RESPONSE, vec![0xa000]),
                (DIALOGUE_GET_CURRENT_RESPONSE, vec![0xa000]),
                (DIALOGUE_RUN_RESULT, vec![0xa000, 0]),
                (REFERENCE_SET_SAY_TO_TOPIC, vec![w.actor, 0x9900]),
                (GET_WORD_0C, vec![0xa000]),
                (REFERENCE_SET_SAY_TO_TOPIC_INFO, vec![w.actor, 0x8800]),
                (GET_WORD_0C, vec![w.actor]),
                (SAY_TO_CALLBACK, vec![0x8800, 1]),
                (actor_slot(ACTOR_SLOT_140), vec![w.actor]),
            ]
        );
        let mut w = world();
        greet(&mut w, 1, 0xa000, 0xa100, 0);
        assert_eq!(*w.addresses().last().unwrap(), DIALOGUE_RUN_RESULT);
        let mut w = world();
        greet(&mut w, 1, 0xa000, 0, 1);
        assert_eq!(
            *w.addresses().last().unwrap(),
            DIALOGUE_GET_CURRENT_RESPONSE
        );
        let mut w = world();
        greet(&mut w, 1, 0, 0, 1);
        assert_eq!(*w.addresses().last().unwrap(), TOPIC_CREATE_DIALOGUE_ITEM);
        let mut w = world();
        greet(&mut w, 0x100, 0, 0, 1);
        assert_eq!(*w.addresses().last().unwrap(), TOPIC_TEST_00508D70);
    }

    /// `AddToProcedureIndexRunning` with the index after the step, the
    /// procedure count and the procedure at the clamped index.
    fn procedure_step(index: i32, count: i32, procedure: u32, done_once: u32) -> World {
        let mut w = world();
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(process_slot(ADD_TO_CURRENT_PROCEDURE_INDEX_SLOT), 0);
        let process = w.process;
        let index_word = w.e.mem.alloc(4);
        w.e.mem.set_i32(index_word, index);
        let log = w.log.clone();
        w.e.register_double(
            process_slot(GET_CURRENT_PROCEDURE_INDEX_SLOT),
            move |e, a| {
                log.borrow_mut()
                    .push((process_slot(GET_CURRENT_PROCEDURE_INDEX_SLOT), a.to_vec()));
                ret(e.mem.u32(index_word))
            },
        );
        let log = w.log.clone();
        w.e.register_double(
            process_slot(SET_CURRENT_PROCEDURE_INDEX_SLOT),
            move |e, a| {
                log.borrow_mut()
                    .push((process_slot(SET_CURRENT_PROCEDURE_INDEX_SLOT), a.to_vec()));
                assert_eq!(a[0], process);
                e.mem.set_u32(index_word, a[1]);
                ret(0)
            },
        );
        w.on(PACKAGE_GET_PROCEDURE_TYPE, 3);
        w.on(PROCEDURE_COUNT, count as u32);
        w.e.map(PROCEDURE_LISTS, 0x40);
        let list = w.e.mem.alloc(0x40);
        w.e.mem.set_u32(PROCEDURE_LISTS + 12, list);
        let clamped = w.e.mem.u32(index_word).min(count as u32 - 1);
        w.e.mem.set_u32(list + 4 * clamped.min(15), procedure);
        w.on(process_slot(IS_PACKAGE_DONE_ONCE_SLOT), done_once);
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(SET_ACTION_FLAG, 0);
        w.on(process_slot(PACKAGE_DONE_SLOT), 0);
        w.on(process_slot(SET_PACKAGE_DONE_ONCE_SLOT), 0);
        w.e.call(0x0091_19b0, &args![w.process, w.actor, 1u32]);
        w
    }

    #[test]
    fn procedure_index_steps_clamp_and_finish_the_package() {
        let w = procedure_step(5, 4, PROCEDURE_DONE, 0);
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_PROCEDURE_INDEX_SLOT)),
            [vec![w.process, 3]]
        );
        assert_eq!(w.calls_to(PROCEDURE_COUNT), [vec![3]]);
        assert_eq!(
            w.calls_to(SET_ACTION_FLAG),
            [vec![w.package, 0x4444, 0x400]]
        );
        assert_eq!(
            w.calls_to(process_slot(PACKAGE_DONE_SLOT)),
            [vec![w.process, w.actor, w.package]]
        );
        assert_eq!(
            w.calls_to(process_slot(SET_PACKAGE_DONE_ONCE_SLOT)),
            [vec![w.process, 1]]
        );
        // Below zero: back to 0.
        let w = procedure_step(-1, 4, 0, 0);
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_PROCEDURE_INDEX_SLOT)),
            [vec![w.process, 0]]
        );
        assert!(w.calls_to(SET_ACTION_FLAG).is_empty());
        // Done once already.
        let w = procedure_step(1, 4, PROCEDURE_DONE, 1);
        assert!(w.calls_to(process_slot(PACKAGE_DONE_SLOT)).is_empty());
        assert!(w
            .calls_to(process_slot(SET_CURRENT_PROCEDURE_INDEX_SLOT))
            .is_empty());
    }

    #[test]
    fn schedule_package_becomes_current_with_its_start_location() {
        let mut w = world();
        w.on(ACTOR_GET_PACKAGE, 0x7100);
        w.on_seq(process_slot(GET_CURRENT_PACKAGE_SLOT), vec![0x7000, 0x7100]);
        w.on(process_slot(SET_CURRENT_PACKAGE_SLOT), 0);
        w.on(process_slot(SETUP_NEW_PACKAGE_SLOT), 0);
        w.on(ACTOR_CLEAR_LOOK_AT_TARGET, 0);
        w.on(GET_0055B980, 0x7300);
        w.on(PACKAGE_LOCATION_GET_TYPE, 2);
        let object = w.object();
        w.e.mem.set_u32(object + 8, 1.5f32.to_bits());
        w.on(ACTOR_GET_00430830, object);
        w.on(actor_slot(ACTOR_GET_LOCATION_SLOT), 0x7400);
        w.on(ACTOR_GET_008D6F30, 0x7500);
        w.on(REFERENCE_GET_WORLD_SPACE, 0x7600);
        w.on(REFERENCE_SET_PACKAGE_START_LOCATION, 0);
        w.on(actor_slot(ACTOR_GET_COMBAT_CONTROLLER_SLOT), 0x7800);
        w.on(COMBAT_UPDATE_PACKAGE_DATA, 0);
        w.on(process_slot(CHECK_AND_START_COMBAT_SLOT), 0);
        assert!(w.e.call(0x0090_7700, &args![w.process, w.actor]).bool());
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_PACKAGE_SLOT)),
            [vec![w.process, 0x7100, w.actor]]
        );
        assert_eq!(
            w.calls_to(REFERENCE_SET_PACKAGE_START_LOCATION),
            [vec![w.actor, 0x7600, 0x7500, 0x7400, 1.5f32.to_bits()]]
        );
        assert_eq!(
            w.calls_to(COMBAT_UPDATE_PACKAGE_DATA),
            [vec![0x7800, 0x7100]]
        );
        assert_eq!(
            w.calls_to(process_slot(CHECK_AND_START_COMBAT_SLOT)),
            [vec![w.process, w.actor, 0x7100, 0]]
        );
        // A located package of another type keeps its start location.
        w.on(PACKAGE_LOCATION_GET_TYPE, 1);
        w.on(actor_slot(ACTOR_GET_COMBAT_CONTROLLER_SLOT), 0);
        w.on_seq(process_slot(GET_CURRENT_PACKAGE_SLOT), vec![0x7000, 0x7100]);
        w.clear();
        assert!(w.e.call(0x0090_7700, &args![w.process, w.actor]).bool());
        assert!(w.calls_to(REFERENCE_SET_PACKAGE_START_LOCATION).is_empty());
        assert!(w.calls_to(COMBAT_UPDATE_PACKAGE_DATA).is_empty());
        // Already current: false at once.
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), 0x7100);
        w.clear();
        assert!(!w.e.call(0x0090_7700, &args![w.process, w.actor]).bool());
        assert_eq!(
            w.addresses(),
            [ACTOR_GET_PACKAGE, process_slot(GET_CURRENT_PACKAGE_SLOT)]
        );
    }

    /// List doubles: a list is its two head words (empty when both are
    /// zero); taking an item off shifts the second word into the first.
    fn lists(w: &mut World) {
        w.on(DELETE_EMPTY_DESTRUCTOR, 0);
        w.e.register(LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        w.e.register(LIST_HEAD_ITEM, |_, a| ret(a[0]));
        let log = w.log.clone();
        w.e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            log.borrow_mut()
                .push((LIST_REMOVE_ITEM, vec![a[0], e.mem.u32(a[1])]));
            let next = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0], next);
            e.mem.set_u32(a[0] + 4, 0);
            ret(0)
        });
        w.on(LIST_CLEAR, 0);
    }

    #[test]
    fn clear_current_data_frees_the_acquire_state() {
        let mut w = world();
        lists(&mut w);
        for slot in [
            SET_CURRENT_ACTION_COMPLETE_SLOT,
            SET_TARGET_ACTIVATED_SLOT,
            SET_TARGET_SLOT,
        ] {
            w.on(process_slot(slot), 0);
        }
        w.on(SET_FLAG_BITS, 0);
        let p = w.process;
        w.e.mem.set_u32(p + 0x44, 0x4400);
        w.e.mem.set_u32(p + 0x64, 0x6400);
        w.e.mem.set_u32(p + 0x68, 0x6800);
        w.e.mem.set_u32(p + 0x5c, 0x6800);
        w.e.mem.set_u32(p + 0x60, 0x5c01);
        w.e.call(0x0091_0b30, &args![p, 0u32]);
        assert_eq!(
            w.calls(),
            [
                (process_slot(SET_CURRENT_ACTION_COMPLETE_SLOT), vec![p, 0]),
                (SET_FLAG_BITS, vec![p, 0, 0x40]),
                (process_slot(SET_TARGET_ACTIVATED_SLOT), vec![p, 0]),
                (process_slot(SET_TARGET_SLOT), vec![p, 0]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x6400, 1]),
                (LIST_REMOVE_ITEM, vec![p + 0x5c, 0x6800]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x6800, 1]),
                (DELETE_EMPTY_DESTRUCTOR, vec![0x5c01, 1]),
                (LIST_REMOVE_ITEM, vec![p + 0x5c, 0x5c01]),
                (LIST_CLEAR, vec![p + 0x6c]),
            ]
        );
        for offset in [0x44, 0x64, 0x68, 0x5c, 0x60] {
            assert_eq!(w.e.mem.u32(p + offset), 0, "+{offset:#x}");
        }
    }

    /// The door helpers with the given answers.
    fn door(lock: bool, answers: &[(u32, u32)]) -> World {
        let mut w = world();
        w.on(actor_slot(ACTOR_SLOT_22C), 0);
        w.on(REFERENCE_GET_BASE_FORM, 0x3300);
        w.on(FORM_GET_TYPE, DOOR_FORM_TYPE);
        w.on(REFERENCE_GET_TELEPORT_DATA, 0x3400);
        w.on(REFERENCE_IS_AN_OWNER, 1);
        w.on(REFERENCE_GET_LOCK, 0x3500);
        w.on(REFERENCE_UNLOCK, 0);
        w.on(REFERENCE_LOCK, 0);
        w.on(TELEPORT_SET_CONNECTED_CELLS_PUBLIC, 0);
        w.on(TELEPORT_GET_WORLD_SPACE, 0);
        w.on(ACTOR_GET_008D6F30, 0x3600);
        w.on(TEST_00425FD0, 1);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        let entry = if lock { 0x0090_d480u32 } else { 0x0090_d570 };
        assert!(!w.e.call(entry, &args![0x3000u32, w.actor]).bool());
        w
    }

    #[test]
    fn unlocking_a_door_makes_its_cells_public() {
        let w = door(false, &[]);
        assert_eq!(
            w.calls_to(REFERENCE_IS_AN_OWNER),
            [vec![0x3000, w.actor, 1]]
        );
        assert_eq!(w.calls_to(REFERENCE_UNLOCK), [vec![0x3000]]);
        assert_eq!(
            w.calls_to(TELEPORT_SET_CONNECTED_CELLS_PUBLIC),
            [vec![0x3400, 0x3000, 1]]
        );
        let w = door(false, &[(REFERENCE_GET_TELEPORT_DATA, 0)]);
        assert_eq!(w.calls_to(REFERENCE_IS_AN_OWNER)[0][2], 0);
        assert!(w.calls_to(TELEPORT_SET_CONNECTED_CELLS_PUBLIC).is_empty());
        let w = door(false, &[(REFERENCE_GET_LOCK, 0)]);
        assert!(w.calls_to(REFERENCE_UNLOCK).is_empty());
        let w = door(false, &[(FORM_GET_TYPE, 0x1b)]);
        assert!(w.calls_to(REFERENCE_IS_AN_OWNER).is_empty());
        let w = door(false, &[(actor_slot(ACTOR_SLOT_22C), 1)]);
        assert_eq!(w.addresses(), [actor_slot(ACTOR_SLOT_22C)]);
    }

    #[test]
    fn locking_a_door_makes_its_cells_private() {
        // 00425fd0 holds: the owner flag stays 0.
        let w = door(true, &[]);
        assert_eq!(
            w.calls_to(REFERENCE_IS_AN_OWNER),
            [vec![0x3000, w.actor, 0]]
        );
        assert_eq!(w.calls_to(REFERENCE_LOCK), [vec![0x3000]]);
        assert_eq!(
            w.calls_to(TELEPORT_SET_CONNECTED_CELLS_PUBLIC),
            [vec![0x3400, 0x3000, 0]]
        );
        for answers in [
            [(TELEPORT_GET_WORLD_SPACE, 1), (TEST_00425FD0, 1)],
            [(ACTOR_GET_008D6F30, 0), (TEST_00425FD0, 1)],
            [(TEST_00425FD0, 0), (TELEPORT_GET_WORLD_SPACE, 0)],
        ] {
            let w = door(true, &answers);
            assert_eq!(w.calls_to(REFERENCE_IS_AN_OWNER)[0][2], 1);
        }
        let w = door(true, &[(REFERENCE_IS_AN_OWNER, 0)]);
        assert!(w.calls_to(REFERENCE_LOCK).is_empty());
    }
}

#[cfg(test)]
mod b003_tests_3 {
    use super::b003_tests::*;
    use super::*;

    fn constants(w: &mut World) {
        for (address, value) in [
            (ZERO, 0.0f64),
            (PATROL_NEAR, 0.001),
            (FLOAT_MAX, f64::from(f32::MAX)),
            (SECONDS_PER_HOUR, 3600.0),
            (HOURS_PER_DAY, 24.0),
        ] {
            w.e.map(address, 8);
            w.e.mem.set_f64(address, value);
        }
        for (address, value) in [
            (MINUS_ONE, -1.0f32),
            (PATROL_MOVE_DISTANCE, 64.0),
            (DOOR_PATH_LIMIT, 4096.0),
        ] {
            w.e.map(address, 4);
            w.e.mem.set_f32(address, value);
        }
        w.e.map(PLAYER, 4);
        w.e.mem.set_u32(PLAYER, 0x0909_0000);
        w.e.map(TES, 4);
        w.e.mem.set_u32(TES, 0x0808_0000);
        w.e.map(ANGLED_FORM, 4);
    }

    #[test]
    fn patrol_without_instance_data_or_while_busy() {
        let mut w = world();
        w.on(PROCESS_GET_INSTANCE_DATA, 0);
        w.on(process_slot(SET_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        w.e.call(0x0091_21e0, &args![w.process, w.actor]);
        assert_eq!(
            w.calls(),
            [
                (PROCESS_GET_INSTANCE_DATA, vec![w.process]),
                (
                    process_slot(SET_PROCEDURE_INDEX_RUNNING_SLOT),
                    vec![w.process, u32::MAX]
                ),
            ]
        );
        let data = w.e.mem.alloc(0x10);
        w.on(PROCESS_GET_INSTANCE_DATA, data);
        w.on(actor_slot(ACTOR_SLOT_214), 1);
        w.on(actor_slot(ACTOR_SLOT_418), 0);
        w.clear();
        w.e.call(0x0091_21e0, &args![w.process, w.actor]);
        assert_eq!(
            w.addresses(),
            [
                PROCESS_GET_INSTANCE_DATA,
                actor_slot(ACTOR_SLOT_214),
                actor_slot(ACTOR_SLOT_418)
            ]
        );
    }

    #[test]
    fn patrol_spends_the_time_through_the_states() {
        let mut w = world();
        constants(&mut w);
        let data = w.e.mem.alloc(0x10);
        w.on(PROCESS_GET_INSTANCE_DATA, data);
        w.on(actor_slot(ACTOR_SLOT_214), 0);
        // GetTimeDelta: 0.5 game hours at time scale 30 is 60 seconds.
        w.on(LOOKUP_FORM_BY_ID, 0x4400);
        w.answer(CALENDAR_GET_HOUR, st0(10.5));
        w.answer(PROCESS_GET_HOUR, st0(10.0));
        w.answer(GLOBAL_VALUE, st0(30.0));
        // State 2 at a reference with no wait: the patrol moves on to the
        // same reference (no time left) and its travel starts (state 1).
        let reference = w.object();
        w.on(PATROL_GET_CURRENT_REFERENCE, reference);
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(EXTRA_GET_0041FE90, 0);
        w.answer(LARGER_OF_00404010, st0(0.0));
        w.on(PATROL_SET_NEXT_REFERENCE, 1);
        w.on(process_slot(SET_TARGET_SLOT), 0);
        w.on(PLAYER_IS_SLEEPING_OR_RESTING, 1);
        w.e.mem.set_u32(data + 4, 2);
        w.e.call(0x0091_21e0, &args![w.process, w.actor]);
        // The travel ran (state 1 set by ProcessPatrolStartTravelToNextRef).
        assert_eq!(w.e.mem.u32(data + 4), 1);
        assert_eq!(w.calls_to(PATROL_SET_NEXT_REFERENCE), [vec![data, 1]]);
    }

    #[test]
    fn patrol_at_ref_waits_or_moves_on() {
        let mut w = world();
        constants(&mut w);
        let data = w.e.mem.alloc(0x10);
        let reference = w.object();
        w.on(PATROL_GET_CURRENT_REFERENCE, reference);
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(EXTRA_GET_0041FE90, 0x5500);
        w.answer(EXTRA_WAIT_TIME_006A7F50, st0(10.0));
        let log = w.log.clone();
        w.e.register_double(LARGER_OF_00404010, move |_, a| {
            log.borrow_mut().push((LARGER_OF_00404010, a.to_vec()));
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            st0(f64::from(if x <= y { y } else { x }))
        });
        // 4.0 waited of 10.0, 2.0 more passes: still waiting.
        w.e.mem.set_f32(data + 8, 4.0);
        let left =
            w.e.call(0x0091_2990, &args![w.process, w.actor, data, 2.0f32])
                .f32();
        assert_eq!(left, -4.0);
        assert_eq!(w.e.mem.f32(data + 8), 6.0);
        assert_eq!(w.calls_to(LARGER_OF_00404010), [vec![0, 6.0f32.to_bits()]]);
        // 7.0 more passes: 1.0 is left, the next reference is far away.
        let next = w.object();
        w.on_seq(PATROL_GET_CURRENT_REFERENCE, vec![reference, next, next]);
        w.on(PATROL_SET_NEXT_REFERENCE, 1);
        let point = w.e.mem.alloc(12);
        w.on(object_slot(ACTOR_GET_LOCATION_SLOT), point);
        w.on(POINT_SUBTRACT_004578C0, 0);
        w.answer(POINT_LENGTH_004A7290, st0(100.0));
        w.on(process_slot(SET_TARGET_SLOT), 0);
        w.on(PLAYER_IS_SLEEPING_OR_RESTING, 1);
        w.clear();
        let left =
            w.e.call(0x0091_2990, &args![w.process, w.actor, data, 7.0f32])
                .f32();
        assert_eq!(left, 3.0);
        assert_eq!(w.e.mem.u32(data + 4), 1);
        // The next reference is the same one: no time is left.
        w.on(PATROL_GET_CURRENT_REFERENCE, reference);
        w.e.mem.set_f32(data + 8, 0.0);
        let left =
            w.e.call(0x0091_2990, &args![w.process, w.actor, data, 20.0f32])
                .f32();
        assert_eq!(left, 0.0);
        // The next one fails: the index moves on and 0.0 comes back.
        w.on(PATROL_SET_NEXT_REFERENCE, 0);
        w.on(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        w.clear();
        let left =
            w.e.call(0x0091_2990, &args![w.process, w.actor, data, 20.0f32])
                .f32();
        assert_eq!(left, 0.0);
        assert_eq!(
            w.calls_to(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)),
            [vec![w.process, w.actor, 1]]
        );
        // No reference.
        w.on(PATROL_GET_CURRENT_REFERENCE, 0);
        w.on(process_slot(SET_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        let left =
            w.e.call(0x0091_2990, &args![w.process, w.actor, data, 20.0f32])
                .f32();
        assert_eq!(left, 0.0);
    }

    fn ambush(target: bool, answers: &[(u32, u32)]) -> World {
        let mut w = world();
        constants(&mut w);
        w.on(process_slot(GET_PACKAGE_RUNNING_SLOT), w.package);
        w.on(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        w.on(REFERENCE_TEST_00440DA0, 0);
        w.on(REFERENCE_TEST_00440D80, 0);
        w.on(PACKAGE_SET_NEVER_RUN, 0);
        w.on(object_slot(ACTOR_SLOT_22C), 0);
        w.on(package_slot(PACKAGE_SLOT_13C), 1);
        w.answer(REFERENCE_GET_DISTANCE, st0(500.0));
        w.on(package_slot(PACKAGE_SLOT_148), 1);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        if target {
            let target = w.object();
            w.e.mem.set_u32(w.process + 0x40, target);
        }
        w.e.call(0x0091_0c80, &args![w.process, w.actor]);
        w
    }

    fn index_steps(w: &World) -> Vec<u32> {
        w.calls_to(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT))
            .iter()
            .map(|words| words[2])
            .collect()
    }

    #[test]
    fn ambush_wait_branches() {
        assert_eq!(index_steps(&ambush(false, &[])), [2]);
        assert_eq!(
            index_steps(&ambush(true, &[(REFERENCE_TEST_00440DA0, 1)])),
            [2]
        );
        let w = ambush(true, &[(REFERENCE_TEST_00440D80, 1)]);
        assert_eq!(index_steps(&w), [2]);
        assert_eq!(w.calls_to(PACKAGE_SET_NEVER_RUN).len(), 1);
        let w = ambush(true, &[(object_slot(ACTOR_SLOT_22C), 1)]);
        assert!(index_steps(&w).is_empty());
        assert_eq!(w.calls_to(PACKAGE_SET_NEVER_RUN)[0][2], 1);
        let w = ambush(true, &[(package_slot(PACKAGE_SLOT_13C), 0)]);
        assert_eq!(index_steps(&w), [u32::MAX]);
        let w = ambush(true, &[]);
        assert_eq!(index_steps(&w), [1]);
        let w = ambush(true, &[(package_slot(PACKAGE_SLOT_148), 0)]);
        assert!(index_steps(&w).is_empty());
        let w = {
            let mut w = ambush(true, &[]);
            w.answer(REFERENCE_GET_DISTANCE, st0(f64::from(f32::MAX)));
            w.clear();
            w.e.call(0x0091_0c80, &args![w.process, w.actor]);
            w
        };
        assert!(w.calls_to(package_slot(PACKAGE_SLOT_148)).is_empty());
    }

    /// Doubles for the list walks of `ClearObjectFromAcquireList`: a list
    /// node is two words (item, next node).
    fn walk_lists(w: &mut World) {
        w.on(DELETE_EMPTY_DESTRUCTOR, 0);
        w.e.register(LIST_HEAD_ITEM, |_, a| ret(a[0]));
        w.e.register(LIST_NEXT_NODE, |e, a| ret(e.mem.u32(a[0] + 4)));
        let log = w.log.clone();
        w.e.register_double(LIST_REMOVE_ITEM, move |e, a| {
            let item = e.mem.u32(a[1]);
            log.borrow_mut().push((LIST_REMOVE_ITEM, vec![a[0], item]));
            // Unlink the node holding the item, from `a[0]` on.
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == item {
                    let next = e.mem.u32(node + 4);
                    if next != 0 {
                        let (i, n) = (e.mem.u32(next), e.mem.u32(next + 4));
                        e.mem.set_u32(node, i);
                        e.mem.set_u32(node + 4, n);
                    } else {
                        e.mem.set_u32(node, 0);
                    }
                    break;
                }
                node = e.mem.u32(node + 4);
            }
            ret(0)
        });
        let log = w.log.clone();
        w.e.register_double(LIST_REMOVE_HEAD, move |e, a| {
            log.borrow_mut().push((LIST_REMOVE_HEAD, a.to_vec()));
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let (i, n) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], i);
                e.mem.set_u32(a[0] + 4, n);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            ret(0)
        });
    }

    fn node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_u32(p, item);
        e.mem.set_u32(p + 4, next);
        p
    }

    #[test]
    fn clear_object_from_acquire_list_drops_the_reference_everywhere() {
        let mut w = world();
        walk_lists(&mut w);
        let p = w.process;
        let acquire = |e: &mut Engine, reference: u32| {
            let o = e.mem.alloc(8);
            e.mem.set_u32(o, reference);
            o
        };
        let mine = acquire(&mut w.e, 0x9000);
        let other = acquire(&mut w.e, 0x9100);
        let mine2 = acquire(&mut w.e, 0x9000);
        w.e.mem.set_u32(p + 0x64, mine);
        w.e.mem.set_u32(p + 0x68, other);
        // ObjectList: other, mine2.
        let second = node(&mut w.e, mine2, 0);
        w.e.mem.set_u32(p + 0x5c, other);
        w.e.mem.set_u32(p + 0x60, second);
        w.e.mem.set_u32(p + 0x44, 0x9000);
        // GenericLocationsList: 0x9000, 0x9100, 0x9000.
        let third = node(&mut w.e, 0x9000, 0);
        let middle = node(&mut w.e, 0x9100, third);
        w.e.mem.set_u32(p + 0x6c, 0x9000);
        w.e.mem.set_u32(p + 0x70, middle);
        w.e.call(0x0090_cc10, &args![p, 0x9000u32]);
        assert_eq!(w.e.mem.u32(p + 0x64), 0);
        assert_eq!(w.e.mem.u32(p + 0x68), other);
        assert_eq!(w.e.mem.u32(p + 0x44), 0);
        assert_eq!(w.e.mem.u32(p + 0x5c), other);
        assert_eq!(w.e.mem.u32(second), 0);
        assert_eq!(w.e.mem.u32(p + 0x6c), 0x9100);
        assert_eq!(
            w.calls(),
            [
                (DELETE_EMPTY_DESTRUCTOR, vec![mine, 1]),
                (LIST_REMOVE_ITEM, vec![p + 0x5c, mine2]),
                (DELETE_EMPTY_DESTRUCTOR, vec![mine2, 1]),
                (LIST_REMOVE_HEAD, vec![p + 0x6c]),
                (LIST_REMOVE_ITEM, vec![p + 0x6c, 0x9000]),
            ]
        );
    }

    #[test]
    fn clear_object_ov2_compares_the_base_forms() {
        let mut w = world();
        walk_lists(&mut w);
        // A reference's form is its word at +0xc (here: reference + 1).
        w.e.register(GET_WORD_0C, |_, a| ret(a[0] + 1));
        let p = w.process;
        let saved = w.e.mem.alloc(8);
        w.e.mem.set_u32(saved, 0x8fff);
        w.e.mem.set_u32(p + 0x68, saved);
        w.e.mem.set_u32(p + 0x44, 0x8fff);
        w.e.mem.set_u32(p + 0x6c, 0x7000);
        w.e.call(0x0090_cdc0, &args![p, 0x9000u32]);
        assert_eq!(w.e.mem.u32(p + 0x68), 0);
        assert_eq!(w.e.mem.u32(p + 0x44), 0);
        assert_eq!(w.e.mem.u32(p + 0x6c), 0x7000);
        assert_eq!(w.calls(), [(DELETE_EMPTY_DESTRUCTOR, vec![saved, 1])]);
    }

    /// The travel start with the given answers for the cell checks.
    fn travel(sleeping: u32, slot_1d0: u32, loaded: u32, package: u32) -> World {
        let mut w = world();
        constants(&mut w);
        let data = w.e.mem.alloc(0x10);
        let reference = w.object();
        w.on(PATROL_GET_CURRENT_REFERENCE, reference);
        w.on(process_slot(SET_TARGET_SLOT), 0);
        w.on(PLAYER_IS_SLEEPING_OR_RESTING, sleeping);
        w.on(actor_slot(ACTOR_SLOT_1D0), slot_1d0);
        w.on(ACTOR_GET_008D6F30, 0x3600);
        w.on(TES_IS_CELL_LOADED, loaded);
        w.answer(PATROL_TRAVEL_TIME, st0(25.0));
        w.on(process_slot(GET_PACKAGE_RUNNING_SLOT), package);
        w.on(PACKAGE_GET_00676280, 40);
        w.answer(LARGER_OF_00404010, st0(40.0));
        w.on(PATHING_REQUEST_CONSTRUCT, 0);
        w.on(REFERENCE_GET_WORLD_SPACE, 0x3700);
        w.on(object_slot(ACTOR_GET_LOCATION_SLOT), 0x3800);
        w.on(ACTOR_BUILD_REQUEST, 0);
        w.on(PATHING_REQUEST_SET_008E7C50, 0);
        w.on(REFERENCE_GET_BASE_FORM, 0x3900);
        w.on(FORM_GET_TYPE, 0x30);
        let angle = w.object();
        w.e.mem.set_f32(angle + 8, 0.5);
        w.on(ACTOR_GET_00430830, angle);
        w.on(PATHING_REQUEST_SET_008E7C20, 0);
        w.on(ACTOR_SET_PATHFINDING_GOAL, 0);
        w.on(PATHING_REQUEST_DESTRUCT, 0);
        w.e.call(0x0091_2340, &args![w.process, w.actor, data]);
        assert_eq!(w.e.mem.u32(data + 4), 1);
        assert_eq!(
            w.calls_to(process_slot(SET_TARGET_SLOT)),
            [vec![w.process, reference]]
        );
        w
    }

    #[test]
    fn travel_start_builds_a_pathing_request_unless_skipped() {
        let w = travel(0, 0, 1, 0x5000);
        let request = w.calls_to(PATHING_REQUEST_CONSTRUCT)[0][0];
        assert_eq!(w.calls_to(PACKAGE_GET_00676280), [vec![0x5000, w.actor]]);
        assert_eq!(
            w.calls_to(LARGER_OF_00404010),
            [vec![25.0f32.to_bits(), 40.0f32.to_bits()]]
        );
        let reference = w.calls_to(process_slot(SET_TARGET_SLOT))[0][1];
        assert_eq!(
            w.calls_to(ACTOR_BUILD_REQUEST),
            [vec![
                w.actor,
                request,
                0x3800,
                0x3600,
                0x3700,
                40.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(
            w.calls_to(PATHING_REQUEST_SET_008E7C20),
            [vec![request, 0.5f32.to_bits()]]
        );
        assert_eq!(
            w.calls_to(ACTOR_SET_PATHFINDING_GOAL),
            [vec![w.actor, request]]
        );
        assert_eq!(w.calls_to(PATHING_REQUEST_DESTRUCT), [vec![request]]);
        assert_eq!(w.calls_to(ACTOR_GET_008D6F30)[1], [reference]);
        // Sleeping, or no loaded cell: nothing is built.
        assert!(travel(1, 0, 1, 0).calls_to(ACTOR_BUILD_REQUEST).is_empty());
        assert!(travel(0, 0, 0, 0).calls_to(ACTOR_BUILD_REQUEST).is_empty());
        // The actor's slot +0x1d0 set: built without checking the cells.
        let w = travel(0, 1, 0, 0);
        assert_eq!(w.calls_to(ACTOR_BUILD_REQUEST).len(), 1);
        assert!(w.calls_to(TES_IS_CELL_LOADED).is_empty());
        assert!(w.calls_to(LARGER_OF_00404010).is_empty());
    }

    /// The patrol's travel step with the given answers.
    fn step(
        sleeping: u32,
        near_done: u32,
        package_test: u32,
        same_cell: bool,
        time: f32,
    ) -> (World, f32) {
        let mut w = world();
        constants(&mut w);
        let data = w.e.mem.alloc(0x10);
        let reference = w.object();
        w.on(PATROL_GET_CURRENT_REFERENCE, reference);
        let actor = w.actor;
        let log = w.log.clone();
        w.e.register_double(ACTOR_GET_008D6F30, move |_, a| {
            log.borrow_mut().push((ACTOR_GET_008D6F30, a.to_vec()));
            ret(if a[0] == actor || same_cell {
                0x3600
            } else {
                0x3700
            })
        });
        w.on(PLAYER_IS_SLEEPING_OR_RESTING, sleeping);
        w.on(TES_IS_CELL_LOADED, 1);
        w.on(ACTOR_TEST_008B3B90, near_done);
        w.on(process_slot(GET_PACKAGE_RUNNING_SLOT), w.package);
        w.on(PACKAGE_TEST_0067A480, package_test);
        w.answer(REFERENCE_GET_DISTANCE, st0(100.0));
        w.answer(PACKAGE_RADIUS_ACTOR_TO_LOCATION, st0(10.0));
        w.on(process_slot(CALCULATE_MOVE_MODE_SLOT), 0x202);
        w.on(process_slot(SET_ACTORS_ANIMATION_SLOT), 0);
        w.answer(ACTOR_GET_CURRENT_SPEED, st0(50.0));
        w.answer(LARGER_OF_00404010, st0(50.0));
        let point = w.e.mem.alloc(12);
        w.on(object_slot(ACTOR_GET_LOCATION_SLOT), point);
        w.on(actor_slot(ACTOR_GET_LOCATION_SLOT), point);
        w.on(POINT_SUBTRACT_004578C0, 0);
        w.answer(POINT_LENGTH_00457990, st0(200.0));
        w.on(PATHING_LOCATION_CONSTRUCT, 0);
        w.answer(TELEPORT_DOOR_PATH_LENGTH, st0(400.0));
        w.on(PATHING_LOCATION_DESTRUCT, 0);
        w.on(ACTOR_MOVE_TO_008AD910, 0);
        w.on(process_slot(SETUP_NEW_PACKAGE_SLOT), 0);
        w.on(ACTOR_IS_PATHING_COMPLETE, 0);
        // The restart of the travel, when it happens, stops at once.
        w.on(process_slot(SET_TARGET_SLOT), 0);
        w.on(actor_slot(ACTOR_SLOT_1D0), 0);
        w.answer(PATROL_TRAVEL_TIME, st0(0.0));
        w.on(PATHING_REQUEST_CONSTRUCT, 0);
        w.on(REFERENCE_GET_WORLD_SPACE, 0);
        w.on(ACTOR_BUILD_REQUEST, 0);
        w.on(PATHING_REQUEST_SET_008E7C50, 0);
        w.on(REFERENCE_GET_BASE_FORM, 0x3900);
        w.on(FORM_GET_TYPE, 0);
        w.on(ACTOR_SET_PATHFINDING_GOAL, 0);
        w.on(PATHING_REQUEST_DESTRUCT, 0);
        w.on(PACKAGE_GET_00676280, 0);
        let left =
            w.e.call(0x0091_25b0, &args![w.process, w.actor, data, time])
                .f32();
        (w, left)
    }

    #[test]
    fn travel_step_arrives_at_once_when_fast() {
        // Fast (sleeping) in the same cell: 200 / 50 = 4 seconds spent.
        let (w, left) = step(1, 0, 1, true, 10.0);
        assert_eq!(left, 6.0);
        let mode = w.calls_to(process_slot(CALCULATE_MOVE_MODE_SLOT));
        assert_eq!(
            mode,
            [vec![
                w.process,
                w.actor,
                100.0f32.to_bits(),
                10.0f32.to_bits(),
                20.0f32.to_bits(),
                0,
                0
            ]]
        );
        assert_eq!(
            w.calls_to(process_slot(SET_ACTORS_ANIMATION_SLOT)),
            [vec![w.process, w.actor, 0x202, 1]]
        );
        assert_eq!(
            w.calls_to(LARGER_OF_00404010),
            [vec![50.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        assert_eq!(w.calls_to(ACTOR_MOVE_TO_008AD910)[0][2..], [1, 0]);
        assert_eq!(w.calls_to(process_slot(SETUP_NEW_PACKAGE_SLOT)).len(), 1);
        // Other cells: the door path length, 400 / 50 = 8.
        let (w, left) = step(1, 0, 0, false, 10.0);
        assert_eq!(left, 2.0);
        assert_eq!(
            w.calls_to(process_slot(SET_ACTORS_ANIMATION_SLOT)),
            [vec![w.process, w.actor, 0x101, 0]]
        );
        let locations = w.calls_to(PATHING_LOCATION_CONSTRUCT);
        let paths = w.calls_to(TELEPORT_DOOR_PATH_LENGTH);
        assert_eq!(
            paths,
            [vec![
                locations[0][0],
                locations[1][0],
                4096.0f32.to_bits(),
                0,
                0
            ]]
        );
        assert_eq!(w.calls_to(PATHING_LOCATION_DESTRUCT).len(), 2);
    }

    #[test]
    fn travel_step_waits_for_the_path_when_not_fast() {
        // Not fast, 008b3b90 holds: the travel restarts, all time is left.
        let (w, left) = step(0, 1, 0, true, 10.0);
        assert_eq!(left, 10.0);
        assert_eq!(w.calls_to(ACTOR_BUILD_REQUEST).len(), 1);
        assert!(w
            .calls_to(process_slot(SET_ACTORS_ANIMATION_SLOT))
            .is_empty());
        // Not fast, pathing not complete: no time left.
        let (w, left) = step(0, 0, 0, true, 10.0);
        assert_eq!(left, 0.0);
        assert!(w.calls_to(process_slot(SETUP_NEW_PACKAGE_SLOT)).is_empty());
    }
}

#[cfg(test)]
mod b003_tests_4 {
    use super::b003_tests::*;
    use super::*;

    fn constants(w: &mut World) {
        for (address, value) in [
            (MINUS_ONE, -1.0f32),
            (SANDBOX_LEAST_TIME, 10.0),
            (GUARD_DEFAULT_RADIUS, 30.0),
        ] {
            w.e.map(address, 4);
            w.e.mem.set_f32(address, value);
        }
        for (address, value) in [(SECONDS_PER_HOUR, 3600.0f64), (HOURS_PER_DAY, 24.0)] {
            w.e.map(address, 8);
            w.e.mem.set_f64(address, value);
        }
        w.e.map(PLAYER, 4);
        w.e.mem.set_u32(PLAYER, 0x0909_0000);
        w.e.map(TES, 4);
        w.e.mem.set_u32(TES, 0x0808_0000);
    }

    #[test]
    fn save_package_to_extra_data_stores_the_package_state() {
        let mut w = world();
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(PACKAGE_GET_IS_CREATED, 0);
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(process_slot(IS_PACKAGE_DONE_ONCE_SLOT), 0x0000_0101);
        w.on(process_slot(IS_TARGET_ACTIVATED_SLOT), 0);
        w.on(process_slot(GET_CURRENT_ACTION_COMPLETE_SLOT), 1);
        w.on(process_slot(GET_CURRENT_PROCEDURE_INDEX_SLOT), 3);
        w.on(EXTRA_SET_PACKAGE, 0);
        w.on(actor_slot(ACTOR_SLOT_48), 0);
        w.on(PROCESS_GET_INSTANCE_DATA, 0x5500);
        let process = w.object();
        w.on(ACTOR_GET_PROCESS, process);
        w.on(object_slot(GET_ACTOR_PACKAGE_RUNNING_SLOT), 0x5600);
        w.on(PACKAGE_TARGET_GET_VALUE, 9);
        w.on(EXTRA_SET_ACTOR_PACKAGE_DATA, 0);
        w.on(ACTOR_PACKAGE_NULL_INSTANCE_DATA, 0);
        w.e.mem.set_u32(w.process + 0x40, 0x6000);
        w.e.call(0x0091_30f0, &args![w.process, w.actor]);
        assert_eq!(
            w.calls_to(EXTRA_SET_PACKAGE),
            [vec![0x4444, w.package, 3, 0x6000, 1, 0, 1]]
        );
        assert_eq!(
            w.calls_to(actor_slot(ACTOR_SLOT_48)),
            [vec![w.actor, 0x8000_0000]]
        );
        assert_eq!(w.calls_to(PACKAGE_TARGET_GET_VALUE), [vec![0x5600]]);
        assert_eq!(
            w.calls_to(EXTRA_SET_ACTOR_PACKAGE_DATA),
            [vec![0x4444, 0x5500]]
        );
        assert_eq!(
            w.calls_to(ACTOR_PACKAGE_NULL_INSTANCE_DATA),
            [vec![w.process + 4]]
        );
        // Without instance data only the package is stored.
        w.on(PROCESS_GET_INSTANCE_DATA, 0);
        w.clear();
        w.e.call(0x0091_30f0, &args![w.process, w.actor]);
        assert!(w.calls_to(EXTRA_SET_ACTOR_PACKAGE_DATA).is_empty());
        // A created package, or none: nothing.
        w.on(PACKAGE_GET_IS_CREATED, 1);
        w.clear();
        w.e.call(0x0091_30f0, &args![w.process, w.actor]);
        assert!(w.calls_to(EXTRA_SET_PACKAGE).is_empty());
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), 0);
        w.clear();
        w.e.call(0x0091_30f0, &args![w.process, w.actor]);
        assert_eq!(w.addresses(), [process_slot(GET_CURRENT_PACKAGE_SLOT)]);
    }

    fn load_package(w: &mut World, stored: u32, index: u32, count: u32, kind: u32) {
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(EXTRA_GET_PACKAGE, stored);
        w.on(actor_slot(ACTOR_SLOT_100), 1);
        w.on(process_slot(CLEAR_CURRENT_DATA_SLOT), 0);
        w.on(EXTRA_GET_PACKAGE_TARGET, 0x6000);
        w.on(ACTOR_PACKAGE_INIT_FROM_PACKAGE, 0);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), stored);
        w.on(ACTOR_TAKE_PACKAGE_004534F0, 0);
        w.on(EXTRA_GET_PACKAGE_INDEX, index);
        w.on(ACTOR_PACKAGE_GET_PACKAGE, stored);
        w.on(PACKAGE_GET_PROCEDURE_TYPE, kind);
        w.on(PROCEDURE_COUNT, count);
        for slot in [
            SET_CURRENT_PROCEDURE_INDEX_SLOT,
            SET_TARGET_SLOT,
            SET_CURRENT_ACTION_COMPLETE_SLOT,
            SET_PACKAGE_DONE_ONCE_SLOT,
            SET_TARGET_ACTIVATED_SLOT,
            CLEAR_CURRENT_PACKAGE_SLOT,
        ] {
            w.on(process_slot(slot), 0);
        }
        w.on(EXTRA_GET_ACTION_COMPLETE, 0x0000_0201);
        w.on(EXTRA_GET_DONE_ONCE, 0);
        w.on(EXTRA_GET_TARGET_ACTIVATED, 1);
        w.on(EXTRA_REMOVE_PACKAGE, 0);
        w.on(EXTRA_GET_ACTOR_PACKAGE_DATA, 0x5500);
        w.on(SET_INSTANCE_DATA_006ECD40, 0);
        w.on(EXTRA_REMOVE_ACTOR_PACKAGE_DATA, 0);
        w.on(SET_FLAG_BITS, 0);
        w.e.call(0x0091_3250, &args![w.process, w.actor]);
    }

    #[test]
    fn load_package_from_extra_data_restores_the_state() {
        let mut w = world();
        load_package(&mut w, 0x7000, 5, 4, 2);
        let p = w.process;
        assert_eq!(
            w.calls_to(ACTOR_PACKAGE_INIT_FROM_PACKAGE),
            [vec![p + 4, 0x7000, 0x6000]]
        );
        assert_eq!(
            w.calls_to(ACTOR_TAKE_PACKAGE_004534F0),
            [vec![w.actor, 0x7000]]
        );
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_PROCEDURE_INDEX_SLOT)),
            [vec![p, 0]]
        );
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_ACTION_COMPLETE_SLOT)),
            [vec![p, 1]]
        );
        assert_eq!(
            w.calls_to(process_slot(SET_TARGET_ACTIVATED_SLOT)),
            [vec![p, 1]]
        );
        assert_eq!(
            w.calls_to(SET_INSTANCE_DATA_006ECD40),
            [vec![p + 4, 0x5500]]
        );
        assert_eq!(w.calls_to(EXTRA_REMOVE_ACTOR_PACKAGE_DATA).len(), 1);
        // A kept index, and a type of -1 keeps any index.
        let mut w = world();
        load_package(&mut w, 0x7000, 2, 4, 2);
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_PROCEDURE_INDEX_SLOT))[0][1],
            2
        );
        let mut w = world();
        load_package(&mut w, 0x7000, 9, 4, u32::MAX);
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_PROCEDURE_INDEX_SLOT))[0][1],
            9
        );
        assert!(w.calls_to(PROCEDURE_COUNT).is_empty());
        // Nothing stored: the package state is cleared.
        let mut w = world();
        load_package(&mut w, 0, 0, 0, 0);
        let p = w.process;
        assert_eq!(
            w.calls(),
            [
                (REFERENCE_GET_EXTRA_LIST, vec![w.actor]),
                (EXTRA_GET_PACKAGE, vec![0x4444]),
                (actor_slot(ACTOR_SLOT_100), vec![w.actor]),
                (process_slot(CLEAR_CURRENT_DATA_SLOT), vec![p, w.actor]),
                (process_slot(CLEAR_CURRENT_PACKAGE_SLOT), vec![p]),
                (process_slot(SET_TARGET_SLOT), vec![p, 0]),
                (process_slot(SET_CURRENT_ACTION_COMPLETE_SLOT), vec![p, 0]),
                (SET_FLAG_BITS, vec![p, 0, 0x40]),
                (process_slot(SET_PACKAGE_DONE_ONCE_SLOT), vec![p, 0]),
                (process_slot(SET_TARGET_ACTIVATED_SLOT), vec![p, 0]),
            ]
        );
    }

    #[test]
    fn save_game_writes_the_low_process_fields() {
        let mut w = world();
        let p = w.process;
        let buffer = 0x0a0a_0000u32;
        for address in [
            BASE_PROCESS_SAVE_GAME,
            SAVE_DATA,
            SAVE_FORM_ID,
            COMBAT_TIMER_SAVE,
            SAVE_VARIABLE_SIZE,
            MODIFIER_LIST_SAVE,
        ] {
            w.on(address, 0);
        }
        w.on(SAVE_START_VARIABLE_SIZE, 0x77);
        w.e.register(LIST_HEAD_ITEM, |_, a| ret(a[0]));
        w.e.register(LIST_NEXT_NODE, |e, a| ret(e.mem.u32(a[0] + 4)));
        w.on(SAVE_FLAGS_00428110, 0x8800);
        w.on(FLAGS_HAVE_004280F0, 1);
        w.e.mem.set_u32(p + 0x34, 0x3434);
        w.e.mem.set_u32(p + 0x40, 0x4040);
        w.e.mem.set_u32(p + 0x44, 0x4444);
        w.e.mem.set_u32(p + 0x48, 0x4848);
        w.e.mem.set_u32(p + 0x4c, 0x4c4c);
        w.e.mem.set_u32(p + 0x54, 0x5454);
        // GenericLocationsList: 0x9000, a null entry, 0x9100.
        let third = w.e.mem.alloc(8);
        w.e.mem.set_u32(third, 0x9100);
        let second = w.e.mem.alloc(8);
        w.e.mem.set_u32(second + 4, third);
        w.e.mem.set_u32(p + 0x6c, 0x9000);
        w.e.mem.set_u32(p + 0x70, second);
        w.e.call(0x0091_0450, &args![p, buffer]);
        assert_eq!(
            w.calls_to(SAVE_DATA),
            [
                vec![buffer, p + 0x30, 1, 0],
                vec![buffer, p + 0xa4, 4, 0],
                vec![buffer, p + 0x58, 4, 0],
                vec![buffer, p + 0xa8, 4, 0],
                vec![buffer, p + 0xac, 4, 0],
                vec![buffer, p + 0xb0, 1, 0],
                vec![buffer, p + 0x50, 2, 0],
            ]
        );
        let forms: Vec<u32> = w.calls_to(SAVE_FORM_ID).iter().map(|a| a[1]).collect();
        assert_eq!(
            forms,
            [0x3434, 0x4040, 0x4444, 0x4848, 0x4c4c, 0x5454, 0x9000, 0x9100]
        );
        assert_eq!(w.calls_to(COMBAT_TIMER_SAVE), [vec![p + 0x38, buffer]]);
        assert_eq!(w.calls_to(SAVE_VARIABLE_SIZE), [vec![buffer, 2, 0x77]]);
        assert_eq!(
            w.calls_to(FLAGS_HAVE_004280F0),
            [vec![0x8800, SAVE_MODIFIERS_FLAG]]
        );
        assert_eq!(w.calls_to(MODIFIER_LIST_SAVE), [vec![p + 0x94, buffer]]);
        // An acquire object hides the target; no modifier flag, no list.
        w.e.mem.set_u32(p + 0x64, 0x6464);
        w.on(FLAGS_HAVE_004280F0, 0x100);
        w.clear();
        w.e.call(0x0091_0450, &args![p, buffer]);
        assert_eq!(w.calls_to(SAVE_FORM_ID)[1][1], 0);
        assert!(w.calls_to(MODIFIER_LIST_SAVE).is_empty());
    }

    fn follow(w: &mut World, distance: f64, answers: &[(u32, u32)]) {
        constants(w);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(PACKAGE_TEST_008A6290, 0);
        w.on(PLAYER_TEST_00962720, 0);
        w.on(PLAYER_COUNT_00962620, 1);
        let setting = w.e.mem.alloc(8);
        w.e.mem.set_u32(setting, 3);
        w.on(SETTING_VALUE_0043D4D0, setting);
        w.on(PACKAGE_SET_00674F30, 0);
        w.on(process_slot(SET_TARGET_FOR_PACKAGE_SLOT), 0);
        w.on(GET_0055B980, 1);
        w.on(package_slot(PACKAGE_SLOT_13C), 1);
        w.on(PACKAGE_GET_TYPE, 1);
        w.on(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        w.on(PACKAGE_GET_IS_CREATED, 1);
        w.on(process_slot(CLEAR_CURRENT_DATA_SLOT), 0);
        w.on(process_slot(CLEAR_CURRENT_PACKAGE_SLOT), 0);
        w.on(process_slot(CHECK_FOR_NEW_PACKAGE_SLOT), 0);
        w.on(LOOKUP_FORM_BY_ID, 0x4400);
        w.answer(CALENDAR_GET_HOUR, st0(10.5));
        w.answer(PROCESS_GET_HOUR, st0(10.0));
        w.answer(GLOBAL_VALUE, st0(30.0));
        w.answer(PACKAGE_RADIUS_ACTOR_TO_TARGET, st0(100.0));
        w.on(REFERENCE_GET_WORLD_SPACE, 0x2200);
        w.on(ACTOR_IS_PATHING_COMPLETE, 1);
        w.answer(REFERENCE_GET_DISTANCE, st0(distance));
        w.on(ACTOR_GET_008D6F30, 0x3600);
        w.on(object_slot(ACTOR_GET_LOCATION_SLOT), 0x3800);
        w.on(ACTOR_SET_PATHFINDING_GOAL_TO_LOCATION, 0);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        w.e.call(0x0090_9de0, &args![w.process, w.actor, 1u32, 2u32, 3u32]);
    }

    #[test]
    fn follow_finishes_or_paths_to_the_target() {
        // No target can be set: the package is finished and dropped.
        let mut w = world();
        follow(&mut w, 150.0, &[]);
        assert_eq!(
            w.calls_to(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)),
            [vec![w.process, w.actor, 1]]
        );
        assert_eq!(
            w.calls_to(process_slot(CHECK_FOR_NEW_PACKAGE_SLOT)),
            [vec![w.process, w.actor, 0]]
        );
        // With a target farther than the radius: a pathing goal.
        let mut w = world();
        let target = w.object();
        w.e.mem.set_u32(w.process + 0x40, target);
        follow(&mut w, 150.0, &[(PACKAGE_GET_TYPE, 2)]);
        assert!(w
            .calls_to(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT))
            .is_empty());
        assert_eq!(
            w.calls_to(ACTOR_SET_PATHFINDING_GOAL_TO_LOCATION),
            [vec![w.actor, 0x3800, 0x3600, 0x2200, 100.0f32.to_bits(), 0]]
        );
        // Within the radius: no goal.
        let mut w = world();
        let target = w.object();
        w.e.mem.set_u32(w.process + 0x40, target);
        follow(&mut w, 50.0, &[(GET_0055B980, 0)]);
        assert_eq!(w.calls_to(REFERENCE_GET_DISTANCE).len(), 1);
        assert!(w
            .calls_to(ACTOR_SET_PATHFINDING_GOAL_TO_LOCATION)
            .is_empty());
        // The player's follower check stops it at once.
        let mut w = world();
        follow(
            &mut w,
            150.0,
            &[(PACKAGE_TEST_008A6290, 1), (PLAYER_TEST_00962720, 1)],
        );
        assert!(w
            .calls_to(process_slot(SET_TARGET_FOR_PACKAGE_SLOT))
            .is_empty());
        // Below the follower limit the package gets 00674f30(0).
        let mut w = world();
        follow(&mut w, 150.0, &[(PACKAGE_TEST_008A6290, 1)]);
        assert_eq!(w.calls_to(PACKAGE_SET_00674F30), [vec![w.package, 0]]);
        // At the limit it stops.
        let mut w = world();
        follow(
            &mut w,
            150.0,
            &[(PACKAGE_TEST_008A6290, 1), (PLAYER_COUNT_00962620, 3)],
        );
        assert!(w.calls_to(PACKAGE_SET_00674F30).is_empty());
        assert!(w
            .calls_to(process_slot(SET_TARGET_FOR_PACKAGE_SLOT))
            .is_empty());
    }

    fn sandbox(state: u32, answers: &[(u32, u32)]) -> (World, u32) {
        let mut w = world();
        constants(&mut w);
        let data = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(data + 4, state);
        w.on(PROCESS_GET_INSTANCE_DATA, data);
        w.on(process_slot(GET_PACKAGE_RUNNING_SLOT), w.package);
        w.on(GET_0055B980, 0x5500);
        w.on(SANDBOX_START_009F3AF0, 0);
        w.on(ACTOR_IS_PATHING_COMPLETE, 1);
        let old = w.object();
        let next = w.object();
        w.on_seq(SANDBOX_GET_LOCATION, vec![old, next]);
        w.on(SANDBOX_MOVE_TO_NEXT_LOCATION, 1);
        w.on(ACTOR_GET_008D6F30, 0x3600);
        w.on(TES_IS_CELL_LOADED, 1);
        w.on(object_slot(ACTOR_GET_LOCATION_SLOT), 0x3800);
        w.on(actor_slot(ACTOR_SLOT_2A8), 0);
        w.on(REFERENCE_GET_WORLD_SPACE, 0x2200);
        w.on(REFERENCE_MOVE_TO_NEW_SPACE, 0);
        w.on(PACKAGE_GET_00676280, 4);
        let log = w.log.clone();
        w.e.register_double(LARGER_OF_00404010, move |_, a| {
            log.borrow_mut().push((LARGER_OF_00404010, a.to_vec()));
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            st0(f64::from(if x <= y { y } else { x }))
        });
        w.on(ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE, 0);
        w.on(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        w.e.call(0x0091_1f00, &args![w.process, w.actor]);
        (w, data)
    }

    #[test]
    fn sandbox_states() {
        let (w, data) = sandbox(0, &[]);
        assert_eq!(w.e.mem.u32(data + 4), 2);
        assert_eq!(
            w.calls_to(SANDBOX_START_009F3AF0),
            [vec![data, w.actor, 0x5500]]
        );
        // State 1: on to 3, then a path to the next location.
        let (w, data) = sandbox(1, &[]);
        assert_eq!(w.e.mem.u32(data + 4), 3);
        let next = w.calls_to(ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE)[0][1];
        assert_eq!(
            w.calls_to(ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE),
            [vec![w.actor, next, 10.0f32.to_bits(), 0]]
        );
        assert_eq!(
            w.calls_to(LARGER_OF_00404010),
            [vec![4.0f32.to_bits(), 10.0f32.to_bits()]]
        );
        // No loaded cell: the actor is placed, moved to the cell when it
        // differs.
        let (w, _) = sandbox(3, &[(TES_IS_CELL_LOADED, 0)]);
        assert_eq!(
            w.calls_to(actor_slot(ACTOR_SLOT_2A8)),
            [vec![w.actor, 0x3800]]
        );
        assert!(w.calls_to(REFERENCE_MOVE_TO_NEW_SPACE).is_empty());
        let (w, _) = sandbox(3, &[(TES_IS_CELL_LOADED, 0), (ACTOR_GET_008D6F30, 0)]);
        assert_eq!(
            w.calls_to(REFERENCE_MOVE_TO_NEW_SPACE),
            [vec![w.actor, 0, 0x2200]]
        );
        // Pathing not complete, no next location, or an unknown state.
        let (w, _) = sandbox(3, &[(ACTOR_IS_PATHING_COMPLETE, 0)]);
        assert!(w.calls_to(SANDBOX_GET_LOCATION).is_empty());
        let (w, _) = sandbox(3, &[(SANDBOX_MOVE_TO_NEXT_LOCATION, 0)]);
        assert_eq!(w.calls_to(SANDBOX_GET_LOCATION).len(), 1);
        let (w, _) = sandbox(4, &[]);
        assert_eq!(w.addresses(), [PROCESS_GET_INSTANCE_DATA]);
        // No data: the index moves on.
        let mut w = world();
        w.on(PROCESS_GET_INSTANCE_DATA, 0);
        w.on(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        w.e.call(0x0091_1f00, &args![w.process, w.actor]);
        assert_eq!(
            w.calls_to(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)),
            [vec![w.process, w.actor, 1]]
        );
    }

    fn guard(answers: &[(u32, u32)]) -> World {
        let mut w = world();
        constants(&mut w);
        w.on(PROCESS_GET_INSTANCE_DATA, 0x5500);
        w.on(process_slot(GET_RUN_ONCE_PACKAGE_SLOT), 0);
        w.on(process_slot(GET_RUN_ONCE_INSTANCE_DATA_SLOT), 0x5600);
        w.on(PACKAGE_GET_TYPE, 0xe);
        w.on(process_slot(SET_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        w.on(process_slot(GET_PACKAGE_RUNNING_SLOT), w.package);
        let guarded = w.object();
        w.on(PACKAGE_COMPUTE_INITIAL_TARGET, guarded);
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(EXTRA_GET_REFERENCE_POINTER, 0);
        w.on(process_slot(GET_TARGET_SLOT), 0);
        w.on(process_slot(SET_TARGET_SLOT), 0);
        w.on(EXTRA_ADD_GUARD, 0);
        w.on(object_slot(ACTOR_SLOT_48), 0);
        w.on(process_slot(IS_RUNNING_RUN_ONCE_SLOT), 0);
        w.on(actor_slot(ACTOR_SLOT_214), 0);
        w.on(SET_INSTANCE_DATA_006ECD40, 0);
        w.on(actor_slot(ACTOR_SLOT_418), 0);
        w.on(actor_slot(ACTOR_SLOT_4B4), 0);
        w.on(GET_0055B980, 0);
        w.on(package_slot(PACKAGE_SLOT_13C), 0);
        w.on(PACKAGE_GET_00676280, 0);
        w.on(PACKAGE_GET_TARGET, 0);
        w.on(PACKAGE_TARGET_GET_VALUE, 0);
        w.answer(REFERENCE_GET_DISTANCE, st0(50.0));
        w.on(ACTOR_IS_PATHING_COMPLETE, 1);
        w.on(ACTOR_TEST_008B3B90, 0);
        w.on(process_slot(PROCESS_GUARD_START_TRAVEL_SLOT), 0);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        w.e.call(0x0091_2b00, &args![w.process, w.actor]);
        w
    }

    #[test]
    fn guard_takes_the_reference_and_travels_when_far() {
        // 30.0 (no package target) is under 50.0: not near, the travel starts.
        let w = guard(&[]);
        let guarded = w.calls_to(process_slot(SET_TARGET_SLOT))[0][1];
        assert_eq!(w.calls_to(EXTRA_ADD_GUARD), [vec![0x4444, w.actor]]);
        assert_eq!(
            w.calls_to(object_slot(ACTOR_SLOT_48)),
            [vec![guarded, 0x8000_0000]]
        );
        assert_eq!(
            w.calls_to(process_slot(PROCESS_GUARD_START_TRAVEL_SLOT)),
            [vec![w.process, w.actor, 0x5500]]
        );
        // A package target radius of 60 covers 50: near, so only 008b3b90.
        let w = guard(&[(PACKAGE_GET_TARGET, 0x5100), (PACKAGE_TARGET_GET_VALUE, 60)]);
        assert!(w
            .calls_to(process_slot(PROCESS_GUARD_START_TRAVEL_SLOT))
            .is_empty());
        assert_eq!(w.calls_to(ACTOR_TEST_008B3B90).len(), 1);
        // The run-once guard package's data; 008b3b90 starts the travel.
        let w = guard(&[
            (process_slot(GET_RUN_ONCE_PACKAGE_SLOT), 0x7700),
            (ACTOR_IS_PATHING_COMPLETE, 0),
            (ACTOR_TEST_008B3B90, 1),
        ]);
        assert_eq!(
            w.calls_to(process_slot(PROCESS_GUARD_START_TRAVEL_SLOT)),
            [vec![w.process, w.actor, 0x5600]]
        );
        // Already the target: no guard is added.
        let mut w = guard(&[]);
        let guarded = w.calls_to(process_slot(SET_TARGET_SLOT))[0][1];
        w.on(process_slot(GET_TARGET_SLOT), guarded);
        w.on(PACKAGE_COMPUTE_INITIAL_TARGET, guarded);
        w.clear();
        w.e.call(0x0091_2b00, &args![w.process, w.actor]);
        assert!(w.calls_to(EXTRA_ADD_GUARD).is_empty());
        // The actor's state 4: the data is reset and its slot +0x418 runs.
        let w = guard(&[(actor_slot(ACTOR_SLOT_214), 4)]);
        assert_eq!(w.calls_to(SET_INSTANCE_DATA_006ECD40), [vec![0x5500, 0]]);
        assert_eq!(w.calls_to(actor_slot(ACTOR_SLOT_418)).len(), 1);
        assert!(w.calls_to(actor_slot(ACTOR_SLOT_4B4)).is_empty());
        // Busy (slot +0x4b4), or no data.
        let w = guard(&[(actor_slot(ACTOR_SLOT_4B4), 1)]);
        assert!(w.calls_to(GET_0055B980).is_empty());
        let w = guard(&[(PROCESS_GET_INSTANCE_DATA, 0)]);
        assert_eq!(
            w.calls_to(process_slot(SET_PROCEDURE_INDEX_RUNNING_SLOT)),
            [vec![w.process, u32::MAX]]
        );
        // A located package: near is its slot +0x13c.
        let w = guard(&[(GET_0055B980, 1), (package_slot(PACKAGE_SLOT_13C), 1)]);
        assert!(w
            .calls_to(process_slot(PROCESS_GUARD_START_TRAVEL_SLOT))
            .is_empty());
        assert_eq!(w.calls_to(PACKAGE_GET_00676280).len(), 1);
    }
}

#[cfg(test)]
mod b003_tests_5 {
    use super::b003_tests::*;
    use super::*;

    fn location(w: &mut World, kind: u32, generic: u32, empty: bool) {
        w.on(PACKAGE_GET_SECOND_LOCATION, 0x5200);
        w.on(GET_0055B980, 0x5100);
        w.on(PACKAGE_LOCATION_GET_TYPE, kind);
        w.on(LOCATION_GET_REFERENCE, 0x6100);
        w.on(ACTOR_GET_00569B80, 0x6200);
        w.on(LOCATION_IS_GENERIC, generic);
        w.on(PACKAGE_TEST_0067EFD0, 1);
        let list = w.process + 0x6c;
        let log = w.log.clone();
        w.e.register_double(LIST_IS_EMPTY, move |e, a| {
            log.borrow_mut().push((LIST_IS_EMPTY, a.to_vec()));
            ret(u32::from(a[0] == list && (empty || e.mem.u32(list) == 0)))
        });
        w.on(ACTOR_GET_008D6F30, 0x3600);
        let point = w.e.mem.alloc(12);
        w.e.mem.set_f32(point, 1.0);
        w.on(actor_slot(ACTOR_GET_LOCATION_SLOT), point);
        w.on(LOCATION_GET_OBJECT, 0x7100);
        w.on(LOCATION_GET_OBJECT_TYPE, 0x2a);
        w.on(PACKAGE_GET_ACQUIRE_RADIUS, 0);
        let setting = w.e.mem.alloc(4);
        w.e.mem.set_f32(setting, 512.0);
        w.on(SETTING_VALUE_00403E20, setting);
        w.e.map(DATA_HANDLER, 4);
        w.e.mem.set_u32(DATA_HANDLER, 0x0d0d_0000);
        w.on(ENUM_REFERENCES_CLOSE_TO_POINT, 0);
        w.e.register(LIST_HEAD_ITEM, |_, a| ret(a[0]));
        let log = w.log.clone();
        w.e.register_double(LIST_ADD_ITEM, move |e, a| {
            log.borrow_mut()
                .push((LIST_ADD_ITEM, vec![a[0], e.mem.u32(a[1])]));
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            ret(0)
        });
        let log = w.log.clone();
        w.e.register_double(LIST_REMOVE_HEAD, move |e, a| {
            log.borrow_mut().push((LIST_REMOVE_HEAD, a.to_vec()));
            e.mem.set_u32(a[0], 0);
            ret(0)
        });
    }

    #[test]
    fn location_for_package_by_type() {
        let mut w = world();
        location(&mut w, 0, 0, false);
        let result =
            w.e.call(0x0090_89c0, &args![w.process, w.actor, w.package, 0u32])
                .u32();
        assert_eq!(result, 0x6100);
        assert_eq!(w.calls_to(GET_0055B980), [vec![w.package]]);
        let result =
            w.e.call(0x0090_89c0, &args![w.process, w.actor, w.package, 1u32])
                .u32();
        assert_eq!(result, 0x6100);
        assert_eq!(w.calls_to(PACKAGE_GET_SECOND_LOCATION), [vec![w.package]]);
        let mut w = world();
        location(&mut w, 6, 0, false);
        let result =
            w.e.call(0x0090_89c0, &args![w.process, w.actor, w.package, 0u32])
                .u32();
        assert_eq!(result, 0x6200);
        assert!(w.calls_to(LOCATION_GET_REFERENCE).is_empty());
        let mut w = world();
        location(&mut w, 0, 0, false);
        w.on(GET_0055B980, 0);
        let result =
            w.e.call(0x0090_89c0, &args![w.process, w.actor, w.package, 0u32])
                .u32();
        assert_eq!(result, 0);
    }

    #[test]
    fn generic_location_searches_and_takes_the_first_candidate() {
        let mut w = world();
        location(&mut w, 4, 1, true);
        let p = w.process;
        // TempLocList holds one candidate the search left.
        w.e.mem.set_u32(p + 0x7c, 0x8800);
        let result =
            w.e.call(0x0090_89c0, &args![p, w.actor, w.package, 0u32])
                .u32();
        let enumerate = w.calls_to(ENUM_REFERENCES_CLOSE_TO_POINT);
        assert_eq!(enumerate.len(), 1);
        let words = &enumerate[0];
        assert_eq!(words[0], 0x0d0d_0000);
        assert_eq!(words[1], 0x3600);
        assert_eq!(w.e.mem.f32(words[2]), 1.0);
        assert_eq!(words[3], 512.0f32.to_bits());
        assert_eq!(words[5], 512.0f32.to_bits());
        assert_eq!(words[6], COLLECT_LOCATION_CALLBACK);
        assert_eq!(words[7], w.actor);
        assert_eq!(w.calls_to(LIST_ADD_ITEM), [vec![p + 0x6c, 0x8800]]);
        assert_eq!(w.calls_to(LIST_REMOVE_HEAD), [vec![p + 0x7c]]);
        // The list stays "empty" for this double, so nothing is taken.
        assert_eq!(result, 0);
        assert_eq!(w.e.mem.u32(p + 0x88), 0);
        // A filled list: its head is taken without a search.
        let mut w = world();
        location(&mut w, 5, 1, false);
        let p = w.process;
        w.e.mem.set_u32(p + 0x6c, 0x8900);
        let result =
            w.e.call(0x0090_89c0, &args![p, w.actor, w.package, 0u32])
                .u32();
        assert_eq!(result, 0x8900);
        assert!(w.calls_to(ENUM_REFERENCES_CLOSE_TO_POINT).is_empty());
        assert_eq!(w.calls_to(LIST_REMOVE_HEAD), [vec![p + 0x6c]]);
    }

    /// A process whose running procedure is `procedure` (type 3, index 1).
    fn procedure(w: &mut World, procedure: u32) {
        w.on(PACKAGE_GET_PROCEDURE_TYPE, 3);
        w.on(process_slot(GET_PROCEDURE_INDEX_RUNNING_SLOT), 1);
        w.e.map(PROCEDURE_LISTS, 0x40);
        let list = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(PROCEDURE_LISTS + 12, list);
        w.e.mem.set_u32(list + 4, procedure);
    }

    fn destination(procedure_number: u32, answers: &[(u32, u32)]) -> (World, u32, u32, u32) {
        let mut w = world();
        procedure(&mut w, procedure_number);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        let actor = w.actor;
        let target = w.object();
        let log = w.log.clone();
        w.e.register_double(ACTOR_GET_008D6F30, move |_, a| {
            log.borrow_mut().push((ACTOR_GET_008D6F30, a.to_vec()));
            ret(if a[0] == actor { 0xa000 } else { a[0] + 1 })
        });
        let log = w.log.clone();
        w.e.register_double(REFERENCE_GET_WORLD_SPACE, move |_, a| {
            log.borrow_mut()
                .push((REFERENCE_GET_WORLD_SPACE, a.to_vec()));
            ret(if a[0] == actor { 0xb000 } else { a[0] + 2 })
        });
        w.on(GET_0055B980, 1);
        w.on(PACKAGE_GET_LOCATION_CELL, 0xc000);
        w.on(PACKAGE_GET_LOCATION_WORLD, 0xd000);
        w.on(PROCESS_GET_INSTANCE_DATA, 0x5500);
        w.on(INSTANCE_DATA_GET_REFERENCE, 0x7000);
        w.on(PACKAGE_TEST_00901C50, 0);
        w.on(process_slot(GET_TARGET_SLOT), target);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        let cell = w.e.call(0x0090_f0e0, &args![w.process, w.actor]).u32();
        let space = w.e.call(0x0090_f3f0, &args![w.process, w.actor]).u32();
        (w, cell, space, target)
    }

    #[test]
    fn destination_cell_and_world_space_by_procedure() {
        // Package location.
        for number in [0u32, 4, 5, 7, 50] {
            let (_, cell, space, _) = destination(number, &[]);
            assert_eq!((cell, space), (0xc000, 0xd000), "procedure {number}");
        }
        let (_, cell, space, _) = destination(0, &[(GET_0055B980, 0)]);
        assert_eq!((cell, space), (0xa000, 0xb000));
        // Target or actor.
        for number in [1u32, 2, 3, 6, 8, 13, 14, 15, 30] {
            let (_, cell, space, _) = destination(number, &[]);
            assert_eq!((cell, space), (0xa000, 0xb000), "procedure {number}");
        }
        // The actor's, the instance data's, the cascade and the rest.
        let (_, cell, space, _) = destination(27, &[]);
        assert_eq!((cell, space), (0xa000, 0xb000));
        let (_, cell, space, _) = destination(42, &[]);
        assert_eq!((cell, space), (0x7001, 0x7002));
        let (_, cell, space, _) = destination(42, &[(INSTANCE_DATA_GET_REFERENCE, 0)]);
        assert_eq!((cell, space), (0xa000, 0xb000));
        let (w, cell, space, target) = destination(47, &[]);
        assert_eq!(w.calls_to(process_slot(GET_TARGET_SLOT)).len(), 4);
        assert_eq!((cell, space), (target + 1, target + 2));
        let (_, cell, space, _) = destination(47, &[(PACKAGE_TEST_00901C50, 1)]);
        assert_eq!((cell, space), (0xc000, 0xd000));
        let (_, cell, space, _) = destination(47, &[(process_slot(GET_TARGET_SLOT), 0)]);
        assert_eq!((cell, space), (0xa000, 0xb000));
        let (_, cell, space, _) = destination(9, &[(GET_0055B980, 0)]);
        assert_eq!((cell, space), (0xa000, 0xb000));
        // No package: the actor's; no actor either: 0.
        let (_, cell, space, _) = destination(0, &[(process_slot(GET_CURRENT_PACKAGE_SLOT), 0)]);
        assert_eq!((cell, space), (0xa000, 0xb000));
        let mut w = world();
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), 0);
        assert_eq!(w.e.call(0x0090_f0e0, &args![w.process, 0u32]).u32(), 0);
        assert_eq!(w.e.call(0x0090_f3f0, &args![w.process, 0u32]).u32(), 0);
    }

    #[test]
    fn destination_cell_uses_the_target_when_set() {
        let mut w = world();
        procedure(&mut w, 1);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.e.mem.set_u32(w.process + 0x40, 0x9000);
        let actor = w.actor;
        w.e.register(ACTOR_GET_008D6F30, |_, a| ret(a[0] + 1));
        w.e.register(REFERENCE_GET_WORLD_SPACE, |_, a| ret(a[0] + 2));
        assert_eq!(
            w.e.call(0x0090_f0e0, &args![w.process, actor]).u32(),
            0x9001
        );
        assert_eq!(
            w.e.call(0x0090_f3f0, &args![w.process, actor]).u32(),
            0x9002
        );
    }

    /// `GetCurrentDestinationCoordinate` for `procedure_number`; points are
    /// returned as their first word.
    fn coordinate(procedure_number: u32, flag: u8, answers: &[(u32, u32)]) -> (World, u32) {
        coordinate_with(procedure_number, flag, false, answers)
    }

    fn coordinate_with(
        procedure_number: u32,
        flag: u8,
        with_target: bool,
        answers: &[(u32, u32)],
    ) -> (World, u32) {
        let mut w = world();
        if with_target {
            let target = w.object();
            w.e.mem.set_u32(w.process + 0x40, target);
        }
        procedure(&mut w, procedure_number);
        w.e.map(DESTINATION_DEFAULT_POINT, 12);
        w.e.mem.set_u32(DESTINATION_DEFAULT_POINT, 0x11);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(PACKAGE_IS_INTERRUPT, 0);
        let point = |e: &mut Engine, first: u32| {
            let p = e.mem.alloc(12);
            e.mem.set_u32(p, first);
            p
        };
        let actor_point = point(&mut w.e, 0xa1);
        let object_point = point(&mut w.e, 0xb1);
        let coord_point = point(&mut w.e, 0xc1);
        let start_point = point(&mut w.e, 0xd1);
        w.on(actor_slot(ACTOR_GET_LOCATION_SLOT), actor_point);
        w.on(object_slot(ACTOR_GET_LOCATION_SLOT), object_point);
        w.on(GET_0055B980, 1);
        w.on(PACKAGE_GET_LOCATION_COORD, coord_point);
        w.on(object_slot(ACTOR_SLOT_100), 1);
        w.on(MOBILE_GET_CURRENT_PACKAGE, 0x5300);
        w.on(PACKAGE_TEST_00901C50, 0);
        let reference = w.object();
        w.on(PACKAGE_GET_TARGET, 0x5100);
        w.on(PACKAGE_TARGET_GET_REFERENCE, reference);
        w.on(PACKAGE_GET_TYPE, 6);
        w.on(REFERENCE_GET_START_LOCATION_COORD, start_point);
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(EXTRA_GET_PACKAGE, w.package);
        w.on(EXTRA_GET_PACKAGE_INDEX, 1);
        w.on(EXTRA_GET_PACKAGE_TARGET, 0);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        let result = w.e.mem.alloc(12);
        let returned =
            w.e.call(
                0x0090_ec70,
                &args![w.process, result, w.actor, u32::from(flag)],
            )
            .u32();
        assert_eq!(returned, result);
        let first = w.e.mem.u32(result);
        (w, first)
    }

    #[test]
    fn destination_coordinate_by_procedure() {
        assert_eq!(coordinate(0, 0, &[]).1, 0xc1);
        assert_eq!(coordinate(0, 0, &[(GET_0055B980, 0)]).1, 0xa1);
        assert_eq!(coordinate(9, 0, &[]).1, 0xa1);
        assert_eq!(coordinate(27, 0, &[]).1, 0xa1);
        assert_eq!(coordinate(47, 0, &[]).1, 0xb1);
        assert_eq!(coordinate(47, 0, &[(PACKAGE_TEST_00901C50, 1)]).1, 0xc1);
        assert_eq!(coordinate(47, 0, &[(PACKAGE_GET_TARGET, 0)]).1, 0xa1);
        assert_eq!(coordinate(54, 0, &[]).1, 0xc1);
        assert_eq!(coordinate(54, 0, &[(GET_0055B980, 0)]).1, 0xd1);
        assert_eq!(coordinate(54, 0, &[(PACKAGE_GET_TYPE, 5)]).1, 0xa1);
        assert_eq!(coordinate(54, 1, &[]).1, 0xa1);
        assert_eq!(coordinate(10, 0, &[]).1, 0x11);
        assert_eq!(
            coordinate(0, 0, &[(process_slot(GET_CURRENT_PACKAGE_SLOT), 0)]).1,
            0x11
        );
        assert_eq!(
            coordinate(0, 0, &[(PACKAGE_GET_PROCEDURE_TYPE, u32::MAX)]).1,
            0x11
        );
    }

    #[test]
    fn destination_coordinate_with_targets_and_interrupts() {
        // A target for the target-or-actor procedures and procedure 27.
        assert_eq!(coordinate_with(1, 0, true, &[]).1, 0xb1);
        assert_eq!(coordinate_with(54, 1, true, &[]).1, 0xb1);
        assert_eq!(coordinate_with(27, 0, true, &[]).1, 0xc1);
        assert_eq!(
            coordinate_with(27, 0, true, &[(object_slot(ACTOR_SLOT_100), 0)]).1,
            0xa1
        );
        assert_eq!(
            coordinate_with(27, 0, true, &[(MOBILE_GET_CURRENT_PACKAGE, 0)]).1,
            0xa1
        );
        // An interrupt package takes the extra data's package state.
        let (w, first) = coordinate(
            1,
            0,
            &[(PACKAGE_IS_INTERRUPT, 1), (EXTRA_GET_PACKAGE_TARGET, 0)],
        );
        assert_eq!(first, 0xa1);
        assert_eq!(w.calls_to(EXTRA_GET_PACKAGE).len(), 1);
        assert_eq!(w.calls_to(EXTRA_GET_PACKAGE_INDEX).len(), 1);
        // With flag set the extra data is not asked.
        let (w, _) = coordinate(1, 1, &[(PACKAGE_IS_INTERRUPT, 1)]);
        assert!(w.calls_to(EXTRA_GET_PACKAGE).is_empty());
    }
}

#[cfg(test)]
mod b003_tests_6 {
    use super::b003_tests::*;
    use super::*;

    fn constants(w: &mut World) {
        for (address, value) in [
            (MINUS_ONE, -1.0f32),
            (ONE_HOUR_SECONDS, 3600.0),
            (ACTIVATE_WAIT, 300.0),
            (SEARCH_RADIUS_MAX, f32::MAX),
        ] {
            w.e.map(address, 4);
            w.e.mem.set_f32(address, value);
        }
        for (address, value) in [(SECONDS_PER_HOUR, 3600.0f64), (HOURS_PER_DAY, 24.0)] {
            w.e.map(address, 8);
            w.e.mem.set_f64(address, value);
        }
        for address in [PLAYER, ANGLED_FORM, ACTIVATE_DONE_FORM, DATA_HANDLER] {
            w.e.map(address, 4);
        }
        w.e.mem.set_u32(PLAYER, 0x0909_0000);
        w.e.mem.set_u32(DATA_HANDLER, 0x0d0d_0000);
        w.on(LOOKUP_FORM_BY_ID, 0x4400);
        w.answer(CALENDAR_GET_HOUR, st0(10.5));
        w.answer(PROCESS_GET_HOUR, st0(10.0));
        w.answer(GLOBAL_VALUE, st0(30.0));
    }

    fn steps(w: &World) -> Vec<u32> {
        w.calls_to(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT))
            .iter()
            .map(|words| words[2])
            .collect()
    }

    fn acquire(target: bool, answers: &[(u32, u32)]) -> World {
        let mut w = world();
        constants(&mut w);
        if target {
            let target = w.object();
            w.e.mem.set_u32(w.process + 0x40, target);
        }
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(process_slot(SET_TARGET_FOR_PACKAGE_SLOT), 0);
        w.on(PACKAGE_GET_TARGET, 0x5100);
        w.on(PACKAGE_GET_TYPE, 1);
        w.on(REFERENCE_GET_BASE_FORM, 0x3300);
        w.on(FORM_GET_TYPE, 0x15);
        w.on(DYNAMIC_CAST, 0x3400);
        w.on(LIGHT_TEST_0046F070, 0);
        w.on(PACKAGE_TARGET_GET_REFERENCE, 0x6000);
        w.on(PACKAGE_TEST_0067EFD0, 0);
        w.on(PACKAGE_GET_INITIAL_TARGET_COUNT, 3);
        w.on(process_slot(CHECK_IF_HAS_OBJECT_SLOT), 1);
        w.on(package_slot(PACKAGE_SLOT_13C), 1);
        w.on(process_slot(SET_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        w.on(PACKAGE_GET_PROCEDURE_TYPE, 0);
        w.on(PACKAGE_GET_LIST_0041D8A0, 0x7700);
        w.on(LIST_NEXT_NODE, 0);
        w.on(process_slot(PROCESS_ESCORT_SLOT), 0);
        w.on(MOBILE_GET_PROCESS_TYPE, 0);
        w.on(process_slot(PROCESS_ACTIVATE_ONE_HOUR_SLOT), 0);
        w.on(process_slot(PROCESS_ACTIVATE_SLOT), 0);
        w.on(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        w.e.call(0x0090_8f70, &args![w.process, w.actor]);
        w
    }

    #[test]
    fn acquire_by_base_form_type() {
        // Type 8 packages: form type 0x15 finishes, 0x1e asks the light.
        assert_eq!(steps(&acquire(true, &[(PACKAGE_GET_TYPE, 8)])), [1]);
        let w = acquire(true, &[(PACKAGE_GET_TYPE, 0x10), (FORM_GET_TYPE, 0x1e)]);
        assert_eq!(steps(&w), [1]);
        assert_eq!(
            w.calls_to(DYNAMIC_CAST),
            [vec![0x3300, 0, BOUND_OBJECT_TYPE, LIGHT_TYPE, 0]]
        );
        let w = acquire(
            true,
            &[
                (PACKAGE_GET_TYPE, 8),
                (FORM_GET_TYPE, 0x1e),
                (LIGHT_TEST_0046F070, 1),
            ],
        );
        assert!(steps(&w).is_empty());
        assert!(w
            .calls_to(process_slot(CHECK_IF_HAS_OBJECT_SLOT))
            .is_empty());
    }

    #[test]
    fn acquire_with_the_object_steps_and_escorts() {
        let w = acquire(true, &[]);
        assert_eq!(
            w.calls_to(process_slot(CHECK_IF_HAS_OBJECT_SLOT)),
            [vec![w.process, w.actor, 3]]
        );
        assert_eq!(steps(&w), [1]);
        // Procedure type 0x1a without a second node: 2.
        assert_eq!(
            steps(&acquire(true, &[(PACKAGE_GET_PROCEDURE_TYPE, 0x1a)])),
            [2]
        );
        // A type 3 package refusing: the index goes back to 0.
        let w = acquire(
            true,
            &[(PACKAGE_GET_TYPE, 3), (package_slot(PACKAGE_SLOT_13C), 0)],
        );
        assert!(steps(&w).is_empty());
        assert_eq!(
            w.calls_to(process_slot(SET_PROCEDURE_INDEX_RUNNING_SLOT)),
            [vec![w.process, 0]]
        );
        // Escort packages escort.
        let w = acquire(true, &[(PACKAGE_GET_TYPE, 2)]);
        assert_eq!(
            w.calls_to(process_slot(PROCESS_ESCORT_SLOT)),
            [vec![w.process, w.actor]]
        );
    }

    #[test]
    fn acquire_without_the_object_activates() {
        let w = acquire(true, &[(process_slot(CHECK_IF_HAS_OBJECT_SLOT), 0)]);
        assert_eq!(
            w.calls_to(process_slot(PROCESS_ACTIVATE_ONE_HOUR_SLOT)),
            [vec![w.process, w.actor, 3600.0f32.to_bits()]]
        );
        let w = acquire(
            true,
            &[
                (process_slot(CHECK_IF_HAS_OBJECT_SLOT), 0),
                (MOBILE_GET_PROCESS_TYPE, 2),
            ],
        );
        assert_eq!(
            w.calls_to(process_slot(PROCESS_ACTIVATE_SLOT)),
            [vec![w.process, w.actor, 1]]
        );
        // Without a target reference or a 0067efd0 package: a step.
        let w = acquire(false, &[]);
        assert_eq!(steps(&w), [1]);
        assert!(w
            .calls_to(process_slot(CHECK_IF_HAS_OBJECT_SLOT))
            .is_empty());
        let w = acquire(false, &[(PACKAGE_GET_PROCEDURE_TYPE, 0x1a)]);
        assert_eq!(steps(&w), [2]);
        // A 0067efd0 package without a target still checks for the object.
        let w = acquire(
            false,
            &[(PACKAGE_TEST_0067EFD0, 1), (PACKAGE_GET_TARGET, 0)],
        );
        assert_eq!(
            w.calls_to(process_slot(CHECK_IF_HAS_OBJECT_SLOT)),
            [vec![w.process, w.actor, 1]]
        );
    }

    fn activate(
        acquire: u8,
        answers: &[(u32, u32)],
        setup: impl FnOnce(&mut World),
    ) -> (World, bool) {
        let mut w = world();
        constants(&mut w);
        let target = w.object();
        w.e.mem.set_u32(w.process + 0x40, target);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(REFERENCE_TEST_00440D80, 0);
        w.on(process_slot(SET_TARGET_FOR_PACKAGE_SLOT), 0);
        w.on(process_slot(ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT), 0);
        w.on(PACKAGE_GET_TARGET, 0x5100);
        w.on(PACKAGE_TARGET_GET_TYPE, 1);
        w.on(PACKAGE_GET_LOCATION_REFERENCE, 0);
        w.on(PACKAGE_GET_TYPE, 1);
        w.answer(PACKAGE_RADIUS_ACTOR_TO_TARGET, st0(100.0));
        w.answer(REFERENCE_GET_DISTANCE, st0(50.0));
        w.on(ACTOR_IS_PATHING_COMPLETE, 1);
        w.on(MOBILE_GET_PROCESS_TYPE, 0);
        w.on(ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE, 1);
        w.on(package_slot(PACKAGE_SLOT_144), 1);
        w.on(object_slot(ACTOR_SLOT_100), 0);
        w.on(object_slot(ACTOR_SLOT_22C), 0);
        w.on(object_slot(REFERENCE_SLOT_218), 0);
        w.on(REFERENCE_ACTIVATE, 0);
        w.on(DELETE_EMPTY_DESTRUCTOR, 0);
        w.on(process_slot(SET_TARGET_SLOT), 0);
        w.on(REFERENCE_RUN_SCRIPT, 0);
        w.on(PLAYER_IS_SLEEPING_OR_RESTING, 0);
        w.on(PLAYER_005C1A00, 0);
        w.on(process_slot(ENTER_COMBAT_SLOT), 0);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        setup(&mut w);
        let result =
            w.e.call(0x0090_d620, &args![w.process, w.actor, u32::from(acquire)])
                .bool();
        (w, result)
    }

    #[test]
    fn activate_without_acquire() {
        // A non-actor target is activated and its script runs.
        let (w, result) = activate(0, &[], |_| {});
        assert!(result);
        let target = w.calls_to(REFERENCE_RUN_SCRIPT)[0][0];
        assert_eq!(
            w.calls_to(REFERENCE_ACTIVATE),
            [vec![target, w.actor, 0, 0, 1]]
        );
        // The package does not finish: false.
        let (w, result) = activate(0, &[(package_slot(PACKAGE_SLOT_144), 0)], |_| {});
        assert!(!result);
        assert!(w.calls_to(REFERENCE_ACTIVATE).is_empty());
        // An actor target: true without activating; type 2 steps its process.
        let (w, result) = activate(0, &[(object_slot(ACTOR_SLOT_100), 1)], |_| {});
        assert!(result);
        assert!(w.calls_to(REFERENCE_RUN_SCRIPT).is_empty());
        let (w, result) = activate(
            0,
            &[(object_slot(ACTOR_SLOT_100), 1), (PACKAGE_GET_TYPE, 2)],
            |w| {
                let process = w.process;
                w.on(ACTOR_GET_PROCESS, process);
            },
        );
        assert!(result);
        assert_eq!(steps(&w), [2]);
        // No target found: false (a step of 2 with acquire).
        let (w, result) = activate(1, &[], |w| w.e.mem.set_u32(w.process + 0x40, 0));
        assert!(!result);
        assert_eq!(steps(&w), [2]);
    }

    #[test]
    fn activate_paths_to_a_far_target() {
        let (w, result) = activate(0, &[], |w| {
            w.answer(REFERENCE_GET_DISTANCE, st0(150.0));
        });
        assert!(result);
        let target = w.calls_to(ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE)[0][1];
        assert_eq!(
            w.calls_to(ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE),
            [vec![w.actor, target, (-1.0f32).to_bits(), 0]]
        );
        let (_, result) = activate(0, &[(ACTOR_SET_PATHFINDING_GOAL_TO_REFERENCE, 0)], |w| {
            w.answer(REFERENCE_GET_DISTANCE, st0(150.0));
        });
        assert!(!result);
        // The process type changed: true at once.
        let (w, result) = activate(0, &[], |w| {
            w.answer(REFERENCE_GET_DISTANCE, st0(150.0));
            w.on_seq(MOBILE_GET_PROCESS_TYPE, vec![0, 1]);
        });
        assert!(result);
        assert!(w.calls_to(package_slot(PACKAGE_SLOT_144)).is_empty());
    }

    #[test]
    fn activate_for_an_acquire_object() {
        let (w, result) = activate(1, &[], |w| {
            let p = w.process;
            let object = w.e.mem.alloc(0x20);
            let reference = w.object();
            w.e.mem.set_u32(object, reference);
            w.e.mem.set_u32(object + 4, 7);
            w.e.mem.set_u32(object + 0xc, 2);
            w.e.mem.set_u32(p + 0x64, object);
            w.e.mem.set_u32(p + 0x58, 2);
        });
        assert!(result);
        let target = w.calls_to(REFERENCE_RUN_SCRIPT)[0][0];
        assert_eq!(
            w.calls_to(REFERENCE_ACTIVATE),
            [vec![target, w.actor, 1, 7, 2]]
        );
        assert_eq!(w.e.mem.u32(w.process + 0x64), 0);
        assert_eq!(w.e.mem.u32(w.process + 0x58), 0);
        // Out of items, then the activation's own step: 2 and 2.
        assert_eq!(steps(&w), [2, 2]);
        assert_eq!(
            w.calls_to(process_slot(SET_TARGET_SLOT)),
            [vec![w.process, 0]]
        );
    }

    #[test]
    fn activate_an_actor_target_for_acquire() {
        // A resting player is woken.
        let (w, result) = activate(
            1,
            &[
                (object_slot(ACTOR_SLOT_100), 1),
                (PLAYER_IS_SLEEPING_OR_RESTING, 1),
            ],
            |w| {
                let target = w.e.mem.alloc(0x220);
                w.e.mem.set_u32(target, OBJECT_VTABLE);
                w.e.mem.set_u32(w.process + 0x40, target);
                w.e.mem.set_u32(PLAYER, target);
            },
        );
        assert!(!result);
        let player = w.e.mem.u32(PLAYER);
        assert_eq!(w.calls_to(PLAYER_005C1A00), [vec![player, 0, 0]]);
        // No acquire object and not the player: a step of 2, false.
        let (w, result) = activate(1, &[(object_slot(ACTOR_SLOT_100), 1)], |_| {});
        assert!(!result);
        assert_eq!(steps(&w), [2]);
        // An acquire object with byte +9: combat.
        let (w, result) = activate(1, &[(object_slot(ACTOR_SLOT_100), 1)], |w| {
            let object = w.e.mem.alloc(0x20);
            let reference = w.object();
            w.e.mem.set_u32(object, reference);
            w.e.mem.set_u8(object + 9, 1);
            w.e.mem.set_u32(w.process + 0x64, object);
            w.e.mem.set_u32(w.process + 0x58, 5);
        });
        assert!(result);
        let combat = w.calls_to(process_slot(ENTER_COMBAT_SLOT));
        assert_eq!(combat.len(), 1);
        assert_eq!(combat[0][3..], [0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0]);
    }

    fn target_for(answers: &[(u32, u32)], setup: impl FnOnce(&mut World)) -> (World, u32) {
        let mut w = world();
        constants(&mut w);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(PACKAGE_GET_TARGET, 0x5100);
        w.on(PACKAGE_TARGET_GET_TYPE, 0);
        let reference = w.object();
        w.on(PACKAGE_TARGET_GET_REFERENCE, reference);
        w.on(object_slot(ACTOR_SLOT_22C), 0);
        w.on(PACKAGE_SET_NEVER_RUN, 0);
        w.on(PACKAGE_GET_TYPE, 3);
        w.on(PACKAGE_IS_TARGET_AN_ACTOR, 0);
        w.on(PACKAGE_GET_ACTOR_TARGET, 0x6600);
        w.on(PROCESS_GET_INSTANCE_DATA, 0);
        w.on(PATROL_GET_CURRENT_REFERENCE, 0x6700);
        w.on(PATROL_INITIAL_REFERENCE, 0x6800);
        w.on(REFERENCE_TEST_00440D80, 0);
        w.on(object_slot(ACTOR_SLOT_100), 0);
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(EXTRA_GET_REFERENCE_POINTER, 0x6900);
        w.on(ACTOR_GET_00569B80, 0x6a00);
        w.on(PACKAGE_TEST_0067EFD0, 1);
        setup(&mut w);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        let result =
            w.e.call(0x0090_82d0, &args![w.process, w.actor, w.package])
                .u32();
        (w, result)
    }

    #[test]
    fn target_for_package_from_a_reference_target() {
        let (w, result) = target_for(&[], |_| {});
        let reference = w.calls_to(object_slot(ACTOR_SLOT_22C))[0][0];
        assert_eq!(result, reference);
        // 00440d80 holds: the reference pointer replaces it.
        let (_, result) = target_for(&[(REFERENCE_TEST_00440D80, 1)], |_| {});
        assert_eq!(result, 0x6900);
        // The reference's slot +0x22c (1): never run, none.
        let (w, result) = target_for(&[(object_slot(ACTOR_SLOT_22C), 1)], |_| {});
        assert_eq!(result, 0);
        assert_eq!(
            w.calls_to(PACKAGE_SET_NEVER_RUN),
            [vec![w.package, reference_of(&w), 1]]
        );
        // Actor and patrol types.
        let (_, result) = target_for(
            &[(PACKAGE_GET_TYPE, 1), (PACKAGE_IS_TARGET_AN_ACTOR, 1)],
            |_| {},
        );
        assert_eq!(result, 0x6600);
        let (_, result) = target_for(&[(PACKAGE_GET_TYPE, 0xd)], |_| {});
        assert_eq!(result, 0x6800);
        let (_, result) = target_for(
            &[(PACKAGE_GET_TYPE, 7), (PROCESS_GET_INSTANCE_DATA, 0x5500)],
            |w| w.e.mem.set_u32(w.process + 0x40, 0x7000),
        );
        assert_eq!(result, 0x6700);
        // Without a reference: none; without a package: none.
        let (_, result) = target_for(&[(PACKAGE_TARGET_GET_REFERENCE, 0)], |_| {});
        assert_eq!(result, 0);
        let mut w = world();
        assert_eq!(
            w.e.call(0x0090_82d0, &args![w.process, w.actor, 0u32])
                .u32(),
            0
        );
        // Package target type 3: the actor's 00569b80.
        let (_, result) = target_for(&[(PACKAGE_TARGET_GET_TYPE, 3)], |_| {});
        assert_eq!(result, 0x6a00);
    }

    fn reference_of(w: &World) -> u32 {
        w.calls_to(object_slot(ACTOR_SLOT_22C))[0][0]
    }

    #[test]
    fn target_for_package_takes_the_first_acquire_candidate() {
        // ObjectList holds an object of kind 1 whose reference is the target.
        let (w, result) = target_for(&[(PACKAGE_TARGET_GET_TYPE, 1)], |w| {
            let p = w.process;
            let object = w.e.mem.alloc(0x20);
            w.e.mem.set_u32(object, 0x7100);
            w.e.mem.set_u32(object + 0x18, 1);
            w.e.mem.set_u32(p + 0x5c, object);
            w.e.register(LIST_IS_EMPTY, |e, a| {
                ret(u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0))
            });
            w.e.register(LIST_HEAD_ITEM, |_, a| ret(a[0]));
            w.on(LIST_REMOVE_ITEM, 0);
        });
        assert_eq!(result, 0x7100);
        let p = w.process;
        let object = w.e.mem.u32(p + 0x64);
        assert_eq!(w.e.mem.u32(object), 0x7100);
        assert_eq!(w.calls_to(LIST_REMOVE_ITEM), [vec![p + 0x5c, p + 0x64]]);
    }

    #[test]
    fn target_for_package_searches_when_the_list_is_empty() {
        let (w, result) = target_for(&[(PACKAGE_TARGET_GET_TYPE, 2)], |w| {
            w.e.register(LIST_IS_EMPTY, |_, _| ret(1));
            w.on(ACTOR_GET_008D6F30, 0x3600);
            let point = w.e.mem.alloc(12);
            w.on(actor_slot(ACTOR_GET_LOCATION_SLOT), point);
            w.on(PACKAGE_TARGET_GET_OBJECT_TYPE, 0x2a);
            w.on(PACKAGE_GET_SEARCH_LOCATION, 0);
            w.on(PACKAGE_GET_ACQUIRE_RADIUS, 1);
            w.on(ENUM_REFERENCES_CLOSE_TO_POINT, 0);
            w.on(process_slot(EVALUATE_ORDER_ACQUIRE_LIST_SLOT), 0);
            w.on(PACKAGE_GET_INITIAL_TARGET_COUNT, 4);
        });
        assert_eq!(result, 0);
        let enumerate = w.calls_to(ENUM_REFERENCES_CLOSE_TO_POINT);
        assert_eq!(enumerate.len(), 1);
        assert_eq!(enumerate[0][1], 0x3600);
        assert_eq!(enumerate[0][6], COLLECT_TARGET_CALLBACK);
        assert_eq!(enumerate[0][7], w.actor);
        assert_eq!(
            w.calls_to(process_slot(EVALUATE_ORDER_ACQUIRE_LIST_SLOT)),
            [vec![w.process, w.actor]]
        );
        assert_eq!(w.e.mem.u32(w.process + 0x58), 4);
        assert_eq!(w.e.mem.u32(w.process + 0x8c), 0);
        // The search location's radius when it has one.
        let (w, _) = target_for(&[(PACKAGE_TARGET_GET_TYPE, 2)], |w| {
            w.e.register(LIST_IS_EMPTY, |_, _| ret(1));
            w.on(ACTOR_GET_008D6F30, 0x3600);
            let point = w.e.mem.alloc(12);
            w.on(actor_slot(ACTOR_GET_LOCATION_SLOT), point);
            w.on(PACKAGE_TARGET_GET_OBJECT_TYPE, 0x2a);
            w.on(PACKAGE_GET_SEARCH_LOCATION, 1);
            w.on(PACKAGE_GET_SEARCH_RADIUS, 250);
            w.on(PACKAGE_GET_SEARCH_CELL, 0x3700);
            let coord = w.e.mem.alloc(12);
            w.on(PACKAGE_GET_SEARCH_COORD, coord);
            w.on(ENUM_REFERENCES_CLOSE_TO_POINT, 0);
            w.on(process_slot(EVALUATE_ORDER_ACQUIRE_LIST_SLOT), 0);
            w.on(PACKAGE_GET_INITIAL_TARGET_COUNT, 4);
        });
        let enumerate = w.calls_to(ENUM_REFERENCES_CLOSE_TO_POINT);
        assert_eq!(enumerate[0][1], 0x3700);
        assert_eq!(enumerate[0][3], 250.0f32.to_bits());
        assert_eq!(enumerate[0][2], enumerate[0][4]);
    }
}

#[cfg(test)]
mod b003_tests_7 {
    use super::b003_tests::*;
    use super::*;

    fn constants(w: &mut World) {
        for (address, value) in [
            (MINUS_ONE, -1.0f32),
            (DOOR_SEARCH_RADIUS, 5000.0),
            (SEARCH_RADIUS_MAX, f32::MAX),
        ] {
            w.e.map(address, 4);
            w.e.mem.set_f32(address, value);
        }
        for (address, value) in [
            (SECONDS_PER_HOUR, 3600.0f64),
            (HOURS_PER_DAY, 24.0),
            (ZERO, 0.0),
        ] {
            w.e.map(address, 8);
            w.e.mem.set_f64(address, value);
        }
        for address in [PLAYER, DATA_HANDLER, ANGLED_FORM] {
            w.e.map(address, 4);
        }
        w.e.mem.set_u32(PLAYER, 0x0909_0000);
        w.e.mem.set_u32(DATA_HANDLER, 0x0d0d_0000);
        w.e.map(PROCEDURE_LISTS, 0x100);
    }

    /// The running procedure for procedure type `kind` at index 1.
    fn procedure(w: &mut World, kind: u32, number: u32) {
        let list = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(PROCEDURE_LISTS + 4 * kind, list);
        w.e.mem.set_u32(list + 4, number);
        w.on(PACKAGE_GET_PROCEDURE_TYPE, kind);
        w.on(process_slot(GET_PROCEDURE_INDEX_RUNNING_SLOT), 1);
    }

    fn missed(answers: &[(u32, u32)]) -> World {
        let mut w = world();
        constants(&mut w);
        let actor = w.actor;
        w.on(DYNAMIC_CAST, actor);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), 0);
        w.on(PACKAGE_TEST_0067EFD0, 0);
        w.on(PACKAGE_IS_SCRIPT, 0);
        w.on(PACKAGE_TEST_008B1FF0, 0);
        let block = w.e.mem.alloc(8);
        w.on(ALLOCATE, block);
        // The list: one node holding the missed package 0x7000.
        w.e.mem.set_u32(block, 0x7000);
        w.on(PACKAGE_LIST_CONSTRUCT, block);
        w.on(PROCESS_GET_DATE, 5);
        w.on(ACTOR_GET_MISSED_PACKS, 0);
        w.e.register(LIST_IS_EMPTY, |e, a| ret(u32::from(e.mem.u32(a[0]) == 0)));
        w.e.register(LIST_HEAD_ITEM, |_, a| ret(a[0]));
        w.e.register(WORD_04, |e, a| ret(e.mem.u32(a[0] + 4)));
        w.on(PACKAGE_GET_TARGET, 0);
        w.on(process_slot(SET_LOCKED_LOCATION_SLOT), 0);
        w.on(PACKAGE_GET_CONDITIONS, 0x7100);
        w.on(CONDITION_IS_TRUE, 1);
        w.on(process_slot(SET_CURRENT_PACKAGE_SLOT), 0);
        w.on(ACTOR_UNLOCK_LOCK_DOORS, 0);
        w.on(process_slot(SET_LOCATION_FOR_PACKAGE_SLOT), 0);
        w.on(PACKAGE_GET_TYPE, 5);
        let coord = w.e.mem.alloc(12);
        w.on(PACKAGE_GET_LOCATION_COORD, coord);
        w.on(REFERENCE_SET_LOCATION, 0);
        w.on(PACKAGE_GET_LOCATION_CELL, 0x3600);
        w.on(PACKAGE_GET_LOCATION_WORLD, 0x3700);
        w.on(GET_0055B980, 0x5500);
        w.on(PACKAGE_LOCATION_GET_TYPE, 1);
        w.on(PACKAGE_GET_ONCE_PER_DAY, 0);
        w.on(PACKAGE_TEST_00670EF0, 0);
        w.on(PACKAGE_TEST_00670F10, 0);
        w.on(LIST_CLEAR, 0);
        w.on(LIST_DELETE, 0);
        w.on(REFERENCE_SET_ANGLE_00575770, 0);
        w.on(REFERENCE_MOVE_TO_NEW_SPACE, 0);
        w.on(process_slot(CLEAR_CURRENT_PACKAGE_SLOT), 0);
        w.on(process_slot(CLEAR_CURRENT_DATA_SLOT), 0);
        let process = w.object();
        w.on(ACTOR_GET_PROCESS, process);
        w.on(object_slot(SET_LOCKED_LOCATION_SLOT), 0);
        w.on(process_slot(PROCESS_GUARD_SLOT), 0);
        w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
        w.on(SET_ACTION_FLAG, 0);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        w.e.call(0x0090_c3e0, &args![w.process, w.actor, 1u32]);
        w
    }

    #[test]
    fn missed_packages_move_the_actor_to_the_location() {
        let w = missed(&[]);
        assert_eq!(
            w.calls_to(ACTOR_GET_MISSED_PACKS),
            [vec![
                w.actor,
                w.calls_to(PACKAGE_LIST_CONSTRUCT)[0][0],
                5.0f32.to_bits()
            ]]
        );
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_PACKAGE_SLOT)),
            [vec![w.process, 0x7000, w.actor]]
        );
        assert_eq!(
            w.calls_to(SET_ACTION_FLAG),
            [vec![0x7000, 0x4444, 0x200], vec![0, 0x4444, 0x400]]
        );
        assert_eq!(
            w.calls_to(REFERENCE_SET_ANGLE_00575770),
            [vec![w.actor, f32::MAX.to_bits()]]
        );
        assert_eq!(
            w.calls_to(REFERENCE_MOVE_TO_NEW_SPACE),
            [vec![w.actor, 0x3600, 0x3700]]
        );
        assert_eq!(w.calls_to(ACTOR_UNLOCK_LOCK_DOORS).len(), 2);
        assert_eq!(w.calls_to(LIST_DELETE).len(), 1);
        // The end: the actor's process clears its locked location.
        assert_eq!(
            *w.addresses().last().unwrap(),
            object_slot(SET_LOCKED_LOCATION_SLOT)
        );
    }

    #[test]
    fn missed_packages_by_type_and_conditions() {
        // Guard packages run ProcessGuard and move nothing.
        let w = missed(&[(PACKAGE_GET_TYPE, 0xe)]);
        assert_eq!(
            w.calls_to(process_slot(PROCESS_GUARD_SLOT)),
            [vec![w.process, w.actor]]
        );
        assert!(w.calls_to(REFERENCE_MOVE_TO_NEW_SPACE).is_empty());
        // Failing conditions skip the package.
        let w = missed(&[(CONDITION_IS_TRUE, 0)]);
        assert!(w
            .calls_to(process_slot(SET_CURRENT_PACKAGE_SLOT))
            .is_empty());
        // A script package current: nothing but the locked location.
        let w = missed(&[
            (process_slot(GET_CURRENT_PACKAGE_SLOT), 0x7700),
            (PACKAGE_IS_SCRIPT, 1),
        ]);
        assert!(w.calls_to(ALLOCATE).is_empty());
        assert_eq!(w.calls_to(object_slot(SET_LOCKED_LOCATION_SLOT)).len(), 1);
        // Types 3 and 4 face the location reference only when there is one.
        let w = missed(&[(PACKAGE_GET_TYPE, 3), (LOCATION_GET_REFERENCE, 0)]);
        assert!(w.calls_to(REFERENCE_SET_ANGLE_00575770).is_empty());
        assert_eq!(w.calls_to(REFERENCE_MOVE_TO_NEW_SPACE).len(), 1);
    }

    fn check(
        flag: u8,
        answers: &[(u32, u32)],
        setup: impl FnOnce(&mut World, u32),
    ) -> (World, bool) {
        let mut w = world();
        constants(&mut w);
        w.answer(CALENDAR_GET_HOUR, st0(10.0));
        w.on(ACTOR_IS_ALARMED, 0);
        w.on(ACTOR_TEST_00493BB0, 0);
        // The current package lives in a word SetCurrentPackage writes.
        let current = w.e.mem.alloc(4);
        w.e.mem.set_u32(current, w.package);
        w.e.register_double(process_slot(GET_CURRENT_PACKAGE_SLOT), move |e, _| {
            ret(e.mem.u32(current))
        });
        w.on(PACKAGE_TEST_00670EF0, 0);
        w.on(PACKAGE_TEST_00670F10, 0);
        w.on(PACKAGE_TEST_005394A0, 0);
        w.on(PACKAGE_TEST_00670ED0, 0);
        let schedule = w.e.mem.alloc(8);
        w.on(PACKAGE_GET_SCHEDULE, schedule);
        w.e.register(WORD_04, |e, a| ret(e.mem.u32(a[0] + 4)));
        w.e.register(SCHEDULE_GET_HOUR, |e, a| ret(u32::from(e.mem.u8(a[0] + 3))));
        w.on(PACKAGE_TEST_0067EFD0, 0);
        w.on(PACKAGE_TEST_008B1FF0, 0);
        w.on(PACKAGE_IS_SCRIPT, 0);
        w.on(PACKAGE_GET_IS_CREATED, 0);
        w.on(PACKAGE_IS_INTERRUPT, 0);
        w.on(PACKAGE_TEST_008840F0, 0);
        w.on(ACTOR_GET_PACKAGE, w.package);
        let log = w.log.clone();
        w.e.register_double(process_slot(SET_CURRENT_PACKAGE_SLOT), move |e, a| {
            log.borrow_mut()
                .push((process_slot(SET_CURRENT_PACKAGE_SLOT), a.to_vec()));
            e.mem.set_u32(current, a[1]);
            ret(0)
        });
        for slot in [
            SETUP_NEW_PACKAGE_SLOT,
            CHECK_AND_START_COMBAT_SLOT,
            SET_TARGET_SLOT,
            SET_TARGET_ACTIVATED_SLOT,
            SET_LOCKED_LOCATION_SLOT,
            SET_CONTINUING_PACKAGE_SLOT,
            CLEAR_RUN_ONCE_PACKAGE_SLOT,
            SET_CURRENT_ACTION_COMPLETE_SLOT,
            START_NEW_PACKAGE_SLOT,
            SET_PACKAGE_DONE_ONCE_SLOT,
            CLEAR_CURRENT_DATA_SLOT,
            CLEAR_CURRENT_PACKAGE_SLOT,
        ] {
            w.on(process_slot(slot), 0);
        }
        w.on(process_slot(GET_RUN_ONCE_PACKAGE_SLOT), 0);
        w.on(ACTOR_CLEAR_LOOK_AT_TARGET, 0);
        w.on(ACTOR_UNLOCK_LOCK_DOORS, 0);
        w.on(actor_slot(ACTOR_GET_COMBAT_CONTROLLER_SLOT), 0);
        w.on(actor_slot(ACTOR_SLOT_2C8), 0);
        w.on(PACKAGE_IS_TARGET_AN_ACTOR, 0);
        let angle = w.object();
        w.on(ACTOR_GET_00430830, angle);
        w.on(actor_slot(ACTOR_GET_LOCATION_SLOT), 0x3800);
        w.on(ACTOR_GET_008D6F30, 0x3600);
        w.on(REFERENCE_GET_WORLD_SPACE, 0x3700);
        w.on(REFERENCE_SET_PACKAGE_START_LOCATION, 0);
        w.on(SET_FLAG_BITS, 0);
        w.e.register(LIST_IS_EMPTY, |_, _| ret(1));
        w.on(GET_0055B980, 0);
        for (address, value) in answers {
            w.on(*address, *value);
        }
        setup(&mut w, current);
        let result =
            w.e.call(0x0090_a1a0, &args![w.process, w.actor, u32::from(flag)])
                .bool();
        (w, result)
    }

    #[test]
    fn check_for_new_package_keeps_an_alarmed_actor() {
        let (w, result) = check(0, &[(ACTOR_IS_ALARMED, 1)], |_, _| {});
        assert!(!result);
        assert!(w.calls_to(process_slot(START_NEW_PACKAGE_SLOT)).is_empty());
        // With flag set the alarm is not asked.
        let (w, _) = check(1, &[(ACTOR_IS_ALARMED, 1)], |_, _| {});
        assert!(w.calls_to(ACTOR_IS_ALARMED).is_empty());
    }

    #[test]
    fn check_for_new_package_without_a_package_takes_the_schedule() {
        let (w, result) = check(0, &[], |w, current| w.e.mem.set_u32(current, 0));
        assert!(result);
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_PACKAGE_SLOT)),
            [vec![w.process, w.package, w.actor]]
        );
        assert_eq!(
            w.calls_to(process_slot(START_NEW_PACKAGE_SLOT)),
            [vec![w.process, w.actor, w.package]]
        );
        assert_eq!(
            w.calls_to(process_slot(SET_PACKAGE_DONE_ONCE_SLOT)),
            [vec![w.process, 0]]
        );
        assert_eq!(
            w.calls_to(REFERENCE_SET_PACKAGE_START_LOCATION)
                .last()
                .unwrap(),
            &vec![w.actor, 0x3700, 0x3600, 0x3800, 0]
        );
    }

    #[test]
    fn check_for_new_package_replaces_an_ended_package() {
        // An ordinary package always ends; the schedule gives another.
        let (w, result) = check(0, &[(ACTOR_GET_PACKAGE, 0x7700)], |_, _| {});
        assert!(result);
        assert_eq!(
            w.calls_to(process_slot(SET_CURRENT_PACKAGE_SLOT)),
            [vec![w.process, 0x7700, w.actor]]
        );
        // The schedule gives the same package: no change.
        let (w, result) = check(0, &[], |_, _| {});
        assert!(!result);
        assert!(w.calls_to(process_slot(START_NEW_PACKAGE_SLOT)).is_empty());
    }

    #[test]
    fn check_for_new_package_timed_packages_end_with_their_hours() {
        // A timed 0067efd0 package of type 3 from 8 for 1 hour: over at 10.
        let schedule_hours = |w: &mut World, start: u8, length: u32| {
            let schedule = w.e.mem.alloc(8);
            w.e.mem.set_u8(schedule + 3, start);
            w.e.mem.set_u32(schedule + 4, length);
            w.on(PACKAGE_GET_SCHEDULE, schedule);
        };
        let (w, result) = check(
            0,
            &[
                (PACKAGE_TEST_0067EFD0, 1),
                (PACKAGE_GET_TYPE, 3),
                (PACKAGE_GET_ONCE_PER_DAY, 0),
            ],
            |w, _| {
                schedule_hours(w, 8, 1);
                let process = w.object();
                w.on(ACTOR_GET_PROCESS, process);
                w.on(object_slot(GET_CURRENT_ACTION_COMPLETE_SLOT), 0);
                procedure(w, 2, 0);
                w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
                w.on(SET_ACTION_FLAG, 0);
                w.on(ACTOR_GET_PACKAGE, 0x7700);
                w.e.register(SCHEDULE_GET_HOUR, |_, _| ret(8));
                w.e.register(FTOL, |_, a| {
                    ret(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
                });
            },
        );
        assert!(result);
        assert_eq!(
            w.calls_to(SET_ACTION_FLAG),
            [vec![w.package, 0x4444, 0x400]]
        );
        // Within the hours (8 for 4): kept, the flag dropped.
        let (w, result) = check(
            1,
            &[
                (PACKAGE_TEST_0067EFD0, 1),
                (PACKAGE_GET_TYPE, 3),
                (PACKAGE_GET_ONCE_PER_DAY, 0),
            ],
            |w, _| {
                schedule_hours(w, 8, 4);
                let process = w.object();
                w.on(ACTOR_GET_PROCESS, process);
                w.on(object_slot(GET_CURRENT_ACTION_COMPLETE_SLOT), 0);
                procedure(w, 2, 0);
                w.on(REFERENCE_GET_EXTRA_LIST, 0x4444);
                w.on(SET_ACTION_FLAG, 0);
                w.on(actor_slot(ACTOR_SLOT_26C), 0);
                w.e.register(SCHEDULE_GET_HOUR, |_, _| ret(8));
                w.e.register(FTOL, |_, a| {
                    ret(f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32 as u32)
                });
            },
        );
        assert!(!result);
        assert!(w.calls_to(ACTOR_GET_PACKAGE).is_empty());
    }

    fn update(answers: &[(u32, u32)], setup: impl FnOnce(&mut World)) -> World {
        let mut w = world();
        constants(&mut w);
        w.e.map(UPDATE_MARKER, 4);
        w.on(EMPTY_00483710, 0);
        w.answer(CALENDAR_GET_HOUR, st0(10.0));
        w.on(actor_slot(ACTOR_SLOT_100), 1);
        w.on(actor_slot(ACTOR_SLOT_274), 0);
        w.on(process_slot(GET_CURRENT_PACKAGE_SLOT), w.package);
        w.on(process_slot(CHECK_FOR_NEW_PACKAGE_SLOT), 0);
        w.on(ACTOR_TEST_008B3B90, 0);
        w.on(PACKAGE_TEST_008A6290, 0);
        w.on(PACKAGE_GET_TARGET, 0);
        w.on(MOBILE_GET_PROCESS_TYPE, 0);
        w.on(ACTOR_IS_WAITING_ON_PATH, 0);
        let process = w.object();
        w.on(ACTOR_GET_PROCESS, process);
        w.on(object_slot(COMPUTE_LAST_TIME_PROCESSED_SLOT), 0);
        procedure(&mut w, 2, 3);
        for slot in [
            PROCESS_ACQUIRE_SLOT,
            PROCESS_ACTIVATE_SLOT,
            PROCESS_ALARM_SLOT,
            PROCESS_FLEE_SLOT,
            PROCESS_GUARD_SLOT,
            PROCESS_PATROL_SLOT,
            PROCESS_SANDBOX_SLOT,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
            PROCESS_FOLLOW_SLOT,
            PROCESS_PURSUE_SLOT,
            PROCESS_REMOVE_WORN_SLOT,
            PROCESS_ESCORT_SLOT,
        ] {
            w.on(process_slot(slot), 0);
        }
        for (address, value) in answers {
            w.on(*address, *value);
        }
        setup(&mut w);
        w.e.call(0x0090_ad40, &args![w.process, w.actor]);
        w
    }

    #[test]
    fn update_runs_the_procedure_step_and_marks_the_time() {
        let w = update(&[], |_| {});
        assert_eq!(
            w.calls_to(process_slot(PROCESS_ACQUIRE_SLOT)),
            [vec![w.process, w.actor]]
        );
        assert_eq!(
            w.calls_to(process_slot(CHECK_FOR_NEW_PACKAGE_SLOT)),
            [vec![w.process, w.actor, 0]]
        );
        let process = w.calls_to(object_slot(COMPUTE_LAST_TIME_PROCESSED_SLOT))[0][0];
        assert_ne!(process, 0);
        assert_eq!(w.e.mem.u8(w.process + 0xb0), 0);
        assert_eq!(
            w.calls_to(EMPTY_00483710),
            [vec![UPDATE_MARKER], vec![UPDATE_MARKER]]
        );
        // Waiting on the path: the time stamp is skipped.
        let w = update(&[(ACTOR_IS_WAITING_ON_PATH, 1)], |_| {});
        assert!(w
            .calls_to(object_slot(COMPUTE_LAST_TIME_PROCESSED_SLOT))
            .is_empty());
        assert_eq!(w.e.mem.u8(w.process + 0xb0), 1);
    }

    #[test]
    fn update_procedure_steps() {
        for (number, slot, words) in [
            (2u32, PROCESS_ACTIVATE_SLOT, vec![1u32]),
            (8, PROCESS_ALARM_SLOT, vec![]),
            (10, PROCESS_FLEE_SLOT, vec![]),
            (6, PROCESS_FOLLOW_SLOT, vec![1, 0x101, 0]),
            (14, PROCESS_ACTIVATE_SLOT, vec![0]),
            (15, PROCESS_PURSUE_SLOT, vec![0, 1, 0x101]),
            (35, PROCESS_REMOVE_WORN_SLOT, vec![1]),
            (41, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT, vec![2]),
            (42, PROCESS_SANDBOX_SLOT, vec![]),
            (44, PROCESS_PATROL_SLOT, vec![]),
            (47, PROCESS_GUARD_SLOT, vec![]),
        ] {
            let w = update(&[], |w| procedure(w, 4, number));
            let mut expected = vec![w.process, w.actor];
            expected.extend(words);
            assert_eq!(
                w.calls_to(process_slot(slot)),
                [expected],
                "procedure {number}"
            );
        }
        // Procedure 7 escorts only for package types 1, 2, 7 and 0x16.
        let w = update(&[(PACKAGE_GET_TYPE, 2)], |w| procedure(w, 4, 7));
        assert_eq!(w.calls_to(process_slot(PROCESS_ESCORT_SLOT)).len(), 1);
        let w = update(&[(PACKAGE_GET_TYPE, 3)], |w| procedure(w, 4, 7));
        assert!(w.calls_to(process_slot(PROCESS_ESCORT_SLOT)).is_empty());
        // Unknown numbers do nothing.
        let w = update(&[], |w| procedure(w, 4, 9));
        assert!(w
            .calls()
            .iter()
            .all(|(a, _)| *a < process_slot(0x7b4) || *a > process_slot(0x810)));
    }

    #[test]
    fn update_without_an_actor_or_package() {
        let w = update(&[(actor_slot(ACTOR_SLOT_100), 0)], |_| {});
        assert!(w
            .calls_to(process_slot(CHECK_FOR_NEW_PACKAGE_SLOT))
            .is_empty());
        assert_eq!(
            w.calls_to(object_slot(COMPUTE_LAST_TIME_PROCESSED_SLOT))
                .len(),
            1
        );
        let w = update(&[(process_slot(GET_CURRENT_PACKAGE_SLOT), 0)], |_| {});
        assert!(w.calls_to(process_slot(PROCESS_ACQUIRE_SLOT)).is_empty());
        assert_eq!(
            w.calls_to(object_slot(COMPUTE_LAST_TIME_PROCESSED_SLOT))
                .len(),
            1
        );
        let mut w = world();
        w.on(EMPTY_00483710, 0);
        w.e.call(0x0090_ad40, &args![w.process, 0u32]);
        assert_eq!(w.calls_to(EMPTY_00483710).len(), 1);
    }

    #[test]
    fn update_finishes_a_done_package() {
        let w = update(
            &[
                (MOBILE_GET_PROCESS_TYPE, 3),
                (process_slot(IS_PACKAGE_DONE_ONCE_SLOT), 0),
                (REFERENCE_GET_EXTRA_LIST, 0x4444),
                (SET_ACTION_FLAG, 0),
                (process_slot(PACKAGE_DONE_SLOT), 0),
                (process_slot(SET_PACKAGE_DONE_ONCE_SLOT), 0),
                (process_slot(SET_IS_AGGRESSOR_SLOT), 0),
                (PACKAGE_GET_ONCE_PER_DAY, 0),
                (PACKAGE_GET_IS_CREATED, 0),
                (INTERFACE_GET_TARGET, 0),
                (PACKAGE_GET_SCHEDULE, 0x5000),
                (WORD_04, 1),
            ],
            |w| {
                procedure(w, 2, PROCEDURE_DONE);
                w.e.register(LIST_IS_EMPTY, |_, _| ret(1));
            },
        );
        assert_eq!(
            w.calls_to(SET_ACTION_FLAG),
            [vec![w.package, 0x4444, 0x400]]
        );
        assert_eq!(
            w.calls_to(process_slot(PACKAGE_DONE_SLOT)),
            [vec![w.process, w.actor, w.package]]
        );
        assert_eq!(
            w.calls_to(process_slot(SET_IS_AGGRESSOR_SLOT)),
            [vec![w.process, 0]]
        );
    }
}

#[cfg(test)]
mod n027_tests {
    use super::*;

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The double address of slot `offset` of the vtable at `vtable`.
    fn slot(vtable: u32, offset: u32) -> u32 {
        vtable + 0x2000 + offset
    }

    /// A block of `size` bytes whose vtable `vtable` sends every listed slot to
    /// [`slot`]`(vtable, offset)`.
    fn object(e: &mut Engine, vtable: u32, size: u32, slots: &[u32]) -> u32 {
        let mut table = vec![0u32; 0x220];
        for offset in slots {
            table[*offset as usize / 4] = slot(vtable, *offset);
        }
        e.put_vtable(vtable, &table);
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, vtable);
        block
    }

    const PROCESS_VTABLE: u32 = 0x0070_0000;
    const PACKAGE_VTABLE: u32 = 0x0071_0000;
    const ACTOR_VTABLE: u32 = 0x0072_0000;

    /// A process whose slot `+0x22c` answers `package` (doubles for the slots `+0x280`,
    /// `+0x288` and `+0x22c` registered by the caller as needed).
    fn process(e: &mut Engine) -> Ptr<LowProcess> {
        let slots = [
            GET_CURRENT_PACKAGE_SLOT,
            GET_PROCEDURE_INDEX_RUNNING_SLOT,
            ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
        ];
        Ptr::new(object(e, PROCESS_VTABLE, 0x200, &slots))
    }

    #[test]
    fn damage_mod_actor_value_only_for_actor_value_0x10() {
        for (code, flagged, adjusts) in
            [(0x10u32, false, 1usize), (0x11, false, 0), (0x10, true, 0)]
        {
            let mut e = Engine::new();
            let this = process(&mut e);
            e.register_double(ACTOR_VALUE_HAS_FLAG, move |_, _| flagged.into_ret());
            e.register(CHECK_CLAMP_DAMAGE_MODIFIER, |_, a| {
                (f64::from(f32::from_bits(a[2])) - 1.0).into_ret()
            });
            e.register(MODIFIER_LIST_ADJUST_MODIFIER, nothing);
            e.call_log = Some(vec![]);
            e.call(0x0090_7460, &args![this, 0x3000u32, code, 4.0f32]);
            let adjusted = calls(&e, MODIFIER_LIST_ADJUST_MODIFIER);
            assert_eq!(adjusted.len(), adjusts);
            if adjusts == 1 {
                assert_eq!(
                    calls(&e, CHECK_CLAMP_DAMAGE_MODIFIER),
                    vec![vec![0x30a4, 0x10, 4.0f32.to_bits()]]
                );
                assert_eq!(
                    adjusted[0],
                    vec![this.addr() + 0x94, 0x10, 3.0f32.to_bits(), 0]
                );
            }
        }
        // No actor: the owner is 0.
        let mut e = Engine::new();
        let this = process(&mut e);
        e.register(ACTOR_VALUE_HAS_FLAG, |_, _| false.into_ret());
        e.register(CHECK_CLAMP_DAMAGE_MODIFIER, |_, _| 0.0f64.into_ret());
        e.register(MODIFIER_LIST_ADJUST_MODIFIER, nothing);
        e.call_log = Some(vec![]);
        e.call(0x0090_7460, &args![this, 0u32, 0x10u32, 4.0f32]);
        assert_eq!(calls(&e, CHECK_CLAMP_DAMAGE_MODIFIER)[0][0], 0);
    }

    #[test]
    fn mod_essential_down_timer_subtracts_the_time_step() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.mem.set_f32(this.addr() + 0xa4, 5.0);
        e.register(GLOBAL_TIME_STEP, |_, _| 1.25f64.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0090_7620, &args![this]);
        assert_eq!(e.mem.f32(this.addr() + 0xa4), 3.75);
        assert_eq!(calls(&e, GLOBAL_TIME_STEP), vec![vec![GLOBAL_TIME_OBJECT]]);
    }

    #[test]
    fn should_stop_combat_controller_by_type_and_procedure() {
        let mut e = Engine::new();
        e.map(0x011a_3000, 0x1000);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list + 4, PROCEDURE_DONE);
        e.mem.set_u32(list + 8, 3);
        e.mem.set_u32(PROCEDURE_LISTS + 4 * 2, list);
        // The package's type is its word +0, its procedure type the word +4.
        e.register(PACKAGE_GET_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(PACKAGE_GET_PROCEDURE_TYPE, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        let package = e.mem.alloc(8);
        for (kind, procedure_type, index, expected) in [
            (0u32, 2u32, 1u32, false),
            (17, 2, 2, false),
            (2, 2, 1, false),
            (2, 2, 2, true),
            (0x12, 2, 1, false),
            (5, 0xffff_ffff, 1, true),
        ] {
            e.mem.set_u32(package, kind);
            e.mem.set_u32(package + 4, procedure_type);
            assert_eq!(
                e.call(0x0090_7960, &args![0x1000u32, package, index])
                    .bool(),
                expected,
                "type {kind}"
            );
        }
    }

    /// The actor `0x3000` whose extra list is `0x3044` and package data `data` (or none).
    fn combat_target_engine(data: u32, init: bool, next: bool) -> Engine {
        let mut e = Engine::new();
        e.register(PACKAGE_GET_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(REFERENCE_GET_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register_double(EXTRA_GET_ACTOR_PACKAGE_DATA, move |_, _| data.into_ret());
        e.register_double(PATROL_DATA_INIT, move |_, _| init.into_ret());
        e.register_double(PATROL_SET_NEXT_REFERENCE, move |_, _| next.into_ret());
        e.register(PATROL_GET_CURRENT_REFERENCE, |_, _| 0x7777u32.into_ret());
        e.register(EXTRA_SET_PACKAGE_TARGET, nothing);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn update_package_target_for_combat_initialises_or_moves_the_patrol() {
        // Not a patrol package.
        let mut e = combat_target_engine(0, true, true);
        let patrol_package = e.mem.alloc(4);
        e.mem.set_u32(patrol_package, 0xd);
        let other_package = e.mem.alloc(4);
        e.mem.set_u32(other_package, 0xc);
        assert!(!e
            .call(0x0090_79f0, &args![0u32, 0x3000u32, other_package])
            .bool());
        assert!(calls(&e, REFERENCE_GET_EXTRA_LIST).is_empty());
        // No data.
        assert!(!e
            .call(0x0090_79f0, &args![0u32, 0x3000u32, patrol_package])
            .bool());
        // First time: initialised.
        let mut e = combat_target_engine(0, true, true);
        let data = e.mem.alloc(8);
        e.register_double(EXTRA_GET_ACTOR_PACKAGE_DATA, move |_, _| data.into_ret());
        let patrol_package = e.mem.alloc(4);
        e.mem.set_u32(patrol_package, 0xd);
        assert!(e
            .call(0x0090_79f0, &args![0u32, 0x3000u32, patrol_package])
            .bool());
        assert_eq!(e.mem.u32(data + 4), 1);
        assert_eq!(
            calls(&e, PATROL_DATA_INIT),
            vec![vec![data, 0x3000, patrol_package]]
        );
        assert_eq!(
            calls(&e, EXTRA_SET_PACKAGE_TARGET),
            vec![vec![0x3044, 0x7777]]
        );
        // Then: the next reference.
        assert!(e
            .call(0x0090_79f0, &args![0u32, 0x3000u32, patrol_package])
            .bool());
        assert_eq!(calls(&e, PATROL_SET_NEXT_REFERENCE), vec![vec![data, 1]]);
        assert_eq!(calls(&e, EXTRA_SET_PACKAGE_TARGET).len(), 2);
        // Failures answer false and set nothing.
        for (initialised, init, next) in [(0u32, false, true), (1, true, false)] {
            let mut e = combat_target_engine(0, init, next);
            let data = e.mem.alloc(8);
            e.mem.set_u32(data + 4, initialised);
            e.register_double(EXTRA_GET_ACTOR_PACKAGE_DATA, move |_, _| data.into_ret());
            let patrol_package = e.mem.alloc(4);
            e.mem.set_u32(patrol_package, 0xd);
            assert!(!e
                .call(0x0090_79f0, &args![0u32, 0x3000u32, patrol_package])
                .bool());
            assert!(calls(&e, EXTRA_SET_PACKAGE_TARGET).is_empty());
            assert_eq!(e.mem.u32(data + 4), initialised);
        }
    }

    #[test]
    fn process_avoid_area_moves_the_procedure_on() {
        let mut e = Engine::new();
        let this = process(&mut e);
        e.register(
            slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT),
            nothing,
        );
        e.call_log = Some(vec![]);
        e.call(0x0090_8f40, &args![this, 0x3000u32]);
        assert_eq!(
            calls(
                &e,
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)
            ),
            vec![vec![this.addr(), 0x3000, 1]]
        );
    }

    #[test]
    fn process_flee_ends_the_interrupt_package_when_done() {
        for done in [true, false] {
            let mut e = Engine::new();
            e.map(0x0101_2000, 0x1000);
            e.set_global(MINUS_ONE, -1.0f32);
            let this = process(&mut e);
            let package = object(&mut e, PACKAGE_VTABLE, 0x10, &[PACKAGE_SLOT_13C]);
            e.register_double(
                slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT),
                move |_, _| package.into_ret(),
            );
            e.register_double(slot(PACKAGE_VTABLE, PACKAGE_SLOT_13C), move |_, _| {
                done.into_ret()
            });
            e.register(ACTOR_END_INTERRUPT_PACKAGE, nothing);
            // The time delta's callees.
            e.register(LOOKUP_FORM_BY_ID, |_, _| 0x9000u32.into_ret());
            e.register(CALENDAR_GET_HOUR, |_, _| 0.0f64.into_ret());
            e.register(PROCESS_GET_HOUR, |_, _| 0.0f64.into_ret());
            e.register(GLOBAL_VALUE, |_, _| 1.0f64.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0090_9c90, &args![this, 0x3000u32]);
            assert_eq!(
                calls(&e, slot(PACKAGE_VTABLE, PACKAGE_SLOT_13C)),
                vec![vec![package, 0x3000, 0, (-1.0f32).to_bits(), 0]]
            );
            let ended = calls(&e, ACTOR_END_INTERRUPT_PACKAGE);
            if done {
                assert_eq!(ended, vec![vec![0x3000, 0]]);
            } else {
                assert!(ended.is_empty());
            }
        }
    }

    #[test]
    fn current_package_location_or_the_zero_point() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem
                .set_f32(DESTINATION_DEFAULT_POINT + 4 * i as u32, *value);
        }
        let this = process(&mut e);
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0u32.into_ret()
        });
        let out = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0090_a130, &args![this, out, 0x3000u32]).u32(), out);
        assert_eq!(e.mem.f32(out + 8), 3.0);
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0x4000u32.into_ret()
        });
        e.register(PACKAGE_GET_LOCATION_COORD, nothing);
        assert_eq!(e.call(0x0090_a130, &args![this, out, 0x3000u32]).u32(), out);
        assert_eq!(
            calls(&e, PACKAGE_GET_LOCATION_COORD),
            vec![vec![0x4000, out, 0x3000]]
        );
    }

    #[test]
    fn is_wandering_checks_the_running_procedure() {
        let mut e = Engine::new();
        e.map(0x011a_3000, 0x1000);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list + 4, 1);
        e.mem.set_u32(list + 8, 2);
        e.mem.set_u32(PROCEDURE_LISTS + 4 * 3, list);
        let this = process(&mut e);
        e.register(PACKAGE_GET_PROCEDURE_TYPE, |_, _| 3u32.into_ret());
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0u32.into_ret()
        });
        assert!(!e.call(0x0090_cfa0, &args![this]).bool());
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0x4000u32.into_ret()
        });
        for (index, wandering) in [(1u32, true), (2, false)] {
            e.register_double(
                slot(PROCESS_VTABLE, GET_PROCEDURE_INDEX_RUNNING_SLOT),
                move |_, _| index.into_ret(),
            );
            assert_eq!(e.call(0x0090_cfa0, &args![this]).bool(), wandering);
        }
    }

    #[test]
    fn enter_combat_passes_its_words_on() {
        let mut e = Engine::new();
        let actor = object(&mut e, ACTOR_VTABLE, 0x10, &[ACTOR_START_COMBAT_SLOT]);
        e.register(slot(ACTOR_VTABLE, ACTOR_START_COMBAT_SLOT), nothing);
        e.call_log = Some(vec![]);
        let words: Vec<u32> = (1..=12).map(|i| 0x100 * i + 0x55).collect();
        let mut all = vec![0x1000u32, actor];
        all.extend(&words);
        assert!(e.call(0x0090_dc10, &all).bool());
        assert_eq!(
            calls(&e, slot(ACTOR_VTABLE, ACTOR_START_COMBAT_SLOT)),
            vec![vec![actor, 0x155, 0x455, 0x55, 0x55, 0x55, 0, 0x55, 0xc55]]
        );
    }

    #[test]
    fn fn_0090e600_sets_the_start_values() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x1c);
        for offset in (0..0x1c).step_by(4) {
            e.mem.set_u32(block + offset, 0xabab_abab);
        }
        assert_eq!(e.call(0x0090_e600, &args![block]).u32(), block);
        let words: Vec<u32> = (0..7).map(|i| e.mem.u32(block + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0, 0, 0xabab_0000, 0, 0x7fff_ffff, 0, 0xffff_ffff]
        );
    }

    #[test]
    fn fn_00911ae0_answers_zero() {
        let mut e = Engine::new();
        let answer = e.call(0x0091_1ae0, &args![0x1000u32, 1u32, 2u32]).f32();
        assert_eq!(answer.to_bits(), 0);
    }

    #[test]
    fn process_summon_creature_defend_hands_over() {
        let mut e = Engine::new();
        let this =
            Ptr::<LowProcess>::new(object(&mut e, PROCESS_VTABLE, 0x200, &[PROCESS_SLOT_298]));
        e.register(slot(PROCESS_VTABLE, PROCESS_SLOT_298), nothing);
        e.call_log = Some(vec![]);
        e.call(0x0091_1ed0, &args![this, 0x3000u32]);
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, PROCESS_SLOT_298)),
            vec![vec![this.addr(), 0x3000, 0, 0x101, 0]]
        );
    }

    #[test]
    fn process_movement_blocked_waits_for_the_limit() {
        for (hour, moves_on) in [(5.0f64, true), (1.05, false), (f64::NAN, false)] {
            let mut e = Engine::new();
            e.map(0x0101_f000, 0x1000);
            e.set_global(BLOCKED_TIME_LIMIT, 0.1f64);
            let this = process(&mut e);
            e.register_double(CALENDAR_GET_HOUR, move |_, _| hour.into_ret());
            e.register(ACTOR_PACKAGE_GET_START_HOUR, |_, _| 1.0f64.into_ret());
            e.register(BLOCKED_TIME_00408840, |_, a| {
                f64::from(f32::from_bits(a[0]).abs()).into_ret()
            });
            e.register(
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT),
                nothing,
            );
            e.register(WORD_AT_00559450, |_, _| 0x4000u32.into_ret());
            e.register(PACKAGE_SET_IS_CREATED, nothing);
            e.register(ACTOR_STOP_MOVING, nothing);
            e.call_log = Some(vec![]);
            e.call(0x0091_2150, &args![this, 0x3000u32]);
            assert_eq!(
                calls(&e, ACTOR_PACKAGE_GET_START_HOUR),
                vec![vec![this.addr() + 4]]
            );
            if moves_on {
                assert_eq!(
                    calls(
                        &e,
                        slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)
                    ),
                    vec![vec![this.addr(), 0x3000, 1]]
                );
                assert_eq!(calls(&e, PACKAGE_SET_IS_CREATED), vec![vec![0x4000, 1]]);
                assert!(calls(&e, ACTOR_STOP_MOVING).is_empty());
            } else {
                assert_eq!(calls(&e, ACTOR_STOP_MOVING), vec![vec![0x3000]]);
                assert!(calls(&e, PACKAGE_SET_IS_CREATED).is_empty());
            }
        }
    }

    #[test]
    fn is_package_done_checks_the_done_procedure() {
        let mut e = Engine::new();
        e.map(0x011a_3000, 0x1000);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list + 4, PROCEDURE_DONE);
        e.mem.set_u32(PROCEDURE_LISTS + 4, list);
        let this = process(&mut e);
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0u32.into_ret()
        });
        assert!(e.call(0x0091_3460, &args![this, 0u32]).bool());
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0x4000u32.into_ret()
        });
        e.register(PACKAGE_GET_PROCEDURE_TYPE, |_, _| 1u32.into_ret());
        for (index, done) in [(1u32, true), (0, false)] {
            e.register_double(
                slot(PROCESS_VTABLE, GET_PROCEDURE_INDEX_RUNNING_SLOT),
                move |_, _| index.into_ret(),
            );
            assert_eq!(e.call(0x0091_3460, &args![this, 0u32]).bool(), done);
        }
    }

    #[test]
    fn mod_tracked_damage_for_tracked_actors() {
        for (slot_360, test, adds) in [
            (true, false, true),
            (false, true, true),
            (false, false, false),
        ] {
            let mut e = Engine::new();
            let this = process(&mut e);
            e.mem.set_f32(this.addr() + 0xac, 2.0);
            let actor = object(&mut e, ACTOR_VTABLE, 0x10, &[ACTOR_SLOT_360]);
            e.register_double(slot(ACTOR_VTABLE, ACTOR_SLOT_360), move |_, _| {
                slot_360.into_ret()
            });
            e.register_double(ACTOR_TEST_00566950, move |_, _| test.into_ret());
            e.call(0x0091_34c0, &args![this, actor, 1.5f32]);
            let expected = if adds { 3.5 } else { 2.0 };
            assert_eq!(e.mem.f32(this.addr() + 0xac), expected);
        }
        let mut e = Engine::new();
        let this = process(&mut e);
        e.call(0x0091_34c0, &args![this, 0u32, 1.5f32]);
        assert_eq!(e.mem.f32(this.addr() + 0xac), 0.0);
    }

    #[test]
    fn lock_map_deleting_destructors_free_on_bit_0() {
        for (address, vtable) in [
            (0x0091_37e0u32, LOCK_MAP_BASE_VTABLE),
            (0x0091_3980, LOCK_MAP_BASE_VTABLE),
        ] {
            for (flags, frees) in [(1u32, 1usize), (0, 0)] {
                let mut e = Engine::new();
                let map = e.mem.alloc(0x10);
                e.mem.set_u32(map + 8, 0x4440);
                e.register(MAP_REMOVE_ALL, nothing);
                e.register(NI_FREE, nothing);
                e.register(FREE_OBJECT, nothing);
                e.call_log = Some(vec![]);
                assert_eq!(e.call(address, &args![map, flags]).u32(), map);
                assert_eq!(calls(&e, FREE_OBJECT).len(), frees);
                assert_eq!(calls(&e, NI_FREE), vec![vec![0x4440]]);
                assert_eq!(e.mem.u32(map), vtable);
            }
        }
    }

    #[test]
    fn lock_map_item_delete_clears_the_value() {
        let mut e = Engine::new();
        let value = e.mem.alloc(8);
        e.mem.set_u32(value, 9);
        e.mem.set_u32(value + 4, 9);
        assert_eq!(e.call(0x0091_3950, &args![value, 7u32]).u32(), value);
        assert_eq!((e.mem.u32(value), e.mem.u32(value + 4)), (0, 0));
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(item + 8, 0x11);
        e.mem.set_u32(item + 0xc, 0x22);
        e.register(MAP_ALLOCATOR_DEALLOCATE, nothing);
        e.call_log = Some(vec![]);
        e.call(0x0091_3910, &args![0x3000u32, item]);
        assert_eq!((e.mem.u32(item + 8), e.mem.u32(item + 0xc)), (0, 0));
        assert_eq!(
            calls(&e, MAP_ALLOCATOR_DEALLOCATE),
            vec![vec![0x300c, item]]
        );
    }

    #[test]
    fn revert_ov2_clears_the_state() {
        for has in [true, false] {
            let mut e = Engine::new();
            let this =
                Ptr::<LowProcess>::new(object(&mut e, PROCESS_VTABLE, 0x200, &[SET_TARGET_SLOT]));
            for offset in (4..0xb4).step_by(4) {
                e.mem.set_u32(this.addr() + offset, 0x5555_5555);
            }
            e.register(BASE_PROCESS_REVERT_OV2, nothing);
            e.register(slot(PROCESS_VTABLE, SET_TARGET_SLOT), nothing);
            e.register(COMBAT_TIMER_RESET_00910B10, nothing);
            e.register(REVERT_CHANGE_FLAGS, |_, a| (a[1] + 1).into_ret());
            e.register_double(FLAGS_HAVE_004280F0, move |_, a| {
                (a[1] == SAVE_MODIFIERS_FLAG && has).into_ret()
            });
            e.register(MODIFIER_LIST_REVERT, nothing);
            // The release body's callees.
            for callee in [LIST_IS_EMPTY, LIST_CLEAR, DELETE_EMPTY_DESTRUCTOR] {
                e.register(callee, |_, _| true.into_ret());
            }
            e.mem.set_u32(this.addr() + OBJECT_TO_ACQUIRE, 0);
            e.call_log = Some(vec![]);
            e.call(0x0091_0a60, &args![this, 0x6000u32]);
            let base = this.addr();
            assert_eq!(calls(&e, BASE_PROCESS_REVERT_OV2), vec![vec![base, 0x6000]]);
            assert_eq!(e.mem.u32(base + 0x58), 0);
            assert_eq!(
                calls(&e, slot(PROCESS_VTABLE, SET_TARGET_SLOT)),
                vec![vec![base, 0]]
            );
            for offset in [0xa4u32, 0x34, 0xa8, 0xac] {
                assert_eq!(e.mem.u32(base + offset), 0, "offset {offset:#x}");
            }
            assert_eq!(e.mem.u8(base + 0x30), 0);
            assert_eq!(
                calls(&e, COMBAT_TIMER_RESET_00910B10),
                vec![vec![base + 0x38]]
            );
            assert_eq!(calls(&e, MODIFIER_LIST_REVERT).len(), usize::from(has));
        }
    }

    #[test]
    fn init_load_game_resolves_target_and_location() {
        let mut e = Engine::new();
        let this =
            Ptr::<LowProcess>::new(object(&mut e, PROCESS_VTABLE, 0x200, &[SET_TARGET_SLOT]));
        let base = this.addr();
        e.mem.set_u32(base + 0x40, 0x11);
        e.mem.set_u32(base + 0x44, 0x22);
        e.register(BASE_PROCESS_INIT_LOAD_GAME, nothing);
        e.register(LOOKUP_FORM_BY_ID, |_, a| (a[0] << 8).into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 1).into_ret());
        e.register(slot(PROCESS_VTABLE, SET_TARGET_SLOT), |e, a| {
            e.mem.set_u32(a[0] + 0x40, a[1]);
            Ret::default()
        });
        e.register(REFERENCE_SET_TARGETED, nothing);
        e.call_log = Some(vec![]);
        e.call(0x0091_0250, &args![this, 1u32, 2u32, 3u32]);
        assert_eq!(
            calls(&e, BASE_PROCESS_INIT_LOAD_GAME),
            vec![vec![base, 1, 2, 3]]
        );
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, SET_TARGET_SLOT)),
            vec![vec![base, 0x1101]]
        );
        assert_eq!(calls(&e, REFERENCE_SET_TARGETED), vec![vec![0x1101, 1]]);
        assert_eq!(e.mem.u32(base + 0x44), 0x2201);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            vec![
                0x1100,
                0,
                FORM_TYPE_DESCRIPTOR,
                REFERENCE_TYPE_DESCRIPTOR,
                0
            ]
        );
        // No saved ids: only the base call.
        e.mem.set_u32(base + 0x40, 0);
        e.mem.set_u32(base + 0x44, 0);
        e.call_log = Some(vec![]);
        e.call(0x0091_0250, &args![this, 1u32, 2u32, 3u32]);
        assert!(calls(&e, LOOKUP_FORM_BY_ID).is_empty());
    }

    /// The escort world: process `this` (its current package `0x4100`), the target actor
    /// (`+0x40`, its process `0x7000` with slots `+0x28`, `+0x710`, `+0x12c`), the escort
    /// package `0x4200`, the new package `0x4300`.
    fn escort_engine(target_is_player: bool) -> (Engine, Ptr<LowProcess>, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let this = process(&mut e);
        let actor = object(
            &mut e,
            ACTOR_VTABLE,
            0x10,
            &[ACTOR_SLOT_214_ESCORT, ACTOR_SLOT_418_ESCORT, ACTOR_SLOT_2F4],
        );
        let target = object(
            &mut e,
            0x0073_0000,
            0x10,
            &[ACTOR_SLOT_214_ESCORT, ACTOR_SLOT_418_ESCORT, ACTOR_SLOT_2F4],
        );
        let target_process = object(
            &mut e,
            0x0074_0000,
            0x10,
            &[
                COMPUTE_LAST_TIME_PROCESSED_SLOT,
                PROCESS_SAVE_PACKAGE_SLOT,
                SET_TARGET_SLOT,
            ],
        );
        e.mem.set_u32(this.addr() + TARGET, target);
        if target_is_player {
            e.set_global(PLAYER, target);
        }
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0x4100u32.into_ret()
        });
        e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_214_ESCORT), |_, _| {
            1u32.into_ret()
        });
        e.register(PACKAGE_CREATE, |_, _| 0x4300u32.into_ret());
        e.register(PACKAGE_LOCATION_0055B980, |_, a| (a[0] + 0x10).into_ret());
        e.register(PACKAGE_GET_ESCORT_FOLLOW_DISTANCE, |_, _| {
            0x99u32.into_ret()
        });
        e.register(PACKAGE_GET_TARGET, |_, a| (a[0] + 0x20).into_ret());
        e.register(WORD_08_0044DDC0, |_, a| (a[0] + 8).into_ret());
        e.register_double(ACTOR_GET_PROCESS, move |_, _| target_process.into_ret());
        for callee in [
            slot(ACTOR_VTABLE, ACTOR_SLOT_418_ESCORT),
            slot(0x0073_0000, ACTOR_SLOT_2F4),
            slot(0x0074_0000, COMPUTE_LAST_TIME_PROCESSED_SLOT),
            slot(0x0074_0000, PROCESS_SAVE_PACKAGE_SLOT),
            slot(0x0074_0000, SET_TARGET_SLOT),
            PACKAGE_SET_PACK_TYPE,
            PACKAGE_00826B40,
            PACKAGE_00826B90,
            PACKAGE_SET_LOCATION,
            LOCATION_0067F1F0,
            PACKAGE_SET_TARGET,
            PACKAGE_00984F60,
            PACKAGE_TARGET_SET_TYPE,
            PACKAGE_TARGET_SET_REFERENCE,
            ACTOR_PACKAGE_SET_TARGET,
            PACKAGE_0067A1B0,
            ACTOR_ADD_FOLLOWER,
        ] {
            e.register(callee, nothing);
        }
        e.call_log = Some(vec![]);
        (e, this, actor, target)
    }

    #[test]
    fn create_follow_for_escort_builds_the_follow_package() {
        let (mut e, this, actor, target) = escort_engine(false);
        e.call(0x0090_8cd0, &args![this, actor, 0x4200u32, 0u32]);
        assert_eq!(
            calls(&e, slot(ACTOR_VTABLE, ACTOR_SLOT_418_ESCORT)),
            vec![vec![actor]]
        );
        assert_eq!(calls(&e, PACKAGE_CREATE), vec![vec![1]]);
        assert_eq!(calls(&e, PACKAGE_SET_PACK_TYPE), vec![vec![0x4300, 1]]);
        assert_eq!(calls(&e, PACKAGE_SET_LOCATION), vec![vec![0x4300, 0x4110]]);
        assert_eq!(calls(&e, LOCATION_0067F1F0), vec![vec![0x4310, 0x99]]);
        assert_eq!(calls(&e, PACKAGE_SET_TARGET), vec![vec![0x4300, 0x4120]]);
        assert_eq!(calls(&e, PACKAGE_00984F60), vec![vec![0x4300, 6]]);
        assert_eq!(calls(&e, PACKAGE_TARGET_SET_TYPE), vec![vec![0x4320, 0]]);
        assert_eq!(
            calls(&e, PACKAGE_TARGET_SET_REFERENCE),
            vec![vec![0x4320, actor]]
        );
        assert_eq!(
            calls(&e, ACTOR_PACKAGE_SET_TARGET),
            vec![vec![0x4320, 0x4228]]
        );
        assert_eq!(calls(&e, PACKAGE_0067A1B0), vec![vec![0x4300, 0x4200]]);
        assert_eq!(calls(&e, ACTOR_ADD_FOLLOWER), vec![vec![actor, target]]);
        assert_eq!(
            calls(&e, slot(0x0073_0000, ACTOR_SLOT_2F4)),
            vec![vec![target, 0x4300, 0, 1]]
        );
        let target_process = calls(&e, slot(0x0074_0000, SET_TARGET_SLOT))[0][0];
        assert_eq!(
            calls(&e, slot(0x0074_0000, PROCESS_SAVE_PACKAGE_SLOT)),
            vec![vec![target_process, target]]
        );
        assert_eq!(
            calls(&e, slot(0x0074_0000, SET_TARGET_SLOT)),
            vec![vec![target_process, actor]]
        );
        // The player, the actor itself or no target: nothing.
        let (mut e, this, actor, _) = escort_engine(true);
        e.call(0x0090_8cd0, &args![this, actor, 0x4200u32, 0u32]);
        assert!(calls(&e, PACKAGE_CREATE).is_empty());
        let (mut e, this, _, target) = escort_engine(false);
        e.call(0x0090_8cd0, &args![this, target, 0x4200u32, 0u32]);
        assert!(calls(&e, PACKAGE_CREATE).is_empty());
    }

    #[test]
    fn process_remove_worn_for_npcs_and_creatures() {
        for (npc, package_type, special, expected) in [
            (true, 0u32, false, None),
            (false, 4, false, Some((0u32, 0u32))),
            (false, 1, true, Some((1, 0))),
            (false, 1, false, Some((1, 1))),
        ] {
            let mut e = Engine::new();
            let this = process(&mut e);
            let actor = object(&mut e, ACTOR_VTABLE, 0x10, &[ACTOR_SLOT_1E8]);
            e.register(REFERENCE_GET_BASE_FORM, |_, _| 0x5000u32.into_ret());
            e.register_double(DYNAMIC_CAST, move |_, a| {
                let is_npc = a[3] == NPC_TYPE_DESCRIPTOR;
                if is_npc == npc {
                    0x5100u32.into_ret()
                } else {
                    0u32.into_ret()
                }
            });
            e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
                0x4000u32.into_ret()
            });
            e.register_double(PACKAGE_GET_TYPE, move |_, _| package_type.into_ret());
            e.register_double(PACKAGE_TEST_00441B00, move |_, _| special.into_ret());
            e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_1E8), |_, _| {
                0x6000u32.into_ret()
            });
            e.register(NPC_INIT_DEFAULT_WORN, nothing);
            e.register(CREATURE_INIT_DEFAULT_WORN, nothing);
            e.register(
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT),
                nothing,
            );
            e.call_log = Some(vec![]);
            e.call(0x0090_9320, &args![this, actor, 1u32]);
            match expected {
                None => {
                    assert_eq!(
                        calls(&e, NPC_INIT_DEFAULT_WORN),
                        vec![vec![0x5100, actor, 0x6000, 1]]
                    );
                }
                Some((first, second)) => {
                    assert_eq!(
                        calls(&e, CREATURE_INIT_DEFAULT_WORN),
                        vec![vec![0x5100, actor, first, second, 1]]
                    );
                }
            }
            assert_eq!(
                calls(
                    &e,
                    slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)
                ),
                vec![vec![this.addr(), actor, 1]]
            );
        }
    }

    #[test]
    fn check_if_has_object_by_target_type() {
        for (kind, expected) in [(0u32, 0), (1, 1), (2, 2), (3, 0), (4, 9)] {
            let mut e = Engine::new();
            let this = process(&mut e);
            e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
                0x4000u32.into_ret()
            });
            e.register(PACKAGE_GET_TARGET, |_, _| 0x4400u32.into_ret());
            e.register_double(PACKAGE_TARGET_GET_TYPE, move |_, _| kind.into_ret());
            e.register(WORD_08_0044DDC0, |_, _| 0x7700u32.into_ret());
            e.register(GET_WORD_0C, |_, _| 0x0cu32.into_ret());
            e.register(REFERENCE_GET_BASE_FORM, |_, _| 0x7710u32.into_ret());
            e.register(PACKAGE_TARGET_GET_OBJECT, |_, _| 0x8800u32.into_ret());
            e.register(PACKAGE_TARGET_GET_OBJECT_TYPE, |_, _| 0x31u32.into_ret());
            e.register(ACTOR_HAS_OBJECTS, |_, _| true.into_ret());
            e.call_log = Some(vec![]);
            let answer = e.call(0x0090_ac40, &args![this, 0x3000u32, 5u32]).bool();
            let asked = calls(&e, ACTOR_HAS_OBJECTS);
            if expected == 9 {
                assert!(!answer);
                assert!(asked.is_empty());
                continue;
            }
            assert!(answer);
            assert_eq!(asked.len(), 1);
            let words = &asked[0][..5];
            match expected {
                0 => assert_eq!(words, &[0x3000, 0x7710, 0, 1, 0x0c]),
                1 => assert_eq!(words, &[0x3000, 0x8800, 0, 5, 0]),
                _ => assert_eq!(words, &[0x3000, 0, 0x31, 5, 0]),
            }
        }
        // No target, or no reference for type 0.
        let mut e = Engine::new();
        let this = process(&mut e);
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0x4000u32.into_ret()
        });
        e.register(PACKAGE_GET_TARGET, |_, _| 0u32.into_ret());
        assert!(!e.call(0x0090_ac40, &args![this, 0x3000u32, 5u32]).bool());
        e.register(PACKAGE_GET_TARGET, |_, _| 0x4400u32.into_ret());
        e.register(PACKAGE_TARGET_GET_TYPE, |_, _| 0u32.into_ret());
        e.register(WORD_08_0044DDC0, |_, _| 0u32.into_ret());
        assert!(!e.call(0x0090_ac40, &args![this, 0x3000u32, 5u32]).bool());
    }

    /// The combat world of `ProcessCombat`: process (its package's slot `+0x144` answers
    /// `package_ok`), the actor and the target (slot `+0x22c` false, `00493bb0` true for the
    /// actor and `target_busy` for the target), the target's process.
    struct CombatWorld {
        e: Engine,
        this: Ptr<LowProcess>,
        actor: u32,
        target: u32,
        target_process: u32,
    }

    fn combat_world(package_ok: bool, target_busy: bool, rounds: f32) -> CombatWorld {
        let mut e = Engine::new();
        e.map(0x011c_d000, 0x1000);
        e.set_global(COMBAT_ROUNDS_SETTING + 4, rounds);
        e.map(0x0101_d000, 0x1000);
        e.set_global(COMBAT_ROUNDS_FACTOR, 1.0f64);
        e.map(0x011d_e000, 0x1000);
        let this = Ptr::<LowProcess>::new(object(
            &mut e,
            PROCESS_VTABLE,
            0x200,
            &[
                GET_CURRENT_PACKAGE_SLOT,
                SET_TARGET_FOR_PACKAGE_SLOT,
                PROCESS_SLOT_298,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                ENTER_COMBAT_SLOT,
                PROCESS_PURSUE_SLOT,
            ],
        ));
        let package = object(&mut e, PACKAGE_VTABLE, 0x10, &[PACKAGE_SLOT_144]);
        let actor = object(
            &mut e,
            ACTOR_VTABLE,
            0x10,
            &[ACTOR_SLOT_22C, ACTOR_COMBAT_ROUND_SLOT, ACTOR_SLOT_304],
        );
        let target = object(
            &mut e,
            0x0073_0000,
            0x10,
            &[ACTOR_SLOT_22C, ACTOR_COMBAT_ROUND_SLOT],
        );
        let target_process = object(
            &mut e,
            0x0074_0000,
            0x10,
            &[ENTER_COMBAT_SLOT, COMPUTE_LAST_TIME_PROCESSED_SLOT],
        );
        e.mem.set_u32(this.addr() + TARGET, target);
        e.register_double(
            slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT),
            move |_, _| package.into_ret(),
        );
        e.register_double(slot(PACKAGE_VTABLE, PACKAGE_SLOT_144), move |_, _| {
            package_ok.into_ret()
        });
        e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_22C), |_, _| false.into_ret());
        e.register(slot(0x0073_0000, ACTOR_SLOT_22C), |_, _| false.into_ret());
        e.register_double(ACTOR_TEST_00493BB0, move |_, a| {
            (a[0] == actor || target_busy).into_ret()
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(SETTING_VALUE_00403E20, |_, a| (a[0] + 4).into_ret());
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        e.register_double(ACTOR_GET_PROCESS, move |_, _| target_process.into_ret());
        for callee in [
            slot(PROCESS_VTABLE, SET_TARGET_FOR_PACKAGE_SLOT),
            slot(PROCESS_VTABLE, PROCESS_SLOT_298),
            slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT),
            slot(PROCESS_VTABLE, ENTER_COMBAT_SLOT),
            slot(PROCESS_VTABLE, PROCESS_PURSUE_SLOT),
            slot(ACTOR_VTABLE, ACTOR_COMBAT_ROUND_SLOT),
            slot(0x0073_0000, ACTOR_COMBAT_ROUND_SLOT),
            slot(0x0074_0000, ENTER_COMBAT_SLOT),
            slot(0x0074_0000, COMPUTE_LAST_TIME_PROCESSED_SLOT),
            ACTOR_SET_CURRENT_TARGET,
            PLAYER_005C1A00,
            ACTOR_END_INTERRUPT_PACKAGE,
            ACTOR_REMOVE_FLEE_TARGET,
        ] {
            e.register(callee, nothing);
        }
        e.call_log = Some(vec![]);
        CombatWorld {
            e,
            this,
            actor,
            target,
            target_process,
        }
    }

    #[test]
    fn process_combat_runs_the_rounds() {
        let mut w = combat_world(true, false, 2.0);
        w.e.register(PLAYER_IS_SLEEPING_OR_RESTING, |_, _| false.into_ret());
        w.e.call(0x0090_d000, &args![w.this, w.actor]);
        assert_eq!(
            calls(&w.e, slot(ACTOR_VTABLE, ACTOR_COMBAT_ROUND_SLOT)).len(),
            2
        );
        assert_eq!(
            calls(&w.e, slot(0x0073_0000, ACTOR_COMBAT_ROUND_SLOT)).len(),
            2
        );
        assert_eq!(
            calls(&w.e, ACTOR_SET_CURRENT_TARGET)[0],
            vec![w.target, w.actor]
        );
        let entered = calls(&w.e, slot(0x0074_0000, ENTER_COMBAT_SLOT));
        assert_eq!(entered.len(), 2);
        assert_eq!(
            entered[0],
            vec![
                w.target_process,
                w.target,
                w.actor,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                0
            ]
        );
        assert_eq!(
            calls(&w.e, slot(0x0074_0000, COMPUTE_LAST_TIME_PROCESSED_SLOT)),
            vec![vec![w.target_process]]
        );
        // A busy target fights without entering combat again.
        let mut w = combat_world(true, true, 1.0);
        w.e.call(0x0090_d000, &args![w.this, w.actor]);
        assert!(calls(&w.e, slot(0x0074_0000, ENTER_COMBAT_SLOT)).is_empty());
        assert_eq!(
            calls(&w.e, slot(0x0073_0000, ACTOR_COMBAT_ROUND_SLOT)).len(),
            1
        );
    }

    #[test]
    fn process_combat_hands_over_or_waits_for_the_player() {
        let mut w = combat_world(false, false, 2.0);
        w.e.register(PLAYER_IS_SLEEPING_OR_RESTING, |_, _| false.into_ret());
        w.e.call(0x0090_d000, &args![w.this, w.actor]);
        assert_eq!(
            calls(&w.e, slot(PROCESS_VTABLE, PROCESS_SLOT_298)),
            vec![vec![w.this.addr(), w.actor, 0, 0x101, 0]]
        );
        assert!(calls(&w.e, slot(ACTOR_VTABLE, ACTOR_COMBAT_ROUND_SLOT)).is_empty());
        // The resting player as the target.
        let mut w = combat_world(true, false, 2.0);
        let player = w.target;
        w.e.set_global(PLAYER, player);
        w.e.mem.map(player & !0xfff, 0x1000);
        w.e.register(PLAYER_IS_SLEEPING_OR_RESTING, |_, _| true.into_ret());
        w.e.call(0x0090_d000, &args![w.this, w.actor]);
        assert_eq!(calls(&w.e, PLAYER_005C1A00), vec![vec![player, 0, 1]]);
        assert_eq!(
            calls(
                &w.e,
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)
            ),
            vec![vec![w.this.addr(), w.actor, u32::MAX]]
        );
        // Without a target the package's target is asked for first.
        let mut w = combat_world(true, false, 0.0);
        w.e.register(PLAYER_IS_SLEEPING_OR_RESTING, |_, _| false.into_ret());
        w.e.mem.set_u32(w.this.addr() + TARGET, 0);
        w.e.call(0x0090_d000, &args![w.this, w.actor]);
        assert_eq!(
            calls(&w.e, slot(PROCESS_VTABLE, SET_TARGET_FOR_PACKAGE_SLOT)),
            vec![vec![w.this.addr(), w.actor]]
        );
    }

    #[test]
    fn process_alarm_for_guards_and_others() {
        // A non-guard whose alarm list has an item that should be attacked.
        let mut w = combat_world(true, false, 0.0);
        w.e.register(ACTOR_GET_CLASS, |_, _| 0x9000u32.into_ret());
        w.e.register(CLASS_IS_GUARD, |_, _| false.into_ret());
        let node = w.e.mem.alloc(8);
        let item = w.e.mem.alloc(8);
        w.e.mem.set_u32(node, item);
        w.e.mem.set_u32(item + 4, 1);
        w.e.register_double(ALARM_PACKAGE_LIST, move |_, _| node.into_ret());
        w.e.register(LIST_HEAD_ITEM, |_, a| a[0].into_ret());
        w.e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        w.e.register(GET_WORD_0C, |_, _| 0x7700u32.into_ret());
        w.e.register(WORD_08_0044DDC0, |_, _| 0u32.into_ret());
        w.e.register(ACTOR_GET_SHOULD_ATTACK_ACTOR, |_, _| true.into_ret());
        w.e.call(0x0090_d240, &args![w.this, w.actor]);
        let asked = calls(&w.e, ACTOR_GET_SHOULD_ATTACK_ACTOR);
        assert_eq!(asked.len(), 1);
        assert_eq!(&asked[0][..3], &[w.actor, 0x7700, 0]);
        assert_eq!(
            calls(&w.e, slot(PROCESS_VTABLE, ENTER_COMBAT_SLOT)),
            vec![vec![
                w.this.addr(),
                w.actor,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                1,
                0
            ]]
        );
        // An item with a larger word is never asked: not found.
        let mut w = combat_world(true, false, 0.0);
        w.e.register(ACTOR_GET_CLASS, |_, _| 0x9000u32.into_ret());
        w.e.register(CLASS_IS_GUARD, |_, _| false.into_ret());
        let node = w.e.mem.alloc(8);
        let item = w.e.mem.alloc(8);
        w.e.mem.set_u32(node, item);
        w.e.mem.set_u32(item + 4, 3);
        w.e.register_double(ALARM_PACKAGE_LIST, move |_, _| node.into_ret());
        w.e.register(LIST_HEAD_ITEM, |_, a| a[0].into_ret());
        w.e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        w.e.register(GET_WORD_0C, |_, _| 0x7700u32.into_ret());
        w.e.register(WORD_08_0044DDC0, |_, _| 0u32.into_ret());
        w.e.register(ACTOR_GET_SHOULD_ATTACK_ACTOR, |_, _| true.into_ret());
        w.e.call(0x0090_d240, &args![w.this, w.actor]);
        assert!(calls(&w.e, ACTOR_GET_SHOULD_ATTACK_ACTOR).is_empty());
        assert_eq!(
            calls(
                &w.e,
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)
            ),
            vec![vec![w.this.addr(), w.actor, 3]]
        );
        // A guard far from the player in the same world space.
        for (distance, ends) in [(4000.0f64, true), (3000.0, false)] {
            let mut w = combat_world(true, false, 0.0);
            w.e.map(0x0108_8000, 0x1000);
            w.e.set_global(ALARM_GUARD_DISTANCE, 3500.0f64);
            w.e.register(ACTOR_GET_CLASS, |_, _| 0x9000u32.into_ret());
            w.e.register(CLASS_IS_GUARD, |_, _| true.into_ret());
            w.e.register_double(REFERENCE_GET_DISTANCE, move |_, _| distance.into_ret());
            w.e.register(REFERENCE_GET_WORLD_SPACE, |_, _| 0x55u32.into_ret());
            w.e.call(0x0090_d240, &args![w.this, w.actor]);
            assert_eq!(
                calls(&w.e, slot(PROCESS_VTABLE, PROCESS_PURSUE_SLOT)),
                vec![vec![w.this.addr(), w.actor, 0, 1, 0x101]]
            );
            assert_eq!(
                calls(&w.e, ACTOR_END_INTERRUPT_PACKAGE).len(),
                usize::from(ends)
            );
        }
        // Not an alarm package.
        let mut w = combat_world(true, false, 0.0);
        w.e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        w.e.call(0x0090_d240, &args![w.this, w.actor]);
        assert!(calls(&w.e, ACTOR_GET_CLASS).is_empty());
    }

    #[test]
    fn process_pursue_by_distance() {
        for (distance, slot_304, package_type, moves) in [
            (300.0f64, false, 0u32, false),
            (100.0, false, 0, true),
            (100.0, true, 0x13, true),
            (100.0, true, 0x12, false),
        ] {
            let mut w = combat_world(true, false, 0.0);
            w.e.map(0x0101_7000, 0x1000);
            w.e.set_global(PURSUE_DISTANCE, 200.0f64);
            w.e.register(MOBILE_GET_PROCESS_TYPE, |_, _| 1u32.into_ret());
            w.e.register_double(REFERENCE_GET_DISTANCE, move |_, _| distance.into_ret());
            w.e.register_double(slot(ACTOR_VTABLE, ACTOR_SLOT_304), move |_, _| {
                slot_304.into_ret()
            });
            w.e.register_double(PACKAGE_GET_TYPE, move |_, _| package_type.into_ret());
            w.e.call(0x0090_dc60, &args![w.this, w.actor, 0u32, 0u32, 0x77u32]);
            let handed = calls(&w.e, slot(PROCESS_VTABLE, PROCESS_SLOT_298));
            let moved = calls(
                &w.e,
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT),
            );
            if distance > 200.0 {
                assert_eq!(handed, vec![vec![w.this.addr(), w.actor, 0, 0x77, 1]]);
            }
            assert_eq!(moved.len(), usize::from(moves));
        }
        // A target without a process, pursued by an actor that is not alarmed.
        let mut w = combat_world(true, false, 0.0);
        w.e.map(0x0101_7000, 0x1000);
        w.e.register(MOBILE_GET_PROCESS_TYPE, |_, _| 0u32.into_ret());
        w.e.register(ACTOR_IS_ALARMED, |_, _| false.into_ret());
        w.e.register(REFERENCE_GET_DISTANCE, |_, _| 1.0f64.into_ret());
        w.e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_304), |_, _| false.into_ret());
        w.e.call(0x0090_dc60, &args![w.this, w.actor, 0u32, 0u32, 0u32]);
        assert_eq!(
            calls(&w.e, ACTOR_REMOVE_FLEE_TARGET),
            vec![vec![w.target, w.actor]]
        );
        assert_eq!(
            calls(
                &w.e,
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)
            )[0],
            vec![w.this.addr(), w.actor, 2]
        );
        // No target at all.
        let mut w = combat_world(true, false, 0.0);
        w.e.mem.set_u32(w.this.addr() + TARGET, 0);
        w.e.call(0x0090_dc60, &args![w.this, w.actor, 0u32, 0u32, 0u32]);
        assert_eq!(
            calls(
                &w.e,
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)
            ),
            vec![vec![w.this.addr(), w.actor, 1]]
        );
    }

    #[test]
    fn revert_clears_lists_and_state_when_asked() {
        for clears in [true, false] {
            let mut e = Engine::new();
            e.map(0x011d_e000, 0x1000);
            let this =
                Ptr::<LowProcess>::new(object(&mut e, PROCESS_VTABLE, 0x200, &[SET_TARGET_SLOT]));
            let base = this.addr();
            for offset in (4..0xb4).step_by(4) {
                e.mem.set_u32(base + offset, 0x4444_4444);
            }
            e.register(BASE_PROCESS_REVERT, nothing);
            e.register_double(SAVE_LOAD_TEST_0055F5B0, move |_, _| clears.into_ret());
            e.register(LIST_IS_EMPTY, |_, _| true.into_ret());
            e.register(LIST_CLEAR, nothing);
            e.register(slot(PROCESS_VTABLE, SET_TARGET_SLOT), nothing);
            e.register(REFERENCE_TEST_00440D80, |_, _| false.into_ret());
            e.register(MODIFIER_LIST_REVERT, nothing);
            e.call_log = Some(vec![]);
            e.call(0x0091_0310, &args![this, SAVE_MODIFIERS_FLAG, 0x8000u32]);
            assert_eq!(
                calls(&e, BASE_PROCESS_REVERT),
                vec![vec![base, SAVE_MODIFIERS_FLAG, 0x8000]]
            );
            assert_eq!(calls(&e, SAVE_LOAD_TEST_0055F5B0).len(), 2);
            assert_eq!(calls(&e, MODIFIER_LIST_REVERT), vec![vec![base + 0x94]]);
            if clears {
                for offset in [0x58u32, 0x64, 0x68, 0x44, 0xa4, 0x34, 0xa8, 0xac] {
                    assert_eq!(e.mem.u32(base + offset), 0, "offset {offset:#x}");
                }
                assert_eq!(calls(&e, LIST_CLEAR), vec![vec![base + 0x6c]]);
            } else {
                assert_eq!(e.mem.u32(base + 0x58), 0x4444_4444);
                assert!(calls(&e, LIST_CLEAR).is_empty());
            }
        }
    }

    /// The save/load object at `0x5000`: blocks when `blocks`, size reports when `reports`,
    /// version `version`, a record of the form `0x00aa_0001` (named `0x7777`) when
    /// `with_record`; the position answers successive values from `positions`; writes and
    /// reads are recorded as (address, size).
    fn save_rig(
        blocks: bool,
        reports: bool,
        version: u32,
        with_record: bool,
        positions: Vec<u32>,
    ) -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x5000u32);
        e.register_double(USE_SAVE_GAME_BLOCKS, move |_, _| blocks.into_ret());
        let flag = e.mem.alloc(4);
        e.mem.set_u8(flag, reports as u8);
        e.register_double(SETTING_BYTE_ADDRESS, move |_, _| flag.into_ret());
        let mut next = 0usize;
        e.register_double(SAVE_LOAD_POSITION, move |_, _| {
            let value = positions[next.min(positions.len() - 1)];
            next += 1;
            value.into_ret()
        });
        e.register_double(SAVE_LOAD_VERSION, move |_, _| version.into_ret());
        let record = if with_record { e.mem.alloc(0x10) } else { 0 };
        if with_record {
            e.mem.set_u32(record, 0x00aa_0001);
            e.mem.set_u32(record + 5, 0x0102_0304);
            e.mem.set_u8(record + 9, 0x0f);
        }
        e.register_double(SAVED_FORM_RECORD, move |_, _| record.into_ret());
        e.register_double(LOADED_FORM_RECORD, move |_, _| record.into_ret());
        let form = object(&mut e, 0x0075_0000, 0x10, &[FORM_NAME_SLOT]);
        e.register(slot(0x0075_0000, FORM_NAME_SLOT), |_, _| {
            0x7777u32.into_ret()
        });
        e.register_double(LOOKUP_FORM_BY_ID, move |_, a| {
            if a[0] == 0x00aa_0001 {
                form.into_ret()
            } else {
                (a[0] + 0x10_0000).into_ret()
            }
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        for address in [
            ERROR_REPORT,
            SAVE_DEBUG_PRINT,
            SAVE_LOAD_WRITE,
            SAVE_LOAD_READ,
            SAVE_NUMERIC_ID,
            LOAD_NUMERIC_ID,
            SAVE_LOAD_SKIP,
        ] {
            e.register(address, nothing);
        }
        e.call_log = Some(vec![]);
        e
    }

    /// `(address - base, size)` of each call to `address`.
    fn offsets(e: &Engine, address: u32, base: u32) -> Vec<(u32, u32)> {
        calls(e, address)
            .iter()
            .map(|w| (w[1].wrapping_sub(base), w[2]))
            .collect()
    }

    #[test]
    fn get_save_size_counts_by_version() {
        for (blocks, version, flags, size) in [
            (
                true,
                0x74u32,
                SAVE_MODIFIERS_FLAG,
                20u16 + 6 + 3 + 12 + 8 + 11 + 4,
            ),
            (false, 0x4b, 0, 20 + 3 + 4 + 8),
            (false, 0x30, 0, 20 + 3 + 8),
        ] {
            let mut e = save_rig(blocks, false, version, false, vec![0]);
            let this = process(&mut e);
            e.mem.set_u8(this.addr() + 0x30, 3);
            e.register(BASE_PROCESS_GET_SAVE_SIZE, |_, _| 20u32.into_ret());
            e.register(MODIFIER_LIST_GET_SAVE_SIZE, |_, _| 11u32.into_ret());
            assert_eq!(
                e.call(0x0090_f8b0, &args![this, flags, 0x22u32]).u16(),
                size
            );
            assert_eq!(
                calls(&e, BASE_PROCESS_GET_SAVE_SIZE),
                vec![vec![this.addr(), flags, 0x22]]
            );
        }
        // The report.
        let mut e = save_rig(false, true, 0, true, vec![0]);
        let this = process(&mut e);
        e.register(BASE_PROCESS_GET_SAVE_SIZE, |_, _| 20u32.into_ret());
        e.call(0x0090_f8b0, &args![this, 0u32, 0u32]);
        assert_eq!(
            calls(&e, ERROR_REPORT),
            vec![vec![
                GET_SAVE_SIZE_FORM_FORMAT,
                8,
                0x00aa_0001,
                0x7777,
                0x0102_0304,
                GET_SAVE_SIZE_LINE,
                LOW_PROCESS_SOURCE_FILE
            ]]
        );
    }

    #[test]
    fn save_game_writes_the_fields_by_version() {
        let mut e = save_rig(
            true,
            true,
            0x74,
            false,
            vec![0x100, 0x110, 0x9004, 0x150, 0x9020],
        );
        e.mem.map(0x9000, 0x100);
        let this = process(&mut e);
        let base = this.addr();
        e.mem.set_u32(base + 0x34, 0x3400);
        e.mem.set_u32(base + TARGET, 0x4000);
        e.mem.set_u32(base + GENERIC_LOCATION, 0x4400);
        e.register(BASE_PROCESS_SAVE_GAME_V1, nothing);
        e.register(MODIFIER_LIST_SAVE_GAME_V1, nothing);
        e.register(GET_WORD_0C, |_, a| (a[0] + 1).into_ret());
        let ids = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = ids.clone();
        e.register_double(SAVE_NUMERIC_ID, move |e, a| {
            record.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call(0x0090_fa70, &args![this, SAVE_MODIFIERS_FLAG, 7u32]);
        let writes = offsets(&e, SAVE_LOAD_WRITE, base);
        assert_eq!(writes.len(), 7);
        assert_eq!(writes[0].1, 4);
        assert_eq!(writes[1].1, 2);
        assert_eq!(
            &writes[2..],
            &[(0x30, 1), (0xa4, 4), (0x58, 4), (0xa8, 4), (0xac, 4)]
        );
        assert_eq!(*ids.borrow(), vec![0x3401, 0x4001, 0x4401]);
        assert_eq!(
            calls(&e, MODIFIER_LIST_SAVE_GAME_V1),
            vec![vec![base + 0x94]]
        );
        assert_eq!(
            calls(&e, ERROR_REPORT),
            vec![vec![
                SAVE_GAME_FORMAT,
                0x40,
                SAVE_GAME_LINE,
                LOW_PROCESS_SOURCE_FILE
            ]]
        );
        assert_eq!(e.mem.u16(0x9004), 0x1c);
        // An acquire object hides the target; an old version writes fewer fields.
        let mut e = save_rig(false, false, 0x30, false, vec![0]);
        let this = process(&mut e);
        let base = this.addr();
        e.mem.set_u32(base + TARGET, 0x4000);
        e.mem.set_u32(base + ACQUIRE_OBJECT, 0x6000);
        e.register(BASE_PROCESS_SAVE_GAME_V1, nothing);
        let ids = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = ids.clone();
        e.register_double(SAVE_NUMERIC_ID, move |e, a| {
            record.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call(0x0090_fa70, &args![this, 0u32, 0u32]);
        assert_eq!(
            offsets(&e, SAVE_LOAD_WRITE, base),
            vec![(0x30, 1), (0xac, 4)]
        );
        assert_eq!(*ids.borrow(), vec![0, 0]);
    }

    #[test]
    fn load_game_reads_the_fields_by_version() {
        let mut e = save_rig(true, false, 0x74, false, vec![0x400, 0x420]);
        let this =
            Ptr::<LowProcess>::new(object(&mut e, PROCESS_VTABLE, 0x200, &[SET_TARGET_SLOT]));
        let base = this.addr();
        e.register(BASE_PROCESS_LOAD_GAME, nothing);
        e.register(MODIFIER_LIST_LOAD_GAME, nothing);
        e.register(slot(PROCESS_VTABLE, SET_TARGET_SLOT), nothing);
        e.register(SAVE_LOAD_READ, |e, a| {
            if a[2] == 2 {
                e.mem.set_u16(a[1], 0x20);
            } else if a[2] == 4 && e.mem.u32(a[1]) == 0 {
                e.mem.set_u32(a[1], BLOCK_MARKER);
            }
            Ret::default()
        });
        let mut next = 0x10u32;
        e.register_double(LOAD_NUMERIC_ID, move |e, a| {
            e.mem.set_u32(a[1], next);
            next += 0x10;
            Ret::default()
        });
        e.call(0x0090_fd80, &args![this, SAVE_MODIFIERS_FLAG, 2u32, 3u32]);
        assert_eq!(
            calls(&e, BASE_PROCESS_LOAD_GAME),
            vec![vec![base, SAVE_MODIFIERS_FLAG, 2, 3]]
        );
        let reads: Vec<(u32, u32)> = offsets(&e, SAVE_LOAD_READ, base)
            .into_iter()
            .filter(|(offset, _)| *offset < 0x200)
            .collect();
        assert_eq!(
            reads,
            vec![(0x30, 1), (0xa4, 4), (0x58, 4), (0xa8, 4), (0xac, 4)]
        );
        assert_eq!(e.mem.u32(base + 0x34), 0x10 + 0x10_0000);
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, SET_TARGET_SLOT)),
            vec![vec![base, 0x20]]
        );
        assert_eq!(e.mem.u32(base + GENERIC_LOCATION), 0x30);
        assert_eq!(calls(&e, MODIFIER_LIST_LOAD_GAME), vec![vec![base + 0x94]]);
        assert!(calls(&e, SAVE_DEBUG_PRINT).is_empty());
    }

    #[test]
    fn load_game_old_versions_convert_the_date_and_skip_the_block() {
        let mut e = save_rig(false, false, 0x30, false, vec![0]);
        let this =
            Ptr::<LowProcess>::new(object(&mut e, PROCESS_VTABLE, 0x200, &[SET_TARGET_SLOT]));
        let base = this.addr();
        e.map(0x0101_6000, 0x1000);
        e.set_global(DATE_HUNDRED, 100.0f32);
        e.map(0x0102_2000, 0x1000);
        e.set_global(DATE_TEN_THOUSAND, 10000.0f32);
        e.register(BASE_PROCESS_LOAD_GAME, nothing);
        e.register(slot(PROCESS_VTABLE, SET_TARGET_SLOT), nothing);
        // The old date 2281.0512 (year 2281, month 5, day 12) as one float; the count 3.
        e.register(SAVE_LOAD_READ, |e, a| {
            match a[2] {
                4 => e.mem.set_f32(a[1], 22_810_512.0),
                2 => e.mem.set_u16(a[1], 3),
                _ => {}
            }
            Ret::default()
        });
        e.register(FLOAT_PAIR_004B1500, |_, a| {
            let value = f64::from(f32::from_bits(a[0]));
            let by = f64::from(f32::from_bits(a[1]));
            (value % by).into_ret()
        });
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        e.register(LOAD_NUMERIC_ID, nothing);
        e.call(0x0090_fd80, &args![this, OLD_EXTRA_BLOCK_FLAG, 0u32, 0u32]);
        let date = e.mem.u32(base + DATE_LAST_PROCESSED);
        assert_eq!(date & 0x1ff, 12);
        assert_eq!((date >> 9) & 0xf, 5);
        assert_eq!(date >> 13, 2281);
        assert_eq!(calls(&e, SAVE_LOAD_SKIP), vec![vec![0x5000, 24]]);
        // Version 0x30: no essential timer, item, activate count or death time.
        let reads: Vec<(u32, u32)> = offsets(&e, SAVE_LOAD_READ, base)
            .into_iter()
            .filter(|(offset, _)| *offset < 0x200)
            .collect();
        assert_eq!(reads, vec![(0x30, 1), (0xac, 4)]);
    }

    #[test]
    fn load_game_reports_header_and_size_errors() {
        for (with_record, end, format, by) in [
            (true, 0x428u32, OVERRUN_FORM_FORMAT, 8u32),
            (true, 0x41c, UNDERRUN_FORM_FORMAT, 4),
            (false, 0x428, OVERRUN_FORMAT, 8),
            (false, 0x41c, UNDERRUN_FORMAT, 4),
        ] {
            let mut e = save_rig(true, false, 0x74, with_record, vec![0x400, end]);
            let this =
                Ptr::<LowProcess>::new(object(&mut e, PROCESS_VTABLE, 0x200, &[SET_TARGET_SLOT]));
            e.register(BASE_PROCESS_LOAD_GAME, nothing);
            e.register(slot(PROCESS_VTABLE, SET_TARGET_SLOT), nothing);
            e.register(SAVE_LOAD_READ, |e, a| {
                if a[2] == 2 {
                    e.mem.set_u16(a[1], 0x20);
                }
                Ret::default()
            });
            e.call(0x0090_fd80, &args![this, 0u32, 0u32, 0u32]);
            let prints = calls(&e, SAVE_DEBUG_PRINT);
            assert_eq!(prints.len(), 2);
            if with_record {
                assert_eq!(
                    prints[0],
                    vec![
                        BAD_HEADER_FORM_FORMAT,
                        LOW_PROCESS_SOURCE_FILE,
                        LOAD_HEADER_LINE,
                        0x00aa_0001,
                        0x7777,
                        0x0f,
                        0x0102_0304
                    ]
                );
                assert_eq!(&prints[1][4..], &[0x00aa_0001, 0x7777, 0x0f, 0x0102_0304]);
            } else {
                assert_eq!(
                    prints[0],
                    vec![
                        BAD_HEADER_FORMAT,
                        LOW_PROCESS_SOURCE_FILE,
                        LOAD_HEADER_LINE,
                        0x74
                    ]
                );
                assert_eq!(&prints[1][4..], &[0x74]);
            }
            assert_eq!(
                &prints[1][..4],
                &[format, by, LOW_PROCESS_SOURCE_FILE, LOAD_SIZE_LINE]
            );
        }
    }

    #[test]
    fn load_game_ov2_reads_the_buffer() {
        for (version, flagged) in [(5u32, true), (6, false)] {
            let mut e = Engine::new();
            let this = process(&mut e);
            let base = this.addr();
            let buffer = object(&mut e, 0x0076_0000, 0x10, &[0]);
            e.register_double(slot(0x0076_0000, 0), move |_, _| version.into_ret());
            e.register(BASE_PROCESS_LOAD_GAME_OV2, nothing);
            e.register(BUFFER_LOAD_BYTES, nothing);
            let mut next = 0x100u32;
            e.register_double(BUFFER_LOAD_FORM_ID, move |_, _| {
                next += 1;
                next.into_ret()
            });
            e.register(LOOKUP_FORM_BY_ID, |_, a| (a[0] << 4).into_ret());
            e.register(DYNAMIC_CAST, |_, a| (a[0] + 2).into_ret());
            e.register(BUFFER_LOAD_FORM_ID_INTO, |e, a| {
                e.mem.set_u32(a[1], 0x77);
                Ret::default()
            });
            e.register(BUFFER_LOAD_VARIABLE_SIZE, |_, _| 2u32.into_ret());
            e.register(LIST_ADD_ITEM, nothing);
            e.register(COMBAT_TIMER_LOAD_GAME, nothing);
            e.register(SAVE_FLAGS_00428110, |_, a| (a[1] + 1).into_ret());
            e.register_double(FLAGS_HAVE_004280F0, move |_, a| {
                (a[1] == SAVE_MODIFIERS_FLAG && flagged).into_ret()
            });
            e.register(MODIFIER_LIST_LOAD_GAME_OV2, nothing);
            e.call_log = Some(vec![]);
            e.call(0x0091_0630, &args![this, buffer]);
            let loads: Vec<(u32, u32)> = offsets(&e, BUFFER_LOAD_BYTES, base)
                .into_iter()
                .filter(|(offset, _)| *offset < 0x200)
                .collect();
            assert_eq!(
                loads,
                vec![
                    (0x30, 1),
                    (0xa4, 4),
                    (0x58, 4),
                    (0xa8, 4),
                    (0xac, 4),
                    (0xb0, 1),
                    (0x50, 2)
                ]
            );
            let dropped = calls(&e, BUFFER_LOAD_BYTES).len() - loads.len();
            assert_eq!(dropped, if version < 6 { 2 } else { 0 });
            assert_eq!(e.mem.u32(base + 0x34), (0x101 << 4) + 2);
            assert_eq!(e.mem.u32(base + TARGET_FORM_LIST), (0x102 << 4) + 2);
            assert_eq!(
                calls(&e, DYNAMIC_CAST)[1],
                vec![0x1020, 0, FORM_TYPE_DESCRIPTOR, LIST_FORM_DESCRIPTOR, 0]
            );
            let into: Vec<u32> = calls(&e, BUFFER_LOAD_FORM_ID_INTO)
                .iter()
                .map(|w| w[1].wrapping_sub(base))
                .collect();
            assert_eq!(&into[..4], &[0x40, 0x44, 0x48, 0x54]);
            assert_eq!(calls(&e, LIST_ADD_ITEM).len(), 2);
            assert_eq!(calls(&e, LIST_ADD_ITEM)[0][0], base + 0x6c);
            assert_eq!(
                calls(&e, COMBAT_TIMER_LOAD_GAME),
                vec![vec![base + 0x38, buffer]]
            );
            assert_eq!(
                calls(&e, MODIFIER_LIST_LOAD_GAME_OV2).len(),
                usize::from(flagged)
            );
        }
    }

    #[test]
    fn init_load_game_ov2_resolves_ids_and_prunes_locations() {
        let mut e = Engine::new();
        let this =
            Ptr::<LowProcess>::new(object(&mut e, PROCESS_VTABLE, 0x200, &[SET_TARGET_SLOT]));
        let base = this.addr();
        e.mem.set_u32(base + TARGET, 0x11);
        e.mem.set_u32(base + GENERIC_LOCATION, 0x22);
        e.mem.set_u32(base + 0x48, 0);
        e.mem.set_u32(base + 0x54, 0x55);
        // GenericLocationsList: the head (id 0x66, resolves), then 0x77 (no reference), then
        // 0x88 (resolves).
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(base + 0x6c, 0x66);
        e.mem.set_u32(base + 0x70, second);
        e.mem.set_u32(second, 0x77);
        e.mem.set_u32(second + 4, third);
        e.mem.set_u32(third, 0x88);
        e.register(BASE_PROCESS_INIT_LOAD_GAME_OV2, nothing);
        e.register(LOOKUP_FORM_BY_ID, |_, a| (a[0] << 4).into_ret());
        e.register(DYNAMIC_CAST, |_, a| {
            if a[0] == 0x770 {
                0u32.into_ret()
            } else {
                (a[0] + 1).into_ret()
            }
        });
        e.register(slot(PROCESS_VTABLE, SET_TARGET_SLOT), nothing);
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_HEAD_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_SET_ITEM_00726C60, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(LIST_REMOVE_ITEM, |e, a| {
            // Unlink the node after `a[0]`.
            let removed = e.mem.u32(a[0] + 4);
            let after = e.mem.u32(removed + 4);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        e.register(LIST_REMOVE_HEAD, nothing);
        e.call_log = Some(vec![]);
        e.call(0x0091_0820, &args![this, 0x6000u32]);
        assert_eq!(
            calls(&e, BASE_PROCESS_INIT_LOAD_GAME_OV2),
            vec![vec![base, 0x6000]]
        );
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, SET_TARGET_SLOT)),
            vec![vec![base, 0x111]]
        );
        assert_eq!(e.mem.u32(base + TARGET), 0);
        assert_eq!(e.mem.u32(base + GENERIC_LOCATION), 0x221);
        assert_eq!(e.mem.u32(base + 0x48), 0);
        assert_eq!(e.mem.u32(base + 0x54), 0x551);
        assert_eq!(e.mem.u32(base + 0x6c), 0x661);
        assert_eq!(e.mem.u32(base + 0x70), third);
        assert_eq!(e.mem.u32(third), 0x881);
        assert_eq!(calls(&e, LIST_REMOVE_ITEM), vec![vec![base + 0x6c, second]]);
        assert!(calls(&e, LIST_REMOVE_HEAD).is_empty());
    }

    /// The world of `ShouldUpdateCombatControllerOrEndCombat`: the actor's extra list
    /// (`actor + 0x44`) holds the package `package` (0 for none) at index 2; the package
    /// types come from the word at `+0` of each package; the next package is `next`.
    fn end_combat_world(package_kind: u32, next: u32) -> (Engine, Ptr<LowProcess>, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011a_3000, 0x1000);
        let this = Ptr::<LowProcess>::new(object(
            &mut e,
            PROCESS_VTABLE,
            0x200,
            &[
                PROCESS_SLOT_328,
                PACKAGE_DONE_SLOT,
                SET_PACKAGE_DONE_ONCE_SLOT,
                SET_TARGET_ACTIVATED_SLOT,
                SET_CURRENT_ACTION_COMPLETE_SLOT,
                GET_TARGET_FOR_PACKAGE_SLOT,
                GET_LOCATION_FOR_PACKAGE_SLOT,
                IS_PACKAGE_DONE_ONCE_SLOT,
                IS_TARGET_ACTIVATED_SLOT,
                GET_CURRENT_ACTION_COMPLETE_SLOT,
                START_NEW_PACKAGE_SLOT,
            ],
        ));
        let actor = object(&mut e, ACTOR_VTABLE, 0x100, &[ACTOR_SLOT_48]);
        let package = e.mem.alloc(0x20);
        e.mem.set_u32(package, package_kind);
        e.register(REFERENCE_GET_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register_double(EXTRA_GET_PACKAGE, move |_, _| package.into_ret());
        e.register(EXTRA_GET_PACKAGE_INDEX, |_, _| 2u32.into_ret());
        e.register(PACKAGE_GET_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
        // A procedure list where index 2 is not the done procedure.
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(PROCEDURE_LISTS, list);
        e.register(PACKAGE_GET_PROCEDURE_TYPE, |_, _| 0u32.into_ret());
        e.register(PROCEDURE_COUNT, |_, _| 5u32.into_ret());
        e.register(EXTRA_GET_ACTOR_PACKAGE_DATA, |_, _| 0u32.into_ret());
        e.register_double(ACTOR_GET_PACKAGE, move |_, _| next.into_ret());
        e.register(PACKAGE_GET_FLAGS, |_, _| 0x0200_0000u32.into_ret());
        e.register(PROCESS_GET_0045CD60, |_, _| 0u32.into_ret());
        e.register(slot(PROCESS_VTABLE, GET_TARGET_FOR_PACKAGE_SLOT), |_, _| {
            0u32.into_ret()
        });
        e.register(
            slot(PROCESS_VTABLE, GET_LOCATION_FOR_PACKAGE_SLOT),
            |_, _| 0x8800u32.into_ret(),
        );
        e.register(slot(PROCESS_VTABLE, IS_PACKAGE_DONE_ONCE_SLOT), |_, _| {
            1u32.into_ret()
        });
        e.register(slot(PROCESS_VTABLE, IS_TARGET_ACTIVATED_SLOT), |_, _| {
            0u32.into_ret()
        });
        e.register(
            slot(PROCESS_VTABLE, GET_CURRENT_ACTION_COMPLETE_SLOT),
            |_, _| 1u32.into_ret(),
        );
        e.register(CREATE_ACTOR_PACKAGE_DATA, |_, _| 0x9900u32.into_ret());
        for callee in [
            slot(PROCESS_VTABLE, PROCESS_SLOT_328),
            slot(PROCESS_VTABLE, PACKAGE_DONE_SLOT),
            slot(PROCESS_VTABLE, SET_PACKAGE_DONE_ONCE_SLOT),
            slot(PROCESS_VTABLE, SET_TARGET_ACTIVATED_SLOT),
            slot(PROCESS_VTABLE, SET_CURRENT_ACTION_COMPLETE_SLOT),
            slot(PROCESS_VTABLE, START_NEW_PACKAGE_SLOT),
            slot(ACTOR_VTABLE, ACTOR_SLOT_48),
            EXTRA_SET_PACKAGE_INDEX,
            SET_ACTION_FLAG,
            EXTRA_SET_ACTION_COMPLETE,
            EXTRA_SET_PACKAGE,
            EXTRA_SET_ACTOR_PACKAGE_DATA,
        ] {
            e.register(callee, nothing);
        }
        e.call_log = Some(vec![]);
        (e, this, actor, package)
    }

    #[test]
    fn end_combat_without_a_package_or_when_the_controller_should_stop() {
        let (mut e, this, actor, _) = end_combat_world(0, 0);
        e.register(EXTRA_GET_PACKAGE, |_, _| 0u32.into_ret());
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x55);
        assert!(!e.call(0x0090_7ab0, &args![this, actor, out]).bool());
        assert_eq!(e.mem.u32(out), 0);
        // Type 2 at a procedure that is not done stops the controller.
        let (mut e, this, actor, _) = end_combat_world(2, 0);
        e.mem.set_u32(out, 0x55);
        assert!(e.call(0x0090_7ab0, &args![this, actor, out]).bool());
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, PROCESS_SLOT_328)),
            vec![vec![this.addr(), 1]]
        );
    }

    #[test]
    fn end_combat_finishes_the_package_and_starts_the_next() {
        let (mut e, this, actor, package) = end_combat_world(0, 0);
        let next = e.mem.alloc(0x20);
        e.mem.set_u32(next, 0xd);
        e.register_double(ACTOR_GET_PACKAGE, move |_, _| next.into_ret());
        let out = e.mem.alloc(4);
        assert!(!e.call(0x0090_7ab0, &args![this, actor, out]).bool());
        assert_eq!(e.mem.u32(out), next);
        assert_eq!(
            calls(&e, EXTRA_SET_PACKAGE_INDEX),
            vec![vec![actor + 0x44, 4]]
        );
        assert_eq!(
            calls(&e, SET_ACTION_FLAG),
            vec![vec![package, actor + 0x44, 0x400]]
        );
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, PACKAGE_DONE_SLOT)),
            vec![vec![this.addr(), actor, package]]
        );
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, SET_PACKAGE_DONE_ONCE_SLOT)),
            vec![vec![this.addr(), 1], vec![this.addr(), 0]]
        );
        assert_eq!(
            calls(&e, EXTRA_SET_PACKAGE),
            vec![vec![actor + 0x44, next, 0, 0x8800, 1, 0, 1]]
        );
        assert_eq!(calls(&e, CREATE_ACTOR_PACKAGE_DATA), vec![vec![0xd]]);
        assert_eq!(
            calls(&e, EXTRA_SET_ACTOR_PACKAGE_DATA),
            vec![vec![actor + 0x44, 0x9900]]
        );
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, START_NEW_PACKAGE_SLOT)),
            vec![vec![this.addr(), actor, next]]
        );
        assert_eq!(
            calls(&e, slot(ACTOR_VTABLE, ACTOR_SLOT_48)),
            vec![vec![actor, 0x8000_0000]]
        );
        // No next package: the word is 0.
        let (mut e, this, actor, _) = end_combat_world(0, 0);
        e.mem.set_u32(out, 0x55);
        assert!(!e.call(0x0090_7ab0, &args![this, actor, out]).bool());
        assert_eq!(e.mem.u32(out), 0);
        assert!(calls(&e, EXTRA_SET_PACKAGE).is_empty());
    }

    /// One acquire entry (reference, form) at the head of `TempObjectList`; the reference is
    /// an actor when `is_actor`; the owner (`00567790`) answers `owner`.
    struct AcquireWorld {
        e: Engine,
        this: Ptr<LowProcess>,
        actor: u32,
        entry: u32,
        reference: u32,
    }

    const REFERENCE_VTABLE: u32 = 0x0077_0000;

    fn acquire_world(is_actor: bool, owner: u32, running: u32) -> AcquireWorld {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        let this = Ptr::<LowProcess>::new(object(
            &mut e,
            PROCESS_VTABLE,
            0x200,
            &[GET_PACKAGE_RUNNING_SLOT],
        ));
        let actor = object(&mut e, ACTOR_VTABLE, 0x10, &[ACTOR_SLOT_21C]);
        let reference = object(
            &mut e,
            REFERENCE_VTABLE,
            0x10,
            &[REFERENCE_SLOT_100, REFERENCE_SLOT_218, ACTOR_SLOT_22C],
        );
        let entry = e.mem.alloc(0x1c);
        e.mem.set_u32(entry, reference);
        e.mem.set_u32(entry + 4, 0x6600);
        e.mem.set_u32(entry + ACQUIRE_STATE, 1);
        e.mem.set_u32(this.addr() + TEMP_OBJECT_LIST, entry);
        e.register_double(
            slot(PROCESS_VTABLE, GET_PACKAGE_RUNNING_SLOT),
            move |_, _| running.into_ret(),
        );
        e.register(LIST_HEAD_ITEM, |_, a| a[0].into_ret());
        e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register_double(slot(REFERENCE_VTABLE, REFERENCE_SLOT_100), move |_, _| {
            is_actor.into_ret()
        });
        e.register(slot(REFERENCE_VTABLE, REFERENCE_SLOT_218), |_, _| {
            false.into_ret()
        });
        e.register(slot(REFERENCE_VTABLE, ACTOR_SLOT_22C), |_, _| {
            false.into_ret()
        });
        e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_21C), |_, _| false.into_ret());
        e.register(ACTOR_TEST_00566950, |_, _| false.into_ret());
        e.register(ACTOR_TEST_00493BB0, |_, _| false.into_ret());
        e.register_double(REFERENCE_GET_OWNER, move |_, _| owner.into_ret());
        e.register(REFERENCE_GET_BASE_FORM, |_, _| 0x5000u32.into_ret());
        e.register(REFERENCE_GET_DISTANCE, |_, _| 123.75f64.into_ret());
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        e.register(FORM_VALUE, |_, _| 10u32.into_ret());
        e.register(ACTOR_GET_GOLD_AMOUNT, |_, _| 5u32.into_ret());
        e.register(RUNNING_TEST_008E30E0, |_, _| false.into_ret());
        e.register(RUNNING_TEST_008E3120, |_, _| true.into_ret());
        e.register(RUNNING_TEST_008E3100, |_, _| false.into_ret());
        e.register(ACTOR_HAS_OBJECTS, |_, _| false.into_ret());
        e.register(REFERENCE_GET_LOCK, |_, _| 0u32.into_ret());
        for callee in [
            REFERENCE_SET_TARGETED,
            OBJECT_LIST_ADD_00905820,
            DELETE_EMPTY_DESTRUCTOR,
            LIST_CLEAR,
        ] {
            e.register(callee, nothing);
        }
        e.call_log = Some(vec![]);
        AcquireWorld {
            e,
            this,
            actor,
            entry,
            reference,
        }
    }

    #[test]
    fn evaluate_acquire_list_keeps_own_objects_with_their_distance() {
        let mut w = acquire_world(false, 0, 0);
        w.e.call(0x0090_7d90, &args![w.this, w.actor]);
        assert_eq!(w.e.mem.u32(w.entry + ACQUIRE_DISTANCE), 123);
        assert_eq!(
            calls(&w.e, REFERENCE_SET_TARGETED),
            vec![vec![w.reference, 1]]
        );
        assert_eq!(
            calls(&w.e, OBJECT_LIST_ADD_00905820)[0][0],
            w.this.addr() + OBJECT_LIST
        );
        assert_eq!(
            calls(&w.e, LIST_CLEAR),
            vec![vec![w.this.addr() + TEMP_OBJECT_LIST]]
        );
        assert!(calls(&w.e, FORM_VALUE).is_empty());
    }

    #[test]
    fn evaluate_acquire_list_wanted_owned_object() {
        // An object owned by someone else, wanted by the running package: state 1 becomes 4
        // and, without the key and with a lock, the entry is dropped.
        let mut w = acquire_world(false, 0x7000, 0x4000);
        w.e.register(REFERENCE_GET_LOCK, |_, _| 0x99u32.into_ret());
        w.e.call(0x0090_7d90, &args![w.this, w.actor]);
        assert_eq!(w.e.mem.u32(w.entry + ACQUIRE_STATE), 4);
        assert_eq!(w.e.mem.u8(w.entry + ACQUIRE_FLAG_8), 1);
        assert_eq!(w.e.mem.u8(w.entry + ACQUIRE_FLAG_9), 0);
        assert_eq!(
            calls(&w.e, ACTOR_HAS_OBJECTS)[0][..5],
            [w.actor, 0, 0, 1, 0]
        );
        assert_eq!(calls(&w.e, DELETE_EMPTY_DESTRUCTOR), vec![vec![w.entry, 1]]);
        assert!(calls(&w.e, REFERENCE_SET_TARGETED).is_empty());
        // Another state: 3, kept.
        let mut w = acquire_world(false, 0x7000, 0x4000);
        w.e.mem.set_u32(w.entry + ACQUIRE_STATE, 2);
        w.e.call(0x0090_7d90, &args![w.this, w.actor]);
        assert_eq!(w.e.mem.u32(w.entry + ACQUIRE_STATE), 3);
        assert_eq!(calls(&w.e, REFERENCE_SET_TARGETED).len(), 1);
    }

    #[test]
    fn evaluate_acquire_list_unwanted_owned_object_is_dropped() {
        // Nothing wants it: the non-actor reference is dropped.
        let mut w = acquire_world(false, 0x7000, 0);
        w.e.call(0x0090_7d90, &args![w.this, w.actor]);
        assert_eq!(calls(&w.e, DELETE_EMPTY_DESTRUCTOR), vec![vec![w.entry, 1]]);
        // An actor owned by another: state 5, kept.
        let mut w = acquire_world(true, 0x7000, 0);
        w.e.call(0x0090_7d90, &args![w.this, w.actor]);
        assert_eq!(w.e.mem.u32(w.entry + ACQUIRE_STATE), 5);
        assert_eq!(calls(&w.e, REFERENCE_SET_TARGETED).len(), 1);
    }

    #[test]
    fn evaluate_acquire_list_buys_from_a_seller() {
        let mut w = acquire_world(true, 0x7000, 0x4000);
        w.e.register(RUNNING_TEST_008E30E0, |_, _| true.into_ret());
        w.e.register(ACTOR_GET_GOLD_AMOUNT, |_, _| 50u32.into_ret());
        w.e.register(slot(REFERENCE_VTABLE, REFERENCE_SLOT_218), |_, _| {
            true.into_ret()
        });
        w.e.register(BASE_OBJECT_004181E0, |_, _| 0x3300u32.into_ret());
        w.e.register(AI_FORM_SELL_BUYS_ITEM, |_, _| true.into_ret());
        w.e.call(0x0090_7d90, &args![w.this, w.actor]);
        assert_eq!(
            calls(&w.e, AI_FORM_SELL_BUYS_ITEM),
            vec![vec![0x3390, 0x6600]]
        );
        assert_eq!(w.e.mem.u32(w.entry + ACQUIRE_STATE), 2);
        assert_eq!(w.e.mem.u32(w.entry + ACQUIRE_DISTANCE), 123);
        assert_eq!(calls(&w.e, REFERENCE_SET_TARGETED).len(), 1);
    }

    /// The location callback world: the reference (its slot `+0x100` answers `is_actor`, its
    /// base form `0x5500` of type `form_type`), the actor (slot `+0x1a0` false) whose
    /// process (`0x4000`-sized block) is a `LowProcess` with `eFormType` `wanted_type`.
    fn location_world(is_actor: bool, form_type: u32, wanted_type: u32) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let reference = object(
            &mut e,
            REFERENCE_VTABLE,
            0x20,
            &[REFERENCE_SLOT_100, ACTOR_SLOT_22C],
        );
        let actor = object(&mut e, ACTOR_VTABLE, 0x20, &[ACTOR_SLOT_1A0]);
        let process = e.mem.alloc(0x100);
        e.mem.set_u32(process + FORM_TYPE_FOR_LOCATION, wanted_type);
        for callee in [
            REFERENCE_TEST_00440D80,
            REFERENCE_TEST_004077C0,
            REFERENCE_TEST_00440DA0,
        ] {
            e.register(callee, |_, _| false.into_ret());
        }
        e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_1A0), |_, _| false.into_ret());
        e.register(slot(REFERENCE_VTABLE, ACTOR_SLOT_22C), |_, _| {
            false.into_ret()
        });
        e.register_double(slot(REFERENCE_VTABLE, REFERENCE_SLOT_100), move |_, _| {
            is_actor.into_ret()
        });
        e.register_double(ACTOR_GET_PROCESS, move |_, _| process.into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(REFERENCE_GET_BASE_FORM, |_, _| 0x5500u32.into_ret());
        e.register_double(FORM_GET_TYPE, move |_, _| form_type.into_ret());
        e.register(FORM_MATCHES_PACKAGE_OBJECT_TYPE, |_, a| {
            (a[0] == 0x5500 || a[0] == 0x6600).into_ret()
        });
        e.register(REFERENCE_SET_TARGETED, nothing);
        e.register(OBJECT_LIST_ADD_00905820, nothing);
        e.register(REFERENCE_ITEM_COUNT, |_, _| 2u32.into_ret());
        e.register(REFERENCE_ITEM_AT, |_, a| (0x7000 + a[1]).into_ret());
        e.register(ITEM_GET_WORN, |_, a| (a[0] == 0x7001).into_ret());
        e.register(WORD_08_0044DDC0, |_, _| 0x6600u32.into_ret());
        e.register(ITEM_DELETE, nothing);
        e.call_log = Some(vec![]);
        (e, reference, actor, process)
    }

    #[test]
    fn location_callback_takes_matching_references() {
        let (mut e, reference, actor, process) = location_world(false, 0x10, 7);
        assert!(!e.call(0x0090_e660, &args![reference, actor]).bool());
        assert_eq!(calls(&e, REFERENCE_SET_TARGETED), vec![vec![reference, 1]]);
        assert_eq!(
            calls(&e, OBJECT_LIST_ADD_00905820)[0][0],
            process + TEMP_LOC_LIST
        );
        assert!(calls(&e, REFERENCE_ITEM_COUNT).is_empty());
        // An actor reference also offers its unworn items: one more add, both items deleted.
        let (mut e, reference, actor, _) = location_world(true, 0x10, 7);
        e.call(0x0090_e660, &args![reference, actor]);
        assert_eq!(calls(&e, REFERENCE_SET_TARGETED).len(), 2);
        assert_eq!(
            calls(&e, ITEM_DELETE),
            vec![vec![0x7000, 1], vec![0x7001, 1]]
        );
        // Rejected references, a missing actor and a missing process.
        let (mut e, _, actor, _) = location_world(false, 0x10, 7);
        assert!(!e.call(0x0090_e660, &args![0u32, actor]).bool());
        let (mut e, reference, _, _) = location_world(false, 0x10, 7);
        e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_1A0), |_, _| true.into_ret());
        e.register(ACTOR_TEST_008859E0, |_, _| false.into_ret());
        let actor = object(&mut e, ACTOR_VTABLE, 0x20, &[ACTOR_SLOT_1A0]);
        assert!(!e.call(0x0090_e660, &args![reference, actor]).bool());
        let (mut e, reference, actor, _) = location_world(false, 0x10, 7);
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0090_e660, &args![reference, actor]).bool());
        assert!(calls(&e, REFERENCE_SET_TARGETED).is_empty());
    }

    #[test]
    fn location_callback_by_object_or_when_unset() {
        // No type: the object must be pObjectForLocation.
        let (mut e, reference, actor, process) = location_world(false, 0x1b, 0);
        e.mem.set_u32(process + OBJECT_FOR_LOCATION, 0x5500);
        e.call(0x0090_e660, &args![reference, actor]);
        // The reference matches the object; the unworn item's object (0x6600) does not.
        assert_eq!(calls(&e, REFERENCE_SET_TARGETED).len(), 1);
        // Neither set: every reference and every unworn item.
        let (mut e, reference, actor, _) = location_world(false, 0x1b, 0);
        e.call(0x0090_e660, &args![reference, actor]);
        assert_eq!(calls(&e, REFERENCE_SET_TARGETED).len(), 2);
    }

    #[test]
    fn destination_reference_by_procedure() {
        let mut e = Engine::new();
        e.map(0x011a_3000, 0x1000);
        let this = Ptr::<LowProcess>::new(object(
            &mut e,
            PROCESS_VTABLE,
            0x200,
            &[GET_CURRENT_PACKAGE_SLOT, GET_PROCEDURE_INDEX_RUNNING_SLOT],
        ));
        let base = this.addr();
        let list = e.mem.alloc(0x100);
        e.mem.set_u32(PROCEDURE_LISTS, list);
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0x4000u32.into_ret()
        });
        e.register(
            slot(PROCESS_VTABLE, GET_PROCEDURE_INDEX_RUNNING_SLOT),
            |_, _| 0u32.into_ret(),
        );
        e.register(PACKAGE_IS_INTERRUPT_PACKAGE, |_, _| false.into_ret());
        e.register(PACKAGE_GET_PROCEDURE_TYPE, |_, _| 0u32.into_ret());
        e.register(PACKAGE_LOCATION_0055B980, |_, _| 0x4100u32.into_ret());
        e.register(LOCATION_GET_REFERENCE, |_, _| 0x4200u32.into_ret());
        e.register(ACTOR_GET_PROCESS, |_, _| 0u32.into_ret());
        let actor = 0x3000u32;
        e.mem.set_u32(base + TARGET, 0x5000);
        for (procedure, expected) in [(0u32, 0x4200u32), (1, 0x5000), (10, 0x5000), (54, 0x4200)] {
            e.mem.set_u32(list, procedure);
            assert_eq!(
                e.call(0x0090_e970, &args![this, actor, 1u32]).u32(),
                expected,
                "procedure {procedure}"
            );
        }
        // Procedure 1 without a target: the actor; above 0x36: the target.
        e.mem.set_u32(base + TARGET, 0);
        e.mem.set_u32(list, 1);
        assert_eq!(e.call(0x0090_e970, &args![this, actor, 1u32]).u32(), actor);
        e.mem.set_u32(list, 0x37);
        assert_eq!(e.call(0x0090_e970, &args![this, actor, 1u32]).u32(), 0);
        // Case 0 without a location reference: the actor.
        e.register(LOCATION_GET_REFERENCE, |_, _| 0u32.into_ret());
        e.mem.set_u32(list, 0);
        assert_eq!(e.call(0x0090_e970, &args![this, actor, 1u32]).u32(), actor);
        // Without the flag an interrupt package is replaced by the extra one (none here).
        e.register(PACKAGE_IS_INTERRUPT_PACKAGE, |_, _| true.into_ret());
        e.register(REFERENCE_GET_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(EXTRA_GET_PACKAGE, |_, _| 0u32.into_ret());
        e.register(EXTRA_GET_PACKAGE_INDEX, |_, _| 0u32.into_ret());
        e.register(EXTRA_GET_PACKAGE_EXTRA_TARGET, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0090_e970, &args![this, actor, 0u32]).u32(), 0);
    }

    #[test]
    fn destination_radius_by_procedure() {
        let mut e = Engine::new();
        e.map(0x011a_3000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.set_global(DEFAULT_DESTINATION_RADIUS, 128.0f32);
        e.map(0x0101_8000, 0x1000);
        e.set_global(DEFAULT_TARGET_RADIUS, 30.0f32);
        let this = Ptr::<LowProcess>::new(object(
            &mut e,
            PROCESS_VTABLE,
            0x200,
            &[GET_CURRENT_PACKAGE_SLOT, GET_PROCEDURE_INDEX_RUNNING_SLOT],
        ));
        let list = e.mem.alloc(0x100);
        e.mem.set_u32(PROCEDURE_LISTS, list);
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0x4000u32.into_ret()
        });
        e.register(
            slot(PROCESS_VTABLE, GET_PROCEDURE_INDEX_RUNNING_SLOT),
            |_, _| 0u32.into_ret(),
        );
        e.register(PACKAGE_GET_PROCEDURE_TYPE, |_, _| 0u32.into_ret());
        e.register(PACKAGE_LOCATION_0055B980, |_, _| 0x4100u32.into_ret());
        e.register(PACKAGE_GET_TARGET, |_, _| 0x4200u32.into_ret());
        e.register(PACKAGE_RADIUS_ACTOR_TO_LOCATION, |_, _| 50.0f64.into_ret());
        e.register(PACKAGE_RADIUS_ACTOR_TO_REF_TARGET, |_, _| {
            70.0f64.into_ret()
        });
        e.register(PACKAGE_TEST_00901C50, |_, _| true.into_ret());
        e.register(PACKAGE_GET_00676280, |_, _| 9u32.into_ret());
        for (procedure, expected) in [
            (0u32, 50.0f32),
            (2, 70.0),
            (6, 70.0),
            (47, 128.0),
            (9, 128.0),
            (0x33, 128.0),
        ] {
            e.mem.set_u32(list, procedure);
            assert_eq!(
                e.call(0x0090_f670, &args![this, 0x3000u32]).f32(),
                expected,
                "procedure {procedure}"
            );
        }
        e.call_log = Some(vec![]);
        e.mem.set_u32(list, 47);
        e.call(0x0090_f670, &args![this, 0x3000u32]);
        assert_eq!(calls(&e, PACKAGE_GET_00676280), vec![vec![0x4000, 0x3000]]);
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0u32.into_ret()
        });
        assert_eq!(e.call(0x0090_f670, &args![this, 0x3000u32]).f32(), 128.0);
    }

    #[test]
    fn update_followers_keeps_or_drops_followers() {
        for (dead, actor_type, follower_type, kept) in [
            (false, 2u32, 0u32, true),
            (false, 0, 1, true),
            (false, 0, 3, false),
            (true, 2, 1, false),
        ] {
            let mut e = Engine::new();
            e.map(0x011d_e000, 0x1000);
            let actor = object(&mut e, ACTOR_VTABLE, 0x20, &[ACTOR_SLOT_22C]);
            let follower = object(&mut e, 0x0073_0000, 0x20, &[SET_AVOID_WAIT_TIMER_SLOT]);
            e.register_double(slot(ACTOR_VTABLE, ACTOR_SLOT_22C), move |_, _| {
                dead.into_ret()
            });
            e.register(REFERENCE_TEST_00440DA0, |_, _| false.into_ret());
            e.register(REFERENCE_GET_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
            let followers = e.mem.alloc(0x10);
            let followers_node = e.mem.alloc(8);
            e.mem.set_u32(followers_node, follower);
            e.mem.set_u32(followers + FOLLOWER_LIST, followers_node);
            e.register_double(EXTRA_GET_FOLLOWERS, move |_, _| followers.into_ret());
            e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(PACKAGE_LIST_CONSTRUCT, |_, a| a[0].into_ret());
            e.register(LIST_HEAD_ITEM, |_, a| a[0].into_ret());
            e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
            e.register(OBJECT_LIST_ADD_00905820, |e, a| {
                let item = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], item);
                Ret::default()
            });
            e.register(LIST_ADD_ITEM, |e, a| {
                let item = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], item);
                Ret::default()
            });
            let actor_package = e.mem.alloc(4);
            e.mem.set_u32(actor_package, actor_type);
            let follower_package = e.mem.alloc(4);
            e.mem.set_u32(follower_package, follower_type);
            e.register_double(ACTOR_GET_PACKAGE_SET_AS_CURRENT, move |_, a| {
                if a[0] == actor {
                    actor_package.into_ret()
                } else {
                    follower_package.into_ret()
                }
            });
            e.register(EXTRA_GET_PACKAGE, |_, _| 0u32.into_ret());
            e.register(PACKAGE_IS_INTERRUPT_PACKAGE, |_, _| false.into_ret());
            e.register(PACKAGE_GET_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
            e.register(MOBILE_GET_PROCESS_TYPE, |_, _| 1u32.into_ret());
            e.register(slot(0x0073_0000, SET_AVOID_WAIT_TIMER_SLOT), nothing);
            e.register(REFERENCE_RUN_SCRIPT, nothing);
            e.register(EXTRA_REMOVE_FOLLOWER, nothing);
            e.register(LIST_CLEAR, nothing);
            e.register(LIST_DELETE, nothing);
            e.register(LIST_IS_EMPTY, |_, _| false.into_ret());
            e.call_log = Some(vec![]);
            e.call(0x0091_1b00, &args![0x1000u32, actor, 0.5f32]);
            if kept {
                assert_eq!(
                    calls(&e, slot(0x0073_0000, SET_AVOID_WAIT_TIMER_SLOT)),
                    vec![vec![follower, 0.5f32.to_bits()]]
                );
                assert_eq!(calls(&e, REFERENCE_RUN_SCRIPT), vec![vec![follower]]);
                assert!(calls(&e, EXTRA_REMOVE_FOLLOWER).is_empty());
            } else {
                assert_eq!(
                    calls(&e, EXTRA_REMOVE_FOLLOWER),
                    vec![vec![actor + 0x44, follower]]
                );
            }
            assert_eq!(calls(&e, LIST_DELETE).len(), 2);
        }
    }

    #[test]
    fn guard_travel_to_a_reference_or_a_point() {
        for (location_type, expect_reference) in [(2u32, true), (3, false)] {
            let mut e = Engine::new();
            e.map(0x0101_1000, 0x1000);
            e.set_global(GUARD_RADIUS_FACTOR, 0.5f64);
            e.map(0x0101_e000, 0x1000);
            e.set_global(GUARD_RADIUS_MINIMUM, 15.0f32);
            let this = Ptr::<LowProcess>::new(object(
                &mut e,
                PROCESS_VTABLE,
                0x200,
                &[
                    GET_PACKAGE_RUNNING_SLOT,
                    GET_TARGET_SLOT,
                    GET_RUN_ONCE_PACKAGE_SLOT,
                ],
            ));
            let actor = object(
                &mut e,
                ACTOR_VTABLE,
                0x20,
                &[
                    ACTOR_SLOT_29C,
                    ACTOR_SLOT_298,
                    ACTOR_SLOT_294,
                    ACTOR_GET_LOCATION_SLOT,
                ],
            );
            let point = e.mem.alloc(0xc);
            e.mem.set_f32(point, 4.0);
            e.register_double(slot(ACTOR_VTABLE, ACTOR_SLOT_29C), move |_, _| {
                point.into_ret()
            });
            e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_298), |_, _| {
                0x8800u32.into_ret()
            });
            e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_294), |_, _| {
                0x8900u32.into_ret()
            });
            e.register(slot(ACTOR_VTABLE, ACTOR_GET_LOCATION_SLOT), |_, a| {
                (a[0] + 0x10).into_ret()
            });
            e.register(slot(PROCESS_VTABLE, GET_PACKAGE_RUNNING_SLOT), |_, _| {
                0x4000u32.into_ret()
            });
            e.register(LIST_HEAD_ITEM, |_, a| a[0].into_ret());
            e.register(PACKAGE_LOCATION_0055B980, |_, _| 0x4100u32.into_ret());
            e.register(PACKAGE_GET_TARGET, |_, _| 0x4200u32.into_ret());
            e.register_double(PACKAGE_LOCATION_GET_TYPE, move |_, _| {
                location_type.into_ret()
            });
            e.register(PACKAGE_GET_00676280, |_, _| 100u32.into_ret());
            e.register(LARGER_OF_00404010, |_, a| {
                f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))).into_ret()
            });
            e.register(REFERENCE_GET_WORLD_SPACE, |_, _| 0x55u32.into_ret());
            e.register(ACTOR_GET_008D6F30, |_, _| 0x66u32.into_ret());
            for callee in [
                PATHING_REQUEST_CONSTRUCT,
                ACTOR_BUILD_REQUEST,
                PATHING_REQUEST_SET_008E7C50,
                ACTOR_SET_PATHFINDING_GOAL,
                SET_INSTANCE_DATA_006ECD40,
                PATHING_REQUEST_DESTRUCT,
            ] {
                e.register(callee, nothing);
            }
            e.call_log = Some(vec![]);
            e.call(0x0091_2db0, &args![this, actor, 0x9000u32]);
            let built = calls(&e, ACTOR_BUILD_REQUEST);
            assert_eq!(built.len(), 1);
            assert_eq!(built[0][0], actor);
            assert_eq!(built[0][5], 50.0f32.to_bits());
            if expect_reference {
                assert_eq!(&built[0][2..5], &[actor + 0x10, 0x66, 0x55]);
            } else {
                assert_eq!(&built[0][3..5], &[0x8800, 0x8900]);
                assert_ne!(built[0][2], point);
            }
            assert_eq!(calls(&e, SET_INSTANCE_DATA_006ECD40), vec![vec![0x9000, 1]]);
        }
    }

    /// The escort world: the actor `0x3000`-like object, its process `this`, the current
    /// package's slot `+0x13c` answering `done`.
    fn escort_world(done: bool, with_item: bool) -> (Engine, Ptr<LowProcess>, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(MINUS_ONE, -1.0f32);
        let this = Ptr::<LowProcess>::new(object(
            &mut e,
            PROCESS_VTABLE,
            0x200,
            &[
                GET_PACKAGE_RUNNING_SLOT,
                GET_CURRENT_PACKAGE_SLOT,
                SET_TARGET_FOR_PACKAGE_SLOT,
                ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT,
                CHECK_IF_HAS_OBJECT_SLOT,
            ],
        ));
        let actor = object(&mut e, ACTOR_VTABLE, 0x20, &[ACTOR_SLOT_3CC]);
        let package = object(&mut e, PACKAGE_VTABLE, 0x20, &[PACKAGE_SLOT_13C]);
        e.register(
            slot(PROCESS_VTABLE, GET_PACKAGE_RUNNING_SLOT),
            move |_, _| 0x4000u32.into_ret(),
        );
        e.register_double(
            slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT),
            move |_, _| package.into_ret(),
        );
        e.register_double(slot(PACKAGE_VTABLE, PACKAGE_SLOT_13C), move |_, _| {
            done.into_ret()
        });
        e.register(slot(PROCESS_VTABLE, SET_TARGET_FOR_PACKAGE_SLOT), nothing);
        e.register(
            slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT),
            nothing,
        );
        e.register(slot(PROCESS_VTABLE, CHECK_IF_HAS_OBJECT_SLOT), |_, _| {
            true.into_ret()
        });
        e.register(slot(ACTOR_VTABLE, ACTOR_SLOT_3CC), nothing);
        let item = if with_item { 0x7700u32 } else { 0 };
        e.register_double(ACTOR_ESCORT_ITEM_008A2D40, move |_, _| item.into_ret());
        e.register(PACKAGE_GET_TARGET, |_, _| 0u32.into_ret());
        e.register(ITEM_DELETE, nothing);
        // The time delta's callees.
        e.register(LOOKUP_FORM_BY_ID, |_, _| 0x9000u32.into_ret());
        e.register(CALENDAR_GET_HOUR, |_, _| 0.0f64.into_ret());
        e.register(PROCESS_GET_HOUR, |_, _| 0.0f64.into_ret());
        e.register(GLOBAL_VALUE, |_, _| 1.0f64.into_ret());
        e.call_log = Some(vec![]);
        (e, this, actor)
    }

    #[test]
    fn escort_without_item_or_target_moves_on() {
        let (mut e, this, actor) = escort_world(true, false);
        e.call(0x0090_9430, &args![this, actor]);
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, SET_TARGET_FOR_PACKAGE_SLOT)).len(),
            2
        );
        assert_eq!(
            calls(
                &e,
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)
            ),
            vec![vec![this.addr(), actor, 1]]
        );
        assert!(calls(&e, ITEM_DELETE).is_empty());
    }

    #[test]
    fn escort_done_gives_the_item() {
        let (mut e, this, actor) = escort_world(true, true);
        e.register(WORD_AT_00559450, |_, _| 0u32.into_ret());
        e.register(PACKAGE_LOCATION_0055B980, |_, _| 0x4100u32.into_ret());
        e.register(LOCATION_GET_REFERENCE, |_, _| 0u32.into_ret());
        e.register(PACKAGE_TARGET_GET_REFERENCE, |_, _| 0u32.into_ret());
        e.register(PACKAGE_GET_INITIAL_TARGET_COUNT, |_, _| 3u32.into_ret());
        e.register(WORD_08_0044DDC0, |_, _| 0x6600u32.into_ret());
        e.call(0x0090_9430, &args![this, actor]);
        assert_eq!(
            calls(&e, slot(ACTOR_VTABLE, ACTOR_SLOT_3CC)),
            vec![vec![actor, 0x6600, 0, 3, 0, 0]]
        );
        assert_eq!(
            calls(
                &e,
                slot(PROCESS_VTABLE, ADD_TO_PROCEDURE_INDEX_RUNNING_SLOT)
            ),
            vec![vec![this.addr(), actor, 1]]
        );
        assert_eq!(calls(&e, ITEM_DELETE), vec![vec![0x7700, 1]]);
    }

    #[test]
    fn escort_not_done_walks_to_the_location() {
        let (mut e, this, actor) = escort_world(false, true);
        e.register(PACKAGE_RADIUS_ACTOR_TO_REF_TARGET, |_, _| {
            100.0f64.into_ret()
        });
        e.register(FTOL, |_, a| (f64::take(a, &mut 0) as i32).into_ret());
        e.register(ACTOR_IS_PATHING_COMPLETE, |_, _| true.into_ret());
        e.register(PACKAGE_GET_LOCATION_WORLD, |_, _| 0x55u32.into_ret());
        e.register(PACKAGE_GET_LOCATION_CELL, |_, _| 0x66u32.into_ret());
        e.register(PACKAGE_GET_LOCATION_COORD, |_, a| a[1].into_ret());
        e.register(ACTOR_SET_PATHFINDING_GOAL_TO_LOCATION, nothing);
        e.call(0x0090_9430, &args![this, actor]);
        let goals = calls(&e, ACTOR_SET_PATHFINDING_GOAL_TO_LOCATION);
        assert_eq!(goals.len(), 1);
        assert_eq!(goals[0][0], actor);
        assert_eq!(&goals[0][2..], &[0x66, 0x55, (-1.0f32).to_bits(), 0]);
        assert_eq!(calls(&e, ITEM_DELETE), vec![vec![0x7700, 1]]);
    }

    /// The acquire callback world: the reference (base `0x5500` of type `form_type`, actor
    /// when `is_actor`), the actor whose process (state 1) has `eFormType` `wanted_type`.
    fn acquire_callback_world(
        form_type: u32,
        wanted_type: u32,
        is_actor: bool,
    ) -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        let reference = object(&mut e, REFERENCE_VTABLE, 0x20, &[REFERENCE_SLOT_100]);
        let actor = e.mem.alloc(0x20);
        let process = e.mem.alloc(0x100);
        e.mem.set_u32(process + FORM_TYPE_FOR_LOCATION, wanted_type);
        for callee in [
            REFERENCE_TEST_00440D80,
            REFERENCE_TEST_004077C0,
            REFERENCE_TEST_00440DA0,
        ] {
            e.register(callee, |_, _| false.into_ret());
        }
        e.register_double(slot(REFERENCE_VTABLE, REFERENCE_SLOT_100), move |_, _| {
            is_actor.into_ret()
        });
        e.register(REFERENCE_GET_BASE_FORM, |_, _| 0x5500u32.into_ret());
        e.register_double(FORM_GET_TYPE, move |_, _| form_type.into_ret());
        e.register_double(ACTOR_GET_PROCESS, move |_, _| process.into_ret());
        e.register(PROCESS_GET_0045CD60, |_, _| 1u32.into_ret());
        e.register(FORM_MATCHES_PACKAGE_OBJECT_TYPE, |_, a| {
            (a[0] == 0x5500).into_ret()
        });
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(REFERENCE_GET_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(EXTRA_GET_COUNT, |_, _| 0xffffu32.into_ret());
        e.register(OBJECT_LIST_ADD_00905820, nothing);
        e.register(EXTRA_GET_CONTAINER_CHANGES, |_, _| 0u32.into_ret());
        e.register(REFERENCE_ITEM_COUNT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        (e, reference, actor, process)
    }

    #[test]
    fn acquire_callback_adds_matching_references() {
        let (mut e, reference, actor, process) = acquire_callback_world(0x10, 7, false);
        assert!(!e.call(0x0090_dd80, &args![reference, actor]).bool());
        let added = calls(&e, OBJECT_LIST_ADD_00905820);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], process + TEMP_OBJECT_LIST);
        let entries: Vec<u32> = calls(&e, ALLOCATE).iter().map(|w| w[0]).collect();
        assert_eq!(entries, vec![0x1c]);
        // A book for which 0073b460 holds is not wanted.
        let (mut e, reference, actor, _) = acquire_callback_world(0x19, 7, false);
        e.register(BOOK_TEST_0073B460, |_, _| true.into_ret());
        e.call(0x0090_dd80, &args![reference, actor]);
        assert!(calls(&e, OBJECT_LIST_ADD_00905820).is_empty());
        // Own reference, no actor, a process in state 4.
        let (mut e, reference, _, _) = acquire_callback_world(0x10, 7, false);
        assert!(!e.call(0x0090_dd80, &args![reference, reference]).bool());
        assert!(e.call(0x0090_dd80, &args![reference, 0u32]).bool());
        let (mut e, reference, actor, _) = acquire_callback_world(0x10, 7, false);
        e.register(PROCESS_GET_0045CD60, |_, _| 4u32.into_ret());
        assert!(e.call(0x0090_dd80, &args![reference, actor]).bool());
    }

    #[test]
    fn acquire_callback_searches_containers() {
        // Form type 0x1b: the inventory items in the same space become entries of state 1.
        let (mut e, reference, actor, process) = acquire_callback_world(0x1b, 0, false);
        e.register(REFERENCE_ITEM_COUNT, |_, _| 1u32.into_ret());
        e.register(REFERENCE_ITEM_AT, |_, _| 0x7000u32.into_ret());
        e.register(ITEM_GET_WORN, |_, _| false.into_ret());
        e.register(WORD_08_0044DDC0, |_, _| 0x6600u32.into_ret());
        e.register(REFERENCE_GET_SPACE, |_, _| 0x99u32.into_ret());
        e.register(REFERENCE_HAS_CONTAINER, |_, _| 0u32.into_ret());
        e.register(WORD_AT_00559450, |_, _| 0u32.into_ret());
        e.register(ITEM_DELETE, nothing);
        e.call(0x0090_dd80, &args![reference, actor]);
        // The reference itself (neither set) and the item.
        assert_eq!(calls(&e, OBJECT_LIST_ADD_00905820).len(), 2);
        assert!(calls(&e, OBJECT_LIST_ADD_00905820)
            .iter()
            .all(|w| w[0] == process + TEMP_OBJECT_LIST));
        assert_eq!(calls(&e, ITEM_DELETE), vec![vec![0x7000, 1]]);
    }

    #[test]
    fn build_multi_target_list_from_a_container() {
        for (kind, expected) in [(0xcu32, 1usize), (0x13, 2), (0x14, 3), (0x16, 1), (0x15, 0)] {
            let mut e = Engine::new();
            let this = process(&mut e);
            e.register(REFERENCE_HAS_CONTAINER, |_, _| 1u32.into_ret());
            e.register(GET_INVENTORY_CHANGES, |_, _| 0x8000u32.into_ret());
            e.register(CHANGES_ITEM_COUNT, |_, _| 3u32.into_ret());
            e.register(CHANGES_ITEM_AT, |_, a| (0x7000 + a[1]).into_ret());
            e.register(WORD_08_0044DDC0, |_, a| (a[0] + 0x100).into_ret());
            // Item 0 is a weapon, 1 armor, 2 clothing.
            e.register(DYNAMIC_CAST, |_, a| {
                let is = match a[3] {
                    WEAPON_DESCRIPTOR => a[0] == 0x7100,
                    ARMOR_DESCRIPTOR => a[0] == 0x7101,
                    CLOTHING_DESCRIPTOR => a[0] == 0x7102,
                    _ => false,
                };
                (if is { a[0] } else { 0 }).into_ret()
            });
            e.register(WEAPON_TEST_006450C0, |_, _| true.into_ret());
            e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(LIST_NEXT_NODE, |_, _| 5u32.into_ret());
            e.register(LIST_ADD_ITEM, nothing);
            e.register(ITEM_DELETE, nothing);
            e.call_log = Some(vec![]);
            e.call(0x0091_0e20, &args![this, 0x3000u32, 0x4400u32, kind]);
            assert_eq!(calls(&e, LIST_ADD_ITEM).len(), expected, "kind {kind:#x}");
            if expected > 0 {
                assert_eq!(calls(&e, ITEM_DELETE).len(), 3);
                assert_eq!(calls(&e, LIST_ADD_ITEM)[0][0], this.addr() + OBJECT_LIST);
            }
        }
    }

    #[test]
    fn build_multi_target_list_around_the_actor() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(DATA_HANDLER, 0x6600u32);
        e.map(0x011c_d000, 0x1000);
        let this = Ptr::<LowProcess>::new(object(
            &mut e,
            PROCESS_VTABLE,
            0x200,
            &[
                GET_PACKAGE_RUNNING_SLOT,
                GET_CURRENT_PACKAGE_SLOT,
                EVALUATE_ORDER_ACQUIRE_LIST_SLOT,
            ],
        ));
        let base = this.addr();
        let actor = object(&mut e, ACTOR_VTABLE, 0x20, &[ACTOR_GET_LOCATION_SLOT]);
        let position = e.mem.alloc(0xc);
        e.register_double(slot(ACTOR_VTABLE, ACTOR_GET_LOCATION_SLOT), move |_, _| {
            position.into_ret()
        });
        e.register(ACTOR_GET_008D6F30, |_, _| 0x77u32.into_ret());
        e.register(slot(PROCESS_VTABLE, GET_PACKAGE_RUNNING_SLOT), |_, _| {
            0x4000u32.into_ret()
        });
        e.register(slot(PROCESS_VTABLE, GET_CURRENT_PACKAGE_SLOT), |_, _| {
            0x4000u32.into_ret()
        });
        e.register(PACKAGE_GET_SEARCH_LOCATION, |_, _| 0u32.into_ret());
        e.register(PACKAGE_GET_ACQUIRE_RADIUS, |e, a| {
            e.mem.set_f32(a[2], 750.0);
            true.into_ret()
        });
        e.register(ENUM_REFERENCES_CLOSE_TO_POINT, nothing);
        e.register(
            slot(PROCESS_VTABLE, EVALUATE_ORDER_ACQUIRE_LIST_SLOT),
            nothing,
        );
        e.mem.set_u32(base + OBJECT_TO_ACQUIRE, 0x11);
        e.call_log = Some(vec![]);
        e.call(0x0091_0e20, &args![this, actor, 0u32, 0x13u32]);
        let enumerated = calls(&e, ENUM_REFERENCES_CLOSE_TO_POINT);
        assert_eq!(enumerated.len(), 1);
        assert_eq!(enumerated[0][0], 0x6600);
        assert_eq!(enumerated[0][1], 0x77);
        assert_eq!(enumerated[0][3], 750.0f32.to_bits());
        assert_eq!(enumerated[0][4], position);
        assert_eq!(&enumerated[0][6..], &[COLLECT_TARGET_CALLBACK, actor]);
        assert_eq!(e.mem.u32(base + FORM_TYPE_FOR_LOCATION), 0);
        assert_eq!(e.mem.u32(base + OBJECT_TO_ACQUIRE), 0);
        assert_eq!(
            calls(&e, slot(PROCESS_VTABLE, EVALUATE_ORDER_ACQUIRE_LIST_SLOT)),
            vec![vec![base, actor]]
        );
    }
}
