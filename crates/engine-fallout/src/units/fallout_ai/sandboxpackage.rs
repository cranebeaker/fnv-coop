//! `fallout/ai/sandboxpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00558dd0` (unnamed in the engine map): the base body the wrapper forwards to;
/// `thiscall`, `this` plus one stack word (`RET 4`).
const ELEMENT_TARGET_00558DD0: u32 = 0x0055_8dd0;

// Translated from 009f6190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (sandbox package; callers `ForgetTargetAndPickProcedure`,
/// `009f4700`): forwards `this` and an index to `00558dd0` (`tesobjectcell.cpp` per the engine
/// map), which returns the word at `this+4` plus `index` shifted left by 4: the address of a
/// 16-byte element. The wrapper returns that address.
pub fn fn_009f6190(e: &mut Engine, this: Ptr, index: u32) -> Ptr {
    e.call(ELEMENT_TARGET_00558DD0, &args![this, index]).ptr()
}

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VSandBoxChoiceListItem@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_01093614: u32 = 0x0109_3614;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 009f61e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@VSandBoxChoiceListItem@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009f61e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_01093614);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// The vtable `009f61b0` stores (`BSSimpleArray<SandBoxChoiceListItem,1024>`).
const CHOICE_ARRAY_VTABLE: u32 = ARRAY_VTABLE_01093614;
/// Constructs `count` choice items at `buffer` (`009f6460`, `this`, the
/// buffer and the count).
const CHOICE_ARRAY_CONSTRUCT_ITEMS: u32 = 0x009f_6460;
/// Grows the array by one and answers the new item's index (`006f31f0`).
const CHOICE_ARRAY_GROW: u32 = 0x006f_31f0;
/// Destroys `count` items at an address (`0072ba80`, `this`, the address
/// and the count).
const CHOICE_ARRAY_DESTROY_ITEMS: u32 = 0x0072_ba80;
/// Moves `count` items (`006c8bd0`, `this`, the destination, the source and
/// the count).
const CHOICE_ARRAY_MOVE_ITEMS: u32 = 0x006c_8bd0;
/// cdecl helper of two words (`0042f5a0`; the count of items to move).
const MOVE_COUNT_0042F5A0: u32 = 0x0042_f5a0;
/// `BSSimpleArray` (Xbox PDB): `pBuffer` +4, `uiSize` +8, `uiAllocSize` +0xC.
const ARRAY_BUFFER: u32 = 0x04;
const ARRAY_SIZE: u32 = 0x08;
const ARRAY_ALLOC_SIZE: u32 = 0x0c;
/// The size of a `SandBoxChoiceListItem` (Xbox PDB, 0x10 bytes).
const CHOICE_SIZE: u32 = 0x10;
/// `TESObjectREFR::SetTargeted` (`00564db0`, the reference and a flag).
const SET_TARGETED: u32 = 0x0056_4db0;
/// `TESObjectREFR::SetMarkerReserved` (`00568150`, the reference, the
/// marker and a flag).
const SET_MARKER_RESERVED: u32 = 0x0056_8150;
/// `TESObjectREFR::GetMarkerUsed` (`00567f80`, the reference, the marker and
/// a flag).
const GET_MARKER_USED: u32 = 0x0056_7f80;
/// Writes bytes into the save buffer (`00865e50`, the buffer, the address,
/// the size and a flag word).
const SAVE_DATA: u32 = 0x0086_5e50;
/// `BGSSaveGameBuffer::SaveFormID_ov2` (`00865df0`, the buffer, the form and
/// a flag word).
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// The game's `operator delete` (`00401030`, cdecl, the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `ActorPackageData`'s destructor body (`0067d970`).
const ACTOR_PACKAGE_DATA_DESTRUCT: u32 = 0x0067_d970;
/// The `SandBoxActorPackageData` vtable (its RTTI locator names
/// `.?AVSandBoxActorPackageData@@`).
const SANDBOX_ACTOR_PACKAGE_DATA_VTABLE: u32 = 0x0109_35f8;
/// The `TESDataHandler` singleton pointer (`011c3f2c`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// A `TESDataHandler` test (`004226e0`) that keeps the marker reserved.
const DATA_HANDLER_TEST_004226E0: u32 = 0x0042_26e0;
/// The word at +8 of an object (`0044ddc0`): an array's size, the data's
/// `eProcedure`, ...
const WORD_AT_8: u32 = 0x0044_ddc0;
/// The procedure's package procedure type, by `SANDBOX_PROCEDURE` (six
/// words in `.rdata`).
const PROCEDURE_TYPES: u32 = 0x0109_35dc;
/// `SandBoxActorPackageData::ProceduresForActivities` (Xbox PDB static
/// array, by `SANDBOX_ACTIVITY`).
const PROCEDURES_FOR_ACTIVITIES: u32 = 0x011a_5f3c;
/// `PackageLocation::GetLocType` (`00678ca0`: the location's first byte,
/// sign-extended).
const PACKAGE_LOCATION_GET_LOC_TYPE: u32 = 0x0067_8ca0;
/// `PackageLocation::GetLocReference` (`0067f390`).
const PACKAGE_LOCATION_GET_LOC_REFERENCE: u32 = 0x0067_f390;
/// Reads the word at +0x68 (`008d8520`; the engine map's name
/// `MiddleHighProcess::GetSavedAcquireObject` belongs to folded code).
const OBJECT_AT_0X68: u32 = 0x008d_8520;
/// `TESObjectREFR` getter (`00569b80`).
const REFERENCE_VALUE_00569B80: u32 = 0x0056_9b80;
/// A reference's extra data list (`005d43c0`: its address +0x44).
const EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// The extra data list's entry of type 0x51 (`0041e410`; its word at +0xC,
/// or 0).
const EXTRA_DATA_0X51: u32 = 0x0041_e410;
/// The extra data list's entry of type 0x6F (`0041fe90`; its word at +0xC,
/// or 0).
const EXTRA_DATA_0X6F: u32 = 0x0041_fe90;
/// The float at +0 of an object (`006a7f50`, in `ST0`).
const FLOAT_AT_0: u32 = 0x006a_7f50;
/// The object whose word at +0x14 `00825c00` answers (`011f6394`); the
/// sandbox compares it with times in milliseconds.
const TIMER: u32 = 0x011f_6394;
const TIMER_NOW: u32 = 0x0082_5c00;
/// `1000.0` (`double`).
const THOUSAND: u32 = 0x0101_7b70;
/// `24.0` (`double`).
const HOURS_PER_DAY: u32 = 0x0103_56d8;
/// `60.0` (`double`).
const SIXTY: u32 = 0x0101_2638;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// The `Calendar` singleton (`011de7b8`); `009611e0` answers its word at
/// +0x18 (`iMidnightsPassed`, Xbox PDB).
const CALENDAR: u32 = 0x011d_e7b8;
const CALENDAR_DAYS: u32 = 0x0096_11e0;
/// `Calendar::GetHour` (`00867da0`, a `float` in `ST0`).
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// `_ftol2` (`00ec62c0`): the `ST0` value as a leading `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `abs` (`00ec7d40`, cdecl).
const ABS: u32 = 0x00ec_7d40;
/// A choice item's target (`00559450`, the word at +0).
const CHOICE_TARGET: u32 = 0x0055_9450;
/// `Actor::GetAnimAction` (`008a7570`).
const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
/// `Animation::SpecialIdleDonePlaying` (`004985f0`).
const SPECIAL_IDLE_DONE_PLAYING: u32 = 0x0049_85f0;
/// An `Animation` getter (`00498cf0`).
const ANIMATION_VALUE_00498CF0: u32 = 0x0049_8cf0;
/// `LowProcess::GetGenericLocation` (`008041a0`, by the engine map).
const VALUE_008041A0: u32 = 0x0080_41a0;
/// `MobileObject::GetCurrentPackage` (`009344a0`).
const MOBILE_OBJECT_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;

/// `SandBoxActorPackageData` (Xbox PDB field names, PC offsets as the code
/// uses them).
const STATE: u32 = 0x04;
const PROCEDURE: u32 = 0x08;
const PROCEDURE_DURATION: u32 = 0x0c;
const LOCATION: u32 = 0x10;
const LAST_TIME_CHANGED: u32 = 0x14;
const TOTAL_TIME: u32 = 0x18;
const NEXT_TIME_TO_CHANGE_LOCATIONS: u32 = 0x1c;
const POTENTIAL_TARGETS: u32 = 0x20;
const NEXT_TIME_TO_SCAN_WORLD: u32 = 0x30;
const SHOULD_PICK_NEW_PROCEDURE: u32 = 0x34;
const FOOD: u32 = 0x35;
const CHAIR: u32 = 0x36;
const BED: u32 = 0x37;
const CURRENT_TARGET_OBJECT: u32 = 0x3c;
const CURRENT_TARGET_MARKER: u32 = 0x40;
const PREV_TARGET_OBJECT: u32 = 0x44;
const ACTIVITY_START_DAY: u32 = 0x4c;
const ACTIVITY_START_HOUR: u32 = 0x50;
const ACTIVITY_DURATION: u32 = 0x54;
const ACTIVITY: u32 = 0x58;

fn now(e: &mut Engine) -> u32 {
    e.call(TIMER_NOW, &args![TIMER]).u32()
}

// Translated from 009f61b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<SandBoxChoiceListItem,1024>` constructor: stores the
/// vtable and initializes it empty (`009f65f0(this, 0, 0)`). Returns `this`.
pub fn fn_009f61b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), CHOICE_ARRAY_VTABLE);
    fn_009f65f0(e, this, 0, 0);
    this
}

// Translated from 009f65f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SandBoxChoiceListItem,1024>::Initialize` (by its use):
/// clears the buffer, size and allocated size; the allocation is at least
/// `size`; with a non-zero allocation the buffer comes from the array's
/// virtual +4 and the allocated size is set; with a non-zero size that
/// many items are constructed (`009f6460`) and the size set.
pub fn fn_009f65f0(e: &mut Engine, this: Ptr, alloc_size: u32, size: u32) {
    let base = this.addr();
    e.mem.set_u32(base + ARRAY_BUFFER, 0);
    e.mem.set_u32(base + ARRAY_SIZE, 0);
    e.mem.set_u32(base + ARRAY_ALLOC_SIZE, 0);
    let mut alloc_size = alloc_size;
    if size > alloc_size {
        alloc_size = size;
    }
    if alloc_size > 0 {
        let buffer = e.vcall(base, 4, &args![alloc_size]).u32();
        e.mem.set_u32(base + ARRAY_BUFFER, buffer);
        e.mem.set_u32(base + ARRAY_ALLOC_SIZE, alloc_size);
    }
    if size > 0 {
        let buffer = e.mem.u32(base + ARRAY_BUFFER);
        e.call(CHOICE_ARRAY_CONSTRUCT_ITEMS, &args![this, buffer, size]);
        e.mem.set_u32(base + ARRAY_SIZE, size);
    }
}

// Translated from 009f6340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SandBoxChoiceListItem,1024>::Add` (by its use): grows the
/// array by one (`006f31f0`), constructs the new item (`009f6460`), copies
/// the 16 bytes of `item` into it and returns its index.
pub fn fn_009f6340(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let base = this.addr();
    let index = e.call(CHOICE_ARRAY_GROW, &args![this]).u32();
    let slot = (index << 4).wrapping_add(e.mem.u32(base + ARRAY_BUFFER));
    e.call(CHOICE_ARRAY_CONSTRUCT_ITEMS, &args![this, slot, 1u32]);
    let slot = e.mem.u32(base + ARRAY_BUFFER).wrapping_add(index << 4);
    for word in 0..4 {
        let value = e.mem.u32(item.addr() + 4 * word);
        e.mem.set_u32(slot + 4 * word, value);
    }
    index
}

// Translated from 009f63a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SandBoxChoiceListItem,1024>::RemoveAt` (by its use):
/// destroys `count` items from `start` (`0072ba80`), moves the
/// `0042f5a0(size - start - count, count)` items from the end into the gap
/// (`006c8bd0`) and lowers the size by `count`.
pub fn fn_009f63a0(e: &mut Engine, this: Ptr, start: u32, count: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + ARRAY_SIZE);
    let moved = e
        .call(
            MOVE_COUNT_0042F5A0,
            &args![size.wrapping_sub(start).wrapping_sub(count), count],
        )
        .u32();
    let gap = (start << 4).wrapping_add(e.mem.u32(base + ARRAY_BUFFER));
    e.call(CHOICE_ARRAY_DESTROY_ITEMS, &args![this, gap, count]);
    let size = e.mem.u32(base + ARRAY_SIZE);
    let source = e
        .mem
        .u32(base + ARRAY_BUFFER)
        .wrapping_add(size << 4)
        .wrapping_sub(moved << 4);
    let gap = (start << 4).wrapping_add(e.mem.u32(base + ARRAY_BUFFER));
    e.call(CHOICE_ARRAY_MOVE_ITEMS, &args![this, gap, source, moved]);
    let size = e.mem.u32(base + ARRAY_SIZE);
    e.mem.set_u32(base + ARRAY_SIZE, size.wrapping_sub(count));
}

/// Whether the bits `mask` of the 16-bit word at +0x24 are all clear.
fn word_at_24_clear(e: &mut Engine, this: Ptr, mask: u16) -> bool {
    e.mem.u16(this.addr() + 0x24) & mask == 0
}

// Translated from 009f45e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the 16-bit word at +0x24 is clear (on a package, the
/// sandbox's food test).
pub fn fn_009f45e0(e: &mut Engine, this: Ptr) -> bool {
    word_at_24_clear(e, this, 0x1)
}

// Translated from 009f4600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 of the 16-bit word at +0x24 is clear (on a package, the
/// sandbox's bed test).
pub fn fn_009f4600(e: &mut Engine, this: Ptr) -> bool {
    word_at_24_clear(e, this, 0x2)
}

// Translated from 009f4620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 4 of the 16-bit word at +0x24 is clear (on a package, the
/// sandbox's chair test).
pub fn fn_009f4620(e: &mut Engine, this: Ptr) -> bool {
    word_at_24_clear(e, this, 0x10)
}

// Translated from 009f54b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 3 of the 16-bit word at +0x24 is clear.
pub fn fn_009f54b0(e: &mut Engine, this: Ptr) -> bool {
    word_at_24_clear(e, this, 0x8)
}

// Translated from 009f5690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 5 of the 16-bit word at +0x24 is clear.
pub fn fn_009f5690(e: &mut Engine, this: Ptr) -> bool {
    word_at_24_clear(e, this, 0x20)
}

// Translated from 009f4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The package procedure type of the data's `eProcedure` (+8, read through
/// `0044ddc0`): -1 for -1, else the entry of the six-word table at
/// `010935dc`.
pub fn fn_009f4640(e: &mut Engine, this: Ptr) -> u32 {
    let procedure = e.call(WORD_AT_8, &args![this]).u32();
    if procedure == 0xffff_ffff {
        return 0xffff_ffff;
    }
    let procedure = e.call(WORD_AT_8, &args![this]).u32();
    e.mem
        .u32(PROCEDURE_TYPES.wrapping_add(procedure.wrapping_mul(4)))
}

// Translated from 009f3800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SandBoxChoiceListItem` constructor (Xbox PDB fields `pTarget`,
/// `eProcedure`, `iBaseScore`, `iFullScore`): stores the three arguments,
/// clears the full score and marks a target as targeted
/// (`TESObjectREFR::SetTargeted(target, 1)`). Returns `this`.
pub fn fn_009f3800(e: &mut Engine, this: Ptr, target: Ptr, procedure: u32, base_score: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, target.addr());
    e.mem.set_u32(base + 4, procedure);
    e.mem.set_u32(base + 8, base_score);
    e.mem.set_u32(base + 0xc, 0);
    if e.mem.u32(base) != 0 {
        let target = e.mem.u32(base);
        e.call(SET_TARGETED, &args![target, 1u32]);
    }
    this
}

// Translated from 009f3850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxChoiceListItem`'s save: the words at +4, +8 and +0xC, then the
/// target's form id.
pub fn fn_009f3850(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    for offset in [4u32, 8, 0xc] {
        e.call(SAVE_DATA, &args![buffer, base + offset, 4u32, 0u32]);
    }
    let target = e.mem.u32(base);
    e.call(SAVE_FORM_ID, &args![buffer, target, 0u32]);
}

// Translated from 009f4060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lets go of the current target marker: unless the data handler exists and
/// its test `004226e0` passes, a current target object gets its marker
/// unreserved (`SetMarkerReserved(object, marker, 0)`); then the object is
/// cleared and the marker becomes 0x7F.
pub fn fn_009f4060(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let handler = e.global::<u32>(DATA_HANDLER);
    let keep = handler != 0 && e.call(DATA_HANDLER_TEST_004226E0, &args![handler]).bool();
    if !keep && e.mem.u32(base + CURRENT_TARGET_OBJECT) != 0 {
        let marker = u32::from(e.mem.u8(base + CURRENT_TARGET_MARKER));
        let object = e.mem.u32(base + CURRENT_TARGET_OBJECT);
        e.call(SET_MARKER_RESERVED, &args![object, marker, 0u32]);
    }
    e.mem.set_u32(base + CURRENT_TARGET_OBJECT, 0);
    e.mem.set_u8(base + CURRENT_TARGET_MARKER, 0x7f);
}

// Translated from 009f3a70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f3a70 - C++ exception unwinding (FS:[0] frame) not translated
/// The `SandBoxActorPackageData` destructor body: the class's vtable, the
/// target marker let go (`009f4060`), the `PotentialTargets` array released
/// (`008454f0(array, 1)`) and destroyed (`009f61e0`), then the
/// `ActorPackageData` part (`0067d970`).
pub fn fn_009f3a70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SANDBOX_ACTOR_PACKAGE_DATA_VTABLE);
    fn_009f4060(e, this);
    e.call(
        ARRAY_RELEASE_008454F0,
        &args![base + POTENTIAL_TARGETS, 1u32],
    );
    fn_009f61e0(e, Ptr::new(base + POTENTIAL_TARGETS));
    e.call(ACTOR_PACKAGE_DATA_DESTRUCT, &args![this]);
}

// Translated from 009f3a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::_scalar_deleting_destructor_` (engine map
/// name): the destructor body, then the object freed when bit 0 of `flags`
/// is set. Returns `this`.
pub fn sand_box_actor_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_009f3a70(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 009f3e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the activity's need is met, by `eActivity` (+0x58): none (-1)
/// false, sleep (0) `bBed`, breakfast, lunch and dinner (1 to 3) `bFood`,
/// any other value true (the compiler's `switch` on `activity + 1`).
pub fn fn_009f3e20(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    match e.mem.u32(base + ACTIVITY).wrapping_add(1) {
        0 => false,
        1 => e.mem.u8(base + BED) != 0,
        2..=4 => e.mem.u8(base + FOOD) != 0,
        _ => true,
    }
}

// Translated from 009f3af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the sandbox's location (`pLocation`, +0x10) from the package
/// location: by its type (the compiler's `switch`, unsigned) 0 the
/// location's reference, 4 and 5 the actor's +0x68 object's virtual
/// `+0x514` (left as it was without that object), 6 `00569b80` of the
/// actor, 1 to 3 and any other type none. With a location whose extra data
/// 0x51 is set, the next change time is cleared and, with extra data 0x6F,
/// set to the timer plus its float times 1000 (truncated, `FISTP qword`,
/// low word).
pub fn fn_009f3af0(e: &mut Engine, this: Ptr, actor: Ptr, location: Ptr) {
    let base = this.addr();
    let kind = e
        .call(PACKAGE_LOCATION_GET_LOC_TYPE, &args![location])
        .u32();
    match kind {
        0 => {
            let reference = e
                .call(PACKAGE_LOCATION_GET_LOC_REFERENCE, &args![location])
                .u32();
            e.mem.set_u32(base + LOCATION, reference);
        }
        4 | 5 => {
            let object = e.call(OBJECT_AT_0X68, &args![actor]).u32();
            if object != 0 {
                let reference = e.vcall(object, 0x514, &args![]).u32();
                e.mem.set_u32(base + LOCATION, reference);
            }
        }
        6 => {
            let reference = e.call(REFERENCE_VALUE_00569B80, &args![actor]).u32();
            e.mem.set_u32(base + LOCATION, reference);
        }
        _ => e.mem.set_u32(base + LOCATION, 0),
    }
    let reference = e.mem.u32(base + LOCATION);
    if reference == 0 {
        return;
    }
    let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
    if e.call(EXTRA_DATA_0X51, &args![list]).u32() == 0 {
        return;
    }
    e.mem.set_u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS, 0);
    let reference = e.mem.u32(base + LOCATION);
    let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
    let extra = e.call(EXTRA_DATA_0X6F, &args![list]).u32();
    if extra == 0 {
        return;
    }
    let start = now(e);
    let seconds = e.call(FLOAT_AT_0, &args![extra]).f64();
    let thousand: f64 = e.global(THOUSAND);
    let milliseconds = x87_fistp64_chop(X87Ext::load64(x87_mul(seconds, thousand))) as u32;
    e.mem.set_u32(
        base + NEXT_TIME_TO_CHANGE_LOCATIONS,
        start.wrapping_add(milliseconds),
    );
}

// Translated from 009f3c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::MoveToNextLocation` (Xbox PDB): with a
/// location and a next change time (not -1) that the timer has passed,
/// the location's extra data 0x51 (when set and not the location itself)
/// becomes the location, the change time -1, and the answer true; false
/// otherwise.
pub fn sand_box_actor_package_data_move_to_next_location(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    if e.mem.u32(base + LOCATION) == 0 {
        return false;
    }
    if e.mem.u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS) == 0xffff_ffff {
        return false;
    }
    let time = now(e);
    if time <= e.mem.u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS) {
        return false;
    }
    let reference = e.mem.u32(base + LOCATION);
    let list = e.call(EXTRA_DATA_LIST, &args![reference]).u32();
    let next = e.call(EXTRA_DATA_0X51, &args![list]).u32();
    if next == 0 || next == e.mem.u32(base + LOCATION) {
        return false;
    }
    e.mem.set_u32(base + LOCATION, next);
    e.mem
        .set_u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS, 0xffff_ffff);
    true
}

/// The activity's end: start hour plus duration, carried past 24 into
/// following days. Answers the end day and the end hour's float bits.
fn activity_end(e: &mut Engine, base: u32) -> (u32, u32) {
    let mut day = e.mem.u32(base + ACTIVITY_START_DAY);
    let start = x87_load32(e.mem.u32(base + ACTIVITY_START_HOUR));
    let mut end = x87_store32(x87_add(start, x87_m32(e.mem.u32(base + ACTIVITY_DURATION))));
    let hours_per_day: f64 = e.global(HOURS_PER_DAY);
    // FCOMP with 24.0, C0 tested: on while not below and not unordered.
    while x87_load32(end) >= hours_per_day {
        end = x87_store32(x87_sub(x87_load32(end), hours_per_day));
        day = day.wrapping_add(1);
    }
    (day, end)
}

// Translated from 009f3c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the activity is over (unnamed in the engine map). True without
/// an activity (-1). Otherwise its end day and hour are computed: on the
/// end day the answer is true once the hour has reached the end (not
/// before it), with `*ended` set to 1; after the end day true with
/// `*ended` 0; before it false. `ended` may be null; it is written only
/// when the answer is true and there is an activity.
pub fn fn_009f3c90(e: &mut Engine, this: Ptr, ended: Ptr) -> bool {
    let base = this.addr();
    let day = e.call(CALENDAR_DAYS, &args![CALENDAR]).u32();
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    let hour = x87_store32(hour);
    let mut flag = 0u8;
    if e.mem.u32(base + ACTIVITY) == 0xffff_ffff {
        return true;
    }
    let (end_day, end) = activity_end(e, base);
    if day == end_day {
        // FCOMPP of the end with the hour: on when the end is below, equal
        // or unordered.
        let end = x87_load32(end);
        let hour = x87_load32(hour);
        if end > hour {
            return false;
        }
        flag = 1;
    } else if day < end_day {
        return false;
    }
    if !ended.is_null() {
        e.mem.set_u8(ended.addr(), flag);
    }
    true
}

// Translated from 009f3d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the calendar is inside the activity (unnamed in the engine map):
/// true without an activity (-1); false before the start day or after the
/// end day; on the start day false while the start hour is after the hour;
/// on the end day false once the end hour is at or before the hour (not
/// unordered); true otherwise.
pub fn fn_009f3d50(e: &mut Engine, this: Ptr) -> bool {
    let base = this.addr();
    let day = e.call(CALENDAR_DAYS, &args![CALENDAR]).u32();
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    let hour = x87_load32(x87_store32(hour));
    if e.mem.u32(base + ACTIVITY) == 0xffff_ffff {
        return true;
    }
    let (end_day, end) = activity_end(e, base);
    if day < e.mem.u32(base + ACTIVITY_START_DAY) || day > end_day {
        return false;
    }
    if day == e.mem.u32(base + ACTIVITY_START_DAY) {
        let start = x87_load32(e.mem.u32(base + ACTIVITY_START_HOUR));
        // FCOMPP of the start hour with the hour: false only when greater.
        if start > hour {
            return false;
        }
    }
    if day == end_day {
        let end = x87_load32(end);
        // FCOMPP of the end with the hour: on only when greater or unordered.
        if end <= hour {
            return false;
        }
    }
    true
}

// Translated from 009f40c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::HandleDeletedReference` (Xbox PDB): forgets a
/// deleted reference: the location and the previous target are cleared
/// when they are it, and every potential target choosing it is removed.
/// When it was the current target or a choice, the procedure is reset
/// (none, duration 0.0, no current or previous target, marker 0x7F, a new
/// procedure to pick, scan time 0).
pub fn sand_box_actor_package_data_handle_deleted_reference(
    e: &mut Engine,
    this: Ptr,
    reference: Ptr,
) {
    let base = this.addr();
    let reference = reference.addr();
    if reference == 0 {
        return;
    }
    let mut found = false;
    if e.mem.u32(base + LOCATION) == reference {
        e.mem.set_u32(base + LOCATION, 0);
    }
    if e.mem.u32(base + PREV_TARGET_OBJECT) == reference {
        e.mem.set_u32(base + PREV_TARGET_OBJECT, 0);
    }
    if e.mem.u32(base + CURRENT_TARGET_OBJECT) == reference {
        found = true;
    }
    let targets = Ptr::new(base + POTENTIAL_TARGETS);
    let mut count = e.call(WORD_AT_8, &args![targets]).i32();
    let mut index = 0i32;
    while index < count {
        let choice = fn_009f6190(e, targets, index as u32);
        if e.call(CHOICE_TARGET, &args![choice]).u32() == reference {
            found = true;
            fn_009f63a0(e, targets, index as u32, 1);
            count = count.wrapping_sub(1);
        } else {
            index = index.wrapping_add(1);
        }
    }
    if found {
        e.mem.set_u32(base + PROCEDURE, 0xffff_ffff);
        e.mem.set_f32(base + PROCEDURE_DURATION, 0.0);
        e.mem.set_u32(base + CURRENT_TARGET_OBJECT, 0);
        e.mem.set_u32(base + PREV_TARGET_OBJECT, 0);
        e.mem.set_u8(base + CURRENT_TARGET_MARKER, 0x7f);
        e.mem.set_u8(base + SHOULD_PICK_NEW_PROCEDURE, 1);
        e.mem.set_u32(base + NEXT_TIME_TO_SCAN_WORLD, 0);
    }
}

/// `_ftol2` of an x87 value, as the byte the callers keep.
fn ftol_byte(e: &mut Engine, value: f64) -> i8 {
    e.call(FTOL, &args![value]).u8() as i8
}

// Translated from 009f41f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::GetTimeDifference` (Xbox PDB): the hours from
/// `from` to `to`, both hours of the day: `|whole(to) - whole(from)|` plus
/// the difference of their minutes (each `(hour - whole(from)) * 60`
/// truncated to a byte; the exe takes `to`'s minutes against `from`'s
/// whole hour too), added when `to`'s minutes are greater, else taken off.
/// The seconds are computed (`_ftol2`) but not used.
pub fn sand_box_actor_package_data_get_time_difference(
    e: &mut Engine,
    _this: Ptr,
    from: f32,
    to: f32,
) -> f32 {
    let sixty: f64 = e.global(SIXTY);
    let from_value = x87_load32(from.to_bits());
    let from_hour = ftol_byte(e, from_value);
    let from_fraction = x87_sub(x87_m32(from.to_bits()), f64::from(from_hour));
    let from_minute = ftol_byte(e, x87_mul(from_fraction, sixty));
    let from_fraction = x87_sub(x87_m32(from.to_bits()), f64::from(from_hour));
    let seconds = x87_mul(
        x87_sub(x87_mul(from_fraction, sixty), f64::from(from_minute)),
        sixty,
    );
    ftol_byte(e, seconds);
    let to_value = x87_load32(to.to_bits());
    let to_hour = ftol_byte(e, to_value);
    let to_fraction = x87_sub(x87_m32(to.to_bits()), f64::from(from_hour));
    let to_minute = ftol_byte(e, x87_mul(to_fraction, sixty));
    let to_fraction = x87_sub(x87_m32(to.to_bits()), f64::from(from_hour));
    let seconds = x87_mul(
        x87_sub(x87_mul(to_fraction, sixty), f64::from(from_minute)),
        sixty,
    );
    ftol_byte(e, seconds);
    let hours = e
        .call(ABS, &args![i32::from(to_hour) - i32::from(from_hour)])
        .i32();
    let mut result = x87_store32(f64::from(hours));
    if to_minute > from_minute {
        let minutes = i32::from(to_minute) - i32::from(from_minute);
        result = x87_store32(x87_add(f64::from(minutes), x87_m32(result)));
    } else {
        let minutes = i32::from(from_minute) - i32::from(to_minute);
        result = x87_store32(x87_sub(x87_m32(result), f64::from(minutes)));
    }
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 009f4300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor is free to leave its procedure (unnamed in the engine
/// map). Only for the sit (0) and idle-marker (4) procedures: false while
/// the actor's animation action is 12 or 13; with an animation (virtual
/// `+0x1E4`) false while its special idle is not done
/// (`Animation::SpecialIdleDonePlaying`); with `stop` set and the
/// animation's `00498cf0` object whose `008041a0` is not 3, the actor's
/// +0x68 object's virtual `+0x394` is called with the actor. True
/// otherwise.
pub fn fn_009f4300(e: &mut Engine, this: Ptr, actor: Ptr, stop: bool) -> bool {
    let procedure = e.mem.u32(this.addr() + PROCEDURE);
    if procedure != 0 && procedure != 4 {
        return true;
    }
    let action = e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32();
    if (12..=13).contains(&action) {
        return false;
    }
    let animation = e.vcall(actor.addr(), 0x1e4, &args![]).u32();
    if animation == 0 {
        return true;
    }
    if !e.call(SPECIAL_IDLE_DONE_PLAYING, &args![animation]).bool() {
        return false;
    }
    if !stop {
        return true;
    }
    let idle = e.call(ANIMATION_VALUE_00498CF0, &args![animation]).u32();
    if idle == 0 {
        return true;
    }
    if e.call(VALUE_008041A0, &args![idle]).u32() == 3 {
        return true;
    }
    let object = e.call(OBJECT_AT_0X68, &args![actor]).u32();
    e.vcall(object, 0x394, &args![actor]);
    true
}

// Translated from 009f43c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::ShouldPickNewProcedure` (Xbox PDB). True, with
/// the scan time set to now, when the current package no longer allows a
/// bed, food or chair the data still has (each flag cleared). Then, in
/// order: true when the location's change time (not for the idle-marker
/// procedure, not -1) has passed; false when the actor is not free to
/// leave (`009f4300` with `stop` set); true when a new procedure is wanted,
/// there is no procedure, the duration is not above 0.0 (ordered), the
/// total time is 0.0, or the current marker (not in state 3) is used
/// (`TESObjectREFR::GetMarkerUsed(object, marker, 1)`), or the activity is
/// over (`009f3c90`); inside the activity with its need met, whether the
/// procedure differs from the activity's; else whether the duration is
/// below the time since the last change (`GetTimeDifference`).
pub fn sand_box_actor_package_data_should_pick_new_procedure(
    e: &mut Engine,
    this: Ptr,
    actor: Ptr,
) -> bool {
    let base = this.addr();
    let package = e
        .call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![actor])
        .u32();
    if package != 0 {
        let package = Ptr::new(package);
        let mut changed = false;
        if e.mem.u8(base + BED) != 0 && !fn_009f4600(e, package) {
            e.mem.set_u8(base + BED, 0);
            changed = true;
        }
        if e.mem.u8(base + FOOD) != 0 && !fn_009f45e0(e, package) {
            e.mem.set_u8(base + FOOD, 0);
            changed = true;
        }
        if e.mem.u8(base + CHAIR) != 0 && !fn_009f4620(e, package) {
            e.mem.set_u8(base + CHAIR, 0);
            changed = true;
        }
        if changed {
            let time = now(e);
            e.mem.set_u32(base + NEXT_TIME_TO_SCAN_WORLD, time);
            return true;
        }
    }
    if e.mem.u32(base + LOCATION) != 0
        && e.mem.u32(base + PROCEDURE) != 4
        && e.mem.u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS) != 0xffff_ffff
    {
        let time = now(e);
        if time > e.mem.u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS) {
            return true;
        }
    }
    if !fn_009f4300(e, this, actor, true) {
        return false;
    }
    if e.mem.u8(base + SHOULD_PICK_NEW_PROCEDURE) != 0 {
        return true;
    }
    if e.mem.u32(base + PROCEDURE) == 0xffff_ffff {
        return true;
    }
    let zero: f64 = e.global(ZERO);
    let duration = x87_load32(e.mem.u32(base + PROCEDURE_DURATION));
    // FCOMP with 0.0, C0 and C3 tested: on when greater or unordered.
    if duration <= zero {
        return true;
    }
    let total = x87_load32(e.mem.u32(base + TOTAL_TIME));
    // FCOMP with 0.0, C3 and C2 tested: equal (ordered) answers true.
    if total == zero {
        return true;
    }
    if e.mem.u32(base + STATE) != 3 && e.mem.u32(base + CURRENT_TARGET_OBJECT) != 0 {
        let marker = u32::from(e.mem.u8(base + CURRENT_TARGET_MARKER));
        let object = e.mem.u32(base + CURRENT_TARGET_OBJECT);
        if e.call(GET_MARKER_USED, &args![object, marker, 1u32]).bool() {
            return true;
        }
    }
    if fn_009f3c90(e, this, Ptr::NULL) {
        return true;
    }
    if fn_009f3d50(e, this) && fn_009f3e20(e, this) {
        let activity = e.mem.u32(base + ACTIVITY);
        let wanted = e
            .mem
            .u32(PROCEDURES_FOR_ACTIVITIES.wrapping_add(activity.wrapping_mul(4)));
        return e.mem.u32(base + PROCEDURE) != wanted;
    }
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    let hour = f32::from_bits(x87_store32(hour));
    let last = f32::from_bits(x87_quiet_bits(e.mem.u32(base + LAST_TIME_CHANGED)));
    let passed = sand_box_actor_package_data_get_time_difference(e, this, last, hour);
    let passed = x87_load32(x87_store32(f64::from(passed)));
    let duration = x87_load32(e.mem.u32(base + PROCEDURE_DURATION));
    // FCOMPP of the duration with the time passed, C0 and C2 tested: true
    // only when below (ordered).
    duration < passed
}

/// An activity's start hour and duration (`00643540`, cdecl: the activity
/// and the addresses of the two floats).
const ACTIVITY_TIMES: u32 = 0x0064_3540;
/// `FLT_MAX` (`float`), the starting best wait.
const FLOAT_MAX: u32 = 0x0101_6970;
/// The default total time (`24.0`, `float`).
const DEFAULT_TOTAL_TIME: u32 = 0x0104_e0e8;
/// The `float` the class keeps at -1 (`-1.0`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `20.0` (`float`).
const TWENTY: u32 = 0x0101_7868;
/// `90.0` (`float`).
const NINETY: u32 = 0x0102_49d8;
/// `20.0` (`double`).
const TWENTY_DOUBLE: u32 = 0x0102_fc70;
/// Two base forms (`011ca248`, `011ca244`) whose references get the 20.0
/// distance.
const SPECIAL_BASE_A: u32 = 0x011c_a248;
const SPECIAL_BASE_B: u32 = 0x011c_a244;
/// The `Setting` value getters (`00403e20` for a `float`, `0043d4d0` for an
/// `int`: the setting's address +4, or a zero in `01202800` for null).
const SETTING_FLOAT: u32 = 0x0040_3e20;
const SETTING_INT: u32 = 0x0043_d4d0;
/// The search radius setting (`011cd3f0`, a float).
const SETTING_RADIUS: u32 = 0x011c_d3f0;
/// The dialogue range added to the radius (`011ccf38`, a float).
const SETTING_DIALOGUE_RANGE: u32 = 0x011c_cf38;
/// The seconds a previous target stays remembered (`011cdb3c`, an int).
const SETTING_PREVIOUS_TARGET_SECONDS: u32 = 0x011c_db3c;
/// The two bounds of the next scan's delay in seconds (`011cdf60`,
/// `011cd0a4`, ints).
const SETTING_SCAN_DELAY_A: u32 = 0x011c_df60;
const SETTING_SCAN_DELAY_B: u32 = 0x011c_d0a4;
/// The fallback distance (`011cde98`, an int).
const SETTING_DEFAULT_DISTANCE: u32 = 0x011c_de98;
/// cdecl helper of two words (`00944460`) answering the scan delay.
const SCAN_DELAY_00944460: u32 = 0x0094_4460;
/// The reference's cell (`008d6f30`, the word at +0x40).
const REFERENCE_CELL: u32 = 0x008d_6f30;
/// `TESObjectCELL::GetWorldSpace` (`0054ddd0`).
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// `TESObjectREFR::GetWorldSpace` (`00575d70`).
const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
/// Bit 0 of a cell's byte at +0x24 (`00425fd0`).
const CELL_FLAG_0: u32 = 0x0042_5fd0;
/// `TESPackage::GetLocationCoord` (`00675de0`: the package, the result
/// address and the actor; answers the address of three floats).
const PACKAGE_GET_LOCATION_COORD: u32 = 0x0067_5de0;
/// `TESPackage::GetLocationCell` (`00675c20`, the package and the actor).
const PACKAGE_GET_LOCATION_CELL: u32 = 0x0067_5c20;
/// The package's location radius for the actor (`00676280`).
const PACKAGE_LOCATION_RADIUS: u32 = 0x0067_6280;
/// `TESDataHandler::EnumReferencesCloseToPoint` (`0046f280`).
const ENUM_REFERENCES_CLOSE_TO_POINT: u32 = 0x0046_f280;
/// The two-word functor constructor (`004b0680`: the functor, then its two
/// words).
const FUNCTOR_CONSTRUCT: u32 = 0x004b_0680;
/// The scan's callback (`009f5070`), passed by address.
const SCAN_CALLBACK: u32 = 0x009f_5070;
/// `Actor::GetBestFoodItem` (`00891d30`).
const ACTOR_GET_BEST_FOOD_ITEM: u32 = 0x0089_1d30;
/// The word at +4 of an object (`00726070`): a choice's procedure.
const WORD_AT_4: u32 = 0x0072_6070;
/// A reference's base form (`007af430`, the word at +0x20).
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
/// A form's type byte (`00401170`, the byte at +4).
const FORM_TYPE: u32 = 0x0040_1170;
/// The reference flag tests of the scan (`00440d80` bit 0x20, `004077c0`
/// bit 0x4000, `00440da0` bit 0x800 of the word at +8).
const REFERENCE_FLAG_0X20: u32 = 0x0044_0d80;
const REFERENCE_FLAG_0X4000: u32 = 0x0040_77c0;
const REFERENCE_FLAG_0X800: u32 = 0x0044_0da0;
/// An actor test with the reference (`008859e0`).
const ACTOR_TEST_008859E0: u32 = 0x0088_59e0;
/// A reference test (`0057b500`) that skips the reference.
const REFERENCE_TEST_0057B500: u32 = 0x0057_b500;
/// `TESObjectREFR::GetOwner` (`00567790`).
const REFERENCE_GET_OWNER: u32 = 0x0056_7790;
/// `TESObjectREFR::IsAnOwner` (`005785e0`: the reference, the actor and a
/// flag).
const REFERENCE_IS_AN_OWNER: u32 = 0x0057_85e0;
/// The idle collection's getters (`0092b440`, `00479e80`) and its test with
/// the actor and the reference (`00479fb0`).
const IDLE_COLLECTION_VALUE_0092B440: u32 = 0x0092_b440;
const IDLE_COLLECTION_VALUE_00479E80: u32 = 0x0047_9e80;
const IDLE_COLLECTION_TEST: u32 = 0x0047_9fb0;
/// Bit 2 of the package's word at +0x24 clear (`00901c50`).
const PACKAGE_WORD_24_BIT_2_CLEAR: u32 = 0x0090_1c50;
/// The player reference (`011dea3c`).
const PLAYER: u32 = 0x011d_ea3c;
/// `BSFurnitureMarker::Find` (`00c54400`, cdecl, the model).
const FURNITURE_MARKER_FIND: u32 = 0x00c5_4400;
/// A furniture marker getter (`00509490`).
const FURNITURE_MARKER_VALUE: u32 = 0x0050_9490;
/// `TESFurniture::CanSitOn` (`005093f0`) and `CanSleepOn` (`00509420`).
const FURNITURE_CAN_SIT_ON: u32 = 0x0050_93f0;
const FURNITURE_CAN_SLEEP_ON: u32 = 0x0050_9420;
/// `TESObjectREFR::GetModelBoundSize` (`00571600`, a float in `ST0`).
const REFERENCE_GET_MODEL_BOUND_SIZE: u32 = 0x0057_1600;
/// The actor's +0x90 test (`0087f990`).
const ACTOR_VALUE_0087F990: u32 = 0x0087_f990;
/// The procedure's duration (`006435c0`, cdecl: `0087f990`'s value and the
/// procedure; a float in `ST0`).
const PROCEDURE_DURATION_006435C0: u32 = 0x0064_35c0;
/// cdecl helper of two words (`00647b70`) applied to a choice's score.
const SCORE_00647B70: u32 = 0x0064_7b70;
/// Sets a choice's full score (`0041fd00`, the word at +0xC).
const CHOICE_SET_FULL_SCORE: u32 = 0x0041_fd00;
/// A choice's full score (`0084e3a0`, the word at +0xC).
const CHOICE_FULL_SCORE: u32 = 0x0084_e3a0;
/// The difference of two positions (`00439ef0`: the first position, the
/// result address and the second position; answers the result).
const POSITION_DIFFERENCE: u32 = 0x0043_9ef0;
/// A vector's squared length (`004a7290`, in `ST0`).
const SQUARED_LENGTH: u32 = 0x004a_7290;
/// `TESObjectREFR::HasFreeMarker` (`00568260`).
const REFERENCE_HAS_FREE_MARKER: u32 = 0x0056_8260;
/// `TESObjectREFR::GetFirstFreeMarkerIndex` (`005682c0`).
const REFERENCE_GET_FIRST_FREE_MARKER_INDEX: u32 = 0x0056_82c0;
/// `Actor::GetShouldAttackActor` (`008b06d0`).
const ACTOR_GET_SHOULD_ATTACK_ACTOR: u32 = 0x008b_06d0;
/// `_rand` (`00ecadb8`).
const RAND: u32 = 0x00ec_adb8;
/// `BSSimpleArray<SandBoxChoiceListItem,1024>`'s release (`008454f0`, the
/// array and a flag).
const CHOICE_ARRAY_RELEASE: u32 = ARRAY_RELEASE_008454F0;
/// `BGSSaveGameBuffer::SaveVariableSizedValue` (`00865f60`).
const SAVE_VARIABLE_SIZED_VALUE: u32 = 0x0086_5f60;
/// `SandBoxActorPackageData::iPrevTargetChangedTimeStamp` (Xbox PDB) +0x48
/// and `cProcedureWeight` +0x5C.
const PREV_TARGET_CHANGED_TIME_STAMP: u32 = 0x48;
const PROCEDURE_WEIGHT: u32 = 0x5c;
/// `SandBoxActorPackageData::iIdleMarkers` (Xbox PDB) +0x38.
const IDLE_MARKERS: u32 = 0x38;
/// The package's address +0x38 (`0041d8a0`); the word at +4 there (`00726070`)
/// is the default total time.
const PACKAGE_LIST_0041D8A0: u32 = 0x0041_d8a0;

/// A setting's value address.
fn setting(e: &mut Engine, getter: u32, setting: u32) -> u32 {
    e.call(getter, &args![setting]).u32()
}

/// Copies the three words at `from` to `to`.
fn copy_point(e: &mut Engine, from: u32, to: u32) {
    for word in 0..3 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// `SandBoxChoiceListItem(target, procedure, 1)` built in a stack slot and
/// added to the potential targets.
fn add_choice(e: &mut Engine, data: u32, target: u32, procedure: u32) {
    e.with_stack(CHOICE_SIZE, |e, item| {
        let item = fn_009f3800(e, item, Ptr::new(target), procedure, 1);
        fn_009f6340(e, Ptr::new(data + POTENTIAL_TARGETS), item);
    });
}

// Translated from 009f3e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the next activity once the current one is over (`009f3c90`):
/// among the four activities (`00643540` gives each start hour and
/// duration; one already ended today starts tomorrow, at its start plus
/// 24), skipping the current one when it ended and has not been passed
/// today, the one with the least wait from the calendar's hour is taken,
/// at once when its wait is not above 0.0. The chosen activity, start
/// hour, start day (tomorrow for a next-day one) and duration are stored.
pub fn fn_009f3e80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let picked = e.with_stack(9, |e, slots| {
        let ended = Ptr::new(slots.addr() + 8);
        if !fn_009f3c90(e, this, ended) {
            return None;
        }
        let day = e.call(CALENDAR_DAYS, &args![CALENDAR]).u32();
        let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
        let hour = x87_store32(hour);
        let max = x87_quiet_bits(e.mem.u32(FLOAT_MAX));
        let mut best_wait = max;
        let mut best_start = max;
        let mut best_duration = max;
        e.mem.set_u32(slots.addr(), max);
        e.mem.set_u32(slots.addr() + 4, max);
        let mut best_activity = 0xffff_fffeu32;
        let mut best_day = 0u32;
        let mut found = false;
        let hours_per_day: f64 = e.global(HOURS_PER_DAY);
        let zero: f64 = e.global(ZERO);
        let mut activity = 0i32;
        while activity < 4 {
            let start_slot = slots.addr();
            let duration_slot = slots.addr() + 4;
            e.call(ACTIVITY_TIMES, &args![activity, start_slot, duration_slot]);
            let start = e.mem.u32(start_slot);
            let duration = e.mem.u32(duration_slot);
            let end = x87_add(x87_load32(start), x87_m32(duration));
            // FCOMPP of the hour with the end, C0 tested: next day when not
            // below and not unordered.
            let next_day = x87_load32(hour) >= end;
            if e.mem.u8(ended.addr()) != 0
                && !next_day
                && activity as u32 == e.mem.u32(base + ACTIVITY)
            {
                activity += 1;
                continue;
            }
            let begin = if next_day {
                x87_add(x87_load32(start), hours_per_day)
            } else {
                x87_load32(start)
            };
            let begin = x87_store32(begin);
            let wait = x87_store32(x87_sub(x87_load32(begin), x87_m32(hour)));
            // FCOMP with 0.0, C0 and C3 tested: at once when below or equal.
            let immediate = x87_load32(wait) <= zero;
            // FCOMPP of the best wait with the wait: taken when greater.
            if immediate || x87_load32(best_wait) > x87_load32(wait) {
                best_wait = wait;
                best_activity = activity as u32;
                best_start = x87_quiet_bits(start);
                best_day = if next_day { day.wrapping_add(1) } else { day };
                best_duration = x87_quiet_bits(duration);
                found = true;
                if immediate {
                    break;
                }
            }
            activity += 1;
        }
        found.then_some((best_activity, best_start, best_day, best_duration))
    });
    if let Some((activity, start, day, duration)) = picked {
        e.mem.set_u32(base + ACTIVITY, activity);
        e.mem.set_u32(base + ACTIVITY_START_HOUR, start);
        e.mem.set_u32(base + ACTIVITY_START_DAY, day);
        e.mem.set_u32(base + ACTIVITY_DURATION, duration);
    }
}

/// The scan's or the pick's centre: the actor's position and cell, the
/// package's location (coordinates, radius when positive) and the
/// sandbox location's position and cell. Answers the cell and the radius
/// bits; the position is written at `position`.
fn search_centre(
    e: &mut Engine,
    base: u32,
    actor: Ptr,
    position: u32,
    cell_by_package: bool,
) -> (u32, u32, u32) {
    let radius_address = setting(e, SETTING_FLOAT, SETTING_RADIUS);
    let mut radius = x87_quiet_bits(e.mem.u32(radius_address));
    let mut cell = 0;
    if !cell_by_package {
        cell = e.call(REFERENCE_CELL, &args![actor]).u32();
    }
    let actor_position = e.vcall(actor.addr(), 0x1f4, &args![]).u32();
    copy_point(e, actor_position, position);
    if cell_by_package {
        cell = e.call(REFERENCE_CELL, &args![actor]).u32();
    }
    let package = e
        .call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![actor])
        .u32();
    if package != 0 {
        // The point is the frame's local at stack offset -0x38: `00675de0` fills it and answers its address, and
        // the centre is copied from it, so the copy is made while it is there. (Scoped to
        // `00675de0` alone, the copy read a freed block.)
        e.with_stack(12, |e, result| {
            let coord = e
                .call(PACKAGE_GET_LOCATION_COORD, &args![package, result, actor])
                .u32();
            copy_point(e, coord, position);
        });
        if cell_by_package {
            cell = e
                .call(PACKAGE_GET_LOCATION_CELL, &args![package, actor])
                .u32();
        }
        let package_radius = e
            .call(PACKAGE_LOCATION_RADIUS, &args![package, actor])
            .i32();
        radius = if package_radius > 0 {
            x87_store32(f64::from(package_radius))
        } else {
            x87_quiet_bits(radius)
        };
        radius = x87_quiet_bits(radius);
    }
    let location = e.mem.u32(base + LOCATION);
    if location != 0 {
        let location_position = e.vcall(location, 0x1f4, &args![]).u32();
        copy_point(e, location_position, position);
        cell = e.call(REFERENCE_CELL, &args![location]).u32();
    }
    (cell, radius, package)
}

// Translated from 009f54d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::ScanWorld` (Xbox PDB): empties the potential
/// targets and the bed, chair and food flags, then lets the data handler
/// enumerate the references close to the search centre (the actor, the
/// package's location coordinates and radius, the sandbox location) with
/// the callback `009f5070` and a functor of the actor and `this`. With
/// food allowed (`009f45e0`) and a best food item of non-zero count, an
/// eat choice without target is added and the food flag set; with
/// wandering allowed (`009f5690`), a wander choice. The package is read
/// even when the actor has none, as in the exe.
pub fn sand_box_actor_package_data_scan_world(e: &mut Engine, this: Ptr, actor: Ptr) {
    let base = this.addr();
    e.call(CHOICE_ARRAY_RELEASE, &args![base + POTENTIAL_TARGETS, 1u32]);
    e.mem.set_u8(base + BED, 0);
    e.mem.set_u8(base + CHAIR, 0);
    e.mem.set_u8(base + FOOD, 0);
    let package = e.with_stack(0x14, |e, locals| {
        let position = locals.addr();
        let functor = locals.addr() + 0xc;
        let (cell, radius, package) = search_centre(e, base, actor, position, false);
        e.call(FUNCTOR_CONSTRUCT, &args![functor, actor, this]);
        let handler = e.global::<u32>(DATA_HANDLER);
        e.call(
            ENUM_REFERENCES_CLOSE_TO_POINT,
            &args![
                handler,
                cell,
                position,
                radius,
                position,
                radius,
                SCAN_CALLBACK,
                functor
            ],
        );
        package
    });
    let package = Ptr::new(package);
    if fn_009f45e0(e, package) {
        let food = e.call(ACTOR_GET_BEST_FOOD_ITEM, &args![actor]).u32();
        if food != 0 && e.call(WORD_AT_8, &args![food]).u32() != 0 {
            add_choice(e, base, 0, 2);
            e.mem.set_u8(base + FOOD, 1);
        }
    }
    if fn_009f5690(e, package) {
        add_choice(e, base, 0, 3);
    }
}

/// The ownership test of the scan: no owner, or the actor is one
/// (`TESObjectREFR::IsAnOwner(reference, actor, 1)`).
fn may_use(e: &mut Engine, reference: u32, actor: u32) -> bool {
    e.call(REFERENCE_GET_OWNER, &args![reference]).u32() == 0
        || e.call(REFERENCE_IS_AN_OWNER, &args![reference, actor, 1u32])
            .bool()
}

// Translated from 009f5070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scan's reference callback (cdecl: the reference and the functor of
/// the actor and the sandbox data); always answers false (go on), and
/// false early when it skips. A reference flagged 0x20, 0x4000 or 0x800,
/// or one the actor's virtual `+0x1A0` and `008859e0` turn down, a missing
/// actor, base form or package, and `0057b500` skip it. By its base form's
/// type: 0x30 (with `009f54b0` allowed and the actor allowed to use it) an
/// idle-marker choice when its idle collection accepts the actor; 0x2A and
/// 0x2B (with `00901c50`) a dialogue choice for another actor that is not
/// the player; 0x27 with a furniture marker a sleep choice (bed allowed,
/// can sleep on), else a sit choice (chair allowed, can sit on, or neither
/// sit nor sleep); 0x1D and 0x2F (food allowed) an eat choice. Each choice
/// sets the matching flag (bed, chair, food).
// The exe tests sit-on and then neither-sit-nor-sleep as two branches that do the same thing;
// merging them would change the order and number of its calls.
#[allow(clippy::if_same_then_else)]
pub fn fn_009f5070(e: &mut Engine, reference: Ptr, functor: Ptr) -> bool {
    let reference = reference.addr();
    let data = e.call(WORD_AT_4, &args![functor]).u32();
    let actor = e.call(CHOICE_TARGET, &args![functor]).u32();
    if data == 0 {
        return false;
    }
    if actor == 0 || e.call(REFERENCE_BASE_FORM, &args![actor]).u32() == 0 {
        return false;
    }
    if reference != 0 {
        if e.call(REFERENCE_FLAG_0X20, &args![reference]).bool()
            || e.call(REFERENCE_FLAG_0X4000, &args![reference]).bool()
            || e.call(REFERENCE_FLAG_0X800, &args![reference]).bool()
        {
            return false;
        }
        if e.vcall(actor, 0x1a0, &args![0u32]).bool()
            && !e.call(ACTOR_TEST_008859E0, &args![actor, reference]).bool()
        {
            return false;
        }
    }
    let package = e
        .call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![actor])
        .u32();
    if package == 0 {
        return false;
    }
    let package = Ptr::new(package);
    if e.call(REFERENCE_TEST_0057B500, &args![reference]).bool() {
        return false;
    }
    let base_form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    let kind = e.call(FORM_TYPE, &args![base_form]).u32();
    match kind.wrapping_sub(0x1d) {
        // 0x30
        0x13 => {
            if fn_009f54b0(e, package) && may_use(e, reference, actor) {
                let form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
                let collection = form + 0x30;
                e.call(IDLE_COLLECTION_VALUE_0092B440, &args![collection]);
                e.call(IDLE_COLLECTION_VALUE_00479E80, &args![collection]);
                if e.call(IDLE_COLLECTION_TEST, &args![collection, actor, reference])
                    .bool()
                {
                    add_choice(e, data, reference, 4);
                }
            }
        }
        // 0x2A, 0x2B
        0x0d | 0x0e => {
            if e.call(PACKAGE_WORD_24_BIT_2_CLEAR, &args![package]).bool()
                && e.vcall(reference, 0x100, &args![]).bool()
                && reference != e.global::<u32>(PLAYER)
                && reference != actor
            {
                add_choice(e, data, reference, 5);
            }
        }
        // 0x27
        0x0a => {
            let model = e.vcall(reference, 0x1d0, &args![]).u32();
            if model == 0 {
                return false;
            }
            let marker = e.call(FURNITURE_MARKER_FIND, &args![model]).u32();
            if marker == 0 {
                return false;
            }
            if e.call(FURNITURE_MARKER_VALUE, &args![marker]).u32() == 0 {
                return false;
            }
            let furniture = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
            if fn_009f4600(e, package)
                && e.call(FURNITURE_CAN_SLEEP_ON, &args![furniture]).bool()
                && may_use(e, reference, actor)
            {
                add_choice(e, data, reference, 1);
                e.mem.set_u8(data + BED, 1);
            } else if fn_009f4620(e, package)
                && e.call(FURNITURE_CAN_SIT_ON, &args![furniture]).bool()
                && may_use(e, reference, actor)
            {
                add_choice(e, data, reference, 0);
                e.mem.set_u8(data + CHAIR, 1);
            } else if fn_009f4620(e, package)
                && !e.call(FURNITURE_CAN_SIT_ON, &args![furniture]).bool()
                && !e.call(FURNITURE_CAN_SLEEP_ON, &args![furniture]).bool()
                && may_use(e, reference, actor)
            {
                add_choice(e, data, reference, 0);
                e.mem.set_u8(data + CHAIR, 1);
            }
        }
        // 0x1D, 0x2F
        0x00 | 0x12 => {
            if fn_009f45e0(e, package) && may_use(e, reference, actor) {
                add_choice(e, data, reference, 2);
                e.mem.set_u8(data + FOOD, 1);
            }
        }
        _ => {}
    }
    false
}

/// The cell of a reference, kept only when its bit 0 at +0x24 is set.
fn flagged_cell(e: &mut Engine, reference: u32) -> u32 {
    let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
    if cell != 0 && !e.call(CELL_FLAG_0, &args![cell]).bool() {
        return 0;
    }
    cell
}

/// The location's extra data 0x51 starts its change time: 0, then with
/// extra data 0x6F the timer plus its float times 1000 (truncated).
fn start_change_time(e: &mut Engine, base: u32) {
    let location = e.mem.u32(base + LOCATION);
    if e.call(EXTRA_DATA_LIST, &args![location]).u32() == 0 {
        return;
    }
    let location = e.mem.u32(base + LOCATION);
    let list = e.call(EXTRA_DATA_LIST, &args![location]).u32();
    if e.call(EXTRA_DATA_0X51, &args![list]).u32() == 0 {
        return;
    }
    e.mem.set_u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS, 0);
    let location = e.mem.u32(base + LOCATION);
    let list = e.call(EXTRA_DATA_LIST, &args![location]).u32();
    let extra = e.call(EXTRA_DATA_0X6F, &args![list]).u32();
    if extra == 0 {
        return;
    }
    let start = now(e);
    let seconds = e.call(FLOAT_AT_0, &args![extra]).f64();
    let thousand: f64 = e.global(THOUSAND);
    let milliseconds = x87_fistp64_chop(X87Ext::load64(x87_mul(seconds, thousand))) as u32;
    e.mem.set_u32(
        base + NEXT_TIME_TO_CHANGE_LOCATIONS,
        start.wrapping_add(milliseconds),
    );
}

// Translated from 009f56b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the actor to the sandbox location (unnamed in the engine map).
/// Without a location, an actor's package's virtual `+0x13C` decides
/// (with the actor, 0, -1.0 and 0). Otherwise false unless the actor and
/// the location share a flagged cell, or, with neither in one, a
/// worldspace. The distance is `distance`, or when 0.0: 20.0 for the two
/// special base forms, 90.0 for an actor location whose virtual `+0x214`
/// is 9 (then not the last argument's way), its bound size plus 20.0 for
/// another actor, else the bound size plus 20.0 (0.0 for base form types
/// 0x15, 0x1B, 0x20 and 0x21), not below 0.0; still 0.0, the default
/// distance setting. The actor's virtual `+0x2CC` (position, distance,
/// flag, 1) moves it; when it does, the location's change time starts
/// (extra data 0x51 and 0x6F) if unset, and for procedures 0 to 5 the
/// duration is recomputed (`006435c0`).
pub fn fn_009f56b0(e: &mut Engine, this: Ptr, actor: Ptr, distance: f32) -> bool {
    let base = this.addr();
    if actor.is_null() {
        return false;
    }
    let mut mover = 0u32;
    if e.vcall(actor.addr(), 0x100, &args![]).bool() {
        mover = actor.addr();
    }
    if e.mem.u32(base + LOCATION) == 0 {
        if mover != 0 {
            let package = e
                .call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![mover])
                .u32();
            if package != 0 {
                let minus_one = x87_quiet_bits(e.mem.u32(MINUS_ONE));
                return e
                    .vcall(package, 0x13c, &args![mover, 0u32, minus_one, 0u32])
                    .bool();
            }
        }
        return false;
    }
    let location = e.mem.u32(base + LOCATION);
    let location_world = e.call(REFERENCE_GET_WORLD_SPACE, &args![location]).u32();
    let location_cell = flagged_cell(e, location);
    let actor_world = e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32();
    let actor_cell = flagged_cell(e, actor.addr());
    let same = if actor_cell != 0 || location_cell != 0 {
        actor_cell == location_cell
    } else {
        actor_world == location_world
    };
    if !same {
        return false;
    }
    let zero: f64 = e.global(ZERO);
    let mut distance = x87_quiet_bits(distance.to_bits());
    let mut fixed = false;
    let location = e.mem.u32(base + LOCATION);
    let position = e.vcall(location, 0x1f4, &args![]).u32();
    // FCOMP with 0.0, C3 and C2 tested: equal (ordered).
    if x87_load32(distance) == zero {
        let form = e.call(REFERENCE_BASE_FORM, &args![location]).u32();
        let special_a = e.global::<u32>(SPECIAL_BASE_A);
        let special_b = e.global::<u32>(SPECIAL_BASE_B);
        if form == special_a || e.call(REFERENCE_BASE_FORM, &args![location]).u32() == special_b {
            distance = x87_quiet_bits(e.mem.u32(TWENTY));
        } else if e.vcall(location, 0x100, &args![]).bool() {
            if e.vcall(location, 0x214, &args![]).u32() == 9 {
                distance = x87_quiet_bits(e.mem.u32(NINETY));
                fixed = true;
            } else {
                let size = e
                    .call(REFERENCE_GET_MODEL_BOUND_SIZE, &args![location])
                    .f64();
                let twenty: f64 = e.global(TWENTY_DOUBLE);
                distance = x87_store32(x87_add(size, twenty));
            }
        } else {
            let mut extra = x87_quiet_bits(e.mem.u32(TWENTY));
            let form = e.call(REFERENCE_BASE_FORM, &args![location]).u32();
            let kind = e.call(FORM_TYPE, &args![form]).u32();
            if matches!(kind.wrapping_sub(0x15), 0 | 6 | 11 | 12) {
                extra = 0;
            }
            let size = e
                .call(REFERENCE_GET_MODEL_BOUND_SIZE, &args![location])
                .f64();
            distance = x87_store32(x87_add(size, x87_m32(extra)));
            // FCOMP with 0.0, C0 and C2 tested: below (ordered) becomes 0.
            if x87_load32(distance) < zero {
                distance = 0;
            }
        }
    }
    if x87_load32(distance) == zero {
        let value = setting(e, SETTING_INT, SETTING_DEFAULT_DISTANCE);
        distance = x87_store32(f64::from(e.mem.i32(value)));
    }
    let moved = e
        .vcall(
            actor.addr(),
            0x2cc,
            &args![position, x87_quiet_bits(distance), u32::from(!fixed), 1u32],
        )
        .bool();
    if !moved {
        return false;
    }
    if e.mem.u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS) == 0xffff_ffff {
        start_change_time(e, base);
    }
    let procedure = e.mem.i32(base + PROCEDURE);
    if procedure > -1 && procedure < 6 {
        let value = e.call(ACTOR_VALUE_0087F990, &args![mover]).u32();
        let duration = e
            .call(PROCEDURE_DURATION_006435C0, &args![value, procedure])
            .f64();
        e.mem
            .set_u32(base + PROCEDURE_DURATION, x87_store32(duration));
    }
    true
}

// Translated from 009f5a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::SaveGame` (Xbox PDB): the state, procedure,
/// duration, last change, total time, next change, duration again, scan
/// time, the four flags, the idle markers, the marker byte, the time since
/// the previous target changed, the activity's day, hour, duration and
/// type, the six procedure weights, the form ids of the location, the
/// current and the previous target, then the choice count and each choice.
pub fn sand_box_actor_package_data_save_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    let fields: [(u32, u32); 14] = [
        (STATE, 4),
        (PROCEDURE, 4),
        (PROCEDURE_DURATION, 4),
        (LAST_TIME_CHANGED, 4),
        (TOTAL_TIME, 4),
        (NEXT_TIME_TO_CHANGE_LOCATIONS, 4),
        (PROCEDURE_DURATION, 4),
        (NEXT_TIME_TO_SCAN_WORLD, 4),
        (SHOULD_PICK_NEW_PROCEDURE, 1),
        (FOOD, 1),
        (CHAIR, 1),
        (BED, 1),
        (IDLE_MARKERS, 4),
        (CURRENT_TARGET_MARKER, 1),
    ];
    for (offset, size) in fields {
        e.call(SAVE_DATA, &args![buffer, base + offset, size, 0u32]);
    }
    let time = now(e);
    let since = time.wrapping_sub(e.mem.u32(base + PREV_TARGET_CHANGED_TIME_STAMP));
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), since);
        e.call(SAVE_DATA, &args![buffer, slot, 4u32, 0u32]);
    });
    let fields: [(u32, u32); 5] = [
        (ACTIVITY_START_DAY, 4),
        (ACTIVITY_START_HOUR, 4),
        (ACTIVITY_DURATION, 4),
        (ACTIVITY, 4),
        (PROCEDURE_WEIGHT, 6),
    ];
    for (offset, size) in fields {
        e.call(SAVE_DATA, &args![buffer, base + offset, size, 0u32]);
    }
    for offset in [LOCATION, CURRENT_TARGET_OBJECT, PREV_TARGET_OBJECT] {
        let form = e.mem.u32(base + offset);
        e.call(SAVE_FORM_ID, &args![buffer, form, 0u32]);
    }
    let targets = base.wrapping_add(POTENTIAL_TARGETS);
    let count = if targets != 0 {
        e.call(WORD_AT_8, &args![targets]).u32()
    } else {
        0
    };
    e.call(SAVE_VARIABLE_SIZED_VALUE, &args![buffer, count]);
    let mut index = 0u32;
    while index < count {
        let choice = fn_009f6190(e, Ptr::new(targets), index);
        fn_009f3850(e, choice, buffer);
        index += 1;
    }
}

// Translated from 009f4700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the next procedure and its target (unnamed in the engine map).
/// Clears the wish for a new procedure, picks the activity (`009f3e80`),
/// gives a zero total time the package's (`package` +0x38 list count) or
/// 24.0, remembers a changed current target as the previous one for the
/// setting's seconds (and forgets it after), lets go of the current
/// marker, resets the procedure, and rescans (`ScanWorld`) once the scan
/// time has come, setting the next one (`00944460` of the two delay
/// settings). The procedure weights are the activity's procedure only
/// (10) inside a met activity, else the stored weights with sleep 0, eat
/// +20 with food and -15 without a chair, dialogue 0 when the actor's
/// +0x68 object's virtual `+0xB8` is above 0.0. Each potential target
/// scores its base score times its procedure's weight (`00647b70`), 0
/// for the previous target, a target out of range (squared distance above
/// the radius squared, the radius plus the dialogue range squared for
/// dialogue), without a free marker, or, for dialogue, one the actor's
/// process refuses or either would attack; targets in another flagged cell
/// or worldspace are skipped without a score. With a positive total a
/// choice is drawn by `rand() % total` over the full scores (the exe reads
/// the item at the count when the draw passes them all): its procedure,
/// target, first free marker (0 when none), duration (`006435c0`); every
/// weight below 255 goes up by one and the chosen one becomes 1; its
/// marker is reserved. Without, the procedure is reset. The last change
/// becomes the calendar's hour.
pub fn fn_009f4700(e: &mut Engine, this: Ptr, actor: Ptr, package: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + SHOULD_PICK_NEW_PROCEDURE, 0);
    fn_009f3e80(e, this);
    let zero: f64 = e.global(ZERO);
    if x87_load32(e.mem.u32(base + TOTAL_TIME)) == zero {
        if !package.is_null() && e.call(PACKAGE_LIST_0041D8A0, &args![package]).u32() != 0 {
            let list = e.call(PACKAGE_LIST_0041D8A0, &args![package]).u32();
            let count = e.call(WORD_AT_4, &args![list]).i32();
            e.mem
                .set_u32(base + TOTAL_TIME, x87_store32(f64::from(count)));
        }
        if x87_load32(e.mem.u32(base + TOTAL_TIME)) == zero {
            let value = x87_quiet_bits(e.mem.u32(DEFAULT_TOTAL_TIME));
            e.mem.set_u32(base + TOTAL_TIME, value);
        }
    }
    let current = e.mem.u32(base + CURRENT_TARGET_OBJECT);
    if current != 0 && e.mem.u32(base + PREV_TARGET_OBJECT) != current {
        let time = now(e);
        let seconds = setting(e, SETTING_INT, SETTING_PREVIOUS_TARGET_SECONDS);
        let seconds = e.mem.u32(seconds);
        e.mem.set_u32(
            base + PREV_TARGET_CHANGED_TIME_STAMP,
            time.wrapping_add(seconds.wrapping_mul(1000)),
        );
        let current = e.mem.u32(base + CURRENT_TARGET_OBJECT);
        e.mem.set_u32(base + PREV_TARGET_OBJECT, current);
    }
    if e.mem.u32(base + PREV_TARGET_OBJECT) != 0 {
        let time = now(e);
        if time > e.mem.u32(base + PREV_TARGET_CHANGED_TIME_STAMP) {
            e.mem.set_u32(base + PREV_TARGET_OBJECT, 0);
        }
    }
    if e.mem.u32(base + CURRENT_TARGET_OBJECT) != 0 {
        let marker = u32::from(e.mem.u8(base + CURRENT_TARGET_MARKER));
        let object = e.mem.u32(base + CURRENT_TARGET_OBJECT);
        e.call(SET_MARKER_RESERVED, &args![object, marker, 0u32]);
    }
    e.mem.set_u32(base + CURRENT_TARGET_OBJECT, 0);
    e.mem.set_u8(base + CURRENT_TARGET_MARKER, 0x7f);
    e.mem.set_u32(base + PROCEDURE, 0xffff_ffff);
    e.mem.set_f32(base + PROCEDURE_DURATION, 0.0);
    let time = now(e);
    if time >= e.mem.u32(base + NEXT_TIME_TO_SCAN_WORLD) {
        fn_009f4060(e, this);
        sand_box_actor_package_data_scan_world(e, this, actor);
        let time = now(e);
        let first = setting(e, SETTING_INT, SETTING_SCAN_DELAY_A);
        let first = e.mem.u32(first).wrapping_mul(1000);
        let second = setting(e, SETTING_INT, SETTING_SCAN_DELAY_B);
        let second = e.mem.u32(second).wrapping_mul(1000);
        let delay = e.call(SCAN_DELAY_00944460, &args![second, first]).u32();
        e.mem
            .set_u32(base + NEXT_TIME_TO_SCAN_WORLD, time.wrapping_add(delay));
    }
    let in_activity = fn_009f3d50(e, this) && fn_009f3e20(e, this);
    let mut weights = [0i32; 6];
    if in_activity {
        let activity = e.mem.u32(base + ACTIVITY);
        let procedure = e
            .mem
            .u32(PROCEDURES_FOR_ACTIVITIES.wrapping_add(activity.wrapping_mul(4)));
        // The exe indexes its six-word local array without a check.
        weights[procedure as usize] = 10;
    } else {
        for (i, weight) in weights.iter_mut().enumerate() {
            *weight = i32::from(e.mem.u8(base + PROCEDURE_WEIGHT + i as u32));
        }
        weights[1] = 0;
        if e.mem.u8(base + FOOD) != 0 {
            weights[2] = weights[2].wrapping_add(0x14);
        }
        if e.mem.u8(base + CHAIR) == 0 {
            weights[2] = weights[2].wrapping_sub(0xf);
        }
        if e.call(OBJECT_AT_0X68, &args![actor]).u32() != 0 {
            let object = e.call(OBJECT_AT_0X68, &args![actor]).u32();
            let value = e.vcall(object, 0xb8, &args![]).f64();
            // FCOMP with 0.0, C0 and C3 tested: above clears dialogue.
            if value > zero {
                weights[5] = 0;
            }
        }
    }
    let mut total = 0i32;
    let targets = Ptr::new(base + POTENTIAL_TARGETS);
    let count = e.call(WORD_AT_8, &args![targets]).i32();
    // The exe keeps the actor's +0x68 object here without reading it again.
    e.call(OBJECT_AT_0X68, &args![actor]);
    e.with_stack(0x24, |e, locals| {
        let position = locals.addr();
        let difference = locals.addr() + 0xc;
        let attack = locals.addr() + 0x18;
        let (cell, radius, _) = search_centre(e, base, actor, position, true);
        let world = if cell != 0 {
            e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32()
        } else {
            e.call(REFERENCE_GET_WORLD_SPACE, &args![actor]).u32()
        };
        let range = setting(e, SETTING_FLOAT, SETTING_DIALOGUE_RANGE);
        let far = x87_store32(x87_add(x87_load32(radius), x87_m32(e.mem.u32(range))));
        let near_squared = x87_store32(x87_mul(x87_load32(radius), x87_m32(radius)));
        let far_squared = x87_store32(x87_mul(x87_load32(far), x87_m32(far)));
        let mut index = 0i32;
        while index < count {
            let choice = fn_009f6190(e, targets, index as u32);
            index += 1;
            let base_score = e.call(WORD_AT_8, &args![choice]).i32();
            let procedure = e.call(WORD_AT_4, &args![choice]).u32();
            let weight = weights[procedure as usize];
            let score = e
                .call(
                    SCORE_00647B70,
                    &args![base_score.wrapping_mul(weight), 0u32],
                )
                .u32();
            e.call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![actor]);
            let target = e.call(CHOICE_TARGET, &args![choice]).u32();
            if target != 0 {
                if target == e.mem.u32(base + PREV_TARGET_OBJECT) {
                    e.call(CHOICE_SET_FULL_SCORE, &args![choice, 0u32]);
                    continue;
                }
                let mut flagged = false;
                let target_cell = e.call(REFERENCE_CELL, &args![target]).u32();
                if cell != 0
                    && target_cell != 0
                    && (e.call(CELL_FLAG_0, &args![cell]).bool()
                        || e.call(CELL_FLAG_0, &args![target_cell]).bool())
                {
                    flagged = true;
                    if cell != target_cell {
                        continue;
                    }
                }
                if !flagged {
                    let target_world = if target_cell != 0 {
                        e.call(CELL_GET_WORLD_SPACE, &args![target_cell]).u32()
                    } else {
                        e.call(REFERENCE_GET_WORLD_SPACE, &args![target]).u32()
                    };
                    if world != target_world {
                        continue;
                    }
                }
                let dialogue = e.call(WORD_AT_4, &args![choice]).u32() == 5;
                let limit = if dialogue { far_squared } else { near_squared };
                let limit = x87_quiet_bits(x87_quiet_bits(limit));
                let target_position = e.vcall(target, 0x1f4, &args![]).u32();
                let vector = e
                    .call(
                        POSITION_DIFFERENCE,
                        &args![target_position, difference, position],
                    )
                    .u32();
                let squared = e.call(SQUARED_LENGTH, &args![vector]).f64();
                // FCOMPP of the limit with the squared distance, C0 and C2
                // tested: out of range when below (ordered).
                if x87_load32(limit) < squared {
                    e.call(CHOICE_SET_FULL_SCORE, &args![choice, 0u32]);
                    continue;
                }
                if !e
                    .call(REFERENCE_HAS_FREE_MARKER, &args![target, 0u32])
                    .bool()
                {
                    e.call(CHOICE_SET_FULL_SCORE, &args![choice, 0u32]);
                    continue;
                }
                if dialogue {
                    let refused = if e.call(OBJECT_AT_0X68, &args![actor]).u32() == 0 {
                        true
                    } else {
                        let object = e.call(OBJECT_AT_0X68, &args![actor]).u32();
                        !e.vcall(object, 0x204, &args![actor, target]).bool()
                    };
                    if refused {
                        e.call(CHOICE_SET_FULL_SCORE, &args![choice, 0u32]);
                        continue;
                    }
                    e.mem.set_u32(attack, 0);
                    let hostile = e
                        .call(
                            ACTOR_GET_SHOULD_ATTACK_ACTOR,
                            &args![actor, target, 0u32, attack, 0u32],
                        )
                        .bool()
                        || e.call(
                            ACTOR_GET_SHOULD_ATTACK_ACTOR,
                            &args![target, actor, 0u32, attack, 0u32],
                        )
                        .bool();
                    if hostile {
                        e.call(CHOICE_SET_FULL_SCORE, &args![choice, 0u32]);
                        continue;
                    }
                }
            }
            e.call(CHOICE_SET_FULL_SCORE, &args![choice, score]);
            total = total.wrapping_add(score as i32);
        }
    });
    if total > 0 {
        let draw = e.call(RAND, &args![]).i32();
        let mut remaining = draw.wrapping_rem(total);
        let mut index = 0i32;
        while index < count {
            let choice = fn_009f6190(e, targets, index as u32);
            let score = e.call(CHOICE_FULL_SCORE, &args![choice]).i32();
            remaining = remaining.wrapping_sub(score);
            if remaining < 0 {
                break;
            }
            index += 1;
        }
        e.with_stack(CHOICE_SIZE, |e, chosen| {
            let choice = fn_009f6190(e, targets, index as u32);
            for word in 0..4 {
                let value = e.mem.u32(choice.addr() + 4 * word);
                e.mem.set_u32(chosen.addr() + 4 * word, value);
            }
            let procedure = e.call(WORD_AT_4, &args![chosen]).u32();
            e.mem.set_u32(base + PROCEDURE, procedure);
            let target = e.call(CHOICE_TARGET, &args![chosen]).u32();
            e.mem.set_u32(base + CURRENT_TARGET_OBJECT, target);
        });
        let marker = if e.mem.u32(base + CURRENT_TARGET_OBJECT) != 0 {
            let object = e.mem.u32(base + CURRENT_TARGET_OBJECT);
            e.call(REFERENCE_GET_FIRST_FREE_MARKER_INDEX, &args![object, 0u32])
                .i32()
        } else {
            -1
        };
        let marker = if marker < 0 { 0 } else { marker as u8 };
        e.mem.set_u8(base + CURRENT_TARGET_MARKER, marker);
        let value = e.call(ACTOR_VALUE_0087F990, &args![actor]).u32();
        let procedure = e.mem.u32(base + PROCEDURE);
        let duration = e
            .call(PROCEDURE_DURATION_006435C0, &args![value, procedure])
            .f64();
        e.mem
            .set_u32(base + PROCEDURE_DURATION, x87_store32(duration));
        for i in 0..6 {
            let weight = e.mem.u8(base + PROCEDURE_WEIGHT + i);
            let weight = if u32::from(weight) < 0xff {
                (u32::from(weight) + 1) as u8
            } else {
                weight
            };
            e.mem.set_u8(base + PROCEDURE_WEIGHT + i, weight);
        }
        let procedure = e.mem.u32(base + PROCEDURE);
        e.mem.set_u8(
            base.wrapping_add(procedure).wrapping_add(PROCEDURE_WEIGHT),
            1,
        );
        if e.mem.u32(base + CURRENT_TARGET_OBJECT) != 0 {
            let marker = u32::from(e.mem.u8(base + CURRENT_TARGET_MARKER));
            let object = e.mem.u32(base + CURRENT_TARGET_OBJECT);
            e.call(SET_MARKER_RESERVED, &args![object, marker, 1u32]);
        }
    } else {
        e.mem.set_u32(base + PROCEDURE, 0xffff_ffff);
        e.mem.set_f32(base + PROCEDURE_DURATION, 0.0);
        e.mem.set_u32(base + CURRENT_TARGET_OBJECT, 0);
        e.mem.set_u8(base + CURRENT_TARGET_MARKER, 0x7f);
    }
    let hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
    e.mem.set_u32(base + LAST_TIME_CHANGED, x87_store32(hour));
}

/// `BGSLoadGameBuffer`'s read of bytes (`00864980`, `thiscall` on the
/// buffer, `RET 8`: the destination and the size).
const LOAD_DATA: u32 = 0x0086_4980;
/// `BGSLoadGameBuffer::LoadFormID_ov2` (`008648e0`, engine map; `thiscall`
/// on the buffer, `RET 4`: the address of the form id word).
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// `BGSLoadGameBuffer::LoadVariableSizedValue` (`00864a60`, engine map;
/// `ECX` only).
const LOAD_VARIABLE_SIZED_VALUE: u32 = 0x0086_4a60;
/// `004839c0` (cdecl, one word): the form of an id.
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The type descriptors of `InitLoadGame`'s casts: `.?AVTESForm@@`
/// (`01183028`) to `.?AVTESObjectREFR@@` (`011841cc`).
const CAST_FROM_FORM: u32 = 0x0118_3028;
const CAST_TO_REFERENCE: u32 = 0x0118_41cc;
/// `009a4f40` (`thiscall` on a choice item, `RET 4`: the load buffer): the
/// item's own `InitLoadGame` step.
const CHOICE_INIT_LOAD_GAME_009A4F40: u32 = 0x009a_4f40;
/// `0092a980` (cdecl, one word): the test on a target-less choice's
/// procedure that drops the choice.
const PROCEDURE_TEST_0092A980: u32 = 0x0092_a980;
/// `009f6540` (`thiscall`, `RET 8`: the new reserved size and the count to
/// keep): reallocates the choice buffer.
const CHOICE_ARRAY_REALLOCATE_009F6540: u32 = 0x009f_6540;
/// The array's virtual slot (byte offset) that allocates a buffer of `n`
/// items.
const SLOT_ARRAY_ALLOCATE: u32 = 0x4;

// Translated from 009f38b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxChoiceListItem`'s load (unnamed in the engine map; `RET 4`), the
/// mirror of `009f3850`: the words at +4, +8 and +0xC (four bytes each,
/// `00864980`), then the target's form id into +0 (`LoadFormID_ov2`).
pub fn fn_009f38b0(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    for offset in [4u32, 8, 0xc] {
        e.call(LOAD_DATA, &args![buffer, base + offset, 4u32]);
    }
    e.call(LOAD_FORM_ID, &args![buffer, base]);
}

// Translated from 009f41c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): lets go of the target marker
/// (`009f4060`), asks for a new procedure (`bShouldPickNewProcedure` +0x34
/// set) and sets the state (+4) to 1.
pub fn fn_009f41c0(e: &mut Engine, this: Ptr) {
    fn_009f4060(e, this);
    let base = this.addr();
    e.mem.set_u8(base + SHOULD_PICK_NEW_PROCEDURE, 1);
    e.mem.set_u32(base + STATE, 1);
}

// Translated from 009f4680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::ForgetTargetAndPickProcedure` (engine map;
/// `RET 8`): with a current target object, every potential target choosing
/// it (`00559450`) gets its base score (+8) cleared (the count and index
/// compared signed); then the next procedure is picked (`009f4700`).
pub fn sand_box_actor_package_data_forget_target_and_pick_procedure(
    e: &mut Engine,
    this: Ptr,
    actor: Ptr,
    package: Ptr,
) {
    let base = this.addr();
    if e.mem.u32(base + CURRENT_TARGET_OBJECT) != 0 {
        let targets = Ptr::new(base + POTENTIAL_TARGETS);
        let count = e.call(WORD_AT_8, &args![targets]).u32() as i32;
        let mut index: i32 = 0;
        while index < count {
            let choice = fn_009f6190(e, targets, index as u32);
            let target = e.call(CHOICE_TARGET, &args![choice]).u32();
            if target == e.mem.u32(base + CURRENT_TARGET_OBJECT) {
                e.mem.set_u32(choice.addr() + 8, 0);
            }
            index = index.wrapping_add(1);
        }
    }
    fn_009f4700(e, this, actor, package);
}

// Translated from 009f5ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::LoadGame` (engine map; `RET 4`), the mirror of
/// `SaveGame`: the state, procedure, duration, last change, total time,
/// next change, duration again, scan time, the four flags, the idle
/// markers and the marker byte; the time since the previous target changed
/// (read into a local) becomes `iPrevTargetChangedTimeStamp` (+0x48) as now
/// minus it; the activity's day, hour, duration and type and the six
/// procedure weights; the form ids of the location, the current and the
/// previous target; then the choice count (`LoadVariableSizedValue`), the
/// array sized to it (`009f6200(count, 1)`) and each choice loaded
/// (`009f38b0`).
pub fn sand_box_actor_package_data_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    let fields: [(u32, u32); 14] = [
        (STATE, 4),
        (PROCEDURE, 4),
        (PROCEDURE_DURATION, 4),
        (LAST_TIME_CHANGED, 4),
        (TOTAL_TIME, 4),
        (NEXT_TIME_TO_CHANGE_LOCATIONS, 4),
        (PROCEDURE_DURATION, 4),
        (NEXT_TIME_TO_SCAN_WORLD, 4),
        (SHOULD_PICK_NEW_PROCEDURE, 1),
        (FOOD, 1),
        (CHAIR, 1),
        (BED, 1),
        (IDLE_MARKERS, 4),
        (CURRENT_TARGET_MARKER, 1),
    ];
    for (offset, size) in fields {
        e.call(LOAD_DATA, &args![buffer, base + offset, size]);
    }
    let since = e.with_stack(4, |e, slot| {
        e.call(LOAD_DATA, &args![buffer, slot, 4u32]);
        e.mem.u32(slot.addr())
    });
    let time = now(e);
    e.mem.set_u32(
        base + PREV_TARGET_CHANGED_TIME_STAMP,
        time.wrapping_sub(since),
    );
    let fields: [(u32, u32); 5] = [
        (ACTIVITY_START_DAY, 4),
        (ACTIVITY_START_HOUR, 4),
        (ACTIVITY_DURATION, 4),
        (ACTIVITY, 4),
        (PROCEDURE_WEIGHT, 6),
    ];
    for (offset, size) in fields {
        e.call(LOAD_DATA, &args![buffer, base + offset, size]);
    }
    for offset in [LOCATION, CURRENT_TARGET_OBJECT, PREV_TARGET_OBJECT] {
        e.call(LOAD_FORM_ID, &args![buffer, base + offset]);
    }
    let count = e.call(LOAD_VARIABLE_SIZED_VALUE, &args![buffer]).u32();
    let targets = Ptr::new(base + POTENTIAL_TARGETS);
    fn_009f6200(e, targets, count, 1);
    let mut index = 0u32;
    while index < count {
        let choice = fn_009f6190(e, targets, index);
        fn_009f38b0(e, choice, buffer);
        index = index.wrapping_add(1);
    }
}

/// `InitLoadGame`'s resolution of a loaded form id: null stays null, else
/// `__RTDynamicCast(004839c0(id), 0, TESForm, TESObjectREFR, 0)`.
fn resolve_reference(e: &mut Engine, id: u32) -> u32 {
    if id == 0 {
        return 0;
    }
    let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, CAST_FROM_FORM, CAST_TO_REFERENCE, 0u32],
    )
    .u32()
}

// Translated from 009f5ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::InitLoadGame` (engine map; `RET 4`): resolves
/// the location (+0x10), the current target (+0x3C, then marked targeted)
/// and the previous target (+0x44, likewise) from their form ids; runs each
/// choice's step `009a4f40` with the buffer; removes every target-less
/// choice whose procedure passes `0092a980` (`009f63a0(index, 1)`, the
/// count and index compared signed). Then, with a current target, its
/// marker is reserved again when it has a free marker (`00568260(0)`), else
/// the choice is marked dropped; a dropped choice or no current target
/// resets the procedure (-1), its duration (0.0), the target, the marker
/// (0x7F), asks for a new procedure and clears the scan time. With a
/// location and no change time yet, the change time becomes -1, then 0
/// when the location's extra data has type 0x51, then now plus the 0x6F
/// entry's seconds times 1000.0 (truncated to 64 bits) when it has one.
pub fn sand_box_actor_package_data_init_load_game(e: &mut Engine, this: Ptr, buffer: Ptr) {
    let base = this.addr();
    let id = e.mem.u32(base + LOCATION);
    let location = resolve_reference(e, id);
    e.mem.set_u32(base + LOCATION, location);
    let id = e.mem.u32(base + CURRENT_TARGET_OBJECT);
    let current = resolve_reference(e, id);
    e.mem.set_u32(base + CURRENT_TARGET_OBJECT, current);
    if e.mem.u32(base + CURRENT_TARGET_OBJECT) != 0 {
        let target = e.mem.u32(base + CURRENT_TARGET_OBJECT);
        e.call(SET_TARGETED, &args![target, 1u32]);
    }
    let id = e.mem.u32(base + PREV_TARGET_OBJECT);
    let previous = resolve_reference(e, id);
    e.mem.set_u32(base + PREV_TARGET_OBJECT, previous);
    if e.mem.u32(base + PREV_TARGET_OBJECT) != 0 {
        let target = e.mem.u32(base + PREV_TARGET_OBJECT);
        e.call(SET_TARGETED, &args![target, 1u32]);
    }
    let targets = Ptr::new(base + POTENTIAL_TARGETS);
    let count = e.call(WORD_AT_8, &args![targets]).u32();
    let mut index = 0u32;
    while index < count {
        let choice = fn_009f6190(e, targets, index);
        e.call(CHOICE_INIT_LOAD_GAME_009A4F40, &args![choice, buffer]);
        index = index.wrapping_add(1);
    }
    let mut count = e.call(WORD_AT_8, &args![targets]).u32() as i32;
    let mut index: i32 = 0;
    let mut dropped = false;
    while index < count {
        let choice = fn_009f6190(e, targets, index as u32);
        if e.call(CHOICE_TARGET, &args![choice]).u32() == 0 {
            let procedure = e.call(WORD_AT_4, &args![choice]).u32();
            if e.call(PROCEDURE_TEST_0092A980, &args![procedure]).u8() != 0 {
                fn_009f63a0(e, targets, index as u32, 1);
                count = count.wrapping_sub(1);
                dropped = true;
                continue;
            }
        }
        index = index.wrapping_add(1);
    }
    if e.mem.u32(base + CURRENT_TARGET_OBJECT) != 0 {
        let target = e.mem.u32(base + CURRENT_TARGET_OBJECT);
        if e.call(REFERENCE_HAS_FREE_MARKER, &args![target, 0u32]).u8() != 0 {
            let marker = u32::from(e.mem.u8(base + CURRENT_TARGET_MARKER));
            let target = e.mem.u32(base + CURRENT_TARGET_OBJECT);
            e.call(SET_MARKER_RESERVED, &args![target, marker, 1u32]);
        } else {
            dropped = true;
        }
    }
    if dropped || e.mem.u32(base + CURRENT_TARGET_OBJECT) == 0 {
        e.mem.set_u32(base + PROCEDURE, 0xffff_ffff);
        e.mem.set_u32(base + PROCEDURE_DURATION, x87_store32(0.0));
        e.mem.set_u32(base + CURRENT_TARGET_OBJECT, 0);
        e.mem.set_u8(base + CURRENT_TARGET_MARKER, 0x7f);
        e.mem.set_u8(base + SHOULD_PICK_NEW_PROCEDURE, 1);
        e.mem.set_u32(base + NEXT_TIME_TO_SCAN_WORLD, 0);
    }
    if e.mem.u32(base + LOCATION) == 0 || e.mem.u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS) != 0 {
        return;
    }
    e.mem
        .set_u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS, 0xffff_ffff);
    let location = e.mem.u32(base + LOCATION);
    let list = e.call(EXTRA_DATA_LIST, &args![location]).u32();
    if e.call(EXTRA_DATA_0X51, &args![list]).u32() == 0 {
        return;
    }
    e.mem.set_u32(base + NEXT_TIME_TO_CHANGE_LOCATIONS, 0);
    let location = e.mem.u32(base + LOCATION);
    let list = e.call(EXTRA_DATA_LIST, &args![location]).u32();
    let extra = e.call(EXTRA_DATA_0X6F, &args![list]).u32();
    if extra == 0 {
        return;
    }
    let start = now(e);
    let seconds = e.call(FLOAT_AT_0, &args![extra]).f64();
    let thousand: f64 = e.global(THOUSAND);
    let milliseconds = x87_fistp64_chop(X87Ext::load64(x87_mul(seconds, thousand))) as u32;
    e.mem.set_u32(
        base + NEXT_TIME_TO_CHANGE_LOCATIONS,
        start.wrapping_add(milliseconds),
    );
}

// Translated from 009f6200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SandBoxChoiceListItem,1024>`'s `SetSize` (unnamed in the
/// engine map; `RET 8`; items of 0x10 bytes). Size 0 empties the array
/// (`008454f0(shrink)`). Above the reserved size the buffer is allocated
/// (virtual slot +4 with the size) when there is none, else reallocated
/// (`009f6540(size, old size)`), the new items constructed (`009f6460`) and
/// the size set. Below the current size the extra items are destroyed
/// (`0072ba80`), the size set, and with `shrink` and a size at most a
/// quarter of the reserved one the buffer is reallocated to the size.
/// Otherwise the new items are constructed and the size set.
pub fn fn_009f6200(e: &mut Engine, this: Ptr, size: u32, shrink: u8) {
    let base = this.addr();
    if size == 0 {
        e.call(ARRAY_RELEASE_008454F0, &args![this, u32::from(shrink)]);
        return;
    }
    let item = |e: &Engine, index: u32| (index << 4).wrapping_add(e.mem.u32(base + ARRAY_BUFFER));
    if size > e.mem.u32(base + ARRAY_ALLOC_SIZE) {
        if e.mem.u32(base + ARRAY_ALLOC_SIZE) == 0 {
            let buffer = e.vcall(base, SLOT_ARRAY_ALLOCATE, &args![size]).u32();
            e.mem.set_u32(base + ARRAY_BUFFER, buffer);
            e.mem.set_u32(base + ARRAY_ALLOC_SIZE, size);
        } else {
            let old = e.mem.u32(base + ARRAY_SIZE);
            e.call(CHOICE_ARRAY_REALLOCATE_009F6540, &args![this, size, old]);
            e.mem.set_u32(base + ARRAY_ALLOC_SIZE, size);
        }
        let old = e.mem.u32(base + ARRAY_SIZE);
        let first = item(e, old);
        e.call(
            CHOICE_ARRAY_CONSTRUCT_ITEMS,
            &args![this, first, size.wrapping_sub(old)],
        );
        e.mem.set_u32(base + ARRAY_SIZE, size);
    } else if size < e.mem.u32(base + ARRAY_SIZE) {
        let old = e.mem.u32(base + ARRAY_SIZE);
        let first = item(e, size);
        e.call(
            CHOICE_ARRAY_DESTROY_ITEMS,
            &args![this, first, old.wrapping_sub(size)],
        );
        e.mem.set_u32(base + ARRAY_SIZE, size);
        if shrink != 0 && size <= e.mem.u32(base + ARRAY_ALLOC_SIZE) >> 2 {
            e.call(CHOICE_ARRAY_REALLOCATE_009F6540, &args![this, size, size]);
            e.mem.set_u32(base + ARRAY_ALLOC_SIZE, size);
        }
    } else {
        let old = e.mem.u32(base + ARRAY_SIZE);
        let first = item(e, old);
        e.call(
            CHOICE_ARRAY_CONSTRUCT_ITEMS,
            &args![this, first, size.wrapping_sub(old)],
        );
        e.mem.set_u32(base + ARRAY_SIZE, size);
    }
}

// Translated from 009f6430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<SandBoxChoiceListItem,1024>` scalar deleting destructor
/// (engine map, from the vtable; `RET 4`): the destructor body `009f61e0`,
/// then `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_sand_box_choice_list_item_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_009f61e0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_38b0, fn_009f38b0(Ptr, Ptr)),
        entry!(0x009f_41c0, fn_009f41c0(Ptr)),
        entry!(
            0x009f_4680,
            sand_box_actor_package_data_forget_target_and_pick_procedure(Ptr, Ptr, Ptr)
        ),
        entry!(0x009f_5ca0, sand_box_actor_package_data_load_game(Ptr, Ptr)),
        entry!(
            0x009f_5ea0,
            sand_box_actor_package_data_init_load_game(Ptr, Ptr)
        ),
        entry!(0x009f_6200, fn_009f6200(Ptr, u32, u8)),
        entry!(
            0x009f_6430,
            bs_simple_array_sand_box_choice_list_item_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x009f_6190, fn_009f6190(Ptr, u32) -> Ptr),
        entry!(0x009f_61e0, fn_009f61e0(Ptr)),
        entry!(0x009f61b0, fn_009f61b0(Ptr) -> Ptr),
        entry!(0x009f65f0, fn_009f65f0(Ptr, u32, u32)),
        entry!(0x009f6340, fn_009f6340(Ptr, Ptr) -> u32),
        entry!(0x009f63a0, fn_009f63a0(Ptr, u32, u32)),
        entry!(0x009f45e0, fn_009f45e0(Ptr) -> bool),
        entry!(0x009f4600, fn_009f4600(Ptr) -> bool),
        entry!(0x009f4620, fn_009f4620(Ptr) -> bool),
        entry!(0x009f54b0, fn_009f54b0(Ptr) -> bool),
        entry!(0x009f5690, fn_009f5690(Ptr) -> bool),
        entry!(0x009f4640, fn_009f4640(Ptr) -> u32),
        entry!(0x009f3800, fn_009f3800(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x009f3850, fn_009f3850(Ptr, Ptr)),
        entry!(0x009f4060, fn_009f4060(Ptr)),
        entry!(0x009f3a70, fn_009f3a70(Ptr)),
        entry!(
            0x009f3a40,
            sand_box_actor_package_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x009f3e20, fn_009f3e20(Ptr) -> bool),
        entry!(0x009f3af0, fn_009f3af0(Ptr, Ptr, Ptr)),
        entry!(
            0x009f3c10,
            sand_box_actor_package_data_move_to_next_location(Ptr) -> bool
        ),
        entry!(0x009f3c90, fn_009f3c90(Ptr, Ptr) -> bool),
        entry!(0x009f3d50, fn_009f3d50(Ptr) -> bool),
        entry!(
            0x009f40c0,
            sand_box_actor_package_data_handle_deleted_reference(Ptr, Ptr)
        ),
        entry!(
            0x009f41f0,
            sand_box_actor_package_data_get_time_difference(Ptr, f32, f32) -> f32
        ),
        entry!(0x009f4300, fn_009f4300(Ptr, Ptr, bool) -> bool),
        entry!(
            0x009f43c0,
            sand_box_actor_package_data_should_pick_new_procedure(Ptr, Ptr) -> bool
        ),
        entry!(0x009f3e80, fn_009f3e80(Ptr)),
        entry!(0x009f54d0, sand_box_actor_package_data_scan_world(Ptr, Ptr)),
        entry!(0x009f5070, fn_009f5070(Ptr, Ptr) -> bool),
        entry!(0x009f56b0, fn_009f56b0(Ptr, Ptr, f32) -> bool),
        entry!(0x009f5a60, sand_box_actor_package_data_save_game(Ptr, Ptr)),
        entry!(0x009f4700, fn_009f4700(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_009f6190_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(ELEMENT_TARGET_00558DD0, |_, a| {
            (0x4000_0000 | a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x009f_6190, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == ELEMENT_TARGET_00558DD0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
        assert_eq!(result.u32(), 0x4000_2000);
    }
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_009f61e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009f_61e0, ARRAY_VTABLE_01093614, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod sandbox_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A double answering `value`.
    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    /// A double answering the float `value` in `ST0`.
    fn answer_float(e: &mut Engine, addr: u32, value: f32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    /// A double answering the word at `offset` of its first argument.
    fn word_of(e: &mut Engine, addr: u32, offset: u32) {
        e.register_double(addr, move |e, a| e.mem.u32(a[0] + offset).into_ret());
    }

    /// `_ftol2` truncating its `f64` argument.
    fn ftol(e: &mut Engine) {
        e.register(FTOL, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (v as i32).into_ret()
        });
    }

    fn constant(e: &mut Engine, addr: u32, value: f64) {
        e.map(addr, 8);
        e.set_global(addr, value);
    }

    fn constant32(e: &mut Engine, addr: u32, value: f32) {
        e.map(addr, 4);
        e.set_global(addr, value);
    }

    fn constants(e: &mut Engine) {
        constant(e, ZERO, 0.0);
        constant(e, HOURS_PER_DAY, 24.0);
        constant(e, SIXTY, 60.0);
        constant(e, THOUSAND, 1000.0);
        constant(e, TWENTY_DOUBLE, 20.0);
        constant32(e, FLOAT_MAX, f32::MAX);
        constant32(e, DEFAULT_TOTAL_TIME, 24.0);
        constant32(e, MINUS_ONE, -1.0);
        constant32(e, TWENTY, 20.0);
        constant32(e, NINETY, 90.0);
    }

    fn data(e: &mut Engine) -> u32 {
        e.mem.alloc(0x64)
    }

    /// An object whose vtable has `slots` (byte offset, target).
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let size = slots.iter().map(|(s, _)| s + 4).max().unwrap_or(4);
        let vtable = e.mem.alloc(size);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(object, vtable);
        object
    }

    fn calendar(e: &mut Engine, day: u32, hour: f32) {
        answer(e, CALENDAR_DAYS, day);
        answer_float(e, CALENDAR_GET_HOUR, hour);
    }

    #[test]
    fn choice_array_construct_and_initialize() {
        let mut e = Engine::new();
        e.register(CHOICE_ARRAY_CONSTRUCT_ITEMS, nothing);
        let array = object(&mut e, &[(4, 0x0600_0004)]);
        e.register(0x0600_0004, |e, a| e.mem.alloc(a[1] * 16).into_ret());
        let vtable = e.mem.u32(array);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_61b0, &args![array]).u32(), array);
        assert_eq!(e.mem.u32(array), CHOICE_ARRAY_VTABLE);
        assert_eq!(e.mem.bytes(array + 4, 12), vec![0; 12]);
        assert!(calls_to(&e, 0x0600_0004).is_empty());
        e.mem.set_u32(array, vtable);
        e.call(0x009f_65f0, &args![array, 2u32, 3u32]);
        let buffer = e.mem.u32(array + ARRAY_BUFFER);
        assert_eq!(calls_to(&e, 0x0600_0004), vec![vec![array, 3]]);
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_CONSTRUCT_ITEMS),
            vec![vec![array, buffer, 3]]
        );
        assert_eq!(e.mem.u32(array + ARRAY_SIZE), 3);
        assert_eq!(e.mem.u32(array + ARRAY_ALLOC_SIZE), 3);
        e.call(0x009f_65f0, &args![array, 5u32, 0u32]);
        assert_eq!(e.mem.u32(array + ARRAY_SIZE), 0);
        assert_eq!(e.mem.u32(array + ARRAY_ALLOC_SIZE), 5);
        assert_eq!(calls_to(&e, CHOICE_ARRAY_CONSTRUCT_ITEMS).len(), 1);
    }

    #[test]
    fn choice_array_add_and_remove() {
        let mut e = Engine::new();
        e.register(CHOICE_ARRAY_CONSTRUCT_ITEMS, nothing);
        e.register(CHOICE_ARRAY_DESTROY_ITEMS, nothing);
        e.register(CHOICE_ARRAY_MOVE_ITEMS, nothing);
        answer(&mut e, CHOICE_ARRAY_GROW, 2);
        answer(&mut e, MOVE_COUNT_0042F5A0, 1);
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(array + ARRAY_BUFFER, buffer);
        e.mem.set_u32(array + ARRAY_SIZE, 3);
        let item = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_u32(item + 4 * word, 0x100 + word);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_6340, &args![array, item]).u32(), 2);
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_CONSTRUCT_ITEMS),
            vec![vec![array, buffer + 0x20, 1]]
        );
        assert_eq!(e.mem.u32(buffer + 0x2c), 0x103);
        e.call(0x009f_63a0, &args![array, 0u32, 1u32]);
        assert_eq!(calls_to(&e, MOVE_COUNT_0042F5A0), vec![vec![2, 1]]);
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_DESTROY_ITEMS),
            vec![vec![array, buffer, 1]]
        );
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_MOVE_ITEMS),
            vec![vec![array, buffer, buffer + 0x20, 1]]
        );
        assert_eq!(e.mem.u32(array + ARRAY_SIZE), 2);
    }

    #[test]
    fn package_word_24_bit_tests() {
        let mut e = Engine::new();
        let package = e.mem.alloc(0x28);
        let tests = [
            (0x009f_45e0u32, 0x1u16),
            (0x009f_4600, 0x2),
            (0x009f_4620, 0x10),
            (0x009f_54b0, 0x8),
            (0x009f_5690, 0x20),
        ];
        for (addr, mask) in tests {
            e.mem.set_u16(package + 0x24, 0);
            assert!(e.call(addr, &args![package]).bool());
            e.mem.set_u16(package + 0x24, mask);
            assert!(!e.call(addr, &args![package]).bool());
            e.mem.set_u16(package + 0x24, !mask);
            assert!(e.call(addr, &args![package]).bool());
        }
    }

    #[test]
    fn procedure_type_of_the_procedure() {
        let mut e = Engine::new();
        word_of(&mut e, WORD_AT_8, 8);
        e.map(PROCEDURE_TYPES, 24);
        e.mem.set_u32(PROCEDURE_TYPES + 8, 5);
        let this = data(&mut e);
        e.mem.set_u32(this + PROCEDURE, 2);
        assert_eq!(e.call(0x009f_4640, &args![this]).u32(), 5);
        e.mem.set_u32(this + PROCEDURE, 0xffff_ffff);
        assert_eq!(e.call(0x009f_4640, &args![this]).u32(), 0xffff_ffff);
    }

    #[test]
    fn choice_item_construct_and_save() {
        let mut e = Engine::new();
        e.register(SET_TARGETED, nothing);
        e.register(SAVE_DATA, nothing);
        e.register(SAVE_FORM_ID, nothing);
        let item = e.mem.alloc(0x10);
        e.mem.set_u32(item + 0xc, 9);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x009f_3800, &args![item, 0x500u32, 3u32, 7u32])
                .u32(),
            item
        );
        assert_eq!(e.mem.u32(item), 0x500);
        assert_eq!(e.mem.u32(item + 4), 3);
        assert_eq!(e.mem.u32(item + 8), 7);
        assert_eq!(e.mem.u32(item + 0xc), 0);
        assert_eq!(calls_to(&e, SET_TARGETED), vec![vec![0x500, 1]]);
        e.call(0x009f_3800, &args![item, 0u32, 3u32, 7u32]);
        assert_eq!(calls_to(&e, SET_TARGETED).len(), 1);
        e.mem.set_u32(item, 0x600);
        e.call(0x009f_3850, &args![item, 0x77u32]);
        assert_eq!(
            calls_to(&e, SAVE_DATA),
            vec![
                vec![0x77, item + 4, 4, 0],
                vec![0x77, item + 8, 4, 0],
                vec![0x77, item + 0xc, 4, 0]
            ]
        );
        assert_eq!(calls_to(&e, SAVE_FORM_ID), vec![vec![0x77, 0x600, 0]]);
    }

    #[test]
    fn release_marker_and_destructors() {
        let mut e = Engine::new();
        e.register(SET_MARKER_RESERVED, nothing);
        e.register(ARRAY_RELEASE_008454F0, nothing);
        e.register(ACTOR_PACKAGE_DATA_DESTRUCT, nothing);
        e.register(OPERATOR_DELETE, nothing);
        e.map(DATA_HANDLER, 4);
        let this = data(&mut e);
        e.mem.set_u32(this + CURRENT_TARGET_OBJECT, 0x900);
        e.mem.set_u8(this + CURRENT_TARGET_MARKER, 3);
        e.call_log = Some(vec![]);
        e.call(0x009f_4060, &args![this]);
        assert_eq!(calls_to(&e, SET_MARKER_RESERVED), vec![vec![0x900, 3, 0]]);
        assert_eq!(e.mem.u32(this + CURRENT_TARGET_OBJECT), 0);
        assert_eq!(e.mem.u8(this + CURRENT_TARGET_MARKER), 0x7f);
        // The data handler's test keeps the marker reserved.
        e.set_global(DATA_HANDLER, 0x1234u32);
        answer(&mut e, DATA_HANDLER_TEST_004226E0, 1);
        e.mem.set_u32(this + CURRENT_TARGET_OBJECT, 0x900);
        e.call(0x009f_4060, &args![this]);
        assert_eq!(calls_to(&e, SET_MARKER_RESERVED).len(), 1);
        assert_eq!(calls_to(&e, DATA_HANDLER_TEST_004226E0), vec![vec![0x1234]]);
        // Destructor and deleting destructor.
        assert_eq!(e.call(0x009f_3a40, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), SANDBOX_ACTOR_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + POTENTIAL_TARGETS), ARRAY_VTABLE_01093614);
        assert_eq!(
            calls_to(&e, ARRAY_RELEASE_008454F0),
            vec![vec![this + 0x20, 1], vec![this + 0x20, 1]]
        );
        assert_eq!(calls_to(&e, ACTOR_PACKAGE_DATA_DESTRUCT), vec![vec![this]]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x009f_3a40, &args![this, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn activity_need_by_activity() {
        let mut e = Engine::new();
        let this = data(&mut e);
        e.mem.set_u8(this + BED, 1);
        let cases = [
            (0xffff_ffffu32, false),
            (0, true),
            (1, false),
            (3, false),
            (4, true),
            (0x40, true),
        ];
        for (activity, expected) in cases {
            e.mem.set_u32(this + ACTIVITY, activity);
            assert_eq!(e.call(0x009f_3e20, &args![this]).bool(), expected);
        }
        e.mem.set_u8(this + FOOD, 1);
        e.mem.set_u32(this + ACTIVITY, 2);
        assert!(e.call(0x009f_3e20, &args![this]).bool());
    }

    #[test]
    fn location_from_the_package_location() {
        let mut e = Engine::new();
        constants(&mut e);
        e.register(PACKAGE_LOCATION_GET_LOC_TYPE, |e, a| {
            u32::from(e.mem.u8(a[0])).into_ret()
        });
        answer(&mut e, PACKAGE_LOCATION_GET_LOC_REFERENCE, 0x1000);
        answer(&mut e, REFERENCE_VALUE_00569B80, 0x2000);
        let holder = object(&mut e, &[(0x514, 0x0600_0514)]);
        answer(&mut e, 0x0600_0514, 0x3000);
        let actor = e.mem.alloc(0x70);
        e.mem.set_u32(actor + 0x68, holder);
        word_of(&mut e, OBJECT_AT_0X68, 0x68);
        answer(&mut e, EXTRA_DATA_LIST, 0x4000);
        answer(&mut e, EXTRA_DATA_0X51, 0);
        let this = data(&mut e);
        let location = e.mem.alloc(4);
        for (kind, expected) in [
            (0u8, 0x1000u32),
            (4, 0x3000),
            (5, 0x3000),
            (6, 0x2000),
            (2, 0),
            (7, 0),
        ] {
            e.mem.set_u8(location, kind);
            e.call(0x009f_3af0, &args![this, actor, location]);
            assert_eq!(e.mem.u32(this + LOCATION), expected);
        }
        // Types 4 and 5 without the object leave the location.
        e.mem.set_u32(this + LOCATION, 0x77);
        e.mem.set_u32(actor + 0x68, 0);
        e.mem.set_u8(location, 4);
        e.call(0x009f_3af0, &args![this, actor, location]);
        assert_eq!(e.mem.u32(this + LOCATION), 0x77);
        // Extra data 0x51 and 0x6F: the change time.
        answer(&mut e, EXTRA_DATA_0X51, 0x51);
        answer(&mut e, EXTRA_DATA_0X6F, 0x6f);
        answer(&mut e, TIMER_NOW, 10_000);
        answer_float(&mut e, FLOAT_AT_0, 1.5);
        e.mem.set_u8(location, 6);
        e.call(0x009f_3af0, &args![this, actor, location]);
        assert_eq!(e.mem.u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS), 11_500);
        answer(&mut e, EXTRA_DATA_0X6F, 0);
        e.call(0x009f_3af0, &args![this, actor, location]);
        assert_eq!(e.mem.u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS), 0);
    }

    #[test]
    fn move_to_next_location_after_the_change_time() {
        let mut e = Engine::new();
        answer(&mut e, TIMER_NOW, 500);
        answer(&mut e, EXTRA_DATA_LIST, 0x4000);
        answer(&mut e, EXTRA_DATA_0X51, 0x2222);
        let this = data(&mut e);
        assert!(!e.call(0x009f_3c10, &args![this]).bool());
        e.mem.set_u32(this + LOCATION, 0x1111);
        e.mem.set_u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS, 500);
        assert!(!e.call(0x009f_3c10, &args![this]).bool());
        e.mem.set_u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS, 499);
        assert!(e.call(0x009f_3c10, &args![this]).bool());
        assert_eq!(e.mem.u32(this + LOCATION), 0x2222);
        assert_eq!(e.mem.u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS), 0xffff_ffff);
        assert!(!e.call(0x009f_3c10, &args![this]).bool());
        // The next location is the location itself.
        e.mem.set_u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS, 1);
        assert!(!e.call(0x009f_3c10, &args![this]).bool());
    }

    /// An activity from day `day` at `start` for `duration` hours.
    fn activity(e: &mut Engine, this: u32, day: u32, start: f32, duration: f32) {
        e.mem.set_u32(this + ACTIVITY, 1);
        e.mem.set_u32(this + ACTIVITY_START_DAY, day);
        e.mem.set_f32(this + ACTIVITY_START_HOUR, start);
        e.mem.set_f32(this + ACTIVITY_DURATION, duration);
    }

    #[test]
    fn activity_over_and_inside() {
        let mut e = Engine::new();
        constants(&mut e);
        let this = data(&mut e);
        let ended = e.mem.alloc(4);
        e.mem.set_u32(this + ACTIVITY, 0xffff_ffff);
        calendar(&mut e, 3, 10.0);
        assert!(e.call(0x009f_3c90, &args![this, ended]).bool());
        assert!(e.call(0x009f_3d50, &args![this]).bool());
        // 22:00 for 4 hours: ends on day 4 at 2:00.
        activity(&mut e, this, 3, 22.0, 4.0);
        let cases = [
            (3u32, 23.0f32, false, None, true),
            (4, 1.0, false, None, true),
            (4, 2.0, true, Some(1u8), false),
            (5, 0.0, true, Some(0), false),
            (2, 23.0, false, None, false),
            (3, 21.0, false, None, false),
        ];
        for (day, hour, over, flag, inside) in cases {
            calendar(&mut e, day, hour);
            e.mem.set_u8(ended, 9);
            assert_eq!(e.call(0x009f_3c90, &args![this, ended]).bool(), over);
            assert_eq!(e.mem.u8(ended), flag.unwrap_or(9));
            assert_eq!(e.call(0x009f_3d50, &args![this]).bool(), inside);
        }
        calendar(&mut e, 4, 3.0);
        assert!(e.call(0x009f_3c90, &args![this, 0u32]).bool());
    }

    #[test]
    fn handle_deleted_reference_forgets_it() {
        let mut e = Engine::new();
        word_of(&mut e, WORD_AT_8, 8);
        word_of(&mut e, CHOICE_TARGET, 0);
        e.register(ELEMENT_TARGET_00558DD0, |e, a| {
            (e.mem.u32(a[0] + 4) + (a[1] << 4)).into_ret()
        });
        answer(&mut e, MOVE_COUNT_0042F5A0, 0);
        e.register(CHOICE_ARRAY_DESTROY_ITEMS, nothing);
        e.register_double(CHOICE_ARRAY_MOVE_ITEMS, |_, _| Ret::default());
        let this = data(&mut e);
        let buffer = e.mem.alloc(0x30);
        e.mem.set_u32(this + POTENTIAL_TARGETS + 4, buffer);
        e.mem.set_u32(this + POTENTIAL_TARGETS + 8, 2);
        e.mem.set_u32(buffer, 0x700);
        e.mem.set_u32(buffer + 0x10, 0x800);
        e.mem.set_u32(this + LOCATION, 0x800);
        e.mem.set_u32(this + PREV_TARGET_OBJECT, 0x800);
        e.mem.set_u32(this + PROCEDURE, 2);
        e.call_log = Some(vec![]);
        e.call(0x009f_40c0, &args![this, 0x800u32]);
        assert_eq!(e.mem.u32(this + LOCATION), 0);
        assert_eq!(e.mem.u32(this + PREV_TARGET_OBJECT), 0);
        assert_eq!(e.mem.u32(this + POTENTIAL_TARGETS + 8), 1);
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_DESTROY_ITEMS),
            vec![vec![this + 0x20, buffer + 0x10, 1]]
        );
        assert_eq!(e.mem.u32(this + PROCEDURE), 0xffff_ffff);
        assert_eq!(e.mem.u8(this + SHOULD_PICK_NEW_PROCEDURE), 1);
        assert_eq!(e.mem.u8(this + CURRENT_TARGET_MARKER), 0x7f);
        // A reference nothing holds changes nothing; null returns at once.
        e.mem.set_u32(this + PROCEDURE, 3);
        e.call(0x009f_40c0, &args![this, 0x999u32]);
        assert_eq!(e.mem.u32(this + PROCEDURE), 3);
        e.call(0x009f_40c0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + PROCEDURE), 3);
    }

    #[test]
    fn time_difference_in_hours_and_minutes() {
        let mut e = Engine::new();
        constants(&mut e);
        ftol(&mut e);
        e.register(ABS, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        let r = e.call(0x009f_41f0, &args![0u32, 8.5f32, 9.75f32]).f32();
        // 1 hour, then 105 - 30 minutes (both against 8:00).
        assert_eq!(r, 76.0);
        let r = e.call(0x009f_41f0, &args![0u32, 9.75f32, 8.5f32]).f32();
        // |8 - 9| = 1, minutes 45 vs (8.5 - 9) * 60 = -30: 1 - 75.
        assert_eq!(r, -74.0);
    }

    #[test]
    fn free_to_leave_the_procedure() {
        let mut e = Engine::new();
        let this = data(&mut e);
        e.mem.set_u32(this + PROCEDURE, 2);
        assert!(e.call(0x009f_4300, &args![this, 0u32, 1u32]).bool());
        e.mem.set_u32(this + PROCEDURE, 0);
        answer(&mut e, ACTOR_GET_ANIM_ACTION, 12);
        let process = object(&mut e, &[(0x394, 0x0600_0394)]);
        e.register(0x0600_0394, nothing);
        let actor = object(&mut e, &[(0x1e4, 0x0600_01e4)]);
        e.mem.set_u32(actor + 0x68, process);
        word_of(&mut e, OBJECT_AT_0X68, 0x68);
        answer(&mut e, 0x0600_01e4, 0x5000);
        assert!(!e.call(0x009f_4300, &args![this, actor, 1u32]).bool());
        answer(&mut e, ACTOR_GET_ANIM_ACTION, 3);
        answer(&mut e, SPECIAL_IDLE_DONE_PLAYING, 0);
        assert!(!e.call(0x009f_4300, &args![this, actor, 1u32]).bool());
        answer(&mut e, SPECIAL_IDLE_DONE_PLAYING, 1);
        answer(&mut e, ANIMATION_VALUE_00498CF0, 0x6000);
        answer(&mut e, VALUE_008041A0, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x009f_4300, &args![this, actor, 0u32]).bool());
        assert!(calls_to(&e, 0x0600_0394).is_empty());
        assert!(e.call(0x009f_4300, &args![this, actor, 1u32]).bool());
        assert_eq!(calls_to(&e, 0x0600_0394), vec![vec![process, actor]]);
        answer(&mut e, VALUE_008041A0, 3);
        assert!(e.call(0x009f_4300, &args![this, actor, 1u32]).bool());
        assert_eq!(calls_to(&e, 0x0600_0394).len(), 1);
        answer(&mut e, 0x0600_01e4, 0);
        assert!(e.call(0x009f_4300, &args![this, actor, 1u32]).bool());
    }

    #[test]
    fn should_pick_new_procedure_branches() {
        let mut e = Engine::new();
        constants(&mut e);
        ftol(&mut e);
        e.register(ABS, |_, a| (a[0] as i32).wrapping_abs().into_ret());
        answer(&mut e, TIMER_NOW, 1000);
        let package = e.mem.alloc(0x28);
        e.mem.set_u16(package + 0x24, 0x2);
        answer(&mut e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, package);
        let this = data(&mut e);
        // The package no longer allows a bed.
        e.mem.set_u8(this + BED, 1);
        assert!(e.call(0x009f_43c0, &args![this, 0u32]).bool());
        assert_eq!(e.mem.u8(this + BED), 0);
        assert_eq!(e.mem.u32(this + NEXT_TIME_TO_SCAN_WORLD), 1000);
        // The location's change time has passed.
        e.mem.set_u32(this + LOCATION, 0x10);
        e.mem.set_u32(this + PROCEDURE, 2);
        e.mem.set_u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS, 999);
        assert!(e.call(0x009f_43c0, &args![this, 0u32]).bool());
        e.mem
            .set_u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS, 0xffff_ffff);
        // A new procedure is wanted.
        e.mem.set_u8(this + SHOULD_PICK_NEW_PROCEDURE, 1);
        assert!(e.call(0x009f_43c0, &args![this, 0u32]).bool());
        e.mem.set_u8(this + SHOULD_PICK_NEW_PROCEDURE, 0);
        // Duration not above 0.
        assert!(e.call(0x009f_43c0, &args![this, 0u32]).bool());
        e.mem.set_f32(this + PROCEDURE_DURATION, 2.0);
        // Total time 0.
        assert!(e.call(0x009f_43c0, &args![this, 0u32]).bool());
        e.mem.set_f32(this + TOTAL_TIME, 24.0);
        // The marker is used.
        e.mem.set_u32(this + CURRENT_TARGET_OBJECT, 0x20);
        answer(&mut e, GET_MARKER_USED, 1);
        assert!(e.call(0x009f_43c0, &args![this, 0u32]).bool());
        e.mem.set_u32(this + STATE, 3);
        // The activity: none means over.
        e.mem.set_u32(this + ACTIVITY, 0xffff_ffff);
        calendar(&mut e, 1, 9.0);
        assert!(e.call(0x009f_43c0, &args![this, 0u32]).bool());
        // Inside a met activity: whether the procedure differs.
        e.map(PROCEDURES_FOR_ACTIVITIES, 16);
        e.mem.set_u32(PROCEDURES_FOR_ACTIVITIES + 4, 2);
        activity(&mut e, this, 1, 8.0, 4.0);
        e.mem.set_u8(this + FOOD, 1);
        assert!(!e.call(0x009f_43c0, &args![this, 0u32]).bool());
        e.mem.set_u32(this + PROCEDURE, 3);
        assert!(e.call(0x009f_43c0, &args![this, 0u32]).bool());
        // Outside the need: the duration against the time passed.
        e.mem.set_u8(this + FOOD, 0);
        e.mem.set_f32(this + LAST_TIME_CHANGED, 8.5);
        assert!(e.call(0x009f_43c0, &args![this, 0u32]).bool());
        e.mem.set_f32(this + PROCEDURE_DURATION, 40.0);
        assert!(!e.call(0x009f_43c0, &args![this, 0u32]).bool());
    }

    #[test]
    fn next_activity_takes_the_least_wait() {
        let mut e = Engine::new();
        constants(&mut e);
        // Activities: 0 at 22 for 8, 1 at 7 for 1, 2 at 12 for 1, 3 at 18
        // for 1.
        e.register(ACTIVITY_TIMES, |e, a| {
            let (start, duration) =
                [(22.0f32, 8.0f32), (7.0, 1.0), (12.0, 1.0), (18.0, 1.0)][a[0] as usize];
            e.mem.set_f32(a[1], start);
            e.mem.set_f32(a[2], duration);
            Ret::default()
        });
        let this = data(&mut e);
        e.mem.set_u32(this + ACTIVITY, 0xffff_ffff);
        calendar(&mut e, 5, 10.0);
        e.call(0x009f_3e80, &args![this]);
        assert_eq!(e.mem.u32(this + ACTIVITY), 2);
        assert_eq!(e.mem.f32(this + ACTIVITY_START_HOUR), 12.0);
        assert_eq!(e.mem.u32(this + ACTIVITY_START_DAY), 5);
        assert_eq!(e.mem.f32(this + ACTIVITY_DURATION), 1.0);
        // At 19:30 the breakfast tomorrow waits longer than the night at 22.
        calendar(&mut e, 5, 19.5);
        e.mem.set_u32(this + ACTIVITY, 0xffff_ffff);
        e.call(0x009f_3e80, &args![this]);
        assert_eq!(e.mem.u32(this + ACTIVITY), 0);
        assert_eq!(e.mem.u32(this + ACTIVITY_START_DAY), 5);
        // At 23:00 the night started already: taken at once.
        calendar(&mut e, 5, 23.0);
        e.mem.set_u32(this + ACTIVITY, 0xffff_ffff);
        e.call(0x009f_3e80, &args![this]);
        assert_eq!(e.mem.u32(this + ACTIVITY), 0);
        // An activity not over: nothing changes.
        activity(&mut e, this, 5, 22.0, 8.0);
        e.mem.set_u32(this + ACTIVITY, 3);
        e.call(0x009f_3e80, &args![this]);
        assert_eq!(e.mem.u32(this + ACTIVITY), 3);
    }

    fn scan_doubles(e: &mut Engine) -> (u32, u32) {
        e.map(SETTING_RADIUS, 8);
        e.register(SETTING_FLOAT, |_, a| (a[0] + 4).into_ret());
        e.mem.set_f32(SETTING_RADIUS + 4, 500.0);
        e.map(DATA_HANDLER, 4);
        e.set_global(DATA_HANDLER, 0x0700_0000u32);
        word_of(e, REFERENCE_CELL, 0x40);
        e.register(ARRAY_RELEASE_008454F0, nothing);
        e.register(FUNCTOR_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(ENUM_REFERENCES_CLOSE_TO_POINT, nothing);
        e.register(SET_TARGETED, nothing);
        e.register(CHOICE_ARRAY_CONSTRUCT_ITEMS, nothing);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        let actor = object(e, &[(0x1f4, 0x0600_01f4)]);
        e.register_double(0x0600_01f4, move |_, _| position.into_ret());
        e.mem.set_u32(actor + 0x40, 0x0800_0000);
        let package = e.mem.alloc(0x28);
        (actor, package)
    }

    #[test]
    fn scan_world_enumerates_and_adds_food_and_wander() {
        let mut e = Engine::new();
        let (actor, package) = scan_doubles(&mut e);
        answer(&mut e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, package);
        e.register(PACKAGE_GET_LOCATION_COORD, |e, a| {
            e.mem.set_f32(a[1], 7.0);
            a[1].into_ret()
        });
        answer(&mut e, PACKAGE_LOCATION_RADIUS, 300);
        answer(&mut e, ACTOR_GET_BEST_FOOD_ITEM, 0);
        // Grows into a buffer of 4.
        let this = data(&mut e);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(this + POTENTIAL_TARGETS + 4, buffer);
        e.register_double(CHOICE_ARRAY_GROW, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        // The centre and the functor as the enumeration sees them (the frame they lie in ends
        // with the scan).
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let record = std::rc::Rc::clone(&seen);
        e.register_double(ENUM_REFERENCES_CLOSE_TO_POINT, move |e, a| {
            record.borrow_mut().push([
                e.mem.f32(a[2]).to_bits(),
                e.mem.u32(a[7]),
                e.mem.u32(a[7] + 4),
            ]);
            Ret::default()
        });
        e.mem.set_u8(this + BED, 1);
        e.call_log = Some(vec![]);
        e.call(0x009f_54d0, &args![this, actor]);
        assert_eq!(e.mem.u8(this + BED), 0);
        let enumerate = calls_to(&e, ENUM_REFERENCES_CLOSE_TO_POINT);
        assert_eq!(enumerate.len(), 1);
        let w = &enumerate[0];
        assert_eq!(w[0], 0x0700_0000);
        assert_eq!(w[1], 0x0800_0000);
        assert_eq!(w[3], 300.0f32.to_bits());
        assert_eq!(w[4], w[2]);
        assert_eq!(w[5], 300.0f32.to_bits());
        assert_eq!(w[6], SCAN_CALLBACK);
        // Centre x 7.0, the functor of the actor and `this`.
        assert_eq!(*seen.borrow(), vec![[7.0f32.to_bits(), actor, this]]);
        // Food and wandering allowed, no food item: one wander choice.
        assert_eq!(e.mem.u32(this + POTENTIAL_TARGETS + 8), 1);
        assert_eq!(e.mem.u32(buffer + 4), 3);
        // A food item with a count: an eat choice and the food flag.
        e.mem.set_u16(package + 0x24, 0x20);
        let food = e.mem.alloc(0xc);
        e.mem.set_u32(food + 8, 2);
        answer(&mut e, ACTOR_GET_BEST_FOOD_ITEM, food);
        word_of(&mut e, WORD_AT_8, 8);
        e.call(0x009f_54d0, &args![this, actor]);
        assert_eq!(e.mem.u8(this + FOOD), 1);
        assert_eq!(e.mem.u32(buffer + 0x14), 2);
    }

    /// The scan's centre is copied from the package's point in the frame's local at stack offset -0x38 while it
    /// is still there (`00675de0` answers its address). With the freed-block check a point freed
    /// before the copy would give the fill word as the centre.
    #[test]
    fn scan_world_copies_the_centre_from_a_live_point() {
        let mut e = Engine::new();
        e.mem.enable_freed_check(true);
        let (actor, package) = scan_doubles(&mut e);
        answer(&mut e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, package);
        e.register(PACKAGE_GET_LOCATION_COORD, |e, a| {
            e.mem.set_f32(a[1], 7.0);
            e.mem.set_f32(a[1] + 4, -3.0);
            e.mem.set_f32(a[1] + 8, 0.5);
            a[1].into_ret()
        });
        answer(&mut e, PACKAGE_LOCATION_RADIUS, 300);
        answer(&mut e, ACTOR_GET_BEST_FOOD_ITEM, 0);
        let this = data(&mut e);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(this + POTENTIAL_TARGETS + 4, buffer);
        e.register_double(CHOICE_ARRAY_GROW, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        // The centre as the enumeration sees it (the frame it lies in ends with the scan).
        let centres = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen = std::rc::Rc::clone(&centres);
        e.register_double(ENUM_REFERENCES_CLOSE_TO_POINT, move |e, a| {
            seen.borrow_mut()
                .push([e.mem.f32(a[2]), e.mem.f32(a[2] + 4), e.mem.f32(a[2] + 8)]);
            Ret::default()
        });
        e.call(0x009f_54d0, &args![this, actor]);
        assert_eq!(*centres.borrow(), vec![[7.0, -3.0, 0.5]]);
    }

    /// Doubles for the scan callback: a functor of `actor` and `data`, and a
    /// reference of base form type `kind`.
    fn callback_setup(e: &mut Engine, kind: u8) -> (u32, u32, u32, u32) {
        word_of(e, WORD_AT_4, 4);
        word_of(e, CHOICE_TARGET, 0);
        word_of(e, REFERENCE_BASE_FORM, 0x20);
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        answer(e, REFERENCE_FLAG_0X20, 0);
        answer(e, REFERENCE_FLAG_0X4000, 0);
        answer(e, REFERENCE_FLAG_0X800, 0);
        answer(e, REFERENCE_TEST_0057B500, 0);
        answer(e, REFERENCE_GET_OWNER, 0);
        e.register(SET_TARGETED, nothing);
        e.register(CHOICE_ARRAY_CONSTRUCT_ITEMS, nothing);
        e.register_double(CHOICE_ARRAY_GROW, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        let package = e.mem.alloc(0x28);
        answer(e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, package);
        let actor = object(e, &[(0x1a0, 0x0600_01a0)]);
        answer(e, 0x0600_01a0, 0);
        let base_form = e.mem.alloc(0x40);
        e.mem.set_u32(actor + 0x20, base_form);
        let data = e.mem.alloc(0x64);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(data + POTENTIAL_TARGETS + 4, buffer);
        let functor = e.mem.alloc(8);
        e.mem.set_u32(functor, actor);
        e.mem.set_u32(functor + 4, data);
        let reference = object(e, &[(0x100, 0x0600_0100), (0x1d0, 0x0600_01d0)]);
        let form = e.mem.alloc(0x40);
        e.mem.set_u8(form + 4, kind);
        e.mem.set_u32(reference + 0x20, form);
        (functor, data, reference, package)
    }

    #[test]
    fn scan_callback_adds_food_dialogue_idle_and_furniture() {
        // Food (0x2F).
        let mut e = Engine::new();
        let (functor, data, reference, _) = callback_setup(&mut e, 0x2f);
        assert!(!e.call(0x009f_5070, &args![reference, functor]).bool());
        let buffer = e.mem.u32(data + POTENTIAL_TARGETS + 4);
        assert_eq!(e.mem.u32(buffer), reference);
        assert_eq!(e.mem.u32(buffer + 4), 2);
        assert_eq!(e.mem.u8(data + FOOD), 1);
        // Dialogue (0x2A) with an actor that is not the player.
        let mut e = Engine::new();
        let (functor, data, reference, _) = callback_setup(&mut e, 0x2a);
        answer(&mut e, PACKAGE_WORD_24_BIT_2_CLEAR, 1);
        answer(&mut e, 0x0600_0100, 1);
        e.map(PLAYER, 4);
        e.call(0x009f_5070, &args![reference, functor]);
        let buffer = e.mem.u32(data + POTENTIAL_TARGETS + 4);
        assert_eq!(e.mem.u32(buffer + 4), 5);
        // The player is skipped.
        e.set_global(PLAYER, reference);
        e.call(0x009f_5070, &args![reference, functor]);
        assert_eq!(e.mem.u32(data + POTENTIAL_TARGETS + 8), 1);
        // Idle marker (0x30) accepted by its collection.
        let mut e = Engine::new();
        let (functor, data, reference, _) = callback_setup(&mut e, 0x30);
        answer(&mut e, IDLE_COLLECTION_VALUE_0092B440, 0);
        answer(&mut e, IDLE_COLLECTION_VALUE_00479E80, 0);
        answer(&mut e, IDLE_COLLECTION_TEST, 1);
        e.call(0x009f_5070, &args![reference, functor]);
        let buffer = e.mem.u32(data + POTENTIAL_TARGETS + 4);
        assert_eq!(e.mem.u32(buffer + 4), 4);
        // Furniture (0x27): a bed, then a chair when sleeping is not allowed.
        let mut e = Engine::new();
        let (functor, data, reference, package) = callback_setup(&mut e, 0x27);
        answer(&mut e, 0x0600_01d0, 0x9000);
        answer(&mut e, FURNITURE_MARKER_FIND, 0x9100);
        answer(&mut e, FURNITURE_MARKER_VALUE, 1);
        answer(&mut e, FURNITURE_CAN_SLEEP_ON, 1);
        answer(&mut e, FURNITURE_CAN_SIT_ON, 1);
        e.call(0x009f_5070, &args![reference, functor]);
        let buffer = e.mem.u32(data + POTENTIAL_TARGETS + 4);
        assert_eq!(e.mem.u32(buffer + 4), 1);
        assert_eq!(e.mem.u8(data + BED), 1);
        e.mem.set_u16(package + 0x24, 0x2);
        e.call(0x009f_5070, &args![reference, functor]);
        assert_eq!(e.mem.u32(buffer + 0x14), 0);
        assert_eq!(e.mem.u8(data + CHAIR), 1);
        // Neither sit nor sleep: still a sit choice.
        answer(&mut e, FURNITURE_CAN_SLEEP_ON, 0);
        answer(&mut e, FURNITURE_CAN_SIT_ON, 0);
        e.call(0x009f_5070, &args![reference, functor]);
        assert_eq!(e.mem.u32(data + POTENTIAL_TARGETS + 8), 3);
        // No model: nothing.
        answer(&mut e, 0x0600_01d0, 0);
        e.call(0x009f_5070, &args![reference, functor]);
        assert_eq!(e.mem.u32(data + POTENTIAL_TARGETS + 8), 3);
    }

    #[test]
    fn scan_callback_skips_flagged_references_and_other_types() {
        let mut e = Engine::new();
        let (functor, data, reference, _) = callback_setup(&mut e, 0x2f);
        answer(&mut e, REFERENCE_FLAG_0X4000, 1);
        e.call(0x009f_5070, &args![reference, functor]);
        assert_eq!(e.mem.u32(data + POTENTIAL_TARGETS + 8), 0);
        answer(&mut e, REFERENCE_FLAG_0X4000, 0);
        answer(&mut e, 0x0600_01a0, 1);
        answer(&mut e, ACTOR_TEST_008859E0, 0);
        e.call(0x009f_5070, &args![reference, functor]);
        assert_eq!(e.mem.u32(data + POTENTIAL_TARGETS + 8), 0);
        answer(&mut e, ACTOR_TEST_008859E0, 1);
        answer(&mut e, REFERENCE_GET_OWNER, 0x55);
        answer(&mut e, REFERENCE_IS_AN_OWNER, 0);
        e.call(0x009f_5070, &args![reference, functor]);
        assert_eq!(e.mem.u32(data + POTENTIAL_TARGETS + 8), 0);
        answer(&mut e, REFERENCE_IS_AN_OWNER, 1);
        e.call(0x009f_5070, &args![reference, functor]);
        assert_eq!(e.mem.u32(data + POTENTIAL_TARGETS + 8), 1);
        let form = e.mem.u32(reference + 0x20);
        e.mem.set_u8(form + 4, 0x28);
        e.call(0x009f_5070, &args![reference, functor]);
        assert_eq!(e.mem.u32(data + POTENTIAL_TARGETS + 8), 1);
        e.mem.set_u32(functor + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x009f_5070, &args![reference, functor]);
        assert!(calls_to(&e, REFERENCE_BASE_FORM).is_empty());
    }

    #[test]
    fn move_to_location_with_the_right_distance() {
        let mut e = Engine::new();
        constants(&mut e);
        answer(&mut e, REFERENCE_GET_WORLD_SPACE, 0x30);
        answer(&mut e, REFERENCE_CELL, 0);
        e.map(SPECIAL_BASE_B, 8);
        word_of(&mut e, REFERENCE_BASE_FORM, 0x20);
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        answer_float(&mut e, REFERENCE_GET_MODEL_BOUND_SIZE, 30.0);
        answer(&mut e, EXTRA_DATA_LIST, 0);
        answer(&mut e, ACTOR_VALUE_0087F990, 1);
        answer_float(&mut e, PROCEDURE_DURATION_006435C0, 5.0);
        let actor = object(&mut e, &[(0x100, 0x0600_0100), (0x2cc, 0x0600_02cc)]);
        answer(&mut e, 0x0600_0100, 1);
        answer(&mut e, 0x0600_02cc, 1);
        let location = object(&mut e, &[(0x100, 0x0600_1100), (0x1f4, 0x0600_11f4)]);
        answer(&mut e, 0x0600_1100, 0);
        answer(&mut e, 0x0600_11f4, 0x4444);
        let form = e.mem.alloc(8);
        e.mem.set_u8(form + 4, 0x40);
        e.mem.set_u32(location + 0x20, form);
        let this = data(&mut e);
        e.mem.set_u32(this + LOCATION, location);
        e.mem.set_u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS, 5);
        e.mem.set_u32(this + PROCEDURE, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x009f_56b0, &args![this, actor, 0.0f32]).bool());
        assert_eq!(
            calls_to(&e, 0x0600_02cc),
            vec![vec![actor, 0x4444, 50.0f32.to_bits(), 1, 1]]
        );
        assert_eq!(e.mem.f32(this + PROCEDURE_DURATION), 5.0);
        assert_eq!(calls_to(&e, PROCEDURE_DURATION_006435C0), vec![vec![1, 2]]);
        // Types 0x15, 0x1B, 0x20, 0x21 add nothing; a given distance wins.
        e.mem.set_u8(form + 4, 0x1b);
        e.call(0x009f_56b0, &args![this, actor, 0.0f32]);
        assert_eq!(calls_to(&e, 0x0600_02cc)[1][2], 30.0f32.to_bits());
        e.call(0x009f_56b0, &args![this, actor, 64.0f32]);
        assert_eq!(calls_to(&e, 0x0600_02cc)[2][2], 64.0f32.to_bits());
        // An actor location of kind 9: 90 and not the last way.
        answer(&mut e, 0x0600_1100, 1);
        let location_vtable = e.mem.u32(location);
        let slots = e.mem.alloc(0x220);
        let copied = e.mem.bytes(location_vtable, 0x1f8);
        e.mem.write(slots, &copied);
        e.mem.set_u32(slots + 0x214, 0x0600_1214);
        e.mem.set_u32(location, slots);
        answer(&mut e, 0x0600_1214, 9);
        e.call(0x009f_56b0, &args![this, actor, 0.0f32]);
        assert_eq!(calls_to(&e, 0x0600_02cc)[3][2..], [90.0f32.to_bits(), 0, 1]);
        // Different worldspaces: no move.
        e.register_double(REFERENCE_GET_WORLD_SPACE, |_, a| a[0].into_ret());
        assert!(!e.call(0x009f_56b0, &args![this, actor, 0.0f32]).bool());
        // No location: the package decides.
        e.mem.set_u32(this + LOCATION, 0);
        let package = object(&mut e, &[(0x13c, 0x0600_013c)]);
        answer(&mut e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, package);
        answer(&mut e, 0x0600_013c, 1);
        assert!(e.call(0x009f_56b0, &args![this, actor, 0.0f32]).bool());
        assert_eq!(
            calls_to(&e, 0x0600_013c),
            vec![vec![package, actor, 0, (-1.0f32).to_bits(), 0]]
        );
        assert!(!e.call(0x009f_56b0, &args![this, 0u32, 0.0f32]).bool());
    }

    #[test]
    fn save_game_writes_every_field_and_choice() {
        let mut e = Engine::new();
        e.register(SAVE_DATA, nothing);
        e.register(SAVE_FORM_ID, nothing);
        e.register(SAVE_VARIABLE_SIZED_VALUE, nothing);
        answer(&mut e, TIMER_NOW, 100);
        word_of(&mut e, WORD_AT_8, 8);
        e.register(ELEMENT_TARGET_00558DD0, |e, a| {
            (e.mem.u32(a[0] + 4) + (a[1] << 4)).into_ret()
        });
        let this = data(&mut e);
        e.mem.set_u32(this + PREV_TARGET_CHANGED_TIME_STAMP, 40);
        e.mem.set_u32(this + LOCATION, 0x11);
        e.mem.set_u32(this + CURRENT_TARGET_OBJECT, 0x22);
        e.mem.set_u32(this + PREV_TARGET_OBJECT, 0x33);
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(this + POTENTIAL_TARGETS + 4, buffer);
        e.mem.set_u32(this + POTENTIAL_TARGETS + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x009f_5a60, &args![this, 0x77u32]);
        let saved = calls_to(&e, SAVE_DATA);
        assert_eq!(saved.len(), 20 + 6);
        let offsets: Vec<u32> = saved[..14].iter().map(|w| w[1] - this).collect();
        assert_eq!(
            offsets,
            vec![4, 8, 0xc, 0x14, 0x18, 0x1c, 0xc, 0x30, 0x34, 0x35, 0x36, 0x37, 0x38, 0x40]
        );
        assert_eq!(saved[19], vec![0x77, this + 0x5c, 6, 0]);
        let forms = calls_to(&e, SAVE_FORM_ID);
        assert_eq!(
            forms[..3],
            [
                vec![0x77, 0x11, 0],
                vec![0x77, 0x22, 0],
                vec![0x77, 0x33, 0]
            ]
        );
        assert_eq!(forms.len(), 5);
        assert_eq!(calls_to(&e, SAVE_VARIABLE_SIZED_VALUE), vec![vec![0x77, 2]]);
        assert_eq!(saved[20], vec![0x77, buffer + 4, 4, 0]);
    }

    #[test]
    fn save_game_time_since_the_previous_target() {
        let mut e = Engine::new();
        e.register(SAVE_FORM_ID, nothing);
        e.register(SAVE_VARIABLE_SIZED_VALUE, nothing);
        answer(&mut e, TIMER_NOW, 100);
        word_of(&mut e, WORD_AT_8, 8);
        let this = data(&mut e);
        e.mem.set_u32(this + PREV_TARGET_CHANGED_TIME_STAMP, 40);
        let seen = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let record = seen.clone();
        e.register_double(SAVE_DATA, move |e, a| {
            if a[2] == 4 && (a[1] < this || a[1] >= this + 0x64) {
                record.set(e.mem.u32(a[1]));
            }
            Ret::default()
        });
        e.call(0x009f_5a60, &args![this, 0x77u32]);
        assert_eq!(seen.get(), 60);
    }

    fn pick_doubles(e: &mut Engine) -> (u32, u32) {
        constants(e);
        answer(e, TIMER_NOW, 1000);
        e.register(SET_MARKER_RESERVED, nothing);
        e.map(DATA_HANDLER, 4);
        e.register(ARRAY_RELEASE_008454F0, nothing);
        e.register(ACTIVITY_TIMES, |e, a| {
            e.mem.set_f32(a[1], 30.0);
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        calendar(e, 2, 10.0);
        e.map(SETTING_RADIUS, 8);
        e.mem.set_f32(SETTING_RADIUS + 4, 100.0);
        e.map(SETTING_DIALOGUE_RANGE, 8);
        e.mem.set_f32(SETTING_DIALOGUE_RANGE + 4, 50.0);
        e.register(SETTING_FLOAT, |_, a| (a[0] + 4).into_ret());
        e.register(SETTING_INT, |_, a| (a[0] + 4).into_ret());
        for setting in [
            SETTING_PREVIOUS_TARGET_SECONDS,
            SETTING_SCAN_DELAY_A,
            SETTING_SCAN_DELAY_B,
        ] {
            e.map(setting, 8);
            e.mem.set_u32(setting + 4, 2);
        }
        answer(e, SCAN_DELAY_00944460, 3000);
        word_of(e, REFERENCE_CELL, 0x40);
        answer(e, CELL_GET_WORLD_SPACE, 0x30);
        answer(e, REFERENCE_GET_WORLD_SPACE, 0x30);
        answer(e, CELL_FLAG_0, 0);
        word_of(e, WORD_AT_8, 8);
        word_of(e, WORD_AT_4, 4);
        word_of(e, CHOICE_TARGET, 0);
        word_of(e, CHOICE_FULL_SCORE, 0xc);
        e.register(CHOICE_SET_FULL_SCORE, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        e.register(SCORE_00647B70, |_, a| a[0].into_ret());
        e.register(ELEMENT_TARGET_00558DD0, |e, a| {
            (e.mem.u32(a[0] + 4) + (a[1] << 4)).into_ret()
        });
        e.register(POSITION_DIFFERENCE, |e, a| {
            for i in 0..3 {
                let d = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, d);
            }
            a[1].into_ret()
        });
        e.register(SQUARED_LENGTH, |e, a| {
            let mut sum = 0.0f32;
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i);
                sum += v * v;
            }
            sum.into_ret()
        });
        answer(e, REFERENCE_HAS_FREE_MARKER, 1);
        answer(e, REFERENCE_GET_FIRST_FREE_MARKER_INDEX, 2);
        answer(e, ACTOR_VALUE_0087F990, 1);
        answer_float(e, PROCEDURE_DURATION_006435C0, 4.0);
        answer(e, RAND, 7);
        answer(e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, 0);
        let position = e.mem.alloc(12);
        let actor = object(e, &[(0x1f4, 0x0600_01f4)]);
        e.register_double(0x0600_01f4, move |_, _| position.into_ret());
        word_of(e, OBJECT_AT_0X68, 0x68);
        (actor, position)
    }

    /// A target reference at `x` on the axis.
    fn target(e: &mut Engine, x: f32, slot: u32) -> u32 {
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, x);
        let target = object(e, &[(0x1f4, slot)]);
        e.register_double(slot, move |_, _| position.into_ret());
        target
    }

    #[test]
    fn pick_procedure_draws_a_scored_choice() {
        let mut e = Engine::new();
        let (actor, _) = pick_doubles(&mut e);
        let this = data(&mut e);
        e.mem.set_u32(this + NEXT_TIME_TO_SCAN_WORLD, 5000);
        e.mem.set_u32(this + ACTIVITY, 0xffff_ffff);
        for i in 0..6 {
            e.mem.set_u8(this + PROCEDURE_WEIGHT + i, 2);
        }
        e.mem.set_u8(this + PROCEDURE_WEIGHT + 5, 0xff);
        e.mem.set_u8(this + CHAIR, 1);
        // Choices: sit near (score 3 * 2), eat far (0), wander (5 * 2).
        let near = target(&mut e, 10.0, 0x0610_0000);
        let far = target(&mut e, 1000.0, 0x0610_0004);
        let buffer = e.mem.alloc(0x30);
        let choices = [(near, 0u32, 3u32), (far, 2, 9), (0, 3, 5)];
        for (i, (t, p, s)) in choices.iter().enumerate() {
            let item = buffer + 0x10 * i as u32;
            e.mem.set_u32(item, *t);
            e.mem.set_u32(item + 4, *p);
            e.mem.set_u32(item + 8, *s);
        }
        e.mem.set_u32(this + POTENTIAL_TARGETS + 4, buffer);
        e.mem.set_u32(this + POTENTIAL_TARGETS + 8, 3);
        e.mem.set_u32(this + CURRENT_TARGET_OBJECT, 0x9999);
        e.call_log = Some(vec![]);
        e.call(0x009f_4700, &args![this, actor, 0u32]);
        assert_eq!(e.mem.u32(buffer + 0xc), 6);
        assert_eq!(e.mem.u32(buffer + 0x1c), 0);
        assert_eq!(e.mem.u32(buffer + 0x2c), 10);
        // rand 7 % 16 = 7: past the sit (6), into the wander.
        assert_eq!(e.mem.u32(this + PROCEDURE), 3);
        assert_eq!(e.mem.u32(this + CURRENT_TARGET_OBJECT), 0);
        assert_eq!(e.mem.u8(this + CURRENT_TARGET_MARKER), 0);
        assert_eq!(e.mem.f32(this + PROCEDURE_DURATION), 4.0);
        assert_eq!(
            e.mem.bytes(this + PROCEDURE_WEIGHT, 6),
            vec![3, 3, 3, 1, 3, 0xff]
        );
        assert_eq!(e.mem.f32(this + TOTAL_TIME), 24.0);
        assert_eq!(e.mem.f32(this + LAST_TIME_CHANGED), 10.0);
        // The old target became the previous one for 2 seconds.
        assert_eq!(e.mem.u32(this + PREV_TARGET_OBJECT), 0x9999);
        assert_eq!(e.mem.u32(this + PREV_TARGET_CHANGED_TIME_STAMP), 3000);
        assert_eq!(calls_to(&e, SET_MARKER_RESERVED), vec![vec![0x9999, 0, 0]]);
        assert!(calls_to(&e, ENUM_REFERENCES_CLOSE_TO_POINT).is_empty());
    }

    #[test]
    fn pick_procedure_without_any_score_resets() {
        let mut e = Engine::new();
        let (actor, _) = pick_doubles(&mut e);
        e.register(ENUM_REFERENCES_CLOSE_TO_POINT, nothing);
        e.register(FUNCTOR_CONSTRUCT, nothing);
        let this = data(&mut e);
        e.mem.set_u32(this + ACTIVITY, 0xffff_ffff);
        e.mem.set_u32(this + PROCEDURE, 2);
        e.mem.set_f32(this + TOTAL_TIME, 6.0);
        let package = e.mem.alloc(0x28);
        e.mem.set_u16(package + 0x24, 0x21);
        e.register(PACKAGE_GET_LOCATION_COORD, |_, a| a[1].into_ret());
        answer(&mut e, PACKAGE_LOCATION_RADIUS, 0);
        answer(&mut e, PACKAGE_GET_LOCATION_CELL, 0);
        answer(&mut e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, package);
        e.call_log = Some(vec![]);
        e.call(0x009f_4700, &args![this, actor, 0u32]);
        // The scan ran (scan time 0) and set the next one.
        assert_eq!(calls_to(&e, ENUM_REFERENCES_CLOSE_TO_POINT).len(), 1);
        assert_eq!(e.mem.u32(this + NEXT_TIME_TO_SCAN_WORLD), 4000);
        assert_eq!(calls_to(&e, SCAN_DELAY_00944460), vec![vec![2000, 2000]]);
        assert_eq!(e.mem.u32(this + PROCEDURE), 0xffff_ffff);
        assert_eq!(e.mem.u8(this + CURRENT_TARGET_MARKER), 0x7f);
        assert_eq!(e.mem.f32(this + TOTAL_TIME), 6.0);
        assert!(calls_to(&e, RAND).is_empty());
    }
}

#[cfg(test)]
mod load_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn answer_float(e: &mut Engine, addr: u32, value: f32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn word_of(e: &mut Engine, addr: u32, offset: u32) {
        e.register_double(addr, move |e, a| e.mem.u32(a[0] + offset).into_ret());
    }

    fn constant(e: &mut Engine, addr: u32, value: f64) {
        e.map(addr, 8);
        e.set_global(addr, value);
    }

    fn constant32(e: &mut Engine, addr: u32, value: f32) {
        e.map(addr, 4);
        e.set_global(addr, value);
    }

    /// An object whose vtable has `slots` (byte offset, target).
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let size = slots.iter().map(|(s, _)| s + 4).max().unwrap_or(4);
        let vtable = e.mem.alloc(size);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(0x80);
        e.mem.set_u32(object, vtable);
        object
    }

    /// The doubles `009f4700` needs to pick among the choices (as the
    /// `sandbox_tests` set-up does): returns the actor.
    fn pick_doubles(e: &mut Engine) -> u32 {
        constant(e, ZERO, 0.0);
        constant(e, HOURS_PER_DAY, 24.0);
        constant(e, SIXTY, 60.0);
        constant(e, THOUSAND, 1000.0);
        constant(e, TWENTY_DOUBLE, 20.0);
        constant32(e, FLOAT_MAX, f32::MAX);
        constant32(e, DEFAULT_TOTAL_TIME, 24.0);
        constant32(e, MINUS_ONE, -1.0);
        constant32(e, TWENTY, 20.0);
        constant32(e, NINETY, 90.0);
        answer(e, TIMER_NOW, 1000);
        e.register(SET_MARKER_RESERVED, nothing);
        e.map(DATA_HANDLER, 4);
        e.register(ARRAY_RELEASE_008454F0, nothing);
        e.register(ACTIVITY_TIMES, |e, a| {
            e.mem.set_f32(a[1], 30.0);
            e.mem.set_f32(a[2], 1.0);
            Ret::default()
        });
        answer(e, CALENDAR_DAYS, 2);
        answer_float(e, CALENDAR_GET_HOUR, 10.0);
        e.map(SETTING_RADIUS, 8);
        e.mem.set_f32(SETTING_RADIUS + 4, 100.0);
        e.map(SETTING_DIALOGUE_RANGE, 8);
        e.mem.set_f32(SETTING_DIALOGUE_RANGE + 4, 50.0);
        e.register(SETTING_FLOAT, |_, a| (a[0] + 4).into_ret());
        e.register(SETTING_INT, |_, a| (a[0] + 4).into_ret());
        for setting in [
            SETTING_PREVIOUS_TARGET_SECONDS,
            SETTING_SCAN_DELAY_A,
            SETTING_SCAN_DELAY_B,
        ] {
            e.map(setting, 8);
            e.mem.set_u32(setting + 4, 2);
        }
        answer(e, SCAN_DELAY_00944460, 3000);
        word_of(e, REFERENCE_CELL, 0x40);
        answer(e, CELL_GET_WORLD_SPACE, 0x30);
        answer(e, REFERENCE_GET_WORLD_SPACE, 0x30);
        answer(e, CELL_FLAG_0, 0);
        word_of(e, WORD_AT_8, 8);
        word_of(e, WORD_AT_4, 4);
        word_of(e, CHOICE_TARGET, 0);
        word_of(e, CHOICE_FULL_SCORE, 0xc);
        e.register(CHOICE_SET_FULL_SCORE, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        e.register(SCORE_00647B70, |_, a| a[0].into_ret());
        e.register(ELEMENT_TARGET_00558DD0, |e, a| {
            (e.mem.u32(a[0] + 4) + (a[1] << 4)).into_ret()
        });
        e.register(POSITION_DIFFERENCE, |e, a| {
            for i in 0..3 {
                let d = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, d);
            }
            a[1].into_ret()
        });
        e.register(SQUARED_LENGTH, |e, a| {
            let mut sum = 0.0f32;
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i);
                sum += v * v;
            }
            sum.into_ret()
        });
        answer(e, REFERENCE_HAS_FREE_MARKER, 1);
        answer(e, REFERENCE_GET_FIRST_FREE_MARKER_INDEX, 2);
        answer(e, ACTOR_VALUE_0087F990, 1);
        answer_float(e, PROCEDURE_DURATION_006435C0, 4.0);
        answer(e, RAND, 7);
        answer(e, MOBILE_OBJECT_GET_CURRENT_PACKAGE, 0);
        let position = e.mem.alloc(12);
        let actor = object(e, &[(0x1f4, 0x0620_01f4)]);
        e.register_double(0x0620_01f4, move |_, _| position.into_ret());
        word_of(e, OBJECT_AT_0X68, 0x68);
        actor
    }

    /// A target reference at `x` on the axis.
    fn target(e: &mut Engine, x: f32, slot: u32) -> u32 {
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, x);
        let target = object(e, &[(0x1f4, slot)]);
        e.register_double(slot, move |_, _| position.into_ret());
        target
    }

    /// The array doubles over 0x10-byte items: the element address, the
    /// item moves of `009f63a0` (copies) and its count helper.
    fn choice_array(e: &mut Engine) {
        e.register(ELEMENT_TARGET_00558DD0, |e, a| {
            (e.mem.u32(a[0] + 4) + (a[1] << 4)).into_ret()
        });
        e.register(MOVE_COUNT_0042F5A0, |_, a| a[0].min(a[1]).into_ret());
        e.register(CHOICE_ARRAY_DESTROY_ITEMS, nothing);
        e.register(CHOICE_ARRAY_MOVE_ITEMS, |e, a| {
            for i in 0..a[3] * 4 {
                let word = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            Ret::default()
        });
        word_of(e, WORD_AT_8, 8);
        word_of(e, WORD_AT_4, 4);
        word_of(e, CHOICE_TARGET, 0);
    }

    fn place_choices(e: &mut Engine, this: u32, choices: &[(u32, u32, u32)]) -> u32 {
        let buffer = e.mem.alloc(0x100);
        for (i, (t, p, s)) in choices.iter().enumerate() {
            let item = buffer + 0x10 * i as u32;
            e.mem.set_u32(item, *t);
            e.mem.set_u32(item + 4, *p);
            e.mem.set_u32(item + 8, *s);
        }
        e.mem.set_u32(this + POTENTIAL_TARGETS + 4, buffer);
        e.mem
            .set_u32(this + POTENTIAL_TARGETS + 8, choices.len() as u32);
        e.mem.set_u32(this + POTENTIAL_TARGETS + 0xc, 0x10);
        buffer
    }

    #[test]
    fn at_009f38b0_loads_three_words_then_the_target() {
        let mut e = Engine::new();
        e.register(LOAD_DATA, nothing);
        e.register(LOAD_FORM_ID, nothing);
        e.call_log = Some(vec![]);
        e.call(0x009f_38b0, &args![0x7000u32, 0x55u32]);
        assert_eq!(
            calls_to(&e, LOAD_DATA),
            vec![
                vec![0x55, 0x7004, 4],
                vec![0x55, 0x7008, 4],
                vec![0x55, 0x700c, 4]
            ]
        );
        assert_eq!(calls_to(&e, LOAD_FORM_ID), vec![vec![0x55, 0x7000]]);
    }

    #[test]
    fn at_009f41c0_lets_go_of_the_marker_and_asks_for_a_procedure() {
        let mut e = Engine::new();
        e.map(DATA_HANDLER, 4);
        e.register(SET_MARKER_RESERVED, nothing);
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this + CURRENT_TARGET_OBJECT, 0x4444);
        e.mem.set_u8(this + CURRENT_TARGET_MARKER, 3);
        e.call_log = Some(vec![]);
        e.call(0x009f_41c0, &args![this]);
        assert_eq!(calls_to(&e, SET_MARKER_RESERVED), vec![vec![0x4444, 3, 0]]);
        assert_eq!(e.mem.u32(this + CURRENT_TARGET_OBJECT), 0);
        assert_eq!(e.mem.u8(this + CURRENT_TARGET_MARKER), 0x7f);
        assert_eq!(e.mem.u8(this + SHOULD_PICK_NEW_PROCEDURE), 1);
        assert_eq!(e.mem.u32(this + STATE), 1);
    }

    #[test]
    fn at_009f4680_clears_the_current_target_choices_then_picks() {
        for forget in [true, false] {
            let mut e = Engine::new();
            let actor = pick_doubles(&mut e);
            let this = e.mem.alloc(0x64);
            e.mem.set_u32(this + NEXT_TIME_TO_SCAN_WORLD, 5000);
            e.mem.set_u32(this + ACTIVITY, 0xffff_ffff);
            for i in 0..6 {
                e.mem.set_u8(this + PROCEDURE_WEIGHT + i, 2);
            }
            let near = target(&mut e, 10.0, 0x0630_0000);
            let buffer = place_choices(&mut e, this, &[(near, 0, 3), (0, 3, 5), (near, 1, 4)]);
            if forget {
                e.mem.set_u32(this + CURRENT_TARGET_OBJECT, near);
            }
            e.call(0x009f_4680, &args![this, actor, 0u32]);
            let scores: Vec<u32> = (0..3).map(|i| e.mem.u32(buffer + 0x10 * i + 8)).collect();
            if forget {
                assert_eq!(scores, vec![0, 5, 0]);
            } else {
                assert_eq!(scores, vec![3, 5, 4]);
            }
            // `009f4700` ran: the procedure was picked and the wish cleared.
            assert_eq!(e.mem.u8(this + SHOULD_PICK_NEW_PROCEDURE), 0);
            assert_eq!(e.mem.f32(this + LAST_TIME_CHANGED), 10.0);
        }
    }

    #[test]
    fn at_009f5ca0_loads_every_field_and_choice() {
        let mut e = Engine::new();
        choice_array(&mut e);
        answer(&mut e, TIMER_NOW, 1000);
        e.register(LOAD_FORM_ID, nothing);
        answer(&mut e, LOAD_VARIABLE_SIZED_VALUE, 2);
        e.register(CHOICE_ARRAY_CONSTRUCT_ITEMS, nothing);
        let items = e.mem.alloc(0x20);
        let allocate = 0x0640_0000;
        e.register_double(allocate, move |_, _| items.into_ret());
        let this = e.mem.alloc(0x64);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, allocate);
        e.mem.set_u32(this + POTENTIAL_TARGETS, vtable);
        // The local the time since the previous target is read into: 250.
        e.register_double(LOAD_DATA, move |e, a| {
            let in_data = (this..this + 0x64).contains(&a[1]);
            let in_items = (items..items + 0x20).contains(&a[1]);
            if !in_data && !in_items {
                e.mem.set_u32(a[1], 250);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009f_5ca0, &args![this, 0x55u32]);
        let loads: Vec<(u32, u32)> = calls_to(&e, LOAD_DATA)
            .iter()
            .filter(|a| (this..this + 0x64).contains(&a[1]))
            .map(|a| (a[1] - this, a[2]))
            .collect();
        assert_eq!(
            loads,
            vec![
                (4, 4),
                (8, 4),
                (0xc, 4),
                (0x14, 4),
                (0x18, 4),
                (0x1c, 4),
                (0xc, 4),
                (0x30, 4),
                (0x34, 1),
                (0x35, 1),
                (0x36, 1),
                (0x37, 1),
                (0x38, 4),
                (0x40, 1),
                (0x4c, 4),
                (0x50, 4),
                (0x54, 4),
                (0x58, 4),
                (0x5c, 6),
            ]
        );
        assert_eq!(e.mem.u32(this + PREV_TARGET_CHANGED_TIME_STAMP), 750);
        assert_eq!(
            calls_to(&e, LOAD_FORM_ID),
            vec![
                vec![0x55, this + 0x10],
                vec![0x55, this + 0x3c],
                vec![0x55, this + 0x44],
                vec![0x55, items],
                vec![0x55, items + 0x10],
            ]
        );
        assert_eq!(e.mem.u32(this + POTENTIAL_TARGETS + 4), items);
        assert_eq!(e.mem.u32(this + POTENTIAL_TARGETS + 8), 2);
    }

    /// The doubles `InitLoadGame` needs: ids resolve to `id + 0x1000`.
    fn init_doubles(e: &mut Engine) {
        choice_array(e);
        answer(e, TIMER_NOW, 1000);
        e.register(LOOKUP_FORM_004839C0, |_, a| (a[0] + 0x1000).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(SET_TARGETED, nothing);
        e.register(CHOICE_INIT_LOAD_GAME_009A4F40, nothing);
        e.register(PROCEDURE_TEST_0092A980, |_, a| (a[0] == 7).into_ret());
        e.register(SET_MARKER_RESERVED, nothing);
        e.register(EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        answer(e, EXTRA_DATA_0X51, 1);
        answer(e, EXTRA_DATA_0X6F, 0x6f00);
        answer_float(e, FLOAT_AT_0, 2.5);
        constant(e, THOUSAND, 1000.0);
    }

    #[test]
    fn at_009f5ea0_drops_target_less_choices_and_resets() {
        let mut e = Engine::new();
        init_doubles(&mut e);
        answer(&mut e, REFERENCE_HAS_FREE_MARKER, 1);
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this + LOCATION, 0x10);
        e.mem.set_u32(this + CURRENT_TARGET_OBJECT, 0x20);
        e.mem.set_u8(this + CURRENT_TARGET_MARKER, 2);
        e.mem.set_u32(this + PROCEDURE, 4);
        let buffer = place_choices(&mut e, this, &[(5, 1, 1), (0, 7, 2), (0, 8, 3)]);
        e.call_log = Some(vec![]);
        e.call(0x009f_5ea0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(this + LOCATION), 0x1010);
        assert_eq!(calls_to(&e, SET_TARGETED), vec![vec![0x1020, 1]]);
        assert_eq!(calls_to(&e, CHOICE_INIT_LOAD_GAME_009A4F40).len(), 3);
        // The target-less choice of procedure 7 went; the last moved down.
        assert_eq!(e.mem.u32(this + POTENTIAL_TARGETS + 8), 2);
        assert_eq!(e.mem.u32(buffer + 0x14), 8);
        // The marker is reserved again, but the drop resets the procedure.
        assert_eq!(calls_to(&e, SET_MARKER_RESERVED), vec![vec![0x1020, 2, 1]]);
        assert_eq!(e.mem.u32(this + PROCEDURE), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + CURRENT_TARGET_OBJECT), 0);
        assert_eq!(e.mem.u8(this + CURRENT_TARGET_MARKER), 0x7f);
        assert_eq!(e.mem.u8(this + SHOULD_PICK_NEW_PROCEDURE), 1);
        // The location's change time: now plus 2.5 s.
        assert_eq!(e.mem.u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS), 3500);
    }

    #[test]
    fn at_009f5ea0_keeps_a_target_with_a_free_marker() {
        let mut e = Engine::new();
        init_doubles(&mut e);
        answer(&mut e, REFERENCE_HAS_FREE_MARKER, 1);
        let this = e.mem.alloc(0x64);
        e.mem.set_u32(this + LOCATION, 0x10);
        e.mem.set_u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS, 77);
        e.mem.set_u32(this + CURRENT_TARGET_OBJECT, 0x20);
        e.mem.set_u32(this + PREV_TARGET_OBJECT, 0x30);
        e.mem.set_u32(this + PROCEDURE, 4);
        place_choices(&mut e, this, &[(5, 1, 1), (0, 8, 3)]);
        e.call_log = Some(vec![]);
        e.call(0x009f_5ea0, &args![this, 0x55u32]);
        assert_eq!(
            calls_to(&e, SET_TARGETED),
            vec![vec![0x1020, 1], vec![0x1030, 1]]
        );
        assert_eq!(e.mem.u32(this + POTENTIAL_TARGETS + 8), 2);
        assert_eq!(e.mem.u32(this + PROCEDURE), 4);
        assert_eq!(e.mem.u32(this + CURRENT_TARGET_OBJECT), 0x1020);
        assert_eq!(e.mem.u32(this + NEXT_TIME_TO_CHANGE_LOCATIONS), 77);
        // No free marker: the choice is dropped all the same.
        answer(&mut e, REFERENCE_HAS_FREE_MARKER, 0);
        e.mem.set_u32(this + CURRENT_TARGET_OBJECT, 0x20);
        e.call(0x009f_5ea0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u32(this + PROCEDURE), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + CURRENT_TARGET_OBJECT), 0);
    }

    #[test]
    fn at_009f6200_sizes_the_choice_array() {
        let mut e = Engine::new();
        e.register(ARRAY_RELEASE_008454F0, nothing);
        e.register(CHOICE_ARRAY_CONSTRUCT_ITEMS, nothing);
        e.register(CHOICE_ARRAY_DESTROY_ITEMS, nothing);
        e.register(CHOICE_ARRAY_REALLOCATE_009F6540, nothing);
        let allocate = 0x0650_0000;
        e.register(allocate, |_, _| 0x7000u32.into_ret());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, allocate);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        // Size 0: the release with the flag.
        e.call_log = Some(vec![]);
        e.call(0x009f_6200, &args![this, 0u32, 1u32]);
        assert_eq!(calls_to(&e, ARRAY_RELEASE_008454F0), vec![vec![this, 1]]);
        // No buffer yet: allocated through slot +4, items constructed.
        e.call_log = Some(vec![]);
        e.call(0x009f_6200, &args![this, 3u32, 0u32]);
        assert_eq!(calls_to(&e, allocate), vec![vec![this, 3]]);
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_CONSTRUCT_ITEMS),
            vec![vec![this, 0x7000, 3]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.u32(this + 8), 3);
        // Past the reserved size: reallocated, the new items constructed.
        e.call_log = Some(vec![]);
        e.call(0x009f_6200, &args![this, 0x10u32, 0u32]);
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_REALLOCATE_009F6540),
            vec![vec![this, 0x10, 3]]
        );
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_CONSTRUCT_ITEMS),
            vec![vec![this, 0x7030, 0xd]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 0x10);
        // Shrinking without the flag keeps the buffer.
        e.call_log = Some(vec![]);
        e.call(0x009f_6200, &args![this, 0xcu32, 0u32]);
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_DESTROY_ITEMS),
            vec![vec![this, 0x70c0, 4]]
        );
        assert!(calls_to(&e, CHOICE_ARRAY_REALLOCATE_009F6540).is_empty());
        assert_eq!(e.mem.u32(this + 8), 0xc);
        // Shrinking with the flag to a quarter of the reserved size reallocates.
        e.call_log = Some(vec![]);
        e.call(0x009f_6200, &args![this, 4u32, 1u32]);
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_DESTROY_ITEMS),
            vec![vec![this, 0x7040, 8]]
        );
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_REALLOCATE_009F6540),
            vec![vec![this, 4, 4]]
        );
        assert_eq!(e.mem.u32(this + 0xc), 4);
        // Growing within the reserved size only constructs.
        e.mem.set_u32(this + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x009f_6200, &args![this, 4u32, 1u32]);
        assert_eq!(
            calls_to(&e, CHOICE_ARRAY_CONSTRUCT_ITEMS),
            vec![vec![this, 0x7020, 2]]
        );
        assert!(calls_to(&e, CHOICE_ARRAY_REALLOCATE_009F6540).is_empty());
        assert_eq!(e.mem.u32(this + 8), 4);
    }

    #[test]
    fn at_009f6430_destroys_then_frees_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (1, true), (2, false), (3, true)] {
            let mut e = Engine::new();
            e.register(ARRAY_RELEASE_008454F0, nothing);
            e.register(OPERATOR_DELETE, nothing);
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009f_6430, &args![this, flags]).u32(), this);
            assert_eq!(e.mem.u32(this), ARRAY_VTABLE_01093614);
            assert_eq!(calls_to(&e, ARRAY_RELEASE_008454F0), vec![vec![this, 1]]);
            assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), usize::from(frees));
        }
    }
}
