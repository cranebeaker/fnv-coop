//! `fallout shared/tesanimgroup.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

const GROUP_TYPE_TABLE: u32 = 0x0119_77e4;
const SECTION_NAME_TABLE: u32 = 0x0119_9a50;
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const MEMSET: u32 = 0x0040_3d30;
const MEMCPY: u32 = 0x0040_1460;
const REALLOC: u32 = 0x0042_f5d0;
const SECONDS_SINCE_START: u32 = 0x0063_9aa0;
const STRCHR: u32 = 0x0043_b9d0;
const STRRCHR: u32 = 0x00ec_6e30;
const STRLEN: u32 = 0x0044_a670;
const STRNICMP: u32 = 0x00ec_7ec0;
const ISNAN: u32 = 0x00ec_75b1;
const INVALID_ACTION_LOG: u32 = 0x005b_5e40;
const TYPE_NAME_TABLE: u32 = 0x0119_77d8;
const MOVE_NAME_TABLE: u32 = 0x0119_7794;
const WEAPON_NAME_TABLE: u32 = 0x0119_77a4;
const PA_PREFIX: u32 = 0x0104_85c4;
const LOG_ANIM_GROUP_ACTION_LOW: u32 = 0x0104_8578;
const LOG_ANIM_GROUP_ACTION_HIGH: u32 = 0x0104_8528;
const LOG_ANIM_GROUP_TIME_NAN: u32 = 0x0104_84f8;
const NOTE_PRIORITY_SCALE: u32 = 0x0104_84f0;

fn group_type(group: u16) -> u32 {
    u32::from(group & 0xff)
}

fn group_category(e: &Engine, group: u16) -> u32 {
    let ty = group_type(group);
    e.mem
        .u32(GROUP_TYPE_TABLE.wrapping_add(ty.wrapping_mul(0x24)))
}

fn read_c_string_byte(e: &Engine, address: u32) -> u8 {
    e.mem.u8(address)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0049_3800, fn_00493800(Ptr, Ptr) -> f64),
        entry!(0x005f_21c0, fn_005f21c0(Ptr, u16) -> Ptr),
        entry!(0x005f_22a0, fn_005f22a0(Ptr, u32) -> Ptr),
        entry!(0x005f_22d0, fn_005f22d0(Ptr)),
        entry!(0x005f_2370, fn_005f2370(u32, u16, u32, u8) -> u32),
        entry!(0x005f_23a0, fn_005f23a0(Ptr) -> u32),
        entry!(0x005f_23c0, fn_005f23c0(u16) -> u32),
        entry!(0x005f_23e0, fn_005f23e0(Ptr) -> u32),
        entry!(0x005f_2400, fn_005f2400(u16) -> u32),
        entry!(0x005f_2420, fn_005f2420(Ptr) -> u32),
        entry!(0x005f_2440, fn_005f2440(u16) -> u8),
        entry!(0x005f_2450, fn_005f2450(Ptr, u32)),
        entry!(0x005f_2540, fn_005f2540(u16) -> bool),
        entry!(0x005f_25a0, fn_005f25a0(i32) -> bool),
        entry!(0x005f_25d0, fn_005f25d0(u16) -> bool),
        entry!(0x005f_2610, fn_005f2610(u32) -> bool),
        entry!(0x005f_2630, fn_005f2630(u16) -> bool),
        entry!(0x005f_2670, fn_005f2670(u16) -> bool),
        entry!(0x005f_26b0, fn_005f26b0(u32) -> bool),
        entry!(0x005f_26c0, fn_005f26c0(Ptr) -> bool),
        entry!(0x005f_2700, fn_005f2700(i32) -> bool),
        entry!(0x005f_2720, fn_005f2720(u16) -> bool),
        entry!(0x005f_2750, fn_005f2750(u32) -> bool),
        entry!(0x005f_2820, fn_005f2820(Ptr, u32)),
        entry!(0x005f_29f0, fn_005f29f0(Ptr, u32, f32)),
        entry!(0x005f_2a30, fn_005f2a30(Ptr, u32, u32)),
        entry!(0x005f_2a70, fn_005f2a70(Ptr, u32, u8, u32)),
        entry!(0x005f_2ac0, fn_005f2ac0(Ptr, u32, f32)),
        entry!(0x005f_2b20, fn_005f2b20(Ptr, u32, u32)),
        entry!(0x005f_3740, fn_005f3740(Ptr, u32, f32) -> bool),
        entry!(0x005f_3780, fn_005f3780(Ptr, u32) -> f64),
        entry!(0x005f_3870, fn_005f3870(Ptr) -> Ptr),
        entry!(0x005f_38d0, fn_005f38d0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x005f_3a20, fn_005f3a20(Ptr, Ptr) -> Ptr),
        entry!(0x005f_4b70, fn_005f4b70(Ptr, u8)),
        entry!(0x005f_4b90, fn_005f4b90(Ptr, u8)),
        entry!(0x005f_4bb0, fn_005f4bb0(Ptr, u32)),
        entry!(0x005f_4bd0, fn_005f4bd0(Ptr, u32, u8)),
        entry!(0x005f_4c00, fn_005f4c00(Ptr, u32) -> u8),
        entry!(0x005f_4c40, fn_005f4c40(Ptr, Ptr)),
        entry!(0x005f_4c70, fn_005f4c70(Ptr) -> f64),
        entry!(0x005f_4c90, fn_005f4c90(Ptr, Ptr) -> Ptr),
        entry!(0x005f_4cc0, fn_005f4cc0(u16) -> bool),
        entry!(0x005f_4d00, fn_005f4d00(Ptr, Ptr)),
    ]
}

// Translated from 005f21c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup` constructor (Xbox PDB): initializes its base guard,
/// zeroes the owned arrays and flags, copies the default movement vector,
/// then initializes the selected group. The compiler's SEH unwind table is
/// not modeled.
// Translation incomplete: 005f21c0 - compiler SEH constructor-unwind cleanup is not modeled.
pub fn fn_005f21c0(e: &mut Engine, this: Ptr, group: u16) -> Ptr {
    e.call(0x0049_68b0, &args![this]);
    e.mem.set_u32(this.addr(), 0x0104_84e8);
    e.call(0x0068_15c0, &args![this.addr().wrapping_add(0x1c)]);
    e.mem.set_u32(this.addr().wrapping_add(0x18), 0);
    for offset in [0x28, 0x29, 0x2a, 0x2b, 0x2c] {
        e.mem.set_u8(this.addr().wrapping_add(offset), 0);
    }
    for offset in [0x1c, 0x20, 0x24] {
        let value = e.global::<u32>(0x011f_426c + (offset - 0x1c));
        e.mem.set_u32(this.addr().wrapping_add(offset), value);
    }
    e.mem.set_u32(this.addr().wrapping_add(0x30), 0);
    fn_005f2450(e, this, u32::from(group));
    e.mem.set_u32(this.addr().wrapping_add(0x38), 0);
    e.mem.set_u32(this.addr().wrapping_add(0x34), 0);
    this
}

// Translated from 005f22d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup` destructor body (Xbox PDB): frees its time and sound
/// arrays, clears their fields, and destroys the constructor guard. Its
/// compiler-generated SEH cleanup is not modeled.
// Translation incomplete: 005f22d0 - compiler SEH cleanup is not modeled.
pub fn fn_005f22d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0104_84e8);
    let times = e.mem.u32(this.addr().wrapping_add(0x18));
    e.call(OPERATOR_DELETE, &args![times]);
    e.mem.set_u32(this.addr().wrapping_add(0x18), 0);
    let sounds = e.mem.u32(this.addr().wrapping_add(0x38));
    e.call(OPERATOR_DELETE, &args![sounds]);
    e.mem.set_u32(this.addr().wrapping_add(0x38), 0);
    e.mem.set_u32(this.addr().wrapping_add(0x34), 0);
    e.call(0x0049_6910, &args![this]);
}

// Translated from 005f22a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup` scalar deleting destructor (Xbox PDB): invokes its
/// destructor and calls the engine sized-delete helper when bit 0 is set.
pub fn fn_005f22a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_005f22d0(e, this);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x3cu32]);
    }
    this
}

// Translated from 005f2450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `sType`, derives the section count by checking five names for the
/// type's category, and replaces the owned time array with a zeroed array.
pub fn fn_005f2450(e: &mut Engine, this: Ptr, group: u32) {
    let group = group as u16;
    e.mem.set_u16(this.addr().wrapping_add(0x10), group);
    let category = group_category(e, group);
    let mut count = 0u32;
    while count < 5 {
        let name_address = SECTION_NAME_TABLE
            .wrapping_add(count.wrapping_mul(0x2c))
            .wrapping_add(category.wrapping_mul(4));
        let name = e.mem.u32(name_address);
        if read_c_string_byte(e, name) == 0 {
            break;
        }
        count += 1;
    }
    e.mem.set_u32(this.addr().wrapping_add(0x14), count);
    let old_times = e.mem.u32(this.addr().wrapping_add(0x18));
    if old_times != 0 {
        e.call(OPERATOR_DELETE, &args![old_times]);
    }
    let frame_count = e.mem.u32(this.addr().wrapping_add(0x14));
    if frame_count == 0 {
        e.mem.set_u32(this.addr().wrapping_add(0x18), 0);
    } else {
        let bytes = frame_count.saturating_mul(4);
        let times = e.call(OPERATOR_NEW, &args![bytes]).u32();
        e.mem.set_u32(this.addr().wrapping_add(0x18), times);
        let reloaded_times = e.mem.u32(this.addr().wrapping_add(0x18));
        let memset_bytes = e.mem.u32(this.addr().wrapping_add(0x14)).wrapping_shl(2);
        e.call(MEMSET, &args![reloaded_times, 0u32, memset_bytes]);
    }
}

// Translated from 005f2820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resizes the 16-byte note-entry array. New entries receive a zero time,
/// priority byte 0xfa, weight 1.0 and a zero trailing word.
pub fn fn_005f2820(e: &mut Engine, this: Ptr, new_count: u32) {
    let old_count = e.mem.u32(this.addr().wrapping_add(0x34));
    e.mem.set_u32(this.addr().wrapping_add(0x34), new_count);
    let array_field = this.addr().wrapping_add(0x38);
    if new_count == 0 {
        let old = e.mem.u32(array_field);
        e.call(OPERATOR_DELETE, &args![old]);
        e.mem.set_u32(array_field, 0);
        return;
    }

    if old_count == 0 {
        let allocation_bytes = e.mem.u32(this.addr().wrapping_add(0x34)).wrapping_mul(0x10);
        let new_array = e.call(OPERATOR_NEW, &args![allocation_bytes]).u32();
        e.mem.set_u32(array_field, new_array);
        initialize_note_entries(e, this, 0);
        return;
    }

    let old_bytes = old_count.wrapping_mul(0x10);
    let preserved = e.call(OPERATOR_NEW, &args![old_bytes]).u32();
    let old_array = e.mem.u32(array_field);
    e.call(MEMCPY, &args![preserved, old_array, old_bytes]);
    let reloaded_count = e.mem.u32(this.addr().wrapping_add(0x34));
    let reloaded_array = e.mem.u32(array_field);
    let new_array = e
        .call(
            REALLOC,
            &args![reloaded_array, reloaded_count.wrapping_mul(0x10)],
        )
        .u32();
    e.mem.set_u32(array_field, new_array);
    let current_count = e.mem.u32(this.addr().wrapping_add(0x34));
    let copied_count = old_count.min(current_count);
    let copied_bytes = copied_count.wrapping_mul(0x10);
    let destination = e.mem.u32(array_field);
    e.call(MEMCPY, &args![destination, preserved, copied_bytes]);
    e.call(OPERATOR_DELETE, &args![preserved]);
    if old_count < e.mem.u32(this.addr().wrapping_add(0x34)) {
        initialize_note_entries(e, this, old_count);
    }
}

fn initialize_note_entries(e: &mut Engine, this: Ptr, from: u32) {
    let mut index = from;
    while index < e.mem.u32(this.addr().wrapping_add(0x34)) {
        let array = e.mem.u32(this.addr().wrapping_add(0x38));
        let entry = array.wrapping_add(index.wrapping_mul(0x10));
        e.mem.set_u32(entry.wrapping_add(0xc), 0);
        e.mem.set_f32(entry, 0.0);
        e.mem.set_u8(entry.wrapping_add(4), 0xfa);
        e.mem.set_f32(entry.wrapping_add(8), 1.0);
        index = index.wrapping_add(1);
    }
}

// Translated from 005f29f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a note entry's time when the index is below `noteCount` and the
/// array has been allocated.
pub fn fn_005f29f0(e: &mut Engine, this: Ptr, index: u32, value: f32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index < count && array != 0 {
        e.mem.set_u32(
            array.wrapping_add(index.wrapping_mul(0x10)),
            x87_quiet_bits(value.to_bits()),
        );
    }
}

// Translated from 005f2a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a note entry's trailing word when the index is valid.
pub fn fn_005f2a30(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index < count && array != 0 {
        e.mem.set_u32(
            array
                .wrapping_add(index.wrapping_mul(0x10))
                .wrapping_add(0xc),
            value,
        );
    }
}

// Translated from 005f2a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the note entry's priority byte and weight. The PC accepts index
/// equal to `noteCount`, matching the disassembly's unsigned `JBE` branch.
pub fn fn_005f2a70(e: &mut Engine, this: Ptr, index: u32, priority: u8, weight_bits: u32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index <= count && array != 0 {
        let entry = array.wrapping_add(index.wrapping_mul(0x10));
        e.mem.set_u8(entry.wrapping_add(4), priority);
        e.mem.set_u32(entry.wrapping_add(8), weight_bits);
    }
}

// Translated from 005f2ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `trunc(value * 250)` in the note priority byte when
/// the index is valid.
pub fn fn_005f2ac0(e: &mut Engine, this: Ptr, index: u32, value: f32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index < count && array != 0 {
        let scale = e.global::<f64>(NOTE_PRIORITY_SCALE);
        let scaled = (f64::from(value) * scale).trunc();
        let byte = if scaled.is_finite() && scaled >= i32::MIN as f64 && scaled <= i32::MAX as f64 {
            (scaled as i32) as u8
        } else {
            0
        };
        e.mem.set_u8(
            array.wrapping_add(index.wrapping_mul(0x10)).wrapping_add(4),
            byte,
        );
    }
}

// Translated from 005f2b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a note entry's weight bits when the index is valid.
pub fn fn_005f2b20(e: &mut Engine, this: Ptr, index: u32, value: u32) {
    let count = e.mem.u32(this.addr().wrapping_add(0x34));
    let array = e.mem.u32(this.addr().wrapping_add(0x38));
    if index < count && array != 0 {
        e.mem.set_u32(
            array.wrapping_add(index.wrapping_mul(0x10)).wrapping_add(8),
            value,
        );
    }
}

// Translated from 005f3740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores one animation time and reports success when the array exists and
/// the index is below `frameCount`.
pub fn fn_005f3740(e: &mut Engine, this: Ptr, index: u32, time: f32) -> bool {
    let times = e.mem.u32(this.addr().wrapping_add(0x18));
    let count = e.mem.u32(this.addr().wrapping_add(0x14));
    if times == 0 || index >= count {
        return false;
    }
    e.mem.set_u32(
        times.wrapping_add(index.wrapping_mul(4)),
        x87_quiet_bits(time.to_bits()),
    );
    true
}

// Translated from 00493800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the float at +0xd0 of `animation` (`this`) to `00639aa0`'s float of `sequence` (the
/// one stack word, `RET 4`); the sum goes through a float local, so it comes back rounded to
/// a float. The `TESAnimGroup::IsJumpingLoopAnim` map name is folded onto this body.
pub fn fn_00493800(e: &mut Engine, animation: Ptr, sequence: Ptr) -> f64 {
    // 0049380c: `00639aa0` is a thiscall on the stack word (ECX loaded at 00493809).
    let seconds = e.call(SECONDS_SINCE_START, &args![sequence]).f64();
    let offset = e.mem.u32(animation.addr().wrapping_add(0xd0));
    // 00493814: FADD float ptr [+0xd0], FSTP float [EBP-8], FLD float [EBP-8].
    x87_load32(x87_store32(x87_add(seconds, x87_m32(offset))))
}

// Translated from 005f2370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs the movement, weapon, type, and flag selectors into an animation
/// group id, preserving the arithmetic and truncation performed by the PC.
pub fn fn_005f2370(_e: &mut Engine, movement: u32, weapon: u16, group: u32, flag: u8) -> u32 {
    let selectors = group
        .wrapping_add(movement.wrapping_mul(0x1000))
        .wrapping_add(u32::from(flag).wrapping_mul(0x8000));
    let low = (i32::from(weapon as i16)
        .wrapping_mul(0x100)
        .wrapping_add(i32::from(selectors as u16 as i16))) as u16;
    (selectors & 0xffff_0000) | u32::from(low)
}

// Translated from 005f23c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts the movement selector from bits 12 through 14 of the group id.
pub fn fn_005f23c0(_e: &mut Engine, group: u16) -> u32 {
    u32::from((group & 0x7000) >> 12)
}

// Translated from 005f2400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts the weapon selector from bits 8 through 11 of the group id.
pub fn fn_005f2400(_e: &mut Engine, group: u16) -> u32 {
    u32::from((group & 0x0f00) >> 8)
}

// Translated from 005f2440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the low byte of an animation group id.
pub fn fn_005f2440(_e: &mut Engine, group: u16) -> u8 {
    group as u8
}

// Translated from 005f23a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts the movement selector from the group's `sType` field.
pub fn fn_005f23a0(e: &mut Engine, this: Ptr) -> u32 {
    fn_005f23c0(e, e.mem.u16(this.addr().wrapping_add(0x10)))
}

// Translated from 005f23e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Extracts the weapon selector from the group's `sType` field.
pub fn fn_005f23e0(e: &mut Engine, this: Ptr) -> u32 {
    fn_005f2400(e, e.mem.u16(this.addr().wrapping_add(0x10)))
}

// Translated from 005f2420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the low byte of the group's `sType` field.
pub fn fn_005f2420(e: &mut Engine, this: Ptr) -> u32 {
    let group = e.mem.u16(this.addr().wrapping_add(0x10));
    u32::from(fn_005f2440(e, group))
}

// Translated from 005f2610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether a type-table category equals 9.
pub fn fn_005f2610(_e: &mut Engine, category: u32) -> bool {
    category == 9
}

// Translated from 005f26b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether a type-table category equals 6.
pub fn fn_005f26b0(_e: &mut Engine, category: u32) -> bool {
    category == 6
}

// Translated from 005f25a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether an animation type is between 5 and 9, inclusive.
pub fn fn_005f25a0(_e: &mut Engine, value: i32) -> bool {
    (5..=9).contains(&value)
}

// Translated from 005f2700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether an animation type is between 0xb1 and 0xc7, inclusive.
pub fn fn_005f2700(_e: &mut Engine, value: i32) -> bool {
    (0xb1..=0xc7).contains(&value)
}

// Translated from 005f25d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the group's type-table category is 9; group id 0xff is
/// rejected before indexing the table.
pub fn fn_005f25d0(e: &mut Engine, group: u16) -> bool {
    if group == 0xff {
        return false;
    }
    let category = group_category(e, group);
    fn_005f2610(e, category)
}

// Translated from 005f26c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the group's low-byte type is in the inclusive range 0x5c..=0x65.
pub fn fn_005f26c0(e: &mut Engine, this: Ptr) -> bool {
    let group = e.mem.u16(this.addr().wrapping_add(0x10));
    let ty = u32::from(fn_005f2440(e, group));
    (0x5c..=0x65).contains(&ty)
}

// Translated from 005f2540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the type is 0x4a..=0x4f or its category is 5..=9.
pub fn fn_005f2540(e: &mut Engine, group: u16) -> bool {
    let ty = u32::from(fn_005f2440(e, group));
    if ty == 0xff {
        false
    } else if (0x4a..=0x4f).contains(&ty) {
        true
    } else {
        let category = group_category(e, group) as i32;
        fn_005f25a0(e, category)
    }
}

// Translated from 005f2630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the group type is in the inclusive range 0x11..=0x16,
/// except the invalid id 0xff.
pub fn fn_005f2630(e: &mut Engine, group: u16) -> bool {
    if group == 0xff {
        return false;
    }
    let ty = u32::from(fn_005f2440(e, group));
    (0x11..=0x16).contains(&ty)
}

// Translated from 005f2670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether a group's type-table category is 6, except the invalid id.
pub fn fn_005f2670(e: &mut Engine, group: u16) -> bool {
    if group == 0xff {
        return false;
    }
    let category = group_category(e, group);
    fn_005f26b0(e, category)
}

// Translated from 005f2720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests a 16-bit group id against the game's iron-sights type set.
pub fn fn_005f2720(e: &mut Engine, group: u16) -> bool {
    if group == 0xff {
        return false;
    }
    let ty = u32::from(fn_005f2440(e, group));
    fn_005f2750(e, ty)
}

// Translated from 005f2750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns whether `ty` belongs to the 60-value iron-sights type set.
pub fn fn_005f2750(_e: &mut Engine, ty: u32) -> bool {
    matches!(ty, 0x14..=0x16 | 0x1d..=0x1f | 0x23..=0x25 | 0x29..=0x2b | 0x2f..=0x31 | 0x35..=0x37 | 0x3b..=0x3d | 0x41..=0x43 | 0x47..=0x49 | 0x4d..=0x4f | 0x53..=0x55 | 0x59..=0x5b | 0x69..=0x6b | 0x6f..=0x71 | 0x75..=0x77 | 0x7b..=0x7d | 0x81..=0x83 | 0x87..=0x89 | 0x8d..=0x8f | 0x93..=0x95)
}

// Translated from 005f4b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `cBlendFrames`.
pub fn fn_005f4b70(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x29), value);
}

// Translated from 005f4b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `cBlendOutFrames`.
pub fn fn_005f4b90(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x2b), value);
}

// Translated from 005f4bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `pParentName`.
pub fn fn_005f4bb0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x30), value);
}

// Translated from 005f4c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies three float words from `value` into `speed`.
pub fn fn_005f4c40(e: &mut Engine, this: Ptr, value: Ptr) {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(value.addr().wrapping_add(offset));
        e.mem.set_u32(this.addr().wrapping_add(0x1c + offset), word);
    }
}

// Translated from 005f4c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the length of this group's vector at +0x1c through the game's
/// vector helper, preserving its x87 floating-point result.
pub fn fn_005f4c70(e: &mut Engine, this: Ptr) -> f64 {
    e.call(0x0045_7990, &args![this.addr().wrapping_add(0x1c)])
        .f64()
}

// Translated from 005f4c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies this group's three-word vector at +0x1c to `out` and returns `out`.
pub fn fn_005f4c90(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    for offset in [0, 4, 8] {
        let word = e.mem.u32(this.addr().wrapping_add(0x1c + offset));
        e.mem.set_u32(out.addr().wrapping_add(offset), word);
    }
    out
}

// Translated from 005f4cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the group type is 0, 1 or 0xaa. A static `cdecl` of one word (its caller
/// 0089739d pushes only the group and pops one word; ECX is not read).
pub fn fn_005f4cc0(e: &mut Engine, group: u16) -> bool {
    matches!(fn_005f2440(e, group), 0 | 1 | 0xaa)
}

// Translated from 005f4d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup::FillPriorityArray` (Xbox PDB): reads priorities for eight
/// named sequence bones and stores each non-null table entry at +8..+15.
pub fn fn_005f4d00(e: &mut Engine, this: Ptr, sequence: Ptr) {
    for index in 0..8u32 {
        let name = e.mem.u32(0x0119_9bcc + index * 4);
        if name != 0 {
            let priority = e.call(0x004e_f9a0, &args![sequence, name, 0u32]).u32();
            e.mem.set_u8(this.addr() + 8 + index, priority as u8);
        }
    }
}

// Translated from 005f4bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the per-sound flag at entry `index` when it is below `soundCount`.
pub fn fn_005f4bd0(e: &mut Engine, this: Ptr, index: u32, value: u8) {
    if index < e.mem.u32(this.addr().wrapping_add(0x0c)) {
        let sounds = e.mem.u32(this.addr().wrapping_add(0x14));
        e.mem.set_u8(
            sounds
                .wrapping_add(0x0d)
                .wrapping_add(index.wrapping_mul(0x10)),
            value,
        );
    }
}

// Translated from 005f4c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the per-sound flag, or 0xff when `index` is outside `soundCount`.
pub fn fn_005f4c00(e: &mut Engine, this: Ptr, index: u32) -> u8 {
    let count = e.mem.u32(this.addr().wrapping_add(0x0c));
    if index >= count {
        0xff
    } else {
        let sounds = e.mem.u32(this.addr().wrapping_add(0x14));
        e.mem.u8(sounds
            .wrapping_add(0x0d)
            .wrapping_add(index.wrapping_mul(0x10)))
    }
}

// Translated from 005f3780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the time stored at `index` (anything but NaN), zero when the time
/// array is null, and zero after logging a negative or out-of-range index or a
/// NaN time. The type name is looked up only for the two index logs.
pub fn fn_005f3780(e: &mut Engine, this: Ptr, index: u32) -> f64 {
    let signed_index = index as i32;
    let group = e.mem.u16(this.addr().wrapping_add(0x10));
    if signed_index < 0 {
        // 005f3790-005f37bd: four words (ADD ESP,0x10); the group id pushed
        // first at 005f3794 is the trailing %04X argument.
        let ty = u32::from(fn_005f2440(e, group));
        let type_name = e
            .mem
            .u32(TYPE_NAME_TABLE.wrapping_add(ty.wrapping_mul(0x24)));
        e.call(
            INVALID_ACTION_LOG,
            &args![
                LOG_ANIM_GROUP_ACTION_LOW,
                index,
                type_name,
                u32::from(group)
            ],
        );
        return 0.0;
    }

    let count = e.mem.u32(this.addr().wrapping_add(0x14));
    if index >= count {
        // 005f37d5-005f37ed: the type name is looked up here, not on entry.
        let ty = u32::from(fn_005f2440(e, group));
        let type_name = e
            .mem
            .u32(TYPE_NAME_TABLE.wrapping_add(ty.wrapping_mul(0x24)));
        e.call(
            INVALID_ACTION_LOG,
            &args![
                LOG_ANIM_GROUP_ACTION_HIGH,
                index,
                count,
                type_name,
                u32::from(group)
            ],
        );
        return 0.0;
    }

    let times = e.mem.u32(this.addr().wrapping_add(0x18));
    if times == 0 {
        return 0.0;
    }
    let time = e.mem.f32(times.wrapping_add(index.wrapping_mul(4)));
    if e.call(ISNAN, &args![f64::from(time)]).bool() {
        e.call(
            INVALID_ACTION_LOG,
            &args![LOG_ANIM_GROUP_TIME_NAN, index, u32::from(group)],
        );
        0.0
    } else {
        f64::from(time)
    }
}

// Translated from 005f3870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the next line-feed and skips consecutive CR/LF bytes; returns null
/// for a missing line-feed or at the end of the string.
pub fn fn_005f3870(e: &mut Engine, string: Ptr) -> Ptr {
    let found = e.call(STRCHR, &args![string, 10u32]).u32();
    if found == 0 {
        return Ptr::NULL;
    }
    let mut next = found;
    loop {
        let byte = e.mem.u8(next);
        if byte == 0 || (byte != b'\r' && byte != b'\n') {
            break;
        }
        next = next.wrapping_add(1);
    }
    if e.mem.u8(next) == 0 {
        Ptr::NULL
    } else {
        Ptr::new(next)
    }
}

// Translated from 005f38d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Parses move, weapon, and prefix selectors from the filename component
/// after its final backslash. Every string comparison uses the game's
/// installed tables and CRT routines.
pub fn fn_005f38d0(
    e: &mut Engine,
    filename: Ptr,
    out_move: Ptr,
    out_weapon: Ptr,
    out_prefixed: Ptr,
) {
    let base = e.call(STRRCHR, &args![filename, 0x5cu32]).u32();
    e.mem.set_u8(out_prefixed.addr(), 0);
    e.mem.set_u32(out_move.addr(), 0);
    e.mem.set_u32(out_weapon.addr(), 0);
    if base == 0 {
        return;
    }
    let mut cursor = base.wrapping_add(1);
    let prefix_len = e.call(STRLEN, &args![PA_PREFIX]).u32();
    if e.call(STRNICMP, &args![cursor, PA_PREFIX, prefix_len])
        .i32()
        == 0
    {
        e.mem.set_u8(out_prefixed.addr(), 1);
        let prefix_len = e.call(STRLEN, &args![PA_PREFIX]).u32();
        cursor = cursor.wrapping_add(prefix_len);
    }
    for index in 1..4u32 {
        let name_address = MOVE_NAME_TABLE.wrapping_add(index.wrapping_mul(4));
        let name = e.mem.u32(name_address);
        let len = e.call(STRLEN, &args![name]).u32();
        let compare_name = e.mem.u32(name_address);
        if e.call(STRNICMP, &args![cursor, compare_name, len]).i32() == 0 {
            e.mem.set_u32(out_move.addr(), index);
            let name = e.mem.u32(name_address);
            let len = e.call(STRLEN, &args![name]).u32();
            cursor = cursor.wrapping_add(len);
            break;
        }
    }
    for index in 1..12u32 {
        let name_address = WEAPON_NAME_TABLE.wrapping_add(index.wrapping_mul(4));
        let name = e.mem.u32(name_address);
        let len = e.call(STRLEN, &args![name]).u32();
        let compare_name = e.mem.u32(name_address);
        if e.call(STRNICMP, &args![cursor, compare_name, len]).i32() == 0 {
            e.mem.set_u32(out_weapon.addr(), index);
            break;
        }
    }
}

const ANIM_NAME_TABLE: u32 = 0x0119_77d8;
const ANIM_LOOP_TABLE: u32 = 0x0119_77e0;
const ANIM_CATEGORY_TABLE: u32 = 0x0119_77e4;
const ANIM_SECTION_TABLE: u32 = 0x0119_77e8;
const SECTION_OVERRIDE_TABLE: u32 = 0x0119_9ad4;
const ENUM_NAME_TABLE: u32 = 0x0119_9b50;
const NOTE_SPECIAL_PREFIX: u32 = 0x011a_8244;
const NOTE_FRAME_SCALE: u32 = 0x0101_db88;

fn anim_prefix(e: &mut Engine, text: u32, prefix: u32) -> bool {
    let length = e.call(STRLEN, &args![prefix]).u32();
    e.call(STRNICMP, &args![text, prefix, length]).i32() == 0
}

fn anim_bad_note(e: &mut Engine, note: u32, time: f32, filename: Ptr) {
    let line_end = e.call(STRCHR, &args![note, 13u32]).u32();
    if line_end != 0 {
        e.mem.set_u8(line_end, 0);
    }
    anim_bad_note_truncated(e, note, time, filename);
    if line_end != 0 {
        e.mem.set_u8(line_end, 13);
    }
}

fn anim_bad_note_truncated(e: &mut Engine, note: u32, time: f32, filename: Ptr) {
    let scale = e.global::<f64>(NOTE_FRAME_SCALE);
    let frame_time = (f64::from(time) * scale) as f32;
    // The conversion helper takes the float in one stack word; the filename
    // pushed before it remains on the stack for the subsequent log call.
    let frame = e.call(0x0040_6d90, &args![frame_time]).u32();
    e.call(
        INVALID_ACTION_LOG,
        &args![0x0104_8748u32, note, frame, filename],
    );
}

fn anim_split_field(e: &mut Engine, text: u32) -> u32 {
    let comma = e.call(STRCHR, &args![text, 44u32]).u32();
    let separator = if comma == 0 {
        e.call(STRCHR, &args![text, 32u32]).u32()
    } else {
        comma
    };
    if separator == 0 {
        0
    } else {
        e.mem.set_u8(separator, 0);
        separator.wrapping_add(1)
    }
}

#[allow(clippy::too_many_arguments)]
fn anim_sound_or_enum(
    e: &mut Engine,
    group: Ptr,
    note: u32,
    line_base: u32,
    key_index: u32,
    time: f32,
    filename: Ptr,
    is_enum: bool,
    frame: u32,
) -> bool {
    if group.is_null() {
        return false;
    }
    let line_end = e.call(STRCHR, &args![note, 13u32]).u32();
    if line_end != 0 {
        e.mem.set_u8(line_end, 0);
    }
    let prefix = if is_enum { 0x0104_877c } else { 0x0102_38a8 };
    let value_start = note.wrapping_add(e.call(STRLEN, &args![prefix]).u32());
    let value_buffer = frame.wrapping_add(0x350);
    e.call(0x0040_6d30, &args![value_buffer, 0x104u32, value_start]);
    let mut second = anim_split_field(e, value_buffer);
    while second != 0 && e.mem.u8(second) == b' ' {
        second = second.wrapping_add(1);
    }
    let third = if second != 0 {
        anim_split_field(e, second)
    } else {
        0
    };
    let mut bad = false;
    if is_enum {
        let mut enum_index = 0u32;
        while enum_index < 0x1f {
            let name = e.mem.u32(ENUM_NAME_TABLE + enum_index * 4);
            if e.call(0x0040_4dc0, &args![name, value_buffer]).i32() == 0 {
                break;
            }
            enum_index += 1;
        }
        if enum_index == 0x1f {
            bad = true;
        } else {
            let index = e.call(0x005f_36f0, &args![group]).u32();
            fn_005f2820(e, group, index.wrapping_add(1));
            fn_005f29f0(e, group, index, time);
            let packed = note.wrapping_sub(line_base).wrapping_shl(8) | key_index;
            fn_005f2a70(e, group, index, enum_index as u8, packed);
        }
    } else {
        let handler = e.global::<u32>(0x011c_3f2c);
        let sound = e.call(0x0046_16c0, &args![handler, value_buffer]).u32();
        if sound == 0 {
            bad = true;
        } else {
            let audio_value = e.call(0x0084_e3a0, &args![sound]).u32();
            let audio = e.call(0x0045_3a70, &args![]).u32();
            e.call(0x00ad_8050, &args![audio, audio_value, 0u32]);
            let index = e.call(0x005f_36f0, &args![group]).u32();
            fn_005f2820(e, group, index.wrapping_add(1));
            fn_005f29f0(e, group, index, time);
            fn_005f2a30(e, group, index, sound);
            if second != 0 && e.mem.u8(second) != 0 {
                let number = e.call(0x00ec_a573, &args![second]).f64() as f32;
                fn_005f2ac0(e, group, index, number);
            }
            if third != 0 {
                let number = e.call(0x00ec_a573, &args![third]).f64() as f32;
                fn_005f2b20(e, group, index, number.to_bits());
            }
        }
    }
    if bad {
        anim_bad_note_truncated(e, note, time, filename);
    }
    if line_end != 0 {
        e.mem.set_u8(line_end, 13);
    }
    bad
}

fn anim_read_number_tag(e: &mut Engine, group: Ptr, text: u32) -> bool {
    for (prefix, length, target) in [
        (0x0104_87b0u32, 8u32, 0x007f_7c90u32),
        (0x0104_87a4, 9, 0x005f_4b90),
        (0x0104_879c, 6, 0x005f_4b70),
        (0x0104_8794, 6, 0x008b_f4e0),
    ] {
        if e.call(STRNICMP, &args![text, prefix, length]).i32() == 0 {
            let mut number = text.wrapping_add(length);
            if e.mem.u8(number) == b' ' {
                number = number.wrapping_add(1);
            }
            let value = e.call(0x00ec_a6d3, &args![number]).u32();
            if target == 0x005f_4b90 {
                fn_005f4b90(e, group, value as u8);
            } else if target == 0x005f_4b70 {
                fn_005f4b70(e, group, value as u8);
            } else {
                e.call(target, &args![group, value]);
            }
            return true;
        }
    }
    false
}

fn anim_parent_note(e: &mut Engine, group: Ptr, note_string: u32) {
    let inspected = e.call(0x0043_b1b0, &args![note_string]).u32();
    let offset = if e.mem.u8(inspected + 4) == b' ' {
        5
    } else {
        4
    };
    let value = e
        .call(0x0043_b1b0, &args![note_string])
        .u32()
        .wrapping_add(offset);
    fn_005f4bb0(e, group, value);
}

fn anim_time_section(e: &Engine, group_type: u32, section: u32) -> u32 {
    e.mem
        .u32(ANIM_SECTION_TABLE + group_type * 0x24 + section * 4)
}

#[allow(clippy::too_many_arguments)]
fn anim_timing_note(
    e: &mut Engine,
    sequence: Ptr,
    filename: Ptr,
    note: u32,
    time: f32,
    move_type: u32,
    weapon: u32,
    prefixed: u8,
    group: &mut Ptr,
    cached_type: &mut u32,
    section: &mut u32,
) -> (bool, bool) {
    let mut ty = 0u32;
    while ty < 0xf5 {
        if *cached_type != 0xff {
            ty = *cached_type;
        }
        let type_name = e.mem.u32(ANIM_NAME_TABLE + ty * 0x24);
        let name = e.call(0x0041_3f40, &args![sequence]).u32();
        let text = e.call(0x0043_b1b0, &args![name]).u32();
        if e.call(0x0040_4dc0, &args![type_name, text]).i32() == 0 {
            *cached_type = ty;
            let mut section_id = anim_time_section(e, ty, *section);
            if section_id == u32::MAX {
                return (false, false);
            }
            let category = e.mem.u32(ANIM_CATEGORY_TABLE + ty * 0x24);
            if (ty == 2 && *section == 1) || (category == 5 && *section == 2) {
                let override_name = e.mem.u32(SECTION_OVERRIDE_TABLE + category * 4);
                if anim_prefix(e, note, override_name) {
                    section_id = 3;
                    *section = 3;
                }
            }
            let expected = e
                .mem
                .u32(SECTION_NAME_TABLE + section_id * 0x2c + category * 4);
            if anim_prefix(e, note, expected) {
                if group.is_null() {
                    let allocated = e.call(0x00aa_13e0, &args![0x3cu32]).u32();
                    if allocated != 0 {
                        let group_id = if ty == 2 {
                            ty
                        } else {
                            fn_005f2370(e, move_type, weapon as u16, ty, prefixed)
                        };
                        *group = fn_005f21c0(e, Ptr::new(allocated), group_id as u16);
                    }
                }
                if time > -1.0 {
                    fn_005f3740(e, *group, section_id, time);
                }
                let looping = e.call(0x0059_bb30, &args![sequence]).bool();
                if category == 1 && looping {
                    if e.mem.u32(ANIM_LOOP_TABLE + ty * 0x24) != 1 {
                        e.call(INVALID_ACTION_LOG, &args![0x0104_8718u32, filename]);
                        *section += 1;
                        return (false, true);
                    }
                } else if category != 1 && !looping {
                    e.call(INVALID_ACTION_LOG, &args![0x0104_86e4u32, filename]);
                    *section += 1;
                    return (false, true);
                }
                *section += 1;
                return (false, false);
            }
            let next = anim_time_section(e, ty, *section);
            if *section == 5 || next == u32::MAX {
                anim_bad_note(e, note, time, filename);
                return (false, true);
            }
            return (false, false);
        }
        if ty == *cached_type {
            break;
        }
        ty += 1;
    }
    if ty == 0xf5 {
        let name = e.call(0x0041_3f40, &args![sequence]).u32();
        let text = e.call(0x0043_b1b0, &args![name]).u32();
        e.call(INVALID_ACTION_LOG, &args![0x0104_86a0u32, text, filename]);
        return (true, true);
    }
    (false, false)
}

fn anim_validate(e: &mut Engine, group: Ptr, sequence: Ptr, filename: Ptr) -> bool {
    fn_005f4d00(e, group, sequence);
    let ty = fn_005f2420(e, group);
    let category = e.mem.u32(ANIM_CATEGORY_TABLE + ty * 0x24);
    let mut last = 1u32;
    match category {
        2 => {
            let one = fn_005f3780(e, group, 1);
            let zero = fn_005f3780(e, group, 0);
            if one < zero {
                fn_005f3740(e, group, 1, zero as f32);
            }
            let two = fn_005f3780(e, group, 2);
            let one = fn_005f3780(e, group, 1);
            // 005f4a1f TEST AH,0x1: the time at 2 moves when it is not above the time at 1.
            if two <= one {
                let three = fn_005f3780(e, group, 3);
                fn_005f3740(e, group, 2, three as f32);
            }
            last = 3;
        }
        3 | 4 => last = 2,
        5 => {
            let two = fn_005f3780(e, group, 2);
            let one = fn_005f3780(e, group, 1);
            if two < one {
                fn_005f3740(e, group, 2, one as f32);
            }
            last = 4;
        }
        6 | 10 => last = 2,
        7 => last = 4,
        8 | 9 => last = 3,
        _ => {}
    }
    let end = fn_005f3780(e, group, last);
    let start = fn_005f3780(e, group, 0);
    let mut bad = false;
    if end <= start {
        let current_type = fn_005f2420(e, group);
        let name = e.mem.u32(ANIM_NAME_TABLE + current_type * 0x24);
        e.call(INVALID_ACTION_LOG, &args![0x0104_8658u32, name, filename]);
        bad = true;
    }
    let blend = e.mem.u8(group.addr() + 0x29);
    let blend_in = e.mem.u8(group.addr() + 0x2a);
    let blend_out = e.mem.u8(group.addr() + 0x2b);
    if blend != 0 && (blend_in != 0 || blend_out != 0) {
        let current_type = fn_005f2420(e, group);
        let name = e.mem.u32(ANIM_NAME_TABLE + current_type * 0x24);
        e.call(
            INVALID_ACTION_LOG,
            &args![0x0104_8600u32, name, filename, blend, blend_in, blend_out],
        );
        bad = true;
    }
    bad
}

// Translated from 005f3a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAnimGroup::LoadAnimGroup` (Xbox PDB): scans the sequence's object names,
/// parses note keys into timing, sound and enum entries, validates the group,
/// and returns its allocated pointer. Compiler SEH unwinding is not modeled.
// Translation incomplete: 005f3a20 - compiler SEH unwinding is not modeled.
pub fn fn_005f3a20(e: &mut Engine, sequence: Ptr, filename: Ptr) -> Ptr {
    e.with_stack(0x500, |e, frame| {
        e.call(STRRCHR, &args![filename, 0x5cu32]);
        e.call(0x0043_b2b0, &args![1u32]);
        let model = e.call(0x006c_9900, &args![sequence]).u32();
        let model_name = e.call(0x0043_b1b0, &args![model]).u32();
        let non_accum_name = frame.addr();
        e.call(
            0x0040_6d00,
            &args![non_accum_name, 0x40u32, 0x0104_884cu32, model_name],
        );
        let mut model_index = u32::MAX;
        let mut non_accum_index = u32::MAX;
        let object_name = frame.addr() + 0x80;
        let mut index = 0u32;
        loop {
            let count = e.call(0x0084_e3a0, &args![sequence]).u32();
            if index >= count {
                break;
            }
            e.call(0x00a3_07c0, &args![sequence, index, object_name]);
            if e.mem.u8(object_name) == 0 {
                let name = e.call(0x0041_3f40, &args![sequence]).u32();
                let text = e.call(0x0043_b1b0, &args![name]).u32();
                e.call(INVALID_ACTION_LOG, &args![0x0104_881cu32, index, text]);
            } else if e.call(0x0040_4dc0, &args![model_name, object_name]).i32() == 0 {
                model_index = index;
            } else if e
                .call(0x0040_4dc0, &args![non_accum_name, object_name])
                .i32()
                == 0
            {
                non_accum_index = index;
            }
            if model_index != u32::MAX && non_accum_index != u32::MAX {
                let sound = fn_005f4c00(e, sequence, model_index);
                fn_005f4bd0(e, sequence, non_accum_index, sound);
                break;
            }
            index += 1;
        }
        let out_move = frame.addr() + 0x190;
        let out_weapon = out_move + 4;
        let out_prefixed = out_move + 8;
        fn_005f38d0(
            e,
            filename,
            Ptr::new(out_move),
            Ptr::new(out_weapon),
            Ptr::new(out_prefixed),
        );
        let mut group = Ptr::NULL;
        let mut bad = false;
        if !sequence.is_null() && e.call(0x0070_0300, &args![sequence]).u32() != 0 {
            let note_out = frame.addr() + 0x1a0;
            let notes = e.call(0x0070_0300, &args![sequence]).u32();
            let note_keys = e.call(0x0049_5f30, &args![notes, note_out]).u32();
            let note_count = e.mem.u32(note_out);
            let mut cached_type = 0xffu32;
            let mut section = 0u32;
            let mut stop_notes = false;
            for key_index in 0..note_count {
                let key = note_keys + key_index * 8;
                let handle = e.call(0x0071_7e50, &args![key]).u32();
                let note_string = frame.addr() + 0x1b0;
                e.call(0x0048_af40, &args![note_string, handle]);
                let time = e.call(0x006a_7f50, &args![key]).f64() as f32;
                if !e.call(0x0048_af70, &args![note_string]).bool() {
                    let name = e.call(0x0041_3f40, &args![sequence]).u32();
                    let text = e.call(0x0043_b1b0, &args![name]).u32();
                    e.call(
                        INVALID_ACTION_LOG,
                        &args![0x0104_87c8u32, f64::from(time), text, filename],
                    );
                    bad = true;
                } else {
                    let compare_source = e.call(0x0043_b1b0, &args![note_string]).u32();
                    // 005f3c8c-005f3c9f: the parent prefix (010487bc) is compared with the
                    // constant length 4 (PUSH 0x4); no strlen call is made for it.
                    if e.call(STRNICMP, &args![compare_source, 0x0104_87bcu32, 4u32])
                        .i32()
                        == 0
                    {
                        anim_parent_note(e, group, note_string);
                    } else {
                        let source = e.call(0x0043_b1b0, &args![note_string]).u32();
                        let copied = frame.addr() + 0x1c0;
                        e.call(0x0040_6d30, &args![copied, 0x104u32, source]);
                        let mut note = copied;
                        while note != 0 && e.call(STRLEN, &args![note]).u32() != 0 {
                            // 005f3d59-005f3d6d: a leading CR skips to the next
                            // line, and the result reaches the _strnicmp at
                            // 005f3d90 unchecked, a null pointer included. A null
                            // there (a CR with no line feed after it) ends the
                            // process: 00ec7ec0 reports it through 00ec7c30, and
                            // with no handler installed that goes to
                            // __invoke_watson (00ec7b08).
                            if e.mem.u8(note) == 13 {
                                note = fn_005f3870(e, Ptr::new(note)).addr();
                            }
                            let special = e.global::<u32>(NOTE_SPECIAL_PREFIX);
                            if anim_prefix(e, note, special) {
                                if !e.call(0x0097_4d90, &args![group]).bool() {
                                    e.call(0x0092_9260, &args![group, e.mem.u8(note + 2)]);
                                }
                            } else if !anim_read_number_tag(e, group, note) {
                                let is_sound = anim_prefix(e, note, 0x0104_878c);
                                let is_enum = anim_prefix(e, note, 0x0104_8784);
                                if is_sound || is_enum {
                                    bad |= anim_sound_or_enum(
                                        e,
                                        group,
                                        note,
                                        copied,
                                        key_index,
                                        time,
                                        filename,
                                        is_enum,
                                        frame.addr(),
                                    );
                                } else {
                                    let (stop, note_bad) = anim_timing_note(
                                        e,
                                        sequence,
                                        filename,
                                        note,
                                        time,
                                        e.mem.u32(out_move),
                                        e.mem.u32(out_weapon),
                                        e.mem.u8(out_prefixed),
                                        &mut group,
                                        &mut cached_type,
                                        &mut section,
                                    );
                                    bad |= note_bad;
                                    if stop {
                                        stop_notes = true;
                                        break;
                                    }
                                }
                            }
                            // 005f4881 CMP [EBP-0x3b8],0 / JZ 005f489f: only a
                            // non-null note advances to the next line.
                            if note != 0 {
                                note = fn_005f3870(e, Ptr::new(note)).addr();
                            }
                        }
                    }
                }
                e.call(0x0043_81b0, &args![note_string]);
                if stop_notes {
                    break;
                }
            }
            if !group.is_null() {
                bad |= anim_validate(e, group, sequence, filename);
            }
        }
        e.call(0x0043_b2b0, &args![0u32]);
        if bad {
            let path = e.call(0x00c3_bb80, &args![]).u32();
            e.call(INVALID_ACTION_LOG, &args![0x0104_85c8u32, path]);
        }
        group
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ret;

    #[test]
    fn loader_scans_sequence_then_returns_null_without_note_keys() {
        let mut e = Engine::new();
        e.map(0x2000, 0x100);
        e.map(0x3000, 0x20);
        e.map(0x3100, 0x20);
        e.mem.set_u8(0x3000, b'a');
        e.mem.set_u8(0x3100, b'm');
        e.register_double(STRRCHR, |_, arguments| {
            assert_eq!(arguments, [0x3000, 0x5c]);
            Ret::default()
        });
        e.register_double(0x0043_b2b0, |_, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret::default()
        });
        e.register_double(0x006c_9900, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret {
                eax: 0x2100,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_b1b0, |_, arguments| {
            assert_eq!(arguments, [0x2100]);
            Ret {
                eax: 0x3100,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d00, |e, arguments| {
            assert_eq!(arguments.len(), 4);
            assert_eq!(arguments[1..], [0x40, 0x0104_884c, 0x3100]);
            e.mem.set_u8(arguments[0], 0);
            Ret::default()
        });
        e.register_double(0x0084_e3a0, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret::default()
        });
        e.register_double(0x0070_0300, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret {
                eax: 0x4000,
                ..Ret::default()
            }
        });
        e.register_double(0x0049_5f30, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[0], 0x4000);
            e.mem.set_u32(arguments[1], 0);
            Ret {
                eax: 0x5000,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_3a20, &[0x2000, 0x3000]).u32(), 0);
        let calls = e.call_log.take().unwrap();
        let toggles: Vec<_> = calls
            .iter()
            .filter(|(address, _)| *address == 0x0043_b2b0)
            .map(|(_, arguments)| arguments[0])
            .collect();
        assert_eq!(toggles, [1, 0]);
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == STRRCHR)
                .count(),
            2
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0070_0300)
                .count(),
            2
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0049_5f30)
                .count(),
            1
        );
    }

    #[test]
    fn loader_reads_note_callback_and_ignores_sound_before_group_allocation() {
        let mut e = Engine::new();
        e.map(0x2000, 0x100);
        e.map(0x3000, 0x20);
        e.map(0x3100, 0x20);
        e.map(0x5000, 8);
        e.map(0x6100, 16);
        e.map(0x6200, 4);
        e.map(NOTE_SPECIAL_PREFIX, 4);
        e.mem.set_u32(0x5004, 0x6100);
        e.mem.set_u32(NOTE_SPECIAL_PREFIX, 0x9000);
        for (index, byte) in b"Sound: x\0".iter().enumerate() {
            e.mem.set_u8(0x6100 + index as u32, *byte);
        }
        e.register_double(STRRCHR, |_, arguments| {
            assert_eq!(arguments, [0x3000, 0x5c]);
            Ret::default()
        });
        e.register_double(0x0043_b2b0, |_, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret::default()
        });
        e.register_double(0x006c_9900, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret {
                eax: 0x2100,
                ..Ret::default()
            }
        });
        let mut note_getter_calls = 0;
        e.register_double(0x0043_b1b0, move |_, arguments| {
            let pointer = if arguments[0] == 0x2100 {
                0x3100
            } else {
                note_getter_calls += 1;
                if note_getter_calls == 1 {
                    0x6200
                } else {
                    0x6100
                }
            };
            Ret {
                eax: pointer,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d00, |e, arguments| {
            assert_eq!(arguments.len(), 4);
            e.mem.set_u8(arguments[0], 0);
            Ret::default()
        });
        e.register_double(0x0084_e3a0, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret::default()
        });
        e.register_double(0x0070_0300, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret {
                eax: 0x4000,
                ..Ret::default()
            }
        });
        e.register_double(0x0049_5f30, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[0], 0x4000);
            e.mem.set_u32(arguments[1], 1);
            Ret {
                eax: 0x5000,
                ..Ret::default()
            }
        });
        e.register_double(0x0071_7e50, |_, arguments| {
            assert_eq!(arguments, [0x5000]);
            Ret {
                eax: 0x5004,
                ..Ret::default()
            }
        });
        e.register_double(0x0048_af40, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[1], 0x5004);
            e.mem.set_u32(arguments[0], e.mem.u32(arguments[1]));
            Ret::default()
        });
        e.register_double(0x006a_7f50, |_, arguments| {
            assert_eq!(arguments, [0x5000]);
            Ret {
                st0: 1.25,
                ..Ret::default()
            }
        });
        e.register_double(0x0048_af70, |e, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret {
                eax: u32::from(e.mem.u32(arguments[0]) != 0),
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |e, arguments| {
            assert_eq!(arguments.len(), 3);
            assert_eq!(arguments[2], 0x6100);
            let mut index = 0;
            loop {
                let byte = e.mem.u8(arguments[2] + index);
                e.mem.set_u8(arguments[0] + index, byte);
                if byte == 0 {
                    break;
                }
                index += 1;
            }
            Ret::default()
        });
        e.register_double(STRLEN, |e, arguments| {
            let length = match arguments[0] {
                0x9000 => 3,
                0x0104_878c => 6,
                0x0104_8784 => 5,
                address => {
                    let mut size = 0;
                    while e.mem.u8(address + size) != 0 {
                        size += 1;
                    }
                    size
                }
            };
            Ret {
                eax: length,
                ..Ret::default()
            }
        });
        e.register_double(STRNICMP, |_, arguments| {
            assert_eq!(arguments.len(), 3);
            Ret {
                eax: u32::from(arguments[1] != 0x0104_878c),
                ..Ret::default()
            }
        });
        e.register_double(STRCHR, |_, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[1], 10);
            Ret::default()
        });
        e.register_double(0x0043_81b0, |_, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_3a20, &[0x2000, 0x3000]).u32(), 0);
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0071_7e50)
                .count(),
            1
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0043_81b0)
                .count(),
            1
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, arguments)| *address == 0x0043_b1b0 && arguments[0] != 0x2100)
                .count(),
            2
        );
        assert_eq!(e.mem.u8(0x6100), b'S');
    }

    #[test]
    fn lone_cr_note_passes_null_to_the_special_compare_which_ends_the_process() {
        let mut e = Engine::new();
        e.map(0x2000, 0x100);
        e.map(0x3000, 0x20);
        e.map(0x3100, 0x20);
        e.map(0x5000, 8);
        e.map(0x6100, 16);
        e.map(0x6200, 4);
        e.map(NOTE_SPECIAL_PREFIX, 4);
        e.mem.set_u32(0x5004, 0x6100);
        e.mem.set_u32(NOTE_SPECIAL_PREFIX, 0x9000);
        // A lone CR: 005f3870 finds no line feed and returns null.
        e.mem.set_u8(0x6100, b'\r');
        e.mem.set_u8(0x6101, 0);
        e.register_double(STRRCHR, |_, arguments| {
            assert_eq!(arguments, [0x3000, 0x5c]);
            Ret::default()
        });
        e.register_double(0x0043_b2b0, |_, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret::default()
        });
        e.register_double(0x006c_9900, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret {
                eax: 0x2100,
                ..Ret::default()
            }
        });
        let mut note_getter_calls = 0;
        e.register_double(0x0043_b1b0, move |_, arguments| {
            let pointer = if arguments[0] == 0x2100 {
                0x3100
            } else {
                note_getter_calls += 1;
                if note_getter_calls == 1 {
                    0x6200
                } else {
                    0x6100
                }
            };
            Ret {
                eax: pointer,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d00, |e, arguments| {
            assert_eq!(arguments.len(), 4);
            e.mem.set_u8(arguments[0], 0);
            Ret::default()
        });
        e.register_double(0x0084_e3a0, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret::default()
        });
        e.register_double(0x0070_0300, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret {
                eax: 0x4000,
                ..Ret::default()
            }
        });
        e.register_double(0x0049_5f30, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[0], 0x4000);
            e.mem.set_u32(arguments[1], 1);
            Ret {
                eax: 0x5000,
                ..Ret::default()
            }
        });
        e.register_double(0x0071_7e50, |_, arguments| {
            assert_eq!(arguments, [0x5000]);
            Ret {
                eax: 0x5004,
                ..Ret::default()
            }
        });
        e.register_double(0x0048_af40, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[1], 0x5004);
            e.mem.set_u32(arguments[0], e.mem.u32(arguments[1]));
            Ret::default()
        });
        e.register_double(0x006a_7f50, |_, arguments| {
            assert_eq!(arguments, [0x5000]);
            Ret {
                st0: 1.25,
                ..Ret::default()
            }
        });
        e.register_double(0x0048_af70, |e, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret {
                eax: u32::from(e.mem.u32(arguments[0]) != 0),
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |e, arguments| {
            assert_eq!(arguments.len(), 3);
            assert_eq!(arguments[2], 0x6100);
            let mut index = 0;
            loop {
                let byte = e.mem.u8(arguments[2] + index);
                e.mem.set_u8(arguments[0] + index, byte);
                if byte == 0 {
                    break;
                }
                index += 1;
            }
            Ret::default()
        });
        e.register_double(STRLEN, |e, arguments| {
            let length = match arguments[0] {
                0x9000 => 3,
                address => {
                    let mut size = 0;
                    while e.mem.u8(address + size) != 0 {
                        size += 1;
                    }
                    size
                }
            };
            Ret {
                eax: length,
                ..Ret::default()
            }
        });
        e.register_double(STRNICMP, |_, arguments| {
            assert_eq!(arguments.len(), 3);
            // 00ec7ed0: a null string sets errno to EINVAL (00ec7edf) and calls
            // 00ec7c30. No handler is ever installed (the only store to 0126fd9c,
            // 00ec7b01, is the encoded null __init_pointers passes at 00eccf26), so
            // 00ec7c30 goes on to __invoke_watson (00ec7b08), which ends the process;
            // 00ec7eed (0x7fffffff) is not reached.
            if arguments[0] == 0 {
                panic!("_strnicmp(NULL): __invoke_watson ends the process");
            }
            // 005f3c8c-005f3c9f: the empty line at 0x6200 against the parent prefix
            // (010487bc) with length 4; __ascii_strnicmp (00ed5bc0) stops at the
            // terminator and answers -1 (00ed5c11).
            assert_eq!(arguments, [0x6200, 0x0104_87bc, 4]);
            Ret {
                eax: u32::MAX,
                ..Ret::default()
            }
        });
        e.register_double(STRCHR, |_, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[1], 10);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let ended = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x005f_3a20, &[0x2000, 0x3000])
        }));
        assert!(ended.is_err());
        let calls = e.call_log.take().unwrap();
        let note_string = calls
            .iter()
            .find(|(address, _)| *address == 0x0048_af40)
            .unwrap()
            .1[0];
        let copied = calls
            .iter()
            .find(|(address, _)| *address == 0x0040_6d30)
            .unwrap()
            .1[0];
        let start = calls
            .iter()
            .position(|(address, _)| *address == 0x0048_af70)
            .unwrap();
        let expected: [(u32, Vec<u32>); 9] = [
            (0x0048_af70, vec![note_string]),
            // 005f3c8c-005f3c9f: the parent prefix compare, no strlen before it.
            (0x0043_b1b0, vec![note_string]),
            (STRNICMP, vec![0x6200, 0x0104_87bc, 4]),
            (0x0043_b1b0, vec![note_string]),
            (0x0040_6d30, vec![copied, 0x104, 0x6100]),
            // 005f3d40: the copied line is not empty.
            (STRLEN, vec![copied]),
            // 005f3d65: 005f3870 finds no line feed after the CR and returns null.
            (STRCHR, vec![copied, 10]),
            // 005f3d7a-005f3d90: the null note goes to _strnicmp unchecked, and the
            // process ends there: the blend branch (005f3d9c), the number-tag
            // compares (005f3dc3) and the string's destructor (004381b0) never run.
            (STRLEN, vec![0x9000]),
            (STRNICMP, vec![0, 0x9000, 3]),
        ];
        assert_eq!(calls[start..], expected);
    }

    #[test]
    fn empty_note_log_keeps_filename_and_two_word_time_argument() {
        let mut e = Engine::new();
        e.map(0x2000, 0x100);
        e.map(0x3000, 0x20);
        e.register_double(STRRCHR, |_, arguments| {
            assert_eq!(arguments, [0x3000, 0x5c]);
            Ret::default()
        });
        e.register_double(0x0043_b2b0, |_, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret::default()
        });
        e.register_double(0x006c_9900, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret {
                eax: 0x2100,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_b1b0, |_, arguments| Ret {
            eax: if arguments[0] == 0x2100 {
                0x3100
            } else {
                0x3200
            },
            ..Ret::default()
        });
        e.register_double(0x0040_6d00, |e, arguments| {
            assert_eq!(arguments.len(), 4);
            e.mem.set_u8(arguments[0], 0);
            Ret::default()
        });
        e.register_double(0x0084_e3a0, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret::default()
        });
        e.register_double(0x0070_0300, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret {
                eax: 0x4000,
                ..Ret::default()
            }
        });
        e.register_double(0x0049_5f30, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[0], 0x4000);
            e.mem.set_u32(arguments[1], 1);
            Ret {
                eax: 0x5000,
                ..Ret::default()
            }
        });
        e.register_double(0x0071_7e50, |_, arguments| {
            assert_eq!(arguments, [0x5000]);
            Ret {
                eax: 0x5004,
                ..Ret::default()
            }
        });
        e.register_double(0x0048_af40, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            e.mem.set_u32(arguments[0], 0);
            Ret::default()
        });
        e.register_double(0x006a_7f50, |_, arguments| {
            assert_eq!(arguments, [0x5000]);
            Ret {
                st0: 1.5,
                ..Ret::default()
            }
        });
        e.register_double(0x0048_af70, |_, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret::default()
        });
        e.register_double(0x0041_3f40, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret {
                eax: 0x2008,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_81b0, |_, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret::default()
        });
        e.register_double(0x00c3_bb80, |_, arguments| {
            assert!(arguments.is_empty());
            Ret {
                eax: 0x3300,
                ..Ret::default()
            }
        });
        e.register_double(INVALID_ACTION_LOG, |_, arguments| {
            if arguments[0] == 0x0104_87c8 {
                assert_eq!(arguments.len(), 5);
                let time_words = 1.5f64.to_bits();
                assert_eq!(arguments[1], time_words as u32);
                assert_eq!(arguments[2], (time_words >> 32) as u32);
                assert_eq!(arguments[3..], [0x3200, 0x3000]);
            } else {
                assert_eq!(arguments, [0x0104_85c8, 0x3300]);
            }
            Ret::default()
        });
        assert_eq!(e.call(0x005f_3a20, &[0x2000, 0x3000]).u32(), 0);
    }

    #[test]
    fn timing_note_stores_time_and_advances_section_after_a_name_match() {
        let mut e = Engine::new();
        e.map(0x2000, 0x3c);
        e.map(0x3000, 0x100);
        e.map(0x4000, 0x20);
        e.map(0x4010, 0x20);
        e.map(ANIM_NAME_TABLE, 0x24);
        e.map(SECTION_NAME_TABLE, 0x2c);
        e.mem.set_u32(0x2014, 4);
        e.mem.set_u32(0x2018, 0x3000);
        e.mem.set_u32(ANIM_NAME_TABLE, 0x4000);
        e.mem.set_u32(ANIM_CATEGORY_TABLE, 2);
        e.mem.set_u32(ANIM_SECTION_TABLE, 0);
        e.mem.set_u32(SECTION_NAME_TABLE + 8, 0x4010);
        for (address, value) in [
            (0x4000, b'T'),
            (0x4010, b'S'),
            (0x4011, b't'),
            (0x4012, b'a'),
            (0x4013, b'r'),
            (0x4014, b't'),
        ] {
            e.mem.set_u8(address, value);
        }
        e.register_double(0x0041_3f40, |_, arguments| {
            assert_eq!(arguments, [0x5000]);
            Ret {
                eax: 0x5008,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_b1b0, |_, arguments| {
            assert_eq!(arguments, [0x5008]);
            Ret {
                eax: 0x4000,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_4dc0, |_, arguments| {
            assert_eq!(arguments, [0x4000, 0x4000]);
            Ret::default()
        });
        e.register_double(STRLEN, |_, arguments| {
            assert_eq!(arguments, [0x4010]);
            Ret {
                eax: 5,
                ..Ret::default()
            }
        });
        e.register_double(STRNICMP, |_, arguments| {
            assert_eq!(arguments, [0x6000, 0x4010, 5]);
            Ret::default()
        });
        e.register_double(0x0059_bb30, |_, arguments| {
            assert_eq!(arguments, [0x5000]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let mut group = Ptr::new(0x2000);
        let mut cached = 0xff;
        let mut section = 0;
        assert_eq!(
            anim_timing_note(
                &mut e,
                Ptr::new(0x5000),
                Ptr::new(0x7000),
                0x6000,
                2.5,
                0,
                0,
                0,
                &mut group,
                &mut cached,
                &mut section,
            ),
            (false, false)
        );
        assert_eq!(group.addr(), 0x2000);
        assert_eq!(cached, 0);
        assert_eq!(section, 1);
        assert_eq!(e.mem.f32(0x3000), 2.5);
    }

    #[test]
    fn bad_sound_note_splits_the_name_and_reports_the_original_line() {
        let mut e = Engine::new();
        let note = b"Sound: Missing, 0.7, 2.0\r\n\0";
        let cr_address = 0x6000 + note.iter().position(|byte| *byte == b'\r').unwrap() as u32;
        e.map(0x2000, 0x3c);
        e.map(0x5000, 0x500);
        e.map(0x6000, note.len() as u32);
        e.map(0x011c_3f2c, 4);
        e.map(NOTE_FRAME_SCALE, 8);
        e.mem.set_u32(0x011c_3f2c, 0x8000);
        e.set_global(NOTE_FRAME_SCALE, 30.0f64);
        for (index, byte) in note.iter().enumerate() {
            e.mem.set_u8(0x6000 + index as u32, *byte);
        }
        e.register_double(STRCHR, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            let mut address = arguments[0];
            while e.mem.u8(address) != 0 {
                if e.mem.u8(address) == arguments[1] as u8 {
                    return Ret {
                        eax: address,
                        ..Ret::default()
                    };
                }
                address += 1;
            }
            Ret::default()
        });
        e.register_double(STRLEN, |_, arguments| {
            assert_eq!(arguments, [0x0102_38a8]);
            Ret {
                eax: 7,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |e, arguments| {
            assert_eq!(arguments.len(), 3);
            assert_eq!(arguments[1], 0x104);
            let mut index = 0;
            loop {
                let byte = e.mem.u8(arguments[2] + index);
                e.mem.set_u8(arguments[0] + index, byte);
                if byte == 0 {
                    break;
                }
                index += 1;
            }
            Ret::default()
        });
        e.register_double(0x0046_16c0, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[0], 0x8000);
            assert_eq!(e.mem.u8(arguments[1]), b'M');
            assert_eq!(e.mem.u8(arguments[1] + 7), 0);
            Ret::default()
        });
        e.register_double(0x0040_6d90, |_, arguments| {
            assert_eq!(arguments, [37.5f32.to_bits()]);
            Ret {
                eax: 38,
                ..Ret::default()
            }
        });
        e.register_double(INVALID_ACTION_LOG, move |e, arguments| {
            assert_eq!(arguments, [0x0104_8748, 0x6000, 38, 0x7000]);
            assert_eq!(e.mem.u8(cr_address), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(anim_sound_or_enum(
            &mut e,
            Ptr::new(0x2000),
            0x6000,
            0x6000,
            0,
            1.25,
            Ptr::new(0x7000),
            false,
            0x5000,
        ));
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == STRCHR)
                .count(),
            3
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0046_16c0)
                .count(),
            1
        );
        assert_eq!(e.mem.u8(cr_address), b'\r');
    }

    #[test]
    fn valid_sound_note_precaches_and_stores_time_priority_and_weight() {
        let mut e = Engine::new();
        let note = b"Sound: Snd, 0.5, 1.25\0";
        e.map(0x2000, 0x3c);
        e.map(0x3000, 0x10);
        e.map(0x5000, 0x500);
        e.map(0x6000, note.len() as u32);
        e.map(0x011c_3f2c, 4);
        e.map(NOTE_PRIORITY_SCALE, 8);
        e.mem.set_u32(0x011c_3f2c, 0x7000);
        e.set_global(NOTE_PRIORITY_SCALE, 250.0f64);
        for (index, byte) in note.iter().enumerate() {
            e.mem.set_u8(0x6000 + index as u32, *byte);
        }
        e.register_double(STRCHR, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            let mut address = arguments[0];
            while e.mem.u8(address) != 0 {
                if e.mem.u8(address) == arguments[1] as u8 {
                    return Ret {
                        eax: address,
                        ..Ret::default()
                    };
                }
                address += 1;
            }
            Ret::default()
        });
        e.register_double(STRLEN, |_, arguments| {
            assert_eq!(arguments, [0x0102_38a8]);
            Ret {
                eax: 7,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |e, arguments| {
            assert_eq!(arguments.len(), 3);
            let mut index = 0;
            loop {
                let byte = e.mem.u8(arguments[2] + index);
                e.mem.set_u8(arguments[0] + index, byte);
                if byte == 0 {
                    break;
                }
                index += 1;
            }
            Ret::default()
        });
        e.register_double(0x0046_16c0, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[0], 0x7000);
            assert_eq!(e.mem.u8(arguments[1]), b'S');
            assert_eq!(e.mem.u8(arguments[1] + 3), 0);
            Ret {
                eax: 0x8000,
                ..Ret::default()
            }
        });
        e.register_double(0x0084_e3a0, |_, arguments| {
            assert_eq!(arguments, [0x8000]);
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        e.register_double(0x0045_3a70, |_, arguments| {
            assert!(arguments.is_empty());
            Ret {
                eax: 0xa000,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_8050, |_, arguments| {
            assert_eq!(arguments, [0xa000, 0x9000, 0]);
            Ret::default()
        });
        e.register_double(0x005f_36f0, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret::default()
        });
        e.register_double(OPERATOR_NEW, |_, arguments| {
            assert_eq!(arguments, [0x10]);
            Ret {
                eax: 0x3000,
                ..Ret::default()
            }
        });
        e.register_double(0x00ec_a573, |e, arguments| {
            assert_eq!(arguments.len(), 1);
            Ret {
                st0: if e.mem.u8(arguments[0]) == b'0' {
                    0.5
                } else {
                    1.25
                },
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert!(!anim_sound_or_enum(
            &mut e,
            Ptr::new(0x2000),
            0x6000,
            0x6000,
            0,
            1.5,
            Ptr::new(0x7000),
            false,
            0x5000,
        ));
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x00ad_8050)
                .count(),
            1
        );
        assert_eq!(e.mem.u32(0x2034), 1);
        assert_eq!(e.mem.f32(0x3000), 1.5);
        assert_eq!(e.mem.u8(0x3004), 125);
        assert_eq!(e.mem.u32(0x3008), 1.25f32.to_bits());
        assert_eq!(e.mem.u32(0x300c), 0x8000);
    }

    #[test]
    fn blend_tag_skips_one_space_and_writes_the_group_byte() {
        let mut e = Engine::new();
        e.map(0x2000, 0x3c);
        e.map(0x6000, 12);
        for (index, byte) in b"Blend: 17\0".iter().enumerate() {
            e.mem.set_u8(0x6000 + index as u32, *byte);
        }
        e.register_double(STRNICMP, |_, arguments| {
            assert_eq!(arguments.len(), 3);
            Ret {
                eax: if arguments[1] == 0x0104_879c { 0 } else { 1 },
                ..Ret::default()
            }
        });
        e.register_double(0x00ec_a6d3, |_, arguments| {
            assert_eq!(arguments, [0x6007]);
            Ret {
                eax: 17,
                ..Ret::default()
            }
        });
        assert!(anim_read_number_tag(&mut e, Ptr::new(0x2000), 0x6000));
        assert_eq!(e.mem.u8(0x2029), 17);
    }

    #[test]
    fn parent_note_reloads_the_text_after_inspecting_the_space() {
        let mut e = Engine::new();
        e.map(0x2000, 0x3c);
        e.map(0x4000, 8);
        e.mem.set_u8(0x4004, b' ');
        let mut getter_calls = 0;
        e.register_double(0x0043_b1b0, move |_, arguments| {
            assert_eq!(arguments, [0x3000]);
            getter_calls += 1;
            Ret {
                eax: if getter_calls == 1 { 0x4000 } else { 0x5000 },
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        anim_parent_note(&mut e, Ptr::new(0x2000), 0x3000);
        assert_eq!(e.mem.u32(0x2030), 0x5005);
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0043_b1b0)
                .count(),
            2
        );
    }

    #[test]
    fn enum_note_allocates_entry_and_packs_key_index_into_weight() {
        let mut e = Engine::new();
        let note = b"Enum: Foo\0";
        e.map(0x2000, 0x3c);
        e.map(0x3000, 0x10);
        e.map(0x5000, 0x500);
        e.map(0x6000, note.len() as u32);
        e.map(ENUM_NAME_TABLE, 0x1f * 4);
        e.mem.set_u32(ENUM_NAME_TABLE, 0x7000);
        for (index, byte) in note.iter().enumerate() {
            e.mem.set_u8(0x6000 + index as u32, *byte);
        }
        e.register_double(STRCHR, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            let mut address = arguments[0];
            while e.mem.u8(address) != 0 {
                if e.mem.u8(address) == arguments[1] as u8 {
                    return Ret {
                        eax: address,
                        ..Ret::default()
                    };
                }
                address += 1;
            }
            Ret::default()
        });
        e.register_double(STRLEN, |_, arguments| {
            assert_eq!(arguments, [0x0104_877c]);
            Ret {
                eax: 6,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |e, arguments| {
            assert_eq!(arguments.len(), 3);
            let mut index = 0;
            loop {
                let byte = e.mem.u8(arguments[2] + index);
                e.mem.set_u8(arguments[0] + index, byte);
                if byte == 0 {
                    break;
                }
                index += 1;
            }
            Ret::default()
        });
        e.register_double(0x0040_4dc0, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            assert_eq!(arguments[0], 0x7000);
            assert_eq!(e.mem.u8(arguments[1]), b'F');
            Ret::default()
        });
        e.register_double(0x005f_36f0, |_, arguments| {
            assert_eq!(arguments, [0x2000]);
            Ret::default()
        });
        e.register_double(OPERATOR_NEW, |_, arguments| {
            assert_eq!(arguments, [0x10]);
            Ret {
                eax: 0x3000,
                ..Ret::default()
            }
        });
        assert!(!anim_sound_or_enum(
            &mut e,
            Ptr::new(0x2000),
            0x6000,
            0x6000,
            3,
            1.25,
            Ptr::new(0x8000),
            true,
            0x5000,
        ));
        assert_eq!(e.mem.u32(0x2034), 1);
        assert_eq!(e.mem.u32(0x2038), 0x3000);
        assert_eq!(e.mem.f32(0x3000), 1.25);
        assert_eq!(e.mem.u8(0x3004), 0);
        assert_eq!(e.mem.u32(0x3008), 3);
    }

    #[test]
    fn validation_repairs_required_times_for_category_two() {
        let mut e = Engine::new();
        e.map(0x2000, 0x3c);
        e.map(0x3000, 0x10);
        e.map(ANIM_NAME_TABLE, 0x24);
        e.map(0x0119_9bcc, 0x20);
        e.mem.set_u16(0x2010, 0);
        e.mem.set_u32(0x2014, 4);
        e.mem.set_u32(0x2018, 0x3000);
        e.mem.set_u32(ANIM_CATEGORY_TABLE, 2);
        for (index, time) in [2.0f32, 1.0, 1.0, 3.0].into_iter().enumerate() {
            e.mem.set_f32(0x3000 + index as u32 * 4, time);
        }
        e.register_double(ISNAN, |_, arguments| {
            assert_eq!(arguments.len(), 2);
            Ret::default()
        });
        assert!(!anim_validate(
            &mut e,
            Ptr::new(0x2000),
            Ptr::new(0x4000),
            Ptr::new(0x5000),
        ));
        assert_eq!(e.mem.f32(0x3004), 2.0);
        assert_eq!(e.mem.f32(0x3008), 3.0);
    }

    #[test]
    fn validation_reloads_group_type_for_both_tail_diagnostics() {
        let mut e = Engine::new();
        e.map(0x2000, 0x3c);
        e.map(0x3000, 8);
        e.map(ANIM_NAME_TABLE, 0x48);
        e.map(0x0119_9bcc, 0x20);
        e.mem.set_u16(0x2010, 0);
        e.mem.set_u32(0x2014, 2);
        e.mem.set_u32(0x2018, 0x3000);
        e.mem.set_f32(0x3000, 1.0);
        e.mem.set_f32(0x3004, 0.0);
        e.mem.set_u32(ANIM_NAME_TABLE, 0x7000);
        e.mem.set_u32(ANIM_NAME_TABLE + 0x24, 0x7010);
        e.mem.set_u8(0x2029, 1);
        e.mem.set_u8(0x202a, 2);
        e.register_double(ISNAN, |e, arguments| {
            assert_eq!(arguments.len(), 2);
            e.mem.set_u16(0x2010, 1);
            Ret::default()
        });
        e.register_double(INVALID_ACTION_LOG, |_, arguments| {
            if arguments[0] == 0x0104_8658 {
                assert_eq!(arguments, [0x0104_8658, 0x7010, 0x5000]);
            } else {
                assert_eq!(arguments, [0x0104_8600, 0x7010, 0x5000, 1, 2, 0]);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(anim_validate(
            &mut e,
            Ptr::new(0x2000),
            Ptr::new(0x4000),
            Ptr::new(0x5000),
        ));
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == INVALID_ACTION_LOG)
                .count(),
            2
        );
    }

    #[test]
    fn vector_length_forwards_x87_result_from_interior_vector() {
        let mut e = Engine::new();
        e.register_double(0x0045_7990, |_, arguments| {
            assert_eq!(arguments, [0x201c]);
            Ret {
                st0: 3.25,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x005f_4c70, &[0x2000]).f64(), 3.25);
    }

    #[test]
    fn vector_reader_copies_three_words_and_returns_the_output() {
        let mut e = Engine::new();
        e.map(0x2000, 0x3c);
        e.map(0x3000, 12);
        for (offset, word) in [(0, 0x3f80_0000), (4, 0x4000_0000), (8, 0x4040_0000)] {
            e.mem.set_u32(0x201c + offset, word);
        }
        assert_eq!(e.call(0x005f_4c90, &[0x2000, 0x3000]).u32(), 0x3000);
        assert_eq!(e.mem.u32(0x3000), 0x3f80_0000);
        assert_eq!(e.mem.u32(0x3004), 0x4000_0000);
        assert_eq!(e.mem.u32(0x3008), 0x4040_0000);
    }

    #[test]
    fn type_predicate_accepts_only_native_zero_one_and_aa_types() {
        let mut e = Engine::new();
        for (group, expected) in [
            (0x100u32, true),
            (0x101, true),
            (0x1aa, true),
            (0x102, false),
        ] {
            assert_eq!(e.call(0x005f_4cc0, &[group]).bool(), expected);
        }
    }

    #[test]
    fn fill_priority_array_skips_null_names_and_preserves_other_bytes() {
        let mut e = Engine::new();
        e.map(0x2000, 0x3c);
        e.map(0x0119_9bcc, 0x20);
        e.mem.set_u32(0x0119_9bcc, 0x4000);
        e.mem.set_u32(0x0119_9bcc + 8, 0x4010);
        e.mem.set_u8(0x2008, 0x7e);
        e.mem.set_u8(0x2009, 0x7d);
        e.mem.set_u8(0x200a, 0x7c);
        e.register_double(0x004e_f9a0, |_, arguments| {
            assert_eq!(arguments[0], 0x3000);
            assert_eq!(arguments[2], 0);
            Ret {
                eax: if arguments[1] == 0x4000 { 4 } else { 9 },
                ..Ret::default()
            }
        });
        e.call(0x005f_4d00, &[0x2000, 0x3000]);
        assert_eq!(
            (e.mem.u8(0x2008), e.mem.u8(0x2009), e.mem.u8(0x200a)),
            (4, 0x7d, 9)
        );
    }

    #[test]
    fn animation_group_packing_matches_native_vectors() {
        let mut e = Engine::new();
        let cases = [
            ([0, 0, 0, 0], 0x0000_0000),
            ([7, 15, 255, 1], 0x0000_ffff),
            ([0, 1, 65_535, 0], 0x0000_00ff),
            ([0, 65_535, 65_536, 0], 0x0001_ff00),
            ([u32::MAX, 0x1234, 0xdead_beef, 255], 0xdf2d_62ef),
            ([524_288, 0x1234_5678, u32::MAX, 128], 0x803f_77ff),
        ];
        for (input, expected) in cases {
            assert_eq!(
                e.call(0x005f_2370, &input).u32(),
                expected,
                "input={input:?}"
            );
        }
    }

    #[test]
    fn aim_action_matches_native_type_vectors() {
        let mut e = Engine::new();
        for (group, expected) in [
            (16, false),
            (17, true),
            (22, true),
            (23, false),
            (255, false),
            (273, true),
            (65_535, false),
        ] {
            assert_eq!(
                e.call(0x005f_2630, &[group]).bool(),
                expected,
                "group={group}"
            );
        }
    }

    #[test]
    fn note_resize_reloads_fields_after_allocator_callbacks() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.mem.set_u32(this + 0x34, 3);
        e.mem.set_u32(this + 0x38, 0x2000);
        e.map(0x2000, 0x40);
        e.map(0x3000, 0x40);
        e.map(0x4000, 0x40);
        e.map(0x6000, 0x40);

        e.register_double(OPERATOR_NEW, |_, _| Ret {
            eax: 0x3000,
            ..Ret::default()
        });
        let mut copy_count = 0;
        e.register_double(MEMCPY, move |e, _| {
            copy_count += 1;
            if copy_count == 1 {
                e.mem.set_u32(this + 0x34, 5);
                e.mem.set_u32(this + 0x38, 0x2222);
            }
            Ret::default()
        });
        e.register_double(REALLOC, |_, _| Ret {
            eax: 0x4000,
            ..Ret::default()
        });
        e.register_double(OPERATOR_DELETE, move |e, _| {
            e.mem.set_u32(this + 0x34, 4);
            e.mem.set_u32(this + 0x38, 0x6000);
            Ret::default()
        });
        e.call_log = Some(vec![]);

        e.call(0x005f_2820, &[this, 7]);

        let log = e.call_log.take().unwrap();
        let realloc_args = log.iter().find(|(addr, _)| *addr == REALLOC).unwrap();
        assert_eq!(realloc_args.1, [0x2222, 5 * 0x10]);
        let copies: Vec<_> = log.iter().filter(|(addr, _)| *addr == MEMCPY).collect();
        assert_eq!(copies[0].1, [0x3000, 0x2000, 3 * 0x10]);
        assert_eq!(copies[1].1, [0x4000, 0x3000, 3 * 0x10]);
        assert_eq!(e.mem.u32(this + 0x34), 4);
        assert_eq!(e.mem.u32(this + 0x38), 0x6000);
        assert_eq!(e.mem.u32(0x6030 + 0xc), 0);
        assert_eq!(e.mem.u32(0x6030), 0);
        assert_eq!(e.mem.u8(0x6030 + 4), 0xfa);
        assert_eq!(e.mem.u32(0x6030 + 8), 1.0f32.to_bits());
    }

    #[test]
    fn frame_count_uses_section_stride_and_reloads_after_allocator_callbacks() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.mem.set_u32(this + 0x18, 0x5000);
        e.map(0x5000, 0x40);
        e.map(GROUP_TYPE_TABLE, 4 * 0x24);
        e.mem.set_u32(GROUP_TYPE_TABLE + 3 * 0x24, 2);
        e.map(SECTION_NAME_TABLE, 0x200);
        e.map(0x2000, 0x20);
        e.mem.set_u32(SECTION_NAME_TABLE + 2 * 4, 0x2000);
        e.mem.set_u8(0x2000, b'A');
        e.mem.set_u32(SECTION_NAME_TABLE + 0x2c + 2 * 4, 0x2010);
        e.mem.set_u8(0x2010, 0);
        // The incorrect `(row * stride + category) * 4` expression would
        // read this non-empty decoy instead of the row-one terminator.
        e.mem.set_u32(SECTION_NAME_TABLE + (0x2c + 2) * 4, 0x2000);

        e.register_double(OPERATOR_DELETE, move |e, _| {
            e.mem.set_u32(this + 0x14, 3);
            Ret::default()
        });
        e.register_double(OPERATOR_NEW, move |e, args| {
            assert_eq!(args, [12]);
            e.mem.set_u32(this + 0x14, 4);
            Ret {
                eax: 0x6000,
                ..Ret::default()
            }
        });
        e.register_double(MEMSET, |_, args| {
            assert_eq!(args, [0x6000, 0, 16]);
            Ret::default()
        });

        e.call(0x005f_2450, &[this, 3]);

        assert_eq!(e.mem.u32(this + 0x14), 4);
        assert_eq!(e.mem.u32(this + 0x18), 0x6000);
    }

    #[test]
    fn animation_times_and_note_times_quiet_signaling_nan_on_x87_store() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.mem.set_u32(this + 0x14, 2);
        e.mem.set_u32(this + 0x18, 0x2000);
        e.mem.set_u32(this + 0x34, 2);
        e.mem.set_u32(this + 0x38, 0x3000);
        e.map(0x2000, 8);
        e.map(0x3000, 0x20);

        assert!(e.call(0x005f_3740, &[this, 1, 0x7f80_0001]).bool());
        e.call(0x005f_29f0, &[this, 1, 0x7f80_0001]);

        assert_eq!(e.mem.u32(0x2004), 0x7fc0_0001);
        assert_eq!(e.mem.u32(0x3010), 0x7fc0_0001);
    }

    #[test]
    fn constructor_does_not_destroy_its_base_on_normal_return() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x3c);
        e.map(0, 1);
        e.map(GROUP_TYPE_TABLE, 0x24);
        e.map(SECTION_NAME_TABLE, 4);
        e.map(0x011f_426c, 12);
        e.register_double(0x0049_68b0, |_, _| Ret::default());
        e.register_double(0x0068_15c0, |_, _| Ret::default());
        e.call_log = Some(vec![]);

        assert_eq!(e.call(0x005f_21c0, &[this, 0]).u32(), this);

        let calls = e.call_log.take().unwrap();
        assert!(calls.iter().any(|(address, _)| *address == 0x0049_68b0));
        assert!(calls.iter().any(|(address, _)| *address == 0x0068_15c0));
        assert!(!calls.iter().any(|(address, _)| *address == 0x0049_6910));
    }

    #[test]
    fn destructor_frees_arrays_before_destroying_its_base() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x3c);
        e.mem.set_u32(this + 0x18, 0x2000);
        e.mem.set_u32(this + 0x38, 0x3000);
        e.register_double(OPERATOR_DELETE, |_, _| Ret::default());
        e.register_double(0x0049_6910, |_, _| Ret::default());
        e.call_log = Some(vec![]);

        e.call(0x005f_22d0, &[this]);

        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x34), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        let calls = e.call_log.take().unwrap();
        let calls: Vec<_> = calls
            .into_iter()
            .filter(|(address, _)| *address == OPERATOR_DELETE || *address == 0x0049_6910)
            .collect();
        assert_eq!(calls[0], (OPERATOR_DELETE, [0x2000].to_vec()));
        assert_eq!(calls[1], (OPERATOR_DELETE, [0x3000].to_vec()));
        assert_eq!(calls[2], (0x0049_6910, [this].to_vec()));
    }

    #[test]
    fn filename_parser_reloads_table_values_and_prefix_length_after_callbacks() {
        let mut e = Engine::new();
        e.map(PA_PREFIX, 0x20);
        e.map(MOVE_NAME_TABLE, 0x30);
        e.map(WEAPON_NAME_TABLE, 0x30);
        e.map(0x3000, 12);
        e.mem.set_u32(MOVE_NAME_TABLE + 4, 0x2100);
        e.mem.set_u32(WEAPON_NAME_TABLE + 4, 0x2300);
        e.register_double(STRRCHR, |_, _| Ret {
            eax: 0x4000,
            ..Ret::default()
        });
        let mut prefix_lengths = 0;
        e.register_double(STRLEN, move |e, args| {
            let length = match args[0] {
                PA_PREFIX => {
                    prefix_lengths += 1;
                    if prefix_lengths == 1 {
                        2
                    } else {
                        3
                    }
                }
                0x2100 => {
                    e.mem.set_u32(MOVE_NAME_TABLE + 4, 0x2200);
                    2
                }
                0x2200 => 3,
                0x2300 => 2,
                address => panic!("unexpected STRLEN argument {address:#x}"),
            };
            Ret {
                eax: length,
                ..Ret::default()
            }
        });
        e.register_double(STRNICMP, |_, _| Ret {
            eax: 0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);

        e.call(0x005f_38d0, &[0x4000, 0x3000, 0x3004, 0x3008]);

        assert_eq!(e.mem.u32(0x3000), 1);
        assert_eq!(e.mem.u32(0x3004), 1);
        assert_eq!(e.mem.u8(0x3008), 1);
        let calls = e.call_log.take().unwrap();
        let lengths: Vec<_> = calls
            .iter()
            .filter(|(address, _)| *address == STRLEN)
            .map(|(_, args)| args[0])
            .collect();
        assert_eq!(lengths, [PA_PREFIX, PA_PREFIX, 0x2100, 0x2200, 0x2300]);
        let comparisons: Vec<_> = calls
            .iter()
            .filter(|(address, _)| *address == STRNICMP)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(comparisons[0], [0x4001, PA_PREFIX, 2]);
        assert_eq!(comparisons[1], [0x4004, 0x2200, 2]);
        assert_eq!(comparisons[2], [0x4007, 0x2300, 2]);
    }

    #[test]
    fn group_selectors_and_category_predicates_match_pc_ranges() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.map(GROUP_TYPE_TABLE, 0x24 * 8);
        e.mem.set_u16(this + 0x10, 0x5c32);
        assert_eq!(e.call(0x005f_23a0, &[this]).u32(), 5);
        assert_eq!(e.call(0x005f_23c0, &[0x5c32]).u32(), 5);
        assert_eq!(e.call(0x005f_23e0, &[this]).u32(), 12);
        assert_eq!(e.call(0x005f_2400, &[0x5c32]).u32(), 12);
        assert_eq!(e.call(0x005f_2420, &[this]).u32(), 0x32);
        assert_eq!(e.call(0x005f_2440, &[0x5c32]).u32(), 0x32);
        e.mem.set_u16(this + 0x10, 0x5c5c);
        assert!(e.call(0x005f_26c0, &[this]).bool());

        for (value, expected) in [(4, false), (5, true), (9, true), (10, false)] {
            assert_eq!(e.call(0x005f_25a0, &[value]).bool(), expected);
        }
        for (value, expected) in [(0xb0, false), (0xb1, true), (0xc7, true), (0xc8, false)] {
            assert_eq!(e.call(0x005f_2700, &[value]).bool(), expected);
        }
        assert!(e.call(0x005f_2610, &[9]).bool());
        assert!(!e.call(0x005f_2610, &[8]).bool());
        assert!(e.call(0x005f_26b0, &[6]).bool());
        assert!(!e.call(0x005f_26b0, &[5]).bool());

        e.mem.set_u32(GROUP_TYPE_TABLE + 3 * 0x24, 9);
        assert!(e.call(0x005f_25d0, &[3]).bool());
        assert!(!e.call(0x005f_25d0, &[0xff]).bool());
        assert!(e.call(0x005f_2540, &[0x4a]).bool());
        assert!(e.call(0x005f_2540, &[3]).bool());
        e.mem.set_u32(GROUP_TYPE_TABLE + 3 * 0x24, 6);
        assert!(e.call(0x005f_2670, &[3]).bool());
        assert!(!e.call(0x005f_2670, &[0xff]).bool());
        assert!(e.call(0x005f_2720, &[0x14]).bool());
        assert!(!e.call(0x005f_2720, &[0xff]).bool());
        assert!(e.call(0x005f_2750, &[0x14]).bool());
        assert!(!e.call(0x005f_2750, &[0x18]).bool());
    }

    #[test]
    fn sequence_clock_time_lookup_and_line_skip_use_native_helpers() {
        let mut e = Engine::new();
        let animation = 0x1000;
        e.map(animation, 0xe0);
        e.mem.set_f32(animation + 0xd0, 1.25);
        e.register_double(SECONDS_SINCE_START, |_, a| {
            assert_eq!(a, [0x2000]);
            Ret {
                st0: 42.5,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x0049_3800, &[animation, 0x2000]).f64(), 43.75);

        e.map(GROUP_TYPE_TABLE, 0x24 * 4);
        e.map(TYPE_NAME_TABLE, 0x24 * 4);
        e.map(0x3000, 1);
        e.mem.set_u16(animation + 0x10, 3);
        e.mem.set_u32(TYPE_NAME_TABLE + 3 * 0x24, 0x3000);
        e.mem.set_u32(animation + 0x14, 2);
        e.mem.set_u32(animation + 0x18, 0x4000);
        e.map(0x4000, 8);
        e.mem.set_f32(0x4000 + 4, 1.5);
        e.register_double(ISNAN, |_, _| Ret::default());
        assert_eq!(e.call(0x005f_3780, &[animation, 1]).f64(), 1.5);

        e.map(0x2000, 8);
        e.mem.set_u8(0x2000, b'a');
        e.mem.set_u8(0x2001, b'\n');
        e.mem.set_u8(0x2002, b'\r');
        e.mem.set_u8(0x2003, b'b');
        e.register_double(STRCHR, |_, _| Ret {
            eax: 0x2001,
            ..Ret::default()
        });
        assert_eq!(e.call(0x005f_3870, &[0x2000]).u32(), 0x2003);
    }

    #[test]
    fn time_lookup_index_logs_end_with_the_group_id() {
        let mut e = Engine::new();
        let animation = 0x1000;
        e.map(animation, 0x20);
        e.map(TYPE_NAME_TABLE, 0x24 * 4);
        // Type byte 3 under weapon bits, so the group word differs from the type.
        e.mem.set_u16(animation + 0x10, 0x0203);
        e.mem.set_u32(TYPE_NAME_TABLE + 3 * 0x24, 0x3000);
        e.mem.set_u32(animation + 0x14, 2);
        e.register_double(INVALID_ACTION_LOG, |_, arguments| {
            if arguments[0] == LOG_ANIM_GROUP_ACTION_LOW {
                // 005f37bd ADD ESP,0x10: format, index, type name, group id.
                assert_eq!(
                    arguments,
                    [LOG_ANIM_GROUP_ACTION_LOW, 0xffff_fffe, 0x3000, 0x0203]
                );
            } else {
                // 005f3809 ADD ESP,0x14: format, index, count, type name, group id.
                assert_eq!(
                    arguments,
                    [LOG_ANIM_GROUP_ACTION_HIGH, 2, 2, 0x3000, 0x0203]
                );
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005f_3780, &[animation, 0xffff_fffe]).f64(), 0.0);
        assert_eq!(e.call(0x005f_3780, &[animation, 2]).f64(), 0.0);
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == INVALID_ACTION_LOG)
                .count(),
            2
        );
    }

    #[test]
    fn note_mutators_and_sound_flags_write_expected_fields() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x40);
        e.mem.set_u32(this + 0x34, 2);
        e.mem.set_u32(this + 0x38, 0x3000);
        e.mem.set_u32(this + 0x0c, 2);
        e.mem.set_u32(this + 0x14, 0x4000);
        e.map(0x3000, 0x20);
        e.map(0x4000, 0x20);
        e.map(NOTE_PRIORITY_SCALE, 8);
        e.set_global(NOTE_PRIORITY_SCALE, 250.0f64);

        e.call(0x005f_2a30, &[this, 0, 0xdead_beef]);
        e.call(0x005f_2a70, &[this, 1, 0x12, 2.5f32.to_bits()]);
        e.call(0x005f_2ac0, &[this, 1, 0.02f32.to_bits()]);
        e.call(0x005f_2b20, &[this, 1, 3.25f32.to_bits()]);
        e.call(0x005f_4bd0, &[this, 1, 0x7e]);
        assert_eq!(e.call(0x005f_4c00, &[this, 1]).u32(), 0x7e);
        assert_eq!(e.call(0x005f_4c00, &[this, 2]).u32(), 0xff);

        assert_eq!(e.mem.u32(0x3000 + 0xc), 0xdead_beef);
        assert_eq!(e.mem.u8(0x3010 + 4), 4);
        assert_eq!(e.mem.u32(0x3010 + 8), 3.25f32.to_bits());
        assert_eq!(e.mem.u8(0x4000 + 0x0d + 0x10), 0x7e);
    }

    #[test]
    fn blend_parent_and_speed_setters_use_class_offsets() {
        let mut e = Engine::new();
        let this = 0x1000;
        let source = 0x2000;
        e.map(this, 0x40);
        e.map(source, 12);
        e.mem.set_u32(source, 0x3f80_0000);
        e.mem.set_u32(source + 4, 0x4000_0000);
        e.mem.set_u32(source + 8, 0x4040_0000);
        e.call(0x005f_4b70, &[this, 0x123]);
        e.call(0x005f_4b90, &[this, 0x45]);
        e.call(0x005f_4bb0, &[this, 0x8765_4321]);
        e.call(0x005f_4c40, &[this, source]);

        assert_eq!(e.mem.u8(this + 0x29), 0x23);
        assert_eq!(e.mem.u8(this + 0x2b), 0x45);
        assert_eq!(e.mem.u32(this + 0x30), 0x8765_4321);
        assert_eq!(e.mem.u32(this + 0x1c), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0x20), 0x4000_0000);
        assert_eq!(e.mem.u32(this + 0x24), 0x4040_0000);
    }

    #[test]
    fn scalar_deleting_destructor_runs_sized_delete_when_flag_is_set() {
        let mut e = Engine::new();
        let this = 0x1000;
        e.map(this, 0x3c);
        e.register_double(OPERATOR_DELETE, |_, _| Ret::default());
        e.register_double(0x0049_6910, |_, _| Ret::default());
        e.register_double(0x00aa_1460, move |_, args| {
            assert_eq!(args, [this, 0x3c]);
            Ret::default()
        });
        assert_eq!(e.call(0x005f_22a0, &[this, 1]).u32(), this);
    }
}
